//! Staging module oc1353 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1353`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `gp` word holding the SLogin card-state block pointer, shared by the
/// `SLoginMc_SyncResult_CmdNN` translators below.
pub(crate) const SLOGIN_MC_SYNC_RESULT_STATE_GP: u32 = 0x1ae8;
/// Card-state word holding the in-progress command phase the translators gate on.
pub(crate) const SLOGIN_MC_SYNC_RESULT_PHASE: u32 = 0x04;
/// Result code while the card layer is in another command's phase.
pub(crate) const SLOGIN_MC_SYNC_RESULT_WRONG_PHASE: u32 = 0x12;
/// Result code for a sync result the translator does not recognize.
pub(crate) const SLOGIN_MC_SYNC_RESULT_BAD_RESULT: u32 = 0x10;
/// A pass-through result the translators echo back unchanged.
pub(crate) const SLOGIN_MC_SYNC_RESULT_PASS: u32 = 0x11;

/// `gp` word holding the SLogin XOR cipher's key base pointer.
pub(crate) const SLOGIN_XOR_CIPHER_KEY_BASE_GP: u32 = 0x1af4;
/// `gp` word holding the cipher's key end pointer (last key byte).
pub(crate) const SLOGIN_XOR_CIPHER_KEY_END_GP: u32 = 0x1af8;
/// `gp` word holding the cipher's key cursor pointer.
pub(crate) const SLOGIN_XOR_CIPHER_KEY_CURSOR_GP: u32 = 0x1afc;
/// `gp` word holding the cipher mode (4 idle, 5 keyed, 7/8 after encrypt/decrypt).
pub(crate) const SLOGIN_XOR_CIPHER_MODE_GP: u32 = 0x1b00;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003f_8838,
        slogin_mc_sync_result_cmd02,
        "SLoginMc_SyncResult_Cmd02",
    ),
    (
        0x003f_8880,
        slogin_mc_sync_result_cmd07,
        "SLoginMc_SyncResult_Cmd07",
    ),
    (
        0x003f_8908,
        slogin_mc_sync_result_cmd08,
        "SLoginMc_SyncResult_Cmd08",
    ),
    (
        0x003f_8968,
        slogin_mc_sync_result_cmd09,
        "SLoginMc_SyncResult_Cmd09",
    ),
    (
        0x003f_89c8,
        slogin_mc_sync_result_cmd1a,
        "SLoginMc_SyncResult_Cmd1A",
    ),
    (
        0x003f_8a08,
        slogin_mc_sync_result_cmd05,
        "SLoginMc_SyncResult_Cmd05",
    ),
    (
        0x003f_8a58,
        slogin_mc_sync_result_cmd06,
        "SLoginMc_SyncResult_Cmd06",
    ),
    (
        0x003f_8ac0,
        slogin_mc_sync_result_cmd13,
        "SLoginMc_SyncResult_Cmd13",
    ),
    (
        0x003f_8b28,
        slogin_mc_sync_result_cmd12,
        "SLoginMc_SyncResult_Cmd12",
    ),
    (
        0x003f_8b78,
        slogin_mc_sync_result_cmd03,
        "SLoginMc_SyncResult_Cmd03",
    ),
    (
        0x003f_8bb0,
        slogin_mc_sync_result_cmd04,
        "SLoginMc_SyncResult_Cmd04",
    ),
    (
        0x003f_8be8,
        slogin_mc_sync_result_cmd17,
        "SLoginMc_SyncResult_Cmd17",
    ),
    (0x003f_8c30, slogin_mc_idle_step, "SLoginMc_IdleStep"),
    (
        0x003f_b5f8,
        slogin_xor_cipher_shutdown,
        "SLoginXorCipher_Shutdown",
    ),
    (0x003f_b620, slogin_xor_cipher_init, "SLoginXorCipher_Init"),
    (
        0x003f_b640,
        slogin_xor_cipher_is_keyed,
        "SLoginXorCipher_IsKeyed",
    ),
    (
        0x003f_b650,
        slogin_xor_cipher_set_key,
        "SLoginXorCipher_SetKey",
    ),
    (
        0x003f_b6a0,
        slogin_xor_cipher_encrypt,
        "SLoginXorCipher_Encrypt",
    ),
    (
        0x003f_b730,
        slogin_xor_cipher_decrypt,
        "SLoginXorCipher_Decrypt",
    ),
    (
        0x003f_b7c0,
        slogin_xor_cipher_reset,
        "SLoginXorCipher_Reset",
    ),
];

