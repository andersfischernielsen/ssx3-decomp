//! Sun shadows (options: shadows; `docs/plans/shadow-maps.md`).
//!
//! The 3D draws are drawn a second time into a depth atlas from the sun
//! (shadow.wgsl: each GPU-computed vertex goes back to the world through the
//! terrain camera), once per cascade (four nested boxes ahead of the camera,
//! each 3× wider than the last, one quarter of the atlas each). The same
//! draws then look their world position up in it as they are drawn
//! (gpuvu.wgsl `sun_factor`) and darken what the sun does not reach. World
//! positions are relative to a point near the camera (`post::cam_matrices`):
//! the game's coordinates are too large for f32.
//!
//! The sun is the key light the game lights its meshes with (the linear
//! terms of their SH lighting, `gs::Gs::key_light`), so shadows fall the
//! way the shading says the light comes from.
//!
//! Tuning while this is explored (environment): `SSX3_SUN=azimuth,elevation`
//! (degrees, the world's z is up; overrides the key light),
//! `SSX3_SHADOW_RANGE` (the widest cascade's half width in world units),
//! `SSX3_SHADOW_SIZE` (the atlas' texels), `SSX3_SHADOW_STRENGTH` (0..1).
//! The debug knobs ([`DEBUG_KNOBS`]: views, sun, casters, the bake) also
//! change while running, from the settings overlay's debug group.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{OnceLock, RwLock};

use crate::gs::Cam;
use crate::post;

/// The cascades.
pub const CASCADES: usize = 4;

/// The light this frame: matrices for the shadow pass and the receivers.
#[derive(Clone, Copy, Debug)]
pub struct Light {
    /// GS screen (homogeneous) → rebased world (columns).
    pub s2w: [[f32; 4]; 4],
    /// Rebased world → light clip space per cascade (columns).
    pub lights: [[[f32; 4]; 4]; CASCADES],
    /// Towards the sun (world, unit).
    pub sun: [f32; 3],
    /// The eye (rebased world).
    pub eye: [f32; 3],
    /// One texel in atlas UV.
    pub texel: f32,
    /// World units per texel, per cascade.
    pub world_texel: [f32; CASCADES],
    /// The point world positions are relative to (absolute world).
    pub origin: [f32; 3],
}

fn env_f(k: &str) -> Option<Vec<f64>> {
    let v = std::env::var(k).ok()?;
    let r: Vec<f64> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    (!r.is_empty()).then_some(r)
}

/// Whether draws of this program cast and receive sun shadows: the 3D
/// geometry (meshes, riders, terrain). Not particles, and not the strips:
/// board trails and wakes, blended decals on the snow (darkening their
/// colour changes the blend, not the light, and they would shadow the
/// snow they cover).
pub fn receives(p: ssx3_render::gpu::Program) -> bool {
    use ssx3_render::gpu::Program as P;
    matches!(p, P::Mat3Mesh | P::Mat6Skin | P::Bezier)
}

/// The renderer's terrain lighting (options: lighting): L = sky fill
/// (between `ground` facing down and `sky` facing up) + `sun` · N·L · lit.
/// Fitted per course to the game's baked look (`tools/fit_lighting.py`).
#[derive(Clone, Copy, Debug)]
pub struct Lighting {
    pub sun: [f32; 3],
    pub sky: [f32; 3],
    pub ground: [f32; 3],
}

