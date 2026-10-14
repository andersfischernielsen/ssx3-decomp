// GS rasterizer on the GPU (compute).
//
// The design follows ps2-web-recomp's WebGPU GS (runtime/src/gs/gpu/gs.wgsl,
// MIT, Copyright (c) 2026 Samir Nuri, https://github.com/snuri00/ps2-web-recomp):
// the 4 MiB of GS local memory live in a storage buffer in the native
// swizzled layout, primitives are binned on the CPU into pixel tiles (8x8 here; the reference uses 16x16),
// and one workgroup per tile walks the tile's primitives in submission order
// with one thread per pixel, so every pixel sees the primitives in the
// original order. Sub-word writes use atomics; texture reads come from a
// copy of the batch's texture pages taken before the batch (`snap`).
//
// The per-pixel semantics follow the CPU rasterizer (`gs/raster.rs`), which
// stays the reference: integer edge functions with a top-left rule, sprite
// sampling, TEX1 filtering and mipmapping, TFX, fog, alpha test, DATE,
// depth test, blending, dithering, COLCLAMP, FBA and FBMSK.
// The CPU sets primitives up (`raster::setup`) and encodes the result; see
// `gpu/mod.rs` for the record layouts.

struct Params {
    a: vec4<u32>,
    b: vec4<u32>,
    c: vec4<u32>,
    d: vec4<u32>,
}

@group(0) @binding(0) var<storage, read_write> vram: array<atomic<u32>>;
@group(0) @binding(1) var<storage, read> tables: array<u32>;
@group(0) @binding(2) var<storage, read> data: array<u32>;
@group(0) @binding(3) var<uniform> params: Params;
@group(0) @binding(4) var<storage, read> snap: array<u32>;
@group(0) @binding(5) var<storage, read_write> outbuf: array<u32>;
@group(0) @binding(6) var<storage, read_write> decoded: array<u32>;

// Setup record (per draw state), word offsets.
const S_FLAGS: u32 = 0u;
const S_FB: u32 = 1u;
const S_FBMSK: u32 = 4u;
const S_ZB: u32 = 5u;
const S_ZMAX: u32 = 8u;
const S_MODES: u32 = 9u;
const S_REFS: u32 = 10u;
const S_SCX: u32 = 11u;
const S_SCY: u32 = 12u;
const S_FOGCOL: u32 = 13u;
const S_TEXP: u32 = 14u;
const S_UCLAMP: u32 = 15u;
const S_VCLAMP: u32 = 16u;
const S_K: u32 = 17u;
const S_PAL: u32 = 18u;
const S_DIMX: u32 = 19u;
const S_TEX: u32 = 21u;
// Per level: decoded offset (texels; 0xFFFFFFFF = none), u0 | v0 << 16, width.
const S_DEC: u32 = 42u;

const F_TME: u32 = 1u;
const F_FGE: u32 = 2u;
const F_ABE: u32 = 4u;
const F_FST: u32 = 8u;
const F_ZTE: u32 = 16u;
const F_ZUSED: u32 = 32u;
const F_ZMSK: u32 = 64u;
const F_ATE: u32 = 128u;
const F_DATE: u32 = 256u;
const F_DATM: u32 = 512u;
const F_PABE: u32 = 1024u;
const F_COLCLAMP: u32 = 2048u;
const F_FBA: u32 = 4096u;
const F_TCC: u32 = 8192u;
const F_AEM: u32 = 16384u;
const F_MIP: u32 = 32768u;
const F_DITHER: u32 = 65536u;
const F_MAGLIN: u32 = 131072u;
const F_LCM: u32 = 524288u;

// Primitive record.
const P_SETUP: u32 = 0u;
const P_KIND: u32 = 1u;
const P_LO: u32 = 2u;
const P_HI: u32 = 3u;

const PK_LINEAR: u32 = 256u;
const PK_BIG: u32 = 512u;

// ------------------------------------------------------------------ memory

// A surface: [base (words), packed, pages per row], where
// packed = table offset | pw_shift << 16 | ph_shift << 19 | es << 22 | kind << 24.
struct Surf {
    base: u32,
    toff: u32,
    pws: u32,
    phs: u32,
    es: u32,
    kind: u32,
    ppr: u32,
}

const KIND_W32: u32 = 0u;
const KIND_W24: u32 = 1u;

fn surf_at(at: u32) -> Surf {
    let p = data[at + 1u];
    return Surf(data[at], p & 0xFFFFu, (p >> 16u) & 7u, (p >> 19u) & 7u, (p >> 22u) & 3u, (p >> 24u) & 7u, data[at + 2u]);
}

struct Loc {
    word: u32,
    shift: u32,
}

fn locate(s: Surf, xi: u32, yi: u32) -> Loc {
    let x = xi & 0x7FFu;
    let y = yi & 0x7FFu;
    let page = (x >> s.pws) + (y >> s.phs) * s.ppr;
    let off = tables[s.toff + (((y & ((1u << s.phs) - 1u)) << s.pws) | (x & ((1u << s.pws) - 1u)))];
    let wb = s.base + page * 2048u;
    let e = ((wb << s.es) + off) & ((0x100000u << s.es) - 1u);
    let word = e >> s.es;
    let shift = (e & ((1u << s.es) - 1u)) << (5u - s.es);
    return Loc(word, shift);
}

