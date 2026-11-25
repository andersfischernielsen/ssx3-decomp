//! Staging module oc676 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc676`.

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
        0x0014fc40,
        be_bagt_int_reset_profile,
        "BEBAGTInt_ResetProfile",
    ),
    (
        0x001502c8,
        be_bagt_int_get_trick_anim2,
        "BEBAGTInt_GetTrickAnim2",
    ),
    (
        0x001503f8,
        be_bagt_int_get_trick_value,
        "BEBAGTInt_GetTrickValue",
    ),
    (
        0x00150c20,
        be_econ_int_buy_attribute,
        "BEEconInt_BuyAttribute",
    ),
    (
        0x00151040,
        be_reward_int_get_event_reward_points,
        "BERewardInt_GetEventRewardPoints",
    ),
    (
        0x001520e8,
        be_validate_exclusive_bolts,
        "BE_ValidateExclusiveBolts",
    ),
    (
        0x001521f0,
        be_ensure_default_bolts_equipped,
        "BE_EnsureDefaultBoltsEquipped",
    ),
    (
        0x00152758,
        be_save_int_build_system_save,
        "BESaveInt_BuildSystemSave",
    ),
    (
        0x00152950,
        be_save_int_build_profile_save,
        "BESaveInt_BuildProfileSave",
    ),
    (
        0x00152bb0,
        be_save_int_load_system_save,
        "BESaveInt_LoadSystemSave",
    ),
];

