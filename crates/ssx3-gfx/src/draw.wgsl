// ssx3-gfx: GS-style shading on hardware rasterization.
//
// Colours are in GS units (0..255; 0x80 = 1.0 for alpha and modulation).
// The colour target stores RGB/255 and alpha/128 (so DstAlpha is the GS's
// Ad as a blend factor).

struct Batch {
    // Target size in pixels.
    size: vec2<f32>,
    // TFX (0 modulate, 1 decal, 2 highlight, 3 highlight2).
    tfx: u32,
    // Bit 0 TME, 1 TCC, 2 FGE, 3 FBA, 4 the texture is a target we drew
    // (alpha stored as a/128), 5 that target is CT16 (1-bit alpha read
    // through TEXA), 6 DATE (destination alpha test), 7 DATM, 8 depth fog
    // (the texel is the CLUT entry for depth bits 8..15), 9 half strength
    // (C = As halved; a second draw doubles the result).
    flags: u32,
    // Alpha test: ATST, AREF, side (0 none, 1 draw passing, 2 draw failing).
    atst: u32,
    aref: u32,
    side: u32,
    // Source output: 0 plain, 1 premultiplied by (cs_a * C + cs_b), 2 C.
    src_mode: u32,
    cs_a: f32,
    cs_b: f32,
    // C: 0 = As, 2 = FIX (fix / 128).
    c_kind: u32,
    fix: f32,
    fogcol: vec4<f32>,
    // TEXA TA0, TA1.
    ta: vec2<f32>,
    // Depth fog: ZMAX + 1 (depth is stored as Z / (ZMAX + 1)).
    zscale: f32,
    // Texture LOD bias (options: texture sharpness).
    lod_bias: f32,
    // Seconds (animated effects: flag 2048, snow glints).
    time: f32,
    pad0: f32,
    pad1: f32,
    pad2: f32,
}

@group(0) @binding(0) var<uniform> b: Batch;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;
@group(0) @binding(3) var dst: texture_2d<f32>;
@group(0) @binding(4) var zsnap: texture_depth_2d;

struct VIn {
    @location(0) pos: vec3<f32>,
    @location(1) fog: f32,
    @location(2) col: vec4<f32>,
    @location(3) stq: vec4<f32>,
}

// Attributes interpolate linearly in screen space, as on the GS (the CPU
// vertices have w = 1; GPU-computed ones, gpuvu.wgsl, a real w).
struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(linear) col: vec4<f32>,
    @location(1) @interpolate(linear) stq: vec3<f32>,
    @location(2) @interpolate(linear) fog: f32,
    // GPU-computed vertices (gpuvu.wgsl `fs_gpu`): the screen z times the
    // position's w, and w, perspective-correct: their ratio is the GS's
    // screen-linear z inside a triangle and right where the GPU clips; and
    // (XYZF2, ZMAX) for the GS depth rules.
    @location(3) @interpolate(perspective) zw: vec2<f32>,
    @location(4) @interpolate(flat) zm: vec2<f32>,
    // Sun shadows (gpuvu.wgsl): the world position (relative to a point
    // near the camera), w = 1; w = 0 without.
    @location(5) @interpolate(perspective) wp: vec4<f32>,
}

@vertex
fn vs(v: VIn) -> VOut {
    var o: VOut;
    // GS samples pixels at integer coordinates, the GPU at their centres.
    let p = (v.pos.xy + vec2<f32>(0.5)) / b.size;
    o.pos = vec4<f32>(p.x * 2.0 - 1.0, 1.0 - p.y * 2.0, v.pos.z, 1.0);
    o.col = v.col;
    o.stq = v.stq.xyz;
    o.fog = v.fog;
    o.zw = vec2<f32>(0.0, 1.0);
    o.zm = vec2<f32>(0.0);
    o.wp = vec4<f32>(0.0);
    return o;
}

fn alpha_ok(a: f32) -> bool {
    let ai = i32(a);
    let r = i32(b.aref);
    switch b.atst {
        case 0u: { return false; }
        case 1u: { return true; }
        case 2u: { return ai < r; }
        case 3u: { return ai <= r; }
        case 4u: { return ai == r; }
        case 5u: { return ai >= r; }
        case 6u: { return ai > r; }
        default: { return ai != r; }
    }
}

fn hash2(p: vec2<f32>) -> f32 {
    let q = fract(p * vec2<f32>(0.1031, 0.1030));
    let r = q + dot(q, q.yx + 33.33);
    return fract((r.x + r.y) * r.x);
}

