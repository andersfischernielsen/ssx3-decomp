//! Staging wave 201/202 (oc213): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::{abi, guest::Addr};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_d120,
        wscript_cache_get_object_slot,
        "tWScriptCache_GetObjectSlot",
    ),
    (
        0x0012_7998,
        player_get_control_input,
        "Player_GetControlInput",
    ),
    (
        0x0033_42d0,
        overlap_list_find_closest_contact,
        "OverlapList_FindClosestContact",
    ),
    (
        0x0011_2fb0,
        rider_update_dist_to_finish,
        "Rider_UpdateDistToFinish",
    ),
    (
        0x0016_6f90,
        chase_camera_build_output_view,
        "ChaseCameraAlgorithm_BuildOutputView",
    ),
    (
        0x0010_49c0,
        rider_anim_seq_type02_update,
        "RiderAnim_UpdateSeqType02",
    ),
    (
        0x0011_3f88,
        rider_set_slew3_and4_targets,
        "Rider_SetSlew3And4Targets",
    ),
];

// ------------------------------------------------ tWScriptCache_GetObjectSlot

/// `int tWScriptCache_GetObjectSlot(sEntry* entry, Object* obj, int i)`
/// (0x3ad120): an object's cached script slot `i`. The object's script
/// (`obj+0x88`) carries an id (`script+8`, -1 = none) whose upper bits
/// (`id >> 8`) index the entry's slot table (`entry+8` -> table, rows of
/// 0x18 bytes at `table+0x1c`). Returns -1 when the object has no script,
/// the entry no table, or the script no id.
pub fn wscript_cache_get_object_slot(c: &mut Cpu) {
    const OBJ_SCRIPT: u32 = 0x88;
    const ENTRY_TABLE: u32 = 0x8;
    const SCRIPT_ID: u32 = 0x8;
    const TABLE_ROWS: u32 = 0x1c;
    const ROW_SIZE: u64 = 0x18;

    c.cycles += 3;
    let script = Addr(abi::arg(c, 1)).off(OBJ_SCRIPT).lw(c);
    c.g[V1][0] = script;
    if script != 0 {
        c.cycles += 3;
        let table = Addr(abi::arg(c, 0)).off(ENTRY_TABLE).lw(c);
        c.g[V0][0] = table;
        c.g[A0][0] = table;
        if table != 0 {
            c.cycles += 4;
            let id = Addr(script as u32).off(SCRIPT_ID).lw(c);
            c.g[V1][0] = id;
            if id != u64::MAX {
                c.cycles += 9;
                c.g[V0][0] = sx((id as u32) >> 8);
                let rows = Addr(table as u32).off(TABLE_ROWS).lw(c);
                c.g[A0][0] = rows;
                c.g[V1][0] = ROW_SIZE;
                c.mult(0, V0, V0, V1); // row index * 0x18 (also LO/HI)
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(rows as u32));
                let slot = (c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32);
                c.g[V1][0] = sx(slot);
                c.g[V0][0] = Addr(slot).lw(c);
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

// ------------------------------------------------ Player_GetControlInput

/// `Player` fields read by `Player_GetControlInput`.
mod gci {
    /// The rider object (`Rider_GetSubState` argument).
    pub const RIDER: u32 = 0x18;
    /// The input binding the buttons and sticks are read through.
    pub const BINDING: u32 = 0xdf0;
    /// The replay cache that records (or plays back) the input.
    pub const REPLAY: u32 = 0xdf8;
    /// `memset`.
    pub const MEMSET: u32 = 0x003e_6448;
    /// `int InputBinding_EvalBool(InputBinding*, int id)`.
    pub const EVAL_BOOL: u32 = 0x0032_0c48;
    /// `float InputBinding_EvalFloat(InputBinding*, int id)`.
    pub const EVAL_FLOAT: u32 = 0x0032_0bf0;
    /// `ReplayCache_Update(ReplayCache*, Input*)`.
    pub const REPLAY_UPDATE: u32 = 0x0026_d178;
    /// The `switch (substate)` jump table (14 cases) in `.rodata`.
    pub const CASES: u32 = 0x0045_7ea0;
    /// `31.0f`: stick values in [-1, 1] become 6-bit fields.
    pub const STICK_SCALE: u32 = 0x41f8_0000;
}

/// Which registers a bit-field merge leaves the word and the mask in (the
/// original's register allocation differs per site).
#[derive(Clone, Copy)]
enum Merge {
    WordV1MaskA0,
    WordA0MaskV1,
    WordA1MaskV1,
}

/// `word = (word & ~(bits << shift)) | ((v0 & bits) << shift)` at `at`: the
/// compiled bit-field store of the value in `v0`.
fn gci_field(c: &mut Cpu, at: Addr, bits: u32, shift: u32, regs: Merge) {
    let word = at.lw(c);
    let mask = sx(!(bits << shift));
    let value = sx(((c.g[V0][0] as u32) & bits) << shift);
    let merged = (word & mask) | value;
    c.g[V0][0] = value;
    let (w, m) = match regs {
        Merge::WordV1MaskA0 => (V1, A0),
        Merge::WordA0MaskV1 => (A0, V1),
        Merge::WordA1MaskV1 => (A1, V1),
    };
    c.g[w][0] = merged;
    c.g[m][0] = mask;
    at.sw(c, merged);
}

/// One button bit (`v0 & 1`) into bit `n` of the first input word.
fn gci_bit(c: &mut Cpu, input: Addr, n: u32, regs: Merge) {
    gci_field(c, input, 1, n, regs);
}

/// `f0 *= f[k]; f1 = (int)f0; v0 = f1`: a stick value scaled to an integer.
fn gci_scaled(c: &mut Cpu, k: usize) {
    use crate::ps2f::FpuF32;
    let v = FpuF32(c.f[0]) * FpuF32(c.f[k]);
    c.f[0] = v.0;
    c.f[1] = v.to_i32() as u32;
    c.g[V0][0] = sx(c.f[1]);
}

/// `li at, 31.0; mtc1 at, f[k]`.
fn gci_scale_into(c: &mut Cpu, k: usize) {
    c.g[AT][0] = sx(gci::STICK_SCALE);
    c.f[k] = gci::STICK_SCALE;
}

/// `InputBinding_Eval{Bool,Float}(player->binding, id)` returning to `ret`.
fn gci_eval(c: &mut Cpu, player: u32, f: u32, id: u64, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = id;
    c.g[A0][0] = Addr(player).off(gci::BINDING).lw(c);
    call_ptr(c, f, ret)
}

/// The `0x00128148` tail shared by substates 0, 1, 2 and 7: the stick value
/// already read goes into bits 0-5 of the second word, then stick `next`
/// into bits 6-11.
fn gci_second_word_sticks(c: &mut Cpu, player: u32, input: Addr, next: u64) -> crate::call::Flow {
    c.cycles += 19;
    let second = input.off(4);
    gci_scaled(c, 20);
    gci_field(c, second, 0x3f, 0, Merge::WordV1MaskA0);
    gci_eval(c, player, gci::EVAL_FLOAT, next, 0x0012_8168)?;
    gci_scaled(c, 20);
    gci_field(c, second, 0x3f, 6, Merge::WordA0MaskV1);
    Ok(())
}

/// The `0x001285f4` tail of substates 6 and 10: stick `id` into bits 12-17
/// of the first word.
fn gci_single_stick(c: &mut Cpu, player: u32, input: Addr, id: u64) -> crate::call::Flow {
    c.cycles += 13;
    gci_eval(c, player, gci::EVAL_FLOAT, id, 0x0012_85fc)?;
    gci_scale_into(c, 1);
    gci_scaled(c, 1);
    gci_field(c, input, 0x3f, 12, Merge::WordA0MaskV1);
    c.cycles += 2;
    Ok(())
}

/// `void Player_GetControlInput(Player* player, Input* input)` (0x127998):
/// clear the 8-byte input record, then pack the buttons (one bit each) and
/// the analog sticks (6 bits each, `(int)(value * 31)`) that the rider's
/// current substate uses into it; the replay cache (`player+0xdf8`), if
/// any, then records or plays back the record.
pub fn player_get_control_input(c: &mut Cpu) {
    use self::Merge::*;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        const BOOL: u32 = gci::EVAL_BOOL;
        const FLOAT: u32 = gci::EVAL_FLOAT;
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x30);
        let player = r32(c, S1);
        let input = Addr(r32(c, S0));
        // memset(input, 0, 8)
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, gci::MEMSET, 0x0012_79c4)?;
        c.g[A0][0] = Addr(player).off(gci::RIDER).lw(c);
        Callee {
            addr: 0x0011_fee8,
            f: ee::s00::Rider_GetSubState,
        }
        .call(c, 0x0012_79cc)?;
        let substate = c.g[V0][0];
        c.g[V1][0] = substate;
        c.g[V0][0] = 0x0045_0000;
        // Each case's code addresses the fields with these helpers.
        let b = |c: &mut Cpu, id: u64, ret: u32| gci_eval(c, player, BOOL, id, ret);
        let a = |c: &mut Cpu, id: u64, ret: u32| gci_eval(c, player, FLOAT, id, ret);
        if substate < 0xe {
            c.cycles += 6;
            c.g[V1][0] = sx((substate as u32) << 2);
            c.g[V0][0] = sx(gci::CASES);
            let entry = gci::CASES.wrapping_add(substate as u32 * 4);
            c.g[V1][0] = sx(entry);
            c.g[A0][0] = Addr(entry).lw(c);
            match substate {
                // 0x001279f4
                0 => {
                    c.cycles += 122;
                    gci_scale_into(c, 20);
                    b(c, 0x2e, 0x0012_7a08)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x19, 0x0012_7a30)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, 0x11, 0x0012_7a58)?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    b(c, 0x12, 0x0012_7a80)?;
                    gci_bit(c, input, 15, WordA0MaskV1);
                    b(c, 0x13, 0x0012_7aac)?;
                    gci_bit(c, input, 16, WordA0MaskV1);
                    b(c, 0x14, 0x0012_7ad8)?;
                    gci_bit(c, input, 17, WordA0MaskV1);
                    b(c, 5, 0x0012_7b04)?;
                    gci_bit(c, input, 18, WordA0MaskV1);
                    b(c, 6, 0x0012_7b30)?;
                    gci_bit(c, input, 19, WordA0MaskV1);
                    a(c, 0, 0x0012_7b5c)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 20, WordA0MaskV1);
                    a(c, 1, 0x0012_7b94)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 26, WordA0MaskV1);
                    a(c, 2, 0x0012_7bc8)?;
                    gci_second_word_sticks(c, player, input, 3)?;
                }
                // 0x00127c68
                1 => {
                    c.cycles += 101;
                    gci_scale_into(c, 20);
                    b(c, 0x2e, 0x0012_7c7c)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x13, 0x0012_7ca4)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, 0x14, 0x0012_7ccc)?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    b(c, 0x11, 0x0012_7cf4)?;
                    gci_bit(c, input, 16, WordA0MaskV1);
                    b(c, 0x12, 0x0012_7d20)?;
                    gci_bit(c, input, 15, WordA0MaskV1);
                    b(c, 0x16, 0x0012_7d4c)?;
                    gci_bit(c, input, 17, WordA0MaskV1);
                    a(c, 0x17, 0x0012_7d78)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 24, WordA0MaskV1);
                    a(c, 0, 0x0012_7db0)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 18, WordA0MaskV1);
                    a(c, 3, 0x0012_7de8)?;
                    gci_second_word_sticks(c, player, input, 4)?;
                }
                // 0x0012803c
                2 => {
                    c.cycles += 67;
                    gci_scale_into(c, 20);
                    b(c, 0x2e, 0x0012_8050)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x12, 0x0012_8078)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, 0x14, 0x0012_80a0)?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    a(c, 8, 0x0012_80c8)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 15, WordA0MaskV1);
                    a(c, 9, 0x0012_8100)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 21, WordA0MaskV1);
                    a(c, 7, 0x0012_8138)?;
                    gci_second_word_sticks(c, player, input, 0x17)?;
                }
                // 0x00128194
                3 => {
                    c.cycles += 59;
                    gci_scale_into(c, 20);
                    b(c, 0x2e, 0x0012_81a8)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x13, 0x0012_81d0)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, 0x14, 0x0012_81f8)?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    a(c, 0, 0x0012_8220)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 15, WordA0MaskV1);
                    a(c, 0x17, 0x0012_8258)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 21, WordA0MaskV1);
                    c.cycles += 2;
                }
                // 0x00128280 (4) and 0x001283a4 (5): with the trick button byte.
                4 | 5 => {
                    let five = substate == 5;
                    c.cycles += if five { 109 } else { 73 };
                    gci_scale_into(c, 20);
                    let (ids, rets): ([u64; 4], [u32; 4]) = if five {
                        (
                            [0x2e, 0x19, 0x15, 0xc],
                            [0x0012_83b8, 0x0012_83e0, 0x0012_8408, 0x0012_8430],
                        )
                    } else {
                        (
                            [0x2e, 0x19, 5, 6],
                            [0x0012_8294, 0x0012_82bc, 0x0012_82e4, 0x0012_830c],
                        )
                    };
                    b(c, ids[0], rets[0])?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, ids[1], rets[1])?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, ids[2], rets[2])?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    b(c, ids[3], rets[3])?;
                    gci_bit(c, input, 15, WordA1MaskV1);
                    c.g[A0][0] = c.g[S1][0];
                    let ret = if five { 0x0012_8458 } else { 0x0012_8334 };
                    Callee {
                        addr: 0x0012_76f0,
                        f: ee::s00::Player_GetTrickButton,
                    }
                    .call(c, ret)?;
                    input.off(2).sb(c, c.g[V0][0]);
                    let first = if five { 0xe } else { 0 };
                    a(c, first, if five { 0x0012_8468 } else { 0x0012_8344 })?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 24, WordA0MaskV1);
                    if !five {
                        a(c, 1, 0x0012_837c)?;
                        gci_scaled(c, 20);
                        gci_field(c, input.off(4), 0x3f, 0, WordV1MaskA0);
                    } else {
                        let second = input.off(4);
                        a(c, 0xf, 0x0012_84a0)?;
                        gci_scaled(c, 20);
                        gci_field(c, second, 0x3f, 0, WordV1MaskA0);
                        a(c, 0xb, 0x0012_84d0)?;
                        gci_scaled(c, 20);
                        gci_field(c, second, 0x3f, 12, WordA0MaskV1);
                        a(c, 0xa, 0x0012_8508)?;
                        gci_scaled(c, 20);
                        gci_field(c, second, 0x3f, 6, WordV1MaskA0);
                        a(c, 3, 0x0012_853c)?;
                        // Stick 3 as a -1/0/1 direction (dead zone 0.5).
                        let v = FpuF32(c.f[0]);
                        c.f[1] = v.0;
                        gci_dead_zone(c, v);
                        c.cycles += 9;
                        let word = second.lw(c);
                        let mask = 0xffff_ffff_fff3_ffffu64;
                        c.g[V1][0] = mask;
                        c.g[A0][0] = sx(((c.g[A0][0] & 3) as u32) << 18);
                        let merged = (word & mask) | c.g[A0][0];
                        c.g[V0][0] = merged;
                        second.sw(c, merged);
                    }
                }
                // 0x00127dfc
                6 => {
                    c.cycles += 3;
                    gci_single_stick(c, player, input, 0x1a)?;
                }
                // 0x00127e08
                7 => {
                    c.cycles += 86;
                    gci_scale_into(c, 20);
                    b(c, 0x2e, 0x0012_7e1c)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x11, 0x0012_7e44)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, 0x12, 0x0012_7e6c)?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    b(c, 0x13, 0x0012_7e94)?;
                    gci_bit(c, input, 15, WordA0MaskV1);
                    b(c, 0x14, 0x0012_7ec0)?;
                    gci_bit(c, input, 16, WordA1MaskV1);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0012_7848,
                        f: ee::s00::Player_GetUberGrindButton,
                    }
                    .call(c, 0x0012_7ee8)?;
                    gci_field(c, input, 0xff, 17, WordA0MaskV1);
                    a(c, 0x17, 0x0012_7f14)?;
                    gci_scaled(c, 20);
                    gci_field(c, input, 0x3f, 25, WordA0MaskV1);
                    a(c, 0xd, 0x0012_7f4c)?;
                    gci_second_word_sticks(c, player, input, 3)?;
                }
                // 0x00128598
                8 => {
                    c.cycles += 21;
                    b(c, 0x2e, 0x0012_85a4)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x10, 0x0012_85cc)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                }
                // 0x001285ec
                10 => {
                    c.cycles += 2;
                    gci_single_stick(c, player, input, 0)?;
                }
                // 0x00127bdc
                11 => {
                    c.cycles += 35;
                    b(c, 0x2e, 0x0012_7be8)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x19, 0x0012_7c10)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    a(c, 0x18, 0x0012_7c38)?;
                    gci_scale_into(c, 1);
                    gci_scaled(c, 1);
                    gci_field(c, input, 0x3f, 14, WordA0MaskV1);
                    c.cycles += 2;
                }
                // 0x00127f60
                12 => {
                    c.cycles += 55;
                    b(c, 0x2e, 0x0012_7f6c)?;
                    gci_bit(c, input, 12, WordV1MaskA0);
                    b(c, 0x13, 0x0012_7f94)?;
                    gci_bit(c, input, 13, WordV1MaskA0);
                    b(c, 0x14, 0x0012_7fbc)?;
                    gci_bit(c, input, 14, WordV1MaskA0);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0012_7848,
                        f: ee::s00::Player_GetUberGrindButton,
                    }
                    .call(c, 0x0012_7fe0)?;
                    gci_field(c, input, 0xff, 15, WordA0MaskV1);
                    a(c, 0x17, 0x0012_800c)?;
                    gci_scale_into(c, 1);
                    gci_scaled(c, 1);
                    gci_field(c, input, 0x3f, 23, WordA0MaskV1);
                    c.cycles += 2;
                }
                // 9 and 13: nothing to read (0x00128630).
                _ => {}
            }
        }

        // 0x00128630: let the replay cache record or play back the input.
        c.cycles += 3;
        let replay = Addr(player).off(gci::REPLAY).lw(c);
        c.g[A0][0] = replay;
        if replay != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, gci::REPLAY_UPDATE, 0x0012_8644)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// Substate 5's stick direction: `a0 = v < -0.5 ? -1 : (0.5 < v)`, with
