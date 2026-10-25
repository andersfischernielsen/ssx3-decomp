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

// Sun shadows (shadow.rs): GS screen → world (relative to a point near the
// camera), world → light clip per cascade (4, in the atlas' quarters:
// 0 top left, 1 top right, 2 bottom left, 3 bottom right), (towards the
// sun, strength), the eye (world), (atlas texel in UV, -, on, show the
// factor), each cascade's world units per texel.
struct ShadowU {
    s2w: mat4x4<f32>,
    lights: array<mat4x4<f32>, 4>,
    sun: vec4<f32>,
    eye: vec4<f32>,
    p: vec4<f32>,
    wt: vec4<f32>,
    // The point world positions are relative to (absolute world).
    origin: vec4<f32>,
    // Full shadow: the colour lit surfaces are multiplied by (sky-lit
    // shade, cool like the game's baked shadows).
    shade: vec4<f32>,
    // Terrain lighting (shadow.rs `Lighting`).
    sun_col: vec4<f32>,
    sky_col: vec4<f32>,
    ground_col: vec4<f32>,
    // The world lights (shadow.rs `MAX_LIGHTS`): (count, gain), then per
    // light (position, radius), (colour, intensity), (spot axis, kind:
    // 1 spot, 2 point), (cos inner, cos outer, falloff, exponent).
    nl: vec4<f32>,
    pl: array<vec4<f32>, 256>,
}
@group(1) @binding(5) var<uniform> sh: ShadowU;
@group(1) @binding(6) var smap: texture_depth_2d;
@group(1) @binding(7) var scmp: sampler_comparison;

// The shadow pass's cascade (the draw's instance).
var<private> cascade: u32;

// The cascade `sun_lit` last used (debug view), 4 = none.
var<private> used_cascade: u32;

// How much sun reaches world point `w` with normal `n` (PCF over 3×3
// texels in the first cascade that holds it: four filtered taps, each
// blending 2×2, half a texel off the point): 1 outside them all.
fn sun_lit(w: vec3<f32>, n: vec3<f32>) -> f32 {
    used_cascade = 4u;
    for (var c = 0u; c < 4u; c = c + 1u) {
        let lc = sh.lights[c] * vec4<f32>(w + n * sh.wt[c] * 1.5, 1.0);
        let uv = vec2<f32>(lc.x * 0.5 + 0.5, 0.5 - lc.y * 0.5);
        let m = 4.0 * sh.p.x;
        if (any(uv < vec2<f32>(m)) || any(uv > vec2<f32>(1.0 - m)) || lc.z > 1.0 || lc.z < 0.0) {
            continue;
        }
        used_cascade = c;
        let at = (uv + vec2<f32>(f32(c & 1u), f32(c >> 1u))) * 0.5;
        let h = 0.5 * sh.p.x;
        let s = textureSampleCompareLevel(smap, scmp, at + vec2<f32>(-h, -h), lc.z)
            + textureSampleCompareLevel(smap, scmp, at + vec2<f32>(h, -h), lc.z)
            + textureSampleCompareLevel(smap, scmp, at + vec2<f32>(-h, h), lc.z)
            + textureSampleCompareLevel(smap, scmp, at + vec2<f32>(h, h), lc.z);
        return s * 0.25;
    }
    return 1.0;
}

