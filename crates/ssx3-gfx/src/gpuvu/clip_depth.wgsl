// The clip-depth vertices (draw.rs `clip_depth_source`): each program's
// entry point (renamed `vsb_*`) with the GS depth as the position's z and
// the program's z planes as clip distances, so the fragment stage does not
// write depth (which costs the GPU its early depth test: ~1.9 ms a frame at
// 1080p on an M1, Metro City). Needs unclipped depth (the planes are the
// clip distances). The depth is `fs_gpu`'s without its per-pixel rules:
// interpolated by the rasterizer (screen-linear, as the GS's), XYZF2's Z
// not cut to a whole number, ZMAX not clamped, negative Z not wrapped (the
// near plane removes it).

struct VOutClip {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(linear) col: vec4<f32>,
    @location(1) @interpolate(linear) stq: vec3<f32>,
    @location(2) @interpolate(linear) fog: f32,
    @location(3) @interpolate(perspective) zw: vec2<f32>,
    @location(4) @interpolate(flat) zm: vec2<f32>,
    @location(5) @interpolate(perspective) wp: vec4<f32>,
    @location(6) @interpolate(perspective) nrm: vec4<f32>,
    @builtin(clip_distances) cd: array<f32, 2>,
}

fn clip_depth(o: VOut) -> VOutClip {
    var r: VOutClip;
    r.pos = o.pos;
    r.col = o.col;
    r.stq = o.stq;
    r.fog = o.fog;
    r.zw = o.zw;
    r.zm = o.zm;
    r.wp = o.wp;
    r.nrm = o.nrm;
    // (pos.z was (cz + w) / 2: the planes -w <= cz <= w.)
    r.cd[0] = 2.0 * o.pos.z;
    r.cd[1] = 2.0 * (o.pos.w - o.pos.z);
    // Screen z = zw.x / zw.y (zw.y is pos.w): XYZ2 takes it x16, XYZF2 as
    // is; depth = Z / (ZMAX + 1).
    r.pos.z = select(o.zw.x * 16.0, o.zw.x, o.zm.x != 0.0) / (o.zm.y + 1.0);
    return r;
}

@vertex
fn vs_mat3_mesh(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat3_mesh(k, inst));
}

@vertex
fn vs_bezier(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_bezier(k, inst));
}

@vertex
fn vs_mat3_strip(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat3_strip(k, inst));
}

@vertex
fn vs_mat6_skin(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat6_skin(k, inst));
}

@vertex
fn vs_mat6_shadow(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat6_shadow(k, inst));
}

@vertex
fn vs_mat4_particles(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat4_particles(k, inst));
}

@vertex
fn vs_mat4_glitter(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat4_glitter(k, inst));
}

@vertex
fn vs_mat5_snow(@builtin(vertex_index) k: u32, @builtin(instance_index) inst: u32) -> VOutClip {
    return clip_depth(vsb_mat5_snow(k, inst));
}
