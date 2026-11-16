//! Staging module oc379 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc379`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002d_c190,
    ps2_fog_particle_man_v02,
    "cPS2FogParticleMan_v02",
)];

// ---------------------------------------------------------------------------
// cPS2FogParticleMan_v02 (0x2dc190)
// ---------------------------------------------------------------------------

/// `gp+0x2a90`: the graphics manager.
pub(crate) const FOG_V02_GP_GFX: u32 = 0x2a90;
/// Graphics manager fields: sprite scale x/y (`+0x6af0`, `+0x6b04`, the
/// projection's diagonal), the view-projection matrix, the clip box
/// (min x/y/z, max x/y/z) and the "matrices current" flag.
pub(crate) const FOG_V02_GFX_SCALE_X: u32 = 0x6af0;
pub(crate) const FOG_V02_GFX_SCALE_Y: u32 = 0x6b04;
pub(crate) const FOG_V02_GFX_VIEW: u32 = 0x6b30;
pub(crate) const FOG_V02_GFX_MIN_X: u32 = 0x6b70;
pub(crate) const FOG_V02_GFX_MIN_Y: u32 = 0x6b74;
pub(crate) const FOG_V02_GFX_MAX_Z: u32 = 0x6b78;
pub(crate) const FOG_V02_GFX_MAX_X: u32 = 0x6b80;
pub(crate) const FOG_V02_GFX_MAX_Y: u32 = 0x6b84;
pub(crate) const FOG_V02_GFX_MIN_Z: u32 = 0x6b88;
pub(crate) const FOG_V02_GFX_VIEW_VALID: u32 = 0x6b90;

/// `.lit4` constants (`gp-`): far cut-off 25000, fade start 18000, fade
/// scale 1/7000, near ramp end 2499.67 and near ramp scale 1/1999.67.
pub(crate) const FOG_V02_LIT_FAR: u32 = 0x3c58;
pub(crate) const FOG_V02_LIT_FADE_START: u32 = 0x3c54;
pub(crate) const FOG_V02_LIT_FADE_SCALE: u32 = 0x3c50;
pub(crate) const FOG_V02_LIT_NEAR_END: u32 = 0x3c4c;
pub(crate) const FOG_V02_LIT_NEAR_SCALE: u32 = 0x3c48;

/// Fog object: world matrix (`+0x10`), mesh (`+0x64`), bounding box min
/// (`+0x68`) and max (`+0x74`).
pub(crate) const FOG_V02_OBJ_MAT: u32 = 0x10;
pub(crate) const FOG_V02_OBJ_MESH: u32 = 0x64;
pub(crate) const FOG_V02_OBJ_MIN: u32 = 0x68;
pub(crate) const FOG_V02_OBJ_MAX: u32 = 0x74;
/// Mesh: part-list count (`+4`), list (`+8`; the list's part at `+4`).
pub(crate) const FOG_V02_MESH_COUNT: u32 = 0x4;
pub(crate) const FOG_V02_MESH_LIST: u32 = 0x8;
pub(crate) const FOG_V02_LIST_PART: u32 = 0x4;
/// Part: vertex count (`+0x1c`) and vertices (`+0x20`, 0x1c bytes each:
/// position, colour rgb at `+0xc`, size at `+0x18`).
pub(crate) const FOG_V02_PART_COUNT: u32 = 0x1c;
pub(crate) const FOG_V02_PART_VERTS: u32 = 0x20;
pub(crate) const FOG_V02_VERT_SIZE: u32 = 0x1c;
/// The sort buffer (`{index, depth}` pairs) the render pass qsorts.
pub(crate) const FOG_V02_SORT_BUF: u32 = 0x0053_7ac0;

/// Load the 4x4 matrix at `m` into vf4..vf7.
fn fog_v02_load_mat(c: &mut Cpu, m: u32) {
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
}

