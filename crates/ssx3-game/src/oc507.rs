//! Staging module oc507 (wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc507`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_b2b0, lobby_menu_init_abort_page, "LobbyMenu_InitAbortPage"),
    (0x001d_b330, lobby_menu_fill_delete_persona_page, "LobbyMenu_FillDeletePersonaPage"),
    (0x001d_b418, lobby_menu_fill_persona_pages, "LobbyMenu_FillPersonaPages"),
    (0x001d_b148, lobby_menu_init_challenge_page, "LobbyMenu_InitChallengePage"),
];

/// `LobbyMenu_InitAbortPage(page, arg)` (0x1db2b0): calls the page's base
/// constructor (0x2caa58), stores `0x46d378` at `page+0x12c` and the word at
/// `arg` at `page+0x130`, builds the abort item at `page+0x134` (0x2ccf98, with
/// the string at `0x467de0`) and adds it to the page's menu (0x2cbcc8, index
/// -1). Returns the page.
pub fn lobby_menu_init_abort_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x002c_aa58, 0x001d_b2d0)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x134));
        c.g[V0][0] = 0x470000;
        c.g[A2][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2c88));
        c.g[A3][0] = 0;
        let title = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x12c).sw(c, title);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7de0));
        c.g[A1][0] = 0;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let word = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x130).sw(c, word);
        call_ptr(c, 0x002c_cf98, 0x001d_b300)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b310)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_FillDeletePersonaPage(page, list)` (0x1db330): one 0x24-byte
/// item per persona of the list (count at `list+0x6c`, persona words at
/// `list+0x70`), from `page+0x168`. Each is activated (`MenuSysItem_SetVisible`,
/// flag 1), gets its persona word at `+0x17c` and the next index at `+0x170`;
/// the items from the count up to the fourth are deactivated (flag 0).
pub fn lobby_menu_fill_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S4][0] as u32).off(0x6c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S5][0] = 0x24;
            c.mult(0, V0, S2, S5);
            loop {
                c.cycles += 19;
                c.mult(0, S0, S2, S5);
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = 0x1;
                c.g[S2][0] = c.g[S1][0];
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x168));
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x70).lw(c);
                let word = c.g[V1][0];
                Addr(c.g[A2][0] as u32).off(0x17c).sw(c, word);
                call_ptr(c, 0x002c_a388, 0x001d_b3a0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                let next = c.g[S1][0];
                Addr(c.g[S0][0] as u32).off(0x170).sw(c, next);
                c.g[V0][0] = Addr(c.g[S4][0] as u32).off(0x6c).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let more = (c.g[S2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.mult(0, V0, S2, S5);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b370);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x24;
        if more {
            c.cycles += 4;
            c.mult(0, V0, S2, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x168));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x002c_a388, 0x001d_b3e4)?;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b3d8);
                }
            }
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

/// `LobbyMenu_FillPersonaPages(page, list)` (0x1db418): the select-persona
/// items at `page+0x518` (0x24 each, one per persona of the list, count at
/// `list+0x6c`, persona words at `list+0x70`) get their persona word at `+0x14`
/// and index at `+0x8`, and are activated; the items up to the fourth are
/// deactivated. Then the delete-persona page at `page+0x2e8` is filled
/// (`LobbyMenu_FillDeletePersonaPage`), and the two menu items at `page+0x5c4`
/// and `page+0x5a8` are shown when the count is 2 or more (the first) and 4 or
/// more (the second; their visibility flags are `count < 2 ^ 1` and `count < 4`).
pub fn lobby_menu_fill_persona_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x518));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x70));
            loop {
                c.cycles += 12;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                let word = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                Addr(c.g[S0][0] as u32).off(0x14).sw(c, word);
                call_ptr(c, 0x002c_a388, 0x001d_b470)?;
                let index = c.g[S1][0];
                Addr(c.g[S0][0] as u32).off(0x8).sw(c, index);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b458);
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2e8));
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c4));
        let more = c.g[V0][0] != 0;
        c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a8));
        if more {
            c.cycles += 5;
            c.g[V0][0] = 0x24;
            c.mult(0, V0, S1, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x518));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x002c_a388, 0x001d_b4bc)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b4b0);
                }
            }
        }
        c.cycles += 22;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x001d_b330,
            f: lobby_menu_fill_delete_persona_page,
        }
        .call(c, 0x001d_b4d8)?;
        c.g[A1][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = ((c.g[A1][0] as i64) < 2) as u64;
        c.g[A1][0] ^= 0x1;
        call_ptr(c, 0x002c_a388, 0x001d_b4ec)?;
        c.g[A1][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = ((c.g[A1][0] as i64) < 4) as u64;
        call_ptr(c, 0x002c_a388, 0x001d_b4fc)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_InitChallengePage(page, accept_owner, decline_owner)` (0x1db148):
