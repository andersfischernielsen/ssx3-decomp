//! The terrain bake's colour, blurred wide (real-time lighting, flag 32768:
//! the artists' zone and shade colours without the shapes of the bake's
//! shadows): each bake texture blurred once into a texture of its own
//! (17 taps on two rings of 8 and 4 texels, through the draw's own sampler)
//! instead of in every pixel that draws it.

use std::collections::HashMap;

const SHADER: &str = r#"
@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;

@vertex
fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn fs(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(src));
    let uv = p.xy / dims;
    let px = 8.0 / dims;
    var acc = textureSampleLevel(src, samp, uv, 0.0);
    for (var i = 0; i < 8; i = i + 1) {
        let a = f32(i) * 0.7853982;
        let d = vec2<f32>(cos(a), sin(a));
        acc = acc + textureSampleLevel(src, samp, uv + d * px, 0.0);
        acc = acc + textureSampleLevel(src, samp, uv + d * px * 0.5, 0.0);
    }
    return acc / 17.0;
}
"#;

pub struct BakeBlur<K> {
    pipe: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    /// Blurred textures by source and sampler, with the frame last used.
    cache: HashMap<K, (wgpu::TextureView, u64)>,
}

impl<K: std::hash::Hash + Eq> BakeBlur<K> {
    pub fn new(dev: &wgpu::Device) -> Self {
        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ssx3-gfx bake blur"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ssx3-gfx bake blur"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pl = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ssx3-gfx bake blur"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let pipe = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ssx3-gfx bake blur"),
            layout: Some(&pl),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        BakeBlur {
            pipe,
            layout,
            cache: HashMap::new(),
        }
    }

    /// `src` blurred through `samp` (made the first time this key asks).
    pub fn blurred(
        &mut self,
        dev: &wgpu::Device,
        queue: &wgpu::Queue,
        key: K,
        src: &wgpu::Texture,
        samp: &wgpu::Sampler,
        frame: u64,
    ) -> wgpu::TextureView {
        if let Some((v, used)) = self.cache.get_mut(&key) {
            *used = frame;
            return v.clone();
        }
        let t = dev.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssx3-gfx bake blurred"),
            size: wgpu::Extent3d {
                width: src.width(),
                height: src.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = t.create_view(&Default::default());
        let sv = src.create_view(&wgpu::TextureViewDescriptor {
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });
        let group = dev.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ssx3-gfx bake blur"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sv),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(samp),
                },
            ],
        });
        let mut enc = dev.create_command_encoder(&Default::default());
        {
            let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssx3-gfx bake blur"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            rp.set_pipeline(&self.pipe);
            rp.set_bind_group(0, &group, &[]);
            rp.draw(0..3, 0..1);
        }
        // (Before the frame that draws with it: submitted in order.)
        queue.submit(std::iter::once(enc.finish()));
        self.cache.insert(key, (view.clone(), frame));
        view
    }

    /// Drop those unused for `age` frames.
    pub fn evict(&mut self, frame: u64, age: u64) {
        self.cache.retain(|_, (_, used)| frame - *used < age);
    }
}