fn extract(kind: u32, v: u32, sh: u32) -> u32 {
    switch kind {
        case 0u: { return v; }
        case 1u: { return v & 0xFFFFFFu; }
        case 2u: { return (v >> sh) & 0xFFFFu; }
        case 3u: { return (v >> sh) & 0xFFu; }
        case 4u: { return (v >> sh) & 0xFu; }
        case 5u: { return v >> 24u; }
        case 6u: { return (v >> 24u) & 0xFu; }
        default: { return v >> 28u; }
    }
}

fn read_at(s: Surf, l: Loc) -> u32 {
    return extract(s.kind, atomicLoad(&vram[l.word]), l.shift);
}

// Bits of the word a format element occupies.
fn elem_mask(kind: u32, sh: u32) -> u32 {
    switch kind {
        case 0u: { return 0xFFFFFFFFu; }
        case 1u: { return 0xFFFFFFu; }
        case 2u: { return 0xFFFFu << sh; }
        case 3u: { return 0xFFu << sh; }
        case 4u: { return 0xFu << sh; }
        case 5u: { return 0xFF000000u; }
        case 6u: { return 0x0F000000u; }
        default: { return 0xF0000000u; }
    }
}

fn elem_shift(kind: u32, sh: u32) -> u32 {
    switch kind {
        case 2u, 3u, 4u: { return sh; }
        case 5u, 6u: { return 24u; }
        case 7u: { return 28u; }
        default: { return 0u; }
    }
}

fn write_at(s: Surf, l: Loc, value: u32) {
    if (s.kind == KIND_W32) {
        atomicStore(&vram[l.word], value);
        return;
    }
    let m = elem_mask(s.kind, l.shift);
    let v = (value << elem_shift(s.kind, l.shift)) & m;
    atomicAnd(&vram[l.word], ~m);
    atomicOr(&vram[l.word], v);
}

fn write_masked_at(s: Surf, l: Loc, value: u32, keep: u32) {
    if (keep == 0u) {
        write_at(s, l, value);
    } else {
        let old = read_at(s, l);
        write_at(s, l, (value & ~keep) | (old & keep));
    }
}

// Texture reads come from the copy of the batch's texture pages taken
// before the batch (`params.a.z..a.w`, words); see `gpu/mod.rs`.
fn tex_read(s: Surf, x: u32, y: u32) -> u32 {
    let l = locate(s, x, y);
    var v: u32;
    if (l.word >= params.a.z && l.word < params.a.w) {
        v = snap[l.word];
    } else {
        v = atomicLoad(&vram[l.word]);
    }
    return extract(s.kind, v, l.shift);
}

// ------------------------------------------------------------------ texturing

struct Tex {
    surf: Surf,
    tw: i32,
    th: i32,
    minu: i32,
    maxu: i32,
    minv: i32,
    maxv: i32,
    dec: u32,
    du0: u32,
    dv0: u32,
    dw: u32,
}

fn tex_level(st: u32, l: u32) -> Tex {
    var t: Tex;
    var dec: vec3<u32>;
    if (l == 0u) {
        t.surf = surf_at(st + S_TEX);
        dec = vec3<u32>(data[st + S_DEC], data[st + S_DEC + 1u], data[st + S_DEC + 2u]);
    } else {
        t.surf = surf_at(st + S_TEX + 3u * l);
        let a = st + S_DEC + 3u * l;
        dec = vec3<u32>(data[a], data[a + 1u], data[a + 2u]);
    }
    t.dec = dec.x;
    t.du0 = dec.y & 0xFFFFu;
    t.dv0 = dec.y >> 16u;
    t.dw = dec.z;
    t.tw = max(i32(data[st + S_TEXP] & 15u) - i32(l), 0);
    t.th = max(i32((data[st + S_TEXP] >> 4u) & 15u) - i32(l), 0);
    t.minu = i32(data[st + S_UCLAMP] & 0xFFFFu) >> l;
    t.maxu = i32(data[st + S_UCLAMP] >> 16u) >> l;
    t.minv = i32(data[st + S_VCLAMP] & 0xFFFFu) >> l;
    t.maxv = i32(data[st + S_VCLAMP] >> 16u) >> l;
    return t;
}

fn wrapc(c: i32, mode: u32, size_log: i32, mn: i32, mx: i32) -> u32 {
    switch mode {
        case 0u: { return u32(c & ((1 << u32(size_log)) - 1)); }
        case 1u: { return u32(clamp(c, 0, (1 << u32(size_log)) - 1)); }
        case 2u: { return u32(clamp(c, mn, max(mx, mn))); }
        default: { return u32((c & mn) | mx); }
    }
}

fn expand16(c: u32, ta0: u32, ta1: u32, aem: bool) -> u32 {
    let r = (c & 0x1Fu) << 3u;
    let g = ((c >> 5u) & 0x1Fu) << 3u;
    let b = ((c >> 10u) & 0x1Fu) << 3u;
    var a = ta0;
    if ((c & 0x8000u) != 0u) {
        a = ta1;
    } else if (aem && (c & 0x7FFFu) == 0u) {
        a = 0u;
    }
    return r | (g << 8u) | (b << 16u) | (a << 24u);
}