/// the challenge page's base constructor (0x2caa58), its screen pointer
/// `0x46d3d0`-tagged title, the two owners stored at `+0x130` and `+0x134` (the
/// words they point to), then the six menu items (two label items, a spacer,
/// two command items for accept and decline) added to the page in order.
/// Returns the page.
pub fn lobby_menu_init_challenge_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 90;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x138));
        frame.save(c, S6, 0x20);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x154));
        frame.save(c, S7, 0x10);
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x170));
        frame.save_ra(c, 0);
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18c));
        call_ptr(c, 0x002c_aa58, 0x001d_b190)?;
        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
        c.g[V0][0] = 0x470000;
        c.g[A1][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2c30));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7da8));
        let screen = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x12c).sw(c, screen);
        c.g[A0][0] = c.g[S4][0];
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a4));
        let accept = c.g[V1][0];
        Addr(c.g[S0][0] as u32).off(0x130).sw(c, accept);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let decline = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x134).sw(c, decline);
        call_ptr(c, 0x002c_d2a0, 0x001d_b1c4)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7dc0));
        call_ptr(c, 0x002c_d2a0, 0x001d_b1d4)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7dc8));
        call_ptr(c, 0x002c_d2a0, 0x001d_b1e4)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_b1f0)?;
        c.g[A2][0] = 0x460000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7dd0));
        c.g[A1][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_b208)?;
        c.g[A2][0] = 0x460000;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S7][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7dd8));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002c_cf98, 0x001d_b220)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b230)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b240)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b250)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b260)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b270)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b280)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x70);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, fall_back, hook};
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate: the fakes below only log when this word is set in the machine's RAM.
    pub(crate) const OC507_GATE: u32 = DATA + 0x0020_0000;
    pub(crate) const OC507_MAGIC: u32 = 0x0507_0507;
    /// The fixture page and the argument word read by the function.
    pub(crate) const OC507_PAGE: u32 = DATA + 0x0020_1000;
    pub(crate) const OC507_ARG: u32 = DATA + 0x0020_2000;
    /// The fixture persona list read by the delete-persona fill.
    pub(crate) const OC507_OBJ: u32 = DATA + 0x0020_4000;

    /// A gated fake for a callee in the function table: logs the call with
    /// [`fake_call`] when armed, otherwise runs what the slot held before.
    macro_rules! oc507_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC507_GATE) != OC507_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                fake_call(c, $target);
            }
        };
    }

    oc507_fake!(
        oc507_fake_page_ctor,
        OC507_PREV_PAGE_CTOR,
        0x002c_aa58,
        ee::s09::f_002caa58__generated
    );
    oc507_fake!(
        oc507_fake_item_ctor,
        OC507_PREV_ITEM_CTOR,
        0x002c_cf98,
        ee::s09::f_002ccf98__generated
    );
    oc507_fake!(
        oc507_fake_activate,
        OC507_PREV_ACTIVATE,
        0x002c_a388,
        ee::s09::f_002ca388__generated
    );
    oc507_fake!(
        oc507_fake_menu_add,
        OC507_PREV_MENU_ADD,
        0x002c_bcc8,
        ee::s09::f_002cbcc8__generated
    );

    /// Install the fakes in their override slots (once per process).
    fn oc507_hook_all() {
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA58),
            oc507_fake_page_ctor,
            &OC507_PREV_PAGE_CTOR,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CCF98),
            oc507_fake_item_ctor,
            &OC507_PREV_ITEM_CTOR,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA388),
            oc507_fake_activate,
            &OC507_PREV_ACTIVATE,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBCC8),
            oc507_fake_menu_add,
            &OC507_PREV_MENU_ADD,
        );
    }

    /// Scrambled machine with the gate armed.
    fn oc507_arm(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        crate::difftest::fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC507_GATE, OC507_MAGIC);
    }

    #[test]
    fn lobby_menu_fill_delete_persona_page_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db330__generated,
            lobby_menu_fill_delete_persona_page,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db330, 6);
                c.pc = 0x001d_b330;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_OBJ);
                let count = [0u32, 1, 2, 3, 5, 9][(seed % 6) as usize];
                c.bus.write32(OC507_OBJ + 0x6c, count);
                for i in 0..10u32 {
                    c.bus.write32(
                        OC507_OBJ + 0x70 + 4 * i,
                        seed.wrapping_mul(0x2545_f491).rotate_left(i),
                    );
                }
            },
        );
    }

    #[test]
    fn lobby_menu_fill_persona_pages_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db418__generated,
            lobby_menu_fill_persona_pages,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db418, 12);
                c.pc = 0x001d_b418;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_OBJ);
                let count = [0u32, 1, 2, 3, 5, 9][(seed % 6) as usize];
                c.bus.write32(OC507_OBJ + 0x6c, count);
                for i in 0..10u32 {
                    c.bus.write32(
                        OC507_OBJ + 0x70 + 4 * i,
                        seed.wrapping_mul(0x9e37_79b9).rotate_left(i + 3),
                    );
                }
            },
        );
    }

    #[test]
    fn lobby_menu_init_challenge_page_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db148__generated,
            lobby_menu_init_challenge_page,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db148, 9);
                c.pc = 0x001d_b148;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_ARG);
                c.g[A2][0] = sx(OC507_OBJ);
                c.bus.write32(OC507_ARG, seed.wrapping_mul(0x2545_f491) ^ 0x31);
                c.bus.write32(OC507_OBJ, seed.wrapping_mul(0x9e37_79b9) ^ 0x77);
            },
        );
    }

    #[test]
    fn lobby_menu_init_abort_page_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db2b0__generated,
            lobby_menu_init_abort_page,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db2b0, 3);
                c.pc = 0x001d_b2b0;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_ARG);
                c.bus.write32(OC507_ARG, seed.wrapping_mul(0x9e37_79b9) ^ 0x55);
            },
        );
    }
}
