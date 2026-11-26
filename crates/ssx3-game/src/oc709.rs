//! Staging module oc709 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc709`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_dd10,
        bolt_file_compare_model_slots,
        "BoltFile_CompareModelSlots",
    ),
    (0x002a_f960, speech_bank_load, "SpeechBank_Load"),
    (0x0039_7298, ui_anim_set_data, "UIAnim_SetData"),
    (0x0018_76a8, cfe_state_mem_card_v26, "cFEStateMemCard_v26"),
    (0x003a_0b90, ui_text_count_lines, "UIText_CountLines"),
];

/// `cUIAnimationChannel::setAnimChanData` (0x397298): stores `a1` as the word
/// at `a0`. A leaf of two cycles.
pub fn ui_anim_set_data(c: &mut Cpu) {
    c.cycles += 2;
    let value = c.g[A1][0];
    Addr(crate::abi::arg(c, 0)).sw(c, value);
    ret(c);
}

/// `cUIText_getNumTextLines` (0x3a0b90): the number of lines in the text of a
/// UI text object (`self+0xb4`, a zero-terminated array of 16-bit characters).
/// A pair of equal line-break characters (`gp+0x16b8`) counts as one line, and
/// a carriage return (13) ends a line. Returns the count as a sign-extended
/// byte.
pub fn ui_text_count_lines(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xb4)).lw(c);
    let has_text = c.g[V0][0] != 0;
    c.g[V1][0] = c.g[V0][0];
    if !has_text {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
    c.g[A1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16b8)).lhu(c);
        c.g[A3][0] = 0xd;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
            let t = c.g[V0][0] != c.g[A2][0];
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(2)).lhu(c);
            let mut pair = false;
            if !t {
                c.cycles += 2;
                pair = c.g[A0][0] == c.g[V0][0];
            }
            if pair {
                // Two equal line-break characters: skip both, one line.
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                let carriage = c.g[V0][0] == c.g[A3][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
                if carriage {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                }
            }
            // Loop test at the back edge.
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_0bb8);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
    ret(c);
}

/// Result of the compare passes of `BoltFile_CompareModelSlots`: where the
/// function leaves to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum BoltCompareExit {
    /// Second pass over the records (their words at +0 and +0x110/+0x11c).
    Second,
    /// Early return (1 cycle, then the epilogue).
    Early,
    /// Late return (2 cycles, then the epilogue).
    Late,
    /// Compare the two names (`+4`) with `strcasecmp`, then return.
    Names,
}

/// First pass of `BoltFile_CompareModelSlots`: the record at `a0` (the
/// computed slot) and the one at `a1` must match in their words at +0 (3 for
/// `a0`), +0x110 (8) and +0x11c (0).
fn bcms_first_pass(c: &mut Cpu) -> BoltCompareExit {
    if c.g[V1][0] != c.g[A3][0] {
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        return BoltCompareExit::Second;
    }
    c.cycles += 4;
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x110)).lw(c);
    c.g[V0][0] = 8;
    if c.g[A2][0] != c.g[V0][0] {
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        return BoltCompareExit::Second;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x11c)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        return BoltCompareExit::Second;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    let t = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = u64::MAX;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x110)).lw(c);
    let t = c.g[V0][0] != c.g[A2][0];
    c.g[V0][0] = u64::MAX;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x11c)).lw(c);
    let t = c.g[V0][0] == 0;
    c.g[V0][0] = u64::MAX;
    if t {
        return BoltCompareExit::Names;
    }
    BoltCompareExit::Late
}

/// Second pass of `BoltFile_CompareModelSlots`: the same words again, with
/// `a2` (the word at +0 of `a1`) as the expected value of `a0`'s word at +0.
fn bcms_second_pass(c: &mut Cpu) -> BoltCompareExit {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let t = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 4;
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x110)).lw(c);
    c.g[V0][0] = 8;
    let t = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x11c)).lw(c);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    let t = c.g[V0][0] != c.g[A2][0];
    c.g[V0][0] = 1;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x110)).lw(c);
    let t = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = 1;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x11c)).lw(c);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if t {
        return BoltCompareExit::Early;
    }
    BoltCompareExit::Names
}

