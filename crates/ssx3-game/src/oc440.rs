//! Staging module oc440 (wave wave223): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc440`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_2ad8,
    event_select_update_mode_panels,
    "EventSelect_UpdateModePanels",
)];

use crate::call::Flow;

/// `cUIState_hideObjSafe(state, name)`.
const ES_HIDE_OBJECT: u32 = 0x0039_e8b8;
/// `cUIState_showObjSafe(state, name)`.
const ES_SHOW_OBJECT: u32 = 0x0039_e948;
/// `GetHashValue32(str)`.
const ES_STR_HASH: u32 = 0x0031_7670;
/// `UIScreen_FindObject(screen, hash)`.
const ES_FIND_OBJECT: u32 = 0x0039_d860;
/// `UIText_SetTextId(text, id)`.
const ES_SET_TEXT_ID: u32 = 0x003a_1000;
/// `sprintf(buf, fmt, ...)`.
const ES_SPRINTF: u32 = 0x0041_7828;
/// `EventSelect_RefreshEventList(self)` (location info of the event list).
const ES_REFRESH_EVENT_LIST: u32 = 0x0020_7430;

/// Panel owner fields (the event-select screen object, `s1`).
pub(crate) const ES_HAS_FOCUS: u32 = 0x0;
pub(crate) const ES_LOCKS_FLAG: u32 = 0x2cc;
pub(crate) const ES_SCREEN: u32 = 0x2e8;
pub(crate) const ES_UI_STATE: u32 = 0x2ec;
/// Selected-list state byte of a list-menu object.
pub(crate) const ES_LIST_STATE: u32 = 0x95;
/// gp-relative map globals: state (0 peaks, 1 peak goals, 2 events), selected
/// event kind, peak number, and map style.
pub(crate) const ES_GP_MAP_STATE: u32 = 0xb54;
pub(crate) const ES_GP_SELECTED_EVENT: u32 = 0xb50;
pub(crate) const ES_GP_PEAK: u32 = 0xb4c;
pub(crate) const ES_GP_MAP_STYLE: u32 = 0xb5c;

/// An address in the string table the code builds with `lui 0x47`.
#[inline]
fn es_str47(off: u32) -> u64 {
    sx(0x0047_0000u32.wrapping_add(off))
}

/// An address in the string table the code builds with `lui 0x4a`.
#[inline]
fn es_str4a(off: u32) -> u64 {
    sx(0x004a_0000u32.wrapping_add(off))
}

/// `a0 = self->uiState` (the screen's `cUIState`, read again before each call).
#[inline]
fn es_load_ui_state(c: &mut Cpu) {
    let s1 = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(s1.wrapping_add(ES_UI_STATE));
}

/// `showObjSafe(a0, a1)` returning to `ret`.
#[inline]
fn es_show(c: &mut Cpu, a1: u64, ret: u32) -> Flow {
    c.g[A1][0] = a1;
    call_ptr(c, ES_SHOW_OBJECT, ret)
}

/// `hideObjSafe(a0, a1)` returning to `ret`.
#[inline]
fn es_hide(c: &mut Cpu, a1: u64, ret: u32) -> Flow {
    c.g[A1][0] = a1;
    call_ptr(c, ES_HIDE_OBJECT, ret)
}

/// `a1 = GetHashValue32(a0)`; `UIText_SetTextId(title (s0), a1)`: set the
/// sub-title text from the string id named by `a0`.
#[inline]
fn es_set_title(c: &mut Cpu, hash_ret: u32, text_ret: u32) -> Flow {
    call_ptr(c, ES_STR_HASH, hash_ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, ES_SET_TEXT_ID, text_ret)
}

