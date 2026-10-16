//! Phase 3 stage 2B (`SSX3_RENDER_NATIVE=type3`): the triangles of the
//! type-3 VU1 program (d159716b) drawn with wgpu's hardware rasterizer
//! inside native runs (`native.rs`), everything else still emulated.
//!
//! The type-3 XGKICK packets go through the GS as usual (GIF tags, PRIM,
//! RGBAQ/ST/XYZ2, ADC, strips and fans), so each triangle is decoded in
//! stream order with the GS's register state at that point (`Draw`). With
//! `Gs::n3_capture` set, `draw_primitive` offers the triangle to
//! [`GpuGs::n3_push`] instead of the compute rasterizer. Accepted triangles
//! are collected; consecutive ones (across packets) form one native run,
//! which is queued when anything else reaches the GS backend: an emulated
//! primitive, a batch close (sync, readback, display, transfers that touch
//! the run's pages or textures), or a triangle that needs other surfaces.
//!
//! Per run, triangles are drawn in submission order with render pipelines
//! keyed by the fixed-function state (blend, colour write mask, depth
//! compare/write); everything else (texture, TFX, fog, alpha test, FBA) is
//! read per triangle from the compute GS's setup record by the fragment
//! shader (`native3.wgsl`), which reuses gs.wgsl's texturing functions.
//!
//! Triangles whose state the hardware path cannot express go to the
//! emulated GS (counted per [`Fallback`] reason).

use std::collections::HashMap;
use std::sync::Arc;

use super::super::raster::{Blend, Draw, FbFmt, Pages, Prim, Shape};
use super::super::Vertex;
use super::native::{NativeStats, COLOR_FORMAT, DEPTH_FORMAT};
use super::{GpuGs, NativeSurfaces, SHADER, SLOT};

const N3_SHADER: &str = include_str!("native3.wgsl");
const N3_DUAL: &str = include_str!("native3_dual.wgsl");
const VWORDS: usize = 17;

/// Why a type-3 triangle went to the emulated GS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallback {
    /// Not a triangle (lines of entry 0x3bf0).
    NotTri = 0,
    /// Alpha test with AFAIL other than KEEP (FB_ONLY needs two pipelines).
    AlphaFail,
    /// Blend with destination alpha, or DATE.
    DestAlpha,
    /// Blend equation with no fixed-function form (factor 1 + C, ...).
    Blend,
    /// PABE with blending.
    Pabe,
    /// Blending without COLCLAMP (wrapping).
    NoColclamp,
    /// FBMSK other than whole bytes.
    Fbmsk,
    /// CT16 frame (dithering, 5-bit rounding) or SCANMSK.
    FrameFormat,
    /// Z32 buffer.
    Z32,
    /// Surfaces a native run refuses (aliasing, too large, past the
    /// buffer width).
    Surface,
    /// The texture overlaps the frame/Z the run draws.
    TexFeedback,
    /// Blend factor As from a source alpha that can exceed 0x80.
    AlphaOver,
}

const REASONS: [&str; 12] = [
    "not-tri",
    "afail",
    "dest-alpha",
    "blend",
    "pabe",
    "no-colclamp",
    "fbmsk",
    "frame-fmt",
    "z32",
    "surface",
    "tex-feedback",
    "alpha>0x80",
];

/// Statistics (`GpuStats::n3`).
#[derive(Default, Clone, Copy, Debug)]
pub struct N3Stats {
    /// Triangles drawn natively, runs and draw calls.
    pub tris: u64,
    pub runs: u64,
    pub draws: u64,
    /// Triangles sent to the emulated GS, per reason.
    pub fallback: [u64; 12],
    /// Runs the composition refused (their triangles went emulated).
    pub refused_runs: u64,
    /// FB_ONLY alpha-tested triangles by outcome: always pass, always
    /// fail, mixed (two draws).
    pub fb_only_modes: [u64; 3],
    /// Texels read to bound texture alpha.
    pub scanned_texels: u64,
    /// Why runs ended: an emulated primitive, a batch close (sync, display,
    /// readback, ...), a transfer touching the run, other surfaces.
    pub flush_why: [u64; 4],
}

impl N3Stats {
    pub fn report(&self, native: &NativeStats) -> String {
        let fb: u64 = self.fallback.iter().sum();
        let mut s = format!(
            "[render-native type3] native tris {} in {} runs ({} draw calls), fallback tris {} ({:.2}%), refused runs {}",
            self.tris,
            self.runs,
            self.draws,
            fb,
            100.0 * fb as f64 / (fb + self.tris).max(1) as f64,
            self.refused_runs
        );
        for (i, n) in self.fallback.iter().enumerate() {
            if *n > 0 {
                s += &format!("\n    fallback {}: {}", REASONS[i], n);
            }
        }
        s += &format!(
            "\n    FB_ONLY tris: always pass {}, always fail {}, mixed {}; texels scanned {}",
            self.fb_only_modes[0], self.fb_only_modes[1], self.fb_only_modes[2], self.scanned_texels
        );
        s += &format!(
            "\n    runs ended by: emulated prim {}, close {}, transfer {}, surfaces {}",
            self.flush_why[0], self.flush_why[1], self.flush_why[2], self.flush_why[3]
        );
        s += &format!(
            "\n    native runs (all) {}, refused {}, {:.1} Mpx round-tripped",
            native.runs,
            native.refused,
            native.pixels as f64 / 1e6
        );
        s
    }
}

