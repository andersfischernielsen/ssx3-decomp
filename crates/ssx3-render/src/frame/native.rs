//! The frame built from DeferMan's buckets and the captured draws, without
//! walking the published chains (plan step N2, stage B,
//! `docs/plans/native-rendering.md`; what RenderFrame emits:
//! `docs/investigation/n2-capture/prelude.md`).
//!
//! The events are the ones the chain walk produces (`build_cached`) for the
//! same frame, in the same order and with the same owners, except what
//! the renderer does not use: microcode uploads, texture uploads and their
//! PATH3 windows (textures come from TexMan). `SSX3_NATIVE_FRAME_CHECK=1`
//! (`compare`) lists the differences.

use std::collections::HashMap;
use std::sync::Arc;

use super::*;
use crate::direct::{Cmd, PostKind, Recorded};

/// RenderFrame's per-type material constants: the VIF BASE / OFFSET its
/// program chain sets.
fn base_offset(ty: u32) -> (u32, u32) {
    match ty {
        1 | 2 => (0x250, 0xd8),
        3 => (0x26, 0x1b6),
        4 => (0x7, 0x9a),
        5 => (0x3, 0xb),
        _ => (0xc0, 0x169),
    }
}

/// GIF tags of the setups: strip / fan, contexts 1 / 2 (`NLOOP 0, EOP,
/// PRE, NREG 3, REGS 0x412`), and the particle sprite tag.
const S2: u128 = 0x312e_4000_0000_8000 | 0x412 << 64;
const SPRITE: u128 = 0x502b_4000_0000_8000 | 0x5_2512 << 64;

/// An empty GIF tag with EOP and the A+D register (the PATH3 windows).
const EMPTY_TAG: u128 = 0x1000_0000_0000_8000 | 0xe << 64;

/// An A+D block: GIF tag NLOOP n EOP NREG 1 (A+D), then {value, register}.
fn ad_block(writes: &[(u8, Option<u64>)]) -> Option<Event> {
    if writes.is_empty() {
        return None;
    }
    let mut q = vec![writes.len() as u128 | 0x1000_0000_0000_8000 | 0xe << 64];
    q.extend(writes.iter().map(|&(r, v)| v.unwrap_or(0) as u128 | (r as u128) << 64));
    Some(Event::Direct(crate::vif::Direct { qwords: q, native: true }))
}

/// `{0, 0, 0, UNPACK V4-32 n at addr}` (absolute) and its data.
fn unpack(addr: u32, data: &[u128]) -> Vec<u128> {
    let mut v = vec![((0x6c00_0000 | (data.len() as u32 & 0xff) << 16 | addr) as u128) << 96];
    v.extend_from_slice(data);
    v
}

fn qw(m: &dyn Mem, a: u32) -> u128 {
    m.qword(a)
}

/// z lane × 1/16 (types 4, 5).
fn z16(q: u128) -> u128 {
    let z = (q >> 64) as u32;
    let z = (f32::from_bits(z) * 0.0625).to_bits();
    (q & !(0xffff_ffffu128 << 64)) | (z as u128) << 64
}

struct Native<'a> {
    m: &'a dyn Mem,
    g: u32,
    defer: u32,
    vif: Vif1,
    events: Vec<(Owner, Event)>,
    direct: &'a [Recorded],
    at: HashMap<u32, usize>,
    /// Post passes in call order, per kind.
    posts: [Vec<usize>; 3],
    next_post: [usize; 3],
    path3: [Vec<Vec<u128>>; 2],
    phase: usize,
    /// TexMan: the ids bound per context, the CLUT id (tm+0x1f4c..).
    bound: [i32; 2],
    clut: i32,
    issues: Vec<String>,
}

