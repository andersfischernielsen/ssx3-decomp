//! Staging wave 204 (oc231): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use crate::call::VSlot;
use crate::guest::Addr;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_2b00, lens_fx_project_source, "LensFX_ProjectSource"),
    (0x0035_3448, modifier_block_save, "ModifierBlock_Save"),
    (0x002d_dd30, board_wake_fx_build_trail, "cBoardWakeFX_BuildTrail"),
    (0x003a_8668, world_cache_update_hull_lru, "cWorldCache_UpdateHullLRU"),
    (0x0023_1d60, app_module_render_fade, "AppModule_RenderFade"),
    (0x002b_75d0, world_sound_refresh_nearby_emitters, "cWorldSound_RefreshNearbyEmitters"),
    (0x002d_bf98, ps2_fog_particle_man_render, "cPS2FogParticleMan_Render"),
    (0x002f_0cd0, ps2_visual_effects_man_render, "PS2VisualEffectsMan_Render"),
    (0x0031_b178, mat4_determinant, "Mat4_Determinant"),
];

// ---------------------------------------------------------------------------
// LensFX_ProjectSource (0x2e2b00)
// ---------------------------------------------------------------------------

/// The camera / view object (`*(gp + 0x2a90)`).
const LENS_CAMERA_GP: u32 = 0x2a90;
/// The camera's embedded projector (vtable pointer at +0x10d8).
const CAM_PROJECTOR_VT: u32 = 0x10d8;
/// Projector slot: `IsOccluded(point)` (non-zero hides the flare).
const VT_OCCLUDED: u32 = 0x2d8;
/// Projector slot: `Project(out, point)` (screen x/y at +0/+4, depth +8).
const VT_PROJECT: u32 = 0x2e0;
/// Lens flare: the light source it follows.
const LENS_SOURCE: u32 = 0x20;
/// Lens flare: visible / on-screen flags (halfwords).
const LENS_VISIBLE: u32 = 0x2c;
const LENS_ON_SCREEN: u32 = 0x2e;
/// Light source: world position (x, y, z) and its kind bits (`& 0x70`).
const SRC_POS: u32 = 0x1c;
const SRC_KIND: u32 = 0xc;

