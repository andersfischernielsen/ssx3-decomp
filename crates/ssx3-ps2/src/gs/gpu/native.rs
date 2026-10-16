//! Native runs: host wgpu rendering at an exact point of the GS command
//! stream (phase 3 composition, `docs/investigation/phase3-render.md` §4.1).
//!
//! While any draw of a frame is still emulated, GS VRAM stays the master
//! copy of the frame and Z buffers. A *native run* borrows them for a while:
//!
//! 1. the open raster batch is closed (everything the GS was given before
//!    the run is ordered before it);
//! 2. **unswizzle**: one fullscreen render pass reads the frame buffer
//!    (CT32/CT24/CT16/CT16S, in the native swizzled layout) into an
//!    `Rgba8Unorm` texture and the Z buffer (Z24/Z16/Z16S) into a
//!    `Depth32Float` texture holding `Zgs / 2^24`;
//! 3. the run's recorded passes ([`NativeRecord`]) draw into those
//!    textures with ordinary wgpu render passes;
//! 4. **swizzle**: a compute pass writes both textures back into VRAM, and
//!    the pages are marked GPU-dirty so CPU readbacks see the result.
//!
//! All of it is recorded into the GS's own command stream (`Cmd::Native*`)
//! and encoded in order with the raster batches at the next submission, so
//! no extra submission or wait is involved.
//!
//! **Exactness of the round trip** (checked by the unit tests and by the
//! full race replay with `SSX3_RENDER_ROUNDTRIP`):
//! - Colour: an 8-bit channel `c` is written as `c / 255.0` to a unorm8
//!   target, which stores the nearest code (`c`); it reads back as the
//!   nearest f32 to `c / 255`, and `round(v * 255)` is `c` again. CT24 reads
//!   as alpha 0x80 and writes back only the low 24 bits; CT16 goes through
//!   the GS's own 5:5:5:1 expansion (`c << 3`, A = 0x80 or 0), whose inverse
//!   (`>> 3`) is exact.
//! - Z: every integer below 2^24 is exact in f32 (24-bit significand), and
//!   dividing by 2^24 only changes the exponent, so `Zgs / 2^24` is the
//!   exact depth value in [0, 1 - 2^-24], stored unchanged by
//!   `Depth32Float` (fragment depth is only clamped to the viewport's
//!   [0, 1]). `depth * 2^24` converts back exactly. The same holds for Z16.
//!   Z32 is refused: values of 2^24 and above would lose low bits.
//!   Depth comparisons between such values (`Greater`, `GreaterEqual`) are
//!   therefore the GS's integer comparisons.
//! - Pixels a run leaves alone go back with their original bits, so a run
//!   need not track coverage.

use super::super::raster::Pages;
use super::super::swizzle::{self, Surface};
use super::{surf_words, Cmd, GpuContext, GpuGs, SLOT};

/// Submit before queueing a run past this many in one submission.
const MAX_RUNS_PER_SUBMIT: usize = 32;

/// Colour target format of a run.
pub const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
/// Depth target format of a run; it holds `Zgs / DEPTH_SCALE`.
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// GS Z units per 1.0 of depth (`Zgs = depth * DEPTH_SCALE`).
pub const DEPTH_SCALE: f32 = 16_777_216.0;

/// A buffer in GS local memory, as FRAME/ZBUF/BITBLTBUF name it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeSurface {
    /// Base pointer in 64-word blocks (FBP * 32, ZBP * 32).
    pub bp: u32,
    /// Width in 64-pixel units (FBW; ZBUF uses the frame's).
    pub bw: u32,
    /// Storage format (PSMCT32, PSMZ24, ...).
    pub psm: u32,
}

/// What a native run borrows: the frame and/or Z buffer, over the
/// rectangle `(0, 0)..(width, height)` of their pixel coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeSurfaces {
    pub frame: Option<NativeSurface>,
    pub z: Option<NativeSurface>,
    pub width: u32,
    pub height: u32,
}

impl NativeSurfaces {
    /// The surfaces of a drawing context: FRAME_n, ZBUF_n, and SCISSOR_n
    /// for the extent (the whole lines of the buffer width down to the
    /// scissor's last row). Z is left out when ZBUF is Z32 (not exact).
    pub fn from_regs(frame: u64, zbuf: u64, scissor: u64) -> NativeSurfaces {
        let fbw = ((frame >> 16) & 0x3F) as u32;
        let fb = NativeSurface {
            bp: (frame & 0x1FF) as u32 * 32,
            bw: fbw,
            psm: ((frame >> 24) & 0x3F) as u32,
        };
        let zb = NativeSurface {
            bp: (zbuf & 0x1FF) as u32 * 32,
            bw: fbw,
            psm: 0x30 | ((zbuf >> 24) & 0xF) as u32,
        };
        let scy1 = ((scissor >> 48) & 0x7FF) as u32;
        NativeSurfaces {
            frame: Some(fb),
            z: (zb.psm != swizzle::PSMZ32).then_some(zb),
            width: fbw * 64,
            height: scy1 + 1,
        }
    }
}

