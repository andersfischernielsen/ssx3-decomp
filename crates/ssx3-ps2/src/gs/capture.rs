//! GS trace capture and replay.
//!
//! `Gs::start_capture` snapshots the complete GS state (VRAM, registers, GIF
//! path state, CLUT, transfer state) and then logs every externally driven
//! call (`gif_write`, `write_reg`, `write_priv`, `vblank`, `read_local`,
//! `raise_event`) into memory. `Gs::finish_capture` returns the trace as a
//! self-contained byte string that `Trace::parse` turns back into a GS plus
//! the event list, so a captured scene can be replayed offline
//! (`gs_replay` binary) to benchmark and verify the rasterizer.
//!
//! Format (little endian): magic `GSTR`, version u32, snapshot length u64,
//! snapshot bytes, then events, each a tag byte followed by its payload.

use super::{gif::GifPath, transfer::Xfer, Ctx, Gs, Priv, Regs, Vertex};
use crate::Qword;

const MAGIC: &[u8; 4] = b"GSTR";
const VERSION: u32 = 1;

const EV_GIF: u8 = 1;
const EV_REG: u8 = 2;
const EV_PRIV: u8 = 3;
const EV_VBLANK: u8 = 4;
const EV_READ_LOCAL: u8 = 5;
const EV_RAISE: u8 = 6;

/// One recorded call into the GS.
#[derive(Clone, Debug)]
pub enum Event {
    Gif { path: u8, data: Vec<Qword> },
    Reg { addr: u8, value: u64 },
    Priv { offset: u32, value: u64 },
    VBlank { field: u32 },
    ReadLocal { max: u32 },
    Raise { bit: u64 },
}

impl Event {
    /// Re-issue this call on `gs`.
    pub fn apply(&self, gs: &mut Gs) {
        match self {
            Event::Gif { path, data } => gs.gif_write(*path as usize, data),
            Event::Reg { addr, value } => gs.write_reg(*addr, *value),
            Event::Priv { offset, value } => gs.write_priv(*offset, *value),
            Event::VBlank { field } => gs.vblank(*field),
            Event::ReadLocal { max } => {
                gs.read_local(*max as usize);
            }
            Event::Raise { bit } => gs.raise_event(*bit),
        }
    }
}

/// A parsed trace: the GS as it was when capture started, and what followed.
pub struct Trace {
    pub gs: Gs,
    pub events: Vec<Event>,
}

/// Recording in progress.
pub(crate) struct Recorder {
    pub buf: Vec<u8>,
}

// ------------------------------------------------------------------ writer

struct W<'a>(&'a mut Vec<u8>);

impl W<'_> {
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    fn u32(&mut self, v: u32) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn u128(&mut self, v: u128) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn f32(&mut self, v: f32) {
        self.u32(v.to_bits());
    }
    fn bool(&mut self, v: bool) {
        self.u8(v as u8);
    }
}

struct R<'a> {
    b: &'a [u8],
    pos: usize,
}

impl R<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.pos.checked_add(n).filter(|&e| e <= self.b.len());
        let end = end.ok_or_else(|| format!("trace truncated at byte {}", self.pos))?;
        let s = &self.b[self.pos..end];
        self.pos = end;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn u128(&mut self) -> Result<u128, String> {
        Ok(u128::from_le_bytes(self.take(16)?.try_into().unwrap()))
    }
    fn f32(&mut self) -> Result<f32, String> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn bool(&mut self) -> Result<bool, String> {
        Ok(self.u8()? != 0)
    }
}

// ------------------------------------------------------------------ state

fn put_ctx(w: &mut W, c: &Ctx) {
    for v in [
        c.tex0, c.clamp, c.tex1, c.xyoffset, c.miptbp1, c.miptbp2, c.scissor, c.alpha, c.test,
        c.fba, c.frame, c.zbuf,
    ] {
        w.u64(v);
    }
}

fn get_ctx(r: &mut R) -> Result<Ctx, String> {
    Ok(Ctx {
        tex0: r.u64()?,
        clamp: r.u64()?,
        tex1: r.u64()?,
        xyoffset: r.u64()?,
        miptbp1: r.u64()?,
        miptbp2: r.u64()?,
        scissor: r.u64()?,
        alpha: r.u64()?,
        test: r.u64()?,
        fba: r.u64()?,
        frame: r.u64()?,
        zbuf: r.u64()?,
    })
}

fn put_regs(w: &mut W, g: &Regs) {
    for v in [g.prim, g.rgbaq, g.st, g.uv, g.fog] {
        w.u64(v);
    }
    put_ctx(w, &g.ctx[0]);
    put_ctx(w, &g.ctx[1]);
    for v in [
        g.prmodecont,
        g.prmode,
        g.texclut,
        g.scanmsk,
        g.texa,
        g.fogcol,
        g.dimx,
        g.dthe,
        g.colclamp,
        g.pabe,
        g.bitbltbuf,
        g.trxpos,
        g.trxreg,
        g.trxdir,
    ] {
        w.u64(v);
    }
}

