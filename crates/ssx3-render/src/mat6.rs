//! # Types 6/7 (`9fa47842`) natively: the rider program, bit-exact
//!
//! A readable Rust implementation of the VU1 microprogram with hash
//! `0x9fa478425b3700e5` (`generated/vu/vu_9fa478425b3700e5.rs` is the literal
//! translation; `docs/investigation/phase3-render.md` §1.4 describes it).
//! [`run`] takes the program's VU1 data memory and produces the same XGKICK
//! packets **and** the same data memory as the translated program, bit for
//! bit (checked on real frames by `SSX3_VERIFY_MAT6=1`,
//! `crates/ssx3-ps2/src/mat6_verify.rs`).
//!
//! The program draws the riders (and anything else skinned):
//!
//! | MSCAL | jumps to | what |
//! |---|---|---|
//! | 0x00 | 0x398 | lit skinned strips, one bone (and normal rotation) per vertex |
//! | 0x10 | 0x258 | morph: add scaled position deltas into the batch before it is drawn |
//! | 0x20 | 0xcb8 | shadow silhouette: positions only, flat colour from the EE |
//! | 0x30 | 0xb58 | env pass: rewrite ST of the last drawn batch as a sphere map, kick again |
//! | 0x40 | 0x7d8 | kick the ctx2 GIF tag the EE left at qw 0x392 |
//! | 0x50 | 0x7f8 | lit strips, one bone for the whole batch |
//! | 0x60 | 0x80  | bone setup: M·B, raw rotations, E·R per bone |
//! | 0x70 | -     | empty (ends at once) |
//! | 0x1150 | -   | multiply the 10 lighting coefficients by 255 in place |
//!
//! Both the slot (0x00..0x70) and the target address are accepted.
//!
//! Exactness rules (as in [`crate::mat3`]):
//! - every float op goes through the PS2 float helpers the translated code
//!   uses (`ssx3_vu::vu::fast`, `float`), in the original operand order and
//!   evaluation order (transforms go x, y, z, w; the lighting sum goes
//!   L0, x², y², z², xy, zx, yz, x, y, z), including the accumulator round
//!   trips (`ACC = offset·1`, `cross = OPMULA + 0·0`);
//! - each `q` is the reciprocal of the vertex's own w: the program's
//!   software pipelining (DIV a vertex ahead, MULq 7 pairs later) never makes
//!   a vertex see another vertex's Q, in the translated timing model;
//! - the CLIP flag history is the triangle test: a strip vertex draws (no
//!   ADC) only when it and the two before it are inside the guard band
//!   (`FCAND 0x3ffff` after `FCSET 0x3ffff` at each strip start, so the first
//!   two vertices of a strip never draw a triangle);
//! - memory side effects are the outputs only (no scratch), and the morph
//!   keeps the original load/store interleave (a delta whose target equals
//!   the previous entry's target replaces rather than adds).
//!
//! Nothing crosses calls except VU1 data memory: every entry writes each
//! register before reading it, so [`VuState`] is empty. The double-buffer
//! dependence of `MSCAL 0x50, 0x30, 0x30` is entirely in memory: the env
//! pass (0x30) rewrites the ST of the output a draw left at TOP+0xb4 and
//! reads each vertex's Q from it, and its GIF tag keeps the draw's NLOOP/EOP
//! (only .yzw come from the ctx2 tag at 0x392).
//!
//! Inputs on which the original loops would never end (a strip with fewer
//! vertices than the pipeline primes, zero sub-strips, zero morph entries,
//! an env pass over fewer than 3 vertices), and inputs whose output would
//! overlap the input (never seen), are declined: [`run`] returns `false`
//! without touching memory and the caller runs the translated program.
//!
//! What a wgpu vertex shader needs (see the functions below):
//! - per draw: M (guard clip, qw0-3, folded into the bones by 0x60), the
//!   guard scale/offset (qw4/5), the 10 SH coefficients ×255 (qw7-16);
//! - per bone i: the 4 columns of M·B at 0x11+4i (positions), the raw
//!   rotation columns at 0x75+3i (normals, not renormalised), and E·R at
//!   0x396+3i (the env pass);
//! - per vertex: {u, v (4.12), boneAddr, nrmAddr}, normal (1.15), position
//!   (w from STROW); boneAddr/nrmAddr are absolute VU addresses, so index =
//!   (boneAddr - 0x11) / 4 and (nrmAddr - 0x75) / 3;
//! - every vertex is independent except the ADC bit (strip triangle test,
//!   the vertex and the two before it).

use ssx3_vu::vu::{fast, float};

/// A VU vector register: four PS2 floats (or integers) as raw bits, x..w.
pub type V = [u32; 4];

const ONE: u32 = 0x3f80_0000;
const HALF: u32 = 0x3f00_0000;
/// The I register for the colour clamp (LOI 255).
const I255: u32 = 0x437f_0000;
const ZERO: V = [0; 4];

