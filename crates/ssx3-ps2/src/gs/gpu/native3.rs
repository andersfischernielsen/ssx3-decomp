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
    /// A point (lines, sprites and triangles are drawn natively).
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
    /// The texture overlaps the frame/Z the run draws (no longer a
    /// fallback: such primitives read VRAM, ending the run first when they
    /// sample what it drew).
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
    /// readback, ...), a transfer touching the run, other surfaces, a
    /// primitive sampling what the run drew.
    pub flush_why: [u64; 5],
    /// Of `tris`: sprites (two triangles each), lines, destination-read
    /// primitives.
    pub sprites: u64,
    pub lines: u64,
    /// Of `tris`: primitives sampling the run's own surfaces (from VRAM,
    /// as they were before the run), and runs ended for one.
    pub feedback: u64,
    pub feedback_flushes: u64,
    pub dr_prims: u64,
    /// Destination-read segment breaks (render pass + copy of the target).
    pub dr_copies: u64,
    /// Alpha-range cache hits.
    pub alpha_cache_hits: u64,
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
            "\n    FB_ONLY tris: always pass {}, always fail {}, mixed {}; texels scanned {} (alpha cache hits {})",
            self.fb_only_modes[0], self.fb_only_modes[1], self.fb_only_modes[2], self.scanned_texels, self.alpha_cache_hits
        );
        s += &format!(
            "\n    sprites {}, lines {}, destination-read prims {}, segment copies {}, texture feedback prims {}",
            self.sprites, self.lines, self.dr_prims, self.dr_copies, self.feedback
        );
        s += &format!(
            "\n    runs ended by: emulated prim {}, close {}, transfer {}, surfaces {}, texture feedback {}",
            self.flush_why[0], self.flush_why[1], self.flush_why[2], self.flush_why[3], self.flush_why[4]
        );
        s += &format!(
            "\n    native runs (all) {}, reused targets {} (misses {:?}), refused {}, {:.1} Mpx round-tripped",
            native.runs,
            native.reused,
            native.miss,
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
/// Destination read (`n3_fs_dr`): the shader computes the GS blend from the
/// destination pixel; no fixed-function blending.
const K_DR: u32 = 1 << 18;

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

/// The fixed-function state of `d`, or why the native path cannot draw it.
/// States the blend unit cannot express exactly read the destination in
/// the shader instead (`K_DR`, see `dr_state`).
fn classify(d: &Draw, dual_ok: bool, exact: bool) -> Result<State, Fallback> {
    if d.ate && d.afail > 1 {
        return Err(Fallback::AlphaFail);
    }
    if d.scanmsk >= 2 {
        return Err(Fallback::FrameFormat);
    }
    let fb_only = d.ate && d.afail == 1;
    let mut dr = false;
    let mut mask = 0u32;
    for i in 0..4 {
        match (d.fbmsk >> (8 * i)) & 0xFF {
            0 => mask |= 1 << i,
            0xFF => {}
            _ => dr = true,
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
    // DATE (ignored on CT24 frames, as `GpuGs::setup`), dithering.
    dr |= d.date && d.fb_fmt != FbFmt::C24;
    dr |= d.dither.is_some();
    let blending = d.abe && d.blend != Blend::Src;
    // CT16: the target holds what the GS stores (5:5:5:1); blending
    // results are not, so blended CT16 draws read the destination.
    dr |= blending && (exact || d.pabe || !d.colclamp || d.fb_fmt == FbFmt::C16);
    if dr {
        return Ok(dr_state(key, fb_only));
    }
    let mut fix = 0;
    let mut dst_as = false;
    if blending {
        match fixed_blend(d, dual_ok) {
            Some((k, f, a)) => {
                key |= k;
                fix = f;
                dst_as = a;
            }
            None => return Ok(dr_state(key, fb_only)),
        }
    }
    Ok(State { key, fix, dst_as, fb_only })
}

/// The destination-read form of a state with pipeline key `key` (only its
/// depth bits are kept: the shader merges FBMSK itself).
fn dr_state(key: u32, fb_only: bool) -> State {
    State {
        key: (key & (3 << K_ZCMP | K_ZWRITE)) | 15 << K_MASK | K_DR,
        fix: 0,
        dst_as: false,
        fb_only,
    }
}

/// The fixed-function form of `d`'s blend `(A - B) * C >> 7 + D`: pipeline
/// key bits, FIX, whether the destination factor is As; None if there is
/// none (factors from Ad, 1 + C, ...).
fn fixed_blend(d: &Draw, dual_ok: bool) -> Option<(u32, u32, bool)> {
    // out = (A - B) * C + D, as k0 + kc * C per colour.
    let sel = |x: u32, w: u32| (x == w) as i32;
    let cs = (sel(d.bd, 0), sel(d.ba, 0) - sel(d.bb, 0));
    let cd = (sel(d.bd, 1), sel(d.ba, 1) - sel(d.bb, 1));
    let uses_c = cs.1 != 0 || cd.1 != 0;
    let c_as = d.bc == 0;
    if uses_c && d.bc == 1 {
        return None;
    }
    if cd.1 != 0 && c_as && !dual_ok {
        return None;
    }
    let (fc, fic) = if c_as { (F_SRC1A, F_INV_SRC1A) } else { (F_CONST, F_INV_CONST) };
    // The source term is premultiplied (and biased) in the shader and
    // added with factor One; only its sign matters here.
    let src_pos = match cs {
        (1, _) | (0, 1) => true,
        (0, 0) | (0, -1) => false,
        _ => return None,
    };
    let (df, dn) = match cd {
        (0, 0) => (F_ZERO, false),
        (1, 0) => (F_ONE, false),
        (0, 1) => (fc, false),
        (1, -1) => (fic, false),
        (0, -1) => (fc, true),
        _ => return None,
    };
    let op = match (src_pos, dn) {
        (true, false) => 1,
        (true, true) => 2,
        (false, false) => 3,
        (false, true) => return None,
    };
    let mut key = F_ONE << K_SRC | df << K_DST | op << K_OP;
    let mut dst_as = false;
    let mut fix = 0;
    if cd.1 != 0 && c_as {
        key |= K_DUAL;
        dst_as = true;
    }
    if cd.1 != 0 && !c_as {
        fix = d.fix as u32;
    }
    Some((key, fix, dst_as))
}

/// What a primitive covers, for `Coverage`.
#[derive(Clone, Copy)]
enum Geo<'a> {
    Tri(&'a super::super::raster::TriSetup, &'a Prim),
    /// Pixels x0..=x1, y0..=y1 (a sprite).
    Rect([i32; 4]),
}

impl Geo<'_> {
    fn rows(&self) -> (i32, i32) {
        match self {
            Geo::Tri(_, p) => (p.y0, p.y1),
            Geo::Rect(r) => (r[1], r[3]),
        }
    }

    /// The covered columns of row `y` (empty when a > b).
    fn span(&self, y: i32) -> (i64, i64) {
        match self {
            Geo::Tri(t, p) => {
                let j = (y - t.by0) as i64;
                let (mut lo, mut hi) = (0i64, (p.x1 - t.bx0) as i64);
                for k in 0..3 {
                    super::super::raster::edge_span(t.e0[k] + t.edy[k] * j, t.edx[k], &mut lo, &mut hi);
                }
                (t.bx0 as i64 + lo, t.bx0 as i64 + hi)
            }
            Geo::Rect(r) => (r[0] as i64, r[2] as i64),
        }
    }
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

    fn init(&mut self) {
        if self.rows.is_empty() {
            self.rows = vec![[0; 32]; 2048];
            self.y0 = i32::MAX;
            self.y1 = i32::MIN;
        }
    }

    /// Whether `g` covers a pixel of the bitmap.
    fn hits(&mut self, g: Geo) -> bool {
        self.init();
        let (y0, y1) = g.rows();
        for y in y0.max(self.y0)..=y1.min(self.y1) {
            let (a, b) = g.span(y);
            if a > b || a < 0 || b >= 2048 {
                continue;
            }
            let row = &self.rows[y as usize];
            if (a as usize / 64..=b as usize / 64).any(|w| row[w] & span_mask(a, b, w) != 0) {
                return true;
            }
        }
        false
    }

    /// Add the pixels of `g`.
    fn mark(&mut self, g: Geo) {
        self.init();
        let (y0, y1) = g.rows();
        for y in y0..=y1 {
            let (a, b) = g.span(y);
            if a > b || a < 0 || b >= 2048 || y < 0 || y >= 2048 {
                continue;
            }
            let row = &mut self.rows[y as usize];
            for w in a as usize / 64..=(b as usize / 64) {
                row[w] |= span_mask(a, b, w);
            }
            self.y0 = self.y0.min(y);
            self.y1 = self.y1.max(y);
        }
    }

    /// Add the pixels of `g`; true (and the bitmap restarts with them) if
    /// one was already covered.
    fn add(&mut self, g: Geo) -> bool {
        let hit = self.hits(g);
        if hit {
            self.clear();
        }
        self.mark(g);
        hit
    }
}

/// Bits of word `w` (pixels 64w..64w+63) inside a..=b.
fn span_mask(a: i64, b: i64, w: usize) -> u64 {
    let (s, e) = ((w * 64) as i64, (w * 64 + 63) as i64);
    let (a, b) = (a.max(s), b.min(e));
    if a > b {
        return 0;
    }
    let n = (b - a + 1) as u32;
    let m = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
    m << (a - s)
}

/// Emit an FB_ONLY group: the passing pixels, then the failing ones.
fn close_group(group: &mut Option<(Call, usize)>, calls: &mut Vec<Call>) {
    if let Some((c, pipe_b)) = group.take() {
        let b = Call { pipe: pipe_b, scissor: c.scissor, fix: c.fix, first: c.first, count: c.count, inst: 1 };
        calls.push(c);
        calls.push(b);
    }
}

/// Texel alpha ranges of CT32 textures (`tex_alpha_range`), keyed by
/// what the scan reads: per level the surface (base, format, width) and the
/// wrap-reachable texel rectangle. Entries carry the pages they read and a
/// stamp; `bump` stamps pages when VRAM changes there (uploads of CPU
/// writes, GPU drawing), which invalidates older entries. Pages the CPU
/// wrote but has not uploaded yet (`cpu_dirty`) are never looked up.
pub(crate) struct AlphaCache {
    map: HashMap<Vec<u32>, ((u32, u32), u32, Pages)>,
    page_gen: Box<[u32; 512]>,
    clock: u32,
    pub(crate) hits: u64,
}

impl Default for AlphaCache {
    fn default() -> Self {
        AlphaCache { map: HashMap::new(), page_gen: Box::new([0; 512]), clock: 1, hits: 0 }
    }
}

fn each_page(p: &Pages, mut f: impl FnMut(usize)) {
    for (i, w) in p.0.iter().enumerate() {
        let mut w = *w;
        while w != 0 {
            f(i * 64 + w.trailing_zeros() as usize);
            w &= w - 1;
        }
    }
}

impl AlphaCache {
    /// VRAM changed in `pages`.
    pub(crate) fn bump(&mut self, pages: &Pages) {
        if self.map.is_empty() {
            return;
        }
        self.clock += 1;
        let (g, c) = (&mut self.page_gen, self.clock);
        each_page(pages, |p| g[p] = c);
    }

    fn get(&mut self, key: &[u32]) -> Option<(u32, u32)> {
        let (r, stamp, pages) = self.map.get(key)?;
        let mut ok = true;
        each_page(pages, |p| ok &= self.page_gen[p] <= *stamp);
        if ok {
            self.hits += 1;
            Some(*r)
        } else {
            None
        }
    }

    fn put(&mut self, key: Vec<u32>, r: (u32, u32), pages: Pages) {
        if self.map.len() >= 8192 {
            self.map.clear();
        }
        self.map.insert(key, (r, self.clock, pages));
    }
}

/// The range of texel alpha `d` can sample (level 0 and mipmaps; filtering
/// stays inside it): from the palette, TEXA, or by reading the texels from
/// the VRAM mirror (0..=255 when the mirror is not current there). Also
/// returns the texels read.
fn tex_alpha_range(d: &Draw, vram: &[u32], gpu_dirty: &Pages, cpu_dirty: &Pages, cache: &mut AlphaCache) -> ((u32, u32), u64) {
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
            let levels = d.tex_levels();
            let mut key = Vec::with_capacity(levels.len() * 7);
            for t in &levels {
                let (u0, u1) = super::super::raster::wrap_range(t.wms, t.tw, t.minu, t.maxu);
                let (v0, v1) = super::super::raster::wrap_range(t.wmt, t.th, t.minv, t.maxv);
                key.extend_from_slice(&[t.surf.base, t.surf.psm, t.surf.pages_per_row, u0, u1, v0, v1]);
            }
            let cacheable = !d.tex_pages.hits(cpu_dirty);
            if cacheable {
                if let Some(r) = cache.get(&key) {
                    return (r, 0);
                }
            }
            let (mut lo, mut hi, mut n) = (255, 0, 0u64);
            'levels: for (t, k) in levels.iter().zip(key.chunks_exact(7)) {
                let (u0, u1, v0, v1) = (k[3], k[4], k[5], k[6]);
                if (u1 - u0 + 1) as u64 * (v1 - v0 + 1) as u64 > 1 << 20 {
                    (lo, hi) = (0, 255);
                    break;
                }
                for y in v0..=v1 {
                    for x in u0..=u1 {
                        let a = t.surface().read(vram, x, y) >> 24;
                        lo = lo.min(a);
                        hi = hi.max(a);
                    }
                    n += (u1 - u0 + 1) as u64;
                    if lo == 0 && hi == 255 {
                        break 'levels;
                    }
                }
            }
            if cacheable {
                cache.put(key, (lo, hi), d.tex_pages);
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

/// `SSX3_N3_DIAG=1`: tally fallbacks by draw state (printed at exit).
pub fn diag_knob() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var_os("SSX3_N3_DIAG").is_some())
}