// Raw texel to RGBA8 (TEXA expansion, palette lookup).
fn conv(st: u32, raw: u32) -> u32 {
    let ta0 = (data[st + S_REFS] >> 16u) & 0xFFu;
    let ta1 = data[st + S_REFS] >> 24u;
    let aem = (data[st + S_FLAGS] & F_AEM) != 0u;
    switch (data[st + S_MODES] >> 25u) & 3u {
        case 0u: { return raw; }
        case 1u: {
            var a = ta0;
            if (aem && raw == 0u) {
                a = 0u;
            }
            return raw | (a << 24u);
        }
        case 2u: { return expand16(raw, ta0, ta1, aem); }
        default: {
            var pm = 15u;
            if (((data[st + S_MODES] >> 27u) & 1u) != 0u) {
                pm = 255u;
            }
            return data[data[st + S_PAL] + (raw & pm)];
        }
    }
}

// Texel (u, v) (already wrapped) as RGBA8: from the decoded copy if the
// level has one, else from (the snapshot of) VRAM through the swizzle.
fn texel(st: u32, t: Tex, u: u32, v: u32) -> u32 {
    if (t.dec != 0xFFFFFFFFu) {
        return decoded[t.dec + (v - t.dv0) * t.dw + (u - t.du0)];
    }
    return conv(st, tex_read(t.surf, u, v));
}

fn unpack4(c: u32) -> vec4<u32> {
    return vec4<u32>(c & 0xFFu, (c >> 8u) & 0xFFu, (c >> 16u) & 0xFFu, c >> 24u);
}

fn pack4(v: vec4<u32>) -> u32 {
    return v.x | (v.y << 8u) | (v.z << 16u) | (v.w << 24u);
}

// One texture level, nearest or bilinear (raster.rs `sample` / `bilerp`):
// level `l` is sampled at (u, v) / 2^l. The bilinear result is
// (sum of texel * weight + 0x8000) >> 16 with 8-bit weights per axis,
// exactly the CPU's `bilerp`; zero-weight texels are not fetched. Written
// as loops with a single `texel` call site: inlining every sampling path
// separately made the kernel big enough to hurt occupancy everywhere.
fn sample(st: u32, l: u32, uin: f32, vin: f32, linear: bool) -> u32 {
    let t = tex_level(st, l);
    let modes = data[st + S_MODES];
    let wms = (modes >> 17u) & 3u;
    let wmt = (modes >> 19u) & 3u;
    let scale = 1.0 / f32(1u << l);
    let u = uin * scale;
    let v = vin * scale;
    if (!linear) {
        return texel(st, t, wrapc(i32(floor(u)), wms, t.tw, t.minu, t.maxu), wrapc(i32(floor(v)), wmt, t.th, t.minv, t.maxv));
    }
    let iu = i32(floor((u - 0.5) * 256.0));
    let iv = i32(floor((v - 0.5) * 256.0));
    let x0 = iu >> 8u;
    let y0 = iv >> 8u;
    let fx = u32(iu & 255);
    let fy = u32(iv & 255);
    let xa = wrapc(x0, wms, t.tw, t.minu, t.maxu);
    let xb = wrapc(x0 + 1, wms, t.tw, t.minu, t.maxu);
    let ya = wrapc(y0, wmt, t.th, t.minv, t.maxv);
    let yb = wrapc(y0 + 1, wmt, t.th, t.minv, t.maxv);
    // Products and sums stay below 2^24, so f32 is exact here.
    var acc = vec4<f32>(32768.0);
    for (var k = 0u; k < 4u; k = k + 1u) {
        let dx = (k & 1u) == 1u;
        let dy = (k >> 1u) == 1u;
        let w = select(256u - fx, fx, dx) * select(256u - fy, fy, dy);
        if (w == 0u) {
            continue;
        }
        acc = acc + vec4<f32>(unpack4(texel(st, t, select(xa, xb, dx), select(ya, yb, dy)))) * f32(w);
    }
    return pack4(vec4<u32>(acc) >> vec4<u32>(16u));
}

