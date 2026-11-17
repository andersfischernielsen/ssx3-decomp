//! Staging module oc1041 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1041`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::fememcardcallbacks::{
    memcard_text, MEMCARD_CARD_VT_SAVE_COUNT, MEMCARD_CARD_VT_SAVE_SIZE,
};
use crate::game::movie::movieplayer::{mcus, mcus_at, mcus_callee, mcus_card_call, mcus_load_card};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_1710,
        mc_manager_get_card_error_text,
        "MCManager_GetCardErrorText",
    ),
    (
        0x0024_1b20,
        mc_manager_read_card_status,
        "MCManager_ReadCardStatus",
    ),
    (
        0x0024_1e18,
        mc_manager_get_status_text,
        "MCManager_GetStatusText",
    ),
    (0x0024_20c8, mc_manager_read_dir_info, "MCManager_ReadDirInfo"),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;

    /// `int UStr_Len_2(const u16* s)`: the length of a UTF-16 string.
    pub const USTR_LEN_2: Callee = Callee {
        addr: 0x002c_24d0,
        f: ee::s08::UStr_Len_2,
    };
}

// ---------------------------------------------------------------------------
// Card driver queries and the per-device record.

/// Card driver vtable slots (`{short delta; fn}` in the vtable of the object
/// at `MCManager+0x434`). Any of these three true: the card is reported as
/// not detected, and `MCManager_ReadDirInfo` clears the device's first word.
pub(crate) const MC_CARD_VT_NOT_DETECTED_A: u32 = 0x180;

pub(crate) const MC_CARD_VT_NOT_DETECTED_B: u32 = 0x190;

pub(crate) const MC_CARD_VT_NOT_DETECTED_C: u32 = 0x1e0;

/// False: the card is reported as not formatted.
pub(crate) const MC_CARD_VT_FORMATTED: u32 = 0x1c0;

/// True: the device's name is read (slot 0x230) and shown.
pub(crate) const MC_CARD_VT_HAS_NAME: u32 = 0x1c8;

/// `fn(card, slot, u16* name, int max_chars)`: fills the device's name.
pub(crate) const MC_CARD_VT_READ_NAME: u32 = 0x230;

/// A count `MCManager_ReadCardStatus` compares with the space a new save
/// needs; `MCManager_ReadDirInfo` keeps it for the current slot.
pub(crate) const MC_CARD_VT_FREE_SPACE: u32 = 0xa0;

/// The status flag slots of `MCManager_ReadCardStatus`: bit 0 is the low bit
/// of slot 0x188, bit 1 is set when slot 0x198 or 0x1d8 is true, bit 4 when
/// `mcus::VT_CAN_LOAD` or slot 0x1d0 is false.
pub(crate) const MC_CARD_VT_STATUS_BIT0: u32 = 0x188;

pub(crate) const MC_CARD_VT_STATUS_BIT1_A: u32 = 0x198;

pub(crate) const MC_CARD_VT_STATUS_BIT1_B: u32 = 0x1d8;

pub(crate) const MC_CARD_VT_STATUS_BIT4: u32 = 0x1d0;

/// The status word `MCManager_ReadCardStatus` returns unchanged while the
/// checked slot (`+0x42c`) is not the selected one (`+0x428`).
pub(crate) const MC_LAST_CARD_STATUS: u32 = 0x43c;

/// Words of the device record (`mcus::DEVICES + slot * mcus::DEVICE_SIZE`)
/// that `MCManager_ReadDirInfo` sets to 1 and clears on the matching
/// failure (the first word is `mcus::DEVICES` itself, cleared when the card
/// is not detected).
pub(crate) const MC_DEVICE_FORMATTED: u32 = mcus::DEVICES + 4;

pub(crate) const MC_DEVICE_HAS_NAME: u32 = mcus::DEVICES + 8;

/// The free-space count (`MC_CARD_VT_FREE_SPACE`), read for the current slot.
pub(crate) const MC_DEVICE_FREE_SPACE: u32 = mcus::DEVICES + 0x10;

/// The device name (UTF-16, 100 units).
pub(crate) const MC_DEVICE_NAME: u32 = mcus::DEVICES + 0x18;

pub(crate) const MC_DEVICE_NAME_BYTES: u64 = 200;

pub(crate) const MC_DEVICE_NAME_CHARS: u64 = 100;

/// Text ids (ASCII, hashed and looked up in the application's text database).
pub(crate) const MC_TEXT_NOT_DETECTED: u32 = 0x0047_c360;

pub(crate) const MC_TEXT_NOT_FORMATTED: u32 = 0x0047_c378;

pub(crate) const MC_TEXT_MEM_DEVICE: u32 = 0x0047_c390;

pub(crate) const MC_TEXT_CHECKING_CARD: u32 = 0x0047_c3b8;

pub(crate) const MC_TEXT_FORMATTING: u32 = 0x0047_c3d0;

pub(crate) const MC_TEXT_SAVING: u32 = 0x0047_c3e8;

