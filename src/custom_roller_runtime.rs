//! Imported Roller properties backed by native ModObj rigid bodies.
//!
//! Source SLUS_203.26: ctor13d6c0, explicit launch13dbe8, contact
//! launch13dd40 and update13e8a8. Geometry, integration, collision and draw
//! matrices remain owned by SSX 3. No shared sphere/material data is changed.

use crate::call::{call_ptr, Flow};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

const FRAME: u32 = 0x200;
const CONTACT_A: u32 = 0x20;
const CONTACT_B: u32 = 0xa0;
const LAUNCH: u32 = 0xe0;
const SPIN: u32 = 0xf0;
const TEMP: u32 = 0x100;
const OUT: u32 = 0x140;
const ARG: u32 = 0x150;
const ROLLER_VTABLE: u32 = 0x0048_f080;

#[derive(Clone, Copy)]
struct Pending {
    vm_stack: u32,
    args: [u32; 7],
    instance: u32,
    node: u32,
    modifier: u32,
    phase: u8,
}

#[derive(Clone, Copy)]
struct Body {
    owner: u32,
    instance: u32,
    node: u32,
    sleeping: bool,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    pending: HashMap<u32, Pending>,
    bodies: HashMap<u32, Body>,
}
static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) {
    let track = crate::custom_tracks::active().map(|t| t.id);
    if state.track != track {
        state.track = track;
        state.bodies.clear();
        // A native return still needs its original VM stack even if a
        // course retires. Pending stack ownership ends only in finish().
    }
}

pub fn restart() { STATE.lock().unwrap().bodies.clear(); }

fn valid(address: u32, size: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000 && physical.checked_add(size).is_some_and(|n| n <= 0x0200_0000)
}

fn object(c: &mut Cpu, instance: u32) -> Option<(u32, u32)> {
    let node = c.bus.read32(instance + 12);
    if !valid(node, 0x2c) || c.bus.read16(node + 0x10) != 17
        || c.bus.read32(node + 0x18) != instance { return None }
    let block = c.bus.read32(node + 0x1c);
    let modifier = if valid(block, 4) { c.bus.read32(block) } else { 0 };
    // A modifier slot/address can be reused after replacement or streaming.
    // A retained Body record must never interpret another modifier's +20 as
    // the Roller's timer. The surrounding ModObj can still be reused.
    let roller = valid(modifier, 0x2d0) && c.bus.read32(modifier) == ROLLER_VTABLE;
    Some((node, if roller { modifier } else { 0 }))
}

fn sphere(c: &mut Cpu, instance: u32) -> Option<u32> {
    let property = c.bus.read32(instance + 0x88);
    if !valid(property, 16) || c.bus.read32(property) != 3 { return None }
    let resource = c.bus.read32(property + 12);
    if !valid(resource, 16) { return None }
    let header = c.bus.read32(resource + 8);
    if !valid(header, 8) || c.bus.read32(header + 4) == 0 { return None }
    let body = c.bus.read32(header);
    valid(body, 0x140).then_some(body)
}

fn trace(message: std::fmt::Arguments<'_>) {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some()) { return }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 96 {
        eprintln!("[tricky] roller {message}");
    }
}

fn args(c: &mut Cpu) -> Option<[u32; 7]> {
    if c.g[A1][0] != 7 || c.g[A2][0] == 0 { return None }
    let at = c.g[A2][0] as u32;
    let mut result = [0; 7];
    let mut seen = 0;
    for i in 0..7 {
        let a = at + i * 16;
        let key = c.bus.read32(a);
        if key >= 7 || seen & (1 << key) != 0
            || c.bus.read32(a + 12) != if key == 0 { 1 } else { 2 } { return None }
        seen |= 1 << key;
        result[key as usize] = c.bus.read32(a + 4);
    }
    result[1..].iter().all(|v| f32::from_bits(*v).is_finite()).then_some(result)
}

