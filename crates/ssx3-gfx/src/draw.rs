//! wgpu pipelines for GS draw states.
//!
//! GS blending is `(A − B) · C + D` (`docs/investigation/phase3-render.md`
//! §2.1), with A, B, D ∈ {Cs, Cd, 0} and C ∈ {As, Ad, FIX}. The coefficients
//! of Cs and Cd are each `α·C + β`. When C is known in the shader (As, FIX)
//! the source term is premultiplied there and the hardware adds the
//! destination term; with C = Ad both sides use the DstAlpha factor. Two
//! forms have no direct factor: `Cd·(1 + C)` (drawn as `C · Cd + Cd`, the
//! shader outputting C) and `Cs·(1 + C)` (premultiplied; the clamp matches
//! the GS's).

use std::collections::HashMap;

use crate::gs::DrawState;

pub const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// Uniforms of one batch (`draw.wgsl` `Batch`), padded to 256 bytes.
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct BatchU {
    pub size: [f32; 2],
    pub tfx: u32,
    pub flags: u32,
    pub atst: u32,
    pub aref: u32,
    pub side: u32,
    pub src_mode: u32,
    pub cs_a: f32,
    pub cs_b: f32,
    pub c_kind: u32,
    pub fix: f32,
    pub fogcol: [f32; 4],
    pub ta: [f32; 2],
    pub pad: [f32; 2],
}

/// Size of [`BatchU`] in bytes.
pub const BATCH_BYTES: u64 = 80;

pub const UNIFORM_STRIDE: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PipeKey {
    pub blend: Option<(wgpu::BlendFactor, wgpu::BlendFactor, wgpu::BlendOperation)>,
    pub depth: wgpu::CompareFunction,
    pub depth_write: bool,
    pub write: u8, // ColorWrites bits
}

/// What one draw of a batch needs: pipeline, uniforms, blend constant.
pub struct Plan {
    pub key: PipeKey,
    pub u: BatchU,
    pub constant: f64,
}

fn coef(a: u64, b: u64, d: u64, which: u64) -> (f32, f32) {
    let al = (a == which) as i32 - (b == which) as i32;
    (al as f32, (d == which) as u32 as f32)
}

/// Blend plan for ALPHA (with ABE) → (blend, src_mode, cs_a, cs_b, c_kind).
fn blend(alpha: u64) -> (Option<(wgpu::BlendFactor, wgpu::BlendFactor, wgpu::BlendOperation)>, u32, f32, f32, u32) {
    use wgpu::BlendFactor as F;
    use wgpu::BlendOperation as O;
    let (a, b, c, d) = (alpha & 3, (alpha >> 2) & 3, (alpha >> 4) & 3, (alpha >> 6) & 3);
    let (sa, sb) = coef(a, b, d, 0);
    let (da, db) = coef(a, b, d, 1);
    if c == 1 {
        // C = Ad: factors from the stored destination alpha.
        let f = |al: f32, be: f32| match (al as i32, be as i32) {
            (0, 0) => (F::Zero, false),
            (0, 1) => (F::One, false),
            (1, 0) => (F::DstAlpha, false),
            (-1, 1) => (F::OneMinusDstAlpha, false),
            (-1, 0) => (F::DstAlpha, true),
            _ => (F::One, false),
        };
        let ((fs, ns), (fd, nd)) = (f(sa, sb), f(da, db));
        let op = match (ns, nd) {
            (true, false) => O::ReverseSubtract,
            (false, true) => O::Subtract,
            _ => O::Add,
        };
        return (Some((fs, fd, op)), 0, 0.0, 0.0, 1);
    }
    let c_kind = if c == 2 { 2 } else { 0 };
    let cf = if c == 2 { F::Constant } else { F::SrcAlpha };
    let cf1 = if c == 2 { F::OneMinusConstant } else { F::OneMinusSrcAlpha };
    let s_neg = sa < 0.0 && sb == 0.0;
    let (fd, d_neg) = match (da as i32, db as i32) {
        (0, 0) => (F::Zero, false),
        (0, 1) => (F::One, false),
        (1, 0) => (cf, false),
        (-1, 1) => (cf1, false),
        (-1, 0) => (cf, true),
        _ => {
            // Cd·(1 + C): only without a source term.
            if sa == 0.0 && sb == 0.0 {
                return (Some((F::Dst, F::One, O::Add)), 2, 0.0, 0.0, c_kind);
            }
            (F::One, false)
        }
    };
    let op = if s_neg {
        O::ReverseSubtract
    } else if d_neg {
        O::Subtract
    } else {
        O::Add
    };
    (Some((F::One, fd, op)), 1, sa, sb, c_kind)
}