/// Why a native run was refused (nothing was queued).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeError {
    /// The CPU rasterizer is in use: there is no GPU VRAM to compose with.
    NoGpu,
    /// Neither a frame nor a Z buffer, or an empty rectangle.
    Empty,
    /// Wider or taller than 2048 pixels, or wider than the buffer width.
    TooLarge,
    /// A frame format the run cannot represent (T8/T4 and friends).
    FrameFormat(u32),
    /// A Z format the run cannot represent exactly (Z32).
    ZFormat(u32),
    /// The frame and Z rectangles share VRAM pages (draws would race).
    Aliased,
    /// A rectangle reaches past the end of VRAM (wraps around).
    Wraps,
}

/// What recorded passes draw into.
pub struct NativeTargets<'a> {
    pub ctx: &'static GpuContext,
    /// `COLOR_FORMAT` view of at least `width` x `height` texels; texel
    /// (x, y) is frame pixel (x, y). The alpha channel holds the GS's 8 bits
    /// (0x80 = 1.0 for blending).
    pub color: &'a wgpu::TextureView,
    /// `DEPTH_FORMAT` view, `Zgs / DEPTH_SCALE`; larger is nearer.
    pub depth: &'a wgpu::TextureView,
    pub width: u32,
    pub height: u32,
    pub surfaces: NativeSurfaces,
}

/// A pass (or several) recorded into a run. It runs on the GS thread when
/// the GS submits, after the unswizzle and before the swizzle, and must
/// only render into `targets` (with viewport/scissor inside
/// `width` x `height`). Buffers it reads must not be rewritten with
/// `queue.write_buffer` by a later run of the same submission (write
/// buffers are applied before the whole submission); give each run its
/// own buffers or copy with the encoder.
pub type NativeRecord = Box<dyn FnOnce(&NativeTargets<'_>, &mut wgpu::CommandEncoder) + Send>;

struct Run {
    surfaces: NativeSurfaces,
    /// Top-left corner of the round-tripped rectangle (`begin_native_run_at`).
    x0: u32,
    y0: u32,
    /// The targets already hold these surfaces (`Held`): no unswizzle.
    reuse: bool,
    /// Unswizzle the whole target within the buffer width
    /// (`full_w` x `full_h`), not just the run's rectangle: the run continues
    /// on buffers the targets held, so later runs can reuse.
    full: bool,
    full_w: u32,
    full_h: u32,
    /// The N× pass of this run (`hires`).
    hi: Option<HiRun>,
}

#[derive(Clone, Copy)]
struct HiRun {
    frame: Option<super::hires::Key>,
    z: Option<super::hires::Key>,
    refresh_frame: bool,
    refresh_z: bool,
    /// Unswizzle parameter slot with the scale.
    slot: u32,
}

/// What the targets hold after the last run: its surfaces, the rectangle in
/// which they equal VRAM (`x0, y0, x1, y1` exclusive), and the write clock
/// at that point. A later run over the same surfaces inside that rectangle
/// can skip the unswizzle as long as nothing else wrote their pages since
/// (`note_vram_write`). VRAM is still written back at every run's end.
#[derive(Clone, Copy)]
struct Held {
    surfaces: NativeSurfaces,
    valid: [u32; 4],
    clock: u64,
}

/// Pipelines, targets and the queued runs of the open submission.
pub(super) struct NativeState {
    bgl0: wgpu::BindGroupLayout,
    bgl1: wgpu::BindGroupLayout,
    unswizzle_pipe: wgpu::RenderPipeline,
    unswizzle_c: wgpu::RenderPipeline,
    unswizzle_z: wgpu::RenderPipeline,
    swizzle_pipe: wgpu::ComputePipeline,
    /// N× copies (`SSX3_RENDER_SCALE`).
    pub(super) hires: Option<super::hires::Hires>,
    targets: Option<Targets>,
    runs: Vec<Run>,
    records: Vec<Option<NativeRecord>>,
    held: Option<Held>,
    /// Per VRAM page: write clock of the last write that did not come from
    /// a native run (uploads, compute raster batches, GPU transfers).
    page_clock: Box<[u64; 512]>,
    clock: u64,
    /// Runs that skipped the unswizzle.
    pub(super) reused: u64,
    /// Why a run could not reuse: nothing held, other surfaces, outside
    /// the valid rectangle, pages written since.
    pub(super) miss: [u64; 4],
}

struct Targets {
    w: u32,
    h: u32,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// Copy of `color` at the last segment break of a run (native-3
    /// destination reads, `native3.wgsl` `n3_fs_dr`).
    prev: wgpu::TextureView,
}

fn layout_entry(binding: u32, ty: wgpu::BindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
        ty,
        count: None,
    }
}

fn storage(read_only: bool) -> wgpu::BindingType {
    wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage { read_only },
        has_dynamic_offset: false,
        min_binding_size: None,
    }
}