/// Fixed-function state of a triangle: a pipeline key and the blend
/// constant (FIX).
#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    key: u32,
    fix: u32,
    /// The destination factor uses As (wrong when As > 0x80: the
    /// hardware clamps factors to 1).
    dst_as: bool,
    /// Alpha test with AFAIL = FB_ONLY: failing pixels write colour, not Z
    /// (drawn as two draws, see `n3_flush`).
    fb_only: bool,
}

// Pipeline key bits.
const K_SRC: u32 = 0; // 4 bits: factor index
const K_DST: u32 = 4; // 4 bits
const K_OP: u32 = 8; // 2 bits: 0 replace, 1 add, 2 subtract, 3 reverse subtract
const K_MASK: u32 = 10; // 4 bits: ColorWrites
const K_ZCMP: u32 = 14; // 2 bits: 0 always, 1 gequal, 2 greater, 3 never
const K_ZWRITE: u32 = 1 << 16;
const K_DUAL: u32 = 1 << 17;

// Factor indices.
const F_ZERO: u32 = 0;
const F_ONE: u32 = 1;
const F_SRC1A: u32 = 2;
const F_INV_SRC1A: u32 = 3;
const F_CONST: u32 = 4;
const F_INV_CONST: u32 = 5;

fn factor(i: u32) -> wgpu::BlendFactor {
    match i {
        F_ZERO => wgpu::BlendFactor::Zero,
        F_ONE => wgpu::BlendFactor::One,
        F_SRC1A => wgpu::BlendFactor::Src1Alpha,
        F_INV_SRC1A => wgpu::BlendFactor::OneMinusSrc1Alpha,
        F_CONST => wgpu::BlendFactor::Constant,
        _ => wgpu::BlendFactor::OneMinusConstant,
    }
}

/// The fixed-function state of `d`, or why it has none.
fn classify(d: &Draw, dual_ok: bool) -> Result<State, Fallback> {
    if d.date {
        return Err(Fallback::DestAlpha);
    }
    if d.fb_fmt == FbFmt::C16 || d.dither.is_some() || d.scanmsk >= 2 {
        return Err(Fallback::FrameFormat);
    }
    if d.ate && d.afail > 1 {
        return Err(Fallback::AlphaFail);
    }
    let fb_only = d.ate && d.afail == 1;
    let mut mask = 0u32;
    for i in 0..4 {
        match (d.fbmsk >> (8 * i)) & 0xFF {
            0 => mask |= 1 << i,
            0xFF => {}
            _ => return Err(Fallback::Fbmsk),
        }
    }
    let mut key = mask << K_MASK;
    // Depth (gs.wgsl `pixel`): ZTE with NEVER draws nothing; Z is written
    // when tested and not masked.
    if d.zte {
        let cmp = match d.ztst {
            0 => 3,
            1 => 0,
            2 => 1,
            _ => 2,
        };
        key |= cmp << K_ZCMP;
        if d.ztst != 0 && !d.zmsk && d.z_used {
            key |= K_ZWRITE;
        }
    }
    let mut fix = 0;
    let mut dst_as = false;
    if d.abe && d.blend != Blend::Src {
        if d.pabe {
            return Err(Fallback::Pabe);
        }
        if !d.colclamp {
            return Err(Fallback::NoColclamp);
        }
        // out = (A - B) * C + D, as k0 + kc * C per colour.
        let sel = |x: u32, w: u32| (x == w) as i32;
        let cs = (sel(d.bd, 0), sel(d.ba, 0) - sel(d.bb, 0));
        let cd = (sel(d.bd, 1), sel(d.ba, 1) - sel(d.bb, 1));
        let uses_c = cs.1 != 0 || cd.1 != 0;
        let c_as = d.bc == 0;
        if uses_c && d.bc == 1 {
            return Err(Fallback::DestAlpha);
        }
        if cd.1 != 0 && c_as && !dual_ok {
            return Err(Fallback::Blend);
        }
        let (fc, fic) = if c_as { (F_SRC1A, F_INV_SRC1A) } else { (F_CONST, F_INV_CONST) };
        // The source term is premultiplied (and biased) in the shader and
        // added with factor One; only its sign matters here.
        let src_pos = match cs {
            (1, _) | (0, 1) => true,
            (0, 0) | (0, -1) => false,
            _ => return Err(Fallback::Blend),
        };
        let (df, dn) = match cd {
            (0, 0) => (F_ZERO, false),
            (1, 0) => (F_ONE, false),
            (0, 1) => (fc, false),
            (1, -1) => (fic, false),
            (0, -1) => (fc, true),
            _ => return Err(Fallback::Blend),
        };
        let op = match (src_pos, dn) {
            (true, false) => 1,
            (true, true) => 2,
            (false, false) => 3,
            (false, true) => return Err(Fallback::Blend),
        };
        key |= F_ONE << K_SRC | df << K_DST | op << K_OP;
        if cd.1 != 0 && c_as {
            key |= K_DUAL;
            dst_as = true;
        }
        if cd.1 != 0 && !c_as {
            fix = d.fix as u32;
        }
    }
    Ok(State { key, fix, dst_as, fb_only })
}