impl Native<'_> {
    fn flush(&mut self, owner: Owner) {
        for e in std::mem::take(&mut self.vif.events) {
            self.events.push((owner, e));
        }
    }

    fn push(&mut self, owner: Owner, e: Event) {
        self.flush(owner);
        self.events.push((owner, e));
    }

    /// The matrix record `i`'s qword at `off`.
    fn rec(&self, i: i32, off: u32) -> u128 {
        qw(self.m, self.defer + dm::MATRICES + 0x180 * i.max(0) as u32 + off)
    }

    /// A CALL of a w3 packet (slot 124 lighting, slot 101 shadow header).
    fn call_w3(&mut self, w3: u32, cache: &mut ChainCache) {
        match self.at.get(&w3).map(|&k| &self.direct[k].cmd) {
            Some(Cmd::Vu(steps)) => {
                let steps = steps.clone();
                self.vif.native(&steps);
            }
            _ => {
                self.issues.push(format!("w3 packet {w3:#x} not captured"));
                let _ = cache;
                let mut asr = [0u32; 2];
                let vif = &mut self.vif;
                crate::chain::walk(self.m, w3, false, false, 0, &mut asr, &mut |d| vif.write(d));
            }
        }
    }

    /// `DeferMan_EmitMaterialSetup` (0x364cd0) for draw `d`.
    fn material_setup(&mut self, d: &Draw, cache: &mut ChainCache) {
        let ty = d.state.material.type_id();
        let w3 = d.state.raw[3] & 0x3fff_ffff;
        let mode = if d.setup.program { 0 } else if d.setup.matrix { 1 } else if d.setup.param { 2 } else { return };
        let mi = d.matrix as i32;
        let set_a = |n: &Self| -> Vec<u128> { [0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0].iter().map(|&o| n.rec(mi, o)).collect() };
        if mode == 0 {
            // The program chain: STCYCL 4,4; STMASK 0; STMOD 0; BASE;
            // OFFSET; MPG (not modelled).
            let (b, o) = base_offset(ty);
            let r = &mut self.vif.regs;
            r.cycle = 0x0404;
            r.mask = 0;
            r.mode = 0;
            r.base = b;
            r.ofst = o;
            r.dbf = false;
            r.tops = b;
            match ty {
                4 => {
                    self.vif.write(&unpack(0x13b, &[SPRITE]));
                    self.vif.write(&unpack(0x29a, &[SPRITE]));
                }
                5 => {
                    self.vif.write(&unpack(0x19, &[SPRITE]));
                    self.vif.write(&unpack(0x209, &[SPRITE]));
                }
                _ => {}
            }
        }
        match (ty, mode) {
            (1, 0) => {
                let q: Vec<u128> = (0..12).map(|i| qw(self.m, 0x0044_b080 + 16 * i)).collect();
                self.vif.write(&q);
                let a = set_a(self);
                self.vif.write(&unpack(0, &a));
            }
            (1 | 2, 1) => {
                let a = set_a(self);
                self.vif.write(&unpack(0, &a));
            }
            (2, 0) => {
                let mut d11 = set_a(self);
                d11.push(0x80 | 0x80 << 32 | 0x80 << 64 | 0x80 << 96);
                d11.extend([0x302e_4000_0000_8000 | 0x412 << 64, S2, 0x302e_c000_0000_8000 | 0x412 << 64, 0x312e_c000_0000_8000 | 0x412 << 64]);
                self.vif.write(&unpack(0, &d11));
                let x = word(self.m, 0x004a_30f0 + 0x1158);
                let mut blk: Vec<u128> = Vec::new();
                for p in [word(self.m, x + 0x1a0), word(self.m, x + 0x320)] {
                    blk.extend((0..0x12).map(|i| qw(self.m, p + 16 * i)));
                }
                let f = |o: u32| word(self.m, 0x004a_30f0u32.wrapping_sub(o)) as u128;
                let k = |a: u128, b: u128| a | b << 32;
                let r3 = f(0x28a8);
                let r5 = f(0x28a4);
                let r7 = f(0x28a0);
                blk.extend([k(0x200, 0x212), r3 | r3 << 32 | r3 << 64 | r3 << 96, k(0x204, 0x216), r5 | r5 << 32 | r5 << 64 | r5 << 96, k(0x20a, 0x21c), r7 | r7 << 32 | r7 << 64 | r7 << 96]);
                self.vif.write(&unpack(0x200, &blk));
            }
            (3, 0) => {
                let mut a = set_a(self);
                a.push(0x10c | 0x2c2 << 32);
                self.vif.write(&unpack(0, &a));
                let q: Vec<u128> = (0..7).map(|i| qw(self.m, 0x0044_b140 + 16 * i)).collect();
                self.vif.write(&q);
            }
            (3, 1) => {
                let a = set_a(self);
                self.vif.write(&unpack(0, &a));
            }
            (4, 0) => {
                let mut a: Vec<u128> = [0x80, 0x90, 0xa0, 0xb0].iter().map(|&o| self.rec(mi, o)).collect();
                a.push(z16(self.rec(mi, 0xc0)));
                a.push(z16(self.rec(mi, 0xd0)));
                let (x, y) = (self.rec(mi, 0x40) as u32, (self.rec(mi, 0x50) >> 32) as u32);
                a.push(x as u128 | (y as u128) << 32);
                self.vif.write(&unpack(0, &a));
            }
            (4, 1) => {
                let mut a: Vec<u128> = [0x120, 0x130, 0x140, 0x150].iter().map(|&o| self.rec(mi, o)).collect();
                a.push(z16(self.rec(mi, 0x160)));
                a.push(z16(self.rec(mi, 0x170)));
                let (x, y) = (self.rec(mi, 0xe0) as u32, (self.rec(mi, 0xf0) >> 32) as u32);
                a.push(x as u128 | (y as u128) << 32);
                self.vif.write(&unpack(0, &a));
            }
            (5, 0 | 1) => {
                let (x, y) = (self.rec(mi, 0xe0) as u32, (self.rec(mi, 0xf0) >> 32) as u32);
                let a = [z16(self.rec(mi, 0x160)), z16(self.rec(mi, 0x170)), x as u128 | (y as u128) << 32];
                self.vif.write(&unpack(0, &a));
            }
            (6, 0 | 1) => {
                let a = set_a(self);
                self.vif.write(&unpack(0, &a));
                // Env rows: C (0x4ffb80) times the world matrix rows.
                let c: [crate::mat4::V; 4] = std::array::from_fn(|i| v4(qw(self.m, 0x004f_fb80 + 16 * i as u32)));
                let mut rows = vec![S2];
                for i in 0..3 {
                    let w = v4(self.rec(mi, 16 * i));
                    let acc = crate::mat4::mul(c[0], [w[0]; 4]);
                    let acc = crate::mat4::madd(acc, c[1], [w[1]; 4]);
                    let acc = crate::mat4::madd(acc, c[2], [w[2]; 4]);
                    let e = crate::mat4::madd(acc, c[3], [w[3]; 4]);
                    rows.push(q4(e));
                }
                self.vif.write(&unpack(0x392, &rows));
                if w3 != 0 {
                    self.call_w3(w3, cache);
                }
            }
            (6 | 7, _) => {
                if w3 != 0 {
                    self.call_w3(w3, cache);
                }
            }
            _ => {}
        }
    }

    /// `TexMan_BindTextures` (0x368660) for draw `d`, without the uploads:
    /// `TexMan_EmitTex0` (0x368970) per context.
    fn bind(&mut self, d: &Draw, owner: Owner) {
        let tm = word(self.m, self.g + gm::TEXMAN);
        for ctx in 0..2 {
            let Some(id) = d.tex[ctx] else { continue };
            let id = id as i32;
            let info = word(self.m, tm + 8 + 4 * id as u32);
            let tex0 = |n: &Self| word(n.m, info + 0x38) as u64 | (word(n.m, info + 0x3c) as u64) << 32;
            let cbp = word(self.m, info + 0x2c) as i32;
            if self.bound[ctx] == id {
                if self.clut != id && cbp >= 0 {
                    self.clut = id;
                    let v = tex0(self) | (cbp as u64) << 37;
                    if let Some(e) = ad_block(&[(0x16 + ctx as u8, Some(v))]) {
                        self.push(owner, e);
                    }
                }
                continue;
            }
            self.bound[ctx] = id;
            // The VRAM block: TexInfo's, unless the texture was moved out
            // again later in the frame (TBP -1 when the frame is built):
            // then one of its own that is no render target's (never a
            // multiple of 32); the renderer takes the texture from TexMan.
            let tbp = match word(self.m, info + 0x28) as i32 {
                t if t >= 0 => t,
                _ => (id << 1 | 1) & 0x3fff,
            };
            let mips = word(self.m, info + 0x24) as i32;
            let mut w: Vec<(u8, Option<u64>)> = Vec::new();
            if mips > 0 {
                let t0 = tex0(self);
                let bpp = word(self.m, info + 0x18) as i32;
                let shift = if word(self.m, info + 0xc) == 9 { 6 } else { 8 };
                let (tw, th) = (1i32 << ((t0 >> 26) & 15), 1i32 << ((t0 >> 30) & 15));
                let page = |w: i32, h: i32| -> i32 {
                    let (pw, ph) = match bpp {
                        4 => (0x80, 0x80),
                        8 => (0x80, 0x40),
                        0xf | 0x10 => (0x40, 0x40),
                        _ => (0x40, 0x20),
                    };
                    let bytes = pw.max(w) * ph.max(h) * bpp;
                    let b = if bytes < 0 { bytes + 7 } else { bytes } >> 3;
                    (b + (-b & 0xff)) >> shift
                };
                let t1 = tbp + page(tw, th);
                let t2 = t1 + page(tw >> 1, th >> 1);
                let t3 = t2 + page(tw >> 2, th >> 2);
                let mip1 = word(self.m, info + 0x48) as u64 | (word(self.m, info + 0x4c) as u64) << 32;
                w.push((0x34 + ctx as u8, Some(mip1 | t1 as i64 as u64 | (t2 as i64 as u64) << 20 | (t3 as i64 as u64) << 40)));
                if mips > 3 {
                    let t4 = t3 + page(tw >> 3, th >> 3);
                    let t5 = t4 + page(tw >> 4, th >> 4);
                    let t6 = t5 + page(tw >> 5, th >> 5);
                    let mip2 = word(self.m, info + 0x50) as u64 | (word(self.m, info + 0x54) as u64) << 32;
                    w.push((0x36 + ctx as u8, Some(mip2 | t4 as i64 as u64 | (t5 as i64 as u64) << 20 | (t6 as i64 as u64) << 40)));
                }
            }
            let tex1 = word(self.m, info + 0x40) as u64 | (word(self.m, info + 0x44) as u64) << 32;
            w.push((0x14 + ctx as u8, Some(tex1)));
            let mut c = 0u64;
            if cbp >= 0 {
                self.clut = id;
                c = cbp as u64;
            }
            w.push((6 + ctx as u8, Some(tex0(self) | tbp as u64 | c << 37)));
            if let Some(e) = ad_block(&w) {
                self.push(owner, e);
            }
        }
    }

    /// A captured packet at `t` applied (as the walker does); its end.
    fn packet(&mut self, t: u32, owner: Owner, cache: &mut ChainCache) -> Option<u32> {
        let &k = self.at.get(&t)?;
        let r = &self.direct[k];
        match &r.cmd {
            Cmd::Vu(steps) => {
                let steps = steps.clone();
                for run in steps.split_inclusive(|s| matches!(s, crate::direct::Step::Chain(_))) {
                    self.vif.native(run);
                    if let Some(&crate::direct::Step::Chain(a)) = run.last() {
                        play_chain(self.m, &mut self.vif, cache, a);
                    }
                }
                let n = self.vif.events.len();
                for e in &mut self.vif.events[..n] {
                    match e {
                        Event::Call(c) => c.native = true,
                        Event::Direct(d) => d.native = true,
                        _ => {}
                    }
                }
                self.flush(owner);
            }
            Cmd::Post { .. } => return None,
            cmd => {
                let header = std::array::from_fn(|i| u128::from_le_bytes(self.vif.mem[16 * i..16 * i + 16].try_into().unwrap()));
                let starts = cmd.starts();
                self.push(owner, Event::Native { index: k as u32, regs: self.vif.regs, header });
                self.vif.skip_starts(starts);
            }
        }
        Some(r.end)
    }

    /// A bucket's packets `start..last` (its first packet's start, its
    /// last packet's end), linked by NEXT. (RenderFrame's RET at `last` is
    /// not needed: a native RenderFrame writes none.)
    fn payload(&mut self, start: u32, last: u32, owner: Owner, cache: &mut ChainCache) {
        let mut t = start;
        for _ in 0..4096 {
            let end = match self.packet(t, owner, cache) {
                Some(e) => e,
                None => {
                    // (Not captured: decoded, from here to the bucket's RET.)
                    self.issues.push(format!("packet {t:#x} not captured"));
                    let mut asr = [0u32; 2];
                    let vif = &mut self.vif;
                    crate::chain::walk(self.m, t, false, false, 0, &mut asr, &mut |d| vif.write(d));
                    self.flush(owner);
                    return;
                }
            };
            if end == last {
                return;
            }
            let tag = self.m.qword(end);
            match (tag >> 28) & 7 {
                2 => t = ((tag >> 32) & 0x7fff_fff0) as u32,
                6 => return,
                id => {
                    self.issues.push(format!("packet end {end:#x}: tag id {id}"));
                    return;
                }
            }
        }
    }

    /// The next captured post pass of `kind`: its PATH3 windows (and its
    /// GIF data for PATH3).
    fn post(&mut self, kind: PostKind) {
        let i = kind as usize;
        let Some(&k) = self.posts[i].get(self.next_post[i]) else {
            self.issues.push(format!("{kind:?} not captured"));
            return;
        };
        self.next_post[i] += 1;
        let Cmd::Post { gif, windows, .. } = &self.direct[k].cmd else { return };
        let (gif, windows) = (gif.clone(), *windows);
        self.path3[self.phase].extend(split_gif(&gif));
        for _ in 0..windows {
            let d = || Event::Direct(crate::vif::Direct { qwords: vec![EMPTY_TAG], native: true });
            self.push(Owner::Post, d());
            self.push(Owner::Post, Event::MaskPath3(false));
            self.push(Owner::Post, Event::MaskPath3(true));
            self.push(Owner::Post, d());
        }
    }
}

