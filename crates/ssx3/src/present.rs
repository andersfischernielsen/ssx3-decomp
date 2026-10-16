//! `wgpu` presenter for frames produced by the GS (`Gs::display`).
//! Adapted from the VRAM presenter in `ssx3-gs`.
//!
//! With the GPU GS backend (`SSX3_GS=gpu`) the presenter uses the GS's own
//! wgpu device and draws the frame straight from the GS's display buffer
//! (`Gs::display_gpu`, `render_gpu`), so frames never round-trip through
//! the CPU.

use std::sync::Arc;

use winit::window::Window;

const SHADER: &str = r#"
@group(0) @binding(0) var frame_tex: texture_2d<f32>;
@group(0) @binding(1) var frame_smp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VsOut {
    // Fullscreen triangle.
    var p = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uv = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(p[i], 0.0, 1.0);
    out.uv = uv[i];
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return textureSample(frame_tex, frame_smp, in.uv);
}
"#;

/// Presents a GPU GS frame: RGBA8 words in a storage buffer.
const BUFFER_SHADER: &str = r#"
@group(0) @binding(0) var<storage, read> frame: array<u32>;
// (width, height, 1 if the surface is sRGB)
@group(0) @binding(1) var<uniform> dims: vec4<u32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VsOut {
    var p = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uv = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(p[i], 0.0, 1.0);
    out.uv = uv[i];
    return out;
}

fn to_linear(c: vec3<f32>) -> vec3<f32> {
    return select(pow((c + 0.055) / 1.055, vec3<f32>(2.4)), c / 12.92, c <= vec3<f32>(0.04045));
}

fn frame_rgb(x: u32, y: u32) -> vec3<f32> {
    let c = frame[y * dims.x + x];
    var rgb = vec3<f32>(f32(c & 0xFFu), f32((c >> 8u) & 0xFFu), f32((c >> 16u) & 0xFFu)) / 255.0;
    if (dims.z != 0u) {
        // The texture path samples an sRGB texture (decoding to linear) into
        // an sRGB surface (encoding again); do the same.
        rgb = to_linear(rgb);
    }
    return rgb;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let w = dims.x;
    let h = dims.y;
    // Frame pixels per window pixel: above 1 (an N× frame in a smaller
    // window) average the footprint, otherwise nearest.
    let n = vec2<u32>(clamp(ceil(fwidth(in.uv) * vec2<f32>(f32(w), f32(h)) - 0.01), vec2<f32>(1.0), vec2<f32>(4.0)));
    let x = min(u32(max(in.uv.x, 0.0) * f32(w)), w - 1u);
    let y = min(u32(max(in.uv.y, 0.0) * f32(h)), h - 1u);
    if (n.x == 1u && n.y == 1u) {
        return vec4<f32>(frame_rgb(x, y), 1.0);
    }
    let x0 = min(u32(max(in.uv.x * f32(w) - 0.5 * f32(n.x - 1u), 0.0)), w - 1u);
    let y0 = min(u32(max(in.uv.y * f32(h) - 0.5 * f32(n.y - 1u), 0.0)), h - 1u);
    var sum = vec3<f32>(0.0);
    for (var j = 0u; j < n.y; j = j + 1u) {
        for (var i = 0u; i < n.x; i = i + 1u) {
            sum = sum + frame_rgb(min(x0 + i, w - 1u), min(y0 + j, h - 1u));
        }
    }
    return vec4<f32>(sum / f32(n.x * n.y), 1.0);
}
"#;

/// The pipeline and bindings for presenting a GPU GS frame buffer.
struct BufferPath {
    pipeline: wgpu::RenderPipeline,
    bgl: wgpu::BindGroupLayout,
    dims: wgpu::Buffer,
    /// Bind group for the last frame buffer seen.
    bound: Option<(wgpu::Buffer, wgpu::BindGroup)>,
}

#[derive(Debug)]
pub enum PresentError {
    Reconfigured,
    Lost,
    Skip,
}

/// A `wgpu`-backed presenter that shows the GS VRAM in a window.
pub struct Presenter {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    bgl: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    tex: wgpu::Texture,
    tex_size: (u32, u32),
    buffer_path: BufferPath,
    /// Display aspect ratio the image is fitted to (letterboxed or
    /// pillarboxed); 0 stretches it over the whole window.
    aspect: f32,
    // Keep the window handle alive for as long as the surface exists.
    _window: Arc<Window>,
}

