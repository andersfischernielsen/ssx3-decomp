//! Staging module oc441 (wave wave223): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc441`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_30a0,
    event_select_set_mode,
    "EventSelect_SetMode",
)];

/// `EventSelect_SetMapTabMode`, `EventSelect_RefreshPeakList`,
/// `EventSelect_RefreshPeakGoals` (direct calls through their generated path).
const ES_SETMODE_SET_MAP_TAB_MODE: Callee = Callee {
    addr: 0x0020_3cb8,
    f: ee::s05::f_00203cb8,
};
const ES_SETMODE_REFRESH_PEAK_LIST: Callee = Callee {
    addr: 0x0020_3d40,
    f: ee::s05::f_00203d40,
};
const ES_SETMODE_REFRESH_PEAK_GOALS: Callee = Callee {
    addr: 0x0020_5098,
    f: ee::s05::f_00205098,
};

/// `StrHash`, `UIScreen_FindObject`, `UIState_HideObject` (hideObjSafe),
/// `UIText_SetTextID` (0x3a1000).
const ES_SETMODE_STR_HASH: u32 = 0x0031_7670;
const ES_SETMODE_FIND_OBJECT: u32 = 0x0039_d860;
const ES_SETMODE_HIDE_OBJ_SAFE: u32 = 0x0039_e8b8;
const ES_SETMODE_SET_TEXT_ID: u32 = 0x003a_1000;

/// Base of the screen's name strings (`str_*` at 0x470000 + offset).
const ES_SETMODE_STR_BASE: u32 = 0x0047_0000;

/// `EventSelect` fields.
const ES_SETMODE_POPUP: u32 = 0x2cc;
const ES_SETMODE_NO_SELECTED: u32 = 0x2d0;
const ES_SETMODE_SCREEN: u32 = 0x2e8;
const ES_SETMODE_STATE: u32 = 0x2ec;

/// Widget vtable slots (`{delta: i16, pad, fn: u32}` at `8 * index`): v07
/// (set visible) at 0x38, v08 at 0x40, v09 at 0x48.
const ES_SETMODE_V07: u32 = 0x38;
const ES_SETMODE_V08: u32 = 0x40;
const ES_SETMODE_V09: u32 = 0x48;

/// `gp`-relative globals: map state (panel), selected tab, peak, flow.
const ES_SETMODE_G_MAP_STATE: u32 = 0xb54;
const ES_SETMODE_G_FLOW: u32 = 0xb5c;
const ES_SETMODE_G_PEAK: u32 = 0xb4c;
const ES_SETMODE_G_EVENT: u32 = 0xb50;

/// `gp - off`.
#[inline(always)]
fn es_setmode_gp(c: &Cpu, off: u32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_sub(off))
}

/// `sp + off`.
#[inline(always)]
fn es_setmode_sp(c: &Cpu, off: u32) -> Addr {
    Addr(sp(c).wrapping_add(off))
}

/// Which panel `EventSelect_SetMode` configures.
#[derive(Clone, Copy)]
enum EsSetModePanel {
    Peaks,
    Goals,
    Events,
}

