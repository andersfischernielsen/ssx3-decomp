//! The submit thread: finishes and submits the GS command buffers so the GS
//! thread can record the next one meanwhile.
//!
//! wgpu does most of a command buffer's work (validation, resource tracking,
//! the Metal/Vulkan encoding) in `CommandEncoder::finish`, which was about a
//! third of the GS thread with native rendering. Jobs are submitted strictly
//! in order, each after its own buffer uploads (`write_buffer` is staged for
//! the next submission on the queue, so the uploads travel with the job).
//!
//! Anything on the GS thread that maps a buffer, polls for GPU completion or
//! hands a buffer to another submitter (the presenter) first calls
//! [`Submitter::drain`]. `SSX3_GS_SUBMIT_THREAD=0` submits on the GS thread.

use std::collections::VecDeque;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Condvar, Mutex};

/// One command buffer and the uploads it reads.
pub(super) struct Job {
    pub enc: wgpu::CommandEncoder,
    /// (buffer, words) written at offset 0 before the submit.
    pub data: Option<(wgpu::Buffer, Vec<u32>)>,
    pub params: Option<(wgpu::Buffer, Vec<u8>)>,
    /// Run right after the submit (with its index): maps a readback.
    pub after: Option<AfterSubmit>,
}

/// A hook run with a job's submission index once it is submitted.
pub(super) type AfterSubmit = Box<dyn FnOnce(&wgpu::SubmissionIndex) + Send>;

struct Done {
    count: Mutex<u64>,
    cv: Condvar,
    /// Upload vectors handed back for reuse.
    spare: Mutex<Vec<Vec<u32>>>,
}

pub(super) struct Submitter {
    tx: Option<SyncSender<Job>>,
    done: Arc<Done>,
    sent: u64,
    thread: Option<std::thread::JoinHandle<()>>,
}

/// Submissions the GPU may lag behind (the submit thread waits beyond).
const MAX_INFLIGHT: usize = 3;
/// Jobs queued for the submit thread before `submit` blocks.
const MAX_QUEUED: usize = 2;

impl Submitter {
    /// `None` when `SSX3_GS_SUBMIT_THREAD=0`.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Option<Submitter> {
        if std::env::var("SSX3_GS_SUBMIT_THREAD").is_ok_and(|v| v == "0") {
            return None;
        }
        let (tx, rx) = sync_channel::<Job>(MAX_QUEUED);
        let done = Arc::new(Done { count: Mutex::new(0), cv: Condvar::new(), spare: Mutex::new(Vec::new()) });
        let (dev, q, d) = (device.clone(), queue.clone(), done.clone());
        let thread = std::thread::Builder::new()
            .name("gs-submit".into())
            .spawn(move || run(rx, dev, q, d))
            .expect("gs submit thread");
        Some(Submitter { tx: Some(tx), done, sent: 0, thread: Some(thread) })
    }

    pub fn submit(&mut self, job: Job) {
        self.sent += 1;
        self.tx.as_ref().expect("submit thread").send(job).expect("gs submit thread gone");
    }

    /// Wait until every job handed over so far is submitted.
    pub fn drain(&self) {
        let mut c = self.done.count.lock().unwrap();
        while *c < self.sent {
            c = self.done.cv.wait(c).unwrap();
        }
    }

    /// A vector for the next job's uploads (reused).
    pub fn spare(&self) -> Vec<u32> {
        self.done.spare.lock().unwrap().pop().unwrap_or_default()
    }
}

impl Drop for Submitter {
    fn drop(&mut self) {
        self.tx = None;
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn run(rx: Receiver<Job>, dev: wgpu::Device, q: wgpu::Queue, done: Arc<Done>) {
    let mut inflight: VecDeque<wgpu::SubmissionIndex> = VecDeque::new();
    // SSX3_GS_WAITS: (GPU busy time, last completion), reported at exit.
    type Busy = Arc<Mutex<(std::time::Duration, Option<std::time::Instant>)>>;
    let busy: Option<Busy> = std::env::var_os("SSX3_GS_WAITS").map(|_| Arc::new(Mutex::new((std::time::Duration::ZERO, None))));
    let t0 = std::time::Instant::now();
    // Completion callbacks run in `poll`: poll often while measuring.
    let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
    if busy.is_some() {
        let (d, st) = (dev.clone(), stop.clone());
        std::thread::spawn(move || {
            while !st.load(std::sync::atomic::Ordering::Relaxed) {
                let _ = d.poll(wgpu::PollType::Poll);
                std::thread::sleep(std::time::Duration::from_micros(200));
            }
        });
    }
    while let Ok(job) = rx.recv() {
        let mut back = None;
        if let Some((buf, words)) = job.data {
            q.write_buffer(&buf, 0, bytemuck_words(&words));
            back = Some(words);
        }
        if let Some((buf, bytes)) = &job.params {
            q.write_buffer(buf, 0, bytes);
        }
        let idx = q.submit(std::iter::once(job.enc.finish()));
        if let Some(b) = busy.as_ref() {
            // GPU busy time: each submission from when it was submitted (or
            // the previous one completed) to its completion.
            let t = std::time::Instant::now();
            let b = b.clone();
            q.on_submitted_work_done(move || {
                let done = std::time::Instant::now();
                let mut g = b.lock().unwrap();
                let start = g.1.map_or(t, |p: std::time::Instant| p.max(t));
                g.0 += done.saturating_duration_since(start);
                g.1 = Some(done);
            });
        }
        if let Some(f) = job.after {
            f(&idx);
        }
        inflight.push_back(idx);
        while inflight.len() > MAX_INFLIGHT {
            let old = inflight.pop_front().expect("inflight");
            let _ = dev.poll(wgpu::PollType::Wait { submission_index: Some(old), timeout: None });
        }
        if let Some(mut w) = back {
            w.clear();
            let mut s = done.spare.lock().unwrap();
            if s.len() < MAX_QUEUED + 2 {
                s.push(w);
            }
        }
        *done.count.lock().unwrap() += 1;
        done.cv.notify_all();
    }
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    if let Some(b) = busy {
        let _ = dev.poll(wgpu::PollType::wait_indefinitely());
        let g = b.lock().unwrap();
        eprintln!("[gs-waits] GPU busy {:.1} s of {:.1} s", g.0.as_secs_f64(), t0.elapsed().as_secs_f64());
    }
}

fn bytemuck_words(w: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding and any byte pattern is valid u8.
    unsafe { std::slice::from_raw_parts(w.as_ptr() as *const u8, std::mem::size_of_val(w)) }
}
