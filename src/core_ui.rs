//! Hooks for the fork's in-place edits of the front-end, load-screen, world
//! cache and texture code. Every row of the table is one upstream function
//! the SSX Tricky fork changed inside its body; here each becomes a hook.
//!
//! Almost everything is a wrapper (`CHAINS`): code before and/or after
//! `PREV.call(c)`. A wrapper is entered at the function's address
//! (`c.pc == entry`); when a callee unwound and the function is re-entered in
//! the middle, the wrapper forwards to `PREV` untouched. Only the plugin's own
//! paths (custom course active, custom menu rows) deviate from the retail
//! cycle and register contract.
//!
//! | function | address | kind | what changed | uncertain |
//! |---|---|---|---|---|
//! | `cFEStateEventSelect_Update` | 0x001869d8 | chain, after | `refresh_catalog_preview(widget = self+0x48)` after `EventSelect_Update` | runs after the whole function, not before its epilogue; a yield inside the refresh resumes in the caller |
//! | `EventSelect_LoadMapImage` | 0x00203990 | chain, replaces PREV while a Tricky preview is set | `load_catalog_preview` instead of the retail map image | none |
//! | `EventSelect_OnWidgetAction` | 0x002018a8 | chain, before (+ own handling) | widget row id remap; custom peak row highlight / unlocked colour / accept; tab and event accept of the Tricky peak; `deactivate` on retail peak accept; debug log | big: see below |
//! | `EventSelect_UpdateModePanels` | 0x00202ad8 | chain, after (RA-selected) | `refresh_catalog_panels` after the two OnWidgetAction call sites (`es_refresh_mode`, `es_show_mode`) | selected by return address 0x0020_21fc / 0x0020_23c4 |
//! | `EventSelect_GetEventRecord` | 0x00200388 | chain, replaces PREV for one call site | record of the template event while a custom course is active (OnWidgetAction's `es_start_event`, return address 0x0020_1e10) | **no slot yet: needs a hookable.txt entry** |
//! | `GameStateFinish_Enter` | 0x00233c50 | chain (diagnostic) | finish trace `finish-enter`, `finish-delay-armed` | the second observation is made at the function's end |
//! | `GameUI_OpenFinishScreen` | 0x0020cc60 | chain (diagnostic) | finish trace; the per-branch `request`/`returned` details (0xc, 0xd) are reduced to enter/returned | diagnostic only |
//! | `SectionMan_Init` | 0x0022cd40 | chain, after | `remap_section(section_man)` | the fork called it before `SectionMan_InitNodePool`; it only reads memory and writes the slot map, so after is equivalent; a resumed call takes the manager from S2 |
//! | `SectionMan_GetEventWorldSection` | 0x0022e098 | chain, after | `v0 = event_world_section(v0)` | none |
//! | `GameStateResults_Update` | 0x00236e60 | chain, handles two custom cases itself | pending results screen retry; custom course leaves through `GameModule_QuitToLoadScreen(module, 0)` | cycles of the custom paths are approximate |
//! | `cPreFELoadScreen_Update` | 0x00232a08 | chain, before | `deactivate` when the app has no next module | the fork deactivated after the fade step of the same call |
//! | `TexMan_FreeNBTexInfo` | 0x003671c8 | chain, before | world texture slots 1500..world_texture_slots go to `TexMan_FreeBTexInfo` | none |
//! | `cUITextScroll_SetTextId` | 0x003a1f18 | chain, before | `a1 = menu_text_id(a1)` | none |
//! | `WorldBlockPool_AllocBlock` | 0x003a7768 | chain, before/after | lazy geometry blocks (128 KiB pools): allocate the payload on claim | none |
//! | `WorldBlockPool_FreeBlock` | 0x003a7790 | chain, before/after | free that payload on release | none |
//! | `WorldBlockPool_Init` | 0x003a7598 | chain; own copy of upstream's body for lazy pools only | no per-block payload allocation for lazy pools; other pools run PREV | the copy follows upstream at the time of writing |
//!
//! `EventSelect_OnWidgetAction`: the fork edited the helpers `es_highlight`,
//! `es_unlocked`, `es_accept_peak`, `es_accept_tab`, `es_accept_event` and
//! `es_start_event` inside the function. They are not separate guest
//! functions, so the wrapper decides the custom cases itself from the same
//! conditions (`action`, the `gp-0xb54` mode, flow `gp-0xb5c`, widget name
//! 0x0056b7e4) and either finishes with its own result (`Handled`) or lets
//! the retail body run:
//! - highlight (1) and unlocked (2) of a custom row: its own colour call,
//!   preview and highlight grow, return 0;
//! - accept (5) of the Tricky peak row, and of a tab row of that peak: own
//!   code (sound, mode switch), return 0;
//! - accept of an event row of that peak: the wrapper activates the track
//!   and sets the selected peak to 0, then lets the retail body start the
//!   event with the row's "unlocked" word (`self + 4*row + 0x20*tab + 0x20`)
//!   forced to 1 for the call (restored afterwards; left set if the call
//!   unwinds). `EventSelect_GetEventRecord` is hooked to deliver the template
//!   record at that call site.
//!
//! Not handled here: `ScriptEngine_QueueIntroScripts` (not in the files of
//! this module).

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_game::call::{call_ptr, guest, sp, Flow, Frame, Unwound};
use ssx3_game::events::{deliver_event_here, event_due};
use ssx3_plugin::Prev;

