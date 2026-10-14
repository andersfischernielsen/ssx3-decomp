//! The IOP end of SIF: user command handlers (`sceSifAddCmdHandler`), RPC
//! servers (`sceSifRegisterRpc` + `sceSifGetNextRequest`/`sceSifExecRequest`),
//! and IOP → EE traffic (`sceSifSendCmd`, `sceSifSetDma`).
//!
//! The EE side of SIF (packet parsing, bind/call bookkeeping) stays in
//! `ssx3-ps2/src/sif.rs`; it hands commands addressed to a recompiled module
//! to [`IopSif::incoming`], and this module processes them at the next IOP
//! service point. Replies are collected in [`crate::Outbox`].

use std::collections::{BTreeMap, VecDeque};

use crate::irx::layout;
use crate::kernel::{Status, Wait};
use crate::{Iop, EXEC_FINISH, UNWIND};

pub const CMD_REND: u32 = 0x8000_0008;

#[derive(Clone, Debug)]
pub enum Incoming {
    /// A user (non-system) SIF command: the raw packet.
    UserCmd { cid: u32, packet: Vec<u8> },
    /// `sceSifCallRpc` from the EE: the call packet words and the send data.
    Call { words: [u32; 14], data: Vec<u8> },
}

#[derive(Clone, Copy, Debug)]
pub struct Server {
    pub sd: u32,
    pub func: u32,
    pub buf: u32,
    pub qd: u32,
}

#[derive(Clone, Debug)]
struct Request {
    words: [u32; 14],
    data: Vec<u8>,
}

#[derive(Default)]
pub struct IopSif {
    pub cmd_handlers: BTreeMap<u32, (u32, u32)>,
    pub servers: BTreeMap<u32, Server>,
    /// Queue descriptor → owning thread.
    pub queues: BTreeMap<u32, i32>,
    requests: BTreeMap<u32, VecDeque<Request>>,
    /// Requests handed out by `sceSifGetNextRequest`, by server data address.
    active: BTreeMap<u32, Request>,
    pub incoming: VecDeque<Incoming>,
    pub sreg: [u32; 32],
    next_dma_id: u32,
    pub calls: u64,
    pub user_cmds: u64,
    pub sent_cmds: u64,
}

impl IopSif {
    pub fn has_server(&self, sid: u32) -> bool {
        self.servers.contains_key(&sid)
    }
    pub fn has_cmd_handler(&self, cid: u32) -> bool {
        self.cmd_handlers.contains_key(&cid)
    }
}

/// Process EE → IOP traffic received since the last service point.
pub fn service(c: &mut Iop) {
    while let Some(m) = c.sif.incoming.pop_front() {
        match m {
            Incoming::UserCmd { cid, packet } => {
                c.sif.user_cmds += 1;
                let Some(&(handler, data)) = c.sif.cmd_handlers.get(&cid) else {
                    eprintln!("[iop-sif] command {cid:#x} has no handler");
                    continue;
                };
                let n = packet.len().min(0x400);
                c.write_bytes(layout::CMD_RECV, &packet[..n]);
                if c.trace {
                    eprintln!("[iop-sif] cmd {cid:#x} -> {handler:#x}");
                }
                c.call_guest(handler, &[layout::CMD_RECV, data]);
            }
            Incoming::Call { words, data } => {
                c.sif.calls += 1;
                let sid = words[13] ^ 0xACAC_ACAC;
                let Some(s) = c.sif.servers.get(&sid).copied() else {
                    eprintln!("[iop-sif] call to unregistered server {sid:#x}");
                    continue;
                };
                c.sif.requests.entry(s.qd).or_default().push_back(Request { words, data });
                // The queue's thread is woken like iWakeupThread does.
                if let Some(&tid) = c.sif.queues.get(&s.qd) {
                    wakeup(c, tid);
                }
            }
        }
    }
}

/// `WakeupThread` semantics: wake a sleeping thread or count a wakeup.
pub fn wakeup(c: &mut Iop, tid: i32) {
    let Some(t) = c.k.threads.get_mut(&tid) else { return };
    if t.status == Status::Wait && t.wait == Wait::Sleep {
        c.k.wake(tid, 0);
    } else {
        t.wakeups += 1;
    }
}

fn rend(dest: u32, words: &[u32; 14], command: u32) -> Vec<u8> {
    let w = [48u32, dest, CMD_REND, 0, words[4], words[5], words[6], words[7], command, 0, 0, 0];
    w.iter().flat_map(|v| v.to_le_bytes()).collect()
}

// --------------------------------------------------------------- sifcmd natives

pub fn register_rpc(c: &mut Iop) {
    let (sd, sid, func, buf) = (c.arg(0), c.arg(1), c.arg(2), c.arg(3));
    let (cfunc, cbuf, qd) = (c.arg(4), c.arg(5), c.arg(6));
    c.set_ram32(sd, sid);
    c.set_ram32(sd + 4, func);
    c.set_ram32(sd + 8, buf);
    c.set_ram32(sd + 16, cfunc);
    c.set_ram32(sd + 20, cbuf);
    c.set_ram32(sd + 64, qd);
    c.sif.servers.insert(sid, Server { sd, func, buf, qd });
    eprintln!("[iop-sif] server {sid:#010x} registered (func {func:#x} buf {buf:#x} queue {qd:#x})");
    c.r[2] = 0;
}