/// Hash of the program this module implements.
pub const HASH: u64 = 0x9fa4_7842_5b37_00e5;

/// Register state the program carries from one call to the next.
///
/// Empty: no entry reads a VU register before writing it (checked against
/// the translated program on real frames), everything that crosses calls is
/// in data memory. It exists so the API matches [`crate::mat3`].
#[derive(Clone, Debug, Default)]
pub struct VuState {}

/// Entries [`run`] implements natively (MSCAL byte addresses).
pub fn implemented(entry: u32) -> bool {
    matches!(
        entry,
        0x00 | 0x398 | 0x10 | 0x258 | 0x20 | 0xcb8 | 0x30 | 0xb58 | 0x40 | 0x7d8 | 0x50 | 0x7f8 | 0x60 | 0x80 | 0x70 | 0x1150
    )
}

/// Run the program at `entry` with VIF `top` on VU1 data memory `mem`,
/// handing every XGKICK packet to `kick`. Returns `false` (and does nothing)
/// when the entry, or the input, is not handled natively.
pub fn run(
    entry: u32,
    top: u32,
    _itop: u32,
    mem: &mut [u8; 16384],
    _regs: &mut VuState,
    kick: &mut dyn FnMut(&[u128]),
) -> bool {
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    match entry {
        0x00 | 0x398 => draw(&mut m, top, Skin::PerVertex, kick),
        0x50 | 0x7f8 => draw(&mut m, top, Skin::PerBatch, kick),
        0x10 | 0x258 => morph(&mut m, top),
        0x20 | 0xcb8 => shadow(&mut m, top, kick),
        0x30 | 0xb58 => env_pass(&mut m, top, kick),
        0x40 | 0x7d8 => {
            kick(&m.gif_packet(CTX2_TAG));
            true
        }
        0x60 | 0x80 => bone_setup(&mut m),
        0x70 => true,
        0x1150 => {
            light_scale(&mut m);
            true
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

/// VU1 data memory: 1024 qwords, addressed by qword index (wrapping).
struct Mem<'a>(&'a mut [u8; 16384]);

impl Mem<'_> {
    fn off(a: u16) -> usize {
        (a as usize & 0x3ff) * 16
    }
    fn ld(&self, a: u16) -> V {
        let o = Self::off(a);
        std::array::from_fn(|l| u32::from_le_bytes(self.0[o + 4 * l..o + 4 * l + 4].try_into().unwrap()))
    }
    /// Store the lanes of `v` selected by `mask` (bit 3 = x .. bit 0 = w).
    fn st_mask(&mut self, a: u16, v: V, mask: u8) {
        let o = Self::off(a);
        for (l, x) in v.iter().enumerate() {
            if mask & (8 >> l) != 0 {
                self.0[o + 4 * l..o + 4 * l + 4].copy_from_slice(&x.to_le_bytes());
            }
        }
    }
    fn st(&mut self, a: u16, v: V) {
        self.st_mask(a, v, 0b1111);
    }
    /// ILW: the low 16 bits of one lane.
    fn ilw(&self, a: u16, lane: usize) -> u16 {
        self.ld(a)[lane] as u16
    }
    /// The GIF packet XGKICK sends from qword `a`: tags up to EOP (as
    /// `Vu::gif_packet`).
    fn gif_packet(&self, a: u16) -> Vec<u128> {
        let n = 1024u32;
        let mut q = a as u32 & (n - 1);
        let mut out = Vec::new();
        let read = |q: u32| -> u128 { u128::from_le_bytes(self.0[q as usize * 16..q as usize * 16 + 16].try_into().unwrap()) };
        loop {
            let tag = read(q);
            out.push(tag);
            q = (q + 1) & (n - 1);
            let lo = tag as u64;
            let nloop = (lo & 0x7fff) as u32;
            let eop = (lo >> 15) & 1 != 0;
            let flg = (lo >> 58) & 3;
            let nreg = match ((lo >> 60) & 0xf) as u32 {
                0 => 16,
                r => r,
            };
            let body = match flg {
                0 => nloop * nreg,
                1 => (nloop * nreg).div_ceil(2),
                _ => nloop,
            };
            for _ in 0..body {
                if out.len() as u32 >= n {
                    break;
                }
                out.push(read(q));
                q = (q + 1) & (n - 1);
            }
            if eop || out.len() as u32 >= n {
                break;
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// PS2 float vector ops (the translated code's helpers)
// ---------------------------------------------------------------------------

fn mul(a: V, b: V) -> V {
    fast::mul4::<0b1111, true, true>(&a, &b).0
}
/// `acc + a * b`.
fn madd(acc: V, a: V, b: V) -> V {
    fast::madd4::<0b1111, true, true, true>(&acc, &a, &b).0
}
fn max(a: V, b: V) -> V {
    fast::max4(&a, &b)
}
fn min(a: V, b: V) -> V {
    fast::min4(&a, &b)
}
fn ftoi0(a: V) -> V {
    fast::ftoi4::<0>(&a)
}
fn ftoi4(a: V) -> V {
    fast::ftoi4::<4>(&a)
}
fn itof12(a: V) -> V {
    fast::itof4::<12>(&a)
}
fn itof15(a: V) -> V {
    fast::itof4::<15>(&a)
}
fn bc(x: u32) -> V {
    [x; 4]
}
/// DIV: `a / b` as the DIV unit computes it.
fn div(a: u32, b: u32) -> u32 {
    float::div(a, b).value
}

/// `cols · (p.x, p.y, p.z, w)` as MULAx / MADDAy / MADDAz / MADDw.
fn xform(cols: &[V; 4], p: V, w: u32) -> V {
    let acc = mul(cols[0], bc(p[0]));
    let acc = madd(acc, cols[1], bc(p[1]));
    let acc = madd(acc, cols[2], bc(p[2]));
    madd(acc, cols[3], bc(w))
}

/// `cols · (v.x, v.y, v.z)` as MULAx / MADDAy / MADDz (3 columns).
fn xform3(cols: &[V; 3], v: V) -> V {
    let acc = mul(cols[0], bc(v[0]));
    let acc = madd(acc, cols[1], bc(v[1]));
    madd(acc, cols[2], bc(v[2]))
}

// ---------------------------------------------------------------------------
// Memory layout (qword addresses)
// ---------------------------------------------------------------------------

/// M: model → guard-clip matrix columns (folded into the bones by 0x60).
const MATRIX: u16 = 0;
/// Guard scale and offset (.xyz).
const GUARD_SCALE: u16 = 4;
const GUARD_OFFSET: u16 = 5;
/// The 10 SH lighting coefficients L0..L9 (qw 7-16).
const LIGHTS: u16 = 7;
/// Bones: 4 columns each, M·B after 0x60 (raw B before).
const BONES: u16 = 0x11;
/// Raw bone rotations (3 columns each), the normal matrices.
const RAW_ROT: u16 = 0x75;
/// The ctx2 strip GIF tag (env pass, kick 0x40).
const CTX2_TAG: u16 = 0x392;
/// E = bias{0.5, -0.5} × M (3 columns), and E·R per bone (3 columns each).
const ENV_MATRIX: u16 = 0x393;
const ENV_ROT: u16 = 0x396;
/// .x: the bone count for 0x60.
const BONE_COUNT: u16 = 0x3ff;
/// Output (GIF tag + vertices) offset from TOP.
const OUT: u16 = 0xb4;
/// Morph header at TOP+0xb4 (.x weight, .y scale), count at TOP+0xb5.x,
/// entries {delta xyz (1.15), target offset .w} from TOP+0xb6.
const MORPH: u16 = 0xb4;

/// The batch header at TOP+1: {nSub, boneAddr, nrmAddr, total}.
struct Batch {
    /// Address of the sub-strip counts (TOP+2).
    subs: u16,
    n_sub: u16,
    /// Address of the first vertex (TOP+2+nSub).
    verts: u16,
}

impl Batch {
    fn read(m: &Mem, top: u16) -> Self {
        let n_sub = m.ilw(top.wrapping_add(1), 0);
        let subs = top.wrapping_add(2);
        Batch { subs, n_sub, verts: subs.wrapping_add(n_sub) }
    }
    /// The sub-strip vertex counts, or None when the original loops would
    /// not end (no sub-strips, a strip shorter than `min`) or the output
    /// (`qw_per_vertex` per vertex after the tag at TOP+0xb4) would overlap
    /// the input (`in_per_vertex` qwords per vertex).
    fn counts(&self, m: &Mem, min: u16, in_per_vertex: u32, qw_per_vertex: u32) -> Option<Vec<u16>> {
        if self.n_sub == 0 || self.n_sub > OUT {
            return None;
        }
        let counts: Vec<u16> = (0..self.n_sub).map(|i| m.ilw(self.subs.wrapping_add(i), 0)).collect();
        if counts.iter().any(|&c| c < min || c > 0x3ff) {
            return None;
        }
        let total: u32 = counts.iter().map(|&c| c as u32).sum();
        let input = 2 + self.n_sub as u32 + in_per_vertex * total;
        let output = 1 + qw_per_vertex * total;
        if input > OUT as u32 || OUT as u32 + output > 1024 {
            return None;
        }
        Some(counts)
    }
}

// ---------------------------------------------------------------------------
// 0x00 / 0x50: lit strips
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Skin {
    /// 0x00 (0x398): each vertex names its bone (.z) and normal rotation
    /// (.w) in its UV qword.
    PerVertex,
    /// 0x50 (0x7f8): one bone and rotation for the batch (header .y/.z).
    PerBatch,
}

/// Per-draw constants.
struct Lit {
    lights: [V; 10],
    scale: V,
    offset: V,
}

impl Lit {
    fn read(m: &Mem) -> Self {
        Lit {
            lights: std::array::from_fn(|i| m.ld(LIGHTS + i as u16)),
            scale: m.ld(GUARD_SCALE),
            offset: m.ld(GUARD_OFFSET),
        }
    }

    /// The vertex colour before clamping: the SH sum (coefficients already
    /// ×255 by 0x1150) for world normal `n`.
    fn sh(&self, n: V) -> V {
        let l = &self.lights;
        let sq = mul(n, n);
        // OPMULA n, n then MADD.xyz + vf0·vf0: (yz, zx, xy).
        let cross = madd(mul([n[1], n[2], n[0], 0], [n[2], n[0], n[1], 0]), ZERO, ZERO);
        let mut acc = mul(l[0], bc(ONE));
        acc = madd(acc, l[1], bc(sq[0]));
        acc = madd(acc, l[2], bc(sq[1]));
        acc = madd(acc, l[3], bc(sq[2]));
        acc = madd(acc, l[4], bc(cross[2]));
        acc = madd(acc, l[5], bc(cross[1]));
        acc = madd(acc, l[6], bc(cross[0]));
        acc = madd(acc, l[7], bc(n[0]));
        acc = madd(acc, l[8], bc(n[1]));
        madd(acc, l[9], bc(n[2]))
    }

    /// One strip vertex: `uv` raw (4.12 in .xy), `normal` raw (1.15),
    /// `clip` = bone · position. Returns {STQ, RGBA, XYZ (w unset)} and the
    /// CLIP judgement bits.
    fn vertex(&self, uv: V, normal: V, rot: &[V; 3], clip: V) -> ([V; 3], u32) {
        let n = xform3(rot, itof15(normal));
        let col = min(max(self.sh(n), ZERO), bc(I255));
        let q = div(ONE, clip[3]);
        let proj = mul(clip, bc(q));
        let judged = fast::clip(proj, proj[3]);
        let st = itof12(uv);
        let stq = mul([st[0], st[1], ONE, ONE], bc(q));
        let screen = madd(mul(self.offset, bc(ONE)), proj, self.scale);
        ([stq, ftoi0(col), ftoi4(screen)], judged)
    }
}

/// XYZ .w after `FCAND 0x3ffff; IADDIU +0x7fff; MFIR.w`: 0xffff8000 (ADC,
/// no triangle) or 0x7fff.
fn adc_word(adc: bool) -> u32 {
    (adc as u16).wrapping_add(0x7fff) as i16 as i32 as u32
}

/// Input at TOP: qw0 GIF tag, qw1 {nSub, boneAddr, nrmAddr, total}, nSub
/// strip counts (.x), then 3 qw per vertex: {u, v, boneAddr, nrmAddr} (16-bit
/// ints), normal (1.15), position (w = 1 from STROW). Output at TOP+0xb4: the
/// tag, then {STQ, RGBA, XYZ2} per vertex, kicked once.
fn draw(m: &mut Mem, top: u16, skin: Skin, kick: &mut dyn FnMut(&[u128])) -> bool {
    let batch = Batch::read(m, top);
    // The per-vertex program primes two vertices before its loop test, the
    // per-batch one a single vertex.
    let min = if skin == Skin::PerVertex { 3 } else { 2 };
    let Some(counts) = batch.counts(m, min, 3, 3) else {
        return false;
    };
    let lit = Lit::read(m);
    let out = top.wrapping_add(OUT);
    m.st(out, m.ld(top));
    let header = m.ld(top.wrapping_add(1));
    let (batch_bone, batch_rot) = (header[1] as u16, header[2] as u16);
    let mut src = batch.verts;
    let mut dst = out.wrapping_add(1);
    for n in counts {
        // The CLIP history: FCSET 0x3ffff at each strip start.
        let mut hist: u32 = 0x3_ffff;
        for _ in 0..n {
            let uv = m.ld(src);
            let (bone, rot) = match skin {
                Skin::PerVertex => (uv[2] as u16, uv[3] as u16),
                Skin::PerBatch => (batch_bone, batch_rot),
            };
            let cols: [V; 4] = std::array::from_fn(|i| m.ld(bone.wrapping_add(i as u16)));
            let rot: [V; 3] = std::array::from_fn(|i| m.ld(rot.wrapping_add(i as u16)));
            let p = m.ld(src.wrapping_add(2));
            let clip = xform(&cols, p, p[3]);
            let ([stq, rgba, xyz], judged) = lit.vertex(uv, m.ld(src.wrapping_add(1)), &rot, clip);
            hist = ((hist << 6) | judged) & 0xff_ffff;
            m.st(dst, stq);
            m.st(dst.wrapping_add(1), rgba);
            m.st(dst.wrapping_add(2), [xyz[0], xyz[1], xyz[2], adc_word(hist & 0x3_ffff != 0)]);
            src = src.wrapping_add(3);
            dst = dst.wrapping_add(3);
        }
    }
    kick(&m.gif_packet(out));
    true
}

// ---------------------------------------------------------------------------
// 0x30: env pass
// ---------------------------------------------------------------------------

/// Re-kick the batch the last draw left at TOP+0xb4 with the ctx2 tag
/// (.yzw from 0x392; .x, NLOOP/EOP, stays the draw's) and ST replaced by
/// `((E·R)[nrm] · n).xy + 0.5`, times the vertex's Q (read back from the
/// output STQ.w). RGBA, XYZ and Q are reused as they are.
fn env_pass(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    let header = m.ld(top.wrapping_add(1));
    let (n_sub, total) = (header[0] as u16, header[3] as u16);
    // The loop primes two vertices before its first test.
    let input = 2 + n_sub as u32 + 3 * total as u32;
    if total < 3 || input > OUT as u32 || OUT as u32 + 1 + 3 * total as u32 > 1024 {
        return false;
    }
    let out = top.wrapping_add(OUT);
    m.st_mask(out, m.ld(CTX2_TAG), 0b0111);
    let verts = top.wrapping_add(2).wrapping_add(n_sub);
    for k in 0..total {
        let v = verts.wrapping_add(3 * k);
        let rot = m.ilw(v, 3);
        // nrmAddr points at the raw rotation; E·R sits at the same offset
        // from ENV_ROT.
        let er = rot.wrapping_add(ENV_ROT.wrapping_sub(RAW_ROT));
        let er: [V; 3] = std::array::from_fn(|i| m.ld(er.wrapping_add(i as u16)));
        let st = madd(xform3(&er, itof15(m.ld(v.wrapping_add(1)))), bc(ONE), bc(HALF));
        let stq = out.wrapping_add(1 + 3 * k);
        let q = m.ld(stq)[3];
        m.st_mask(stq, mul(st, bc(q)), 0b1100);
    }
    kick(&m.gif_packet(out));
    true
}

// ---------------------------------------------------------------------------
// 0x20: shadow silhouette
// ---------------------------------------------------------------------------

/// Input at TOP: qw0 GIF tag (XYZ2 only), qw1.x nSub, nSub strip counts,
/// then 1 qw per vertex {x, y, z (f32), boneAddr}. Output at TOP+0xb4: the
/// tag, then XYZ2 per vertex (w = 0, or 0xffff = ADC for the first two of
/// each strip). No clip test: the light projection keeps it on screen.
fn shadow(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    let batch = Batch::read(m, top);
    // The pipeline primes three vertices before its first test.
    let Some(counts) = batch.counts(m, 3, 1, 1) else {
        return false;
    };
    let (scale, offset) = (m.ld(GUARD_SCALE), m.ld(GUARD_OFFSET));
    let out = top.wrapping_add(OUT);
    m.st(out, m.ld(top));
    let mut src = batch.verts;
    let mut dst = out.wrapping_add(1);
    for n in counts {
        let first = dst;
        for _ in 0..n {
            let p = m.ld(src);
            let bone = p[3] as u16;
            let cols: [V; 4] = std::array::from_fn(|i| m.ld(bone.wrapping_add(i as u16)));
            let c = xform(&cols, p, ONE);
            let q = div(ONE, c[3]);
            // MAX.xyz vf6, vf7, vf7 (a copy), then MULq.xyz.
            let proj = mul(max(c, c), bc(q));
            let s = ftoi4(madd(mul(offset, bc(ONE)), proj, scale));
            m.st(dst, [s[0], s[1], s[2], 0]);
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
        }
        // ISW.w vi1 (= -1): no triangle for the first two vertices.
        m.st_mask(first, bc(0xffff), 0b0001);
        m.st_mask(first.wrapping_add(1), bc(0xffff), 0b0001);
    }
    kick(&m.gif_packet(out));
    true
}

// ---------------------------------------------------------------------------
// 0x10: morph
// ---------------------------------------------------------------------------

/// `pos.xyz += itof15(delta) · (weight · scale)` for each entry, into the
/// batch at TOP before it is drawn. Header at TOP+0xb4 (.x weight, .y
/// scale), count at TOP+0xb5.x, entries from TOP+0xb6: {delta xyz (1.15),
/// target}, target & 0xff = qword offset of the position from the first
/// vertex's position.
fn morph(m: &mut Mem, top: u16) -> bool {
    let count = m.ilw(top.wrapping_add(MORPH + 1), 0);
    if count == 0 || count > 0x3ff {
        return false;
    }
    let h = m.ld(top.wrapping_add(MORPH));
    let w = mul(h, bc(h[1]))[0];
    let first_pos = top.wrapping_add(m.ilw(top.wrapping_add(1), 0)).wrapping_add(4);
    let entries = top.wrapping_add(MORPH + 2);
    // The original loads entry k's target before it stores entry k-1's
    // result: keep one result pending.
    let mut pending: Option<(u16, V)> = None;
    for k in 0..count {
        let e = m.ld(entries.wrapping_add(k));
        let target = first_pos.wrapping_add(e[3] as u16 & 0xff);
        let p = m.ld(target);
        if let Some((a, v)) = pending.take() {
            m.st_mask(a, v, 0b1110);
        }
        pending = Some((target, madd(mul(p, bc(ONE)), itof15(e), bc(w))));
    }
    if let Some((a, v)) = pending {
        m.st_mask(a, v, 0b1110);
    }
    true
}

// ---------------------------------------------------------------------------
// 0x60: bone setup
// ---------------------------------------------------------------------------

/// For each bone i < count (qw 0x3ff.x; at least 1): B (4 columns at
/// 0x11+4i) becomes M·B in place, B's 3 rotation columns are copied to
/// 0x75+3i and E·R (E at 0x393) goes to 0x396+3i.
fn bone_setup(m: &mut Mem) -> bool {
    let count = m.ilw(BONE_COUNT, 0) as i16;
    // A do-while: 0 or negative counts still do one bone. More than 25
    // would make the three tables overlap (never seen).
    let n = count.max(1) as u16;
    if n > 25 {
        return false;
    }
    let mat: [V; 4] = std::array::from_fn(|i| m.ld(MATRIX + i as u16));
    let env: [V; 3] = std::array::from_fn(|i| m.ld(ENV_MATRIX + i as u16));
    for i in 0..n {
        let b = BONES + 4 * i;
        let cols: [V; 4] = std::array::from_fn(|j| m.ld(b + j as u16));
        for (j, c) in cols.iter().enumerate() {
            m.st(b + j as u16, xform(&mat, *c, c[3]));
        }
        for (j, c) in cols[..3].iter().enumerate() {
            m.st(RAW_ROT + 3 * i + j as u16, *c);
            m.st(ENV_ROT + 3 * i + j as u16, xform3(&env, *c));
        }
    }
    true
}

// ---------------------------------------------------------------------------
// 0x1150: lighting coefficients × 255
// ---------------------------------------------------------------------------

fn light_scale(m: &mut Mem) {
    for a in LIGHTS..LIGHTS + 10 {
        m.st(a, mul(m.ld(a), bc(I255)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ssx3_vu::vu::capture::Record;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn light_scale_multiplies_by_255() {
        let mut mem = Box::new([0u8; 16384]);
        Mem(&mut mem).st(LIGHTS, [f(1.0), f(0.5), f(-2.0), f(0.0)]);
        Mem(&mut mem).st(LIGHTS + 9, [f(0.25), 0, 0, f(1.0)]);
        assert!(run(0x1150, 0, 0, &mut mem, &mut VuState::default(), &mut |_| panic!("no kick")));
        let m = Mem(&mut mem);
        assert_eq!(m.ld(LIGHTS), [f(255.0), f(127.5), f(-510.0), 0]);
        assert_eq!(m.ld(LIGHTS + 9), [f(63.75), 0, 0, f(255.0)]);
    }

    /// A hand-made one-strip draw: identity bone and rotation, constant
    /// light (L0 only), three vertices inside the guard band.
    fn draw_input() -> (Box<[u8; 16384]>, u16) {
        let mut mem = Box::new([0u8; 16384]);
        let mut m = Mem(&mut mem);
        for i in 0..4u16 {
            let mut c = [0; 4];
            c[i as usize] = f(1.0);
            m.st(BONES + i, c);
            if i < 3 {
                m.st(RAW_ROT + i, c);
            }
        }
        m.st(GUARD_SCALE, [f(320.0), f(-224.0), f(-8388467.5), 0]);
        m.st(GUARD_OFFSET, [f(2368.0), f(2272.0), f(8388467.5), 0]);
        m.st(LIGHTS, [f(100.0), f(300.0), f(-5.0), f(128.0)]);
        let top = 0x20u16;
        m.st(top, [3 | 0x8000, 3 << 28, 0x412, 0]);
        m.st(top + 1, [1, 0, 0, 3]);
        m.st(top + 2, [3, 0, 0, 0]);
        let verts = [(0.0f32, 0.5f32), (0.5, -0.5), (-0.5, -0.5)];
        for (k, (x, y)) in verts.into_iter().enumerate() {
            let a = top + 3 + 3 * k as u16;
            m.st(a, [0x800, 0x1000, BONES as u32, RAW_ROT as u32]);
            m.st(a + 1, [0, 0, 0x7fff, 0]);
            m.st(a + 2, [f(x), f(y), f(0.5), f(1.0)]);
        }
        (mem, top)
    }

    #[test]
    fn draw_one_strip() {
        let (mut mem, top) = draw_input();
        let mut kicks = Vec::new();
        assert!(run(0x398, top as u32, 0, &mut mem, &mut VuState::default(), &mut |p| kicks.push(p.to_vec())));
        assert_eq!(kicks.len(), 1);
        let k = &kicks[0];
        assert_eq!(k.len(), 10);
        let lane = |v: u128, l: u32| (v >> (32 * l)) as u32;
        // ADC on the first two vertices only.
        assert_eq!((lane(k[3], 3), lane(k[6], 3), lane(k[9], 3)), (0xffff_8000, 0xffff_8000, 0x7fff));
        // STQ = (0.5, 1.0, 1, 1) · q with q = 1.
        assert_eq!((lane(k[1], 0), lane(k[1], 1), lane(k[1], 3)), (f(0.5), f(1.0), f(1.0)));
        // RGBA: L0 clamped to 0..255.
        assert_eq!((lane(k[2], 0), lane(k[2], 1), lane(k[2], 2), lane(k[2], 3)), (100, 255, 0, 128));
        // XY of vertex 0: (0 · 320 + 2368, 0.5 · -224 + 2272) in 12.4.
        assert_eq!((lane(k[3], 0), lane(k[3], 1)), (2368 * 16, 2160 * 16));
        // The same strip through the per-batch entry (bone/rotation from the
        // header) gives the same packet.
        let (mut mem2, top) = draw_input();
        Mem(&mut mem2).st(top + 1, [1, BONES as u32, RAW_ROT as u32, 3]);
        let mut kicks2 = Vec::new();
        assert!(run(0x50, top as u32, 0, &mut mem2, &mut VuState::default(), &mut |p| kicks2.push(p.to_vec())));
        assert_eq!(kicks2[0][1..], kicks[0][1..]);
    }

    #[test]
    fn short_strips_are_declined() {
        let (mut mem, top) = draw_input();
        Mem(&mut mem).st(top + 2, [2, 0, 0, 0]);
        let before = mem.clone();
        assert!(!run(0x398, top as u32, 0, &mut mem, &mut VuState::default(), &mut |_| {}));
        assert_eq!(mem[..], before[..]);
    }

    /// Replays VU captures (`SSX3_VU_CAPTURE` files, comma-separated in
    /// `MAT6_CAPTURES`) of this program: every implemented call must give
    /// the translated program's kicks and data memory.
    #[test]
    fn captures() {
        let Ok(paths) = std::env::var("MAT6_CAPTURES") else {
            eprintln!("MAT6_CAPTURES not set; skipping");
            return;
        };
        let mut stats: std::collections::BTreeMap<u32, (u64, u64, u64)> = Default::default();
        let mut shown = 0;
        for path in paths.split(',') {
            let bytes = std::fs::read(path).unwrap();
            for r in Record::read_all(&bytes) {
                if r.hash != HASH || r.unit != 1 {
                    continue;
                }
                let e = stats.entry(r.pc).or_default();
                match check(&r, &mut shown) {
                    Some(true) => e.0 += 1,
                    Some(false) => e.1 += 1,
                    None => e.2 += 1,
                }
            }
        }
        for (pc, (ok, bad, ni)) in &stats {
            eprintln!("entry {pc:#06x}: ok {ok} mismatch {bad} not-implemented {ni}");
        }
        assert!(stats.values().all(|s| s.1 == 0));
    }

    /// Captured draw inputs with the positions scrambled (extrapolated
    /// between random vertices of the batch), so that many vertices leave
    /// the guard band, through both draw entries and the env pass after.
    #[test]
    fn fuzz_draws() {
        let Ok(paths) = std::env::var("MAT6_CAPTURES") else {
            return;
        };
        let mut seed = 0x9fa4_7842_0bad_cafeu64;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let rd = |d: &[u8], q: usize, l: usize| u32::from_le_bytes(d[q * 16 + 4 * l..q * 16 + 4 * l + 4].try_into().unwrap());
        let (mut ok, mut bad, mut ni, mut adc) = (0, 0, 0, 0);
        let mut shown = 0;
        for path in paths.split(',') {
            let bytes = std::fs::read(path).unwrap();
            for r in Record::read_all(&bytes) {
                if r.hash != HASH || r.unit != 1 || !matches!(r.pc, 0x00 | 0x398 | 0x50 | 0x7f8 | 0x20 | 0xcb8) {
                    continue;
                }
                let top = r.top as usize & 0x3ff;
                let n_sub = rd(&r.data_in, top + 1, 0) as usize & 0xffff;
                let total: usize = (0..n_sub).map(|i| rd(&r.data_in, top + 2 + i, 0) as usize & 0xffff).sum();
                let shadow = matches!(r.pc, 0x20 | 0xcb8);
                let (stride, pos) = if shadow { (1, 0) } else { (3, 2) };
                if total == 0 {
                    continue;
                }
                for _ in 0..8 {
                    let mut rr = r.clone();
                    for k in 0..total {
                        let a = (rnd() as usize) % total;
                        let b = (rnd() as usize) % total;
                        let t = ((rnd() % 1000) as f32 / 1000.0) * 8.0 - 4.0;
                        let at = |k: usize| (top + 2 + n_sub + stride * k + pos) & 0x3ff;
                        for l in 0..3 {
                            let va = f32::from_bits(rd(&r.data_in, at(a), l));
                            let vb = f32::from_bits(rd(&r.data_in, at(b), l));
                            let v = va + (vb - va) * t;
                            let o = at(k) * 16 + 4 * l;
                            rr.data_in[o..o + 4].copy_from_slice(&v.to_bits().to_le_bytes());
                        }
                    }
                    match check(&rr, &mut shown) {
                        Some(true) => ok += 1,
                        Some(false) => bad += 1,
                        None => ni += 1,
                    }
                    // The env pass on top of that draw.
                    if !shadow {
                        let mut vu = rr.setup();
                        vu.run(5_000_000, &mut |_| {});
                        let out = &vu.data;
                        adc += (0..total).filter(|k| rd(out, (top + 0xb4 + 3 + 3 * k) & 0x3ff, 3) == 0xffff_8000).count();
                        let mut r2 = rr.clone();
                        r2.data_in = vu.data.clone();
                        r2.start = ssx3_vu::vu::capture::ArchState::of(&vu);
                        r2.pc = 0x30;
                        // The env pass reads per-vertex rotations: give the
                        // per-batch draws the batch's.
                        if matches!(r.pc, 0x50 | 0x7f8) {
                            let rot = rd(&r2.data_in, top + 1, 2);
                            for k in 0..total {
                                let o = ((top + 2 + n_sub + 3 * k) & 0x3ff) * 16 + 12;
                                r2.data_in[o..o + 4].copy_from_slice(&rot.to_le_bytes());
                            }
                        }
                        match check(&r2, &mut shown) {
                            Some(true) => ok += 1,
                            Some(false) => bad += 1,
                            None => ni += 1,
                        }
                    }
                }
            }
        }
        eprintln!("fuzz draws: ok {ok} mismatch {bad} not-implemented {ni} (ADC vertices: {adc})");
        assert_eq!(bad, 0);
    }

    /// Compare native and translated on one input; prints the first few
    /// mismatches. Returns None when either side declines the input.
    fn check(r: &Record, shown: &mut u32) -> Option<bool> {
        let mut vu = r.setup();
        let mut ref_kicks: Vec<Vec<u128>> = Vec::new();
        if !vu.run(5_000_000, &mut |p| ref_kicks.push(p.to_vec())) {
            return None;
        }
        let mut mem: Box<[u8; 16384]> = r.data_in.clone().into_boxed_slice().try_into().unwrap();
        let mut kicks: Vec<Vec<u128>> = Vec::new();
        if !run(r.pc, r.top, r.itop, &mut mem, &mut VuState::default(), &mut |p| kicks.push(p.to_vec())) {
            return None;
        }
        let ok = kicks == ref_kicks && mem[..] == vu.data[..];
        if !ok && *shown < 4 {
            *shown += 1;
            report(r, &ref_kicks, &vu.data, &kicks, &mem[..]);
        }
        Some(ok)
    }

    fn report(r: &Record, ref_kicks: &[Vec<u128>], ref_mem: &[u8], kicks: &[Vec<u128>], mem: &[u8]) {
        eprintln!("--- mismatch: entry {:#06x} top {:#x} frame {} seq {}", r.pc, r.top, r.frame, r.seq);
        if kicks.len() != ref_kicks.len() {
            eprintln!("  kick count native {} translated {}", kicks.len(), ref_kicks.len());
        }
        for (i, (a, b)) in kicks.iter().zip(ref_kicks).enumerate() {
            if a != b {
                eprintln!("  kick {i}: len native {} translated {}", a.len(), b.len());
                for (j, (x, y)) in a.iter().zip(b).enumerate().filter(|(_, (x, y))| x != y).take(8) {
                    eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
                }
                break;
            }
        }
        let diff: Vec<usize> = (0..1024).filter(|q| mem[q * 16..q * 16 + 16] != ref_mem[q * 16..q * 16 + 16]).collect();
        for &q in diff.iter().take(8) {
            eprintln!("  mem qw {q:#05x}: native {:02x?} translated {:02x?}", &mem[q * 16..q * 16 + 16], &ref_mem[q * 16..q * 16 + 16]);
        }
        if !diff.is_empty() {
            eprintln!("  {} qwords of data memory differ", diff.len());
        }
    }
}