/// `int SLoginMc_SyncResult_Cmd02(int result)` (0x3f8838): translate the
/// memory-card sync result for command 0x02. Returns 0x12 while the card
/// layer is in another command's phase, 0x10 for an unrecognized result,
/// and the mapped code otherwise (0 maps to 4, 4 to 3, 3 and 9 to 9,
/// 0x11 echoes).
pub fn slogin_mc_sync_result_cmd02(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x2;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x4;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x3;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd07(int result)` (0x3f8880): translate the
/// memory-card sync result for command 0x07. The card phase must be one of
/// 0x1b/0x1d/0x1c/0x07/0x0b/0x18, else 0x12. The result ladder maps
/// 0 to 3, 3 to 4, 4 to 9, 9 to 0x0e, 0x0e to 0x0d, 0x0d to 0x11,
/// 0x11 to itself, 0x0a echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd07(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x1b;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_1b = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x1d;
            if gate_1b {
                break 'gate;
            }
            c.cycles += 2;
            let gate_1d = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x1c;
            if gate_1d {
                break 'gate;
            }
            c.cycles += 2;
            let gate_1c = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x7;
            if gate_1c {
                break 'gate;
            }
            c.cycles += 2;
            let gate_07 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xb;
            if gate_07 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0b = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x18;
            if gate_0b {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0e = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_0e {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x11;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_11 = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = 0xa;
            if is_11 {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd08(int result)` (0x3f8908): translate the
/// memory-card sync result for command 0x08. The card phase must be one of
/// 0x18/0x08/0x0c/0x10, else 0x12. The result ladder maps 0 to 3, 3 to 4,
/// 4 to 9, 9 to itself, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd08(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x18;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_18 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x8;
            if gate_18 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_08 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xc;
            if gate_08 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0c = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x10;
            if gate_0c {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}
/// `int SLoginMc_SyncResult_Cmd09(int result)` (0x3f8968): translate the
/// memory-card sync result for command 0x09. The card phase must be one of
/// 0x19/0x09/0x0d/0x11, else 0x12. The result ladder maps 0 to 3, 3 to 4,
/// 4 to 9, 9 to itself, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd09(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x19;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_19 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if gate_19 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_09 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if gate_09 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0d = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x11;
            if gate_0d {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd1A(int result)` (0x3f89c8): translate the
/// memory-card sync result for command 0x1a. The card phase must be 0x1a,
/// else 0x12. The result ladder maps 0 to 3, 3 to 9, 9 to itself,
/// 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd1a(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x1a;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd05(int result)` (0x3f8a08): translate the
/// memory-card sync result for command 0x05. The card phase must be 0x05,
/// else 0x12. The result ladder maps 0 to 3, 3 to 9, 9 to 0x0e, 0x0e to 8,
/// 8 to itself, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd05(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x5;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0e = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x8;
            if is_0e {
                break 'result;
            }
            c.cycles += 2;
            let is_eight = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_eight {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}
/// `int SLoginMc_SyncResult_Cmd06(int result)` (0x3f8a58): translate the
/// memory-card sync result for command 0x06. The card phase must be one of
/// 0x16/0x06/0x0e/0x0a, else 0x12. The result ladder maps 0 to 3, 3 to 4,
/// 4 to 9, 9 to 0x0d, 0x0d to itself, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd06(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x16;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_16 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if gate_16 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_06 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if gate_06 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0e = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xa;
            if gate_0e {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd13(int result)` (0x3f8ac0): translate the
/// memory-card sync result for command 0x13. The card phase must be
/// 0x13-0x15, else 0x12. The result ladder maps 0 to 3, 3 to 4, 4 to 9,
/// 9 to 0x0d, 0x0d to 6, 6 to 5, 5 to itself, 0x11 echoes, else 0x10.
pub fn slogin_mc_sync_result_cmd13(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 6;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x13));
            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
            let wrong_phase = c.g[V0][0] == 0;
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_six = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x5;
            if is_six {
                break 'result;
            }
            c.cycles += 2;
            let is_five = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_five {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd12(int result)` (0x3f8b28): translate the
/// memory-card sync result for command 0x12. The card phase must be 0x12,
/// else 0x12. The result ladder maps 0 to 3, 3 to 4, 4 to 9, 9 to 0x0d,
/// 0x0d to itself, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd12(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x12;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}
/// `int SLoginMc_SyncResult_Cmd03(int result)` (0x3f8b78): translate the
/// memory-card sync result for command 0x03. The card phase must be 0x03,
/// else 0x12. Results 0 and 3 echo back, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd03(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
    c.g[V0][0] = 0x3;
    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
    let wrong_phase = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
    if wrong_phase {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let is_zero = c.g[A0][0] == 0;
    c.g[V0][0] = c.g[A0][0];
    if is_zero {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let is_phase = c.g[A0][0] == c.g[V1][0];
    c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
    if is_phase {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let is_other = c.g[A0][0] != c.g[V1][0];
    c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
    if is_other {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[A0][0];
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd04(int result)` (0x3f8bb0): translate the
/// memory-card sync result for command 0x04. The card phase must be 0x04,
/// else 0x12. The result ladder maps 0 to 3, 3 to itself, 0x11 echoes,
/// anything else is 0x10.
pub fn slogin_mc_sync_result_cmd04(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x4;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd17(int result)` (0x3f8be8): translate the
/// memory-card sync result for command 0x17. The card phase must be 0x17,
/// else 0x12. The result ladder maps 0 to 3, 3 to 4, 4 to 9, 9 to itself,
/// 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd17(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x17;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_IdleStep(int result)` (0x3f8c30): accept any result while
/// the card layer idles; echo it back to the caller.
pub fn slogin_mc_idle_step(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}
/// `int SLoginXorCipher_Shutdown(void)` (0x3fb5f8): shut the login cipher
/// down once it is idle (mode 4): record mode 6 and return 0. Any other
/// mode is left alone and reported with 3.
pub fn slogin_xor_cipher_shutdown(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).lw(c);
    c.g[V0][0] = 0x4;
    let not_idle = c.g[V1][0] != c.g[V0][0];
    c.g[V1][0] = 0x6;
    if not_idle {
        c.cycles += 2;
        c.g[V0][0] = 0x3;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0;
    let shut = c.g[V1][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).sw(c, shut);
    ret(c);
}

/// `int SLoginXorCipher_Init(void)` (0x3fb620): clear the login cipher's
/// key words and park it in the idle mode 4; return 0.
pub fn slogin_xor_cipher_init(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x4;
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).sw(c, 0);
    let idle = c.g[V0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).sw(c, idle);
    c.g[V0][0] = 0;
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_END_GP)).sw(c, 0);
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).sw(c, 0);
    ret(c);
}

/// `int SLoginXorCipher_IsKeyed(void)` (0x3fb640): report whether the login
/// cipher holds a key (mode word anything but the idle 4).
pub fn slogin_xor_cipher_is_keyed(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).lw(c);
    c.g[V0][0] ^= 0x4;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `int SLoginXorCipher_SetKey(unsigned char *key, unsigned len)`
/// (0x3fb650): arm the login cipher with `len` key bytes at `key`. A null
/// key or a zero length is refused with 2; the idle mode 4 is refused with
/// 4. Otherwise the key base, end and cursor are recorded, the mode becomes
/// 5 (keyed) and 0 is returned.
pub fn slogin_xor_cipher_set_key(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).lw(c);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x4;
    let idle = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    if idle {
        c.cycles += 2;
        c.g[V0][0] = 0x4;
        ret(c);
        return;
    }
    c.cycles += 8;
    c.g[V1][0] = 0x5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let key = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).sw(c, key);
    let end = c.g[V0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_END_GP)).sw(c, end);
    c.g[V0][0] = 0;
    let keyed = c.g[V1][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).sw(c, keyed);
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).sw(c, key);
    ret(c);
}