/// `void LensFX_ProjectSource(LensFX* lens, View* view)` (0x2e2b00): project
/// the flare's light source onto the screen. Hidden (occluded) sources clear
/// both flags. Visible ones in front of the camera get a sample rectangle
/// sized from their depth (clamped to 16x8 pixels, at least 1x1) centred on
/// the projected point and clipped to the view, a projected flare radius
/// (100/80/200 world units by source kind) and a horizontal screen offset
/// normalised to -1..1.
pub fn lens_fx_project_source(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue; the source position as a homogeneous point at sp+0.
        c.cycles += 27;
        let frame = Frame::open(c, 0xc0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x80);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x70);
        let lens = r32(c, S0);
        let view = r32(c, S1);
        let point = sp(c);
        let gp = c.g[GP][0] as u32;
        c.g[S2][0] = Addr(gp).off(LENS_CAMERA_GP).lw(c);
        let cam = r32(c, S2);
        c.g[V0][0] = Addr(lens).off(LENS_SOURCE).lw(c);
        let src = r32(c, V0);
        c.lwc1(0, src + SRC_POS);
        c.swc1(0, point);
        c.lwc1(1, src + SRC_POS + 4);
        c.swc1(1, point + 4);
        c.lwc1(0, src + SRC_POS + 8);
        c.swc1(2, point + 0xc);
        c.swc1(0, point + 8);

        // Occluded?
        c.g[V1][0] = Addr(cam).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_OCCLUDED);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[S2][0]);
        call_ptr(c, slot.func as u32, 0x002e_2b64)?;
        c.g[S3][0] = 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            Addr(lens).off(LENS_ON_SCREEN).sh(c, 0);
            Addr(lens).off(LENS_VISIBLE).sh(c, 0);
            c.cycles += 1;
            return epilogue(c, frame);
        }

        // Project it (screen point at sp+0x10).
        c.cycles += 16;
        c.g[A0][0] = sx(point.wrapping_add(0x10));
        Addr(lens).off(LENS_VISIBLE).sh(c, 1);
        c.g[A2][0] = c.g[SP][0];
        c.g[V0][0] = Addr(cam).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V0][0];
        let slot = VSlot::read(c, vt, VT_PROJECT);
        c.g[A1][0] = slot.delta;
        c.g[V1][0] = slot.func;
        c.g[A1][0] = slot.this(c.g[S2][0]);
        call_ptr(c, slot.func as u32, 0x002e_2b98)?;
        let point = sp(c);
        let gp = c.g[GP][0] as u32;
        c.lwc1(2, point + 0x18);
        c.lwc1(0, gp.wrapping_sub(0x3a40));
        c.lwc1(1, gp.wrapping_sub(0x3a3c));
        c.mul_s(2, 2, 0);
        c.c_le_s(2, 1);
        if c.fcc() {
            // Behind the near plane: visible but not on screen.
            return epilogue(c, frame);
        }

        // Sample rectangle size from the depth: w = clamp(800 d, 1, 16),
        // h = clamp(400 d, 1, 8).
        c.cycles += 13;
        c.g[AT][0] = 0x4448_0000;
        c.mtc1(AT, 1);
        c.g[V1][0] = 0x10;
        c.g[AT][0] = 0x43c8_0000;
        c.mtc1(AT, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(0, 2, 0);
        c.cvt_w_s(2, 1);
        c.mfc1(V0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(A1, 1);
        Addr(lens).off(LENS_ON_SCREEN).sh(c, 1);
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[A0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[V1][0];
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 8;
        if (c.g[A1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V1][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.g[A1][0];
            }
        }

        // The rectangle around the projected point, clipped to the view.
        c.cycles += 39;
        c.g[A1][0] = c.g[V1][0];
        c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
        c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
        c.lwc1(4, point + 0x14);
        c.mtc1(V1, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(3, point + 0x10);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(AT, 2);
        c.g[AT][0] = 0x4100_0000;
        c.mtc1(AT, 7);
        c.mtc1(A0, 5);
        c.cvt_s_w(5, 5);
        c.sub_s(1, 4, 1);
        c.g[AT][0] = 0x4180_0000;
        c.mtc1(AT, 6);
        c.sub_s(0, 3, 0);
        c.swc1(7, lens + 0x1c);
        c.sub_s(4, 4, 2);
        c.swc1(5, lens + 0x8);
        c.sub_s(3, 3, 7);
        c.swc1(1, lens + 0x4);
        c.mtc1(A1, 2);
        c.cvt_s_w(2, 2);
        c.swc1(0, lens);
        c.swc1(4, lens + 0x14);
        c.swc1(3, lens + 0x10);
        c.swc1(2, lens + 0xc);
        c.swc1(6, lens + 0x18);
        c.mtc1(0, 2);
        c.lwc1(0, view);
        c.lwc1(1, view + 0x8);
        c.c_le_s(2, 3);
        c.mov_s(5, 2);
        c.add_s(0, 0, 1);
        c.sub_s(0, 0, 6);
        if c.fcc() {
            c.cycles += 1;
            c.min_s(5, 3, 0);
        }
        c.cycles += 8;
        c.swc1(5, lens + 0x10);
        c.c_le_s(2, 4);
        c.mov_s(3, 2);
        c.lwc1(0, view + 0x4);
        c.lwc1(1, view + 0xc);
        c.add_s(0, 0, 1);
        c.sub_s(0, 0, 7);
        if c.fcc() {
            c.cycles += 1;
            c.min_s(3, 4, 0);
        }

        // Flare radius by source kind (f2; 0 for the other kinds).
        c.cycles += 7;
        c.swc1(3, lens + 0x14);
        c.g[A0][0] = 0x20;
        c.g[V0][0] = Addr(lens).off(LENS_SOURCE).lw(c);
        let src = r32(c, V0);
        c.g[V1][0] = Addr(src).off(SRC_KIND).lw(c);
        c.g[V1][0] &= 0x70;
        let kind = c.g[V1][0];
        c.g[V0][0] = (kind < 0x21) as u64;
        let radius_bits = if kind == 0x20 {
            c.cycles += 4;
            Some(0x42a0_0000)
        } else if kind < 0x21 {
            c.cycles += 2;
            c.g[V0][0] = 0x10;
            c.cycles += 2;
            if kind == 0x10 {
                c.cycles += 4;
                Some(0x42c8_0000)
            } else {
                c.cycles += 2;
                None
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x10;
            c.cycles += 3;
            c.g[V0][0] = 0x40;
            c.cycles += 2;
            if kind == 0x40 {
                Some(0x4348_0000)
            } else {
                None
            }
        };
        if let Some(bits) = radius_bits {
            c.g[AT][0] = bits;
            c.mtc1(AT, 2);
        }
        c.lwc1(0, view + 0x18);
        if radius_bits.is_some() || kind < 0x21 {
            c.cycles += 1;
        }
        c.g[A0][0] = sx(point.wrapping_add(0x40));

        // Project the point offset by the radius (out at sp+0x40).
        c.cycles += 41;
        c.lwc1(1, view + 0x28);
        c.g[A2][0] = sx(point.wrapping_add(0x30));
        c.lwc1(3, view + 0x38);
        c.mul_s(4, 0, 2);
        c.mul_s(5, 1, 2);
        c.swc1(0, point + 0x20);
        c.mul_s(2, 3, 2);
        c.swc1(1, point + 0x24);
        c.swc1(4, point + 0x40);
        c.swc1(3, point + 0x28);
        c.swc1(2, point + 0x48);
        c.swc1(4, point + 0x50);
        c.swc1(2, point + 0x58);
        c.swc1(5, point + 0x44);
        c.swc1(5, point + 0x54);
        Addr(point).off(0x4c).sw(c, 0);
        c.lqc2(3, point);
        c.lqc2(4, point + 0x40);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, point + 0x60);
        c.lq(2, point + 0x60);
        c.sq(2, point + 0x30);
        c.g[V1][0] = Addr(cam).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_PROJECT);
        c.g[A1][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A1][0] = slot.this(c.g[S2][0]);
        call_ptr(c, slot.func as u32, 0x002e_2dc0)?;
        let point = sp(c);
        let gp = c.g[GP][0] as u32;
        c.lwc1(0, point + 0x48);
        c.lwc1(3, point + 0x10);
        c.swc1(0, lens + 0x24);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 4);
        c.lwc1(0, view);
        c.lwc1(2, view + 0x8);
        c.sub_s(0, 3, 0);
        c.lwc1(1, gp.wrapping_sub(0x3a38));
        c.add_s(0, 0, 0);
        c.div_s(0, 0, 2);
        c.sub_s(0, 0, 4);
        c.mul_s(1, 0, 1);
        c.swc1(1, lens + 0x28);
        c.cycles += 1;
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 6;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ModifierBlock_Save (0x353448)
// ---------------------------------------------------------------------------

/// `this = obj + vt[off].delta; v[func_reg] = vt[off].func; call` (the
/// game's virtual-call sequence: `lh a0`, `lw func`, `addu a0`, `jalr`).
fn vcall(c: &mut Cpu, obj: u64, vt: u64, off: u32, func_reg: usize, ret: u32) -> crate::call::Flow {
    let slot = VSlot::read(c, vt, off);
    c.g[func_reg][0] = slot.func;
    c.g[A0][0] = slot.this(obj);
    call_ptr(c, slot.func as u32, ret)
}

/// Modifier block: the modifiers it holds.
const MB_PRIMARY: u32 = 0x0;
const MB_UV_SCROLL: u32 = 0x4;
const MB_RAIL: u32 = 0x8;
const MB_SECONDARY: u32 = 0xc;
/// Intrusive lists (`next` at +0, the object's vtable at +8).
const MB_ID_LIST: u32 = 0x10;
const MB_EXTRA_LIST: u32 = 0x1c;
/// The 0x34-byte header at sp+0: flags, two ids, two counts, 32 id bytes.
const HDR_FLAGS: u32 = 0x0;
const HDR_PRIMARY_ID: u32 = 0x4;
const HDR_SECONDARY_ID: u32 = 0x8;
const HDR_ID_COUNT: u32 = 0xc;
const HDR_EXTRA_COUNT: u32 = 0x10;
const HDR_IDS: u32 = 0x14;

/// `void ModifierBlock_Save(ModifierBlock* block, Stream* stream)`
/// (0x353448): write a header (which modifiers are present, the ids of the
/// primary and secondary ones, the id byte of each node in the id list, the
/// length of the extra list) with `stream->Write(&hdr, 0x34)`, then let every
/// present modifier and list node save itself to the stream.
pub fn modifier_block_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        let hdr = sp(c);
        c.g[V1][0] = 0x1f;
        frame.save(c, S1, 0x70);
        c.g[V0][0] = sx(hdr + HDR_IDS + 0x1f);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S3][0] = sx(hdr + HDR_IDS);
        frame.save_ra(c, 0x40);
        Addr(hdr).off(HDR_FLAGS).sw(c, 0);
        Addr(hdr).off(HDR_SECONDARY_ID).sw(c, 0);
        Addr(hdr).off(HDR_PRIMARY_ID).sw(c, 0);
        Addr(hdr).off(HDR_ID_COUNT).sw(c, 0);
        Addr(hdr).off(HDR_EXTRA_COUNT).sw(c, 0);
        let block = r32(c, S1);
        let stream = c.g[S2][0];

        // Clear the 32 id bytes, last to first.
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sb(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if (c.g[V1][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_3488);
            }
        }

        // Flags and ids.
        c.cycles += 3;
        c.g[A1][0] = Addr(block).off(MB_PRIMARY).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 8;
            or_flag(c, hdr, 0x1);
            let obj = c.g[A1][0];
            c.g[V1][0] = Addr(obj as u32).lw(c);
            let vt = c.g[V1][0];
            vcall(c, obj, vt, 0xd0, V0, 0x0035_34cc)?;
            let id = c.g[V0][0];
            Addr(sp(c)).off(HDR_PRIMARY_ID).sw(c, id);
        }
        for (field, bit) in [(MB_UV_SCROLL, 0x2), (MB_RAIL, 0x4)] {
            c.cycles += 3;
            let present = Addr(block).off(field).lw(c) != 0;
            c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
            if present {
                c.cycles += 2;
                or_flag(c, hdr, bit);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = Addr(block).off(MB_SECONDARY).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 8;
            or_flag(c, hdr, 0x8);
            let obj = c.g[A1][0];
            c.g[V1][0] = Addr(obj as u32).off(4).lw(c);
            let vt = c.g[V1][0];
            vcall(c, obj, vt, 0x28, V0, 0x0035_3520)?;
            let id = c.g[V0][0];
            Addr(sp(c)).off(HDR_SECONDARY_ID).sw(c, id);
        }

        // One id byte per node of the id list.
        c.cycles += 3;
        c.g[S0][0] = Addr(block).off(MB_ID_LIST).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_ID_COUNT).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
            loop {
                c.cycles += 12;
                let (node, vt) = (c.g[S0][0], c.g[V1][0]);
                vcall(c, node, vt, 0x10, V0, 0x0035_3548)?;
                let hdr = sp(c);
                c.g[A0][0] = Addr(hdr).off(HDR_ID_COUNT).lw(c);
                c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                let id = c.g[V0][0];
                Addr(r32(c, V1)).sb(c, id);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                let n = c.g[A0][0];
                Addr(hdr).off(HDR_ID_COUNT).sw(c, n);
                c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(hdr).off(HDR_ID_COUNT).lw(c);
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3538);
                }
            }
        }
        let hdr = sp(c);
        c.cycles += 2;
        let has_ids = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if has_ids {
            c.cycles += 2;
            or_flag(c, hdr, 0x10);
        }

        // Count the extra list.
        c.cycles += 3;
        c.g[V1][0] = Addr(block).off(MB_EXTRA_LIST).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_EXTRA_COUNT).lw(c);
        if c.g[V1][0] != 0 {
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let n = c.g[V0][0];
                Addr(hdr).off(HDR_EXTRA_COUNT).sw(c, n);
                c.g[V1][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = Addr(hdr).off(HDR_EXTRA_COUNT).lw(c);
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3588);
                }
            }
        }
        c.cycles += 2;
        let has_extra = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if has_extra {
            c.cycles += 2;
            or_flag(c, hdr, 0x20);
        }

        // stream->Write(&hdr, 0x34).
        c.cycles += 10;
        c.g[V0][0] = Addr(stream as u32).lw(c);
        c.g[A2][0] = 0x34;
        c.g[A1][0] = sx(hdr);
        let vt = c.g[V0][0];
        vcall(c, stream, vt, 0x8, V1, 0x0035_35d0)?;
        c.g[A2][0] = Addr(block).off(MB_PRIMARY).lw(c);
        c.g[A1][0] = stream;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let obj = c.g[A2][0];
            c.g[V0][0] = Addr(obj as u32).lw(c);
            let vt = c.g[V0][0];
            vcall(c, obj, vt, 0xd8, V1, 0x0035_35f0)?;
        }

        // The UV-scroll and rail modifiers save themselves.
        c.cycles += 3;
        c.g[A0][0] = Addr(block).off(MB_UV_SCROLL).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = stream;
            Callee { addr: 0x0035_fd98, f: ee::s12::f_0035fd98 }.call(c, 0x0035_3604)?;
        }
        c.g[A0][0] = Addr(block).off(MB_RAIL).lw(c);
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = stream;
            Callee { addr: 0x0035_f688, f: ee::s12::f_0035f688 }.call(c, 0x0035_3618)?;
        }
        c.g[A2][0] = Addr(block).off(MB_SECONDARY).lw(c);
        c.cycles += 2;
        c.g[A1][0] = stream;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let obj = c.g[A2][0];
            c.g[V0][0] = Addr(obj as u32).off(4).lw(c);
            let vt = c.g[V0][0];
            vcall(c, obj, vt, 0x30, V1, 0x0035_3638)?;
        }

        // The id-list nodes, if any had ids.
        let hdr = sp(c);
        c.cycles += 3;
        let id_count = Addr(hdr).off(HDR_ID_COUNT).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_EXTRA_COUNT).lw(c);
        if (id_count as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = Addr(block).off(MB_ID_LIST).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = stream;
                    let (node, vt) = (c.g[S0][0], c.g[V0][0]);
                    vcall(c, node, vt, 0x20, V1, 0x0035_366c)?;
                    c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(sp(c)).off(HDR_EXTRA_COUNT).lw(c);
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3658);
                    }
                }
            }
        }

        // The extra-list nodes, if any.
        c.cycles += 2;
        let has_extra = (c.g[V0][0] as i64) > 0;
        frame.restore(c, S0, 0x80);
        if has_extra {
            c.cycles += 3;
            c.g[S0][0] = Addr(block).off(MB_EXTRA_LIST).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = stream;
                    let (node, vt) = (c.g[S0][0], c.g[V0][0]);
                    vcall(c, node, vt, 0x10, V1, 0x0035_36ac)?;
                    c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3698);
                    }
                }
            }
            frame.restore(c, S0, 0x80);
        }

        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });

    /// `v0 |= bit; hdr.flags = v0` (v0 holds the flags just loaded).
    fn or_flag(c: &mut Cpu, hdr: u32, bit: u64) {
        c.g[V0][0] |= bit;
        let v = c.g[V0][0];
        Addr(hdr).off(HDR_FLAGS).sw(c, v);
    }
}

// ---------------------------------------------------------------------------
// VU0 macro sequences shared by the ports below (the original's exact ops)
// ---------------------------------------------------------------------------

/// `mfc1 gpr, f; lqc2 vf4, src; qmtc2 gpr, vf3; vmul.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, dst`: `dst = src * f`.
fn vscale(c: &mut Cpu, gpr: usize, f: usize, src: u32, dst: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, src);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// The xyz length of the vector at `src` (`vf3`), left in `gpr` by
/// `cfc2 gpr, Q` after `vsqrt`.
fn vlen(c: &mut Cpu, gpr: usize, src: u32) {
    c.lqc2(3, src);
    vlen_vf3(c, gpr);
}

/// [`vlen`] of the vector already in vf3.
fn vlen_vf3(c: &mut Cpu, gpr: usize) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
}

/// `lqc2 vf3, a; lqc2 vf4, b; v{add,sub}.xyzw vf5, vf3, vf4; sqc2 vf5, dst`.
fn vadd(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, dst);
}
fn vsub(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, dst);
}

/// `lq gpr, src; sq gpr, dst` (the compiler's quadword copies through a GPR).
fn qcopy(c: &mut Cpu, gpr: usize, src: u32, dst: u32) {
    c.lq(gpr, src);
    c.sq(gpr, dst);
}

