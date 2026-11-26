//! Staging module oc927 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc927`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001c_5430, cfe_state_net_news_v31, "cFEStateNetNews_v31")];

/// `cFEStateNetNews_v31` (0x001c5430): the news panel's message handler.
/// Message 226 (a news item arrived) copies the item's text into a string,
/// turns each newline into a space (a blank line keeps two carriage returns),
/// adds the result to the news text scroller and destroys the copy. Other
/// messages go to `NetFEState_HandleNetEvent`. The scroller loop checks events
/// at its back edge; an unwound event returns with `pc` at the loop head, and
/// the generated body resumes there (the entry guard below).
pub fn cfe_state_net_news_v31(c: &mut Cpu) {
    if c.pc != 0x001c_5430 {
        return ee::s03::f_001c5430__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xe2;
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x20);
        let other = c.g[A1][0] != c.g[V0][0];
        frame.save_ra(c, 0x10);

        if other {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            Callee {
                addr: 0x001a_8e40,
                f: ee::s03::NetFEState_HandleNetEvent,
            }
            .call(c, 0x001c_5514)?;
            frame.restore(c, 16, 0x50);
        } else {
            c.cycles += 9;
            c.g[A0][0] = 0x0046_5f38;
            c.g[RA][0] = sx(0x001c_5464);
            call_ptr(c, 0x0031_7670, 0x001c_5464)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_5470)?;
            c.g[S2][0] = c.g[V0][0];
            let no_text = c.g[S2][0] == 0;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
            if no_text {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                news_add_text(c)?;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x50);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The text part of `cFEStateNetNews_v31` (generated blocks 0x1c547c to
/// 0x1c54e8): copy the item's text (a1 = its string) into the local string at
/// sp, rewrite its newlines, add it to the scroller in s2 and destroy the copy.
/// Leaves v0 = 0.
fn news_add_text(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7fe8, 0x001c_5484)?;
    c.g[S3][0] = u64::MAX;
    c.g[S1][0] = 0xa;
    // The block at 0x1c54cc (a1 = '\n') runs before the first search and after
    // a single newline.
    let mut from_cc = true;
    loop {
        if from_cc {
            c.cycles += 1;
            c.g[A1][0] = 0xa;
        }
        // Find the next newline from the start of the string.
        c.cycles += 6;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_8c18, 0x001c_54dc)?;
        c.g[A1][0] = c.g[V0][0];
        let found = c.g[A1][0] != c.g[S3][0];
        c.g[V0][0] = Addr(sp(c)).lw(c);
        if !found {
            break;
        }
        // The back edge: an event may unwind here, with pc at the loop head.
        if back_edge_unwinds(c, 0x001c_5490) {
            return Err(Unwound);
        }

        // The character after the newline decides the rewrite.
        c.cycles += 5;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
        let not_double = c.g[V1][0] != c.g[S1][0];
        c.g[A0][0] = c.g[SP][0];
        if not_double {
            // A single newline becomes a space.
            c.cycles += 2;
            c.g[A2][0] = 0x20;
            call_ptr(c, 0x0031_8e68, 0x001c_54cc)?;
            from_cc = true;
        } else {
            // A blank line becomes two carriage returns (and skips 0x1c54cc).
            c.cycles += 8;
            c.g[A2][0] = 0xd;
            call_ptr(c, 0x0031_8e68, 0x001c_54ac)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = 0xd;
            call_ptr(c, 0x0031_8e68, 0x001c_54bc)?;
            c.g[A1][0] = 0xa;
            from_cc = false;
        }
    }

    c.cycles += 8;
    c.g[A1][0] = Addr(sp(c)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x003a_19f8, 0x001c_54f4)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x0031_8260, 0x001c_5500)?;
    c.g[V0][0] = 0;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of the news fakes: they act only while the magic is in RAM.
    const NEWS_CTRL: u32 = DATA + 0x20_0300;
    const NEWS_MAGIC: u32 = 0x0927_5eed;
    /// The news panel, its event (`data`; the text is at +8), the item's
    /// string and the text scroller's object.
    const NEWS_SELF: u32 = DATA + 0x20_0400;
    const NEWS_EVENT: u32 = DATA + 0x20_0600;
    const NEWS_STR: u32 = DATA + 0x20_0700;
    const NEWS_TEXT: u32 = DATA + 0x20_0800;

    fn news_gate(c: &mut Cpu) -> bool {
        c.bus.read32(NEWS_CTRL) == NEWS_MAGIC
    }

    /// `cUIScreen_getObjectByHashName` (0x39d860): the scroller for seeds
    /// that have one (word +4 of the gate: 1), else null.
    fn oc927_news_object(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_object),
                ee::s13::f_0039d860__generated,
            );
        }
        let has_text = c.bus.read32(NEWS_CTRL + 4) != 0;
        fakes::fake_call(c, 0x0039_d860);
        c.g[V0][0] = if has_text { NEWS_TEXT as u64 } else { 0 };
    }

    /// `GetHashValue32` (0x317670): the plain fake.
    fn oc927_news_hash(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_hash),
                ee::s10::f_00317670__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7670);
    }

    /// `cBXString` copy constructor (0x317fe8): copies the source's string
    /// pointer (the strings are shared; setAt writes through them).
    fn oc927_news_copy(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_copy),
                ee::s10::f_00317fe8__generated,
            );
        }
        let obj = c.g[A0][0] as u32;
        let src = c.g[A1][0] as u32;
        let s = c.bus.read32(src);
        c.bus.write32(obj, s);
        fakes::fake_call(c, 0x0031_7fe8);
    }

    /// `cBXString::find` (0x318c18): the index of byte a1 from a2 in the
    /// NUL-terminated string at word 0 of the object, or -1.
    fn oc927_news_find(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_find),
                ee::s10::f_00318c18__generated,
            );
        }
        let obj = c.g[A0][0] as u32;
        let ch = c.g[A1][0] as u8;
        let from = c.g[A2][0] as u32;
        let s = c.bus.read32(obj);
        let mut found = u64::MAX;
        for i in from..from + 64 {
            let b = c.bus.read8(s + i);
            if b == 0 {
                break;
            }
            if b == ch {
                found = i as u64;
                break;
            }
        }
        fakes::fake_call(c, 0x0031_8c18);
        c.g[V0][0] = found;
    }

    /// `cBXString::setAt` (0x318e68): stores a2 at index a1 of the string.
    fn oc927_news_set_at(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_set_at),
                ee::s10::f_00318e68__generated,
            );
        }
        let obj = c.g[A0][0] as u32;
        let idx = c.g[A1][0] as u32;
        let ch = c.g[A2][0] as u8;
        let s = c.bus.read32(obj);
        c.bus.write8(s.wrapping_add(idx), ch);
        fakes::fake_call(c, 0x0031_8e68);
    }

    /// `cBXString::~cBXString` (0x318260): the plain fake.
    fn oc927_news_dtor(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_dtor),
                ee::s10::f_00318260__generated,
            );
        }
        fakes::fake_call(c, 0x0031_8260);
    }

    /// `NetFEState_HandleNetEvent` (0x1a8e40): the plain fake.
    fn oc927_news_handle(c: &mut Cpu) {
        if !news_gate(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc927_news_handle),
                ee::s03::f_001a8e40__generated,
            );
        }
        fakes::fake_call(c, 0x001a_8e40);
    }

    fn oc927_news_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc927_news_object,
            fakes::prev_cell(oc927_news_object),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc927_news_hash,
            fakes::prev_cell(oc927_news_hash),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317FE8),
            oc927_news_copy,
            fakes::prev_cell(oc927_news_copy),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318C18),
            oc927_news_find,
            fakes::prev_cell(oc927_news_find),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318E68),
            oc927_news_set_at,
            fakes::prev_cell(oc927_news_set_at),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc927_news_dtor,
            fakes::prev_cell(oc927_news_dtor),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8E40),
            oc927_news_handle,
            fakes::prev_cell(oc927_news_handle),
        );
    }

    /// The news panel for `seed`: message 226 (every fourth seed another),
    /// a scroller for four seeds in five, and a 48-character text of newlines
    /// and letters (single and doubled newlines both occur).
    fn news_fixture(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.bus.write32(NEWS_CTRL, NEWS_MAGIC);
        c.bus.write32(NEWS_CTRL + 4, (seed % 5 != 0) as u32);
        c.bus.write32(NEWS_EVENT + 8, NEWS_STR);
        const ALPHABET: [u8; 5] = [b'\n', b'\n', b'x', b' ', b'\n'];
        for i in 0..48u32 {
            let ch = ALPHABET[(r.next_u32() % 5) as usize];
            c.bus.write8(NEWS_STR + i, ch);
        }
        c.bus.write8(NEWS_STR + 48, 0);
        c.g[A0][0] = sx(NEWS_SELF);
        c.g[A1][0] = if seed % 4 == 0 { 0xe1 } else { 0xe2 };
        c.g[A2][0] = sx(NEWS_EVENT);
        c.pc = 0x001c_5430;
    }

    #[test]
    fn cfe_state_net_news_v31_matches() {
        oc927_news_install();
        check(
            ee::s03::f_001c5430__generated,
            cfe_state_net_news_v31,
            300,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x927_0003, 6);
                news_fixture(c, seed, &mut r);
            },
        );
    }
}
