//! Tricky CrowdBox17: original images, sixteen independent spectators.
//!
//! Native Object owns focus, culling, transforms, lighting and drawing. Its
//! ordinary render call receives a private native-format material table in
//! the imported instance's reserved tail. No eight-slot TexFlip is involved.

use ssx3_ee_rt::cpu::reg::{A1, A2, GP};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

const MAGIC: u32 = 0x3144_5243;
const BYTES: u32 = 420;
const TABLE: u32 = 32;
const RECORDS: u32 = 100;
// Source145e08 selects through the four sequences at31d378.
const FRAMES: [[u8; 16]; 4] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 1, 2, 3, 2, 1, 0, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1],
];
const IDLE: [u32; 7] = [256, 0, 50, 100, 200, 100, 50];

/// Source2401e0's six-word carry generator, kept private to these cosmetic
/// actors. The source global RNG also serves unrelated systems; its exact
/// global seed/call order is not shared by the native SSX3 simulation.
struct Random([u32; 6]);

impl Random {
    fn new(mut seed: u32) -> Self {
        Self(std::array::from_fn(|_| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed
        }))
    }

    fn next(&mut self) -> u32 {
        let words = &mut self.0;
        let mut value = words[5].wrapping_add(words[4]);
        let mut carry = u32::from(value < words[5] || value < words[4]);
        words[4] = value;
        for index in (0..4).rev() {
            let previous = words[index];
            value = value.wrapping_add(previous).wrapping_add(carry);
            carry = u32::from(value < previous);
            words[index] = value;
        }
        words[5] = words[5].wrapping_add(1);
        if words[5] == 0 {
            for index in (0..5).rev() {
                words[index] = words[index].wrapping_add(1);
                if words[index] != 0 { break }
            }
        }
        words[0]
    }
}

struct Crowd {
    owner: u32,
    instance: u32,
    storage: u32,
    phase: [i32; 16],
    animation: [usize; 16],
    random: Random,
}

#[derive(Default)]
struct State {
    track: Option<&'static str>,
    remainder: f64,
    crowds: HashMap<u32, Crowd>,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if state.track != track { *state = State { track, ..State::default() }; }
    track.is_some()
}

pub fn restart() { *STATE.lock().unwrap() = State::default(); }

fn valid(address: u32, size: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000 && physical.checked_add(size).is_some_and(|end| end <= 0x0200_0000)
}

fn installed(c: &mut Cpu, node: u32, crowd: &Crowd) -> bool {
    valid(node, 0x20) && c.bus.read16(node + 0x10) == 17
        && valid(crowd.instance, 0xa0) && c.bus.read32(node + 0x18) == crowd.instance
        && c.bus.read32(crowd.instance + 12) == node
        && c.bus.read32(crowd.instance + 0x78) == crowd.owner
        && c.bus.read32(crowd.storage) == MAGIC
        && c.bus.read32(crowd.storage + 4) == 1
}

fn write_frames(c: &mut Cpu, crowd: &Crowd) {
    let base = c.bus.read32(crowd.storage + 8);
    for material in 0..16 {
        let slot = c.bus.read8(crowd.storage + 16 + material) as usize;
        let phase = if crowd.phase[slot] < 0 { 0 }
            else { (crowd.phase[slot] as usize + slot) & 15 };
        let frame = FRAMES[crowd.animation[slot]][phase] as u32;
        c.bus.write16(crowd.storage + RECORDS + 20 * material, (base + frame) as u16);
    }
}

/// Builtin134 follows native Object creation: RID, reserved storage offset,
/// authored rows/columns. Storage is part of the ordinary streamed resource;
/// no guest allocation, fixed-size modifier overflow or extra actor is needed.
pub fn register(c: &mut Cpu) {
    let args = c.g[A2][0] as u32;
    if c.g[A1][0] != 4 || !valid(args, 64) { return }
    let mut values = [0u32; 4];
    let mut seen = 0;
    for i in 0..4 {
        let at = args + i * 16;
        let key = c.bus.read32(at);
        if key >= 4 || seen & (1 << key) != 0 || c.bus.read32(at + 12) != 1 { return }
        seen |= 1 << key;
        values[key as usize] = c.bus.read32(at + 4);
    }
    let [owner, offset, rows, columns] = values;
    if rows == 0 || rows.checked_mul(columns) != Some(16)
        || offset < 160 || offset > 0x10_0000 || offset & 15 != 0 { return }
    let Some(instance) = crate::custom_script_effects::resident_instance(c, owner) else { return };
    let node = c.bus.read32(instance + 12);
    let Some(storage) = instance.checked_add(offset) else { return };
    if !valid(storage, BYTES) || !valid(node, 0x20)
        || c.bus.read16(node + 0x10) != 17 || c.bus.read32(node + 0x18) != instance
        || c.bus.read32(storage) != MAGIC || c.bus.read32(storage + 4) != 1
        || c.bus.read32(storage + 12) != 16 || c.bus.read32(storage + 8) > 32752 { return }
    let source = c.bus.read32(instance + 0x94);
    if !valid(source, 68) || c.bus.read32(source) != 16 { return }
    // Validate the whole source/mapping before writing any private record.
    let mut mask = 0;
    let mut materials = [0u32; 16];
    for (i, material) in materials.iter_mut().enumerate() {
        let slot = c.bus.read8(storage + 16 + i as u32);
        if slot >= 16 || mask & (1 << slot) != 0 { return }
        mask |= 1 << slot;
        *material = c.bus.read32(source + 4 + i as u32 * 4);
        if !valid(*material, 20) { return }
    }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    if state.crowds.get(&node).is_some_and(|crowd|
        crowd.owner == owner && crowd.storage == storage && installed(c, node, crowd)) { return }
    let mut crowd = Crowd { owner, instance, storage, phase: [0; 16],
        animation: [0; 16], random: Random::new(owner ^ c.bus.read32(storage + 8)) };
    for i in 0..16 {
        crowd.phase[i] = (crowd.random.next() & 15) as i32;
        crowd.animation[i] = (crowd.random.next() & 3) as usize;
    }
    c.bus.write32(storage + TABLE, 16);
    for (i, material) in materials.into_iter().enumerate() {
        let record = storage + RECORDS + i as u32 * 20;
        c.bus.write32(storage + TABLE + 4 + i as u32 * 4, record);
        for offset in (0..20).step_by(4) {
            let value = c.bus.read32(material + offset);
            c.bus.write32(record + offset, value);
        }
    }
    write_frames(c, &crowd);
    state.crowds.insert(node, crowd);
}

