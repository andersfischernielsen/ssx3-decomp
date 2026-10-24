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
mod options;
pub mod pack;
mod post;
mod shadow;
pub mod tex;

pub use options::{Aa, Bloom, Options};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ssx3_render::chain::Mem;
use ssx3_render::frame::{self, Built, Material, Owner};
use ssx3_render::vif::Event;
use ssx3_render::vujob::CallOut;

// GPU buffers are uploaded as the host's bytes (little-endian words).
#[cfg(target_endian = "big")]
compile_error!("ssx3-gfx uploads u32 data as host bytes: little-endian targets only");

/// Statistics of the last frame and totals.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub frames: u64,
    pub vu_calls: u64,
    pub vu_native: u64,
    /// Calls with no native program (MSCNT, unimplemented entries).
    pub vu_missing: u64,
    /// Calls drawn by the GPU (`ssx3_render::gpu`).
    pub vu_gpu: u64,
    /// Draws captured at the game's draw functions (`ssx3_render::direct`).
    pub native_draws: u64,
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
    dev: wgpu::Device,
    queue: wgpu::Queue,
    pipes: draw::Pipelines,
    targets: Option<Targets>,
    rts: HashMap<u32, RtTarget>,
    gs: gs::Gs,
    vu: Box<[u8; 16384]>,
    texcache: HashMap<tex::TexKey, CachedTex>,
    /// Texture keys reused while their memory is not written.
    texkeys: tex::KeyCache,
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
    /// Modern graphics options (`Options`; the default is the original).
    opts: Options,
    /// The renderer's own passes (created on first use).
    post: Option<post::Post>,
    /// The shadow map (options: shadows).
    shadow: Option<shadow::Shadow>,
    /// The game's key light (towards it), eased from frame to frame.
    key_light: [f32; 3],
    /// The sun's direction where the game last drew it (the lens flare's
    /// query), eased from frame to frame.
    flare_sun: Option<[f32; 3]>,
    /// The game's live light (the lit meshes': ambient, key colour; ×255),
    /// eased from frame to frame (`gs::Gs::key_ambient`).
    live_ambient: [f32; 3],
    live_colour: [f32; 3],
    /// The GraphicsMan of the frame being drawn (TexMan for the cut-outs).
    frame_gm: u32,
    /// The world's shadow casters of the frame being drawn.
    world_casters: Option<std::sync::Arc<ssx3_render::world_casters::Casters>>,
    /// Seconds since the renderer started (animated effects).
    t0: web_time::Instant,
    /// Batches of VIF phase 1 (the 3D scene) in this frame.
    phase1_end: usize,
    /// Keep the depth after phase 1 (`depth()`, native mode).
    pub want_depth: bool,
    /// This frame's depth readback in flight (buffer, size, ZMAX + 1).
    /// (The flag: mapped; the browser cannot wait for it.)
    pending_depth: Option<(
        wgpu::Buffer,
        wgpu::SubmissionIndex,
        u32,
        u32,
        f32,
        u32,
        u32,
        Arc<std::sync::atomic::AtomicBool>,
        u64,
    )>,
    /// Frames received by `render_built` (the next frame's number).
    frames_in: u64,
    /// ZMAX + 1 of the frame's Z buffer (depth is stored as Z / it).
    zscale: f32,
    /// Frames timed and the time (ms) in build, prepare, encode+submit,
    /// depth readback.
    timing: (u64, [f64; 4]),
    /// The image on screen, for presenting (sRGB view allowed).
    present: Vec<wgpu::Texture>,
    /// Vertex and uniform buffers, kept across frames (grown as needed).
    vbuf: Option<wgpu::Buffer>,
    ubuf: Option<wgpu::Buffer>,
    /// GPU-computed draws: call uniforms, inputs, indices.
    gbuf: [Option<wgpu::Buffer>; 4],
    /// The mesh arena on the GPU (`gs::MeshArena`) and the arena epoch it
    /// holds.
    arena_buf: Option<wgpu::Buffer>,
    arena_epoch: u64,
}

/// The renderer; clone the handle to read images from another thread.
#[derive(Clone)]
pub struct Renderer(pub Arc<Mutex<Inner>>, Arc<Shared>);

/// A frame handed to the render thread.
type Job = ssx3_ps2::frame_sink::Published;

/// What the render thread shares with the game and the window without
/// holding the renderer: the latest depth for the light queries and the
/// latest image on screen.
#[derive(Default)]
pub struct Shared {
    tx: Mutex<Option<std::sync::mpsc::SyncSender<Job>>>,
    /// Frames sent and not yet rendered.
    pending: Mutex<usize>,
    idle: std::sync::Condvar,
    /// The latest depth readback and the number of the frame it is from
    /// (frames counted as `render_built` receives them).
    depth: Mutex<Option<(u64, ssx3_ps2::frame_sink::Depth)>>,
    depth_ready: std::sync::Condvar,
    /// The latest frame's images (in-between ones first) and when they
    /// were ready.
    present: Mutex<Option<(Vec<wgpu::Texture>, web_time::Instant)>>,
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
    [d.draw_fbp, d.draw_fbp + size, d.zbp]
        .iter()
        .any(|&b| page >= b && page < b + size)
}

impl Renderer {
    pub fn new(dev: wgpu::Device, queue: wgpu::Queue) -> Renderer {
        let pipes = draw::Pipelines::new(&dev);
        let white = dev
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("ssx3-gfx white"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
            .create_view(&Default::default());
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: white.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &[255, 255, 255, 128],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        let shared = Arc::new(Shared::default());
        Renderer(
            Arc::new(Mutex::new(Inner {
                shared: shared.clone(),
                dev,
                queue,
                pipes,
                targets: None,
                rts: HashMap::new(),
                gs: gs::Gs::new(),
                vu: Box::new([0; 16384]),
                texcache: HashMap::new(),
                texkeys: Default::default(),
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
                opts: Options::default(),
                post: None,
                shadow: None,
                key_light: [0.0, 0.6, 0.8],
                world_casters: None,
                frame_gm: 0,
                live_ambient: [110.0, 112.0, 120.0],
                live_colour: [90.0, 85.0, 70.0],
                flare_sun: None,
                t0: web_time::Instant::now(),
                phase1_end: usize::MAX,
                want_depth: false,
                pending_depth: None,
                frames_in: 0,
                zscale: 16777216.0,
                timing: (0, [0.0; 4]),
                present: Vec::new(),
                staged: None,
                vbuf: None,
                gbuf: [None, None, None, None],
                arena_buf: None,
                arena_epoch: u64::MAX,
                ubuf: None,
            })),
            shared,
        )
    }

    /// Render on a thread of its own from now on: frames are rendered while
    /// the game goes on (at most one frame queued; the game waits if the
    /// renderer falls further behind).
    pub fn start_worker(&self) {
        let (tx, rx) = std::sync::mpsc::sync_channel::<Job>(1);
        *self.1.tx.lock().unwrap() = Some(tx);
        let (inner, shared) = (self.0.clone(), self.1.clone());
        std::thread::Builder::new()
            .name("ssx3-gfx".into())
            .spawn(move || {
                while let Ok(f) = rx.recv() {
                    inner
                        .lock()
                        .unwrap()
                        .render_built(&f.built, &f.carry, &f.mem);
                    drop(f);
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
        let g = self.1.present.lock().unwrap();
        let t = g.as_ref()?.0.last()?;
        let view = t.create_view(&wgpu::TextureViewDescriptor {
            format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
            ..Default::default()
        });
        Some((view, t.width(), t.height()))
    }

    /// The modern graphics options.
    pub fn set_options(&self, o: Options) {
        let mut g = self.0.lock().unwrap();
        if g.opts.mipmaps != o.mipmaps {
            // Textures are made with or without mipmaps.
            g.texcache.clear();
        }
        g.opts = o;
        ssx3_render::bezier::gpu::set_tessellation(o.terrain);
        ssx3_render::world_casters::set_enabled(o.shadows);
    }

    /// Anisotropic filtering: 1 (off) to 16 (the default).
    pub fn set_anisotropy(&self, n: u16) {
        let mut g = self.0.lock().unwrap();
        g.aniso = n.clamp(1, 16);
        g.samplers.clear();
    }

    /// The last frame's images on screen, read back (in-between ones
    /// first; debug).
    pub fn read_presents(&self) -> Vec<(u32, u32, Vec<u32>)> {
        self.flush();
        let g = self.0.lock().unwrap();
        g.present
            .iter()
            .map(|p| g.read_texture(p, p.width(), p.height()))
            .collect()
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
        g.rts
            .iter()
            .map(|(&fbp, r)| {
                let (w, h, px) = g.read_texture(&r.color, r.w, r.h);
                (fbp, w, h, px)
            })
            .collect()
    }

    pub fn stats(&self) -> Stats {
        self.0.lock().unwrap().stats
    }

    /// Primitives drawn into other frame buffers (debug).
    pub fn dropped_fbp(&self) -> Vec<((u32, u32, u32), u64)> {
        let mut v: Vec<_> = self
            .0
            .lock()
            .unwrap()
            .gs
            .dropped_fbp
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        v.sort_by_key(|x| std::cmp::Reverse(x.1));
        v
    }
}

impl ssx3_ps2::frame_sink::FrameSink for Renderer {
    fn frame(&mut self, f: ssx3_ps2::frame_sink::Published) {
        let tx = self.1.tx.lock().unwrap().clone();
        match tx {
            Some(tx) => {
                *self.1.pending.lock().unwrap() += 1;
                if tx.send(f).is_err() {
                    *self.1.pending.lock().unwrap() -= 1;
                }
            }
            None => {
                let mut g = self.0.lock().unwrap();
                if OFFLOAD.get().is_some() {
                    g.publish_staged(f);
                } else {
                    g.finish_staged();
                    g.render_built(&f.built, &f.carry, &f.mem);
                }
            }
        }
    }

    fn depth(&mut self) -> Option<ssx3_ps2::frame_sink::Depth> {
        self.1.depth.lock().unwrap().take().map(|d| d.1)
    }

    /// Frame `frame`'s depth, waiting for the render thread to read it back
    /// (it does while rendering the frame after it): the light queries then
    /// see the same depth whatever the threads' timing. Without a render
    /// thread (the browser), or after 100 ms, the latest depth instead.
    fn depth_of(&mut self, frame: u64) -> Option<ssx3_ps2::frame_sink::Depth> {
        if self.1.tx.lock().unwrap().is_none() {
            return self.depth();
        }
        let deadline = web_time::Instant::now() + std::time::Duration::from_millis(100);
        let mut g = self.1.depth.lock().unwrap();
        loop {
            match &*g {
                Some((f, _)) if *f == frame => return g.take().map(|d| d.1),
                Some((f, _)) if *f > frame => return g.take().map(|d| d.1),
                _ => {}
            }
            let now = web_time::Instant::now();
            if now >= deadline {
                return g.take().map(|d| d.1);
            }
            g = self
                .1
                .depth_ready
                .wait_timeout(g, deadline - now)
                .unwrap()
                .0;
        }
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
                    | extra,
                view_formats: &[],
            })
        };
        let color = mk(
            "ssx3-gfx colour",
            draw::COLOR_FORMAT,
            wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
        );
        let snapshot = mk(
            "ssx3-gfx snapshot",
            draw::COLOR_FORMAT,
            wgpu::TextureUsages::COPY_DST,
        );
        let depth = mk(
            "ssx3-gfx depth",
            draw::DEPTH_FORMAT,
            wgpu::TextureUsages::COPY_SRC,
        );
        let zsnap = mk(
            "ssx3-gfx depth snapshot",
            draw::DEPTH_FORMAT,
            wgpu::TextureUsages::COPY_DST,
        );
        let qdepth = mk(
            "ssx3-gfx query depth",
            draw::DEPTH_FORMAT,
            wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
        );
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
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &dv,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }
        self.queue.submit(std::iter::once(enc.finish()));
        self.targets = Some(Targets {
            w,
            h,
            color,
            depth,
            snapshot,
            zsnap,
            qdepth,
        });
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
            let (pw, ph) = (
                (rt.w as f32 * fx).round() as u32,
                (rt.h as f32 * fy).round() as u32,
            );
            if self.rts.get(&fbp).is_some_and(|t| t.w == pw && t.h == ph) {
                continue;
            }
            let mk = |format, extra| {
                self.dev.create_texture(&wgpu::TextureDescriptor {
                    label: Some("ssx3-gfx rt"),
                    size: wgpu::Extent3d {
                        width: pw,
                        height: ph,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING
                        | extra,
                    view_formats: &[],
                })
            };
            let color = mk(
                draw::COLOR_FORMAT,
                wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            );
            let depth = mk(draw::DEPTH_FORMAT, wgpu::TextureUsages::empty());
            self.rts.insert(
                fbp,
                RtTarget {
                    w: pw,
                    h: ph,
                    color,
                    depth,
                },
            );
        }
    }

