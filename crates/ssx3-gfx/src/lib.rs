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
pub mod interp;
pub mod pack;
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
    shared: Arc<Shared>,
    /// The browser: the frame whose VU1 work is running on the helper
    /// workers (`publish_staged`).
    staged: Option<Staged>,
    /// Typed frames from the published memory (in order).
    builder: ssx3_ps2::frame_sink::FrameBuilder,
    dev: wgpu::Device,
    queue: wgpu::Queue,
    pipes: draw::Pipelines,
    targets: Option<Targets>,
    rts: HashMap<u32, RtTarget>,
    gs: gs::Gs,
    vu: Box<[u8; 16384]>,
    texcache: HashMap<tex::TexKey, CachedTex>,
    samplers: HashMap<draw::SampKey, wgpu::Sampler>,
    /// Anisotropic filtering level for linear samplers (1 = off; affects
    /// textures with mip levels: replacements).
    aniso: u16,
    white: wgpu::TextureView,
    /// TexInfos that failed to decode (reported once).
    failed: std::collections::HashSet<u32>,
    pub stats: Stats,
    /// Texture dumps and replacements (`pack`).
    pack: pack::Pack,
    /// The display buffer, when the game blends frames into it.
    display_fbp: Option<u32>,
    /// Internal resolution: GPU targets are this many times the GS's.
    scale: u32,
    /// Output size (overrides `scale`): the main target at any size; GS
    /// pixels map to it with separate x and y factors.
    out_size: Option<(u32, u32)>,
    /// This frame's GS → target pixel factors (main target), and the
    /// integer factor of the render targets.
    fxy: (f32, f32),
    rt_scale: u32,
    /// Horizontal squeeze of the HUD (layers 9 and up) around the centre,
    /// so 4:3 HUD art keeps its shape on a wider output (1: none).
    pub hud_squeeze: f32,
    /// Batches of VIF phase 1 (the 3D scene) in this frame.
    phase1_end: usize,
    /// Keep the depth after phase 1 (`depth()`, native mode).
    pub want_depth: bool,
    /// This frame's depth readback in flight (buffer, size, ZMAX + 1).
    /// (The flag: mapped; the browser cannot wait for it.)
    pending_depth: Option<(wgpu::Buffer, wgpu::SubmissionIndex, u32, u32, f32, u32, u32, Arc<std::sync::atomic::AtomicBool>)>,
    /// ZMAX + 1 of the frame's Z buffer (depth is stored as Z / it).
    zscale: f32,
    /// Frames timed and the time (ms) in build, prepare, encode+submit,
    /// depth readback.
    timing: (u64, [f64; 4]),
    /// The images on screen, for presenting (sRGB view allowed): with
    /// interpolation the in-between images first, the frame's own last.
    present: Vec<wgpu::Texture>,
    /// Images per game frame (1: no interpolation; see `interp`), and how
    /// many frames in a row took longer than the game's frame time.
    subframes: u32,
    over_budget: u32,
    /// The last frame's matrix records (interpolation).
    prev_records: Option<interp::Records>,
    /// While an in-between image is drawn: its VU header substitutions and
    /// its slot in `present`.
    subst: Option<Arc<HashMap<[u32; 16], [u32; 16]>>>,
    model_subst: Option<Arc<HashMap<[u32; 16], [u32; 16]>>>,
    slot: Option<usize>,
    /// Images per frame this frame (1, or `subframes` once there is a
    /// previous frame to interpolate from).
    nslots: usize,
    /// Render target contents saved while in-between images are drawn.
    backups: HashMap<u32, wgpu::Texture>,
    /// Vertex and uniform buffers, kept across frames (grown as needed).
    vbuf: Option<wgpu::Buffer>,
    ubuf: Option<wgpu::Buffer>,
}

/// The renderer; clone the handle to read images from another thread.
#[derive(Clone)]
pub struct Renderer(pub Arc<Mutex<Inner>>, Arc<Shared>);

/// A frame handed to the render thread: the GraphicsMan that published it
/// and a snapshot of EE memory.
type Job = (u32, Box<ssx3_ps2::frame_sink::DumpMem>);

