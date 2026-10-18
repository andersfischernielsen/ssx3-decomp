//! GS register state and primitive assembly from GIF packets, producing
//! hardware-friendly triangle batches (no GS rasterization semantics beyond
//! what the GPU does natively).
//!
//! The GIF formats (PACKED, REGLIST, IMAGE) and register layouts follow the
//! GS User's Manual. Primitives are expanded on the CPU into triangle lists
//! in target pixel coordinates; flat shading takes the last vertex's colour
//! (the GS's provoking vertex), sprites become two triangles.

/// One vertex as the shaders see it.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Vtx {
    /// Target pixels (GS sample points at integer coordinates; the shader
    /// adds the half-pixel), depth in 0..1.
    pub pos: [f32; 3],
    pub fog: f32,
    /// RGBA in GS units (0..255, 0x80 = 1.0 for alpha and modulation).
    pub col: [f32; 4],
    /// S, T, Q (normalized texture coordinates × Q).
    pub stq: [f32; 4],
}

// The renderer uploads `Vtx` slices as raw bytes (twelve f32s, no padding).
const _: () = assert!(std::mem::size_of::<Vtx>() == 48);

/// `DrawState::tex` for primitives that sample the frame being drawn.
pub const FRAME_TEX: u32 = u32::MAX;
/// `DrawState::tex` for a render target: this bit | its FBP.
pub const RT_TEX: u32 = 0x8000_0000;

/// `DrawState::tex` for the depth-fog pass: this bit | the index of its
/// CLUT in `Gs::fog_cluts` (the upload at its CBP when the pass was drawn:
/// the same VRAM is reused for other uploads later in the frame). The game first copies Z
/// bits 8..15 into the frame's alpha (a 16-bit view of the frame drawn
/// from a 16-bit view of Z), then draws the frame's alpha as a PSMT8H
/// texture through that CLUT (fog colour and amount per depth). Here the
/// copy is dropped and the second pass reads the depth buffer directly.
pub const ZFOG_TEX: u32 = 0x4000_0000;

/// A small host→VRAM upload (CLUTs), as PSMCT32 words in transfer order.
#[derive(Clone, Debug, Default)]
pub struct Upload {
    pub psm: u32,
    pub w: u32,
    pub h: u32,
    pub data: Vec<u32>,
}

/// An offscreen frame buffer the game renders into (rider shadows, glow):
/// width from the largest FBW, height from the largest scissor seen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rt {
    pub w: u32,
    pub h: u32,
    pub psm: u32,
}

/// Per-context state that changes how a primitive is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Ctx {
    pub tex0: u64,
    pub tex1: u64,
    pub clamp: u64,
    pub xyoffset: u64,
    pub scissor: u64,
    pub alpha: u64,
    pub test: u64,
    pub fba: u64,
    pub frame: u64,
    pub zbuf: u64,
}

/// What a batch of triangles is drawn with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DrawState {
    /// PRIM bits that matter: TME, FGE, ABE, FST (as in PRIM).
    pub prim: u32,
    pub ctx: Ctx,
    pub texa: u64,
    pub fogcol: u64,
    /// TexMan texture for TEX0 (resolved by the caller), or none.
    pub tex: Option<u32>,
    /// FRAME.FBP drawn into (the main buffer or a render target).
    pub target: u32,
}

#[derive(Clone)]
pub struct Batch {
    pub state: DrawState,
    pub first: u32,
    pub count: u32,
    /// Vertices computed by the GPU (`ssx3_render::gpu`): `first`/`count`
    /// are unused.
    pub gpu: Option<GpuBatch>,
}

/// A batch of GPU-computed vertices: one call's triangle list.
#[derive(Clone, Copy, Debug)]
pub struct GpuBatch {
    pub program: ssx3_render::gpu::Program,
    /// The call's index in [`GpuFrame::calls`].
    pub call: u32,
    /// Its indices in [`GpuFrame::indices`].
    pub first: u32,
    pub count: u32,
    /// The largest vertex alpha (GS units).
    pub alpha_max: f32,
}

/// The GPU-computed draws of a frame: per-call uniforms, vertex inputs and
/// triangle lists (indices relative to the call's data).
#[derive(Clone, Default)]
pub struct GpuFrame {
    pub calls: Vec<[[u32; 4]; ssx3_render::gpu::UNIFORM_QWORDS]>,
    pub data: Vec<[u32; 4]>,
    pub indices: Vec<u32>,
    /// `Link::Source` calls by key: their first data qword, alpha maximum.
    pub sources: std::collections::HashMap<u32, (u32, f32)>,
}

