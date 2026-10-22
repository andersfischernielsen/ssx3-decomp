//! Runtime A/B checks of installed replacements (`SSX3_SHADOW=addr,addr`,
//! or `all`): each call of a listed function runs the hand-written form and
//! the generated body from the same state and reports the first
//! differences (registers, cycles, the RAM pages either wrote, the
//! scratchpad, VU0). The generated run's results stay, so the game goes on
//! exactly as generated and one bad call does not hide the next. Finds the
//! branches a port's differential test never reached, in real play.
//!
//! - `SSX3_SHADOW_FIRST=k` (default 20) and `SSX3_SHADOW_EVERY=n`
//!   (default 1): check the first k calls of each function and every n-th
//!   after that (the others run the replacement). Each check copies guest
//!   RAM, so long lists want a large n.
//! - Side effects outside guest memory (hardware registers, syscalls)
//!   happen twice; it is a diagnostic mode.
//! - VU0 status/MAC/clip flags cannot be restored through the `Vu0`
//!   interface: they are compared at the end but a difference there may
//!   come from the other run's flags.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

struct Entry {
    name: &'static str,
    native: GuestFn,
    generated: GuestFn,
    calls: u64,
    bad: u64,
}

static TABLE: OnceLock<Mutex<HashMap<u32, Entry>>> = OnceLock::new();
static INSIDE: AtomicBool = AtomicBool::new(false);

fn every() -> u64 {
    static E: OnceLock<u64> = OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_SHADOW_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(1).max(1))
}

fn first() -> u64 {
    static F: OnceLock<u64> = OnceLock::new();
    *F.get_or_init(|| std::env::var("SSX3_SHADOW_FIRST").ok().and_then(|v| v.parse().ok()).unwrap_or(20))
}

/// The functions to shadow (`SSX3_SHADOW`), if any: `None` when the
/// variable is unset.
pub fn wanted() -> Option<Option<Vec<u32>>> {
    let v = std::env::var("SSX3_SHADOW").ok()?;
    if v == "all" {
        return Some(None);
    }
    Some(Some(v.split(',').filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok()).collect()))
}

/// Register `addr` for shadowing; returns the function to install in its
/// slot (the shadow entry), or `None` when its generated body is unknown.
pub fn wrap(addr: u32, name: &'static str, native: GuestFn) -> Option<GuestFn> {
    let generated = ssx3_ee::generated_body(addr)?;
    TABLE.get_or_init(Default::default).lock().unwrap().insert(addr, Entry { name, native, generated, calls: 0, bad: 0 });
    Some(entry)
}

/// The shadow entry (the slot's function): `c.pc` is the function's start.
fn entry(c: &mut Cpu) {
    let addr = c.pc;
    let (native, generated, name, check) = {
        let mut t = TABLE.get().unwrap().lock().unwrap();
        let e = t.get_mut(&addr).expect("shadowed function");
        e.calls += 1;
        let check = !INSIDE.load(Ordering::Relaxed) && (e.calls <= first() || e.calls % every() == 0);
        (e.native, e.generated, e.name, check)
    };
    if !check {
        return native(c);
    }
    INSIDE.store(true, Ordering::Relaxed);
    let args = [A0, A1, A2, A3].map(|r| c.g[r][0] as u32);
    let diffs = run_both(c, native, generated);
    INSIDE.store(false, Ordering::Relaxed);
    if !diffs.is_empty() {
        let mut t = TABLE.get().unwrap().lock().unwrap();
        let e = t.get_mut(&addr).unwrap();
        e.bad += 1;
        if e.bad <= 5 {
            eprintln!(
                "[shadow] {name} ({addr:#x}) call {} (a0-a3 {args:x?}): {} differences: {}",
                e.calls,
                diffs.len(),
                diffs.iter().take(16).cloned().collect::<Vec<_>>().join("; ")
            );
        }
    }
}

#[derive(Clone)]
struct Regs {
    g: [[u64; 2]; 32],
    hi: [u64; 2],
    lo: [u64; 2],
    sa: u32,
    f: [u32; 32],
    facc: u32,
    fcr31: u32,
    pc: u32,
    cycles: u64,
    vf: Vec<[u32; 4]>,
    acc: [u32; 4],
    /// VI0-15, then status, MAC, clip, R, I, Q.
    ctrl: Vec<u32>,
}

const CTRL: [u32; 22] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22];

fn regs(c: &mut Cpu) -> Regs {
    let vf = (0..32).map(|r| c.vu0().vf(r)).collect();
    let acc = c.vu0().acc();
    let ctrl = CTRL.iter().map(|&r| c.vu0().read_ctrl(r)).collect();
    Regs { g: c.g, hi: c.hi, lo: c.lo, sa: c.sa, f: c.f, facc: c.facc, fcr31: c.fcr31, pc: c.pc, cycles: c.cycles, vf, acc, ctrl }
}

fn set_regs(c: &mut Cpu, r: &Regs) {
    (c.g, c.hi, c.lo, c.sa, c.f, c.facc, c.fcr31, c.pc, c.cycles) = (r.g, r.hi, r.lo, r.sa, r.f, r.facc, r.fcr31, r.pc, r.cycles);
    for (i, v) in r.vf.iter().enumerate() {
        c.vu0().set_vf(i, *v);
    }
    c.vu0().set_acc(r.acc);
    // (VI, R, I, Q; the flag registers can't be written back.)
    for (k, &reg) in CTRL.iter().enumerate() {
        if !(16..=18).contains(&reg) {
            c.vu0().write_ctrl(reg, r.ctrl[k]);
        }
    }
}

