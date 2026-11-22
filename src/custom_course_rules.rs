//! Authored Tricky velocity volumes and rider-local circuit pass countdown.
//!
//! TFX2 files are derived from SSF/AIP/SOP and the executable clock catalogue,
//! alongside each BAM (legacy TFX1 is readable too). They
//! contain no renderer settings. Native SSX 3 still owns rider physics,
//! contacts, pickup scripts, finish handling and the race clock.

use ssx3_ee_rt::Cpu;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

#[derive(Clone)]
struct Volume {
    kind: u32,
    mode: u32,
    f: [f32; 32],
}
#[derive(Default)]
struct Contact {
    inside: bool,
    retired: bool,
    serviced: bool,
    elapsed: f32,
}
struct Rider {
    remaining: u32,
    lap_ready: bool,
    last_cross: Option<u32>,
    previous: Option<[f32; 3]>,
    stage: usize,
    contacts: Vec<Contact>,
    time_gates: HashSet<u32>,
}
#[derive(Default)]
struct Rules {
    root: Option<PathBuf>,
    track: &'static str,
    event: Option<crate::custom_tracks::EventKind>,
    passes: u32,
    volumes: Vec<Volume>,
    riders: HashMap<u32, Rider>,
    showoff_hundredths: Option<u32>,
    timer_seeded: bool,
}
pub struct BoostUpdate {
    pub take_off: bool,
}
static RULES: LazyLock<Mutex<Rules>> = LazyLock::new(|| {
    Mutex::new(Rules {
        passes: 1,
        ..Rules::default()
    })
});

pub fn set_root(root: Option<PathBuf>) {
    *RULES.lock().unwrap() = Rules {
        root,
        passes: 1,
        ..Rules::default()
    };
}
pub fn restart() {
    ssx3_render::diagnostics::restart();
    crate::custom_finish_trace::restart();
    crate::custom_results_enter::restart();
    crate::custom_rail_diagnostics::restart();
    crate::custom_script_queue::restart();
    let mut rules = RULES.lock().unwrap();
    rules.riders.clear();
    rules.timer_seeded = false;
}

/// A native warp changes position discontinuously. Preserve lap progress and
/// awarded time gates, while restarting only the spatial-volume contact sweep.
pub fn relocated(address: u32) {
    let mut rules = RULES.lock().unwrap();
    if let Some(rider) = rules.riders.get_mut(&address) {
        rider.previous = None;
        rider.stage = 0;
        for contact in &mut rider.contacts {
            *contact = Contact::default();
        }
    }
}

/// Distance queries used by standings and AI must include later circuits.
/// The raw rider fields stay local to their TrackPath for event projection.
pub fn distance(c: &mut Cpu, address: u32, local: f32) -> f32 {
    if crate::custom_tracks::active().is_none() {
        return local;
    }
    let mut rules = RULES.lock().unwrap();
    if !load(&mut rules) || rules.passes <= 1 {
        return local;
    }
    let extra = rules
        .riders
        .get(&address)
        .map_or(rules.passes - 1, |rider| {
            // While in the tube the local distance still approaches the old
            // finish. The altitude marker resets it to a full circuit and rearms
            // the gate, so the extra-circuit count changes at that same point.
            rider.remaining.saturating_sub(u32::from(rider.lap_ready))
        });
    let path = c.bus.read32(0x004d_33b4);
    if path == 0 {
        return local;
    }
    local + extra as f32 * float(c, path + 0x38)
}

fn word(data: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(data[at..at + 4].try_into().unwrap())
}

fn load(rules: &mut Rules) -> bool {
    let Some(track) = crate::custom_tracks::active() else {
        return false;
    };
    if rules.track == track.id && rules.event == Some(track.event) {
        return true;
    }
    rules.track = track.id;
    rules.event = Some(track.event);
    rules.riders.clear();
    rules.volumes.clear();
    rules.passes = 1;
    rules.showoff_hundredths = None;
    rules.timer_seeded = false;
    let Some(root) = &rules.root else {
        return false;
    };
    let mode = if track.event == crate::custom_tracks::EventKind::Race {
        "race"
    } else {
        "freestyle"
    };
    let path = root.join(format!(
        "DATA/WORLDS/TRICKY/{}-{mode}.FX",
        track.world_location
    ));
    let Ok(data) = std::fs::read(&path) else {
        return false;
    };
    load_data(rules, &data)
}

