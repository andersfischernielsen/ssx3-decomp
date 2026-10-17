//! SSX 3 in the browser. Runs in a Web Worker (`www/worker.js`): the disc
//! image is a `File` the page hands over, read synchronously with
//! `FileReaderSync` (only workers have it); frames are drawn by `ssx3-gfx`
//! with WebGPU into an `OffscreenCanvas` transferred from the page. The EE
//! code is the unwind form (`generated/ee-unwind`): one host thread, so a
//! game frame (`Game::frame`) returns to the browser like any other call.
//!
//! What the native window has and this has not (yet): sound (the mixer
//! runs, nothing plays it), the memory card, gamepads, the settings overlay
//! and interpolation.
#![cfg(target_arch = "wasm32")]

extern crate ssx3_ee_unwind as ssx3_ee;

use std::sync::Arc;

use ssx3_core::iso::{self, Source};
use ssx3_ps2::Cpu;
use wasm_bindgen::prelude::*;

#[path = "../../ssx3/src/game.rs"]
mod game;

/// The settings file of the native front end: none here, every setting is
/// its default.
mod settings {
    pub fn get(_key: &str) -> Option<String> {
        None
    }
}

fn log(s: &str) {
    web_sys::console::log_1(&s.into());
}

/// The disc image: byte ranges of the `File`, read synchronously.
struct BlobSource {
    blob: web_sys::Blob,
    reader: web_sys::FileReaderSync,
}

// SAFETY: a wasm32 build without atomics has one thread.
unsafe impl Send for BlobSource {}

impl Source for BlobSource {
    fn len(&self) -> u64 {
        self.blob.size() as u64
    }

    fn read_at(&mut self, off: u64, buf: &mut [u8]) -> std::io::Result<()> {
        let err = |e: JsValue| std::io::Error::other(format!("{e:?}"));
        let part = self.blob.slice_with_f64_and_f64(off as f64, (off + buf.len() as u64) as f64).map_err(err)?;
        let ab = self.reader.read_as_array_buffer(&part).map_err(err)?;
        let a = js_sys::Uint8Array::new(&ab);
        if a.length() as usize != buf.len() {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        a.copy_to(buf);
        Ok(())
    }
}

struct SendBlob(web_sys::Blob);
// SAFETY: as for `BlobSource`.
unsafe impl Send for SendBlob {}
unsafe impl Sync for SendBlob {}

/// The VU helper workers' shared buffer (`www/vuworker.js`). Int32 header:
/// [0] generation, [1] job count, [2] next job (generation << 16 | index),
/// [3] jobs done, [4] output base, [5] arena bytes per helper; from int 16,
/// per job: input offset, length, output offset (-1: did not fit), length.
/// Inputs follow the header; each helper writes its outputs to its arena.
mod vu_pool {
    use js_sys::{Atomics, Int32Array, SharedArrayBuffer, Uint8Array};

    const HEADER: u32 = 65536;
    const TABLE: u32 = 16;
    const MAX_JOBS: usize = ((HEADER / 4 - TABLE) / 4) as usize;
    const INPUTS: u32 = 32 << 20;
    const ARENA: u32 = 8 << 20;

    pub struct Pool {
        pub sab: SharedArrayBuffer,
        i32: Int32Array,
        u8: Uint8Array,
    }

    // SAFETY: a wasm32 build without atomics has one thread; the helpers
    // only touch the buffer through their own JS views.
    unsafe impl Send for Pool {}
    unsafe impl Sync for Pool {}

    impl Pool {
        pub fn new(helpers: u32) -> Option<Pool> {
            let sab = SharedArrayBuffer::new(HEADER + INPUTS + helpers * ARENA);
            let (i32, u8) = (Int32Array::new(&sab), Uint8Array::new(&sab));
            i32.set_index(4, (HEADER + INPUTS) as i32);
            i32.set_index(5, ARENA as i32);
            Some(Pool { sab, i32, u8 })
        }

        fn claim(&self, tag: i32, n: i32) -> Option<usize> {
            loop {
                let v = Atomics::load(&self.i32, 2).ok()?;
                if v & !0xffff != tag || (v & 0xffff) >= n {
                    return None;
                }
                if Atomics::compare_exchange(&self.i32, 2, v, v + 1).ok()? == v {
                    return Some((v & 0xffff) as usize);
                }
            }
        }

