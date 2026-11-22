//! Source Movie23 pictures in one shared native sixteen-bit world texture.
//!
//! Prepared TVM1 contains the source decoder's RGB5551 pictures. Native
//! Object/material rendering and TexMan eviction/upload own their display.

use ssx3_ee_rt::cpu::reg::{A1, A2};
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

const MOVIE_MAGIC: u32 = 0x314d_5654;
const SCREEN_MAGIC: u32 = 0x3153_5654;
const PICTURE_BYTES: usize = 128 * 64 * 2;
const PERIOD: f64 = 1. / 15.;

#[derive(Clone, Copy)]
struct Texture {
    manager: u32,
    index: u32,
    info: u32,
    pixels: u32,
    frames: usize,
}

struct Movie {
    data: Vec<u8>,
    frames: usize,
}

impl Movie {
    fn open(root: &std::path::Path, code: &str) -> Option<Self> {
        let path = root.join(format!("DATA/WORLDS/TRICKY/{code}.TVM"));
        let length = std::fs::metadata(&path).ok()?.len();
        if !(32..=64 * 1024 * 1024).contains(&length) { return None }
        let data = std::fs::read(path).ok()?;
        // Explicit fields below keep malformed/older overlays on their static
        // screen rather than reading outside the prepared picture sequence.
        let word = |at: usize| -> Option<u32> {
            Some(u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?))
        };
        let frames = word(16)? as usize;
        if word(0)? != MOVIE_MAGIC || word(4)? != 1 || word(8)? != 128
            || word(12)? != 64 || !(1..=3600).contains(&frames) || word(20)? != 15
            || word(24)? != PICTURE_BYTES as u32 || word(28)? != 0
            || data.len() != 32 + frames * PICTURE_BYTES { return None }
        Some(Self { data, frames })
    }
}

#[derive(Clone, Copy)]
struct Screen {
    owner: u32,
    instance: u32,
    storage: u32,
}

#[derive(Default)]
struct State {
    root: Option<PathBuf>,
    track: Option<&'static str>,
    texture: Option<Texture>,
    movie: Option<Movie>,
    attempted: bool,
    screens: HashMap<u32, Screen>,
    time: f64,
    next_due: f64,
    cursor: usize,
    playing: bool,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::default()));

fn sync(state: &mut State) -> bool {
    let track = crate::custom_tracks::active().map(|track| track.id);
    if state.track != track {
        *state = State { root: state.root.take(), track, ..State::default() };
    }
    track.is_some()
}

pub fn set_root(root: Option<PathBuf>) {
    let mut state = STATE.lock().unwrap();
    if state.root != root { *state = State { root, ..State::default() }; }
}

/// Native world textures can survive a same-course restart. Keep their
/// published ownership and cached pictures, but restart source playback and
/// require new live Object registrations. A course change clears everything.
pub fn restart() {
    let mut state = STATE.lock().unwrap();
    state.screens.clear();
    state.time = 0.;
    state.next_due = 0.;
    state.cursor = 0;
    state.playing = false;
}

fn valid(address: u32, size: u32) -> bool {
    let physical = address & 0x1fff_ffff;
    physical >= 0x10000 && physical.checked_add(size).is_some_and(|end| end <= 0x0200_0000)
}

fn installed(c: &mut Cpu, node: u32, screen: &Screen) -> bool {
    valid(node, 0x20) && c.bus.read16(node + 0x10) == 17
        && valid(screen.instance, 0xa0) && c.bus.read32(node + 0x18) == screen.instance
        && c.bus.read32(screen.instance + 12) == node
        && c.bus.read32(screen.instance + 0x78) == screen.owner
        && c.bus.read32(screen.storage) == SCREEN_MAGIC
        && c.bus.read32(screen.storage + 4) == 1
}

fn live_texture(c: &mut Cpu, texture: Texture) -> bool {
    valid(texture.manager + 8 + texture.index * 4, 4)
        && c.bus.read32(texture.manager + 8 + texture.index * 4) == texture.info
        && valid(texture.info, 0x38) && c.bus.read32(texture.info + 12) == 2
        && c.bus.read32(texture.info + 0x10) == PICTURE_BYTES as u32
        && c.bus.read32(texture.info + 0x1c) == texture.pixels
        && valid(texture.pixels, PICTURE_BYTES as u32)
}

/// Capture an explicitly tagged image after the ordinary native shape load.
/// The TexMan `this` adjustment is the same vtable178 entry used by that load;
/// no guessed PS2GraphicsMan field offset or substitute texture is used.
pub fn published(c: &mut Cpu, shape: u32, graphics: u32, index: u32) {
    if crate::custom_tracks::active().is_none() || !valid(shape, 128)
        || c.bus.read32(shape + 32) != MOVIE_MAGIC || c.bus.read32(shape + 36) != 1
        || index >= 2000 || c.bus.read32(shape + 40) != index
        || c.bus.read8(shape) != 3 || c.bus.read16(shape + 4) != 128
        || c.bus.read16(shape + 6) != 64 || !valid(graphics, 0x10dc) { return }
    let frames = c.bus.read32(shape + 44) as usize;
    if !(1..=3600).contains(&frames) { return }
    let vtable = c.bus.read32(graphics + 0x10d8);
    if !valid(vtable, 0x180) { return }
    let adjustment = c.bus.read16(vtable + 0x178) as i16 as i32 as u32;
    let manager = graphics.wrapping_add(adjustment);
    let entry = manager.wrapping_add(8 + index * 4);
    if !valid(entry, 4) { return }
    let info = c.bus.read32(entry);
    if !valid(info, 0x38) { return }
    let texture = Texture { manager, index, info, pixels: c.bus.read32(info + 0x1c), frames };
    if !live_texture(c, texture) { return }
    let mut state = STATE.lock().unwrap();
    if sync(&mut state) { state.texture = Some(texture); }
}

