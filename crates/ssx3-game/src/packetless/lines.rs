//! Indexed triangles (slot 72 `DrawIndexedTriangles`) and line strips
//! (slot 73 `DrawLineStrip`) without packets: the EE projects (and, for
//! lines, clips) the vertices and sends GIF REGLIST blocks with `DIRECT`
//! (`docs/investigation/n2-capture/direct2d.md` §1, §2); here those blocks
//! are the recorded draw (`Step::Gif`, as `crate::capture` reads the
//! packet back).
//!
//! Slot 73's builder (`LineStrip_Init` 0x384d98, `_Begin` 0x3850a8,
//! `_AddVertex` 0x384fd0, `_Close` 0x384dc0, `_Finish` 0x384e50) and its
//! clipper (`PS2GraphicsMan_ClipLineSegment` 0x383a10) are reached only
//! from slot 73; they are folded in here (`Strips`, `clip`).

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::vu0::{self, V};
use super::*;
use crate::ps2f::FpuF32;

/// A 32-bit value as the generated code holds it in a 64-bit register
/// (`lw` / `mfc1` / `cvt.w.s` + `mfc1` sign-extend).
fn sx64(x: u32) -> u64 {
    x as i32 as i64 as u64
}

fn qw(lo: u64, hi: u64) -> u128 {
    lo as u128 | (hi as u128) << 64
}

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// `cvt.w.s` of a float word, sign-extended.
fn int(x: FpuF32) -> u64 {
    sx64(x.to_i32() as u32)
}

// ---------------------------------------------------------------------------
// Slot 72.

/// `PS2GraphicsMan_DrawIndexedTriangles(gm, n, -, idx, pos, uv, col, off)`
/// (slot 72, 0x379bd0): `n` triangles from byte indices, transformed by
/// (gm+0x6b30) · translate(`off`) (staged in the scratchpad matrix at
/// `*(gp+0x2a94)`, which keeps it), divided by w. Untextured (state
/// +0x10 < 0): PRIM 0x4b, 3n vertices {RGBAQ, XYZ2}. Textured: PRIM 0x5b,
/// {ST, RGBAQ, XYZ2} with every colour channel halved; the generated body
/// writes only `n` such vertices under a CNT sized for 3n, so for n > 0 its
/// packet's DMA chain overruns the record's end and the capture reads
/// nothing: neither does this form.
pub fn draw_indexed_triangles(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    let n = c.g[A1][0] as i32;
    // (t1, the UVs, is read only by textured vertices, which are not built.)
    let (idx, pos, col, off) = (c.g[A3][0] as u32, c.g[T0][0] as u32, c.g[T2][0] as u32, c.g[T3][0] as u32);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0xc0);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 3, 0xc000_0000, 0);
    ensure_view_matrices(c, gm, 0x0037_9c4c);
    // The matrix: the global identity with row 3 := I · (off, 1), then each
    // row through the projection (`vmula.x .. vmadd.w`).
    let gp = c.g[GP][0] as u32;
    let spr = r32(c, gp + 0x2a94);
    let ident = qm(c, 0x004f_f1a0);
    let o: V = [r32(c, off), r32(c, off + 4), r32(c, off + 8), vu0::ONE];
    let t: vu0::M = [ident[0], ident[1], ident[2], vu0::xform(&ident, &o)];
    let proj = qm(c, gm + 0x6b30);
    let m = vu0::mul_m(&proj, &t);
    wm(c, spr, &m);
    let st = r32(c, gm + gmf::CUR_STATE);
    let untextured = (c.bus.read16(st + 0x10) as i16) < 0;
    let half = FpuF32(0x3f00_0000);
    let q1 = (vu0::ONE as u64) << 32;
    // GIFtag (NLOOP 1, A+D), PRIM.
    let mut gif = vec![qw(0x1000_0000_0000_0001, 0xe)];
    let mut cycles = 120;
    let ok = if untextured {
        gif.push(qw(0x4b, 0));
        // REGLIST, NREG 6 (RGBAQ, XYZ2 ×3), EOP, NLOOP n.
        gif.push(qw(sx64(n as u32) | 0x8000 | 0x6400 << 48, 0x51_5151));
        let k = n.wrapping_mul(3);
        for i in 0..k.max(0) {
            let j = c.bus.read8(idx.wrapping_add(i as u32)) as u32;
            let p = project(c, &m, pos.wrapping_add(j * 0xc));
            let cv = col.wrapping_add(j * 0x10);
            let r = int(f(c, cv + 4));
            let g = int(f(c, cv + 8));
            let b = int(f(c, cv + 0xc));
            let a = int(f(c, cv) * half);
            gif.push(qw(r | g << 8 | b << 16 | a << 24 | q1, xyz(&p)));
        }
        cycles += 72 * k.max(0) as u64;
        // (CNT 3n+4 ends at the record's end.)
        n >= 0 && k as i64 + 4 <= 0xffff
    } else {
        gif.push(qw(0x5b, 0));
        // REGLIST, NREG 9 (ST, RGBAQ, XYZ2 ×3), EOP, NLOOP n.
        gif.push(qw(sx64(n as u32) | 0x8000 | 0x9400 << 48, 0x5_1251_2512));
        // (CNT 9n/2 + (n&1) + 4 ends at the record's end only for n = 0;
        // for n > 0 nothing is captured, so the vertices are not built.)
        if n != 0 {
            cycles += 82 * n.max(0) as u64;
        }
        n == 0
    };
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(if ok { vec![Step::Gif(gif)] } else { Vec::new() }));
    add_record(c, gm, start, start, false);
    ret(c, cycles);
}

