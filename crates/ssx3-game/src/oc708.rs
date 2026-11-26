//! Staging module oc708 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc708`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_69f0, spch_add_bank, "SPCH_AddBank"),
    (
        0x002c_6dc0,
        loc_string_table_resolve_string,
        "LocStringTable_ResolveString",
    ),
    (0x0039_72a0, ui_anim_ctor, "UIAnim_ctor"),
    (
        0x0019_4bf0,
        cfe_state_main_menu_on_render,
        "cFEStateMainMenu_OnRender",
    ),
    (0x003a_34c0, cui_vector_set_ui_data, "cUIVector_setUIData"),
    (
        0x003a_0d00,
        cui_text_set_unicode_string_wrap,
        "cUIText_setUnicodeStringWrap",
    ),
];

/// The address `gp + off` (a 32-bit guest pointer).
fn spch_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// The address `base + off`, for a register value `base`.
fn spch_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `int SPCH_AddBank(void* bank)` (0x3d69f0): scans the bank table at
/// `gp+0x1734` (a zero-terminated pointer array, at most `gp+0x1738` slots
/// used) and returns the first free slot index, or -1. Its loops and the
/// bytes of `bank` (flags at +4 and +6, the bit field at +5) decide the
/// path. A free slot gets `bank` stored in it. No calls are made, so the
/// frame is leaf-only: no stack, no `ra` save.
pub fn spch_add_bank(c: &mut Cpu) {
    'b_3d6bb4: {
        c.cycles += 4;
        c.g[T1][0] = c.lw(spch_gp(c, 0x1734));
        c.g[A3][0] = c.g[A0][0];
        let t = c.g[T1][0] == 0;
        c.g[T2][0] = u64::MAX;
        if t {
            break 'b_3d6bb4;
        }
        'b_3d6a9c: {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(spch_ea(c.g[A3][0], 0x6));
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(spch_gp(c, 0x1738));
            if t {
                break 'b_3d6a9c;
            }
            'b_3d6a58: {
                c.cycles += 13;
                c.g[A0][0] = c.lbu(spch_ea(c.g[A3][0], 0x4));
                c.g[T0][0] = 0;
                c.g[A1][0] = c.lbu(spch_ea(c.g[A3][0], 0x5));
                c.g[V1][0] = c.g[A0][0] & 0xf;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                c.g[A0][0] = c.g[A0][0] & 0x80;
                c.mult(0, 2, 5, 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                c.g[V0][0] &= 0x3ffc;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let t = c.g[A0][0] == 0;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if t {
                    break 'b_3d6a58;
                }
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe));
                c.g[A0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[A0][0] != 0 {
                    c.g[V1][0] = c.g[V0][0];
                }
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 4;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
            let t = c.g[V0][0] != 0;
            if t {
                break 'b_3d6bb4;
            }
            'b_3d6a98: {
                c.cycles += 3;
                c.g[A1][0] = c.lbu(spch_ea(c.g[A3][0], 0x6));
                let t = c.g[A1][0] == 0;
                c.g[A0][0] = 0;
                if t {
                    break 'b_3d6a98;
                }
                c.cycles += 3;
                c.g[T0][0] = 0xff;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                loop {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu(spch_ea(c.g[V0][0], 0x1));
                    let t = c.g[V1][0] != c.g[T0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_3d6bb4;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_6a80);
                        }
                        continue;
                    }
                    break 'b_3d6a98;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(spch_gp(c, 0x1738));
        }
        'b_3d6ae0: {
            c.cycles += 4;
            c.g[A0][0] = u64::MAX;
            c.g[A1][0] = 0;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = c.g[V0][0];
            if t {
                break 'b_3d6ae0;
            }
            'b_3d6ac0: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[T1][0] as u32);
                let t = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if t {
                    break 'b_3d6ac0;
                }
                c.cycles += 2;
                c.g[A0][0] = 0;
                break 'b_3d6ae0;
            }
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                if t {
                    break 'b_3d6ae0;
                }
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] != 0 {
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_6ac0);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[A1][0];
                break 'b_3d6ae0;
            }
        }
        c.cycles += 3;
        c.g[T2][0] = c.g[A0][0];
        let t = (c.g[T2][0] as i64) < 0;
        if t {
            break 'b_3d6bb4;
        }
        'b_3d6ba8: {
            c.cycles += 4;
            c.g[V1][0] = c.lbu(spch_ea(c.g[A3][0], 0x4));
            c.g[V0][0] = c.g[V1][0] & 0x80;
            let t = c.g[V0][0] == 0;
            c.g[T3][0] = sx((c.g[T2][0] as u32) << 2);
            if t {
                break 'b_3d6ba8;
            }
            c.cycles += 12;
            c.g[V1][0] &= 0xf;
            c.g[A1][0] = c.lbu(spch_ea(c.g[A3][0], 0x5));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
            c.mult(0, 2, 5, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
            c.g[V0][0] &= 0x3ffc;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = c.lbu(c.g[T0][0] as u32);
            let t = (c.g[A0][0] as i64) <= 0;
            c.g[V1][0] = c.lw(spch_gp(c, 0x1744));
            if t {
                break 'b_3d6ba8;
            }
            c.cycles += 2;
            c.cycles += 14;
            c.divu(0, 3, 4);
            c.g[V0][0] = c.hi[0];
            c.mult(0, 3, 2, 5);
            c.mult(0, 6, 2, 5);
            c.divu(0, 3, 4);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = c.lo[0];
            c.divu(0, 2, 4);
            c.g[A1][0] = c.g[V1][0] & 0x7;
            c.g[A0][0] = sx((c.g[V1][0] as u32) >> 3);
            c.g[V0][0] = c.lo[0];
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            let t = (c.g[A2][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if t {
                break 'b_3d6ba8;
            }
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[T1][0] = 0x1;
            c.g[T0][0] = 0x8;
            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
            loop {
                'b_3d6b9c: {
                    c.cycles += 5;
                    c.g[V1][0] = sx(((c.g[T1][0] as u32) << ((c.g[A1][0] as u32) & 31)) as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] |= c.g[V1][0];
                    let t = c.g[A1][0] != c.g[T0][0];
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    if t {
                        break 'b_3d6b9c;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                if c.g[A2][0] != 0 {
                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_6b80);
                    }
                    continue;
                }
                break 'b_3d6ba8;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(spch_gp(c, 0x1734));
        c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[A3][0]);
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[T2][0];
    c.pc = t;
}

