//! # Type 5 (`e72540c9`) natively: SnowBox snow, bit-exact
//!
//! A readable Rust implementation of the VU1 microprogram with hash
//! `0xe72540c9e7c434ee` (`generated/vu/vu_e72540c9e7c434ee.rs` is the literal
//! translation; `docs/investigation/phase3-render.md` §1.5). [`run`] takes
//! the program's VU1 data memory and produces the same XGKICK packets **and**
//! the same data memory as the translated program, bit for bit (checked on
//! real frames by `SSX3_VERIFY_PARTICLES=1`,
//! `crates/ssx3-ps2/src/particle_verify.rs`).
//!
//! | MSCAL | what |
//! |---|---|
//! | 0x0000 | flakes culled by CLIP (centre only), constant colour |
//! | 0x0408 | no culling; alpha faded by depth (near and far) |
//!
//! ## Memory (qword addresses)
//!
//! Header: qw0 screen scale (z/16), qw1 screen offset, qw2 {P[0].x, P[1].y,
//! 0, 0}. Input at TOP (slots 85/86 `SnowBox`):
//!
//! | qw | contents |
//! |---|---|
//! | T0..T3 | M: box space → clip columns |
//! | T4 / T5 | the sprite's two STQ corners (.z := 1) |
//! | T6 | box offset (.xyz) added to every random point |
//! | T7 | colour (floats; 0x0408 takes alpha from .w) |
//! | T8 | {flake count (int), seed x, seed y, seed z} |
//! | T9 | {box min (.xyz), flake size (.w)} |
//! | T10 | {box max (.xyz), depth scale (.w, 0x0408)} |
//!
//! Output: GIF packets at 0x19 and 0x209 (EE-written SPRITE tags), 99
//! sprites per kick, 5 qw per sprite {ST T4, RGBA, XYZ2 centre+h, ST T5,
//! XYZ2 centre−h}. The last packet is kicked even when empty.
//!
//! ## The flakes
//!
//! Flake k (from 0) sits at `p = LFSR^(k+2)(seed) + T6` per axis (each axis
//! its own sequence: `RINIT v; RNEXT v` from the axis's previous value; the
//! first step happens before the loop), wrapped by one step into
//! [min, max): `p += 1` on each axis where `p − min < 0`, then `p −= 1` on
//! each axis where `p − max ≥ 0` (the LFSR values are in [1, 2), so the box
//! is one unit wide). The MAC sign flags of the two SUBs decide (an
//! `ABS vf0` forces the stall that makes them visible to FMAND).
//! `c = M·(p, 1)`; the sprite half size is `(proj·T9.w·scale)/c.w`, not
//! clamped.
//!
//! Depth fade (0x0408): `d = c.z · (1/T10.w)`; with `e = (p₁.z − max.z)`
//! (p₁ = after the first wrap only — the program reuses that SUB's register;
//! the 0.3 loaded for it is dead):
//! - `e ≥ d` (near): alpha = T7.w · clamp(3.33·d, 0, 1);
//! - else if `d > 0.8` (far): alpha = T7.w · clamp(1 − 3.33·(d − 0.8), 0, 1);
//! - else alpha = T7.w.
//!
//! Inputs whose loop would not end (a flake count of 0) are declined.

use crate::bezier::Mem;
use crate::mat4::{add, bc, div, ftoi0, ftoi4, madd, max, min, mul, mul1, neg, sub, sub1, with_xyz, xform, Lfsr, Packets, V, ONE};

/// Hash of the program this module implements.
pub const HASH: u64 = 0xe725_40c9_e7c4_34ee;

/// Register state the program carries from one call to the next.
///
/// Empty: both entries seed R and write every register before reading it.
#[derive(Clone, Debug, Default)]
pub struct VuState {}

/// Header: screen scale, screen offset, projection x/y.
const SCREEN_SCALE: u16 = 0;
const SCREEN_OFFSET: u16 = 1;
const PROJ: u16 = 2;
/// Output packets.
const PACKET_A: u16 = 0x19;
const PACKET_B: u16 = 0x209;
const SPRITES_PER_KICK: u16 = 99;

const FADE_FAR: u32 = 0x3f4c_cccd; // 0.8
const FADE_SLOPE: u32 = 0x4055_1eb8; // 3.33

