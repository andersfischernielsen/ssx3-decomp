//! Staging module oc931 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc931`.

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
    0x001d_0bc8,
    c_fe_state_reward_gallery_update_page_text,
    "cFEStateRewardGallery_UpdatePageText",
)];

/// `cFEStateRewardGallery_UpdatePageText` (0x001d0bc8): puts "page n of m" into the
/// gallery's page text widget (format string from the screen's text object,
/// `per = self+0x58`, `pages = ceil(self+0x60 / per)`), and shows or hides the
/// two page arrows (`pages > 1`). Without a format string it sets the plain
/// fallback text.
pub fn c_fe_state_reward_gallery_update_page_text(c: &mut Cpu) {
    if c.pc != 0x001d0bc8 {
        return ee::s04::f_001d0bc8__generated(c);
    }
    guest(c, |c| {
        'b_1d0d18: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x904));
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            if t {
                break 'b_1d0d18;
            } else {
                'b_1d0d04: {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_1d0d04;
                    } else {
                        c.cycles += 12;
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7060));
                        c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001d0c18);
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x001d0c18)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A1][0] = c.g[V0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001d0c28);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, t, 0x001d0c28)?;
                        c.g[A1][0] = c.g[V0][0];
                        let t = c.g[A1][0] == 0u64;
                        c.g[A0][0] = c.g[SP][0];
                        if t {
                            break 'b_1d0d04;
                        } else {
                            'b_1d0c44: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                                if c.g[V0][0] == 0u64 {
                                    break 'b_1d0c44;
                                } else {
                                    break 'b_1d0c44;
                                }
                            }
                            'b_1d0cbc: {
                                c.cycles += 24;
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                c.div(0, 3, 2);
                                c.g[A2][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                c.g[S0][0] = c.lo[0];
                                c.div(0, 6, 2);
                                c.g[A3][0] = c.g[S0][0];
                                c.g[A2][0] = c.lo[0];
                                c.g[RA][0] = sx(0x001d0c74);
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                call_ptr(c, 0x002c26d0, 0x001d0c74)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x904));
                                c.g[RA][0] = sx(0x001d0c80);
                                c.g[A1][0] = c.g[SP][0];
                                call_ptr(c, 0x003a0e90, 0x001d0c80)?;
                                c.g[A0][0] = 0x4a0000;
                                c.g[RA][0] = sx(0x001d0c8c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ef8));
                                call_ptr(c, 0x00317670, 0x001d0c8c)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001d0c98);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001d0c98)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                if t {
                                    break 'b_1d0cbc;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = c.g[A1][0] ^ 0x1;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001d0cbc);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001d0cbc)?;
                                    break 'b_1d0cbc;
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = 0x4a0000;
                            c.g[RA][0] = sx(0x001d0cc8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f00));
                            call_ptr(c, 0x00317670, 0x001d0cc8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001d0cd4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001d0cd4)?;
                            c.g[A2][0] = c.g[V0][0];
                            if c.g[A2][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                                break 'b_1d0d18;
                            } else {
                                c.cycles += 9;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                c.g[A1][0] = c.g[A1][0] ^ 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001d0cfc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001d0cfc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                                if t {
                                    break 'b_1d0d18;
                                } else {
                                    break 'b_1d0d04;
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x904));
                c.g[RA][0] = sx(0x001d0d14);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                call_ptr(c, 0x003a0c50, 0x001d0d14)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                break 'b_1d0d18;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check_lohi, fresh, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) const OC931_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC931_GATE: u32 = OC931_CTRL + 4;
    pub(crate) const OC931_FOUND: u32 = OC931_CTRL + 8;
    pub(crate) const OC931_SEEN: u32 = OC931_CTRL + 12;
    pub(crate) const OC931_MAGIC: u32 = 0x4f43_3933;
    pub(crate) const OC931_SELF: u32 = DATA + 0x1_0000;
    pub(crate) const OC931_SCREEN: u32 = DATA + 0x2_0000;
    pub(crate) const OC931_OBJ: u32 = DATA + 0x3_0000;
    pub(crate) const OC931_VT: u32 = DATA + 0x4_0000;
    pub(crate) const OC931_TEXT: u32 = DATA + 0x5_0000;
    pub(crate) const OC931_WIDGET: u32 = DATA + 0x6_0000;
    pub(crate) const OC931_WVT: u32 = DATA + 0x7_0000;

    /// The fake callee: logs like the plain fake; `getObjectByHashName`
    /// answers with a widget or none, as the fixture's `OC931_FOUND` bits say.
    pub(crate) fn oc931_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra || target != 0x0039_d860 {
            return;
        }
        let k = c.bus.read32(OC931_SEEN);
        c.bus.write32(OC931_SEEN, k + 1);
        let found = c.bus.read32(OC931_FOUND) >> (k & 31) & 1;
        c.g[V0][0] = if found == 1 { OC931_WIDGET as u64 } else { 0 };
    }

    macro_rules! oc931_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC931_GATE) != OC931_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc931_hook(c, $addr);
                }
            )*
            pub(crate) fn oc931_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc931_fakes!(
        (
            oc931_fk_00317670,
            OC931_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc931_fk_002c26d0,
            OC931_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002C26D0
        ),
        (
            oc931_fk_003a0e90,
            OC931_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003A0E90
        ),
        (
            oc931_fk_0039d860,
            OC931_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039D860
        ),
        (
            oc931_fk_003a0c50,
            OC931_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003A0C50
        ),
    );

    /// Fixture: seed picks the path (no text widget; no screen text object;
    /// format string method returning none; full path with both, one or no
    /// arrow widgets found), the page numbers (including per = 0, 1 page and
    /// many) and, through `machine`, the unwinds of the calls.
    fn oc931_fixture(c: &mut Cpu, seed: u32, unwind: bool) {
        let mut r = machine(c, seed, 0x0931_1d0b, 10);
        if !unwind {
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        }
        oc931_install_fakes();
        c.bus.write32(OC931_GATE, OC931_MAGIC);
        c.pc = 0x001d_0bc8;
        c.g[A0][0] = sx(OC931_SELF);
        let class = seed % 9;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        w(
            c,
            OC931_SELF + 0x904,
            if class == 0 { 0 } else { OC931_TEXT },
        );
        w(c, OC931_SELF + 0x40, OC931_SCREEN);
        w(c, OC931_SCREEN + 0xd0, OC931_SCREEN + 0x100);
        w(c, OC931_SCREEN + 0x100 + 0x10, OC931_SCREEN + 0x200);
        w(
            c,
            OC931_SCREEN + 0x200 + 0x10,
            if class == 1 { 0 } else { OC931_OBJ },
        );
        w(c, OC931_OBJ + 4, OC931_VT);
        c.bus.write16(OC931_VT + 0x20, (r.next_u32() & 0x3c) as u16);
        // Format string method: a fake (non-zero result) or a real leaf that
        // returns 0.
        w(
            c,
            OC931_VT + 0x24,
            if class == 2 { 0x0014_06c0 } else { fakes::FAKE },
        );
        let (per, total, cur) = match seed / 9 % 6 {
            0 => (4, 10, 3),
            1 => (4, 3, 1),
            2 => (1, 7, 6),
            3 => (0, 5, 2),
            4 => (3, 0, 0),
            _ => (8, 9, 100),
        };
        w(c, OC931_SELF + 0x58, per);
        w(c, OC931_SELF + 0x60, total);
        w(c, OC931_SELF + 0x64, cur);
        w(c, OC931_FOUND, if class < 6 { 0 } else { (seed / 9) % 4 });
        w(c, OC931_SEEN, 0);
        // The arrow widget: vtable with the visibility method at slot 9.
        w(c, OC931_WIDGET + 8, OC931_WVT);
        c.bus.write16(OC931_WVT + 0x48, 0);
        w(c, OC931_WVT + 0x4c, fakes::FAKE + 0x10);
    }

    /// `cFEStateRewardGallery_UpdatePageText`: every path (no text widget, no
    /// text object, no format string, widgets found or missing), page counts
    /// and the unwinds of its calls; LO/HI of the divisions too.
    #[test]
    fn c_fe_state_reward_gallery_update_page_text_matches() {
        check_lohi(
            ee::s04::f_001d0bc8__generated,
            c_fe_state_reward_gallery_update_page_text,
            540,
            &|c, seed| oc931_fixture(c, seed, true),
        );
    }

    /// The fixture reaches every path of the generated body (distinct fake
    /// call counts without unwinds: 0, 1, 2 and the full path with 0, 1 or 2
    /// arrows found).
    #[test]
    fn c_fe_state_reward_gallery_update_page_text_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..108 {
            let mut c = fresh();
            oc931_fixture(&mut c, seed, false);
            ee::s04::f_001d0bc8__generated(&mut c);
            seen.insert(c.bus.read32(fakes::LOG_COUNT));
        }
        assert!(seen.len() >= 6, "fake call counts {seen:?}");
    }
}