/// GS-side uniform qwords of a GPU call (`gpuvu.wgsl` `Call`).
pub mod gpu_params {
    use ssx3_render::gpu::GS_PARAMS;
    /// x, y: XYOFFSET / 16 (pixels); z: 1 / (ZMAX + 1); w: ZMAX.
    pub const OFFSET: usize = GS_PARAMS;
    /// x, y: texture coordinate scale (render-target textures); z: HUD
    /// squeeze factor (1 = none); w: its centre x.
    pub const TEX: usize = GS_PARAMS + 1;
    /// x: FOG (0..255); y: the call's first data qword; z: the first data
    /// qword of the call whose vertices it reuses (`Link::Reuse`); w: the
    /// tag's vertex register (4 XYZF2: Z / 16 and per-vertex fog; 5 XYZ2).
    pub const MISC: usize = GS_PARAMS + 2;
}

#[derive(Clone, Default)]
pub struct Gs {
    pub ctx: [Ctx; 2],
    prim: u64,
    prmode: u64,
    prmodecont: u64,
    rgbaq: [u8; 4],
    q: f32,
    s: f32,
    t: f32,
    uv: (u32, u32),
    fog: u8,
    pub texa: u64,
    pub fogcol: u64,
    /// Kicked vertices of the current primitive.
    queue: Vec<QVtx>,
    pub verts: Vec<Vtx>,
    pub batches: Vec<Batch>,
    /// GPU-computed draws (`gpu_draw`).
    pub gpu: GpuFrame,
    /// Primitives dropped (unsupported target, unknown texture, ...).
    pub dropped: u64,
    pub prims: u64,
    /// Frame buffer this renderer draws (FRAME.FBP), Z buffer (ZBUF.ZBP).
    pub main_fbp: u32,
    pub main_zbp: u32,
    pub size: (u32, u32),
    /// IMAGE transfers in progress (skipped unless captured).
    image_left: u32,
    bitbltbuf: u64,
    trxreg: u64,
    /// The upload being captured (DBP, data so far).
    capture: Option<(u32, Upload)>,
    /// Small PSMCT32 uploads by DBP (CLUTs), kept across frames.
    pub uploads: std::collections::HashMap<u32, Upload>,
    /// The frame's alpha holds Z bits 8..15 (the depth-fog copy ran).
    zalpha: bool,
    /// The primitives now coming are HUD (squeezed horizontally by
    /// `hud_squeeze` around the frame's centre, main target only).
    pub hud: bool,
    pub hud_squeeze: f32,
    /// CLUTs of this frame's depth-fog passes (see ZFOG_TEX).
    pub fog_cluts: Vec<Upload>,
    /// XYOFFSET_1/2 written: the game sets context 2's once at startup,
    /// outside any frame, so until then it follows context 1.
    xyoffset_set: [bool; 2],
    /// Texture base pointer (TEX0.TBP0) → TexInfo, kept current by the
    /// renderer as it walks the frame.
    pub tex_map: std::collections::HashMap<u32, u32>,
    /// Textured primitives whose TBP is not in `tex_map`.
    pub unknown_tex: u64,
    /// Debug trace (`SSX3_GFX_TRACE`): label of what is being processed,
    /// and per (label, outcome, PRIM type, FBP, TBP, ALPHA, TEST, rgba)
    /// the primitive count.
    pub trace: Option<String>,
    pub trace_log: Vec<String>,
    /// Render targets by FBP (kept across frames).
    pub rts: std::collections::HashMap<u32, Rt>,
    /// Primitives for other frame buffers: (FBP, FBW, PSM) → count.
    pub dropped_fbp: std::collections::HashMap<(u32, u32, u32), u64>,
}

#[derive(Clone, Copy, Debug)]
struct QVtx {
    x: f32,
    y: f32,
    z: u32,
    rgba: [u8; 4],
    s: f32,
    t: f32,
    q: f32,
    u: u32,
    v: u32,
    fog: u8,
}

fn bits(v: u64, lo: u32, n: u32) -> u64 {
    (v >> lo) & ((1u64 << n) - 1)
}

impl Gs {
    pub fn new() -> Gs {
        // Reset values: PRMODECONT.AC = 1 (attributes from PRIM).
        Gs { texa: 0x80 << 32, prmodecont: 1, ..Gs::default() }
    }

    /// Start a frame: drop the previous frame's geometry.
    pub fn begin(&mut self, size: (u32, u32), fbp: u32, zbp: u32) {
        self.verts.clear();
        self.batches.clear();
        self.gpu.calls.clear();
        self.gpu.data.clear();
        self.gpu.indices.clear();
        self.gpu.sources.clear();
        self.queue.clear();
        self.size = size;
        self.main_fbp = fbp;
        self.main_zbp = zbp;
        self.image_left = 0;
        self.capture = None;
        self.zalpha = false;
        self.fog_cluts.clear();
    }

