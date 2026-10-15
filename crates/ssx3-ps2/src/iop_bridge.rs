//! Bridge between the EE machine and the recompiled IOP (`ssx3-iop`).
//!
//! Enabled with `SSX3_IOP=1` (and only if the game registered the generated
//! IOP code). Then:
//!
//! * loading `LIBSD.IRX` / `SNDDRV.IRX` (sound) and `SIO2MAN.IRX`,
//!   `PADMAN.IRX`, `MCMAN.IRX`, `MCSERV.IRX` (controllers and memory card;
//!   `SSX3_IOP_SIO2=0` keeps these four on the HLE servers) through the
//!   LOADFILE RPC starts the real (recompiled) modules on the native IOP
//!   kernel instead of only recording the name;
//! * SIF binds/calls to RPC servers the IOP registered or will register
//!   (EA's `'SND'` 0x00534E44, padman 0x80000100/0x80000101, mcserv
//!   0x80000400) and user SIF commands with an IOP handler (EA's command 0)
//!   are queued for the IOP instead of the HLE stubs in `sif.rs`;
//! * every EE VBlank is signalled to the IOP (vblank.irx handlers: PADMAN
//!   polls the pads from there), and the host pad state
//!   (`Sif::pad_buttons`) is copied to the modelled DualShock 2;
//! * [`sync`] runs the IOP up to the EE clock / 8 at every EE event service
//!   point, then delivers what the IOP produced (EE memory writes from
//!   SIF DMA and command packets) to the EE.
//!
//! IOP RAM is shared: `Bus::iop` holds it while the EE runs (SIF DMA and the
//! HLE servers write there) and it is swapped into the [`Iop`] while IOP code
//! runs.

use std::sync::OnceLock;

use ssx3_iop::irx::{layout, module_name_of};
use ssx3_iop::sif::Incoming;
use ssx3_iop::{kernel, Iop};

use crate::cpu::Cpu;
use crate::iso::SECTOR;

const SERVER_XOR: u32 = 0xACAC_ACAC;

pub struct IopSide {
    pub iop: Box<Iop>,
    syncs: u64,
    /// RPC servers that a loaded recompiled module will register.
    expected_servers: Vec<u32>,
}

/// RPC servers registered by the recompiled modules (binds to them wait for
/// the IOP instead of being answered by the HLE servers).
const MODULE_SERVERS: &[(&str, &[u32])] = &[("PADMAN", &[0x8000_0100, 0x8000_0101]), ("MCSERV", &[0x8000_0400])];

/// Recompiled SIO2MAN/PADMAN/MCMAN/MCSERV (default on; `SSX3_IOP_SIO2=0`
/// leaves pads and memory card to the HLE servers in `pad.rs`/`sif.rs`).
pub fn sio2_enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("SSX3_IOP_SIO2").map(|v| v != "0").unwrap_or(true))
}

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        let want = std::env::var("SSX3_IOP").map(|v| v != "0").unwrap_or(true);
        if want && ssx3_iop::generated().is_none() {
            eprintln!("[iop] SSX3_IOP set but no recompiled IOP code is registered; using the HLE IOP");
            return false;
        }
        want
    })
}

fn ensure(c: &mut Cpu) {
    if m!(c).sif.iop.is_some() {
        return;
    }
    let mut iop = Iop::new();
    iop.cycles = c.cycles / 8;
    iop.spu.next_tick = iop.cycles + ssx3_iop::spu2::BATCH as u64 * ssx3_iop::spu2::CYCLES_PER_SAMPLE;
    // With native sound the IOP's SPU2 is idle and must not open a second
    // output (or WAV writer).
    iop.spu.sink = (!crate::snd_native::enabled()).then(ssx3_iop::audio::Sink::from_env);
    eprintln!("[iop] recompiled IOP enabled (the game's IRX modules run as translated code)");
    m!(c).sif.iop = Some(Box::new(IopSide { iop, syncs: 0, expected_servers: Vec::new() }));
}

/// Run `f` with the IOP owning IOP RAM.
fn with_iop<R>(c: &mut Cpu, f: impl FnOnce(&mut Iop) -> R) -> Option<R> {
    let mut side = m!(c).sif.iop.take()?;
    std::mem::swap(&mut m!(c).iop, &mut side.iop.ram);
    side.iop.ee_ram = c.bus.ram.as_ptr();
    side.iop.ee_ram_len = c.bus.ram.len();
    for (i, pad) in side.iop.sio2.pads.iter_mut().enumerate().take(2) {
        pad.buttons = m!(c).sif.pad_buttons[i];
        pad.sticks = m!(c).sif.pad_sticks[i];
    }
    let r = f(&mut side.iop);
    side.iop.ee_ram = std::ptr::null();
    std::mem::swap(&mut m!(c).iop, &mut side.iop.ram);
    flush(c, &mut side.iop);
    m!(c).sif.iop = Some(side);
    Some(r)
}