/// `BoltFile_CompareModelSlots` (0x19dd10): finds the slot of a model in a
/// bolt file table and compares it with the slot named by the second
/// argument. `a0` points at a word whose high half selects a row (times
/// `0x12000`) and whose low half selects a slot (times `0x120`, the slot
/// record size); `a1` points at a 16-bit slot index. The two records are
/// compared on their type words (3 and 8) and their name field at +0x11c
/// (0); equal records compare their names (`+4`) with `strcasecmp`. Returns
/// the `strcasecmp` result, or 0/1/-1 for the early outs.
pub fn bolt_file_compare_model_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x12000;
        c.g[T0][0] = 0x120;
        c.g[A3][0] = 3;
        c.g[V1][0] = Addr(crate::abi::arg(c, 0)).lw(c);
        c.g[A2][0] = Addr(crate::abi::arg(c, 1)).lhu(c);
        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
        c.mult(0, A0, A0, V0);
        c.g[V1][0] &= 0xffff;
        c.mult(0, V1, V1, T0);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1784)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.mult(0, V1, A2, T0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);

        let exit = match bcms_first_pass(c) {
            BoltCompareExit::Second => bcms_second_pass(c),
            other => other,
        };
        match exit {
            BoltCompareExit::Early => {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
            BoltCompareExit::Late => {
                c.cycles += 2;
                frame.restore_ra(c, 0);
            }
            BoltCompareExit::Names => {
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                call_ptr(c, 0x0041_aa88, 0x0019_de00)?;
                frame.restore_ra(c, 0);
            }
            BoltCompareExit::Second => {}
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Menu-kind values `cFEStateMemCard_v26` compares the manager's word at
/// +0x130 against, in the order the generated compare chain tests them.
const CFE_MEM_CARD_KINDS: [u64; 13] = [
    0x1f, 0x21, 0x20, 0x22, 0x31, 0x32, 0x33, 0x35, 0x36, 0x34, 0x2, 0x3, 0x6,
];

/// `cFEStateMemCard_v26` (0x1876a8, reached through `cFEStateMemCard`): asks
/// the UI screen (`gp-0x1c38`) for an object by hash name, asks the MC
/// manager for its message kind (`+0x130`), and either pulls the message
/// from the overlay manager and sets the text (kinds in the table above), or
/// calls the object's vtable slot. Frame 0x660, s0 saved at +0x650.
pub fn cfe_state_mem_card_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x660);
        frame.save(c, S0, 0x650);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2380));
        frame.save_ra(c, 0x640);
        c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_76c4)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_76d0)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0022_7f80, 0x0018_76dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x130)).lw(c);

        // The kind compare chain: each step costs 2 cycles after the first.
        let mut matched = false;
        for (i, &kind) in CFE_MEM_CARD_KINDS.iter().enumerate() {
            if i > 0 {
                c.cycles += 2;
            }
            let hit = c.g[V1][0] == kind;
            c.g[V0][0] = CFE_MEM_CARD_KINDS[(i + 1).min(CFE_MEM_CARD_KINDS.len() - 1)];
            if hit {
                matched = true;
                break;
            }
        }

        if matched {
            c.cycles += 14;
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x320;
            call_ptr(c, 0x0024_1e18, 0x0018_775c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0d00, 0x0018_7768)?;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_7780)?;
            frame.restore(c, S0, 0x650);
        } else {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.cycles += 6;
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_779c)?;
            frame.restore(c, S0, 0x650);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x640);
        frame.close(c);
        Ok(())
    });
}

