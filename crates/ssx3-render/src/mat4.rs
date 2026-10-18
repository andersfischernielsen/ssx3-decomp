//! # Type 4 (`f55ee634`) natively: procedural particles, bit-exact
//!
//! A readable Rust implementation of the VU1 microprogram with hash
//! `0xf55ee63416fe5dcf` (`generated/vu/vu_f55ee63416fe5dcf.rs` is the literal
//! translation; `docs/investigation/phase3-render.md` §1.5). [`run`] takes
//! the program's VU1 data memory and produces the same XGKICK packets **and**
//! the same data memory as the translated program, bit for bit (checked on
//! real frames by `SSX3_VERIFY_PARTICLES=1`,
//! `crates/ssx3-ps2/src/particle_verify.rs`).
//!
//! Nothing is simulated across frames: the EE uploads a descriptor and the
//! program regenerates every particle of the emitter from it, with the VU's
//! LFSR ([`Lfsr`]) as the random source.
//!
//! | MSCAL | user | what |
//! |---|---|---|
//! | 0x0000 | slot 82 `DrawEmitter`, board sparks | one emitter: particle i is born `t0 - i·dt` ago; spawn, velocity and gravity are transformed by M on the VU |
//! | 0x0510 | slot 83 `DrawDynamicEmitter` | a moving emitter: a ring of points (position, direction + seed), particles interpolated along each segment |
//! | 0x0a00 | slot 84 `DrawMEmitter` | as 0x0510 with a colour per point (×1/255 × D15) and RGB clamped to 128 |
//! | 0x0f10 | slot 84 `DrawMEmitter` | as 0x0a00 with alpha = 128·(1 − (age/life)²)·D16.w and RGB = colour/2 |
//! | 0x1460 | `PS2BezierMan_DrawPatchFX` | glitter: random points on a Bezier patch (power basis), kept when a hashed direction faces the patch normal |
//!
//! ## Memory (qword addresses)
//!
//! Header (from `DeferMan_EmitMaterialSetup`): qw0-3 M (model → clip
//! columns), qw4 screen scale (z/16), qw5 screen offset, qw6 {P[0].x,
//! P[1].y, 0, 0} (the projection's x/y focal scale). **The mode-1 header
//! (M = worldClip) is the one these entries are written for**; the mode-0
//! header (worldView, guard scale) that the game sends right after a
//! material-type switch is a game bug (§1.5). The program does not care: it
//! uses whatever is there.
//!
//! Descriptor at TOP (D0..D21, written by the EE per draw):
//!
//! | qw | 0x0 emitter | 0x510/0xa00/0xf10 ring |
//! |---|---|---|
//! | D0 | {count (int), trail samples (int), t0, dt} | {-, trail samples (int), -, age step} |
//! | D1 | {size random, life random, size base, life base} | same |
//! | D2 | {size growth, -, -, -} | same |
//! | D3 | {trail fade step, trail dt, seed spawn 1, seed spawn 2} | {trail fade step, trail dt, -, -} |
//! | D4 | seeds velocity 1-3 (.xyz) | - |
//! | D5 | seeds colour 1, colour 2, size, life | - |
//! | D6 | gravity G | gravity G |
//! | D7..D10 | velocity base + 3 random axes | velocity base + 3 random axes (added to the lerped direction) |
//! | D11..D14 | spawn base, 2 random axes, step per particle | spawn base, 2 random axes, × age |
//! | D15..D18 | colour base, 2 random, × age | same (0xa00: base × point colour) |
//! | D19 / D20 | the sprite's two STQ corners (.z := 1) | same |
//! | D21 | - | {segments - 1 (int), particles per segment (int), particles per unit (float), first age} |
//!
//! Ring (0x510): positions P\[k\] at TOP+22+k, directions E\[k\] at TOP+87+k
//! (E.w = the segment's LFSR seed; E.w < 1 marks an empty segment). 0xa00 and
//! 0xf10: E\[k\] at TOP+66+k, point colours (ints) at TOP+110+k. Segment k
//! lerps from point k to k+1, so the EE uploads one more point than
//! segments; the loop runs `D21.x + 1` times (the "c+1 off-by-one").
//!
//! Output: two GIF packets at 0x13b and 0x29a (double buffered, EE-written
//! tags: SPRITE, REGS ST,RGBAQ,XYZ2,ST,XYZ2), 70 sprites per kick, 5 qw per
//! sprite. The program only patches NLOOP|EOP into the tag's .x.
//!
//! ## Exactness rules
//!
//! - Every float op is the PS2 one (`ssx3_vu::vu::fast` / `float`), in the
//!   original operand order, including the accumulator chains (`ACC = a·1`,
//!   `ACC = 0 + x`) — see the helpers at the end of this file.
//! - The LFSR: `rinit` keeps the seed's 23 mantissa bits with exponent 0
//!   (`(s & 0x7fffff) | 0x3f800000`), `rnext` steps
//!   `r = ((r << 1) ^ bit4 ^ bit22) & 0x7fffff | 0x3f800000` and returns r
//!   as a float in [1, 2). The emitter (0x0) reseeds per value from the value
//!   itself, so each of its 9 random lanes is an independent sequence that
//!   advances once per particle, dead or alive. The ring entries seed once
//!   per segment and draw 9 values per particle in a fixed order. 0x1460
//!   chains a float hash through rinit (see [`glitter`]).
//! - The MAC sign flag the program branches on is the sign bit of the FMAC
//!   result (`ABS vf0` before each FMAND forces the stall that makes the
//!   flag visible).
//! - Every MULq sees the Q of the DIV just before it (no stale Q in any
//!   path, in the translated timing model).
//!
//! Inputs whose loops would not end (0x1460 with a count of 0) are declined:
//! [`run`] returns `false` and the caller runs the translated program.

use ssx3_vu::vu::{fast, float};

use crate::bezier::Mem;

/// A VU vector register: four PS2 floats (or integers) as raw bits, x..w.
pub type V = [u32; 4];

/// Hash of the program this module implements.
pub const HASH: u64 = 0xf55e_e634_16fe_5dcf;

/// Register state the program carries from one call to the next.
///
/// Empty: every entry writes each register (and seeds R) before reading it;
/// everything that crosses calls is in data memory. It exists so the API
/// matches [`crate::mat3`].
#[derive(Clone, Debug, Default)]
pub struct VuState {}

/// Entries [`run`] implements natively (MSCAL byte addresses).
pub fn implemented(entry: u32) -> bool {
    matches!(entry, 0x0000 | 0x0510 | 0x0a00 | 0x0f10 | 0x1460)
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
        0x0000 => emitter(&mut m, top, kick),
        0x0510 => ring(&mut m, top, Ring::Dynamic, kick),
        0x0a00 => ring(&mut m, top, Ring::Coloured, kick),
        0x0f10 => ring(&mut m, top, Ring::Faded, kick),
        0x1460 => return glitter(&mut m, top, kick),
        _ => return false,
    }
    true
}

// ---------------------------------------------------------------------------
// Layout
// ---------------------------------------------------------------------------

/// Header: M columns (qw0-3), screen scale, screen offset, projection x/y.
const MATRIX: u16 = 0;
const SCREEN_SCALE: u16 = 4;
const SCREEN_OFFSET: u16 = 5;
const PROJ: u16 = 6;

/// The two output packets (GIF tag + 70 × 5 qw each).
const PACKET_A: u16 = 0x13b;
const PACKET_B: u16 = 0x29a;
const SPRITES_PER_KICK: u16 = 70;

/// Ring entries: the ring starts at TOP+22; the per-point records follow.
const RING: u16 = 22;
const DIR_DYNAMIC: u16 = 65;
const DIR_COLOURED: u16 = 44;
const COLOUR_COLOURED: u16 = 88;

/// Constants from LOI.
const DRAG_T: u32 = 0xbf3a_e148; // -0.73
const DRAG_T2: u32 = 0x3de7_6c8b; // 0.113
const DRAG_T_MAX: u32 = 0x402c_cccd; // 2.7
const F255: u32 = 0x437f_0000;
const F128: u32 = 0x4300_0000;
const F64: u32 = 0x4280_0000;
const INV255: u32 = 0x3b80_8081; // 0.003921569
const HALF: u32 = 0x3f00_0000;
const F12: u32 = 0x4140_0000;
const F1_5: u32 = 0x3fc0_0000;
const HASH_K: u32 = 0x3f85_a5a5; // 1.0441176

pub(crate) const ONE: u32 = 0x3f80_0000;
/// vf0.
pub(crate) const VF0: V = [0, 0, 0, ONE];

// ---------------------------------------------------------------------------
// The VU random generator
// ---------------------------------------------------------------------------

/// The VU's R register: a 23-bit LFSR whose value reads as a float in
/// [1, 2) (exponent 0x7f, the 23 bits as mantissa).
#[derive(Clone, Copy, Debug)]
pub struct Lfsr(pub u32);