impl NativeState {
    /// `scale` > 1 also keeps N× copies (`hires`).
    pub(super) fn new(dev: &wgpu::Device, module: &wgpu::ShaderModule, scale: u32) -> NativeState {
        // Group 0: the GS bindings the native entry points use (vram,
        // tables, params), visible to fragment and compute stages.
        let bgl0 = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gs-native-bgl0"),
            entries: &[
                layout_entry(0, storage(false)),
                layout_entry(1, storage(true)),
                layout_entry(
                    3,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                ),
            ],
        });
        // Group 1: the run's targets, read by the swizzle pass.
        let bgl1 = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gs-native-bgl1"),
            entries: &[
                layout_entry(
                    0,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
                layout_entry(
                    1,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
            ],
        });
        let l0 = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gs-native-unswizzle"),
            bind_group_layouts: &[Some(&bgl0)],
            immediate_size: 0,
        });
        let l1 = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gs-native-swizzle"),
            bind_group_layouts: &[Some(&bgl0), Some(&bgl1)],
            immediate_size: 0,
        });
        let unswizzle = |label: &'static str, colour: wgpu::ColorWrites, depth: bool| {
            dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&l0),
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some("native_vs"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: Some(depth),
                    depth_compare: Some(wgpu::CompareFunction::Always),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some("native_unswizzle"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: COLOR_FORMAT,
                        blend: None,
                        write_mask: colour,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        let unswizzle_pipe = unswizzle("gs-native-unswizzle", wgpu::ColorWrites::ALL, true);
        // N× refreshes of one buffer of a pair (`hires`).
        let unswizzle_c = unswizzle("gs-hires-unswizzle-colour", wgpu::ColorWrites::ALL, false);
        let unswizzle_z = unswizzle("gs-hires-unswizzle-depth", wgpu::ColorWrites::empty(), true);
        let swizzle_pipe = dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("gs-native-swizzle"),
            layout: Some(&l1),
            module,
            entry_point: Some("native_swizzle"),
            compilation_options: Default::default(),
            cache: None,
        });
        NativeState {
            bgl0,
            bgl1,
            unswizzle_pipe,
            unswizzle_c,
            unswizzle_z,
            hires: (scale > 1).then(|| super::hires::Hires::new(scale)),
            swizzle_pipe,
            targets: None,
            runs: Vec::new(),
            records: Vec::new(),
            held: None,
            page_clock: Box::new([0; 512]),
            clock: 1,
            reused: 0,
            miss: [0; 4],
        }
    }

    /// VRAM `pages` changed outside native runs.
    pub(super) fn note_vram_write(&mut self, pages: &Pages) {
        if let Some(h) = self.hires.as_mut() {
            h.note_write(pages);
        }
        self.clock += 1;
        let c = self.clock;
        for (i, w) in pages.0.iter().enumerate() {
            let mut w = *w;
            while w != 0 {
                self.page_clock[i * 64 + w.trailing_zeros() as usize] = c;
                w &= w - 1;
            }
        }
    }

    /// Whether a run over `s` / `pages` from `(x0, y0)` can use what the
    /// targets hold.
    fn can_reuse(&mut self, s: &NativeSurfaces, pages: &Pages, x0: u32, y0: u32) -> bool {
        let Some(h) = self.held else {
            self.miss[0] += 1;
            return false;
        };
        if !compatible(&h.surfaces, s) {
            self.miss[1] += 1;
            return false;
        }
        if x0 < h.valid[0] || y0 < h.valid[1] || s.width > h.valid[2] || s.height > h.valid[3] {
            self.miss[2] += 1;
            return false;
        }
        let mut ok = true;
        for (i, w) in pages.0.iter().enumerate() {
            let mut w = *w;
            while w != 0 {
                ok &= self.page_clock[i * 64 + w.trailing_zeros() as usize] <= h.clock;
                w &= w - 1;
            }
        }
        if !ok {
            self.miss[3] += 1;
        }
        ok
    }

    /// Grow the targets to at least `w` x `h` (runs queued earlier in the
    /// submission are encoded with the grown ones).
    fn ensure_targets(&mut self, dev: &wgpu::Device, w: u32, h: u32) {
        if let Some(t) = &self.targets {
            if t.w >= w && t.h >= h {
                return;
            }
        }
        let (w, h) = match &self.targets {
            Some(t) => (t.w.max(w), t.h.max(h)),
            None => (w.max(640), h.max(512)),
        };
        // New textures: nothing is held any more.
        self.held = None;
        // Powers of two: native-3 maps window pixels to NDC exactly over
        // the whole target.
        let (w, h) = (w.next_power_of_two(), h.next_power_of_two());
        let rt = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC;
        let mk = |label, format, usage| {
            dev.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
            .create_view(&Default::default())
        };
        self.targets = Some(Targets {
            w,
            h,
            color: mk("gs-native-color", COLOR_FORMAT, rt),
            depth: mk("gs-native-depth", DEPTH_FORMAT, rt),
            prev: mk(
                "gs-native-prev",
                COLOR_FORMAT,
                wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            ),
        });
    }

    pub(super) fn has_work(&self) -> bool {
        !self.runs.is_empty()
    }
}

/// Whether targets holding `held` serve a run over `s`: the same frame
/// buffer (and the same Z buffer when `s` uses one); the extent (from the
/// scissor) is checked separately.
fn compatible(held: &NativeSurfaces, s: &NativeSurfaces) -> bool {
    held.frame == s.frame && (s.z.is_none() || held.z == s.z)
}

