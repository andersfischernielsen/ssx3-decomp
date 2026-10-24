//! VU1 calls drawn by the GPU (`docs/plans/native-rendering.md`, step N1).
//!
//! For a call whose program has a GPU port, [`plan`] does the program's
//! cheap, order-dependent work on the VU1 memory image (setup matrices,
//! lighting coefficients, in-place texgen, flags: what later calls read) and
//! returns a [`GpuCall`]: the vertex inputs and per-call uniforms a vertex
//! shader (`ssx3-gfx`, `vs_<program>`) turns into the same vertices the
//! program would kick, the triangle list over them, and the GIF tag the
//! program would kick first (its PRIM). The per-vertex work (transform,
//! perspective divide, lighting) is the shader's; the guard-band cull and
//! the clipper are the GPU's clipping.
//!
//! GPU floats are IEEE, not PS2: the vertices are close to the program's,
//! not bit-exact (the CPU programs stay the reference, `SSX3_GPU_VU=0`).

/// The vertex programs (`vs_*` entries of `ssx3-gfx/src/gpuvu.wgsl`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Program {
    /// Type 3 meshes (`mat3`, entries 0x2270 / 0x22c8 / 0x2320 / 0x2358).
    Mat3Mesh = 0,
    /// Type 3 float strips and ribbons (`mat3`, entries 0x12b8 / 0x3a08).
    Mat3Strip = 3,
    /// Types 6/7 skinned strips and their env pass (`mat6`, entries 0x00 /
    /// 0x50 / 0x30).
    Mat6Skin = 1,
    /// Type 7 shadow silhouettes (`mat6`, entry 0x20).
    Mat6Shadow = 2,
    /// Type 4 emitters and ring emitters (`mat4`, entries 0x0000 / 0x0510 /
    /// 0x0a00 / 0x0f10).
    Mat4Particles = 40,
    /// Type 4 patch glitter (`mat4`, entry 0x1460).
    Mat4Glitter = 41,
    /// Type 5 SnowBox snow (`mat5`).
    Mat5Snow = 50,
    /// Types 1 and 2 Bezier terrain strips (`bezier::gpu`).
    Bezier = 10,
}

impl Program {
    /// Its vertex entry in `ssx3-gfx` (`src/gpuvu/<program>.wgsl`).
    pub fn vs_entry(self) -> &'static str {
        match self {
            Program::Mat3Mesh => "vs_mat3_mesh",
            Program::Mat3Strip => "vs_mat3_strip",
            Program::Mat6Skin => "vs_mat6_skin",
            Program::Mat6Shadow => "vs_mat6_shadow",
            Program::Mat4Particles => "vs_mat4_particles",
            Program::Mat4Glitter => "vs_mat4_glitter",
            Program::Mat5Snow => "vs_mat5_snow",
            Program::Bezier => "vs_bezier",
        }
    }

    pub fn from_u8(v: u8) -> Option<Program> {
        match v {
            0 => Some(Program::Mat3Mesh),
            3 => Some(Program::Mat3Strip),
            1 => Some(Program::Mat6Skin),
            2 => Some(Program::Mat6Shadow),
            40 => Some(Program::Mat4Particles),
            41 => Some(Program::Mat4Glitter),
            50 => Some(Program::Mat5Snow),
            10 => Some(Program::Bezier),
            _ => None,
        }
    }
}

/// Per-call uniform qwords (`Call` in the shader).
pub const UNIFORM_QWORDS: usize = 32;
/// Uniform qwords the GS side fills in (`Gs::gpu_draw`): the batch's
/// coordinate offset, depth scale, texture scale, fog.
pub const GS_PARAMS: usize = 28;

/// The uniform qword whose x a split call's static data index goes to
/// (`GpuCall::split`).
pub const SPLIT_BASE: usize = 1;