/// `vf[dst] = (vf4 vf5 vf6 vf7) * vf[v]` (the VU0 matrix-vector product).
fn fog_v02_xform(c: &mut Cpu, v: u8, dst: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(v, X));
    c.vu0().madda(0b1111, 5, Src::Bc(v, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(v, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(v, W));
}

/// `void cPS2FogParticleMan_v02(FogList* out, FogObj** objs, int n, int clip)`
/// (0x2dc190): queue the fog particles of `n` objects. Each object whose
/// bounding-box centre is nearer than 25000 gets a distance fade (1 below
/// 18000, then linear to 0); each vertex of its mesh parts is transformed by
/// view * object matrix, faded in between depth 500 and 2500, projected, and
/// (unless both corners are outside the same clip plane when `clip` is set)
/// appended to `out` as a screen quad (corners `p -/+ size * scale`) with
/// colour `(alpha, r, g, b) * 128`, its index and depth going to the sort
/// buffer.
pub fn ps2_fog_particle_man_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        frame.save(c, S1, 0x150);
        frame.save(c, S0, 0x160);
        frame.save(c, S5, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x100);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, FP, 0xe0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S2, 0x140);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S7, 0xf0);
        frame.save_ra(c, 0xd0);
        let gp = r32(c, GP);
        c.g[S1][0] = Addr(gp.wrapping_add(FOG_V02_GP_GFX)).lw(c);
        let gfx = r32(c, S1);
        c.g[V0][0] = Addr(gfx).off(FOG_V02_GFX_VIEW_VALID).lw(c);
        c.g[FP][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            crate::game::render::particle::UPDATE_VIEW_MATRICES.call(c, 0x002d_c1e0)?;
        }

        c.cycles += 2;
        c.g[A0][0] = 0;
        if (c.g[S6][0] as i64) > 0 {
            c.cycles += 10;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 11); // 0.5
            c.g[S4][0] = sx(r32(c, S1).wrapping_add(FOG_V02_GFX_VIEW));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 13); // 1.0
            c.g[S2][0] = sx(sp(c).wrapping_add(0x90));
            let gp = r32(c, GP);
            c.lwc1(12, gp.wrapping_sub(FOG_V02_LIT_FAR));
            c.g[S7][0] = sx(sp(c).wrapping_add(0x50));
            c.g[V0][0] = sx(r32(c, A0) << 2);
            loop {
                fog_v02_object(c);
                c.cycles += 3;
                let more = (c.g[A0][0] as i64) < (c.g[S6][0] as i64);
                c.g[V0][0] = sx(r32(c, A0) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_c210);
                }
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// One object (`objs[a0]`, `v0 = a0 * 4`): distance fade from the box
/// centre, then every part of its mesh. Leaves `a0` = the next index.
fn fog_v02_object(c: &mut Cpu) {
    c.cycles += 46;
    c.g[T7][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, FP)));
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    let obj = r32(c, V1);
    let s = sp(c);
    // Centre = (min + max) * 0.5, w = 1, through the view matrix.
    c.lwc1(3, obj.wrapping_add(FOG_V02_OBJ_MAX));
    c.lwc1(0, obj.wrapping_add(FOG_V02_OBJ_MIN));
    c.lwc1(1, obj.wrapping_add(FOG_V02_OBJ_MIN + 4));
    c.add_s(0, 0, 3);
    c.lwc1(4, obj.wrapping_add(FOG_V02_OBJ_MAX + 4));
    c.lwc1(3, obj.wrapping_add(FOG_V02_OBJ_MAX + 8));
    c.lwc1(2, obj.wrapping_add(FOG_V02_OBJ_MIN + 8));
    c.add_s(1, 1, 4);
    c.mul_s(5, 0, 11);
    c.swc1(0, s.wrapping_add(0x30));
    c.add_s(2, 2, 3);
    c.swc1(13, s.wrapping_add(0x1c));
    c.mul_s(3, 1, 11);
    c.swc1(1, s.wrapping_add(0x34));
    c.swc1(5, s.wrapping_add(0x10));
    c.mul_s(0, 2, 11);
    c.swc1(2, s.wrapping_add(0x38));
    c.swc1(3, s.wrapping_add(0x14));
    c.swc1(5, s.wrapping_add(0x20));
    c.swc1(0, s.wrapping_add(0x18));
    c.swc1(0, s.wrapping_add(0x28));
    c.swc1(3, s.wrapping_add(0x24));
    c.lqc2(8, s.wrapping_add(0x10));
    let view = r32(c, S4);
    fog_v02_load_mat(c, view);
    fog_v02_xform(c, 8, 12);
    c.sqc2(12, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s);
    // Depth = (float)(int)w.
    c.lwc1(0, s.wrapping_add(0xc));
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.c_le_s(12, 1);
    if c.fcc() {
        // Beyond the far cut-off.
        c.g[A0][0] = c.g[T7][0];
        return;
    }

    // Fade (f10): 1 below the fade start, else (far - d) / 7000.
    c.cycles += 5;
    let gp = r32(c, GP);
    c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_FADE_START));
    c.c_lt_s(1, 0);
    let near = c.fcc();
    c.g[V0][0] = sx(obj.wrapping_add(FOG_V02_OBJ_MAT));
    if near {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 10);
    } else {
        c.cycles += 3;
        c.sub_s(1, 12, 1);
        c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_FADE_SCALE));
        c.mul_s(10, 1, 0);
    }

    // view * object matrix -> sp+0x90 (s2), copied to sp+0x50 (s7).
    c.cycles += 43;
    let view = r32(c, S4);
    fog_v02_load_mat(c, view);
    let m = r32(c, V0);
    c.lqc2(8, m);
    c.lqc2(9, m.wrapping_add(0x10));
    c.lqc2(10, m.wrapping_add(0x20));
    c.lqc2(11, m.wrapping_add(0x30));
    for k in 0..4u8 {
        fog_v02_xform(c, 8 + k, 12 + k);
    }
    let s2 = r32(c, S2);
    for k in 0..4u32 {
        c.sqc2(12 + k as usize, s2.wrapping_add(0x10 * k));
    }
    for k in 0..4u32 {
        c.lqc2(1 + k as usize, s2.wrapping_add(0x10 * k));
    }
    let s7 = r32(c, S7);
    for k in 0..4u32 {
        c.sqc2(1 + k as usize, s7.wrapping_add(0x10 * k));
    }
    let obj = r32(c, V1);
    c.g[T3][0] = Addr(obj).off(FOG_V02_OBJ_MESH).lw(c);
    c.g[T9][0] = sx(sp(c).wrapping_add(0x50));
    c.g[T7][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = 0;
    let mesh = r32(c, T3);
    c.g[V0][0] = Addr(mesh).off(FOG_V02_MESH_COUNT).lw(c);
    let any = (c.g[V0][0] as i64) > 0;
    c.g[T8][0] = Addr(mesh).off(FOG_V02_MESH_LIST).lw(c);
    if any {
        c.cycles += 1;
        let list = r32(c, T8);
        c.g[T0][0] = Addr(list).off(FOG_V02_LIST_PART).lw(c);
        loop {
            fog_v02_part(c);
            c.cycles += 4;
            c.g[V1][0] = c.g[T6][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let list = r32(c, T8);
            c.g[T0][0] = Addr(list).off(FOG_V02_LIST_PART).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002d_c3a8);
            }
        }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[T7][0];
}