/// No full replacements: everything is a wrapper.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

static PREV_FE_STATE_EVENT_SELECT_UPDATE: Prev = Prev::new();
static PREV_LOAD_MAP_IMAGE: Prev = Prev::new();
static PREV_ON_WIDGET_ACTION: Prev = Prev::new();
static PREV_UPDATE_MODE_PANELS: Prev = Prev::new();
static PREV_GET_EVENT_RECORD: Prev = Prev::new();
static PREV_FINISH_ENTER: Prev = Prev::new();
static PREV_OPEN_FINISH_SCREEN: Prev = Prev::new();
static PREV_SECTION_MAN_INIT: Prev = Prev::new();
static PREV_SECTION_EVENT_WORLD: Prev = Prev::new();
static PREV_RESULTS_UPDATE: Prev = Prev::new();
static PREV_PRE_FE_UPDATE: Prev = Prev::new();
static PREV_FREE_NB_TEX_INFO: Prev = Prev::new();
static PREV_TEXT_SCROLL_SET_TEXT_ID: Prev = Prev::new();
static PREV_ALLOC_BLOCK: Prev = Prev::new();
static PREV_FREE_BLOCK: Prev = Prev::new();
static PREV_POOL_INIT: Prev = Prev::new();

pub static CHAINS: &[crate::Chain] = &[
    crate::Chain {
        addr: 0x0018_69d8,
        f: fe_state_event_select_update,
        prev: &PREV_FE_STATE_EVENT_SELECT_UPDATE,
        name: "cFEStateEventSelect_Update",
    },
    crate::Chain {
        addr: 0x0020_3990,
        f: event_select_load_map_image,
        prev: &PREV_LOAD_MAP_IMAGE,
        name: "EventSelect_LoadMapImage",
    },
    crate::Chain {
        addr: 0x0020_18a8,
        f: event_select_on_widget_action,
        prev: &PREV_ON_WIDGET_ACTION,
        name: "EventSelect_OnWidgetAction",
    },
    crate::Chain {
        addr: 0x0020_2ad8,
        f: event_select_update_mode_panels,
        prev: &PREV_UPDATE_MODE_PANELS,
        name: "EventSelect_UpdateModePanels",
    },
    crate::Chain {
        addr: 0x0020_0388,
        f: event_select_get_event_record,
        prev: &PREV_GET_EVENT_RECORD,
        name: "EventSelect_GetEventRecord",
    },
    crate::Chain {
        addr: 0x0023_3c50,
        f: game_state_finish_enter,
        prev: &PREV_FINISH_ENTER,
        name: "GameStateFinish_Enter",
    },
    crate::Chain {
        addr: 0x0020_cc60,
        f: game_ui_open_finish_screen,
        prev: &PREV_OPEN_FINISH_SCREEN,
        name: "GameUI_OpenFinishScreen",
    },
    crate::Chain {
        addr: 0x0022_cd40,
        f: section_man_init,
        prev: &PREV_SECTION_MAN_INIT,
        name: "SectionMan_Init",
    },
    crate::Chain {
        addr: 0x0022_e098,
        f: section_man_get_event_world_section,
        prev: &PREV_SECTION_EVENT_WORLD,
        name: "SectionMan_GetEventWorldSection",
    },
    crate::Chain {
        addr: 0x0023_6e60,
        f: game_state_results_update,
        prev: &PREV_RESULTS_UPDATE,
        name: "GameStateResults_Update",
    },
    crate::Chain {
        addr: 0x0023_2a08,
        f: c_pre_fe_load_screen_update,
        prev: &PREV_PRE_FE_UPDATE,
        name: "cPreFELoadScreen_Update",
    },
    crate::Chain {
        addr: 0x0036_71c8,
        f: tex_man_free_nb_tex_info,
        prev: &PREV_FREE_NB_TEX_INFO,
        name: "TexMan_FreeNBTexInfo",
    },
    crate::Chain {
        addr: 0x003a_1f18,
        f: c_ui_text_scroll_set_text_id,
        prev: &PREV_TEXT_SCROLL_SET_TEXT_ID,
        name: "cUITextScroll_SetTextId",
    },
    crate::Chain {
        addr: 0x003a_7768,
        f: world_block_pool_alloc_block,
        prev: &PREV_ALLOC_BLOCK,
        name: "WorldBlockPool_AllocBlock",
    },
    crate::Chain {
        addr: 0x003a_7790,
        f: world_block_pool_free_block,
        prev: &PREV_FREE_BLOCK,
        name: "WorldBlockPool_FreeBlock",
    },
    crate::Chain {
        addr: 0x003a_7598,
        f: world_block_pool_init,
        prev: &PREV_POOL_INIT,
        name: "WorldBlockPool_Init",
    },
];

