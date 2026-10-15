//! Top-level execution: the trampoline that enters recompiled functions, the
//! event scheduler (VBlank, alarms, interrupts), and the address → function
//! table that generated code registers.
//!
//! Recompiled functions are plain Rust `fn(&mut Cpu)`. Direct calls between
//! them are direct Rust calls; the trampoline is only used to (re-)enter code
//! at an arbitrary `pc`: at boot, after a thread switch unwound the host
//! stack, for indirect jumps to other functions, and when guest recursion is
//! deeper than the host-stack cap.

use crate::cpu::{Cpu, UNWIND};
use crate::kernel::{self, HANDLER_RETURN, IDLE, THREAD_EXIT};

pub use ssx3_ee_rt::funcs::{call_indirect, has_code, lookup, register_functions, FnEntry, GuestFn};

/// EE clock and NTSC video timing.
pub const EE_CLOCK: u64 = 294_912_000;
pub const FRAME_CYCLES: u64 = EE_CLOCK * 1001 / 60_000; // 59.94 Hz
pub const HSYNC_CYCLES: u64 = FRAME_CYCLES / 263;
/// VBlank lasts ~22 lines on NTSC.
pub const VBLANK_CYCLES: u64 = HSYNC_CYCLES * 22;

/// Per-machine scheduling state that lives outside `Cpu`'s architectural part.
#[derive(Default)]
pub struct Sched {
    pub next_vblank: u64,
    pub next_vblank_end: u64,
    pub in_vblank: bool,
    pub frames: u64,
    pub field: u32,
    /// Set at VBlank start; the host consumes it to present a frame.
    pub frame_ready: bool,
    /// Translated-code misses: pc → count.
    pub misses: std::collections::BTreeMap<u32, u64>,
    pub gs_crt: (u32, u32, u32),
    pub trace_calls: bool,
}

/// A debug switch read once (`getenv` takes a process-wide lock and scans
/// the environment: far too slow for paths taken per call).
fn env_flag(cell: &'static std::sync::OnceLock<bool>, name: &str) -> bool {
    *cell.get_or_init(|| std::env::var_os(name).is_some())
}

/// No translated code at `target` (hook for `call_indirect`).
pub fn miss(c: &mut Cpu, target: u32) {
    *m!(c).sched.misses.entry(target).or_default() += 1;
    fatal(c, target, "no translated code at target");
}

/// Stop the machine with a diagnostic.
pub fn fatal(c: &mut Cpu, pc: u32, msg: &str) {
    if m!(c).kernel.halted.is_none() {
        let ra = c.g[31][0] as u32;
        eprintln!(
            "[fatal] {msg} at {pc:#010x} (ra {ra:#010x}, thread {}, cycles {})",
            m!(c).kernel.current, c.cycles
        );
        m!(c).kernel.halted = Some(format!("{msg} at {pc:#010x}"));
    }
    c.pc = IDLE | UNWIND;
    if m!(c).kernel.host_threads && m!(c).kernel.current != 0 {
        // Park this guest host thread forever; the frame driver sees `halted`.
        let me = crate::threads::my_token();
        crate::threads::handoff(me, 0);
    }
}

/// Called from a generated function whose dispatcher got a `pc` that is not
/// one of its block labels.
pub fn bad_entry(c: &mut Cpu, pc: u32) {
    fatal(c, pc, "pc is not a block label of the enclosing function");
}

/// Run guest code from `c.pc` until `stop` is reached (nested handlers), or
/// until a frame is ready / the machine halts (top level, `stop == None`).
pub fn trampoline(c: &mut Cpu, stop: Option<u32>) {
    loop {
        c.pc &= !UNWIND;
        if Some(c.pc) == stop {
            return;
        }
        if m!(c).kernel.halted.is_some() {
            return;
        }
        if stop.is_none() && m!(c).sched.frame_ready {
            return;
        }
        match c.pc {
            IDLE => {
                if stop.is_some() {
                    // An interrupt handler switched threads; should not happen.
                    fatal(c, IDLE, "idle inside a nested guest call");
                    return;
                }
                idle(c);
                continue;
            }
            THREAD_EXIT => {
                let fake = [0u32; 4];
                let _ = fake;
                c.g[3][0] = 0x23;
                kernel::syscall(c, THREAD_EXIT);
                continue;
            }
            HANDLER_RETURN => {
                fatal(c, HANDLER_RETURN, "handler return outside a handler");
                return;
            }
            _ => {}
        }
        if c.cycles >= c.next_event && service_events(c) {
            continue;
        }
        c.depth = 0;
        let pc = c.pc;
        match lookup(pc) {
            Some(f) => f(c),
            None => miss(c, pc),
        }
    }
}

/// No runnable thread: jump time forward to the next event.
fn idle(c: &mut Cpu) {
    if c.cycles < c.next_event {
        c.cycles = c.next_event;
    }
    service_events(c);
    if c.pc & !UNWIND == IDLE && c.cycles < c.next_event {
        c.cycles = c.next_event;
    }
}

