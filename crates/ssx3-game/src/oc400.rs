//! Staging module oc400 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc400`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::dirtysock::tagsunk::GAME_COMM_GP;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_fd50, lobby_man_send_race_report, "LobbyMan_SendRaceReport"),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const TAG_FIELD_SET_NUMBER: Callee = Callee {
        addr: 0x003e_be40,
        f: ee::s14::f_003ebe40,
    };

    pub const TAG_FIELD_SET_DATE: Callee = Callee {
        addr: 0x003e_c7e0,
        f: ee::s14::f_003ec7e0,
    };

    // `TagFieldSetString` (the unit's `callee::SNPRINTF` in tagsunk.rs names
    // this same address; the correct name is used here).
    pub const TAG_FIELD_SET_STRING: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::f_003ec180,
    };

    pub const FIXUNSSFSI: Callee = Callee {
        addr: 0x0041_39f8,
        f: ee::s15::f_004139f8,
    };

    pub const BE_NEW_RACE_INT_GET_TRACK: Callee = Callee {
        addr: 0x0014_4be0,
        f: ee::s01::f_00144be0,
    };

    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };

    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
}

/// Tag-name string addresses (ELF `.rodata`) written into the race report.
pub(crate) mod tag {
    pub const WHEN: u32 = 0x004a_2f80;
    pub const REPT: u32 = 0x004a_3278;
    pub const AUTH: u32 = 0x004a_2f78;
    pub const COMMERR: u32 = 0x004a_3280;
    pub const SCOMERR: u32 = 0x004a_3288;
    pub const FINISH0: u32 = 0x004a_3290;
    pub const FINISH1: u32 = 0x004a_3298;
    pub const DNF0: u32 = 0x004a_32a0;
    pub const DNF1: u32 = 0x004a_32a8;
    pub const DESYNC: u32 = 0x004a_32b0;
    pub const NAME0: u32 = 0x004a_32b8;
    pub const QUIT0: u32 = 0x004a_32c0;
    pub const CHEAT0: u32 = 0x004a_32c8;
    pub const DISC0: u32 = 0x004a_32d0;
    pub const SCORE0: u32 = 0x004a_32d8;
    pub const POINTS0: u32 = 0x004a_32e0;
    pub const SCORE1: u32 = 0x004a_32e8;
    pub const NAME1: u32 = 0x004a_32f0;
    pub const QUIT1: u32 = 0x004a_32f8;
    pub const CHEAT1: u32 = 0x004a_3300;
    pub const DISC1: u32 = 0x004a_3308;
    pub const POINTS1: u32 = 0x004a_3310;
    pub const PINGRSLT: u32 = 0x0048_0920;
    pub const STALLCNT: u32 = 0x0048_0930;
    pub const CERRSTAT: u32 = 0x0048_0940;
    pub const CERRFLGS: u32 = 0x0048_0950;
    pub const DIDQUIT0: u32 = 0x0048_0960;
    pub const DIDQUIT1: u32 = 0x0048_0970;
    pub const RSLTSLCK: u32 = 0x0048_0980;
    pub const RACETRA0: u32 = 0x0048_0990;
    pub const RACEEVE0: u32 = 0x0048_09a0;
    pub const CHARACTER0: u32 = 0x0048_09b0;
    pub const RACETIM0: u32 = 0x0048_09c0;
    /// First per-rider stat tag for rider 0 (`SPINDEG0`); the rest follow at
    /// +0x10 steps through `PLOSTRE0`, except the `POINTS0` tag.
    pub const STATS0_BASE: u32 = 0x0048_09d0;
    pub const CHARACTER1: u32 = 0x0048_0be0;
    pub const RACETIM1: u32 = 0x0048_0bf0;
    /// First per-rider stat tag for rider 1 (`SPINDEG1`).
    pub const STATS1_BASE: u32 = 0x0048_0c00;
}

