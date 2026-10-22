//! Dynamic call graph of recompiled EE code (feature `callprof`).
//!
//! `recomp-ee` puts `prof::enter(c, start)` at the top of every generated
//! function. Without the feature it is an empty inline function and compiles
//! away. With it, every call between frames `from` and `to` is counted as an
//! edge caller → callee, and the table is written when frame `to` ends:
//!
//! ```text
//! SSX3_CALLPROF=<out.tsv>:<from>:<to>
//! ```
//!
//! The caller is the function containing the return address (`ra - 8`), so a
//! tail call (`j`) is attributed to the function that called the tail-caller,
//! and resume entries (mid-function `pc`) count as calls of that function.
//! The runner reports frame boundaries with [`frame`].

use crate::cpu::Cpu;

#[inline(always)]
pub fn enter(c: &Cpu, start: u32) {
    #[cfg(feature = "callprof")]
    imp::enter(c, start);
    #[cfg(feature = "calltrace")]
    trace::enter(c, start);
    let _ = (c, start);
}

/// Differential tests (feature `calltrace`): the calls a function makes,
/// with their arguments, so a replacement that passes a wrong argument to a
/// real callee shows up even when the result does not.
#[cfg(feature = "calltrace")]
pub mod trace {
    use crate::cpu::Cpu;
    use std::cell::RefCell;

    /// (callee, a0, a1, a2, a3, f12, f13)
    pub type Call = [u64; 7];

    thread_local! {
        static T: RefCell<Option<Vec<Call>>> = const { RefCell::new(None) };
    }

    /// Start recording on this thread.
    pub fn start() {
        T.with(|t| *t.borrow_mut() = Some(Vec::new()));
    }

    /// Stop recording; the calls since [`start`].
    pub fn take() -> Vec<Call> {
        T.with(|t| t.borrow_mut().take().unwrap_or_default())
    }

    pub(super) fn enter(c: &Cpu, start: u32) {
        T.with(|t| {
            if let Some(v) = t.borrow_mut().as_mut() {
                if v.len() < 4096 {
                    v.push([start as u64, c.g[4][0], c.g[5][0], c.g[6][0], c.g[7][0], c.f[12] as u64, c.f[13] as u64]);
                }
            }
        });
    }
}

/// Frame `n` has just finished.
pub fn frame(n: u64) {
    #[cfg(feature = "callprof")]
    imp::frame(n);
    #[cfg(not(feature = "callprof"))]
    let _ = n;
}

#[cfg(feature = "callprof")]
mod imp {
    use crate::cpu::{reg::RA, Cpu};
    use crate::funcs::funcs;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, OnceLock};

    struct Conf {
        out: String,
        from: u64,
        to: u64,
    }

    fn conf() -> Option<&'static Conf> {
        static C: OnceLock<Option<Conf>> = OnceLock::new();
        C.get_or_init(|| {
            let v = std::env::var("SSX3_CALLPROF").ok()?;
            let mut it = v.rsplitn(3, ':');
            let to = it.next()?.parse().ok()?;
            let from = it.next()?.parse().ok()?;
            Some(Conf { out: it.next()?.to_string(), from, to })
        })
        .as_ref()
    }

    static ON: AtomicBool = AtomicBool::new(false);
    static EDGES: Mutex<Option<HashMap<(u32, u32), u64>>> = Mutex::new(None);

    fn func_start(pc: u32) -> u32 {
        let t = funcs();
        let i = t.partition_point(|e| e.start <= pc);
        if i > 0 && pc < t[i - 1].end {
            t[i - 1].start
        } else {
            0
        }
    }

    pub fn enter(c: &Cpu, start: u32) {
        if !ON.load(Ordering::Relaxed) {
            return;
        }
        let caller = func_start((c.g[RA][0] as u32).wrapping_sub(8));
        let mut g = EDGES.lock().unwrap();
        *g.get_or_insert_with(HashMap::new).entry((caller, start)).or_insert(0) += 1;
    }

    pub fn frame(n: u64) {
        let Some(cf) = conf() else { return };
        // Frame n is done: count frames from+1 ..= to.
        if n == cf.from {
            ON.store(true, Ordering::Relaxed);
        } else if n == cf.to {
            ON.store(false, Ordering::Relaxed);
            let edges = EDGES.lock().unwrap().take().unwrap_or_default();
            let mut v: Vec<_> = edges.into_iter().collect();
            v.sort();
            let mut s = format!("# frames {}..={}\n# caller\tcallee\tcount\n", cf.from + 1, cf.to);
            for ((a, b), n) in v {
                s += &format!("{a:08x}\t{b:08x}\t{n}\n");
            }
            match std::fs::write(&cf.out, s) {
                Ok(()) => eprintln!("[callprof] frames {}..={} -> {}", cf.from + 1, cf.to, cf.out),
                Err(e) => eprintln!("[callprof] {}: {e}", cf.out),
            }
        }
    }
}