/// `v0 = UIScreen_FindObject(self->screen, GetHashValue32(a0))`.
#[inline]
fn es_find_object(c: &mut Cpu, hash_ret: u32, find_ret: u32) -> Flow {
    call_ptr(c, ES_STR_HASH, hash_ret)?;
    let s1 = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(s1.wrapping_add(ES_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_FIND_OBJECT, find_ret)
}

/// The six pda map-tab objects, hidden (`hide = true`) or shown. The hide
/// form is entered with `a0` already loaded; the show form loads it first.
fn es_map_tab_objects(c: &mut Cpu, hide: bool) -> Flow {
    let names: [u64; 6] = [
        es_str4a(0x25b0),
        es_str47(0x808),
        es_str47(0x818),
        es_str4a(0x2618),
        es_str4a(0x2620),
        es_str4a(0x2628),
    ];
    let rets: [u32; 6] = if hide {
        [
            0x0020_2e14,
            0x0020_2e24,
            0x0020_2e34,
            0x0020_2e44,
            0x0020_2e54,
            0x0020_2e64,
        ]
    } else {
        [
            0x0020_2e88,
            0x0020_2e98,
            0x0020_2ea8,
            0x0020_2eb8,
            0x0020_2ec8,
            0x0020_2ed8,
        ]
    };
    for i in 0..6 {
        if i > 0 {
            es_load_ui_state(c);
        }
        if hide {
            es_hide(c, names[i], rets[i])?;
        } else {
            es_show(c, names[i], rets[i])?;
        }
    }
    Ok(())
}

/// Hide the map tab objects (entered with `a0` loaded), restore `s0`.
fn es_hide_section(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 25;
    es_map_tab_objects(c, true)?;
    frame.restore(c, S0, 0xd0);
    Ok(())
}

/// Hide section entered after one cycle and a reload of `a0`.
fn es_hide_section_reload(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    es_load_ui_state(c);
    es_hide_section(c, frame)
}

/// Show the map tab objects when `v0` (map style `< 2`) is 0, then restore
/// `s0`: the tail shared by the peak and goal screens.
fn es_show_section(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let skip = c.g[V0][0] != 0;
    frame.restore(c, S0, 0xd0);
    if !skip {
        c.cycles += 26;
        es_load_ui_state(c);
        es_map_tab_objects(c, false)?;
        frame.restore(c, S0, 0xd0);
    }
    Ok(())
}

/// `v0 = map style` on entry: `v0 = (style < 2)`, then the show section.
fn es_style_section(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    es_show_section(c, frame)
}

/// Map state 0: "Select Peak".
fn es_state_peaks(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 25;
    c.g[A0][0] = es_str47(0x7c0);
    es_set_title(c, 0x0020_2b5c, 0x0020_2b68)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7d8), 0x0020_2b78)?;
    c.g[V0][0] = 0x4a0000;
    es_load_ui_state(c);
    c.g[S2][0] = es_str4a(0x2608);
    let a1 = c.g[S2][0];
    es_hide(c, a1, 0x0020_2b8c)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7e8), 0x0020_2b9c)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7f8), 0x0020_2bac)?;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_LOCKS_FLAG));
    let locked = c.g[V0][0] != 0;
    es_load_ui_state(c);
    if !locked {
        c.cycles += 4;
        es_show(c, es_str4a(0x2610), 0x0020_2bc4)?;
        es_load_ui_state(c);
    }
    c.cycles += 15;
    c.g[S0][0] = 0;
    es_hide(c, es_str47(0x7a0), 0x0020_2bd8)?;
    es_load_ui_state(c);
    let a1 = c.g[S2][0];
    es_hide(c, a1, 0x0020_2be4)?;
    c.g[A0][0] = es_str47(0x600);
    call_ptr(c, ES_STR_HASH, 0x0020_2bf0)?;
    let s1 = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(s1.wrapping_add(ES_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_FIND_OBJECT, 0x0020_2bfc)?;
    if c.g[V0][0] != 0 {
        let list = c.g[V0][0] as u32;
        c.g[S0][0] = c.lbu(list.wrapping_add(ES_LIST_STATE));
    }
    c.cycles += 3;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_HAS_FOCUS));
    let no_focus = c.g[V0][0] == 0;
    c.g[V0][0] = 3;
    if !no_focus {
        c.cycles += 2;
        let is_earnings = c.g[S0][0] == c.g[V0][0];
        es_load_ui_state(c);
        if is_earnings {
            return es_hide_section(c, frame);
        }
        c.cycles += 2;
        let a1 = c.g[S2][0];
        es_show(c, a1, 0x0020_2c20)?;
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    let is_earnings = c.g[S0][0] == c.g[V0][0];
    let s_gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(s_gp.wrapping_sub(ES_GP_MAP_STYLE));
    if is_earnings {
        return es_hide_section_reload(c, frame);
    }
    c.cycles += 2;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    es_show_section(c, frame)
}