/// `SSX3_NATIVE_NO_REUSE=1`: always unswizzle (A/B check of the reuse).
fn no_reuse() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var_os("SSX3_NATIVE_NO_REUSE").is_some())
}

/// `FbFmt` code of a frame format (gs.wgsl `read_fb`/`write_fb`), as
/// `Draw::build` maps it; `None` for formats a frame cannot have.
fn frame_fmt(psm: u32) -> Option<u32> {
    match psm {
        swizzle::PSMCT32 | swizzle::PSMZ32 => Some(0),
        swizzle::PSMCT24 | swizzle::PSMZ24 => Some(1),
        swizzle::PSMCT16 | swizzle::PSMCT16S | swizzle::PSMZ16 | swizzle::PSMZ16S => Some(2),
        _ => None,
    }
}

/// The pages of `s` over `w` x `h`, or `Err(Wraps)` past the end of VRAM.
fn surface_pages(s: &Surface, w: u32, h: u32) -> Result<Pages, NativeError> {
    let rows = (h - 1) >> s.ph_shift;
    let cols = (w - 1) >> s.pw_shift;
    let last = (s.base >> 11) + rows * s.pages_per_row + cols + (s.base & 2047 != 0) as u32;
    if last >= 512 {
        return Err(NativeError::Wraps);
    }
    Ok(Pages::rect(s, 0, 0, w - 1, h - 1))
}

/// Statistics of native runs (`GpuStats`).
#[derive(Default, Clone, Copy, Debug)]
pub struct NativeStats {
    pub runs: u64,
    /// Pixels round-tripped (width x height per run).
    pub pixels: u64,
    pub refused: u64,
    /// Runs that reused the held targets (no unswizzle).
    pub reused: u64,
    pub miss: [u64; 4],
    /// GPU time of the unswizzle and swizzle passes (`SSX3_GS_PROFILE`).
    pub gpu_ns: u64,
}

/// A native run being recorded ([`GpuGs::begin_native_run`]). Passes
/// recorded with [`NativeRun::record`] run in order between the unswizzle
/// and the swizzle; [`NativeRun::end`] (or dropping the run) queues the
/// swizzle.
pub(crate) struct NativeRun<'a> {
    pub(super) gs: &'a mut GpuGs,
    pub(super) run: u32,
    ended: bool,
}

impl NativeRun<'_> {
    /// Queue `rec` into the run.
    pub(crate) fn record(&mut self, rec: NativeRecord) {
        let n = self.gs.native.as_mut().expect("native state");
        let idx = n.records.len() as u32;
        n.records.push(Some(rec));
        self.gs.cmds.push(Cmd::NativePass { run: self.run, rec: idx });
    }

    /// Queue the swizzle back into VRAM.
    pub(crate) fn end(mut self) {
        self.finish();
    }

    fn finish(&mut self) {
        if self.ended {
            return;
        }
        self.ended = true;
        let slot = self.gs.native_slot(self.run);
        self.gs.cmds.push(Cmd::NativeEnd { run: self.run, slot });
    }
}

impl Drop for NativeRun<'_> {
    fn drop(&mut self) {
        self.finish();
    }
}

