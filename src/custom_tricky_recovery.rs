//! Tricky recovery uses its current route as the positional reference.
//! Keep route elevation when placing a recovered rider, including AI.

use ssx3_ee_rt::cpu::reg::{A1, RA, S1, S2};
use ssx3_ee_rt::Cpu;

fn valid(at: u32, bytes: u32) -> bool {
    at >= 0x10000 && at.checked_add(bytes).is_some_and(|end| end <= 0x0200_0000)
}

fn float(c: &mut Cpu, at: u32) -> f32 { f32::from_bits(c.bus.read32(at)) }

/// Native Path_GetPointAtDistance's path layout. Tricky118c08 seeds its
/// recovery search from this path point, not the fallen rider position.
fn path_point(c: &mut Cpu, path: u32, mut distance: f32) -> Option<[f32; 3]> {
    if !valid(path, 0x40) || !distance.is_finite() { return None }
    let count = c.bus.read32(path + 8);
    let segments = c.bus.read32(path + 0x18);
    if count == 0 || count > 100_000 || !valid(segments, count.checked_mul(16)?) {
        return None;
    }
    let mut point = std::array::from_fn(|i| float(c, path + 0xc + i as u32 * 4));
    let mut last = [0.; 3];
    for index in 0..count {
        let segment = segments + index * 16;
        let direction: [f32; 3] = std::array::from_fn(|i| float(c, segment + i as u32 * 4));
        let length = float(c, segment + 12);
        if !length.is_finite() || length < 0. || !direction.iter().all(|v| v.is_finite()) {
            return None;
        }
        let step = distance.min(length);
        for axis in 0..3 { point[axis] += direction[axis] * step; }
        if distance < length { return point.iter().all(|v| v.is_finite()).then_some(point) }
        distance -= length;
        last = direction;
    }
    // Tricky118c9c also extrapolates the final direction beyond a path end.
    for axis in 0..3 { point[axis] += last[axis] * distance; }
    point.iter().all(|v| v.is_finite()).then_some(point)
}

pub fn seed_query(c: &mut Cpu, rider: u32) {
    if crate::custom_tracks::active().is_none() { return }
    let path = c.bus.read32(rider + 0xab8);
    let distance = float(c, rider + 0x4c4);
    let point = path_point(c, path, distance).unwrap_or_else(||
        std::array::from_fn(|i| float(c, rider + 0x110 + i as u32 * 4)));
    // SnapToNearestAIPath replaces this look-ahead output after its search.
    // Until then it holds the immutable query seed, separate from its
    // candidate/final point output at+490 and from the actual rider pose.
    for axis in 0..3 { c.bus.write32(rider + 0x4a0 + axis * 4, point[axis as usize].to_bits()); }
    c.bus.write32(rider + 0x4ac, 1f32.to_bits());
}

pub fn query_position(rider: u32) -> u32 {
    rider + if crate::custom_tracks::active().is_some() { 0x4a0 } else { 0x110 }
}

pub fn ahead_bits() -> u32 {
    // Source Tricky uses800cm; the native SSX3 path uses796cm.
    if crate::custom_tracks::active().is_some() { 800f32.to_bits() } else { 796f32.to_bits() }
}

pub fn prepare_ground_query(c: &mut Cpu) {
    if crate::custom_tracks::active().is_some() && c.g[RA][0] as u32 == 0x0011_d814 {
        let rider = c.g[S1][0] as u32;
        if valid(rider, 0xac4) && c.g[S2][0] as u32 == rider + 0x490 {
            let motion = c.bus.read32(rider + 0x77c);
            if valid(motion, 0xde4) && c.bus.read32(motion + 0xde0) == 3 {
                let start = c.g[A1][0] as u32;
                let height = float(c, rider + 0x498);
                let clearance = f32::from_bits(c.f[23]);
                if valid(start, 16) && height.is_finite() && clearance.is_finite() && clearance >= 0. {
                    // Source11cc8c..11ccd4: [pathZ-clearance-200, pathZ+200],
                    // choosing from the lower endpoint (reference0). Native
                    // Rider_Place otherwise starts7000cm below the path.
                    // Preserve native clearance, end point, real contacts,
                    // ground tilt and the no-hit path-height fallback.
                    c.bus.write32(start + 8, (height - (clearance + 200.)).to_bits());
                    c.f[12] = 0;
                }
            }
        }
    }
}