/// `obj->vtbl[slot](a1)`: `a0 = obj + vt[slot].delta`, target `vt[slot].fn`.
/// The compiled code keeps the vtable pointer in `vt_reg` and the target in
/// `fn_reg` (v0/v1 in either order).
fn es_setmode_vcall(
    c: &mut Cpu,
    obj: u32,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> Flow {
    let vt = Addr(obj).off(8).lw(c);
    c.g[vt_reg][0] = vt;
    c.g[A0][0] = Addr(vt as u32).off(slot).lh(c);
    c.g[fn_reg][0] = Addr(vt as u32).off(slot + 4).lw(c);
    let target = c.g[fn_reg][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `hash = StrHash(str_base + name); obj = UIScreen_FindObject(self->screen,
/// hash)` (returns in v0). `ra` is the first call's return address, the
/// second returns 12 bytes later.
fn es_setmode_find_widget(c: &mut Cpu, name: u32, ra: u32) -> Flow {
    c.g[A0][0] = sx(ES_SETMODE_STR_BASE + name);
    call_ptr(c, ES_SETMODE_STR_HASH, ra)?;
    let this = c.g[S2][0] as u32;
    c.g[A0][0] = Addr(this).off(ES_SETMODE_SCREEN).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_SETMODE_FIND_OBJECT, ra + 0xc)
}

/// `hideObj(self, name)` with the widget kept in the saved register `dest`:
/// find the widget; if it exists, `v07(0)` and `v09(0)`. `pre` are the block's
/// own cycles (the first widget's are in the prologue).
fn es_setmode_hide_reg(c: &mut Cpu, pre: u64, ra: u32, name: u32, dest: usize) -> Flow {
    c.cycles += pre;
    es_setmode_find_widget(c, name, ra)?;
    c.g[dest][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if c.g[dest][0] != 0 {
        c.cycles += 11;
        let obj = c.g[dest][0] as u32;
        es_setmode_vcall(c, obj, V0, V1, ES_SETMODE_V07, ra + 0x2c)?;
        let obj = c.g[dest][0] as u32;
        c.g[A1][0] = 0;
        es_setmode_vcall(c, obj, V0, V1, ES_SETMODE_V09, ra + 0x44)?;
    }
    Ok(())
}

/// As [`es_setmode_hide_reg`] with the widget spilled to `sp + slot` (the
/// compiled code reloads it from there for every use, leaving it in v0).
fn es_setmode_hide_stack(c: &mut Cpu, ra: u32, name: u32, slot: u32) -> Flow {
    c.cycles += 8;
    es_setmode_find_widget(c, name, ra)?;
    let found = c.g[V0][0];
    es_setmode_sp(c, slot).sw(c, found);
    if found != 0 {
        c.cycles += 15;
        let vt = Addr(found as u32).off(8).lw(c);
        c.g[V0][0] = vt;
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(vt as u32).off(ES_SETMODE_V07).lh(c);
        c.g[V1][0] = Addr(vt as u32).off(ES_SETMODE_V07 + 4).lw(c);
        let obj = es_setmode_sp(c, slot).lw(c);
        c.g[V0][0] = obj;
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ra + 0x30)?;
        let obj = es_setmode_sp(c, slot).lw(c);
        c.g[V1][0] = obj;
        c.g[A1][0] = 0;
        let vt = Addr(obj as u32).off(8).lw(c);
        c.g[V0][0] = vt;
        c.g[A0][0] = Addr(vt as u32).off(ES_SETMODE_V09).lh(c);
        c.g[V1][0] = Addr(vt as u32).off(ES_SETMODE_V09 + 4).lw(c);
        let obj = es_setmode_sp(c, slot).lw(c);
        c.g[V0][0] = obj;
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ra + 0x50)?;
    }
    Ok(())
}

/// Show the widget in saved register `obj_reg`: set bit 0 of its flags word
/// (`+0x14`), then call the vtable slots `first` and `second` with a1 = 1
/// (the callers' delay slot already loaded a1 = 1).
fn es_setmode_show_widget(
    c: &mut Cpu,
    obj_reg: usize,
    first: u32,
    second: u32,
    rets: [u32; 2],
) -> Flow {
    let obj = c.g[obj_reg][0] as u32;
    c.g[V0][0] = Addr(obj).off(0x14).lw(c);
    c.g[V1][0] = Addr(obj).off(8).lw(c);
    c.g[V0][0] |= 1;
    let flags = c.g[V0][0];
    Addr(obj).off(0x14).sw(c, flags);
    es_setmode_vcall(c, obj, V1, V0, first, rets[0])?;
    let obj = c.g[obj_reg][0] as u32;
    c.g[A1][0] = 1;
    es_setmode_vcall(c, obj, V1, V0, second, rets[1])
}

/// `s0 = UIScreen_FindObject(screen, StrHash("title_title"))`; the title
/// widget name is at `str_base + 0x518`.
fn es_setmode_find_title(c: &mut Cpu, ra: u32) -> Flow {
    es_setmode_find_widget(c, 0x518, ra)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// `title->setTextID(StrHash(str_base + text))` (0x3a1000).
fn es_setmode_set_title(c: &mut Cpu, text: u32, ra_hash: u32, ra_set: u32) -> Flow {
    c.g[A0][0] = sx(ES_SETMODE_STR_BASE + text);
    call_ptr(c, ES_SETMODE_STR_HASH, ra_hash)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_SETMODE_SET_TEXT_ID, ra_set)
}

/// `void EventSelect_SetMode(EventSelect* self, int focus)` (0x2030a0):
/// `cUITemplate_MAP_setupMenuFocus`. Sets the map tab (`SetMapTabMode(self,
/// 0)`), stores `focus` as the panel (`gp-0xb54`), hides the 14 panel widgets
/// (peak list, three goal lists, nine event lists, the confirm popup) and then
///
/// - with the confirm popup open (`self+0x2cc`): hides the "Locks" objects
///   through the UI state, shows the popup and highlights its "no" or "yes"
///   button by `self+0x2d0`;
/// - else by panel: 0 refreshes the peak list and shows the peak list, 1
///   refreshes the peak goals and shows the selected peak's goal list, 2 shows
///   the event list of the selected peak and tab; panels 0 to 2 also set the
///   screen title unless the flow (`gp-0xb5c`) is 2 or more.
///
/// The widgets live in saved registers (peak list, goals, last three event
/// lists, popup) and in stack slots `sp+0..0x14` (the other event lists), as
/// the compiled code spilled them; the scratch registers it leaves on each
/// path (a1, v0/v1, a0) are reproduced.
pub fn event_select_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0xc0 frame, ra at 0x20, s0..s7 and fp saved.
        c.cycles += 25;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S2, 0x90);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, FP, 0x30);
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save_ra(c, 0x20);

        // SetMapTabMode(self, 0); mMapState = focus.
        c.g[A1][0] = 0;
        ES_SETMODE_SET_MAP_TAB_MODE.call(c, 0x0020_30dc)?;
        c.g[A0][0] = 0x0047_0000;
        let focus = c.g[S0][0];
        es_setmode_gp(c, ES_SETMODE_G_MAP_STATE).sw(c, focus);

        // Hide the widgets: peak list (fp), goal lists (s7, s6, s4), event
        // lists (stack, stack, stack, stack, stack, stack, s5, s3, s1), popup
        // (s0).
        es_setmode_hide_reg(c, 0, 0x0020_30ec, 0x600, FP)?;
        es_setmode_hide_reg(c, 9, 0x0020_313c, 0x918, S7)?;
        es_setmode_hide_reg(c, 9, 0x0020_318c, 0x928, S6)?;
        es_setmode_hide_reg(c, 9, 0x0020_31dc, 0x938, S4)?;
        es_setmode_hide_stack(c, 0x0020_322c, 0x948, 0x00)?;
        es_setmode_hide_stack(c, 0x0020_3288, 0x960, 0x04)?;
        es_setmode_hide_stack(c, 0x0020_32e4, 0x978, 0x08)?;
        es_setmode_hide_stack(c, 0x0020_3340, 0x990, 0x0c)?;
        es_setmode_hide_stack(c, 0x0020_339c, 0x9a8, 0x10)?;
        es_setmode_hide_stack(c, 0x0020_33f8, 0x9c0, 0x14)?;
        es_setmode_hide_reg(c, 9, 0x0020_3454, 0x9d8, S5)?;
        es_setmode_hide_reg(c, 9, 0x0020_34a4, 0x9f0, S3)?;
        es_setmode_hide_reg(c, 9, 0x0020_34f4, 0xa08, S1)?;
        es_setmode_hide_reg(c, 9, 0x0020_3544, 0xa20, S0)?;

        'epilogue: {
            'restored_s0: {
                // The confirm popup.
                c.cycles += 3;
                let this = c.g[S2][0] as u32;
                c.g[V0][0] = Addr(this).off(ES_SETMODE_POPUP).lw(c);
                c.g[A1][0] = 0x004a_0000;
                if c.g[V0][0] != 0 {
                    c.cycles += 18;
                    let state = Addr(this).off(ES_SETMODE_STATE).lw(c);
                    c.g[A0][0] = state;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2610));
                    call_ptr(c, ES_SETMODE_HIDE_OBJ_SAFE, 0x0020_35a0)?;
                    // popup->v07(1); popup->v09(1).
                    let popup = c.g[S0][0] as u32;
                    c.g[A1][0] = 1;
                    es_setmode_vcall(c, popup, V0, V1, ES_SETMODE_V07, 0x0020_35b8)?;
                    let popup = c.g[S0][0] as u32;
                    c.g[A1][0] = 1;
                    es_setmode_vcall(c, popup, V0, V1, ES_SETMODE_V09, 0x0020_35d0)?;
                    c.g[V0][0] = Addr(this).off(ES_SETMODE_NO_SELECTED).lw(c);
                    let no_selected = c.g[V0][0] != 0;
                    c.g[A0][0] = 0x0047_0000;
                    if !no_selected {
                        // "No" button: v09(1), then v08(0).
                        c.cycles += 15;
                        es_setmode_find_widget(c, 0x6c8, 0x0020_35e4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 1;
                        let no = c.g[S0][0] as u32;
                        es_setmode_vcall(c, no, V0, V1, ES_SETMODE_V09, 0x0020_360c)?;
                        let no = c.g[S0][0] as u32;
                        c.g[V0][0] = Addr(no).off(8).lw(c);
                        c.g[A1][0] = 0;
                    } else {
                        // "No" button: v09(0), then v08(1).
                        c.cycles += 14;
                        es_setmode_find_widget(c, 0x6c8, 0x0020_3620)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        let no = c.g[S0][0] as u32;
                        es_setmode_vcall(c, no, V0, V1, ES_SETMODE_V09, 0x0020_3648)?;
                        let no = c.g[S0][0] as u32;
                        c.g[V0][0] = Addr(no).off(8).lw(c);
                        c.g[A1][0] = 1;
                    }
                    c.cycles += 6;
                    let no = c.g[S0][0] as u32;
                    es_setmode_vcall(c, no, V0, V1, ES_SETMODE_V08, 0x0020_3660)?;
                    frame.restore(c, S0, 0xb0);
                    break 'restored_s0;
                }

                // The panel switch.
                'panel_done: {
                    c.cycles += 4;
                    c.g[V1][0] = es_setmode_gp(c, ES_SETMODE_G_MAP_STATE).lw(c);
                    c.g[A0][0] = 1;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    let panel = if c.g[V1][0] == c.g[A0][0] {
                        EsSetModePanel::Goals
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            // 2 or more: only 2 is a panel.
                            c.g[V0][0] = 2;
                            c.cycles += 2;
                            let is_events = c.g[V1][0] == c.g[V0][0];
                            frame.restore(c, S0, 0xb0);
                            if !is_events {
                                c.cycles += 2;
                                frame.restore(c, S1, 0xa0);
                                break 'epilogue;
                            }
                            EsSetModePanel::Events
                        } else {
                            // Below 2: 0 is the peak panel, negative none.
                            c.cycles += 2;
                            let is_peaks = c.g[V1][0] == 0;
                            frame.restore(c, S0, 0xb0);
                            if !is_peaks {
                                c.cycles += 2;
                                frame.restore(c, S1, 0xa0);
                                break 'epilogue;
                            }
                            EsSetModePanel::Peaks
                        }
                    };

                    match panel {
                        EsSetModePanel::Peaks => {
                            // Refresh the peak list and show it.
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            ES_SETMODE_REFRESH_PEAK_LIST.call(c, 0x0020_36a8)?;
                            c.g[A1][0] = 1;
                            if c.g[FP][0] != 0 {
                                c.cycles += 14;
                                es_setmode_show_widget(
                                    c,
                                    FP,
                                    ES_SETMODE_V07,
                                    ES_SETMODE_V09,
                                    [0x0020_36d0, 0x0020_36e8],
                                )?;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            c.g[A0][0] = 0x0047_0000;
                            if c.g[V0][0] == 0 {
                                break 'panel_done;
                            }
                            c.cycles += 9;
                            es_setmode_find_title(c, 0x0020_3700)?;
                            c.cycles += 7;
                            es_setmode_set_title(c, 0x7c0, 0x0020_3844, 0x0020_3850)?;
                            frame.restore(c, S0, 0xb0);
                            break 'restored_s0;
                        }
                        EsSetModePanel::Goals => {
                            // Refresh the goals and show the selected peak's.
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            ES_SETMODE_REFRESH_PEAK_GOALS.call(c, 0x0020_3724)?;
                            es_setmode_show_goals(c)?;
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            c.g[A0][0] = 0x0047_0000;
                            if c.g[V0][0] == 0 {
                                break 'panel_done;
                            }
                            c.cycles += 8;
                            es_setmode_find_title(c, 0x0020_3824)?;
                            c.cycles += 7;
                            es_setmode_set_title(c, 0xa30, 0x0020_3844, 0x0020_3850)?;
                            frame.restore(c, S0, 0xb0);
                            break 'restored_s0;
                        }
                        EsSetModePanel::Events => {
                            // Show the selected peak's and tab's event list.
                            es_setmode_select_event(c);
                            c.cycles += 2;
                            c.g[A1][0] = 1;
                            if c.g[S0][0] != 0 {
                                c.cycles += 14;
                                es_setmode_show_widget(
                                    c,
                                    S0,
                                    ES_SETMODE_V09,
                                    ES_SETMODE_V07,
                                    [0x0020_3904, 0x0020_391c],
                                )?;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            c.g[A0][0] = 0x0047_0000;
                            if c.g[V0][0] == 0 {
                                break 'panel_done;
                            }
                            c.cycles += 12;
                            es_setmode_find_title(c, 0x0020_3934)?;
                            es_setmode_set_title(c, 0xa48, 0x0020_3950, 0x0020_395c)?;
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0xb0);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0xa0);
        }

        // Epilogue.
        c.cycles += 10;
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Panel 1: show the goal list of the selected peak (`gp-0xb4c`: 0 is s7, 1 is
/// s6, anything else s4; a missing list falls through to the next). Leaves
/// v0 = the flow (`gp-0xb5c`).
fn es_setmode_show_goals(c: &mut Cpu) -> Flow {
    c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_PEAK).lw(c);
    c.g[V1][0] = es_setmode_gp(c, ES_SETMODE_G_PEAK).lw(c);
    'shown: {
        'goal_two: {
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    if c.g[S7][0] != 0 {
                        c.cycles += 16;
                        es_setmode_show_widget(
                            c,
                            S7,
                            ES_SETMODE_V09,
                            ES_SETMODE_V07,
                            [0x0020_3758, 0x0020_3770],
                        )?;
                        c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                        break 'shown;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 1;
                if c.g[V1][0] != c.g[V0][0] {
                    break 'goal_two;
                }
                c.cycles += 2;
                c.g[A1][0] = 1;
                if c.g[S6][0] == 0 {
                    break 'goal_two;
                }
                c.cycles += 16;
                es_setmode_show_widget(
                    c,
                    S6,
                    ES_SETMODE_V09,
                    ES_SETMODE_V07,
                    [0x0020_37ac, 0x0020_37c4],
                )?;
                c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                break 'shown;
        }
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[S4][0] != 0 {
            c.cycles += 14;
            es_setmode_show_widget(
                c,
                S4,
                ES_SETMODE_V09,
                ES_SETMODE_V07,
                [0x0020_37f4, 0x0020_380c],
            )?;
        }
        c.cycles += 1;
        c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
    }
    Ok(())
}