fn compare(test: u64) -> wgpu::CompareFunction {
    use wgpu::CompareFunction as C;
    if (test >> 16) & 1 == 0 {
        return C::Always;
    }
    match (test >> 17) & 3 {
        0 => C::Never,
        1 => C::Always,
        2 => C::GreaterEqual,
        _ => C::Greater,
    }
}

/// The draws for one batch: one, or two for an alpha test whose failing
/// pixels still write something (AFAIL ≠ KEEP); each split in two when FBA
/// is set and the blend reads the source alpha (the GS blends with As but
/// stores As | 0x80; the output alpha is both here).
/// `rt_psm`: the PSM of the render target the batch samples, if any.
///
/// `as_max`: the largest As the batch can produce. The shader's output is
/// clamped to 1.0 before blending, so a blend without a D term
/// ((A - B)·As) whose As can exceed 1.0 is drawn at half strength and then
/// doubled (Cd + Cd), the stored alpha separately.
pub fn plans(st: &DrawState, size: (u32, u32), rt_psm: Option<u32>, as_max: f32) -> Vec<Plan> {
    use wgpu::BlendFactor as F;
    let reads_as = |k: &PipeKey| k.blend.is_some_and(|(s, d, _)| [s, d].iter().any(|f| matches!(f, F::SrcAlpha | F::OneMinusSrcAlpha)));
    let alpha = st.ctx.alpha;
    let half = st.prim & (1 << 6) != 0 && (alpha >> 4) & 3 == 0 && (alpha >> 6) & 3 == 2 && as_max > 128.0;
    let mut out = Vec::new();
    for p in plans_unsplit(st, size, rt_psm) {
        if half && p.key.blend.is_some() && p.key.write & 7 != 0 {
            let last_z = p.key.depth_write;
            let alpha_too = p.key.write & 8 != 0;
            out.push(Plan {
                key: PipeKey { depth_write: false, write: p.key.write & 7, ..p.key },
                u: BatchU { flags: (p.u.flags | 512) & !8, ..p.u },
                constant: p.constant,
            });
            out.push(Plan {
                key: PipeKey { blend: Some((F::Dst, F::One, wgpu::BlendOperation::Add)), depth_write: last_z && !alpha_too, write: p.key.write & 7, ..p.key },
                u: BatchU { flags: p.u.flags & !8, src_mode: 2, c_kind: 2, fix: 1.0, ..p.u },
                constant: p.constant,
            });
            if alpha_too {
                out.push(Plan { key: PipeKey { blend: None, write: 8, ..p.key }, u: p.u, constant: p.constant });
            }
            continue;
        }
        if p.u.flags & 8 == 0 || p.key.write & 8 == 0 || p.key.write & 7 == 0 || !reads_as(&p.key) {
            out.push(p);
            continue;
        }
        // Colour with the true As (no depth write, so the alpha draw still
        // passes the same depth test), then the stored alpha.
        out.push(Plan { key: PipeKey { depth_write: false, write: p.key.write & 7, ..p.key }, u: BatchU { flags: p.u.flags & !8, ..p.u }, constant: p.constant });
        out.push(Plan { key: PipeKey { blend: None, write: 8, ..p.key }, u: p.u, constant: p.constant });
    }
    out
}

