//! High-level emulation of the EE kernel (the PS2 BIOS syscall layer).
//!
//! The kernel is firmware, not game code, so it is provided natively. Design
//! and much of the behaviour are ported from ps2-web-recomp's
//! `runtime/src/kernel.cpp` (MIT, Copyright (c) 2026 Samir Nuri):
//!
//! * Threads are register contexts. A switch saves the live CPU state into the
//!   current thread, loads the next one, and sets [`UNWIND`] in `pc` so every
//!   generated frame returns to the trampoline, which then re-enters the
//!   recompiled code at the new thread's `pc`.
//! * Interrupt and alarm handlers run nested via [`run_guest`], on a kernel
//!   stack, returning through the magic address [`HANDLER_RETURN`].
//!
//! Syscall numbers follow this game's libkernel (see
//! `docs/investigation/boot-path.md`): SetAlarm/ReleaseAlarm are also reachable
//! as 0xFC/0xFD, and 0x79/0x7A are sceSifSetReg/GetReg.

use std::collections::{BTreeMap, VecDeque};

use crate::cpu::{sx, Cpu, UNWIND};

/// Fake return address for nested handler calls; never translated code.
pub const HANDLER_RETURN: u32 = 0x0000_1000;
/// Where the trampoline parks when no thread is runnable.
pub const IDLE: u32 = 0x0000_1010;
/// Return address given to new threads (falling off the entry = ExitThread).
pub const THREAD_EXIT: u32 = 0x0000_1020;
const KERNEL_STACK_TOP: u32 = 0x0008_0000;
const SYSCALL_TABLE: u32 = 0x0000_1800;
const CONTEXT_RESERVE: u32 = 0x2A0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Run = 1,
    Ready = 2,
    Wait = 4,
    Suspend = 8,
    WaitSuspend = 12,
    Dormant = 16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitType {
    None,
    Sleep,
    Sema,
}

/// Saved register state of a thread.
#[derive(Clone, Copy)]
pub struct Context {
    pub g: [[u64; 2]; 32],
    pub hi: [u64; 2],
    pub lo: [u64; 2],
    pub sa: u32,
    pub f: [u32; 32],
    pub facc: u32,
    pub fcr31: u32,
    pub pc: u32,
}

impl Default for Context {
    fn default() -> Self {
        Self { g: [[0; 2]; 32], hi: [0; 2], lo: [0; 2], sa: 0, f: [0; 32], facc: 0, fcr31: 0, pc: 0 }
    }
}

#[derive(Clone)]
pub struct Thread {
    pub id: i32,
    pub status: Status,
    pub priority: i32,
    pub init_priority: i32,
    pub entry: u32,
    pub stack: u32,
    pub stack_size: u32,
    pub gp: u32,
    pub heap: u32,
    pub option: u32,
    pub attr: u32,
    pub wakeups: i32,
    pub suspends: i32,
    pub wait: WaitType,
    pub wait_id: i32,
    pub order: u64,
    pub ctx: Context,
}

#[derive(Clone, Default)]
pub struct Sema {
    pub count: i32,
    pub max: i32,
    pub init: i32,
    pub attr: u32,
    pub option: u32,
    pub waiters: VecDeque<i32>,
}

#[derive(Clone, Copy)]
pub struct Handler {
    pub id: i32,
    pub cause: u32,
    pub address: u32,
    pub arg: u32,
    pub gp: u32,
}

#[derive(Clone, Copy)]
pub struct Alarm {
    pub callback: u32,
    pub arg: u32,
    pub gp: u32,
    pub due: u64,
}