/// Deliver IOP → EE traffic.
fn flush(c: &mut Cpu, iop: &mut Iop) {
    for (a, d) in iop.out.ee_writes.drain(..) {
        c.bus.write_bytes(a & 0x01FF_FFFF, &d);
    }
    let packets: Vec<Vec<u8>> = iop.out.packets.drain(..).collect();
    for p in packets {
        crate::sif::send_raw(c, p);
    }
}

/// LOADFILE hook: start a recompiled module. Returns `(module id, start
/// result)`, or `None` to let the HLE path handle the request.
pub fn load_module(c: &mut Cpu, path: &str) -> Option<(u32, u32)> {
    if !enabled() {
        return None;
    }
    let name = module_name_of(path);
    layout::base_of(&name)?;
    if crate::snd_native::enabled() && (name == "LIBSD" || name == "SNDDRV") {
        // Sound runs natively (snd_native.rs); the HLE LOADFILE path records it.
        return None;
    }
    let sio2_module = layout::SIO2_MODULES.contains(&name.as_str());
    if sio2_module && !sio2_enabled() {
        return None;
    }
    let bytes = {
        let d = m!(c).sif.disc.as_mut()?;
        let e = d.find(path)?;
        let mut v = d.read_vec(e.sector, e.size.div_ceil(SECTOR as u32));
        v.truncate(e.size as usize);
        v
    };
    ensure(c);
    if name == "SIO2MAN" {
        let side = m!(c).sif.iop.as_mut().unwrap();
        if side.iop.sio2.card.is_none() {
            side.iop.sio2.card = ssx3_iop::memcard::MemoryCard::from_env();
        }
    }
    let r = with_iop(c, |iop| -> Result<(i32, u32), String> {
        let id = kernel::load_module(iop, &bytes, path)?;
        // Run the start thread to completion (module_start does not wait on the EE).
        for _ in 0..1000 {
            if let Some(res) = iop.k.modules[&id].result {
                return Ok((id, res));
            }
            if let Some(h) = &iop.halted {
                return Err(h.clone());
            }
            let t = iop.cycles + 10_000;
            iop.run(t);
        }
        Err("module start did not finish".into())
    })?;
    match r {
        Ok((id, res)) => {
            if let Some((_, ids)) = MODULE_SERVERS.iter().find(|(m, _)| *m == name) {
                m!(c).sif.iop.as_mut().unwrap().expected_servers.extend_from_slice(ids);
            }
            Some((id as u32 + 100, res))
        }
        Err(e) => {
            eprintln!("[iop] cannot start {name}: {e}");
            None
        }
    }
}

