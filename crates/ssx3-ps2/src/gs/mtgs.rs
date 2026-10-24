//! MTGS: the GS on its own host thread (PCSX2 "MTGS" style).
//!
//! [`Mtgs`] is what the machine talks to (`Hw::gs`). The guest side (the EE
//! thread holding the baton) only records GS work — GIF data per path,
//! register and privileged register writes, VBlank, sync — into packets,
//! which a dedicated host thread replays into the real [`Gs`] in order.
//! Everything the guest can observe is either answered from guest-side state
//! or after a **sync** (wait until the GS thread has drained every queued
//! command, then read the [`Gs`] directly):
//!
//! | guest observation | answered from |
//! | --- | --- |
//! | GIF_STAT, PATH3 masking/queueing | guest side (the PATH3 queue lives here) |
//! | privileged registers PMODE..BGCOLOR, IMR, BUSDIR | guest-side shadow |
//! | CSR, SIGLBLID, `irq_pending` | guest side: SIGNAL/FINISH/LABEL are applied as GIF data is queued (the GIF tags of all paths are tracked here) |
//! | local->host FIFO (`read_local`) | sync |
//! | display, VRAM, captures, backend switches | sync ([`Mtgs::core`]) |
//!
//! The [`Gs`] receives exactly the same calls in the same order as without
//! the thread, so its output (VRAM, display, captures) is identical.
//! `SSX3_MTGS=0` runs every call synchronously on the guest thread.
//! See `docs/investigation/gs-mtgs.md`.

use std::collections::VecDeque;
#[cfg(feature = "emu-gs")]
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(feature = "emu-gs")]
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use super::gif::GifPath;
#[cfg(feature = "emu-gs")]
use super::gpu::{NativeRecord, NativeSurfaces};
#[cfg(feature = "emu-gs")]
use super::Gs;
use super::{Priv, CSR_FINISH, CSR_SIGNAL, CSR_VSINT};
use crate::Qword;

const CSR_RESET: u64 = 1 << 9;
const CSR_FIELD: u64 = 1 << 13;

/// Hand a packet to the GS thread once it holds this many qwords...
const PACKET_QWORDS: usize = 2048;
/// ...or this many commands.
const PACKET_CMDS: usize = 512;
/// Packets in flight before the guest waits for the GS thread.
const MAX_INFLIGHT: u64 = 256;

#[derive(Clone, Copy, Debug)]
enum Cmd {
    /// `Gs::gif_write(path, next len qwords of the packet's data)`.
    Gif {
        path: u8,
        len: u32,
    },
    Reg {
        addr: u8,
        value: u64,
    },
    Priv {
        offset: u32,
        value: u64,
    },
    Vblank(u32),
    Raise(u64),
    Sync,
    Path3Reset,
    /// `Gs::display_gpu`, result into the display slot.
    Display,
    /// `Gs::native_run` with the packet's next [`NativeJob`].
    Native,
    /// `Gs::set_draws_suppressed`.
    Suppress(bool),
}

/// A native run queued at a point of the GS command stream
/// ([`Mtgs::native_run`]): host wgpu passes composed with the emulated GS
/// output there (`gs::gpu::native`).
#[cfg(feature = "emu-gs")]
pub struct NativeJob {
    /// The frame/Z surfaces to borrow; `None` takes context 1's FRAME,
    /// ZBUF and SCISSOR as they are at that point of the stream.
    pub surfaces: Option<NativeSurfaces>,
    /// Passes recorded between the unswizzle and the swizzle.
    pub passes: Vec<NativeRecord>,
}

/// The latest `Cmd::Display` result: `Some(frame)` once one has run
/// (`frame` is `None` when nothing is displayed or the GS draws on the CPU).
#[cfg(feature = "emu-gs")]
type DisplaySlot = Mutex<Option<Option<super::gpu::GpuFrame>>>;

#[cfg(feature = "emu-gs")]
#[derive(Default)]
struct Packet {
    cmds: Vec<Cmd>,
    data: Vec<Qword>,
    /// Payloads of the packet's `Cmd::Native`, in order.
    natives: VecDeque<NativeJob>,
}

#[cfg(feature = "emu-gs")]
impl Packet {
    fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }

    fn clear(&mut self) {
        self.cmds.clear();
        self.data.clear();
        self.natives.clear();
    }
}