/// the compares, `f0`, `at` and the condition bit as the original leaves them.
fn gci_dead_zone(c: &mut Cpu, v: crate::ps2f::FpuF32) {
    use crate::ps2f::FpuF32;
    c.g[AT][0] = sx(0xbf00_0000);
    c.f[0] = 0xbf00_0000;
    let above_low = FpuF32(0xbf00_0000).le(v);
    abi::set_fcc(c, above_low);
    if !above_low {
        c.g[A0][0] = u64::MAX;
        return;
    }
    c.cycles += 6;
    c.g[AT][0] = 0x3f00_0000;
    c.f[0] = 0x3f00_0000;
    let above_high = FpuF32(0x3f00_0000).lt(v);
    abi::set_fcc(c, above_high);
    if above_high {
        c.g[A0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
}

// ------------------------------------------------ OverlapList_FindClosestContact

/// Contact records `OverlapList_CollideVolume` fills (0x80 bytes each).
mod contact {
    pub const SIZE: u32 = 0x80;
    /// Distance along the volume.
    pub const DIST: u32 = 0x40;
    /// The object hit (0 for world geometry).
    pub const OBJECT: u32 = 0x50;
    /// The surface hit.
    pub const SURFACE: u32 = 0x54;
    /// Tie-break keys: `object+0x78`, `surface+0x150`.
    pub const OBJECT_KEY: u32 = 0x78;
    pub const SURFACE_KEY: u32 = 0x150;
}

/// `float OverlapList_FindClosestContact(OverlapList* l, Volume* v,
/// Contact* out, int flags)` (0x3342d0): collide the volume against the
/// list into 64 contacts on the stack, pick the one whose distance is
/// closest to the volume's reference distance (virtual slot 3 of the
/// volume), ties going to the smaller object key (contacts with an object
/// first) or, between world contacts, the smaller surface key; copy it to
/// `out` and return its distance (-1 when nothing was hit).
pub fn overlap_list_find_closest_contact(c: &mut Cpu) {
    use self::contact::*;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x2050);
        c.g[V0][0] = 0x3f;
        frame.save(c, S0, 0x2040);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S3, 0x2010);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x2030);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x2020);
        frame.save_ra(c, 0x2000);

        // 0x003342f8: the 64 contacts' (empty) constructors.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[T0][0] = c.g[A3][0];
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0033_42f8);
            }
        }

        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x0033_3ef8,
            f: ee::s11::OverlapList_CollideVolume,
        }
        .call(c, 0x0033_4324)?;
        let count = c.g[V0][0];
        c.g[S2][0] = count;
        if count == 0 {
            c.cycles += 4;
            c.g[AT][0] = sx(0xbf80_0000);
            c.f[0] = 0xbf80_0000; // -1.0
            frame.restore(c, S0, 0x2040);
        } else {
            // The reference distance: virtual slot 3 of the volume.
            let vtable = Addr(r32(c, S0)).off(0x50).lw(c);
            c.g[V0][0] = vtable;
            c.cycles += 13;
            c.g[S1][0] = 0;
            let slot = crate::call::VSlot::read(c, vtable, 0x18);
            c.g[A0][0] = slot.this(c.g[S0][0]);
            c.g[V1][0] = slot.func;
            call_ptr(c, slot.func as u32, 0x0033_4354)?;
            c.f[2] = c.f[0];
            let reference = FpuF32(c.f[2]);
            c.g[A3][0] = 1;
            let hits = sp(c);
            let first = Addr(hits).off(DIST).read_fpu(c) - reference;
            c.f[0] = first.0;
            c.f[1] = first.abs().0;
            c.g[V0][0] = (1 < count as i64) as u64;
            c.g[A2][0] = sx(hits.wrapping_add(SIZE));
            if 1 < count as i64 {
                c.cycles += 1;
                // 0x00334378: a3 = index, a2 = its contact, s1 = best,
                // f1 = best |dist - reference|.
                loop {
                    c.cycles += 6;
                    let hit = r32(c, A2);
                    let d = (Addr(hit).off(DIST).read_fpu(c) - FpuF32(c.f[2])).abs();
                    c.f[0] = d.0;
                    let closer = d.lt(FpuF32(c.f[1]));
                    abi::set_fcc(c, closer);
                    if closer {
                        c.cycles += 3;
                        c.f[1] = d.0;
                        c.g[S1][0] = c.g[A3][0];
                    } else {
                        c.cycles += 4;
                        let tie = d.eq_s(FpuF32(c.f[1]));
                        abi::set_fcc(c, tie);
                        c.g[V0][0] = sx((c.g[S1][0] as u32) << 7);
                        if tie {
                            contact_tie_break(c, hit);
                        }
                    }
                    c.cycles += 4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    let more = (c.g[A3][0] as i64) < (c.g[S2][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[A2][0] = sx(hit.wrapping_add(SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0033_4378);
                    }
                }
            }

            // 0x0033440c: copy the best contact to `out`, 0x20 bytes a turn.
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[S1][0] as u32) << 7).wrapping_add(sp(c)));
            c.g[V1][0] = c.g[S3][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(SIZE));
            loop {
                c.cycles += 7;
                let (src, dst) = (r32(c, V0), r32(c, V1));
                c.lq(A1, src);
                c.lq(A2, src.wrapping_add(0x10));
                c.sq(A1, dst);
                c.sq(A2, dst.wrapping_add(0x10));
                c.g[V0][0] = sx(src.wrapping_add(0x20));
                c.g[V1][0] = sx(dst.wrapping_add(0x20));
                if c.g[V0][0] == c.g[A0][0] {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_441c);
                }
            }
            c.cycles += 2;
            let out = r32(c, S3);
            c.lwc1(0, out.wrapping_add(DIST));
            frame.restore(c, S0, 0x2040);
        }

        c.cycles += 6;
        frame.restore(c, S1, 0x2030);
        frame.restore(c, S2, 0x2020);
        frame.restore(c, S3, 0x2010);
        frame.restore_ra(c, 0x2000);
        frame.close(c);
        Ok(())
    });
}

