//! Native implementations of the IOP library functions the recompiled
//! modules import, dispatched by `(library id, export index)`.
//!
//! Export indices are those of the IOP libraries the modules were linked
//! against (sysmem 1.1, intrman 1.2, thbase 1.2, sifcmd 1.1, ...); names and
//! semantics follow ps2-web-recomp `runtime/src/iop/libs.cpp` and `sif.cpp`
//! (MIT). Only what SNDDRV.IRX and LIBSD.IRX use is implemented; anything
//! else logs once and returns 0.

use crate::irx::LIBRARIES;
use crate::kernel::{self, Wait};
use crate::{sif, Iop, UNWIND};

const E_UNKNOWN_THREAD: i32 = -407;
const E_UNKNOWN_EVENT: i32 = -409;
const E_UNKNOWN_SEMA: i32 = -408;
const E_SEMA_ZERO: i32 = -419;
const E_EVENT_CONDITION: i32 = -421;

/// Called by generated code for `jal stub`. On a normal return `pc` is set to
/// `$ra`; a blocking call or a thread switch leaves `pc` with [`UNWIND`].
pub fn call(c: &mut Iop, lib: usize, idx: u32) {
    let ret = c.r[31];
    c.pc = ret;
    c.cycles += 8;
    if c.trace {
        eprintln!(
            "[iop-hle] {}#{idx}({:#x}, {:#x}, {:#x}, {:#x}) ra {ret:#x} thread {}",
            LIBRARIES.get(lib).unwrap_or(&"?"),
            c.r[4],
            c.r[5],
            c.r[6],
            c.r[7],
            c.k.current
        );
    }
    dispatch(c, lib, idx);
    if c.pc == ret && c.k.switch_pending && c.in_handler == 0 && c.k.current != 0 {
        // Preempted by a thread this call made ready.
        c.pc = ret | UNWIND;
        c.yield_now = true;
    }
}

fn v0(c: &mut Iop, v: i32) {
    c.r[2] = v as u32;
}

fn sysclock(c: &Iop) -> u64 {
    c.cycles
}