/// Run one command (`data`: its GIF payload; `job`: a `Cmd::Native`'s).
#[cfg(feature = "emu-gs")]
#[inline]
fn apply(gs: &mut Gs, c: Cmd, data: &[Qword], slot: &DisplaySlot, job: Option<NativeJob>) {
    match c {
        Cmd::Gif { path, .. } if path & 0x80 != 0 => gs.gif_write_native((path & 3) as usize, data),
        Cmd::Gif { path, .. } => gs.gif_write(path as usize, data),
        Cmd::Reg { addr, value } => gs.write_reg(addr, value),
        Cmd::Priv { offset, value } => gs.write_priv(offset, value),
        Cmd::Vblank(f) => gs.vblank(f),
        Cmd::Raise(b) => gs.raise_event(b),
        Cmd::Sync => gs.sync(),
        Cmd::Path3Reset => gs.path3_reset(),
        Cmd::Display => {
            let f = gs.display_gpu();
            *slot.lock().unwrap() = Some(f);
        }
        Cmd::Suppress(on) => gs.set_draws_suppressed(on),
        Cmd::Native => {
            let job = job.expect("native job");
            // A refused run queues nothing (counted in `GpuStats::native`).
            let _ = gs.native_run(job.surfaces, job.passes);
        }
    }
}

/// Replay a packet into the GS.
#[cfg(feature = "emu-gs")]
fn run(gs: &mut Gs, p: &mut Packet, slot: &DisplaySlot) {
    let mut pos = 0usize;
    let Packet {
        cmds,
        data,
        natives,
    } = p;
    for &c in cmds.iter() {
        let len = match c {
            Cmd::Gif { len, .. } => len as usize,
            _ => 0,
        };
        let job = if matches!(c, Cmd::Native) {
            natives.pop_front()
        } else {
            None
        };
        apply(gs, c, &data[pos..pos + len], slot, job);
        pos += len;
    }
}

#[cfg(feature = "emu-gs")]
struct Queue {
    packets: VecDeque<Packet>,
    shutdown: bool,
}

#[cfg(feature = "emu-gs")]
struct Shared {
    core: Mutex<Gs>,
    queue: Mutex<Queue>,
    /// The GS thread waits here for packets.
    work: Condvar,
    /// Packets fully processed.
    done: AtomicU64,
    /// Guest-side waiters for `done` (see `wait_done`).
    waiters: AtomicU64,
    done_lock: Mutex<()>,
    done_cv: Condvar,
    /// Processed packets, cleared, for reuse.
    free: Mutex<Vec<Packet>>,
    display: DisplaySlot,
}

#[cfg(feature = "emu-gs")]
fn worker(sh: Arc<Shared>) {
    loop {
        let mut pkt = {
            let mut q = sh.queue.lock().unwrap();
            loop {
                if let Some(p) = q.packets.pop_front() {
                    break p;
                }
                if q.shutdown {
                    return;
                }
                q = sh.work.wait(q).unwrap();
            }
        };
        {
            let mut gs = sh.core.lock().unwrap();
            run(&mut gs, &mut pkt, &sh.display);
        }
        pkt.clear();
        sh.free.lock().unwrap().push(pkt);
        sh.done.fetch_add(1, Ordering::SeqCst);
        if sh.waiters.load(Ordering::SeqCst) != 0 {
            let _g = sh.done_lock.lock().unwrap();
            sh.done_cv.notify_all();
        }
    }
}

/// Why the guest had to wait for the GS thread (`SSX3_MTGS_STATS=1`).
#[derive(Default, Debug, Clone, Copy)]
pub struct MtgsStats {
    pub packets: u64,
    pub sync_read_local: u64,
    pub sync_core: u64,
    pub sync_backpressure: u64,
    /// Syncs that actually had to wait (work was outstanding).
    pub waits: u64,
    pub wait_ns: u64,
}

#[cfg(feature = "emu-gs")]
struct Threaded {
    shared: Arc<Shared>,
    thread: Option<std::thread::JoinHandle<()>>,
    /// Packets handed to the GS thread.
    sent: u64,
    cur: Packet,
}

#[cfg(feature = "emu-gs")]
enum Mode {
    /// Every call runs on the calling thread (`SSX3_MTGS=0`).
    Direct(Arc<Mutex<Gs>>, DisplaySlot),
    Threaded(Threaded),
}

