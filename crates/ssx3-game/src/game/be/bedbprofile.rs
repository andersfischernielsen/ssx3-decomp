//! `src/be/bedbprofile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::object::flexrailnode::at_3 as at;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0015_2460,
    be_record_char_profile_event_score,
    "BE_RecordCharProfileEventScore",
)];

/// Direct-call targets (dossier callees, found by address in `generated/ee`).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const BE_GET_EVENT_RECORD_INDEX: Callee = Callee {
        addr: 0x0014_ab68,
        f: ee::s01::BE_GetEventRecordIndex,
    };

    pub const WORLD_TRIGGER_FIND_INSTANCE: Callee = Callee {
        addr: 0x002b_5bc0,
        f: ee::s08::WorldTriggerManager_FindInstance,
    };

    pub const WORLD_TRIGGER_FIND_FREE: Callee = Callee {
        addr: 0x002b_5b90,
        f: ee::s08::WorldTriggerManager_FindFreeInstance,
    };

    pub const SHP_NORMALIZE_BPP: Callee = Callee {
        addr: 0x003b_2528,
        f: ee::s13::SHP_NormalizeBpp,
    };

    pub const SHP_CALC_IMAGE_SIZE: Callee = Callee {
        addr: 0x003b_2558,
        f: ee::s13::SHP_CalcImageSize,
    };

    pub const SHP_PALETTE_ENTRIES_FOR_BPP: Callee = Callee {
        addr: 0x003b_2688,
        f: ee::s13::SHP_PaletteEntriesForBpp,
    };

    pub const CLOSEST_POINT_ON_SEGMENT: Callee = Callee {
        addr: 0x0033_0128,
        f: ee::s11::ClosestPointOnSegment,
    };
}

