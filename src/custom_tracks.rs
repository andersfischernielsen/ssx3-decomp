//! Host-owned catalog for additional Single Event courses.
//!
//! IDs here never index the guest's 17-entry TrackInfo table or its career
//! save records. Menu and race integration can look up an entry by its stable
//! string ID, then resolve its own world and race data.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

use ssx3_ee_rt::Cpu;

/// A selectable group of additional Single Event courses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CustomPeak {
    pub id: &'static str,
    pub name: &'static str,
}

/// The original SSX 3 race setup remains the authority for game mechanics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventKind {
    Race,
    Freestyle,
}

/// Source disc and archive needed for a course's world assets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrackSource {
    TrickyPs2 { archive: &'static str },
}

/// A host course, deliberately separate from original numeric course IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CustomTrack {
    pub id: &'static str,
    pub name: &'static str,
    pub peak_id: &'static str,
    pub locale_key: &'static str,
    pub event: EventKind,
    pub source: TrackSource,
    /// Name of the appended location in the derived BAM archive.
    pub world_location: &'static str,
    /// Native SSX 3 event whose race state supplies gameplay and UI flow.
    pub template_event: &'static str,
    /// World location associated with that native event.
    pub template_location: &'static str,
}

pub const PEAKS: &[CustomPeak] = &[CustomPeak {
    id: "tricky",
    name: "Peak Tricky",
}];

include!("tricky_catalog.rs");

pub fn menu_tracks(tab: u32) -> impl Iterator<Item = &'static CustomTrack> {
    let kind = if tab == 1 { EventKind::Freestyle } else { EventKind::Race };
    TRACKS.iter().filter(move |track| track.event == kind)
}

/// UIMenu initialization can assign its own child IDs after widget creation.
/// Restore the bounded retail IDs and the host catalog's row IDs at dispatch.
pub fn widget_row_id(name: u32) -> Option<u32> {
    let hash = crate::game::hashvalue::str_hash;
    for (label, id) in [(b"Peak4".as_slice(), 3),
                       (b"BonusRace", 0), (b"BonusFreestyle", 1)] {
        if name == hash(label) { return Some(id); }
    }
    for tab in 0..2 {
        let kind = if tab == 1 { "Freestyle" } else { "Race" };
        for (index, _) in menu_tracks(tab).enumerate() {
            if name == hash(format!("Tricky{kind}{index}").as_bytes()) { return Some(index as u32); }
        }
    }
    None
}

// Zero means that the game follows only the original event/section tables.
// The stored index is one-based so later custom courses need no new flag.
static ACTIVE: AtomicUsize = AtomicUsize::new(0);
static ACTIVE_WORLD_SECTION: AtomicU32 = AtomicU32::new(u32::MAX);
static TEMPLATE_WORLD_SECTION: AtomicU32 = AtomicU32::new(u32::MAX);
static LEASED_MAP_SLOT: AtomicU32 = AtomicU32::new(u32::MAX);
static BONUS_SELECTED: AtomicBool = AtomicBool::new(false);
static ASSETS_AVAILABLE: AtomicBool = AtomicBool::new(false);
// Retail reserves slots 0..1500 for streamed WorldSSH shapes/lightmaps.
// An appended world can extend that range; global FX must start after it.
static WORLD_TEXTURE_SLOTS: AtomicU32 = AtomicU32::new(1500);
static TRACK_INFO: AtomicU32 = AtomicU32::new(0);
static MENU_ACTION_FRAME: AtomicU32 = AtomicU32::new(u32::MAX);
static MENU_PREVIEW: AtomicUsize = AtomicUsize::new(0);

pub fn preview_logo() { MENU_PREVIEW.store(1, Ordering::Release); }
pub fn clear_preview() { MENU_PREVIEW.store(0, Ordering::Release); }
pub fn preview_track(track: &'static CustomTrack) {
    let index = TRACKS.iter().position(|item| item.id == track.id).unwrap();
    MENU_PREVIEW.store(index + 2, Ordering::Release);
}
pub fn menu_preview() -> Option<(u32, String)> {
    let index = MENU_PREVIEW.load(Ordering::Acquire);
    if index == 0 { return None; }
    let image = if index == 1 { "logo" } else { TRACKS[index - 2].world_location };
    Some((0x100 + index as u32, format!("|tricky_{}.ssh", image.to_ascii_lowercase())))
}