impl Lfsr {
    /// RINIT: the low 23 bits of `seed` (any float or int bits).
    pub fn init(seed: u32) -> Lfsr {
        Lfsr((seed & 0x7f_ffff) | ONE)
    }
    /// RNEXT: one step, then the new value.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        let r = self.0;
        let feedback = ((r >> 4) ^ (r >> 22)) & 1;
        self.0 = (((r << 1) ^ feedback) & 0x7f_ffff) | ONE;
        self.0
    }
    /// `RINIT s; RNEXT`: the value after `seed` in its own sequence.
    pub fn after(seed: u32) -> u32 {
        Lfsr::init(seed).next()
    }
}

// ---------------------------------------------------------------------------
// Output packets
// ---------------------------------------------------------------------------

/// The double-buffered sprite packets: vi4 (current tag), vi5 (the other
/// one), vi6 (write pointer), vi7 (sprites in the current packet).
pub(crate) struct Packets {
    tag: u16,
    spare: u16,
    next: u16,
    pub count: u16,
}

impl Packets {
    pub fn new(a: u16, b: u16) -> Packets {
        Packets { tag: a, spare: b, next: a + 1, count: 0 }
    }
    /// One sprite: {ST, RGBA, XYZ2, ST, XYZ2}.
    pub fn push(&mut self, m: &mut Mem, sprite: [V; 5]) {
        for (i, q) in sprite.into_iter().enumerate() {
            m.st(self.next.wrapping_add(i as u16), q);
        }
        self.next = self.next.wrapping_add(5);
        self.count = self.count.wrapping_add(1);
    }
    /// Patch NLOOP|EOP into the tag, kick it, switch buffers.
    pub fn kick(&mut self, m: &mut Mem, kick: &mut dyn FnMut(&[u128])) {
        m.isw(self.tag, 0, self.count.wrapping_add(0x8000));
        kick(&m.gif_packet(self.tag));
        std::mem::swap(&mut self.tag, &mut self.spare);
        self.next = self.tag.wrapping_add(1);
        self.count = 0;
    }
}

/// The header, as the sprite code reads it.
struct Screen {
    m: [V; 4],
    scale: V,
    offset: V,
    proj: V,
}

impl Screen {
    fn load(m: &Mem) -> Screen {
        Screen {
            m: m.matrix(MATRIX),
            scale: m.ld(SCREEN_SCALE),
            offset: m.ld(SCREEN_OFFSET),
            proj: m.ld(PROJ),
        }
    }
    /// The sprite's half size in screen units before the divide:
    /// `(proj.xy·size)·scale.xy`, z = w = 0.
    fn half_size(&self, size: u32) -> V {
        let p = mul(self.proj, bc(size));
        let h = mul(p, self.scale);
        [h[0], h[1], zero_sub(), zero_sub()]
    }
}

/// `SUB.zw vf, vf0, vf0`: 0 − 0 and 1 − 1, both +0.
fn zero_sub() -> u32 {
    sub(VF0, VF0)[2]
}

/// An STQ corner from the descriptor: (s, t, 1, w) (`MR32.z vf, vf0` then
/// `LQ.xyw`).
fn st_corner(d: V) -> V {
    [d[0], d[1], ONE, d[3]]
}

/// The drag curve's position: `ACC = (G·t + spawn)·1`, `+ (G − V)·poly`
/// with `poly = −0.73·tc + 0.113·tc²`, `tc = min(t, 2.7)`.
fn drag_poly(t: u32) -> u32 {
    let tc = min1(t, DRAG_T_MAX);
    let tc2 = mul1(tc, tc);
    madd1(mul1(tc, DRAG_T), tc2, DRAG_T2)
}

// ---------------------------------------------------------------------------
// 0x0000: one emitter
// ---------------------------------------------------------------------------

/// The 9 random lanes of the emitter (0x0), each reseeded from its own
/// previous value once per particle.
struct EmitterSeeds {
    life: u32,     // D5.w
    vel: [u32; 3], // D4.xyz
    colour: [u32; 2], // D5.xy
    size: u32,     // D5.z
    spawn: [u32; 2], // D3.zw
}

impl EmitterSeeds {
    /// One particle: each lane steps once (`RINIT x; RNEXT x`).
    fn step(&mut self) {
        // The original order (all lanes are independent): life, vel 1,
        // colour 1, vel 2, vel 3, spawn 1, spawn 2, colour 2, size.
        let [v0, v1, v2] = &mut self.vel;
        let [c0, c1] = &mut self.colour;
        let [s0, s1] = &mut self.spawn;
        for s in [&mut self.life, v0, c0, v1, v2, s0, s1, c1, &mut self.size] {
            *s = Lfsr::after(*s);
        }
    }
}

/// One particle of an emitter (0x0) or a ring (0x510/0xa00/0xf10), as its
/// trail loop reads it (the GPU plan hands the same to the shader).
#[derive(Clone, Copy, Debug)]
struct Particle {
    /// The spawn point and G − V: clip space for 0x0, object space for the
    /// rings (their samples go through M).
    spawn: V,
    g_minus_v: V,
    /// RGB (clamped; .w unused) and the alpha before the trail fade.
    rgb: V,
    alpha: u32,
    /// The sprite's half size before the divide (z = w = 0).
    half: V,
    /// The first trail sample's time (the particle's age).
    t: u32,
}

/// The trail every particle of a call draws: samples at t, t − dt, ...
/// (stops at a negative time or after D0.y samples).
struct Trail {
    gravity: V,
    fade_step: u32,
    dt: u32,
    samples: i16,
    st_a: V,
    st_b: V,
    half_max: u32,
}

impl Trail {
    fn load(m: &Mem, top: u16, gravity: V, half_max: u32) -> Trail {
        let d = |i: u16| m.ld(top.wrapping_add(i));
        Trail {
            gravity,
            fade_step: d(3)[0],
            dt: d(3)[1],
            samples: d(0)[1] as u16 as i16,
            st_a: st_corner(d(19)),
            st_b: st_corner(d(20)),
            half_max,
        }
    }
    /// How many samples (drawn or culled) the trail of a particle born at
    /// `t` visits; `None` past `cap`.
    fn samples_from(&self, t: u32, cap: u32) -> Option<u32> {
        let (mut n, mut tk, mut left) = (0u32, t, self.samples);
        loop {
            n += 1;
            if n > cap {
                return None;
            }
            tk = sub1(tk, self.dt);
            if neg(tk) {
                return Some(n);
            }
            left = left.wrapping_sub(1);
            if left <= 0 {
                return Some(n);
            }
        }
    }
}

/// The emitter's particles that are alive, in order.
fn emitter_particles(m: &mut Mem, top: u16, scr: &Screen, each: &mut dyn FnMut(&mut Mem, &Particle)) {
    let d = |m: &Mem, i: u16| m.ld(top.wrapping_add(i));
    let to_clip = |m: &Mem, i: u16| xform(&scr.m, d(m, i));
    // Directions and points in clip space (M·D, w included).
    let spawn_base = to_clip(m, 11);
    let spawn_rand = [to_clip(m, 12), to_clip(m, 13)];
    let spawn_step = to_clip(m, 14);
    let gravity = to_clip(m, 6);
    let vel_base = to_clip(m, 7);
    let vel_rand = [to_clip(m, 8), to_clip(m, 9), to_clip(m, 10)];
    let (d0, d1, d3, d4, d5) = (d(m, 0), d(m, 1), d(m, 3), d(m, 4), d(m, 5));
    let size_growth = d(m, 2)[0];
    let colour_d = [d(m, 15), d(m, 16), d(m, 17), d(m, 18)];
    let mut seeds = EmitterSeeds {
        life: d5[3],
        vel: [d4[0], d4[1], d4[2]],
        colour: [d5[0], d5[1]],
        size: d5[2],
        spawn: [d3[2], d3[3]],
    };

    let mut left = d0[0] as u16 as i16;
    let mut t = d0[2];
    let dt = d0[3];
    let mut index = 0u32; // float, particle number
    loop {
        seeds.step();
        let life = madd1(add1(0, d1[3]), d1[1], seeds.life);
        let alive = neg(sub1(t, life));
        let spawn = {
            let acc = mul(spawn_base, bc(ONE));
            let acc = madd(acc, spawn_step, bc(index));
            let acc = madd(acc, spawn_rand[0], bc(seeds.spawn[0]));
            madd(acc, spawn_rand[1], bc(seeds.spawn[1]))
        };
        let vel = {
            let acc = mul(vel_base, bc(ONE));
            let acc = madd(acc, vel_rand[0], bc(seeds.vel[0]));
            let acc = madd(acc, vel_rand[1], bc(seeds.vel[1]));
            madd(acc, vel_rand[2], bc(seeds.vel[2]))
        };
        let colour = {
            let acc = mul(colour_d[0], bc(ONE));
            let acc = madd(acc, colour_d[1], bc(seeds.colour[0]));
            let acc = madd(acc, colour_d[3], bc(t));
            madd(acc, colour_d[2], bc(seeds.colour[1]))
        };
        let g_minus_v = sub(gravity, vel);
        if alive {
            let q = div(ONE, life);
            let growth = mul1(mul1(size_growth, t), q);
            let size = madd1(add1(growth, d1[2]), d1[0], seeds.size);
            let rgb = min(max(colour, bc(0)), bc(F255));
            let half = scr.half_size(size);
            each(m, &Particle { spawn, g_minus_v, rgb, alpha: colour[3], half, t });
        }
        t = sub1(t, dt);
        left = left.wrapping_sub(1);
        index = add1(index, ONE);
        if neg(t) || left <= 0 {
            break;
        }
    }
}