    /// Build the geometry of one frame.
    /// The texture for a TexInfo, and its largest texel alpha.
    /// Plan the frame's VU1 calls: split into chunks (see `VuPlan`), each
    /// with the VU memory it starts from.

    fn texture(&mut self, info: u32, texa: u64, mem: &dyn Mem) -> Option<(wgpu::TextureView, u8)> {
        let key = self.texkeys.key(mem, info, texa);
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
            let (a, b) = (
                r[0].width as f32 / r[0].height as f32,
                d.width as f32 / d.height as f32,
            );
            if (a / b - 1.0).abs() > 0.01 {
                eprintln!(
                    "[gfx] replacement {} is {}x{}, the texture {}x{}: aspect ratio differs",
                    pack::name(&key),
                    r[0].width,
                    r[0].height,
                    d.width,
                    d.height
                );
            }
        }
        // Options: mipmaps for the game's own textures.
        let generated = (replaced.is_none() && self.opts.mipmaps).then(|| tex::mipmaps(&d));
        let levels: &[tex::Decoded] = replaced
            .as_deref()
            .or(generated.as_deref())
            .unwrap_or(std::slice::from_ref(&d));
        let t = self.dev.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssx3-gfx tex"),
            size: wgpu::Extent3d {
                width: levels[0].width,
                height: levels[0].height,
                depth_or_array_layers: 1,
            },
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
                wgpu::TexelCopyTextureInfo {
                    texture: &t,
                    mip_level: lv as u32,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &bytes,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(l.width * 4),
                    rows_per_image: Some(l.height),
                },
                wgpu::Extent3d {
                    width: l.width,
                    height: l.height,
                    depth_or_array_layers: 1,
                },
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
        let amax = levels[0]
            .rgba
            .iter()
            .map(|p| (p >> 24) as u8)
            .max()
            .unwrap_or(0);
        self.texcache.insert(
            key,
            CachedTex {
                view: view.clone(),
                used: frame,
                amax,
            },
        );
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
        let up = self
            .gs
            .fog_cluts
            .get(i as usize)
            .filter(|u| !u.data.is_empty())?;
        let mut px = [0u32; 256];
        for (i, p) in px.iter_mut().enumerate() {
            // CSM1: entries 8..15 and 16..23 of each 32 swap places.
            let j = (i & !0x18) | (i & 0x8) << 1 | (i & 0x10) >> 1;
            *p = up.data.get(j).copied().unwrap_or(0);
        }
        let t = self.dev.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssx3-gfx clut"),
            size: wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let bytes: Vec<u8> = px.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &t,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1024),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
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

    /// With a [`VuOffload`] and no render thread (the browser): start this
    /// frame's VU1 chunks on the helpers and draw it when the next frame
    /// comes, so the game's next frame runs while they work. One frame of
    /// latency, as with the native render thread.
    fn publish_staged(&mut self, f: ssx3_ps2::frame_sink::Published) {
        self.finish_staged();
        let b = self.take_build();
        let collect = {
            let draws: Vec<&frame::Draw> = f.built.frame.draws().collect();
            let plan = Build::plan_vu_calls(&b, &f.built, &draws);
            plan.offloadable()
                .then(|| (plan.bounds.len(), (OFFLOAD.get().unwrap().0)(plan.jobs())))
        };
        self.put_build(b);
        self.staged = Some(Staged { f, collect });
    }

    /// Draw the staged frame, if any (`publish_staged`).
    pub fn finish_staged(&mut self) {
        let Some(s) = self.staged.take() else { return };
        *PREFETCH.lock().unwrap() = s.collect;
        self.render_built(&s.f.built, &s.f.carry, &s.f.mem);
        // Not taken (the plan came out different): still wait, the helpers
        // must be idle before the next jobs.
        if let Some((_, c)) = PREFETCH.lock().unwrap().take() {
            c();
        }
    }

    /// Render a published frame (built with `carry`) over the memory it
    /// reads; keep it in the frame-dump ring if it carries all of memory.
    pub fn render_built(
        &mut self,
        built: &Built,
        carry: &ssx3_render::frame::Carry,
        mem: &ssx3_ps2::frame_sink::FrameMem,
    ) {
        self.frames_in += 1;
        if let Some(full) = &mem.full {
            ssx3_ps2::frame_sink::dump_frame(carry, full, &self.vu[..]);
        }
        self.render(built, mem);
    }

    /// Render a frame.
    pub fn render(&mut self, built: &Built, mem: &(dyn Mem + Sync)) {
        self.render_one(built, mem);
        if !self.present.is_empty() {
            *self.shared.present.lock().unwrap() =
                Some((self.present.clone(), web_time::Instant::now()));
        }
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
        self.stats.frames += 1;
        let t_start = web_time::Instant::now();
        let mut b = self.take_build();
        self.world_casters = built.casters.clone();
        self.frame_gm = built.frame.gm;
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
        if self.gs.verts.is_empty() && self.gs.gpu.calls.is_empty() {
            return;
        }
        // Vertex buffer.
        // SAFETY: `Vtx` is `repr(C)`, twelve f32s with no padding, in the
        // vertex layout's order; the slice covers exactly the vertices.
        let vbytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.gs.verts.as_ptr() as *const u8,
                std::mem::size_of_val(&self.gs.verts[..]),
            )
        };
        let vbuf = grow_buffer(
            &self.dev,
            &mut self.vbuf,
            (vbytes.len() as u64).max(256),
            wgpu::BufferUsages::VERTEX,
            "ssx3-gfx verts",
        );
        if !vbytes.is_empty() {
            self.queue.write_buffer(&vbuf, 0, vbytes);
        }
        // Sun shadows (shadow.rs): this frame's light, the map and its
        // uniforms (bound with the GPU draws, group 1).
        if shadow::debug("gpustats") {
            // The VU calls of this frame left to the CPU programs.
            let mut st = ssx3_render::gpu::STATS.lock().unwrap();
            let cpu: Vec<String> = st
                .iter()
                .filter(|(k, _)| !k.3)
                .map(|((h, e, i, _), n)| {
                    format!("{h:x}/{e:#x}{} x{n}", if *i { " itop" } else { "" })
                })
                .collect();
            let gpu: u64 = st.iter().filter(|(k, _)| k.3).map(|(_, n)| n).sum();
            eprintln!(
                "gpustats frame {}: gpu {gpu} cpu {cpu:?}",
                self.stats.frames
            );
            st.clear();
        }
        let projector = ssx3_render::bezier::take_projector();
        if shadow::debug("print") {
            eprintln!(
                "shadow: key light {:?} projector {:?}",
                self.key_light, projector
            );
        }
        if self.gs.key_count > 0 {
            let n = self.gs.key_count as f32;
            for c in 0..3 {
                self.live_ambient[c] =
                    self.live_ambient[c] * 0.9 + self.gs.key_ambient[c] / n * 0.1;
                self.live_colour[c] = self.live_colour[c] * 0.9 + self.gs.key_colour[c] / n * 0.1;
            }
        }
        if shadow::debug("lightlog") {
            eprintln!(
                "[light] {} amb {:.1} {:.1} {:.1} col {:.1} {:.1} {:.1}",
                self.world_casters.as_ref().map_or("", |c| c.world.as_str()),
                self.live_ambient[0],
                self.live_ambient[1],
                self.live_ambient[2],
                self.live_colour[0],
                self.live_colour[1],
                self.live_colour[2]
            );
        }
        let k = self.gs.key_light;
        let kn = (k[0] * k[0] + k[1] * k[1] + k[2] * k[2]).sqrt();
        if kn > 1e-3 {
            let p = self.key_light;
            let m: [f32; 3] = std::array::from_fn(|i| p[i] * 0.9 + k[i] / kn * 0.1);
            let mn = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt().max(1e-6);
            self.key_light = m.map(|x| x / mn);
        }
        // The sun: the meshes' key light (`SSX3_SHADOW_DEBUG=flare-sun`:
        // where the game draws the sun sprite instead).
        if let (Some((x, y, z)), Some(cam)) =
            (ssx3_ps2::frame_sink::take_light(), self.gs.cam.as_ref())
        {
            if let Some(d) = shadow::sun_from_screen(cam, x, y, z) {
                let d = d.map(|v| v as f32);
                let m = match self.flare_sun {
                    Some(p) => std::array::from_fn(|i| p[i] * 0.9 + d[i] * 0.1),
                    None => d,
                };
                let l = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt().max(1e-6);
                self.flare_sun = Some(m.map(|v: f32| v / l));
            }
        }
        if shadow::debug("print") {
            eprintln!("shadow: flare sun {:?}", self.flare_sun);
        }
        let key = match self.flare_sun {
            Some(f) if shadow::debug("flare-sun") => f,
            _ => self.key_light,
        };
        let light = if self.opts.shadows {
            self.gs.cam.as_ref().and_then(|c| shadow::light(c, key))
        } else {
            None
        };
        let map_size = if light.is_some() {
            shadow::tuning().size
        } else {
            self.shadow.as_ref().map_or(1, |s| s.size)
        };
        if self.shadow.as_ref().is_none_or(|s| s.size != map_size) {
            self.shadow = Some(shadow::Shadow::new(&self.dev, map_size));
        }
        if let (Some(c), true) = (self.world_casters.clone(), light.is_some()) {
            self.shadow
                .as_mut()
                .unwrap()
                .set_casters(&self.dev, &self.queue, &c);
        }
        // With the world's casters the terrain casts from them (the game's
        // terrain draws only cover what the camera sees).
        let native_terrain =
            light.is_some() && self.world_casters.is_some() && !shadow::debug("no-world");
        let sh = self.shadow.as_ref().unwrap();
        let world = self.world_casters.as_ref().map_or("", |c| c.world.as_str());
        let g = shadow::lighting(world);
        let (amb, col) = (
            self.live_ambient.map(|v| v / 128.0),
            self.live_colour.map(|v| v / 128.0),
        );
        let lt = shadow::Lighting {
            sun: std::array::from_fn(|c| g.sun[c] * col[c]),
            sky: std::array::from_fn(|c| g.sky[c] * amb[c]),
            ground: std::array::from_fn(|c| g.ground[c] * amb[c]),
        };
        sh.upload(&self.queue, light.as_ref(), shadow::show(), lt);
        // GPU-computed draws (ssx3_render::gpu): their uniforms, inputs and
        // triangle lists, bound as group 1 (and again with the placeholder
        // shadow map for the shadow pass).
        let mut gpu_bind_shadow = None;
        let gpu_bind = (!self.gs.gpu.calls.is_empty()).then(|| {
            let g = &self.gs.gpu;
            // (u32 slices as their bytes, no copy: the GPU reads little-endian
            // words, as the targets store them.)
            fn bytes(w: &[u32]) -> &[u8] {
                // SAFETY: any u32 slice is a valid byte slice of 4× its length.
                unsafe { std::slice::from_raw_parts(w.as_ptr() as *const u8, std::mem::size_of_val(w)) }
            }
            let calls = bytes(g.calls.as_flattened());
            let data = bytes(g.data.as_flattened());
            let idx = bytes(&g.indices);
            let of = bytes(&g.call_start);
            static DBG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            if *DBG.get_or_init(|| std::env::var_os("DBG_UPLOAD").is_some()) {
                eprintln!("[upload] calls {} KiB, data {} KiB, indices {} KiB, call starts {} KiB, verts {} KiB", calls.len() >> 10, data.len() >> 10, idx.len() >> 10, of.len() >> 10, vbytes.len() >> 10);
            }
            let [cb, db, ib, ob] = &mut self.gbuf;
            let cb = grow_buffer(&self.dev, cb, (calls.len() as u64).max(256), wgpu::BufferUsages::STORAGE, "ssx3-gfx gpu calls");
            let db = grow_buffer(&self.dev, db, (data.len() as u64).max(256), wgpu::BufferUsages::STORAGE, "ssx3-gfx gpu data");
            let ib = grow_buffer(&self.dev, ib, (idx.len() as u64).max(256), wgpu::BufferUsages::STORAGE, "ssx3-gfx gpu indices");
            let ob = grow_buffer(&self.dev, ob, (of.len() as u64).max(256), wgpu::BufferUsages::STORAGE, "ssx3-gfx gpu call starts");
            self.queue.write_buffer(&cb, 0, calls);
            if !data.is_empty() {
                self.queue.write_buffer(&db, 0, data);
            }
            if !idx.is_empty() {
                self.queue.write_buffer(&ib, 0, idx);
            }
            self.queue.write_buffer(&ob, 0, of);
            // The arena: what is new since the last upload (all of it when
            // it was cleared or outgrew its buffer).
            let ab = {
                let mut a = g.arena.lock().unwrap();
                let need = (a.data.len() as u64 * 16).max(256);
                let fresh = self.arena_epoch != a.epoch || self.arena_buf.as_ref().is_none_or(|b| b.size() < need);
                if fresh {
                    self.arena_buf = None;
                    a.uploaded = 0;
                    self.arena_epoch = a.epoch;
                }
                let ab = grow_buffer(&self.dev, &mut self.arena_buf, need, wgpu::BufferUsages::STORAGE, "ssx3-gfx mesh arena");
                if a.uploaded < a.data.len() {
                    let new = bytes(a.data[a.uploaded..].as_flattened());
                    self.queue.write_buffer(&ab, a.uploaded as u64 * 16, new);
                    a.uploaded = a.data.len();
                }
                ab
            };
            let group = |real: bool| {
                let [s5, s6, s7] = sh.entries(real);
                self.dev.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("ssx3-gfx gpuvu"),
                    layout: &self.pipes.gpu_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: cb.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 1, resource: db.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 2, resource: ib.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 3, resource: ob.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 4, resource: ab.as_entire_binding() },
                        s5,
                        s6,
                        s7,
                    ],
                })
            };
            if light.is_some() {
                gpu_bind_shadow = Some(group(false));
            }
            group(true)
        });
        // Plans per batch, uniforms, bind groups.
        let batches = std::mem::take(&mut self.gs.batches);
        // With sun shadows the game's own rider shadows go: the render
        // targets its silhouettes (mat6 entry 0x20) are drawn into, and every
        // draw that projects them (`SSX3_SHADOW_DEBUG=game-shadows` keeps
        // them).
        let silhouette_rts: std::collections::HashSet<u32> =
            if light.is_some() && !shadow::debug("game-shadows") {
                batches
                    .iter()
                    .filter(|b| {
                        b.gpu
                            .is_some_and(|g| g.program == ssx3_render::gpu::Program::Mat6Shadow)
                    })
                    .map(|b| b.state.target)
                    .filter(|&t| t != self.gs.main_fbp)
                    .collect()
            } else {
                Default::default()
            };
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
            /// GPU-computed vertices.
            gpu: Option<gs::GpuBatch>,
            /// What produced it (the renderer's own passes go before the
            /// game's post passes and before the HUD).
            tag: gs::Tag,
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
        let probe = dbg
            .probe
            .filter(|_| dbg.trace.is_none_or(|f| f == self.stats.frames));
        let opts = self.opts;
        let time = self.t0.elapsed().as_secs_f32();
        for (bi, b) in batches.iter().enumerate() {
            let st = &b.state;
            // The game's bloom, replaced or off.
            if !opts.keeps_post(b.tag.post) {
                continue;
            }
            if !silhouette_rts.is_empty()
                && (silhouette_rts.contains(&st.target)
                    || st.tex.is_some_and(|t| {
                        t != gs::FRAME_TEX
                            && t & gs::RT_TEX != 0
                            && silhouette_rts.contains(&(t & !gs::RT_TEX))
                    }))
            {
                continue;
            }
            // Debug: the terrain without its second pass.
            if b.tag.pass2 && shadow::debug("no-pass2") {
                continue;
            }
            let terrain_blend = (b
                .gpu
                .is_some_and(|g| g.program == ssx3_render::gpu::Program::Bezier)
                && st.prim & (1 << 6) != 0)
                .then_some(st.ctx.alpha & 0xff);
            // The renderer's terrain lighting: the game's baked layer goes,
            // the base layer is lit (flag 16384). For fitting
            // (`SSX3_SHADOW_SHOW=normals|sunlit`) only the lit terrain draws.
            let native_light = light.is_some() && !shadow::debug("baked");
            let lit_base = native_light
                && st.target == self.gs.main_fbp
                && !b.tag.hud
                && b.tag.post.is_none()
                && b.gpu
                    .is_some_and(|g| g.program == ssx3_render::gpu::Program::Bezier)
                && (st.prim & (1 << 6) == 0 || terrain_blend == Some(0x2a));
            if native_light && (terrain_blend == Some(0x81) || (shadow::show() >= 4.0 && !lit_base))
            {
                continue;
            }
            if (terrain_blend == Some(0x81) && shadow::debug("no-sub"))
                || (terrain_blend == Some(0x58) && shadow::debug("no-add"))
            {
                continue;
            }
            if shadow::debug("blend")
                && b.gpu
                    .is_some_and(|g| g.program == ssx3_render::gpu::Program::Bezier)
            {
                eprintln!(
                    "blend: pass2 {} prim {:#x} alpha {:#x} tfx {} tcc {} test {:#x} zbuf-msk {}",
                    b.tag.pass2,
                    st.prim,
                    st.ctx.alpha,
                    (st.ctx.tex0 >> 35) & 3,
                    (st.ctx.tex0 >> 34) & 1,
                    st.ctx.test,
                    (st.ctx.zbuf >> 32) & 1
                );
            }
            let alpha_only = (st.ctx.frame >> 32) as u32 & 0x00ff_ffff == 0x00ff_ffff;
            // `z`: keep the batch but without depth writes.
            let no_zw = skip_test
                .iter()
                .any(|&(t, m)| t == st.ctx.test && m == b'z');
            if skip_test.iter().any(|&(t, m)| {
                t == st.ctx.test && m != b'z' && (m == 0 || (m == b'a') == alpha_only)
            }) {
                continue;
            }
            // A render target sampled while drawn into (a GS feedback
            // loop; wgpu forbids it): dropped.
            if st.tex.is_some_and(|t| {
                t != gs::FRAME_TEX && t & gs::RT_TEX != 0 && t & !gs::RT_TEX == st.target
            }) {
                self.stats.dropped += 1;
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
                        Some(t) if t & gs::RT_TEX != 0 => (
                            self.rts
                                .get(&(t & !gs::RT_TEX))
                                .map(|r| r.color.create_view(&Default::default())),
                            128,
                        ),
                        Some(t) if t & gs::ZFOG_TEX != 0 => {
                            (self.clut_texture(t & !gs::ZFOG_TEX), 255)
                        }
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
                match self
                    .gs
                    .rts
                    .get(&st.target)
                    .filter(|_| self.rts.contains_key(&st.target))
                {
                    Some(r) => (r.w, r.h),
                    None => continue,
                }
            };
            let sc = st.ctx.scissor;
            let (x0, x1, y0, y1) = (
                (sc & 0x7ff) as u32,
                ((sc >> 16) & 0x7ff) as u32,
                ((sc >> 32) & 0x7ff) as u32,
                ((sc >> 48) & 0x7ff) as u32,
            );
            // In target pixels.
            let (fx, fy) = if st.target == self.gs.main_fbp {
                self.fxy
            } else {
                self.rt_fxy(st.target)
            };
            let px = |v: u32, f: f32| (v as f32 * f).round() as u32;
            let scissor = (x0 <= x1 && y0 <= y1 && x0 < tw && y0 < th).then(|| {
                let (a, b) = (px(x0, fx), px(y0, fy));
                [
                    a,
                    b,
                    px((x1 + 1).min(tw), fx).max(a + 1) - a,
                    px((y1 + 1).min(th), fy).max(b + 1) - b,
                ]
            });
            if scissor.is_none() {
                continue;
            }
            let rt_psm = st
                .tex
                .filter(|t| *t != gs::FRAME_TEX && t & gs::RT_TEX != 0)
                .and_then(|t| self.gs.rts.get(&(t & !gs::RT_TEX)))
                .map(|r| r.psm);
            // DATE on the main target (render targets: not yet).
            let date = (st.ctx.test >> 14) & 1 != 0 && st.target == self.gs.main_fbp;
            let zfog = st.tex.is_some_and(|t| {
                t != gs::FRAME_TEX && t & gs::RT_TEX == 0 && t & gs::ZFOG_TEX != 0
            });
            // The largest As the batch can produce (blends with As > 1
            // need care: see draw::plans).
            let av = match b.gpu {
                Some(g) => g.alpha_max,
                None => self.gs.verts[b.first as usize..(b.first + b.count) as usize]
                    .iter()
                    .fold(0f32, |m, v| m.max(v.col[3])),
            };
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
            // Debug: the terrain's subtractive (baked lighting) layer alone,
            // drawn opaque (its colour is what it subtracts).
            if shadow::debug("sub-only")
                && b.gpu
                    .is_some_and(|g| g.program == ssx3_render::gpu::Program::Bezier)
                && terrain_blend != Some(0x81)
            {
                continue;
            }
            for mut p in draw::plans(st, (tw, th), rt_psm, as_max) {
                if no_zw {
                    p.key.depth_write = false;
                }
                if shadow::debug("sub-only") && terrain_blend == Some(0x81) {
                    p.key.blend = None;
                    p.u.src_mode = 0;
                    p.key.depth = wgpu::CompareFunction::GreaterEqual;
                    p.key.depth_write = true;
                }
                if dbg.planlog == Some(st.ctx.test) {
                    eprintln!("[gfx-plan] alpha {:#x} fba {} flags {:#x} tfx {} src_mode {} cs {} {} c_kind {} fix {} key {:?}", st.ctx.alpha, st.ctx.fba, p.u.flags, p.u.tfx, p.u.src_mode, p.u.cs_a, p.u.cs_b, p.u.c_kind, p.u.fix, p.key);
                }
                let ubo = (ubytes.len() as u64 / draw::UNIFORM_STRIDE) as u32;
                if date {
                    p.u.flags |= 64 | (((st.ctx.test >> 15) & 1) as u32) << 7;
                }
                // Sun shadows: the 3D draws receive them.
                if lit_base {
                    p.u.flags |= 16384;
                } else if light.is_some()
                    && !shadow::debug("no-recv")
                    && st.target == self.gs.main_fbp
                    && !b.tag.hud
                    && b.tag.post.is_none()
                    && b.gpu.is_some_and(|g| shadow::receives(g.program))
                {
                    p.u.flags |= 4096;
                }
                // Glints on bright snow (textured terrain).
                if opts.snow && b.tag.terrain && st.tex.is_some() {
                    p.u.flags |= 2048;
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
                let words: [u32; 24] = [
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
                    opts.lod_bias.to_bits(),
                    time.to_bits(),
                    0,
                    0,
                    0,
                ];
                // SAFETY: u32 words as their (little-endian) bytes.
                ubytes.extend_from_slice(unsafe {
                    std::slice::from_raw_parts(
                        words.as_ptr() as *const u8,
                        std::mem::size_of_val(&words),
                    )
                });
                ubytes.resize(
                    ubytes.len().next_multiple_of(draw::UNIFORM_STRIDE as usize),
                    0,
                );
                let desc = if probe.is_some() {
                    format!(
                        "prim {:#x} tex {:x?} tex0 {:#x} alpha {:#x} test {:#x} fba {} frame {:#x} flags {:#x} src_mode {} key {:?}",
                        st.prim, st.tex, st.ctx.tex0, st.ctx.alpha, st.ctx.test, st.ctx.fba, st.ctx.frame, p.u.flags, p.u.src_mode, p.key
                    )
                } else {
                    String::new()
                };
                items.push(Item {
                    tag: b.tag,
                    batch: bi,
                    gpu: b.gpu,
                    desc,
                    frame: st.tex == Some(gs::FRAME_TEX),
                    date,
                    zfog,
                    target: st.target,
                    key: p.key,
                    ubo,
                    constant: p.constant,
                    bind,
                    first: b.first,
                    count: b.count,
                    scissor,
                });
            }
        }
        if items.is_empty() {
            return;
        }
        let t_prepared = web_time::Instant::now();
        self.timing.1[1] += (t_prepared - t_built).as_secs_f64() * 1e3;
        let ubuf = grow_buffer(
            &self.dev,
            &mut self.ubuf,
            ubytes.len() as u64,
            wgpu::BufferUsages::UNIFORM,
            "ssx3-gfx uniforms",
        );
        self.queue.write_buffer(&ubuf, 0, &ubytes);
        let snap = self
            .targets
            .as_ref()
            .expect("targets")
            .snapshot
            .create_view(&Default::default());
        let zsnap = self
            .targets
            .as_ref()
            .expect("targets")
            .zsnap
            .create_view(&Default::default());
        let groups: Vec<wgpu::BindGroup> = binds
            .iter()
            .map(|(view, sk)| {
                let aniso = self.aniso;
                let samp = self
                    .samplers
                    .entry(*sk)
                    .or_insert_with(|| draw::sampler(&self.dev, *sk, aniso))
                    .clone();
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
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                view.as_ref().unwrap_or(&self.white),
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::Sampler(&samp),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::TextureView(&snap),
                        },
                        wgpu::BindGroupEntry {
                            binding: 4,
                            resource: wgpu::BindingResource::TextureView(&zsnap),
                        },
                    ],
                })
            })
            .collect();
        for it in &items {
            match it.gpu {
                Some(g) => {
                    self.pipes.get_gpu(&self.dev, it.key, g.program);
                }
                None => {
                    self.pipes.get(&self.dev, it.key);
                }
            }
        }
        // The renderer's own passes: AO before the game's post passes, the
        // rest before the HUD (or at the end of a frame without them).
        let hud_post = opts.bloom == Bloom::Modern || opts.aa != Aa::Off;
        // (Sun shadows are applied by the draws themselves: they need the
        // split only for their debug views.)
        let ao_at = if opts.ao || (opts.shadows && post::debug_view() != 0.0) {
            items
                .iter()
                .position(|it| it.tag.post.is_some() || it.tag.hud)
                .unwrap_or(items.len())
        } else {
            usize::MAX
        };
        let hud_at = if hud_post {
            items
                .iter()
                .position(|it| it.tag.hud)
                .unwrap_or(items.len())
        } else {
            usize::MAX
        };
        if (opts.ao || ao_at != usize::MAX || hud_post) && self.post.is_none() {
            self.post = Some(post::Post::new(&self.dev));
        }
        if let Some(po) = self.post.as_mut() {
            po.begin_frame();
        }
        // The world's cut-outs (trees): each texture's bind group.
        let mut cut_groups: Vec<(wgpu::BindGroup, u32, u32)> = Vec::new();
        let runs = self
            .shadow
            .as_ref()
            .and_then(|s| s.cutouts.as_ref())
            .map(|c| c.1.clone());
        if let (true, Some(runs), Some(_)) = (native_terrain, runs, light.as_ref()) {
            let texman = frame::word(mem, self.frame_gm + frame::gm::TEXMAN);
            let samp = self.dev.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("ssx3-gfx cut-out"),
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            for (t, first, n) in runs {
                let info = if texman != 0 {
                    tex::info(mem, texman, t as u16)
                } else {
                    0
                };
                let Some((view, _)) = (info != 0)
                    .then(|| self.texture(info, 0x80 << 32, mem))
                    .flatten()
                else {
                    continue;
                };
                let bg = self.dev.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("ssx3-gfx cut-out"),
                    layout: &self.pipes.cut_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&samp),
                        },
                    ],
                });
                cut_groups.push((bg, first, n));
            }
        }
        let t = self.targets.as_ref().expect("targets");
        let cv = t.color.create_view(&Default::default());
        let dv = t.depth.create_view(&Default::default());
        let mut enc = self.dev.create_command_encoder(&Default::default());
        let main = self.gs.main_fbp;
        // Sun shadows: the 3D draws before the game's post passes again,
        // into the shadow map (shadow.rs), before everything else.
        if let (Some(_), Some(gb), false) = (
            light.as_ref(),
            gpu_bind_shadow.as_ref(),
            shadow::debug("no-pass"),
        ) {
            let sh = self.shadow.as_ref().unwrap();
            let casts = |it: &Item| {
                it.target == main
                    && !it.tag.hud
                    && it.tag.post.is_none()
                    && it.key.depth_write
                    && it.scissor.is_some()
                    && it.gpu.is_some_and(|g| {
                        shadow::receives(g.program)
                            && !(native_terrain && g.program == ssx3_render::gpu::Program::Bezier)
                    })
            };
            let end = ao_at.min(items.len());
            if shadow::debug("items") {
                let mut by: std::collections::BTreeMap<String, (u32, u32)> = Default::default();
                for it in items[..end]
                    .iter()
                    .filter(|it| it.target == main && !it.tag.hud)
                {
                    let k = format!("{:?} zw {}", it.gpu.map(|g| g.program), it.key.depth_write);
                    let e = by.entry(k).or_default();
                    e.0 += 1;
                    e.1 += it.gpu.map_or(it.count, |g| g.count);
                }
                eprintln!("shadow items: {by:?}");
            }
            for it in items[..end].iter().filter(|it| casts(it)) {
                self.pipes.shadow(&self.dev, it.gpu.unwrap().program);
            }
            let mut sp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssx3-gfx shadow map"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &sh.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            sp.set_bind_group(1, gb, &[]);
            // One quarter of the atlas per cascade (the instance index).
            let tile = (sh.size / 2) as f32;
            for c in 0..shadow::CASCADES as u32 {
                sp.set_viewport(
                    (c & 1) as f32 * tile,
                    (c >> 1) as f32 * tile,
                    tile,
                    tile,
                    0.0,
                    1.0,
                );
                let mut last_prog = None;
                for it in items[..end].iter().filter(|it| casts(it)) {
                    let g = it.gpu.unwrap();
                    if last_prog != Some(g.program) {
                        last_prog = Some(g.program);
                        sp.set_pipeline(self.pipes.cached_shadow(g.program));
                    }
                    sp.set_bind_group(0, &groups[it.bind], &[it.ubo * draw::UNIFORM_STRIDE as u32]);
                    sp.draw(g.first..g.first + g.count, c..c + 1);
                }
                if let (true, Some((_, buf, n))) = (native_terrain, sh.casters.as_ref()) {
                    sp.set_pipeline(&self.pipes.caster);
                    sp.set_bind_group(0, &groups[0], &[0]);
                    sp.set_vertex_buffer(0, buf.slice(..));
                    sp.draw(0..*n, c..c + 1);
                }
                if let (false, Some((buf, _))) = (cut_groups.is_empty(), sh.cutouts.as_ref()) {
                    sp.set_pipeline(&self.pipes.caster_cut);
                    sp.set_bind_group(0, &groups[0], &[0]);
                    sp.set_vertex_buffer(0, buf.slice(..));
                    for (bg, first, n) in &cut_groups {
                        sp.set_bind_group(2, bg, &[]);
                        sp.draw(*first..*first + *n, c..c + 1);
                    }
                }
            }
        }
        let rts = &self.rts;
        // A pass on target `fbp`; depth starts cleared on a target's first
        // pass of the frame.
        let begin =
            |enc: &mut wgpu::CommandEncoder, fbp: u32, first: bool| -> wgpu::RenderPass<'static> {
                let (c, d) = if fbp == main {
                    (cv.clone(), dv.clone())
                } else {
                    let r = &rts[&fbp];
                    (
                        r.color.create_view(&Default::default()),
                        r.depth.create_view(&Default::default()),
                    )
                };
                enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ssx3-gfx frame"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &c,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &d,
                        // Each frame starts with depth cleared to the farthest
                        // value (the GS's Z history across frames is not kept).
                        depth_ops: Some(wgpu::Operations {
                            load: if first {
                                wgpu::LoadOp::Clear(0.0)
                            } else {
                                wgpu::LoadOp::Load
                            },
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
        // The pass state last set (each command only when it changes; a new
        // pass starts with none).
        #[derive(Default, PartialEq)]
        struct Last {
            pipe: Option<(draw::PipeKey, Option<ssx3_render::gpu::Program>)>,
            group0: Option<(usize, u32)>,
            group1: bool,
            constant: Option<u64>,
            scissor: Option<[u32; 4]>,
        }
        let mut last = Last::default();
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
                wgpu::TexelCopyTextureInfo {
                    texture: &t.depth,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &t.qdepth,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: t.w,
                    height: t.h,
                    depth_or_array_layers: 1,
                },
            );
        };
        let post_frame = post::Frame {
            color: &t.color,
            snapshot: &t.snapshot,
            depth: &t.depth,
            zsnap: &t.zsnap,
            w: t.w,
            h: t.h,
        };
        for (k, it) in items.iter().enumerate() {
            if k == ao_at || k == hud_at {
                drop(pass);
                let po = self.post.as_mut().expect("post passes");
                if k == ao_at {
                    if let (Some(l), Some(sh), Some(cam), true) = (
                        light.as_ref(),
                        self.shadow.as_ref(),
                        self.gs.cam.as_ref(),
                        post::debug_view() != 0.0,
                    ) {
                        po.shadow(
                            &self.dev,
                            &self.queue,
                            &mut enc,
                            &post_frame,
                            cam,
                            self.fxy,
                            &sh.view,
                            l,
                        );
                    }
                    if opts.ao {
                        po.ao(
                            &self.dev,
                            &self.queue,
                            &mut enc,
                            &post_frame,
                            self.gs.cam.as_ref(),
                            self.fxy,
                        );
                    }
                }
                if k == hud_at {
                    po.before_hud(&self.dev, &self.queue, &mut enc, &post_frame, &opts);
                }
                pass = begin(&mut enc, cur_target, false);
                pass.set_vertex_buffer(0, vbuf.slice(..));
                last = Last::default();
                // (They overwrote the snapshot.)
                prev_frame = false;
            }
            if self.want_depth && !depth_taken && it.batch >= phase1_end {
                depth_taken = true;
                drop(pass);
                take_depth(&mut enc);
                pass = begin(&mut enc, cur_target, false);
                pass.set_vertex_buffer(0, vbuf.slice(..));
                last = Last::default();
            }
            let Some([x, y, sw, sh]) = it.scissor else {
                continue;
            };
            // (Frame-sampling items may draw into another target: the
            // display buffer copy.)
            // A new run of frame-sampling items, or one into another target
            // (the display copy after the colour filter), needs a fresh one.
            let snapshot = it.frame && (!prev_frame || it.target != cur_target)
                || it.date && it.target == main;
            let zsnapshot = it.zfog && it.target == main;
            if it.target != cur_target || snapshot || zsnapshot {
                drop(pass);
                if zsnapshot {
                    enc.copy_texture_to_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &t.depth,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::TexelCopyTextureInfo {
                            texture: &t.zsnap,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::Extent3d {
                            width: t.w,
                            height: t.h,
                            depth_or_array_layers: 1,
                        },
                    );
                }
                if snapshot {
                    // Snapshot the frame as drawn so far.
                    enc.copy_texture_to_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &t.color,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::TexelCopyTextureInfo {
                            texture: &t.snapshot,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::Extent3d {
                            width: t.w,
                            height: t.h,
                            depth_or_array_layers: 1,
                        },
                    );
                }
                cur_target = it.target;
                let first = started.insert(cur_target);
                pass = begin(&mut enc, cur_target, first);
                pass.set_vertex_buffer(0, vbuf.slice(..));
                last = Last::default();
            }
            prev_frame = it.frame;
            let program = it.gpu.map(|g| g.program);
            if program.is_some() && gpu_bind.is_none() {
                continue;
            }
            if last.pipe != Some((it.key, program)) {
                // (The GPU programs' pipelines have another layout: their
                // bind groups are set again.)
                if last
                    .pipe
                    .is_some_and(|(_, p)| p.is_some() != program.is_some())
                {
                    last.group0 = None;
                    last.group1 = false;
                }
                last.pipe = Some((it.key, program));
                pass.set_pipeline(match program {
                    Some(p) => self.pipes.cached_gpu(it.key, p),
                    None => self.pipes.cached(it.key),
                });
            }
            if last.group0 != Some((it.bind, it.ubo)) {
                last.group0 = Some((it.bind, it.ubo));
                pass.set_bind_group(0, &groups[it.bind], &[it.ubo * draw::UNIFORM_STRIDE as u32]);
            }
            if let (Some(bg), false) =
                (gpu_bind.as_ref().filter(|_| program.is_some()), last.group1)
            {
                last.group1 = true;
                pass.set_bind_group(1, bg, &[]);
            }
            if last.constant != Some(it.constant.to_bits()) {
                last.constant = Some(it.constant.to_bits());
                pass.set_blend_constant(wgpu::Color {
                    r: it.constant,
                    g: it.constant,
                    b: it.constant,
                    a: it.constant,
                });
            }
            if last.scissor != Some([x, y, sw, sh]) {
                last.scissor = Some([x, y, sw, sh]);
                pass.set_scissor_rect(x, y, sw, sh);
            }
            match it.gpu {
                Some(g) => pass.draw(g.first..g.first + g.count, 0..1),
                None => pass.draw(it.first..it.first + it.count, 0..1),
            }
            if let (Some((px, py)), Some(pb)) = (probe, probe_buf.as_ref()) {
                if it.target == main {
                    drop(pass);
                    enc.copy_texture_to_buffer(
                        wgpu::TexelCopyTextureInfo {
                            texture: &t.color,
                            mip_level: 0,
                            origin: wgpu::Origin3d { x: px, y: py, z: 0 },
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::TexelCopyBufferInfo {
                            buffer: pb,
                            layout: wgpu::TexelCopyBufferLayout {
                                offset: probed.len() as u64 * 256,
                                bytes_per_row: Some(256),
                                rows_per_image: Some(1),
                            },
                        },
                        wgpu::Extent3d {
                            width: 1,
                            height: 1,
                            depth_or_array_layers: 1,
                        },
                    );
                    probed.push(it.desc.clone());
                    pass = begin(&mut enc, cur_target, false);
                    pass.set_vertex_buffer(0, vbuf.slice(..));
                    last = Last::default();
                }
            }
        }
        drop(pass);
        if ao_at == items.len() || hud_at == items.len() {
            let po = self.post.as_mut().expect("post passes");
            if ao_at == items.len() {
                if let (Some(l), Some(sh), Some(cam), true) = (
                    light.as_ref(),
                    self.shadow.as_ref(),
                    self.gs.cam.as_ref(),
                    post::debug_view() != 0.0,
                ) {
                    po.shadow(
                        &self.dev,
                        &self.queue,
                        &mut enc,
                        &post_frame,
                        cam,
                        self.fxy,
                        &sh.view,
                        l,
                    );
                }
                if opts.ao {
                    po.ao(
                        &self.dev,
                        &self.queue,
                        &mut enc,
                        &post_frame,
                        self.gs.cam.as_ref(),
                        self.fxy,
                    );
                }
            }
            if hud_at == items.len() {
                po.before_hud(&self.dev, &self.queue, &mut enc, &post_frame, &opts);
            }
        }
        if self.want_depth && !depth_taken {
            take_depth(&mut enc);
        }
        // The image on screen, for the window.
        let shown = match self.display_fbp.and_then(|f| self.rts.get(&f)) {
            Some(r) => &r.color,
            None => &t.color,
        };
        let (slots, slot) = (1, 0);
        if self.present.len() != slots
            || self
                .present
                .first()
                .is_some_and(|p| (p.width(), p.height()) != (t.w, t.h))
        {
            self.present = (0..slots)
                .map(|_| {
                    self.dev.create_texture(&wgpu::TextureDescriptor {
                        label: Some("ssx3-gfx present"),
                        size: wgpu::Extent3d {
                            width: t.w,
                            height: t.h,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING
                            | wgpu::TextureUsages::COPY_DST
                            | wgpu::TextureUsages::COPY_SRC,
                        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
                    })
                })
                .collect();
        }
        enc.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: shown,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &self.present[slot],
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: t.w,
                height: t.h,
                depth_or_array_layers: 1,
            },
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
                wgpu::TexelCopyTextureInfo {
                    texture: &t.qdepth,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::DepthOnly,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buf,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some((t.w * 4).next_multiple_of(256)),
                        rows_per_image: Some(t.h),
                    },
                },
                wgpu::Extent3d {
                    width: t.w,
                    height: t.h,
                    depth_or_array_layers: 1,
                },
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
            if let Some((prev, prev_idx, pw, ph, zs, lw, lh, mapped, prev_frame)) =
                self.pending_depth.take()
            {
                let slice = prev.slice(..);
                if cfg!(target_arch = "wasm32") {
                    let _ = self.dev.poll(wgpu::PollType::Poll);
                } else {
                    let _ = self.dev.poll(wgpu::PollType::Wait {
                        submission_index: Some(prev_idx),
                        timeout: None,
                    });
                }
                // (Not mapped yet in the browser: the queries keep the
                // depth before.)
                if let Some(v) = mapped
                    .load(std::sync::atomic::Ordering::Acquire)
                    .then(|| slice.get_mapped_range())
                    .and_then(|r| r.ok())
                {
                    // In GS pixels: the sample at each pixel's centre,
                    // converted when a query reads it.
                    let raw = v.to_vec();
                    let row = (pw * 4).next_multiple_of(256) as usize;
                    let zs = zs as f64;
                    let z = move |x: u32, y: u32| {
                        let py = (((y as f32 + 0.5) * ph as f32 / lh as f32) as usize)
                            .min(ph as usize - 1);
                        let pxl = (((x as f32 + 0.5) * pw as f32 / lw as f32) as usize)
                            .min(pw as usize - 1);
                        let o = py * row + 4 * pxl;
                        let d = f32::from_le_bytes([raw[o], raw[o + 1], raw[o + 2], raw[o + 3]]);
                        (d as f64 * zs).round().min(zs - 1.0) as u32
                    };
                    *self.shared.depth.lock().unwrap() = Some((
                        prev_frame,
                        ssx3_ps2::frame_sink::Depth {
                            w: lw,
                            h: lh,
                            z: Box::new(z),
                        },
                    ));
                    self.shared.depth_ready.notify_all();
                }
            }
            let mapped = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let m = mapped.clone();
            buf.slice(..).map_async(wgpu::MapMode::Read, move |r| {
                m.store(r.is_ok(), std::sync::atomic::Ordering::Release)
            });
            self.pending_depth = Some((
                buf,
                sub_idx,
                t.w,
                t.h,
                self.zscale,
                w,
                h,
                mapped,
                self.frames_in - 1,
            ));
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
                        let (r, g, b, a) =
                            (px & 0xff, (px >> 8) & 0xff, (px >> 16) & 0xff, px >> 24);
                        eprintln!(
                            "[probe] item {k}: rgba {r} {g} {b} a{:.0} <- {d}",
                            a as f32 * 128.0 / 255.0
                        );
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
            let out = (0..h)
                .flat_map(|y| px[(y * rw) as usize..(y * rw + w) as usize].iter().copied())
                .collect();
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
            wgpu::TexelCopyTextureInfo {
                texture: tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row),
                    rows_per_image: Some(t.h),
                },
            },
            wgpu::Extent3d {
                width: t.w,
                height: t.h,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit(std::iter::once(enc.finish()));
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.dev.poll(wgpu::PollType::wait_indefinitely());
        let Ok(v) = slice.get_mapped_range() else {
            return (w, h, Vec::new());
        };
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

/// One chunk's result: (event index, GIF output) per call with a native
/// program, and the VU memory after the chunk.
type VuChunk = (Vec<(usize, CallOut)>, Box<[u8; 16384]>);

/// A frame's VU1 calls split into chunks run by worker threads. Chunks
/// start only where a draw of a different program begins (programs pass
/// state within a draw, and terrain patches across draws), from the VU
/// memory with every earlier call's inputs applied. Checked against a
/// sequential run with `SSX3_GFX_VUCHECK`; `SSX3_GFX_SEQ_VU=1` forces one
/// chunk.
struct VuPlan<'a> {
    vu0: Box<[u8; 16384]>,
    /// The frame's VU calls and captured draws, in order.
    calls: Vec<ssx3_render::vujob::JobItem<'a>>,
    bounds: Vec<usize>,
    starts: Vec<std::sync::Mutex<Option<Box<[u8; 16384]>>>>,
    done: Vec<(std::sync::Mutex<Option<VuChunk>>, std::sync::Condvar)>,
    /// Event index → chunk.
    chunk_of: HashMap<usize, usize>,
    next: std::sync::atomic::AtomicUsize,
    threads: usize,
}