/// One pass over the part `t0` (mesh index `v1`); leaves `t6 = v1 + 1` and
/// `v0` = the mesh's part count.
fn fog_v02_part(c: &mut Cpu) {
    c.cycles += 2;
    c.g[T6][0] = sx(r32(c, V1).wrapping_add(1));
    if c.g[T0][0] != 0 {
        c.cycles += 3;
        let part = r32(c, T0);
        c.g[V0][0] = Addr(part).off(FOG_V02_PART_COUNT).lw(c);
        if c.g[V0][0] == 0 {
            let mesh = r32(c, T3);
            c.g[V0][0] = Addr(mesh).off(FOG_V02_MESH_COUNT).lw(c);
            return;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(part).off(FOG_V02_PART_VERTS).lw(c);
        let any = (c.g[V0][0] as i64) > 0;
        c.g[T2][0] = 0;
        if any {
            c.cycles += 10;
            c.g[V0][0] = 0x53_0000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 9); // 1.0
            c.g[T4][0] = sx(FOG_V02_SORT_BUF);
            c.g[AT][0] = 0x4300_0000;
            c.mtc1(AT, 8); // 128.0
            c.g[S3][0] = sx(FOG_V02_SORT_BUF + 4);
            c.g[T5][0] = c.g[T9][0];
            c.g[T1][0] = 0x30;
            loop {
                fog_v02_vertex(c);
                c.cycles += 4;
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
                c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(FOG_V02_VERT_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_c3f0);
                }
            }
        }
    }
    c.cycles += 1;
    let mesh = r32(c, T3);
    c.g[V0][0] = Addr(mesh).off(FOG_V02_MESH_COUNT).lw(c);
}

