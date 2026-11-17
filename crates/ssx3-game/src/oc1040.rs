//! Staging module oc1040 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1040`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::Flow;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::movie::movieplayer::mcus;
use crate::game::movie::movieplayer::mcus_card_call;
use crate::game::movie::movieplayer::mcus_load_card;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0023_cf38, mc_manager_v01, "MCManager_v01")];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_MANAGER_REQUEST_CARD_INFO;

    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;

    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_SET_MODE;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;

    pub const MC_MANAGER_GET_PRODUCT_CODE: Callee = Callee {
        addr: 0x0024_1aa0,
        f: ee::s06::MCManager_GetProductCode,
    };

    pub const MC_MANAGER_GET_SAVE_TYPE_TAG: Callee = Callee {
        addr: 0x0024_1d40,
        f: ee::s06::MCManager_GetSaveTypeTag,
    };

    pub const MC_MANAGER_READ_CURRENT_DIR_INFO: Callee = Callee {
        addr: 0x0024_1fb0,
        f: ee::s06::MCManager_ReadCurrentDirInfo,
    };

    pub const MC_MANAGER_BUILD_SAVE_TITLE: Callee = Callee {
        addr: 0x0023_cdb0,
        f: ee::s06::MCManager_BuildSaveTitle,
    };
}

/// `MCManager_v01`'s addresses and the manager words it writes that the
/// `mcus` offsets do not name.
pub(crate) mod mc_v01 {
    /// The function's first instruction and its end (for `jump_out`).
    pub const ENTRY: u32 = 0x0023_cf38;

    pub const END: u32 = 0x0023_d570;

    /// The switch's jump table (states 1..=0x38) in `.rodata`.
    pub const JUMP_TABLE: u32 = 0x0047_c1a0;

    /// The word at `gp - 0x478` that most states set to 1 or clear.
    pub const GP_FLAG: u32 = 0x478;

    /// The word at +0xb4 that `MCManager_BuildSaveTitle` and
    /// `MCManager_FormatSaveDesc` switch on (0, 2 and other values).
    pub const SAVE_MODE: u32 = 0xb4;

    /// The state timer, in updates.
    pub const WAIT: u32 = 0x108;

    /// The floats at +0x110 (set to 1.0) and +0x114 (set to one of the
    /// `gp - 0x5130..=0x5148` constants).
    pub const RAMP: u32 = 0x110;

    pub const RAMP_STEP: u32 = 0x114;

    /// The UStr buffer state 0xf prints the save's file name into.
    pub const SAVE_NAME: u32 = 0x134;

    /// `"%s-%s%04d"` and `"data/icon"` in `.rodata`.
    pub const NAME_FORMAT: u32 = 0x0047_c178;

    pub const ICON_PATH: u32 = 0x0047_c188;
}

/// The paths out of `MCManager_v01`'s switch, in the order the compiled
/// tails run them.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum McV01Tail {
    /// 0x23d548: clear the `gp - 0x478` word, then record the state.
    Clear,
    /// 0x23d54c: record the state at `mState` (+0x130).
    Record,
    /// 0x23d550: the state was recorded in a delay slot.
    Recorded,
    /// The state is left as it was (s0 reloaded in the delay slot).
    Kept,
}

/// `self + off`, with `self` reread from s3 (where the compiled code keeps it).
#[inline]
pub(crate) fn mcv01_at(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S3).wrapping_add(off))
}

/// The word at `gp - 0x478`.
#[inline]
pub(crate) fn mcv01_flag(c: &Cpu) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_sub(mc_v01::GP_FLAG))
}

/// `mState = state` (the state is in s4).
#[inline]
pub(crate) fn mcv01_record(c: &mut Cpu) {
    let state = c.g[S4][0];
    mcv01_at(c, mcus::STATE).sw(c, state);
}

/// `lui at, 0x3f80; mtc1 at, $f`: 1.0f in FPU register `f`.
#[inline]
pub(crate) fn mcv01_one(c: &mut Cpu, f: usize) {
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, f);
}

/// `lwc1 $f, -off(gp)`.
#[inline]
pub(crate) fn mcv01_gp_float(c: &mut Cpu, f: usize, off: u32) {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lwc1(f, a);
}

/// 0x23d404: f0 and f1 to +0x110 and +0x114.
pub(crate) fn mcv01_ramp(c: &mut Cpu) {
    c.cycles += 3;
    let a = mcv01_at(c, mc_v01::RAMP).0;
    c.swc1(0, a);
    let a = mcv01_at(c, mc_v01::RAMP_STEP).0;
    c.swc1(1, a);
}