/// `SpeechBank_Load` (0x2af960): builds the bank path with sprintf from the
/// format at 0x4a37c0 and `a0`. When `a2` is 0 it loads the file (only if the
/// file exists; otherwise it returns 0). It then allocates and constructs a
/// 0x4c-byte bank object, registers the bank with the sound system, copies the
/// path into the object and stores the file name part (after the last
/// backslash). Returns the object.
pub fn speech_bank_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x80);
        c.g[A1][0] = 0x004a_37c0;
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S0, 0xa0);
        c.g[S4][0] = (c.g[S2][0] < 0x1) as u64;
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x50);
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0041_7828, 0x002a_f99c)?;

        c.g[A1][0] = 0x0048_0000;
        let mut loaded = true;
        if c.g[S2][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7f38, 0x002a_f9ac)?;
            let exists = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[SP][0];
            if exists {
                c.cycles += 4;
                c.g[A1][0] = 0;
                call_ptr(c, 0x003e_18c8, 0x002a_f9c4)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = 0x0048_0000;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                loaded = false;
            }
        }

        if loaded {
            c.cycles += 22;
            c.g[A2][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x32b0));
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x4c;
            call_ptr(c, 0x0031_7d70, 0x002a_f9e0)?;
            c.g[A0][0] = c.g[V0][0];
            Callee {
                addr: 0x002a_f8e0,
                f: ee::s08::f_002af8e0,
            }
            .call(c, 0x002a_f9e8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            let s4 = c.g[S4][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x48)).sw(c, s4);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            let s2 = c.g[S2][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x44)).sw(c, s2);
            call_ptr(c, 0x003d_69f0, 0x002a_fa00)?;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S1][0];
            let bank = c.g[V0][0];
            Addr(c.g[S0][0] as u32).sw(c, bank);
            call_ptr(c, 0x0041_66f4, 0x002a_fa10)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x5c;
            call_ptr(c, 0x0041_acc0, 0x002a_fa1c)?;
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            if found {
                c.cycles += 1;
                let name = c.g[V0][0];
                Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, name);
            } else {
                c.cycles += 2;
                let path = c.g[S1][0];
                Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, path);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }

        c.cycles += 8;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC709_MAGIC` (otherwise the generated body runs).
    const OC709_CTRL: u32 = DATA + 0x0020_0000;
    const OC709_MAGIC: u32 = 0x0709_c0de;
    /// Per-call results of the path-exists fake (four words, one per call index).
    const OC709_RES: u32 = DATA + 0x0020_1000;
    const OC709_UI: u32 = DATA + 0x0020_3000;
    const OC709_OBJ: u32 = DATA + 0x0020_4000;
    const OC709_OBJ2: u32 = DATA + 0x0020_5000;
    const OC709_MGR: u32 = DATA + 0x0020_6000;
    const OC709_NAME: u32 = DATA + 0x0020_7000;
    const OC709_FILE: u32 = DATA + 0x0020_8000;
    /// The bank object the allocator fake returns.
    const OC709_BANK: u32 = DATA + 0x0030_0000;

    fn oc709_open(c: &mut Cpu) -> bool {
        c.bus.read32(OC709_CTRL) == OC709_MAGIC
    }

    fn oc709_fake_317670(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_317670),
                ee::s10::f_00317670__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7670);
    }

    fn oc709_fake_39d860(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_39d860),
                ee::s13::f_0039d860__generated,
            );
        }
        fakes::fake_call(c, 0x0039_d860);
    }

    fn oc709_fake_227f80(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_227f80),
                ee::s05::f_00227f80__generated,
            );
        }
        fakes::fake_call(c, 0x0022_7f80);
    }

    fn oc709_fake_241e18(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_241e18),
                ee::s06::f_00241e18__generated,
            );
        }
        fakes::fake_call(c, 0x0024_1e18);
    }

    fn oc709_fake_417828(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_417828),
                ee::s15::f_00417828__generated,
            );
        }
        fakes::fake_call(c, 0x0041_7828);
    }

    fn oc709_fake_3e18c8(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_3e18c8),
                ee::s14::f_003e18c8__generated,
            );
        }
        fakes::fake_call(c, 0x003e_18c8);
    }

    /// `EAFile_PathExists`: answers the per-call word the setup wrote.
    fn oc709_fake_317f38(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_317f38),
                ee::s10::f_00317f38__generated,
            );
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let exists = c.bus.read32(OC709_RES + 4 * (n % 4));
        fakes::fake_call(c, 0x0031_7f38);
        c.g[V0][0] = exists as u64;
    }

    /// The bank allocator: a fixed object (the constructor runs on it).
    fn oc709_fake_317d70(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_317d70),
                ee::s10::f_00317d70__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7d70);
        c.g[V0][0] = OC709_BANK as u64;
    }

    fn oc709_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc709_fake_317670,
            fakes::prev_cell(oc709_fake_317670),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc709_fake_39d860,
            fakes::prev_cell(oc709_fake_39d860),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc709_fake_227f80,
            fakes::prev_cell(oc709_fake_227f80),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241E18),
            oc709_fake_241e18,
            fakes::prev_cell(oc709_fake_241e18),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc709_fake_417828,
            fakes::prev_cell(oc709_fake_417828),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E18C8),
            oc709_fake_3e18c8,
            fakes::prev_cell(oc709_fake_3e18c8),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317F38),
            oc709_fake_317f38,
            fakes::prev_cell(oc709_fake_317f38),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc709_fake_317d70,
            fakes::prev_cell(oc709_fake_317d70),
        );
    }

    #[test]
    fn ui_anim_set_data_matches() {
        check(
            ee::s13::f_00397298__generated,
            ui_anim_set_data,
            300,
            &|c, seed| {
                let mut r = Rng(0x0039_7298 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0039_7298;
                c.g[A0][0] = sx(DATA + 0x100);
                let hi = r.next_u32() as u64;
                let lo = r.next_u32() as u64;
                c.g[A1][0] = (hi << 32) | lo;
                c.bus.write32(DATA + 0x100, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_text_count_lines_matches() {
        check(
            ee::s13::f_003a0b90__generated,
            ui_text_count_lines,
            300,
            &|c, seed| {
                let mut r = Rng(0x003a_0b90 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_0b90;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                let text = DATA + 0x1000;
                c.bus
                    .write32(DATA + 0xb4, if seed % 9 == 0 { 0 } else { text });
                let a2: u16 = [0x0a, 0x0d, 0x61][(seed % 3) as usize];
                c.bus.write16(GAME_GP + 0x16b8, a2);
                let n = if seed % 7 == 0 { 0 } else { r.next_u32() % 16 };
                for i in 0..n {
                    let ch: u16 = match r.next_u32() % 4 {
                        0 | 1 => a2,
                        2 => 0x0d,
                        _ => 0x61,
                    };
                    c.bus.write16(text + 2 * i, ch);
                }
                c.bus.write16(text + 2 * n, 0);
            },
        );
    }

    #[test]
    fn bolt_file_compare_model_slots_matches() {
        check(
            ee::s03::f_0019dd10__generated,
            bolt_file_compare_model_slots,
            300,
            &|c, seed| {
                let mut r = Rng(0x0019_dd10 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0019_dd10;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
                let table = DATA + 0x0020_0000;
                let k = seed % 3;
                let j = 3 + seed % 3;
                c.bus.write32(DATA, k);
                c.bus.write16(DATA + 0x40, j as u16);
                c.bus.write32(GAME_GP - 0x1784, table);
                fill(c, &mut r, table, 0x120 * 8 / 4);
                let a = table + k * 0x120;
                let b = table + j * 0x120;
                // Modes walk the first pass, the second pass and the names.
                let (a0, a110, a11c, b0, b110, b11c) = match seed % 8 {
                    0 => (0, 8, 0, 3, 8, 0),
                    1 => (3, 0, 0, 3, 8, 0),
                    2 => (3, 8, 1, 3, 8, 0),
                    3 => (3, 8, 0, 4, 8, 0),
                    4 => (3, 8, 0, 3, 0, 0),
                    5 => (3, 8, 0, 3, 8, 1),
                    6 => (3, 8, 0, 3, 8, 0),
                    _ => (9, 8, 0, 3, 8, 0),
                };
                c.bus.write32(a, a0);
                c.bus.write32(a + 0x110, a110);
                c.bus.write32(a + 0x11c, a11c);
                c.bus.write32(b, b0);
                c.bus.write32(b + 0x110, b110);
                c.bus.write32(b + 0x11c, b11c);
                // The names at +4 differ in the third letter.
                c.bus.write32(a + 4, u32::from_le_bytes(*b"ABC\0"));
                c.bus.write32(b + 4, u32::from_le_bytes(*b"ABD\0"));
            },
        );
    }

    #[test]
    fn cfe_state_mem_card_v26_matches() {
        oc709_install_fakes();
        check(
            ee::s02::f_001876a8__generated,
            cfe_state_mem_card_v26,
            300,
            &|c, seed| {
                let mut r = Rng(0x0018_76a8 ^ seed as u64);
                scramble(c, &mut r);
                let _ = fakes::machine(c, seed, 0x0018_76a8, 0);
                c.pc = 0x0018_76a8;
                c.bus.write32(OC709_CTRL, OC709_MAGIC);
                c.bus.write32(GAME_GP - 0x1c38, OC709_UI);
                // Call 1 returns the UI object, call 2 the manager.
                c.bus.write32(fakes::LOG_RESULTS + 4, OC709_OBJ);
                c.bus.write32(fakes::LOG_RESULTS + 8, OC709_MGR);
                c.bus.write32(OC709_OBJ + 8, OC709_OBJ2);
                c.bus
                    .write16(OC709_OBJ2 + 0x48, (0x10 * (seed % 3 + 1)) as u16);
                c.bus.write32(OC709_OBJ2 + 0x4c, fakes::FAKE);
                let kind = if seed % 4 == 0 {
                    r.next_u32()
                } else {
                    CFE_MEM_CARD_KINDS[(seed % 13) as usize] as u32
                };
                c.bus.write32(OC709_MGR + 0x130, kind);
            },
        );
    }

    #[test]
    fn speech_bank_load_matches() {
        oc709_install_fakes();
        check(
            ee::s08::f_002af960__generated,
            speech_bank_load,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_f960 ^ seed as u64);
                scramble(c, &mut r);
                let _ = fakes::machine(c, seed, 0x002a_f960, 0);
                c.pc = 0x002a_f960;
                c.bus.write32(OC709_CTRL, OC709_MAGIC);
                let name: &[u8] = if seed % 2 == 0 {
                    b"data\\bank.snd\0"
                } else {
                    b"bank.snd\0"
                };
                for (i, byte) in name.iter().enumerate() {
                    c.bus.write8(OC709_NAME + i as u32, *byte);
                }
                c.g[A0][0] = sx(OC709_NAME);
                c.g[A2][0] = if seed % 3 == 1 { sx(OC709_FILE) } else { 0 };
                let exists = (seed % 3 != 0) as u32;
                for i in 0..4 {
                    c.bus.write32(OC709_RES + 4 * i, exists);
                }
            },
        );
    }
}