// How much of the sun is missing at world point `w` with face derivatives
// `dx`, `dy` (0 lit .. 1 in shadow, times the strength). Surfaces turned away
// from the sun are left alone: the game's baked lighting already darkens
// them.
fn sun_factor(w: vec3<f32>, dx: vec3<f32>, dy: vec3<f32>, vn: vec4<f32>) -> f32 {
    // (No surface to speak of, e.g. a pixel whose neighbours lie on the
    // same point: lit. A NaN here would poison the blended layers.)
    let c = cross(dx, dy);
    let cl = dot(c, c);
    if (!(cl > 1e-20) || !(cl < 1e30)) {
        return 0.0;
    }
    var n = c * inverseSqrt(cl);
    if (dot(n, sh.eye.xyz - w) < 0.0) {
        n = -n;
    }
    // A smooth vertex normal (riders): the terminator follows the shape
    // (the game's own lighting darkens the side away from the sun), and
    // the faces near it, which would shadow themselves, fade in later.
    if (vn.w > 0.5) {
        let ns = normalize(vn.xyz);
        let ndl = dot(ns, sh.sun.xyz);
        let lit = mix(1.0, sun_lit(w, ns), smoothstep(0.1, 0.4, ndl));
        return sh.sun.w * (1.0 - lit);
    }
    let ndl = dot(n, sh.sun.xyz);
    let lit = mix(1.0, sun_lit(w, n), smoothstep(0.0, 0.2, ndl));
    return sh.sun.w * (1.0 - lit);
}

// The world lights' light at world point `w` with normal `n`: the game's
// formula for its riders (`Irradiance_AddDynamicLight`,
// docs/investigation/world-lights.md): k = falloff (100/d)^power ·
// intensity · the far fade of radius-5000 lights · the spot cone, at most
// 5; light = colour · k · N·L.
fn world_lights(w: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    var acc = vec3<f32>(0.0);
    let count = u32(sh.nl.x);
    for (var i = 0u; i < count; i = i + 1u) {
        let p0 = sh.pl[4u * i];
        let v = p0.xyz - w;
        let d2 = dot(v, v);
        if (d2 > p0.w * p0.w) {
            continue;
        }
        let d = max(sqrt(d2), 1.0);
        let l = v / d;
        let ndl = dot(n, l);
        if (ndl <= 0.0) {
            continue;
        }
        let p1 = sh.pl[4u * i + 1u];
        let p2 = sh.pl[4u * i + 2u];
        let p3 = sh.pl[4u * i + 3u];
        let spot = p2.w < 1.5;
        let cs = -dot(l, p2.xyz);
        if (spot && cs < p3.y) {
            continue;
        }
        let lin = select(1.0, 1.0 - (d - 3750.0) * 0.0008, p0.w >= 5000.0 && d > 3750.0);
        let x = 100.0 / d;
        var fall = 1.0;
        if (x < 1.0) {
            if (p3.z == 1.0) {
                fall = x;
            } else if (p3.z == 2.0) {
                fall = x * x;
            } else if (p3.z == 3.0) {
                fall = x * x * x;
            }
        }
        var k = fall * p1.w * lin;
        if (spot) {
            if (cs >= p3.x) {
                k = k * pow(cs, p3.w);
            } else {
                k = k * pow(p3.x, p3.w) * (cs - p3.y) / max(p3.x - p3.y, 1e-6);
            }
        }
        acc = acc + p1.rgb * clamp(k, 0.0, 5.0) * ndl;
    }
    return acc * sh.nl.y;
}

// The renderer's terrain lighting at world point `w` with smooth normal `n`
// (flag 16384, the terrain's base layer; its baked layer is not drawn):
// sky fill + sun · N·L · lit. Returns (L, the lit amount, N·L).
fn terrain_light(w: vec3<f32>, n_in: vec3<f32>) -> mat3x3<f32> {
    // Terrain faces the sky (the camera can dip under a lip: turning the
    // normal to the eye would light its underside); walls, to the eye.
    var n = normalize(n_in);
    if ((abs(n.z) > 0.2 && n.z < 0.0) || (abs(n.z) <= 0.2 && dot(n, sh.eye.xyz - w) < 0.0)) {
        n = -n;
    }
    let ndl = max(dot(n, sh.sun.xyz), 0.0);
    var lit = 0.0;
    if (ndl > 0.0) {
        lit = sun_lit(w, n);
    }
    let sky = mix(sh.ground_col.rgb, sh.sky_col.rgb, n.z * 0.5 + 0.5);
    let l = (sky + sh.sun_col.rgb * (ndl * lit) + world_lights(w, n)) * sh.sun_col.w;
    return mat3x3<f32>(l, vec3<f32>(lit, ndl, 1.0), n);
}