/// 0x23d394: the card driver call shared by several states (the driver in a2,
/// its vtable in v0): `a0 = a2 + vt[off].delta`, call `vt[off].fn`, then record
/// the state.
pub(crate) fn mcv01_card_call(c: &mut Cpu, off: u32) -> Flow {
    c.cycles += 4;
    let vt = r32(c, V0);
    c.g[A0][0] = Addr(vt.wrapping_add(off)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(off + 4)).lw(c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x0023_d39c)?;
    mcv01_record(c);
    Ok(())
}

/// 0x23d35c: the state timer = v0, call the callback in v1, record the state.
pub(crate) fn mcv01_wait_call(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let target = r32(c, V1);
    let wait = c.g[V0][0];
    mcv01_at(c, mc_v01::WAIT).sw(c, wait);
    call_ptr(c, target, 0x0023_d364)?;
    mcv01_record(c);
    Ok(())
}

/// 0x23d3f0: call the callback in v0, then 1.0f and the `gp - 0x5138`
/// constant to +0x110/+0x114.
pub(crate) fn mcv01_call_ramp(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let target = r32(c, V0);
    call_ptr(c, target, 0x0023_d3f8)?;
    mcv01_one(c, 0);
    mcv01_gp_float(c, 1, 0x5138);
    mcv01_ramp(c);
    Ok(())
}

/// 0x23d540: call the callback in v0 (the `gp - 0x478` word is cleared after).
pub(crate) fn mcv01_call_clear(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let target = r32(c, V0);
    call_ptr(c, target, 0x0023_d548)
}

/// Call the callback word at `self + off` returning to `ret`, then record the
/// state.
pub(crate) fn mcv01_callback(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = mcv01_at(c, off).lw(c);
    let target = r32(c, V0);
    call_ptr(c, target, ret)?;
    mcv01_record(c);
    Ok(())
}

/// State 0xf's format string: `s2 = self + 0x134`, `s1 = sp + 0x400`,
/// `UStr_FromAscii_2(s1, "%s-%s%04d")` returning to `ret`, then
/// `s0 = sp + 0x200`.
pub(crate) fn mcv01_name_format(c: &mut Cpu, ret: u32) -> Flow {
    c.g[S2][0] = sx(r32(c, S3).wrapping_add(mc_v01::SAVE_NAME));
    c.g[S1][0] = sx(sp(c).wrapping_add(0x400));
    c.g[A1][0] = sx(mc_v01::NAME_FORMAT);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, callee::USTR_FROM_ASCII, ret)?;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x200));
    Ok(())
}

/// The shared tails (0x23d548, 0x23d54c, 0x23d550) and the epilogue: reload
/// s0..s4 and `ra`, pop the 0x660 frame and return.
pub(crate) fn mcv01_finish(c: &mut Cpu, frame: Frame, tail: McV01Tail) {
    let mut tail = tail;
    if tail == McV01Tail::Clear {
        c.cycles += 1;
        mcv01_flag(c).sw(c, 0);
        tail = McV01Tail::Record;
    }
    if tail == McV01Tail::Record {
        c.cycles += 1;
        mcv01_record(c);
        tail = McV01Tail::Recorded;
    }
    if tail == McV01Tail::Recorded {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x650);
    c.cycles += 7;
    frame.restore(c, S1, 0x640);
    frame.restore(c, S2, 0x630);
    frame.restore(c, S3, 0x620);
    frame.restore(c, S4, 0x610);
    frame.restore_ra(c, 0x600);
    frame.close(c);
}

