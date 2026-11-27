//! Staging module oc989 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc989`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_5a38,
        oc989_career_stats_show_page,
        "cFEStateCareerStats_ShowPage",
    ),
    (
        0x001f_7298,
        oc989_enter_lodge_shutdown,
        "OVScreenEnterLodge_Shutdown",
    ),
    (
        0x001f_75a0,
        oc989_bc_start_on_widget_action,
        "OVScreenBCStart_OnWidgetAction",
    ),
    (
        0x001f_7800,
        oc989_bc_fail_on_widget_action,
        "OVScreenBCFail_OnWidgetAction",
    ),
];

/// `base + off` as a 32-bit guest address (the `off(reg)` operand).
fn oc989_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `gp + off` (negative offsets wrap, as the assembler's `-0x69c(gp)` does).
fn oc989_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `void cFEStateBuyAttrib_UpdateMoney(cFEStateBuyAttrib* self)` (0x1f52a0):
/// the money text (hash of the string at 0x46f0a0, found under `self+0x48`)
/// gets the bank value `self+0xa4` as a string, if the object exists.
#[allow(dead_code)]
pub fn oc989_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f0a0);
        call_ptr(c, 0x0031_7670, 0x001f_52c0)?;
        c.g[A0][0] = Addr(oc989_at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_52cc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(oc989_at(c, S1, 0xa4)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_52e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_52ec)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_CommitPurchase(cFEStateBuyAttrib* self)`
/// (0x1f5300): spend the bank on the selected attribute. Charges the cost
/// (`self+0xa0`) to the bank (`self+0xa4`), clears the pending purchase,
/// then sums the purchased levels into the owned-level table (`self+0x84`,
/// eight words, each scaled by its index) and refreshes the level, cost,
/// bar and money texts. Calls with the generated argument and return-address
/// sequence throughout.
#[allow(dead_code)]
pub fn oc989_commit_purchase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x50);
        c.g[V0][0] = sx(oc989_at(c, S3, 0x4c));
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        c.g[S6][0] = sx(oc989_at(c, S3, 0x84));
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = c.g[S6][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x10);
        let spilled = c.g[V0][0];
        let slot = frame.at(c, 0);
        c.sw(slot, spilled);
        c.g[V0][0] = Addr(oc989_at(c, S3, 0xa4)).lw(c);
        c.g[V1][0] = Addr(oc989_at(c, S3, 0xa0)).lw(c);
        Addr(oc989_at(c, S3, 0xa0)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let bank = c.g[V0][0];
        Addr(oc989_at(c, S3, 0xa4)).sw(c, bank);
        call_ptr(c, 0x001f_5240, 0x001f_5358)?;
        Addr(oc989_at(c, S3, 0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(0x0046_f060);
        call_ptr(c, 0x0039_e8b8, 0x001f_536c)?;
        c.g[A2][0] = Addr(oc989_at(c, S3, 0xac)).lw(c);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(oc989_at(c, A2, 0x8)).lw(c);
        c.g[A0][0] = Addr(oc989_at(c, V0, 0x38)).lh(c);
        c.g[V1][0] = Addr(oc989_at(c, V0, 0x3c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_5388)?;
        call_ptr(c, 0x0014_dc80, 0x001f_5390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001f_539c)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V1][0] = 0;
        c.g[V0][0] = 0x480000;
        c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f50));
        c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
        'l_1f53b0: loop {
            'b_1f53fc: {
                c.cycles += 5;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if none {
                    break 'b_1f53fc;
                }
                c.cycles += 3;
                c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[S1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[A0][0] = 0;
                'l_1f53d0: loop {
                    c.cycles += 11;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x0014_a080, 0x001f_53d8)?;
                    c.g[A3][0] = Addr(oc989_at(c, S1, 0)).lw(c);
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0015_0c20, 0x001f_53ec)?;
                    c.g[V1][0] = Addr(oc989_at(c, S2, 0)).lw(c);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[A0][0] = 0;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f_53d0);
                        }
                        continue 'l_1f53d0;
                    }
                    break 'b_1f53fc;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S4][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_53b0);
                }
                continue 'l_1f53b0;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(oc989_at(c, S5, 0xc)).lw(c);
            c.g[A0][0] = Addr(oc989_at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(oc989_at(c, V0, 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001f_5420)?;
            c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = 6;
            'l_1f5430: loop {
                c.cycles += 9;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let sum = c.g[V1][0];
                Addr(c.g[A0][0] as u32).sw(c, sum);
                Addr(c.g[A1][0] as u32).sw(c, 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_5430);
                    }
                    continue 'l_1f5430;
                }
                c.cycles += 20;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_4ea8, 0x001f_545c)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_4fc0, 0x001f_5464)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_50b0, 0x001f_546c)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_52a0, 0x001f_5474)?;
                frame.restore(c, S0, 0xa0);
                frame.restore(c, S1, 0x90);
                frame.restore(c, S2, 0x80);
                frame.restore(c, S3, 0x70);
                frame.restore(c, S4, 0x60);
                frame.restore(c, S5, 0x50);
                frame.restore(c, S6, 0x40);
                frame.restore(c, S7, 0x30);
                frame.restore(c, FP, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void cFEStateCareerStats_Init(cFEStateCareerStats* self)` (0x1f5508):
/// creates the career-stats screen under the engine (`self+0x10`), plays its
/// first frame, clears the medal and page counters, counts the medals, shows
/// the first page and finishes the screen's setup with two calls.
#[allow(dead_code)]
pub fn oc989_career_stats_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_f168);
        c.g[S0][0] = Addr(oc989_at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_552c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_5540)?;
        let screen = c.g[V0][0];
        Addr(oc989_at(c, S1, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_5558)?;
        }
        c.cycles += 26;
        Addr(oc989_at(c, S1, 0x4c)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        for off in [0x50, 0x54, 0x58, 0x5c, 0x64, 0x60, 0x68, 0x70, 0x6c, 0x74] {
            Addr(oc989_at(c, S1, off)).sw(c, 0);
        }
        Addr(oc989_at(c, S1, 0x48)).sw(c, 0);
        call_ptr(c, 0x001f_6f30, 0x001f_5590)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_5a38, 0x001f_5598)?;
        call_ptr(c, 0x0028_b180, 0x001f_55a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, 0x0028_f140, 0x001f_55ac)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One visibility update in the show-page tail: the object named by `name`
