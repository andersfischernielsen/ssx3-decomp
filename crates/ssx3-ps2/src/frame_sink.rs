//! Hand each frame the game publishes to a host renderer (phase 3, draw
//! layer: `docs/plans/phase3-draw-layer.md`).
//!
//! At the same point as `frame_verify` (the kick-semaphore signal at the end
//! of `PS2GraphicsMan_EndFrame`), the typed frame is built from EE memory and
//! passed, with that memory, to the installed [`FrameSink`]. The sink runs on
//! the EE thread before the game continues, so the memory it reads is the
//! frame's.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use ssx3_render::chain::Mem;
use ssx3_render::frame::{self, Built, Carry};

use crate::cpu::Cpu;

/// A host renderer fed with the game's frames.
pub trait FrameSink: Send {
    /// One frame: the GraphicsMan at `gm` published it, `mem` is a
    /// snapshot of EE memory as the game left it at that point. The sink
    /// builds the typed frame from it with a [`FrameBuilder`] (in order:
    /// the builder carries VIF state from frame to frame), may do that and
    /// render later, and hands the snapshot back with [`recycle`].
    fn frame(&mut self, gm: u32, mem: Box<DumpMem>);

    /// The last frame's depth after its 3D phase (VIF phase 1), for the
    /// light visibility queries in native mode.
    fn depth(&mut self) -> Option<Depth> {
        None
    }
}

/// A frame's depth as GS Z values, `w`×`h`; `z(x, y)` converts one pixel
/// from the renderer's depth when it is read (the light queries read a few
/// small rectangles of it).
pub struct Depth {
    pub w: u32,
    pub h: u32,
    pub z: Box<dyn Fn(u32, u32) -> u32 + Send>,
}

struct Sink {
    sink: Box<dyn FrameSink>,
}

/// Builds a sink's typed frames, carrying the VIF state from one frame to
/// the next (frames must be built in order).
#[derive(Default)]
pub struct FrameBuilder {
    carry: Carry,
    /// Mesh chains decoded once (`frame::ChainCache`).
    cache: frame::ChainCache,
    /// Time spent building, frames built.
    pub ms: f64,
    pub frames: u64,
}

impl FrameBuilder {
    /// The typed frame published by the GraphicsMan at `gm`, and the carry
    /// it was built with (for [`dump_frame`]).
    pub fn build(&mut self, gm: u32, mem: &DumpMem) -> (Built, Carry) {
        let t = web_time::Instant::now();
        let carry = self.carry.clone();
        let b = frame::build_cached(mem, gm, &carry, &mut self.cache);
        self.carry = Carry { regs: b.regs_end, matrix: b.emitter.matrix, material_param: b.emitter.material_param };
        self.ms += t.elapsed().as_secs_f64() * 1e3;
        self.frames += 1;
        (b, carry)
    }
}

static SINK: Mutex<Option<Sink>> = Mutex::new(None);
static ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Install the renderer that receives every frame from now on.
pub fn install(sink: Box<dyn FrameSink>) {
    *SINK.lock().unwrap() = Some(Sink { sink });
    ON.store(true, std::sync::atomic::Ordering::Release);
}

pub fn enabled() -> bool {
    ON.load(std::sync::atomic::Ordering::Acquire)
}

// Native mode: the sink is the renderer. While the game publishes frames
// the emulated GS stops rasterizing (it still runs state, transfers and
// events, so the game sees the same hardware), and the Z buffer the light
// visibility queries read back is the sink's depth. Movies and loading
// screens, which do not publish frames, draw on the emulated GS as before.

/// Frames, ms in `frame::build`, ms in the sink.
static TIMING: Mutex<(u64, f64, f64)> = Mutex::new((0, 0.0, 0.0));

/// Mean time per published frame on the game's thread (memory snapshot and
/// hand-over).
pub fn timing_report() -> String {
    let t = TIMING.lock().unwrap();
    let n = t.0.max(1) as f64;
    format!("frame sink: {:.2} ms per frame on the game thread ({} frames)", t.2 / n, t.0)
}

static NATIVE: AtomicBool = AtomicBool::new(false);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static LAST_PUBLISH: AtomicU64 = AtomicU64::new(0);

/// The published frame's chain heads: VIF1 phase 1/2, GIF phase 1/2.
static HEADS: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];

/// Native mode: the DMA kick of a chain the sink already drew (`ch`: true
/// for VIF1, false for GIF) completes without running it. The game's
/// handshake (DMA end interrupts, the kick thread's state machine) is
/// unchanged; no VIF, VU1 or GS work happens for the frame.
pub fn skip_chain(vif: bool, tadr: u32) -> bool {
    static RUN: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    // (Any chain of a frame the sink drew, even while the game publishes
    // frames only now and then, as on loading screens.)
    if !native() || tadr == 0 || *RUN.get_or_init(|| std::env::var_os("SSX3_NATIVE_RUN_CHAINS").is_some()) {
        return false;
    }
    let k = if vif { 0 } else { 2 };
    HEADS[k..k + 2].iter().any(|h| h.load(Ordering::Acquire) == tadr as u64)
}

