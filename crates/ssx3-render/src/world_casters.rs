//! Sun-shadow casters from the world data (renderer option "Sun shadows",
//! `docs/plans/shadow-maps.md`): the course's terrain read from game memory
//! on the game thread, independent of what the game culls for the camera,
//! so the shadow map holds what is off screen too. Nothing here runs game
//! code or writes game memory.
//!
//! The terrain patches live in the Nash loose octree (`g_App` (gp−0x848)
//! +0x84 GameModule +0x20 Nash, `docs/investigation/structs/world.md`):
//! eight root records {level, ix, iy, iz, node} of 0x14 bytes; a node has
//! eight children (+0x00..+0x1c) and lists of instances (+0x20), patches
//! (+0x24) and effects (+0x28). A patch (list link at +0x00):
//!
//! | Off | What |
//! |---|---|
//! | 0x040..0x13f | 16 coefficient vectors C (vec4, w = 1) of the power basis: P(u, v) = Σ u^(3−k) v^(3−j) C[4k+j], u, v ∈ [0, 1]; C[15] is the corner (0, 0) in world units (`BezierPatchCache_TessPositions` 0x374180) |
//! | 0x140 | bounding sphere (centre, radius) |
//! | 0x155 / 0x156 | section index (u8) / slot (i16) |
//!
//! A patch is drawn when its section is shown, as `Nash_CollectPatches`
//! (0x22a698) tests: with S = *(GameModule+0x10), slot ≥ 0,
//! S[section·8 + 0x24] = 6 and S[slot·0x18 + 0x3f0] = 3. (On Snow Jam about
//! a third of the loaded patches pass; the rest belong to sections not
//! shown.)
//!
//! Each patch is tessellated once (an [`GRID`]×[`GRID`] grid, keyed by its
//! address and coefficients) and the triangles of all resident patches are
//! published as one list, renewed when the set changes (sections stream in
//! and out along the course).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::chain::Mem;

/// The game's `gp`.
const GAME_GP: u32 = 0x004a_30f0;
/// `g_App` at gp−0x848; its GameModule at +0x84; the Nash at +0x20.
const APP_GP: u32 = 0x848;
const APP_GAME_MODULE: u32 = 0x84;
const GAME_MODULE_NASH: u32 = 0x20;
const GAME_MODULE_SECTIONS: u32 = 0x10;
/// Patch fields.
const PATCH_COEFFS: u32 = 0x40;
/// Grid points per patch side.
pub const GRID: usize = 8;

static ON: AtomicBool = AtomicBool::new(false);

/// Collect casters (the renderer's shadows are on).
pub fn set_enabled(on: bool) {
    ON.store(on, Ordering::Relaxed);
}

pub fn enabled() -> bool {
    ON.load(Ordering::Relaxed)
}

/// The world's casters: world-space triangles, per vertex its position and
/// its triangle's normal (oriented up, +z: the terrain's sky side).
#[derive(Debug, Default)]
pub struct Casters {
    /// Renewed with every change of the set (the renderer uploads then).
    pub generation: u64,
    pub triangles: Vec<[f32; 6]>,
    /// Alpha cut-out triangles (position, UV), in runs per texture:
    /// (TexMan id, first vertex, vertices).
    pub cutouts: Vec<[f32; 5]>,
    pub cutout_runs: Vec<(i16, u32, u32)>,
    pub patches: u32,
    pub instances: u32,
    /// The world file's name (`BAM` for Snow Jam): cWorldCache +0x304 holds
    /// `|data/worlds/<name>.ssb`.
    pub world: String,
}

/// One object's casters: solid triangles, and alpha cut-outs per texture
/// (TexMan id; position and UV per vertex).
#[derive(Default, Debug)]
pub struct Geometry {
    pub solid: Vec<[f32; 6]>,
    pub cut: Vec<(i16, Vec<[f32; 5]>)>,
}

/// The tessellated patches, kept from frame to frame.
#[derive(Default)]
pub struct CasterCache {
    meshes: HashMap<u32, (u64, Geometry)>,
    /// (object, its hash, an instance (else a patch)).
    set: Vec<(u32, u64, bool)>,
    current: Option<Arc<Casters>>,
    generation: u64,
}

fn word(m: &dyn Mem, a: u32) -> u32 {
    (m.qword(a & !15) >> ((a & 12) * 8)) as u32
}