fn v4(q: u128) -> crate::mat4::V {
    std::array::from_fn(|l| (q >> (32 * l)) as u32)
}

fn q4(v: crate::mat4::V) -> u128 {
    v.iter().enumerate().fold(0u128, |q, (l, &x)| q | (x as u128) << (32 * l))
}

/// GIF data as packets, each up to a tag with EOP.
fn split_gif(q: &[u128]) -> Vec<Vec<u128>> {
    let mut out = Vec::new();
    let (mut i, mut start) = (0, 0);
    while i < q.len() {
        let lo = q[i] as u64;
        let nloop = (lo & 0x7fff) as usize;
        let eop = (lo >> 15) & 1 != 0;
        let nreg = match (lo >> 60) as usize {
            0 => 16,
            n => n,
        };
        i += 1 + match (lo >> 58) & 3 {
            0 => nloop * nreg,
            1 => (nloop * nreg).div_ceil(2),
            _ => nloop,
        };
        if eop || i >= q.len() {
            out.push(q[start..i.min(q.len())].to_vec());
            start = i;
        }
    }
    out
}


/// Build the frame from the buckets and the captures (see the module).
pub(super) fn build(m: &dyn Mem, g: u32, carry: &Carry, cache: &mut ChainCache, direct: Arc<Vec<Recorded>>) -> Built {
    let defer = word(m, g + gm::DEFER);
    let mut issues = Vec::new();
    let nm = word(m, defer + dm::NUM_MATRICES).min(dm::MAX_MATRICES);
    let matrices: Vec<_> = (0..nm).map(|i| MatrixRecord::read(m, defer + dm::MATRICES + i * 0x180)).collect();
    let ns = word(m, defer + dm::NUM_SCISSORS).min(32);
    let scissors: Vec<[i16; 4]> = (0..32)
        .map(|i| {
            let a = defer + dm::SCISSORS + i * 8;
            [half(m, a), half(m, a + 2), half(m, a + 4), half(m, a + 6)]
        })
        .collect();
    let (plan, emitter) = plan(m, g, defer, carry, &scissors, &mut issues);
    let num_views = word(m, g + gm::NUM_VIEWS).min(4);
    let vif_heads = [word(m, g + gm::QUEUED_VIF1), word(m, g + gm::QUEUED_VIF2)];
    let gif_heads = [word(m, g + gm::QUEUED_GIF1), word(m, g + gm::QUEUED_GIF2)];
    let mut at = HashMap::new();
    let mut posts: [Vec<usize>; 3] = Default::default();
    for (k, r) in direct.iter().enumerate() {
        match &r.cmd {
            Cmd::Post { kind, .. } => posts[*kind as usize].push(k),
            _ => {
                at.insert(r.start, k);
            }
        }
    }
    let mut n = Native {
        m,
        g,
        defer,
        vif: Vif1::new(carry.regs),
        events: Vec::new(),
        direct: &direct,
        at,
        posts,
        next_post: [0; 3],
        path3: Default::default(),
        phase: 0,
        bound: [-1; 2],
        clut: -1,
        issues: Vec::new(),
    };
    let default = [0u32, 4, 8, 12, 16].map(|o| word(m, addr::DEFAULT_STATE + o));
    let mut e0 = Emitter { state: default, scissor: -1, matrix: -1, material_param: 0, regs: GsRegs::default(), zbp: word(m, g + gm::ZBP), zpsm: word(m, g + gm::ZPSM), writes: Vec::new() };
    e0.default_state(default);
    let default_writes = e0.writes;
    let views = |base: u32, stride: u32| -> Vec<u32> { (0..num_views).map(|v| g + base + v * stride).collect() };
    let mut passes: Vec<Pass> = Vec::new();
    let mut cur: Vec<Draw> = Vec::new();
    let mut phase2 = 0;
    let default_block = |n: &mut Native, passes: &mut Vec<Pass>, cur: &mut Vec<Draw>| {
        if !cur.is_empty() {
            passes.push(Pass::Draws(std::mem::take(cur)));
        }
        if let Some(e) = ad_block(&default_writes) {
            n.push(Owner::DefaultState, e);
        }
        passes.push(Pass::DefaultState(Vec::new()));
        n.bound = [-1; 2];
        n.clut = -1;
    };
    let draws = plan.draws;
    if plan.num_sorted > 0 {
        default_block(&mut n, &mut passes, &mut cur);
        for k in 0..3 {
            if k == 2 {
                if !cur.is_empty() {
                    passes.push(Pass::Draws(std::mem::take(&mut cur)));
                }
                passes.push(Pass::PhaseBoundary);
                n.flush(Owner::Unattributed);
                phase2 = n.events.len();
                n.phase = 1;
                if plan.thresholds[1] == 9 {
                    for (v, a) in views(gm::BLOOM, 0x1c).into_iter().enumerate() {
                        n.post(PostKind::Bloom);
                        passes.push(Pass::Bloom { view: v as u8, params: core::array::from_fn(|k| word(m, a + 4 * k as u32)) });
                    }
                    for (v, a) in views(gm::COLOR_FILTER, 0x18).into_iter().enumerate() {
                        n.post(PostKind::ColorFilter);
                        let f = |k: u32| f32_at(m, a + 4 * k);
                        passes.push(Pass::ColorFilter { view: v as u8, scale: [f(0), f(1), f(2)], offset: [f(3), f(4), f(5)] });
                    }
                    default_block(&mut n, &mut passes, &mut cur);
                }
            }
            for i in plan.ranges[k].clone() {
                let d = draws[i].clone();
                if d.state_change {
                    n.material_setup(&d, cache);
                    n.flush(Owner::Prelude(i));
                    if let Some(e) = ad_block(&d.ad) {
                        n.push(Owner::Prelude(i), e);
                    }
                }
                n.bind(&d, Owner::Prelude(i));
                if d.packets.start != 0 {
                    n.payload(d.packets.start, d.packets.end, Owner::Payload(i), cache);
                }
                cur.push(d);
            }
            if k == 0 && plan.thresholds[0] == 6 {
                if !cur.is_empty() {
                    passes.push(Pass::Draws(std::mem::take(&mut cur)));
                }
                for (v, a) in views(gm::DEPTH_FOG, 0x20).into_iter().enumerate() {
                    n.post(PostKind::DepthFog);
                    passes.push(Pass::DepthFog { view: v as u8, params: core::array::from_fn(|k| word(m, a + 4 * k as u32)) });
                }
                default_block(&mut n, &mut passes, &mut cur);
            }
        }
    }
    if !cur.is_empty() {
        passes.push(Pass::Draws(cur));
    }
    n.flush(Owner::Unattributed);
    issues.append(&mut n.issues);
    let display = DisplayState {
        fb_width: word(m, g + gm::FB_WIDTH),
        fb_height: word(m, g + gm::FB_HEIGHT),
        draw_fbp: word(m, g + gm::DRAW_FBP),
        zbp: word(m, g + gm::ZBP),
        zpsm: word(m, g + gm::ZPSM),
        blit: word(m, g + gm::BLIT) != 0,
        brightness_rgba: core::array::from_fn(|k| word(m, g + gm::BRIGHTNESS_RGBA + 4 * k as u32) as i32),
        brightness: f32_at(m, g + gm::BRIGHTNESS),
        screen_offset: [f32_at(m, g + gm::SCREEN_OFFSET), f32_at(m, g + gm::SCREEN_OFFSET + 4)],
    };
    let views = (0..num_views).map(|v| View { screen: mat4(m, g + gm::VIEW_MATRICES + v * 0x40) }).collect();
    let n_draws = draws.len();
    let path3 = std::mem::take(&mut n.path3);
    Built {
        frame: Frame { gm: g, defer, vif_heads, gif_heads, views, matrices, scissors: scissors[..ns.max(1) as usize].to_vec(), passes, display },
        events: n.events,
        obs: vec![ChainObs::default(); n_draws],
        issues,
        phase2,
        regs_end: n.vif.regs,
        emitter,
        direct: direct.clone(),
        path3: Some(path3),
        flip: super::flip_packet(m),
    }
}