/// The sink's latest depth (after the 3D phase).
static DEPTH: Mutex<Option<Depth>> = Mutex::new(None);

/// Native mode: run `f` on the sink's latest depth, for the light
/// visibility queries.
pub fn with_depth<R>(f: impl FnOnce(&Depth) -> R) -> Option<R> {
    let g = DEPTH.lock().unwrap();
    Some(f(g.as_ref()?))
}

// View settings: the aspect ratio the game renders for and a field-of-view
// multiplier. The game's own widescreen setting (gm+0x6b94, written by
// 0x377950 when the option changes) scales its projection in
// `PS2GraphicsMan_SetPerspective` by gm+0x6ba0 (x) and gm+0x6ba4 (y) before
// it derives its cull matrices; mode 2 is "anamorphic" (x 0.75). Writing those
// fields at every publish makes the next frame's projection and culling
// match any aspect and field of view.

static VIEW_ASPECT: AtomicU64 = AtomicU64::new(0);
static VIEW_FOV: AtomicU64 = AtomicU64::new(0);

/// Render for `aspect` (width / height; 0: the game's own setting) with the
/// field of view scaled by `fov` (1: the game's).
pub fn set_view(aspect: f32, fov: f32) {
    VIEW_ASPECT.store(aspect.to_bits() as u64, Ordering::Release);
    VIEW_FOV.store(fov.to_bits() as u64, Ordering::Release);
}

/// The aspect set with [`set_view`] (0: the game's own).
pub fn view_aspect() -> f32 {
    f32::from_bits(VIEW_ASPECT.load(Ordering::Acquire) as u32)
}

mod view {
    pub const MODE: u32 = 0x6b94;
    pub const Y_OFFSET: u32 = 0x6b98;
    pub const X_SCALE2: u32 = 0x6b9c;
    pub const X_SCALE: u32 = 0x6ba0;
    pub const Y_SCALE: u32 = 0x6ba4;
}

fn apply_view(c: &mut Cpu, gm: u32) {
    let aspect = view_aspect();
    let fov = f32::from_bits(VIEW_FOV.load(Ordering::Acquire) as u32);
    let fov = if fov > 0.0 { fov } else { 1.0 };
    if aspect <= 0.0 && fov == 1.0 {
        return;
    }
    // The game renders for 4:3 at x 1.0 (and 16:9 anamorphic at x 0.75).
    let x = if aspect > 0.0 { (4.0 / 3.0) / aspect } else { 1.0 } / fov;
    c.bus.write32(gm + view::MODE, 2);
    c.bus.write32(gm + view::Y_OFFSET, 0);
    c.bus.write32(gm + view::X_SCALE2, 1.0f32.to_bits());
    c.bus.write32(gm + view::X_SCALE, x.to_bits());
    c.bus.write32(gm + view::Y_SCALE, (1.0 / fov).to_bits());
}

/// Frames published this recently (VBlanks) keep the native path active.
const ACTIVE_VBLANKS: u64 = 2;

pub fn set_native(on: bool) {
    NATIVE.store(on, Ordering::Release);
}

pub fn native() -> bool {
    NATIVE.load(Ordering::Acquire)
}

/// Native mode and the game is publishing frames: the sink's image is the
/// one to show.
pub fn native_active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

/// At each VBlank: switch the emulated GS's rasterizing off while frames
/// are being published, back on when they stop.
pub fn on_vblank(c: &mut Cpu) {
    if !native() {
        return;
    }
    let frames = crate::mr!(c).sched.frames;
    let last = LAST_PUBLISH.load(Ordering::Acquire);
    let active = last != 0 && frames.saturating_sub(last) <= ACTIVE_VBLANKS;
    if active != ACTIVE.swap(active, Ordering::AcqRel) {
        crate::m!(c).hw.gs.set_draws_suppressed(active);
    }
}

struct EeMem<'a> {
    ram: &'a [u8],
    spr: &'a [u8],
}

impl Mem for EeMem<'_> {
    fn qword(&self, addr: u32) -> u128 {
        crate::hw::mem_q(self.ram, self.spr, addr, addr & 0x8000_0000 != 0)
    }
}