/// Advance hardware time and deliver interrupts. Returns true when the current
/// thread must unwind (thread switch, or the host wants a frame).
pub fn service_events(c: &mut Cpu) -> bool {
    m!(c).hw.now = c.cycles;
    // VBlank start/end.
    if c.cycles >= m!(c).sched.next_vblank {
        m!(c).sched.next_vblank += FRAME_CYCLES;
        m!(c).sched.next_vblank_end = c.cycles + VBLANK_CYCLES;
        m!(c).sched.in_vblank = true;
        m!(c).sched.frames += 1;
        m!(c).sched.field ^= 1;
        let field = m!(c).sched.field;
        m!(c).hw.gs.vblank(field);
        m!(c).hw.raise(crate::hw::int::VBON);
        kernel::on_vblank_start(c);
        crate::pad::refresh(c);
        crate::sif::vblank(c);
        m!(c).sched.frame_ready = true;
    }
    if m!(c).sched.in_vblank && c.cycles >= m!(c).sched.next_vblank_end {
        m!(c).sched.in_vblank = false;
        m!(c).hw.raise(crate::hw::int::VBOF);
    }
    m!(c).hw.timers_service();
    if m!(c).hw.gs.irq_pending() {
        m!(c).hw.raise(crate::hw::int::GS);
    }
    crate::iop_bridge::sync(c); // recompiled IOP (SSX3_IOP=1); no-op otherwise
    let snd_tick = crate::snd_native::service(c); // SSX3_SND=native; no-op otherwise
    crate::sif::pump(c);
    m!(c).hw.irq_dirty = false;
    kernel::service(c);
    // Schedule the next check.
    let mut next = m!(c).sched.next_vblank;
    if m!(c).sched.in_vblank {
        next = next.min(m!(c).sched.next_vblank_end);
    }
    if let Some(a) = kernel::next_alarm(c) {
        next = next.min(a.max(c.cycles + 1));
    }
    if let Some(t) = m!(c).hw.next_timer_event() {
        next = next.min(t.max(c.cycles + 1));
    }
    if let Some(t) = snd_tick {
        next = next.min(t.max(c.cycles + 1));
    }
    // Poll at least every ~20k cycles so pending I/O interrupts get delivered.
    next = next.min(c.cycles + 20_000);
    c.next_event = next;
    let want_frame = m!(c).sched.frame_ready && !m!(c).kernel.in_handler;
    if m!(c).kernel.host_threads {
        // Return the baton to the frame driver; resume here next frame.
        if want_frame && m!(c).kernel.current != 0 {
            let me = crate::threads::my_token();
            if me != 0 {
                crate::threads::handoff(me, 0);
            }
        }
        return false;
    }
    if want_frame {
        c.pc |= UNWIND;
    }
    c.pc & UNWIND != 0
}

/// SetGsCrt(interlace, mode, field/frame).
pub fn set_gs_crt(c: &mut Cpu, interlace: u32, mode: u32, ffmd: u32) {
    m!(c).sched.gs_crt = (interlace, mode, ffmd);
    // SMODE2: INT bit 0, FFMD bit 1.
    m!(c).hw.gs.write_priv(0x20, (interlace & 1) as u64 | ((ffmd & 1) as u64) << 1);
}

/// Run until the next VBlank (one video field) or until the machine halts.
/// Returns false once halted.
pub fn run_frame(c: &mut Cpu) -> bool {
    m!(c).sched.frame_ready = false;
    if m!(c).kernel.host_threads {
        run_frame_threads(c);
    } else {
        trampoline(c, None);
    }
    // Finish background rasterization so `vram` / `display()` are current.
    m!(c).hw.gs.sync();
    m!(c).kernel.halted.is_none()
}

/// Host-thread mode frame driver (runs on the main host thread, token 0).
fn run_frame_threads(c: &mut Cpu) {
    loop {
        if m!(c).kernel.halted.is_some() || m!(c).sched.frame_ready {
            return;
        }
        if m!(c).kernel.current == 0 {
            // Idle: no runnable guest thread. Jump to the next event.
            c.pc = IDLE;
            if c.cycles < c.next_event {
                c.cycles = c.next_event;
            }
            service_events(c);
        } else {
            let cur = m!(c).kernel.current;
            let tok = kernel::token_of(c, cur);
            let tok = if tok == 0 {
                // First run of this guest thread: its context is live in `c`.
                let t = m!(c).kernel.next_token;
                m!(c).kernel.next_token += 1;
                m!(c).kernel.tokens.insert(cur, t);
                crate::threads::spawn(c, t);
                t
            } else {
                tok
            };
            crate::threads::handoff(0, tok);
        }
    }
}

/// Load the executable's segments and start the main thread at its entry.
pub fn boot(c: &mut Cpu, elf: &[u8], argv0: &str) -> Result<u32, String> {
    let entry = crate::elf::load(c, elf)?;
    // Boot arguments block at a free kernel-area address.
    let argv = 0x0000_2000;
    let s = 0x0000_2100;
    let mut bytes = argv0.as_bytes().to_vec();
    bytes.push(0);
    c.bus.write_bytes(s, &bytes);
    c.bus.write32(argv, s);
    kernel::start_main_thread(c, entry, 1, argv);
    m!(c).sched.next_vblank = FRAME_CYCLES;
    c.next_event = 0;
    Ok(entry)
}

/// A computed jump (`jr`) left the current function's labels: either a jump
/// into the middle of this function that isn't a known label (an error), or a
/// tail jump to other code.
pub fn jump_out(c: &mut Cpu, pc: u32, start: u32, end: u32) {
    static JUMPOUT: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if env_flag(&JUMPOUT, "SSX3_TRACE_JUMPOUT") {
        eprintln!(
            "[jump_out] pc {pc:#x} from fn {start:#x}..{end:#x} ra {:#x} thread {}",
            c.g[31][0] as u32, m!(c).kernel.current
        );
    }
    if pc >= start && pc < end {
        bad_entry(c, pc);
        return;
    }
    // `jr t9`-style returns: a computed jump to the current return address is
    // a return, not a tail call into the caller.
    if pc == c.g[31][0] as u32 {
        c.pc = pc;
        return;
    }
    if m!(c).kernel.host_threads {
        // No depth cap in host-thread mode: there is no trampoline to defer to.
        c.pc = pc;
        call_indirect(c, pc);
        return;
    }
    if !c.enter(pc) {
        return;
    }
    call_indirect(c, pc);
    c.leave();
}