pub struct Kernel {
    pub threads: BTreeMap<i32, Thread>,
    pub semas: BTreeMap<i32, Sema>,
    pub intc_handlers: Vec<Handler>,
    pub dmac_handlers: Vec<Handler>,
    pub alarms: BTreeMap<i32, Alarm>,
    pub current: i32,
    next_handler: i32,
    order: u64,
    pub in_handler: bool,
    pub reschedule: bool,
    pub vsync_flag: u32,
    pub vsync_data: u32,
    osd_config: u32,
    pub boot_argc: u32,
    pub boot_argv: u32,
    pub trace_syscalls: bool,
    pub trace_threads: bool,
    pub halted: Option<String>,
    pub syscall_counts: BTreeMap<u32, u64>,
    /// Cycles consumed per thread (profiling).
    pub thread_cycles: BTreeMap<i32, u64>,
    pub last_switch: u64,
    /// Host-thread mode (see `threads.rs`): guest thread id → baton token of
    /// its host thread. Token 0 is the main (frame-driving) host thread.
    pub host_threads: bool,
    pub tokens: BTreeMap<i32, i32>,
    pub next_token: i32,
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

fn lowest_free<V>(m: &BTreeMap<i32, V>, limit: i32) -> i32 {
    (1..limit).find(|id| !m.contains_key(id)).unwrap_or(-1)
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            threads: BTreeMap::new(),
            semas: BTreeMap::new(),
            intc_handlers: Vec::new(),
            dmac_handlers: Vec::new(),
            alarms: BTreeMap::new(),
            current: 0,
            next_handler: 1,
            order: 0,
            in_handler: false,
            reschedule: false,
            vsync_flag: 0,
            vsync_data: 0,
            osd_config: 0,
            boot_argc: 0,
            boot_argv: 0,
            trace_syscalls: std::env::var("SSX3_TRACE_SYSCALLS").is_ok(),
            trace_threads: std::env::var("SSX3_TRACE_THREADS").is_ok(),
            halted: None,
            syscall_counts: BTreeMap::new(),
            thread_cycles: BTreeMap::new(),
            last_switch: 0,
            host_threads: false,
            tokens: BTreeMap::new(),
            next_token: 1,
        }
    }

    fn cur(&mut self) -> Option<&mut Thread> {
        let id = self.current;
        self.threads.get_mut(&id)
    }

    fn thread_id(&self, id: i32) -> i32 {
        if id == 0 {
            self.current
        } else {
            id
        }
    }

    fn make_ready(&mut self, id: i32) {
        self.order += 1;
        let order = self.order;
        if let Some(t) = self.threads.get_mut(&id) {
            t.status = Status::Ready;
            t.order = order;
        }
        self.reschedule = true;
    }

    fn best_ready(&self) -> Option<i32> {
        let mut best: Option<&Thread> = None;
        for t in self.threads.values() {
            if t.status != Status::Ready && t.status != Status::Run {
                continue;
            }
            if best.is_none_or(|b| t.priority < b.priority || (t.priority == b.priority && t.order < b.order)) {
                best = Some(t);
            }
        }
        best.map(|t| t.id)
    }

    fn release(&mut self, id: i32, result: i64) {
        if let Some(t) = self.threads.get_mut(&id) {
            t.wait = WaitType::None;
            t.ctx.g[2][0] = result as u64;
            if t.status == Status::WaitSuspend {
                t.status = Status::Suspend;
                return;
            }
        }
        self.make_ready(id);
    }
}

pub fn save_context(c: &Cpu) -> Context {
    Context { g: c.g, hi: c.hi, lo: c.lo, sa: c.sa, f: c.f, facc: c.facc, fcr31: c.fcr31, pc: c.pc & !UNWIND }
}

pub fn load_context(c: &mut Cpu, x: &Context) {
    c.g = x.g;
    c.hi = x.hi;
    c.lo = x.lo;
    c.sa = x.sa;
    c.f = x.f;
    c.facc = x.facc;
    c.fcr31 = x.fcr31;
    c.pc = x.pc;
}

fn arg(c: &Cpu, i: usize) -> u32 {
    c.g[4 + i][0] as u32
}

fn ret(c: &mut Cpu, v: i64) {
    c.g[2][0] = v as u64;
}

fn switch_to(c: &mut Cpu, next: Option<i32>) {
    let cur = m!(c).kernel.current;
    if Some(cur) == next {
        return;
    }
    let spent = c.cycles - m!(c).kernel.last_switch;
    *m!(c).kernel.thread_cycles.entry(cur).or_default() += spent;
    m!(c).kernel.last_switch = c.cycles;
    let ctx = save_context(c);
    if let Some(t) = m!(c).kernel.threads.get_mut(&cur) {
        if t.status != Status::Dormant {
            t.ctx = ctx;
            if t.status == Status::Run {
                t.status = Status::Ready;
            }
        }
    }
    match next {
        Some(id) => {
            let t = m!(c).kernel.threads.get_mut(&id).unwrap();
            t.status = Status::Run;
            let ctx = t.ctx;
            m!(c).kernel.current = id;
            load_context(c, &ctx);
            if m!(c).kernel.trace_threads {
                eprintln!("[thread] -> {id} at {:#010x}", ctx.pc);
            }
        }
        None => {
            m!(c).kernel.current = 0;
            c.pc = IDLE;
            if m!(c).kernel.trace_threads {
                eprintln!("[thread] -> idle");
            }
        }
    }
    if m!(c).kernel.host_threads {
        // The calling host thread's own token: robust even when `cur` was
        // just removed (ExitDeleteThread) or this is the frame driver.
        let from = crate::threads::my_token();
        let to = match next {
            Some(id) => token_or_spawn(c, id),
            None => 0,
        };
        crate::threads::handoff(from, to);
        return;
    }
    c.pc |= UNWIND;
}

/// Baton token of guest thread `id`'s host thread (0 = main host thread).
pub fn token_of(c: &Cpu, id: i32) -> i32 {
    mr!(c).kernel.tokens.get(&id).copied().unwrap_or(0)
}

fn token_or_spawn(c: &mut Cpu, id: i32) -> i32 {
    if let Some(t) = m!(c).kernel.tokens.get(&id) {
        return *t;
    }
    let t = m!(c).kernel.next_token;
    m!(c).kernel.next_token += 1;
    m!(c).kernel.tokens.insert(id, t);
    crate::threads::spawn(c, t);
    t
}

pub fn reschedule(c: &mut Cpu) {
    m!(c).kernel.reschedule = false;
    if m!(c).kernel.in_handler {
        return;
    }
    let cur = m!(c).kernel.current;
    let best = m!(c).kernel.best_ready();
    if best == Some(cur) && m!(c).kernel.threads.get(&cur).is_some_and(|t| t.status == Status::Run) {
        return;
    }
    switch_to(c, best);
}