impl VuPlan<'_> {
    fn run(chunk: &[ssx3_render::vujob::JobItem], mut mem: Box<[u8; 16384]>) -> VuChunk {
        let mut outs = Vec::with_capacity(chunk.len());
        for it in chunk {
            let &ssx3_render::vujob::JobItem::Call((i, c, hash, kept)) = it else {
                if let Some(o) = it.run(&mut mem) {
                    outs.push((it.event(), o));
                }
                continue;
            };
            for &(a, v) in c.writes() {
                let a = (a as usize & 1023) * 16;
                mem[a..a + 16].copy_from_slice(&v.to_le_bytes());
            }
            let (Some(entry), Some(hh)) = (c.entry, hash) else {
                continue;
            };
            if let Some(o) =
                ssx3_render::vujob::run_call(hh, entry, c.top, c.itop, c.src, kept, &mut mem)
            {
                outs.push((i, o));
            }
        }
        (outs, mem)
    }

    /// Worker: take chunks until none are left.
    fn work(&self) {
        loop {
            let j = self.next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let Some(start) = self.starts.get(j) else {
                break;
            };
            let st = start.lock().unwrap().take().expect("chunk");
            let e = self.bounds.get(j + 1).copied().unwrap_or(self.calls.len());
            let r = Self::run(&self.calls[self.bounds[j]..e], st);
            let (slot, cv) = &self.done[j];
            *slot.lock().unwrap() = Some(r);
            cv.notify_all();
        }
    }

    /// Whether the chunks go to the installed [`VuOffload`] (the browser's
    /// helper workers): not without one.
    fn offloadable(&self) -> bool {
        OFFLOAD.get().is_some() && self.bounds.len() >= 2
    }

    /// Every chunk, encoded (takes the start images).
    fn jobs(&self) -> Vec<Vec<u8>> {
        let mut store = STORE.get().map(|s| s.lock().unwrap());
        (0..self.bounds.len())
            .map(|j| {
                let start = self.starts[j].lock().unwrap().take().expect("chunk");
                let e = self.bounds.get(j + 1).copied().unwrap_or(self.calls.len());
                let st = store
                    .as_mut()
                    .map(|s| &mut ***s as &mut dyn ssx3_render::vujob::SharedStore);
                ssx3_render::vujob::encode_with(&start, &self.calls[self.bounds[j]..e], st)
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
        let pre = PREFETCH
            .lock()
            .unwrap()
            .take_if(|(n, _)| *n == self.bounds.len());
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
    outs: HashMap<usize, CallOut>,
    last_mem: Option<Box<[u8; 16384]>>,
    /// `SSX3_GFX_VUCHECK`: every output, for the comparison in `finish`.
    seen: Option<HashMap<usize, CallOut>>,
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

    /// The output of the call at event `i` (None: no native program).
    fn take(&mut self, plan: &VuPlan, i: usize) -> Option<CallOut> {
        let j = *plan.chunk_of.get(&i)?;
        self.merge_to(plan, j);
        let out = self.outs.remove(&i);
        if plan.bounds.len() > 1 && debug::get().vucheck {
            if let Some(o) = &out {
                self.seen
                    .get_or_insert_with(HashMap::new)
                    .insert(i, o.clone());
            }
        }
        out
    }

    /// Wait for every chunk; `vu` becomes the memory after the last.
    fn finish(&mut self, plan: &VuPlan, vu: &mut Box<[u8; 16384]>) {
        let check = debug::get().vucheck && plan.bounds.len() > 1;
        let mut all = if check {
            Some(self.seen.take().unwrap_or_default())
        } else {
            None
        };
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
            let (seq, _) = VuPlan::run(&plan.calls, plan.vu0.clone());
            let bad = seq
                .iter()
                .filter(|(i, out)| all.get(i).is_some_and(|o| o != out))
                .count();
            eprintln!(
                "[vucheck] {bad} of {} calls differ from a sequential run ({} chunks)",
                seq.len(),
                n
            );
            for (i, out) in seq
                .iter()
                .filter(|(i, out)| all.get(i).is_some_and(|o| o != out))
                .take(3)
            {
                let kind = |o: &CallOut| match o {
                    CallOut::Gif(q) => format!("gif {}", q.len()),
                    CallOut::Gpu(g) => format!("gpu {}", g.len()),
                    CallOut::Native(v) => format!("native {}", v.len()),
                };
                let item = plan
                    .calls
                    .iter()
                    .find(|c| c.event() == *i)
                    .map(|c| match c {
                        ssx3_render::vujob::JobItem::Call((_, c, h, _)) => {
                            format!("call {:x?} {:x?} top {:#x}", h, c.entry, c.top)
                        }
                        ssx3_render::vujob::JobItem::Native { .. } => "native".into(),
                    });
                eprintln!(
                    "[vucheck]   event {i} {item:?}: sequential {}, chunked {}",
                    kind(out),
                    kind(&all[i])
                );
            }
        }
    }
}

/// What building one image's geometry from a frame uses and changes: the
/// GS state the frame starts from, VU memory, counters, and the frame's
/// facts the drawing needs.
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
        // (A frame built without the chains carries its PATH3 packets.)
        // (Each packet with the post pass it belongs to, if any.)
        let tagged = |ph: usize,
                      v: &Vec<Vec<u128>>|
         -> Vec<(Vec<u128>, Option<ssx3_render::direct::PostKind>)> {
            v.iter()
                .enumerate()
                .map(|(k, q)| (q.clone(), built.path3_post[ph].get(k).copied().flatten()))
                .collect()
        };
        let (p1, mut p2) = match &built.path3 {
            Some([a, c]) => (tagged(0, a), tagged(1, c)),
            None => Default::default(),
        };
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
        let flip = if !dbg.no_flip {
            built.flip.clone()
        } else {
            Vec::new()
        };
        let split = flip
            .iter()
            .position(|&q| q as u64 == 0x1000_0000_0000_8010)
            .unwrap_or(0);
        label(&mut b.gs, "flip".into());
        b.gs.gif(&flip[split..]);
        b.display_fbp = (split > 0)
            .then(|| d.draw_fbp + (d.fb_width.div_ceil(64) * d.fb_height.div_ceil(32)).max(1));
        let mode = dbg.path3;
        let mut p3: std::collections::VecDeque<(Vec<u128>, Option<ssx3_render::direct::PostKind>)> =
            std::collections::VecDeque::new();
        // A PATH3 packet through the GS, tagged with its post pass.
        let path3 =
            |gs: &mut gs::Gs, (pk, post): (Vec<u128>, Option<ssx3_render::direct::PostKind>)| {
                let hud = std::mem::replace(&mut gs.hud, false);
                let tag = std::mem::replace(
                    &mut gs.tag,
                    gs::Tag {
                        post,
                        ..Default::default()
                    },
                );
                gs.gif(&pk);
                gs.hud = hud;
                gs.tag = tag;
            };
        if mode & 1 != 0 {
            p3.extend(p1);
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
                            path3(&mut b.gs, pk);
                        }
                    }
                    b.phase1_end = b.gs.batches.len();
                    p3_masked = true;
                    if mode & 2 != 0 {
                        p3.extend(std::mem::take(&mut p2));
                    }
                }
                let n = match owner {
                    Owner::Prelude(n) | Owner::Payload(n) => Some(*n),
                    _ => None,
                };
                // The batches' tag: the draw's material (none between draws).
                let m = n.and_then(|n| draws.get(n)).map(|d| d.state.material);
                b.gs.tag = gs::Tag {
                    terrain: matches!(m, Some(Material::Bezier1 | Material::Bezier2)),
                    ..Default::default()
                };
                if let Some(n) = n {
                    if cur != Some(n) {
                        cur = Some(n);
                        b.gs.hud = draws.get(n).is_some_and(|d| d.state.layer >= 9);
                        if let Some(d) = draws.get(n) {
                            if tracing {
                                label(
                                    &mut b.gs,
                                    format!(
                                        "draw{n} {:?} layer {} tex {:?}",
                                        d.state.material, d.state.layer, d.tex
                                    ),
                                );
                            }
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
                            Some(CallOut::Gif(out)) => {
                                b.stats.vu_native += 1;
                                b.gs.gif(&out);
                            }
                            Some(CallOut::Gpu(g)) => {
                                b.stats.vu_native += 1;
                                b.stats.vu_gpu += 1;
                                for c in g {
                                    b.gs.gpu_draw(c);
                                }
                            }
                            Some(CallOut::Native(_)) | None => b.stats.vu_missing += 1,
                        }
                    }
                    Event::Direct(dd) => b.gs.gif(&dd.qwords),
                    Event::Native { .. } => {
                        // A draw captured at the game's draw function, planned
                        // from its command with the VU calls.
                        match vu.take(&plan, i) {
                            Some(CallOut::Native(outs)) => {
                                b.stats.native_draws += 1;
                                for o in outs {
                                    match o {
                                        ssx3_render::direct::Out::Gpu(c) => b.gs.gpu_draw(c),
                                        ssx3_render::direct::Out::Gif(g) => b.gs.gif(&g),
                                    }
                                }
                            }
                            _ => b.stats.vu_missing += 1,
                        }
                    }
                    Event::MaskPath3(m) => {
                        if !m {
                            if let Some(pk) = p3.pop_front() {
                                label(&mut b.gs, "path3".into());
                                path3(&mut b.gs, pk);
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
                    p3.extend(std::mem::take(&mut p2));
                }
            }
            vu.finish(&plan, &mut b.vu);
        });
        // Whatever is left runs before the next frame.
        label(&mut b.gs, "path3".into());
        while let Some(pk) = p3.pop_front() {
            path3(&mut b.gs, pk);
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
            eprintln!(
                "[gfx] frame {} traced: {} lines -> {path}",
                b.frame,
                log.len()
            );
        }
    }

    fn plan_vu_calls<'a>(b: &Build, built: &'a Built, draws: &[&frame::Draw]) -> VuPlan<'a> {
        // Each call's program: the material of the draw it belongs to.
        let mut calls: Vec<ssx3_render::vujob::JobItem> = Vec::new();
        let mut last_hash = None;
        // Mesh data the GPU keeps already: the plan leaves it out (the
        // arena only grows until the next frame's plan).
        let mut arena = b.gs.gpu.arena.lock().unwrap();
        arena.limit();
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
                draw_start.push(cur != last_draw && (calls.is_empty() || last_hash != hash));
                last_hash = hash;
                last_draw = cur;
                let program = match hash {
                    Some(ssx3_render::mat3::HASH) => Some(ssx3_render::gpu::Program::Mat3Mesh),
                    Some(ssx3_render::mat6::HASH) => Some(ssx3_render::gpu::Program::Mat6Skin),
                    _ => None,
                };
                let kept = c
                    .src
                    .zip(program)
                    .and_then(|(s, p)| arena.kept(ssx3_render::gpu::mesh_key(s, p)));
                calls.push(ssx3_render::vujob::JobItem::Call((i, c, hash, kept)));
            } else if let Event::Native {
                index,
                regs,
                header,
            } = ev
            {
                // (Chunks start at them as at calls.)
                if let Some(r) = built.direct.get(*index as usize) {
                    draw_start.push(cur != last_draw && (calls.is_empty() || last_hash != hash));
                    last_hash = hash;
                    last_draw = cur;
                    calls.push(ssx3_render::vujob::JobItem::Native {
                        event: i,
                        cmd: &r.cmd,
                        regs: *regs,
                        header,
                    });
                }
            }
        }
        static SEQ: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let seq = *SEQ.get_or_init(|| std::env::var_os("SSX3_GFX_SEQ_VU").is_some());
        let parallel = OFFLOAD
            .get()
            .map(|o| o.1)
            .unwrap_or_else(|| std::thread::available_parallelism().map_or(4, |n| n.get()));
        let threads = if seq || calls.len() < 64 {
            1
        } else {
            parallel.clamp(1, 8)
        };
        // About four chunks per thread, taken by whichever thread is free.
        let per = calls.len().div_ceil(threads * 4).max(1);
        // Chunk boundaries: the first draw start at or after each multiple
        // of `per`; the start images have every earlier call's inputs applied.
        let mut bounds: Vec<usize> = Vec::new();
        let mut starts = Vec::new();
        let mut img = b.vu.clone();
        drop(arena);
        for (k, it) in calls.iter().enumerate() {
            if k == 0 || (k >= bounds.last().unwrap() + per && draw_start[k] && threads > 1) {
                bounds.push(k);
                starts.push(std::sync::Mutex::new(Some(img.clone())));
            }
            if let (true, ssx3_render::vujob::JobItem::Call((_, c, _, _))) = (threads > 1, it) {
                for &(a, v) in c.writes() {
                    let a = (a as usize & 1023) * 16;
                    img[a..a + 16].copy_from_slice(&v.to_le_bytes());
                }
            }
        }
        if bounds.is_empty() {
            bounds.push(0);
            starts.push(std::sync::Mutex::new(Some(b.vu.clone())));
        }
        let chunk_of: HashMap<usize, usize> = calls
            .iter()
            .enumerate()
            .map(|(k, c)| (c.event(), bounds.partition_point(|&b| b <= k) - 1))
            .collect();
        let done = (0..bounds.len())
            .map(|_| (std::sync::Mutex::new(None), std::sync::Condvar::new()))
            .collect();
        VuPlan {
            vu0: b.vu.clone(),
            calls,
            bounds,
            starts,
            done,
            chunk_of,
            next: std::sync::atomic::AtomicUsize::new(0),
            threads,
        }
    }
}