/// One vertex (`a2`): transform by view * object matrix (`t5`), and when
/// in front of the camera emit it. Leaves `v0` = the part's vertex count.
fn fog_v02_vertex(c: &mut Cpu) {
    c.cycles += 25;
    let v = r32(c, A2);
    let s = sp(c);
    c.lwc1(0, v);
    c.lwc1(1, v.wrapping_add(4));
    c.lwc1(2, v.wrapping_add(8));
    c.swc1(0, s.wrapping_add(0x20));
    c.swc1(1, s.wrapping_add(0x24));
    c.swc1(2, s.wrapping_add(0x28));
    c.swc1(9, s.wrapping_add(0x2c));
    c.lqc2(8, s.wrapping_add(0x20));
    let m = r32(c, T5);
    fog_v02_load_mat(c, m);
    fog_v02_xform(c, 8, 12);
    c.sqc2(12, s.wrapping_add(0x30));
    c.lq(V0, s.wrapping_add(0x30));
    c.sq(V0, s.wrapping_add(0x10));
    c.lwc1(0, s.wrapping_add(0x1c));
    c.cvt_w_s(1, 0);
    c.mfc1(A1, 1); // depth = (int)w
    if (c.g[A1][0] as i64) > 0 {
        fog_v02_emit(c);
    }
    let part = r32(c, T0);
    c.g[V0][0] = Addr(part).off(FOG_V02_PART_COUNT).lw(c);
}

/// Alpha from the near ramp; when non-zero, project and append the quad.
fn fog_v02_emit(c: &mut Cpu) {
    c.cycles += 8;
    c.mtc1(A1, 1);
    c.cvt_s_w(1, 1);
    c.g[AT][0] = 0x43fa_0000;
    c.mtc1(AT, 2); // 500.0
    c.c_le_s(1, 2);
    c.mtc1(ZERO, 0);
    if c.fcc() {
        c.mul_s(7, 10, 0);
    } else {
        c.cycles += 4;
        let gp = r32(c, GP);
        c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_NEAR_END));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 4;
            c.sub_s(1, 1, 2);
            c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_NEAR_SCALE));
            c.mul_s(0, 1, 0);
        } else {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
        }
        c.cycles += 1;
        c.mul_s(7, 10, 0);
    }
    c.cycles += 5;
    c.mtc1(ZERO, 6);
    c.c_eq_s(7, 6);
    if c.fcc() {
        return;
    }

    // Sort entry, perspective divide, and the two corners.
    c.cycles += 47;
    let s = sp(c);
    let out = r32(c, S0);
    let gfx = r32(c, S1);
    c.lwc1(3, s.wrapping_add(0x1c));
    c.g[A3][0] = 0x30;
    c.g[A0][0] = Addr(out).lhu(c);
    c.div_s(3, 9, 3);
    c.lwc1(0, s.wrapping_add(0x10));
    c.g[V0][0] = sx(r32(c, A0) << 3);
    c.lwc1(2, s.wrapping_add(0x14));
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, T4)));
    let (at, val) = (r32(c, V1), c.g[A0][0]);
    c.sw(at, val);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
    let (at, val) = (r32(c, V0), c.g[A1][0]);
    c.sw(at, val);
    c.swc1(3, s.wrapping_add(0x1c));
    c.mul_s(0, 0, 3);
    c.mul_s(2, 2, 3);
    c.g[A0][0] = Addr(out).lw(c);
    c.swc1(0, s.wrapping_add(0x10));
    c.swc1(2, s.wrapping_add(0x14));
    c.mult(0, V0, A0, A3);
    let v = r32(c, A2);
    c.lwc1(1, v.wrapping_add(0x18));
    c.lwc1(2, gfx.wrapping_add(FOG_V02_GFX_SCALE_X));
    c.mul_s(1, 1, 3);
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_SCALE_Y));
    c.swc1(6, s.wrapping_add(0x28));
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(out));
    c.swc1(6, s.wrapping_add(0x2c));
    c.mul_s(0, 0, 1);
    c.mul_s(2, 2, 1);
    c.swc1(0, s.wrapping_add(0x24));
    c.swc1(2, s.wrapping_add(0x20));
    // a = p - offset
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x90));
    c.lq(V0, s.wrapping_add(0x90));
    c.sq(V0, s.wrapping_add(0x30));
    let e = r32(c, A0);
    c.sq(V0, e.wrapping_add(0x10));
    // b = p + offset
    c.g[V1][0] = Addr(out).lw(c);
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x90));
    c.mult(0, V0, V1, A3);
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(out));
    c.lq(V0, s.wrapping_add(0x90));
    c.sq(V0, s.wrapping_add(0x30));
    let clip = c.g[S5][0] != 0;
    let e = r32(c, V1);
    c.sq(V0, e.wrapping_add(0x20));

    if clip && fog_v02_clipped(c, out, gfx) {
        return;
    }

    // Colour: alpha, r, g, b (each * 128); then count++.
    c.cycles += 27;
    for k in 0..4u32 {
        let (cnt, idx) = if k % 2 == 0 { (V0, V1) } else { (V1, V0) };
        c.g[cnt][0] = Addr(out).lw(c);
        if k == 0 {
            c.mul_s(0, 7, 8);
            c.mult(0, idx, cnt, T1);
        } else {
            let v = r32(c, A2);
            c.lwc1(0, v.wrapping_add(0x8 + 4 * k));
            c.mult(0, idx, cnt, T1);
            c.mul_s(0, 0, 8);
        }
        c.g[cnt][0] = sx(r32(c, idx).wrapping_add(out));
        let e = r32(c, cnt);
        c.swc1(0, e.wrapping_add(0x30 + 4 * k));
    }
    c.g[V0][0] = Addr(out).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    let n = c.g[V0][0];
    Addr(out).sw(c, n);
}

