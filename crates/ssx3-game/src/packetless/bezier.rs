//! Bézier terrain without packets (`docs/investigation/n2-capture/bezier.md`):
//! `PS2BezierMan_v18` (0x38b370), the end of the terrain pass, with its
//! emitters (`EmitPatchType1` / `Type2` / `MatrixPass` / `ProjectorPasses`)
//! and the deferred type-1 groups. The patch cache (tessellation, the slot
//! packets `BuildPatchPacket` writes when a slot is filled) and the glitter
//! pass (`RenderPatchFX` → native `DrawPatchFX`) stay guest calls.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::vu0::{self, M};
use super::*;
use crate::ps2f::FpuF32;

const SIN_COS: u32 = 0x0031_be50;
const MARK_USED: u32 = 0x0037_4a88;
const MARK_USED2: u32 = 0x0037_4ae0;
const RELEASE_UNUSED: u32 = 0x0037_4b38;
const INTERN_STATE: u32 = 0x0039_4ed0;
const TESSELLATE: u32 = 0x0037_47a0;
const BUILD_PATCH_PACKET: u32 = 0x0038_c788;
const FIND_BUCKET: u32 = 0x0039_5000;
const COMPUTE_SORT_KEY: u32 = 0x0036_4240;
const APPEND_PACKET: u32 = 0x0036_2978;
const RENDER_PATCH_FX: u32 = 0x0038_d968;
const TEST_PATCH_BOUNDS: u32 = 0x0038_ca08;
/// Per lod, the MSCAL table (8-byte entries: 2/3 draw, 6/7 projector,
/// 8/9 shine, by the entry's flag).
const LOD_TABLES: u32 = 0x0044_b9d0;
const TYPE2_TABLE: u32 = 0x0050_4960;
/// Bias of the shine matrix, the identity the glitter matrix starts from,
/// its three rotation axes.
const SHINE_BIAS: u32 = 0x0050_49c0;
const IDENTITY: u32 = 0x004f_f1a0;
const FX_AXES: u32 = 0x004f_f140;

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// `Math_SinCos(angle, &a, &b)` (as the compiled code calls it: the
/// results through the guest stack).
fn sin_cos(c: &mut Cpu, angle: FpuF32, ret: u32) -> (FpuF32, FpuF32) {
    let sp = c.g[SP][0] as u32;
    let out = sp - 0x40;
    c.g[SP][0] = (sp - 0x80) as i32 as i64 as u64;
    c.f[12] = angle.0;
    guest_call(c, SIN_COS, &[out, out + 4], ret);
    c.g[SP][0] = sp as i32 as i64 as u64;
    (f(c, out), f(c, out + 4))
}

/// `(x - (float)(int)x)`: the fraction the angles take.
fn frac(x: FpuF32) -> FpuF32 {
    x - FpuF32::from_i32(x.to_i32())
}

/// The DeferMan hash of a key (`InternState`, buckets).
pub(super) fn hash8(k: u32) -> u32 {
    let mix = |sh: u32| -> u32 { ((((k as i32 >> sh) as u32) << 5).wrapping_add(k) as i32 >> sh) as u32 };
    ((k << 5).wrapping_add(k) ^ mix(8) ^ mix(16) ^ mix(24)) & 0xff
}

/// The bucket key of a state and a texture word (tex0 | tex1 << 16).
fn tex_key(state: u32, tex: u32) -> u32 {
    state ^ (tex as u16 as i16 as i32 as u32 | ((tex as i32 >> 16) + 2) as u32 * 0x1_0000)
}

/// The bucket of (state, textures, matrix), created if needed.
fn bucket(c: &mut Cpu, dm: u32, state: u32, tex: u32, matrix: u32, rets: (u32, u32)) -> u32 {
    let hash = hash8(tex_key(state, tex));
    let b = guest_call(c, FIND_BUCKET, &[dm, state, tex, hash, matrix], rets.0);
    if b != 0 {
        return b;
    }
    let n = r32(c, dm + 0x57494);
    w32(c, dm + 0x57494, n + 1);
    let b = dm + 0x57498 + 0x20 * n;
    w32(c, b, state);
    w32(c, b + 4, tex);
    w32(c, b + 0xc, 0);
    w32(c, b + 8, matrix);
    w32(c, b + 0x14, 0);
    w32(c, b + 0x18, 0);
    let key = guest_call(c, COMPUTE_SORT_KEY, &[state, tex], rets.1);
    let slot = dm + 0x678a0 + 4 * hash;
    let head = r32(c, slot);
    w32(c, b + 0x1c, head);
    w32(c, slot, b);
    let k = r32(c, dm + 0x67ca4);
    w32(c, dm + 0x67ca8 + 8 * k, key);
    w32(c, dm + 0x67cac + 8 * k, b);
    w32(c, dm + 0x67ca4, k + 1);
    b
}

