//! Staging module oc383 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc383`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::abi;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_0048,
        ui_object_set_anim_param,
        "UIObject_SetAnimParam",
    ),
    (0x003a_39f8, ui_vector_render, "UIVector_Render"),
    (
        0x003a_36d0,
        ui_vector_set_anim_param,
        "UIVector_SetAnimParam",
    ),
    (
        0x002d_8ea8,
        avalanche_render_active,
        "Avalanche_RenderActive",
    ),
    (0x0041_ea18, tolower, "tolower"),
    (0x0041_aa88, strcasecmp, "strcasecmp"),
    (0x0039_21f0, font_measure_text_w, "Font_MeasureTextW"),
    (0x0039_1e30, font_draw_text_w, "Font_DrawTextW"),
    (0x003e_5398, ea_thread_yield, "EAThread_Yield"),
    (
        0x002b_d068,
        t_wpigd_light_glow_blend,
        "tWPIGD_LightGlow_Blend",
    ),
    (0x0039_8868, c_ui_group_render, "cUIGroup_Render"),
    (
        0x0015_65c8,
        be_pre_race_get_interface,
        "BEPreRace_GetInterface",
    ),
    (
        0x0035_7108,
        parent_modifier_update_transform,
        "ParentModifier_UpdateTransform",
    ),
    (
        0x0035_0288,
        world_obj_build_matrices,
        "WorldObj_BuildMatrices",
    ),
    (
        0x003a_c508,
        c_ps2_chunk_resolve_patch_instance_dma_chain,
        "cPS2ChunkResolve_PatchInstanceDmaChain",
    ),
    (
        0x0019_c938,
        bolt_file_index_manager_find_node,
        "BoltFileIndexManager_FindNode",
    ),
    (0x0039_e510, ui_state_update, "UIState_Update"),
    (0x0035_6608, mod_obj_v05, "ModObj_v05"),
    (
        0x003a_7878,
        section_allocator_alloc,
        "SectionAllocator_Alloc",
    ),
    (0x0040_e878, ieee754_rem_pio2f, "__ieee754_rem_pio2f"),
];

// ---------------------------------------------------------------------------
// UIObject_SetAnimParam (0x3a0048)

/// The jump table of `UIObject_SetAnimParam` (17 entries, `.rodata`).
pub(crate) const UI_OBJECT_SET_ANIM_PARAM_TABLE: u32 = 0x0049_3fe0;

/// What one animation channel of `cUIThing::setChannelData` writes.
#[derive(Clone, Copy)]
enum UiAnimChannel {
    /// `this+off = data`.
    Plain(u32),
    /// `this+off = data * *(gp - lit)`.
    Scaled(u32, u32),
    /// Channel 12: the frame (6 bits at bit 13 of the flags word +0x14).
    FrameIndex,
}

/// The cases of the jump table at [`UI_OBJECT_SET_ANIM_PARAM_TABLE`].
fn ui_object_anim_channel(key: u32) -> Option<UiAnimChannel> {
    use UiAnimChannel::*;
    Some(match key {
        0 => Plain(0x44),           // position x
        1 => Plain(0x48),           // position y
        9 => Scaled(0x50, 0x2540),  // scale x (* 0.01)
        10 => Scaled(0x54, 0x253c), // scale y (* 0.01)
        3 => Scaled(0x2c, 0x2538),  // angle x (degrees to radians)
        4 => Scaled(0x30, 0x2534),  // angle y
        5 => Scaled(0x34, 0x2530),  // angle z
        12 => FrameIndex,
        13 => Scaled(0x1c, 0x252c), // colour A (/ 255)
        14 => Scaled(0x20, 0x2528), // colour R
        15 => Scaled(0x24, 0x2524), // colour G
        16 => Scaled(0x28, 0x2520), // colour B
        6 => Plain(0x60),           // size x
        7 => Plain(0x64),           // size y
        _ => return None,           // 2, 8, 11: nothing
    })
}

/// `void cUIThing::setChannelData(cUIThing* this, u16 channel, float data)`
/// (0x3a0048): store one animation channel's value into the UI object
/// (position, scale, angle, colour, size, frame), converting units.
pub fn ui_object_set_anim_param(c: &mut Cpu) {
    c.cycles += 4;
    let key = c.g[A1][0] & 0xffff;
    let this = c.g[A0][0];
    c.g[V1][0] = key;
    c.g[V0][0] = (key < 0x11) as u64;
    c.g[A2][0] = this;
    if key >= 0x11 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = sx(UI_OBJECT_SET_ANIM_PARAM_TABLE);
    let entry = UI_OBJECT_SET_ANIM_PARAM_TABLE + 4 * key as u32;
    c.g[V1][0] = sx(entry);
    c.g[A0][0] = Addr(entry).lw(c);
    let obj = Addr(this as u32);
    let data = FpuF32(c.f[12]);
    let gp = c.g[GP][0] as u32;
    match ui_object_anim_channel(key as u32) {
        Some(UiAnimChannel::Plain(off)) => {
            // Size y (+0x64) falls through to the common return.
            c.cycles += if off == 0x64 { 1 + 2 } else { 2 };
            obj.off(off).write_fpu(c, data);
        }
        Some(UiAnimChannel::Scaled(off, lit)) => {
            c.cycles += 4;
            let k = Addr(gp.wrapping_sub(lit)).read_fpu(c);
            let v = data * k;
            c.f[0] = v.0;
            obj.off(off).write_fpu(c, v);
        }
        Some(UiAnimChannel::FrameIndex) => {
            c.cycles += 11;
            let frame = data.to_i32() as u32;
            c.f[0] = frame;
            let flags = obj.off(0x14).lw(c);
            let mask = 0xffff_ffff_fff8_1fffu64;
            c.g[V1][0] = mask;
            c.g[V0][0] = sx((frame & 0x3f) << 13);
            let v = (flags & mask) | c.g[V0][0];
            c.g[A0][0] = v;
            obj.off(0x14).sw(c, v);
        }
        None => {
            c.cycles += 2;
        }
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UIVector_Render (0x3a39f8)

/// `cUIVector` fields.
pub(crate) const UI_VECTOR_POSITION_X: u32 = 0x44;
pub(crate) const UI_VECTOR_SCALE_X: u32 = 0x50;
/// `count << 17 | mode << 2 | indexed` (bits 17.. vertex count, bit 0: draw indexed triangles).
pub(crate) const UI_VECTOR_COUNTS: u32 = 0x74;
pub(crate) const UI_VECTOR_TRIANGLES: u32 = 0x78;
pub(crate) const UI_VECTOR_COLOURS: u32 = 0x7c;
pub(crate) const UI_VECTOR_POSITIONS: u32 = 0x80;
pub(crate) const UI_VECTOR_RENDER_POSITIONS: u32 = 0x84;
pub(crate) const UI_VECTOR_VERTICES: u32 = 0x8c;
/// `GrMan` (the graphics manager) at `gp - 0x854`; its current draw state
/// at `+0xe84`, its vtable at `+0x10d8`.
pub(crate) const UI_VECTOR_GRMAN_GP: u32 = 0x854;
/// `__fixunssfsi` (float to unsigned int).
const UI_VECTOR_FIXUNSSFSI: u32 = 0x0041_39f8;

/// `this + off` read as a word (`lw`), for `cUIVector` code.
#[inline]
fn ui_vector_lw(c: &mut Cpu, this: u64, off: u32) -> u64 {
    Addr((this as u32).wrapping_add(off)).lw(c)
}

/// `void cUIVector::render2D(cUIVector* this, float x, float y)` (0x3a39f8):
/// scale the vertex positions into the render copy, set the draw layer, then
/// either draw indexed triangles through GrMan (vtable `+0x240`) or build
/// screen vertices (position + offset, colour * 0.5 as integers) and draw
/// them as a strip (vtable `+0x248`).
pub fn ui_vector_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x40);
        frame.save_fpr(c, 23, 0x98);
        c.g[S3][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x90);
        c.mov_s(23, 13);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        let this = c.g[S3][0];
        c.g[V0][0] = ui_vector_lw(c, this, UI_VECTOR_POSITIONS);
        c.mov_s(22, 12);
        if c.g[V0][0] != 0 {
            if ui_vector_render_body(c, this, frame)? {
                // Indexed draw: s0 already reloaded in the call's delay slot.
                ui_vector_render_epilogue(c, frame);
                return Ok(());
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
        ui_vector_render_epilogue(c, frame);
        Ok(())
    })
}

fn ui_vector_render_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 23, 0x98);
    frame.restore_fpr(c, 22, 0x90);
    frame.restore_fpr(c, 21, 0x88);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
}

/// The body after the `mpVertexPosition != 0` test; `Ok(true)` when it took
/// the indexed draw (which reloads s0 itself).
fn ui_vector_render_body(
    c: &mut Cpu,
    this: u64,
    frame: Frame,
) -> Result<bool, crate::call::Unwound> {
    // Render positions = positions scaled by mScale.
    c.cycles += 4;
    c.g[V0][0] = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
    c.g[A2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 28;
            let off = c.g[A1][0] as u32;
            let dst = (ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS) as u32).wrapping_add(off);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            let src = (ui_vector_lw(c, this, UI_VECTOR_POSITIONS) as u32).wrapping_add(off);
            c.g[A0][0] = sx(dst);
            c.g[V0][0] = sx(src);
            let xy = Addr(src).read_u64_unaligned(c);
            c.g[V1][0] = xy;
            let z = Addr(src).off(8).lw(c);
            c.g[A3][0] = z;
            Addr(dst).write_u64_unaligned(c, xy);
            Addr(dst).off(8).sw(c, z);
            for (k, scale) in [(0u32, UI_VECTOR_SCALE_X), (4, UI_VECTOR_SCALE_X + 4)] {
                let p =
                    (ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS) as u32).wrapping_add(off);
                if k == 0 {
                    c.g[V1][0] = sx(p);
                } else {
                    c.g[A0][0] = sx(p);
                }
                let s = Addr((this as u32).wrapping_add(scale)).read_fpu(c);
                c.f[1] = s.0;
                let v = Addr(p).off(k).read_fpu(c) * s;
                c.f[0] = v.0;
                Addr(p).off(k).write_fpu(c, v);
            }
            let n = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
            let more = c.g[A2][0] < n;
            c.g[V0][0] = more as u64;
            c.g[A1][0] = sx(off.wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3a50);
            }
        }
    }

    // Draw state: layer (flags +0x15 & 0x1f) + 9 into bits 5..9, word +0x10 = -1.
    c.cycles += 19;
    let gp = c.g[GP][0] as u32;
    let grman = Addr(gp.wrapping_sub(UI_VECTOR_GRMAN_GP)).lw(c);
    c.g[A2][0] = grman;
    c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
    c.g[A3][0] = u64::MAX;
    let layer = Addr((this as u32).wrapping_add(0x15)).lbu(c) & 0x1f;
    let ds = ui_vector_lw(c, grman, 0xe84);
    c.g[A0][0] = ds;
    let bits = sx(((layer as u32).wrapping_add(9)) << 5) & 0x3e0;
    c.g[V0][0] = bits;
    let w = (ui_vector_lw(c, ds, 8) & 0xffff_ffff_ffff_fc1f) | bits;
    c.g[V1][0] = w;
    Addr((ds as u32).wrapping_add(8)).sw(c, w);
    let ds2 = ui_vector_lw(c, grman, 0xe84);
    c.g[V0][0] = ds2;
    Addr((ds2 as u32).wrapping_add(0x10)).sh(c, u64::MAX);
    let counts = ui_vector_lw(c, this, UI_VECTOR_COUNTS);
    c.g[A2][0] = counts;
    c.g[V0][0] = counts & 1;
    c.g[A1][0] = sx((counts as u32) >> 2);
    if counts & 1 != 0 {
        // Indexed triangles: GrMan->vt[0x240](mode, count, tris, render positions, 0, colours, pos).
        c.cycles += 22;
        let x = Addr((this as u32).wrapping_add(UI_VECTOR_POSITION_X)).read_fpu(c);
        let y = Addr((this as u32).wrapping_add(UI_VECTOR_POSITION_X + 4)).read_fpu(c);
        c.g[A1][0] &= 0x7fff;
        let sp = frame.at(c, 0);
        Addr(sp).off(8).sw(c, 0);
        let px = x + FpuF32(c.f[22]);
        let py = y + FpuF32(c.f[23]);
        c.f[0] = px.0;
        c.f[1] = py.0;
        let t4 = Addr(gp.wrapping_sub(UI_VECTOR_GRMAN_GP)).lw(c);
        c.g[T4][0] = t4;
        c.g[T2][0] = ui_vector_lw(c, this, UI_VECTOR_COLOURS);
        c.g[A2][0] = sx((counts as u32) >> 17);
        Addr(sp).write_fpu(c, px);
        c.g[T1][0] = 0;
        Addr(sp).off(4).write_fpu(c, py);
        c.g[T3][0] = c.g[SP][0];
        let vt = ui_vector_lw(c, t4, 0x10d8);
        c.g[V0][0] = vt;
        c.g[A3][0] = ui_vector_lw(c, this, UI_VECTOR_TRIANGLES);
        let delta = Addr((vt as u32).wrapping_add(0x240)).lh(c);
        c.g[T0][0] = ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS);
        let func = ui_vector_lw(c, vt, 0x244);
        c.g[V1][0] = func;
        c.g[A0][0] = sx((t4 as u32).wrapping_add(delta as u32));
        call_ptr(c, func as u32, 0x003a_3b5c)?;
        frame.restore(c, S0, 0x70);
        return Ok(true);
    }

    c.cycles += 3;
    let verts = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
    let count = sx((counts as u32) >> 17);
    c.g[V0][0] = count;
    if verts == 0 {
        return Ok(false);
    }
    c.cycles += 2;
    c.g[S4][0] = 0;
    if count != 0 {
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        c.f[21] = 0x3f80_0000; // 1.0f
        c.g[AT][0] = 0x3f00_0000;
        c.f[20] = 0x3f00_0000; // 0.5f
        c.g[V0][0] = 0xc;
        loop {
            // Vertex i (0x30 bytes): pos (x, y, z, 1) + offset at +0x20, q = 1, s = t = 0,
            // colour (A, R, G, B) * 0.5 as integers at +0x1c, +0x10, +0x14, +0x18.
            c.cycles += 80;
            let i = c.g[S4][0];
            let rp = ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS);
            c.g[V1][0] = rp;
            c.mult(0, V0, S4, V0);
            let v = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
            c.g[A0][0] = v;
            c.g[S1][0] = 0x30;
            c.g[S2][0] = sx((i as u32) << 4);
            c.mult(0, S1, S4, S1);
            c.g[S4][0] = sx((i as u32).wrapping_add(1));
            let s1 = c.g[S1][0] as u32;
            let p = (c.g[V0][0] as u32).wrapping_add(rp as u32);
            c.g[V0][0] = sx(p);
            let tmp = frame.at(c, 0x10);
            c.lwc1(1, p);
            c.g[A0][0] = sx(s1.wrapping_add(v as u32));
            c.swc1(1, tmp);
            c.lwc1(0, p.wrapping_add(4));
            c.swc1(0, tmp + 4);
            c.lwc1(1, p.wrapping_add(8));
            c.swc1(21, tmp + 0xc);
            c.swc1(1, tmp + 8);
            c.lq(V0, tmp);
            c.sq(V0, (c.g[A0][0] as u32).wrapping_add(0x20));
            // pos.x += mPosition.x + x; pos.y += mPosition.y + y.
            for k in [0u32, 4] {
                let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
                let at = vtx.wrapping_add(0x20);
                let reg = if k == 0 { V1 } else { A0 };
                c.g[reg][0] = sx(at);
                let base = Addr((this as u32).wrapping_add(UI_VECTOR_POSITION_X + k)).read_fpu(c);
                let off = base + FpuF32(c.f[if k == 0 { 22 } else { 23 }]);
                c.f[1] = off.0;
                let r = Addr(at).off(k).read_fpu(c) + off;
                c.f[0] = r.0;
                Addr(at).off(k).write_fpu(c, r);
            }
            let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
            c.g[V0][0] = sx(vtx);
            c.swc1(21, vtx.wrapping_add(8));
            let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
            c.g[V1][0] = sx(vtx);
            Addr(vtx).sw(c, 0);
            let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
            c.g[V0][0] = sx(vtx);
            Addr(vtx).off(4).sw(c, 0);
            // Colours: (channel offset in the colour, field in the vertex, return address).
            let s2 = c.g[S2][0] as u32;
            for (k, field, ret) in [
                (0u32, 0x1cu32, 0x003a_3c58u32),
                (4, 0x10, 0x003a_3c78),
                (8, 0x14, 0x003a_3c98),
                (0xc, 0x18, 0x003a_3cb8),
            ] {
                let colours = ui_vector_lw(c, this, UI_VECTOR_COLOURS);
                let vtx = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
                let col = s2.wrapping_add(colours as u32);
                let dst = s1.wrapping_add(vtx as u32);
                if k == 0xc {
                    c.g[V0][0] = colours;
                    c.g[V1][0] = vtx;
                    c.g[S2][0] = sx(col);
                    c.g[S1][0] = sx(dst);
                } else {
                    c.g[if k == 0 { V1 } else { V0 }][0] = sx(col);
                    c.g[S0][0] = sx(dst);
                }
                c.lwc1(12, col.wrapping_add(k));
                c.mul_s(12, 12, 20);
                call_ptr(c, UI_VECTOR_FIXUNSSFSI, ret)?;
                let reg = if k == 0xc { S1 } else { S0 };
                Addr((c.g[reg][0] as u32).wrapping_add(field)).sw(c, c.g[V0][0]);
            }
            let n = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
            let more = c.g[S4][0] < n;
            c.g[V0][0] = 0xc;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3b90);
            }
        }
    }
    // Strip: GrMan->vt[0x248](count, vertices, 0).
    c.cycles += 10;
    let t0 = Addr(gp.wrapping_sub(UI_VECTOR_GRMAN_GP)).lw(c);
    c.g[T0][0] = t0;
    c.g[A3][0] = 0;
    let counts = ui_vector_lw(c, this, UI_VECTOR_COUNTS);
    let vt = ui_vector_lw(c, t0, 0x10d8);
    c.g[V1][0] = vt;
    c.g[A2][0] = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
    c.g[A1][0] = sx((counts as u32) >> 17);
    let delta = Addr((vt as u32).wrapping_add(0x248)).lh(c);
    let func = ui_vector_lw(c, vt, 0x24c);
    c.g[V0][0] = func;
    c.g[A0][0] = sx((t0 as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, 0x003a_3cf8)?;
    Ok(false)
}

// ---------------------------------------------------------------------------
// UIVector_SetAnimParam (0x3a36d0)