/// Each course's tone: the gain (neutral: the colour is the game's live
/// light's, which follows the lighting triggers' zones; per-channel gains
/// cast whole courses purple or green) from the game's live light to the
/// terrain's that makes the mean terrain light equal the bake's
/// (`tools/fit_lighting.py --tone`, the scenarios' reference shots,
/// 2026-10-25), by track code; the sun gain is 0.6 everywhere (the spatial
/// structure is the real sun's, the bake's pattern is art).
const COURSES: &[(&str, Lighting)] = &[
    ("ARA1", tone([1.241, 1.241, 1.241])), // Snow Jam
    ("BRA2", tone([0.597, 0.597, 0.597])), // Metro City (night)
    ("CRA3", tone([1.364, 1.364, 1.364])), // Ruthless Ridge
    ("DRA4", tone([0.707, 0.707, 0.707])), // Intimidator
    ("ERA5", tone([1.228, 1.228, 1.228])), // Gravitude
    ("ASS1", tone([0.997, 0.997, 0.997])), // R&B
    ("DSS2", tone([0.893, 0.893, 0.893])), // Style Mile
    ("ESS3", tone([1.133, 1.133, 1.133])), // Kick Doubt
    ("ABA1", tone([1.381, 1.381, 1.381])), // Crow's Nest
    ("CBA2", tone([1.769, 1.769, 1.769])), // Launch Time
    ("EBA3", tone([1.500, 1.500, 1.500])), // Much-2-Much
    ("BHP1", tone([1.146, 1.146, 1.146])), // The Junction
    ("CHP2", tone([1.389, 1.389, 1.389])), // Schizophrenia
    ("EHP3", tone([1.360, 1.360, 1.360])), // Perpendiculous
    ("ABC1", tone([1.229, 1.229, 1.229])), // Happiness (and its jam)
    ("DBC2", tone([0.827, 0.827, 0.827])), // Ruthless (and its jam)
    ("EBC3", tone([1.087, 1.087, 1.087])), // The Throne (and its jam)
];

/// A course's lighting: its ambient gain, the common sun gain.
const fn tone(ambient: [f32; 3]) -> Lighting {
    Lighting {
        sun: [0.6, 0.6, 0.6],
        sky: ambient,
        ground: ambient,
    }
}

/// The gains from the game's live light (the lit meshes' ambient and key
/// colour, 128 = 1) to the terrain's for a course not in the table.
const DEFAULT_LIGHTING: Lighting = tone([1.117, 1.117, 1.117]);

/// The terrain lighting of course `world` (track code): `SSX3_LIGHT=sun r,g,b;sky r,g,b;
/// ground r,g,b` (parts given override), else the course's fit.
pub fn lighting(world: &str) -> Lighting {
    let mut l = COURSES
        .iter()
        .find(|(w, _)| w.eq_ignore_ascii_case(world))
        .map_or(DEFAULT_LIGHTING, |c| c.1);
    static ENV: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    for part in ENV
        .get_or_init(|| std::env::var("SSX3_LIGHT").unwrap_or_default())
        .split(';')
    {
        let mut it = part.split_whitespace();
        let (Some(k), Some(v)) = (it.next(), it.next()) else {
            continue;
        };
        let c: Vec<f32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
        if c.len() < 3 {
            continue;
        }
        let c = [c[0], c[1], c[2]];
        match k {
            "sun" => l.sun = c,
            "sky" => l.sky = c,
            "ground" => l.ground = c,
            _ => {}
        }
    }
    l
}

/// The debug knobs that can change while running: the environment
/// variables they start from, set again by [`set_debug_knob`] (the
/// settings overlay's debug group, F10). `docs/options.md` lists them.
pub const DEBUG_KNOBS: &[&str] = &[
    "SSX3_SHADOW_SHOW",
    "SSX3_GFX_SHOW",
    "SSX3_SUN",
    "SSX3_CASTERS",
    "SSX3_BAKED",
    "SSX3_SHADOW_DEBUG",
];

/// The `SSX3_SHADOW_DEBUG` switches that only change what is drawn: they
/// follow [`set_debug_knob`] (bit i of the live masks). The others (logs:
/// print, lightlog, recvlog, items, gpustats, blend) are read once from
/// the environment.
const LIVE_SWITCHES: &[&str] = &[
    "no-pass",
    "no-recv",
    "no-world",
    "no-items",
    "no-strips",
    "no-pass2",
    "baked",
    "tint",
    "no-sub",
    "no-add",
    "sub-only",
    "game-shadows",
    "flare-sun",
];

/// The sources of live switches: `SSX3_SHADOW_DEBUG`, `SSX3_BAKED`,
/// `SSX3_CASTERS`, `SSX3_SUN` (`flare`).
const SW_DEBUG: usize = 0;
const SW_BAKED: usize = 1;
const SW_CASTERS: usize = 2;
const SW_SUN: usize = 3;