fn get_regs(r: &mut R) -> Result<Regs, String> {
    Ok(Regs {
        prim: r.u64()?,
        rgbaq: r.u64()?,
        st: r.u64()?,
        uv: r.u64()?,
        fog: r.u64()?,
        ctx: [get_ctx(r)?, get_ctx(r)?],
        prmodecont: r.u64()?,
        prmode: r.u64()?,
        texclut: r.u64()?,
        scanmsk: r.u64()?,
        texa: r.u64()?,
        fogcol: r.u64()?,
        dimx: r.u64()?,
        dthe: r.u64()?,
        colclamp: r.u64()?,
        pabe: r.u64()?,
        bitbltbuf: r.u64()?,
        trxpos: r.u64()?,
        trxreg: r.u64()?,
        trxdir: r.u64()?,
    })
}

fn put_priv(w: &mut W, p: &Priv) {
    for v in [
        p.pmode,
        p.smode1,
        p.smode2,
        p.srfsh,
        p.synch1,
        p.synch2,
        p.syncv,
        p.dispfb[0],
        p.dispfb[1],
        p.display[0],
        p.display[1],
        p.extbuf,
        p.extdata,
        p.extwrite,
        p.bgcolor,
        p.csr,
        p.imr,
        p.busdir,
        p.siglblid,
    ] {
        w.u64(v);
    }
}

fn get_priv(r: &mut R) -> Result<Priv, String> {
    Ok(Priv {
        pmode: r.u64()?,
        smode1: r.u64()?,
        smode2: r.u64()?,
        srfsh: r.u64()?,
        synch1: r.u64()?,
        synch2: r.u64()?,
        syncv: r.u64()?,
        dispfb: [r.u64()?, r.u64()?],
        display: [r.u64()?, r.u64()?],
        extbuf: r.u64()?,
        extdata: r.u64()?,
        extwrite: r.u64()?,
        bgcolor: r.u64()?,
        csr: r.u64()?,
        imr: r.u64()?,
        busdir: r.u64()?,
        siglblid: r.u64()?,
    })
}

fn put_vertex(w: &mut W, v: &Vertex) {
    w.u32(v.x);
    w.u32(v.y);
    w.u32(v.z);
    w.u8(v.f);
    for c in v.rgba {
        w.u8(c);
    }
    w.f32(v.s);
    w.f32(v.t);
    w.f32(v.q);
    w.u32(v.u);
    w.u32(v.v);
}

fn get_vertex(r: &mut R) -> Result<Vertex, String> {
    Ok(Vertex {
        x: r.u32()?,
        y: r.u32()?,
        z: r.u32()?,
        f: r.u8()?,
        rgba: [r.u8()?, r.u8()?, r.u8()?, r.u8()?],
        s: r.f32()?,
        t: r.f32()?,
        q: r.f32()?,
        u: r.u32()?,
        v: r.u32()?,
    })
}

fn put_path(w: &mut W, p: &GifPath) {
    w.bool(p.active);
    w.u32(p.nloop);
    w.u32(p.loop_);
    w.u32(p.nreg);
    w.u32(p.reg);
    w.u32(p.flg);
    w.u64(p.regs);
    w.bool(p.eop);
}

fn get_path(r: &mut R) -> Result<GifPath, String> {
    Ok(GifPath {
        active: r.bool()?,
        nloop: r.u32()?,
        loop_: r.u32()?,
        nreg: r.u32()?,
        reg: r.u32()?,
        flg: r.u32()?,
        regs: r.u64()?,
        eop: r.bool()?,
    })
}

/// Serialize the complete GS state.
fn snapshot(gs: &Gs) -> Vec<u8> {
    let mut buf = Vec::with_capacity(gs.vram.len() * 4 + 4096);
    let mut w = W(&mut buf);
    w.u32(gs.vram.len() as u32);
    for &v in &gs.vram {
        w.u32(v);
    }
    put_priv(&mut w, &gs.pr);
    put_regs(&mut w, &gs.regs);
    w.f32(gs.internal_q);
    for v in &gs.vq {
        put_vertex(&mut w, v);
    }
    w.u32(gs.vcount as u32);
    for &c in &gs.clut {
        w.0.extend_from_slice(&c.to_le_bytes());
    }
    w.u32(gs.cbp0);
    w.u32(gs.cbp1);
    w.u64(gs.clut_gen);
    for p in &gs.paths {
        put_path(&mut w, p);
    }
    let x = &gs.xfer;
    w.bool(x.active);
    w.u32(x.x);
    w.u32(x.y);
    w.0.extend_from_slice(&x.carry);
    w.u32(x.ncarry as u32);
    w.u32(gs.readback.len() as u32);
    for &q in &gs.readback {
        w.u128(q);
    }
    w.u64(gs.prims_drawn);
    buf
}