    /// IMAGE data for the transfer in progress.
    fn image_data(&mut self, q: &[u128]) {
        let Some((dbp, up)) = self.capture.as_mut() else { return };
        for v in q {
            for k in 0..4 {
                up.data.push((v >> (32 * k)) as u32);
            }
        }
        if up.data.len() >= (up.w * up.h) as usize {
            up.data.truncate((up.w * up.h) as usize);
            let (dbp, up) = (*dbp, std::mem::take(up));
            self.uploads.insert(dbp, up);
            self.capture = None;
        }
    }

    fn cur_prim(&self) -> u64 {
        // PRMODECONT.AC = 1: attributes from PRIM, else from PRMODE.
        if self.prmodecont & 1 != 0 {
            self.prim
        } else {
            (self.prim & 7) | (self.prmode & !7)
        }
    }

    /// A GIF packet (one or more tags with their data).
    pub fn gif(&mut self, qwords: &[u128]) {
        let mut i = 0;
        while i < qwords.len() {
            if self.image_left > 0 {
                let n = (self.image_left as usize).min(qwords.len() - i);
                self.image_left -= n as u32;
                self.image_data(&qwords[i..i + n]);
                i += n;
                continue;
            }
            let tag = qwords[i];
            i += 1;
            let lo = tag as u64;
            let nloop = bits(lo, 0, 15) as usize;
            let pre = bits(lo, 46, 1) != 0;
            let prim = bits(lo, 47, 11);
            let flg = bits(lo, 58, 2);
            let nreg = match bits(lo, 60, 4) {
                0 => 16,
                n => n as usize,
            };
            let regs = (tag >> 64) as u64;
            if pre && flg != 2 {
                self.write_reg(0, prim);
            }
            match flg {
                0 => {
                    for _ in 0..nloop {
                        for r in 0..nreg {
                            if i >= qwords.len() {
                                return;
                            }
                            let d = qwords[i];
                            i += 1;
                            self.packed(((regs >> (4 * r)) & 15) as u8, d);
                        }
                    }
                }
                1 => {
                    let total = nloop * nreg;
                    let words = total.div_ceil(2);
                    for k in 0..total {
                        let q = i + k / 2;
                        if q >= qwords.len() {
                            return;
                        }
                        let v = (qwords[q] >> (64 * (k & 1))) as u64;
                        let r = ((regs >> (4 * (k % nreg))) & 15) as u8;
                        if r != 0xe {
                            self.write_reg(r as u32, v);
                        }
                    }
                    i += words;
                }
                _ => {
                    // IMAGE (host→local uploads): textures come from TexMan.
                    let n = nloop.min(qwords.len() - i);
                    self.image_left = (nloop - n) as u32;
                    self.image_data(&qwords[i..i + n]);
                    i += n;
                }
            }
        }
    }

    fn packed(&mut self, r: u8, d: u128) {
        let w = |k: u32| (d >> (32 * k)) as u32;
        match r {
            0x0 => self.write_reg(0, d as u64 & 0x7ff),
            0x1 => {
                self.rgbaq = [w(0) as u8, w(1) as u8, w(2) as u8, w(3) as u8];
            }
            0x2 => {
                self.s = f32::from_bits(w(0));
                self.t = f32::from_bits(w(1));
                self.q = f32::from_bits(w(2));
            }
            0x3 => self.uv = (w(0) & 0x3fff, w(1) & 0x3fff),
            0x4 => {
                let adc = (d >> 111) & 1 != 0;
                let (x, y, z, f) = (w(0) & 0xffff, w(1) & 0xffff, (w(2) >> 4) & 0xff_ffff, (w(3) >> 4) & 0xff);
                self.fog = f as u8;
                self.vertex(x, y, z, !adc);
            }
            0x5 => {
                let adc = (d >> 111) & 1 != 0;
                self.vertex(w(0) & 0xffff, w(1) & 0xffff, w(2), !adc);
            }
            0xa => self.fog = (w(3) >> 4) as u8,
            0xc => {
                let (x, y, z, f) = (w(0) & 0xffff, w(1) & 0xffff, (w(2) >> 4) & 0xff_ffff, (w(3) >> 4) & 0xff);
                self.fog = f as u8;
                self.vertex(x, y, z, false);
            }
            0xd => self.vertex(w(0) & 0xffff, w(1) & 0xffff, w(2), false),
            0xe => {
                let a = ((d >> 64) & 0xff) as u32;
                self.write_reg(a, d as u64);
            }
            0xf => {}
            // TEX0_1/2, CLAMP_1/2 by their register numbers.
            0x6 | 0x7 | 0x8 | 0x9 => self.write_reg(r as u32, d as u64),
            _ => {}
        }
    }

