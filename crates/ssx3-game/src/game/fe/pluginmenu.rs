//! Plugin peaks and events on the Select Peak screen (`docs/plans/plugin-peaks.md`).
//!
//! The plugins' catalog (`plugin.toml`: `[[peak]]`, `[[event]]`) becomes a
//! [`MenuPlan`] at boot: which rows the screen gets, what they are called in the
//! screen's data, and which event each row starts. The plan is pure data
//! (unit-tested here); the screen itself is patched by [`super::lui_patch`] and
//! the rows are driven by the hooks further down.
//!
//! Nothing here runs without a plugin that has a peak with an event: the plan
//! is empty, no file is generated and no hook is installed.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use ssx3_plugin::catalog::Catalog;
use ssx3_plugin::manifest::Kind;

use crate::game::fe::pluginpic;
use crate::game::hashvalue::str_hash;

/// Peaks that fit on the screen next to the three of the game (the list has no
/// scrolling: it is as tall as its rows).
pub const MAX_PEAKS: usize = 4;
/// Rows in one event list (22 lines each).
pub const MAX_ROWS: usize = 12;
/// Longest name shown, in ASCII characters.
pub const MAX_NAME: usize = 31;

/// What a row says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    /// A key of the game's locale table (`name_key`): hashed.
    Key(u32),
    /// Plain ASCII text.
    Ascii(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventPlan {
    /// `<plugin>/<id>`.
    pub id: String,
    pub label: Label,
    /// The native event whose rules it takes (`SSX3_EVENT=list`).
    pub template: String,
    /// The location name loaded for it (`data/worlds/<world>.big`).
    pub world: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PeakPlan {
    /// Position among the plugin peaks (0 based).
    pub index: usize,
    /// `<plugin>/<id>`.
    pub id: String,
    pub plugin: String,
    pub name: String,
    pub picture: Option<PathBuf>,
    pub race: Vec<EventPlan>,
    pub freestyle: Vec<EventPlan>,
}

/// A kind of event list: the goal tab of a peak (0 race, 1 freestyle).
pub const RACE: usize = 0;
pub const FREESTYLE: usize = 1;

impl PeakPlan {
    /// The events of a tab (0 race, 1 freestyle).
    pub fn events(&self, tab: usize) -> &[EventPlan] {
        if tab == FREESTYLE {
            &self.freestyle
        } else {
            &self.race
        }
    }

    /// The tabs that have events, in screen order.
    pub fn tabs(&self) -> Vec<usize> {
        [RACE, FREESTYLE]
            .into_iter()
            .filter(|t| !self.events(*t).is_empty())
            .collect()
    }
}

/// What a row of the patched screen is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Widget {
    /// A plugin peak's row in the peak list.
    Peak { peak: usize },
    /// The Race / Freestyle row of a peak's goal list.
    Goal { peak: usize, tab: usize },
    /// An event row.
    Event { peak: usize, tab: usize, row: usize },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MenuPlan {
    pub peaks: Vec<PeakPlan>,
    /// What was dropped or changed, for the log.
    pub warnings: Vec<String>,
    by_hash: HashMap<u32, Widget>,
}

/// The widget names in the screen's data. Index based and ASCII, so plugins
/// cannot clash with each other or with the game.
pub mod names {
    pub fn peak(p: usize) -> String {
        format!("PlugPeak{p}")
    }
    pub fn goals(p: usize) -> String {
        format!("PlugGoals{p}")
    }
    pub fn goal(p: usize, tab: usize) -> String {
        format!("PlugGoal{p}{}", if tab == 0 { "Race" } else { "Free" })
    }
    pub fn locs(p: usize, tab: usize) -> String {
        format!("Plug{p}{}Locs", if tab == 0 { "Race" } else { "Free" })
    }
    pub fn event(p: usize, tab: usize, row: usize) -> String {
        format!("Plug{p}{}Loc{row}", if tab == 0 { "Race" } else { "Free" })
    }
}

/// Printable ASCII only, at most [`MAX_NAME`] characters: the text goes to the
/// game's ASCII string setter and must not carry anything it could mistake.
pub fn clean_name(s: &str) -> String {
    s.chars()
        .map(|ch| if (' '..='~').contains(&ch) { ch } else { '?' })
        .take(MAX_NAME)
        .collect::<String>()
        .trim()
        .to_string()
}

impl MenuPlan {
    /// Build the plan from the plugins' catalog: catalog order (plugin id, then
    /// manifest order), at most [`MAX_PEAKS`] peaks with at most [`MAX_ROWS`]
    /// events per list; a peak without events is not listed.
    pub fn build(catalog: &Catalog) -> MenuPlan {
        let mut plan = MenuPlan::default();
        for peak in &catalog.peaks {
            let mut race = Vec::new();
            let mut freestyle = Vec::new();
            for e in catalog.events_of(&peak.id) {
                let list = match e.kind {
                    Kind::Race => &mut race,
                    Kind::Freestyle => &mut freestyle,
                };
                if list.len() == MAX_ROWS {
                    plan.warnings.push(format!(
                        "plugin {}: event {} left out: a list holds {MAX_ROWS} events",
                        e.plugin, e.id
                    ));
                    continue;
                }
                let local = e.id.split_once('/').map_or(e.id.as_str(), |(_, l)| l);
                let label = match (&e.name_key, &e.name) {
                    (Some(k), _) => Label::Key(str_hash(k.as_bytes())),
                    (None, Some(n)) => Label::Ascii(clean_name(n)),
                    (None, None) => Label::Ascii(clean_name(local)),
                };
                list.push(EventPlan {
                    id: e.id.clone(),
                    label,
                    template: e.template.clone(),
                    world: e.world.clone(),
                });
            }
            if race.is_empty() && freestyle.is_empty() {
                plan.warnings.push(format!(
                    "plugin {}: peak {} has no events and is not listed",
                    peak.plugin, peak.id
                ));
                continue;
            }
            if plan.peaks.len() == MAX_PEAKS {
                plan.warnings.push(format!(
                    "plugin {}: peak {} left out: the screen holds {MAX_PEAKS} plugin peaks",
                    peak.plugin, peak.id
                ));
                continue;
            }
            plan.peaks.push(PeakPlan {
                index: plan.peaks.len(),
                id: peak.id.clone(),
                plugin: peak.plugin.clone(),
                name: clean_name(&peak.name),
                picture: peak.picture.clone(),
                race,
                freestyle,
            });
        }
        plan.index_widgets();
        plan
    }

    fn index_widgets(&mut self) {
        let mut map = HashMap::new();
        for p in &self.peaks {
            map.insert(
                str_hash(names::peak(p.index).as_bytes()),
                Widget::Peak { peak: p.index },
            );
            for tab in p.tabs() {
                map.insert(
                    str_hash(names::goal(p.index, tab).as_bytes()),
                    Widget::Goal {
                        peak: p.index,
                        tab,
                    },
                );
                for row in 0..p.events(tab).len() {
                    map.insert(
                        str_hash(names::event(p.index, tab, row).as_bytes()),
                        Widget::Event {
                            peak: p.index,
                            tab,
                            row,
                        },
                    );
                }
            }
        }
        self.by_hash = map;
    }

    pub fn is_empty(&self) -> bool {
        self.peaks.is_empty()
    }

    /// The row a widget name hash (`widget + 0x38`) is, if it is ours.
    pub fn widget(&self, name_hash: u32) -> Option<Widget> {
        self.by_hash.get(&name_hash).copied()
    }

    /// The event a row starts.
    pub fn event(&self, peak: usize, tab: usize, row: usize) -> Option<&EventPlan> {
        self.peaks.get(peak)?.events(tab).get(row)
    }
}

// ------------------------------------------------------------------ runtime

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_plugin::hooks::Prev;

use crate::call::{call_ptr, Flow, Unwound, VSlot};

static PLAN: OnceLock<MenuPlan> = OnceLock::new();
/// Which peaks have a usable picture of their own (`plug_<n>.ssh` in the
/// generated `MAPGFX.BIG`); the others show the mountain.
static PICTURES: OnceLock<Vec<bool>> = OnceLock::new();

/// The plan of this run (empty without plugins).
pub fn plan() -> &'static MenuPlan {
    PLAN.get_or_init(MenuPlan::default)
}

const ON_WIDGET_CREATED: u32 = 0x0020_09d0;
const ON_WIDGET_ACTION: u32 = 0x0020_18a8;
const SET_MODE: u32 = 0x0020_30a0;
const UPDATE_MODE_PANELS: u32 = 0x0020_2ad8;
const UPDATE: u32 = 0x0020_2828;
const SHUTDOWN: u32 = 0x0020_0af0;
const QUIT_TO_LOAD_SCREEN: u32 = 0x0023_0488;
const HIDE_MAP_PIC: u32 = 0x0020_1868;
const LOAD_MAP_IMAGE: u32 = 0x0020_3990;
const UNLOAD_FE_ASYNC_FILE: u32 = 0x001a_38c0;
const LOAD_1_PEAK_PIC: u32 = 0x001a_3768;
const SET_HIGHLIGHT_COLOR: u32 = 0x0020_83d8;
const SET_UNLOCKED_COLOR: u32 = 0x0020_8438;
const FIND_OBJECT: u32 = 0x0039_d860;
const SET_STRING: u32 = 0x003a_0c50;
const SET_TEXT_ID: u32 = 0x003a_1000;
const REFRESH_TEXT: u32 = 0x003a_0528;
const BE_LIBRARY_SET_PHASE: u32 = 0x0014_de68;
const EA_MEM_SET: u32 = 0x003e_6448;

/// The new-race settings: the track at +0x40, the event-type byte at +0x48.
const NEW_RACE_INFO: u32 = 0x0053_5bc8;

// `EventSelect` (`this`) fields.
const THIS_CONFIRM: u32 = 0x2cc;
const THIS_SCREEN: u32 = 0x2e8;
const THIS_STATE: u32 = 0x2ec;
/// The picture variant loaded (the game's own are 1 to 5), the async slot it
/// was requested in, whether it has been bound, and the "no picture" flag.
const THIS_VARIANT: u32 = 0x2d4;
const THIS_SLOT: u32 = 0x2d8;
const THIS_BOUND: u32 = 0x2dc;
const THIS_NO_PICTURE: u32 = 0x2f0;
/// Plugin peak `p` is picture variant `PICTURE_VARIANT + p`.
const PICTURE_VARIANT: u32 = 0x100;
// `gp`-relative globals.
const GP_FLOW: u32 = 0xb5c;
const GP_MODE: u32 = 0xb54;
const GP_TAB: u32 = 0xb50;
const GP_PEAK: u32 = 0xb4c;
const GP_EVENT_STARTED: u32 = 0x1f38;
const GP_WORLD: u32 = 0x848;

/// The plugin peak whose goals or events are shown (none: `u32::MAX`).
const NONE: u32 = u32::MAX;
static SELECTED: AtomicU32 = AtomicU32::new(NONE);
/// The frame of the last plugin accept: a menu shown by an accept is next in
/// the same frame's input traversal, and must not take the same press.
static LAST_ACCEPT: AtomicU32 = AtomicU32::new(NONE);
/// What the heading text was last set for (screen, peak, mode, tab).
static HEADING: AtomicU64 = AtomicU64::new(0);
/// What the help text was last set for (screen, peak, tab, row).
static HELP: AtomicU64 = AtomicU64::new(0);

static CREATED: Prev = Prev::new();
static ACTION: Prev = Prev::new();
static SET_MODE_PREV: Prev = Prev::new();
static PANELS: Prev = Prev::new();
static UPDATE_PREV: Prev = Prev::new();
static SHUTDOWN_PREV: Prev = Prev::new();
static QUIT_PREV: Prev = Prev::new();
static LOAD_PIC: Prev = Prev::new();

/// Build the plan from the plugins' catalog and, when it has peaks, make the
/// patched `FE.LUI` available to the file layer and hook the screen. Call once
/// at boot, after the catalog is published, the game's own installers have run
/// and before the game opens the file. Does nothing (touches no slot,
/// registers nothing) without plugin peaks.
pub fn install() {
    let plan = MenuPlan::build(crate::plugin_host::catalog());
    for w in &plan.warnings {
        eprintln!("[plugin-menu] {w}");
    }
    if plan.is_empty() || PLAN.set(plan).is_err() {
        return;
    }
    for p in &self::plan().peaks {
        eprintln!(
            "[plugin-menu] peak {} ({}): {} race, {} freestyle events",
            p.id,
            p.name,
            p.race.len(),
            p.freestyle.len()
        );
    }
    crate::native_files::register_generator(
        "DATA/UI/FE.LUI",
        Box::new(|original| match super::lui_patch::patch_lui(original, self::plan()) {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                eprintln!("[plugin-menu] FE.LUI not patched, the plugin peaks stay off the screen: {e}");
                None
            }
        }),
    );
    crate::event_boot::install_world_swap();
    let pictures: Vec<bool> = self::plan()
        .peaks
        .iter()
        .map(|p| match &p.picture {
            None => false,
            Some(path) => match std::fs::read(path).map_err(|e| e.to_string()).and_then(|b| pluginpic::decode_png(&b).map(|_| ())) {
                Ok(()) => true,
                Err(e) => {
                    eprintln!("[plugin-menu] picture of {} ({}): {e}; the mountain is shown instead", p.id, path.display());
                    false
                }
            },
        })
        .collect();
    if pictures.iter().any(|b| *b) {
        crate::native_files::register_generator(
            "DATA/UI/MAPGFX.BIG",
            Box::new(|original| {
                match pluginpic::patch_mapgfx(original, self::plan(), &|p| std::fs::read(p)) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        eprintln!("[plugin-menu] MAPGFX.BIG not patched, plugin peaks show the mountain: {e}");
                        None
                    }
                }
            }),
        );
    }
    let _ = PICTURES.set(pictures);
    chain(LOAD_MAP_IMAGE, load_map_image, &LOAD_PIC);
    chain(ON_WIDGET_CREATED, on_widget_created, &CREATED);
    chain(ON_WIDGET_ACTION, on_widget_action, &ACTION);
    chain(SET_MODE, set_mode, &SET_MODE_PREV);
    chain(UPDATE_MODE_PANELS, update_mode_panels, &PANELS);
    chain(UPDATE, update, &UPDATE_PREV);
    chain(SHUTDOWN, shutdown, &SHUTDOWN_PREV);
    chain(QUIT_TO_LOAD_SCREEN, quit_to_load_screen, &QUIT_PREV);
}

