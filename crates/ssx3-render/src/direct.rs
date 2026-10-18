//! Draws captured at the game's draw functions (plan step N2, direct
//! rendering, `docs/plans/native-rendering.md`).
//!
//! A captured draw function still runs its generated body (the packet is
//! built, guest memory stays exactly as without the capture) and records
//! what it drew as a typed [`Cmd`], keyed by the start of the packet it
//! wrote. The frame build ([`crate::frame`]) meets that packet when the
//! chain CALLs it and, instead of decoding it, emits
//! [`crate::vif::Event::Native`]; the renderer draws the command
//! ([`plan`]). Packets without a command (other draw functions, a capture
//! the game unwound out of) take the decoding path.
//!
//! The game thread records ([`record`]) while the renderer is active
//! ([`set_active`]); the frame sink takes the list when the frame is
//! published ([`take`]).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::vif::VifRegs;

/// What a captured draw function drew.
#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    /// `PS2GraphicsMan_DrawTriStrip` (slot 71, 0x379860): a float triangle
    /// strip through the type 3 program at `entry` (0x12b8, or 0x12b0 once
    /// after gp+0x1290 was set), in batches of at most 72 vertices that
    /// overlap by two.
    Strip {
        entry: u32,
        /// PRIM: 0x5c textured, 0x4c untextured.
        prim: u32,
        /// Per vertex {STQ (f32), RGBA (i32), XYZW (f32)}.
        verts: Vec<[u128; 3]>,
    },
    /// `PS2GraphicsMan_EmitBoardWakeStrips` (0x3885e0) and
    /// `EmitShadowStrips` (0x387ec0): ribbons through the type 3 program at
    /// `entry` (0x3a08, or 0x3a00 once after their flag was set). Per
    /// program start, the qwords its input holds (offset from TOP, value):
    /// the strip tag, the counts and alpha ramp, edge A, edge B.
    Ribbon { entry: u32, calls: Vec<Vec<(u16, u128)>> },
    /// What a packet does to VU1, in order, without its encoding: the frame
    /// build applies it to its VIF state as the packet would have
    /// (`crate::vif::Vif1::native`), so its program starts become ordinary
    /// VU calls (and later packets see the memory and registers it left).
    Vu(Vec<Step>),
    /// A post pass (`RenderDepthFog` / `RenderBloom` / `RenderColorFilter`,
    /// one view): the GIF data it queued for PATH3 and the PATH3 windows it
    /// put into the VIF chain (each runs the queued data up to the next
    /// EOP). `start..end` is its stretch of the VIF chain.
    Post { kind: PostKind, gif: Vec<u128>, windows: u32 },
}

/// Which post pass a `Cmd::Post` is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostKind {
    DepthFog = 0,
    Bloom = 1,
    ColorFilter = 2,
}

/// One effect of a packet on VU1 (`Cmd::Vu`).
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    /// Quadwords stored from `addr` (relative to TOPS when `rel`, as UNPACK
    /// with FLG), as VU memory holds them afterwards.
    Write { addr: u16, rel: bool, data: Vec<u128> },
    /// STCYCL, ITOP, STMOD, STMASK.
    Cycle(u32),
    Itop(u32),
    Mode(u32),
    Mask(u32),
    /// MSCAL.
    Start(u32),
    /// A CALL into a prebuilt chain (a model's mesh chain): played through
    /// the frame build's chain cache.
    Chain(u32),
    /// GIF data sent with DIRECT / DIRECTHL.
    Gif(Vec<u128>),
    /// MSKPATH3.
    MaskPath3(bool),
    /// VIFcodes a capture does not turn into effects (masked UNPACKs whose
    /// fill depends on registers set elsewhere): run by the VIF as they are.
    Raw(Vec<u128>),
}

/// The vertex count of a strip batch.
pub const STRIP_BATCH: usize = 0x48;

impl Cmd {
    /// The VU1 program starts (MSCALs) its packet makes.
    pub fn starts(&self) -> u32 {
        match self {
            Cmd::Strip { verts, .. } => strip_batches(verts.len()).count() as u32,
            Cmd::Ribbon { calls, .. } => calls.len() as u32,
            Cmd::Vu(steps) => steps.iter().filter(|s| matches!(s, Step::Start(_))).count() as u32,
            Cmd::Post { .. } => 0,
        }
    }
}

