//! # ssx3-gfx: the host renderer (phase 3, draw layer)
//!
//! Draws the frames the game publishes (`ssx3_render::frame::Frame`) with
//! wgpu, without the PS2 DMA/VIF/VU1/GS models. Plan and progress:
//! `docs/plans/phase3-draw-layer.md`.
//!
//! Per frame ([`Renderer::render`]):
//! 1. The frame's VIF events are replayed in chain order into a VU1 data
//!    memory image kept across frames; each microprogram start runs the
//!    native Rust program of the draw's material (`ssx3_render::run_native`).
//! 2. Their GIF output and the frame's DIRECT streams go through [`gs::Gs`]
//!    (register state, primitive assembly) into triangle batches. Texture
//!    identities come from the draws' TexMan ids, bound to the TBPs their
//!    preludes write.
//! 3. The batches are drawn onto persistent master targets with hardware
//!    depth, sampling and blending ([`draw`]).
//!
//! - [`tex`]: TexMan textures decoded from EE memory.
//! - [`gs`]: GS register state and primitive assembly.
//! - [`draw`]: pipelines for GS draw states.

pub mod debug;
mod draw;
pub mod gs;
pub mod tex;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ssx3_render::chain::Mem;
use ssx3_render::frame::{self, Built, Material, Owner};
use ssx3_render::vif::Event;

/// Statistics of the last frame and totals.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub frames: u64,
    pub vu_calls: u64,
    pub vu_native: u64,
    /// Calls with no native program (MSCNT, unimplemented entries).
    pub vu_missing: u64,
    pub prims: u64,
    pub dropped: u64,
    pub unknown_tex: u64,
    pub batches: u64,
    pub textures: u64,
}

struct Targets {
    w: u32,
    h: u32,
    color: wgpu::Texture,
    depth: wgpu::Texture,
    /// Copy of `color` for primitives that sample the frame.
    snapshot: wgpu::Texture,
    /// Copy of `depth` for the depth-fog pass.
    zsnap: wgpu::Texture,
    /// Copy of `depth` after the 3D phase (light queries).
    qdepth: wgpu::Texture,
}

/// An offscreen render target (`gs::Rt`) on the GPU.
struct RtTarget {
    w: u32,
    h: u32,
    color: wgpu::Texture,
    depth: wgpu::Texture,
}

struct CachedTex {
    view: wgpu::TextureView,
    used: u64,
    /// Largest texel alpha (GS units).
    amax: u8,
}

pub struct Inner {
    dev: wgpu::Device,
    queue: wgpu::Queue,
    pipes: draw::Pipelines,
    targets: Option<Targets>,
    rts: HashMap<u32, RtTarget>,
    gs: gs::Gs,
    vu: Box<[u8; 16384]>,
    texcache: HashMap<tex::TexKey, CachedTex>,
    samplers: HashMap<draw::SampKey, wgpu::Sampler>,
    white: wgpu::TextureView,
    /// TexInfos that failed to decode (reported once).
    failed: std::collections::HashSet<u32>,
    pub stats: Stats,
    /// The display buffer, when the game blends frames into it.
    display_fbp: Option<u32>,
    /// Batches of VIF phase 1 (the 3D scene) in this frame.
    phase1_end: usize,
    /// Keep the depth after phase 1 (`depth()`, native mode).
    pub want_depth: bool,
    last_depth: Option<(u32, u32, Vec<u32>)>,
    /// ZMAX + 1 of the frame's Z buffer (depth is stored as Z / it).
    zscale: f32,
    /// Frames timed and the time (ms) in build, prepare, encode+submit,
    /// depth readback.
    timing: (u64, [f64; 4]),
    /// The image on screen, for presenting (sRGB view allowed).
    present: Option<wgpu::Texture>,
}

/// The renderer; clone the handle to read images from another thread.
#[derive(Clone)]
pub struct Renderer(pub Arc<Mutex<Inner>>);

fn program(m: Material) -> Option<u64> {
    use ssx3_render::*;
    Some(match m {
        Material::Bezier1 => mat1::HASH,
        Material::Bezier2 => mat2::HASH,
        Material::General => mat3::HASH,
        Material::Particle => mat4::HASH,
        Material::Snow => mat5::HASH,
        Material::Skinned | Material::Shadow => mat6::HASH,
        _ => return None,
    })
}

/// Whether texture base `tbp` (64-word blocks) lies in the draw, display or
/// Z buffer (2048-word pages from FBP/ZBP).
fn in_buffers(tbp: u32, d: &frame::DisplayState) -> bool {
    let page = tbp / 32;
    let size = (d.fb_width.div_ceil(64) * d.fb_height.div_ceil(32)).max(1);
    // The display buffer follows the draw buffer (TexMan's VramHeap: draw,
    // display, Z).
    [d.draw_fbp, d.draw_fbp + size, d.zbp].iter().any(|&b| page >= b && page < b + size)
}

