//! Host audio: the SPU2 pushes 48 kHz stereo frames into a [`Sink`], which
//! forwards them to a cpal output stream through a bounded ring buffer and,
//! optionally, to a WAV file.
//!
//! * `SSX3_AUDIO=0` disables the output device (headless runs default to off).
//! * `SSX3_WAV=path.wav` records everything the SPU2 produces.
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

pub struct Sink {
    ring: Option<Ring>,
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
        let wav = std::env::var("SSX3_WAV")
            .ok()
            .and_then(|p| match Wav::create(&p) {
                Ok(w) => {
                    eprintln!("[audio] recording to {p}");
                    Some(w)
                }
                Err(e) => {
                    eprintln!("[audio] cannot create {p}: {e}");
                    None
                }
            });
        let mode = std::env::var("SSX3_AUDIO").unwrap_or_default();
        let ring = match mode.as_str() {
            "0" => None,
            // Test device: drained in real time, plays nothing.
            "null" => start_null_device(),
            _ => start_device(),
        };
        Self { ring, wav }
    }

    /// Interleaved stereo frames.
    pub fn push(&mut self, s: &[i16]) {
        if let Some(w) = self.wav.as_mut() {
            w.push(s);
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
    /// in the browser, where the page drains the ring every frame).
    pub fn queued_frames(&self) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            return None;
        }
        self.ring
            .as_ref()
            .map(|r| r.lock().map_or(0, |q| q.len() / 2))
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

/// The browser: no device here. The ring is drained by the page ([`take`]),
/// which plays it with Web Audio.
#[cfg(target_arch = "wasm32")]
fn start_device() -> Option<Ring> {
    let ring: Ring = Arc::new(Mutex::new(VecDeque::with_capacity(RING_FRAMES * 2)));
    *PAGE.lock().unwrap() = Some(ring.clone());
    Some(ring)
}

#[cfg(target_arch = "wasm32")]
static PAGE: Mutex<Option<Ring>> = Mutex::new(None);

/// The browser: the samples produced since the last call (interleaved
/// stereo, 48 kHz).
#[cfg(target_arch = "wasm32")]
pub fn take(out: &mut Vec<i16>) {
    if let Some(r) = PAGE.lock().unwrap().as_ref() {
        out.extend(r.lock().unwrap().drain(..));
    }
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