/// `void LocStringTable_ResolveString(LocStringTable* t)` (0x2c6dc0): a
/// forwarder. Calls `LocStringTable_GetStringPtr` (0x3dbb68) through the
/// function table with `a0 = t->table` (+4), in a 0x10 frame holding `ra`,
/// and returns to the caller at 0x2c6dd0 with that call's results.
pub fn loc_string_table_resolve_string(c: &mut Cpu) {
    crate::call::tail_wrapper(c, Some(0x4), Target::Table(0x003d_bb68), 0x002c_6dd0);
}

/// `void UIAnim_ctor(UIAnim* self, UIAnimDesc* desc)` (0x3972a0): installs
/// the UIAnim vtable (0x494d08) and the default fields (+0x8 = -1, +0xc =
/// 0xffff, +0xa = 0, +0x4 = 0), keeps `desc`, and for each of the desc's
/// `count` (+4) entries lays out a pointer array of `count` slots at +4,
/// filling it from the desc entries (16 bytes of header, stride from each
/// entry's halfword at +4) through `UIAnim_SetData` (0x397298).
pub fn ui_anim_ctor(c: &mut Cpu) {
    guest(c, ui_anim_ctor_body);
}

fn ui_anim_ctor_body(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0x40);
    c.g[V0][0] = 0x490000;
    frame.save(c, S2, 0x10);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4d08));
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S1, 0x20);
    frame.save(c, S0, 0x30);
    c.g[V1][0] = u64::MAX;
    frame.save_ra(c, 0);
    c.g[A0][0] = 0xffff;
    let this = c.g[S2][0] as u32;
    c.sw(this.wrapping_add(0x10), c.g[V0][0]);
    c.sh(this.wrapping_add(0x8), c.g[V1][0]);
    c.sh(this.wrapping_add(0xc), c.g[A0][0]);
    c.sw(this, c.g[A1][0]);
    c.sw(this.wrapping_add(0x4), 0);
    c.sh(this.wrapping_add(0xa), 0);
    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    let t = c.g[S0][0] == 0;
    c.g[S1][0] = sx(this.wrapping_add(0x4));
    'b_397388: {
        if t {
            break 'b_397388;
        }
        'b_39733c: {
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d60));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0039_7308)?;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0;
            c.g[V0][0] = c.g[A0][0];
            if t {
                break 'b_39733c;
            }
            c.cycles += 1;
            c.g[A1][0] = u64::MAX;
            loop {
                c.cycles += 7;
                c.sw(c.g[V0][0] as u32, 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                let t = c.g[V1][0] != c.g[A1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_7320);
                    }
                    continue;
                }
                break 'b_39733c;
            }
        }
        c.cycles += 6;
        c.sw(c.g[S1][0] as u32, c.g[A0][0]);
        c.g[S1][0] = 0;
        c.g[V0][0] = c.lw(this);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        let t = c.g[V1][0] == 0;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        if t {
            break 'b_397388;
        }
        c.cycles += 1;
        loop {
            c.cycles += 12;
            c.g[V0][0] = c.lw(this.wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A1][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Callee {
                addr: 0x0039_7298,
                f: ee::s13::f_00397298,
            }
            .call(c, 0x0039_7370)?;
            c.g[V1][0] = c.lw(this);
            c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7358);
                }
                continue;
            }
            break 'b_397388;
        }
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEStateMainMenu_OnRender(cFEStateMainMenu* self)` (0x194bf0): looks
/// up the screen object (`UIScreen_FindObject(self->screen, StrHash(...))`,
/// +0x40), and when it exists and `self`'s mode bits (`lb +0x55 < 2`, then
/// the `& 0x3f == 5` test on the global at gp-0x704) say so, animates the
/// menu's two sub-objects through their vtables (the entries at +0xc8/+0xcc
/// and +0x40/+0x44 of the object's vtable), then the object's +0x58/+0x5c
/// slot, with the alpha-half show on the way out.
pub fn cfe_state_main_menu_on_render(c: &mut Cpu) {
    guest(c, cfe_state_main_menu_on_render_body);
}

fn cfe_state_main_menu_on_render_body(c: &mut Cpu) -> Flow {
    let frame = Frame::open(c, 0x40);
    'b_194cd0: {
        c.cycles += 14;
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x46_0000;
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x0031_7670, 0x0019_4c10)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4c1c)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
        if t {
            break 'b_194cd0;
        }
        'b_194c48: {
            c.cycles += 4;
            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x55));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x1d));
                break 'b_194c48;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0019_4498,
                f: ee::s02::f_00194498,
            }
            .call(c, 0x0019_4c40)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
            break 'b_194cd0;
        }
        c.cycles += 4;
        c.g[V1][0] = 0x5;
        c.g[V0][0] &= 0x3f;
        let t = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
        if t {
            break 'b_194cd0;
        }
        c.cycles += 30;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0019_4c70)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0019_4c88)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = c.g[SP][0];
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x23));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1c));
        c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x2b));
        c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x24));
        let sp_now = sp(c);
        c.sdl(2, sp_now.wrapping_add(0x7));
        c.sdr(2, sp_now);
        c.sdl(3, sp_now.wrapping_add(0xf));
        c.sdr(3, sp_now.wrapping_add(0x8));
        c.swc1(0, sp_now);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0019_4ccc)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
        break 'b_194cd0;
    }
    'b_194d04: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if t {
            break 'b_194d04;
        }
        c.cycles += 8;
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17d8));
        call_ptr(c, 0x0031_7670, 0x0019_4ce4)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4cf0)?;
        let t = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if t {
            break 'b_194d04;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0019_4498,
            f: ee::s02::f_00194498,
        }
        .call(c, 0x0019_4d00)?;
        frame.restore(c, S0, 0x30);
        break 'b_194d04;
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void cUIVector_setUIData(cUIVector* self, UIVectorData* data)`
/// (0x3a34c0): loads the base object (`cUIThing_setUIData`, 0x39fe00), then
/// sets the vector's flag bits from `data` (+0x22, +0x21) in the word at
/// +0x74, and allocates the element arrays for `data->count` (+0x20, a
/// signed byte): one of 16 bytes per element (self+0x80), two of 12 bytes
/// (+0x84 and self+0x7c, which holds its pointer on the second) and an
/// optional 48-byte block when the flag at +0x74 says so. The element copy
/// loop moves 16-byte records from the static table at 0x4ff0d8 into the
/// element array at +0x80.
pub fn cui_vector_set_ui_data(c: &mut Cpu) {
    guest(c, cui_vector_set_ui_data_body);
}