fn block_current(c: &mut Cpu, w: WaitType, id: i32) {
    let ctx = save_context(c);
    if let Some(t) = m!(c).kernel.cur() {
        t.status = Status::Wait;
        t.wait = w;
        t.wait_id = id;
        t.ctx = ctx;
    }
    reschedule(c);
}

/// Run guest code at `address` to completion as a nested call (interrupt and
/// alarm handlers, custom syscalls). Returns `$v0`.
pub fn run_guest(c: &mut Cpu, address: u32, a: [u32; 4], gp: u32) -> u32 {
    let saved = save_context(c);
    let saved_depth = c.depth;
    let nested = m!(c).kernel.in_handler;
    m!(c).kernel.in_handler = true;
    let status = c.cop0[12];
    c.cop0[12] = (status & !0x10001) | 2;
    for (i, v) in a.iter().enumerate() {
        c.g[4 + i][0] = sx(*v);
    }
    if gp != 0 {
        c.g[28][0] = sx(gp);
    }
    c.g[29][0] = sx(KERNEL_STACK_TOP - if nested { 0x4000 } else { 0 });
    c.g[31][0] = sx(HANDLER_RETURN);
    c.pc = address;
    c.depth = 0;
    crate::machine::trampoline(c, Some(HANDLER_RETURN));
    let result = c.g[2][0] as u32;
    load_context(c, &saved);
    c.depth = saved_depth;
    c.cop0[12] = status;
    m!(c).kernel.in_handler = nested;
    result
}

fn add_handler(c: &mut Cpu, dmac: bool) {
    let h = Handler {
        id: m!(c).kernel.next_handler,
        cause: arg(c, 0),
        address: arg(c, 1),
        arg: arg(c, 3),
        gp: c.g[28][0] as u32,
    };
    m!(c).kernel.next_handler += 1;
    let next = arg(c, 2) as i32;
    let list = if dmac { &mut m!(c).kernel.dmac_handlers } else { &mut m!(c).kernel.intc_handlers };
    if next == 0 {
        list.insert(0, h);
    } else if next == -1 {
        list.push(h);
    } else {
        let pos = list.iter().position(|x| x.id == next).unwrap_or(list.len());
        list.insert(pos, h);
    }
    ret(c, h.id as i64);
}

fn remove_handler(c: &mut Cpu, dmac: bool) {
    let id = arg(c, 1) as i32;
    let list = if dmac { &mut m!(c).kernel.dmac_handlers } else { &mut m!(c).kernel.intc_handlers };
    list.retain(|h| h.id != id);
    ret(c, 0);
}

pub fn interrupts_enabled(c: &Cpu) -> bool {
    let s = c.cop0[12];
    (s & 0x10001) == 0x10001 && s & 6 == 0
}

/// Deliver pending interrupts and alarms, then reschedule if needed. Called
/// from [`crate::machine::service_events`] outside handlers.
pub fn service(c: &mut Cpu) {
    if m!(c).kernel.in_handler {
        return;
    }
    run_alarms(c);
    if interrupts_enabled(c) {
        for _ in 0..16 {
            let intc = m!(c).hw.intc_pending();
            let dmac = m!(c).hw.dmac_pending();
            if intc == 0 && dmac == 0 {
                break;
            }
            if dmac != 0 {
                let chn = dmac.trailing_zeros();
                m!(c).hw.d_stat &= !(1 << chn);
                let hs: Vec<Handler> =
                    m!(c).kernel.dmac_handlers.iter().filter(|h| h.cause == chn).copied().collect();
                for h in hs {
                    if (run_guest(c, h.address, [chn, h.arg, 0, 0], h.gp) as i32) < 0 {
                        break;
                    }
                }
                if chn == 5 {
                    crate::sif::packet_processed(c);
                }
            } else {
                let cause = intc.trailing_zeros();
                m!(c).hw.i_stat &= !(1 << cause);
                let hs: Vec<Handler> =
                    m!(c).kernel.intc_handlers.iter().filter(|h| h.cause == cause).copied().collect();
                for h in hs {
                    if (run_guest(c, h.address, [cause, h.arg, 0, 0], h.gp) as i32) < 0 {
                        break;
                    }
                }
            }
        }
    }
    if m!(c).kernel.reschedule || (c.pc & !UNWIND) == IDLE {
        if c.no_unwind && !m!(c).kernel.host_threads {
            // At the caller's next check (see `Cpu::no_unwind`).
            m!(c).kernel.reschedule = true;
            return;
        }
        reschedule(c);
    }
}

fn run_alarms(c: &mut Cpu) {
    loop {
        let now = c.cycles;
        let due = m!(c).kernel.alarms.iter().find(|(_, a)| a.due <= now).map(|(id, a)| (*id, *a));
        let Some((id, a)) = due else { break };
        m!(c).kernel.alarms.remove(&id);
        let hsync = (now / crate::machine::HSYNC_CYCLES) as u32;
        run_guest(c, a.callback, [id as u32, hsync, a.arg, 0], a.gp);
        m!(c).kernel.reschedule = true;
    }
}