/// The GS as the machine sees it: GS work goes to a GS host thread; PATH3
/// arbitration and the guest-visible register shadows stay here.
pub struct Mtgs {
    #[cfg(feature = "emu-gs")]
    mode: Mode,
    /// Shadow of the privileged registers the guest side writes.
    pr: Priv,
    /// CSR event bits and FIELD, and SIGLBLID: VBlank, `raise_event` and
    /// acknowledges are applied here, and SIGNAL/FINISH/LABEL as the GIF
    /// data is queued (their effect only depends on the data), so these
    /// always equal the GS's own once it has run the queue.
    csr: u64,
    siglblid: u64,
    /// PATH3 DMA data the GIF has not accepted (see `Gs::path3_dma`).
    p3_queue: VecDeque<Qword>,
    p3_mask_vif: bool,
    p3_mask_reg: bool,
    p3_mid: bool,
    /// GIF tag state of every path (mirrors `Gs::paths`): PATH3 packet
    /// boundaries and the A+D writes to SIGNAL/FINISH/LABEL.
    tags: [GifPath; 4],
    scratch: Vec<Qword>,
    pub stats: MtgsStats,
    /// No GS core: the native renderer draws everything (`detach`).
    detached: bool,
    /// Debug (`SSX3_RENDER_ROUNDTRIP=N`): queue an empty native run (a
    /// VRAM -> wgpu targets -> VRAM round trip of context 1's frame and Z)
    /// before every N-th GIF transfer; 0 = off.
    roundtrip_every: u64,
    gif_writes: u64,
    vblanks: u64,
}

/// `SSX3_RENDER_ROUNDTRIP=N` (see `Mtgs::roundtrip_every`); `1` is every
/// GIF transfer.
fn roundtrip_every() -> u64 {
    std::env::var("SSX3_RENDER_ROUNDTRIP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn mtgs_enabled() -> bool {
    // No threads in the browser.
    if cfg!(target_arch = "wasm32") {
        return false;
    }
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_MTGS").map_or(true, |v| v != "0"))
}

/// Check the guest-side mirrors against the GS at every sync (always in
/// tests; `SSX3_MTGS_CHECK=1` otherwise).
fn mtgs_check() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    cfg!(test) || *E.get_or_init(|| std::env::var_os("SSX3_MTGS_CHECK").is_some())
}

impl Mtgs {
    /// Wrap `gs`; threaded unless `SSX3_MTGS=0`.
    #[cfg(feature = "emu-gs")]
    pub fn new(gs: Gs) -> Self {
        Self::with_thread(gs, mtgs_enabled())
    }

    #[cfg(feature = "emu-gs")]
    pub fn with_thread(gs: Gs, threaded: bool) -> Self {
        let pr = gs.pr;
        let csr = gs.pr.csr & (0x1F | CSR_FIELD);
        let siglblid = gs.pr.siglblid;
        let tags = gs.paths;
        let mode = if threaded {
            let shared = Arc::new(Shared {
                core: Mutex::new(gs),
                queue: Mutex::new(Queue {
                    packets: VecDeque::new(),
                    shutdown: false,
                }),
                work: Condvar::new(),
                done: AtomicU64::new(0),
                waiters: AtomicU64::new(0),
                done_lock: Mutex::new(()),
                done_cv: Condvar::new(),
                free: Mutex::new(Vec::new()),
                display: Mutex::new(None),
            });
            let sh = shared.clone();
            let thread = std::thread::Builder::new()
                .name("gs".into())
                .spawn(move || worker(sh))
                .expect("spawn GS thread");
            Mode::Threaded(Threaded {
                shared,
                thread: Some(thread),
                sent: 0,
                cur: Packet::default(),
            })
        } else {
            Mode::Direct(Arc::new(Mutex::new(gs)), Mutex::new(None))
        };
        Mtgs {
            mode,
            pr,
            csr,
            siglblid,
            p3_queue: VecDeque::new(),
            p3_mask_vif: false,
            p3_mask_reg: false,
            p3_mid: false,
            tags,
            scratch: Vec::new(),
            stats: MtgsStats::default(),
            roundtrip_every: roundtrip_every(),
            detached: false,
            gif_writes: 0,
            vblanks: 0,
        }
    }

    #[cfg(feature = "emu-gs")]
    pub fn threaded(&self) -> bool {
        matches!(self.mode, Mode::Threaded(_))
    }

    // ------------------------------------------------------------ plumbing