    pub fn write_reg(&mut self, a: u32, v: u64) {
        if matches!(a, 0x50..=0x53) {
            if let Some(label) = &self.trace {
                self.trace_log.push(format!("{label} reg {a:#x} = {v:#x}"));
            }
        }
        match a {
            0x50 => self.bitbltbuf = v,
            0x52 => self.trxreg = v,
            0x53 => {
                // Host→local, PSMCT32, small: capture (CLUTs).
                let (dbp, dpsm) = (bits(self.bitbltbuf, 32, 14) as u32, bits(self.bitbltbuf, 56, 6) as u32);
                let (w, h) = (bits(self.trxreg, 0, 12) as u32, bits(self.trxreg, 32, 12) as u32);
                self.capture = (v & 3 == 0 && dpsm == 0 && w * h <= 1024).then(|| (dbp, Upload { psm: dpsm, w, h, data: Vec::new() }));
            }
            0x00 => {
                self.prim = v;
                self.queue.clear();
            }
            0x01 => {
                self.rgbaq = [v as u8, (v >> 8) as u8, (v >> 16) as u8, (v >> 24) as u8];
                self.q = f32::from_bits((v >> 32) as u32);
            }
            0x02 => {
                self.s = f32::from_bits(v as u32);
                self.t = f32::from_bits((v >> 32) as u32);
            }
            0x03 => self.uv = (v as u32 & 0x3fff, (v >> 16) as u32 & 0x3fff),
            0x04 => {
                self.fog = (v >> 56) as u8;
                self.vertex(v as u32 & 0xffff, (v >> 16) as u32 & 0xffff, (v >> 32) as u32 & 0xff_ffff, true);
            }
            0x05 => self.vertex(v as u32 & 0xffff, (v >> 16) as u32 & 0xffff, (v >> 32) as u32, true),
            0x0c => {
                self.fog = (v >> 56) as u8;
                self.vertex(v as u32 & 0xffff, (v >> 16) as u32 & 0xffff, (v >> 32) as u32 & 0xff_ffff, false);
            }
            0x0d => self.vertex(v as u32 & 0xffff, (v >> 16) as u32 & 0xffff, (v >> 32) as u32, false),
            0x06 | 0x07 => self.ctx[(a - 6) as usize].tex0 = v,
            0x08 | 0x09 => self.ctx[(a - 8) as usize].clamp = v,
            0x0a => self.fog = (v >> 56) as u8,
            0x14 | 0x15 => self.ctx[(a - 0x14) as usize].tex1 = v,
            0x16 | 0x17 => {
                // TEX2: PSM and CLUT fields only.
                let c = &mut self.ctx[(a - 0x16) as usize];
                let m = (0x3fu64 << 20) | (0x7ff_ffffu64 << 37);
                c.tex0 = (c.tex0 & !m) | (v & m);
            }
            0x18 | 0x19 => {
                self.ctx[(a - 0x18) as usize].xyoffset = v;
                self.xyoffset_set[(a - 0x18) as usize] = true;
            }
            0x1a => self.prmodecont = v,
            0x1b => self.prmode = v,
            0x3b => self.texa = v,
            0x3d => self.fogcol = v,
            0x40 | 0x41 => self.ctx[(a - 0x40) as usize].scissor = v,
            0x42 | 0x43 => self.ctx[(a - 0x42) as usize].alpha = v,
            0x47 | 0x48 => self.ctx[(a - 0x47) as usize].test = v,
            0x4a | 0x4b => self.ctx[(a - 0x4a) as usize].fba = v,
            0x4c | 0x4d => self.ctx[(a - 0x4c) as usize].frame = v,
            0x4e | 0x4f => self.ctx[(a - 0x4e) as usize].zbuf = v,
            _ => {}
        }
    }

