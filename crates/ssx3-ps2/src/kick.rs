//! N4: the game's DMA kick thread (`Renderer_DmaKickThread`, 0x382760) as
//! native code, in native mode.
//!
//! The guest thread (priority 5, above the game's threads) waits on the
//! kick semaphore and runs a state machine over `kickState` (gm+0x5a8c):
//!
//! - 0, the queued half's buffer state 2 ("queued"): take it (state 3, "in
//!   flight"), kick its phase-1 chains -> 1;
//! - the VIF1 DMA end interrupt: 1 -> 2 and a signal; 2: the phase-2
//!   callback, kick the phase-2 chains -> 3; their end: 3 -> 4;
//! - `PS2GraphicsMan_OnVBlankStart`: 4 and the frame interval reached -> 5
//!   and a signal; 5: the half is free again (0), `Renderer_SetDisplayRegs`
//!   flips (its packet is the renderer's frame blend and clear), -> 0, and
//!   the next queued half is kicked at once.
//!
//! In native mode the sink draws every kicked chain, so the kicks complete
//! without running (`frame_sink::skip_chain`). Here no chain is kicked at
//! all: a signal of the kick semaphore runs the step right away (the thread
//! would preempt the signalling thread) and phases 1 and 2 complete in it.
//! A signal from an interrupt handler (the VBlank flip) runs after the
//! handlers, as the thread would. The guest thread stays blocked on the
//! semaphore. `SSX3_NATIVE_KICK=0` keeps the guest thread.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::cpu::Cpu;
use ssx3_render::frame::{addr, gm};

/// `int kickState`.
const STATE: u32 = 0x5a8c;
/// `int bufState[2]`: 0 free, 1 building, 2 queued, 3 in flight.
const BUF_STATE: u32 = 0x5a90;
/// The half in flight.
const CURRENT: u32 = 0x5a98;
/// The phase-2 chain heads of the half in flight (VIF1, GIF).
const PHASE2_VIF: u32 = 0x5a9c;
const PHASE2_GIF: u32 = 0x5aa0;
/// The queued half.
const QUEUED: u32 = 0x5aa4;
/// The kick thread's id.
const THREAD: u32 = 0x5ad0;
/// `void (*phase2Callback)(void *)`, its argument.
const PHASE2_CALLBACK: u32 = 0x6d0c;
const PHASE2_ARG: u32 = 0x6d10;
/// `Renderer_SetDisplayRegs(gm, pmode, waitVsync)`.
const SET_DISPLAY_REGS: u32 = 0x0038_2af0;

/// Kick-semaphore signals from interrupt handlers not yet run.
static PENDING: AtomicU32 = AtomicU32::new(0);

fn on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    crate::frame_sink::native()
        && *ON.get_or_init(|| std::env::var("SSX3_NATIVE_KICK").map_or(true, |v| v != "0"))
}

/// The GraphicsMan if `id` is its kick semaphore and the kick is native.
fn kick_gm(c: &mut Cpu, id: i32) -> Option<u32> {
    if !on() {
        return None;
    }
    let g = c.bus.read32(addr::GM_PTR);
    (g != 0 && c.bus.read32(g + gm::KICK_SEMA) as i32 == id).then_some(g)
}

/// `SignalSema` / `iSignalSema(id)`: true if this took the signal (the kick
/// semaphore, native mode).
pub fn on_signal(c: &mut Cpu, id: i32, interrupt: bool) -> bool {
    let Some(g) = kick_gm(c, id) else {
        return false;
    };
    if interrupt {
        PENDING.fetch_add(1, Ordering::Relaxed);
    } else {
        step(c, g);
    }
    true
}

/// After the interrupt handlers: the steps their signals asked for.
pub fn service(c: &mut Cpu) {
    while PENDING.load(Ordering::Relaxed) != 0 {
        PENDING.fetch_sub(1, Ordering::Relaxed);
        let g = c.bus.read32(addr::GM_PTR);
        if g != 0 {
            step(c, g);
        }
    }
}

/// One pass of the thread's loop.
fn step(c: &mut Cpu, g: u32) {
    let r = |c: &mut Cpu, o: u32| c.bus.read32(g + o);
    let w = |c: &mut Cpu, o: u32, v: u32| c.bus.write32(g + o, v);
    let buf = |i: u32| BUF_STATE + i.wrapping_mul(4);
    if r(c, STATE) == 5 {
        let cur = r(c, CURRENT);
        w(c, buf(cur), 0);
        w(c, STATE, 0);
        guest(c, g, SET_DISPLAY_REGS, [g, 1, 0, 0]);
    }
    let state = r(c, STATE);
    let q = r(c, QUEUED);
    if state == 0 && r(c, buf(q)) == 2 {
        w(c, CURRENT, q);
        w(c, buf(q), 3);
        let (v2, g2) = (r(c, gm::QUEUED_VIF2), r(c, gm::QUEUED_GIF2));
        w(c, PHASE2_VIF, v2);
        w(c, PHASE2_GIF, g2);
        // Phase 1 drawn by the sink; its end interrupt's 1 -> 2.
        w(c, STATE, 2);
        phase2(c, g);
    } else if state == 2 {
        phase2(c, g);
    }
}

/// Phase 2: the callback, then the phase-2 chains (drawn by the sink; their
/// end interrupt's 3 -> 4).
fn phase2(c: &mut Cpu, g: u32) {
    let f = c.bus.read32(g + PHASE2_CALLBACK);
    if f != 0 {
        let arg = c.bus.read32(g + PHASE2_ARG);
        guest(c, g, f, [arg, 0, 0, 0]);
    }
    c.bus.write32(g + STATE, 4);
}

/// Guest code run as the kick thread would (its gp).
fn guest(c: &mut Cpu, g: u32, f: u32, a: [u32; 4]) {
    let tid = c.bus.read32(g + THREAD) as i32;
    let gp = m!(c).kernel.threads.get(&tid).map_or(0, |t| t.gp);
    crate::kernel::run_guest(c, f, a, gp);
}