/// SIF command hook (`sif.rs command()`): returns true if the recompiled IOP
/// takes the command. `w` are the first 14 packet words, `data_addr` the EE
/// address of the last RPC send data, `src` the EE address of the packet.
pub fn command(c: &mut Cpu, w: &[u32], data_addr: u32, src: u32) -> bool {
    let Some(side) = m!(c).sif.iop.as_mut() else { return false };
    if side.iop.halted.is_some() {
        return false;
    }
    let cid = w[2];
    match cid {
        0x8000_0009 => {
            let server = w[8];
            if side.iop.sif.has_server(server) {
                let rend = [48, w[1], 0x8000_0008, 0, w[4], w[5], w[6], w[7], 0x8000_0009, server ^ SERVER_XOR, 0xDEAD_BEF0, 0xDEAD_CAFE];
                let p: Vec<u8> = rend.iter().flat_map(|v| v.to_le_bytes()).collect();
                crate::sif::send_raw(c, p);
                return true;
            }
            if !side.expected_servers.contains(&server) {
                return false;
            }
            // The module is loaded but its server thread has not registered
            // yet: the IOP answers when it has (or reports "no server").
            let mut words = [0u32; 14];
            words.copy_from_slice(&w[..14]);
            side.iop.sif.incoming.push_back(Incoming::Bind { words });
            if m!(c).sif.trace {
                eprintln!("[iop-bridge] bind {server:#x} queued for the IOP");
            }
            c.next_event = c.cycles;
            true
        }
        0x8000_000A => {
            let server = w[13] ^ SERVER_XOR;
            if !side.iop.sif.has_server(server) {
                return false;
            }
            let ssize = w[9] as usize;
            let mut words = [0u32; 14];
            words.copy_from_slice(&w[..14]);
            let data = if ssize > 0 { c.bus.read_bytes(data_addr & 0x01FF_FFFF, ssize) } else { Vec::new() };
            if m!(c).sif.trace {
                let ws: Vec<String> = data.chunks(4).take(12).map(|b| format!("{:08x}", u32::from_le_bytes(b.try_into().unwrap()))).collect();
                eprintln!("[iop-bridge] call {server:#x} fn {:#x} send {ssize} recv {:#x}/{}: {}", w[8], w[10], w[11], ws.join(" "));
            }
            let side = m!(c).sif.iop.as_mut().unwrap();
            side.iop.sif.incoming.push_back(Incoming::Call { words, data });
            c.next_event = c.cycles;
            true
        }
        cid if cid & 0x8000_0000 == 0 => {
            if !side.iop.sif.has_cmd_handler(cid) {
                return false;
            }
            let psize = ((w[0] & 0xFF) as usize).clamp(16, 0x70);
            let packet = c.bus.read_bytes(src & 0x01FF_FFFF, psize);
            if m!(c).sif.trace {
                let ws: Vec<String> = packet.chunks(4).map(|b| format!("{:08x}", u32::from_le_bytes(b.try_into().unwrap()))).collect();
                eprintln!("[iop-bridge] user cmd {cid:#x}: {}", ws.join(" "));
            }
            let side = m!(c).sif.iop.as_mut().unwrap();
            side.iop.sif.incoming.push_back(Incoming::UserCmd { cid, packet });
            c.next_event = c.cycles;
            true
        }
        _ => false,
    }
}

/// EE VBlank start: the IOP's vblank.irx handlers run at its next service
/// point (called from `sif::vblank`).
pub fn vblank(c: &mut Cpu) {
    if let Some(side) = m!(c).sif.iop.as_mut() {
        if !side.iop.k.vblank.is_empty() {
            side.iop.vblank_pending += 1;
        }
    }
}

/// Run the IOP up to the EE's clock (EE cycles / 8). Called from the EE event
/// service (`machine::service_events`).
pub fn sync(c: &mut Cpu) {
    let Some(side) = m!(c).sif.iop.as_mut() else { return };
    let target = c.cycles / 8;
    if side.iop.halted.is_some() || target <= side.iop.cycles && side.iop.sif.incoming.is_empty() && side.iop.vblank_pending == 0 {
        return;
    }
    side.syncs += 1;
    with_iop(c, |iop| {
        let t = target.max(iop.cycles + 1);
        iop.run(t);
    });
}

/// One-line IOP status for the front ends.
pub fn status(c: &Cpu) -> Option<String> {
    let side = mr!(c).sif.iop.as_ref()?;
    let i = &side.iop;
    let threads: Vec<String> = i
        .k
        .threads
        .values()
        .map(|t| format!("{}:{:?}{}@{:x}", t.id, t.status, if t.status == kernel::Status::Wait { format!("({:?})", t.wait) } else { String::new() }, if t.id == i.k.current { i.pc } else { t.ctx.pc }))
        .collect();
    Some(format!(
        "iop {}Mc calls {} cmds {}/{} audio {:.1}s heap {}K free | {} | {} | {}{}",
        i.cycles / 1_000_000,
        i.sif.calls,
        i.sif.user_cmds,
        i.sif.sent_cmds,
        i.spu.samples_out as f64 / 48_000.0,
        i.k.free_total() / 1024,
        threads.join(" "),
        ssx3_iop::spu2::describe(&i.spu),
        i.sio2.describe(),
        i.halted.as_ref().map(|h| format!(" HALTED: {h}")).unwrap_or_default()
    ))
}

/// Flush audio capture (WAV) at shutdown.
pub fn shutdown(c: &mut Cpu) {
    crate::snd_native::shutdown(c);
    if let Some(side) = m!(c).sif.iop.as_mut() {
        if let Some(s) = side.iop.spu.sink.as_mut() {
            s.flush();
        }
        if let Some(card) = side.iop.sio2.card.as_mut() {
            card.flush();
        }
        if !side.iop.unknown_io.is_empty() {
            eprintln!("[iop] unknown I/O: {:x?}", side.iop.unknown_io);
        }
        if !side.iop.k.missing.is_empty() {
            eprintln!("[iop] missing imports: {:?}", side.iop.k.missing);
        }
    }
}