/// What the render thread shares with the game and the window without
/// holding the renderer: the latest depth for the light queries and the
/// latest image on screen.
#[derive(Default)]
pub struct Shared {
    tx: Mutex<Option<std::sync::mpsc::SyncSender<Job>>>,
    /// Frames sent and not yet rendered.
    pending: Mutex<usize>,
    idle: std::sync::Condvar,
    depth: Mutex<Option<ssx3_ps2::frame_sink::Depth>>,
    /// The latest frame's images (in-between ones first) and when they
    /// were ready.
    present: Mutex<Option<(Vec<wgpu::Texture>, web_time::Instant)>>,
    /// The frame thread's building time (ms) and frames.
    build_ms: Mutex<(f64, u64)>,
}

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
        let shared = Arc::new(Shared::default());
        Renderer(Arc::new(Mutex::new(Inner {
            shared: shared.clone(),
            builder: Default::default(),
            dev,
            queue,
            pipes,
            targets: None,
            rts: HashMap::new(),
            gs: gs::Gs::new(),
            vu: Box::new([0; 16384]),
            texcache: HashMap::new(),
            samplers: HashMap::new(),
            aniso: 16,
            white,
            failed: Default::default(),
            stats: Stats::default(),
            pack: pack::Pack::from_env(),
            display_fbp: None,
            scale: 1,
            out_size: None,
            fxy: (1.0, 1.0),
            rt_scale: 1,
            hud_squeeze: 1.0,
            phase1_end: usize::MAX,
            want_depth: false,
            pending_depth: None,
            zscale: 16777216.0,
            timing: (0, [0.0; 4]),
            present: Vec::new(),
            subframes: 1,
            staged: None,
            over_budget: 0,
            prev_records: None,
            subst: None,
            model_subst: None,
            slot: None,
            nslots: 1,
            backups: HashMap::new(),
            vbuf: None,
            ubuf: None,
        })), shared)
    }

    /// Render on a thread of its own from now on: frames are rendered while
    /// the game goes on (at most one frame queued; the game waits if the
    /// renderer falls further behind).
    pub fn start_worker(&self) {
        // Two stages: the typed frame is built from the snapshot on one
        // thread while the previous frame renders on another.
        let (tx, rx) = std::sync::mpsc::sync_channel::<Job>(1);
        let (btx, brx) = std::sync::mpsc::sync_channel::<(Built, ssx3_render::frame::Carry, Box<ssx3_ps2::frame_sink::DumpMem>)>(1);
        *self.1.tx.lock().unwrap() = Some(tx);
        let mut builder = ssx3_ps2::frame_sink::FrameBuilder::default();
        let stats = self.1.clone();
        std::thread::Builder::new()
            .name("ssx3-gfx-frame".into())
            .spawn(move || {
                while let Ok((gm, mem)) = rx.recv() {
                    let (built, carry) = builder.build(gm, &mem);
                    *stats.build_ms.lock().unwrap() = (builder.ms, builder.frames);
                    if btx.send((built, carry, mem)).is_err() {
                        break;
                    }
                }
            })
            .expect("frame thread");
        let (inner, shared) = (self.0.clone(), self.1.clone());
        std::thread::Builder::new()
            .name("ssx3-gfx".into())
            .spawn(move || {
                while let Ok((built, carry, mem)) = brx.recv() {
                    inner.lock().unwrap().render_built(&built, &carry, &mem);
                    drop(built);
                    ssx3_ps2::frame_sink::recycle(mem);
                    let mut p = shared.pending.lock().unwrap();
                    *p -= 1;
                    shared.idle.notify_all();
                }
            })
            .expect("render thread");
    }

    /// Wait until every frame handed over is rendered.
    pub fn flush(&self) {
        let mut p = self.1.pending.lock().unwrap();
        while *p > 0 {
            p = self.1.idle.wait(p).unwrap();
        }
    }

    /// The image on screen as RGBA8 (GS alpha), read back (after the
    /// frames handed over are rendered).
    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)> {
        self.flush();
        self.0.lock().unwrap().read_image()
    }

    /// The last frame's image on screen as an sRGB view (same device as
    /// the GS), with its size.
    pub fn present_view(&self) -> Option<(wgpu::TextureView, u32, u32)> {
        self.present_view_at(None)
    }

    /// The image to show now: with interpolation, the in-between images of
    /// the latest frame one after another over `frame` (the game's frame
    /// time) from when they were ready, then the frame's own.
    pub fn present_view_at(&self, frame: Option<std::time::Duration>) -> Option<(wgpu::TextureView, u32, u32)> {
        let g = self.1.present.lock().unwrap();
        let (ts, at) = g.as_ref()?;
        let i = match frame {
            Some(f) if ts.len() > 1 => ((at.elapsed().as_secs_f64() / f.as_secs_f64() * ts.len() as f64) as usize).min(ts.len() - 1),
            _ => ts.len() - 1,
        };
        let t = &ts[i];
        let view = t.create_view(&wgpu::TextureViewDescriptor { format: Some(wgpu::TextureFormat::Rgba8UnormSrgb), ..Default::default() });
        Some((view, t.width(), t.height()))
    }

    /// Anisotropic filtering: 1 (off) to 16 (the default).
    pub fn set_anisotropy(&self, n: u16) {
        let mut g = self.0.lock().unwrap();
        g.aniso = n.clamp(1, 16);
        g.samplers.clear();
    }

    /// Images per game frame: 1 (the default) shows the game's frames; `n`
    /// adds `n - 1` interpolated ones before each (see `interp`), for a
    /// display at `n` × 60 Hz. One frame of extra latency.
    pub fn set_subframes(&self, n: u32) {
        self.0.lock().unwrap().subframes = n.clamp(1, 8);
    }

    /// The last frame's images on screen, read back (in-between ones
    /// first; debug).
    pub fn read_presents(&self) -> Vec<(u32, u32, Vec<u32>)> {
        self.flush();
        let g = self.0.lock().unwrap();
        g.present.iter().map(|p| g.read_texture(p, p.width(), p.height())).collect()
    }

    /// Render at `n`× the GS resolution (1..=8).
    pub fn set_scale(&self, n: u32) {
        self.0.lock().unwrap().scale = n.clamp(1, 8);
    }

    /// Render to `w`×`h` pixels whatever the GS resolution (None: `scale`×
    /// the GS's). With an aspect wider than 4:3 the HUD is drawn at 4:3
    /// proportions in the centre (the game's 3D view is widened separately,
    /// `ssx3_ps2::frame_sink::set_view`).
    pub fn set_output_size(&self, size: Option<(u32, u32)>) {
        let mut g = self.0.lock().unwrap();
        g.out_size = size.map(|(w, h)| (w.clamp(64, 8192), h.clamp(64, 8192)));
        g.hud_squeeze = match g.out_size {
            Some((w, h)) => ((4.0 / 3.0) / (w as f32 / h as f32)).min(1.0),
            None => 1.0,
        };
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
    fn frame(&mut self, gm: u32, mem: Box<ssx3_ps2::frame_sink::DumpMem>) {
        let tx = self.1.tx.lock().unwrap().clone();
        match tx {
            Some(tx) => {
                *self.1.pending.lock().unwrap() += 1;
                if tx.send((gm, mem)).is_err() {
                    *self.1.pending.lock().unwrap() -= 1;
                }
            }
            None => {
                let mut g = self.0.lock().unwrap();
                if OFFLOAD.get().is_some() && g.subframes <= 1 {
                    g.publish_staged(gm, mem);
                } else {
                    g.finish_staged();
                    g.publish(gm, &mem);
                    ssx3_ps2::frame_sink::recycle(mem);
                }
            }
        }
    }

    fn depth(&mut self) -> Option<ssx3_ps2::frame_sink::Depth> {
        self.1.depth.lock().unwrap().take()
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
        let color = mk("ssx3-gfx colour", draw::COLOR_FORMAT, wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST);
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
        let s = self.rt_scale;
        for (&fbp, rt) in &self.gs.rts {
            // The display buffer is a copy of the frame: the main target's
            // scale; other targets an integer one.
            let (fx, fy) = self.rt_fxy(fbp);
            let _ = s;
            let (pw, ph) = ((rt.w as f32 * fx).round() as u32, (rt.h as f32 * fy).round() as u32);
            if self.rts.get(&fbp).is_some_and(|t| t.w == pw && t.h == ph) {
                continue;
            }
            let mk = |format, extra| {
                self.dev.create_texture(&wgpu::TextureDescriptor {
                    label: Some("ssx3-gfx rt"),
                    size: wgpu::Extent3d { width: pw, height: ph, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | extra,
                    view_formats: &[],
                })
            };
            let color = mk(draw::COLOR_FORMAT, wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST);
            let depth = mk(draw::DEPTH_FORMAT, wgpu::TextureUsages::empty());
            self.rts.insert(fbp, RtTarget { w: pw, h: ph, color, depth });
        }
    }

    /// Build the geometry of one frame.
    /// The texture for a TexInfo, and its largest texel alpha.
    /// Plan the frame's VU1 calls: split into chunks (see `VuPlan`), each
    /// with the VU memory it starts from.

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
        // Texture packs: dump it, or use its replacement (with mipmaps).
        self.pack.dump(&key, &d);
        let replaced = self.pack.load(&key);
        if let Some(r) = &replaced {
            let (a, b) = (r[0].width as f32 / r[0].height as f32, d.width as f32 / d.height as f32);
            if (a / b - 1.0).abs() > 0.01 {
                eprintln!("[gfx] replacement {} is {}x{}, the texture {}x{}: aspect ratio differs", pack::name(&key), r[0].width, r[0].height, d.width, d.height);
            }
        }
        let levels: &[tex::Decoded] = replaced.as_deref().unwrap_or(std::slice::from_ref(&d));
        let t = self.dev.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssx3-gfx tex"),
            size: wgpu::Extent3d { width: levels[0].width, height: levels[0].height, depth_or_array_layers: 1 },
            mip_level_count: levels.len() as u32,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        for (lv, l) in levels.iter().enumerate() {
            let bytes: Vec<u8> = l.rgba.iter().flat_map(|v| v.to_le_bytes()).collect();
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &t, mip_level: lv as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                &bytes,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(l.width * 4), rows_per_image: Some(l.height) },
                wgpu::Extent3d { width: l.width, height: l.height, depth_or_array_layers: 1 },
            );
        }
        let bytes: Vec<u8> = d.rgba.iter().flat_map(|v| v.to_le_bytes()).collect();
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
        let amax = levels[0].rgba.iter().map(|p| (p >> 24) as u8).max().unwrap_or(0);
        self.texcache.insert(key, CachedTex { view: view.clone(), used: frame, amax });
        Some((view, amax))
    }

    /// Replace the VU1 data memory image (offline replays start from the
    /// emulated one).
    pub fn seed_vu(&mut self, data: &[u8]) {
        let n = data.len().min(self.vu.len());
        self.vu[..n].copy_from_slice(&data[..n]);
    }

    /// GS → target pixel factors of render target `fbp`.
    fn rt_fxy(&self, fbp: u32) -> (f32, f32) {
        if Some(fbp) == self.display_fbp {
            self.fxy
        } else {
            (self.rt_scale as f32, self.rt_scale as f32)
        }
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
            "gfx per frame: typed frame {:.2} ms, build {:.2} ms, prepare {:.2} ms, encode+submit {:.2} ms, depth readback {:.2} ms ({} frames)",
            {
                let (ms, n) = *self.shared.build_ms.lock().unwrap();
                let (ms, n) = if n > 0 { (ms, n) } else { (self.builder.ms, self.builder.frames) };
                ms / n.max(1) as f64
            },
            t[0] / n, t[1] / n, t[2] / n, t[3] / n, self.timing.0
        )
    }

    /// A published frame: build the typed frame from the memory snapshot
    /// and render it.
    pub fn publish(&mut self, gm: u32, mem: &ssx3_ps2::frame_sink::DumpMem) {
        let (built, carry) = self.builder.build(gm, mem);
        self.render_built(&built, &carry, mem);
    }

    /// With a [`VuOffload`] and no render thread (the browser): start this
    /// frame's VU1 chunks on the helpers and draw it when the next frame
    /// comes, so the game's next frame runs while they work. One frame of
    /// latency, as with the native render thread.
    fn publish_staged(&mut self, gm: u32, mem: Box<ssx3_ps2::frame_sink::DumpMem>) {
        self.finish_staged();
        let (built, carry) = self.builder.build(gm, &mem);
        let b = self.take_build();
        let collect = {
            let draws: Vec<&frame::Draw> = built.frame.draws().collect();
            let plan = Build::plan_vu_calls(&b, &built, &draws);
            plan.offloadable().then(|| (plan.bounds.len(), (OFFLOAD.get().unwrap().0)(plan.jobs())))
        };
        self.put_build(b);
        self.staged = Some(Staged { mem, built, carry, collect });
    }

    /// Draw the staged frame, if any (`publish_staged`).
    pub fn finish_staged(&mut self) {
        let Some(s) = self.staged.take() else { return };
        *PREFETCH.lock().unwrap() = s.collect;
        self.render_built(&s.built, &s.carry, &s.mem);
        // Not taken (the plan came out different): still wait, the helpers
        // must be idle before the next jobs.
        if let Some((_, c)) = PREFETCH.lock().unwrap().take() {
            c();
        }
        ssx3_ps2::frame_sink::recycle(s.mem);
    }

    /// Render a typed frame (built with `carry`) over its memory snapshot.
    pub fn render_built(&mut self, built: &Built, carry: &ssx3_render::frame::Carry, mem: &ssx3_ps2::frame_sink::DumpMem) {
        ssx3_ps2::frame_sink::dump_frame(carry, mem, &self.vu[..]);
        self.render(built, mem);
    }

    /// Render a frame. With interpolation (`set_subframes`), first the
    /// in-between images (frame N with its matrices moved back towards
    /// N-1, see `interp`), each from the state frame N starts from, then
    /// the frame's own (exactly as without interpolation).
    pub fn render(&mut self, built: &Built, mem: &(dyn Mem + Sync)) {
        let t_frame = web_time::Instant::now();
        let cur = (self.subframes > 1).then(|| records(built));
        let k = if cur.is_some() && self.prev_records.is_some() { self.subframes as usize } else { 1 };
        self.nslots = k;
        if k > 1 {
            let (prev, now) = (self.prev_records.as_ref().unwrap(), cur.as_ref().unwrap());
            let pairs = interp::pair(prev, now);
            let subs: Vec<_> = (1..k)
                .map(|s| {
                    let t = debug_t().unwrap_or(s as f32 / k as f32);
                    (interp::substitutions(prev, now, &pairs, t), interp::model_substitutions(prev, now, t))
                })
                .collect();
            // Build every image's geometry at once: the in-between ones from
            // copies of the state the frame starts from, the frame's own
            // from the renderer's.
            self.gs.verts.clear();
            self.gs.batches.clear();
            self.stats.frames += 1;
            let t_start = web_time::Instant::now();
            let mut fin = self.take_build();
            let mut mids: Vec<Build> = subs
                .into_iter()
                .map(|(m, mm)| {
                    let mut b = fin.clone();
                    b.subst = Some(Arc::new(m));
                    b.model_subst = Some(Arc::new(mm));
                    b
                })
                .collect();
            std::thread::scope(|sc| {
                for b in mids.iter_mut() {
                    if cfg!(target_arch = "wasm32") {
                        Build::run(b, built, mem);
                    } else {
                        sc.spawn(|| Build::run(b, built, mem));
                    }
                }
                Build::run(&mut fin, built, mem);
            });
            self.timing.1[0] += t_start.elapsed().as_secs_f64() * 1e3;
            // Draw them in order; what the in-between ones leave in the
            // render targets is undone before the frame's own.
            let final_stats = fin.stats;
            self.save_targets();
            for (s, b) in mids.into_iter().enumerate() {
                self.put_build(b);
                self.slot = Some(s);
                self.draw_built(mem);
            }
            self.slot = None;
            self.restore_targets();
            self.put_build(fin);
            self.stats = final_stats;
            self.draw_built(mem);
        } else {
            self.render_one(built, mem);
        }
        if !self.present.is_empty() {
            *self.shared.present.lock().unwrap() = Some((self.present.clone(), web_time::Instant::now()));
        }
        self.prev_records = cur;
        // Interpolation must keep up with the game: after a second of
        // frames over budget, one image per frame fewer.
        if k > 1 {
            let over = t_frame.elapsed().as_secs_f64() > 0.016;
            self.over_budget = if over { self.over_budget + 1 } else { 0 };
            if self.over_budget >= 60 {
                self.over_budget = 0;
                self.subframes -= 1;
                eprintln!("[gfx] interpolation too slow here: {} images per game frame from now on", self.subframes);
            }
        }
    }

    /// The colour of every target (main, render targets) into `backups`.
    fn save_targets(&mut self) {
        let mut srcs: Vec<(u32, &wgpu::Texture)> = self.rts.iter().map(|(&f, r)| (f, &r.color)).collect();
        if let Some(t) = self.targets.as_ref() {
            srcs.push((u32::MAX, &t.color));
        }
        let mut enc = self.dev.create_command_encoder(&Default::default());
        for (f, tex) in srcs {
            if !self.backups.get(&f).is_some_and(|b| (b.width(), b.height()) == (tex.width(), tex.height())) {
                let b = self.dev.create_texture(&wgpu::TextureDescriptor {
                    label: Some("ssx3-gfx target backup"),
                    size: wgpu::Extent3d { width: tex.width(), height: tex.height(), depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: tex.format(),
                    usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
                    view_formats: &[],
                });
                self.backups.insert(f, b);
            }
            copy_whole(&mut enc, tex, &self.backups[&f]);
        }
        self.queue.submit(std::iter::once(enc.finish()));
    }

    /// Put back what `save_targets` saved (targets of the same size).
    fn restore_targets(&mut self) {
        let mut enc = self.dev.create_command_encoder(&Default::default());
        let mut dsts: Vec<(u32, &wgpu::Texture)> = self.rts.iter().map(|(&f, r)| (f, &r.color)).collect();
        if let Some(t) = self.targets.as_ref() {
            dsts.push((u32::MAX, &t.color));
        }
        for (f, tex) in dsts {
            if let Some(b) = self.backups.get(&f).filter(|b| (b.width(), b.height()) == (tex.width(), tex.height())) {
                copy_whole(&mut enc, b, tex);
            }
        }
        self.queue.submit(std::iter::once(enc.finish()));
    }

    /// The build state, moved out of the renderer (see `Build`).
    fn take_build(&mut self) -> Build {
        Build {
            gs: std::mem::take(&mut self.gs),
            vu: std::mem::replace(&mut self.vu, Box::new([0; 16384])),
            stats: self.stats,
            phase1_end: self.phase1_end,
            display_fbp: self.display_fbp,
            zscale: self.zscale,
            hud_squeeze: self.hud_squeeze,
            frame: self.stats.frames,
            subst: self.subst.clone(),
            model_subst: self.model_subst.clone(),
        }
    }

    fn put_build(&mut self, b: Build) {
        self.gs = b.gs;
        self.vu = b.vu;
        self.stats = b.stats;
        self.phase1_end = b.phase1_end;
        self.display_fbp = b.display_fbp;
        self.zscale = b.zscale;
    }

    fn render_one(&mut self, built: &Built, mem: &(dyn Mem + Sync)) {
        if self.slot.is_none() {
            self.stats.frames += 1;
        }
        let t_start = web_time::Instant::now();
        let mut b = self.take_build();
        Build::run(&mut b, built, mem);
        self.put_build(b);
        self.timing.1[0] += t_start.elapsed().as_secs_f64() * 1e3;
        self.draw_built(mem);
    }

    /// Draw the geometry built last (`Build::run`).
    fn draw_built(&mut self, mem: &dyn Mem) {
        let t_built = web_time::Instant::now();
        self.timing.0 += 1;
        let p0 = (self.gs.prims, self.gs.dropped, self.gs.unknown_tex);
        let (w, h) = self.gs.size;
        let (pw, ph) = self.out_size.unwrap_or((w * self.scale, h * self.scale));
        self.fxy = (pw as f32 / w as f32, ph as f32 / h as f32);
        self.rt_scale = (self.fxy.0.max(self.fxy.1).round() as u32).clamp(1, 8);
        self.ensure_targets(pw, ph);
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
        let vbuf = grow_buffer(&self.dev, &mut self.vbuf, vbytes.len() as u64, wgpu::BufferUsages::VERTEX, "ssx3-gfx verts");
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
            // Logical (GS pixel) size of the target; the GPU target is
            // `scale` times that.
            let (tw, th) = if st.target == self.gs.main_fbp {
                (w, h)
            } else {
                match self.gs.rts.get(&st.target).filter(|_| self.rts.contains_key(&st.target)) {
                    Some(r) => (r.w, r.h),
                    None => continue,
                }
            };
            let sc = st.ctx.scissor;
            let (x0, x1, y0, y1) = ((sc & 0x7ff) as u32, ((sc >> 16) & 0x7ff) as u32, ((sc >> 32) & 0x7ff) as u32, ((sc >> 48) & 0x7ff) as u32);
            // In target pixels.
            let (fx, fy) = if st.target == self.gs.main_fbp { self.fxy } else { self.rt_fxy(st.target) };
            let px = |v: u32, f: f32| (v as f32 * f).round() as u32;
            let scissor = (x0 <= x1 && y0 <= y1 && x0 < tw && y0 < th).then(|| {
                let (a, b) = (px(x0, fx), px(y0, fy));
                [a, b, px((x1 + 1).min(tw), fx).max(a + 1) - a, px((y1 + 1).min(th), fy).max(b + 1) - b]
            });
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
        let t_prepared = web_time::Instant::now();
        self.timing.1[1] += (t_prepared - t_built).as_secs_f64() * 1e3;
        let ubuf = grow_buffer(&self.dev, &mut self.ubuf, ubytes.len() as u64, wgpu::BufferUsages::UNIFORM, "ssx3-gfx uniforms");
        self.queue.write_buffer(&ubuf, 0, &ubytes);
        let snap = self.targets.as_ref().expect("targets").snapshot.create_view(&Default::default());
        let zsnap = self.targets.as_ref().expect("targets").zsnap.create_view(&Default::default());
        let groups: Vec<wgpu::BindGroup> = binds
            .iter()
            .map(|(view, sk)| {
                let aniso = self.aniso;
                let samp = self.samplers.entry(*sk).or_insert_with(|| draw::sampler(&self.dev, *sk, aniso)).clone();
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
        // One image per sub-frame (see `render`); this one's slot.
        let slots = self.nslots.max(1);
        let slot = self.slot.unwrap_or(slots - 1);
        if self.present.len() != slots || self.present.first().is_some_and(|p| (p.width(), p.height()) != (t.w, t.h)) {
            self.present = (0..slots)
                .map(|_| {
                    self.dev.create_texture(&wgpu::TextureDescriptor {
                        label: Some("ssx3-gfx present"),
                        size: wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
                        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
                    })
                })
                .collect();
        }
        enc.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo { texture: shown, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyTextureInfo { texture: &self.present[slot], mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
        );
        // Depth after phase 1, read back for the light queries.
        let depth_buf = (self.want_depth && self.slot.is_none()).then(|| {
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
        let sub_idx = self.queue.submit(std::iter::once(enc.finish()));
        let t_sub = web_time::Instant::now();
        self.timing.1[2] += (t_sub - t_prepared).as_secs_f64() * 1e3;
        if let Some(buf) = depth_buf {
            // The previous frame's depth (its readback has had a frame to
            // complete): the light queries see depth one frame late, and
            // the CPU does not wait for this frame's GPU work.
            if let Some((prev, prev_idx, pw, ph, zs, lw, lh, mapped)) = self.pending_depth.take() {
                let slice = prev.slice(..);
                if cfg!(target_arch = "wasm32") {
                    let _ = self.dev.poll(wgpu::PollType::Poll);
                } else {
                    let _ = self.dev.poll(wgpu::PollType::Wait { submission_index: Some(prev_idx), timeout: None });
                }
                // (Not mapped yet in the browser: the queries keep the
                // depth before.)
                if let Some(v) = mapped.load(std::sync::atomic::Ordering::Acquire).then(|| slice.get_mapped_range()).and_then(|r| r.ok()) {
                    // In GS pixels: the sample at each pixel's centre,
                    // converted when a query reads it.
                    let raw = v.to_vec();
                    let row = (pw * 4).next_multiple_of(256) as usize;
                    let zs = zs as f64;
                    let z = move |x: u32, y: u32| {
                        let py = (((y as f32 + 0.5) * ph as f32 / lh as f32) as usize).min(ph as usize - 1);
                        let pxl = (((x as f32 + 0.5) * pw as f32 / lw as f32) as usize).min(pw as usize - 1);
                        let o = py * row + 4 * pxl;
                        let d = f32::from_le_bytes([raw[o], raw[o + 1], raw[o + 2], raw[o + 3]]);
                        (d as f64 * zs).round().min(zs - 1.0) as u32
                    };
                    *self.shared.depth.lock().unwrap() = Some(ssx3_ps2::frame_sink::Depth { w: lw, h: lh, z: Box::new(z) });
                }
            }
            let mapped = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let m = mapped.clone();
            buf.slice(..).map_async(wgpu::MapMode::Read, move |r| m.store(r.is_ok(), std::sync::atomic::Ordering::Release));
            self.pending_depth = Some((buf, sub_idx, t.w, t.h, self.zscale, w, h, mapped));
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

/// Split a GIF stream into packets: tags up to and including one with EOP,
/// with their data.
fn gif_packets(q: &[u128]) -> Vec<Vec<u128>> {
    let mut out = Vec::new();
    let (mut i, mut start) = (0, 0);
    while i < q.len() {
        let lo = q[i] as u64;
        let nloop = (lo & 0x7fff) as usize;
        let eop = (lo >> 15) & 1 != 0;
        let nreg = match (lo >> 60) as usize {
            0 => 16,
            n => n,
        };
        i += 1 + match (lo >> 58) & 3 {
            0 => nloop * nreg,
            1 => (nloop * nreg).div_ceil(2),
            _ => nloop,
        };
        if eop || i >= q.len() {
            out.push(q[start..i.min(q.len())].to_vec());
            start = i;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::gif_packets;

    #[test]
    fn gif_packets_end_at_eop() {
        // A+D tag (2 regs, no EOP), its 2 qwords; IMAGE tag (3 qwords, EOP),
        // its data; a second packet: A+D tag with EOP and 1 qword.
        let ad = |n: u128, eop: bool| n | (eop as u128) << 15 | 1 << 60 | 0xe << 64;
        let image = |n: u128| n | 1 << 15 | 2 << 58;
        let q = vec![ad(2, false), 1, 2, image(3), 3, 4, 5, ad(1, true), 6];
        let p = gif_packets(&q);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].len(), 7);
        assert_eq!(p[1], vec![ad(1, true), 6]);
    }
}

/// One chunk's result: (event index, GIF output) per call with a native
/// program, and the VU memory after the chunk.
type VuChunk = (Vec<(usize, Vec<u128>)>, Box<[u8; 16384]>);

/// A frame's VU1 calls split into chunks run by worker threads. Chunks
/// start only where a draw of a different program begins (programs pass
/// state within a draw, and terrain patches across draws), from the VU
/// memory with every earlier call's inputs applied. Checked against a
/// sequential run with `SSX3_GFX_VUCHECK`; `SSX3_GFX_SEQ_VU=1` forces one
/// chunk.
struct VuPlan<'a> {
    vu0: Box<[u8; 16384]>,
    calls: Vec<(usize, &'a ssx3_render::vif::VuCall, Option<u64>)>,
    bounds: Vec<usize>,
    starts: Vec<std::sync::Mutex<Option<Box<[u8; 16384]>>>>,
    done: Vec<(std::sync::Mutex<Option<VuChunk>>, std::sync::Condvar)>,
    /// Event index → chunk.
    chunk_of: HashMap<usize, usize>,
    next: std::sync::atomic::AtomicUsize,
    threads: usize,
    /// VU header substitutions (an in-between image, see `interp`), and
    /// mesh model matrix ones.
    subst: Option<Arc<HashMap<[u32; 16], [u32; 16]>>>,
    model_subst: Option<Arc<HashMap<[u32; 16], [u32; 16]>>>,
}

impl VuPlan<'_> {
    fn run(chunk: &[(usize, &ssx3_render::vif::VuCall, Option<u64>)], mut mem: Box<[u8; 16384]>, subst: Option<&HashMap<[u32; 16], [u32; 16]>>, model_subst: Option<&HashMap<[u32; 16], [u32; 16]>>) -> VuChunk {
        let mut outs = Vec::with_capacity(chunk.len());
        for &(i, c, hash) in chunk {
            for &(a, v) in &c.input {
                let a = (a as usize & 1023) * 16;
                mem[a..a + 16].copy_from_slice(&v.to_le_bytes());
            }
            let (Some(entry), Some(hh)) = (c.entry, hash) else { continue };
            if let Some(s) = subst {
                interp::substitute(&mut mem[..64], s);
            }
            if let Some(s) = model_subst.filter(|_| hh == ssx3_render::mat3::HASH && ssx3_render::mat3::mesh_matrix_use(entry, c.itop) == Some(true)) {
                let top = (c.top as usize & 1023) * 16;
                if top + 64 <= mem.len() {
                    interp::substitute(&mut mem[top..top + 64], s);
                }
            }
            let mut out: Vec<u128> = Vec::new();
            if ssx3_render::run_native(hh, entry, c.top, c.itop, &mut mem, &mut |p: &[u128]| out.extend_from_slice(p)) {
                outs.push((i, out));
            }
        }
        (outs, mem)
    }

    /// Worker: take chunks until none are left.
    fn work(&self) {
        loop {
            let j = self.next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let Some(start) = self.starts.get(j) else { break };
            let st = start.lock().unwrap().take().expect("chunk");
            let e = self.bounds.get(j + 1).copied().unwrap_or(self.calls.len());
            let r = Self::run(&self.calls[self.bounds[j]..e], st, self.subst.as_deref(), self.model_subst.as_deref());
            let (slot, cv) = &self.done[j];
            *slot.lock().unwrap() = Some(r);
            cv.notify_all();
        }
    }

    /// Whether the chunks go to the installed [`VuOffload`] (the browser's
    /// helper workers): not without one, nor for an in-between image (its
    /// substitutions stay here).
    fn offloadable(&self) -> bool {
        OFFLOAD.get().is_some() && self.subst.is_none() && self.model_subst.is_none() && self.bounds.len() >= 2
    }

    /// Every chunk, encoded (takes the start images).
    fn jobs(&self) -> Vec<Vec<u8>> {
        (0..self.bounds.len())
            .map(|j| {
                let start = self.starts[j].lock().unwrap().take().expect("chunk");
                let e = self.bounds.get(j + 1).copied().unwrap_or(self.calls.len());
                ssx3_render::vujob::encode(&start, &self.calls[self.bounds[j]..e])
            })
            .collect()
    }

    /// Run every chunk through the offload, or take the results of the
    /// same plan started earlier (`PREFETCH`); false (nothing done) when
    /// not offloadable.
    fn offload(&self) -> bool {
        if !self.offloadable() {
            return false;
        }
        let pre = PREFETCH.lock().unwrap().take_if(|(n, _)| *n == self.bounds.len());
        let results = match pre {
            Some((_, collect)) => collect(),
            None => (OFFLOAD.get().unwrap().0)(self.jobs())(),
        };
        for (j, r) in results.into_iter().enumerate() {
            *self.done[j].0.lock().unwrap() = Some(ssx3_render::vujob::decode(&r));
        }
        true
    }

    fn wait(&self, j: usize) -> VuChunk {
        let (slot, cv) = &self.done[j];
        let mut g = slot.lock().unwrap();
        loop {
            if let Some(r) = g.take() {
                return r;
            }
            g = cv.wait(g).unwrap();
        }
    }
}

/// The consuming side of a `VuPlan`: chunks merged in order as calls need
/// them.
#[derive(Default)]
struct VuReceiver {
    merged: usize,
    outs: HashMap<usize, Vec<u128>>,
    last_mem: Option<Box<[u8; 16384]>>,
    /// `SSX3_GFX_VUCHECK`: every output, for the comparison in `finish`.
    seen: Option<HashMap<usize, Vec<u128>>>,
}

impl VuReceiver {
    fn merge_to(&mut self, plan: &VuPlan, j: usize) {
        while self.merged <= j && self.merged < plan.done.len() {
            let (outs, mem) = plan.wait(self.merged);
            self.outs.extend(outs);
            self.last_mem = Some(mem);
            self.merged += 1;
        }
    }

    /// The GIF output of the call at event `i` (None: no native program).
    fn take(&mut self, plan: &VuPlan, i: usize) -> Option<Vec<u128>> {
        let j = *plan.chunk_of.get(&i)?;
        self.merge_to(plan, j);
        let out = self.outs.remove(&i);
        if plan.bounds.len() > 1 && debug::get().vucheck {
            if let Some(o) = &out {
                self.seen.get_or_insert_with(HashMap::new).insert(i, o.clone());
            }
        }
        out
    }

    /// Wait for every chunk; `vu` becomes the memory after the last.
    fn finish(&mut self, plan: &VuPlan, vu: &mut Box<[u8; 16384]>) {
        let check = debug::get().vucheck && plan.bounds.len() > 1;
        let mut all = if check { Some(self.seen.take().unwrap_or_default()) } else { None };
        let n = plan.done.len();
        while self.merged < n {
            let (outs, mem) = plan.wait(self.merged);
            match all.as_mut() {
                Some(a) => a.extend(outs),
                None => {}
            }
            self.last_mem = Some(mem);
            self.merged += 1;
        }
        if let Some(m) = self.last_mem.take() {
            *vu = m;
        }
        if let Some(all) = all {
            // Debug: which calls differ from a sequential run (only the calls
            // not yet consumed are compared).
            let (seq, _) = VuPlan::run(&plan.calls, plan.vu0.clone(), plan.subst.as_deref(), plan.model_subst.as_deref());
            let bad = seq.iter().filter(|(i, out)| all.get(i).is_some_and(|o| o != out)).count();
            eprintln!("[vucheck] {bad} of {} calls differ from a sequential run ({} chunks)", seq.len(), n);
        }
    }
}

/// Debug: `SSX3_INTERP_T` forces the in-between images' t.
fn debug_t() -> Option<f32> {
    static T: std::sync::OnceLock<Option<f32>> = std::sync::OnceLock::new();
    *T.get_or_init(|| std::env::var("SSX3_INTERP_T").ok().and_then(|v| v.parse().ok()))
}

/// Copy a whole texture (same size and format).
fn copy_whole(enc: &mut wgpu::CommandEncoder, from: &wgpu::Texture, to: &wgpu::Texture) {
    enc.copy_texture_to_texture(
        wgpu::TexelCopyTextureInfo { texture: from, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        wgpu::TexelCopyTextureInfo { texture: to, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        wgpu::Extent3d { width: from.width(), height: from.height(), depth_or_array_layers: 1 },
    );
}

/// A frame's matrix records with the draws' state keys (interpolation).
fn records(built: &Built) -> interp::Records {
    let fr = &built.frame;
    let draws: Vec<&frame::Draw> = fr.draws().collect();
    // Each draw as (state and textures, n-th such draw).
    let mut nth: HashMap<u64, u32> = HashMap::new();
    let keys: Vec<(u64, u32)> = draws
        .iter()
        .map(|d| {
            let alt = hash(&(d.state.raw, d.tex, d.state.layer));
            let n = nth.entry(alt).or_insert(0);
            *n += 1;
            (alt, *n - 1)
        })
        .collect();
    let mut users: Vec<Vec<(u64, u32)>> = vec![Vec::new(); fr.matrices.len()];
    for (d, key) in draws.iter().zip(&keys) {
        for m in [d.matrix as i32, d.header_matrix] {
            if let Some(a) = usize::try_from(m).ok().and_then(|m| users.get_mut(m)) {
                if !a.contains(key) {
                    a.push(*key);
                }
            }
        }
    }
    // Mesh setups' model matrices: TOP qw0-3 of their input.
    let mut models = HashMap::new();
    let mut setups: HashMap<usize, u32> = HashMap::new();
    let mut cur = None;
    for (owner, ev) in &built.events {
        if let Owner::Prelude(n) | Owner::Payload(n) = owner {
            cur = Some(*n);
        }
        let (Event::Call(c), Some(n)) = (ev, cur) else { continue };
        let (Some(e), Some(d)) = (c.entry, draws.get(n)) else { continue };
        if program(d.state.material) != Some(ssx3_render::mat3::HASH) || ssx3_render::mat3::mesh_matrix_use(e, c.itop) != Some(true) {
            continue;
        }
        let top = c.top as u16 & 1023;
        let mut w = [0u32; 16];
        let mut got = 0;
        for &(a, v) in &c.input {
            let k = (a & 1023).wrapping_sub(top);
            if k < 4 {
                for l in 0..4 {
                    w[k as usize * 4 + l] = (v >> (32 * l)) as u32;
                }
                got += 1;
            }
        }
        let s = setups.entry(n).or_insert(0);
        if got == 4 {
            models.insert((keys[n], *s), w);
        }
        *s += 1;
    }
    interp::Records { records: fr.matrices.iter().copied().zip(users).collect(), models }
}


/// What building one image's geometry from a frame uses and changes: the
/// GS state the frame starts from, VU memory, counters, and the frame's
/// facts the drawing needs. In-between images (interpolation) each build
/// from a copy, on their own threads, while the frame's own builds.
#[derive(Clone)]
struct Build {
    gs: gs::Gs,
    vu: Box<[u8; 16384]>,
    stats: Stats,
    phase1_end: usize,
    display_fbp: Option<u32>,
    zscale: f32,
    hud_squeeze: f32,
    /// The frame number (debug switches).
    frame: u64,
    subst: Option<Arc<HashMap<[u32; 16], [u32; 16]>>>,
    model_subst: Option<Arc<HashMap<[u32; 16], [u32; 16]>>>,
}

impl Build {
    fn run(b: &mut Build, built: &Built, mem: &(dyn Mem + Sync)) {
        let fr = &built.frame;
        let d = &fr.display;
        let (w, h) = (d.fb_width.clamp(64, 2048), d.fb_height.clamp(64, 2048));
        // FRAME.FBP is in 2048-word units, gm's draw_fbp likewise.
        b.gs.begin((w, h), d.draw_fbp, d.zbp);
        b.gs.hud = false;
        b.gs.hud_squeeze = b.hud_squeeze;
        b.zscale = match d.zpsm {
            0x30 => 4294967296.0,
            0x31 => 16777216.0,
            _ => 65536.0,
        };
        // Resident textures by their permanent VRAM address.
        let texman = frame::word(mem, fr.gm + frame::gm::TEXMAN);
        b.gs.tex_map.clear();
        for id in 0..tex::tm::MAX as u16 {
            let info = tex::info(mem, texman, id);
            if info != 0 {
                let tbp = frame::word(mem, info + tex::ti::TBP) as i32;
                // Never the frame, display or Z buffers (self-sampling post
                // passes address them; those become native passes).
                if tbp >= 0 && !in_buffers(tbp as u32, d) {
                    b.gs.tex_map.insert(tbp as u32, info);
                }
            }
        }
        let draws: Vec<&frame::Draw> = fr.draws().collect();
        // The PATH3 chains (GifBuff), one per phase, kicked with PATH3
        // masked (the kick thread writes MSKPATH3 1 into the VIF1 FIFO
        // first). As on the GS (`ssx3_ps2::gs::Mtgs::set_path3_mask`), each
        // MSKPATH3 0 in the VIF stream releases one GIF packet (texture
        // uploads in the gaps between draws); what is left drains when the
        // phase's VIF stream ends unmasked (the post passes, before the
        // HUD of phase 2 is drawn).
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
        let tracing = dbg.trace == Some(b.frame);
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
        label(&mut b.gs, "flip".into());
        b.gs.gif(&flip[split..]);
        b.display_fbp = (split > 0).then(|| d.draw_fbp + (d.fb_width.div_ceil(64) * d.fb_height.div_ceil(32)).max(1));
        let mode = dbg.path3;
        let mut p3: std::collections::VecDeque<Vec<u128>> = std::collections::VecDeque::new();
        if mode & 1 != 0 {
            p3.extend(gif_packets(&p1));
        }
        let mut p3_masked = true;
        let mut phase2_started = false;
        let mut cur: Option<usize> = None;
        // Debug: SSX3_GFX_SKIP=n,m,... drops those draws' payloads.
        // (`a-b` ranges too.)
        let skip = &dbg.skip;
        // The VU1 calls run on worker threads while this loop parses their
        // output in order (`VuPlan`).
        let plan = Build::plan_vu_calls(b, built, &draws);
        let mut vu = VuReceiver::default();
        std::thread::scope(|sc| {
        // The helper workers (the browser) take every chunk up front;
        // without threads, this thread does.
        if plan.offload() {
        } else if cfg!(target_arch = "wasm32") {
            plan.work();
        } else {
            for _ in 0..plan.threads {
                sc.spawn(|| plan.work());
            }
        }
        for (i, (owner, ev)) in built.events.iter().enumerate() {
            if matches!(owner, Owner::Payload(n) if skip.contains(n)) {
                continue;
            }
            if !phase2_started && i >= built.phase2 {
                phase2_started = true;
                label(&mut b.gs, "path3".into());
                if !p3_masked {
                    while let Some(pk) = p3.pop_front() {
                        let hud = std::mem::replace(&mut b.gs.hud, false);
                    b.gs.gif(&pk);
                    b.gs.hud = hud;
                    }
                }
                b.phase1_end = b.gs.batches.len();
                p3_masked = true;
                if mode & 2 != 0 {
                    p3.extend(gif_packets(&p2));
                }
            }
            let n = match owner {
                Owner::Prelude(n) | Owner::Payload(n) => Some(*n),
                _ => None,
            };
            if let Some(n) = n {
                if cur != Some(n) {
                    cur = Some(n);
                    b.gs.hud = draws.get(n).is_some_and(|d| d.state.layer >= 9);
                    if let Some(d) = draws.get(n) {
                        label(&mut b.gs, format!("draw{n} {:?} layer {} tex {:?}", d.state.material, d.state.layer, d.tex));
                    }
                }
                // In a draw's payload its textures are bound to the TBPs its
                // prelude wrote.
                if let (Owner::Payload(_), Some(dr)) = (owner, draws.get(n)) {
                    for (ctx, id) in dr.tex.iter().enumerate() {
                        if let Some(id) = id {
                            let info = tex::info(mem, texman, *id);
                            if info != 0 {
                                let tbp = (b.gs.ctx[ctx].tex0 & 0x3fff) as u32;
                                b.gs.tex_map.insert(tbp, info);
                            }
                        }
                    }
                }
            }
            match ev {
                Event::Call(_) => {
                    b.stats.vu_calls += 1;
                    match vu.take(&plan, i) {
                        Some(out) => {
                            b.stats.vu_native += 1;
                            b.gs.gif(&out);
                        }
                        None => b.stats.vu_missing += 1,
                    }
                }
                Event::Direct(dd) => b.gs.gif(&dd.qwords),
                Event::MaskPath3(m) => {
                    if !m {
                        if let Some(pk) = p3.pop_front() {
                            label(&mut b.gs, "path3".into());
                            let hud = std::mem::replace(&mut b.gs.hud, false);
                    b.gs.gif(&pk);
                    b.gs.hud = hud;
                        }
                    }
                    p3_masked = *m;
                }
                _ => {}
            }
        }
        if !phase2_started {
            b.phase1_end = b.gs.batches.len();
            if mode & 2 != 0 {
                p3.extend(gif_packets(&p2));
            }
        }
        vu.finish(&plan, &mut b.vu);
        });
        // Whatever is left runs before the next frame.
        label(&mut b.gs, "path3".into());
        while let Some(pk) = p3.pop_front() {
            let hud = std::mem::replace(&mut b.gs.hud, false);
                    b.gs.gif(&pk);
                    b.gs.hud = hud;
        }
        if split > 0 {
            label(&mut b.gs, "flip-blend".into());
            b.gs.hud = false;
            b.gs.gif(&flip[..split]);
        }
        if tracing {
            b.gs.trace = None;
            let log = std::mem::take(&mut b.gs.trace_log);
            let path = &dbg.trace_out;
            let _ = std::fs::write(&path, log.join("\n"));
            eprintln!("[gfx] frame {} traced: {} lines -> {path}", b.frame, log.len());
        }
    }

    fn plan_vu_calls<'a>(b: &Build, built: &'a Built, draws: &[&frame::Draw]) -> VuPlan<'a> {
        // Each call's program: the material of the draw it belongs to.
        let mut calls: Vec<(usize, &ssx3_render::vif::VuCall, Option<u64>)> = Vec::new();
        // Calls that start a draw (chunks may only begin there: within a
        // draw, programs pass state from call to call).
        let mut draw_start: Vec<bool> = Vec::new();
        let mut last_draw = None;
        let (mut cur, mut hash) = (None, None);
        for (i, (owner, ev)) in built.events.iter().enumerate() {
            if let Owner::Prelude(n) | Owner::Payload(n) = owner {
                if cur != Some(*n) {
                    cur = Some(*n);
                    if let Some(p) = draws.get(*n).and_then(|d| program(d.state.material)) {
                        hash = Some(p);
                    }
                }
            }
            if let Event::Call(c) = ev {
                // (And not between two draws of the same program: terrain
                // patches share state across draws.)
                draw_start.push(cur != last_draw && calls.last().is_none_or(|l| l.2 != hash));
                last_draw = cur;
                calls.push((i, c, hash));
            }
        }
        static SEQ: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let seq = *SEQ.get_or_init(|| std::env::var_os("SSX3_GFX_SEQ_VU").is_some());
        let parallel = OFFLOAD.get().map(|o| o.1).unwrap_or_else(|| std::thread::available_parallelism().map_or(4, |n| n.get()));
        let threads = if seq || calls.len() < 64 { 1 } else { parallel.clamp(1, 8) };
        // About four chunks per thread, taken by whichever thread is free.
        let per = calls.len().div_ceil(threads * 4).max(1);
        // Chunk boundaries: the first draw start at or after each multiple
        // of `per`; the start images have every earlier call's inputs applied.
        let mut bounds: Vec<usize> = Vec::new();
        let mut starts = Vec::new();
        let mut img = b.vu.clone();
        for (k, (_, c, _)) in calls.iter().enumerate() {
            if k == 0 || (k >= bounds.last().unwrap() + per && draw_start[k] && threads > 1) {
                bounds.push(k);
                starts.push(std::sync::Mutex::new(Some(img.clone())));
            }
            if threads > 1 {
                for &(a, v) in &c.input {
                    let a = (a as usize & 1023) * 16;
                    img[a..a + 16].copy_from_slice(&v.to_le_bytes());
                }
            }
        }
        if bounds.is_empty() {
            bounds.push(0);
            starts.push(std::sync::Mutex::new(Some(b.vu.clone())));
        }
        let chunk_of: HashMap<usize, usize> = calls.iter().enumerate().map(|(k, c)| (c.0, bounds.partition_point(|&b| b <= k) - 1)).collect();
        let done = (0..bounds.len()).map(|_| (std::sync::Mutex::new(None), std::sync::Condvar::new())).collect();
        VuPlan { vu0: b.vu.clone(), calls, bounds, starts, done, chunk_of, next: std::sync::atomic::AtomicUsize::new(0), threads, subst: b.subst.clone(), model_subst: b.model_subst.clone() }
    }
}

/// Waits for the results of started VU1 chunks, in order.
pub type VuCollect = Box<dyn FnOnce() -> Vec<Vec<u8>> + Send>;

/// Starts encoded chunks of VU1 calls ([`ssx3_render::vujob`]) elsewhere
/// (the browser build's helper workers) and returns how to wait for them.
pub type VuOffload = Box<dyn Fn(Vec<Vec<u8>>) -> VuCollect + Send + Sync>;

static OFFLOAD: std::sync::OnceLock<(VuOffload, usize)> = std::sync::OnceLock::new();

/// The staged frame's started chunks (chunk count, results), for its
/// `VuPlan::offload` when it is drawn.
static PREFETCH: Mutex<Option<(usize, VuCollect)>> = Mutex::new(None);

/// A frame published but not drawn yet (`Inner::publish_staged`).
struct Staged {
    mem: Box<ssx3_ps2::frame_sink::DumpMem>,
    built: Built,
    carry: ssx3_render::frame::Carry,
    collect: Option<(usize, VuCollect)>,
}

/// Run one encoded job (what an offload does with each).
pub fn run_vu_job(job: &[u8]) -> Vec<u8> {
    ssx3_render::vujob::run(job)
}

/// Install the VU offload, which runs `workers` jobs at a time (the frame's
/// calls are split for that many).
pub fn set_vu_offload(f: VuOffload, workers: usize) {
    let _ = OFFLOAD.set((f, workers.max(1)));
}

/// A stable hash (interpolation keys).
fn hash<T: std::hash::Hash>(v: &T) -> u64 {
    use std::hash::Hasher;
    let mut h = std::hash::DefaultHasher::new();
    v.hash(&mut h);
    h.finish()
}

/// `slot`'s buffer if it holds `size` bytes, else a new one (twice as big as
/// needed, so it rarely grows again).
fn grow_buffer(dev: &wgpu::Device, slot: &mut Option<wgpu::Buffer>, size: u64, usage: wgpu::BufferUsages, label: &str) -> wgpu::Buffer {
    if let Some(b) = slot.as_ref().filter(|b| b.size() >= size) {
        return b.clone();
    }
    let b = dev.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: (size * 2).next_multiple_of(256),
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    *slot = Some(b.clone());
    b
}