/// `void BEBAGTInt_ResetProfile(char profile)`: copies the 10 per-character
/// trick-variant records (0x1fe bytes each) of a saved profile (`0x4a788e + profile*0x9b50`)
/// into the live table `D_00530EC0` (`0x530ec0 + profile*0x13ec`). Leaf; the doubleword
/// copy takes the unaligned (ldl/ldr) form for every record on the real addresses.
pub fn be_bagt_int_reset_profile(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x13ec;
    c.g[V1][0] = 0x9b50;
    c.mult(0, 7, 4, 2);
    c.mult(0, 6, 4, 3);
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x4a0000;
    c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xec0));
    c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x788e));
    c.g[A1][0] = 0u64;
    c.g[T1][0] = 0x1fe;
    c.g[T0][0] = 0xf88;
    c.mult(0, 3, 5, 9);
    'l_14fc70: loop {
        'b_14fd20: {
            'b_14fcec: {
                c.cycles += 9;
                c.mult(0, 4, 5, 8);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if t {
                    break 'b_14fcec;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
                    'l_14fc98: loop {
                        c.cycles += 19;
                        c.ldl(12, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(12, (c.g[V1][0] as u32));
                        c.ldl(13, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(13, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(14, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(14, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(15, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(15, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(12, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(12, (c.g[A0][0] as u32));
                        c.sdl(13, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(13, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(14, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(14, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(15, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(15, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[A1][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014fc98);
                            }
                            continue 'l_14fc98;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_14fd20;
                            } else {
                                break 'b_14fcec;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
            'l_14fcf0: loop {
                c.cycles += 11;
                c.g[T4][0] = c.ld((c.g[V1][0] as u32));
                c.g[T5][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T7][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A0][0] as u32), c.g[T4][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T6][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T7][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[A1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014fcf0);
                    }
                    continue 'l_14fcf0;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    break 'b_14fd20;
                }
            }
        }
        c.cycles += 21;
        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[V1][0] as u32));
        c.ldl(12, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.ldr(12, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.ldl(13, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.ldr(13, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.lwl(14, (c.g[V1][0] as u32).wrapping_add(0x1b));
        c.lwr(14, (c.g[V1][0] as u32).wrapping_add(0x18));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[A0][0] as u32));
        c.sdl(12, (c.g[A0][0] as u32).wrapping_add(0xf));
        c.sdr(12, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.sdl(13, (c.g[A0][0] as u32).wrapping_add(0x17));
        c.sdr(13, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.swl(14, (c.g[A0][0] as u32).wrapping_add(0x1b));
        c.swr(14, (c.g[A0][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.sh((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.mult(0, 3, 5, 9);
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0014fc70);
            }
            continue 'l_14fc70;
        } else {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.pc = t;
            return;
        }
    }
}

/// `int BEBAGTInt_GetTrickAnim2(self, int rider, int idx, int second)`: the second
/// animation id of grab trick `idx`: fixed ids (210/209/212/211) for the cheat characters
/// 20/24/25/28 on tricks 4/4/9/4, else the grab table entry the rider's uber variant picks
/// (`kGrabDB[idx].anims[k].anim2`, 438 when there is no variant).
pub fn be_bagt_int_get_trick_anim2(c: &mut Cpu) {
    guest(c, |c| {
        'b_1503d8: {
            'b_150398: {
                'b_150394: {
                    'b_150390: {
                        c.cycles += 15;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S1][0] = c.g[A1][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S2][0] = c.g[A0][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S0][0] = c.g[A2][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[S3][0] = c.g[A3][0];
                        c.g[RA][0] = sx(0x001502f8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0014a0b0,
                            f: ee::s01::BE_GetRiderAltCharacter,
                        }
                        .call(c, 0x001502f8)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = 0x18;
                        if t {
                            break 'b_150390;
                        } else {
                            'b_15037c: {
                                'b_150368: {
                                    'b_150354: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 25i64) as u64;
                                        if t {
                                            break 'b_150354;
                                        } else {
                                            'b_150340: {
                                                'b_150324: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x14;
                                                    if t {
                                                        break 'b_150324;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_150340;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_150398;
                                                            } else {
                                                                break 'b_150324;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x19;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x1c;
                                                if t {
                                                    break 'b_150368;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];
                                                    c.g[A0][0] = c.g[S2][0];
                                                    if t {
                                                        break 'b_15037c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_150398;
                                                        } else {
                                                            break 'b_150340;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x4;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            if t {
                                                break 'b_150398;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = 0xd2;
                                                if t {
                                                    break 'b_1503d8;
                                                } else {
                                                    break 'b_150354;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x4;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_150394;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0xd1;
                                        if t {
                                            break 'b_1503d8;
                                        } else {
                                            break 'b_150368;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x9;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_150394;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0xd4;
                                    if t {
                                        break 'b_1503d8;
                                    } else {
                                        break 'b_15037c;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x4;
                            let t = c.g[S0][0] != c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            if t {
                                break 'b_150398;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0xd3;
                                if t {
                                    break 'b_1503d8;
                                } else {
                                    break 'b_150390;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S2][0];
                    break 'b_150394;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[S1][0];
                break 'b_150398;
            }
            'b_1503d4: {
                c.cycles += 6;
                c.g[A3][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001503a4);
                c.g[A2][0] = c.g[S0][0];
                Callee {
                    addr: 0x0014fea8,
                    f: ee::s01::BEBAGTInt_GetUberTrickSlotValue,
                }
                .call(c, 0x001503a4)?;
                c.g[V1][0] = c.g[V0][0];
                let t = (c.g[V1][0] as i64) < 0;
                c.g[A1][0] = 0x14;
                if t {
                    break 'b_1503d4;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = 0x460000;
                    c.mult(0, 5, 16, 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5148));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    let t = true;
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                    if t {
                        break 'b_1503d8;
                    } else {
                        break 'b_1503d4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1b6;
            break 'b_1503d8;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int BEBAGTInt_GetTrickValue(self, int rider, int idx, int second)`: like GetTrickAnim2 but
/// for the trick value: fixed 82/81/84/83 for the cheat characters, else
/// `kGrabDB[idx].anims[k].anim3` (0 when there is no variant).
pub fn be_bagt_int_get_trick_value(c: &mut Cpu) {
    guest(c, |c| {
        'b_150508: {
            'b_1504c8: {
                'b_1504c4: {
                    'b_1504c0: {
                        c.cycles += 15;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S1][0] = c.g[A1][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S2][0] = c.g[A0][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S0][0] = c.g[A2][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[S3][0] = c.g[A3][0];
                        c.g[RA][0] = sx(0x00150428);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0014a0b0,
                            f: ee::s01::BE_GetRiderAltCharacter,
                        }
                        .call(c, 0x00150428)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = 0x18;
                        if t {
                            break 'b_1504c0;
                        } else {
                            'b_1504ac: {
                                'b_150498: {
                                    'b_150484: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 25i64) as u64;
                                        if t {
                                            break 'b_150484;
                                        } else {
                                            'b_150470: {
                                                'b_150454: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x14;
                                                    if t {
                                                        break 'b_150454;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_150470;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_1504c8;
                                                            } else {
                                                                break 'b_150454;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x19;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x1c;
                                                if t {
                                                    break 'b_150498;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];
                                                    c.g[A0][0] = c.g[S2][0];
                                                    if t {
                                                        break 'b_1504ac;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_1504c8;
                                                        } else {
                                                            break 'b_150470;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x4;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            if t {
                                                break 'b_1504c8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = 0x52;
                                                if t {
                                                    break 'b_150508;
                                                } else {
                                                    break 'b_150484;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x4;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_1504c4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x51;
                                        if t {
                                            break 'b_150508;
                                        } else {
                                            break 'b_150498;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x9;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1504c4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0x54;
                                    if t {
                                        break 'b_150508;
                                    } else {
                                        break 'b_1504ac;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x4;
                            let t = c.g[S0][0] != c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            if t {
                                break 'b_1504c8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x53;
                                if t {
                                    break 'b_150508;
                                } else {
                                    break 'b_1504c0;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S2][0];
                    break 'b_1504c4;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[S1][0];
                break 'b_1504c8;
            }
            'b_150504: {
                c.cycles += 6;
                c.g[A3][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001504d4);
                c.g[A2][0] = c.g[S0][0];
                Callee {
                    addr: 0x0014fea8,
                    f: ee::s01::BEBAGTInt_GetUberTrickSlotValue,
                }
                .call(c, 0x001504d4)?;
                c.g[V1][0] = c.g[V0][0];
                let t = (c.g[V1][0] as i64) < 0;
                c.g[A1][0] = 0x14;
                if t {
                    break 'b_150504;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = 0x460000;
                    c.mult(0, 5, 16, 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5148));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    let t = true;
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_150508;
                    } else {
                        break 'b_150504;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_150508;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int BEEconInt_BuyAttribute(self, int human, int character, int attribute)`: buys one level
/// of a stat (speed, accel, trick, edging, spin, toughness, stability) if the character has
/// the money (`GetAttributeCost(level/5 - 1)`) and the stat is not at 55: deducts the money,
/// raises the stat byte and notifies the stat interface (virtual call). Returns 1 on a
/// purchase, else 0. Resumable (jump table): any other `pc` resumes in the generated body.
pub fn be_econ_int_buy_attribute(c: &mut Cpu) {
    if c.pc != 0x00150c20 {
        // resume after an unwound callee or event: the generated body (at install: `__generated`)
        return ee::s01::f_00150c20__generated(c);
    }
    guest(c, |c| {
        'b_150e30: {
            'b_150de4: {
                'b_150ddc: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.g[T1][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_150ddc;
                    } else {
                        'b_150d98: {
                            c.cycles += 13;
                            c.g[V0][0] = 0xf88;
                            c.g[A0][0] = 0x9b50;
                            c.mult(0, 2, 18, 2);
                            c.mult(0, 4, 17, 4);
                            c.g[V1][0] = 0x4a0000;
                            c.g[S0][0] = 0u64;
                            c.g[S3][0] = c.g[V1][0];
                            c.g[A1][0] = (c.g[A3][0] < 0x7u64) as u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            let t = c.g[A1][0] == 0u64;
                            c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac4));
                            if t {
                                break 'b_150d98;
                            } else {
                                'b_150d78: {
                                    'b_150d54: {
                                        'b_150d30: {
                                            'b_150d0c: {
                                                'b_150ce8: {
                                                    'b_150cc4: {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = 0x460000;
                                                        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_sub(0x58d0));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x00150ca0 => {
                                                                c.cycles += 9;
                                                                c.g[V0][0] = 0xf88;
                                                                c.g[V1][0] = 0x9b50;
                                                                c.mult(0, 2, 18, 2);
                                                                c.mult(0, 3, 17, 3);
                                                                c.g[A0][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6ca8));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                let t = true;
                                                                c.g[S0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0xbdf));
                                                                if t {
                                                                    break 'b_150d98;
                                                                } else {
                                                                    break 'b_150cc4;
                                                                }
                                                            }
                                                            0x00150cc4 => {
                                                                break 'b_150cc4;
                                                            }
                                                            0x00150ce8 => {
                                                                break 'b_150ce8;
                                                            }
                                                            0x00150d0c => {
                                                                break 'b_150d0c;
                                                            }
                                                            0x00150d30 => {
                                                                break 'b_150d30;
                                                            }
                                                            0x00150d54 => {
                                                                break 'b_150d54;
                                                            }
                                                            0x00150d78 => {
                                                                break 'b_150d78;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x00150c20, 0x00150e50,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 9;
                                                    c.g[V0][0] = 0xf88;
                                                    c.g[V1][0] = 0x9b50;
                                                    c.mult(0, 2, 18, 2);
                                                    c.mult(0, 3, 17, 3);
                                                    c.g[A0][0] =
                                                        sx((c.g[S3][0] as u32)
                                                            .wrapping_add(0x6ca8));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    let t = true;
                                                    c.g[S0][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0xbe0));
                                                    if t {
                                                        break 'b_150d98;
                                                    } else {
                                                        break 'b_150ce8;
                                                    }
                                                }
                                                c.cycles += 9;
                                                c.g[V0][0] = 0xf88;
                                                c.g[V1][0] = 0x9b50;
                                                c.mult(0, 2, 18, 2);
                                                c.mult(0, 3, 17, 3);
                                                c.g[A0][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                let t = true;
                                                c.g[S0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0xbe1));
                                                if t {
                                                    break 'b_150d98;
                                                } else {
                                                    break 'b_150d0c;
                                                }
                                            }
                                            c.cycles += 9;
                                            c.g[V0][0] = 0xf88;
                                            c.g[V1][0] = 0x9b50;
                                            c.mult(0, 2, 18, 2);
                                            c.mult(0, 3, 17, 3);
                                            c.g[A0][0] =
                                                sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            let t = true;
                                            c.g[S0][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0xbe2));
                                            if t {
                                                break 'b_150d98;
                                            } else {
                                                break 'b_150d30;
                                            }
                                        }
                                        c.cycles += 9;
                                        c.g[V0][0] = 0xf88;
                                        c.g[V1][0] = 0x9b50;
                                        c.mult(0, 2, 18, 2);
                                        c.mult(0, 3, 17, 3);
                                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        let t = true;
                                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe3));
                                        if t {
                                            break 'b_150d98;
                                        } else {
                                            break 'b_150d54;
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[V0][0] = 0xf88;
                                    c.g[V1][0] = 0x9b50;
                                    c.mult(0, 2, 18, 2);
                                    c.mult(0, 3, 17, 3);
                                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    let t = true;
                                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe4));
                                    if t {
                                        break 'b_150d98;
                                    } else {
                                        break 'b_150d78;
                                    }
                                }
                                c.cycles += 8;
                                c.g[V0][0] = 0xf88;
                                c.g[V1][0] = 0x9b50;
                                c.mult(0, 2, 18, 2);
                                c.mult(0, 3, 17, 3);
                                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe5));
                                break 'b_150d98;
                            }
                        }
                        'b_150da8: {
                            c.cycles += 4;
                            c.g[A1][0] = c.lb((c.g[S0][0] as u32));
                            c.g[V0][0] = 0x5;
                            if c.g[V0][0] == 0u64 {
                                break 'b_150da8;
                            } else {
                                break 'b_150da8;
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[T1][0];
                        c.div(0, 5, 2);
                        c.g[A1][0] = c.lo[0];
                        c.g[RA][0] = sx(0x00150dbc);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        Callee {
                            addr: 0x00150e50,
                            f: ee::s01::BEEconInt_GetAttributeCost,
                        }
                        .call(c, 0x00150dbc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_150e30;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lb((c.g[S0][0] as u32));
                            c.g[V0][0] = 0x37;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = 0xf88;
                            if t {
                                break 'b_150de4;
                            } else {
                                break 'b_150ddc;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_150e30;
                } else {
                    break 'b_150de4;
                }
            }
            c.cycles += 19;
            c.g[V0][0] = 0x9b50;
            c.mult(0, 2, 17, 2);
            c.mult(0, 4, 18, 4);
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xac4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
            c.sw((c.g[A0][0] as u32).wrapping_add(0xac4), c.g[V0][0]);
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[RA][0] = sx(0x00150e18);
            c.sb((c.g[S0][0] as u32), c.g[V1][0]);
            Callee {
                addr: 0x00147f78,
                f: ee::s01::BECharInt_GetInstance,
            }
            .call(c, 0x00150e18)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x00150e2c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00150e2c)?;
            c.g[V0][0] = 0x1;
            break 'b_150e30;
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `int BERewardInt_GetEventRewardPoints(self, int medal, int key, int place)`: the prize
/// money of an event: for places 6..11 the race interface's special event score, else the
/// per-medal prize of the event `key` in the table at 0x4405a0 (times 100).
pub fn be_reward_int_get_event_reward_points(c: &mut Cpu) {
    guest(c, |c| {
        'b_151164: {
            'b_151160: {
                'b_15115c: {
                    'b_1510e0: {
                        c.cycles += 17;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S0][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S1][0] = c.g[A3][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[RA][0] = sx(0x00151064);
                        c.g[S2][0] = c.g[A2][0];
                        Callee {
                            addr: 0x0014dc80,
                            f: ee::s01::BELibrary_GetInstance,
                        }
                        .call(c, 0x00151064)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00151070);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x0014dd58,
                            f: ee::s01::BELibrary_GetInterface,
                        }
                        .call(c, 0x00151070)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6));
                        c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1510e0;
                        } else {
                            'b_1510d0: {
                                'b_1510c8: {
                                    'b_1510bc: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                                        if t {
                                            break 'b_1510bc;
                                        } else {
                                            'b_1510a8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1510a8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1510c8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = c.g[A1][0];
                                                        if t {
                                                            break 'b_151164;
                                                        } else {
                                                            break 'b_1510a8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = (c.g[S0][0] as i64) < 0;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_151160;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A1][0] = c.g[S1][0];
                                                let t = true;
                                                c.g[A2][0] = 0x2;
                                                if t {
                                                    break 'b_1510d0;
                                                } else {
                                                    break 'b_1510bc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A1][0] = c.g[S1][0];
                                    let t = true;
                                    c.g[A2][0] = 0x1;
                                    if t {
                                        break 'b_1510d0;
                                    } else {
                                        break 'b_1510c8;
                                    }
                                }
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                break 'b_1510d0;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001510d8);
                            Callee {
                                addr: 0x00145630,
                                f: ee::s01::BENewRaceInt_GetSpecialEventScore,
                            }
                            .call(c, 0x001510d8)?;
                            let t = true;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_15115c;
                            } else {
                                break 'b_1510e0;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = 0x440000;
                    c.g[T2][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a0));
                    c.g[A2][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x154));
                    c.g[T0][0] = 0x2;
                    c.g[A3][0] = 0x3;
                    c.g[A0][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    'l_151108: loop {
                        'b_151150: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[S2][0] {
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                break 'b_151150;
                            } else {
                                'b_151148: {
                                    c.cycles += 2;
                                    if c.g[S0][0] == c.g[T2][0] {
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                        break 'b_151148;
                                    } else {
                                        'b_151130: {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == 0u64;

                                            if t {
                                                break 'b_151130;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[S0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                                    break 'b_151148;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                                    if t {
                                                        break 'b_151150;
                                                    } else {
                                                        break 'b_151130;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] == c.g[T0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            break 'b_151148;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S0][0] == c.g[A3][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                                break 'b_151148;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_151150;
                                                } else {
                                                    break 'b_151148;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.mult(0, 5, 2, 4);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                break 'b_151150;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T1][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                            if event_due(c) {
                                deliver_event_here(c, 0x00151108);
                            }
                            continue 'l_151108;
                        } else {
                            break 'b_15115c;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_151160;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A1][0];
            break 'b_151164;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `int BE_ValidateExclusiveBolts(profile, int* changed)`: walks the character's bolt
/// definitions; every exclusive one (flag 0x1000) whose owned item has flag 0x10 is
/// re-applied through `BE_ApplyBoltDependencies`. Sets `*changed` and returns 0 as soon as
/// a re-apply fails, else 1.
pub fn be_validate_exclusive_bolts(c: &mut Cpu) {
    guest(c, |c| {
        'b_1521c8: {
            'b_1521c4: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S4][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x00152118);
                c.g[S2][0] = 0u64;
                Callee {
                    addr: 0x0014bdb8,
                    f: ee::s01::Bolts_GetDB,
                }
                .call(c, 0x00152118)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xbc0));
                c.g[RA][0] = sx(0x00152128);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0014d988,
                    f: ee::s01::Bolts_GetCharBoltCount,
                }
                .call(c, 0x00152128)?;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xbc0));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00152138);
                c.g[S3][0] = c.g[V0][0];
                Callee {
                    addr: 0x0014d998,
                    f: ee::s01::Bolts_GetCharBolts,
                }
                .call(c, 0x00152138)?;
                c.g[S0][0] = c.g[V0][0];
                let t = (c.g[S3][0] as i64) <= 0;
                c.sw((c.g[S4][0] as u32), 0u64);
                if t {
                    break 'b_1521c4;
                } else {
                    c.cycles += 1;
                    c.g[S5][0] = 0x1;
                    'l_152148: loop {
                        'b_1521b8: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V0][0] & 0x1000;
                            if c.g[V0][0] == 0u64 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_1521b8;
                            } else {
                                'b_152184: {
                                    'b_152180: {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_152180;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            let t = true;
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            if t {
                                                break 'b_152184;
                                            } else {
                                                break 'b_152180;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                    break 'b_152184;
                                }
                                'b_1521b4: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] & 0x10;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_1521b4;
                                    } else {
                                        c.cycles += 6;
                                        c.sw((c.g[S4][0] as u32), c.g[S5][0]);
                                        c.g[A2][0] = 0x1;
                                        c.g[RA][0] = sx(0x001521a4);
                                        c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        Callee {
                                            addr: 0x00151ef0,
                                            f: ee::s01::BE_ApplyBoltDependencies,
                                        }
                                        .call(c, 0x001521a4)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1521b8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1521c8;
                                            } else {
                                                break 'b_1521b4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_1521b8;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00152148);
                            }
                            continue 'l_152148;
                        } else {
                            break 'b_1521c4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_1521c8;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `int BE_EnsureDefaultBoltsEquipped(profile)`: for each equipment group of the character,
/// if none of its bolts is equipped, equips the group's default bolt. Returns 0 as soon as
/// one equip fails, else 1.
pub fn be_ensure_default_bolts_equipped(c: &mut Cpu) {
    guest(c, |c| {
        'b_152318: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8b0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
            c.g[S2][0] = 0u64;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x850));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
            c.g[RA][0] = sx(0x00152220);
            c.g[S5][0] = 0x1;
            Callee {
                addr: 0x0014bdb8,
                f: ee::s01::Bolts_GetDB,
            }
            .call(c, 0x00152220)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xbc0));
            c.g[RA][0] = sx(0x00152230);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x0014dc50,
                f: ee::s01::Bolts_GetCharTable4,
            }
            .call(c, 0x00152230)?;
            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xbc0));
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00152240);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x0014dc40,
                f: ee::s01::Bolts_GetCharTable4Count,
            }
            .call(c, 0x00152240)?;
            c.g[S4][0] = c.g[V0][0];
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[V0][0] = c.g[S5][0];
            if t {
                break 'b_152318;
            } else {
                c.cycles += 1;
                'l_152250: loop {
                    'b_1522e0: {
                        c.cycles += 10;
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xbc0));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A3][0] = c.g[SP][0];
                        c.g[T0][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0015226c);
                        c.g[T1][0] = 0x1;
                        Callee {
                            addr: 0x0014d608,
                            f: ee::s01::Bolts_CollectChildren,
                        }
                        .call(c, 0x0015226c)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = (c.g[A2][0] as i64) <= 0;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1522e0;
                        } else {
                            c.cycles += 2;
                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
                            c.g[A0][0] = c.g[SP][0];
                            'l_152280: loop {
                                'b_1522ac: {
                                    'b_1522a8: {
                                        c.cycles += 7;
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                        c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_1522a8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            let t = true;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            if t {
                                                break 'b_1522ac;
                                            } else {
                                                break 'b_1522a8;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                    break 'b_1522ac;
                                }
                                'b_1522d4: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] & 0x10;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        break 'b_1522d4;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                        c.g[V0][0] = c.g[V0][0] & 0x10;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1522e0;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_1522d4;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00152280);
                                    }
                                    continue 'l_152280;
                                } else {
                                    break 'b_1522e0;
                                }
                            }
                        }
                    }
                    'b_152314: {
                        'b_152308: {
                            c.cycles += 2;
                            if c.g[A1][0] != c.g[A2][0] {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_152308;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001522f8);
                                c.g[A2][0] = 0x1;
                                Callee {
                                    addr: 0x00151c90,
                                    f: ee::s01::BE_SetCharProfileBoltEquipped,
                                }
                                .call(c, 0x001522f8)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_152308;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S5][0] = 0u64;
                                    if t {
                                        break 'b_152314;
                                    } else {
                                        break 'b_152308;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00152250);
                            }
                            continue 'l_152250;
                        } else {
                            break 'b_152314;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S5][0];
                    break 'b_152318;
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x850));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.pc = t;
        return Ok(());
    });
}

/// `void* BESaveInt_BuildSystemSave(self)`: allocates the system save buffer, copies the option
/// DB (0x288 bytes), the high scores (0xa28 bytes), packs the controller bits of the six
/// players and appends the CRC16. Returns the buffer (also in `gp-0x1ed0`).
pub fn be_save_int_build_system_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_152858: {
            'b_152818: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[RA][0] = sx(0x00152770);
                c.g[S0][0] = c.g[A0][0];
                Callee {
                    addr: 0x00152948,
                    f: ee::s01::BESaveInt_GetSystemSaveSize,
                }
                .call(c, 0x00152770)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0015277c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00152700,
                    f: ee::s01::BESaveInt_FreeSaveBuffer,
                }
                .call(c, 0x0015277c)?;
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = c.g[S1][0];
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5810));
                c.g[RA][0] = sx(0x00152794);
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x00317e30, 0x00152794)?;
                c.g[A0][0] = 0x530000;
                c.g[S0][0] = c.g[V0][0];
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5898));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ed0), c.g[V0][0]);
                c.g[V1][0] = c.g[S0][0] | c.g[T0][0];
                c.g[V1][0] = c.g[V1][0] & 0x7;
                let t = c.g[V1][0] == 0u64;
                c.g[A3][0] = c.g[S0][0];
                if t {
                    break 'b_152818;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x280));
                    c.g[A1][0] = 0x530000;
                    'l_1527c4: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                        c.ldr(3, (c.g[T0][0] as u32));
                        c.ldl(4, (c.g[T0][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0x17));
                        c.ldr(9, (c.g[T0][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[T0][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[T0][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.sdr(3, (c.g[A3][0] as u32));
                        c.sdl(4, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.sdr(4, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(9, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.sdr(9, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                        let t = c.g[T0][0] != c.g[A2][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001527c4);
                            }
                            continue 'l_1527c4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S0][0] = c.g[V0][0];
                            if t {
                                break 'b_152858;
                            } else {
                                break 'b_152818;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
            c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x280));
            c.g[A1][0] = 0x530000;
            'l_152828: loop {
                c.cycles += 11;
                c.g[V1][0] = c.ld((c.g[T0][0] as u32));
                c.g[A0][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[T1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[T2][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                let t = c.g[T0][0] != c.g[A2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00152828);
                    }
                    continue 'l_152828;
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[V0][0];
                    break 'b_152858;
                }
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5c18));
        c.ldl(2, (c.g[T0][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[T0][0] as u32));
        c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.g[A0][0] = c.g[S0][0];
        c.sdr(2, (c.g[A3][0] as u32));
        c.g[RA][0] = sx(0x00152878);
        c.g[A2][0] = 0xa28;
        call_ptr(c, 0x0041605c, 0x00152878)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa28));
        c.g[V0][0] = 0x530000;
        c.g[T1][0] = 0xfffffffffffffffe;
        c.g[T0][0] = 0xfffffffffffffe01;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
        c.g[A3][0] = 0x5;
        'l_152898: loop {
            c.cycles += 15;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[A0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x13));
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V1][0] = c.g[V1][0] & c.g[T1][0];
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
            c.sw((c.g[A1][0] as u32), c.g[V1][0]);
            let t = (c.g[A3][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00152898);
                }
                continue 'l_152898;
            } else {
                c.cycles += 28;
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[SP][0] as u32));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[S0][0] as u32));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x17));
                c.g[A2][0] = 0xfbea;
                c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00152918);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x003e62d0, 0x00152918)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.lwl(9, (c.g[SP][0] as u32).wrapping_add(0x23));
                c.lwr(9, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swl(9, (c.g[S0][0] as u32).wrapping_add(0x3));
                c.swr(9, (c.g[S0][0] as u32));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void* BESaveInt_BuildProfileSave(self, int idx)`: allocates the profile save buffer, copies
/// the user name, the player DB record (0x9b40 bytes) and one profile byte, lets the five
/// interfaces (new player, BAGT, mission, score, stat) write their parts through their
/// virtual `discard` and appends the CRC16. Returns the buffer.
pub fn be_save_int_build_profile_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_152a88: {
            'b_152a48: {
                c.cycles += 37;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x00152970);
                c.g[S2][0] = c.g[A1][0];
                Callee {
                    addr: 0x00152ba8,
                    f: ee::s01::BESaveInt_GetProfileSaveSize,
                }
                .call(c, 0x00152970)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0015297c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00152700,
                    f: ee::s01::BESaveInt_FreeSaveBuffer,
                }
                .call(c, 0x0015297c)?;
                c.g[A1][0] = 0x460000;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5810));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00152994);
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x00317e30, 0x00152994)?;
                c.g[V1][0] = 0x1c;
                c.g[A1][0] = 0x530000;
                c.mult(0, 3, 18, 3);
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b20));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0xc;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ed0), c.g[V0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[RA][0] = sx(0x001529c0);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, 0x00416ce0, 0x001529c0)?;
                c.g[A0][0] = 0x9b50;
                c.g[V1][0] = 0x4a0000;
                c.mult(0, 4, 18, 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.g[A2][0] | c.g[S0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[S0][0];
                if t {
                    break 'b_152a48;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = 0x9b40;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = 0x530000;
                    'l_1529f4: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(3, (c.g[A2][0] as u32));
                        c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.sdr(3, (c.g[A3][0] as u32));
                        c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[V0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001529f4);
                            }
                            continue 'l_1529f4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V1][0] = 0x1c;
                            if t {
                                break 'b_152a88;
                            } else {
                                break 'b_152a48;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 0x9b40;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = 0x530000;
            'l_152a58: loop {
                c.cycles += 11;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32));
                c.g[A1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00152a58);
                    }
                    continue 'l_152a58;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = 0x1c;
                    break 'b_152a88;
                }
            }
        }
        c.cycles += 71;
        c.g[V0][0] = 0x9b50;
        c.mult(0, 3, 18, 3);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[A2][0] as u32));
        c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
        c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[A3][0] as u32));
        c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b31));
        c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
        c.g[RA][0] = sx(0x00152ac8);
        c.sb((c.g[S0][0] as u32), c.g[V0][0]);
        Callee {
            addr: 0x00145970,
            f: ee::s01::BENewPlayerInt_GetInstance,
        }
        .call(c, 0x00152ac8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152ae4);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00152ae4)?;
        c.g[RA][0] = sx(0x00152aec);
        Callee {
            addr: 0x0014f900,
            f: ee::s01::BEBAGTInt_GetInstance,
        }
        .call(c, 0x00152aec)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b04);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00152b04)?;
        c.g[RA][0] = sx(0x00152b0c);
        Callee {
            addr: 0x001532a0,
            f: ee::s01::BEMissionInt_GetInstance,
        }
        .call(c, 0x00152b0c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b24);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00152b24)?;
        c.g[RA][0] = sx(0x00152b2c);
        Callee {
            addr: 0x00154948,
            f: ee::s01::BEScoreInt_GetInstance,
        }
        .call(c, 0x00152b2c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b44);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00152b44)?;
        c.g[RA][0] = sx(0x00152b4c);
        Callee {
            addr: 0x00147f78,
            f: ee::s01::BECharInt_GetInstance,
        }
        .call(c, 0x00152b4c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b64);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00152b64)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00152b74);
        c.g[A2][0] = 0xfbea;
        call_ptr(c, 0x003e62d0, 0x00152b74)?;
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
        c.lwl(3, (c.g[SP][0] as u32).wrapping_add(0x3));
        c.lwr(3, (c.g[SP][0] as u32));
        c.swl(3, (c.g[S0][0] as u32).wrapping_add(0x3));
        c.swr(3, (c.g[S0][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int BESaveInt_LoadSystemSave(self, void* buf)`: validates the CRC16 of a system save and
/// loads it: option DB, high scores, controller bits, with the `onLoad` calls of the option,
/// score and new-player interfaces and the locale/display refresh. Returns 0 on a bad CRC,
/// else 1.
pub fn be_save_int_load_system_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_152dc8: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[RA][0] = sx(0x00152bcc);
            c.g[S1][0] = c.g[A1][0];
            Callee {
                addr: 0x00152948,
                f: ee::s01::BESaveInt_GetSystemSaveSize,
            }
            .call(c, 0x00152bcc)?;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00152be0);
            c.g[A2][0] = 0xfbea;
            call_ptr(c, 0x003e62d0, 0x00152be0)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.lwl(3, (c.g[S0][0] as u32).wrapping_add(0x3));
            c.lwr(3, (c.g[S0][0] as u32));
            c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x23));
            c.swr(3, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0u64;
            if t {
                break 'b_152dc8;
            } else {
                'b_152cb8: {
                    'b_152c80: {
                        c.cycles += 7;
                        c.g[V0][0] = 0x530000;
                        c.g[S0][0] = c.g[S1][0];
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5898));
                        c.g[V0][0] = c.g[S0][0] | c.g[A2][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.g[S0][0];
                        if t {
                            break 'b_152c80;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                            c.g[S2][0] = 0x530000;
                            'l_152c2c: loop {
                                c.cycles += 19;
                                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.ldr(7, (c.g[V1][0] as u32));
                                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                                c.sdr(7, (c.g[A2][0] as u32));
                                c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0xf));
                                c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
                                c.sdr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x1f));
                                c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00152c2c);
                                    }
                                    continue 'l_152c2c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_152cb8;
                                    } else {
                                        break 'b_152c80;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                    c.g[S2][0] = 0x530000;
                    'l_152c8c: loop {
                        c.cycles += 11;
                        c.g[A3][0] = c.ld((c.g[V1][0] as u32));
                        c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sd((c.g[A2][0] as u32), c.g[A3][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00152c8c);
                            }
                            continue 'l_152c8c;
                        } else {
                            break 'b_152cb8;
                        }
                    }
                }
                c.cycles += 42;
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[V1][0] as u32));
                c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.g[S0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00152cd0);
                c.sdr(7, (c.g[A2][0] as u32));
                Callee {
                    addr: 0x0014f250,
                    f: ee::s01::BEOptionInt_GetInstance,
                }
                .call(c, 0x00152cd0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00152ce8);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00152ce8)?;
                c.g[A2][0] = 0xa28;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5c18));
                c.g[RA][0] = sx(0x00152cfc);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa28));
                call_ptr(c, 0x0041605c, 0x00152cfc)?;
                c.g[RA][0] = sx(0x00152d04);
                Callee {
                    addr: 0x00154948,
                    f: ee::s01::BEScoreInt_GetInstance,
                }
                .call(c, 0x00152d04)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00152d1c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00152d1c)?;
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[S0][0] as u32));
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x17));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[SP][0] as u32));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.g[V0][0] = 0x530000;
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = 0xfffffffffffffffd;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0x5;
                'l_152d60: loop {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[A0][0] & 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sb((c.g[A1][0] as u32).wrapping_add(0x13), c.g[A0][0]);
                    let t = (c.g[A3][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00152d60);
                        }
                        continue 'l_152d60;
                    } else {
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x00152d9c);
                        Callee {
                            addr: 0x00145970,
                            f: ee::s01::BENewPlayerInt_GetInstance,
                        }
                        .call(c, 0x00152d9c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00152db4);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00152db4)?;
                        c.g[RA][0] = sx(0x00152dbc);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        call_ptr(c, 0x00228a78, 0x00152dbc)?;
                        c.g[RA][0] = sx(0x00152dc4);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        call_ptr(c, 0x00228c08, 0x00152dc4)?;
                        c.g[V0][0] = 0x1;
                        break 'b_152dc8;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (outside the fakes' call log).
    const OC676_CTRL: u32 = DATA + 0x20_0000;
    const OC676_MAGIC: u32 = 0x0c67_6c67;
    /// `Bolts_GetDB`: the bolt database in the game's data.
    const OC676_BOLTS_DB: u32 = 0x004a_6750;

    fn oc676_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC676_CTRL) == OC676_MAGIC
    }

    macro_rules! oc676_fake {
        ($name:ident, $addr:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if !oc676_gated(c) {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }
    oc676_fake!(
        oc676_fake_new_named,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc676_fake!(
        oc676_fake_memcpy,
        0x0041_605c,
        ee::s15::f_0041605c__generated
    );
    oc676_fake!(
        oc676_fake_strncpy,
        0x0041_6ce0,
        ee::s15::f_00416ce0__generated
    );
    oc676_fake!(
        oc676_fake_load_locale,
        0x0022_8a78,
        ee::s05::f_00228a78__generated
    );
    oc676_fake!(
        oc676_fake_apply_display,
        0x0022_8c08,
        ee::s05::f_00228c08__generated
    );

    /// Hooks the gated fakes of the save functions' callees (once per process).
    fn oc676_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc676_fake_new_named,
            fakes::prev_cell(oc676_fake_new_named),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0041605C),
            oc676_fake_memcpy,
            fakes::prev_cell(oc676_fake_memcpy),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416CE0),
            oc676_fake_strncpy,
            fakes::prev_cell(oc676_fake_strncpy),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00228A78),
            oc676_fake_load_locale,
            fakes::prev_cell(oc676_fake_load_locale),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00228C08),
            oc676_fake_apply_display,
            fakes::prev_cell(oc676_fake_apply_display),
        );
    }

    /// Opens the gate on this machine.
    fn oc676_gate(c: &mut Cpu) {
        oc676_install_fakes();
        c.bus.write32(OC676_CTRL, OC676_MAGIC);
    }

    /// An interface singleton in the gp slot `gp - gp_off`: an object whose
    /// vtable (at +0xc) has `{delta, method}` at +0x10/+0x14, the method being
    /// a fake outside the function table.
    fn oc676_instance(c: &mut Cpu, gp_off: u32, k: u32) {
        let obj = DATA + 0x6_0000 + 0x100 * k;
        let vt = DATA + 0x6_1000 + 0x40 * k;
        c.bus.write32(GAME_GP - gp_off, obj);
        c.bus.write32(obj + 0xc, vt);
        c.bus.write16(vt + 0x10, (k * 4) as u16);
        c.bus.write32(vt + 0x14, fakes::FAKE + 0x10 * k);
    }

    /// `BEBAGTInt_ResetProfile`: all three profiles; random saved records.
    #[test]
    fn be_bagt_int_reset_profile_matches() {
        check(
            ee::s01::f_0014fc40__generated,
            be_bagt_int_reset_profile,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14fc40, 0);
                let p = seed % 3;
                fill(c, &mut r, 0x004a_7880 + p * 0x9b50, 0x2400);
                fill(c, &mut r, 0x0053_0ec0 + p * 0x13ec, 0x800);
                c.g[A0][0] = p as u64;
                c.pc = 0x0014fc40;
            },
        );
    }

    /// The rider tables `BE_GetRiderAltCharacter` & co read, the random
    /// variant bytes, and the grab table from the game image: rider 0 with
    /// alt character `alt`, trick `idx`, `second`, and a valid (or not) profile.
    fn oc676_trick_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 0);
        elf_image(c);
        fill(c, &mut r, 0x0053_0ec0, 0x1800);
        let prof = seed % 3;
        c.bus.write32(0x0053_05b0, prof);
        let rec = 0x0053_5b20 + prof * 0x1c;
        let alt = [0u32, 0x14, 0x18, 0x19, 0x1c, 7][(seed % 6) as usize];
        let idx = [4u32, 9, 3, 4, 9, 10][((seed / 6) % 6) as usize];
        let second = (seed / 36) & 1;
        let valid = (seed / 72) % 3 != 0;
        let word = if valid { r.next_u32() % 4 } else { 0xffff_ffff };
        c.bus.write32(rec + 0xc, word);
        c.bus.write32(rec + 0x10, r.next_u32());
        c.bus.write8(rec + 0x11, (r.next_u32() % 28) as u8);
        c.bus.write8(rec + 0x12, alt as u8);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = 0;
        c.g[A2][0] = idx as u64;
        c.g[A3][0] = second as u64;
        c.pc = entry;
    }

    /// `BEBAGTInt_GetTrickAnim2`: the cheat characters (20/24/25/28) with and
    /// without the matching trick, and the uber-variant lookup (with and
    /// without a variant), real callees on the game's grab table.
    #[test]
    fn be_bagt_int_get_trick_anim2_matches() {
        check(
            ee::s01::f_001502c8__generated,
            be_bagt_int_get_trick_anim2,
            216,
            &|c, seed| oc676_trick_setup(c, seed, 0x1502c8, 0x001502c8),
        );
    }

    /// `BEBAGTInt_GetTrickValue`: same fixture as GetTrickAnim2.
    #[test]
    fn be_bagt_int_get_trick_value_matches() {
        check(
            ee::s01::f_001503f8__generated,
            be_bagt_int_get_trick_value,
            216,
            &|c, seed| oc676_trick_setup(c, seed, 0x1503f8, 0x001503f8),
        );
    }

    /// `BEEconInt_BuyAttribute`: human 0..2 (2 is rejected), the seven stats
    /// and out-of-range ones, levels around the cost table, money below, at
    /// and above the cost; the stat interface's `discard` is a fake.
    #[test]
    fn be_econ_int_buy_attribute_matches() {
        check(
            ee::s01::f_00150c20__generated,
            be_econ_int_buy_attribute,
            324,
            &|c, seed| {
                let mut r = machine(c, seed, 0x150c20, 1);
                elf_image(c);
                oc676_instance(c, 0x1f2c, 0);
                let human = seed % 3;
                let ch = r.next_u32() % 6;
                let attr = r.next_u32() % 9;
                let base = 0x004a_6ca8 + human * 0x9b50 + ch * 0xf88;
                const LEVELS: [u8; 10] = [0, 3, 5, 10, 20, 30, 40, 50, 54, 55];
                for i in 0..7 {
                    let lvl = LEVELS[(r.next_u32() % 10) as usize];
                    c.bus.write8(base + 0xbdf + i, lvl);
                }
                let lvl = c.bus.read8(base + 0xbdf + attr.min(6)) as i32;
                let cost = c.bus.read32((0x0044_0550 + (lvl / 5 - 1) * 8 + 4) as u32);
                let money = match r.next_u32() % 4 {
                    0 => 0,
                    1 => cost,
                    2 => cost.wrapping_sub(1),
                    _ => cost.wrapping_add(5000),
                };
                c.bus.write32(base + 0xac4, money);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = human as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = attr as u64;
                c.pc = 0x00150c20;
            },
        );
    }

    /// `BERewardInt_GetEventRewardPoints`: places 6..11 (the race interface's
    /// score through a fake vtable call) and other places (the prize table of
    /// the game image: a known event key or none), medals -1..5.
    #[test]
    fn be_reward_int_get_event_reward_points_matches() {
        check(
            ee::s01::f_00151040__generated,
            be_reward_int_get_event_reward_points,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x151040, 2);
                elf_image(c);
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x100);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let obj = DATA + 0x6_0000;
                let vt = DATA + 0x6_1000;
                c.bus.write32(0x0053_08a8, obj);
                c.bus.write32(obj + 4, vt);
                c.bus.write16(vt + 8, 0x20);
                c.bus.write32(vt + 0xc, fakes::FAKE);
                let medal = [0u32, 1, 2, 3, 0xffff_ffff, 5][(r.next_u32() % 6) as usize];
                let place = if seed % 2 == 0 {
                    6 + r.next_u32() % 6
                } else {
                    [0u32, 1, 3, 5, 12, 0xffff_ffff][(r.next_u32() % 6) as usize]
                };
                let key = if seed % 5 == 4 {
                    r.next_u32() % 100
                } else {
                    c.bus.read32(0x0044_05a0 + 0x14 * (r.next_u32() % 17))
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = medal as i32 as i64 as u64;
                c.g[A2][0] = key as u64;
                c.g[A3][0] = place as i32 as i64 as u64;
                c.pc = 0x00151040;
            },
        );
    }

    /// Bolt database for character `ch`: `n` definitions from `off`, with ids
    /// 0..7, group 100/101 and flags without bits 1/8 (keeps the real bolt
    /// callees on their short paths), `n4` default-bolt groups from `t4off`.
    fn oc676_bolts_db(c: &mut Cpu, r: &mut Rng, ch: u32, n: u32, off: u32, n4: u32, t4off: u32) {
        let defs = DATA + 0x4_0000;
        let t4 = DATA + 0x5_0000;
        let db = OC676_BOLTS_DB;
        c.bus.write32(db + 4, defs);
        c.bus.write32(db + 0x10, t4);
        c.bus.write32(db + 0x2c + ch * 4, n);
        c.bus.write32(db + 0xa4 + ch * 4, off);
        c.bus.write32(db + 0x3ec + ch * 4, n4);
        c.bus.write32(db + 0x464 + ch * 4, t4off);
        for e in 0..(off + n) {
            let d = defs + e * 0x38;
            c.bus.write16(d + 4, (r.next_u32() % 8) as u16);
            c.bus.write16(d + 6, (100 + r.next_u32() % 2) as u16);
            let flags =
                (r.next_u32() & 0x7ff2) | ((r.next_u32() & 1) << 12) | (r.next_u32() & 0x20);
            c.bus.write32(d + 0x34, flags);
        }
        for e in 0..(t4off + n4) {
            let d = t4 + e * 8;
            c.bus.write16(d + 4, (100 + r.next_u32() % 2) as u16);
            c.bus.write16(d + 6, (r.next_u32() % 8) as u16);
        }
    }

    /// The profile object at `DATA`: character `ch`, bolt map (id -> item) and
    /// the items' flags at +0x290.
    fn oc676_bolt_profile(c: &mut Cpu, r: &mut Rng, ch: u32) {
        let tbl = DATA + 0x3_0000;
        c.bus.write8(DATA + 0xbc0, ch as u8);
        c.bus.write32(DATA + 0x288, tbl);
        for i in 0..16 {
            let v = if r.next_u32() % 4 == 0 {
                0xffff
            } else {
                r.next_u32() % 8
            };
            c.bus.write16(tbl + 2 * i, v as u16);
        }
        for i in 0..8 {
            c.bus.write16(DATA + 0x290 + 4 * i + 2, r.next_u32() as u16);
        }
    }

    /// `BE_ValidateExclusiveBolts`: 0..4 definitions per character, exclusive
    /// flags and owned items at random; `BE_ApplyBoltDependencies` is real
    /// (no dependency links: it succeeds). The failure return needs a failing
    /// recursive equip, which the fixture cannot build.
    #[test]
    fn be_validate_exclusive_bolts_matches() {
        check(
            ee::s01::f_001520e8__generated,
            be_validate_exclusive_bolts,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1520e8, 0);
                let ch = seed % 3;
                oc676_bolt_profile(c, &mut r, ch);
                let n = r.next_u32() % 5;
                let off = r.next_u32() % 3;
                oc676_bolts_db(c, &mut r, ch, n, off, 0, 0);
                c.bus.write32(DATA + 0x500, 0xdead_beef);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.pc = 0x001520e8;
            },
        );
    }

    /// `BE_EnsureDefaultBoltsEquipped`: 0..3 groups per character, 0..5 bolts
    /// per group, owned items at random; the real `BE_SetCharProfileBoltEquipped`
    /// runs on definitions without the bits that recurse. Its failure return
    /// (`ok = 0`) is not reachable from this fixture.
    #[test]
    fn be_ensure_default_bolts_equipped_matches() {
        check(
            ee::s01::f_001521f0__generated,
            be_ensure_default_bolts_equipped,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1521f0, 0);
                let ch = seed % 3;
                oc676_bolt_profile(c, &mut r, ch);
                let n = 1 + r.next_u32() % 5;
                let off = r.next_u32() % 3;
                let n4 = r.next_u32() % 4;
                let t4off = r.next_u32() % 3;
                oc676_bolts_db(c, &mut r, ch, n, off, n4, t4off);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x001521f0;
            },
        );
    }

    /// The buffer the allocator fake hands out (alignment alternates).
    fn oc676_save_buffer(c: &mut Cpu, r: &mut Rng, seed: u32, words: u32) -> u32 {
        let buf = DATA + 0x1_0000 + (seed % 2) * 4;
        c.bus.write32(fakes::LOG_RESULTS, buf);
        fill(c, r, buf, words);
        c.bus.write32(GAME_GP - 0x1ed0, 0);
        buf
    }

    /// `BESaveInt_BuildSystemSave`: both copy forms (aligned and unaligned
    /// buffer); allocator and memcpy are gated fakes, the CRC is real.
    #[test]
    fn be_save_int_build_system_save_matches() {
        covered(
            ee::s01::f_00152758__generated,
            be_save_int_build_system_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x152758, 3);
                oc676_gate(c);
                oc676_save_buffer(c, &mut r, seed, 0x340);
                fill(c, &mut r, 0x0053_5898, 0xd2);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x00152758;
            },
        );
    }

    /// `BESaveInt_BuildProfileSave`: profiles 0..2, both copy forms; the five
    /// interfaces are singletons with a fake `discard`.
    #[test]
    fn be_save_int_build_profile_save_matches() {
        covered(
            ee::s01::f_00152950__generated,
            be_save_int_build_profile_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x152950, 7);
                oc676_gate(c);
                oc676_save_buffer(c, &mut r, seed, 0x2700);
                let idx = seed % 3;
                fill(c, &mut r, 0x004a_6ca8 + idx * 0x9b50, 0x26d0);
                fill(c, &mut r, 0x0053_5b20, 42);
                oc676_instance(c, 0x1f30, 0);
                oc676_instance(c, 0x1ee0, 1);
                oc676_instance(c, 0x1eb8, 2);
                oc676_instance(c, 0x1ea8, 3);
                oc676_instance(c, 0x1f2c, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                c.pc = 0x00152950;
            },
        );
    }

    /// CRC16 of `len` bytes at `buf` (`EA_Crc16` seed 0xfbea) on a scratch machine.
    fn oc676_crc16(c: &mut Cpu, buf: u32, len: u32) -> u32 {
        let mut t = crate::difftest::fresh();
        for i in 0..len {
            let b = c.bus.read8(buf + i);
            t.bus.write8(buf + i, b);
        }
        t.g[A0][0] = buf as u64;
        t.g[A1][0] = len as u64;
        t.g[A2][0] = 0xfbea;
        t.pc = 0x003e_62d0;
        ee::s14::f_003e62d0(&mut t);
        t.g[V0][0] as u32
    }

    /// `BESaveInt_LoadSystemSave`: a good CRC (loads: copy forms aligned and
    /// not, the three `onLoad` calls, the locale refresh) and a bad one.
    #[test]
    fn be_save_int_load_system_save_matches() {
        covered(
            ee::s01::f_00152bb0__generated,
            be_save_int_load_system_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x152bb0, 6);
                oc676_gate(c);
                let buf = DATA + 0x1_0000 + (seed % 2) * 4;
                fill(c, &mut r, buf, 0x333);
                let crc = oc676_crc16(c, buf, 0xcc8);
                let stored = if seed % 4 == 0 {
                    crc.wrapping_add(1)
                } else {
                    crc
                };
                c.bus.write32(buf + 0xcc8, stored);
                fill(c, &mut r, 0x0053_5b20, 42);
                oc676_instance(c, 0x1ee4, 0);
                oc676_instance(c, 0x1ea8, 1);
                oc676_instance(c, 0x1f30, 2);
                c.bus.write32(GAME_GP - 0x848, DATA + 0x200);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = buf as u64;
                c.pc = 0x00152bb0;
            },
        );
    }
}