/// Earliest pending alarm deadline (for the event scheduler).
pub fn next_alarm(c: &Cpu) -> Option<u64> {
    mr!(c).kernel.alarms.values().map(|a| a.due).min()
}

/// Called at VBlank start (SetVSyncFlag support).
pub fn on_vblank_start(c: &mut Cpu) {
    let flag = m!(c).kernel.vsync_flag;
    if flag == 0 {
        return;
    }
    c.bus.write32(flag, 1);
    if m!(c).kernel.vsync_data != 0 {
        let csr = m!(c).hw.gs.read_priv(0x1000);
        let d = m!(c).kernel.vsync_data;
        c.bus.write64(d, csr);
    }
    m!(c).kernel.vsync_flag = 0;
    m!(c).kernel.vsync_data = 0;
}

/// Create the boot thread (id 1) running at `entry`.
pub fn start_main_thread(c: &mut Cpu, entry: u32, argc: u32, argv: u32) {
    m!(c).kernel.boot_argc = argc;
    m!(c).kernel.boot_argv = argv;
    let t = Thread {
        id: 1,
        status: Status::Run,
        priority: 0,
        init_priority: 0,
        entry,
        stack: 0,
        stack_size: 0,
        gp: 0,
        heap: 0,
        option: 0,
        attr: 0,
        wakeups: 0,
        suspends: 0,
        wait: WaitType::None,
        wait_id: 0,
        order: 0,
        ctx: Context::default(),
    };
    m!(c).kernel.threads.insert(1, t);
    m!(c).kernel.current = 1;
    c.pc = entry;
    c.g[4][0] = sx(argc);
    c.g[5][0] = sx(argv);
    c.g[31][0] = sx(THREAD_EXIT);
    c.cop0[12] = 0x7001_0C01;
    c.cop0[15] = 0x2E20;
    m!(c).hw.vu0.vf[0][3] = 0x3F80_0000;
    m!(c).hw.vu1.vf[0][3] = 0x3F80_0000;
}

fn signal_sema(c: &mut Cpu, id: i32, interrupt: bool) {
    let Some(s) = m!(c).kernel.semas.get_mut(&id) else {
        ret(c, -1);
        return;
    };
    let waiter = s.waiters.pop_front();
    if waiter.is_none() {
        s.count += 1;
    }
    ret(c, id as i64);
    if let Some(w) = waiter {
        m!(c).kernel.release(w, id as i64);
        if !interrupt {
            reschedule(c);
        }
    }
}

fn wait_sema(c: &mut Cpu, id: i32) {
    let in_handler = m!(c).kernel.in_handler;
    let cur = m!(c).kernel.current;
    let Some(s) = m!(c).kernel.semas.get_mut(&id) else {
        ret(c, -1);
        return;
    };
    if s.count > 0 {
        s.count -= 1;
        ret(c, id as i64);
        return;
    }
    if in_handler {
        eprintln!("[kernel] WaitSema({id}) inside an interrupt handler");
        ret(c, id as i64);
        return;
    }
    s.waiters.push_back(cur);
    ret(c, id as i64);
    block_current(c, WaitType::Sema, id);
}

fn create_thread(c: &mut Cpu) -> i32 {
    let p = arg(c, 0);
    let id = lowest_free(&m!(c).kernel.threads, 256);
    if id < 0 {
        return -1;
    }
    let heap = m!(c).kernel.cur().map(|t| t.heap).unwrap_or(0);
    let rd = |c: &mut Cpu, o: u32| c.bus.read32(p + o);
    let t = Thread {
        id,
        status: Status::Dormant,
        entry: rd(c, 4),
        stack: rd(c, 8),
        stack_size: rd(c, 12),
        gp: rd(c, 16),
        priority: rd(c, 20) as i32,
        init_priority: rd(c, 20) as i32,
        attr: rd(c, 28),
        option: rd(c, 32),
        heap,
        wakeups: 0,
        suspends: 0,
        wait: WaitType::None,
        wait_id: 0,
        order: 0,
        ctx: Context::default(),
    };
    if m!(c).kernel.trace_threads {
        eprintln!(
            "[thread] create {id} entry {:#010x} stack {:#010x}+{:#x} prio {}",
            t.entry, t.stack, t.stack_size, t.priority
        );
    }
    m!(c).kernel.threads.insert(id, t);
    id
}

fn exit_current(c: &mut Cpu, remove: bool) {
    let id = m!(c).kernel.current;
    if let Some(t) = m!(c).kernel.threads.get_mut(&id) {
        t.status = Status::Dormant;
    }
    if remove {
        m!(c).kernel.threads.remove(&id);
        m!(c).kernel.current = 0;
    }
    reschedule(c);
}

