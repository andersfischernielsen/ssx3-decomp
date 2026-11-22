//! Optional, bounded native object-query observations. No collision changes.

use ssx3_ee_rt::Cpu;
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex, OnceLock};

fn target() -> Option<u32> {
    static TARGET: OnceLock<Option<u32>> = OnceLock::new();
    *TARGET.get_or_init(|| {
        let value = std::env::var("SSX3_TRACE_OBJECT_VOLUME").ok()?;
        u32::from_str_radix(value.trim_start_matches("0x"), 16).ok()
    })
}

fn vector(c: &mut Cpu, at: u32) -> [f32; 3] {
    std::array::from_fn(|i| f32::from_bits(c.bus.read32(at + i as u32 * 4)))
}

pub fn query(c: &mut Cpu, list: u32, query: u32, mode: u32) {
    let Some(rid) = target() else { return };
    if crate::custom_tracks::active().is_none() { return }
    let Some(instance) = crate::custom_script_effects::resident_instance(c, rid) else { return };
    let low = vector(c, instance + 0x60);
    let high = vector(c, instance + 0x6c);
    // Native query boxes store maximum first (+0x20), minimum at +0x30.
    let qhigh = vector(c, query + 0x20);
    let qlow = vector(c, query + 0x30);
    if (0..3).any(|i| qlow[i] > high[i] + 3000. || qhigh[i] < low[i] - 3000.) { return }
    static SEEN: LazyLock<Mutex<HashSet<(u32, i32)>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
    let mut seen = SEEN.lock().unwrap();
    if seen.len() >= 96 || !seen.insert((mode, ((qlow[2] + qhigh[2]) / 100.) as i32)) { return }
    let count = c.bus.read32(list + 8);
    let present = (0..count.min(128)).any(|i| c.bus.read32(list + 12 + i * 4) == instance);
    eprintln!("[object volume] {rid:08x} mode={mode} present={present}/{count} flags={:08x} query={qlow:?}..{qhigh:?} object={low:?}..{high:?}", c.bus.read32(instance + 8));
}

pub fn candidate(c: &mut Cpu, instance: u32, mode: u32, result: Option<u32>) {
    let Some(rid) = target() else { return };
    if c.bus.read32(instance + 0x78) != rid { return }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 96 {
        eprintln!("[object volume] candidate {rid:08x} mode={mode}, hits={result:?}");
    }
}