/// `cUIVector` per-vertex channels: (base field, element shift or `None`
/// for the 12-byte position stride, offset in the element, cycles of the
/// case block before the common store).
fn ui_vector_anim_channel(sub: u64) -> (u32, Option<u32>, u32, u64) {
    match sub {
        0 => (UI_VECTOR_POSITIONS, None, 0, 4),    // vertex x
        1 => (UI_VECTOR_POSITIONS, None, 4, 4),    // vertex y
        2 => (UI_VECTOR_POSITIONS, None, 8, 4),    // vertex z
        3 => (0x88, Some(3), 0, 3),                // uv u
        4 => (0x88, Some(3), 4, 3),                // uv v
        5 => (UI_VECTOR_COLOURS, Some(4), 0, 2),   // colour r
        6 => (UI_VECTOR_COLOURS, Some(4), 4, 2),   // colour g
        7 => (UI_VECTOR_COLOURS, Some(4), 8, 2),   // colour b
        _ => (UI_VECTOR_COLOURS, Some(4), 0xc, 2), // colour a
    }
}

/// `void cUIVector::setChannelData(cUIVector* this, u16 channel, float v)`
/// (0x3a36d0): channels below 0x15 go to `cUIThing::setChannelData`; the
/// rest address vertex `(ch - 0x15) / 9`, field `(ch - 0x15) % 9` (position
/// xyz, uv, colour rgba). Nothing without vertices or past the count.
pub fn ui_vector_set_anim_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[A2][0] = this;
        frame.save_ra(c, 0);
        let verts = ui_vector_lw(c, this, UI_VECTOR_POSITIONS);
        c.g[V0][0] = verts;
        let key = c.g[A1][0] & 0xffff;
        c.g[A1][0] = key;
        if verts != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx((key as u32).wrapping_sub(0x15));
            if key < 0x15 {
                c.cycles += 2;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x003a_0048,
                    f: ee::s13::UIObject_SetAnimParam,
                }
                .call(c, 0x003a_37f8)?;
            } else {
                c.cycles += 4 + 9;
                c.g[V1][0] = 9;
                let ch = c.g[V0][0] & 0xffff;
                c.g[A1][0] = ch;
                c.divu(0, A1, V1);
                let count = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
                c.g[V1][0] = c.lo[0];
                c.g[A0][0] = c.hi[0];
                let idx = c.lo[0] & 0xffff;
                c.g[A1][0] = idx;
                c.g[V0][0] = 9;
                if idx < count {
                    c.cycles += 3 + 3;
                    let sub = c.hi[0] & 0xffff;
                    c.g[A0][0] = sub;
                    c.g[V1][0] = (sub < 9) as u64;
                    c.g[V0][0] = 0x49_0000;
                    if sub < 9 {
                        c.cycles += 6;
                        let entry = 0x0049_4110 + 4 * sub as u32;
                        c.g[V0][0] = 0x0049_4110;
                        c.g[V1][0] = sx(entry);
                        c.g[A0][0] = Addr(entry).lw(c);
                        let (field, shift, elem_off, pre) = ui_vector_anim_channel(sub);
                        c.cycles += pre;
                        let base = ui_vector_lw(c, this, field);
                        c.g[V1][0] = base;
                        match shift {
                            None => {
                                c.g[V0][0] = 0xc;
                                c.mult(0, V0, A1, V0);
                            }
                            Some(sh) => c.g[V0][0] = sx((idx as u32) << sh),
                        }
                        c.cycles += 3;
                        let at = (c.g[V0][0] as u32).wrapping_add(base as u32);
                        c.g[V0][0] = sx(at);
                        c.swc1(12, at.wrapping_add(elem_off));
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Avalanche_RenderActive (0x2d8ea8)

/// `tActiveAvalanche` (the slot): group, list of active nodes, limit.
pub(crate) const AVALANCHE_SLOT_ACTIVE: u32 = 0x4;
pub(crate) const AVALANCHE_SLOT_LIMIT: u32 = 0x18;
/// Active node: data pointer, next node, emitter at +0xd0.
pub(crate) const AVALANCHE_NODE_DATA: u32 = 0x2e0;
pub(crate) const AVALANCHE_NODE_NEXT: u32 = 0x2e4;
/// Debug globals: draw lines for the first N nodes (`gp + 0x968`), draw the
/// node effects (`gp + 0x964`).
pub(crate) const AVALANCHE_DEBUG_LINES_GP: u32 = 0x968;
pub(crate) const AVALANCHE_DEBUG_FX_GP: u32 = 0x964;
/// `cGraphicsMan_TheGraphicsMan` (`gp + 0x2a90`).
pub(crate) const AVALANCHE_GRMAN_GP: u32 = 0x2a90;
/// The debug line vertices (`D_vtx`, 0x30 bytes each, two per pair).
const AVALANCHE_LINE_VERTS: u32 = 0x004e_db70;

const AVALANCHE_DEBUG_DRAW_INSTANCE: Callee = Callee {
    addr: 0x002d_8a00,
    f: ee::s09::Avalanche_DebugDrawInstance,
};

/// `void tActiveAvalanche::render(tActiveAvalanche* this)` (0x2d8ea8): the
/// avalanche debug view. For each active node with data: its lines (first
/// `gp+0x968` nodes) and its emitter (`gp+0x964`, data `+0xf2`); then, with
/// lines on, one line per group pair from the pair's node to the rider
/// (red below the slot's limit, green at or above) drawn through GrMan
/// (vtable `+0x248`) with the draw state's bits 23-24 forced to 2.
pub fn avalanche_render_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[S3][0];
        let gp = c.g[GP][0] as u32;
        let group = ui_vector_lw(c, this, 0);
        c.g[V0][0] = group;
        let grman = Addr(gp.wrapping_add(AVALANCHE_GRMAN_GP)).lw(c);
        c.g[S6][0] = grman;
        if group == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
            avalanche_render_active_epilogue(c, frame);
            return Ok(());
        }
        // The active nodes.
        c.cycles += 3;
        c.g[S0][0] = ui_vector_lw(c, this, AVALANCHE_SLOT_ACTIVE);
        c.g[S1][0] = 0;
        while c.g[S0][0] != 0 {
            let node = c.g[S0][0];
            c.cycles += 3;
            let data = ui_vector_lw(c, node, AVALANCHE_NODE_DATA);
            c.g[V0][0] = Addr(gp.wrapping_add(AVALANCHE_DEBUG_LINES_GP)).lw(c);
            'next: {
                if data != 0 {
                    c.cycles += 3;
                    let draw_lines = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = draw_lines as u64;
                    c.g[V0][0] = Addr(gp.wrapping_add(AVALANCHE_DEBUG_FX_GP)).lw(c);
                    if draw_lines {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        AVALANCHE_DEBUG_DRAW_INSTANCE.call(c, 0x002d_8f10)?;
                        c.g[V0][0] = Addr(gp.wrapping_add(AVALANCHE_DEBUG_FX_GP)).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        let node = c.g[S0][0];
                        c.g[S0][0] = ui_vector_lw(c, node, AVALANCHE_NODE_NEXT);
                        break 'next;
                    }
                    c.cycles += 4;
                    let node = c.g[S0][0];
                    let data = ui_vector_lw(c, node, AVALANCHE_NODE_DATA);
                    c.g[V0][0] = data;
                    let fx = Addr((data as u32).wrapping_add(0xf2)).lbu(c);
                    c.g[V1][0] = fx;
                    c.g[A0][0] = sx((node as u32).wrapping_add(0xd0));
                    if fx != 0 {
                        c.cycles += 2;
                        c.g[A1][0] = 7;
                        call_ptr(c, 0x0037_1688, 0x002d_8f34)?;
                    }
                }
                c.cycles += 1;
                let node = c.g[S0][0];
                c.g[S0][0] = ui_vector_lw(c, node, AVALANCHE_NODE_NEXT);
            }
            c.cycles += 2;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            if c.g[S0][0] != 0 && event_due(c) {
                deliver_event_here(c, 0x002d_8ef0);
            }
        }
        c.cycles += 3;
        let lines = Addr(gp.wrapping_add(AVALANCHE_DEBUG_LINES_GP)).lw(c);
        c.g[V0][0] = lines;
        frame.restore(c, S0, 0xa0);
        if lines == 0 {
            avalanche_render_active_epilogue(c, frame);
            return Ok(());
        }
        // One line per pair: pair node's position (+0x40) to the rider.
        c.cycles += 4;
        let group = ui_vector_lw(c, this, 0);
        c.g[V1][0] = group;
        c.g[V0][0] = Addr((group as u32).wrapping_add(0xc)).lhu(c);
        c.g[S2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x4f_0000;
            c.g[S4][0] = 1;
            c.g[S7][0] = sx(AVALANCHE_LINE_VERTS + 0x20);
            c.g[S5][0] = sx(AVALANCHE_LINE_VERTS);
            c.g[FP][0] = sx(AVALANCHE_LINE_VERTS + 0x50);
            loop {
                c.cycles += 18;
                let j = c.g[S2][0] as u32;
                let pair_off = j << 3;
                c.g[S0][0] = sx(pair_off);
                c.g[V0][0] = 0x60;
                c.mult(0, S1, S2, V0);
                let s1 = c.g[S1][0] as u32;
                let pair = (c.g[V1][0] as u32).wrapping_add(pair_off);
                c.g[V1][0] = sx(pair);
                let pnode = Addr(pair).off(0x14).lw(c);
                c.g[V0][0] = pnode;
                c.g[A1][0] = 0;
                c.lq(V1, (pnode as u32).wrapping_add(0x40));
                let v = s1.wrapping_add(c.g[S7][0] as u32);
                c.g[V0][0] = sx(v);
                c.sq(V1, v);
                let sp = frame.at(c, 0);
                c.sq(V1, sp);
                let below = avalanche_pair_below_limit(c, this, pair_off);
                c.g[A0][0] = sx(s1.wrapping_add(c.g[S5][0] as u32));
                if below {
                    c.cycles += 1;
                    c.g[A1][0] = 0xff;
                }
                c.cycles += 9;
                let a0 = c.g[A0][0] as u32;
                Addr(a0).off(0x10).sw(c, c.g[A1][0]);
                let above = avalanche_pair_at_limit(c, this, pair_off);
                c.g[V1][0] = 0;
                if above {
                    c.cycles += 1;
                    c.g[V1][0] = 0xff;
                }
                c.cycles += 25;
                Addr(a0).off(0x14).sw(c, c.g[V1][0]);
                // The rider's position: (SSXApp->Game->AI->rider + 0x6c0)->getPos().
                let app = Addr(gp.wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = app;
                let game = ui_vector_lw(c, app, 0x84);
                let ai = ui_vector_lw(c, game, 0xc);
                c.g[A1][0] = ai;
                let rider = ui_vector_lw(c, ai, 0x28);
                let vt = ui_vector_lw(c, rider, 0x6c0);
                c.g[V1][0] = vt;
                let delta = Addr((vt as u32).wrapping_add(0x28)).lh(c);
                c.g[A1][0] = delta;
                let func = ui_vector_lw(c, vt, 0x2c);
                c.g[V0][0] = func;
                c.g[A0][0] = sx((rider as u32)
                    .wrapping_add(0x6c0)
                    .wrapping_add(delta as u32));
                call_ptr(c, func as u32, 0x002d_9010)?;
                c.lq(V1, c.g[V0][0] as u32);
                c.g[A0][0] = 0x30;
                let s1 = c.g[S1][0] as u32;
                let v = s1.wrapping_add(c.g[FP][0] as u32);
                c.g[V0][0] = sx(v);
                c.sq(V1, v);
                c.mult(0, V0, S4, A0);
                let vert = (c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32);
                c.g[V1][0] = sx(vert);
                let pair_off = c.g[S0][0] as u32;
                let below = avalanche_pair_below_limit(c, this, pair_off);
                c.g[A1][0] = 0;
                if below {
                    c.cycles += 1;
                    c.g[A1][0] = 0xff;
                }
                c.cycles += 9;
                Addr(vert).off(0x10).sw(c, c.g[A1][0]);
                let above = avalanche_pair_at_limit(c, this, pair_off);
                c.g[A0][0] = 0;
                if above {
                    c.cycles += 1;
                    c.g[A0][0] = 0xff;
                }
                c.cycles += 7;
                Addr(vert).off(0x14).sw(c, c.g[A0][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                let group = ui_vector_lw(c, this, 0);
                c.g[V1][0] = group;
                let n = Addr((group as u32).wrapping_add(0xc)).lhu(c);
                let more = (c.g[S2][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_8f70);
                }
            }
        }
        // Draw state word +4: bits 23-24 = 2 for the draw, then put back.
        c.cycles += 28;
        let grman = c.g[S6][0];
        let ds = ui_vector_lw(c, grman, 0xe84);
        c.g[A0][0] = ds;
        c.g[S1][0] = 0xffff_ffff_fe7f_ffff;
        c.g[V1][0] = 0x0100_0000;
        let w = ui_vector_lw(c, ds, 4);
        c.g[S2][0] = 0x0180_0000;
        c.g[A3][0] = 0;
        c.g[A2][0] = sx(AVALANCHE_LINE_VERTS);
        c.g[V0][0] = (w & 0xffff_ffff_fe7f_ffff) | 0x0100_0000;
        Addr((ds as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[S0][0] = sx(((w & 0x0180_0000) as u32) >> 23);
        let vt = ui_vector_lw(c, grman, 0x10d8);
        c.g[V1][0] = vt;
        let group = ui_vector_lw(c, this, 0);
        c.g[V0][0] = group;
        let delta = Addr((vt as u32).wrapping_add(0x248)).lh(c);
        let func = ui_vector_lw(c, vt, 0x24c);
        c.g[T0][0] = func;
        c.g[A1][0] = Addr((group as u32).wrapping_add(0xc)).lhu(c);
        c.g[A0][0] = sx((grman as u32).wrapping_add(delta as u32));
        call_ptr(c, func as u32, 0x002d_90e4)?;
        let grman = c.g[S6][0];
        let ds = ui_vector_lw(c, grman, 0xe84);
        c.g[V1][0] = ds;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 23) & c.g[S2][0];
        let w = ui_vector_lw(c, ds, 4);
        c.g[V0][0] = (w & c.g[S1][0]) | c.g[S0][0];
        Addr((ds as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.cycles += 1;
        frame.restore(c, S0, 0xa0);
        avalanche_render_active_epilogue(c, frame);
        Ok(())
    })
}

/// `group->pairs[j].v < this->limit` (`c.lt.s`; f0, f1, v0 as the original
/// leaves them).
fn avalanche_pair_below_limit(c: &mut Cpu, this: u64, pair_off: u32) -> bool {
    let group = ui_vector_lw(c, this, 0);
    c.lwc1(1, (this as u32).wrapping_add(AVALANCHE_SLOT_LIMIT));
    let pair = (group as u32).wrapping_add(pair_off);
    c.g[V0][0] = sx(pair);
    c.lwc1(0, pair.wrapping_add(0x10));
    c.c_lt_s(0, 1);
    c.fcc()
}

/// `this->limit <= group->pairs[j].v` (`c.le.s`).
fn avalanche_pair_at_limit(c: &mut Cpu, this: u64, pair_off: u32) -> bool {
    let group = ui_vector_lw(c, this, 0);
    c.lwc1(1, (this as u32).wrapping_add(AVALANCHE_SLOT_LIMIT));
    let pair = (group as u32).wrapping_add(pair_off);
    c.g[V0][0] = sx(pair);
    c.lwc1(0, pair.wrapping_add(0x10));
    c.c_le_s(1, 0);
    c.fcc()
}

fn avalanche_render_active_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore(c, FP, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// tolower (0x41ea18), strcasecmp (0x41aa88)

/// `_ctype_ + 1`: the newlib character class table (bit 0: upper case).
pub(crate) const CTYPE_TABLE: u32 = 0x0049_9c81;

/// `int tolower(int ch)` (0x41ea18): `ch + 0x20` if `_ctype_[ch]` says upper
/// case, else `ch` (a1 = the table entry's address, v1 = its upper bit).
pub fn tolower(c: &mut Cpu) {
    c.cycles += 8;
    let ch = c.g[A0][0];
    let at = CTYPE_TABLE.wrapping_add(ch as u32);
    c.g[A1][0] = sx(at);
    let upper = Addr(at).lbu(c) & 1;
    c.g[V1][0] = upper;
    c.g[V0][0] = if upper != 0 {
        sx((ch as u32).wrapping_add(0x20))
    } else {
        ch
    };
    abi::ret(c);
}

const STRCASECMP_ENTRY: u32 = 0x0041_aa88;
/// The loop head the trampoline resumes at after an unwind.
const STRCASECMP_HEAD: u32 = 0x0041_aaa8;
const STRCASECMP_END: u32 = 0x0041_ab04;
const TOLOWER: Callee = Callee {
    addr: 0x0041_ea18,
    f: ee::s15::tolower,
};

/// `int strcasecmp(const char* a, const char* b)` (0x41aa88): compare
/// case-insensitively (signed chars while scanning, unsigned for the
/// result); returns `tolower(*a) - tolower(*b)` at the first difference or
/// the end of `a`. s1/s2 walk the strings; resumable at its loop head.
pub fn strcasecmp(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut advance_b = match c.pc {
            STRCASECMP_ENTRY => {
                c.cycles += 8;
                Frame::open(c, 0x40);
                frame.save(c, S1, 0x20);
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S0, 0x30);
                c.g[S2][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                false
            }
            STRCASECMP_HEAD => true,
            pc => {
                c.jump_out(pc, STRCASECMP_ENTRY, STRCASECMP_END);
                return Ok(());
            }
        };
        loop {
            if advance_b {
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            }
            advance_b = true;
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lb(c);
            if c.g[A0][0] == 0 {
                break;
            }
            c.cycles += 7;
            TOLOWER.call(c, 0x0041_aac0)?;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lb(c);
            c.g[S0][0] = c.g[V0][0];
            TOLOWER.call(c, 0x0041_aacc)?;
            if c.g[S0][0] != c.g[V0][0] {
                break;
            }
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            if back_edge_unwinds(c, STRCASECMP_HEAD) {
                return Ok(());
            }
        }
        c.cycles += 12;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lbu(c);
        TOLOWER.call(c, 0x0041_aadc)?;
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lbu(c);
        c.g[S0][0] = c.g[V0][0];
        TOLOWER.call(c, 0x0041_aae8)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Font_MeasureTextW (0x3921f0)

/// `cFont` glyph lookup: glyph count (+0), glyph table (+4, 12-byte
/// entries keyed by their first u16), default glyph (+8), first and last
/// directly indexed character (+0xc, +0x10), shadow offset (+0x28, +0x2c).
pub(crate) const FONT_GLYPH_COUNT: u32 = 0x0;
pub(crate) const FONT_GLYPHS: u32 = 0x4;
pub(crate) const FONT_DEFAULT_GLYPH: u32 = 0x8;
pub(crate) const FONT_FIRST_CHAR: u32 = 0xc;
pub(crate) const FONT_LAST_DIRECT: u32 = 0x10;
pub(crate) const FONT_SHADOW: u32 = 0x28;

/// The glyph of character `v1` (font a0, next character pointer left in a1,
/// result in t2), as `Font_MeasureTextW` inlines it: none below the first
/// character, a direct index up to the last direct one, else a binary search
/// of the table (default glyph if absent). Registers as the original leaves
/// them (a2 = the string pointer on entry).
fn font_find_glyph_inline(c: &mut Cpu, measure_head: u32) -> u64 {
    let font = c.g[A0][0] as u32;
    let ch = c.g[V1][0];
    c.cycles += 4;
    c.g[T2][0] = Addr(font + FONT_FIRST_CHAR).lw(c);
    let below = (ch as i64) < (c.g[T2][0] as i64);
    c.g[V0][0] = below as u64;
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(2));
    if below {
        c.cycles += 2;
        c.g[T2][0] = 0;
        return 0;
    }
    c.cycles += 4;
    c.g[A1][0] = Addr(font + FONT_LAST_DIRECT).lw(c);
    let searched = (c.g[A1][0] as i64) < (ch as i64);
    c.g[V0][0] = searched as u64;
    if !searched {
        // Direct: glyphs + (ch - first) * 12.
        c.cycles += 7;
        c.mult(0, V1, V1, T4);
        c.mult(0, T2, T2, T4);
        let glyphs = Addr(font + FONT_GLYPHS).lw(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(2));
        c.g[V0][0] = sx((glyphs as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
        return c.g[T2][0];
    }
    // Binary search over [lo = last - first, hi = count).
    c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T2][0] as u32));
    c.cycles += 5;
    c.g[T0][0] = Addr(font + FONT_GLYPH_COUNT).lw(c);
    c.g[T6][0] = Addr(font + FONT_GLYPHS).lw(c);
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(2));
    c.g[T1][0] = 0xc;
    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T0][0] as u32));
    loop {
        c.cycles += 6;
        c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.mult(0, A2, A3, T1);
        let entry = (c.g[A2][0] as u32).wrapping_add(c.g[T6][0] as u32);
        c.g[V0][0] = sx(entry);
        c.g[A2][0] = Addr(entry).lhu(c);
        if ch == c.g[A2][0] {
            c.g[T2][0] = sx(entry);
            return c.g[T2][0];
        }
        c.cycles += 3;
        let lower = (ch as i64) < (c.g[A2][0] as i64);
        c.g[V0][0] = lower as u64;
        if !lower {
            c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 1;
            c.g[T0][0] = c.g[A3][0];
        }
        c.cycles += 3;
        let more = (c.g[T2][0] as i64) < (c.g[T0][0] as i64);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T0][0] as u32));
        if !more {
            c.cycles += 1;
            c.g[T2][0] = Addr(font + FONT_DEFAULT_GLYPH).lw(c);
            return c.g[T2][0];
        }
        if event_due(c) {
            deliver_event_here(c, measure_head);
        }
    }
}

