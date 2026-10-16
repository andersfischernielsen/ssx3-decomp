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

fn emitter(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) {
    let d = |i: u16| m.ld(top.wrapping_add(i));
    let scr = Screen::load(m);
    let to_clip = |i: u16| xform(&scr.m, d(i));
    // Directions and points in clip space (M·D, w included).
    let spawn_base = to_clip(11);
    let spawn_rand = [to_clip(12), to_clip(13)];
    let spawn_step = to_clip(14);
    let gravity = to_clip(6);
    let vel_base = to_clip(7);
    let vel_rand = [to_clip(8), to_clip(9), to_clip(10)];
    let (d0, d1, d3, d4, d5) = (d(0), d(1), d(3), d(4), d(5));
    let size_growth = d(2)[0];
    let colour_d = [d(15), d(16), d(17), d(18)];
    let (st_a, st_b) = (st_corner(d(19)), st_corner(d(20)));
    let fade_step = d3[0];
    let trail_dt = d3[1];
    let mut seeds = EmitterSeeds {
        life: d5[3],
        vel: [d4[0], d4[1], d4[2]],
        colour: [d5[0], d5[1]],
        size: d5[2],
        spawn: [d3[2], d3[3]],
    };

    let mut out = Packets::new(PACKET_A, PACKET_B);
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
            let mut fade = ONE;
            let mut tk = t;
            let mut trails = d0[1] as u16 as i16;
            loop {
                let base = add(mul(gravity, bc(tk)), spawn);
                let pos = madd(mul(base, bc(ONE)), g_minus_v, bc(drag_poly(tk)));
                if fast::clip(pos, pos[3]) & 0x3f == 0 {
                    let rgba = ftoi0([rgb[0], rgb[1], rgb[2], mul1(colour[3], fade)]);
                    let q = div(ONE, pos[3]);
                    let h = mul(half, bc(q));
                    let p = mul(pos, bc(q));
                    let mut h = abs(h);
                    let hc = min(h, bc(F64));
                    h[0] = hc[0];
                    h[1] = hc[1];
                    let c = madd(mul(scr.offset, bc(ONE)), p, scr.scale);
                    let a = add(c, h);
                    let b = sub(c, h);
                    out.push(m, [st_a, rgba, ftoi4(b), st_b, ftoi4(a)]);
                    // Only drawn samples fade (the step is in the kick
                    // branch's delay slot).
                    fade = sub1(fade, fade_step);
                    if out.count == SPRITES_PER_KICK {
                        out.kick(m, kick);
                    }
                }
                tk = sub1(tk, trail_dt);
                if neg(tk) {
                    break;
                }
                trails = trails.wrapping_sub(1);
                if trails <= 0 {
                    break;
                }
            }
        }
        t = sub1(t, dt);
        left = left.wrapping_sub(1);
        index = add1(index, ONE);
        if neg(t) || left <= 0 {
            break;
        }
    }
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

