//! # Type 2 (`9790369f`) natively: Bezier terrain with stitched LOD edges
//!
//! A readable Rust implementation of the VU1 microprogram with hash
//! `0x9790369fcc84762b` (`generated/vu/vu_9790369fcc84762b.rs` is the
//! literal translation; `docs/investigation/phase3-render.md` §1.3). [`run`]
//! produces the same XGKICK packets **and** the same data memory as the
//! translated program, bit for bit (checked on real frames by
//! `SSX3_VERIFY_BEZIER=1`, `crates/ssx3-ps2/src/bezier_verify.rs`).
//!
//! A type-2 patch is one whose four edges do not share a LOD. The EE
//! uploads, from TOP: the CPU's 8×8 grid of positions (TOP+0, row-major,
//! `row·8 + col`), the 8×8 UVs (TOP+0x40), UV2s or a matrix (TOP+0x80), the
//! 16 power-basis coefficients (TOP+0xc0..0xcf) and the edge LODs
//! (TOP+0xd0 {x, y, z, w}, each 4, 6 or 8). The edges are
//!
//! | LOD | grid line | packet |
//! |---|---|---|
//! | z | column 0 | A |
//! | y | row 7 | A |
//! | w | column 7 | B |
//! | x | row 0 | B |
//!
//! The program
//! 1. re-evaluates every edge with L < 8 on the VU: L-1 points (from the
//!    second one to the far corner) of the bicubic at the basis rows of the
//!    table for L, written over the first points of that grid line (xyz);
//!    the UVs of all four edges are re-interpolated linearly from the
//!    corners (5 points per edge, step from a per-L table);
//! 2. draws the interior (columns 1-6, rows 1-6) as 5 two-row strips of 12
//!    vertices (3 in packet A, 2 in packet B), and every edge as a "zipper"
//!    strip between the inner line (6 points) and the edge (L points): the
//!    longer list first, alternating, the 2 extra points of the longer list
//!    appended;
//! 3. culls/clips like type 1 (shared code in [`crate::bezier`]).
//!
//! | Entries | What |
//! |---|---|
//! | 0x09a0 | ctx1 draw (UV), then pass 2 "flag 0": ST := UV2·q in place (UV2 edges re-interpolated), re-kick ctx2 |
//! | 0x0a38 | ctx1 draw, then a full ctx2 re-draw with UV2 |
//! | 0x0c48 | projector: border normals (unused), UV2 = (L·P).xy for the last call's grid, ctx draw with the tags left by the last call |
//! | 0x0cf0 | shine: border normals into the normal grid at TOP+0x40, UV2 = (R·N).xy, draw |
//! | 0x0938, 0x0b00, 0x0b68 | never used by the game, not implemented |
//!
//! State between calls is in data memory only: qw510.x (TOP of the last
//! draw), qw511 (saved return addresses: .x of the draw routines, .y of the
//! normal pass, .w of the zipper clip handlers), qw554 {positions, UVs} of
//! the current draw, the tag slots, the fan toggle (qw41.x). One quirk: the
//! projector/shine entries jump into the middle of the draw routine, whose
//! end returns through qw511.x, i.e. to wherever the **previous** call's
//! draw returned: after 0x09a0 (0x144) or 0x0a38 (0x15d) that is the end of
//! the program, after a 0x09a0 whose pass 2 did not run yet (0x13c) it
//! would run pass 2 "flag 0"; anything else is not implemented.

use crate::bezier::{
    add, bc, emit_strip, msub, mul, project_xy, rewrite_st, sub, xform, ClipFlags, Mem, Raster, Rgba, Vertex, V,
    COLOUR, EOP, FAN_TAG_A, FAN_TAG_B, FAN_TAG_CTX1, FAN_TAG_CTX2, FAN_TOGGLE, LAST_TOP, PACKET_A, PACKET_B,
    RETURN_SAVE, STRIP_TAG_CTX1, STRIP_TAG_CTX2,
};
use ssx3_vu::vu::float;

/// Hash of the program this module implements.
pub const HASH: u64 = 0x9790_369f_cc84_762b;

/// Register state the program carries from one call to the next.
///
/// Empty: no implemented entry reads a VU register before writing it
/// (checked against the translated program on real frames). It exists so
/// the API matches [`crate::mat3`].
#[derive(Clone, Debug, Default)]
pub struct VuState {}

