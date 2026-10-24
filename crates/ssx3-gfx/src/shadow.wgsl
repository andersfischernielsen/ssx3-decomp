// The shadow pass (shadow.rs): the GPU programs' vertices (gpuvu.wgsl) seen
// from the sun. `gs_vertex` hands each vertex's homogeneous GS screen
// position to `shadow_out`: the terrain camera's screen → world matrix gives
// its world position (relative to a point near the camera, see
// post.rs `cam_matrices`), the light's matrix its place in the shadow map.
// Appended to the GPU-vertex module instead of `NO_SHADOW` (draw.rs).

const SHADOW_PASS: bool = true;

fn shadow_out(o_in: VOut, hc: vec4<f32>) -> VOut {
    var o = o_in;
    let h = sh.s2w * hc;
    let w = h.xyz / h.w;
    o.pos = sh.lights[cascade] * vec4<f32>(w, 1.0);
    return o;
}

// The world's casters (ssx3_render::world_casters): absolute world
// positions and their triangle's upward normal, into cascade `inst` (depth
// only). Only triangles turned away from the sun cast: a ridge, wall or
// kicker casts from its far side, and terrain facing the sun never shadows
// itself (the game's terrain mesh, at its own LOD, differs from this one by
// more than any bias).
@vertex
fn vs_caster(@location(0) p: vec3<f32>, @location(1) n: vec3<f32>, @builtin(instance_index) inst: u32) -> @builtin(position) vec4<f32> {
    if (dot(n, sh.sun.xyz) > 0.0) {
        return vec4<f32>(2.0, 2.0, 2.0, 1.0);
    }
    return sh.lights[inst] * vec4<f32>(p - sh.origin.xyz, 1.0);
}

// Depth only; cut-outs (alpha test, blended texels below half) let the
// light through.
@fragment
fn fs_shadow(v: VOut) {
    let s = shade(v);
    if (b.src_mode != 0u && s.a < 0.5) {
        discard;
    }
}