/// `SignalSema(id)` from the EE: if it is the kick semaphore, the frame was
/// just published.
pub fn on_signal_sema(c: &mut Cpu, id: i32) {
    let mem = EeMem { ram: &c.bus.ram, spr: &c.bus.spr };
    let g = frame::word(&mem, frame::addr::GM_PTR);
    if g == 0 || frame::word(&mem, g + frame::gm::KICK_SEMA) as i32 != id {
        return;
    }
    let mut guard = SINK.lock().unwrap();
    let Some(s) = guard.as_mut() else { return };
    let t0 = web_time::Instant::now();
    let heads = [frame::gm::QUEUED_VIF1, frame::gm::QUEUED_VIF2, frame::gm::QUEUED_GIF1, frame::gm::QUEUED_GIF2].map(|o| frame::word(&mem, g + o));
    // The memory the sink reads: a copy, so the sink can build and render
    // the frame while the game goes on. Each buffer gets only the pages
    // written since it was last filled (`bus::take_written`).
    let mut snap = POOL.lock().unwrap().pop().unwrap_or_else(|| Box::new(DumpMem { ram: Vec::new(), spr: Vec::new(), vu1: Vec::new(), id: usize::MAX }));
    snapshot_ram(&mut snap, &c.bus.ram);
    snap.spr.clear();
    snap.spr.extend_from_slice(&c.bus.spr);
    s.sink.frame(g, snap);
    // For the next frame (this one's projection is already built).
    apply_view(c, g);
    {
        let mut t = TIMING.lock().unwrap();
        t.0 += 1;
        t.2 += t0.elapsed().as_secs_f64() * 1e3;
    }
    if native() {
        for (h, v) in HEADS.iter().zip(heads) {
            h.store(v as u64, Ordering::Release);
        }
        let depth = s.sink.depth();
        drop(guard);
        LAST_PUBLISH.store(crate::mr!(c).sched.frames.max(1), Ordering::Release);
        if let Some(d) = depth {
            *DEPTH.lock().unwrap() = Some(d);
        }
    }
}

// Frame dumps (debug): `SSX3_FRAME_DUMP=<n>` keeps the inputs of the last n
// frames (EE RAM, scratchpad, the carried chain state and the sink's VU1
// data memory at that point), so a renderer can
// be run on them offline (`gfx_replay`). They contain game data: keep them
// out of the repository.

static RING: Mutex<std::collections::VecDeque<Vec<u8>>> = Mutex::new(std::collections::VecDeque::new());

const DUMP_MAGIC: &[u8; 8] = b"SSX3FRM1";

fn dump_keep() -> Option<usize> {
    static K: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();
    *K.get_or_init(|| std::env::var("SSX3_FRAME_DUMP").ok().and_then(|v| v.parse().ok()).filter(|&n| n > 0))
}

fn encode(carry: &Carry, ram: &[u8], spr: &[u8], vu1: &[u8]) -> Vec<u8> {
    let r = &carry.regs;
    let mut words = vec![r.cycle, r.mode, r.mask];
    words.extend(r.row);
    words.extend(r.col);
    words.extend([r.base, r.ofst, r.tops, r.itops, r.dbf as u32, carry.matrix as u32, carry.material_param]);
    let mut out = Vec::with_capacity(16 + words.len() * 4 + ram.len() + spr.len());
    out.extend_from_slice(DUMP_MAGIC);
    out.extend_from_slice(&(words.len() as u32).to_le_bytes());
    out.extend_from_slice(&(ram.len() as u32).to_le_bytes());
    out.extend(words.iter().flat_map(|w| w.to_le_bytes()));
    out.extend_from_slice(ram);
    out.extend_from_slice(spr);
    out.extend_from_slice(vu1);
    out
}

/// Keep a frame's inputs in the dump ring (`SSX3_FRAME_DUMP`): the carry it
/// was built with, its memory and the sink's VU1 memory at its start.
pub fn dump_frame(carry: &Carry, mem: &DumpMem, vu1: &[u8]) {
    let Some(keep) = dump_keep() else { return };
    let mut ring = RING.lock().unwrap();
    while ring.len() >= keep {
        ring.pop_front();
    }
    ring.push_back(encode(carry, &mem.ram, &mem.spr, vu1));
}

/// Write the kept frames to `dir/frame-K.bin`, K = 0 for the newest.
pub fn write_dumps(dir: &str) -> std::io::Result<usize> {
    let ring = RING.lock().unwrap();
    for (k, d) in ring.iter().rev().enumerate() {
        std::fs::write(format!("{dir}/frame-{k}.bin"), d)?;
    }
    Ok(ring.len())
}

/// Snapshot buffers handed back by sinks.
static POOL: Mutex<Vec<Box<DumpMem>>> = Mutex::new(Vec::new());

/// Per snapshot buffer (`DumpMem::id`): the RAM pages written since it was
/// last filled.
static STALE: Mutex<Vec<Vec<u64>>> = Mutex::new(Vec::new());

