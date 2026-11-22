//! Bounded observations at the native memory API, without allocator changes.
//!
//! Preserve the first invalid heap state before a later allocation can loop
//! or overwrite the evidence. Pool ownership and all native calls stay intact.

use ssx3_ee_rt::cpu::reg::{A0, A1, A2, A3, RA, SP};
use ssx3_ee_rt::Cpu;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, OnceLock};

const HEAP: u32 = 0x004f_f250;
const RECENT_LIMIT: usize = 256;
const OWNER_LIMIT: usize = 8192;
const JOURNAL_LIMIT: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct Site {
    cycles: u64,
    ra: u32,
    stack: u32,
    args: [u32; 4],
    callers: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Owner {
    allocation: Site,
    last_free: Option<Site>,
    previous_lifetime: Option<(Site, Option<Site>)>,
}

#[derive(Default)]
struct Trace {
    recent: VecDeque<String>,
    owners: HashMap<u32, Owner>,
    reported: bool,
    saw_arena: bool,
    journal: Option<File>,
    journal_opened: bool,
    journal_bytes: usize,
    journal_index: u8,
}

static TRACE: LazyLock<Mutex<Trace>> = LazyLock::new(|| Mutex::new(Trace::default()));

fn output_dir() -> Option<&'static PathBuf> {
    static DIR: OnceLock<Option<PathBuf>> = OnceLock::new();
    DIR.get_or_init(|| {
        let epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../.cache/diagnostics")
            .join(format!("heap-{}-{epoch}", std::process::id()));
        std::fs::create_dir_all(&dir).ok()?;
        eprintln!("[heap trace] observations: {}", dir.display());
        Some(dir)
    })
    .as_ref()
}

fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    crate::custom_tracks::assets_available()
        && *ON.get_or_init(|| {
            std::env::var("SSX3_HEAP_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
        })
}

fn physical(at: u32) -> Option<u32> {
    let at = match at >> 28 {
        0 => at,
        2 | 3 => at & 0x0fff_ffff,
        8..=11 => at & 0x1fff_ffff,
        _ => return None,
    };
    (at < 0x0200_0000).then_some(at)
}

fn word(c: &Cpu, at: u32) -> Option<u32> {
    let at = physical(at)? as usize;
    Some(u32::from_le_bytes(
        c.bus.ram.get(at..at.checked_add(4)?)?.try_into().ok()?,
    ))
}

fn site(c: &Cpu) -> Site {
    let stack = c.g[SP][0] as u32;
    let mut callers = Vec::new();
    for offset in (0..0x200).step_by(4) {
        let Some(pc) = word(c, stack.wrapping_add(offset)) else {
            break;
        };
        if !(0x0010_0008..0x0043_0000).contains(&pc) || pc & 3 != 0 || callers.contains(&pc) {
            continue;
        }
        let Some(call) = word(c, pc - 8) else {
            continue;
        };
        if call >> 26 == 3 || (call >> 26 == 0 && call & 63 == 9) {
            callers.push(pc);
            if callers.len() == 12 {
                break;
            }
        }
    }
    Site {
        cycles: c.cycles,
        ra: c.g[RA][0] as u32,
        stack,
        args: [A0, A1, A2, A3].map(|r| c.g[r][0] as u32),
        callers,
    }
}

/// The main heap's native arena, including space not yet requested by sbrk.
fn arena(c: &Cpu) -> Option<(u32, u32, u32)> {
    let state = physical(word(c, HEAP)?)?;
    let base = physical(word(c, HEAP + 4)?)?;
    let end = base.checked_add(word(c, HEAP + 12)?)?;
    (state >= 0x10000 && base >= 0x10000 && end <= 0x0200_0000 && base < end)
        .then_some((state, base, end))
}

/// Only inspect the public, steady-state unsorted-list invariant. Native
/// fast bins are singly linked and may legitimately end in zero.
fn invalid_unsorted(c: &Cpu) -> Option<String> {
    let (state, base, end) = arena(c)?;
    if word(c, state)? == 0 {
        return None;
    } // Lazy initialization not done.
    let head = state + 0x38;
    let fd = word(c, head + 8)?;
    let bk = word(c, head + 12)?;
    for (name, ptr, reverse) in [("fd", fd, 12), ("bk", bk, 8)] {
        let valid =
            physical(ptr).is_some_and(|p| p == head || (base..end.saturating_sub(16)).contains(&p));
        if !valid {
            return Some(format!(
                "unsorted {name}={ptr:08x} outside arena {base:08x}..{end:08x}; head={head:08x}"
            ));
        }
        if word(c, ptr.wrapping_add(reverse)).and_then(physical) != Some(head) {
            return Some(format!("unsorted {name}={ptr:08x} has invalid reciprocal link; head={head:08x}, fd={fd:08x}, bk={bk:08x}"));
        }
    }
    None
}

impl Trace {
    fn check(&mut self, c: &Cpu) -> Option<String> {
        if arena(c).is_some() {
            self.saw_arena = true;
            invalid_unsorted(c)
        } else if self.saw_arena {
            Some("previously initialized main heap header is no longer valid".to_owned())
        } else {
            None
        }
    }