    fn vertex(&mut self, x: u32, y: u32, z: u32, kick: bool) {
        let p = self.cur_prim();
        let ci = bits(p, 9, 1) as usize;
        let xy = if self.xyoffset_set[ci] { self.ctx[ci].xyoffset } else { self.ctx[0].xyoffset };
        let ox = bits(xy, 0, 16) as f32;
        let oy = bits(xy, 32, 16) as f32;
        let v = QVtx {
            x: (x as f32 - ox) / 16.0,
            y: (y as f32 - oy) / 16.0,
            z,
            rgba: self.rgbaq,
            s: self.s,
            t: self.t,
            q: self.q,
            u: self.uv.0,
            v: self.uv.1,
            fog: self.fog,
        };
        self.queue.push(v);
        let ty = bits(p, 0, 3);
        let need = match ty {
            0 => 1,
            1 | 2 | 6 => 2,
            7 => {
                self.queue.clear();
                return;
            }
            _ => 3,
        };
        if self.queue.len() < need {
            return;
        }
        if kick {
            let n = self.queue.len();
            let tri = |a: usize, b: usize, c: usize, q: &[QVtx]| [q[a], q[b], q[c]];
            match ty {
                0 => {
                    let a = self.queue[n - 1];
                    self.point(a);
                }
                1 | 2 => {
                    let (a, b) = (self.queue[n - 2], self.queue[n - 1]);
                    self.line(a, b);
                }
                3 | 4 => {
                    let t = tri(n - 3, n - 2, n - 1, &self.queue);
                    self.tri(t);
                }
                5 => {
                    let t = tri(0, n - 2, n - 1, &self.queue);
                    self.tri(t);
                }
                6 => {
                    let (a, b) = (self.queue[n - 2], self.queue[n - 1]);
                    self.sprite(a, b);
                }
                _ => {}
            }
        }
        // Keep what the next primitive needs.
        match ty {
            0 | 1 | 3 | 6 => self.queue.clear(),
            2 => {
                let l = *self.queue.last().unwrap();
                self.queue.clear();
                self.queue.push(l);
            }
            4 => {
                let n = self.queue.len();
                self.queue.drain(..n - 2);
            }
            5 => {
                let (f, l) = (self.queue[0], *self.queue.last().unwrap());
                self.queue.clear();
                self.queue.push(f);
                self.queue.push(l);
            }
            _ => {}
        }
    }

    /// The draw state for a primitive kicked now; `None` when it can't be
    /// drawn (another target, unknown texture).
    fn state(&mut self) -> Option<DrawState> {
        let p = self.cur_prim();
        let c = self.ctx[bits(p, 9, 1) as usize];
        let target = bits(c.frame, 0, 9) as u32;
        if target != self.main_fbp {
            let (fbw, fpsm) = (bits(c.frame, 16, 6) as u32, bits(c.frame, 24, 6) as u32);
            let h = (bits(c.scissor, 48, 11) as u32 + 1).next_power_of_two().clamp(8, 1024);
            if fbw == 0 || fbw > 16 {
                *self.dropped_fbp.entry((target, fbw, fpsm)).or_default() += 1;
                return None;
            }
            // The game reuses a buffer at several widths (the glow chain
            // draws 256, then 64, then 32 wide into the same FBP): the
            // texture covers the largest, and each image keeps its pixel
            // positions.
            let rt = self.rts.entry(target).or_insert(Rt { w: fbw * 64, h, psm: fpsm });
            rt.w = rt.w.max(fbw * 64);
            rt.h = rt.h.max(h);
            rt.psm = fpsm;
        }
        let tbp = bits(c.tex0, 0, 14) as u32;
        let psm = bits(c.tex0, 20, 6);
        let fpsm = bits(c.frame, 24, 6);
        if p & (1 << 4) != 0 && tbp == self.main_zbp * 32 && (0x30..=0x3a).contains(&psm) && fpsm == 2 && target == self.main_fbp {
            // The depth-fog Z → alpha copy (see ZFOG_TEX).
            self.zalpha = true;
            return None;
        }
        let tex = if p & (1 << 4) != 0 && self.zalpha && tbp == self.main_fbp * 32 && psm == 0x1b {
            let clut = self.uploads.get(&(bits(c.tex0, 37, 14) as u32)).cloned().unwrap_or_default();
            if self.fog_cluts.last().is_none_or(|l| l.data != clut.data) {
                self.fog_cluts.push(clut);
            }
            Some(ZFOG_TEX | (self.fog_cluts.len() - 1) as u32)
        } else if p & (1 << 4) != 0 && tbp == self.main_fbp * 32 && matches!(psm, 0 | 1) {
            // The frame itself (colour filter, speed blur): sampled from a
            // snapshot of the colour target.
            Some(FRAME_TEX)
        } else if p & (1 << 4) != 0 && tbp % 32 == 0 && matches!(psm, 0 | 1 | 2 | 0xa) && self.rts.contains_key(&(tbp / 32)) {
            // (PSMT8H/T4HL/T4HH textures live in the alpha bits of a
            // colour buffer: those are TexMan textures, not the target.)
            Some(RT_TEX | tbp / 32)
        } else if p & (1 << 4) != 0 {
            match self.tex_map.get(&tbp) {
                Some(&t) => Some(t),
                None => {
                    self.unknown_tex += 1;
                    return None;
                }
            }
        } else {
            None
        };
        Some(DrawState {
            prim: (p & ((1 << 4) | (1 << 5) | (1 << 6) | (1 << 8) | (1 << 3))) as u32,
            ctx: c,
            texa: self.texa,
            fogcol: self.fogcol,
            tex,
            target,
        })
    }