// ---------------------------------------------------------------- helpers

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// The word at `gp - negative_offset`.
#[inline]
fn gp_word(c: &mut Cpu, negative_offset: u32) -> u32 {
    let address = r(c, GP).wrapping_sub(negative_offset);
    c.bus.read32(address)
}

// ------------------------------------------- cFEStateEventSelect_Update

const FE_STATE_EVENT_SELECT_UPDATE: u32 = 0x0018_69d8;

/// After the base state's update and the widget's, redraw a Tricky preview
/// (the widget is the state's `+0x48` member).
fn fe_state_event_select_update(c: &mut Cpu) {
    if c.pc != FE_STATE_EVENT_SELECT_UPDATE {
        PREV_FE_STATE_EVENT_SELECT_UPDATE.call(c);
        return;
    }
    let this = r(c, A0);
    let back = r(c, RA);
    PREV_FE_STATE_EVENT_SELECT_UPDATE.call(c);
    if c.pc != back || crate::custom_tracks::menu_preview().is_none() {
        return;
    }
    guest(c, |c| {
        crate::bonus_menu_mode::refresh_catalog_preview(c, this.wrapping_add(0x48), back)
    });
}

// ------------------------------------------------- EventSelect_LoadMapImage

fn event_select_load_map_image(c: &mut Cpu) {
    if c.pc == LOAD_MAP_IMAGE && crate::custom_tracks::menu_preview().is_some() {
        let this = r(c, A0);
        let back = r(c, RA);
        guest(c, |c| crate::bonus_menu_mode::load_catalog_preview(c, this, back));
        return;
    }
    PREV_LOAD_MAP_IMAGE.call(c);
}

// --------------------------------------------- EventSelect_OnWidgetAction

const ON_WIDGET_ACTION: u32 = 0x0020_18a8;
const LOAD_MAP_IMAGE: u32 = 0x0020_3990;
const UPDATE_MODE_PANELS: u32 = 0x0020_2ad8;
const SET_MODE: u32 = 0x0020_30a0;
const SET_HIGHLIGHT_COLOR: u32 = 0x0020_83d8;
const SET_UNLOCKED_COLOR: u32 = 0x0020_8438;
const STR_HASH: u32 = 0x0031_7670;
const FIND_SCREEN_OBJECT: u32 = 0x0039_d860;

/// `gp`-relative globals (negative offsets) of the event-select screen.
const GP_MODE: u32 = 0xb54;
const GP_TAB: u32 = 0xb50;
const GP_PEAK: u32 = 0xb4c;
const GP_FLOW: u32 = 0xb5c;

/// Name hash word of the "Peak4" row (the Tricky peak) in the peak panel.
const TRICKY_PEAK_ROW: u32 = 0x0056_b7e4;

/// What a custom case of OnWidgetAction decided.
enum Step {
    /// Finished: return this value.
    Handled(u32),
    /// Run the retail body (A0..A2, RA restored).
    Original,
    /// Run the retail body with the event row's unlocked word forced on.
    EventStart { flag: u32, old: u32 },
}

/// Play front-end sound `n` through the screen's sound player (the screen
/// state's, else the default one at `gp + 0x2968`).
fn es_sound(c: &mut Cpu, this: u32, n: u32, back: u32) -> Flow {
    let mut player = r(c, GP).wrapping_add(0x2968);
    let state = c.bus.read32(this.wrapping_add(0x2ec));
    let owner = c.bus.read32(state.wrapping_add(0x10));
    let custom_player = c.bus.read32(owner.wrapping_add(0x14));
    if custom_player != 0 {
        player = custom_player;
    }
    let table = c.bus.read32(player);
    let delta = c.bus.read16(table.wrapping_add(0x10)) as i16 as i32;
    let target = c.bus.read32(table.wrapping_add(0x14));
    c.g[A1][0] = n as u64;
    c.g[A0][0] = sx(player.wrapping_add(delta as u32));
    call_ptr(c, target, back)
}

