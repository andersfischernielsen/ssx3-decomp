// ---------------------------------------------------------------------------
// Types 4 and 5: particles (`mat4::plan_gpu`, `mat5::plan_gpu`; layouts in
// `mat4::gpu_particles`, CPU mirror `mat4::gpu_sprites`). Every program
// kicks SPRITEs {ST a, RGBA, XYZ2 a, ST b, XYZ2 b}: call vertex
// 4·slot + corner is one corner of sprite `slot`, drawn as `Gs::sprite`
// draws it (two triangles over the corners a, (b.x, a.y), b, (a.x, b.y)).

// The program's CLIP test: inside all six planes (|x|, |y|, |z| <= |w|).
fn ptc_inside(v: vec4<f32>) -> bool {
    return all(abs(v.xyz) <= vec3<f32>(abs(v.w)));
}

// The sprite's screen centre (offset + clip / w · scale).
fn ptc_centre(c: u32, cl: vec4<f32>, scale: u32, offset: u32) -> vec3<f32> {
    return fq(c, offset).xyz + cl.xyz * (1.0 / cl.w) * fq(c, scale).xyz;
}

// One corner of the sprite with vertices `a` (first, ST `st.xy`) and `b`
// (second, ST `st.zw`, the colour and Z), positions FTOI4 as the program's.
// Culled sprites go outside the clip volume.
fn ptc_corner(c: u32, corner: u32, a: vec3<f32>, b: vec3<f32>, st: vec4<f32>, rgba: vec4<i32>, visible: bool) -> VOut {
    let xb = corner == 1u || corner == 2u;
    let yb = corner >= 2u;
    let p = trunc(vec3<f32>(select(a.x, b.x, xb), select(a.y, b.y, yb), b.z) * 16.0) / 16.0;
    let tc = vec2<f32>(select(st.x, st.z, xb), select(st.y, st.w, yb));
    let col = vec4<f32>(rgba & vec4<i32>(255));
    var o = gs_vertex(c, vec4<f32>(p, 1.0), 0.0, false, col, vec3<f32>(tc, 1.0));
    if (!visible) {
        o.pos = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    }
    return o;
}

// Emitters / rings: q0-3 M, q4 scale, q5 offset, q6 (fade step, trail dt,
// half max, ring), q7 G, q8/q9 ST corners, q10.x slots per particle; data
// per particle: spawn, G − V, (RGB, alpha), (half.xy, t, -).
fn ptc_sample(c: u32, ring: bool, spawn: vec4<f32>, gmv: vec4<f32>, tk: f32) -> vec4<f32> {
    let tc = min(tk, 2.7);
    let poly = tc * -0.73 + tc * tc * 0.113;
    let p = fq(c, 7u) * tk + spawn + gmv * poly;
    if (ring) {
        return fq(c, 0u) * p.x + fq(c, 1u) * p.y + fq(c, 2u) * p.z + fq(c, 3u);
    }
    return p;
}

fn mat4_particles_vertex(vi: u32, c: u32) -> VOut {
    let slots = calls[c].q[10].x;
    let slot = vi >> 2u;
    let k = slot % slots;
    let base = calls[c].q[30].y + 4u * (slot / slots);
    let spawn = bitcast<vec4<f32>>(data[base]);
    let gmv = bitcast<vec4<f32>>(data[base + 1u]);
    let col = bitcast<vec4<f32>>(data[base + 2u]);
    let misc = bitcast<vec4<f32>>(data[base + 3u]);
    let tr = fq(c, 6u);
    let ring = calls[c].q[6].w != 0u;
    // Rings fade every sample, the emitter only the drawn ones.
    var tk = misc.z;
    var fade = 1.0;
    for (var j = 0u; j < k; j++) {
        if (ring || ptc_inside(ptc_sample(c, ring, spawn, gmv, tk))) {
            fade -= tr.x;
        }
        tk -= tr.y;
    }
    let cl = ptc_sample(c, ring, spawn, gmv, tk);
    let h = vec3<f32>(min(abs(misc.xy * (1.0 / cl.w)), vec2<f32>(tr.z)), 0.0);
    let centre = ptc_centre(c, cl, 4u, 5u);
    let lo = centre - h;
    let hi = centre + h;
    let rgba = vec4<i32>(vec4<f32>(col.xyz, col.w * fade));
    let st = vec4<f32>(fq(c, 8u).xy, fq(c, 9u).xy);
    return ptc_corner(c, vi & 3u, select(lo, hi, ring), select(hi, lo, ring), st, rgba, ptc_inside(cl));
}