        /// Run `jobs` on the helpers and on this thread; results in order.
        pub fn run(&self, jobs: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
            let local = |j: &[u8]| ssx3_gfx::run_vu_job(j);
            let size: usize = jobs.iter().map(|j| j.len()).sum();
            if jobs.len() > MAX_JOBS || size > INPUTS as usize {
                return jobs.iter().map(|j| local(j)).collect();
            }
            let n = jobs.len() as i32;
            let mut off = HEADER;
            for (j, job) in jobs.iter().enumerate() {
                self.u8.subarray(off, off + job.len() as u32).copy_from(job);
                let t = TABLE + 4 * j as u32;
                self.i32.set_index(t, off as i32);
                self.i32.set_index(t + 1, job.len() as i32);
                self.i32.set_index(t + 2, -1);
                off += job.len() as u32;
            }
            let gen = (self.i32.get_index(0) + 1) & 0x7fff;
            let tag = gen << 16;
            let _ = Atomics::store(&self.i32, 1, n);
            let _ = Atomics::store(&self.i32, 3, 0);
            let _ = Atomics::store(&self.i32, 2, tag);
            let _ = Atomics::store(&self.i32, 0, gen);
            let _ = Atomics::notify(&self.i32, 0);
            let mut out: Vec<Option<Vec<u8>>> = vec![None; jobs.len()];
            while let Some(j) = self.claim(tag, n) {
                out[j] = Some(local(&jobs[j]));
                let _ = Atomics::add(&self.i32, 3, 1);
            }
            loop {
                let d = Atomics::load(&self.i32, 3).unwrap_or(n);
                if d >= n {
                    break;
                }
                let _ = Atomics::wait(&self.i32, 3, d);
            }
            out.into_iter()
                .enumerate()
                .map(|(j, r)| {
                    r.unwrap_or_else(|| {
                        let t = TABLE + 4 * j as u32;
                        let (o, len) = (self.i32.get_index(t + 2), self.i32.get_index(t + 3));
                        if o < 0 {
                            return local(&jobs[j]);
                        }
                        let mut v = vec![0u8; len as usize];
                        self.u8.subarray(o as u32, (o + len) as u32).copy_to(&mut v);
                        v
                    })
                })
                .collect()
        }
    }
}

/// Draws the renderer's image (an sRGB view) to the canvas.
const BLIT: &str = r#"
@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
struct VsOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i: u32) -> VsOut {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var o: VsOut;
    o.pos = vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(p.x, 1.0 - p.y);
    return o;
}
@fragment fn fs(v: VsOut) -> @location(0) vec4<f32> { return vec4<f32>(textureSample(tex, smp, v.uv).rgb, 1.0); }
"#;

#[wasm_bindgen]
pub struct Game {
    cpu: Box<Cpu>,
    gfx: ssx3_gfx::Renderer,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    view_format: wgpu::TextureFormat,
    pipeline: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    started: f64,
    /// ms in `run_frame` (game and, in it, the renderer) and in `present`
    /// since the last report.
    ms_frame: f64,
    ms_present: f64,
    /// The VU helper workers' buffer (none without cross-origin isolation).
    vu_buffer: Option<js_sys::SharedArrayBuffer>,
}

fn now_ms() -> f64 {
    js_sys::Date::now()
}