/// Size of the report record built on the stack.
pub(crate) const REPORT_BUF_SIZE: u32 = 0x800;
/// `EA_MemAllocNamed` name string for the queued lobby message.
pub(crate) const NET_EVENT_TAG: u32 = 0x0048_0488;
/// Vtable stored in the queued lobby message.
pub(crate) const NET_EVENT_VTABLE: u32 = 0x0048_12b0;
/// The `'rank'` request kind passed to `LobbyMan_SendRequest`.
pub(crate) const REQ_RANK: u32 = 0x7261_6e6b;

/// The race-state object reached through `gp + GAME_COMM_GP`.
fn race_state(c: &Cpu) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(GAME_COMM_GP as u32)
}

/// The race-state pointer itself (the word at the `gp` slot).
fn race_ptr(c: &mut Cpu) -> u32 {
    c.lw(race_state(c)) as u32
}

/// `TagFieldSetNumber(sp, 0x800, tag, a3)`: the caller sets `a3` itself
/// (a stats load, a stack spill, or `v0` after `__fixunssfsi`).
fn set_number(c: &mut Cpu, tag_addr: u32, ret_addr: u32) -> crate::call::Flow {
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = REPORT_BUF_SIZE as u64;
    c.g[A2][0] = sx(tag_addr);
    callee::TAG_FIELD_SET_NUMBER.call(c, ret_addr)
}

/// Return addresses of one rider's stat-block calls, in call order.
struct RiderRets {
    ints: [u32; 14],
    float_fix: [u32; 10],
    float_num: [u32; 10],
    tail: [u32; 10],
}

const RIDER_STATS_INT_OFFS: [u32; 14] = [
    0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
];
const RIDER_STATS_FLOAT_OFFS: [u32; 10] = [
    0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x60, 0x64, 0x68, 0x6c,
];
const RIDER_STATS_TAIL_OFFS: [u32; 10] = [
    0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x90, 0x9c, 0xa4, 0xa8,
];

/// One rider's stat block: 14 int fields, 10 float fields converted with
/// `__fixunssfsi`, then 10 more int fields (the `+0x9c` field takes
/// `points_tag` instead of the sequential tag). The two generated blocks
/// have this exact shape apart from the stats register, the tag base and
/// the return addresses, which are all parameters.
fn report_rider_stats(
    c: &mut Cpu,
    stats_reg: usize,
    base_tag: u32,
    points_tag: u32,
    rets: &RiderRets,
) -> crate::call::Flow {
    for (i, &off) in RIDER_STATS_INT_OFFS.iter().enumerate() {
        c.g[A3][0] = c.lw((c.g[stats_reg][0] as u32).wrapping_add(off));
        set_number(c, base_tag.wrapping_add(0x10 * i as u32), rets.ints[i])?;
    }
    for (i, &off) in RIDER_STATS_FLOAT_OFFS.iter().enumerate() {
        let tag_addr = base_tag.wrapping_add(0x10 * (14 + i) as u32);
        c.lwc1(12, (c.g[stats_reg][0] as u32).wrapping_add(off));
        c.g[S0][0] = sx(tag_addr);
        callee::FIXUNSSFSI.call(c, rets.float_fix[i])?;
        c.g[A3][0] = c.g[V0][0];
        set_number(c, tag_addr, rets.float_num[i])?;
    }
    for (i, &off) in RIDER_STATS_TAIL_OFFS.iter().enumerate() {
        let tag_addr = if i == 7 {
            points_tag
        } else if i < 7 {
            base_tag.wrapping_add(0x10 * (24 + i) as u32)
        } else {
            base_tag.wrapping_add(0x10 * (23 + i) as u32)
        };
        c.g[A3][0] = c.lw((c.g[stats_reg][0] as u32).wrapping_add(off));
        set_number(c, tag_addr, rets.tail[i])?;
    }
    Ok(())
}

