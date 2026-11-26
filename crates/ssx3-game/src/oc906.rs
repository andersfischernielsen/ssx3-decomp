//! Staging module oc906 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc906`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::Flow;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::feasyncfile::{
    oc901_call_event, oc901_call_fill_list, oc901_call_reset, oc901_set_time_text, oc901_shown,
    oc901_white,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001c_3ee8,
    cfe_state_stats_select_category,
    "cFEStateStats_SelectCategory",
)];

/// `cFEStateStats_SelectCategory(self, mode)` (0x1c3ee8): switches the results screen
/// between overall (0), race (1) and freestyle (2). Stores `mode` in miMode and remembers
/// whether it changed (S2). With no screen (mpScreen at +0x40) it only stores the mode.
/// Otherwise it recolours the three category labels, shows only the matching highlight,
/// sets the "TimePoints" text (or "Time" for race), and sets mbShowTime (+0x6ec) to 1 for
/// race, 0 otherwise. Race and freestyle show the event course (SetEventCourseVisible) and
/// refill the event list when the mode changed. Overall hides them and resets the results
/// page. Modes other than 0..2 do only the prologue and the store.
pub fn cfe_state_stats_select_category(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0x10);
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = c.lw(this.wrapping_add(0x6e0));
        c.g[V1][0] = c.lw(this.wrapping_add(0x40));
        c.g[V0][0] ^= c.g[A1][0];
        c.sw(this.wrapping_add(0x6e0), c.g[A1][0]);
        let no_screen = c.g[V1][0] == 0;
        c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
        'b4528: {
            'b4524: {
                'b4520: {
                    if no_screen {
                        break 'b4520;
                    }
                    'b4330: {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        let is_one = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                        if is_one {
                            break 'b4330;
                        }
                        'b413c: {
                            'b3f3c: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = 2;
                                    break 'b3f3c;
                                }
                                c.cycles += 2;
                                let mode_zero = c.g[A1][0] == 0;
                                frame.restore(c, 16, 0x40);
                                if mode_zero {
                                    break 'b413c;
                                }
                                c.cycles += 2;
                                frame.restore(c, 17, 0x30);
                                break 'b4528;
                            }
                            // Freestyle (mode 2).
                            c.cycles += 2;
                            let other = c.g[A1][0] != c.g[V0][0];
                            frame.restore(c, 16, 0x40);
                            if other {
                                break 'b4524;
                            }
                            c.cycles += 13;
                            c.sw(this.wrapping_add(0x6ec), 0u64);
                            oc901_call_event(c, 1, 0x001c_3f54)?;
                            oc901_white(c, 0x0046_5d10, [0x3f60, 0x3f6c, 0x3fa4])?;
                            c.cycles += 9;
                            oc901_white(c, 0x0046_5cf0, [0x3fb0, 0x3fbc, 0x3ff4])?;
                            c.cycles += 9;
                            oc901_white(c, 0x0046_5d00, [0x4000, 0x400c, 0x4044])?;
                            c.cycles += 9;
                            oc901_shown(c, 0x0046_5e58, [0x4050, 0x405c, 0x407c], 0)?;
                            c.cycles += 9;
                            oc901_shown(c, 0x004a_1c28, [0x4088, 0x4094, 0x40b4], 0)?;
                            c.cycles += 9;
                            oc901_shown(c, 0x0046_5e68, [0x40c0, 0x40cc, 0x40ec], 1)?;
                            c.cycles += 9;
                            oc901_set_time_text(
                                c,
                                0x0046_5e78,
                                0x0046_5e88,
                                [0x40f8, 0x4104, 0x4118, 0x4124],
                            )?;
                            c.cycles += 2;
                            let no_change = c.g[S2][0] == 0;
                            frame.restore(c, 16, 0x40);
                            if no_change {
                                break 'b4524;
                            }
                            c.cycles += 4;
                            oc901_call_fill_list(c, 0x001c_4134)?;
                            frame.restore(c, 16, 0x40);
                            break 'b4524;
                        }
                        // Overall (mode 0).
                        c.cycles += 13;
                        c.sw(this.wrapping_add(0x6ec), 0u64);
                        oc901_call_event(c, 0, 0x001c_414c)?;
                        oc901_white(c, 0x0046_5cf0, [0x4158, 0x4164, 0x419c])?;
                        c.cycles += 9;
                        oc901_white(c, 0x0046_5d10, [0x41a8, 0x41b4, 0x41ec])?;
                        c.cycles += 9;
                        oc901_white(c, 0x0046_5d00, [0x41f8, 0x4204, 0x423c])?;
                        c.cycles += 9;
                        oc901_shown(c, 0x0046_5e58, [0x4248, 0x4254, 0x4274], 1)?;
                        c.cycles += 9;
                        oc901_shown(c, 0x004a_1c28, [0x4280, 0x428c, 0x42ac], 0)?;
                        c.cycles += 9;
                        oc901_shown(c, 0x0046_5e68, [0x42b8, 0x42c4, 0x42e4], 0)?;
                        c.cycles += 9;
                        oc901_set_time_text(
                            c,
                            0x0046_5e78,
                            0x0046_5e88,
                            [0x42f0, 0x42fc, 0x4310, 0x431c],
                        )?;
                        c.cycles += 5;
                        oc901_call_reset(c, 0x001c_4328)?;
                        frame.restore(c, 16, 0x40);
                        break 'b4524;
                    }
                    // Race (mode 1).
                    c.cycles += 13;
                    let mode = c.g[A1][0];
                    c.sw(this.wrapping_add(0x6ec), mode);
                    oc901_call_event(c, 1, 0x001c_4340)?;
                    oc901_white(c, 0x0046_5d00, [0x434c, 0x4358, 0x4390])?;
                    c.cycles += 9;
                    oc901_white(c, 0x0046_5cf0, [0x439c, 0x43a8, 0x43e0])?;
                    c.cycles += 9;
                    oc901_white(c, 0x0046_5d10, [0x43ec, 0x43f8, 0x4430])?;
                    c.cycles += 9;
                    oc901_shown(c, 0x0046_5e58, [0x443c, 0x4448, 0x4468], 0)?;
                    c.cycles += 9;
                    oc901_shown(c, 0x004a_1c28, [0x4474, 0x4480, 0x44a0], 1)?;
                    c.cycles += 9;
                    oc901_shown(c, 0x0046_5e68, [0x44ac, 0x44b8, 0x44d8], 0)?;
                    c.cycles += 9;
                    oc901_set_time_text(
                        c,
                        0x0046_5e78,
                        0x0046_5e98,
                        [0x44e4, 0x44f0, 0x4504, 0x4510],
                    )?;
                    c.cycles += 2;
                    let no_change = c.g[S2][0] == 0;
                    frame.restore(c, 16, 0x40);
                    if no_change {
                        break 'b4524;
                    }
                    c.cycles += 2;
                    oc901_call_fill_list(c, 0x001c_4520)?;
                    break 'b4520;
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x40);
            }
            c.cycles += 1;
            frame.restore(c, 17, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word, widget mask / lookup counter and the call log hash (the fakes' state).
    pub(crate) const OC906_CTRL: u32 = DATA + 0x5000;
    pub(crate) const OC906_MAGIC: u32 = 0x0906_0906;
    /// The `cFEStateStats` object, the screen, one widget and its vtable.
    pub(crate) const OC906_SELF: u32 = DATA + 0x100;
    pub(crate) const OC906_SCREEN: u32 = DATA + 0x6000;
    pub(crate) const OC906_WIDGET: u32 = DATA + 0x2000;
    pub(crate) const OC906_VT: u32 = DATA + 0x3000;

    pub(crate) static OC906_PREV_FIND: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC906_PREV_EVENT: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC906_PREV_FILL: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC906_PREV_RESET: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC906_PREV_TEXT: AtomicUsize = AtomicUsize::new(0);

    /// Fold a fake call (target, a0, a1) into the log hash both sides must agree on.
    fn oc906_log(c: &mut Cpu, target: u32) {
        let h = c.bus.read32(OC906_CTRL + 0x20) as u64;
        let x = fakes::mix(h ^ (target as u64) << 32 ^ c.g[A0][0] ^ c.g[A1][0].rotate_left(21));
        c.bus.write32(OC906_CTRL + 0x20, x as u32);
        c.g[V0][0] = (x & 0xff) as u64;
        c.cycles += 7;
        ret(c);
    }

    /// `cUIScreen_getObjectByHashName` (0x39d860): lookup k returns the widget when bit k of
    /// the mask is set, else null.
    pub(crate) fn oc906_fake_find(c: &mut Cpu) {
        if c.bus.read32(OC906_CTRL) != OC906_MAGIC {
            return fakes::fall_back(c, &OC906_PREV_FIND, ee::s13::f_0039d860__generated);
        }
        let k = c.bus.read32(OC906_CTRL + 8);
        c.bus.write32(OC906_CTRL + 8, k + 1);
        let mask = c.bus.read32(OC906_CTRL + 4);
        c.g[V0][0] = if k < 32 && (mask >> k) & 1 != 0 {
            OC906_WIDGET as u64
        } else {
            0
        };
        c.cycles += 5;
        ret(c);
    }

    pub(crate) fn oc906_fake_event(c: &mut Cpu) {
        if c.bus.read32(OC906_CTRL) != OC906_MAGIC {
            return fakes::fall_back(c, &OC906_PREV_EVENT, ee::s03::f_001c4538__generated);
        }
        oc906_log(c, 0x001c_4538)
    }

    pub(crate) fn oc906_fake_fill(c: &mut Cpu) {
        if c.bus.read32(OC906_CTRL) != OC906_MAGIC {
            return fakes::fall_back(c, &OC906_PREV_FILL, ee::s03::f_001c47a8__generated);
        }
        oc906_log(c, 0x001c_47a8)
    }

    pub(crate) fn oc906_fake_reset(c: &mut Cpu) {
        if c.bus.read32(OC906_CTRL) != OC906_MAGIC {
            return fakes::fall_back(c, &OC906_PREV_RESET, ee::s03::f_001c4b78__generated);
        }
        oc906_log(c, 0x001c_4b78)
    }

    pub(crate) fn oc906_fake_text(c: &mut Cpu) {
        if c.bus.read32(OC906_CTRL) != OC906_MAGIC {
            return fakes::fall_back(c, &OC906_PREV_TEXT, ee::s13::f_003a1000__generated);
        }
        oc906_log(c, 0x003a_1000)
    }

    /// `cFEStateStats_SelectCategory(self, mode)`: the modes 0, 1, 2 and others, with and
    /// without a screen, with the mode changed or not, and with each subset of the widgets
    /// found (the lookup mask), the callees stood in by logging fakes and a widget whose
    /// colour and visibility slots are a real leaf.
    #[test]
    fn cfe_state_stats_select_category_matches() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc906_fake_find,
            &OC906_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C4538),
            oc906_fake_event,
            &OC906_PREV_EVENT,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C47A8),
            oc906_fake_fill,
            &OC906_PREV_FILL,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C4B78),
            oc906_fake_reset,
            &OC906_PREV_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc906_fake_text,
            &OC906_PREV_TEXT,
        );
        check(
            ee::s03::f_001c3ee8__generated,
            cfe_state_stats_select_category,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x906, 0);
                elf_image(c);
                let mode = [0u32, 1, 2, 3, 4, 2, 1, 0][(seed % 8) as usize];
                let old = if seed % 3 == 0 { mode } else { mode ^ 1 };
                c.bus.write32(OC906_SELF + 0x6e0, old);
                let screen = if seed % 11 == 0 { 0 } else { OC906_SCREEN };
                c.bus.write32(OC906_SELF + 0x40, screen);
                c.bus.write32(OC906_WIDGET + 8, OC906_VT);
                c.bus.write16(OC906_VT + 0x58, 0);
                c.bus.write16(OC906_VT + 0x48, 0);
                c.bus.write32(OC906_VT + 0x5c, 0x0014_06c0);
                c.bus.write32(OC906_VT + 0x4c, 0x0014_06c0);
                let mask = match seed % 4 {
                    0 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(OC906_CTRL, OC906_MAGIC);
                c.bus.write32(OC906_CTRL + 4, mask);
                c.bus.write32(OC906_CTRL + 8, 0);
                c.bus.write32(OC906_CTRL + 0x20, 0);
                c.g[A0][0] = sx(OC906_SELF);
                c.g[A1][0] = mode as u64;
                c.pc = 0x001c_3ee8;
            },
        );
    }
}
