//! Frame interpolation: images between two game frames, for displays
//! faster than the game's 60 Hz.
//!
//! An in-between image is frame N drawn again with its objects' matrices
//! moved back towards frame N-1: every draw of the 3D scene runs its VU1
//! program with the VU header (worldView, qw0-3: `MatrixRecord::world_view`)
//! the program reads. Each of frame N's MatrixRecords is matched with the
//! same object's record in frame N-1, and while the VU programs run, a
//! header equal to a matched record's worldView is replaced by
//! `lerp(prev, cur, t)` (componentwise; between consecutive frames that is
//! as good as a proper rotation blend). Culling and clipping run on the
//! moved geometry, so every image is consistent in itself; states,
//! textures, post passes and screen-space draws (HUD, flares) are frame
//! N's.
//!
//! Matching a record of frame N:
//! 1. a record of N-1 with the same world matrix (static objects: only the
//!    camera moved), the one whose view is closest (split screen);
//! 2. else, among the records of N-1 used by draws with the same state and
//!    textures (`alt` keys), the one whose world position is closest
//!    (riders, moving objects);
//! 3. else none: the object stays where N has it.

use ssx3_render::frame::MatrixRecord;
use std::collections::HashMap;

pub type M4 = [[f32; 4]; 4];

/// A frame's matrix records, each with the draws that use it: `(alt, n)`
/// = the n-th draw of the frame with that state and textures.
#[derive(Clone, Default)]
pub struct Records {
    pub records: Vec<(MatrixRecord, Vec<(u64, u32)>)>,
    /// Mesh model matrices (General mesh setups, VU TOP qw0-3) by (draw,
    /// n-th setup in it): blended like the records for objects that move
    /// (the sky and backdrop follow the camera; props).
    pub models: HashMap<((u64, u32), u32), [u32; 16]>,
}

fn bits(m: &M4) -> [u32; 16] {
    let mut b = [0; 16];
    for (i, r) in m.iter().enumerate() {
        for (j, v) in r.iter().enumerate() {
            b[i * 4 + j] = v.to_bits();
        }
    }
    b
}

fn dist(a: &M4, b: &M4) -> f32 {
    a.iter().flatten().zip(b.iter().flatten()).map(|(x, y)| (x - y).abs()).sum()
}

fn position(m: &M4) -> [f32; 3] {
    [m[3][0], m[3][1], m[3][2]]
}

/// For each record of `cur`, its counterpart in `prev`.
pub fn pair(prev: &Records, cur: &Records) -> Vec<Option<usize>> {
    let mut by_world: HashMap<[u32; 16], Vec<usize>> = HashMap::new();
    let mut by_draw: HashMap<(u64, u32), usize> = HashMap::new();
    let mut by_alt: HashMap<u64, Vec<usize>> = HashMap::new();
    for (j, (r, draws)) in prev.records.iter().enumerate() {
        by_world.entry(bits(&r.world)).or_default().push(j);
        for &(a, n) in draws {
            by_draw.entry((a, n)).or_insert(j);
            by_alt.entry(a).or_default().push(j);
        }
    }
    cur.records
        .iter()
        .map(|(r, draws)| {
            if let Some(js) = by_world.get(&bits(&r.world)) {
                return js.iter().copied().min_by(|&a, &b| dist(&prev.records[a].0.view, &r.view).total_cmp(&dist(&prev.records[b].0.view, &r.view)));
            }
            // A moving object: the record of the same draw (n-th with this
            // state and textures) last frame, else the nearest record of a
            // draw with this state and textures.
            if let Some(&j) = draws.iter().find_map(|d| by_draw.get(d)) {
                return Some(j);
            }
            let p = position(&r.world);
            let d = |j: usize| {
                let q = position(&prev.records[j].0.world);
                (0..3).map(|k| (p[k] - q[k]) * (p[k] - q[k])).sum::<f32>()
            };
            draws.iter().filter_map(|(a, _)| by_alt.get(a)).flatten().copied().min_by(|&a, &b| d(a).total_cmp(&d(b)))
        })
        .collect()
}

/// Two matrices (as words) within a quarter of the second's size of each
/// other: the same moving object a frame apart.
pub fn close(a: &[u32; 16], b: &[u32; 16]) -> bool {
    let f = |w: &[u32; 16], k: usize| f32::from_bits(w[k]) as f64;
    let (mut d, mut n) = (0.0, 0.0);
    for k in 0..16 {
        d += (f(a, k) - f(b, k)).powi(2);
        n += f(b, k).powi(2);
    }
    d <= 0.0625 * n
}