/// The panels of the new mode, then the map image (`es_show_mode`). The
/// UpdateModePanels hook below is selected by return address, so the custom
/// panel refresh is made here.
fn es_show_mode(c: &mut Cpu, this: u32, back: u32) -> Flow {
    c.g[A0][0] = sx(this);
    call_ptr(c, UPDATE_MODE_PANELS, back)?;
    let mode = gp_word(c, GP_MODE);
    crate::bonus_menu_mode::refresh_catalog_panels(c, this, mode, back)?;
    c.g[A0][0] = sx(this);
    call_ptr(c, LOAD_MAP_IMAGE, back)
}

/// Move a highlight object (named by the string at `name`) to the row's
/// y + 100 (`es_grow_highlight`).
fn es_grow_highlight(c: &mut Cpu, this: u32, row: u32, name: u32, back: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, STR_HASH, back)?;
    c.g[A0][0] = sx(c.bus.read32(this.wrapping_add(0x2e8)));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, FIND_SCREEN_OBJECT, back)?;
    let object = r(c, V0);
    if object == 0 {
        return Ok(());
    }
    let y = f32::from_bits(c.bus.read32(row.wrapping_add(0x48))) + 100.0;
    c.bus.write32(object.wrapping_add(0x48), y.to_bits());
    Ok(())
}

fn custom_action(
    c: &mut Cpu,
    this: u32,
    w: u32,
    action: u32,
    back: u32,
) -> Result<Step, Unwound> {
    let flow = gp_word(c, GP_FLOW);
    let mode = gp_word(c, GP_MODE);
    let name = c.bus.read32(w.wrapping_add(0x38));
    let assets = crate::custom_tracks::assets_available();
    let front = flow < 2;
    let custom_row = assets
        && front
        && ((mode == 0 && name == TRICKY_PEAK_ROW)
            || (crate::custom_tracks::bonus_peak_selected() && (mode == 1 || mode == 2)));
    match action {
        // Colour the row as highlighted.
        1 => {
            if assets && front {
                if custom_row {
                    if mode == 2 {
                        let tab = gp_word(c, GP_TAB);
                        let row = c.bus.read32(w.wrapping_add(0x18)) as usize;
                        if let Some(track) = crate::custom_tracks::menu_tracks(tab).nth(row) {
                            crate::custom_tracks::preview_track(track);
                        }
                    } else {
                        crate::custom_tracks::preview_logo();
                    }
                } else if mode == 0 && crate::custom_tracks::menu_preview().is_some() {
                    // Retail peak hovers share the mountain picture. After a
                    // custom picture, request the shared image again before
                    // the retail highlight.
                    crate::custom_tracks::clear_preview();
                    let registers = c.g;
                    let floats = c.f;
                    c.g[A0][0] = sx(this);
                    call_ptr(c, LOAD_MAP_IMAGE, back)?;
                    c.g = registers;
                    c.f = floats;
                }
            }
            if custom_row {
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(w);
                c.g[A2][0] = 1;
                call_ptr(c, SET_HIGHLIGHT_COLOR, back)?;
                crate::bonus_menu_mode::load_catalog_preview(c, this, back)?;
                let mode = gp_word(c, GP_MODE);
                if mode == 1 || mode == 2 {
                    let highlight = if mode == 1 { 0x004a_25f8 } else { 0x004a_2600 };
                    es_grow_highlight(c, this, w, highlight, back)?;
                }
                return Ok(Step::Handled(0));
            }
            Ok(Step::Original)
        }
        // Colour the row as unlocked.
        2 => {
            if custom_row {
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(w);
                c.g[A2][0] = 1;
                call_ptr(c, SET_UNLOCKED_COLOR, back)?;
                return Ok(Step::Handled(0));
            }
            Ok(Step::Original)
        }
        // Accept.
        5 => {
            let confirm = c.bus.read32(this.wrapping_add(0x2cc));
            match mode {
                0 => {
                    if confirm != 0 {
                        return Ok(Step::Original);
                    }
                    if assets && front && name == TRICKY_PEAK_ROW {
                        if !crate::custom_tracks::accept_menu_step(c) {
                            return Ok(Step::Handled(0));
                        }
                        crate::custom_tracks::select_bonus_peak();
                        es_sound(c, this, 6, back)?;
                        // The host category chooses its own one-item panels
                        // in EventSelect_SetMode.
                        let peak = r(c, GP).wrapping_sub(GP_PEAK);
                        c.bus.write32(peak, 0);
                        c.g[A0][0] = sx(this);
                        c.g[A1][0] = 1;
                        call_ptr(c, SET_MODE, back)?;
                        es_show_mode(c, this, back)?;
                        return Ok(Step::Handled(0));
                    }
                    let row = c.bus.read32(w.wrapping_add(0x18));
                    if row < 3 {
                        crate::custom_tracks::deactivate(c);
                    }
                    Ok(Step::Original)
                }
                1 if crate::custom_tracks::bonus_peak_selected() && front => {
                    if !crate::custom_tracks::accept_menu_step(c) {
                        return Ok(Step::Handled(0));
                    }
                    let tab = c.bus.read32(w.wrapping_add(0x18));
                    if tab > 1 {
                        es_sound(c, this, 4, back)?;
                        return Ok(Step::Handled(0));
                    }
                    es_sound(c, this, 6, back)?;
                    let tab_global = r(c, GP).wrapping_sub(GP_TAB);
                    c.bus.write32(tab_global, tab);
                    c.g[A0][0] = sx(this);
                    c.g[A1][0] = 2;
                    call_ptr(c, SET_MODE, back)?;
                    es_show_mode(c, this, back)?;
                    Ok(Step::Handled(0))
                }
                2 if crate::custom_tracks::bonus_peak_selected() && front => {
                    if !crate::custom_tracks::accept_menu_step(c) {
                        return Ok(Step::Handled(0));
                    }
                    let row = c.bus.read32(w.wrapping_add(0x18));
                    let tab = gp_word(c, GP_TAB);
                    let Some(track) = crate::custom_tracks::menu_tracks(tab).nth(row as usize)
                    else {
                        es_sound(c, this, 4, back)?;
                        return Ok(Step::Handled(0));
                    };
                    crate::custom_tracks::activate_in_game(c, track.id);
                    crate::custom_tracks::prepare_track_info(c, back)?;
                    let peak = r(c, GP).wrapping_sub(GP_PEAK);
                    c.bus.write32(peak, 0);
                    // The retail event start reads the row's "unlocked"
                    // word at self + 4*row + 0x20*tab + 0x60*peak + 0x20.
                    let flag = this
                        .wrapping_add(row.wrapping_mul(4))
                        .wrapping_add(tab.wrapping_mul(0x20))
                        .wrapping_add(0x20);
                    let old = c.bus.read32(flag);
                    c.bus.write32(flag, 1);
                    Ok(Step::EventStart { flag, old })
                }
                _ => Ok(Step::Original),
            }
        }
        _ => Ok(Step::Original),
    }
}