// Texturing with TEX1 filtering and mipmapping (raster.rs `sample_mip`).
// LOD is K (LCM=1) or log2(1/|Q|) * 2^L + K. LOD <= 0 magnifies level 0
// with MMAG; otherwise MMIN picks the nearest level (2, 4) or blends the two
// nearest (3, 5), with nearest (2, 3) or bilinear (4, 5) sampling in a
// level and LOD clamped to MXL. Without mipmapping, `linear` (chosen per
// primitive) applies to level 0.
fn sample_tex(st: u32, u: f32, v: f32, qin: f32, linear: bool) -> u32 {
    let flags = data[st + S_FLAGS];
    var l0 = 0u;
    var w = 0u;
    var lin = linear;
    if ((flags & F_MIP) != 0u) {
        let texp = data[st + S_TEXP];
        let k = bitcast<f32>(data[st + S_K]);
        let mxl = (texp >> 12u) & 7u;
        let mmin = (texp >> 16u) & 7u;
        var lod: f32;
        if ((flags & F_LCM) != 0u) {
            lod = k;
        } else {
            var q = abs(qin);
            if (!(q > 0.0)) {
                q = 1.0;
            }
            lod = log2(1.0 / q) * f32(1u << ((texp >> 8u) & 3u)) + k;
        }
        if (!(lod > 0.0)) {
            lin = (flags & F_MAGLIN) != 0u;
        } else {
            lin = mmin >= 4u;
            lod = min(lod, f32(mxl));
            if ((mmin & 1u) == 0u) {
                l0 = min(u32(lod + 0.5), mxl);
            } else {
                l0 = u32(lod);
                if (l0 < mxl) {
                    w = u32((lod - f32(l0)) * 256.0);
                }
            }
        }
    }
    let n = select(1u, 2u, w != 0u);
    var acc = vec4<u32>(128u);
    for (var i = 0u; i < n; i = i + 1u) {
        let c = unpack4(sample(st, l0 + i, u, v, lin));
        var wi = 256u;
        if (n == 2u) {
            wi = select(256u - w, w, i == 1u);
        }
        acc = acc + c * wi;
    }
    return pack4(acc >> vec4<u32>(8u));
}

// ------------------------------------------------------------------ pixel pipeline

struct Frag {
    r: i32,
    g: i32,
    b: i32,
    a: i32,
    u: f32,
    v: f32,
    q: f32,
    f: i32,
}

fn read_fb(fmt: u32, fb: Surf, l: Loc) -> u32 {
    let c = read_at(fb, l);
    switch fmt {
        case 1u: { return c | 0x80000000u; }
        case 2u: {
            var a = 0u;
            if ((c & 0x8000u) != 0u) {
                a = 0x80000000u;
            }
            return ((c & 0x1Fu) << 3u) | (((c >> 5u) & 0x1Fu) << 11u) | (((c >> 10u) & 0x1Fu) << 19u) | a;
        }
        default: { return c; }
    }
}

fn pack16(v: u32) -> u32 {
    return ((v >> 3u) & 0x1Fu) | (((v >> 11u) & 0x1Fu) << 5u) | (((v >> 19u) & 0x1Fu) << 10u) | ((v >> 16u) & 0x8000u);
}

fn write_fb(fmt: u32, fb: Surf, l: Loc, c: u32, keep: u32) {
    switch fmt {
        case 1u: { write_masked_at(fb, l, c & 0xFFFFFFu, keep & 0xFFFFFFu); }
        case 2u: { write_masked_at(fb, l, pack16(c), pack16(keep)); }
        default: { write_masked_at(fb, l, c, keep); }
    }
}

