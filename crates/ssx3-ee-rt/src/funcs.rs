//! The address → function table of recompiled EE code, and indirect calls
//! through it.
//!
//! Recompiled functions are plain Rust `fn(&mut Cpu)`. Direct calls between
//! functions in one shard crate are direct Rust calls; everything else (calls
//! across shards, `jalr`, re-entry at an arbitrary `pc`) goes through
//! [`call_indirect`].

use crate::cpu::Cpu;

pub type GuestFn = fn(&mut Cpu);

/// One translated function: `[start, end)` and its entry point. A function
/// accepts any `pc` inside its range that is a block label.
#[derive(Clone, Copy)]
pub struct FnEntry {
    pub start: u32,
    pub end: u32,
    pub f: GuestFn,
}

static mut FUNCS: &[FnEntry] = &[];

/// Register the generated function table (sorted by `start`). Called once at
/// startup by the game crate.
pub fn register_functions(table: &'static [FnEntry]) {
    // SAFETY: called once before any guest code runs, single-threaded.
    unsafe { FUNCS = table };
}

/// The registered table.
pub fn funcs() -> &'static [FnEntry] {
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

/// A debug switch read once (`getenv` takes a process-wide lock and scans
/// the environment: far too slow for paths taken per call).
fn env_flag(cell: &'static std::sync::OnceLock<bool>, name: &str) -> bool {
    *cell.get_or_init(|| std::env::var_os(name).is_some())
}

/// Indirect jump/call into another function (or a resume point).
#[inline(never)]
pub fn call_indirect(c: &mut Cpu, target: u32) {
    static MIDCALL: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if env_flag(&MIDCALL, "SSX3_TRACE_MIDCALL") {
        let t = funcs();
        let i = t.partition_point(|e| e.start <= target);
        if i > 0 && t[i - 1].start != target && target < t[i - 1].end {
            eprintln!(
                "[midcall] {target:#x} inside fn {:#x} (ra {:#x}, thread {})",
                t[i - 1].start,
                c.g[31][0] as u32,
                c.bus.plat.debug_thread()
            );
        }
    }
    match lookup(target) {
        Some(f) => f(c),
        None => (c.hooks.miss)(c, target),
    }
}