/// Map state 1: "Select Peak Goal".
fn es_state_goals(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 81;
    c.g[S2][0] = 0;
    c.g[A0][0] = es_str47(0x828);
    es_set_title(c, 0x0020_2c44, 0x0020_2c50)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7d8), 0x0020_2c60)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7e8), 0x0020_2c70)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7f8), 0x0020_2c80)?;
    es_load_ui_state(c);
    es_hide(c, es_str4a(0x2610), 0x0020_2c90)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7a0), 0x0020_2ca0)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x840), 0x0020_2cb0)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x850), 0x0020_2cc0)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x860), 0x0020_2cd0)?;
    // sprintf(name, "peak%d indicators", mPeak)
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[A1][0] = es_str47(0x870);
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, ES_SPRINTF, 0x0020_2ce4)?;
    es_load_ui_state(c);
    let name = c.g[SP][0];
    es_show(c, name, 0x0020_2cf0)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S6][0] = es_str47(0x888);
    let a1 = c.g[S6][0];
    es_hide(c, a1, 0x0020_2d04)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S5][0] = es_str47(0x8a0);
    let a1 = c.g[S5][0];
    es_hide(c, a1, 0x0020_2d18)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S4][0] = es_str47(0x8b8);
    let a1 = c.g[S4][0];
    es_hide(c, a1, 0x0020_2d2c)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S3][0] = es_str47(0x8d0);
    let a1 = c.g[S3][0];
    es_hide(c, a1, 0x0020_2d40)?;
    // sprintf(next, "SPG_Peak%d_Goals", mPeak + 1); find its list menu
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A1][0] = es_str47(0x6d8);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    call_ptr(c, ES_SPRINTF, 0x0020_2d5c)?;
    c.g[A0][0] = c.g[S0][0];
    es_find_object(c, 0x0020_2d64, 0x0020_2d70)?;
    if c.g[V0][0] != 0 {
        let list = c.g[V0][0] as u32;
        c.g[S2][0] = c.lbu(list.wrapping_add(ES_LIST_STATE));
    }
    // Info back panel and the per-state goal panel.
    c.cycles += 3;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_HAS_FOCUS));
    if c.g[V0][0] != 0 {
        es_load_ui_state(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        let not_earnings = c.g[S2][0] != c.g[V0][0];
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(gp.wrapping_sub(ES_GP_MAP_STYLE));
        if not_earnings {
            return es_style_section(c, frame);
        }
        c.cycles += 1;
        es_load_ui_state(c);
    }
    c.cycles += 5;
    let a1 = c.g[S3][0];
    es_show(c, a1, 0x0020_2d9c)?;
    c.g[V0][0] = 2;
    let is_freeride = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
    if is_freeride {
        c.cycles += 5;
        es_load_ui_state(c);
        let a1 = c.g[S5][0];
        es_show(c, a1, 0x0020_2de8)?;
        c.g[V0][0] = 3;
    } else {
        c.cycles += 2;
        let below_3 = c.g[V0][0] != 0;
        c.g[V0][0] = 3;
        if below_3 {
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.cycles += 5;
                es_load_ui_state(c);
                let a1 = c.g[S6][0];
                es_show(c, a1, 0x0020_2dd4)?;
                c.g[V0][0] = 3;
            }
        } else {
            c.cycles += 2;
            if c.g[S2][0] == c.g[V0][0] {
                es_load_ui_state(c);
                c.cycles += 3;
                let a1 = c.g[S4][0];
                es_show(c, a1, 0x0020_2df8)?;
                c.g[V0][0] = 3;
            } else {
                c.cycles += 2;
            }
        }
    }
    c.cycles += 2;
    let not_earnings = c.g[S2][0] != c.g[V0][0];
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_sub(ES_GP_MAP_STYLE));
    if not_earnings {
        es_style_section(c, frame)
    } else {
        es_hide_section_reload(c, frame)
    }
}