/// Relative to the patch base (TOP of the draw call).
const UV: u16 = 0x40;
const UV2: u16 = 0x80;
const NORMALS: u16 = 0x40;
const COEFFS: u16 = 0xc0;
const LODS: u16 = 0xd0;
/// qw548 + (L-4): {.x basis table, .y derivative basis table} for L;
/// qw549 + (L-4): the edge UV step for L (lane = edge, see `edge_steps`).
const BASIS: u16 = 548;
const UV_STEP: u16 = 549;
/// qw554: {positions, UVs} of the current draw.
const DRAW_SOURCES: u16 = 554;

/// Entries [`run`] implements natively (MSCAL byte addresses).
pub fn implemented(entry: u32) -> bool {
    matches!(entry, 0x09a0 | 0x0a38 | 0x0c48 | 0x0cf0)
}

/// The return address a `BAL vi15` at `pc` leaves in vi15 (code address / 8).
fn ret(pc: u32) -> u16 {
    ((pc + 16) >> 3) as u16
}

/// qw511.x values the draw routine may return to from the projector and
/// shine entries (see the module doc).
const RET_09A0_PASS2: u16 = 0x13c;
const RET_09A0_END: u16 = 0x144;
const RET_0A38_END: u16 = 0x15d;

/// The edge LODs of a patch ({x, y, z, w} at +0xd0).
#[derive(Clone, Copy)]
struct Lods([u16; 4]);

/// LOD lanes.
const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;
const W: usize = 3;

impl Lods {
    fn read(m: &Mem, patch: u16) -> Lods {
        let v = m.ld(patch.wrapping_add(LODS));
        Lods(std::array::from_fn(|i| v[i] as u16))
    }
    fn valid(&self) -> bool {
        self.0.iter().all(|l| matches!(l, 4 | 6 | 8))
    }
    /// Per-edge UV step (the vf5 of 0x25b8): x ← L_z, y ← L_y, z ← L_w,
    /// w ← L_x, each from its own LOD's qword.
    fn uv_steps(&self, m: &Mem) -> V {
        let step = |lane: usize, out: usize| m.ld(UV_STEP + self.0[lane] - 4)[out];
        [step(Z, 0), step(Y, 1), step(W, 2), step(X, 3)]
    }
}

/// Selects the code path: the four edge LODs (`x | y<<4 | z<<8 | w<<12`) of
/// the patch the entry draws (for the verifier's counts).
pub fn variant(entry: u32, top: u32, mem: &[u8; 16384]) -> u32 {
    let mut copy = Box::new(*mem);
    let m = Mem(&mut copy);
    let patch = match entry {
        0x0c48 | 0x0cf0 => m.ilw(LAST_TOP, 0),
        _ => (top & 0x3ff) as u16,
    };
    let l = Lods::read(&m, patch).0;
    l.iter().enumerate().map(|(i, &v)| (v as u32 & 0xf) << (4 * i)).sum()
}

/// [`variant`] as text.
pub fn describe_variant(v: u32) -> String {
    format!("L x{} y{} z{} w{}", v & 0xf, (v >> 4) & 0xf, (v >> 8) & 0xf, v >> 12)
}

// ---------------------------------------------------------------------------
// Topology: the strips of a patch as grid offsets
// ---------------------------------------------------------------------------

/// An edge: which LOD, its grid line (`outer`, step `stride`) and the inner
/// line next to it (6 points), plus the parameters of its re-evaluation
/// (`u0, v0, du, dv` into the basis tables; output from `outer + stride`).
struct Edge {
    lod: usize,
    inner: u16,
    outer: u16,
    stride: u16,
}

const EDGE_Z: Edge = Edge { lod: Z, inner: 0x09, outer: 0x00, stride: 8 };
const EDGE_Y: Edge = Edge { lod: Y, inner: 0x31, outer: 0x38, stride: 1 };
const EDGE_W: Edge = Edge { lod: W, inner: 0x0e, outer: 0x07, stride: 8 };
const EDGE_X: Edge = Edge { lod: X, inner: 0x09, outer: 0x00, stride: 1 };

/// One strip of the packet: grid offsets in vertex order, and whether it is
/// a zipper (whose clip handlers save a mark at qw511.w).
struct Strip {
    offsets: Vec<u16>,
    /// Pairs before the leftover points (zippers).
    short: Option<usize>,
}

/// Interior strips: `count` two-row strips of columns 1-6 from `first`.
fn interior(first: u16, count: u16) -> impl Iterator<Item = Strip> {
    (0..count).map(move |s| {
        let a = first + 8 * s;
        Strip { offsets: (0..12).map(|i| a + (i % 2) * 8 + i / 2).collect(), short: None }
    })
}

