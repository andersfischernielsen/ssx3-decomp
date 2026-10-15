//! GPU rasterization backend (`SSX3_GS=gpu`): the GS pixel pipeline as a
//! wgpu compute shader over VRAM held in a GPU storage buffer.
//!
//! The design is a port of ps2-web-recomp's WebGPU GS
//! (`runtime/src/gs/gpu/gpu.cpp` and `gs.wgsl`, MIT, Copyright (c) 2026
//! Samir Nuri, <https://github.com/snuri00/ps2-web-recomp>): VRAM in the
//! native swizzled layout as a storage buffer, primitives binned on the CPU
//! into pixel tiles (8x8 here, 16x16 in the reference; 8x8 measured
//! ~15% faster on an M1), one workgroup per tile walking its primitives in
//! submission order, and a snapshot of VRAM for primitives that sample what
//! their batch draws. Everything else (GIF, registers, transfers, CLUT
//! loads, primitive setup with `raster::setup`) stays on the CPU and is
//! shared with the software rasterizer, which remains the reference.
//!
//! # VRAM ownership
//!
//! `Gs::vram` becomes a *mirror*. Two page sets say which copy is current:
//! - `gpu_dirty`: pages drawn on the GPU since they were last read back.
//!   CPU reads of them (CLUT loads, local->host and local->local transfers,
//!   partial uploads) first read those pages back (`readback`).
//! - `cpu_dirty`: pages the CPU wrote (host->local transfers) that the GPU
//!   has not seen. They are uploaded, whole pages at a time, in front of the
//!   next raster batch.
//! The common path (draw, draw, ..., display) never reads VRAM back: the
//! display circuit is evaluated on the GPU (`display_gpu`).
//!
//! # Batches
//!
//! A batch is one raster dispatch. Within it, all primitives touching a
//! pixel are applied by the same thread in submission order, so it is exact
//! as long as different pixels never share VRAM through different
//! mappings and texture reads see the right data. A batch is closed before
//! a primitive that
//! - samples blocks the batch has written (read after write; block-exact
//!   texel rectangles from `raster::prim_tex_rect`), or
//! - draws to pages the batch draws through a different buffer mapping.
//! Primitives whose pixels alias other pixels (beyond the buffer width,
//! frame/Z overlap) get a batch of their own. Texture reads of pages the
//! batch writes (a primitive sampling its own target, or writing what an
//! earlier one sampled) are served from a copy of those pages taken before
//! the batch.
//!
//! # Record layouts (u32 words in the `data` buffer)
//!
//! See the `S_*` (draw setup) and `P_*` (primitive) constants in `gs.wgsl`.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, OnceLock};

use super::batch::WriteSet;
use super::raster::{self, Blend, Draw, FbFmt, Pages, Prim, Shape, TexKind};
use super::region::Region;
use super::swizzle::{self, Kind, Surface};
use super::Vertex;

const SHADER: &str = include_str!("gs.wgsl");

const VRAM_BYTES: u64 = (swizzle::VRAM_WORDS * 4) as u64;
const PAGE_WORDS: usize = 2048;
const SETUP_WORDS: usize = 63;
/// Decoded-texture words of a setup record: [offset, u0 | v0 << 16, width] per level.
const S_DEC: usize = 42;
const PRIM_WORDS: usize = 41;
/// Uniform slot stride (min_uniform_buffer_offset_alignment upper bound).
const SLOT: u64 = 256;
/// Tile size (log2 pixels); must match `TILE` in gs.wgsl.
const TILE_SHIFT: i32 = 3;
const TILES_X: usize = 2048 >> TILE_SHIFT;
/// Keep at most this many submissions in flight.
const MAX_INFLIGHT: usize = 3;
/// Close a batch after this many primitives (bounds the data buffer).
const MAX_BATCH_PRIMS: usize = 1 << 16;

// Setup flags (gs.wgsl F_*).
const F_TME: u32 = 1;
const F_FGE: u32 = 2;
const F_ABE: u32 = 4;
const F_FST: u32 = 8;
const F_ZTE: u32 = 16;
const F_ZUSED: u32 = 32;
const F_ZMSK: u32 = 64;
const F_ATE: u32 = 128;
const F_DATE: u32 = 256;
const F_DATM: u32 = 512;
const F_PABE: u32 = 1024;
const F_COLCLAMP: u32 = 2048;
const F_FBA: u32 = 4096;
const F_TCC: u32 = 8192;
const F_AEM: u32 = 16384;
const F_MIP: u32 = 32768;
const F_DITHER: u32 = 65536;
const F_MAGLIN: u32 = 131072;
const F_LCM: u32 = 524288;

const PK_LINEAR: u32 = 256;
const PK_BIG: u32 = 512;

/// The process-wide wgpu device used by the GPU GS. The windowed front end
/// presents with the same device, so GS frames never leave the GPU.
pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// The shared device, created on first use; `None` if no adapter is usable.
pub fn context() -> Option<&'static GpuContext> {
    static C: OnceLock<Option<GpuContext>> = OnceLock::new();
    C.get_or_init(|| match pollster::block_on(create_context()) {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("[gs] GPU backend unavailable: {e}");
            None
        }
    })
    .as_ref()
}

/// `SSX3_GS_TRACE_READBACK`: log every VRAM readback and its cause.
fn trace_readback() -> bool {
    static P: OnceLock<bool> = OnceLock::new();
    *P.get_or_init(|| std::env::var_os("SSX3_GS_TRACE_READBACK").is_some())
}

/// `SSX3_GS_PROFILE=1`: time the raster passes with GPU timestamps
/// (`GpuStats::gpu_raster_ns`; waits for the GPU after every submission).
fn profiling() -> bool {
    static P: OnceLock<bool> = OnceLock::new();
    *P.get_or_init(|| std::env::var_os("SSX3_GS_PROFILE").is_some())
}

/// Timestamp queries for `SSX3_GS_PROFILE`.
struct Prof {
    qs: wgpu::QuerySet,
    resolve: wgpu::Buffer,
    read: wgpu::Buffer,
}

const PROF_PASSES: u32 = 2048;

async fn create_context() -> Result<GpuContext, String> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            ..Default::default()
        })
        .await
        .map_err(|e| format!("request_adapter: {e}"))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("ssx3-gs"),
            required_limits: adapter.limits(),
            required_features: if profiling() {
                adapter.features() & wgpu::Features::TIMESTAMP_QUERY
            } else {
                wgpu::Features::empty()
            },
            ..Default::default()
        })
        .await
        .map_err(|e| format!("request_device: {e}"))?;
    device.set_device_lost_callback(|reason, msg| {
        eprintln!("[gs] GPU device lost ({reason:?}): {msg}");
    });
    Ok(GpuContext {
        instance,
        adapter,
        device,
        queue,
    })
}

/// A displayed frame on the GPU: `width * height` RGBA8 words (`0xAABBGGRR`)
/// at the start of `buffer` (a storage buffer of the shared device).
#[derive(Clone)]
pub struct GpuFrame {
    pub buffer: wgpu::Buffer,
    pub width: u32,
    pub height: u32,
}

/// GPU backend statistics.
#[derive(Default, Clone, Copy, Debug)]
pub struct GpuStats {
    pub submits: u64,
    /// Raster dispatches, primitives, tile records and tile list entries.
    pub batches: u64,
    pub prims: u64,
    pub tiles: u64,
    pub entries: u64,
    /// Primitives that needed a batch of their own / big-triangle span tables.
    pub isolated: u64,
    pub big_tris: u64,
    /// Why batches were closed early.
    pub raw_closes: u64,
    pub alias_closes: u64,
    pub transfer_closes: u64,
    /// Batches that sampled a snapshot, and its size in pages.
    pub snapshots: u64,
    pub snapshot_pages: u64,
    /// Pages uploaded from the CPU mirror.
    pub upload_pages: u64,
    /// VRAM readbacks and pages read back.
    pub readbacks: u64,
    pub readback_pages: u64,
    pub setups: u64,
    pub palettes: u64,
    pub data_bytes: u64,
    /// Primitives and tile list entries per kind (triangle, sprite, line, point).
    pub kind_prims: [u64; 4],
    pub kind_entries: [u64; 4],
    /// Host->local pixels written on the GPU (partial-word formats).
    pub direct_upload_pixels: u64,
    /// Local->host transfers served from the previous frame's copy.
    pub latent_reads: u64,
    /// Texture levels decoded, and their texels.
    pub decodes: u64,
    pub decoded_texels: u64,
    /// GPU time of the raster passes (only with `SSX3_GS_PROFILE`).
    pub gpu_raster_ns: u64,
}