    /// (No GS core: nothing to submit or wait for.)
    #[cfg(not(feature = "emu-gs"))]
    fn kick(&mut self) {}
    #[cfg(not(feature = "emu-gs"))]
    fn drain(&mut self) {}

    /// The guest-visible GS without a core (builds without `emu-gs`).
    #[cfg(not(feature = "emu-gs"))]
    pub fn new() -> Self {
        let pr = Priv {
            csr: super::CSR_FIXED,
            imr: 0x7F00,
            ..Priv::default()
        };
        Mtgs {
            pr,
            csr: 0,
            siglblid: 0,
            p3_queue: VecDeque::new(),
            p3_mask_vif: false,
            p3_mask_reg: false,
            p3_mid: false,
            tags: [GifPath::default(); 4],
            scratch: Vec::new(),
            stats: MtgsStats::default(),
            roundtrip_every: 0,
            detached: true,
            gif_writes: 0,
            vblanks: 0,
        }
    }

    /// Queue (or, in direct mode, run) one command; `data` is the GIF payload.
    #[inline]
    fn exec(&mut self, cmd: Cmd, data: &[Qword]) {
        if self.detached {
            return;
        }
        #[cfg(not(feature = "emu-gs"))]
        let _ = (cmd, data);
        #[cfg(feature = "emu-gs")]
        match &mut self.mode {
            Mode::Direct(core, slot) => {
                apply(&mut core.lock().unwrap(), cmd, data, slot, None);
            }
            Mode::Threaded(t) => {
                t.cur.cmds.push(cmd);
                t.cur.data.extend_from_slice(data);
                if t.cur.data.len() >= PACKET_QWORDS || t.cur.cmds.len() >= PACKET_CMDS {
                    self.kick();
                }
            }
        }
    }

    /// Queue a native run at this point of the GS command stream (after
    /// everything queued so far, before everything queued later). It runs
    /// on the GS thread; see `Gs::native_run`.
    #[cfg(feature = "emu-gs")]
    pub fn native_run(&mut self, job: NativeJob) {
        if self.detached {
            return;
        }
        match &mut self.mode {
            Mode::Direct(core, slot) => {
                apply(&mut core.lock().unwrap(), Cmd::Native, &[], slot, Some(job));
            }
            Mode::Threaded(t) => {
                t.cur.natives.push_back(job);
                t.cur.cmds.push(Cmd::Native);
                if t.cur.cmds.len() >= PACKET_CMDS {
                    self.kick();
                }
            }
        }
    }

    /// Debug `SSX3_RENDER_ROUNDTRIP`: native-run statistics so far.
    #[cfg(feature = "emu-gs")]
    fn roundtrip_report(&self) {
        if let Some(st) = self.lock().gpu_stats() {
            let n = st.native;
            eprintln!(
                "[gs-native] round trips {} (refused {}), {:.1} Mpx, GPU {:.1} ms{}",
                n.runs,
                n.refused,
                n.pixels as f64 / 1e6,
                n.gpu_ns as f64 / 1e6,
                if n.runs > 0 && n.gpu_ns > 0 {
                    format!(" ({:.1} us/run)", n.gpu_ns as f64 / 1e3 / n.runs as f64)
                } else {
                    String::new()
                }
            );
        }
    }

    /// Hand the current packet to the GS thread.
    #[cfg(feature = "emu-gs")]
    fn kick(&mut self) {
        let Mode::Threaded(t) = &mut self.mode else {
            return;
        };
        if t.cur.is_empty() {
            return;
        }
        let next = t.shared.free.lock().unwrap().pop().unwrap_or_default();
        let pkt = std::mem::replace(&mut t.cur, next);
        {
            let mut q = t.shared.queue.lock().unwrap();
            q.packets.push_back(pkt);
        }
        t.shared.work.notify_one();
        t.sent += 1;
        self.stats.packets += 1;
        // Bound the work in flight.
        if t.sent - t.shared.done.load(Ordering::Acquire) > MAX_INFLIGHT {
            self.stats.sync_backpressure += 1;
            let target = t.sent - MAX_INFLIGHT / 2;
            let shared = t.shared.clone();
            Self::wait_done(&shared, target, &mut self.stats);
        }
    }