/// Wrap the function in a slot: what it held (the hand-written port, else the
/// generated body) is what `prev` calls.
fn chain(addr: u32, f: GuestFn, prev: &Prev) {
    let was = ssx3_ee::get_override(addr)
        .or_else(|| ssx3_ee::generated_body(addr))
        .expect("plugin menu: no function to wrap");
    prev.set_for_host(was);
    assert!(
        ssx3_ee::set_override(addr, f),
        "plugin menu: {addr:#x} has no override slot"
    );
}

fn gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// Run what the slot held, then `extra` with every register as the function
/// left it (the function's caller sees nothing of the extra calls).
fn after(c: &mut Cpu, prev: &Prev, extra: impl FnOnce(&mut Cpu, u32) -> Flow) {
    let back = c.g[RA][0] as u32;
    prev.call(c);
    if c.pc != back {
        // The function was switched away from (a guest thread switch).
        return;
    }
    let (g, f) = (c.g, c.f);
    if extra(c, back).is_ok() {
        c.g = g;
        c.f = f;
        c.pc = back;
    }
}

/// Plugin rows only exist in the front end (flows 0 and 1), outside the
/// confirm popup.
fn front_end(c: &mut Cpu, this: u32) -> bool {
    c.bus.read32(gp(c, GP_FLOW)) < 2 && c.bus.read32(this + THIS_CONFIRM) == 0
}