fn event_select_on_widget_action(c: &mut Cpu) {
    if c.pc != ON_WIDGET_ACTION {
        PREV_ON_WIDGET_ACTION.call(c);
        return;
    }
    let this = r(c, A0);
    let w = r(c, A1);
    let action = r(c, A2);
    let back = r(c, RA);
    let registers = c.g;
    let floats = c.f;
    let mut step = Step::Original;
    if w != 0 {
        if crate::custom_tracks::assets_available() {
            let name = c.bus.read32(w.wrapping_add(0x38));
            if let Some(id) = crate::custom_tracks::widget_row_id(name) {
                c.bus.write32(w.wrapping_add(0x18), id);
            }
        }
        if std::env::var_os("SSX3_CUSTOM_MENU_LOG").is_some() && action != 2 {
            let mode = gp_word(c, GP_MODE);
            let id = c.bus.read32(w.wrapping_add(0x18));
            let name = c.bus.read32(w.wrapping_add(0x38));
            eprintln!(
                "[tricky-menu] mode={mode} action={action} widget={w:#x} name={name:#x} id={id} preview={:?}",
                crate::custom_tracks::menu_preview()
            );
        }
        step = match custom_action(c, this, w, action, back) {
            Ok(step) => step,
            Err(Unwound) => return,
        };
    }
    match step {
        Step::Handled(value) => {
            c.g[V0][0] = sx(value);
            c.g[RA][0] = sx(back);
            c.pc = back;
        }
        Step::Original => {
            c.g = registers;
            c.f = floats;
            c.pc = ON_WIDGET_ACTION;
            PREV_ON_WIDGET_ACTION.call(c);
        }
        Step::EventStart { flag, old } => {
            c.g = registers;
            c.f = floats;
            c.pc = ON_WIDGET_ACTION;
            PREV_ON_WIDGET_ACTION.call(c);
            if c.pc == back {
                c.bus.write32(flag, old);
            }
        }
    }
}

// ------------------------------------------- EventSelect_UpdateModePanels

/// OnWidgetAction's `es_refresh_mode` and `es_show_mode` return addresses.
const REFRESH_MODE_RETURN: u32 = 0x0020_21fc;
const SHOW_MODE_RETURN: u32 = 0x0020_23c4;