impl GpuGs {
    /// Start a native run over `s` at the current point of the command
    /// stream: closes the open batch (uploading CPU-written pages first) and
    /// queues the unswizzle. The surfaces' pages count as GPU-drawn from
    /// here on; textures decoded from them are dropped.
    pub(crate) fn begin_native_run(&mut self, s: &NativeSurfaces, vram: &[u32]) -> Result<NativeRun<'_>, NativeError> {
        self.begin_native_run_at(s, 0, 0, vram)
    }

    /// `begin_native_run` over the rectangle `(x0, y0)..(width, height)`
    /// only: pixels outside it are neither read nor written back, so passes
    /// must not draw there.
    pub(crate) fn begin_native_run_at(
        &mut self,
        s: &NativeSurfaces,
        x0: u32,
        y0: u32,
        vram: &[u32],
    ) -> Result<NativeRun<'_>, NativeError> {
        let r = self.check_native(s);
        let pages = match r {
            Ok(p) => p,
            Err(e) => {
                self.stats.native.refused += 1;
                return Err(e);
            }
        };
        self.close(vram);
        // Bound the passes per command buffer (thousands of passes in one
        // submission have lost the device): submit what is queued, without
        // waiting (the batch was just closed).
        if self.native.is_none() {
            self.native = Some(Box::new(NativeState::new(&self.ctx.device, &self.module, self.hires_scale)));
        }
        if self.native.as_ref().is_some_and(|n| n.runs.len() >= MAX_RUNS_PER_SUBMIT) {
            self.submit_with(|_, _| {});
        }
        // CPU-written pages must be on the GPU before the unswizzle reads
        // them (`close` uploads them only when it has primitives).
        self.emit_uploads(vram);
        let n = self.native.as_mut().expect("native state");
        n.ensure_targets(&self.ctx.device, s.width, s.height);
        let (x0, y0) = (x0.min(s.width - 1), y0.min(s.height - 1));
        let reuse = !no_reuse() && n.can_reuse(s, &pages, x0, y0);
        let full = !reuse && !no_reuse() && n.held.is_some_and(|h| compatible(&h.surfaces, s) || compatible(s, &h.surfaces));
        // A full unswizzle covers the whole target within the buffer width,
        // so runs with other scissors on the same buffers reuse it.
        let (tw, th) = n.targets.as_ref().map_or((s.width, s.height), |t| (t.w, t.h));
        let buf_w = s.frame.or(s.z).map_or(s.width, |f| (f.bw * 64).min(tw)).max(s.width);
        let (fw, fh) = (buf_w, th.max(s.height));
        let held_surfaces = match (reuse, &n.held) {
            // Keep the Z buffer the targets still hold for a run without Z.
            (true, Some(h)) => NativeSurfaces { z: s.z.or(h.surfaces.z), ..*s },
            _ => *s,
        };
        let valid = match (reuse, &n.held) {
            (true, Some(h)) => h.valid,
            _ if full => [0, 0, fw, fh],
            _ => [x0, y0, s.width, s.height],
        };
        n.held = Some(Held { surfaces: held_surfaces, valid, clock: n.clock });
        n.reused += reuse as u64;
        self.stats.native.reused += reuse as u64;
        self.stats.native.miss = n.miss;
        let run = n.runs.len() as u32;
        // N× copies: refresh what changed in VRAM, then this run writes them.
        let mut hi = None;
        let dev = &self.ctx.device;
        if let (Some(h), Some(t)) = (n.hires.as_mut(), n.targets.as_ref()) {
            h.ensure_size(dev, t.w, t.h);
            let (tw, th) = (t.w, t.h);
            let mut acquire = |b: &NativeSurface, depth: bool| {
                let key = super::hires::Key { bp: b.bp, bw: b.bw, psm: b.psm, depth };
                let surf = Surface::new(b.psm, b.bp, b.bw);
                let pg = surface_pages(&surf, (b.bw * 64).min(tw).max(1), th)
                    .or_else(|_| surface_pages(&surf, s.width, s.height))
                    .unwrap_or(pages);
                (key, h.acquire(dev, key, &pg))
            };
            let f = s.frame.as_ref().map(|b| acquire(b, false));
            let z = s.z.as_ref().map(|b| acquire(b, true));
            h.note_write(&pages);
            for k in f.iter().chain(z.iter()) {
                h.mark_current(k.0);
            }
            hi = Some(HiRun {
                frame: f.map(|x| x.0),
                z: z.map(|x| x.0),
                refresh_frame: f.is_some_and(|x| x.1),
                refresh_z: z.is_some_and(|x| x.1),
                slot: 0,
            });
        }
        n.runs.push(Run { surfaces: *s, x0, y0, reuse, full, full_w: fw, full_h: fh, hi });
        let slot = self.native_slot(run);
        if self.native.as_ref().is_some_and(|n| n.runs[run as usize].hi.is_some()) {
            // The same parameters with the scale in d.y.
            let mut p = self.slots[slot as usize];
            p[13] = self.hires_scale;
            let hs = self.slots.len() as u32;
            self.slots.push(p);
            if let Some(h) = self.native.as_mut().and_then(|n| n.runs[run as usize].hi.as_mut()) {
                h.slot = hs;
            }
        }
        self.cmds.push(Cmd::NativeBegin { run, slot });
        self.gpu_dirty.union(&pages);
        self.decoded.retain(|_, (_, pg)| !pg.hits(&pages));
        self.n3.alpha_cache.bump(&pages);
        self.stats.native.runs += 1;
        self.stats.native.pixels += s.width as u64 * s.height as u64;
        Ok(NativeRun {
            gs: self,
            run,
            ended: false,
        })
    }

    /// The destination-read copy of the colour target (`Targets::prev`).
    pub(super) fn native_prev(&self) -> Option<&wgpu::TextureView> {
        self.native.as_ref().and_then(|n| n.targets.as_ref()).map(|t| &t.prev)
    }

    /// The size of the run targets (powers of two); valid after
    /// `begin_native_run`.
    pub(super) fn native_viewport(&self) -> (u32, u32) {
        let t = self.native.as_ref().and_then(|n| n.targets.as_ref()).expect("native targets");
        (t.w, t.h)
    }

    /// A whole run: unswizzle, `passes` in order, swizzle.
    pub(crate) fn native_run(&mut self, s: &NativeSurfaces, vram: &[u32], passes: Vec<NativeRecord>) -> Result<(), NativeError> {
        let mut run = self.begin_native_run(s, vram)?;
        for p in passes {
            run.record(p);
        }
        run.end();
        Ok(())
    }

    /// Validate `s`; returns the pages it covers.
    pub(super) fn check_native(&self, s: &NativeSurfaces) -> Result<Pages, NativeError> {
        if (s.frame.is_none() && s.z.is_none()) || s.width == 0 || s.height == 0 {
            return Err(NativeError::Empty);
        }
        if s.width > 2048 || s.height > 2048 {
            return Err(NativeError::TooLarge);
        }
        let mut all = Pages::default();
        let mut fb_pages = Pages::default();
        if let Some(f) = &s.frame {
            if frame_fmt(f.psm).is_none() {
                return Err(NativeError::FrameFormat(f.psm));
            }
            if s.width > f.bw * 64 {
                return Err(NativeError::TooLarge);
            }
            fb_pages = surface_pages(&Surface::new(f.psm, f.bp, f.bw), s.width, s.height)?;
            all.union(&fb_pages);
        }
        if let Some(z) = &s.z {
            if !matches!(z.psm, swizzle::PSMZ24 | swizzle::PSMZ16 | swizzle::PSMZ16S) {
                return Err(NativeError::ZFormat(z.psm));
            }
            if s.width > z.bw * 64 {
                return Err(NativeError::TooLarge);
            }
            let zp = surface_pages(&Surface::new(z.psm, z.bp, z.bw), s.width, s.height)?;
            if zp.hits(&fb_pages) {
                return Err(NativeError::Aliased);
            }
            all.union(&zp);
        }
        Ok(all)
    }

    /// Parameter slot of run `run`: a = frame (surface words, format |
    /// 0x100 when present), b = Z (surface words, largest Z or 0), c = (w, h).
    fn native_slot(&mut self, run: u32) -> u32 {
        let r = &self.native.as_ref().expect("native state").runs[run as usize];
        let (s, x0, y0) = (r.surfaces, r.x0, r.y0);
        let mut p = [0u32; 16];
        if let Some(f) = &s.frame {
            let sw = surf_words(&Surface::new(f.psm, f.bp, f.bw));
            p[..3].copy_from_slice(&sw);
            p[3] = frame_fmt(f.psm).unwrap_or(0) | 0x100;
        }
        if let Some(z) = &s.z {
            let sw = surf_words(&Surface::new(z.psm, z.bp, z.bw));
            p[4..7].copy_from_slice(&sw);
            p[7] = if z.psm == swizzle::PSMZ24 { 0xFF_FFFF } else { 0xFFFF };
        }
        p[8] = s.width;
        p[9] = s.height;
        p[10] = x0;
        p[11] = y0;
        let slot = self.slots.len() as u32;
        self.slots.push(p);
        slot
    }
}