// The pixel pipeline (raster.rs `pixel_g`) for pixel (x, y) inside the
// scissor. `z` is already clamped. The fragment of primitive `p` is only
// computed for pixels passing the early (destination alpha, depth) tests.
fn pixel(st: u32, x: i32, y: i32, z: u32, p: u32, linear: bool) {
    let flags = data[st + S_FLAGS];
    let modes = data[st + S_MODES];
    let fmt = (modes >> 23u) & 3u;
    let xu = u32(x);
    let yu = u32(y);
    let fl = locate(surf_at(st + S_FB), xu, yu);
    var dst = 0u;
    var have_dst = false;
    if ((flags & F_DATE) != 0u) {
        let c = read_fb(fmt, surf_at(st + S_FB), fl);
        if (((c >> 31u) != 0u) != ((flags & F_DATM) != 0u)) {
            return;
        }
        dst = c;
        have_dst = true;
    }
    var zl = Loc(0u, 0u);
    let ztst = modes & 3u;
    if ((flags & F_ZTE) != 0u) {
        if (ztst == 0u) {
            return;
        }
        if ((flags & F_ZUSED) != 0u) {
            zl = locate(surf_at(st + S_ZB), xu, yu);
            if (ztst == 2u) {
                if (z < read_at(surf_at(st + S_ZB), zl)) {
                    return;
                }
            } else if (ztst == 3u) {
                if (z <= read_at(surf_at(st + S_ZB), zl)) {
                    return;
                }
            }
        }
    }
    let f = frag(p, st, x, y);
    var r = f.r;
    var g = f.g;
    var b = f.b;
    var a = f.a;

    if ((flags & F_TME) != 0u) {
        let c = sample_tex(st, f.u, f.v, f.q, linear);
        let tr = i32(c & 0xFFu);
        let tg = i32((c >> 8u) & 0xFFu);
        let tb = i32((c >> 16u) & 0xFFu);
        let ta = i32(c >> 24u);
        let tcc = (flags & F_TCC) != 0u;
        switch (modes >> 15u) & 3u {
            case 0u: {
                r = min((tr * r) >> 7u, 255);
                g = min((tg * g) >> 7u, 255);
                b = min((tb * b) >> 7u, 255);
                if (tcc) {
                    a = min((ta * a) >> 7u, 255);
                }
            }
            case 1u: {
                r = tr;
                g = tg;
                b = tb;
                if (tcc) {
                    a = ta;
                }
            }
            case 2u: {
                r = min(((tr * r) >> 7u) + a, 255);
                g = min(((tg * g) >> 7u) + a, 255);
                b = min(((tb * b) >> 7u) + a, 255);
                if (tcc) {
                    a = min(ta + a, 255);
                }
            }
            default: {
                r = min(((tr * r) >> 7u) + a, 255);
                g = min(((tg * g) >> 7u) + a, 255);
                b = min(((tb * b) >> 7u) + a, 255);
                if (tcc) {
                    a = ta;
                }
            }
        }
    }

    if ((flags & F_FGE) != 0u) {
        let fv = clamp(f.f, 0, 255);
        let fc = data[st + S_FOGCOL];
        r = (fv * r + (255 - fv) * i32(fc & 0xFFu)) >> 8u;
        g = (fv * g + (255 - fv) * i32((fc >> 8u) & 0xFFu)) >> 8u;
        b = (fv * b + (255 - fv) * i32((fc >> 16u) & 0xFFu)) >> 8u;
    }

    var write_fb_ok = true;
    var write_z = (flags & F_ZTE) != 0u && (flags & F_ZMSK) == 0u;
    var keep = data[st + S_FBMSK];
    if ((flags & F_ATE) != 0u) {
        let aref = i32(data[st + S_REFS] & 0xFFu);
        var ok = false;
        switch (modes >> 2u) & 7u {
            case 0u: { ok = false; }
            case 1u: { ok = true; }
            case 2u: { ok = a < aref; }
            case 3u: { ok = a <= aref; }
            case 4u: { ok = a == aref; }
            case 5u: { ok = a >= aref; }
            case 6u: { ok = a > aref; }
            default: { ok = a != aref; }
        }
        if (!ok) {
            switch (modes >> 5u) & 3u {
                case 0u: { return; }
                case 1u: { write_z = false; }
                case 2u: { write_fb_ok = false; }
                default: {
                    write_z = false;
                    keep = keep | 0xFF000000u;
                }
            }
        }
    }

    if ((flags & F_ZUSED) != 0u && write_z) {
        if (ztst < 2u) {
            zl = locate(surf_at(st + S_ZB), xu, yu);
        }
        write_at(surf_at(st + S_ZB), zl, z);
    }

    if (!write_fb_ok || keep == 0xFFFFFFFFu) {
        return;
    }
    if ((flags & F_ABE) != 0u && ((flags & F_PABE) == 0u || a >= 0x80)) {
        var dc = dst;
        if (!have_dst) {
            dc = read_fb(fmt, surf_at(st + S_FB), fl);
        }
        let cd = vec3<i32>(i32(dc & 0xFFu), i32((dc >> 8u) & 0xFFu), i32((dc >> 16u) & 0xFFu));
        let cs = vec3<i32>(r, g, b);
        let ad = i32(dc >> 24u);
        let ba = (modes >> 7u) & 3u;
        let bb = (modes >> 9u) & 3u;
        let bc = (modes >> 11u) & 3u;
        let bd = (modes >> 13u) & 3u;
        var cc = i32((data[st + S_REFS] >> 8u) & 0xFFu);
        if (bc == 0u) {
            cc = a;
        } else if (bc == 1u) {
            cc = ad;
        }
        var av = vec3<i32>(0);
        var bv = vec3<i32>(0);
        var dv = vec3<i32>(0);
        if (ba == 0u) { av = cs; } else if (ba == 1u) { av = cd; }
        if (bb == 0u) { bv = cs; } else if (bb == 1u) { bv = cd; }
        if (bd == 0u) { dv = cs; } else if (bd == 1u) { dv = cd; }
        let o = (((av - bv) * cc) >> vec3<u32>(7u)) + dv;
        r = o.x;
        g = o.y;
        b = o.z;
    }
    if ((flags & F_DITHER) != 0u) {
        let idx = u32(y & 3) * 16u + u32(x & 3) * 4u;
        var bits: u32;
        if (idx >= 32u) {
            bits = (data[st + S_DIMX + 1u] >> (idx - 32u)) & 7u;
        } else {
            bits = (data[st + S_DIMX] >> idx) & 7u;
        }
        var dv = i32(bits);
        if ((bits & 4u) != 0u) {
            dv = dv - 8;
        }
        r = r + dv;
        g = g + dv;
        b = b + dv;
    }
    if ((flags & F_COLCLAMP) != 0u) {
        r = clamp(r, 0, 255);
        g = clamp(g, 0, 255);
        b = clamp(b, 0, 255);
    } else {
        r = r & 0xFF;
        g = g & 0xFF;
        b = b & 0xFF;
    }
    var oa = u32(a & 0xFF);
    if ((flags & F_FBA) != 0u) {
        oa = oa | 0x80u;
    }
    let c = u32(r) | (u32(g) << 8u) | (u32(b) << 16u) | (oa << 24u);
    write_fb(fmt, surf_at(st + S_FB), fl, c, keep);
}

fn fat(i: u32) -> f32 {
    return bitcast<f32>(data[i]);
}

