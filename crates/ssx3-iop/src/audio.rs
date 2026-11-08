//! Host audio: the SPU2 pushes 48 kHz stereo frames into a [`Sink`], which
//! forwards them to the output through a bounded ring buffer and,
//! optionally, to a WAV file.
//!
//! * [`set_output`] (the front end, before the game starts): who plays the
//!   ring. [`Output::Device`] (the default): the default cpal device;
//!   [`Output::Off`]: nothing (no ring); [`Output::Pull`]: the front end
//!   takes the samples with [`pull`] and feeds its own device (the browser's
//!   AudioWorklet, an Android Oboe stream, SDL, ...).
//! * `SSX3_AUDIO` overrides it: `0` off, `null` a silent device drained in
//!   real time (tests), `pull` pull, anything else the device.
//! * `SSX3_WAV=path.wav` records everything the SPU2 produces.
//! * [`suspend`] / [`resume`]: the front end's pause and audio interruptions.
//!
//! The emulator does not run in lock step with the sound card: the ring
//! holds at most ~200 ms; when it is full new samples are dropped, when it is
//! empty the device plays silence. Without an audio device the ring is
//! simply not created.

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

pub const RATE: u32 = 48_000;
const RING_FRAMES: usize = RATE as usize / 5;

type Ring = Arc<Mutex<VecDeque<i16>>>;

/// Frames the output device wanted while the ring was empty (it repeats
/// the last sample then: an audible gap).
static UNDERRUN_FRAMES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Frames the device has played as gaps so far.
pub fn underrun_frames() -> u64 {
    UNDERRUN_FRAMES.load(std::sync::atomic::Ordering::Relaxed)
}