/// Waits for the results of started VU1 chunks, in order.
pub type VuCollect = Box<dyn FnOnce() -> Vec<Vec<u8>> + Send>;

/// Starts encoded chunks of VU1 calls ([`ssx3_render::vujob`]) elsewhere
/// (the browser build's helper workers) and returns how to wait for them.
pub type VuOffload = Box<dyn Fn(Vec<Vec<u8>>) -> VuCollect + Send + Sync>;

static OFFLOAD: std::sync::OnceLock<(VuOffload, usize)> = std::sync::OnceLock::new();

/// Where the offload keeps replayed calls' shared input
/// (`ssx3_render::vujob::SharedStore`), if anywhere.
static STORE: std::sync::OnceLock<Mutex<Box<dyn ssx3_render::vujob::SharedStore + Send>>> =
    std::sync::OnceLock::new();

/// Install the offload's store of shared input (once).
pub fn set_vu_store(store: Box<dyn ssx3_render::vujob::SharedStore + Send>) {
    let _ = STORE.set(Mutex::new(store));
}

/// The staged frame's started chunks (chunk count, results), for its
/// `VuPlan::offload` when it is drawn.
static PREFETCH: Mutex<Option<(usize, VuCollect)>> = Mutex::new(None);

/// A frame published but not drawn yet (`Inner::publish_staged`).
struct Staged {
    f: ssx3_ps2::frame_sink::Published,
    collect: Option<(usize, VuCollect)>,
}

/// Run one encoded job (what an offload does with each).
pub fn run_vu_job(job: &[u8], store: &[u8]) -> Vec<u8> {
    ssx3_render::vujob::run(job, store)
}

/// Install the VU offload, which runs `workers` jobs at a time (the frame's
/// calls are split for that many).
pub fn set_vu_offload(f: VuOffload, workers: usize) {
    let _ = OFFLOAD.set((f, workers.max(1)));
}

/// `slot`'s buffer if it holds `size` bytes, else a new one (twice as big as
/// needed, so it rarely grows again).
fn grow_buffer(
    dev: &wgpu::Device,
    slot: &mut Option<wgpu::Buffer>,
    size: u64,
    usage: wgpu::BufferUsages,
    label: &str,
) -> wgpu::Buffer {
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