fn ram(a: u32) -> bool {
    (0x10_0000..0x200_0000).contains(&a) && a & 3 == 0
}

impl CasterCache {
    /// This frame's casters (None while there is no world).
    pub fn update(&mut self, m: &dyn Mem) -> Option<Arc<Casters>> {
        let app = word(m, GAME_GP.wrapping_sub(APP_GP));
        if !ram(app) {
            return None;
        }
        let module = word(m, app + APP_GAME_MODULE);
        if !ram(module) {
            return None;
        }
        let nash = word(m, module + GAME_MODULE_NASH);
        let sections = word(m, word(m, module + GAME_MODULE_SECTIONS));
        if !ram(nash) || !ram(sections) {
            return None;
        }
        let shown = |p: u32| {
            let w = word(m, p + 0x154);
            let (section, slot) = ((w >> 8) & 0xff, (w >> 16) as i16);
            slot >= 0
                && word(m, sections + section * 8 + 0x24) == 6
                && word(m, sections.wrapping_add((slot as u32) * 0x18) + 0x3f0) == 3
        };
        // Instances (`docs/investigation/world-instances.md`): the static
        // models (kind 3) in shown sections (0xff: shared, always shown).
        let shown_instance = |i: u32| {
            let w = word(m, i + 0x7c);
            let (section, slot) = ((w >> 8) & 0xff, (w >> 16) as i16);
            // (The lists also hold other objects: a valid descriptor and a
            // plausible slot keep to the instances.)
            let desc = word(m, i + 0x80);
            word(m, i + 8) & 3 == 3
                && ram(desc)
                && (1..=64).contains(&word(m, desc + 4))
                && (section == 0xff || (0..4096).contains(&slot))
                && (section == 0xff
                    || (word(m, sections + section * 8 + 0x24) == 6
                        && word(
                            m,
                            sections.wrapping_add((slot as i32 as u32) * 0x18) + 0x3f0,
                        ) == 3))
        };
        let mut set = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut stack: Vec<(u32, u32)> = (0..8)
            .map(|k| (word(m, nash + 0x14 * k + 0x10), 0))
            .collect();
        while let Some((node, depth)) = stack.pop() {
            if !ram(node) || depth > 40 || !seen.insert(node) {
                continue;
            }
            let mut p = word(m, node + 0x24);
            let mut n = 0;
            while ram(p) && n < 100_000 {
                if shown(p) {
                    set.push((p, coeff_hash(m, p), false));
                }
                p = word(m, p);
                n += 1;
            }
            let mut i = word(m, node + 0x20);
            let mut n = 0;
            while ram(i) && n < 100_000 {
                if shown_instance(i) {
                    set.push((i, instance_hash(m, i), true));
                }
                i = word(m, i);
                n += 1;
            }
            stack.extend((0..8).map(|i| (word(m, node + 4 * i), depth + 1)));
        }
        set.sort_unstable();
        if set == self.set {
            return self.current.clone();
        }
        if std::env::var_os("SSX3_CASTERS_DEBUG").is_some() {
            let old: std::collections::HashMap<u32, u64> =
                self.set.iter().map(|e| (e.0, e.1)).collect();
            let new: std::collections::HashMap<u32, u64> = set.iter().map(|e| (e.0, e.1)).collect();
            for (a, h) in &new {
                match old.get(a) {
                    None => eprintln!(
                        "casters: + {a:#x} flags {:#x} sec {:#x}",
                        word(m, a + 8),
                        word(m, a + 0x7c)
                    ),
                    Some(o) if o != h => eprintln!("casters: ~ {a:#x}"),
                    _ => {}
                }
            }
            for a in old.keys().filter(|a| !new.contains_key(a)) {
                eprintln!(
                    "casters: - {a:#x} flags {:#x} sec {:#x}",
                    word(m, a + 8),
                    word(m, a + 0x7c)
                );
            }
        }
        // The set changed: tessellate what is new, drop what is gone.
        let keep: std::collections::HashSet<u32> = set.iter().map(|&(p, _, _)| p).collect();
        self.meshes.retain(|p, _| keep.contains(p));
        let mut triangles = Vec::new();
        for &(p, h, instance) in &set {
            let fresh = self.meshes.get(&p).is_none_or(|(mh, _)| *mh != h);
            if fresh {
                let t = if instance {
                    decode_instance(m, p)
                } else {
                    Geometry {
                        solid: tessellate(m, p),
                        cut: Vec::new(),
                    }
                };
                self.meshes.insert(p, (h, t));
            }
            triangles.extend_from_slice(&self.meshes[&p].1.solid);
        }
        // The cut-outs, gathered by texture.
        let mut by_tex: std::collections::BTreeMap<i16, Vec<[f32; 5]>> = Default::default();
        for &(p, _, _) in &set {
            for (t, v) in &self.meshes[&p].1.cut {
                by_tex.entry(*t).or_default().extend_from_slice(v);
            }
        }
        let (mut cutouts, mut cutout_runs) = (Vec::new(), Vec::new());
        for (t, v) in by_tex {
            cutout_runs.push((t, cutouts.len() as u32, v.len() as u32));
            cutouts.extend(v);
        }
        self.generation += 1;
        let instances = set.iter().filter(|e| e.2).count() as u32;
        self.set = set;
        self.current = Some(Arc::new(Casters {
            generation: self.generation,
            patches: self.set.len() as u32 - instances,
            instances,
            triangles,
            cutouts,
            cutout_runs,
            world: world_name(m, module),
        }));
        self.current.clone()
    }
}