// Texture coordinates from interpolated S, T, Q (raster.rs `tex_uv`).
fn tex_uv(st: u32, ss: f32, tt: f32, qin: f32) -> vec2<f32> {
    if ((data[st + S_FLAGS] & F_TME) == 0u) {
        return vec2<f32>(0.0);
    }
    if ((data[st + S_FLAGS] & F_FST) != 0u) {
        return vec2<f32>(ss / 16.0, tt / 16.0);
    }
    var q = qin;
    if (q == 0.0) {
        q = 1.17549435e-38;
    }
    return vec2<f32>(ss / q * f32(1u << (data[st + S_TEXP] & 15u)), tt / q * f32(1u << ((data[st + S_TEXP] >> 4u) & 15u)));
}

fn unpack_rgba(c: u32) -> vec4<i32> {
    return vec4<i32>(i32(c & 0xFFu), i32((c >> 8u) & 0xFFu), i32((c >> 16u) & 0xFFu), i32(c >> 24u));
}

var<private> line_t: f32 = 0.0;

// The fragment of primitive `p` at pixel (x, y).
fn frag(p: u32, st: u32, x: i32, y: i32) -> Frag {
    var f: Frag;
    let tme = (data[st + S_FLAGS] & F_TME) != 0u;
    switch data[p + P_KIND] & 0xFFu {
        case 0u: {
            let lo = data[p + P_LO];
            let i = f32(x - i32(lo & 0xFFFFu));
            let j = f32(y - i32(lo >> 16u));
            f.r = i32(fat(p + 13u) + fat(p + 21u) * i + fat(p + 29u) * j);
            f.g = i32(fat(p + 14u) + fat(p + 22u) * i + fat(p + 30u) * j);
            f.b = i32(fat(p + 15u) + fat(p + 23u) * i + fat(p + 31u) * j);
            f.a = i32(fat(p + 16u) + fat(p + 24u) * i + fat(p + 32u) * j);
            if (tme) {
                let q = fat(p + 19u) + fat(p + 27u) * i + fat(p + 35u) * j;
                let uv = tex_uv(st, fat(p + 17u) + fat(p + 25u) * i + fat(p + 33u) * j, fat(p + 18u) + fat(p + 26u) * i + fat(p + 34u) * j, q);
                f.u = uv.x;
                f.v = uv.y;
                f.q = q;
            }
            if ((data[st + S_FLAGS] & F_FGE) != 0u) {
                f.f = i32(fat(p + 20u) + fat(p + 28u) * i + fat(p + 36u) * j);
            }
        }
        case 1u: {
            let c = unpack_rgba(data[p + 11u]);
            f.r = c.x;
            f.g = c.y;
            f.b = c.z;
            f.a = c.w;
            f.f = i32(data[p + 12u]);
            if (tme) {
                let q = fat(p + 10u);
                f.q = q;
                let ss = fat(p + 6u) + fat(p + 8u) * (f32(x * 16 - bitcast<i32>(data[p + 4u])) / 16.0);
                let tt = fat(p + 7u) + fat(p + 9u) * (f32(y * 16 - bitcast<i32>(data[p + 5u])) / 16.0);
                let uv = tex_uv(st, ss, tt, q);
                f.u = uv.x;
                f.v = uv.y;
            }
        }
        case 2u: {
            let tt = line_t;
            f.r = i32(mix(fat(p + 9u), fat(p + 17u), tt));
            f.g = i32(mix(fat(p + 10u), fat(p + 18u), tt));
            f.b = i32(mix(fat(p + 11u), fat(p + 19u), tt));
            f.a = i32(mix(fat(p + 12u), fat(p + 20u), tt));
            let q = mix(fat(p + 15u), fat(p + 23u), tt);
            let uv = tex_uv(st, mix(fat(p + 13u), fat(p + 21u), tt), mix(fat(p + 14u), fat(p + 22u), tt), q);
            f.u = uv.x;
            f.v = uv.y;
            f.q = q;
            f.f = i32(mix(fat(p + 16u), fat(p + 24u), tt));
        }
        default: {
            let c = unpack_rgba(data[p + 6u]);
            f.r = c.x;
            f.g = c.y;
            f.b = c.z;
            f.a = c.w;
            f.u = fat(p + 7u);
            f.v = fat(p + 8u);
            f.q = fat(p + 9u);
            f.f = i32(data[p + 10u]);
        }
    }
    return f;
}

// Rust's f64::round: halves away from zero.
fn round_away(v: f32) -> f32 {
    return sign(v) * floor(abs(v) + 0.5);
}

// z0 + floor(d), clamped to [0, zmax].
fn z_add(z0: u32, d: f32, zmax: u32) -> u32 {
    let fd = floor(d);
    var z: u32;
    if (fd >= 0.0) {
        if (fd >= 4294967295.0) {
            z = 0xFFFFFFFFu;
        } else {
            z = z0 + u32(fd);
            if (z < z0) {
                z = 0xFFFFFFFFu;
            }
        }
    } else {
        let nd = -fd;
        if (nd >= 4294967295.0 || u32(nd) >= z0) {
            z = 0u;
        } else {
            z = z0 - u32(nd);
        }
    }
    return min(z, zmax);
}

