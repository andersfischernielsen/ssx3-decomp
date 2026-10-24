// ---------------------------------------------------------------------------
// Type 3 float strips and ribbons (`mat3::plan_gpu_strip`, entries 0x12b8 /
// 0x3a08): q0 guardScale, q1 guardOffset, q2.x the XYZF2 fog of a drawing
// vertex; data per vertex: ST (S, T, Q factor; f32), RGBA (ints), the
// clip-space position (f32, the program's own: the CPU computes it for the
// CLIP test), flags (x: ADC).

fn mat3_strip_vertex(vi: u32, c: u32) -> VOut {
    let base = uq(c, 30).y + 4u * vi;
    let st = bitcast<vec4<f32>>(data[base]);
    let col = vec4<f32>(data[base + 1u] & vec4<u32>(255u));
    let clip = bitcast<vec4<f32>>(data[base + 2u]);
    let adc = data[base + 3u].x != 0u;
    // The screen: guardOffset + guardScale · clip / w, x and y truncated to
    // 1/16 pixel like the program's FTOI4 (the GS's 12.4 positions: HUD
    // quads have edges on pixel centres, and which side of them a pixel
    // falls on is decided by that rounding).
    let q = 1.0 / clip.w;
    let s = fq(c, 1u).xyz + fq(c, 0u).xyz * (clip.xyz * q);
    let hc = vec4<f32>(vec3<f32>(floor(s.xy * 16.0) / 16.0, s.z) * clip.w, clip.w);
    var o = gs_vertex(c, hc, clip.z, adc, col, st.xyz * q);
    // XYZF2: a drawing vertex's fog is FTOI4 of guardOffset.w (not the
    // meshes' 1).
    if (uq(c, 30).w == 4u && !adc) {
        o.fog = fq(c, 2u).x;
    }
    return o;
}

@vertex
fn vs_mat3_strip(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOut {
    cascade = inst;
    let c = call_of(k);
    return mat3_strip_vertex(call_index(k, c), c);
}