fn dispatch(c: &mut Iop, lib: usize, idx: u32) {
    let name = LIBRARIES.get(lib).copied().unwrap_or("?");
    match (name, idx) {
        // ------------------------------------------------------- sysmem
        ("sysmem", 4) => {
            let (mode, size, at) = (c.arg(0), c.arg(1), c.arg(2));
            let a = c.k.alloc(size, mode, at);
            if c.trace || a == 0 {
                eprintln!("[iop] AllocSysMemory({mode}, {size:#x}) = {a:#x}");
            }
            c.r[2] = a;
        }
        ("sysmem", 5) => {
            let a = c.arg(0);
            c.k.free(a);
            v0(c, 0);
        }
        ("sysmem", 6) => c.r[2] = crate::RAM_SIZE as u32,
        ("sysmem", 7) => c.r[2] = c.k.free_max(),
        ("sysmem", 8) => c.r[2] = c.k.free_total(),
        ("sysmem", 14) | ("stdio", 4) => {
            let s = format(c, c.arg(0), 1);
            kernel::tty(c, &s);
            c.r[2] = s.len() as u32;
        }
        // ------------------------------------------------------- loadcore
        ("loadcore", 4) | ("loadcore", 5) => v0(c, 0),
        ("loadcore", 6) => {
            let t = c.arg(0);
            let name = c.read_cstr(t + 12, 8);
            c.k.libraries.insert(name, t);
            v0(c, 0);
        }
        ("loadcore", 7) => v0(c, 0),
        // ------------------------------------------------------- intrman
        ("intrman", 4) => {
            let (line, mode, handler, arg) = (c.arg(0), c.arg(1), c.arg(2), c.arg(3));
            eprintln!("[iop] RegisterIntrHandler({line}, {mode}, {handler:#x}, {arg:#x})");
            c.k.intr.insert(line, kernel::IntrHandler { handler, arg, mode });
            v0(c, 0);
        }
        ("intrman", 5) => {
            let line = c.arg(0);
            c.k.intr.remove(&line);
            v0(c, 0);
        }
        ("intrman", 6) => {
            let line = c.arg(0);
            c.hw.set_mask(line, true);
            v0(c, 0);
        }
        ("intrman", 7) => {
            let (line, old) = (c.arg(0), c.arg(1));
            c.hw.set_mask(line, false);
            if old != 0 {
                c.set_ram32(old, line);
            }
            v0(c, 0);
        }
        ("intrman", 8) => {
            c.k.intr_suspended = true;
            v0(c, 0);
        }
        ("intrman", 9) => {
            c.k.intr_suspended = false;
            v0(c, 0);
        }
        ("intrman", 17) => {
            let was = c.k.intr_suspended;
            let p = c.arg(0);
            if p != 0 {
                c.set_ram32(p, if was { 0 } else { 1 });
            }
            c.k.intr_suspended = true;
            v0(c, if was { -102 } else { 0 });
        }
        ("intrman", 18) => {
            c.k.intr_suspended = c.arg(0) == 0;
            v0(c, 0);
        }
        ("intrman", 23) => c.r[2] = (c.in_handler > 0) as u32,
        // ------------------------------------------------------- thbase
        ("thbase", 4) => {
            let p = c.arg(0);
            let (attr, option, entry, stack, prio) = (c.ram32(p), c.ram32(p + 4), c.ram32(p + 8), c.ram32(p + 12), c.ram32(p + 16));
            let gp = c.r[28];
            let id = c.k.create_thread(entry, stack, prio as i32, gp, attr, option);
            eprintln!("[iop] CreateThread entry {entry:#x} stack {stack:#x} prio {prio} -> {id}");
            v0(c, id);
        }
        ("thbase", 5) => {
            let id = c.arg(0) as i32;
            let ok = c.k.threads.get(&id).is_some_and(|t| t.status == kernel::Status::Dormant);
            if ok {
                let st = c.k.threads.remove(&id).unwrap().stack;
                c.k.free(st);
            }
            v0(c, if ok { 0 } else { -414 });
        }
        ("thbase", 6) | ("thbase", 7) => {
            let (id, a0, a1) = (c.arg(0) as i32, c.arg(1), c.arg(2));
            let ok = c.k.start_thread(id, a0, if idx == 7 { a1 } else { 0 });
            v0(c, if ok { 0 } else { E_UNKNOWN_THREAD });
        }
        ("thbase", 8) | ("thbase", 9) => {
            kernel::exit_thread(c);
            c.pc |= UNWIND;
        }
        ("thbase", 14) | ("thbase", 15) => {
            let (id, prio) = (c.arg(0) as i32, c.arg(1) as i32);
            let id = if id == 0 { c.k.current } else { id };
            match c.k.threads.get_mut(&id) {
                Some(t) => {
                    t.prio = if prio == 0 { t.init_prio } else { prio };
                    c.k.switch_pending = true;
                    v0(c, 0)
                }
                None => v0(c, E_UNKNOWN_THREAD),
            }
        }
        ("thbase", 20) => c.r[2] = c.k.current as u32,
        ("thbase", 24) => {
            let cur = c.k.current;
            v0(c, 0);
            if let Some(t) = c.k.threads.get_mut(&cur) {
                if t.wakeups > 0 {
                    t.wakeups -= 1;
                    return;
                }
            }
            kernel::block(c, Wait::Sleep);
        }
        ("thbase", 25) | ("thbase", 26) => {
            let id = c.arg(0) as i32;
            if !c.k.threads.contains_key(&id) {
                v0(c, E_UNKNOWN_THREAD);
                return;
            }
            sif::wakeup(c, id);
            v0(c, 0);
        }
        ("thbase", 27) | ("thbase", 28) => {
            let id = c.arg(0) as i32;
            let id = if id == 0 { c.k.current } else { id };
            let n = c.k.threads.get_mut(&id).map(|t| std::mem::take(&mut t.wakeups)).unwrap_or(0);
            v0(c, n);
        }
        ("thbase", 33) => {
            let usec = c.arg(0) as u64;
            let cur = c.k.current;
            v0(c, 0);
            if kernel::block(c, Wait::Delay) {
                let at = c.cycles + (usec * crate::CLOCK / 1_000_000).max(1);
                c.k.schedule(at, kernel::Timed::DelayWake(cur));
            }
        }
        ("thbase", 34) => {
            let p = c.arg(0);
            let t = sysclock(c);
            c.set_ram32(p, t as u32);
            c.set_ram32(p + 4, (t >> 32) as u32);
            v0(c, 0);
        }
        ("thbase", 35) | ("thbase", 36) => {
            let p = c.arg(0);
            let delay = c.ram32(p) as u64 | (c.ram32(p + 4) as u64) << 32;
            let (h, a) = (c.arg(1), c.arg(2));
            kernel::set_alarm(c, delay, h, a);
            v0(c, 0);
        }
        ("thbase", 37) | ("thbase", 38) => {
            let (h, a) = (c.arg(0), c.arg(1));
            c.k.alarms.retain(|_, v| *v != (h, a));
            v0(c, 0);
        }
        ("thbase", 39) => {
            let (usec, p) = (c.arg(0) as u64, c.arg(1));
            let t = usec * crate::CLOCK / 1_000_000;
            c.set_ram32(p, t as u32);
            c.set_ram32(p + 4, (t >> 32) as u32);
            v0(c, 0);
        }
        ("thbase", 40) => {
            let p = c.arg(0);
            let clk = c.ram32(p) as u64 | (c.ram32(p + 4) as u64) << 32;
            let usec = clk * 1_000_000 / crate::CLOCK;
            let (ps, pu) = (c.arg(1), c.arg(2));
            c.set_ram32(ps, (usec / 1_000_000) as u32);
            c.set_ram32(pu, (usec % 1_000_000) as u32);
            v0(c, 0);
        }
        ("thbase", 43) => c.r[2] = sysclock(c) as u32,
        // ------------------------------------------------------- thsemap
        ("thsemap", 4) => {
            let p = c.arg(0);
            let (attr, option, init, max) = (c.ram32(p), c.ram32(p + 4), c.ram32(p + 8) as i32, c.ram32(p + 12) as i32);
            let id = c.k.new_object_id();
            c.k.semas.insert(id, kernel::Sema { count: init, max, attr, option, waiters: Default::default() });
            v0(c, id);
        }
        ("thsemap", 5) => {
            let id = c.arg(0) as i32;
            match c.k.semas.remove(&id) {
                Some(s) => {
                    for t in s.waiters {
                        c.k.wake(t, (-425i32) as u32);
                    }
                    v0(c, 0)
                }
                None => v0(c, E_UNKNOWN_SEMA),
            }
        }
        ("thsemap", 6) | ("thsemap", 7) => {
            let id = c.arg(0) as i32;
            let Some(s) = c.k.semas.get_mut(&id) else { return v0(c, E_UNKNOWN_SEMA) };
            match s.waiters.pop_front() {
                Some(t) => c.k.wake(t, 0),
                None => {
                    if s.count < s.max || s.max <= 0 {
                        s.count += 1;
                    }
                }
            }
            v0(c, 0);
        }
        ("thsemap", 8) => {
            let id = c.arg(0) as i32;
            let cur = c.k.current;
            v0(c, 0);
            let Some(s) = c.k.semas.get_mut(&id) else { return v0(c, E_UNKNOWN_SEMA) };
            if s.count > 0 {
                s.count -= 1;
                return;
            }
            s.waiters.push_back(cur);
            kernel::block(c, Wait::Sema(id));
        }
        ("thsemap", 9) => {
            let id = c.arg(0) as i32;
            let Some(s) = c.k.semas.get_mut(&id) else { return v0(c, E_UNKNOWN_SEMA) };
            if s.count > 0 {
                s.count -= 1;
                v0(c, 0)
            } else {
                v0(c, E_SEMA_ZERO)
            }
        }
        // ------------------------------------------------------- thevent
        ("thevent", 4) => {
            let p = c.arg(0);
            let (attr, option, bits) = (c.ram32(p), c.ram32(p + 4), c.ram32(p + 8));
            let id = c.k.new_object_id();
            c.k.events.insert(id, kernel::EventFlag { bits, attr, option, waiters: Default::default() });
            v0(c, id);
        }
        ("thevent", 5) => {
            let id = c.arg(0) as i32;
            match c.k.events.remove(&id) {
                Some(e) => {
                    for t in e.waiters {
                        c.k.wake(t, (-425i32) as u32);
                    }
                    v0(c, 0)
                }
                None => v0(c, E_UNKNOWN_EVENT),
            }
        }
        ("thevent", 6) | ("thevent", 7) => {
            let (id, bits) = (c.arg(0) as i32, c.arg(1));
            set_event(c, id, bits);
        }
        ("thevent", 8) | ("thevent", 9) => {
            let (id, bits) = (c.arg(0) as i32, c.arg(1));
            match c.k.events.get_mut(&id) {
                Some(e) => {
                    e.bits &= bits;
                    v0(c, 0)
                }
                None => v0(c, E_UNKNOWN_EVENT),
            }
        }
        ("thevent", 10) | ("thevent", 11) | ("thevent", 12) => {
            let (id, bits, mode, res) = (c.arg(0) as i32, c.arg(1), c.arg(2), c.arg(3));
            let Some(e) = c.k.events.get_mut(&id) else { return v0(c, E_UNKNOWN_EVENT) };
            if event_matches(e.bits, bits, mode) {
                let old = e.bits;
                event_take(&mut e.bits, bits, mode);
                if res != 0 {
                    c.set_ram32(res, old);
                }
                return v0(c, 0);
            }
            if idx != 10 {
                return v0(c, E_EVENT_CONDITION);
            }
            let cur = c.k.current;
            e.waiters.push_back(cur);
            if let Some(t) = c.k.threads.get_mut(&cur) {
                t.ev_bits = bits;
                t.ev_mode = mode;
                t.ev_result = res;
            }
            v0(c, 0);
            if !kernel::block(c, Wait::Event(id)) {
                if let Some(e) = c.k.events.get_mut(&id) {
                    e.waiters.retain(|&t| t != cur);
                }
            }
        }
        ("thevent", 13) | ("thevent", 14) => {
            let (id, p) = (c.arg(0) as i32, c.arg(1));
            let Some(e) = c.k.events.get(&id) else { return v0(c, E_UNKNOWN_EVENT) };
            let (attr, option, bits, n) = (e.attr, e.option, e.bits, e.waiters.len() as u32);
            c.set_ram32(p, attr);
            c.set_ram32(p + 4, option);
            c.set_ram32(p + 8, 0);
            c.set_ram32(p + 12, bits);
            c.set_ram32(p + 16, n);
            v0(c, 0);
        }
        // ------------------------------------------------------- sifman
        ("sifman", 4) | ("sifman", 5) | ("sifman", 6) => v0(c, 0),
        ("sifman", 7) => sif::set_dma(c),
        ("sifman", 8) => v0(c, -1),
        ("sifman", 29) => v0(c, 1),
        // ------------------------------------------------------- sifcmd
        ("sifcmd", 4) | ("sifcmd", 5) | ("sifcmd", 8) | ("sifcmd", 9) | ("sifcmd", 14) => v0(c, 0),
        ("sifcmd", 6) => c.r[2] = c.sif.sreg[(c.arg(0) & 31) as usize],
        ("sifcmd", 7) => {
            c.sif.sreg[(c.arg(0) & 31) as usize] = c.arg(1);
            v0(c, 0);
        }
        ("sifcmd", 10) => sif::add_cmd_handler(c),
        ("sifcmd", 11) => {
            let cid = c.arg(0);
            c.sif.cmd_handlers.remove(&cid);
            v0(c, 0);
        }
        ("sifcmd", 12) | ("sifcmd", 13) | ("sifcmd", 28) => sif::send_cmd(c),
        ("sifcmd", 17) => sif::register_rpc(c),
        ("sifcmd", 19) => sif::set_rpc_queue(c),
        ("sifcmd", 20) => sif::get_next_request(c),
        ("sifcmd", 21) => sif::exec_request(c),
        // ------------------------------------------------------- sysclib
        ("sysclib", 8) => {
            let ch = c.arg(0) as u8;
            let mut t = 0;
            if ch.is_ascii_uppercase() {
                t |= 1;
            }
            if ch.is_ascii_lowercase() {
                t |= 2;
            }
            if ch.is_ascii_digit() {
                t |= 4;
            }
            if ch.is_ascii_whitespace() {
                t |= 8;
            }
            if ch.is_ascii_punctuation() {
                t |= 16;
            }
            if ch.is_ascii_control() {
                t |= 32;
            }
            if ch.is_ascii_hexdigit() {
                t |= 64;
            }
            c.r[2] = t;
        }
        ("sysclib", 12) | ("sysclib", 13) => {
            let (d, s, n) = (c.arg(0), c.arg(1), c.arg(2));
            let data = c.read_bytes(s, n as usize);
            c.write_bytes(d, &data);
            c.r[2] = d;
        }
        ("sysclib", 14) => {
            let (d, v, n) = (c.arg(0), c.arg(1), c.arg(2));
            c.write_bytes(d, &vec![v as u8; n as usize]);
            c.r[2] = d;
        }
        ("sysclib", 16) => {
            let (s, d, n) = (c.arg(0), c.arg(1), c.arg(2));
            let data = c.read_bytes(s, n as usize);
            c.write_bytes(d, &data);
        }
        ("sysclib", 17) => {
            let (d, n) = (c.arg(0), c.arg(1));
            c.write_bytes(d, &vec![0u8; n as usize]);
        }
        ("sysclib", 19) => {
            let s = format(c, c.arg(1), 2);
            let mut b = s.into_bytes();
            let n = b.len();
            b.push(0);
            let d = c.arg(0);
            c.write_bytes(d, &b);
            c.r[2] = n as u32;
        }
        ("sysclib", 27) => c.r[2] = c.read_cstr(c.arg(0), 0x10000).len() as u32,
        _ => {
            let n = c.k.missing.entry((lib, idx)).or_default();
            *n += 1;
            if *n == 1 {
                eprintln!("[iop] missing import {name}#{idx} (ra {:#x})", c.r[31]);
            }
            v0(c, 0);
        }
    }
}

