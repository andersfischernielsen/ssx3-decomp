//! Imported graph conditions at the native Luno builtin boundary.
//!
//! Random conditions use SSX 3's game RNG. A failed condition branches to
//! its graph's compiler label; native VM control flow owns the actual exit.

use crate::call::{call_ptr, Flow};
use ssx3_ee_rt::cpu::reg::{A0, A1, A2, S4, SP};
use ssx3_ee_rt::Cpu;

fn arguments(c: &mut Cpu, count: u32, at: u32) -> Option<[u32; 4]> {
    if count != 4 || at == 0 { return None }
    let mut values = [0; 4];
    let mut seen = 0;
    for i in 0..count {
        let arg = at + i * 16;
        let key = c.bus.read32(arg);
        let kind = if key == 2 { 2 } else { 1 };
        if key >= 4 || seen & (1 << key) != 0
            || c.bus.read32(arg + 12) != kind { return None }
        seen |= 1 << key;
        values[key as usize] = c.bus.read32(arg + 4);
    }
    Some(values)
}

fn output(c: &mut Cpu, at: u32, allowed: bool) {
    c.bus.write32(at, allowed as u32);
    c.bus.write32(at + 4, 0);
    c.bus.write32(at + 8, 1); // ordinary Luno integer scalar
}

fn trace(args: [u32; 4], allowed: bool, sample: Option<f32>) {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some()) { return }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 96 {
        eprintln!("[tricky] gate {} on {:08x}: {allowed}, value={sample:?}, threshold={}",
            args[0], args[3], f32::from_bits(args[2]));
    }
}

/// Builtin 120: mode, speed comparison, float threshold, current owner RID.
pub fn begin(c: &mut Cpu, back: u32) -> Flow {
    let out = c.g[A0][0] as u32;
    output(c, out, false);
    let Some(args) = arguments(c, c.g[A1][0] as u32, c.g[A2][0] as u32) else { return Ok(()) };
    if args[0] == 1 {
        // AIrandf(0, 1) consumes the existing game RNG stream once and
        // returns [0,1) in F0. finish() writes the native scalar on return.
        c.f[12] = 0;
        c.f[13] = 1.0f32.to_bits();
        return call_ptr(c, 0x0031_7830, back);
    }
    if args[0] == 0 {
        if crate::custom_script_effects::rider(c).is_none() { return Ok(()) }
        // Native WScriptFn_GetRiderSpeed uses the rider's primary velocity
        // interface and original VU XYZ length. It returns a Luno float in
        // this same sret, not in F0. finish() converts only the source units.
        return call_ptr(c, 0x0030_3380, back);
    }
    let allowed = match args[0] {
        2 => crate::custom_script_effects::rider(c)
            .is_some_and(|(_, _, rider)| c.bus.read32(rider + 0x874) != 0),
        3 => crate::custom_script_effects::resident_instance(c, args[3])
            .is_some_and(|instance| c.bus.read32(instance + 12) == 0),
        _ => false,
    };
    output(c, out, allowed);
    trace(args, allowed, None);
    Ok(())
}

/// The native VM's builtin sret is SP+0. Its saved argument base/count are
/// SP+0x7c/+0x80; S4 retains this instruction's argument count across a call.
/// Re-read them before VM cleanup, so a native helper yield needs no host state.
pub fn finish(c: &mut Cpu) {
    let stack = c.g[SP][0] as u32;
    let count = c.g[S4][0] as u32;
    let top = c.bus.read32(stack + 0x80);
    if top < count { return }
    let at = c.bus.read32(stack + 0x7c) + (top - count) * 16;
    let Some(args) = arguments(c, count, at) else { return };
    let threshold = f32::from_bits(args[2]);
    let value = match args[0] {
        0 => {
            if c.bus.read32(stack + 8) != 2 { return }
            let speed = f32::from_bits(c.bus.read32(stack));
            if args[1] == 1 { speed * 0.036 } else { speed }
        }
        1 => f32::from_bits(c.f[0]),
        _ => return,
    };
    let allowed = value.is_finite() && match (args[0], args[1]) {
        (0, 0) => value <= threshold, // source ceiling: raw cm/s
        (0, 1) => value >= threshold, // source floor: km/h
        (1, _) => value < threshold,
        _ => false,
    };
    output(c, stack, allowed);
    trace(args, allowed, Some(value));
}
