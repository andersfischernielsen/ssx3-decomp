//! Staging module oc692 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc692`.

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
        0x003b_6788,
        snd_bank_relocate_patches,
        "SND_BankRelocatePatches",
    ),
    (0x003b_8160, snd_set_master_volume, "SND_SetMasterVolume"),
    (0x003b_c0b0, snd_init_pan_tables, "SND_InitPanTables"),
    (0x003b_c588, snd_random_seed, "SND_RandomSeed"),
    (0x003c_0c90, snd_set_default_config, "SND_SetDefaultConfig"),
    (0x003c_0e28, snd_apply_config, "SND_ApplyConfig"),
    (0x003c_1638, snd_init_iop_cmd_buffer, "SND_InitIopCmdBuffer"),
];

// ---------------------------------------------------------------------------
// SND_BankRelocatePatches
// ---------------------------------------------------------------------------

const BRP_PARSE_PATCH: Callee = Callee {
    addr: 0x003b_aac0,
    f: ee::s13::SND_ParsePatch,
};
const BRP_RELOCATE_PATCH_CALLBACK: Callee = Callee {
    addr: 0x003c_3ea8,
    f: ee::s13::SND_RelocatePatchCallback,
};
const BRP_FIRE_EVENTS: Callee = Callee {
    addr: 0x003b_5f98,
    f: ee::s13::SND_FireEvents,
};

/// Frame offset of the cursor into the tagged patch data.
pub(crate) const BRP_CURSOR: u32 = 0xd0;
/// Offset of the render-mode mask (`u16`) inside the parsed patch block that
/// `SND_ParsePatch` fills at the bottom of the frame.
pub(crate) const BRP_PATCH_FLAGS: u32 = 0x18;

/// `int SND_BankRelocatePatches(u32 renderMask, u8* taggedPatch, u8*
/// sampleData, int* firstOffset)` (0x3b6788): walk the tagged patches of a
/// loaded bank (the first tag at `taggedPatch + 4`). Every patch whose
/// render-mode flags overlap `renderMask` gets its sample offsets relocated
/// (`SND_RelocatePatchCallback`) and its events fired
/// (`SND_FireEvents(patch, 0)`). Returns 0. Frame 0x140 (the parsed patch
/// block at `0(sp)`, the cursor at `0xd0(sp)`).
pub fn snd_bank_relocate_patches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x140);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        frame.save(c, S1, 0x120);
        frame.save(c, S2, 0x110);
        c.g[S1][0] = sx(frame.at(c, BRP_CURSOR));
        frame.save(c, S3, 0x100);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0xf0);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x130);
        c.g[S4][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        let cursor = frame.at(c, BRP_CURSOR);
        c.sw(cursor, c.g[A1][0]);
        c.g[A0][0] = c.g[S1][0];
        loop {
            c.cycles += 7;
            c.g[A1][0] = c.g[SP][0];
            BRP_PARSE_PATCH.call(c, 0x003b_67c8)?;
            c.g[S0][0] = c.g[V0][0];
            let flags_at = frame.at(c, BRP_PATCH_FLAGS);
            c.g[V0][0] = c.lhu(flags_at);
            c.g[V0][0] &= c.g[S4][0];
            let render = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[SP][0];
            if render {
                c.cycles += 6;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S2][0];
                BRP_RELOCATE_PATCH_CALLBACK.call(c, 0x003b_67e8)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0;
                BRP_FIRE_EVENTS.call(c, 0x003b_67f4)?;
            }
            c.cycles += 2;
            let more_tags = c.g[S0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if more_tags {
                if event_due(c) {
                    deliver_event_here(c, 0x003b_67c0);
                }
                continue;
            }
            c.cycles += 9;
            frame.restore(c, S0, 0x130);
            c.g[V0][0] = 0;
            frame.restore(c, S1, 0x120);
            frame.restore(c, S2, 0x110);
            frame.restore(c, S3, 0x100);
            frame.restore(c, S4, 0xf0);
            frame.restore_ra(c, 0xe0);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// SND_SetMasterVolume
// ---------------------------------------------------------------------------

const SMV_LOCK: Callee = Callee {
    addr: 0x003b_58a0,
    f: ee::s13::SND_Lock,
};
const SMV_UNLOCK: Callee = Callee {
    addr: 0x003b_58d8,
    f: ee::s13::SND_Unlock,
};
const SMV_CHANNEL_CALC_VOLUME: Callee = Callee {
    addr: 0x003b_a7b0,
    f: ee::s13::SND_ChannelCalcVolume,
};
const SMV_CHANNEL_APPLY_VOLUME: Callee = Callee {
    addr: 0x003c_2a50,
    f: ee::s13::SND_ChannelApplyVolume,
};

/// The sound driver state block (`sndgs`).
pub(crate) const SMV_STATE: u32 = 0x0050_a8e8;
/// Master volume byte in the state block.
pub(crate) const SMV_MASTER_VOLUME: u32 = 0x175;
/// Channel count (`i16`) in the state block.
pub(crate) const SMV_CHANNEL_COUNT: u32 = 0x17e;
/// Channel array pointer in the state block.
pub(crate) const SMV_CHANNELS: u32 = 0x1f0;
/// Size of a channel record.
pub(crate) const SMV_CHANNEL_SIZE: u32 = 0x8c;

/// `void SND_SetMasterVolume(int volume)` (0x3b8160): under the sound lock,
/// store the master volume byte and recompute and apply the volume of every
/// channel that is in use (`+0x69 == 1`) and has a non-negative handle.
/// Frame 0x50 (s0-s3).
pub fn snd_set_master_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        SMV_LOCK.call(c, 0x003b_8184)?;
        c.g[V0][0] = 0x51_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
        let state = c.g[V0][0] as u32;
        c.g[V1][0] = c.lh(state.wrapping_add(SMV_CHANNEL_COUNT));
        let no_channels = (c.g[V1][0] as i64) <= 0;
        c.sb(state.wrapping_add(SMV_MASTER_VOLUME), c.g[S0][0]);
        'done: {
            if no_channels {
                break 'done;
            }
            c.cycles += 4;
            c.g[S2][0] = c.g[V0][0];
            c.g[S3][0] = 1;
            c.g[S0][0] = 0;
            loop {
                c.cycles += 5;
                let channels = c.lw((c.g[S2][0] as u32).wrapping_add(SMV_CHANNELS));
                c.g[V0][0] = channels;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x69));
                if c.g[V1][0] != c.g[S3][0] {
                    c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(SMV_CHANNEL_COUNT));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    if (c.g[V0][0] as i64) < 0 {
                        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(SMV_CHANNEL_COUNT));
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        SMV_CHANNEL_CALC_VOLUME.call(c, 0x003b_81d0)?;
                        c.g[A0][0] = c.g[S1][0];
                        SMV_CHANNEL_APPLY_VOLUME.call(c, 0x003b_81d8)?;
                        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(SMV_CHANNEL_COUNT));
                    }
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(SMV_CHANNEL_SIZE));
                if !more {
                    break 'done;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_81a8);
                }
            }
        }
        c.cycles += 10;
        SMV_UNLOCK.call(c, 0x003b_81f4)?;
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_CalcPanGains
// ---------------------------------------------------------------------------

const CPG_SINF: Callee = Callee {
    addr: 0x003b_c770,
    f: ee::s13::SND_Sinf,
};
const CPG_COSF: Callee = Callee {
    addr: 0x003b_c840,
    f: ee::s13::SND_Cosf,
};