fn emitter(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) {
    let scr = Screen::load(m);
    let trail = Trail::load(m, top, xform(&scr.m, m.ld(top.wrapping_add(6))), F64);
    let mut out = Packets::new(PACKET_A, PACKET_B);
    emitter_particles(m, top, &scr, &mut |m, p| {
        let mut fade = ONE;
        let mut tk = p.t;
        let mut trails = trail.samples;
        loop {
            let base = add(mul(trail.gravity, bc(tk)), p.spawn);
            let pos = madd(mul(base, bc(ONE)), p.g_minus_v, bc(drag_poly(tk)));
            if fast::clip(pos, pos[3]) & 0x3f == 0 {
                let rgba = ftoi0([p.rgb[0], p.rgb[1], p.rgb[2], mul1(p.alpha, fade)]);
                let q = div(ONE, pos[3]);
                let h = mul(p.half, bc(q));
                let pq = mul(pos, bc(q));
                let mut h = abs(h);
                let hc = min(h, bc(trail.half_max));
                h[0] = hc[0];
                h[1] = hc[1];
                let c = madd(mul(scr.offset, bc(ONE)), pq, scr.scale);
                let a = add(c, h);
                let b = sub(c, h);
                out.push(m, [trail.st_a, rgba, ftoi4(b), trail.st_b, ftoi4(a)]);
                // Only drawn samples fade (the step is in the kick
                // branch's delay slot).
                fade = sub1(fade, trail.fade_step);
                if out.count == SPRITES_PER_KICK {
                    out.kick(m, kick);
                }
            }
            tk = sub1(tk, trail.dt);
            if neg(tk) {
                break;
            }
            trails = trails.wrapping_sub(1);
            if trails <= 0 {
                break;
            }
        }
    });
    if out.count != 0 {
        out.kick(m, kick);
    }
}

// ---------------------------------------------------------------------------
// 0x0510 / 0x0a00 / 0x0f10: ring emitters
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Ring {
    /// 0x0510: colour from the descriptor only, RGB clamped to [0, 255],
    /// half size clamped to 64.
    Dynamic,
    /// 0x0a00: colour base × lerped point colour / 255, RGB clamped to
    /// [0, 128], half size clamped to 128.
    Coloured,
    /// 0x0f10: RGB = lerped point colour / 2, alpha =
    /// 128·(1 − (age/life)²)·D16.w, all clamped to [0, 128]; half size 128.
    Faded,
}

impl Ring {
    fn half_max(self) -> u32 {
        if self == Ring::Dynamic {
            F64
        } else {
            F128
        }
    }
}

/// One segment of the ring: its lerp end points.
struct Segment {
    /// E[k], E[k+1] (.w of `dir0` is the seed).
    dir0: V,
    dir1: V,
    pos0: V,
    pos1: V,
    /// Point colours (floats), Coloured/Faded only.
    col0: V,
    col1: V,
}

/// The ring's particles that are alive, in order.
fn ring_particles(m: &mut Mem, top: u16, kind: Ring, scr: &Screen, each: &mut dyn FnMut(&mut Mem, &Particle)) {
    let d = |m: &Mem, i: u16| m.ld(top.wrapping_add(i));
    let (d0, d21) = (d(m, 0), d(m, 21));
    let k = RingConsts {
        vel_d: [d(m, 7), d(m, 8), d(m, 9), d(m, 10)],
        spawn_d: [d(m, 11), d(m, 12), d(m, 13), d(m, 14)],
        top,
    };
    let age_step = d0[3];
    let per_unit = d21[2];
    let s_step = add1(0, div(ONE, per_unit));
    let dir_at = match kind {
        Ring::Dynamic => DIR_DYNAMIC,
        _ => DIR_COLOURED,
    };

    let mut segments = d21[0] as u16 as i16;
    let per_segment = d21[1] as u16 as i16;
    let mut age = d21[3];
    let mut base = top.wrapping_add(RING);
    loop {
        let at = |m: &Mem, i: u16| m.ld(base.wrapping_add(i));
        let seg = Segment {
            dir0: at(m, dir_at),
            dir1: at(m, dir_at + 1),
            pos0: at(m, 0),
            pos1: at(m, 1),
            col0: itof0(at(m, COLOUR_COLOURED)),
            col1: itof0(at(m, COLOUR_COLOURED + 1)),
        };
        base = base.wrapping_add(1);
        if neg(sub1(seg.dir0[3], ONE)) {
            // Empty segment: skip its particles' ages.
            age = madd1(mul1(age, ONE), age_step, per_unit);
        } else {
            let mut rng = Lfsr::init(seg.dir0[3]);
            let mut s = 0u32;
            let mut left = per_segment;
            loop {
                if let Some(p) = ring_particle(m, scr, kind, &seg, &mut rng, s, age, &k) {
                    each(m, &p);
                }
                age = add1(age, age_step);
                left = left.wrapping_sub(1);
                s = add1(s, s_step);
                if left <= 0 {
                    break;
                }
            }
        }
        segments = segments.wrapping_sub(1);
        if segments < 0 {
            break;
        }
    }
}

fn ring(m: &mut Mem, top: u16, kind: Ring, kick: &mut dyn FnMut(&[u128])) {
    let scr = Screen::load(m);
    let trail = Trail::load(m, top, m.ld(top.wrapping_add(6)), kind.half_max());
    let mut out = Packets::new(PACKET_A, PACKET_B);
    ring_particles(m, top, kind, &scr, &mut |m, p| {
        let mut fade = ONE;
        let mut tk = p.t;
        let mut trails = trail.samples;
        loop {
            let g = mul(trail.gravity, bc(tk));
            let poly = drag_poly(tk);
            let g = add(g, p.spawn);
            let pp = madd(mul(g, bc(ONE)), p.g_minus_v, bc(poly));
            let c = xform(&scr.m, [pp[0], pp[1], pp[2], ONE]);
            let visible = fast::clip(c, c[3]) & 0x3f == 0;
            let a = mul1(p.alpha, fade);
            // Here every sample fades, drawn or not (delay slot of the cull
            // branch).
            fade = sub1(fade, trail.fade_step);
            if visible {
                let q = div(ONE, c[3]);
                let rgba = ftoi0([p.rgb[0], p.rgb[1], p.rgb[2], a]);
                let h = mul(p.half, bc(q));
                let cq = with_xyz(c, mul(c, bc(q)));
                let mut h = abs(h);
                let centre = madd(mul(scr.offset, bc(ONE)), cq, scr.scale);
                let hc = min(h, bc(trail.half_max));
                h[0] = hc[0];
                h[1] = hc[1];
                let lo = sub(centre, h);
                let hi = add(centre, h);
                out.push(m, [trail.st_a, rgba, ftoi4(hi), trail.st_b, ftoi4(lo)]);
                if out.count == SPRITES_PER_KICK {
                    out.kick(m, kick);
                }
            }
            tk = sub1(tk, trail.dt);
            if neg(tk) {
                break;
            }
            trails = trails.wrapping_sub(1);
            if trails <= 0 {
                break;
            }
        }
    });
    if out.count != 0 {
        out.kick(m, kick);
    }
}

struct RingConsts {
    vel_d: [V; 4],
    spawn_d: [V; 4],
    top: u16,
}

/// `a·(1 − s) + b·s` as `ACC = a·w; b·s + ACC`.
fn lerp(a: V, b: V, w: u32, s: u32) -> V {
    madd(mul(a, bc(w)), b, bc(s))
}

