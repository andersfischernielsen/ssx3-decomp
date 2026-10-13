//! SIF: the EE↔IOP link, with the IOP side provided natively.
//!
//! The game's own EE-side SIF/RPC library code runs recompiled. It talks to
//! the IOP by DMA (`SifSetDma` syscall) and receives replies through SIF0 DMA
//! interrupts. This module plays the IOP: it parses the command packets the EE
//! sends, answers RPC calls with native implementations of the IOP servers
//! (loadfile, fileio, iopheap, cdvdfsv, pad, mc, EA's sound driver), and writes
//! reply packets into the EE's receive buffer, raising DMAC channel 5.
//!
//! Protocol details are ported from ps2-web-recomp `runtime/src/iop/sif.cpp`
//! and `cdvdfsv.cpp` (MIT, Copyright (c) 2026 Samir Nuri).

use std::collections::{BTreeMap, VecDeque};
use std::path::PathBuf;

use crate::cpu::Cpu;
use crate::iso::{Disc, SECTOR};

const RPC_RECV_ADDR: u32 = 0xDEAD_BEF0;
const SERVER_XOR: u32 = 0xACAC_ACAC;
const CMD_BUFFER: u32 = 0x0001_E000;
const CMD_REND: u32 = 0x8000_0008;
const CMD_SET_SREG: u32 = 0x8000_0001;

/// `(method, args, send_size, recv, recv_size) -> done`
type Server = fn(&mut Cpu, u32, u32, u32, u32, u32) -> bool;

pub struct Sif {
    mainaddr: u32,
    msflag: u32,
    smflag: u32,
    ee_recv: u32,
    data_addr: u32,
    processed: bool,
    outgoing: VecDeque<Vec<u8>>,
    sreg: [u32; 32],
    ee_dma_id: u32,
    ee_dma_done: u64,
    pub disc: Option<Disc>,
    fds: BTreeMap<i32, (u32, u32, u32)>, // fd → (sector, size, pos)
    next_fd: i32,
    iop_heap: u32,
    stream_pos: u32,
    pub trace: bool,
    pub loaded_modules: Vec<String>,
    pub unknown_servers: BTreeMap<u32, u32>,
    /// Host pad state (active-high button bits) per port.
    pub pad_buttons: [u16; 2],
    pub loadfile_version: [u8; 4],
}

impl Default for Sif {
    fn default() -> Self {
        Self::new()
    }
}

impl Sif {
    pub fn new() -> Self {
        Self {
            mainaddr: 0,
            msflag: 0,
            smflag: 0x60000,
            ee_recv: 0,
            data_addr: 0,
            processed: true,
            outgoing: VecDeque::new(),
            sreg: [0; 32],
            ee_dma_id: 1,
            ee_dma_done: 0,
            disc: None,
            fds: BTreeMap::new(),
            next_fd: 3,
            iop_heap: 0x0010_0000,
            stream_pos: 0,
            trace: std::env::var("SSX3_TRACE_SIF").is_ok(),
            loaded_modules: Vec::new(),
            unknown_servers: BTreeMap::new(),
            pad_buttons: [0; 2],
            loadfile_version: *b"3000",
        }
    }

    pub fn open_disc(&mut self, path: PathBuf) -> std::io::Result<()> {
        self.disc = Some(Disc::open(&path)?);
        Ok(())
    }
}

fn rd(c: &mut Cpu, a: u32) -> u32 {
    c.bus.ram_u32(a & 0x01FF_FFFF)
}
fn wr(c: &mut Cpu, a: u32, v: u32) {
    c.bus.set_ram_u32(a & 0x01FF_FFFF, v);
}

fn send(c: &mut Cpu, words: &[u32]) {
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    c.sif.outgoing.push_back(bytes);
    pump(c);
}

/// RPC end packet: header(4 words) + record, packet, rpc, client, command,
/// server, buffer, cbuffer.
#[allow(clippy::too_many_arguments)]
fn rend(dest: u32, record: u32, packet: u32, rpc: u32, client: u32, command: u32, server: u32, buf: u32, cbuf: u32) -> [u32; 12] {
    [48, dest, CMD_REND, 0, record, packet, rpc, client, command, server, buf, cbuf]
}