/// The shine matrix (bm+0x360): rotations by the camera position's
/// fractional x / y, times the bias.
fn shine_matrix(c: &mut Cpu, cam: [FpuF32; 3]) -> M {
    let gp = c.g[GP][0] as u32;
    let (k1, k2, k3) = (f(c, gp + 0x1350), f(c, gp.wrapping_sub(0x26bc)), f(c, gp.wrapping_sub(0x26b8)));
    let ay = frac(cam[1] * k1 * k2) * k3;
    let ax = frac(cam[0] * k1 * k2) * k3;
    let (s1, c1) = sin_cos(c, ay, 0x0038_b58c);
    let (s2, c2) = sin_cos(c, ax, 0x0038_b59c);
    let neg = |v: FpuF32| v.0 ^ 0x8000_0000;
    let one = vu0::ONE;
    let a: M = [[one, 0, 0, 0], [0, s1.0, c1.0, 0], [0, neg(c1), s1.0, 0], [0, 0, 0, one]];
    let b: M = [[s2.0, 0, neg(c2), 0], [0, one, 0, 0], [c2.0, 0, s2.0, 0], [0, 0, 0, one]];
    let cm = vu0::mul_m(&a, &b);
    vu0::mul_m(&qm(c, SHINE_BIAS), &cm)
}

/// The glitter (FX) matrix (bm+0x3e0): the identity rotated about three
/// axes by angles from the camera position.
fn fx_matrix(c: &mut Cpu, cam: [FpuF32; 3]) -> M {
    let gp = c.g[GP][0] as u32;
    let (k4, k2, k3) = (f(c, gp + 0x1254), f(c, gp.wrapping_sub(0x26bc)), f(c, gp.wrapping_sub(0x26b8)));
    let one = FpuF32(vu0::ONE);
    let angles = [frac(cam[1] * k4 * k2) * k3, frac(cam[0] * k4 * k2) * k3, frac(cam[2] * k4 * k2) * k3];
    let rets = [0x0038_b870, 0x0038_b9b8, 0x0038_baf8];
    let mut fx = qm(c, IDENTITY);
    for k in 0..3 {
        let (s, co) = sin_cos(c, angles[k], rets[k]);
        let axis = FX_AXES + 0x10 * k as u32;
        let (x, y, z) = (f(c, axis), f(c, axis + 4), f(c, axis + 8));
        let t = one - co;
        let (tz, tx, ty) = (t * z, t * x, t * y);
        let r: M = [
            [(tx * x + co).0, (ty * x - s * z).0, (tz * x + s * y).0, 0],
            [(tx * y + s * z).0, (ty * y + co).0, (tz * y - s * x).0, 0],
            [(tx * z - s * y).0, (ty * z + s * x).0, (tz * z + co).0, 0],
            [0, 0, 0, vu0::ONE],
        ];
        fx = vu0::mul_m(&fx, &r);
    }
    fx
}

/// The MSCAL byte address of entry `i` of a table (8-byte entries).
fn mscal(c: &mut Cpu, table: u32, i: u32) -> u32 {
    r32(c, table + 8 * i) * 8
}