/// A bitmap of the pixels covered by an FB_ONLY group (the GS's exact
/// triangle coverage, which the hardware rasterizer reproduces).
#[derive(Default)]
struct Coverage {
    rows: Vec<[u64; 32]>,
    /// Rows touched since the last clear.
    y0: i32,
    y1: i32,
}

impl Coverage {
    fn clear(&mut self) {
        if !self.rows.is_empty() && self.y0 <= self.y1 {
            for r in &mut self.rows[self.y0 as usize..=self.y1 as usize] {
                *r = [0; 32];
            }
        }
        self.y0 = i32::MAX;
        self.y1 = i32::MIN;
    }

    /// Add the pixels of `p`; true (and the bitmap restarts with them) if
    /// one was already covered.
    fn add(&mut self, t: &super::super::raster::TriSetup, p: &Prim) -> bool {
        if self.rows.is_empty() {
            self.rows = vec![[0; 32]; 2048];
            self.y0 = i32::MAX;
            self.y1 = i32::MIN;
        }
        let spans = |j: i64| {
            let (mut lo, mut hi) = (0i64, (p.x1 - t.bx0) as i64);
            for k in 0..3 {
                super::super::raster::edge_span(t.e0[k] + t.edy[k] * j, t.edx[k], &mut lo, &mut hi);
            }
            (t.bx0 as i64 + lo, t.bx0 as i64 + hi)
        };
        let mask = |a: i64, b: i64, w: usize| -> u64 {
            let (s, e) = ((w * 64) as i64, (w * 64 + 63) as i64);
            let (a, b) = (a.max(s), b.min(e));
            if a > b {
                return 0;
            }
            let n = (b - a + 1) as u32;
            let m = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
            m << (a - s)
        };
        let mut hit = false;
        for y in p.y0..=p.y1 {
            let (a, b) = spans((y - t.by0) as i64);
            if a > b || a < 0 || b >= 2048 {
                continue;
            }
            let row = &self.rows[y as usize];
            if (a as usize / 64..=b as usize / 64).any(|w| row[w] & mask(a, b, w) != 0) {
                hit = true;
                break;
            }
        }
        if hit {
            self.clear();
        }
        for y in p.y0..=p.y1 {
            let (a, b) = spans((y - t.by0) as i64);
            if a > b || a < 0 || b >= 2048 {
                continue;
            }
            let row = &mut self.rows[y as usize];
            for w in a as usize / 64..=b as usize / 64 {
                row[w] |= mask(a, b, w);
            }
            self.y0 = self.y0.min(y);
            self.y1 = self.y1.max(y);
        }
        hit
    }
}

/// Emit an FB_ONLY group: the passing pixels, then the failing ones.
fn close_group(group: &mut Option<(Call, usize)>, calls: &mut Vec<Call>) {
    if let Some((c, pipe_b)) = group.take() {
        let b = Call { pipe: pipe_b, scissor: c.scissor, fix: c.fix, first: c.first, count: c.count, inst: 1 };
        calls.push(c);
        calls.push(b);
    }
}

/// The range of texel alpha `d` can sample (level 0 and mipmaps; filtering
/// stays inside it): from the palette, TEXA, or by reading the texels from
/// the VRAM mirror (0..=255 when the mirror is not current there). Also
/// returns the texels read.
fn tex_alpha_range(d: &Draw, vram: &[u32], gpu_dirty: &Pages) -> ((u32, u32), u64) {
    use super::super::raster::TexKind;
    let Some(t0) = &d.tex else { return ((0, 0), 0) };
    match t0.kind {
        TexKind::Pal => {
            let it = t0.palette[..=t0.pal_mask].iter().map(|c| c >> 24);
            ((it.clone().min().unwrap_or(0), it.max().unwrap_or(0)), 0)
        }
        TexKind::C24 => ((if t0.aem { 0 } else { t0.ta0 }, t0.ta0), 0),
        TexKind::C16 => {
            let lo = if t0.aem { 0 } else { t0.ta0.min(t0.ta1) };
            ((lo, t0.ta0.max(t0.ta1)), 0)
        }
        TexKind::C32 => {
            if d.tex_pages.hits(gpu_dirty) {
                return ((0, 255), 0);
            }
            let (mut lo, mut hi, mut n) = (255, 0, 0u64);
            for t in d.tex_levels() {
                let (u0, u1) = super::super::raster::wrap_range(t.wms, t.tw, t.minu, t.maxu);
                let (v0, v1) = super::super::raster::wrap_range(t.wmt, t.th, t.minv, t.maxv);
                if (u1 - u0 + 1) as u64 * (v1 - v0 + 1) as u64 > 1 << 20 {
                    return ((0, 255), n);
                }
                for y in v0..=v1 {
                    for x in u0..=u1 {
                        let a = t.surface().read(vram, x, y) >> 24;
                        lo = lo.min(a);
                        hi = hi.max(a);
                    }
                    n += (u1 - u0 + 1) as u64;
                    if lo == 0 && hi == 255 {
                        return ((0, 255), n);
                    }
                }
            }
            ((lo, hi), n)
        }
    }
}