/// Deliver the next queued packet to the EE once it acknowledged the last.
pub fn pump(c: &mut Cpu) {
    if !c.sif.processed || c.sif.outgoing.is_empty() || c.sif.ee_recv == 0 {
        return;
    }
    let p = c.sif.outgoing.pop_front().unwrap();
    if c.sif.trace {
        let w: Vec<String> = p.chunks(4).take(12).map(|b| format!("{:08x}", u32::from_le_bytes(b.try_into().unwrap()))).collect();
        eprintln!("[sif] -> ee {}", w.join(" "));
    }
    let at = c.sif.ee_recv;
    c.bus.write_bytes(at, &p);
    c.sif.processed = false;
    c.bus.hw.d_stat |= 1 << 5;
    c.bus.hw.irq_dirty = true;
    c.next_event = c.cycles;
}

pub fn packet_processed(c: &mut Cpu) {
    c.sif.processed = true;
    pump(c);
}

pub fn get_reg(c: &mut Cpu, reg: u32) -> u32 {
    match reg {
        1 => c.sif.mainaddr,
        2 => CMD_BUFFER,
        3 => c.sif.msflag,
        4 => c.sif.smflag,
        0x8000_0000 => 0,
        0x8000_0002 => 1,
        r if r & 0x8000_0000 != 0 => c.sif.sreg[(r & 31) as usize],
        _ => 0,
    }
}

pub fn set_reg(c: &mut Cpu, reg: u32, v: u32) {
    match reg {
        1 => c.sif.mainaddr = v,
        3 => c.sif.msflag |= v,
        // SMFLAG keeps SIFINIT|CMDINIT|BOOTEND set: our IOP is always booted.
        4 => {}
        r if r & 0x8000_0000 != 0 => c.sif.sreg[(r & 31) as usize] = v,
        _ => {}
    }
}

pub fn dma_stat(c: &mut Cpu, id: u32) -> i32 {
    if id + 1 == c.sif.ee_dma_id && c.cycles < c.sif.ee_dma_done {
        0
    } else {
        -1
    }
}

/// `SifSetDma(transfers, count)`: each transfer is {src, dest, size, attr}.
pub fn set_dma(c: &mut Cpu, transfers: u32, count: u32) -> u32 {
    let count = count.max(1);
    let mut bytes = 0u64;
    for i in 0..count {
        let base = transfers + i * 16;
        let src = rd(c, base) & 0x01FF_FFFF;
        let dst = rd(c, base + 4);
        let size = rd(c, base + 8) & 0x7FFF_FFFF;
        bytes += size as u64;
        if dst == RPC_RECV_ADDR {
            c.sif.data_addr = src;
        } else if dst == 0 {
            reboot(c, src);
        } else if dst == CMD_BUFFER {
            command(c, src, size);
        } else {
            // Plain EE→IOP data: mirror into IOP RAM.
            let data = c.bus.read_bytes(src, size as usize);
            let o = (dst & 0x1F_FFFF) as usize;
            let end = (o + data.len()).min(c.bus.hw_iop_ram_len());
            let n = end.saturating_sub(o);
            c.bus.iop_ram_mut()[o..end].copy_from_slice(&data[..n]);
        }
    }
    c.sif.ee_dma_done = c.sif.ee_dma_done.max(c.cycles) + 256 + bytes * 2;
    let id = c.sif.ee_dma_id;
    c.sif.ee_dma_id += 1;
    id
}

fn reboot(c: &mut Cpu, src: u32) {
    let path = c.bus.read_cstr(src + 24);
    eprintln!("[sif] IOP reboot: \"{path}\"");
    // The replacement IOP image carries its own LOADFILE; libsif checks its
    // version string ("PsIIloadfile" + 4 digits) after the reboot.
    if let Some(img) = path.split_once(' ').map(|(_, p)| p.to_string()) {
        if let Some(d) = c.sif.disc.as_mut() {
            if let Some(e) = d.find(&img) {
                let data = d.read_vec(e.sector, e.size.div_ceil(SECTOR as u32));
                let tag = b"PsIIloadfile";
                if let Some(i) = data.windows(tag.len()).position(|w| w == tag) {
                    let v = &data[i + tag.len()..i + tag.len() + 4];
                    c.sif.loadfile_version.copy_from_slice(v);
                    eprintln!("[sif] LOADFILE version {}", String::from_utf8_lossy(v));
                }
            }
        }
    }
}