/// `int SLoginXorCipher_Encrypt(unsigned char *buf, unsigned len)`
/// (0x3fb6a0): XOR `len` bytes at `buf` with the cipher key stream, advancing
/// the key cursor (wrapping to the key base at the key end). A null buffer
/// or zero length is refused with 2; modes other than the keyed 5 and the
/// encrypt-done 7 return the mode unchanged. Otherwise the mode becomes 7
/// (encrypt-done) and 0 is returned.
pub fn slogin_xor_cipher_encrypt(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).lw(c);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x5;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 2;
        let not_resumed = c.g[V1][0] != 0x7;
        c.g[V0][0] = c.g[V1][0];
        if not_resumed {
            c.cycles += 2;
            ret(c);
            return;
        }
    } else {
        c.g[V0][0] = 0x7;
    }
    c.cycles += 2;
    c.g[A3][0] = 0;
    if c.g[A1][0] != 0 {
        c.cycles += 1;
        c.g[A2][0] = c.g[A0][0];
        loop {
            c.cycles += 9;
            c.g[A0][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).lw(c);
            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
            c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
            c.g[V0][0] ^= c.g[V1][0];
            let mixed = c.g[V0][0];
            c.sb(c.g[A2][0] as u32, mixed);
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_END_GP)).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[V0][0] =
                    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).lw(c);
            } else {
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            }
            c.cycles += 5;
            let cursor = c.g[V0][0];
            Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).sw(c, cursor);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[A3][0] < c.g[A1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_b6d8);
            }
        }
    }
    c.cycles += 4;
    c.g[V1][0] = 0x7;
    c.g[V0][0] = 0;
    let done = c.g[V1][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).sw(c, done);
    ret(c);
}

