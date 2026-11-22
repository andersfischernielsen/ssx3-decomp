//! Source Tricky board-contact volume for imported plain grind splines.
//!
//! Tricky125cb8/125dd0..125fcc offsets its oriented board box by30cm along
//! local Y and tests its three axes. SSX3's1086b8 instead uses an unshifted
//! capsule. Preserve native hits and add the source box only after a native
//! miss, at Rider_FindRail's ordinary attach call. Native eligibility, input
//! states, closest point, stance, velocity and railslide handling stay native.

use ssx3_ee_rt::cpu::reg::{A0, A1, RA, SP, V0};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_86b8, probe_rail, "Rider_ProbeRail_TrickyContact"),
];

fn valid(at: u32, size: u32) -> bool {
    at >= 0x10000 && at.checked_add(size).is_some_and(|end| end <= 0x0200_0000)
}

fn float(c: &mut Cpu, at: u32) -> f32 {
    f32::from_bits(c.bus.read32(at))
}

fn source_box_contains(c: &mut Cpu, rider: u32, hit: u32) -> bool {
    if !valid(rider, 0x9e0) || !valid(hit, 0x74) {
        return false;
    }
    // Kind2 is a plain WorldSpline contact. Do not extend native modifier,
    // flex-rail or object contacts, even inside an imported course.
    if c.bus.read32(hit + 0x44) != 2 {
        return false;
    }
    let spline = c.bus.read32(hit + 0x58);
    if !valid(spline, 0x30)
        || !crate::custom_tracks::owns_world_resource(c.bus.read32(spline))
        || c.bus.read32(spline + 0x1c) & 1 == 0
    {
        return false;
    }
    let animation = c.bus.read32(rider + 0x780);
    if !valid(animation, 0x30) {
        return false;
    }
    let bones = c.bus.read32(animation + 0x2c);
    let index = c.bus.read32(rider + 0x8a0);
    let Some(bone) = index.checked_mul(32).and_then(|offset| bones.checked_add(offset)) else {
        return false;
    };
    if !valid(bone, 32) {
        return false;
    }
    let point: [f32; 3] = std::array::from_fn(|i| float(c, hit + i as u32 * 4));
    let origin: [f32; 3] = std::array::from_fn(|i|
        float(c, bone + i as u32 * 4) + float(c, rider + 0x9d0 + i as u32 * 4));
    let q: [f32; 4] = std::array::from_fn(|i| float(c, bone + 16 + i as u32 * 4));
    let body_blend = float(c, rider + 0x25c);
    if !point.iter().chain(origin.iter()).chain(q.iter()).all(|v| v.is_finite())
        || !(0. ..=1.).contains(&body_blend)
    {
        return false;
    }
    let [x, y, z, w] = q;
    // The same board-quaternion basis used by native ProbeRail and stance:
    // X forward, Y board normal, Z across the board.
    let axes = [
        [1. - 2. * (y*y + z*z), 2. * (x*y + w*z), 2. * (x*z - w*y)],
        [2. * (x*y - w*z), 1. - 2. * (x*x + z*z), 2. * (y*z + w*x)],
        [2. * (x*z + w*y), 2. * (y*z - w*x), 1. - 2. * (x*x + y*y)],
    ];
    let delta: [f32; 3] = std::array::from_fn(|i|
        point[i] - origin[i] - 30. * axes[1][i]);
    let other_pose = 1. - body_blend;
    // Source125ef0..125fcc: forward80..150, normal50, across30..50,
    // all contracted by0.9. These are source dimensions, not snap tuning.
    let half_extents = [
        (150. * body_blend + 80. * other_pose) * 0.9,
        (50. * body_blend + 50. * other_pose) * 0.9,
        (50. * body_blend + 30. * other_pose) * 0.9,
    ];
    axes.iter().zip(half_extents).all(|(axis, extent)|
        axis.iter().zip(delta).map(|(a, d)| a*d).sum::<f32>().abs() < extent)
}

fn probe_rail(c: &mut Cpu) {
    let (rider, hit, ret, stack) = (
        c.g[A0][0] as u32, c.g[A1][0] as u32,
        c.g[RA][0] as u32, c.g[SP][0] as u32,
    );
    ssx3_ee::s00::f_001086b8__generated(c);
    if c.pc == ret && c.g[SP][0] as u32 == stack && c.g[V0][0] == 0
        && ret == 0x0010_8c10 && crate::custom_tracks::active().is_some()
        && source_box_contains(c, rider, hit)
    {
        c.g[V0][0] = 1;
    }
}
