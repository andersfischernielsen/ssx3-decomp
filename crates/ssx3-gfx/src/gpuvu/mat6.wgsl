// ---------------------------------------------------------------------------
// Types 6/7 riders (`mat6::plan_gpu`, layout in `mat6::gpu_skin`).
//
// Skinned draw data, from its base: [0] = (first vertex, count), [1..11)
// the SH coefficients x255, [11] guard scale, [12] guard offset, the bone
// tables, then per vertex {u, v (4.12), bone, rotation (data offsets)},
// normal (1.15; .w: the strip's first two vertices), position (w from
// STROW). Indices are (k << 2) | corner:
// corner 0..2 (vertex k-2+corner) of the strip triangle ending at vertex k.
// The program draws a triangle only when its three vertices pass the CLIP
// test (guard band, z planes): a triangle with one outside is dropped
// whole, here by putting all three corners outside the clip volume.
//
// The env pass (q0.x = 1) redraws its draw's vertices (`Link::Reuse`: the
// draw's data at q30.z) with ST = (E·R) · normal + 0.5 (E·R per vertex:
// offsets from its own base at q0.y + vertex).

fn m6(i: u32) -> vec4<f32> {
    return bitcast<vec4<f32>>(data[i]);
}

// The vertex's guard-clip position: M·B · position.
fn mat6_clip(b: u32, k: u32) -> vec4<f32> {
    let v = b + data[b].x + 3u * k;
    let bo = b + data[v].z;
    let p = m6(v + 2u);
    return m6(bo) * p.x + m6(bo + 1u) * p.y + m6(bo + 2u) * p.z + m6(bo + 3u) * p.w;
}

// CLIP of (clip · q) against its |w|: inside the guard band and z planes.
fn mat6_inside(c: vec4<f32>) -> bool {
    let p = c * (1.0 / c.w);
    return all(abs(p.xyz) <= vec3<f32>(abs(p.w)));
}

// The program's spherical-harmonic lighting (mat6 `Lit::sh`), clamped.
fn mat6_light(b: u32, n: vec3<f32>) -> vec4<f32> {
    let l = b + 1u;
    let sq = n * n;
    var acc = m6(l);
    acc += m6(l + 1u) * sq.x;
    acc += m6(l + 2u) * sq.y;
    acc += m6(l + 3u) * sq.z;
    acc += m6(l + 4u) * (n.x * n.y);
    acc += m6(l + 5u) * (n.z * n.x);
    acc += m6(l + 6u) * (n.y * n.z);
    acc += m6(l + 7u) * n.x;
    acc += m6(l + 8u) * n.y;
    acc += m6(l + 9u) * n.z;
    return floor(clamp(acc, vec4<f32>(0.0), vec4<f32>(255.0)));
}

// The screen x, y as the program's FTOI4 leaves them (1/16 pixel,
// truncated), so the triangles cover the pixels the GS's would.
fn mat6_snap(hc: vec4<f32>) -> vec4<f32> {
    let s = trunc(hc.xy / hc.w * 16.0) / 16.0;
    return vec4<f32>(s * hc.w, hc.zw);
}

// A corner of a dropped triangle (all three land outside x <= w).
fn mat6_culled() -> VOut {
    var o: VOut;
    o.pos = vec4<f32>(2.0, 2.0, 0.5, 1.0);
    return o;
}