/// One ring particle: its nine random values, then `None` when it is dead.
#[allow(clippy::too_many_arguments)]
fn ring_particle(m: &Mem, scr: &Screen, kind: Ring, seg: &Segment, rng: &mut Lfsr, s: u32, age: u32, k: &RingConsts) -> Option<Particle> {
    let d = |i: u16| m.ld(k.top.wrapping_add(i));
    // Nine values per particle, in this order whatever the variant
    // (0x0f10 draws and discards the two colour ones).
    let r_spawn = [rng.next(), rng.next()];
    let r_vel = [rng.next(), rng.next(), rng.next()];
    let r_colour = [rng.next(), rng.next()];
    let r_size = rng.next();
    let r_life = rng.next();

    let w = sub1(ONE, s);
    let dir = lerp(seg.dir0, seg.dir1, w, s);
    let pos = lerp(seg.pos0, seg.pos1, w, s);
    let d1 = d(1);
    let life = madd1(add1(0, d1[3]), d1[1], r_life);
    let spawn = {
        let acc = add(pos, k.spawn_d[0]);
        let acc = madd(acc, k.spawn_d[3], bc(age));
        let acc = madd(acc, k.spawn_d[1], bc(r_spawn[0]));
        madd(acc, k.spawn_d[2], bc(r_spawn[1]))
    };
    let vel = {
        let acc = add(dir, k.vel_d[0]);
        let acc = madd(acc, k.vel_d[1], bc(r_vel[0]));
        let acc = madd(acc, k.vel_d[2], bc(r_vel[1]));
        madd(acc, k.vel_d[3], bc(r_vel[2]))
    };
    let colour_d = [d(15), d(16), d(17), d(18)];
    // Colour before the life test (clamped later, per variant).
    let colour = match kind {
        Ring::Dynamic | Ring::Coloured => {
            let acc = if kind == Ring::Dynamic {
                mul(colour_d[0], bc(ONE))
            } else {
                let point = mul(lerp(seg.col0, seg.col1, w, s), bc(INV255));
                mul(colour_d[0], point)
            };
            let acc = madd(acc, colour_d[1], bc(r_colour[0]));
            let acc = madd(acc, colour_d[3], bc(age));
            let c = madd(acc, colour_d[2], bc(r_colour[1]));
            if kind == Ring::Coloured {
                let c = with_xyz(c, min(c, bc(F128)));
                max(c, bc(0))
            } else {
                c
            }
        }
        // Halved in the delay slot of the life test.
        Ring::Faded => mul(lerp(seg.col0, seg.col1, w, s), bc(HALF)),
    };
    if !neg(sub1(age, life)) {
        return None;
    }
    let g_minus_v = sub(d(6), vel);
    let q = div(ONE, life);
    let (rgb, alpha) = match kind {
        Ring::Dynamic | Ring::Coloured => (min(max(colour, bc(0)), bc(F255)), colour[3]),
        Ring::Faded => {
            let u = mul1(age, q);
            let u = mul1(u, u);
            let u = sub1(ONE, u);
            let a = mul1(mul1(u, F128), d(16)[3]);
            let c = min([colour[0], colour[1], colour[2], a], bc(F128));
            let c = max(c, bc(0));
            (min(max(c, bc(0)), bc(F255)), c[3])
        }
    };
    let growth = mul1(mul1(d(2)[0], age), q);
    let size = madd1(add1(growth, d1[2]), d1[0], r_size);
    Some(Particle { spawn, g_minus_v, rgb, alpha, half: scr.half_size(size), t: age })
}

// ---------------------------------------------------------------------------
// 0x1460: glitter on a Bezier patch
// ---------------------------------------------------------------------------

/// `PS2BezierMan_DrawPatchFX`: D21.w points on the patch (D0..D15: the 16
/// power-basis coefficients, as 4 groups of 4 rows), each at random
/// (u, v) from a chained float hash, kept when a random direction faces the
/// patch normal (D16..D18 × M's z column). Sprite: colour D20, alpha =
/// 128·min(dot, 1), size D21.x clamped to [D21.y, D21.z] in screen units,
/// lifted 12 units in z. STQ corners (0,0,1) and (1,1,1).
///
/// The hash: a 3-lane state h plus the previous point's cubic terms feed
/// three RINITs (h.z, h.x, h.y advance to new random values each point), and
/// two more RINITs seeded from sums of the previous terms give r1, r2 with
/// u = (r1−1)³, (r1−1)², (r1−1) and likewise for r2. It is order dependent
/// (point n's randoms depend on every earlier point), so a GPU version must
/// run it serially per patch (or precompute it on the CPU).
/// A glitter point whose direction faces the patch normal: its cubic terms
/// (u = (w1³, w1², w1), v likewise), the facing dot product, and whether it
/// is the call's last point.
struct GlitterPoint {
    u: [u32; 3],
    v: [u32; 3],
    dot: u32,
    last: bool,
}

/// The glitter header values the sprite code reads.
struct GlitterSetup {
    coef: [V; 16],
    colour: V,
    d21: V,
}

/// The facing points of a glitter call, in order (the hash runs over every
/// point). `count` must not be 0.
fn glitter_points(m: &mut Mem, top: u16, scr: &Screen, each: &mut dyn FnMut(&mut Mem, &GlitterPoint)) {
    let d = |m: &Mem, i: u16| m.ld(top.wrapping_add(i));
    let count = d(m, 21)[3] as u16;
    let coef: [V; 16] = std::array::from_fn(|i| d(m, i as u16));
    let (d0, d1, d2, d3) = (coef[0], coef[1], coef[2], coef[3]);

    // The patch normal direction: (D16 D17 D18 D18) · (M · (0,0,1,0)).
    let mz = xform(&scr.m, [0, 0, ONE, 0]);
    let normal = {
        let acc = mul(d(m, 16), bc(mz[0]));
        let acc = madd(acc, d(m, 17), bc(mz[1]));
        let acc = madd(acc, d(m, 18), bc(mz[2]));
        madd(acc, d(m, 18), bc(mz[3]))
    };

    // Hash state.
    let a0 = add(d0, d1);
    let b0 = add(d1, d2);
    let p16 = add(a0, bc(HASH_K)); // vf16.xyz
    let p17 = add(b0, bc(HASH_K)); // vf17.xyz
    let half_unit = mul1(ONE, F1_5); // vf16.w
    let s19z = add1(p16[2], p16[1]);
    let s17z = add1(p17[2], p17[1]);
    let mut e = add1(s19z, p16[0]); // vf21.z
    let mut c = add1(s17z, p17[0]); // vf22.z
    let mut h = [add1(d1[0], d2[3]), add1(d2[1], d1[1]), add1(d3[2], d0[3])];
    for x in h.iter_mut() {
        *x = add1(*x, HASH_K);
    }
    let mut a = 0u32; // vf19.x
    let mut b = 0u32; // vf19.y
    let mut dd = 0u32; // vf20.x

    let mut done = 0u16;
    loop {
        h[2] = add1(h[2], a);
        h[0] = add1(h[0], b);
        h[1] = add1(h[1], c);
        let s1 = add1(h[2], dd);
        h[0] = add1(h[0], h[1]);
        h[1] = add1(h[1], h[2]);
        h[2] = Lfsr::after(s1);
        h[0] = Lfsr::after(h[0]);
        h[1] = Lfsr::after(h[1]);
        let s2 = add1(e, c);
        let s3 = add1(c, s2);
        let r1 = Lfsr::after(s2);
        let r2 = Lfsr::after(s3);
        for x in h.iter_mut() {
            *x = sub1(*x, half_unit);
        }
        let w1 = sub1(r1, ONE);
        let n = mul([h[0], h[1], h[2], 0], normal);
        let w2 = sub1(r2, ONE);
        let v1 = mul1(w1, w1);
        let v2 = mul1(w2, w2);
        let u2 = mul1(v2, w2);
        let dot = add1(add1(n[0], n[1]), n[2]);
        let facing = !neg(sub1(dot, 0));
        let u1 = mul1(v1, w1);
        // The chained terms.
        a = u1;
        b = v1;
        c = w2;
        dd = u2;
        e = w1;
        done = done.wrapping_add(1);
        let last = done == count;
        if facing {
            each(m, &GlitterPoint { u: [u1, v1, w1], v: [u2, v2, w2], dot, last });
        }
        if last {
            break;
        }
    }
}

impl GlitterSetup {
    fn load(m: &Mem, top: u16) -> GlitterSetup {
        let d = |i: u16| m.ld(top.wrapping_add(i));
        GlitterSetup { coef: std::array::from_fn(|i| d(i as u16)), colour: d(20), d21: d(21) }
    }
    /// The point on the patch, lifted 12 units in z.
    fn eval(&self, g: &GlitterPoint) -> V {
        let [u1, v1, w1] = g.u;
        let eval = |k: usize| -> V {
            let acc = mul(self.coef[k], bc(u1));
            let acc = madd(acc, self.coef[k + 1], bc(v1));
            let acc = madd(acc, self.coef[k + 2], bc(w1));
            madd(acc, self.coef[k + 3], bc(ONE))
        };
        let rows = [eval(0), eval(4), eval(8), eval(12)];
        let [u2, v2, w2] = g.v;
        let p = {
            let acc = mul(rows[0], bc(u2));
            let acc = madd(acc, rows[1], bc(v2));
            let acc = madd(acc, rows[2], bc(w2));
            madd(acc, rows[3], bc(ONE))
        };
        let lift = mul1(ONE, F12); // vf17.w
        [p[0], p[1], add1(p[2], lift), 0]
    }
    /// The sprite's alpha (an integer, FTOI0).
    fn alpha(g: &GlitterPoint) -> u32 {
        ftoi0(bc(mul1(min1(g.dot, ONE), F128)))[0]
    }
    /// The half size before the divide.
    fn half(&self, scr: &Screen) -> V {
        let hp = mul(scr.proj, bc(self.d21[0]));
        let hs = mul(hp, scr.scale);
        [hs[0], hs[1], zero_sub(), zero_sub()]
    }
}