/// Encoding of the native commands of one submission.
pub(super) struct NativeEncode {
    bg0: wgpu::BindGroup,
    bg1: wgpu::BindGroup,
    bg3: Option<wgpu::BindGroup>,
    /// `bg3` with the N× destination-read copy (`hires`).
    bg3h: Option<wgpu::BindGroup>,
    records: Vec<Option<NativeRecord>>,
}

impl GpuGs {
    /// Bind groups and records for this submission's native commands
    /// (`None` without native work).
    pub(super) fn native_encode_start(&mut self) -> Option<NativeEncode> {
        let n = self.native.as_deref_mut().filter(|n| n.has_work())?;
        let dev = &self.ctx.device;
        let t = n.targets.as_ref().expect("native targets");
        let bg0 = dev.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gs-native-bg0"),
            layout: &n.bgl0,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.vram.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.tables.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.params,
                        offset: 0,
                        size: wgpu::BufferSize::new(64),
                    }),
                },
            ],
        });
        let bg1 = dev.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gs-native-bg1"),
            layout: &n.bgl1,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&t.color),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&t.depth),
                },
            ],
        });
        let bg3 = self.n3_bind_group();
        let bg3h = self
            .native
            .as_deref()
            .and_then(|n| n.hires.as_ref())
            .and_then(|h| h.prev.as_ref().zip(h.dummy_color.as_ref()))
            .and_then(|(p, d)| self.n3_bind_group_with(p, d));
        let n = self.native.as_deref_mut().expect("native state");
        Some(NativeEncode {
            bg0,
            bg1,
            bg3,
            bg3h,
            records: std::mem::take(&mut n.records),
        })
    }

    /// Encode one native command (`Cmd::Native*`).
    pub(super) fn native_encode(
        &self,
        e: &mut NativeEncode,
        c: &Cmd,
        enc: &mut wgpu::CommandEncoder,
        ts_render: Option<wgpu::RenderPassTimestampWrites<'_>>,
        ts_compute: Option<wgpu::ComputePassTimestampWrites<'_>>,
    ) {
        let n = self.native.as_deref().expect("native state");
        let t = n.targets.as_ref().expect("native targets");
        match *c {
            Cmd::NativeBegin { run, slot } => {
                let r = &n.runs[run as usize];
                let s = &r.surfaces;
                let list = self.n3_list_of(run);
                let (load_c, load_d) = if r.reuse {
                    (wgpu::LoadOp::Load, wgpu::LoadOp::Load)
                } else {
                    (wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), wgpu::LoadOp::Clear(0.0))
                };
                let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("gs-native-unswizzle"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &t.color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: load_c,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &t.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: load_d,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: ts_render,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_viewport(0.0, 0.0, s.width as f32, s.height as f32, 0.0, 1.0);
                if r.full {
                    pass.set_viewport(0.0, 0.0, r.full_w as f32, r.full_h as f32, 0.0, 1.0);
                    pass.set_scissor_rect(0, 0, r.full_w, r.full_h);
                } else {
                    pass.set_scissor_rect(r.x0, r.y0, s.width - r.x0, s.height - r.y0);
                }
                if !r.reuse {
                    pass.set_pipeline(&n.unswizzle_pipe);
                    pass.set_bind_group(0, &e.bg0, &[slot * SLOT as u32]);
                    pass.draw(0..3, 0..1);
                }
                // Native-3 triangles draw in the same pass; destination-read
                // segments after the first follow a copy of the colour
                // target into `prev`, each in a pass of its own.
                if let Some(list) = list {
                    let bg = e.bg3.as_ref().expect("n3 bind group");
                    self.n3_encode(bg, list, 0, &mut pass);
                    drop(pass);
                    for seg in 1..self.n3_segments(list) {
                        let [x0, y0, x1, y1] = self.n3_copy_rect(list, seg);
                        enc.copy_texture_to_texture(
                            wgpu::TexelCopyTextureInfo {
                                texture: t.color.texture(),
                                mip_level: 0,
                                origin: wgpu::Origin3d { x: x0, y: y0, z: 0 },
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::TexelCopyTextureInfo {
                                texture: t.prev.texture(),
                                mip_level: 0,
                                origin: wgpu::Origin3d { x: x0, y: y0, z: 0 },
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::Extent3d { width: x1 - x0 + 1, height: y1 - y0 + 1, depth_or_array_layers: 1 },
                        );
                        let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("gs-native-n3"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &t.color,
                                depth_slice: None,
                                resolve_target: None,
                                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                            })],
                            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                view: &t.depth,
                                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }),
                                stencil_ops: None,
                            }),
                            timestamp_writes: None,
                            occlusion_query_set: None,
                            multiview_mask: None,
                        });
                        self.n3_encode(bg, list, seg, &mut pass);
                    }
                } else {
                    drop(pass);
                }
                if let (Some(hr), Some(h)) = (r.hi, n.hires.as_ref()) {
                    self.encode_hires(e, n, h, r, &hr, list, enc);
                }
            }
            Cmd::NativePass { run, rec } => {
                let s = n.runs[run as usize].surfaces;
                if let Some(f) = e.records[rec as usize].take() {
                    let targets = NativeTargets {
                        ctx: self.ctx,
                        color: &t.color,
                        depth: &t.depth,
                        width: s.width,
                        height: s.height,
                        surfaces: s,
                    };
                    f(&targets, enc);
                }
            }
            Cmd::NativeEnd { run, slot } => {
                let r = &n.runs[run as usize];
                let s = &r.surfaces;
                let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("gs-native-swizzle"),
                    timestamp_writes: ts_compute,
                });
                pass.set_pipeline(&n.swizzle_pipe);
                pass.set_bind_group(0, &e.bg0, &[slot * SLOT as u32]);
                pass.set_bind_group(1, &e.bg1, &[]);
                pass.dispatch_workgroups((s.width - r.x0).div_ceil(8), (s.height - r.y0).div_ceil(8), 1);
            }
            _ => unreachable!("not a native command"),
        }
    }

    /// The N× pass of a run: refresh stale copies from VRAM, then the run's
    /// draw list at N×, each segment after a copy into the N× `prev`.
    #[allow(clippy::too_many_arguments)]
    fn encode_hires(
        &self,
        e: &NativeEncode,
        n: &NativeState,
        h: &super::hires::Hires,
        r: &Run,
        hr: &HiRun,
        list: Option<u32>,
        enc: &mut wgpu::CommandEncoder,
    ) {
        let k = h.scale;
        let cview = hr.frame.and_then(|key| h.surfs.get(&key)).map(|x| &x.view).or(h.dummy_color.as_ref()).expect("hires colour");
        let dview = hr.z.and_then(|key| h.surfs.get(&key)).map(|x| &x.view).or(h.dummy_depth.as_ref()).expect("hires depth");
        let begin = |enc: &mut wgpu::CommandEncoder| -> wgpu::RenderPass<'static> {
            enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gs-hires"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: cview,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: dview,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            })
            .forget_lifetime()
        };
        if hr.refresh_frame || hr.refresh_z {
            let mut pass = begin(enc);
            pass.set_viewport(0.0, 0.0, h.w as f32, h.h as f32, 0.0, 1.0);
            pass.set_scissor_rect(0, 0, h.w, h.h);
            pass.set_bind_group(0, &e.bg0, &[hr.slot * SLOT as u32]);
            if hr.refresh_frame {
                pass.set_pipeline(&n.unswizzle_c);
                pass.draw(0..3, 0..1);
            }
            if hr.refresh_z {
                pass.set_pipeline(&n.unswizzle_z);
                pass.draw(0..3, 0..1);
            }
        }
        let (Some(list), Some(bg0h), Some(prev)) = (list, e.bg3h.as_ref(), h.prev.as_ref()) else { return };
        // A list sampling another buffer's N× copy gets its own bind group.
        let own = self.n3.lists[list as usize]
            .src
            .and_then(|k| h.surfs.get(&k))
            .and_then(|sv| self.n3_bind_group_with(prev, &sv.view));
        let bg = own.as_ref().unwrap_or(bg0h);
        let s = &r.surfaces;
        for seg in 0..self.n3_segments(list) {
            let [x0, y0, x1, y1] = if seg == 0 { [r.x0, r.y0, s.width - 1, s.height - 1] } else { self.n3_copy_rect(list, seg) };
            let (cx, cy) = ((x0 * k).min(h.w - 1), (y0 * k).min(h.h - 1));
            let cw = ((x1 - x0 + 1) * k).min(h.w - cx);
            let ch = ((y1 - y0 + 1) * k).min(h.h - cy);
            enc.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo { texture: cview.texture(), mip_level: 0, origin: wgpu::Origin3d { x: cx, y: cy, z: 0 }, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyTextureInfo { texture: prev.texture(), mip_level: 0, origin: wgpu::Origin3d { x: cx, y: cy, z: 0 }, aspect: wgpu::TextureAspect::All },
                wgpu::Extent3d { width: cw, height: ch, depth_or_array_layers: 1 },
            );
            let mut pass = begin(enc);
            self.n3_encode_at(bg, list, seg, &mut pass, k);
        }
    }

    /// Forget this submission's runs (after encoding).
    pub(super) fn native_encode_done(&mut self) {
        if let Some(n) = self.native.as_mut() {
            n.runs.clear();
            n.records.clear();
        }
        self.n3.lists.clear();
    }

    /// Debug/test: read the run targets back (`width` x `height` RGBA8
    /// words and depth values). Waits for the GPU.
    #[cfg(test)]
    pub(crate) fn native_read_targets(&mut self, w: u32, h: u32) -> (Vec<u32>, Vec<f32>) {
        let t = self.native.as_ref().and_then(|n| n.targets.as_ref()).expect("native targets");
        let tex_c = t.color.texture().clone();
        let tex_d = t.depth.texture().clone();
        // Depth copies must cover the whole texture.
        let (tw, th) = (t.w, t.h);
        let row = (tw * 4).div_ceil(256) * 256;
        let bytes = row as u64 * th as u64;
        let dev = &self.ctx.device;
        let buf = dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("native-read"),
            size: bytes * 2,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut enc = dev.create_command_encoder(&Default::default());
        for (i, (tex, aspect)) in [(tex_c, wgpu::TextureAspect::All), (tex_d, wgpu::TextureAspect::DepthOnly)]
            .into_iter()
            .enumerate()
        {
            enc.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buf,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: i as u64 * bytes,
                        bytes_per_row: Some(row),
                        rows_per_image: Some(th),
                    },
                },
                wgpu::Extent3d {
                    width: tw,
                    height: th,
                    depth_or_array_layers: 1,
                },
            );
        }
        self.ctx.queue.submit(std::iter::once(enc.finish()));
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = dev.poll(wgpu::PollType::wait_indefinitely());
        let v = slice.get_mapped_range().expect("map");
        let word = |o: usize| u32::from_le_bytes([v[o], v[o + 1], v[o + 2], v[o + 3]]);
        let mut c = Vec::with_capacity((w * h) as usize);
        let mut d = Vec::with_capacity((w * h) as usize);
        for y in 0..h as usize {
            for x in 0..w as usize {
                c.push(word(y * row as usize + x * 4));
                d.push(f32::from_bits(word(bytes as usize + y * row as usize + x * 4)));
            }
        }
        (c, d)
    }
}