/// The world file's name from cWorldCache (GameModule+0x10 → World → cache)
/// +0x304: `|data/worlds/<name>.ssb`.
fn world_name(m: &dyn Mem, module: u32) -> String {
    let world = word(m, module + GAME_MODULE_SECTIONS);
    let cache = if ram(world) { word(m, world) } else { 0 };
    if !ram(cache) {
        return String::new();
    }
    let bytes: Vec<u8> = (0..12)
        .flat_map(|k| word(m, cache + 0x304 + 4 * k).to_le_bytes())
        .take_while(|&b| b != 0)
        .collect();
    let path = String::from_utf8_lossy(&bytes);
    path.rsplit('/')
        .next()
        .unwrap_or("")
        .split('.')
        .next()
        .unwrap_or("")
        .to_string()
}

/// A hash of what an instance's geometry comes from: its matrix, scale,
/// descriptor, material table and chain base.
fn instance_hash(m: &dyn Mem, i: u32) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let words = (0..16)
        .map(|k| i + 0x10 + 4 * k)
        .chain([i + 0x80, i + 0x84, i + 0x94, i + 0x98]);
    for a in words {
        h = (h ^ word(m, a) as u64).wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// A row-vector 4×4 matrix at `a` (rows of four f32).
fn matrix(m: &dyn Mem, a: u32) -> [[f64; 4]; 4] {
    std::array::from_fn(|r| {
        let q = m.qword(a + 16 * r as u32);
        std::array::from_fn(|c| f32::from_bits((q >> (32 * c)) as u32) as f64)
    })
}

/// `a · b` (row vectors: `a`'s rows through `b`).
fn mul(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..4).map(|k| a[r][k] * b[k][c]).sum()))
}