    fn remember(&mut self, line: String) {
        if !self.journal_opened {
            self.journal_opened = true;
            self.journal = output_dir().and_then(|dir| File::create(dir.join("events-0.log")).ok());
        }
        if self.journal.is_some() && self.journal_bytes + line.len() + 1 > JOURNAL_LIMIT {
            self.journal_index ^= 1;
            self.journal_bytes = 0;
            self.journal = output_dir().and_then(|dir| {
                File::create(dir.join(format!("events-{}.log", self.journal_index))).ok()
            });
        }
        // Write directly to File, without a userspace buffered tail. A hang
        // inside malloc/consolidation cannot return to the post-call check.
        if let Some(file) = self.journal.as_mut() {
            if writeln!(file, "{line}").is_err() {
                self.journal = None;
            }
            self.journal_bytes += line.len() + 1;
        }
        if self.recent.len() == RECENT_LIMIT {
            self.recent.pop_front();
        }
        self.recent.push_back(line);
    }

    fn report(&mut self, c: &Cpu, reason: String, operation: &Site, ptr: Option<u32>) {
        if self.reported {
            return;
        }
        self.reported = true;
        let owner = ptr.and_then(physical).and_then(|p| self.owners.get(&p));
        let mut body =
            format!("reason={reason}\noperation={operation:x?}\nlast_known_owner={owner:x?}\n");
        // Other entry points (notably realloc) can bypass this journal.
        // Owner history is a clue; only the structural check diagnoses a fault.
        body.push_str("owner_history_is_partial=true\nrecent:\n");
        for line in &self.recent {
            body.push_str(line);
            body.push('\n');
        }
        body.push_str(&format!("known_owners={:x?}\n", self.owners));
        body.push_str(&format!("registers={:x?}\n", c.g));
        if let Some((state, base, end)) = arena(c) {
            body.push_str(&format!(
                "heap={HEAP:08x} state={state:08x} arena={base:08x}..{end:08x}\n"
            ));
        }
        if let Some(dir) = output_dir() {
            let report = std::fs::write(dir.join("first-invalid-state.txt"), body);
            let ram = std::fs::write(dir.join("first-invalid-ram.bin"), &c.bus.ram);
            eprintln!(
                "[heap trace] {reason}; capture={} state={report:?} ram={ram:?}",
                dir.display()
            );
        } else {
            eprintln!("[heap trace] {reason}; capture directory unavailable");
        }
    }
}

/// Begin an ordinary memory operation while its original arguments and
/// caller stack are intact. No guest state, calls, cycles or decisions change.
pub fn begin(c: &Cpu, operation: &str) -> Option<Site> {
    if !enabled() {
        return None;
    }
    let mut trace = TRACE.lock().unwrap_or_else(|p| p.into_inner());
    if trace.reported {
        return None;
    }
    let at = site(c);
    trace.remember(format!("{operation}-begin {at:x?}"));
    if let Some(reason) = trace.check(c) {
        trace.report(c, reason, &at, None);
    }
    Some(at)
}

pub fn allocated(c: &Cpu, at: Option<&Site>, ptr: u32, pooled: bool) {
    let Some(at) = at else { return };
    let mut trace = TRACE.lock().unwrap_or_else(|p| p.into_inner());
    if trace.reported {
        return;
    }
    trace.remember(format!(
        "alloc-return cycles={:x} ptr={ptr:08x} pooled={pooled} begin={:x}",
        c.cycles, at.cycles
    ));
    if ptr != 0 && !pooled {
        if let Some(ptr) = physical(ptr) {
            if trace.owners.len() == OWNER_LIMIT && !trace.owners.contains_key(&ptr) {
                trace.owners.clear(); // Keep observation memory bounded.
            }
            let previous_lifetime = trace
                .owners
                .get(&ptr)
                .map(|owner| (owner.allocation.clone(), owner.last_free.clone()));
            trace.owners.insert(
                ptr,
                Owner {
                    allocation: at.clone(),
                    last_free: None,
                    previous_lifetime,
                },
            );
        }
    }
    if let Some(reason) = trace.check(c) {
        trace.report(c, reason, at, Some(ptr));
    }
}

pub fn freeing(c: &Cpu, at: Option<&Site>, ptr: u32) {
    let Some(at) = at else { return };
    let mut trace = TRACE.lock().unwrap_or_else(|p| p.into_inner());
    if trace.reported {
        return;
    }
    trace.remember(format!("heap-free ptr={ptr:08x} at={at:x?}"));
    if let Some((_, base, end)) = arena(c) {
        if !physical(ptr).is_some_and(|p| p >= base + 8 && p < end) {
            trace.report(
                c,
                format!("free pointer {ptr:08x} outside main heap arena"),
                at,
                Some(ptr),
            );
        }
    }
    if let Some(ptr) = physical(ptr) {
        if let Some(owner) = trace.owners.get_mut(&ptr) {
            owner.last_free = Some(at.clone());
        }
    }
}

pub fn freed(c: &Cpu, at: Option<&Site>, ptr: u32) {
    let Some(at) = at else { return };
    let mut trace = TRACE.lock().unwrap_or_else(|p| p.into_inner());
    if trace.reported {
        return;
    }
    trace.remember(format!(
        "heap-free-return cycles={:x} ptr={ptr:08x}",
        c.cycles
    ));
    if let Some(reason) = trace.check(c) {
        trace.report(c, reason, at, Some(ptr));
    }
}