fn glitter(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    if m.ld(top.wrapping_add(21))[3] as u16 == 0 {
        return false;
    }
    let scr = Screen::load(m);
    let g = GlitterSetup::load(m, top);
    let st_a = [0, 0, ONE, 0];
    let st_b = [ONE, ONE, ONE, 0];
    let mut out = Packets::new(PACKET_A, PACKET_B);
    glitter_points(m, top, &scr, &mut |m, pt| {
        let p = g.eval(pt);
        let cl = xform(&scr.m, [p[0], p[1], p[2], ONE]);
        if fast::clip(cl, cl[3]) & 0x3f == 0 {
            let q = div(ONE, cl[3]);
            let mut half = mul(g.half(&scr), bc(q));
            let ha = abs(half);
            half[0] = ha[0];
            half[1] = ha[1];
            let rgba = [ftoi0(g.colour)[0], ftoi0(g.colour)[1], ftoi0(g.colour)[2], GlitterSetup::alpha(pt)];
            let cq = with_xyz(cl, mul(cl, bc(q)));
            let hm = max(half, bc(g.d21[1]));
            half[0] = hm[0];
            half[1] = hm[1];
            let centre = madd(mul(scr.offset, bc(ONE)), cq, scr.scale);
            let hm = min(half, bc(g.d21[2]));
            half[0] = hm[0];
            half[1] = hm[1];
            let hi = add(centre, half);
            let lo = sub(centre, half);
            out.push(m, [st_a, rgba, ftoi4(hi), st_b, ftoi4(lo)]);
            if !pt.last && out.count == SPRITES_PER_KICK {
                out.kick(m, kick);
            }
        }
    });
    // Kicked even when empty.
    out.kick(m, kick);
    true
}

// ---------------------------------------------------------------------------
// GPU plans (`crate::gpu`, plan step N1), shared with mat5
// ---------------------------------------------------------------------------

/// Uniform and data layouts of the particle shaders
/// (`ssx3-gfx/src/gpuvu/particles.wgsl`), shared by [`plan_gpu`],
/// `mat5::plan_gpu` and the CPU mirror [`gpu_sprites`].
///
/// Every program draws sprites: call vertex `4·slot + corner` is corner
/// `corner` of sprite slot `slot` (the indices list the slots that may draw,
/// two triangles each; the shader culls the ones the program's CLIP test
/// would).
pub mod gpu_particles {
    // `vs_mat4_particles` (0x0000 emitters, 0x0510/0x0a00/0x0f10 rings).
    // Data per particle: spawn, G − V, (RGB, alpha), (half.xy, t, -);
    // slot = particle · SLOTS.x + trail sample.
    /// M columns (rings: samples are in object space).
    pub const MATRIX: usize = 0;
    pub const SCALE: usize = 4;
    pub const OFFSET: usize = 5;
    /// (fade step, trail dt, half size max, ring (u32)).
    pub const TRAIL: usize = 6;
    /// G (clip space for the emitter, object space for rings).
    pub const GRAVITY: usize = 7;
    /// The two STQ corners.
    pub const ST_A: usize = 8;
    pub const ST_B: usize = 9;
    /// x: trail slots per particle.
    pub const SLOTS: usize = 10;

    // `vs_mat4_glitter` (0x1460). Data per facing point: (u³, u², u,
    // alpha (int)), (v³, v², v, -).
    /// The 16 patch coefficients.
    pub const G_COEF: usize = 0;
    pub const G_MATRIX: usize = 16;
    pub const G_SCALE: usize = 20;
    pub const G_OFFSET: usize = 21;
    /// The half size before the divide.
    pub const G_HALF: usize = 22;
    /// RGB (ints).
    pub const G_RGB: usize = 23;
    /// (half size min, half size max).
    pub const G_LIMITS: usize = 24;

    // `vs_mat5_snow` (mat5). Data per flake: (p.xyz, p₁.z − max.z).
    pub const S_MATRIX: usize = 0;
    pub const S_SCALE: usize = 4;
    pub const S_OFFSET: usize = 5;
    /// The half size before the divide.
    pub const S_HALF: usize = 6;
    /// RGBA (ints).
    pub const S_RGBA: usize = 7;
    /// (alpha (float), depth scale, depth fade on (u32), 0.8).
    pub const S_FADE: usize = 8;
    pub const S_ST_A: usize = 9;
    pub const S_ST_B: usize = 10;

    /// The longest trail the GPU plan takes (the shader's fade loop walks
    /// the earlier samples).
    pub const MAX_SLOTS: u32 = 64;
}

/// The sprite tag the program kicks (EE-written, the same in both
/// buffers) when the GPU draw takes it: PRE, SPRITE, STQ (not FST), packed
/// {ST, RGBAQ, XYZ2, ST, XYZ2}. Also the screen's w lanes must not put ADC
/// in the XYZ2 qwords (scale.w = 0, FTOI4(offset.w) without bit 15).
pub(crate) fn gpu_sprite_tag(m: &Mem, a: u16, b: u16, scale: V, offset: V) -> Option<u128> {
    let (ta, tb) = (m.ld(a), m.ld(b));
    let ok = ta[1..] == tb[1..]
        && ta[0] >> 16 == tb[0] >> 16
        && ta[1] >> 26 == 5 << 2
        && ta[1] & (1 << 14) != 0
        && (ta[1] >> 15) & 7 == 6
        && (ta[1] >> 15) & (1 << 8) == 0
        && ta[2] == 0x52512
        && ta[3] == 0
        && scale[3] & 0x7fff_ffff == 0
        && ftoi4(offset)[3] & 0x8000 == 0;
    let w0 = (ta[0] & !0xffff) | 0x8000;
    ok.then(|| w0 as u128 | (ta[1] as u128) << 32 | (ta[2] as u128) << 64 | (ta[3] as u128) << 96)
}

/// A sprite draw over `slots` (two triangles per slot).
pub(crate) fn sprite_call(program: crate::gpu::Program, tag: u128, uniforms: [V; crate::gpu::UNIFORM_QWORDS], data: Vec<V>, slots: impl Iterator<Item = u32>, alpha_max: u32) -> crate::gpu::GpuCall {
    let mut indices = Vec::new();
    for s in slots {
        let b = 4 * s;
        indices.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
    }
    crate::gpu::GpuCall { program, pre: Vec::new(), tag, uniforms, data, indices, alpha_max: alpha_max as f32, link: crate::gpu::Link::None, mesh_key: None }
}

/// The alpha byte the GS takes from an alpha (FTOI0, low 8 bits).
pub(crate) fn alpha_byte(a: u32) -> u32 {
    ftoi0(bc(a))[0] & 0xff
}

/// Plan a call for the GPU (`crate::gpu::plan`). Nothing the program
/// writes is read by a later call (its descriptor comes from the EE each
/// time, its LFSR is reseeded per call, the output packets are its own), so
/// there are no memory effects. Per particle the CPU does the random draws
/// and everything up to the trail (life, spawn, G − V, colour, size); per
/// trail sample the shader does the position, the CLIP cull, the fade and
/// the sprite. Glitter: the CPU runs the serial hash and the facing test,
/// the shader evaluates the patch.
pub fn plan_gpu(entry: u32, top: u32, _itop: u32, mem: &mut [u8; 16384]) -> Option<Vec<crate::gpu::GpuCall>> {
    use gpu_particles::*;
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    let m = &mut m;
    let scr = Screen::load(m);
    let tag = gpu_sprite_tag(m, PACKET_A, PACKET_B, scr.scale, scr.offset)?;
    let d = |m: &Mem, i: u16| m.ld(top.wrapping_add(i));
    let mut u = [[0u32; 4]; crate::gpu::UNIFORM_QWORDS];
    if entry == 0x1460 {
        if d(m, 21)[3] as u16 == 0 {
            return None;
        }
        let g = GlitterSetup::load(m, top);
        let mut data = Vec::new();
        let mut amax = 0;
        glitter_points(m, top, &scr, &mut |_, pt| {
            let a = GlitterSetup::alpha(pt);
            amax = amax.max(a & 0xff);
            data.push([pt.u[0], pt.u[1], pt.u[2], a]);
            data.push([pt.v[0], pt.v[1], pt.v[2], 0]);
        });
        u[G_COEF..G_COEF + 16].copy_from_slice(&g.coef);
        u[G_MATRIX..G_MATRIX + 4].copy_from_slice(&scr.m);
        u[G_SCALE] = scr.scale;
        u[G_OFFSET] = scr.offset;
        u[G_HALF] = g.half(&scr);
        u[G_RGB] = ftoi0(g.colour);
        u[G_LIMITS] = [g.d21[1], g.d21[2], 0, 0];
        // (Kicked even when empty: the tag still sets PRIM.)
        let n = data.len() as u32 / 2;
        return Some(vec![sprite_call(crate::gpu::Program::Mat4Glitter, tag, u, data, 0..n, amax)]);
    }
    let kind = match entry {
        0x0000 => None,
        0x0510 => Some(Ring::Dynamic),
        0x0a00 => Some(Ring::Coloured),
        0x0f10 => Some(Ring::Faded),
        _ => return None,
    };
    let trail = match kind {
        None => Trail::load(m, top, xform(&scr.m, d(m, 6)), F64),
        Some(k) => Trail::load(m, top, d(m, 6), k.half_max()),
    };
    let mut data = Vec::new();
    let mut samples = Vec::new();
    let mut amax = 0;
    let mut ok = true;
    let mut each = |_: &mut Mem, p: &Particle| {
        let Some(n) = trail.samples_from(p.t, MAX_SLOTS) else {
            ok = false;
            return;
        };
        data.extend_from_slice(&[p.spawn, p.g_minus_v, [p.rgb[0], p.rgb[1], p.rgb[2], p.alpha], [p.half[0], p.half[1], p.t, 0]]);
        samples.push(n);
        // (The alphas the samples can have: drawn emitter samples take
        // the first fades of the same sequence.)
        let mut fade = ONE;
        for _ in 0..n {
            amax = amax.max(alpha_byte(mul1(p.alpha, fade)));
            fade = sub1(fade, trail.fade_step);
        }
    };
    match kind {
        None => emitter_particles(m, top, &scr, &mut each),
        Some(k) => ring_particles(m, top, k, &scr, &mut each),
    }
    if !ok {
        return None;
    }
    if samples.is_empty() {
        // Nothing alive: the program kicks nothing.
        return Some(Vec::new());
    }
    let slots = samples.iter().copied().max().unwrap_or(1);
    u[MATRIX..MATRIX + 4].copy_from_slice(&scr.m);
    u[SCALE] = scr.scale;
    u[OFFSET] = scr.offset;
    u[TRAIL] = [trail.fade_step, trail.dt, trail.half_max, kind.is_some() as u32];
    u[GRAVITY] = trail.gravity;
    u[ST_A] = trail.st_a;
    u[ST_B] = trail.st_b;
    u[SLOTS] = [slots, 0, 0, 0];
    let order = samples.iter().enumerate().flat_map(|(i, &n)| (0..n).map(move |k| i as u32 * slots + k)).collect::<Vec<_>>();
    Some(vec![sprite_call(crate::gpu::Program::Mat4Particles, tag, u, data, order.into_iter(), amax)])
}

