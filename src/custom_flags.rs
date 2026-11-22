//! Imported wind flags, constructed and animated by the native Flag manager.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;

fn valid(address: u32, bytes: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000
        && physical
            .checked_add(bytes)
            .is_some_and(|end| end <= 0x0200_0000)
}

/// Builtin130 forwards an explicit native Flag descriptor to retail builtin12.
/// The importer supplies only these seven named fields: owner, two pinned-edge
/// booleans, rate, amplitude, spatial frequency and constant wind blend.
///
/// A Flag hides its static instance before AddFlag searches the fifteen native
/// groups. That search has no failure result. Keep the static quad when all
/// groups are occupied, rather than creating an invisible, unregistered Flag.
/// Source flag models are distinct, so each needs an empty group. Native focus
/// cleanup frees the group; the next focus entry may construct the flag again.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let arguments = c.g[A2][0] as u32;
    if c.g[A1][0] != 7 || !valid(arguments, 7 * 16) {
        return Ok(());
    }
    let mut owner = None;
    let mut seen = 0u32;
    for index in 0..7 {
        let at = arguments + index * 16;
        let key = c.bus.read32(at);
        let value = c.bus.read32(at + 4);
        if !matches!(key, 0 | 1 | 2 | 4 | 5 | 6 | 22)
            || seen & (1 << key) != 0
            || c.bus.read32(at + 12) != if key <= 2 { 1 } else { 2 }
            || (key > 2 && !f32::from_bits(value).is_finite())
            || (matches!(key, 1 | 2) && value > 1)
        {
            return Ok(());
        }
        seen |= 1 << key;
        if key == 0 {
            owner = Some(value);
        }
    }
    let Some(instance) =
        owner.and_then(|rid| crate::custom_script_effects::resident_instance(c, rid))
    else {
        return Ok(());
    };
    // Preserve any existing receiver and let native focus/reset own its life.
    if c.bus.read32(instance + 12) != 0 {
        return Ok(());
    }
    let world = c.bus.read32((c.g[GP][0] as u32).wrapping_sub(0x848));
    if !valid(world, 0x88) {
        return Ok(());
    }
    let managers = c.bus.read32(world + 0x84);
    if !valid(managers, 0x74) {
        return Ok(());
    }
    let manager = c.bus.read32(managers + 0x70);
    if !valid(manager, 0x20 + 15 * 0x188)
        || !(0..15).any(|index| c.bus.read32(manager + 0x20 + index * 0x188 + 0x5c) == 0)
    {
        return Ok(());
    }
    // The native factory owns allocation, descriptor conversion, registration,
    // 8x2 mesh, phase updates, rendering and restorable focus/reset cleanup.
    // Its caller is the ordinary VM return site, including allocation yields.
    call_ptr(c, 0x002f_c9c8, back)
}