    fn zmax(c: &Ctx) -> u32 {
        match bits(c.zbuf, 24, 4) {
            0 => u32::MAX,  // Z32
            1 => 0xff_ffff, // Z24
            _ => 0xffff,    // Z16/Z16S
        }
    }

    fn emit(&mut self, vs: &[QVtx], flat: Option<[u8; 4]>) {
        let got = self.state();
        if let Some(label) = &self.trace {
            let p = self.cur_prim();
            let c = &self.ctx[bits(p, 9, 1) as usize];
            let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
            for v in vs {
                x0 = x0.min(v.x);
                y0 = y0.min(v.y);
                x1 = x1.max(v.x);
                y1 = y1.max(v.y);
            }
            let rgba = flat.unwrap_or(vs[0].rgba);
            let line = format!(
                "{label} {} prim {:#05x} fbp {} fbmsk {:08x} tbp {:#x} info {:x?} psm {:#x} clamp {:#x} texa {:#x} scissor {:#x} alpha {:#x} test {:#x} zbuf {:#x} rgba {:?} stq {:e},{:e},{:e} uv {},{} z {:#x} box {:.0},{:.0}-{:.0},{:.0}",
                if got.is_some() { "draw" } else { "DROP" },
                p & 0x7ff,
                bits(c.frame, 0, 9),
                c.frame >> 32,
                bits(c.tex0, 0, 14),
                self.tex_map.get(&(bits(c.tex0, 0, 14) as u32)),
                bits(c.tex0, 20, 6),
                c.clamp,
                self.texa,
                c.scissor,
                c.alpha,
                c.test,
                c.zbuf,
                rgba,
                vs[0].s,
                vs[0].t,
                vs[0].q,
                vs[0].u,
                vs[0].v,
                vs[0].z,
                x0,
                y0,
                x1,
                y1
            );
            let line = format!("{line} frame {:#x} tex0 {:#x} tex1 {:#x} v{}", c.frame, c.tex0, c.tex1, vs.iter().map(|v| format!(" {:.2},{:.2},q{:.3e}", v.x, v.y, v.q)).collect::<String>());
            match self.trace_log.last_mut() {
                Some(l) if l.ends_with(&format!("|{line}")) || l.starts_with(&line) => {}
                _ => self.trace_log.push(line),
            }
        }
        let Some(st) = got else {
            self.dropped += 1;
            return;
        };

        self.prims += 1;
        let c = st.ctx;
        let zmax = Self::zmax(&c);
        let zscale = 1.0 / (zmax as f64 + 1.0);
        let fst = st.prim & (1 << 8) != 0;
        // Texture size for FST coordinates; the frame snapshot is the target's
        // size (texels = pixels).
        let (tw, th) = match st.tex {
            Some(FRAME_TEX) => self.size,
            Some(t) if t & RT_TEX != 0 => self.rts.get(&(t & !RT_TEX)).map_or((1, 1), |r| (r.w, r.h)),
            _ => (1u32 << bits(c.tex0, 26, 4), 1u32 << bits(c.tex0, 30, 4)),
        };
        // STQ coordinates are relative to the texture's 2^TW × 2^TH; a
        // render target's texture can be larger.
        let (ss, ts) = match st.tex {
            Some(t) if t & RT_TEX != 0 && t != FRAME_TEX => {
                ((1u32 << bits(c.tex0, 26, 4)) as f32 / tw as f32, (1u32 << bits(c.tex0, 30, 4)) as f32 / th as f32)
            }
            _ => (1.0, 1.0),
        };
        let first = self.verts.len() as u32;
        for q in vs {
            let rgba = flat.unwrap_or(q.rgba);
            let stq = if fst {
                [q.u as f32 / 16.0 / tw as f32, q.v as f32 / 16.0 / th as f32, 1.0, 0.0]
            } else {
                [q.s * ss, q.t * ts, q.q, 0.0]
            };
            let x = if self.hud && self.hud_squeeze != 1.0 && st.target == self.main_fbp {
                let cx = self.size.0 as f32 / 2.0;
                cx + (q.x - cx) * self.hud_squeeze
            } else {
                q.x
            };
            self.verts.push(Vtx {
                pos: [x, q.y, (q.z.min(zmax) as f64 * zscale) as f32],
                fog: q.fog as f32,
                col: [rgba[0] as f32, rgba[1] as f32, rgba[2] as f32, rgba[3] as f32],
                stq,
            });
        }
        let n = vs.len() as u32;
        match self.batches.last_mut() {
            Some(b) if b.gpu.is_none() && b.state == st && b.first + b.count == first => b.count += n,
            _ => self.batches.push(Batch { state: st, first, count: n, gpu: None }),
        }
    }

