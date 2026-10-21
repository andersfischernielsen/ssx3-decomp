//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, script, lobby, memory-card and
//! codec helpers, each with a differential test against the generated body.
//!
//! The generated code is the authority for cycles, registers and memory.
//! Calls use [`crate::call`] (a [`Callee`] for a same-shard direct call,
//! [`call_ptr`] for `jalr`/cross-shard calls) so that overridden callees and
//! host-thread unwinds behave exactly as the compiled caller.

use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_a500,
        cfe_state_create_step2_on_widget_created,
        "cFEStateCreateStep2_OnWidgetCreated",
    ),(
        0x001d_8c50,
        memcard_state_base_handle_input,
        "MemCardStateBase_HandleInput",
    ),
    (0x0025_47b8, vd_texture_sink_dtor, "VD_TextureSink_dtor"),
];

// ---------------------------------------------------------------------------
// Addresses and helpers.

/// `StrHash(c)` (0x00317670): hashes the C string at `c` in place and returns
/// the hash.
const STR_HASH: u32 = 0x0031_7670;
/// `UIText_SetTextId(text, id)` (0x003a1000).
const UI_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;
/// `UIText_SetString(text, str)` (0x003a0c50).
const UI_TEXT_SET_STRING: u32 = 0x003a_0c50;
/// `sprintf` (0x00417828).
const SPRINTF: u32 = 0x0041_7828;

/// `a + off`.
#[inline(always)]
fn p(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

/// The address of a `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `self->field` for a word field.
#[inline(always)]
fn lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(p(base, off)).lw(c)
}

/// `self->field` for a signed halfword field.
#[inline(always)]
fn lh(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(p(base, off)).lh(c)
}

/// `self->field = v` for a word field.
#[inline(always)]
fn sw(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(p(base, off)).sw(c, v);
}

