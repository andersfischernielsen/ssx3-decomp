//! Tricky Cracked14 health, contacts and source clock policy.
//!
//! Native Object/TexFlip own geometry, private materials and focus cleanup.
//! Native WorldObj_KillRestorable dispatches the authored end graph, which
//! removes the pane/support and launches its original fragments.

use ssx3_ee_rt::cpu::reg::{A0, A1, A2, AT, GP, V0};
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;
use std::collections::BTreeMap;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy)]
struct Pane {
    node: u32,
    block: u32,
    modifier: u32,
    strength: f32,
    expires: Option<f64>,
    contact_after: f64,
    done: bool,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    time: f64,
    panes: BTreeMap<u32, Pane>,
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

fn valid(at: u32, size: u32) -> bool {
    let at = at & 0x1fff_ffff;
    at >= 0x10000 && at.checked_add(size).is_some_and(|end| end <= 0x0200_0000)
}

fn installed(c: &mut Cpu, owner: u32) -> Option<(u32, u32, u32)> {
    let instance = crate::custom_script_effects::resident_instance(c, owner)?;
    let node = c.bus.read32(instance + 12);
    if !valid(node, 0x20) || c.bus.read16(node + 0x10) != 17
        || c.bus.read32(node + 0x18) != instance { return None }
    let block = c.bus.read32(node + 0x1c);
    if !valid(block, 12) { return None }
    let modifier = c.bus.read32(block + 8);
    valid(modifier, 0xe8).then_some((node, block, modifier))
}

fn arguments(c: &mut Cpu, tags: &[u32]) -> Option<Vec<u32>> {
    let base = c.g[A2][0] as u32;
    if c.g[A1][0] != tags.len() as u64 || !valid(base, tags.len() as u32 * 16) { return None }
    let mut values = vec![0; tags.len()];
    let mut seen = 0u32;
    for i in 0..tags.len() {
        let at = base + i as u32 * 16;
        let key = c.bus.read32(at) as usize;
        if key >= tags.len() || seen & (1 << key) != 0
            || c.bus.read32(at + 12) != tags[key] { return None }
        values[key] = c.bus.read32(at + 4);
        seen |= 1 << key;
    }
    Some(values)
}

fn trace(owner: u32, action: &str, before: f32, after: f32) {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some()) { return }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 96 {
        eprintln!("[tricky] cracked {action} on {owner:08x}: {before} -> {after}");
    }
}

/// Original Tricky Damage0x145808. Keep VU0's four-lane reduction and FPU
/// operation order, including the recovered 0x3d1374bd multiplier.
fn damage(c: &mut Cpu, strength: f32) -> (f32, bool) {
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if !valid(manager, 0x29c) || strength <= 0. { return (strength, false) }
    let contact = c.bus.read32(manager + 0x294);
    if !valid(contact, 0x34) { return (strength, false) }
    c.lwc1(1, contact + 0x30);
    c.lqc2(3, contact + 0x10);
    c.lqc2(5, contact + 0x20);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    c.g[AT][0] = 0x3d13_74bd;
    c.mtc1(AT, 2);
    c.mul_s(0, 0, 1);
    c.abs_s(0, 0);
    c.mul_s(0, 0, 2);
    c.f[3] = strength.to_bits();
    c.sub_s(0, 3, 0);
    (f32::from_bits(c.f[0]), true)
}

// The source constructor truncates positive seconds*60 to a frame count.
// Preserve that authored duration while the native game rate is configurable.
fn expiry(now: f64, seconds: f32) -> Option<f64> {
    let frames = (seconds * 60.).trunc();
    (frames > 0.).then_some(now + frames as f64 / 60.)
}

/// Builtin124: true only when native Object/TexFlip creation is needed.
/// Subsequent contacts use the existing pane and its 30-source-tick gate;
/// they do not reset health, lifetime or the private cracked appearance.
pub fn contact(c: &mut Cpu) {
    let out = c.g[A0][0] as u32;
    c.bus.write32(out, 0);
    c.bus.write32(out + 8, 1);
    let Some(args) = arguments(c, &[1, 2, 2]) else { return };
    let owner = args[0];
    if !f32::from_bits(args[1]).is_finite() || !f32::from_bits(args[2]).is_finite() { return }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    let live = state.panes.get(&owner).copied().filter(|pane|
        installed(c, owner) == Some((pane.node, pane.block, pane.modifier)));
    if let Some(mut pane) = live {
        if !pane.done && state.time + 1e-9 >= pane.contact_after {
            let before = pane.strength;
            let (strength, hit) = damage(c, before);
            pane.strength = strength;
            if hit {
                pane.contact_after = state.time + 0.5;
                trace(owner, "contact", before, strength);
            }
            state.panes.insert(owner, pane);
        }
    } else {
        state.panes.remove(&owner);
        c.bus.write32(out, 1);
        c.bus.write32(out + 8, 1);
    }
}

/// Builtin125, after the native constructors: retain real ownership and
/// apply the original constructor's initial contact without the later gate.
pub fn attach(c: &mut Cpu) {
    let Some(args) = arguments(c, &[1, 2, 2]) else { return };
    let [owner, length, strength] = [args[0], args[1], args[2]];
    let (length, strength) = (f32::from_bits(length), f32::from_bits(strength));
    if !length.is_finite() || !strength.is_finite() { return }
    let Some((node, block, modifier)) = installed(c, owner) else { return };
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    let (remaining, _) = damage(c, strength);
    let pane = Pane { node, block, modifier, strength: remaining,
        expires: expiry(state.time, length), contact_after: state.time + 0.5, done: false };
    state.panes.insert(owner, pane);
    trace(owner, "create", strength, remaining);
}

/// Builtin126: source commands1/2 replace lifetime/strength respectively.
pub fn message(c: &mut Cpu) {
    let Some(args) = arguments(c, &[1, 1, 2]) else { return };
    let value = f32::from_bits(args[2]);
    if !value.is_finite() { return }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    let now = state.time;
    let Some(pane) = state.panes.get_mut(&args[0]) else { return };
    if installed(c, args[0]) != Some((pane.node, pane.block, pane.modifier)) { return }
    match args[1] {
        1 => pane.expires = expiry(now, value),
        2 => pane.strength = value,
        _ => {},
    }
}

/// After native process iteration, call native KillRestorable(node,1).
/// Its existing end callback owns support/fragments; no host target list.
pub fn next_ready(c: &mut Cpu) -> Option<u32> {
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return None }
    let now = state.time;
    let mut ready = None;
    state.panes.retain(|&owner, pane| {
        if installed(c, owner) != Some((pane.node, pane.block, pane.modifier)) { return false }
        if ready.is_none() && !pane.done
            && (pane.strength <= 0. || pane.expires.is_some_and(|deadline| now + 1e-9 >= deadline)) {
            pane.done = true;
            ready = Some(pane.node);
            trace(owner, "end", pane.strength, pane.strength);
        }
        true
    });
    ready
}
