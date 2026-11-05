//! Staging wave 204 (oc232): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_4cc0, rider_turn_around_if_backward, "Rider_TurnAroundIfBackward"),
    (0x001f_1b30, hud_draw_text_elem, "HUD_DrawTextElem"),
    (0x0028_7c48, ssx_audio_system_update_fade, "SSXAudioSystem_UpdateFade"),
    (0x0041_26a0, fixunsdfsi, "__fixunsdfsi"),
    (0x0040_e048, ieee754_expf, "__ieee754_expf"),
    (0x003b_85f0, snd_set_volume, "SND_SetVolume"),
    (0x0034_5f90, dynamic_particle_modifier_update, "DynamicParticleModifier_Update"),
    (
        0x0017_6e10,
        default_3_chase_camera_algorithm_update,
        "Default_3ChaseCameraAlgorithm_Update",
    ),
    (0x0034_2358, anim_teeter_v15, "AnimTeeter_v15"),
    (0x002d_a028, avalanche_apply_history_data, "Avalanche_ApplyHistoryData"),
];

// ---------------------------------------------------------------------------
// Rider_TurnAroundIfBackward (0x114cc0)

/// `Rider` fields used by [`rider_turn_around_if_backward`].
mod turn_around {
    /// `+0x120` orientation quaternion.
    pub const ROT: u32 = 0x120;
    /// `+0x1c0` world matrix row z (the up axis).
    pub const WORLD_UP: u32 = 0x1c0;
    /// `+0x1e0` velocity.
    pub const VEL: u32 = 0x1e0;
    /// `+0x214` current brake slew value.
    pub const BRAKE: u32 = 0x214;
    /// `+0x3a0` ground forward direction.
    pub const GROUND_FORWARD: u32 = 0x3a0;
    /// `gp-0x7b98`: how far backward (velocity along the ground forward
    /// axis) the rider must move before turning.
    pub const GP_BACKWARD_LIMIT: u32 = 0x7b98;
}

const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::Rider_BuildWorldMatrix,
};
const RIDER_TOGGLE_STANCE: Callee = Callee {
    addr: 0x0011_5168,
    f: ee::s00::Rider_ToggleStance,
};