/// A virtual call of slot `slot` at `*(obj + vt_off)`, with the method pointer
/// left in `v1` (the vtable is read into `v0` first), as gcc emits for
/// `obj->vt->fn(...)`.
fn vcall_v1(c: &mut Cpu, obj: u64, vt_off: u32, slot: u32, ret: u32) -> Flow {
    let vt = lw(c, obj, vt_off);
    c.g[V0][0] = vt;
    let delta = lh(c, vt, slot);
    let func = lw(c, vt, slot + 4);
    c.g[V1][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

/// A virtual call of slot `slot` at `*(obj + vt_off)`, with the method pointer
/// left in `v0` (the vtable is read into `v1` first), as gcc emits for
/// `((Vt*)*(void**)(obj + vt_off))->fn(...)` when the vtable is reloaded.
fn vcall_v0(c: &mut Cpu, obj: u64, vt_off: u32, slot: u32, ret: u32) -> Flow {
    let vt = lw(c, obj, vt_off);
    c.g[V1][0] = vt;
    let delta = lh(c, vt, slot);
    let func = lw(c, vt, slot + 4);
    c.g[V0][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

// ---------------------------------------------------------------------------
// Front end: widget-created handlers and stats column navigation.

/// `cFEStateCreateStep2_OnWidgetCreated(self, widget)` (0x001ba500): the
/// create-rider screen created one of its five list boxes (month, day,
/// weekday, year, gender); fill the list that belongs to the widget just
/// created. The widget is identified by the hash of its name (`widget+0x38`)
/// against the five constant names.
pub fn cfe_state_create_step2_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        let widget = c.g[S1][0];
        let this = c.g[S2][0];

        'out: {
            // Month.
            c.g[S0][0] = lw(c, widget, 0x38);
            c.g[A0][0] = sx(0x0046_4a08);
            call_ptr(c, STR_HASH, 0x001b_a52c)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 6;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 0;
                Callee {
                    addr: 0x001a_9a18,
                    f: ee::s03::NetFEState_FillYesNoList,
                }
                .call(c, 0x001b_a544)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Day of the month.
            c.cycles += 5;
            c.g[S0][0] = lw(c, widget, 0x38);
            c.g[A0][0] = sx(0x004a_1bd8);
            call_ptr(c, STR_HASH, 0x001b_a558)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 6;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 1;
                Callee {
                    addr: 0x001b_b5d0,
                    f: ee::s03::cFEStateCreateStep2_FillMonthList,
                }
                .call(c, 0x001b_a570)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Weekday.
            c.cycles += 5;
            c.g[S0][0] = lw(c, widget, 0x38);
            c.g[A0][0] = sx(0x004a_1be0);
            call_ptr(c, STR_HASH, 0x001b_a584)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 8;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0x7c1;
                c.g[T0][0] = 1;
                Callee {
                    addr: 0x001b_b840,
                    f: ee::s03::cFEStateCreateStep2_FillDayList,
                }
                .call(c, 0x001b_a5a4)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Year.
            c.cycles += 5;
            c.g[S0][0] = lw(c, widget, 0x38);
            c.g[A0][0] = sx(0x004a_1be8);
            call_ptr(c, STR_HASH, 0x001b_a5b8)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 6;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 0x7c1;
                Callee {
                    addr: 0x001b_b798,
                    f: ee::s03::cFEStateCreateStep2_FillYearList,
                }
                .call(c, 0x001b_a5d0)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }

            // Gender.
            c.cycles += 5;
            c.g[S0][0] = lw(c, widget, 0x38);
            c.g[A0][0] = sx(0x0046_4a18);
            call_ptr(c, STR_HASH, 0x001b_a5e4)?;
            let mismatch = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x30);
            if !mismatch {
                c.cycles += 5;
                c.g[A0][0] = this;
                c.g[A1][0] = widget;
                c.g[A2][0] = 1;
                Callee {
                    addr: 0x001b_b520,
                    f: ee::s03::cFEStateCreateStep2_FillGenderList,
                }
                .call(c, 0x001b_a5fc)?;
                frame.restore(c, S0, 0x30);
            }
        }

        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateDuplicate_OnWidgetCreated(self, text)` (0x001bbe00): one of
/// the duplicate-rider widgets was created; give its text the right label (a
/// fixed caption string, or one of the four weekday names).
pub fn cfe_state_create_duplicate_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        frame.save(c, S0, 0x80);
        let this = c.g[S1][0];
        let text = c.g[S2][0];

        // A fixed caption.
        c.g[S0][0] = lw(c, text, 0x38);
        c.g[A0][0] = sx(0x0046_4c68);
        call_ptr(c, STR_HASH, 0x001b_be30)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = 4;
            sw(c, text, 0x18, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        } else {
            // Two captions only apply while no rider slot is selected.
            let mut label = 0u64;
            let mut weekday_loop = false;
            c.g[V0][0] = lw(c, this, 0x6d0);
            c.cycles += 2;
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                weekday_loop = true;
            } else {
                c.cycles += 6;
                c.g[S0][0] = lw(c, text, 0x38);
                c.g[A0][0] = sx(0x0046_1520);
                call_ptr(c, STR_HASH, 0x001b_be5c)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 3;
                    label = 0x0046_4c78;
                } else {
                    c.g[V0][0] = lw(c, this, 0x6d0);
                    c.cycles += 2;
                    c.g[S1][0] = 0;
                    if c.g[V0][0] != 0 {
                        weekday_loop = true;
                    } else {
                        c.cycles += 6;
                        c.g[S0][0] = lw(c, text, 0x38);
                        c.g[A0][0] = sx(0x0046_4c90);
                        call_ptr(c, STR_HASH, 0x001b_be88)?;
                        let mismatch = c.g[S0][0] != c.g[V0][0];
                        c.g[S1][0] = 0;
                        if mismatch {
                            weekday_loop = true;
                        } else {
                            c.cycles += 2;
                            label = 0x0046_4ca0;
                        }
                    }
                }
            }
            if label != 0 {
                // `UIText_SetTextId(text, GetHashValue32(name))`.
                c.cycles += 7;
                c.g[A0][0] = sx(label as u32);
                call_ptr(c, STR_HASH, 0x001b_bea0)?;
                c.g[A0][0] = text;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, UI_TEXT_SET_TEXT_ID, 0x001b_beac)?;
                frame.restore(c, S0, 0x80);
            } else {
                debug_assert!(weekday_loop);
                // Try the four weekday names.
                c.cycles += 1;
                c.g[S3][0] = 0x0046_0000;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 4) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    if done {
                        break;
                    }
                    c.cycles += 8;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4cb8));
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, SPRINTF, 0x001b_bed4)?;
                    c.g[S0][0] = lw(c, text, 0x38);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, STR_HASH, 0x001b_bee0)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_bebc);
                        }
                        continue;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = text;
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, UI_TEXT_SET_STRING, 0x001b_bef4)?;
                    sw(c, text, 0x18, c.g[S1][0]);
                    break;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
            }
        }

        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetConfig_OnChildEvent(self, child, kind)` (0x001bf5a8): a widget
/// of the network-config screen fired. Kind 15 ("OK") allocates and enters the
/// network-config state; 16 and 20 (and everything else) go to the base
/// handler.
pub fn cfe_state_net_config_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let kind = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];

        if kind == 0x10 {
            c.cycles += 4;
            c.g[A0][0] = this;
            Callee {
                addr: 0x001a_97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b_f69c)?;
        } else if kind < 0x11 {
            if kind == 0xf {
                c.cycles += 3 + 2 + 15;
                c.g[V0][0] = 1; // set by the switch
                sw(c, this, 0x6b0, c.g[V0][0]);
                c.g[A1][0] = sx(0x0046_0bf0);
                c.g[A0][0] = 0x91c;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001b_f60c)?;
                c.g[A1][0] = lw(c, this, 0x10);
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x001b_3f78,
                    f: ee::s03::cFEStateNetConfig_ctor,
                }
                .call(c, 0x001b_f618)?;
                c.g[A0][0] = this;
                c.g[A1][0] = c.g[V0][0];
                Callee {
                    addr: 0x001a_8a40,
                    f: ee::s03::NetFEState_GotoNextState,
                }
                .call(c, 0x001b_f624)?;
                let failed = c.g[V0][0] == 0;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                if !failed {
                    // Wait for the session, then drop it.
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        call_ptr(c, 0x0026_0f80, 0x001b_f63c)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                    call_ptr(c, 0x0026_1460, 0x001b_f644)?;
                    c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                        call_ptr(c, 0x0026_13c8, 0x001b_f654)?;
                        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                        call_ptr(c, 0x0025_b800, 0x001b_f668)?;
                    }
                    c.cycles += 11;
                    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                    call_ptr(c, 0x0026_1408, 0x001b_f670)?;
                    c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                    call_ptr(c, 0x0025_b800, 0x001b_f678)?;
                    call_ptr(c, 0x0025_91b8, 0x001b_f680)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0026_36f0, 0x001b_f68c)?;
                }
                c.cycles += 1;
            } else {
                c.cycles += 3 + 2 + 2 + 2 + 1;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001a_97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b_f6ac)?;
            }
        } else {
            c.cycles += 3 + 3;
            if kind == 0x14 {
                c.cycles += 4;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001a_97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b_f69c)?;
            } else {
                c.cycles += 2 + 2 + 1;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001a_97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b_f6ac)?;
            }
        }

        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateStats_FocusNextColumn(self)` (0x001c4d28): move to the next stats
/// column: unfocus the old column, show the new column's items and advance the
/// index, recursing while the columns are exhausted.
pub fn cfe_state_stats_focus_next_column(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0];

        'out: {
            // Unfocus the widget that had focus (the global one if none).
            let panel = lw(c, this, 0x10);
            let old = lw(c, panel, 0x14);
            c.g[V1][0] = sx(gp(c, 0x2968));
            c.g[A2][0] = old;
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            c.g[A1][0] = 1;
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                vcall_v1(c, c.g[A2][0], 0, 0x10, 0x001c_4d68)?;
            }

            // Hide the current column unless it is the last one.
            c.cycles += 4;
            c.g[S1][0] = 3;
            let column = lw(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column != c.g[S1][0] {
                c.cycles += 8;
                c.g[A1][0] = 0;
                let item = lw(
                    c,
                    (this as u32).wrapping_add((column as u32) << 2) as u64,
                    0x6d0,
                );
                c.g[A2][0] = item;
                vcall_v0(c, item, 8, 0x38, 0x001c_4d98)?;
            }

            // Advance the column, wrapping 4 -> 0.
            c.cycles += 5;
            let column = lw(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32).wrapping_add(1));
            c.g[V1][0] = (c.g[V0][0] < 4) as u64;
            let in_range = c.g[V1][0] != 0;
            let next = c.g[V0][0];
            sw(c, this, 0x6e4, next);
            if !in_range {
                c.cycles += 1;
                sw(c, this, 0x6e4, 0);
            }

            // Show the new column.
            c.cycles += 3;
            let column = lw(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column == c.g[S1][0] {
                // Past the last column: show the "next page" item and recurse.
                c.cycles += 11;
                let item = lw(c, this, 0x6dc);
                c.g[A2][0] = item;
                c.g[A1][0] = 1;
                vcall_v0(c, item, 8, 0x38, 0x001c_4dd8)?;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001c_4d28,
                    f: ee::s03::f_001c4d28,
                }
                .call(c, 0x001c_4de0)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 7;
            let item = lw(
                c,
                (this as u32).wrapping_add((column as u32) << 2) as u64,
                0x6d0,
            );
            c.g[A2][0] = item;
            c.g[V1][0] = lw(c, item, 0x14);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V1][0] = c.g[V1][0] & 1;
            let focused = c.g[V1][0] != 0;
            c.g[A1][0] = 1;
            if focused {
                c.cycles += 7;
                vcall_v0(c, item, 8, 0x38, 0x001c_4e18)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            // Not focusable: skip past it to the next column.
            c.cycles += 3;
            c.g[A0][0] = this;
            Callee {
                addr: 0x001c_4d28,
                f: ee::s03::f_001c4d28,
            }
            .call(c, 0x001c_4e28)?;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateStats_FocusPrevColumn(self)` (0x001c4e40): the mirror image of