pub(crate) const MC_TEXT_LOADING: u32 = 0x0047_c408;

pub(crate) const MC_TEXT_DELETING: u32 = 0x0047_c420;

/// An empty ASCII string (the status text starts out empty).
pub(crate) const MC_TEXT_EMPTY: u32 = 0x004a_29e8;

/// `MCManager_GetStatusText`'s switch: 0x38 code addresses, one per state.
pub(crate) const MC_STATUS_TEXT_TABLE: u32 = 0x0047_c440;

// ---------------------------------------------------------------------------
// Card error text.

/// `void MCManager_GetCardErrorText(MCManager* m, int slot, u16* dst)`
/// (0x241710, `cMCOverlayManager::GetDeviceDisplayString`): the text shown
/// for the card in `slot`. Not detected (slot `mcus::VT_DEVICE_OK` false, or
/// any of the three not-detected slots true) and not formatted give the
/// localised message with the slot name ("1", "2", ...) formatted in; a
/// formatted card with a name gives the device name (`+0x35c` of the device
/// record), or the "memory device" message when that name is empty; a
/// formatted card without one leaves `dst` alone.
pub fn mc_manager_get_card_error_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x240);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x210);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[S2][0] = c.g[A2][0];
        let m = r32(c, S1);
        mcus_load_card(c, m, A3);
        mcus_card_call(c, A3, V1, V0, mcus::VT_DEVICE_OK, 0x0024_1748)?;
        let device_ok = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[S0][0];
        if !device_ok {
            c.cycles += 20;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_18cc)?;
            mc_card_error_sprintf(
                c,
                frame,
                MC_TEXT_NOT_DETECTED,
                [0x0024_18ec, 0x0024_18fc, 0x0024_190c],
            )?;
            c.cycles += 1;
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }

        // Any of the three: not detected.
        c.cycles += 8;
        let mut not_detected = false;
        for (i, (slot, ret_at)) in [
            (MC_CARD_VT_NOT_DETECTED_A, 0x0024_1768),
            (MC_CARD_VT_NOT_DETECTED_B, 0x0024_178c),
            (MC_CARD_VT_NOT_DETECTED_C, 0x0024_17b0),
        ]
        .into_iter()
        .enumerate()
        {
            let m = r32(c, S1);
            mcus_load_card(c, m, A2);
            if i > 0 {
                c.cycles += 9;
                c.g[A1][0] = c.g[S0][0];
            }
            mcus_card_call(c, A2, V1, V0, slot, ret_at)?;
            let hit = c.g[V0][0] != 0;
            c.g[A2][0] = c.g[S0][0];
            if hit {
                not_detected = true;
                break;
            }
        }
        if not_detected {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_17c4)?;
            c.cycles += 15;
            mc_card_error_sprintf(
                c,
                frame,
                MC_TEXT_NOT_DETECTED,
                [0x0024_1884, 0x0024_1894, 0x0024_18a4],
            )?;
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 9;
        let m = r32(c, S1);
        mcus_load_card(c, m, A2);
        c.g[A1][0] = c.g[S0][0];
        mcus_card_call(c, A2, V1, V0, MC_CARD_VT_FORMATTED, 0x0024_17f0)?;
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1808)?;
            c.cycles += 15;
            mc_card_error_sprintf(
                c,
                frame,
                MC_TEXT_NOT_FORMATTED,
                [0x0024_1884, 0x0024_1894, 0x0024_18a4],
            )?;
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }
        let m = r32(c, S1);
        mcus_load_card(c, m, A2);

        c.cycles += 8;
        c.g[A1][0] = c.g[S0][0];
        mcus_card_call(c, A2, V1, V0, MC_CARD_VT_HAS_NAME, 0x0024_1830)?;
        let has_name = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if !has_name {
            c.cycles += 1;
            frame.restore(c, S0, 0x230);
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 11;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1844)?;
        c.g[V0][0] = mcus::DEVICE_SIZE;
        c.mult(0, V1, S0, V0);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(MC_DEVICE_NAME));
        c.g[A0][0] = c.g[S0][0];
        callee::USTR_LEN_2.call(c, 0x0024_185c)?;
        let named = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if named {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::USTR_COPY.call(c, 0x0024_18b4)?;
            frame.restore(c, S0, 0x230);
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 3;
        c.cycles += 15;
        mc_card_error_sprintf(
            c,
            frame,
            MC_TEXT_MEM_DEVICE,
            [0x0024_1884, 0x0024_1894, 0x0024_18a4],
        )?;
        mc_card_error_epilogue(c, frame);
        Ok(())
    })
}

/// `UStr_Sprintf(dst, text(key), slot_name)` into `dst` (`s2`), the slot
/// name in the frame's buffer; then reload `s0` as the compiled tail does.
/// `rets`: the hash, lookup and format calls' return addresses.
fn mc_card_error_sprintf(c: &mut Cpu, frame: Frame, key: u32, rets: [u32; 3]) -> Flow {
    memcard_text(c, key, [rets[0], rets[1]])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[SP][0];
    callee::U_STR_SPRINTF.call(c, rets[2])?;
    frame.restore(c, S0, 0x230);
    Ok(())
}

