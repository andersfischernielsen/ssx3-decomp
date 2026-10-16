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

/// A frame's matrix records, each with the `alt` keys (state and textures)
/// of the draws that use it.
#[derive(Clone, Default)]
pub struct Records {
    pub records: Vec<(MatrixRecord, Vec<u64>)>,
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
    let mut by_alt: HashMap<u64, Vec<usize>> = HashMap::new();
    for (j, (r, alts)) in prev.records.iter().enumerate() {
        by_world.entry(bits(&r.world)).or_default().push(j);
        for a in alts {
            by_alt.entry(*a).or_default().push(j);
        }
    }
    cur.records
        .iter()
        .map(|(r, alts)| {
            if let Some(js) = by_world.get(&bits(&r.world)) {
                return js.iter().copied().min_by(|&a, &b| dist(&prev.records[a].0.view, &r.view).total_cmp(&dist(&prev.records[b].0.view, &r.view)));
            }
            let p = position(&r.world);
            let d = |j: usize| {
                let q = position(&prev.records[j].0.world);
                (0..3).map(|k| (p[k] - q[k]) * (p[k] - q[k])).sum::<f32>()
            };
            alts.iter().filter_map(|a| by_alt.get(a)).flatten().copied().min_by(|&a, &b| d(a).total_cmp(&d(b)))
        })
        .collect()
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
    fn static_objects_pair_by_world_riders_by_state_and_place() {
        let prev = Records { records: vec![(rec(I, at(0.0)), vec![1]), (rec(at(10.0), at(0.0)), vec![2]), (rec(at(50.0), at(0.0)), vec![2])] };
        let cur = Records { records: vec![(rec(at(52.0), at(1.0)), vec![2]), (rec(I, at(1.0)), vec![1]), (rec(at(11.0), at(1.0)), vec![2]), (rec(at(5.0), at(1.0)), vec![3])] };
        assert_eq!(pair(&prev, &cur), [Some(2), Some(0), Some(1), None]);
    }

    #[test]
    fn split_screen_pairs_the_closest_view() {
        let prev = Records { records: vec![(rec(I, at(0.0)), vec![]), (rec(I, at(100.0)), vec![])] };
        let cur = Records { records: vec![(rec(I, at(101.0)), vec![]), (rec(I, at(1.0)), vec![])] };
        assert_eq!(pair(&prev, &cur), [Some(1), Some(0)]);
    }

    #[test]
    fn headers_are_replaced_by_the_blend() {
        let prev = Records { records: vec![(rec(I, at(0.0)), vec![])] };
        let cur = Records { records: vec![(rec(I, at(4.0)), vec![])] };
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
