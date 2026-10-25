//! The light queries' depth (`frame_sink::Depth`): the sample at each GS
//! pixel's centre of the depth after phase 1, gathered on the GPU into one
//! f32 per GS pixel, so only that (640×448: 1.1 MB) is read back instead of
//! the whole target (1080p: 8 MB, twice copied). The sample positions are
//! the CPU's (the same f32 arithmetic the queries used on the full read-back):
//! the values are the same.

const SHADER: &str = r#"
@group(0) @binding(0) var depth: texture_depth_2d;
// lw, lh, then the target column of each GS column and the row of each row.
@group(0) @binding(1) var<storage, read> idx: array<u32>;
@group(0) @binding(2) var<storage, read_write> out: array<f32>;

@compute @workgroup_size(8, 8)
fn gather(@builtin(global_invocation_id) g: vec3<u32>) {
    let lw = idx[0];
    let lh = idx[1];
    if (g.x >= lw || g.y >= lh) {
        return;
    }
    out[g.y * lw + g.x] = textureLoad(depth, vec2<u32>(idx[2u + g.x], idx[2u + lw + g.y]), 0);
}
"#;

pub struct DepthQuery {
    pipe: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
    /// (target w, h, GS w, h) the buffers are for.
    key: (u32, u32, u32, u32),
    idx: Option<wgpu::Buffer>,
    out: Option<wgpu::Buffer>,
}

/// One frame's gather: run it with [`Gather::run`] after phase 1, copy
/// [`Gather::out`] (`len` bytes) into the read-back buffer.
pub struct Gather {
    pipe: wgpu::ComputePipeline,
    group: wgpu::BindGroup,
    lw: u32,
    lh: u32,
    pub out: wgpu::Buffer,
    pub len: u64,
}

impl Gather {
    pub fn run(&self, enc: &mut wgpu::CommandEncoder) {
        let mut cp = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("ssx3-gfx depth query"),
            timestamp_writes: None,
        });
        cp.set_pipeline(&self.pipe);
        cp.set_bind_group(0, &self.group, &[]);
        cp.dispatch_workgroups(self.lw.div_ceil(8), self.lh.div_ceil(8), 1);
    }
}

impl DepthQuery {
    pub fn new(dev: &wgpu::Device) -> Self {
        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ssx3-gfx depth query"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let storage = |binding, read_only| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let layout = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ssx3-gfx depth query"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                storage(1, true),
                storage(2, false),
            ],
        });
        let pl = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ssx3-gfx depth query"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let pipe = dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("ssx3-gfx depth query"),
            layout: Some(&pl),
            module: &module,
            entry_point: Some("gather"),
            compilation_options: Default::default(),
            cache: None,
        });
        DepthQuery {
            pipe,
            layout,
            key: (0, 0, 0, 0),
            idx: None,
            out: None,
        }
    }

    /// The gather of `depth` (`pw`×`ph`) at the centres of `lw`×`lh` GS
    /// pixels.
    pub fn prepare(
        &mut self,
        dev: &wgpu::Device,
        queue: &wgpu::Queue,
        depth: &wgpu::Texture,
        (pw, ph): (u32, u32),
        (lw, lh): (u32, u32),
    ) -> Gather {
        if self.key != (pw, ph, lw, lh) || self.idx.is_none() {
            self.key = (pw, ph, lw, lh);
            // (As the queries took them from the full read-back.)
            let at = |v: u32, p: u32, l: u32| {
                (((v as f32 + 0.5) * p as f32 / l as f32) as u32).min(p - 1)
            };
            let idx: Vec<u32> = [lw, lh]
                .into_iter()
                .chain((0..lw).map(|x| at(x, pw, lw)))
                .chain((0..lh).map(|y| at(y, ph, lh)))
                .collect();
            let b = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ssx3-gfx depth query positions"),
                size: idx.len() as u64 * 4,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(
                &b,
                0,
                &idx.iter()
                    .flat_map(|v| v.to_le_bytes())
                    .collect::<Vec<u8>>(),
            );
            self.idx = Some(b);
            self.out = Some(dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ssx3-gfx depth query"),
                size: lw as u64 * lh as u64 * 4,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }));
        }
        let (idx, out) = (self.idx.as_ref().unwrap(), self.out.clone().unwrap());
        let view = depth.create_view(&wgpu::TextureViewDescriptor {
            aspect: wgpu::TextureAspect::DepthOnly,
            ..Default::default()
        });
        let group = dev.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ssx3-gfx depth query"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: idx.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: out.as_entire_binding(),
                },
            ],
        });
        Gather {
            pipe: self.pipe.clone(),
            group,
            lw,
            lh,
            len: lw as u64 * lh as u64 * 4,
            out,
        }
    }
}