impl Renderer {
    pub fn new(dev: wgpu::Device, queue: wgpu::Queue) -> Renderer {
        let pipes = draw::Pipelines::new(&dev);
        let white = dev
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("ssx3-gfx white"),
                size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
            .create_view(&Default::default());
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: white.texture(), mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &[255, 255, 255, 128],
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4), rows_per_image: Some(1) },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        Renderer(Arc::new(Mutex::new(Inner {
            dev,
            queue,
            pipes,
            targets: None,
            rts: HashMap::new(),
            gs: gs::Gs::new(),
            vu: Box::new([0; 16384]),
            texcache: HashMap::new(),
            samplers: HashMap::new(),
            white,
            failed: Default::default(),
            stats: Stats::default(),
            display_fbp: None,
            phase1_end: usize::MAX,
            want_depth: false,
            last_depth: None,
            zscale: 16777216.0,
            timing: (0, [0.0; 4]),
            present: None,
        })))
    }

    /// The master colour target as RGBA8 (GS alpha), read back.
    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)> {
        self.0.lock().unwrap().read_image()
    }

    /// The last frame's image on screen as an sRGB view (same device as
    /// the GS), with its size.
    pub fn present_view(&self) -> Option<(wgpu::TextureView, u32, u32)> {
        let g = self.0.lock().unwrap();
        let t = g.present.as_ref()?;
        let view = t.create_view(&wgpu::TextureViewDescriptor { format: Some(wgpu::TextureFormat::Rgba8UnormSrgb), ..Default::default() });
        Some((view, t.width(), t.height()))
    }

    /// Native mode: keep each frame's depth after the 3D phase for the
    /// light visibility queries.
    pub fn set_want_depth(&self, on: bool) {
        self.0.lock().unwrap().want_depth = on;
    }

    /// The render targets (FBP, width, height, RGBA8), read back (debug).
    pub fn read_rts(&self) -> Vec<(u32, u32, u32, Vec<u32>)> {
        let g = self.0.lock().unwrap();
        g.rts.iter().map(|(&fbp, r)| {
            let (w, h, px) = g.read_texture(&r.color, r.w, r.h);
            (fbp, w, h, px)
        }).collect()
    }

    pub fn stats(&self) -> Stats {
        self.0.lock().unwrap().stats
    }

    /// Primitives drawn into other frame buffers (debug).
    pub fn dropped_fbp(&self) -> Vec<((u32, u32, u32), u64)> {
        let mut v: Vec<_> = self.0.lock().unwrap().gs.dropped_fbp.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort_by_key(|x| std::cmp::Reverse(x.1));
        v
    }
}

impl ssx3_ps2::frame_sink::FrameSink for Renderer {
    fn frame(&mut self, built: &Built, mem: &dyn Mem) {
        self.0.lock().unwrap().render(built, mem);
    }

    fn depth(&mut self) -> Option<(u32, u32, Vec<u32>)> {
        self.0.lock().unwrap().last_depth.take()
    }
}