fn command(c: &mut Cpu, src: u32, size: u32) {
    let w: Vec<u32> = (0..14).map(|i| rd(c, src + i * 4)).collect();
    let cid = w[2];
    if c.sif.trace {
        let s: Vec<String> = w.iter().map(|x| format!("{x:08x}")).collect();
        eprintln!("[sif] ee -> cmd {}", s.join(" "));
    }
    let _ = size;
    match cid {
        0x8000_0001 => c.sif.sreg[(w[4] & 31) as usize] = w[5],
        0x8000_0002 => {
            if w[3] == 0 {
                c.sif.ee_recv = w[4] & 0x01FF_FFFF;
            } else {
                send(c, &[24, 0, CMD_SET_SREG, 0, 0, 1]);
            }
        }
        0x8000_0008 => {} // RPC end for IOP-side clients: we have none.
        0x8000_0009 => {
            // Bind: record, packet, rpc, client, server id.
            let (record, packet, rpc, client, server) = (w[4], w[5], w[6], w[7], w[8]);
            let known = server_for(server).is_some();
            if !known {
                *c.sif.unknown_servers.entry(server).or_default() += 1;
            }
            if c.sif.trace || !known {
                eprintln!("[sif] bind {server:#010x}{}", if known { "" } else { " (no HLE server; answering generically)" });
            }
            let r = rend(w[1], record, packet, rpc, client, 0x8000_0009, server ^ SERVER_XOR, RPC_RECV_ADDR, 0xDEAD_CAFE);
            send(c, &r);
        }
        0x8000_000A => {
            // Call: record, packet, rpc, client, fno, ssize, recv, rsize, rmode, server.
            let (record, packet, rpc, client) = (w[4], w[5], w[6], w[7]);
            let (fno, ssize, recv, rsize, server) = (w[8], w[9], w[10], w[11], w[13] ^ SERVER_XOR);
            let args = c.sif.data_addr;
            if c.sif.trace {
                eprintln!("[sif] call {server:#010x} fn {fno:#x} send {ssize} recv {recv:#x}/{rsize}");
            }
            let done = match server_for(server) {
                Some(f) => f(c, fno, args, ssize, recv & 0x01FF_FFFF, rsize),
                None => {
                    *c.sif.unknown_servers.entry(server).or_default() += 1;
                    if rsize > 0 {
                        let z = vec![0u8; rsize as usize];
                        c.bus.write_bytes(recv, &z);
                    }
                    true
                }
            };
            if done {
                let r = rend(w[1], record, packet, rpc, client, 0x8000_000A, 0, 0, 0);
                send(c, &r);
            }
        }
        0x8000_000C => {
            // Get other data: copy IOP memory to EE.
            let (record, packet, rpc, receive, srca, dst, n) = (w[4], w[5], w[6], w[7], w[8], w[9], w[10]);
            let o = (srca & 0x1F_FFFF) as usize;
            let data = c.bus.iop_ram_mut()[o..(o + n as usize).min(0x20_0000)].to_vec();
            c.bus.write_bytes(dst, &data);
            let r = rend(w[1], record, packet, rpc, receive, 0x8000_000C, 0, 0, 0);
            send(c, &r);
        }
        _ => {
            if c.sif.trace {
                eprintln!("[sif] unhandled command {cid:#010x}");
            }
        }
    }
}

fn server_for(id: u32) -> Option<Server> {
    Some(match id {
        0x8000_0001 => fileio,
        0x8000_0003 => iopheap,
        0x8000_0006 => loadfile,
        0x8000_0592 => cd_init,
        0x8000_0593 => cd_scmd,
        0x8000_0595 => cd_ncmd,
        0x8000_0596 => cd_misc,
        0x8000_0597 => cd_search,
        0x8000_059A | 0x8000_059C => cd_ready,
        0x8000_0100 | 0x8000_0101 => crate::pad::rpc,
        0x8000_0400 => mcserv,
        0x0053_4E44 => snd,
        _ => return None,
    })
}

// ------------------------------------------------------------------ servers

fn loadfile(c: &mut Cpu, method: u32, args: u32, _s: u32, ret: u32, _rs: u32) -> bool {
    match method {
        0x00 => {
            let path = c.bus.read_cstr(args + 8);
            eprintln!("[loadfile] {path}");
            c.sif.loaded_modules.push(path);
            let id = c.sif.loaded_modules.len() as u32 + 1;
            wr(c, ret, id);
            wr(c, ret + 4, 0);
        }
        0x06 => {
            c.sif.loaded_modules.push("<buffer>".into());
            let id = c.sif.loaded_modules.len() as u32 + 1;
            wr(c, ret, id);
            wr(c, ret + 4, 0);
        }
        0x09 => wr(c, ret, (-202i32) as u32),
        0xFF => {
            let v = c.sif.loadfile_version;
            c.bus.write_bytes(ret, &v)
        }
        _ => wr(c, ret, 0),
    }
    true
}