enum Cmd {
    /// Copy `n` pages of the data buffer (from word `src`) to VRAM page `page`.
    Upload { page: u32, n: u32, src: u32 },
    /// Copy VRAM words `lo..hi` into the snapshot buffer.
    Snapshot { lo: u32, hi: u32 },
    Raster { slot: u32, tiles: u32 },
    /// Write `n` host->local pixels (see `pixel_write`).
    Pixels { slot: u32, n: u32 },
    /// Decode `n` texels of a texture level into the decoded buffer.
    Decode { slot: u32, n: u32 },
    Display { slot: u32, w: u32, h: u32 },
}

#[derive(Clone, Copy)]
struct Span {
    prim: u32,
    ty: u16,
    tx0: u16,
    tx1: u16,
}

/// The open batch.
#[derive(Default)]
struct GBatch {
    spans: Vec<Span>,
    prims: usize,
    write: WriteSet,
    /// Texture pages sampled.
    read: Pages,
    /// Buffer mappings drawn to, with their pages.
    surfs: Vec<((u32, u32, u32), Pages)>,
    /// Textured draw states (setup offset), with their estimated texel fetches.
    tex_use: Vec<(u32, Arc<Draw>, u64)>,
}

impl GBatch {
    fn clear(&mut self) {
        self.spans.clear();
        self.prims = 0;
        self.write.clear();
        self.read = Pages::default();
        self.surfs.clear();
        self.tex_use.clear();
    }
}

pub(crate) struct GpuGs {
    ctx: &'static GpuContext,
    vram: wgpu::Buffer,
    snap: wgpu::Buffer,
    tables: wgpu::Buffer,
    data: wgpu::Buffer,
    params: wgpu::Buffer,
    out: wgpu::Buffer,
    /// Decoded textures (RGBA8 texels), filled per submission.
    dec: wgpu::Buffer,
    staging: wgpu::Buffer,
    bgl: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    raster_pipe: wgpu::ComputePipeline,
    display_pipe: wgpu::ComputePipeline,
    decode_pipe: wgpu::ComputePipeline,
    pixels_pipe: wgpu::ComputePipeline,

    arena: Vec<u32>,
    slots: Vec<[u32; 16]>,
    cmds: Vec<Cmd>,
    batch: GBatch,
    /// Scratch texel region for the read-after-write test in `push`.
    scratch: Region,
    /// Setup record of the last draw state, by identity.
    last_draw: Option<(Arc<Draw>, u32)>,
    palettes: HashMap<u64, u32>,
    /// Textures decoded in this submission: texel offset and VRAM pages.
    decoded: HashMap<DecKey, (u32, Pages)>,
    dec_used: u32,
    pub(crate) cpu_dirty: Pages,
    pub(crate) gpu_dirty: Pages,
    inflight: VecDeque<wgpu::SubmissionIndex>,
    counts: Vec<u32>,
    fill: Vec<u32>,
    touched: Vec<u32>,
    pub(crate) stats: GpuStats,
    prof: Option<Prof>,
    /// Latent readbacks (see `latent_read`).
    latent: Latent,
    /// Serve local->host transfers with `latent_read`. Off by default: the
    /// game consumes a GS download every frame (VIF1 → memory), and a latent
    /// read returns data only if the GPU copy happens to be ready, which makes
    /// runs nondeterministic. `SSX3_GS_LATENT_READBACK=1` turns it on.
    pub(crate) latent_readback: bool,
    /// Per raster pass of the submission: primitives, tiles, entries.
    prof_info: Vec<(u32, u32, u32)>,
}

fn storage_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn swizzle_tables() -> Vec<u32> {
    let t = swizzle::tables();
    let mut v = Vec::with_capacity(45056);
    for part in [&t.t32, &t.z32, &t.t16, &t.t16s, &t.z16, &t.z16s, &t.t8, &t.t4] {
        v.extend(part.iter().map(|&x| x as u32));
    }
    debug_assert_eq!(v.len(), 45056);
    v
}

/// Offset of a format's table in `swizzle_tables()`.
fn table_offset(psm: u32) -> u32 {
    match psm {
        swizzle::PSMZ32 | swizzle::PSMZ24 => 2048,
        swizzle::PSMCT16 => 4096,
        swizzle::PSMCT16S => 8192,
        swizzle::PSMZ16 => 12288,
        swizzle::PSMZ16S => 16384,
        swizzle::PSMT8 => 20480,
        swizzle::PSMT4 => 28672,
        _ => 0,
    }
}

fn kind_index(k: Kind) -> u32 {
    match k {
        Kind::W32 => 0,
        Kind::W24 => 1,
        Kind::H16 => 2,
        Kind::B8 => 3,
        Kind::N4 => 4,
        Kind::Hi8 => 5,
        Kind::Hl4 => 6,
        Kind::Hh4 => 7,
    }
}

fn surf_words(s: &Surface) -> [u32; 3] {
    [
        s.base,
        table_offset(s.psm)
            | s.pw_shift << 16
            | s.ph_shift << 19
            | s.es << 22
            | kind_index(s.kind) << 24,
        s.pages_per_row,
    ]
}