struct Knobs {
    /// `SSX3_SHADOW_SHOW` (f32 bits).
    show: AtomicU32,
    /// Live switches per source.
    switches: [AtomicU32; 4],
    /// `SSX3_SUN=azimuth,elevation`.
    sun: RwLock<Option<(f64, f64)>>,
}

fn knobs() -> &'static Knobs {
    static K: OnceLock<Knobs> = OnceLock::new();
    K.get_or_init(|| {
        let env = |k: &str| std::env::var(k).unwrap_or_default();
        let sun = env("SSX3_SUN");
        Knobs {
            show: AtomicU32::new(show_code(&env("SSX3_SHADOW_SHOW")).to_bits()),
            switches: [
                AtomicU32::new(switch_mask(&env("SSX3_SHADOW_DEBUG"))),
                AtomicU32::new(switch_mask(&env("SSX3_BAKED"))),
                AtomicU32::new(switch_mask(&env("SSX3_CASTERS"))),
                AtomicU32::new(sun_switches(&sun)),
            ],
            sun: RwLock::new(sun_angles(&sun)),
        }
    })
}

fn switch_mask(list: &str) -> u32 {
    list.split(',')
        .filter_map(|w| LIVE_SWITCHES.iter().position(|s| *s == w.trim()))
        .fold(0, |m, i| m | 1 << i)
}

fn sun_switches(v: &str) -> u32 {
    if v.trim() == "flare" {
        switch_mask("flare-sun")
    } else {
        0
    }
}

fn sun_angles(v: &str) -> Option<(f64, f64)> {
    let r: Vec<f64> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    (r.len() >= 2).then(|| (r[0], r[1]))
}

/// `SSX3_SHADOW_SHOW`'s code for the shader (`sh.p.w`).
fn show_code(v: &str) -> f32 {
    match v.trim() {
        "factor" => 1.0,
        "cascade" => 2.0,
        "ndl" => 3.0,
        // (For fitting the lighting: the lit terrain only, unprocessed.)
        "normals" => 4.0,
        "sunlit" => 5.0,
        // (n.x, n.y as 0..1, lit as 0.5..1: one run for the fit.)
        "fit" => 6.0,
        // (Board trails: their vertex colour, x2.)
        "stripcol" => 7.0,
        // (World position bands, 200 units, on every GPU draw.)
        "wp" => 8.0,
        _ => 0.0,
    }
}

/// Set debug knob `key` (one of [`DEBUG_KNOBS`]) to `value` (as its
/// environment variable takes it; empty: off) while running. Returns
/// whether `key` is one.
pub fn set_debug_knob(key: &str, value: &str) -> bool {
    let k = knobs();
    match key {
        "SSX3_SHADOW_SHOW" => k.show.store(show_code(value).to_bits(), Ordering::Relaxed),
        "SSX3_GFX_SHOW" => post::set_show(value),
        "SSX3_SUN" => {
            *k.sun.write().unwrap() = sun_angles(value);
            k.switches[SW_SUN].store(sun_switches(value), Ordering::Relaxed);
        }
        "SSX3_CASTERS" => {
            k.switches[SW_CASTERS].store(switch_mask(value), Ordering::Relaxed);
            ssx3_render::world_casters::set_filter(value);
        }
        "SSX3_BAKED" => k.switches[SW_BAKED].store(switch_mask(value), Ordering::Relaxed),
        "SSX3_SHADOW_DEBUG" => k.switches[SW_DEBUG].store(switch_mask(value), Ordering::Relaxed),
        _ => return false,
    }
    true
}

/// `SSX3_SHADOW_SHOW`: the receivers show the shadow factor (1), their
/// cascade (2) or N·L (3) instead of their colour (0: off); live.
pub fn show() -> f32 {
    f32::from_bits(knobs().show.load(Ordering::Relaxed))
}

