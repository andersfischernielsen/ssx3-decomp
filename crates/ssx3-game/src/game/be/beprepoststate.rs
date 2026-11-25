//! `src/be/beprepoststate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bedbreward::POST_RACE_GP;
pub(crate) use crate::game::be::bedbreward::PRE_RACE_GP;
pub(crate) use crate::game::be::beintmission::event_check;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
pub(crate) use crate::game::be::beintreward::alloc_named;
pub(crate) use crate::game::be::belibrary::EA_MEM_COPY;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_6338,
        be_post_race_get_instance,
        "BEPostRace_GetInstance",
    ),
    (
        0x0015_6570,
        be_pre_race_get_instance,
        "BEPreRace_GetInstance",
    ),
    (0x0015_64b0, be_post_race_exit, "BEPostRace_Exit"),
    (0x0015_6510, be_post_race_enter, "BEPostRace_Enter"),
    (0x0015_66e8, be_pre_race_exit, "BEPreRace_Exit"),
    (0x0015_6750, be_pre_race_enter, "BEPreRace_Enter"),
    (
        0x0015_67b8,
        be_reset_profile_characters,
        "BE_ResetProfileCharacters",
    ),
    (
        0x0015_6810,
        be_reset_profile_rivalries,
        "BE_ResetProfileRivalries",
    ),
    (0x0015_6950, be_backup_character, "BE_BackupCharacter"),
    (0x0015_6988, be_restore_character, "BE_RestoreCharacter"),
    (
        0x0015_65c8,
        be_pre_race_get_interface,
        "BEPreRace_GetInterface",
    ),
    (
        0x0015_6858,
        be_copy_profile_characters,
        "BE_CopyProfileCharacters",
    ),
    (
        0x0015_6390,
        be_post_race_get_interface,
        "BEPostRace_GetInterface",
    ),
];

/// `BEPostRace_GetInstance(int id)` (0x156338): as above with `gp - 0x1e9c`
/// and the "BEPostRace" name.
pub fn be_post_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        POST_RACE_GP,
        0x0015_6368,
        8,
        0x0045_a8a8,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_abb8))],
    );
}

/// `BEPreRace_GetInstance(int id)` (0x156570): as above with `gp - 0x1e98`
/// and the "BEPreRace" name.
pub fn be_pre_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        PRE_RACE_GP,
        0x0015_65a0,
        8,
        0x0045_a8a8,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_ac18))],
    );
}

/// What to store in an allocated singleton's word.
#[derive(Clone, Copy)]
pub(crate) enum Arg {
    /// The function's first argument (`a0`).
    A0,
    /// A constant word.
    Const(u32),
}