fn ring(m: &mut Mem, top: u16, kind: Ring, kick: &mut dyn FnMut(&[u128])) {
    let d = |m: &Mem, i: u16| m.ld(top.wrapping_add(i));
    let scr = Screen::load(m);
    let (d0, d3, d21) = (d(m, 0), d(m, 3), d(m, 21));
    let vel_d = [d(m, 7), d(m, 8), d(m, 9), d(m, 10)];
    let spawn_d = [d(m, 11), d(m, 12), d(m, 13), d(m, 14)];
    let (st_a, st_b) = (st_corner(d(m, 19)), st_corner(d(m, 20)));
    let age_step = d0[3];
    let (fade_step, trail_dt) = (d3[0], d3[1]);
    let per_unit = d21[2];
    let s_step = add1(0, div(ONE, per_unit));
    let (dir_at, half_max) = match kind {
        Ring::Dynamic => (DIR_DYNAMIC, F64),
        _ => (DIR_COLOURED, F128),
    };

    let mut out = Packets::new(PACKET_A, PACKET_B);
    let mut segments = d21[0] as u16 as i16;
    let per_segment = d21[1] as u16 as i16;
    let mut age = d21[3];
    let mut base = top.wrapping_add(RING);
    loop {
        let at = |i: u16| m.ld(base.wrapping_add(i));
        let seg = Segment {
            dir0: at(dir_at),
            dir1: at(dir_at + 1),
            pos0: at(0),
            pos1: at(1),
            col0: itof0(at(COLOUR_COLOURED)),
            col1: itof0(at(COLOUR_COLOURED + 1)),
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
                ring_particle(m, &scr, &mut out, kind, &seg, &mut rng, s, age, kick, &RingConsts {
                    vel_d,
                    spawn_d,
                    st_a,
                    st_b,
                    fade_step,
                    trail_dt,
                    half_max,
                    top,
                });
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
    if out.count != 0 {
        out.kick(m, kick);
    }
}

struct RingConsts {
    vel_d: [V; 4],
    spawn_d: [V; 4],
    st_a: V,
    st_b: V,
    fade_step: u32,
    trail_dt: u32,
    half_max: u32,
    top: u16,
}

/// `a·(1 − s) + b·s` as `ACC = a·w; b·s + ACC`.
fn lerp(a: V, b: V, w: u32, s: u32) -> V {
    madd(mul(a, bc(w)), b, bc(s))
}

#[allow(clippy::too_many_arguments)]
fn ring_particle(
    m: &mut Mem,
    scr: &Screen,
    out: &mut Packets,
    kind: Ring,
    seg: &Segment,
    rng: &mut Lfsr,
    s: u32,
    age: u32,
    kick: &mut dyn FnMut(&[u128]),
    k: &RingConsts,
) {
    let d = |m: &Mem, i: u16| m.ld(k.top.wrapping_add(i));
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
    let d1 = d(m, 1);
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
    let colour_d = [d(m, 15), d(m, 16), d(m, 17), d(m, 18)];
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
        return;
    }
    let gravity = d(m, 6);
    let g_minus_v = sub(gravity, vel);
    let q = div(ONE, life);
    // (rgb, alpha before fade)
    let (rgb, alpha) = match kind {
        Ring::Dynamic => (min(max(colour, bc(0)), bc(F255)), colour[3]),
        Ring::Coloured => (min(max(colour, bc(0)), bc(F255)), colour[3]),
        Ring::Faded => {
            let u = mul1(age, q);
            let u = mul1(u, u);
            let u = sub1(ONE, u);
            let a = mul1(mul1(u, F128), d(m, 16)[3]);
            let c = min([colour[0], colour[1], colour[2], a], bc(F128));
            let c = max(c, bc(0));
            (min(max(c, bc(0)), bc(F255)), c[3])
        }
    };
    let growth = mul1(mul1(d(m, 2)[0], age), q);
    let size = madd1(add1(growth, d1[2]), d1[0], r_size);
    let half = scr.half_size(size);
    let mut fade = ONE;
    let mut tk = age;
    let mut trails = d(m, 0)[1] as u16 as i16;
    loop {
        let g = mul(d(m, 6), bc(tk));
        let poly = drag_poly(tk);
        let g = add(g, spawn);
        let p = madd(mul(g, bc(ONE)), g_minus_v, bc(poly));
        let c = xform(&scr.m, [p[0], p[1], p[2], ONE]);
        let visible = fast::clip(c, c[3]) & 0x3f == 0;
        let a = mul1(alpha, fade);
        // Here every sample fades, drawn or not (delay slot of the cull
        // branch).
        fade = sub1(fade, k.fade_step);
        if visible {
            let q = div(ONE, c[3]);
            let rgba = ftoi0([rgb[0], rgb[1], rgb[2], a]);
            let h = mul(half, bc(q));
            let cq = with_xyz(c, mul(c, bc(q)));
            let mut h = abs(h);
            let centre = madd(mul(scr.offset, bc(ONE)), cq, scr.scale);
            let hc = min(h, bc(k.half_max));
            h[0] = hc[0];
            h[1] = hc[1];
            let lo = sub(centre, h);
            let hi = add(centre, h);
            out.push(m, [k.st_a, rgba, ftoi4(hi), k.st_b, ftoi4(lo)]);
            if out.count == SPRITES_PER_KICK {
                out.kick(m, kick);
            }
        }
        tk = sub1(tk, k.trail_dt);
        if neg(tk) {
            break;
        }
        trails = trails.wrapping_sub(1);
        if trails <= 0 {
            break;
        }
    }
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
fn glitter(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    let d = |i: u16| m.ld(top.wrapping_add(i));
    let d21 = d(21);
    let count = d21[3] as u16;
    if count == 0 {
        return false;
    }
    let scr = Screen::load(m);
    let coef: [V; 16] = std::array::from_fn(|i| d(i as u16));
    let (d0, d1, d2, d3) = (coef[0], coef[1], coef[2], coef[3]);
    let colour = d(20);
    let st_a = [0, 0, ONE, 0];
    let st_b = [ONE, ONE, ONE, 0];

    // The patch normal direction: (D16 D17 D18 D18) · (M · (0,0,1,0)).
    let mz = xform(&scr.m, [0, 0, ONE, 0]);
    let normal = {
        let acc = mul(d(16), bc(mz[0]));
        let acc = madd(acc, d(17), bc(mz[1]));
        let acc = madd(acc, d(18), bc(mz[2]));
        madd(acc, d(18), bc(mz[3]))
    };

    // Hash state.
    let a0 = add(d0, d1);
    let b0 = add(d1, d2);
    let p16 = add(a0, bc(HASH_K)); // vf16.xyz
    let p17 = add(b0, bc(HASH_K)); // vf17.xyz
    let half_unit = mul1(ONE, F1_5); // vf16.w
    let lift = mul1(ONE, F12); // vf17.w
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

    let mut out = Packets::new(PACKET_A, PACKET_B);
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
        if facing {
            let eval = |k: usize, u: u32, v: u32, w: u32| -> V {
                let acc = mul(coef[k], bc(u));
                let acc = madd(acc, coef[k + 1], bc(v));
                let acc = madd(acc, coef[k + 2], bc(w));
                madd(acc, coef[k + 3], bc(ONE))
            };
            let rows = [eval(0, u1, v1, w1), eval(4, u1, v1, w1), eval(8, u1, v1, w1), eval(12, u1, v1, w1)];
            let p = {
                let acc = mul(rows[0], bc(u2));
                let acc = madd(acc, rows[1], bc(v2));
                let acc = madd(acc, rows[2], bc(w2));
                madd(acc, rows[3], bc(ONE))
            };
            let p = [p[0], p[1], add1(p[2], lift), 0];
            let cl = xform(&scr.m, [p[0], p[1], p[2], ONE]);
            if fast::clip(cl, cl[3]) & 0x3f == 0 {
                let alpha = min1(dot, ONE);
                let hp = mul(scr.proj, bc(d21[0]));
                let hs = mul(hp, scr.scale);
                let half = [hs[0], hs[1], zero_sub(), zero_sub()];
                let q = div(ONE, cl[3]);
                let mut half = mul(half, bc(q));
                let ha = abs(half);
                half[0] = ha[0];
                half[1] = ha[1];
                let rgba = ftoi0([colour[0], colour[1], colour[2], mul1(alpha, F128)]);
                let cq = with_xyz(cl, mul(cl, bc(q)));
                let hm = max(half, bc(d21[1]));
                half[0] = hm[0];
                half[1] = hm[1];
                let centre = madd(mul(scr.offset, bc(ONE)), cq, scr.scale);
                let hm = min(half, bc(d21[2]));
                half[0] = hm[0];
                half[1] = hm[1];
                let hi = add(centre, half);
                let lo = sub(centre, half);
                out.push(m, [st_a, rgba, ftoi4(hi), st_b, ftoi4(lo)]);
                done = done.wrapping_add(1);
                if done == count {
                    break;
                }
                if out.count == SPRITES_PER_KICK {
                    out.kick(m, kick);
                }
                continue;
            }
        }
        done = done.wrapping_add(1);
        if done == count {
            break;
        }
    }
    // Kicked even when empty.
    out.kick(m, kick);
    true
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