/// `PS2BezierMan_v18(bm)` (0x38b370).
pub fn end_pass(c: &mut Cpu) {
    let bm = c.g[A0][0] as u32;
    let gp = c.g[GP][0] as u32;
    let gm = r32(c, gp + 0x2a90);
    let cursor = gm + gmf::VIF_CURSOR;
    w32(c, bm + 0x5364, 0);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, 0xffff_fff3, 0);
    state_word(c, st, 0, !0, 0x30);
    let dm = r32(c, gm + gmf::DEFER);
    ensure_view_matrices(c, gm, 0x0038_b408);
    let matrix = r32(c, dm + dmf::MATRIX);
    let (this, func) = model::vmethod(c, gm, 0x11c);
    let rp = guest_call(c, func, &[this], 0x0038_b438);
    let r = qm(c, rp);
    let view = r32(c, gm + 0x274);
    // The camera position: the matrix's 3×3 transposed times −translation.
    let t: M = [[r[0][0], r[1][0], r[2][0], r[0][3]], [r[0][1], r[1][1], r[2][1], r[1][3]], [r[0][2], r[1][2], r[2][2], r[2][3]], r[3]];
    let mut p = vu0::mul(&r[3], &[0xbf80_0000; 4]);
    p[3] = 0;
    let cam = vu0::xform(&t, &p);
    let cam = [FpuF32(cam[0]), FpuF32(cam[1]), FpuF32(cam[2])];
    let shine = shine_matrix(c, cam);
    wm(c, bm + 0x360, &shine);
    let fx = fx_matrix(c, cam);
    wm(c, bm + 0x3e0, &fx);
    for k in 0..4 {
        w32(c, bm + 0x420 + 4 * k, 0x4300_0000);
    }
    for (k, a) in [0x004a_434c, 0x004a_4350, 0x004a_4354].into_iter().enumerate() {
        let v = r32(c, a);
        w32(c, bm + 0x430 + 4 * k as u32, v);
    }
    let st = push_state(c, gm);
    state_word(c, st, 2, 0xffff_fc1f, 0x40);
    state_word(c, st, 1, !3, 0);
    state_word(c, st, 2, 0xe000_03ff, 0);
    state_word(c, st, 1, 0xfe7f_ffff, 0x80_0000);
    state_word(c, st, 1, 0xffcf_ffff, 0);
    state_word(c, st, 1, 0xfff0_0fff, 0x1_4000);
    state_word(c, st, 1, 0xffff_ff83, 4);
    state_word(c, st, 1, 0xffff_f07f, 0x400);
    let cache = bm + 0x10;
    // The queues: (count, entries, entry size, lod).
    let queues = [(bm + 0x458, 0x10, 0), (bm + 0x3c9c, 0x10, 1), (bm + 0x4a60, 0x10, 2), (bm + 0x4d84, 0x14, 2)];
    for (i, &(q, _, lod)) in queues.iter().enumerate() {
        let n = r32(c, q);
        let rets = [0x0038_bd84, 0x0038_bd9c, 0x0038_bdb4, 0x0038_bdcc];
        guest_call(c, if i == 3 { MARK_USED2 } else { MARK_USED }, &[cache, n, q + 4, lod, view], rets[i]);
    }
    if view == 0 {
        guest_call(c, RELEASE_UNUSED, &[cache], 0x0038_bddc);
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0x40);
    state_word(c, st, 3, 0xc000_0000, 0);
    let scissor = r32(c, dm + dmf::SCISSOR) & 0x1f;
    state_word(c, st, 2, !0x1f, scissor);
    let words: Vec<u32> = (0..4).map(|k| r32(c, st + 4 * k)).collect();
    let base = guest_call(c, INTERN_STATE, &[dm, st, hash8(words[0] ^ words[1] ^ words[2] ^ words[3])], 0x0038_bea4);
    // Uniform queues: lod 2, 1, 0.
    let uniform = [(bm + 0x4a60, 2u32, [0x0038_bee8, 0x0038_bf04]), (bm + 0x3c9c, 1, [0x0038_bf70, 0x0038_bf8c]), (bm + 0x458, 0, [0x0038_bfe8, 0x0038_c004])];
    for &(q, lod, rets) in &uniform {
        let n = r32(c, q) as i32;
        for i in 0..n.max(0) as u32 {
            let entry = q + 4 + 0x10 * i;
            let slot = slot_of(c, bm, cache, entry, lod, view, rets);
            emit_type1(c, gm, bm, lod, slot, entry, cursor);
        }
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0x80);
    state_word(c, st, 3, 0xc000_0000, 0);
    let q = bm + 0x4d84;
    let n = r32(c, q) as i32;
    for i in 0..n.max(0) as u32 {
        let entry = q + 4 + 0x14 * i;
        let slot = slot_of(c, bm, cache, entry, 2, view, [0x0038_c0a0, 0x0038_c0bc]);
        emit_type2(c, gm, bm, slot, entry, entry + 0x10, cursor);
    }
    flush_deferred(c, gm, bm, dm, base, matrix, cursor);
    let st = r32(c, gm + gmf::CUR_STATE);
    w32(c, gm + gmf::CUR_STATE, st - 0x14);
    let st = st - 0x14;
    c.bus.write16(st + 0x10, 0xffff);
    c.bus.write16(st + 0x12, 0xffff);
    clear_type(c, gm);
    guest_call(c, RENDER_PATCH_FX, &[bm, cursor], 0x0038_c548);
    ret(c, 2000);
}