/// `m · (pos, 1)`, then all lanes times Q = 1/w (`vdiv`, `vmulq`).
fn project(c: &mut Cpu, m: &vu0::M, pos: u32) -> V {
    let v: V = [r32(c, pos), r32(c, pos + 4), r32(c, pos + 8), vu0::ONE];
    let p = vu0::xform(m, &v);
    let q = float::div(vu0::ONE, p[3]).value;
    vu0::mul(&p, &[q; 4])
}

/// XYZ2 from x, y, z: `int(x) | int(y) << 16 | int(z) << 32` (64-bit ORs of
/// the sign-extended integers).
fn xyz(p: &V) -> u64 {
    int(FpuF32(p[0])) | int(FpuF32(p[1])) << 16 | int(FpuF32(p[2])) << 32
}

// ---------------------------------------------------------------------------
// Slot 73.

/// A line vertex (0x30 bytes, as words): S, T (+0, +4), R, G, B, A (+0x10
/// .. +0x1c, u32), position (+0x20).
type Vert = [u32; 12];

fn vert(c: &mut Cpu, a: u32) -> Vert {
    std::array::from_fn(|i| r32(c, a.wrapping_add(4 * i as u32)))
}

/// The clip bounds (gm+0x6b70 ..): x / y minimums and maximums, the two z
/// planes' z and w.
struct Bounds {
    x70: FpuF32,
    y74: FpuF32,
    z78: FpuF32,
    w7c: FpuF32,
    x80: FpuF32,
    y84: FpuF32,
    z88: FpuF32,
    w8c: FpuF32,
}

impl Bounds {
    fn read(c: &mut Cpu, gm: u32) -> Bounds {
        Bounds {
            x70: f(c, gm + 0x6b70),
            y74: f(c, gm + 0x6b74),
            z78: f(c, gm + 0x6b78),
            w7c: f(c, gm + 0x6b7c),
            x80: f(c, gm + 0x6b80),
            y84: f(c, gm + 0x6b84),
            z88: f(c, gm + 0x6b88),
            w8c: f(c, gm + 0x6b8c),
        }
    }

    /// x outcode (x < min: 2, x > max: 1) and, with `y`, y (y < min: 8,
    /// y > max: 4).
    fn code_xy(&self, p: &V, y: bool) -> u32 {
        let x = FpuF32(p[0]);
        let mut k = if x.lt(self.x70) { 2 } else { 0 };
        if self.x80.lt(x) {
            k |= 1;
        }
        if y {
            let yv = FpuF32(p[1]);
            if yv.lt(self.y74) {
                k |= 8;
            }
            if self.y84.lt(yv) {
                k |= 4;
            }
        }
        k
    }