// ---------------------------------------------------------------------------
// SND_RandomSeed
// ---------------------------------------------------------------------------

/// The initial random state words (six words at 0x44c690).
pub(crate) const RANDOM_SEED_TABLE: u32 = 0x0044_c690;
/// The random generator state (six words) that `SND_Random` steps.
pub(crate) const RANDOM_STATE: u32 = 0x0050_ac50;

/// `void SND_RandomSeed(unsigned seed)` (0x3bc588): state[i] = table[i] +
/// seed for the six state words of `SND_Random`.
pub fn snd_random_seed(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x51_0000;
    c.g[V1][0] = 0x45_0000;
    c.g[A2][0] = sx(RANDOM_STATE);
    c.g[V1][0] = sx(RANDOM_SEED_TABLE);
    c.g[A1][0] = 5;
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_c5a0);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// SND_Sinf / SND_Cosf
// ---------------------------------------------------------------------------

/// Reduce the angle in `f12` below 2*pi by subtracting 2*pi (f0) until it
/// fits, then leave its square in `f<square>`: the common head of
/// `SND_Sinf` and `SND_Cosf`. 7 cycles, plus 6 per subtraction.
fn trig_reduce_angle(c: &mut Cpu, square: usize, head: u32) {
    c.cycles += 7;
    c.g[AT][0] = 0x40c9_0fdb; // 2 * pi
    c.mtc1(AT, 0);
    c.c_lt_s(0, 12);
    if !c.fcc() {
        c.mul_s(square, 12, 12);
        return;
    }
    c.cycles += 1;
    c.sub_s(12, 12, 0);
    loop {
        c.cycles += 5;
        c.c_lt_s(0, 12);
        if c.fcc() {
            c.sub_s(12, 12, 0);
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        c.cycles += 1;
        c.mul_s(square, 12, 12);
        return;
    }
}

// ---------------------------------------------------------------------------
// SND_InitPanTables
// ---------------------------------------------------------------------------

const IPT_LOCK: Callee = Callee {
    addr: 0x003b_58a0,
    f: ee::s13::SND_Lock,
};
const IPT_MEMALLOC: Callee = Callee {
    addr: 0x003b_5c60,
    f: ee::s13::SND_MemAlloc,
};
const IPT_UNLOCK: Callee = Callee {
    addr: 0x003b_58d8,
    f: ee::s13::SND_Unlock,
};
const IPT_CALCPANGAINS: Callee = Callee {
    addr: 0x003b_c000,
    f: ee::s13::SND_CalcPanGains,
};

/// `void SND_InitPanTables(void)` (0x3bc0b0): build the 256-step pan tables.
/// Under the sound lock, allocate `outputchannels * 0x100` bytes
/// (`0x50ac30`), point the per-channel row pointers (`0x50ac38`) into it,
/// then for each of the 256 angle steps (position = step << 8): mono gets
/// the constant 0x7f; otherwise the position is located in the speaker
/// angle row of that channel count (`0x50a954`, six halfwords per count)
/// and `SND_CalcPanGains` gives the two neighbouring speakers' gains
/// (scaled by 255 and rounded half away from zero, `>> 8` into a byte).
/// Frame 0xb0 (s0-s7, fp).
#[allow(unused_labels)]
pub fn snd_init_pan_tables(c: &mut Cpu) {
    guest(c, |c| {
        'b_3bc134: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = 0x510000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5718));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S1][0] = 0x510000;
            IPT_LOCK.call(c, 0x003bc0ec)?;
            c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x47));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
            IPT_MEMALLOC.call(c, 0x003bc0f8)?;
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x53d0), c.g[V0][0]);
            IPT_UNLOCK.call(c, 0x003bc100)?;
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x47));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x510000;
            if t {
                break 'b_3bc134;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x53d0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x53c8));
                c.g[S0][0] = c.g[V1][0];
                'l_3bc118: loop {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = c.g[S0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x100));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003bc118);
                        }
                        continue 'l_3bc118;
                    } else {
                        break 'b_3bc134;
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[S4][0] = 0u64;
        c.g[S5][0] = 0x510000;
        c.g[V0][0] = 0x510000;
        'l_3bc140: loop {
            'b_3bc47c: {
                'b_3bc37c: {
                    'b_3bc2cc: {
                        'b_3bc2ac: {
                            'b_3bc290: {
                                'b_3bc284: {
                                    'b_3bc164: {
                                        c.cycles += 5;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x47));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[S1][0] = sx((c.g[S4][0] as u32) << 8);
                                        if t {
                                            break 'b_3bc164;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S5][0] as u32).wrapping_sub(0x53c8));
                                            c.g[V1][0] = 0x7f;
                                            let t = true;
                                            c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_3bc284;
                                            } else {
                                                break 'b_3bc164;
                                            }
                                        }
                                    }
                                    'b_3bc1b0: {
                                        c.cycles += 7;
                                        c.g[V1][0] = 0x510000;
                                        c.g[S0][0] = 0u64;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
                                        c.g[A2][0] = 0x510000;
                                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x47));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_3bc1b0;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x510000;
                                            c.g[V1][0] = 0x510000;
                                            c.g[A1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
                                            c.g[A0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x53c8));
                                            'l_3bc190: loop {
                                                c.cycles += 8;
                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[S4][0] as u32)));
                                                c.sb((c.g[V1][0] as u32), 0u64);
                                                c.g[V0][0] =
                                                    c.lbu((c.g[A1][0] as u32).wrapping_add(0x47));
                                                c.g[V0][0] = ((c.g[S0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003bc190);
                                                    }
                                                    continue 'l_3bc190;
                                                } else {
                                                    break 'b_3bc1b0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[V0][0] = 0x510000;
                                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x56ac));
                                    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                                    c.g[A0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x47));
                                    c.g[V0][0] = 0xc;
                                    c.mult(0, 2, 4, 2);
                                    c.g[S3][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A1][0] = c.lhu((c.g[S3][0] as u32));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
                                    if t {
                                        break 'b_3bc2ac;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
                                        if t {
                                            break 'b_3bc290;
                                        } else {
                                            'b_3bc22c: {
                                                c.cycles += 15;
                                                c.g[A2][0] =
                                                    c.lhu((c.g[S3][0] as u32).wrapping_add(0x2));
                                                c.g[A3][0] = c.g[S1][0];
                                                c.g[A0][0] = c.g[SP][0];
                                                IPT_CALCPANGAINS.call(c, 0x003bc1f8)?;
                                                c.g[AT][0] = 0x477f0000;
                                                c.g[AT][0] = c.g[AT][0] | 0xff00;
                                                c.mtc1(1, 2);
                                                c.lwc1(0, (c.g[SP][0] as u32));
                                                c.mtc1(0, 3);
                                                c.mul_s(0, 0, 2);
                                                c.g[AT][0] = 0x3f000000;
                                                c.mtc1(1, 1);
                                                c.c_lt_s(0, 3);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_3bc22c;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0xffffffffbf000000;
                                                    c.mtc1(1, 1);
                                                    break 'b_3bc22c;
                                                }
                                            }
                                            'b_3bc270: {
                                                c.cycles += 15;
                                                c.add_s(1, 1, 0);
                                                c.cvt_w_s(4, 1);
                                                c.mfc1(3, 4);
                                                c.g[V0][0] =
                                                    c.lw((c.g[S5][0] as u32).wrapping_sub(0x53c8));
                                                c.g[V1][0] =
                                                    sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                                                c.g[AT][0] = 0x3f000000;
                                                c.mtc1(1, 1);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[S4][0] as u32)));
                                                c.sb((c.g[V0][0] as u32), c.g[V1][0]);
                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.mul_s(0, 0, 2);
                                                c.c_lt_s(0, 3);
                                                let t = !c.fcc();
                                                c.g[A0][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_sub(0x53c8));
                                                if t {
                                                    break 'b_3bc270;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0xffffffffbf000000;
                                                    c.mtc1(1, 1);
                                                    break 'b_3bc270;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.add_s(1, 1, 0);
                                            c.cvt_w_s(4, 1);
                                            c.mfc1(3, 4);
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] =
                                                sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                                            break 'b_3bc284;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                let t = true;
                                c.sb((c.g[V0][0] as u32), c.g[V1][0]);
                                if t {
                                    break 'b_3bc47c;
                                } else {
                                    break 'b_3bc290;
                                }
                            }
                            c.cycles += 7;
                            c.g[V1][0] = 0xffff;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                            c.g[A3][0] = c.g[S1][0];
                            c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_sub(0x2));
                            c.g[A0][0] = c.g[SP][0];
                            let t = true;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                            if t {
                                break 'b_3bc2cc;
                            } else {
                                break 'b_3bc2ac;
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                        c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_sub(0x2));
                        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0xffff;
                        if t {
                            break 'b_3bc37c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                            c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                            break 'b_3bc2cc;
                        }
                    }
                    'b_3bc308: {
                        c.cycles += 13;
                        IPT_CALCPANGAINS.call(c, 0x003bc2d4)?;
                        c.g[AT][0] = 0x477f0000;
                        c.g[AT][0] = c.g[AT][0] | 0xff00;
                        c.mtc1(1, 2);
                        c.lwc1(0, (c.g[SP][0] as u32));
                        c.mtc1(0, 3);
                        c.mul_s(0, 0, 2);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 1);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();

                        if t {
                            break 'b_3bc308;
                        } else {
                            c.cycles += 2;
                            c.g[AT][0] = 0xffffffffbf000000;
                            c.mtc1(1, 1);
                            break 'b_3bc308;
                        }
                    }
                    'b_3bc34c: {
                        c.cycles += 15;
                        c.add_s(1, 1, 0);
                        c.cvt_w_s(4, 1);
                        c.mfc1(3, 4);
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x53c8));
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                        c.sb((c.g[V0][0] as u32), c.g[V1][0]);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();
                        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x53c8));
                        if t {
                            break 'b_3bc34c;
                        } else {
                            c.cycles += 2;
                            c.g[AT][0] = 0xffffffffbf000000;
                            c.mtc1(1, 1);
                            break 'b_3bc34c;
                        }
                    }
                    c.cycles += 12;
                    c.add_s(1, 1, 0);
                    c.cvt_w_s(4, 1);
                    c.mfc1(2, 4);
                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x47));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    let t = true;
                    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                    if t {
                        break 'b_3bc47c;
                    } else {
                        break 'b_3bc37c;
                    }
                }
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_3bc47c;
                } else {
                    c.cycles += 3;
                    c.g[S7][0] = c.g[S2][0];
                    c.g[S6][0] = 0x510000;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
                    'l_3bc390: loop {
                        'b_3bc468: {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                            c.g[A1][0] = c.lhu((c.g[V1][0] as u32));
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_3bc468;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                c.g[A0][0] = c.g[SP][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A3][0] = c.g[S1][0];
                                if t {
                                    break 'b_3bc468;
                                } else {
                                    'b_3bc3f8: {
                                        c.cycles += 13;
                                        IPT_CALCPANGAINS.call(c, 0x003bc3c4)?;
                                        c.lwc1(0, (c.g[SP][0] as u32));
                                        c.g[AT][0] = 0x477f0000;
                                        c.g[AT][0] = c.g[AT][0] | 0xff00;
                                        c.mtc1(1, 2);
                                        c.mtc1(0, 3);
                                        c.mul_s(0, 0, 2);
                                        c.g[AT][0] = 0x3f000000;
                                        c.mtc1(1, 1);
                                        c.c_lt_s(0, 3);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_3bc3f8;
                                        } else {
                                            c.cycles += 2;
                                            c.g[AT][0] = 0xffffffffbf000000;
                                            c.mtc1(1, 1);
                                            break 'b_3bc3f8;
                                        }
                                    }
                                    'b_3bc444: {
                                        c.cycles += 17;
                                        c.add_s(1, 1, 0);
                                        c.cvt_w_s(4, 1);
                                        c.mfc1(4, 4);
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x53c8));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 8) as u32);
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                        c.g[AT][0] = 0x3f000000;
                                        c.mtc1(1, 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                        c.sb((c.g[V0][0] as u32), c.g[A0][0]);
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.mul_s(0, 0, 2);
                                        c.c_lt_s(0, 3);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_3bc444;
                                        } else {
                                            c.cycles += 2;
                                            c.g[AT][0] = 0xffffffffbf000000;
                                            c.mtc1(1, 1);
                                            break 'b_3bc444;
                                        }
                                    }
                                    c.cycles += 9;
                                    c.add_s(1, 1, 0);
                                    c.cvt_w_s(4, 1);
                                    c.mfc1(4, 4);
                                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 8) as u32);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                    c.sb((c.g[V0][0] as u32), c.g[A0][0]);
                                    break 'b_3bc468;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lbu((c.g[S7][0] as u32).wrapping_add(0x47));
                        c.g[S0][0] = c.g[S2][0];
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003bc390);
                            }
                            continue 'l_3bc390;
                        } else {
                            break 'b_3bc47c;
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[S4][0] = c.g[FP][0];
            c.g[V0][0] = ((c.g[S4][0] as i64) < 256i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = 0x510000;
                if event_due(c) {
                    deliver_event_here(c, 0x003bc140);
                }
                continue 'l_3bc140;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SND_SetDefaultConfig
// ---------------------------------------------------------------------------

/// `int SND_SetDefaultConfig(void)` (0x3c0c90): fill the sound system
/// capabilities (`sndgs.sso.cap`: 24-48 kHz, 1-5 output channels, 0x20 main
/// CPU / 0x10 IOP / 0x30 SPU voices, six render modes) and the default
/// settings (`sso.set`: 48 kHz stereo, S/PDIF mode 1, 8 main CPU and 8 IOP
/// voices, 0x30 SPU voices, 8 streams, resample quality 25, the virtual
/// speaker layouts of 1-5 channels, the IOP module path from `0x44dd3c`).
/// Straight-line stores; leaves its constants in the scratch registers.
/// Frame 0x90 (s0-s7, fp).
#[allow(unused_labels)]
pub fn snd_set_default_config(c: &mut Cpu) {
    c.cycles += 101;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.g[V0][0] = 0x450000;
    c.sq(30, (c.g[SP][0] as u32));
    c.g[T1][0] = 0x510000;
    c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x22c4));
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x5718));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[V0][0] = 0x5dc0;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[T3][0] = 0xbb80;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[A1][0] = 0x1;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[T0][0] = 0x8;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S1][0] = 0x30;
    c.sh((c.g[T1][0] as u32).wrapping_sub(0x5718), c.g[V0][0]);
    c.g[S2][0] = 0x28;
    c.g[S3][0] = 0x24;
    c.g[S4][0] = 0x120;
    c.g[A0][0] = 0x20;
    c.g[A2][0] = 0x10;
    c.g[A3][0] = 0x5;
    c.g[T2][0] = 0x6;
    c.g[T4][0] = 0x48;
    c.g[T5][0] = 0x44;
    c.g[T6][0] = 0x140;
    c.g[T7][0] = 0x3;
    c.g[S0][0] = 0x2;
    c.sb((c.g[V1][0] as u32).wrapping_add(0x6), c.g[A0][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x7), c.g[A2][0]);
    c.g[T9][0] = 0x19;
    c.sb((c.g[V1][0] as u32).wrapping_add(0x5), c.g[A3][0]);
    c.g[T1][0] = 0xe000;
    c.sb((c.g[V1][0] as u32).wrapping_add(0x13), c.g[T2][0]);
    c.g[T8][0] = 0x4000;
    c.sh((c.g[V1][0] as u32).wrapping_add(0x4e), c.g[S2][0]);
    c.g[T2][0] = 0xc000;
    c.sh((c.g[V1][0] as u32).wrapping_add(0x50), c.g[S3][0]);
    c.g[A3][0] = 0x2aaa;
    c.sh((c.g[V1][0] as u32).wrapping_add(0x52), c.g[S4][0]);
    c.g[A2][0] = 0x8000;
    c.sb((c.g[V1][0] as u32).wrapping_add(0x30), c.g[S1][0]);
    c.g[A0][0] = 0xd555;
    c.sb((c.g[V1][0] as u32).wrapping_add(0x47), c.g[S0][0]);
    c.g[V0][0] = 0u64;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[FP][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S5][0] = 0xa000;
    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S6][0] = 0x6000;
    c.sh((c.g[V1][0] as u32).wrapping_add(0x1a), c.g[T4][0]);
    c.g[S7][0] = 0x2000;
    c.sh((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T5][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x1e), c.g[T6][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[T7][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x46), c.g[T0][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x2c), c.g[T3][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x3f), c.g[A1][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[T3][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x49), c.g[A1][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x14), c.g[S2][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x16), c.g[S3][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x18), c.g[S4][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x3e), 0u64);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x2e), c.g[T0][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x2f), c.g[T0][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x2a), c.g[T3][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x42), 0u64);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x40), c.g[T9][0]);
    c.sb((c.g[V1][0] as u32).wrapping_add(0x41), c.g[T9][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0xaa), c.g[S7][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0xac), c.g[S6][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0xae), c.g[S5][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S7][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x9e), c.g[S6][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, (c.g[SP][0] as u32));
    c.sh((c.g[V1][0] as u32).wrapping_add(0xb0), c.g[T1][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x84), c.g[T8][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x86), c.g[T2][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x90), c.g[A3][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x92), c.g[A2][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x94), c.g[A0][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0x78), 0u64);
    c.sh((c.g[V1][0] as u32).wrapping_add(0xa2), c.g[T1][0]);
    c.sh((c.g[V1][0] as u32).wrapping_add(0xa8), 0u64);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// SND_ApplyConfig
// ---------------------------------------------------------------------------

const ACFG_CHANNELAPPLYPAN: Callee = Callee {
    addr: 0x003c_38e0,
    f: ee::s13::SND_ChannelApplyPan,
};

/// `int SND_ApplyConfig(void)` (0x3c0e28): validate and apply the requested
/// sound settings (`0x50a908`, copied to the stack). If the system is
/// already installed the previous settings are restored first and a changed
/// S/PDIF mode makes it return 0 at once. Output rates snap to 24, 36 or 48
/// kHz; S/PDIF mode bits force 4+ channels / 2 channels and the SPU voice
/// count; channels, voice counts, stream count and DTS quality are clamped
/// to the capabilities; the resample quality goes to the EE mixer
/// (`SND_EeMixerSetMode`); a fast-RAM address is aligned to 16. Installed:
/// the resample qualities are stored, and when the channel count changed
/// every live channel is re-panned (`SND_ChannelApplyPan`). Otherwise the
/// total voice count is computed and the settings stored. Returns 0.
/// Frame 0xf0 (s0-s3).
#[allow(unused_labels)]
pub fn snd_apply_config(c: &mut Cpu) {
    guest(c, |c| {
        'b_3c0ee8: {
            'b_3c0eb0: {
                c.cycles += 11;
                c.g[V1][0] = 0x510000;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x56f8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[A0][0] = c.g[SP][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                if t {
                    break 'b_3c0eb0;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[S1][0] = 0x510000;
                    'l_3c0e5c: loop {
                        c.cycles += 19;
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(5, (c.g[V1][0] as u32));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(5, (c.g[A0][0] as u32));
                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003c0e5c);
                            }
                            continue 'l_3c0e5c;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                            if t {
                                break 'b_3c0ee8;
                            } else {
                                break 'b_3c0eb0;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[S1][0] = 0x510000;
            'l_3c0eb8: loop {
                c.cycles += 11;
                c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003c0eb8);
                    }
                    continue 'l_3c0eb8;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                    break 'b_3c0ee8;
                }
            }
        }
        'b_3c1278: {
            'b_3c0f9c: {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x174));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_3c0f9c;
                } else {
                    'b_3c0f8c: {
                        'b_3c0f5c: {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd4));
                            c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x174));
                            if t {
                                break 'b_3c0f5c;
                            } else {
                                'l_3c0f08: loop {
                                    c.cycles += 19;
                                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.ldr(5, (c.g[V1][0] as u32));
                                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.sdr(5, (c.g[A0][0] as u32));
                                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003c0f08);
                                        }
                                        continue 'l_3c0f08;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                                        if t {
                                            break 'b_3c0f8c;
                                        } else {
                                            break 'b_3c0f5c;
                                        }
                                    }
                                }
                            }
                        }
                        'l_3c0f5c: loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003c0f5c);
                                }
                                continue 'l_3c0f5c;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                                break 'b_3c0f8c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x29));
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xfd));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_3c1278;
                    } else {
                        break 'b_3c0f9c;
                    }
                }
            }
            'b_3c0fcc: {
                'b_3c0fc8: {
                    'b_3c0fb4: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0xa));
                        c.g[V1][0] = (c.g[V0][0] < 0x5dc1u64) as u64;
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = 0x8ca0;
                        if t {
                            break 'b_3c0fb4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x5dc0;
                            if t {
                                break 'b_3c0fc8;
                            } else {
                                break 'b_3c0fb4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xbb80;
                    if t {
                        break 'b_3c0fc8;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sh((c.g[SP][0] as u32).wrapping_add(0xa), c.g[V1][0]);
                        if t {
                            break 'b_3c0fcc;
                        } else {
                            break 'b_3c0fc8;
                        }
                    }
                }
                c.cycles += 1;
                c.sh((c.g[SP][0] as u32).wrapping_add(0xa), c.g[V0][0]);
                break 'b_3c0fcc;
            }
            'b_3c0ffc: {
                'b_3c0ff8: {
                    'b_3c0fe4: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = (c.g[V0][0] < 0x5dc1u64) as u64;
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = 0x8ca0;
                        if t {
                            break 'b_3c0fe4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x5dc0;
                            if t {
                                break 'b_3c0ff8;
                            } else {
                                break 'b_3c0fe4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xbb80;
                    if t {
                        break 'b_3c0ff8;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        if t {
                            break 'b_3c0ffc;
                        } else {
                            break 'b_3c0ff8;
                        }
                    }
                }
                c.cycles += 1;
                c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                break 'b_3c0ffc;
            }
            'b_3c1024: {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x29));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x27));
                if t {
                    break 'b_3c1024;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = 0x18;
                    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    if t {
                        break 'b_3c1024;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 0x4;
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x27), c.g[V0][0]);
                        break 'b_3c1024;
                    }
                }
            }
            'b_3c104c: {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x29));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x27));
                if t {
                    break 'b_3c104c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        break 'b_3c104c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x27), c.g[V0][0]);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        break 'b_3c104c;
                    }
                }
            }
            'b_3c1064: {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a));
                c.g[V0][0] = (c.g[V0][0] < 0x65u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                if t {
                    break 'b_3c1064;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2a), c.g[V0][0]);
                    break 'b_3c1064;
                }
            }
            'b_3c1090: {
                'b_3c108c: {
                    c.cycles += 5;
                    c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x27));
                    c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x27), c.g[V1][0]);
                        break 'b_3c108c;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
                        c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x27));
                            break 'b_3c1090;
                        } else {
                            c.cycles += 1;
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x27), c.g[V1][0]);
                            break 'b_3c108c;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x27));
                break 'b_3c1090;
            }
            'b_3c10a4: {
                c.cycles += 3;
                c.g[V0][0] = 0x5;
                if c.g[V1][0] != c.g[V0][0] {
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                    break 'b_3c10a4;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    break 'b_3c10a4;
                }
            }
            'b_3c10bc: {
                c.cycles += 6;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0xe));
                c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sb((c.g[SP][0] as u32).wrapping_add(0xe), c.g[V1][0]);
                    break 'b_3c10bc;
                } else {
                    break 'b_3c10bc;
                }
            }
            'b_3c10d0: {
                c.cycles += 5;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0xf));
                c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sb((c.g[SP][0] as u32).wrapping_add(0xf), c.g[V1][0]);
                    break 'b_3c10d0;
                } else {
                    break 'b_3c10d0;
                }
            }
            'b_3c10e4: {
                c.cycles += 5;
                c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    break 'b_3c10e4;
                } else {
                    break 'b_3c10e4;
                }
            }
            'b_3c10f8: {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x26));
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x20;
                if t {
                    break 'b_3c10f8;
                } else {
                    c.cycles += 1;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x26), c.g[V0][0]);
                    break 'b_3c10f8;
                }
            }
            'b_3c1130: {
                c.cycles += 5;
                c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                call_ptr(c, 0x003c8148, 0x003c1100)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                if t {
                    break 'b_3c1130;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0xf;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf));
                    if t {
                        break 'b_3c1130;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x10));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                        break 'b_3c1130;
                    }
                }
            }
            'b_3c1274: {
                'b_3c11c4: {
                    c.cycles += 4;
                    c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5718));
                    c.g[V0][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x174));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x21));
                    if t {
                        break 'b_3c11c4;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = 0x510000;
                        c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x4650));
                        c.sb((c.g[T0][0] as u32).wrapping_add(0x40), c.g[A1][0]);
                        c.sb((c.g[T0][0] as u32).wrapping_add(0x41), c.g[V1][0]);
                        c.sb((c.g[A0][0] as u32).wrapping_add(0x13), c.g[V1][0]);
                        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x27));
                        c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0xfb));
                        let t = c.g[V0][0] == c.g[V1][0];
                        c.g[S0][0] = 0u64;
                        if t {
                            break 'b_3c1274;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lh((c.g[T0][0] as u32).wrapping_add(0x17e));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.sb((c.g[T0][0] as u32).wrapping_add(0x47), c.g[V1][0]);
                            if t {
                                break 'b_3c1274;
                            } else {
                                c.cycles += 3;
                                c.g[S2][0] = c.g[T0][0];
                                c.g[S3][0] = 0x1;
                                c.g[S1][0] = 0u64;
                                'l_3c1180: loop {
                                    'b_3c11ac: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f0));
                                        c.g[V0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x69));
                                        if c.g[V1][0] != c.g[S3][0] {
                                            c.g[V0][0] =
                                                c.lh((c.g[S2][0] as u32).wrapping_add(0x17e));
                                            break 'b_3c11ac;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                                            if (c.g[V0][0] as i64) >= 0 {
                                                c.g[V0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x17e));
                                                break 'b_3c11ac;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = c.g[S0][0];
                                                ACFG_CHANNELAPPLYPAN.call(c, 0x003c11a8)?;
                                                c.g[V0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x17e));
                                                break 'b_3c11ac;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003c1180);
                                        }
                                        continue 'l_3c1180;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_3c1278;
                                        } else {
                                            break 'b_3c11c4;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'b_3c1244: {
                    c.cycles += 10;
                    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0xe));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0xf));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.g[A3][0] & 0x7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    let t = c.g[A1][0] == 0u64;
                    c.sh((c.g[T0][0] as u32).wrapping_add(0x17e), c.g[V0][0]);
                    if t {
                        break 'b_3c1244;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        'l_3c11f0: loop {
                            c.cycles += 19;
                            c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                            c.ldr(3, (c.g[A2][0] as u32));
                            c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                            c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x17));
                            c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x18));
                            c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                            c.sdr(3, (c.g[A3][0] as u32));
                            c.sdl(4, (c.g[A3][0] as u32).wrapping_add(0xf));
                            c.sdr(4, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0x17));
                            c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003c11f0);
                                }
                                continue 'l_3c11f0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_3c1278;
                                } else {
                                    break 'b_3c1244;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                'l_3c1248: loop {
                    c.cycles += 11;
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32));
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                    c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003c1248);
                        }
                        continue 'l_3c1248;
                    } else {
                        break 'b_3c1274;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_3c1278;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        return Ok(());
    })
}