/// A queued patch's cache slot, tessellated (and its slot packet built)
/// when it has none.
fn slot_of(c: &mut Cpu, bm: u32, cache: u32, entry: u32, lod: u32, view: u32, rets: [u32; 2]) -> i32 {
    let slot = c.bus.read16(entry + 0xc) as i16 as i32;
    if slot >= 0 {
        return slot;
    }
    let patch = r32(c, entry);
    let normals = lod != 0 && r32(c, patch + 0xc) & 0x60_0000 != 0;
    let s = guest_call(c, TESSELLATE, &[cache, patch, lod, view, normals as u32], rets[0]) as u16 as i16;
    c.bus.write16(entry + 0xc, s as u16);
    if s >= 0 {
        guest_call(c, BUILD_PATCH_PACKET, &[bm, lod, s as i32 as u32], rets[1]);
    }
    s as i32
}

/// The patch's ctx1 clamp bits into the state (w0 bits 2-3); whether
/// there are any.
fn clamp(c: &mut Cpu, gm: u32, flags: u32) -> bool {
    let st = r32(c, gm + gmf::CUR_STATE);
    let (bits, on) = match flags & 0x18_0000 {
        0x10_0000 => (4, true),
        0x8_0000 => (8, true),
        0x18_0000 => {
            state_word(c, st, 0, !0, 0xc);
            return true;
        }
        _ => (0, false),
    };
    state_word(c, st, 0, 0xffff_fff3, bits);
    on
}

