//! Staging module oc535 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc535`.

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
        0x0027_4318,
        in_seq_get_block_payload,
        "InSeq_GetBlockPayload",
    ),
    (0x0027_4398, in_seq_block_get_field4, "InSeq_BlockGetField4"),
    (0x0027_43c0, in_seq_block_get_size, "InSeq_BlockGetSize"),
    (
        0x0027_4808,
        in_seq_instance_set_speed,
        "InSeq_InstanceSetSpeed",
    ),
    (
        0x0027_4810,
        in_seq_instance_restart,
        "InSeq_InstanceRestart",
    ),
    (
        0x0027_4c58,
        in_seq_instance_get_data,
        "InSeq_InstanceGetData",
    ),
    (
        0x0027_4c60,
        in_seq_instance_get_owner,
        "InSeq_InstanceGetOwner",
    ),
    (
        0x0027_4c68,
        in_seq_get_channel_count,
        "InSeq_GetChannelCount",
    ),
    (
        0x0027_4d90,
        in_seq_find_channel_data,
        "InSeq_FindChannelData",
    ),
    (
        0x0027_66b0,
        script_list_set_paused_nested,
        "ScriptList_SetPausedNested",
    ),
    (
        0x0027_6a08,
        script_list_get_entry_ext_type,
        "ScriptList_GetEntryExtType",
    ),
    (0x0027_6ca8, script_list_count, "ScriptList_Count"),
];

/// `InSeq_GetBlockPayload` (0x274318, `cScriptPlayerEntity_GetHeader1`): the payload of block
/// `index` of a replay sequence. The block's table entry is `seq + 4*index`; its word at +8 is
/// the offset of the block header from `seq`. A header whose 16-bit kind is below 9 has no
/// payload (returns 0); any other returns `header + 8`.
pub fn in_seq_get_block_payload(c: &mut Cpu) {
    c.cycles += 8;
    let scaled = (c.g[A1][0] as u32) << 2;
    let entry = (c.g[A0][0] as u32).wrapping_add(scaled);
    c.g[A1][0] = sx(entry);
    c.g[V1][0] = Addr(entry.wrapping_add(0x8)).lw(c);
    let header = (c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32);
    c.g[A0][0] = sx(header);
    c.g[V0][0] = Addr(header).lhu(c);
    let kind_small = c.g[V0][0] < 0x9;
    c.g[V0][0] = 0;
    c.cycles += 2;
    if !kind_small {
        c.g[V0][0] = sx(header.wrapping_add(0x8));
    }
    ret(c);
}

/// `InSeq_BlockGetField4` (0x274398): the word at +4 of a sequence block.
pub fn in_seq_block_get_field4(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
    ret(c);
}

/// `InSeq_BlockGetSize` (0x2743c0): the size word at +0 of a sequence block.
pub fn in_seq_block_get_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    ret(c);
}

/// `InSeq_InstanceSetSpeed` (0x274808): stores the float in f12 as the instance speed at +0x24.
pub fn in_seq_instance_set_speed(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x24));
    ret(c);
}