/// Imported builtin122: RID, mass, source U1/U2, authored XYZ launch.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let Some(args) = args(c) else { return Ok(()) };
    if f32::from_bits(args[1]) <= 0. { return Ok(()) }
    let Some(instance) = crate::custom_script_effects::resident_instance(c, args[0]) else { return Ok(()) };
    if let Some((node, modifier)) = object(c, instance) {
        let mut state = STATE.lock().unwrap();
        sync(&mut state);
        if let Some(body) = state.bodies.get_mut(&modifier) {
            if body.owner == args[0] && body.node == node && body.instance == instance {
                // Source dispatch13c634..13c658 resumes a paused Roller
                // through149e10, without reconstructing or relaunching it.
                // Source Stop resets its elapsed time before pausing. The
                // native modifier encodes that pause as timer11 instead.
                // Direct native contacts already reset it in361cd8; this
                // also covers ActOn/deferred retriggers on another owner.
                if f32::from_bits(c.bus.read32(modifier + 0x20)) >= 11. {
                    c.bus.write32(modifier + 0x20, 0);
                    body.sleeping = false;
                    trace(format_args!("{:08x} resumed at native pose", body.owner));
                }
                return Ok(());
            }
        }
    }
    // A live body owns its copied sphere tree. Only a new construction
    // needs the source sphere resource to be resident again.
    let Some(mut shape) = sphere(c, instance) else {
        trace(format_args!("{:08x}: native sphere body unavailable", args[0]));
        return Ok(());
    };
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if !valid(manager, 0x29c) { return Ok(()) }
    let contact_a = c.bus.read32(manager + 0x298);
    let contact_b = c.bus.read32(manager + 0x294);
    // A direct contact retains the actually hit sphere part. A scripted
    // target resolves its own first body, never the originating can's body.
    if valid(contact_a, 0x80) && c.bus.read32(contact_a + 0x50) == instance {
        let hit = c.bus.read32(contact_a + 0x48);
        if valid(hit, 0x140) { shape = hit; }
    }
    let copied_b: [u32; 16] = if valid(contact_b, 64) {
        std::array::from_fn(|i| c.bus.read32(contact_b + i as u32 * 4))
    } else { [0; 16] };
    let vm_stack = c.g[SP][0] as u32;
    let stack = vm_stack - FRAME;
    for i in 0..FRAME / 4 { c.bus.write32(stack + i * 4, 0); }
    for (i, value) in copied_b.into_iter().enumerate() {
        c.bus.write32(stack + CONTACT_B + i as u32 * 4, value);
    }
    for (i, value) in args[4..].iter().enumerate() {
        c.bus.write32(stack + LAUNCH + i as u32 * 4, *value);
    }
    for (i, value) in args[..4].iter().enumerate() { c.bus.write32(stack + i as u32 * 4, *value); }
    c.bus.write32(stack + CONTACT_A + 0x48, shape);
    c.bus.write32(stack + CONTACT_A + 0x50, instance);
    // The native Object factory consumes this ordinary named RID argument.
    c.bus.write32(stack + ARG, 0);
    c.bus.write32(stack + ARG + 4, args[0]);
    c.bus.write32(stack + ARG + 12, 1);
    let mut state = STATE.lock().unwrap();
    sync(&mut state);
    state.pending.insert(stack, Pending { vm_stack, args, instance, node: 0, modifier: 0, phase: 0 });
    drop(state);
    c.g[SP][0] = sx(stack);
    after(c, back)
}

fn finish(c: &mut Cpu, stack: u32, pending: Pending) {
    STATE.lock().unwrap().pending.remove(&stack);
    c.g[SP][0] = sx(pending.vm_stack);
}