fn load_data(rules: &mut Rules, data: &[u8]) -> bool {
    if data.len() < 16 || !matches!(&data[..4], b"TFX1" | b"TFX2") {
        return false;
    }
    let header = if &data[..4] == b"TFX2" { 24 } else { 16 };
    if data.len() < header {
        return false;
    }
    let passes = word(&data, 4);
    let count = word(&data, 8) as usize;
    if !(1..=16).contains(&passes) || count > 4096 || data.len() != header + count * 144 {
        return false;
    }
    let mut volumes = Vec::with_capacity(count);
    for record in data[header..].chunks_exact(144) {
        let mut f = [0.; 32];
        for (i, value) in f.iter_mut().enumerate() {
            *value = f32::from_bits(word(record, 16 + i * 4));
        }
        let kind = word(record, 4);
        if !matches!(kind, 7 | 15 | 18 | 24) || !f.iter().all(|v| v.is_finite()) {
            return false;
        }
        volumes.push(Volume {
            kind,
            mode: word(record, 8),
            f,
        });
    }
    // Circuit Showoff events use the authored pass limit too. Tokyo's four
    // tube entries comprise the initial ascent followed by three full laps.
    rules.passes = passes;
    rules.volumes = volumes;
    if header == 24 && rules.event == Some(crate::custom_tracks::EventKind::Freestyle) {
        let hundredths = word(&data, 12);
        if hundredths > 30000 {
            return false;
        }
        rules.showoff_hundredths = Some(hundredths);
    }
    // A lap mouth must make its entry decision before overlapping lift nodes.
    rules.volumes.sort_by_key(|v| match v.kind {
        15 => 0,
        18 => 1,
        24 => 2,
        _ => 3,
    });
    eprintln!(
        "[tricky] {}: {} authored boost volumes, {} passes",
        rules.track, count, passes
    );
    true
}

fn float(c: &mut Cpu, at: u32) -> f32 {
    f32::from_bits(c.bus.read32(at))
}
fn put(c: &mut Cpu, at: u32, value: f32) {
    c.bus.write32(at, value.to_bits());
}
fn vec(c: &mut Cpu, at: u32) -> [f32; 3] {
    [float(c, at), float(c, at + 4), float(c, at + 8)]
}

/// The native event entity holds an absolute deadline in game ticks at +0x78;
/// +0x88 enables expiry. The AI's +0x0c remains the unmodified elapsed clock.
fn event_entity(c: &mut Cpu) -> Option<(u32, u32)> {
    let app = c.bus.read32(0x004a_28a8);
    if app == 0 {
        return None;
    }
    let module = c.bus.read32(app + 0x84);
    let entity = c.bus.read32(app + 0xc0);
    if module == 0 || entity == 0 {
        return None;
    }
    let ai = c.bus.read32(module + 0xc);
    if ai == 0 {
        return None;
    }
    Some((entity, ai))
}

fn seed_timer(c: &mut Cpu, rules: &mut Rules) {
    if rules.timer_seeded {
        return;
    }
    let Some(hundredths) = rules.showoff_hundredths else {
        return;
    };
    let Some((entity, ai)) = event_entity(c) else {
        return;
    };
    if !matches!(c.bus.read32(ai), 4 | 5) {
        return;
    }
    let ticks = (u64::from(hundredths) * u64::from(crate::tick::rate(c)) / 100) as u32;
    c.bus.write32(entity + 0x78, ticks);
    c.bus.write32(entity + 0x88, u32::from(hundredths != 0));
    rules.timer_seeded = true;
    eprintln!(
        "[tricky] native Showoff clock: {} hundredths, {} ticks",
        hundredths, ticks
    );
}

fn crosses_box(a: [f32; 3], b: [f32; 3], lo: &[f32], hi: &[f32]) -> bool {
    let (mut begin, mut end) = (0f32, 1f32);
    for axis in 0..3 {
        let delta = b[axis] - a[axis];
        if delta.abs() < 1e-6 {
            if a[axis] < lo[axis] || a[axis] > hi[axis] {
                return false;
            }
        } else {
            let x = (lo[axis] - a[axis]) / delta;
            let y = (hi[axis] - a[axis]) / delta;
            begin = begin.max(x.min(y));
            end = end.min(x.max(y));
            if begin > end {
                return false;
            }
        }
    }
    true
}

fn push(v: &mut [f32; 3], axis: &[f32], speed: f32, rate: f32, dt: f32) {
    let length = axis.iter().map(|x| x * x).sum::<f32>().sqrt();
    if length <= 1e-6 {
        return;
    }
    let along = (0..3).map(|i| v[i] * axis[i] / length).sum::<f32>();
    let gain = (speed * 100. - along).max(0.) * (rate * dt).clamp(0., 1.);
    for i in 0..3 {
        v[i] += axis[i] / length * gain;
    }
}