fn event_matches(have: u32, bits: u32, mode: u32) -> bool {
    if mode & 1 != 0 {
        have & bits != 0
    } else {
        have & bits == bits
    }
}

fn event_take(have: &mut u32, bits: u32, mode: u32) {
    if mode & 0x10 != 0 {
        *have &= !bits;
    }
    if mode & 0x20 != 0 {
        *have = 0;
    }
}

fn set_event(c: &mut Iop, id: i32, bits: u32) {
    let Some(e) = c.k.events.get_mut(&id) else { return v0(c, E_UNKNOWN_EVENT) };
    e.bits |= bits;
    let waiters: Vec<i32> = e.waiters.iter().copied().collect();
    let mut woken = Vec::new();
    for t in waiters {
        let Some(th) = c.k.threads.get(&t) else { continue };
        let (tb, tm, tr) = (th.ev_bits, th.ev_mode, th.ev_result);
        let e = c.k.events.get_mut(&id).unwrap();
        if event_matches(e.bits, tb, tm) {
            let old = e.bits;
            event_take(&mut e.bits, tb, tm);
            e.waiters.retain(|&w| w != t);
            if tr != 0 {
                c.set_ram32(tr, old);
            }
            woken.push(t);
        }
    }
    for t in woken {
        c.k.wake(t, 0);
    }
    v0(c, 0);
}