fn iopheap(c: &mut Cpu, method: u32, args: u32, _s: u32, ret: u32, _rs: u32) -> bool {
    match method {
        1 | 4 => {
            let size = (rd(c, args) + 255) & !255;
            let a = c.sif.iop_heap;
            c.sif.iop_heap += size;
            if c.sif.iop_heap > 0x1F_0000 {
                c.sif.iop_heap = 0x10_0000;
            }
            wr(c, ret, a)
        }
        5 => wr(c, ret, 0x20_0000),
        6 | 7 => wr(c, ret, 0x0E_0000),
        _ => wr(c, ret, 0),
    }
    true
}

fn fileio(c: &mut Cpu, method: u32, args: u32, _s: u32, ret: u32, _rs: u32) -> bool {
    let word = |c: &mut Cpu, i: u32| rd(c, args + 4 * i);
    let result: i32 = match method {
        0 => {
            let path = c.bus.read_cstr(args + 4);
            let found = if path.to_ascii_lowercase().starts_with("cdrom") {
                c.sif.disc.as_mut().and_then(|d| d.find(&path))
            } else {
                None
            };
            let r = match found {
                Some(e) if !e.dir => {
                    let fd = c.sif.next_fd;
                    c.sif.next_fd += 1;
                    c.sif.fds.insert(fd, (e.sector, e.size, 0));
                    fd
                }
                _ => -2,
            };
            eprintln!("[fileio] open {path} -> {r}");
            r
        }
        1 => {
            let fd = word(c, 0) as i32;
            if c.sif.fds.remove(&fd).is_some() { 0 } else { -9 }
        }
        2 => {
            let (fd, buf, n) = (word(c, 0) as i32, word(c, 1), word(c, 2));
            match c.sif.fds.get(&fd).copied() {
                Some((sector, size, pos)) => {
                    let n = n.min(size.saturating_sub(pos));
                    let data = read_file(c, sector, pos, n);
                    c.bus.write_bytes(buf, &data);
                    c.sif.fds.insert(fd, (sector, size, pos + n));
                    n as i32
                }
                None => -9,
            }
        }
        3 => word(c, 2) as i32,
        4 => {
            let (fd, off, whence) = (word(c, 0) as i32, word(c, 1) as i32, word(c, 2));
            match c.sif.fds.get(&fd).copied() {
                Some((sector, size, pos)) => {
                    let base = match whence {
                        0 => 0,
                        1 => pos as i64,
                        _ => size as i64,
                    };
                    let np = (base + off as i64).clamp(0, size as i64) as u32;
                    c.sif.fds.insert(fd, (sector, size, np));
                    np as i32
                }
                None => -9,
            }
        }
        _ => {
            eprintln!("[fileio] method {method}");
            -1
        }
    };
    wr(c, ret, result as u32);
    true
}

fn read_file(c: &mut Cpu, sector: u32, pos: u32, n: u32) -> Vec<u8> {
    let Some(d) = c.sif.disc.as_mut() else {
        return vec![0; n as usize];
    };
    let first = sector + pos / SECTOR as u32;
    let skip = (pos % SECTOR as u32) as usize;
    let count = (skip as u32 + n).div_ceil(SECTOR as u32);
    let buf = d.read_vec(first, count);
    buf[skip..skip + n as usize].to_vec()
}

/// Read `count` sectors from the disc into EE memory at `dst`.
pub fn read_sectors_to_ee(c: &mut Cpu, sector: u32, count: u32, dst: u32) {
    let data = match c.sif.disc.as_mut() {
        Some(d) => d.read_vec(sector, count),
        None => vec![0; count as usize * SECTOR],
    };
    c.bus.write_bytes(dst, &data);
}

fn cd_init(c: &mut Cpu, _m: u32, _a: u32, _s: u32, ret: u32, rsize: u32) -> bool {
    if rsize >= 16 {
        wr(c, ret + 12, 0xFF);
    }
    true
}

fn bcd(v: u32) -> u8 {
    (((v / 10) << 4) | (v % 10)) as u8
}