/// `bool Rider_TurnAroundIfBackward(Rider* rider)` (0x114cc0): when the
/// rider is not braking and moves backward along the ground
/// (`vel · groundForward < -limit`), turn half way round the up axis (the
/// orientation becomes `(up, 0) ⊗ rot`, a 180° rotation), rebuild the world
/// matrix and switch stance. Returns whether it turned.
pub fn rider_turn_around_if_backward(c: &mut Cpu) {
    use turn_around::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.mtc1(0, 2);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        let r = rider as u32;
        c.lwc1(0, r.wrapping_add(BRAKE));
        c.c_eq_s(0, 2);
        let not_braking = c.fcc();
        c.g[V0][0] = 0;
        if not_braking {
            c.cycles += 15;
            // f1 = vel · groundForward (VU0 dot product, w lane).
            let gp = c.g[GP][0] as u32;
            c.lwc1(0, gp.wrapping_sub(GP_BACKWARD_LIMIT));
            c.lqc2(3, r.wrapping_add(VEL));
            c.lqc2(5, r.wrapping_add(GROUND_FORWARD));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V0, 4);
            c.mtc1(V0, 1);
            c.neg_s(0, 0);
            c.c_lt_s(0, 1);
            if c.fcc() {
                // Not backward enough.
                c.g[V0][0] = 0;
            } else {
                c.cycles += 32;
                // The up axis as a pure quaternion (w = 0 from f2), built
                // through the stack as the original does.
                c.lq(V0, r.wrapping_add(WORLD_UP));
                c.g[A0][0] = rider;
                let s = sp(c);
                c.swc1(2, s.wrapping_add(0x1c));
                c.sq(V0, s.wrapping_add(0x30));
                c.lwc1(0, s.wrapping_add(0x30));
                c.lwc1(1, s.wrapping_add(0x34));
                c.lwc1(2, s.wrapping_add(0x38));
                for base in [0x10u32, 0x20] {
                    c.swc1(0, s.wrapping_add(base));
                    c.swc1(1, s.wrapping_add(base + 4));
                    c.swc1(2, s.wrapping_add(base + 8));
                }
                // rot = up ⊗ rot (VU0 quaternion product).
                c.lqc2(4, s.wrapping_add(0x10));
                c.lqc2(5, r.wrapping_add(ROT));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, s.wrapping_add(0x40));
                c.lq(V0, s.wrapping_add(0x40));
                c.sq(V0, r.wrapping_add(ROT));
                c.sq(V0, s);
                RIDER_BUILD_WORLD_MATRIX.call(c, 0x0011_4d98)?;
                c.g[A0][0] = rider;
                RIDER_TOGGLE_STANCE.call(c, 0x0011_4da0)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD_DrawTextElem (0x1f1b30)

/// HUD element definitions (`sHudElemDef`, 0x24 bytes) and font fields used
/// by [`hud_draw_text_elem`].
mod hud_text {
    /// Size of one element definition.
    pub const DEF_SIZE: u64 = 0x24;
    /// `+0x20` default horizontal alignment (signed byte).
    pub const DEF_ALIGN: u32 = 0x20;
    /// `+0x21` default vertical alignment (signed byte).
    pub const DEF_FLAGS: u32 = 0x21;
    /// `+0x22` render-state layer (5 bits at bit 5 of the render state).
    pub const DEF_LAYER: u32 = 0x22;
    /// "Use the definition's value" for the alignment arguments.
    pub const FROM_DEF: u64 = 0x7f;
    /// Font `+0x30/+0x34` base scale.
    pub const FONT_BASE_SCALE: u32 = 0x30;
    /// Font `+0x38/+0x3c` current scale.
    pub const FONT_SCALE: u32 = 0x38;
    /// Font `+0x40` colour (16 bytes).
    pub const FONT_COLOR: u32 = 0x40;
    /// Font `+0x50` tint (16 bytes).
    pub const FONT_TINT: u32 = 0x50;
    /// `gp-0x854`: the renderer (`D_004A289C`); its `+0xe84` render state
    /// has the layer bits at `+0x8`.
    pub const GP_RENDERER: u32 = 0x854;
    pub const RENDER_STATE: u32 = 0xe84;
    /// Layer bits of the render-state word: kept mask.
    pub const LAYER_KEEP: u64 = 0xffff_ffff_ffff_fc1f;
}

const HUD_GET_ELEM_POS: Callee = Callee {
    addr: 0x001e_91f8,
    f: ee::s04::HUD_GetElemPos,
};
const HUD_GET_ELEM_SCALE: Callee = Callee {
    addr: 0x001e_9290,
    f: ee::s04::HUD_GetElemScale,
};
const HUD_GET_ELEM_COLOR: Callee = Callee {
    addr: 0x001e_91a8,
    f: ee::s04::HUD_GetElemColor,
};
const HUD_ALIGN_ELEM_POS: Callee = Callee {
    addr: 0x001f_10f8,
    f: ee::s04::HUD_AlignElemPos,
};
/// `Font_TextExtent(font, str, rect, 0, sx, sy)` (0x391fb0, other shard).
const FONT_TEXT_EXTENT: u32 = 0x0039_1fb0;
/// `Font_DrawText(font, x, y, str)` (0x391cb0, other shard).
const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;

/// Copy 16 unaligned bytes from `src` to `dst` through GPRs `lo`/`hi` (the
/// `ldl/ldr/sdl/sdr` pairs of a struct copy).
fn copy16_unaligned(c: &mut Cpu, lo: usize, hi: usize, src: u32, dst: u32) {
    c.ldl(lo, src.wrapping_add(0x7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(0x8));
    c.sdl(lo, dst.wrapping_add(0x7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(0x8));
}

/// `void HUD_DrawTextElem(ctx, Font* font, HudElemDef* defs, int idx,
/// const char* str, a5, float* scaleMul, float* posOff, Color* color,
/// float* rect, int align, int flags)` (0x1f1b30): draw `str` at HUD
/// element `defs[idx]`: the element's scale (times `scaleMul`) sizes the
/// font, the text extent and the alignment (`0x7f` = the element's own)
/// place it, `posOff` shifts it; the drawn rectangle goes to `rect`. The
/// colour is `color` or the element's; the element's layer goes into the
/// render state. Afterwards the font's tint and scale are reset.
pub fn hud_draw_text_elem(c: &mut Cpu) {
    use hud_text::*;
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x140);
        c.g[V0][0] = DEF_SIZE;
        frame.save(c, S5, 0xe0);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S4, 0xf0);
        c.mult(0, V1, S5, V0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S1, 0x120);
        frame.save(c, S2, 0x110);
        let font = c.g[A1][0];
        c.g[S1][0] = font;
        let s = sp(c);
        c.sw(s.wrapping_add(0x90), c.g[A0][0]);
        c.g[A1][0] = sx(s.wrapping_add(0x10));
        // The element definition: defs + idx * 0x24.
        let def = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
        c.g[S2][0] = def;
        frame.save(c, S0, 0x130);
        frame.save(c, S3, 0x100);
        c.g[A0][0] = def;
        frame.save(c, S6, 0xd0);
        let scale_mul = c.g[T2][0];
        c.g[S0][0] = scale_mul;
        frame.save(c, S7, 0xc0);
        frame.save(c, FP, 0xb0);
        c.sw(s.wrapping_add(0x94), c.g[T0][0]);
        c.sw(s.wrapping_add(0x98), c.g[T1][0]);
        // Default scale (1, 1) at sp+0.
        c.swc1(0, s.wrapping_add(0x4));
        let rect = c.lw(s.wrapping_add(0x148));
        c.g[S6][0] = rect;
        let align = c.lw(s.wrapping_add(0x150));
        c.g[S7][0] = align;
        let flags = c.lw(s.wrapping_add(0x158));
        c.g[FP][0] = flags;
        c.swc1(0, s);
        frame.save_ra(c, 0xa0);
        let pos_off = c.g[T3][0];
        c.g[S3][0] = pos_off;
        HUD_GET_ELEM_POS.call(c, 0x001f_1bb0)?;
        c.g[A0][0] = def;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
        HUD_GET_ELEM_SCALE.call(c, 0x001f_1bbc)?;
        let s = sp(c);
        c.lwc1(1, s.wrapping_add(0x20));
        if scale_mul != 0 {
            // escale *= scaleMul.
            c.cycles += 7;
            let m = scale_mul as u32;
            c.lwc1(3, m.wrapping_add(0x4));
            c.lwc1(2, m);
            c.lwc1(0, s.wrapping_add(0x24));
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, s.wrapping_add(0x20));
            c.swc1(0, s.wrapping_add(0x24));
        }

        // font->scale = font->baseScale * escale; measure the text.
        c.cycles += 18;
        let f = font as u32;
        c.lwc1(3, s.wrapping_add(0x20));
        c.g[A0][0] = font;
        c.lwc1(2, s.wrapping_add(0x24));
        c.g[A2][0] = sx(s.wrapping_add(0x30));
        c.lwc1(1, f.wrapping_add(FONT_BASE_SCALE));
        c.g[A3][0] = 0;
        c.lwc1(0, f.wrapping_add(FONT_BASE_SCALE + 4));
        c.mul_s(1, 1, 3);
        c.g[A1][0] = c.lw(s.wrapping_add(0x94));
        c.mul_s(0, 0, 2);
        c.mov_s(12, 1);
        c.swc1(1, f.wrapping_add(FONT_SCALE));
        c.mov_s(13, 0);
        c.swc1(0, f.wrapping_add(FONT_SCALE + 4));
        call_ptr(c, FONT_TEXT_EXTENT, 0x001f_1c1c)?;

        // Alignment: 0x7f takes the element's own.
        c.g[V0][0] = FROM_DEF;
        c.g[T1][0] = align;
        if align == FROM_DEF {
            c.cycles += 1;
            c.g[T1][0] = c.lb((def as u32).wrapping_add(DEF_ALIGN));
        }
        c.cycles += 2;
        c.g[T2][0] = flags;
        if flags == FROM_DEF {
            c.cycles += 4;
            c.g[V0][0] = DEF_SIZE;
            c.mult(0, V1, S5, V0);
            let d = (c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32);
            c.g[V0][0] = sx(d);
            c.g[T2][0] = c.lb(d.wrapping_add(DEF_FLAGS));
        }

        // HUD_AlignElemPos(ctx, out, pos, size, scale, ax, ay, a5).
        c.cycles += 13;
        let s = sp(c);
        c.lwc1(0, s.wrapping_add(0x38));
        c.g[A2][0] = sx(s.wrapping_add(0x10));
        c.lwc1(1, s.wrapping_add(0x3c));
        c.g[A1][0] = sx(s.wrapping_add(0x40));
        c.g[A0][0] = c.lw(s.wrapping_add(0x90));
        c.g[A3][0] = sx(s.wrapping_add(0x50));
        c.g[T3][0] = c.lw(s.wrapping_add(0x98));
        c.g[T0][0] = c.g[SP][0];
        c.swc1(0, s.wrapping_add(0x50));
        c.swc1(1, s.wrapping_add(0x54));
        HUD_ALIGN_ELEM_POS.call(c, 0x001f_1c70)?;
        let s = sp(c);
        c.lwc1(1, s.wrapping_add(0x40));
        if pos_off != 0 {
            // out += posOff.
            c.cycles += 7;
            let o = pos_off as u32;
            c.lwc1(3, o.wrapping_add(0x4));
            c.lwc1(2, o);
            c.lwc1(0, s.wrapping_add(0x44));
            c.add_s(1, 1, 2);
            c.add_s(0, 0, 3);
            c.swc1(1, s.wrapping_add(0x40));
            c.swc1(0, s.wrapping_add(0x44));
        }
        c.cycles += 2;
        c.lwc1(1, s.wrapping_add(0x40));
        if rect != 0 {
            // rect = (out.x, out.y, extent.w, extent.h).
            c.cycles += 7;
            let r = rect as u32;
            c.lwc1(2, s.wrapping_add(0x44));
            c.lwc1(3, s.wrapping_add(0x38));
            c.lwc1(0, s.wrapping_add(0x3c));
            c.swc1(1, r);
            c.swc1(0, r.wrapping_add(0xc));
            c.swc1(2, r.wrapping_add(0x4));
            c.swc1(3, r.wrapping_add(0x8));
        }

        // The colour (sp+0x60): the caller's or the element's.
        c.cycles += 3;
        let color = c.lw(s.wrapping_add(0x140));
        c.g[A0][0] = color;
        c.g[V1][0] = color;
        if color == 0 {
            c.cycles += 7;
            c.g[A0][0] = DEF_SIZE;
            c.g[A1][0] = sx(s.wrapping_add(0x60));
            c.mult(0, V0, S5, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            HUD_GET_ELEM_COLOR.call(c, 0x001f_1cd8)?;
        } else {
            c.cycles += 9;
            copy16_unaligned(c, A0, A1, color as u32, s.wrapping_add(0x60));
        }
        c.g[A2][0] = DEF_SIZE;

        // Tint (col.r, 0, 0, 0) at sp+0x70; the element's layer into the
        // render state; colour and tint into the font; draw.
        c.cycles += 72;
        let s = sp(c);
        c.lwc1(0, s.wrapping_add(0x60));
        c.mult(0, V0, S5, A2);
        c.sw(s.wrapping_add(0x74), 0);
        c.swc1(0, s.wrapping_add(0x70));
        c.g[T0][0] = LAYER_KEEP;
        c.sw(s.wrapping_add(0x78), 0);
        c.sw(s.wrapping_add(0x7c), 0);
        let d = (c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32);
        c.g[A2][0] = sx(d);
        c.g[A1][0] = c.lw(s.wrapping_add(0x94));
        let gp = c.g[GP][0] as u32;
        let renderer = c.lw(gp.wrapping_sub(GP_RENDERER));
        let layer = c.lb(d.wrapping_add(DEF_LAYER));
        let state = c.lw((renderer as u32).wrapping_add(RENDER_STATE));
        c.g[A3][0] = state;
        let layer = sx((layer as u32) << 5) & 0x3e0;
        c.g[V1][0] = layer;
        let word = c.lw((state as u32).wrapping_add(0x8));
        let word = (word & LAYER_KEEP) | layer;
        c.g[V0][0] = word;
        c.sw((state as u32).wrapping_add(0x8), word);
        copy16_unaligned(c, V1, A2, s.wrapping_add(0x60), f.wrapping_add(FONT_COLOR));
        c.lwc1(12, s.wrapping_add(0x40));
        c.lwc1(13, s.wrapping_add(0x44));
        copy16_unaligned(c, V1, A2, s.wrapping_add(0x70), f.wrapping_add(FONT_TINT));
        c.g[A0][0] = font;
        call_ptr(c, FONT_DRAW_TEXT, 0x001f_1d9c)?;

        // Reset the tint to (1, 0, 0, 0) and the scale to baseScale * (1, 1).
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        let s = sp(c);
        c.sw(s.wrapping_add(0x84), 0);
        c.swc1(0, s.wrapping_add(0x80));
        c.sw(s.wrapping_add(0x88), 0);
        c.sw(s.wrapping_add(0x8c), 0);
        c.lwc1(3, s);
        copy16_unaligned(c, V1, A0, s.wrapping_add(0x80), f.wrapping_add(FONT_TINT));
        frame.restore(c, S0, 0x130);
        c.lwc1(2, s.wrapping_add(0x4));
        c.lwc1(0, f.wrapping_add(FONT_BASE_SCALE));
        c.lwc1(1, f.wrapping_add(FONT_BASE_SCALE + 4));
        c.mul_s(0, 0, 3);
        frame.restore(c, S2, 0x110);
        c.mul_s(1, 1, 2);
        frame.restore(c, S3, 0x100);
        frame.restore(c, S4, 0xf0);
        c.swc1(0, f.wrapping_add(FONT_SCALE));
        c.swc1(1, f.wrapping_add(FONT_SCALE + 4));
        frame.restore(c, S1, 0x120);
        frame.restore(c, S5, 0xe0);
        frame.restore(c, S6, 0xd0);
        frame.restore(c, S7, 0xc0);
        frame.restore(c, FP, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateFade (0x287c48)

/// `SSXAudioSystem` mixer fade fields used by
/// [`ssx_audio_system_update_fade`].
mod audio_fade {
    /// `+0x63c0` a fade runs.
    pub const FADING: u32 = 0x63c0;
    /// `+0x63c4` which mixer channels follow the fade (bit mask).
    pub const MASK: u32 = 0x63c4;
    /// `+0x63c8` fade step per tick.
    pub const RATE: u32 = 0x63c8;
    /// `+0x63cc` fade target level.
    pub const TARGET: u32 = 0x63cc;
    /// `+0x63d0` current fade level.
    pub const CUR: u32 = 0x63d0;
    /// A mixer channel: `+0` volume, `+4` volume * fade, `+8` that * gain.
    pub struct Chan {
        pub bit: u64,
        pub base: u32,
        pub gain: u32,
        /// Where the channel's copy of the fade level lives.
        pub level: u32,
    }
    const fn ch(bit: u64, base: u32, gain: u32, level: u32) -> Chan {
        Chan { bit, base, gain, level }
    }
    /// Channels 1, 5 (before the reverb-mix channel 4).
    pub const FIRST: [Chan; 2] = [
        ch(0x1, 0x62d0, 0x63e8, 0x6398),
        ch(0x2, 0x6310, 0x63f8, 0x63a8),
    ];
    /// Channel 4: also scales the ten send levels at `+0x636c` by the
    /// constants at `gp+0x43c..`.
    pub const MIX: Chan = ch(0x4, 0x6300, 0x63f4, 0x63a4);
    pub const MIX_SENDS: u32 = 0x636c;
    pub const GP_MIX_LEVELS: u32 = 0x43c;
    /// FPU registers the ten send levels go through.
    pub const MIX_REGS: [usize; 10] = [1, 2, 4, 5, 7, 9, 10, 8, 6, 3];
    /// Channel 2 (its mask is reloaded first).
    pub const AFTER_MIX: Chan = ch(0x8, 0x62e0, 0x63ec, 0x639c);
    /// Channels 3, 6, 7, 8, 9.
    pub const REST: [Chan; 5] = [
        ch(0x80, 0x62f0, 0x63f0, 0x63a0),
        ch(0x20, 0x6320, 0x63fc, 0x63ac),
        ch(0x40, 0x6330, 0x6400, 0x63b0),
        ch(0x10, 0x6340, 0x6404, 0x63b4),
        ch(0x100, 0x6350, 0x6408, 0x63b8),
    ];
    /// Channel 10 (last: no mask reload).
    pub const LAST: Chan = ch(0x200, 0x6360, 0x640c, 0x63bc);
}

/// One mixer channel follows the fade level: `level = cur`,
/// `vol' = vol * cur`, `out = vol' * gain`.
fn fade_channel(c: &mut Cpu, s: u32, ch: &audio_fade::Chan) {
    c.lwc1(0, s.wrapping_add(audio_fade::CUR));
    c.lwc1(1, s.wrapping_add(ch.base));
    c.lwc1(2, s.wrapping_add(ch.gain));
    c.mul_s(1, 1, 0);
    c.swc1(0, s.wrapping_add(ch.level));
    c.mul_s(2, 1, 2);
    c.swc1(1, s.wrapping_add(ch.base + 4));
    c.swc1(2, s.wrapping_add(ch.base + 8));
}

/// `void SSXAudioSystem::UpdateFade()` (0x287c48): while a fade runs, step
/// the fade level by the rate and stop at the target (from either side);
/// then every channel selected by the mask takes the level (channel 4 also
/// rescales its ten send levels).
pub fn ssx_audio_system_update_fade(c: &mut Cpu) {
    use audio_fade::*;
    let s = c.g[A0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = c.lw(s.wrapping_add(FADING));
    if c.g[V0][0] != 0 {
        // cur += rate; past the target: clamp and stop.
        c.cycles += 8;
        c.lwc1(0, s.wrapping_add(CUR));
        c.lwc1(1, s.wrapping_add(RATE));
        c.mtc1(0, 2);
        c.add_s(0, 0, 1);
        c.c_lt_s(1, 2);
        let falling = c.fcc();
        c.swc1(0, s.wrapping_add(CUR));
        if falling {
            c.cycles += 4;
            c.lwc1(1, s.wrapping_add(TARGET));
            c.c_lt_s(0, 1);
        } else {
            c.cycles += 3;
            c.lwc1(1, s.wrapping_add(TARGET));
            c.c_lt_s(1, 0);
        }
        c.cycles += 2;
        if c.fcc() {
            c.cycles += 3;
            c.swc1(1, s.wrapping_add(CUR));
            c.sw(s.wrapping_add(FADING), 0);
        }
        c.g[V0][0] = c.lw(s.wrapping_add(MASK));

        for ch in FIRST.iter() {
            c.cycles += 3;
            c.g[V0][0] &= ch.bit;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                fade_channel(c, s, ch);
            }
            c.g[V0][0] = c.lw(s.wrapping_add(MASK));
        }

        c.cycles += 3;
        c.g[V0][0] &= MIX.bit;
        let gp = c.g[GP][0] as u32;
        c.lwc1(MIX_REGS[0], gp.wrapping_add(GP_MIX_LEVELS));
        if c.g[V0][0] != 0 {
            c.cycles += 37;
            c.lwc1(12, s.wrapping_add(CUR));
            c.lwc1(11, s.wrapping_add(MIX.base));
            c.lwc1(0, s.wrapping_add(MIX.gain));
            c.mul_s(11, 11, 12);
            c.mul_s(0, 11, 0);
            for (i, &f) in MIX_REGS.iter().enumerate() {
                let k = gp.wrapping_add(GP_MIX_LEVELS + 4 * i as u32);
                c.lwc1(f, k);
                c.mul_s(f, 0, f);
                c.swc1(f, s.wrapping_add(MIX_SENDS + 4 * i as u32));
            }
            c.swc1(12, s.wrapping_add(MIX.level));
            c.swc1(11, s.wrapping_add(MIX.base + 4));
            c.swc1(0, s.wrapping_add(MIX.base + 8));
        }

        c.cycles += 1;
        c.g[V0][0] = c.lw(s.wrapping_add(MASK));
        for ch in core::iter::once(&AFTER_MIX).chain(REST.iter()) {
            c.cycles += 3;
            c.g[V0][0] &= ch.bit;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                fade_channel(c, s, ch);
            }
            c.g[V0][0] = c.lw(s.wrapping_add(MASK));
        }

        c.cycles += 3;
        c.g[V0][0] &= LAST.bit;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            fade_channel(c, s, &LAST);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// __fixunsdfsi (0x4126a0)

/// `__unpack_d(src, dst)` (0x411c38).
const UNPACK_D: Callee = Callee {
    addr: 0x0041_1c38,
    f: ee::s15::__unpack_d,
};

/// `fp_number_type` (the soft-float unpacked number) fields.
mod fp_number {
    /// `+0x0` class: 0/1 NaN, 2 zero, 3 normal, 4 infinity.
    pub const CLASS: u32 = 0x0;
    pub const SIGN: u32 = 0x4;
    pub const EXP: u32 = 0x8;
    /// `+0x10` 64-bit fraction (binary point after bit 60).
    pub const FRACTION: u32 = 0x10;
    pub const CLASS_ZERO: u64 = 2;
    pub const CLASS_INFINITY: u64 = 4;
    /// Fraction bits below the binary point.
    pub const FRACBITS: u64 = 0x3c;
}

/// `unsigned __fixunsdfsi(double x)` (0x4126a0, libgcc soft-float):
/// truncate `x` (in a0) to an unsigned int: zero, NaN, negative numbers
/// and |x| < 1 give 0, infinity and x >= 2^32 give 0xffffffff, otherwise
/// the fraction shifted down to the integer part (sign-extended in v0).
pub fn fixunsdfsi(c: &mut Cpu) {
    use fp_number::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let s = sp(c);
        c.sd(s.wrapping_add(0x20), c.g[A0][0]);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx(s.wrapping_add(0x20));
        UNPACK_D.call(c, 0x0041_26b8)?;
        let class = c.lw(s.wrapping_add(CLASS));
        c.g[V1][0] = class;
        let result: u64 = 'r: {
            if class == CLASS_ZERO {
                c.cycles += 2;
                break 'r 0;
            }
            // NaN.
            c.cycles += 2;
            c.g[V0][0] = c.lw(s.wrapping_add(SIGN));
            if class < 2 {
                c.cycles += 2;
                break 'r 0;
            }
            // Negative.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                break 'r 0;
            }
            c.cycles += 3;
            let exp = c.lw(s.wrapping_add(EXP));
            c.g[A0][0] = exp;
            if class == CLASS_INFINITY {
                c.cycles += 3;
                break 'r u64::MAX;
            }
            // |x| < 1: the original's backward branch to the `return 0`
            // block checks for events.
            c.cycles += 2;
            if (exp as i64) < 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0041_26d0);
                }
                c.cycles += 2;
                break 'r 0;
            }
            // Too large.
            c.cycles += 2;
            if (exp as i64) >= 32 {
                c.cycles += 3;
                break 'r u64::MAX;
            }
            c.cycles += 2;
            c.g[V1][0] = FRACBITS;
            let fraction = c.ld(s.wrapping_add(FRACTION));
            let int = if (exp as i64) < 61 {
                c.cycles += 3;
                let shift = sx((FRACBITS as u32).wrapping_sub(exp as u32));
                c.g[V1][0] = shift;
                fraction >> (shift & 63)
            } else {
                c.cycles += 4;
                let shift = sx((exp as u32).wrapping_sub(FRACBITS as u32));
                c.g[V1][0] = shift;
                fraction << (shift & 63)
            };
            c.cycles += 2;
            sx(int as u32)
        };
        c.g[V0][0] = result;
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __ieee754_expf (0x40e048)

