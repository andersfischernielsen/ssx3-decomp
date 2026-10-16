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
    // through TEXA), 6 DATE (destination alpha test), 7 DATM.
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
}

@group(0) @binding(0) var<uniform> b: Batch;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;
@group(0) @binding(3) var dst: texture_2d<f32>;

struct VIn {
    @location(0) pos: vec3<f32>,
    @location(1) fog: f32,
    @location(2) col: vec4<f32>,
    @location(3) stq: vec4<f32>,
}

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec4<f32>,
    @location(1) stq: vec3<f32>,
    @location(2) fog: f32,
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

@fragment
fn fs(v: VOut) -> @location(0) vec4<f32> {
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
        let ts = textureSample(tex, samp, v.stq.xy / v.stq.z);
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
    }
    var out = rgb / 255.0;
    if (b.src_mode == 1u) {
        out = out * abs(b.cs_a * c + b.cs_b);
    } else if (b.src_mode == 2u) {
        out = vec3<f32>(c);
    }
    var oa = a;
    if ((b.flags & 8u) != 0u) {
        oa = max(oa, 128.0);
    }
    return vec4<f32>(out, oa / 128.0);
}