/// Added to interpolated colour and fog before truncating to integers
/// (`SSX3_N3_EPS`, default 1e-4): a parity knob (see native3.wgsl).
fn eps_knob() -> f32 {
    static E: std::sync::OnceLock<f32> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_N3_EPS").ok().and_then(|v| v.parse().ok()).unwrap_or(1e-4))
}

struct Tri {
    d: Arc<Draw>,
    /// Kicked vertices (3; a sprite uses the first 2) and PRIM type.
    v: [Vertex; 3],
    prim: u32,
    linear: bool,
    st: State,
    /// The compute GS's Z plane (`GpuGs::encode`): origin (x0 | y0 << 16),
    /// integer base, fraction, dZ/dx, dZ/dy.
    zp: [u32; 5],
    /// `SSX3_N3_PLANES`: the compute GS's attribute planes (origin, then
    /// R G B A S T Q F at the origin, d/dx, d/dy).
    planes: Option<Box<[u32; 25]>>,
    /// A sprite: its pixels (x0, y0, x1, y1) and the compute GS's prim
    /// record (kind 1), which the fragment shader evaluates.
    sprite: Option<Box<([i32; 4], [u32; 14])>>,
    /// A line: the quad drawn around it (hardware coordinates, 4 corners)
    /// and the compute GS's prim record (kind 2).
    line: Option<Box<([f32; 8], [u32; 29])>>,
    /// FB_ONLY alpha test: 0 always passes (or no test), 1 always fails
    /// (colour only), 2 mixed (pass and fail draws).
    mode: u8,
    /// FB_ONLY: covers a pixel an earlier triangle of its group covers, so
    /// a new group starts here.
    brk: bool,
    /// A destination-read segment starts here: the colour target is copied
    /// to `prev` over this rectangle (the run's pixels so far) first.
    copy: Option<[i32; 4]>,
    /// Bounding box (Prim x0 y0 x1 y1).
    zp_bbox: [i32; 4],
}