// The colour a lit surface is multiplied by for shadow amount `s`.
fn shadow_tint(s: f32) -> vec3<f32> {
    return mix(vec3<f32>(1.0), sh.shade.rgb, s);
}

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
    if (sh.p.z > 0.5) {
        let h = sh.s2w * hc;
        o.wp = vec4<f32>(h.xyz / h.w, 1.0);
    }
    // (The shadow pass, shadow.wgsl: the same point seen from the sun.)
    return shadow_out(o, hc);
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

// The renderer's lighting is on (`fs_gpu`'s pipelines are made with it
// false when it is off: its code compiled out. Untaken, it still cost the
// shader's registers: 18.5 -> 12.2 ms a frame at 1080p on an M1, Metro City).
override LIGHTING: bool = true;

// The GS depth of a fragment (the position's z is the clip z).
fn gs_depth(v: VOut) -> f32 {
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
    return min(z, zmax) / (zmax + 1.0);
}

// The GS fragment stage with the GS depth.
@fragment
fn fs_gpu(v: VOut) -> FsOut {
    return gpu_fragment(v);
}

// The same with the depth from the vertices (the clip-depth pipelines,
// gpuvu/clip_depth.wgsl): the GPU keeps its early depth test.
@fragment
fn fs_gpu_nd(v: VOut) -> @location(0) vec4<f32> {
    return gpu_fragment(v).color;
}