fn in_scissor(st: u32, x: i32, y: i32) -> bool {
    if (x < i32(data[st + S_SCX] & 0xFFFFu) || x > i32(data[st + S_SCX] >> 16u) || y < i32(data[st + S_SCY] & 0xFFFFu) || y > i32(data[st + S_SCY] >> 16u)) {
        return false;
    }
    let scan = (data[st + S_MODES] >> 21u) & 3u;
    return !((scan == 2u && (y & 1) == 0) || (scan == 3u && (y & 1) != 0));
}

// Draw primitive `p` at pixel (x, y), if it covers it. The bounding box
// (inside the scissor for triangles and sprites) was checked by the caller.
fn draw_prim(p: u32, x: i32, y: i32) {
    let kind = data[p + P_KIND];
    let linear = (kind & PK_LINEAR) != 0u;
    switch kind & 0xFFu {
        case 0u: {
            let lo = data[p + P_LO];
            let i = x - i32(lo & 0xFFFFu);
            let j = y - i32(lo >> 16u);
            if ((kind & PK_BIG) != 0u) {
                let sp = data[data[p + 4u] + u32(j)];
                if (x < i32(sp & 0xFFFFu) || x > i32(sp >> 16u)) {
                    return;
                }
            } else {
                // Wrapping u32 arithmetic; the CPU checked that the exact
                // values fit in an i32 over the bounding box.
                let ui = bitcast<u32>(i);
                let uj = bitcast<u32>(j);
                let e0 = bitcast<i32>(data[p + 4u] + data[p + 7u] * ui + data[p + 10u] * uj);
                let e1 = bitcast<i32>(data[p + 5u] + data[p + 8u] * ui + data[p + 11u] * uj);
                let e2 = bitcast<i32>(data[p + 6u] + data[p + 9u] * ui + data[p + 12u] * uj);
                if ((e0 | e1 | e2) < 0) {
                    return;
                }
            }
            let st = data[p + P_SETUP];
            if (!in_scissor(st, x, y)) {
                return;
            }
            var z = 0u;
            if ((data[st + S_FLAGS] & F_ZTE) != 0u) {
                z = z_add(data[p + 37u], fat(p + 38u) + fat(p + 39u) * f32(i) + fat(p + 40u) * f32(j), data[st + S_ZMAX]);
            }
            pixel(st, x, y, z, p, linear);
        }
        case 1u: {
            let st = data[p + P_SETUP];
            if (!in_scissor(st, x, y)) {
                return;
            }
            pixel(st, x, y, min(data[p + 13u], data[st + S_ZMAX]), p, linear);
        }
        case 2u: {
            let x0 = fat(p + 4u);
            let y0 = fat(p + 5u);
            let x1 = fat(p + 6u);
            let y1 = fat(p + 7u);
            let steps = i32(data[p + 8u]);
            let n = max(steps, 1);
            var guess = 0;
            if (steps > 0) {
                if (abs(x1 - x0) >= abs(y1 - y0)) {
                    if (x1 != x0) {
                        guess = i32(round_away((f32(x) - x0) / (x1 - x0) * f32(steps)));
                    }
                } else {
                    guess = i32(round_away((f32(y) - y0) / (y1 - y0) * f32(steps)));
                }
            }
            var hit = -1;
            for (var k = guess - 1; k <= guess + 1; k = k + 1) {
                if (k < 0 || k >= n || hit >= 0) {
                    continue;
                }
                var t = 0.0;
                if (steps > 0) {
                    t = f32(k) / f32(steps);
                }
                if (i32(round_away(x0 + (x1 - x0) * t)) == x && i32(round_away(y0 + (y1 - y0) * t)) == y) {
                    hit = k;
                }
            }
            if (hit < 0) {
                return;
            }
            let st = data[p + P_SETUP];
            if (!in_scissor(st, x, y)) {
                return;
            }
            var t = 0.0;
            if (steps > 0) {
                t = f32(hit) / f32(steps);
            }
            line_t = t;
            let z = z_add(data[p + 27u], fat(p + 28u) * t, data[st + S_ZMAX]);
            pixel(st, x, y, z, p, linear);
        }
        default: {
            if (x != bitcast<i32>(data[p + 4u]) || y != bitcast<i32>(data[p + 5u])) {
                return;
            }
            let st = data[p + P_SETUP];
            if (!in_scissor(st, x, y)) {
                return;
            }
            pixel(st, x, y, min(data[p + 11u], data[st + S_ZMAX]), p, linear);
        }
    }
}

// params.a = (tile records, tile count, snapshot lo word, snapshot hi word).
// A tile record is [origin (tx | ty << 16), first list entry, count]; list
// entries are primitive record offsets. The workgroup loads the list and the
// bounding boxes into workgroup memory one tile-size chunk at a time, so the
// per-primitive loop (most primitives miss most pixels of a tile) does not
// wait on global memory.
const TILE: u32 = 8u;
const TILE_PX: u32 = TILE * TILE;
var<workgroup> sh_p: array<u32, TILE_PX>;
var<workgroup> sh_lo: array<u32, TILE_PX>;
var<workgroup> sh_hi: array<u32, TILE_PX>;