/// Map state 2: "Select Event". Enters with `v0` = selected event kind and
/// `a0` = 1 (left by the dispatch).
fn es_state_events(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let not_race = c.g[V0][0] != 0;
    // Title string for race (0), free-style (1) or free-ride (other) events.
    if !not_race {
        c.cycles += 6;
        es_load_ui_state(c);
        es_hide(c, es_str4a(0x2610), 0x0020_2ef4)?;
        c.cycles += 7;
        c.g[A0][0] = es_str47(0x8e0);
        es_set_title(c, 0x0020_2f24, 0x0020_2f30)?;
    } else {
        c.cycles += 2;
        let not_fs = c.g[V0][0] != c.g[A0][0];
        es_load_ui_state(c);
        if !not_fs {
            c.cycles += 5;
            es_hide(c, es_str4a(0x2610), 0x0020_2f14)?;
            c.g[A0][0] = es_str47(0x8f8);
            c.cycles += 7;
            es_set_title(c, 0x0020_2f24, 0x0020_2f30)?;
        } else {
            c.cycles += 10;
            es_hide(c, es_str4a(0x2610), 0x0020_2f44)?;
            c.g[A0][0] = es_str47(0x908);
            es_set_title(c, 0x0020_2f50, 0x0020_2f5c)?;
        }
    }
    es_load_ui_state(c);
    c.cycles += 47;
    es_hide(c, es_str47(0x7d8), 0x0020_2f6c)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7e8), 0x0020_2f7c)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7f8), 0x0020_2f8c)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7a0), 0x0020_2f9c)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S5][0] = es_str47(0x840);
    let a1 = c.g[S5][0];
    es_hide(c, a1, 0x0020_2fb0)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S4][0] = es_str47(0x850);
    let a1 = c.g[S4][0];
    es_hide(c, a1, 0x0020_2fc4)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S3][0] = es_str47(0x860);
    let a1 = c.g[S3][0];
    es_hide(c, a1, 0x0020_2fd8)?;
    c.g[V0][0] = 0x470000;
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[S2][0] = es_str47(0x870);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, ES_SPRINTF, 0x0020_2ff0)?;
    es_load_ui_state(c);
    let name = c.g[SP][0];
    es_show(c, name, 0x0020_2ffc)?;
    c.g[V0][0] = 0x4a0000;
    es_load_ui_state(c);
    c.g[S0][0] = es_str4a(0x2630);
    let a1 = c.g[S0][0];
    es_hide(c, a1, 0x0020_3010)?;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_HAS_FOCUS));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        es_load_ui_state(c);
        let a1 = c.g[S0][0];
        es_show(c, a1, 0x0020_3028)?;
    }
    c.cycles += 20;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, ES_REFRESH_EVENT_LIST, 0x0020_3030)?;
    es_load_ui_state(c);
    let a1 = c.g[S5][0];
    es_hide(c, a1, 0x0020_303c)?;
    es_load_ui_state(c);
    let a1 = c.g[S4][0];
    es_hide(c, a1, 0x0020_3048)?;
    es_load_ui_state(c);
    let a1 = c.g[S3][0];
    es_hide(c, a1, 0x0020_3054)?;
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, ES_SPRINTF, 0x0020_3068)?;
    es_load_ui_state(c);
    let a1 = c.g[S0][0];
    es_show(c, a1, 0x0020_3074)?;
    frame.restore(c, S0, 0xd0);
    Ok(())
}

