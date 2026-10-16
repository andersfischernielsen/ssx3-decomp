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

pub mod draw;
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
}

struct CachedTex {
    view: wgpu::TextureView,
    used: u64,
}

pub struct Inner {
    dev: wgpu::Device,
    queue: wgpu::Queue,
    pipes: draw::Pipelines,
    targets: Option<Targets>,
    gs: gs::Gs,
    vu: Box<[u8; 16384]>,
    texcache: HashMap<tex::TexKey, CachedTex>,
    samplers: HashMap<draw::SampKey, wgpu::Sampler>,
    white: wgpu::TextureView,
    pub stats: Stats,
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
        Renderer(Arc::new(Mutex::new(Inner {
            dev,
            queue,
            pipes,
            targets: None,
            gs: gs::Gs::new(),
            vu: Box::new([0; 16384]),
            texcache: HashMap::new(),
            samplers: HashMap::new(),
            white,
            stats: Stats::default(),
        })))
    }

    /// The master colour target as RGBA8 (GS alpha), read back.
    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)> {
        self.0.lock().unwrap().read_image()
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
        let depth = mk("ssx3-gfx depth", draw::DEPTH_FORMAT, wgpu::TextureUsages::empty());
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
        self.targets = Some(Targets { w, h, color, depth });
    }

    /// Build the geometry of one frame.
    fn build(&mut self, built: &Built, mem: &dyn Mem) {
        let fr = &built.frame;
        let d = &fr.display;
        let (w, h) = (d.fb_width.clamp(64, 2048), d.fb_height.clamp(64, 2048));
        // FRAME.FBP is in 2048-word units, gm's draw_fbp likewise.
        self.gs.begin((w, h), d.draw_fbp, d.zbp);
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
        // The PATH3 chains (GifBuff): phase 1 before the VIF phase-1
        // draws, phase 2 (post passes, display blit) before the first
        // draw of layer 9 or above.
        let path3 = |head: u32| -> Vec<u128> {
            let mut q = Vec::new();
            if head != 0 {
                let mut asr = [0u32; 2];
                ssx3_render::chain::walk(mem, head, false, false, 0, &mut asr, &mut |d: &[u128]| q.extend_from_slice(d));
            }
            q
        };
        let (p1, p2) = (path3(fr.gif_heads[0]), path3(fr.gif_heads[1]));
        let tracing = std::env::var("SSX3_GFX_TRACE").ok().and_then(|v| v.parse::<u64>().ok()) == Some(self.stats.frames);
        let label = |gs: &mut gs::Gs, l: String| {
            if tracing {
                gs.trace = Some(l);
            }
        };
        label(&mut self.gs, "path3-1".into());
        let mode = std::env::var("SSX3_GFX_PATH3").ok().and_then(|v| v.parse::<u32>().ok()).unwrap_or(3);
        if mode & 1 != 0 {
            self.gs.gif(&p1);
        }
        let mut p2_done = mode & 2 == 0;
        let mut cur: Option<usize> = None;
        let mut hash: Option<u64> = None;
        for (owner, ev) in &built.events {
            let n = match owner {
                Owner::Prelude(n) | Owner::Payload(n) => Some(*n),
                _ => None,
            };
            if let Some(n) = n {
                if !p2_done && draws.get(n).is_some_and(|d| d.state.layer >= 9) {
                    p2_done = true;
                    label(&mut self.gs, "path3-2".into());
                    self.gs.gif(&p2);
                }
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
        if !p2_done {
            label(&mut self.gs, "path3-2".into());
            self.gs.gif(&p2);
        }
        if tracing {
            self.gs.trace = None;
            let log = std::mem::take(&mut self.gs.trace_log);
            let path = std::env::var("SSX3_GFX_TRACE_OUT").unwrap_or_else(|_| "/tmp/gfx-trace.txt".into());
            let _ = std::fs::write(&path, log.join("\n"));
            eprintln!("[gfx] frame {} traced: {} lines -> {path}", self.stats.frames, log.len());
        }
    }

    fn texture(&mut self, info: u32, texa: u64, mem: &dyn Mem) -> Option<wgpu::TextureView> {
        let key = tex::key(mem, info, texa);
        let frame = self.stats.frames;
        if let Some(t) = self.texcache.get_mut(&key) {
            t.used = frame;
            return Some(t.view.clone());
        }
        let Some(d) = tex::decode(mem, info, texa) else {
            if self.stats.textures == 0 && self.stats.frames % 300 == 1 {
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
        let view = t.create_view(&Default::default());
        self.texcache.insert(key, CachedTex { view: view.clone(), used: frame });
        Some(view)
    }

    pub fn render(&mut self, built: &Built, mem: &dyn Mem) {
        self.stats.frames += 1;
        let p0 = (self.gs.prims, self.gs.dropped, self.gs.unknown_tex);
        self.build(built, mem);
        let (w, h) = self.gs.size;
        self.ensure_targets(w, h);
        self.stats.prims += self.gs.prims - p0.0;
        self.stats.dropped += self.gs.dropped - p0.1;
        self.stats.unknown_tex += self.gs.unknown_tex - p0.2;
        if self.gs.verts.is_empty() {
            return;
        }
        // Vertex buffer.
        let vbytes: Vec<u8> = self
            .gs
            .verts
            .iter()
            .flat_map(|v| {
                let f = [v.pos[0], v.pos[1], v.pos[2], v.fog, v.col[0], v.col[1], v.col[2], v.col[3], v.stq[0], v.stq[1], v.stq[2], v.stq[3]];
                f.into_iter().flat_map(|x| x.to_le_bytes())
            })
            .collect();
        let vbuf = self.dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx verts"),
            size: vbytes.len() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&vbuf, 0, &vbytes);
        // Plans per batch, uniforms, bind groups.
        let batches = std::mem::take(&mut self.gs.batches);
        self.stats.batches += batches.len() as u64;
        struct Item {
            key: draw::PipeKey,
            ubo: u32,
            constant: f64,
            bind: usize,
            first: u32,
            count: u32,
            scissor: Option<[u32; 4]>,
        }
        let mut items = Vec::new();
        let mut ubytes: Vec<u8> = Vec::new();
        let mut binds: Vec<(Option<wgpu::TextureView>, draw::SampKey)> = Vec::new();
        let mut bind_ix: HashMap<(Option<u32>, u64, draw::SampKey), usize> = HashMap::new();
        for b in &batches {
            let st = &b.state;
            let sk = draw::samp_key(st);
            let bkey = (st.tex, st.texa, sk);
            let bind = match bind_ix.get(&bkey) {
                Some(&i) => i,
                None => {
                    let view = st.tex.and_then(|info| self.texture(info, st.texa, mem));
                    binds.push((view, sk));
                    bind_ix.insert(bkey, binds.len() - 1);
                    binds.len() - 1
                }
            };
            let sc = st.ctx.scissor;
            let (x0, x1, y0, y1) = ((sc & 0x7ff) as u32, ((sc >> 16) & 0x7ff) as u32, ((sc >> 32) & 0x7ff) as u32, ((sc >> 48) & 0x7ff) as u32);
            let scissor = (x0 <= x1 && y0 <= y1 && x0 < w && y0 < h).then(|| [x0, y0, (x1 + 1).min(w) - x0, (y1 + 1).min(h) - y0]);
            if scissor.is_none() {
                continue;
            }
            for p in draw::plans(st, (w, h)) {
                let ubo = (ubytes.len() as u64 / draw::UNIFORM_STRIDE) as u32;
                let u = &p.u;
                let words: [u32; 16] = [
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
                ];
                ubytes.extend(words.iter().flat_map(|w| w.to_le_bytes()));
                ubytes.resize(ubytes.len().next_multiple_of(draw::UNIFORM_STRIDE as usize), 0);
                items.push(Item { key: p.key, ubo, constant: p.constant, bind, first: b.first, count: b.count, scissor });
            }
        }
        if items.is_empty() {
            return;
        }
        let ubuf = self.dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx uniforms"),
            size: ubytes.len() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&ubuf, 0, &ubytes);
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
                                size: wgpu::BufferSize::new(64),
                            }),
                        },
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(view.as_ref().unwrap_or(&self.white)) },
                        wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&samp) },
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
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssx3-gfx frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &cv,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &dv,
                    // Each frame starts with depth cleared to the farthest
                    // value (the GS's Z history across frames is not kept).
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_vertex_buffer(0, vbuf.slice(..));
            for it in &items {
                let Some([x, y, sw, sh]) = it.scissor else { continue };
                pass.set_pipeline(self.pipes.cached(it.key));
                pass.set_bind_group(0, &groups[it.bind], &[it.ubo * draw::UNIFORM_STRIDE as u32]);
                pass.set_blend_constant(wgpu::Color { r: it.constant, g: it.constant, b: it.constant, a: it.constant });
                pass.set_scissor_rect(x, y, sw, sh);
                pass.draw(it.first..it.first + it.count, 0..1);
            }
        }
        self.queue.submit(std::iter::once(enc.finish()));
        // Drop textures unused for a while.
        let frame = self.stats.frames;
        self.texcache.retain(|_, t| frame - t.used < 600);
    }

    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)> {
        let t = self.targets.as_ref()?;
        let row = (t.w * 4).next_multiple_of(256);
        let buf = self.dev.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ssx3-gfx read"),
            size: row as u64 * t.h as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut enc = self.dev.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &t.color, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(t.h) } },
            wgpu::Extent3d { width: t.w, height: t.h, depth_or_array_layers: 1 },
        );
        self.queue.submit(std::iter::once(enc.finish()));
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.dev.poll(wgpu::PollType::wait_indefinitely());
        let v = slice.get_mapped_range().ok()?;
        let mut px = Vec::with_capacity((t.w * t.h) as usize);
        for y in 0..t.h as usize {
            for x in 0..t.w as usize {
                let o = y * row as usize + x * 4;
                px.push(u32::from_le_bytes([v[o], v[o + 1], v[o + 2], v[o + 3]]));
            }
        }
        Some((t.w, t.h, px))
    }
}