impl Tri {
    /// Pixels the primitive may cover.
    fn bbox(&self) -> [i32; 4] {
        self.zp_bbox
    }

    fn nverts(&self) -> u32 {
        if self.sprite.is_some() || self.line.is_some() {
            6
        } else {
            3
        }
    }
}

/// Most destination-read layers deferred at once.
const MAX_DR_LAYERS: usize = 8;

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
    /// Segments after the first: (first call, rectangle copied to `prev`
    /// before it, x0 y0 x1 y1).
    segs: Vec<(usize, [u32; 4])>,
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
    last: Option<(Arc<Draw>, bool, Result<State, Fallback>)>,
    /// Blended primitives read the destination (the exact GS integer
    /// blend) instead of using the blend unit (PATH2 2D packets).
    pub(crate) exact: bool,
    /// Bounding box of the pending triangles (the run round-trips only it).
    rect: [i32; 4],
    /// Destination reads: pixels drawn since the last copy to `prev` (or
    /// the run's start), the deferred destination-read triangles and their
    /// pixels, whether they need a copy first, and the last copied
    /// rectangle (None: no copy yet, they read VRAM).
    /// The deferred triangles are layered: one goes into the layer after
    /// the last one whose pixels it covers (layers are drawn in order with
    /// a copy before each layer after the first), `cov_d` per layer and
    /// `cov_any` their union. `need_copy` is for layer 0.
    dirty: Coverage,
    deferred: Vec<Vec<Tri>>,
    /// Per layer: other triangles drawn after its destination reads.
    deferred_nd: Vec<Vec<Tri>>,
    cov_d: Vec<Coverage>,
    cov_any: Coverage,
    need_copy: bool,
    last_copy: Option<[i32; 4]>,
    pub(crate) alpha_cache: AlphaCache,
    /// Pixels covered by the open FB_ONLY group, and its state.
    cov: Coverage,
    cov_key: Option<(u32, u32, [i32; 4])>,
    /// Scratch block sets: texels a primitive samples, blocks the run drew.
    tex_region: super::super::region::Region,
    run_region: super::super::region::Region,
    /// Largest texel alpha of the last draw's texture (`tex_alpha_bound`).
    last_tex: Option<(Arc<Draw>, (u32, u32))>,
    gpu: Option<Box<Gpu>>,
    /// `SSX3_N3_DIAG`: fallback tallies by state, and the last alpha bounds.
    pub(crate) diag: HashMap<String, u64>,
    pub(super) lists: Vec<List>,
    pub(crate) stats: N3Stats,
}