/// `float Font_MeasureTextW(cFont* font, const u16* str, Rect* out, int mono,
/// float sx, float sy)` (0x3921f0): the bounding box (x0, y0, x1, y1) of a
/// wide string drawn at the origin with scale (sx, sy), widened by the
/// font's shadow offset; returns its x1. `out` may be null (a stack rect).
pub fn font_measure_text_w(c: &mut Cpu) {
    c.cycles += 13;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[T5][0] = c.g[A3][0];
    c.g[T3][0] = if c.g[A2][0] != 0 {
        c.g[A2][0]
    } else {
        c.g[SP][0]
    };
    let rc = c.g[T3][0] as u32;
    Addr(rc).sw(c, 0);
    c.g[A2][0] = c.g[A1][0];
    Addr(rc + 4).sw(c, 0);
    Addr(rc + 8).sw(c, 0);
    Addr(rc + 0xc).sw(c, 0);
    c.lwc1(6, rc); // x = 0
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
    c.mov_s(7, 6); // y = 0
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[T4][0] = 0xc;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lhu(c);
        loop {
            let glyph = font_find_glyph_inline(c, 0x0039_2288);
            c.cycles += 2;
            c.g[A2][0] = c.g[A1][0];
            if glyph != 0 {
                let g = glyph as u32;
                c.cycles += 31;
                c.g[V0][0] = Addr(g + 9).lb(c);
                c.lwc1(2, rc);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.g[V1][0] = Addr(g + 8).lb(c);
                c.lwc1(3, rc + 4);
                c.mtc1(V1, 1);
                c.cvt_s_w(1, 1);
                c.lwc1(4, rc + 0xc);
                c.mul_s(0, 0, 12);
                c.mul_s(5, 1, 12); // advance * sx
                c.add_s(0, 6, 0);
                c.min_s(0, 0, 2);
                c.swc1(0, rc); // x0 = min(x + xoff * sx, x0)
                c.g[V0][0] = Addr(g + 0xa).lb(c);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 13);
                c.add_s(0, 7, 0);
                c.min_s(0, 0, 3);
                c.swc1(0, rc + 4); // y0 = min(y + yoff * sy, y0)
                let yoff = Addr(g + 0xa).lb(c);
                c.g[V1][0] = Addr(g + 3).lbu(c);
                c.g[V0][0] = sx((yoff as u32).wrapping_add(c.g[V1][0] as u32));
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 13);
                c.add_s(0, 7, 0);
                c.max_s(0, 0, 4);
                c.swc1(0, rc + 0xc); // y1 = max(y + (yoff + h) * sy, y1)
                if c.g[T5][0] != 0 {
                    // Monospaced: x1 = max(x + advance, x1).
                    c.cycles += 3;
                    c.add_s(0, 6, 5);
                    c.lwc1(1, rc + 8);
                    c.mov_s(2, 0);
                } else {
                    // x1 = max(x + (xoff + w) * sx, x1).
                    c.cycles += 10;
                    c.g[V1][0] = Addr(g + 2).lbu(c);
                    c.add_s(2, 6, 5);
                    let xoff = Addr(g + 9).lb(c);
                    c.lwc1(1, rc + 8);
                    c.g[V0][0] = sx((xoff as u32).wrapping_add(c.g[V1][0] as u32));
                    c.mtc1(V0, 0);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 12);
                    c.add_s(0, 6, 0);
                }
                c.cycles += 4;
                c.max_s(0, 0, 1);
                c.swc1(0, rc + 8);
                c.mov_s(6, 2); // x += advance
                c.g[A2][0] = c.g[A1][0];
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lhu(c);
            if event_due(c) {
                deliver_event_here(c, 0x0039_2230);
            }
        }
    }
    // x1 += x0, y1 += y0; then the shadow offset.
    let font = c.g[A0][0] as u32;
    c.cycles += 14;
    c.lwc1(2, rc + 8);
    c.lwc1(0, rc + 0xc);
    c.lwc1(4, rc);
    c.lwc1(1, rc + 4);
    c.add_s(3, 2, 4);
    c.add_s(0, 0, 1);
    c.f[2] = 0;
    c.swc1(3, rc + 8);
    c.swc1(0, rc + 0xc);
    c.lwc1(0, font + FONT_SHADOW);
    c.c_lt_s(0, 2);
    if !c.fcc() {
        c.add_s(0, 3, 0);
    } else {
        c.cycles += 4;
        c.sub_s(1, 4, 0);
        c.swc1(1, rc);
        c.lwc1(0, font + FONT_SHADOW);
        c.sub_s(0, 3, 0);
    }
    c.cycles += 7;
    c.swc1(0, rc + 8);
    c.lwc1(1, font + FONT_SHADOW + 4);
    c.f[0] = 0;
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.lwc1(0, rc + 0xc);
        c.cycles += 2;
        c.add_s(0, 0, 1);
        c.swc1(0, rc + 0xc);
    } else {
        c.cycles += 8;
        c.lwc1(0, rc + 4);
        c.lwc1(2, rc + 0xc);
        c.sub_s(0, 0, 1);
        c.swc1(0, rc + 4);
        c.lwc1(1, font + FONT_SHADOW + 4);
        c.sub_s(2, 2, 1);
        c.swc1(2, rc + 0xc);
    }
    c.cycles += 3;
    c.lwc1(0, rc + 8);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Font_DrawTextW (0x391e30)

/// `cFont` drawing fields: offset (+0x18), UV scale (+0x20), shadow offset
/// (+0x28), scale (+0x38), colour (+0x40), shadow colour (+0x50), texture
/// id (+0x60).
pub(crate) const FONT_OFFSET: u32 = 0x18;
pub(crate) const FONT_UV_SCALE: u32 = 0x20;
pub(crate) const FONT_SCALE: u32 = 0x38;
pub(crate) const FONT_TEXTURE: u32 = 0x60;

/// GrMan's text draw (vtable `+0x288`): font, string, colour at
/// `font + colour`, position (f12, f13), scale, UV scale, z = 0
/// (`sp + 8`). Registers as `Font_DrawTextW` sets them.
fn font_draw_text_w_call(c: &mut Cpu, colour: u32, ret: u32) -> crate::call::Flow {
    let grman = c.g[S3][0] as u32;
    let vt = c.g[V0][0] as u32;
    let font = c.g[S2][0] as u32;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S4][0];
    c.g[A3][0] = sx(font.wrapping_add(colour));
    let delta = Addr(vt + 0x288).lh(c);
    let func = Addr(vt + 0x28c).lw(c);
    c.g[A0][0] = sx(grman.wrapping_add(delta as u32));
    c.lwc1(14, font + FONT_SCALE);
    c.lwc1(15, font + FONT_SCALE + 4);
    c.lwc1(16, font + FONT_UV_SCALE);
    c.lwc1(17, font + FONT_UV_SCALE + 4);
    let z = sp(c) + 8;
    c.lwc1(18, z);
    if colour == 0x40 {
        c.g[V1][0] = func;
    } else {
        c.g[T0][0] = func;
    }
    call_ptr(c, func as u32, ret)
}