fn gpu_fragment(v: VOut) -> FsOut {
    if (!LIGHTING) {
        var o: FsOut;
        o.color = shade(v);
        o.depth = gs_depth(v);
        return o;
    }
    // (Derivatives before any discard.)
    let dx = dpdx(v.wp.xyz);
    let dy = dpdy(v.wp.xyz);
    let d = gs_depth(v);
    // The renderer's lighting multiplies the colour before the fog (in
    // `shade`): fogged-out distant terrain stays the fog's colour.
    let terrain = (b.flags & 16384u) != 0u && v.wp.w > 0.5 && v.nrm.w > 0.5;
    let receiver = !terrain && (b.flags & 4096u) != 0u && v.wp.w > 0.5 && b.src_mode != 2u;
    var t: mat3x3<f32>;
    var s = 0.0;
    var spray = false;
    if ((b.flags & 65536u) != 0u && v.wp.w > 0.5) {
        // Decals on the terrain (board trails, flag 65536): lit as the
        // surface they lie on (their face, turned to the sky).
        let c = cross(dx, dy);
        if (dot(c, c) > 1e-20) {
            pre_fog = terrain_light(v.wp.xyz, normalize(c))[0];
            hdr = true;
        }
    } else if ((b.flags & 131072u) != 0u && v.wp.w > 0.5) {
        // Snow spray (flag 131072): its colour is the baked light where it
        // flew up; neutral, lit as the terrain there (facing up). (The
        // glowing effects are additive: not flagged.)
        spray = true;
        pre_fog = terrain_light(v.wp.xyz, vec3<f32>(0.0, 0.0, 1.0))[0];
        hdr = true;
    } else if (terrain) {
        // The terrain's lighting (flag 16384).
        t = terrain_light(v.wp.xyz, v.nrm.xyz);
        pre_fog = t[0];
        hdr = true;
    } else if (receiver) {
        s = sun_factor(v.wp.xyz, dx, dy, v.nrm);
        pre_fog = shadow_tint(s);
    }
    var vs = v;
    if ((b.flags & 65536u) != 0u || spray) {
        vs.col = vec4<f32>(vec3<f32>(128.0), v.col.a);
    }
    // The baked layer's colour (flag 32768): blurred wide, its zone and
    // shade colours without the shapes of its shadows.
    if ((b.flags & 32768u) != 0u) {
        blur_texels = 8.0;
    }
    var o: FsOut;
    o.color = shade(vs);
    // The bake's colour only (flag 32768, real-time lighting): its source
    // (premultiplied: what it subtracts) less its grey part, and the
    // destination kept (alpha 1: the blend scales it by As).
    if ((b.flags & 32768u) != 0u) {
        let c = o.color.rgb;
        o.color = vec4<f32>(max(c - vec3<f32>(min(min(c.r, c.g), c.b)), vec3<f32>(0.0)), 1.0);
    }
    // Debug views.
    if (sh.p.w == 9.0 && terrain) {
        // (Blue: the sun's share of the light.)
        let lum = vec3<f32>(0.2126, 0.7152, 0.0722);
        let share = dot(sh.sun_col.rgb * (t[1].y * t[1].x), lum) / max(dot(t[0] / sh.sun_col.w, lum), 1e-6);
        o.color = vec4<f32>(clamp((log2(max(hdr_y, 1e-4)) + 8.0) / 12.0, 0.0, 1.0), 1.0, clamp(share, 0.0, 1.0), 1.0);
        o.depth = d;
        return o;
    }
    if (sh.p.w == 8.0) {
        // (The opaque layers of the lit draws only.)
        let opaque = b.src_mode == 0u && b.c_kind != 1u || b.src_mode == 1u && b.cs_a == 0.0 && b.cs_b == 1.0;
        if (!(terrain || receiver) || !opaque || v.wp.w < 0.5) {
            discard;
        }
        o.color = vec4<f32>(fract((v.wp.xyz + sh.origin.xyz) / 200.0), 1.0);
        o.depth = d;
        return o;
    }
    if ((b.flags & 65536u) != 0u && sh.p.w == 7.0) {
        o.color = vec4<f32>(v.col.rgb / 128.0 * select(1.0, o.color.a, b.src_mode == 1u), o.color.a);
    }
    if (terrain) {
        if (sh.p.w == 4.0) {
            o.color = vec4<f32>(t[2] * 0.5 + 0.5, 1.0);
        } else if (sh.p.w == 5.0) {
            o.color = vec4<f32>(t[1], 1.0);
        } else if (sh.p.w == 6.0) {
            o.color = vec4<f32>(t[2].xy * 0.5 + 0.5, 0.5 + 0.5 * t[1].x, 1.0);
        }
    } else if (receiver && sh.p.w > 0.0) {
        let f = 1.0 - s;
        // (On the opaque layer and the plain alpha blend (premultiplied by
        // As, as `shade` gives it: riders); the other blends would add or
        // subtract the view's colours.)
        let lerp = b.src_mode == 1u && b.cs_a == 1.0 && b.cs_b == 0.0;
        let plain = (b.src_mode == 0u && b.c_kind != 1u) || (b.src_mode == 1u && b.cs_a == 0.0 && b.cs_b == 1.0);
        if (!plain && !lerp) {
            discard;
        }
        let pm = select(1.0, o.color.a, lerp);
        if (sh.p.w == 1.0) {
            o.color = vec4<f32>(vec3<f32>(f) * pm, o.color.a);
        } else if (sh.p.w == 2.0) {
            let cc = array<vec3<f32>, 5>(vec3<f32>(1.0, 0.3, 0.3), vec3<f32>(0.3, 1.0, 0.3), vec3<f32>(0.3, 0.3, 1.0), vec3<f32>(1.0, 1.0, 0.3), vec3<f32>(0.5));
            o.color = vec4<f32>(cc[used_cascade] * (0.4 + 0.6 * f) * pm, o.color.a);
        } else if (sh.p.w == 3.0) {
            var n = normalize(cross(dx, dy));
            if (dot(n, sh.eye.xyz - v.wp.xyz) < 0.0) {
                n = -n;
            }
            let ndl = dot(n, sh.sun.xyz);
            o.color = vec4<f32>(vec3<f32>(max(ndl, 0.0), max(-ndl, 0.0), select(0.0, 1.0, ndl > 0.0 && ndl < 0.2)) * pm, o.color.a);
        }
    }
    o.depth = d;
    return o;
}