/// A zipper strip: the longer of (inner 6 points, outer L points) first,
/// alternating, then two more of the longer list if it is longer.
fn zipper(e: &Edge, lods: Lods) -> Strip {
    let l = lods.0[e.lod];
    let inner: Vec<u16> = (0..6).map(|k| e.inner + k * e.stride).collect();
    let outer: Vec<u16> = (0..l).map(|k| e.outer + k * e.stride).collect();
    let (long, short) = if 6 > l { (inner, outer) } else { (outer, inner) };
    let mut offsets = Vec::new();
    for k in 0..short.len() {
        offsets.push(long[k]);
        offsets.push(short[k]);
    }
    if long.len() > short.len() {
        offsets.push(long[short.len()]);
        offsets.push(long[short.len() + 1]);
    }
    Strip { offsets, short: Some(short.len()) }
}

/// The two packets: (slot, NLOOP, strips).
fn packets(lods: Lods) -> [(u16, u16, Vec<Strip>); 2] {
    let l = lods.0;
    let mut a: Vec<Strip> = interior(0x09, 3).collect();
    a.push(zipper(&EDGE_Z, lods));
    a.push(zipper(&EDGE_Y, lods));
    let mut b: Vec<Strip> = interior(0x21, 2).collect();
    b.push(zipper(&EDGE_W, lods));
    b.push(zipper(&EDGE_X, lods));
    [(PACKET_A, l[Z] + 0x30 + l[Y], a), (PACKET_B, l[W] + 0x24 + l[X], b)]
}

/// The mark a zipper clip handler saves at qw511.w for vertex `i`.
fn zipper_mark(short: usize, i: usize) -> u16 {
    match i {
        _ if i == 2 * short => 0x5d7,
        _ if i == 2 * short + 1 => 0x5eb,
        _ if i.is_multiple_of(2) => 0x5be,
        _ => 0x5c9,
    }
}

// ---------------------------------------------------------------------------
// Entries
// ---------------------------------------------------------------------------

/// Run the program at `entry` with VIF `top` on VU1 data memory `mem`,
/// handing every XGKICK packet to `kick`. Returns `false` (and does
/// nothing) when the entry, or the variant selected by the input, is not
/// implemented natively.
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
    let tags = |m: &mut Mem, strip: u16, fan: Option<u16>| {
        m.st_mask(PACKET_A, m.ld(strip), 0b0111);
        m.st_mask(PACKET_B, m.ld(strip), 0b0111);
        if let Some(fan) = fan {
            m.st_mask(FAN_TAG_A, m.ld(fan), 0b0111);
            m.st_mask(FAN_TAG_B, m.ld(fan), 0b0111);
        }
    };
    match entry {
        0x09a0 | 0x0a38 => {
            if !Lods::read(&m, top).valid() {
                return false;
            }
            let fans = (entry == 0x0a38).then_some(FAN_TAG_CTX1);
            tags(&mut m, STRIP_TAG_CTX1, fans);
            m.isw(LAST_TOP, 0, top);
            // The first draw's BAL: 0x09d0 / 0x0a80.
            let bal = if entry == 0x09a0 { 0x09d0 } else { 0x0a80 };
            draw_patch(&mut m, top, top.wrapping_add(UV), ret(bal), kick);
            if entry == 0x09a0 {
                tags(&mut m, STRIP_TAG_CTX2, None);
                pass2_st(&mut m, top, ret(0x0a10), kick);
            } else {
                tags(&mut m, STRIP_TAG_CTX2, Some(FAN_TAG_CTX2));
                m.isw(LAST_TOP, 0, top);
                draw_patch(&mut m, top, top.wrapping_add(UV2), ret(0x0ad8), kick);
            }
        }
        0x0c48 | 0x0cf0 => {
            let last = m.ilw(LAST_TOP, 0);
            let back = m.ilw(RETURN_SAVE, 0);
            if !Lods::read(&m, last).valid() || !matches!(back, RET_09A0_PASS2 | RET_09A0_END | RET_0A38_END) {
                return false;
            }
            let uv2 = top.wrapping_add(UV2);
            m.isw(DRAW_SOURCES, 0, last);
            m.isw(DRAW_SOURCES, 1, uv2);
            m.isw(RETURN_SAVE, 1, ret(entry + 0x20));
            border_normals(&mut m, last, top);
            let mat = m.matrix(top.wrapping_add(0x80));
            let src = if entry == 0x0c48 { last } else { top.wrapping_add(NORMALS) };
            project_xy(&mut m, &mat, src, uv2, 0x40);
            draw(&mut m, last, uv2, kick);
            // The draw returns through qw511.x (the previous call's).
            if back == RET_09A0_PASS2 {
                tags(&mut m, STRIP_TAG_CTX2, None);
                pass2_st(&mut m, top, ret(0x0a10), kick);
            }
        }
        _ => return false,
    }
    true
}