/// The alpha test on every value of `lo..=hi`: Some(true) all pass,
/// Some(false) all fail, None mixed.
fn alpha_test_range(atst: u32, aref: u32, lo: u32, hi: u32) -> Option<bool> {
    let all = |f: &dyn Fn(u32) -> bool| {
        let (a, b) = (f(lo), f(hi));
        // Every test here is monotone or an equality on the range.
        (a == b).then_some(a)
    };
    match atst {
        0 => Some(false),
        1 => Some(true),
        2 => all(&|a| a < aref),
        3 => all(&|a| a <= aref),
        5 => all(&|a| a >= aref),
        6 => all(&|a| a > aref),
        4 | 7 => {
            let eq = if lo == hi && lo == aref {
                Some(true)
            } else if aref < lo || aref > hi {
                Some(false)
            } else {
                None
            };
            eq.map(|e| if atst == 4 { e } else { !e })
        }
        _ => None,
    }
}

/// `SSX3_N3_PLANES=1` (diagnostics): interpolate with the compute GS's
/// attribute planes instead of the hardware's barycentrics.
fn planes_knob() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var_os("SSX3_N3_PLANES").is_some())
}

/// Added to interpolated colour and fog before truncating to integers
/// (`SSX3_N3_EPS`, default 1e-4): a parity knob (see native3.wgsl).
fn eps_knob() -> f32 {
    static E: std::sync::OnceLock<f32> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_N3_EPS").ok().and_then(|v| v.parse().ok()).unwrap_or(1e-4))
}

struct Tri {
    d: Arc<Draw>,
    v: [Vertex; 3],
    linear: bool,
    st: State,
    /// The compute GS's Z plane (`GpuGs::encode`): origin (x0 | y0 << 16),
    /// integer base, fraction, dZ/dx, dZ/dy.
    zp: [u32; 5],
    /// `SSX3_N3_PLANES`: the compute GS's attribute planes (origin, then
    /// R G B A S T Q F at the origin, d/dx, d/dy).
    planes: Option<Box<[u32; 25]>>,
    /// FB_ONLY alpha test: 0 always passes (or no test), 1 always fails
    /// (colour only), 2 mixed (pass and fail draws).
    mode: u8,
    /// FB_ONLY: covers a pixel an earlier triangle of its group covers, so
    /// a new group starts here.
    brk: bool,
}

/// One draw call of a run.
struct Call {
    pipe: usize,
    scissor: [u32; 4],
    fix: u32,
    first: u32,
    count: u32,
    /// Instance 1: the failing pixels of an FB_ONLY alpha test.
    inst: u32,
}

/// The draw calls of one run (`Cmd::NativeDraws`).
pub(super) struct List {
    /// The native run it belongs to (drawn in the run's unswizzle pass).
    pub(super) run: u32,
    slot: u32,
    vw: u32,
    vh: u32,
    calls: Vec<Call>,
}

struct Gpu {
    bgl: wgpu::BindGroupLayout,
    layout: wgpu::PipelineLayout,
    module: wgpu::ShaderModule,
    dual: bool,
    pipes: HashMap<u32, usize>,
    pipelines: Vec<wgpu::RenderPipeline>,
}

/// Pending triangles and per-submission draw lists.
#[derive(Default)]
pub(crate) struct N3 {
    tris: Vec<Tri>,
    surf: Option<NativeSurfaces>,
    /// Pages the pending run draws, and the pages its textures read.
    target_pages: Pages,
    tex_pages: Pages,
    last: Option<(Arc<Draw>, Result<State, Fallback>)>,
    /// Bounding box of the pending triangles (the run round-trips only it).
    rect: [i32; 4],
    /// Pixels covered by the open FB_ONLY group, and its state.
    cov: Coverage,
    cov_key: Option<(u32, u32, [i32; 4])>,
    /// Largest texel alpha of the last draw's texture (`tex_alpha_bound`).
    last_tex: Option<(Arc<Draw>, (u32, u32))>,
    gpu: Option<Box<Gpu>>,
    pub(super) lists: Vec<List>,
    pub(crate) stats: N3Stats,
}

impl N3 {
    pub(crate) fn pending(&self) -> bool {
        !self.tris.is_empty()
    }
}

fn entry(binding: u32, vis: wgpu::ShaderStages, ty: wgpu::BindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry { binding, visibility: vis, ty, count: None }
}

fn storage(read_only: bool) -> wgpu::BindingType {
    wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage { read_only },
        has_dynamic_offset: false,
        min_binding_size: None,
    }
}

