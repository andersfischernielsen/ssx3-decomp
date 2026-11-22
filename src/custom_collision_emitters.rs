//! Tricky collision bursts using native particle allocation and simulation.
//!
//! Source148560/148748 supplies the contact point and normal-scaled launch;
//! source148900 caps live effects per rider. Native modifiers own their
//! lifetime, rendering, list membership and focus cleanup.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

const FRAME: u32 = 0x400;
const OUT: u32 = 0x30;
const ARGUMENTS: u32 = 0x40;
const ARGUMENT_COUNT: u32 = 54;

#[derive(Clone, Copy)]
struct Effect {
    rider: u32,
    owner: u32,
    instance: u32,
    node: u32,
    modifier: u32,
    description: u32,
}

#[derive(Clone, Copy)]
struct Pending {
    vm_stack: u32,
    track: Option<&'static str>,
    rider: u32,
    owner: u32,
    instance: u32,
    previous: u32,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    effects: Vec<Effect>,
    pending: HashMap<u32, Pending>,
}
static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if state.track != track {
        state.track = track;
        state.effects.clear();
    }
    // Pending native calls retain their private stack until their return,
    // including when a course retires or is restarted during that call.
}

pub fn restart() { STATE.lock().unwrap().effects.clear(); }

fn valid(address: u32, bytes: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000 && physical.checked_add(bytes).is_some_and(|end| end <= 0x0200_0000)
}

fn kind(key: u32) -> u32 {
    if matches!(key, 0 | 1 | 49 | 50 | 52 | 54) { 1 } else { 2 }
}

fn arguments(c: &mut Cpu) -> Option<[u32; 55]> {
    let base = c.g[A2][0] as u32;
    if c.g[A1][0] != u64::from(ARGUMENT_COUNT) || !valid(base, ARGUMENT_COUNT * 16) { return None }
    let mut values = [0; 55];
    let mut seen = 0u64;
    for index in 0..ARGUMENT_COUNT {
        let at = base + index * 16;
        let key = c.bus.read32(at);
        if key > 54 || key == 51 || seen & (1u64 << key) != 0
            || c.bus.read32(at + 12) != kind(key) { return None }
        let value = c.bus.read32(at + 4);
        if kind(key) == 2 && !f32::from_bits(value).is_finite() { return None }
        seen |= 1u64 << key;
        values[key as usize] = value;
    }
    (values[49] & 0xffff_0000 == 0x5458_0000
        && values[50] & 0xffff_0000 == 0x4250_0000).then_some(values)
}

/// Particle actor -> native ModifierBlock's particle-list head. The hidden
/// collision host is dedicated to this family, never an animated source prop.
fn particle_head(c: &mut Cpu, instance: u32) -> Option<(u32, u32)> {
    let node = c.bus.read32(instance + 12);
    if !valid(node, 0x2c) || c.bus.read16(node + 0x10) != 13
        || c.bus.read32(node + 0x18) != instance { return None }
    let block = c.bus.read32(node + 0x1c);
    if !valid(block, 0x14) { return None }
    Some((node, c.bus.read32(block + 0x10)))
}

fn alive(c: &mut Cpu, effect: &Effect) -> bool {
    if crate::custom_script_effects::resident_instance(c, effect.owner) != Some(effect.instance) {
        return false;
    }
    let Some((node, mut modifier)) = particle_head(c, effect.instance) else { return false };
    if node != effect.node { return false }
    // Native ModifierList_PushFront345700 stores next at+0. This host can
    // have at most the source per-rider caps, not an unbounded scene list.
    for _ in 0..128 {
        if !valid(modifier, 0x1e8) { return false }
        if modifier == effect.modifier {
            return c.bus.read32(modifier + 8) == 0x0049_12b0
                && c.bus.read32(modifier + 12) == effect.instance
                && c.bus.read32(modifier + 0x1e0) == effect.description
                && c.bus.read32(modifier + 0x1e4) == 0;
        }
        modifier = c.bus.read32(modifier);
    }
    false
}