/// `__ieee754_expf` constants (fdlibm `e_expf.c`).
mod expf {
    /// Above this |x| bit pattern the result overflows or underflows.
    pub const LIMIT: u64 = 0x42b1_7217;
    pub const INF: u64 = 0x7f80_0000;
    /// `o_threshold` 88.72168 and `u_threshold` -103.97208.
    pub const O_THRESHOLD: u64 = 0x42b1_7180;
    pub const U_THRESHOLD: u64 = 0xffff_ffff_c2cf_f1b5;
    /// `0.5 ln2`, `1.5 ln2` (|x| bit patterns) and `2^-28`.
    pub const HALF_LN2: u64 = 0x3eb1_7218;
    pub const THREE_HALVES_LN2: u64 = 0x3f85_1591;
    pub const TINY: u64 = 0x317f_ffff;
    /// `huge`, `invln2`, `twom100`.
    pub const HUGE: u64 = 0x7149_f2ca;
    pub const INVLN2: u64 = 0x3fb8_aa3b;
    pub const TWOM100: u64 = 0x0d80_0000;
    pub const ONE: u64 = 0x3f80_0000;
    pub const TWO: u64 = 0x4000_0000;
    /// Polynomial coefficients P1..P5 (applied P5 first).
    pub const P: [u64; 5] = [
        0x3331_bb4c,
        0xffff_ffff_b5dd_ea0e,
        0x388a_b355,
        0xffff_ffff_bb36_0b61,
        0x3e2a_aaab,
    ];
    /// `.rodata` tables (by sign): `halF[2]`, `ln2HI[2]`, `ln2LO[2]`, and
    /// the overflow result `huge * huge`.
    pub const HALF_F: u32 = 0x0049_91a0;
    pub const LN2_HI: u32 = 0x0049_91b8;
    pub const LN2_LO: u32 = 0x0049_91c0;
    pub const OVERFLOW: u32 = 0x0049_91e0;
    /// `k` below which the result is scaled through `2^-100`.
    pub const K_SUBNORMAL: i64 = -125;
}

/// `at = k; fN = at` (a `lui/ori; mtc1` float constant).
fn expf_const(c: &mut Cpu, f: usize, bits: u64) {
    c.g[AT][0] = bits;
    c.mtc1(AT, f);
}