/// Debug: the differences between a walked frame's events and the native
/// build's, ignoring what the native build leaves out (microcode uploads,
/// texture-upload windows) and VRAM placement (TBP / CBP fields).
pub fn compare(walked: &Built, native: &Built, max: usize) -> Vec<String> {
    let keep = |(o, e): &&(Owner, Event)| match e {
        Event::Microcode { .. } => false,
        Event::MaskPath3(_) => !matches!(o, Owner::Prelude(_)),
        Event::Direct(d) => !(matches!(o, Owner::Prelude(_)) && d.qwords == [EMPTY_TAG]),
        _ => true,
    };
    let a: Vec<_> = walked.events.iter().filter(keep).collect();
    let b: Vec<_> = native.events.iter().collect();
    let tex = |e: &Event| -> Event {
        match e {
            Event::Direct(d) => {
                let mut q = d.qwords.clone();
                for x in q.iter_mut().skip(1) {
                    let r = (*x >> 64) as u8;
                    if matches!(r, 6 | 7 | 0x16 | 0x17) {
                        *x &= !(0x3fffu128 | 0x3fffu128 << 37);
                    }
                    if matches!(r, 0x34..=0x37) {
                        *x &= !(0x3fffu128 | 0x3fffu128 << 20 | 0x3fffu128 << 40);
                    }
                }
                Event::Direct(crate::vif::Direct { qwords: q, native: false })
            }
            e => e.clone(),
        }
    };
    let mut out = Vec::new();
    for (i, (x, y)) in a.iter().zip(&b).enumerate() {
        if x.0 != y.0 || tex(&x.1) != tex(&y.1) {
            out.push(format!("event {i}: walked {:?} {} / native {:?} {}", x.0, short(&x.1), y.0, short(&y.1)));
            if out.len() >= max {
                break;
            }
        }
    }
    if a.len() != b.len() {
        out.push(format!("{} events walked, {} native", a.len(), b.len()));
    }
    out
}

fn short(e: &Event) -> String {
    match e {
        Event::Call(c) => format!("call {:x?} top {:#x} itop {} in {}+{}", c.entry, c.top, c.itop, c.input.len(), c.shared.as_ref().map_or(0, |s| s.len())),
        Event::Direct(d) => format!("gif {:x?}", &d.qwords[..d.qwords.len().min(4)]),
        e => format!("{e:?}").chars().take(80).collect(),
    }
}