fn event_select_update_mode_panels(c: &mut Cpu) {
    let back = r(c, RA);
    if c.pc != UPDATE_MODE_PANELS || !matches!(back, REFRESH_MODE_RETURN | SHOW_MODE_RETURN) {
        PREV_UPDATE_MODE_PANELS.call(c);
        return;
    }
    let this = r(c, A0);
    PREV_UPDATE_MODE_PANELS.call(c);
    if c.pc != back {
        return;
    }
    let mode = gp_word(c, GP_MODE);
    guest(c, |c| crate::bonus_menu_mode::refresh_catalog_panels(c, this, mode, back));
}

// ---------------------------------------------- EventSelect_GetEventRecord

/// The `GetEventRecord` call of OnWidgetAction's `es_start_event`.
const START_EVENT_RETURN: u32 = 0x0020_1e10;

/// While a custom course is active, the event start resolves a bounded
/// native template record instead of indexing the retail event array.
fn event_select_get_event_record(c: &mut Cpu) {
    if r(c, RA) == START_EVENT_RETURN {
        if let Some(track) = crate::custom_tracks::active() {
            let events = ssx3_game::event_boot::events(c);
            let template = ssx3_game::event_boot::find(&events, track.template_event)
                .expect("native template event is absent");
            let record = r(c, A0);
            c.bus.write32(record, template.track);
            c.bus.write32(record.wrapping_add(4), template.kind);
            c.bus.write32(record.wrapping_add(8), template.event);
            ssx3_game::abi::ret(c);
            return;
        }
    }
    PREV_GET_EVENT_RECORD.call(c);
}

// ------------------------------------------------------ finish traces

fn game_state_finish_enter(c: &mut Cpu) {
    let this = r(c, A0);
    let back = r(c, RA);
    let at_entry = c.pc == 0x0023_3c50;
    if at_entry {
        crate::custom_finish_trace::observe(c, "finish-enter", this, 0);
    }
    PREV_FINISH_ENTER.call(c);
    if at_entry && c.pc == back {
        crate::custom_finish_trace::observe(c, "finish-delay-armed", this, 0);
    }
}

fn game_ui_open_finish_screen(c: &mut Cpu) {
    let back = r(c, RA);
    let at_entry = c.pc == 0x0020_cc60;
    if at_entry {
        crate::custom_finish_trace::observe(c, "finish-screen-enter", 0, 0);
    }
    PREV_OPEN_FINISH_SCREEN.call(c);
    if at_entry && c.pc == back {
        crate::custom_finish_trace::observe(c, "finish-screen-returned", 0, 0);
    }
}

// ----------------------------------------------------------- SectionMan

const SECTION_MAN_INIT: u32 = 0x0022_cd40;
const SECTION_MAN_INIT_END: u32 = 0x0022_cea8;

/// After the retail init has built the logical section map, lease the
/// template's slot to the custom world section. Re-entered in the middle
/// (after a yield), the manager is in S2.
fn section_man_init(c: &mut Cpu) {
    let man = if c.pc == SECTION_MAN_INIT { r(c, A0) } else { r(c, S2) };
    PREV_SECTION_MAN_INIT.call(c);
    if !(SECTION_MAN_INIT..SECTION_MAN_INIT_END).contains(&c.pc) {
        crate::custom_tracks::remap_section(c, man);
    }
}

fn section_man_get_event_world_section(c: &mut Cpu) {
    PREV_SECTION_EVENT_WORLD.call(c);
    c.g[V0][0] = sx(crate::custom_tracks::event_world_section(r(c, V0)));
}

// ------------------------------------------------ GameStateResults_Update

const RESULTS_UPDATE: u32 = 0x0023_6e60;
const OPEN_SCREEN_FOR_STATE: u32 = 0x0020_a8f8;
const QUIT_TO_LOAD_SCREEN: u32 = 0x0023_0488;
const RESULTS_UPDATE_CALL_RETURN: u32 = 0x0023_6e90;

fn game_state_results_update(c: &mut Cpu) {
    if c.pc != RESULTS_UPDATE {
        PREV_RESULTS_UPDATE.call(c);
        return;
    }
    let results = r(c, A0);
    if crate::custom_results_enter::retry_pending_screen(c, results) {
        // Retry the results screen request that was rejected earlier.
        guest(c, |c| {
            c.cycles += 6;
            let frame = Frame::open(c, 0x10);
            c.g[V0][0] = 1;
            frame.save_ra(c, 0);
            c.g[A0][0] = 7;
            crate::custom_finish_trace::observe(c, "results-screen-retry", 0, results);
            call_ptr(c, OPEN_SCREEN_FOR_STATE, RESULTS_UPDATE_CALL_RETURN)?;
            crate::custom_finish_trace::observe(c, "results-screen-retry-returned", 0, results);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            Ok(())
        });
        return;
    }
    if crate::custom_tracks::active().is_some() && c.bus.read32(results.wrapping_add(8)) == 1 {
        // Additional courses have no Conquer-the-Mountain transport
        // destination: leave through the native front-end load path and keep
        // the custom context until the world has shut down.
        guest(c, |c| {
            c.cycles += 13;
            let frame = Frame::open(c, 0x10);
            frame.save_ra(c, 0);
            let world = gp_word(c, 0x848);
            let module = c.bus.read32(world.wrapping_add(0x84));
            c.g[A0][0] = sx(module);
            c.bus.write32(results.wrapping_add(8), 0);
            c.g[A1][0] = 0;
            call_ptr(c, QUIT_TO_LOAD_SCREEN, RESULTS_UPDATE_CALL_RETURN)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            Ok(())
        });
        return;
    }
    PREV_RESULTS_UPDATE.call(c);
}

