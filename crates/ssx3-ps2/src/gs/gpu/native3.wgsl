
// ------------------------------------------------------------------ native type-3 triangles
//
// Phase 3 stage 2B (`gpu/native3.rs`): triangles of the type-3 VU1 program
// drawn with the hardware rasterizer inside a native run. Appended to
// gs.wgsl, whose texturing, TFX, fog and Z helpers it reuses: the draw state
// is the compute GS's setup record (`data[st + S_*]`), textures are read
// from the VRAM storage buffer exactly as the compute rasterizer reads them.
//
// params: a.x = first vertex record (words); c.xy = viewport size (a power
// of two, so window -> NDC is exact). A vertex record is N3_VWORDS words:
// [x, y] window pixels (+0.5: the GS samples pixel (x, y) at its corner, the
// hardware at the centre), RGBA8, S, T, Q, F, dz (f32: Z - Z of the
// triangle's first vertex), z0 (u32: that Z), setup record, flags
// (1: bilinear, 2: flat colour), then the triangle's Z plane exactly as the
// compute GS encodes it (origin, base, fraction, dZ/dx, dZ/dy): Z is
// evaluated with the same f32 arithmetic at the same pixel, so depth tests
// and Z writes match the emulated GS bit for bit.

const N3_VWORDS: u32 = 17u;

// params.d.x (f32, SSX3_N3_EPS, default 1e-4): added before truncating
// interpolated colour and fog to integers. The GS's f32 plane often lands
// exactly on integers (axis-aligned gradients); the hardware's barycentric
// interpolation then lands a few ulps below. Race replay: 2.0% of pixels
// differ by 1 with 0, 0.67% with 1e-4 (0.60% with the GS's own planes).
const N3_BIAS: f32 = 0.496;

struct N3V {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(linear) col: vec4<f32>,
    @location(1) @interpolate(linear) stq: vec3<f32>,
    // fog, dz
    @location(2) @interpolate(linear) fz: vec2<f32>,
    // setup record, z0, flags, RGBA8 of the provoking vertex
    @location(3) @interpolate(flat) info: vec4<u32>,
    // Z plane as the compute GS evaluates it: origin, base, fraction,
    // dZ/dx; and dZ/dy
    @location(4) @interpolate(flat) zp: vec4<u32>,
    @location(5) @interpolate(flat) zdy: u32,
    // Diagnostics (SSX3_N3_PLANES=1): the compute GS's attribute planes
    // (`data` offset, 0xFFFFFFFF = none) instead of hardware interpolation.
    @location(6) @interpolate(flat) pl: u32,
}

@vertex
fn n3_vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) inst: u32) -> N3V {
    let b = params.a.x + vi * N3_VWORDS;
    var o: N3V;
    let sx = 2.0 / f32(params.c.x);
    let sy = 2.0 / f32(params.c.y);
    o.pos = vec4<f32>(bitcast<f32>(data[b]) * sx - 1.0, 1.0 - bitcast<f32>(data[b + 1u]) * sy, 0.5, 1.0);
    o.col = vec4<f32>(unpack4(data[b + 2u]));
    o.stq = vec3<f32>(bitcast<f32>(data[b + 3u]), bitcast<f32>(data[b + 4u]), bitcast<f32>(data[b + 5u]));
    o.fz = vec2<f32>(bitcast<f32>(data[b + 6u]), bitcast<f32>(data[b + 7u]));
    o.info = vec4<u32>(data[b + 9u], data[b + 8u], data[b + 10u] | (inst << 4u), data[b + 2u]);
    o.zp = vec4<u32>(data[b + 11u], data[b + 12u], data[b + 13u], data[b + 14u]);
    o.zdy = data[b + 15u];
    o.pl = data[b + 16u];
    return o;
}

struct N3Px {
    // r, g, b, alpha written (FBA applied)
    c: vec4<i32>,
    // source alpha (blend factor As)
    a: i32,
    // colour output (0..255 scale): Cs, or the premultiplied source term
    rgb: vec3<f32>,
    depth: f32,
    kill: bool,
}