fn bytes_of(v: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding; any byte view of it is valid.
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

fn f(v: f64) -> u32 {
    (v as f32).to_bits()
}

/// Clamp a real depth to `u32` (truncating, like `as u32` on the CPU).
fn z_u32(z: f64) -> u32 {
    z.clamp(0.0, u32::MAX as f64) as u32
}

/// Contiguous page range `[lo, hi)` covering a page set.
fn page_span(p: &Pages) -> Option<(u32, u32)> {
    let lo = p.0.iter().enumerate().find(|(_, w)| **w != 0)?;
    let hi = p.0.iter().enumerate().rev().find(|(_, w)| **w != 0)?;
    let lo = lo.0 as u32 * 64 + lo.1.trailing_zeros();
    let hi = hi.0 as u32 * 64 + 63 - hi.1.leading_zeros();
    Some((lo, hi + 1))
}

/// Identity of a decoded texture level (see `plan_decodes`).
type DecKey = ([u32; 3], [u32; 4], u32, u32);

/// The latent readback buffer (see `GpuGs::latent_read`).
struct Latent {
    buf: wgpu::Buffer,
    /// Set when the mapping of the last copy completed.
    ready: Arc<std::sync::atomic::AtomicBool>,
    /// Pages of the copy in flight or mapped.
    pending: Option<Pages>,
    /// Pages read this frame.
    want: Pages,
}

/// Runs `[a, b)` of consecutive pages in a set.
fn page_runs(p: &Pages) -> Vec<(u32, u32)> {
    let mut runs = Vec::new();
    let mut pg = 0;
    while pg < 512 {
        if !page_set(p, pg) {
            pg += 1;
            continue;
        }
        let start = pg;
        while pg < 512 && page_set(p, pg) {
            pg += 1;
        }
        runs.push((start, pg));
    }
    runs
}

fn page_set(p: &Pages, pg: u32) -> bool {
    p.0[(pg >> 6) as usize] >> (pg & 63) & 1 != 0
}

fn pages_and(a: &Pages, b: &Pages) -> Pages {
    let mut p = Pages::default();
    for i in 0..8 {
        p.0[i] = a.0[i] & b.0[i];
    }
    p
}

impl GpuGs {
    /// Create the backend; the current CPU VRAM is uploaded on first use.
    pub(crate) fn new() -> Option<GpuGs> {
        let ctx = context()?;
        let dev = &ctx.device;
        let mk = |label: &str, size: u64, usage: wgpu::BufferUsages| {
            dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size,
                usage,
                mapped_at_creation: false,
            })
        };
        use wgpu::BufferUsages as U;
        let vram = mk("gs-vram", VRAM_BYTES, U::STORAGE | U::COPY_DST | U::COPY_SRC);
        let snap = mk("gs-snapshot", VRAM_BYTES, U::STORAGE | U::COPY_DST);
        let tab = swizzle_tables();
        let tables = mk("gs-tables", tab.len() as u64 * 4, U::STORAGE | U::COPY_DST);
        ctx.queue.write_buffer(&tables, 0, bytes_of(&tab));
        let data = mk("gs-data", 16 << 20, U::STORAGE | U::COPY_DST | U::COPY_SRC);
        let params = mk("gs-params", 1024 * SLOT, U::UNIFORM | U::COPY_DST);
        let out = mk("gs-display", 1024 * 1024 * 4, U::STORAGE | U::COPY_SRC);
        let staging = mk("gs-readback", VRAM_BYTES, U::MAP_READ | U::COPY_DST);
        let dec = mk("gs-decoded", 16 << 20, U::STORAGE);

        let bgl = dev.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gs-bgl"),
            entries: &[
                storage_entry(0, false),
                storage_entry(1, true),
                storage_entry(2, true),
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                storage_entry(4, true),
                storage_entry(5, false),
                storage_entry(6, false),
            ],
        });
        let desc = wgpu::ShaderModuleDescriptor {
            label: Some("gs.wgsl"),
            // SSX3_GS_WGSL=path overrides the built-in shader (development).
            source: wgpu::ShaderSource::Wgsl(match std::env::var("SSX3_GS_WGSL") {
                Ok(p) => std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{p}: {e}")).into(),
                Err(_) => SHADER.into(),
            }),
        };
        // Without wgpu's injected bounds checks and loop bounding the
        // rasterizer is several times faster. Every index the shader forms
        // stays inside its buffer: VRAM, snapshot and table indices are
        // masked or bounded by construction, and `data` / `decoded` indices
        // are offsets the CPU encoded into buffers it sized.
        // SSX3_GS_CHECKED=1 keeps the checks (debugging).
        let module = if std::env::var_os("SSX3_GS_CHECKED").is_some() {
            dev.create_shader_module(desc)
        } else {
            // SAFETY: see above; the shader never indexes out of bounds.
            unsafe { dev.create_shader_module_trusted(desc, wgpu::ShaderRuntimeChecks::unchecked()) }
        };
        let layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gs-layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
        let pipe = |entry: &str| {
            dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: Some(&layout),
                module: &module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let raster_pipe = pipe("raster");
        let display_pipe = pipe("display");
        let decode_pipe = pipe("decode");
        let pixels_pipe = pipe("pixels");
        let bind_group = Self::make_bind_group(dev, &bgl, [&vram, &tables, &data, &params, &snap, &out, &dec]);
        Some(GpuGs {
            ctx,
            vram,
            snap,
            tables,
            data,
            params,
            out,
            dec,
            staging,
            bgl,
            bind_group,
            raster_pipe,
            display_pipe,
            decode_pipe,
            pixels_pipe,
            arena: Vec::with_capacity(1 << 20),
            slots: Vec::new(),
            cmds: Vec::new(),
            batch: GBatch::default(),
            scratch: Region::default(),
            last_draw: None,
            palettes: HashMap::new(),
            decoded: HashMap::new(),
            dec_used: 0,
            cpu_dirty: Pages::ALL,
            gpu_dirty: Pages::default(),
            inflight: VecDeque::new(),
            counts: vec![0; TILES_X * TILES_X],
            fill: vec![0; TILES_X * TILES_X],
            touched: Vec::new(),
            stats: GpuStats::default(),
            prof_info: Vec::new(),
            latent_readback: std::env::var_os("SSX3_GS_LATENT_READBACK").is_some(),
            latent: Latent {
                buf: mk("gs-latent", VRAM_BYTES, U::MAP_READ | U::COPY_DST),
                ready: Arc::new(std::sync::atomic::AtomicBool::new(false)),
                pending: None,
                want: Pages::default(),
            },
            prof: (profiling() && dev.features().contains(wgpu::Features::TIMESTAMP_QUERY)).then(|| Prof {
                qs: dev.create_query_set(&wgpu::QuerySetDescriptor {
                    label: Some("gs-timestamps"),
                    ty: wgpu::QueryType::Timestamp,
                    count: PROF_PASSES * 2,
                }),
                resolve: mk("gs-ts-resolve", PROF_PASSES as u64 * 16, U::QUERY_RESOLVE | U::COPY_SRC),
                read: mk("gs-ts-read", PROF_PASSES as u64 * 16, U::MAP_READ | U::COPY_DST),
            }),
        })
    }

    /// Bindings: vram, tables, data, params (dynamic uniform), snapshot,
    /// display output, decoded textures.
    fn make_bind_group(dev: &wgpu::Device, bgl: &wgpu::BindGroupLayout, b: [&wgpu::Buffer; 7]) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = b
            .iter()
            .enumerate()
            .map(|(i, buf)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: if i == 3 {
                    wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: buf,
                        offset: 0,
                        size: wgpu::BufferSize::new(64),
                    })
                } else {
                    buf.as_entire_binding()
                },
            })
            .collect();
        dev.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gs-bg"),
            layout: bgl,
            entries: &entries,
        })
    }

    pub(crate) fn device(&self) -> &'static GpuContext {
        self.ctx
    }

    // ------------------------------------------------------------ setup records

    fn palette(&mut self, pal: &[u32]) -> u32 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325 ^ pal.len() as u64;
        for &v in pal {
            h = (h ^ v as u64).wrapping_mul(0x0000_0100_0000_01B3);
        }
        if let Some(&off) = self.palettes.get(&h) {
            let o = off as usize;
            if self.arena[o..o + pal.len()] == *pal {
                return off;
            }
        }
        let off = self.arena.len() as u32;
        self.arena.extend_from_slice(pal);
        self.palettes.insert(h, off);
        self.stats.palettes += 1;
        off
    }

    /// Append the setup record of `d`; returns its word offset.
    fn setup(&mut self, d: &Draw) -> u32 {
        let mut w = [0u32; SETUP_WORDS];
        let mut flags = 0;
        let bit = |c: bool, b: u32| if c { b } else { 0 };
        flags |= bit(d.tex.is_some(), F_TME);
        flags |= bit(d.fge, F_FGE);
        flags |= bit(d.abe && d.blend != Blend::Src, F_ABE);
        flags |= bit(d.fst, F_FST);
        flags |= bit(d.zte, F_ZTE);
        flags |= bit(d.z_used, F_ZUSED);
        flags |= bit(d.zmsk, F_ZMSK);
        flags |= bit(d.ate, F_ATE);
        flags |= bit(d.date && d.fb_fmt != FbFmt::C24, F_DATE);
        flags |= bit(d.datm, F_DATM);
        flags |= bit(d.pabe, F_PABE);
        flags |= bit(d.colclamp, F_COLCLAMP);
        flags |= bit(d.fba, F_FBA);
        flags |= bit(d.tcc, F_TCC);
        flags |= bit(d.mip.is_some(), F_MIP);
        flags |= bit(d.dither.is_some(), F_DITHER);
        w[1..4].copy_from_slice(&surf_words(&d.fb));
        w[4] = d.fbmsk;
        w[5..8].copy_from_slice(&surf_words(&d.zb));
        w[8] = d.zmax;
        let fbfmt = match d.fb_fmt {
            FbFmt::C32 => 0,
            FbFmt::C24 => 1,
            FbFmt::C16 => 2,
        };
        let mut modes = d.ztst
            | d.atst << 2
            | d.afail << 5
            | d.ba << 7
            | d.bb << 9
            | d.bc << 11
            | d.bd << 13
            | d.tfx << 15
            | d.scanmsk << 21
            | fbfmt << 23;
        let mut refs = (d.aref as u32 & 0xFF) | (d.fix as u32 & 0xFF) << 8;
        w[11] = d.sx0 as u32 | (d.sx1 as u32) << 16;
        w[12] = d.sy0 as u32 | (d.sy1 as u32) << 16;
        w[13] = d.fogcol[0] as u32 | (d.fogcol[1] as u32) << 8 | (d.fogcol[2] as u32) << 16;
        if let Some(m) = &d.dither {
            let mut dimx = 0u64;
            for (i, row) in m.iter().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    dimx |= ((*v as u64) & 7) << (i * 16 + j * 4);
                }
            }
            w[19] = dimx as u32;
            w[20] = (dimx >> 32) as u32;
        }
        w[18] = u32::MAX;
        for l in 0..7 {
            w[S_DEC + 3 * l] = u32::MAX;
        }
        if let Some(t) = &d.tex {
            flags |= bit(t.aem, F_AEM) | bit(t.mag_linear, F_MAGLIN) | bit(t.lcm, F_LCM);
            let kind = match t.kind {
                TexKind::C32 => 0,
                TexKind::C24 => 1,
                TexKind::C16 => 2,
                TexKind::Pal => 3,
            };
            modes |= t.wms << 17 | t.wmt << 19 | kind << 25 | ((t.pal_mask == 255) as u32) << 27;
            refs |= t.ta0 << 16 | t.ta1 << 24;
            let (mxl, mmin) = d.mip.as_ref().map(|m| (m.mxl, m.mmin)).unwrap_or((0, 0));
            w[14] = t.tw as u32 | (t.th as u32) << 4 | t.l << 8 | mxl << 12 | mmin << 16;
            w[15] = t.minu as u32 | (t.maxu as u32) << 16;
            w[16] = t.minv as u32 | (t.maxv as u32) << 16;
            w[17] = t.k.to_bits();
            w[21..24].copy_from_slice(&surf_words(&t.surf));
            if let Some(m) = &d.mip {
                for (i, l) in m.levels.iter().enumerate().take(6) {
                    let at = 24 + 3 * i;
                    w[at..at + 3].copy_from_slice(&surf_words(&l.surf));
                }
            }
            if t.kind == TexKind::Pal {
                let pal: Vec<u32> = t.palette[..=t.pal_mask].to_vec();
                w[18] = self.palette(&pal);
            }
        }
        w[0] = flags;
        w[9] = modes;
        w[10] = refs;
        let off = self.arena.len() as u32;
        self.arena.extend_from_slice(&w);
        self.stats.setups += 1;
        off
    }

    // ------------------------------------------------------------ primitives

    /// Queue a set-up primitive. `vram` is the CPU mirror (for uploads when
    /// a batch closes).
    pub(crate) fn push(&mut self, d: &Arc<Draw>, p: &Prim, prim: u32, verts: &[Vertex], vram: &[u32]) {
        let (x0, y0, x1, y1) = (p.x0 as u32, p.y0 as u32, p.x1 as u32, p.y1 as u32);
        let fb_pages = Pages::rect(&d.fb, x0, y0, x1, y1);
        let zb_pages = if d.z_used {
            Pages::rect(&d.zb, x0, y0, x1, y1)
        } else {
            Pages::default()
        };
        let mut wpages = fb_pages;
        wpages.union(&zb_pages);
        // Pixels that alias other pixels of the same primitive: drawn in a
        // batch of their own (still racy inside, but ordered against the rest).
        let isolate = p.x1 >= d.fb_width
            || (d.z_used && p.x1 >= d.zb_width)
            || fb_pages == Pages::ALL
            || (d.z_used && raster::surf_key(&d.fb) != raster::surf_key(&d.zb) && fb_pages.hits(&zb_pages));
        if isolate {
            self.stats.isolated += 1;
            self.close(vram);
        }
        // Different mapping of pages the batch draws to.
        if self.batch.prims > 0 {
            // (No heap allocation: this runs for every primitive.)
            let both = [(raster::surf_key(&d.fb), fb_pages), (raster::surf_key(&d.zb), zb_pages)];
            let mine = &both[..if d.z_used { 2 } else { 1 }];
            let alias = mine.iter().any(|(k, pg)| {
                self.batch
                    .surfs
                    .iter()
                    .any(|(k2, p2)| k != k2 && pg.hits(p2))
            });
            if alias {
                self.stats.alias_closes += 1;
                self.close(vram);
            }
        }
        // Read after write: the texels this primitive can reach, against the
        // blocks the batch writes.
        if let Some(ts) = d.tex_surface() {
            if self.batch.prims > 0 && d.tex_pages.hits(&self.batch.write.pages) {
                // Reused scratch region (a fresh one is a 2 KiB allocation).
                let r = &mut self.scratch;
                r.clear();
                match raster::prim_tex_rect(d, p, prim, verts) {
                    Some([u0, v0, u1, v1]) => r.add_rect(ts, u0, v0, u1, v1),
                    None => r.add_pages(&d.tex_pages),
                }
                if self.batch.write.hits(&self.scratch) {
                    self.stats.raw_closes += 1;
                    self.close(vram);
                }
            }
            self.batch.read.union(&d.tex_pages);
        }
        // Setup record (after any close: records stay valid until submit).
        let st = match &self.last_draw {
            Some((ld, off)) if Arc::ptr_eq(ld, d) => *off,
            _ => {
                let off = self.setup(d);
                self.last_draw = Some((d.clone(), off));
                off
            }
        };
        if d.tex.is_some() {
            let fetches = p.area as u64 * if p.linear || d.mip.is_some() { 4 } else { 1 };
            match self.batch.tex_use.last_mut() {
                Some((s2, _, n)) if *s2 == st => *n += fetches,
                _ => self.batch.tex_use.push((st, d.clone(), fetches)),
            }
        }
        let off = self.encode(d, p, st);
        // Bookkeeping.
        let b = &mut self.batch;
        let add = |surfs: &mut Vec<((u32, u32, u32), Pages)>, s: &Surface, pg: Pages| {
            if pg.is_empty() {
                return;
            }
            let k = raster::surf_key(s);
            match surfs.iter_mut().find(|(k2, _)| *k2 == k) {
                Some((_, p2)) => p2.union(&pg),
                None => surfs.push((k, pg)),
            }
        };
        add(&mut b.surfs, &d.fb, fb_pages);
        b.write.add(&d.fb, [x0, y0, x1, y1], &fb_pages);
        if d.z_used {
            add(&mut b.surfs, &d.zb, zb_pages);
            b.write.add(&d.zb, [x0, y0, x1, y1], &zb_pages);
        }
        b.prims += 1;
        self.stats.prims += 1;
        self.bin(p, off);
        if isolate || self.batch.prims >= MAX_BATCH_PRIMS {
            self.close(vram);
        }
    }

    /// Tile spans of a primitive.
    fn bin(&mut self, p: &Prim, off: u32) {
        let n0 = self.batch.spans.len();
        self.bin_spans(p, off);
        let kind = match p.shape {
            Shape::Tri(_) => 0,
            Shape::Sprite { .. } => 1,
            Shape::Line { .. } => 2,
            Shape::Point { .. } => 3,
        };
        let tiles: u64 = self.batch.spans[n0..].iter().map(|s| (s.tx1 - s.tx0 + 1) as u64).sum();
        self.stats.kind_prims[kind] += 1;
        self.stats.kind_entries[kind] += tiles;
    }

    fn bin_spans(&mut self, p: &Prim, off: u32) {
        // The record's bounding box (lines are bounded by their endpoints).
        let off_lo = self.arena[off as usize + 2];
        let off_hi = self.arena[off as usize + 3];
        let (bx0, by0) = ((off_lo & 0xFFFF) as i32, (off_lo >> 16) as i32);
        let (bx1, by1) = ((off_hi & 0xFFFF) as i32, (off_hi >> 16) as i32);
        if bx0 > bx1 || by0 > by1 {
            return;
        }
        let (tx0, tx1) = ((bx0 >> TILE_SHIFT) as u16, (bx1 >> TILE_SHIFT) as u16);
        let (ty0, ty1) = ((by0 >> TILE_SHIFT) as u16, (by1 >> TILE_SHIFT) as u16);
        if let Shape::Tri(t) = &p.shape {
            if tx1 - tx0 >= 2 && ty1 > ty0 {
                // Narrow each tile row to the triangle's extent in it.
                for ty in ty0..=ty1 {
                    let ja = ((ty as i32) << TILE_SHIFT).max(p.y0) - t.by0;
                    let jb = (((ty as i32) << TILE_SHIFT) + (1 << TILE_SHIFT) - 1).min(p.y1) - t.by0;
                    let w = (p.x1 - t.bx0) as i64;
                    let (mut lo, mut hi) = (0i64, w);
                    for k in 0..3 {
                        let (ea, eb) = (t.e0[k] + t.edy[k] * ja as i64, t.e0[k] + t.edy[k] * jb as i64);
                        // The most permissive end of the slab for this edge.
                        let e = if t.edx[k] > 0 { ea.max(eb) } else { ea.max(eb) };
                        raster::edge_span(e, t.edx[k], &mut lo, &mut hi);
                    }
                    if lo > hi {
                        continue;
                    }
                    let a = ((t.bx0 as i64 + lo) >> TILE_SHIFT) as u16;
                    let b = ((t.bx0 as i64 + hi) >> TILE_SHIFT) as u16;
                    self.batch.spans.push(Span { prim: off, ty, tx0: a, tx1: b });
                }
                return;
            }
        }
        for ty in ty0..=ty1 {
            self.batch.spans.push(Span { prim: off, ty, tx0, tx1 });
        }
    }

    /// Append the record of `p`; returns its word offset.
    fn encode(&mut self, d: &Draw, p: &Prim, st: u32) -> u32 {
        use raster::{A, B, F, G, NATTR, Q, R, S, T, Z};
        let mut w = [0u32; PRIM_WORDS];
        w[0] = st;
        let lin = if p.linear { PK_LINEAR } else { 0 };
        let (mut x0, mut y0, mut x1, mut y1) = (p.x0, p.y0, p.x1, p.y1);
        match &p.shape {
            Shape::Tri(t) => {
                w[1] = lin;
                let (wd, ht) = ((p.x1 - t.bx0) as i64, (p.y1 - t.by0) as i64);
                let fits = (0..3).all(|k| {
                    [(0, 0), (wd, 0), (0, ht), (wd, ht)].iter().all(|&(i, j)| {
                        let e = t.e0[k] + t.edx[k] * i + t.edy[k] * j;
                        e.abs() < (1i64 << 31) - 1
                    })
                });
                if fits {
                    for k in 0..3 {
                        w[4 + k] = t.e0[k] as i32 as u32;
                        w[7 + k] = t.edx[k] as i32 as u32;
                        w[10 + k] = t.edy[k] as i32 as u32;
                    }
                } else {
                    // Exact per-row spans (as `tri_rows` computes them).
                    w[1] |= PK_BIG;
                    self.stats.big_tris += 1;
                    let at = self.arena.len() as u32;
                    w[4] = at;
                    for j in 0..=ht {
                        let (mut lo, mut hi) = (0i64, wd);
                        for k in 0..3 {
                            raster::edge_span(t.e0[k] + t.edy[k] * j, t.edx[k], &mut lo, &mut hi);
                        }
                        self.arena.push(if lo > hi {
                            0xFFFF
                        } else {
                            (t.bx0 as i64 + lo) as u32 | ((t.bx0 as i64 + hi) as u32) << 16
                        });
                    }
                }
                let dy = t.by0 as f64 - t.oy;
                let order = [R, G, B, A, S, T, Q, F];
                for (i, &k) in order.iter().enumerate() {
                    w[13 + i] = f(t.c[k] + t.dady[k] * dy);
                    w[21 + i] = f(t.dadx[k]);
                    w[29 + i] = f(t.dady[k]);
                }
                let cz = t.c[Z] + t.dady[Z] * dy;
                let base = cz.floor().clamp(0.0, u32::MAX as f64);
                w[37] = base as u32;
                w[38] = f(cz - base);
                w[39] = f(t.dadx[Z]);
                w[40] = f(t.dady[Z]);
            }
            Shape::Sprite {
                x0: sx0,
                y0: sy0,
                a0,
                at,
                dsdx,
                dtdy,
                base,
                ..
            } => {
                w[1] = 1 | lin;
                w[4] = *sx0 as u32;
                w[5] = *sy0 as u32;
                w[6] = f(a0[S]);
                w[7] = f(a0[T]);
                w[8] = f(*dsdx);
                w[9] = f(*dtdy);
                w[10] = f(at[Q]);
                w[11] = (base.r as u32 & 0xFF)
                    | (base.g as u32 & 0xFF) << 8
                    | (base.b as u32 & 0xFF) << 16
                    | (base.a as u32 & 0xFF) << 24;
                w[12] = base.f as u32;
                w[13] = z_u32(base.z);
            }
            Shape::Line {
                x0: lx0,
                y0: ly0,
                x1: lx1,
                y1: ly1,
                a0,
                a1,
            } => {
                w[1] = 2 | lin;
                w[4] = f(*lx0);
                w[5] = f(*ly0);
                w[6] = f(*lx1);
                w[7] = f(*ly1);
                w[8] = (lx1 - lx0).abs().max((ly1 - ly0).abs()).round() as i32 as u32;
                let order = [R, G, B, A, S, T, Q, F];
                for (i, &k) in order.iter().enumerate() {
                    w[9 + i] = f(a0[k]);
                    w[17 + i] = f(a1[k]);
                }
                w[27] = z_u32(a0[Z]);
                w[28] = f(a1[Z] - a0[Z]);
                debug_assert_eq!(NATTR, 9);
                // The CPU walks the whole scissor; bound by the endpoints.
                x0 = x0.max(lx0.min(*lx1).floor() as i32 - 1);
                x1 = x1.min(lx0.max(*lx1).ceil() as i32 + 1);
                y0 = y0.max(ly0.min(*ly1).floor() as i32 - 1);
                y1 = y1.min(ly0.max(*ly1).ceil() as i32 + 1);
            }
            Shape::Point { x, y, f: fr } => {
                w[1] = 3 | lin;
                w[4] = *x as u32;
                w[5] = *y as u32;
                w[6] = (fr.r as u32 & 0xFF)
                    | (fr.g as u32 & 0xFF) << 8
                    | (fr.b as u32 & 0xFF) << 16
                    | (fr.a as u32 & 0xFF) << 24;
                w[7] = fr.u.to_bits();
                w[8] = fr.v.to_bits();
                w[9] = fr.q.to_bits();
                w[10] = fr.f as u32;
                w[11] = z_u32(fr.z);
            }
        }
        let _ = d;
        if x0 > x1 || y0 > y1 {
            // Empty after bounding (a line outside the scissor): never covers.
            x0 = 1;
            x1 = 0;
        }
        w[2] = (x0.max(0) as u32) | (y0.max(0) as u32) << 16;
        w[3] = (x1.max(0) as u32) | (y1.max(0) as u32) << 16;
        let off = self.arena.len() as u32;
        self.arena.extend_from_slice(&w);
        off
    }

    // ------------------------------------------------------------ batches

    /// Queue uploads of the CPU-written pages.
    fn emit_uploads(&mut self, vram: &[u32]) {
        if self.cpu_dirty.is_empty() {
            return;
        }
        let mut page = 0u32;
        while page < 512 {
            let set = |p: u32| self.cpu_dirty.0[(p >> 6) as usize] >> (p & 63) & 1 != 0;
            if !set(page) {
                page += 1;
                continue;
            }
            let start = page;
            while page < 512 && set(page) {
                page += 1;
            }
            let src = self.arena.len() as u32;
            let (a, b) = (start as usize * PAGE_WORDS, page as usize * PAGE_WORDS);
            self.arena.extend_from_slice(&vram[a..b]);
            self.cmds.push(Cmd::Upload {
                page: start,
                n: page - start,
                src,
            });
            self.stats.upload_pages += (page - start) as u64;
        }
        let up = self.cpu_dirty;
        self.decoded.retain(|_, (_, pg)| !pg.hits(&up));
        self.cpu_dirty = Pages::default();
    }

    /// Close the open batch: uploads, then the raster dispatch.
    pub(crate) fn close(&mut self, vram: &[u32]) {
        self.emit_uploads(vram);
        if self.batch.prims == 0 {
            self.batch.clear();
            return;
        }
        // Tile lists.
        let spans = std::mem::take(&mut self.batch.spans);
        self.touched.clear();
        for s in &spans {
            let row = s.ty as usize * TILES_X;
            for tx in s.tx0..=s.tx1 {
                let t = row + tx as usize;
                if self.counts[t] == 0 {
                    self.touched.push(t as u32);
                }
                self.counts[t] += 1;
            }
        }
        let ntiles = self.touched.len();
        let recs = self.arena.len();
        let mut cursor = recs + ntiles * 3;
        self.arena.resize(cursor, 0);
        for (i, &t) in self.touched.iter().enumerate() {
            let t = t as usize;
            let n = self.counts[t];
            self.arena[recs + i * 3] = (t % TILES_X) as u32 | ((t / TILES_X) as u32) << 16;
            self.arena[recs + i * 3 + 1] = cursor as u32;
            self.arena[recs + i * 3 + 2] = n;
            self.fill[t] = cursor as u32;
            cursor += n as usize;
            self.counts[t] = 0;
        }
        let entries = cursor - recs - ntiles * 3;
        self.arena.resize(cursor, 0);
        for s in &spans {
            let row = s.ty as usize * TILES_X;
            for tx in s.tx0..=s.tx1 {
                let t = row + tx as usize;
                let at = self.fill[t] as usize;
                self.arena[at] = s.prim;
                self.fill[t] += 1;
            }
        }
        self.batch.spans = spans;
        // Texture reads come from a copy of the batch's texture pages taken
        // before it: plain (non-atomic) loads, and pages the batch draws
        // read as they were before the batch.
        let read = self.batch.read;
        let (slo, shi) = match page_span(&read) {
            Some((lo, hi)) => {
                let mut pg = lo;
                while pg < hi {
                    if !page_set(&read, pg) {
                        pg += 1;
                        continue;
                    }
                    let start = pg;
                    while pg < hi && page_set(&read, pg) {
                        pg += 1;
                    }
                    self.cmds.push(Cmd::Snapshot {
                        lo: start * PAGE_WORDS as u32,
                        hi: pg * PAGE_WORDS as u32,
                    });
                    self.stats.snapshot_pages += (pg - start) as u64;
                }
                self.stats.snapshots += 1;
                (lo * PAGE_WORDS as u32, hi * PAGE_WORDS as u32)
            }
            None => (0, 0),
        };
        self.plan_decodes(slo, shi);
        let slot = self.slots.len() as u32;
        let mut s = [0u32; 16];
        s[..4].copy_from_slice(&[recs as u32, ntiles as u32, slo, shi]);
        self.slots.push(s);
        self.cmds.push(Cmd::Raster {
            slot,
            tiles: ntiles as u32,
        });
        if self.prof.is_some() {
            self.prof_info.push((self.batch.prims as u32, ntiles as u32, entries as u32));
        }
        self.stats.batches += 1;
        self.stats.tiles += ntiles as u64;
        self.stats.entries += entries as u64;
        let written = self.batch.write.pages;
        self.gpu_dirty.union(&written);
        self.decoded.retain(|_, (_, pg)| !pg.hits(&written));
        // Setup records carry this batch's decoded textures.
        self.last_draw = None;
        self.batch.clear();
    }

    /// Decode the textures this batch samples often enough into the decoded
    /// buffer (RGBA8, the whole reachable rectangle of each level), and
    /// point the setup records at them. Decoding happens on the GPU right
    /// before the batch, from the same snapshot the batch would sample.
    fn plan_decodes(&mut self, slo: u32, shi: u32) {
        static OFF: OnceLock<bool> = OnceLock::new();
        if *OFF.get_or_init(|| std::env::var_os("SSX3_GS_NO_DECODE").is_some()) {
            return;
        }
        let uses = std::mem::take(&mut self.batch.tex_use);
        for (st, d, fetches) in &uses {
            for (l, t) in d.tex_levels().into_iter().enumerate().take(7) {
                let Some(rect) = raster::decode_rect(t) else { continue };
                let [u0, v0, u1, v1] = rect;
                let (w, h) = (u1 - u0 + 1, v1 - v0 + 1);
                let texels = w * h;
                if self.dec_used + texels > 64 << 20 {
                    continue;
                }
                // Worth it when sampled about as often as it has texels.
                if (*fetches as u32) < texels / 2 && texels > 4096 {
                    continue;
                }
                // Surface, rectangle, conversion; palettes are deduplicated in
                // the arena, so their offset identifies their contents.
                let key: DecKey = (
                    surf_words(t.surface()),
                    rect,
                    t.kind as u32 | (t.aem as u32) << 2 | t.ta0 << 8 | t.ta1 << 16,
                    self.arena[*st as usize + 18],
                );
                let off = match self.decoded.get(&key) {
                    Some(&(off, _)) => off,
                    None => {
                        let off = self.dec_used;
                        self.dec_used += texels;
                        let pages = Pages::rect(t.surface(), u0, v0, u1, v1);
                        self.decoded.insert(key, (off, pages));
                        let slot = self.slots.len() as u32;
                        let mut s = [0u32; 16];
                        s[..8].copy_from_slice(&[*st, l as u32, slo, shi, off, u0, v0, w]);
                        s[8] = texels;
                        self.slots.push(s);
                        self.cmds.push(Cmd::Decode { slot, n: texels });
                        self.stats.decodes += 1;
                        self.stats.decoded_texels += texels as u64;
                        off
                    }
                };
                let at = *st as usize + S_DEC + 3 * l;
                self.arena[at] = off;
                self.arena[at + 1] = u0 | v0 << 16;
                self.arena[at + 2] = w;
            }
        }
        self.batch.tex_use = uses;
    }

    /// Whether the open batch reads or writes blocks of `r`.
    pub(crate) fn batch_touches(&mut self, r: &Region) -> bool {
        self.batch.prims > 0 && (r.pages.hits(&self.batch.read) || self.batch.write.hits(r))
    }

    /// Host->local data for `r` in a partial-word format: when those pages
    /// were drawn on the GPU, write the pixels on the GPU (`pixel_write`)
    /// rather than reading the pages back. Returns whether to do so.
    pub(crate) fn direct_upload(&mut self, r: &Region, vram: &[u32]) -> bool {
        if std::env::var_os("SSX3_GS_NO_DIRECT").is_some() {
            return false;
        }
        if !self.gpu_dirty.hits(&r.pages) && !self.batch_writes(r) {
            return false;
        }
        if self.batch_touches(r) {
            self.stats.transfer_closes += 1;
            self.close(vram);
        }
        // Earlier CPU writes to these pages go first.
        self.emit_uploads(vram);
        self.gpu_dirty.union(&r.pages);
        let pages = r.pages;
        self.decoded.retain(|_, (_, pg)| !pg.hits(&pages));
        true
    }

    /// Queue host->local pixels: `pos` = (DSAX, DSAY, RRW, index of the
    /// first pixel in the transfer rectangle).
    pub(crate) fn pixel_write(&mut self, s: &Surface, pos: [u32; 4], vals: &[u32]) {
        // GIF delivers images in small pieces: extend the previous write
        // when this continues it.
        if let Some(Cmd::Pixels { slot, n }) = self.cmds.last_mut() {
            let p = &mut self.slots[*slot as usize];
            let sw = surf_words(s);
            if p[..3] == sw
                && p[4..7] == pos[..3]
                && p[7] + p[3] == pos[3]
                && (p[8] + p[3]) as usize == self.arena.len()
            {
                self.arena.extend_from_slice(vals);
                p[3] += vals.len() as u32;
                *n += vals.len() as u32;
                self.stats.direct_upload_pixels += vals.len() as u64;
                return;
            }
        }
        let at = self.arena.len() as u32;
        self.arena.extend_from_slice(vals);
        let sw = surf_words(s);
        if trace_readback() {
            eprintln!("[gs-gpu] pixel write {} px at {pos:?} surf {sw:x?}", vals.len());
        }
        let mut p = [0u32; 16];
        p[..4].copy_from_slice(&[sw[0], sw[1], sw[2], vals.len() as u32]);
        p[4..8].copy_from_slice(&pos);
        p[8] = at;
        let slot = self.slots.len() as u32;
        self.slots.push(p);
        self.cmds.push(Cmd::Pixels {
            slot,
            n: vals.len() as u32,
        });
        self.stats.direct_upload_pixels += vals.len() as u64;
    }

    /// Whether the open batch writes blocks of `r`.
    pub(crate) fn batch_writes(&mut self, r: &Region) -> bool {
        self.batch.prims > 0 && self.batch.write.hits(r)
    }

    pub(crate) fn note_transfer_close(&mut self) {
        self.stats.transfer_closes += 1;
    }

    // ------------------------------------------------------------ submission

    fn ensure_capacity(&mut self) {
        let dev = &self.ctx.device;
        let need = (self.arena.len() * 4) as u64;
        let mut rebind = false;
        if need > self.data.size() {
            let size = need.next_power_of_two().max(16 << 20);
            self.data = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gs-data"),
                size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
            rebind = true;
        }
        let pneed = self.slots.len() as u64 * SLOT;
        if pneed > self.params.size() {
            let size = pneed.next_power_of_two();
            self.params = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gs-params"),
                size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            rebind = true;
        }
        let dneed = self.dec_used as u64 * 4;
        if dneed > self.dec.size() {
            self.dec = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gs-decoded"),
                size: dneed.next_power_of_two(),
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            });
            rebind = true;
        }
        if rebind {
            self.rebind();
        }
    }

    fn rebind(&mut self) {
        self.bind_group = Self::make_bind_group(
            &self.ctx.device,
            &self.bgl,
            [&self.vram, &self.tables, &self.data, &self.params, &self.snap, &self.out, &self.dec],
        );
    }

    fn ensure_out(&mut self, words: u64) {
        if words * 4 > self.out.size() {
            self.out = self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gs-display"),
                size: (words * 4).next_power_of_two(),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
            self.rebind();
        }
    }

    /// Encode and submit everything queued (the open batch stays open).
    /// `extra` may append copies (readbacks) after the queued work.
    fn submit_with(&mut self, extra: impl FnOnce(&Self, &mut wgpu::CommandEncoder)) {
        self.ensure_capacity();
        let q = &self.ctx.queue;
        if !self.arena.is_empty() {
            q.write_buffer(&self.data, 0, bytes_of(&self.arena));
            self.stats.data_bytes += self.arena.len() as u64 * 4;
        }
        if !self.slots.is_empty() {
            let mut bytes = vec![0u8; self.slots.len() * SLOT as usize];
            for (i, s) in self.slots.iter().enumerate() {
                bytes[i * SLOT as usize..i * SLOT as usize + 64].copy_from_slice(bytes_of(s));
            }
            q.write_buffer(&self.params, 0, &bytes);
        }
        let mut enc = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("gs") });
        let mut passes = 0u32;
        for c in &self.cmds {
            match *c {
                Cmd::Upload { page, n, src } => enc.copy_buffer_to_buffer(
                    &self.data,
                    src as u64 * 4,
                    &self.vram,
                    page as u64 * PAGE_WORDS as u64 * 4,
                    n as u64 * PAGE_WORDS as u64 * 4,
                ),
                Cmd::Snapshot { lo, hi } => enc.copy_buffer_to_buffer(
                    &self.vram,
                    lo as u64 * 4,
                    &self.snap,
                    lo as u64 * 4,
                    (hi - lo) as u64 * 4,
                ),
                Cmd::Raster { slot, tiles } => {
                    let ts = self.prof.as_ref().filter(|_| passes < PROF_PASSES).map(|p| {
                        passes += 1;
                        wgpu::ComputePassTimestampWrites {
                            query_set: &p.qs,
                            beginning_of_pass_write_index: Some(2 * passes - 2),
                            end_of_pass_write_index: Some(2 * passes - 1),
                        }
                    });
                    let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("gs-raster"),
                        timestamp_writes: ts,
                    });
                    pass.set_pipeline(&self.raster_pipe);
                    pass.set_bind_group(0, &self.bind_group, &[slot * SLOT as u32]);
                    pass.dispatch_workgroups(tiles.min(32768), tiles.div_ceil(32768), 1);
                }
                Cmd::Pixels { slot, n } => {
                    let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("gs-pixels"),
                        timestamp_writes: None,
                    });
                    pass.set_pipeline(&self.pixels_pipe);
                    pass.set_bind_group(0, &self.bind_group, &[slot * SLOT as u32]);
                    let groups = n.div_ceil(64);
                    pass.dispatch_workgroups(groups.min(32768), groups.div_ceil(32768), 1);
                }
                Cmd::Decode { slot, n } => {
                    let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("gs-decode"),
                        timestamp_writes: None,
                    });
                    pass.set_pipeline(&self.decode_pipe);
                    pass.set_bind_group(0, &self.bind_group, &[slot * SLOT as u32]);
                    let groups = n.div_ceil(64);
                    pass.dispatch_workgroups(groups.min(32768), groups.div_ceil(32768), 1);
                }
                Cmd::Display { slot, w, h } => {
                    let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("gs-display"),
                        timestamp_writes: None,
                    });
                    pass.set_pipeline(&self.display_pipe);
                    pass.set_bind_group(0, &self.bind_group, &[slot * SLOT as u32]);
                    pass.dispatch_workgroups(w.div_ceil(8), h.div_ceil(8), 1);
                }
            }
        }
        extra(self, &mut enc);
        if let (Some(p), true) = (&self.prof, passes > 0) {
            enc.resolve_query_set(&p.qs, 0..2 * passes, &p.resolve, 0);
            enc.copy_buffer_to_buffer(&p.resolve, 0, &p.read, 0, passes as u64 * 16);
        }
        let idx = q.submit(std::iter::once(enc.finish()));
        if let (Some(p), true) = (&self.prof, passes > 0) {
            let slice = p.read.slice(0..passes as u64 * 16);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            let _ = self.ctx.device.poll(wgpu::PollType::wait_indefinitely());
            let period = q.get_timestamp_period() as f64;
            {
                let v = slice.get_mapped_range().expect("timestamps");
                let t: Vec<u64> = v.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
                let verbose = std::env::var("SSX3_GS_PROFILE").is_ok_and(|v| v == "2");
                for (i, pair) in t.chunks_exact(2).enumerate() {
                    let ns = (pair[1].saturating_sub(pair[0]) as f64 * period) as u64;
                    self.stats.gpu_raster_ns += ns;
                    if verbose {
                        let (pr, ti, en) = self.prof_info.get(i).copied().unwrap_or_default();
                        eprintln!("[gs-prof] pass {i}: {:.3} ms, {pr} prims, {ti} tiles, {en} entries", ns as f64 / 1e6);
                    }
                }
            }
            p.read.unmap();
        }
        self.prof_info.clear();
        self.stats.submits += 1;
        self.arena.clear();
        self.slots.clear();
        self.cmds.clear();
        self.palettes.clear();
        self.decoded.clear();
        self.dec_used = 0;
        self.last_draw = None;
        self.inflight.push_back(idx);
        while self.inflight.len() > MAX_INFLIGHT {
            let old = self.inflight.pop_front().expect("inflight");
            let _ = self.ctx.device.poll(wgpu::PollType::Wait {
                submission_index: Some(old),
                timeout: None,
            });
        }
    }

    /// Close the batch and submit everything (no waiting).
    pub(crate) fn submit(&mut self, vram: &[u32]) {
        self.close(vram);
        let want = self.latent.want;
        if !want.is_empty() && self.latent.pending.is_none() {
            // Copy the pages local->host transfers read this frame, as they
            // are once the frame is drawn, for the next frame's transfers.
            let runs = page_runs(&want);
            let bpp = PAGE_WORDS as u64 * 4;
            self.submit_with(|s, enc| {
                for &(a, b) in &runs {
                    let off = a as u64 * bpp;
                    enc.copy_buffer_to_buffer(&s.vram, off, &s.latent.buf, off, (b - a) as u64 * bpp);
                }
            });
            let ready = self.latent.ready.clone();
            ready.store(false, std::sync::atomic::Ordering::Release);
            self.latent.buf.slice(..).map_async(wgpu::MapMode::Read, move |r| {
                ready.store(r.is_ok(), std::sync::atomic::Ordering::Release);
            });
            self.latent.pending = Some(want);
            self.latent.want = Pages::default();
            return;
        }
        if !self.cmds.is_empty() {
            self.submit_with(|_, _| {});
        }
    }

    /// Serve a local->host transfer of GPU-drawn `pages` without waiting
    /// for the GPU: the mirror gets those pages as they were at the end of
    /// the previous frame (when that copy has arrived), and the pages are
    /// copied again at the end of this frame. SSX 3 reads small rectangles
    /// of the Z buffer every frame (light flare occlusion); a synchronous
    /// readback would make the CPU wait for the whole frame's drawing.
    /// `SSX3_GS_SYNC_READBACK=1` disables this.
    pub(crate) fn latent_read(&mut self, pages: &Pages, vram: &mut [u32]) {
        self.latent.want.union(pages);
        self.stats.latent_reads += 1;
        let Some(have) = self.latent.pending else { return };
        let _ = self.ctx.device.poll(wgpu::PollType::Poll);
        if !self.latent.ready.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        {
            let view = self.latent.buf.slice(..).get_mapped_range().expect("latent map");
            // Only pages the GPU drew: the others are current in the mirror.
            let pg = pages_and(&have, &self.gpu_dirty);
            for (a, b) in page_runs(&pg) {
                let (wa, wb) = (a as usize * PAGE_WORDS, b as usize * PAGE_WORDS);
                for (d, c) in vram[wa..wb].iter_mut().zip(view[wa * 4..wb * 4].chunks_exact(4)) {
                    *d = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                }
            }
        }
        self.latent.buf.unmap();
        self.latent.pending = None;
    }


    /// Wait until the GPU has finished everything submitted.
    pub(crate) fn wait_idle(&mut self) {
        let _ = self.ctx.device.poll(wgpu::PollType::wait_indefinitely());
        self.inflight.clear();
    }

    /// Map `staging[..bytes]`, hand it to `f`, unmap.
    fn read_staging(&mut self, bytes: u64, f: impl FnOnce(&[u8])) {
        let slice = self.staging.slice(0..bytes);
        slice.map_async(wgpu::MapMode::Read, |r| {
            if let Err(e) = r {
                eprintln!("[gs] readback map failed: {e}");
            }
        });
        let _ = self.ctx.device.poll(wgpu::PollType::wait_indefinitely());
        self.inflight.clear();
        {
            let view = slice.get_mapped_range().expect("readback map");
            f(&view);
        }
        self.staging.unmap();
    }

    /// Bring the mirror up to date for the GPU-drawn pages among `pages`
    /// (plus whatever the open batch draws there).
    pub(crate) fn readback(&mut self, pages: &Pages, vram: &mut [u32], why: &str) {
        self.close(vram);
        let want = pages_and(pages, &self.gpu_dirty);
        if want.is_empty() {
            return;
        }
        // Read back every GPU-drawn page, not just the ones asked for: games
        // read several small rectangles in a row (SSX 3 reads Z around each
        // light for its flares), and each readback waits for the GPU.
        let all = self.gpu_dirty;
        if trace_readback() {
            eprintln!("[gs-gpu] readback ({why}): pages {:?}, all dirty {:?}", page_span(&want), page_span(&all));
        }
        let Some((_, hi)) = page_span(&all) else { return };
        let mut runs = Vec::new();
        let mut pg = 0;
        while pg < hi {
            if !page_set(&all, pg) {
                pg += 1;
                continue;
            }
            let start = pg;
            while pg < hi && page_set(&all, pg) {
                pg += 1;
            }
            runs.push((start, pg));
        }
        let bytes_per_page = PAGE_WORDS as u64 * 4;
        self.submit_with(|s, enc| {
            for &(a, b) in &runs {
                let off = a as u64 * bytes_per_page;
                enc.copy_buffer_to_buffer(&s.vram, off, &s.staging, off, (b - a) as u64 * bytes_per_page);
            }
        });
        self.read_staging(hi as u64 * bytes_per_page, |bytes| {
            for &(a, b) in &runs {
                let (wa, wb) = (a as usize * PAGE_WORDS, b as usize * PAGE_WORDS);
                for (d, c) in vram[wa..wb].iter_mut().zip(bytes[wa * 4..wb * 4].chunks_exact(4)) {
                    *d = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                }
            }
        });
        for &(a, b) in &runs {
            self.stats.readback_pages += (b - a) as u64;
        }
        self.gpu_dirty = Pages::default();
        self.stats.readbacks += 1;
    }

    // ------------------------------------------------------------ display

    /// Queue the display circuits into the output buffer; `None` when the
    /// circuit setup has nothing to show (the caller produces black).
    pub(crate) fn queue_display(&mut self, d: &DisplaySetup, vram: &[u32]) {
        self.close(vram);
        self.ensure_out(d.w as u64 * d.h as u64);
        let at = self.arena.len() as u32;
        self.arena.extend_from_slice(&d.circuits);
        let slot = self.slots.len() as u32;
        let mut s = [0u32; 16];
        s[..7].copy_from_slice(&[d.w, d.h, d.row_div, d.flags, d.alp, d.bg, at]);
        self.slots.push(s);
        self.cmds.push(Cmd::Display {
            slot,
            w: d.w,
            h: d.h,
        });
    }

    /// Show the display on the GPU: returns the output buffer.
    pub(crate) fn display_gpu(&mut self, d: &DisplaySetup, vram: &[u32]) -> GpuFrame {
        self.queue_display(d, vram);
        self.submit_with(|_, _| {});
        GpuFrame {
            buffer: self.out.clone(),
            width: d.w,
            height: d.h,
        }
    }

    /// Show the display and read the image back.
    pub(crate) fn display_read(&mut self, d: &DisplaySetup, vram: &[u32], out: &mut Vec<u32>) {
        self.queue_display(d, vram);
        let len = d.w as u64 * d.h as u64 * 4;
        if len > self.staging.size() {
            self.staging = self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gs-readback"),
                size: len.next_power_of_two(),
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        self.submit_with(|s, enc| enc.copy_buffer_to_buffer(&s.out, 0, &s.staging, 0, len));
        out.clear();
        self.read_staging(len, |bytes| {
            out.extend(bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])));
        });
    }
}

/// Display circuit parameters for the GPU (see `display.rs`).
pub(crate) struct DisplaySetup {
    pub w: u32,
    pub h: u32,
    pub row_div: u32,
    /// bit 0 merge, bit 1 SLBG, bit 2 MMOD.
    pub flags: u32,
    pub alp: u32,
    pub bg: u32,
    /// Primary circuit then circuit 2: [surf(3), is16, dbx, dby, w, h] each.
    pub circuits: [u32; 16],
}

pub(crate) fn circuit_words(s: &Surface, is16: bool, dbx: u32, dby: u32, w: u32, h: u32) -> [u32; 8] {
    let sw = surf_words(s);
    [sw[0], sw[1], sw[2], is16 as u32, dbx, dby, w, h]
}

#[cfg(test)]
mod tests;
