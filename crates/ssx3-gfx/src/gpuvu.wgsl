// VU1 calls drawn by the GPU (`ssx3_render::gpu`, plan step N1): one vertex
// entry per program, pulling the call's inputs. Appended to draw.wgsl (the
// `Batch` uniforms, `VOut` and the GS fragment stage are shared).

struct Call {
    q: array<vec4<u32>, 32>,
}

@group(1) @binding(0) var<storage, read> calls: array<Call>;
@group(1) @binding(1) var<storage, read> data: array<vec4<u32>>;
// Per vertex of a draw (a run of calls of one program merged into one
// draw): the call's vertex index and the call (`vs_<program>` wrappers).
@group(1) @binding(2) var<storage, read> gidx: array<u32>;
@group(1) @binding(3) var<storage, read> gcall: array<u32>;

fn fq(c: u32, i: u32) -> vec4<f32> {
    return bitcast<vec4<f32>>(calls[c].q[i]);
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
    let xyzf = calls[c].q[30].w == 4u;
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