fn widget_id(w: Widget) -> u32 {
    match w {
        Widget::Peak { peak } => 4 + peak as u32,
        Widget::Goal { tab, .. } => tab as u32,
        Widget::Event { row, .. } => row as u32,
    }
}

/// Our row for a widget pointer.
fn row_of(c: &mut Cpu, widget: u32) -> Option<Widget> {
    if widget == 0 {
        return None;
    }
    plan().widget(c.bus.read32(widget + 0x38))
}

// ---- calling the game's UI

fn find_object(c: &mut Cpu, this: u32, name: &str, back: u32) -> Result<u32, Unwound> {
    c.g[A0][0] = sx(c.bus.read32(this + THIS_SCREEN));
    c.g[A1][0] = sx(str_hash(name.as_bytes()));
    call_ptr(c, FIND_OBJECT, back)?;
    Ok(c.g[V0][0] as u32)
}

/// Show or hide an object through its own vtable, as the stock `SetMode` does.
fn set_visible(c: &mut Cpu, obj: u32, visible: bool, back: u32) -> Flow {
    if obj == 0 {
        return Ok(());
    }
    if visible {
        let flags = c.bus.read32(obj + 0x14);
        c.bus.write32(obj + 0x14, flags | 1);
    }
    let slots = if visible { [0x48, 0x38] } else { [0x38, 0x48] };
    for slot in slots {
        let table = c.bus.read32(obj + 8);
        let delta = c.bus.read16(table + slot) as i16 as i32;
        let target = c.bus.read32(table + slot + 4);
        c.g[A0][0] = sx(obj.wrapping_add(delta as u32));
        c.g[A1][0] = visible as u64;
        call_ptr(c, target, back)?;
    }
    Ok(())
}