/// Entries [`run`] implements natively (MSCAL byte addresses).
pub fn implemented(entry: u32) -> bool {
    matches!(entry, 0x0000 | 0x0408)
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
    let fade = match entry {
        0x0000 => false,
        0x0408 => true,
        _ => return false,
    };
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    let t = |i: u16| m.ld(top.wrapping_add(i));
    let count = t(8)[0] as u16;
    if count == 0 {
        return false;
    }
    let mat: [V; 4] = std::array::from_fn(|i| t(i as u16));
    let st_a = [t(4)[0], t(4)[1], ONE, t(4)[3]];
    let st_b = [t(5)[0], t(5)[1], ONE, t(5)[3]];
    let offset = t(6);
    let colour = t(7);
    let seeds = t(8);
    let (box_min, box_max) = (t(9), t(10));
    let scale = m.ld(SCREEN_SCALE);
    let screen_offset = m.ld(SCREEN_OFFSET);
    // Half size before the divide: proj · T9.w · scale.
    let half = mul(mul(m.ld(PROJ), bc(box_min[3])), scale);
    let rgba = ftoi0(colour);
    let depth_scale = div(ONE, box_max[3]);
    let depth_scale = add(bc(0), bc(depth_scale))[0];
    let far = mul1(ONE, FADE_FAR);

    let mut out = Packets::new(PACKET_A, PACKET_B);
    flakes(seeds, offset, box_min, box_max, count, &mut |p, e, last| {
        let c = xform(&mat, [p[0], p[1], p[2], ONE]);
        let visible = fade || ssx3_vu::vu::fast::clip(c, c[3]) & 0x3f == 0;
        if visible {
            let rgba = if fade {
                let alpha = depth_fade(max(bc(colour[3]), bc(colour[3]))[0], c[2], depth_scale, e, far);
                [rgba[0], rgba[1], rgba[2], ftoi0(bc(alpha))[0]]
            } else {
                rgba
            };
            let acc = mul(screen_offset, bc(ONE));
            let q = div(ONE, c[3]);
            let cq = with_xyz(c, mul(c, bc(q)));
            let h = mul(half, bc(q));
            let centre = madd(acc, cq, scale);
            let hi = add(centre, h);
            let lo = sub(centre, h);
            out.push(&mut m, [st_a, rgba, ftoi4(hi), st_b, ftoi4(lo)]);
            if !last && out.count == SPRITES_PER_KICK {
                out.kick(&mut m, kick);
            }
        }
    });
    out.kick(&mut m, kick);
    true
}

/// Plan a call for the GPU (`crate::gpu::plan`): no memory effects (the
/// LFSR is reseeded from T8 per call; the packets are the program's own).
/// The CPU draws the random points and wraps them into the box; the shader
/// (`vs_mat5_snow`) transforms, culls, fades and makes the sprites.
pub fn plan_gpu(entry: u32, top: u32, _itop: u32, mem: &mut [u8; 16384]) -> Option<Vec<crate::gpu::GpuCall>> {
    use crate::mat4::gpu_particles::*;
    let fade = match entry {
        0x0000 => false,
        0x0408 => true,
        _ => return None,
    };
    let top = (top & 0x3ff) as u16;
    let m = Mem(mem);
    let t = |i: u16| m.ld(top.wrapping_add(i));
    let count = t(8)[0] as u16;
    if count == 0 {
        return None;
    }
    let scale = m.ld(SCREEN_SCALE);
    let screen_offset = m.ld(SCREEN_OFFSET);
    let tag = crate::mat4::gpu_sprite_tag(&m, PACKET_A, PACKET_B, scale, screen_offset)?;
    let colour = t(7);
    let (box_min, box_max) = (t(9), t(10));
    let mut u = [[0u32; 4]; crate::gpu::UNIFORM_QWORDS];
    for i in 0..4 {
        u[S_MATRIX + i] = t(i as u16);
    }
    u[S_SCALE] = scale;
    u[S_OFFSET] = screen_offset;
    u[S_HALF] = mul(mul(m.ld(PROJ), bc(box_min[3])), scale);
    let rgba = ftoi0(colour);
    u[S_RGBA] = rgba;
    let alpha = max(bc(colour[3]), bc(colour[3]))[0];
    let depth_scale = add(bc(0), bc(div(ONE, box_max[3])))[0];
    u[S_FADE] = [alpha, depth_scale, fade as u32, mul1(ONE, FADE_FAR)];
    u[S_ST_A] = [t(4)[0], t(4)[1], ONE, t(4)[3]];
    u[S_ST_B] = [t(5)[0], t(5)[1], ONE, t(5)[3]];
    // Faded alphas are alpha · [0, 1].
    let amax = if !fade {
        rgba[3] & 0xff
    } else {
        match ftoi0(bc(alpha))[0] as i32 {
            a @ 0..=255 => a as u32,
            _ => 255,
        }
    };
    let mut data = Vec::with_capacity(count as usize);
    flakes(t(8), t(6), box_min, box_max, count, &mut |p, e, _| data.push([p[0], p[1], p[2], e]));
    // (Kicked even when empty: the tag still sets PRIM.)
    Some(vec![crate::mat4::sprite_call(crate::gpu::Program::Mat5Snow, tag, u, data, 0..count as u32, amax)])
}