fn plans_unsplit(st: &DrawState, size: (u32, u32), rt_psm: Option<u32>) -> Vec<Plan> {
    let c = &st.ctx;
    let abe = st.prim & (1 << 6) != 0;
    let (bl, src_mode, cs_a, cs_b, c_kind) = if abe { blend(c.alpha) } else { (None, 0, 0.0, 0.0, 0) };
    let fix = ((c.alpha >> 32) & 0xff) as f64 / 128.0;
    let fbmsk = (c.frame >> 32) as u32;
    let mut write = 0xfu8;
    if fbmsk & 0xff00_0000 == 0xff00_0000 {
        write &= !8;
    }
    if fbmsk & 0x00ff_ffff == 0x00ff_ffff {
        write &= 8;
    }
    let zmsk = (c.zbuf >> 32) & 1 != 0;
    let depth = compare(c.test);
    let tex0 = c.tex0;
    let fogcol = [(st.fogcol & 0xff) as f32, ((st.fogcol >> 8) & 0xff) as f32, ((st.fogcol >> 16) & 0xff) as f32, 0.0];
    let frame_src = (st.tex == Some(crate::gs::FRAME_TEX) || rt_psm.is_some()) as u32;
    let ct16 = rt_psm.is_some_and(|p| matches!(p, 2 | 10)) as u32;
    let flags = (st.prim >> 4 & 1) | (((tex0 >> 34) & 1) as u32) << 1 | (st.prim >> 5 & 1) << 2 | ((c.fba & 1) as u32) << 3 | frame_src << 4 | ct16 << 5;
    let u = BatchU {
        size: [size.0 as f32, size.1 as f32],
        tfx: ((tex0 >> 35) & 3) as u32,
        flags,
        atst: ((c.test >> 1) & 7) as u32,
        aref: ((c.test >> 4) & 0xff) as u32,
        side: 0,
        src_mode,
        cs_a,
        cs_b,
        c_kind,
        fix: fix as f32,
        fogcol,
        ta: [(st.texa & 0xff) as f32, ((st.texa >> 32) & 0xff) as f32],
        pad: [0.0; 2],
    };
    let base = PipeKey { blend: bl, depth, depth_write: !zmsk && depth != wgpu::CompareFunction::Never, write };
    let ate = c.test & 1 != 0;
    let atst = (c.test >> 1) & 7;
    let afail = (c.test >> 12) & 3;
    if !ate || atst == 1 {
        return vec![Plan { key: base, u, constant: fix }];
    }
    let mut out = Vec::new();
    out.push(Plan { key: base, u: BatchU { side: 1, ..u }, constant: fix });
    let fail = match afail {
        0 => None,
        1 => Some(PipeKey { depth_write: false, ..base }),
        2 => Some(PipeKey { write: 0, ..base }),
        _ => Some(PipeKey { depth_write: false, write: write & 7, ..base }),
    };
    if let Some(k) = fail {
        out.push(Plan { key: k, u: BatchU { side: 2, ..u }, constant: fix });
    }
    out
}

/// The GPU-vertex shader module: the GS stages (draw.wgsl), the shared
/// vertex helpers (gpuvu.wgsl) and one file per program (`vs_<program>`).
const GPUVU_SOURCES: &[&str] = &[
    include_str!("draw.wgsl"),
    include_str!("gpuvu.wgsl"),
    include_str!("gpuvu/mat3.wgsl"),
    include_str!("gpuvu/mat3_strip.wgsl"),
    include_str!("gpuvu/mat6.wgsl"),
    include_str!("gpuvu/particles.wgsl"),
    include_str!("gpuvu/bezier.wgsl"),
];

pub struct Pipelines {
    module: wgpu::ShaderModule,
    pub layout: wgpu::BindGroupLayout,
    pl: wgpu::PipelineLayout,
    cache: HashMap<PipeKey, wgpu::RenderPipeline>,
    /// GPU-computed vertices (gpuvu.wgsl after draw.wgsl): bind group 1
    /// holds the calls' uniforms and inputs.
    gpu_module: wgpu::ShaderModule,
    pub gpu_layout: wgpu::BindGroupLayout,
    gpu_pl: wgpu::PipelineLayout,
    gpu_cache: HashMap<(PipeKey, ssx3_render::gpu::Program), wgpu::RenderPipeline>,
}