/// `void EventSelect_UpdateModePanels(EventSelect* self)` (0x202ad8): show and
/// hide the panels of the map screen by the map state (`gp-0xb54`): 0 select
/// a peak, 1 select a peak goal, 2 select an event. Sets the sub-title text,
/// toggles the objects of the screen's UI state by name, builds the
/// "peakN indicators" names with sprintf, and, for the event list, refreshes
/// the event list.
pub fn event_select_update_mode_panels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x60);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x620));
        // title = FindObject(self->screen, GetHashValue32("SubTitle"))
        es_find_object(c, 0x0020_2b0c, 0x0020_2b18)?;
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = c.lw(gp.wrapping_sub(ES_GP_MAP_STATE));
        c.g[A0][0] = 1;
        c.g[S0][0] = c.g[V0][0];
        let state = c.g[V1][0];
        if state == 1 {
            es_state_goals(c, frame)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((state as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let is_events = state == c.g[V0][0];
                c.g[V0][0] = c.lw(gp.wrapping_sub(ES_GP_SELECTED_EVENT));
                if is_events {
                    es_state_events(c, frame)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xd0);
                }
            } else {
                c.cycles += 2;
                let is_peaks = state == 0;
                c.g[A0][0] = 0x470000;
                if is_peaks {
                    es_state_peaks(c, frame)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xd0);
                }
            }
        }
        // Epilogue.
        c.cycles += 9;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// The owner screen object, its UI state (empty object list: the real
    /// show/hide callees return at once), the widget screen with one list-menu
    /// object, and that object's vtable.
    const OC440_SELF: u32 = DATA + 0x400;
    const OC440_UI_STATE: u32 = DATA + 0x2000;
    const OC440_SCREEN: u32 = DATA + 0x3000;
    const OC440_COUNT: u32 = DATA + 0x3100;
    const OC440_OBJECTS: u32 = DATA + 0x3200;
    const OC440_OBJ: u32 = DATA + 0x4000;
    const OC440_VTABLE: u32 = DATA + 0x4200;

    /// `GetHashValue32(addr)` as the game computes it (every string of the
    /// test RAM is empty, so all names hash alike and the one object matches
    /// every lookup).
    fn oc440_hash(c: &mut Cpu, s: u32) -> u32 {
        const BACK: u32 = 0x0012_3450;
        c.g[RA][0] = BACK as u64;
        c.g[A0][0] = sx(s);
        c.pc = ES_STR_HASH;
        ssx3_ee_rt::funcs::call_indirect(c, ES_STR_HASH);
        assert_eq!(c.pc, BACK, "GetHashValue32 did not return");
        c.g[V0][0] as u32
    }

    fn oc440_fixture(c: &mut Cpu, seed: u32, state: u32) {
        let mut r: Rng = machine(c, seed, 0x20_2ad8, 0);
        let gp = GAME_GP;
        c.bus.write32(gp - ES_GP_MAP_STATE, state);
        c.bus
            .write32(gp - ES_GP_SELECTED_EVENT, [0, 1, 2, 5][seed as usize % 4]);
        c.bus.write32(gp - ES_GP_PEAK, r.next_u32() % 4);
        c.bus.write32(gp - ES_GP_MAP_STYLE, r.next_u32() % 4);
        c.bus
            .write32(OC440_SELF + ES_HAS_FOCUS, (seed % 3 != 0) as u32);
        c.bus
            .write32(OC440_SELF + ES_LOCKS_FLAG, (seed % 5 < 2) as u32);
        c.bus.write32(OC440_SELF + ES_SCREEN, OC440_SCREEN);
        c.bus.write32(OC440_SELF + ES_UI_STATE, OC440_UI_STATE);
        // A list-menu object found by every name (its text-id slot at +0xb0
        // and a vtable whose virtual calls land in the fake).
        let h = oc440_hash(c, DATA + 0x100);
        c.bus.write32(OC440_OBJ + 0x38, h);
        c.bus
            .write8(OC440_OBJ + ES_LIST_STATE, (r.next_u32() % 5) as u8);
        c.bus.write32(OC440_OBJ + 8, OC440_VTABLE);
        for off in [0x28u32, 0x48] {
            c.bus.write16(OC440_VTABLE + off, 0);
            c.bus.write32(OC440_VTABLE + off + 4, FAKE);
        }
        c.bus.write32(OC440_COUNT, 1);
        c.bus.write32(OC440_OBJECTS, OC440_OBJ);
        c.bus.write32(OC440_SCREEN + 0x38, OC440_COUNT);
        // Every seventh seed has no objects: all lookups return null.
        c.bus.write32(
            OC440_SCREEN + 0x3c,
            if seed % 7 == 0 { 0 } else { OC440_OBJECTS },
        );
        c.g[A0][0] = sx(OC440_SELF);
        c.pc = 0x0020_2ad8;
    }

    /// Map states 0 (peaks) and 1 (peak goals), plus the states that only
    /// restore and return (negative, 3): focus, lock flag, list state 0..4,
    /// map style, peak number all vary.
    #[test]
    fn event_select_update_mode_panels_matches() {
        check(
            ee::s05::f_00202ad8__generated,
            event_select_update_mode_panels,
            210,
            &|c, seed| {
                let state = [0u32, 1, 0, 1, 0xffff_ffff, 3, 1][seed as usize % 7];
                oc440_fixture(c, seed, state);
            },
        );
    }

    /// Map state 2 (events): runs the real `EventSelect_RefreshEventList` on
    /// the fixture, so it needs the back-end library singleton.
    #[test]
    fn event_select_update_mode_panels_events_matches() {
        check(
            ee::s05::f_00202ad8__generated,
            event_select_update_mode_panels,
            24,
            &|c, seed| {
                oc440_fixture(c, seed, 2);
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x20_0000);
            },
        );
    }
}