impl GpuGs {
    /// `SSX3_RENDER_SCALE` diagnostics: read back every N× colour copy as
    /// (label, width, height, RGBA words). Waits for the GPU.
    pub(crate) fn hires_snapshots(&mut self) -> Vec<(String, u32, u32, Vec<u32>)> {
        let Some(h) = self.native.as_ref().and_then(|n| n.hires.as_ref()) else { return Vec::new() };
        let (w, hh) = (h.w, h.h);
        let row = (w * 4).div_ceil(256) * 256;
        let bytes = row as u64 * hh as u64;
        let dev = &self.ctx.device;
        let mut out = Vec::new();
        for (key, s) in h.surfs.iter().filter(|(k, _)| !k.depth) {
            let buf = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("hires-read"),
                size: bytes,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let mut enc = dev.create_command_encoder(&Default::default());
            enc.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo { texture: s.view.texture(), mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buf,
                    layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(hh) },
                },
                wgpu::Extent3d { width: w, height: hh, depth_or_array_layers: 1 },
            );
            self.ctx.queue.submit(std::iter::once(enc.finish()));
            let slice = buf.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            let _ = dev.poll(wgpu::PollType::wait_indefinitely());
            let v = slice.get_mapped_range().expect("map");
            // Only the buffer's width (FBW × 64 × N).
            let bw = (key.bw * 64 * h.scale).min(w);
            let mut px = Vec::with_capacity((bw * hh) as usize);
            for y in 0..hh as usize {
                for x in 0..bw as usize {
                    let o = y * row as usize + x * 4;
                    px.push(u32::from_le_bytes([v[o], v[o + 1], v[o + 2], v[o + 3]]));
                }
            }
            out.push((format!("{:05x}-{}-{:02x}", key.bp, key.bw, key.psm), bw, hh, px));
        }
        out
    }
}
