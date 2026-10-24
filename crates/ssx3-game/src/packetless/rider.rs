//! Riders without packets (`docs/investigation/n2-capture/riders.md`):
//! slot 124 `EmitLightingPacket`, slot 101 `SetShadowHeader` (the type-6 /
//! type-7 material parameters, CALLed through `w3`), slot 99
//! `DrawSkinnedModel` and slot 100 `DrawSkinnedModelShadow` (bone batches
//! from the scratchpad, then the model file's prebuilt chains).

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::*;
use crate::ps2f::FpuF32;

const SET_TEX_FUNCTION: u32 = 0x0037_cbc8;
/// `12800.0`: the morph weight scale the program takes with each weight.
const MORPH_SCALE: u32 = 0x4648_0000;

/// A `w3` material parameter at the cursor: its VU writes and start(s),
/// recorded at the address `w3` holds.
fn material_param(c: &mut Cpu, gm: u32, ty: u32, steps: Vec<Step>) {
    let at = empty_packet(c, gm);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, ty);
    state_word(c, st, 3, 0xc000_0000, at & 0x3fff_ffff);
    record(at, Cmd::Vu(steps));
}

/// Slot 124 `EmitLightingPacket(gm)` (0x396b40): type 6; the 10 lighting
/// qwords at `gm+0x6bb0` to VU1 7 (absolute), program 0x1150 (× 255).
pub fn emit_lighting_packet(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    let lights = qwords(c, gm + 0x6bb0, 10);
    material_param(
        c,
        gm,
        0x180,
        vec![
            Step::Cycle(0x0101),
            Step::Write {
                addr: 7,
                rel: false,
                data: lights,
            },
            Step::Start(0x1150),
        ],
    );
    ret(c, 40);
}

/// Slot 101 `SetShadowHeader(gm, matrix, scale, offset)` (0x37ba50): type
/// 7; the light matrix, scale and offset to VU1 0-5 (absolute).
pub fn set_shadow_header(c: &mut Cpu) {
    let (gm, matrix, scale, offset) = (
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] as u32,
        c.g[A3][0] as u32,
    );
    let mut data = qwords(c, matrix, 4);
    data.push(q(c, scale));
    data.push(q(c, offset));
    material_param(
        c,
        gm,
        0x1c0,
        vec![Step::Write {
            addr: 0,
            rel: false,
            data,
        }],
    );
    ret(c, 30);
}

/// One bone batch: the palette's bones from the scratchpad bone base to
/// VU1 0x11 (absolute), their count to 0x3ff, the bone setup (0x80) and
/// the empty program (0x70).
fn bone_batch(c: &mut Cpu, bones: u32, palette: u32, steps: &mut Vec<Step>) {
    let n = r32(c, palette);
    let idx = r32(c, palette + 4);
    let mut data = Vec::with_capacity(4 * n as usize);
    for i in 0..n {
        let b = r32(c, idx + 4 * i);
        data.extend(qwords(c, bones.wrapping_add(b.wrapping_mul(0x40)), 4));
    }
    steps.push(Step::Cycle(0x0101));
    steps.push(Step::Write {
        addr: 0x11,
        rel: false,
        data,
    });
    steps.push(Step::Write {
        addr: 0x3ff,
        rel: false,
        data: vec![q4([n; 4])],
    });
    steps.push(Step::Start(0x80));
    steps.push(Step::Start(0x70));
}

fn fgt(a: u32, b: u32) -> bool {
    FpuF32(a).to_f32() > FpuF32(b).to_f32()
}

/// The morph weight write (VU1 TOPS+0xb4 = {w, 12800, w, 12800}).
fn weight(w: u32) -> Step {
    Step::Write {
        addr: 0xb4,
        rel: true,
        data: vec![q4([w, MORPH_SCALE, w, MORPH_SCALE])],
    }
}

