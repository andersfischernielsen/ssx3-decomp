//! Staging module oc901 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc901`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// The return addresses in the call tables are offsets into 0x1c0000.
const RA_BASE: u32 = 0x001c_0000;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `GetHashValue32(name)` (0x317670), then `cUIScreen_getObjectByHashName(self->mpScreen, hash)`
/// (0x39d860). The widget ends in V0 (A0 and A1 hold the call's last arguments).
fn oc901_lookup(c: &mut Cpu, name: u32, ret_hash: u32, ret_find: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)
}

/// The white `setColor` of the widget in A2: the widget's table slot 0x5c, colour (1, 1, 1, 1)
/// at SP (A1). 11 cycles on this path.
fn oc901_set_white(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 11;
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
    let s = sp(c);
    c.swc1(0, s.wrapping_add(0xc));
    c.swc1(0, s);
    c.swc1(0, s.wrapping_add(0x4));
    c.swc1(0, s.wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

/// The visibility call of the widget in A2: the widget's table slot 0x4c with the flag in A1.
/// 5 cycles on this path.
fn oc901_set_shown(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
    let f = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

/// A colour block: look the widget `name` up, colour it white when found (the caller adds
/// the block's cycles). `rets`: hash, find, colour.
fn oc901_white(c: &mut Cpu, name: u32, rets: [u32; 3]) -> Flow {
    oc901_lookup(c, name, RA_BASE | rets[0], RA_BASE | rets[1])?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[SP][0];
    if c.g[A2][0] != 0 {
        oc901_set_white(c, RA_BASE | rets[2])?;
    }
    Ok(())
}

/// A visibility block: look the widget `name` up, set its visibility to `flag` when found.
/// `rets`: hash, find, visibility call.
fn oc901_shown(c: &mut Cpu, name: u32, rets: [u32; 3], flag: u64) -> Flow {
    oc901_lookup(c, name, RA_BASE | rets[0], RA_BASE | rets[1])?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = flag;
    if c.g[A2][0] != 0 {
        oc901_set_shown(c, RA_BASE | rets[2])?;
    }
    Ok(())
}

/// The "Txt_TimePoints" block: when the widget `name` is found,
/// `cUIText_setUnicodeStringByID(uiText, GetHashValue32(id))` (0x3a1000).
/// `rets`: hash, find, id hash, set.
fn oc901_set_time_text(c: &mut Cpu, name: u32, id: u32, rets: [u32; 4]) -> Flow {
    oc901_lookup(c, name, RA_BASE | rets[0], RA_BASE | rets[1])?;
    c.g[S0][0] = c.g[V0][0];
    let found = c.g[S0][0] != 0;
    c.g[A0][0] = 0x0046_0000;
    if found {
        c.cycles += 5;
        c.g[A0][0] = sx(id);
        call_ptr(c, 0x0031_7670, RA_BASE | rets[2])?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, RA_BASE | rets[3])?;
    }
    Ok(())
}

/// `cFEStateStats_SetEventCourseVisible(self, shown)` (0x1c4538).
fn oc901_call_event(c: &mut Cpu, shown: u64, ret: u32) -> Flow {
    c.g[A1][0] = shown;
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x001c_4538,
        f: ee::s03::f_001c4538,
    }
    .call(c, ret)
}

/// `cFEStateStats_FillEventList(self)` (0x1c47a8).
fn oc901_call_fill_list(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x001c_47a8,
        f: ee::s03::f_001c47a8,
    }
    .call(c, ret)
}

/// `cFEStateStats_ResetResultsPage(self, 0)` (0x1c4b78).
fn oc901_call_reset(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0;
    Callee {
        addr: 0x001c_4b78,
        f: ee::s03::f_001c4b78,
    }
    .call(c, ret)
}

/// `cFEStateStats_SelectCategory(self, mode)` (0x1c3ee8): switches the results screen
/// between overall (0), race (1) and freestyle (2). Stores `mode` in miMode and remembers
/// whether it changed (S2). With no screen (mpScreen at +0x40) it only stores the mode.
/// Otherwise it recolours the three category labels, shows only the matching highlight,
/// sets the "TimePoints" text (or "Time" for race), and sets mbShowTime (+0x6ec) to 1 for
/// race, 0 otherwise. Race and freestyle show the event course (SetEventCourseVisible) and
/// refill the event list when the mode changed. Overall hides them and resets the results
/// page. Modes other than 0..2 do only the prologue and the store.
/// NOT INSTALLED: the test cannot be bounded (the real SetEventCourseVisible /
/// FillEventList spin in `EAFile_AllocRequest` on fixture data); a path-diff
/// pass fixed two bugs but the seed-1 path was never verified.
#[allow(dead_code)]
pub fn state_stats_select_category(c: &mut Cpu) {
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
    #![allow(dead_code)]
    use super::*;
    use crate::difftest::{check, fakes, DATA};
    use std::ptr::addr_of_mut;

    /// The gate word: the fakes answer only while the machine holds `OC901_MAGIC` here.
    const OC901_CTRL: u32 = DATA + 0x2a00;
    const OC901_MAGIC: u32 = 0x0901_c0de;
    /// Non-zero when the lookups find a widget.
    const OC901_HAS_WIDGET: u32 = DATA + 0x2a04;
    const OC901_THIS: u32 = DATA + 0x1400;
    const OC901_WIDGET: u32 = DATA + 0x1300;
    const OC901_VT: u32 = DATA + 0x1100;
    const OC901_SCREEN: u32 = DATA + 0x1500;

    /// `GetHashValue32` fake: the hash is the string's address; logs the real call.
    fn oc901_str_hash(c: &mut Cpu) {
        if c.bus.read32(OC901_CTRL) != OC901_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc901_str_hash),
                ee::s10::f_00317670__generated,
            );
        }
        let s = c.g[A0][0];
        fakes::fake_call(c, 0x0031_7670);
        c.g[V0][0] = s;
    }

    /// `cUIScreen_getObjectByHashName` fake: the widget when the test says there is one.
    fn oc901_find(c: &mut Cpu) {
        if c.bus.read32(OC901_CTRL) != OC901_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc901_find),
                ee::s13::f_0039d860__generated,
            );
        }
        fakes::fake_call(c, 0x0039_d860);
        c.g[V0][0] = if c.bus.read32(OC901_HAS_WIDGET) != 0 {
            OC901_WIDGET as u64
        } else {
            0
        };
    }

    /// `cUIText_setUnicodeStringByID` fake.
    fn oc901_set_text(c: &mut Cpu) {
        if c.bus.read32(OC901_CTRL) != OC901_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc901_set_text),
                ee::s13::f_003a1000__generated,
            );
        }
        fakes::fake_call(c, 0x003a_1000);
    }

    /// `cFEStateStats_ResetResultsPage` fake.
    fn oc901_reset(c: &mut Cpu) {
        if c.bus.read32(OC901_CTRL) != OC901_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc901_reset),
                ee::s03::f_001c4b78__generated,
            );
        }
        fakes::fake_call(c, 0x001c_4b78);
    }

    /// Puts the gated fakes in the callees' override slots (once per process).
    fn oc901_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc901_str_hash,
            fakes::prev_cell(oc901_str_hash),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc901_find,
            fakes::prev_cell(oc901_find),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc901_set_text,
            fakes::prev_cell(oc901_set_text),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C4B78),
            oc901_reset,
            fakes::prev_cell(oc901_reset),
        );
    }
}