fn mat6_skin_vertex(vi: u32, c: u32) -> VOut {
    let env = calls[c].q[0].x != 0u;
    let b = select(calls[c].q[30].y, calls[c].q[30].z, env);
    let k = vi >> 2u;
    let corner = vi & 3u;
    let c0 = mat6_clip(b, k - 2u);
    let c1 = mat6_clip(b, k - 1u);
    let c2 = mat6_clip(b, k);
    if (!(mat6_inside(c0) && mat6_inside(c1) && mat6_inside(c2))) {
        return mat6_culled();
    }
    let v = k - 2u + corner;
    let clip = select(select(c2, c1, corner == 1u), c0, corner == 0u);
    let a = b + data[b].x + 3u * v;
    let uv = data[a];
    let nr = vec3<f32>(bitcast<vec4<i32>>(data[a + 1u]).xyz) / 32768.0;
    let r = b + uv.w;
    let n = m6(r) * nr.x + m6(r + 1u) * nr.y + m6(r + 2u) * nr.z;
    let col = mat6_light(b, n.xyz);
    let q = 1.0 / clip.w;
    var st: vec2<f32>;
    if (env) {
        let eb = calls[c].q[30].y;
        let e = eb + data[eb + calls[c].q[0].y + v].x;
        st = (m6(e) * nr.x + m6(e + 1u) * nr.y + m6(e + 2u) * nr.z).xy + 0.5;
    } else {
        st = vec2<f32>(bitcast<vec2<i32>>(uv.xy)) / 4096.0;
    }
    let scale = m6(b + 11u);
    let off = m6(b + 12u);
    let hc = mat6_snap(vec4<f32>(off.xyz * clip.w + scale.xyz * clip.xyz, clip.w));
    var o = gs_vertex(c, hc, clip.z, false, col, vec3<f32>(st * q, q));
    // XYZF2: the fog is the vertex's ADC word (0x7fff: 0xff; ADC
    // 0xffff8000: 0), ADC when it is one of its strip's first two or one of
    // it and the two before it fails CLIP (the triangle's own three pass).
    if (calls[c].q[30].w == 4u) {
        var adc = data[a + 1u].w != 0u;
        if (!adc && corner < 2u) {
            adc = !mat6_inside(mat6_clip(b, k - 3u));
            if (!adc && corner == 0u) {
                adc = !mat6_inside(mat6_clip(b, k - 4u));
            }
        }
        o.fog = select(255.0, 0.0, adc);
    }
    return o;
}

// Shadow data, from its base: [0] = (first vertex, count), [1] guard
// scale, [2] guard offset, the bone tables, then per vertex {x, y, z (f32),
// bone (data offset; bit 31: the strip's first two vertices)}. No CLIP test in the program; the colour (and Q) are
// the GS registers' (q31).
fn mat6_shadow_vertex(vi: u32, c: u32) -> VOut {
    let b = calls[c].q[30].y;
    let v = b + data[b].x + vi;
    let p = m6(v);
    let bo = b + (data[v].w & 0x7fffffffu);
    let clip = m6(bo) * p.x + m6(bo + 1u) * p.y + m6(bo + 2u) * p.z + m6(bo + 3u);
    let scale = m6(b + 1u);
    let off = m6(b + 2u);
    let hc = mat6_snap(vec4<f32>(off.xyz * clip.w + scale.xyz * clip.xyz, clip.w));
    let g = calls[c].q[31];
    let col = vec4<f32>(f32(g.x & 0xffu), f32((g.x >> 8u) & 0xffu), f32((g.x >> 16u) & 0xffu), f32(g.x >> 24u));
    let gq = bitcast<f32>(g.y);
    let stq = vec3<f32>(bitcast<f32>(g.z), bitcast<f32>(g.w), gq);
    // (No z clip: the program's z goes to the GS as it is.)
    var o = gs_vertex(c, hc, 0.0, false, col, stq);
    // XYZF2: the fog is the vertex's w word (the strip's first two: 0xffff,
    // fog 0xff; the others 0).
    if (calls[c].q[30].w == 4u) {
        o.fog = select(0.0, 255.0, (data[v].w >> 31u) != 0u);
    }
    return o;
}

@vertex
fn vs_mat6_skin(@builtin(vertex_index) k: u32) -> VOut {
    return mat6_skin_vertex(gidx[k], gcall[k]);
}

@vertex
fn vs_mat6_shadow(@builtin(vertex_index) k: u32) -> VOut {
    return mat6_shadow_vertex(gidx[k], gcall[k]);
}