    #[cfg(feature = "emu-gs")]
    fn wait_done(sh: &Shared, target: u64, stats: &mut MtgsStats) {
        if sh.done.load(Ordering::Acquire) >= target {
            return;
        }
        let t0 = web_time::Instant::now();
        stats.waits += 1;
        for _ in 0..4000 {
            if sh.done.load(Ordering::Acquire) >= target {
                stats.wait_ns += t0.elapsed().as_nanos() as u64;
                return;
            }
            std::hint::spin_loop();
        }
        let mut g = sh.done_lock.lock().unwrap();
        sh.waiters.fetch_add(1, Ordering::SeqCst);
        while sh.done.load(Ordering::SeqCst) < target {
            g = sh.done_cv.wait(g).unwrap();
        }
        sh.waiters.fetch_sub(1, Ordering::SeqCst);
        drop(g);
        stats.wait_ns += t0.elapsed().as_nanos() as u64;
    }

    /// Wait until the GS thread has run every queued command.
    #[cfg(feature = "emu-gs")]
    fn drain(&mut self) {
        self.kick();
        if let Mode::Threaded(t) = &self.mode {
            let (shared, sent) = (t.shared.clone(), t.sent);
            Self::wait_done(&shared, sent, &mut self.stats);
        }
    }

    #[cfg(feature = "emu-gs")]
    fn lock(&self) -> MutexGuard<'_, Gs> {
        match &self.mode {
            Mode::Direct(core, _) => core.lock().unwrap(),
            Mode::Threaded(t) => t.shared.core.lock().unwrap(),
        }
    }

    /// Drain the queue and lock the GS for direct access.
    #[cfg(feature = "emu-gs")]
    fn synced(&mut self) -> MutexGuard<'_, Gs> {
        self.drain();
        let gs = self.lock();
        if mtgs_check() {
            for p in 0..4 {
                assert!(
                    gs.paths[p].same(&self.tags[p]),
                    "MTGS GIF path {p} tag mirror diverged"
                );
            }
            assert_eq!(
                gs.pr.csr & (0x1F | CSR_FIELD),
                self.csr,
                "MTGS CSR mirror diverged"
            );
            assert_eq!(
                gs.pr.siglblid, self.siglblid,
                "MTGS SIGLBLID mirror diverged"
            );
        }
        gs
    }

    /// Exclusive access to the GS, after every queued command has run
    /// (display, VRAM, captures, statistics, backend switches).
    /// Native rendering: stop feeding the GS core. The guest-visible GS
    /// (CSR, privileged registers, SIGNAL/FINISH/LABEL, PATH3 arbitration,
    /// GIF_STAT) stays here, on the guest side; drawing, VRAM and the
    /// display are the native renderer's.
    pub fn detach(&mut self) {
        self.sync();
        self.drain();
        self.detached = true;
        // No GS work follows: stop the GS thread and keep the core here
        // (debug readbacks only).
        #[cfg(feature = "emu-gs")]
        if let Mode::Threaded(t) = &mut self.mode {
            t.shared.queue.lock().unwrap().shutdown = true;
            t.shared.work.notify_all();
            if let Some(h) = t.thread.take() {
                let _ = h.join();
            }
            let core = std::mem::take(&mut *t.shared.core.lock().unwrap());
            self.mode = Mode::Direct(Arc::new(Mutex::new(core)), Mutex::new(None));
        }
    }

    pub fn detached(&self) -> bool {
        self.detached
    }

    #[cfg(feature = "emu-gs")]
    pub fn core(&mut self) -> MutexGuard<'_, Gs> {
        self.stats.sync_core += 1;
        self.synced()
    }

    /// Primitives drawn up to the last sync point (`run_frame` ends with
    /// one), without forcing a new one.
    pub fn prims_drawn(&self) -> u64 {
        #[cfg(not(feature = "emu-gs"))]
        return 0;
        #[cfg(feature = "emu-gs")]
        if let Mode::Threaded(t) = &self.mode {
            let mut st = MtgsStats::default();
            let sent = t.sent;
            Self::wait_done(&t.shared, sent, &mut st);
        }
        #[cfg(feature = "emu-gs")]
        self.lock().prims_drawn
    }

    /// Queue `Gs::display_gpu` at this point of the command stream (the
    /// windowed front end, after `run_frame`). Returns a ticket for
    /// [`Mtgs::take_display`].
    #[cfg(feature = "emu-gs")]
    pub fn request_display(&mut self) -> u64 {
        if self.detached {
            return 0;
        }
        self.exec(Cmd::Display, &[]);
        self.kick();
        match &self.mode {
            Mode::Threaded(t) => t.sent,
            Mode::Direct(..) => 0,
        }
    }

    /// Wait until the display requested with `ticket` has run, then take
    /// the newest display result (possibly from a later request).
    /// `None`: no request has run since the last take; `Some(None)`: the GS
    /// has no GPU frame (nothing displayed, or the CPU backend), so use
    /// `core().display()`.
    #[cfg(feature = "emu-gs")]
    pub fn take_display(&mut self, ticket: u64) -> Option<Option<super::gpu::GpuFrame>> {
        if self.detached {
            return None;
        }
        match &self.mode {
            Mode::Threaded(t) => {
                let shared = t.shared.clone();
                Self::wait_done(&shared, ticket, &mut self.stats);
                let f = shared.display.lock().unwrap().take();
                f
            }
            Mode::Direct(_, slot) => slot.lock().unwrap().take(),
        }
    }

    // ------------------------------------------------------------ privileged

    pub fn write_priv(&mut self, offset: u32, value: u64) {
        let p = &mut self.pr;
        match offset & 0x1FF0 {
            0x0000 => p.pmode = value,
            0x0010 => p.smode1 = value,
            0x0020 => p.smode2 = value,
            0x0030 => p.srfsh = value,
            0x0040 => p.synch1 = value,
            0x0050 => p.synch2 = value,
            0x0060 => p.syncv = value,
            0x0070 => p.dispfb[0] = value,
            0x0080 => p.display[0] = value,
            0x0090 => p.dispfb[1] = value,
            0x00A0 => p.display[1] = value,
            0x00B0 => p.extbuf = value,
            0x00C0 => p.extdata = value,
            0x00D0 => p.extwrite = value,
            0x00E0 => p.bgcolor = value,
            0x1000 => {
                self.csr &= !(value & 0x1F);
                if value & CSR_RESET != 0 {
                    self.csr = 0;
                    self.tags = [GifPath::default(); 4];
                }
            }
            0x1010 => p.imr = value & 0x7F00,
            0x1040 => p.busdir = value,
            0x1080 => self.siglblid = value,
            _ => {}
        }
        self.exec(Cmd::Priv { offset, value }, &[]);
    }

    pub fn read_priv(&mut self, offset: u32) -> u64 {
        let p = &self.pr;
        match offset & 0x1FF0 {
            0x0000 => p.pmode,
            0x0010 => p.smode1,
            0x0020 => p.smode2,
            0x0030 => p.srfsh,
            0x0040 => p.synch1,
            0x0050 => p.synch2,
            0x0060 => p.syncv,
            0x0070 => p.dispfb[0],
            0x0080 => p.display[0],
            0x0090 => p.dispfb[1],
            0x00A0 => p.display[1],
            0x00B0 => p.extbuf,
            0x00C0 => p.extdata,
            0x00D0 => p.extwrite,
            0x00E0 => p.bgcolor,
            0x1010 => p.imr,
            0x1040 => p.busdir,
            0x1000 => (self.csr & (0x1F | CSR_FIELD)) | super::CSR_FIXED,
            0x1080 => self.siglblid,
            _ => 0,
        }
    }

    /// A 32-bit store into a 64-bit privileged register (`hi`: the upper
    /// half), merged with the register's current value like the bus does.
    pub fn write_priv32(&mut self, offset: u32, hi: bool, v: u32) {
        let cur = self.read_priv(offset);
        let nv = if hi {
            (cur & 0xffff_ffff) | (v as u64) << 32
        } else {
            (cur & !0xffff_ffff) | v as u64
        };
        self.write_priv(offset, nv);
    }

    pub fn irq_pending(&mut self) -> bool {
        let masked = (self.pr.imr >> 8) & 0x1F;
        self.csr & 0x1F & !masked != 0
    }

    pub fn vblank(&mut self, field: u32) {
        self.csr |= CSR_VSINT;
        if field & 1 != 0 {
            self.csr |= CSR_FIELD;
        } else {
            self.csr &= !CSR_FIELD;
        }
        self.exec(Cmd::Vblank(field), &[]);
        self.kick();
        #[cfg(feature = "emu-gs")]
        if self.roundtrip_every != 0 {
            self.vblanks += 1;
            if self.vblanks % 1200 == 0 {
                self.roundtrip_report();
            }
        }
    }

    pub fn raise_event(&mut self, bit: u64) {
        self.csr |= bit & 0x1F;
        self.exec(Cmd::Raise(bit), &[]);
    }

    /// Submit queued drawing (the end of every emulated frame). Does not
    /// wait for the GS thread.
    pub fn sync(&mut self) {
        self.exec(Cmd::Sync, &[]);
        self.kick();
    }

    // ------------------------------------------------------------ GIF

    pub fn gif_write(&mut self, path: usize, data: &[Qword]) {
        let path = path.min(3);
        let (csr, sig) = (&mut self.csr, &mut self.siglblid);
        self.tags[path].scan(data, |addr, value| ad_side_effects(csr, sig, addr, value));
        #[cfg(feature = "emu-gs")]
        if self.roundtrip_every != 0 {
            self.gif_writes += 1;
            if self.gif_writes % self.roundtrip_every == 0 {
                self.native_run(NativeJob {
                    surfaces: None,
                    passes: Vec::new(),
                });
            }
        }
        self.exec(
            Cmd::Gif {
                path: path as u8,
                len: data.len() as u32,
            },
            data,
        );
    }

    /// A PATH1 (XGKICK) packet of the type-3 VU1 program (d159716b): the
    /// same as `gif_write(1, data)`, except that with
    /// `SSX3_RENDER_NATIVE=type3` the GS draws its triangles with the
    /// hardware rasterizer (`Gs::gif_write_type3`).
    pub fn gif_write_type3(&mut self, data: &[Qword]) {
        self.gif_write_native(1, data);
    }

    /// `gif_write(path, data)` with the packet's primitives drawn natively
    /// where possible (`Gs::gif_write_native`).
    pub fn gif_write_native(&mut self, path: usize, data: &[Qword]) {
        let path = path.min(3);
        let (csr, sig) = (&mut self.csr, &mut self.siglblid);
        self.tags[path].scan(data, |addr, value| ad_side_effects(csr, sig, addr, value));
        self.exec(
            Cmd::Gif {
                path: 0x80 | path as u8,
                len: data.len() as u32,
            },
            data,
        );
    }

    /// Stop (or resume) rasterizing primitives; everything else (state,
    /// transfers, events) runs as before. For the native renderer, which
    /// draws the frames itself.
    pub fn set_draws_suppressed(&mut self, on: bool) {
        self.exec(Cmd::Suppress(on), &[]);
    }

    pub fn write_reg(&mut self, addr: u8, value: u64) {
        ad_side_effects(&mut self.csr, &mut self.siglblid, addr, value);
        self.exec(Cmd::Reg { addr, value }, &[]);
    }

    /// Pop up to `max` quadwords of a pending local->host transfer.
    pub fn read_local(&mut self, max: usize) -> Vec<Qword> {
        if self.detached || cfg!(not(feature = "emu-gs")) {
            // No GS behind: nothing to read back (the game tolerates short
            // reads; its one readback, the light query, is native).
            return Vec::new();
        }
        #[cfg(not(feature = "emu-gs"))]
        {
            let _ = max;
            return Vec::new();
        }
        #[cfg(feature = "emu-gs")]
        {
            self.stats.sync_read_local += 1;
            self.synced().read_local(max)
        }
    }

    // ------------------------------------------------------------ PATH3
    // The same arbitration as `Gs::path3_dma` and friends (see there), kept
    // on the guest side because GIF_STAT shows it.

    /// PATH3 GIF DMA data into the GS (natively drawn with
    /// `SSX3_RENDER_NATIVE=post`).
    fn gif_write_p3(&mut self, data: &[Qword]) {
        if super::render_native_post() {
            self.gif_write_native(3, data);
        } else {
            self.gif_write(3, data);
        }
    }

    pub fn path3_dma(&mut self, data: &[Qword]) {
        if !super::path3_mask_enabled() {
            self.gif_write_p3(data);
            return;
        }
        self.p3_queue.extend(data.iter().copied());
        self.pump_path3();
    }

    pub fn set_path3_mask(&mut self, masked: bool) {
        self.p3_mask_vif = masked;
        if !masked {
            self.pump_path3_packets(1);
        }
    }

    pub fn path3_flow(&mut self) {
        if !self.p3_queue.is_empty() {
            self.pump_path3();
        }
    }

    pub fn set_path3_mask_reg(&mut self, masked: bool) {
        self.p3_mask_reg = masked;
        self.pump_path3();
    }

    pub fn path3_reset(&mut self) {
        self.p3_queue.clear();
        self.p3_mid = false;
        self.tags[3] = GifPath::default();
        self.exec(Cmd::Path3Reset, &[]);
    }

    pub fn path3_pending(&self) -> usize {
        self.p3_queue.len()
    }

    pub fn gif_stat(&self) -> u32 {
        let q = self.p3_queue.len().min(16) as u32;
        (self.p3_mask_reg as u32) | (self.p3_mask_vif as u32) << 1 | ((q > 0) as u32) << 6 | q << 24
    }

    fn pump_path3(&mut self) {
        self.pump_path3_packets(usize::MAX);
    }

    fn pump_path3_packets(&mut self, max: usize) {
        let mut packets = 0;
        while !self.p3_queue.is_empty() {
            if !self.p3_mid && (self.p3_mask_vif || self.p3_mask_reg || packets >= max) {
                return;
            }
            let (mut left, mut eop) = self.tags[3].qwords_left();
            let mut n = 0;
            let mut mid = self.p3_mid;
            for &q in self.p3_queue.iter() {
                n += 1;
                if left == 0 {
                    (left, eop) = GifPath::tag_qwords(q);
                } else {
                    left -= 1;
                }
                mid = left != 0 || !eop;
                if !mid {
                    break;
                }
            }
            let mut chunk = std::mem::take(&mut self.scratch);
            chunk.clear();
            chunk.extend(self.p3_queue.drain(..n));
            if !self.p3_mid {
                packets += 1;
            }
            self.gif_write_p3(&chunk);
            self.scratch = chunk;
            self.p3_mid = mid;
        }
    }
}