/// (hashed) under the screen (`self+0x40`) gets a vtable call with the show
/// flag (`S4`). The caller has set `A0` to the generated base (0x470000),
/// which the name's address replaces, as the generated `sx(A0 - off)` does.
fn oc989_show_page_object(
    c: &mut Cpu,
    name: u32,
    hash_ret: u32,
    finder_ret: u32,
    vt_ret: u32,
) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = Addr(oc989_at(c, S0, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, finder_ret)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S4][0];
    if c.g[A2][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(oc989_at(c, A2, 0x8)).lw(c);
        c.g[A0][0] = Addr(oc989_at(c, V0, 0x48)).lh(c);
        c.g[V1][0] = Addr(oc989_at(c, V0, 0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, vt_ret)?;
    }
    Ok(())
}

/// The shared tail of the show-page branches that hide or show one object:
/// `cUIState_showObjSafe(self, A1)` (0x39e948), then `A0 = 0x470000`.
fn oc989_show_page_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_e948, 0x001f_5afc)?;
    c.g[A0][0] = 0x0047_0000;
    Ok(())
}

/// The mode-0 and mode-3 exit: the same object call, then the show flag is
/// cleared (`S4 = 0`).
fn oc989_show_page_l0_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_e948, 0x001f_5b0c)?;
    c.g[S4][0] = 0;
    c.g[A0][0] = 0x0047_0000;
    Ok(())
}