pub fn preview_course() -> Option<&'static CustomTrack> {
    MENU_PREVIEW.load(Ordering::Acquire).checked_sub(2).and_then(|index| TRACKS.get(index))
}

pub fn menu_text_id(original: u32) -> u32 {
    // Retail help descriptions belong to the template event, not the
    // imported course. Remap only those two locale IDs while browsing it.
    if !bonus_peak_selected() || !matches!(original, 0x0af5_21c1 | 0x0af5_2fe1) {
        return original;
    }
    let Some(track) = preview_course() else { return original; };
    let mode = if track.event == EventKind::Race { "Race" } else { "Freestyle" };
    crate::game::hashvalue::str_hash(format!("kT_TrickyHelp{mode}_{}", track.world_location).as_bytes())
}
// The template's native loading code consumes one fixed 40-byte residency
// row. Lease it while the custom event is active and restore it on return.
// Its sky and shared transport assets remain native; its terrain connectors
// are excluded so they cannot overlap Garibaldi or run Snow Jam scripts.
static RESIDENCY_ROW: Mutex<Option<(u32, [u8; 40])>> = Mutex::new(None);
const TEMPLATE_RESIDENCY: u32 = 0x0044_2488;

/// Build a separate metadata record so loading screens and results can use
/// the custom name without changing the retail TrackInfo array.
pub fn prepare_track_info(c: &mut Cpu, back: u32) -> crate::call::Flow {
    let Some(track) = active() else { return Ok(()); };
    use ssx3_ee_rt::cpu::{reg::*, sx};
    let template = 0x0043_d950 + template_course_index(c, track) * 100;
    let mut pointer = TRACK_INFO.load(Ordering::Acquire);
    if pointer == 0 {
        c.g[A0][0] = 100;
        c.g[A1][0] = sx(template + 4);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        crate::call::call_ptr(c, 0x0031_7d70, back)?;
        pointer = c.g[V0][0] as u32;
    }
    if pointer != 0 {
        for offset in 0..100 {
            let value = c.bus.read8(template + offset);
            c.bus.write8(pointer + offset, value);
        }
        // TrackInfo contains a display name, whereas UIText uses locale IDs.
        let name = track.name.as_bytes();
        for offset in 0..48 {
            c.bus.write8(
                pointer + 4 + offset,
                name.get(offset as usize).copied().unwrap_or(0),
            );
        }
        TRACK_INFO.store(pointer, Ordering::Release);
    }
    Ok(())
}

fn template_course_index(c: &mut Cpu, track: &CustomTrack) -> u32 {
    (0..17).find(|index| {
        let address = 0x0043_d950 + index * 100 + 0x34;
        (0..4).all(|i| c.bus.read8(address + i) == track.template_event.as_bytes()[i as usize])
    }).expect("native custom-event template is absent")
}

pub fn active_world_archive() -> Option<String> {
    let track = active()?;
    let mode = if track.event == EventKind::Race { "race" } else { "freestyle" };
    Some(format!("DATA/WORLDS/TRICKY/{}-{mode}.BIG", track.world_location))
}

pub fn overlay_archive(name: &str) -> Option<String> {
    if name.eq_ignore_ascii_case("DATA/WORLDS/BAM.BIG") {
        active_world_archive()
    } else if name.eq_ignore_ascii_case("DATA/UI/COURSPIC.BIG") {
        Some(format!("DATA/UI/TRICKY/{}.BIG", active()?.world_location))
    } else { None }
}

pub fn activate_code(value: &str) -> Option<&'static CustomTrack> {
    let (code, mode) = value.split_once(':').unwrap_or((value, ""));
    let track = TRACKS.iter().find(|track| {
        (track.id.eq_ignore_ascii_case(code) || track.world_location.eq_ignore_ascii_case(code))
            && (mode.is_empty() || (mode.eq_ignore_ascii_case("freestyle") && track.event == EventKind::Freestyle)
                || (mode.eq_ignore_ascii_case("race") && track.event == EventKind::Race))
    })?;
    activate(track.id)
}

pub fn track_info() -> Option<u32> {
    if active().is_none() {
        return None;
    }
    let pointer = TRACK_INFO.load(Ordering::Acquire);
    (pointer != 0).then_some(pointer)
}

/// Whether a compact world-resource ID belongs to the active imported section.
pub(crate) fn owns_world_resource(rid: u32) -> bool {
    let section = ACTIVE_WORLD_SECTION.load(Ordering::Acquire);
    active().is_some() && section != u32::MAX && rid & 255 == section
}