/// Slot 99 `DrawSkinnedModel(gm, model, base, whole)` (0x37a610).
pub fn draw_skinned_model(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    if r32(c, gp + 0x1420) != 0 {
        return ret(c, 6);
    }
    let (gm, model, base, whole) = (
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] as u32,
        c.g[A3][0] as u32 != 0,
    );
    push_state(c, gm);
    let (this, func) = model::vmethod(c, gm, 0x3e4);
    c.f[12] = 0x3f80_0000;
    guest_call(c, func, &[this], 0x0037_a6a8);
    ensure_view_matrices(c, gm, 0x0037_a6bc);
    let mut group = base.wrapping_add(r32(c, model + 0x20));
    let bones = r32(c, gm + 0x13e8);
    let st = r32(c, gm + gmf::CUR_STATE);
    let env = c.bus.read16(group + 8) as i16 as i32;
    let env_tex = r32(c, gm.wrapping_add(0x10b0).wrapping_add((env * 4) as u32));
    c.bus.write16(st + 0x12, env_tex as u16);
    state_word(c, st, 0, !0, 0x30);
    let z = model::sort_value_at(c, gm, bones + 0x30, 0x0037_a714);
    let sort = if z + 0x10 < 0x401 {
        model::sort_value_at(c, gm, bones + 0x30, 0x0037_a788) + 0x10
    } else {
        0x400
    };
    let alt_blend = r32(c, gp + 0x1440) != 0;
    if whole {
        let st = r32(c, gm + gmf::CUR_STATE);
        state_word(c, st, 0, 0xf00f_ffff, 0);
        state_word(c, st, 1, 0xffff_ff83, 0x1c);
        state_word(c, st, 1, 0xffff_f07f, if alt_blend { 0x180 } else { 0x880 });
        state_word(c, st, 1, !3, 2);
        state_word(c, st, 2, 0xe000_03ff, (sort & 0x7_ffff) << 10);
        state_word(c, st, 1, 0xfe7f_ffff, 0x80_0000);
        state_word(c, st, 1, 0xffcf_ffff, 0);
        state_word(c, st, 1, 0xfff0_0fff, 0x1_4000);
    }
    let ngroups = c.bus.read16(model + 0x48) as i16 as i32;
    let palettes = r32(c, model + 0x3c);
    let weights = r32(c, model + 0x40);
    let nmorph = c.bus.read16(model + 0x50) as i16 as i32;
    let mut first_batch = 0u32;
    let mut work = 0u64;
    for _ in 0..ngroups.max(0) {
        let flags = r32(c, group);
        let tex = r32(c, group + 4);
        guest_call(c, SET_TEX_FUNCTION, &[gm, tex, 3], 0x0037_a958);
        let st = r32(c, gm + gmf::CUR_STATE);
        c.bus.write16(st + 0x10, tex as u16);
        if !whole {
            if flags & 0x8000 == 0 {
                state_word(c, st, 1, 0xffff_ff83, 4);
                state_word(c, st, 0, 0xf00f_ffff, 0);
                state_word(c, st, 1, 0xffff_f07f, if alt_blend { 0x180 } else { 0x880 });
                state_word(c, st, 1, !3, 0);
                state_word(c, st, 2, 0xe000_03ff, 0);
                state_word(c, st, 1, 0xfe7f_ffff, 0x80_0000);
                state_word(c, st, 1, 0xffcf_ffff, 0);
                state_word(c, st, 1, 0xfff0_0fff, 0x1_4000);
            } else {
                let alpha = c.bus.read8(group + 0xa) as u32;
                state_word(c, st, 1, 0xffff_ff83, 0x14);
                state_word(c, st, 0, 0xf00f_ffff, alpha << 20);
                state_word(c, st, 1, 0xffff_f07f, 0x600);
                state_word(c, st, 1, !3, 2);
                state_word(c, st, 2, 0xe000_03ff, sort << 10 & 0x1fff_fc00);
                state_word(c, st, 1, 0xfe7f_ffff, 0x80_0000);
                state_word(c, st, 1, 0xffcf_ffff, 0x30_0000);
                state_word(c, st, 1, 0xfff0_0fff, 0x5_c000);
            }
        }
        let nbatch = r32(c, group + 0xc);
        let batches = r32(c, group + 0x10);
        let mut steps = Vec::new();
        let drawn = if flags & 1 != 0 {
            for b in 0..nbatch {
                bone_batch(c, bones, palettes + 8 * (first_batch + b), &mut steps);
                let elems = r32(c, batches + 8 * b);
                steps.push(Step::Chain(r32(c, elems)));
            }
            true
        } else if flags & 0x100 != 0 {
            // Any morph target weighted in?
            let morphs = (0..nmorph.max(0) as u32).any(|j| {
                let w = r32(c, weights + 4 * j);
                fgt(
                    w,
                    r32(c, gp.wrapping_sub(if j == 0 { 0x276c } else { 0x2768 })),
                )
            });
            for b in 0..nbatch {
                bone_batch(c, bones, palettes + 8 * (first_batch + b), &mut steps);
                let elems = r32(c, batches + 8 * b);
                let count = r32(c, batches + 8 * b + 4);
                if !morphs {
                    steps.push(Step::Chain(r32(c, elems)));
                    for k in 1..count {
                        steps.push(Step::Start(0x398));
                        steps.push(Step::Chain(r32(c, elems + 8 * k)));
                    }
                    steps.push(Step::Start(0x398));
                    continue;
                }
                let w0 = r32(c, weights);
                let w0 = if FpuF32(w0).to_f32() < 0.0 { 0 } else { w0 };
                let thr = r32(c, gp.wrapping_sub(0x2764));
                let mut i = 0;
                while i < count {
                    let e0 = elems + 8 * i;
                    let e1 = if count & 1 == 1 && i + 1 >= count {
                        e0
                    } else {
                        e0 + 8
                    };
                    let (mut m0, mut m1) = (r32(c, e0 + 4), r32(c, e1 + 4));
                    steps.push(Step::Chain(r32(c, e0)));
                    steps.push(weight(w0));
                    steps.push(Step::Chain(m0));
                    steps.push(Step::Chain(r32(c, e1)));
                    steps.push(weight(w0));
                    steps.push(Step::Chain(m1));
                    for j in 1..nmorph.max(0) as u32 {
                        m0 += (c.bus.read16(m0) as u32 + 1) * 16;
                        m1 += (c.bus.read16(m1) as u32 + 1) * 16;
                        let wj = r32(c, weights + 4 * j);
                        if fgt(wj, thr) {
                            steps.push(weight(wj));
                            steps.push(Step::Chain(m0));
                            steps.push(weight(wj));
                            steps.push(Step::Chain(m1));
                        }
                    }
                    steps.push(Step::Start(0x398));
                    steps.push(Step::Start(0x398));
                    i += 2;
                }
            }
            true
        } else {
            false
        };
        if drawn {
            work += steps.len() as u64;
            let start = empty_packet(c, gm);
            record(start, Cmd::Vu(steps));
            add_record(c, gm, start, start, false);
        }
        first_batch += nbatch;
        group += 0x14;
    }
    pop_state(c, gm);
    clear_type(c, gm);
    ret(c, 200 + 30 * work);
}