/// `void Font_DrawTextW(cFont* font, const u16* str, float x, float y)`
/// (0x391e30): bind the font texture, draw the string at (x, y) + offset *
/// scale through GrMan (vtable `+0x288`); with a shadow offset, first draw
/// the shadow (shadow colour, one layer lower) shifted by it.
pub fn font_draw_text_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x10);
        let font = c.g[S2][0] as u32;
        let gp = c.g[GP][0] as u32;
        let grman = Addr(gp.wrapping_add(AVALANCHE_GRMAN_GP)).lw(c);
        c.g[S3][0] = grman;
        let tex = Addr(font + FONT_TEXTURE).lw(c);
        c.g[V1][0] = tex;
        let ds = ui_vector_lw(c, grman, 0xe84);
        c.g[V0][0] = ds;
        Addr((ds as u32).wrapping_add(0x10)).sh(c, tex);
        let pos = frame.at(c, 0);
        Addr(pos + 8).sw(c, 0);
        // pos = (x + offset.x * scale.x, y + offset.y * scale.y, 0, 0).
        c.lwc1(2, font + FONT_SCALE);
        c.lwc1(0, font + FONT_OFFSET);
        c.lwc1(1, font + FONT_OFFSET + 4);
        c.lwc1(3, font + FONT_SCALE + 4);
        c.mul_s(0, 0, 2);
        c.lwc1(2, font + FONT_SHADOW);
        c.mul_s(1, 1, 3);
        c.lwc1(4, pos + 8);
        c.add_s(12, 12, 0);
        Addr(pos + 0xc).sw(c, 0);
        c.c_eq_s(2, 4);
        c.add_s(13, 13, 1);
        c.swc1(12, pos);
        let mut shadow = !c.fcc();
        c.swc1(13, pos + 4);
        if !shadow {
            c.cycles += 5;
            c.lwc1(0, font + FONT_SHADOW + 4);
            c.c_eq_s(0, 4);
            shadow = !c.fcc();
        }
        if shadow {
            // Shadow: draw-state layer (bits 5..9 of word +8) minus one.
            c.cycles += 40;
            let ds = ui_vector_lw(c, grman, 0xe84);
            c.g[T0][0] = ds;
            c.g[S1][0] = 0xffff_ffff_ffff_fc1f;
            let w = ui_vector_lw(c, ds, 8);
            c.g[S0][0] = sx(((w & 0x3e0) as u32) >> 5);
            let layer = sx((c.g[S0][0] as u32).wrapping_sub(1));
            let positive = (-1i64) < (layer as i64);
            c.g[A0][0] = positive as u64;
            let layer = if positive { layer } else { 0 };
            c.g[V0][0] = sx((layer as u32) << 5) & 0x3e0;
            c.g[V1][0] = (w & 0xffff_ffff_ffff_fc1f) | c.g[V0][0];
            Addr((ds as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
            c.g[V0][0] = ui_vector_lw(c, grman, 0x10d8);
            c.lwc1(0, pos);
            c.lwc1(12, font + FONT_SHADOW);
            c.lwc1(1, pos + 4);
            c.lwc1(13, font + FONT_SHADOW + 4);
            c.add_s(12, 0, 12);
            c.add_s(13, 1, 13);
            font_draw_text_w_call(c, 0x50, 0x0039_1f3c)?;
            let ds = ui_vector_lw(c, c.g[S3][0], 0xe84);
            c.g[V1][0] = ds;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 5);
            let w = ui_vector_lw(c, ds, 8);
            c.g[V0][0] = (w & c.g[S1][0]) | c.g[S0][0];
            Addr((ds as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        }
        let grman = c.g[S3][0];
        c.g[V0][0] = ui_vector_lw(c, grman, 0x10d8);
        c.cycles += 22;
        let pos = frame.at(c, 0);
        c.lwc1(12, pos);
        c.lwc1(13, pos + 4);
        font_draw_text_w_call(c, 0x40, 0x0039_1f90)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAThread_Yield (0x3e5398)

/// The EA thread tick rate (ticks per second, `0x450dc8`) and the "in
/// interrupt context" flag (`0x450de4`).
pub(crate) const EATHREAD_TICK_RATE: u32 = 0x0045_0dc8;
pub(crate) const EATHREAD_IN_INTERRUPT: u32 = 0x0045_0de4;

/// `void EAThread_Yield(int ms)` (0x3e5398): sleep `ms * rate / 1000` ticks
/// (at least one) with `THREAD_yieldticks`; with `ms == 0`, rotate this
/// thread's priority ready queue instead (the `i` syscalls in interrupt
/// context).
pub fn ea_thread_yield(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        if c.g[A2][0] != 0 {
            c.cycles += 6 + 9;
            c.g[A1][0] = 0x45_0000;
            c.g[V1][0] = 1000;
            c.g[A0][0] = Addr(EATHREAD_TICK_RATE).lw(c);
            c.g[V0][0] = 0;
            c.g[A1][0] = 1;
            c.mult(0, A0, A2, A0);
            c.div(0, A0, V1);
            c.g[A0][0] = c.lo[0];
            let positive = 0 < (c.g[A0][0] as i64);
            c.g[V0][0] = positive as u64;
            if !positive {
                c.g[A0][0] = 1;
            }
            Callee {
                addr: 0x003e_52b0,
                f: ee::s14::THREAD_yieldticks,
            }
            .call(c, 0x003e_53dc)?;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            let in_interrupt = Addr(EATHREAD_IN_INTERRUPT).lw(c);
            c.g[V1][0] = in_interrupt;
            // (status call, rotate call, return addresses)
            let (refer, rotate, rets) = if in_interrupt != 0 {
                c.cycles += 9;
                (
                    0x0042_3cb0,
                    0x0042_4ce8,
                    [0x003e_53fc, 0x003e_5408, 0x003e_5410],
                )
            } else {
                c.cycles += 8;
                (
                    0x0042_3ca0,
                    0x0042_3c50,
                    [0x003e_5420, 0x003e_542c, 0x003e_5434],
                )
            };
            call_ptr(c, 0x0042_3c90, rets[0])?; // GetThreadId
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, refer, rets[1])?; // (i)ReferThreadStatus(id, &info)
            let priority = frame.at(c, 0x18);
            c.g[A0][0] = Addr(priority).lw(c);
            call_ptr(c, rotate, rets[2])?; // (i)RotateThreadReadyQueue(info.currentPriority)
        }
        frame.restore_ra(c, 0x30);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// tWPIGD_LightGlow_Blend (0x2bd068)

/// The light glow block: 7 (current, target) float pairs from `+0x8`.
pub(crate) const LIGHT_GLOW_PAIRS: u32 = 0x8;

/// Call the virtual at vtable `+0x218` (`this` at `+4`'s vtable), returning to `ret`.
fn light_glow_notify(c: &mut Cpu, ret: u32, vt_reg: usize, fn_reg: usize) -> crate::call::Flow {
    let this = c.g[S0][0] as u32;
    let vt = Addr(this + 4).lw(c);
    c.g[vt_reg][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(0x218)).lh(c);
    let func = Addr((vt as u32).wrapping_add(0x21c)).lw(c);
    c.g[fn_reg][0] = func;
    c.g[A0][0] = sx(this.wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

/// `void tWPIGD::LightGlow_Blend(tWPIGD* this, const float src[8], float t)`
/// (0x2bd068): notify (vtable `+0x218`), then for the 7 light glow values
/// `cur = t² * src + (1 - t²) * cur`, `target = src` (src from `+4`), and
/// notify again.
pub fn t_wpigd_light_glow_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 84;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.mul_s(20, 20, 20); // w = t * t
        light_glow_notify(c, 0x002b_d0a0, V1, V0)?;
        let this = c.g[S0][0] as u32;
        let src = c.g[S1][0] as u32;
        let cur = |k: u32| this + LIGHT_GLOW_PAIRS + 8 * k;
        let val = |k: u32| src + 4 + 4 * k;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.lwc1(1, val(0));
        c.sub_s(2, 2, 20); // 1 - w
                           // (1 - w) * cur for all seven first (f0, f3, f5, f6, f7, f8, f2).
        c.lwc1(0, cur(0));
        c.mul_s(1, 20, 1);
        c.lwc1(3, cur(1));
        c.lwc1(5, cur(2));
        c.mul_s(0, 2, 0);
        c.lwc1(6, cur(3));
        c.mul_s(3, 2, 3);
        c.lwc1(7, cur(4));
        c.mul_s(5, 2, 5);
        c.lwc1(8, cur(5));
        c.add_s(1, 1, 0);
        c.lwc1(4, cur(6));
        c.mul_s(6, 2, 6);
        c.mul_s(7, 2, 7);
        c.swc1(1, cur(0));
        c.mul_s(8, 2, 8);
        c.mul_s(2, 2, 4);
        c.lwc1(1, val(0));
        c.swc1(1, cur(0) + 4);
        // Pairs 1..5: cur = w * src + (1 - w) * cur; target = src.
        for (k, kept) in [(1u32, 3usize), (2, 5), (3, 6), (4, 7), (5, 8)] {
            c.lwc1(0, val(k));
            c.mul_s(0, 20, 0);
            c.add_s(0, 0, kept);
            c.swc1(0, cur(k));
            c.lwc1(1, val(k));
            c.swc1(1, cur(k) + 4);
        }
        c.lwc1(0, val(6));
        c.mul_s(20, 20, 0);
        c.add_s(20, 20, 2);
        c.swc1(20, cur(6));
        c.lwc1(0, val(6));
        c.swc1(0, cur(6) + 4);
        light_glow_notify(c, 0x002b_d1a0, V0, V1)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cUIGroup_Render (0x398868)

/// `cUIGroup`'s child list (`cList` at +0x74).
pub(crate) const UI_GROUP_CHILDREN: u32 = 0x74;
const UI_LIST_FIRST: Callee = Callee {
    addr: 0x0039_7840,
    f: ee::s13::UIList_First,
};
const UI_LIST_NODE_IS_SENTINEL: Callee = Callee {
    addr: 0x0039_7718,
    f: ee::s13::UIListNode_IsSentinel,
};

/// `void cUIGroup::render2D(cUIGroup* this, float x, float y)` (0x398868):
/// render each visible child (flags `+0x14` bit 6) at the group's position
/// plus (x, y) through its vtable `+0x88`.
pub fn c_ui_group_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        frame.save(c, S0, 0x20);
        c.mov_s(20, 13);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(UI_GROUP_CHILDREN));
        UI_LIST_FIRST.call(c, 0x0039_8894)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
            loop {
                c.cycles += 4;
                let node = c.g[S0][0];
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32) & 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 10;
                    let this = c.g[S1][0] as u32;
                    let vt = ui_vector_lw(c, node, 8);
                    c.g[V1][0] = vt;
                    c.lwc1(12, this + 0x44);
                    c.lwc1(13, this + 0x48);
                    let delta = Addr((vt as u32).wrapping_add(0x88)).lh(c);
                    c.add_s(12, 12, 21);
                    c.add_s(13, 13, 20);
                    let func = ui_vector_lw(c, vt, 0x8c);
                    c.g[V0][0] = func;
                    c.g[A0][0] = sx((node as u32).wrapping_add(delta as u32));
                    call_ptr(c, func as u32, 0x0039_88dc)?;
                }
                let node = c.g[S0][0];
                c.g[S0][0] = ui_vector_lw(c, node, 4);
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_88e8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_88a8);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 6;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BEPreRace_GetInterface (0x1565c8)

/// The jump table of `BEPreRace_GetInterface`.
pub(crate) const BE_PRE_RACE_INTERFACE_TABLE: u32 = 0x0045_a900;

/// The interface singletons by kind: (getter, its case block address).
const BE_PRE_RACE_INTERFACES: [(Callee, u32); 14] = [
    (
        Callee {
            addr: 0x0014_48d8,
            f: ee::s01::BENewRaceInt_GetInstance,
        },
        0x0015_65f4,
    ),
    (
        Callee {
            addr: 0x0014_5970,
            f: ee::s01::BENewPlayerInt_GetInstance,
        },
        0x0015_6604,
    ),
    (
        Callee {
            addr: 0x0014_edd8,
            f: ee::s01::BE_GetCharDBInterface,
        },
        0x0015_6614,
    ),
    (
        Callee {
            addr: 0x0014_7f78,
            f: ee::s01::BECharInt_GetInstance,
        },
        0x0015_6624,
    ),
    (
        Callee {
            addr: 0x0014_f250,
            f: ee::s01::BEOptionInt_GetInstance,
        },
        0x0015_6634,
    ),
    (
        Callee {
            addr: 0x0015_2638,
            f: ee::s01::BESaveInt_GetInstance,
        },
        0x0015_6644,
    ),
    (
        Callee {
            addr: 0x0014_f900,
            f: ee::s01::BEBAGTInt_GetInstance,
        },
        0x0015_6684,
    ),
    (
        Callee {
            addr: 0x0014_e0e0,
            f: ee::s01::BENetworkInt_GetInstance,
        },
        0x0015_6654,
    ),
    (
        Callee {
            addr: 0x0015_4948,
            f: ee::s01::BEScoreInt_GetInstance,
        },
        0x0015_6664,
    ),
    (
        Callee {
            addr: 0x0014_abe0,
            f: ee::s01::BEBoltInt_GetInstance,
        },
        0x0015_6674,
    ),
    (
        Callee {
            addr: 0x0015_32a0,
            f: ee::s01::BEMissionInt_GetInstance,
        },
        0x0015_66a4,
    ),
    (
        Callee {
            addr: 0x0015_08d8,
            f: ee::s01::BEEconInt_GetInstance,
        },
        0x0015_6694,
    ),
    (
        Callee {
            addr: 0x0015_5a50,
            f: ee::s01::BEAggressionInt_GetInstance,
        },
        0x0015_66b4,
    ),
    (
        Callee {
            addr: 0x0015_69c0,
            f: ee::s01::BERewardInt_GetInstance,
        },
        0x0015_66c4,
    ),
];

/// `void* BEPreRace_GetInterface(void* this, int kind)` (0x1565c8): the
/// back-end interface singleton of `kind` (new race, new player, character
/// DB, characters, options, save, BAGT, network, score, bolt, mission,
/// economy, aggression, reward), or null for other kinds.
pub fn be_pre_race_get_interface(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let kind = c.g[A1][0];
        c.g[V0][0] = (kind < 0xe) as u64;
        frame.save_ra(c, 0);
        if kind < 0xe {
            c.cycles += 7;
            let entry = BE_PRE_RACE_INTERFACE_TABLE.wrapping_add((kind as u32) << 2);
            c.g[V0][0] = sx(BE_PRE_RACE_INTERFACE_TABLE);
            c.g[V1][0] = sx(entry);
            c.g[A0][0] = Addr(entry).lw(c);
            let (getter, block) = BE_PRE_RACE_INTERFACES[kind as usize];
            c.cycles += 4;
            getter.call(c, block + 8)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ParentModifier_UpdateTransform (0x357108)

/// `cParentModifier`: local position (+0x30), dirty flag (+0x44), parent
/// node matrix (+0x50), transform translation row (+0x80), mesh/node index
/// (+0x90), parent instance (+0xa0).
pub(crate) const PARENT_MOD_LOCAL_POS: u32 = 0x30;
pub(crate) const PARENT_MOD_DIRTY: u32 = 0x44;
pub(crate) const PARENT_MOD_MATRIX: u32 = 0x50;
pub(crate) const PARENT_MOD_TRANSLATION: u32 = 0x80;
pub(crate) const PARENT_MOD_NODE: u32 = 0x90;
pub(crate) const PARENT_MOD_INSTANCE: u32 = 0xa0;

/// `void ParentModifier_UpdateTransform(cParentModifier* this)` (0x357108):
/// with a parent object (instance `+0xc`), fetch the parent node's matrix
/// (the object's own node matrix, vtable `+0xe8`, when it has one, vtable
/// `+0xd0`; else `Instance_GetNodeMatrix`), then translation += matrix *
/// local position (VU0). Clears the dirty flag either way.
pub fn parent_modifier_update_transform(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        let this = c.g[S1][0] as u32;
        let inst = Addr(this + PARENT_MOD_INSTANCE).lw(c);
        c.g[V0][0] = inst;
        let obj = ui_vector_lw(c, inst, 0xc);
        c.g[S0][0] = obj;
        if obj != 0 {
            c.cycles += 7;
            let vt = ui_vector_lw(c, obj, 0xc);
            c.g[V1][0] = vt;
            let delta = Addr((vt as u32).wrapping_add(0xd0)).lh(c);
            let func = ui_vector_lw(c, vt, 0xd4);
            c.g[V0][0] = func;
            c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x0035_7140)?;
            let this = c.g[S1][0] as u32;
            if c.g[V0][0] != 0 {
                // The object's own node matrix, copied to +0x50.
                c.cycles += 17;
                let obj = c.g[S0][0];
                let vt = ui_vector_lw(c, obj, 0xc);
                c.g[V1][0] = vt;
                c.g[A1][0] = Addr(this + PARENT_MOD_NODE).lw(c);
                let delta = Addr((vt as u32).wrapping_add(0xe8)).lh(c);
                let func = ui_vector_lw(c, vt, 0xec);
                c.g[V0][0] = func;
                c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
                call_ptr(c, func as u32, 0x0035_7160)?;
                let this = c.g[S1][0] as u32;
                let m = this + PARENT_MOD_MATRIX;
                c.g[V1][0] = sx(m);
                crate::game::object::instancenode::vu0_copy_matrix(c, c.g[V0][0] as u32, m);
            } else {
                c.g[S0][0] = sx(this + PARENT_MOD_MATRIX);
                c.cycles += 6;
                c.g[A0][0] = Addr(this + PARENT_MOD_INSTANCE).lw(c);
                c.g[A1][0] = Addr(this + PARENT_MOD_NODE).lw(c);
                c.g[A2][0] = c.g[S0][0];
                Callee {
                    addr: 0x0034_fed8,
                    f: ee::s11::Instance_GetNodeMatrix,
                }
                .call(c, 0x0035_719c)?;
                c.g[V1][0] = c.g[S0][0];
            }
            // translation += M * (local position).
            let this = c.g[S1][0] as u32;
            let m = c.g[V1][0] as u32;
            c.lq(V0, this + PARENT_MOD_TRANSLATION);
            c.cycles += 21;
            let tmp = frame.at(c, 0);
            c.sq(V0, tmp + 0x10);
            c.lqc2(8, this + PARENT_MOD_LOCAL_POS);
            for i in 0..4u32 {
                c.lqc2(4 + i as usize, m.wrapping_add(0x10 * i));
            }
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, tmp + 0x30);
            c.lq(V0, tmp + 0x30);
            c.sq(V0, tmp + 0x20);
            c.lqc2(3, tmp + 0x10);
            c.lqc2(4, tmp + 0x20);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, tmp + 0x30);
            c.lq(V0, tmp + 0x30);
            c.sq(V0, this + PARENT_MOD_TRANSLATION);
            c.sq(V0, tmp);
        }
        let this = c.g[S1][0] as u32;
        Addr(this + PARENT_MOD_DIRTY).sw(c, 0);
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObj_BuildMatrices (0x350288)

/// One bone of `cInstanceNode::makeAnimMats`: the bone's local matrix (at
/// `a0`) with its translation row scaled by `scale` (on the stack at `sp`),
/// times `parent` (VU0, into `sp + 0xb0`), copied through `sp + 0x10` to
/// `out`. `swap_xy`: the root case loads the x/y scale into f6/f7 instead of
/// f7/f6.
fn world_obj_build_bone(c: &mut Cpu, parent: u32, out: u32, swap_xy: bool) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    let sp = sp(c);
    let bone = Addr((c.g[A3][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = bone;
    let bone = bone as u32;
    let (fx, fy) = if swap_xy { (6, 7) } else { (7, 6) };
    c.lwc1(fx, sp);
    c.lq(V0, bone);
    c.lwc1(fy, sp + 4);
    c.sq(V0, sp + 0x90);
    c.sq(V0, sp + 0x50);
    c.lwc1(5, sp + 8);
    c.lq(V0, bone + 0x10);
    c.lwc1(4, sp + 0xc);
    c.sq(V0, sp + 0x90);
    c.sq(V0, sp + 0x60);
    c.lq(V1, bone + 0x20);
    c.sq(V1, sp + 0x90);
    c.sq(V1, sp + 0x70);
    c.lq(V0, bone + 0x30);
    c.sq(V0, sp + 0xa0);
    // Translation row * (s, s, s, 1).
    c.lwc1(1, sp + 0xa0);
    c.lwc1(2, sp + 0xa4);
    c.lwc1(3, sp + 0xa8);
    c.mul_s(1, 1, fx);
    c.lwc1(0, sp + 0xac);
    c.mul_s(2, 2, fy);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, sp + 0x90);
    c.swc1(2, sp + 0x94);
    c.swc1(3, sp + 0x98);
    c.swc1(0, sp + 0x9c);
    c.lq(V0, sp + 0x90);
    c.sq(V0, sp + 0x80);
    // (sp+0xb0) = local (sp+0x50) * parent.
    let local = c.g[T5][0] as u32;
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, parent.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, local.wrapping_add(0x10 * i));
    }
    for i in 0..4u8 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + i, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + i, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + i, Z));
        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(8 + i, W));
    }
    let product = c.g[T0][0] as u32;
    let copy = c.g[T1][0] as u32;
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, product + 0x10 * i);
    }
    crate::game::object::instancenode::vu0_copy_matrix(c, product, copy);
    crate::game::object::instancenode::vu0_copy_matrix(c, copy, out);
}

/// `void cInstanceNode::makeAnimMats(cInstanceNode* this, const t4Matrix*
/// root, t4Matrix* out)` (0x350288): for each skeleton bone (16-byte
/// entries: parent index, ..., local matrix pointer at +0xc), its animation
/// matrix = local matrix (translation scaled by the instance scale) times
/// its parent's (or `root` for parentless bones).
pub fn world_obj_build_matrices(c: &mut Cpu) {
    c.cycles += 15;
    let inst = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
    c.g[V0][0] = inst;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xf0));
    c.g[AT][0] = 0x3f80_0000;
    c.f[1] = 0x3f80_0000;
    c.g[T7][0] = c.g[A1][0];
    let skel = ui_vector_lw(c, inst, 0x80);
    c.g[T6][0] = skel;
    c.lwc1(0, (inst as u32).wrapping_add(0x84));
    c.g[A3][0] = ui_vector_lw(c, skel, 8);
    let sp = sp(c);
    c.swc1(0, sp + 8);
    c.swc1(1, sp + 0xc);
    c.swc1(0, sp);
    c.swc1(0, sp + 4);
    c.g[V0][0] = ui_vector_lw(c, skel, 4);
    c.g[T4][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 7;
        c.g[T2][0] = c.g[A2][0];
        c.g[T8][0] = u64::MAX;
        c.g[T1][0] = sx(sp + 0x10);
        c.g[T0][0] = sx(sp + 0xb0);
        c.g[T5][0] = sx(sp + 0x50);
        c.g[T3][0] = c.g[T2][0];
        loop {
            c.cycles += 3;
            let parent = Addr(c.g[A3][0] as u32).lw(c);
            c.g[A1][0] = sx((parent as u32) << 6);
            if parent != c.g[T8][0] {
                c.cycles += 77;
                let pm = (c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32);
                c.g[A1][0] = sx(pm);
                let out = c.g[T2][0] as u32;
                world_obj_build_bone(c, pm, out, false);
            } else {
                c.cycles += 75;
                let root = c.g[T7][0] as u32;
                let out = c.g[T3][0] as u32;
                world_obj_build_bone(c, root, out, true);
            }
            let skel = c.g[T6][0];
            c.g[V0][0] = ui_vector_lw(c, skel, 4);
            c.cycles += 6;
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x40));
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x40));
            let more = (c.g[T4][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_02e0);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp.wrapping_add(0xf0));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_PatchInstanceDmaChain (0x3ac508)

#[cfg(test)]
const CHUNK_DMA_ENTRY: u32 = 0x003a_c508;

/// The DMA tag id (bits 28..30) of the doubleword `tag`.
#[inline]
fn chunk_dma_tag_id(tag: u64) -> u64 {
    (((tag << 4) as i64) >> 32) as u64 & 7
}