impl N3 {
    pub(crate) fn pending(&self) -> bool {
        !self.tris.is_empty() || !self.deferred.is_empty()
    }

    /// Emit the deferred destination-read triangles (after a copy of the
    /// target when one of them covers a pixel drawn since the last copy,
    /// or reaches outside the last copied rectangle).
    fn emit_deferred(&mut self, why: &str) {
        if self.deferred.is_empty() {
            return;
        }
        let bb = self.deferred[0].iter().fold([i32::MAX, i32::MAX, i32::MIN, i32::MIN], |r, t| {
            let [x0, y0, x1, y1] = t.bbox();
            [r[0].min(x0), r[1].min(y0), r[2].max(x1), r[3].max(y1)]
        });
        let outside = self.last_copy.is_some_and(|c| bb[0] < c[0] || bb[1] < c[1] || bb[2] > c[2] || bb[3] > c[3]);
        let layers = std::mem::take(&mut self.deferred);
        let mut nd = std::mem::take(&mut self.deferred_nd);
        let n = layers.len();
        let copy0 = self.need_copy || outside;
        for ((i, mut d), mut e) in layers.into_iter().enumerate().zip(nd.drain(..)) {
            if i > 0 || copy0 {
                if diag_knob() {
                    let t = &d[0].d;
                    let k = format!(
                        "copy {why} layer{} need{} out{} prim{} blend{}{}{}{} fbpsm{:x} n{}",
                        i.min(3), self.need_copy as u8, outside as u8, d[0].prim, t.ba, t.bb, t.bc, t.bd, t.fb.psm,
                        d.len().next_power_of_two()
                    );
                    *self.diag.entry(k).or_default() += 1;
                }
                let r = self.rect;
                d[0].copy = Some(r);
                self.last_copy = Some(r);
            }
            self.tris.append(&mut d);
            self.tris.append(&mut e);
        }
        if n == 1 && !copy0 {
            // The group's pixels join the drawn ones.
            let cd = &self.cov_d[0];
            if cd.y0 <= cd.y1 {
                self.dirty.init();
                for y in cd.y0..=cd.y1 {
                    let (a, b) = (&mut self.dirty.rows, &cd.rows);
                    for w in 0..32 {
                        a[y as usize][w] |= b[y as usize][w];
                    }
                }
                self.dirty.y0 = self.dirty.y0.min(cd.y0);
                self.dirty.y1 = self.dirty.y1.max(cd.y1);
            }
        } else {
            // A copy went before the last layer: only its pixels changed since.
            self.dirty.clear();
            std::mem::swap(&mut self.dirty, &mut self.cov_d[n - 1]);
        }
        for c in &mut self.cov_d[..n] {
            c.clear();
        }
        self.cov_any.clear();
        self.need_copy = false;
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
                entry(
                    7,
                    S::FRAGMENT,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
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
        let op = if key & K_DR != 0 { 0 } else { (key >> K_OP) & 3 };
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
                entry_point: Some(if key & K_DR != 0 {
                    "n3_fs_dr"
                } else if dual {
                    "n3_fs2"
                } else {
                    "n3_fs"
                }),
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
                if diag_knob() {
                    let tk = d.tex.as_ref().map(|t| format!("{:?}/psm{:x}", t.kind, t.surf.psm));
                    let k = format!(
                        "{} prim{} fbpsm{:x} dith{} scan{} abe{} blend{}{}{}{} fix{:x} pabe{} date{} ate{} atst{} afail{} tex{:?} tfx{} tcc{} iip{}",
                        REASONS[r as usize], prim, d.fb.psm, d.dither.is_some() as u8, d.scanmsk, d.abe as u8, d.ba, d.bb, d.bc, d.bd, d.fix,
                        d.pabe as u8, d.date as u8, d.ate as u8, d.atst, d.afail, tk, d.tfx, d.tcc as u8, d.iip as u8
                    );
                    *self.n3.diag.entry(k).or_default() += 1;
                }
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
        // Triangles, sprites (two triangles evaluated with the GS's sprite
        // rules) and lines (a quad around the line, its pixels those the
        // compute GS's line walk hits); points stay emulated.
        let line_box = match &p.shape {
            Shape::Line { x0, y0, x1, y1, .. } => {
                // `GpuGs::encode`: the scissor bounded by the endpoints.
                let bx = [
                    p.x0.max(x0.min(*x1).floor() as i32 - 1),
                    p.y0.max(y0.min(*y1).floor() as i32 - 1),
                    p.x1.min(x0.max(*x1).ceil() as i32 + 1),
                    p.y1.min(y0.max(*y1).ceil() as i32 + 1),
                ];
                if bx[0] > bx[2] || bx[1] > bx[3] || bx[2] < 0 || bx[3] < 0 {
                    // Outside the scissor: draws nothing.
                    return Ok(());
                }
                Some([bx[0].max(0), bx[1].max(0), bx[2], bx[3]])
            }
            _ => None,
        };
        let geo = match (&p.shape, prim) {
            (Shape::Tri(t), 3..=5) => Geo::Tri(t, p),
            (Shape::Sprite { .. }, 6) => Geo::Rect([p.x0, p.y0, p.x1, p.y1]),
            (Shape::Line { .. }, 1 | 2) => Geo::Rect(line_box.expect("line")),
            _ => return Err(Fallback::NotTri),
        };
        let is_sprite = matches!(p.shape, Shape::Sprite { .. });
        let is_line = line_box.is_some();
        // Pixels the primitive may cover.
        let bb = line_box.unwrap_or([p.x0, p.y0, p.x1, p.y1]);
        let mut st = match &self.n3.last {
            Some((ld, x, r)) if Arc::ptr_eq(ld, d) && *x == self.n3.exact => *r,
            _ => {
                if self.n3.gpu.is_none() {
                    self.n3.gpu = Some(Box::new(Gpu::new(&self.ctx.device)));
                }
                let dual = self.n3.gpu.as_ref().is_some_and(|g| g.dual);
                let r = classify(d, dual, self.n3.exact);
                self.n3.last = Some((d.clone(), self.n3.exact, r));
                r
            }
        }?;
        // Source alpha range of this primitive (after the texture function).
        let mut mode = 0u8;
        if st.dst_as || st.fb_only {
            let (tlo, thi) = match &self.n3.last_tex {
                Some((ld, b)) if Arc::ptr_eq(ld, d) => *b,
                _ => {
                    let h0 = self.n3.alpha_cache.hits;
                    let (b, n) = tex_alpha_range(d, vram, &self.gpu_dirty, &self.cpu_dirty, &mut self.n3.alpha_cache);
                    self.n3.stats.scanned_texels += n;
                    self.n3.stats.alpha_cache_hits += self.n3.alpha_cache.hits - h0;
                    self.n3.last_tex = Some((d.clone(), b));
                    b
                }
            };
            // Sprites take their flat attributes from the second vertex.
            let nv = if is_line { 2 } else { 3 };
            let vs = if is_sprite {
                &verts[1..2]
            } else if d.iip {
                &verts[..nv]
            } else {
                &verts[nv - 1..nv]
            };
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
                // The blend unit clamps factors to 1: read the destination.
                st = dr_state(st.key, st.fb_only);
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
        // Pixels past the buffer width alias other rows.
        if bb[2] >= d.fb_width || (d.z_used && bb[2] >= d.zb_width) {
            return Err(Fallback::Surface);
        }
        // Join the pending run, or start a new one.
        let mut flushed = false;
        let mut joins = self.n3.surf.is_some_and(|r| r.frame == s.frame && (s.z.is_none() || s.z == r.z));
        if joins && d.tex.is_some() && d.tex_pages.hits(&self.n3.target_pages) && self.n3_tex_reads_run(d, p, prim, verts) {
            // It samples texels the run has drawn: those are in the run's
            // targets, not in VRAM. End the run (its pixels go back to
            // VRAM) and start another.
            self.n3.stats.feedback_flushes += 1;
            self.n3_flush_why(vram, 4);
            joins = false;
            flushed = true;
        }
        if joins {
            let r = self.n3.surf.expect("run");
            s.z = r.z;
            s.height = s.height.max(r.height);
            s.width = r.width;
            if s.height != r.height {
                let pages = self.check_native(&s).map_err(|_| Fallback::Surface)?;
                self.n3.target_pages = pages;
                self.n3.surf = Some(s);
            }
        } else {
            let pages = self.check_native(&s).map_err(|_| Fallback::Surface)?;
            self.n3_flush_why(vram, 3);
            self.n3.surf = Some(s);
            self.n3.target_pages = pages;
            self.n3.tex_pages = Pages::default();
        }
        self.n3.tex_pages.union(&d.tex_pages);
        let feedback = flushed || (d.tex.is_some() && (d.tex_self || d.tex_pages.hits(&self.n3.target_pages)));
        if feedback {
            // Blended filters over the frame (colour filter, depth fog)
            // take the exact GS blend: the blend unit rounds the
            // destination term differently, off by 1 over whole screens.
            if st.key & K_DR == 0 && (st.key >> K_OP) & 3 != 0 {
                st = dr_state(st.key, st.fb_only);
            }
            // Texture reads come from VRAM, which holds the run's surfaces
            // as they were before the run: what earlier primitives drew
            // (checked above) and nothing of this primitive's own pixels,
            // as the compute GS reads a snapshot taken before its batch.
            self.n3.stats.feedback += 1;
        }
        let mut v = [Vertex::default(); 3];
        let nv = if is_sprite || is_line { 2 } else { 3 };
        v[..nv].copy_from_slice(&verts[..nv]);
        use super::super::raster::Z;
        let mut line = None;
        let (zp, sprite) = match &p.shape {
            Shape::Tri(t) => {
                let cz = t.c[Z] + t.dady[Z] * (t.by0 as f64 - t.oy);
                let base = cz.floor().clamp(0.0, u32::MAX as f64);
                let zp = [
                    (p.x0.max(0) as u32) | (p.y0.max(0) as u32) << 16,
                    base as u32,
                    ((cz - base) as f32).to_bits(),
                    (t.dadx[Z] as f32).to_bits(),
                    (t.dady[Z] as f32).to_bits(),
                ];
                (zp, None)
            }
            Shape::Sprite { x0, y0, a0, at, dsdx, dtdy, base, .. } => {
                use super::super::raster::{S, T, Q};
                use super::{f, z_u32, PK_LINEAR};
                // `GpuGs::encode`, kind 1 (setup offset 0: the shader is
                // given the setup record separately).
                let mut w = [0u32; 14];
                w[1] = 1 | if p.linear { PK_LINEAR } else { 0 };
                w[4] = *x0 as u32;
                w[5] = *y0 as u32;
                w[6] = f(a0[S]);
                w[7] = f(a0[T]);
                w[8] = f(*dsdx);
                w[9] = f(*dtdy);
                w[10] = f(at[Q]);
                w[11] = (base.r as u32 & 0xFF) | (base.g as u32 & 0xFF) << 8 | (base.b as u32 & 0xFF) << 16 | (base.a as u32 & 0xFF) << 24;
                w[12] = base.f as u32;
                w[13] = z_u32(base.z);
                ([0; 5], Some(Box::new(([p.x0, p.y0, p.x1, p.y1], w))))
            }
            Shape::Line { x0, y0, x1, y1, a0, a1 } => {
                use super::super::raster::{A, B, F, G, NATTR, Q, R, S, T};
                use super::{f, z_u32, PK_LINEAR};
                // `GpuGs::encode`, kind 2.
                let mut w = [0u32; 29];
                w[1] = 2 | if p.linear { PK_LINEAR } else { 0 };
                w[4] = f(*x0);
                w[5] = f(*y0);
                w[6] = f(*x1);
                w[7] = f(*y1);
                w[8] = (x1 - x0).abs().max((y1 - y0).abs()).round() as i32 as u32;
                for (i, &k) in [R, G, B, A, S, T, Q, F].iter().enumerate() {
                    w[9 + i] = f(a0[k]);
                    w[17 + i] = f(a1[k]);
                }
                w[27] = z_u32(a0[Z]);
                w[28] = f(a1[Z] - a0[Z]);
                debug_assert_eq!(NATTR, 9);
                // A quad over every pixel within 1.5 of the line (its
                // pixels are within 0.71), in hardware coordinates (+0.5).
                let (x0, y0, x1, y1) = (*x0 + 0.5, *y0 + 0.5, *x1 + 0.5, *y1 + 0.5);
                let (dx, dy) = (x1 - x0, y1 - y0);
                let len = (dx * dx + dy * dy).sqrt();
                let (ux, uy) = if len > 1e-9 { (dx / len * 1.5, dy / len * 1.5) } else { (1.5, 0.0) };
                let (nx, ny) = (-uy, ux);
                let q = [
                    (x0 - ux + nx) as f32,
                    (y0 - uy + ny) as f32,
                    (x1 + ux + nx) as f32,
                    (y1 + uy + ny) as f32,
                    (x1 + ux - nx) as f32,
                    (y1 + uy - ny) as f32,
                    (x0 - ux - nx) as f32,
                    (y0 - uy - ny) as f32,
                ];
                line = Some(Box::new((q, w)));
                ([0; 5], None)
            }
            _ => unreachable!(),
        };
        let fresh = !self.n3.pending();
        let r = &mut self.n3.rect;
        if fresh {
            *r = bb;
        } else {
            *r = [r[0].min(bb[0]), r[1].min(bb[1]), r[2].max(bb[2]), r[3].max(bb[3])];
        }
        // Triangles sampling their own surfaces (screen-space filters over
        // what was drawn) take their attributes from the compute GS's planes,
        // not hardware interpolation, so they sample the same texels.
        let planes = ((planes_knob() || feedback) && !is_sprite && !is_line).then(|| {
            let Shape::Tri(t) = &p.shape else { unreachable!() };
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
        let dr = st.key & K_DR != 0;
        if is_sprite {
            self.n3.stats.sprites += 1;
        }
        if is_line {
            self.n3.stats.lines += 1;
        }
        let mut tri = Tri {
            d: d.clone(),
            v,
            prim,
            linear: p.linear,
            st,
            zp,
            planes,
            sprite,
            line,
            mode,
            brk: false,
            copy: None,
            zp_bbox: bb,
        };
        if dr {
            // Destination reads are deferred while later primitives cover
            // other pixels (they commute), so that few copies of the target
            // are needed: a copy goes before a group of them when one covers
            // a pixel drawn since the last copy. Within a group they cover
            // disjoint pixels; one covering a deferred one goes into the
            // next layer (a group after another copy), not emitting them.
            self.n3.stats.dr_prims += 1;
            let mut layer = 0;
            if self.n3.cov_any.hits(geo) {
                let n = self.n3.deferred.len();
                layer = (0..n).rev().find(|&l| self.n3.cov_d[l].hits(geo)).map_or(0, |l| l + 1);
                if layer >= MAX_DR_LAYERS {
                    self.n3.emit_deferred("dr");
                    layer = 0;
                }
            }
            if layer == 0 && self.n3.dirty.hits(geo) {
                self.n3.need_copy = true;
            }
            if self.n3.cov_d.len() <= layer {
                self.n3.cov_d.push(Coverage::default());
            }
            if self.n3.deferred.len() <= layer {
                self.n3.deferred.push(Vec::new());
                self.n3.deferred_nd.push(Vec::new());
            }
            self.n3.cov_d[layer].mark(geo);
            self.n3.cov_any.mark(geo);
            self.n3.deferred[layer].push(tri);
            return Ok(());
        }
        if !self.n3.deferred.is_empty() && self.n3.cov_any.hits(geo) {
            // It goes after the last layer whose pixels it covers (after
            // that layer's destination reads); later destination reads of
            // its pixels go to the next layer.
            let n = self.n3.deferred.len();
            let layer = (0..n).rev().find(|&l| self.n3.cov_d[l].hits(geo));
            match layer {
                Some(l) if mode != 2 => {
                    self.n3.cov_d[l].mark(geo);
                    self.n3.cov_any.mark(geo);
                    self.n3.deferred_nd[l].push(tri);
                    self.n3.cov_key = None;
                    return Ok(());
                }
                _ => self.n3.emit_deferred("nondr"),
            }
        }
        self.n3.dirty.mark(geo);
        tri.brk = if mode == 2 {
            let gk = (st.key, st.fix, [d.sx0, d.sy0, d.sx1, d.sy1]);
            if self.n3.cov_key != Some(gk) {
                self.n3.cov.clear();
                self.n3.cov_key = Some(gk);
            }
            self.n3.cov.add(geo)
        } else {
            self.n3.cov_key = None;
            false
        };
        self.n3.tris.push(tri);
        Ok(())
    }

    /// Whether the texels `p` can sample include blocks the pending run has
    /// drawn (its bounding box in its frame, and in its Z buffer).
    fn n3_tex_reads_run(&mut self, d: &Draw, p: &Prim, prim: u32, verts: &[Vertex]) -> bool {
        use super::super::swizzle::Surface;
        let (Some(ts), Some(run)) = (d.tex_surface(), self.n3.surf) else { return false };
        if !self.n3.pending() {
            return false;
        }
        let [x0, y0, x1, y1] = self.n3.rect;
        if x1 < 0 || y1 < 0 {
            return false;
        }
        let (x0, y0, x1, y1) = (x0.max(0) as u32, y0.max(0) as u32, x1 as u32, y1 as u32);
        let n3 = &mut self.n3;
        n3.tex_region.clear();
        match super::super::raster::prim_tex_rect(d, p, prim, verts) {
            Some([u0, v0, u1, v1]) => n3.tex_region.add_rect(ts, u0, v0, u1, v1),
            None => n3.tex_region.add_pages(&d.tex_pages),
        }
        n3.run_region.clear();
        for f in [run.frame, run.z].into_iter().flatten() {
            n3.run_region.add_rect(&Surface::new(f.psm, f.bp, f.bw), x0, y0, x1, y1);
        }
        n3.run_region.hits(&n3.tex_region)
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
        self.n3.emit_deferred("flush");
        let tris = std::mem::take(&mut self.n3.tris);
        self.n3.cov.clear();
        self.n3.cov_key = None;
        self.n3.dirty.clear();
        self.n3.last_copy = None;
        self.n3.need_copy = false;
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
                let n = if t.sprite.is_some() || t.line.is_some() { 2 } else { 3 };
                if let Some(p) = super::super::raster::setup(&t.d, 0, t.prim, &t.v[..n]) {
                    self.push(&t.d, &p, t.prim, &t.v[..n], vram);
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
        let mut segs: Vec<(usize, [u32; 4])> = Vec::new();
        let mut group: Option<(Call, usize)> = None;
        let dev = &g.ctx.device;
        let n3gpu = g.n3.gpu.as_mut().expect("n3 gpu");
        // Setup records go after the vertex records (`setup` appends).
        let mut recs: Vec<[u32; VWORDS]> = Vec::with_capacity(tris.len() * 3);
        // Per primitive: setup index, first vertex record, vertex count.
        let mut tri_rec: Vec<(usize, usize, usize)> = Vec::with_capacity(tris.len());
        let mut pending_setups: Vec<Arc<Draw>> = Vec::new();
        let fb_fmt16 = s.frame.is_some_and(|f| matches!(f.psm, 0x02 | 0x0A));
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
            let d = &t.d;
            if let Some(c) = t.copy {
                // A destination-read segment: a new pass after copying the
                // target over the run's pixels so far.
                close_group(&mut group, &mut calls);
                let r = [
                    (c[0].max(0) as u32).max(rx0),
                    (c[1].max(0) as u32).max(ry0),
                    (c[2].max(0) as u32).min(s.width - 1),
                    (c[3].max(0) as u32).min(s.height - 1),
                ];
                segs.push((calls.len(), r));
            }
            let seg_start = segs.last().map_or(0, |x| x.0);
            let pipe = n3gpu.pipeline(dev, t.st.key);
            let scissor = [
                d.sx0.max(0) as u32,
                d.sy0.max(0) as u32,
                (d.sx1.max(0) as u32).min(s.width - 1),
                (d.sy1.max(0) as u32).min(s.height - 1),
            ];
            let first = recs.len() as u32;
            let nv = t.nverts();
            if t.mode == 2 {
                // Pixels passing the alpha test (colour and Z), then the
                // failing ones (colour only, depth-tested): exact as long as
                // the primitives of a group cover disjoint pixels.
                let pipe_b = n3gpu.pipeline(dev, t.st.key & !K_ZWRITE);
                let fits = !t.brk
                    && group.as_ref().is_some_and(|gr| gr.0.pipe == pipe && gr.0.scissor == scissor && gr.0.fix == t.st.fix);
                if fits {
                    group.as_mut().expect("group").0.count += nv;
                } else {
                    close_group(&mut group, &mut calls);
                    group = Some((Call { pipe, scissor, fix: t.st.fix, first, count: nv, inst: 0 }, pipe_b));
                }
            } else {
                close_group(&mut group, &mut calls);
                let (pipe, inst) = if t.mode == 1 { (n3gpu.pipeline(dev, t.st.key & !K_ZWRITE), 1) } else { (pipe, 0) };
                let can_merge = calls.len() > seg_start;
                match calls.last_mut() {
                    Some(c)
                        if can_merge
                            && c.inst == inst
                            && c.pipe == pipe
                            && c.scissor == scissor
                            && c.fix == t.st.fix
                            && c.first + c.count == first =>
                    {
                        c.count += nv
                    }
                    _ => calls.push(Call { pipe, scissor, fix: t.st.fix, first, count: nv, inst }),
                }
            }
            tri_rec.push((si, recs.len(), nv as usize));
            let dr = t.st.key & K_DR != 0;
            let op = if dr { 0 } else { (t.st.key >> K_OP) & 3 };
            let flags = t.linear as u32
                | ((op != 0) as u32) << 2
                | ((op == 3) as u32) << 3
                | ((dr && !segs.is_empty()) as u32) << 5
                | (t.sprite.is_some() as u32) << 6
                | ((fb_fmt16 && !dr && op == 0) as u32) << 7
                | (t.line.is_some() as u32) << 8;
            if let Some(sp) = &t.sprite {
                // Two triangles over the sprite's pixels (edges at pixel
                // boundaries: the hardware samples pixel centres).
                let [x0, y0, x1, y1] = sp.0;
                let (x0, y0, x1, y1) = (x0 as f32, y0 as f32, (x1 + 1) as f32, (y1 + 1) as f32);
                for (x, y) in [(x0, y0), (x1, y0), (x0, y1), (x1, y0), (x1, y1), (x0, y1)] {
                    let mut w = [0u32; VWORDS];
                    w[0] = x.to_bits();
                    w[1] = y.to_bits();
                    w[10] = flags;
                    w[16] = u32::MAX;
                    recs.push(w);
                }
                continue;
            }
            if let Some(ln) = &t.line {
                let q = &ln.0;
                for i in [0, 1, 3, 1, 2, 3] {
                    let mut w = [0u32; VWORDS];
                    w[0] = q[2 * i].to_bits();
                    w[1] = q[2 * i + 1].to_bits();
                    w[10] = flags;
                    w[16] = u32::MAX;
                    recs.push(w);
                }
                continue;
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
                w[10] = flags | (flat.is_some() as u32) << 1;
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
        // Planes (diagnostics) and sprite prim records go after them.
        for (t, &(_, r0, nv)) in tris.iter().zip(&tri_rec) {
            let extra: Option<&[u32]> = match (&t.planes, &t.sprite) {
                _ if t.line.is_some() => Some(&t.line.as_ref().expect("line").1[..]),
                (_, Some(sp)) => Some(&sp.1[..]),
                (Some(pl), None) => Some(&pl[..]),
                _ => None,
            };
            if let Some(words) = extra {
                let at = g.arena.len() as u32;
                g.arena.extend_from_slice(words);
                for k in 0..nv {
                    g.arena[base as usize + (r0 + k) * VWORDS + 16] = at;
                }
            }
        }
        let mut offs = Vec::with_capacity(pending_setups.len());
        for d in &pending_setups {
            offs.push(g.setup(d));
        }
        // Patch the setup offsets into the vertex records.
        for &(si, r0, nv) in &tri_rec {
            for k in 0..nv {
                g.arena[base as usize + (r0 + k) * VWORDS + 9] = offs[si];
            }
        }
        let slot = g.slots.len() as u32;
        let mut prm = [0u32; 16];
        prm[0] = base;
        // b: the frame (destination reads from VRAM, `n3_dst`).
        if let Some(f) = &s.frame {
            let sw = super::surf_words(&super::super::swizzle::Surface::new(f.psm, f.bp, f.bw));
            prm[4..7].copy_from_slice(&sw);
            prm[7] = match f.psm {
                0x01 | 0x31 => 1,
                0x02 | 0x0A | 0x32 | 0x3A => 2,
                _ => 0,
            };
        }
        prm[8] = vw;
        prm[9] = vh;
        prm[12] = eps_knob().to_bits();
        g.slots.push(prm);
        g.n3.stats.tris += tris.len() as u64;
        g.n3.stats.runs += 1;
        g.n3.stats.draws += calls.len() as u64;
        g.n3.stats.dr_copies += segs.len() as u64;
        g.n3.lists.push(List { run: rid, slot, vw, vh, calls, segs });
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
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::TextureView(self.native_prev()?),
                },
            ],
        }))
    }

    /// The draw list of native run `run`, if it has one.
    pub(super) fn n3_list_of(&self, run: u32) -> Option<u32> {
        // Lists are queued in run order; few per submission.
        self.n3.lists.iter().rposition(|l| l.run == run).map(|i| i as u32)
    }

    /// Segments of draw list `list` (1 + its destination-read breaks).
    pub(super) fn n3_segments(&self, list: u32) -> usize {
        1 + self.n3.lists[list as usize].segs.len()
    }

    /// The rectangle copied to `prev` before segment `seg` (>= 1).
    pub(super) fn n3_copy_rect(&self, list: u32, seg: usize) -> [u32; 4] {
        self.n3.lists[list as usize].segs[seg - 1].1
    }

    /// Encode segment `seg` of a run's draw list into `pass` (segment 0 in
    /// the run's unswizzle pass, so a run without destination-read breaks
    /// costs one render pass and one compute pass).
    pub(super) fn n3_encode(&self, bg: &wgpu::BindGroup, list: u32, seg: usize, pass: &mut wgpu::RenderPass<'_>) {
        let l = &self.n3.lists[list as usize];
        let g = self.n3.gpu.as_ref().expect("n3 gpu");
        let c0 = if seg == 0 { 0 } else { l.segs[seg - 1].0 };
        let c1 = l.segs.get(seg).map_or(l.calls.len(), |x| x.0);
        pass.set_viewport(0.0, 0.0, l.vw as f32, l.vh as f32, 0.0, 1.0);
        pass.set_bind_group(0, bg, &[l.slot * SLOT as u32]);
        let mut cur = usize::MAX;
        let mut fix = u32::MAX;
        let mut sc = [u32::MAX; 4];
        for c in &l.calls[c0..c1] {
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