/// One sprite as the program kicks it: the two XYZ2 positions (FTOI4
/// x, y, z), RGBA (ints) and the two STQ corners (s, t).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sprite {
    pub a: [i32; 3],
    pub b: [i32; 3],
    pub rgba: [i32; 4],
    pub st: [f32; 4],
}

/// The sprites of a planned call as the shaders compute them (IEEE, the
/// WGSL's formulas and order), `None` for slots the shader culls.
pub fn gpu_sprites(call: &crate::gpu::GpuCall) -> Vec<Option<Sprite>> {
    use crate::gpu::Program;
    use gpu_particles::*;
    let f = |v: V| v.map(f32::from_bits);
    let uq = |q: usize| f(call.uniforms[q]);
    let inside = |v: [f32; 4]| (0..3).all(|l| v[l].abs() <= v[3].abs());
    let xform = |c: usize, p: [f32; 3]| -> [f32; 4] {
        std::array::from_fn(|l| uq(c)[l] * p[0] + uq(c + 1)[l] * p[1] + uq(c + 2)[l] * p[2] + uq(c + 3)[l])
    };
    let ftoi4 = |v: [f32; 3]| v.map(|x| (x * 16.0) as i32);
    // The sprite's centre on screen.
    let centre = |cl: [f32; 4], scale: usize, offset: usize| -> [f32; 3] {
        let q = 1.0 / cl[3];
        std::array::from_fn(|l| uq(offset)[l] + cl[l] * q * uq(scale)[l])
    };
    let slots: Vec<u32> = call.indices.chunks(6).map(|t| t[0] / 4).collect();
    slots
        .iter()
        .map(|&s| match call.program {
            Program::Mat4Particles => {
                let per = call.uniforms[SLOTS][0];
                let (p, k) = (s / per, s % per);
                let rec = |i: u32| f(call.data[(4 * p + i) as usize]);
                let (spawn, gmv, col, misc) = (rec(0), rec(1), rec(2), rec(3));
                let [fade_step, dt, half_max, _] = uq(TRAIL);
                let ring = call.uniforms[TRAIL][3] != 0;
                let g = uq(GRAVITY);
                let clip = |tk: f32| -> [f32; 4] {
                    let tc = tk.min(2.7);
                    let poly = tc * -0.73 + tc * tc * 0.113;
                    let p: [f32; 4] = std::array::from_fn(|l| g[l] * tk + spawn[l] + gmv[l] * poly);
                    if ring {
                        xform(MATRIX, [p[0], p[1], p[2]])
                    } else {
                        p
                    }
                };
                let mut tk = misc[2];
                let mut fade = 1.0f32;
                for _ in 0..k {
                    if ring || inside(clip(tk)) {
                        fade -= fade_step;
                    }
                    tk -= dt;
                }
                let cl = clip(tk);
                if !inside(cl) {
                    return None;
                }
                let q = 1.0 / cl[3];
                let h = [(misc[0] * q).abs().min(half_max), (misc[1] * q).abs().min(half_max), 0.0];
                let c = centre(cl, SCALE, OFFSET);
                let lo: [f32; 3] = std::array::from_fn(|l| c[l] - h[l]);
                let hi: [f32; 3] = std::array::from_fn(|l| c[l] + h[l]);
                let (a, b) = if ring { (hi, lo) } else { (lo, hi) };
                let rgba = [col[0] as i32, col[1] as i32, col[2] as i32, (col[3] * fade) as i32];
                let (sa, sb) = (uq(ST_A), uq(ST_B));
                Some(Sprite { a: ftoi4(a), b: ftoi4(b), rgba, st: [sa[0], sa[1], sb[0], sb[1]] })
            }
            Program::Mat4Glitter => {
                let pa = f(call.data[2 * s as usize]);
                let pb = f(call.data[2 * s as usize + 1]);
                let row = |k: usize| -> [f32; 4] { std::array::from_fn(|l| uq(G_COEF + k)[l] * pa[0] + uq(G_COEF + k + 1)[l] * pa[1] + uq(G_COEF + k + 2)[l] * pa[2] + uq(G_COEF + k + 3)[l]) };
                let (r0, r1, r2, r3) = (row(0), row(4), row(8), row(12));
                let p: [f32; 4] = std::array::from_fn(|l| r0[l] * pb[0] + r1[l] * pb[1] + r2[l] * pb[2] + r3[l]);
                let cl = xform(G_MATRIX, [p[0], p[1], p[2] + 12.0]);
                if !inside(cl) {
                    return None;
                }
                let q = 1.0 / cl[3];
                let [lo_lim, hi_lim, _, _] = uq(G_LIMITS);
                let hh = |l: usize| (uq(G_HALF)[l] * q).abs().max(lo_lim).min(hi_lim);
                let h = [hh(0), hh(1), 0.0];
                let c = centre(cl, G_SCALE, G_OFFSET);
                let lo: [f32; 3] = std::array::from_fn(|l| c[l] - h[l]);
                let hi: [f32; 3] = std::array::from_fn(|l| c[l] + h[l]);
                let rgb = call.uniforms[G_RGB];
                let rgba = [rgb[0] as i32, rgb[1] as i32, rgb[2] as i32, call.data[2 * s as usize][3] as i32];
                Some(Sprite { a: ftoi4(hi), b: ftoi4(lo), rgba, st: [0.0, 0.0, 1.0, 1.0] })
            }
            Program::Mat5Snow => {
                let p = f(call.data[s as usize]);
                let cl = xform(S_MATRIX, [p[0], p[1], p[2]]);
                let [alpha, ds, _, far] = uq(S_FADE);
                let fade = call.uniforms[S_FADE][2] != 0;
                if !fade && !inside(cl) {
                    return None;
                }
                let r = call.uniforms[S_RGBA];
                let mut rgba = [r[0] as i32, r[1] as i32, r[2] as i32, r[3] as i32];
                if fade {
                    let d = cl[2] * ds;
                    let f = if p[3] - d >= 0.0 {
                        (d * 3.33).clamp(0.0, 1.0)
                    } else if d > far {
                        (1.0 - (d - far) * 3.33).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    rgba[3] = (alpha * f) as i32;
                }
                let q = 1.0 / cl[3];
                let h: [f32; 3] = std::array::from_fn(|l| uq(S_HALF)[l] * q);
                let c = centre(cl, S_SCALE, S_OFFSET);
                let lo: [f32; 3] = std::array::from_fn(|l| c[l] - h[l]);
                let hi: [f32; 3] = std::array::from_fn(|l| c[l] + h[l]);
                let (sa, sb) = (uq(S_ST_A), uq(S_ST_B));
                Some(Sprite { a: ftoi4(hi), b: ftoi4(lo), rgba, st: [sa[0], sa[1], sb[0], sb[1]] })
            }
            _ => None,
        })
        .collect()
}

/// The sprites in a program's GIF output (SPRITE packets of 5 qwords per
/// sprite after each tag).
pub fn kicked_sprites(gif: &[u128]) -> Vec<Sprite> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < gif.len() {
        let n = (gif[i] as u32 & 0x7fff) as usize;
        for k in 0..n {
            let q = |j: usize| -> V {
                let v = gif.get(i + 1 + 5 * k + j).copied().unwrap_or(0);
                std::array::from_fn(|l| (v >> (32 * l)) as u32)
            };
            let xyz = |v: V| [v[0] as i32, v[1] as i32, v[2] as i32];
            let (sa, c, a, sb, b) = (q(0), q(1), q(2), q(3), q(4));
            out.push(Sprite {
                a: xyz(a),
                b: xyz(b),
                rgba: [c[0] as i32, c[1] as i32, c[2] as i32, c[3] as i32],
                st: [f32::from_bits(sa[0]), f32::from_bits(sa[1]), f32::from_bits(sb[0]), f32::from_bits(sb[1])],
            });
        }
        i += 1 + 5 * n;
    }
    out
}