/// Slot 100 `DrawSkinnedModelShadow(gm, model, base, colour)` (0x37b548):
/// one record (a continuation) for the whole call; per rigid group the
/// shadow colour (DIRECT RGBAQ), then per batch the bones and the group's
/// shadow chain.
pub fn draw_skinned_model_shadow(c: &mut Cpu) {
    let (gm, model, base, colour) = (
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] as u32,
        c.g[A3][0] as u32,
    );
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0, 0x30);
    state_word(c, st, 1, 0xffff_ff83, 4);
    state_word(c, st, 1, 0xffff_f07f, 0x580);
    state_word(c, st, 0, 0xf00f_ffff, 0x640_0000);
    state_word(c, st, 1, 0xfe7f_ffff, 0x100_0000);
    state_word(c, st, 1, 0xffcf_ffff, 0x30_0000);
    state_word(c, st, 1, 0xfff0_0fff, 0x1_4000);
    c.bus.write16(st + 0x10, 0xffff);
    c.bus.write16(st + 0x12, 0xffff);
    let bones = r32(c, gm + 0x13e8);
    let palettes = r32(c, model + 0x3c);
    let mut group = base.wrapping_add(r32(c, model + 0x20));
    let ngroups = c.bus.read16(model + 0x48) as i16 as i32;
    let mut first_batch = 0u32;
    let mut steps = Vec::new();
    for _ in 0..ngroups.max(0) {
        let nbatch = r32(c, group + 0xc);
        if r32(c, group) & 1 != 0 {
            let k = FpuF32::from_f32(128.0);
            let ch = |c: &mut Cpu, i: u32| (FpuF32(r32(c, colour + 4 * i)) * k).to_i32() as i64;
            let (a, r, g, b) = (ch(c, 0), ch(c, 1), ch(c, 2), ch(c, 3));
            let rgbaq = (r | g << 8 | b << 16 | a << 24 | 0x3f80_0000_0000_0000) as u64;
            steps.push(Step::Gif(vec![
                0x1000_0000_0000_8001 | 0xe << 64,
                rgbaq as u128 | 1 << 64,
            ]));
            let batches = r32(c, group + 0x10);
            for bi in 0..nbatch as i32 {
                bone_batch(
                    c,
                    bones,
                    palettes + 8 * (first_batch + bi as u32),
                    &mut steps,
                );
                let elems = r32(c, batches + 8 * bi as u32);
                steps.push(Step::Chain(r32(c, elems)));
            }
        }
        first_batch += nbatch;
        group += 0x14;
    }
    let work = steps.len() as u64;
    let start = empty_packet(c, gm);
    record(
        start,
        if steps.is_empty() {
            Cmd::Vu(Vec::new())
        } else {
            Cmd::Vu(steps)
        },
    );
    add_record(c, gm, start, start, true);
    clear_type(c, gm);
    ret(c, 100 + 30 * work);
}