/// `int SLoginXorCipher_Decrypt(unsigned char *buf, unsigned len)`
/// (0x3fb730): XOR `len` bytes at `buf` with the cipher key stream, advancing
/// the key cursor (wrapping to the key base at the key end). A null buffer
/// or zero length is refused with 2; modes other than the keyed 5 and the
/// decrypt-done 8 return the mode unchanged. Otherwise the mode becomes 8
/// (decrypt-done) and 0 is returned.
pub fn slogin_xor_cipher_decrypt(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).lw(c);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x5;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 2;
        let not_resumed = c.g[V1][0] != 0x8;
        c.g[V0][0] = c.g[V1][0];
        if not_resumed {
            c.cycles += 2;
            ret(c);
            return;
        }
    } else {
        c.g[V0][0] = 0x8;
    }
    c.cycles += 2;
    c.g[A3][0] = 0;
    if c.g[A1][0] != 0 {
        c.cycles += 1;
        c.g[A2][0] = c.g[A0][0];
        loop {
            c.cycles += 9;
            c.g[A0][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).lw(c);
            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
            c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
            c.g[V0][0] ^= c.g[V1][0];
            let mixed = c.g[V0][0];
            c.sb(c.g[A2][0] as u32, mixed);
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_END_GP)).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[V0][0] =
                    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).lw(c);
            } else {
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            }
            c.cycles += 5;
            let cursor = c.g[V0][0];
            Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).sw(c, cursor);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[A3][0] < c.g[A1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_b768);
            }
        }
    }
    c.cycles += 4;
    c.g[V1][0] = 0x8;
    c.g[V0][0] = 0;
    let done = c.g[V1][0];
    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).sw(c, done);
    ret(c);
}