/// [`cfe_state_stats_focus_next_column`]; the column index wraps 0 -> 3.
pub fn cfe_state_stats_focus_prev_column(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0];

        'out: {
            let panel = lw(c, this, 0x10);
            let old = lw(c, panel, 0x14);
            c.g[V1][0] = sx(gp(c, 0x2968));
            c.g[A2][0] = old;
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            c.g[A1][0] = 1;
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                vcall_v1(c, c.g[A2][0], 0, 0x10, 0x001c_4e80)?;
            }

            c.cycles += 4;
            c.g[S1][0] = 3;
            let column = lw(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column != c.g[S1][0] {
                c.cycles += 8;
                c.g[A1][0] = 0;
                let item = lw(
                    c,
                    (this as u32).wrapping_add((column as u32) << 2) as u64,
                    0x6d0,
                );
                c.g[A2][0] = item;
                vcall_v0(c, item, 8, 0x38, 0x001c_4eb0)?;
            }

            // Retreat the column, wrapping 0 -> 3 (the unsigned compare also
            // catches the 0xffffffff result).
            c.cycles += 5;
            let column = lw(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32).wrapping_sub(1));
            c.g[V1][0] = (c.g[V0][0] < 4) as u64;
            let in_range = c.g[V1][0] != 0;
            let next = c.g[V0][0];
            sw(c, this, 0x6e4, next);
            if !in_range {
                c.cycles += 1;
                sw(c, this, 0x6e4, c.g[S1][0]);
            }

            c.cycles += 3;
            let column = lw(c, this, 0x6e4);
            c.g[V0][0] = sx((column as u32) << 2);
            if column == c.g[S1][0] {
                c.cycles += 11;
                let item = lw(c, this, 0x6dc);
                c.g[A2][0] = item;
                c.g[A1][0] = 1;
                vcall_v0(c, item, 8, 0x38, 0x001c_4ef0)?;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x001c_4e40,
                    f: ee::s03::f_001c4e40,
                }
                .call(c, 0x001c_4ef8)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 7;
            let item = lw(
                c,
                (this as u32).wrapping_add((column as u32) << 2) as u64,
                0x6d0,
            );
            c.g[A2][0] = item;
            c.g[V1][0] = lw(c, item, 0x14);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V1][0] = c.g[V1][0] & 1;
            let focused = c.g[V1][0] != 0;
            c.g[A1][0] = 1;
            if focused {
                c.cycles += 7;
                vcall_v0(c, item, 8, 0x38, 0x001c_4f30)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 3;
            c.g[A0][0] = this;
            Callee {
                addr: 0x001c_4e40,
                f: ee::s03::f_001c4e40,
            }
            .call(c, 0x001c_4f40)?;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Cameras, codec and audio.

/// `GameCamera_ctor(self, owner)` (0x0015dc70): build the in-race camera: a
/// camera base, the default 3-chase algorithm, a manual camera controller, a
/// chase-cam controller and a script-cam controller, and reset the state.
pub fn game_camera_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 69;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S0][0];
        c.g[S1][0] = c.g[A1][0];

        c.g[A1][0] = c.g[S1][0];
        Callee {
            addr: 0x0015_d698,
            f: ee::s01::CameraBase_ctor,
        }
        .call(c, 0x0015_dc8c)?;

        // The camera's default algorithm.
        c.g[A0][0] = sx((this as u32).wrapping_add(0xc0));
        c.g[V1][0] = sx(0x0045_b888);
        sw(c, this, 0x90, c.g[V1][0]);
        call_ptr(c, 0x0017_6d68, 0x0015_dca0)?;

        // A manual controller.
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx(0x0045_b748);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0015_dcb8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = this;
        Callee {
            addr: 0x0015_cd80,
            f: ee::s01::ManualCamController_ctor,
        }
        .call(c, 0x0015_dcc4)?;
        c.g[A0][0] = 0x44;
        c.g[A1][0] = sx(0x0045_b760);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        sw(c, this, 0xa4, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0015_dce0)?;

        // A chase-cam controller.
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = this;
        Callee {
            addr: 0x0016_19a8,
            f: ee::s01::ChaseCamController_ctor,
        }
        .call(c, 0x0015_dcf0)?;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx(0x0045_b778);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        sw(c, this, 0xa8, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0015_dd0c)?;

        // A script-cam controller.
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = this;
        call_ptr(c, 0x0016_8fe8, 0x0015_dd1c)?;

        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        sw(c, this, 0xac, c.g[V0][0]);
        sw(c, this, 0xb0, c.g[V1][0]);
        sw(c, this, 0xb4, 0);
        sw(c, this, 0x454, 0);
        sw(c, this, 0x458, 0);
        sw(c, this, 0x45c, 0);
        sw(c, this, 0x450, 0);
        c.g[V0][0] = this;
        sw(c, this, 0x460, 0);
        c.g[A0][0] = sx(0x004f_f160);
        c.g[A1][0] = sx(0x004f_f130);
        c.g[A2][0] = 1;
        frame.restore(c, S1, 0x10);
        c.lq(3, c.g[A0][0] as u32);
        frame.restore_ra(c, 0);
        c.sq(3, (this as u32).wrapping_add(0x470));
        c.lq(4, c.g[A1][0] as u32);
        sw(c, this, 0x4a4, c.g[A2][0]);
        c.sq(4, (this as u32).wrapping_add(0x490));
        sw(c, this, 0x4a0, 0);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `PS2_SONY_CODEC_INTERNAL_FeedBitstream(self, stream)` (0x003b0b40): take a
