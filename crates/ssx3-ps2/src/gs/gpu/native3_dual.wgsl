
// Dual-source variant of `n3_fs` (needs `enable dual_source_blending;`):
// the second source carries the blend factor As / 128 (0x80 = 1.0), so the
// frame keeps the GS's 8-bit alpha in the first source.
struct N3Out2 {
    @location(0) @blend_src(0) color: vec4<f32>,
    @location(0) @blend_src(1) factor: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn n3_fs2(v: N3V) -> N3Out2 {
    let p = n3_shade(v);
    if (p.kill) {
        discard;
    }
    var o: N3Out2;
    o.color = vec4<f32>(p.rgb, f32(p.c.w)) / 255.0;
    o.factor = vec4<f32>(f32(p.a) / 128.0);
    o.depth = p.depth;
    return o;
}