/// Run/resume native calls at the VM's existing builtin return site. Our
/// private caller frame remains below the VM frame until all calls finish.
pub fn after(c: &mut Cpu, back: u32) -> Flow {
    loop {
        let stack = c.g[SP][0] as u32;
        let Some(mut pending) = STATE.lock().unwrap().pending.get(&stack).copied() else { return Ok(()) };
        match pending.phase {
            0 => {
                pending.phase = 1;
                STATE.lock().unwrap().pending.insert(stack, pending);
                if object(c, pending.instance).is_none() {
                    c.g[A0][0] = sx(stack + OUT);
                    c.g[A1][0] = 1;
                    c.g[A2][0] = sx(stack + ARG);
                    call_ptr(c, 0x002f_c0d0, back)?;
                }
            }
            1 => {
                let Some((node, _)) = object(c, pending.instance) else {
                    trace(format_args!("{:08x}: no native ModObj receiver", pending.args[0]));
                    finish(c, stack, pending);
                    return Ok(());
                };
                pending.node = node;
                pending.phase = 2;
                STATE.lock().unwrap().pending.insert(stack, pending);
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(stack);
                c.g[A2][0] = sx(stack + CONTACT_A);
                c.g[A3][0] = sx(stack + CONTACT_B);
                call_ptr(c, 0x0035_5db8, back)?;
            }
            2 => {
                let Some((node, modifier)) = object(c, pending.instance) else {
                    finish(c, stack, pending); return Ok(());
                };
                if node != pending.node || !valid(modifier, 0x2d0) {
                    finish(c, stack, pending); return Ok(());
                }
                let mass_data = c.bus.read32(modifier + 0x178);
                if !valid(mass_data, 0x80) { finish(c, stack, pending); return Ok(()); }
                // Tricky ctor13d800 scales the original inverse tensor by
                // 0.075. Re-read the immutable input, avoiding a rounded
                // rescale of the native constructor's different0.5 factor.
                for i in 0..9 {
                    let raw = FpuF32(c.bus.read32(mass_data + 0x5c + i * 4));
                    c.bus.write32(modifier + 0x280 + i * 4, (raw * FpuF32(0x3d99_999a)).0);
                }
                pending.modifier = modifier;
                pending.phase = 3;
                STATE.lock().unwrap().pending.insert(stack, pending);
                c.g[A0][0] = sx(modifier);
                call_ptr(c, 0x0035_e248, back)?;
            }
            3 => {
                let modifier = pending.modifier;
                let explicit = pending.args[4..].iter().any(|v| f32::from_bits(*v) != 0.);
                if explicit {
                    explicit_launch(c, stack, modifier);
                } else {
                    // Original contact law and native InitBody agree. Only
                    // angular momentum must be rebuilt for the source tensor.
                    c.g[AT][0] = 0x4120_0000;
                    c.qmtc2(AT, 3);
                    c.lqc2(4, stack + CONTACT_B + 0x20);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, stack + SPIN);
                }
                pending.phase = 4;
                STATE.lock().unwrap().pending.insert(stack, pending);
                c.g[A0][0] = sx(modifier + 0x30);
                c.g[A1][0] = sx(stack + SPIN);
                call_ptr(c, 0x0035_cff0, back)?;
            }
            4 => {
                energy(c, pending.modifier);
                let mut state = STATE.lock().unwrap();
                sync(&mut state);
                state.bodies.insert(pending.modifier, Body { owner: pending.args[0],
                    instance: pending.instance, node: pending.node, sleeping: false });
                drop(state);
                let velocity: [f32; 3] = std::array::from_fn(|i|
                    f32::from_bits(c.bus.read32(pending.modifier + 0xb0 + i as u32 * 4)));
                trace(format_args!("{:08x} body {:08x}, velocity {velocity:?}", pending.args[0], pending.modifier));
                finish(c, stack, pending);
                return Ok(());
            }
            _ => { finish(c, stack, pending); return Ok(()); }
        }
    }
}

fn explicit_launch(c: &mut Cpu, stack: u32, modifier: u32) {
    // Original13dbe8: raw authored velocity, p=v/inverseMass,
    // angular velocity=-10*normalize(v). No guessed fixed launch speed.
    c.lwc1(0, modifier + 0x70);
    c.mfc1(V0, 0);
    c.qmtc2(V0, 3);
    c.vu0().div(0, W, 3, X);
    c.lqc2(4, stack + LAUNCH);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, modifier + 0x50);
    c.sqc2(4, modifier + 0xb0);
    c.g[AT][0] = 0xbf80_0000;
    c.qmtc2(AT, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, stack + TEMP);
    c.lqc2(3, stack + TEMP);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, W, 4, X);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, stack + TEMP);
    c.g[AT][0] = 0x4120_0000;
    c.qmtc2(AT, 3);
    c.lqc2(4, stack + TEMP);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, stack + SPIN);
}

fn energy(c: &mut Cpu, modifier: u32) {
    for (left, right, out) in [(0xb0, 0x50, 0), (0xc0, 0x60, 1)] {
        c.lqc2(3, modifier + left);
        c.lqc2(5, modifier + right);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(V0, 4);
        c.mtc1(V0, out);
    }
    c.bus.write32(modifier + 4, (FpuF32(c.f[0]) + FpuF32(c.f[1])).0);
}

/// Original Tricky update13e8a8 pauses its node when quiet/after10s.
/// Keep that last pose; the retail SSX3 boulder's later16s drop is unrelated.
pub fn sleeping(c: &mut Cpu, modifier: u32) -> bool {
    let mut state = STATE.lock().unwrap();
    sync(&mut state);
    let Some(mut body) = state.bodies.get(&modifier).copied() else { return false };
    let live = crate::custom_script_effects::resident_instance(c, body.owner) == Some(body.instance)
        && object(c, body.instance) == Some((body.node, modifier));
    if !live { state.bodies.remove(&modifier); return false }
    let sleeping = f32::from_bits(c.bus.read32(modifier + 0x20)) >= 11.;
    if sleeping && !body.sleeping { trace(format_args!("{:08x} sleeping at native pose", body.owner)); }
    body.sleeping = sleeping;
    state.bodies.insert(modifier, body);
    sleeping
}