/// Bring `snap` up to date with `ram`: the pages written since its last
/// fill (all of them for a new buffer). `SSX3_SNAPSHOT_CHECK=1` compares it
/// with a full copy (a page written outside the bus or DMA paths would
/// show here).
fn snapshot_ram(snap: &mut DumpMem, ram: &[u8]) {
    let pages = ram.len() / 4096;
    let mut stale = STALE.lock().unwrap();
    let mut written = vec![0u64; pages.div_ceil(64)];
    ssx3_ee_rt::bus::take_written(&mut written);
    for s in stale.iter_mut() {
        for (a, b) in s.iter_mut().zip(&written) {
            *a |= b;
        }
    }
    if snap.id == usize::MAX || snap.ram.len() != ram.len() {
        snap.id = stale.len();
        stale.push(vec![!0u64; pages.div_ceil(64)]);
        snap.ram.resize(ram.len(), 0);
    }
    let bits = &mut stale[snap.id];
    for (w, word) in bits.iter_mut().enumerate() {
        let mut m = std::mem::take(word);
        while m != 0 {
            let p = w * 64 + m.trailing_zeros() as usize;
            m &= m - 1;
            let r = p * 4096..(p + 1) * 4096;
            snap.ram[r.clone()].copy_from_slice(&ram[r]);
        }
    }
    static CHECK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *CHECK.get_or_init(|| std::env::var_os("SSX3_SNAPSHOT_CHECK").is_some()) {
        let bad: Vec<usize> = (0..pages).filter(|&p| snap.ram[p * 4096..(p + 1) * 4096] != ram[p * 4096..(p + 1) * 4096]).collect();
        if !bad.is_empty() {
            eprintln!("[snapshot-check] {} stale pages, first at {:#x}", bad.len(), bad[0] * 4096);
            // (Copy them so the check reports each miss once.)
            for p in bad {
                let r = p * 4096..(p + 1) * 4096;
                snap.ram[r.clone()].copy_from_slice(&ram[r]);
            }
        }
    }
}

/// Hand a memory snapshot back for reuse (it is 32 MiB).
pub fn recycle(m: Box<DumpMem>) {
    let mut p = POOL.lock().unwrap();
    if p.len() < 4 {
        p.push(m);
    }
}

/// EE memory from a frame dump, or a snapshot handed to a sink.
pub struct DumpMem {
    pub ram: Vec<u8>,
    pub spr: Vec<u8>,
    /// The emulated VU1 data memory when the frame was published.
    pub vu1: Vec<u8>,
    /// The snapshot buffer's slot (`usize::MAX`: a dump, or not filled yet).
    pub id: usize,
}

impl Mem for DumpMem {
    fn qword(&self, addr: u32) -> u128 {
        crate::hw::mem_q(&self.ram, &self.spr, addr, addr & 0x8000_0000 != 0)
    }
}

/// Read a frame dump: its memory and the carry its frame was built with.
pub fn load_dump(bytes: &[u8]) -> Result<(DumpMem, Carry), String> {
    if bytes.len() < 16 || &bytes[..8] != DUMP_MAGIC {
        return Err("not a frame dump".into());
    }
    let u = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
    let (nw, nram) = (u(8) as usize, u(12) as usize);
    let w: Vec<u32> = (0..nw).map(|i| u(16 + 4 * i)).collect();
    if nw != 18 {
        return Err(format!("unexpected carry size {nw}"));
    }
    let o = 16 + 4 * nw;
    if bytes.len() < o + nram + 0x4000 {
        return Err("truncated frame dump".into());
    }
    let mut regs = ssx3_render::vif::VifRegs::default();
    regs.cycle = w[0];
    regs.mode = w[1];
    regs.mask = w[2];
    regs.row.copy_from_slice(&w[3..7]);
    regs.col.copy_from_slice(&w[7..11]);
    regs.base = w[11];
    regs.ofst = w[12];
    regs.tops = w[13];
    regs.itops = w[14];
    regs.dbf = w[15] != 0;
    let carry = Carry { regs, matrix: w[16] as i32, material_param: w[17] };
    let vu1 = bytes.get(o + nram + 0x4000..o + nram + 0x8000).map_or_else(Vec::new, |v| v.to_vec());
    Ok((DumpMem { ram: bytes[o..o + nram].to_vec(), spr: bytes[o + nram..o + nram + 0x4000].to_vec(), vu1, id: usize::MAX }, carry))
}

/// Build the typed frame from a dump's memory.
pub fn build_dump(mem: &DumpMem, carry: &Carry) -> Option<Built> {
    build_dump_cached(mem, carry, &mut frame::ChainCache::default())
}

/// [`build_dump`] with a sub-chain cache carried across dumps.
pub fn build_dump_cached(mem: &DumpMem, carry: &Carry, cache: &mut frame::ChainCache) -> Option<Built> {
    let g = frame::word(mem, frame::addr::GM_PTR);
    (g != 0).then(|| frame::build_cached(mem, g, carry, cache))
}
