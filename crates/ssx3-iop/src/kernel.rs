//! Native IOP kernel: threads, scheduling, semaphores, event flags, alarms,
//! memory, module loading.
//!
//! Semantics follow Sony's IOP kernel as modelled by ps2-web-recomp
//! `runtime/src/iop/kernel.cpp` / `libs.cpp` (MIT, Copyright (c) 2026 Samir
//! Nuri): strict priorities (lower number runs first), FIFO within a
//! priority, `i*` variants usable from interrupt context.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};

use crate::irx::{self, layout};
use crate::{Iop, THREAD_EXIT, UNWIND};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Run,
    Ready,
    Wait,
    Dormant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wait {
    None,
    Sleep,
    Delay,
    Sema(i32),
    Event(i32),
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Ctx {
    pub r: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    pub pc: u32,
}

#[derive(Debug)]
pub struct Thread {
    pub id: i32,
    pub status: Status,
    pub prio: i32,
    pub init_prio: i32,
    pub entry: u32,
    pub stack: u32,
    pub stack_size: u32,
    pub gp: u32,
    pub attr: u32,
    pub option: u32,
    pub wakeups: i32,
    pub wait: Wait,
    pub ctx: Ctx,
    pub order: u64,
    pub intr_suspended: bool,
    /// `sceSifExecRequest` frames: (server data, return address).
    pub exec_stack: Vec<(u32, u32)>,
    pub ev_bits: u32,
    pub ev_mode: u32,
    pub ev_result: u32,
    /// Set for a module's start thread: module id.
    pub module_start: Option<i32>,
    pub cycles: u64,
    /// Queue served by `sceSifRpcLoop` in this thread.
    pub rpc_qd: u32,
}

pub struct Sema {
    pub count: i32,
    pub max: i32,
    pub attr: u32,
    pub option: u32,
    pub waiters: VecDeque<i32>,
}

pub struct EventFlag {
    pub bits: u32,
    pub attr: u32,
    pub option: u32,
    pub waiters: VecDeque<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Timed {
    DelayWake(i32),
    Alarm(u64),
    DmaDone(u32),
    SifDmaCallback(u32, u32),
    VBlankEnd,
}

/// `RegisterVblankHandler` entry (vblank.irx).
#[derive(Clone, Copy, Debug)]
pub struct VblankHandler {
    /// 0: start of VBlank, 1: end.
    pub edge: u32,
    pub prio: i32,
    pub handler: u32,
    pub arg: u32,
}

pub struct Module {
    pub id: i32,
    pub name: String,
    pub base: u32,
    pub size: u32,
    pub entry: u32,
    pub gp: u32,
    /// Start result once the start thread exited.
    pub result: Option<u32>,
}

#[derive(Clone, Copy)]
pub struct IntrHandler {
    pub handler: u32,
    pub arg: u32,
    pub mode: u32,
}

pub struct Kernel {
    pub threads: BTreeMap<i32, Thread>,
    pub current: i32,
    next_thread: i32,
    order: u64,
    pub switch_pending: bool,
    pub intr_suspended: bool,
    pub semas: BTreeMap<i32, Sema>,
    pub events: BTreeMap<i32, EventFlag>,
    next_object: i32,
    timed: BinaryHeap<Reverse<(u64, u64, Timed)>>,
    seq: u64,
    pub alarms: BTreeMap<u64, (u32, u32)>,
    next_alarm: u64,
    /// Allocations: address → size.
    pub used: BTreeMap<u32, u32>,
    pub intr: BTreeMap<u32, IntrHandler>,
    pub modules: BTreeMap<i32, Module>,
    next_module: i32,
    pub libraries: BTreeMap<String, u32>,
    pub tty: String,
    pub missing: BTreeMap<(usize, u32), u64>,
    pub vblank: Vec<VblankHandler>,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            threads: BTreeMap::new(),
            current: 0,
            next_thread: 1,
            order: 0,
            switch_pending: false,
            intr_suspended: false,
            semas: BTreeMap::new(),
            events: BTreeMap::new(),
            next_object: 1,
            timed: BinaryHeap::new(),
            seq: 0,
            alarms: BTreeMap::new(),
            next_alarm: 1,
            used: BTreeMap::new(),
            intr: BTreeMap::new(),
            modules: BTreeMap::new(),
            next_module: 1,
            libraries: BTreeMap::new(),
            tty: String::new(),
            missing: BTreeMap::new(),
            vblank: Vec::new(),
        }
    }

    pub fn schedule(&mut self, at: u64, ev: Timed) {
        self.seq += 1;
        self.timed.push(Reverse((at, self.seq, ev)));
    }
    pub fn next_timed(&self) -> u64 {
        self.timed.peek().map(|r| r.0 .0).unwrap_or(u64::MAX)
    }
    pub fn pop_due(&mut self, now: u64) -> Option<Timed> {
        if self.timed.peek().is_some_and(|r| r.0 .0 <= now) {
            self.timed.pop().map(|r| r.0 .2)
        } else {
            None
        }
    }

    pub fn new_object_id(&mut self) -> i32 {
        let id = self.next_object;
        self.next_object += 1;
        id
    }

    // ---------------------------------------------------------- memory

    /// `mode`: 0 = first fit from the bottom, 1 = from the top, 2 = at `at`.
    pub fn alloc(&mut self, size: u32, mode: u32, at: u32) -> u32 {
        let size = ((size + 0xFF) & !0xFF).max(0x100);
        // Free gaps inside the heap, bottom to top.
        let mut gaps = Vec::new();
        let mut lo = layout::HEAP_START;
        for (&a, &s) in &self.used {
            if a > lo {
                gaps.push((lo, a));
            }
            lo = lo.max(a + s);
        }
        if layout::HEAP_END > lo {
            gaps.push((lo, layout::HEAP_END));
        }
        let chosen = match mode {
            2 => {
                let a = (at & 0x1F_FFFF) & !0xFF;
                gaps.iter().any(|&(l, h)| a >= l && a + size <= h).then_some(a)
            }
            1 => gaps.iter().rev().find(|&&(l, h)| h - l >= size).map(|&(_, h)| h - size),
            _ => gaps.iter().find(|&&(l, h)| h - l >= size).map(|&(l, _)| l),
        };
        match chosen {
            Some(a) => {
                self.used.insert(a, size);
                a
            }
            None => {
                eprintln!("[iop] out of memory allocating {size:#x}");
                0
            }
        }
    }
    pub fn free(&mut self, a: u32) {
        self.used.remove(&(a & 0x1F_FFFF));
    }
    pub fn free_total(&self) -> u32 {
        (layout::HEAP_END - layout::HEAP_START) - self.used.values().sum::<u32>()
    }
    pub fn free_max(&self) -> u32 {
        let mut best = 0;
        let mut start = layout::HEAP_START;
        for (&a, &s) in &self.used {
            if a > start {
                best = best.max(a - start);
            }
            start = start.max(a + s);
        }
        best.max(layout::HEAP_END.saturating_sub(start))
    }

    // ---------------------------------------------------------- threads

    pub fn create_thread(&mut self, entry: u32, stack_size: u32, prio: i32, gp: u32, attr: u32, option: u32) -> i32 {
        let stack_size = (stack_size + 0xFF) & !0xFF;
        let stack = self.alloc(stack_size, 1, 0);
        let id = self.next_thread;
        self.next_thread += 1;
        self.threads.insert(
            id,
            Thread {
                id,
                status: Status::Dormant,
                prio,
                init_prio: prio,
                entry,
                stack,
                stack_size,
                gp,
                attr,
                option,
                wakeups: 0,
                wait: Wait::None,
                ctx: Ctx::default(),
                order: 0,
                intr_suspended: false,
                exec_stack: Vec::new(),
                ev_bits: 0,
                ev_mode: 0,
                ev_result: 0,
                module_start: None,
                cycles: 0,
                rpc_qd: 0,
            },
        );
        id
    }

    pub fn start_thread(&mut self, id: i32, a0: u32, a1: u32) -> bool {
        let Some(t) = self.threads.get_mut(&id) else { return false };
        t.ctx = Ctx::default();
        t.ctx.pc = t.entry;
        t.ctx.r[4] = a0;
        t.ctx.r[5] = a1;
        t.ctx.r[28] = t.gp;
        t.ctx.r[29] = t.stack + t.stack_size - 0x10;
        t.ctx.r[31] = THREAD_EXIT;
        t.prio = t.init_prio;
        t.wakeups = 0;
        t.wait = Wait::None;
        t.intr_suspended = false;
        self.make_ready(id);
        true
    }

    pub fn make_ready(&mut self, id: i32) {
        self.order += 1;
        let order = self.order;
        if let Some(t) = self.threads.get_mut(&id) {
            t.status = Status::Ready;
            t.order = order;
        }
        self.switch_pending = true;
    }

    /// Wake a waiting thread with `result` in its v0.
    pub fn wake(&mut self, id: i32, result: u32) {
        let Some(t) = self.threads.get_mut(&id) else { return };
        if t.status != Status::Wait {
            return;
        }
        t.wait = Wait::None;
        t.ctx.r[2] = result;
        self.make_ready(id);
    }

    fn best_ready(&self) -> Option<i32> {
        self.threads
            .values()
            .filter(|t| matches!(t.status, Status::Ready | Status::Run))
            .min_by_key(|t| (t.prio, t.order))
            .map(|t| t.id)
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

fn save_live(c: &mut Iop) -> Ctx {
    Ctx { r: c.r, hi: c.hi, lo: c.lo, pc: c.pc & !UNWIND }
}

fn load_live(c: &mut Iop, ctx: &Ctx) {
    c.r = ctx.r;
    c.hi = ctx.hi;
    c.lo = ctx.lo;
    c.pc = ctx.pc;
}

/// Pick the thread that should run and switch to it.
pub fn reschedule(c: &mut Iop) {
    let cur = c.k.current;
    let cur_running = c.k.threads.get(&cur).map(|t| t.status == Status::Run).unwrap_or(false);
    if !c.k.switch_pending && (cur_running || cur == 0) {
        if cur == 0 {
            if let Some(b) = c.k.best_ready() {
                switch_to(c, b);
            }
        }
        return;
    }
    c.k.switch_pending = false;
    let best = c.k.best_ready();
    if best == Some(cur) && cur_running {
        return;
    }
    if let Some(t) = c.k.threads.get_mut(&cur) {
        t.intr_suspended = c.k.intr_suspended;
    }
    if cur_running {
        let ctx = save_live(c);
        let t = c.k.threads.get_mut(&cur).unwrap();
        t.ctx = ctx;
        t.status = Status::Ready;
    }
    match best {
        Some(b) => switch_to(c, b),
        None => {
            c.k.current = 0;
            c.k.intr_suspended = false;
        }
    }
}

fn switch_to(c: &mut Iop, id: i32) {
    let t = c.k.threads.get_mut(&id).unwrap();
    t.status = Status::Run;
    let ctx = t.ctx;
    let susp = t.intr_suspended;
    c.k.current = id;
    c.k.intr_suspended = susp;
    load_live(c, &ctx);
    if c.trace {
        eprintln!("[iop] switch to thread {id} pc {:#x}", ctx.pc);
    }
}

/// Block the current thread inside a native call. Its context is saved with
/// `pc = ra` so it resumes after the call; `v0` is filled in by `wake`.
/// Returns false (and does nothing) outside thread context.
pub fn block(c: &mut Iop, wait: Wait) -> bool {
    let cur = c.k.current;
    if c.in_handler > 0 || cur == 0 {
        eprintln!("[iop] blocking call {wait:?} outside a thread (pc {:#x})", c.r[31]);
        return false;
    }
    let mut ctx = save_live(c);
    ctx.pc = c.r[31];
    let susp = c.k.intr_suspended;
    let t = c.k.threads.get_mut(&cur).unwrap();
    t.ctx = ctx;
    t.status = Status::Wait;
    t.wait = wait;
    t.intr_suspended = susp;
    c.k.switch_pending = true;
    c.pc = c.r[31] | UNWIND;
    c.yield_now = true;
    true
}

/// The current thread's entry function returned (or ExitThread).
pub fn exit_thread(c: &mut Iop) {
    let cur = c.k.current;
    let v0 = c.r[2];
    if let Some(t) = c.k.threads.get_mut(&cur) {
        t.status = Status::Dormant;
        t.ctx.r[2] = v0;
        if let Some(m) = t.module_start.take() {
            if let Some(module) = c.k.modules.get_mut(&m) {
                module.result = Some(v0);
                eprintln!("[iop] module {} started, result {}", module.name, v0 as i32);
            }
        }
    }
    c.k.switch_pending = true;
    c.k.current = 0;
    c.k.intr_suspended = false;
    c.yield_now = true;
    c.pc = THREAD_EXIT;
    if c.in_handler > 0 {
        c.fatal(THREAD_EXIT, "thread exit inside a handler");
    }
}

pub fn timed_event(c: &mut Iop, ev: Timed) {
    match ev {
        Timed::DelayWake(id) => {
            if c.k.threads.get(&id).is_some_and(|t| t.wait == Wait::Delay) {
                c.k.wake(id, 0);
            }
        }
        Timed::Alarm(key) => {
            if let Some((handler, arg)) = c.k.alarms.remove(&key) {
                let again = c.call_guest(handler, &[arg]);
                if again != 0 {
                    let k = c.k.next_alarm;
                    c.k.next_alarm += 1;
                    c.k.alarms.insert(k, (handler, arg));
                    let at = c.cycles + again as u64;
                    c.k.schedule(at, Timed::Alarm(k));
                }
            }
        }
        Timed::DmaDone(ch) => crate::hw::dma_done(c, ch),
        Timed::SifDmaCallback(f, arg) => {
            c.call_guest(f, &[arg]);
        }
        Timed::VBlankEnd => run_vblank_handlers(c, 1),
    }
}

/// Deliver a pending VBlank start (signalled by the host) to the vblank.irx
/// handlers and schedule the matching VBlank end.
pub fn vblank_start(c: &mut Iop) {
    if c.vblank_pending == 0 || c.in_handler > 0 {
        return;
    }
    c.vblank_pending = 0;
    c.vblanks += 1;
    run_vblank_handlers(c, 0);
    let at = c.cycles + crate::VBLANK_CYCLES;
    c.k.schedule(at, Timed::VBlankEnd);
}

/// Run the handlers registered for `edge` in priority order; a handler that
/// returns 0 is removed (vblank.irx semantics).
fn run_vblank_handlers(c: &mut Iop, edge: u32) {
    let hs: Vec<VblankHandler> = c.k.vblank.iter().copied().filter(|h| h.edge == edge).collect();
    for h in hs {
        if c.halted.is_some() {
            return;
        }
        let keep = c.call_guest(h.handler, &[h.arg]);
        if keep == 0 {
            c.k.vblank.retain(|x| !(x.edge == h.edge && x.handler == h.handler && x.arg == h.arg));
        }
    }
}

pub fn set_alarm(c: &mut Iop, delay: u64, handler: u32, arg: u32) {
    let k = c.k.next_alarm;
    c.k.next_alarm += 1;
    c.k.alarms.insert(k, (handler, arg));
    let at = c.cycles + delay.max(1);
    c.k.schedule(at, Timed::Alarm(k));
}

/// Load a recompiled module from its disc image and start it. Returns the
/// module id; the start thread runs on the next [`Iop::run`].
pub fn load_module(c: &mut Iop, bytes: &[u8], path: &str) -> Result<i32, String> {
    let name = irx::module_name_of(path);
    let base = layout::base_of(&name).ok_or_else(|| format!("{name} has no fixed load address"))?;
    let gen = crate::generated().ok_or("no generated IOP code registered")?;
    let info = gen.modules.iter().find(|m| m.name.eq_ignore_ascii_case(&name)).ok_or_else(|| format!("{name} was not recompiled"))?;
    let m = irx::load(bytes, base)?;
    if m.text_hash() != info.text_hash || info.base != base {
        return Err(format!("{name}: disc image does not match the recompiled code (rerun recomp-iop)"));
    }
    let size = m.image.len() as u32;
    let start = base as usize;
    c.ram[start..start + m.image.len()].copy_from_slice(&m.image);
    let id = c.k.next_module;
    c.k.next_module += 1;
    // argv: [path, NULL] in a small allocation.
    let argv = c.k.alloc(0x100, 0, 0);
    let mut p = path.as_bytes().to_vec();
    p.truncate(0xF0);
    p.push(0);
    c.write_bytes(argv + 8, &p);
    c.set_ram32(argv, argv + 8);
    c.set_ram32(argv + 4, 0);
    let tid = c.k.create_thread(m.entry, 0x4000, 8, m.gp, 0x0200_0000, 0);
    c.k.threads.get_mut(&tid).unwrap().module_start = Some(id);
    c.k.start_thread(tid, 1, argv);
    eprintln!("[iop] module {} ({}) at {base:#x}, {size:#x} bytes, entry {:#x}", id, m.name, m.entry);
    c.k.modules.insert(id, Module { id, name, base, size, entry: m.entry, gp: m.gp, result: None });
    Ok(id)
}

/// Write one character string to the IOP TTY (Kprintf / printf).
pub fn tty(c: &mut Iop, s: &str) {
    for ch in s.chars() {
        if ch == '\n' {
            eprintln!("[iop-tty] {}", c.k.tty);
            c.k.tty.clear();
        } else if ch != '\r' {
            c.k.tty.push(ch);
        }
    }
}

/// `TerminateThread`: make another thread dormant, dropping its waits.
pub fn terminate(c: &mut Iop, id: i32) {
    for s in c.k.semas.values_mut() {
        s.waiters.retain(|&t| t != id);
    }
    for e in c.k.events.values_mut() {
        e.waiters.retain(|&t| t != id);
    }
    if let Some(t) = c.k.threads.get_mut(&id) {
        t.status = Status::Dormant;
        t.wait = Wait::None;
        t.exec_stack.clear();
    }
    c.k.switch_pending = true;
}