/// `FindClosestContact`'s tie-break (0x003343a8..0x003343f8) between the
/// contact `hit` (index a3) and the best so far (`v0 = best << 7`):
/// take `hit` when it has an object and the best has none, or both have
/// one and its key is smaller (unsigned); between two world contacts, when
/// its surface key is smaller.
fn contact_tie_break(c: &mut Cpu, hit: u32) {
    use self::contact::*;
    c.cycles += 3;
    let object = Addr(hit).off(OBJECT).lw(c);
    c.g[A0][0] = object;
    let best = (c.g[V0][0] as u32).wrapping_add(sp(c));
    c.g[V1][0] = sx(best);
    if object != 0 {
        c.cycles += 3;
        let best_object = Addr(best).off(OBJECT).lw(c);
        c.g[V0][0] = best_object;
        if best_object != 0 {
            let key = Addr(object as u32).off(OBJECT_KEY).lw(c);
            c.g[V1][0] = key;
            c.cycles += 3;
            let best_key = Addr(best_object as u32).off(OBJECT_KEY).lw(c);
            c.g[V0][0] = (key < best_key) as u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 1;
        }
    } else {
        c.cycles += 3;
        let best_object = Addr(best).off(OBJECT).lw(c);
        c.g[V0][0] = 0;
        if best_object == 0 {
            c.cycles += 5;
            let best_surface = Addr(best).off(SURFACE).lw(c);
            c.g[V1][0] = best_surface;
            let surface = Addr(hit).off(SURFACE).lw(c);
            c.g[A0][0] = surface;
            let best_key = Addr(best_surface as u32).off(SURFACE_KEY).lw(c);
            c.g[A1][0] = best_key;
            let key = Addr(surface as u32).off(SURFACE_KEY).lw(c);
            c.g[V0][0] = (key < best_key) as u64;
        }
    }
    c.cycles += 1;
    if c.g[V0][0] != 0 {
        c.g[S1][0] = c.g[A3][0];
    }
}

// ------------------------------------------------ Rider_UpdateDistToFinish

/// `Rider` fields of the distance-to-finish bookkeeping.
mod dist {
    /// Position projected onto the course path.
    pub const POSITION: u32 = 0x110;
    /// Distance to the finish now and on the previous update.
    pub const DIST: u32 = 0x4d0;
    pub const PREV_DIST: u32 = 0x4d4;
    /// The rider's path segment cache (12 bytes) the path updates.
    pub const SEGMENT: u32 = 0x4dc;
    /// What the path's segment query returned.
    pub const SEGMENT_RESULT: u32 = 0x59c;
    /// The course path and its search hint.
    pub const PATH: u32 = 0xab4;
    pub const PATH_HINT: u32 = 0xac0;
    /// `Path`: vtable and total length.
    pub const PATH_VTABLE: u32 = 0x34;
    pub const PATH_LENGTH: u32 = 0x38;
    /// The split markers: `{?, float dist}` pairs, distance 0 ends the list.
    pub const SPLITS: u32 = 0x004d_33b8;
    /// `float Path_GetClosestPointFrom(Path*, vec* pos, vec* out, float*,
    /// hint, 1)`.
    pub const CLOSEST_POINT: u32 = 0x0026_a638;
}

/// `void Rider_UpdateDistToFinish(Rider* r, float* along, vec* out, vec*
/// closest)` (0x112fb0): project the rider onto the course path, store the
/// distance along it and the distance to the finish; when the rider got
/// closer, fire `Rider_OnSplitMarker` for each split marker (up to six)
/// passed since the last update and update the segment cache with the
/// new range; otherwise only refresh it at the old distance.
pub fn rider_update_dist_to_finish(c: &mut Cpu) {
    use self::dist::*;
    use crate::call::VSlot;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x60);
        c.g[T1][0] = 1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x50);
        let rider = Addr(r32(c, S1));
        c.g[A1][0] = sx(rider.off(POSITION).0);
        c.g[A0][0] = rider.off(PATH).lw(c);
        c.g[T0][0] = rider.off(PATH_HINT).lw(c);
        call_ptr(c, CLOSEST_POINT, 0x0011_2ff0)?;

        let along = FpuF32(c.f[0]);
        Addr(r32(c, S0)).write_fpu(c, along);
        let path = rider.off(PATH).lw(c);
        c.g[A3][0] = path;
        let prev = rider.off(PREV_DIST).read_fpu(c);
        c.f[12] = prev.0;
        let dist = Addr(path as u32).off(PATH_LENGTH).read_fpu(c) - along;
        c.f[1] = dist.0;
        let closer = dist.lt(prev);
        abi::set_fcc(c, closer);
        rider.off(DIST).write_fpu(c, dist);

        if !closer {
            // 0x001130e4: refresh the segment cache at the old distance.
            c.cycles += 9;
            let vtable = Addr(path as u32).off(PATH_VTABLE).lw(c);
            c.g[V1][0] = vtable;
            c.f[13] = c.f[12];
            c.g[A1][0] = sx(rider.off(SEGMENT).0);
            c.g[A2][0] = 0xc;
            let slot = VSlot::read(c, vtable, 0x8);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(path);
            call_ptr(c, slot.func as u32, 0x0011_3104)?;
            rider.off(SEGMENT_RESULT).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 10;
            c.g[V0][0] = sx(0x004d_33a0);
            c.f[2] = 0;
            c.g[S2][0] = 0;
            let first = Addr(SPLITS).off(4).read_fpu(c);
            c.f[0] = first.0;
            c.g[A1][0] = sx(SPLITS);
            let no_splits = first.eq_s(FpuF32(c.f[2]));
            abi::set_fcc(c, no_splits);
            c.g[S3][0] = sx(rider.off(SEGMENT).0);
            let mut reload_path = true;
            if !no_splits {
                c.cycles += 4;
                let all_ahead = first.lt(FpuF32(c.f[1]));
                abi::set_fcc(c, all_ahead);
                if all_ahead {
                    c.g[A3][0] = rider.off(PATH).lw(c);
                    reload_path = false;
                } else {
                    c.cycles += 3;
                    c.f[20] = c.f[2];
                    c.g[S0][0] = c.g[A1][0];
                    c.f[1] = Addr(r32(c, A1)).off(4).read_fpu(c).0;
                    // 0x00113058: s0 = marker, s2 = count, f1 = its distance.
                    loop {
                        c.cycles += 5;
                        let prev = rider.off(PREV_DIST).read_fpu(c);
                        c.f[0] = prev.0;
                        let crossed = FpuF32(c.f[1]).lt(prev);
                        abi::set_fcc(c, crossed);
                        if crossed {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x0010_e558,
                                f: ee::s00::Rider_OnSplitMarker,
                            }
                            .call(c, 0x0011_3074)?;
                        }
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                        c.cycles += 3;
                        let more = (c.g[S2][0] as i64) < 6;
                        c.g[V0][0] = more as u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
                        if !more {
                            break;
                        }
                        c.cycles += 5;
                        let next = Addr(r32(c, S0)).off(4).read_fpu(c);
                        c.f[1] = next.0;
                        let end = next.eq_s(FpuF32(c.f[20]));
                        abi::set_fcc(c, end);
                        c.g[A1][0] = c.g[S0][0];
                        if end {
                            break;
                        }
                        c.cycles += 5;
                        let now = rider.off(DIST).read_fpu(c);
                        c.f[0] = now.0;
                        let ahead = next.lt(now);
                        abi::set_fcc(c, ahead);
                        if ahead {
                            break;
                        }
                        c.f[1] = Addr(r32(c, A1)).off(4).read_fpu(c).0;
                        if event_due(c) {
                            deliver_event_here(c, 0x0011_3058);
                        }
                    }
                }
            }
            if reload_path {
                c.cycles += 1;
                c.g[A3][0] = rider.off(PATH).lw(c);
            }

            // 0x001130b0: update the segment cache over [dist, prev dist].
            c.cycles += 13;
            c.g[A1][0] = c.g[S3][0];
            c.f[12] = rider.off(PREV_DIST).read_fpu(c).0;
            c.g[A2][0] = 0xc;
            let path = c.g[A3][0];
            let vtable = Addr(path as u32).off(PATH_VTABLE).lw(c);
            c.g[V1][0] = vtable;
            c.f[13] = rider.off(DIST).read_fpu(c).0;
            let slot = VSlot::read(c, vtable, 0x8);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(path);
            call_ptr(c, slot.func as u32, 0x0011_30d4)?;
            let now = rider.off(DIST).read_fpu(c);
            c.f[0] = now.0;
            rider.off(SEGMENT_RESULT).sw(c, c.g[V0][0]);
            rider.off(PREV_DIST).write_fpu(c, now);
        }

        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------ ChaseCameraAlgorithm_BuildOutputView