/// `int SLoginXorCipher_Reset(void)` (0x3fb7c0): rewind the login cipher's
/// key cursor to the key base and force the mode back to 5 (keyed) whenever
/// the cipher is keyed or has finished an encrypt/decrypt (modes 5/7/8).
/// Any other mode is left alone; 0 is returned on every path.
pub fn slogin_xor_cipher_reset(c: &mut Cpu) {
    'reset: {
        'mode: {
            c.cycles += 4;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).lw(c);
            c.g[V0][0] = 0x5;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V1][0] =
                    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).lw(c);
                break 'mode;
            }
            'done_mode: {
                c.cycles += 3;
                c.g[V0][0] = 0x7;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V1][0] =
                        Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).lw(c);
                    break 'done_mode;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x8;
                let unknown = c.g[V1][0] != c.g[V0][0];
                c.g[V1][0] =
                    Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_BASE_GP)).lw(c);
                if unknown {
                    break 'reset;
                } else {
                    break 'done_mode;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x5;
            break 'mode;
        }
        c.cycles += 2;
        let mode = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_MODE_GP)).sw(c, mode);
        let cursor = c.g[V1][0];
        Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_XOR_CIPHER_KEY_CURSOR_GP)).sw(c, cursor);
        break 'reset;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Card-state block the sync-result fixtures point `gp + 0x1ae8` at.
    /// Word +0/+8 hold seed-varying fillers; the phase at +4 drives the gate.
    #[cfg(test)]
    pub(crate) const SLOGIN_MC_SYNC_RESULT_TEST_STATE: u32 = DATA + 0x1000;

    /// Key bytes the cipher fixtures use (8 bytes).
    #[cfg(test)]
    pub(crate) const SLOGIN_XOR_CIPHER_TEST_KEY: u32 = DATA + 0x3000;

    /// Buffer bytes the cipher fixtures encrypt/decrypt in place (16 bytes).
    #[cfg(test)]
    pub(crate) const SLOGIN_XOR_CIPHER_TEST_BUF: u32 = DATA + 0x4000;

    /// Point the card-state word at the scratch block and write `phase`,
    /// with seed-varying nonzero fillers around it.
    pub(crate) fn slogin_mc_sync_result_fixture(c: &mut Cpu, r: &mut Rng, phase: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.bus
            .write32(SLOGIN_MC_SYNC_RESULT_TEST_STATE, r.next_u32());
        c.bus.write32(
            SLOGIN_MC_SYNC_RESULT_TEST_STATE + SLOGIN_MC_SYNC_RESULT_PHASE,
            phase,
        );
        c.bus.write32(
            SLOGIN_MC_SYNC_RESULT_TEST_STATE + SLOGIN_MC_SYNC_RESULT_PHASE + 4,
            r.next_u32(),
        );
        c.bus.write32(
            GAME_GP + SLOGIN_MC_SYNC_RESULT_STATE_GP,
            SLOGIN_MC_SYNC_RESULT_TEST_STATE,
        );
    }

    /// Write the cipher mode word with seed-varying nonzero neighbors.
    pub(crate) fn slogin_xor_cipher_mode_fixture(c: &mut Cpu, r: &mut Rng, mode: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.bus
            .write32(GAME_GP + SLOGIN_XOR_CIPHER_KEY_BASE_GP, r.next_u32());
        c.bus
            .write32(GAME_GP + SLOGIN_XOR_CIPHER_KEY_END_GP, r.next_u32());
        c.bus
            .write32(GAME_GP + SLOGIN_XOR_CIPHER_KEY_CURSOR_GP, r.next_u32());
        c.bus.write32(GAME_GP + SLOGIN_XOR_CIPHER_MODE_GP, mode);
    }

    #[test]
    fn slogin_mc_sync_result_cmd02_matches() {
        const PHASES: [u32; 7] = [0x2, 0x2, 0, 1, 3, 0x11, 0xffff_fff2];
        const RESULTS: [u64; 10] = [0, 4, 3, 9, 0x11, 0x10, 1, 2, 0x12, 0x7f];
        check(
            ee::s14::f_003f8838__generated,
            slogin_mc_sync_result_cmd02,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f88_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8838;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                let result = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
                c.g[A0][0] = if seed / 70 % 2 == 1 {
                    result | 0xffff_ffff_0000_0000
                } else {
                    result
                };
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd07_matches() {
        const PHASES: [u32; 8] = [0x1b, 0x1d, 0x1c, 0x7, 0xb, 0x18, 0, 0x5a];
        const RESULTS: [u64; 11] = [0, 3, 4, 9, 0xe, 0xd, 0x11, 0xa, 1, 0x10, 0x7f];
        check(
            ee::s14::f_003f8880__generated,
            slogin_mc_sync_result_cmd07,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f88_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8880;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                let result = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
                c.g[A0][0] = if seed / 88 % 2 == 1 {
                    result | 0xffff_ffff_0000_0000
                } else {
                    result
                };
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd08_matches() {
        const PHASES: [u32; 6] = [0x18, 0x8, 0xc, 0x10, 0, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0x11, 0x10, 1, 0x12];
        check(
            ee::s14::f_003f8908__generated,
            slogin_mc_sync_result_cmd08,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f89_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8908;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd09_matches() {
        const PHASES: [u32; 6] = [0x19, 0x9, 0xd, 0x11, 0, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0x11, 0x10, 2, 0x12];
        check(
            ee::s14::f_003f8968__generated,
            slogin_mc_sync_result_cmd09,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f89_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8968;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }
    #[test]
    fn slogin_mc_sync_result_cmd1a_matches() {
        const PHASES: [u32; 5] = [0x1a, 0x1a, 0, 0x19, 0x5a];
        const RESULTS: [u64; 7] = [0, 3, 9, 0x11, 0x10, 1, 0x1a];
        check(
            ee::s14::f_003f89c8__generated,
            slogin_mc_sync_result_cmd1a,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f89_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_89c8;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd05_matches() {
        const PHASES: [u32; 5] = [0x5, 0x5, 0, 4, 0x5a];
        const RESULTS: [u64; 9] = [0, 3, 9, 0xe, 8, 0x11, 0x10, 1, 5];
        check(
            ee::s14::f_003f8a08__generated,
            slogin_mc_sync_result_cmd05,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8a_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8a08;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd06_matches() {
        const PHASES: [u32; 6] = [0x16, 0x6, 0xe, 0xa, 0, 0x5a];
        const RESULTS: [u64; 9] = [0, 3, 4, 9, 0xd, 0x11, 0x10, 1, 6];
        check(
            ee::s14::f_003f8a58__generated,
            slogin_mc_sync_result_cmd06,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8a_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8a58;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd13_matches() {
        const PHASES: [u32; 6] = [0x13, 0x14, 0x15, 0x12, 0, 0x5a];
        const RESULTS: [u64; 11] = [0, 3, 4, 9, 0xd, 6, 5, 0x11, 0x10, 1, 0x13];
        check(
            ee::s14::f_003f8ac0__generated,
            slogin_mc_sync_result_cmd13,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8a_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8ac0;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd12_matches() {
        const PHASES: [u32; 5] = [0x12, 0x12, 0, 0x11, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0xd, 0x11, 0x10, 0x12];
        check(
            ee::s14::f_003f8b28__generated,
            slogin_mc_sync_result_cmd12,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_2800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8b28;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd03_matches() {
        const PHASES: [u32; 5] = [0x3, 0x3, 0, 2, 0x5a];
        const RESULTS: [u64; 7] = [0, 3, 0x11, 1, 0x10, 2, 0x12];
        check(
            ee::s14::f_003f8b78__generated,
            slogin_mc_sync_result_cmd03,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8b78;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd04_matches() {
        const PHASES: [u32; 5] = [0x4, 0x4, 0, 3, 0x5a];
        const RESULTS: [u64; 6] = [0, 3, 0x11, 1, 0x10, 4];
        check(
            ee::s14::f_003f8bb0__generated,
            slogin_mc_sync_result_cmd04,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8bb0;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd17_matches() {
        const PHASES: [u32; 5] = [0x17, 0x17, 0, 0x16, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0x11, 0x10, 1, 0x17];
        check(
            ee::s14::f_003f8be8__generated,
            slogin_mc_sync_result_cmd17,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8be8;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_idle_step_matches() {
        check(
            ee::s14::f_003f8c30__generated,
            slogin_mc_idle_step,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8c_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8c30;
                c.g[A0][0] = if seed % 5 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) | ((r.next_u32() as u64) << 32)
                };
            },
        );
    }
    #[test]
    fn slogin_xor_cipher_shutdown_matches() {
        const MODES: [u32; 7] = [0x4, 0x4, 0, 5, 6, 3, 1];
        check(
            ee::s15::f_003fb5f8__generated,
            slogin_xor_cipher_shutdown,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb5_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b5f8;
                slogin_xor_cipher_mode_fixture(c, &mut r, MODES[(seed as usize) % MODES.len()]);
            },
        );
    }

    #[test]
    fn slogin_xor_cipher_init_matches() {
        check(
            ee::s15::f_003fb620__generated,
            slogin_xor_cipher_init,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb6_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_b620;
                c.bus
                    .write32(GAME_GP + SLOGIN_XOR_CIPHER_KEY_BASE_GP, 0x5a00_0000 | seed);
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_END_GP,
                    0xa500_0000 | (seed ^ 0x3f62_0f00),
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_CURSOR_GP,
                    0x5a00_0000 | (seed >> 2),
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_MODE_GP,
                    0xa500_0000 | (seed >> 5),
                );
            },
        );
    }

    #[test]
    fn slogin_xor_cipher_is_keyed_matches() {
        const MODES: [u32; 8] = [0x4, 0x4, 0, 5, 7, 8, 1, 0x8000_0004];
        check(
            ee::s15::f_003fb640__generated,
            slogin_xor_cipher_is_keyed,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb6_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b640;
                slogin_xor_cipher_mode_fixture(c, &mut r, MODES[(seed as usize) % MODES.len()]);
            },
        );
    }

    #[test]
    fn slogin_xor_cipher_set_key_matches() {
        const LENS: [u64; 4] = [0, 1, 6, 9];
        const MODES: [u32; 4] = [0x4, 0x5, 0x6, 0x5];
        check(
            ee::s15::f_003fb650__generated,
            slogin_xor_cipher_set_key,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb6_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_b650;
                for i in 0..8u32 {
                    c.bus.write8(
                        SLOGIN_XOR_CIPHER_TEST_KEY + i,
                        (r.next_u32() >> (i % 24)) as u8,
                    );
                }
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_MODE_GP,
                    MODES[((seed / 12) as usize) % MODES.len()],
                );
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    SLOGIN_XOR_CIPHER_TEST_KEY as u64
                };
                c.g[A1][0] = LENS[((seed / 3) as usize) % LENS.len()];
            },
        );
    }

    #[test]
    fn slogin_xor_cipher_encrypt_matches() {
        const MODES: [u32; 8] = [0x5, 0x5, 0x7, 0x4, 0, 0x6, 0x8, 0x3];
        const LENS: [u64; 9] = [0, 1, 2, 5, 9, 16, 3, 7, 12];
        const CURSORS: [u32; 3] = [0, 3, 7];
        check(
            ee::s15::f_003fb6a0__generated,
            slogin_xor_cipher_encrypt,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb6_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_b6a0;
                for i in 0..8u32 {
                    c.bus.write8(
                        SLOGIN_XOR_CIPHER_TEST_KEY + i,
                        (r.next_u32() >> (i % 24)) as u8,
                    );
                }
                for i in 0..16u32 {
                    c.bus.write8(
                        SLOGIN_XOR_CIPHER_TEST_BUF + i,
                        (r.next_u32() >> (i % 16)) as u8,
                    );
                }
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_BASE_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY,
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_END_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY + 7,
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_CURSOR_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY + CURSORS[(seed as usize / 72) % CURSORS.len()],
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_MODE_GP,
                    MODES[(seed as usize / 9) % MODES.len()],
                );
                c.g[A0][0] = if seed % 9 == 0 {
                    0
                } else {
                    SLOGIN_XOR_CIPHER_TEST_BUF as u64
                };
                c.g[A1][0] = LENS[(seed as usize) % LENS.len()];
            },
        );
    }

    #[test]
    fn slogin_xor_cipher_decrypt_matches() {
        const MODES: [u32; 8] = [0x5, 0x5, 0x8, 0x4, 0, 0x6, 0x7, 0x3];
        const LENS: [u64; 9] = [0, 1, 2, 5, 9, 16, 3, 7, 12];
        const CURSORS: [u32; 3] = [0, 3, 7];
        check(
            ee::s15::f_003fb730__generated,
            slogin_xor_cipher_decrypt,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb7_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_b730;
                for i in 0..8u32 {
                    c.bus.write8(
                        SLOGIN_XOR_CIPHER_TEST_KEY + i,
                        (r.next_u32() >> (i % 24)) as u8,
                    );
                }
                for i in 0..16u32 {
                    c.bus.write8(
                        SLOGIN_XOR_CIPHER_TEST_BUF + i,
                        (r.next_u32() >> (i % 16)) as u8,
                    );
                }
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_BASE_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY,
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_END_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY + 7,
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_CURSOR_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY + CURSORS[(seed as usize / 72) % CURSORS.len()],
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_MODE_GP,
                    MODES[(seed as usize / 9) % MODES.len()],
                );
                c.g[A0][0] = if seed % 9 == 0 {
                    0
                } else {
                    SLOGIN_XOR_CIPHER_TEST_BUF as u64
                };
                c.g[A1][0] = LENS[(seed as usize) % LENS.len()];
            },
        );
    }

    #[test]
    fn slogin_xor_cipher_reset_matches() {
        const MODES: [u32; 8] = [0x5, 0x7, 0x8, 0x4, 0, 0x6, 0x5, 0x8];
        check(
            ee::s15::f_003fb7c0__generated,
            slogin_xor_cipher_reset,
            300,
            &|c, seed| {
                let mut r = Rng(0x3fb7_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_b7c0;
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_BASE_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY,
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_END_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY + 7,
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_KEY_CURSOR_GP,
                    SLOGIN_XOR_CIPHER_TEST_KEY + (seed % 8),
                );
                c.bus.write32(
                    GAME_GP + SLOGIN_XOR_CIPHER_MODE_GP,
                    MODES[(seed as usize) % MODES.len()],
                );
            },
        );
    }
}