impl Pipelines {
    pub fn new(dev: &wgpu::Device) -> Pipelines {
        let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ssx3-gfx draw.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_str!("draw.wgsl").into()),
        });
        let layout = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ssx3-gfx"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(BATCH_BYTES),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // The depth buffer as drawn before this batch (depth fog).
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // The frame as drawn before this batch (destination alpha
                // test).
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });
        let pl = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ssx3-gfx"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let gpu_module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ssx3-gfx gpuvu.wgsl"),
            source: wgpu::ShaderSource::Wgsl(GPUVU_SOURCES.join("\n").into()),
        });
        let storage = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let gpu_layout = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ssx3-gfx gpuvu"), entries: &[storage(0), storage(1)] });
        let gpu_pl = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ssx3-gfx gpuvu"),
            bind_group_layouts: &[Some(&layout), Some(&gpu_layout)],
            immediate_size: 0,
        });
        Pipelines { module, layout, pl, cache: HashMap::new(), gpu_module, gpu_layout, gpu_pl, gpu_cache: HashMap::new() }
    }

    /// A GPU-vertex pipeline made earlier with [`Pipelines::get_gpu`].
    pub fn cached_gpu(&self, k: PipeKey, p: ssx3_render::gpu::Program) -> &wgpu::RenderPipeline {
        &self.gpu_cache[&(k, p)]
    }

    /// The pipeline for `k` with the vertices of program `p` computed on
    /// the GPU (`vs_<program>`, no vertex buffer).
    pub fn get_gpu(&mut self, dev: &wgpu::Device, k: PipeKey, p: ssx3_render::gpu::Program) -> &wgpu::RenderPipeline {
        let (module, pl) = (&self.gpu_module, &self.gpu_pl);
        self.gpu_cache.entry((k, p)).or_insert_with(|| {
            let entry = p.vs_entry();
            let blend = k.blend.map(|(s, d, op)| wgpu::BlendState {
                color: wgpu::BlendComponent { src_factor: s, dst_factor: d, operation: op },
                alpha: wgpu::BlendComponent::REPLACE,
            });
            dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ssx3-gfx gpuvu draw"),
                layout: Some(pl),
                vertex: wgpu::VertexState { module, entry_point: Some(entry), compilation_options: Default::default(), buffers: &[] },
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: Some(k.depth_write),
                    depth_compare: Some(k.depth),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some("fs_gpu"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: COLOR_FORMAT,
                        blend,
                        write_mask: wgpu::ColorWrites::from_bits_truncate(k.write as u32),
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        })
    }

    /// A pipeline made earlier with [`Pipelines::get`].
    pub fn cached(&self, k: PipeKey) -> &wgpu::RenderPipeline {
        &self.cache[&k]
    }

    pub fn get(&mut self, dev: &wgpu::Device, k: PipeKey) -> &wgpu::RenderPipeline {
        let (module, pl) = (&self.module, &self.pl);
        self.cache.entry(k).or_insert_with(|| {
            let attrs = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4, 3 => Float32x4];
            let blend = k.blend.map(|(s, d, op)| wgpu::BlendState {
                color: wgpu::BlendComponent { src_factor: s, dst_factor: d, operation: op },
                alpha: wgpu::BlendComponent::REPLACE,
            });
            dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ssx3-gfx draw"),
                layout: Some(pl),
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[Some(wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<crate::gs::Vtx>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &attrs,
                    })],
                },
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: Some(k.depth_write),
                    depth_compare: Some(k.depth),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: COLOR_FORMAT,
                        blend,
                        write_mask: wgpu::ColorWrites::from_bits_truncate(k.write as u32),
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        })
    }
}

/// Sampler description from CLAMP and TEX1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SampKey {
    pub linear: bool,
    pub wrap_s: bool,
    pub wrap_t: bool,
}

pub fn samp_key(st: &DrawState) -> SampKey {
    let c = &st.ctx;
    SampKey { linear: (c.tex1 >> 5) & 1 != 0, wrap_s: c.clamp & 3 == 0, wrap_t: (c.clamp >> 2) & 3 == 0 }
}