/// chunk from the MPEG decoder, copy it into the bit reservoir, clear the ring
/// slots not covered by the chunk and hand it to `sceMpegAddBs`.
pub fn ps2_sony_codec_internal_feed_bitstream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        let decoder = c.g[S1][0];
        c.g[A0][0] = lw(c, decoder, 0x28);
        Callee {
            addr: 0x003b_06b0,
            f: ee::s13::RCMP_DECODER_GetChunk,
        }
        .call(c, 0x003b_0b70)?;
        c.g[S3][0] = c.g[V0][0];
        if c.g[S3][0] != 0 {
            c.g[S2][0] = lw(c, c.g[S3][0], 0x4);
        }

        c.cycles += 2;
        if c.g[S2][0] == 0 {
            // No usable chunk: clear the first four ring slots.
            c.cycles += 3;
            c.g[S2][0] = 0x10;
            c.g[S0][0] = 0;
            c.g[A0][0] = sx(0xb701_0000);
            loop {
                c.cycles += 7;
                let dest = lw(c, decoder, 0x7c).wrapping_add(c.g[S0][0] & 0xffff_ffff);
                c.g[V0][0] = sx(dest as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[V1][0] = ((c.g[S0][0] as i64) < 16) as u64;
                let more = c.g[V1][0] != 0;
                sw(c, c.g[V0][0], 0, c.g[A0][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_0be8);
                }
            }
        } else {
            // Copy the chunk into the bit reservoir and clear the rest of the
            // ring it spans.
            c.cycles += 12;
            c.g[S0][0] = lw(c, c.g[S3][0], 0x8);
            c.g[A0][0] = lw(c, decoder, 0x7c);
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(7));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(8));
            c.g[S2][0] = c.g[V0][0] & 0xffff_ffff_ffff_fff0;
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x003b_0ba4)?;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, decoder, 0x7c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    sw(c, c.g[V0][0], 0, 0);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_0bb8);
                    }
                }
            }
        }

        c.cycles += 2;
        if c.g[S3][0] == 0 {
            c.g[V0][0] = lw(c, decoder, 0x14);
        } else {
            c.cycles += 4;
            c.g[A0][0] = lw(c, decoder, 0x28);
            c.g[A1][0] = c.g[S3][0];
            Callee {
                addr: 0x003b_06f8,
                f: ee::s13::RCMP_DECODER_ReleaseChunk,
            }
            .call(c, 0x003b_0c18)?;
            c.g[V0][0] = lw(c, decoder, 0x14);
        }

        c.cycles += 14;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = lw(c, decoder, 0x78);
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        sw(c, decoder, 0x14, c.g[V0][0]);
        call_ptr(c, 0x0040_29d0, 0x003b_0c34)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_ctor(self)` (0x0017d298): construct the lobby menu screen and
/// its seven sub-panels, then bind the session and race-manager pointers.
pub fn lobby_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 69;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(0x0046_d350);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let this = c.g[S0][0];
        sw(c, this, 0x4, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(8));
        sw(c, this, 0, c.g[V0][0]);
        call_ptr(c, 0x002c_c018, 0x0017_d2c0)?;

        // `cUIWidget_ctor(widget, parent, ...)` pairs on the stack.
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        sw(c, c.g[SP][0], 0, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x70));
        sw(c, c.g[SP][0], 4, this);
        call_ptr(c, 0x001d_a988, 0x0017_d2d8)?;

        c.g[A0][0] = sx((this as u32).wrapping_add(0x604));
        Callee {
            addr: 0x0017_d200,
            f: ee::s02::LobbyMenu_InitMessagePage,
        }
        .call(c, 0x0017_d2e0)?;

        c.g[A1][0] = c.g[SP][0] | 8;
        c.g[A2][0] = c.g[SP][0] | 0xc;
        sw(c, c.g[SP][0], 8, this);
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        sw(c, c.g[SP][0], 0xc, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x74c));
        sw(c, c.g[SP][0], 0x10, this);
        call_ptr(c, 0x001d_acb8, 0x0017_d300)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
        sw(c, c.g[SP][0], 0x14, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xd2c));
        call_ptr(c, 0x001d_ae20, 0x0017_d310)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
        sw(c, c.g[SP][0], 0x18, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1044));
        sw(c, c.g[SP][0], 0x1c, this);
        call_ptr(c, 0x001d_af08, 0x0017_d328)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        sw(c, c.g[SP][0], 0x20, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1534));
        call_ptr(c, 0x001d_b028, 0x0017_d338)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
        sw(c, c.g[SP][0], 0x24, this);
        sw(c, c.g[SP][0], 0x28, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x18c0));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x001d_b148, 0x0017_d350)?;

        sw(c, c.g[SP][0], 0x2c, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1aac));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c));
        call_ptr(c, 0x001d_b2b0, 0x0017_d360)?;

        // Session / race-manager bindings.
        c.g[A0][0] = Addr(gp(c, 0xda0)).lw(c);
        c.g[V0][0] = this;
        c.g[A1][0] = Addr(gp(c, -0x848)).lw(c);
        sw(c, this, 0x1c0c, c.g[A0][0]);
        sw(c, this, 0x1c04, c.g[A0][0]);
        sw(c, this, 0x1c08, c.g[A0][0]);
        frame.restore_ra(c, 0x30);
        c.g[V1][0] = lw(c, c.g[A1][0], 0x80);
        c.g[A0][0] = lw(c, c.g[V1][0], 0x18);
        sw(c, this, 0x60, c.g[A0][0]);
        c.g[V1][0] = lw(c, c.g[A1][0], 0xb0);
        sw(c, this, 0x64, c.g[V1][0]);
        c.g[A0][0] = lw(c, c.g[A1][0], 0x80);
        c.g[V1][0] = lw(c, c.g[A0][0], 0x20);
        sw(c, this, 0x68, c.g[V1][0]);
        Addr(gp(c, 0x1da0)).sw(c, this);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SelectRandomSong(self)` (0x0028d488): pick a random song
