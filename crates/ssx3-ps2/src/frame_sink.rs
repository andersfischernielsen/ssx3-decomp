//! Hand each frame the game publishes to a host renderer (phase 3, draw
//! layer: `docs/plans/phase3-draw-layer.md`).
//!
//! At the same point as `frame_verify` (the kick-semaphore signal at the end
//! of `PS2GraphicsMan_EndFrame`), the typed frame is built from EE memory and
//! passed, with that memory, to the installed [`FrameSink`]. The sink runs on
//! the EE thread before the game continues, so the memory it reads is the
//! frame's.

use std::sync::Mutex;

use ssx3_render::chain::Mem;
use ssx3_render::frame::{self, Built, Carry};

use crate::cpu::Cpu;

/// A host renderer fed with the game's frames.
pub trait FrameSink: Send {
    /// One frame: the typed frame and its chain events, and EE memory as
    /// the game left it at that point.
    fn frame(&mut self, built: &Built, mem: &dyn Mem);
}

struct Sink {
    sink: Box<dyn FrameSink>,
    carry: Carry,
}

static SINK: Mutex<Option<Sink>> = Mutex::new(None);
static ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Install the renderer that receives every frame from now on.
pub fn install(sink: Box<dyn FrameSink>) {
    *SINK.lock().unwrap() = Some(Sink { sink, carry: Carry::default() });
    ON.store(true, std::sync::atomic::Ordering::Release);
}

pub fn enabled() -> bool {
    ON.load(std::sync::atomic::Ordering::Acquire)
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
pub fn on_signal_sema(c: &Cpu, id: i32) {
    let mem = EeMem { ram: &c.bus.ram, spr: &c.bus.spr };
    let g = frame::word(&mem, frame::addr::GM_PTR);
    if g == 0 || frame::word(&mem, g + frame::gm::KICK_SEMA) as i32 != id {
        return;
    }
    let mut guard = SINK.lock().unwrap();
    let Some(s) = guard.as_mut() else { return };
    if let Some(keep) = dump_keep() {
        let mut ring = RING.lock().unwrap();
        while ring.len() >= keep {
            ring.pop_front();
        }
        ring.push_back(encode(&s.carry, &c.bus.ram, &c.bus.spr, &crate::mr!(c).hw.vu1.data));
    }
    let b = frame::build(&mem, g, &s.carry);
    s.sink.frame(&b, &mem);
    s.carry = Carry { regs: b.regs_end, matrix: b.emitter.matrix, material_param: b.emitter.material_param };
}

// Frame dumps (debug): `SSX3_FRAME_DUMP=<n>` keeps the inputs of the last n
// frames (EE RAM, scratchpad, the carried chain state and the emulated VU1
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

/// Write the kept frames to `dir/frame-K.bin`, K = 0 for the newest.
pub fn write_dumps(dir: &str) -> std::io::Result<usize> {
    let ring = RING.lock().unwrap();
    for (k, d) in ring.iter().rev().enumerate() {
        std::fs::write(format!("{dir}/frame-{k}.bin"), d)?;
    }
    Ok(ring.len())
}

/// EE memory from a frame dump.
pub struct DumpMem {
    pub ram: Vec<u8>,
    pub spr: Vec<u8>,
    /// The emulated VU1 data memory when the frame was published.
    pub vu1: Vec<u8>,
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
    Ok((DumpMem { ram: bytes[o..o + nram].to_vec(), spr: bytes[o + nram..o + nram + 0x4000].to_vec(), vu1 }, carry))
}

/// Build the typed frame from a dump's memory.
pub fn build_dump(mem: &DumpMem, carry: &Carry) -> Option<Built> {
    let g = frame::word(mem, frame::addr::GM_PTR);
    (g != 0).then(|| frame::build(mem, g, carry))
}