pub fn set_assets_available(available: bool) {
    ASSETS_AVAILABLE.store(available, Ordering::Release);
}

pub fn assets_available() -> bool {
    ASSETS_AVAILABLE.load(Ordering::Acquire)
}

/// Finalize the working native race settings before BEInterface commits them.
/// Tricky Showoff uses Slopestyle scoring with one human and no rival/extra
/// riders. Apply this at the commit boundary after all native setup setters.
pub fn configure_native_run(c: &mut Cpu) {
    if !active().is_some_and(|track| track.event == EventKind::Freestyle) {
        return;
    }
    const SETTINGS: u32 = 0x0053_5bc8;
    if c.bus.read8(SETTINGS + 0x48) != 1 { return; }
    c.bus.write32(SETTINGS + 0x30, 1); // total
    c.bus.write32(SETTINGS + 0x34, 1); // humans
    c.bus.write32(SETTINGS + 0x38, 0); // extra mission riders
    c.bus.write32(SETTINGS + 0x3c, 0); // AI riders
    eprintln!("[tricky] commit Showoff: one human, zero AI/extra riders");
}

pub fn set_world_texture_slots(slots: u32) {
    WORLD_TEXTURE_SLOTS.store(slots.max(1500), Ordering::Release);
}

pub fn world_texture_slots() -> u32 {
    WORLD_TEXTURE_SLOTS.load(Ordering::Acquire)
}

/// The custom menu category is separate from the guest's three-peak index.
pub fn select_bonus_peak() {
    BONUS_SELECTED.store(true, Ordering::Release);
}

pub fn bonus_peak_selected() -> bool {
    BONUS_SELECTED.load(Ordering::Acquire)
}

/// Newly enabled menus occur later in the screen's input traversal. Consume
/// the current press once so it cannot also accept Race and Garibaldi.
pub fn accept_menu_step(c: &mut Cpu) -> bool {
    let frame = crate::tick::vsync_count(c);
    MENU_ACTION_FRAME.swap(frame, Ordering::AcqRel) != frame
}

pub fn peak(id: &str) -> Option<&'static CustomPeak> {
    PEAKS.iter().find(|peak| peak.id == id)
}

pub fn track(id: &str) -> Option<&'static CustomTrack> {
    TRACKS.iter().find(|track| track.id == id)
}

pub fn tracks_on_peak(id: &str) -> impl Iterator<Item = &'static CustomTrack> + '_ {
    TRACKS.iter().filter(move |track| track.peak_id == id)
}

/// Select a host course before entering the native SSX 3 race flow.
pub fn activate(id: &str) -> Option<&'static CustomTrack> {
    let index = TRACKS.iter().position(|track| track.id == id)?;
    if ACTIVE.load(Ordering::Acquire) == index + 1 {
        return TRACKS.get(index);
    }
    ACTIVE_WORLD_SECTION.store(u32::MAX, Ordering::Release);
    TEMPLATE_WORLD_SECTION.store(u32::MAX, Ordering::Release);
    ACTIVE.store(index + 1, Ordering::Release);
    ssx3_render::diagnostics::set_course(Some(TRACKS[index].id));
    TRACKS.get(index)
}

pub fn activate_in_game(c: &mut Cpu, id: &str) -> Option<&'static CustomTrack> {
    track(id)?;
    if active().is_some_and(|old| old.id != id) {
        // Returning between courses may change both the native template
        // and its residency row. Release the previous lease first.
        deactivate(c);
        select_bonus_peak();
    }
    activate(id)
}

/// Restore the original event-to-world mapping on return to Single Event.
pub fn deactivate(c: &mut Cpu) {
    let slot = LEASED_MAP_SLOT.swap(u32::MAX, Ordering::AcqRel);
    let template = TEMPLATE_WORLD_SECTION.load(Ordering::Acquire);
    let custom = ACTIVE_WORLD_SECTION.load(Ordering::Acquire);
    if slot < 49 && template != u32::MAX && custom != u32::MAX {
        let address = 0x0044_216c + slot * 16;
        if c.bus.read32(address) == custom {
            c.bus.write32(address, template);
        }
    }
    if let Some((address, original)) = RESIDENCY_ROW.lock().unwrap().take() {
        for (offset, value) in original.into_iter().enumerate() {
            c.bus.write8(address + offset as u32, value);
        }
    }
    BONUS_SELECTED.store(false, Ordering::Release);
    ACTIVE.store(0, Ordering::Release);
    ssx3_render::diagnostics::set_course(None);
    ACTIVE_WORLD_SECTION.store(u32::MAX, Ordering::Release);
    TEMPLATE_WORLD_SECTION.store(u32::MAX, Ordering::Release);
}