// ---------------------------------------------------------------------------
// cBoardWakeFX_BuildTrail (0x2ddd30)
// ---------------------------------------------------------------------------

/// Board wake: the rider, ring bookkeeping and the trail shape.
const WAKE_RIDER: u32 = 0x0;
const WAKE_HEAD: u32 = 0x8;
const WAKE_ELEMS: u32 = 0xc;
/// `rings[n]` (pointers to 0x30-byte segments, one per head slot).
const WAKE_RINGS: u32 = 0x10;
const WAKE_PHASE: u32 = 0x38;
const WAKE_SPREAD: u32 = 0x60;
const WAKE_CURVE: u32 = 0x70;
const WAKE_BACK: u32 = 0x80;
const WAKE_WIDTH: u32 = 0x90;
const WAKE_ALPHA: u32 = 0x94;
const WAKE_POINTS: u32 = 0xb4;
/// The rider's velocity (+0x1e0) and up vector (+0x370).
const RIDER_VEL: u32 = 0x1e0;
const RIDER_UP: u32 = 0x370;
/// A trail element (0x70 bytes): points from +0, the direction at +0x50,
/// the width at +0x60.
const ELEM_SIZE: u32 = 0x70;
const ELEM_DIR: u32 = 0x50;
const ELEM_WIDTH: u32 = 0x60;
const ELEM_CLEAR: u32 = 0x64;
/// Ring segment (0x30 bytes): position at +0x20, alpha at +0x1c.
const SEG_SIZE: u32 = 0x30;
const SEG_POS: u32 = 0x20;
const SEG_ALPHA: u32 = 0x1c;

/// `void cBoardWakeFX_BuildTrail(cBoardWakeFX* wake, const Vec4* pos, float
/// width)` (0x2ddd30): fill the head trail element. The trail width is
/// modulated by 1D fractal noise of the phase; the direction is the wake's
/// back vector scaled by the rider's speed, minus the reversed velocity,
/// times 0.85; the points follow `w * (0.75 t^2 curve + t spread)` with the
/// last one pulled in by 0.9. The head slot of every ring gets the position
/// `pos - 5 up` and the alpha `(int)(alpha * 128)`.
pub fn board_wake_fx_build_trail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 83;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x80);
        let wake = r32(c, S0);
        c.mov_s(20, 12);
        c.g[A0][0] = 3;
        c.lwc1(12, wake + WAKE_PHASE);
        Callee { addr: 0x002d_1928, f: ee::s09::f_002d1928 }.call(c, 0x002d_dd5c)?;
        let sp0 = sp(c);
        let gp = c.g[GP][0] as u32;

        // The head element; clear it and its first point.
        c.g[V0][0] = Addr(wake).off(WAKE_HEAD).lw(c);
        c.g[V1][0] = ELEM_SIZE as u64;
        c.g[A1][0] = Addr(wake).off(WAKE_ELEMS).lw(c);
        c.g[A0][0] = 0x50_0000;
        c.mult(0, V0, V0, V1);
        c.lwc1(3, wake + WAKE_WIDTH);
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(AT, 1);
        c.g[A0][0] = sx(0x004f_f120);
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 2);
        c.g[A2][0] = 0;
        c.mul_s(0, 0, 1);
        c.lwc1(4, gp.wrapping_sub(0x3be4));
        c.g[T1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        let elem = r32(c, T1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        Addr(elem).off(ELEM_CLEAR).sw(c, 0);
        c.add_s(0, 0, 1);
        qcopy(c, V0, 0x004f_f120, elem);
        // w = width * (noise * 0.25 + 1).
        c.mul_s(3, 3, 0);

        // dir = (-back * |vel| - (-vel)) * k.
        vscale(c, A0, 2, wake + WAKE_BACK, sp0 + 0x20);
        c.lq(V1, sp0 + 0x20);
        c.g[V0][0] = Addr(wake).off(WAKE_RIDER).lw(c);
        let rider = r32(c, V0);
        c.sq(V1, sp0 + 0x10);
        vlen(c, A0, rider + RIDER_VEL);
        c.mtc1(A0, 0);
        vscale(c, V1, 0, sp0 + 0x10, sp0 + 0x20);
        c.lq(V0, sp0 + 0x20);
        c.g[V1][0] = Addr(wake).off(WAKE_RIDER).lw(c);
        c.sq(V0, sp0);
        vscale(c, A0, 2, rider + RIDER_VEL, sp0 + 0x20);
        qcopy(c, V0, sp0 + 0x20, sp0 + 0x10);
        vsub(c, sp0, sp0 + 0x10, sp0 + 0x40);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x30);
        vscale(c, V1, 4, sp0 + 0x30, sp0 + 0x40);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x20);
        c.sq(V0, elem + ELEM_DIR);

        // The trail points.
        c.g[V1][0] = Addr(wake).off(WAKE_POINTS).lw(c);
        c.g[A0][0] = c.g[T1][0];
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 3;
            c.lwc1(4, gp.wrapping_sub(0x3be0));
            c.g[AT][0] = 0x3f40_0000;
            c.mtc1(AT, 2);
            loop {
                // t = i / (n - 1); a = t^2; the last t *= 0.9.
                c.cycles += 8;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
                c.mtc1(A2, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.div_s(1, 1, 0);
                let last = c.g[A2][0] == c.g[V0][0];
                c.mul_s(0, 1, 1);
                if last {
                    c.cycles += 1;
                    c.mul_s(1, 1, 4);
                }
                // point = w * (curve * a * 0.75 + spread * t).
                c.cycles += 40;
                vscale(c, V0, 0, wake + WAKE_CURVE, sp0 + 0x60);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                qcopy(c, V0, sp0 + 0x60, sp0 + 0x50);
                vscale(c, V1, 2, sp0 + 0x50, sp0 + 0x60);
                qcopy(c, V0, sp0 + 0x60, sp0 + 0x40);
                vscale(c, V1, 1, wake + WAKE_SPREAD, sp0 + 0x70);
                qcopy(c, V0, sp0 + 0x70, sp0 + 0x60);
                vadd(c, sp0 + 0x40, sp0 + 0x60, sp0 + 0x70);
                qcopy(c, V0, sp0 + 0x70, sp0 + 0x30);
                vscale(c, V1, 3, sp0 + 0x30, sp0 + 0x70);
                qcopy(c, V0, sp0 + 0x70, sp0 + 0x20);
                let point = r32(c, A0);
                c.sq(V0, point);
                c.g[V1][0] = Addr(wake).off(WAKE_POINTS).lw(c);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A0][0] = sx(point.wrapping_add(0x10));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_de88);
                }
            }
        }

        // Ring 0's head segment: position pos - 5 up, alpha * 128.
        c.cycles += 37;
        c.g[V1][0] = Addr(wake).off(WAKE_RINGS).lw(c);
        c.g[A1][0] = SEG_SIZE as u64;
        c.g[A0][0] = Addr(wake).off(WAKE_HEAD).lw(c);
        c.g[A3][0] = 1;
        c.g[V0][0] = Addr(wake).off(WAKE_RIDER).lw(c);
        let rider = r32(c, V0);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 0);
        c.mult(0, A0, A0, A1);
        vscale(c, A2, 0, rider + RIDER_UP, sp0 + 0x40);
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 1);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x30);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        let pos = r32(c, S1);
        vsub(c, pos, sp0 + 0x30, sp0 + 0x40);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x20);
        let seg0 = r32(c, A0);
        c.sq(V0, seg0 + SEG_POS);
        c.g[V1][0] = Addr(wake).off(WAKE_HEAD).lw(c);
        c.lwc1(0, wake + WAKE_ALPHA);
        c.mult(0, V1, V1, A1);
        c.g[V0][0] = Addr(wake).off(WAKE_RINGS).lw(c);
        c.mul_s(0, 0, 1);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.cvt_w_s(1, 0);
        let seg0 = r32(c, V1);
        c.swc1(1, seg0 + SEG_ALPHA);

        // Copy it into the head slot of rings 1..n.
        c.g[V0][0] = Addr(wake).off(WAKE_POINTS).lw(c);
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[T0][0] = SEG_SIZE as u64;
            c.g[A2][0] = sx(wake + WAKE_RINGS + 4);
            loop {
                c.cycles += 21;
                c.g[V1][0] = Addr(wake).off(WAKE_HEAD).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[A0][0] = Addr(wake).off(WAKE_RINGS).lw(c);
                c.mult(0, V1, V1, T0);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                let (from, to) = (r32(c, A0), r32(c, V1));
                qcopy(c, V0, from + SEG_POS, to + SEG_POS);
                c.g[A0][0] = Addr(wake).off(WAKE_HEAD).lw(c);
                c.g[V0][0] = Addr(wake).off(WAKE_RINGS).lw(c);
                c.mult(0, A0, A0, T0);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V0)).off(SEG_ALPHA).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                let alpha = c.g[V1][0];
                Addr(r32(c, A0)).off(SEG_ALPHA).sw(c, alpha);
                c.g[V0][0] = Addr(wake).off(WAKE_POINTS).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_dfe8);
                }
            }
        }
        c.swc1(20, elem + ELEM_WIDTH);

        c.cycles += 6;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cWorldCache_UpdateHullLRU (0x3a8668)
// ---------------------------------------------------------------------------

/// World cache: the hull streamer (+0x10), the stream manager (+0x300) and
/// its busy flag (+0x390), the hull page slots and the expiry time.
const WC_STREAMER: u32 = 0x10;
const WC_STREAM_MAN: u32 = 0x300;
const WC_BUSY: u32 = 0x390;
const WC_SLOT_COUNT: u32 = 0x3ec;
const WC_SLOTS: u32 = 0x3f0;
const WC_EXPIRY: u32 = 0x1bf0;
/// A hull page slot (0x18 bytes, from +0x3f0).
const SLOT_SIZE: u32 = 0x18;
const SLOT_STATE: u32 = 0x0;
const SLOT_LOCKED: u32 = 0x4;
const SLOT_PAGE: u32 = 0x8;
const SLOT_PRIORITY: u32 = 0xc;
const SLOT_TIME: u32 = 0x10;
const SLOT_STAMP: u32 = 0x14;
/// Slot states: free, wanted, loaded, expiring, freeing.
const HULL_WANTED: u64 = 1;
const HULL_LOADED: u64 = 2;
const HULL_EXPIRING: u64 = 3;
const HULL_FREEING: u64 = 4;
/// Camera vtable slot returning the render frame number.
const VT_FRAME: u32 = 0x390;

