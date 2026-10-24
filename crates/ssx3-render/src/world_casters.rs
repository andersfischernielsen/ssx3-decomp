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
    pub patches: u32,
}

/// The tessellated patches, kept from frame to frame.
#[derive(Default)]
pub struct CasterCache {
    meshes: HashMap<u32, (u64, Vec<[f32; 6]>)>,
    set: Vec<(u32, u64)>,
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
            slot >= 0 && word(m, sections + section * 8 + 0x24) == 6 && word(m, sections.wrapping_add((slot as u32) * 0x18) + 0x3f0) == 3
        };
        let mut set = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut stack: Vec<(u32, u32)> = (0..8).map(|k| (word(m, nash + 0x14 * k + 0x10), 0)).collect();
        while let Some((node, depth)) = stack.pop() {
            if !ram(node) || depth > 40 || !seen.insert(node) {
                continue;
            }
            let mut p = word(m, node + 0x24);
            let mut n = 0;
            while ram(p) && n < 100_000 {
                if shown(p) {
                    set.push((p, coeff_hash(m, p)));
                }
                p = word(m, p);
                n += 1;
            }
            stack.extend((0..8).map(|i| (word(m, node + 4 * i), depth + 1)));
        }
        set.sort_unstable();
        if set == self.set {
            return self.current.clone();
        }
        // The set changed: tessellate what is new, drop what is gone.
        let keep: std::collections::HashSet<u32> = set.iter().map(|&(p, _)| p).collect();
        self.meshes.retain(|p, _| keep.contains(p));
        let mut triangles = Vec::new();
        for &(p, h) in &set {
            let fresh = self.meshes.get(&p).is_none_or(|(mh, _)| *mh != h);
            if fresh {
                self.meshes.insert(p, (h, tessellate(m, p)));
            }
            triangles.extend_from_slice(&self.meshes[&p].1);
        }
        self.generation += 1;
        self.set = set;
        self.current = Some(Arc::new(Casters { generation: self.generation, patches: self.set.len() as u32, triangles }));
        self.current.clone()
    }
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
            let (bu, bv) = (pow((i / n) as f64 / (n - 1) as f64), pow((i % n) as f64 / (n - 1) as f64));
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
        let mut nrm = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
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
            let (p00, p01, p10, p11) = (pts[a * n + b], pts[a * n + b + 1], pts[(a + 1) * n + b], pts[(a + 1) * n + b + 1]);
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
        v.iter().enumerate().fold(0, |acc, (i, x)| acc | (x.to_bits() as u128) << (32 * i))
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
        let last = t.iter().fold([0f32; 3], |m, v| [m[0].max(v[0]), m[1].max(v[1]), m[2].max(v[2])]);
        assert_eq!(last, [110.0, 220.0, 300.0]);
        // A flat patch in z = 300: every normal +z.
        assert!(t.iter().all(|v| v[3..] == [0.0, 0.0, 1.0]));
    }
}