fn set_named_visible(c: &mut Cpu, this: u32, name: &str, visible: bool, back: u32) -> Flow {
    let obj = find_object(c, this, name, back)?;
    set_visible(c, obj, visible, back)
}

/// Text from a string of ours: copied below the stack pointer for the call.
fn set_ascii(c: &mut Cpu, obj: u32, text: &str, back: u32) -> Flow {
    if obj == 0 {
        return Ok(());
    }
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x40);
    c.g[SP][0] = sx(sp);
    for (i, b) in text.bytes().chain(std::iter::once(0)).enumerate() {
        c.bus.write8(sp + 0x10 + i as u32, b);
    }
    c.g[A0][0] = sx(obj);
    c.g[A1][0] = sx(sp + 0x10);
    let r = call_ptr(c, SET_STRING, back);
    if r.is_ok() {
        c.g[SP][0] = sx(sp.wrapping_add(0x40));
    }
    r
}

fn set_label(c: &mut Cpu, obj: u32, label: &Label, back: u32) -> Flow {
    if obj == 0 {
        return Ok(());
    }
    match label {
        Label::Ascii(s) => set_ascii(c, obj, s, back),
        Label::Key(hash) => {
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(*hash);
            call_ptr(c, SET_TEXT_ID, back)?;
            c.g[A0][0] = sx(obj);
            call_ptr(c, REFRESH_TEXT, back)
        }
    }
}

/// Front-end sound `n` through the screen state's sound player (else the
/// default one), as the stock accept does.
fn sound(c: &mut Cpu, this: u32, n: u32, back: u32) -> Flow {
    let mut player = (c.g[GP][0] as u32).wrapping_add(0x2968);
    let state = c.bus.read32(this + THIS_STATE);
    let owner = c.bus.read32(state + 0x10);
    let own = c.bus.read32(owner + 0x14);
    if own != 0 {
        player = own;
    }
    let vt = c.bus.read32(player);
    let m = VSlot::read(c, vt as u64, 0x10);
    c.g[A0][0] = sx(player.wrapping_add(m.delta as u32));
    c.g[A1][0] = n as u64;
    call_ptr(c, m.func as u32, back)
}

// ---- the hooks

/// Plugin rows get their id (the stock code reads it from `+0x18`; the screen
/// data has none for them).
fn on_widget_created(c: &mut Cpu) {
    let w = c.g[A1][0] as u32;
    if let Some(row) = row_of(c, w) {
        c.bus.write32(w + 0x18, widget_id(row));
    }
    CREATED.call(c)
}

/// The rows' texts, panels and headings follow the mode.
fn set_mode(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    let mode = c.g[A1][0] as u32;
    if mode == 0 {
        SELECTED.store(NONE, Ordering::Relaxed);
        crate::event_boot::end_plugin_event(c);
    }
    after(c, &SET_MODE_PREV, |c, back| sync_panels(c, this, mode, back));
}