/// `void cWorldCache_UpdateHullLRU(cWorldCache* wc)` (0x3a8668): age the
/// hull page slots. A slot that is unlocked and not touched for the expiry
/// time expires: a wanted one is dropped, an expiring one starts freeing
/// (stamped with the render frame), and a freeing one is released five
/// frames later. Then, if the stream manager is idle, the wanted slot with
/// the highest priority gets a page allocated and its read queued.
pub fn world_cache_update_hull_lru(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xb0);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp).off(0x2a74).lw(c);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        c.g[S2][0] = 0;
        frame.save(c, S6, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xa0);
        c.g[S6][0] = u64::MAX;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 20);
        let wc = r32(c, S3);
        c.g[V1][0] = Addr(wc).off(WC_SLOT_COUNT).lw(c);
        // now = the game clock's frame counter.
        c.g[FP][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 5;
            c.g[S7][0] = 1;
            c.g[S5][0] = HULL_FREEING;
            c.g[S4][0] = sx(wc + WC_SLOTS + SLOT_STAMP);
            c.g[V0][0] = SLOT_SIZE as u64;
            loop {
                age_slot(c, wc)?;
                // Next slot.
                c.cycles += 4;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                let more = c.g[S2][0] < c.g[V0][0];
                c.g[V0][0] = SLOT_SIZE as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_86d0);
                }
            }
        }

        // Load the best wanted slot.
        c.cycles += 2;
        let best = c.g[S6][0];
        frame.restore(c, S0, 0x90);
        if (best as i64) >= 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(wc).off(WC_BUSY).lw(c);
            c.g[S4][0] = sx(wc + WC_STREAM_MAN);
            if c.g[V0][0] == 0 {
                c.cycles += 27;
                c.g[A1][0] = best;
                c.g[A0][0] = sx(wc + WC_STREAMER);
                Callee { addr: 0x003a_9a98, f: ee::s13::f_003a9a98 }.call(c, 0x003a_8818)?;
                c.g[S0][0] = SLOT_SIZE as u64;
                c.g[S1][0] = sx(wc + WC_SLOTS + SLOT_PAGE);
                c.mult(0, S0, S6, S0);
                c.g[S2][0] = c.g[V0][0];
                // new (0x494e88, 0x20000000, 0) cHullPage(hull)
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(0x0049_4e88);
                c.g[A0][0] = 0x10;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                call_ptr(c, 0x0031_7d70, 0x003a_8844)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                Callee { addr: 0x003a_7bd8, f: ee::s13::f_003a7bd8 }.call(c, 0x003a_8850)?;
                let page = c.g[V0][0];
                Addr(r32(c, S1)).sw(c, page);
                // Stream manager: queue the hull's read (offset, size).
                let hull = r32(c, S2);
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S6][0];
                c.g[A3][0] = Addr(hull).off(4).lw(c);
                c.g[A2][0] = Addr(hull).lhu(c);
                Callee { addr: 0x003a_6f88, f: ee::s13::f_003a6f88 }.call(c, 0x003a_8868)?;
                c.g[S0][0] = sx(wc.wrapping_add(r32(c, S0)));
                c.g[V0][0] = HULL_LOADED;
                Addr(r32(c, S0)).off(WC_SLOTS + SLOT_STATE).sw(c, HULL_LOADED);
                frame.restore(c, S0, 0x90);
            }
        }

        c.cycles += 12;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    });

    /// One slot (`s2` = index): ends with `v0` = the slot count.
    fn age_slot(c: &mut Cpu, wc: u32) -> crate::call::Flow {
        c.cycles += 6;
        c.g[V1][0] = sx(wc + WC_SLOTS);
        c.mult(0, S0, S2, V0);
        let off = r32(c, S0);
        c.g[S1][0] = sx(r32(c, V1).wrapping_add(off));
        let state_at = r32(c, S1);
        c.g[A1][0] = Addr(state_at).lw(c);
        c.g[V1][0] = sx(wc.wrapping_add(off));
        let slot = r32(c, V1) + WC_SLOTS;
        let state = c.g[A1][0];
        if state == 0 {
            return count_and_next(c, wc, 1);
        }
        // Expired: unlocked and untouched for the expiry time.
        c.cycles += 3;
        c.g[V0][0] = Addr(slot).off(SLOT_LOCKED).lw(c);
        c.g[A2][0] = 0;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = c.g[V1][0];
            c.g[A0][0] = Addr(wc).off(WC_EXPIRY).lw(c);
            c.g[V1][0] = Addr(slot).off(SLOT_TIME).lw(c);
            c.g[A2][0] = c.g[S7][0];
            c.g[V1][0] = sx(r32(c, FP).wrapping_sub(r32(c, V1)));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[A2][0] = 0;
            }
        }
        let expired = c.g[A2][0] != 0;
        c.cycles += 3;
        c.g[V0][0] = ((state as i64) < 3) as u64;
        if state == HULL_LOADED {
            return count_and_next(c, wc, 1);
        }
        c.cycles += 2;
        let below3 = c.g[V0][0] != 0;
        c.g[V0][0] = 3;
        if below3 {
            c.cycles += 2;
            if state != HULL_WANTED {
                return count_and_next(c, wc, 2);
            }
            // Wanted: dropped when expired, else a candidate.
            c.cycles += 2;
            if expired {
                Addr(state_at).sw(c, 0);
                return count_and_next(c, wc, 1);
            }
            c.cycles += 6;
            c.g[V0][0] = sx(wc.wrapping_add(off));
            c.lwc1(0, slot + SLOT_PRIORITY);
            c.c_lt_s(20, 0);
            if !c.fcc() {
                return count_and_next(c, wc, 0);
            }
            c.cycles += 3;
            c.mov_s(20, 0);
            c.g[S6][0] = c.g[S2][0];
            return count_and_next(c, wc, 1);
        }
        c.cycles += 2;
        if state == HULL_EXPIRING {
            c.cycles += 2;
            if !expired {
                return count_and_next(c, wc, 0);
            }
            // Start freeing: stamp with the render frame.
            c.cycles += 10;
            Addr(state_at).sw(c, HULL_FREEING);
            let gp = c.g[GP][0] as u32;
            c.g[A1][0] = Addr(gp).off(LENS_CAMERA_GP).lw(c);
            frame_number(c, 0x003a_879c)?;
            c.g[V1][0] = sx(r32(c, S4).wrapping_add(off));
            let now = c.g[V0][0];
            Addr(r32(c, V1)).sw(c, now);
            return count_and_next(c, wc, 1);
        }
        c.cycles += 2;
        let freeing = state == HULL_FREEING;
        let gp = c.g[GP][0] as u32;
        c.g[A1][0] = Addr(gp).off(LENS_CAMERA_GP).lw(c);
        if !freeing {
            c.cycles += 2;
            return count_and_next(c, wc, 0);
        }
        // Freeing: release the page five frames after the stamp.
        c.cycles += 11;
        frame_number(c, 0x003a_87bc)?;
        c.g[V1][0] = sx(r32(c, S4).wrapping_add(off));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 5) as u64;
        if c.g[V0][0] != 0 {
            return count_and_next(c, wc, 0);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee { addr: 0x003a_8528, f: ee::s13::f_003a8528 }.call(c, 0x003a_87e0)?;
        Addr(r32(c, S1)).sw(c, 0);
        count_and_next(c, wc, 1)
    }

    /// `camera->GetFrame()` (`a1` = the camera).
    fn frame_number(c: &mut Cpu, ret: u32) -> crate::call::Flow {
        let cam = c.g[A1][0];
        c.g[V1][0] = Addr(cam as u32).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_FRAME);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, ret)
    }

    /// The loop's join (`extra` cycles first): `v0` = the slot count.
    fn count_and_next(c: &mut Cpu, wc: u32, extra: u64) -> crate::call::Flow {
        c.cycles += extra;
        c.g[V0][0] = Addr(wc).off(WC_SLOT_COUNT).lw(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// AppModule_RenderFade (0x231d60)
// ---------------------------------------------------------------------------

/// The 2D render context (`*(gp - 0x854)`): its render-state stack pointer
/// (0x14-byte entries) and its vtable (+0x10d8).
const R2D_CTX_GP: u32 = 0x854;
const R2D_STATE_TOP: u32 = 0xe84;
const R2D_VT: u32 = 0x10d8;
/// Context slots: `SetViewport(0, -1, -1, 1, 1, 0, 1, 0)` and
/// `SetTexture(tex)`.
const VT_SET_VIEWPORT: u32 = 0xd0;
const VT_SET_TEXTURE: u32 = 0x110;
/// The default render state and the "no texture" object.
const DEFAULT_STATE: u32 = 0x0050_1420;
const NO_TEXTURE: u32 = 0x004f_f1a0;
/// The fade timer (seconds left).
const FADE_TIME: u32 = 0x4;

/// `bool AppModule_RenderFade(AppModule* m)` (0x231d60): while the module's
/// fade timer runs, push a copy of the 2D render state, reset it to the
/// default with a full-screen viewport, no texture and alpha blending, draw
/// a full-screen black rectangle with alpha `min(5 t, 1)` and pop the
/// state. Returns whether the rectangle was fully opaque.
pub fn app_module_render_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save_fpr(c, 21, 0x58);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 23, 0x68);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 20, 0x50);
        let module = r32(c, S0);
        let gp = c.g[GP][0] as u32;
        c.mtc1(0, 21);
        c.lwc1(0, module + FADE_TIME);
        c.c_lt_s(21, 0);
        let fading = c.fcc();
        c.g[A0][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        if !fading {
            c.cycles += 1;
            c.g[V0][0] = 0;
            return epilogue(c, frame);
        }

        // Push the render state (copy the top 0x14-byte entry up).
        c.cycles += 88;
        c.g[T0][0] = 0x50_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        c.g[A1][0] = 0;
        let ctx = r32(c, A0);
        c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        let top = r32(c, V0);
        c.mov_s(16, 21);
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 22);
        c.mov_s(14, 20);
        c.ldl(V1, top + 0x7);
        c.ldr(V1, top);
        c.ldl(A2, top + 0xf);
        c.ldr(A2, top + 0x8);
        c.g[A3][0] = Addr(top).off(0x10).lw(c);
        c.sdl(V1, top + 0x1b);
        c.sdr(V1, top + 0x14);
        c.sdl(A2, top + 0x23);
        c.sdr(A2, top + 0x1c);
        let w = c.g[A3][0];
        Addr(top).off(0x24).sw(c, w);
        c.mov_s(15, 20);
        c.g[A2][0] = 0;
        c.mov_s(13, 22);
        c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        c.mov_s(17, 20);
        c.g[A3][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        c.mov_s(12, 22);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
        c.mov_s(23, 21);
        let new_top = c.g[V1][0];
        Addr(ctx).off(R2D_STATE_TOP).sw(c, new_top);

        // *top = the default state.
        let ctx = r32(c, A3);
        c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        let top = r32(c, V0);
        c.g[T1][0] = sx(DEFAULT_STATE);
        c.ldl(V1, DEFAULT_STATE + 0x7);
        c.ldr(V1, DEFAULT_STATE);
        c.ldl(A0, DEFAULT_STATE + 0xf);
        c.ldr(A0, DEFAULT_STATE + 0x8);
        c.g[A3][0] = Addr(DEFAULT_STATE).off(0x10).lw(c);
        c.sdl(V1, top + 0x7);
        c.sdr(V1, top);
        c.sdl(A0, top + 0xf);
        c.sdr(A0, top + 0x8);
        let w = c.g[A3][0];
        Addr(top).off(0x10).sw(c, w);

        // ctx->SetViewport(0, -1, -1, 1, 1, 0, 1, 0); ctx->SetTexture(none).
        c.g[A3][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        let obj = c.g[A3][0];
        c.g[V0][0] = Addr(obj as u32).off(R2D_VT).lw(c);
        let vt = c.g[V0][0];
        vcall(c, obj, vt, VT_SET_VIEWPORT, V1, 0x0023_1e54)?;
        let gp = c.g[GP][0] as u32;
        c.g[A2][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        c.g[A1][0] = sx(NO_TEXTURE);
        let obj = c.g[A2][0];
        c.g[V0][0] = Addr(obj as u32).off(R2D_VT).lw(c);
        let vt = c.g[V0][0];
        vcall(c, obj, vt, VT_SET_TEXTURE, V1, 0x0023_1e74)?;

        // Blend bits of the new state: alpha blend on, test/depth bits off.
        let gp = c.g[GP][0] as u32;
        c.g[A3][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        let ctx = r32(c, A3);
        c.g[V1][0] = 0xffff_ffff_fe7f_ffff;
        c.g[T1][0] = 0x100_0000;
        c.g[T0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        c.g[A1][0] = 0xffff_ffff_ffcf_ffff;
        c.g[A2][0] = 0xffff_ffff_fff0_0fff;
        let top = r32(c, T0);
        c.g[V0][0] = Addr(top).off(0x8).lw(c);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 1);
        c.g[V0][0] |= 0x3e0;
        let v = c.g[V0][0];
        Addr(top).off(0x8).sw(c, v);
        for (reg, mask, or) in [(A0, V1, true), (V1, A1, false), (A0, A2, false)] {
            c.g[reg][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let top = r32(c, reg);
            c.g[V0][0] = Addr(top).off(0x4).lw(c);
            c.g[V0][0] &= c.g[mask][0];
            if or {
                c.g[V0][0] |= c.g[T1][0];
            }
            let v = c.g[V0][0];
            Addr(top).off(0x4).sw(c, v);
        }

        // alpha = min(5 t, 1) (0 when negative).
        c.lwc1(0, module + FADE_TIME);
        c.mul_s(0, 0, 1);
        c.c_le_s(21, 0);
        if c.fcc() {
            c.min_s(23, 0, 20);
        }

        // Render2D_DrawFilledRect(r2d, {-1,-1}, {1,1}, {alpha,0,0,0}, 0).
        c.cycles += 21;
        let gp = c.g[GP][0] as u32;
        let sp0 = sp(c);
        c.g[A0][0] = Addr(gp.wrapping_sub(0x844)).lw(c);
        c.mov_s(12, 21);
        c.swc1(22, sp0 + 0x4);
        c.g[A1][0] = sx(sp0);
        c.swc1(22, sp0);
        c.g[A2][0] = sx(sp0 + 0x10);
        c.swc1(20, sp0 + 0x10);
        c.g[A3][0] = sx(sp0 + 0x20);
        c.swc1(20, sp0 + 0x14);
        c.swc1(23, sp0 + 0x20);
        c.swc1(21, sp0 + 0x24);
        c.swc1(21, sp0 + 0x28);
        c.swc1(21, sp0 + 0x2c);
        call_ptr(c, 0x0038_ec40, 0x0023_1f30)?;

        // Pop the state.
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        let ctx = r32(c, V1);
        c.c_le_s(20, 23);
        c.g[A0][0] = 1;
        c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        let opaque = c.fcc();
        let top = c.g[V0][0];
        Addr(ctx).off(R2D_STATE_TOP).sw(c, top);
        if !opaque {
            c.cycles += 1;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 23, 0x68);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// cWorldSound_RefreshNearbyEmitters (0x2b75d0)
// ---------------------------------------------------------------------------

/// Nearby-emitter list: count at +0 (entries follow), the position it was
/// built for (+0x110) and its refresh distance flag (+0x11c, 0 = always).
const NEAR_COUNT: u32 = 0x0;
const NEAR_POS: u32 = 0x110;
const NEAR_REFRESH: u32 = 0x11c;
/// The sound module's emitter octree: eight 0x14-byte root cells, each
/// with its subtree at +0x10.
const SOUND_TREE: u32 = 0x20;
const CELL_SIZE: u32 = 0x14;
const CELL_SUBTREE: u32 = 0x10;

/// `void cWorldSound_RefreshNearbyEmitters(NearList* list, const Vec4* pos)`
/// (0x2b75d0): rebuild the list of emitters near `pos` when the list wants
/// a refresh every time or the listener moved more than 10 units since the
/// last build: clear it and query each of the eight root cells of the
/// emitter tree that has a subtree, then remember `pos`.
pub fn world_sound_refresh_nearby_emitters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        c.mtc1(0, 1);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save_ra(c, 0x30);
        let list = r32(c, S3);
        c.lwc1(0, list + NEAR_REFRESH);
        c.c_eq_s(0, 1);
        let always = c.fcc();
        c.g[S4][0] = c.g[A1][0];
        let pos = r32(c, S4);
        let sp0 = sp(c);
        if !always {
            // Moved more than 10 units?
            c.cycles += 22;
            vsub(c, pos, list + NEAR_POS, sp0 + 0x10);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 1);
            qcopy(c, V0, sp0 + 0x10, sp0);
            vlen(c, V1, sp0);
            c.mtc1(V1, 0);
            c.c_lt_s(1, 0);
            let moved = c.fcc();
            frame.restore(c, S0, 0x80);
            if !moved {
                return epilogue(c, frame);
            }
        }

        // Clear the list and query the tree around pos.
        c.cycles += 9;
        Addr(list).off(NEAR_COUNT).sw(c, 0);
        call_ptr(c, 0x0028_b1c8, 0x002b_7668)?;
        let module = r32(c, V0);
        c.g[S0][0] = Addr(module).off(SOUND_TREE).lw(c);
        let tree = r32(c, S0);
        let sp0 = sp(c);
        // Query context at sp+0x20: {list, pos}.
        c.g[S1][0] = sx(sp0 + 0x20);
        Addr(sp0).off(0x20).sw(c, sx(list));
        Addr(sp0).off(0x24).sw(c, sx(pos));
        c.g[T0][0] = Addr(tree).off(CELL_SUBTREE).lw(c);
        c.g[S2][0] = sx(sp0 + 0x24);
        for k in 0..8u32 {
            let cell = tree + CELL_SIZE * k;
            if k > 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(cell).off(CELL_SUBTREE).lw(c);
                c.g[T1][0] = sx(cell);
                if c.g[V0][0] == 0 {
                    continue;
                }
                c.cycles += 6;
                c.g[T0][0] = Addr(cell).off(CELL_SUBTREE).lw(c);
            } else {
                if c.g[T0][0] == 0 {
                    continue;
                }
                c.cycles += 6;
                c.g[T1][0] = c.g[S0][0];
            }
            // cWorldSound_QueryTreeNode(&visited[k], &ctx, &ctx, &ctx.pos,
            // subtree, cell)
            let sp0 = sp(c);
            c.g[A0][0] = sx(sp0 + 0x28 + k);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S2][0];
            Callee { addr: 0x002b_83f8, f: ee::s08::f_002b83f8 }.call(c, 0x002b_769c + 0x24 * k)?;
        }
        c.cycles += 3;
        qcopy(c, V0, pos, list + NEAR_POS);
        frame.restore(c, S0, 0x80);
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 7;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Render-state stack helpers (2D render context, `*(gp - 0x854)`)
// ---------------------------------------------------------------------------

/// The render context (`lw gp-0x854`).
fn r2d_ctx(c: &mut Cpu) -> u64 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c)
}