/// 0x0d90: evaluate the edges, re-interpolate the edge UVs, draw.
fn draw_patch(m: &mut Mem, patch: u16, uv: u16, ret: u16, kick: &mut dyn FnMut(&[u128])) {
    m.isw(RETURN_SAVE, 0, ret);
    m.isw(DRAW_SOURCES, 0, patch);
    m.isw(DRAW_SOURCES, 1, uv);
    let lods = Lods::read(m, patch);
    let c = coefficients(m, patch);
    for e in [&EDGE_Z, &EDGE_Y, &EDGE_W, &EDGE_X] {
        let l = lods.0[e.lod];
        if l < 8 {
            let basis = m.ilw(BASIS + l - 4, 0);
            for (k, (u, v)) in edge_params(e, l).enumerate() {
                let p = bezier(&c, m.ld(basis.wrapping_add(u)), m.ld(basis.wrapping_add(v)));
                m.st_mask(patch.wrapping_add(e.outer + (k as u16 + 1) * e.stride), p, 0b1110);
            }
        }
    }
    border_uvs(m, uv, lods.uv_steps(m));
    draw(m, patch, uv, kick);
}

/// The basis-table indices (u, v) of the L-1 re-evaluated points of an
/// edge: z (1, 0)+(1, 0)k, y (L-1, 1)+(0, 1)k, w (1, L-1)+(1, 0)k,
/// x (0, 1)+(0, 1)k.
fn edge_params(e: &Edge, l: u16) -> impl Iterator<Item = (u16, u16)> {
    let (u0, v0, du, dv) = match e.lod {
        Z => (1, 0, 1, 0),
        Y => (l - 1, 1, 0, 1),
        W => (1, l - 1, 1, 0),
        _ => (0, 1, 0, 1),
    };
    (0..l - 1).map(move |k| (u0 + du * k, v0 + dv * k))
}

fn coefficients(m: &Mem, patch: u16) -> [V; 16] {
    std::array::from_fn(|i| m.ld(patch.wrapping_add(COEFFS + i as u16)))
}

/// The bicubic at basis rows `u`, `v`: `R_j = C[4j..4j+4]·u`, `P = R·v`
/// (0x2338).
fn bezier(c: &[V; 16], u: V, v: V) -> V {
    let r: [V; 4] = std::array::from_fn(|j| xform(&[c[4 * j], c[4 * j + 1], c[4 * j + 2], c[4 * j + 3]], u));
    xform(&r, v)
}

/// 0x25b8: the edge UVs from the corner UVs (row 0 col 0, row 7 col 0,
/// row 7 col 7, row 0 col 7) by repeated addition of `(corner' - corner)·
/// step`: 5 points along each edge (xy only).
fn border_uvs(m: &mut Mem, uv: u16, steps: V) {
    let at = |o: u16| uv.wrapping_add(o);
    let (c00, c70, c77, c07) = (m.ld(at(0)), m.ld(at(56)), m.ld(at(63)), m.ld(at(7)));
    let d_col0 = mul(sub(c70, c00), bc(steps[0]));
    let d_row7 = mul(sub(c77, c70), bc(steps[1]));
    let d_col7 = mul(sub(c77, c07), bc(steps[2]));
    let d_row0 = mul(sub(c07, c00), bc(steps[3]));
    let (mut col0, mut row7, mut col7, mut row0) = (c00, c70, c07, c00);
    for i in 1..=5u16 {
        col0 = add(col0, d_col0);
        row7 = add(row7, d_row7);
        col7 = add(col7, d_col7);
        row0 = add(row0, d_row0);
        m.st_mask(at(8 * i), col0, 0b1100);
        m.st_mask(at(56 + i), row7, 0b1100);
        m.st_mask(at(7 + 8 * i), col7, 0b1100);
        m.st_mask(at(i), row0, 0b1100);
    }
}