/// `void cFEStateCareerStats_ShowPage(cFEStateCareerStats* self)` (0x1f5a38):
/// hides the four page panels, shows the one for the page (`self+0x48`: 0
/// shows panel 0 and clears the show flag, 1 and 2 show panels 1 and 2, 3
/// shows panel 3 and clears the flag), then shows or hides the two page
/// labels (hash of 0x46f1f0 and 0x46f200) with the vtable's show method.
pub fn oc989_career_stats_show_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S5][0] = sx(0x0046_f1a8);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[A1][0] = c.g[S5][0];
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        c.g[S4][0] = 1;
        call_ptr(c, 0x0039_e8b8, 0x001f_5a70)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = sx(0x0046_f1b8);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0039_e8b8, 0x001f_5a84)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = sx(0x0046_f1c8);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e8b8, 0x001f_5a98)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx(0x0046_f1e0);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e8b8, 0x001f_5aac)?;
        c.g[V1][0] = Addr(oc989_at(c, S0, 0x48)).lw(c);
        let mode_is_one = c.g[V1][0] == c.g[S4][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        c.cycles += 2;
        if mode_is_one {
            c.g[A1][0] = c.g[S3][0];
            oc989_show_page_tail(c)?;
        } else if c.g[V0][0] != 0 {
            // Mode 0 or below (mode 1 was handled above).
            c.g[V0][0] = 2;
            c.cycles += 2;
            c.g[A1][0] = c.g[S5][0];
            if c.g[V1][0] == 0 {
                oc989_show_page_l0_tail(c)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0000;
            }
        } else {
            // Mode 2 or above.
            c.g[V0][0] = 2;
            c.cycles += 2;
            let is_two = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is_two {
                c.cycles += 1;
                c.g[A1][0] = c.g[S2][0];
                oc989_show_page_tail(c)?;
            } else {
                c.cycles += 2;
                let is_three = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                if is_three {
                    oc989_show_page_l0_tail(c)?;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x0047_0000;
                }
            }
        }
        c.cycles += 8;
        oc989_show_page_object(c, 0x0046_f1f0, 0x001f_5b1c, 0x001f_5b28, 0x001f_5b48)?;
        c.cycles += 9;
        c.g[A0][0] = 0x0047_0000;
        oc989_show_page_object(c, 0x0046_f200, 0x001f_5b54, 0x001f_5b60, 0x001f_5b80)?;
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

/// `void cOVScreenEnterLodge_Shutdown(cOVScreenEnterLodge* self)` (0x1f7298):
/// clears the overlay's `+0x50` word and runs the overlay state shutdown.
pub fn oc989_enter_lodge_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(oc989_at(c, V0, 0x50)).sw(c, 0);
        call_ptr(c, 0x0020_a430, 0x001f_72ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `[gp+0xce8]` buy-attrib object's table update of the start screen
/// (`0x530000` table of pointers indexed by the level `gp-0x69c`).
fn oc989_bc_start_table(c: &mut Cpu) {
    c.g[V0][0] = sx(0x0053_66e8);
    c.g[A2][0] = 0x0044_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = 3;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let a0 = c.g[A0][0];
    Addr(oc989_gp(c, -0x9f4)).sw(c, a0);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let a1 = c.g[A1][0];
    Addr(oc989_gp(c, -0x69c)).sw(c, a1);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let v1 = c.g[V1][0];
    Addr(oc989_gp(c, -0x6a0)).sw(c, v1);
}

/// The `I` block of the start screen's message switch: `[gp+0xce8]` gets the
/// attribute value, then one call (with a store on the zero path).
fn oc989_bc_start_i_part(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
    call_ptr(c, 0x0030_b8c0, 0x001f_75e4)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
        c.cycles += 4;
        c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
        let value = c.g[V0][0];
        Addr(oc989_at(c, A0, 0x2a0)).sw(c, value);
        call_ptr(c, 0x0030_b540, 0x001f_7608)?;
        c.g[A1][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    } else {
        c.cycles += 4;
        c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_b6f8, 0x001f_75f4)?;
        c.g[A1][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    }
    Ok(())
}

/// The `cycles += 19` tail of the start screen's message switch.
fn oc989_bc_start_g_tail(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
    let value = c.g[V0][0];
    Addr(oc989_at(c, A0, 0x2a0)).sw(c, value);
    call_ptr(c, 0x0030_b658, 0x001f_7668)?;
    c.g[A1][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    oc989_bc_start_table(c);
    Ok(())
}

/// The start screen's last block (the `cycles += 20` path): the table
/// update, then two calls.
fn oc989_bc_start_20(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    oc989_bc_start_table(c);
    call_ptr(c, 0x0028_b180, 0x001f_764c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0029_d6d0, 0x001f_7654)?;
    Ok(())
}

/// The message switch of `OVScreenBCStart::OnWidgetAction`. Returns true when
/// the function falls into its `cycles += 1` tail, false when it leaves at
/// once.
fn oc989_bc_start_switch(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_five = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 6;
    if !is_five {
        c.cycles += 2;
        if c.g[A2][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
            oc989_bc_start_g_tail(c)?;
            return Ok(true);
        }
        c.cycles += 2;
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(oc989_at(c, A1, 0x18)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
        oc989_bc_start_g_tail(c)?;
        return Ok(true);
    }
    oc989_bc_start_i_part(c)?;
    oc989_bc_start_20(c)?;
    Ok(false)
}

/// `void OVScreenBCStart_OnWidgetAction(OVScreenBCStart* self, item, int msg)`
/// (0x1f75a0): on a widget action for the first (5) or second (6) message,
/// records the chosen value in the start screen's object at `[gp+0xce8]`.
/// For `msg` 5 the item's `+0x18` word picks which of the two paths runs.
pub fn oc989_bc_start_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let fall = if c.g[A1][0] == 0 {
            true
        } else {
            oc989_bc_start_switch(c)?
        };
        if fall {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The table update of the fail screen (the `0x530000` pointer table).
fn oc989_bc_fail_table(c: &mut Cpu) {
    c.g[A0][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    c.g[A1][0] = sx(0x0053_66e8);
    c.g[A2][0] = sx(0x0044_28f0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    let level = c.g[A0][0];
    Addr(oc989_gp(c, -0x69c)).sw(c, level);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let v1 = c.g[V1][0];
    Addr(oc989_gp(c, -0x6a0)).sw(c, v1);
}

/// The `cycles += 20` block of the fail screen: two calls, the table update.
fn oc989_bc_fail_twenty(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
    call_ptr(c, 0x0030_b658, 0x001f_78fc)?;
    c.g[A0][0] = Addr(oc989_at(c, S0, 0x10)).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = sx(oc989_at(c, A0, 0x18));
    call_ptr(c, 0x0039_f190, 0x001f_790c)?;
    oc989_bc_fail_table(c);
    Ok(())
}

/// The `A2 == 6` block of the fail screen: a hash lookup and one object call.
fn oc989_bc_fail_h(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = sx(0x0046_e050);
    call_ptr(c, 0x0031_7670, 0x001f_78cc)?;
    c.g[A0][0] = Addr(oc989_at(c, S0, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_c778, 0x001f_78d8)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    let not_found = c.g[A1][0] != c.g[V0][0];
    c.g[A1][0] &= 0xffff;
    if not_found {
        c.cycles += 3;
        c.g[A0][0] = Addr(oc989_at(c, S0, 0x40)).lw(c);
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_c870, 0x001f_78f4)?;
    }
    Ok(())
}

/// The message switch of `OVScreenBCFail::OnWidgetAction`. Returns true when
/// the function falls into its `cycles += 1` tail, false when it leaves at
/// once.
fn oc989_bc_fail_switch(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_five = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 6;
    if !is_five {
        c.cycles += 2;
        c.g[A0][0] = 0x0047_0000;
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 2;
            return Ok(false);
        }
        oc989_bc_fail_h(c)?;
        oc989_bc_fail_twenty(c)?;
        return Ok(true);
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(oc989_at(c, A1, 0x18)).lw(c);
    c.g[V0][0] = 2;
    if c.g[A1][0] == 0 {
        c.cycles += 28;
        c.g[V0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
        c.g[V1][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        let value = c.g[V1][0];
        Addr(oc989_at(c, V0, 0x2a0)).sw(c, value);
        call_ptr(c, 0x0030_b6f8, 0x001f_7850)?;
        c.g[A0][0] = Addr(oc989_at(c, S0, 0x10)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx(oc989_at(c, A0, 0x18));
        call_ptr(c, 0x0039_f190, 0x001f_7860)?;
        oc989_bc_fail_table(c);
        call_ptr(c, 0x0028_b180, 0x001f_789c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0029_d6d0, 0x001f_78a4)?;
        return Ok(false);
    }
    c.cycles += 2;
    if c.g[A1][0] != c.g[V0][0] {
        oc989_bc_fail_twenty(c)?;
        return Ok(true);
    }
    c.cycles += 4;
    c.g[A0][0] = 0x1d;
    call_ptr(c, 0x0020_ab50, 0x001f_78bc)?;
    Ok(false)
}

/// `void OVScreenBCFail_OnWidgetAction(OVScreenBCFail* self, item, int msg)`
/// (0x1f7800): the fail screen's counterpart of the start screen's handler:
/// message 5 with the item's `+0x18` word picks the path, message 6 looks up
/// a widget by hash, and the rest of the work is the same table update.
pub fn oc989_bc_fail_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let fall = if c.g[A1][0] == 0 {
            true
        } else {
            oc989_bc_fail_switch(c)?
        };
        if fall {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{DATA, GAME_GP};
    use ssx3_ee as ee;

    /// Gate word of this module's allocation-request fake (written by the
    /// fixtures that reach the allocator; the rest of the suite falls back).
    const OC989_ALLOC_GATE: u32 = DATA + 0x9000;
    const OC989_ALLOC_MAGIC: u32 = 0x4f43_3938;
    static OC989_PREV_ALLOC: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// `EAFile_AllocRequest` (0x3de670): with the gate armed it returns a null
    /// request at once, which ends the allocator spin on zeroed test data.
    fn oc989_fake_alloc_request(c: &mut Cpu) {
        if c.bus.read32(OC989_ALLOC_GATE) != OC989_ALLOC_MAGIC {
            return fakes::fall_back(c, &OC989_PREV_ALLOC, ee::s14::f_003de670__generated);
        }
        c.g[V0][0] = 0;
        crate::abi::ret(c);
    }

    fn oc989_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            oc989_fake_alloc_request,
            &OC989_PREV_ALLOC,
        );
    }

    /// `GetHashValue32` of the NUL-terminated string at `addr` in the loaded
    /// game image (the setup has to run `elf_image` first).
    fn hash_at(c: &mut Cpu, addr: u32) -> u32 {
        let mut s = Vec::new();
        let mut a = addr;
        loop {
            let b = Addr(a).read_u8(c);
            if b == 0 {
                break;
            }
            s.push(b);
            a += 1;
        }
        crate::game::hashvalue::str_hash(&s)
    }

    #[test]
    fn oc989_career_stats_show_page_matches() {
        check(
            ee::s04::f_001f5a38__generated,
            oc989_career_stats_show_page,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_5a38, 0);
                elf_image(c);
                let this = DATA + 0x100;
                let mode = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 5,
                    _ => u32::MAX,
                };
                c.bus.write32(this + 0x48, mode);
                let screen = DATA + 0x500;
                c.bus.write32(this + 0x40, screen);
                c.bus.write32(screen + 0x38, DATA + 0x600);
                c.bus.write32(DATA + 0x600, 2);
                c.bus.write32(screen + 0x3c, DATA + 0x610);
                let (n1, n2) = (DATA + 0x620, DATA + 0x640);
                c.bus.write32(DATA + 0x610, n1);
                c.bus.write32(DATA + 0x614, n2);
                let h1 = hash_at(c, 0x0046_f1f0);
                let h2 = hash_at(c, 0x0046_f200);
                c.bus.write32(n1 + 0x38, h1);
                c.bus.write32(n1 + 0x8, DATA + 0x660);
                c.bus.write32(n2 + 0x38, h2);
                c.bus.write32(n2 + 0x8, DATA + 0x660);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc989_enter_lodge_shutdown_matches() {
        check(
            ee::s04::f_001f7298__generated,
            oc989_enter_lodge_shutdown,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_7298, 0);
                elf_image(c);
                c.bus.write32(DATA + 0x10, seed);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn oc989_bc_start_on_widget_action_matches() {
        check(
            ee::s04::f_001f75a0__generated,
            oc989_bc_start_on_widget_action,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_75a0, 0);
                elf_image(c);
                let this = DATA + 0x100;
                let item = DATA + 0x200;
                c.bus.write32(item + 0x18, seed % 2);
                c.bus.write32(this + 0x9c, seed);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x800);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), 2 + seed % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn oc989_bc_fail_on_widget_action_matches() {
        check(
            ee::s04::f_001f7800__generated,
            oc989_bc_fail_on_widget_action,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_7800, 0);
                elf_image(c);
                let this = DATA + 0x100;
                let item = DATA + 0x200;
                c.bus.write32(item + 0x18, seed % 3);
                c.bus.write32(this + 0x9c, seed);
                c.bus.write32(this + 0x40, DATA + 0x500);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x800);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), 2 + seed % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
            },
        );
    }
}