/// One draw of a call computed by the GPU (a call may make several).
#[derive(Clone, Debug, PartialEq)]
pub struct GpuCall {
    pub program: Program,
    /// GIF data the program kicks before this draw's vertices (register
    /// setup, other primitives), processed by the GS side first.
    pub pre: Vec<u128>,
    /// The GIF tag the program kicks (PRE / PRIM select the primitive).
    pub tag: u128,
    /// Per-call uniforms; qwords from [`GS_PARAMS`] on are the GS side's.
    pub uniforms: [[u32; 4]; UNIFORM_QWORDS],
    /// Vertex inputs (program-specific layout, qwords).
    pub data: Vec<[u32; 4]>,
    /// Triangle list over the call's vertices (0-based); with an
    /// [`index_mode`](Self::index_mode), the triangles it expands.
    pub indices: Vec<u32>,
    /// 0: `indices` as they are. Otherwise `per << 8 | shift`: each entry
    /// `t` stands for the `per` indices `t << shift | corner`, corner =
    /// 0..per (the shader expands them; [`GpuCall::expanded_indices`]).
    pub index_mode: u32,
    /// The largest vertex alpha (GS units: 0x80 = 1.0).
    pub alpha_max: f32,
    /// Calls that redraw an earlier call's vertices (type 3 pass 2 redraws
    /// pass 1's output with new texture coordinates).
    pub link: Link,
    /// `Some(s)`: `data[s..]` is the static part (vertex records) and
    /// `data[..s]` changes every frame; the GS side writes the static
    /// part's absolute data index into uniform [`SPLIT_BASE`]`.x`. With a
    /// `mesh_key`, only the static part is kept by the GPU.
    pub split: Option<u32>,
    /// A key under which `data` is the same every time (the GPU keeps it):
    /// the planner sets `Some(0)` when the data is the input's as it came
    /// (no in-place rewrite), and [`crate::vujob::run_call`] the input's
    /// source key (`VuCall::src`), or `None`.
    pub mesh_key: Option<u64>,
}

impl GpuCall {
    /// The number of vertices the call draws (its indices, expanded).
    pub fn draw_len(&self) -> usize {
        match self.index_mode {
            0 => self.indices.len(),
            m => self.indices.len() * (m >> 8) as usize,
        }
    }

    /// The indices with the index mode applied.
    pub fn expanded_indices(&self) -> std::borrow::Cow<'_, [u32]> {
        match self.index_mode {
            0 => std::borrow::Cow::Borrowed(&self.indices),
            m => std::borrow::Cow::Owned(
                self.indices
                    .iter()
                    .flat_map(|&t| (0..m >> 8).map(move |c| t << (m & 0xff) | c))
                    .collect(),
            ),
        }
    }
}

/// The index mode of triangle lists whose entries expand to `per` indices
/// `t << shift | corner`.
pub const fn tri_mode(per: u32, shift: u32) -> u32 {
    per << 8 | shift
}

/// How a call's vertices relate to another call's (keyed by the VU address
/// of the output buffer both use).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    None,
    /// Later calls may reuse this call's vertex data.
    Source(u32),
    /// Positions and colours from the latest `Source` with this key.
    Reuse(u32),
}

impl Link {
    fn code(self) -> (u32, u32) {
        match self {
            Link::None => (0, 0),
            Link::Source(k) => (1, k),
            Link::Reuse(k) => (2, k),
        }
    }
    fn from_code(t: u32, k: u32) -> Link {
        match t {
            1 => Link::Source(k),
            2 => Link::Reuse(k),
            _ => Link::None,
        }
    }
}

/// Whether calls go to the GPU (`SSX3_GPU_VU=0`: the CPU programs).
pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("SSX3_GPU_VU").map_or(true, |v| v != "0"))
}

/// `SSX3_GPU_VU_SKIP` key of the type 3 strips (entries 0x12b8 / 0x3a08).
const STRIPS: u64 = 1;

/// Debug: `SSX3_GPU_VU_SKIP=mat1,mat3,mat3strip,...` keeps those programs
/// (or the type 3 strips) on the CPU.
fn skipped(hash: u64) -> bool {
    static SKIP: std::sync::OnceLock<Vec<u64>> = std::sync::OnceLock::new();
    SKIP.get_or_init(|| {
        let v = std::env::var("SSX3_GPU_VU_SKIP").unwrap_or_default();
        v.split(',')
            .filter_map(|n| match n.trim() {
                "mat1" => Some(crate::mat1::HASH),
                "mat2" => Some(crate::mat2::HASH),
                "mat3" => Some(crate::mat3::HASH),
                "mat4" => Some(crate::mat4::HASH),
                "mat5" => Some(crate::mat5::HASH),
                "mat6" => Some(crate::mat6::HASH),
                "mat3strip" => Some(STRIPS),
                _ => None,
            })
            .collect()
    })
    .contains(&hash)
}