/// The batches of a strip of `n` vertices, as the game splits it: ranges of
/// at most [`STRIP_BATCH`] vertices, each starting two before the last end.
pub fn strip_batches(n: usize) -> impl Iterator<Item = std::ops::Range<usize>> {
    let mut at = 0usize;
    let mut left = n;
    std::iter::from_fn(move || {
        if left <= 2 {
            return None;
        }
        let k = left.min(STRIP_BATCH);
        let r = at..at + k;
        at += k - 2;
        left = left - k + 2;
        Some(r)
    })
}

/// A captured draw: its packet in VifBuff (`start..end`, `end` = where the
/// bucket's link tag goes), the packet's first DMA tag (to see it is the
/// same packet) and the command.
#[derive(Clone, Debug, PartialEq)]
pub struct Recorded {
    pub start: u32,
    pub end: u32,
    pub first_tag: u128,
    pub cmd: Cmd,
}

static ACTIVE: AtomicBool = AtomicBool::new(false);
static LIST: Mutex<Vec<Recorded>> = Mutex::new(Vec::new());

/// Captures are recorded (a renderer takes them); `SSX3_DIRECT=0` keeps
/// them off.
pub fn set_active(on: bool) {
    let on = on && std::env::var("SSX3_DIRECT").map_or(true, |v| v != "0");
    ACTIVE.store(on, Ordering::Relaxed);
    if !on {
        LIST.lock().unwrap().clear();
    }
}

pub fn active() -> bool {
    ACTIVE.load(Ordering::Relaxed)
}

/// Record a capture (game thread).
pub fn record(r: Recorded) {
    if active() {
        LIST.lock().unwrap().push(r);
    }
}

/// The captures since the last call (the frame being published).
pub fn take() -> Vec<Recorded> {
    std::mem::take(&mut *LIST.lock().unwrap())
}

/// Draw a captured command: its GPU draws, or the GIF output of the CPU
/// program for what has no GPU port. `regs`: the VIF registers at the
/// packet (the program's TOP for each start); `header`: VU1 qw0-5 as the
/// draw's state change uploaded them; `vu`:
/// VU1 memory with the programs' static data (any frame's).
pub fn plan(cmd: &Cmd, regs: VifRegs, header: &[u128; 6], vu: &[u8; 16384]) -> Vec<Out> {
    let qw = |a: usize| u128::from_le_bytes(vu[a * 16..a * 16 + 16].try_into().unwrap());
    match cmd {
        Cmd::Strip { entry, prim, verts } => {
            let tops = tops(regs, cmd.starts());
            crate::mat3::plan_strip_cmd(*entry, *prim, verts, &tops, header, [qw(0x3c8), qw(0x3e4)])
        }
        Cmd::Ribbon { entry, calls } => {
            let tops = tops(regs, cmd.starts());
            crate::mat3::plan_ribbon_cmd(*entry, calls, &tops, header, [qw(0x3c8), qw(0x3e4)])
        }
        // (Applied to the VIF state by the frame build: ordinary calls.)
        Cmd::Vu(_) | Cmd::Post { .. } => Vec::new(),
    }
}

/// One output of [`plan`], in kick order.
#[derive(Clone, Debug, PartialEq)]
pub enum Out {
    Gpu(crate::gpu::GpuCall),
    Gif(Vec<u128>),
}

/// The TOP of each program start from `regs` on (VIF1 double buffering).
pub fn tops(mut regs: VifRegs, n: u32) -> Vec<u32> {
    (0..n)
        .map(|_| {
            let top = regs.tops & 0x3ff;
            regs.tops = if regs.dbf { regs.base } else { regs.base + regs.ofst };
            regs.dbf = !regs.dbf;
            top
        })
        .collect()
}

// Encoding (frame dumps, VU jobs).

/// Append `list` to `out`.
pub fn encode(list: &[Recorded], out: &mut Vec<u8>) {
    out.extend_from_slice(&(list.len() as u32).to_le_bytes());
    for r in list {
        out.extend_from_slice(&r.start.to_le_bytes());
        out.extend_from_slice(&r.end.to_le_bytes());
        out.extend_from_slice(&r.first_tag.to_le_bytes());
        encode_cmd(&r.cmd, out);
    }
}

