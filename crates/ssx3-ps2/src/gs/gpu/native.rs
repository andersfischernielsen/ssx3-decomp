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
}

/// Pipelines, targets and the queued runs of the open submission.
pub(super) struct NativeState {
    bgl0: wgpu::BindGroupLayout,
    bgl1: wgpu::BindGroupLayout,
    unswizzle_pipe: wgpu::RenderPipeline,
    swizzle_pipe: wgpu::ComputePipeline,
    targets: Option<Targets>,
    runs: Vec<Run>,
    records: Vec<Option<NativeRecord>>,
}

struct Targets {
    w: u32,
    h: u32,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
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
    pub(super) fn new(dev: &wgpu::Device, module: &wgpu::ShaderModule) -> NativeState {
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
        let unswizzle_pipe = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gs-native-unswizzle"),
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
                depth_write_enabled: Some(true),
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
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
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
            swizzle_pipe,
            targets: None,
            runs: Vec::new(),
            records: Vec::new(),
        }
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
        // Powers of two: native-3 maps window pixels to NDC exactly over
        // the whole target.
        let (w, h) = (w.next_power_of_two(), h.next_power_of_two());
        let mk = |label, format| {
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
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            })
            .create_view(&Default::default())
        };
        self.targets = Some(Targets {
            w,
            h,
            color: mk("gs-native-color", COLOR_FORMAT),
            depth: mk("gs-native-depth", DEPTH_FORMAT),
        });
    }

    pub(super) fn has_work(&self) -> bool {
        !self.runs.is_empty()
    }
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
            self.native = Some(Box::new(NativeState::new(&self.ctx.device, &self.module)));
        }
        if self.native.as_ref().is_some_and(|n| n.runs.len() >= MAX_RUNS_PER_SUBMIT) {
            self.submit_with(|_, _| {});
        }
        // CPU-written pages must be on the GPU before the unswizzle reads
        // them (`close` uploads them only when it has primitives).
        self.emit_uploads(vram);
        let n = self.native.as_mut().expect("native state");
        n.ensure_targets(&self.ctx.device, s.width, s.height);
        let run = n.runs.len() as u32;
        n.runs.push(Run { surfaces: *s, x0: x0.min(s.width - 1), y0: y0.min(s.height - 1) });
        let slot = self.native_slot(run);
        self.cmds.push(Cmd::NativeBegin { run, slot });
        self.gpu_dirty.union(&pages);
        self.decoded.retain(|_, (_, pg)| !pg.hits(&pages));
        self.stats.native.runs += 1;
        self.stats.native.pixels += s.width as u64 * s.height as u64;
        Ok(NativeRun {
            gs: self,
            run,
            ended: false,
        })
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
        let n = self.native.as_deref_mut().expect("native state");
        Some(NativeEncode {
            bg0,
            bg1,
            bg3,
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
                let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("gs-native-unswizzle"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &t.color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &t.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: ts_render,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_viewport(0.0, 0.0, s.width as f32, s.height as f32, 0.0, 1.0);
                pass.set_scissor_rect(r.x0, r.y0, s.width - r.x0, s.height - r.y0);
                pass.set_pipeline(&n.unswizzle_pipe);
                pass.set_bind_group(0, &e.bg0, &[slot * SLOT as u32]);
                pass.draw(0..3, 0..1);
                // Native-3 triangles draw in the same pass.
                if let Some(list) = self.n3_list_of(run) {
                    let bg = e.bg3.as_ref().expect("n3 bind group");
                    self.n3_encode(bg, list, &mut pass);
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