/// `aniso`: the anisotropic filtering level (1 = off, up to 16) for linear
/// samplers.
pub fn sampler(dev: &wgpu::Device, k: SampKey, aniso: u16) -> wgpu::Sampler {
    let mode = |w: bool| if w { wgpu::AddressMode::Repeat } else { wgpu::AddressMode::ClampToEdge };
    let f = if k.linear { wgpu::FilterMode::Linear } else { wgpu::FilterMode::Nearest };
    dev.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("ssx3-gfx"),
        address_mode_u: mode(k.wrap_s),
        address_mode_v: mode(k.wrap_t),
        mag_filter: f,
        min_filter: f,
        // (Only replacement textures have mip levels; anisotropic filtering
        // keeps their detail on surfaces seen at a glancing angle.)
        mipmap_filter: if k.linear { wgpu::MipmapFilterMode::Linear } else { wgpu::MipmapFilterMode::Nearest },
        anisotropy_clamp: if k.linear { aniso.clamp(1, 16) } else { 1 },
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gs::{Ctx, DrawState};

    /// A textured, blended triangle with ALPHA = (Cs - Cd)·As + Cd.
    fn blended(fba: u64, fbmsk: u32) -> DrawState {
        DrawState {
            prim: (1 << 4) | (1 << 6),
            ctx: Ctx { alpha: 0x44, fba, frame: (fbmsk as u64) << 32, test: 0x5_0000, zbuf: 0, ..Default::default() },
            texa: 0,
            fogcol: 0,
            tex: None,
            target: 0,
        }
    }

    #[test]
    fn blend_reads_source_alpha() {
        let p = plans(&blended(0, 0), (512, 448), None, 128.0);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].key.blend, Some((wgpu::BlendFactor::One, wgpu::BlendFactor::OneMinusSrcAlpha, wgpu::BlendOperation::Add)));
        assert_eq!(p[0].u.src_mode, 1);
    }

    #[test]
    fn fba_does_not_change_the_blend_factor() {
        // The GS blends with As and stores As | 0x80: colour with the true
        // As first (FBA off, no depth write), then the stored alpha alone.
        let p = plans(&blended(1, 0), (512, 448), None, 128.0);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].key.write, 7);
        assert_eq!(p[0].u.flags & 8, 0);
        assert!(!p[0].key.depth_write);
        assert!(p[0].key.blend.is_some());
        assert_eq!(p[1].key.write, 8);
        assert_eq!(p[1].u.flags & 8, 8);
        assert!(p[1].key.blend.is_none());
        assert!(p[1].key.depth_write);
    }

    #[test]
    fn source_alpha_over_one_without_d_is_halved_then_doubled() {
        // (Cd - Cs)·As (lightmaps) with As up to 212/128: half strength,
        // then Cd + Cd; the depth write on the last draw.
        let mut st = blended(0, 0xff00_0000);
        st.ctx.alpha = 0x81;
        let p = plans(&st, (512, 448), None, 212.0);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].u.flags & 512, 512);
        assert!(!p[0].key.depth_write);
        assert_eq!(p[1].key.blend, Some((wgpu::BlendFactor::Dst, wgpu::BlendFactor::One, wgpu::BlendOperation::Add)));
        assert_eq!((p[1].u.src_mode, p[1].u.c_kind, p[1].u.fix), (2, 2, 1.0));
        assert!(p[1].key.depth_write);
        // As never above 1.0: one draw.
        assert_eq!(plans(&st, (512, 448), None, 128.0).len(), 1);
        // With a D term (Cs·As + Cd) halving would be wrong: one draw.
        st.ctx.alpha = 0x48;
        assert_eq!(plans(&st, (512, 448), None, 212.0).len(), 1);
    }

    #[test]
    fn fba_alone_needs_no_split() {
        // Alpha masked off (FBMSK 0xff000000): one colour draw.
        let p = plans(&blended(1, 0xff00_0000), (512, 448), None, 128.0);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].key.write, 7);
    }
}