/// Reload s1, s2 and `ra`, pop the 0x240 frame and return.
fn mc_card_error_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    frame.restore(c, S1, 0x220);
    frame.restore(c, S2, 0x210);
    frame.restore_ra(c, 0x200);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Card status flags.

/// `u32 MCManager_ReadCardStatus(MCManager* m)` (0x241b20): the card's status
/// flags in `v0`. While the checked slot (`+0x42c`) is not the selected one
/// (`+0x428`) the word at `+0x43c`; otherwise, from the card driver: bit 0 the
/// low bit of slot 0x188, bit 1 slot 0x198 or 0x1d8, bit 2 "no saves and the
/// free count below the space a save needs", bit 3 the inverted low bit of
/// `mcus::VT_CAN_SAVE`, bit 4 `mcus::VT_CAN_LOAD` or slot 0x1d0 false; bits 5-8
/// clear.
pub fn mc_manager_read_card_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S2);
        c.g[V1][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);
        c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = mcus_at(m, MC_LAST_CARD_STATUS).lw(c);
            mc_card_status_epilogue(c, frame);
            return Ok(());
        }
        mcus_load_card(c, m, A1);

        // Bits 0 and 1.
        c.cycles += 15;
        c.g[S1][0] = 0;
        mcus_card_call(c, A1, V1, V0, MC_CARD_VT_STATUS_BIT0, 0x0024_1b6c)?;
        let m = r32(c, S2);
        mcus_load_card(c, m, A1);
        c.g[S0][0] = c.g[V0][0] & 1;
        mcus_card_call(c, A1, V1, V0, MC_CARD_VT_STATUS_BIT1_A, 0x0024_1b88)?;
        if c.g[V0][0] != 0 {
            c.g[S1][0] = 1;
            c.cycles += 1;
        } else {
            c.cycles += 8;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, MC_CARD_VT_STATUS_BIT1_B, 0x0024_1ba8)?;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[S1][0] = 1;
                c.cycles += 1;
            }
        }
        let m = r32(c, S2);
        mcus_load_card(c, m, A1);

        // Bit 2: no saves, and less free than a save needs.
        c.cycles += 10;
        c.g[V0][0] = sx(r32(c, S1) << 1);
        c.g[S1][0] = c.g[S0][0] | c.g[V0][0];
        c.g[S0][0] = 0;
        mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_COUNT, 0x0024_1bd8)?;
        if c.g[V0][0] == 0 {
            c.cycles += 15;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, MC_CARD_VT_FREE_SPACE, 0x0024_1bf8)?;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            c.g[S0][0] = c.g[V0][0];
            mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_SIZE, 0x0024_1c14)?;
            c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        }
        let m = r32(c, S2);
        mcus_load_card(c, m, A2);

        // Bits 3 and 4.
        c.cycles += 24;
        c.g[V0][0] = 0xffff_ffff_ffff_fffb;
        c.g[S1][0] &= c.g[V0][0];
        c.g[V1][0] = sx(r32(c, S0) << 2);
        c.g[S1][0] |= c.g[V1][0];
        c.g[S3][0] = 0;
        mcus_card_call(c, A2, A1, V0, mcus::VT_CAN_SAVE, 0x0024_1c44)?;
        let m = r32(c, S2);
        mcus_load_card(c, m, A2);
        c.g[V0][0] = (c.g[V0][0] ^ 1) & 1;
        c.g[V1][0] = 0xffff_ffff_ffff_fff7;
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[S1][0] &= c.g[V1][0];
        c.g[S0][0] = c.g[S1][0] | c.g[V0][0];
        mcus_card_call(c, A2, A1, V0, mcus::VT_CAN_LOAD, 0x0024_1c74)?;
        if c.g[V0][0] == 0 {
            c.g[S3][0] = 1;
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        } else {
            c.cycles += 8;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, MC_CARD_VT_STATUS_BIT4, 0x0024_1c94)?;
            let ok = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffef;
            if !ok {
                c.cycles += 1;
                c.g[S3][0] = 1;
                c.cycles += 1;
            }
        }
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, S3) << 4);
        c.g[S1][0] = c.g[S0][0] & c.g[V0][0];
        c.g[V0][0] = 0xffff_ffff_ffff_fe1f;
        c.g[S1][0] |= c.g[V1][0];
        c.g[V0][0] = c.g[S1][0] & c.g[V0][0];
        mc_card_status_epilogue(c, frame);
        Ok(())
    })
}

/// Reload s0..s3 and `ra`, pop the 0x50 frame and return.
fn mc_card_status_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Status text.