/// Mesh model matrix substitutions for the image at `t`: this frame's
/// model (words) → the blend from the same mesh's last frame, for meshes
/// whose model moved (a little).
pub fn model_substitutions(prev: &Records, cur: &Records, t: f32) -> HashMap<[u32; 16], [u32; 16]> {
    let m4 = |w: &[u32; 16]| -> M4 { std::array::from_fn(|i| std::array::from_fn(|j| f32::from_bits(w[i * 4 + j]))) };
    let mut m = HashMap::new();
    for (k, now) in &cur.models {
        let Some(p) = prev.models.get(k) else { continue };
        if p == now || !close(p, now) {
            continue;
        }
        let l = lerp(&m4(p), &m4(now), t);
        m.entry(*now).or_insert_with(|| bits(&l));
    }
    m
}

pub fn lerp(a: &M4, b: &M4, t: f32) -> M4 {
    let mut m = *b;
    for i in 0..4 {
        for j in 0..4 {
            m[i][j] = a[i][j] + (b[i][j] - a[i][j]) * t;
        }
    }
    m
}

/// VU header substitutions for the image at `t` (0 = frame N-1, 1 = N):
/// frame N's worldView bits → the interpolated worldView bits.
pub fn substitutions(prev: &Records, cur: &Records, pairs: &[Option<usize>], t: f32) -> HashMap<[u32; 16], [u32; 16]> {
    let mut m = HashMap::new();
    for ((r, _), p) in cur.records.iter().zip(pairs) {
        let Some(j) = *p else { continue };
        let from = &prev.records[j].0.world_view;
        if from == &r.world_view {
            continue;
        }
        m.entry(bits(&r.world_view)).or_insert_with(|| bits(&lerp(from, &r.world_view, t)));
    }
    m
}

/// Apply a substitution to a VU memory image's header (qw0-3).
pub fn substitute(mem: &mut [u8], subst: &HashMap<[u32; 16], [u32; 16]>) {
    if subst.is_empty() {
        return;
    }
    let mut key = [0u32; 16];
    for (i, k) in key.iter_mut().enumerate() {
        *k = u32::from_le_bytes(mem[i * 4..i * 4 + 4].try_into().unwrap());
    }
    if let Some(v) = subst.get(&key) {
        for (i, w) in v.iter().enumerate() {
            mem[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I: M4 = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];

    fn at(x: f32) -> M4 {
        let mut m = I;
        m[3][0] = x;
        m
    }

    fn rec(world: M4, view: M4) -> MatrixRecord {
        // (worldView is world · view here; only identity matters.)
        let mut wv = world;
        wv[3][0] += view[3][0];
        MatrixRecord { world, view, world_view: wv, guard_scale: [0.0; 4], guard_offset: [0.0; 4], view_proj: I, world_clip: I, screen_scale: [0.0; 4], screen_offset: [0.0; 4] }
    }

    #[test]
    fn static_objects_pair_by_world_moving_ones_by_draw_then_place() {
        let prev = Records { models: HashMap::new(), records: vec![(rec(I, at(0.0)), vec![(1, 0)]), (rec(at(10.0), at(0.0)), vec![(2, 0)]), (rec(at(50.0), at(0.0)), vec![(2, 1)])] };
        let cur = Records {
            models: HashMap::new(),
            records: vec![
                // Same draw (2, 1): paired whatever the order.
                (rec(at(52.0), at(1.0)), vec![(2, 1)]),
                (rec(I, at(1.0)), vec![(1, 0)]),
                (rec(at(11.0), at(1.0)), vec![(2, 0)]),
                // No such draw last frame: the nearest of its kind.
                (rec(at(48.0), at(1.0)), vec![(2, 7)]),
                (rec(at(5.0), at(1.0)), vec![(3, 0)]),
            ],
        };
        assert_eq!(pair(&prev, &cur), [Some(2), Some(0), Some(1), Some(2), None]);
    }

    #[test]
    fn split_screen_pairs_the_closest_view() {
        let prev = Records { models: HashMap::new(), records: vec![(rec(I, at(0.0)), vec![]), (rec(I, at(100.0)), vec![])] };
        let cur = Records { models: HashMap::new(), records: vec![(rec(I, at(101.0)), vec![]), (rec(I, at(1.0)), vec![])] };
        assert_eq!(pair(&prev, &cur), [Some(1), Some(0)]);
    }

    #[test]
    fn headers_are_replaced_by_the_blend() {
        let prev = Records { models: HashMap::new(), records: vec![(rec(I, at(0.0)), vec![])] };
        let cur = Records { models: HashMap::new(), records: vec![(rec(I, at(4.0)), vec![])] };
        let s = substitutions(&prev, &cur, &pair(&prev, &cur), 0.25);
        let mut mem = vec![0u8; 64];
        for (i, v) in cur.records[0].0.world_view.iter().flatten().enumerate() {
            mem[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        substitute(&mut mem, &s);
        let x = f32::from_le_bytes(mem[48..52].try_into().unwrap());
        assert_eq!(x, 1.0);
    }
}
