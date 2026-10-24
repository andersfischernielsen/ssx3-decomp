// ---------------------------------------------------------------------------
// Types 1 and 2 Bezier terrain strips (`ssx3_render::bezier::gpu`): q0-3 M
// (model → guard-clip space), q4 guardScale, q5 guardOffset, q6 the RGBA
// (integers), q7 = (mode, ST rewrite, vertex count, reuse). Data per strip
// vertex: position, UV (floats); an ST rewrite's own data: UV2 per vertex,
// positions and UVs from the source draw (q30.z).
//
// A triangle is named by its last strip vertex i (vertices i-2 .. i); every
// invocation evaluates the triangle's CLIP flags, so all agree on the
// program's rules. Mode 1 (strips, index (i << 2) | corner): the triangles
// inside the guard band. Mode 2 (fans, index (i << 5) | (3k + corner)): the
// others but the dropped ones (a vertex beyond the far plane, or all three
// outside one plane) go through the program's clipper, run here as it is
// (planes tested against |w|, signed divides), and fan triangle k of its
// polygon is drawn.
//
// The GS draws the program's screen points as they are (no clipping but
// the scissor): positions go out with w > 0 (sign folded in) and a clip z
// of 0, the GS depth goes to `fs_gpu`.

struct BzVert {
    p: vec4<f32>,
    st: vec4<f32>,
}

fn bz_clip(c: u32, src: u32, v: u32) -> vec4<f32> {
    let p = bitcast<vec4<f32>>(data[src + 2u * v]);
    return fq(c, 0u) * p.x + fq(c, 1u) * p.y + fq(c, 2u) * p.z + fq(c, 3u) * p.w;
}

// CLIP judgement bits (+x -x +y -y +z -z) against |w|.
fn bz_flags(h: vec4<f32>) -> u32 {
    let a = abs(h.w);
    return select(0u, 1u, h.x > a) | select(0u, 2u, h.x < -a) | select(0u, 4u, h.y > a) | select(0u, 8u, h.y < -a)
        | select(0u, 16u, h.z > a) | select(0u, 32u, h.z < -a);
}

fn bz_killed() -> VOut {
    var o: VOut;
    o.pos = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    return o;
}

// The vertex the GS gets from the program's clip-space position `h` (screen
// = gO + h·q·gS, q = 1/h.w) with STQ `stq`.
fn bz_out(c: u32, h: vec4<f32>, stq: vec3<f32>, fold: bool) -> VOut {
    let gs = fq(c, 4u);
    let go = fq(c, 5u);
    let hc = vec4<f32>(h.xyz * gs.xyz + go.xyz * h.w, h.w);
    let col = vec4<f32>(bitcast<vec4<i32>>(uq(c, 6)));
    if (!fold) {
        return gs_vertex(c, hc, h.z, false, col, stq);
    }
    return gs_vertex(c, hc * sign(h.w), 0.0, false, col, stq);
}

var<private> bz_poly: array<BzVert, 9>;
var<private> bz_next: array<BzVert, 9>;

// The program's clipper (bezier::clip_triangle) on bz_poly[0..3]: planes
// z-, x-, x+, y+, y- in order; returns the polygon's vertex count (in
// bz_poly). An emitted or "entering" vertex's ST gets z = w = 1, a
// "leaving" intersection keeps the inside vertex's.
fn bz_clipper() -> u32 {
    var n = 3u;
    for (var pl = 0u; pl < 5u; pl++) {
        var l = 1u;
        if (pl == 0u) {
            l = 2u;
        } else if (pl < 3u) {
            l = 0u;
        }
        let minus = pl == 0u || pl == 1u || pl == 4u;
        let bit = 1u << (2u * l + select(0u, 1u, minus));
        let sg = select(1.0, -1.0, minus);
        var m = 0u;
        for (var k = 0u; k < n; k++) {
            let a = bz_poly[(k + n - 1u) % n];
            let b = bz_poly[k];
            let ao = (bz_flags(a.p) & bit) != 0u;
            let bo = (bz_flags(b.p) & bit) != 0u;
            if (ao && bo) {
                continue;
            }
            let b_st1 = vec4<f32>(b.st.xy, 1.0, 1.0);
            if (!ao && !bo) {
                if (m < 9u) {
                    bz_next[m] = BzVert(b.p, b_st1);
                }
                m++;
                continue;
            }
            let d = a.p - b.p;
            let d_st = a.st.xy - b_st1.xy;
            // minus: d(v) = v[l] + v.w, from - d·t; plus: v[l] - v.w, from + d·t.
            let den = d.w - sg * d[l];
            if (ao) {
                let t = (b.p[l] - sg * b.p.w) / den;
                if (m < 9u) {
                    bz_next[m] = BzVert(b.p + sg * d * t, vec4<f32>(b_st1.xy + sg * d_st * t, 1.0, 1.0));
                }
                if (m + 1u < 9u) {
                    bz_next[m + 1u] = BzVert(b.p, b_st1);
                }
                m += 2u;
            } else {
                let t = (a.p[l] - sg * a.p.w) / den;
                if (m < 9u) {
                    bz_next[m] = BzVert(a.p + sg * d * t, vec4<f32>(a.st.xy + sg * d_st * t, a.st.zw));
                }
                m++;
            }
        }
        n = min(m, 9u);
        for (var k = 0u; k < n; k++) {
            bz_poly[k] = bz_next[k];
        }
        if (n == 0u) {
            break;
        }
    }
    return n;
}

