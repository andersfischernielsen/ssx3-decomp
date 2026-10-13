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

pub type GuestFn = fn(&mut Cpu);

/// One translated function: `[start, end)` and its entry point. A function
/// accepts any `pc` inside its range that is a block label.
#[derive(Clone, Copy)]
pub struct FnEntry {
    pub start: u32,
    pub end: u32,
    pub f: GuestFn,
}

/// EE clock and NTSC video timing.
pub const EE_CLOCK: u64 = 294_912_000;
pub const FRAME_CYCLES: u64 = EE_CLOCK * 1001 / 60_000; // 59.94 Hz
pub const HSYNC_CYCLES: u64 = FRAME_CYCLES / 263;
/// VBlank lasts ~22 lines on NTSC.
pub const VBLANK_CYCLES: u64 = HSYNC_CYCLES * 22;

static mut FUNCS: &[FnEntry] = &[];

/// Register the generated function table (sorted by `start`). Called once at
/// startup by the game crate.
pub fn register_functions(table: &'static [FnEntry]) {
    // SAFETY: called once before any guest code runs, single-threaded.
    unsafe { FUNCS = table };
}

fn funcs() -> &'static [FnEntry] {
    // SAFETY: written once at startup.
    unsafe { FUNCS }
}

pub fn lookup(pc: u32) -> Option<GuestFn> {
    let t = funcs();
    let i = t.partition_point(|e| e.start <= pc);
    if i == 0 {
        return None;
    }
    let e = &t[i - 1];
    (pc < e.end).then_some(e.f)
}

pub fn has_code(pc: u32) -> bool {
    lookup(pc).is_some()
}

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

/// Indirect jump/call into another function (or a resume point).
#[inline(never)]
pub fn call_indirect(c: &mut Cpu, target: u32) {
    match lookup(target) {
        Some(f) => f(c),
        None => miss(c, target),
    }
}

fn miss(c: &mut Cpu, target: u32) {
    *c.sched.misses.entry(target).or_default() += 1;
    fatal(c, target, "no translated code at target");
}

/// Stop the machine with a diagnostic.
pub fn fatal(c: &mut Cpu, pc: u32, msg: &str) {
    if c.kernel.halted.is_none() {
        let ra = c.g[31][0] as u32;
        eprintln!(
            "[fatal] {msg} at {pc:#010x} (ra {ra:#010x}, thread {}, cycles {})",
            c.kernel.current, c.cycles
        );
        c.kernel.halted = Some(format!("{msg} at {pc:#010x}"));
    }
    c.pc = IDLE | UNWIND;
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
        if c.kernel.halted.is_some() {
            return;
        }
        if stop.is_none() && c.sched.frame_ready {
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
    c.bus.hw.now = c.cycles;
    // VBlank start/end.
    if c.cycles >= c.sched.next_vblank {
        c.sched.next_vblank += FRAME_CYCLES;
        c.sched.next_vblank_end = c.cycles + VBLANK_CYCLES;
        c.sched.in_vblank = true;
        c.sched.frames += 1;
        c.sched.field ^= 1;
        let field = c.sched.field;
        c.bus.hw.gs.vblank(field);
        c.bus.hw.raise(crate::hw::int::VBON);
        kernel::on_vblank_start(c);
        c.sched.frame_ready = true;
    }
    if c.sched.in_vblank && c.cycles >= c.sched.next_vblank_end {
        c.sched.in_vblank = false;
        c.bus.hw.raise(crate::hw::int::VBOF);
    }
    if c.bus.hw.gs.irq_pending() {
        c.bus.hw.raise(crate::hw::int::GS);
    }
    crate::sif::pump(c);
    c.bus.hw.irq_dirty = false;
    kernel::service(c);
    // Schedule the next check.
    let mut next = c.sched.next_vblank;
    if c.sched.in_vblank {
        next = next.min(c.sched.next_vblank_end);
    }
    if let Some(a) = kernel::next_alarm(c) {
        next = next.min(a.max(c.cycles + 1));
    }
    // Poll at least every ~20k cycles so pending I/O interrupts get delivered.
    next = next.min(c.cycles + 20_000);
    c.next_event = next;
    let want_frame = c.sched.frame_ready && !c.kernel.in_handler;
    if want_frame {
        c.pc |= UNWIND;
    }
    c.pc & UNWIND != 0
}

/// SetGsCrt(interlace, mode, field/frame).
pub fn set_gs_crt(c: &mut Cpu, interlace: u32, mode: u32, ffmd: u32) {
    c.sched.gs_crt = (interlace, mode, ffmd);
    // SMODE2: INT bit 0, FFMD bit 1.
    c.bus.hw.gs.write_priv(0x20, (interlace & 1) as u64 | ((ffmd & 1) as u64) << 1);
}

/// Run until the next VBlank (one video field) or until the machine halts.
/// Returns false once halted.
pub fn run_frame(c: &mut Cpu) -> bool {
    c.sched.frame_ready = false;
    trampoline(c, None);
    c.kernel.halted.is_none()
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
    c.sched.next_vblank = FRAME_CYCLES;
    c.next_event = 0;
    Ok(entry)
}
