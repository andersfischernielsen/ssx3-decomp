// VU1 calls drawn by the GPU (`ssx3_render::gpu`, plan step N1): one vertex
// entry per program, pulling the call's inputs. Appended to draw.wgsl (the
// `Batch` uniforms, `VOut` and the GS fragment stage are shared).

// Per call its uniforms, compacted (see `uq`).
@group(1) @binding(0) var<storage, read> calls: array<vec4<u32>>;
@group(1) @binding(1) var<storage, read> data: array<vec4<u32>>;
// The calls' triangle lists (`ssx3_render::gpu::GpuCall::indices`);
// gstart[0] = the number of calls, then per call {its first vertex in the
// draw, its first entry in gidx, its index mode, its first uniform qword in
// `calls`, its program uniform count}. `vs_<program>` wrappers
// find a vertex's call (the last one starting at or before it) and the
// call's vertex index.
@group(1) @binding(2) var<storage, read> gidx: array<u32>;
@group(1) @binding(3) var<storage, read> gstart: array<u32>;

fn call_of(k: u32) -> u32 {
    var lo = 0u;
    var hi = gstart[0];
    while (hi - lo > 1u) {
        let mid = (lo + hi) / 2u;
        if (gstart[1u + 5u * mid] <= k) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    return lo;
}

// Vertex `k` of the draw as call `c`'s vertex index: its triangle list
// entry, or (index mode `per << 8 | shift`) entry `t` expanded to
// `t << shift | corner`.
fn call_index(k: u32, c: u32) -> u32 {
    let local = k - gstart[1u + 5u * c];
    let base = gstart[2u + 5u * c];
    let mode = gstart[3u + 5u * c];
    if (mode == 0u) {
        return gidx[base + local];
    }
    let per = mode >> 8u;
    return (gidx[base + local / per] << (mode & 255u)) | (local % per);
}
// Uniform qword `i` of call `c` (`ssx3_render::gpu::GpuCall::uniforms`):
// stored as the 4 GS-side qwords (from 28 on), then the program's up to
// the last non-zero one (later ones read as 0).
fn uq(c: u32, i: u32) -> vec4<u32> {
    let base = gstart[4u + 5u * c];
    if (i >= 28u) {
        return calls[base + i - 28u];
    }
    if (i >= gstart[5u + 5u * c]) {
        return vec4<u32>(0u);
    }
    return calls[base + 4u + i];
}

// Vertex data kept from frame to frame (gs::MeshArena).
@group(1) @binding(4) var<storage, read> arena: array<vec4<u32>>;

// A call's data qword: from the arena when the index has gs::ARENA_BIT.
fn rd(i: u32) -> vec4<u32> {
    if ((i & 0x80000000u) != 0u) {
        return arena[i & 0x7fffffffu];
    }
    return data[i];
}

fn fq(c: u32, i: u32) -> vec4<f32> {
    return bitcast<vec4<f32>>(uq(c, i));
}

// The vertex as the GS would draw it, from the program's homogeneous
// screen position `hc` (screen = hc.xyz / hc.w, x and y in GS pixels plus
// the guard offset, z in GS depth / 16) and its clip-space z `cz` (the
// program's CLIP test: |z| <= w). GS-side parameters (gs::gpu_params):
// q28 = (XYOFFSET / 16, 1 / (ZMAX + 1), ZMAX), q29 = (texture scale, HUD
// squeeze, its centre), q30 = (FOG, data base). The position is linear in
// `hc` and `cz`, so the GPU clips what the program's guard band, clipper
// and z planes would; the screen z goes to `fs_gpu` (`zw`), which applies
// the GS depth rules per pixel; the attributes are screen-linear like the
// GS's. (A caller that changes `pos.w` must keep `zw.y` equal to it.)
// `adc`: the vertex has ADC (its XYZF2 fog reads 0xff).
fn gs_vertex(c: u32, hc: vec4<f32>, cz: f32, adc: bool, col: vec4<f32>, stq: vec3<f32>) -> VOut {
    let off = fq(c, 28u);
    let tx = fq(c, 29u);
    let w = hc.w;
    let px = tx.z * (hc.x - off.x * w) + tx.w * (1.0 - tx.z) * w;
    let py = hc.y - off.y * w;
    // XYZF2: the fog from the w lane (1.0 there: FTOI4 → 16 → 1).
    let xyzf = uq(c, 30).w == 4u;
    var fog = fq(c, 30u).x;
    if (xyzf) {
        fog = select(1.0, 255.0, adc);
    }
    var o: VOut;
    o.pos = vec4<f32>((px + 0.5 * w) / b.size.x * 2.0 - w, w - (py + 0.5 * w) / b.size.y * 2.0, (cz + w) * 0.5, w);
    o.zw = vec2<f32>(hc.z, w);
    o.zm = vec2<f32>(select(0.0, 1.0, xyzf), off.w);
    o.col = col;
    o.stq = vec3<f32>(stq.x * tx.x, stq.y * tx.y, stq.z);
    o.fog = fog;
    return o;
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

// The GS fragment stage with the GS depth (the position's z is the clip z).
@fragment
fn fs_gpu(v: VOut) -> FsOut {
    // The program's FTOI4 depth (x16): XYZ2 takes it as is, XYZF2 / 16 (24
    // bits); negative values wrap (XYZ2: a huge Z; XYZF2: & 0xffffff).
    let sz = v.zw.x / v.zw.y;
    let zmax = v.zm.y;
    var z: f32;
    if (v.zm.x != 0.0) {
        z = select(floor(sz), 16777216.0 + floor(sz), sz < 0.0);
    } else {
        z = select(sz * 16.0, zmax, sz < 0.0);
    }
    let d = min(z, zmax) / (zmax + 1.0);
    var o: FsOut;
    o.color = shade(v);
    o.depth = d;
    return o;
}