@compute @workgroup_size(TILE, TILE)
fn raster(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let ti = wid.x + wid.y * 32768u;
    if (ti >= params.a.y) {
        return;
    }
    let rec = params.a.x + ti * 3u;
    let origin = data[rec];
    let x = i32((origin & 0xFFFFu) * TILE + lid.x);
    let y = i32((origin >> 16u) * TILE + lid.y);
    let first = data[rec + 1u];
    let count = data[rec + 2u];
    for (var base = 0u; base < count; base = base + TILE_PX) {
        let n = min(TILE_PX, count - base);
        workgroupBarrier();
        if (li < n) {
            let p = data[first + base + li];
            sh_p[li] = p;
            sh_lo[li] = data[p + P_LO];
            sh_hi[li] = data[p + P_HI];
        }
        workgroupBarrier();
        for (var k = 0u; k < n; k = k + 1u) {
            let lo = sh_lo[k];
            let hi = sh_hi[k];
            if (x < i32(lo & 0xFFFFu) || y < i32(lo >> 16u) || x > i32(hi & 0xFFFFu) || y > i32(hi >> 16u)) {
                continue;
            }
            draw_prim(sh_p[k], x, y);
        }
    }
}

// Decode a texture level: params a = (setup, level, snapshot lo, snapshot
// hi), b = (destination, u0, v0, width), c.x = texel count.
@compute @workgroup_size(64)
fn decode(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x + gid.y * 32768u * 64u;
    if (i >= params.c.x) {
        return;
    }
    let st = params.a.x;
    let t = tex_level(st, params.a.y);
    let w = params.b.w;
    decoded[params.b.x + i] = conv(st, tex_read(t.surf, params.b.y + i % w, params.b.z + i / w));
}

// Host->local pixels into GPU-drawn pages: params a = (surface base,
// packed, pages per row, count), b = (DSAX, DSAY, RRW, first pixel index),
// c.x = offset of the values in `data`.
@compute @workgroup_size(64)
fn pixels(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x + gid.y * 32768u * 64u;
    if (i >= params.a.w) {
        return;
    }
    let p = params.a.y;
    let s = Surf(params.a.x, p & 0xFFFFu, (p >> 16u) & 7u, (p >> 19u) & 7u, (p >> 22u) & 3u, (p >> 24u) & 7u, params.a.z);
    let k = params.b.w + i;
    let x = (params.b.x + k % params.b.z) & 0x7FFu;
    let y = (params.b.y + k / params.b.z) & 0x7FFu;
    write_at(s, locate(s, x, y), data[params.c.x + i]);
}

// ------------------------------------------------------------------ display

// PCRTC readout (display.rs). params: a = (w, h, row_div, flags), where
// flags bit 0 = merge, bit 1 = SLBG, bit 2 = MMOD; b.x = ALP, b.y = BGCOLOR,
// b.z = offset in `data` of the two circuits (primary first):
// [surf(3), is16, dbx, dby, w, h] each.
fn circ_fetch(c: u32, x: u32, y: u32, row_div: u32) -> u32 {
    let s = surf_at(c);
    let l = locate(s, data[c + 4u] + x, data[c + 5u] + y / row_div);
    let v = extract(s.kind, atomicLoad(&vram[l.word]), l.shift);
    if (data[c + 3u] != 0u) {
        var a = 0u;
        if ((v & 0x8000u) != 0u) {
            a = 0x80000000u;
        }
        return ((v & 0x1Fu) << 3u) | ((v & 0x3E0u) << 6u) | ((v & 0x7C00u) << 9u) | a;
    }
    if (s.kind == KIND_W24) {
        return v | 0x80000000u;
    }
    return v;
}

@compute @workgroup_size(8, 8)
fn display(@builtin(global_invocation_id) gid: vec3<u32>) {
    let w = params.a.x;
    let h = params.a.y;
    if (gid.x >= w || gid.y >= h) {
        return;
    }
    let x = gid.x;
    let y = gid.y;
    let row_div = params.a.z;
    let flags = params.a.w;
    let c0 = params.b.z;
    let c1 = c0 + 8u;
    var px = circ_fetch(c0, x, y, row_div);
    if ((flags & 1u) != 0u) {
        var c2 = 0u;
        if ((flags & 2u) != 0u) {
            c2 = params.b.y;
        } else if (x < data[c1 + 6u] && y < data[c1 + 7u]) {
            c2 = circ_fetch(c1, x, y, row_div);
        }
        var a = min((px >> 24u) * 2u, 255u);
        if ((flags & 4u) != 0u) {
            a = params.b.x;
        }
        var o = 0u;
        for (var sh = 0u; sh < 24u; sh = sh + 8u) {
            let s1 = (px >> sh) & 0xFFu;
            let s2 = (c2 >> sh) & 0xFFu;
            o = o | (((s1 * a + s2 * (255u - a) + 127u) / 255u) << sh);
        }
        px = o;
    }
    outbuf[y * w + x] = px | 0xFF000000u;
}