pub fn active() -> Option<&'static CustomTrack> {
    let one_based = ACTIVE.load(Ordering::Acquire);
    one_based.checked_sub(1).and_then(|index| TRACKS.get(index))
}

/// Resolve the appended BAM location and lease the native template's logical
/// section slot to it for the active custom race. The original slot value is
/// recreated by SectionMan_Init whenever the normal event screen is entered.
/// No extra entry is written to the game's fixed 50-slot logical table.
pub fn remap_section(c: &mut Cpu, section_man: u32) {
    let Some(track) = active() else { return };
    let game = c.bus.read32(section_man.wrapping_add(0x1b0));
    if game == 0 {
        return;
    }
    let world = c.bus.read32(game.wrapping_add(0x10));
    if world == 0 {
        return;
    }
    let cache = c.bus.read32(world);
    if cache == 0 {
        return;
    }
    let sdb = c.bus.read32(cache.wrapping_add(0x14));
    if sdb == 0 {
        eprintln!("custom track {}: world index has not loaded", track.id);
        return;
    }
    let count = c.bus.read32(sdb.wrapping_add(8));
    if count > 255 {
        eprintln!(
            "custom track {}: unsupported world section count {count}",
            track.id
        );
        return;
    }
    let find = |name: &str, c: &Cpu| -> Option<u32> {
        (0..count).find(|&index| {
            let name_at = sdb.wrapping_add(80 + index * 88);
            c.bus.read_cstr(name_at).eq_ignore_ascii_case(name)
        })
    };
    let (Some(template), Some(custom)) = (
        find(track.template_location, c),
        find(track.world_location, c),
    ) else {
        eprintln!(
            "custom track {}: BAM lacks {} or {}",
            track.id, track.template_location, track.world_location
        );
        return;
    };
    let map = 0x0044_216c_u32;
    let slots: Vec<_> = (0..49)
        .filter(|&slot| c.bus.read32(map + slot * 16) == template)
        .collect();
    if slots.len() != 1 {
        eprintln!(
            "custom track {}: expected one logical section for {}, found {}",
            track.id,
            track.template_location,
            slots.len()
        );
        return;
    }
    c.bus.write32(map + slots[0] * 16, custom);
    let mut residency = RESIDENCY_ROW.lock().unwrap();
    let residency_address = TEMPLATE_RESIDENCY + template_course_index(c, track) * 40;
    if residency.is_none() {
        let mut original = [0; 40];
        for (offset, value) in original.iter_mut().enumerate() {
            *value = c.bus.read8(residency_address + offset as u32);
        }
        *residency = Some((residency_address, original));
    }
    c.bus.write32(residency_address + 4, 1);
    c.bus.write32(residency_address + 0x10, slots[0]);
    LEASED_MAP_SLOT.store(slots[0], Ordering::Release);
    TEMPLATE_WORLD_SECTION.store(template, Ordering::Release);
    ACTIVE_WORLD_SECTION.store(custom, Ordering::Release);
}

/// Redirect the template event's world-section lookup only while its custom
/// section is active. Other events and all unmodified sessions use retail IDs.
pub fn event_world_section(original: u32) -> u32 {
    if active().is_none() {
        return original;
    }
    let custom = ACTIVE_WORLD_SECTION.load(Ordering::Acquire);
    if custom == u32::MAX {
        return original;
    }
    // The template's current world index is looked up from the SDB by
    // remap_section. The original event lookup identifies it by the same
    // location name in the retail table; the value is cached below.
    let template = TEMPLATE_WORLD_SECTION.load(Ordering::Acquire);
    if original == template {
        custom
    } else {
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_custom_track_has_a_unique_id_and_registered_peak() {
        for (index, track) in TRACKS.iter().enumerate() {
            assert!(peak(track.peak_id).is_some());
            assert!(!TRACKS[index + 1..].iter().any(|later| later.id == track.id));
            assert_eq!(super::track(track.id), Some(track));
        }
    }
}