    /// A VU1 call whose vertices the GPU computes (`ssx3_render::gpu`):
    /// its GIF tag starts the primitive as in [`Gs::gif`] (PRE: PRIM), the
    /// draw takes the GS state now, and the GS parameters its vertices
    /// need go into the call's uniforms ([`gpu_params`]). Never merged with
    /// other batches.
    pub fn gpu_draw(&mut self, mut call: ssx3_render::gpu::GpuCall) {
        use ssx3_render::gpu::Link;
        let base = self.gpu.data.len() as u32;
        let reuse = match call.link {
            Link::Reuse(k) => match self.gpu.sources.get(&k) {
                Some(&(b, a)) => {
                    call.alpha_max = a;
                    b
                }
                None => {
                    self.dropped += 1;
                    return;
                }
            },
            Link::Source(k) => {
                self.gpu.sources.insert(k, (base, call.alpha_max));
                0
            }
            Link::None => 0,
        };
        let lo = call.tag as u64;
        if bits(lo, 46, 1) != 0 {
            self.prim = bits(lo, 47, 11);
        }
        let Some(st) = self.state() else {
            self.dropped += 1;
            return;
        };
        self.prims += call.indices.len() as u64 / 3;
        let c = st.ctx;
        let p = self.cur_prim();
        let ci = bits(p, 9, 1) as usize;
        let xy = if self.xyoffset_set[ci] { self.ctx[ci].xyoffset } else { self.ctx[0].xyoffset };
        let zmax = Self::zmax(&c);
        let (tw, th) = match st.tex {
            Some(FRAME_TEX) => self.size,
            Some(t) if t & RT_TEX != 0 => self.rts.get(&(t & !RT_TEX)).map_or((1, 1), |r| (r.w, r.h)),
            _ => (1, 1),
        };
        let (ss, ts) = match st.tex {
            Some(t) if t & RT_TEX != 0 && t != FRAME_TEX => {
                ((1u32 << bits(c.tex0, 26, 4)) as f32 / tw as f32, (1u32 << bits(c.tex0, 30, 4)) as f32 / th as f32)
            }
            _ => (1.0, 1.0),
        };
        let (squeeze, cx) = if self.hud && self.hud_squeeze != 1.0 && st.target == self.main_fbp {
            (self.hud_squeeze, self.size.0 as f32 / 2.0)
        } else {
            (1.0, 0.0)
        };
        let f = |v: f32| v.to_bits();
        let mut u = call.uniforms;
        u[gpu_params::OFFSET] = [f(bits(xy, 0, 16) as f32 / 16.0), f(bits(xy, 32, 16) as f32 / 16.0), f((1.0 / (zmax as f64 + 1.0)) as f32), f(zmax as f32)];
        u[gpu_params::TEX] = [f(ss), f(ts), f(squeeze), f(cx)];
        // The tag's last register is the vertex kick (XYZ2 / XYZF2).
        let nreg = match bits(lo, 60, 4) { 0 => 16, n => n as u32 };
        let vreg = ((call.tag >> 64) >> (4 * (nreg - 1))) as u32 & 0xf;
        u[gpu_params::MISC] = [f(self.fog as f32), base, reuse, vreg];
        let gb = GpuBatch {
            program: call.program,
            call: self.gpu.calls.len() as u32,
            first: self.gpu.indices.len() as u32,
            count: call.indices.len() as u32,
            alpha_max: call.alpha_max,
        };
        self.gpu.calls.push(u);
        self.gpu.data.extend_from_slice(&call.data);
        self.gpu.indices.extend_from_slice(&call.indices);
        self.batches.push(Batch { state: st, first: 0, count: 0, gpu: Some(gb) });
    }

    fn flat(&self, last: &QVtx) -> Option<[u8; 4]> {
        (bits(self.cur_prim(), 3, 1) == 0).then_some(last.rgba)
    }

    fn tri(&mut self, t: [QVtx; 3]) {
        let f = self.flat(&t[2]);
        self.emit(&t, f);
    }