impl Inner {
    fn ensure_targets(&mut self, w: u32, h: u32) {
        if self.targets.as_ref().is_some_and(|t| t.w == w && t.h == h) {
            return;
        }
        let mk = |label, format, extra| {
            self.dev.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | extra,
                view_formats: &[],
            })
        };
        let color = mk("ssx3-gfx colour", draw::COLOR_FORMAT, wgpu::TextureUsages::COPY_SRC);
        let snapshot = mk("ssx3-gfx snapshot", draw::COLOR_FORMAT, wgpu::TextureUsages::COPY_DST);
        let depth = mk("ssx3-gfx depth", draw::DEPTH_FORMAT, wgpu::TextureUsages::COPY_SRC);
        let zsnap = mk("ssx3-gfx depth snapshot", draw::DEPTH_FORMAT, wgpu::TextureUsages::COPY_DST);
        let qdepth = mk("ssx3-gfx query depth", draw::DEPTH_FORMAT, wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC);
        // Start cleared (the game clears with its own draws afterwards).
        let mut enc = self.dev.create_command_encoder(&Default::default());
        {
            let cv = color.create_view(&Default::default());
            let dv = depth.create_view(&Default::default());
            let _ = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssx3-gfx clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &cv,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &dv,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }
        self.queue.submit(std::iter::once(enc.finish()));
        self.targets = Some(Targets { w, h, color, depth, snapshot, zsnap, qdepth });
    }

    /// Textures for the render targets the game drew into (resized when
    /// they grow).
    fn ensure_rts(&mut self) {
        for (&fbp, rt) in &self.gs.rts {
            if self.rts.get(&fbp).is_some_and(|t| t.w == rt.w && t.h == rt.h) {
                continue;
            }
            let mk = |format, extra| {
                self.dev.create_texture(&wgpu::TextureDescriptor {
                    label: Some("ssx3-gfx rt"),
                    size: wgpu::Extent3d { width: rt.w, height: rt.h, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | extra,
                    view_formats: &[],
                })
            };
            let color = mk(draw::COLOR_FORMAT, wgpu::TextureUsages::COPY_SRC);
            let depth = mk(draw::DEPTH_FORMAT, wgpu::TextureUsages::empty());
            self.rts.insert(fbp, RtTarget { w: rt.w, h: rt.h, color, depth });
        }
    }

    /// Build the geometry of one frame.
    fn build(&mut self, built: &Built, mem: &dyn Mem) {
        let fr = &built.frame;
        let d = &fr.display;
        let (w, h) = (d.fb_width.clamp(64, 2048), d.fb_height.clamp(64, 2048));
        // FRAME.FBP is in 2048-word units, gm's draw_fbp likewise.
        self.gs.begin((w, h), d.draw_fbp, d.zbp);
        self.zscale = match d.zpsm {
            0x30 => 4294967296.0,
            0x31 => 16777216.0,
            _ => 65536.0,
        };
        // Resident textures by their permanent VRAM address.
        let texman = frame::word(mem, fr.gm + frame::gm::TEXMAN);
        self.gs.tex_map.clear();
        for id in 0..tex::tm::MAX as u16 {
            let info = tex::info(mem, texman, id);
            if info != 0 {
                let tbp = frame::word(mem, info + tex::ti::TBP) as i32;
                // Never the frame, display or Z buffers (self-sampling post
                // passes address them; those become native passes).
                if tbp >= 0 && !in_buffers(tbp as u32, d) {
                    self.gs.tex_map.insert(tbp as u32, info);
                }
            }
        }
        let draws: Vec<&frame::Draw> = fr.draws().collect();
        // The PATH3 chains (GifBuff) follow their VIF phases: phase 1
        // (the colour filter and other post passes) between VIF phase 1
        // and 2, phase 2 at the end.
        let path3 = |head: u32| -> Vec<u128> {
            let mut q = Vec::new();
            if head != 0 {
                let mut asr = [0u32; 2];
                ssx3_render::chain::walk(mem, head, false, false, 0, &mut asr, &mut |d: &[u128]| q.extend_from_slice(d));
            }
            q
        };
        let (p1, p2) = (path3(fr.gif_heads[0]), path3(fr.gif_heads[1]));
        let dbg = debug::get();
        let tracing = dbg.trace == Some(self.stats.frames);
        let label = |gs: &mut gs::Gs, l: String| {
            if tracing {
                gs.trace = Some(l);
            }
        };
        // The flip packet (FRAME, ZBUF, XYOFFSET, TEXA, PRMODECONT, ...,
        // and the clear) the game sent before this frame's drawing.
        // With the frame blend on (gm+0x59e8 ≠ 0), the packet starts with a
        // block that blends the finished draw buffer into the display
        // buffer (the image on screen): that part runs at the end.
        let flip = if !dbg.no_flip { flip_packet(mem) } else { Vec::new() };
        let split = flip.iter().position(|&q| q as u64 == 0x1000_0000_0000_8010).unwrap_or(0);
        label(&mut self.gs, "flip".into());
        self.gs.gif(&flip[split..]);
        self.display_fbp = (split > 0).then(|| d.draw_fbp + (d.fb_width.div_ceil(64) * d.fb_height.div_ceil(32)).max(1));
        let mode = dbg.path3;
        let mut p1_done = mode & 1 == 0;
        let mut cur: Option<usize> = None;
        let mut hash: Option<u64> = None;
        // Debug: SSX3_GFX_SKIP=n,m,... drops those draws' payloads.
        // (`a-b` ranges too.)
        let skip = &dbg.skip;
        for (i, (owner, ev)) in built.events.iter().enumerate() {
            if matches!(owner, Owner::Payload(n) if skip.contains(n)) {
                continue;
            }
            if !p1_done && i >= built.phase2 {
                p1_done = true;
                self.phase1_end = self.gs.batches.len();
                label(&mut self.gs, "path3-1".into());
                self.gs.gif(&p1);
            }
            let n = match owner {
                Owner::Prelude(n) | Owner::Payload(n) => Some(*n),
                _ => None,
            };
            if let Some(n) = n {
                if cur != Some(n) {
                    cur = Some(n);
                    if let Some(d) = draws.get(n) {
                        label(&mut self.gs, format!("draw{n} {:?} layer {} tex {:?}", d.state.material, d.state.layer, d.tex));
                    }
                    if let Some(p) = draws.get(n).and_then(|d| program(d.state.material)) {
                        hash = Some(p);
                    }
                }
                // In a draw's payload its textures are bound to the TBPs its
                // prelude wrote.
                if let (Owner::Payload(_), Some(dr)) = (owner, draws.get(n)) {
                    for (ctx, id) in dr.tex.iter().enumerate() {
                        if let Some(id) = id {
                            let info = tex::info(mem, texman, *id);
                            if info != 0 {
                                let tbp = (self.gs.ctx[ctx].tex0 & 0x3fff) as u32;
                                self.gs.tex_map.insert(tbp, info);
                            }
                        }
                    }
                }
            }
            match ev {
                Event::Call(c) => {
                    self.stats.vu_calls += 1;
                    for &(a, v) in &c.input {
                        let a = (a as usize & 1023) * 16;
                        self.vu[a..a + 16].copy_from_slice(&v.to_le_bytes());
                    }
                    let (Some(entry), Some(hh)) = (c.entry, hash) else {
                        self.stats.vu_missing += 1;
                        continue;
                    };
                    let mut out: Vec<u128> = Vec::new();
                    let ok = ssx3_render::run_native(hh, entry, c.top, c.itop, &mut self.vu, &mut |p: &[u128]| out.extend_from_slice(p));
                    if ok {
                        self.stats.vu_native += 1;
                        self.gs.gif(&out);
                    } else {
                        self.stats.vu_missing += 1;
                    }
                }
                Event::Direct(dd) => self.gs.gif(&dd.qwords),
                _ => {}
            }
        }
        if !p1_done {
            self.phase1_end = self.gs.batches.len();
            label(&mut self.gs, "path3-1".into());
            self.gs.gif(&p1);
        }
        if mode & 2 != 0 {
            label(&mut self.gs, "path3-2".into());
            self.gs.gif(&p2);
        }
        if split > 0 {
            label(&mut self.gs, "flip-blend".into());
            self.gs.gif(&flip[..split]);
        }
        if tracing {
            self.gs.trace = None;
            let log = std::mem::take(&mut self.gs.trace_log);
            let path = &dbg.trace_out;
            let _ = std::fs::write(&path, log.join("\n"));
            eprintln!("[gfx] frame {} traced: {} lines -> {path}", self.stats.frames, log.len());
        }
    }

    /// The texture for a TexInfo, and its largest texel alpha.
    fn texture(&mut self, info: u32, texa: u64, mem: &dyn Mem) -> Option<(wgpu::TextureView, u8)> {
        let key = tex::key(mem, info, texa);
        let frame = self.stats.frames;
        if let Some(t) = self.texcache.get_mut(&key) {
            t.used = frame;
            return Some((t.view.clone(), t.amax));
        }
        let Some(d) = tex::decode(mem, info, texa) else {
            if self.failed.insert(info) {
                eprintln!(
                    "[gfx] texture {info:#x} not decoded: format {:#x} pixels {:#x} tex0 {:#x}",
                    frame::word(mem, info + tex::ti::FORMAT),
                    frame::word(mem, info + tex::ti::PIXELS),
                    tex::tex0(mem, info)
                );
            }
            return None;
        };
        let t = self.dev.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssx3-gfx tex"),
            size: wgpu::Extent3d { width: d.width, height: d.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let bytes: Vec<u8> = d.rgba.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &t, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &bytes,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(d.width * 4), rows_per_image: Some(d.height) },
            wgpu::Extent3d { width: d.width, height: d.height, depth_or_array_layers: 1 },
        );
        self.stats.textures += 1;
        // Debug: SSX3_GFX_TEXDUMP=dir writes each decoded texture as raw RGBA.
        if let Some(dir) = &debug::get().texdump {
            let t0 = tex::tex0(mem, info);
            let name = format!(
                "{dir}/tex_{info:08x}_f{:x}_psm{:x}_tcc{}_{}x{}.rgba",
                frame::word(mem, info + tex::ti::FORMAT),
                (t0 >> 20) & 0x3f,
                (t0 >> 34) & 1,
                d.width,
                d.height
            );
            let _ = std::fs::write(name, &bytes);
        }
        let view = t.create_view(&Default::default());
        let amax = d.rgba.iter().map(|p| (p >> 24) as u8).max().unwrap_or(0);
        self.texcache.insert(key, CachedTex { view: view.clone(), used: frame, amax });
        Some((view, amax))
    }

    /// Replace the VU1 data memory image (offline replays start from the
    /// emulated one).
    pub fn seed_vu(&mut self, data: &[u8]) {
        let n = data.len().min(self.vu.len());
        self.vu[..n].copy_from_slice(&data[..n]);
    }

    /// A 256×1 texture from the depth-fog CLUT `i` of this frame (PSMCT32,
    /// CSM1).
    fn clut_texture(&self, i: u32) -> Option<wgpu::TextureView> {
        let up = self.gs.fog_cluts.get(i as usize).filter(|u| !u.data.is_empty())?;
        let mut px = [0u32; 256];
        for (i, p) in px.iter_mut().enumerate() {
            // CSM1: entries 8..15 and 16..23 of each 32 swap places.
            let j = (i & !0x18) | (i & 0x8) << 1 | (i & 0x10) >> 1;
            *p = up.data.get(j).copied().unwrap_or(0);
        }
        let t = self.dev.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssx3-gfx clut"),
            size: wgpu::Extent3d { width: 256, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let bytes: Vec<u8> = px.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &t, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &bytes,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1024), rows_per_image: Some(1) },
            wgpu::Extent3d { width: 256, height: 1, depth_or_array_layers: 1 },
        );
        Some(t.create_view(&Default::default()))
    }

    /// Mean time per frame in each stage of `render` (ms).
    pub fn timing_report(&self) -> String {
        let n = self.timing.0.max(1) as f64;
        let t = &self.timing.1;
        format!(
            "gfx per frame: build {:.2} ms, prepare {:.2} ms, encode+submit {:.2} ms, depth readback {:.2} ms ({} frames)",
            t[0] / n, t[1] / n, t[2] / n, t[3] / n, self.timing.0
        )
    }

    pub fn render(&mut self, built: &Built, mem: &dyn Mem) {
        self.stats.frames += 1;
        let p0 = (self.gs.prims, self.gs.dropped, self.gs.unknown_tex);
        let t_start = std::time::Instant::now();
        self.build(built, mem);
        let t_built = std::time::Instant::now();
        self.timing.0 += 1;
        self.timing.1[0] += (t_built - t_start).as_secs_f64() * 1e3;
        let (w, h) = self.gs.size;
        self.ensure_targets(w, h);
        self.ensure_rts();
        self.stats.prims += self.gs.prims - p0.0;
        self.stats.dropped += self.gs.dropped - p0.1;
        self.stats.unknown_tex += self.gs.unknown_tex - p0.2;
        if self.gs.verts.is_empty() {
            return;
        }
        // Vertex buffer.
        // SAFETY: `Vtx` is `repr(C)`, twelve f32s with no padding, in the
        // vertex layout's order; the slice covers exactly the vertices.
        let vbytes: &[u8] = unsafe { std::slice::from_raw_parts(self.gs.verts.as_ptr() as *const u8, std::mem::size_of_val(&self.gs.verts[..])) };
        let vbuf = self.dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx verts"),
            size: vbytes.len() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&vbuf, 0, vbytes);
        // Plans per batch, uniforms, bind groups.
        let batches = std::mem::take(&mut self.gs.batches);
        self.stats.batches += batches.len() as u64;
        struct Item {
            /// Samples the frame: needs a fresh snapshot unless the previous
            /// item did too.
            frame: bool,
            /// Tests the destination alpha: needs a fresh snapshot.
            date: bool,
            /// Reads the depth buffer: needs a fresh depth snapshot.
            zfog: bool,
            /// FBP drawn into.
            target: u32,
            key: draw::PipeKey,
            ubo: u32,
            constant: f64,
            bind: usize,
            first: u32,
            count: u32,
            scissor: Option<[u32; 4]>,
            /// What it is (pixel probe).
            desc: String,
            /// Its batch.
            batch: usize,
        }
        let mut items = Vec::new();
        let mut ubytes: Vec<u8> = Vec::new();
        let mut binds: Vec<(Option<wgpu::TextureView>, draw::SampKey)> = Vec::new();
        let mut bind_amax: Vec<u8> = Vec::new();
        let mut bind_ix: HashMap<(Option<u32>, u64, draw::SampKey), usize> = HashMap::new();
        // Debug: SSX3_GFX_SKIPTEST=hex,... drops batches with that TEST.
        // A trailing `c`/`a`: only colour-writing / only alpha-only batches.
        let dbg = debug::get();
        let skip_test = &dbg.skip_test;
        // Debug: SSX3_GFX_PROBE=x,y prints that main-target pixel after every
        // item that changes it (on the SSX3_GFX_TRACE frame, if set).
        let probe = dbg.probe.filter(|_| dbg.trace.is_none_or(|f| f == self.stats.frames));
        for (bi, b) in batches.iter().enumerate() {
            let st = &b.state;
            let alpha_only = (st.ctx.frame >> 32) as u32 & 0x00ff_ffff == 0x00ff_ffff;
            // `z`: keep the batch but without depth writes.
            let no_zw = skip_test.iter().any(|&(t, m)| t == st.ctx.test && m == b'z');
            if skip_test.iter().any(|&(t, m)| t == st.ctx.test && m != b'z' && (m == 0 || (m == b'a') == alpha_only)) {
                continue;
            }
            let sk = draw::samp_key(st);
            let bkey = (st.tex, st.texa, sk);
            let bind = match bind_ix.get(&bkey) {
                Some(&i) => i,
                None => {
                    // (Frame and render-target alpha is stored up to 0x80.)
                    let (view, amax) = match st.tex {
                        Some(gs::FRAME_TEX) => {
                            let t = self.targets.as_ref().expect("targets");
                            (Some(t.snapshot.create_view(&Default::default())), 128)
                        }
                        Some(t) if t & gs::RT_TEX != 0 => (self.rts.get(&(t & !gs::RT_TEX)).map(|r| r.color.create_view(&Default::default())), 128),
                        Some(t) if t & gs::ZFOG_TEX != 0 => (self.clut_texture(t & !gs::ZFOG_TEX), 255),
                        Some(info) => {
                            if dbg.texlog == Some(self.stats.frames) {
                                let k = tex::key(mem, info, st.texa);
                                eprintln!("[gfx-tex] info {info:#x} format {:#x} pixels {:#x} tex0 {:#x} hash {:#x}", k.format, k.pixels, k.tex0, k.hash);
                            }
                            match self.texture(info, st.texa, mem) {
                                Some((v, a)) => (Some(v), a),
                                None => (None, 128),
                            }
                        }
                        None => (None, 0),
                    };
                    bind_amax.push(amax);
                    binds.push((view, sk));
                    bind_ix.insert(bkey, binds.len() - 1);
                    binds.len() - 1
                }
            };
            let (tw, th) = if st.target == self.gs.main_fbp {
                (w, h)
            } else {
                match self.rts.get(&st.target) {
                    Some(r) => (r.w, r.h),
                    None => continue,
                }
            };
            let sc = st.ctx.scissor;
            let (x0, x1, y0, y1) = ((sc & 0x7ff) as u32, ((sc >> 16) & 0x7ff) as u32, ((sc >> 32) & 0x7ff) as u32, ((sc >> 48) & 0x7ff) as u32);
            let scissor = (x0 <= x1 && y0 <= y1 && x0 < tw && y0 < th).then(|| [x0, y0, (x1 + 1).min(tw) - x0, (y1 + 1).min(th) - y0]);
            if scissor.is_none() {
                continue;
            }
            let rt_psm = st.tex.filter(|t| *t != gs::FRAME_TEX && t & gs::RT_TEX != 0).and_then(|t| self.gs.rts.get(&(t & !gs::RT_TEX))).map(|r| r.psm);
            // DATE on the main target (render targets: not yet).
            let date = (st.ctx.test >> 14) & 1 != 0 && st.target == self.gs.main_fbp;
            let zfog = st.tex.is_some_and(|t| t != gs::FRAME_TEX && t & gs::RT_TEX == 0 && t & gs::ZFOG_TEX != 0);
            // The largest As the batch can produce (blends with As > 1
            // need care: see draw::plans).
            let av = self.gs.verts[b.first as usize..(b.first + b.count) as usize].iter().fold(0f32, |m, v| m.max(v.col[3]));
            let ta = bind_amax[bind] as f32;
            let tcc = (st.ctx.tex0 >> 34) & 1 != 0;
            let as_max = if st.prim & (1 << 4) == 0 || !tcc {
                av
            } else {
                match (st.ctx.tex0 >> 35) & 3 {
                    0 => ta * av / 128.0,
                    2 => ta + av,
                    _ => ta,
                }
            };
            for mut p in draw::plans(st, (tw, th), rt_psm, as_max) {
                if no_zw {
                    p.key.depth_write = false;
                }
                if dbg.planlog == Some(st.ctx.test) {
                    eprintln!("[gfx-plan] alpha {:#x} fba {} flags {:#x} tfx {} src_mode {} cs {} {} c_kind {} fix {} key {:?}", st.ctx.alpha, st.ctx.fba, p.u.flags, p.u.tfx, p.u.src_mode, p.u.cs_a, p.u.cs_b, p.u.c_kind, p.u.fix, p.key);
                }
                let ubo = (ubytes.len() as u64 / draw::UNIFORM_STRIDE) as u32;
                if date {
                    p.u.flags |= 64 | (((st.ctx.test >> 15) & 1) as u32) << 7;
                }
                if zfog {
                    p.u.flags |= 256;
                    // Depth is stored as Z / (ZMAX + 1).
                    p.u.pad[0] = match (self.gs.ctx[0].zbuf >> 24) & 15 {
                        0 => 4294967296.0,
                        1 => 16777216.0,
                        _ => 65536.0,
                    };
                }
                let u = &p.u;
                let words: [u32; 20] = [
                    u.size[0].to_bits(),
                    u.size[1].to_bits(),
                    u.tfx,
                    u.flags,
                    u.atst,
                    u.aref,
                    u.side,
                    u.src_mode,
                    u.cs_a.to_bits(),
                    u.cs_b.to_bits(),
                    u.c_kind,
                    u.fix.to_bits(),
                    u.fogcol[0].to_bits(),
                    u.fogcol[1].to_bits(),
                    u.fogcol[2].to_bits(),
                    u.fogcol[3].to_bits(),
                    u.ta[0].to_bits(),
                    u.ta[1].to_bits(),
                    u.pad[0].to_bits(),
                    0,
                ];
                ubytes.extend(words.iter().flat_map(|w| w.to_le_bytes()));
                ubytes.resize(ubytes.len().next_multiple_of(draw::UNIFORM_STRIDE as usize), 0);
                let desc = if probe.is_some() {
                    format!(
                        "prim {:#x} tex {:x?} tex0 {:#x} alpha {:#x} test {:#x} fba {} frame {:#x} flags {:#x} src_mode {} key {:?}",
                        st.prim, st.tex, st.ctx.tex0, st.ctx.alpha, st.ctx.test, st.ctx.fba, st.ctx.frame, p.u.flags, p.u.src_mode, p.key
                    )
                } else {
                    String::new()
                };
                items.push(Item { batch: bi, desc, frame: st.tex == Some(gs::FRAME_TEX), date, zfog, target: st.target, key: p.key, ubo, constant: p.constant, bind, first: b.first, count: b.count, scissor });
            }
        }
        if items.is_empty() {
            return;
        }
        let t_prepared = std::time::Instant::now();
        self.timing.1[1] += (t_prepared - t_built).as_secs_f64() * 1e3;
        let ubuf = self.dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx uniforms"),
            size: ubytes.len() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&ubuf, 0, &ubytes);
        let snap = self.targets.as_ref().expect("targets").snapshot.create_view(&Default::default());
        let zsnap = self.targets.as_ref().expect("targets").zsnap.create_view(&Default::default());
        let groups: Vec<wgpu::BindGroup> = binds
            .iter()
            .map(|(view, sk)| {
                let samp = self.samplers.entry(*sk).or_insert_with(|| draw::sampler(&self.dev, *sk)).clone();
                self.dev.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("ssx3-gfx batch"),
                    layout: &self.pipes.layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                buffer: &ubuf,
                                offset: 0,
                                size: wgpu::BufferSize::new(draw::BATCH_BYTES),
                            }),
                        },
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(view.as_ref().unwrap_or(&self.white)) },
                        wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&samp) },
                        wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&snap) },
                        wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&zsnap) },
                    ],
                })
            })
            .collect();
        for it in &items {
            self.pipes.get(&self.dev, it.key);
        }
        let t = self.targets.as_ref().expect("targets");
        let cv = t.color.create_view(&Default::default());
        let dv = t.depth.create_view(&Default::default());
        let mut enc = self.dev.create_command_encoder(&Default::default());
        let main = self.gs.main_fbp;
        let rts = &self.rts;
        // A pass on target `fbp`; depth starts cleared on a target's first
        // pass of the frame.
        let begin = |enc: &mut wgpu::CommandEncoder, fbp: u32, first: bool| -> wgpu::RenderPass<'static> {
            let (c, d) = if fbp == main {
                (cv.clone(), dv.clone())
            } else {
                let r = &rts[&fbp];
                (r.color.create_view(&Default::default()), r.depth.create_view(&Default::default()))
            };
            enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssx3-gfx frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &c,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &d,
                    // Each frame starts with depth cleared to the farthest
                    // value (the GS's Z history across frames is not kept).
                    depth_ops: Some(wgpu::Operations {
                        load: if first { wgpu::LoadOp::Clear(0.0) } else { wgpu::LoadOp::Load },
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            })
            .forget_lifetime()
        };
        let mut started: std::collections::HashSet<u32> = Default::default();
        let mut cur_target = main;
        started.insert(main);
        let mut pass = begin(&mut enc, main, true);
        pass.set_vertex_buffer(0, vbuf.slice(..));
        let mut prev_frame = false;
        let mut probed: Vec<String> = Vec::new();
        let probe_buf = probe.map(|_| {
            self.dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ssx3-gfx probe"),
                size: items.len().max(1) as u64 * 256,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            })
        });
        let phase1_end = self.phase1_end;
        let mut depth_taken = false;
        let take_depth = |enc: &mut wgpu::CommandEncoder| {
            enc.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo { texture: &t.depth, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyTextureInfo { texture: &t.qdepth, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
            );
        };
        for it in &items {
            if self.want_depth && !depth_taken && it.batch >= phase1_end {
                depth_taken = true;
                drop(pass);
                take_depth(&mut enc);
                pass = begin(&mut enc, cur_target, false);
                pass.set_vertex_buffer(0, vbuf.slice(..));
            }
            let Some([x, y, sw, sh]) = it.scissor else { continue };
            // (Frame-sampling items may draw into another target: the
            // display buffer copy.)
            // A new run of frame-sampling items, or one into another target
            // (the display copy after the colour filter), needs a fresh one.
            let snapshot = it.frame && (!prev_frame || it.target != cur_target) || it.date && it.target == main;
            let zsnapshot = it.zfog && it.target == main;
            if it.target != cur_target || snapshot || zsnapshot {
                drop(pass);
                if zsnapshot {
                    enc.copy_texture_to_texture(
                        wgpu::TexelCopyTextureInfo { texture: &t.depth, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                        wgpu::TexelCopyTextureInfo { texture: &t.zsnap, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                        wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
                    );
                }
                if snapshot {
                    // Snapshot the frame as drawn so far.
                    enc.copy_texture_to_texture(
                        wgpu::TexelCopyTextureInfo { texture: &t.color, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                        wgpu::TexelCopyTextureInfo { texture: &t.snapshot, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                        wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
                    );
                }
                cur_target = it.target;
                let first = started.insert(cur_target);
                pass = begin(&mut enc, cur_target, first);
                pass.set_vertex_buffer(0, vbuf.slice(..));
            }
            prev_frame = it.frame;
            pass.set_pipeline(self.pipes.cached(it.key));
            pass.set_bind_group(0, &groups[it.bind], &[it.ubo * draw::UNIFORM_STRIDE as u32]);
            pass.set_blend_constant(wgpu::Color { r: it.constant, g: it.constant, b: it.constant, a: it.constant });
            pass.set_scissor_rect(x, y, sw, sh);
            pass.draw(it.first..it.first + it.count, 0..1);
            if let (Some((px, py)), Some(pb)) = (probe, probe_buf.as_ref()) {
                if it.target == main {
                    drop(pass);
                    enc.copy_texture_to_buffer(
                        wgpu::TexelCopyTextureInfo { texture: &t.color, mip_level: 0, origin: wgpu::Origin3d { x: px, y: py, z: 0 }, aspect: wgpu::TextureAspect::All },
                        wgpu::TexelCopyBufferInfo { buffer: pb, layout: wgpu::TexelCopyBufferLayout { offset: probed.len() as u64 * 256, bytes_per_row: Some(256), rows_per_image: Some(1) } },
                        wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
                    );
                    probed.push(it.desc.clone());
                    pass = begin(&mut enc, cur_target, false);
                    pass.set_vertex_buffer(0, vbuf.slice(..));
                }
            }
        }
        drop(pass);
        if self.want_depth && !depth_taken {
            take_depth(&mut enc);
        }
        // The image on screen, for the window.
        let shown = match self.display_fbp.and_then(|f| self.rts.get(&f)) {
            Some(r) => &r.color,
            None => &t.color,
        };
        let present = self.present.get_or_insert_with(|| {
            self.dev.create_texture(&wgpu::TextureDescriptor {
                label: Some("ssx3-gfx present"),
                size: wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            })
        });
        if (present.width(), present.height()) != (t.w, t.h) {
            *present = self.dev.create_texture(&wgpu::TextureDescriptor {
                label: Some("ssx3-gfx present"),
                size: wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            });
        }
        enc.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo { texture: shown, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyTextureInfo { texture: present, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
        );
        // Depth after phase 1, read back for the light queries.
        let depth_buf = self.want_depth.then(|| {
            let buf = self.dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ssx3-gfx depth readback"),
                size: (t.w * 4).next_multiple_of(256) as u64 * t.h as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            enc.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo { texture: &t.qdepth, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::DepthOnly },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buf,
                    layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some((t.w * 4).next_multiple_of(256)), rows_per_image: Some(t.h) },
                },
                wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
            );
            buf
        });
        self.queue.submit(std::iter::once(enc.finish()));
        let t_sub = std::time::Instant::now();
        self.timing.1[2] += (t_sub - t_prepared).as_secs_f64() * 1e3;
        if let Some(buf) = depth_buf {
            let slice = buf.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            let _ = self.dev.poll(wgpu::PollType::wait_indefinitely());
            if let Ok(v) = slice.get_mapped_range() {
                let row = (t.w * 4).next_multiple_of(256) as usize;
                let zs = self.zscale as f64;
                let mut z = Vec::with_capacity((t.w * t.h) as usize);
                for y in 0..t.h as usize {
                    for d in v[y * row..y * row + 4 * t.w as usize].chunks_exact(4) {
                        let d = f32::from_le_bytes([d[0], d[1], d[2], d[3]]);
                        z.push((d as f64 * zs).round().min(zs - 1.0) as u32);
                    }
                }
                self.last_depth = Some((t.w, t.h, z));
            }
            self.timing.1[3] += t_sub.elapsed().as_secs_f64() * 1e3;
        }
        if let Some(pb) = probe_buf {
            let slice = pb.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            let _ = self.dev.poll(wgpu::PollType::wait_indefinitely());
            if let Ok(v) = slice.get_mapped_range() {
                let mut last = u32::MAX;
                for (k, d) in probed.iter().enumerate() {
                    let px = u32::from_le_bytes(v[k * 256..k * 256 + 4].try_into().unwrap());
                    if px != last {
                        let (r, g, b, a) = (px & 0xff, (px >> 8) & 0xff, (px >> 16) & 0xff, px >> 24);
                        eprintln!("[probe] item {k}: rgba {r} {g} {b} a{:.0} <- {d}", a as f32 * 128.0 / 255.0);
                        last = px;
                    }
                }
            }
        }
        // Drop textures unused for a while.
        let frame = self.stats.frames;
        self.texcache.retain(|_, t| frame - t.used < 600);
    }

    /// The image on screen: the display buffer when the game blends frames
    /// into it, else the draw buffer.
    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)> {
        let t = self.targets.as_ref()?;
        if let Some(r) = self.display_fbp.and_then(|f| self.rts.get(&f)) {
            let (rw, _, px) = self.read_texture(&r.color, r.w, r.h);
            let (w, h) = (t.w.min(rw), t.h.min(r.h));
            let out = (0..h).flat_map(|y| px[(y * rw) as usize..(y * rw + w) as usize].iter().copied()).collect();
            return Some((w, h, out));
        }
        Some(self.read_texture(&t.color, t.w, t.h))
    }

    /// The draw buffer as last drawn.
    pub fn read_draw_buffer(&self) -> Option<(u32, u32, Vec<u32>)> {
        let t = self.targets.as_ref()?;
        Some(self.read_texture(&t.color, t.w, t.h))
    }

    fn read_texture(&self, tex: &wgpu::Texture, w: u32, h: u32) -> (u32, u32, Vec<u32>) {
        struct T {
            w: u32,
            h: u32,
        }
        let t = T { w, h };
        let row = (t.w * 4).next_multiple_of(256);
        let buf = self.dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx read"),
            size: row as u64 * t.h as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut enc = self.dev.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(t.h) } },
            wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
        );
        self.queue.submit(std::iter::once(enc.finish()));
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.dev.poll(wgpu::PollType::wait_indefinitely());
        let Ok(v) = slice.get_mapped_range() else { return (w, h, Vec::new()) };
        let mut px = Vec::with_capacity((t.w * t.h) as usize);
        for y in 0..t.h as usize {
            for x in 0..t.w as usize {
                let o = y * row as usize + x * 4;
                px.push(u32::from_le_bytes([v[o], v[o + 1], v[o + 2], v[o + 3]]));
            }
        }
        (t.w, t.h, px)
    }
}

/// Renderer_SetDisplayRegs (0x382af0) builds a GS state packet at
/// 0x4ffcc0 on every flip and sends it by normal GIF DMA, outside every
/// chain. Its last tag writes SCISSOR_1/2 (NLOOP 2, A+D, EOP).
const FLIP_PACKET: u32 = 0x4ffcc0;

pub fn flip_packet(mem: &dyn Mem) -> Vec<u128> {
    let mut q = Vec::new();
    let mut a = FLIP_PACKET;
    while q.len() < 512 {
        let tag = mem.qword(a);
        let lo = tag as u64;
        let nloop = (lo & 0x7fff) as usize;
        let flg = (lo >> 58) & 3;
        let nreg = match (lo >> 60) as usize { 0 => 16, n => n };
        let n = match flg {
            0 => nloop * nreg,
            1 => (nloop * nreg).div_ceil(2),
            _ => nloop,
        };
        for k in 0..=n as u32 {
            q.push(mem.qword(a + 16 * k));
        }
        a += 16 * (n as u32 + 1);
        if lo == 0x1000_0000_0000_8002 && (tag >> 64) as u64 == 0xe {
            break;
        }
    }
    q
}