pub fn set_rpc_queue(c: &mut Iop) {
    let (qd, tid) = (c.arg(0), c.arg(1) as i32);
    c.set_ram32(qd, tid as u32);
    c.sif.queues.insert(qd, tid);
    c.r[2] = 0;
}

pub fn get_next_request(c: &mut Iop) {
    let qd = c.arg(0);
    let Some(req) = c.sif.requests.get_mut(&qd).and_then(|q| q.pop_front()) else {
        c.r[2] = 0;
        return;
    };
    let sid = req.words[13] ^ 0xACAC_ACAC;
    let s = c.sif.servers[&sid];
    let w = req.words;
    // SifRpcServerData_t fields: size 12, client 28, pkt_addr 32,
    // rpc_number 36, receive 40, rsize 44, rmode 48, rid 52.
    c.set_ram32(s.sd + 12, w[9]);
    c.set_ram32(s.sd + 28, w[7]);
    c.set_ram32(s.sd + 32, w[5]);
    c.set_ram32(s.sd + 36, w[8]);
    c.set_ram32(s.sd + 40, w[10]);
    c.set_ram32(s.sd + 44, w[11]);
    c.set_ram32(s.sd + 48, w[12]);
    c.set_ram32(s.sd + 52, w[6]);
    if s.buf != 0 && !req.data.is_empty() {
        let data = req.data.clone();
        c.write_bytes(s.buf, &data);
    }
    c.sif.active.insert(s.sd, req);
    c.r[2] = s.sd;
}

/// `sceSifExecRequest(sd)`: run the server function as a guest tail call that
/// returns to [`EXEC_FINISH`], so it may block like any thread code.
pub fn exec_request(c: &mut Iop) {
    let sd = c.arg(0);
    let Some(req) = c.sif.active.get(&sd) else {
        eprintln!("[iop-sif] ExecRequest on {sd:#x} without a request");
        return;
    };
    let (fno, size) = (req.words[8], req.words[9]);
    let func = c.ram32(sd + 4);
    let buf = c.ram32(sd + 8);
    let cur = c.k.current;
    let ra = c.r[31];
    match c.k.threads.get_mut(&cur) {
        Some(t) => t.exec_stack.push((sd, ra)),
        None => {
            eprintln!("[iop-sif] ExecRequest outside a thread");
            return;
        }
    }
    if c.trace {
        eprintln!("[iop-sif] exec fn {fno:#x} size {size} -> {func:#x}");
    }
    c.r[4] = fno;
    c.r[5] = buf;
    c.r[6] = size;
    c.r[31] = EXEC_FINISH;
    c.pc = func | UNWIND;
}

pub fn exec_finish(c: &mut Iop) {
    let cur = c.k.current;
    let Some((sd, ra)) = c.k.threads.get_mut(&cur).and_then(|t| t.exec_stack.pop()) else {
        c.fatal(EXEC_FINISH, "RPC finish without ExecRequest");
        return;
    };
    let result = c.r[2];
    if let Some(req) = c.sif.active.remove(&sd) {
        let (recv, rsize) = (req.words[10], req.words[11]);
        if recv != 0 && rsize != 0 && result != 0 {
            let n = ((rsize + 3) & !3) as usize;
            let data = c.read_bytes(result, n);
            c.out.ee_writes.push((recv, data));
        }
        let p = rend(req.words[1], &req.words, 0x8000_000A);
        c.out.packets.push(p);
    }
    c.pc = ra;
}

pub fn add_cmd_handler(c: &mut Iop) {
    let (cid, handler, data) = (c.arg(0), c.arg(1), c.arg(2));
    c.sif.cmd_handlers.insert(cid, (handler, data));
    eprintln!("[iop-sif] command {cid:#x} handler {handler:#x}");
    c.r[2] = 0;
}

pub fn send_cmd(c: &mut Iop) {
    let (cid, packet, psize) = (c.arg(0), c.arg(1), c.arg(2));
    let (src, dst, size) = (c.arg(3), c.arg(4), c.arg(5));
    if size != 0 && dst != 0 {
        let data = c.read_bytes(src, size as usize);
        c.out.ee_writes.push((dst, data));
    }
    let psize = psize.clamp(16, 0x70);
    let mut bytes = c.read_bytes(packet, psize as usize);
    bytes[0..4].copy_from_slice(&((psize & 0xFF) | (size << 8)).to_le_bytes());
    bytes[4..8].copy_from_slice(&dst.to_le_bytes());
    bytes[8..12].copy_from_slice(&cid.to_le_bytes());
    if c.trace {
        let w: Vec<String> = bytes.chunks(4).take(8).map(|b| format!("{:08x}", u32::from_le_bytes(b.try_into().unwrap()))).collect();
        eprintln!("[iop-sif] send cmd {cid:#x}: {} (+{size} bytes -> {dst:#x})", w.join(" "));
    }
    c.out.packets.push(bytes);
    c.sif.sent_cmds += 1;
    c.sif.next_dma_id += 1;
    c.r[2] = c.sif.next_dma_id;
}

/// `sceSifSetDma(list, count)` from the IOP: IOP → EE copies.
pub fn set_dma(c: &mut Iop) {
    let (list, count) = (c.arg(0), c.arg(1));
    for i in 0..count {
        let b = list + i * 16;
        let (src, dst, size) = (c.ram32(b), c.ram32(b + 4), c.ram32(b + 8));
        let data = c.read_bytes(src, size as usize);
        c.out.ee_writes.push((dst, data));
    }
    c.sif.next_dma_id += 1;
    c.r[2] = c.sif.next_dma_id;
}