/// Boot the game: `canvas` is drawn to, `iso` is the disc image, `helpers`
/// the number of VU helper workers the page will start (0: none; needs
/// cross-origin isolation for the shared buffer).
#[wasm_bindgen]
pub async fn start(canvas: web_sys::OffscreenCanvas, iso: web_sys::Blob, helpers: u32) -> Result<Game, JsValue> {
    console_error_panic_hook::set_once();
    let blob = Arc::new(SendBlob(iso));
    iso::set_opener(Box::new(move || {
        let reader = web_sys::FileReaderSync::new().map_err(|e| std::io::Error::other(format!("FileReaderSync: {e:?}")))?;
        Ok(Box::new(BlobSource { blob: blob.0.clone(), reader }) as Box<dyn Source>)
    }));
    let ctx = ssx3_ps2::gs::gpu::init_context().await.ok_or("WebGPU is not available")?;
    let (w, h) = (canvas.width().max(64), canvas.height().max(64));
    let surface = ctx
        .instance
        .create_surface(wgpu::SurfaceTarget::OffscreenCanvas(canvas))
        .map_err(|e| format!("surface: {e}"))?;
    let caps = surface.get_capabilities(&ctx.adapter);
    let format = *caps.formats.first().ok_or("no surface format")?;
    let view_format = format.add_srgb_suffix();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: w,
        height: h,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![view_format],
        color_space: Default::default(),
    };
    surface.configure(&ctx.device, &config);
    let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("blit"), source: wgpu::ShaderSource::Wgsl(BLIT.into()) });
    let pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("blit"),
        layout: None,
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs"), buffers: &[], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs"),
            targets: &[Some(view_format.into())],
            compilation_options: Default::default(),
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    });
    let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    log("[ssx3] booting");
    let mut cpu = game::boot(false);
    let gfx = game::install_renderer(&mut cpu).ok_or("renderer: no GPU")?;
    let pool = (helpers > 0).then(|| vu_pool::Pool::new(helpers)).flatten();
    let vu_buffer = pool.as_ref().map(|p| p.sab.clone());
    if let Some(p) = pool {
        log(&format!("[ssx3] {helpers} VU helper workers"));
        ssx3_gfx::set_vu_offload(Box::new(move |jobs| p.run(jobs)), helpers as usize + 1);
    }
    let game = Game {
        cpu,
        gfx,
        device: ctx.device.clone(),
        queue: ctx.queue.clone(),
        surface,
        config,
        view_format,
        pipeline,
        sampler,
        started: now_ms(),
        ms_frame: 0.0,
        ms_present: 0.0,
        vu_buffer,
    };
    game.follow_size();
    Ok(game)
}

#[wasm_bindgen]
impl Game {
    /// The buffer to hand to the VU helper workers.
    pub fn vu_buffer(&self) -> Option<js_sys::SharedArrayBuffer> {
        self.vu_buffer.clone()
    }

    /// Render at the canvas size, the view widened to its aspect.
    fn follow_size(&self) {
        let (w, h) = (self.config.width, self.config.height);
        self.gfx.set_output_size(Some((w, h)));
        ssx3_ps2::frame_sink::set_view(w as f32 / h as f32, 1.0);
    }

    /// The canvas was resized (device pixels).
    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 || (w, h) == (self.config.width, self.config.height) {
            return;
        }
        self.config.width = w;
        self.config.height = h;
        self.surface.configure(&self.device, &self.config);
        self.follow_size();
    }

    /// Run one game frame with pad 1's buttons (libpad bits, pressed = 1).
    /// False when the game has stopped.
    pub fn frame(&mut self, buttons: u16) -> bool {
        ssx3_ps2::m!(self.cpu).sif.pad_buttons[0] = buttons;
        let t0 = now_ms();
        let alive = ssx3_ps2::machine::run_frame(&mut self.cpu);
        self.ms_frame += now_ms() - t0;
        let frames = ssx3_ps2::m!(self.cpu).sched.frames;
        if frames % 300 == 0 {
            log(&format!("[{:6.1}s] {}", (now_ms() - self.started) / 1e3, game::status(&self.cpu)));
            log(&format!(
                "  last 300 frames: {:.2} ms per frame in run_frame, {:.2} ms in present; {}; {}",
                self.ms_frame / 300.0,
                self.ms_present / 300.0,
                ssx3_ps2::frame_sink::timing_report(),
                self.gfx.0.lock().unwrap().timing_report()
            ));
            self.ms_frame = 0.0;
            self.ms_present = 0.0;
        }
        if !alive {
            log(&format!("halted: {:?}", ssx3_ps2::m!(self.cpu).kernel.halted));
        }
        alive
    }

    /// Draw the latest image to the canvas.
    pub fn present(&mut self) {
        let t0 = now_ms();
        self.present_image();
        self.ms_present += now_ms() - t0;
    }

    fn present_image(&mut self) {
        let Some((view, _, _)) = self.gfx.present_view_at(None) else { return };
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            _ => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };
        let target = frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.view_format), ..Default::default() });
        let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit"),
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let mut enc = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("blit"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(enc.finish()));
        self.queue.present(frame);
    }
}