pub fn syscall_name(n: u32) -> &'static str {
    match n {
        0x01 => "ResetEE",
        0x02 => "SetGsCrt",
        0x04 => "Exit",
        0x06 => "LoadExecPS2",
        0x07 => "ExecPS2",
        0x10 => "AddIntcHandler",
        0x11 => "RemoveIntcHandler",
        0x12 => "AddDmacHandler",
        0x13 => "RemoveDmacHandler",
        0x14 => "EnableIntc",
        0x15 => "DisableIntc",
        0x16 => "EnableDmac",
        0x17 => "DisableDmac",
        0x18 | 0xFC => "SetAlarm",
        0x19 | 0xFD => "ReleaseAlarm",
        0x1A => "iEnableIntc",
        0x1B => "iDisableIntc",
        0x1C => "iEnableDmac",
        0x1D => "iDisableDmac",
        0x1E | 0xFE => "iSetAlarm",
        0x1F | 0xFF => "iReleaseAlarm",
        0x20 => "CreateThread",
        0x21 => "DeleteThread",
        0x22 => "StartThread",
        0x23 => "ExitThread",
        0x24 => "ExitDeleteThread",
        0x25 => "TerminateThread",
        0x26 => "iTerminateThread",
        0x29 => "ChangeThreadPriority",
        0x2A => "iChangeThreadPriority",
        0x2B => "RotateThreadReadyQueue",
        0x2C => "iRotateThreadReadyQueue",
        0x2D => "ReleaseWaitThread",
        0x2E => "iReleaseWaitThread",
        0x2F => "GetThreadId",
        0x30 => "ReferThreadStatus",
        0x31 => "iReferThreadStatus",
        0x32 => "SleepThread",
        0x33 => "WakeupThread",
        0x34 => "iWakeupThread",
        0x35 => "CancelWakeupThread",
        0x36 => "iCancelWakeupThread",
        0x37 => "SuspendThread",
        0x38 => "iSuspendThread",
        0x39 => "ResumeThread",
        0x3A => "iResumeThread",
        0x3C => "InitMainThread",
        0x3D => "InitHeap",
        0x3E => "EndOfHeap",
        0x40 => "CreateSema",
        0x41 => "DeleteSema",
        0x42 => "SignalSema",
        0x43 => "iSignalSema",
        0x44 => "WaitSema",
        0x45 => "PollSema",
        0x46 => "iPollSema",
        0x47 => "ReferSemaStatus",
        0x48 => "iReferSemaStatus",
        0x4A => "SetOsdConfigParam",
        0x4B => "GetOsdConfigParam",
        0x4C => "GetGsHParam",
        0x4D => "GetGsVParam",
        0x4E => "SetGsHParam",
        0x4F => "SetGsVParam",
        0x55..=0x59 => "TLB",
        0x5A => "kernel-patch",
        0x5B => "GetEntryAddress",
        0x64 => "FlushCache",
        0x68 => "iFlushCache",
        0x6B => "SifStopDma",
        0x70 => "GsGetIMR",
        0x71 => "GsPutIMR",
        0x73 => "SetVSyncFlag",
        0x74 => "SetSyscall",
        0x76 => "SifDmaStat",
        0x77 => "SifSetDma",
        0x78 => "SifSetDChain",
        0x79 => "SifSetReg",
        0x7A => "SifGetReg",
        0x7C => "Deci2Call",
        0x7E => "MachineType",
        0x7F => "GetMemorySize",
        0x83 => "FindAddress",
        0x85 => "SetMemoryMode",
        _ => "?",
    }
}