fn bezier_vertex(vi: u32, c: u32) -> VOut {
    let info = uq(c, 7);
    let fans = info.x == 2u;
    let i = select(vi >> 2u, vi >> 5u, fans);
    let r = select(vi & 3u, vi & 31u, fans);
    let src = select(uq(c, 30).y, uq(c, 30).z, info.w != 0u);
    let h0 = bz_clip(c, src, i - 2u);
    let h1 = bz_clip(c, src, i - 1u);
    let h2 = bz_clip(c, src, i);
    let f0 = bz_flags(h0);
    let f1 = bz_flags(h1);
    let f2 = bz_flags(h2);
    let fa = f0 | f1 | f2;
    if (!fans) {
        if (fa != 0u) {
            return bz_killed();
        }
        let h = select(select(h0, h1, r == 1u), h2, r == 2u);
        let v = i - 2u + r;
        let q = 1.0 / h.w;
        let uv = bitcast<vec4<f32>>(data[src + 2u * v + 1u]);
        var stq = uv.xyz * q;
        if (info.y != 0u) {
            // Pass 2 "flag 0": ST := UV2 · STQ.w (Q stays).
            let uv2 = bitcast<vec4<f32>>(data[uq(c, 30).y + v]);
            stq = vec3<f32>(uv2.xy * (uv.w * q), uv.z * q);
        }
        return bz_out(c, h, stq, false);
    }
    let dropped = (fa & 16u) != 0u || (f0 & f1 & f2) != 0u;
    if (fa == 0u || dropped) {
        return bz_killed();
    }
    let k = r / 3u;
    let j = r % 3u;
    bz_poly[0] = BzVert(h0, bitcast<vec4<f32>>(data[src + 2u * (i - 2u) + 1u]));
    bz_poly[1] = BzVert(h1, bitcast<vec4<f32>>(data[src + 2u * (i - 1u) + 1u]));
    bz_poly[2] = BzVert(h2, bitcast<vec4<f32>>(data[src + 2u * i + 1u]));
    if (h0.w > 0.0 && h1.w > 0.0 && h2.w > 0.0) {
        // In front of the eye the program's planes are the true ones: the
        // GPU clips the triangle itself (numerically closer to the
        // program's than the clipper in IEEE floats), x/y at the viewport
        // (inside the guard band), z at the near plane like the z- plane.
        if (k != 0u) {
            return bz_killed();
        }
        let p = bz_poly[j];
        let q = 1.0 / p.p.w;
        return bz_out(c, p.p, vec3<f32>(p.st.xy * q, q), false);
    }
    // Fan triangle k = (v0, vk+1, vk+2) of the program's polygon.
    let n = bz_clipper();
    if (k + 2u >= n) {
        return bz_killed();
    }
    let p = bz_poly[select(k + j, 0u, j == 0u)];
    let q = 1.0 / p.p.w;
    return bz_out(c, p.p, p.st.xyz * q, true);
}

@vertex
fn vs_bezier(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOut {
    cascade = inst;
    let c = call_of(k);
    return bezier_vertex(call_index(k, c), c);
}
