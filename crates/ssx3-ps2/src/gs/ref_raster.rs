//! The original, straightforward rasterizer, kept unchanged as the reference
//! for the differential tests in `diff_tests.rs` (the optimized `raster` must
//! produce bit-identical VRAM).
#![allow(dead_code)]

use super::swizzle::{self, Surface};
use super::{Regs, Vertex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum FbFmt {
    C32,
    C24,
    C16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TexKind {
    C32,
    C24,
    C16,
    Pal,
}

pub(crate) struct Tex {
    surf: Surface,
    kind: TexKind,
    tw: i32,
    th: i32,
    wms: u32,
    wmt: u32,
    minu: i32,
    maxu: i32,
    minv: i32,
    maxv: i32,
    ta0: u32,
    ta1: u32,
    aem: bool,
    palette: Vec<u32>,
    mag_linear: bool,
    min_linear: bool,
    lcm: bool,
    l: u32,
    k: f32,
}

pub(crate) struct Draw {
    iip: bool,
    tme: bool,
    fge: bool,
    abe: bool,
    fst: bool,
    fb: Surface,
    fb_fmt: FbFmt,
    fbmsk: u32,
    zb: Surface,
    zmax: u32,
    zte: bool,
    ztst: u32,
    zmsk: bool,
    ate: bool,
    atst: u32,
    aref: i32,
    afail: u32,
    date: bool,
    datm: bool,
    ba: u32,
    bb: u32,
    bc: u32,
    bd: u32,
    fix: i32,
    pabe: bool,
    colclamp: bool,
    fba: bool,
    sx0: i32,
    sx1: i32,
    sy0: i32,
    sy1: i32,
    scanmsk: u32,
    dither: Option<[[i32; 4]; 4]>,
    ofx: i32,
    ofy: i32,
    tfx: u32,
    tcc: bool,
    fogcol: [i32; 3],
    tex: Option<Tex>,
}

#[inline]
fn expand16(c: u32, ta0: u32, ta1: u32, aem: bool) -> u32 {
    let r = (c & 0x1F) << 3;
    let g = ((c >> 5) & 0x1F) << 3;
    let b = ((c >> 10) & 0x1F) << 3;
    let a = if c & 0x8000 != 0 {
        ta1
    } else if aem && c & 0x7FFF == 0 {
        0
    } else {
        ta0
    };
    r | (g << 8) | (b << 16) | (a << 24)
}

impl Draw {
    pub(crate) fn build(regs: &Regs, clut: &[u16; 512]) -> Draw {
        let prim = regs.prim;
        let attr = if regs.prmodecont & 1 != 0 {
            prim
        } else {
            (prim & 7) | (regs.prmode & !7)
        };
        let ctx = &regs.ctx[((attr >> 9) & 1) as usize];

        let fbp = (ctx.frame & 0x1FF) as u32 * 32;
        let fbw = ((ctx.frame >> 16) & 0x3F) as u32;
        let fpsm = ((ctx.frame >> 24) & 0x3F) as u32;
        let fbmsk = (ctx.frame >> 32) as u32;
        let fb_fmt = match fpsm {
            swizzle::PSMCT24 | swizzle::PSMZ24 => FbFmt::C24,
            swizzle::PSMCT16 | swizzle::PSMCT16S | swizzle::PSMZ16 | swizzle::PSMZ16S => FbFmt::C16,
            _ => FbFmt::C32,
        };
        let zbp = (ctx.zbuf & 0x1FF) as u32 * 32;
        let zpsm = 0x30 | ((ctx.zbuf >> 24) & 0xF) as u32;
        let zmax = match zpsm {
            swizzle::PSMZ24 => 0xFF_FFFF,
            swizzle::PSMZ16 | swizzle::PSMZ16S => 0xFFFF,
            _ => 0xFFFF_FFFF,
        };
        let test = ctx.test;
        let alpha = ctx.alpha;
        let sc = ctx.scissor;

        let dthe = regs.dthe & 1 != 0 && fb_fmt == FbFmt::C16;
        let dither = if dthe {
            let mut m = [[0i32; 4]; 4];
            for (i, row) in m.iter_mut().enumerate() {
                for (j, v) in row.iter_mut().enumerate() {
                    let bits = ((regs.dimx >> (i * 16 + j * 4)) & 7) as i32;
                    *v = if bits & 4 != 0 { bits - 8 } else { bits };
                }
            }
            Some(m)
        } else {
            None
        };

        let tme = attr & 0x10 != 0;
        let tex0 = ctx.tex0;
        let tex = if tme {
            let tbp = (tex0 & 0x3FFF) as u32;
            let tbw = ((tex0 >> 14) & 0x3F) as u32;
            let tpsm = ((tex0 >> 20) & 0x3F) as u32;
            let tw = ((tex0 >> 26) & 0xF).min(10) as i32;
            let th = ((tex0 >> 30) & 0xF).min(10) as i32;
            let cpsm = ((tex0 >> 51) & 0xF) as u32;
            let csa = ((tex0 >> 56) & 0x1F) as u32;
            let texa = regs.texa;
            let ta0 = (texa & 0xFF) as u32;
            let aem = (texa >> 15) & 1 != 0;
            let ta1 = ((texa >> 32) & 0xFF) as u32;
            let kind = match tpsm {
                swizzle::PSMCT24 | swizzle::PSMZ24 => TexKind::C24,
                swizzle::PSMCT16 | swizzle::PSMCT16S | swizzle::PSMZ16 | swizzle::PSMZ16S => {
                    TexKind::C16
                }
                swizzle::PSMT8
                | swizzle::PSMT8H
                | swizzle::PSMT4
                | swizzle::PSMT4HL
                | swizzle::PSMT4HH => TexKind::Pal,
                _ => TexKind::C32,
            };
            let mut palette = Vec::new();
            if kind == TexKind::Pal {
                let n = if matches!(tpsm, swizzle::PSMT8 | swizzle::PSMT8H) {
                    256
                } else {
                    16
                };
                palette.reserve(n);
                let is32 = !matches!(cpsm, 0x2 | 0xA);
                for i in 0..n as u32 {
                    let c = if is32 {
                        let slot = (((csa & 15) * 16 + i) & 255) as usize;
                        clut[slot] as u32 | (clut[slot + 256] as u32) << 16
                    } else {
                        expand16(clut[((csa * 16 + i) & 511) as usize] as u32, ta0, ta1, aem)
                    };
                    palette.push(c);
                }
            }
            let tex1 = ctx.tex1;
            let cl = ctx.clamp;
            let mmin = ((tex1 >> 6) & 7) as u32;
            let kraw = ((tex1 >> 32) & 0xFFF) as i32;
            let k = ((kraw << 20) >> 20) as f32 / 16.0;
            Some(Tex {
                surf: Surface::new(tpsm, tbp, tbw),
                kind,
                tw,
                th,
                wms: (cl & 3) as u32,
                wmt: ((cl >> 2) & 3) as u32,
                minu: ((cl >> 4) & 0x3FF) as i32,
                maxu: ((cl >> 14) & 0x3FF) as i32,
                minv: ((cl >> 24) & 0x3FF) as i32,
                maxv: ((cl >> 34) & 0x3FF) as i32,
                ta0,
                ta1,
                aem,
                palette,
                mag_linear: (tex1 >> 5) & 1 != 0,
                min_linear: matches!(mmin, 1 | 4 | 5),
                lcm: tex1 & 1 != 0,
                l: ((tex1 >> 19) & 3) as u32,
                k,
            })
        } else {
            None
        };

        Draw {
            iip: attr & 0x08 != 0,
            tme,
            fge: attr & 0x20 != 0,
            abe: attr & 0x40 != 0,
            fst: attr & 0x100 != 0,
            fb: Surface::new(fpsm, fbp, fbw),
            fb_fmt,
            fbmsk,
            zb: Surface::new(zpsm, zbp, fbw),
            zmax,
            zte: (test >> 16) & 1 != 0,
            ztst: ((test >> 17) & 3) as u32,
            zmsk: (ctx.zbuf >> 32) & 1 != 0,
            ate: test & 1 != 0,
            atst: ((test >> 1) & 7) as u32,
            aref: ((test >> 4) & 0xFF) as i32,
            afail: ((test >> 12) & 3) as u32,
            date: (test >> 14) & 1 != 0,
            datm: (test >> 15) & 1 != 0,
            ba: (alpha & 3) as u32,
            bb: ((alpha >> 2) & 3) as u32,
            bc: ((alpha >> 4) & 3) as u32,
            bd: ((alpha >> 6) & 3) as u32,
            fix: ((alpha >> 32) & 0xFF) as i32,
            pabe: regs.pabe & 1 != 0,
            colclamp: regs.colclamp & 1 != 0,
            fba: ctx.fba & 1 != 0,
            sx0: (sc & 0x7FF) as i32,
            sx1: ((sc >> 16) & 0x7FF) as i32,
            sy0: ((sc >> 32) & 0x7FF) as i32,
            sy1: ((sc >> 48) & 0x7FF) as i32,
            scanmsk: (regs.scanmsk & 3) as u32,
            dither,
            ofx: (ctx.xyoffset & 0xFFFF) as i32,
            ofy: ((ctx.xyoffset >> 32) & 0xFFFF) as i32,
            tfx: ((tex0 >> 35) & 3) as u32,
            tcc: (tex0 >> 34) & 1 != 0,
            fogcol: [
                (regs.fogcol & 0xFF) as i32,
                ((regs.fogcol >> 8) & 0xFF) as i32,
                ((regs.fogcol >> 16) & 0xFF) as i32,
            ],
            tex,
        }
    }

    #[inline]
    fn read_fb(&self, vram: &[u32], x: u32, y: u32) -> u32 {
        let c = self.fb.read(vram, x, y);
        match self.fb_fmt {
            FbFmt::C32 => c,
            FbFmt::C24 => c | 0x8000_0000,
            FbFmt::C16 => {
                ((c & 0x1F) << 3)
                    | (((c >> 5) & 0x1F) << 11)
                    | (((c >> 10) & 0x1F) << 19)
                    | if c & 0x8000 != 0 { 0x8000_0000 } else { 0 }
            }
        }
    }

    #[inline]
    fn write_fb(&self, vram: &mut [u32], x: u32, y: u32, c: u32, keep: u32) {
        match self.fb_fmt {
            FbFmt::C32 => self.fb.write_masked(vram, x, y, c, keep),
            FbFmt::C24 => self
                .fb
                .write_masked(vram, x, y, c & 0xFF_FFFF, keep & 0xFF_FFFF),
            FbFmt::C16 => {
                let pack = |v: u32| {
                    ((v >> 3) & 0x1F)
                        | (((v >> 11) & 0x1F) << 5)
                        | (((v >> 19) & 0x1F) << 10)
                        | ((v >> 16) & 0x8000)
                };
                self.fb.write_masked(vram, x, y, pack(c), pack(keep));
            }
        }
    }
}

/// Interpolated fragment inputs.
#[derive(Clone, Copy, Default)]
struct Frag {
    r: i32,
    g: i32,
    b: i32,
    a: i32,
    /// Texel-space coordinates.
    u: f32,
    v: f32,
    f: i32,
    z: f64,
}

// --------------------------------------------------------------------- texturing

#[inline]
fn wrap(c: i32, mode: u32, size_log: i32, min: i32, max: i32) -> i32 {
    match mode {
        0 => c & ((1 << size_log) - 1),
        1 => c.clamp(0, (1 << size_log) - 1),
        2 => c.clamp(min, max.max(min)),
        _ => (c & min) | max,
    }
}

#[inline]
fn texel(vram: &[u32], t: &Tex, x: i32, y: i32) -> u32 {
    let x = wrap(x, t.wms, t.tw, t.minu, t.maxu) as u32;
    let y = wrap(y, t.wmt, t.th, t.minv, t.maxv) as u32;
    let raw = t.surf.read(vram, x, y);
    match t.kind {
        TexKind::C32 => raw,
        TexKind::C24 => {
            let a = if t.aem && raw == 0 { 0 } else { t.ta0 };
            raw | (a << 24)
        }
        TexKind::C16 => expand16(raw, t.ta0, t.ta1, t.aem),
        TexKind::Pal => t.palette[raw as usize & (t.palette.len() - 1)],
    }
}

#[inline]
fn sample(vram: &[u32], t: &Tex, u: f32, v: f32, linear: bool) -> u32 {
    if !linear {
        return texel(vram, t, u.floor() as i32, v.floor() as i32);
    }
    let iu = ((u - 0.5) * 256.0).floor() as i32;
    let iv = ((v - 0.5) * 256.0).floor() as i32;
    let (x0, y0) = (iu >> 8, iv >> 8);
    let (fx, fy) = ((iu & 255) as u32, (iv & 255) as u32);
    let c00 = texel(vram, t, x0, y0);
    let c10 = texel(vram, t, x0 + 1, y0);
    let c01 = texel(vram, t, x0, y0 + 1);
    let c11 = texel(vram, t, x0 + 1, y0 + 1);
    let mut out = 0u32;
    for sh in [0, 8, 16, 24] {
        let ch = |c: u32| (c >> sh) & 0xFF;
        let top = ch(c00) * (256 - fx) + ch(c10) * fx;
        let bot = ch(c01) * (256 - fx) + ch(c11) * fx;
        let v = (top * (256 - fy) + bot * fy + (1 << 15)) >> 16;
        out |= v.min(255) << sh;
    }
    out
}

// --------------------------------------------------------------------- pixel pipeline

#[inline]
fn pixel(vram: &mut [u32], d: &Draw, x: i32, y: i32, f: &Frag, linear: bool) {
    if x < d.sx0 || x > d.sx1 || y < d.sy0 || y > d.sy1 {
        return;
    }
    match d.scanmsk {
        2 if y & 1 == 0 => return,
        3 if y & 1 == 1 => return,
        _ => {}
    }
    let (xu, yu) = (x as u32, y as u32);
    let (mut r, mut g, mut b, mut a) = (f.r, f.g, f.b, f.a);

    if let Some(t) = &d.tex {
        let c = sample(vram, t, f.u, f.v, linear);
        let tr = (c & 0xFF) as i32;
        let tg = ((c >> 8) & 0xFF) as i32;
        let tb = ((c >> 16) & 0xFF) as i32;
        let ta = (c >> 24) as i32;
        match d.tfx {
            0 => {
                r = ((tr * r) >> 7).min(255);
                g = ((tg * g) >> 7).min(255);
                b = ((tb * b) >> 7).min(255);
                if d.tcc {
                    a = ((ta * a) >> 7).min(255);
                }
            }
            1 => {
                r = tr;
                g = tg;
                b = tb;
                if d.tcc {
                    a = ta;
                }
            }
            2 => {
                r = (((tr * r) >> 7) + a).min(255);
                g = (((tg * g) >> 7) + a).min(255);
                b = (((tb * b) >> 7) + a).min(255);
                if d.tcc {
                    a = (ta + a).min(255);
                }
            }
            _ => {
                r = (((tr * r) >> 7) + a).min(255);
                g = (((tg * g) >> 7) + a).min(255);
                b = (((tb * b) >> 7) + a).min(255);
                if d.tcc {
                    a = ta;
                }
            }
        }
    }

    if d.fge {
        let fv = f.f.clamp(0, 255);
        r = (fv * r + (255 - fv) * d.fogcol[0]) >> 8;
        g = (fv * g + (255 - fv) * d.fogcol[1]) >> 8;
        b = (fv * b + (255 - fv) * d.fogcol[2]) >> 8;
    }

    let mut write_fb = true;
    let mut write_z = d.zte && !d.zmsk;
    let mut keep = d.fbmsk;
    if d.ate {
        let pass = match d.atst {
            0 => false,
            1 => true,
            2 => a < d.aref,
            3 => a <= d.aref,
            4 => a == d.aref,
            5 => a >= d.aref,
            6 => a > d.aref,
            _ => a != d.aref,
        };
        if !pass {
            match d.afail {
                0 => return,
                1 => write_z = false,
                2 => write_fb = false,
                _ => {
                    write_z = false;
                    keep |= 0xFF00_0000;
                }
            }
        }
    }

    let mut dst: Option<u32> = None;
    if d.date && d.fb_fmt != FbFmt::C24 {
        let c = d.read_fb(vram, xu, yu);
        if (c >> 31 != 0) != d.datm {
            return;
        }
        dst = Some(c);
    }

    if d.zte {
        let z = f.z.clamp(0.0, d.zmax as f64) as u32;
        let pass = match d.ztst {
            0 => false,
            1 => true,
            2 => z >= d.zb.read(vram, xu, yu),
            _ => z > d.zb.read(vram, xu, yu),
        };
        if !pass {
            return;
        }
        if write_z {
            d.zb.write(vram, xu, yu, z);
        }
    }

    if !write_fb || keep == 0xFFFF_FFFF {
        return;
    }
    if d.abe && (!d.pabe || a >= 0x80) {
        let dc = dst.unwrap_or_else(|| d.read_fb(vram, xu, yu));
        let cd = [
            (dc & 0xFF) as i32,
            ((dc >> 8) & 0xFF) as i32,
            ((dc >> 16) & 0xFF) as i32,
        ];
        let ad = (dc >> 24) as i32;
        let cs = [r, g, b];
        let c = match d.bc {
            0 => a,
            1 => ad,
            _ => d.fix,
        };
        let mut out = [0i32; 3];
        for i in 0..3 {
            let sel = |s: u32| match s {
                0 => cs[i],
                1 => cd[i],
                _ => 0,
            };
            out[i] = (((sel(d.ba) - sel(d.bb)) * c) >> 7) + sel(d.bd);
        }
        r = out[0];
        g = out[1];
        b = out[2];
    }
    if let Some(m) = &d.dither {
        let dv = m[(y & 3) as usize][(x & 3) as usize];
        r += dv;
        g += dv;
        b += dv;
    }
    if d.colclamp {
        r = r.clamp(0, 255);
        g = g.clamp(0, 255);
        b = b.clamp(0, 255);
    } else {
        r &= 0xFF;
        g &= 0xFF;
        b &= 0xFF;
    }
    let mut oa = (a & 0xFF) as u32;
    if d.fba {
        oa |= 0x80;
    }
    let c = r as u32 | (g as u32) << 8 | (b as u32) << 16 | oa << 24;
    d.write_fb(vram, xu, yu, c, keep);
}

// --------------------------------------------------------------------- primitives

const R: usize = 0;
const G: usize = 1;
const B: usize = 2;
const A: usize = 3;
const S: usize = 4;
const T: usize = 5;
const Q: usize = 6;
const F: usize = 7;
const Z: usize = 8;
const NATTR: usize = 9;

fn attrs(d: &Draw, v: &Vertex) -> [f64; NATTR] {
    let (s, t, q) = if d.fst {
        (v.u as f64, v.v as f64, 1.0)
    } else {
        (v.s as f64, v.t as f64, v.q as f64)
    };
    [
        v.rgba[0] as f64,
        v.rgba[1] as f64,
        v.rgba[2] as f64,
        v.rgba[3] as f64,
        s,
        t,
        q,
        v.f as f64,
        v.z as f64,
    ]
}

/// Build a fragment from interpolated attributes.
#[inline]
fn frag(d: &Draw, at: &[f64; NATTR]) -> Frag {
    let (u, v) = match &d.tex {
        Some(t) if d.tme => {
            if d.fst {
                ((at[S] / 16.0) as f32, (at[T] / 16.0) as f32)
            } else {
                let q = if at[Q] != 0.0 {
                    at[Q]
                } else {
                    f64::MIN_POSITIVE
                };
                (
                    (at[S] / q * (1 << t.tw) as f64) as f32,
                    (at[T] / q * (1 << t.th) as f64) as f32,
                )
            }
        }
        _ => (0.0, 0.0),
    };
    Frag {
        r: at[R] as i32,
        g: at[G] as i32,
        b: at[B] as i32,
        a: at[A] as i32,
        u,
        v,
        f: at[F] as i32,
        z: at[Z],
    }
}

/// Choose bilinear vs nearest for a primitive from TEX1 (no mipmapping).
fn linear_filter(d: &Draw, verts: &[Vertex]) -> bool {
    let Some(t) = &d.tex else { return false };
    let lod = if t.lcm {
        t.k
    } else {
        let q = if d.fst {
            1.0
        } else {
            let sum: f32 = verts.iter().map(|v| v.q.abs()).sum();
            sum / verts.len() as f32
        };
        let q = if q > 0.0 { q } else { 1.0 };
        (1.0 / q).log2() * (1u32 << t.l) as f32 + t.k
    };
    if lod <= 0.0 {
        t.mag_linear
    } else {
        t.min_linear
    }
}

pub(crate) fn draw(vram: &mut [u32], d: &Draw, prim: u32, verts: &[Vertex]) {
    let linear = linear_filter(d, verts);
    match prim {
        0 => point(vram, d, &verts[0], linear),
        1 | 2 => line(vram, d, &verts[0], &verts[1], linear),
        3..=5 => triangle(vram, d, verts, linear),
        6 => sprite(vram, d, &verts[0], &verts[1], linear),
        _ => {}
    }
}

fn point(vram: &mut [u32], d: &Draw, v: &Vertex, linear: bool) {
    let x = (v.x as i32 - d.ofx + 8) >> 4;
    let y = (v.y as i32 - d.ofy + 8) >> 4;
    let f = frag(d, &attrs(d, v));
    pixel(vram, d, x, y, &f, linear);
}

fn line(vram: &mut [u32], d: &Draw, v0: &Vertex, v1: &Vertex, linear: bool) {
    let x0 = (v0.x as i32 - d.ofx) as f64 / 16.0;
    let y0 = (v0.y as i32 - d.ofy) as f64 / 16.0;
    let x1 = (v1.x as i32 - d.ofx) as f64 / 16.0;
    let y1 = (v1.y as i32 - d.ofy) as f64 / 16.0;
    let mut a0 = attrs(d, v0);
    let a1 = attrs(d, v1);
    if !d.iip {
        a0[..4].copy_from_slice(&a1[..4]);
    }
    let steps = (x1 - x0).abs().max((y1 - y0).abs()).round() as i32;
    let n = steps.max(1);
    let mut at = [0.0; NATTR];
    for i in 0..n {
        let t = if steps > 0 {
            i as f64 / steps as f64
        } else {
            0.0
        };
        let px = (x0 + (x1 - x0) * t).round() as i32;
        let py = (y0 + (y1 - y0) * t).round() as i32;
        for k in 0..NATTR {
            at[k] = a0[k] + (a1[k] - a0[k]) * t;
        }
        pixel(vram, d, px, py, &frag(d, &at), linear);
    }
}

fn sprite(vram: &mut [u32], d: &Draw, v0: &Vertex, v1: &Vertex, linear: bool) {
    let x0 = v0.x as i32 - d.ofx;
    let y0 = v0.y as i32 - d.ofy;
    let x1 = v1.x as i32 - d.ofx;
    let y1 = v1.y as i32 - d.ofy;
    let (left, right) = (x0.min(x1), x0.max(x1));
    let (top, bottom) = (y0.min(y1), y0.max(y1));
    // Pixel p is covered when left <= p*16 < right.
    let px0 = ((left + 15) >> 4).max(d.sx0);
    let px1 = (((right + 15) >> 4) - 1).min(d.sx1);
    let py0 = ((top + 15) >> 4).max(d.sy0);
    let py1 = (((bottom + 15) >> 4) - 1).min(d.sy1);
    if px0 > px1 || py0 > py1 {
        return;
    }
    let a0 = attrs(d, v0);
    let a1 = attrs(d, v1);
    // Flat attributes come from the second vertex; S/T (or U/V) interpolate.
    let mut at = a1;
    let dsdx = if x1 != x0 {
        (a1[S] - a0[S]) * 16.0 / (x1 - x0) as f64
    } else {
        0.0
    };
    let dtdy = if y1 != y0 {
        (a1[T] - a0[T]) * 16.0 / (y1 - y0) as f64
    } else {
        0.0
    };
    let s_at = |px: i32| a0[S] + dsdx * ((px * 16 - x0) as f64 / 16.0);
    let t_at = |py: i32| a0[T] + dtdy * ((py * 16 - y0) as f64 / 16.0);
    let texturing = d.tex.is_some();
    let base = frag(d, &at);
    for py in py0..=py1 {
        at[T] = t_at(py);
        for px in px0..=px1 {
            let f = if texturing {
                at[S] = s_at(px);
                frag(d, &at)
            } else {
                base
            };
            pixel(vram, d, px, py, &f, linear);
        }
    }
}

fn triangle(vram: &mut [u32], d: &Draw, verts: &[Vertex], linear: bool) {
    let mut p = [(0i64, 0i64); 3];
    let mut a = [[0.0f64; NATTR]; 3];
    for i in 0..3 {
        p[i] = (
            (verts[i].x as i32 - d.ofx) as i64,
            (verts[i].y as i32 - d.ofy) as i64,
        );
        a[i] = attrs(d, &verts[i]);
    }
    if !d.iip {
        let flat = a[2];
        for ai in a.iter_mut().take(2) {
            ai[..4].copy_from_slice(&flat[..4]);
        }
    }
    let mut area = (p[1].0 - p[0].0) * (p[2].1 - p[0].1) - (p[2].0 - p[0].0) * (p[1].1 - p[0].1);
    if area == 0 {
        return;
    }
    if area < 0 {
        p.swap(1, 2);
        a.swap(1, 2);
        area = -area;
    }

    // Bounding box in pixels, clipped to the scissor.
    let minx = p.iter().map(|q| q.0).min().unwrap();
    let maxx = p.iter().map(|q| q.0).max().unwrap();
    let miny = p.iter().map(|q| q.1).min().unwrap();
    let maxy = p.iter().map(|q| q.1).max().unwrap();
    let bx0 = (((minx + 15) >> 4) as i32).max(d.sx0);
    let bx1 = ((maxx >> 4) as i32).min(d.sx1);
    let by0 = (((miny + 15) >> 4) as i32).max(d.sy0);
    let by1 = ((maxy >> 4) as i32).min(d.sy1);
    if bx0 > bx1 || by0 > by1 {
        return;
    }

    // Edge functions E_ab(p) = (xb-xa)(py-ya) - (yb-ya)(px-xa), positive inside.
    struct Edge {
        dx: i64, // step per +1 pixel in x
        dy: i64, // step per +1 pixel in y
        row: i64,
    }
    let mut edges: [Edge; 3] = std::array::from_fn(|i| {
        let (xa, ya) = p[i];
        let (xb, yb) = p[(i + 1) % 3];
        let gx = -(yb - ya);
        let gy = xb - xa;
        let top_left = gx > 0 || (gx == 0 && gy > 0);
        let sx = bx0 as i64 * 16;
        let sy = by0 as i64 * 16;
        let e = gy * (sy - ya) + gx * (sx - xa);
        Edge {
            dx: gx * 16,
            dy: gy * 16,
            row: if top_left { e } else { e - 1 },
        }
    });

    // Attribute plane gradients (per pixel).
    let fx = |i: usize| p[i].0 as f64 / 16.0;
    let fy = |i: usize| p[i].1 as f64 / 16.0;
    let (x10, y10) = (fx(1) - fx(0), fy(1) - fy(0));
    let (x20, y20) = (fx(2) - fx(0), fy(2) - fy(0));
    let inv = 1.0 / (area as f64 / 256.0);
    let mut dadx = [0.0f64; NATTR];
    let mut dady = [0.0f64; NATTR];
    for k in 0..NATTR {
        let d1 = a[1][k] - a[0][k];
        let d2 = a[2][k] - a[0][k];
        dadx[k] = (d1 * y20 - d2 * y10) * inv;
        dady[k] = (d2 * x10 - d1 * x20) * inv;
    }
    let (ox, oy) = (fx(0), fy(0));
    let mut at = [0.0f64; NATTR];

    for y in by0..=by1 {
        let (e0, e1, e2) = (edges[0].row, edges[1].row, edges[2].row);
        let mut row_attr = [0.0f64; NATTR];
        let dyf = y as f64 - oy;
        for k in 0..NATTR {
            row_attr[k] = a[0][k] + dadx[k] * (bx0 as f64 - ox) + dady[k] * dyf;
        }
        let (mut w0, mut w1, mut w2) = (e0, e1, e2);
        let mut entered = false;
        for x in bx0..=bx1 {
            if (w0 | w1 | w2) >= 0 {
                entered = true;
                let dxp = (x - bx0) as f64;
                for k in 0..NATTR {
                    at[k] = row_attr[k] + dadx[k] * dxp;
                }
                pixel(vram, d, x, y, &frag(d, &at), linear);
            } else if entered {
                break;
            }
            w0 += edges[0].dx;
            w1 += edges[1].dx;
            w2 += edges[2].dx;
        }
        for e in edges.iter_mut() {
            e.row += e.dy;
        }
    }
}
