// ssx3-gfx: the renderer's own full-screen passes (options.rs), drawn on the
// 3D image before the HUD. Colours are the frame's stored values (sRGB-ish
// 0..1); the alpha channel (the GS's destination alpha) is never written.

struct P {
    // Source texel size (1 / source size).
    texel: vec2<f32>,
    // Pass parameter: the grade mode, the blur direction, ...
    mode: u32,
    strength: f32,
    dir: vec2<f32>,
    // Depth: the main target's size in pixels.
    size: vec2<f32>,
    // Sun shadows: world → GS screen (homogeneous) and back, (XYOFFSET/16,
    // target pixels per GS pixel), (ZMAX + 1, Z per screen z, ray length,
    // strength), the light direction (world, towards the light).
    w2s: mat4x4<f32>,
    s2w: mat4x4<f32>,
    off: vec4<f32>,
    zp: vec4<f32>,
    light: vec4<f32>,
}

@group(0) @binding(0) var<uniform> p: P;
@group(0) @binding(1) var src: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;
@group(0) @binding(3) var depth: texture_depth_2d;
// A second image (the bloom composite: the frame before the bloom).
@group(0) @binding(4) var image: texture_2d<f32>;

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// One triangle covering the target.
@vertex
fn vs(@builtin(vertex_index) i: u32) -> VOut {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var o: VOut;
    o.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(xy.x, 1.0 - xy.y);
    return o;
}

