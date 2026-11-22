//! Native rider-hull queries for the lifetime of imported shared force nodes.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// Rider_HandleWallCollision reserves at least0x370 bytes for this same Hull.
// Keep0x400 aligned bytes, then the native query's64 0x80-byte contact records.
const CONTACTS: u32 = 0x400;
const CAPACITY: u32 = 64;
const FRAME: u32 = CONTACTS + CAPACITY * 0x80;
const RIDER_TREE: u32 = 0xaa0;

#[derive(Clone, Copy)]
struct Pending {
    vm_stack: u32,
    output: u32,
    instance: u32,
    riders: [u32; 6],
    index: usize,
    phase: u8,
    hit: bool,
}

static PENDING: LazyLock<Mutex<HashMap<u32, Pending>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn restart() {
    PENDING.lock().unwrap().clear();
}

fn valid(address: u32, bytes: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000
        && physical
            .checked_add(bytes)
            .is_some_and(|end| end <= 0x0200_0000)
}

fn active_riders(c: &mut Cpu) -> Option<[u32; 6]> {
    let world = c.bus.read32((c.g[GP][0] as u32).wrapping_sub(0x848));
    if !valid(world, 0x88) {
        return None;
    }
    let managers = c.bus.read32(world + 0x84);
    if !valid(managers, 0x10) {
        return None;
    }
    let ai = c.bus.read32(managers + 12);
    if !valid(ai, 0x40) {
        return None;
    }
    Some(std::array::from_fn(|slot| {
        let rider = c.bus.read32(ai + 0x28 + slot as u32 * 4);
        if valid(rider, RIDER_TREE + 4) && c.bus.read32(rider + 0x878) == 0 {
            rider
        } else {
            0
        }
    }))
}

fn finish(c: &mut Cpu, stack: u32, pending: Pending) {
    PENDING.lock().unwrap().remove(&stack);
    c.bus.write32(pending.output, u32::from(pending.hit));
    c.bus.write32(pending.output + 4, 0);
    c.bus.write32(pending.output + 8, 1);
    c.g[SP][0] = sx(pending.vm_stack);
}

/// Builtin132: true while any active rider hull penetrates the mode2 trigger.
/// The graph's existing deferred/busy scope supplies shared node ownership.
/// This query does not apply forces, dispatch scripts, or move a rider.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let args = c.g[A2][0] as u32;
    if c.g[A1][0] != 1
        || !valid(args, 16)
        || c.bus.read32(args) != 0
        || c.bus.read32(args + 12) != 1
    {
        return Ok(());
    }
    let rid = c.bus.read32(args + 4);
    let Some(instance) = crate::custom_script_effects::resident_instance(c, rid) else {
        return Ok(());
    };
    let property = c.bus.read32(instance + 0x88);
    if !valid(property, 0x1c)
        || c.bus.read32(property) != 2
        || c.bus.read16(property + 0x1a) != u16::MAX
    {
        return Ok(());
    }
    let Some(riders) = active_riders(c) else {
        return Ok(());
    };
    let vm_stack = c.g[SP][0] as u32;
    let Some(stack) = vm_stack.checked_sub(FRAME).filter(|&at| valid(at, FRAME)) else {
        return Ok(());
    };
    let pending = Pending {
        vm_stack,
        output: c.g[A0][0] as u32,
        instance,
        riders,
        index: 0,
        phase: 0,
        hit: false,
    };
    PENDING.lock().unwrap().insert(stack, pending);
    c.g[SP][0] = sx(stack);
    after(c, back)
}

/// Same Hull constructor/borrow flag/destructor pattern as the native rider's
/// wall query. Native Collide_VolumeObject owns the actual transformed-box
/// intersection; positive contact depth matches the source force-node check.
pub fn after(c: &mut Cpu, back: u32) -> Flow {
    loop {
        let stack = c.g[SP][0] as u32;
        let Some(mut pending) = PENDING.lock().unwrap().get(&stack).copied() else {
            return Ok(());
        };
        match pending.phase {
            0 => {
                while pending.index < pending.riders.len() {
                    let rider = pending.riders[pending.index];
                    let tree = if rider != 0 {
                        c.bus.read32(rider + RIDER_TREE)
                    } else {
                        0
                    };
                    if valid(tree, 0x40) {
                        pending.phase = 1;
                        PENDING.lock().unwrap().insert(stack, pending);
                        c.g[A0][0] = sx(stack);
                        c.g[A1][0] = sx(tree);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0032_f650, back)?;
                        break;
                    }
                    pending.index += 1;
                }
                if pending.index == pending.riders.len() {
                    finish(c, stack, pending);
                    return Ok(());
                }
            }
            1 => {
                pending.phase = 2;
                PENDING.lock().unwrap().insert(stack, pending);
                c.g[A0][0] = sx(stack);
                c.g[A1][0] = sx(pending.instance);
                c.g[A2][0] = sx(stack + CONTACTS);
                c.g[A3][0] = CAPACITY as u64;
                c.g[T0][0] = 1; // non-solid, signed surface-1
                call_ptr(c, 0x0033_4888, back)?;
            }
            2 => {
                let count = if (c.g[V0][0] as i64) > 0 {
                    (c.g[V0][0] as u32).min(CAPACITY)
                } else {
                    0
                };
                pending.hit = (0..count)
                    .any(|i| f32::from_bits(c.bus.read32(stack + CONTACTS + i * 0x80 + 0x40)) > 0.);
                pending.phase = 3;
                PENDING.lock().unwrap().insert(stack, pending);
                c.g[A0][0] = sx(stack);
                c.g[A1][0] = 2; // destroy the embedded tree, retain stack Hull
                call_ptr(c, 0x0032_f708, back)?;
            }
            _ => {
                if pending.hit {
                    finish(c, stack, pending);
                    return Ok(());
                }
                pending.index += 1;
                pending.phase = 0;
                PENDING.lock().unwrap().insert(stack, pending);
            }
        }
    }
}