/// The key of a draw's mesh data (`GpuCall::mesh_key`): the source of the
/// call's input (`VuCall::src`) and the program.
pub fn mesh_key(src: u64, program: Program) -> u64 {
    src.wrapping_mul(0x9e37_79b9).wrapping_add(program as u64)
}

/// Plan the call for the GPU when its program and variant have a GPU port
/// (and do its memory side effects): its draws in kick order. `None`
/// leaves the memory untouched, for the CPU program.
pub fn plan(
    hash: u64,
    entry: u32,
    top: u32,
    itop: u32,
    kept: Option<f32>,
    mem: &mut [u8; 16384],
) -> Option<Vec<GpuCall>> {
    let r = plan_inner(hash, entry, top, itop, kept, mem);
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *ON.get_or_init(|| std::env::var_os("SSX3_GPU_STATS").is_some()) {
        *STATS
            .lock()
            .unwrap()
            .entry((hash, entry, itop != 0, r.is_some()))
            .or_default() += 1;
    }
    r
}

/// Debug (`SSX3_GPU_STATS`): calls per (program, entry, ITOP set, on the GPU).
pub static STATS: std::sync::Mutex<std::collections::BTreeMap<(u64, u32, bool, bool), u64>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());

fn plan_inner(
    hash: u64,
    entry: u32,
    top: u32,
    itop: u32,
    kept: Option<f32>,
    mem: &mut [u8; 16384],
) -> Option<Vec<GpuCall>> {
    if skipped(hash) {
        return None;
    }
    match hash {
        crate::mat3::HASH => match entry {
            0x12b8 | 0x3a08 if !skipped(STRIPS) => {
                crate::mat3::plan_gpu_strip(entry, top, mem).map(|c| vec![c])
            }
            0x12b8 | 0x3a08 => None,
            _ => crate::mat3::plan_gpu(entry, top, itop, kept, mem).map(|c| vec![c]),
        },
        crate::mat6::HASH => crate::mat6::plan_gpu(entry, top, itop, kept.is_some(), mem),
        crate::mat4::HASH => crate::mat4::plan_gpu(entry, top, itop, mem),
        crate::mat5::HASH => crate::mat5::plan_gpu(entry, top, itop, mem),
        crate::mat1::HASH => crate::mat1::plan_gpu(entry, top, itop, mem),
        crate::mat2::HASH => crate::mat2::plan_gpu(entry, top, itop, mem),
        _ => None,
    }
}

#[cfg(target_endian = "big")]
compile_error!("the VU job encoding copies u32 data as host bytes: little-endian targets only");

/// Words (u32 or [u32; 4]: every bit pattern is a value) stored as
/// little-endian bytes.
trait Words: Copy {}
impl Words for u32 {}
impl Words for [u32; 4] {}

/// `n` items from the bytes `b` (a plain copy: the targets are
/// little-endian).
fn words<T: Words>(b: &[u8], n: usize) -> Vec<T> {
    let bytes = n * std::mem::size_of::<T>();
    let b = &b[..bytes];
    let mut v: Vec<T> = Vec::with_capacity(n);
    // SAFETY: `b` holds `bytes` bytes, the capacity is `n` items of size
    // `bytes / n`, and any bytes are a valid `T` (`Words`).
    unsafe {
        std::ptr::copy_nonoverlapping(b.as_ptr(), v.as_mut_ptr() as *mut u8, bytes);
        v.set_len(n);
    }
    v
}

/// Append `w` as little-endian bytes.
fn put_words<T: Words>(out: &mut Vec<u8>, w: &[T]) {
    // SAFETY: any initialized `T` (`Words`: no padding) is valid bytes.
    out.extend_from_slice(unsafe {
        std::slice::from_raw_parts(w.as_ptr() as *const u8, std::mem::size_of_val(w))
    });
}