    /// A projected vertex's outcode: x / y, then for q < 0 `^ 0x2f`, else
    /// z > +0x6b78: 0x20, z < +0x6b88: 0x10.
    fn code(&self, s: &V) -> u32 {
        let k = self.code_xy(s, true);
        if FpuF32(s[3]).lt(FpuF32::ZERO) {
            return k ^ 0x2f;
        }
        let z = FpuF32(s[2]);
        let mut k = k;
        if self.z78.lt(z) {
            k |= 0x20;
        }
        if z.lt(self.z88) {
            k |= 0x10;
        }
        k
    }
}

/// `ClipLineSegment`'s arguments: the two vertices, their projections
/// {x, y, z, q}, their outcodes (clipped in place).
struct Seg {
    v: [Vert; 2],
    p: [V; 2],
    code: [u32; 2],
}

/// A word converted as unsigned (`cvt.s.w`, or for the top bit set half of
/// it with the low bit kept, converted and doubled).
fn u2f(v: u32) -> FpuF32 {
    if (v as i32) >= 0 {
        FpuF32::from_i32(v as i32)
    } else {
        let h = FpuF32::from_i32((v >> 1 | v & 1) as i32);
        h + h
    }
}

impl Seg {
    /// Vertex `i` := vertex 0 + k · (vertex 1 - vertex 0): S, T as floats,
    /// the colour words as unsigned (the difference too, wrapping).
    fn lerp(&mut self, i: usize, k: FpuF32) {
        let (a, b) = (self.v[0], self.v[1]);
        let o = &mut self.v[i];
        for l in 0..2 {
            let (x, y) = (FpuF32(a[l]), FpuF32(b[l]));
            o[l] = (x + k * (y - x)).0;
        }
        for l in 4..8 {
            o[l] = (u2f(a[l]) + k * u2f(b[l].wrapping_sub(a[l]))).to_i32() as u32;
        }
    }

    /// Clip at a z plane (`w`: its w, `z`: the z written) for outcode bit
    /// `1 << sh`: t along w, the vertex factor t · w1 / w; the clipped end's
    /// x / y outcode. False when the segment is then off one side.
    fn clip_z(&mut self, b: &Bounds, sh: u32, z: FpuF32, w: FpuF32) -> bool {
        let (p0, p1) = (self.p[0].map(FpuF32), self.p[1].map(FpuF32));
        let t = (p0[3] - w).div(p0[3] - p1[3]);
        let i = (((self.code[0] as i32 >> sh) ^ 1) & 1) as usize;
        let dx = p0[0] - p1[0];
        let dy = p0[1] - p1[1];
        let k = (t * p1[3]).div(w);
        let o: V = [(p0[0] - t * dx).0, (p0[1] - t * dy).0, z.0, w.0];
        self.p[i] = o;
        self.lerp(i, k);
        self.code[i] = b.code_xy(&o, true);
        self.code[0] & self.code[1] == 0
    }

    /// Clip at y = `y` for bit `1 << sh`; the clipped end's x outcode.
    fn clip_y(&mut self, b: &Bounds, sh: u32, y: FpuF32) -> bool {
        let (p0, p1) = (self.p[0].map(FpuF32), self.p[1].map(FpuF32));
        let t = (p0[1] - y).div(p0[1] - p1[1]);
        let i = (((self.code[0] as i32 >> sh) ^ 1) & 1) as usize;
        let dx = p0[0] - p1[0];
        let dz = p0[2] - p1[2];
        let dw = p0[3] - p1[3];
        let o: V = [(p0[0] - t * dx).0, y.0, (p0[2] - t * dz).0, (p0[3] - t * dw).0];
        self.p[i] = o;
        self.lerp(i, t);
        self.code[i] = b.code_xy(&o, false);
        self.code[0] & self.code[1] == 0
    }

    /// Clip at x = `x` for bit `1 << sh`; the clipped end's outcode 0.
    fn clip_x(&mut self, sh: u32, x: FpuF32) {
        let (p0, p1) = (self.p[0].map(FpuF32), self.p[1].map(FpuF32));
        let t = (p0[0] - x).div(p0[0] - p1[0]);
        let i = (((self.code[0] as i32 >> sh) ^ 1) & 1) as usize;
        let dy = p0[1] - p1[1];
        let dz = p0[2] - p1[2];
        let dw = p0[3] - p1[3];
        let o: V = [x.0, (p0[1] - t * dy).0, (p0[2] - t * dz).0, (p0[3] - t * dw).0];
        self.p[i] = o;
        self.lerp(i, t);
        self.code[i] = 0;
    }