/// `void MCManager_GetStatusText(MCManager* m, u16* out)` (0x241e18): the
/// text for the manager's state (`+0x130`): "checking card" (states 0, 2, 3,
/// 6, 7, 13, 29, 49..=54), "not detected" (4), "formatting" (10, 11, 39, 40),
/// "saving" (15..=18), "loading" (31..=34), each with the selected slot's name
/// (`+0x428`) formatted in, or "deleting" (25, 26, 44) as is; every other
/// state leaves `out` empty. The state picks a code address from the switch's
/// table in `.rodata`, read from guest memory as the compiled `jr` does.
pub fn mc_manager_get_status_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x450);
        frame.save(c, S3, 0x410);
        frame.save(c, S0, 0x440);
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x430);
        c.g[A0][0] = c.g[S3][0];
        frame.save(c, S2, 0x420);
        frame.save_ra(c, 0x400);
        c.g[A1][0] = sx(MC_TEXT_EMPTY);
        callee::USTR_FROM_ASCII_2.call(c, 0x0024_1e48)?;
        let m = r32(c, S0);
        c.g[A2][0] = mcus_at(m, mcus::SELECTED).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1e58)?;
        let m = r32(c, S0);
        c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
        let in_table = c.g[V1][0] < 0x38;
        c.g[V0][0] = 0x48_0000;
        if !in_table {
            c.cycles += 1;
            frame.restore(c, S0, 0x440);
            mc_status_text_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V0][0] = sx(MC_STATUS_TEXT_TABLE);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let block = r32(c, A0);
        let key = match block {
            0x0024_1e80 => {
                c.cycles += 4;
                MC_TEXT_CHECKING_CARD
            }
            0x0024_1e90 => {
                // Not detected: the slot name goes to a second buffer.
                c.cycles += 23;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = sx(sp(c).wrapping_add(0x200));
                let m = r32(c, A0);
                c.g[A2][0] = mcus_at(m, mcus::SELECTED).lw(c);
                c.g[A1][0] = c.g[S2][0];
                callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1ea4)?;
                mc_status_text_sprintf(
                    c,
                    frame,
                    MC_TEXT_NOT_DETECTED,
                    S2,
                    [0x0024_1ec4, 0x0024_1ed4, 0x0024_1ee4],
                )?;
                mc_status_text_epilogue(c, frame);
                return Ok(());
            }
            0x0024_1eec => {
                c.cycles += 4;
                MC_TEXT_FORMATTING
            }
            0x0024_1efc => {
                c.cycles += 4;
                MC_TEXT_SAVING
            }
            0x0024_1f0c => {
                c.cycles += 3;
                MC_TEXT_LOADING
            }
            0x0024_1f54 => {
                // Deleting: the text as is.
                c.cycles += 15;
                memcard_text(c, MC_TEXT_DELETING, [0x0024_1f74, 0x0024_1f84])?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                callee::USTR_COPY.call(c, 0x0024_1f90)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x440);
                mc_status_text_epilogue(c, frame);
                return Ok(());
            }
            0x0024_1f90 => {
                c.cycles += 1;
                frame.restore(c, S0, 0x440);
                mc_status_text_epilogue(c, frame);
                return Ok(());
            }
            _ => {
                c.jump_out(block, 0x0024_1e18, 0x0024_1fac);
                return Ok(());
            }
        };
        c.cycles += 15;
        mc_status_text_sprintf(
            c,
            frame,
            key,
            SP,
            [0x0024_1f2c, 0x0024_1f3c, 0x0024_1f4c],
        )?;
        mc_status_text_epilogue(c, frame);
        Ok(())
    })
}

/// `UStr_Sprintf(out, text(key), slot_name)` into `out` (`s3`), the slot name
/// in the buffer that `name_reg` points at; then reload `s0` as the compiled
/// tails do. `rets`: the hash, lookup and format calls' return addresses.
fn mc_status_text_sprintf(
    c: &mut Cpu,
    frame: Frame,
    key: u32,
    name_reg: usize,
    rets: [u32; 3],
) -> Flow {
    memcard_text(c, key, [rets[0], rets[1]])?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[name_reg][0];
    callee::U_STR_SPRINTF.call(c, rets[2])?;
    frame.restore(c, S0, 0x440);
    Ok(())
}

/// Reload s1..s3 and `ra`, pop the 0x450 frame and return.
fn mc_status_text_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x430);
    frame.restore(c, S2, 0x420);
    frame.restore(c, S3, 0x410);
    frame.restore_ra(c, 0x400);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Device record.