/// index in `[0, tracks)`, store it and make the music player switch to it.
pub fn ssx_audio_system_select_random_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x7fff;
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        let player = lw(c, this, 0x118);
        c.g[S1][0] = lw(c, this, 0x504);
        c.g[S3][0] = lw(c, this, 0x62a4);
        c.g[V1][0] = player;
        c.g[V0][0] = lw(c, player, 0);
        c.g[A0][0] = lw(c, c.g[V0][0], 0x1d8);
        call_ptr(c, 0x002a_df60, 0x0028_d4c0)?;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;

        // Random fraction of the (scaled) track count.
        c.cycles += 14;
        c.mult(0, V0, V0, S1);
        c.g[V1][0] = lw(c, this, 0x118);
        c.divu(0, V0, S0);
        c.g[V0][0] = c.lo[0];
        sw(c, this, 0x508, c.g[V0][0]);
        c.g[V0][0] = lw(c, c.g[V1][0], 0);
        c.g[A0][0] = lw(c, c.g[V0][0], 0x1d8);
        call_ptr(c, 0x002a_df60, 0x0028_d4ec)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff;

        // Skip up to ten songs to find a playable one.
        c.cycles += 5;
        c.divu(0, V0, S0);
        c.g[V1][0] = c.lo[0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = this;
            loop {
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                Callee {
                    addr: 0x0028_d8a0,
                    f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
                }
                .call(c, 0x0028_d528)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = sx((this as u32).wrapping_add(0x118));
                c.g[A1][0] = 1;
                call_ptr(c, 0x002b_42b8, 0x0028_d538)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = this;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_d520);
                }
            }
        }

        // Restart the music player on the new selection.
        c.cycles += 3;
        c.g[V0][0] = lw(c, this, 0x508);
        let same = c.g[S3][0] == c.g[V0][0];
        c.g[S0][0] = sx((this as u32).wrapping_add(0x118));
        if !same {
            c.cycles += 1;
        } else {
            c.cycles += 9;
            c.g[A0][0] = this;
            Callee {
                addr: 0x0028_d8a0,
                f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
            }
            .call(c, 0x0028_d554)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002b_42b8, 0x0028_d564)?;
            c.g[V0][0] = lw(c, this, 0x508);
            if c.g[S3][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x002b_42b8, 0x0028_d580)?;
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldLightMan_AllocOverlapList(self)` (0x002f5298): build the eight
/// 0x40-byte overlap-list headers of the world light manager, each with an
/// empty 0xa14-byte list, then initialise the visibility flags and the 0x20
/// scratch slots.
pub fn world_light_man_alloc_overlap_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_0000 | 0xffff;
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = 0x0048_0000;
        let this = c.g[S3][0];
        c.g[S0][0] = this;
        loop {
            c.cycles += 18;
            sw(c, c.g[S0][0], 0x10, c.g[S1][0]);
            c.g[A0][0] = 0xa14;
            sw(c, c.g[S0][0], 0x14, 0);
            c.g[A1][0] = sx(0x0048_7cd8);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x002f_52ec)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V1][0] = lw(c, this, 0);
            c.g[A0][0] = (c.g[S2][0] < 8) as u64;
            sw(c, c.g[S0][0], 0x40, c.g[V0][0]);
            sw(c, c.g[V0][0], 0, c.g[V1][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
            sw(c, c.g[V0][0], 4, 0);
            sw(c, c.g[V0][0], 8, c.g[S1][0]);
            sw(c, c.g[V0][0], 0x10c, c.g[S1][0]);
            let more = c.g[A0][0] != 0;
            sw(c, c.g[V0][0], 0x210, c.g[S1][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_52d0);
            }
        }

        // Initialise the flags over the eight list headers.
        c.cycles += 8;
        c.g[A0][0] = sx(0xffff_0000 | 0xffff);
        c.g[V0][0] = sx(0x004f_f120);
        c.g[A3][0] = c.g[V0][0];
        c.g[A2][0] = sx((this as u32).wrapping_add(0x3210));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = this;
        c.g[A1][0] = sx((this as u32).wrapping_add(0x3000));
        sw(c, c.g[V1][0], 0x214, 0);
        loop {
            c.cycles += 10;
            sw(c, c.g[V1][0], 0x210, c.g[A0][0]);
            c.lq(2, c.g[A3][0] as u32);
            sw(c, c.g[V1][0], 0x2d0, 0);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x2c0));
            sw(c, c.g[V1][0], 0x380, 0);
            sw(c, c.g[V1][0], 0x384, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x180));
            c.g[V0][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            sw(c, c.g[V1][0], 0x214, 0);
            if event_due(c) {
                deliver_event_here(c, 0x002f_5338);
            }
        }

        c.cycles += 4;
        sw(c, this, 0x3290, 0);
        c.g[V1][0] = c.g[A2][0];
        c.g[S2][0] = 0;
        loop {
            c.cycles += 7;
            sw(c, c.g[V1][0], 0, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[S2][0] < 0x20) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_5370);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MemCardUI_OnNoCards(self)` (0x001d8240): the "no memory cards" reply