fn restore(r: &mut R) -> Result<Gs, String> {
    // Always the CPU backend; `Gs::set_backend` switches (uploading VRAM).
    let mut gs = Gs::new_cpu();
    let n = r.u32()? as usize;
    if n != gs.vram.len() {
        return Err(format!("unexpected VRAM size {n}"));
    }
    let bytes = r.take(n * 4)?;
    for (d, c) in gs.vram.iter_mut().zip(bytes.chunks_exact(4)) {
        *d = u32::from_le_bytes(c.try_into().unwrap());
    }
    gs.pr = get_priv(r)?;
    gs.regs = get_regs(r)?;
    gs.internal_q = r.f32()?;
    for i in 0..3 {
        gs.vq[i] = get_vertex(r)?;
    }
    gs.vcount = r.u32()? as usize;
    for i in 0..gs.clut.len() {
        let b = r.take(2)?;
        gs.clut[i] = u16::from_le_bytes([b[0], b[1]]);
    }
    gs.cbp0 = r.u32()?;
    gs.cbp1 = r.u32()?;
    gs.clut_gen = r.u64()?;
    for i in 0..gs.paths.len() {
        gs.paths[i] = get_path(r)?;
    }
    gs.xfer = Xfer {
        active: r.bool()?,
        x: r.u32()?,
        y: r.u32()?,
        carry: r.take(4)?.try_into().unwrap(),
        ncarry: r.u32()? as usize,
    };
    let n = r.u32()? as usize;
    for _ in 0..n {
        gs.readback.push_back(r.u128()?);
    }
    gs.prims_drawn = r.u64()?;
    Ok(gs)
}

// ------------------------------------------------------------------ recording

impl Recorder {
    pub(crate) fn gif(&mut self, path: usize, data: &[Qword]) {
        let mut w = W(&mut self.buf);
        w.u8(EV_GIF);
        w.u8(path as u8);
        w.u32(data.len() as u32);
        for &q in data {
            w.u128(q);
        }
    }
    pub(crate) fn reg(&mut self, addr: u8, value: u64) {
        let mut w = W(&mut self.buf);
        w.u8(EV_REG);
        w.u8(addr);
        w.u64(value);
    }
    pub(crate) fn privileged(&mut self, offset: u32, value: u64) {
        let mut w = W(&mut self.buf);
        w.u8(EV_PRIV);
        w.u32(offset);
        w.u64(value);
    }
    pub(crate) fn vblank(&mut self, field: u32) {
        let mut w = W(&mut self.buf);
        w.u8(EV_VBLANK);
        w.u32(field);
    }
    pub(crate) fn read_local(&mut self, max: usize) {
        let mut w = W(&mut self.buf);
        w.u8(EV_READ_LOCAL);
        w.u32(max.min(u32::MAX as usize) as u32);
    }
    pub(crate) fn raise(&mut self, bit: u64) {
        let mut w = W(&mut self.buf);
        w.u8(EV_RAISE);
        w.u64(bit);
    }
}

impl Gs {
    /// Snapshot the GS and start logging every call into it (see `capture`).
    pub fn start_capture(&mut self) {
        self.sync();
        self.sync_vram();
        let snap = snapshot(self);
        let mut buf = Vec::with_capacity(snap.len() + (1 << 20));
        let mut w = W(&mut buf);
        w.0.extend_from_slice(MAGIC);
        w.u32(VERSION);
        w.u64(snap.len() as u64);
        buf.extend_from_slice(&snap);
        self.capture = Some(Box::new(Recorder { buf }));
    }

    /// Whether a capture is in progress.
    pub fn capturing(&self) -> bool {
        self.capture.is_some()
    }

    /// Stop capturing and return the serialized trace (empty if none was running).
    pub fn finish_capture(&mut self) -> Vec<u8> {
        self.capture.take().map(|r| r.buf).unwrap_or_default()
    }
}

impl Trace {
    pub fn parse(bytes: &[u8]) -> Result<Trace, String> {
        let mut r = R { b: bytes, pos: 0 };
        if r.take(4)? != MAGIC {
            return Err("not a GS trace".into());
        }
        let ver = r.u32()?;
        if ver != VERSION {
            return Err(format!("unsupported trace version {ver}"));
        }
        let snap_len = r.u64()? as usize;
        let snap = r.take(snap_len)?;
        let gs = restore(&mut R { b: snap, pos: 0 })?;
        let mut events = Vec::new();
        while r.pos < bytes.len() {
            let ev = match r.u8()? {
                EV_GIF => {
                    let path = r.u8()?;
                    let n = r.u32()? as usize;
                    let mut data = Vec::with_capacity(n);
                    for _ in 0..n {
                        data.push(r.u128()?);
                    }
                    Event::Gif { path, data }
                }
                EV_REG => Event::Reg {
                    addr: r.u8()?,
                    value: r.u64()?,
                },
                EV_PRIV => Event::Priv {
                    offset: r.u32()?,
                    value: r.u64()?,
                },
                EV_VBLANK => Event::VBlank { field: r.u32()? },
                EV_READ_LOCAL => Event::ReadLocal { max: r.u32()? },
                EV_RAISE => Event::Raise { bit: r.u64()? },
                t => return Err(format!("bad event tag {t} at byte {}", r.pos - 1)),
            };
            events.push(ev);
        }
        Ok(Trace { gs, events })
    }
}