/// `void MCManager_ReadDirInfo(MCManager* m, int slot)` (0x2420c8): refresh the
/// device record of `slot` (`mcus::DEVICES + slot * 0xe0`). Sets its first
/// four words to 1 and clears its name, then asks the card driver: any
/// not-detected slot true clears the first word, not formatted clears
/// `MC_DEVICE_FORMATTED`, no name clears `MC_DEVICE_HAS_NAME` (each stops
/// there); otherwise stores the free count when `slot` is the current one
/// (`+0x42c`) and reads the device name (at most 100 units).
pub fn mc_manager_read_dir_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x70);
        c.g[A3][0] = 1;
        frame.save(c, S1, 0x50);
        c.g[A2][0] = MC_DEVICE_NAME_BYTES;
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S5][0] = mcus::DEVICE_SIZE;
        frame.save(c, S0, 0x60);
        c.mult(0, V1, S1, S5);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(MC_DEVICE_FORMATTED));
        frame.save(c, S3, 0x30);
        c.g[T0][0] = sx(r32(c, S0).wrapping_add(MC_DEVICE_HAS_NAME));
        frame.save(c, S4, 0x20);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
        Addr(r32(c, V0)).off(mcus::DEVICES).sw(c, c.g[A3][0]);
        c.g[S2][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
        c.g[S3][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
        Addr(r32(c, S2)).sw(c, c.g[A3][0]);
        c.g[S4][0] = sx(r32(c, V0).wrapping_add(MC_DEVICE_NAME));
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
        Addr(r32(c, S3)).sw(c, c.g[A3][0]);
        c.g[A0][0] = c.g[S4][0];
        Addr(r32(c, V1)).off(mcus::DEVICE_PLUGGED_IN).sw(c, c.g[A3][0]);
        // EA_MemSet(name, 0, 200)
        call_ptr(c, mcus_callee::MEM_FILL, 0x0024_213c)?;

        // Any of the three: not detected.
        let mut not_detected = false;
        for (i, (slot, ret_at)) in [
            (MC_CARD_VT_NOT_DETECTED_A, 0x0024_2158),
            (MC_CARD_VT_NOT_DETECTED_B, 0x0024_217c),
            (MC_CARD_VT_NOT_DETECTED_C, 0x0024_21a0),
        ]
        .into_iter()
        .enumerate()
        {
            if i > 0 {
                c.cycles += 9;
            }
            let m = r32(c, S0);
            mcus_load_card(c, m, A2);
            c.g[A1][0] = c.g[S1][0];
            mcus_card_call(c, A2, V1, V0, slot, ret_at)?;
            let hit = c.g[V0][0] != 0;
            c.g[V0][0] = mcus::DEVICE_SIZE;
            if hit {
                not_detected = true;
                break;
            }
        }
        if not_detected {
            c.cycles += 4;
            c.mult(0, V1, S1, V0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
            Addr(r32(c, V0)).off(mcus::DEVICES).sw(c, 0);
            mc_read_dir_info_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 9;
        let m = r32(c, S0);
        mcus_load_card(c, m, A2);
        c.g[A1][0] = c.g[S1][0];
        mcus_card_call(c, A2, V1, V0, MC_CARD_VT_FORMATTED, 0x0024_21d4)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            Addr(r32(c, S2)).sw(c, 0);
            mc_read_dir_info_epilogue(c, frame);
            return Ok(());
        }
        let m = r32(c, S0);
        mcus_load_card(c, m, A2);

        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        mcus_card_call(c, A2, V1, V0, MC_CARD_VT_HAS_NAME, 0x0024_21fc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            Addr(r32(c, S3)).sw(c, 0);
            mc_read_dir_info_epilogue(c, frame);
            return Ok(());
        }
        let m = r32(c, S0);
        c.g[V0][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);

        c.cycles += 2;
        if c.g[S1][0] == c.g[V0][0] {
            // The current slot: keep its free count.
            c.cycles += 11;
            let m = r32(c, S0);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, MC_CARD_VT_FREE_SPACE, 0x0024_222c)?;
            let m = r32(c, S0);
            c.g[V1][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);
            c.mult(0, A0, V1, S5);
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
            Addr(r32(c, V1)).off(MC_DEVICE_FREE_SPACE).sw(c, c.g[V0][0]);
        }
        let m = r32(c, S0);
        mcus_load_card(c, m, T0);

        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[A3][0] = MC_DEVICE_NAME_CHARS;
        mcus_card_call(c, T0, V0, V1, MC_CARD_VT_READ_NAME, 0x0024_2260)?;
        mc_read_dir_info_epilogue(c, frame);
        Ok(())
    })
}

