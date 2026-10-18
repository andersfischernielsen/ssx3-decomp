// ---------------------------------------------------------------------------
// Type 3 meshes (`mat3::plan_gpu`): q0-3 the screen matrix columns, q4 =
// (lit, n, pass), q5-7 the lighting basis, q8-17 the coefficients, q18 =
// (guard scale z, guard offset z) (clip z from screen z); data
// per vertex: position (int, ITOF15), UV (int, ITOF12), RGBA or normal.

// The program's spherical-harmonic lighting (mat3 `light`).
fn mat3_light(c: u32, n: vec3<f32>) -> vec4<f32> {
    let d = fq(c, 5u) * n.x + fq(c, 6u) * n.y + fq(c, 7u) * n.z;
    let sq = d * d;
    var acc = fq(c, 8u);
    acc += fq(c, 9u) * sq.x;
    acc += fq(c, 10u) * sq.y;
    acc += fq(c, 11u) * sq.z;
    acc += fq(c, 12u) * (d.x * d.y);
    acc += fq(c, 13u) * (d.z * d.x);
    acc += fq(c, 14u) * (d.y * d.z);
    acc += fq(c, 15u) * d.x;
    acc += fq(c, 16u) * d.y;
    acc += fq(c, 17u) * d.z;
    return floor(clamp(acc, vec4<f32>(0.0), vec4<f32>(255.0)));
}

// Pass 2 (q4.z = 2) redraws pass 1's vertices (positions and colours from
// pass 1's data, q30.z) with its own UVs (one qword per vertex).
fn mat3_mesh_vertex(vi: u32, c: u32) -> VOut {
    let pass2 = calls[c].q[4].z == 2u;
    let base = select(calls[c].q[30].y, calls[c].q[30].z, pass2) + 4u * vi;
    let xyz = bitcast<vec4<i32>>(rd(base));
    let uv = bitcast<vec4<i32>>(rd(select(base + 1u, calls[c].q[30].y + vi, pass2)));
    let cw = bitcast<vec4<i32>>(rd(base + 2u));
    let p = vec3<f32>(xyz.xyz) / 32768.0;
    let hc = fq(c, 0u) * p.x + fq(c, 1u) * p.y + fq(c, 2u) * p.z + fq(c, 3u);
    let q = 1.0 / hc.w;
    let t = vec2<f32>(uv.xy) / 4096.0;
    var col: vec4<f32>;
    if (calls[c].q[4].x != 0u) {
        col = mat3_light(c, vec3<f32>(cw.xyz) / 32768.0);
    } else {
        col = vec4<f32>(cw);
    }
    let g = fq(c, 18u);
    let cz = (hc.z - g.y * hc.w) / g.x;
    var o = gs_vertex(c, hc, cz, rd(base + 3u).x != 0u, col, vec3<f32>(t * q, q));
    if (calls[c].q[4].w == 0u) {
        // A mesh without the clip flag: the program draws its projection
        // as is (a vertex behind the eye too, mirrored by the divide), and
        // the GS only scissors. So no clipping here: the screen position.
        o.pos = vec4<f32>(o.pos.xy / o.pos.w, 0.5, 1.0);
        o.zw = vec2<f32>(o.zw.x / o.zw.y, 1.0);
    }
    return o;
}

@vertex
fn vs_mat3_mesh(@builtin(vertex_index) k: u32) -> VOut {
    let c = call_of(k);
    return mat3_mesh_vertex(call_index(k, c), c);
}