/// Encode for the browser's VU helper workers (`vujob`).
pub fn encode(c: &GpuCall, w: &mut Vec<u8>) {
    w.push(c.program as u8);
    w.extend_from_slice(&(c.pre.len() as u32).to_le_bytes());
    for q in &c.pre {
        w.extend_from_slice(&q.to_le_bytes());
    }
    w.extend_from_slice(&c.tag.to_le_bytes());
    put_words(w, &c.uniforms[..]);
    w.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
    put_words(w, &c.data);
    w.extend_from_slice(&(c.indices.len() as u32).to_le_bytes());
    put_words(w, &c.indices);
    w.extend_from_slice(&c.index_mode.to_le_bytes());
    w.extend_from_slice(&c.split.unwrap_or(u32::MAX).to_le_bytes());
    w.extend_from_slice(&c.alpha_max.to_le_bytes());
    let (t, k) = c.link.code();
    w.extend_from_slice(&t.to_le_bytes());
    w.extend_from_slice(&k.to_le_bytes());
    w.extend_from_slice(&c.mesh_key.map_or(u64::MAX, |k| k).to_le_bytes());
}

/// Decode what [`encode`] wrote; returns the call and the bytes used.
pub fn decode(b: &[u8]) -> (GpuCall, usize) {
    let mut p = 0;
    let mut take = |n: usize| {
        let s = &b[p..p + n];
        p += n;
        s
    };
    let u32_at = |s: &[u8]| u32::from_le_bytes(s.try_into().unwrap());
    let program = Program::from_u8(take(1)[0]).expect("gpu program");
    let n = u32_at(take(4)) as usize;
    let pre = (0..n)
        .map(|_| u128::from_le_bytes(take(16).try_into().unwrap()))
        .collect();
    let tag = u128::from_le_bytes(take(16).try_into().unwrap());
    let mut uniforms = [[0u32; 4]; UNIFORM_QWORDS];
    let u: Vec<[u32; 4]> = words(take(16 * UNIFORM_QWORDS), UNIFORM_QWORDS);
    uniforms.copy_from_slice(&u);
    let n = u32_at(take(4)) as usize;
    let data = words(take(16 * n), n);
    let n = u32_at(take(4)) as usize;
    let indices = words(take(4 * n), n);
    let index_mode = u32_at(take(4));
    let split = Some(u32_at(take(4))).filter(|&s| s != u32::MAX);
    let alpha_max = f32::from_le_bytes(take(4).try_into().unwrap());
    let t = u32_at(take(4));
    let link = Link::from_code(t, u32_at(take(4)));
    let mk = u64::from_le_bytes(take(8).try_into().unwrap());
    let mesh_key = (mk != u64::MAX).then_some(mk);
    (
        GpuCall {
            program,
            pre,
            tag,
            uniforms,
            data,
            indices,
            index_mode,
            alpha_max,
            link,
            split,
            mesh_key,
        },
        p,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_round_trip() {
        let mut uniforms = [[0u32; 4]; UNIFORM_QWORDS];
        uniforms[3] = [1, 2, 3, 4];
        let c = GpuCall {
            program: Program::Mat3Mesh,
            pre: vec![7, 8],
            tag: 0x1234_5678_9abc_def0_0fed_cba9_8765_4321,
            uniforms,
            data: vec![[5, 6, 7, 8], [9, 10, 11, 12]],
            indices: vec![0, 1, 2, 1, 2, 3],
            index_mode: tri_mode(21, 5),
            alpha_max: 128.0,
            link: Link::Reuse(0x1db),
            split: Some(1),
            mesh_key: Some(77),
        };
        let mut w = Vec::new();
        encode(&c, &mut w);
        w.push(0xee);
        let (d, used) = decode(&w);
        assert_eq!(d, c);
        assert_eq!(used, w.len() - 1);
    }
}

/// Debug (`SSX3_GPU_CHECK=1`): the shader's formulas for a planned call,
/// evaluated on the CPU (IEEE), against the program's own output for the
/// same call (`gif`, run on a copy of the memory before planning). Returns
/// a description of the first vertices that differ beyond rounding (each
/// program adds its arm; none: not checked).
pub fn check(hash: u64, calls: &[GpuCall], gif: &[u128]) -> Option<String> {
    match (hash, calls) {
        (crate::mat3::HASH, [call]) if call.program == Program::Mat3Mesh => check_mat3(call, gif),
        (crate::mat3::HASH, [call]) if call.program == Program::Mat3Strip => {
            crate::mat3::check_gpu_strip(call, gif)
        }
        (crate::mat6::HASH, [call]) => crate::mat6::check_gpu(call, gif),
        (crate::mat4::HASH | crate::mat5::HASH, _) => crate::mat4::gpu_check(calls, gif),
        (crate::mat1::HASH | crate::mat2::HASH, _) => crate::bezier::gpu::check(calls, gif),
        _ => None,
    }
}

fn check_mat3(call: &GpuCall, gif: &[u128]) -> Option<String> {
    // (Pass 2 redraws pass 1's vertices: nothing of its own to compare.)
    if matches!(call.link, Link::Reuse(_)) {
        return None;
    }
    let f = |q: usize| -> [f32; 4] { call.uniforms[q].map(f32::from_bits) };
    let lit = call.uniforms[4][0] != 0;
    // The program's vertices: the last packet's {STQ, RGBA, XYZ2} triples
    // (clipped fans come first; skip to the strip packet).
    let n = call.data.len() / 4;
    let want: Vec<[u32; 12]> = gif[gif.len().saturating_sub(3 * n)..]
        .chunks(3)
        .map(|v| std::array::from_fn(|i| (v[i / 4] >> (32 * (i % 4))) as u32))
        .collect();
    let mut bad = Vec::new();
    for (i, w) in want.iter().enumerate().take(n) {
        let xyz = call.data[4 * i];
        let uv = call.data[4 * i + 1];
        let cw = call.data[4 * i + 2];
        let p = [
            xyz[0] as i32 as f32 / 32768.0,
            xyz[1] as i32 as f32 / 32768.0,
            xyz[2] as i32 as f32 / 32768.0,
        ];
        let s = [f(0), f(1), f(2), f(3)];
        let hc: [f32; 4] =
            std::array::from_fn(|l| s[0][l] * p[0] + s[1][l] * p[1] + s[2][l] * p[2] + s[3][l]);
        let q = 1.0 / hc[3];
        let t = [uv[0] as i32 as f32 / 4096.0, uv[1] as i32 as f32 / 4096.0];
        let stq = [t[0] * q, t[1] * q, q];
        let col: [f32; 4] = if lit {
            let n = [
                cw[0] as i32 as f32 / 32768.0,
                cw[1] as i32 as f32 / 32768.0,
                cw[2] as i32 as f32 / 32768.0,
            ];
            let d: [f32; 4] =
                std::array::from_fn(|l| f(5)[l] * n[0] + f(6)[l] * n[1] + f(7)[l] * n[2]);
            let terms = [
                1.0,
                d[0] * d[0],
                d[1] * d[1],
                d[2] * d[2],
                d[0] * d[1],
                d[2] * d[0],
                d[1] * d[2],
                d[0],
                d[1],
                d[2],
            ];
            std::array::from_fn(|l| {
                (0..10)
                    .map(|k| f(8 + k)[l] * terms[k])
                    .sum::<f32>()
                    .clamp(0.0, 255.0)
                    .floor()
            })
        } else {
            std::array::from_fn(|l| cw[l] as i32 as f32)
        };
        let sx = [hc[0] * q * 16.0, hc[1] * q * 16.0, hc[2] * q * 16.0];
        let got_xy = [w[8] as i32 as f32, w[9] as i32 as f32, w[10] as i32 as f32];
        let got_stq = [
            f32::from_bits(w[0]),
            f32::from_bits(w[1]),
            f32::from_bits(w[2]),
        ];
        let got_col = [w[4] as f32, w[5] as f32, w[6] as f32, w[7] as f32];
        let close = |a: f32, b: f32, tol: f32| (a - b).abs() <= tol * (1.0 + b.abs());
        let ok_xy = (0..3).all(|l| (sx[l] - got_xy[l]).abs() <= 2.0 + 1e-4 * got_xy[l].abs());
        let ok_stq = (0..3).all(|l| close(stq[l], got_stq[l], 1e-3));
        let ok_col = (0..4).all(|l| (col[l] - got_col[l]).abs() <= 2.0);
        if !(ok_xy && ok_stq && ok_col) && bad.len() < 3 {
            bad.push(format!(
                "v{i}: xy {:?} want {:?} | stq {:?} want {:?} | rgba {:?} want {:?}",
                sx, got_xy, stq, got_stq, col, got_col
            ));
        }
    }
    (!bad.is_empty()).then(|| {
        format!(
            "mat3 mesh ({} vertices, lit {lit}, pass {}): {}",
            n,
            call.uniforms[4][2],
            bad.join("; ")
        )
    })
}