    fn sprite(&mut self, a: QVtx, b: QVtx) {
        // Corners: a (x0, y0), b (x1, y1); attributes of b for colour,
        // texture coordinates per corner.
        let corner = |x: &QVtx, y: &QVtx| QVtx { x: x.x, u: x.u, s: x.s, y: y.y, v: y.v, t: y.t, ..b };
        let (c00, c11) = (corner(&a, &a), corner(&b, &b));
        let (c10, c01) = (corner(&b, &a), corner(&a, &b));
        let f = Some(b.rgba);
        let z = |mut v: QVtx| {
            v.z = b.z;
            v.q = b.q;
            v
        };
        self.emit(&[z(c00), z(c10), z(c11), z(c00), z(c11), z(c01)], f);
    }

    fn line(&mut self, a: QVtx, b: QVtx) {
        // A one-pixel-wide quad along the line.
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
        let (nx, ny) = (-dy / len * 0.5, dx / len * 0.5);
        let off = |v: QVtx, s: f32| QVtx { x: v.x + nx * s, y: v.y + ny * s, ..v };
        let f = self.flat(&b);
        let q = [off(a, 1.0), off(b, 1.0), off(b, -1.0), off(a, 1.0), off(b, -1.0), off(a, -1.0)];
        self.emit(&q, f);
    }

    fn point(&mut self, a: QVtx) {
        let c = |dx: f32, dy: f32| QVtx { x: a.x + dx, y: a.y + dy, ..a };
        let q = [c(-0.5, -0.5), c(0.5, -0.5), c(0.5, 0.5), c(-0.5, -0.5), c(0.5, 0.5), c(-0.5, 0.5)];
        self.emit(&q, Some(a.rgba));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// One A+D GIF packet writing `regs` in order.
    fn ad(regs: &[(u32, u64)]) -> Vec<u128> {
        let tag = regs.len() as u128 | 1 << 60 | 0xe << 64;
        std::iter::once(tag).chain(regs.iter().map(|&(a, v)| v as u128 | (a as u128) << 64)).collect()
    }

    /// A 16×16 PSMCT32 host→local upload of `v` at `dbp`.
    fn upload(dbp: u32, v: u32) -> Vec<u128> {
        let mut q = ad(&[(0x50, (dbp as u64) << 32 | 1 << 48), (0x51, 0), (0x52, 16 | 16 << 32), (0x53, 0)]);
        q.push(64 | 2 << 58);
        q.extend(std::iter::repeat_n(v as u128 * 0x0000_0001_0000_0001_0000_0001_0000_0001, 64));
        q
    }

    fn sprite(prim: u64) -> Vec<(u32, u64)> {
        vec![(0x00, prim), (0x05, 0), (0x05, 16 * 16 | (16 * 16) << 32)]
    }

    #[test]
    fn fog_pass_keeps_the_clut_current_when_it_is_drawn() {
        let mut gs = Gs::new();
        let zbp = 0xe0;
        gs.begin((512, 448), 0, zbp);
        gs.gif(&upload(0x2fd4, 1));
        // The Z → alpha copy: Z as a 16-bit texture into a CT16 view of the
        // frame. Dropped, but it arms the fog pass.
        let mut a = vec![(0x06, (zbp * 32) as u64 | 0x32 << 20), (0x4c, 8 << 16 | 2 << 24)];
        a.extend(sprite(6 | 1 << 4));
        gs.gif(&ad(&a));
        assert!(gs.batches.is_empty());
        // The fog pass: the frame's alpha as PSMT8H through the CLUT.
        let mut b = vec![(0x06, 0x1b << 20 | 0x2fd4 << 37), (0x4c, 8 << 16)];
        b.extend(sprite(6 | 1 << 4 | 1 << 6));
        gs.gif(&ad(&b));
        // The same VRAM is reused later in the frame.
        gs.gif(&upload(0x2fd4, 2));
        let st = gs.batches.last().expect("fog batch").state;
        assert_eq!(st.tex, Some(ZFOG_TEX));
        assert_eq!(gs.fog_cluts.len(), 1);
        assert!(gs.fog_cluts[0].data.iter().all(|&v| v == 1));
        assert!(gs.uploads[&0x2fd4].data.iter().all(|&v| v == 2));
    }

    #[test]
    fn a_target_reused_narrower_keeps_its_widest_size() {
        let mut gs = Gs::new();
        gs.begin((512, 448), 0, 0xe0);
        // The glow chain: 256×256 (FBW 4) into FBP 0xe0, then 64×64 (FBW 1).
        for (fbw, max) in [(4u64, 255u64), (1, 63)] {
            let mut a = vec![(0x4c, 0xe0 | fbw << 16 | 1 << 24), (0x40, max << 16 | max << 48)];
            a.extend(sprite(6));
            gs.gif(&ad(&a));
        }
        assert_eq!(gs.rts[&0xe0], Rt { w: 256, h: 256, psm: 1 });
    }
}