impl Gpu {
    fn new(dev: &wgpu::Device) -> Gpu {
        let dual = dev.features().contains(wgpu::Features::DUAL_SOURCE_BLENDING);
        use wgpu::ShaderStages as S;
        let bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gs-n3-bgl"),
            entries: &[
                entry(0, S::FRAGMENT, storage(false)),
                entry(1, S::FRAGMENT, storage(true)),
                entry(2, S::VERTEX | S::FRAGMENT, storage(true)),
                entry(
                    3,
                    S::VERTEX | S::FRAGMENT,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                ),
                entry(4, S::FRAGMENT, storage(true)),
                entry(6, S::FRAGMENT, storage(false)),
            ],
        });
        let layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gs-n3"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
        let src = if dual {
            format!("enable dual_source_blending;\n{SHADER}\n{N3_SHADER}\n{N3_DUAL}")
        } else {
            format!("{SHADER}\n{N3_SHADER}")
        };
        let desc = wgpu::ShaderModuleDescriptor {
            label: Some("gs-n3.wgsl"),
            source: wgpu::ShaderSource::Wgsl(src.into()),
        };
        // As the GS module (gpu/mod.rs): every index stays in bounds.
        let module = if std::env::var_os("SSX3_GS_CHECKED").is_some() {
            dev.create_shader_module(desc)
        } else {
            // SAFETY: see `GpuGs::new`; vertex records are offsets the CPU
            // encoded into the data buffer it sized.
            unsafe { dev.create_shader_module_trusted(desc, wgpu::ShaderRuntimeChecks::unchecked()) }
        };
        Gpu { bgl, layout, module, dual, pipes: HashMap::new(), pipelines: Vec::new() }
    }

    fn pipeline(&mut self, dev: &wgpu::Device, key: u32) -> usize {
        if let Some(&i) = self.pipes.get(&key) {
            return i;
        }
        let op = (key >> K_OP) & 3;
        let blend = (op != 0).then(|| wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: factor((key >> K_SRC) & 15),
                dst_factor: factor((key >> K_DST) & 15),
                operation: match op {
                    1 => wgpu::BlendOperation::Add,
                    2 => wgpu::BlendOperation::Subtract,
                    _ => wgpu::BlendOperation::ReverseSubtract,
                },
            },
            alpha: wgpu::BlendComponent::REPLACE,
        });
        let dual = key & K_DUAL != 0;
        let zcmp = match (key >> K_ZCMP) & 3 {
            0 => wgpu::CompareFunction::Always,
            1 => wgpu::CompareFunction::GreaterEqual,
            2 => wgpu::CompareFunction::Greater,
            _ => wgpu::CompareFunction::Never,
        };
        let p = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gs-n3"),
            layout: Some(&self.layout),
            vertex: wgpu::VertexState {
                module: &self.module,
                entry_point: Some("n3_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(key & K_ZWRITE != 0),
                depth_compare: Some(zcmp),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &self.module,
                entry_point: Some(if dual { "n3_fs2" } else { "n3_fs" }),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: COLOR_FORMAT,
                    blend,
                    write_mask: wgpu::ColorWrites::from_bits_truncate((key >> K_MASK) & 15),
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let i = self.pipelines.len();
        self.pipelines.push(p);
        self.pipes.insert(key, i);
        i
    }
}

impl GpuGs {
    /// Offer a kicked type-3 primitive to the native path: true when it was
    /// taken (it will be drawn by a native run), false when the caller must
    /// draw it emulated (`push`, which first queues pending native work).
    pub(crate) fn n3_push(
        &mut self,
        d: &Arc<Draw>,
        p: &Prim,
        prim: u32,
        verts: &[Vertex],
        s: NativeSurfaces,
        vram: &[u32],
    ) -> bool {
        match self.n3_accept(d, p, prim, verts, s, vram) {
            Ok(()) => true,
            Err(r) => {
                self.n3.stats.fallback[r as usize] += 1;
                false
            }
        }
    }