    /// `PS2GraphicsMan_ClipLineSegment(gm, verts, proj, codes)` (0x383a10):
    /// near (0x20), far (0x10), y min (8), y max (4) with a reject test
    /// after each, then x min (2), x max (1) by the outcodes from before
    /// them. False: rejected.
    fn clip(&mut self, b: &Bounds) -> bool {
        let mut u = self.code[0] | self.code[1];
        if u & 0x20 != 0 {
            if !self.clip_z(b, 5, b.z78, b.w7c) {
                return false;
            }
            u = self.code[0] | self.code[1];
        }
        if u & 0x10 != 0 {
            if !self.clip_z(b, 4, b.z88, b.w8c) {
                return false;
            }
            u = self.code[0] | self.code[1];
        }
        if u & 8 != 0 {
            if !self.clip_y(b, 3, b.y74) {
                return false;
            }
            u = self.code[0] | self.code[1];
        }
        if u & 4 != 0 {
            if !self.clip_y(b, 2, b.y84) {
                return false;
            }
            u = self.code[0] | self.code[1];
        }
        if u & 2 != 0 {
            self.clip_x(1, b.x70);
        }
        if u & 1 != 0 {
            self.clip_x(0, b.x80);
        }
        true
    }
}

/// The `LineStrip` builder's GIF data (from the packet's qw2), as dwords:
/// per strip a 3-qword header, then 3 dwords per vertex.
struct Strips {
    data: Vec<u64>,
    /// The open strip's header (dword index); none before the first.
    header: Option<usize>,
    count: u32,
}

impl Strips {
    /// `LineStrip_AddVertex(ls, v, s)`: RGBAQ (`R | G<<8 | B<<16 | A<<24`,
    /// Q = s.q), ST (S·q, T·q), XYZ2 (int x, int y, int(z·q)).
    fn add(&mut self, v: &Vert, s: &V) {
        if self.header.is_none() {
            return;
        }
        let q = FpuF32(s[3]);
        let rgbaq = v[4] as u64 | (v[5] as u64) << 8 | (v[6] as u64) << 16 | (v[7] as u64) << 24 | (s[3] as u64) << 32;
        let stq = sx64((FpuF32(v[0]) * q).0) | ((FpuF32(v[1]) * q).0 as u64) << 32;
        let xyz = int(FpuF32(s[0])) | int(FpuF32(s[1])) << 16 | int(FpuF32(s[2]) * q) << 32;
        self.data.extend([rgbaq, stq, xyz]);
        self.count += 1;
    }

    /// `LineStrip_Begin(ls, v, s)`: close the open strip, open one, its
    /// first vertex.
    fn begin(&mut self, v: &Vert, s: &V) {
        if self.header.is_some() {
            self.close();
        }
        self.header = Some(self.data.len());
        self.data.extend([0; 6]);
        self.count = 0;
        self.add(v, s);
    }

    /// `LineStrip_Close`: an odd count padded with a zero dword; the header
    /// {GIFtag NLOOP 1 REGLIST PRIM, PRIM 0x4a, GIFtag EOP NLOOP count
    /// REGLIST RGBAQ ST XYZ2}.
    fn close(&mut self) {
        let Some(h) = self.header else { return };
        if self.count & 1 != 0 {
            self.data.push(0);
        }
        let tag = self.count as u64 | 0x8000 | 0xd000 << 46;
        self.data[h..h + 6].copy_from_slice(&[0x1400_0000_0000_0001, 0, 0x4a, 0, tag, 0x521]);
    }

    /// `LineStrip_Finish(ls, 2)`: the DIRECT data, or none when no strip
    /// was begun (no packet, no record).
    fn finish(mut self) -> Option<Vec<u128>> {
        self.header?;
        self.close();
        Some(self.data.chunks_exact(2).map(|d| qw(d[0], d[1])).collect())
    }
}

