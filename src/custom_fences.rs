//! Imported fence creation and authored adjacency; native Fence owns motion.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

type Course = (&'static str, crate::custom_tracks::EventKind);
const FRAME: u32 = 0x40;

#[derive(Clone, Copy)]
struct Pending {
    vm_stack: u32,
    instance: u32,
    node: u32,
    phase: u8,
}

#[derive(Default)]
struct State {
    course: Option<Course>,
    neighbours: HashMap<u32, [u32; 2]>,
    pending: HashMap<u32, Pending>,
}
static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) {
    let course = crate::custom_tracks::active().map(|track| (track.id, track.event));
    if state.course != course {
        state.course = course;
        state.neighbours.clear();
    }
    // Static RID links survive a restart. Native nodes/motion do not: the
    // ordinary world reset owns those. Pending caller frames end on return.
}

fn valid(address: u32, bytes: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000
        && physical
            .checked_add(bytes)
            .is_some_and(|end| end <= 0x0200_0000)
}

fn args(c: &mut Cpu, registration: bool) -> Option<[u32; 3]> {
    let base = c.g[A2][0] as u32;
    if c.g[A1][0] != 3 || !valid(base, 48) {
        return None;
    }
    let mut values = [0; 3];
    let mut seen = 0;
    for index in 0..3 {
        let at = base + index * 16;
        let key = c.bus.read32(at);
        if key >= 3
            || seen & (1 << key) != 0
            || c.bus.read32(at + 12) != if !registration && key == 2 { 2 } else { 1 }
        {
            return None;
        }
        seen |= 1 << key;
        values[key as usize] = c.bus.read32(at + 4);
    }
    Some(values)
}

/// Builtin129: native owner RID, source next RID, source previous RID.
/// Registration creates no WorldObj and starts no movement.
pub fn register(c: &mut Cpu) {
    let Some(values) = args(c, true) else { return };
    let mut state = STATE.lock().unwrap();
    sync(&mut state);
    if state.course.is_some() {
        state.neighbours.insert(values[0], [values[1], values[2]]);
    }
}

/// Supply the source adjacency after native BuildMesh cleared its links.
/// Also called on each native Fence update for propagation-created nodes.
pub fn prepare(c: &mut Cpu, node: u32) {
    if !valid(node, 0x10c) || c.bus.read16(node + 0x10) != 9 {
        return;
    }
    let instance = c.bus.read32(node + 0x18);
    if !valid(instance, 0x9c) || c.bus.read32(instance + 12) != node {
        return;
    }
    let owner = c.bus.read32(instance + 0x78);
    let links = {
        let mut state = STATE.lock().unwrap();
        sync(&mut state);
        state.neighbours.get(&owner).copied()
    };
    let Some(links) = links else { return };
    for (index, rid) in links.into_iter().enumerate() {
        let other = if rid == u32::MAX {
            0
        } else {
            crate::custom_script_effects::resident_instance(c, rid).unwrap_or(0)
        };
        c.bus.write32(node + 0x104 + index as u32 * 4, other);
    }
}

fn finish(c: &mut Cpu, stack: u32, pending: Pending) {
    STATE.lock().unwrap().pending.remove(&stack);
    c.g[SP][0] = sx(pending.vm_stack);
}

/// Builtin128: construct the first-contact Fence that retail builtin10
/// cannot create. The source initial impulse follows the receiver's local X
/// axis (143550); later native contacts use Fence_v40's contact normal law.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let Some(values) = args(c, false) else {
        return Ok(());
    };
    if values[1] > 1 || !f32::from_bits(values[2]).is_finite() {
        return Ok(());
    }
    let Some(instance) = crate::custom_script_effects::resident_instance(c, values[0]) else {
        return Ok(());
    };
    let Some((manager, _, _)) = crate::custom_script_effects::rider(c) else {
        return Ok(());
    };
    let contact = c.bus.read32(manager + 0x294);
    if !valid(contact, 0x40) {
        return Ok(());
    }
    let node = c.bus.read32(instance + 12);
    if node != 0 && (!valid(node, 0x10c) || c.bus.read16(node + 0x10) != 9) {
        return Ok(());
    }
    let vm_stack = c.g[SP][0] as u32;
    let Some(stack) = vm_stack
        .checked_sub(FRAME)
        .filter(|&stack| valid(stack, FRAME))
    else {
        return Ok(());
    };
    for (index, value) in [u32::MAX, values[1], values[2], 0].into_iter().enumerate() {
        c.bus.write32(stack + index as u32 * 4, value);
    }
    // Source143614..1436a8: normalize the authored local X world axis,
    // dot it with contact travel direction, then multiply abs(speed)*0.001.
    c.lqc2(3, instance + 0x10);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, W, 4, X);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, stack + 0x10);
    c.lqc2(3, stack + 0x10);
    c.lqc2(5, contact + 0x10);
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 12);
    c.lwc1(0, contact + 0x30);
    c.abs_s(0, 0);
    c.f[1] = 0x3a83_126f;
    c.mul_s(0, 0, 1);
    c.mul_s(12, 12, 0);
    c.bus.write32(stack + 0x30, c.f[12]);
    let pending = Pending {
        vm_stack,
        instance,
        node,
        phase: if node == 0 { 0 } else { 3 },
    };
    STATE.lock().unwrap().pending.insert(stack, pending);
    c.g[SP][0] = sx(stack);
    if node != 0 {
        c.g[A0][0] = sx(node);
        c.g[A1][0] = sx(contact);
        call_ptr(c, 0x0034_d650, back)?;
    }
    after(c, back)
}

pub fn after(c: &mut Cpu, back: u32) -> Flow {
    loop {
        let stack = c.g[SP][0] as u32;
        let Some(mut pending) = STATE.lock().unwrap().pending.get(&stack).copied() else {
            return Ok(());
        };
        match pending.phase {
            0 => {
                pending.phase = 1;
                STATE.lock().unwrap().pending.insert(stack, pending);
                c.g[A0][0] = 0x10c;
                c.g[A1][0] = 0x004a_4018;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, back)?;
            }
            1 => {
                pending.node = c.g[V0][0] as u32;
                if !valid(pending.node, 0x10c) {
                    finish(c, stack, pending);
                    return Ok(());
                }
                pending.phase = 2;
                STATE.lock().unwrap().pending.insert(stack, pending);
                c.g[A0][0] = sx(pending.node);
                c.g[A1][0] = 1;
                c.g[A2][0] = sx(pending.instance);
                c.g[A3][0] = sx(stack);
                c.g[T0][0] = 0;
                c.f[12] = c.bus.read32(stack + 0x30);
                call_ptr(c, 0x0034_cab8, back)?;
            }
            _ => {
                prepare(c, pending.node);
                finish(c, stack, pending);
                return Ok(());
            }
        }
    }
}
