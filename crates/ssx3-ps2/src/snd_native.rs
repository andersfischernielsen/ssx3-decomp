//! Phase 3, sound: the native sound driver (`ssx3-snd`) in place of EA's IOP
//! driver (`docs/investigation/phase3-sound.md`).
//!
//! On by default (`SSX3_SND=iop` for the recompiled IOP driver). Then LIBSD.IRX / SNDDRV.IRX are not started
//! on the recompiled IOP (which keeps running the pad and memory-card
//! modules), so the game's `'SND'` RPC and SIF user command 0 reach the HLE
//! side (`sif.rs`), which hands them to [`NativeSnd`]:
//!
//! - RPC fno 0 (init) configures the driver; the first tick follows at once,
//!   as on the IOP (the init RPC itself wakes SNDDRV's main thread);
//! - user command 0 type 1 uploads bank data from IOP RAM to SPU RAM and
//!   answers with command 1 type 2 (token);
//! - every [`TICK_EE_CYCLES`] (512 frames at 48 kHz) [`service`] runs one
//!   tick on the tag buffer the EE last DMA'd to [`TAG_RX`], writes the
//!   status block to the EE and sends command 1 type 0 with `opt = TAG_RX`,
//!   which wakes the EE sound thread.

use std::sync::OnceLock;

use ssx3_snd::driver::{SndDriver, TICK_EE_CYCLES};
use ssx3_snd::proto::{InitParams, UploadCmd};

use crate::cpu::Cpu;

/// IOP RAM address the EE DMAs its tag buffer to (SNDDRV's receive area).
pub const TAG_RX: u32 = 0x0003_b3c4;
/// Largest tag buffer (`SND_InitIopCmdBuffer`).
const TAG_MAX: usize = 0x2c00;

pub struct NativeSnd {
    drv: SndDriver,
    /// EE cycle of the next tick (`None` until the init RPC).
    next_tick: Option<u64>,
    sink: ssx3_iop::audio::Sink,
    pub ticks: u64,
    /// The EE sent a tag buffer since the last tick (it answers each tick's
    /// command 1 with the next one: voice commands and the PCM it mixed).
    tag_fresh: bool,
}

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    // Native by default; SSX3_SND=iop runs the recompiled IOP sound driver.
    *ON.get_or_init(|| std::env::var("SSX3_SND").map_or(true, |v| v != "iop"))
}

fn state(c: &mut Cpu) -> &mut NativeSnd {
    m!(c).sif.native_snd.get_or_insert_with(|| {
        eprintln!("[snd] native sound driver (ssx3-snd) instead of SNDDRV on the IOP");
        Box::new(NativeSnd { drv: SndDriver::new(), next_tick: None, sink: ssx3_iop::audio::Sink::from_env(), ticks: 0, tag_fresh: false })
    })
}

/// `'SND'` RPC call. `args` is the EE address of the send data.
pub fn rpc(c: &mut Cpu, fno: u32, args: u32, size: u32) {
    if fno != 0 {
        // 1 = shutdown, 2 = IOP effect preset (not used during play).
        return;
    }
    let data = c.bus.read_bytes(args & 0x01FF_FFFF, size.max(0x14) as usize);
    let Some(p) = InitParams::parse(&data) else { return };
    if m!(c).sif.trace {
        eprintln!("[snd] init {p:?}");
    }
    let now = c.cycles;
    let s = state(c);
    s.drv.init(p);
    s.next_tick = Some(now);
    c.next_event = c.cycles;
}

/// SIF user command 0 (`packet` = the command packet bytes).
pub fn user_cmd0(c: &mut Cpu, packet: &[u8]) -> Option<u32> {
    let cmd = UploadCmd::parse(packet)?;
    let iop = std::mem::take(&mut m!(c).iop);
    state(c).drv.upload(&cmd, &iop);
    m!(c).iop = iop;
    Some(cmd.token)
}

/// Real-time mode: keep the device's queue at
/// [`crate::machine::audio_lead_frames`] (at least 60 ms), and look again
/// this often.
const RT_POLL_CYCLES: u64 = crate::machine::EE_CLOCK / 4000;

/// An EE → IOP DMA to `[dst, dst + size)` (`SifSetDma`): note a new tag
/// buffer.
pub fn note_dma(c: &mut Cpu, dst: u32, size: u32) {
    let o = dst & 0x1F_FFFF;
    if o <= TAG_RX && TAG_RX < o + size.max(1) {
        if let Some(s) = m!(c).sif.native_snd.as_mut() {
            s.tag_fresh = true;
        }
    }
}

/// Run the ticks that are due. Returns the EE cycle of the next one.
///
/// Ticks follow the EE clock (replays are exact). In real-time mode
/// ([`crate::machine::realtime`]) a guest slower than real time would starve
/// the device, so extra ticks run when its queue runs low, as the IOP runs
/// on its own clock on the console; they move the EE-clock schedule along
/// so the total stays at real time. An extra tick waits for the EE's next
/// tag buffer: running one on the last buffer again would replay its PCM
/// (the music the EE mixes) and repeat its voice commands.
pub fn service(c: &mut Cpu) -> Option<u64> {
    if !enabled() {
        return None;
    }
    let s = m!(c).sif.native_snd.as_mut()?;
    let mut due = s.next_tick?;
    while c.cycles >= due {
        run_tick(c);
        due += TICK_EE_CYCLES;
        m!(c).sif.native_snd.as_mut().unwrap().next_tick = Some(due);
    }
    if !crate::machine::realtime() {
        return Some(due);
    }
    let queued = |c: &mut Cpu| m!(c).sif.native_snd.as_ref().and_then(|s| s.sink.queued_frames());
    let fresh = |c: &mut Cpu| m!(c).sif.native_snd.as_ref().is_some_and(|s| s.tag_fresh);
    let lead = (crate::machine::audio_lead_frames() as usize).max(2880);
    if queued(c).is_some_and(|q| q < lead) && fresh(c) {
        // One tick per fresh buffer; the EE answers within its own time and
        // the next service call (every RT_POLL_CYCLES) can run another.
        run_tick(c);
        due += TICK_EE_CYCLES;
        m!(c).sif.native_snd.as_mut().unwrap().next_tick = Some(due);
    }
    Some(due.min(c.cycles + RT_POLL_CYCLES))
}

/// One sound driver tick: mix 512 frames from the tag buffer the EE last
/// sent, write the status back, signal the EE.
fn run_tick(c: &mut Cpu) {
    let iop = std::mem::take(&mut m!(c).iop);
    let tag = &iop[TAG_RX as usize..TAG_RX as usize + TAG_MAX];
    let s = m!(c).sif.native_snd.as_mut().unwrap();
    let out = s.drv.tick(tag, &iop);
    s.sink.push(&out.audio);
    s.ticks += 1;
    s.tag_fresh = false;
    let status_ee = s.drv.cfg.map_or(0, |p| p.status_ee);
    m!(c).iop = iop;
    if status_ee != 0 {
        c.bus.write_bytes(status_ee & 0x01FF_FFFF, &out.status.to_bytes());
    }
    crate::sif::send_user_cmd1(c, TAG_RX, 0);
}

/// Flush the WAV capture.
pub fn shutdown(c: &mut Cpu) {
    if let Some(s) = m!(c).sif.native_snd.as_mut() {
        s.sink.flush();
    }
}

/// Status line part.
pub fn status(c: &Cpu) -> Option<String> {
    let s = mr!(c).sif.native_snd.as_ref()?;
    Some(format!("snd native ticks {} audio {:.1}s", s.ticks, s.ticks as f64 * 512.0 / 48000.0))
}
