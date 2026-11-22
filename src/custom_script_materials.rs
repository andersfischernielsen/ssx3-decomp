//! Finite imported material properties, using native modifier ownership.
//!
//! Playback and private material records remain native. Only the source
//! lifetime is retained here. Expiry retires a standalone Object through
//! WorldObj_Kill, restoring static flags and removing its instance backlink.
//! If another modifier still owns the Object, only this property is freed.

use ssx3_ee_rt::cpu::reg::{A1, A2};
use ssx3_ee_rt::Cpu;
use std::collections::BTreeMap;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy)]
struct Property {
    node: u32,
    block: u32,
    modifier: u32,
    deadline: f64,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    time: f64,
    properties: BTreeMap<(u32, u32), Property>,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if state.track != track { *state = State { track, ..State::default() }; }
    track.is_some()
}

pub fn restart() { *STATE.lock().unwrap() = State::default(); }

pub fn begin_update(c: &mut Cpu) {
    let mut state = STATE.lock().unwrap();
    if sync(&mut state) { state.time += 1. / crate::tick::rate(c).max(1) as f64; }
}

fn slot(kind: u32) -> Option<u32> {
    match kind { 10 => Some(4), 11 => Some(8), _ => None }
}

fn installed(c: &mut Cpu, owner: u32, kind: u32) -> Option<(u32, u32, u32)> {
    let offset = slot(kind)?;
    let instance = crate::custom_script_effects::resident_instance(c, owner)?;
    let node = c.bus.read32(instance + 12);
    // The compiler creates a native Object before these standalone material
    // properties. Other actor layouts must not be treated as a ModObj.
    if node == 0 || c.bus.read16(node + 0x10) != 17
        || c.bus.read32(node + 0x18) != instance { return None }
    let block = c.bus.read32(node + 0x1c);
    if block == 0 { return None }
    let modifier = c.bus.read32(block + offset);
    (modifier != 0).then_some((node, block, modifier))
}

fn trace(action: &str, owner: u32, kind: u32, time: f64, length: Option<f32>) {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some()) { return }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 96 {
        eprintln!("[tricky] material {action} {kind} on {owner:08x}, clock {time}, length {length:?}");
    }
}

/// Builtin 121: owner RID, source property kind, source lifetime seconds.
/// Called after every imported UV/flip constructor, including persistent
/// replacements, so an older deadline can never retire a newer property.
pub fn register(c: &mut Cpu) {
    if c.g[A1][0] != 3 || c.g[A2][0] == 0 { return }
    let base = c.g[A2][0] as u32;
    let mut values = [0; 3];
    let mut seen = 0;
    for i in 0..3 {
        let at = base + i * 16;
        let key = c.bus.read32(at);
        if key >= 3 || seen & (1 << key) != 0
            || c.bus.read32(at + 12) != if key == 2 { 2 } else { 1 } { return }
        values[key as usize] = c.bus.read32(at + 4);
        seen |= 1 << key;
    }
    let [owner, kind, length] = values;
    let length = f32::from_bits(length);
    if slot(kind).is_none() || !length.is_finite() { return }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    state.properties.remove(&(owner, kind));
    if length <= 0. { return }
    let Some((node, block, modifier)) = installed(c, owner, kind) else { return };
    let hz = crate::tick::rate(c).max(1) as f32;
    let deadline = state.time + (length * hz).ceil() as f64 / hz as f64;
    state.properties.insert((owner, kind), Property { node, block, modifier, deadline });
    trace("start", owner, kind, state.time, Some(length));
}

/// Called at the manager's epilogue, after native process iteration. Remove
/// the record before calling a native free, so returning after a yield cannot
/// expire it twice. Focus cleanup/replacement invalidates pointer identity;
/// the new constructor also replaces its lifetime explicitly via register().
pub fn next_expired(c: &mut Cpu) -> Option<(u32, u32)> {
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return None }
    let now = state.time;
    let mut expired = None;
    state.properties.retain(|&(owner, kind), property| {
        if installed(c, owner, kind) != Some((property.node, property.block, property.modifier)) {
            return false;
        }
        if expired.is_none() && property.deadline <= now + 1e-9 {
            expired = Some((owner, kind, *property));
            return false;
        }
        true
    });
    let (owner, kind, property) = expired?;
    trace("end", owner, kind, now, None);
    // ModifierBlock_ctor initializes these eight ownership fields; +18/+24
    // are list vtables. Kill an otherwise empty Object so NoLiveNode gates
    // can see completion, while preserving any other native modifier owner.
    let standalone = [0, 4, 8, 12, 16, 20, 28, 32].into_iter()
        .all(|offset| offset == slot(kind).unwrap()
            || c.bus.read32(property.block + offset) == 0);
    if standalone {
        Some((0x0034_fd90, property.node))
    } else {
        Some((if kind == 10 { 0x0035_3118 } else { 0x0035_3150 }, property.block))
    }
}