/// `int BE_RecordCharProfileEventScore(self, event, rank, score)` (0x152460):
/// record a profile event score when it beats the stored one.
pub fn be_record_char_profile_event_score(c: &mut Cpu) {
    guest(c, |c| {
        'b_152514: {
            'b_152510: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A2][0];
                c.g[A0][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S1][0] = c.g[A3][0];
                c.g[RA][0] = sx(0x0015248c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0014ab68, 0x0015248c)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x1a;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_152510;
                } else {
                    c.cycles += 2;
                    if c.g[S1][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_152514;
                    } else {
                        'b_152508: {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xad0));
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x4;
                            if t {
                                break 'b_152508;
                            } else {
                                'b_152500: {
                                    'b_1524fc: {
                                        'b_1524f8: {
                                            'b_1524f0: {
                                                c.cycles += 2;
                                                if c.g[S0][0] == c.g[V0][0] {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    break 'b_1524f0;
                                                } else {
                                                    'b_1524e0: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            ((c.g[S0][0] as i64) < 5i64) as u64;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] =
                                                                ((c.g[S0][0] as i64) < 9i64) as u64;
                                                            break 'b_1524e0;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[S0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            if t {
                                                                break 'b_1524f0;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[V0][0] = (c.g[V0][0]
                                                                    < c.g[S1][0])
                                                                    as u64;
                                                                if t {
                                                                    break 'b_152500;
                                                                } else {
                                                                    break 'b_1524e0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] =
                                                        ((c.g[S0][0] as i64) < 6i64) as u64;
                                                    if t {
                                                        break 'b_1524f8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            break 'b_1524fc;
                                                        } else {
                                                            break 'b_1524f0;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                                            if t {
                                                break 'b_152500;
                                            } else {
                                                break 'b_1524f8;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                        break 'b_1524fc;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = (c.g[V0][0] < c.g[S1][0]) as u64;
                                    break 'b_152500;
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_152510;
                                } else {
                                    break 'b_152508;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.g[V0][0] = 0x1;
                        break 'b_152510;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_152514;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `int BE_RecordCharProfileEventPlace(Profile* p, int event, int place,
/// int value)` (0x152528): record that `event` happened at `place` with
/// `value` in the character profile — bump the per-place counter, keep the
/// best (smallest) value, and bump the event's record count. Returns 1 when
/// recorded, 0 when the event is unknown or the value is worse.
pub fn be_record_char_profile_event_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x60));
        c.sq(16, spa(c, 0x50));
        c.sq(17, spa(c, 0x40));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, spa(c, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x20));
        c.g[S2][0] = c.g[A3][0];
        c.sq(20, spa(c, 0x10));
        c.g[S3][0] = c.g[A1][0];
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        let no_value = c.g[S2][0] == c.g[S4][0];
        c.sd(spa(c, 0), c.g[RA][0]);
        if no_value {
            // No value: nothing recorded.
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0015_2568);
            c.g[A1][0] = c.g[S0][0];
            callee::BE_GET_EVENT_RECORD_INDEX.call(c, 0x0015_2568)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 0x1a;
            let unknown = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx(r32v(c, V1) << 3);
            if unknown {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                // The per-event byte table at profile+0xad0: mark the record
                // seen, and count the places when this value is >= 2.
                let rec = r32v(c, S1).wrapping_add(r32v(c, V0)).wrapping_add(0xad0);
                c.g[A2][0] = sx(rec);
                c.cycles += 6;
                c.g[A0][0] = 1;
                c.g[V1][0] = (c.g[S2][0] < 2) as u64;
                let count_place = c.g[V1][0] == 0;
                c.g[A2][0] = sx(rec);
                c.sb(rec, c.g[A0][0]);
                if count_place {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at(rec, 2)).lhu(c);
                    c.g[V0][0] = sx(r32v(c, V0).wrapping_add(1));
                    c.sh(at(rec, 2), c.g[V0][0]);
                }
                // Keep the best (smallest) value for the record: 0 = the
                // record was unset (run the per-place counters), 1 = it was
                // improved, 2 = the new value is worse (nothing recorded).
                let mut outcome = 0u32;
                c.cycles += 3;
                c.g[V0][0] = Addr(at(rec, 1)).lb(c);
                let unset = c.g[V0][0] == c.g[S4][0];
                let better = (c.g[S2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = better as u64;
                if !unset {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        outcome = 2;
                    } else {
                        c.cycles += 2;
                        c.sb(at(rec, 1), c.g[S2][0]);
                        outcome = 1;
                    }
                }
                if outcome != 2 {
                    if outcome == 0 {
                        // Per-place record: 100 bytes each, the counter index
                        // at +0x54, counters at profile+0xba0 (place 0) or
                        // +0xba4 (places 1..3); places >= 4 have no counter.
                        c.cycles += 7;
                        c.g[V1][0] = 0x64;
                        c.g[V0][0] = 0x44_0000;
                        c.mult(0, 3, 19, 3);
                        c.g[V0][0] = sx(r32v(c, V0).wrapping_sub(0x26b0));
                        c.g[V0][0] = sx(r32v(c, V0).wrapping_add(r32v(c, V1)));
                        let have_place = c.g[S0][0] != 0;
                        c.g[V1][0] = Addr(at(r32v(c, V0), 0x54)).lw(c);
                        let mut bump = have_place;
                        if !have_place {
                            c.cycles += 3;
                            c.g[A0][0] = sx(r32v(c, V1) << 3);
                            c.g[V1][0] = sx(r32v(c, S1).wrapping_add(0xba0));
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx(r32v(c, S0).wrapping_sub(1));
                            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                            let no_counter = c.g[V0][0] == 0;
                            c.g[A0][0] = sx(r32v(c, V1) << 3);
                            if no_counter {
                                bump = false;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = sx(r32v(c, S1).wrapping_add(0xba4));
                            }
                        }
                        if bump {
                            c.cycles += 4;
                            c.g[V1][0] = sx(r32v(c, V1).wrapping_add(r32v(c, A0)));
                            c.g[V0][0] = Addr(r32v(c, V1)).lw(c);
                            c.g[V0][0] = sx(r32v(c, V0).wrapping_add(1));
                            c.sw(r32v(c, V1), c.g[V0][0]);
                        }
                    }
                    c.cycles += 1;
                    c.sb(at(rec, 1), c.g[S2][0]);
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                } else {
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 8;
        c.lq(16, spa(c, 0x50));
        c.lq(17, spa(c, 0x40));
        c.lq(18, spa(c, 0x30));
        c.lq(19, spa(c, 0x20));
        c.lq(20, spa(c, 0x10));
        c.g[RA][0] = c.ld(spa(c, 0));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x60));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    use ssx3_ee as ee;

    /// A strict differential run: every GPR (including caller-saved scratch),
    /// every FPR and LO/HI must match. Call sites are visible to the fake
    /// callees, so scratch registers matter even where `check` would ignore
    /// them.
    pub(crate) fn strict(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seeds {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    #[test]
    fn be_record_char_profile_event_score_matches() {
        strict(
            ee::s01::f_00152460__generated,
            be_record_char_profile_event_score,
            120,
            &|c, seed| {
                let _r = base(c, seed, 0x1524);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 0x1c) as u64;
                c.g[A2][0] = (seed % 10) as u64;
                c.g[A3][0] = (seed % 5) as u64;
                for i in 0..0x1c {
                    let rec = DATA + 0xad0 + 8 * i;
                    if seed % 3 == 0 {
                        c.bus.write32(rec + 4, 0);
                    } else {
                        c.bus.write32(
                            rec + 4,
                            (seed as u32).wrapping_mul(7).wrapping_add(i as u32),
                        );
                    }
                }
            },
        );
    }
}