/// Builtin127 has the native AddParticleData arguments. Only six dynamic
/// fields differ: origin9..11 and velocity18..20, as in Tricky148748.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let Some(mut args) = arguments(c) else { return Ok(()) };
    let Some((manager, _, rider)) = crate::custom_script_effects::rider(c) else { return Ok(()) };
    let contact = c.bus.read32(manager + 0x294);
    if !valid(contact, 0x40) { return Ok(()) }
    let owner = args[54];
    let Some(instance) = crate::custom_script_effects::resident_instance(c, owner) else { return Ok(()) };
    let track;
    {
        let mut state = STATE.lock().unwrap();
        sync(&mut state);
        state.effects.retain(|effect| alive(c, effect));
        // Source148900 allows three effects for a device/human rider and
        // one for other riders. Native IsHuman uses the field at+874.
        let limit = if c.bus.read32(rider + 0x874) != 0 { 3 } else { 1 };
        let live = state.effects.iter().filter(|effect| effect.rider == rider).count();
        let pending = state.pending.values().filter(|call| call.rider == rider && call.track == state.track).count();
        if live + pending >= limit { return Ok(()) }
        track = state.track;
    }
    let previous = particle_head(c, instance).map_or(0, |(_, head)| head);
    let vm_stack = c.g[SP][0] as u32;
    let Some(stack) = vm_stack.checked_sub(FRAME).filter(|&stack| valid(stack, FRAME)) else { return Ok(()) };
    for index in 0..4 {
        c.bus.write32(stack + index * 4, if index < 3 { args[18 + index as usize] } else { 0 });
    }
    // The source uses the original VU XYZ length, then contact normal*speed.
    // Keep VU operations (including the Q result) instead of a host sqrt.
    c.lqc2(3, stack);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, X);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    c.mtc1(V0, 0);
    c.mfc1(V1, 0);
    c.lqc2(4, contact + 0x20);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, stack + 0x10);
    for axis in 0..3 {
        args[9 + axis] = c.bus.read32(contact + axis as u32 * 4);
        args[18 + axis] = c.bus.read32(stack + 0x10 + axis as u32 * 4);
        if !f32::from_bits(args[9 + axis]).is_finite()
            || !f32::from_bits(args[18 + axis]).is_finite() { return Ok(()) }
    }
    for offset in [0, 4, 8] { c.bus.write32(stack + OUT + offset, 0); }
    let mut index = 0;
    for key in 0..55 {
        if key == 51 { continue }
        let at = stack + ARGUMENTS + index * 16;
        c.bus.write32(at, key);
        c.bus.write32(at + 4, args[key as usize]);
        c.bus.write32(at + 8, 0);
        c.bus.write32(at + 12, kind(key));
        index += 1;
    }
    STATE.lock().unwrap().pending.insert(stack, Pending { vm_stack, track, rider,
        owner, instance, previous });
    c.g[SP][0] = sx(stack);
    c.g[A0][0] = sx(stack + OUT);
    c.g[A1][0] = u64::from(ARGUMENT_COUNT);
    c.g[A2][0] = sx(stack + ARGUMENTS);
    call_ptr(c, 0x002f_e840, back)?;
    after(c)
}

/// The VM invokes this at its ordinary builtin return site, including a
/// suspended native allocation. Always release the private caller frame.
pub fn after(c: &mut Cpu) -> Flow {
    let stack = c.g[SP][0] as u32;
    let Some(pending) = STATE.lock().unwrap().pending.remove(&stack) else { return Ok(()) };
    if let Some((node, modifier)) = particle_head(c, pending.instance) {
        if modifier != pending.previous && valid(modifier, 0x1e8)
            && c.bus.read32(modifier + 8) == 0x0049_12b0 {
            let description = c.bus.read32(modifier + 0x1e0);
            let mut state = STATE.lock().unwrap();
            sync(&mut state);
            if state.track.is_some() && state.track == pending.track {
                state.effects.push(Effect { rider: pending.rider, owner: pending.owner,
                    instance: pending.instance, node, modifier, description });
            }
        }
    }
    c.g[SP][0] = sx(pending.vm_stack);
    Ok(())
}