/// Reload s0..s5 and `ra`, pop the 0x70 frame and return.
fn mc_read_dir_info_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::game::fe::fememcardcallbacks::MEMCARD_APP_GP;
    use crate::game::fe::fememcardcallbacks::MEMCARD_APP_TEXT_DB;
    use crate::game::fe::fememcardcallbacks::MEMCARD_TEXT_DB_VT_FIND;
    use crate::game::movie::movieplayer::tests::mgr;
    use crate::game::movie::movieplayer::tests::ret_slot;
    use crate::game::movie::movieplayer::tests::w32;
    use crate::game::movie::movieplayer::tests::wh;
    use crate::game::movie::movieplayer::tests::RET_ONE;
    use crate::game::movie::movieplayer::tests::RET_ZERO;

    /// `_close` (a newlib stub): a translated leaf returning -1 in 2 cycles.
    pub(crate) const RET_MINUS_ONE: u32 = 0x0042_4488;

    /// The manager and its card driver vtable in the `mgr` fixture.
    pub(crate) const MC_FIXTURE_THIS: u32 = DATA + 0x400;

    pub(crate) const MC_FIXTURE_CARD_VT: u32 = DATA + 0x1100;

    /// The text the lookup fake returns: `"<%s>"` (UTF-16).
    pub(crate) const MC_TEXT_FIXTURE_FORMAT: u32 = DATA + 0x3200;

    /// Write `s` as a NUL-terminated UTF-16 string.
    pub(crate) fn mc_text_fixture_put(c: &mut Cpu, at: u32, s: &str) {
        let units: Vec<u16> = s.encode_utf16().collect();
        for (i, u) in units.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *u);
        }
        c.bus.write16(at + 2 * units.len() as u32, 0);
    }

    /// The application's text database for the card texts: the world's
    /// `+0x8c` points at a database whose find slot (`+0x20` of its vtable,
    /// `this` adjusted by `delta`) is the logging fake, so each lookup logs the
    /// id's hash and returns `"<%s>"`; with `unwind` the lookup unwinds.
    pub(crate) fn mc_text_fixture_db(c: &mut Cpu, delta: u16, unwind: bool) {
        let world = c.bus.read32(GAME_GP.wrapping_add(MEMCARD_APP_GP));
        let db = DATA + 0x3000;
        let vt = DATA + 0x3100;
        w32(c, world + MEMCARD_APP_TEXT_DB, db);
        w32(c, db + 4, vt);
        wh(c, vt + MEMCARD_TEXT_DB_VT_FIND, delta);
        w32(c, vt + MEMCARD_TEXT_DB_VT_FIND + 4, FAKE);
        mc_text_fixture_put(c, MC_TEXT_FIXTURE_FORMAT, "<%s>");
        w32(c, LOG_RESULTS, MC_TEXT_FIXTURE_FORMAT);
        w32(c, LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
    }

    /// The card driver slots `MCManager_GetCardErrorText` asks, in call order.
    pub(crate) const GET_CARD_ERROR_TEXT_SLOTS: [u32; 6] = [
        mcus::VT_DEVICE_OK,
        MC_CARD_VT_NOT_DETECTED_A,
        MC_CARD_VT_NOT_DETECTED_B,
        MC_CARD_VT_NOT_DETECTED_C,
        MC_CARD_VT_FORMATTED,
        MC_CARD_VT_HAS_NAME,
    ];

    /// Their answers per arm (`seed % 8`); slots an arm never reaches answer 0.
    pub(crate) const GET_CARD_ERROR_TEXT_ARMS: [[u32; 6]; 8] = [
        // 0: no device: not detected.
        [RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO],
        // 1..=3: one of the three not-detected slots.
        [RET_ONE, RET_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO],
        [RET_ONE, RET_ZERO, RET_MINUS_ONE, RET_ZERO, RET_ZERO, RET_ZERO],
        [RET_ONE, RET_ZERO, RET_ZERO, RET_ONE, RET_ZERO, RET_ZERO],
        // 4: not formatted.
        [RET_MINUS_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO],
        // 5: formatted, no name: `dst` untouched.
        [RET_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_MINUS_ONE, RET_ZERO],
        // 6: an empty name: the memory device text.
        [RET_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_ONE, RET_ONE],
        // 7: the name is copied.
        [RET_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_ONE, RET_MINUS_ONE],
    ];

    #[test]
    fn mc_manager_get_card_error_text_matches() {
        check_lohi(
            ee::s06::f_00241710__generated,
            mc_manager_get_card_error_text,
            96,
            &|c, seed| {
                elf_image(c);
                let mut r = mgr(c, seed, 0x24_1710, RET_ZERO);
                let arm = (seed % 8) as usize;
                for (off, target) in GET_CARD_ERROR_TEXT_SLOTS
                    .into_iter()
                    .zip(GET_CARD_ERROR_TEXT_ARMS[arm])
                {
                    ret_slot(c, MC_FIXTURE_CARD_VT, off, target);
                }
                let slot = [0u32, 2, 3][(seed / 8 % 3) as usize];
                if arm == 7 {
                    let name = MC_FIXTURE_THIS + MC_DEVICE_NAME + slot * mcus::DEVICE_SIZE as u32;
                    mc_text_fixture_put(c, name, ["AB", "Card 1"][(seed / 24 % 2) as usize]);
                }
                // Seeds 48.. unwind in the text lookup.
                mc_text_fixture_db(c, [0, 0x40][(seed / 24 % 2) as usize], seed / 48 % 2 == 1);
                let dst = DATA + 0x5000;
                fill(c, &mut r, dst, 0x40);
                c.g[A0][0] = sx(MC_FIXTURE_THIS);
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = sx(dst);
                c.pc = 0x0024_1710;
            },
        );
    }

    /// The card status test's steering: bits 0/1 (`seed % 3`), bit 2
    /// (`seed / 3 % 6`), bits 3/4 (`seed / 18 % 3`); seeds 54.. return the
    /// cached word. `logged`: slots answering "yes" are the logging fake (so
    /// their arguments are compared) and some runs unwind; otherwise every
    /// slot is a 2-cycle leaf, so the path totals are fixed.
    pub(crate) fn mc_read_card_status_setup(c: &mut Cpu, seed: u32, logged: bool) {
        let _ = mgr(c, seed, 0x24_1b20, RET_ZERO);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let yes = if logged {
            FAKE
        } else {
            [RET_ONE, RET_MINUS_ONE][(seed / 2 % 2) as usize]
        };
        let any = [RET_ZERO, RET_ONE, RET_MINUS_ONE];
        ret_slot(c, vt, MC_CARD_VT_STATUS_BIT0, any[(seed / 5 % 3) as usize]);
        let (bit1_a, bit1_b) = match seed % 3 {
            0 => (yes, RET_ZERO),
            1 => (RET_ZERO, RET_ZERO),
            _ => (RET_ZERO, yes),
        };
        ret_slot(c, vt, MC_CARD_VT_STATUS_BIT1_A, bit1_a);
        ret_slot(c, vt, MC_CARD_VT_STATUS_BIT1_B, bit1_b);
        // Saves on the card, else the signed `free < size` compare.
        let (count, free, size) = match seed / 3 % 6 {
            0 => (yes, RET_ZERO, RET_ZERO),
            1 => (RET_MINUS_ONE, RET_ZERO, RET_ZERO),
            2 => (RET_ZERO, RET_ZERO, RET_ONE),
            3 => (RET_ZERO, RET_MINUS_ONE, RET_ZERO),
            4 => (RET_ZERO, RET_ZERO, RET_MINUS_ONE),
            _ => (RET_ZERO, RET_ONE, RET_ONE),
        };
        ret_slot(c, vt, MEMCARD_CARD_VT_SAVE_COUNT, count);
        ret_slot(c, vt, MC_CARD_VT_FREE_SPACE, free);
        ret_slot(c, vt, MEMCARD_CARD_VT_SAVE_SIZE, size);
        ret_slot(c, vt, mcus::VT_CAN_SAVE, any[(seed / 7 % 3) as usize]);
        let (can_load, bit4) = match seed / 18 % 3 {
            0 => (RET_ZERO, RET_ZERO),
            1 => (yes, yes),
            _ => (yes, RET_ZERO),
        };
        ret_slot(c, vt, mcus::VT_CAN_LOAD, can_load);
        ret_slot(c, vt, MC_CARD_VT_STATUS_BIT4, bit4);
        let slot = seed / 11 % 3;
        w32(c, this + mcus::SELECTED, slot);
        let checked = if seed / 54 % 2 == 1 { slot + 1 } else { slot };
        w32(c, this + mcus::CURR_CHECK, checked);
        w32(
            c,
            this + MC_LAST_CARD_STATUS,
            [0x8000_001f, 0x1234_5678, 0][(seed / 11 % 3) as usize],
        );
        if logged {
            w32(c, LOG_UNWIND_AT, if seed % 4 == 3 { 0 } else { u32::MAX });
        }
        c.g[A0][0] = sx(this);
        c.pc = 0x0024_1b20;
    }

    /// Path totals with every slot a 2-cycle leaf: 82 plus bits 0/1 (1, 10 or
    /// 12), bit 2 (0 or 19) and bits 3/4 (1, 10 or 12); 20 for the cached word.
    pub(crate) const MC_READ_CARD_STATUS_PATHS: [u64; 13] =
        [20, 84, 93, 95, 102, 104, 106, 103, 112, 114, 121, 123, 125];

    #[test]
    fn mc_manager_read_card_status_matches() {
        check(
            ee::s06::f_00241b20__generated,
            mc_manager_read_card_status,
            64,
            &|c, seed| mc_read_card_status_setup(c, seed, true),
        );
        check_paths(
            ee::s06::f_00241b20__generated,
            mc_manager_read_card_status,
            64,
            &MC_READ_CARD_STATUS_PATHS,
            &|c, seed| mc_read_card_status_setup(c, seed, false),
        );
    }

    /// The states of each switch target (`seed % 8`), from the game's table,
    /// and three past its end.
    pub(crate) const GET_STATUS_TEXT_STATES: [&[u32]; 8] = [
        // Checking card.
        &[0, 2, 3, 6, 7, 13, 29, 49, 50, 51, 52, 53, 54],
        // Not detected (the name in the second buffer).
        &[4],
        // Formatting, saving, loading.
        &[10, 11, 39, 40],
        &[15, 16, 17, 18],
        &[31, 32, 33, 34],
        // Deleting (copied as is).
        &[25, 26, 44],
        // No text.
        &[
            1, 5, 8, 9, 12, 14, 19, 20, 21, 22, 23, 24, 27, 28, 30, 35, 36, 37, 38, 41, 42, 43,
            45, 46, 47, 48, 55,
        ],
        // Past the table.
        &[0x38, 0x1000, 0xffff_ffff],
    ];

    #[test]
    fn mc_manager_get_status_text_matches() {
        check(
            ee::s06::f_00241e18__generated,
            mc_manager_get_status_text,
            96,
            &|c, seed| {
                // The switch's table and the empty start string are the game's.
                elf_image(c);
                let mut r = mgr(c, seed, 0x24_1e18, RET_ZERO);
                let states = GET_STATUS_TEXT_STATES[(seed % 8) as usize];
                let state = states[(seed / 8) as usize % states.len()];
                w32(c, MC_FIXTURE_THIS + mcus::STATE, state);
                w32(
                    c,
                    MC_FIXTURE_THIS + mcus::SELECTED,
                    [0, 1, 3][(seed / 24 % 3) as usize],
                );
                // Seeds 48.. unwind in the text lookup.
                mc_text_fixture_db(c, [0, 0x40][(seed / 16 % 2) as usize], seed / 48 % 2 == 1);
                let out = DATA + 0x5000;
                fill(c, &mut r, out, 0x40);
                c.g[A0][0] = sx(MC_FIXTURE_THIS);
                c.g[A1][0] = sx(out);
                c.pc = 0x0024_1e18;
            },
        );
    }

    /// The directory read test's steering: the arm (`seed % 7`) is where the
    /// card driver answers stop it: 0..=2 one of the not-detected slots, 3
    /// not formatted, 4 no name, 5 the name read for another slot than the
    /// current one, 6 for the current one (free count stored). `logged`: as
    /// in [`mc_read_card_status_setup`].
    pub(crate) fn mc_read_dir_info_setup(c: &mut Cpu, seed: u32, logged: bool) {
        let mut r = mgr(c, seed, 0x24_20c8, RET_ZERO);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let yes = if logged {
            FAKE
        } else {
            [RET_ONE, RET_MINUS_ONE][(seed / 21 % 2) as usize]
        };
        let arm = seed % 7;
        let answer = |on: bool| if on { yes } else { RET_ZERO };
        ret_slot(c, vt, MC_CARD_VT_NOT_DETECTED_A, answer(arm == 0));
        ret_slot(c, vt, MC_CARD_VT_NOT_DETECTED_B, answer(arm == 1));
        ret_slot(c, vt, MC_CARD_VT_NOT_DETECTED_C, answer(arm == 2));
        ret_slot(c, vt, MC_CARD_VT_FORMATTED, answer(arm >= 4));
        ret_slot(c, vt, MC_CARD_VT_HAS_NAME, answer(arm >= 5));
        ret_slot(
            c,
            vt,
            MC_CARD_VT_FREE_SPACE,
            [RET_ZERO, RET_ONE, RET_MINUS_ONE][(seed / 21 % 3) as usize],
        );
        ret_slot(c, vt, MC_CARD_VT_READ_NAME, if logged { FAKE } else { RET_ZERO });
        let slot = [0u32, 2, 3][(seed / 7 % 3) as usize];
        w32(c, this + mcus::CURR_CHECK, if arm == 6 { slot } else { slot + 1 });
        // The record (and its name) non-zero, so every store shows.
        fill(c, &mut r, this + mcus::DEVICES + slot * mcus::DEVICE_SIZE as u32, 0x38);
        if logged {
            w32(c, LOG_UNWIND_AT, if seed % 4 == 3 { seed / 4 % 3 } else { u32::MAX });
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = slot as u64;
        c.pc = 0x0024_20c8;
    }

    /// Cycles `EA_MemSet` takes to clear a device name (200 bytes at the
    /// fixture's alignment, the same for every slot), part of every path.
    pub(crate) fn mc_read_dir_info_memset_cycles() -> u64 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = sx(MC_FIXTURE_THIS + MC_DEVICE_NAME);
        c.g[A1][0] = 0;
        c.g[A2][0] = MC_DEVICE_NAME_BYTES;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = mcus_callee::MEM_FILL;
        let before = c.cycles;
        ee::s14::EA_MemSet(&mut c);
        assert_eq!(c.pc, SENTINEL_RA);
        c.cycles - before
    }

    /// Path totals without the clear, per arm 0..=6.
    pub(crate) const MC_READ_DIR_INFO_PATHS: [u64; 7] = [53, 64, 75, 84, 94, 104, 117];

    #[test]
    fn mc_manager_read_dir_info_matches() {
        check_lohi(
            ee::s06::f_002420c8__generated,
            mc_manager_read_dir_info,
            63,
            &|c, seed| mc_read_dir_info_setup(c, seed, true),
        );
        let clear = mc_read_dir_info_memset_cycles();
        let paths: Vec<u64> = MC_READ_DIR_INFO_PATHS.iter().map(|p| p + clear).collect();
        check_paths_lohi(
            ee::s06::f_002420c8__generated,
            mc_manager_read_dir_info,
            63,
            &paths,
            &|c, seed| mc_read_dir_info_setup(c, seed, false),
        );
    }
}