/// `PS2BezierMan_EmitPatchType1(bm, lod, slot, entry, &cursor)`
/// (0x38ca70): a uniform patch, drawn now (with its shine / projector
/// passes) or deferred to a type-1 group.
fn emit_type1(c: &mut Cpu, gm: u32, bm: u32, lod: u32, slot: i32, entry: u32, cursor: u32) {
    let gp = c.g[GP][0] as u32;
    let slots = r32(c, bm + 0x28 + 4 * lod);
    let patch = r32(c, entry);
    let pkt = r32(c, bm + 0x44c + 4 * lod).wrapping_add((slot * 0x100) as u32);
    let flags = r32(c, patch + 0xc);
    let mut immediate = false;
    let mut shine = flags & 0x60_0000 != 0;
    if shine {
        if r32(c, gp + 0x13e8) == 0 {
            immediate = true;
        } else {
            shine = false;
        }
    }
    let mut lights = 0;
    if lod == 2 {
        lights = guest_call(c, TEST_PATCH_BOUNDS, &[bm, patch], 0x0038_cb48);
    }
    let mut proj = lights != 0;
    if proj {
        if r32(c, gp + 0x13ec) == 0 {
            immediate = true;
        } else {
            proj = false;
        }
    }
    if clamp(c, gm, flags) {
        immediate = true;
    }
    if !immediate {
        let n = r32(c, bm + 0x5364);
        let e = bm + 0x5368 + 0x10 * n;
        let (t0, t1) = (c.bus.read16(patch + 0x1a0), c.bus.read16(patch + 0x1a2));
        c.bus.write16(e, t0);
        c.bus.write16(e + 2, t1);
        w32(c, e + 4, lod);
        w32(c, e + 8, pkt);
        w32(c, e + 0xc, entry);
        w32(c, bm + 0x5364, n + 1);
        return;
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    let (t0, t1) = (c.bus.read16(patch + 0x1a0), c.bus.read16(patch + 0x1a2));
    c.bus.write16(st + 0x10, t0);
    c.bus.write16(st + 0x12, t1);
    let table = r32(c, LOD_TABLES + 4 * lod);
    let flag = r32(c, entry + 4);
    let e = mscal(c, table, 2 + flag);
    let start = r32(c, cursor);
    w32(c, cursor, start + 0x10);
    record(start, Cmd::Vu(vec![Step::Chain(pkt), Step::Start(e)]));
    add_record(c, gm, start, start, false);
    if shine {
        matrix_pass(c, gm, bm, slots.wrapping_add((slot * 0xc) as u32), patch, lod, table, entry, cursor);
    }
    if proj {
        projector_passes(c, gm, table, entry, lights, cursor);
    }
}

/// `PS2BezierMan_EmitPatchType2(bm, slot, entry, lods, &cursor)`
/// (0x38ce20): a mixed-LOD patch: its edge LODs and coefficients, the
/// lod-2 slot grid, the stitching program.
fn emit_type2(c: &mut Cpu, gm: u32, bm: u32, slot: i32, entry: u32, lods: u32, cursor: u32) {
    let gp = c.g[GP][0] as u32;
    let slots = r32(c, bm + 0x30);
    let patch = r32(c, entry);
    let pkt = r32(c, bm + 0x454).wrapping_add((slot * 0x100) as u32);
    let flags = r32(c, patch + 0xc);
    let shine = flags & 0x60_0000 != 0 && r32(c, gp + 0x13e8) == 0;
    let lights = guest_call(c, TEST_PATCH_BOUNDS, &[bm, patch], 0x0038_cec0);
    let st = r32(c, gm + gmf::CUR_STATE);
    let (t0, t1) = (c.bus.read16(patch + 0x1a0), c.bus.read16(patch + 0x1a2));
    c.bus.write16(st + 0x10, t0);
    let proj = r32(c, gp + 0x13ec) == 0 && lights != 0;
    c.bus.write16(st + 0x12, t1);
    clamp(c, gm, flags);
    let l = r32(c, lods);
    let lodq = q4(std::array::from_fn(|k| (l >> (8 * k)) as u8 as i8 as i32 as u32));
    let flag = r32(c, entry + 4);
    let e = mscal(c, TYPE2_TABLE, 2 + flag);
    let coeffs = qwords(c, patch + 0x40, 16);
    let start = r32(c, cursor);
    w32(c, cursor, start + 0x10);
    record(
        start,
        Cmd::Vu(vec![Step::Write { addr: 0xd0, rel: true, data: vec![lodq] }, Step::Write { addr: 0xc0, rel: true, data: coeffs }, Step::Chain(pkt), Step::Start(e)]),
    );
    add_record(c, gm, start, start, false);
    if shine {
        matrix_pass(c, gm, bm, slots.wrapping_add((slot * 0xc) as u32), patch, 2, TYPE2_TABLE, entry, cursor);
    }
    if proj {
        projector_passes(c, gm, TYPE2_TABLE, entry, lights, cursor);
    }
}

/// `PS2BezierMan_EmitPatchMatrixPass` (0x38d168): the shine pass over the
/// patch just drawn: its normals (a masked UNPACK, raw VIFcodes as the
/// packet has them) and the shine matrix; a continuation record in a
/// pushed state.
#[allow(clippy::too_many_arguments)]
fn matrix_pass(c: &mut Cpu, gm: u32, bm: u32, slot_info: u32, patch: u32, lod: u32, table: u32, entry: u32, cursor: u32) {
    let st = push_state(c, gm);
    let nrm = match lod {
        0 => r32(c, bm + 0x1c4).wrapping_add(r32(c, slot_info + 8).wrapping_mul(0xc0)),
        1 => r32(c, bm + 0x1c8).wrapping_add(r32(c, slot_info + 8).wrapping_mul(0x1b0)),
        2 => r32(c, bm + 0x1cc).wrapping_add(r32(c, slot_info + 8).wrapping_mul(0x300)),
        _ => 0,
    };
    c.bus.write16(st + 0x10, 0xffff);
    let mut t = c.bus.read16(patch + 0x1a4) as i16;
    if t < 0 {
        t = r32(c, gm + 0x10a8) as i16;
    }
    c.bus.write16(st + 0x12, t as u16);
    state_word(c, st, 0, !0, 0x30);
    let blend = if r32(c, patch + 0xc) & 0x40_0000 == 0 { 0x100 } else { 0x880 };
    state_word(c, st, 1, 0xffff_f07f, blend);
    let n = (2 * lod + 4) * (2 * lod + 4);
    // (A REF with address bit 31 reads the scratchpad: a lod-0 slot has no
    // normals, its index -1 points there, and the pass takes whatever is
    // in it.)
    let nrm = if nrm & 0x8000_0000 != 0 { 0x7000_0000 | (nrm & 0x3ff0) } else { nrm };
    let mut words = vec![0x7800_8040 | n << 16];
    for i in 0..(3 * n).div_ceil(4) {
        let v = q(c, nrm.wrapping_add(16 * i));
        words.extend(v4(v));
    }
    words.truncate(1 + 3 * n as usize);
    words.extend([0x2000_0000, 0, 0, 0x6c04_8080]);
    for i in 0..4 {
        words.extend(v4(q(c, bm + 0x360 + 16 * i)));
    }
    let flag = r32(c, entry + 4);
    words.extend([0, 0, 0, 0x1400_0000 | r32(c, table + 0x40 + 8 * flag)]);
    words.resize(words.len().next_multiple_of(4), 0);
    let raw = words.chunks_exact(4).map(|w| q4([w[0], w[1], w[2], w[3]])).collect();
    let start = r32(c, cursor);
    w32(c, cursor, start + 0x10);
    record(start, Cmd::Vu(vec![Step::Mask(0x4040_4040), Step::Raw(raw)]));
    add_record(c, gm, start, start, true);
    pop_state(c, gm);
}

/// `PS2BezierMan_EmitPatchProjectorPasses` (0x38d448): per light touching
/// the patch, its projection matrix and the projector program; a
/// continuation record each.
fn projector_passes(c: &mut Cpu, gm: u32, table: u32, entry: u32, lights: u32, cursor: u32) {
    let flag = r32(c, entry + 4);
    let n = r32(c, gm + 0x13ec);
    for k in 0..n {
        if lights >> (k & 31) & 1 == 0 {
            continue;
        }
        let st = r32(c, gm + gmf::CUR_STATE);
        let tex = r32(c, gm + 0x1484 + 0xa0 * k);
        c.bus.write16(st + 0x12, tex as u16);
        state_word(c, st, 0, 0xffff_ffcf, 0x30);
        let m = qwords(c, gm + 0x1440 + 0xa0 * k, 4);
        let e = mscal(c, table, 6 + flag);
        let start = r32(c, cursor);
        w32(c, cursor, start + 0x10);
        record(start, Cmd::Vu(vec![Step::Mask(0), Step::Write { addr: 0x80, rel: true, data: m }, Step::Start(e)]));
        add_record(c, gm, start, start, true);
    }
}

/// The deferred uniform patches (bm+0x5364): runs with the same textures
/// go into the bucket of (the interned type-1 state, textures, matrix),
/// one group each.
fn flush_deferred(c: &mut Cpu, gm: u32, bm: u32, dm: u32, base: u32, matrix: u32, cursor: u32) {
    let n = r32(c, bm + 0x5364) as i32;
    if n == 0 {
        return;
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    let mut tex = r32(c, bm + 0x5368);
    c.bus.write16(st + 0x10, tex as u16);
    c.bus.write16(st + 0x12, (tex >> 16) as u16);
    state_word(c, st, 0, 0xffff_fff3, 0);
    state_word(c, st, 0, !0, 0x30);
    let mut b = bucket(c, dm, base, tex, matrix, (0x0038_c1b4, 0x0038_c220));
    let mut steps = Vec::new();
    for i in 0..n.max(0) as u32 {
        let e = bm + 0x5368 + 0x10 * i;
        let t = r32(c, e);
        if t != tex {
            group(c, b, std::mem::take(&mut steps), cursor);
            let st = r32(c, gm + gmf::CUR_STATE);
            c.bus.write16(st + 0x10, t as u16);
            c.bus.write16(st + 0x12, (t >> 16) as u16);
            tex = t;
            b = bucket(c, dm, base, t, matrix, (0x0038_c388, 0x0038_c3f0));
        }
        let lod = r32(c, e + 4);
        let pkt = r32(c, e + 8);
        let entry = r32(c, e + 0xc);
        let table = r32(c, LOD_TABLES + 4 * lod);
        let flag = r32(c, entry + 4);
        let ent = mscal(c, table, 2 + flag);
        steps.push(Step::Chain(pkt));
        steps.push(Step::Start(ent));
    }
    group(c, b, steps, cursor);
}

/// One deferred group: an empty packet appended to its bucket.
fn group(c: &mut Cpu, bucket: u32, steps: Vec<Step>, cursor: u32) {
    let start = r32(c, cursor);
    w32(c, cursor, start + 0x10);
    record(start, Cmd::Vu(steps));
    guest_call(c, APPEND_PACKET, &[bucket, start, start], 0x0038_c2d4);
}
