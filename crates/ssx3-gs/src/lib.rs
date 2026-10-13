//! # ssx3-gs
//!
//! Model of the PlayStation 2 **Graphics Synthesizer** plus a `wgpu` presenter.
//!
//! The GS owns 4 MiB of VRAM. That is exactly `1024 * 1024 * 4` bytes, so the
//! presenter reinterprets VRAM as a `1024x1024` `RGBA8` texture and blits it to
//! the window — a convenient debug view while the real GIF packet interpreter
//! and colour-format handling are built out.

use std::sync::Arc;

use winit::window::Window;

use ssx3_core::gs as gsmodel;

pub use gsmodel::{DispFb, VRAM_SIZE};

/// VRAM is presented as a square `RGBA8` texture of this many pixels per side.
pub const VRAM_DIM: u32 = 1024;

const SHADER: &str = r#"
@group(0) @binding(0) var vram_tex: texture_2d<f32>;
@group(0) @binding(1) var vram_smp: sampler;

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
    return textureSample(vram_tex, vram_smp, in.uv);
}
"#;

/// The Graphics Synthesizer state that matters to the runtime.
pub struct Gs {
    /// 4 MiB of embedded DRAM.
    pub vram: Vec<u8>,
    /// Display/environment registers we currently track.
    pub dispfb: [DispFb; 2],
    pub bgcolor: u32,
}

impl Default for Gs {
    fn default() -> Self {
        Self::new()
    }
}

impl Gs {
    pub fn new() -> Self {
        Self {
            vram: vec![0; VRAM_SIZE],
            dispfb: [DispFb::default(); 2],
            bgcolor: 0,
        }
    }

    /// View VRAM as `RGBA8` texels (1024x1024) without copying.
    pub fn vram_as_rgba(&self) -> &[u8] {
        &self.vram
    }

    /// Handle a write to a GS privileged register (`0x12000000..`).
    pub fn write_privileged(&mut self, addr: u32, val: u32) {
        match addr {
            gsmodel::reg::BGCOLOR => self.bgcolor = val,
            gsmodel::reg::DISPFB1 => self.dispfb[0] = decode_dispfb(val),
            gsmodel::reg::DISPFB2 => self.dispfb[1] = decode_dispfb(val),
            _ => {}
        }
    }
}

/// Decode the low 32 bits of a `DISPFB` register. The `fbx`/`fby` fields live in
/// the upper 32 bits, which we do not model yet.
fn decode_dispfb(v: u32) -> DispFb {
    DispFb {
        fbp: v & 0x1ff,
        fbw: (v >> 9) & 0x3f,
        psm: (v >> 15) & 0x1f,
        fbx: 0,
        fby: 0,
    }
}

/// Why a frame could not be presented.
#[derive(Debug)]
pub enum PresentError {
    /// The swapchain needs reconfiguring; we have already done so.
    Reconfigured,
    /// The surface was lost and must be recreated by the caller.
    Lost,
    /// Transient condition (timeout/occluded); skip the frame.
    Skip,
}

/// A `wgpu`-backed presenter that shows the GS VRAM in a window.
pub struct GsRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    vram_texture: wgpu::Texture,
    // Keep the window handle alive for as long as the surface exists.
    _window: Arc<Window>,
}

impl GsRenderer {
    /// Create the device and render pipeline for `window`.
    pub async fn new(window: Arc<Window>) -> Self {
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

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let mut config = surface
            .get_default_config(&adapter, width, height)
            .expect("surface not supported by adapter");
        config.present_mode = wgpu::PresentMode::AutoVsync;
        surface.configure(&device, &config);

        let vram_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("gs-vram"),
            size: wgpu::Extent3d {
                width: VRAM_DIM,
                height: VRAM_DIM,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let vram_view = vram_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("gs-vram-sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gs-vram-bgl"),
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
            label: Some("gs-vram-bg"),
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

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            bind_group,
            vram_texture,
            _window: window,
        }
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

    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    /// Upload `gs` VRAM and draw it to the window.
    pub fn render(&mut self, gs: &Gs) -> Result<(), PresentError> {
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.vram_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &gs.vram,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(VRAM_DIM * 4),
                rows_per_image: Some(VRAM_DIM),
            },
            wgpu::Extent3d {
                width: VRAM_DIM,
                height: VRAM_DIM,
                depth_or_array_layers: 1,
            },
        );

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
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });
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
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(acquired);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vram_is_exactly_a_1024x1024_rgba_texture() {
        let gs = Gs::new();
        assert_eq!(gs.vram.len(), VRAM_SIZE);
        assert_eq!(gs.vram_as_rgba().len(), (VRAM_DIM * VRAM_DIM * 4) as usize);
    }

    #[test]
    fn dispfb_decode_extracts_fields() {
        // fbp=0x10, fbw=1, psm=1 (CT16)
        let v = 0x10 | (1 << 9) | (1 << 15);
        let d = decode_dispfb(v);
        assert_eq!(d.fbp, 0x10);
        assert_eq!(d.fbw, 1);
        assert_eq!(d.psm, 1);
        assert_eq!(d.vram_offset(), 0x10 * 8192);
    }

    #[test]
    fn privileged_writes_update_gs_state() {
        let mut gs = Gs::new();
        gs.write_privileged(gsmodel::reg::BGCOLOR, 0x0011_2233);
        assert_eq!(gs.bgcolor, 0x0011_2233);

        gs.write_privileged(gsmodel::reg::DISPFB1, 0x8000);
        assert_eq!(gs.dispfb[0].psm, 1);
    }

    #[test]
    fn writes_to_vram_are_visible_to_the_presenter() {
        let mut gs = Gs::new();
        gs.vram[4] = 0xab;
        assert_eq!(gs.vram_as_rgba()[4], 0xab);
    }
}
