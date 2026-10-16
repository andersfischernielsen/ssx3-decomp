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
    let b = frame::build(&mem, g, &s.carry);
    s.sink.frame(&b, &mem);
    s.carry = Carry { regs: b.regs_end, matrix: b.emitter.matrix, material_param: b.emitter.material_param };
}