/// `PS2GraphicsMan_DrawLineStrip(gm, n, verts)` (slot 73, 0x381ad0): `n`
/// vertices (0x30 each) projected through gm+0x6b30 (x, y divided by w,
/// q = 1/w), outcoded, the segments that leave or enter the bounds
/// clipped (a strip begins at each entry); all strips one DIRECT block in
/// one record (`LineStrip_Finish` with the graphics manager at
/// gp+0x2a90). The cursor is `gm`'s.
pub fn draw_line_strip(c: &mut Cpu) {
    let (gm, n, verts) = (c.g[A0][0] as u32, c.g[A1][0] as i32, c.g[A2][0] as u32);
    ensure_view_matrices(c, gm, 0x0038_1b18);
    let b = Bounds::read(c, gm);
    let m = qm(c, gm + 0x6b30);
    let mut ls = Strips { data: Vec::new(), header: None, count: 0 };
    let mut prev_code = u32::MAX;
    let mut prev_s: V = [0; 4];
    let mut v = verts;
    for i in 0..n.max(0) {
        let cur = vert(c, v);
        let p = vu0::xform(&m, &qv(c, v + 0x20));
        let q = FpuF32::ONE.div(FpuF32(p[3]));
        let s: V = [(FpuF32(p[0]) * q).0, (FpuF32(p[1]) * q).0, p[2], q.0];
        let code = b.code(&s);
        if code == 0 {
            if prev_code == 0 {
                ls.add(&cur, &s);
            } else if i == 0 {
                ls.begin(&cur, &s);
            } else {
                // Entering: the clipped previous end begins a strip (the
                // clip's result is not looked at).
                let mut seg = Seg { v: [vert(c, v.wrapping_sub(0x30)), cur], p: [prev_s, s], code: [prev_code, 0] };
                seg.clip(&b);
                ls.begin(&seg.v[0], &seg.p[0]);
                ls.add(&cur, &s);
            }
        } else if prev_code == 0 {
            // Leaving: the clipped end (the result not looked at).
            let mut seg = Seg { v: [vert(c, v.wrapping_sub(0x30)), cur], p: [prev_s, s], code: [0, code] };
            seg.clip(&b);
            ls.add(&seg.v[1], &seg.p[1]);
        } else if prev_code & code == 0 {
            // Both out, maybe crossing: a strip of the clipped segment.
            let mut seg = Seg { v: [vert(c, v.wrapping_sub(0x30)), cur], p: [prev_s, s], code: [prev_code, code] };
            if seg.clip(&b) {
                ls.begin(&seg.v[0], &seg.p[0]);
                ls.add(&seg.v[1], &seg.p[1]);
            }
        }
        prev_code = code;
        prev_s = s;
        v = v.wrapping_add(0x30);
    }
    if let Some(gif) = ls.finish() {
        let gm_rec = r32(c, c.g[GP][0] as u32 + 0x2a90);
        let start = empty_packet(c, gm);
        record(start, Cmd::Vu(vec![Step::Gif(gif)]));
        add_record(c, gm_rec, start, start, false);
    }
    ret(c, 80 + 90 * n.max(0) as u64);
}

/// Slot 75 `PS2GraphicsMan_DrawLineShape(gm, matrix, argb, n)` (0x381f10,
/// debug paths): a line shape through the type 3 program at 0x3bf0: its
/// GIF tags, colour (× 128), the matrix and the count to VU1 0.
pub fn draw_line_shape(c: &mut Cpu) {
    let (gm, matrix, argb, n) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32, c.g[A3][0]);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0xc0);
    state_word(c, st, 3, 0xc000_0000, 0);
    ensure_view_matrices(c, gm, 0x0038_1f74);
    let k = FpuF32(0x4300_0000);
    let ch = |c: &mut Cpu, o: u32| (FpuF32(r32(c, argb + o)) * k).to_i32() as i64 as u64;
    let (r, g, b, a) = (ch(c, 4), ch(c, 8), ch(c, 0xc), ch(c, 0));
    let rgbaq = r | g << 8 | b << 16 | a << 24 | 0x3f80_0000_0000_0000;
    let mut data = vec![0x1001_4000_0000_0001 | 0xe << 64, rgbaq as u128 | 1 << 64, (n | 0x1001_4000_0000_8000) as u128 | 4 << 64];
    data.extend(qwords(c, matrix, 4));
    data.push(n as u128);
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(vec![Step::Write { addr: 0, rel: true, data }, Step::Start(0x3bf0)]));
    add_record(c, gm, start, start, false);
    ret(c, 80);
}