// Snow glints: sparse texture-space cells on bright, unsaturated texels
// flash as the view and time change.
fn sparkle(rgb: vec3<f32>, v: VOut) -> vec3<f32> {
    let lum = (rgb.r + rgb.g + rgb.b) / 765.0;
    let sat = max(rgb.r, max(rgb.g, rgb.b)) - min(rgb.r, min(rgb.g, rgb.b));
    if (lum < 0.7 || sat > 40.0) {
        return rgb;
    }
    let uv = v.stq.xy / v.stq.z;
    let cell = floor(uv * 384.0);
    let h = hash2(cell);
    if (h < 0.97) {
        return rgb;
    }
    // Each glint flashes on its own phase; the screen position makes it
    // shift as the camera moves.
    let ph = h * 97.0 + b.time * 2.5 + (v.pos.x + v.pos.y) * 0.013;
    let flash = pow(max(sin(ph), 0.0), 16.0);
    return min(rgb + vec3<f32>(255.0) * flash * smoothstep(0.7, 0.9, lum), vec3<f32>(255.0));
}

@fragment
fn fs(v: VOut) -> @location(0) vec4<f32> {
    return shade(v);
}

// The GS fragment stage: texture function, alpha test, blending inputs.
fn shade(v: VOut) -> vec4<f32> {
    if ((b.flags & 64u) != 0u) {
        // Destination alpha bit 7 (stored as a/128, so a >= 128 is 1.0).
        let da = textureLoad(dst, vec2<i32>(v.pos.xy), 0).a;
        if ((da > 0.996) != ((b.flags & 128u) != 0u)) {
            discard;
        }
    }
    var rgb = floor(v.col.rgb);
    var a = floor(v.col.a);
    if ((b.flags & 1u) != 0u) {
        let uv = v.stq.xy / v.stq.z;
        var ts = textureSampleBias(tex, samp, uv, b.lod_bias);
        if ((b.flags & 256u) != 0u) {
            let z = u32(round(textureLoad(zsnap, vec2<i32>(v.pos.xy), 0) * b.zscale));
            ts = textureLoad(tex, vec2<i32>(i32((z >> 8u) & 255u), 0), 0);
        }
        var t = floor(ts * 255.0 + 0.5);
        if ((b.flags & 16u) != 0u) {
            t.a = floor(ts.a * 128.0 + 0.5);
            if ((b.flags & 32u) != 0u) {
                t.a = select(b.ta.x, b.ta.y, t.a >= 128.0);
            }
        }
        let tcc = (b.flags & 2u) != 0u;
        switch b.tfx {
            case 0u: {
                rgb = floor(t.rgb * rgb / 128.0);
                if (tcc) { a = floor(t.a * a / 128.0); }
            }
            case 1u: {
                rgb = t.rgb;
                if (tcc) { a = t.a; }
            }
            case 2u: {
                rgb = floor(t.rgb * rgb / 128.0) + a;
                if (tcc) { a = t.a + a; }
            }
            default: {
                rgb = floor(t.rgb * rgb / 128.0) + a;
                if (tcc) { a = t.a; }
            }
        }
        rgb = min(rgb, vec3<f32>(255.0));
        a = min(a, 255.0);
    }
    if ((b.flags & 2048u) != 0u) {
        rgb = sparkle(rgb, v);
    }
    if ((b.flags & 4u) != 0u) {
        let f = clamp(floor(v.fog), 0.0, 255.0);
        rgb = floor((f * rgb + (255.0 - f) * b.fogcol.rgb) / 255.0);
    }
    if (b.side != 0u) {
        let ok = alpha_ok(a);
        if ((b.side == 1u && !ok) || (b.side == 2u && ok)) {
            discard;
        }
    }
    var c = 1.0;
    if (b.c_kind == 2u) {
        c = b.fix;
    } else {
        c = a / 128.0;
        if ((b.flags & 512u) != 0u) {
            c = c * 0.5;
        }
    }
    var out = rgb / 255.0;
    if (b.src_mode == 1u) {
        out = out * abs(b.cs_a * c + b.cs_b);
    } else if (b.src_mode == 2u) {
        out = vec3<f32>(c);
    }
    var oa = a;
    if ((b.flags & 512u) != 0u) {
        oa = a * 0.5;
    }
    if ((b.flags & 8u) != 0u) {
        oa = max(oa, 128.0);
    }
    return vec4<f32>(out, oa / 128.0);
}