/// Handle a `syscall` instruction at `pc`. On entry `c.pc` is `pc + 4`.
pub fn syscall(c: &mut Cpu, pc: u32) {
    let mut n = c.g[3][0] as u32;
    let interrupt_variant = n & 0x8000_0000 != 0;
    if interrupt_variant {
        n = 0u32.wrapping_sub(n);
    }
    *m!(c).kernel.syscall_counts.entry(n).or_default() += 1;
    if m!(c).kernel.trace_syscalls {
        eprintln!(
            "[sys] {}({n:#04x}) {:08x} {:08x} {:08x} {:08x} from {pc:#010x} thread {}",
            syscall_name(n),
            arg(c, 0),
            arg(c, 1),
            arg(c, 2),
            arg(c, 3),
            m!(c).kernel.current
        );
    }
    // Syscalls redirected with SetSyscall to translated game code.
    if n < 0x100 && !matches!(n, 0x5A | 0x74 | 0x83) {
        let custom = c.bus.ram_u32(SYSCALL_TABLE + n * 4);
        if custom != 0 && crate::machine::has_code(custom & 0x1FFF_FFFF) {
            let a = [arg(c, 0), arg(c, 1), arg(c, 2), arg(c, 3)];
            let gp = c.g[28][0] as u32;
            let r = run_guest(c, custom & 0x1FFF_FFFF, a, gp);
            ret(c, r as i32 as i64);
            return;
        }
    }
    match n {
        0x01 => ret(c, 0),
        0x02 => {
            crate::machine::set_gs_crt(c, arg(c, 0), arg(c, 1), arg(c, 2));
            ret(c, 0)
        }
        0x04 => {
            let a0 = arg(c, 0);
            m!(c).kernel.halted = Some(format!("Exit({})", a0 as i32));
            c.pc = IDLE | UNWIND;
        }
        0x06 | 0x07 => {
            let path = c.bus.read_cstr(arg(c, 0));
            m!(c).kernel.halted = Some(format!("LoadExecPS2({path})"));
            c.pc = IDLE | UNWIND;
        }
        0x10 => add_handler(c, false),
        0x11 => remove_handler(c, false),
        0x12 => add_handler(c, true),
        0x13 => remove_handler(c, true),
        0x14 | 0x1A | 0x15 | 0x1B => {
            let bit = 1u32 << (arg(c, 0) & 31);
            let enable = n == 0x14 || n == 0x1A;
            let was = m!(c).hw.i_mask & bit != 0;
            if enable {
                m!(c).hw.i_mask |= bit;
            } else {
                m!(c).hw.i_mask &= !bit;
            }
            m!(c).hw.irq_dirty = true;
            ret(c, (was != enable) as i64)
        }
        0x16 | 0x1C | 0x17 | 0x1D => {
            let bit = 1u32 << (16 + (arg(c, 0) & 15));
            let enable = n == 0x16 || n == 0x1C;
            let was = m!(c).hw.d_stat & bit != 0;
            if enable {
                m!(c).hw.d_stat |= bit;
            } else {
                m!(c).hw.d_stat &= !bit;
            }
            m!(c).hw.irq_dirty = true;
            ret(c, (was != enable) as i64)
        }
        0x18 | 0x1E | 0xFC | 0xFE => {
            let id = lowest_free(&m!(c).kernel.alarms, 64);
            if id < 0 {
                ret(c, -1);
            } else {
                let due = c.cycles
                    + (arg(c, 0) & 0xFFFF) as u64 * crate::machine::HSYNC_CYCLES;
                let a = Alarm { callback: arg(c, 1), arg: arg(c, 2), gp: c.g[28][0] as u32, due };
                m!(c).kernel.alarms.insert(id, a);
                c.next_event = c.next_event.min(due);
                ret(c, id as i64);
            }
        }
        0x19 | 0x1F | 0xFD | 0xFF => {
            let a0 = arg(c, 0);
            m!(c).kernel.alarms.remove(&(a0 as i32));
            ret(c, 0)
        }
        0x20 => {
            let id = create_thread(c);
            ret(c, id as i64)
        }
        0x21 => {
            let a0 = arg(c, 0);
            m!(c).kernel.threads.remove(&(a0 as i32));
            ret(c, 0)
        }
        0x22 => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let a1 = arg(c, 1);
            let Some(t) = m!(c).kernel.threads.get_mut(&id) else {
                ret(c, -1);
                return;
            };
            let mut ctx = Context { pc: t.entry, ..Default::default() };
            ctx.g[4][0] = sx(a1);
            ctx.g[28][0] = sx(t.gp);
            ctx.g[29][0] = sx(t.stack + t.stack_size - CONTEXT_RESERVE);
            ctx.g[31][0] = sx(THREAD_EXIT);
            t.ctx = ctx;
            t.priority = t.init_priority;
            t.wakeups = 0;
            t.wait = WaitType::None;
            // A (re)started thread gets a fresh host thread in host-thread mode.
            m!(c).kernel.tokens.remove(&id);
            ret(c, id as i64);
            m!(c).kernel.make_ready(id);
            reschedule(c);
        }
        0x23 => exit_current(c, false),
        0x24 => exit_current(c, true),
        0x25 | 0x26 => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let ok = m!(c).kernel.threads.contains_key(&id);
            for s in m!(c).kernel.semas.values_mut() {
                s.waiters.retain(|w| *w != id);
            }
            if let Some(t) = m!(c).kernel.threads.get_mut(&id) {
                t.status = Status::Dormant;
                t.wait = WaitType::None;
            }
            ret(c, if ok { 0 } else { -1 })
        }
        0x29 | 0x2A => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let prio = (arg(c, 1) as i32) & 127;
            m!(c).kernel.order += 1;
            let order = m!(c).kernel.order;
            let Some(t) = m!(c).kernel.threads.get_mut(&id) else {
                ret(c, -1);
                return;
            };
            let old = t.priority;
            t.priority = prio;
            if t.status == Status::Ready {
                t.order = order;
            }
            ret(c, old as i64);
            if n == 0x29 {
                reschedule(c);
            } else {
                m!(c).kernel.reschedule = true;
            }
        }
        0x2B | 0x2C => {
            let prio = (arg(c, 0) as i32) & 127;
            ret(c, prio as i64);
            m!(c).kernel.order += 1;
            let order = m!(c).kernel.order;
            let cur = m!(c).kernel.current;
            let cur_prio = m!(c).kernel.threads.get(&cur).map(|t| t.priority);
            if cur_prio == Some(prio) {
                m!(c).kernel.threads.get_mut(&cur).unwrap().order = order;
            } else {
                let first = m!(c)
                    .kernel
                    .threads
                    .values()
                    .filter(|t| t.status == Status::Ready && t.priority == prio)
                    .min_by_key(|t| t.order)
                    .map(|t| t.id);
                if let Some(f) = first {
                    m!(c).kernel.threads.get_mut(&f).unwrap().order = order;
                }
            }
            if n == 0x2B {
                reschedule(c);
            } else {
                m!(c).kernel.reschedule = true;
            }
        }
        0x2D | 0x2E => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let waiting = m!(c)
                .kernel
                .threads
                .get(&id)
                .is_some_and(|t| matches!(t.status, Status::Wait | Status::WaitSuspend));
            if waiting {
                for s in m!(c).kernel.semas.values_mut() {
                    s.waiters.retain(|w| *w != id);
                }
                m!(c).kernel.release(id, -418);
                if n == 0x2D {
                    reschedule(c);
                }
            }
            let ok = m!(c).kernel.threads.contains_key(&id);
            ret(c, if ok { id as i64 } else { -1 })
        }
        0x2F => {
            let id = m!(c).kernel.current;
            ret(c, id as i64)
        }
        0x30 | 0x31 => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let p = arg(c, 1);
            let Some(t) = m!(c).kernel.threads.get(&id).cloned() else {
                ret(c, -1);
                return;
            };
            if p != 0 {
                let words = [
                    t.status as u32,
                    t.entry,
                    t.stack,
                    t.stack_size,
                    t.gp,
                    t.init_priority as u32,
                    t.priority as u32,
                    t.attr,
                    t.option,
                    match t.wait {
                        WaitType::Sema => 2,
                        WaitType::Sleep => 1,
                        WaitType::None => 0,
                    },
                    t.wait_id as u32,
                    t.wakeups as u32,
                ];
                for (i, w) in words.iter().enumerate() {
                    c.bus.write32(p + i as u32 * 4, *w);
                }
            }
            ret(c, t.status as i64)
        }
        0x32 => {
            let id = m!(c).kernel.current;
            ret(c, id as i64);
            let t = m!(c).kernel.threads.get_mut(&id).unwrap();
            if t.wakeups > 0 {
                t.wakeups -= 1;
            } else {
                block_current(c, WaitType::Sleep, 0);
            }
        }
        0x33 | 0x34 => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let Some(t) = m!(c).kernel.threads.get_mut(&id) else {
                ret(c, -1);
                return;
            };
            if t.wait == WaitType::Sleep {
                m!(c).kernel.release(id, id as i64);
                ret(c, id as i64);
                if n == 0x33 {
                    reschedule(c);
                }
            } else {
                t.wakeups += 1;
                ret(c, id as i64);
            }
        }
        0x35 | 0x36 => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let r = m!(c).kernel.threads.get_mut(&id).map(|t| std::mem::take(&mut t.wakeups));
            ret(c, r.map(|w| w as i64).unwrap_or(-1))
        }
        0x37 | 0x38 => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let cur = m!(c).kernel.current;
            if let Some(t) = m!(c).kernel.threads.get_mut(&id) {
                if id != cur {
                    if t.status == Status::Wait {
                        t.status = Status::WaitSuspend;
                    } else if t.status == Status::Ready {
                        t.status = Status::Suspend;
                    }
                    t.suspends += 1;
                }
                ret(c, id as i64)
            } else {
                ret(c, -1)
            }
        }
        0x39 | 0x3A => {
            let a0 = arg(c, 0);
            let id = m!(c).kernel.thread_id(a0 as i32);
            let mut ready = false;
            let ok = if let Some(t) = m!(c).kernel.threads.get_mut(&id) {
                if t.suspends > 0 {
                    t.suspends -= 1;
                    if t.suspends == 0 {
                        if t.status == Status::WaitSuspend {
                            t.status = Status::Wait;
                        } else if t.status == Status::Suspend {
                            ready = true;
                        }
                    }
                }
                true
            } else {
                false
            };
            if ready {
                m!(c).kernel.make_ready(id);
                if n == 0x39 {
                    reschedule(c);
                }
            }
            ret(c, if ok { id as i64 } else { -1 })
        }
        0x3C => {
            let (gp, stack, size, args) = (arg(c, 0), arg(c, 1), arg(c, 2), arg(c, 3));
            let top = if stack == 0xFFFF_FFFF { crate::bus::RAM_SIZE as u32 - 0x1000 } else { stack + size };
            if let Some(t) = m!(c).kernel.cur() {
                t.stack = top - size;
                t.stack_size = size;
                t.gp = gp;
            }
            if args != 0 {
                // argc/argv block: argc, argv[16], strings.
                let argc = m!(c).kernel.boot_argc.min(16);
                let mut payload = args + 4 + 16 * 4;
                for i in 0..argc {
                    let argv = m!(c).kernel.boot_argv;
                    let from = c.bus.read32(argv + 4 * i);
                    let s = c.bus.read_cstr(from);
                    c.bus.write32(args + 4 + 4 * i, payload);
                    let mut bytes = s.into_bytes();
                    bytes.push(0);
                    c.bus.write_bytes(payload, &bytes);
                    payload += bytes.len() as u32;
                }
                c.bus.write32(args, argc);
            }
            ret(c, (top - CONTEXT_RESERVE) as i32 as i64)
        }
        0x3D => {
            let (start, size) = (arg(c, 0), arg(c, 1));
            let heap = if size == 0xFFFF_FFFF {
                m!(c).kernel.cur().map(|t| t.stack).unwrap_or(0)
            } else {
                start + size
            };
            if let Some(t) = m!(c).kernel.cur() {
                t.heap = heap;
            }
            ret(c, heap as i32 as i64)
        }
        0x3E => {
            let h = m!(c).kernel.cur().map(|t| t.heap).unwrap_or(0);
            ret(c, h as i32 as i64)
        }
        0x40 => {
            let p = arg(c, 0);
            let s = Sema {
                count: c.bus.read32(p + 8) as i32,
                max: c.bus.read32(p + 4) as i32,
                init: c.bus.read32(p + 8) as i32,
                attr: c.bus.read32(p + 16),
                option: c.bus.read32(p + 20),
                waiters: VecDeque::new(),
            };
            let id = lowest_free(&m!(c).kernel.semas, 256);
            if id > 0 {
                m!(c).kernel.semas.insert(id, s);
            }
            ret(c, id as i64)
        }
        0x41 => {
            let id = arg(c, 0) as i32;
            let Some(s) = m!(c).kernel.semas.remove(&id) else {
                ret(c, -1);
                return;
            };
            for w in s.waiters {
                m!(c).kernel.release(w, -425);
            }
            ret(c, id as i64);
            reschedule(c);
        }
        0x42 => {
            if crate::frame_verify::enabled() {
                crate::frame_verify::on_signal_sema(c, arg(c, 0) as i32);
            }
            if crate::frame_sink::enabled() {
                crate::frame_sink::on_signal_sema(c, arg(c, 0) as i32);
            }
            signal_sema(c, arg(c, 0) as i32, false)
        }
        0x43 => signal_sema(c, arg(c, 0) as i32, true),
        0x44 => wait_sema(c, arg(c, 0) as i32),
        0x45 | 0x46 => {
            let id = arg(c, 0) as i32;
            match m!(c).kernel.semas.get_mut(&id) {
                Some(s) if s.count > 0 => {
                    s.count -= 1;
                    ret(c, id as i64)
                }
                _ => ret(c, -1),
            }
        }
        0x47 | 0x48 => {
            let id = arg(c, 0) as i32;
            let p = arg(c, 1);
            let Some(s) = m!(c).kernel.semas.get(&id).cloned() else {
                ret(c, -1);
                return;
            };
            let words =
                [s.count as u32, s.max as u32, s.init as u32, s.waiters.len() as u32, s.attr, s.option];
            for (i, w) in words.iter().enumerate() {
                c.bus.write32(p + i as u32 * 4, *w);
            }
            ret(c, id as i64)
        }
        0x4A => {
            let v = c.bus.read32(arg(c, 0));
            m!(c).kernel.osd_config = v;
            ret(c, 0)
        }
        0x4B => {
            let v = m!(c).kernel.osd_config;
            c.bus.write32(arg(c, 0), v);
            ret(c, 0)
        }
        // TLB setup and kernel patching done by _InitSys: accept and ignore.
        0x55..=0x5A | 0x5B | 0x64 | 0x68 | 0x6B => ret(c, 0),
        0x70 => {
            let v = m!(c).hw.gs.read_priv(0x1010);
            ret(c, v as i64)
        }
        0x71 => {
            let old = m!(c).hw.gs.read_priv(0x1010);
            let a0 = arg(c, 0);
            m!(c).hw.gs.write_priv(0x1010, a0 as u64);
            ret(c, old as i64)
        }
        0x73 => {
            let a0 = arg(c, 0);
            m!(c).kernel.vsync_flag = a0;
            let a1 = arg(c, 1);
            m!(c).kernel.vsync_data = a1;
            ret(c, 0)
        }
        0x74 => {
            let (num, addr) = (arg(c, 0), arg(c, 1));
            if num < 0x100 {
                c.bus.set_ram_u32(SYSCALL_TABLE + num * 4, addr);
            }
            ret(c, 0)
        }
        0x76 => {
            let r = crate::sif::dma_stat(c, arg(c, 0));
            ret(c, r as i64)
        }
        0x77 => {
            let r = crate::sif::set_dma(c, arg(c, 0), arg(c, 1));
            ret(c, r as i64)
        }
        0x78 => {
            if interrupt_variant {
                crate::sif::packet_processed(c);
            }
            ret(c, 0)
        }
        0x79 => {
            crate::sif::set_reg(c, arg(c, 0), arg(c, 1));
            ret(c, 0)
        }
        0x7A => {
            let r = crate::sif::get_reg(c, arg(c, 0));
            ret(c, r as i32 as i64)
        }
        0x7C => {
            let f = arg(c, 0);
            ret(c, if (1..=6).contains(&f) { -1 } else { 0 })
        }
        0x7E => ret(c, 0),
        0x7F => ret(c, crate::bus::RAM_SIZE as i64),
        // FindAddress(start, end, value): libkernl's _InitSys (0x42c1f0) looks up two
        // kernel syscall-table entries and loops until `found - 0x20c` (for 0x42c168)
        // equals `found - 0x168` (for 0x42c130). Answer consistently so both agree on a
        // fake table base in otherwise unused kernel RAM.
        0x83 => {
            const BASE: u32 = 0x8003_0000;
            let off = match arg(c, 2) {
                0x0042_c168 => 0x20c,
                0x0042_c130 => 0x168,
                _ => 0x100,
            };
            ret(c, (BASE + off) as i32 as i64)
        }
        0x85 => ret(c, 0),
        0x4C..=0x4F => ret(c, 0),
        _ => {
            eprintln!("[sys] unhandled syscall {n:#04x} ({}) from {pc:#010x}", syscall_name(n));
            ret(c, 0)
        }
    }
}