fn cui_vector_set_ui_data_body(c: &mut Cpu) -> Flow {
    c.cycles += 32;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x60);
    frame.save(c, S3, 0x30);
    frame.save(c, S4, 0x20);
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    c.g[RA][0] = sx(0x003a_34ec);
    c.g[S2][0] = c.g[A1][0];
    Callee {
        addr: 0x0039_fe00,
        f: ee::s13::f_0039fe00,
    }
    .call(c, 0x003a_34ec)?;
    // 'b_3a3548: the flag bits at +0x74.
    c.g[A0][0] = c.lbu(spch_ea(c.g[S2][0], 0x22));
    c.g[A2][0] = 0xffff_ffff_ffff_fffe;
    c.g[V0][0] = c.lw(spch_ea(c.g[S1][0], 0x74));
    c.g[V1][0] = 0xffff_ffff_ffff_fffd;
    c.g[A0][0] &= 0x1;
    c.g[A1][0] = 0xffff_ffff_fffe_0000;
    c.g[V0][0] &= c.g[A2][0];
    c.g[A1][0] |= 0x3;
    c.g[V0][0] |= c.g[A0][0];
    c.g[A2][0] = 0x10000;
    c.g[V1][0] &= c.g[V0][0];
    c.g[A2][0] |= 0xfffc;
    c.sw(spch_ea(c.g[S1][0], 0x74), c.g[V1][0]);
    c.g[V1][0] &= c.g[A1][0];
    c.g[V0][0] = c.lb(spch_ea(c.g[S2][0], 0x21));
    c.g[V0][0] &= 0x7fff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] |= c.g[V0][0];
    c.g[A2][0] &= c.g[V1][0];
    let t = c.g[A2][0] == 0;
    c.sw(spch_ea(c.g[S1][0], 0x74), c.g[V1][0]);
    if !t {
        c.cycles += 2;
        c.g[V0][0] = sx(spch_ea(c.g[S2][0], 0x24));
        c.sw(spch_ea(c.g[S1][0], 0x78), c.g[V0][0]);
    }
    'b_3a3660: {
        c.cycles += 11;
        c.g[A0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
        c.g[V1][0] = 0x10000;
        c.g[V0][0] = c.lw(spch_ea(c.g[S1][0], 0x74));
        c.g[V1][0] |= 0xffff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 17);
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= c.g[A0][0];
        c.sw(spch_ea(c.g[S1][0], 0x74), c.g[V0][0]);
        c.g[S0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
        let t = c.g[S0][0] == 0;
        c.g[A2][0] = 0x100;
        if t {
            break 'b_3a3660;
        }
        'b_3a35a0: {
            c.cycles += 10;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40c8));
            c.g[A3][0] = 0;
            c.g[S3][0] = sx(spch_ea(c.g[S1][0], 0x7c));
            call_ptr(c, 0x0031_7e30, 0x003a_358c)?;
            c.g[S4][0] = sx(spch_ea(c.g[S1][0], 0x80));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0;
            c.g[S5][0] = sx(spch_ea(c.g[S1][0], 0x84));
            if t {
                break 'b_3a35a0;
            }
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        'b_3a35d8: {
            c.cycles += 12;
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = 0x49_0000;
            c.g[A2][0] = 0x100;
            c.g[S0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40d8));
            c.mult(0, 0, 16, 4);
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x003a_35c8)?;
            let t = c.g[S0][0] == 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_3a35d8;
            }
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        'b_3a3610: {
            c.cycles += 12;
            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = 0x49_0000;
            c.g[A2][0] = 0x100;
            c.g[S0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40e8));
            c.mult(0, 0, 16, 4);
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x003a_3600)?;
            let t = c.g[S0][0] == 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_3a3610;
            }
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 4;
        c.sw(c.g[S5][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if t {
            break 'b_3a3660;
        }
        c.cycles += 2;
        c.g[A1][0] = 0x50_0000;
        c.g[A0][0] = 0;
        loop {
            c.cycles += 14;
            c.g[V0][0] = c.lw(spch_ea(c.g[S1][0], 0x80));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
            c.ldl(6, spch_ea(c.g[T1][0], 0x7));
            c.ldr(6, spch_ea(c.g[T1][0], 0));
            c.g[A3][0] = c.lw(spch_ea(c.g[T1][0], 0x8));
            c.sdl(6, spch_ea(c.g[V0][0], 0x7));
            c.sdr(6, spch_ea(c.g[V0][0], 0));
            c.sw(spch_ea(c.g[V0][0], 0x8), c.g[A3][0]);
            c.g[V0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003a_3628);
                }
                continue;
            }
            break 'b_3a3660;
        }
    }
    'b_3a36b0: {
        c.cycles += 4;
        c.g[S0][0] = c.lw(spch_ea(c.g[S1][0], 0x74));
        c.g[V0][0] = c.g[S0][0] & 0x1;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x60);
            break 'b_3a36b0;
        }
        'b_3a36a8: {
            c.cycles += 12;
            c.g[S0][0] = sx((c.g[S0][0] as u32) >> 17);
            c.g[A0][0] = 0x30;
            c.mult(0, 0, 16, 4);
            c.g[A1][0] = 0x49_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40f8));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lo[0];
            c.g[S1][0] = sx(spch_ea(c.g[S1][0], 0x8c));
            call_ptr(c, 0x0031_7e30, 0x003a_3698)?;
            let t = c.g[S0][0] == 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_3a36a8;
            }
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 2;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x60);
    }
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cUIText_setUnicodeStringWrap(cUIText* self, const u16* str)`
/// (0x3a0d00): sets the wide string and wraps it. It asks the text's font
/// (`UIText_GetFont`, through the object's vtable slot +0x2c), and when there
/// is one, copies `str` (`USTR_copy`) into a fresh buffer of `len + 1`
/// halfwords (`operator_new_named`), breaking the copy into lines: each
/// glyph's advance (`Font_GetCharWidthW`, scaled by the object's +0x50 and
/// +0x60 floats) is summed, and a line break is taken at a space (0x20) or
/// at the width limit. The lines are stored with `UIText_SetWideStringInternal`
/// and the buffer is freed afterwards.
pub fn cui_text_set_unicode_string_wrap(c: &mut Cpu) {
    guest(c, cui_text_set_unicode_string_wrap_body);
}

fn cui_text_set_unicode_string_wrap_body(c: &mut Cpu) -> Flow {
    c.cycles += 24;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, S0, 0x80);
    frame.save(c, S3, 0x50);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S6, 0x20);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S1, 0x70);
    c.g[A1][0] = 1;
    frame.save(c, S2, 0x60);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    frame.save(c, S7, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x90);
    c.g[V0][0] = c.lw(spch_ea(c.g[S3][0], 0x8));
    c.g[A0][0] = c.lh(spch_ea(c.g[V0][0], 0x28));
    c.g[V1][0] = c.lw(spch_ea(c.g[V0][0], 0x2c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(spch_ea(c.g[S3][0], c.g[A0][0] as u32));
    call_ptr(c, t, 0x003a_0d4c)?;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x003a_04f0,
        f: ee::s13::f_003a04f0,
    }
    .call(c, 0x003a_0d54)?;
    c.g[S6][0] = c.g[V0][0];
    if c.g[S6][0] == 0 {
        frame.restore(c, S0, 0x80);
    } else {
        'b_3a0e44: {
            c.cycles += 19;
            c.g[A0][0] = c.g[S0][0];
            c.g[S4][0] = u64::MAX;
            call_ptr(c, 0x003d_bca0, 0x003a_0d6c)?;
            c.mtc1(0, 20);
            c.g[S5][0] = c.g[V0][0];
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4078));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[S1][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x003a_0d94)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x003d_be20, 0x003a_0da4)?;
            let t = (c.g[S5][0] as i64) <= 0;
            c.g[S7][0] = 0xd;
            if t {
                break 'b_3a0e44;
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
            loop {
                'b_3a0e38: {
                    'b_3a0de8: {
                        'b_3a0de4: {
                            c.cycles += 6;
                            c.g[V1][0] = c.lhu(spch_gp(c, 0x16b8));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            if t {
                                break 'b_3a0de4;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu(spch_ea(c.g[V0][0], 0x2));
                            let t = c.g[V0][0] != c.g[A0][0];
                            c.g[A0][0] = c.g[S6][0];
                            if t {
                                break 'b_3a0de8;
                            }
                            c.cycles += 4;
                            c.g[S4][0] = u64::MAX;
                            c.mtc1(0, 20);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                            break 'b_3a0e38;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S6][0];
                    }
                    'b_3a0e34: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x003a_0df0);
                        c.g[A1][0] = c.lhu(c.g[S0][0] as u32);
                        Callee {
                            addr: 0x0039_2c60,
                            f: ee::s13::f_00392c60,
                        }
                        .call(c, 0x003a_0df0)?;
                        c.lwc1(1, spch_ea(c.g[S3][0], 0x50));
                        c.lwc1(2, spch_ea(c.g[S3][0], 0x60));
                        c.mul_s(0, 0, 1);
                        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                        c.g[V0][0] ^= 0x20;
                        c.add_s(20, 20, 0);
                        c.c_lt_s(2, 20);
                        let t = !c.fcc();
                        if c.g[V0][0] == 0 {
                            c.g[S4][0] = c.g[S1][0];
                        }
                        if t {
                            break 'b_3a0e34;
                        }
                        c.cycles += 2;
                        let t = (c.g[S4][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[S4][0] as u32) << 1);
                        if t {
                            break 'b_3a0e34;
                        }
                        c.cycles += 5;
                        c.g[S1][0] = c.g[S4][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.mtc1(0, 20);
                        c.sh(c.g[V0][0] as u32, c.g[S7][0]);
                        c.g[S4][0] = u64::MAX;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_0db0);
                    }
                    continue;
                }
                break 'b_3a0e44;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x003a_0eb0,
            f: ee::s13::f_003a0eb0,
        }
        .call(c, 0x003a_0e50)?;
        let t = c.g[S2][0] == 0;
        frame.restore(c, S0, 0x80);
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_0e60);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7e98, 0x003a_0e60)?;
            frame.restore(c, S0, 0x80);
        }
    }
    c.cycles += 11;
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x90);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};

    /// `SPCH_AddBank`: the bank table, its count, the bank's bytes and the
    /// scan window are all fixture data.
    #[test]
    fn spch_add_bank_matches() {
        const TABLE: u32 = DATA + 0x1000;
        const BANK: u32 = DATA + 0x3000;
        check_lohi(
            ee::s14::f_003d69f0__generated,
            spch_add_bank,
            200,
            &|c, seed| {
                let mut r = Rng(0x3d69_f0 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x003d_69f0;
                c.g[A0][0] = sx(BANK);
                fill(c, &mut r, DATA + 0x3000, 0x2000);
                c.bus
                    .write8(BANK + 4, if seed % 3 == 0 { 0 } else { r.next_u32() as u8 });
                c.bus.write8(BANK + 5, r.next_u32() as u8);
                c.bus.write8(
                    BANK + 6,
                    if seed % 5 == 0 {
                        0
                    } else {
                        (r.next_u32() % 9) as u8
                    },
                );
                let count = r.next_u32() % 6;
                c.bus.write32(GAME_GP + 0x1734, TABLE);
                c.bus.write32(GAME_GP + 0x1738, count);
                c.bus.write32(GAME_GP + 0x1744, r.next_u32() & 0xff);
                for i in 0..8 {
                    let used = i < count && r.next_u32() % 4 != 0;
                    c.bus
                        .write32(TABLE + 4 * i, if used { 0x0080_8000 + i } else { 0 });
                }
            },
        );
    }

    /// `LocStringTable_ResolveString`: the forwarder runs the real
    /// `LocStringTable_GetStringPtr` on the table pointer at +4.
    #[test]
    fn loc_string_table_resolve_string_matches() {
        const LT: u32 = DATA + 0x6000;
        const TABLE: u32 = DATA + 0x6100;
        check(
            ee::s08::f_002c6dc0__generated,
            loc_string_table_resolve_string,
            200,
            &|c, seed| {
                let mut r = Rng(0x2c6d_c0 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x002c_6dc0;
                c.g[A0][0] = sx(LT);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(LT + 4, TABLE);
                fill(c, &mut r, TABLE, 0x40);
                c.bus.write8(TABLE + 8, (r.next_u32() & !1) as u8);
            },
        );
    }

    /// `UIAnim_ctor` with a desc of 0..=3 entries: the pointer array comes
    /// from the real `operator_new_named` (the test heap), and each entry
    /// is handed to the real `UIAnim_SetData`.
    #[test]
    fn ui_anim_ctor_matches() {
        const SELF: u32 = DATA + 0x7000;
        const DESC: u32 = DATA + 0x7100;
        check(
            ee::s13::f_003972a0__generated,
            ui_anim_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3972_a0 ^ (seed as u64) << 10);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x0039_72a0;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(DESC);
                fill(c, &mut r, DESC, 0x40);
                c.bus.write32(DESC + 4, seed % 4);
            },
        );
    }

    /// `cFEStateMainMenu_OnRender` with the screen lookup returning nothing
    /// (the zeroed object, `+0x3c == 0`), so the paths through the vtable
    /// calls are taken only when the lookup succeeds: this covers the
    /// lookup-miss paths and the mode-bit tests. The vtable slots point at
    /// the fake methods (the `miss` hook), so the call log is compared.
    #[test]
    fn cfe_state_main_menu_on_render_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const SELF: u32 = DATA + 0x8000;
        const OBJ: u32 = DATA + 0x8100;
        const VT: u32 = DATA + 0x8200;
        check(
            ee::s02::f_00194bf0__generated,
            cfe_state_main_menu_on_render,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_4bf0, 0);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x0019_4bf0;
                c.g[A0][0] = sx(SELF);
                for (i, off) in [0xc8u32, 0x40, 0x58].into_iter().enumerate() {
                    c.bus.write16(VT + off, 0);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
                }
                c.bus.write32(OBJ + 0x8, VT);
                c.bus.write32(OBJ + 0x3c, 0);
                c.bus.write32(SELF + 0x40, OBJ);
                c.bus.write8(SELF + 0x55, r.next_u32() as u8);
                c.bus.write8(SELF + 0x1d, r.next_u32() as u8);
                c.bus.write32(GAME_GP - 0x704, r.next_u32() & 0x3f);
            },
        );
    }

    /// `cUIVector_setUIData` on a zeroed-object base: the base load
    /// (`UIObject_Load`) is the real function, the element arrays come from
    /// the test heap, and `data->count` (0..=3) picks the loop and the
    /// allocation sizes; the flag word at +0x74 is kept small so the 48-byte
    /// block is taken for some seeds and skipped for others.
    #[test]
    fn cui_vector_set_ui_data_matches() {
        const SELF: u32 = DATA + 0x9000;
        const OBJ: u32 = DATA + 0x9200;
        check(
            ee::s13::f_003a34c0__generated,
            cui_vector_set_ui_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a34_c0 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x003a_34c0;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OBJ);
                fill(c, &mut r, SELF, 0x40);
                fill(c, &mut r, OBJ, 0x20);
                c.bus.write8(OBJ + 0x20, (seed % 4) as u8);
                c.bus.write32(SELF + 0x74, r.next_u32() & 0x0003_ffff);
            },
        );
    }

    /// `cUIText_setUnicodeStringWrap`: a short wide string (possibly with
    /// spaces) against a zeroed text object whose vtable's font method is a
    /// fake. The font lookup (`UIText_GetFont`) and the glyph advance are the
    /// real functions; the allocator is the test heap.
    #[test]
    fn cui_text_set_unicode_string_wrap_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const SELF: u32 = DATA + 0xa000;
        const VT: u32 = DATA + 0xa200;
        const STR: u32 = DATA + 0xa800;
        check_lohi(
            ee::s13::f_003a0d00__generated,
            cui_text_set_unicode_string_wrap,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_0d00, 0);
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x003a_0d00;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(STR);
                c.bus.write32(SELF + 0x8, VT);
                c.bus.write16(VT + 0x28, 0);
                c.bus.write32(VT + 0x2c, FAKE);
                c.bus.write32(SELF + 0x50, r.f32(2.0));
                c.bus.write32(SELF + 0x60, r.f32(2.0));
                c.bus.write16(GAME_GP + 0x16b8, 0x0a);
                let len = r.next_u32() % 20;
                for i in 0..len {
                    let ch = if r.next_u32() % 4 == 0 {
                        0x20
                    } else {
                        0x41 + (r.next_u32() % 26) as u16
                    };
                    c.bus.write16(STR + 2 * i, ch);
                }
                c.bus.write16(STR + 2 * len, 0);
            },
        );
    }
}