fn run_both(c: &mut Cpu, native: GuestFn, generated: GuestFn) -> Vec<String> {
    let pages = c.bus.ram.len() / 4096;
    let mut pending = vec![0u64; pages.div_ceil(64)];
    ssx3_ee_rt::bus::take_written(&mut pending);
    let pre_ram = c.bus.ram.clone();
    let pre_spr = c.bus.spr.clone();
    let pre = regs(c);
    let next = c.next_event;
    c.next_event = u64::MAX;
    let sp = c.g[SP][0] as u32;
    native(c);
    let after_native = regs(c);
    let mut wn = vec![0u64; pending.len()];
    ssx3_ee_rt::bus::take_written(&mut wn);
    let bits = |b: &[u64]| -> Vec<usize> { (0..pages).filter(|&p| b[p >> 6] >> (p & 63) & 1 != 0).collect() };
    let native_pages: Vec<(usize, Vec<u8>)> = bits(&wn).into_iter().map(|p| (p, c.bus.ram[p * 4096..(p + 1) * 4096].to_vec())).collect();
    let native_spr = c.bus.spr.clone();
    for (p, _) in &native_pages {
        c.bus.ram[p * 4096..(p + 1) * 4096].copy_from_slice(&pre_ram[p * 4096..(p + 1) * 4096]);
    }
    c.bus.spr.copy_from_slice(&pre_spr);
    set_regs(c, &pre);
    c.next_event = u64::MAX;
    generated(c);
    c.next_event = if c.next_event == u64::MAX { next } else { c.next_event.min(next) };
    let after_gen = regs(c);
    let mut wg = vec![0u64; pending.len()];
    ssx3_ee_rt::bus::take_written(&mut wg);
    let mut bad = Vec::new();
    // Registers.
    let (a, b) = (&after_native, &after_gen);
    for r in 0..32 {
        if a.g[r] != b.g[r] {
            bad.push(format!("gpr{r}: native {:x?} generated {:x?}", a.g[r], b.g[r]));
        }
        if a.f[r] != b.f[r] {
            bad.push(format!("f{r}: native {:#x} generated {:#x}", a.f[r], b.f[r]));
        }
        if a.vf[r] != b.vf[r] {
            bad.push(format!("vf{r}: native {:x?} generated {:x?}", a.vf[r], b.vf[r]));
        }
    }
    for (what, x, y) in [("hi", a.hi, b.hi), ("lo", a.lo, b.lo)] {
        if x != y {
            bad.push(format!("{what}: native {x:x?} generated {y:x?}"));
        }
    }
    if (a.facc, a.fcr31, a.sa) != (b.facc, b.fcr31, b.sa) {
        bad.push(format!("facc/fcr31/sa: native {:x?} generated {:x?}", (a.facc, a.fcr31, a.sa), (b.facc, b.fcr31, b.sa)));
    }
    if a.pc != b.pc || a.cycles != b.cycles {
        bad.push(format!("pc/cycles: native {:#x}/{} generated {:#x}/{}", a.pc, a.cycles - pre.cycles, b.pc, b.cycles - pre.cycles));
    }
    if a.acc != b.acc {
        bad.push(format!("vu0 acc: native {:x?} generated {:x?}", a.acc, b.acc));
    }
    for (k, &reg) in CTRL.iter().enumerate() {
        if a.ctrl[k] != b.ctrl[k] {
            bad.push(format!("vu0 ctrl {reg}: native {:#x} generated {:#x}", a.ctrl[k], b.ctrl[k]));
        }
    }
    // Memory: every page either run wrote (stack scratch below sp aside).
    let mut all = bits(&wn);
    all.extend(bits(&wg));
    all.sort_unstable();
    all.dedup();
    for p in all {
        let nat = native_pages.iter().find(|x| x.0 == p).map_or(&pre_ram[p * 4096..(p + 1) * 4096], |x| &x.1[..]);
        for o in (0..4096).step_by(4) {
            let addr = (p * 4096 + o) as u32;
            let x = u32::from_le_bytes(nat[o..o + 4].try_into().unwrap());
            let y = u32::from_le_bytes(c.bus.ram[p * 4096 + o..p * 4096 + o + 4].try_into().unwrap());
            if x != y && !(sp.wrapping_sub(0x4000)..sp).contains(&addr) {
                bad.push(format!("{addr:#x}: native {x:#x} generated {y:#x}"));
            }
        }
    }
    for o in (0..c.bus.spr.len()).step_by(4) {
        let x = u32::from_le_bytes(native_spr[o..o + 4].try_into().unwrap());
        let y = u32::from_le_bytes(c.bus.spr[o..o + 4].try_into().unwrap());
        if x != y {
            bad.push(format!("spr {:#x}: native {x:#x} generated {y:#x}", 0x7000_0000 + o));
        }
    }
    for b in [&pending, &wn, &wg] {
        for p in bits(b) {
            ssx3_ee_rt::bus::mark_written(p * 4096);
        }
    }
    bad
}

/// One line per shadowed function that was called: calls, calls that
/// differed.
pub fn report() -> Vec<String> {
    let Some(t) = TABLE.get() else { return Vec::new() };
    let t = t.lock().unwrap();
    let mut v: Vec<_> = t.iter().filter(|(_, e)| e.calls > 0).map(|(a, e)| (e.bad, format!("{} ({a:#x}): {} calls, {} differ", e.name, e.calls, e.bad))).collect();
    v.sort_by(|x, y| y.0.cmp(&x.0));
    v.into_iter().map(|x| x.1).collect()
}