/// Output suspended ([`suspend`]): the device plays silence and new samples
/// are dropped.
static SUSPENDED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Silence the output (a paused game, an audio interruption, a hidden app):
/// the device plays silence and drops what was queued, new samples are not
/// queued (a WAV capture still records them), and the real-time mixer stops
/// topping the queue up ([`Sink::queued_frames`] is `None`). The device
/// stays open.
pub fn suspend() {
    SUSPENDED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Undo [`suspend`]: sound resumes with the next samples the game mixes.
pub fn resume() {
    SUSPENDED.store(false, std::sync::atomic::Ordering::Relaxed);
}

/// Whether the output is suspended.
pub fn suspended() -> bool {
    SUSPENDED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Who plays the game's sound ([`set_output`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Output {
    /// The default output device (cpal). In the browser, [`Output::Pull`].
    #[default]
    Device,
    /// No output (a WAV capture still records).
    Off,
    /// The front end pulls the samples ([`pull`]): interleaved stereo i16
    /// at [`RATE`], at most ~200 ms queued (newer samples are dropped while
    /// the queue is full).
    Pull,
}

static OUTPUT: Mutex<Output> = Mutex::new(Output::Device);

/// Choose the output before the game starts (`ssx3_engine::EngineConfig::audio`).
/// `SSX3_AUDIO` overrides it.
pub fn set_output(o: Output) {
    *OUTPUT.lock().unwrap_or_else(|e| e.into_inner()) = o;
}

/// The output chosen: `SSX3_AUDIO` if set, else [`set_output`]'s.
pub fn output() -> Output {
    match ssx3_core::config::var("SSX3_AUDIO").as_deref() {
        Some("0") => Output::Off,
        Some("pull") => Output::Pull,
        Some(_) => Output::Device,
        None => *OUTPUT.lock().unwrap_or_else(|e| e.into_inner()),
    }
}

/// The ring [`pull`] drains ([`Output::Pull`]).
static PULLED: Mutex<Option<Ring>> = Mutex::new(None);

/// [`Output::Pull`]: stereo frames the front end holds beyond the ring
/// ([`report_buffered`]); negative until it reports.
static FRONT_BUFFERED: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(-1);

/// [`Output::Pull`]: how many stereo frames the front end has buffered but
/// not played yet (its own queue, a device buffer it can see). Once
/// reported, [`Sink::queued_frames`] counts the ring plus this, so the
/// real-time mixer keeps the front end's stream fed when the game catches
/// up; report again whenever it changes. Unreported, a front end that pulls
/// once per presented image is not topped up (it may drain the ring every
/// frame). [`pull_into`] reports 0 the first time it is called: a front end
/// pulling from its audio callback holds nothing beyond the ring.
pub fn report_buffered(frames: usize) {
    FRONT_BUFFERED.store(
        frames.min(i64::MAX as usize) as i64,
        std::sync::atomic::Ordering::Relaxed,
    );
}

fn front_buffered() -> Option<usize> {
    let v = FRONT_BUFFERED.load(std::sync::atomic::Ordering::Relaxed);
    (v >= 0).then_some(v as usize)
}

/// [`Output::Pull`]: append the samples produced since the last call to
/// `out` (interleaved stereo, 48 kHz). Returns the stereo frames appended.
/// Call it from the front end's audio callback or once per presented frame;
/// suspended ([`suspend`]), it drops what was queued and appends nothing.
pub fn pull(out: &mut Vec<i16>) -> usize {
    let Some(r) = PULLED.lock().unwrap_or_else(|e| e.into_inner()).clone() else {
        return 0;
    };
    let mut q = r.lock().unwrap_or_else(|e| e.into_inner());
    if suspended() {
        q.clear();
        return 0;
    }
    let n = q.len();
    out.extend(q.drain(..));
    n / 2
}

/// Up to `out.len() / 2` stereo frames into `out` (interleaved), for an
/// audio callback that fills a fixed buffer; the rest of `out` is left as
/// it is. Returns the frames written.
pub fn pull_into(out: &mut [i16]) -> usize {
    let _ = FRONT_BUFFERED.compare_exchange(
        -1,
        0,
        std::sync::atomic::Ordering::Relaxed,
        std::sync::atomic::Ordering::Relaxed,
    );
    let Some(r) = PULLED.lock().unwrap_or_else(|e| e.into_inner()).clone() else {
        return 0;
    };
    let mut q = r.lock().unwrap_or_else(|e| e.into_inner());
    if suspended() {
        q.clear();
        return 0;
    }
    let n = (out.len() & !1).min(q.len());
    for (o, v) in out.iter_mut().zip(q.drain(..n)) {
        *o = v;
    }
    n / 2
}

fn start_pull() -> Option<Ring> {
    let ring: Ring = Arc::new(Mutex::new(VecDeque::with_capacity(RING_FRAMES * 2)));
    *PULLED.lock().unwrap_or_else(|e| e.into_inner()) = Some(ring.clone());
    Some(ring)
}

pub struct Sink {
    ring: Option<Ring>,
    /// The front end drains the ring ([`Output::Pull`]).
    pulled: bool,
    wav: Option<Wav>,
}

struct Wav {
    file: File,
    bytes: u32,
    pending: Vec<u8>,
}

impl Wav {
    fn create(path: &str) -> std::io::Result<Self> {
        let mut file = File::create(path)?;
        file.write_all(&header(0))?;
        Ok(Self {
            file,
            bytes: 0,
            pending: Vec::with_capacity(1 << 16),
        })
    }
    fn push(&mut self, s: &[i16]) {
        for v in s {
            self.pending.extend_from_slice(&v.to_le_bytes());
        }
        if self.pending.len() >= 1 << 16 {
            self.flush();
        }
    }
    fn flush(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        let _ = self.file.seek(SeekFrom::End(0));
        let _ = self.file.write_all(&self.pending);
        self.bytes += self.pending.len() as u32;
        self.pending.clear();
        // Keep the header valid so the file is usable even if we never exit cleanly.
        let _ = self.file.seek(SeekFrom::Start(0));
        let _ = self.file.write_all(&header(self.bytes));
    }
}

impl Drop for Wav {
    fn drop(&mut self) {
        self.flush();
    }
}

fn header(bytes: u32) -> Vec<u8> {
    let mut h = Vec::with_capacity(44);
    h.extend_from_slice(b"RIFF");
    h.extend_from_slice(&(36 + bytes).to_le_bytes());
    h.extend_from_slice(b"WAVEfmt ");
    h.extend_from_slice(&16u32.to_le_bytes());
    h.extend_from_slice(&1u16.to_le_bytes());
    h.extend_from_slice(&2u16.to_le_bytes());
    h.extend_from_slice(&RATE.to_le_bytes());
    h.extend_from_slice(&(RATE * 4).to_le_bytes());
    h.extend_from_slice(&4u16.to_le_bytes());
    h.extend_from_slice(&16u16.to_le_bytes());
    h.extend_from_slice(b"data");
    h.extend_from_slice(&bytes.to_le_bytes());
    h
}

impl Sink {
    /// Build the sink from the environment (see module docs).
    pub fn from_env() -> Self {
        let wav = ssx3_core::config::var("SSX3_WAV").and_then(|p| match Wav::create(&p) {
            Ok(w) => {
                eprintln!("[audio] recording to {p}");
                Some(w)
            }
            Err(e) => {
                eprintln!("[audio] cannot create {p}: {e}");
                None
            }
        });
        let null = ssx3_core::config::var("SSX3_AUDIO").is_some_and(|v| v == "null");
        let out = output();
        let ring = match out {
            Output::Off => None,
            // Test device: drained in real time, plays nothing.
            Output::Device if null => start_null_device(),
            Output::Device => start_device(),
            Output::Pull => start_pull(),
        };
        Self {
            pulled: ring.is_some() && (out == Output::Pull || cfg!(target_arch = "wasm32")),
            ring,
            wav,
        }
    }

    /// Interleaved stereo frames.
    pub fn push(&mut self, s: &[i16]) {
        if let Some(w) = self.wav.as_mut() {
            w.push(s);
        }
        if suspended() {
            return;
        }
        if let Some(r) = &self.ring {
            if let Ok(mut q) = r.lock() {
                if q.len() + s.len() <= RING_FRAMES * 2 {
                    q.extend(s.iter().copied());
                }
            }
        }
    }

    /// Stereo frames waiting for the output device (`None` without one, and
    /// when the front end pulls the samples without [`report_buffered`]: it
    /// may drain them every frame; with it, the ring plus what it reported).
    pub fn queued_frames(&self) -> Option<usize> {
        if suspended() {
            return None;
        }
        let front = if self.pulled { front_buffered()? } else { 0 };
        self.ring
            .as_ref()
            .map(|r| r.lock().map_or(0, |q| q.len() / 2) + front)
    }

    pub fn flush(&mut self) {
        if let Some(w) = self.wav.as_mut() {
            w.flush();
        }
    }
}

/// `SSX3_AUDIO=null`: a ring a thread drains at 48 kHz in real time, as a
/// device would (tests of real-time audio without sound).
#[cfg(not(target_arch = "wasm32"))]
fn start_null_device() -> Option<Ring> {
    let ring: Ring = Arc::new(Mutex::new(VecDeque::with_capacity(RING_FRAMES * 2)));
    let r = ring.clone();
    std::thread::Builder::new()
        .name("ssx3-audio-null".into())
        .spawn(move || {
            let t0 = std::time::Instant::now();
            let mut taken = 0u64;
            loop {
                std::thread::sleep(std::time::Duration::from_millis(5));
                let due = (t0.elapsed().as_secs_f64() * RATE as f64) as u64;
                let n = ((due - taken) * 2) as usize;
                let mut q = r.lock().unwrap();
                if suspended() {
                    q.clear();
                    taken = due;
                    continue;
                }
                let k = n.min(q.len());
                q.drain(..k);
                UNDERRUN_FRAMES
                    .fetch_add(((n - k) / 2) as u64, std::sync::atomic::Ordering::Relaxed);
                taken = due;
            }
        })
        .ok()?;
    Some(ring)
}

#[cfg(target_arch = "wasm32")]
fn start_null_device() -> Option<Ring> {
    None
}

/// The browser: no device here. The page pulls the samples ([`pull`]) and
/// plays them with Web Audio.
#[cfg(target_arch = "wasm32")]
fn start_device() -> Option<Ring> {
    start_pull()
}

/// Open the default output device on its own thread (cpal streams are not
/// `Send` on every platform). Returns the ring it drains.
#[cfg(not(target_arch = "wasm32"))]
fn start_device() -> Option<Ring> {
    let ring: Ring = Arc::new(Mutex::new(VecDeque::with_capacity(RING_FRAMES * 2)));
    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let r2 = ring.clone();
    let spawned = std::thread::Builder::new()
        .name("ssx3-audio".into())
        .spawn(move || {
            match open_stream(r2) {
                Ok(stream) => {
                    let _ = tx.send(true);
                    // Keep the stream alive for the life of the process.
                    let _keep = stream;
                    loop {
                        std::thread::park();
                    }
                }
                Err(e) => {
                    eprintln!("[audio] no output device: {e}");
                    let _ = tx.send(false);
                }
            }
        });
    if spawned.is_err() {
        return None;
    }
    match rx.recv() {
        Ok(true) => Some(ring),
        _ => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn open_stream(ring: Ring) -> Result<cpal::Stream, String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no default output device")?;
    let supported = device.default_output_config().map_err(|e| e.to_string())?;
    let channels = supported.channels() as usize;
    let device_rate = supported.sample_rate();
    let config = cpal::StreamConfig {
        channels: supported.channels(),
        sample_rate: supported.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    };
    eprintln!(
        "[audio] output: {} ({} ch, {} Hz, {:?})",
        device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_default(),
        channels,
        device_rate,
        supported.sample_format()
    );
    // Resample 48 kHz → device rate by stepping a fractional position.
    let step = RATE as f64 / device_rate as f64;
    let mut frac = 0.0f64;
    let mut cur = [0i16; 2];
    let mut next = [0i16; 2];
    let mut fill = move |out: &mut dyn FnMut(usize, f32), frames: usize| {
        let mut q = ring.lock().unwrap();
        if suspended() {
            // Silence, and nothing stale left to play on resume.
            q.clear();
            cur = [0; 2];
            next = [0; 2];
            for i in 0..frames * channels {
                out(i, 0.0);
            }
            return;
        }
        for i in 0..frames {
            frac += step;
            while frac >= 1.0 {
                frac -= 1.0;
                cur = next;
                if q.len() >= 2 {
                    next = [q.pop_front().unwrap(), q.pop_front().unwrap()];
                } else {
                    UNDERRUN_FRAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            let t = frac as f32;
            for ch in 0..channels {
                let k = ch.min(1);
                let v = cur[k] as f32 * (1.0 - t) + next[k] as f32 * t;
                out(i * channels + ch, v / 32768.0);
            }
        }
    };
    let err = |e| eprintln!("[audio] stream error: {e}");
    let stream = match supported.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            config,
            move |data: &mut [f32], _| {
                let frames = data.len() / channels;
                fill(&mut |i, v| data[i] = v, frames);
            },
            err,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            config,
            move |data: &mut [i16], _| {
                let frames = data.len() / channels;
                fill(&mut |i, v| data[i] = (v * 32767.0) as i16, frames);
            },
            err,
            None,
        ),
        f => return Err(format!("unsupported sample format {f:?}")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_output_hands_the_samples_to_the_front_end() {
        if ssx3_core::config::var_os("SSX3_AUDIO").is_some() {
            return;
        }
        set_output(Output::Pull);
        let mut sink = Sink::from_env();
        assert_eq!(sink.queued_frames(), None);
        sink.push(&[1, 2, 3, 4]);
        let mut out = Vec::new();
        assert_eq!(pull(&mut out), 2);
        assert_eq!(out, [1, 2, 3, 4]);
        sink.push(&[5, 6, 7, 8, 9, 10]);
        let mut buf = [0i16; 4];
        assert_eq!(pull_into(&mut buf), 2);
        assert_eq!(buf, [5, 6, 7, 8]);
        assert_eq!(pull(&mut out), 1);
        assert_eq!(&out[4..], [9, 10]);
        // Pulling from a callback counts the ring; a report adds to it.
        sink.push(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(sink.queued_frames(), Some(3));
        report_buffered(100);
        assert_eq!(sink.queued_frames(), Some(103));
        suspend();
        assert_eq!(sink.queued_frames(), None);
        resume();
        set_output(Output::Off);
        assert!(Sink::from_env().ring.is_none());
    }
}