// ------------------------------------------------ cPreFELoadScreen_Update

/// Once the app has no next module the front end is about to be built:
/// restore the section and residency mappings of the custom course first.
fn c_pre_fe_load_screen_update(c: &mut Cpu) {
    if c.pc == 0x0023_2a08 && crate::custom_tracks::active().is_some() {
        let app = gp_word(c, 0x848);
        if app != 0 && c.bus.read32(app.wrapping_add(4)) == 0 {
            crate::custom_tracks::deactivate(c);
        }
    }
    PREV_PRE_FE_UPDATE.call(c);
}

// ----------------------------------------------- TexMan_FreeNBTexInfo

/// TexMan_FreeTexture still dispatches by the retail boundary: the extended
/// world slots (1500 up to the imported count) have bound TexInfos and must
/// not join the dynamic free list.
fn tex_man_free_nb_tex_info(c: &mut Cpu) {
    if c.pc == 0x0036_71c8 {
        let id = r(c, A1);
        if (1500..crate::custom_tracks::world_texture_slots()).contains(&id) {
            crate::thunk::<0x0036_72c0>(c);
            return;
        }
    }
    PREV_FREE_NB_TEX_INFO.call(c);
}

// ------------------------------------------- cUITextScroll_SetTextId

fn c_ui_text_scroll_set_text_id(c: &mut Cpu) {
    if c.pc == 0x003a_1f18 {
        c.g[A1][0] = sx(crate::custom_tracks::menu_text_id(r(c, A1)));
    }
    PREV_TEXT_SCROLL_SET_TEXT_ID.call(c);
}

// ------------------------------------------------------- WorldBlockPool

const ALLOC_BLOCK: u32 = 0x003a_7768;
const FREE_BLOCK: u32 = 0x003a_7790;
const POOL_INIT: u32 = 0x003a_7598;
const HEAP_ALLOC: u32 = 0x0031_7e30;
const HEAP_FREE: u32 = 0x0031_7e98;

// Converted Tricky worlds keep their full-resolution shared textures in the
// named heap. Reserving all 53 geometry payloads at startup strands
// megabytes in the pool (Alaska uses only 27), starving its final lightmaps.
// Keep the native block count/packing; allocate only claimed payloads and
// return them when their section is released, including across course swaps.
fn lazy_geometry_pool(c: &mut Cpu, pool: u32) -> bool {
    crate::custom_tracks::assets_available() && c.bus.read32(pool) == 0x20000
}

/// A heap call can yield. Returning to the entry with its own address in RA
/// resumes the saved frame rather than claiming a second block.
fn world_block_pool_alloc_block(c: &mut Cpu) {
    if r(c, RA) == ALLOC_BLOCK {
        finish_lazy_block_alloc(c);
        return;
    }
    let pool = ssx3_game::abi::arg(c, 0);
    let block = c.bus.read32(pool.wrapping_add(0x10));
    if lazy_geometry_pool(c, pool) && c.bus.read32(block) == 0 {
        // SectionAllocator keeps its request in S1 at both block-claim
        // sites. Retail assumes every single resource fits the pool size;
        // an imported WorldScript can exceed it without being splittable.
        // Give that descriptor a large enough backing allocation.
        let bytes = if matches!(r(c, RA), 0x003a_78a8 | 0x003a_79d0) {
            // Its first-block fit check is strict (<), so keep at least one
            // byte spare, rounded to the native sixteen-byte alignment.
            c.bus.read32(pool).max(r(c, S1).saturating_add(16) & !15)
        } else {
            c.bus.read32(pool)
        };
        guest(c, |c| {
            save_lazy_block_frame(c);
            c.g[S0][0] = c.g[A0][0];
            c.g[S1][0] = c.lw(r(c, S0).wrapping_add(0x10));
            c.g[S2][0] = sx(bytes);
            c.g[A0][0] = sx(bytes);
            c.g[A1][0] = 0x0049_4dd8;
            c.g[A2][0] = 0x2500_0000;
            c.g[A3][0] = 0;
            call_ptr(c, HEAP_ALLOC, ALLOC_BLOCK)?;
            finish_lazy_block_alloc(c);
            Ok(())
        });
        return;
    }
    PREV_ALLOC_BLOCK.call(c);
}