/// `top[1] = top[0]` (0x14 bytes: two unaligned doublewords through `d0`,
/// `d1`, then a word through `w`); `top` is the current state.
fn r2d_copy_up(c: &mut Cpu, top: u32, d0: usize, d1: usize, w: usize) {
    c.ldl(d0, top + 0x7);
    c.ldr(d0, top);
    c.ldl(d1, top + 0xf);
    c.ldr(d1, top + 0x8);
    c.g[w][0] = Addr(top).off(0x10).lw(c);
    c.sdl(d0, top + 0x1b);
    c.sdr(d0, top + 0x14);
    c.sdl(d1, top + 0x23);
    c.sdr(d1, top + 0x1c);
    let v = c.g[w][0];
    Addr(top).off(0x24).sw(c, v);
}

/// `reg = ctx->top; v0 = reg->word[off]; v0 = (v0 & and) | or; store`.
fn r2d_bits(c: &mut Cpu, ctx: u32, reg: usize, off: u32, and: u64, or: Option<u64>) {
    c.g[reg][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
    let top = r32(c, reg);
    c.g[V0][0] = Addr(top).off(off).lw(c);
    c.g[V0][0] &= and;
    if let Some(or) = or {
        c.g[V0][0] |= or;
    }
    let v = c.g[V0][0];
    Addr(top).off(off).sw(c, v);
}

// ---------------------------------------------------------------------------
// cPS2FogParticleMan_Render (0x2dbf98)
// ---------------------------------------------------------------------------

/// The fog particle manager: queued particle count (+0), vtable (+4; slot
/// 0x18 draws them).
const FOG_COUNT: u32 = 0x0;
const FOG_VT: u32 = 0x4;
const VT_FOG_DRAW: u32 = 0x18;
/// The sort buffer (8-byte {particle, depth} pairs) and its comparator
/// (`cPS2FogParticleMan_CompareDesc`).
const FOG_SORT_BUF: u32 = 0x0053_7ac0;
const FOG_COMPARE: u32 = 0x002d_c168;

/// `void cPS2FogParticleMan_Render(cPS2FogParticleMan* m)` (0x2dbf98): if
/// particles were queued, sort them back to front (`qsort`), push the 2D
/// render state and set it up for fog sprites (alpha blending, depth test
/// and write settings, the context's fog colour index, full mask), let the
/// manager draw them, pop the state and empty the queue.
pub fn ps2_fog_particle_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let man = r32(c, S3);
        c.g[A1][0] = Addr(man).off(FOG_COUNT).lw(c);
        c.g[A3][0] = 0x2e_0000;
        if c.g[A1][0] != 0 {
            c.cycles += 98;
            // qsort(buf, count, 8, CompareDesc)
            c.g[A0][0] = sx(FOG_SORT_BUF);
            c.g[A3][0] = sx(FOG_COMPARE);
            c.g[A2][0] = 8;
            call_ptr(c, 0x0041_8ef8, 0x002d_bfd4)?;

            // Push the render state.
            c.g[V1][0] = r2d_ctx(c);
            let ctx = r32(c, V1);
            c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
            c.g[T6][0] = 0xffff_ffff_ffff_fffc;
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[A3][0] = 0xffff_ffff_e000_03ff;
            c.g[T7][0] = u64::MAX;
            let top = r32(c, V0);
            r2d_copy_up(c, top, A0, A1, T0);
            c.g[A2][0] = 0xf_fc00;
            c.g[S0][0] = 0xffff_ffff_ffff_ff83;
            c.g[T1][0] = 0xffff_ffff_ffbf_ffff;
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[A1][0] = r2d_ctx(c);
            c.g[S1][0] = 0x40_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
            c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
            let v = c.g[V0][0];
            Addr(ctx).off(R2D_STATE_TOP).sw(c, v);
            c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
            c.g[S2][0] = 0x30_0000;

            // Fog sprite state.
            let ctx = r32(c, A1);
            c.g[A0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[T4][0] = Addr(ctx).off(0xf60).lw(c);
            c.g[T3][0] = 0xffff_ffff_fff0_0fff;
            let top = r32(c, A0);
            c.g[V0][0] = Addr(top).off(0x8).lw(c);
            c.g[V0][0] &= c.g[T5][0];
            c.g[V0][0] |= 0xe0;
            let v = c.g[V0][0];
            Addr(top).off(0x8).sw(c, v);
            let (t6, a3, a2) = (c.g[T6][0], c.g[A3][0], c.g[A2][0]);
            r2d_bits(c, ctx, V1, 0x4, t6, Some(0x2));
            r2d_bits(c, ctx, A0, 0x8, a3, Some(a2));
            c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let fog = c.g[T4][0];
            Addr(r32(c, V1)).off(0x10).sh(c, fog);
            c.g[A1][0] = r2d_ctx(c);
            let ctx = r32(c, A1);
            c.g[A0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let top = r32(c, A0);
            c.g[V0][0] = Addr(top).lw(c);
            c.g[V0][0] |= 0xc;
            let v = c.g[V0][0];
            Addr(top).sw(c, v);
            c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let mask = c.g[T7][0];
            Addr(r32(c, V1)).off(0x12).sh(c, mask);
            c.g[A2][0] = r2d_ctx(c);
            let ctx = r32(c, A2);
            let (s0, t1, s1, t0, t2, s2, t3) =
                (c.g[S0][0], c.g[T1][0], c.g[S1][0], c.g[T0][0], c.g[T2][0], c.g[S2][0], c.g[T3][0]);
            r2d_bits(c, ctx, V1, 0x4, s0, Some(0x14));
            r2d_bits(c, ctx, A0, 0x4, t1, Some(s1));
            r2d_bits(c, ctx, V1, 0x4, t0, None);
            r2d_bits(c, ctx, A1, 0x4, t2, Some(s2));
            r2d_bits(c, ctx, A0, 0x4, t3, None);

            // m->Draw()
            c.g[V1][0] = Addr(man).off(FOG_VT).lw(c);
            let (obj, vt) = (c.g[S3][0], c.g[V1][0]);
            vcall(c, obj, vt, VT_FOG_DRAW, V0, 0x002d_c134)?;

            // Pop the state; empty the queue.
            c.g[V1][0] = r2d_ctx(c);
            let ctx = r32(c, V1);
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let v = c.g[V0][0];
            Addr(ctx).off(R2D_STATE_TOP).sw(c, v);
            Addr(man).off(FOG_COUNT).sw(c, 0);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2VisualEffectsMan_Render (0x2f0cd0)
// ---------------------------------------------------------------------------

/// The visual effects manager's effect systems (17 pointers from +0).
const VFX_SYSTEMS: u32 = 17;
/// Effect system vtable slot: render.
const VT_VFX_RENDER: u32 = 0x18;

/// `void PS2VisualEffectsMan_Render(PS2VisualEffectsMan* m)` (0x2f0cd0):
/// render each of the 17 effect systems in order (`sys->Render()`).
pub fn ps2_visual_effects_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 110;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let man = r32(c, S0);
        for k in 0..VFX_SYSTEMS {
            c.g[A1][0] = Addr(man).off(4 * k).lw(c);
            let sys = c.g[A1][0];
            c.g[V0][0] = Addr(sys as u32).lw(c);
            let vt = c.g[V0][0];
            vcall(c, sys, vt, VT_VFX_RENDER, V1, 0x002f_0cf8 + 0x18 * k)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Mat4_Determinant (0x31b178)
// ---------------------------------------------------------------------------

/// `float Mat4_Determinant(const Mat4* m)` (0x31b178): Laplace expansion
/// along the first row, `m00 D0 - m01 D1 + m02 D2 - m03 D3`, with the four
/// 3x3 minors of rows 1-3 from `Math_Det3x3(f12..f19, [sp])`.
pub fn mat4_determinant(c: &mut Cpu) {
    const DET3: Callee = Callee { addr: 0x0031_b098, f: ee::s10::f_0031b098 };
    guest(c, |c| {
        c.cycles += 101;
        let frame = Frame::open(c, 0xa0);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x40);
        for (i, f) in (21..=31).rev().enumerate() {
            frame.save_fpr(c, f, 0x98 - 8 * i as u32);
        }
        let m = r32(c, A0);
        let sp0 = sp(c);
        // Row 0 in f20, f28, f29, f30; rows 1-3 in f24/f25/f22/f23/f21/f26/
        // f27/f31 and spilled columns at sp+0x10..0x1c.
        c.lwc1(20, m);
        c.lwc1(0, m + 0x14);
        c.lwc1(27, m + 0x3c);
        c.swc1(0, sp0 + 0x10);
        c.mov_s(12, 0);
        c.lwc1(1, m + 0x18);
        c.swc1(1, sp0 + 0x18);
        c.mov_s(15, 1);
        c.lwc1(0, m + 0x24);
        c.lwc1(25, m + 0x1c);
        c.swc1(0, sp0 + 0x14);
        c.mov_s(13, 0);
        c.mov_s(18, 25);
        c.lwc1(1, m + 0x28);
        c.swc1(1, sp0 + 0x1c);
        c.mov_s(16, 1);
        c.lwc1(21, m + 0x2c);
        c.lwc1(31, m + 0x34);
        c.lwc1(26, m + 0x38);
        c.mov_s(19, 21);
        c.lwc1(24, m + 0x10);
        c.mov_s(14, 31);
        c.lwc1(23, m + 0x20);
        c.mov_s(17, 26);
        c.lwc1(22, m + 0x30);
        c.swc1(27, sp0);
        c.lwc1(28, m + 0x4);
        c.lwc1(29, m + 0x8);
        c.lwc1(30, m + 0xc);
        // D0: columns 1, 2, 3.
        DET3.call(c, 0x0031_b228)?;
        // D1: columns 0, 2, 3.
        let sp0 = sp(c);
        c.lwc1(15, sp0 + 0x18);
        c.mov_s(12, 24);
        c.lwc1(16, sp0 + 0x1c);
        c.mov_s(13, 23);
        c.swc1(0, sp0 + 0x20);
        c.mov_s(14, 22);
        c.swc1(27, sp0);
        c.mov_s(17, 26);
        c.mov_s(18, 25);
        c.mov_s(19, 21);
        DET3.call(c, 0x0031_b254)?;
        // D2: columns 0, 1, 3.
        let sp0 = sp(c);
        c.lwc1(15, sp0 + 0x10);
        c.mov_s(18, 25);
        c.lwc1(16, sp0 + 0x14);
        c.mov_s(19, 21);
        c.swc1(0, sp0 + 0x24);
        c.mov_s(12, 24);
        c.swc1(27, sp0);
        c.mov_s(13, 23);
        c.mov_s(14, 22);
        c.mov_s(17, 31);
        DET3.call(c, 0x0031_b280)?;
        // D3: columns 0, 1, 2.
        let sp0 = sp(c);
        c.lwc1(15, sp0 + 0x10);
        c.mov_s(21, 0);
        c.lwc1(16, sp0 + 0x14);
        c.mov_s(12, 24);
        c.lwc1(18, sp0 + 0x18);
        c.mov_s(13, 23);
        c.lwc1(19, sp0 + 0x1c);
        c.mov_s(14, 22);
        c.swc1(26, sp0);
        c.mov_s(17, 31);
        DET3.call(c, 0x0031_b2ac)?;
        // m00 D0 - m01 D1 + m02 D2 - m03 D3 (restoring f20-f31 on the way).
        let sp0 = sp(c);
        c.lwc1(1, sp0 + 0x20);
        c.mul_s(29, 29, 21);
        c.mul_s(30, 30, 0);
        frame.restore_ra(c, 0x30);
        c.mul_s(20, 20, 1);
        c.lwc1(31, sp0 + 0x98);
        c.lwc1(1, sp0 + 0x24);
        c.lwc1(27, sp0 + 0x78);
        c.mul_s(28, 28, 1);
        c.lwc1(26, sp0 + 0x70);
        c.lwc1(25, sp0 + 0x68);
        c.lwc1(24, sp0 + 0x60);
        c.sub_s(20, 20, 28);
        c.lwc1(23, sp0 + 0x58);
        c.lwc1(28, sp0 + 0x80);
        c.lwc1(22, sp0 + 0x50);
        c.add_s(20, 20, 29);
        c.lwc1(21, sp0 + 0x48);
        c.lwc1(29, sp0 + 0x88);
        c.sub_s(0, 20, 30);
        c.lwc1(30, sp0 + 0x90);
        c.lwc1(20, sp0 + 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{put_words, Rng, DATA, SP as STACK_TOP};

    /// A real leaf returning 0 (`RiderIface_HasVisited`: `v0 = 0; jr ra`),
    /// for vtable slots whose zero answer the fakes never give.
    const RETURN_ZERO: u32 = 0x0014_06c0;

    /// Write vtable slot `off` (`{delta, func}`) at `vt`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// Occluded and visible sources, behind and in front of the near plane,
    /// depths that clamp the sample rectangle both ways, every source kind,
    /// rectangles inside and outside the view; fake projector calls (some
    /// unwind).
    #[test]
    fn lens_fx_project_source_matches() {
        covered(ee::s09::f_002e2b00__generated, lens_fx_project_source, 300, &|c, seed| {
            c.pc = 0x002e_2b00;
            let mut r = machine(c, seed, 0x2e2b00, 3);
            let gp = c.g[GP][0] as u32;
            let lens = DATA;
            let src = DATA + 0x200;
            let view = DATA + 0x400;
            let cam = DATA + 0x1000;
            let vt = DATA + 0x3000;
            c.bus.write32(gp + LENS_CAMERA_GP, cam);
            c.bus.write32(cam + CAM_PROJECTOR_VT, vt);
            let occluded = if seed % 5 == 0 { FAKE } else { RETURN_ZERO };
            slot(c, vt, VT_OCCLUDED, (r.next_u32() as i16) & !3, occluded);
            slot(c, vt, VT_PROJECT, (r.next_u32() as i16) & !3, FAKE + 0x10);
            // Depth scale, near limit, x-offset scale.
            put_words(c, gp.wrapping_sub(0x3a40), &[1.0f32.to_bits(), 0.001f32.to_bits(), r.f32(2.0)]);
            c.g[A0][0] = sx(lens);
            c.g[A1][0] = sx(view);
            c.bus.write32(lens + LENS_SOURCE, src);
            for i in 0..3 {
                c.bus.write32(src + SRC_POS + 4 * i, r.f32(500.0));
            }
            let kind = [0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x70][(seed / 5 % 7) as usize];
            c.bus.write32(src + SRC_KIND, (r.next_u32() & !0x70) | kind);
            // The view rectangle (x, y, w, h), then +0x18/+0x28/+0x38.
            put_words(c, view, &[r.f32(50.0), r.f32(50.0), r.f32(640.0), r.f32(480.0)]);
            for off in [0x18u32, 0x28, 0x38] {
                c.bus.write32(view + off, if seed % 7 == 0 { r.edge_f32() } else { r.f32(5.0) });
            }
            // What the fake projector "returns" at sp+0x10: screen x, y, depth.
            let frame = STACK_TOP - 0xc0;
            let depth = match seed % 6 {
                0 => r.f32(0.001),
                1 => 0.0011f32.to_bits(),
                2 => 0.05f32.to_bits(),
                3 => 0.002f32.to_bits(),
                4 => r.edge_f32(),
                _ => r.f32(0.03),
            };
            put_words(c, frame + 0x10, &[r.f32(700.0), r.f32(500.0), depth]);
            c.bus.write32(frame + 0x18, depth);
            put_words(c, frame + 0x48, &[r.f32(300.0)]);
        });
    }
    /// Every modifier present or absent, id and extra lists of 0-4 nodes;
    /// all virtual calls go to fakes (some unwind), the two direct callees
    /// (UV-scroll and rail save) run for real into the fake stream.
    #[test]
    fn modifier_block_save_matches() {
        covered(ee::s11::f_00353448__generated, modifier_block_save, 300, &|c, seed| {
            c.pc = 0x0035_3448;
            let mut r = machine(c, seed, 0x353448, 14);
            let block = DATA;
            let stream = DATA + 0x800;
            let vt = |i: u32| DATA + 0x2000 + 0x400 * i;
            let mut fake = 0;
            let mut fslot = |c: &mut Cpu, r: &mut Rng, vt: u32, off: u32| {
                slot(c, vt, off, (r.next_u32() as i16) & !3, FAKE + 0x10 * fake);
                fake += 1;
            };
            // Stream: Write (0x8) and the rail modifier's 0x28.
            c.bus.write32(stream, vt(0));
            fslot(c, &mut r, vt(0), 0x8);
            fslot(c, &mut r, vt(0), 0x28);
            let bits = r.next_u32();
            let present = |i: u32| seed % 11 == 0 || (bits >> i) & 1 != 0;
            // Primary (vtable at +0: id 0xd0, save 0xd8).
            let primary = if present(0) { DATA + 0x100 } else { 0 };
            c.bus.write32(DATA + 0x100, vt(1));
            fslot(c, &mut r, vt(1), 0xd0);
            fslot(c, &mut r, vt(1), 0xd8);
            // UV scroll / rail (direct callees; the rail one reads +0x18).
            let uv = if present(1) { DATA + 0x300 } else { 0 };
            let rail = if present(2) { DATA + 0x380 } else { 0 };
            c.bus.write32(DATA + 0x380 + 0x18, r.next_u32());
            // Secondary (vtable at +4: id 0x28, save 0x30).
            let secondary = if present(3) { DATA + 0x200 } else { 0 };
            c.bus.write32(DATA + 0x204, vt(2));
            fslot(c, &mut r, vt(2), 0x28);
            fslot(c, &mut r, vt(2), 0x30);
            // Lists: next at +0, vtable at +8.
            fslot(c, &mut r, vt(3), 0x10);
            fslot(c, &mut r, vt(3), 0x20);
            fslot(c, &mut r, vt(4), 0x10);
            let mut list = |c: &mut Cpu, base: u32, n: u32, vtab: u32| -> u32 {
                for i in 0..n {
                    let node = base + 0x20 * i;
                    c.bus.write32(node, if i + 1 < n { node + 0x20 } else { 0 });
                    c.bus.write32(node + 8, vtab);
                }
                if n == 0 { 0 } else { base }
            };
            let ids = list(c, DATA + 0x400, (seed / 3) % 5, vt(3));
            let extra = list(c, DATA + 0x600, (seed / 7) % 5, vt(4));
            put_words(c, block, &[primary, uv, rail, secondary, ids]);
            c.bus.write32(block + MB_EXTRA_LIST, extra);
            c.g[A0][0] = sx(block);
            c.g[A1][0] = sx(stream);
        });
    }
    /// Trail lengths -1..5 (no points, one point whose t is 0/0, several),
    /// head slots 0-3, random vectors; Noise_Fractal1D runs for real.
    #[test]
    fn board_wake_fx_build_trail_matches() {
        crate::difftest::check(ee::s09::f_002ddd30__generated, board_wake_fx_build_trail, 300, &|c, seed| {
            let mut r = Rng(0x2ddd_3000 ^ (seed as u64) << 16);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            let gp = crate::difftest::GAME_GP;
            let wake = DATA;
            let rider = DATA + 0x1000;
            let pos = DATA + 0x800;
            let n = [-1i32, 0, 1, 2, 3, 5][(seed % 6) as usize];
            let head = (seed / 6) % 4;
            let v = |r: &mut Rng| [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(2.0)];
            c.bus.write32(wake + WAKE_RIDER, rider);
            c.bus.write32(wake + WAKE_HEAD, head);
            c.bus.write32(wake + WAKE_ELEMS, DATA + 0x2000);
            for j in 0..6 {
                c.bus.write32(wake + WAKE_RINGS + 4 * j, DATA + 0x3000 + 0x400 * j);
                let seg = DATA + 0x3000 + 0x400 * j + SEG_SIZE * head;
                crate::difftest::fill(c, &mut r, seg, 12);
            }
            c.bus.write32(wake + WAKE_PHASE, r.f32(50.0));
            for off in [WAKE_SPREAD, WAKE_CURVE, WAKE_BACK] {
                let q = v(&mut r);
                crate::difftest::put_vec(c, wake + off, q);
            }
            c.bus.write32(wake + WAKE_WIDTH, r.f32(4.0));
            c.bus.write32(wake + WAKE_ALPHA, if seed % 9 == 0 { r.edge_f32() } else { r.f32(2.0) });
            c.bus.write32(wake + WAKE_POINTS, n as u32);
            let q = v(&mut r);
            crate::difftest::put_vec(c, rider + RIDER_VEL, q);
            let q = v(&mut r);
            crate::difftest::put_vec(c, rider + RIDER_UP, q);
            let q = v(&mut r);
            crate::difftest::put_vec(c, pos, q);
            let q = v(&mut r);
            crate::difftest::put_vec(c, 0x004f_f120, q);
            put_words(c, gp - 0x3be4, &[0.85f32.to_bits(), 0.9f32.to_bits()]);
            // Noise_Sample1D's table.
            for i in 0..0x180u32 {
                c.bus.write32(0x0044_5ab0 - 0x300 + 4 * i, r.f32(1.0));
            }
            c.g[A0][0] = sx(wake);
            c.g[A1][0] = sx(pos);
            c.f[12] = r.f32(3.0);
        });
    }
    /// Slots in every state (and an unknown one), locked or not, fresh or
    /// expired, freeing ones before and after their five frames (real pages
    /// allocated in the setup and freed by the real FreeHullPage), the best
    /// wanted slot loaded through the real allocator, page constructor and
    /// stream manager (idle, no file: the read is refused) or skipped when
    /// busy. The camera's frame counter is a fake (some calls unwind).
    #[test]
    fn world_cache_update_hull_lru_matches() {
        use crate::difftest::fakes::{LOG_COUNT, LOG_RESULTS};
        use crate::game::fe::feasyncfile::tests::{alloc_base_3, run_7};
        const NOW: u32 = 1000;
        const EXPIRY: u32 = 30;
        const FRAME: u32 = 0x500;
        covered(ee::s13::f_003a8668__generated, world_cache_update_hull_lru, 200, &|c, seed| {
            c.pc = 0x003a_8668;
            let mut r = alloc_base_3(c, seed, 0x3a8668, 4);
            let gp = c.g[GP][0] as u32;
            let clock = DATA + 0x100;
            let cam = DATA + 0x200;
            let vt = DATA + 0x300;
            let wc = DATA + 0x1000;
            let hulls = DATA + 0x8000;
            c.bus.write32(gp + 0x2a74, clock);
            c.bus.write32(clock + 0x1c, NOW);
            c.bus.write32(gp + LENS_CAMERA_GP, cam);
            c.bus.write32(cam + CAM_PROJECTOR_VT, vt);
            slot(c, vt, VT_FRAME, (r.next_u32() as i16) & !3, FAKE);
            let n = seed % 7;
            c.bus.write32(wc + WC_SLOT_COUNT, n);
            c.bus.write32(wc + WC_EXPIRY, EXPIRY);
            let busy = (seed % 5 == 0) as u32;
            c.bus.write32(wc + WC_BUSY, busy);
            // The stream manager's file handle: none (the read is refused).
            c.bus.write32(wc + WC_STREAM_MAN + 0x88, 0);
            c.bus.write32(wc + WC_STREAMER + 0x10, hulls);
            for i in 0..8 {
                let hull = hulls + 0x44 * i;
                c.bus.write16(hull, 1 + r.next_u32() as u16 % 4);
                c.bus.write32(hull + 4, r.next_u32());
            }
            for i in 0..n {
                let slot_at = wc + WC_SLOTS + SLOT_SIZE * i;
                let state = [0u32, 1, 1, 1, 2, 3, 3, 4, 4, 5, u32::MAX][(r.next_u32() % 11) as usize];
                c.bus.write32(slot_at + SLOT_STATE, state);
                c.bus.write32(slot_at + SLOT_LOCKED, (r.next_u32() % 3 == 0) as u32);
                let age = [0, EXPIRY - 1, EXPIRY, EXPIRY + 50][(r.next_u32() % 4) as usize];
                c.bus.write32(slot_at + SLOT_TIME, NOW - age);
                c.bus.write32(slot_at + SLOT_PRIORITY, if r.next_u32() % 5 == 0 { r.edge_f32() } else { r.f32(10.0) });
                c.bus.write32(slot_at + SLOT_STAMP, FRAME - [0, 4, 5, 9][(r.next_u32() % 4) as usize]);
                let page = if state == 4 {
                    // A real page (freed by FreeHullPage).
                    let page = run_7(c, 0x0031_7d70, ee::s10::f_00317d70, &[0x10, 0x0049_4e88, 0x2000_0000, 0]);
                    run_7(c, 0x003a_7bd8, ee::s13::f_003a7bd8, &[page, sx(hulls + 0x44 * i)]);
                    page as u32
                } else {
                    r.next_u32()
                };
                c.bus.write32(slot_at + SLOT_PAGE, page);
            }
            // The frame counter's answers (after any fakes the setup used).
            let n0 = c.bus.read32(LOG_COUNT);
            for k in n0..8 {
                c.bus.write32(LOG_RESULTS + 4 * k, FRAME);
            }
            c.g[A0][0] = sx(wc);
            c.g[SP][0] = sx(STACK_TOP);
            c.g[RA][0] = sx(crate::difftest::RA);
        });
    }
    /// Fade timer negative, zero, partial and saturating (alpha 1), random
    /// render-state stack contents; the context's virtual calls and the
    /// camera call inside the real Render2D_DrawFilledRect are fakes.
    #[test]
    fn app_module_render_fade_matches() {
        covered(ee::s06::f_00231d60__generated, app_module_render_fade, 300, &|c, seed| {
            c.pc = 0x0023_1d60;
            let mut r = machine(c, seed, 0x231d60, 4);
            let gp = c.g[GP][0] as u32;
            let module = DATA;
            let ctx = DATA + 0x1000;
            let stack = DATA + 0x2004;
            let r2d = DATA + 0x3000;
            let cam = DATA + 0x4000;
            let t = match seed % 6 {
                0 => r.f32(1.0) | 0x8000_0000,
                1 => 0,
                2 => 0.1f32.to_bits(),
                3 => 0.3f32.to_bits(),
                4 => r.edge_f32(),
                _ => r.f32(0.4),
            };
            c.bus.write32(module + FADE_TIME, t);
            c.bus.write32(gp.wrapping_sub(R2D_CTX_GP), ctx);
            let top = stack + 0x14 * (seed % 3);
            c.bus.write32(ctx + R2D_STATE_TOP, top);
            crate::difftest::fill(c, &mut r, stack, 0x14);
            crate::difftest::fill(c, &mut r, DEFAULT_STATE, 5);
            c.bus.write32(ctx + R2D_VT, DATA + 0x5000);
            slot(c, DATA + 0x5000, VT_SET_VIEWPORT, (r.next_u32() as i16) & !3, FAKE);
            slot(c, DATA + 0x5000, VT_SET_TEXTURE, (r.next_u32() as i16) & !3, FAKE + 0x10);
            c.bus.write32(gp.wrapping_sub(0x844), r2d);
            c.bus.write32(gp + LENS_CAMERA_GP, cam);
            c.bus.write32(cam + CAM_PROJECTOR_VT, DATA + 0x6000);
            slot(c, DATA + 0x6000, 0x238, (r.next_u32() as i16) & !3, FAKE + 0x20);
            c.g[A0][0] = sx(module);
        });
    }
    /// Refresh always (flag 0) or by distance (moved less / more than 10),
    /// root cells with and without subtrees; the real tree query runs on
    /// empty subtrees with random cell bounds.
    #[test]
    fn world_sound_refresh_nearby_emitters_matches() {
        crate::difftest::check(ee::s08::f_002b75d0__generated, world_sound_refresh_nearby_emitters, 300, &|c, seed| {
            let mut r = Rng(0x2b75_d000 ^ (seed as u64) << 16);
            crate::difftest::scramble(c, &mut r);
            let gp = crate::difftest::GAME_GP;
            c.g[GP][0] = gp as u64;
            let list = DATA;
            let pos = DATA + 0x200;
            let world = DATA + 0x300;
            let module = DATA + 0x400;
            let tree = DATA + 0x500;
            c.bus.write32(gp.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, module);
            c.bus.write32(module + SOUND_TREE, tree);
            c.bus.write32(gp.wrapping_sub(0x452c), r.f32(2.0));
            let bits = r.next_u32();
            for k in 0..8 {
                let cell = tree + CELL_SIZE * k;
                put_words(c, cell, &[r.next_u32() % 5, (r.next_u32() % 9).wrapping_sub(4), (r.next_u32() % 9).wrapping_sub(4), (r.next_u32() % 9).wrapping_sub(4)]);
                let sub = if seed % 7 == 0 || (bits >> k) & 1 != 0 { DATA + 0x1000 + 0x100 * k } else { 0 };
                c.bus.write32(cell + CELL_SUBTREE, sub);
            }
            let p = [r.f32(20.0), r.f32(20.0), r.f32(20.0), r.f32(1.0)];
            crate::difftest::put_vec(c, pos, p);
            let d = if seed % 2 == 0 { 3.0 } else { 12.0 };
            crate::difftest::put_vec(c, list + NEAR_POS, [
                (f32::from_bits(p[0]) + d).to_bits(), p[1], p[2], r.next_u32(),
            ]);
            c.bus.write32(list + NEAR_REFRESH, if seed % 3 == 0 { 0 } else { r.f32(5.0) | 1 });
            c.bus.write32(list + NEAR_COUNT, r.next_u32() % 8);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(pos);
        });
    }
    /// Empty queue (nothing happens) and 1-6 queued particles sorted by the
    /// real qsort and comparator; random render-state stack contents; the
    /// manager's draw call is a fake (some unwind).
    #[test]
    fn ps2_fog_particle_man_render_matches() {
        covered(ee::s09::f_002dbf98__generated, ps2_fog_particle_man_render, 300, &|c, seed| {
            c.pc = 0x002d_bf98;
            let mut r = machine(c, seed, 0x2dbf98, 1);
            let gp = c.g[GP][0] as u32;
            let man = DATA;
            let ctx = DATA + 0x1000;
            let stack = DATA + 0x2004;
            let n = seed % 7;
            c.bus.write32(man + FOG_COUNT, n);
            c.bus.write32(man + FOG_VT, DATA + 0x5000);
            slot(c, DATA + 0x5000, VT_FOG_DRAW, (r.next_u32() as i16) & !3, FAKE);
            crate::difftest::fill(c, &mut r, FOG_SORT_BUF, 2 * n);
            c.bus.write32(gp.wrapping_sub(R2D_CTX_GP), ctx);
            c.bus.write32(ctx + R2D_STATE_TOP, stack + 0x14 * (seed % 3));
            c.bus.write32(ctx + 0xf60, r.next_u32());
            crate::difftest::fill(c, &mut r, stack, 0x14);
            c.g[A0][0] = sx(man);
        });
    }
    /// 17 effect systems with fake render methods (some unwind).
    #[test]
    fn ps2_visual_effects_man_render_matches() {
        covered(ee::s09::f_002f0cd0__generated, ps2_visual_effects_man_render, 300, &|c, seed| {
            c.pc = 0x002f_0cd0;
            let mut r = machine(c, seed, 0x2f0cd0, VFX_SYSTEMS);
            let man = DATA;
            for k in 0..VFX_SYSTEMS {
                let sys = DATA + 0x100 + 0x40 * k;
                let vt = DATA + 0x1000 + 0x40 * k;
                c.bus.write32(man + 4 * k, sys);
                c.bus.write32(sys, vt);
                slot(c, vt, VT_VFX_RENDER, (r.next_u32() as i16) & !3, FAKE + 0x10 * k);
            }
            c.g[A0][0] = sx(man);
        });
    }
    /// Random and edge-case matrices; the real Math_Det3x3 computes the
    /// minors.
    #[test]
    fn mat4_determinant_matches() {
        crate::difftest::check(ee::s10::f_0031b178__generated, mat4_determinant, 300, &|c, seed| {
            let mut r = Rng(0x31b1_7800 ^ (seed as u64) << 16);
            crate::difftest::scramble(c, &mut r);
            for i in 0..16 {
                let v = if seed % 5 == 0 { r.edge_f32() } else { r.f32(4.0) };
                c.bus.write32(DATA + 4 * i, v);
            }
            c.g[A0][0] = sx(DATA);
        });
    }
}