fn count_pass(rider: &mut Rider, address: u32, now: u32, debounce: u32) {
    if rider.lap_ready
        && rider.remaining > 0
        && rider
            .last_cross
            .is_none_or(|last| now.wrapping_sub(last) >= debounce)
    {
        rider.remaining -= 1;
        rider.lap_ready = false;
        rider.last_cross = Some(now);
        eprintln!(
            "[tricky] rider {address:#x}: {} passes remaining",
            rider.remaining
        );
    }
}

/// Apply source volumes once per native rider update, including AI riders.
pub fn update(c: &mut Cpu, address: u32) -> Option<BoostUpdate> {
    if crate::custom_tracks::active().is_none() {
        return None;
    }
    let mut rules = RULES.lock().unwrap();
    if !load(&mut rules) {
        return None;
    }
    update_loaded(c, address, &mut rules)
}

fn update_loaded(c: &mut Cpu, address: u32, rules: &mut Rules) -> Option<BoostUpdate> {
    seed_timer(c, rules);
    if float(c, address + 0x470) >= 0. {
        // Native OnFinishRace starts this timer. Authored fans/conveyors
        // stop affecting that rider once the finish presentation owns it.
        if let Some(rider) = rules.riders.get_mut(&address) {
            rider.previous = None;
            rider.stage = 0;
            for contact in &mut rider.contacts {
                *contact = Contact::default();
            }
        }
        return None;
    }
    if rules.volumes.is_empty() {
        return None;
    }
    let passes = rules.passes;
    let Rules {
        volumes, riders, ..
    } = &mut *rules;
    let circuit = volumes.iter().any(|volume| volume.kind == 15);
    let rider = riders.entry(address).or_insert_with(|| Rider {
        remaining: passes,
        lap_ready: true,
        last_cross: None,
        previous: None,
        stage: 0,
        contacts: (0..volumes.len()).map(|_| Contact::default()).collect(),
        time_gates: HashSet::new(),
    });
    // Native main state 3 relocates the rider during reset. Do not sweep the
    // old-to-respawn segment through unrelated lifts or the Tokyo lap mouth.
    let owner = c.bus.read32(address + 0x77c);
    if owner != 0 && c.bus.read32(owner + 0xde0) == 3 {
        rider.previous = None;
        rider.stage = 0;
        for contact in &mut rider.contacts {
            *contact = Contact::default();
        }
        return None;
    }
    let position = vec(c, address + 0x110);
    let previous = rider.previous.unwrap_or(position);
    rider.previous = Some(position);
    let mut velocity = vec(c, address + 0x1e0);
    let mut changed = false;
    let dt = 1. / crate::tick::rate(c) as f32;
    let now = crate::tick::vsync_count(c);
    for (index, volume) in volumes.iter().enumerate() {
        let f = &volume.f;
        let inside = crosses_box(previous, position, &f[8..11], &f[12..15]);
        if !inside {
            rider.contacts[index] = Contact::default();
            continue;
        }
        let entering = !rider.contacts[index].inside;
        if entering && volume.kind == 15 {
            if passes > 1 {
                count_pass(rider, address, now, 180);
            }
            rider.contacts[index].serviced = passes <= 1 || rider.remaining != 0;
            rider.stage = if position[2] <= f[10] + 1000. {
                0
            } else {
                let side = (position[0] - (f[8] + f[12]) * 0.5) * f[16]
                    + (position[1] - (f[9] + f[13]) * 0.5) * f[17];
                if side < 0. {
                    1
                } else {
                    2
                }
            };
        }
        let contact = &mut rider.contacts[index];
        contact.inside = true;
        let suppressed = volume.mode >= 2 || (volume.mode == 0 && contact.elapsed < f[2]);
        contact.elapsed += dt;
        if contact.retired {
            continue;
        }
        match volume.kind {
            15 if contact.serviced => {
                push(&mut velocity, &f[4..7], f[1], f[0], dt);
                velocity[0] = 0.;
                velocity[1] = 0.;
                changed = true;
            }
            18 if position[2] < f[19] => {
                if f[19] - position[2] < f[3] {
                    // The authored finish lift snaps to its target altitude;
                    // do not sweep that relocation through every course volume.
                    for offset in [0x110, 0x1d0] {
                        put(c, address + offset + 8, f[19]);
                    }
                    rider.previous = Some([position[0], position[1], f[19]]);
                    changed = true;
                    rider.lap_ready = true;
                    rider.time_gates.clear();
                    contact.retired = true;
                    let first_path = c.bus.read32(0x004d_33b4);
                    if circuit && first_path != 0 {
                        c.bus.write32(address + 0xab4, first_path);
                        let distance = float(c, first_path + 0x38);
                        put(c, address + 0x4d0, distance);
                        put(c, address + 0x4d4, distance);
                        let cursor = c.bus.read32(address + 0xac0);
                        if cursor != 0 {
                            c.bus.write32(cursor + 0x14, u32::MAX);
                        }
                    }
                } else {
                    push(&mut velocity, &f[4..7], f[1], f[0], dt);
                    velocity[0] = 0.;
                    velocity[1] = 0.;
                    changed = true;
                }
            }
            18 => contact.retired = true,
            24 if !suppressed => {
                let start = 20 + rider.stage * 4;
                push(&mut velocity, &f[start..start + 3], f[start + 3], f[0], dt);
                changed = true;
            }
            7 if !suppressed => {
                push(&mut velocity, &f[4..7], f[1], f[0], dt);
                changed = true;
            }
            _ => {}
        }
    }
    if changed {
        for i in 0..3 {
            put(c, address + 0x1e0 + i * 4, velocity[i as usize]);
        }
    }
    let owner = c.bus.read32(address + 0x77c);
    changed.then(|| BoostUpdate {
        take_off: velocity[2] > 0. && owner != 0 && c.bus.read32(owner + 0xde0) == 0,
    })
}

