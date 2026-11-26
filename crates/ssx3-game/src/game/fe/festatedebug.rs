//! `src/fe/festatedebug.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_1090,
        fe_state_char_select_ctor,
        "cFEStateCharSelect_ctor",
    ),
    (
        0x0018_0b60,
        keyboard_screen_init_keys,
        "KeyboardScreen_InitKeys",
    ),
    (
        0x0018_0ef0,
        keyboard_screen_handle_input,
        "KeyboardScreen_HandleInput",
    ),
];

// ---------------------------------------------------------------------------
// FE state: character select.

/// `cFEStateCharSelect* cFEStateCharSelect_ctor(cFEStateCharSelect* s, void*
/// screen, int count)` (0x181090): install the character-select screen and
/// pre-fill every rider slot it owns.
pub fn fe_state_char_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S1, 0x40);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            c.g[A2][0] = sx((r(c, A2) as u32) << 24);
            frame.save(c, S2, 0x30);
            frame.save(c, S0, 0x50);
            c.g[S2][0] = sx((r(c, A2) as i32 >> 24) as u32);
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S4][0] = 0xff;
            call_ptr(c, 0x0039_e2a0, 0x0018_10c4)?;

            c.g[V0][0] = sx(0x0046_d000);
            c.g[A0][0] = 9;
            c.g[V1][0] = 1;
            Addr(p(r(c, S1), 8)).sw(c, c.g[V0][0]);
            Addr(p(r(c, S1), 0xc)).sw(c, c.g[A0][0]);
            Addr(p(r(c, S1), 0x54)).sw(c, c.g[V1][0]);
            Addr(p(r(c, S1), 0x44)).sb(c, c.g[S2][0]);
            Addr(p(r(c, S1), 0x48)).sw(c, 0);
            Addr(p(r(c, S1), 0x4c)).sw(c, 0);
            c.g[V0][0] = Addr(p(r(c, S1), 0x10)).lw(c);
            let none = (c.g[S2][0] as i64) <= 0;
            c.g[S3][0] = Addr(r(c, V0)).lw(c);
            if none {
                break 'outer;
            }
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx((r(c, S0) as u32) << 24);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = sx((r(c, A1) as i32 >> 24) as u32);
                c.g[RA][0] = sx(0x0018_110c);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                call_ptr(c, 0x001a_1cd0, 0x0018_110c)?;
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S4][0] = c.g[S4][0] & c.g[V0][0];
                if !more {
                    break 'outer;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0018_10f8);
                }
            }
        }
        c.cycles += 10;
        Addr(p(r(c, S1), 0x15)).sb(c, c.g[S4][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_InitKeys(KeyboardScreen* self, UIScreen* screen)`
/// (0x180b60): set up the on-screen keyboard objects: the arrow and END keys
/// get their texts, the letter keys `A..Z` and digit keys `0..9` get their
/// character, every cursor object is hidden, the `A` cursor is shown, and
/// the text field is cleared and bound to `self+0xec`. Each object is looked
/// up by `StrHash(name)` with `UIScreen_FindObject`.
pub fn keyboard_screen_init_keys(c: &mut Cpu) {
    guest(c, |c| {
        'b_180bcc: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S4][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[RA][0] = sx(0x00180b90);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a90));
            call_ptr(c, 0x00317670, 0x00180b90)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180b9c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180b9c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_180bcc;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180bbc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180bbc)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00180bcc);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1370));
                call_ptr(c, 0x003a0c50, 0x00180bcc)?;
                break 'b_180bcc;
            }
        }
        'b_180c14: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180bd8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a78));
            call_ptr(c, 0x00317670, 0x00180bd8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180be4);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180be4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_180c14;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180c04);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180c04)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00180c14);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1378));
                call_ptr(c, 0x003a0c50, 0x00180c14)?;
                break 'b_180c14;
            }
        }
        'b_180c5c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180c20);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a60));
            call_ptr(c, 0x00317670, 0x00180c20)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180c2c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180c2c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_180c5c;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180c4c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180c4c)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00180c5c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1380));
                call_ptr(c, 0x003a0c50, 0x00180c5c)?;
                break 'b_180c5c;
            }
        }
        'b_180c94: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180c68);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a50));
            call_ptr(c, 0x00317670, 0x00180c68)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180c74);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180c74)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180c94;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180c94);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180c94)?;
                break 'b_180c94;
            }
        }
        'b_180ccc: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180ca0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a40));
            call_ptr(c, 0x00317670, 0x00180ca0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180cac);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180cac)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180ccc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180ccc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180ccc)?;
                break 'b_180ccc;
            }
        }
        'b_180d04: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180cd8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a30));
            call_ptr(c, 0x00317670, 0x00180cd8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180ce4);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180ce4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180d04;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180d04);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180d04)?;
                break 'b_180d04;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x41;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x21), 0u64);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[S3][0] = 0u64;
        'l_180d18: loop {
            'b_180d54: {
                c.cycles += 13;
                c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xa));
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00180d2c);
                c.g[A0][0] = c.g[S4][0];
                Callee {
                    addr: 0x001809b0,
                    f: ee::s02::KeyboardScreen_GetKeyObjectName,
                }
                .call(c, 0x00180d2c)?;
                c.g[RA][0] = sx(0x00180d34);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00180d34)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00180d40);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039d860, 0x00180d40)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_180d54;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00180d54);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x003a0c50, 0x00180d54)?;
                    break 'b_180d54;
                }
            }
            'b_180d98: {
                c.cycles += 12;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00180d64);
                c.g[A0][0] = c.g[S4][0];
                Callee {
                    addr: 0x00180a90,
                    f: ee::s02::KeyboardScreen_GetCursorObjectName,
                }
                .call(c, 0x00180d64)?;
                c.g[RA][0] = sx(0x00180d6c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00180d6c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00180d78);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039d860, 0x00180d78)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_180d98;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00180d98);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00180d98)?;
                    break 'b_180d98;
                }
            }
            c.cycles += 6;
            c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = ((c.g[S3][0] as i64) < 26i64) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            let t = c.g[V1][0] != 0u64;
            c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00180d18);
                }
                continue 'l_180d18;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x30;
                c.g[S1][0] = 0u64;
                c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                'l_180dc0: loop {
                    'b_180df8: {
                        c.cycles += 12;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00180dd0);
                        c.g[A0][0] = c.g[S4][0];
                        Callee {
                            addr: 0x001809b0,
                            f: ee::s02::KeyboardScreen_GetKeyObjectName,
                        }
                        .call(c, 0x00180dd0)?;
                        c.g[RA][0] = sx(0x00180dd8);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00180dd8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00180de4);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x00180de4)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_180df8;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x00180df8);
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            call_ptr(c, 0x003a0c50, 0x00180df8)?;
                            break 'b_180df8;
                        }
                    }
                    'b_180e3c: {
                        c.cycles += 12;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00180e08);
                        c.g[A0][0] = c.g[S4][0];
                        Callee {
                            addr: 0x00180a90,
                            f: ee::s02::KeyboardScreen_GetCursorObjectName,
                        }
                        .call(c, 0x00180e08)?;
                        c.g[RA][0] = sx(0x00180e10);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00180e10)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00180e1c);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x00180e1c)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_180e3c;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00180e3c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00180e3c)?;
                            break 'b_180e3c;
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[S1][0] as i64) < 10i64) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = c.g[V1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00180dc0);
                        }
                        continue 'l_180dc0;
                    } else {
                        'b_180e94: {
                            c.cycles += 9;
                            c.g[A0][0] = 0x460000;
                            c.g[RA][0] = sx(0x00180e60);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a20));
                            call_ptr(c, 0x00317670, 0x00180e60)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00180e6c);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0039d860, 0x00180e6c)?;
                            c.g[S0][0] = c.g[V0][0];
                            if c.g[S0][0] == 0u64 {
                                c.g[A1][0] = 0x4a0000;
                                break 'b_180e94;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00180e90);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00180e90)?;
                                c.g[A1][0] = 0x4a0000;
                                break 'b_180e94;
                            }
                        }
                        'b_180ecc: {
                            c.cycles += 12;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[RA][0] = sx(0x00180ea0);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1388));
                            Callee {
                                addr: 0x001805d8,
                                f: ee::s02::KeyboardScreen_SetText,
                            }
                            .call(c, 0x00180ea0)?;
                            c.g[A0][0] = 0x460000;
                            c.g[RA][0] = sx(0x00180eac);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a10));
                            call_ptr(c, 0x00317670, 0x00180eac)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00180eb8);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00180eb8)?;
                            c.g[S0][0] = c.g[V0][0];
                            let t = c.g[S0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_180ecc;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x00180ecc);
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xec));
                                call_ptr(c, 0x003a0c50, 0x00180ecc)?;
                                break 'b_180ecc;
                            }
                        }
                        c.cycles += 8;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `int KeyboardScreen_HandleInput(KeyboardScreen* self, FEUIInput* in)`
/// (0x180ef0): hide the cursor object of the current key, act on the first
/// pressed button (left/right/up/down move the cursor, next types the key,
/// shift toggles caps), show the cursor object of the new key. Returns 1.
pub fn keyboard_screen_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_180f48: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = c.g[SP][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[RA][0] = sx(0x00180f14);
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
            Callee {
                addr: 0x00180a90,
                f: ee::s02::KeyboardScreen_GetCursorObjectName,
            }
            .call(c, 0x00180f14)?;
            c.g[RA][0] = sx(0x00180f1c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00317670, 0x00180f1c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180f28);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0039d860, 0x00180f28)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180f48;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180f48);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180f48)?;
                break 'b_180f48;
            }
        }
        'b_181038: {
            'b_180f74: {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180f5c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180f5c)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180f74;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180f6c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180618,
                        f: ee::s02::KeyboardScreen_CursorLeft,
                    }
                    .call(c, 0x00180f6c)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180f74;
                    }
                }
            }
            'b_180f9c: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180f84);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180f84)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180f9c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180f94);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180678,
                        f: ee::s02::KeyboardScreen_CursorRight,
                    }
                    .call(c, 0x00180f94)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180f9c;
                    }
                }
            }
            'b_180fc4: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180fac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180fac)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180fc4;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180fbc);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001806d8,
                        f: ee::s02::KeyboardScreen_CursorUp,
                    }
                    .call(c, 0x00180fbc)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180fc4;
                    }
                }
            }
            'b_180fec: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180fd4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180fd4)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180fec;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180fe4);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180768,
                        f: ee::s02::KeyboardScreen_CursorDown,
                    }
                    .call(c, 0x00180fe4)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180fec;
                    }
                }
            }
            'b_181014: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180ffc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180ffc)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_181014;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0018100c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180840,
                        f: ee::s02::KeyboardScreen_TypeKey,
                    }
                    .call(c, 0x0018100c)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_181014;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00181024);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00181024)?;
            if c.g[V0][0] == 0u64 {
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                break 'b_181038;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00181034);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x001807c8,
                    f: ee::s02::KeyboardScreen_ToggleCaps,
                }
                .call(c, 0x00181034)?;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                break 'b_181038;
            }
        }
        'b_181078: {
            c.cycles += 11;
            c.g[A2][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x00181044);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x00180a90,
                f: ee::s02::KeyboardScreen_GetCursorObjectName,
            }
            .call(c, 0x00181044)?;
            c.g[RA][0] = sx(0x0018104c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00317670, 0x0018104c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00181058);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00181058)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_181078;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00181078);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00181078)?;
                break 'b_181078;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;

    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    #[test]
    fn fe_state_char_select_ctor_matches() {
        check(
            ee::s02::f_00181090__generated,
            fe_state_char_select_ctor,
            60,
            &|c, seed| {
                base(c, seed, 0x1810, 1);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 4) as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
            },
        );
    }

    pub(crate) use crate::game::fe::debugfe::tests::oc826_keyboard;

    pub(crate) use crate::game::fe::festatecharselect::tests::oc826_force;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc826_machine;

    pub(crate) use crate::game::fe::festatecharselect::tests::oc826_strict;

    pub(crate) use crate::game::fe::festatecharselect::tests::OC826_OBJA;
    pub(crate) use crate::game::fe::festatecharselect::tests::OC826_OBJB;

    pub(crate) use crate::game::fe::festatecharselect::tests::OC826_SELF;
    pub(crate) use crate::game::fe::festatecharselect::tests::OC826_STR;

    /// `KeyboardScreen_InitKeys`: the objects found or not (random results of
    /// `UIScreen_FindObject`), every unwind point.
    #[test]
    fn keyboard_screen_init_keys_matches() {
        oc826_strict(
            ee::s02::f_00180b60__generated,
            keyboard_screen_init_keys,
            40,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_0b60, 330);
                oc826_keyboard(c, &mut r, 0);
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = sx(OC826_STR);
            },
        );
    }

    /// `KeyboardScreen_HandleInput`: the cursor object found or not, and
    /// each of the six buttons being the first one pressed (or none).
    #[test]
    fn keyboard_screen_handle_input_matches() {
        oc826_strict(
            ee::s02::f_00180ef0__generated,
            keyboard_screen_handle_input,
            84,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_0ef0, 16);
                let key = r.next_u32() % 44;
                oc826_keyboard(c, &mut r, key);
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = sx(OC826_OBJB);
                // Calls: 0 cursor name, 1 hash, 2 find, [3 hide], then one
                // button test each: pressed is the first nonzero result.
                let found = seed % 5 != 0;
                oc826_force(c, 2, found.then_some(OC826_OBJA));
                let base = if found { 4 } else { 3 };
                let pressed = seed % 7;
                for j in 0..6 {
                    let v = (j == pressed).then_some(OC826_OBJA);
                    if j <= pressed {
                        oc826_force(c, base + j, v);
                    }
                }
            },
        );
    }
}