    fn n3_accept(
        &mut self,
        d: &Arc<Draw>,
        p: &Prim,
        prim: u32,
        verts: &[Vertex],
        mut s: NativeSurfaces,
        vram: &[u32],
    ) -> Result<(), Fallback> {
        if !matches!(prim, 3..=5) || !matches!(p.shape, Shape::Tri(_)) {
            return Err(Fallback::NotTri);
        }
        let st = match &self.n3.last {
            Some((ld, r)) if Arc::ptr_eq(ld, d) => *r,
            _ => {
                if self.n3.gpu.is_none() {
                    self.n3.gpu = Some(Box::new(Gpu::new(&self.ctx.device)));
                }
                let dual = self.n3.gpu.as_ref().is_some_and(|g| g.dual);
                let r = classify(d, dual);
                self.n3.last = Some((d.clone(), r));
                r
            }
        }?;
        // Source alpha range of this triangle (after the texture function).
        let mut mode = 0u8;
        if st.dst_as || st.fb_only {
            let (tlo, thi) = match &self.n3.last_tex {
                Some((ld, b)) if Arc::ptr_eq(ld, d) => *b,
                _ => {
                    let (b, n) = tex_alpha_range(d, vram, &self.gpu_dirty);
                    self.n3.stats.scanned_texels += n;
                    self.n3.last_tex = Some((d.clone(), b));
                    b
                }
            };
            let vs = if d.iip { &verts[..3] } else { &verts[2..3] };
            let vlo = vs.iter().map(|v| v.rgba[3] as u32).min().unwrap_or(0);
            let vhi = vs.iter().map(|v| v.rgba[3] as u32).max().unwrap_or(0);
            let (alo, ahi) = match (&d.tex, d.tcc) {
                (Some(_), true) => match d.tfx {
                    0 => (((tlo * vlo) >> 7).min(255), ((thi * vhi) >> 7).min(255)),
                    1 | 3 => (tlo, thi),
                    _ => ((tlo + vlo).min(255), (thi + vhi).min(255)),
                },
                _ => (vlo, vhi),
            };
            if st.dst_as && ahi > 0x80 {
                return Err(Fallback::AlphaOver);
            }
            if st.fb_only {
                // An FB_ONLY test that cannot fail is no test; one that
                // cannot pass draws colour only; only mixed outcomes need
                // the two draws.
                mode = match alpha_test_range(d.atst, d.aref as u32, alo, ahi) {
                    Some(true) => 0,
                    Some(false) => 1,
                    None => 2,
                };
                self.n3.stats.fb_only_modes[mode as usize] += 1;
            }
        }
        if d.z_used && s.z.is_none() {
            return Err(Fallback::Z32);
        }
        if d.tex_self {
            return Err(Fallback::TexFeedback);
        }
        // Pixels past the buffer width alias other rows.
        if p.x1 >= d.fb_width || (d.z_used && p.x1 >= d.zb_width) {
            return Err(Fallback::Surface);
        }
        // Join the pending run, or start a new one.
        let joins = self.n3.surf.is_some_and(|r| r.frame == s.frame && (s.z.is_none() || s.z == r.z));
        if joins {
            let r = self.n3.surf.expect("run");
            s.z = r.z;
            s.height = s.height.max(r.height);
            s.width = r.width;
            if d.tex_pages.hits(&self.n3.target_pages) {
                return Err(Fallback::TexFeedback);
            }
            if s.height != r.height {
                let pages = self.check_native(&s).map_err(|_| Fallback::Surface)?;
                self.n3.target_pages = pages;
                self.n3.surf = Some(s);
            }
        } else {
            let pages = self.check_native(&s).map_err(|_| Fallback::Surface)?;
            if d.tex_pages.hits(&pages) {
                return Err(Fallback::TexFeedback);
            }
            self.n3_flush_why(vram, 3);
            self.n3.surf = Some(s);
            self.n3.target_pages = pages;
            self.n3.tex_pages = Pages::default();
        }
        self.n3.tex_pages.union(&d.tex_pages);
        let mut v = [Vertex::default(); 3];
        v.copy_from_slice(&verts[..3]);
        let Shape::Tri(t) = &p.shape else { unreachable!() };
        use super::super::raster::Z;
        let cz = t.c[Z] + t.dady[Z] * (t.by0 as f64 - t.oy);
        let base = cz.floor().clamp(0.0, u32::MAX as f64);
        let zp = [
            (p.x0.max(0) as u32) | (p.y0.max(0) as u32) << 16,
            base as u32,
            ((cz - base) as f32).to_bits(),
            (t.dadx[Z] as f32).to_bits(),
            (t.dady[Z] as f32).to_bits(),
        ];
        let brk = if mode == 2 {
            let gk = (st.key, st.fix, [d.sx0, d.sy0, d.sx1, d.sy1]);
            if self.n3.cov_key != Some(gk) {
                self.n3.cov.clear();
                self.n3.cov_key = Some(gk);
            }
            self.n3.cov.add(t, p)
        } else {
            self.n3.cov_key = None;
            false
        };
        let r = &mut self.n3.rect;
        if self.n3.tris.is_empty() {
            *r = [p.x0, p.y0, p.x1, p.y1];
        } else {
            *r = [r[0].min(p.x0), r[1].min(p.y0), r[2].max(p.x1), r[3].max(p.y1)];
        }
        let planes = planes_knob().then(|| {
            use super::super::raster::{A, B, F, G, Q, R, S, T};
            let mut w = [0u32; 25];
            w[0] = zp[0];
            let dy = t.by0 as f64 - t.oy;
            for (i, &k) in [R, G, B, A, S, T, Q, F].iter().enumerate() {
                w[1 + i] = ((t.c[k] + t.dady[k] * dy) as f32).to_bits();
                w[9 + i] = (t.dadx[k] as f32).to_bits();
                w[17 + i] = (t.dady[k] as f32).to_bits();
            }
            Box::new(w)
        });
        self.n3.tris.push(Tri { d: d.clone(), v, linear: p.linear, st, zp, planes, brk, mode });
        Ok(())
    }

