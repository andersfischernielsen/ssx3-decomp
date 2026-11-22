//! Deferred imported graph segments, executed by the native script manager.
//!
//! Only a compact function RID and copied contact values survive a wait.
//! Native instance/function pointers are resolved again when the job runs.

use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

type Key = (u32, u32); // root instance RID / native callback slot

#[derive(Clone)]
struct Job {
    function: u32,
    owner: u32,
    key: Key,
    rider: u32,
    contact_a: Option<[u32; 32]>,
    contact_b: Option<[u32; 16]>,
    deadline: f64,
    epoch: u64,
}

#[derive(Default)]
struct Queue {
    track: Option<&'static str>,
    time: f64,
    epoch: u64,
    jobs: Vec<Job>,
    busy: HashMap<Key, usize>,
    scopes: HashMap<u32, Vec<[u32; 6]>>,
}

static QUEUE: LazyLock<Mutex<Queue>> = LazyLock::new(|| Mutex::new(Queue::default()));
const CONTEXT_FIELDS: [u32; 6] = [0, 0x290, 0x294, 0x298, 0x2a4, 0x2a8];
// The manager's consumed command snapshot occupies stack +0..+0x183.
// Its saved registers begin at +0x190, outside these temporary records.
const CONTACT_B: u32 = 0x10;
const CONTACT_A: u32 = 0x50;
const SAVED_CONTEXT: u32 = 0xd0;
const BUSY_KEY: u32 = 0xf0;

fn sync(queue: &mut Queue) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if queue.track != track {
        *queue = Queue { track, ..Queue::default() };
    }
    track.is_some()
}

pub fn restart() {
    crate::custom_rail_rules::restart();
    crate::custom_script_materials::restart();
    crate::custom_roller_runtime::restart();
    crate::custom_initial_debounce::restart();
    crate::custom_animation_debounce::restart();
    crate::custom_crowds::restart();
    crate::custom_movies::restart();
    crate::custom_cracked_glass::restart();
    crate::custom_collision_emitters::restart();
    crate::custom_spline_movers::restart();
    crate::custom_force_overlap::restart();
    *QUEUE.lock().unwrap() = Queue::default();
}

pub fn begin_update(c: &mut Cpu) {
    crate::custom_rail_rules::update(c);
    let mut queue = QUEUE.lock().unwrap();
    if !sync(&mut queue) { return }
    queue.time += 1. / crate::tick::rate(c).max(1) as f64;
    queue.epoch += 1;
}

fn arguments(c: &mut Cpu) -> HashMap<u32, u32> {
    let count = c.g[ssx3_ee_rt::cpu::reg::A1][0] as u32;
    let base = c.g[ssx3_ee_rt::cpu::reg::A2][0] as u32;
    (0..count.min(6)).map(|i| {
        let at = base + i * 16;
        (c.bus.read32(at), c.bus.read32(at + 4))
    }).collect()
}

pub fn can_start(c: &mut Cpu) -> bool {
    let args = arguments(c);
    let (Some(&owner), Some(&slot)) = (args.get(&0), args.get(&1)) else { return false };
    let mut queue = QUEUE.lock().unwrap();
    sync(&mut queue) && !queue.busy.contains_key(&(owner, slot))
}

/// Luno builtin 115: function, owner, seconds, root owner, root slot, detached.
pub fn schedule(c: &mut Cpu) {
    use ssx3_ee_rt::cpu::reg::GP;
    let args = arguments(c);
    let (Some(&function), Some(&owner), Some(&delay), Some(&root), Some(&slot)) =
        (args.get(&0), args.get(&1), args.get(&2), args.get(&3), args.get(&4)) else { return };
    let delay = f32::from_bits(delay);
    if !delay.is_finite() || delay < 0. { return }
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if manager == 0 { return }
    let detached = args.get(&5).copied().unwrap_or(0) != 0;
    let rider = if detached { u32::MAX } else { c.bus.read32(manager) };
    let a = if detached { 0 } else { c.bus.read32(manager + 0x298) };
    let b = if detached { 0 } else { c.bus.read32(manager + 0x294) };
    let contact_a = (a != 0).then(|| std::array::from_fn(|i| c.bus.read32(a + i as u32 * 4)));
    let contact_b = (b != 0).then(|| std::array::from_fn(|i| c.bus.read32(b + i as u32 * 4)));
    let hz = crate::tick::rate(c).max(1) as f32;
    // Multiply in source float precision before quantizing to native ticks;
    // e.g. an authored 0.033333335 seconds is two ticks at 60, not three.
    let seconds = (delay * hz).ceil() as f64 / hz as f64;
    let mut queue = QUEUE.lock().unwrap();
    if !sync(&mut queue) { return }
    let key = (root, slot);
    let job = Job { function, owner, key, rider, contact_a, contact_b,
        deadline: queue.time + seconds, epoch: queue.epoch + 1 };
    queue.jobs.push(job);
    *queue.busy.entry(key).or_default() += 1;
    if tracing() {
        static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 64 {
            eprintln!("[tricky] scheduled {function:08x} on {owner:08x}, delay {delay}, rider {rider}, clock {}", queue.time);
        }
    }
}