/// printf-style formatting with arguments from `first` on.
pub fn format(c: &Iop, fmt_addr: u32, first: usize) -> String {
    let fmt = c.read_cstr(fmt_addr, 1024);
    let b = fmt.as_bytes();
    let mut out = String::new();
    let mut next = first;
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'%' {
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        i += 1;
        let mut zero = false;
        let mut width = 0usize;
        let mut left = false;
        while i < b.len() && b"-+ #0".contains(&b[i]) {
            zero |= b[i] == b'0';
            left |= b[i] == b'-';
            i += 1;
        }
        while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
            if b[i].is_ascii_digit() {
                width = width * 10 + (b[i] - b'0') as usize;
            }
            i += 1;
        }
        while i < b.len() && (b[i] == b'l' || b[i] == b'h') {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        let conv = b[i];
        i += 1;
        let s = match conv {
            b'%' => "%".to_string(),
            b'd' | b'i' => {
                next += 1;
                (c.arg(next - 1) as i32).to_string()
            }
            b'u' => {
                next += 1;
                c.arg(next - 1).to_string()
            }
            b'x' | b'p' => {
                next += 1;
                format!("{:x}", c.arg(next - 1))
            }
            b'X' => {
                next += 1;
                format!("{:X}", c.arg(next - 1))
            }
            b'c' => {
                next += 1;
                ((c.arg(next - 1) as u8) as char).to_string()
            }
            b's' => {
                next += 1;
                c.read_cstr(c.arg(next - 1), 256)
            }
            other => format!("%{}", other as char),
        };
        if s.len() < width {
            let pad = width - s.len();
            if left {
                out += &s;
                out += &" ".repeat(pad);
            } else {
                out += &(if zero { "0" } else { " " }).repeat(pad);
                out += &s;
            }
        } else {
            out += &s;
        }
    }
    out
}