fn sync_panels(c: &mut Cpu, this: u32, mode: u32, back: u32) -> Flow {
    if !front_end(c, this) {
        return Ok(());
    }
    let plan = plan();
    // Texts: set again at every mode change (the screen rebuilds its rows).
    for p in &plan.peaks {
        let obj = find_object(c, this, &names::peak(p.index), back)?;
        set_ascii(c, obj, &p.name, back)?;
        for tab in p.tabs() {
            let obj = find_object(c, this, &names::goal(p.index, tab), back)?;
            set_ascii(c, obj, if tab == FREESTYLE { "Freestyle" } else { "Race" }, back)?;
            for (row, e) in p.events(tab).iter().enumerate() {
                let obj = find_object(c, this, &names::event(p.index, tab, row), back)?;
                set_label(c, obj, &e.label, back)?;
            }
        }
    }
    // Panels: ours hidden, then the selected peak's shown in place of the
    // stock panel (peak 1's) the stock code chose.
    let selected = SELECTED.load(Ordering::Relaxed);
    let tab = c.bus.read32(gp(c, GP_TAB)) as usize;
    for p in &plan.peaks {
        let mine = p.index as u32 == selected;
        set_named_visible(c, this, &names::goals(p.index), mine && mode == 1, back)?;
        for t in p.tabs() {
            let show = mine && mode == 2 && t == tab;
            set_named_visible(c, this, &names::locs(p.index, t), show, back)?;
        }
    }
    if selected != NONE {
        match mode {
            1 => set_named_visible(c, this, "SPG_Peak1Goals", false, back)?,
            2 => {
                let stock = if tab == FREESTYLE {
                    "Peak1FreestyleLocations"
                } else {
                    "Peak1RaceLocations"
                };
                set_named_visible(c, this, stock, false, back)?;
            }
            _ => {}
        }
    }
    Ok(())
}

/// The headings and info panels of the peak a plugin row is about.
fn update_mode_panels(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    after(c, &PANELS, |c, back| {
        // The game has just written its own help text for its own event.
        HELP.store(0, Ordering::Relaxed);
        context(c, this, back)
    });
}

fn update(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    after(c, &UPDATE_PREV, |c, back| context(c, this, back));
}

/// The plugin peak the screen is about: the one picked (modes 1 and 2), or
/// the highlighted row of the peak list (mode 0).
fn context_peak(c: &mut Cpu, this: u32, mode: u32, back: u32) -> Result<Option<usize>, Unwound> {
    if mode != 0 {
        let s = SELECTED.load(Ordering::Relaxed);
        return Ok((s != NONE).then_some(s as usize));
    }
    let list = find_object(c, this, "SP_PeakList", back)?;
    if list == 0 {
        return Ok(None);
    }
    let row = c.bus.read32(list + 0xa0);
    Ok(match row_of(c, row) {
        Some(Widget::Peak { peak }) => Some(peak),
        _ => None,
    })
}

const INFO_PANELS: [&str; 7] = [
    "SP_Info",
    "SP_GameInfo",
    "SE_Info",
    "Indicators",
    "peak1outline",
    "peak2outline",
    "peak3outline   ",
];

fn context(c: &mut Cpu, this: u32, back: u32) -> Flow {
    if !front_end(c, this) {
        return Ok(());
    }
    let mode = c.bus.read32(gp(c, GP_MODE));
    let Some(p) = context_peak(c, this, mode, back)? else {
        HEADING.store(0, Ordering::Relaxed);
        if c.bus.read32(this + THIS_VARIANT) >= PICTURE_VARIANT {
            // Back on the game's own row: its picture again.
            c.g[A0][0] = sx(this);
            call_ptr(c, LOAD_MAP_IMAGE, back)?;
        }
        return Ok(());
    };
    request_picture(c, this, p, back)?;
    let Some(peak) = plan().peaks.get(p) else {
        return Ok(());
    };
    // The game's own info panels talk about its peaks.
    for name in INFO_PANELS {
        set_named_visible(c, this, name, false, back)?;
    }
    let tab = c.bus.read32(gp(c, GP_TAB));
    let key = (this as u64) << 32 | (p as u64) << 16 | (mode as u64) << 8 | tab as u64 & 0xff | 1 << 63;
    let caption = find_object(c, this, "PEAKno", back)?;
    // The stock refresh puts a locale text back (+0xb0 set); ours has none.
    if HEADING.load(Ordering::Relaxed) != key
        || (caption != 0 && c.bus.read32(caption + 0xb0) != 0)
    {
        let text = if mode == 2 {
            if tab as usize == FREESTYLE {
                "Freestyle"
            } else {
                "Race"
            }
        } else {
            &peak.name
        };
        set_ascii(c, caption, text, back)?;
        HEADING.store(key, Ordering::Relaxed);
    }
    if mode == 2 {
        help_text(c, this, p, tab as usize, back)?;
    }
    Ok(())
}