/// Read what [`encode`] wrote; `None` if malformed.
pub fn decode(b: &[u8]) -> Option<Vec<Recorded>> {
    let mut r = Bytes(b);
    let n = r.u32()?;
    let mut list = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let (start, end, first_tag) = (r.u32()?, r.u32()?, r.u128()?);
        list.push(Recorded { start, end, first_tag, cmd: decode_cmd(&mut r)? });
    }
    Some(list)
}

/// Append one command to `out`.
pub fn encode_cmd(cmd: &Cmd, out: &mut Vec<u8>) {
    match cmd {
        Cmd::Strip { entry, prim, verts } => {
            out.push(0);
            out.extend_from_slice(&entry.to_le_bytes());
            out.extend_from_slice(&prim.to_le_bytes());
            out.extend_from_slice(&(verts.len() as u32).to_le_bytes());
            for v in verts.iter().flatten() {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        Cmd::Post { kind, gif, windows } => {
            out.push(3);
            out.push(*kind as u8);
            out.extend_from_slice(&windows.to_le_bytes());
            out.extend_from_slice(&(gif.len() as u32).to_le_bytes());
            for q in gif {
                out.extend_from_slice(&q.to_le_bytes());
            }
        }
        Cmd::Vu(steps) => {
            out.push(2);
            out.extend_from_slice(&(steps.len() as u32).to_le_bytes());
            for s in steps {
                let (k, v) = match s {
                    Step::Write { addr, rel, data } => {
                        out.push(0);
                        out.extend_from_slice(&addr.to_le_bytes());
                        out.push(*rel as u8);
                        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
                        for q in data {
                            out.extend_from_slice(&q.to_le_bytes());
                        }
                        continue;
                    }
                    Step::Raw(data) => {
                        out.push(9);
                        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
                        for q in data {
                            out.extend_from_slice(&q.to_le_bytes());
                        }
                        continue;
                    }
                    Step::Gif(data) => {
                        out.push(7);
                        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
                        for q in data {
                            out.extend_from_slice(&q.to_le_bytes());
                        }
                        continue;
                    }
                    Step::MaskPath3(m) => (8, &(*m as u32)),
                    Step::Cycle(v) => (1, v),
                    Step::Itop(v) => (2, v),
                    Step::Mode(v) => (3, v),
                    Step::Mask(v) => (4, v),
                    Step::Start(v) => (5, v),
                    Step::Chain(v) => (6, v),
                };
                out.push(k);
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        Cmd::Ribbon { entry, calls } => {
            out.push(1);
            out.extend_from_slice(&entry.to_le_bytes());
            out.extend_from_slice(&(calls.len() as u32).to_le_bytes());
            for c in calls {
                out.extend_from_slice(&(c.len() as u32).to_le_bytes());
                for (a, v) in c {
                    out.extend_from_slice(&a.to_le_bytes());
                    out.extend_from_slice(&v.to_le_bytes());
                }
            }
        }
    }
}

/// A byte reader for the decoders.
pub struct Bytes<'a>(pub &'a [u8]);

impl Bytes<'_> {
    pub fn take(&mut self, n: usize) -> Option<&[u8]> {
        let (a, b) = (self.0.get(..n)?, self.0.get(n..)?);
        self.0 = b;
        Some(a)
    }
    pub fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn u128(&mut self) -> Option<u128> {
        Some(u128::from_le_bytes(self.take(16)?.try_into().unwrap()))
    }
}

/// Read what [`encode_cmd`] wrote.
pub fn decode_cmd(r: &mut Bytes) -> Option<Cmd> {
    Some(match r.take(1)?[0] {
        0 => {
            let (entry, prim, nv) = (r.u32()?, r.u32()?, r.u32()?);
            let mut verts = Vec::with_capacity(nv as usize);
            for _ in 0..nv {
                verts.push([r.u128()?, r.u128()?, r.u128()?]);
            }
            Cmd::Strip { entry, prim, verts }
        }
        1 => {
            let (entry, n) = (r.u32()?, r.u32()?);
            let mut calls = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let k = r.u32()?;
                let mut c = Vec::with_capacity(k as usize);
                for _ in 0..k {
                    c.push((r.u16()?, r.u128()?));
                }
                calls.push(c);
            }
            Cmd::Ribbon { entry, calls }
        }
        2 => {
            let n = r.u32()?;
            let mut steps = Vec::with_capacity(n as usize);
            for _ in 0..n {
                steps.push(match r.take(1)?[0] {
                    0 => {
                        let addr = r.u16()?;
                        let rel = r.take(1)?[0] != 0;
                        let k = r.u32()?;
                        let mut data = Vec::with_capacity(k as usize);
                        for _ in 0..k {
                            data.push(r.u128()?);
                        }
                        Step::Write { addr, rel, data }
                    }
                    1 => Step::Cycle(r.u32()?),
                    2 => Step::Itop(r.u32()?),
                    3 => Step::Mode(r.u32()?),
                    4 => Step::Mask(r.u32()?),
                    5 => Step::Start(r.u32()?),
                    6 => Step::Chain(r.u32()?),
                    7 => {
                        let k = r.u32()?;
                        let mut data = Vec::with_capacity(k as usize);
                        for _ in 0..k {
                            data.push(r.u128()?);
                        }
                        Step::Gif(data)
                    }
                    8 => Step::MaskPath3(r.u32()? != 0),
                    9 => {
                        let k = r.u32()?;
                        let mut data = Vec::with_capacity(k as usize);
                        for _ in 0..k {
                            data.push(r.u128()?);
                        }
                        Step::Raw(data)
                    }
                    _ => return None,
                });
            }
            Cmd::Vu(steps)
        }
        3 => {
            let kind = match r.take(1)?[0] {
                0 => PostKind::DepthFog,
                1 => PostKind::Bloom,
                _ => PostKind::ColorFilter,
            };
            let (windows, k) = (r.u32()?, r.u32()?);
            let mut gif = Vec::with_capacity(k as usize);
            for _ in 0..k {
                gif.push(r.u128()?);
            }
            Cmd::Post { kind, gif, windows }
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_batches_overlap_by_two() {
        assert_eq!(strip_batches(2).count(), 0);
        assert_eq!(strip_batches(3).collect::<Vec<_>>(), vec![0..3]);
        assert_eq!(strip_batches(72).collect::<Vec<_>>(), vec![0..72]);
        assert_eq!(strip_batches(73).collect::<Vec<_>>(), vec![0..72, 70..73]);
        assert_eq!(strip_batches(142).collect::<Vec<_>>(), vec![0..72, 70..142]);
        assert_eq!(strip_batches(143).collect::<Vec<_>>(), vec![0..72, 70..142, 140..143]);
    }

    #[test]
    fn encode_decode_round_trip() {
        let list = vec![
            Recorded { start: 0x10_0000, end: 0x10_0400, first_tag: 7 << 100 | 3, cmd: Cmd::Strip { entry: 0x12b8, prim: 0x5c, verts: vec![[1, 2, 3], [4, 5, u128::MAX]] } },
            Recorded { start: 0x10_0410, end: 0x10_0500, first_tag: 9, cmd: Cmd::Strip { entry: 0x12b0, prim: 0x4c, verts: Vec::new() } },
            Recorded { start: 0x10_0500, end: 0x10_0600, first_tag: 3, cmd: Cmd::Ribbon { entry: 0x3a08, calls: vec![vec![(0, 5), (2, u128::MAX)], vec![]] } },
            Recorded {
                start: 0x10_0700,
                end: 0x10_0800,
                first_tag: 4,
                cmd: Cmd::Vu(vec![Step::Cycle(0x101), Step::Write { addr: 0x11, rel: false, data: vec![1, 2] }, Step::Write { addr: 0, rel: true, data: vec![3] }, Step::Itop(1), Step::Mode(0), Step::Mask(7), Step::Start(0x80)]),
            },
        ];
        let mut b = Vec::new();
        encode(&list, &mut b);
        assert_eq!(decode(&b), Some(list));
    }

    #[test]
    fn tops_alternate_like_mscal() {
        let regs = VifRegs { base: 0x10, ofst: 0x200, tops: 0x210, dbf: true, ..Default::default() };
        assert_eq!(tops(regs, 3), vec![0x210, 0x10, 0x210]);
    }
}