/// arrived; mark the screen as busy, ask the front end to show the "no cards"
/// panel and, when the object exists, relabel it with the localized text.
pub fn memcard_ui_on_no_cards(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let fe = Addr(gp(c, -0x10c8)).lw(c);
        c.g[A2][0] = fe;
        c.g[V1][0] = 1;
        sw(c, fe, 0x1c0, 1);
        c.g[V0][0] = lw(c, fe, 0x214);
        let already = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 1;
        if !already {
            c.cycles += 6;
            sw(c, fe, 0x214, c.g[V1][0]);
            c.g[V0][0] = lw(c, fe, 8);
            let delta = lh(c, c.g[V0][0], 0x118);
            let func = lw(c, c.g[V0][0], 0x11c);
            c.g[V1][0] = func;
            c.g[A0][0] = sx((fe as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x001d_8288)?;
        }

        // The "no cards" object (found by name hash).
        c.cycles += 10;
        c.g[A0][0] = sx(0x0046_78b0);
        c.g[S0][0] = Addr(gp(c, -0x10c8)).lw(c);
        call_ptr(c, STR_HASH, 0x001d_8298)?;
        c.g[A0][0] = lw(c, c.g[S0][0], 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001d_82a4)?;
        c.g[S3][0] = c.g[V0][0];
        let none = c.g[S3][0] == 0;
        c.g[A1][0] = 0x0046_0000;
        let mut tail = true;
        if !none {
            c.cycles += 32;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d60));
            c.g[A0][0] = 0x7d0;
            call_ptr(c, 0x0031_7e30, 0x001d_82c4)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_78c8);
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[S0][0] = lw(c, c.g[V0][0], 0x8c);
            c.g[S1][0] = lw(c, c.g[S0][0], 4);
            c.g[V0][0] = lh(c, c.g[S1][0], 0x20);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, STR_HASH, 0x001d_82e8)?;
            c.g[V1][0] = lw(c, c.g[S1][0], 0x24);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, c.g[V1][0] as u32, 0x001d_82f8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x001d_8304)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x003a_0d00, 0x001d_8310)?;
            c.g[V0][0] = lw(c, c.g[S3][0], 8);
            c.g[A1][0] = 1;
            c.g[A0][0] = lh(c, c.g[V0][0], 0x48);
            c.g[V1][0] = lw(c, c.g[V0][0], 0x4c);
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_8328)?;
            let no_buf = c.g[S2][0] == 0;
            tail = !no_buf;
            if !no_buf {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0031_7e98, 0x001d_8338)?;
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MemCardStateBase_HandleInput(self, input)` (0x001d8c50): let the input
/// object's "confirm" and "cancel" slots drive the two save/load actions and
/// play the matching screen frame. Returns whether input was consumed.
pub fn memcard_state_base_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        c.g[S0][0] = c.g[A1][0]; // input
        c.g[S1][0] = c.g[A0][0]; // self
        let input = c.g[S0][0];
        let this = c.g[S1][0];
        c.g[S2][0] = 0;
        c.g[V1][0] = lw(c, this, 0x1a8);
        let busy = c.g[V1][0] != 1;
        let mut tail = false;
        if busy {
            // Confirm slot: `input->vt[19]()`.
            c.cycles += 7;
            let vt = lw(c, input, 8);
            c.g[V1][0] = vt;
            let delta = lh(c, vt, 0x98);
            let func = lw(c, vt, 0x9c);
            c.g[V0][0] = func;
            c.g[A0][0] = sx((input as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x001d_8c98)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S2][0] = 1;
                c.g[V0][0] = lw(c, this, 0x40);
                let no_screen = c.g[V0][0] == 0;
                sw(c, this, 0x1fc, c.g[S2][0]);
                if no_screen {
                    tail = true;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = sx(0x0046_7940);
                    call_ptr(c, STR_HASH, 0x001d_8cbc)?;
                    c.g[A0][0] = lw(c, this, 0x40);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x001d_8cc8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let no_frame = c.g[A1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if !no_frame {
                        c.cycles += 5;
                        c.g[A0][0] = lw(c, this, 0x40);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x001d_8ce4)?;
                        c.g[V0][0] = c.g[S2][0];
                    }
                }
            } else {
                // Cancel slot: `input->vt[20]()`.
                c.g[V1][0] = lw(c, input, 8);
                c.cycles += 6;
                c.g[A0][0] = lh(c, c.g[V1][0], 0xa0);
                c.g[V0][0] = lw(c, c.g[V1][0], 0xa4);
                c.g[A0][0] = sx((input as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x001d_8cfc)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S2][0];
                if !none {
                    c.cycles += 4;
                    c.g[S2][0] = 1;
                    c.g[V0][0] = lw(c, this, 0x40);
                    let no_screen = c.g[V0][0] == 0;
                    sw(c, this, 0x1f8, c.g[S2][0]);
                    if no_screen {
                        tail = true;
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = sx(0x0046_7950);
                        call_ptr(c, STR_HASH, 0x001d_8d20)?;
                        c.g[A0][0] = lw(c, this, 0x40);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001d_8d2c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = c.g[S2][0];
                        if no_frame {
                            tail = true;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = lw(c, this, 0x40);
                            c.g[A2][0] = 1;
                            call_ptr(c, 0x0039_c870, 0x001d_8d48)?;
                            tail = true;
                        }
                    }
                }
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        if tail {
            c.cycles += 1;
        }
        c.g[V0][0] = c.g[S2][0];
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VD_TextureSink_dtor(self, flags)` (0x002547b8): release every texture of
/// the sink, then free the table, the name and the decoder.
pub fn vd_texture_sink_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        c.g[S5][0] = c.g[A1][0];
        c.g[V0][0] = sx(0x0048_0270);
        sw(c, this, 4, c.g[V0][0]);

        'scan: {
            c.g[V1][0] = lw(c, this, 0x18);
            if (c.g[V1][0] as i64) <= 0 {
                break 'scan;
            }
            c.cycles += 4;
            c.g[S4][0] = 0xffff_ffff_ffff_ffff;
            c.g[S3][0] = sx(0x0050_9430);
            c.g[V1][0] = lw(c, this, 0x14);
            loop {
                // Release texture `i`.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S0][0] = lw(c, c.g[V0][0], 0);
                let empty = c.g[S0][0] == 0;
                c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
                if !empty {
                    c.cycles += 10;
                    c.g[A1][0] = lw(c, c.g[S0][0], 0);
                    c.g[V0][0] = lw(c, c.g[A2][0], 0x10d8);
                    c.g[A0][0] = lh(c, c.g[V0][0], 0x190);
                    c.g[V1][0] = lw(c, c.g[V0][0], 0x194);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0025_4834)?;
                    sw(c, c.g[S0][0], 0, c.g[S4][0]);
                    c.g[V0][0] = lw(c, c.g[S3][0], 4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, c.g[V0][0] as u32, 0x0025_4844)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, this, 0x18);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[V1][0] = lw(c, this, 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0025_4808);
                }
            }
        }

        // Free the texture table.
        c.cycles += 3;
        c.g[A0][0] = lw(c, this, 0x14);
        let no_table = c.g[A0][0] == 0;
        let mut name;
        if no_table {
            name = lw(c, this, 0xc);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0025_486c)?;
            name = lw(c, this, 0xc);
        }
        c.g[A1][0] = name;
        // Free the name.
        c.cycles += 2;
        let no_name = c.g[A1][0] == 0;
        sw(c, this, 0x14, 0);
        if !no_name {
            c.cycles += 3;
            c.g[A0][0] = lw(c, this, 0);
            call_ptr(c, 0x003b_0680, 0x0025_4880)?;
            sw(c, this, 0xc, 0);
        }
        // Virtual destructor tail: restore the base-object vtable and, for a
        // deleting destructor, call the operator delete.
        c.cycles += 5;
        c.g[V0][0] = sx(0x0048_02b0);
        c.g[V1][0] = c.g[S5][0] & 1;
        sw(c, this, 4, c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = sx(0x0051_0000);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            c.g[A0][0] = this;
            call_ptr(c, c.g[V0][0] as u32, 0x0025_48a8)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
    use crate::difftest::{Rng, DATA, GAME_GP, RA as SENTINEL};

    /// `check`, but a caller-saved difference fails too: scratch registers are
    /// visible to fake callees, so nothing may differ.
    pub(super) fn check_all(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// The test world / race manager / BE singleton live high in `DATA`
    /// (below `DATA + 0x40000`, where the allocator arena starts).
    const W: u32 = DATA + 0x10_0000;
    const RM: u32 = DATA + 0x10_0100;
    const BE: u32 = DATA + 0x10_2000;
    /// The main heap's dlmalloc arena.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// A real heap block of `size` bytes.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    /// The engine's size-class pool allocator, enabled through `gp+0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A machine with a plausible game `gp`, the world singleton and a
    /// working pool + dlmalloc allocator.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP - 0x1f04, BE);
        c.bus.write32(GAME_GP - 0x848, W);
        c.bus.write32(W + 0x84, RM);
        c.bus.write32(RM + 0x7c, 0);
        c.bus.write32(RM + 0x64, 0);
        c.bus.write32(RM + 0x84, 0);
        pool_allocator(c);
        heap_init(c);
        r
    }

    /// Write the one-character string `"A"` (hash 0x41) at a constant-name
    /// address so a `StrHash` comparison can match a name.
    fn name_a(c: &mut Cpu, addr: u32) {
        c.bus.write8(addr, b'A');
        c.bus.write8(addr + 1, 0);
    }

    #[test]
    fn cfe_state_create_step2_on_widget_created_matches() {
        const HASHES: [u32; 5] = [
            0x0046_4a08,
            0x004a_1bd8,
            0x004a_1be0,
            0x004a_1be8,
            0x0046_4a18,
        ];
        check_all(
            ee::s03::f_001ba500__generated,
            cfe_state_create_step2_on_widget_created,
            120,
            &|c, seed| {
                machine(c, seed, 0x1ba5, 0);
                let widget = DATA + 0x400;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = widget as u64;
                let k = seed % 6;
                if k < 5 {
                    name_a(c, HASHES[k as usize]);
                    c.bus.write32(widget + 0x38, 0x41);
                } else {
                    // No list matches: the widget name is something else.
                    c.bus.write32(widget + 0x38, 1);
                }
                // The list-box fillers read their widget; leave everything at
                // zero except the name so they take their short paths.
                c.bus.write32(widget + 0x8, 0);
                c.bus.write32(widget + 0x40, 0);
            },
        );
    }

            /// The stats-column fixture: a self object, a panel with an old focus
    /// widget, five column items with vtables of fakes, and a chosen column.
    fn stats_fixture(c: &mut Cpu, seed: u32, salt: u64, forward: bool) -> u64 {
        let mut r = machine(c, seed, salt, 0);
        let this_ = DATA;
        let panel = DATA + 0x100;
        c.bus.write32(this_ + 0x10, panel);
        if seed % 2 == 0 {
            let old = DATA + 0x200;
            c.bus.write32(panel + 0x14, old);
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(old, vt);
        }
        for i in 0..5u32 {
            let obj = DATA + 0x1000 + 0x100 * i;
            c.bus.write32(this_ + 0x6d0 + 4 * i, obj);
            let vt0 = vtable(c, &mut r, &[0x38]);
            c.bus.write32(obj + 8, vt0);
        }
        // The column the function lands on after its increment/decrement is
        // the one whose focus flag matters.
        let before = seed % 3;
        let after = if forward {
            (before + 1) % 4
        } else {
            (before + 3) % 4
        };
        c.bus.write32(this_ + 0x6e4, before);
        let flag = if seed % 3 == 0 { 0x40u32 } else { 0 };
        c.bus.write32(DATA + 0x1000 + 0x100 * after + 0x14, flag);
        this_ as u64
    }

            #[test]
    fn game_camera_ctor_matches() {
        check_all(ee::s01::f_0015dc70, game_camera_ctor, 40, &|c, seed| {
            let _r = alloc_base(c, seed, 0x15dc);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = BE as u64;
        });
    }

    #[test]
    fn ps2_sony_codec_internal_feed_bitstream_matches() {
        check_all(
            ee::s13::f_003b0b40,
            ps2_sony_codec_internal_feed_bitstream,
            60,
            &|c, seed| {
                machine(c, seed, 0x3b0b, 0);
                let decoder = DATA + 0x2000;
                let stream = DATA + 0x3000;
                c.g[A0][0] = decoder as u64;
                c.g[A1][0] = stream as u64;
                c.bus.write32(decoder + 0x7c, DATA + 0x4000);
                c.bus.write32(decoder + 0x78, DATA + 0x5000);
                c.bus.write32(decoder + 0x28, DATA + 0x6000);
                if seed % 2 == 0 {
                    // No chunk pending.
                    c.bus.write32(decoder, 0);
                    c.bus.write32(decoder + 8, 0);
                } else {
                    // A chunk of 0x20 bytes at ring offset 0x20.
                    let chunk = DATA + 0x7000;
                    c.bus.write32(decoder, chunk);
                    c.bus.write32(chunk + 4, 0x20);
                    c.bus.write32(chunk + 8, 0x28);
                }
                c.bus.write32(DATA + 0x6000, 0);
            },
        );
    }

    #[test]
    fn lobby_menu_ctor_matches() {
        check_all(ee::s02::f_0017d298, lobby_menu_ctor, 40, &|c, seed| {
            let _r = alloc_base(c, seed, 0x17d2);
            c.g[A0][0] = DATA as u64;
            let obj = DATA + 0x10_1000;
            c.bus.write32(W + 0x80, obj);
            c.bus.write32(obj + 0x18, seed);
            c.bus.write32(obj + 0x20, seed * 3 + 1);
            c.bus.write32(W + 0xb0, seed ^ 0x55);
            c.bus.write32(gp_at(0xda0), DATA + 0x10_2000);
        });
    }

                #[test]
    fn memcard_state_base_handle_input_matches() {
        check_all(
            ee::s04::f_001d8c50__generated,
            memcard_state_base_handle_input,
            60,
            &|c, seed| {
                machine(c, seed, 0x1d8c, 0);
                let this_ = DATA + 0x1000;
                let input = DATA + 0x2000;
                c.g[A0][0] = this_ as u64;
                c.g[A1][0] = input as u64;
                c.bus.write32(this_ + 0x1a8, 0);
                c.bus.write32(this_ + 0x40, 0);
                // `MenuSys_IsActive` stands in for the input slots: it returns
                // whether the object's first word is non-zero and +0x54 is too.
                let vt = DATA + 0x3000;
                c.bus.write16(vt + 0x98, 0);
                c.bus.write32(vt + 0x9c, 0x002c_c098);
                c.bus.write16(vt + 0xa0, 0x100);
                c.bus.write32(vt + 0xa4, 0x002c_c098);
                c.bus.write32(input + 8, vt);
                let first = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(input, first);
                c.bus.write32(input + 0x54, 1);
                let second = if seed % 3 == 1 { 1 } else { 0 };
                c.bus.write32(input + 0x100, second);
                c.bus.write32(input + 0x154, 1);
            },
        );
    }

    #[test]
    fn vd_texture_sink_dtor_matches() {
        check_all(ee::s06::f_002547b8__generated, vd_texture_sink_dtor, 60, &|c, seed| {
            let _r = alloc_base(c, seed, 0x2547);
            let this_ = DATA + 0x1000;
            let n = seed % 4;
            let table = alloc(c, 4 * (n + 1));
            for i in 0..n {
                if seed % 2 == 0 || i == 0 {
                    let tex = alloc(c, 0x10);
                    c.bus.write32(table + 4 * i, tex);
                } else {
                    c.bus.write32(table + 4 * i, 0);
                }
            }
            c.bus.write32(this_ + 0x18, n);
            c.bus.write32(this_ + 0x14, table);
            c.bus.write32(this_ + 0xc, 0);
            c.bus.write32(this_, 0);
            c.g[A0][0] = this_ as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }
}