/// The help line under the list: the highlighted event's own name (the game
/// writes the help of the template event it starts, which is not what the
/// player picked).
fn help_text(c: &mut Cpu, this: u32, p: usize, tab: usize, back: u32) -> Flow {
    let menu = find_object(c, this, &names::locs(p, tab), back)?;
    if menu == 0 {
        return Ok(());
    }
    let item = c.bus.read32(menu + 0xa0);
    let Some(Widget::Event { row, .. }) = row_of(c, item) else {
        return Ok(());
    };
    let key = (this as u64) << 32 | (p as u64) << 24 | (tab as u64) << 16 | row as u64 | 1 << 63;
    if HELP.load(Ordering::Relaxed) == key {
        return Ok(());
    }
    let Some(event) = plan().event(p, tab, row) else {
        return Ok(());
    };
    let help = find_object(c, this, "HelpText", back)?;
    set_label(c, help, &event.label, back)?;
    HELP.store(key, Ordering::Relaxed);
    Ok(())
}

/// Ask the front end's async loader for the peak's picture, as the game does
/// for its own (`Load1PeakPic` in the screen's slot), then name our member in
/// the request: the `|` prefix is a member of `DATA/UI/MAPGFX.BIG`.
fn request_picture(c: &mut Cpu, this: u32, p: usize, back: u32) -> Flow {
    let variant = PICTURE_VARIANT + p as u32;
    c.bus.write32(this + THIS_NO_PICTURE, 0);
    if c.bus.read32(this + THIS_VARIANT) == variant {
        return Ok(());
    }
    let own = PICTURES.get().and_then(|v| v.get(p)).copied().unwrap_or(false);
    let member = if own {
        pluginpic::member_name(p)
    } else {
        pluginpic::FALLBACK_MEMBER.to_string()
    };
    let world = c.bus.read32(gp(c, GP_WORLD));
    let manager = c.bus.read32(world + 0x11c);
    let slot = c.bus.read32(this + THIS_SLOT);
    c.g[A0][0] = sx(manager);
    c.g[A1][0] = sx(slot);
    call_ptr(c, UNLOAD_FE_ASYNC_FILE, back)?;
    c.g[A0][0] = sx(manager);
    c.g[A1][0] = 0;
    c.g[A2][0] = sx(slot);
    c.g[A3][0] = 1;
    call_ptr(c, LOAD_1_PEAK_PIC, back)?;
    let request = manager + slot * 0x11c;
    for (i, b) in format!("|{member}").bytes().chain(std::iter::once(0)).enumerate() {
        c.bus.write8(request + i as u32, b);
    }
    c.bus.write32(this + THIS_VARIANT, variant);
    c.bus.write32(this + THIS_BOUND, 0);
    Ok(())
}

/// The map picture: ours for a plugin peak, the game's otherwise.
fn load_map_image(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    let back = c.g[RA][0] as u32;
    let plugin = front_end(c, this) && {
        let mode = c.bus.read32(gp(c, GP_MODE));
        mode != 0 && SELECTED.load(Ordering::Relaxed) != NONE
    };
    if !plugin {
        return LOAD_PIC.call(c);
    }
    let p = SELECTED.load(Ordering::Relaxed) as usize;
    let (g, f) = (c.g, c.f);
    if request_picture(c, this, p, back).is_ok() {
        c.g = g;
        c.f = f;
        c.pc = back;
    }
}

fn shutdown(c: &mut Cpu) {
    SELECTED.store(NONE, Ordering::Relaxed);
    SHUTDOWN_PREV.call(c)
}

/// Leaving a world ends the plugin event: the next load is the game's own.
fn quit_to_load_screen(c: &mut Cpu) {
    crate::event_boot::end_plugin_event(c);
    QUIT_PREV.call(c)
}

/// `EventSelect_OnWidgetAction(this, widget, action)`: plugin rows are ours,
/// everything else (and the actions that only change mode) is the game's.
fn on_widget_action(c: &mut Cpu) {
    let (this, w, action) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
    let back = c.g[RA][0] as u32;
    let row = row_of(c, w);
    if row.is_none() && action == 5 {
        // A native accept: whatever it starts is not a plugin's event.
        crate::event_boot::end_plugin_event(c);
    }
    let Some(row) = row else {
        return ACTION.call(c);
    };
    if !matches!(action, 1 | 2 | 5) || !front_end(c, this) {
        return ACTION.call(c);
    }
    let (g, f) = (c.g, c.f);
    match row_action(c, this, w, row, action, back) {
        Ok(v0) => {
            c.g = g;
            c.f = f;
            c.g[V0][0] = v0;
            c.pc = back;
        }
        Err(Unwound) => {}
    }
}

fn row_action(c: &mut Cpu, this: u32, w: u32, row: Widget, action: u32, back: u32) -> Result<u64, Unwound> {
    let mode = c.bus.read32(gp(c, GP_MODE));
    match action {
        1 | 2 => {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(w);
            c.g[A2][0] = 1;
            let colour = if action == 1 { SET_HIGHLIGHT_COLOR } else { SET_UNLOCKED_COLOR };
            call_ptr(c, colour, back)?;
            if action == 1 && matches!(mode, 1 | 2) {
                grow_highlight(c, this, w, if mode == 1 { "pghl" } else { "ehl" }, back)?;
            }
            Ok(0)
        }
        _ => accept(c, this, row, mode, back),
    }
}

/// The tab or event highlight bar goes to the row (y + 100), as the stock code
/// does.
fn grow_highlight(c: &mut Cpu, this: u32, w: u32, name: &str, back: u32) -> Flow {
    let bar = find_object(c, this, name, back)?;
    if bar != 0 {
        let y = f32::from_bits(c.bus.read32(w + 0x48)) + 100.0;
        c.bus.write32(bar + 0x48, y.to_bits());
    }
    Ok(())
}