/// An instance's triangles in world space (normal 0: they cast from both
/// faces; fences and cards are single sheets). Per mesh node i, the matrix
/// `H = S · N[i] · W` of `PS2GraphicsMan_DrawModel` (0x37e238): S the
/// descriptor's dequantisation scale × the instance scale, N the node
/// hierarchy (local translations × the instance scale), W the instance's
/// world matrix (+0x10). Each group's DMA chain (base +0x98, or the
/// descriptor's +0x24 for SH-lit instances) holds batches of triangle
/// strips: header (UNPACK V4-32 ×2), strip lengths (V2-8), positions (the
/// first V3-16 after the header, int16 / 32768). Blended materials (glass,
/// decals) do not cast.
fn decode_instance(m: &dyn Mem, i: u32) -> Geometry {
    let mut geo = Geometry::default();
    let desc = word(m, i + 0x80);
    let sc = f32::from_bits(word(m, i + 0x84)) as f64;
    if !ram(desc) || !sc.is_finite() {
        return geo;
    }
    let ds = |k: u32| f32::from_bits(word(m, desc + 0x18 + 4 * k)) as f64 * sc;
    let s = [
        [ds(0), 0.0, 0.0, 0.0],
        [0.0, ds(1), 0.0, 0.0],
        [0.0, 0.0, ds(2), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let w = matrix(m, i + 0x10);
    let lit = word(m, i + 8) & 0x4000 != 0;
    let base = if lit {
        word(m, desc + 0x24)
    } else {
        word(m, i + 0x98)
    };
    let count = word(m, desc + 4) as usize;
    let nodes = word(m, desc + 8);
    if count == 0 || count > 64 || !ram(nodes) {
        return geo;
    }
    let mats = word(m, i + 0x94);
    let mut n: Vec<[[f64; 4]; 4]> = Vec::with_capacity(count);
    for k in 0..count {
        let e = nodes + 16 * k as u32;
        let parent = word(m, e) as i32;
        let la = word(m, e + 12);
        let mut l = if ram(la) {
            matrix(m, la)
        } else {
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };
        for c in 0..3 {
            l[3][c] *= sc;
        }
        let nk = if parent >= 0 && (parent as usize) < k {
            mul(&l, &n[parent as usize])
        } else {
            l
        };
        n.push(nk);
    }
    for (k, nk) in n.iter().enumerate() {
        let ml = word(m, nodes + 16 * k as u32 + 4);
        if !ram(ml) {
            continue;
        }
        let h = mul(&mul(&s, nk), &w);
        let groups = word(m, ml + 0x1c).min(256);
        let list = word(m, ml + 0x20);
        for g in 0..groups {
            let ga = word(m, list + 4 * g);
            if !ram(ga) {
                continue;
            }
            let gw = word(m, ga);
            let material = gw as u16 as i16;
            // Alpha-tested cut-outs (trees): with their texture, by id.
            let mut cutout: Option<i16> = None;
            if ram(mats) && material >= 0 && (material as u32) < word(m, mats) {
                let mat = word(m, mats + 4 + 4 * material as u32);
                if ram(mat) {
                    match word(m, mat + 0xc) & 0x66_0000 {
                        0x4_0000 | 0x6_0000 => continue,
                        0x2_0000 => cutout = Some(word(m, mat) as u16 as i16).filter(|&t| t >= 0),
                        _ => {}
                    }
                }
            }
            let chain = base.wrapping_add(word(m, ga + 4));
            for (lens, xyz, uv) in batches(&chain_words(m, chain)) {
                let world: Vec<[f32; 3]> = xyz
                    .iter()
                    .map(|p| {
                        let v = [
                            p[0] as f64 / 32768.0,
                            p[1] as f64 / 32768.0,
                            p[2] as f64 / 32768.0,
                            1.0,
                        ];
                        std::array::from_fn(|c| (0..4).map(|r| v[r] * h[r][c]).sum::<f64>() as f32)
                    })
                    .collect();
                // The strips' triangles (vertex indices; a short last strip
                // would leave a partial triangle).
                let mut tris: Vec<usize> = Vec::new();
                let mut at = 0usize;
                for len in lens {
                    let len = len as usize;
                    for t in 0..len.saturating_sub(2) {
                        if at + t + 2 < world.len() {
                            tris.extend([at + t, at + t + 1, at + t + 2]);
                        }
                    }
                    at += len;
                }
                match cutout {
                    Some(t) if uv.len() == world.len() => {
                        if geo.cut.last().is_none_or(|c| c.0 != t) {
                            geo.cut.push((t, Vec::new()));
                        }
                        let c = &mut geo.cut.last_mut().unwrap().1;
                        c.extend(tris.iter().map(|&k| {
                            let (v, u) = (world[k], uv[k]);
                            [v[0], v[1], v[2], u[0] as f32 / 4096.0, u[1] as f32 / 4096.0]
                        }));
                    }
                    _ => geo.solid.extend(tris.iter().map(|&k| {
                        let v = world[k];
                        [v[0], v[1], v[2], 0.0, 0.0, 0.0]
                    })),
                }
            }
        }
    }
    geo
}

/// The VIF words a group's DMA chain transfers (tag upper halves, then each
/// transfer's data), up to its RET/END.
fn chain_words(m: &dyn Mem, mut t: u32) -> Vec<u32> {
    let mut w = Vec::new();
    for _ in 0..256 {
        if !ram(t & 0x1ff_fff0) {
            break;
        }
        let tag = m.qword(t);
        let qwc = (tag & 0xffff) as u32;
        let id = ((tag >> 28) & 7) as u32;
        let addr = ((tag >> 32) & 0x7fff_fff0) as u32;
        w.push((tag >> 64) as u32);
        w.push((tag >> 96) as u32);
        let data = match id {
            1 | 2 => t + 16,
            3 | 4 => addr,
            _ => break,
        };
        for k in 0..qwc {
            let q = m.qword(data + 16 * k);
            w.extend((0..4).map(|l| (q >> (32 * l)) as u32));
        }
        t = match id {
            1 => t + 16 + 16 * qwc,
            2 => addr,
            _ => t + 16,
        };
        if id == 4 || w.len() > 1 << 20 {
            break;
        }
    }
    w
}

/// The pass-1 batches of a type-3 mesh chain: per batch (a V4-32 ×2 header
/// up to its MSCAL), its strip lengths and positions.
/// A batch: strip lengths, positions, UVs (4.12; empty if none came).
type BatchData = (Vec<u8>, Vec<[i16; 3]>, Vec<[i16; 2]>);

fn batches(w: &[u32]) -> Vec<BatchData> {
    let mut out = Vec::new();
    // (lens, positions, base address, UVs)
    #[allow(clippy::type_complexity)]
    let mut cur: Option<(
        Option<Vec<u8>>,
        Option<Vec<[i16; 3]>>,
        u32,
        Option<Vec<[i16; 2]>>,
    )> = None;
    let mut i = 0;
    while i < w.len() {
        let c = w[i];
        let (cmd, num, imm) = ((c >> 24) & 0x7f, (c >> 16) & 0xff, c & 0xffff);
        i += 1;
        if cmd >= 0x60 {
            let (vn, vl) = ((cmd >> 2) & 3, cmd & 3);
            let n = if num == 0 { 256 } else { num } as usize;
            let size = if vn == 3 && vl == 3 {
                (n * 2).div_ceil(4)
            } else {
                (n * (vn as usize + 1) * (32 >> vl)).div_ceil(32)
            };
            let raw = &w[i.min(w.len())..(i + size).min(w.len())];
            i += size;
            let bytes: Vec<u8> = raw.iter().flat_map(|x| x.to_le_bytes()).collect();
            match (vn, vl) {
                (3, 0) if n == 2 => cur = Some((None, None, imm & 0x3ff, None)),
                (1, 1) => {
                    if let Some((_, _, _, uv @ None)) = cur.as_mut() {
                        *uv = Some(
                            (0..n)
                                .map(|k| {
                                    std::array::from_fn(|l| {
                                        bytes
                                            .get(4 * k + 2 * l..4 * k + 2 * l + 2)
                                            .map_or(0, |b| i16::from_le_bytes([b[0], b[1]]))
                                    })
                                })
                                .collect(),
                        );
                    }
                }
                (1, 2) => {
                    if let Some((lens @ None, _, _, _)) = cur.as_mut() {
                        *lens = Some(
                            (0..n)
                                .map(|k| bytes.get(2 * k).copied().unwrap_or(0))
                                .collect(),
                        );
                    }
                }
                (2, 1) => {
                    if let Some((Some(_), pos @ None, _, _)) = cur.as_mut() {
                        *pos = Some(
                            (0..n)
                                .map(|k| {
                                    std::array::from_fn(|l| {
                                        bytes
                                            .get(6 * k + 2 * l..6 * k + 2 * l + 2)
                                            .map_or(0, |b| i16::from_le_bytes([b[0], b[1]]))
                                    })
                                })
                                .collect(),
                        );
                    }
                }
                _ => {}
            }
        } else {
            match cmd {
                0x20 => i += 1,
                0x30 | 0x31 => i += 4,
                0x50 | 0x51 => i += (if imm == 0 { 65536 } else { imm as usize }) * 4,
                // MSCAL / MSCALF: the batch is complete.
                0x14 | 0x15 => {
                    if let Some((Some(lens), Some(pos), _, uv)) = cur.take() {
                        out.push((lens, pos, uv.unwrap_or_default()));
                    }
                }
                _ => {}
            }
        }
    }
    out
}

/// A hash of the patch's coefficients (a patch slot reused by another
/// patch tessellates again).
fn coeff_hash(m: &dyn Mem, p: u32) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for i in 0..16 {
        let q = m.qword(p + PATCH_COEFFS + 16 * i);
        h = (h ^ q as u64).wrapping_mul(0x100_0000_01b3);
        h = (h ^ (q >> 64) as u64).wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// The patch's triangles on a [`GRID`]² grid (none for coefficients that
/// are not a plausible patch).
fn tessellate(m: &dyn Mem, p: u32) -> Vec<[f32; 6]> {
    let c: Vec<[f64; 3]> = (0..16)
        .map(|i| {
            let q = m.qword(p + PATCH_COEFFS + 16 * i);
            std::array::from_fn(|l| f32::from_bits((q >> (32 * l)) as u32) as f64)
        })
        .collect();
    if c.iter().flatten().any(|v| !v.is_finite() || v.abs() > 1e7) {
        return Vec::new();
    }
    let n = GRID;
    let pow = |t: f64| [t * t * t, t * t, t, 1.0];
    let pts: Vec<[f32; 3]> = (0..n * n)
        .map(|i| {
            let (bu, bv) = (
                pow((i / n) as f64 / (n - 1) as f64),
                pow((i % n) as f64 / (n - 1) as f64),
            );
            let mut s = [0f64; 3];
            for k in 0..4 {
                for j in 0..4 {
                    let w = bu[k] * bv[j];
                    for l in 0..3 {
                        s[l] += w * c[4 * k + j][l];
                    }
                }
            }
            s.map(|v| v as f32)
        })
        .collect();
    let mut t = Vec::with_capacity((n - 1) * (n - 1) * 6);
    let mut tri = |a: [f32; 3], b: [f32; 3], c: [f32; 3]| {
        let (u, v) = (sub(b, a), sub(c, a));
        let mut nrm = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        let l = (nrm[0] * nrm[0] + nrm[1] * nrm[1] + nrm[2] * nrm[2]).sqrt();
        if l <= 0.0 || !l.is_finite() {
            return;
        }
        let s = if nrm[2] < 0.0 { -1.0 / l } else { 1.0 / l };
        nrm = nrm.map(|x| x * s);
        for p in [a, b, c] {
            t.push([p[0], p[1], p[2], nrm[0], nrm[1], nrm[2]]);
        }
    };
    for a in 0..n - 1 {
        for b in 0..n - 1 {
            let (p00, p01, p10, p11) = (
                pts[a * n + b],
                pts[a * n + b + 1],
                pts[(a + 1) * n + b],
                pts[(a + 1) * n + b + 1],
            );
            tri(p00, p10, p11);
            tri(p00, p11, p01);
        }
    }
    t
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Memory of qwords at addresses.
    struct M(HashMap<u32, u128>);
    impl Mem for M {
        fn qword(&self, a: u32) -> u128 {
            self.0.get(&a).copied().unwrap_or(0)
        }
    }

    fn q(v: [f32; 4]) -> u128 {
        v.iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc | (x.to_bits() as u128) << (32 * i))
    }

    #[test]
    fn corners_of_a_power_basis_patch() {
        // P(u, v) = C15 + u·(10, 0, 0) + v·(0, 20, 0): C[11] (u¹v⁰), C[14] (u⁰v¹).
        let p = 0x20_0000;
        let mut mem = HashMap::new();
        for i in 0..16 {
            mem.insert(p + PATCH_COEFFS + 16 * i, q([0.0, 0.0, 0.0, 1.0]));
        }
        mem.insert(p + PATCH_COEFFS + 16 * 15, q([100.0, 200.0, 300.0, 1.0]));
        mem.insert(p + PATCH_COEFFS + 16 * 11, q([10.0, 0.0, 0.0, 1.0]));
        mem.insert(p + PATCH_COEFFS + 16 * 14, q([0.0, 20.0, 0.0, 1.0]));
        let t = tessellate(&M(mem), p);
        assert_eq!(t.len(), (GRID - 1) * (GRID - 1) * 6);
        assert_eq!(t[0][..3], [100.0, 200.0, 300.0]);
        let last = t.iter().fold([0f32; 3], |m, v| {
            [m[0].max(v[0]), m[1].max(v[1]), m[2].max(v[2])]
        });
        assert_eq!(last, [110.0, 220.0, 300.0]);
        // A flat patch in z = 300: every normal +z.
        assert!(t.iter().all(|v| v[3..] == [0.0, 0.0, 1.0]));
    }
}
