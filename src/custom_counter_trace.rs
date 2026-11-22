//! Bounded observations of native counter state during windowed development.

use ssx3_ee_rt::Cpu;

fn enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    crate::custom_tracks::active().is_some()
        && *ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some())
}

pub fn before(c: &mut Cpu, node: u32) -> Option<(i32, u32)> {
    enabled().then(|| (c.bus.read32(node + 0x1c) as i32, c.bus.read32(node + 0x20)))
}

fn owner(c: &mut Cpu, node: u32) -> u32 {
    let instance = c.bus.read32(node + 0x18);
    if instance == 0 { 0 } else { c.bus.read32(instance + 0x78) }
}

pub fn event(c: &mut Cpu, node: u32, message: u64, level: u32, before: Option<(i32, u32)>) {
    let Some((count, marks)) = before else { return };
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 128 { return }
    let after = (c.bus.read32(node + 0x1c) as i32, c.bus.read32(node + 0x20));
    let rid = owner(c, node);
    eprintln!("[tricky] counter {rid:08x} message {message}, level {}, count {count}->{}, marks {marks:x}->{:x}",
        f32::from_bits(level), after.0, after.1);
}

pub fn completed(c: &mut Cpu, node: u32) {
    if !enabled() { return }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 32 {
        let rid = owner(c, node);
        eprintln!("[tricky] counter {rid:08x} completed: native end-slot dispatch");
    }
}