/// `InSeq_InstanceRestart` (0x274810): restarts an instance, starting it again on the track
/// word at +0x1c (`InSeq_InstanceStart(inst, inst->track)`).
pub fn in_seq_instance_restart(c: &mut Cpu) {
    const INSTANCE_START: Callee = Callee {
        addr: 0x0027_4830,
        f: ee::s07::f_00274830,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
        INSTANCE_START.call(c, 0x0027_4820)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InSeq_InstanceGetData` (0x274c58): the data pointer at +4 of an instance.
pub fn in_seq_instance_get_data(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
    ret(c);
}

/// `InSeq_InstanceGetOwner` (0x274c60): the owner pointer at +8 of an instance.
pub fn in_seq_instance_get_owner(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
    ret(c);
}

/// `InSeq_GetChannelCount` (0x274c68): the byte at +1 of a sequence header.
pub fn in_seq_get_channel_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1)).lbu(c);
    ret(c);
}

/// `InSeq_FindChannelData` (0x274d90): walks `count` blocks of a sequence. The first block
/// is at `seq + u16 at +2`; each step adds the block's size word (`InSeq_BlockGetSize`).
/// Returns the address after the last block walked. The loop checks for due events at its
/// back edge, as the generated body does.
pub fn in_seq_find_channel_data(c: &mut Cpu) {
    const BLOCK_GET_SIZE: Callee = Callee {
        addr: 0x0027_43c0,
        f: ee::s07::f_002743c0,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2)).lhu(c);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                BLOCK_GET_SIZE.call(c, 0x0027_4dbc)?;
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_4db0);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_SetPausedNested` (0x2766b0): `ScriptList_SetPaused(list, a1, 0)`, a tail
/// wrapper that clears the nested flag.
pub fn script_list_set_paused_nested(c: &mut Cpu) {
    const SET_PAUSED: Callee = Callee {
        addr: 0x0027_7c08,
        f: ee::s07::f_00277c08,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        SET_PAUSED.call(c, 0x0027_66c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_GetEntryExtType` (0x276a08): the extension type of queue entry `index`
/// (`a1`) of a script list, when that entry is in the queue and its type word is below 29
/// (`29` is the first non-movie type). Otherwise 0x20. The queue is the list at +0xc.
pub fn script_list_get_entry_ext_type(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        CMD_QUEUE_COUNT.call(c, 0x0027_6a28)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0x20;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            CMD_QUEUE_AT.call(c, 0x0027_6a3c)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 29) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0x20;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0027_6a58);
                c.g[A1][0] = c.g[S0][0];
                CMD_QUEUE_AT.call(c, 0x0027_6a58)?;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_Count` (0x276ca8): the number of queued commands of a script list
/// (`ScriptCmdQueue_Count` of the queue at +0xc).
pub fn script_list_count(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        CMD_QUEUE_COUNT.call(c, 0x0027_6cb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA};

    #[test]
    fn in_seq_get_block_payload_matches() {
        check(
            ee::s07::f_00274318__generated,
            in_seq_get_block_payload,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274318, 0);
                let seq = DATA + 0x400;
                let index = seed % 16;
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = index as u64;
                let header_off = 0x200 + (seed % 8) * 0x10;
                c.bus.write32(seq + index * 4 + 8, header_off);
                // Kinds below 9 take the no-payload path, others return header + 8.
                let kind = if seed % 2 == 0 {
                    r.next_u32() % 9
                } else {
                    9 + r.next_u32() % 200
                };
                c.bus.write16(seq + header_off, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_block_get_field4_matches() {
        check(
            ee::s07::f_00274398__generated,
            in_seq_block_get_field4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274398, 0);
                let block = DATA + 0x500;
                c.g[A0][0] = sx(block);
                c.bus.write32(block + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_block_get_size_matches() {
        check(
            ee::s07::f_002743c0__generated,
            in_seq_block_get_size,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2743c0, 0);
                let block = DATA + 0x500;
                c.g[A0][0] = sx(block);
                c.bus.write32(block, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_set_speed_matches() {
        check(
            ee::s07::f_00274808__generated,
            in_seq_instance_set_speed,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274808, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.f[12] = r.f32(400.0);
            },
        );
    }

    #[test]
    fn in_seq_instance_restart_matches() {
        check(
            ee::s07::f_00274810__generated,
            in_seq_instance_restart,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274810, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                // Path: inactive (start now) or active; a pending start request or none.
                c.bus.write32(inst + 0x1c, r.next_u32());
                c.bus.write32(inst + 0x20, seed % 2);
                c.bus
                    .write32(inst + 0x3c, if seed % 3 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn in_seq_instance_get_data_matches() {
        check(
            ee::s07::f_00274c58__generated,
            in_seq_instance_get_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c58, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.bus.write32(inst + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_owner_matches() {
        check(
            ee::s07::f_00274c60__generated,
            in_seq_instance_get_owner,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c60, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.bus.write32(inst + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_get_channel_count_matches() {
        check(
            ee::s07::f_00274c68__generated,
            in_seq_get_channel_count,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c68, 0);
                let header = DATA + 0x600;
                c.g[A0][0] = sx(header);
                c.bus.write8(header + 1, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn in_seq_find_channel_data_matches() {
        check(
            ee::s07::f_00274d90__generated,
            in_seq_find_channel_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274d90, 0);
                let seq = DATA + 0x100;
                c.g[A0][0] = sx(seq);
                // Zero to three blocks; the first block is at seq + 0x40.
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write16(seq + 2, 0x40);
                for i in 0..0x100 {
                    c.bus
                        .write32(seq + 0x40 + i * 4, (r.next_u32() % 0x20) * 4 + 4);
                }
            },
        );
    }

    #[test]
    fn script_list_set_paused_nested_matches() {
        check(
            ee::s07::f_002766b0__generated,
            script_list_set_paused_nested,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2766b0, 0);
                let list = DATA + 0x700;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                // ScriptList_SetPaused's early exit: a state word other than 3.
                let states: [u32; 4] = [0, 1, 2, 4];
                c.bus.write32(list + 0xa4, states[(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn script_list_get_entry_ext_type_matches() {
        check(
            ee::s07::f_00276a08__generated,
            script_list_get_entry_ext_type,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x276a08, 0);
                let list = DATA + 0x200;
                let queue = list + 0xc;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (seed % 6) as u64;
                c.bus
                    .write32(queue + 0x94, if seed % 5 == 0 { 1 } else { 0 });
                c.bus.write32(queue + 0x8c, r.next_u32() % 6);
                c.bus.write32(queue + 0x90, r.next_u32() % 6);
                // Ring entries (28 bytes each): type words below and above 29.
                for k in 0..5 {
                    let v = if (seed + k) % 2 == 0 {
                        r.next_u32() % 29
                    } else {
                        29 + r.next_u32() % 100
                    };
                    c.bus.write32(queue + k * 28, v);
                }
            },
        );
    }

    #[test]
    fn script_list_count_matches() {
        check(ee::s07::f_00276ca8__generated, script_list_count, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x276ca8, 0);
            let list = DATA + 0x200;
            let queue = list + 0xc;
            c.g[A0][0] = sx(list);
            c.bus
                .write32(queue + 0x94, if seed % 4 == 0 { 1 } else { 0 });
            c.bus.write32(queue + 0x8c, r.next_u32() % 6);
            c.bus.write32(queue + 0x90, r.next_u32() % 6);
        });
    }
}
