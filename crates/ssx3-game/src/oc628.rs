//! Staging module oc628 (wave g08.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc628`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_bc18,
        bolt_int_collect_equipped_file_names,
        "BEBoltInt_CollectEquippedFileNames",
    ),
    (0x0014_d608, bolts_collect_children, "Bolts_CollectChildren"),
];

/// `Bolts_GetDB()` (0x14bdb8): the bolt database.
const COLLECT_DB_CALL: Callee = Callee {
    addr: 0x0014_bdb8,
    f: ee::s01::Bolts_GetDB,
};

/// `BE_GetCharProfileData(self, profile, ch)` (0x14ad28).
const COLLECT_PROFILE_CALL: Callee = Callee {
    addr: 0x0014_ad28,
    f: ee::s01::BE_GetCharProfileData,
};

/// `Bolts_GetCharBoltCount(db, ch)` (0x14d988).
const COLLECT_COUNT_CALL: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

/// `Bolts_GetCharBolts(db, ch)` (0x14d998).
const COLLECT_BOLTS_CALL: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `BEBoltInt_GetBoltTextureName(self, profile, ch, def, out)` (0x14b988).
const COLLECT_TEXTURE_CALL: Callee = Callee {
    addr: 0x0014_b988,
    f: ee::s01::BEBoltInt_GetBoltTextureName,
};

/// `Bolts_FindFirstChild(self, group, key)` (0x14d558).
const COLLECT_FIRST_CHILD_CALL: Callee = Callee {
    addr: 0x0014_d558,
    f: ee::s01::Bolts_FindFirstChild,
};

/// `Bolts_CollectChildren` (0x14d608), through its generated name (the
/// override slot once installed).
const COLLECT_CHILDREN_CALL: Callee = Callee {
    addr: 0x0014_d608,
    f: ee::s01::f_0014d608,
};

/// `strcmp` (0x4165a8) and `strcpy` (0x4166f4), through the function table.
const COLLECT_STRCMP: u32 = 0x0041_65a8;
const COLLECT_STRCPY: u32 = 0x0041_66f4;

