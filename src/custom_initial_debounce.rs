//! Source focus-entry debounce starts with the native countdown.
//!
//! Tricky loads its start-reset guard with four seconds of immunity. SSX 3
//! can spend that entire interval in loading/briefing before its three-second
//! countdown. Retain the constructor's exact native delay until AI state4;
//! the ordinary Debounce node owns countdown, contacts and expiry afterwards.

use ssx3_ee_rt::cpu::reg::{A1, A2};
use ssx3_ee_rt::Cpu;
use std::collections::BTreeMap;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy)]
struct Guard {
    instance: u32,
    node: u32,
    ticks: u32,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    guards: BTreeMap<u32, Guard>,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if state.track != track { *state = State { track, ..State::default() }; }
    track.is_some()
}

fn race_state(c: &mut Cpu) -> Option<u32> {
    let app = c.bus.read32(0x004a_28a8);
    if app == 0 { return None }
    let module = c.bus.read32(app + 0x84);
    if module == 0 { return None }
    let ai = c.bus.read32(module + 0xc);
    (ai != 0).then(|| c.bus.read32(ai))
}

fn installed(c: &mut Cpu, owner: u32) -> Option<(u32, u32)> {
    let instance = crate::custom_script_effects::resident_instance(c, owner)?;
    let node = c.bus.read32(instance + 12);
    if node == 0 || c.bus.read32(node + 12) != 0x0049_06f0
        || c.bus.read32(node + 0x18) != instance { return None }
    Some((instance, node))
}

pub fn restart() { *STATE.lock().unwrap() = State::default(); }

/// Imported builtin123 follows only source focus-entry Debounce creation.
/// Contact-created debounce continues to start at the actual contact.
pub fn register(c: &mut Cpu) {
    if c.g[A1][0] != 1 || c.g[A2][0] == 0 { return }
    let arg = c.g[A2][0] as u32;
    if c.bus.read32(arg) != 0 || c.bus.read32(arg + 12) != 1 { return }
    if race_state(c).is_some_and(|state| state >= 4) { return }
    let owner = c.bus.read32(arg + 4);
    let Some((instance, node)) = installed(c, owner) else { return };
    let ticks = c.bus.read32(node + 0x2c);
    if ticks as i32 <= 0 { return }
    let mut state = STATE.lock().unwrap();
    if sync(&mut state) { state.guards.insert(owner, Guard { instance, node, ticks }); }
}

pub fn begin_update(c: &mut Cpu) {
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) || state.guards.is_empty() { return }
    // AI_SetRaceState(4) initializes the native 180-frame countdown. State5
    // is racing; later focus entries need no initial-load protection.
    if race_state(c).is_some_and(|phase| phase >= 4) {
        state.guards.clear();
        return;
    }
    state.guards.retain(|&owner, guard| {
        if installed(c, owner) != Some((guard.instance, guard.node)) { return false }
        c.bus.write32(guard.node + 0x2c, guard.ticks);
        true
    });
}