// Glitter: q0-15 the patch coefficients, q16-19 M, q20 scale, q21 offset,
// q22 half size, q23 RGB (ints), q24 (half min, half max); data per point:
// (u³, u², u, alpha (int)), (v³, v², v, -).
fn mat4_glitter_vertex(vi: u32, c: u32) -> VOut {
    let base = calls[c].q[30].y + 2u * (vi >> 2u);
    let pa = bitcast<vec4<f32>>(data[base]);
    let pb = bitcast<vec4<f32>>(data[base + 1u]);
    var rows: array<vec4<f32>, 4>;
    for (var k = 0u; k < 4u; k++) {
        rows[k] = fq(c, 4u * k) * pa.x + fq(c, 4u * k + 1u) * pa.y + fq(c, 4u * k + 2u) * pa.z + fq(c, 4u * k + 3u);
    }
    let p = rows[0] * pb.x + rows[1] * pb.y + rows[2] * pb.z + rows[3];
    let cl = fq(c, 16u) * p.x + fq(c, 17u) * p.y + fq(c, 18u) * (p.z + 12.0) + fq(c, 19u);
    let lim = fq(c, 24u);
    let h = vec3<f32>(min(max(abs(fq(c, 22u).xy * (1.0 / cl.w)), vec2<f32>(lim.x)), vec2<f32>(lim.y)), 0.0);
    let centre = ptc_centre(c, cl, 20u, 21u);
    let rgba = vec4<i32>(bitcast<vec4<i32>>(calls[c].q[23]).xyz, bitcast<i32>(data[base].w));
    return ptc_corner(c, vi & 3u, centre + h, centre - h, vec4<f32>(0.0, 0.0, 1.0, 1.0), rgba, ptc_inside(cl));
}

// Snow: q0-3 M, q4 scale, q5 offset, q6 half size, q7 RGBA (ints), q8
// (alpha, depth scale, depth fade, 0.8), q9/q10 ST corners; data per
// flake: (p.xyz, p₁.z − max.z).
fn mat5_snow_vertex(vi: u32, c: u32) -> VOut {
    let p = bitcast<vec4<f32>>(data[calls[c].q[30].y + (vi >> 2u)]);
    let cl = fq(c, 0u) * p.x + fq(c, 1u) * p.y + fq(c, 2u) * p.z + fq(c, 3u);
    let fd = fq(c, 8u);
    let fade = calls[c].q[8].z != 0u;
    var rgba = bitcast<vec4<i32>>(calls[c].q[7]);
    if (fade) {
        // Near (p₁.z − max.z >= d), far (d > 0.8) or neither.
        let d = cl.z * fd.y;
        var f = 1.0;
        if (p.w - d >= 0.0) {
            f = clamp(d * 3.33, 0.0, 1.0);
        } else if (d > fd.w) {
            f = clamp(1.0 - (d - fd.w) * 3.33, 0.0, 1.0);
        }
        rgba.w = i32(fd.x * f);
    }
    let h = fq(c, 6u).xyz * (1.0 / cl.w);
    let centre = ptc_centre(c, cl, 4u, 5u);
    let st = vec4<f32>(fq(c, 9u).xy, fq(c, 10u).xy);
    return ptc_corner(c, vi & 3u, centre + h, centre - h, st, rgba, fade || ptc_inside(cl));
}

@vertex
fn vs_mat4_particles(@builtin(vertex_index) k: u32) -> VOut {
    let c = call_of(k);
    return mat4_particles_vertex(call_index(k, c), c);
}

@vertex
fn vs_mat4_glitter(@builtin(vertex_index) k: u32) -> VOut {
    let c = call_of(k);
    return mat4_glitter_vertex(call_index(k, c), c);
}

@vertex
fn vs_mat5_snow(@builtin(vertex_index) k: u32) -> VOut {
    let c = call_of(k);
    return mat5_snow_vertex(call_index(k, c), c);
}