/// Fields and globals of the chase camera's output view.
mod chase_view {
    /// The camera's look-at offset (a vec4 transformed by the base matrix).
    pub const OFFSET: u32 = 0x40;
    /// Set to 1.0 each build.
    pub const SCALE: u32 = 0x4c;
    /// The yaw (`+0x50`) and pitch (`+0x54`) angles, applied negated.
    pub const YAW: u32 = 0x50;
    pub const PITCH: u32 = 0x54;
    /// The output: position (`+0x60`) and orientation quaternion (`+0x70`).
    pub const OUT_POS: u32 = 0x60;
    pub const OUT_QUAT: u32 = 0x70;
    /// The base matrix and the pitch and yaw axes.
    pub const BASE: u32 = 0x004f_f1a0;
    pub const PITCH_AXIS: u32 = 0x004f_f150;
    pub const YAW_AXIS: u32 = 0x004f_f160;
    /// The `{0, 0, 0, 1}`-style row the rotations' last row is copied from.
    pub const LAST_ROW: u32 = 0x004f_f130;
    /// `void Math_SinCos(float a, float* sin, float* cos)`.
    pub const SIN_COS: u32 = 0x0031_be50;
    /// `void Mat4_ToPosQuat(Mat4* m, PosQuat* out)`.
    pub const TO_POS_QUAT: u32 = 0x0031_b748;
}

/// `lqc2 vf1..vf4, src; sqc2 vf1..vf4, dst`: a 4x4 matrix copy through VU0.
fn chase_copy_matrix(c: &mut Cpu, dst: u32, src: u32) {
    for i in 0..4 {
        c.lqc2(1 + i, src.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.sqc2(1 + i, dst.wrapping_add(0x10 * i as u32));
    }
}

/// `dst = a * b` on VU0 (rows of `b` combined through the rows of `a`):
/// `vf4..7 = a`, `vf8..11 = b`, `vf12..15` the product.
fn chase_mul_matrix(c: &mut Cpu, dst: u32, a: u32, b: u32) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    for i in 0..4 {
        c.lqc2(4 + i, a.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.lqc2(8 + i, b.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + i, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + i, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + i, Z));
        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(8 + i, W));
    }
    for i in 0..4 {
        c.sqc2(12 + i, dst.wrapping_add(0x10 * i as u32));
    }
}

/// One axis rotation of `ChaseCameraAlgorithm_BuildOutputView`: with the
/// angle already in f12, save the current matrix (at sp) to sp+0x40, get
/// sin/cos into `sp+sincos`, build the axis-angle rotation matrix at sp
/// (its last row copied from the quadword the original parks at sp+0x80),
/// then `sp = rotation * saved`.
fn chase_axis_rotation(c: &mut Cpu, axis: u32, sincos: u32, ret: u32) -> crate::call::Flow {
    use self::chase_view::*;
    use crate::ps2f::FpuF32;
    let base = sp(c);
    c.g[S1][0] = 0x0050_0000;
    c.g[S2][0] = sx(axis);
    c.g[S5][0] = sx(base);
    c.f[12] = (-FpuF32(c.f[12])).0;
    chase_copy_matrix(c, base.wrapping_add(0x40), base);
    c.g[S3][0] = sx(LAST_ROW);
    c.lq(V1, base.wrapping_add(0x70));
    c.lq(V0, LAST_ROW);
    c.g[A0][0] = sx(base.wrapping_add(sincos));
    c.g[A1][0] = sx(base.wrapping_add(sincos + 4));
    c.sq(V1, base.wrapping_add(0x80));
    c.sq(V0, base.wrapping_add(0x70));
    call_ptr(c, SIN_COS, ret)?;

    let f = |c: &mut Cpu, n: usize| FpuF32(c.f[n]);
    let cos = Addr(base.wrapping_add(sincos + 4)).read_fpu(c);
    c.f[11] = cos.0;
    let z = Addr(axis).off(8).read_fpu(c);
    c.f[3] = z.0;
    let t = f(c, 20) - cos; // 1 - cos
    c.f[1] = t.0;
    let x = Addr(axis).read_fpu(c);
    c.f[0] = x.0;
    let y = Addr(axis).off(4).read_fpu(c);
    c.f[7] = y.0;
    let sin = Addr(base.wrapping_add(sincos)).read_fpu(c);
    c.f[8] = sin.0;
    let tz = t * z;
    c.lq(V0, base.wrapping_add(0x80));
    let tx = t * x;
    let ty = t * y;
    let sz = sin * z;
    let sx_ = sin * x;
    let tzz = tz * z;
    let txz = tx * z;
    let tzx = tz * x;
    let txx = tx * x;
    let tyz = ty * z;
    let tyx = ty * x;
    let sy = sin * y;
    let tzy = tz * y;
    let txy = tx * y;
    let tyy = ty * y;
    // Rows 0..2 of the rotation, column 3 zero; row 3 from sp+0x80.
    let m = [
        (0x00, txx + cos, 6),
        (0x04, tyx - sz, 0),
        (0x08, tzx + sy, 5),
        (0x10, txy + sz, 4),
        (0x14, tyy + cos, 1),
        (0x18, tzy - sx_, 2),
        (0x20, txz - sy, 10),
        (0x24, tyz + sx_, 3),
        (0x28, tzz + cos, 9),
    ];
    for &(off, v, reg) in &m {
        c.f[reg] = v.0;
        Addr(base.wrapping_add(off)).write_fpu(c, v);
    }
    c.f[8] = sy.0;
    c.f[12] = sx_.0;
    c.f[13] = sz.0;
    for off in [0x0c, 0x1c, 0x2c, 0x30, 0x34, 0x38] {
        Addr(base.wrapping_add(off)).sw(c, 0);
    }
    let one = f(c, 20);
    Addr(base.wrapping_add(0x3c)).write_fpu(c, one);
    c.sq(V0, base.wrapping_add(0x30));

    chase_mul_matrix(c, base, base, base.wrapping_add(0x40));
    Ok(())
}

/// `void ChaseCameraAlgorithm_BuildOutputView(ChaseCameraAlgorithm* self)`
/// (0x166f90): the chase camera's output transform. Start from the base
/// matrix with its translation replaced by the transformed look-at offset,
/// rotate by -pitch about the pitch axis and by -yaw about the yaw axis,
/// split the result into position and quaternion (`Mat4_ToPosQuat`) and
/// store them with the quaternion conjugated.
pub fn chase_camera_build_output_view(c: &mut Cpu) {
    use self::chase_view::*;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 260;
        let frame = Frame::open(c, 0x120);
        c.g[V0][0] = sx(BASE);
        frame.save_fpr(c, 20, 0x110);
        frame.save(c, S0, 0x100);
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = 0x3f80_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0xf0);
        let this = Addr(r32(c, S0));
        c.g[V1][0] = sx(this.off(OFFSET).0);
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save_ra(c, 0xa0);
        let one = FpuF32(c.f[20]);
        this.off(SCALE).write_fpu(c, one);
        let base = sp(c);
        chase_copy_matrix(c, base, BASE);
        // Translation row = base matrix * offset.
        for i in 0..4 {
            c.lqc2(4 + i, base.wrapping_add(0x10 * i as u32));
        }
        c.lqc2(8, this.off(OFFSET).0);
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, base.wrapping_add(0x30));

        c.f[12] = this.off(PITCH).read_fpu(c).0;
        c.g[S4][0] = sx(base.wrapping_add(0x40));
        chase_axis_rotation(c, PITCH_AXIS, 0x90, 0x0016_7074)?;
        c.f[12] = this.off(YAW).read_fpu(c).0;
        chase_axis_rotation(c, YAW_AXIS, 0x98, 0x0016_71f4)?;

        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = sx(base);
        call_ptr(c, TO_POS_QUAT, 0x0016_7330)?;
        let base = sp(c);
        c.lq(V1, base.wrapping_add(0x50));
        c.lq(V0, base.wrapping_add(0x40));
        c.sq(V1, this.off(OUT_QUAT).0);
        c.sq(V0, this.off(OUT_POS).0);
        // Conjugate the quaternion (negate x, y, z).
        for i in 0..3u32 {
            let q = -this.off(OUT_QUAT + 4 * i).read_fpu(c);
            c.f[i as usize] = q.0;
            Addr(base.wrapping_add(0x50 + 4 * i)).write_fpu(c, q);
        }
        let w = this.off(OUT_QUAT + 0xc).read_fpu(c);
        c.f[3] = w.0;
        Addr(base.wrapping_add(0x5c)).write_fpu(c, w);
        c.lq(V0, base.wrapping_add(0x50));
        c.sq(V0, this.off(OUT_QUAT).0);
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 20, 0x110);
        c.sq(V0, base.wrapping_add(0x40));
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------ RiderAnim_UpdateSeqType02