/// Main 26 executes the new thread's prefix without a rider. Its wait copies
/// this detached context; restoring the caller does not change that job.
pub fn detach(c: &mut Cpu) {
    use ssx3_ee_rt::cpu::reg::{GP, SP};
    let args = arguments(c);
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if manager == 0 { return }
    let context = std::array::from_fn(|i| c.bus.read32(manager + CONTEXT_FIELDS[i]));
    {
        let mut queue = QUEUE.lock().unwrap();
        if !sync(&mut queue) { return }
        queue.scopes.entry(c.g[SP][0] as u32).or_default().push(context);
    }
    c.bus.write32(manager, u32::MAX);
    c.bus.write32(manager + 0x294, 0);
    c.bus.write32(manager + 0x298, 0);
    if let Some(&rid) = args.get(&0) {
        if let Some(owner) = crate::custom_script_effects::resident_instance(c, rid) {
            c.bus.write32(manager + 0x290, owner);
        }
    }
}

pub fn restore_scope(c: &mut Cpu) {
    use ssx3_ee_rt::cpu::reg::{GP, SP};
    let key = c.g[SP][0] as u32;
    let context = {
        let mut queue = QUEUE.lock().unwrap();
        let Some(stack) = queue.scopes.get_mut(&key) else { return };
        let context = stack.pop();
        if stack.is_empty() { queue.scopes.remove(&key); }
        context
    };
    let Some(context) = context else { return };
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if manager == 0 { return }
    for (offset, value) in CONTEXT_FIELDS.iter().zip(context) {
        c.bus.write32(manager + offset, value);
    }
}

fn tracing() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some())
}

fn function_resident(c: &mut Cpu, manager: u32, rid: u32) -> bool {
    let world = c.bus.read32(manager + 0x28c);
    if world == 0 { return false }
    let cache = c.bus.read32(world + 4);
    if cache == 0 { return false }
    let rows = c.bus.read32(cache + 4);
    if rows == 0 { return false }
    let row = rows + (rid & 255) * 12;
    if c.bus.read16(row) != 3 { return false }
    let data = c.bus.read32(row + 8);
    data != 0 && rid >> 8 < c.bus.read32(data + 0x38)
}

/// Bind a ready job in the manager's consumed snapshot area. S7 in the
/// manager override records that its next epilogue entry must restore it.
pub fn prepare_next(c: &mut Cpu, manager: u32, stack: u32) -> bool {
    loop {
        let job = {
            let mut queue = QUEUE.lock().unwrap();
            if !sync(&mut queue) { return false }
            let Some(index) = queue.jobs.iter().position(|job|
                job.epoch <= queue.epoch && job.deadline <= queue.time + 1e-9) else { return false };
            queue.jobs.remove(index)
        };
        let owner = crate::custom_script_effects::resident_instance(c, job.owner);
        if owner.is_none() || !function_resident(c, manager, job.function) {
            let mut queue = QUEUE.lock().unwrap();
            let mut job = job;
            job.epoch = queue.epoch + 1;
            queue.jobs.push(job);
            continue;
        }
        for (i, offset) in CONTEXT_FIELDS.iter().enumerate() {
            let value = c.bus.read32(manager + offset);
            c.bus.write32(stack + SAVED_CONTEXT + i as u32 * 4, value);
        }
        c.bus.write32(stack, job.function);
        c.bus.write32(stack + BUSY_KEY, job.key.0);
        c.bus.write32(stack + BUSY_KEY + 4, job.key.1);
        c.bus.write32(manager, job.rider);
        c.bus.write32(manager + 0x290, owner.unwrap());
        c.bus.write32(manager + 0x2a4, 0);
        c.bus.write32(manager + 0x2a8, 0);
        for (offset, values) in [(CONTACT_B, job.contact_b.as_ref().map(|v| v.as_slice())),
                                 (CONTACT_A, job.contact_a.as_ref().map(|v| v.as_slice()))] {
            if let Some(values) = values {
                for (i, value) in values.iter().enumerate() {
                    c.bus.write32(stack + offset + i as u32 * 4, *value);
                }
            }
            let field = if offset == CONTACT_B { 0x294 } else { 0x298 };
            c.bus.write32(manager + field, if values.is_some() { stack + offset } else { 0 });
        }
        if tracing() {
            static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 64 {
                let queue = QUEUE.lock().unwrap();
                eprintln!("[tricky] running deferred {:08x} on {:08x}, rider {}, clock {}", job.function, job.owner, job.rider, queue.time);
            }
        }
        return true;
    }
}

pub fn finish(c: &mut Cpu, manager: u32, stack: u32) {
    for (i, offset) in CONTEXT_FIELDS.iter().enumerate() {
        let value = c.bus.read32(stack + SAVED_CONTEXT + i as u32 * 4);
        c.bus.write32(manager + offset, value);
    }
    let key = (c.bus.read32(stack + BUSY_KEY), c.bus.read32(stack + BUSY_KEY + 4));
    let mut queue = QUEUE.lock().unwrap();
    if let Some(count) = queue.busy.get_mut(&key) {
        *count -= 1;
        if *count == 0 { queue.busy.remove(&key); }
    }
}