/// Native event indices: stored key 12 becomes 5; finish key 0 becomes 1.
/// Returning true suppresses a finish until the source pass countdown is zero.
pub fn path_event(
    c: &mut Cpu,
    rider_address: u32,
    event: u32,
    entering: bool,
    active: bool,
) -> bool {
    if crate::custom_tracks::active().is_none() {
        return false;
    }
    let kind = c.bus.read32(event);
    if kind != 1 && kind != 5 {
        return false;
    }
    let mut rules = RULES.lock().unwrap();
    if !load(&mut rules) {
        return false;
    }
    path_event_loaded(c, &mut rules, rider_address, event, kind, entering, active)
}

fn path_event_loaded(
    c: &mut Cpu,
    rules: &mut Rules,
    rider_address: u32,
    event: u32,
    kind: u32,
    entering: bool,
    active: bool,
) -> bool {
    let passes = rules.passes;
    let lap_volume = rules.volumes.iter().any(|volume| volume.kind == 15);
    let count = rules.volumes.len();
    let rider = rules.riders.entry(rider_address).or_insert_with(|| Rider {
        remaining: passes,
        lap_ready: true,
        last_cross: None,
        previous: None,
        stage: 0,
        contacts: (0..count).map(|_| Contact::default()).collect(),
        time_gates: HashSet::new(),
    });
    if rules.event == Some(crate::custom_tracks::EventKind::Freestyle) && kind == 5 {
        if entering && c.bus.read32(rider_address + 0x874) != 0 {
            let award = c.bus.read32(event + 4);
            let group = award & 0xffff;
            let seconds = award >> 16;
            if seconds != 0 && seconds <= 600 && rider.time_gates.insert(group) {
                if let Some((entity, _)) = event_entity(c) {
                    if c.bus.read32(entity + 0x88) != 0 {
                        let ticks = seconds.saturating_mul(crate::tick::rate(c));
                        let limit = c.bus.read32(entity + 0x78);
                        c.bus.write32(entity + 0x78, limit.saturating_add(ticks));
                        eprintln!("[tricky] SOP checkpoint {group}: +{seconds}s (rider {rider_address:#x})");
                    }
                }
            }
        }
        return true;
    }
    if passes <= 1 {
        return false;
    }
    if kind == 5 && entering && !lap_volume {
        count_pass(rider, rider_address, crate::tick::vsync_count(c), 180);
    } else if kind == 5 && !active && !lap_volume {
        rider.lap_ready = true;
    }
    kind == 1 && rider.remaining > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_tracks::EventKind;

    const RIDER: u32 = 0x0080_0000;
    const APPMAN: u32 = 0x0081_0000;
    const EVENT: u32 = 0x0082_0000;
    const ENTITY: u32 = 0x0083_0000;

    fn fixture(event: EventKind, passes: u32, circuit: bool) -> (Box<Cpu>, Rules) {
        let mut c = crate::difftest::fresh();
        c.bus.write32(crate::tick::APPMAN_PTR, APPMAN);
        c.bus.write32(APPMAN + crate::tick::APPMAN_RATE, 60);
        put(&mut c, RIDER + 0x470, -1.);
        c.bus.write32(RIDER + 0x874, 1); // Human rider.
        let mut data = Vec::new();
        data.extend_from_slice(b"TFX2");
        for word in [passes, if circuit { 2 } else { 0 }, 9000, 1, 0] {
            data.extend_from_slice(&word.to_le_bytes());
        }
        if circuit {
            for kind in [15u32, 18] {
                let mut f = [0f32; 32];
                f[0] = 60.;
                f[1] = 20.;
                f[3] = 2.;
                f[6] = 1.;
                f[8..11].copy_from_slice(&[-1., -1., if kind == 15 { -1. } else { 8. }]);
                f[12..15].copy_from_slice(&[1., 1., if kind == 15 { 1. } else { 10. }]);
                f[19] = 10.;
                for word in [0, kind, 1, 0] {
                    data.extend_from_slice(&word.to_le_bytes());
                }
                for value in f {
                    data.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        let mut rules = Rules {
            event: Some(event),
            ..Rules::default()
        };
        assert!(load_data(&mut rules, &data));
        (c, rules)
    }

    fn step(c: &mut Cpu, rules: &mut Rules, position: [f32; 3], tick: u32) {
        c.bus.write32(APPMAN + crate::tick::APPMAN_VSYNC, tick);
        for (axis, value) in position.into_iter().enumerate() {
            put(c, RIDER + 0x110 + axis as u32 * 4, value);
        }
        update_loaded(c, RIDER, rules);
    }

    #[test]
    fn tokyo_freestyle_finishes_after_initial_ascent_and_three_laps() {
        for mode in [EventKind::Freestyle, EventKind::Race] {
            let (mut c, mut rules) = fixture(mode, 4, true);
            // Initial tube ascent, then three complete circuits. The finish
            // remains suppressed until the fourth mouth entry; no final lift.
            for pass in 0..4 {
                let tick = 600 + pass * 600;
                put(&mut c, RIDER + 0x1e8, 0.);
                step(&mut c, &mut rules, [0.; 3], tick);
                assert_eq!(rules.riders[&RIDER].remaining, 3 - pass);
                assert_eq!(rules.riders[&RIDER].contacts[0].serviced, pass != 3);
                assert_eq!(float(&mut c, RIDER + 0x1e8) > 0., pass != 3);
                assert_eq!(
                    path_event_loaded(&mut c, &mut rules, RIDER, EVENT, 1, true, true),
                    pass != 3
                );
                // Staying in the mouth cannot consume another pass.
                step(&mut c, &mut rules, [0.; 3], tick + 200);
                assert_eq!(rules.riders[&RIDER].remaining, 3 - pass);
                if pass != 3 {
                    step(&mut c, &mut rules, [0., 0., 9.], tick + 201);
                    assert!(rules.riders[&RIDER].lap_ready);
                    // Leave both volumes, including their swept contacts.
                    step(&mut c, &mut rules, [20., 0., 20.], tick + 202);
                    step(&mut c, &mut rules, [20., 0., 20.], tick + 203);
                }
            }
        }
    }

    #[test]
    fn freestyle_time_awards_work_on_finite_circuits_and_linear_courses() {
        for (passes, circuit) in [(4, true), (1, false)] {
            let (mut c, mut rules) = fixture(EventKind::Freestyle, passes, circuit);
            let app = 0x0084_0000;
            let module = 0x0085_0000;
            let ai = 0x0086_0000;
            c.bus.write32(0x004a_28a8, app);
            c.bus.write32(app + 0x84, module);
            c.bus.write32(app + 0xc0, ENTITY);
            c.bus.write32(module + 0xc, ai);
            c.bus.write32(ENTITY + 0x88, 1);
            c.bus.write32(ENTITY + 0x78, 5400);
            c.bus.write32(EVENT + 4, (15 << 16) | 7);
            for _ in 0..2 {
                assert!(path_event_loaded(
                    &mut c, &mut rules, RIDER, EVENT, 5, true, true
                ));
                assert_eq!(c.bus.read32(ENTITY + 0x78), 6300);
                assert_eq!(rules.riders[&RIDER].remaining, passes);
            }
            if circuit {
                step(&mut c, &mut rules, [0.; 3], 600);
                step(&mut c, &mut rules, [0., 0., 9.], 601);
                assert!(path_event_loaded(
                    &mut c, &mut rules, RIDER, EVENT, 5, true, true
                ));
                assert_eq!(c.bus.read32(ENTITY + 0x78), 7200);
            } else {
                assert!(!path_event_loaded(
                    &mut c, &mut rules, RIDER, EVENT, 1, true, true
                ));
            }
        }
    }
}