    /// Queue the pending run if `pages` hits what it draws or samples.
    pub(crate) fn n3_touch(&mut self, pages: &Pages, vram: &[u32]) {
        if self.n3.pending() && (pages.hits(&self.n3.target_pages) || pages.hits(&self.n3.tex_pages)) {
            self.n3_flush_why(vram, 2);
        }
    }

    /// Whether the pending run draws into `pages`.
    pub(crate) fn n3_writes(&self, pages: &Pages) -> bool {
        self.n3.pending() && pages.hits(&self.n3.target_pages)
    }

    /// Queue the pending triangles as one native run at this point of the
    /// command stream.
    pub(crate) fn n3_flush(&mut self, vram: &[u32]) {
        self.n3_flush_why(vram, 1);
    }

    /// `n3_flush`, counting why (`N3Stats::flush_why`).
    pub(crate) fn n3_flush_why(&mut self, vram: &[u32], why: usize) {
        if !self.n3.pending() {
            return;
        }
        self.n3.stats.flush_why[why] += 1;
        let tris = std::mem::take(&mut self.n3.tris);
        self.n3.cov.clear();
        self.n3.cov_key = None;
        let mut s = self.n3.surf.take().expect("run surfaces");
        let [rx0, ry0, rx1, ry1] = self.n3.rect;
        s.width = s.width.min(rx1.max(0) as u32 + 1);
        s.height = s.height.min(ry1.max(0) as u32 + 1);
        let (rx0, ry0) = (rx0.max(0) as u32, ry0.max(0) as u32);
        self.n3.target_pages = Pages::default();
        self.n3.tex_pages = Pages::default();
        if self.check_native(&s).is_err() {
            // Draw them emulated (checked when accepted, so rare).
            self.n3.stats.refused_runs += 1;
            for t in &tris {
                self.n3.stats.fallback[Fallback::Surface as usize] += 1;
                if let Some(p) = super::super::raster::setup(&t.d, 0, 3, &t.v) {
                    self.push(&t.d, &p, 3, &t.v, vram);
                }
            }
            return;
        }
        let Ok(run) = self.begin_native_run_at(&s, rx0, ry0, vram) else { unreachable!("checked") };
        let rid = run.run;
        let g: &mut GpuGs = &mut *run.gs;
        let (vw, vh) = g.native_viewport();
        let base = g.arena.len() as u32;
        let mut setups: Vec<(*const Draw, u32)> = Vec::new();
        let mut calls: Vec<Call> = Vec::new();
        let mut group: Option<(Call, usize)> = None;
        let dev = &g.ctx.device;
        let n3gpu = g.n3.gpu.as_mut().expect("n3 gpu");
        // Setup records go after the vertex records (`setup` appends).
        let mut recs: Vec<[u32; VWORDS]> = Vec::with_capacity(tris.len() * 3);
        let mut tri_setup: Vec<usize> = Vec::with_capacity(tris.len());
        let mut pending_setups: Vec<Arc<Draw>> = Vec::new();
        for t in &tris {
            let ptr = Arc::as_ptr(&t.d);
            let si = match setups.iter().rposition(|(p, _)| *p == ptr) {
                Some(i) => i,
                None => {
                    setups.push((ptr, 0));
                    pending_setups.push(t.d.clone());
                    setups.len() - 1
                }
            };
            tri_setup.push(si);
            let d = &t.d;
            let pipe = n3gpu.pipeline(dev, t.st.key);
            let scissor = [
                d.sx0.max(0) as u32,
                d.sy0.max(0) as u32,
                (d.sx1.max(0) as u32).min(s.width - 1),
                (d.sy1.max(0) as u32).min(s.height - 1),
            ];
            let first = (recs.len()) as u32;
            if t.mode == 2 {
                // Pixels passing the alpha test (colour and Z), then the
                // failing ones (colour only, depth-tested): exact as long as
                // the triangles of a group cover disjoint pixels, so a group
                // only takes triangles with disjoint bounding boxes.
                let pipe_b = n3gpu.pipeline(dev, t.st.key & !K_ZWRITE);
                let fits = !t.brk
                    && group.as_ref().is_some_and(|gr| gr.0.pipe == pipe && gr.0.scissor == scissor && gr.0.fix == t.st.fix);
                if fits {
                    group.as_mut().expect("group").0.count += 3;
                } else {
                    close_group(&mut group, &mut calls);
                    group = Some((Call { pipe, scissor, fix: t.st.fix, first, count: 3, inst: 0 }, pipe_b));
                }
            } else {
                close_group(&mut group, &mut calls);
                let (pipe, inst) = if t.mode == 1 { (n3gpu.pipeline(dev, t.st.key & !K_ZWRITE), 1) } else { (pipe, 0) };
                match calls.last_mut() {
                    Some(c) if c.inst == inst && c.pipe == pipe && c.scissor == scissor && c.fix == t.st.fix && c.first + c.count == first => c.count += 3,
                    _ => calls.push(Call { pipe, scissor, fix: t.st.fix, first, count: 3, inst }),
                }
            }
            let z0 = t.v[0].z;
            let flat = if d.iip { None } else { Some(t.v[2].rgba) };
            for v in &t.v {
                let mut w = [0u32; VWORDS];
                w[0] = (((v.x as i32 - d.ofx) as f64 / 16.0 + 0.5) as f32).to_bits();
                w[1] = (((v.y as i32 - d.ofy) as f64 / 16.0 + 0.5) as f32).to_bits();
                w[2] = u32::from_le_bytes(flat.unwrap_or(v.rgba));
                let (ss, tt, q) = if d.fst { (v.u as f32, v.v as f32, 1.0) } else { (v.s, v.t, v.q) };
                w[3] = ss.to_bits();
                w[4] = tt.to_bits();
                w[5] = q.to_bits();
                w[6] = (v.f as f32).to_bits();
                w[7] = ((v.z as i64 - z0 as i64) as f32).to_bits();
                w[8] = z0;
                let op = (t.st.key >> K_OP) & 3;
                w[10] = t.linear as u32
                    | (flat.is_some() as u32) << 1
                    | ((op != 0) as u32) << 2
                    | ((op == 3) as u32) << 3;
                w[11..16].copy_from_slice(&t.zp);
                w[16] = u32::MAX;
                recs.push(w);
            }
        }
        close_group(&mut group, &mut calls);
        g.arena.reserve(recs.len() * VWORDS);
        for r in &recs {
            g.arena.extend_from_slice(r);
        }
        for (i, t) in tris.iter().enumerate() {
            if let Some(pl) = &t.planes {
                let at = g.arena.len() as u32;
                g.arena.extend_from_slice(&pl[..]);
                for k in 0..3 {
                    g.arena[base as usize + (i * 3 + k) * VWORDS + 16] = at;
                }
            }
        }
        let mut offs = Vec::with_capacity(pending_setups.len());
        for d in &pending_setups {
            offs.push(g.setup(d));
        }
        // Patch the setup offsets into the vertex records.
        for (i, &si) in tri_setup.iter().enumerate() {
            for k in 0..3 {
                g.arena[base as usize + (i * 3 + k) * VWORDS + 9] = offs[si];
            }
        }
        let slot = g.slots.len() as u32;
        let mut prm = [0u32; 16];
        prm[0] = base;
        prm[8] = vw;
        prm[9] = vh;
        prm[12] = eps_knob().to_bits();
        g.slots.push(prm);
        g.n3.stats.tris += tris.len() as u64;
        g.n3.stats.runs += 1;
        g.n3.stats.draws += calls.len() as u64;
        g.n3.lists.push(List { run: rid, slot, vw, vh, calls });
        run.end();
    }