impl Presenter {
    /// Create the device and render pipeline for `window`. With `gs`
    /// (the GPU GS backend's device), present with that device so GS frames
    /// can be shown without leaving the GPU.
    pub async fn new(window: Arc<Window>, gs: Option<&'static ssx3_ps2::gs::gpu::GpuContext>) -> Self {
        let shared = gs.and_then(|ctx| {
            let surface = ctx.instance.create_surface(window.clone()).ok()?;
            if !ctx.adapter.is_surface_supported(&surface) {
                eprintln!("[present] GS device cannot present to this window; using a separate device");
                return None;
            }
            Some((surface, ctx.adapter.clone(), ctx.device.clone(), ctx.queue.clone()))
        });
        let (surface, adapter, device, queue) = match shared {
            Some(s) => s,
            None => {
                let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
                let surface = instance
                    .create_surface(window.clone())
                    .expect("create surface");
                let adapter = instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::HighPerformance,
                        compatible_surface: Some(&surface),
                        ..Default::default()
                    })
                    .await
                    .expect("request adapter");
                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor {
                        label: Some("ssx3"),
                        ..Default::default()
                    })
                    .await
                    .expect("request device");
                (surface, adapter, device, queue)
            }
        };

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let mut config = surface
            .get_default_config(&adapter, width, height)
            .expect("surface not supported by adapter");
        config.present_mode = wgpu::PresentMode::AutoVsync;
        surface.configure(&device, &config);

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("frame"),
            size: wgpu::Extent3d {
                width: 640,
                height: 448,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let vram_view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("frame-sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("frame-bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("frame-bg"),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&vram_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gs-present"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gs-present-layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gs-present-pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        let buffer_path = BufferPath::new(&device, config.format);
        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            bind_group,
            bgl,
            sampler,
            tex,
            tex_size: (640, 448),
            buffer_path,
            aspect: 4.0 / 3.0,
            _window: window,
        }
    }

    /// Whether `frame` (from `Gs::display_gpu`) lives on this presenter's device.
    pub fn same_device(&self, gs: Option<&'static ssx3_ps2::gs::gpu::GpuContext>) -> bool {
        gs.is_some_and(|c| c.device == self.device)
    }

    /// Draw a GPU GS frame (on this presenter's device) to the window.
    pub fn render_gpu(&mut self, frame: &ssx3_ps2::gs::gpu::GpuFrame) -> Result<(), PresentError> {
        let srgb = self.config.format.is_srgb() as u32;
        let dims = [frame.width, frame.height, srgb, 0u32];
        let bytes: Vec<u8> = dims.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.queue.write_buffer(&self.buffer_path.dims, 0, &bytes);
        let bp = &mut self.buffer_path;
        if bp.bound.as_ref().is_none_or(|(b, _)| *b != frame.buffer) {
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("gs-frame-bg"),
                layout: &bp.bgl,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: frame.buffer.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: bp.dims.as_entire_binding() },
                ],
            });
            bp.bound = Some((frame.buffer.clone(), bg));
        }
        let Some((_, bg)) = &self.buffer_path.bound else { unreachable!() };
        let bg = bg.clone();
        let pipeline = self.buffer_path.pipeline.clone();
        self.draw_fullscreen(&pipeline, &bg)
    }

    /// Draw a texture view on this presenter's device (an sRGB view of the
    /// native renderer's image) to the window.
    pub fn render_view(&mut self, view: &wgpu::TextureView) -> Result<(), PresentError> {
        let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gfx-frame-bg"),
            layout: &self.bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let pipeline = self.pipeline.clone();
        self.draw_fullscreen(&pipeline, &bg)
    }

    /// Resize the swapchain to the window's new size.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Fit the image to `aspect` (width / height; 0 = fill the window).
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    /// Upload an RGBA8 frame (`w`×`h`) and draw it to the window.
    pub fn render(&mut self, w: u32, h: u32, rgba: &[u32]) -> Result<(), PresentError> {
        if (w, h) != self.tex_size && w > 0 && h > 0 {
            self.tex = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("frame"),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = self.tex.create_view(&wgpu::TextureViewDescriptor::default());
            self.bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("frame-bg"),
                layout: &self.bgl,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                ],
            });
            self.tex_size = (w, h);
        }
        let bytes: &[u8] = unsafe { std::slice::from_raw_parts(rgba.as_ptr() as *const u8, rgba.len() * 4) };
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &bytes[..(w * h * 4) as usize],
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );

        let (pipeline, bg) = (self.pipeline.clone(), self.bind_group.clone());
        self.draw_fullscreen(&pipeline, &bg)
    }

    /// Acquire the next surface texture and draw a fullscreen triangle.
    fn draw_fullscreen(&mut self, pipeline: &wgpu::RenderPipeline, bind_group: &wgpu::BindGroup) -> Result<(), PresentError> {
        let frame = self.surface.get_current_texture();
        let acquired = match frame {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Err(PresentError::Reconfigured);
            }
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Validation => {
                return Err(PresentError::Lost);
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(PresentError::Skip);
            }
            #[allow(unreachable_patterns)]
            _ => return Err(PresentError::Skip),
        };

        let view = acquired
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            let (w, h) = (self.config.width as f32, self.config.height as f32);
            if self.aspect > 0.0 {
                let (vw, vh) = if w / h > self.aspect { (h * self.aspect, h) } else { (w, w / self.aspect) };
                pass.set_viewport(((w - vw) / 2.0).floor(), ((h - vh) / 2.0).floor(), vw.round(), vh.round(), 0.0, 1.0);
            }
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(acquired);
        Ok(())
    }
}


impl BufferPath {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> BufferPath {
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gs-frame-bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gs-frame-present"),
            source: wgpu::ShaderSource::Wgsl(BUFFER_SHADER.into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gs-frame-layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gs-frame-pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let dims = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gs-frame-dims"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        BufferPath {
            pipeline,
            bgl,
            dims,
            bound: None,
        }
    }
}