/// `float __ieee754_expf(float x)` (0x40e048, fdlibm): `e^x`. Large |x|
/// overflows to `huge*huge` or underflows to 0 (inf gives inf or 0, NaN
/// gives NaN); otherwise `x = k ln2 + r` and `e^r` by the rational
/// approximation, scaled by `2^k`.
pub fn ieee754_expf(c: &mut Cpu) {
    use expf::*;
    c.cycles += 11;
    c.mfc1(V0, 12);
    let bits = c.g[V0][0];
    c.g[A0][0] = bits;
    let xsb = sx((bits as u32) >> 31);
    c.g[A3][0] = xsb;
    let hx = bits & 0x7fff_ffff;
    c.g[A0][0] = hx;
    c.g[V0][0] = (LIMIT < hx) as u64;
    c.g[V1][0] = INF;
    if LIMIT < hx {
        c.cycles += 3;
        c.g[V0][0] = (INF < hx) as u64;
        if INF < hx {
            // NaN.
            c.cycles += 2;
            c.add_s(0, 12, 12);
            ret(c);
            return;
        }
        c.cycles += 2;
        if hx == INF {
            // e^+inf = inf, e^-inf = 0.
            c.cycles += 3;
            c.mtc1(0, 0);
            if xsb == 0 {
                c.mov_s(0, 12);
            }
            c.cycles += 2;
            ret(c);
            return;
        }
        c.cycles += 7;
        expf_const(c, 0, O_THRESHOLD);
        c.c_lt_s(0, 12);
        c.g[V0][0] = 0x4a_0000;
        if c.fcc() {
            // Overflow.
            c.cycles += 2;
            c.lwc1(0, OVERFLOW);
            ret(c);
            return;
        }
        c.cycles += 7;
        expf_const(c, 0, U_THRESHOLD);
        c.c_lt_s(12, 0);
        c.g[V0][0] = 0x3eb1_0000;
        if c.fcc() {
            // Underflow.
            c.cycles += 3;
            c.mtc1(0, 0);
            ret(c);
            return;
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0x3eb1_0000;
    }

    // Argument reduction: x = hi - lo = x - k ln2 (k in a2); f5 = x*x.
    c.cycles += 4;
    c.g[V0][0] = (HALF_LN2 < hx) as u64;
    c.g[V0][0] = 0x3f85_0000;
    if HALF_LN2 < hx {
        c.cycles += 4;
        c.g[V0][0] = (THREE_HALVES_LN2 < hx) as u64;
        c.g[V0][0] = 0x4a_0000;
        if THREE_HALVES_LN2 < hx {
            // k = (int)(invln2 * x + halF[xsb]).
            c.cycles += 20;
            expf_const(c, 0, INVLN2);
            c.g[V0][0] = sx(HALF_F);
            let half = HALF_F.wrapping_add((xsb as u32) << 2);
            c.g[V1][0] = sx(half);
            c.mul_s(0, 12, 0);
            c.lwc1(2, half);
            c.g[V0][0] = 0x4a_0000;
            c.lwc1(1, LN2_HI);
            c.g[V1][0] = 0x4a_0000;
            c.add_s(0, 0, 2);
            c.lwc1(3, LN2_LO);
            c.cvt_w_s(2, 0);
            c.mfc1(A2, 2);
            c.mtc1(A2, 5);
            c.cvt_s_w(5, 5);
            c.mul_s(1, 5, 1);
            c.mul_s(6, 5, 3);
            c.sub_s(7, 12, 1);
        } else {
            // k = 1 - xsb - xsb.
            c.cycles += 14;
            let idx = sx((xsb as u32) << 2);
            c.g[A1][0] = idx;
            let hi_at = LN2_HI.wrapping_add(idx as u32);
            c.g[V0][0] = sx(hi_at);
            c.g[A0][0] = sx(LN2_LO);
            c.lwc1(0, hi_at);
            let lo_at = LN2_LO.wrapping_add(idx as u32);
            c.g[A1][0] = sx(lo_at);
            let one_minus = sx(1u32.wrapping_sub(xsb as u32));
            c.g[V1][0] = one_minus;
            c.sub_s(7, 12, 0);
            c.g[A2][0] = sx((one_minus as u32).wrapping_sub(xsb as u32));
            c.lwc1(6, lo_at);
        }
        c.cycles += 2;
        c.sub_s(12, 7, 6);
        c.cycles += 1;
        c.mul_s(5, 12, 12);
    } else {
        c.cycles += 5;
        c.g[V0][0] = (TINY < hx) as u64;
        if TINY < hx {
            c.g[A2][0] = 0;
            c.cycles += 1;
            c.mul_s(5, 12, 12);
        } else {
            // |x| < 2^-28: 1 + x (raising inexact).
            c.cycles += 10;
            expf_const(c, 0, HUGE);
            expf_const(c, 1, ONE);
            c.add_s(0, 12, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 2;
                c.add_s(0, 12, 1);
                ret(c);
                return;
            }
            c.mul_s(5, 12, 12);
        }
    }

    // c = x - t*(P1 + t*(P2 + t*(P3 + t*(P4 + t*P5)))).
    c.cycles += 26;
    expf_const(c, 0, P[0]);
    expf_const(c, 1, P[1]);
    expf_const(c, 2, P[2]);
    c.mul_s(0, 5, 0);
    expf_const(c, 3, P[3]);
    expf_const(c, 4, P[4]);
    for f in 1..5 {
        c.add_s(0, 0, f);
        c.mul_s(0, 5, 0);
    }
    let k = c.g[A2][0];
    c.sub_s(3, 12, 0);
    if k != 0 {
        // y = 1 - ((lo - x*c/(2-c)) - hi), scaled by 2^k.
        c.cycles += 16;
        expf_const(c, 1, TWO);
        c.mul_s(0, 12, 3);
        expf_const(c, 2, ONE);
        let subnormal = (k as i64) < K_SUBNORMAL;
        c.g[V0][0] = subnormal as u64;
        c.sub_s(1, 1, 3);
        c.div_s(0, 0, 1);
        c.sub_s(0, 6, 0);
        c.sub_s(0, 0, 7);
        c.sub_s(2, 2, 0);
        c.mfc1(V1, 2);
        let y = c.g[V1][0];
        c.g[V0][0] = y;
        if subnormal {
            c.cycles += 9;
            let e = sx(((k as u32).wrapping_add(100)) << 23);
            c.g[V1][0] = sx((y as u32).wrapping_add(e as u32));
            expf_const(c, 0, TWOM100);
            c.mtc1(V1, 1);
            c.mul_s(0, 1, 0);
        } else {
            c.cycles += 5;
            let e = sx((k as u32) << 23);
            c.g[V0][0] = e;
            c.g[V1][0] = sx((y as u32).wrapping_add(e as u32));
            c.mtc1(V1, 0);
        }
    } else {
        // y = 1 - ((x*c)/(c-2) - x).
        c.cycles += 12;
        expf_const(c, 1, TWO);
        c.mul_s(0, 12, 3);
        expf_const(c, 2, ONE);
        c.sub_s(1, 3, 1);
        c.div_s(0, 0, 1);
        c.sub_s(0, 0, 12);
        c.sub_s(0, 2, 0);
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_SetVolume (0x3b85f0)

/// Addresses of `SND_SetVolume`.
mod set_volume {
    pub const START: u32 = 0x003b_85f0;
    pub const END: u32 = 0x003b_86c0;
    /// Loop head (the per-channel body), the resume point after an unwind.
    pub const HEAD: u32 = 0x003b_8638;
}

/// The sound driver's state block (`0x510000 - 0x5718`) and its channels
/// (0x8c bytes each), as `SND_SetVolume` uses them.
mod snd_vol {
    /// `lui 0x51` (the state block's high half, left in v0).
    pub const STATE_HI: u64 = 0x51_0000;
    pub const STATE: u32 = 0x0050_a8e8;
    /// `+0x1f0` the channel table.
    pub const CHANNELS: u32 = 0x1f0;
    pub const CHANNEL_SIZE: u64 = 0x8c;
    /// Channel `+0x4` the channel the send applies to (`s16`).
    pub const SEND_CHANNEL: u32 = 0x4;
    /// Channel `+0x30` cleared on a volume change.
    pub const RAMP: u32 = 0x30;
    /// Channel `+0x38` the handle's volume (`volume << 16`).
    pub const VOLUME: u32 = 0x38;
    /// Channel `+0x62` effect send level (`s16`, > 0: apply the send).
    pub const SEND: u32 = 0x62;
}

const SND_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};
const SND_CHANNEL_CALC_VOLUME: Callee = Callee {
    addr: 0x003b_a7b0,
    f: ee::s13::SND_ChannelCalcVolume,
};
const SND_NEXT_HANDLE_CHANNEL: Callee = Callee {
    addr: 0x003b_aa00,
    f: ee::s13::SND_NextHandleChannel,
};
const SND_CHANNEL_APPLY_VOLUME: Callee = Callee {
    addr: 0x003c_2a50,
    f: ee::s13::SND_ChannelApplyVolume,
};
const SND_CHANNEL_APPLY_SEND: Callee = Callee {
    addr: 0x003c_27e0,
    f: ee::s13::SND_ChannelApplySend,
};

/// `int SND_SetVolume(int handle, unsigned volume)` (0x3b85f0): set the
/// volume of every voice channel of a handle and recompute and apply it
/// (and the effect send where the channel has one). Returns the handle's
/// first channel (or the error from `SND_HandleToChannel`), but 0 at once
/// when a channel already has this volume (the rest are not visited).
///
/// The original is a `match pc` dispatcher whose loop returns on an unwind
/// request, so the loop head is an entry point too (registers: s1 the first
/// channel, s2 the volume, s3 0x8c, s4 the state block, a1 the cursor; the
/// cursor also at `0(sp)`).
pub fn snd_set_volume(c: &mut Cpu) {
    use set_volume::*;
    use snd_vol::*;
    let frame = Frame { size: 0x70 };
    let resumed = match c.pc {
        START => false,
        HEAD => true,
        pc => {
            c.jump_out(pc, START, END);
            return;
        }
    };
    guest(c, |c| {
        if !resumed {
            c.cycles += 12;
            Frame::open(c, frame.size);
            frame.save(c, S2, 0x40);
            frame.save(c, S1, 0x50);
            frame.save(c, S0, 0x60);
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S2][0] = c.g[A1][0];
            SND_HANDLE_TO_CHANNEL.call(c, 0x003b_8614)?;
            let first = c.g[V0][0];
            c.g[S1][0] = first;
            c.g[V1][0] = u64::MAX;
            if (first as i64) < 0 {
                return set_volume_exit(c, frame);
            }
            // The cursor starts at -1; the first step finds the first
            // channel.
            c.cycles += 5;
            c.g[V0][0] = STATE_HI;
            c.g[S4][0] = sx(STATE);
            c.sw(sp(c), u64::MAX);
            c.g[S3][0] = CHANNEL_SIZE;
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            if !set_volume_next(c)? {
                return set_volume_exit(c, frame);
            }
            if back_edge_unwinds(c, HEAD) {
                return Ok(());
            }
        }
        loop {
            // Loop head: the cursor's channel (a1).
            c.cycles += 7;
            let volume = sx((c.g[S2][0] as u32) << 16);
            c.g[A2][0] = volume;
            let table = c.lw((c.g[S4][0] as u32).wrapping_add(CHANNELS));
            c.g[V1][0] = table;
            c.mult(0, V0, A1, S3);
            let ch = sx((table as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = ch;
            let ch = ch as u32;
            let current = c.lw(ch.wrapping_add(VOLUME));
            c.g[A0][0] = current;
            c.sw(ch.wrapping_add(RAMP), 0);
            if current == volume {
                c.cycles += 2;
                c.g[V0][0] = 0;
                return set_volume_epilogue(c, frame);
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            c.sw(ch.wrapping_add(VOLUME), volume);
            SND_CHANNEL_CALC_VOLUME.call(c, 0x003b_8668)?;
            c.g[A0][0] = c.lw(sp(c));
            SND_CHANNEL_APPLY_VOLUME.call(c, 0x003b_8670)?;
            let send = c.lh(ch.wrapping_add(SEND));
            c.g[V0][0] = send;
            c.g[A0][0] = c.g[S1][0];
            if (send as i64) > 0 {
                c.cycles += 3;
                c.g[A0][0] = c.lh(ch.wrapping_add(SEND_CHANNEL));
                c.g[A1][0] = 0;
                SND_CHANNEL_APPLY_SEND.call(c, 0x003b_8688)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
            }
            if !set_volume_next(c)? {
                return set_volume_exit(c, frame);
            }
            if back_edge_unwinds(c, HEAD) {
                return Ok(());
            }
        }
    })
}

/// `SND_NextHandleChannel(first, sp)` for [`snd_set_volume`]: true when
/// there is another channel (its index then in a1).
fn set_volume_next(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[A1][0] = c.g[SP][0];
    SND_NEXT_HANDLE_CHANNEL.call(c, 0x003b_8694)?;
    if c.g[V0][0] != 0 {
        c.g[A1][0] = c.lw(sp(c));
        return Ok(true);
    }
    Ok(false)
}

/// Return the first channel (s1).
fn set_volume_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 1;
    c.g[V0][0] = c.g[S1][0];
    set_volume_epilogue(c, frame)
}

fn set_volume_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 8;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// DynamicParticleModifier_Update (0x345f90)

/// `DynamicParticleModifier` fields used by
/// [`dynamic_particle_modifier_update`].
mod particle_mod {
    /// `+0x14` emit count (> 0: a continuous emitter).
    pub const EMIT_COUNT: u32 = 0x14;
    /// `+0x18` finished.
    pub const DONE: u32 = 0x18;
    /// `+0x30`, `+0x40` the transformed emitter points.
    pub const POINT_A: u32 = 0x30;
    pub const POINT_B: u32 = 0x40;
    /// `+0x60` the particle system (its first float: remaining life).
    pub const SYSTEM: u32 = 0x60;
    /// `+0x240` the system's live particle count.
    pub const LIVE: u32 = 0x240;
    /// `gp-0x2b48/-0x2b44/-0x2b40`: the frame step (1/60) of the three
    /// update calls.
    pub const GP_STEP_LIVE: u32 = 0x2b48;
    pub const GP_STEP_EXPIRED: u32 = 0x2b44;
    pub const GP_STEP_DYING: u32 = 0x2b40;
}

const DYNAMIC_PARTICLE_MODIFIER_TRANSFORM_POINTS: Callee = Callee {
    addr: 0x0034_5ef8,
    f: ee::s11::DynamicParticleModifier_TransformPoints,
};
/// `ParticleSystem_Update(system, a, b, emit, dt)` (0x3710d0, other shard).
const PARTICLE_SYSTEM_UPDATE: u32 = 0x0037_10d0;

/// `ParticleSystem_Update(self + 0x60, self + 0x30, self + 0x40, emit,
/// *(gp - step))` returning to `ret`.
fn particle_update(c: &mut Cpu, s: u32, emit: u64, step: u32, ret: u32) -> crate::call::Flow {
    use particle_mod::*;
    let gp = c.g[GP][0] as u32;
    c.lwc1(12, gp.wrapping_sub(step));
    c.g[A2][0] = sx(s.wrapping_add(POINT_B));
    c.g[A1][0] = sx(s.wrapping_add(POINT_A));
    c.g[A3][0] = emit;
    call_ptr(c, PARTICLE_SYSTEM_UPDATE, ret)
}

/// `void DynamicParticleModifier::Update()` (0x345f90): unless finished,
/// move the emitter points with the instance and update the particle
/// system: a continuous emitter (count > 0) updates while particles live
/// and finishes when none are left; a one-shot one updates with emission
/// on and finishes once its life (`system[0]`) runs out.
pub fn dynamic_particle_modifier_update(c: &mut Cpu) {
    use particle_mod::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let s = c.g[A0][0];
        c.g[S0][0] = s;
        frame.save_fpr(c, 20, 0x20);
        let s = s as u32;
        c.g[V0][0] = c.lw(s.wrapping_add(DONE));
        let finish = 'f: {
            if c.g[V0][0] != 0 {
                break 'f false;
            }
            c.cycles += 5;
            DYNAMIC_PARTICLE_MODIFIER_TRANSFORM_POINTS.call(c, 0x0034_5fb8)?;
            c.g[V0][0] = c.lw(s.wrapping_add(EMIT_COUNT));
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(s.wrapping_add(LIVE));
                c.g[A0][0] = sx(s.wrapping_add(SYSTEM));
                if (c.g[V0][0] as i64) <= 0 {
                    break 'f true;
                }
                c.cycles += 7;
                particle_update(c, s, 0, GP_STEP_LIVE, 0x0034_5fe4)?;
                break 'f false;
            }
            c.lwc1(0, s.wrapping_add(SYSTEM));
            c.cycles += 5;
            c.mtc1(0, 20);
            c.c_lt_s(0, 20);
            c.g[A0][0] = sx(s.wrapping_add(SYSTEM));
            if c.fcc() {
                c.cycles += 7;
                particle_update(c, s, 1, GP_STEP_EXPIRED, 0x0034_6014)?;
                break 'f false;
            }
            c.cycles += 10;
            particle_update(c, s, 1, GP_STEP_DYING, 0x0034_6030)?;
            c.lwc1(0, s.wrapping_add(SYSTEM));
            c.c_le_s(0, 20);
            c.fcc()
        };
        if finish {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sw(s.wrapping_add(DONE), 1);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Default_3ChaseCameraAlgorithm_Update (0x176e10)

/// One stage of a chase camera update: `fn(algo, &info, f12..)` at `addr`
/// (other shard), returning to `ret`; `gp` lists the `.lit4` tuning floats
/// loaded (`fN = *(gp - off)`) before the call.
struct ChaseStage {
    addr: u32,
    ret: u32,
    gp: &'static [(usize, u32)],
}

/// The stages of [`default_3_chase_camera_algorithm_update`] after the
/// first (GatherRiderState), up to the constant-argument ones.
const DEFAULT_3_CHASE_STAGES: [ChaseStage; 6] = [
    // ChaseCamera_*(algo, &info): the derived state.
    ChaseStage { addr: 0x0016_2998, ret: 0x0017_6e60, gp: &[] },
    ChaseStage { addr: 0x0016_2a20, ret: 0x0017_6e70, gp: &[(21, 0x6100)] },
    ChaseStage { addr: 0x0016_2b80, ret: 0x0017_6e7c, gp: &[] },
    ChaseStage { addr: 0x0016_2c78, ret: 0x0017_6e88, gp: &[] },
    ChaseStage {
        addr: 0x0016_3010,
        ret: 0x0017_6eb0,
        gp: &[
            (17, 0x60fc),
            (18, 0x60f8),
            (12, 0x60f4),
            (13, 0x60f0),
            (14, 0x60ec),
            (15, 0x60e8),
            (16, 0x60e4),
        ],
    },
    ChaseStage {
        addr: 0x0016_3270,
        ret: 0x0017_6ed0,
        gp: &[(16, 0x60e0), (12, 0x60dc), (13, 0x60d8), (14, 0x60d4), (15, 0x60d0)],
    },
];

/// Load a stage's tuning floats and call it with `(algo, sp)`.
fn chase_stage(c: &mut Cpu, algo: u64, st: &ChaseStage) -> crate::call::Flow {
    let gp = c.g[GP][0] as u32;
    for &(f, off) in st.gp {
        c.lwc1(f, gp.wrapping_sub(off));
    }
    chase_call(c, algo, st.addr, st.ret)
}

/// `addr(algo, &info)` (info at `sp`) returning to `ret`.
fn chase_call(c: &mut Cpu, algo: u64, addr: u32, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = algo;
    call_ptr(c, addr, ret)
}

/// `void Default_3ChaseCameraAlgorithm::Update()` (0x176e10): the default
/// chase camera (tuning set 3): gather the rider's state into a local info
/// block, then run the shared chase-camera stages over it with this
/// camera's tuning (distances, heights, damping, look-ahead, FOV, the
/// collision push and the final smoothing), all from `.lit4` constants and
/// a few literal ones (0, 1, 15, 1.5, 100, 300, 600).
pub fn default_3_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.lwc1(19, gp.wrapping_sub(0x6120));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        let algo = c.g[A0][0];
        c.g[S0][0] = algo;
        for (f, off) in [
            (12, 0x611cu32),
            (13, 0x6118),
            (14, 0x6114),
            (15, 0x6110),
            (16, 0x610c),
            (17, 0x6108),
            (18, 0x6104),
        ] {
            c.lwc1(f, gp.wrapping_sub(off));
        }
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        // GatherRiderState(algo, &info, tuning f12..f19).
        call_ptr(c, 0x0016_2568, 0x0017_6e50)?;
        // f20 = 0 and f21 (a .lit4 tuning value) are reused below.
        c.mtc1(0, 20);
        for st in DEFAULT_3_CHASE_STAGES.iter() {
            chase_stage(c, algo, st)?;
        }

        let gp = c.g[GP][0] as u32;
        c.lwc1(13, gp.wrapping_sub(0x60cc));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.lwc1(15, gp.wrapping_sub(0x60c8));
        c.mov_s(12, 20);
        chase_call(c, algo, 0x0016_33b0, 0x0017_6ef0)?;

        let gp = c.g[GP][0] as u32;
        c.lwc1(18, gp.wrapping_sub(0x60c4));
        c.lwc1(19, gp.wrapping_sub(0x60c0));
        c.mov_s(17, 21);
        c.lwc1(12, gp.wrapping_sub(0x60bc));
        c.mov_s(16, 20);
        for (f, off) in [(13, 0x60b8u32), (14, 0x60b4), (15, 0x60b0)] {
            c.lwc1(f, gp.wrapping_sub(off));
        }
        chase_call(c, algo, 0x0016_3450, 0x0017_6f1c)?;

        for st in [
            ChaseStage {
                addr: 0x0016_2b90,
                ret: 0x0017_6f3c,
                gp: &[(16, 0x60ac), (12, 0x60a8), (13, 0x60a4), (14, 0x60a0), (15, 0x609c)],
            },
            ChaseStage {
                addr: 0x0016_35f8,
                ret: 0x0017_6f58,
                gp: &[(14, 0x6098), (15, 0x6094), (13, 0x6090), (12, 0x608c)],
            },
            ChaseStage { addr: 0x0016_4878, ret: 0x0017_6f68, gp: &[(12, 0x6088)] },
            ChaseStage {
                addr: 0x0016_46a0,
                ret: 0x0017_6f7c,
                gp: &[(12, 0x6084), (13, 0x6080)],
            },
        ] {
            chase_stage(c, algo, &st)?;
        }

        // (algo, &info, 600, 300, 100, f21).
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(AT, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(AT, 13);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 14);
        chase_call(c, algo, 0x0016_41c0, 0x0017_6fa4)?;

        // (algo, &info, 15, 1.5, 1.5).
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 13);
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 12);
        c.mov_s(14, 13);
        chase_call(c, algo, 0x0016_43a8, 0x0017_6fc4)?;

        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimTeeter_v15 (0x342358)

/// The teeter (a damped spring with end stops) of [`anim_teeter_v15`].
mod teeter {
    /// `+0x0` force applied this frame (cleared after).
    pub const FORCE: u32 = 0x0;
    /// `+0x4` position step of this frame.
    pub const STEP: u32 = 0x4;
    /// `+0x8` force gain (negative: the force pushes the other way).
    pub const GAIN: u32 = 0x8;
    /// `+0xc` velocity kept when bouncing off an end stop.
    pub const BOUNCE: u32 = 0xc;
    /// `+0x10`, `+0x14` end stops.
    pub const MIN: u32 = 0x10;
    pub const MAX: u32 = 0x14;
    /// `+0x18` largest step per frame.
    pub const MAX_STEP: u32 = 0x18;
    pub const DAMPING: u32 = 0x1c;
    pub const STIFFNESS: u32 = 0x20;
    /// `+0x24` rest position.
    pub const REST: u32 = 0x24;
    pub const VEL: u32 = 0x28;
    /// `+0x30` position (also the animation object's value), `+0x34`,
    /// `+0x3c` copies of it, `+0x38` the previous one.
    pub const POS: u32 = 0x30;
    pub const POS_OUT: u32 = 0x34;
    pub const PREV: u32 = 0x38;
    pub const POS_COPY: u32 = 0x3c;
    /// `+0x56` flags (bit 0: the end script fired).
    pub const FLAGS: u32 = 0x56;
    /// `gp-0x2b5c/-0x2b58`: the speed below which an end stop stops it.
    pub const GP_STOP_SPEED_MIN: u32 = 0x2b5c;
    pub const GP_STOP_SPEED_MAX: u32 = 0x2b58;
}

const ANIM_OBJECT_FIRE_END_SCRIPT: Callee = Callee {
    addr: 0x0034_eba0,
    f: ee::s11::AnimObject_FireEndScript,
};
/// `World_GetFrameTime()` (0x2d1c70, other shard).
const WORLD_GET_FRAME_TIME: u32 = 0x002d_1c70;

/// Hitting an end stop: the position is the stop; fast enough it bounces
/// (`vel *= bounce`), else it stops (`step = vel = 0`). f3 holds the new
/// velocity, f4 the stop. `stop_cycles`: the stopping branch's first block
/// (2 at the lower stop, 1 at the upper).
fn teeter_end_stop(c: &mut Cpu, s: u32, stop_speed: u32, stop_cycles: u64) {
    use teeter::*;
    c.cycles += 6;
    let gp = c.g[GP][0] as u32;
    c.lwc1(1, gp.wrapping_sub(stop_speed));
    c.abs_s(0, 3);
    c.c_lt_s(1, 0);
    let bounce = c.fcc();
    c.swc1(4, s.wrapping_add(POS));
    if bounce {
        c.cycles += 4;
        c.lwc1(0, s.wrapping_add(BOUNCE));
        c.mul_s(0, 3, 0);
        c.swc1(0, s.wrapping_add(VEL));
    } else {
        c.cycles += stop_cycles;
        c.sw(s.wrapping_add(STEP), 0);
        c.cycles += 1;
        c.sw(s.wrapping_add(VEL), 0);
    }
    c.cycles += 1;
    c.lwc1(0, s.wrapping_add(POS));
}

/// `int AnimTeeter::Update()` (0x342358, v-table slot 15): one step of the
/// teeter spring: `acc = ±gain*force - stiffness*(pos - rest) - damping*vel`,
/// the position moves by `vel*dt` (at most `maxStep`), the velocity by
/// `acc*dt`; at an end stop it bounces or stops. Then the force is cleared
/// and the animation object's end script fires; returns 1 (and sets flag
/// bit 0) when it did.
pub fn anim_teeter_v15(c: &mut Cpu) {
    use teeter::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let s = c.g[A0][0];
        c.g[S0][0] = s;
        let s = s as u32;
        c.lwc1(0, s.wrapping_add(POS));
        c.swc1(0, s.wrapping_add(PREV));
        call_ptr(c, WORLD_GET_FRAME_TIME, 0x0034_2374)?;
        // f8 = dt, f9 = 0, f7 = gain * force (negated for a negative gain).
        c.mov_s(8, 0);
        c.mtc1(0, 9);
        c.lwc1(0, s.wrapping_add(GAIN));
        c.lwc1(1, s.wrapping_add(FORCE));
        c.c_lt_s(0, 9);
        let negative = c.fcc();
        c.mul_s(7, 0, 1);
        if negative {
            c.cycles += 1;
            c.neg_s(7, 7);
        }

        // f3 = acceleration; step = vel*dt clamped to ±maxStep.
        c.cycles += 18;
        c.lwc1(1, s.wrapping_add(REST));
        c.lwc1(3, s.wrapping_add(STIFFNESS));
        c.lwc1(2, s.wrapping_add(POS));
        c.neg_s(3, 3);
        c.lwc1(0, s.wrapping_add(DAMPING));
        c.sub_s(2, 2, 1);
        c.lwc1(4, s.wrapping_add(VEL));
        c.neg_s(0, 0);
        c.lwc1(6, s.wrapping_add(MAX_STEP));
        c.mul_s(5, 4, 8);
        c.mul_s(3, 3, 2);
        c.abs_s(1, 5);
        c.mul_s(0, 0, 4);
        c.c_lt_s(6, 1);
        c.swc1(5, s.wrapping_add(STEP));
        c.add_s(3, 3, 7);
        let too_far = c.fcc();
        c.add_s(3, 3, 0);
        if too_far {
            c.cycles += 4;
            c.c_lt_s(5, 9);
            if c.fcc() {
                c.cycles += 2;
                c.neg_s(0, 6);
                c.swc1(0, s.wrapping_add(STEP));
            } else {
                c.swc1(6, s.wrapping_add(STEP));
            }
        }

        // pos += step, vel += acc*dt; the end stops.
        c.cycles += 12;
        c.lwc1(2, s.wrapping_add(POS));
        c.mul_s(3, 3, 8);
        c.lwc1(0, s.wrapping_add(STEP));
        c.lwc1(1, s.wrapping_add(VEL));
        c.add_s(2, 2, 0);
        c.lwc1(4, s.wrapping_add(MIN));
        c.add_s(3, 1, 3);
        c.c_lt_s(2, 4);
        c.swc1(2, s.wrapping_add(POS));
        c.swc1(2, s.wrapping_add(POS_COPY));
        let below = c.fcc();
        c.swc1(3, s.wrapping_add(VEL));
        if below {
            teeter_end_stop(c, s, GP_STOP_SPEED_MIN, 2);
        } else {
            c.cycles += 5;
            c.lwc1(4, s.wrapping_add(MAX));
            c.c_lt_s(4, 2);
            if c.fcc() {
                teeter_end_stop(c, s, GP_STOP_SPEED_MAX, 1);
            } else {
                c.lwc1(0, s.wrapping_add(POS));
            }
        }

        c.cycles += 6;
        c.g[A0][0] = sx(s.wrapping_add(POS));
        c.sw(s.wrapping_add(FORCE), 0);
        c.swc1(0, s.wrapping_add(POS_OUT));
        ANIM_OBJECT_FIRE_END_SCRIPT.call(c, 0x0034_24a0)?;
        let fired = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if fired {
            c.cycles += 4;
            let flags = c.lhu(s.wrapping_add(FLAGS)) | 1;
            c.g[V1][0] = flags;
            c.sh(s.wrapping_add(FLAGS), flags);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Avalanche_ApplyHistoryData (0x2da028)

/// The avalanche's globals and its group/node layout, as
/// [`avalanche_apply_history_data`] uses them.
mod avalanche {
    /// `gp+0x9c0` the first group, `gp+0x9c4` the loaded history data,
    /// `gp+0x9cc` the data was applied.
    pub const GP_GROUPS: u32 = 0x9c0;
    pub const GP_DATA: u32 = 0x9c4;
    pub const GP_APPLIED: u32 = 0x9cc;
    /// Group: `+0x4` first node, `+0x8` next group, `+0xc` pair count
    /// (`u16`, zeroed after), `+0xe` index count, `+0x10` pairs (`{f32
    /// v, u32 0}`), `+0x110` index pairs (`{u16 b, u16 a}`).
    pub const GROUP_NODES: u32 = 0x4;
    pub const GROUP_NEXT: u32 = 0x8;
    pub const GROUP_PAIRS: u32 = 0xc;
    pub const GROUP_INDICES: u32 = 0xe;
    pub const GROUP_PAIR_ARRAY: u32 = 0x10;
    pub const GROUP_INDEX_ARRAY: u32 = 0x110;
    /// Node: `+0xe0` position, `+0xf3` owns its data, `+0xf4` next node,
    /// `+0xfc` data, `+0x100` data count (float; 10 bytes per entry).
    pub const NODE_POS: u32 = 0xe0;
    pub const NODE_OWNED: u32 = 0xf3;
    pub const NODE_NEXT: u32 = 0xf4;
    pub const NODE_DATA: u32 = 0xfc;
    pub const NODE_COUNT: u32 = 0x100;
    pub const ENTRY_SIZE: u64 = 0xa;
    /// Loop heads (event checks at their back edges).
    pub const HEAD_GROUPS: u32 = 0x002d_a070;
    pub const HEAD_NODES: u32 = 0x002d_a088;
    pub const HEAD_PAIRS: u32 = 0x002d_a120;
    pub const HEAD_INDICES: u32 = 0x002d_a158;
}

/// `cMemMan_free(p)` (0x317e98, other shard).
const CMEMMAN_FREE: u32 = 0x0031_7e98;

/// `addiu reg, reg, n`.
fn bump(c: &mut Cpu, reg: usize, n: u32) -> u32 {
    let v = (c.g[reg][0] as u32).wrapping_add(n);
    c.g[reg][0] = sx(v);
    v
}

/// `void cAvalanche::resolveDataPointers()` (0x2da028, proposed
/// `Avalanche_ApplyHistoryData`): once, when there are groups and loaded
/// history data, walk the data alongside the groups: each node takes its
/// position (w = 1) and points at its slice of the data (freeing data it
/// owned), each group reads its pairs (value, 0) and index pairs; then the
/// data counts as applied.
pub fn avalanche_apply_history_data(c: &mut Cpu) {
    use avalanche::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let groups = c.lw(gp.wrapping_add(GP_GROUPS));
        c.g[A0][0] = groups;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0x60);
        'body: {
            if groups == 0 {
                break 'body;
            }
            c.cycles += 3;
            let data = c.lw(gp.wrapping_add(GP_DATA));
            c.g[V1][0] = data;
            let applied = c.lw(gp.wrapping_add(GP_APPLIED));
            c.g[V0][0] = applied;
            if data == 0 {
                break 'body;
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            if applied != 0 {
                return epilogue(c, frame);
            }
            c.cycles += 4;
            c.g[S0][0] = sx((data as u32).wrapping_add(8));
            c.g[S2][0] = groups;
            c.g[S1][0] = c.lw((groups as u32).wrapping_add(GROUP_NODES));
            loop {
                // Group loop head: the nodes.
                c.cycles += 2;
                if c.g[S1][0] != 0 {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 20);
                    c.g[S3][0] = ENTRY_SIZE;
                    loop {
                        // Node loop head: the position (x, y, z, 1).
                        c.cycles += 13;
                        let p = bump(c, S0, 4);
                        c.lwc1(0, p);
                        c.lwc1(1, p.wrapping_add(4));
                        c.lwc1(2, p.wrapping_add(8));
                        let s = sp(c);
                        c.swc1(0, s);
                        bump(c, S0, 0xc);
                        c.swc1(1, s.wrapping_add(4));
                        c.swc1(2, s.wrapping_add(8));
                        c.swc1(20, s.wrapping_add(0xc));
                        c.lq(V0, s);
                        let node = c.g[S1][0] as u32;
                        let owned = c.lbu(node.wrapping_add(NODE_OWNED));
                        c.g[V1][0] = owned;
                        c.sq(V0, node.wrapping_add(NODE_POS));
                        if owned == 0 {
                            c.cycles += 1;
                        } else {
                            c.cycles += 3;
                            let old = c.lw(node.wrapping_add(NODE_DATA));
                            c.g[A0][0] = old;
                            if old != 0 {
                                c.cycles += 2;
                                call_ptr(c, CMEMMAN_FREE, 0x002d_a0d0)?;
                                c.cycles += 1;
                            }
                        }
                        c.lwc1(0, node.wrapping_add(NODE_COUNT));
                        // The node's data slice; the next node.
                        c.cycles += 8;
                        c.sw(node.wrapping_add(NODE_DATA), c.g[S0][0]);
                        c.cvt_w_s(1, 0);
                        c.mfc1(V0, 1);
                        c.sb(node.wrapping_add(NODE_OWNED), 0);
                        c.g[S1][0] = c.lw(node.wrapping_add(NODE_NEXT));
                        c.mult(0, V1, V0, S3);
                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        if c.g[S1][0] == 0 {
                            c.cycles += 1;
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_NODES);
                        }
                    }
                }
                c.g[V0][0] = c.lhu(c.g[S0][0] as u32);

                // The group's counts and pairs.
                let group = c.g[S2][0] as u32;
                c.cycles += 8;
                c.g[A0][0] = 0;
                bump(c, S0, 2);
                c.sh(group.wrapping_add(GROUP_PAIRS), c.g[V0][0]);
                c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                c.g[V0][0] &= 0xffff;
                bump(c, S0, 2);
                c.sh(group.wrapping_add(GROUP_INDICES), c.g[V1][0]);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V1][0] = sx(group.wrapping_add(GROUP_PAIR_ARRAY + 4));
                    loop {
                        c.cycles += 9;
                        c.lwc1(0, c.g[S0][0] as u32);
                        bump(c, A0, 1);
                        let pair = c.g[V1][0] as u32;
                        c.sw(pair, 0);
                        bump(c, S0, 8);
                        c.swc1(0, pair.wrapping_sub(4));
                        let n = c.lhu(group.wrapping_add(GROUP_PAIRS));
                        let more = ((c.g[A0][0] as i64) < (n as i64)) as u64;
                        c.g[V0][0] = more;
                        bump(c, V1, 8);
                        if more == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_PAIRS);
                        }
                    }
                }

                // The index pairs (stored swapped).
                c.cycles += 3;
                c.g[V0][0] = c.lhu(group.wrapping_add(GROUP_INDICES));
                c.g[A1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = sx(group.wrapping_add(GROUP_INDEX_ARRAY));
                    loop {
                        c.cycles += 11;
                        let at = c.g[A0][0] as u32;
                        c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                        bump(c, A1, 1);
                        bump(c, S0, 2);
                        c.sh(at.wrapping_add(2), c.g[V1][0]);
                        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                        bump(c, S0, 2);
                        c.sh(at, c.g[V0][0]);
                        let n = c.lhu(group.wrapping_add(GROUP_INDICES));
                        let more = ((c.g[A1][0] as i64) < (n as i64)) as u64;
                        c.g[V0][0] = more;
                        bump(c, A0, 4);
                        if more == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_INDICES);
                        }
                    }
                }

                // Next group.
                c.cycles += 4;
                c.sh(group.wrapping_add(GROUP_PAIRS), 0);
                let next = c.lw(group.wrapping_add(GROUP_NEXT));
                c.g[S2][0] = next;
                if next == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    c.sw(gp.wrapping_add(GP_APPLIED), 1);
                    break 'body;
                }
                c.g[S1][0] = c.lw((next as u32).wrapping_add(GROUP_NODES));
                if event_due(c) {
                    deliver_event_here(c, HEAD_GROUPS);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{machine, ptr};
    use crate::difftest::{check, elf_image, fresh, require_paths, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::Cpu;

    /// A 16-byte aligned object in the lower half of the test data area
    /// (clear of the fake-call log at 0x90_0000).
    fn obj(r: &mut Rng) -> u32 {
        DATA + (r.next_u32() & 0x7_fff0)
    }

    /// Rider_TurnAroundIfBackward: braking (no turn), moving forward (no
    /// turn) and moving backward (turn: rebuild the world matrix and toggle
    /// the stance through the real callees).
    #[test]
    fn rider_turn_around_if_backward_matches() {
        use super::turn_around::*;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x4cc0, 0);
            elf_image(c);
            c.pc = 0x0011_4cc0;
            let rider = obj(&mut r);
            c.g[A0][0] = rider as u64;
            // Whole rider area the callees touch: random floats.
            for off in (0..0x800).step_by(4) {
                c.bus.write32(rider + off, r.f32(1.5));
            }
            let brake = match seed % 4 {
                0 => r.f32(1.0),
                1 => 0x8000_0000,
                _ => 0,
            };
            c.bus.write32(rider + BRAKE, brake);
            let vel_scale = if seed % 3 == 0 { 1.0 } else { -1.0 };
            for i in 0..4 {
                let v = r.f32(1.0);
                let fwd = r.f32(1.0);
                c.bus.write32(rider + GROUND_FORWARD + 4 * i, fwd);
                // Velocity roughly along (or against) the ground forward.
                let vf = f32::from_bits(fwd) * 3000.0 * vel_scale + f32::from_bits(v);
                c.bus.write32(rider + VEL + 4 * i, vf.to_bits());
                c.bus.write32(rider + 0x110 + 4 * i, r.f32(1e5));
            }
            let owner = obj(&mut r);
            c.bus.write32(rider + 0x77c, owner);
            c.bus.write32(rider + 0x784, owner);
        };
        check(
            ee::s00::f_00114cc0__generated,
            rider_turn_around_if_backward,
            300,
            &setup,
        );
        // Braking (10+4), not backward (10+15+4), turned (and more).
        require_paths(ee::s00::f_00114cc0__generated, 300, &[14, 29], &setup);
        let mut turned = 0;
        for seed in 0..300 {
            let mut c = fresh();
            setup(&mut c, seed);
            ee::s00::f_00114cc0__generated(&mut c);
            turned += (c.g[V0][0] == 1) as u32;
        }
        assert!(turned > 20, "turned {turned}");
    }

    /// HUD_DrawTextElem: with and without a scale multiplier, position
    /// offset, output rectangle and caller colour; alignment from the
    /// arguments or the element (0x7f); the font's draw callback is a fake
    /// (some seeds unwind there).
    #[test]
    fn hud_draw_text_elem_matches() {
        use super::hud_text::*;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{fill, SP};
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x1b30, 1);
            c.pc = 0x001f_1b30;
            // Renderer: render state and the font draw callback (a fake).
            let renderer = obj(&mut r);
            let state = obj(&mut r);
            let vt = obj(&mut r);
            c.bus.write32(GAME_GP - GP_RENDERER, renderer);
            c.bus.write32(GAME_GP + 0x2a90, renderer);
            c.bus.write32(renderer + RENDER_STATE, state);
            c.bus.write32(state + 0x8, r.next_u32());
            c.bus.write32(renderer + 0x10d8, vt);
            c.bus.write16(vt + 0x280, (r.next_u32() & 0x3f0) as u16);
            c.bus.write32(vt + 0x284, FAKE);
            // Font, element definitions, empty string.
            let font = obj(&mut r);
            for off in (0..0x70).step_by(4) {
                c.bus.write32(font + off, r.f32(4.0));
            }
            if seed % 5 == 0 {
                c.bus.write32(font + 0x28, 0);
                c.bus.write32(font + 0x2c, 0);
            }
            let defs = obj(&mut r);
            fill(c, &mut r, defs, 0x24);
            for i in 0..4 {
                for off in (0..0x20).step_by(4) {
                    c.bus.write32(defs + 0x24 * i + off, r.f32(100.0));
                }
            }
            let text = obj(&mut r);
            c.bus.write8(text, 0);
            let opt = |c: &mut Cpu, r: &mut Rng, on: bool| -> u32 {
                if on {
                    let p = obj(r);
                    for i in 0..4 {
                        c.bus.write32(p + 4 * i, r.f32(2.0));
                    }
                    p
                } else {
                    0
                }
            };
            c.g[A0][0] = obj(&mut r) as u64;
            c.g[A1][0] = font as u64;
            c.g[A2][0] = defs as u64;
            c.g[A3][0] = (r.next_u32() % 4) as u64;
            c.g[T0][0] = text as u64;
            c.g[T1][0] = (r.next_u32() % 3) as u64;
            c.g[T2][0] = opt(c, &mut r, seed % 2 == 0) as u64;
            c.g[T3][0] = opt(c, &mut r, seed % 3 == 0) as u64;
            let color = opt(c, &mut r, seed % 4 < 2);
            let rect = opt(c, &mut r, seed % 5 < 3);
            c.bus.write32(SP, color);
            c.bus.write32(SP + 0x8, rect);
            let align = if seed % 2 == 1 { 0x7f } else { r.next_u32() % 3 };
            let flags = if seed % 7 < 4 { 0x7f } else { r.next_u32() % 3 };
            c.bus.write32(SP + 0x10, align);
            c.bus.write32(SP + 0x18, flags);
        };
        check(ee::s04::f_001f1b30__generated, hud_draw_text_elem, 300, &setup);
    }

    /// SSXAudioSystem_UpdateFade: no fade; rising and falling fades that
    /// do or do not reach the target; random channel masks (all bits).
    #[test]
    fn ssx_audio_system_update_fade_matches() {
        use super::audio_fade::*;
        use crate::difftest::scramble;
        check(
            ee::s07::f_00287c48__generated,
            ssx_audio_system_update_fade,
            300,
            &|c, seed| {
                let mut r = Rng(0x287c_48 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0028_7c48;
                c.g[GP][0] = GAME_GP as u64;
                for i in 0..10 {
                    c.bus.write32(GAME_GP + GP_MIX_LEVELS + 4 * i, r.f32(2.0));
                }
                let s = DATA;
                c.g[A0][0] = s as u64;
                for off in (0x62d0..0x6410).step_by(4) {
                    c.bus.write32(s + off, r.f32(2.0));
                }
                c.bus.write32(s + FADING, if seed % 6 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(
                    s + MASK,
                    if seed % 5 == 0 { 0x3ff } else { r.next_u32() & 0x3ff },
                );
                c.bus.write32(s + RATE, r.f32(0.5));
                c.bus.write32(s + TARGET, r.f32(1.0));
                c.bus.write32(s + CUR, r.f32(1.0));
            },
        );
    }

    /// __fixunsdfsi: zero, NaN, negative, infinity, |x| < 1, x >= 2^32 and
    /// in-range values (with fractions) through the real `__unpack_d`.
    #[test]
    fn fixunsdfsi_matches() {
        use crate::difftest::scramble;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = Rng(0x4126_a0 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.pc = 0x0041_26a0;
            let sign = (r.next_u32() as u64 & 1) << 63;
            let frac = (r.next_u32() as u64) << 20 | r.next_u32() as u64 & 0xf_ffff;
            let x: u64 = match seed % 8 {
                0 => sign,
                1 => 0x7ff0_0000_0000_0000 | frac | 1,
                2 => 0x7ff0_0000_0000_0000 | sign,
                3 => (1u64 << 63) | ((1023 + r.next_u32() % 40) as u64) << 52 | frac,
                4 => ((1000 + r.next_u32() % 23) as u64) << 52 | frac,
                5 => ((1023 + 32 + r.next_u32() % 100) as u64) << 52 | frac,
                _ => ((1023 + r.next_u32() % 32) as u64) << 52 | frac,
            };
            c.g[A0][0] = x;
        };
        check(ee::s15::f_004126a0__generated, fixunsdfsi, 300, &setup);
    }

    /// __ieee754_expf: NaN/inf bit patterns, overflow, underflow, the
    /// three reduction ranges (k from the table, k = ±1, k = 0), tiny x,
    /// and results scaled through 2^-100 (k < -125).
    #[test]
    fn ieee754_expf_matches() {
        use crate::difftest::scramble;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = Rng(0x40e0_48 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            elf_image(c);
            c.pc = 0x0040_e048;
            let sign = (r.next_u32() & 1) << 31;
            let u = r.next_u32() as f32 / u32::MAX as f32;
            let x: u32 = match seed % 12 {
                0 => sign | 0x7f80_0000 | (1 + (r.next_u32() & 0x7f_fffe)),
                1 => sign | 0x7f80_0000,
                2 => (89.0 + u * 1e20).to_bits(),
                3 => (-(104.0 + u * 1e20)).to_bits(),
                4 => (-(87.0 + u * 16.9)).to_bits(),
                5 => sign | (1.04 + u * 87.0).to_bits(),
                6 => sign | (0.35 + u * 0.69).to_bits(),
                7 => sign | (u * 0.346).to_bits(),
                8 => sign | (u * 3.7e-9).to_bits(),
                9 => sign | (r.next_u32() & 0x317f_ffff),
                10 => (88.0 + u * 0.72).to_bits(),
                _ => r.edge_f32(),
            };
            c.f[12] = x;
        };
        check(ee::s15::f_0040e048__generated, ieee754_expf, 300, &setup);
    }

    /// SND_SetVolume fixture: `n` channels in the driver state block (some
    /// sharing a handle group), with short voice lists so the volume and
    /// send callees take their quick paths; a handle that is mostly one of
    /// the channels'.
    fn set_volume_setup(c: &mut Cpu, r: &mut Rng) {
        use super::snd_vol::*;
        let channels = DATA + 0x4_0000;
        let n = r.next_u32() % 12;
        c.bus.write32(STATE + CHANNELS, channels);
        c.bus.write16(STATE + 0x17e, n as u16);
        let volume = r.next_u32() & 0xffff;
        for i in 0..n {
            let ch = channels + i * CHANNEL_SIZE as u32;
            for off in (0..CHANNEL_SIZE as u32).step_by(4) {
                c.bus.write32(ch + off, r.next_u32());
            }
            let h = if r.next_u32() % 5 != 0 {
                (r.next_u32() << 8 | i) & 0x7fff_ffff
            } else {
                r.next_u32()
            };
            c.bus.write32(ch, h);
            c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
            c.bus.write8(ch + 0x69, if r.next_u32() % 4 != 0 { 1 } else { 0 });
            // Voice flags with the "voices listed" bit and no voices.
            c.bus.write16(ch + 0x24, 0x8);
            c.bus.write8(ch + 0x23, 0);
            c.bus.write32(ch + 0x78, 0);
            c.bus.write16(ch + SEND_CHANNEL, (r.next_u32() % n) as u16);
            c.bus.write16(
                ch + SEND,
                [0, 1, 0x40, 0xffff][(r.next_u32() % 4) as usize] as u16,
            );
            let v = if r.next_u32() % 6 == 0 { volume } else { r.next_u32() & 0xffff };
            c.bus.write32(ch + VOLUME, v << 16);
        }
        let h = if n > 0 && r.next_u32() % 5 != 0 {
            c.bus.read32(channels + (r.next_u32() % n) * CHANNEL_SIZE as u32)
        } else {
            r.next_u32()
        };
        c.g[A0][0] = h as i32 as u64;
        c.g[A1][0] = volume as u64;
    }

    crate::difftest::events::driven!(gen_set_volume, ee::s13::f_003b85f0__generated);
    crate::difftest::events::driven!(hand_set_volume, snd_set_volume);

    /// SND_SetVolume: bad handles, single and multi-channel handles, an
    /// unchanged volume (early 0), channels with and without a send; and
    /// with events at the loop's back edge (delivered or unwinding, then
    /// resumed at the loop head).
    #[test]
    fn snd_set_volume_matches() {
        use crate::difftest::events::{check_events, schedule};
        check(ee::s13::f_003b85f0__generated, snd_set_volume, 400, &|c, seed| {
            let mut r = machine(c, seed, 0x85f0, 0);
            c.pc = 0x003b_85f0;
            set_volume_setup(c, &mut r);
        });
        check_events(gen_set_volume, hand_set_volume, 600, &|c, seed| {
            let mut r = machine(c, seed, 0x85f1, 0);
            set_volume_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_85f0)
        });
    }

    /// DynamicParticleModifier_Update: finished; continuous emitter with
    /// and without live particles; one-shot emitter with life left, run
    /// out, or running out now. The instance has no owner and node -1 (the
    /// node matrix is the instance's own), the particle system has no
    /// particle array (its update takes the short path).
    #[test]
    fn dynamic_particle_modifier_update_matches() {
        use super::particle_mod::*;
        check(
            ee::s11::f_00345f90__generated,
            dynamic_particle_modifier_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5f90, 0);
                c.pc = 0x0034_5f90;
                let s = DATA + 0x1000;
                let inst = DATA + 0x2000;
                let skel = DATA + 0x2400;
                for off in (0..0x260).step_by(4) {
                    c.bus.write32(s + off, r.f32(10.0));
                }
                for off in (0..0x90).step_by(4) {
                    c.bus.write32(inst + off, r.f32(4.0));
                }
                c.bus.write32(inst + 0xc, 0);
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(skel + 8, DATA + 0x2800);
                c.bus.write32(s + 0xc, inst);
                c.bus.write32(s + 0x10, u32::MAX);
                c.bus.write32(s + DONE, if seed % 7 == 0 { 1 } else { 0 });
                c.bus.write32(
                    s + EMIT_COUNT,
                    [0, 1, 5, 0xffff_ffff][(seed / 7 % 4) as usize],
                );
                c.bus.write32(s + LIVE, [0, 3, 0xffff_fffe][(r.next_u32() % 3) as usize]);
                // Remaining life: positive, zero or negative.
                let life = match r.next_u32() % 3 {
                    0 => 0,
                    1 => r.f32(1.0) & 0x7fff_ffff,
                    _ => r.f32(1.0) | 0x8000_0000,
                };
                c.bus.write32(s + SYSTEM, life);
                c.bus.write32(s + SYSTEM + 0x174, 0);
                for k in [GP_STEP_LIVE, GP_STEP_EXPIRED, GP_STEP_DYING] {
                    c.bus.write32(GAME_GP - k, r.f32(1.0));
                }
                c.g[A0][0] = s as u64;
            },
        );
    }

    /// Default_3ChaseCameraAlgorithm_Update: a chase camera at `DATA` with
    /// random tuning state, its target (`+0x30`) whose v-table slots are
    /// fakes, the rider behind it and the world's player count (as the
    /// post-race chase camera's test builds them); the game's `.lit4`
    /// constants from the ELF image. The real stages run on it.
    #[test]
    fn default_3_chase_camera_algorithm_update_matches() {
        use crate::difftest::fakes::vtable;
        use crate::difftest::put_vec;
        check(
            ee::s02::f_00176e10__generated,
            default_3_chase_camera_algorithm_update,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x176e10, 8);
                c.pc = 0x0017_6e10;
                let (algo, t, rider, ms) = (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                c.g[A0][0] = sx(algo);
                for i in 0..0x100 {
                    c.bus.write32(algo + 4 * i, r.f32(400.0));
                }
                put_vec(c, algo + 0x390, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                // The five tuning splines (`cCSICubicSpline` at +0x304..+0x334,
                // 12 bytes: word 0 count << 11 | current << 19, +4 end
                // time, +8 segments of 0x14 bytes with the start time at
                // +0x10): one segment spanning every float, so
                // cCSICubicSpline_Evaluate always finds it as the current
                // one (random headers sent its binary search into a loop).
                for (k, off) in [0x304u32, 0x310, 0x31c, 0x328, 0x334].into_iter().enumerate() {
                    let segs = DATA + 0x5000 + 0x40 * k as u32;
                    for i in 0..4 {
                        c.bus.write32(segs + 4 * i, r.f32(4.0));
                    }
                    c.bus.write32(segs + 0x10, 0xff7f_ffff);
                    c.bus.write32(segs + 0x14 + 0x10, 0x7f7f_ffff);
                    c.bus.write32(algo + off, 1 << 11);
                    c.bus.write32(algo + off + 4, 0x7f7f_ffff);
                    c.bus.write32(algo + off + 8, segs);
                }
                // +0x2d0 set: stage 0x1635f8 skips rebuilding the +0x31c
                // spline from state-dependent key times (unsorted on
                // random state).
                c.bus.write32(algo + 0x2d0, 1);
                c.bus.write32(algo + 0x2c4, 0);
                c.bus.write32(algo + 0x30, t);
                let slots: Vec<u32> = (1..0x20).map(|k| 8 * k).collect();
                let vt = vtable(c, &mut r, &slots);
                c.bus.write32(t, vt);
                c.bus.write32(t + 4, rider);
                c.bus.write32(rider + 0x77c, ms);
                c.bus.write32(ms + 0xde0, r.next_u32() % 8);
                let (w, w84, w0c) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
                c.bus.write32(w + 0x84, w84);
                c.bus.write32(w84 + 0xc, w0c);
                c.bus.write32(w0c + 0x78, r.next_u32() % 4);
                c.bus.write32(crate::tick::APPMAN_PTR, 0);
            },
        );
    }

    /// AnimTeeter_v15: positive and negative gain, steps over and under
    /// the limit (both signs), both end stops with bounce or stop, and the
    /// end script firing or not (the script manager has no script table,
    /// as the AnimObject_FireEndScript test builds it).
    #[test]
    fn anim_teeter_v15_matches() {
        use super::teeter::*;
        check(ee::s11::f_00342358__generated, anim_teeter_v15, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x342358, 0);
            c.pc = 0x0034_2358;
            let s = DATA + 0x100;
            let obj = s + POS;
            let list = DATA + 0x1000;
            let man = DATA + 0x2000;
            let world = DATA + 0x3000;
            for off in (0..0x60).step_by(4) {
                c.bus.write32(s + off, r.f32(2.0));
            }
            c.bus.write32(s + MIN, (-1.0f32).to_bits());
            c.bus.write32(s + MAX, 1.0f32.to_bits());
            c.bus.write32(s + MAX_STEP, r.f32(0.5) & 0x7fff_ffff);
            c.bus.write32(s + BOUNCE, r.f32(1.0));
            c.bus.write32(s + POS, r.f32(1.5));
            c.bus.write32(s + VEL, r.f32(if seed % 2 == 0 { 0.2 } else { 30.0 }));
            c.bus.write16(s + FLAGS, r.next_u32() as u16);
            // World frame time.
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x14, if seed % 3 == 0 { r.f32(1.0) } else { (1.0f32 / 60.0).to_bits() });
            c.bus.write32(GAME_GP - GP_STOP_SPEED_MIN, 0.1f32.to_bits());
            c.bus.write32(GAME_GP - GP_STOP_SPEED_MAX, 0.1f32.to_bits());
            // AnimObject_FireEndScript: `list->head == obj + 0x14` decides.
            c.bus.write32(GAME_GP + 0xce8, man);
            c.bus.write32(man + 0x28c, 0);
            c.bus.write32(list + 0x78, 0);
            c.bus.write32(list + 0x88, 0);
            c.bus.write32(obj + 0x2c, list);
            c.bus.write32(
                list + 0xc,
                if seed % 4 < 2 { obj + 0x14 } else { obj + 0x40 },
            );
            c.g[A0][0] = sx(s);
        });
    }

    /// Avalanche_ApplyHistoryData: no groups, no data, already applied;
    /// otherwise 1-3 groups of 0-3 nodes (owning heap data that is freed,
    /// owning none, or not owning) with 0-4 pairs and index pairs each,
    /// read from a history buffer laid out as the function walks it.
    #[test]
    fn avalanche_apply_history_data_matches() {
        use super::avalanche::*;
        use crate::game::scripter::ssxscriptengine::tests::{alloc, heap_init};
        check(
            ee::s09::f_002da028__generated,
            avalanche_apply_history_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2da028, 0);
                let groups_at = DATA + 0x1_0000;
                let nodes_at = DATA + 0x2_0000;
                let data = DATA + 0x3_0000;
                let mut heap = false;
                // The history buffer, written as it is read.
                let mut p = data + 8;
                let n_groups = 1 + r.next_u32() % 3;
                for g in 0..n_groups {
                    let group = groups_at + 0x400 * g;
                    for off in (0..0x200).step_by(4) {
                        c.bus.write32(group + off, r.next_u32());
                    }
                    let next = if g + 1 < n_groups { group + 0x400 } else { 0 };
                    c.bus.write32(group + GROUP_NEXT, next);
                    let n_nodes = r.next_u32() % 4;
                    let mut prev_link = group + GROUP_NODES;
                    for k in 0..n_nodes {
                        let node = nodes_at + 0x1000 * g + 0x200 * k;
                        for off in (0xe0..0x110).step_by(4) {
                            c.bus.write32(node + off, r.next_u32());
                        }
                        c.bus.write32(prev_link, node);
                        prev_link = node + NODE_NEXT;
                        let count = 2 * (r.next_u32() % 3);
                        c.bus.write32(node + NODE_COUNT, (count as f32).to_bits());
                        let owned = r.next_u32() % 3;
                        c.bus.write8(node + NODE_OWNED, (owned != 0) as u8);
                        let old = if owned == 1 {
                            if !heap {
                                heap_init(c);
                                heap = true;
                            }
                            alloc(c, 0x40)
                        } else if owned == 2 {
                            0
                        } else {
                            r.next_u32()
                        };
                        c.bus.write32(node + NODE_DATA, old);
                        c.bus.write32(p, r.next_u32());
                        for i in 0..3 {
                            c.bus.write32(p + 4 + 4 * i, r.f32(1000.0));
                        }
                        p += 16;
                        for i in 0..count * 10 / 4 {
                            c.bus.write32(p + 4 * i, r.next_u32());
                        }
                        p += count * 10;
                    }
                    c.bus.write32(prev_link, 0);
                    let pairs = r.next_u32() % 5;
                    let indices = r.next_u32() % 5;
                    c.bus.write16(p, pairs as u16);
                    c.bus.write16(p + 2, indices as u16);
                    p += 4;
                    for i in 0..pairs {
                        c.bus.write32(p + 8 * i, r.f32(10.0));
                        c.bus.write32(p + 8 * i + 4, r.next_u32());
                    }
                    p += 8 * pairs;
                    for i in 0..indices {
                        c.bus.write32(p + 4 * i, r.next_u32());
                    }
                    p += 4 * indices;
                }
                let groups = if seed % 9 == 0 { 0 } else { groups_at };
                let buffer = if seed % 9 == 1 { 0 } else { data };
                let applied = if seed % 9 == 2 { 1 } else { 0 };
                c.bus.write32(GAME_GP + GP_GROUPS, groups);
                c.bus.write32(GAME_GP + GP_DATA, buffer);
                c.bus.write32(GAME_GP + GP_APPLIED, applied);
                c.pc = 0x002d_a028;
            },
        );
    }
}