/// Debug switch `what` is on: `SSX3_SHADOW_DEBUG` (comma-separated
/// switches), `SSX3_BAKED`, `SSX3_CASTERS` or `SSX3_SUN=flare` holds it.
/// The drawing switches (`LIVE_SWITCHES`) follow [`set_debug_knob`];
/// the log switches stay as the environment set them.
pub fn debug(what: &str) -> bool {
    if let Some(i) = LIVE_SWITCHES.iter().position(|s| *s == what) {
        return knobs()
            .switches
            .iter()
            .any(|m| m.load(Ordering::Relaxed) & (1 << i) != 0);
    }
    static S: OnceLock<String> = OnceLock::new();
    S.get_or_init(|| std::env::var("SSX3_SHADOW_DEBUG").unwrap_or_default())
        .split(',')
        .any(|w| w == what)
}

/// The tuning values (read once; the sun is live, [`set_debug_knob`]).
#[derive(Clone, Copy, Debug)]
pub struct Tuning {
    /// `SSX3_SUN`: (azimuth, elevation) in degrees.
    pub sun: Option<(f64, f64)>,
    pub range: f64,
    pub size: u32,
    pub strength: f32,
    /// `SSX3_SHADOW_TINT=r,g,b`: full shadow's multiplier (default the
    /// cool shade of the game's baked shadows on Snow Jam's snow).
    pub tint: [f32; 3],
}