/// How a planned call's shader sprites compare with the program's: the
/// number of sprites both draw and agree on (positions within 2/16 pixel
/// and 1e-3 relative for z, colour bytes within 1), and a description of
/// the first differences. (Z: the samples come from world-space sums that
/// cancel in the transform, so the PS2's truncating rounding and IEEE's
/// differ by a few 1e-4 of the depth on real frames.)
pub fn compare_sprites(calls: &[crate::gpu::GpuCall], gif: &[u128]) -> (usize, usize, usize, Vec<String>) {
    let want = kicked_sprites(gif);
    let got: Vec<Sprite> = calls.iter().flat_map(gpu_sprites).flatten().collect();
    let close = |a: &Sprite, b: &Sprite| {
        let pos = |x: [i32; 3], y: [i32; 3]| (0..2).all(|l| (x[l] - y[l]).abs() <= 2) && (x[2] as f64 - y[2] as f64).abs() <= 2.0 + 1e-3 * (y[2] as f64).abs();
        let col = (0..4).all(|l| {
            let d = ((a.rgba[l] & 0xff) - (b.rgba[l] & 0xff)).abs();
            d <= 1 || d == 255
        });
        pos(a.a, b.a) && pos(a.b, b.b) && col && a.st == b.st
    };
    // Align in order, letting either side skip a sprite (CLIP decisions at
    // the boundary may differ).
    let (mut i, mut j, mut same) = (0, 0, 0);
    let mut bad = Vec::new();
    while i < got.len() && j < want.len() {
        if close(&got[i], &want[j]) {
            same += 1;
            i += 1;
            j += 1;
        } else if j + 1 < want.len() && close(&got[i], &want[j + 1]) {
            j += 1;
        } else if i + 1 < got.len() && close(&got[i + 1], &want[j]) {
            i += 1;
        } else {
            if bad.len() < 3 {
                bad.push(format!("sprite {j}: {:?} want {:?}", got[i], want[j]));
            }
            i += 1;
            j += 1;
        }
    }
    (same, got.len(), want.len(), bad)
}

/// `gpu::check` for types 4 and 5: a description when the shader sprites
/// differ from the program's beyond rounding and CLIP-boundary decisions.
pub fn gpu_check(calls: &[crate::gpu::GpuCall], gif: &[u128]) -> Option<String> {
    let (same, got, want, bad) = compare_sprites(calls, gif);
    let off = got.max(want) - same;
    (off > want / 100 || !bad.is_empty()).then(|| format!("particles: {same} sprites agree, shader {got}, program {want}: {}", bad.join("; ")))
}

// ---------------------------------------------------------------------------
// PS2 float ops (the translated code's helpers), shared with mat5
// ---------------------------------------------------------------------------

pub(crate) fn bc(x: u32) -> V {
    [x; 4]
}
pub(crate) fn add(a: V, b: V) -> V {
    fast::add4::<0b1111, true, true>(&a, &b).0
}
pub(crate) fn sub(a: V, b: V) -> V {
    fast::sub4::<0b1111, true, true>(&a, &b).0
}
pub(crate) fn mul(a: V, b: V) -> V {
    fast::mul4::<0b1111, true, true>(&a, &b).0
}
/// `acc + a·b`.
pub(crate) fn madd(acc: V, a: V, b: V) -> V {
    fast::madd4::<0b1111, true, true, true>(&acc, &a, &b).0
}
pub(crate) fn max(a: V, b: V) -> V {
    fast::max4(&a, &b)
}
pub(crate) fn min(a: V, b: V) -> V {
    fast::min4(&a, &b)
}
pub(crate) fn abs(a: V) -> V {
    fast::abs4(&a)
}
pub(crate) fn ftoi0(a: V) -> V {
    fast::ftoi4::<0>(&a)
}
pub(crate) fn ftoi4(a: V) -> V {
    fast::ftoi4::<4>(&a)
}
pub(crate) fn itof0(a: V) -> V {
    fast::itof4::<0>(&a)
}
/// DIV: `a / b` as the DIV unit computes it.
pub(crate) fn div(a: u32, b: u32) -> u32 {
    float::div(a, b).value
}
/// Single-lane forms (the same vector helpers, one lane used).
pub(crate) fn add1(a: u32, b: u32) -> u32 {
    add(bc(a), bc(b))[0]
}
pub(crate) fn sub1(a: u32, b: u32) -> u32 {
    sub(bc(a), bc(b))[0]
}
pub(crate) fn mul1(a: u32, b: u32) -> u32 {
    mul(bc(a), bc(b))[0]
}
pub(crate) fn madd1(acc: u32, a: u32, b: u32) -> u32 {
    madd(bc(acc), bc(a), bc(b))[0]
}
pub(crate) fn min1(a: u32, b: u32) -> u32 {
    min(bc(a), bc(b))[0]
}
/// The MAC sign flag of an FMAC result lane.
pub(crate) fn neg(x: u32) -> bool {
    x & 0x8000_0000 != 0
}
/// `v` with x, y, z from `xyz` (a `.xyz` destination mask).
pub(crate) fn with_xyz(v: V, xyz: V) -> V {
    [xyz[0], xyz[1], xyz[2], v[3]]
}
/// `cols · p` as MULAx / MADDAy / MADDAz / MADDw.
pub(crate) fn xform(cols: &[V; 4], p: V) -> V {
    let acc = mul(cols[0], bc(p[0]));
    let acc = madd(acc, cols[1], bc(p[1]));
    let acc = madd(acc, cols[2], bc(p[2]));
    madd(acc, cols[3], bc(p[3]))
}

