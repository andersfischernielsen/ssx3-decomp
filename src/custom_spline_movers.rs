//! Detached Tricky copies driven by native Object/MultiSplineModifier actors.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

const FRAME: u32 = 0x100;

#[derive(Clone, Copy)]
struct Pending {
    vm_stack: u32,
    output: u32,
    arguments: u32,
    instance: u32,
    prototype: u32,
    flags: u32,
    phase: u8,
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

fn finish(c: &mut Cpu, stack: u32, pending: Pending) {
    PENDING.lock().unwrap().remove(&stack);
    c.g[SP][0] = sx(pending.vm_stack);
}

/// Builtin131 takes the twelve native MultiSpline fields plus prototype RID12.
/// Parameter0 is a dedicated initially invisible host, rather than Tricky's
/// prototype: the latter retains its authored Debounce/no-node gate.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let arguments = c.g[A2][0] as u32;
    if c.g[A1][0] != 13 || !valid(arguments, 13 * 16) {
        return Ok(());
    }
    let mut values = [0; 13];
    let mut seen = 0u32;
    for index in 0..13 {
        let at = arguments + index * 16;
        let key = c.bus.read32(at);
        let value = c.bus.read32(at + 4);
        if key >= 13
            || seen & (1 << key) != 0
            || c.bus.read32(at + 12)
                != if matches!(key, 0 | 1 | 2 | 10 | 11 | 12) {
                    1
                } else {
                    2
                }
            || ((3..=9).contains(&key) && !f32::from_bits(value).is_finite())
        {
            return Ok(());
        }
        seen |= 1 << key;
        values[key as usize] = value;
    }
    if values[1] == u32::MAX || values[2] != 2 || values[10] > 1 || !(1..=64).contains(&values[11])
    {
        return Ok(());
    }
    let Some(instance) = crate::custom_script_effects::resident_instance(c, values[0]) else {
        return Ok(());
    };
    let Some(prototype) = crate::custom_script_effects::resident_instance(c, values[12]) else {
        return Ok(());
    };
    if instance == prototype {
        return Ok(());
    }
    // Several pylons can address the same chair prototype. A live host already
    // owns its native copies and must not acquire a second transform modifier.
    if c.bus.read32(instance + 12) != 0 {
        return Ok(());
    }
    let vm_stack = c.g[SP][0] as u32;
    let Some(stack) = vm_stack.checked_sub(FRAME).filter(|&at| valid(at, FRAME)) else {
        return Ok(());
    };
    // Object(factory0): explicit host, force an owned dynamic instance. Its
    // default special-contact mode remains zero; the host has no collider.
    for (index, (key, value)) in [(0, values[0]), (2, 1)].into_iter().enumerate() {
        let at = stack + index as u32 * 16;
        for (offset, word) in [(0, key), (4, value), (8, 0), (12, 1)] {
            c.bus.write32(at + offset, word);
        }
    }
    // Canonicalize the twelve native records, keeping adapter-only RID12 out
    // of the factory's opaque tail word even if Luno supplied keys out of order.
    for key in 0..12u32 {
        let at = stack + 0x40 + key * 16;
        let kind = if matches!(key, 0 | 1 | 2 | 10 | 11) {
            1
        } else {
            2
        };
        for (offset, word) in [(0, key), (4, values[key as usize]), (8, 0), (12, kind)] {
            c.bus.write32(at + offset, word);
        }
    }
    let flags = c.bus.read32(instance + 8);
    let pending = Pending {
        vm_stack,
        output: c.g[A0][0] as u32,
        arguments: stack + 0x40,
        instance,
        prototype,
        flags,
        phase: 0,
    };
    PENDING.lock().unwrap().insert(stack, pending);
    c.g[SP][0] = sx(stack);
    after(c, back)
}

/// Keep the private Object argument records alive across native allocation
/// suspension, then forward the original MultiSpline descriptor unchanged.
pub fn after(c: &mut Cpu, back: u32) -> Flow {
    loop {
        let stack = c.g[SP][0] as u32;
        let Some(mut pending) = PENDING.lock().unwrap().get(&stack).copied() else {
            return Ok(());
        };
        match pending.phase {
            0 => {
                pending.phase = 1;
                PENDING.lock().unwrap().insert(stack, pending);
                c.g[A0][0] = sx(stack + 0x30);
                c.g[A1][0] = 2;
                c.g[A2][0] = sx(stack);
                call_ptr(c, 0x002f_c0d0, back)?;
            }
            1 => {
                let node = c.bus.read32(pending.instance + 12);
                if !valid(node, 0x2c) || c.bus.read16(node + 0x10) != 0x11 {
                    c.bus.write32(pending.instance + 8, pending.flags);
                    finish(c, stack, pending);
                    return Ok(());
                }
                // Native clones inherit their template's draw flag. Set it
                // only after Object owns the host, so no static prototype is
                // exposed before its movement process has been constructed.
                let flags = c.bus.read32(pending.instance + 8);
                c.bus.write32(pending.instance + 8, flags | 1);
                pending.phase = 2;
                PENDING.lock().unwrap().insert(stack, pending);
                c.g[A0][0] = sx(pending.output);
                c.g[A1][0] = 12;
                c.g[A2][0] = sx(pending.arguments);
                call_ptr(c, 0x002f_e0c0, back)?;
            }
            _ => {
                let node = c.bus.read32(pending.instance + 12);
                let block = if valid(node, 0x2c) {
                    c.bus.read32(node + 0x1c)
                } else {
                    0
                };
                let transform = if valid(block, 4) {
                    c.bus.read32(block)
                } else {
                    0
                };
                if valid(transform, 0x58) && c.bus.read32(transform) == 0x0048_f168 {
                    // Source ctor1fa440..45c suppresses both static and node
                    // draw on the original template; Debounce stays attached.
                    let flags = c.bus.read32(pending.prototype + 8);
                    c.bus.write32(pending.prototype + 8, flags & !6);
                } else {
                    c.bus.write32(pending.instance + 8, pending.flags);
                }
                finish(c, stack, pending);
                return Ok(());
            }
        }
    }
}