pub fn tuning() -> Tuning {
    static T: OnceLock<Tuning> = OnceLock::new();
    let mut t = *T.get_or_init(|| Tuning {
        sun: None,
        range: env_f("SSX3_SHADOW_RANGE").map_or(24000.0, |v| v[0]),
        size: env_f("SSX3_SHADOW_SIZE").map_or(8192, |v| (v[0] as u32).clamp(512, 8192)),
        strength: env_f("SSX3_SHADOW_STRENGTH").map_or(1.0, |v| v[0] as f32),
        tint: env_f("SSX3_SHADOW_TINT")
            .filter(|v| v.len() >= 3)
            .map_or([0.55, 0.6, 0.75], |v| {
                [v[0] as f32, v[1] as f32, v[2] as f32]
            }),
    });
    // (The sun follows the debug knob.)
    t.sun = *knobs().sun.read().unwrap();
    t
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn norm(a: [f64; 3]) -> [f64; 3] {
    let l = dot(a, a).sqrt().max(1e-12);
    [a[0] / l, a[1] / l, a[2] / l]
}

/// The camera's eye and forward direction (absolute world), from the
/// terrain camera: along the screen centre's ray, where w is 0.
pub fn eye(c: &Cam) -> Option<([f64; 3], [f64; 3])> {
    let s = post::screen_matrix(c);
    let inv = post::invert(s)?;
    let k = if c.xyzf { 1.0 } else { 16.0 };
    let zs = |d: f64| d * (c.zmax as f64 + 1.0) / k;
    let a = post::unproject(&inv, 2048.0, 2048.0, zs(0.9));
    let b = post::unproject(&inv, 2048.0, 2048.0, zs(0.1));
    let w = |p: [f64; 3]| s[3][0] * p[0] + s[3][1] * p[1] + s[3][2] * p[2] + s[3][3];
    let (wa, wb) = (w(a), w(b));
    if (wb - wa).abs() < 1e-12 {
        return None;
    }
    let t = -wa / (wb - wa);
    let e = [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
    ];
    Some((e, norm(sub(b, a))))
}

/// The direction towards the point the game draws the sun at: frame pixel
/// (`x`, `y`) at depth `z` (GS Z), from the lens flare's visibility query,
/// back through the terrain camera.
pub fn sun_from_screen(c: &Cam, x: f32, y: f32, z: u32) -> Option<[f64; 3]> {
    let s = post::screen_matrix(c);
    let inv = post::invert(s)?;
    let k = if c.xyzf { 1.0 } else { 16.0 };
    let p = post::unproject(
        &inv,
        x as f64 + c.off[0] as f64,
        y as f64 + c.off[1] as f64,
        z as f64 / k,
    );
    let (e, _) = eye(c)?;
    let d = norm(sub(p, e));
    d.iter().all(|v| v.is_finite()).then_some(d)
}

/// The direction towards the sun (world, z up) at azimuth and elevation
/// (degrees).
pub fn sun_dir(azimuth: f64, elevation: f64) -> [f64; 3] {
    let (az, el) = (azimuth.to_radians(), elevation.to_radians());
    [el.cos() * az.cos(), el.cos() * az.sin(), el.sin()]
}

/// This frame's light, or None without a terrain camera. `key`: the
/// game's key light (towards it), used unless `SSX3_SUN` is set.
pub fn light(c: &Cam, key: [f32; 3]) -> Option<Light> {
    let t = tuning();
    let (_, s2w, o) = post::cam_matrices(c)?;
    let (e, f) = eye(c)?;
    let l = match t.sun {
        Some((az, el)) => sun_dir(az, el),
        None => norm(key.map(|x| x as f64)),
    };
    // Light axes: x, y across the map, z away from the sun.
    let zl = [-l[0], -l[1], -l[2]];
    let up = if l[2].abs() < 0.95 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let xl = norm(cross(up, zl));
    let yl = cross(zl, xl);
    let tile = t.size as f64 / 2.0;
    // Depth range: far towards the sun (tall casters), less beyond.
    let (near, far) = (t.range, t.range * 0.5);
    let mut lights = [[[0f32; 4]; 4]; CASCADES];
    let mut world_texel = [0f32; CASCADES];
    for k in 0..CASCADES {
        let r = t.range / 3f64.powi((CASCADES - 1 - k) as i32);
        // The box: ahead of the camera, most of it in view.
        let centre = [
            e[0] + f[0] * r * 0.7,
            e[1] + f[1] * r * 0.7,
            e[2] + f[2] * r * 0.7,
        ];
        // Snap the centre to whole texels (absolute world: no shimmer as
        // the camera moves).
        let texel = 2.0 * r / tile;
        let snap = |v: f64| (v / texel).round() * texel;
        let (u0, v0, w0) = (
            snap(dot(centre, xl)),
            snap(dot(centre, yl)),
            dot(centre, zl),
        );
        // Rebased world p' = p − o: u = dot(p' + o, xl) − u0.
        let row = |a: [f64; 3], off: f64, scale: f64| -> [f64; 4] {
            [
                a[0] * scale,
                a[1] * scale,
                a[2] * scale,
                (dot(o, a) - off) * scale,
            ]
        };
        // z = (d + near) / (near + far), d the distance from the centre.
        let rows = [
            row(xl, u0, 1.0 / r),
            row(yl, v0, 1.0 / r),
            row(zl, w0 - near, 1.0 / (near + far)),
            [0.0, 0.0, 0.0, 1.0],
        ];
        lights[k] = std::array::from_fn(|col| std::array::from_fn(|i| rows[i][col] as f32));
        world_texel[k] = texel as f32;
    }
    if debug("print") {
        eprintln!(
            "shadow: eye {:.0},{:.0},{:.0} fwd {:.2},{:.2},{:.2} sun {:.2},{:.2},{:.2}",
            e[0], e[1], e[2], f[0], f[1], f[2], l[0], l[1], l[2]
        );
    }
    Some(Light {
        s2w,
        lights,
        sun: l.map(|x| x as f32),
        eye: [
            (e[0] - o[0]) as f32,
            (e[1] - o[1]) as f32,
            (e[2] - o[2]) as f32,
        ],
        texel: 1.0 / t.size as f32,
        world_texel,
        origin: o.map(|x| x as f32),
    })
}

/// The shadow map, its sampler and the uniforms (`ShadowU` in gpuvu.wgsl).
pub struct Shadow {
    pub size: u32,
    /// The world's casters on the GPU: (generation, buffer, vertices).
    pub casters: Option<(u64, wgpu::Buffer, u32)>,
    /// Their alpha cut-outs: buffer (position, UV) and runs per texture.
    pub cutouts: Option<(wgpu::Buffer, Vec<(i16, u32, u32)>)>,
    pub view: wgpu::TextureView,
    /// A 1×1 map for the shadow pass itself (it draws into `view`).
    pub dummy: wgpu::TextureView,
    pub cmp: wgpu::Sampler,
    pub ubuf: wgpu::Buffer,
}

/// `ShadowU`'s size in bytes.
const U_BYTES: u64 = 464;

impl Shadow {
    pub fn new(dev: &wgpu::Device, size: u32) -> Shadow {
        let depth = |size: u32, label| {
            dev.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: size,
                    height: size,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: crate::draw::SHADOW_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default())
        };
        let ubuf = dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx shadow uniforms"),
            size: U_BYTES,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let cmp = dev.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("ssx3-gfx shadow compare"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        Shadow {
            size,
            casters: None,
            cutouts: None,
            view: depth(size, "ssx3-gfx shadow map"),
            dummy: depth(1, "ssx3-gfx shadow dummy"),
            cmp,
            ubuf,
        }
    }

    /// This frame's uniforms (`None`: shadows off).
    pub fn upload(&self, queue: &wgpu::Queue, l: Option<&Light>, show: f32, lt: Lighting) {
        let mut w: Vec<f32> = Vec::with_capacity(U_BYTES as usize / 4);
        match l {
            Some(l) => {
                w.extend(l.s2w.iter().flatten());
                w.extend(l.lights.iter().flatten().flatten());
                w.extend([l.sun[0], l.sun[1], l.sun[2], tuning().strength]);
                w.extend([l.eye[0], l.eye[1], l.eye[2], 1.0]);
                w.extend([l.texel, 0.0, 1.0, show]);
                w.extend(l.world_texel);
                w.extend([l.origin[0], l.origin[1], l.origin[2], 0.0]);
                let t = tuning();
                w.extend([t.tint[0], t.tint[1], t.tint[2], 0.0]);
                for c in [lt.sun, lt.sky, lt.ground] {
                    w.extend([c[0], c[1], c[2], 0.0]);
                }
            }
            None => w.resize(U_BYTES as usize / 4, 0.0),
        }
        let b: Vec<u8> = w.iter().flat_map(|v| v.to_le_bytes()).collect();
        queue.write_buffer(&self.ubuf, 0, &b);
    }

    /// The world's casters, uploaded when they changed.
    pub fn set_casters(
        &mut self,
        dev: &wgpu::Device,
        queue: &wgpu::Queue,
        c: &ssx3_render::world_casters::Casters,
    ) {
        if self
            .casters
            .as_ref()
            .is_some_and(|(g, _, _)| *g == c.generation)
        {
            return;
        }
        let bytes: Vec<u8> = c
            .triangles
            .iter()
            .flatten()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let buf = dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx world casters"),
            size: (bytes.len() as u64).max(16),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&buf, 0, &bytes);
        let cut: Vec<u8> = c
            .cutouts
            .iter()
            .flatten()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let cbuf = dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx world cut-outs"),
            size: (cut.len() as u64).max(20),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&cbuf, 0, &cut);
        self.cutouts = Some((cbuf, c.cutout_runs.clone()));
        if debug("print") {
            eprintln!(
                "shadow: world casters generation {}: {} patches, {} instances, {} triangles",
                c.generation,
                c.patches,
                c.instances,
                c.triangles.len() / 3
            );
        }
        self.casters = Some((c.generation, buf, c.triangles.len() as u32));
    }

    /// Bind group 1's shadow entries: the map, or the placeholder (for
    /// the shadow pass).
    pub fn entries(&self, real: bool) -> [wgpu::BindGroupEntry<'_>; 3] {
        [
            wgpu::BindGroupEntry {
                binding: 5,
                resource: self.ubuf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 6,
                resource: wgpu::BindingResource::TextureView(if real {
                    &self.view
                } else {
                    &self.dummy
                }),
            },
            wgpu::BindGroupEntry {
                binding: 7,
                resource: wgpu::BindingResource::Sampler(&self.cmp),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sun_dir_is_unit_and_up() {
        let d = sun_dir(30.0, 45.0);
        assert!((dot(d, d) - 1.0).abs() < 1e-12);
        assert!(d[2] > 0.7 && d[2] < 0.71);
    }
}