/// `void MCManager_v01(MCManager *self, int state)` (0x23cf38): the manager's
/// virtual method 1 (slot +8 of the vtable at `self+0x748`), which enters
/// `state`: it does that state's entry work, then records the state in
/// `mState` (+0x130).
///
/// The entry work is a switch over states 1..=0x38 through the jump table at
/// 0x47c1a0 (read from guest memory, as the compiled `jr` does): resetting the
/// pending-operation words and the update step (`MCManager_SetMode`, state 1),
/// starting the state timer (+0x108) and the floats at +0x110/+0x114, setting
/// or clearing the word at `gp - 0x478`, calling the screen callbacks stored
/// in the manager's first words, and calling the card driver at +0x434: the
/// busy poll, card info, the directory list, and the slots the format (0xb,
/// 0x28), delete (0xe, 0x19), save (0xf) and load (0x1f) states start with.
/// State 0xf first prints the save's file name into +0x134 as `"%s-%s%04d"`
/// (product code, save type tag, number) and passes `"data/icon"` to driver
/// slot 0x138, then the name to slot 0x118. States 0x32 and 0x34, and state 0x33
/// while the card is busy (it posts state 0x35 instead), leave `mState` as it
/// was. Other states, and values outside 1..=0x38, only record the state.
pub fn mc_manager_v01(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x660 };
        let tail = loop {
            c.cycles += 12;
            Frame::open(c, 0x660);
            frame.save(c, S4, 0x610);
            c.g[S4][0] = c.g[A1][0];
            frame.save(c, S3, 0x620);
            c.g[V1][0] = sx(r32(c, S4).wrapping_sub(1));
            frame.save(c, S0, 0x650);
            frame.save(c, S1, 0x640);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S2, 0x630);
            c.g[V0][0] = (c.g[V1][0] < 0x38) as u64;
            let in_table = c.g[V0][0] != 0;
            frame.save_ra(c, 0x600);
            if !in_table {
                break McV01Tail::Record;
            }
            c.cycles += 7;
            c.g[V0][0] = 0x48_0000;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(mc_v01::JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let block = r32(c, A0);
            let tail = match block {
                mc_v01::ENTRY => {
                    // A table word naming the entry restarts the function
                    // (prologue and all), after any due event.
                    if event_due(c) {
                        deliver_event_here(c, mc_v01::ENTRY);
                    }
                    continue;
                }
                0x0023_cf84 => {
                    // State 1: update step 1, clear the pending operations.
                    c.cycles += 10;
                    mcv01_flag(c).sw(c, 0);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 1;
                    callee::MC_MANAGER_SET_MODE.call(c, 0x0023_cf94)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    mcv01_at(c, 0xc4).sw(c, 0);
                    let none = c.g[V0][0];
                    mcv01_at(c, 0xfc).sw(c, none);
                    mcv01_at(c, mcus::SAVE).sw(c, 0);
                    mcv01_at(c, mcus::DELETE).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_cfac => {
                    // State 0x31: ask for the selected slot's card info.
                    c.cycles += 6;
                    mcv01_at(c, mcus::CARD_CHANGED).sw(c, 0);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = mcv01_at(c, mcus::SELECTED).lw(c);
                    callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0023_cfbc)?;
                    c.g[V0][0] = 1;
                    c.cycles += 2;
                    let one = c.g[V0][0];
                    mcv01_at(c, mcus::CHECK_CARD).sw(c, one);
                    McV01Tail::Record
                }
                0x0023_cfc4 => {
                    // State 0x32: reread the selected slot's directory; clear
                    // its plugged-in flag when driver slot 0x1a8 says no; run
                    // the callback at +0x90. mState is left alone.
                    c.cycles += 12;
                    c.g[V0][0] = mcv01_at(c, mcus::SELECTED).lw(c);
                    c.g[A0][0] = c.g[S3][0];
                    let slot = c.g[V0][0];
                    mcv01_at(c, mcus::CURR_CHECK).sw(c, slot);
                    callee::MC_MANAGER_READ_CURRENT_DIR_INFO.call(c, 0x0023_cfd4)?;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0023_cfec)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 6;
                        c.g[V0][0] = mcv01_at(c, mcus::CURR_CHECK).lw(c);
                        c.g[V1][0] = mcus::DEVICE_SIZE;
                        c.mult(0, A0, V0, V1);
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                        Addr(r32(c, V0).wrapping_add(mcus::DEVICE_PLUGGED_IN)).sw(c, 0);
                    }
                    c.g[V0][0] = mcv01_at(c, 0x90).lw(c);
                    c.cycles += 4;
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d014)?;
                    McV01Tail::Kept
                }
                0x0023_d01c => {
                    // State 0x33: while the card is busy post state 0x35
                    // (mState left alone), else ask for the directory list.
                    c.cycles += 9;
                    mcv01_at(c, mcus::CARD_CHANGED).sw(c, 0);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_d038)?;
                    let idle = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x35;
                    if !idle {
                        c.cycles += 7;
                        c.g[V0][0] = mcv01_at(c, mcus::MSG).lw(c);
                        let vt = r32(c, V0);
                        c.g[A0][0] = Addr(vt.wrapping_add(8)).lh(c);
                        c.g[V1][0] = Addr(vt.wrapping_add(0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0023_d054)?;
                        McV01Tail::Kept
                    } else {
                        c.cycles += 6;
                        let m = r32(c, S3);
                        mcus_load_card(c, m, A2);
                        c.g[A1][0] = sx(r32(c, S3).wrapping_add(0x1b4));
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        mcv01_card_call(c, 0x38)?;
                        McV01Tail::Recorded
                    }
                }
                0x0023_d074 => {
                    // State 0x34: the callback at +0x94; mState is left alone.
                    c.cycles += 5;
                    c.g[V0][0] = mcv01_at(c, 0x94).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d080)?;
                    McV01Tail::Kept
                }
                0x0023_d088 => {
                    // State 2: wait 90 updates.
                    c.cycles += 6;
                    mcv01_one(c, 0);
                    c.g[V0][0] = 0x5a;
                    mcv01_gp_float(c, 1, 0x5148);
                    let wait = c.g[V0][0];
                    mcv01_at(c, mc_v01::WAIT).sw(c, wait);
                    mcv01_ramp(c);
                    McV01Tail::Record
                }
                0x0023_d0a0 => {
                    // State 3: unless the card is busy or a check is already
                    // pending, start a check of the selected slot.
                    c.cycles += 8;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_d0b8)?;
                    if c.g[V0][0] != 0 {
                        mcv01_record(c);
                        McV01Tail::Recorded
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = mcv01_at(c, mcus::CHECK_CARD).lw(c);
                        if c.g[V0][0] != 0 {
                            mcv01_record(c);
                            McV01Tail::Recorded
                        } else {
                            c.cycles += 8;
                            let m = r32(c, S3);
                            mcus_load_card(c, m, A2);
                            c.g[A1][0] = mcv01_at(c, mcus::SELECTED).lw(c);
                            mcus_card_call(
                                c,
                                A2,
                                V0,
                                V1,
                                mcus::VT_SET_CHECK_DEVICE,
                                0x0023_d0e8,
                            )?;
                            c.g[V0][0] = 1;
                            c.cycles += 2;
                            let one = c.g[V0][0];
                            mcv01_at(c, mcus::CHECK_CARD).sw(c, one);
                            McV01Tail::Record
                        }
                    }
                }
                0x0023_d0f4 => {
                    // State 0x18: delete pending; the callback at +0x54.
                    c.cycles += 11;
                    mcv01_one(c, 1);
                    c.g[V0][0] = 1;
                    mcv01_gp_float(c, 0, 0x5144);
                    c.g[V1][0] = mcv01_at(c, 0x54).lw(c);
                    let one = c.g[V0][0];
                    mcv01_at(c, mcus::DELETE).sw(c, one);
                    let a = mcv01_at(c, mc_v01::RAMP).0;
                    c.swc1(1, a);
                    let a = mcv01_at(c, mc_v01::RAMP_STEP).0;
                    c.swc1(0, a);
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0023_d118)?;
                    mcv01_record(c);
                    McV01Tail::Recorded
                }
                0x0023_d120 => {
                    // State 6: list the card's directory into +0x1b4.
                    c.cycles += 7;
                    mcv01_at(c, mcus::CARD_CHANGED).sw(c, 0);
                    c.g[A1][0] = sx(r32(c, S3).wrapping_add(0x1b4));
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    mcv01_card_call(c, 0x38)?;
                    McV01Tail::Recorded
                }
                0x0023_d13c => {
                    // State 0xb: start the format (driver slot 0x40), wait 120.
                    c.cycles += 16;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    c.g[V0][0] = 1;
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    mcus_card_call(c, A1, V1, V0, 0x40, 0x0023_d15c)?;
                    mcv01_one(c, 1);
                    c.g[V0][0] = 0x78;
                    mcv01_gp_float(c, 0, 0x5140);
                    let a = mcv01_at(c, mc_v01::RAMP).0;
                    c.swc1(1, a);
                    let a = mcv01_at(c, mc_v01::RAMP_STEP).0;
                    c.swc1(0, a);
                    let wait = c.g[V0][0];
                    mcv01_at(c, mc_v01::WAIT).sw(c, wait);
                    McV01Tail::Record
                }
                0x0023_d17c => {
                    // State 0xd: build the save title for the file cursor,
                    // then (as state 0x23) clear the timer.
                    c.cycles += 7;
                    c.g[V0][0] = 1;
                    c.g[A1][0] = mcv01_at(c, mcus::FILE_CURSOR).lw(c);
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    c.g[A0][0] = c.g[S3][0];
                    callee::MC_MANAGER_BUILD_SAVE_TITLE.call(c, 0x0023_d190)?;
                    mcv01_at(c, 0x10c).sw(c, 0);
                    mcv01_at(c, 0xe4).sw(c, 0);
                    c.cycles += 2;
                    mcv01_at(c, mc_v01::WAIT).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d198 => {
                    // State 0x23: clear the timer.
                    c.cycles += 2;
                    mcv01_at(c, mc_v01::WAIT).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d1a0 => {
                    // State 0xe: delete pending; driver slot 0x50 with the
                    // file cursor.
                    c.cycles += 7;
                    c.g[V1][0] = 1;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    let one = c.g[V1][0];
                    mcv01_at(c, mcus::DELETE).sw(c, one);
                    c.g[A1][0] = mcv01_at(c, mcus::FILE_CURSOR).lw(c);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    mcv01_flag(c).sw(c, one);
                    c.cycles += 2;
                    mcv01_card_call(c, 0x50)?;
                    McV01Tail::Recorded
                }
                0x0023_d1bc => {
                    // State 0xf: print the save's file name into +0x134
                    // (`"%s-%s%04d"`: product code and type tag, numbered by
                    // the file cursor + 1; save mode 2 swaps the two and
                    // numbers it 1), then driver slot 0x138 with
                    // `"data/icon"` and slot 0x118 with the name.
                    c.cycles += 3;
                    c.g[V1][0] = mcv01_at(c, mc_v01::SAVE_MODE).lw(c);
                    let mode = c.g[V1][0];
                    c.g[V0][0] = 2;
                    if mode == 0 {
                        c.cycles += 13;
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = 0x100;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[S3][0];
                        callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_d1dc)?;
                        mcv01_name_format(c, 0x0023_d1f4)?;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = c.g[SP][0];
                        if mode != c.g[V0][0] {
                            c.cycles += 11;
                            c.g[A3][0] = 0x100;
                            c.g[A1][0] = 1;
                            c.g[A0][0] = c.g[S3][0];
                            callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_d268)?;
                            mcv01_name_format(c, 0x0023_d280)?;
                        } else {
                            c.cycles += 21;
                            c.g[S1][0] = sx(sp(c).wrapping_add(0x400));
                            c.g[A1][0] = sx(mc_v01::NAME_FORMAT);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, callee::USTR_FROM_ASCII, 0x0023_d218)?;
                            c.g[S2][0] = sx(r32(c, S3).wrapping_add(mc_v01::SAVE_NAME));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = 0x200;
                            callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x0023_d228)?;
                            c.g[S0][0] = sx(sp(c).wrapping_add(0x200));
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = 2;
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = 0x200;
                            callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_d240)?;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[SP][0];
                            c.g[A0][0] = c.g[S2][0];
                            c.g[T0][0] = 1;
                        }
                    }
                    if mode != 2 {
                        // 0x23d284: the product code, numbered cursor + 1.
                        c.cycles += 9;
                        c.g[A1][0] = 0x100;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x0023_d290)?;
                        c.g[T0][0] = mcv01_at(c, mcus::FILE_CURSOR).lw(c);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A3][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    }
                    // 0x23d2a8: UStr_Sprintf(self+0x134, fmt, a2, a3, t0).
                    c.cycles += 20;
                    call_ptr(c, callee::U_STR_SPRINTF.addr, 0x0023_d2b0)?;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A3);
                    c.g[A1][0] = sx(mc_v01::ICON_PATH);
                    c.g[A2][0] = 0;
                    mcus_card_call(c, A3, V0, V1, 0x138, 0x0023_d2d4)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V1][0] = 1;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    let one = c.g[V1][0];
                    mcv01_flag(c).sw(c, one);
                    mcv01_at(c, 0x118).sw(c, one);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    mcv01_card_call(c, 0x118)?;
                    McV01Tail::Recorded
                }
                0x0023_d2f8 => {
                    // State 0x11: wait 60 updates; the callback at +0x0.
                    c.cycles += 3;
                    c.g[V0][0] = 0x3c;
                    c.g[V1][0] = mcv01_at(c, 0).lw(c);
                    mcv01_wait_call(c)?;
                    McV01Tail::Recorded
                }
                0x0023_d304 => {
                    // State 0x1f: driver slot 0x70 with the file cursor and
                    // +0x234; word 0x4d at +0xe8; the callback at +0x14.
                    c.cycles += 20;
                    mcv01_one(c, 1);
                    c.g[A2][0] = sx(r32(c, S3).wrapping_add(0x234));
                    mcv01_gp_float(c, 0, 0x513c);
                    let a = mcv01_at(c, mc_v01::RAMP).0;
                    c.swc1(1, a);
                    let a = mcv01_at(c, mc_v01::RAMP_STEP).0;
                    c.swc1(0, a);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A3);
                    c.g[A1][0] = mcv01_at(c, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A3, V0, V1, 0x70, 0x0023_d338)?;
                    c.g[A0][0] = 0x4d;
                    mcv01_at(c, 0x10c).sw(c, 0);
                    c.g[V0][0] = mcv01_at(c, 0x14).lw(c);
                    let target = r32(c, V0);
                    let word = c.g[A0][0];
                    mcv01_at(c, 0xe8).sw(c, word);
                    call_ptr(c, target, 0x0023_d34c)?;
                    mcv01_record(c);
                    McV01Tail::Recorded
                }
                0x0023_d354 => {
                    // State 0x20: wait 60 updates; the callback at +0x18.
                    c.cycles += 2;
                    c.g[V0][0] = 0x3c;
                    c.g[V1][0] = mcv01_at(c, 0x18).lw(c);
                    mcv01_wait_call(c)?;
                    McV01Tail::Recorded
                }
                0x0023_d36c => {
                    // State 0x19: delete pending, wait 60 updates; driver
                    // slot 0x50 with the file cursor.
                    c.cycles += 8;
                    c.g[V0][0] = 1;
                    c.g[V1][0] = 0x3c;
                    let one = c.g[V0][0];
                    mcv01_at(c, mcus::DELETE).sw(c, one);
                    mcv01_flag(c).sw(c, one);
                    let wait = c.g[V1][0];
                    mcv01_at(c, mc_v01::WAIT).sw(c, wait);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcv01_at(c, mcus::FILE_CURSOR).lw(c);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    c.cycles += 2;
                    mcv01_card_call(c, 0x50)?;
                    McV01Tail::Recorded
                }
                0x0023_d3a4 => {
                    // State 0x1b: no delete pending.
                    c.cycles += 2;
                    mcv01_at(c, mcus::DELETE).sw(c, 0);
                    McV01Tail::Clear
                }
                0x0023_d3ac => {
                    // State 0x1c: no delete pending; the callback at +0x34.
                    c.cycles += 6;
                    mcv01_at(c, mcus::DELETE).sw(c, 0);
                    c.g[V0][0] = mcv01_at(c, 0x34).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d3bc)?;
                    mcv01_flag(c).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d3c4 => {
                    // State 0x13: word 1 at +0xdc, -1 at +0xfc.
                    c.cycles += 6;
                    mcv01_flag(c).sw(c, 0);
                    c.g[V0][0] = 1;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    let one = c.g[V0][0];
                    mcv01_at(c, 0xdc).sw(c, one);
                    let none = c.g[V1][0];
                    mcv01_at(c, 0xfc).sw(c, none);
                    McV01Tail::Record
                }
                0x0023_d3dc => {
                    // State 0xa: the callback at +0x48, then the floats.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x48).lw(c);
                    mcv01_call_ramp(c)?;
                    McV01Tail::Record
                }
                0x0023_d3e4 => {
                    // State 0xc: the callback at +0x4c.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x4c).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d3ec => {
                    // State 9: the callback at +0x44, then the floats.
                    c.cycles += 1;
                    c.g[V0][0] = mcv01_at(c, 0x44).lw(c);
                    mcv01_call_ramp(c)?;
                    McV01Tail::Record
                }
                0x0023_d410 => {
                    // State 0x14: the callback at +0x10.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x10).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d418 => {
                    // State 0x15: the callback at +0x60.
                    mcv01_callback(c, 0x60, 0x0023_d424)?;
                    McV01Tail::Recorded
                }
                0x0023_d42c => {
                    // State 0x17: the callback at +0x5c.
                    mcv01_callback(c, 0x5c, 0x0023_d438)?;
                    McV01Tail::Recorded
                }
                0x0023_d440 => {
                    // State 4: the callback at +0x3c.
                    mcv01_callback(c, 0x3c, 0x0023_d44c)?;
                    McV01Tail::Recorded
                }
                0x0023_d454 => {
                    // State 0x24: the callback at +0x24, clear the timer.
                    c.cycles += 5;
                    c.g[V0][0] = mcv01_at(c, 0x24).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d460)?;
                    mcv01_at(c, mc_v01::WAIT).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d468 => {
                    // State 0x25: the callback at +0x40.
                    mcv01_callback(c, 0x40, 0x0023_d474)?;
                    McV01Tail::Recorded
                }
                0x0023_d47c => {
                    // State 0x16: the callback at +0x8c.
                    mcv01_callback(c, 0x8c, 0x0023_d488)?;
                    McV01Tail::Recorded
                }
                0x0023_d490 => {
                    // State 0x27: the callback at +0x48.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x48).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d498 => {
                    // State 0x28: wait 120 updates; driver slot 0x40.
                    c.cycles += 17;
                    c.g[V0][0] = 1;
                    mcv01_one(c, 0);
                    mcv01_gp_float(c, 1, 0x5134);
                    c.g[V1][0] = 0x78;
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    let a = mcv01_at(c, mc_v01::RAMP).0;
                    c.swc1(0, a);
                    let a = mcv01_at(c, mc_v01::RAMP_STEP).0;
                    c.swc1(1, a);
                    let wait = c.g[V1][0];
                    mcv01_at(c, mc_v01::WAIT).sw(c, wait);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V0, V1, 0x40, 0x0023_d4d4)?;
                    mcv01_record(c);
                    McV01Tail::Recorded
                }
                0x0023_d4dc => {
                    // State 0x29: the callback at +0x4c.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x4c).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d4e4 => {
                    // State 0x2a: the callback at +0x50.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x50).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d4ec => {
                    // State 0x2b: the callback at +0x54.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0x54).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d4f4 => {
                    // State 0x2c: wait 60 updates, clear the word at +0x438.
                    c.cycles += 11;
                    c.g[V0][0] = 1;
                    mcv01_one(c, 0);
                    mcv01_gp_float(c, 1, 0x5130);
                    c.g[V1][0] = 0x3c;
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    let a = mcv01_at(c, mc_v01::RAMP).0;
                    c.swc1(0, a);
                    let a = mcv01_at(c, mc_v01::RAMP_STEP).0;
                    c.swc1(1, a);
                    let wait = c.g[V1][0];
                    mcv01_at(c, mc_v01::WAIT).sw(c, wait);
                    mcv01_at(c, 0x438).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d520 => {
                    // States 5 and 0x30: the callback at +0x58.
                    mcv01_callback(c, 0x58, 0x0023_d52c)?;
                    McV01Tail::Recorded
                }
                0x0023_d534 => {
                    // State 0x37: the callback at +0xa0.
                    c.cycles += 2;
                    c.g[V0][0] = mcv01_at(c, 0xa0).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d53c => {
                    // State 0x38: the callback at +0xa8.
                    c.cycles += 1;
                    c.g[V0][0] = mcv01_at(c, 0xa8).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                // States 0x2d, 0x2e.
                0x0023_d548 => McV01Tail::Clear,
                // States without entry work.
                0x0023_d54c => McV01Tail::Record,
                _ => {
                    c.jump_out(block, mc_v01::ENTRY, mc_v01::END);
                    return Ok(());
                }
            };
            break tail;
        };
        mcv01_finish(c, frame, tail);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check_paths;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::movie::movieplayer::tests::globals;
    use crate::game::movie::movieplayer::tests::mgr;
    use crate::game::movie::movieplayer::tests::ret_slot;
    use crate::game::movie::movieplayer::tests::w32;
    use crate::game::movie::movieplayer::tests::RET_ONE;
    use crate::game::movie::movieplayer::tests::RET_ZERO;

    /// Extra `(state, variant)` cases after states 0..=0x39 (variant 0): the
    /// inner branches of states 3, 0xd, 0xf, 0x32 and 0x33, and two more
    /// values outside the table.
    const MCV01_EXTRA: [(u32, u32); 10] = [
        (3, 1),
        (3, 2),
        (0xd, 1),
        (0xd, 2),
        (0xf, 1),
        (0xf, 2),
        (0x32, 1),
        (0x33, 1),
        (0xffff_ffff, 0),
        (0x8000_0000, 0),
    ];

    /// Cases per pass: states 0..=0x39 plus [`MCV01_EXTRA`].
    const MCV01_CASES: u32 = 0x3a + MCV01_EXTRA.len() as u32;

    /// `(state, variant)` for case `i` (`seed % MCV01_CASES`).
    fn mcv01_case(i: u32) -> (u32, u32) {
        if i < 0x3a {
            (i, 0)
        } else {
            MCV01_EXTRA[(i - 0x3a) as usize]
        }
    }

    /// The manager fixture (`mgr`) with the game image loaded (the jump table,
    /// the format strings, the `gp` floats), every callback word and driver
    /// slot pointed at `RET_ZERO`/`RET_ONE` (`leaf`: fixed cycle costs) or at
    /// the fakes (calls logged, some unwinding), and every word the function
    /// writes pre-filled non-zero.
    ///
    /// Variants: state 3 busy (0) / idle with a check pending (1) / idle
    /// (2); state 0xd save mode 0/1/2; state 0xf save mode 0 (0) / 2 (1) /
    /// other (2); state 0x32 driver slot 0x1a8 true (0) / false (1); state
    /// 0x33 busy (0) / idle (1).
    fn mcv01_fixture(c: &mut Cpu, seed: u32, salt: u64, leaf: bool) {
        let (state, variant) = mcv01_case(seed % MCV01_CASES);
        let yes = if leaf { RET_ONE } else { FAKE };
        let any = if leaf { RET_ZERO } else { FAKE };
        let mut r = mgr(c, seed, salt, RET_ZERO);
        elf_image(c);
        globals(c);
        let this = DATA + 0x400;
        let vt = DATA + 0x1100;
        let crowd = DATA + 0x1200;
        for off in [
            0x28, 0x38, 0x40, 0x50, 0x70, 0x118, 0x138, 0x168, 0x180, 0x190, 0x1a8, 0x1e0,
        ] {
            ret_slot(c, vt, off, any);
        }
        ret_slot(c, crowd, 8, any);
        // The screen callbacks in the first words.
        for off in (0..=0xa8).step_by(4) {
            w32(c, this + off, if leaf { RET_ZERO } else { FAKE + off });
        }
        // Words the states write.
        for off in [
            0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xdc, 0xe4, 0xe8, 0xfc, 0x108, 0x10c, 0x110, 0x114,
            0x118, 0x130, 0x338, 0x42c, 0x438,
        ] {
            w32(c, this + off, r.next_u32() | 2);
        }
        for i in 0..4 {
            w32(c, this + mcus::DEVICE_PLUGGED_IN + 0xe0 * i, r.next_u32() | 2);
        }
        fill(c, &mut r, this + mc_v01::SAVE_NAME, 0x20);
        let top = c.g[SP][0] as u32;
        fill(c, &mut r, top - 0x660, 0x180);
        w32(c, GAME_GP - mc_v01::GP_FLAG, r.next_u32() | 2);
        // Data: the file cursor (printed as a number), the selected slot.
        w32(c, this + mcus::FILE_CURSOR, r.next_u32() % 100);
        w32(c, this + mcus::SELECTED, r.next_u32() % 4);
        match (state, variant) {
            (3, 0) | (0x33, 0) => ret_slot(c, vt, mcus::VT_BUSY, yes),
            (3, 2) => w32(c, this + mcus::CHECK_CARD, 0),
            (0x32, 0) => ret_slot(c, vt, mcus::VT_CAN_SAVE, yes),
            (0x32, 1) => ret_slot(c, vt, mcus::VT_CAN_SAVE, RET_ZERO),
            (0xd, v) => {
                w32(c, this + mc_v01::SAVE_MODE, v);
                // Save mode 1 reads +0xfc as the player index.
                w32(c, this + 0xfc, 0);
            }
            (0xf, v) => w32(c, this + mc_v01::SAVE_MODE, [0, 2, 1][v as usize]),
            _ => {}
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (c.g[A1][0] & !0xffff_ffff) | state as u64;
        c.pc = mc_v01::ENTRY;
    }

    /// Every table state, the inner branches and out-of-range states, with
    /// leaf callees so each path has a fixed cycle total. The totals below
    /// are every path except those through the string, directory and title
    /// callees (states 0xd, 0xf, 0x32), computed from the generated blocks
    /// (prologue 12 + dispatch 7, leaf call 2, tails 7/8/9/10):
    ///
    /// | cycles | states |
    /// | --- | --- |
    /// | 21 | out of range (0, 0x39, -1, 0x80000000) |
    /// | 28 | no entry work (7, 8, 0x10, 0x12, 0x1a, 0x1d, 0x1e, 0x21, 0x22, 0x26, 0x2f, 0x35, 0x36) |
    /// | 29 | 0x2d, 0x2e |
    /// | 30 | 0x23 |
    /// | 31 | 0x1b |
    /// | 33 | 0x34 |
    /// | 34 | 0x13, 4, 5, 0x15, 0x16, 0x17, 0x25, 0x30, 0x38 |
    /// | 35 | 0xc, 0x14, 0x20, 0x24, 0x27, 0x29, 0x2a, 0x2b, 0x37 |
    /// | 36 | 0x11, 0x1c |
    /// | 37 | 2, 3 busy |
    /// | 39 | 9, 0x2c |
    /// | 40 | 0xa, 3 check pending, 6, 0x18 |
    /// | 42 | 0xe |
    /// | 43 | 0x19 |
    /// | 46 | 0xb, 0x28, 0x33 busy |
    /// | 50 | 0x33 idle |
    /// | 51 | 1, 0x31, 0x1f |
    /// | 53 | 3 idle |
    #[test]
    fn mc_manager_v01_matches() {
        check_paths(
            ee::s06::f_0023cf38__generated,
            mc_manager_v01,
            MCV01_CASES,
            &[
                21, 28, 29, 30, 31, 33, 34, 35, 36, 37, 39, 40, 42, 43, 46, 50, 51, 53,
            ],
            &|c, seed| mcv01_fixture(c, seed, 0x23cf38, true),
        );
    }

    /// The same cases with the callbacks and driver slots on the fakes (their
    /// arguments are logged to RAM and compared), each case run with no
    /// unwind, an unwind at the first fake call and one at the second
    /// (`seed / MCV01_CASES % 3`).
    #[test]
    fn mc_manager_v01_calls_match() {
        fakes::covered(
            ee::s06::f_0023cf38__generated,
            mc_manager_v01,
            3 * MCV01_CASES,
            &|c, seed| {
                mcv01_fixture(c, seed, 0x23cf39, false);
                let unwind_at = match seed / MCV01_CASES % 3 {
                    0 => u32::MAX,
                    n => n - 1,
                };
                c.bus.write32(LOG_UNWIND_AT, unwind_at);
            },
        );
    }
}