// ---------------------------------------------------------------------------
// SND_InitIopCmdBuffer
// ---------------------------------------------------------------------------

/// `void SND_InitIopCmdBuffer(void)` (0x3c1638): lay out the command buffer
/// sent to the IOP sound driver in uncached RAM (`0x512b80 | 0x20000000`,
/// 0x2c00 bytes, cleared): a 672-byte parameter block (kind 3), the optional
/// FX-routing block (kind 2, the EE output buffer set through
/// `SND_EeSetOutputBuffer`), the PCM block (kind 1, one buffer of
/// `rate = outputrate * 512 / 48000` samples per channel, the second one
/// for mono), the S/PDIF block (kind 4, 0x800 bytes) and the closing kinds 5
/// and 6; the buffer's total size lands in the chain's header. Frame 0x80
/// (s0-s6).
#[allow(unused_labels)]
pub fn snd_init_iop_cmd_buffer(c: &mut Cpu) {
    guest(c, |c| {
        'b_3c16a4: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.g[V1][0] = 0x510000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = 0x510000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5300));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S4][0] = c.g[V1][0];
            c.g[S6][0] = c.g[V0][0];
            c.g[V1][0] = 0x20000000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2b80));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A2][0] = 0x2c00;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1ab4), c.g[V0][0]);
            call_ptr(c, 0x00416210, 0x003c1690)?;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0xbb80;
            c.g[S5][0] = c.g[V1][0];
            if c.g[A0][0] == 0u64 {
                break 'b_3c16a4;
            } else {
                break 'b_3c16a4;
            }
        }
        'b_3c176c: {
            c.cycles += 39;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2a));
            c.g[A2][0] = 0x2a0;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 9);
            c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), 0u64);
            c.div(0, 2, 4);
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
            c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = c.g[V0][0] & 0xffff;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xcb4), c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[A0][0] as u32), c.g[A1][0]);
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[V1][0] & 0xffff;
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
            c.g[V1][0] = c.g[V0][0] & 0xffff;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xcb0), c.g[A0][0]);
            c.sw((c.g[A1][0] as u32), c.g[A2][0]);
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2a0));
            c.g[V1][0] = c.g[V0][0] & 0xf;
            c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
            c.g[S2][0] = c.lo[0];
            let t = c.g[V1][0] == 0u64;
            c.g[S3][0] = c.g[S2][0];
            if t {
                break 'b_3c176c;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x1ab2));
                'l_3c1748: loop {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sh((c.g[A0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x1ab2));
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x1ab2));
                        if event_due(c) {
                            deliver_event_here(c, 0x003c1748);
                        }
                        continue 'l_3c1748;
                    } else {
                        break 'b_3c176c;
                    }
                }
            }
        }
        'b_3c17bc: {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5718));
            c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x42));
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
            if t {
                break 'b_3c17bc;
            } else {
                c.cycles += 16;
                c.g[A0][0] = 0x2;
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V1][0]);
                c.g[A0][0] = c.g[V1][0] & 0xffff;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, 0x003cbc00, 0x003c17ac)?;
                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
                break 'b_3c17bc;
            }
        }
        'b_3c1834: {
            c.cycles += 18;
            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
            c.g[A0][0] = 0x1;
            c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1ab2));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1ab4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
            c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1ab2));
            c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1ab4));
            c.sh((c.g[A3][0] as u32).wrapping_add(0x1ab2), c.g[V1][0]);
            c.g[V0][0] = c.g[V1][0] & 0xffff;
            c.g[A0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x47));
            c.g[T1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            let t = c.g[A0][0] == 0u64;
            c.sh((c.g[A3][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
            if t {
                break 'b_3c1834;
            } else {
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1abc));
                c.g[A1][0] = 0u64;
                c.g[T0][0] = sx((c.g[S3][0] as u32) << 1);
                'l_3c1810: loop {
                    c.cycles += 9;
                    c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1ab2));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1ab4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.sw((c.g[A2][0] as u32), c.g[V1][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003c1810);
                        }
                        continue 'l_3c1810;
                    } else {
                        break 'b_3c1834;
                    }
                }
            }
        }
        'b_3c1888: {
            c.cycles += 11;
            c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5718));
            c.g[A0][0] = 0x1;
            c.g[V1][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x47));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.mult(0, 2, 19, 2);
            c.sh((c.g[T1][0] as u32), c.g[V0][0]);
            c.g[V1][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x47));
            let t = c.g[V1][0] != c.g[A0][0];
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
            if t {
                break 'b_3c1888;
            } else {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
                c.g[A1][0] = sx((c.g[S2][0] as u32) << 1);
                c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x1ab2));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1ab4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1ac0), c.g[V1][0]);
                c.g[V0][0] = c.lhu((c.g[T1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.sh((c.g[T1][0] as u32), c.g[V0][0]);
                break 'b_3c1888;
            }
        }
        'b_3c18e0: {
            c.cycles += 7;
            c.g[V0][0] = c.lhu((c.g[T1][0] as u32));
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
            c.g[A0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x49));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.g[A0][0] & 0x2;
            let t = c.g[A0][0] == 0u64;
            c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V1][0]);
            if t {
                break 'b_3c18e0;
            } else {
                c.cycles += 15;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
                c.g[V1][0] = c.g[V1][0] & 0xffff;
                c.g[A0][0] = 0x4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V1][0] & 0xffff;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[V1][0] & 0xffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80c));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1af4), c.g[A1][0]);
                break 'b_3c18e0;
            }
        }
        c.cycles += 39;
        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
        c.g[A0][0] = 0x5;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
        c.g[A1][0] = 0x6;
        c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2b80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ab4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
        c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] & 0xffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[A0][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sh((c.g[S0][0] as u32).wrapping_add(0x1ab2), c.g[V0][0]);
        c.g[V1][0] = c.g[V0][0] & 0xffff;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b00), c.g[A2][0]);
        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1ab2));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b0c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b08), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA, SP as TEST_SP};

    /// Word in the machine's own RAM that turns the callee fakes below on
    /// (every fixture of this module writes it; other tests' machines do
    /// not, so the shared override slots fall back to the real callees).
    const OC692_CTRL: u32 = DATA + 0x20_0000;
    const OC692_MAGIC: u32 = 0x6f63_3932;

    /// A gated fake of the callee at `$addr`: with the gate set it logs
    /// every register the caller set up and returns (or unwinds, as the
    /// fixture's `machine` decides).
    macro_rules! oc692_gated_fake {
        ($fake:ident, $addr:expr, $generated:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC692_CTRL) != OC692_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc692_gated_fake!(oc692_fake_lock, 0x003b_58a0, ee::s13::f_003b58a0__generated);
    oc692_gated_fake!(
        oc692_fake_unlock,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );
    oc692_gated_fake!(
        oc692_fake_mem_alloc,
        0x003b_5c60,
        ee::s13::f_003b5c60__generated
    );
    oc692_gated_fake!(
        oc692_fake_fire_events,
        0x003b_5f98,
        ee::s13::f_003b5f98__generated
    );
    oc692_gated_fake!(
        oc692_fake_calc_volume,
        0x003b_a7b0,
        ee::s13::f_003ba7b0__generated
    );
    oc692_gated_fake!(
        oc692_fake_apply_volume,
        0x003c_2a50,
        ee::s13::f_003c2a50__generated
    );
    oc692_gated_fake!(
        oc692_fake_apply_pan,
        0x003c_38e0,
        ee::s13::f_003c38e0__generated
    );
    oc692_gated_fake!(
        oc692_fake_memset,
        0x0041_6210,
        ee::s15::f_00416210__generated
    );

    /// `SND_ParsePatch` with its gate set: stores a flags halfword into the
    /// parsed block (`a1 + 0x18`, from a table in the gate area), returns
    /// "more tags" while the countdown at `CTRL + 4` stays above one.
    fn oc692_fake_parse_patch(c: &mut Cpu) {
        if c.bus.read32(OC692_CTRL) != OC692_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc692_fake_parse_patch),
                ee::s13::f_003baac0__generated,
            );
        }
        let left = c.bus.read32(OC692_CTRL + 4);
        let flags = c.bus.read16(OC692_CTRL + 0x10 + 2 * (left & 7));
        let patch = c.g[A1][0] as u32;
        c.bus.write16(patch.wrapping_add(BRP_PATCH_FLAGS), flags);
        c.bus.write32(OC692_CTRL + 4, left.saturating_sub(1));
        fakes::fake_call(c, 0x003b_aac0);
        if c.pc & 0xffff_0000 != 0xdead_0000 {
            c.g[V0][0] = (left > 1) as u64;
        }
    }

    fn oc692_hook_lock_unlock() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc692_fake_lock,
            fakes::prev_cell(oc692_fake_lock),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc692_fake_unlock,
            fakes::prev_cell(oc692_fake_unlock),
        );
    }

    crate::difftest::events::driven!(oc692_gen_brp, ee::s13::f_003b6788__generated);
    crate::difftest::events::driven!(oc692_hand_brp, snd_bank_relocate_patches);
    crate::difftest::events::driven!(oc692_gen_smv, ee::s13::f_003b8160__generated);
    crate::difftest::events::driven!(oc692_hand_smv, snd_set_master_volume);
    crate::difftest::events::driven!(oc692_gen_ipt, ee::s13::f_003bc0b0__generated);
    crate::difftest::events::driven!(oc692_hand_ipt, snd_init_pan_tables);
    crate::difftest::events::driven!(oc692_gen_seed, ee::s13::f_003bc588__generated);
    crate::difftest::events::driven!(oc692_hand_seed, snd_random_seed);
    crate::difftest::events::driven!(oc692_gen_sinf, ee::s13::f_003bc770);
    crate::difftest::events::driven!(oc692_gen_cosf, ee::s13::f_003bc840);
    crate::difftest::events::driven!(oc692_gen_acfg, ee::s13::f_003c0e28__generated);
    crate::difftest::events::driven!(oc692_hand_acfg, snd_apply_config);
    crate::difftest::events::driven!(oc692_gen_iic, ee::s13::f_003c1638__generated);
    crate::difftest::events::driven!(oc692_hand_iic, snd_init_iop_cmd_buffer);

    // ---------------------------------------------------- SND_BankRelocatePatches

    /// SND_BankRelocatePatches fixture: one to four tags (the fake
    /// `SND_ParsePatch` counts them down and stores random render flags),
    /// a render mask that is sometimes 0, a first-offset word that is
    /// sometimes 0 (the relocate callback's two paths), the parsed block at
    /// the bottom of the frame holding a valid destination pointer.
    fn brp_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x6788, calls);
        c.bus.write32(OC692_CTRL, OC692_MAGIC);
        c.bus.write32(OC692_CTRL + 4, 1 + r.next_u32() % 4);
        for i in 0..8u32 {
            let flags = r.next_u32() as u16;
            c.bus.write16(OC692_CTRL + 0x10 + 2 * i, flags);
        }
        c.pc = 0x003b_6788;
        let block = TEST_SP - 0x140;
        fill(c, &mut r, block, 0x34);
        c.bus.write32(block + 0x64, DATA + 0x3000);
        let first_offset = if seed % 2 == 0 { 0 } else { r.next_u32() };
        c.bus.write32(DATA + 0x2100, first_offset);
        c.g[A0][0] = if seed % 5 == 0 {
            0
        } else {
            (r.next_u32() & 0xffff) as u64
        };
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = (DATA + 0x2000) as u64;
        c.g[A3][0] = (DATA + 0x2100) as u64;
        r
    }

    /// SND_BankRelocatePatches: patches whose flags overlap the mask or
    /// not, relocation with and without a first offset, the loop over one
    /// to four tags; fake callees that unwind at chosen calls; and real
    /// events at the loop's back edge (delivered, or asking for an unwind).
    #[test]
    fn snd_bank_relocate_patches_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003BAAC0),
            oc692_fake_parse_patch,
            fakes::prev_cell(oc692_fake_parse_patch),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B5F98),
            oc692_fake_fire_events,
            fakes::prev_cell(oc692_fake_fire_events),
        );
        covered(
            ee::s13::f_003b6788__generated,
            snd_bank_relocate_patches,
            200,
            &|c, seed| {
                brp_setup(c, seed, 6);
            },
        );
        check_events(oc692_gen_brp, oc692_hand_brp, 300, &|c, seed| {
            let mut r = brp_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x003b_6788)
        });
    }

    // ---------------------------------------------------- SND_SetMasterVolume

    /// SND_SetMasterVolume fixture: zero, negative or one to six channels;
    /// channels in use or not, with negative or positive handles.
    fn smv_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x8160, calls);
        c.bus.write32(OC692_CTRL, OC692_MAGIC);
        c.pc = 0x003b_8160;
        let channels = DATA + 0x4_0000;
        let count = match seed % 8 {
            0 => 0,
            1 => 0xffff,
            k => k - 1,
        };
        c.bus.write16(SMV_STATE + SMV_CHANNEL_COUNT, count as u16);
        c.bus.write32(SMV_STATE + SMV_CHANNELS, channels);
        for i in 0..8u32 {
            let ch = channels + i * SMV_CHANNEL_SIZE;
            fill(c, &mut r, ch, SMV_CHANNEL_SIZE / 4);
            let in_use = if r.next_u32() % 4 != 0 { 1 } else { 0 };
            c.bus.write8(ch + 0x69, in_use);
            let handle = if r.next_u32() % 3 != 0 {
                r.next_u32() & 0x7fff_ffff
            } else {
                r.next_u32() | 0x8000_0000
            };
            c.bus.write32(ch, handle);
        }
        c.g[A0][0] = r.next_u32() as u64;
        r
    }

    /// SND_SetMasterVolume: no channels, channels skipped (not in use, or a
    /// negative handle) and updated (volume calc + apply); fakes that
    /// unwind at chosen calls; events at the loop's back edge.
    #[test]
    fn snd_set_master_volume_matches() {
        oc692_hook_lock_unlock();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003BA7B0),
            oc692_fake_calc_volume,
            fakes::prev_cell(oc692_fake_calc_volume),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003C2A50),
            oc692_fake_apply_volume,
            fakes::prev_cell(oc692_fake_apply_volume),
        );
        covered(
            ee::s13::f_003b8160__generated,
            snd_set_master_volume,
            240,
            &|c, seed| {
                smv_setup(c, seed, 8);
            },
        );
        check_events(oc692_gen_smv, oc692_hand_smv, 300, &|c, seed| {
            let mut r = smv_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x003b_8160)
        });
    }

    // ---------------------------------------------------- SND_CalcPanGains

    /// SND_CalcPanGains: positions inside, before and after the gap between
    /// two speaker angles (also negative, wrapped angles), and the 0/0 case.
    /// The real `SND_Sinf` / `SND_Cosf` are the callees (their angle
    /// reduction runs for the far positions).

    // ---------------------------------------------------- SND_InitPanTables

    const PAN_STATE: u32 = 0x0050_a8e8;

    /// SND_InitPanTables fixture: zero to six output channels, the speaker
    /// angle table (increasing angles, 0x1800 apart so no pan gap is
    /// empty), a stale pointer table in the data area.
    fn ipt_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0xc0b0, calls);
        c.bus.write32(OC692_CTRL, OC692_MAGIC);
        c.pc = 0x003b_c0b0;
        c.bus.write8(PAN_STATE + 0x47, (seed % 7) as u8);
        for row in 0..8u32 {
            for j in 0..6u32 {
                let angle = 0x1800 * (j + 1) + r.next_u32() % 0x400;
                c.bus.write16(0x0050_a954 + 12 * row + 2 * j, angle as u16);
            }
        }
        for j in 0..6u32 {
            c.bus
                .write32(0x0050_ac38 + 4 * j, DATA + 0x8000 + 0x100 * j);
        }
        r
    }

    /// SND_InitPanTables: every output channel count 0..=6 (mono's constant
    /// table, the two-channel wrap cases, five or more channels), the real
    /// `SND_CalcPanGains` as callee; lock, allocation and unlock are fakes
    /// that unwind at chosen calls; events at the three loops' back edges.
    #[test]
    fn snd_init_pan_tables_matches() {
        oc692_hook_lock_unlock();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B5C60),
            oc692_fake_mem_alloc,
            fakes::prev_cell(oc692_fake_mem_alloc),
        );
        covered(
            ee::s13::f_003bc0b0__generated,
            snd_init_pan_tables,
            70,
            &|c, seed| {
                ipt_setup(c, seed, 3);
            },
        );
        check_events(oc692_gen_ipt, oc692_hand_ipt, 70, &|c, seed| {
            let mut r = ipt_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x003b_c0b0)
        });
    }

    // ---------------------------------------------------- SND_RandomSeed

    fn seed_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x3bc5_8800 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = 0x003b_c588;
        fill(c, &mut r, RANDOM_SEED_TABLE, 6);
        fill(c, &mut r, RANDOM_STATE, 6);
        r
    }

    /// SND_RandomSeed: random seeds and tables (the sums wrap); events at
    /// the loop's back edge.
    #[test]
    fn snd_random_seed_matches() {
        check(
            ee::s13::f_003bc588__generated,
            snd_random_seed,
            100,
            &|c, seed| {
                seed_setup(c, seed);
            },
        );
        check_events(oc692_gen_seed, oc692_hand_seed, 150, &|c, seed| {
            let mut r = seed_setup(c, seed);
            schedule(c, &mut r, seed, 0x003b_c588)
        });
    }

    // ---------------------------------------------------- SND_Sinf / SND_Cosf

    /// An angle for the trigonometry tests: within one turn, a few turns
    /// (the reduction loop), negative, zero or NaN.
    fn trig_angle(r: &mut Rng, seed: u32) -> u32 {
        match seed % 6 {
            0 => r.f32(3.5),
            1 => r.f32(10.0),
            2 => r.f32(200.0),
            3 => 0x7fc0_0000,
            4 => 0,
            _ => r.f32(7.0),
        }
    }

    fn trig_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(0x7172_0000 ^ (entry as u64) << 8 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = entry;
        c.f[12] = trig_angle(&mut r, seed);
        r
    }

    /// SND_Sinf: the angle reduction (none, one or many subtractions),
    /// negative angles, NaN; events in the reduction loop.

    /// SND_Cosf: as `snd_sinf_matches`.

    // ---------------------------------------------------- SND_SetDefaultConfig

    /// SND_SetDefaultConfig: straight-line stores of the capabilities and
    /// defaults; the one load is the IOP module path pointer.
    #[test]
    fn snd_set_default_config_matches() {
        check(
            ee::s13::f_003c0c90__generated,
            snd_set_default_config,
            40,
            &|c, seed| {
                let mut r = Rng(0x3c0c_9000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_0c90;
                c.bus.write32(0x0044_dd3c, r.next_u32());
            },
        );
    }

    // ---------------------------------------------------- SND_ApplyConfig

    /// SND_ApplyConfig fixture: the driver state block full of random
    /// words (requested settings at +0x20, the previous ones at +0xd4, the
    /// capabilities at +0x4..), installed or not, one to four channels, a
    /// previous S/PDIF mode equal to the requested one in three seeds of
    /// four, valid channel and mixer pointers.
    fn acfg_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x0e28, calls);
        c.bus.write32(OC692_CTRL, OC692_MAGIC);
        c.pc = 0x003c_0e28;
        fill(c, &mut r, PAN_STATE, 0x80);
        let installed = (seed % 2) as u8;
        c.bus.write8(PAN_STATE + 0x174, installed);
        c.bus.write16(PAN_STATE + 0x17e, 1 + (seed / 2 % 4) as u16);
        let channels = DATA + 0x1_0000;
        c.bus.write32(PAN_STATE + 0x1f0, channels);
        for i in 0..4u32 {
            let ch = channels + i * 0x8c;
            fill(c, &mut r, ch, 0x8c / 4);
            let in_use = (r.next_u32() % 4 != 0) as u8;
            c.bus.write8(ch + 0x69, in_use);
        }
        if seed / 2 % 4 != 3 {
            let requested = c.bus.read8(PAN_STATE + 0x49);
            c.bus.write8(PAN_STATE + 0xfd, requested);
        }
        c.bus.write32(0x0050_b9b0, DATA + 0x2_0000);
        r
    }

    /// SND_ApplyConfig: installed (the previous settings restored, early
    /// exit when the S/PDIF mode changed, per-channel pan updates when the
    /// channel count changed) and not installed (the settings clamped to the
    /// capabilities and stored); fake pan callee that unwinds; events at
    /// the copy and channel loops' back edges. (The request is copied by
    /// aligned 8-byte moves at the fixed addresses of the original except
    /// for the prior-settings copy: both kinds are reached.)
    #[test]
    fn snd_apply_config_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003C38E0),
            oc692_fake_apply_pan,
            fakes::prev_cell(oc692_fake_apply_pan),
        );
        covered(
            ee::s13::f_003c0e28__generated,
            snd_apply_config,
            300,
            &|c, seed| {
                acfg_setup(c, seed, 2);
            },
        );
        check_events(oc692_gen_acfg, oc692_hand_acfg, 300, &|c, seed| {
            let mut r = acfg_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x003c_0e28)
        });
    }

    // ---------------------------------------------------- SND_InitIopCmdBuffer

    /// SND_InitIopCmdBuffer fixture: output rate (24, 36, 48 kHz or odd),
    /// FX routing on or off, one to six channels, LFE, S/PDIF mode.
    fn iic_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x1638, calls);
        c.bus.write32(OC692_CTRL, OC692_MAGIC);
        c.pc = 0x003c_1638;
        let rate = [0xbb80, 0x8ca0, 0x5dc0, 0x4000, r.next_u32() & 0xffff][(seed % 5) as usize];
        c.bus.write16(PAN_STATE + 0x2a, rate as u16);
        c.bus.write8(PAN_STATE + 0x42, (seed / 5 % 2) as u8);
        let channels = [1u8, 2, 4, 5, 6][(seed / 10 % 5) as usize];
        c.bus.write8(PAN_STATE + 0x47, channels);
        c.bus.write8(PAN_STATE + 0x48, (seed / 3 % 2) as u8);
        c.bus
            .write8(PAN_STATE + 0x49, [0u8, 2, 1, 3][(seed / 7 % 4) as usize]);
        r
    }

    /// SND_InitIopCmdBuffer: every rate class, with and without FX
    /// routing (the real `SND_EeSetOutputBuffer` callee), mono and
    /// multichannel (and LFE), the S/PDIF block; memset is a fake that
    /// unwinds at chosen calls; events at the alignment and channel loops.
    #[test]
    fn snd_init_iop_cmd_buffer_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00416210),
            oc692_fake_memset,
            fakes::prev_cell(oc692_fake_memset),
        );
        covered(
            ee::s13::f_003c1638__generated,
            snd_init_iop_cmd_buffer,
            150,
            &|c, seed| {
                iic_setup(c, seed, 1);
            },
        );
        check_events(oc692_gen_iic, oc692_hand_iic, 150, &|c, seed| {
            let mut r = iic_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x003c_1638)
        });
    }
}