const RIDER0_RETS: RiderRets = RiderRets {
    ints: [
        0x0026_0378, 0x0026_0390, 0x0026_03a8, 0x0026_03c0, 0x0026_03d8, 0x0026_03f0, 0x0026_0408,
        0x0026_0420, 0x0026_0438, 0x0026_0450, 0x0026_0468, 0x0026_0480, 0x0026_0498, 0x0026_04b0,
    ],
    float_fix: [
        0x0026_04c0, 0x0026_04e4, 0x0026_0508, 0x0026_052c, 0x0026_0550, 0x0026_0574, 0x0026_0598,
        0x0026_05bc, 0x0026_05e0, 0x0026_0604,
    ],
    float_num: [
        0x0026_04d4, 0x0026_04f8, 0x0026_051c, 0x0026_0540, 0x0026_0564, 0x0026_0588, 0x0026_05ac,
        0x0026_05d0, 0x0026_05f4, 0x0026_0618,
    ],
    tail: [
        0x0026_0630, 0x0026_0648, 0x0026_0660, 0x0026_0678, 0x0026_0690, 0x0026_06a8, 0x0026_06c0,
        0x0026_06d8, 0x0026_06f0, 0x0026_0708,
    ],
};

/// `void LobbyMan_SendRaceReport(LobbyMan* report, int auth, int finish0, int finish1, RiderStats* stats0, RiderStats* stats1)` (0x0025fd50): build the post-race "rank" report record (WHEN/REPT/AUTH, race flags, both riders' stat tags) in a 0x800-byte stack buffer, send it with `LobbyMan_SendRequest`, and queue the NetEvent message on the lobby list. Returns at once when `*report == 0`.
pub fn lobby_man_send_race_report(c: &mut Cpu) {
    guest(c, |c| {
        {
            c.cycles += 19;
            let frame = Frame::open(c, 0x900);
            frame.save(c, S0, 0x8f0);
            frame.save(c, S1, 0x8e0);
            c.g[S0][0] = c.g[A2][0];
            frame.save(c, S3, 0x8c0);
            c.g[S1][0] = c.g[A3][0];
            frame.save(c, S5, 0x8a0);
            c.g[S3][0] = c.g[T1][0];
            frame.save(c, S2, 0x8d0);
            frame.save(c, S4, 0x8b0);
            frame.save(c, S6, 0x890);
            frame.save(c, S7, 0x880);
            frame.save(c, FP, 0x870);
            frame.save_ra(c, 0x860);
            c.sw(frame.at(c, 0x820), c.g[A0][0]);
            c.sw(frame.at(c, 0x824), c.g[A1][0]);
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            let no_report = c.g[V0][0] == 0;
            c.g[S5][0] = c.g[T0][0];
            if !no_report {
                {
                    c.cycles += 18;
                    callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0025_fda4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0025_fdb0)?;
                    c.sw(frame.at(c, 0x828), c.g[V0][0]);
                    c.g[A1][0] = 0x7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0025_fdc4)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[T4][0] = 0;
                    c.g[A0][0] = c.lw(race_state(c));
                    c.g[T3][0] = 0;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let no_flags = c.g[V0][0] == 0;
                    c.sw(frame.at(c, 0x82c), c.g[V1][0]);
                    if !no_flags {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                        if c.g[V0][0] != 0 {
                            c.g[T3][0] = 0x1;
                        } else {
                            c.cycles += 1;
                            c.g[T4][0] = 0x1;
                        }
                    }
                }
                'b_260004: {
                    'b_25ff18: {
                        c.cycles += 13;
                        c.g[V0][0] = c.lw(race_state(c));
                        c.g[V1][0] = 0x530000;
                        c.g[A0][0] = 0x1;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));
                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                        c.g[T0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                        c.g[T2][0] = c.g[A0][0];
                        c.g[T1][0] = c.g[A0][0];
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        if c.g[A1][0] != 0 {
                            c.g[T2][0] = 0;
                        }
                        let table_order = c.g[V0][0] == 0;
                        if c.g[A2][0] != 0 {
                            c.g[T1][0] = 0;
                        }
                        if !table_order {
                            // Player order from the 0x534b30 table: rider 0
                            // is finish0/stats0 (report+0x90 name).
                            c.cycles += 4;
                            c.sw(frame.at(c, 0x830), c.g[S0][0]);
                            c.g[V0][0] = c.lw(frame.at(c, 0x820));
                            c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                            if c.g[T1][0] == 0 {
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x834), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] = sx((c.g[V1][0] as u32)
                                        .wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 2;
                                c.sw(frame.at(c, 0x834), c.g[A3][0]);
                            }
                            c.cycles += 2;
                            if c.g[T4][0] == 0 {
                                c.sw(frame.at(c, 0x838), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] = sx((c.g[V1][0] as u32)
                                        .wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x838), c.g[A3][0]);
                            }
                            c.cycles += 15;
                            c.g[V0][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                            c.g[FP][0] = c.g[V0][0];
                            c.g[S7][0] = c.g[V0][0];
                            c.g[V0][0] = 0x530000;
                            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa0));
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b30));
                            if c.g[V1][0] == 0 {
                                c.g[S7][0] = 0;
                            }
                            c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x32c));
                            if c.g[A0][0] == 0 {
                                c.g[FP][0] = 0;
                            }
                            c.g[S4][0] = c.g[S5][0];
                            c.sw(frame.at(c, 0x840), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x24));
                            c.sw(frame.at(c, 0x83c), c.g[V1][0]);
                            if c.g[T2][0] == 0 {
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x844), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] = sx((c.g[V1][0] as u32)
                                        .wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 2;
                                c.sw(frame.at(c, 0x844), c.g[A3][0]);
                            }
                            c.cycles += 2;
                            if c.g[T3][0] == 0 {
                                c.sw(frame.at(c, 0x848), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] = sx((c.g[V1][0] as u32)
                                        .wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x848), c.g[A3][0]);
                            }
                            c.cycles += 9;
                            c.g[V0][0] = 0x1;
                            c.g[V1][0] = c.lw(race_state(c));
                            c.g[S6][0] = c.g[V0][0];
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                            c.g[S5][0] = c.g[S3][0];
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                            break 'b_260004;
                        }
                    }
                    // Player order swapped: rider 0 is finish1/stats1
                    // (report+0x94 name).
                    c.cycles += 4;
                    c.sw(frame.at(c, 0x830), c.g[S1][0]);
                    c.g[A0][0] = c.lw(frame.at(c, 0x820));
                    c.g[S2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
                    if c.g[T2][0] == 0 {
                        c.cycles += 1;
                        c.sw(frame.at(c, 0x834), 0);
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                        let zero = c.g[V1][0] == 0;
                        c.g[A3][0] = 0;
                        if !zero {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                            c.g[A3][0] =
                                sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        }
                        c.cycles += 2;
                        c.sw(frame.at(c, 0x834), c.g[A3][0]);
                    }
                    c.cycles += 2;
                    if c.g[T3][0] == 0 {
                        c.sw(frame.at(c, 0x838), 0);
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                        let zero = c.g[V1][0] == 0;
                        c.g[A3][0] = 0;
                        if !zero {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                            c.g[A3][0] =
                                sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        }
                        c.cycles += 1;
                        c.sw(frame.at(c, 0x838), c.g[A3][0]);
                    }
                    c.cycles += 16;
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
                    c.g[FP][0] = c.g[V0][0];
                    c.g[S7][0] = c.g[V0][0];
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa4));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b30));
                    if c.g[V1][0] == 0 {
                        c.g[S7][0] = 0;
                    }
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x328));
                    if c.g[A0][0] == 0 {
                        c.g[FP][0] = 0;
                    }
                    c.g[S4][0] = c.g[S3][0];
                    c.g[S1][0] = c.g[S0][0];
                    c.sw(frame.at(c, 0x840), c.g[A1][0]);
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x28));
                    c.sw(frame.at(c, 0x83c), c.g[V1][0]);
                    if c.g[T1][0] == 0 {
                        c.cycles += 1;
                        c.sw(frame.at(c, 0x844), 0);
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                        let zero = c.g[V1][0] == 0;
                        c.g[A3][0] = 0;
                        if !zero {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                            c.g[A3][0] =
                                sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        }
                        c.cycles += 2;
                        c.sw(frame.at(c, 0x844), c.g[A3][0]);
                    }
                    c.cycles += 2;
                    if c.g[T4][0] == 0 {
                        c.sw(frame.at(c, 0x848), 0);
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                        let zero = c.g[V1][0] == 0;
                        c.g[A3][0] = 0;
                        if !zero {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                            c.g[A3][0] =
                                sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        }
                        c.cycles += 1;
                        c.sw(frame.at(c, 0x848), c.g[A3][0]);
                    }
                    c.cycles += 7;
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = c.lw(race_state(c));
                    c.g[S6][0] = c.g[V0][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                }
                {
                    c.cycles += 20;
                    if c.g[A0][0] == 0 {
                        c.g[S6][0] = 0;
                    }
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = 0;
                    }
                    c.sw(frame.at(c, 0x84c), c.g[V1][0]);
                    c.g[A0][0] = c.lw(frame.at(c, 0x820));
                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = REPORT_BUF_SIZE as u64;
                    c.g[A2][0] = sx(tag::WHEN);
                    c.sb(sp(c), 0);
                    callee::TAG_FIELD_SET_DATE.call(c, 0x0026_0030)?;
                    c.g[A3][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = REPORT_BUF_SIZE as u64;
                    c.g[A2][0] = sx(tag::REPT);
                    callee::TAG_FIELD_SET_STRING.call(c, 0x0026_0048)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x824));
                    let has_auth = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lw(frame.at(c, 0x820));
                    if has_auth {
                        c.cycles += 6;
                        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = REPORT_BUF_SIZE as u64;
                        c.g[A2][0] = sx(tag::AUTH);
                        callee::TAG_FIELD_SET_STRING.call(c, 0x0026_006c)?;
                    }
                }
                // One 115-cycle charge covers the next 16 calls.
                c.cycles += 115;
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0x30)); }
                c.g[V0][0] = c.lw(frame.at(c, 0x830));
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[S3][0] = 0x1;
                c.g[S2][0] = sx((c.g[V0][0] as u32) << 2);
                set_number(c, tag::COMMERR, 0x0026_0098)?;
                c.sw(frame.at(c, 0x850), c.g[S2][0]);
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0x18)); }
                set_number(c, tag::SCOMERR, 0x0026_00b8)?;
                c.sw(frame.at(c, 0x854), c.g[S1][0]);
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0x9c)); }
                set_number(c, tag::PINGRSLT, 0x0026_00d8)?;
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0xb4)); }
                set_number(c, tag::STALLCNT, 0x0026_00f4)?;
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0x1c)); }
                set_number(c, tag::CERRSTAT, 0x0026_0110)?;
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0x20)); }
                set_number(c, tag::CERRFLGS, 0x0026_012c)?;
                c.g[A3][0] = c.g[S7][0];
                set_number(c, tag::DIDQUIT0, 0x0026_0144)?;
                c.g[A3][0] = c.g[S0][0];
                set_number(c, tag::DIDQUIT1, 0x0026_015c)?;
                c.g[A3][0] = c.g[FP][0];
                set_number(c, tag::FINISH0, 0x0026_0174)?;
                c.g[A3][0] = c.g[S6][0];
                set_number(c, tag::FINISH1, 0x0026_018c)?;
                c.g[S0][0] = 0x530000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x66a8));
                c.g[A3][0] = 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = c.g[S3][0];
                }
                set_number(c, tag::DNF0, 0x0026_01b8)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A3][0] = 0;
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = c.g[S3][0];
                }
                set_number(c, tag::DNF1, 0x0026_01dc)?;
                c.g[A0][0] = c.lw(frame.at(c, 0x82c));
                c.g[A3][0] = 0;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                if c.g[V1][0] != 0 {
                    c.g[A3][0] = c.g[S3][0];
                }
                set_number(c, tag::RSLTSLCK, 0x0026_0200)?;
                c.g[A0][0] = c.lw(frame.at(c, 0x828));
                callee::BE_NEW_RACE_INT_GET_TRACK.call(c, 0x0026_0208)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S0][0];
                set_number(c, tag::RACETRA0, 0x0026_0224)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe));
                c.g[V0][0] = 0x530000;
                c.g[S0][0] = (c.g[S0][0] < 0x3) as u64;
                c.g[S1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
                if c.g[S0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let not_four = c.g[S1][0] != c.g[V0][0];
                    c.g[S1][0] = 0x1;
                    if !not_four {
                        c.cycles += 1;
                        c.g[S1][0] = 0;
                    }
                }
                // Rider 0 header.
                c.cycles += 48;
                c.g[A3][0] = c.g[S1][0];
                set_number(c, tag::RACEEVE0, 0x0026_0260)?;
                { let rp = race_ptr(c); c.g[A3][0] = c.lw(rp.wrapping_add(0xc)); }
                set_number(c, tag::DESYNC, 0x0026_027c)?;
                c.g[V0][0] = c.lw(frame.at(c, 0x820));
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = REPORT_BUF_SIZE as u64;
                c.g[A2][0] = sx(tag::NAME0);
                callee::TAG_FIELD_SET_STRING.call(c, 0x0026_0298)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x834));
                set_number(c, tag::QUIT0, 0x0026_02b0)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x83c));
                set_number(c, tag::CHEAT0, 0x0026_02c8)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x838));
                set_number(c, tag::DISC0, 0x0026_02e0)?;
                c.g[A0][0] = 0x530000;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b30));
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x328));
                set_number(c, tag::CHARACTER0, 0x0026_0300)?;
                c.g[V1][0] = 0x530000;
                if c.g[S1][0] != 0 {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(frame.at(c, 0x850));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[S2][0] = c.g[V1][0];
                    set_number(c, tag::SCORE0, 0x0026_0360)?;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw(frame.at(c, 0x850));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[S2][0] = c.g[V1][0];
                    set_number(c, tag::RACETIM0, 0x0026_0330)?;
                }
                // Rider 0 stats, then rider 1's character.
                c.cycles += 241;
                report_rider_stats(c, S4, tag::STATS0_BASE, tag::POINTS0, &RIDER0_RETS)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x840));
                set_number(c, tag::CHARACTER1, 0x0026_0720)?;
                c.g[A0][0] = c.lw(frame.at(c, 0x854));
                if c.g[S1][0] == 0 {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw(frame.at(c, 0x854));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    set_number(c, tag::RACETIM1, 0x0026_074c)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x820));
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    set_number(c, tag::SCORE1, 0x0026_0774)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x820));
                }
                // Rider 1 header and stats, then send and queue the message.
                {
                    c.cycles += 294;
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = REPORT_BUF_SIZE as u64;
                    c.g[A2][0] = sx(tag::NAME1);
                    c.g[S1][0] = 0x100;
                    callee::TAG_FIELD_SET_STRING.call(c, 0x0026_0794)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0x844));
                    set_number(c, tag::QUIT1, 0x0026_07ac)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0x84c));
                    set_number(c, tag::CHEAT1, 0x0026_07c4)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0x848));
                    set_number(c, tag::DISC1, 0x0026_07dc)?;
                    report_rider_stats(c, S5, tag::STATS1_BASE, tag::POINTS1, &RIDER1_RETS)?;
                    c.g[V1][0] = c.lw(frame.at(c, 0x820));
                    c.g[T0][0] = 0x270000;
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 12);
                    c.g[A2][0] = REQ_RANK as u64;
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A3][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[V1][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7110));
                    callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0026_0bb0)?;
                    c.g[A1][0] = sx(NET_EVENT_TAG);
                    c.g[A0][0] = 0x8;
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_0bc8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw(frame.at(c, 0x820));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xf0));
                    c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), sx(NET_EVENT_VTABLE));
                    c.sw(frame.at(c, 0x810), c.g[V1][0]);
                    c.g[A1][0] = 0x480000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_0c00)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0 {
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(frame.at(c, 0x810));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = c.lw(frame.at(c, 0x810));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                c.sw(frame.at(c, 0x800), c.g[A1][0]);
            }
        }
        c.cycles += 12;
        let frame = Frame { size: 0x900 };
        frame.restore(c, S0, 0x8f0);
        frame.restore(c, S1, 0x8e0);
        frame.restore(c, S2, 0x8d0);
        frame.restore(c, S3, 0x8c0);
        frame.restore(c, S4, 0x8b0);
        frame.restore(c, S5, 0x8a0);
        frame.restore(c, S6, 0x890);
        frame.restore(c, S7, 0x880);
        frame.restore(c, FP, 0x870);
        frame.restore_ra(c, 0x860);
        frame.close(c);
        Ok(())
    });
}