/// Native reaction lookup has the same source per-rider0x190 layout and
/// signed +18c field (source2213f8, native2a77c8). Read it without a guest
/// call or changing audio ownership. Only human riders excite nearby crowds.
fn reactions(c: &mut Cpu) -> (bool, Vec<([f32; 3], u32)>) {
    let app = c.bus.read32(0x004a_28a8);
    if !valid(app, 0x88) { return (false, Vec::new()) }
    let module = c.bus.read32(app + 0x84);
    if !valid(module, 16) { return (false, Vec::new()) }
    let ai = c.bus.read32(module + 12);
    if !valid(ai, 0x80) || c.bus.read32(ai) < 4 { return (false, Vec::new()) }
    let audio = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0x410));
    let (count, voices) = if valid(audio, 0x1c) {
        (c.bus.read32(audio + 0x14), c.bus.read32(audio + 0x18))
    } else { (0, 0) };
    let mut riders = Vec::new();
    for index in 0..c.bus.read32(ai + 0x78).min(6) {
        let rider = c.bus.read32(ai + 0x28 + index * 4);
        if !valid(rider, 0x878) || c.bus.read32(rider + 0x874) == 0 { continue }
        let position = std::array::from_fn(|axis|
            f32::from_bits(c.bus.read32(rider + 0x110 + axis as u32 * 4)));
        let voice_index = c.bus.read32(rider + 0x86c); // native Rider::AI_INDEX
        let voice = voices.wrapping_add(voice_index.wrapping_mul(0x190));
        let reaction = if voice_index < count && valid(voice, 0x190) {
            c.bus.read32(voice + 0x18c)
        } else { 0 };
        riders.push((position, reaction.wrapping_add(4) & 7));
    }
    (true, riders)
}

pub fn begin_update(c: &mut Cpu) {
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    // Source updates once every5 ticks at60Hz, independently of frame rate.
    state.remainder += 12. / crate::tick::rate(c).max(1) as f64;
    let steps = (state.remainder + 1e-9).floor() as u32;
    state.remainder -= steps as f64;
    if steps == 0 { return }
    let (racing, riders) = reactions(c);
    state.crowds.retain(|&node, crowd| {
        if !installed(c, node, crowd) { return false }
        let position: [f32; 3] = std::array::from_fn(|axis|
            f32::from_bits(c.bus.read32(crowd.instance + 0x40 + axis as u32 * 4)));
        let mut mood = if racing { 0 } else { 7 };
        if racing {
            for (rider, reaction) in &riders {
                let distance: f32 = (0..3).map(|axis| (position[axis] - rider[axis]).powi(2)).sum();
                if distance <= 20000. * 20000. { mood = *reaction }
            }
        }
        for _ in 0..steps {
            if mood != 0 {
                for i in 0..16 {
                    crowd.phase[i] = crowd.phase[i].wrapping_add(1);
                    if crowd.phase[i] >= 16 && crowd.phase[i] & 15 == 0 {
                        let value = crowd.random.next();
                        crowd.animation[i] = (value & 3) as usize;
                        if mood < 7 && value & 255 < IDLE[mood as usize] {
                            crowd.phase[i] = -((value & 63) as i32);
                        }
                    }
                }
            }
        }
        write_frames(c, crowd);
        true
    });
}

/// Called only from the existing native ModObj draw path, after culling.
pub fn materials(c: &mut Cpu, node: u32) -> Option<u32> {
    if crate::custom_tracks::active().is_none() { return None }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return None }
    let crowd = state.crowds.get(&node)?;
    installed(c, node, crowd).then_some(crowd.storage + TABLE)
}