/// What a GS register write does to CSR and SIGLBLID (as
/// `Gs::write_state_reg`): SIGNAL, FINISH and LABEL.
#[inline]
fn ad_side_effects(csr: &mut u64, siglblid: &mut u64, addr: u8, value: u64) {
    match addr {
        0x60 => {
            let mask = value >> 32;
            let id = value & 0xFFFF_FFFF;
            let lo = (*siglblid & 0xFFFF_FFFF & !mask) | (id & mask);
            *siglblid = (*siglblid & !0xFFFF_FFFF) | lo;
            *csr |= CSR_SIGNAL;
        }
        0x61 => *csr |= CSR_FINISH,
        0x62 => {
            let mask = value >> 32;
            let id = value & 0xFFFF_FFFF;
            let hi = ((*siglblid >> 32) & !mask) | (id & mask);
            *siglblid = (*siglblid & 0xFFFF_FFFF) | (hi << 32);
        }
        _ => {}
    }
}

impl Drop for Mtgs {
    fn drop(&mut self) {
        if std::env::var_os("SSX3_MTGS_STATS").is_some() {
            eprintln!("[mtgs] {:?}", self.stats);
        }
        #[cfg(feature = "emu-gs")]
        {
            if super::render_native_type3() {
                self.drain();
                if let Some(st) = self.lock().gpu_stats() {
                    eprintln!("{}", st.n3.report(&st.native));
                }
                if super::gpu::native3_diag() {
                    eprint!("{}", self.lock().n3_diag_report());
                }
            }
            if self.roundtrip_every != 0 {
                self.drain();
                self.roundtrip_report();
            }
            if std::env::var_os("SSX3_GS_WAITS").is_some() {
                self.drain();
                if let Some(st) = self.lock().gpu_stats() {
                    eprintln!(
                    "[gs-waits] {} readbacks: waiting for the submit thread {:.1} ms, for the GPU {:.1} ms; {} submits",
                    st.readback_count,
                    st.readback_drain_ns as f64 / 1e6,
                    st.readback_gpu_ns as f64 / 1e6,
                    st.submits
                );
                }
            }
            if std::env::var_os("SSX3_GS_PROFILE").is_some() {
                self.drain();
                if let Some(st) = self.lock().gpu_stats() {
                    eprintln!(
                    "[gs-prof] GPU time: compute raster {:.1} ms, native runs {:.1} ms; {} submits",
                    st.gpu_raster_ns as f64 / 1e6,
                    st.native.gpu_ns as f64 / 1e6,
                    st.submits
                );
                }
            }
            if let Mode::Threaded(t) = &mut self.mode {
                t.shared.queue.lock().unwrap().shutdown = true;
                t.shared.work.notify_all();
                if let Some(h) = t.thread.take() {
                    let _ = h.join();
                }
            }
        }
    }
}