/// Panel 2: pick the event list widget for the selected peak (`gp-0xb4c`) and
/// tab (`gp-0xb50`) into s0 (null for anything out of range): peak 0 and
/// tab 0, 1, 2 are the first three stack slots, peak 1 the next three, peak 2
/// is s5, s3, s1. Leaves v0/a0 as the compiled branches do.
fn es_setmode_select_event(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_PEAK).lw(c);
    c.g[S0][0] = 0;
    let state = c.g[V1][0];
    'chosen: {
        'tail: {
            'peak_not_zero: {
                if c.g[V0][0] != 0 {
                    break 'peak_not_zero;
                }
                // Peak 0: slots 0 (tab 0), 4 (tab 1), 8 (tab 2).
                c.cycles += 3;
                c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_EVENT).lw(c);
                c.g[S0][0] = es_setmode_sp(c, 0).lw(c);
                if c.g[V0][0] == 0 {
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[S0][0] = es_setmode_sp(c, 8).lw(c);
                if c.g[V0][0] == c.g[A0][0] {
                    c.cycles += 2;
                    c.g[S0][0] = es_setmode_sp(c, 4).lw(c);
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[V0][0] ^= 2;
                break 'tail;
            }
            'peak_not_one: {
                c.cycles += 2;
                if c.g[V0][0] != c.g[A0][0] {
                    break 'peak_not_one;
                }
                // Peak 1: slots 0xc, 0x10, 0x14.
                c.cycles += 3;
                c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_EVENT).lw(c);
                c.g[S0][0] = es_setmode_sp(c, 0xc).lw(c);
                if c.g[V0][0] == 0 {
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[S0][0] = es_setmode_sp(c, 0x14).lw(c);
                if c.g[V0][0] == c.g[A0][0] {
                    c.cycles += 2;
                    c.g[S0][0] = es_setmode_sp(c, 0x10).lw(c);
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[V0][0] ^= 2;
                break 'tail;
            }
            // Peak 2: s5, s3, s1 (v1 is the panel, 2).
            c.cycles += 2;
            let not_two = c.g[V0][0] != state;
            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_EVENT).lw(c);
            if not_two {
                break 'chosen;
            }
            c.cycles += 2;
            c.g[S0][0] = c.g[S5][0];
            if c.g[V0][0] == 0 {
                break 'chosen;
            }
            c.cycles += 2;
            let not_one = c.g[V0][0] != c.g[A0][0];
            c.g[V0][0] ^= 2;
            if !not_one {
                c.cycles += 2;
                c.g[S0][0] = c.g[S3][0];
                break 'chosen;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[S1][0];
        }
        // Tab 2 (v0 = tab ^ 2 is zero) keeps the slot, any other tab clears it.
        c.cycles += 1;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{Rng, DATA, GAME_GP};

    /// `EventSelect_SetMode` test fixture addresses (above the fake-call log
    /// and below the heap).
    const T_SM_SELF: u32 = DATA + 0x100;
    const T_SM_STATE: u32 = DATA + 0x1000;
    const T_SM_SCREEN: u32 = DATA + 0x2000;
    const T_SM_COUNT: u32 = DATA + 0x2100;
    const T_SM_LIST: u32 = DATA + 0x2200;
    const T_SM_VT: u32 = DATA + 0x2800;
    const T_SM_OBJS: u32 = DATA + 0x4000;

    /// Name offsets (from `0x470000`) of the 14 panel widgets in hide order,
    /// then the confirm "No" button, the title and the tab label.
    const T_SM_NAMES: [u32; 17] = [
        0x600, 0x918, 0x928, 0x938, 0x948, 0x960, 0x978, 0x990, 0x9a8, 0x9c0, 0x9d8, 0x9f0, 0xa08,
        0xa20, 0x6c8, 0x518, 0xab8,
    ];

    /// `StrHash(string at addr)` through the real function.
    fn t_sm_hash(c: &mut Cpu, addr: u32) -> u32 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7670);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0] as u32
    }

    /// A screen holding the widgets named in `T_SM_NAMES` (the first 14 of
    /// them only where `absent` has no bit set), every widget with the same
    /// all-fake vtable, and the event-select object `T_SM_SELF` pointing at
    /// it and at an empty UI state.
    fn t_sm_screen(c: &mut Cpu, r: &mut Rng, absent: u32, with_screen: bool) {
        for (i, off) in [0x38u32, 0x48, 0x40, 0x28].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(T_SM_VT + off, delta as u16);
            c.bus.write32(T_SM_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        let mut count = 0;
        for (k, off) in T_SM_NAMES.into_iter().enumerate() {
            let name = 0x0047_0000 + off;
            c.bus.write8(name, b'A' + k as u8);
            c.bus.write8(name + 1, b'x');
            c.bus.write8(name + 2, 0);
            if k < 14 && (absent >> k) & 1 != 0 {
                continue;
            }
            let hash = t_sm_hash(c, name);
            let obj = T_SM_OBJS + 0x100 * k as u32;
            c.bus.write32(obj + 8, T_SM_VT);
            c.bus.write32(obj + 0x14, r.next_u32());
            c.bus.write32(obj + 0x38, hash);
            c.bus.write32(T_SM_LIST + 4 * count, obj);
            count += 1;
        }
        c.bus.write32(T_SM_COUNT, count);
        c.bus.write32(T_SM_SCREEN + 0x38, T_SM_COUNT);
        c.bus.write32(T_SM_SCREEN + 0x3c, T_SM_LIST);
        c.bus
            .write32(T_SM_SELF + ES_SETMODE_SCREEN, if with_screen { T_SM_SCREEN } else { 0 });
        c.bus.write32(T_SM_SELF + ES_SETMODE_STATE, T_SM_STATE);
    }

    /// The `gp`-relative globals the function reads.
    fn t_sm_globals(c: &mut Cpu, flow: u32, peak: u32, event: u32) {
        c.bus.write32(GAME_GP - ES_SETMODE_G_FLOW, flow);
        c.bus.write32(GAME_GP - ES_SETMODE_G_PEAK, peak);
        c.bus.write32(GAME_GP - ES_SETMODE_G_EVENT, event);
    }

    /// `EventSelect_SetMode` against the generated body: the confirm popup
    /// (with every focus: no panel code runs), the event panel (focus 2) for
    /// every peak, tab and flow, and focus values with no panel (negative,
    /// 3, 9); widgets present or missing, the screen null in some setups.
    /// The real callees take their short paths: the UI state has an empty
    /// widget list, every widget's vtable ends in fakes. (Panels 0 and 1 call
    /// the not yet ported `RefreshPeakList`/`RefreshPeakGoals`: see
    /// `event_select_set_mode_peak_panels_match`.)
    #[test]
    fn event_select_set_mode_matches() {
        check(
            ee::s05::f_002030a0__generated,
            event_select_set_mode,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_30a0, 24);
                let confirm = seed % 4 == 0;
                let focus: i32 = if confirm {
                    [0, 1, 2, 3, -1][(r.next_u32() % 5) as usize]
                } else {
                    [2, 2, 2, 2, 3, -1, 9][(r.next_u32() % 7) as usize]
                };
                let absent = if seed % 3 == 0 {
                    r.next_u32() & 0x3fff
                } else {
                    0
                };
                let with_screen = seed % 9 != 5;
                t_sm_screen(c, &mut r, absent, with_screen);
                t_sm_globals(c, r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.bus.write32(T_SM_SELF + ES_SETMODE_POPUP, confirm as u32);
                c.bus
                    .write32(T_SM_SELF + ES_SETMODE_NO_SELECTED, r.next_u32() % 2);
                c.bus.write32(T_SM_SELF, r.next_u32());
                c.g[A0][0] = sx(T_SM_SELF);
                c.g[A1][0] = sx(focus as u32);
            },
        );
    }

    /// `EventSelect_SetMode` on panels 0 and 1, which call the real
    /// `RefreshPeakList`/`RefreshPeakGoals`: one screen widget that every name
    /// finds (names hash to 0 in zeroed RAM), the BELibrary dummy and a world,
    /// every peak and flow, the screen null in some setups.
    #[test]
    fn event_select_set_mode_peak_panels_match() {
        const T_BELIB: u32 = 0x008b_8000;
        const T_WORLD: u32 = 0x008a_6000;
        const T_OBJ: u32 = 0x008a_4000;
        const T_OVT: u32 = 0x008a_5000;
        check(
            ee::s05::f_002030a0__generated,
            event_select_set_mode,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_30a1, 40);
                let g = |off: i32| GAME_GP.wrapping_add(off as u32);
                c.bus.write32(g(-0x1f04), T_BELIB);
                c.bus.write32(g(-0x848), T_WORLD);
                c.bus.write32(T_WORLD + 0x78, T_BELIB);
                c.bus.write32(0x004a_7774, [0, 1 << 15, 1 << 16, 3 << 15][(seed % 4) as usize]);
                c.bus.write32(T_SM_SCREEN + 0x38, T_SM_COUNT);
                c.bus.write32(T_SM_SCREEN + 0x3c, T_SM_LIST);
                c.bus.write32(T_SM_COUNT, 1);
                c.bus.write32(T_SM_LIST, T_OBJ);
                c.bus.write32(T_OBJ + 8, T_OVT);
                c.bus.write32(T_OBJ + 0x14, r.next_u32());
                for slot in (0..0x100).step_by(8) {
                    c.bus.write32(T_OVT + slot + 4, FAKE + 0x20);
                }
                c.bus.write32(
                    T_SM_SELF + ES_SETMODE_SCREEN,
                    if seed % 5 == 3 { 0 } else { T_SM_SCREEN },
                );
                c.bus.write32(T_SM_SELF + ES_SETMODE_STATE, T_SM_STATE);
                c.bus.write32(T_SM_SELF + ES_SETMODE_POPUP, 0);
                t_sm_globals(c, r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.g[A0][0] = sx(T_SM_SELF);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