const RIDER1_RETS: RiderRets = RiderRets {
    ints: [
        0x0026_07f4, 0x0026_080c, 0x0026_0824, 0x0026_083c, 0x0026_0854, 0x0026_086c, 0x0026_0884,
        0x0026_089c, 0x0026_08b4, 0x0026_08cc, 0x0026_08e4, 0x0026_08fc, 0x0026_0914, 0x0026_092c,
    ],
    float_fix: [
        0x0026_093c, 0x0026_0960, 0x0026_0984, 0x0026_09a8, 0x0026_09cc, 0x0026_09f0, 0x0026_0a14,
        0x0026_0a38, 0x0026_0a5c, 0x0026_0a80,
    ],
    float_num: [
        0x0026_0950, 0x0026_0974, 0x0026_0998, 0x0026_09bc, 0x0026_09e0, 0x0026_0a04, 0x0026_0a28,
        0x0026_0a4c, 0x0026_0a70, 0x0026_0a94,
    ],
    tail: [
        0x0026_0aac, 0x0026_0ac4, 0x0026_0adc, 0x0026_0af4, 0x0026_0b0c, 0x0026_0b24, 0x0026_0b3c,
        0x0026_0b54, 0x0026_0b6c, 0x0026_0b84,
    ],
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    fn race_report_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// The engine heap, so `EA_MemAllocNamed`/`MEM_AllocInternal` have an arena.
    fn race_report_heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    /// The size-class pool allocator's heads and free lists.
    fn race_report_pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        c.bus.write32(GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pl = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                c.bus.write32(node, if n == 7 { 0 } else { node + 0x400 });
            }
            c.bus.write32(pl + head, base);
            c.bus.write32(GAME_GP.wrapping_add(slot), pl);
        }
    }

    // Branch steering is all seed arithmetic: `seed % 7` the *report==0 exit,
    // `b = seed/7` residues for auth, the race+0x18/+0x9c flags, the table
    // player order, player null-ness and the zero timers; finishes seed % 3.
    #[test]
    fn lobby_man_send_race_report_matches() {
        check(ee::s06::f_0025fd50__generated, lobby_man_send_race_report, 210, &|c, seed| {
            let mut r = machine(c, seed, 0x25_fd50, 0);
            elf_image(c);
            race_report_heap_init(c);
            race_report_pool(c);
            let b = seed / 7;
            // BELibrary dummy, interface table and world.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
            c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
            c.bus.write32(0x0053_08a8, DATA + 0x8000);
            c.bus.write32(DATA + 0x8000 + 4, DATA + 0x8100);
            c.bus.write16(DATA + 0x8100 + 8, 0);
            c.bus.write32(DATA + 0x8100 + 0xc, FAKE);
            let world = DATA + 0x9000;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x78, DATA + 0x9100);
            c.bus.write32(world + 0xc0, DATA + 0x9200);
            c.bus
                .write32(DATA + 0x9200 + 0x9c, if (b / 5) % 2 == 0 { 0 } else { 1 });
            // Race state and the static player-order table.
            let race = DATA + 0x2000;
            c.bus.write32(GAME_GP.wrapping_sub(0x204), race);
            let f18 = (b / 2) % 3;
            c.bus.write32(race + 0x18, if f18 == 0 { 0 } else { 1 });
            c.bus.write32(race + 0x9c, if f18 == 2 { 1 } else { 0 });
            c.bus.write32(
                race + 0xa0,
                if (b / 12) % 2 == 0 { 0 } else { DATA + 0x4000 },
            );
            c.bus.write32(
                race + 0xa4,
                if (b / 24) % 2 == 0 { 0 } else { DATA + 0x4100 },
            );
            c.bus.write32(race + 0x34, if (b / 3) % 2 == 0 { 0 } else { 100 });
            c.bus.write32(race + 0x38, if (b / 5) % 2 == 0 { 0 } else { 250 });
            c.bus.write32(race + 0x3c, if (b / 7) % 2 == 0 { 0 } else { 400 });
            c.bus.write32(race + 0x10, (seed / 11) % 2);
            c.bus.write32(race + 0x14, (seed / 13) % 2);
            for off in [0xcu32, 0x1c, 0x20, 0x24, 0x28, 0x30, 0xb4] {
                c.bus.write32(race + off, r.next_u32());
            }
            c.bus
                .write32(0x0053_4b38, if (b / 6) % 2 == 0 { 0 } else { 1 });
            c.bus.write32(0x0053_4b30 + 0x328, r.next_u32());
            c.bus.write32(0x0053_4b30 + 0x32c, r.next_u32());
            // Track id / desync byte and the finish-indexed tables.
            let track = [0u32, 14, 15, 16][((b / 4) % 4) as usize];
            c.bus.write32(0x0053_5c08, track);
            c.bus.write8(0x0053_5c12, if (b / 8) % 2 == 0 { 4 } else { 7 });
            for i in 0..4u32 {
                c.bus.write32(
                    0x0053_66a8 + 4 * i,
                    if (seed + i) % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write32(0x0053_6640 + 4 * i, r.next_u32());
            }
            // Report object, names and the lobby list head.
            let report = DATA + 0x1000;
            c.bus
                .write32(report, if seed % 7 == 0 { 0 } else { DATA + 0x6000 });
            c.bus.write32(report + 0x54, 0);
            c.bus.write32(report + 0xf0, DATA + 0x5000);
            c.bus.write32(DATA + 0x5000 + 4, DATA + 0x5100);
            c.bus.write32(report + 0x90, DATA + 0x3000);
            c.bus.write32(report + 0x94, DATA + 0x3100);
            c.bus.write32(report + 0x9c, DATA + 0x3200);
            c.bus.write32(report + 0xa0, r.next_u32());
            race_report_put_str(c, DATA + 0x3000, b"rider zero");
            race_report_put_str(c, DATA + 0x3100, b"rider one");
            race_report_put_str(c, DATA + 0x3200, b"auth-str");
            // Two rider stat blocks: payload ints, seed-derived edge floats.
            const EDGE: [u32; 10] = [
                0,
                0x8000_0000,
                0x3f80_0000,
                0xbf80_0000,
                0x7f80_0000,
                0x7fc0_0000,
                0x4b00_0000,
                0xcf00_0000,
                0x42c8_0000,
                0x0000_0001,
            ];
            for (si, stats) in [DATA + 0x7000, DATA + 0x7200].into_iter().enumerate() {
                for off in RIDER_STATS_INT_OFFS.iter().chain(RIDER_STATS_TAIL_OFFS.iter()) {
                    c.bus.write32(stats + off, r.next_u32());
                }
                for (i, &off) in RIDER_STATS_FLOAT_OFFS.iter().enumerate() {
                    c.bus
                        .write32(stats + off, EDGE[(seed as usize + si * 3 + i) % 10]);
                }
            }
            c.g[A0][0] = sx(report);
            c.g[A1][0] = if b % 2 == 0 { 0 } else { sx(DATA + 0x3300) };
            c.g[A2][0] = (seed % 3) as u64;
            c.g[A3][0] = ((seed / 3) % 3) as u64;
            c.g[T0][0] = sx(DATA + 0x7000);
            c.g[T1][0] = sx(DATA + 0x7200);
            c.pc = 0x0025_fd50;
        });
    }
}