fn luma(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

fn tex(uv: vec2<f32>) -> vec3<f32> {
    return textureSampleLevel(src, samp, uv, 0.0).rgb;
}

// ---------------------------------------------------------------- FXAA
// FXAA 3.11 (quality preset 12-ish): edge direction from the luma
// neighbourhood, a search along the edge, a blend across it.
@fragment
fn fs_fxaa(v: VOut) -> @location(0) vec4<f32> {
    let t = p.texel;
    let uv = v.uv;
    let rgbM = tex(uv);
    let lM = luma(rgbM);
    let lN = luma(tex(uv + vec2<f32>(0.0, -t.y)));
    let lS = luma(tex(uv + vec2<f32>(0.0, t.y)));
    let lE = luma(tex(uv + vec2<f32>(t.x, 0.0)));
    let lW = luma(tex(uv + vec2<f32>(-t.x, 0.0)));
    let lMax = max(lM, max(max(lN, lS), max(lE, lW)));
    let lMin = min(lM, min(min(lN, lS), min(lE, lW)));
    let range = lMax - lMin;
    if (range < max(0.0312, lMax * 0.125)) {
        return vec4<f32>(rgbM, 1.0);
    }
    let lNW = luma(tex(uv + vec2<f32>(-t.x, -t.y)));
    let lNE = luma(tex(uv + vec2<f32>(t.x, -t.y)));
    let lSW = luma(tex(uv + vec2<f32>(-t.x, t.y)));
    let lSE = luma(tex(uv + vec2<f32>(t.x, t.y)));
    // Sub-pixel aliasing amount.
    let lAvg = (2.0 * (lN + lS + lE + lW) + lNW + lNE + lSW + lSE) / 12.0;
    let sub = clamp(abs(lAvg - lM) / range, 0.0, 1.0);
    let subBlend = smoothstep(0.0, 1.0, sub) * smoothstep(0.0, 1.0, sub) * 0.75;
    // Horizontal or vertical edge.
    let edgeH = abs(lNW + lNE - 2.0 * lN) + 2.0 * abs(lW + lE - 2.0 * lM) + abs(lSW + lSE - 2.0 * lS);
    let edgeV = abs(lNW + lSW - 2.0 * lW) + 2.0 * abs(lN + lS - 2.0 * lM) + abs(lNE + lSE - 2.0 * lE);
    let horz = edgeH >= edgeV;
    var step = select(t.x, t.y, horz);
    let l1 = select(lW, lN, horz);
    let l2 = select(lE, lS, horz);
    let g1 = abs(l1 - lM);
    let g2 = abs(l2 - lM);
    var lEdge = 0.0;
    var grad = 0.0;
    if (g1 >= g2) {
        step = -step;
        lEdge = 0.5 * (l1 + lM);
        grad = g1;
    } else {
        lEdge = 0.5 * (l2 + lM);
        grad = g2;
    }
    var uvE = uv;
    var along = vec2<f32>(t.x, 0.0);
    if (horz) {
        uvE.y = uvE.y + step * 0.5;
    } else {
        uvE.x = uvE.x + step * 0.5;
        along = vec2<f32>(0.0, t.y);
    }
    // Search both ways along the edge for its ends.
    let gScaled = grad * 0.25;
    var uvP = uvE + along;
    var uvN = uvE - along;
    var dP = luma(tex(uvP)) - lEdge;
    var dN = luma(tex(uvN)) - lEdge;
    var doneP = abs(dP) >= gScaled;
    var doneN = abs(dN) >= gScaled;
    let steps = array<f32, 8>(1.0, 1.5, 2.0, 2.0, 2.0, 4.0, 8.0, 8.0);
    for (var i = 0; i < 8; i = i + 1) {
        if (doneP && doneN) {
            break;
        }
        if (!doneP) {
            uvP = uvP + along * steps[i];
            dP = luma(tex(uvP)) - lEdge;
            doneP = abs(dP) >= gScaled;
        }
        if (!doneN) {
            uvN = uvN - along * steps[i];
            dN = luma(tex(uvN)) - lEdge;
            doneN = abs(dN) >= gScaled;
        }
    }
    let distP = select(uvP.x - uv.x, uvP.y - uv.y, !horz);
    let distN = select(uv.x - uvN.x, uv.y - uvN.y, !horz);
    let dist = min(distP, distN);
    let span = distP + distN;
    let mSmaller = (lM - lEdge) < 0.0;
    let goodSpan = select((dN < 0.0) != mSmaller, (dP < 0.0) != mSmaller, distP < distN);
    var off = select(0.0, 0.5 - dist / span, goodSpan);
    off = max(off, subBlend);
    var uvF = uv;
    if (horz) {
        uvF.y = uvF.y + off * step;
    } else {
        uvF.x = uvF.x + off * step;
    }
    return vec4<f32>(tex(uvF), 1.0);
}

// ---------------------------------------------------------------- grade
fn aces(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_grade(v: VOut) -> @location(0) vec4<f32> {
    var c = tex(v.uv);
    if (p.mode == 1u) {
        // Vivid: saturation, then a gentle contrast S-curve.
        let l = luma(c);
        c = mix(vec3<f32>(l), c, 1.3);
        c = clamp(c, vec3<f32>(0.0), vec3<f32>(1.0));
        c = c + (c - 0.5) * 0.12 * (1.0 - abs(c - 0.5) * 2.0);
    } else if (p.mode == 2u) {
        // Filmic: in linear light, a filmic curve (soft highlights), slight
        // warmth, back to the stored encoding.
        // (Normalised so white stays white: an S-curve, not a brightening.)
        var lin = pow(max(c, vec3<f32>(0.0)), vec3<f32>(2.2));
        lin = lin * vec3<f32>(1.03, 1.0, 0.96);
        let k = 1.2;
        lin = aces(lin * k) / aces(vec3<f32>(k));
        c = pow(clamp(lin, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2));
        let l = luma(c);
        c = mix(vec3<f32>(l), c, 1.12);
    }
    return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}

// ---------------------------------------------------------------- bloom
// Bright pass and the first downsample: a 13-tap box filter of the colour
// above a soft threshold.
@fragment
fn fs_bright(v: VOut) -> @location(0) vec4<f32> {
    let t = p.texel;
    var c = vec3<f32>(0.0);
    c = c + tex(v.uv) * 0.5;
    c = c + (tex(v.uv + vec2<f32>(-t.x, -t.y)) + tex(v.uv + vec2<f32>(t.x, -t.y)) + tex(v.uv + vec2<f32>(-t.x, t.y)) + tex(v.uv + vec2<f32>(t.x, t.y))) * 0.125;
    let lin = pow(max(c, vec3<f32>(0.0)), vec3<f32>(2.2));
    let l = max(lin.r, max(lin.g, lin.b));
    // Soft knee around the threshold.
    // (Above snow white: only real highlights glow.)
    let th = 0.92;
    let knee = 0.08;
    var s = clamp(l - th + knee, 0.0, 2.0 * knee);
    s = s * s / (4.0 * knee + 1e-4);
    let w = max(s, l - th) / max(l, 1e-4);
    return vec4<f32>(lin * w, 1.0);
}

// Dual-filter downsample (Kawase): centre and four diagonal taps.
@fragment
fn fs_down(v: VOut) -> @location(0) vec4<f32> {
    let t = p.texel * 0.5;
    var c = tex(v.uv) * 4.0;
    c = c + tex(v.uv + vec2<f32>(-t.x, -t.y)) + tex(v.uv + vec2<f32>(t.x, -t.y)) + tex(v.uv + vec2<f32>(-t.x, t.y)) + tex(v.uv + vec2<f32>(t.x, t.y));
    return vec4<f32>(c / 8.0, 1.0);
}

// Dual-filter upsample: eight taps, added onto the level above.
@fragment
fn fs_up(v: VOut) -> @location(0) vec4<f32> {
    let t = p.texel * 0.5;
    var c = vec3<f32>(0.0);
    c = c + tex(v.uv + vec2<f32>(-t.x * 2.0, 0.0));
    c = c + tex(v.uv + vec2<f32>(-t.x, t.y)) * 2.0;
    c = c + tex(v.uv + vec2<f32>(0.0, t.y * 2.0));
    c = c + tex(v.uv + vec2<f32>(t.x, t.y)) * 2.0;
    c = c + tex(v.uv + vec2<f32>(t.x * 2.0, 0.0));
    c = c + tex(v.uv + vec2<f32>(t.x, -t.y)) * 2.0;
    c = c + tex(v.uv + vec2<f32>(0.0, -t.y * 2.0));
    c = c + tex(v.uv + vec2<f32>(-t.x, -t.y)) * 2.0;
    return vec4<f32>(c / 12.0, 1.0);
}

// The image (binding 4) with the blurred bright light (src) added in
// linear light.
@fragment
fn fs_bloom_mix(v: VOut) -> @location(0) vec4<f32> {
    let base = textureSampleLevel(image, samp, v.uv, 0.0).rgb;
    let lin = pow(max(base, vec3<f32>(0.0)), vec3<f32>(2.2)) + tex(v.uv) * p.strength;
    return vec4<f32>(pow(clamp(lin, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2)), 1.0);
}

// ---------------------------------------------------------------- AO
// Depth-only ambient occlusion. The depth buffer holds the GS's Z scaled to
// 0..1, larger nearer (proportional to 1/w): view depth is taken as 1/z.
// Samples in a disc whose screen radius shrinks with distance; a sample
// occludes when it is nearer than the pixel by a fraction of the pixel's
// depth, within a range (so distant geometry does not darken edges).
fn vdepth(q: vec2<i32>) -> f32 {
    let qs = clamp(q, vec2<i32>(0), vec2<i32>(p.size) - vec2<i32>(1));
    let z = textureLoad(depth, qs, 0);
    if (p.zp.x > 0.0) {
        // Exact: the view depth (homogeneous w) of the reconstructed point.
        return max(view_w(vec2<f32>(qs) + 0.5, z), 1e-4);
    }
    return 1.0 / max(z, 1e-7);
}

// The view depth of target pixel `px` at depth `d` (terrain camera).
fn view_w(px: vec2<f32>, d: f32) -> f32 {
    let x = px.x / p.off.z - 0.5 + p.off.x;
    let y = px.y / p.off.w - 0.5 + p.off.y;
    let z = d * p.zp.x / p.zp.y;
    let h = p.s2w * vec4<f32>(x, y, z, 1.0);
    let w = h.xyz / h.w;
    return (p.w2s * vec4<f32>(w, 1.0)).w;
}

// World position of pixel `q` (terrain camera).
fn wpos(q: vec2<i32>) -> vec3<f32> {
    let qs = clamp(q, vec2<i32>(0), vec2<i32>(p.size) - vec2<i32>(1));
    return to_world(vec2<f32>(qs) + 0.5, textureLoad(depth, qs, 0)).xyz;
}

// With the camera: occlusion by the points above the surface's tangent
// plane within a world-space radius (normal from the neighbours' positions,
// the shorter difference each way so no normal spans a depth edge).
fn ao_world(v: VOut) -> f32 {
    let px = vec2<i32>(v.pos.xy);
    let pc = wpos(px);
    let rx = wpos(px + vec2<i32>(1, 0)) - pc;
    let lx = pc - wpos(px - vec2<i32>(1, 0));
    let dy = wpos(px + vec2<i32>(0, 1)) - pc;
    let uy = pc - wpos(px - vec2<i32>(0, 1));
    let ex = select(lx, rx, dot(rx, rx) < dot(lx, lx));
    let ey = select(uy, dy, dot(dy, dy) < dot(uy, uy));
    var n = normalize(cross(ex, ey));
    // Towards the camera: along the pixel's ray, from far to near.
    let s0 = to_screen(pc).xy;
    let to_cam = normalize(to_world(s0, 0.9).xyz - to_world(s0, 0.0001).xyz);
    if (dot(n, to_cam) < 0.0) {
        n = -n;
    }
    // World radius, and its size on screen.
    let rw = 80.0;
    let rpx = max(max(length(to_screen(pc + vec3<f32>(rw, 0.0, 0.0)).xy - s0), length(to_screen(pc + vec3<f32>(0.0, rw, 0.0)).xy - s0)), length(to_screen(pc + vec3<f32>(0.0, 0.0, rw)).xy - s0));
    let r = clamp(rpx, 2.0, p.size.y * 0.08);
    let noise = fract(52.9829189 * fract(dot(v.pos.xy, vec2<f32>(0.06711056, 0.00583715))));
    var occ = 0.0;
    let ns = 12;
    for (var i = 0; i < ns; i = i + 1) {
        let fi = f32(i);
        let a = (fi + noise) * 2.3999632;
        let rr = r * sqrt((fi + 0.5) / f32(ns));
        let d = wpos(px + vec2<i32>(vec2<f32>(cos(a), sin(a)) * rr)) - pc;
        let l2 = dot(d, d);
        let fall = max(1.0 - l2 / (4.0 * rw * rw), 0.0);
        occ = occ + max(dot(d, n) / sqrt(l2 + 1e-4) - 0.15, 0.0) * fall;
    }
    return clamp(1.0 - p.strength * 3.0 * occ / f32(ns), 0.0, 1.0);
}

@fragment
fn fs_ao(v: VOut) -> @location(0) vec4<f32> {
    let px = vec2<i32>(v.pos.xy);
    let z = textureLoad(depth, px, 0);
    if (z <= 1e-6) {
        return vec4<f32>(1.0);
    }
    if (p.zp.x > 0.0) {
        return vec4<f32>(vec3<f32>(ao_world(v)), 1.0);
    }
    let d = vdepth(px);
    // Interleaved gradient noise rotates the sample pattern per pixel.
    let noise = fract(52.9829189 * fract(dot(v.pos.xy, vec2<f32>(0.06711056, 0.00583715))));
    let r = p.size.y * 0.045;
    var occ = 0.0;
    let n = 12;
    for (var i = 0; i < n; i = i + 1) {
        let fi = f32(i);
        let a = (fi + noise) * 2.3999632;
        let rr = r * sqrt((fi + 0.5) / f32(n));
        let o = vec2<f32>(cos(a), sin(a)) * rr;
        let ds = vdepth(px + vec2<i32>(o));
        let diff = (d - ds) / d;
        if (diff > 0.004 && diff < 0.25) {
            occ = occ + 1.0 - smoothstep(0.08, 0.25, diff);
        }
    }
    let ao = 1.0 - p.strength * occ / f32(n);
    return vec4<f32>(vec3<f32>(clamp(ao, 0.0, 1.0)), 1.0);
}

// Separable blur of the AO, edge-aware by depth.
@fragment
fn fs_ao_blur(v: VOut) -> @location(0) vec4<f32> {
    let px = vec2<i32>(v.pos.xy);
    let d0 = vdepth(px);
    var sum = 0.0;
    var wsum = 0.0;
    for (var i = -4; i <= 4; i = i + 1) {
        let q = px + vec2<i32>(p.dir * f32(i));
        let w = exp(-f32(i * i) / 8.0) * select(0.0, 1.0, abs(vdepth(q) - d0) / d0 < 0.05);
        sum = sum + textureLoad(src, clamp(q, vec2<i32>(0), vec2<i32>(p.size) - vec2<i32>(1)), 0).r * w;
        wsum = wsum + w;
    }
    return vec4<f32>(vec3<f32>(sum / max(wsum, 1e-4)), 1.0);
}

// The AO as a colour, multiplied onto the image (blend: dst × src).
@fragment
fn fs_ao_apply(v: VOut) -> @location(0) vec4<f32> {
    let a = textureLoad(src, vec2<i32>(v.pos.xy), 0).r;
    return vec4<f32>(vec3<f32>(a), 1.0);
}

// ---------------------------------------------------------------- sun
// Screen-space sun shadows: each pixel's world position from the depth
// buffer and the terrain camera, a short march towards the light through
// the depth buffer; a sample behind a nearer surface (within a thickness)
// is in shadow.
fn to_world(px: vec2<f32>, d: f32) -> vec4<f32> {
    // Target pixel → GS screen (pixels + XYOFFSET), depth → screen z.
    // (The GS samples pixels at integer coordinates, the GPU at centres.)
    let x = px.x / p.off.z - 0.5 + p.off.x;
    let y = px.y / p.off.w - 0.5 + p.off.y;
    let z = d * p.zp.x / p.zp.y;
    let h = p.s2w * vec4<f32>(x, y, z, 1.0);
    return vec4<f32>(h.xyz / h.w, 1.0);
}

// The view depth (homogeneous w) of a world point, and its target pixel.
fn to_screen(w: vec3<f32>) -> vec4<f32> {
    let h = p.w2s * vec4<f32>(w, 1.0);
    let s = h.xyz / h.w;
    return vec4<f32>((s.x - p.off.x + 0.5) * p.off.z, (s.y - p.off.y + 0.5) * p.off.w, h.w, s.z);
}

@fragment
fn fs_sun(v: VOut) -> @location(0) vec4<f32> {
    let px = vec2<i32>(v.pos.xy);
    let d = textureLoad(depth, px, 0);
    if (d <= 1e-6) {
        return vec4<f32>(1.0);
    }
    let wp = to_world(v.pos.xy, d).xyz;
    let w0 = to_screen(wp).z;
    let noise = fract(52.9829189 * fract(dot(v.pos.xy, vec2<f32>(0.06711056, 0.00583715))));
    let steps = 20;
    var lit = 1.0;
    for (var i = 1; i <= steps; i = i + 1) {
        let t = p.zp.z * (f32(i) - noise) / f32(steps);
        // (A start offset keeps a surface from shadowing itself.)
        let q = to_screen(wp + p.light.xyz * (t + 0.02 * w0));
        if (q.z <= 0.0 || q.x < 0.0 || q.y < 0.0 || q.x >= p.size.x || q.y >= p.size.y) {
            break;
        }
        let dq = textureLoad(depth, vec2<i32>(q.xy), 0);
        if (dq <= 1e-6) {
            continue;
        }
        // The surface there, as view depth.
        let wq = to_screen(to_world(q.xy, dq).xyz).z;
        let behind = q.z - wq;
        if (behind > 0.01 * q.z && behind < p.zp.z * 0.5) {
            lit = 0.0;
            break;
        }
    }
    return vec4<f32>(vec3<f32>(1.0 - p.zp.w * (1.0 - lit)), 1.0);
}