/// `int RiderAnim_UpdateSeqType02(RiderAnim* anim, SeqList* list,
/// AnimSequence* seq, float dt)` (0x1049c0): a one-shot sequence. Advance
/// its track by `dt * (1/60)`; when the track has ended, or else when its
/// weight fade has finished, remove it from the list. Returns whether it
/// was removed.
pub fn rider_anim_seq_type02_update(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    /// `AnimSequence_AdvanceTrack(seq, 0, step)`: nonzero when it ended.
    const ADVANCE_TRACK: u32 = 0x0031_35b0;
    /// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
    const UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
    /// `AnimSeqList_Remove(list, seq)`.
    const LIST_REMOVE: u32 = 0x0031_45f8;
    /// `gp-0x7f3c`: 1/60, ticks to seconds.
    const TICK_GP: u32 = 0x7f3c;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        let k = Addr(r32(c, GP).wrapping_sub(TICK_GP)).read_fpu(c);
        c.f[0] = k.0;
        frame.save_fpr(c, 20, 0x30);
        let step = FpuF32(c.f[12]) * k;
        c.f[20] = step.0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = step.0;
        call_ptr(c, ADVANCE_TRACK, 0x0010_49f4)?;
        let ended = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !ended {
            c.cycles += 3;
            c.f[12] = c.f[20];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, UPDATE_WEIGHT_FADE, 0x0010_4a08)?;
        }
        c.cycles += 2;
        let remove = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if remove {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, LIST_REMOVE, 0x0010_4a18)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------ Rider_SetSlew3And4Targets

/// `Rider` slews #3 (brake/powerslide, `+0x214`) and #4 (crouch, `+0x220`):
/// current value, max step per tick and target at `+0, +4, +8`.
mod slew34 {
    pub const BRAKE_CUR: u32 = 0x214;
    pub const BRAKE_STEP: u32 = 0x218;
    pub const BRAKE_TARGET: u32 = 0x21c;
    pub const CROUCH_CUR: u32 = 0x220;
    pub const CROUCH_STEP: u32 = 0x224;
    pub const CROUCH_TARGET: u32 = 0x228;
    /// `vel` and `groundForward` (their x product decides "backward").
    pub const VEL: u32 = 0x1e0;
    pub const GROUND_FORWARD: u32 = 0x3a0;
    /// The lean (sign of the powerslide side).
    pub const LEAN: u32 = 0x1f0;
}

/// `void Rider_SetSlew3And4Targets(Rider* r, float crouch, float brake)`
/// (0x113f88): crouch and brake exclude each other: while one is asked
/// for and the other is still non-zero, only the other is sent to 0.
/// Otherwise the crouch slew gets `crouch` (0 while braking) with a step
/// proportional to the distance (`k * |d|`, floored), and the brake slew
/// gets `brake`, signed by the lean side, cleared while crouching or when
/// moving backward, with a step `k * min(max(|d|, lo), 1)`.
pub fn rider_set_slew3_and4_targets(c: &mut Cpu) {
    use self::slew34::*;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    let rider = Addr(abi::arg(c, 0));
    let gp = r32(c, GP);
    let lit = |c: &mut Cpu, off: u32| Addr(gp.wrapping_sub(off)).read_fpu(c);
    let f = |c: &mut Cpu, n: usize| FpuF32(c.f[n]);
    let zero = FpuF32::ZERO;

    // Crouching asked for while still braking: release the brake first.
    c.cycles += 4;
    c.f[1] = 0;
    let crouch_asked = zero.lt(f(c, 12));
    abi::set_fcc(c, crouch_asked);
    if crouch_asked {
        c.cycles += 4;
        let brake_cur = rider.off(BRAKE_CUR).read_fpu(c);
        c.f[0] = brake_cur.0;
        let released = brake_cur.eq_s(zero);
        abi::set_fcc(c, released);
        if !released {
            c.cycles += 4;
            let step = lit(c, 0x7c20);
            c.f[0] = step.0;
            rider.off(BRAKE_TARGET).write_fpu(c, zero);
            rider.off(BRAKE_STEP).write_fpu(c, step);
            ret(c);
            return;
        }
    }

    // Braking asked for while still crouched: stand up first.
    c.cycles += 4;
    let brake_asked = zero.lt(f(c, 13));
    abi::set_fcc(c, brake_asked);
    let crouch_cur = rider.off(CROUCH_CUR).read_fpu(c);
    c.f[2] = crouch_cur.0;
    if brake_asked {
        c.cycles += 3;
        let standing = crouch_cur.eq_s(zero);
        abi::set_fcc(c, standing);
        if !standing {
            c.cycles += 4;
            let step = lit(c, 0x7c1c);
            c.f[0] = step.0;
            rider.off(CROUCH_TARGET).write_fpu(c, zero);
            rider.off(CROUCH_STEP).write_fpu(c, step);
            ret(c);
            return;
        }
    }

    // Slew #4 (crouch): no crouching while the brake is still on.
    c.cycles += 5;
    c.f[1] = 0;
    let crouch_asked = zero.lt(f(c, 12));
    abi::set_fcc(c, crouch_asked);
    if crouch_asked {
        c.cycles += 5;
        let brake_cur = rider.off(BRAKE_CUR).read_fpu(c);
        c.f[0] = brake_cur.0;
        let released = brake_cur.eq_s(zero);
        abi::set_fcc(c, released);
        if !released {
            c.f[12] = 0;
        }
        c.cycles += 1;
    }
    let delta = f(c, 12) - f(c, 2);
    c.f[0] = delta.0;
    c.cycles += 5;
    let floor = lit(c, 0x7c18);
    c.f[1] = floor.0;
    let dist = delta.abs();
    c.f[2] = dist.0;
    let far = floor.le(dist);
    abi::set_fcc(c, far);
    let step = if far {
        c.cycles += 3;
        dist * lit(c, 0x7c14)
    } else {
        c.cycles += 1;
        lit(c, 0x7c10)
    };
    c.f[0] = step.0;

    // Slew #3 (brake): cleared while crouching or moving backward.
    c.cycles += 6;
    c.f[1] = 0;
    let crouch_target = f(c, 12);
    rider.off(CROUCH_TARGET).write_fpu(c, crouch_target);
    let brake_asked = zero.lt(f(c, 13));
    abi::set_fcc(c, brake_asked);
    rider.off(CROUCH_STEP).write_fpu(c, step);
    let mut charge_reload = true;
    if brake_asked {
        c.cycles += 5;
        let crouch_cur = rider.off(CROUCH_CUR).read_fpu(c);
        c.f[0] = crouch_cur.0;
        let crouching = zero.lt(crouch_cur);
        abi::set_fcc(c, crouching);
        if crouching {
            c.f[13] = 0;
        } else {
            c.cycles += 13;
            c.lqc2(3, rider.off(VEL).0);
            c.lqc2(5, rider.off(GROUND_FORWARD).0);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V0, 4);
            c.f[0] = c.g[V0][0] as u32;
            let backward = f(c, 0).lt(zero);
            abi::set_fcc(c, backward);
            if backward {
                c.cycles += 1;
                c.f[13] = 0;
            } else {
                charge_reload = false;
            }
        }
    }
    if charge_reload {
        c.cycles += 1;
    }
    let brake_cur = rider.off(BRAKE_CUR).read_fpu(c);
    c.f[0] = brake_cur.0;

    // Powerslide side: flip the target when leaning the other way.
    c.cycles += 6;
    let small = lit(c, 0x7c0c);
    c.f[3] = small.0;
    let size = brake_cur.abs();
    c.f[1] = size.0;
    let settled = size.le(small);
    abi::set_fcc(c, settled);
    c.f[2] = brake_cur.0;
    let mut flip = false;
    let mut checked = false;
    if settled {
        c.cycles += 6;
        let lean = rider.off(LEAN).read_fpu(c);
        c.f[1] = lean.0;
        c.f[0] = 0;
        let left = lean.lt(zero);
        abi::set_fcc(c, left);
        flip = left;
        checked = left;
    }
    if !checked {
        c.cycles += 5;
        let lo = lit(c, 0x7c08);
        c.f[0] = lo.0;
        let below = brake_cur.lt(lo);
        abi::set_fcc(c, below);
        if below {
            c.cycles += 5;
            let lean = rider.off(LEAN).read_fpu(c);
            c.f[0] = lean.0;
            let under = lean.lt(f(c, 3));
            abi::set_fcc(c, under);
            if under {
                c.cycles += 1;
            }
            flip = under;
        }
    }
    if flip {
        c.f[13] = (-f(c, 13)).0;
        c.cycles += 1;
    }
    let delta = f(c, 13) - brake_cur;
    c.f[0] = delta.0;

    // Step: k * min(max(|d|, lo), 1).
    c.cycles += 8;
    let lo = lit(c, 0x7c04);
    let dist = delta.abs();
    c.f[0] = dist.0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = 0x3f80_0000;
    let above = lo.le(dist);
    abi::set_fcc(c, above);
    let clamped = if above { dist.min(FpuF32(c.f[2])) } else { lo };
    c.f[1] = clamped.0;
    c.cycles += 5;
    let step = clamped * lit(c, 0x7c00);
    c.f[0] = step.0;
    let brake_target = f(c, 13);
    rider.off(BRAKE_TARGET).write_fpu(c, brake_target);
    rider.off(BRAKE_STEP).write_fpu(c, step);
    ret(c);
}

// ------------------------------------------------ RiderAirState_CollideWalls

/// Fields and callees of `RiderAirState_CollideWalls`.
mod air_walls {
    /// `RiderAirState`: the rider.
    pub const RIDER: u32 = 0x4;
    /// `Rider`: velocity, the controller (vtable slot `+0x40`: "use the
    /// small hull"), the hull's sphere tree, the overlap list and its flags.
    pub const VEL: u32 = 0x1e0;
    pub const POSITION_ROW: u32 = 0x180;
    pub const CONTROLLER: u32 = 0x6c0;
    pub const SPHERE_TREE: u32 = 0xaa0;
    pub const OVERLAP_LIST: u32 = 0x860;
    pub const OVERLAP_FLAGS: u32 = 0x868;
    pub const GET_SUB_STATE: u32 = 0x0011_fee8;
    pub const HULL_CTOR: u32 = 0x0032_f650;
    pub const HULL_DTOR: u32 = 0x0032_f708;
    pub const FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;
    pub const TRANSLATE: u32 = 0x0010_6538;
    pub const BUILD_WORLD_MATRIX: u32 = 0x0011_e098;
    pub const HANDLE_IMPACT: u32 = 0x0010_5d98;
    /// Sub state 9: no wall collision.
    pub const SUB_STATE_SKIP: u64 = 9;
}