/// The flakes in order (`count` > 0): the wrapped point, `p₁.z − max.z`
/// (the depth fade's e) and whether it is the last.
fn flakes(seeds: V, offset: V, box_min: V, box_max: V, count: u16, each: &mut dyn FnMut(V, u32, bool)) {
    let mut r = [Lfsr::after(seeds[1]), Lfsr::after(seeds[2]), Lfsr::after(seeds[3])];
    for k in 0..count {
        // (The original steps y, x, z: the lanes are independent.)
        for x in r.iter_mut() {
            *x = Lfsr::after(*x);
        }
        let mut p = add([r[0], r[1], r[2], 0], offset);
        let below = sub(p, box_min);
        let up = add(p, bc(ONE));
        for l in 0..3 {
            if neg(below[l]) {
                p[l] = up[l];
            }
        }
        let p1 = p;
        let above = sub(p, box_max);
        let down = sub(p, bc(ONE));
        for l in 0..3 {
            if !neg(above[l]) {
                p[l] = down[l];
            }
        }
        each(p, sub(p1, box_max)[2], k + 1 == count);
    }
}

/// The 0x0408 alpha (see the module doc). `e` is `p₁.z − max.z`.
fn depth_fade(alpha: u32, cz: u32, depth_scale: u32, e: u32, far: u32) -> u32 {
    let d = mul1(cz, depth_scale);
    if !neg(sub1(e, d)) {
        // Near: alpha · clamp(3.33·d, 0, 1).
        let f = mul1(d, FADE_SLOPE);
        let f = min(bc(f), bc(ONE))[0];
        let f = max(bc(f), bc(0))[0];
        mul1(alpha, f)
    } else if neg(sub1(far, d)) {
        // Far: alpha · clamp(1 − 3.33·(d − 0.8), 0, 1).
        let x = sub1(d, far);
        let x = mul1(x, FADE_SLOPE);
        let g = sub1(ONE, x);
        let g = min(bc(g), bc(ONE))[0];
        let g = max(bc(g), bc(0))[0];
        mul1(alpha, g)
    } else {
        max(bc(alpha), bc(alpha))[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat4::tests::{check_plan, compare, put, tag, translated, Rng};

    #[test]
    fn random_inputs_match_translated() {
        let mut r = Rng(0x0bad_cafe_f00d_1234);
        let mut agree = std::collections::BTreeMap::new();
        let f = |x: f32| x.to_bits();
        for case in 0..300 {
            for entry in [0x0000u32, 0x0408] {
                let top: u16 = if case % 2 == 0 { 3 } else { 0xb };
                let mut mem = Box::new([0u8; 16384]);
                put(&mut mem, 0, [f(256.0), f(224.0), f(-8388467.5 / 16.0), 0]);
                put(&mut mem, 1, [f(2047.5), f(2047.5), f(8388467.5 / 16.0), 0]);
                put(&mut mem, 2, [r.f(1.0, 2.0), r.f(1.0, 2.0), 0, 0]);
                put(&mut mem, PACKET_A, tag());
                put(&mut mem, PACKET_B, tag());
                let t = |i: u16| top + i;
                put(&mut mem, t(0), [r.f(1.0, 2.5), 0, 0, 0]);
                put(&mut mem, t(1), [0, r.f(1.0, 2.5), 0, 0]);
                put(&mut mem, t(2), [r.f(-0.3, 0.3), r.f(-0.3, 0.3), r.f(0.8, 1.2), f(1.0)]);
                put(&mut mem, t(3), [r.f(-1.0, 0.0), r.f(-1.0, 0.0), r.f(-0.5, 0.0), r.f(1.0, 2.0)]);
                put(&mut mem, t(4), r.v(0.0, 1.0, 1.0));
                put(&mut mem, t(5), r.v(0.0, 1.0, 1.0));
                put(&mut mem, t(6), r.v(-2.5, 0.0, 0.0));
                put(&mut mem, t(7), r.v(0.0, 255.0, 128.0));
                put(&mut mem, t(8), [(r.u() % 400 + 1) as u32, r.u() as u32, r.u() as u32, r.u() as u32]);
                put(&mut mem, t(9), [f(-1.0), f(-1.0), f(-1.0), r.f(0.005, 0.05)]);
                put(&mut mem, t(10), [f(0.0), f(0.0), f(0.0), r.f(0.5, 2.0)]);
                let (s, n) = check_plan(entry, top, &mut mem, &mut |m, k| run(entry, top as u32, 0, m, &mut VuState::default(), k), plan_gpu);
                let e = agree.entry(entry).or_insert((0, 0));
                e.0 += s;
                e.1 += n;
                let want = translated("e72540c9e7c434ee", HASH, entry, top as u32, &mem);
                let mut kicks = Vec::new();
                let ok = run(entry, top as u32, 0, &mut mem, &mut VuState::default(), &mut |k| kicks.push(k.to_vec()));
                compare(&format!("snow case {case} entry {entry:#06x}"), (ok, kicks, mem.to_vec()), want);
            }
        }
        for (entry, (same, total)) in agree {
            eprintln!("snow entry {entry:#06x}: GPU plan, {same} of {total} sprites agree");
            assert!(total > 100 && same as f64 >= 0.99 * total as f64, "entry {entry:#06x}: {same} of {total}");
        }
    }
}