fn cd_scmd(c: &mut Cpu, method: u32, _a: u32, _s: u32, ret: u32, rsize: u32) -> bool {
    match method {
        0x01 => {
            let secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let (s, m, h) = ((secs % 60) as u32, (secs / 60 % 60) as u32, (secs / 3600 % 24) as u32);
            wr(c, ret, 1);
            c.bus.write_bytes(ret + 4, &[0, bcd(s), bcd(m), bcd((h + 9) % 24), 0, bcd(14), bcd(10), bcd(26)]);
        }
        0x03 => wr(c, ret, 0x14),
        0x04 => wr(c, ret, 0),
        0x05 => {
            wr(c, ret, 1);
            wr(c, ret + 4, 0);
        }
        0x0C => wr(c, ret, 0x0A),
        0x27 => {
            wr(c, ret, 1);
            wr(c, ret + 4, 0);
            if rsize >= 12 {
                wr(c, ret + 8, 0);
            }
        }
        _ => {
            eprintln!("[cdvdfsv] S command {method:#x}");
            wr(c, ret, 1);
        }
    }
    true
}

fn cd_ncmd(c: &mut Cpu, method: u32, args: u32, _s: u32, ret: u32, rsize: u32) -> bool {
    let (sector, count, dst) = (rd(c, args), rd(c, args + 4), rd(c, args + 8));
    match method {
        0x01 => {
            if rsize >= 4 {
                wr(c, ret, 0);
            }
            read_sectors_to_ee(c, sector, count, dst);
        }
        0x04 => wr(c, ret, 1),
        0x05 | 0x08 => {}
        0x09 => {
            let cmd = rd(c, args + 12);
            match cmd {
                1 => {
                    c.sif.stream_pos = sector;
                    wr(c, ret, 1)
                }
                2 => {
                    let pos = c.sif.stream_pos;
                    c.sif.stream_pos += count;
                    wr(c, ret, count);
                    read_sectors_to_ee(c, pos, count, dst);
                }
                4 | 9 => {
                    c.sif.stream_pos = sector;
                    wr(c, ret, 1)
                }
                6 => wr(c, ret, sector),
                _ => wr(c, ret, 1),
            }
        }
        0x0D => {
            // Read into IOP memory.
            if rsize >= 4 {
                wr(c, ret, 0);
            }
            let data = match c.sif.disc.as_mut() {
                Some(d) => d.read_vec(sector, count),
                None => vec![0; count as usize * SECTOR],
            };
            let o = (dst & 0x1F_FFFF) as usize;
            let end = (o + data.len()).min(0x20_0000);
            c.bus.iop_ram_mut()[o..end].copy_from_slice(&data[..end - o]);
        }
        0x0E => wr(c, ret, 2),
        _ => {
            eprintln!("[cdvdfsv] N command {method:#x}");
            wr(c, ret, 1);
        }
    }
    true
}

fn cd_misc(_c: &mut Cpu, _m: u32, _a: u32, _s: u32, _r: u32, _rs: u32) -> bool {
    true
}

fn cd_search(c: &mut Cpu, _m: u32, args: u32, size: u32, ret: u32, _rs: u32) -> bool {
    let off = if size == 0x124 { 0x20 } else { 0x24 };
    let path = c.bus.read_cstr(args + off);
    let found = c.sif.disc.as_mut().and_then(|d| d.find(&path));
    if let Some(e) = &found {
        wr(c, args, e.sector);
        wr(c, args + 4, e.size);
    }
    eprintln!("[cdvdfsv] search {path} -> {}", if found.is_some() { "found" } else { "missing" });
    wr(c, ret, found.is_some() as u32);
    true
}

fn cd_ready(c: &mut Cpu, _m: u32, _a: u32, _s: u32, ret: u32, _rs: u32) -> bool {
    wr(c, ret, 2);
    true
}

/// Memory card server: report no card present.
fn mcserv(c: &mut Cpu, method: u32, _a: u32, _s: u32, ret: u32, rsize: u32) -> bool {
    if c.sif.trace {
        eprintln!("[mcserv] method {method:#x}");
    }
    if rsize >= 4 {
        // -10: no card (mcGetInfo style results are negative on absence)
        wr(c, ret, (-10i32) as u32);
    }
    true
}

/// EA's SNDDRV ('SND'): accept every request.
fn snd(c: &mut Cpu, method: u32, _a: u32, _s: u32, ret: u32, rsize: u32) -> bool {
    if c.sif.trace {
        eprintln!("[snd] method {method:#x} recv {rsize}");
    }
    if rsize > 0 {
        let z = vec![0u8; rsize as usize];
        c.bus.write_bytes(ret, &z);
    }
    true
}