fn accept(c: &mut Cpu, this: u32, row: Widget, mode: u32, back: u32) -> Result<u64, Unwound> {
    let frame = crate::tick::vsync_count(c);
    if LAST_ACCEPT.swap(frame, Ordering::Relaxed) == frame {
        return Ok(0);
    }
    let gp_peak = gp(c, GP_PEAK);
    let gp_tab = gp(c, GP_TAB);
    match (row, mode) {
        (Widget::Peak { peak }, 0) => {
            sound(c, this, 6, back)?;
            SELECTED.store(peak as u32, Ordering::Relaxed);
            // The game's own state sees peak 1; our hooks swap its panels.
            c.bus.write32(gp_peak, 0);
            show_mode(c, this, 1, back)?;
            Ok(0)
        }
        (Widget::Goal { tab, .. }, 1) => {
            sound(c, this, 6, back)?;
            c.bus.write32(gp_tab, tab as u32);
            show_mode(c, this, 2, back)?;
            Ok(0)
        }
        (Widget::Event { peak, tab, row }, 2) => start_event(c, this, peak, tab, row, back),
        _ => Ok(0),
    }
}

/// `SetMode`, the panel headings, and the map picture.
fn show_mode(c: &mut Cpu, this: u32, mode: u32, back: u32) -> Flow {
    c.g[A0][0] = sx(this);
    c.g[A1][0] = mode as u64;
    call_ptr(c, SET_MODE, back)?;
    c.g[A0][0] = sx(this);
    call_ptr(c, UPDATE_MODE_PANELS, back)?;
    c.g[A0][0] = sx(this);
    call_ptr(c, LOAD_MAP_IMAGE, back)
}

/// The front-end tail of the stock event accept (`es_start_event`) with the
/// plugin event's template event in place of the record table.
fn start_event(c: &mut Cpu, this: u32, peak: usize, tab: usize, row: usize, back: u32) -> Result<u64, Unwound> {
    let Some(event) = plan().event(peak, tab, row) else {
        return Ok(0);
    };
    let template = crate::event_boot::find(&crate::event_boot::events(c), &event.template);
    let Some(template) = template else {
        eprintln!(
            "[plugin-menu] {}: template event {:?} does not exist (SSX3_EVENT=list)",
            event.id, event.template
        );
        sound(c, this, 4, back)?;
        return Ok(0);
    };
    use crate::event_boot::{call, commit, interface};
    let flow = c.bus.read32(gp(c, GP_FLOW));
    let race = interface(c, 0, back)?;
    sound(c, this, 6, back)?;
    if flow == 1 {
        call(c, BE_LIBRARY_SET_PHASE, &[0], back)?;
    }
    c.bus.write32(NEW_RACE_INFO + 0x40, template.track);
    let was = c.bus.read8(NEW_RACE_INFO + 0x48);
    c.bus.write32(gp(c, GP_EVENT_STARTED), 1);
    call(c, crate::event_boot::BE_NEW_RACE_INT_SET_EVENT_TYPE, &[race, template.kind], back)?;
    call(c, crate::event_boot::BE_NEW_RACE_INT_SET_EVENT, &[race, template.event], back)?;
    commit(c, race, back)?;
    if flow == 1 {
        if c.bus.read8(NEW_RACE_INFO + 0x48) != was {
            call(c, EA_MEM_SET, &[0x0053_08b8, 0, 0x20], back)?;
        }
        let world = c.bus.read32(gp(c, GP_WORLD));
        let manager = c.bus.read32(world + 0x84);
        call(c, QUIT_TO_LOAD_SCREEN, &[manager, 1], back)?;
    }
    // After the quit above, which ends the previous plugin event's world.
    crate::event_boot::set_world(Some(event.world.clone()));
    if let Label::Ascii(name) = &event.label {
        crate::event_boot::set_course_name(c, template.track, name);
    }
    eprintln!("[plugin-menu] start {} (template {}, world {})", event.id, event.template, event.world);
    c.g[A0][0] = sx(this);
    call_ptr(c, HIDE_MAP_PIC, back)?;
    Ok(1)
}

#[cfg(test)]
mod tests {
    use ssx3_plugin::catalog::{Event, Peak};

    use super::*;

    fn peak(plugin: &str, id: &str, name: &str) -> Peak {
        Peak {
            id: format!("{plugin}/{id}"),
            plugin: plugin.to_string(),
            name: name.to_string(),
            picture: None,
        }
    }

    fn event(plugin: &str, id: &str, peak: &str, kind: Kind) -> Event {
        Event {
            id: format!("{plugin}/{id}"),
            plugin: plugin.to_string(),
            name: None,
            name_key: None,
            peak: format!("{plugin}/{peak}"),
            kind,
            template: "ARA1".to_string(),
            world: format!("w{id}"),
        }
    }

    #[test]
    fn an_empty_catalog_gives_an_empty_plan() {
        let p = MenuPlan::build(&Catalog::default());
        assert!(p.is_empty() && p.warnings.is_empty());
        assert_eq!(p.widget(str_hash(b"PlugPeak0")), None);
    }