/// 0x1580: for the edges of the last call's patch with L < 8, the normals
/// `normalize(∂P/∂v × ∂P/∂u)` (derivative basis rows from the .y table)
/// into the normal grid at TOP+0x40 (xyz); ∂P/∂u is stored there first.
fn border_normals(m: &mut Mem, last: u16, top: u16) {
    let lods = Lods::read(m, last);
    let c = coefficients(m, last);
    let grid = top.wrapping_add(NORMALS);
    for e in [&EDGE_Z, &EDGE_Y, &EDGE_W, &EDGE_X] {
        let l = lods.0[e.lod];
        if l >= 8 {
            continue;
        }
        let b = m.ld(BASIS + l - 4);
        let (value, deriv) = (b[0] as u16, b[1] as u16);
        let out = |k: usize| grid.wrapping_add(e.outer + (k as u16 + 1) * e.stride);
        for (k, (u, v)) in edge_params(e, l).enumerate() {
            let t = bezier(&c, m.ld(deriv.wrapping_add(u)), m.ld(value.wrapping_add(v)));
            m.st_mask(out(k), t, 0b1110);
        }
        for (k, (u, v)) in edge_params(e, l).enumerate() {
            let p = bezier(&c, m.ld(value.wrapping_add(u)), m.ld(deriv.wrapping_add(v)));
            let e1 = m.ld(out(k));
            // OPMULA / OPMSUB: p × e1 (xyz; w stays p.w).
            let acc = mul([p[1], p[2], p[0], 0], [e1[2], e1[0], e1[1], 0]);
            let n = msub(acc, [e1[1], e1[2], e1[0], 0], [p[2], p[0], p[1], 0]);
            let n = [n[0], n[1], n[2], p[3]];
            // ERLENG: 1/|n| on the EFU (double precision, then rounded).
            let (x, y, z) = (float::to_f64(n[0]), float::to_f64(n[1]), float::to_f64(n[2]));
            let r = float::efu(1.0 / (x * x + y * y + z * z).sqrt());
            m.st_mask(out(k), mul(n, bc(r)), 0b1110);
        }
    }
}

/// 0x1060: the strip draw of a patch whose positions are at `pos` and UVs
/// at `uv` (grid offsets): tags' NLOOP, the constant colour into every
/// RGBA slot, then per packet the interior strips and the two zippers.
fn draw(m: &mut Mem, pos: u16, uv: u16, kick: &mut dyn FnMut(&[u128])) {
    let lods = Lods::read(m, pos);
    let r = Raster::load(m);
    m.isw(FAN_TOGGLE, 0, FAN_TAG_B);
    let colour = m.ld(COLOUR);
    let packets = packets(lods);
    for (slot, nloop, _) in &packets {
        m.isw(*slot, 0, nloop | EOP);
        // 0x0bd8: two RGBA slots per step while the count stays > 0.
        let mut left = *nloop as i16;
        let mut o = slot + 1;
        loop {
            left = left.wrapping_sub(2);
            m.st(o.wrapping_add(1), colour);
            m.st(o.wrapping_add(4), colour);
            o = o.wrapping_add(6);
            if left <= 0 {
                break;
            }
        }
    }
    let mut clip = ClipFlags::default();
    for (slot, _, strips) in &packets {
        let mut out = slot + 1;
        for s in strips {
            let verts: Vec<Vertex> =
                s.offsets.iter().map(|&o| Vertex { pos: pos.wrapping_add(o), uv: uv.wrapping_add(o) }).collect();
            let mark = |i: usize| s.short.map(|short| zipper_mark(short, i));
            emit_strip(m, &r, &mut clip, &verts, &mut out, Rgba::Keep, &mark, kick);
        }
        kick(&m.gif_packet(*slot));
    }
}

/// 0x1368, pass 2 "flag 0": re-interpolate the edge UV2s, ST := UV2·q in
/// the vertex order of the draw, re-kick both packets (ctx2 tags set by the
/// caller; clipped fans are not redrawn).
fn pass2_st(m: &mut Mem, top: u16, ret: u16, kick: &mut dyn FnMut(&[u128])) {
    m.isw(RETURN_SAVE, 0, ret);
    let lods = Lods::read(m, m.ilw(LAST_TOP, 0));
    let uv2 = top.wrapping_add(UV2);
    border_uvs(m, uv2, lods.uv_steps(m));
    for (slot, _, strips) in packets(lods) {
        let addrs = strips.iter().flat_map(|s| s.offsets.iter().map(|&o| uv2.wrapping_add(o)).collect::<Vec<_>>());
        rewrite_st(m, slot + 1, addrs);
        kick(&m.gif_packet(slot));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Replays sampled calls (`BEZIER_DUMP`, from `SSX3_VERIFY_BEZIER_DUMP`).
    #[test]
    fn dumped_calls() {
        crate::bezier::dump::replay(1, &|e, t, i, m, k| run(e, t, i, m, &mut VuState::default(), k));
    }
}