/// The address `reg + off`.
fn ea_reg(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `getEntry(prof, def->slot)->flags`, inlined the way the generated body
/// does it: the slot (`def+4`) indexes the keytab at `prof+0x288` (halfwords),
/// the index selects a 4-byte entry at `prof+0x290`, and its flags are the
/// halfword at +2. Leaves `v0` = the flags and `v1` = the keytab pointer.
fn collect_entry_flags(c: &mut Cpu, def: u32, prof: u32) -> u64 {
    c.cycles += 7;
    c.g[V0][0] = Addr(def.wrapping_add(4)).lh(c);
    c.g[V1][0] = Addr(prof.wrapping_add(0x288)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if negative {
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
        c.g[V0][0] = sx(prof.wrapping_add(c.g[V0][0] as u32));
    }
    c.cycles += 5;
    c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lhu(c);
    c.g[V0][0]
}

/// `int BEBoltInt_CollectEquippedTextureNames(self, profile, ch, out)`
/// (0x14b7a8). For each bolt of character `ch` (`Bolts_GetCharBolts`) that is
/// active (name pointer at +0x2c) and whose profile entry carries flag 0x10,
/// the bolt's texture file name (`BEBoltInt_GetBoltTextureName`) goes into the
/// next 256-byte slot of `out`, unless an earlier slot already holds it
/// (strcmp). Returns the number of slots used.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn bolt_int_collect_equipped_texture_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, FP, 0x20);
        let self_ptr = c.g[A0][0];
        Addr(sp(c)).sw(c, self_ptr);
        let profile = c.g[A1][0];
        Addr(sp(c).wrapping_add(4)).sw(c, profile);
        c.g[S6][0] = c.g[A3][0];
        c.g[S7][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0;
        COLLECT_DB_CALL.call(c, 0x0014_b7ec)?;
        c.g[A0][0] = Addr(sp(c)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(sp(c).wrapping_add(4)).lw(c);
        c.g[A2][0] = c.g[S7][0];
        COLLECT_PROFILE_CALL.call(c, 0x0014_b800)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[FP][0] = c.g[V0][0];
        COLLECT_BOLTS_CALL.call(c, 0x0014_b810)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S7][0];
        COLLECT_COUNT_CALL.call(c, 0x0014_b820)?;
        let count = c.g[V0][0];
        Addr(sp(c).wrapping_add(8)).sw(c, count);
        c.g[A0][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[V0][0] = ea_reg(c, S1, 0x2c).lw(c);
                c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                let active = c.g[V0][0] != 0;
                c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                if active {
                    let def = c.g[S1][0] as u32;
                    let prof = c.g[FP][0] as u32;
                    let flags = collect_entry_flags(c, def, prof);
                    c.g[V0][0] = flags & 0x10;
                    if c.g[V0][0] != 0 {
                        c.cycles += 10;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 8);
                        c.g[A0][0] = Addr(sp(c)).lw(c);
                        c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = Addr(sp(c).wrapping_add(4)).lw(c);
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S7][0];
                        c.g[T0][0] = c.g[S2][0];
                        COLLECT_TEXTURE_CALL.call(c, 0x0014_b8a0)?;
                        let named = c.g[V0][0] != 0;
                        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                        if named {
                            c.cycles += 2;
                            c.g[S0][0] = c.g[S1][0];
                            loop {
                                c.cycles += 2;
                                let done = (c.g[S0][0] as i64) < 0;
                                c.g[A0][0] = c.g[S2][0];
                                if done {
                                    break;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 8);
                                c.g[A1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                call_ptr(c, COLLECT_STRCMP, 0x0014_b8c4)?;
                                let differs = c.g[V0][0] != 0;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                                if differs {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0014_b8b0);
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                                c.g[S3][0] = c.g[S1][0];
                                break;
                            }
                            c.cycles += 1;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(sp(c).wrapping_add(8)).lw(c);
                c.g[A0][0] = c.g[S5][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = c.g[S4][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b830);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int BEBoltInt_CollectEquippedFileNames(self, profile, ch, out)`
/// (0x14bc18). Like the texture-name collector, but copies each bolt's own
/// name (+0x28) into the next 256-byte slot of `out` for every bolt with flag
/// 0x10 whose name is not yet there (strcmp over the slots). Returns the
/// number of slots used.
pub fn bolt_int_collect_equipped_file_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[FP][0] = c.g[A3][0];
        c.g[S4][0] = 0;
        COLLECT_DB_CALL.call(c, 0x0014_bc5c)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        COLLECT_PROFILE_CALL.call(c, 0x0014_bc70)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[S6][0] = c.g[V0][0];
        COLLECT_COUNT_CALL.call(c, 0x0014_bc80)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[S7][0] = c.g[V0][0];
        COLLECT_BOLTS_CALL.call(c, 0x0014_bc90)?;
        c.g[A0][0] = c.g[V0][0];
        let any = (c.g[S7][0] as i64) > 0;
        c.g[A1][0] = 0;
        if any {
            c.cycles += 1;
            c.g[S5][0] = c.g[FP][0];
            loop {
                c.cycles += 4;
                c.g[S1][0] = ea_reg(c, A0, 0x28).lw(c);
                c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                let named = c.g[S1][0] != 0;
                c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                let mut trailing = true;
                if named {
                    let def = c.g[A0][0] as u32;
                    let prof = c.g[S6][0] as u32;
                    let flags = collect_entry_flags(c, def, prof);
                    c.g[V0][0] = flags & 0x10;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
                        let mut found = false;
                        loop {
                            c.cycles += 2;
                            let done = (c.g[S0][0] as i64) < 0;
                            c.g[A1][0] = sx((c.g[S0][0] as u32) << 8);
                            if done {
                                break;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] =
                                sx((c.g[FP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            call_ptr(c, COLLECT_STRCMP, 0x0014_bd0c)?;
                            let differs = c.g[V0][0] != 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                            if differs {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_bcf8);
                                }
                                continue;
                            }
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S3][0];
                            found = true;
                            break;
                        }
                        if found {
                            trailing = false;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, COLLECT_STRCPY, 0x0014_bd28)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x100));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                        }
                    }
                }
                if trailing {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_bca0);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The scan of `Bolts_CollectChildren` over the group's own list (the
/// generated `key == -1`, or no lookup table, branch). Ends with `v0` = the
/// count (`s3`).
fn collect_children_linear_628(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[V0][0] = sx((c.g[FP][0] as u32) << 2);
    c.g[A0][0] = 0x38;
    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa4)).lw(c);
    c.g[V0][0] = c.g[V1][0];
    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
    c.mult(0, A0, S2, A0);
    c.g[V1][0] = Addr((c.g[S5][0] as u32).wrapping_add(4)).lw(c);
    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let end = c.g[A1][0];
    Addr(sp(c)).sw(c, end);
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let any = c.g[V0][0] != 0;
    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if !any {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
        return Ok(());
    }
    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(6)).lh(c);
        let mismatch = c.g[V0][0] != c.g[S4][0];
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let mut tail = true;
        if mismatch {
            c.cycles += 2;
            let below = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if below {
                return Ok(());
            }
        } else {
            c.cycles += 4;
            let bolt = c.g[S0][0];
            Addr(c.g[S1][0] as u32).sw(c, bolt);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            let depth_zero = c.g[S7][0] == 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if !depth_zero {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x34)).lw(c);
                c.g[V0][0] &= 0x20;
                let mut recurse = true;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
                } else {
                    c.cycles += 2;
                    let no_all = c.g[S6][0] == 0;
                    c.g[V1][0] = Addr(sp(c)).lw(c);
                    if no_all {
                        recurse = false;
                        tail = false;
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
                    }
                }
                if recurse {
                    c.cycles += 10;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[FP][0];
                    c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
                    c.g[T1][0] = c.g[S6][0];
                    COLLECT_CHILDREN_CALL.call(c, 0x0014_d6f8)?;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                }
            }
        }
        if tail {
            c.cycles += 1;
            c.g[V1][0] = Addr(sp(c)).lw(c);
        }
        c.cycles += 4;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[S3][0];
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_d6a8);
        }
    }
}

/// The lookup branch of `Bolts_CollectChildren`: the run of bolts that the
/// group's lookup table gives for `key` (the first one from
/// `Bolts_FindFirstChild`). Ends with `v0` = the count (`s3`).
fn collect_children_lookup_628(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A1][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S4][0];
    COLLECT_FIRST_CHILD_CALL.call(c, 0x0014_d738)?;
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V1][0] = sx((c.g[S4][0] as u32) << 1);
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = c.g[V0][0];
    if none {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
        return Ok(());
    }
    c.cycles += 1;
    loop {
        c.cycles += 4;
        let bolt = c.g[S0][0];
        Addr(c.g[S1][0] as u32).sw(c, bolt);
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        let depth_zero = c.g[S7][0] == 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        let mut tail = true;
        if !depth_zero {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x34)).lw(c);
            c.g[V0][0] &= 0x20;
            let mut recurse = true;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
            } else {
                c.cycles += 2;
                if c.g[S6][0] == 0 {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    recurse = false;
                    tail = false;
                } else {
                    c.cycles += 1;
                    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
                }
            }
            if recurse {
                c.cycles += 9;
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[FP][0];
                c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
                c.g[T1][0] = c.g[S6][0];
                COLLECT_CHILDREN_CALL.call(c, 0x0014_d79c)?;
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            }
        }
        if tail {
            c.cycles += 1;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
        }
        c.cycles += 2;
        let more = c.g[S2][0] != 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_d758);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[S3][0];
    Ok(())
}

/// `int Bolts_CollectChildren(self, group, key, out, depth, all)` (0x14d608):
/// writes the pointers of the bolts of `group` under `key` into `out` (one
/// word each), and for each bolt with children (flag 0x20 clear, or `all`)
/// recurses one level less deep. With `key` -1 or no lookup table for the
/// group, it scans the group's list; otherwise the lookup table gives the run
/// of bolts for `key`. Returns the count.
pub fn bolts_collect_children(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = c.g[T0][0];
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save_ra(c, 0x10);
        let lookup = if c.g[S4][0] == c.g[V0][0] {
            false
        } else {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[FP][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x11c));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let table = c.g[A0][0] != 0;
            c.g[A0][0] = c.g[S5][0];
            table
        };
        if lookup {
            collect_children_lookup_628(c)?;
        } else {
            collect_children_linear_628(c)?;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::{Rng, DATA};

    /// Bolt-list fixture for the two equipped-name collectors: the bolt
    /// database (`Bolts_GetDB`, 0x4a6750) with `count` bolts for character
    /// `ch`, the profile record of (`profile`, `ch`) with its keytab and
    /// flags, and a few shared name strings (some bolts repeat a name).
    pub(crate) fn oc628_bolt_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        profile: u32,
        ch: u32,
        count: u32,
    ) {
        const DB: u32 = 0x004a_6750;
        const PROFILES: u32 = 0x004a_6ca8;
        let list = DATA + 0x3000;
        let keytab = DATA + 0x1000;
        let names = DATA + 0x4000;
        let prof = PROFILES + profile * 0x9b50 + ch * 0x0f88;
        c.bus.write32(DB + ch * 4 + 0x2c, count);
        c.bus.write32(DB + ch * 4 + 0xa4, 0);
        c.bus.write32(DB + 4, list);
        c.bus.write32(prof + 0x288, keytab);
        for k in 0..8u32 {
            let idx = if r.next_u32() % 4 == 0 {
                0xffff
            } else {
                r.next_u32() % 4
            };
            c.bus.write16(keytab + 2 * k, idx as u16);
        }
        for idx in 0..4u32 {
            c.bus
                .write16(prof + 0x290 + 4 * idx + 2, (r.next_u32() & 0x13) as u16);
        }
        for i in 0..count {
            let e = list + 0x38 * i;
            c.bus
                .write8(e + 3, if seed % 7 == 0 { 0xff } else { 0 });
            c.bus.write16(e + 4, ((seed + i) % 8) as u16);
            c.bus.write32(e + 0x34, r.next_u32());
            let name = names + 0x10 * ((seed + i) % 3);
            c.bus.write32(e + 0x28, name);
            c.bus
                .write32(e + 0x2c, if (seed + i) % 5 == 0 { 0 } else { name });
        }
        for k in 0..3u32 {
            let s = names + 0x10 * k;
            c.bus.write8(s, b'b');
            c.bus.write8(s + 1, b'0' + k as u8);
            c.bus.write8(s + 2, 0);
        }
    }



    #[test]
    fn bolt_int_collect_equipped_file_names_matches() {
        check(
            ee::s01::f_0014bc18__generated,
            bolt_int_collect_equipped_file_names,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_bc18, 0);
                let profile = seed % 3;
                let ch = seed / 3 % 4;
                let count = seed / 12 % 4;
                oc628_bolt_fixture(c, &mut r, seed, profile, ch, count);
                c.pc = 0x0014_bc18;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = (DATA + 0x6000) as u64;
            },
        );
    }

    #[test]
    fn bolts_collect_children_matches() {
        check(
            ee::s01::f_0014d608__generated,
            bolts_collect_children,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_d608, 0);
                let group = seed % 3;
                let key = [u64::MAX, 7, 0x41][(seed / 3 % 3) as usize];
                let variant = seed / 9 % 4;
                c.pc = 0x0014_d608;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = group as u64;
                c.g[A2][0] = key;
                c.g[A3][0] = (DATA + 0x6000) as u64;
                c.g[T0][0] = (seed % 3) as u64;
                c.g[T1][0] = (seed / 27 % 2) as u64;
                c.bus.write32(DATA + 4, DATA + 0x2000);
                for g in 0..3u32 {
                    c.bus.write32(DATA + g * 4 + 0x2c, 3);
                    c.bus.write32(DATA + g * 4 + 0xa4, 0);
                    c.bus.write32(
                        DATA + g * 4 + 0x11c,
                        if variant == 0 { 0 } else { DATA + 0x1200 },
                    );
                    c.bus.write32(DATA + g * 4 + 0x194, DATA + 0x1000);
                }
                for k in 0..0x80u32 {
                    c.bus
                        .write16(DATA + 0x1200 + 2 * k, (r.next_u32() % 3) as u16);
                    c.bus
                        .write16(DATA + 0x1000 + 2 * k, (r.next_u32() % 3) as u16);
                }
                for i in 0..3u32 {
                    let e = DATA + 0x2000 + 0x38 * i;
                    for w in 0..0x38 / 4 {
                        c.bus.write32(e + 4 * w, r.next_u32());
                    }
                    c.bus.write16(e + 4, (r.next_u32() % 0x80) as u16);
                    c.bus.write16(e + 6, [0x41u16, 7, 3][i as usize]);
                }
            },
        );
    }
}