    #[test]
    fn peaks_keep_catalog_order_and_two_plugins_coexist() {
        let cat = Catalog {
            peaks: vec![peak("a", "one", "One"), peak("a", "two", "Two"), peak("b", "x", "X")],
            events: vec![
                event("a", "r1", "one", Kind::Race),
                event("a", "f1", "one", Kind::Freestyle),
                event("a", "r2", "one", Kind::Race),
                event("a", "r3", "two", Kind::Race),
                event("b", "r4", "x", Kind::Race),
            ],
            problems: vec![],
        };
        let p = MenuPlan::build(&cat);
        assert_eq!(p.peaks.len(), 3);
        assert_eq!(
            p.peaks.iter().map(|x| x.id.as_str()).collect::<Vec<_>>(),
            ["a/one", "a/two", "b/x"]
        );
        assert_eq!(p.peaks[0].race.len(), 2);
        assert_eq!(p.peaks[0].freestyle.len(), 1);
        assert_eq!(p.peaks[0].tabs(), [RACE, FREESTYLE]);
        assert_eq!(p.peaks[1].tabs(), [RACE]);
        assert_eq!(p.peaks[2].index, 2);
        // The widget map covers exactly the rows of the plan.
        assert_eq!(
            p.widget(str_hash(names::peak(2).as_bytes())),
            Some(Widget::Peak { peak: 2 })
        );
        assert_eq!(
            p.widget(str_hash(names::event(0, 0, 1).as_bytes())),
            Some(Widget::Event {
                peak: 0,
                tab: 0,
                row: 1
            })
        );
        assert_eq!(
            p.widget(str_hash(names::goal(0, 1).as_bytes())),
            Some(Widget::Goal { peak: 0, tab: 1 })
        );
        // No freestyle goal row for a peak without freestyle events.
        assert_eq!(p.widget(str_hash(names::goal(1, 1).as_bytes())), None);
        assert_eq!(p.event(0, 0, 1).unwrap().id, "a/r2");
        assert_eq!(p.event(0, 1, 0).unwrap().id, "a/f1");
        assert!(p.event(0, 1, 1).is_none() && p.event(9, 0, 0).is_none());
    }

    #[test]
    fn widget_names_do_not_collide() {
        let mut seen = HashMap::new();
        for p in 0..MAX_PEAKS {
            let mut all = vec![names::peak(p), names::goals(p)];
            for tab in [RACE, FREESTYLE] {
                all.push(names::goal(p, tab));
                all.push(names::locs(p, tab));
                all.extend((0..MAX_ROWS).map(|r| names::event(p, tab, r)));
            }
            for n in all {
                assert_eq!(seen.insert(str_hash(n.as_bytes()), n.clone()), None, "{n}");
            }
        }
    }

    #[test]
    fn caps_empty_peaks_and_names() {
        let mut cat = Catalog::default();
        cat.peaks.push(peak("a", "empty", "Empty"));
        for i in 0..6 {
            cat.peaks.push(peak("a", &format!("p{i}"), &"N".repeat(80)));
            for j in 0..14 {
                let mut e = event("a", &format!("e{i}-{j}"), &format!("p{i}"), Kind::Race);
                e.name = Some(format!("Name\u{e9}\u{0}{j}"));
                cat.events.push(e);
            }
        }
        let p = MenuPlan::build(&cat);
        assert_eq!(p.peaks.len(), MAX_PEAKS);
        assert!(p.peaks.iter().all(|x| x.race.len() == MAX_ROWS));
        assert!(p.peaks.iter().all(|x| x.name.len() == MAX_NAME));
        // 1 empty peak, 4 peaks x 2 surplus events, 2 surplus peaks x 2 surplus events (peaks past the
        // cap still count their rows first), 2 surplus peaks.
        assert!(p.warnings.iter().any(|w| w.contains("a/empty has no events")), "{:?}", p.warnings);
        assert!(p.warnings.iter().any(|w| w.contains("a/p4 left out")), "{:?}", p.warnings);
        assert!(p.warnings.iter().any(|w| w.contains("a/e0-12 left out")), "{:?}", p.warnings);
        // Non-ASCII and control characters become '?'.
        assert_eq!(
            p.peaks[0].race[0].label,
            Label::Ascii("Name??0".to_string())
        );
    }

    #[test]
    fn labels_prefer_the_locale_key_then_the_name_then_the_id() {
        let mut cat = Catalog {
            peaks: vec![peak("a", "p", "P")],
            events: vec![
                event("a", "keyed", "p", Kind::Race),
                event("a", "named", "p", Kind::Race),
                event("a", "plain", "p", Kind::Race),
            ],
            problems: vec![],
        };
        cat.events[0].name_key = Some("kT_240SelectPeak".to_string());
        cat.events[0].name = Some("ignored".to_string());
        cat.events[1].name = Some("Named".to_string());
        let p = MenuPlan::build(&cat);
        let l: Vec<_> = p.peaks[0].race.iter().map(|e| e.label.clone()).collect();
        assert_eq!(l[0], Label::Key(str_hash(b"kT_240SelectPeak")));
        assert_eq!(l[1], Label::Ascii("Named".to_string()));
        assert_eq!(l[2], Label::Ascii("plain".to_string()));
    }
}