/// The shared `*_GetInstance` body: return the cached singleton, allocating
/// it (and storing the given words) when the global is still zero.
pub(crate) fn get_instance(
    c: &mut Cpu,
    gp_off: u32,
    ret: u32,
    size: u64,
    name: u32,
    init: &[(u32, Arg)],
) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let global = (c.g[GP][0] as u32).wrapping_sub(gp_off);
        let inst = Addr(global).lw(c);
        c.g[V0][0] = inst;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let present = inst != 0;
        c.g[S0][0] = c.g[A0][0];
        if !present {
            c.cycles += 11;
            let block = alloc_named(c, size, name, ret)?;
            for &(off, value) in init {
                let v = match value {
                    Arg::A0 => c.g[S0][0],
                    Arg::Const(x) => x as u64,
                };
                c.g[V1][0] = v;
                Addr(block as u32 + off).sw(c, v);
            }
            Addr(global).sw(c, block);
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(global).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BEPreRace_GetInterface (0x1565c8)

/// The jump table of `BEPreRace_GetInterface`.
pub(crate) const BE_PRE_RACE_INTERFACE_TABLE: u32 = 0x0045_a900;

/// The interface singletons by kind: (getter, its case block address).
pub(crate) const BE_PRE_RACE_INTERFACES: [(Callee, u32); 14] = [
    (
        Callee {
            addr: 0x0014_48d8,
            f: ee::s01::BENewRaceInt_GetInstance,
        },
        0x0015_65f4,
    ),
    (
        Callee {
            addr: 0x0014_5970,
            f: ee::s01::BENewPlayerInt_GetInstance,
        },
        0x0015_6604,
    ),
    (
        Callee {
            addr: 0x0014_edd8,
            f: ee::s01::BE_GetCharDBInterface,
        },
        0x0015_6614,
    ),
    (
        Callee {
            addr: 0x0014_7f78,
            f: ee::s01::BECharInt_GetInstance,
        },
        0x0015_6624,
    ),
    (
        Callee {
            addr: 0x0014_f250,
            f: ee::s01::BEOptionInt_GetInstance,
        },
        0x0015_6634,
    ),
    (
        Callee {
            addr: 0x0015_2638,
            f: ee::s01::BESaveInt_GetInstance,
        },
        0x0015_6644,
    ),
    (
        Callee {
            addr: 0x0014_f900,
            f: ee::s01::BEBAGTInt_GetInstance,
        },
        0x0015_6684,
    ),
    (
        Callee {
            addr: 0x0014_e0e0,
            f: ee::s01::BENetworkInt_GetInstance,
        },
        0x0015_6654,
    ),
    (
        Callee {
            addr: 0x0015_4948,
            f: ee::s01::BEScoreInt_GetInstance,
        },
        0x0015_6664,
    ),
    (
        Callee {
            addr: 0x0014_abe0,
            f: ee::s01::BEBoltInt_GetInstance,
        },
        0x0015_6674,
    ),
    (
        Callee {
            addr: 0x0015_32a0,
            f: ee::s01::BEMissionInt_GetInstance,
        },
        0x0015_66a4,
    ),
    (
        Callee {
            addr: 0x0015_08d8,
            f: ee::s01::BEEconInt_GetInstance,
        },
        0x0015_6694,
    ),
    (
        Callee {
            addr: 0x0015_5a50,
            f: ee::s01::BEAggressionInt_GetInstance,
        },
        0x0015_66b4,
    ),
    (
        Callee {
            addr: 0x0015_69c0,
            f: ee::s01::BERewardInt_GetInstance,
        },
        0x0015_66c4,
    ),
];

/// `void* BEPreRace_GetInterface(void* this, int kind)` (0x1565c8): the
/// back-end interface singleton of `kind` (new race, new player, character
/// DB, characters, options, save, BAGT, network, score, bolt, mission,
/// economy, aggression, reward), or null for other kinds.
pub fn be_pre_race_get_interface(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let kind = c.g[A1][0];
        c.g[V0][0] = (kind < 0xe) as u64;
        frame.save_ra(c, 0);
        if kind < 0xe {
            c.cycles += 7;
            let entry = BE_PRE_RACE_INTERFACE_TABLE.wrapping_add((kind as u32) << 2);
            c.g[V0][0] = sx(BE_PRE_RACE_INTERFACE_TABLE);
            c.g[V1][0] = sx(entry);
            c.g[A0][0] = Addr(entry).lw(c);
            let (getter, block) = BE_PRE_RACE_INTERFACES[kind as usize];
            c.cycles += 4;
            getter.call(c, block + 8)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cBEProfileLib_copy(sBEPlayerData* dst, sBEPlayerData* src)`
/// (0x156858): copy the ten 0xf88-byte rider records from `src` to `dst`
/// and rewrite each record's 16-bit tables (the entry count is gp-0x1f08).
pub fn be_copy_profile_characters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 0xf88;
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);
        c.mult(0, 5, 18, 20);
        'profile: loop {
            'record: {
                c.cycles += 11;
                c.g[A2][0] = 0xf88;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, 0x003e_6574, 0x0015_68a8)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x288), c.g[S1][0]);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0;
                if t {
                    break 'record;
                }
                c.cycles += 3;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[V1][0] = c.g[S1][0];
                loop {
                    c.cycles += 7;
                    c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    if !t {
                        break 'record;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_68c8);
                    }
                }
            }
            c.cycles += 5;
            c.mult(0, 3, 18, 20);
            c.g[A0][0] = 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            c.g[V0][0] = c.lh(c.g[A1][0] as u32);
            'scan: loop {
                'store: {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
                    if t {
                        break 'store;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.sh(c.g[V0][0] as u32, c.g[A0][0]);
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 525) as u64;
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[V0][0] = c.lh(c.g[A1][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0015_68f8);
                }
            }
            c.cycles += 4;
            c.g[S2][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[S2][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                break 'profile;
            }
            c.mult(0, 5, 18, 20);
            if event_due(c) {
                deliver_event_here(c, 0x0015_6890);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEPostRace_GetInterface` (0x00156390): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn be_post_race_get_interface(c: &mut Cpu) {
    if c.pc != 0x00156390 {
        return ee::s01::f_00156390__generated(c);
    }
    guest(c, |c| 'l_156390: loop {
        'b_1564a4: {
            'b_15649c: {
                c.cycles += 4;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V0][0] = (c.g[A1][0] < 0xeu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_15649c;
                } else {
                    'b_15648c: {
                        'b_15647c: {
                            'b_15646c: {
                                'b_15645c: {
                                    'b_15644c: {
                                        'b_15643c: {
                                            'b_15642c: {
                                                'b_15641c: {
                                                    'b_15640c: {
                                                        'b_1563fc: {
                                                            'b_1563ec: {
                                                                'b_1563dc: {
                                                                    'b_1563cc: {
                                                                        c.cycles += 7;
                                                                        c.g[V0][0] = 0x460000;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x5740,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x00156390 => {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x00156390);
                                                                                }
                                                                                continue 'l_156390;
                                                                            }
                                                                            0x001563bc => {
                                                                                c.cycles += 4;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001563c4);
                                                                                Callee { addr: 0x001448d8, f: ee::s01::BENewRaceInt_GetInstance }.call(c, 0x001563c4)?;
                                                                                let t = true;
                                                                                c.g[RA][0] = c
                                                                                    .ld(c.g[SP][0]
                                                                                        as u32);
                                                                                if t {
                                                                                    break 'b_1564a4;
                                                                                } else {
                                                                                    break 'b_1563cc;
                                                                                }
                                                                            }
                                                                            0x001563cc => {
                                                                                break 'b_1563cc;
                                                                            }
                                                                            0x001563dc => {
                                                                                break 'b_1563dc;
                                                                            }
                                                                            0x001563ec => {
                                                                                break 'b_1563ec;
                                                                            }
                                                                            0x001563fc => {
                                                                                break 'b_1563fc;
                                                                            }
                                                                            0x0015640c => {
                                                                                break 'b_15640c;
                                                                            }
                                                                            0x0015641c => {
                                                                                break 'b_15641c;
                                                                            }
                                                                            0x0015642c => {
                                                                                break 'b_15642c;
                                                                            }
                                                                            0x0015643c => {
                                                                                break 'b_15643c;
                                                                            }
                                                                            0x0015644c => {
                                                                                break 'b_15644c;
                                                                            }
                                                                            0x0015645c => {
                                                                                break 'b_15645c;
                                                                            }
                                                                            0x0015646c => {
                                                                                break 'b_15646c;
                                                                            }
                                                                            0x0015647c => {
                                                                                break 'b_15647c;
                                                                            }
                                                                            0x0015648c => {
                                                                                break 'b_15648c;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x00156390,
                                                                                    0x001564ac,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x001563d4);
                                                                    Callee { addr: 0x00145970, f: ee::s01::BENewPlayerInt_GetInstance }.call(c, 0x001563d4)?;
                                                                    let t = true;
                                                                    c.g[RA][0] =
                                                                        c.ld(c.g[SP][0] as u32);
                                                                    if t {
                                                                        break 'b_1564a4;
                                                                    } else {
                                                                        break 'b_1563dc;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x001563e4);
                                                                Callee { addr: 0x0014edd8, f: ee::s01::BE_GetCharDBInterface }.call(c, 0x001563e4)?;
                                                                let t = true;
                                                                c.g[RA][0] =
                                                                    c.ld(c.g[SP][0] as u32);
                                                                if t {
                                                                    break 'b_1564a4;
                                                                } else {
                                                                    break 'b_1563ec;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x001563f4);
                                                            Callee {
                                                                addr: 0x00147f78,
                                                                f: ee::s01::BECharInt_GetInstance,
                                                            }
                                                            .call(c, 0x001563f4)?;
                                                            let t = true;
                                                            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                            if t {
                                                                break 'b_1564a4;
                                                            } else {
                                                                break 'b_1563fc;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x00156404);
                                                        Callee {
                                                            addr: 0x0014f250,
                                                            f: ee::s01::BEOptionInt_GetInstance,
                                                        }
                                                        .call(c, 0x00156404)?;
                                                        let t = true;
                                                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                        if t {
                                                            break 'b_1564a4;
                                                        } else {
                                                            break 'b_15640c;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x00156414);
                                                    Callee {
                                                        addr: 0x00152638,
                                                        f: ee::s01::BESaveInt_GetInstance,
                                                    }
                                                    .call(c, 0x00156414)?;
                                                    let t = true;
                                                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                    if t {
                                                        break 'b_1564a4;
                                                    } else {
                                                        break 'b_15641c;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00156424);
                                                Callee {
                                                    addr: 0x0014e0e0,
                                                    f: ee::s01::BENetworkInt_GetInstance,
                                                }
                                                .call(c, 0x00156424)?;
                                                let t = true;
                                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                if t {
                                                    break 'b_1564a4;
                                                } else {
                                                    break 'b_15642c;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x00156434);
                                            Callee {
                                                addr: 0x00154948,
                                                f: ee::s01::BEScoreInt_GetInstance,
                                            }
                                            .call(c, 0x00156434)?;
                                            let t = true;
                                            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                            if t {
                                                break 'b_1564a4;
                                            } else {
                                                break 'b_15643c;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00156444);
                                        Callee {
                                            addr: 0x0014abe0,
                                            f: ee::s01::BEBoltInt_GetInstance,
                                        }
                                        .call(c, 0x00156444)?;
                                        let t = true;
                                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                        if t {
                                            break 'b_1564a4;
                                        } else {
                                            break 'b_15644c;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00156454);
                                    Callee {
                                        addr: 0x0014f900,
                                        f: ee::s01::BEBAGTInt_GetInstance,
                                    }
                                    .call(c, 0x00156454)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                    if t {
                                        break 'b_1564a4;
                                    } else {
                                        break 'b_15645c;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00156464);
                                Callee {
                                    addr: 0x001508d8,
                                    f: ee::s01::BEEconInt_GetInstance,
                                }
                                .call(c, 0x00156464)?;
                                let t = true;
                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                if t {
                                    break 'b_1564a4;
                                } else {
                                    break 'b_15646c;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00156474);
                            Callee {
                                addr: 0x001532a0,
                                f: ee::s01::BEMissionInt_GetInstance,
                            }
                            .call(c, 0x00156474)?;
                            let t = true;
                            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                            if t {
                                break 'b_1564a4;
                            } else {
                                break 'b_15647c;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00156484);
                        Callee {
                            addr: 0x00155a50,
                            f: ee::s01::BEAggressionInt_GetInstance,
                        }
                        .call(c, 0x00156484)?;
                        let t = true;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        if t {
                            break 'b_1564a4;
                        } else {
                            break 'b_15648c;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00156494);
                    Callee {
                        addr: 0x001569c0,
                        f: ee::s01::BERewardInt_GetInstance,
                    }
                    .call(c, 0x00156494)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        break 'b_1564a4;
                    } else {
                        break 'b_15649c;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_1564a4;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn be_reset_profile_matches() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_001567b8 as fn(&mut Cpu),
                be_reset_profile_characters as fn(&mut Cpu),
                0x67b8,
            ),
            (ssx3_ee::s01::f_00156810, be_reset_profile_rivalries, 0x6810),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x8000);
            });
        }
    }

    #[test]
    fn be_backup_restore_character_matches() {
        check(
            ssx3_ee::s01::f_00156950,
            be_backup_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6950, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 8) as u64;
                fill(c, &mut r, DATA, 0x4000);
            },
        );
        check(
            ssx3_ee::s01::f_00156988,
            be_restore_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6988, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write8(GAME_GP.wrapping_sub(0x1e94), (seed % 8) as u8);
                fill(c, &mut r, DATA, 0x4000);
            },
        );
    }

    pub(crate) const SEEDS: u32 = 40;

    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BELibrary singleton (`gp - 0x1F04`) and a zeroed dummy object it
    /// points at: without it a library lookup tries to build the whole BE
    /// singleton graph (and allocate from a zeroed heap).
    pub(crate) fn belib(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    /// A run of distinct `BE*` singletons for the visibility-flag functions.
    pub(crate) fn race_singletons(c: &mut Cpu) {
        const OFFSETS: [i32; 12] = [
            -0x1f3c, -0x1f30, -0x1f2c, -0x1ee8, -0x1ee4, -0x1ed4, -0x1eec, -0x1f28, -0x1ee0,
            -0x1edc, -0x1ea4, -0x1e8c,
        ];
        for (i, off) in OFFSETS.into_iter().enumerate() {
            w(
                c,
                (GAME_GP as i32 + off) as u32,
                DATA + 0x100 + 0x40 * i as u32,
            );
        }
    }

    // ------------------------------------------------------- race flow flags

    #[test]
    fn be_post_race_exit_matches() {
        check(
            ee::s01::f_001564b0__generated,
            be_post_race_exit,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_64b0, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    #[test]
    fn be_post_race_enter_matches() {
        check(
            ee::s01::f_00156510__generated,
            be_post_race_enter,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_6510, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    #[test]
    fn be_pre_race_exit_matches() {
        check(
            ee::s01::f_001566e8__generated,
            be_pre_race_exit,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_66e8, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    #[test]
    fn be_pre_race_enter_matches() {
        check(
            ee::s01::f_00156750__generated,
            be_pre_race_enter,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0015_6750, 0);
                belib(c);
                race_singletons(c);
            },
        );
    }

    pub(crate) use crate::game::seg::seg_2fe810::tests::oc383_machine;

    #[test]
    fn be_pre_race_get_interface_matches() {
        check(
            ee::s01::f_001565c8__generated,
            be_pre_race_get_interface,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0x65c8, 0x0015_65c8);
                // Every singleton already made (the getters' short path).
                for off in [
                    0x1f3c, 0x1f30, 0x1ee8, 0x1f2c, 0x1ee4, 0x1ed4, 0x1eec, 0x1ea8, 0x1f28, 0x1ee0,
                    0x1edc, 0x1eb8, 0x1ea4, 0x1e8c,
                ] {
                    c.bus.write32(GAME_GP - off, DATA + (r.next_u32() & 0xfff0));
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = match seed % 16 {
                    14 => r.next_u32() as u64,
                    15 => u64::MAX,
                    k => k as u64,
                };
            },
        );
    }

    use crate::difftest::Rng;

    #[test]
    fn be_copy_profile_characters_matches() {
        check(
            ssx3_ee::s01::f_00156858__generated,
            be_copy_profile_characters,
            60,
            &|c, seed| {
                let mut r = Rng(0x6858_0000 ^ seed as u64);
                machine(c, seed, 0x156858, 0);
                let dst = DATA + 0x20_0000;
                let src = DATA + 0x30_0000;
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x1f08), seed % 4);
                for i in 0..10u32 {
                    let s = src + i * 0xf88;
                    fill(c, &mut r, s, 0xf88 / 4);
                    c.bus.write32(s + 0x288, DATA + 0x40_0000 + i * 0x100);
                    c.bus
                        .write32(dst + i * 0xf88 + 0x288, DATA + 0x40_0000 + i * 0x100);
                    for k in 0..0x200u32 {
                        let v = if r.next_u32() & 1 == 0 {
                            0x8000 | (r.next_u32() as u16)
                        } else {
                            (r.next_u32() % 0x100) as u16
                        };
                        c.bus.write16(s + 0x290 + 4 * k, v);
                    }
                }
                fill(c, &mut r, DATA + 0x40_0000, 0x1000);
            },
        );
    }

    #[test]
    fn be_post_race_get_interface_matches() {
        check(
            ee::s01::f_00156390__generated,
            be_post_race_get_interface,
            200,
            &|c, seed| {
                machine(c, seed, 0x6390, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00156390;
            },
        );
    }
}

pub(crate) const F_00151600: Callee = Callee {
    addr: 0x0015_1600,
    f: ssx3_ee::s01::f_00151600,
};

pub(crate) const F_001519E0: Callee = Callee {
    addr: 0x0015_19e0,
    f: ssx3_ee::s01::f_001519e0,
};

/// `void BE_ResetProfileCharacters(void *self, int arg)` (0x1567b8): reset the
/// ten characters (`f_00151600(self + i*0xf88, i, arg)`).
pub fn be_reset_profile_characters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            F_00151600.call(c, 0x0015_67ec)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(RIDER_STRIDE));
            if t {
                event_check(c, 0x0015_67d8);
                continue;
            } else {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BE_ResetProfileRivalries(void *self)` (0x156810): reset the ten
/// rivalries (`f_001519e0(self + i*0xf88, i)`).
pub fn be_reset_profile_rivalries(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            F_001519E0.call(c, 0x0015_6834)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(RIDER_STRIDE));
            if t {
                event_check(c, 0x0015_6828);
                continue;
            } else {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BE_BackupCharacter(int self, int rider)` (0x156950): copy the rider's
/// 0xf88-byte character record to 0x4c4090 and remember the rider index.
pub fn be_backup_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = RIDER_STRIDE as u64;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        c.sb((c.g[GP][0] as u32).wrapping_sub(0x1e94), c.g[A1][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = RIDER_STRIDE as u64;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x4c0000u32.wrapping_add(0x4090));
        call_ptr(c, EA_MEM_COPY, 0x0015_697c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_RestoreCharacter(int self)` (0x156988): copy 0x4c4090 back to the
/// remembered rider's character record.
pub fn be_restore_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_sub(0x1e94));
        c.g[V1][0] = RIDER_STRIDE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x4c0000;
        c.mult(0, A3, V0, V1);
        frame.save_ra(c, 0);
        c.g[A2][0] = RIDER_STRIDE as u64;
        c.g[A1][0] = sx(0x4c0000u32.wrapping_add(0x4090));
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, EA_MEM_COPY, 0x0015_69b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `base + off` as an effective address (`addu` of the low 32 bits).
#[inline(always)]
pub(crate) fn ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gp_rel(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lw rt, gp+off`.
#[inline(always)]
pub(crate) fn gp_lw(c: &mut Cpu, off: i32) -> u64 {
    c.lw(gp_rel(c, off))
}

// ---------------------------------------------------------------------------
// Pre/post-race visibility flags.

/// `void BEPostRace_Exit()` (0x1564B0) and friends: clear (or set) the
/// `visible` word at `+8` of a run of `BE*` singletons. The loads are
/// software-pipelined, one register ahead of the stores.
pub(crate) fn be_race_flags(c: &mut Cpu, value: u64, loads: &[(usize, i32)]) {
    c.g[loads[0].0][0] = gp_lw(c, loads[0].1);
    for k in 1..loads.len() {
        c.g[loads[k].0][0] = gp_lw(c, loads[k].1);
        let prev = loads[k - 1].0;
        c.sw(ea(c.g[prev][0], 8), value);
    }
    let last = loads[loads.len() - 1].0;
    c.sw(ea(c.g[last][0], 8), value);
}

pub fn be_post_race_exit(c: &mut Cpu) {
    c.cycles += 23;
    be_race_flags(
        c,
        0,
        &[
            (V0, -0x1f30),
            (V1, -0x1ee8),
            (A0, -0x1f2c),
            (V0, -0x1ee4),
            (V1, -0x1ed4),
            (A0, -0x1eec),
            (V0, -0x1f28),
            (V1, -0x1ee0),
            (A0, -0x1edc),
            (A1, -0x1ea4),
            (V0, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_post_race_enter(c: &mut Cpu) {
    c.cycles += 24;
    c.g[V0][0] = 1;
    be_race_flags(
        c,
        1,
        &[
            (V1, -0x1f30),
            (A0, -0x1ee8),
            (A1, -0x1f2c),
            (V1, -0x1ee4),
            (A0, -0x1ed4),
            (A1, -0x1eec),
            (V1, -0x1f28),
            (A0, -0x1ee0),
            (A1, -0x1edc),
            (A2, -0x1ea4),
            (V1, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_pre_race_exit(c: &mut Cpu) {
    c.cycles += 25;
    be_race_flags(
        c,
        0,
        &[
            (V0, -0x1f3c),
            (V1, -0x1f30),
            (A0, -0x1ee8),
            (V0, -0x1f2c),
            (V1, -0x1ee4),
            (A0, -0x1ed4),
            (V0, -0x1eec),
            (V1, -0x1f28),
            (A0, -0x1ee0),
            (A1, -0x1edc),
            (V1, -0x1ea4),
            (V0, -0x1e8c),
        ],
    );
    abi::ret(c);
}

pub fn be_pre_race_enter(c: &mut Cpu) {
    c.cycles += 26;
    c.g[V0][0] = 1;
    be_race_flags(
        c,
        1,
        &[
            (V1, -0x1f3c),
            (A0, -0x1f30),
            (A1, -0x1ee8),
            (V1, -0x1f2c),
            (A0, -0x1ee4),
            (A1, -0x1ed4),
            (V1, -0x1eec),
            (A0, -0x1f28),
            (A1, -0x1ee0),
            (A2, -0x1edc),
            (A0, -0x1ea4),
            (V1, -0x1e8c),
        ],
    );
    abi::ret(c);
}