    /// The bind group of the native-3 pipelines for this submission.
    pub(super) fn n3_bind_group(&self) -> Option<wgpu::BindGroup> {
        if self.n3.lists.is_empty() {
            return None;
        }
        let g = self.n3.gpu.as_ref()?;
        fn b(i: u32, buf: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
            wgpu::BindGroupEntry { binding: i, resource: buf.as_entire_binding() }
        }
        Some(self.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gs-n3-bg"),
            layout: &g.bgl,
            entries: &[
                b(0, &self.vram),
                b(1, &self.tables),
                b(2, &self.data),
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.params,
                        offset: 0,
                        size: wgpu::BufferSize::new(64),
                    }),
                },
                b(4, &self.snap),
                b(6, &self.dec),
            ],
        }))
    }

    /// The draw list of native run `run`, if it has one.
    pub(super) fn n3_list_of(&self, run: u32) -> Option<u32> {
        // Lists are queued in run order; few per submission.
        self.n3.lists.iter().rposition(|l| l.run == run).map(|i| i as u32)
    }

    /// Encode a run's draw list into `pass` (the run's unswizzle pass, so
    /// a run costs one render pass and one compute pass).
    pub(super) fn n3_encode(&self, bg: &wgpu::BindGroup, list: u32, pass: &mut wgpu::RenderPass<'_>) {
        let l = &self.n3.lists[list as usize];
        let g = self.n3.gpu.as_ref().expect("n3 gpu");
        pass.set_viewport(0.0, 0.0, l.vw as f32, l.vh as f32, 0.0, 1.0);
        pass.set_bind_group(0, bg, &[l.slot * SLOT as u32]);
        let mut cur = usize::MAX;
        let mut fix = u32::MAX;
        let mut sc = [u32::MAX; 4];
        for c in &l.calls {
            let [x0, y0, x1, y1] = c.scissor;
            if x0 > x1 || y0 > y1 {
                continue;
            }
            if c.pipe != cur {
                pass.set_pipeline(&g.pipelines[c.pipe]);
                cur = c.pipe;
            }
            if c.scissor != sc {
                pass.set_scissor_rect(x0, y0, x1 - x0 + 1, y1 - y0 + 1);
                sc = c.scissor;
            }
            if c.fix != fix {
                let k = c.fix as f64 / 128.0;
                pass.set_blend_constant(wgpu::Color { r: k, g: k, b: k, a: k });
                fix = c.fix;
            }
            pass.draw(c.first..c.first + c.count, c.inst..c.inst + 1);
        }
    }
}
