//! A small persistent worker pool for the rasterizer.
//!
//! A job is `units` calls `f(0)..f(units - 1)`, handed out dynamically to the
//! workers (which balances the M1's fast and slow cores). The pool runs one
//! job at a time:
//! - `spawn` starts a job in the background and returns a `Ticket`;
//!   `wait(ticket)` helps with the remaining units and returns once every
//!   unit has finished. The GS uses this to rasterize one batch while it
//!   decodes the commands for the next.
//! - `run` is the blocking form for borrowed closures.
//!
//! When the pool is busy with another caller's job (several `Gs` instances,
//! e.g. in tests), work runs on the calling thread instead.
//!
//! Idle workers spin briefly before sleeping on a condvar, because the GS
//! submits many small jobs per frame.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};

type JobFn = dyn Fn(usize) + Send + Sync;

struct Job {
    id: u64,
    f: Arc<JobFn>,
    units: usize,
    next: AtomicUsize,
    completed: AtomicUsize,
}

impl Job {
    /// Run units until none are left. Returns true if this call finished the
    /// last unit.
    fn work(&self) -> bool {
        let mut last = false;
        loop {
            let u = self.next.fetch_add(1, Ordering::AcqRel);
            if u >= self.units {
                return last;
            }
            (self.f)(u);
            last = self.completed.fetch_add(1, Ordering::AcqRel) + 1 == self.units;
        }
    }
    fn done(&self) -> bool {
        self.completed.load(Ordering::Acquire) >= self.units
    }
}

struct Shared {
    job: Mutex<Option<Arc<Job>>>,
    wake: Condvar,
    finished: Condvar,
    /// Id of the newest job; lets spinning workers notice work cheaply.
    generation: AtomicU64,
}

pub(crate) struct Pool {
    shared: Arc<Shared>,
    workers: usize,
}

/// A spawned job (see `Pool::wait`).
#[must_use]
pub(crate) struct Ticket(Option<Arc<Job>>);

/// Spin iterations before a worker sleeps (roughly 100-200 µs).
const SPIN: u32 = 20_000;

impl Pool {
    /// The process-wide pool (`available_parallelism - 1` workers; override
    /// the total thread count with `SSX3_GS_THREADS`).
    pub(crate) fn global() -> &'static Pool {
        static POOL: OnceLock<Pool> = OnceLock::new();
        POOL.get_or_init(|| {
            let n = std::env::var("SSX3_GS_THREADS")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_else(|| {
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(1)
                })
                .clamp(1, 64);
            Pool::new(n - 1)
        })
    }

    fn new(workers: usize) -> Pool {
        let shared = Arc::new(Shared {
            job: Mutex::new(None),
            wake: Condvar::new(),
            finished: Condvar::new(),
            generation: AtomicU64::new(0),
        });
        for i in 0..workers {
            let s = shared.clone();
            std::thread::Builder::new()
                .name(format!("gs-raster-{i}"))
                .spawn(move || worker(&s))
                .expect("spawn GS worker");
        }
        Pool { shared, workers }
    }

    /// Total threads that take part in a job.
    pub(crate) fn threads(&self) -> usize {
        self.workers + 1
    }

    /// Start `f(0..units)` on the workers. If the pool is busy, the job is
    /// run right here and the ticket is already complete.
    pub(crate) fn spawn(&self, units: usize, f: Arc<JobFn>) -> Ticket {
        let s = &*self.shared;
        if self.workers > 0 && units > 1 {
            let mut slot = s.job.lock().unwrap_or_else(|e| e.into_inner());
            if slot.is_none() {
                let id = s.generation.load(Ordering::Relaxed) + 1;
                let job = Arc::new(Job {
                    id,
                    f,
                    units,
                    next: AtomicUsize::new(0),
                    completed: AtomicUsize::new(0),
                });
                *slot = Some(job.clone());
                s.generation.store(id, Ordering::Release);
                drop(slot);
                s.wake.notify_all();
                return Ticket(Some(job));
            }
        }
        for u in 0..units {
            f(u);
        }
        Ticket(None)
    }

    /// Help finish the job, wait for it, and release the pool.
    pub(crate) fn wait(&self, t: Ticket) {
        let Some(job) = t.0 else { return };
        let s = &*self.shared;
        job.work();
        let mut slot = s.job.lock().unwrap_or_else(|e| e.into_inner());
        while !job.done() {
            slot = s.finished.wait(slot).unwrap_or_else(|e| e.into_inner());
        }
        if slot.as_ref().is_some_and(|j| j.id == job.id) {
            *slot = None;
        }
    }

    /// Run `f(0..units)` on the pool and the calling thread; returns when
    /// all calls have finished, so `f` may borrow.
    pub(crate) fn run(&self, units: usize, f: &(dyn Fn(usize) + Sync)) {
        if self.workers == 0 || units <= 1 {
            for u in 0..units {
                f(u);
            }
            return;
        }
        struct Borrowed(*const (dyn Fn(usize) + Sync));
        // SAFETY: only dereferenced for units of this job, all of which
        // complete before `run` returns (`wait`); later calls see no units.
        unsafe impl Send for Borrowed {}
        unsafe impl Sync for Borrowed {}
        let p: *const (dyn Fn(usize) + Sync + '_) = f;
        // SAFETY: lifetime erasure only; see above.
        let b = Borrowed(unsafe {
            std::mem::transmute::<*const (dyn Fn(usize) + Sync + '_), *const (dyn Fn(usize) + Sync)>(
                p,
            )
        });
        let job: Arc<JobFn> = Arc::new(move |u| {
            let b = &b;
            // SAFETY: see `Borrowed`.
            unsafe { (*b.0)(u) }
        });
        let t = self.spawn(units, job);
        self.wait(t);
    }
}

fn worker(s: &Shared) {
    let mut seen = s.generation.load(Ordering::Acquire);
    loop {
        // Spin for a little while, then sleep until a new job is posted.
        let mut spins = 0;
        while s.generation.load(Ordering::Acquire) == seen && spins < SPIN {
            std::hint::spin_loop();
            spins += 1;
        }
        let job = {
            let mut slot = s.job.lock().unwrap_or_else(|e| e.into_inner());
            loop {
                let gen = s.generation.load(Ordering::Acquire);
                if gen != seen {
                    seen = gen;
                    if let Some(j) = slot.as_ref().filter(|j| j.id == gen) {
                        break j.clone();
                    }
                }
                slot = s.wake.wait(slot).unwrap_or_else(|e| e.into_inner());
            }
        };
        if job.work() {
            let _guard = s.job.lock().unwrap_or_else(|e| e.into_inner());
            s.finished.notify_all();
        }
    }
}
