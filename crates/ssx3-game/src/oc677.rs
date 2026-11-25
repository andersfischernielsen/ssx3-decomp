//! Staging module oc677 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc677`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0015_91e8,
    beint_reward_on_event_result,
    "BERewardInt_OnEventResult",
)];

/// `void BERewardInt_OnEventResult(self, profile, charID, mode, event, medal)`
/// (0x001591e8; reference `cBERewardInterface_calculateUserTrackMedalRewards`):
/// the reward bookkeeping after an event result. It looks up the character's
/// score record, the track's tier and peak group, the cash reward for the
/// medal, grants the "first medal" award (medal < 2, no earlier result) or the
/// peak foreshadow messages (medal 2/3 on mountain events), awards cash
/// (halved when the medal is no improvement) and the trophy when it became
/// owned, records the place, grants the mountain-conquered award, stores the
/// last medal and cash for the result screen and finally clears the character
/// flag bits (at `record + 0x278`) of the medal and goal checks of the five
/// tracks of the group's tier.
///
/// Registers: `a0` self, `a1` profile, `a2` charID, `a3` mode, `t0` event,
/// `t1` medal. The body is the generated one, every register, cycle and call
/// kept; resume entries (`pc` != entry) run the generated twin.
#[allow(
    unused_labels,
    unused_variables,
    unused_assignments,
    unused_mut,
    unreachable_code,
    clippy::all
)]
pub fn beint_reward_on_event_result(c: &mut Cpu) {
    if c.pc != 0x001591e8 {
        return ee::s01::f_001591e8__generated(c);
    }
    guest(c, |c| {
        'b_1592f4: {
            'b_1592f0: {
                'b_1592c4: {
                    'b_1592bc: {
                        c.cycles += 50;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                        c.g[V1][0] = 0xf88;
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[V0][0] = 0x4a0000;
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S5][0] = c.g[A1][0];
                        c.g[S6][0] = c.g[A2][0];
                        c.g[A1][0] = 0x9b50;
                        c.mult(0, 3, 22, 3);
                        c.mult(0, 5, 21, 5);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.g[S1][0] = c.g[A3][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[S2][0] = c.g[A0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S4][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[S7][0] = c.g[T0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S3][0] = c.g[T1][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                        c.g[RA][0] = sx(0x00159254);
                        c.g[FP][0] = 0xffffffffffffffff;
                        Callee {
                            addr: 0x0014dc80,
                            f: ee::s01::BELibrary_GetInstance,
                        }
                        .call(c, 0x00159254)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00159260);
                        c.g[A1][0] = 0xb;
                        Callee {
                            addr: 0x0014dd58,
                            f: ee::s01::BELibrary_GetInterface,
                        }
                        .call(c, 0x00159260)?;
                        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x00159274);
                        c.g[A2][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00152398,
                            f: ee::s01::BE_GetCharProfileEventRecord,
                        }
                        .call(c, 0x00159274)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00159288);
                        c.g[A2][0] = c.g[S7][0];
                        Callee {
                            addr: 0x0015a3b8,
                            f: ee::s01::BERewardInt_FindRequirementTier,
                        }
                        .call(c, 0x00159288)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00159298);
                        c.g[A1][0] = c.g[S7][0];
                        Callee {
                            addr: 0x0015a398,
                            f: ee::s01::BERewardInt_GetTrackPeak,
                        }
                        .call(c, 0x00159298)?;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x1));
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                        if t {
                            break 'b_1592bc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x9;
                            let t = c.g[S1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x7;
                            if t {
                                break 'b_1592c4;
                            } else {
                                break 'b_1592bc;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_1592f0;
                    } else {
                        break 'b_1592c4;
                    }
                }
                'b_1592dc: {
                    'b_1592d4: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.g[V0][0] = 0xa;
                        if t {
                            break 'b_1592d4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x8;
                            if t {
                                break 'b_1592dc;
                            } else {
                                break 'b_1592d4;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0x1;
                    if t {
                        break 'b_1592f0;
                    } else {
                        break 'b_1592dc;
                    }
                }
                'b_1592ec: {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xb;
                    if t {
                        break 'b_1592ec;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S1][0] != c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                        if t {
                            break 'b_1592f4;
                        } else {
                            break 'b_1592ec;
                        }
                    }
                }
                c.cycles += 1;
                c.g[S0][0] = 0x2;
                break 'b_1592f0;
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32));
            break 'b_1592f4;
        }
        'b_159350: {
            c.cycles += 8;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S7][0];
            c.g[RA][0] = sx(0x00159304);
            c.g[A3][0] = c.g[S1][0];
            Callee {
                addr: 0x00151040,
                f: ee::s01::BERewardInt_GetEventRewardPoints,
            }
            .call(c, 0x00159304)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[V0][0] = (c.g[S1][0] < 0xeu64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x460000;
            if t {
                break 'b_159350;
            } else {
                'b_15934c: {
                    'b_159344: {
                        'b_15933c: {
                            'b_159334: {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5610));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x0015932c => {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
                                        if t {
                                            break 'b_159350;
                                        } else {
                                            break 'b_159334;
                                        }
                                    }
                                    0x00159334 => {
                                        break 'b_159334;
                                    }
                                    0x0015933c => {
                                        break 'b_15933c;
                                    }
                                    0x00159344 => {
                                        break 'b_159344;
                                    }
                                    0x0015934c => {
                                        break 'b_15934c;
                                    }
                                    0x00159350 => {
                                        break 'b_159350;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001591e8, 0x001597ac);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                            if t {
                                break 'b_159350;
                            } else {
                                break 'b_15933c;
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x17));
                        if t {
                            break 'b_159350;
                        } else {
                            break 'b_159344;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a));
                    if t {
                        break 'b_159350;
                    } else {
                        break 'b_15934c;
                    }
                }
                c.cycles += 1;
                c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d));
                break 'b_159350;
            }
        }
        'b_1593e0: {
            'b_159384: {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S3][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_159384;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1593e0;
                    } else {
                        c.cycles += 7;
                        c.g[A3][0] = c.g[FP][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x0015937c);
                        c.g[A2][0] = c.g[S6][0];
                        Callee {
                            addr: 0x00159cd0,
                            f: ee::s01::BERewardInt_GrantReward,
                        }
                        .call(c, 0x0015937c)?;
                        let t = true;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1593e0;
                        } else {
                            break 'b_159384;
                        }
                    }
                }
            }
            'b_1593dc: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6));
                c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x530000;
                if t {
                    break 'b_1593dc;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1593e0;
                    } else {
                        'b_1593c4: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1593c4;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001593bc);
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x001e1dd0, 0x001593bc)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_1593e0;
                                } else {
                                    break 'b_1593c4;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S3][0] != c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1593e0;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001593dc);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x001e1dd0, 0x001593dc)?;
                            break 'b_1593dc;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1593e0;
        }
        'b_1594b4: {
            'b_1594b0: {
                'b_159478: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001593ec);
                    c.g[A2][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00156a60,
                        f: ee::s01::BERewardInt_IsBetterPlace,
                    }
                    .call(c, 0x001593ec)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    if t {
                        break 'b_159478;
                    } else {
                        c.cycles += 24;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x00159408);
                        c.g[A3][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00157bf0,
                            f: ee::s01::BERewardInt_IsPeakGoalComplete,
                        }
                        .call(c, 0x00159408)?;
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[FP][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00159420);
                        c.g[A2][0] = c.g[S6][0];
                        Callee {
                            addr: 0x00159818,
                            f: ee::s01::BERewardInt_AddPoints,
                        }
                        .call(c, 0x00159420)?;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x00159434);
                        c.g[A3][0] = c.g[S3][0];
                        Callee {
                            addr: 0x00152528,
                            f: ee::s01::BE_RecordCharProfileEventPlace,
                        }
                        .call(c, 0x00159434)?;
                        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x0015944c);
                        c.g[A3][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00157bf0,
                            f: ee::s01::BERewardInt_IsPeakGoalComplete,
                        }
                        .call(c, 0x0015944c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_1594b0;
                        } else {
                            c.cycles += 2;
                            let t = c.g[FP][0] != 0u64;
                            c.g[A1][0] = c.g[S5][0];
                            if t {
                                break 'b_1594b4;
                            } else {
                                c.cycles += 7;
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x00159470);
                                c.g[A3][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00159170,
                                    f: ee::s01::BERewardInt_GrantMedalReward,
                                }
                                .call(c, 0x00159470)?;
                                let t = true;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1594b0;
                                } else {
                                    break 'b_159478;
                                }
                            }
                        }
                    }
                }
                c.cycles += 14;
                c.g[A1][0] = c.g[S7][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0015948c);
                c.g[A3][0] = c.g[S3][0];
                Callee {
                    addr: 0x00152528,
                    f: ee::s01::BE_RecordCharProfileEventPlace,
                }
                .call(c, 0x0015948c)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S6][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[RA][0] = sx(0x001594ac);
                c.g[A3][0] = c.g[V0][0];
                Callee {
                    addr: 0x00159818,
                    f: ee::s01::BERewardInt_AddPoints,
                }
                .call(c, 0x001594ac)?;
                c.g[A0][0] = c.g[S2][0];
                break 'b_1594b0;
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[S5][0];
            break 'b_1594b4;
        }
        'b_1594f4: {
            'b_1594f0: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001594bc);
                c.g[A2][0] = c.g[S6][0];
                Callee {
                    addr: 0x001577e0,
                    f: ee::s01::BERewardInt_IsPeakFullyCompleted,
                }
                .call(c, 0x001594bc)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1594f0;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x001594d4);
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x0015a2e0,
                        f: ee::s01::BERewardInt_IsRewardGranted,
                    }
                    .call(c, 0x001594d4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e8c));
                    if t {
                        break 'b_1594f4;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x001594f0);
                        c.g[A3][0] = 0u64;
                        Callee {
                            addr: 0x00159cd0,
                            f: ee::s01::BERewardInt_GrantReward,
                        }
                        .call(c, 0x001594f0)?;
                        break 'b_1594f0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e8c));
            break 'b_1594f4;
        }
        'b_159780: {
            'b_15977c: {
                c.cycles += 9;
                c.g[A2][0] = c.g[S3][0];
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0xffffffffffffffff;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
                c.g[RA][0] = sx(0x00159510);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                Callee {
                    addr: 0x00156a60,
                    f: ee::s01::BERewardInt_IsBetterPlace,
                }
                .call(c, 0x00159510)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_15977c;
                } else {
                    c.cycles += 14;
                    c.g[V0][0] = 0xa;
                    c.g[A1][0] = 0x28;
                    c.g[FP][0] = 0u64;
                    c.mult(0, 2, 3, 2);
                    c.mult(0, 5, 16, 5);
                    c.g[V1][0] = 0x1;
                    c.g[S7][0] = 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5528));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    'l_159550: loop {
                        'b_159634: {
                            'b_159590: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[A3][0] = c.lb((c.g[S1][0] as u32));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
                                c.g[V1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = c.g[A3][0] != c.g[V0][0];
                                c.g[T0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_159590;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = 0x530000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                    if t {
                                        break 'b_159634;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00159588);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x001e1c10, 0x00159588)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                        if t {
                                            break 'b_159634;
                                        } else {
                                            break 'b_159590;
                                        }
                                    }
                                }
                            }
                            'b_159630: {
                                'b_159620: {
                                    'b_1595d0: {
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4));
                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1595d0;
                                        } else {
                                            c.cycles += 10;
                                            c.g[A1][0] = c.g[S5][0];
                                            c.g[A2][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001595b0);
                                            c.g[FP][0] = 0x1;
                                            Callee {
                                                addr: 0x00158910,
                                                f: ee::s01::BERewardInt_GetEventBestPlace,
                                            }
                                            .call(c, 0x001595b0)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001595c0);
                                            c.g[A1][0] = 0xffffffffffffffff;
                                            Callee {
                                                addr: 0x00156a60,
                                                f: ee::s01::BERewardInt_IsBetterPlace,
                                            }
                                            .call(c, 0x001595c0)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S7][0] =
                                                    sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                                break 'b_159620;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                                if t {
                                                    break 'b_159630;
                                                } else {
                                                    break 'b_1595d0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[RA][0] = sx(0x001595dc);
                                    c.g[A2][0] = c.g[S6][0];
                                    Callee {
                                        addr: 0x00158910,
                                        f: ee::s01::BERewardInt_GetEventBestPlace,
                                    }
                                    .call(c, 0x001595dc)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001595ec);
                                    c.g[A1][0] = 0xffffffffffffffff;
                                    Callee {
                                        addr: 0x00156a60,
                                        f: ee::s01::BERewardInt_IsBetterPlace,
                                    }
                                    .call(c, 0x001595ec)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                        break 'b_159620;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[FP][0] != 0u64;
                                        c.g[V1][0] = 0x530000;
                                        if t {
                                            break 'b_159630;
                                        } else {
                                            'b_159618: {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                                                c.g[V0][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x14),
                                                        0u64,
                                                    );
                                                    break 'b_159618;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x00159614);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x001e1c10, 0x00159614)?;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x14),
                                                        0u64,
                                                    );
                                                    break 'b_159618;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                            if t {
                                                break 'b_159630;
                                            } else {
                                                break 'b_159620;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = ((c.g[S7][0] as i64) < 5i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00159550);
                                    }
                                    continue 'l_159550;
                                } else {
                                    break 'b_159630;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            break 'b_159634;
                        }
                        'b_1596dc: {
                            'b_1596d8: {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1596d8;
                                } else {
                                    'b_1596c4: {
                                        'b_1596b8: {
                                            'b_159680: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != 0u64;

                                                if t {
                                                    break 'b_159680;
                                                } else {
                                                    'b_159658: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != 0u64;
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_159658;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.ld((c.g[S4][0] as u32)
                                                                .wrapping_add(0x278));
                                                            let t = true;
                                                            c.g[V1][0] = 0xffffffffffffffbf;
                                                            if t {
                                                                break 'b_1596b8;
                                                            } else {
                                                                break 'b_159658;
                                                            }
                                                        }
                                                    }
                                                    'b_15966c: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x2;
                                                        if t {
                                                            break 'b_15966c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.ld((c.g[S4][0] as u32)
                                                                .wrapping_add(0x278));
                                                            let t = true;
                                                            c.g[V1][0] = 0xffffffffffffff7f;
                                                            if t {
                                                                break 'b_1596b8;
                                                            } else {
                                                                break 'b_15966c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x530000;
                                                    if t {
                                                        break 'b_1596c4;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .ld((c.g[S4][0] as u32)
                                                                .wrapping_add(0x278));
                                                        let t = true;
                                                        c.g[V1][0] = 0xfffffffffffffeff;
                                                        if t {
                                                            break 'b_1596b8;
                                                        } else {
                                                            break 'b_159680;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_159694: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_159694;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.ld(
                                                            (c.g[S4][0] as u32).wrapping_add(0x278)
                                                        );
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffdff;
                                                    if t {
                                                        break 'b_1596b8;
                                                    } else {
                                                        break 'b_159694;
                                                    }
                                                }
                                            }
                                            'b_1596a8: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_1596a8;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.ld(
                                                            (c.g[S4][0] as u32).wrapping_add(0x278)
                                                        );
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffbff;
                                                    if t {
                                                        break 'b_1596b8;
                                                    } else {
                                                        break 'b_1596a8;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x530000;
                                            if t {
                                                break 'b_1596c4;
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                                c.g[V1][0] = 0xfffffffffffff7ff;
                                                break 'b_1596b8;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        c.sd((c.g[S4][0] as u32).wrapping_add(0x278), c.g[V0][0]);
                                        c.g[V0][0] = 0x530000;
                                        break 'b_1596c4;
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5f9));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_1596dc;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001596d8);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x001e2370, 0x001596d8)?;
                                        break 'b_1596d8;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            break 'b_1596dc;
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_15977c;
                        } else {
                            'b_159774: {
                                'b_159730: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != 0u64;

                                    if t {
                                        break 'b_159730;
                                    } else {
                                        'b_159700: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_159700;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                                let t = true;
                                                c.g[V1][0] = 0xffffffffffffbfff;
                                                if t {
                                                    break 'b_159774;
                                                } else {
                                                    break 'b_159700;
                                                }
                                            }
                                        }
                                        'b_159718: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_159718;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] = 0xffffffffffff0000;
                                                c.g[V1][0] = c.g[V1][0] | 0x7fff;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                                if t {
                                                    break 'b_159774;
                                                } else {
                                                    break 'b_159718;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        if t {
                                            break 'b_159780;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V1][0] = 0xfffffffffffe0000;
                                            c.g[V1][0] = c.g[V1][0] | 0xffff;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                            if t {
                                                break 'b_159774;
                                            } else {
                                                break 'b_159730;
                                            }
                                        }
                                    }
                                }
                                'b_159748: {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] != 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_159748;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0xfffffffffffd0000;
                                        c.g[V1][0] = c.g[V1][0] | 0xffff;
                                        let t = true;
                                        c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                        if t {
                                            break 'b_159774;
                                        } else {
                                            break 'b_159748;
                                        }
                                    }
                                }
                                'b_159760: {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_159760;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0xfffffffffffb0000;
                                        c.g[V1][0] = c.g[V1][0] | 0xffff;
                                        let t = true;
                                        c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                        if t {
                                            break 'b_159774;
                                        } else {
                                            break 'b_159760;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                if t {
                                    break 'b_159780;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x278));
                                    c.g[V1][0] = 0xfffffffffff70000;
                                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                                    break 'b_159774;
                                }
                            }
                            c.cycles += 2;
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            c.sd((c.g[S4][0] as u32).wrapping_add(0x278), c.g[V0][0]);
                            break 'b_15977c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_159780;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// Gate word of this module's fakes (in the machine's RAM).
#[cfg(test)]
pub(crate) const OC677_CTRL: u32 = crate::difftest::DATA + 0xf_e000;

#[cfg(test)]
pub(crate) const OC677_MAGIC: u32 = 0x0677_a11e;

/// Scripted results of the fakes (by call index & 31).
#[cfg(test)]
pub(crate) const OC677_RET: u32 = OC677_CTRL + 0x10;

/// The score-stat record `BE_GetCharProfileEventRecord` is faked to return.
#[cfg(test)]
pub(crate) const OC677_STATS: u32 = crate::difftest::DATA + 0x2800;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, fake, fall_back, hook, machine, LOG_COUNT};
    use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Log the call like every fake, then return the scripted result of this
    /// call index (shaped per callee so the data it feeds stays in range).
    fn oc677_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fake(c, addr, ra);
        if c.pc == ra {
            let word = c.bus.read32(OC677_RET + 4 * (n & 31));
            let v = match addr {
                0x0015_2398 => OC677_STATS,
                0x0015_a3b8 => word & 1,
                0x0015_a398 => word % 3,
                0x001e_1ad0 => 0x100,
                0x0014_7398 => word % 10,
                0x001e_3a30 => word & 1,
                _ => word,
            };
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc677_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC677_CTRL) != OC677_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                oc677_fake_call(c, $addr);
            }
        };
    }

    oc677_fake!(
        oc677_fake_0014dc80,
        OC677_PREV_0014DC80,
        0x0014dc80,
        ee::s01::f_0014dc80__generated
    );
    oc677_fake!(
        oc677_fake_0014dd58,
        OC677_PREV_0014DD58,
        0x0014dd58,
        ee::s01::f_0014dd58__generated
    );
    oc677_fake!(
        oc677_fake_00152398,
        OC677_PREV_00152398,
        0x00152398,
        ee::s01::f_00152398__generated
    );
    oc677_fake!(
        oc677_fake_0015a3b8,
        OC677_PREV_0015A3B8,
        0x0015a3b8,
        ee::s01::f_0015a3b8__generated
    );
    oc677_fake!(
        oc677_fake_0015a398,
        OC677_PREV_0015A398,
        0x0015a398,
        ee::s01::f_0015a398__generated
    );
    oc677_fake!(
        oc677_fake_00159cd0,
        OC677_PREV_00159CD0,
        0x00159cd0,
        ee::s01::f_00159cd0__generated
    );
    oc677_fake!(
        oc677_fake_00156a60,
        OC677_PREV_00156A60,
        0x00156a60,
        ee::s01::f_00156a60__generated
    );
    oc677_fake!(
        oc677_fake_00157bf0,
        OC677_PREV_00157BF0,
        0x00157bf0,
        ee::s01::f_00157bf0__generated
    );
    oc677_fake!(
        oc677_fake_00159818,
        OC677_PREV_00159818,
        0x00159818,
        ee::s01::f_00159818__generated
    );
    oc677_fake!(
        oc677_fake_00152528,
        OC677_PREV_00152528,
        0x00152528,
        ee::s01::f_00152528__generated
    );
    oc677_fake!(
        oc677_fake_00159170,
        OC677_PREV_00159170,
        0x00159170,
        ee::s01::f_00159170__generated
    );
    oc677_fake!(
        oc677_fake_001577e0,
        OC677_PREV_001577E0,
        0x001577e0,
        ee::s01::f_001577e0__generated
    );
    oc677_fake!(
        oc677_fake_0015a2e0,
        OC677_PREV_0015A2E0,
        0x0015a2e0,
        ee::s01::f_0015a2e0__generated
    );
    oc677_fake!(
        oc677_fake_00158910,
        OC677_PREV_00158910,
        0x00158910,
        ee::s01::f_00158910__generated
    );
    oc677_fake!(
        oc677_fake_00145630,
        OC677_PREV_00145630,
        0x00145630,
        ee::s01::f_00145630__generated
    );
    oc677_fake!(
        oc677_fake_001e3a30,
        OC677_PREV_001E3A30,
        0x001e3a30,
        ee::s04::f_001e3a30__generated
    );
    oc677_fake!(
        oc677_fake_001e3a78,
        OC677_PREV_001E3A78,
        0x001e3a78,
        ee::s04::f_001e3a78__generated
    );
    oc677_fake!(
        oc677_fake_001e1ad0,
        OC677_PREV_001E1AD0,
        0x001e1ad0,
        ee::s04::f_001e1ad0__generated
    );
    oc677_fake!(
        oc677_fake_00146e98,
        OC677_PREV_00146E98,
        0x00146e98,
        ee::s01::f_00146e98__generated
    );
    oc677_fake!(
        oc677_fake_00147398,
        OC677_PREV_00147398,
        0x00147398,
        ee::s01::f_00147398__generated
    );
    oc677_fake!(
        oc677_fake_001e2370,
        OC677_PREV_001E2370,
        0x001e2370,
        ee::s04::f_001e2370__generated
    );

    fn oc677_install() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc677_fake_0014dc80,
            &OC677_PREV_0014DC80,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc677_fake_0014dd58,
            &OC677_PREV_0014DD58,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152398),
            oc677_fake_00152398,
            &OC677_PREV_00152398,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A3B8),
            oc677_fake_0015a3b8,
            &OC677_PREV_0015A3B8,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A398),
            oc677_fake_0015a398,
            &OC677_PREV_0015A398,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00159CD0),
            oc677_fake_00159cd0,
            &OC677_PREV_00159CD0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00156A60),
            oc677_fake_00156a60,
            &OC677_PREV_00156A60,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00157BF0),
            oc677_fake_00157bf0,
            &OC677_PREV_00157BF0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00159818),
            oc677_fake_00159818,
            &OC677_PREV_00159818,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152528),
            oc677_fake_00152528,
            &OC677_PREV_00152528,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00159170),
            oc677_fake_00159170,
            &OC677_PREV_00159170,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_001577E0),
            oc677_fake_001577e0,
            &OC677_PREV_001577E0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A2E0),
            oc677_fake_0015a2e0,
            &OC677_PREV_0015A2E0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158910),
            oc677_fake_00158910,
            &OC677_PREV_00158910,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145630),
            oc677_fake_00145630,
            &OC677_PREV_00145630,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3A30),
            oc677_fake_001e3a30,
            &OC677_PREV_001E3A30,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3A78),
            oc677_fake_001e3a78,
            &OC677_PREV_001E3A78,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E1AD0),
            oc677_fake_001e1ad0,
            &OC677_PREV_001E1AD0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc677_fake_00146e98,
            &OC677_PREV_00146E98,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            oc677_fake_00147398,
            &OC677_PREV_00147398,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E2370),
            oc677_fake_001e2370,
            &OC677_PREV_001E2370,
        );
    }

    /// `BERewardInt_OnEventResult` against the generated body: every mode
    /// 0..=14, every medal 0..=4, both settings of the peak-message flag
    /// (`D_005305F9`), random medal tables (tracks -1 and 4..5, kinds), random
    /// flag words, stats records with and without a count, and callees
    /// (back-end lookups, awards, place and peak checks, the message centre)
    /// answered by gated fakes, which unwind at random call indices.
    #[test]
    fn beint_reward_on_event_result_matches() {
        oc677_install();
        fakes::covered(
            ee::s01::f_001591e8__generated,
            beint_reward_on_event_result,
            450,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_91e8, 30);
                elf_image(c);
                w(c, OC677_CTRL, OC677_MAGIC);
                for i in 0..32u32 {
                    let word = r.next_u32();
                    w(c, OC677_RET + 4 * i, word);
                }
                // `gMedalsInfo`: [group][tier][5] of (track, kind) bytes.
                for i in 0..0x80u32 {
                    let b = if i % 2 == 0 {
                        [0xff, 0, 1, 2, 3, 4, 5, 6, 0xff][(r.next_u32() % 9) as usize]
                    } else {
                        (r.next_u32() % 4) as u8
                    };
                    c.bus.write8(0x0045_aad8 + i, b);
                }
                // The world: `world->+0x84->+0x28 -> int` (message state).
                let wld = DATA + 0x1000;
                w(c, GAME_GP.wrapping_sub(0x848), wld);
                w(c, wld + 0x84, DATA + 0x1200);
                w(c, DATA + 0x1228, DATA + 0x1300);
                w(c, DATA + 0x1300, [0, 5, 12][(seed % 3) as usize]);
                // The reward interface's last-result record.
                w(c, GAME_GP.wrapping_sub(0x1e8c), DATA + 0x2000);
                // The medal stats: old medal byte at +1, count at +2.
                c.bus.write8(
                    OC677_STATS + 1,
                    [0xff, 0, 1, 2, 3, 4][(r.next_u32() % 6) as usize],
                );
                c.bus
                    .write16(OC677_STATS + 2, if seed % 3 == 0 { 0 } else { 2 });
                let profile = seed % 3;
                let ch = (seed / 3) % 3;
                let mode = seed % 15;
                let medal = (seed / 15) % 5;
                c.bus.write8(0x0053_05f9, ((seed / 75) % 2) as u8);
                let rec = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88;
                c.bus.write32(rec + 0x278, r.next_u32());
                c.bus.write32(rec + 0x27c, r.next_u32());
                c.pc = 0x0015_91e8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = mode as u64;
                c.g[T0][0] = (r.next_u32() % 20) as u64;
                c.g[T1][0] = medal as u64;
            },
        );
    }
}
