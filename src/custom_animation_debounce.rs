//! Source animation contact delays retained by native ModObj timers.
//!
//! Tricky AnimNode198810 stores the authored seconds as ticks, and its
//! contact handler198eb8 rearms that value. Native object-script dispatch
//! instead requests0.5s after every contact. Keep the source value only for
//! explicitly registered imported animation owners at that call boundary.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy)]
struct Delay {
    owner: u32,
    instance: u32,
    seconds: u32,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    nodes: HashMap<u32, Delay>,
    owners: HashMap<u32, u32>,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if state.track != track {
        *state = State {
            track,
            ..State::default()
        };
    }
    track.is_some()
}

pub fn restart() {
    *STATE.lock().unwrap() = State::default();
}

fn valid(address: u32, bytes: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000
        && physical
            .checked_add(bytes)
            .is_some_and(|end| end <= 0x0200_0000)
}

fn animation(c: &mut Cpu, owner: u32) -> Option<(u32, u32)> {
    let instance = crate::custom_script_effects::resident_instance(c, owner)?;
    let node = c.bus.read32(instance + 12);
    if !valid(node, 0x24)
        || !matches!(c.bus.read16(node + 0x10), 1..=3)
        || c.bus.read32(node + 0x18) != instance
    {
        return None;
    }
    Some((instance, node))
}

/// Imported builtin133 follows animation factories3/4/5: owner RID, seconds,
/// whether this graph inherits contact context.
/// A copied contact context also starts the initial guard, as Source198af0
/// does when its context argument is non-null. Detached/focus creation does
/// not acquire that initial guard. The real native leaf converts seconds to
/// configured game ticks; no separate host clock or counter is introduced.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    if c.g[A1][0] != 3 || c.g[A2][0] == 0 {
        return Ok(());
    }
    let args = c.g[A2][0] as u32;
    let mut values = [0; 3];
    let mut seen = 0;
    for i in 0..3 {
        let at = args + i * 16;
        let key = c.bus.read32(at);
        if key >= 3
            || seen & (1 << key) != 0
            || c.bus.read32(at + 12) != if key == 1 { 2 } else { 1 }
        {
            return Ok(());
        }
        seen |= 1 << key;
        values[key as usize] = c.bus.read32(at + 4);
    }
    let [owner, seconds, contact] = values;
    let length = f32::from_bits(seconds);
    if !length.is_finite() || length < 0. || contact > 1 {
        return Ok(());
    }
    let Some((instance, node)) = animation(c, owner) else {
        return Ok(());
    };
    {
        let mut state = STATE.lock().unwrap();
        if !sync(&mut state) {
            return Ok(());
        }
        // Focus re-entry can allocate a new node. Keep at most one record
        // per source owner without scanning every registered animation.
        if let Some(previous) = state.owners.insert(owner, node) {
            if previous != node
                && state
                    .nodes
                    .get(&previous)
                    .is_some_and(|delay| delay.owner == owner)
            {
                state.nodes.remove(&previous);
            }
        }
        state.nodes.insert(
            node,
            Delay {
                owner,
                instance,
                seconds,
            },
        );
    }
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if contact != 0
        && valid(manager, 0x29c)
        && valid(c.bus.read32(manager + 0x298), 0x80)
        && valid(c.bus.read32(manager + 0x294), 0x40)
    {
        c.g[A0][0] = sx(node);
        c.f[12] = seconds;
        return call_ptr(c, 0x0035_5858, back);
    }
    Ok(())
}

/// Only replace WScriptMan_OnObjectTrigger's hard-coded0.5s rearm. Explicit
/// timer resets and other native callers retain their supplied duration.
/// Pointer/backlink/kind identity rejects retired or replaced owners.
pub fn contact_seconds(c: &mut Cpu) -> Option<u32> {
    if c.g[RA][0] as u32 != 0x0030_a240 || c.f[12] != 0x3f00_0000 {
        return None;
    }
    let node = c.g[A0][0] as u32;
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) {
        return None;
    }
    let delay = state.nodes.get(&node).copied()?;
    if animation(c, delay.owner) != Some((delay.instance, node)) {
        state.nodes.remove(&node);
        return None;
    }
    Some(delay.seconds)
}