/// `vf5 = vf4(at) * x` with `x` moved through a GPR (`mfc1; qmtc2`), stored
/// at `sp+out` and reloaded into v0 (`lq`): the original's vector scale.
fn air_walls_scale(c: &mut Cpu, gpr: usize, fpr: usize, at: u32, out: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    let base = sp(c);
    c.mfc1(gpr, fpr);
    c.lqc2(4, base.wrapping_add(at));
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, base.wrapping_add(out));
    c.lq(V0, base.wrapping_add(out));
}

/// `vf4 = vf3 * vf5; vf4.w = vf4.x + vf4.y + vf4.z` (the original's dot
/// product macro, `vf6.w = 1`).
fn air_walls_dot(c: &mut Cpu, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `int RiderAirState_CollideWalls(RiderAirState* self, vec* normal_out)`
/// (0x13aa48): build the rider's collision hull (the small one when the
/// controller says so), find the closest wall contact ahead of it, push the
/// rider out along the contact normal, and when the rider is moving into
/// the wall bounce its velocity off the normal (scaled by speed), rebuild
/// its world matrix and report the impact. Returns 1 (and the normal) on
/// an impact, else 0; sub state 9 skips everything.
///
/// Parked (not installed): its differential test could not be bounded (the
/// real callees under `OverlapList_FindClosestContact` / `Rider_HandleImpact`
/// run away on fixture data, pc 0x312138 in `AnimController_PlayAnimImpl`).
/// The port is a straight transliteration of the generated body.
#[allow(dead_code)]
pub fn rider_air_state_collide_walls(c: &mut Cpu) {
    use self::air_walls::*;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x490);
        frame.save(c, S1, 0x460);
        frame.save(c, S4, 0x430);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x470);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x450);
        frame.save(c, S3, 0x440);
        frame.save_ra(c, 0x420);
        frame.save_fpr(c, 21, 0x488);
        frame.save_fpr(c, 20, 0x480);
        let this = Addr(r32(c, S1));
        c.g[A0][0] = this.off(RIDER).lw(c);
        call_ptr(c, GET_SUB_STATE, 0x0013_aa7c)?;
        c.g[V1][0] = SUB_STATE_SKIP;
        let mut impact = false;
        if c.g[V0][0] == SUB_STATE_SKIP {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            let rider = this.off(RIDER).lw(c);
            c.g[S0][0] = rider;
            let rider = Addr(rider as u32);
            c.cycles += 29;
            let base = sp(c);
            let hull = base.wrapping_add(0x80);
            c.g[S2][0] = sx(hull);
            c.f[21] = 0;
            c.g[S3][0] = sx(hull);
            // controller->useSmallHull()
            let vtable = rider.off(CONTROLLER).lw(c);
            c.g[V1][0] = vtable;
            c.g[A1][0] = sx(rider.off(CONTROLLER).0);
            let slot = crate::call::VSlot::read(c, vtable, 0x40);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(c.g[A1][0]);
            call_ptr(c, slot.func as u32, 0x0013_aab4)?;
            // CollHull hull(rider->sphereTree, !small)
            c.g[A1][0] = rider.off(SPHERE_TREE).lw(c);
            c.g[A0][0] = sx(hull);
            c.g[A2][0] = c.g[V0][0] ^ 1;
            call_ptr(c, HULL_CTOR, 0x0013_aac4)?;
            // OverlapList_FindClosestContact(list, &hull, &contact, flags)
            let rider_now = this.off(RIDER).lw(c);
            c.g[V1][0] = rider_now;
            let rider_now = Addr(rider_now as u32);
            c.g[A0][0] = 1;
            c.g[A1][0] = sx(hull);
            c.g[A2][0] = sx(base);
            c.lq(V0, rider_now.off(POSITION_ROW).0);
            Addr(base.wrapping_add(0x88)).sw(c, 1);
            c.sq(V0, base.wrapping_add(0x90));
            c.sq(V0, base.wrapping_add(0x3a0));
            c.g[A3][0] = rider_now.off(OVERLAP_FLAGS).lw(c);
            c.g[A0][0] = rider_now.off(OVERLAP_LIST).lw(c);
            call_ptr(c, FIND_CLOSEST_CONTACT, 0x0013_aaf0)?;
            c.f[1] = c.f[0];
            let depth = FpuF32(c.f[1]);
            let missed = depth.lt(FpuF32(c.f[21]));
            abi::set_fcc(c, missed);
            c.g[A0][0] = c.g[S3][0];
            if !missed {
                // Push out: translate by normal * (k * depth).
                c.cycles += 40;
                c.f[0] = Addr(r32(c, GP).wrapping_sub(0x7190)).read_fpu(c).0;
                c.g[A1][0] = sx(base.wrapping_add(0x3a0));
                air_walls_scale(c, V1, 0, 0x10, 0x3c0);
                c.sq(V0, base.wrapping_add(0x3b0));
                air_walls_scale(c, V1, 1, 0x3b0, 0x3c0);
                c.g[A0][0] = this.off(RIDER).lw(c);
                c.sq(V0, base.wrapping_add(0x3a0));
                call_ptr(c, TRANSLATE, 0x0013_ab4c)?;
                // Into the wall? (-normal).x * vel.x < 0 means moving in.
                c.g[AT][0] = sx(0xbf80_0000);
                c.f[0] = 0xbf80_0000;
                air_walls_scale(c, V0, 0, 0x10, 0x3c0);
                let rider_now = this.off(RIDER).lw(c);
                c.g[A0][0] = rider_now;
                c.sq(V0, base.wrapping_add(0x3b0));
                c.lqc2(3, base.wrapping_add(0x3b0));
                c.lqc2(5, (rider_now as u32).wrapping_add(VEL));
                air_walls_dot(c, 5);
                c.qmfc2(V1, 4);
                c.f[20] = c.g[V1][0] as u32;
                let receding = FpuF32(c.f[20]).lt(FpuF32(c.f[21]));
                abi::set_fcc(c, receding);
                if receding {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                } else {
                    impact = true;
                }
            }
            if !impact {
                // 0x0013aba8: ~CollHull, no impact.
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, HULL_DTOR, 0x0013_abb0)?;
                c.g[V0][0] = 0;
            } else {
                // 0x0013abb8: bounce, scaled by speed (with a floor).
                c.cycles += 18;
                let rider_now = r32(c, A0);
                c.f[0] = Addr(r32(c, GP).wrapping_sub(0x718c)).read_fpu(c).0;
                c.lqc2(3, rider_now.wrapping_add(VEL));
                air_walls_dot(c, 3);
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, base.wrapping_add(0x3c0));
                let cap = Addr(r32(c, GP).wrapping_sub(0x7188)).read_fpu(c);
                c.f[1] = cap.0;
                let k = FpuF32(c.f[20]) * FpuF32(c.f[0]);
                c.f[0] = k.0;
                c.lq(V0, base.wrapping_add(0x3c0));
                let floored = k.lt(cap);
                abi::set_fcc(c, floored);
                c.sq(V0, base.wrapping_add(0x3b0));
                if floored {
                    c.cycles += 1;
                    c.f[0] = c.f[1];
                }
                c.cycles += 32;
                c.f[0] = (FpuF32(c.f[20]) + FpuF32(c.f[0])).0;
                air_walls_scale(c, V1, 0, 0x10, 0x3d0);
                let rider_now = this.off(RIDER).lw(c);
                c.g[V1][0] = rider_now;
                c.sq(V0, base.wrapping_add(0x3c0));
                let vel = (rider_now as u32).wrapping_add(VEL);
                c.lqc2(3, vel);
                c.lqc2(4, base.wrapping_add(0x3c0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, vel);
                c.g[A0][0] = this.off(RIDER).lw(c);
                call_ptr(c, BUILD_WORLD_MATRIX, 0x0013_ac40)?;
                // Rider_HandleImpact(rider, &{contact.pos, dir, normal, speed}, contact.surface)
                c.lq(V0, base.wrapping_add(0x3b0));
                c.g[A1][0] = sx(base.wrapping_add(0x3e0));
                c.lq(A3, base);
                c.lq(V1, base.wrapping_add(0x10));
                c.g[A0][0] = this.off(RIDER).lw(c);
                c.g[A2][0] = Addr(base.wrapping_add(0x4c)).lw(c);
                c.sq(V0, base.wrapping_add(0x3f0));
                c.sq(A3, base.wrapping_add(0x3e0));
                c.sq(V1, base.wrapping_add(0x400));
                let speed = FpuF32(c.f[20]);
                Addr(base.wrapping_add(0x410)).write_fpu(c, speed);
                call_ptr(c, HANDLE_IMPACT, 0x0013_ac6c)?;
                c.lq(V0, base.wrapping_add(0x10));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 2;
                c.sq(V0, r32(c, S4));
                call_ptr(c, HULL_DTOR, 0x0013_ac80)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x470);
        frame.restore(c, S1, 0x460);
        frame.restore(c, S2, 0x450);
        frame.restore(c, S3, 0x440);
        frame.restore(c, S4, 0x430);
        frame.restore_ra(c, 0x420);
        frame.restore_fpr(c, 21, 0x488);
        frame.restore_fpr(c, 20, 0x480);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    // ------------------------------------------------ tWScriptCache_GetObjectSlot

    /// `tWScriptCache_GetObjectSlot`: no script, no table, id -1, and real
    /// lookups of slot `i` in row `id >> 8`.
    #[test]
    fn wscript_cache_get_object_slot_matches() {
        check(
            ssx3_ee::s13::f_003ad120__generated,
            wscript_cache_get_object_slot,
            300,
            &|c, seed| {
                let mut r = Rng(0x3ad1_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_d120;
                let obj = DATA;
                let entry = DATA + 0x100;
                let script = DATA + 0x200;
                let table = DATA + 0x300;
                let rows = DATA + 0x1000;
                c.g[A0][0] = sx(entry);
                c.g[A1][0] = sx(obj);
                let i = r.next_u32() % 6;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | i as u64;
                c.bus
                    .write32(obj + 0x88, if seed % 5 == 0 { 0 } else { script });
                c.bus
                    .write32(entry + 0x8, if seed % 5 == 1 { 0 } else { table });
                let row = r.next_u32() % 8;
                let id = if seed % 5 == 2 {
                    0xffff_ffff
                } else {
                    row << 8 | (r.next_u32() & 0xff)
                };
                c.bus.write32(script + 0x8, id);
                c.bus.write32(table + 0x1c, rows);
                c.bus.write32(rows + row * 0x18 + i * 4, r.next_u32());
            },
        );
    }

    // ------------------------------------------------ Player_GetControlInput

    /// An input binding whose every id (0..0x40) runs a three-word input VM
    /// program `r0 = value; stop`, so `InputBinding_EvalBool/EvalFloat`
    /// return chosen values through the real interpreter.
    pub(super) fn gci_binding(c: &mut Cpu, r: &mut Rng, binding: u32) {
        let map = binding + 0x100;
        let offsets = binding + 0x200;
        let code = binding + 0x400;
        c.bus.write32(binding, r.next_u32());
        c.bus.write32(binding + 4, map);
        c.bus.write32(map + 4, offsets);
        c.bus.write32(map + 0x14, code);
        for id in 0..0x40u32 {
            c.bus.write32(offsets + 4 * id, 3 * id);
            let value = match r.next_u32() % 6 {
                0 => 0,
                1 => 1.0f32.to_bits(),
                2 => r.f32(1.5),
                3 => [0.5f32, -0.5, 0.500_001, -0.500_001][r.next_u32() as usize % 4].to_bits(),
                4 => r.edge_f32(),
                _ => (-1.0f32).to_bits(),
            };
            let at = code + 12 * id;
            c.bus.write32(at, 0); // op 0: r0 = the next word
            c.bus.write32(at + 4, value);
            c.bus.write32(at + 8, 0x20); // stop
        }
    }

    /// `Player_GetControlInput`: every substate of the jump table (and out
    /// of range), real button/stick reads through the input VM, the
    /// trick/uber-grind helpers, and with or without a replay cache.
    #[test]
    fn player_get_control_input_matches() {
        check(
            ssx3_ee::s00::f_00127998__generated,
            player_get_control_input,
            160,
            &|c, seed| {
                let mut r = Rng(0x0012_7998 ^ (seed as u64) << 20);
                crate::difftest::elf_image(c);
                scramble(c, &mut r);
                c.g[GP][0] = sx(crate::difftest::GAME_GP);
                c.pc = 0x0012_7998;
                let player = DATA;
                let input = DATA + 0x1000;
                let rider = DATA + 0x1100;
                let base = DATA + 0x2000;
                let binding = DATA + 0x4000;
                let replay = DATA + 0x6000;
                c.g[A0][0] = sx(player);
                c.g[A1][0] = sx(input);
                c.bus.write32(input, r.next_u32());
                c.bus.write32(input + 4, r.next_u32());
                c.bus.write32(player + 0x18, rider);
                c.bus.write32(rider + 0x77c, base);
                let substate = match seed % 17 {
                    n @ 0..=15 => n,
                    _ => 0xffff_fff0 | r.next_u32() % 16,
                };
                c.bus.write32(base + 0xde4, substate);
                c.bus.write32(player + 0xdf0, binding);
                gci_binding(c, &mut r, binding);
                // Replay cache: none, or one whose `+0xc` is 0 (returns at once).
                c.bus
                    .write32(player + 0xdf8, if seed % 3 == 0 { 0 } else { replay });
                c.bus.write32(replay + 0xc, 0);
            },
        );
    }

    // ------------------------------------------------ OverlapList_FindClosestContact

    /// Control block of the fake `OverlapList_CollideVolume`: a magic word
    /// (other tests' machines never have it, so their calls run the real
    /// body), the contact count and the template contacts to copy out.
    const FCC_CTRL: u32 = 0x00a0_0000;
    const FCC_MAGIC: u32 = 0xc011_1de5;

    /// `OverlapList_CollideVolume` for the fixture: copy `count` template
    /// contacts to `a2` and return the count (the real body needs a whole
    /// world of collision objects).
    fn fake_collide_volume(c: &mut Cpu) {
        if c.bus.read32(FCC_CTRL) != FCC_MAGIC {
            return ssx3_ee::s11::f_00333ef8__generated(c);
        }
        let count = c.bus.read32(FCC_CTRL + 4);
        let template = c.bus.read32(FCC_CTRL + 8);
        let out = c.g[A2][0] as u32;
        for i in 0..count * 0x20 {
            let w = c.bus.read32(template + 4 * i);
            c.bus.write32(out + 4 * i, w);
        }
        c.cycles += 40 + count as u64;
        c.g[V0][0] = count as u64;
        c.g[V1][0] = 0x1234;
        abi::ret(c);
    }

    /// Installs the gated fakes ([`fake_collide_volume`],
    /// [`fake_closest_point`]) in their callees' slots once for the whole
    /// test process: without their magic word in a machine's RAM they run
    /// the generated body, so other tests are unaffected (and no test
    /// restores a slot under another one running in parallel).
    fn install_fake_slots() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ssx3_ee::s11::OVERRIDE_00333EF8 = fake_collide_volume;
            ssx3_ee::s07::OVERRIDE_0026A638 = fake_closest_point;
        });
    }

    /// `OverlapList_FindClosestContact`: no contact, one, and up to 64 with
    /// distance ties broken by object key, object-vs-world and surface key.
    #[test]
    fn overlap_list_find_closest_contact_matches() {
        install_fake_slots();
        check(
            ssx3_ee::s11::f_003342d0__generated,
            overlap_list_find_closest_contact,
            200,
            &|c, seed| {
                let mut r = Rng(0x0033_42d0 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.pc = 0x0033_42d0;
                let list = DATA;
                let volume = DATA + 0x100;
                let vtable = DATA + 0x200;
                let out = DATA + 0x300;
                let template = DATA + 0x1_0000;
                let objects = DATA + 0x2_0000;
                let surfaces = DATA + 0x2_1000;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(volume);
                c.g[A2][0] = sx(out);
                c.g[A3][0] = r.next_u32() as u64;
                // Slot 3: `CollSegment_GetDiameter` (f0 = *(this + 0xa0)).
                c.bus.write32(volume + 0x50, vtable);
                c.bus.write16(vtable + 0x18, 0);
                c.bus.write32(vtable + 0x1c, 0x0034_0a10);
                let reference = [0.0f32, 1.0, 2.0, 0.5][r.next_u32() as usize % 4];
                c.bus.write32(volume + 0xa0, reference.to_bits());
                let count = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 64,
                    _ => 2 + r.next_u32() % 20,
                };
                c.bus.write32(FCC_CTRL, FCC_MAGIC);
                c.bus.write32(FCC_CTRL + 4, count);
                c.bus.write32(FCC_CTRL + 8, template);
                for k in 0..3 {
                    c.bus.write32(objects + 0x100 * k + 0x78, r.next_u32() % 3);
                    c.bus
                        .write32(surfaces + 0x200 * k + 0x150, r.next_u32() % 3);
                }
                for i in 0..count {
                    let hit = template + 0x80 * i;
                    for w in 0..0x20 {
                        c.bus.write32(hit + 4 * w, r.next_u32());
                    }
                    let dist = [0.0f32, 1.0, 2.0, 3.0, -1.0, 1.5][r.next_u32() as usize % 6];
                    c.bus.write32(hit + 0x40, dist.to_bits());
                    let object = match r.next_u32() % 3 {
                        0 => 0,
                        _ => objects + 0x100 * (r.next_u32() % 3),
                    };
                    c.bus.write32(hit + 0x50, object);
                    c.bus
                        .write32(hit + 0x54, surfaces + 0x200 * (r.next_u32() % 3));
                }
            },
        );
    }

    // ------------------------------------------------ Rider_UpdateDistToFinish

    /// Control block of the fake `Path_GetClosestPointFrom`: magic word
    /// (other tests run the real body) and the distance along it returns.
    const UDF_CTRL: u32 = 0x00a1_0000;
    const UDF_MAGIC: u32 = 0x0d15_7f1e;

    /// `Path_GetClosestPointFrom` for the fixture: return the chosen
    /// distance along the path in f0 (the real one walks a course path).
    fn fake_closest_point(c: &mut Cpu) {
        if c.bus.read32(UDF_CTRL) != UDF_MAGIC {
            return ssx3_ee::s07::f_0026a638__generated(c);
        }
        c.f[0] = c.bus.read32(UDF_CTRL + 4);
        c.f[1] = 0x4000_0000;
        c.g[V0][0] = 0x55;
        c.cycles += 31;
        abi::ret(c);
    }

    /// `Rider_UpdateDistToFinish`: farther (old-distance refresh), closer
    /// with no split list, all splits ahead, and crossing up to six split
    /// markers (the list ending on a 0 entry or one still ahead); the path's
    /// segment query is a fake that sometimes unwinds.
    #[test]
    fn rider_update_dist_to_finish_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        install_fake_slots();
        check(
            ssx3_ee::s00::f_00112fb0__generated,
            rider_update_dist_to_finish,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_2fb0, 1);
                c.pc = 0x0011_2fb0;
                let rider = DATA + 0x1000;
                let path = DATA + 0x3000;
                let vtable = DATA + 0x3100;
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = sx(DATA + 0x100);
                c.g[A3][0] = sx(DATA + 0x200);
                c.bus.write32(rider + 0xab4, path);
                c.bus.write32(rider + 0xac0, r.next_u32());
                c.bus.write32(rider + 0xec, r.next_u32());
                c.bus.write32(path + 0x34, vtable);
                c.bus.write16(vtable + 0x8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vtable + 0xc, FAKE);
                let length = 1000.0f32;
                c.bus.write32(path + 0x38, length.to_bits());
                let pick = |r: &mut Rng, set: &[f32]| set[r.next_u32() as usize % set.len()];
                let dist = pick(&mut r, &[100.0, 180.0, 250.0, 320.0, 400.0]);
                let prev = pick(&mut r, &[150.0, 250.0, 350.0, 450.0, 90.0]);
                c.bus.write32(UDF_CTRL, UDF_MAGIC);
                c.bus.write32(UDF_CTRL + 4, (length - dist).to_bits());
                c.bus.write32(rider + 0x4d4, prev.to_bits());
                // Split markers: distances (0 ends the list), mostly descending.
                let mut d = pick(&mut r, &[0.0, 120.0, 300.0, 440.0, 520.0]);
                for k in 0..8u32 {
                    c.bus.write32(0x004d_33b8 + 8 * k, r.next_u32());
                    c.bus.write32(0x004d_33bc + 8 * k, d.to_bits());
                    d = if r.next_u32() % 6 == 0 {
                        0.0
                    } else {
                        (d - 60.0).max(10.0)
                    };
                }
                // Rider_OnSplitMarker's feature switch: off (its short path).
                c.bus.write32(0x0053_08d0, 1 << 9);
            },
        );
    }

    // ------------------------------------------------ ChaseCameraAlgorithm_BuildOutputView

    /// `ChaseCameraAlgorithm_BuildOutputView`: random base matrix, axes,
    /// offset and angles through the real `Math_SinCos` and
    /// `Mat4_ToPosQuat`.
    #[test]
    fn chase_camera_build_output_view_matches() {
        check(
            ssx3_ee::s02::f_00166f90__generated,
            chase_camera_build_output_view,
            200,
            &|c, seed| {
                let mut r = Rng(0x0016_6f90 ^ (seed as u64) << 16);
                crate::difftest::elf_image(c);
                scramble(c, &mut r);
                c.g[GP][0] = sx(crate::difftest::GAME_GP);
                c.pc = 0x0016_6f90;
                let this = DATA;
                c.g[A0][0] = sx(this);
                for i in 0..0x20 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                for i in 0..4 {
                    c.bus.write32(this + 0x40 + 4 * i, r.f32(50.0));
                }
                c.bus.write32(this + 0x50, r.f32(3.5));
                c.bus.write32(this + 0x54, r.f32(1.5));
                // Base matrix: a rotation-ish matrix with a translation.
                for i in 0..16u32 {
                    let v = if i % 5 == 0 {
                        1.0f32.to_bits()
                    } else {
                        r.f32(0.7)
                    };
                    c.bus.write32(0x004f_f1a0 + 4 * i, v);
                }
                let axes = [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.6, 0.0, 0.8]];
                for (k, a) in [0x004f_f150u32, 0x004f_f160].into_iter().enumerate() {
                    let axis = axes[(k + seed as usize) % 3];
                    for (j, v) in axis.iter().enumerate() {
                        c.bus.write32(a + 4 * j as u32, v.to_bits());
                    }
                }
                for (j, v) in [0.0f32, 0.0, 0.0, 1.0].iter().enumerate() {
                    c.bus.write32(0x004f_f130 + 4 * j as u32, v.to_bits());
                }
                // The stack words the original parks and copies (sp+0x70).
                for k in 0..0x48u32 {
                    c.bus
                        .write32(crate::difftest::SP - 0x120 + 4 * k, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------ RiderAnim_UpdateSeqType02

    /// `RiderAnim_UpdateSeqType02`: the real track advance, weight fade and
    /// list removal on a random sequence (ended / faded / still running).
    #[test]
    fn rider_anim_seq_type02_update_matches() {
        use crate::difftest::fakes::{machine, ptr};
        check(
            ssx3_ee::s00::f_001049c0__generated,
            rider_anim_seq_type02_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_49c0, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0010_49c0;
                let seq = ptr(&mut r);
                let list = ptr(&mut r);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
                for off in [0x8, 0xc, 0x10, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                    let v = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(3.0),
                    };
                    c.bus.write32(seq + off, v);
                }
                c.bus.write32(seq + 0x1c, r.next_u32() & 1);
                c.bus.write32(seq + 0xa0, r.next_u32() & 1);
                let mut at = list + 4;
                for _ in 0..seed % 3 {
                    let other = ptr(&mut r) + 0x8;
                    c.bus.write32(at, other);
                    at = other + 200;
                }
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(0.05)
                };
            },
        );
    }

    // ------------------------------------------------ Rider_SetSlew3And4Targets

    /// `Rider_SetSlew3And4Targets`: crouch/brake asked or not (0, positive,
    /// negative, edge values), the other slew still on or settled, moving
    /// forward or backward, both lean sides and settled/unsettled brake.
    #[test]
    fn rider_set_slew3_and4_targets_matches() {
        check(
            ssx3_ee::s00::f_00113f88__generated,
            rider_set_slew3_and4_targets,
            400,
            &|c, seed| {
                let mut r = Rng(0x0011_3f88 ^ (seed as u64) << 12);
                crate::difftest::elf_image(c);
                scramble(c, &mut r);
                c.g[GP][0] = sx(crate::difftest::GAME_GP);
                c.pc = 0x0011_3f88;
                let rider = DATA;
                c.g[A0][0] = sx(rider);
                let pick = |r: &mut Rng| match r.next_u32() % 5 {
                    0 | 1 => 0,
                    2 => r.f32(1.0),
                    3 => 1.0f32.to_bits(),
                    _ => r.edge_f32(),
                };
                c.f[12] = pick(&mut r);
                c.f[13] = pick(&mut r);
                let small = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.f32(0.01),
                    2 => r.f32(1.0),
                    _ => r.edge_f32(),
                };
                c.bus.write32(rider + 0x214, small(&mut r));
                c.bus.write32(rider + 0x220, small(&mut r));
                c.bus.write32(rider + 0x1f0, small(&mut r));
                for k in 0..4 {
                    c.bus.write32(rider + 0x1e0 + 4 * k, r.f32(20.0));
                    c.bus.write32(rider + 0x3a0 + 4 * k, r.f32(1.0));
                }
            },
        );
    }

    // ------------------------------------------------ RiderAirState_CollideWalls

    /// `RiderAirState_CollideWalls`: sub state 9 (skipped), no contact, and
    /// a contact the rider recedes from (push-out through the real
    /// `Rider_Translate`). The overlap query is the gated fake
    /// `OverlapList_CollideVolume` under the real
    /// `OverlapList_FindClosestContact`; the controller's virtual call lands
    /// in the fakes, which sometimes unwind. The impact branch (bounce,
    /// world matrix, `Rider_HandleImpact`) is not reached: the real
    /// `Rider_HandleImpact` cannot be bounded on a fixture rider.
    #[test]
    #[ignore = "fixture runs away (pc 0x312138); the port is parked, not installed"]
    fn rider_air_state_collide_walls_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::put_vec;
        install_fake_slots();
        check(
            ssx3_ee::s01::f_0013aa48,
            rider_air_state_collide_walls,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_aa48, 4);
                crate::difftest::elf_image(c);
                c.pc = 0x0013_aa48;
                let f = |x: f32| x.to_bits();
                let this = DATA;
                let rider = DATA + 0x1000;
                let sub = DATA + 0x3000;
                let ctl_vt = DATA + 0x3100;
                let tree = DATA + 0x3200;
                let list = DATA + 0x3400;
                let out = DATA + 0x3500;
                let template = DATA + 0x1_0000;
                let hit_obj = DATA + 0x3a00;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(out);
                c.bus.write32(this + 0x4, rider);
                c.bus.write32(rider + 0x77c, sub);
                let sub_state = if seed % 11 == 0 { 9 } else { r.next_u32() % 8 };
                c.bus.write32(sub + 0xde4, sub_state);
                // Controller: vt[0x40] = { delta, fake } ("use the small hull").
                c.bus.write32(rider + 0x6c0, ctl_vt);
                c.bus.write16(ctl_vt + 0x40, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(ctl_vt + 0x44, FAKE);
                // Sphere tree: a single sphere (no children).
                c.bus.write32(rider + 0xaa0, tree);
                c.bus.write32(tree + 0x28, 0);
                put_vec(
                    c,
                    tree + 0x10,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                c.bus
                    .write32(tree + 0x20, f(0.5 + (r.next_u32() % 4) as f32));
                c.bus.write32(rider + 0x860, list);
                c.bus.write32(rider + 0x868, r.next_u32() % 0x40);
                put_vec(
                    c,
                    rider + 0x110,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                put_vec(
                    c,
                    rider + 0x180,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                // Velocity x shares the sign of every contact normal's x, so the rider
                // always recedes ((-n.x) * vel.x < 0): the impact path's
                // Rider_HandleImpact runs the rider's animation/reset machinery (a
                // runaway on a fixture rider, pc 0x312138 in
                // AnimController_PlayAnimImpl), so it is left untested.
                let side = if r.next_u32() & 1 == 0 { 1.0f32 } else { -1.0 };
                let vx = side * (1.0 + (r.next_u32() % 30) as f32);
                put_vec(c, rider + 0x1e0, [f(vx), r.f32(30.0), r.f32(30.0), 0]);
                let unit = |r: &mut Rng| {
                    let v = [
                        (r.next_u32() % 200) as f32 - 100.0,
                        (r.next_u32() % 200) as f32 - 100.0,
                        (r.next_u32() % 200) as f32 - 100.0 + 0.5,
                    ];
                    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                    [v[0] / n, v[1] / n, v[2] / n]
                };
                let fwd = unit(&mut r);
                put_vec(c, rider + 0x370, [f(fwd[0]), f(fwd[1]), f(fwd[2]), 0]);

                // The contacts the fake overlap query reports.
                let count = if seed % 5 == 1 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write32(FCC_CTRL, FCC_MAGIC);
                c.bus.write32(FCC_CTRL + 4, count);
                c.bus.write32(FCC_CTRL + 8, template);
                for i in 0..count {
                    let hit = template + 0x80 * i;
                    for k in 0..0x20 {
                        c.bus.write32(hit + 4 * k, r.next_u32());
                    }
                    put_vec(c, hit, [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)]);
                    let mut n = unit(&mut r);
                    n[0] = side * (n[0].abs() + 0.25);
                    put_vec(c, hit + 0x10, [f(n[0]), f(n[1]), f(n[2]), 0]);
                    put_vec(c, hit + 0x20, [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0]);
                    let depth = if seed % 6 == 2 {
                        -((r.next_u32() % 10) as f32) - 0.5
                    } else {
                        (r.next_u32() % 10) as f32 * 0.25
                    };
                    c.bus.write32(hit + 0x40, f(depth));
                    c.bus.write32(hit + 0x4c, r.next_u32() % 16);
                    c.bus
                        .write32(hit + 0x50, if seed % 3 == 0 { 0 } else { hit_obj });
                    c.bus.write32(hit + 0x54, 0);
                }
                c.bus.write32(hit_obj + 0xc, 0);
            },
        );
    }
}
