//! Nonblocking commands to the sole cpal owner. A new stream has a fresh
//! resampler; a closed stream's queue is discarded only after drop returns.
use super::Ring;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc, Mutex,
};

static ACTIVE_ID: AtomicU64 = AtomicU64::new(0);
static CALLBACK_ID: AtomicU64 = AtomicU64::new(0);
static DESIRED: AtomicBool = AtomicBool::new(false);
static ERROR: AtomicBool = AtomicBool::new(false);
static REQUEST: AtomicU64 = AtomicU64::new(0);
static DONE: AtomicU64 = AtomicU64::new(0);
static CONTROL: Mutex<Option<mpsc::Sender<(u64, Option<bool>)>>> = Mutex::new(None);

pub fn silence() {
    DESIRED.store(false, Ordering::Release);
}
pub fn accepts() -> bool {
    DESIRED.load(Ordering::Acquire)
        && !ERROR.load(Ordering::Acquire)
        && ACTIVE_ID.load(Ordering::Acquire) == REQUEST.load(Ordering::Acquire)
}
pub fn failed() -> bool {
    ERROR.load(Ordering::Acquire)
}
pub fn settled() -> bool {
    DONE.load(Ordering::Acquire) == REQUEST.load(Ordering::Acquire)
}
pub fn stream_error() {
    silence();
    ERROR.store(true, Ordering::Release);
}
pub fn request(active: bool) {
    DESIRED.store(active, Ordering::Release);
    command(Some(active));
}
pub fn shutdown() {
    DESIRED.store(false, Ordering::Release);
    command(None);
}
fn command(active: Option<bool>) {
    ACTIVE_ID.store(0, Ordering::Release);
    ERROR.store(false, Ordering::Release);
    let control = CONTROL.lock().unwrap();
    let id = REQUEST.fetch_add(1, Ordering::AcqRel) + 1;
    if let Some(tx) = control.as_ref() {
        if tx.send((id, active)).is_err() {
            stream_error();
        }
    } else {
        // No Sink yet: let the guest run far enough to create it.
        DONE.store(id, Ordering::Release);
    }
}

pub(super) fn start() -> Option<Ring> {
    let ring = std::sync::Arc::new(Mutex::new(std::collections::VecDeque::with_capacity(
        super::RING_FRAMES * 2,
    )));
    let (tx, rx) = mpsc::channel();
    let mut control = CONTROL.lock().unwrap();
    if control.is_some() {
        stream_error();
        return None;
    }
    let q = ring.clone();
    std::thread::Builder::new()
        .name("ssx3-audio".into())
        .spawn(move || {
            let mut stream = None;
            while let Ok((id, active)) = rx.recv() {
                drop(stream.take());
                q.lock().unwrap().clear();
                let Some(active) = active else {
                    DONE.store(id, Ordering::Release);
                    break;
                };
                if active {
                    match super::open_stream(q.clone()) {
                        Ok(value) => {
                            stream = Some(value);
                            // A later pause can arrive during a slow open. Never
                            // enable an obsolete stream, even for one callback.
                            if REQUEST.load(Ordering::Acquire) == id
                                && DESIRED.load(Ordering::Acquire)
                            {
                                ACTIVE_ID.store(id, Ordering::Release);
                            }
                        }
                        Err(e) => {
                            eprintln!("[audio] reopen failed: {e}");
                            stream_error();
                        }
                    }
                }
                DONE.store(id, Ordering::Release);
            }
        })
        .ok()?;
    let id = REQUEST.fetch_add(1, Ordering::AcqRel) + 1;
    tx.send((id, Some(DESIRED.load(Ordering::Acquire)))).ok()?;
    *control = Some(tx);
    Some(ring)
}

// The callback only publishes an epoch; the event thread reports completion.
pub fn callback() {
    CALLBACK_ID.store(ACTIVE_ID.load(Ordering::Acquire), Ordering::Release);
}
pub fn callbacks_started() -> bool {
    let id = ACTIVE_ID.load(Ordering::Acquire);
    id != 0 && CALLBACK_ID.load(Ordering::Acquire) == id
}