/// Differential tests against the translated microcode on synthetic inputs
/// (shared with [`crate::mat5`]): covers the entries the verify run does not
/// reach (0x0f10, the snow program).
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ssx3_vu::vu::Vu;

    /// The program's code bytes, from the `// 0xADDR: upper lower` comments
    /// of the translation.
    fn code(hash: &str) -> Vec<u8> {
        let path = format!("{}/../../generated/vu/vu_{hash}.rs", env!("CARGO_MANIFEST_DIR"));
        let src = std::fs::read_to_string(path).unwrap();
        let mut words = std::collections::BTreeMap::new();
        for l in src.lines() {
            let p: Vec<&str> = l.split_whitespace().collect();
            if p.len() >= 4 && p[0] == "//" && p[1].starts_with("0x") && p[1].ends_with(':') && p[2].len() == 8 && p[3].len() == 8 {
                let (Ok(a), Ok(hi), Ok(lo)) = (
                    u32::from_str_radix(&p[1][2..p[1].len() - 1], 16),
                    u32::from_str_radix(p[2], 16),
                    u32::from_str_radix(p[3], 16),
                ) else {
                    continue;
                };
                words.insert(a, (hi, lo));
            }
        }
        let mut out = Vec::new();
        for (i, (&a, &(hi, lo))) in words.iter().enumerate() {
            assert_eq!(a, i as u32 * 8);
            out.extend_from_slice(&lo.to_le_bytes());
            out.extend_from_slice(&hi.to_le_bytes());
        }
        out
    }

    /// Run the translated program; returns (kicks, data memory).
    pub fn translated(hash: &str, want: u64, entry: u32, top: u32, mem: &[u8; 16384]) -> (Vec<Vec<u128>>, Vec<u8>) {
        let mut vu = Vu::new(1);
        vu.write_code(0, &code(hash));
        assert_eq!(vu.segment_hash_pub(0), want);
        vu.data.copy_from_slice(mem);
        vu.vif_top = top;
        vu.vif_itop = 0;
        vu.start(entry);
        let mut kicks = Vec::new();
        assert!(vu.run(200_000_000, &mut |p| kicks.push(p.to_vec())));
        (kicks, vu.data.clone())
    }

    /// xorshift for the inputs.
    pub struct Rng(pub u64);
    impl Rng {
        pub fn u(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        pub fn f(&mut self, lo: f32, hi: f32) -> u32 {
            (lo + (hi - lo) * (self.u() % 1_000_000) as f32 / 1_000_000.0).to_bits()
        }
        pub fn v(&mut self, lo: f32, hi: f32, w: f32) -> V {
            [self.f(lo, hi), self.f(lo, hi), self.f(lo, hi), w.to_bits()]
        }
    }

    pub fn put(mem: &mut [u8; 16384], qw: u16, v: V) {
        Mem(mem).st(qw, v);
    }
    /// A SPRITE GIF tag, NREG 5 (NLOOP patched by the program).
    pub fn tag() -> V {
        [0, (1 << 14) | (6 << 15) | (5 << 28), 0x52512, 0]
    }

    pub fn compare(name: &str, native: (bool, Vec<Vec<u128>>, Vec<u8>), want: (Vec<Vec<u128>>, Vec<u8>)) {
        assert!(native.0, "{name}: declined");
        let sprites: u64 = want.0.iter().map(|k| (k[0] as u64) & 0x7fff).sum();
        assert_eq!(native.1.len(), want.0.len(), "{name}: kick count");
        for (i, (a, b)) in native.1.iter().zip(&want.0).enumerate() {
            assert_eq!(a, b, "{name}: kick {i}");
        }
        assert!(native.2 == want.1, "{name}: data memory");
        eprintln!("{name}: {} kicks, {sprites} sprites", want.0.len());
    }

    fn header(r: &mut Rng, mem: &mut [u8; 16384]) {
        let f = |x: f32| x.to_bits();
        put(mem, 0, [r.f(0.8, 1.6), 0, 0, 0]);
        put(mem, 1, [0, r.f(0.8, 1.6), 0, 0]);
        put(mem, 2, [r.f(-0.2, 0.2), r.f(-0.2, 0.2), f(1.0), f(1.0)]);
        put(mem, 3, [r.f(-0.5, 0.5), r.f(-0.5, 0.5), r.f(-0.5, 0.0), r.f(1.5, 4.0)]);
        let scale = if r.u() % 2 == 0 { 256.0 } else { 1024.0 };
        put(mem, 4, [f(scale), f(scale * 0.875), f(-8388467.5 / 16.0), 0]);
        put(mem, 5, [f(2047.5), f(2047.5), f(8388467.5 / 16.0), 0]);
        put(mem, 6, [r.f(1.0, 2.0), r.f(1.0, 2.0), 0, 0]);
        put(mem, PACKET_A, tag());
        put(mem, PACKET_B, tag());
    }

    fn descriptor(r: &mut Rng, mem: &mut [u8; 16384], top: u16, entry: u32) {
        let t = |i: u16| top + i;
        let n = (r.u() % 60 + 1) as u32;
        let trails = (r.u() % 4 + 1) as u32;
        put(mem, t(0), [n, trails, r.f(0.0, 3.0), r.f(0.01, 0.1)]);
        put(mem, t(1), [r.f(0.0, 0.5), r.f(0.0, 1.0), r.f(0.05, 0.5), r.f(0.5, 3.0)]);
        put(mem, t(2), [r.f(-0.3, 0.3), 0, 0, 0]);
        put(mem, t(3), [r.f(0.05, 0.4), r.f(0.02, 0.3), r.u() as u32, r.u() as u32]);
        put(mem, t(4), [r.u() as u32, r.u() as u32, r.u() as u32, 0]);
        put(mem, t(5), [r.u() as u32, r.u() as u32, r.u() as u32, r.u() as u32]);
        put(mem, t(6), r.v(-1.0, 1.0, 0.0));
        for i in 7..=10 {
            put(mem, t(i), r.v(-0.5, 0.5, 0.0));
        }
        put(mem, t(11), r.v(-1.0, 1.0, 1.0));
        for i in 12..=14 {
            put(mem, t(i), r.v(-0.3, 0.3, 0.0));
        }
        put(mem, t(15), r.v(-20.0, 300.0, 100.0));
        put(mem, t(16), r.v(-40.0, 40.0, 1.0));
        put(mem, t(17), r.v(-40.0, 40.0, 20.0));
        put(mem, t(18), r.v(-40.0, 40.0, -10.0));
        put(mem, t(19), r.v(0.0, 1.0, 1.0));
        put(mem, t(20), r.v(0.0, 1.0, 1.0));
        if entry == 0x1460 {
            for i in 0..16 {
                put(mem, t(i), r.v(-1.0, 1.0, 0.5));
            }
            put(mem, t(20), r.v(0.0, 255.0, 0.0));
            put(mem, t(21), [r.f(0.2, 3.0), r.f(0.5, 4.0), r.f(8.0, 60.0), (r.u() % 200 + 1) as u32]);
            return;
        }
        // Ring.
        let segs = (r.u() % 8) as u32;
        put(mem, t(21), [segs, (r.u() % 8 + 1) as u32, r.f(1.0, 8.0), r.f(0.0, 0.5)]);
        let dir = if entry == 0x510 { DIR_DYNAMIC } else { DIR_COLOURED };
        for k in 0..=segs as u16 + 1 {
            put(mem, t(RING + k), r.v(-1.0, 1.0, 0.0));
            let seed = if r.u() % 5 == 0 { r.f(0.0, 0.99) } else { r.f(1.0, 2.0) };
            let mut e = r.v(-0.5, 0.5, 0.0);
            e[3] = seed;
            put(mem, t(RING + dir + k), e);
            if entry != 0x510 {
                let c = [(r.u() % 256) as u32, (r.u() % 256) as u32, (r.u() % 256) as u32, (r.u() % 256) as u32];
                put(mem, t(RING + COLOUR_COLOURED + k), c);
            }
        }
    }

    #[test]
    fn random_inputs_match_translated() {
        let mut r = Rng(0x1234_5678_9abc_def1);
        for case in 0..200 {
            for entry in [0x0000u32, 0x0510, 0x0a00, 0x0f10, 0x1460] {
                let top: u16 = if case % 2 == 0 { 7 } else { 0x9a };
                let mut mem = Box::new([0u8; 16384]);
                header(&mut r, &mut mem);
                descriptor(&mut r, &mut mem, top, entry);
                let want = translated("f55ee63416fe5dcf", HASH, entry, top as u32, &mem);
                let mut kicks = Vec::new();
                let ok = run(entry, top as u32, 0, &mut mem, &mut VuState::default(), &mut |k| kicks.push(k.to_vec()));
                compare(&format!("case {case} entry {entry:#06x}"), (ok, kicks, mem.to_vec()), want);
            }
        }
    }

    /// A GPU plan against the program: no memory written, the shader
    /// mirror's sprites equal to the kicked ones but for CLIP-boundary
    /// decisions. Returns (sprites agreeing, sprites).
    pub fn check_plan(entry: u32, top: u16, mem: &mut [u8; 16384], native: &mut dyn FnMut(&mut [u8; 16384], &mut dyn FnMut(&[u128])) -> bool, plan: fn(u32, u32, u32, &mut [u8; 16384]) -> Option<Vec<crate::gpu::GpuCall>>) -> (usize, usize) {
        let before = Box::new(*mem);
        let mut copy = Box::new(*mem);
        let mut gif = Vec::new();
        assert!(native(&mut copy, &mut |k| gif.extend_from_slice(k)));
        let calls = plan(entry, top as u32, 0, mem).expect("planned");
        assert!(mem[..] == before[..], "the plan wrote memory");
        if calls.is_empty() {
            assert!(gif.is_empty(), "nothing planned, the program kicked");
        } else if let Some(&t) = gif.first() {
            // (Empty: every sample culled; the GPU draw still sets PRIM.)
            assert_eq!(calls[0].tag >> 16, t >> 16, "tag");
        }
        let (same, got, want, _) = compare_sprites(&calls, &gif);
        (same, got.max(want))
    }

    #[test]
    fn gpu_plans_match_program() {
        let mut r = Rng(0x0004_5eed_9abc_def1);
        let mut n = std::collections::BTreeMap::new();
        for case in 0..200 {
            for entry in [0x0000u32, 0x0510, 0x0a00, 0x0f10, 0x1460] {
                let top: u16 = if case % 2 == 0 { 7 } else { 0x9a };
                let mut mem = Box::new([0u8; 16384]);
                header(&mut r, &mut mem);
                descriptor(&mut r, &mut mem, top, entry);
                let (s, t) = check_plan(entry, top, &mut mem, &mut |m, k| run(entry, top as u32, 0, m, &mut VuState::default(), k), plan_gpu);
                let e = n.entry(entry).or_insert((0, 0));
                e.0 += s;
                e.1 += t;
            }
        }
        for (entry, (same, total)) in n {
            eprintln!("entry {entry:#06x}: {same} of {total} sprites agree");
            assert!(total > 100 && same as f64 >= 0.99 * total as f64, "entry {entry:#06x}: {same} of {total}");
        }
    }

    #[test]
    fn lfsr_matches_vu() {
        let mut vu = Vu::new(1);
        for seed in [0u32, 1, 0x3f80_0001, 0xdead_beef, 0x7f_ffff] {
            vu.vf[1] = [seed; 4];
            vu.rinit(1, 0);
            let mut l = Lfsr::init(seed);
            for _ in 0..1000 {
                vu.rnext(0b1000, 2);
                assert_eq!(vu.vf[2][0], l.next());
            }
        }
    }
}