/// Relocate the REF tag at a3 by the word at `base` (`addr += *base`),
/// step a3 past it and point t0 at the last quadword it references
/// (t1 = its qword count; a0, v0, v1 as the original leaves them).
fn chunk_dma_ref_patch(c: &mut Cpu, base: u32) {
    let tag_at = c.g[A3][0] as u32;
    c.g[A0][0] = sx(tag_at.wrapping_add(4));
    c.g[V1][0] = Addr(tag_at.wrapping_add(4)).lw(c);
    let b = Addr(base).lw(c);
    let tag = Addr(tag_at).ld(c);
    c.g[V0][0] = sx((b as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(tag_at.wrapping_add(4)).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx(tag as u32);
    c.g[T1][0] = c.g[V1][0] & 0xffff;
    c.g[A3][0] = sx(tag_at.wrapping_add(0x10));
    let addr = Addr(tag_at.wrapping_add(4)).lw(c);
    c.g[V0][0] = sx(((c.g[T1][0] as u32).wrapping_sub(1)) << 4);
    c.g[T0][0] = sx((addr as u32).wrapping_add(c.g[V0][0] as u32));
}

/// Store the VIF "return" code (first block: t7, others: t6) in the last
/// quadword of the referenced packet (`t0 + 8`).
fn chunk_dma_store_ret(c: &mut Cpu) {
    let t0 = c.g[T0][0] as u32;
    if c.g[T4][0] != 0 {
        c.cycles += 2;
        Addr(t0.wrapping_add(8)).sw(c, c.g[T7][0]);
    } else {
        c.cycles += 1;
        Addr(t0.wrapping_add(8)).sw(c, c.g[T6][0]);
    }
}

/// `void* cPS2ChunkResolve::PatchInstanceDmaChain(void* this, Instance* a1,
/// Chunk* chunk, u64* tag)` (0x3ac508): walk a DMA chain up to its END tag
/// (id 6), relocating each CALL/REF pair (id 3): the first address by the
/// chunk base (`chunk + 0x24`), the second by the instance's (`a1 + 0x98`),
/// marking the last quadword of each referenced packet with a VIF return
/// (`0x14000000 | addr >> 3`, different for the first block). Tags flagged
/// in bit 31 bring a second pair, or, before an END, are rewritten into a
/// short CNT/REF/END sequence. Returns the tag after the END.
pub fn c_ps2_chunk_resolve_patch_instance_dma_chain(c: &mut Cpu) {
    c.cycles += 28;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let sp = sp(c);
    c.sq(S0, sp + 0x30);
    c.sq(S1, sp + 0x20);
    c.sq(S2, sp + 0x10);
    c.sq(S3, sp);
    c.g[T9][0] = 0x2270 >> 3;
    c.g[T8][0] = 0x22c8 >> 3;
    c.g[A0][0] = 0x0050_9400u64 << 32;
    c.g[V1][0] = 0x1400_0000;
    c.g[T5][0] = 0x7fff_ffff;
    c.g[V0][0] = 0x3000_0001;
    c.g[T6][0] = c.g[T8][0] | 0x1400_0000;
    c.g[T7][0] = c.g[T9][0] | 0x1400_0000;
    c.g[S3][0] = c.g[A0][0] | 0x3000_0001;
    c.g[S2][0] = 0;
    c.g[T4][0] = 1;
    c.g[S1][0] = 6;
    c.g[S0][0] = 0xffff_ffff_8000_0000;
    c.g[T0][0] = Addr(c.g[A3][0] as u32).ld(c);
    let chunk_base = (c.g[A2][0] as u32).wrapping_add(0x24);
    let inst_base = (c.g[A1][0] as u32).wrapping_add(0x98);
    loop {
        c.cycles += 6;
        c.g[V1][0] = 3;
        let id = chunk_dma_tag_id(c.g[T0][0]);
        c.g[V0][0] = id;
        if id == 3 {
            let a3 = c.g[A3][0] as u32;
            c.g[V1][0] = Addr(a3 + 4).lw(c);
            // Flag bit 31 of the address: a second pair follows.
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & c.g[S0][0];
            c.g[T2][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & c.g[T5][0];
                c.g[T2][0] = 1;
                Addr(a3 + 4).sw(c, c.g[V0][0]);
            }
            c.cycles += 20;
            c.g[V1][0] = Addr(a3 + 4).lw(c);
            let b = Addr(chunk_base).lw(c);
            c.g[V0][0] = sx((b as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(a3 + 4).sw(c, c.g[V0][0]);
            c.g[A3][0] = sx(a3 + 0x10);
            chunk_dma_ref_patch(c, inst_base);
            chunk_dma_store_ret(c);
            c.cycles += 2;
            if c.g[T2][0] != 0 {
                chunk_dma_patch_flagged(c, chunk_base, inst_base);
            }
            c.g[T4][0] = 0;
        } else {
            c.cycles += 2;
            if id == c.g[S1][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                c.cycles += 1;
                c.g[S2][0] = 1;
            } else {
                c.cycles += 2;
            }
        }
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            break;
        }
        c.g[T0][0] = Addr(c.g[A3][0] as u32).ld(c);
        if event_due(c) {
            deliver_event_here(c, 0x003a_c578);
        }
    }
    c.cycles += 7;
    c.lq(S0, sp + 0x30);
    c.g[V0][0] = c.g[A3][0];
    c.lq(S1, sp + 0x20);
    c.lq(S2, sp + 0x10);
    c.lq(S3, sp);
    c.g[SP][0] = sx(sp.wrapping_add(0x40));
    abi::ret(c);
}

/// The second part of a flagged CALL/REF pair (a3 at the tag after it).
fn chunk_dma_patch_flagged(c: &mut Cpu, chunk_base: u32, inst_base: u32) {
    c.cycles += 6;
    let a3 = c.g[A3][0] as u32;
    let next = Addr(a3 + 0x10).ld(c);
    c.g[V0][0] = chunk_dma_tag_id(next);
    c.g[A0][0] = sx(a3 + 0x10);
    if c.g[V0][0] == c.g[S1][0] {
        // Before an END: rewrite as CNT(1) -> s3, REF(qwc, addr), END.
        c.cycles += 11;
        let tag = Addr(a3).ld(c);
        c.g[T0][0] = tag;
        let b = Addr(chunk_base).lw(c);
        c.g[V0][0] = sx(tag as u32);
        c.g[V1][0] = Addr(a3 + 4).lw(c);
        c.g[T1][0] = c.g[V0][0] & 0xffff;
        c.g[V0][0] = sx(((c.g[T1][0] as u32).wrapping_sub(1)) << 4);
        c.g[A0][0] = sx((b as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        chunk_dma_store_ret(c);
        c.cycles += 13;
        c.g[A0][0] <<= 32;
        c.g[V1][0] = 0x6000_0000;
        c.g[V0][0] = 0x3000_0000 | c.g[T1][0] | c.g[A0][0];
        Addr(a3 + 0x20).sd(c, c.g[V1][0]);
        Addr(a3 + 0x10).sd(c, c.g[V0][0]);
        Addr(a3).sd(c, c.g[S3][0]);
        Addr(a3 + 8).sd(c, 0);
        Addr(a3 + 0x18).sd(c, 0);
        Addr(a3 + 0x28).sd(c, 0);
        c.g[A3][0] = sx(a3 + 0x20);
    } else {
        // Three more references: chunk, instance, chunk.
        c.cycles += 4;
        c.g[V1][0] = Addr(a3 + 4).lw(c);
        c.g[V0][0] = c.g[V1][0] & c.g[S0][0];
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & c.g[T5][0];
        if flagged {
            c.cycles += 2;
            Addr(a3 + 4).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(a3 + 4).lw(c);
        }
        c.cycles += 36;
        c.g[T3][0] = 0x1400_0000;
        let b = Addr(chunk_base).lw(c);
        c.g[T2][0] = c.g[T8][0] | 0x1400_0000;
        c.g[V0][0] = sx((b as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(a3 + 4).sw(c, c.g[V0][0]);
        c.g[A3][0] = c.g[A0][0];
        chunk_dma_ref_patch(c, inst_base);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
        Addr((c.g[T0][0] as u32).wrapping_add(8)).sw(c, c.g[T2][0]);
        chunk_dma_ref_patch(c, chunk_base);
        if c.g[T4][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.g[T9][0] | c.g[T3][0];
            Addr((c.g[T0][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 1;
            Addr((c.g[T0][0] as u32).wrapping_add(8)).sw(c, c.g[T2][0]);
        }
        c.cycles += 15;
        chunk_dma_ref_patch(c, chunk_base);
        Addr((c.g[T0][0] as u32).wrapping_add(8)).sw(c, c.g[T6][0]);
    }
    c.cycles += 2;
}

// ---------------------------------------------------------------------------
// BoltFileIndexManager_FindNode (0x19c938)

/// `BoltFileIndexManager`: 400 nodes of 16 bytes (key +0, prev index +0xc,
/// next index +0xe), 400 bucket heads at +0x1900, the free list at +0x1f40.
pub(crate) const BOLT_INDEX_BUCKETS: u32 = 0x1900;
pub(crate) const BOLT_INDEX_FREE: u32 = 0x1f40;
const BOLT_INDEX_ALLOC_NODE: Callee = Callee {
    addr: 0x0019_c880,
    f: ee::s03::BoltFileIndexManager_AllocNode,
};
const BOLT_INDEX_NODE_INDEX: Callee = Callee {
    addr: 0x0019_c920,
    f: ee::s03::BoltFileIndexManager_NodeIndex,
};
const BOLT_INDEX_GET_NEXT_NODE: Callee = Callee {
    addr: 0x0019_c8f8,
    f: ee::s03::BoltFileIndexManager_GetNextNode,
};

/// `Node* BoltFileIndexManager_FindNode(BoltFileIndexManager* m, int hi,
/// int lo, bool create)` (0x19c938): the node of key `hi << 16 | lo` in
/// bucket `key % 400`; if absent and `create` (and a free node exists), a
/// new node appended to the bucket's chain. The found or new node's key is
/// (re)written; returns it or null.
pub fn bolt_file_index_manager_find_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 16);
        frame.save(c, S4, 0x20);
        c.g[V0][0] = 400;
        c.g[S4][0] = c.g[A1][0] | c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.div(0, S4, V0);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A3][0];
        // The bucket's head.
        c.cycles += 6;
        let m = c.g[S1][0] as u32;
        let free = Addr(m + BOLT_INDEX_FREE).lw(c);
        c.g[V1][0] = free;
        c.g[S3][0] = c.hi[0];
        let bucket = m.wrapping_add((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx(bucket);
        c.g[S0][0] = Addr(bucket.wrapping_add(BOLT_INDEX_BUCKETS)).lw(c);
        let mut searching = true;
        if free == 0 {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.g[V0][0] = 0;
                searching = false;
            } else {
                c.cycles += 1;
            }
        }
        if searching {
            'walk: loop {
                // At a node (or past the chain's end).
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.cycles += 2;
                    if c.g[S5][0] != 0 {
                        bolt_index_append_node(c)?;
                    }
                    break 'walk;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                loop {
                    c.cycles += 2;
                    let found = c.g[V0][0] == c.g[S4][0];
                    c.g[S2][0] = c.g[S0][0];
                    if found {
                        break 'walk;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    BOLT_INDEX_GET_NEXT_NODE.call(c, 0x0019_ca28)?;
                    c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BOLT_INDEX_FREE)).lw(c);
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[V1][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_c9a0);
                        }
                        continue 'walk;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        break 'walk;
                    }
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_ca14);
                    }
                }
            }
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Take a free node and link it after s2 (the chain's last node), or as
/// the bucket's head (bucket s3) when the chain is empty; s0 = the node.
fn bolt_index_append_node(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 8;
    c.g[A0][0] = c.g[S1][0];
    BOLT_INDEX_ALLOC_NODE.call(c, 0x0019_c9b8)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    BOLT_INDEX_NODE_INDEX.call(c, 0x0019_c9c8)?;
    Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sh(c, c.g[V0][0]);
    if c.g[S2][0] != 0 {
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        BOLT_INDEX_NODE_INDEX.call(c, 0x0019_c9dc)?;
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sh(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        BOLT_INDEX_NODE_INDEX.call(c, 0x0019_c9ec)?;
        Addr((c.g[S2][0] as u32).wrapping_add(0xe)).sh(c, c.g[V0][0]);
    } else {
        c.cycles += 5;
        let bucket = (c.g[S1][0] as u32).wrapping_add((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx(bucket);
        c.g[V1][0] = u64::MAX;
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sh(c, u64::MAX);
        Addr(bucket.wrapping_add(BOLT_INDEX_BUCKETS)).sw(c, c.g[S0][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    Addr((c.g[S0][0] as u32).wrapping_add(0xe)).sh(c, u64::MAX);
    Ok(())
}

// ---------------------------------------------------------------------------
// UIState_Update (0x39e510)

/// `UIState`: vtable (+8), owner (+0x10, its pad at +0xc), current pad
/// index (+0x14, -1 when idle), pad mask (+0x15), screen list (+0x24).
pub(crate) const UI_STATE_PAD_INDEX: u32 = 0x14;
pub(crate) const UI_STATE_PAD_MASK: u32 = 0x15;
pub(crate) const UI_STATE_SCREENS: u32 = 0x24;

/// `obj->vt[slot](obj + delta)` with `vt = *(obj + 8)`; the vtable pointer
/// goes to `vt_reg`, the function to `fn_reg` (as the original leaves them).
fn ui_state_vcall(
    c: &mut Cpu,
    obj: u64,
    slot: u32,
    vt_reg: usize,
    fn_reg: usize,
    ret: u32,
) -> crate::call::Flow {
    let vt = ui_vector_lw(c, obj, 8);
    c.g[vt_reg][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(slot)).lh(c);
    let func = ui_vector_lw(c, vt, slot + 4);
    c.g[fn_reg][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

/// `void UIState_Update(UIState* this)` (0x39e510): for pads 0 and 1 (the
/// mask at +0x15 enabling the second test per pad): select the pad, and if
/// it has input (pad vtable `+0x168`) and pressed "accept" (`+0x148`), or
/// "alt" (`+0x140`) with the mask bit, offer the input to the state
/// (vtable `+0x40`), else to each active screen (`+0x20`) until one takes
/// it; then let the state react (`+0x48`) and stop. The pad index ends -1.
pub fn ui_state_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        let owner = ui_vector_lw(c, this, 0x10);
        c.g[V0][0] = owner;
        c.g[S0][0] = Addr((this as u32).wrapping_add(UI_STATE_PAD_MASK)).lb(c);
        c.g[S1][0] = ui_vector_lw(c, owner, 0xc);
        Addr((this as u32).wrapping_add(UI_STATE_PAD_INDEX)).sb(c, 0);
        loop {
            c.cycles += 13;
            let this = c.g[S2][0];
            let pad = c.g[S1][0];
            let vt = ui_vector_lw(c, pad, 8);
            c.g[V0][0] = vt;
            c.g[A1][0] = Addr((this as u32).wrapping_add(UI_STATE_PAD_INDEX)).lbu(c);
            let delta = Addr((vt as u32).wrapping_add(0x160)).lh(c);
            let func = ui_vector_lw(c, vt, 0x164);
            c.g[A2][0] = func;
            c.g[A0][0] = sx((pad as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x0039_e550)?;
            ui_state_vcall(c, c.g[S1][0], 0x168, V1, V0, 0x0039_e564)?;
            let mut handle = false;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                ui_state_vcall(c, c.g[S1][0], 0x148, V1, V0, 0x0039_e580)?;
                if c.g[V0][0] != 0 {
                    handle = true;
                } else {
                    c.cycles += 7;
                    ui_state_vcall(c, c.g[S1][0], 0x140, V1, V0, 0x0039_e59c)?;
                    let alt = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0] & 1;
                    if !alt {
                        c.cycles += 1;
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.cycles += 1;
                            handle = true;
                        }
                    }
                }
            }
            if handle {
                c.g[V1][0] = ui_vector_lw(c, c.g[S2][0], 8);
                ui_state_handle_input(c)?;
                c.cycles += 7;
                c.g[A1][0] = c.g[S1][0];
                ui_state_vcall(c, c.g[S2][0], 0x48, V0, V1, 0x0039_e63c)?;
                break;
            }
            let this = c.g[S2][0] as u32;
            c.g[V1][0] = Addr(this + UI_STATE_PAD_INDEX).lbu(c);
            c.cycles += 7;
            c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx(((c.g[V1][0] as u32) << 24) as i32 as u32 >> 0);
            c.g[V0][0] = sx((((c.g[V1][0] as u32) << 24) as i32 >> 24) as u32);
            let more = (c.g[V0][0] as i64) < 2;
            c.g[V0][0] = more as u64;
            Addr(this + UI_STATE_PAD_INDEX).sb(c, c.g[V1][0]);
            if !more {
                c.cycles += 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_e538);
            }
        }
        c.g[V0][0] = u64::MAX;
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        Addr((c.g[S2][0] as u32).wrapping_add(UI_STATE_PAD_INDEX)).sb(c, u64::MAX);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Offer the pad (s1) to the state (vtable in v1, slot `+0x40`), then to
/// the active screens until one takes it; leaves v0 = the state's vtable.
fn ui_state_handle_input(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 7;
    c.g[A1][0] = c.g[S1][0];
    let this = c.g[S2][0];
    let vt = c.g[V1][0];
    let delta = Addr((vt as u32).wrapping_add(0x40)).lh(c);
    let func = ui_vector_lw(c, vt, 0x44);
    c.g[V0][0] = func;
    c.g[A0][0] = sx((this as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, 0x0039_e5c4)?;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(UI_STATE_SCREENS));
        UI_LIST_FIRST.call(c, 0x0039_e5d4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
            loop {
                c.cycles += 3;
                c.g[V0][0] &= 1;
                c.g[A1][0] = c.g[S1][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    ui_state_vcall(c, c.g[S0][0], 0x20, V1, V0, 0x0039_e608)?;
                    if c.g[V0][0] != 0 {
                        break;
                    }
                }
                c.cycles += 5;
                let node = c.g[S0][0];
                c.g[S0][0] = ui_vector_lw(c, node, 4);
                c.g[A0][0] = c.g[S0][0];
                UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_e61c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_e5e8);
                }
            }
        }
    }
    c.g[V0][0] = ui_vector_lw(c, c.g[S2][0], 8);
    Ok(())
}

// ---------------------------------------------------------------------------
// ModObj_v05 (0x356608)

/// `cMoveNode`: object (+0x18), modifiers (+0x1c: rotation at +4, light at
/// +8), fade (+0x24). Object: light (+0x94), visibility group (+0x7d, 0xff:
/// always), visibility slot (+0x7e).
pub(crate) const MOVE_NODE_OBJECT: u32 = 0x18;
pub(crate) const MOVE_NODE_MODIFIERS: u32 = 0x1c;
pub(crate) const MOVE_NODE_FADE: u32 = 0x24;
/// `cWorld::m_pTheWorld` (`gp + 0x16c8`).
pub(crate) const MOVE_NODE_WORLD_GP: u32 = 0x16c8;

/// `mods && mods->field != 0` (v0 as the original computes it).
fn move_node_has(c: &mut Cpu, mods: u64, field: u32) -> bool {
    c.cycles += 2;
    c.g[V0][0] = 0;
    if mods != 0 {
        c.cycles += 2;
        let v = ui_vector_lw(c, mods, field);
        c.g[V0][0] = (v != 0) as u64;
    }
    c.g[V0][0] != 0
}

/// `void cMoveNode::renderModel(cMoveNode* this, const t4Matrix* animMats,
/// int flags)` (0x356608): if the object's visibility group is on (group
/// 0xff, or cache group type 6 with slot state 3), draw the model through
/// GrMan (vtable `+0x2f8`) with the rotation modifier's UV matrix (flags |=
/// 2) and the light modifier's light (else the object's own).
pub fn mod_obj_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x90);
        let gp = c.g[GP][0] as u32;
        let world = Addr(gp.wrapping_add(MOVE_NODE_WORLD_GP)).lw(c);
        c.g[V0][0] = world;
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        frame.save(c, S2, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        let this = c.g[S0][0];
        let obj = ui_vector_lw(c, this, MOVE_NODE_OBJECT);
        c.g[A0][0] = obj;
        let cache = ui_vector_lw(c, world, 0);
        c.g[A1][0] = cache;
        let group = Addr((obj as u32).wrapping_add(0x7d)).lbu(c);
        c.g[V1][0] = group;
        c.g[V0][0] = 0xff;
        c.g[S1][0] = c.g[A2][0];
        if group == 0xff {
            c.cycles += 2;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 7;
            let g = (cache as u32).wrapping_add(group as u32 * 8);
            c.g[V0][0] = sx(g);
            c.g[V1][0] = Addr(g + 0x24).lw(c) ^ 6;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr((obj as u32).wrapping_add(0x7e)).lh(c);
                c.g[A0][0] = 0x18;
                c.mult(0, V1, V0, A0);
                let slot = (c.g[V1][0] as u32).wrapping_add(cache as u32);
                c.g[V0][0] = sx(slot);
                c.g[V1][0] = Addr(slot + 0x3f0).lw(c) ^ 3;
                c.g[V0][0] = (c.g[V1][0] < 1) as u64;
            }
        }
        c.cycles += 2;
        c.g[T1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            let mods = ui_vector_lw(c, c.g[S0][0], MOVE_NODE_MODIFIERS);
            c.g[A0][0] = mods;
            c.g[S2][0] = Addr(gp.wrapping_add(AVALANCHE_GRMAN_GP)).lw(c);
            c.g[V0][0] = 0;
            if mods != 0 {
                c.cycles += 2;
                let v = ui_vector_lw(c, mods, 4);
                c.g[V0][0] = (v != 0) as u64;
            }
            // Rotation modifier: its UV matrix on the stack.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                if !move_node_has(c, c.g[A0][0], 4) {
                    c.cycles += 2;
                    c.g[S1][0] |= 2;
                } else {
                    c.cycles += 2 + 3;
                    let rot = ui_vector_lw(c, c.g[A0][0], 4);
                    c.g[A0][0] = rot;
                    if rot == 0 {
                        c.g[V0][0] = 0;
                        c.cycles += 1;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x0035_fc20, 0x0035_66d8)?;
                    }
                    c.g[A0][0] = ui_vector_lw(c, c.g[S0][0], MOVE_NODE_MODIFIERS);
                    c.cycles += 2;
                    c.g[T1][0] = c.g[V0][0];
                    c.g[S1][0] |= 2;
                }
            }
            // Light: the light modifier's (+0x24 of it) or the object's.
            let has_light = move_node_has(c, c.g[A0][0], 8);
            c.cycles += 2;
            let mut own_light = true;
            if has_light && move_node_has(c, c.g[A0][0], 8) {
                c.cycles += 2 + 3;
                let light = ui_vector_lw(c, c.g[A0][0], 8);
                c.g[A0][0] = light;
                c.g[A2][0] = 0;
                if light != 0 {
                    c.cycles += 2;
                    c.g[A2][0] = sx((light as u32).wrapping_add(0x24));
                }
                own_light = false;
            } else if has_light {
                c.cycles += 2;
            }
            if own_light {
                let obj = ui_vector_lw(c, c.g[S0][0], MOVE_NODE_OBJECT);
                c.g[V0][0] = obj;
                c.cycles += 1;
                c.g[A2][0] = ui_vector_lw(c, obj, 0x94);
            }
            c.cycles += 11;
            let grman = c.g[S2][0];
            let vt = ui_vector_lw(c, grman, 0x10d8);
            c.g[V0][0] = vt;
            c.g[A3][0] = sx(0x0050_0000 - 0xe60); // t4mat_Identity
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(MOVE_NODE_FADE));
            let delta = Addr((vt as u32).wrapping_add(0x2f8)).lh(c);
            c.g[T0][0] = c.g[S3][0];
            let func = ui_vector_lw(c, vt, 0x2fc);
            c.g[V1][0] = func;
            c.g[T2][0] = c.g[S1][0];
            c.g[A1][0] = ui_vector_lw(c, c.g[S0][0], MOVE_NODE_OBJECT);
            c.g[A0][0] = sx((grman as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x0035_6760)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SectionAllocator_Alloc (0x3a7878)

/// `SectionAllocator`: block pool (+0), block count (+8), block list head
/// (+0xc), current block (+0x10). Block: data (+0), size (+4), used (+8),
/// padding total (+0xc), next (+0x14).
pub(crate) const SECTION_ALLOC_COUNT: u32 = 0x8;
pub(crate) const SECTION_ALLOC_HEAD: u32 = 0xc;
pub(crate) const SECTION_ALLOC_CUR: u32 = 0x10;
const SECTION_ALLOC_ENTRY: u32 = 0x003a_7878;
/// Resume points (loop back edges that may unwind): take the found block,
/// and the block scan's head.
const SECTION_ALLOC_TAKE: u32 = 0x003a_792c;
const SECTION_ALLOC_SCAN: u32 = 0x003a_7978;
const SECTION_ALLOC_END: u32 = 0x003a_7a20;
/// The new-block fallback (not a resume point).
const SECTION_ALLOC_NEW: u32 = 0x003a_79c8;
const WORLD_BLOCK_POOL_ALLOC_BLOCK: Callee = Callee {
    addr: 0x003a_7768,
    f: ee::s13::WorldBlockPool_AllocBlock,
};

/// Allocate a fresh block from the pool and make it the list head and the
/// current block (count + 1). `last` is the register the count goes through.
fn section_alloc_new_block(c: &mut Cpu, ret: u32, last: usize) -> crate::call::Flow {
    let this = c.g[S0][0] as u32;
    c.g[A0][0] = Addr(this).lw(c);
    WORLD_BLOCK_POOL_ALLOC_BLOCK.call(c, ret)?;
    let this = c.g[S0][0] as u32;
    let block = c.g[V0][0] as u32;
    Addr(block + 8).sw(c, 0);
    Addr(block + 0xc).sw(c, 0);
    c.g[V1][0] = Addr(this + SECTION_ALLOC_HEAD).lw(c);
    Addr(block + 0x14).sw(c, c.g[V1][0]);
    Addr(this + SECTION_ALLOC_CUR).sw(c, c.g[V0][0]);
    c.g[last][0] = Addr(this + SECTION_ALLOC_COUNT).lw(c);
    Addr(this + SECTION_ALLOC_HEAD).sw(c, c.g[V0][0]);
    c.g[last][0] = sx((c.g[last][0] as u32).wrapping_add(1));
    Addr(this + SECTION_ALLOC_COUNT).sw(c, c.g[last][0]);
    Ok(())
}

/// Does `size` (s1) fit in the block whose used count is in `pos_reg`, at
/// alignment a2? Leaves t1 = padding, v1 = padding + size, a0 = remainder,
/// a1 = the block size (from `block`), LO/HI of the division.
fn section_alloc_fits(c: &mut Cpu, block: u32, pos_reg: usize) -> bool {
    c.cycles += 10;
    c.div(0, pos_reg, A2);
    c.g[A1][0] = Addr(block + 4).lw(c);
    c.g[A0][0] = c.hi[0];
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    if c.g[A0][0] != 0 {
        c.g[T1][0] = c.g[V1][0];
    }
    let end = (c.g[pos_reg][0] as u32)
        .wrapping_add(c.g[T1][0] as u32)
        .wrapping_add(c.g[S1][0] as u32);
    let fits = (sx(end) as i64) < (c.g[A1][0] as i64);
    c.g[V0][0] = fits as u64;
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[S1][0] as u32));
    fits
}

/// The allocation's tail: v0 = data + used + padding; the current block's
/// padding total += padding.
fn section_alloc_finish(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
    let cur = Addr((c.g[S0][0] as u32) + SECTION_ALLOC_CUR).lw(c);
    c.g[A0][0] = cur;
    let pad = Addr((cur as u32) + 0xc).lw(c);
    c.g[V1][0] = sx((pad as u32).wrapping_add(c.g[T1][0] as u32));
    Addr((cur as u32) + 0xc).sw(c, c.g[V1][0]);
}

/// `void* SectionAllocator_Alloc(SectionAllocator* this, int size, u32
/// flags)` (0x3a7878): `size` bytes aligned to `4 << (flags >> 24 & 15)`,
/// from the current block, else the first other block with room, else a new
/// block (unaligned at its start). Resumable at its two loop back edges.
pub fn section_allocator_alloc(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut at = match c.pc {
            SECTION_ALLOC_ENTRY => {
                c.cycles += 10;
                Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                let this = c.g[S0][0] as u32;
                c.g[V0][0] = Addr(this + SECTION_ALLOC_COUNT).lw(c);
                c.g[S2][0] = c.g[A2][0];
                if c.g[V0][0] == 0 {
                    c.cycles += 11;
                    section_alloc_new_block(c, 0x003a_78a8, V1)?;
                }
                // The current block.
                c.cycles += 9;
                let this = c.g[S0][0] as u32;
                let cur = Addr(this + SECTION_ALLOC_CUR).lw(c);
                c.g[T3][0] = cur;
                c.g[V0][0] = sx((c.g[S2][0] as u32) >> 24) & 0xf;
                c.g[V1][0] = 4;
                c.g[A3][0] = Addr(cur as u32 + 8).lw(c);
                c.g[A2][0] = sx(4u32 << (c.g[V0][0] as u32 & 31));
                c.g[T1][0] = 0;
                if section_alloc_fits(c, cur as u32, A3) {
                    c.cycles += 5;
                    let data = Addr(cur as u32).lw(c);
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    Addr(cur as u32 + 8).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx((data as u32).wrapping_add(c.g[A3][0] as u32));
                    section_alloc_finish(c);
                    0
                } else {
                    // Other blocks, from the head (with at least two).
                    c.cycles += 4;
                    let count = Addr(this + SECTION_ALLOC_COUNT).lw(c);
                    c.g[V0][0] = (count < 2) as u64;
                    if count < 2 {
                        SECTION_ALLOC_NEW
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(this + SECTION_ALLOC_HEAD).lw(c);
                        Addr(this + SECTION_ALLOC_CUR).sw(c, c.g[V0][0]);
                        if c.g[V0][0] == 0 {
                            SECTION_ALLOC_NEW
                        } else {
                            c.cycles += 2;
                            c.g[T4][0] = 0;
                            SECTION_ALLOC_SCAN
                        }
                    }
                }
            }
            SECTION_ALLOC_TAKE => SECTION_ALLOC_TAKE,
            SECTION_ALLOC_SCAN => SECTION_ALLOC_SCAN,
            pc => {
                c.jump_out(pc, SECTION_ALLOC_ENTRY, SECTION_ALLOC_END);
                return Ok(());
            }
        };
        // `at`: 0 done, SCAN the block scan, TAKE a found block, NEW the
        // new-block fallback.
        loop {
            match at {
                SECTION_ALLOC_SCAN => {
                    c.cycles += 3;
                    let this = c.g[S0][0] as u32;
                    let block = Addr(this + SECTION_ALLOC_CUR).lw(c);
                    c.g[A3][0] = block;
                    c.g[T2][0] = block;
                    if block != c.g[T3][0] {
                        c.cycles += 4;
                        c.g[T0][0] = Addr(block as u32 + 8).lw(c);
                        c.g[T1][0] = 0;
                        if section_alloc_fits(c, block as u32, T0) {
                            if back_edge_unwinds(c, SECTION_ALLOC_TAKE) {
                                return Ok(());
                            }
                            at = SECTION_ALLOC_TAKE;
                            continue;
                        }
                    }
                    c.cycles += 3;
                    let next = Addr((c.g[T2][0] as u32) + 0x14).lw(c);
                    c.g[V0][0] = next;
                    Addr(this + SECTION_ALLOC_CUR).sw(c, next);
                    if next != 0 {
                        if back_edge_unwinds(c, SECTION_ALLOC_SCAN) {
                            return Ok(());
                        }
                        continue;
                    }
                    at = SECTION_ALLOC_NEW;
                }
                SECTION_ALLOC_TAKE => {
                    c.cycles += 4;
                    let block = c.g[A3][0] as u32;
                    let data = Addr(block).lw(c);
                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    Addr(block + 8).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx((data as u32).wrapping_add(c.g[T0][0] as u32));
                    section_alloc_finish(c);
                    at = 0;
                }
                SECTION_ALLOC_NEW => {
                    c.cycles += 16;
                    section_alloc_new_block(c, 0x003a_79d0, A0)?;
                    let block = c.g[V0][0] as u32;
                    let used = Addr(block + 8).lw(c);
                    c.g[V1][0] = sx((used as u32).wrapping_add(c.g[S1][0] as u32));
                    Addr(block + 8).sw(c, c.g[V1][0]);
                    let cur = Addr((c.g[S0][0] as u32) + SECTION_ALLOC_CUR).lw(c);
                    c.g[A0][0] = cur;
                    c.g[V0][0] = Addr(cur as u32).lw(c);
                    at = 0;
                }
                _ => break,
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __ieee754_rem_pio2f (0x40e878)

/// `npio2_hw` (0x499540) and `two_over_pi` (0x499228) of newlib's
/// `e_rem_pio2f.c`.
pub(crate) const REM_PIO2F_NPIO2_HW: u32 = 0x0049_9540;
pub(crate) const REM_PIO2F_TWO_OVER_PI: u32 = 0x0049_9228;
const FABSF: Callee = Callee {
    addr: 0x0040_f8a0,
    f: ee::s15::fabsf,
};
const KERNEL_REM_PIO2F: Callee = Callee {
    addr: 0x0040_ec58,
    f: ee::s15::__kernel_rem_pio2f,
};

/// `f = bits` through AT (`lui/ori at; mtc1 at, f`).
#[inline]
fn rem_pio2f_const(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
}

/// The range test of one case: `cycles`, v0 = `v0` (the next constant the
/// original builds), `ix <= limit`.
fn rem_pio2f_below(c: &mut Cpu, cycles: u64, v0: u64, ix: i64, limit: i64) -> bool {
    c.cycles += cycles;
    c.g[V0][0] = v0;
    ix <= limit
}

/// `x -/+ pio2_1` (and the tail) for |x| < 3pi/4: subtracts for x > 0,
/// else adds. y[0] = f1, y[1] = f0; v0 = n = 1 or -1.
fn rem_pio2f_small(c: &mut Cpu, positive: bool) {
    let op = |c: &mut Cpu, fd: usize, fs: usize, ft: usize| {
        if positive {
            c.sub_s(fd, fs, ft)
        } else {
            c.add_s(fd, fs, ft)
        }
    };
    c.cycles += 9;
    rem_pio2f_const(c, 0, 0x3fc9_0f80); // pio2_1
    c.g[V0][0] = c.g[S0][0] & 0xffff_ffff_ffff_fff0;
    c.g[V1][0] = 0x3fc9_0fd0;
    op(c, 12, 12, 0);
    if c.g[V0][0] != c.g[V1][0] {
        c.cycles += 5;
        rem_pio2f_const(c, 2, 0x3735_4443); // pio2_1t
        op(c, 1, 12, 2);
    } else {
        // Near pi/2: 33+53 bits of pi.
        c.cycles += 8;
        rem_pio2f_const(c, 0, 0x3735_4400); // pio2_2
        rem_pio2f_const(c, 2, 0x2e85_a308); // pio2_2t
        op(c, 12, 12, 0);
        op(c, 1, 12, 2);
    }
    c.cycles += 6;
    let y = c.g[S1][0] as u32;
    c.sub_s(0, 12, 1);
    c.swc1(1, y);
    op(c, 0, 0, 2);
    c.swc1(0, y + 4);
    c.g[V0][0] = if positive { 1 } else { u64::MAX };
}

/// `int __ieee754_rem_pio2f(float x, float* y)` (0x40e878): x reduced by
/// n * pi/2 into y[0] + y[1]; returns n (mod 8 is all callers use). Small
/// cases inline, medium by n = round(|x| * 2/pi) with up to three pi/2
/// parts, huge ones through `__kernel_rem_pio2f`.
pub fn ieee754_rem_pio2f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        let sp = sp(c);
        c.sd(sp + 0x20, c.g[S1][0]);
        c.sd(sp + 0x40, c.g[RA][0]);
        c.sd(sp + 0x30, c.g[S2][0]);
        c.sd(sp + 0x10, c.g[S0][0]);
        c.g[S2][0] = sx(c.f[12]); // hx
        c.g[V1][0] = 0x7fff_ffff;
        c.g[S0][0] = c.g[S2][0] & 0x7fff_ffff; // ix
        c.g[S1][0] = c.g[A0][0];
        let ix = c.g[S0][0] as i64;
        let y = c.g[S1][0] as u32;
        if ix <= 0x3f49_0fd8 {
            // |x| <= pi/4: y = (x, 0), n = 0.
            c.g[V0][0] = 0;
            c.cycles += 4;
            c.swc1(12, y);
            Addr(y + 4).sw(c, 0);
        } else if rem_pio2f_below(c, 5, 0x4349_0000, ix, 0x4016_cbe3) {
            // |x| < 3pi/4.
            c.cycles += 2;
            c.g[V1][0] = 0x3fc9_0000;
            rem_pio2f_small(c, (c.g[S2][0] as i64) > 0);
        } else if rem_pio2f_below(c, 4, 0x7f7f_0000, ix, 0x4349_0f80) {
            // |x| <= 2^7 * pi/2: medium.
            c.cycles += 25;
            FABSF.call(c, 0x0040_e9dc)?;
            rem_pio2f_const(c, 1, 0x3f22_f984); // invpio2
            c.mov_s(5, 0); // t = |x|
            rem_pio2f_const(c, 2, 0x3f00_0000);
            c.mul_s(1, 5, 1);
            rem_pio2f_const(c, 0, 0x3fc9_0f80); // pio2_1
            rem_pio2f_const(c, 3, 0x3735_4443); // pio2_1t
            c.add_s(1, 1, 2);
            c.cvt_w_s(2, 1);
            c.mfc1(A1, 2); // n
            c.mtc1(A1, 6);
            c.cvt_s_w(6, 6); // fn
            let n = c.g[A1][0];
            c.g[V0][0] = ((n as i64) < 32) as u64;
            c.mul_s(0, 6, 0);
            c.mul_s(3, 6, 3);
            c.sub_s(4, 5, 0); // r = t - fn * pio2_1
            let y = c.g[S1][0] as u32;
            let mut quick = false;
            if (n as i64) < 32 {
                c.cycles += 11;
                let at = REM_PIO2F_NPIO2_HW.wrapping_add(((n as u32).wrapping_sub(1)) << 2);
                c.g[V1][0] = sx(at);
                c.g[A0][0] = Addr(at).lw(c);
                c.g[V0][0] = c.g[S0][0] & 0xffff_ffff_ffff_ff00;
                c.sub_s(0, 4, 3);
                if c.g[V0][0] != c.g[A0][0] {
                    c.cycles += 2;
                    c.swc1(0, y);
                    c.cycles += 1;
                    c.lwc1(1, y);
                    quick = true;
                }
            } else {
                c.cycles += 1;
                c.sub_s(0, 4, 3);
            }
            if !quick {
                c.cycles += 9;
                c.g[A0][0] = sx(((c.g[S0][0] as u32) as i32 >> 23) as u32); // j
                c.mfc1(V0, 0);
                Addr(y).sw(c, c.g[V0][0]);
                let i = rem_pio2f_exponent_gap(c);
                c.g[V0][0] = (i < 9) as u64;
                if i < 9 {
                    c.lwc1(1, y);
                } else {
                    // Second iteration: 2nd 24 bits of pi/2.
                    c.cycles += 22;
                    rem_pio2f_const(c, 0, 0x3735_4400); // pio2_2
                    c.mov_s(5, 4);
                    rem_pio2f_const(c, 1, 0x2e85_a308); // pio2_2t
                    c.mul_s(3, 6, 0);
                    c.mul_s(1, 6, 1);
                    c.sub_s(4, 4, 3);
                    c.sub_s(0, 5, 4);
                    c.sub_s(0, 0, 3);
                    c.sub_s(3, 1, 0);
                    c.sub_s(2, 4, 3);
                    c.mfc1(V0, 2);
                    Addr(y).sw(c, c.g[V0][0]);
                    let i = rem_pio2f_exponent_gap(c);
                    c.g[V0][0] = (i < 26) as u64;
                    if i < 26 {
                        c.lwc1(1, y);
                    } else {
                        // Third iteration.
                        c.cycles += 15;
                        rem_pio2f_const(c, 0, 0x2e85_a300); // pio2_3
                        c.mov_s(5, 4);
                        rem_pio2f_const(c, 2, 0x248d_3132); // pio2_3t
                        c.mul_s(3, 6, 0);
                        c.mul_s(2, 6, 2);
                        c.sub_s(4, 4, 3);
                        c.sub_s(0, 5, 4);
                        c.sub_s(0, 0, 3);
                        c.sub_s(3, 2, 0);
                        c.sub_s(1, 4, 3);
                        c.swc1(1, y);
                        c.cycles += 1;
                        c.lwc1(1, y);
                    }
                }
            }
            // y[1] = (r - y[0]) - w; negate for x < 0.
            c.cycles += 4;
            c.sub_s(0, 4, 1);
            c.sub_s(0, 0, 3);
            c.swc1(0, y + 4);
            if (c.g[S2][0] as i64) >= 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
            } else {
                c.cycles += 3;
                c.neg_s(1, 1);
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                rem_pio2f_store_negated(c);
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x7f7f_ffff;
            let nan = ix > 0x7f7f_ffff;
            c.g[V0][0] = 0;
            if nan {
                // Inf or NaN: y = (x - x, x - x), n = 0.
                c.cycles += 4;
                c.sub_s(0, 12, 12);
                c.swc1(0, y);
                c.swc1(0, y + 4);
            } else {
                rem_pio2f_large(c)?;
            }
        }
        c.cycles += 6;
        let sp = self::sp(c);
        c.g[RA][0] = c.ld(sp + 0x40);
        c.g[S2][0] = c.ld(sp + 0x30);
        c.g[S1][0] = c.ld(sp + 0x20);
        c.g[S0][0] = c.ld(sp + 0x10);
        c.g[SP][0] = sx(sp.wrapping_add(0x50));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `i = j - ((y[0] >> 23) & 0xff)` from the word in v0 (a0 = j); v1 = i.
fn rem_pio2f_exponent_gap(c: &mut Cpu) -> i64 {
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 23) & 0xff;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V1][0] as i64
}

/// x < 0: y[0] = -f1 (already negated), y[1] = -f0.
fn rem_pio2f_store_negated(c: &mut Cpu) {
    c.cycles += 4;
    let y = c.g[S1][0] as u32;
    c.neg_s(0, 0);
    c.swc1(1, y);
    c.swc1(0, y + 4);
}

/// |x| > 2^7 * pi/2: split x into three 8-bit-exponent floats on the stack
/// and hand them to `__kernel_rem_pio2f`.
fn rem_pio2f_large(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 10;
    let sp = sp(c);
    c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 23) as u32);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x86)); // e0
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 23);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.mtc1(S0, 12); // z
    rem_pio2f_const(c, 2, 0x4380_0000); // two8
    c.g[V1][0] = 1;
    c.g[A0][0] = sx(sp);
    loop {
        c.cycles += 10;
        c.cvt_w_s(0, 12);
        c.mfc1(V0, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(1, 12, 0);
        c.swc1(0, c.g[A0][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        c.mul_s(12, 1, 2);
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_eb90);
        }
    }
    // tx[2] = z; skip trailing zeros (nx).
    c.cycles += 6;
    c.f[1] = 0;
    c.g[A3][0] = 3;
    c.c_eq_s(12, 1);
    c.swc1(12, sp + 8);
    if c.fcc() {
        c.cycles += 2;
        c.g[A0][0] = 2;
        c.g[A3][0] = 2;
        loop {
            c.cycles += 9;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx(sp.wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, c.g[V0][0] as u32);
            c.c_eq_s(0, 1);
            if !c.fcc() {
                break;
            }
            c.g[A3][0] = c.g[A0][0];
            if event_due(c) {
                deliver_event_here(c, 0x0040_ebd8);
            }
        }
    }
    c.cycles += 8;
    c.g[T1][0] = sx(REM_PIO2F_TWO_OVER_PI);
    c.g[A0][0] = sx(sp);
    c.g[A1][0] = c.g[S1][0];
    c.g[T0][0] = 2;
    KERNEL_REM_PIO2F.call(c, 0x0040_ec14)?;
    c.g[A1][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) >= 0 {
        c.cycles += 1;
        c.g[V0][0] = c.g[A1][0];
    } else {
        c.cycles += 4;
        let y = c.g[S1][0] as u32;
        c.lwc1(1, y);
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
        c.lwc1(0, y + 4);
        c.neg_s(1, 1);
        rem_pio2f_store_negated(c);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA, GAME_GP};

    /// A machine with the game image loaded (jump tables, `.lit4`), `gp`
    /// set and `pc` at `entry` (the generated slot dispatches on it).
    fn oc383_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = Rng(0x0383_0000_0000 ^ salt ^ (seed as u64) << 20);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = entry;
        r
    }

    #[test]
    fn ui_object_set_anim_param_matches() {
        check(
            ee::s13::f_003a0048__generated,
            ui_object_set_anim_param,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 1, 0x003a_0048);
                fill(c, &mut r, DATA, 0x20);
                c.g[A0][0] = sx(DATA);
                let key = if seed % 8 == 0 {
                    r.next_u32()
                } else {
                    seed % 19
                };
                c.g[A1][0] = (r.next_u32() as u64) << 32 | key as u64;
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(300.0)
                };
            },
        );
    }

    /// Fill `words` floats at `addr` within +-`range`.
    fn oc383_floats(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32, range: f32) {
        for i in 0..words {
            let v = r.f32(range);
            c.bus.write32(addr + 4 * i, v);
        }
    }

    #[test]
    fn ui_vector_render_matches() {
        use crate::difftest::fakes::{machine, vtable};
        check(
            ee::s13::f_003a39f8__generated,
            ui_vector_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39f8, 1);
                c.pc = 0x003a_39f8;
                let this = DATA;
                fill(c, &mut r, this, 0x28);
                oc383_floats(c, &mut r, this + 0x44, 2, 300.0);
                oc383_floats(c, &mut r, this + 0x50, 2, 3.0);
                let count = seed % 5;
                let indexed = (seed / 5) % 2;
                let counts =
                    count << 17 | (r.next_u32() & 0x7fff) << 2 | (r.next_u32() & 2) | indexed;
                c.bus.write32(this + UI_VECTOR_COUNTS, counts);
                let positions = if seed % 11 == 3 {
                    0
                } else {
                    DATA + 0x1000 + (seed % 3) * 4
                };
                let render = DATA + 0x2000 + ((seed / 3) % 3) * 4;
                let vertices = if seed % 13 == 5 { 0 } else { DATA + 0x3000 };
                c.bus.write32(this + UI_VECTOR_POSITIONS, positions);
                c.bus.write32(this + UI_VECTOR_RENDER_POSITIONS, render);
                c.bus.write32(this + UI_VECTOR_VERTICES, vertices);
                c.bus.write32(this + UI_VECTOR_COLOURS, DATA + 0x4000);
                c.bus.write32(this + UI_VECTOR_TRIANGLES, r.next_u32());
                oc383_floats(c, &mut r, DATA + 0x1000, 24, 200.0);
                fill(c, &mut r, DATA + 0x2000, 24);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                for i in 0..24 {
                    let v = (r.next_u32() % 2560) as f32 / 10.0;
                    c.bus.write32(DATA + 0x4000 + 4 * i, v.to_bits());
                }
                // GrMan with its draw state and vtable.
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP - UI_VECTOR_GRMAN_GP, grman);
                c.bus.write32(grman + 0xe84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 8);
                let vt = vtable(c, &mut r, &[0x240, 0x248]);
                c.bus.write32(grman + 0x10d8, vt);
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(100.0);
            },
        );
    }

    #[test]
    fn ui_vector_set_anim_param_matches() {
        check(
            ee::s13::f_003a36d0__generated,
            ui_vector_set_anim_param,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0x36d0, 0x003a_36d0);
                let this = DATA;
                fill(c, &mut r, this, 0x28);
                let count = 1 + seed % 4;
                c.bus.write32(
                    this + UI_VECTOR_COUNTS,
                    count << 17 | (r.next_u32() & 0x1ffff),
                );
                let verts = if seed % 17 == 4 { 0 } else { DATA + 0x1000 };
                c.bus.write32(this + UI_VECTOR_POSITIONS, verts);
                c.bus.write32(this + 0x88, DATA + 0x2000);
                c.bus.write32(this + UI_VECTOR_COLOURS, DATA + 0x3000);
                let key = match seed % 6 {
                    0 => seed % 0x15,
                    // Vertex index up to `count` (one past the end too).
                    _ => 0x15 + (seed / 6) % (9 * (count + 1)),
                };
                c.g[A1][0] =
                    (r.next_u32() as u64) << 32 | (r.next_u32() & 0xffff_0000) as u64 | key as u64;
                c.f[12] = r.f32(100.0);
            },
        );
    }

    /// A vtable at `at` whose slots `off` are `{small delta, fake i}`.
    fn oc383_vtable(c: &mut Cpu, r: &mut Rng, at: u32, slots: &[u32]) {
        use crate::difftest::fakes::FAKE;
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    #[test]
    fn avalanche_render_active_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s09::f_002d8ea8__generated,
            avalanche_render_active,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8ea8, 8);
                c.pc = 0x002d_8ea8;
                let this = DATA;
                let group = DATA + 0x1000;
                fill(c, &mut r, this, 0x10);
                c.bus.write32(this, if seed % 13 == 7 { 0 } else { group });
                c.bus.write32(this + AVALANCHE_SLOT_LIMIT, r.f32(10.0));
                // Group pairs: v at +0x10 + 8j, node at +0x14 + 8j.
                let pairs = (seed / 3) % 4;
                c.bus.write16(group + 0xc, pairs as u16);
                for j in 0..pairs {
                    let v = if seed % 5 == 0 {
                        c.bus.read32(this + AVALANCHE_SLOT_LIMIT)
                    } else {
                        r.f32(10.0)
                    };
                    c.bus.write32(group + 0x10 + 8 * j, v);
                    let node = DATA + 0x7000 + 0x100 * j;
                    c.bus.write32(group + 0x14 + 8 * j, node);
                    fill(c, &mut r, node + 0x40, 4);
                }
                // Active nodes: data (or none), next; emitter `+0xd0` without texture.
                let nodes = seed % 4;
                c.bus.write32(
                    this + AVALANCHE_SLOT_ACTIVE,
                    if nodes == 0 { 0 } else { DATA + 0x2000 },
                );
                for k in 0..nodes {
                    let node = DATA + 0x2000 + 0x400 * k;
                    let data = DATA + 0x8000 + 0x200 * k;
                    let has = r.next_u32() % 4 != 0;
                    c.bus
                        .write32(node + AVALANCHE_NODE_DATA, if has { data } else { 0 });
                    let next = if k + 1 < nodes { node + 0x400 } else { 0 };
                    c.bus.write32(node + AVALANCHE_NODE_NEXT, next);
                    c.bus.write32(node + 0xd0 + 0x174, 0);
                    c.bus.write8(data + 0xf2, (r.next_u32() & 1) as u8);
                    fill(c, &mut r, data + 0xe0, 4);
                    // No line segments in Avalanche_DebugDrawInstance.
                    c.bus.write32(
                        data + 0x100,
                        if r.next_u32() & 1 == 0 {
                            0
                        } else {
                            0xbf80_0000
                        },
                    );
                }
                c.bus
                    .write32(GAME_GP + AVALANCHE_DEBUG_LINES_GP, (seed / 2) % 3);
                c.bus
                    .write32(GAME_GP + AVALANCHE_DEBUG_FX_GP, (seed / 7) % 2);
                // GrMan (both globals), its draw state and vtable.
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP + AVALANCHE_GRMAN_GP, grman);
                c.bus.write32(GAME_GP - 0x854, grman);
                c.bus.write32(grman + 0xe84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 0x10);
                c.bus.write32(grman + 0x10d8, DATA + 0x5800);
                oc383_vtable(c, &mut r, DATA + 0x5800, &[0x248]);
                // SSXApp->Game->AI->rider + 0x6c0 with its getPos slot.
                c.bus.write32(GAME_GP - 0x848, DATA + 0x9000);
                c.bus.write32(DATA + 0x9000 + 0x84, DATA + 0x9100);
                c.bus.write32(DATA + 0x9100 + 0xc, DATA + 0x9200);
                c.bus.write32(DATA + 0x9200 + 0x28, DATA + 0x9300);
                c.bus.write32(DATA + 0x9300 + 0x6c0, DATA + 0x9c00);
                oc383_vtable(c, &mut r, DATA + 0x9c00, &[0x28]);
            },
        );
    }

    #[test]
    fn tolower_matches() {
        check(ee::s15::f_0041ea18__generated, tolower, 300, &|c, seed| {
            let mut r = oc383_machine(c, seed, 0xea18, 0x0041_ea18);
            let ch = if seed < 256 {
                seed
            } else {
                r.next_u32() & 0x1ff
            };
            c.g[A0][0] = if seed % 7 == 6 {
                sx(ch | 0xffff_ff00)
            } else {
                ch as u64
            };
        });
    }

    /// A random string of letters and punctuation (mixed case) at `at`.
    fn oc383_put_str(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    #[test]
    fn strcasecmp_matches() {
        check(
            ee::s15::f_0041aa88__generated,
            strcasecmp,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0xaa88, STRCASECMP_ENTRY);
                let len = (r.next_u32() % 8) as usize;
                let a: Vec<u8> = (0..len)
                    .map(|_| {
                        let k = r.next_u32();
                        [b'a', b'A', b'0', b'_', 0xc1][(k % 5) as usize]
                            .wrapping_add(((k >> 8) % 26) as u8 * ((k % 5 < 2) as u8))
                    })
                    .collect();
                let mut b: Vec<u8> = a
                    .iter()
                    .map(|&x| {
                        if r.next_u32() & 1 == 0 {
                            x.to_ascii_uppercase()
                        } else {
                            x.to_ascii_lowercase()
                        }
                    })
                    .collect();
                match seed % 4 {
                    0 if !b.is_empty() => {
                        let i = (r.next_u32() as usize) % b.len();
                        b[i] = b[i].wrapping_add(1 + (r.next_u32() % 3) as u8);
                    }
                    1 => b.truncate(b.len() / 2),
                    2 => b.push(b'x'),
                    _ => {}
                }
                let (sa, sb) = (DATA + 0x100, DATA + 0x200);
                oc383_put_str(c, sa, &a);
                oc383_put_str(c, sb, &b);
                c.g[A0][0] = sx(sa);
                c.g[A1][0] = sx(sb);
                if seed % 5 == 4 {
                    // Resume at the loop head inside the frame (s1/s2 mid-string).
                    let sp = crate::difftest::SP - 0x40;
                    fill(c, &mut r, sp, 0x10);
                    c.bus.write64(sp, crate::difftest::RA as u64);
                    c.g[SP][0] = sx(sp);
                    c.g[S1][0] = sx(sa + 1u32.min(len as u32));
                    c.g[S2][0] = sx(sb);
                    c.pc = STRCASECMP_HEAD;
                }
            },
        );
    }

    /// A font at `font`: 16 directly indexed glyphs 'A'..='P', then 8
    /// searched ones (keys 0x60, 0x62, .. 0x6e), random metrics, shadow.
    fn oc383_font(c: &mut Cpu, r: &mut Rng, font: u32, glyphs: u32) {
        c.bus.write32(font + FONT_GLYPH_COUNT, 24);
        c.bus.write32(font + FONT_GLYPHS, glyphs);
        c.bus.write32(font + FONT_FIRST_CHAR, 0x41);
        c.bus.write32(font + FONT_LAST_DIRECT, 0x50);
        for i in 0..24u32 {
            let e = glyphs + 12 * i;
            fill(c, r, e, 3);
            let key = if i < 16 {
                0x41 + i
            } else {
                0x60 + 2 * (i - 16)
            };
            c.bus.write16(e, key as u16);
        }
        let default = if r.next_u32() & 1 == 0 {
            0
        } else {
            glyphs + 12 * 5
        };
        c.bus.write32(font + FONT_DEFAULT_GLYPH, default);
        c.bus.write32(font + FONT_SHADOW, r.f32(3.0));
        c.bus.write32(font + FONT_SHADOW + 4, r.f32(3.0));
    }

    /// A wide string of up to 8 characters around the font's ranges.
    fn oc383_put_wstr(c: &mut Cpu, r: &mut Rng, at: u32) -> u32 {
        let len = r.next_u32() % 9;
        for i in 0..len {
            let k = r.next_u32();
            let ch = match k % 5 {
                0 => 0x30 + (k >> 8) % 0x11,
                1 | 2 => 0x41 + (k >> 8) % 16,
                3 => 0x5f + (k >> 8) % 0x12,
                _ => 0x100 + (k >> 8) % 0x100,
            };
            c.bus.write16(at + 2 * i, ch as u16);
        }
        c.bus.write16(at + 2 * len, 0);
        len
    }

    #[test]
    fn font_measure_text_w_matches() {
        check(
            ee::s12::f_003921f0__generated,
            font_measure_text_w,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0x21f0, 0x0039_21f0);
                oc383_font(c, &mut r, DATA, DATA + 0x100);
                oc383_put_wstr(c, &mut r, DATA + 0x400);
                fill(c, &mut r, DATA + 0x800, 4);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x800) };
                c.g[A3][0] = match seed % 4 {
                    0 | 1 => 0,
                    2 => 1,
                    _ => r.next_u32() as u64,
                };
                c.f[12] = r.f32(2.0);
                c.f[13] = r.f32(2.0);
            },
        );
    }

    #[test]
    fn font_draw_text_w_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s12::f_00391e30__generated,
            font_draw_text_w,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e30, 2);
                c.pc = 0x0039_1e30;
                let font = DATA;
                fill(c, &mut r, font, 0x20);
                oc383_floats(c, &mut r, font + FONT_OFFSET, 4, 5.0);
                oc383_floats(c, &mut r, font + FONT_SCALE, 2, 3.0);
                let (sx_, sy_) = match seed % 4 {
                    0 => (0, 0),
                    1 => (r.f32(3.0), 0),
                    2 => (0, r.f32(3.0)),
                    _ => (0x8000_0000, r.f32(3.0)),
                };
                c.bus.write32(font + FONT_SHADOW, sx_);
                c.bus.write32(font + FONT_SHADOW + 4, sy_);
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP + AVALANCHE_GRMAN_GP, grman);
                c.bus.write32(grman + 0xe84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 8);
                if seed % 5 == 0 {
                    // Layer 0: the shadow stays at layer 0.
                    let w = c.bus.read32(DATA + 0x6008) & !0x3e0;
                    c.bus.write32(DATA + 0x6008, w);
                }
                c.bus.write32(grman + 0x10d8, DATA + 0x5800);
                oc383_vtable(c, &mut r, DATA + 0x5800, &[0x288]);
                c.g[A0][0] = sx(font);
                c.g[A1][0] = sx(DATA + 0x400);
                c.f[12] = r.f32(300.0);
                c.f[13] = r.f32(300.0);
            },
        );
    }

    #[test]
    fn ea_thread_yield_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s14::f_003e5398__generated,
            ea_thread_yield,
            300,
            &|c, seed| {
                // Syscalls (thread status, alarms, sleep) are fakes.
                let mut r = machine(c, seed, 0x5398, 3);
                c.pc = 0x003e_5398;
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => (r.next_u32() % 100) as u64,
                    2 => sx(r.next_u32() % 4 + 0xffff_fffc),
                    _ => r.next_u32() as u64,
                };
                c.bus.write32(
                    EATHREAD_TICK_RATE,
                    [60, 1000, 0, 7][(seed / 4 % 4) as usize],
                );
                c.bus.write32(EATHREAD_IN_INTERRUPT, (seed / 16) % 2);
                c.bus.write32(0x0045_0da0, r.next_u32() % 2000);
            },
        );
    }

    #[test]
    fn t_wpigd_light_glow_blend_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s08::f_002bd068__generated,
            t_wpigd_light_glow_blend,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0xd068, 2);
                c.pc = 0x002b_d068;
                let this = DATA;
                let src = DATA + 0x100;
                c.bus.write32(this + 4, DATA + 0x800);
                oc383_vtable(c, &mut r, DATA + 0x800, &[0x218]);
                oc383_floats(c, &mut r, this + LIGHT_GLOW_PAIRS, 14, 50.0);
                oc383_floats(c, &mut r, src, 8, 50.0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.5)
                };
            },
        );
    }

    /// Link `nodes` into the `cList` at `list` (+4: first), ending at the
    /// self-linked `sentinel`; an empty list is sometimes a null link.
    fn oc383_ui_list(c: &mut Cpu, r: &mut Rng, list: u32, nodes: &[u32], sentinel: u32) {
        if nodes.is_empty() && r.next_u32() % 2 == 0 {
            c.bus.write32(list + 4, 0);
            return;
        }
        let mut prev = list;
        for &n in nodes {
            c.bus.write32(prev + 4, n);
            c.bus.write32(n, prev);
            prev = n;
        }
        c.bus.write32(prev + 4, sentinel);
        c.bus.write32(sentinel, prev);
        c.bus.write32(sentinel + 4, sentinel);
    }

    #[test]
    fn c_ui_group_render_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s13::f_00398868__generated,
            c_ui_group_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8868, 4);
                c.pc = 0x0039_8868;
                let this = DATA;
                fill(c, &mut r, this, 0x20);
                oc383_floats(c, &mut r, this + 0x44, 2, 300.0);
                let n = (seed % 5) as usize;
                let nodes: Vec<u32> = (0..n as u32).map(|i| DATA + 0x1000 + 0x100 * i).collect();
                for &node in &nodes {
                    fill(c, &mut r, node, 8);
                    c.bus.write32(node + 8, DATA + 0x800);
                }
                oc383_ui_list(c, &mut r, this + UI_GROUP_CHILDREN, &nodes, DATA + 0x1800);
                oc383_vtable(c, &mut r, DATA + 0x800, &[0x88]);
                c.g[A0][0] = sx(this);
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(100.0);
            },
        );
    }

    #[test]
    fn be_pre_race_get_interface_matches() {
        check(
            ee::s01::f_001565c8__generated,
            be_pre_race_get_interface,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0x65c8, 0x0015_65c8);
                // Every singleton already made (the getters' short path).
                for off in [
                    0x1f3c, 0x1f30, 0x1ee8, 0x1f2c, 0x1ee4, 0x1ed4, 0x1eec, 0x1ea8, 0x1f28, 0x1ee0,
                    0x1edc, 0x1eb8, 0x1ea4, 0x1e8c,
                ] {
                    c.bus.write32(GAME_GP - off, DATA + (r.next_u32() & 0xfff0));
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = match seed % 16 {
                    14 => r.next_u32() as u64,
                    15 => u64::MAX,
                    k => k as u64,
                };
            },
        );
    }

    #[test]
    fn parent_modifier_update_transform_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        check(
            ee::s11::f_00357108__generated,
            parent_modifier_update_transform,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7108, 2);
                c.pc = 0x0035_7108;
                let this = DATA;
                let inst = DATA + 0x400;
                let obj = DATA + 0x800;
                let vt = DATA + 0xc00;
                fill(c, &mut r, this, 0x30);
                oc383_floats(c, &mut r, this + PARENT_MOD_LOCAL_POS, 4, 10.0);
                oc383_floats(c, &mut r, this + PARENT_MOD_TRANSLATION, 4, 100.0);
                c.bus.write32(this + PARENT_MOD_INSTANCE, inst);
                // Node -1: Instance_GetNodeMatrix takes the owner's world matrix.
                c.bus.write32(this + PARENT_MOD_NODE, 0xffff_ffff);
                fill(c, &mut r, inst, 0x30);
                c.bus
                    .write32(inst + 0xc, if seed % 7 == 3 { 0 } else { obj });
                c.bus.write32(inst + 0x80, DATA + 0x1000);
                c.bus.write32(obj + 0xc, vt);
                oc383_vtable(c, &mut r, vt, &[0xd0, 0xe8, 0xc0]);
                if seed % 2 == 0 {
                    // "Has a node": a real leaf returning 0.
                    c.bus.write32(vt + 0xd4, 0x0014_06c0);
                } else {
                    c.bus.write32(vt + 0xd4, FAKE);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn world_obj_build_matrices_matches() {
        check(
            ee::s11::f_00350288__generated,
            world_obj_build_matrices,
            300,
            &|c, seed| {
                let mut r = Rng(0x0288 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0035_0288;
                let this = DATA;
                let inst = DATA + 0x100;
                let skel = DATA + 0x200;
                let bones = DATA + 0x300;
                let root = DATA + 0x800;
                let out = DATA + 0x1000;
                c.bus.write32(this + 0x18, inst);
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(inst + 0x84, r.f32(3.0));
                let n = seed % 6;
                c.bus
                    .write32(skel + 4, if seed % 11 == 10 { 0xffff_fffe } else { n });
                c.bus.write32(skel + 8, bones);
                for i in 0..n {
                    let b = bones + 0x10 * i;
                    // Parent: -1 (root) or an earlier bone.
                    let parent = if i == 0 || r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % i
                    };
                    c.bus.write32(b, parent);
                    let local = DATA + 0x2000 + 0x40 * i;
                    c.bus.write32(b + 0xc, local);
                    oc383_floats(c, &mut r, local, 16, 2.0);
                }
                oc383_floats(c, &mut r, root, 16, 2.0);
                fill(c, &mut r, out, 0x60);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(root);
                c.g[A2][0] = sx(out);
            },
        );
    }

    /// One REF-like DMA tag at `at`: id, qwc 1..8, address `off` (relative
    /// to its base), optionally flagged in bit 31.
    fn oc383_dma_tag(c: &mut Cpu, r: &mut Rng, at: u32, id: u32, flag: bool) {
        let qwc = 1 + r.next_u32() % 8;
        let off = (r.next_u32() % 0x400) * 0x10;
        c.bus.write32(
            at,
            id << 28 | (r.next_u32() & 0x0fff_0000) & !0x7000_0000 | qwc,
        );
        c.bus
            .write32(at + 4, off | if flag { 0x8000_0000 } else { 0 });
        c.bus.write64(at + 8, r.next_u32() as u64);
    }

    #[test]
    fn c_ps2_chunk_resolve_patch_instance_dma_chain_matches() {
        check(
            ee::s13::f_003ac508__generated,
            c_ps2_chunk_resolve_patch_instance_dma_chain,
            300,
            &|c, seed| {
                let mut r = Rng(0xc508 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = CHUNK_DMA_ENTRY;
                let (inst, chunk) = (DATA + 0x100, DATA + 0x200);
                c.bus.write32(chunk + 0x24, DATA + 0x10000);
                c.bus.write32(inst + 0x98, DATA + 0x20000);
                fill(c, &mut r, DATA + 0x10000, 0x1000);
                fill(c, &mut r, DATA + 0x20000, 0x1000);
                let chain = DATA + 0x1000;
                let mut p = chain;
                let pairs = seed % 4;
                let mut ended = false;
                for _ in 0..pairs {
                    let flagged = r.next_u32() % 2 == 0;
                    oc383_dma_tag(c, &mut r, p, 3, flagged);
                    let refs = r.next_u32() % 8;
                    oc383_dma_tag(c, &mut r, p + 0x10, refs, false);
                    p += 0x20;
                    if flagged {
                        if r.next_u32() % 3 == 0 {
                            // REF then END: rewritten, the chain ends there.
                            oc383_dma_tag(c, &mut r, p, 3, false);
                            oc383_dma_tag(c, &mut r, p + 0x10, 6, false);
                            ended = true;
                            break;
                        }
                        let f2 = r.next_u32() % 2 == 0;
                        oc383_dma_tag(c, &mut r, p, 3, f2);
                        for k in 1..4 {
                            let id = [0, 1, 2, 3, 4, 5, 7][(r.next_u32() % 7) as usize];
                            oc383_dma_tag(c, &mut r, p + 0x10 * k, id, false);
                        }
                        // The next tag is the chain's next one: never END here.
                        p += 0x40;
                    }
                }
                if !ended {
                    oc383_dma_tag(c, &mut r, p, 6, false);
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(inst);
                c.g[A2][0] = sx(chunk);
                c.g[A3][0] = sx(chain);
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_find_node_matches() {
        check(
            ee::s03::f_0019c938__generated,
            bolt_file_index_manager_find_node,
            300,
            &|c, seed| {
                let mut r = Rng(0xc938 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0019_c938;
                let m = DATA;
                let node = |i: u32| m + 0x10 * i;
                let hi = r.next_u32() % 0x80;
                let lo = r.next_u32() % 0x10000;
                let key = hi << 16 | lo;
                let b = key % 400;
                for i in 0..400 {
                    c.bus.write32(BOLT_INDEX_BUCKETS + m + 4 * i, 0);
                }
                // The bucket's chain: nodes 10, 20, 30.. (one may hold the key).
                let len = seed % 4;
                let hit = r.next_u32() % 4;
                for k in 0..len {
                    let n = node(10 * (k + 1));
                    c.bus.write32(
                        n,
                        if k == hit {
                            key
                        } else {
                            key ^ (1 + r.next_u32() % 0xffff)
                        },
                    );
                    c.bus
                        .write16(n + 0xc, if k == 0 { 0xffff } else { (10 * k) as u16 });
                    c.bus.write16(
                        n + 0xe,
                        if k + 1 < len {
                            (10 * (k + 2)) as u16
                        } else {
                            0xffff
                        },
                    );
                }
                c.bus.write32(
                    m + BOLT_INDEX_BUCKETS + 4 * b,
                    if len == 0 { 0 } else { node(10) },
                );
                // The free list: nodes 300, 301 (or none).
                let free = (seed / 4) % 3;
                for k in 0..free {
                    let n = node(300 + k);
                    fill(c, &mut r, n, 3);
                    c.bus.write16(
                        n + 0xe,
                        if k + 1 < free {
                            (301 + k) as u16
                        } else {
                            0xffff
                        },
                    );
                }
                c.bus
                    .write32(m + BOLT_INDEX_FREE, if free == 0 { 0 } else { node(300) });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() as u64) << 32 | hi as u64;
                c.g[A2][0] = lo as u64;
                c.g[A3][0] = ((seed / 12) % 2) as u64;
            },
        );
    }

    /// Point vtable slot `off` at a real leaf returning 0 (fakes always
    /// return non-zero).
    fn oc383_slot_zero(c: &mut Cpu, vt: u32, off: u32) {
        c.bus.write32(vt + off + 4, 0x0014_06c0);
    }

    #[test]
    fn ui_state_update_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s13::f_0039e510__generated,
            ui_state_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0xe510, 8);
                c.pc = 0x0039_e510;
                let this = DATA;
                let (owner, pad) = (DATA + 0x100, DATA + 0x200);
                let (state_vt, pad_vt, screen_vt) = (DATA + 0x800, DATA + 0x900, DATA + 0xa00);
                fill(c, &mut r, this, 0x10);
                c.bus.write32(this + 8, state_vt);
                c.bus.write32(this + 0x10, owner);
                c.bus.write32(owner + 0xc, pad);
                c.bus.write32(pad + 8, pad_vt);
                oc383_vtable(c, &mut r, state_vt, &[0x40, 0x48]);
                oc383_vtable(c, &mut r, pad_vt, &[0x160, 0x168, 0x148, 0x140]);
                oc383_vtable(c, &mut r, screen_vt, &[0x20]);
                let bits = r.next_u32();
                for (i, off) in [0x168u32, 0x148, 0x140].into_iter().enumerate() {
                    if bits >> i & 1 == 0 {
                        oc383_slot_zero(c, pad_vt, off);
                    }
                }
                if bits & 8 == 0 {
                    oc383_slot_zero(c, state_vt, 0x40);
                }
                if bits & 16 == 0 {
                    oc383_slot_zero(c, screen_vt, 0x20);
                }
                // Screens: 0..3 nodes, some active (+0x14 bit 0).
                let n = (seed % 4) as usize;
                let nodes: Vec<u32> = (0..n as u32).map(|i| DATA + 0x1000 + 0x100 * i).collect();
                for &node in &nodes {
                    fill(c, &mut r, node, 8);
                    c.bus.write32(node + 8, screen_vt);
                }
                oc383_ui_list(c, &mut r, this + UI_STATE_SCREENS, &nodes, DATA + 0x1800);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mod_obj_v05_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s11::f_00356608__generated,
            mod_obj_v05,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6608, 1);
                c.pc = 0x0035_6608;
                let this = DATA;
                let (world, obj, mods, rot, light) = (
                    DATA + 0x100,
                    DATA + 0x200,
                    DATA + 0x300,
                    DATA + 0x400,
                    DATA + 0x500,
                );
                let cache = DATA + 0x1000;
                fill(c, &mut r, this, 0x10);
                c.bus.write32(GAME_GP + MOVE_NODE_WORLD_GP, world);
                c.bus.write32(world, cache);
                c.bus.write32(this + MOVE_NODE_OBJECT, obj);
                fill(c, &mut r, obj, 0x28);
                // Visibility: group 0xff, or group g (type 6?) with slot s (state 3?).
                let g = if seed % 3 == 0 {
                    0xff
                } else {
                    r.next_u32() % 4
                };
                let slot = r.next_u32() % 4;
                c.bus.write8(obj + 0x7d, g as u8);
                c.bus.write16(obj + 0x7e, slot as u16);
                for i in 0..4 {
                    c.bus.write32(
                        cache + 8 * i + 0x24,
                        if r.next_u32() % 4 != 0 { 6 } else { 5 },
                    );
                    c.bus.write32(
                        cache + 0x18 * i + 0x3f0,
                        if r.next_u32() % 4 != 0 { 3 } else { 1 },
                    );
                }
                let bits = r.next_u32();
                c.bus.write32(
                    this + MOVE_NODE_MODIFIERS,
                    if bits & 1 == 0 { 0 } else { mods },
                );
                c.bus.write32(mods + 4, if bits & 2 == 0 { 0 } else { rot });
                c.bus
                    .write32(mods + 8, if bits & 4 == 0 { 0 } else { light });
                fill(c, &mut r, rot, 0x10);
                // UV scroll speed 0 (the modifier's short path) mostly.
                c.bus
                    .write32(rot + 0x10, if bits & 8 == 0 { 0 } else { r.f32(2.0) });
                oc383_floats(c, &mut r, rot + 0x20, 4, 2.0);
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP + AVALANCHE_GRMAN_GP, grman);
                c.bus.write32(grman + 0x10d8, DATA + 0x5800);
                oc383_vtable(c, &mut r, DATA + 0x5800, &[0x2f8]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (r.next_u32() & 0xff) as u64;
            },
        );
    }

    #[test]
    fn section_allocator_alloc_matches() {
        check(
            ee::s13::f_003a7878__generated,
            section_allocator_alloc,
            300,
            &|c, seed| {
                let mut r = Rng(0x7878 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = SECTION_ALLOC_ENTRY;
                let this = DATA;
                let pool = DATA + 0x100;
                // The section's blocks (0..3, listed from the head).
                let n = seed % 4;
                let block = |i: u32| DATA + 0x400 + 0x40 * i;
                for i in 0..n {
                    let b = block(i);
                    c.bus.write32(b, DATA + 0x10000 * (i + 1));
                    let size = 0x40 + r.next_u32() % 0x100;
                    c.bus.write32(b + 4, size);
                    c.bus.write32(b + 8, r.next_u32() % (size + 0x20));
                    c.bus.write32(b + 0xc, r.next_u32() % 0x40);
                    c.bus
                        .write32(b + 0x14, if i + 1 < n { block(i + 1) } else { 0 });
                }
                c.bus.write32(this, pool);
                c.bus.write32(
                    this + SECTION_ALLOC_COUNT,
                    if seed % 9 == 8 && n > 0 { 1 } else { n },
                );
                c.bus
                    .write32(this + SECTION_ALLOC_HEAD, if n == 0 { 0 } else { block(0) });
                let cur = if n == 0 { 0 } else { block(r.next_u32() % n) };
                c.bus.write32(this + SECTION_ALLOC_CUR, cur);
                // The pool's free blocks.
                for i in 0..2 {
                    let b = DATA + 0x800 + 0x40 * i;
                    fill(c, &mut r, b, 0x10);
                    c.bus.write32(b + 8, r.next_u32() % 0x40);
                    c.bus.write32(b + 0x14, if i == 0 { b + 0x40 } else { 0 });
                }
                c.bus.write32(pool + 8, 2);
                c.bus.write32(pool + 0x10, DATA + 0x800);
                let size = r.next_u32() % 0x80;
                let flags = (r.next_u32() % 4) << 24 | r.next_u32() & 0xf0ff_ffff;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = size as u64;
                c.g[A2][0] = flags as u64;
                if n > 0 && seed % 7 >= 5 {
                    // Resume inside the frame at a back edge.
                    let sp = crate::difftest::SP - 0x40;
                    fill(c, &mut r, sp, 0x10);
                    c.bus.write64(sp, crate::difftest::RA as u64);
                    c.g[SP][0] = sx(sp);
                    c.g[S0][0] = sx(this);
                    c.g[S1][0] = size as u64;
                    c.g[S2][0] = flags as u64;
                    c.g[A2][0] = 4 << (r.next_u32() % 4);
                    c.g[T3][0] = sx(DATA + 0x2000);
                    c.g[T4][0] = 0;
                    if seed % 7 == 5 {
                        c.pc = SECTION_ALLOC_SCAN;
                    } else {
                        let b = block(r.next_u32() % n);
                        c.pc = SECTION_ALLOC_TAKE;
                        c.g[A3][0] = sx(b);
                        c.g[T0][0] = sx(c.bus.read32(b + 8));
                        c.g[T1][0] = (r.next_u32() % 16) as u64;
                        c.g[V1][0] = c.g[T1][0] + size as u64;
                    }
                }
            },
        );
    }

    #[test]
    fn ieee754_rem_pio2f_matches() {
        check(
            ee::s15::f_0040e878__generated,
            ieee754_rem_pio2f,
            400,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0xe878, 0x0040_e878);
                let mag = match seed % 10 {
                    0 => r.next_u32() % 0x3f49_0fd9,              // |x| <= pi/4
                    1 => 0x3f49_0fd9 + r.next_u32() % 0xcd_bc0b,  // < 3pi/4
                    2 => 0x3fc9_0fd0 + r.next_u32() % 0x10,       // ~pi/2
                    3 => 0x4016_cbe4 + r.next_u32() % 0x1b2_4000, // medium
                    4 => (n_pio2(r.next_u32() % 31 + 1)) ^ (r.next_u32() % 0x100), // near n*pi/2
                    5 => 0x4349_0f81 + r.next_u32() % 0x3b00_0000, // large
                    6 => [0x7f80_0000, 0x7fc0_0000, 0x7f7f_ffff, 0x4349_0f80]
                        [(r.next_u32() % 4) as usize],
                    7 => 0x4400_0000 + r.next_u32() % 0x0800_0000, // large, few bits
                    _ => r.next_u32() & 0x7fff_ffff,
                };
                // Large values with trailing zero parts (the nx scan).
                let mag = if seed % 10 == 7 {
                    mag & 0xffff_0000
                } else {
                    mag
                };
                let sign = if r.next_u32() & 1 == 0 {
                    0
                } else {
                    0x8000_0000
                };
                c.f[12] = mag | sign;
                fill(c, &mut r, DATA, 4);
                c.g[A0][0] = sx(DATA);
            },
        );

        /// n * pi/2 as a float's bits (the npio2_hw entries' neighbourhood).
        fn n_pio2(n: u32) -> u32 {
            (n as f32 * std::f32::consts::FRAC_PI_2).to_bits()
        }
    }
}