// The pixel pipeline of gs.wgsl `pixel` up to the alpha test: Z, colour,
// texture function, fog, alpha test (AFAIL = KEEP only; the CPU sends other
// AFAIL modes to the emulated GS). Depth test, blending and the frame write
// are the render pipeline's.
fn n3_shade(vin: N3V) -> N3Px {
    var v = vin;
    let n3_eps = bitcast<f32>(params.d.x);
    if (v.pl != 0xFFFFFFFFu) {
        // gs.wgsl `frag` (triangles): planes at the integer pixel.
        let p = v.pl;
        let lo = data[p];
        let i = f32(i32(v.pos.x) - i32(lo & 0xFFFFu));
        let j = f32(i32(v.pos.y) - i32(lo >> 16u));
        var at: array<f32, 8>;
        for (var k = 0u; k < 8u; k = k + 1u) {
            at[k] = fat(p + 1u + k) + fat(p + 9u + k) * i + fat(p + 17u + k) * j;
        }
        v.col = vec4<f32>(at[0], at[1], at[2], at[3]);
        v.stq = vec3<f32>(at[4], at[5], at[6]);
        v.fz.x = at[7];
    }
    let st = v.info.x;
    let flags = data[st + S_FLAGS];
    let modes = data[st + S_MODES];
    var o: N3Px;
    o.kill = false;
    var z = 0u;
    if ((flags & F_ZTE) != 0u) {
        // gs.wgsl draw_prim: the same f32 plane at the same integer pixel.
        let i = f32(i32(v.pos.x) - i32(v.zp.x & 0xFFFFu));
        let j = f32(i32(v.pos.y) - i32(v.zp.x >> 16u));
        z = z_add(v.zp.y, bitcast<f32>(v.zp.z) + bitcast<f32>(v.zp.w) * i + bitcast<f32>(v.zdy) * j, data[st + S_ZMAX]);
    }
    o.depth = f32(z) / NATIVE_DEPTH_SCALE;
    var r: i32;
    var g: i32;
    var b: i32;
    var a: i32;
    if ((v.info.z & 2u) != 0u) {
        let c = unpack_rgba(v.info.w);
        r = c.x;
        g = c.y;
        b = c.z;
        a = c.w;
    } else {
        r = i32(v.col.x + n3_eps);
        g = i32(v.col.y + n3_eps);
        b = i32(v.col.z + n3_eps);
        a = i32(v.col.w + n3_eps);
    }
    if ((flags & F_TME) != 0u) {
        let q = v.stq.z;
        let uv = tex_uv(st, v.stq.x, v.stq.y, q);
        let c = sample_tex(st, uv.x, uv.y, q, (v.info.z & 1u) != 0u);
        let tr = i32(c & 0xFFu);
        let tg = i32((c >> 8u) & 0xFFu);
        let tb = i32((c >> 16u) & 0xFFu);
        let ta = i32(c >> 24u);
        let tcc = (flags & F_TCC) != 0u;
        switch (modes >> 15u) & 3u {
            case 0u: {
                r = min((tr * r) >> 7u, 255);
                g = min((tg * g) >> 7u, 255);
                b = min((tb * b) >> 7u, 255);
                if (tcc) {
                    a = min((ta * a) >> 7u, 255);
                }
            }
            case 1u: {
                r = tr;
                g = tg;
                b = tb;
                if (tcc) {
                    a = ta;
                }
            }
            case 2u: {
                r = min(((tr * r) >> 7u) + a, 255);
                g = min(((tg * g) >> 7u) + a, 255);
                b = min(((tb * b) >> 7u) + a, 255);
                if (tcc) {
                    a = min(ta + a, 255);
                }
            }
            default: {
                r = min(((tr * r) >> 7u) + a, 255);
                g = min(((tg * g) >> 7u) + a, 255);
                b = min(((tb * b) >> 7u) + a, 255);
                if (tcc) {
                    a = ta;
                }
            }
        }
    }
    if ((flags & F_FGE) != 0u) {
        let fv = clamp(i32(v.fz.x + n3_eps), 0, 255);
        let fc = data[st + S_FOGCOL];
        r = (fv * r + (255 - fv) * i32(fc & 0xFFu)) >> 8u;
        g = (fv * g + (255 - fv) * i32((fc >> 8u) & 0xFFu)) >> 8u;
        b = (fv * b + (255 - fv) * i32((fc >> 16u) & 0xFFu)) >> 8u;
    }
    if ((flags & F_ATE) != 0u) {
        let aref = i32(data[st + S_REFS] & 0xFFu);
        var ok = false;
        switch (modes >> 2u) & 7u {
            case 0u: { ok = false; }
            case 1u: { ok = true; }
            case 2u: { ok = a < aref; }
            case 3u: { ok = a <= aref; }
            case 4u: { ok = a == aref; }
            case 5u: { ok = a >= aref; }
            case 6u: { ok = a > aref; }
            default: { ok = a != aref; }
        }
        // Instance 1 draws the failing pixels of an FB_ONLY test.
        o.kill = select(!ok, ok, (v.info.z & 16u) != 0u);
    }
    var oa = a & 0xFF;
    if ((flags & F_FBA) != 0u) {
        oa = oa | 0x80;
    }
    o.c = vec4<i32>(r, g, b, oa);
    o.a = a;
    o.rgb = vec3<f32>(f32(r), f32(g), f32(b));
    if ((v.info.z & 4u) != 0u) {
        // Blending: the source term of (A - B) * C + D premultiplied here
        // (the pipeline adds it with factor One), biased so that the
        // hardware's round-to-nearest gives the GS's floor: the exact
        // result is a multiple of 1/128, so floor(R) = round(R - 0.496).
        // Flag 8: the source term is not positive and the pipeline
        // subtracts it (reverse subtract), so the bias changes sign.
        let ba = (modes >> 7u) & 3u;
        let bb = (modes >> 9u) & 3u;
        let bc = (modes >> 11u) & 3u;
        let bd = (modes >> 13u) & 3u;
        var cc = i32((data[st + S_REFS] >> 8u) & 0xFFu);
        if (bc == 0u) {
            cc = a;
        }
        let k = select(0, 128, bd == 0u) + (select(0, 1, ba == 0u) - select(0, 1, bb == 0u)) * cc;
        let s = vec3<f32>(f32(r), f32(g), f32(b)) * (f32(k) / 128.0);
        if ((v.info.z & 8u) != 0u) {
            o.rgb = -s + vec3<f32>(N3_BIAS);
        } else {
            o.rgb = s - vec3<f32>(N3_BIAS);
        }
    }
    return o;
}

struct N3Out {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn n3_fs(v: N3V) -> N3Out {
    let p = n3_shade(v);
    if (p.kill) {
        discard;
    }
    var o: N3Out;
    o.color = vec4<f32>(p.rgb, f32(p.c.w)) / 255.0;
    o.depth = p.depth;
    return o;
}
