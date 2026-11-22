//! Authored Tricky rail candidacy, consumed by SSX 3's ordinary rail queries.
//!
//! Native WorldScript relocation copies initial flags to both halves of
//! WorldSpline+0x1c. Queries mask the low flags. Preserve the upper baseline
//! and the surface at +0x28; no spline geometry or spatial links change.

use ssx3_ee_rt::cpu::reg::{A1, A2, GP};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
struct Rails {
    track: Option<&'static str>,
    // None restores the initial state after a restart, when resident again.
    states: HashMap<u32, Option<bool>>,
}

static RAILS: LazyLock<Mutex<Rails>> = LazyLock::new(|| Mutex::new(Rails::default()));

fn sync(rails: &mut Rails) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if rails.track != track {
        *rails = Rails { track, ..Rails::default() };
    }
    track.is_some()
}

fn resident(c: &mut Cpu, rid: u32) -> Option<u32> {
    let world = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0x16c8));
    if world == 0 { return None }
    let cache = c.bus.read32(world);
    if cache == 0 { return None }
    let banks = c.bus.read32(cache + 8);
    if banks == 0 { return None }
    let bank = c.bus.read32(banks + (rid & 255) * 4);
    if bank == 0 || rid >> 8 >= c.bus.read32(bank + 0x48) { return None }
    let table = c.bus.read32(bank + 0x44);
    if table == 0 { return None }
    let reference = c.bus.read32(table + (rid >> 8) * 4);
    let spline = (reference >> 8) << 2;
    (reference & 255 != 0 && spline != 0
        && spline != (c.g[GP][0] as u32).wrapping_add(0x16e0)).then_some(spline)
}

pub fn restart() {
    let mut rails = RAILS.lock().unwrap();
    if !sync(&mut rails) { return }
    for state in rails.states.values_mut() { *state = None; }
}

/// Keep switches through section streaming and teleports. Native instance
/// reset (0x3a6800) does not visit splines, so restore their baseline explicitly
/// on a course restart, including any which are temporarily nonresident.
pub fn update(c: &mut Cpu) {
    let mut rails = RAILS.lock().unwrap();
    if !sync(&mut rails) { return }
    rails.states.retain(|&rid, state| {
        let Some(spline) = resident(c, rid) else { return true };
        let flags = c.bus.read32(spline + 0x1c);
        let current = match state {
            Some(enabled) => (flags & 0xffff & !3) | if *enabled { 3 } else { 0 },
            None => flags >> 16,
        };
        let updated = (flags & 0xffff0000) | current;
        if flags != updated {
            c.bus.write32(spline + 0x1c, updated);
            if std::env::var_os("SSX3_OBJECT_TRACE").is_some() {
                eprintln!("[tricky] rail {rid:08x} candidacy {flags:08x} -> {updated:08x}");
            }
        }
        state.is_some()
    });
}

/// Imported Luno builtin 119: compact spline RID, enabled (0/1).
pub fn builtin(c: &mut Cpu) {
    if c.g[A1][0] != 2 { return }
    let args = c.g[A2][0] as u32;
    if args == 0 || c.bus.read32(args) != 0 || c.bus.read32(args + 16) != 1 { return }
    let rid = c.bus.read32(args + 4);
    let enabled = c.bus.read32(args + 20);
    if enabled > 1 { return }
    {
        let mut rails = RAILS.lock().unwrap();
        if !sync(&mut rails) { return }
        rails.states.insert(rid, Some(enabled != 0));
    }
    update(c);
}