fn finish_lazy_block_alloc(c: &mut Cpu) {
    c.sw(r(c, S1), c.g[V0][0]);
    c.sw(r(c, S1).wrapping_add(4), c.g[S2][0]);
    restore_lazy_block_frame(c);
    PREV_ALLOC_BLOCK.call(c);
}

fn world_block_pool_free_block(c: &mut Cpu) {
    if r(c, RA) == FREE_BLOCK {
        finish_lazy_block_free(c);
        return;
    }
    let pool = ssx3_game::abi::arg(c, 0);
    let block = ssx3_game::abi::arg(c, 1);
    if lazy_geometry_pool(c, pool) && c.bus.read32(block) != 0 {
        guest(c, |c| {
            save_lazy_block_frame(c);
            c.g[S1][0] = c.g[A1][0];
            c.g[A0][0] = c.lw(r(c, S1));
            call_ptr(c, HEAP_FREE, FREE_BLOCK)?;
            finish_lazy_block_free(c);
            Ok(())
        });
        return;
    }
    PREV_FREE_BLOCK.call(c);
}

fn finish_lazy_block_free(c: &mut Cpu) {
    c.sw(r(c, S1), 0);
    restore_lazy_block_frame(c);
    // A later ordinary allocation must not inherit a released oversized
    // block's capacity while receiving only the usual 128 KiB payload.
    let bytes = c.bus.read32(ssx3_game::abi::arg(c, 0));
    c.sw(ssx3_game::abi::arg(c, 1).wrapping_add(4), bytes as u64);
    PREV_FREE_BLOCK.call(c);
}

// The added heap calls must leave the leaf allocator's callers' scratch
// registers intact as well as the callee-saved registers. Keep this frame in
// game memory so a thread switch can resume it at either leaf entry.
fn save_lazy_block_frame(c: &mut Cpu) {
    let frame = Frame::open(c, 0x240);
    for register in 1..32 {
        if register != SP {
            frame.save(c, register, register as u32 * 16);
        }
    }
    for unit in 0..2 {
        c.sd(sp(c).wrapping_add(0x200 + unit as u32 * 16), c.lo[unit]);
        c.sd(sp(c).wrapping_add(0x208 + unit as u32 * 16), c.hi[unit]);
    }
}

fn restore_lazy_block_frame(c: &mut Cpu) {
    let frame = Frame { size: 0x240 };
    for unit in 0..2 {
        c.lo[unit] = c.ld(sp(c).wrapping_add(0x200 + unit as u32 * 16));
        c.hi[unit] = c.ld(sp(c).wrapping_add(0x208 + unit as u32 * 16));
    }
    for register in 1..32 {
        if register != SP {
            frame.restore(c, register, register as u32 * 16);
        }
    }
    frame.close(c);
}

/// Retail pools run the previous function. A lazy pool (block size 0x20000,
/// custom assets present) is initialised by this copy of the body, which
/// allocates no per-block payload (`+0` stays 0 until the block is claimed).
fn world_block_pool_init(c: &mut Cpu) {
    if c.pc == POOL_INIT && crate::custom_tracks::assets_available() && r(c, A2) == 0x20000 {
        lazy_pool_init(c);
        return;
    }
    PREV_POOL_INIT.call(c);
}

/// `WorldBlockPool_Init` (0x003a7598) as upstream's body, with the per-block
/// payload allocation of the loop skipped (`v0 = 0` instead of the heap call).
fn lazy_pool_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.g[A1][0]; // count
        frame.save(c, S1, 0x50);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0]; // this
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x18; // block size
        frame.save(c, S2, 0x40);
        c.mult(0, 0, 2, 4); // blocks * 0x18
        frame.save(c, S4, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4dc0));
        frame.save(c, S5, 0x10);
        c.g[A3][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0u64;
        c.sw(c.g[S1][0] as u32, c.g[A2][0]);
        c.g[A0][0] = c.lo[0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A2][0] = 0x2000_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, HEAP_ALLOC, 0x003a_75f0)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = c.g[V1][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if !empty {
            c.cycles += 3;
            c.g[S5][0] = 0x49_0000;
            c.g[S4][0] = 0x18;
            c.g[S2][0] = 0u64;
            loop {
                c.cycles += 35;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4dd8));
                c.g[A2][0] = 0x2500_0000;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                // Lazy pool: no payload until the block is claimed.
                c.g[V0][0] = 0;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_7608);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        frame.restore(c, S0, 0x60);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