/// Outcodes of the two corners (bit 2 x < min, 1 x > max, 8 y < min, 4 y >
/// max, 0x20 z > max, 0x10 z < min; all but 0x10 flipped for w < 0): true
/// when both are outside one plane.
fn fog_v02_clipped(c: &mut Cpu, out: u32, gfx: u32) -> bool {
    // Corner a (entry + 0x10); the clip planes stay in f5/f4/f3/f2.
    c.cycles += 12;
    c.g[V0][0] = Addr(out).lw(c);
    c.g[A0][0] = 0;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_X));
    c.mult(0, A1, V0, A3);
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(out));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x10));
    let p = r32(c, V0);
    c.lwc1(1, p);
    c.c_lt_s(1, 0);
    let t = c.fcc();
    c.mov_s(5, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] = 2;
    }
    c.cycles += 5;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_X));
    c.c_lt_s(0, 1);
    let t = c.fcc();
    c.mov_s(4, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] |= 0x1;
    }
    c.cycles += 6;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_Y));
    c.lwc1(1, p.wrapping_add(4));
    c.c_lt_s(1, 0);
    let t = c.fcc();
    c.mov_s(3, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] |= 0x8;
    }
    c.cycles += 5;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_Y));
    c.c_lt_s(0, 1);
    let t = c.fcc();
    c.mov_s(2, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] |= 0x4;
    }
    c.cycles += 5;
    c.lwc1(0, p.wrapping_add(0xc));
    c.c_lt_s(0, 6);
    if !c.fcc() {
        c.lwc1(1, p.wrapping_add(8));
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_Z));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.g[A0][0] |= 0x20;
        }
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_Z));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.g[A0][0] |= 0x10;
        }
    } else {
        c.cycles += 2;
        c.g[A0][0] ^= 0x2f;
    }

    // Corner b (entry + 0x20).
    c.cycles += 8;
    c.mult(0, A1, V1, T1);
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(out));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x20));
    let q = r32(c, V0);
    c.lwc1(0, q);
    c.c_lt_s(0, 5);
    let t = c.fcc();
    c.g[V1][0] = 0;
    if t {
        c.cycles += 1;
        c.g[V1][0] = 2;
    }
    c.cycles += 4;
    c.c_lt_s(4, 0);
    if c.fcc() {
        c.g[V1][0] |= 0x1;
    }
    c.cycles += 5;
    c.lwc1(0, q.wrapping_add(4));
    c.c_lt_s(0, 3);
    if c.fcc() {
        c.g[V1][0] |= 0x8;
    }
    c.cycles += 4;
    c.c_lt_s(2, 0);
    if c.fcc() {
        c.g[V1][0] |= 0x4;
    }
    c.cycles += 6;
    c.lwc1(1, q.wrapping_add(0xc));
    c.mtc1(ZERO, 0);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.lwc1(1, q.wrapping_add(8));
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_Z));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.g[V1][0] |= 0x20;
        }
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_Z));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.g[V1][0] |= 0x10;
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] ^= 0x2f;
    }

    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & c.g[V1][0];
    c.g[V0][0] != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::render::particle::man;

    fn fog_v02_f(v: f32) -> u32 {
        v.to_bits()
    }

    /// `cPS2FogParticleMan_v02` over 0..3 objects with random meshes: box
    /// centres before/inside/after the 18000..25000 fade, vertex depths
    /// around 0, 500 and 2500, empty and missing parts, clip on and off
    /// with corners inside and outside the clip box, and the stale-matrix
    /// path through `PS2GraphicsMan_UpdateViewMatrices`.
    #[test]
    fn ps2_fog_particle_man_v02_matches() {
        check(
            ee::s09::f_002dc190__generated,
            ps2_fog_particle_man_v02,
            300,
            &|c, seed| {
                let mut r = Rng(0x2dc1_9000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x002d_c190;
                c.g[GP][0] = sx(GAME_GP);
                let gp = GAME_GP;
                for (off, v) in [
                    (FOG_V02_LIT_FAR, 0x46c3_5000u32),
                    (FOG_V02_LIT_FADE_START, 0x468c_a000),
                    (FOG_V02_LIT_FADE_SCALE, 0x3915_cbec),
                    (FOG_V02_LIT_NEAR_END, 0x451c_3aae),
                    (FOG_V02_LIT_NEAR_SCALE, 0x3a03_1803),
                ] {
                    c.bus.write32(gp - off, v);
                }

                // Graphics manager at DATA; its view matrix maps w = z.
                let gfx = DATA;
                c.bus.write32(gp + FOG_V02_GP_GFX, gfx);
                let sx_ = 0.5 + (r.next_u32() % 8) as f32;
                let sy_ = 0.5 + (r.next_u32() % 8) as f32;
                for base in [man::WORLD, FOG_V02_GFX_VIEW] {
                    for i in 0..16u32 {
                        c.bus.write32(gfx + base + 4 * i, 0);
                    }
                    c.bus.write32(gfx + base, fog_v02_f(sx_));
                    c.bus.write32(gfx + base + 0x14, fog_v02_f(sy_));
                    c.bus.write32(gfx + base + 0x28, fog_v02_f(1.0));
                    c.bus.write32(gfx + base + 0x2c, fog_v02_f(1.0));
                }
                c.bus.write32(gfx + FOG_V02_GFX_MIN_X, fog_v02_f(-0.6));
                c.bus.write32(gfx + FOG_V02_GFX_MAX_X, fog_v02_f(0.6));
                c.bus.write32(gfx + FOG_V02_GFX_MIN_Y, fog_v02_f(-0.6));
                c.bus.write32(gfx + FOG_V02_GFX_MAX_Y, fog_v02_f(0.6));
                c.bus.write32(
                    gfx + FOG_V02_GFX_MIN_Z,
                    fog_v02_f(r.next_u32() as f32 % 1500.0),
                );
                c.bus.write32(
                    gfx + FOG_V02_GFX_MAX_Z,
                    fog_v02_f(1000.0 + r.next_u32() as f32 % 2000.0),
                );
                let valid = if seed % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 5
                };
                c.bus.write32(gfx + FOG_V02_GFX_VIEW_VALID, valid);
                // For the stale path: identity camera, an empty defer manager.
                let cam = DATA + 0x7000;
                for i in 0..4u32 {
                    for k in 0..4u32 {
                        c.bus.write32(
                            cam + 0x10 * i + 4 * k,
                            if i == k { fog_v02_f(1.0) } else { 0 },
                        );
                    }
                }
                let defer = DATA + 0x8000;
                let rec = defer + 0x68000;
                c.bus.write32(rec + 0x1cc4, 0);
                c.bus.write32(rec + 0x1cc8, 0);
                c.bus.write32(rec + 0x2ed0, 0);
                c.bus.write32(gfx + man::CAMERA, cam);
                c.bus.write32(gfx + man::DEFER, defer);

                // Objects.
                let objs = DATA + 0x80000;
                let out = DATA + 0x90000;
                let n: i32 = match seed % 10 {
                    0 => 0,
                    1 => -1,
                    _ => 1 + (r.next_u32() % 3) as i32,
                };
                for i in 0..3u32 {
                    let obj = DATA + 0x81000 + i * 0x100;
                    let mesh = DATA + 0x82000 + i * 0x40;
                    let list = mesh + 0x20;
                    let part = DATA + 0x83000 + i * 0x40;
                    let verts = DATA + 0x84000 + i * 0x400;
                    c.bus.write32(objs + 4 * i, obj);
                    // Box centre depth: near, fading or culled.
                    let cz = match r.next_u32() % 5 {
                        0 => 30000.0,
                        1 => 25000.0,
                        2 => 18000.0 + (r.next_u32() % 7000) as f32,
                        _ => (r.next_u32() % 18000) as f32,
                    };
                    let hz = (r.next_u32() % 100) as f32;
                    for (k, (lo, hi)) in [(-3.0f32, 5.0f32), (-2.0, 1.0), (cz - hz, cz + hz)]
                        .into_iter()
                        .enumerate()
                    {
                        c.bus
                            .write32(obj + FOG_V02_OBJ_MIN + 4 * k as u32, fog_v02_f(lo));
                        c.bus
                            .write32(obj + FOG_V02_OBJ_MAX + 4 * k as u32, fog_v02_f(hi));
                    }
                    // Object matrix: identity plus a translation.
                    let tz = (r.next_u32() % 1500) as f32 - 200.0;
                    for row in 0..4u32 {
                        for k in 0..4u32 {
                            let v = if row == k { 1.0 } else { 0.0 };
                            c.bus
                                .write32(obj + FOG_V02_OBJ_MAT + 0x10 * row + 4 * k, fog_v02_f(v));
                        }
                    }
                    c.bus.write32(obj + FOG_V02_OBJ_MAT + 0x38, fog_v02_f(tz));
                    c.bus.write32(obj + FOG_V02_OBJ_MESH, mesh);
                    c.bus.write32(mesh + FOG_V02_MESH_COUNT, r.next_u32() % 4);
                    c.bus.write32(mesh + FOG_V02_MESH_LIST, list);
                    let part_ptr = if r.next_u32() % 6 == 0 { 0 } else { part };
                    c.bus.write32(list + FOG_V02_LIST_PART, part_ptr);
                    let nv = match r.next_u32() % 6 {
                        0 => 0,
                        _ => 1 + r.next_u32() % 6,
                    };
                    c.bus.write32(part + FOG_V02_PART_COUNT, nv);
                    c.bus.write32(part + FOG_V02_PART_VERTS, verts);
                    for k in 0..nv {
                        let v = verts + k * FOG_V02_VERT_SIZE;
                        let z = match r.next_u32() % 6 {
                            0 => -300.0 - tz,
                            1 => 500.0 - tz,
                            2 => 0.5 - tz,
                            _ => (r.next_u32() % 3200) as f32 - tz,
                        };
                        let w = (z + tz).abs().max(1.0);
                        let x = ((r.next_u32() % 2000) as f32 / 1000.0 - 1.0) * w;
                        let y = ((r.next_u32() % 2000) as f32 / 1000.0 - 1.0) * w;
                        c.bus.write32(v, fog_v02_f(x));
                        c.bus.write32(v + 4, fog_v02_f(y));
                        c.bus.write32(v + 8, fog_v02_f(z));
                        for ch in 0..3u32 {
                            c.bus.write32(v + 0xc + 4 * ch, r.f32(1.0));
                        }
                        let size = if r.next_u32() % 4 == 0 {
                            0.0
                        } else {
                            (r.next_u32() % 400) as f32
                        };
                        c.bus.write32(v + 0x18, fog_v02_f(size));
                    }
                }
                c.bus.write32(out, r.next_u32() % 4);

                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(objs);
                c.g[A2][0] = n as i64 as u64;
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 9) as u64
                };
            },
        );
    }
}