/// Builtin135 follows native Object creation: owner RID and private storage.
/// Source1ff6f0 exposes four pointers to the same material (flags1). These
/// supplied screen models each use one material; retain one private pointer.
pub fn register(c: &mut Cpu) {
    let args = c.g[A2][0] as u32;
    if c.g[A1][0] != 2 || !valid(args, 32) { return }
    let mut values = [0u32; 2];
    let mut seen = 0;
    for i in 0..2 {
        let at = args + i * 16;
        let key = c.bus.read32(at);
        if key >= 2 || seen & (1 << key) != 0 || c.bus.read32(at + 12) != 1 { return }
        seen |= 1 << key;
        values[key as usize] = c.bus.read32(at + 4);
    }
    let [owner, offset] = values;
    if offset < 160 || offset > 0x10_0000 || offset & 15 != 0 { return }
    let Some(instance) = crate::custom_script_effects::resident_instance(c, owner) else { return };
    let node = c.bus.read32(instance + 12);
    let Some(storage) = instance.checked_add(offset) else { return };
    if !valid(storage, 48) || !valid(node, 0x20) || c.bus.read16(node + 0x10) != 17
        || c.bus.read32(node + 0x18) != instance || c.bus.read32(storage) != SCREEN_MAGIC
        || c.bus.read32(storage + 4) != 1 || c.bus.read32(storage + 12) != 1 { return }
    let texture = c.bus.read32(storage + 8);
    if texture > 32767 { return }
    let original = c.bus.read32(instance + 0x94);
    if !valid(original, 8) || c.bus.read32(original) != 1 { return }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    state.screens.retain(|&node, screen| installed(c, node, screen));
    if state.screens.get(&node).is_some_and(|screen|
        screen.owner == owner && screen.storage == storage) { return }
    if state.screens.is_empty() {
        state.playing = true;
        state.cursor = 0;
        state.next_due = state.time + PERIOD;
    }
    c.bus.write32(storage + 16, 1);
    c.bus.write32(storage + 20, storage + 24);
    // Native WorldMaterial {texture,-1,-1,0,0,0,1,flags1,no flipbook}.
    for (i, word) in [texture | 0xffff_0000, 0xffff, 0, 0x0001_0001, 0].into_iter().enumerate() {
        c.bus.write32(storage + 24 + i as u32 * 4, word);
    }
    state.screens.insert(node, Screen { owner, instance, storage });
}

pub fn begin_update(c: &mut Cpu) {
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return }
    state.time += 1. / crate::tick::rate(c).max(1) as f64;
    state.screens.retain(|&node, screen| installed(c, node, screen));
    if state.screens.is_empty() { state.playing = false; }
}

/// Write at most one picture for all screens, then let the script manager
/// call native TexMan_EvictTexture. The ordinary next draw uploads it. Cursor
/// ownership advances before that call, so an unwind cannot replay the write.
pub fn next_frame(c: &mut Cpu) -> Option<(u32, u32)> {
    let track = crate::custom_tracks::active()?;
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return None }
    state.screens.retain(|&node, screen| installed(c, node, screen));
    if state.screens.is_empty() { state.playing = false; }
    if !state.playing || state.time + 1e-9 < state.next_due { return None }
    let texture = state.texture?;
    if !live_texture(c, texture) { state.texture = None; return None }
    if !state.attempted {
        state.attempted = true;
        state.movie = state.root.as_ref().and_then(|root| Movie::open(root, track.world_location));
        if state.movie.is_none() {
            eprintln!("[tricky movie] missing/incomplete {}.TVM", track.world_location);
        }
    }
    let movie = state.movie.as_ref()?;
    if movie.frames != texture.frames { return None }
    let steps = (((state.time - state.next_due).max(0.) / PERIOD + 1e-9).floor() as usize) + 1;
    let frame = (state.cursor + steps - 1) % movie.frames;
    let begin = 32 + frame * PICTURE_BYTES;
    let address = (texture.pixels & 0x1fff_ffff) as usize;
    c.bus.ram.get_mut(address..address + PICTURE_BYTES)?
        .copy_from_slice(&movie.data[begin..begin + PICTURE_BYTES]);
    state.cursor = (frame + 1) % movie.frames;
    state.next_due += steps as f64 * PERIOD;
    Some((texture.manager, texture.index))
}

pub fn materials(c: &mut Cpu, node: u32) -> Option<u32> {
    if crate::custom_tracks::active().is_none() { return None }
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return None }
    let screen = state.screens.get(&node)?;
    installed(c, node, screen).then_some(screen.storage + 16)
}

/// Source Movie_Render148d28 selects alpha mode3 for both contexts.
/// State bytes15/17 reach GsPkt_AddAlpha1c03e8, whose mode3 emits ALPHA44:
/// (Cs-Cd)*As+Cd. Native preset5 has that same equation. DrawModel normally
/// replaces an opaque material's blend with preset1; the registered movie
/// table must instead keep the source blend, without changing sorting/Z/test.
pub fn is_screen(c: &mut Cpu, instance: u32, table: u32) -> bool {
    if crate::custom_tracks::active().is_none() || !valid(instance, 0xa0) { return false }
    let node = c.bus.read32(instance + 12);
    let mut state = STATE.lock().unwrap();
    if !sync(&mut state) { return false }
    state.screens.get(&node).is_some_and(|screen|
        screen.instance == instance && screen.storage + 16 == table && installed(c, node, screen))
}
