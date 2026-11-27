//! Staging module oc967 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc967`.
//!
//! The bodies keep the generated control flow (labelled blocks, every cycle and
//! register write); callees go through `call_ptr` / `Callee` with the original
//! return addresses.

#![allow(unused_imports, unused_labels, unreachable_code)]

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
        0x001e2828,
        message_center_post_bc_finish_message_b,
        "MessageCenter_PostBCFinishMessageB",
    ),
    (
        0x001e2a08,
        message_center_check_aggression_message,
        "MessageCenter_CheckAggressionMessage",
    ),
    (
        0x001e2c80,
        message_center_get_group_message_id,
        "MessageCenter_GetGroupMessageId",
    ),
    (
        0x001e2fe0,
        message_center_post_message,
        "MessageCenter_PostMessage",
    ),
    (
        0x001e3100,
        message_center_pick_variant,
        "MessageCenter_PickVariant",
    ),
    (
        0x001e33e8,
        message_center_get_message_count,
        "MessageCenter_GetMessageCount",
    ),
    (
        0x001e3760,
        message_center_check_cash_messages,
        "MessageCenter_CheckCashMessages",
    ),
    (
        0x001e38b8,
        message_center_check_peak_messages,
        "MessageCenter_CheckPeakMessages",
    ),
];

/// `MessageCenter_PostBCFinishMessageB(which, flag)` (0x001e2828): the battle-challenge
/// finish messages of cases 14, 15 and 16 (flags 0x19..0x1b, groups 4..6). Does
/// nothing while a replay runs (replay length 1..9).
#[allow(unused_labels)]
pub fn message_center_post_bc_finish_message_b(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e2860: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e2860;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e2860;
            }
        }
        'b_1e29f4: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1e29f4;
            } else {
                'b_1e29e4: {
                    'b_1e29d4: {
                        'b_1e2984: {
                            'b_1e292c: {
                                c.cycles += 16;
                                call_ptr(c, 0x0014dc80, 0x001e2870)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x0014dd58, 0x001e287c)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00146e98, 0x001e288c)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x00147398, 0x001e2898)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V0][0] = 0xf;
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S1][0] as i64) < 16i64) as u64;
                                if t {
                                    break 'b_1e292c;
                                } else {
                                    'b_1e28d4: {
                                        'b_1e28c0: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xe;
                                            if t {
                                                break 'b_1e28c0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_1e28d4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1e29f4;
                                                    } else {
                                                        break 'b_1e28c0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x10;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_1e2984;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_1e29f4;
                                            } else {
                                                break 'b_1e28d4;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x19;
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::f_001e3a30,
                                    }
                                    .call(c, 0x001e28dc)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        break 'b_1e29f4;
                                    } else {
                                        'b_1e291c: {
                                            c.cycles += 6;
                                            c.g[A0][0] = 0x4;
                                            Callee {
                                                addr: 0x001e32c8,
                                                f: ee::s04::f_001e32c8,
                                            }
                                            .call(c, 0x001e28ec)?;
                                            c.g[A0][0] = 0x19;
                                            Callee {
                                                addr: 0x001e32c8,
                                                f: ee::s04::f_001e32c8,
                                            }
                                            .call(c, 0x001e28f4)?;
                                            let t = c.g[S2][0] != 0u64;
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1e291c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] = 0x19;
                                                c.g[A1][0] = 0x1;
                                                Callee {
                                                    addr: 0x001e3a78,
                                                    f: ee::s04::f_001e3a78,
                                                }
                                                .call(c, 0x001e2908)?;
                                                c.g[V0][0] = 0x3;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[A0][0] = 0x80;
                                                if t {
                                                    break 'b_1e29d4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] = 0x82;
                                                    if t {
                                                        break 'b_1e29d4;
                                                    } else {
                                                        break 'b_1e291c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x7f;
                                        if t {
                                            break 'b_1e29d4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = 0x81;
                                            if t {
                                                break 'b_1e29d4;
                                            } else {
                                                break 'b_1e292c;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = 0x1a;
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::f_001e3a30,
                            }
                            .call(c, 0x001e2934)?;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_1e29f4;
                            } else {
                                'b_1e2974: {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x5;
                                    Callee {
                                        addr: 0x001e32c8,
                                        f: ee::s04::f_001e32c8,
                                    }
                                    .call(c, 0x001e2944)?;
                                    c.g[A0][0] = 0x1a;
                                    Callee {
                                        addr: 0x001e32c8,
                                        f: ee::s04::f_001e32c8,
                                    }
                                    .call(c, 0x001e294c)?;
                                    let t = c.g[S2][0] != 0u64;
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_1e2974;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x1a;
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001e3a78,
                                            f: ee::s04::f_001e3a78,
                                        }
                                        .call(c, 0x001e2960)?;
                                        c.g[V0][0] = 0x7;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x84;
                                        if t {
                                            break 'b_1e29d4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = 0x86;
                                            if t {
                                                break 'b_1e29d4;
                                            } else {
                                                break 'b_1e2974;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A0][0] = 0x83;
                                if t {
                                    break 'b_1e29d4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = 0x85;
                                    if t {
                                        break 'b_1e29d4;
                                    } else {
                                        break 'b_1e2984;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = 0x1b;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::f_001e3a30,
                        }
                        .call(c, 0x001e298c)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_1e29f4;
                        } else {
                            'b_1e29cc: {
                                c.cycles += 6;
                                c.g[A0][0] = 0x6;
                                Callee {
                                    addr: 0x001e32c8,
                                    f: ee::s04::f_001e32c8,
                                }
                                .call(c, 0x001e299c)?;
                                c.g[A0][0] = 0x1b;
                                Callee {
                                    addr: 0x001e32c8,
                                    f: ee::s04::f_001e32c8,
                                }
                                .call(c, 0x001e29a4)?;
                                let t = c.g[S2][0] != 0u64;
                                c.g[V0][0] = 0x8;
                                if t {
                                    break 'b_1e29cc;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x1b;
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001e3a78,
                                        f: ee::s04::f_001e3a78,
                                    }
                                    .call(c, 0x001e29b8)?;
                                    c.g[V0][0] = 0x8;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = 0x88;
                                    if t {
                                        break 'b_1e29d4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = 0x8a;
                                        if t {
                                            break 'b_1e29d4;
                                        } else {
                                            break 'b_1e29cc;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S0][0] == c.g[V0][0];
                            c.g[A0][0] = 0x87;
                            if t {
                                break 'b_1e29e4;
                            } else {
                                break 'b_1e29d4;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x001e2fe0,
                        f: ee::s04::f_001e2fe0,
                    }
                    .call(c, 0x001e29dc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1e29f4;
                    } else {
                        break 'b_1e29e4;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = 0x89;
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x001e2fe0,
                    f: ee::s04::f_001e2fe0,
                }
                .call(c, 0x001e29f0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1e29f4;
            }
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

/// `MessageCenter_CheckAggressionMessage(charID, level)` (0x001e2a08): rolls the
/// aggression message of one rider group. Skipped during a replay and for the
/// rider's own character.
#[allow(unused_labels)]
pub fn message_center_check_aggression_message(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e2a40: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e2a40;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e2a40;
            }
        }
        'b_1e2b20: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1e2b20;
            } else {
                c.cycles += 14;
                call_ptr(c, 0x0014dc80, 0x001e2a50)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x001e2a5c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00146e98, 0x001e2a6c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x00147398, 0x001e2a78)?;
                let t = c.g[V0][0] == c.g[S1][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1e2b20;
                } else {
                    'b_1e2b1c: {
                        'b_1e2ac8: {
                            c.cycles += 12;
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001e2b38,
                                f: ee::s04::f_001e2b38,
                            }
                            .call(c, 0x001e2a88)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = 0x14;
                            c.mult(0, 3, 16, 3);
                            c.g[V0][0] = 0x440000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::f_001e3a30,
                            }
                            .call(c, 0x001e2aa8)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                            if t {
                                break 'b_1e2ac8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_1e2b1c;
                                } else {
                                    c.cycles += 4;
                                    c.g[A1][0] = 0u64;
                                    Callee {
                                        addr: 0x001e3a78,
                                        f: ee::s04::f_001e3a78,
                                    }
                                    .call(c, 0x001e2ac0)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1e2b20;
                                    } else {
                                        break 'b_1e2ac8;
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        call_ptr(c, 0x003177f0, 0x001e2ad0)?;
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_1e2b20;
                        } else {
                            'b_1e2b00: {
                                c.cycles += 7;
                                call_ptr(c, 0x003177f0, 0x001e2aec)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.divu(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_1e2b00;
                                } else {
                                    break 'b_1e2b00;
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = c.hi[0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            Callee {
                                addr: 0x001e2fe0,
                                f: ee::s04::f_001e2fe0,
                            }
                            .call(c, 0x001e2b10)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001e3a78,
                                f: ee::s04::f_001e3a78,
                            }
                            .call(c, 0x001e2b1c)?;
                            break 'b_1e2b1c;
                        }
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_1e2b20;
                }
            }
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

/// `int MessageCenter_GetGroupMessageId(type, idx, mode)` (0x001e2c80): walks the
/// 61 message groups of `type` and returns the id of variant `idx`, or -1.
#[allow(unused_labels)]
pub fn message_center_get_group_message_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0x440000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S6][0] = c.g[A0][0];
        'l_1e2cc8: loop {
            'b_1e2d2c: {
                'b_1e2d20: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    if c.g[V0][0] != c.g[S6][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_1e2d20;
                    } else {
                        'b_1e2cfc: {
                            c.cycles += 2;
                            if c.g[S5][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1e2cfc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    break 'b_1e2cfc;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::f_001e3a30,
                                    }
                                    .call(c, 0x001e2cf0)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = 0x1;
                                    if t {
                                        break 'b_1e2cfc;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1e2cfc;
                                    }
                                }
                            }
                        }
                        'b_1e2d18: {
                            c.cycles += 4;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[S3][0] as u32)));
                            if t {
                                break 'b_1e2d18;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                let t = true;
                                c.g[S4][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                if t {
                                    break 'b_1e2d2c;
                                } else {
                                    break 'b_1e2d18;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[S3][0] = c.g[V1][0];
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_1e2d20;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 61i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001e2cc8);
                    }
                    continue 'l_1e2cc8;
                } else {
                    break 'b_1e2d2c;
                }
            }
            c.cycles += 11;
            c.g[V0][0] = c.g[S4][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.pc = t;
            return Ok(());
        }
    });
}

/// `MessageCenter_PostMessage(msg, flags)` (0x001e2fe0): posts `msg` to the inbox,
/// choosing a variant through `MessageCenter_PickVariant` when the table asks.
#[allow(unused_labels)]
pub fn message_center_post_message(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e3024: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0] & 0xff;
            Callee {
                addr: 0x001e39f8,
                f: ee::s04::f_001e39f8,
            }
            .call(c, 0x001e3004)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = 0x19;
            c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xcc));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0x18;
            if t {
                break 'b_1e3024;
            } else {
                c.cycles += 3;
                c.g[A0][0] = 0u64;
                Callee {
                    addr: 0x001e31b8,
                    f: ee::s04::f_001e31b8,
                }
                .call(c, 0x001e3020)?;
                c.g[A0][0] = 0x18;
                break 'b_1e3024;
            }
        }
        'b_1e3054: {
            c.cycles += 8;
            c.g[V0][0] = 0x4c0000;
            c.mult(0, 4, 16, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[S3][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            if c.g[V1][0] != c.g[S3][0] {
                c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xcc));
                break 'b_1e3054;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001e3100,
                    f: ee::s04::f_001e3100,
                }
                .call(c, 0x001e304c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xcc));
                break 'b_1e3054;
            }
        }
        'b_1e30d8: {
            c.cycles += 18;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S0][0]);
            c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xcc));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sb((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
            c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xcc));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = sx((c.g[S3][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc8), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
            let t = c.g[S0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_1e30d8;
            } else {
                c.cycles += 9;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
                call_ptr(c, 0x00317670, 0x001e30a8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039f9d8, 0x001e30b4)?;
                c.g[A3][0] = c.g[V0][0];
                let t = c.g[A3][0] == 0u64;
                c.g[A1][0] = 0x8;
                if t {
                    break 'b_1e30d8;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e30d8)?;
                    break 'b_1e30d8;
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xcc));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sb((c.g[S1][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int MessageCenter_PickVariant(msg)` (0x001e3100): a random variant (0..3 or
/// 0..10) chosen by which message types are present.
#[allow(unused_labels)]
pub fn message_center_pick_variant(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e31a0: {
            'b_1e319c: {
                'b_1e3188: {
                    'b_1e317c: {
                        c.cycles += 11;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S0][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[S1][0] = 0xff;
                        c.g[A0][0] = 0x3;
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001e2d58,
                            f: ee::s04::f_001e2d58,
                        }
                        .call(c, 0x001e3124)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x5;
                        if t {
                            break 'b_1e317c;
                        } else {
                            'b_1e314c: {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001e2d58,
                                    f: ee::s04::f_001e2d58,
                                }
                                .call(c, 0x001e3134)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x8;
                                if t {
                                    break 'b_1e314c;
                                } else {
                                    c.cycles += 4;
                                    call_ptr(c, 0x003177f0, 0x001e3144)?;
                                    let t = true;
                                    c.g[V1][0] = 0xb;
                                    if t {
                                        break 'b_1e3188;
                                    } else {
                                        break 'b_1e314c;
                                    }
                                }
                            }
                            'b_1e316c: {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001e2d58,
                                    f: ee::s04::f_001e2d58,
                                }
                                .call(c, 0x001e3154)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[S0][0];
                                if t {
                                    break 'b_1e316c;
                                } else {
                                    c.cycles += 4;
                                    call_ptr(c, 0x003177f0, 0x001e3164)?;
                                    let t = true;
                                    c.g[S1][0] = c.g[V0][0] & 0x3;
                                    if t {
                                        break 'b_1e319c;
                                    } else {
                                        break 'b_1e316c;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = 0x4;
                            Callee {
                                addr: 0x001e2d58,
                                f: ee::s04::f_001e2d58,
                            }
                            .call(c, 0x001e3174)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S1][0];
                            if t {
                                break 'b_1e31a0;
                            } else {
                                break 'b_1e317c;
                            }
                        }
                    }
                    c.cycles += 3;
                    call_ptr(c, 0x003177f0, 0x001e3184)?;
                    c.g[V1][0] = 0xa;
                    break 'b_1e3188;
                }
                'b_1e3194: {
                    c.cycles += 3;
                    c.divu(0, 2, 3);
                    if c.g[V1][0] == 0u64 {
                        break 'b_1e3194;
                    } else {
                        break 'b_1e3194;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = c.hi[0];
                c.g[S1][0] = c.g[A0][0] & 0xff;
                break 'b_1e319c;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
            break 'b_1e31a0;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `int MessageCenter_GetMessageCount(void)` (0x001e33e8): the inbox count byte.
#[allow(unused_labels)]
pub fn message_center_get_message_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        Callee {
            addr: 0x001e39f8,
            f: ee::s04::f_001e39f8,
        }
        .call(c, 0x001e33f8)?;
        c.g[V0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xcc));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `MessageCenter_CheckCashMessages(rider, amount)` (0x001e3760): the cash
/// messages after earnings of `amount`.
#[allow(unused_labels)]
pub fn message_center_check_cash_messages(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e3898: {
            'b_1e3894: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S5][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[S1][0] != 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1e3894;
                } else {
                    'b_1e3834: {
                        'b_1e3830: {
                            c.cycles += 22;
                            call_ptr(c, 0x0014dc80, 0x001e3794)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0xb;
                            call_ptr(c, 0x0014dd58, 0x001e37a0)?;
                            c.g[S4][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dc80, 0x001e37a8)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0xd;
                            call_ptr(c, 0x0014dd58, 0x001e37b4)?;
                            c.g[S3][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dc80, 0x001e37bc)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014dd58, 0x001e37c8)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00146008, 0x001e37dc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S4][0];
                            if t {
                                break 'b_1e3830;
                            } else {
                                c.cycles += 9;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x001509f8, 0x001e37ec)?;
                                c.g[S0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x001588b0, 0x001e37fc)?;
                                c.g[S0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                let t = c.g[S0][0] != 0u64;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1e3834;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x34;
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::f_001e3a30,
                                    }
                                    .call(c, 0x001e3810)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_1e3834;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x34;
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001e3a78,
                                            f: ee::s04::f_001e3a78,
                                        }
                                        .call(c, 0x001e3824)?;
                                        c.g[A0][0] = 0xf7;
                                        c.g[A1][0] = 0u64;
                                        Callee {
                                            addr: 0x001e2fe0,
                                            f: ee::s04::f_001e2fe0,
                                        }
                                        .call(c, 0x001e3830)?;
                                        break 'b_1e3830;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S2][0];
                        break 'b_1e3834;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0x2;
                    call_ptr(c, 0x00146008, 0x001e3840)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_1e3894;
                    } else {
                        c.cycles += 9;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x001509f8, 0x001e3850)?;
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x001588b0, 0x001e3860)?;
                        c.g[S0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                        let t = c.g[S0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_1e3898;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = 0x35;
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::f_001e3a30,
                            }
                            .call(c, 0x001e3874)?;
                            let t = c.g[V0][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_1e3898;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = 0x35;
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001e3a78,
                                    f: ee::s04::f_001e3a78,
                                }
                                .call(c, 0x001e3888)?;
                                c.g[A0][0] = 0xf8;
                                c.g[A1][0] = 0u64;
                                Callee {
                                    addr: 0x001e2fe0,
                                    f: ee::s04::f_001e2fe0,
                                }
                                .call(c, 0x001e3894)?;
                                break 'b_1e3894;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_1e3898;
        }
        c.cycles += 8;
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

/// `MessageCenter_CheckPeakMessages(rider)` (0x001e38b8): the peak-lock messages
/// of one rider.
#[allow(unused_labels)]
pub fn message_center_check_peak_messages(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e39dc: {
            'b_1e39d8: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S1][0] != 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1e39d8;
                } else {
                    'b_1e3980: {
                        'b_1e397c: {
                            c.cycles += 24;
                            call_ptr(c, 0x0014dc80, 0x001e38e0)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0xd;
                            call_ptr(c, 0x0014dd58, 0x001e38ec)?;
                            c.g[S3][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dc80, 0x001e38f4)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014dd58, 0x001e3900)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00146e98, 0x001e3910)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00147398, 0x001e391c)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00146008, 0x001e3930)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S3][0];
                            if t {
                                break 'b_1e397c;
                            } else {
                                c.cycles += 7;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = c.g[S2][0];
                                c.g[A3][0] = 0u64;
                                c.g[T0][0] = 0x2;
                                call_ptr(c, 0x00157bf0, 0x001e394c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_1e3980;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x36;
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::f_001e3a30,
                                    }
                                    .call(c, 0x001e395c)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_1e3980;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x36;
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001e3a78,
                                            f: ee::s04::f_001e3a78,
                                        }
                                        .call(c, 0x001e3970)?;
                                        c.g[A0][0] = 0xf9;
                                        c.g[A1][0] = 0u64;
                                        Callee {
                                            addr: 0x001e2fe0,
                                            f: ee::s04::f_001e2fe0,
                                        }
                                        .call(c, 0x001e397c)?;
                                        break 'b_1e397c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        break 'b_1e3980;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0x2;
                    call_ptr(c, 0x00146008, 0x001e398c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        break 'b_1e39d8;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A3][0] = 0x1;
                        c.g[T0][0] = 0x2;
                        call_ptr(c, 0x00157bf0, 0x001e39a8)?;
                        let t = c.g[V0][0] == 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        if t {
                            break 'b_1e39dc;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = 0x37;
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::f_001e3a30,
                            }
                            .call(c, 0x001e39b8)?;
                            let t = c.g[V0][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1e39dc;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = 0x37;
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001e3a78,
                                    f: ee::s04::f_001e3a78,
                                }
                                .call(c, 0x001e39cc)?;
                                c.g[A0][0] = 0xfa;
                                c.g[A1][0] = 0u64;
                                Callee {
                                    addr: 0x001e2fe0,
                                    f: ee::s04::f_001e2fe0,
                                }
                                .call(c, 0x001e39d8)?;
                                break 'b_1e39d8;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1e39dc;
        }
        c.cycles += 6;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word: the fakes answer only in a machine that writes `OC967_MAGIC`
    /// there; other tests' machines fall back to the real bodies.
    const OC967_GATE: u32 = DATA + 0x20_0000;
    const OC967_MAGIC: u32 = 0x0967_c0de;
    /// Answer words: `OC967_ANSWER + 4 * k` is what fake `k` returns in v0.
    const OC967_ANSWER: u32 = OC967_GATE + 0x10;
    const K_BE: u32 = 0;
    const K_INTERFACE: u32 = 1;
    const K_HUMAN_RIDER: u32 = 2;
    const K_RIDER_CHAR: u32 = 3;
    const K_PEAK_LOCKED: u32 = 4;
    const K_MONEY: u32 = 5;
    const K_MONEY_CMP: u32 = 6;
    const K_CMP_157: u32 = 7;
    /// Bit mask: `IsFlagSet(id)` is bit `id & 31` of this word.
    const K_FLAG_MASK: u32 = 8;
    const K_CHAR_FLAG: u32 = 15;
    /// Bit mask: `IsMessageOfType(type, _)` is bit `type & 31` of this word.
    const K_OF_TYPE_MASK: u32 = 12;
    const K_INBOX: u32 = 13;
    const K_PICK_VARIANT: u32 = 16;
    const K_RAND: u32 = 18;
    const K_TABLE_OBJ: u32 = 19;
    const K_OBJ_NEXT: u32 = 20;

    /// Fixture addresses (the replay chain `gp - 0x848` -> world -> +0x84 ->
    /// +0x28 holds the replay length).
    const WORLD: u32 = DATA + 0x1000;
    const P1: u32 = DATA + 0x1100;
    const P2: u32 = DATA + 0x1200;
    const INBOX: u32 = DATA + 0x1300;
    const IFACE: u32 = DATA + 0x1400;
    const BE: u32 = DATA + 0x1500;
    const P3: u32 = DATA + 0x1600;
    const OBJ: u32 = DATA + 0x1700;
    const OBJ2: u32 = DATA + 0x1800;
    const VT: u32 = DATA + 0x1900;
    /// The aggression and group tables (0x14 bytes per entry) at `0x441630`, and
    /// the message table of `PostMessage` (`0x4c6c08 + 0x18 * msg`).
    const AGGRESSION_TABLE: u32 = 0x0044_1630;
    const MSG_TABLE: u32 = 0x004c_6c08;

    /// A fake answering with mailbox word `k` in v0, gated on `OC967_GATE`.
    macro_rules! oc967_fake {
        ($name:ident, $k:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC967_GATE) != OC967_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                c.g[V0][0] = c.bus.read32(OC967_ANSWER + 4 * $k) as u64;
                c.pc = c.g[RA][0] as u32;
            }
        };
    }

    oc967_fake!(oc967_fake_be, K_BE, ee::s01::f_0014dc80__generated);
    oc967_fake!(
        oc967_fake_interface,
        K_INTERFACE,
        ee::s01::f_0014dd58__generated
    );
    oc967_fake!(
        oc967_fake_human_rider,
        K_HUMAN_RIDER,
        ee::s01::f_00146e98__generated
    );
    oc967_fake!(
        oc967_fake_rider_char,
        K_RIDER_CHAR,
        ee::s01::f_00147398__generated
    );
    oc967_fake!(
        oc967_fake_peak_locked,
        K_PEAK_LOCKED,
        ee::s01::f_00146008__generated
    );
    oc967_fake!(oc967_fake_money, K_MONEY, ee::s01::f_001509f8__generated);
    oc967_fake!(
        oc967_fake_money_cmp,
        K_MONEY_CMP,
        ee::s01::f_001588b0__generated
    );
    oc967_fake!(
        oc967_fake_cmp_157,
        K_CMP_157,
        ee::s01::f_00157bf0__generated
    );
    oc967_fake!(oc967_fake_set_flag, 9, ee::s04::f_001e3a78__generated);
    oc967_fake!(oc967_fake_remove_group, 10, ee::s04::f_001e32c8__generated);
    oc967_fake!(oc967_fake_post, 11, ee::s04::f_001e2fe0__generated);
    oc967_fake!(
        oc967_fake_remove_message,
        14,
        ee::s04::f_001e31b8__generated
    );
    oc967_fake!(
        oc967_fake_char_flag,
        K_CHAR_FLAG,
        ee::s04::f_001e2b38__generated
    );
    oc967_fake!(
        oc967_fake_pick_variant,
        K_PICK_VARIANT,
        ee::s04::f_001e3100__generated
    );
    oc967_fake!(oc967_fake_inbox, K_INBOX, ee::s04::f_001e39f8__generated);
    oc967_fake!(oc967_fake_rand, K_RAND, ee::s10::f_003177f0__generated);
    oc967_fake!(
        oc967_fake_table_obj,
        K_TABLE_OBJ,
        ee::s10::f_00317670__generated
    );
    oc967_fake!(
        oc967_fake_obj_next,
        K_OBJ_NEXT,
        ee::s13::f_0039f9d8__generated
    );

    /// `IsFlagSet(id)`: bit `id & 31` of the flag mask.
    fn oc967_fake_flag_set(c: &mut Cpu) {
        if c.bus.read32(OC967_GATE) != OC967_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc967_fake_flag_set),
                ee::s04::f_001e3a30__generated,
            );
        }
        let mask = c.bus.read32(OC967_ANSWER + 4 * K_FLAG_MASK);
        c.g[V0][0] = ((mask >> ((c.g[A0][0] as u32) & 31)) & 1) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `IsMessageOfType(type, _)`: bit `type & 31` of the type mask.
    fn oc967_fake_of_type(c: &mut Cpu) {
        if c.bus.read32(OC967_GATE) != OC967_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc967_fake_of_type),
                ee::s04::f_001e2d58__generated,
            );
        }
        let mask = c.bus.read32(OC967_ANSWER + 4 * K_OF_TYPE_MASK);
        c.g[V0][0] = ((mask >> ((c.g[A0][0] as u32) & 31)) & 1) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// Hooks every fake into its override slot (once per process).
    fn oc967_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc967_fake_be,
            fakes::prev_cell(oc967_fake_be),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc967_fake_interface,
            fakes::prev_cell(oc967_fake_interface),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc967_fake_human_rider,
            fakes::prev_cell(oc967_fake_human_rider),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            oc967_fake_rider_char,
            fakes::prev_cell(oc967_fake_rider_char),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146008),
            oc967_fake_peak_locked,
            fakes::prev_cell(oc967_fake_peak_locked),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001509F8),
            oc967_fake_money,
            fakes::prev_cell(oc967_fake_money),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001588B0),
            oc967_fake_money_cmp,
            fakes::prev_cell(oc967_fake_money_cmp),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00157BF0),
            oc967_fake_cmp_157,
            fakes::prev_cell(oc967_fake_cmp_157),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3A30),
            oc967_fake_flag_set,
            fakes::prev_cell(oc967_fake_flag_set),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3A78),
            oc967_fake_set_flag,
            fakes::prev_cell(oc967_fake_set_flag),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E32C8),
            oc967_fake_remove_group,
            fakes::prev_cell(oc967_fake_remove_group),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E2FE0),
            oc967_fake_post,
            fakes::prev_cell(oc967_fake_post),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E2D58),
            oc967_fake_of_type,
            fakes::prev_cell(oc967_fake_of_type),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E39F8),
            oc967_fake_inbox,
            fakes::prev_cell(oc967_fake_inbox),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E31B8),
            oc967_fake_remove_message,
            fakes::prev_cell(oc967_fake_remove_message),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E2B38),
            oc967_fake_char_flag,
            fakes::prev_cell(oc967_fake_char_flag),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E3100),
            oc967_fake_pick_variant,
            fakes::prev_cell(oc967_fake_pick_variant),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003177F0),
            oc967_fake_rand,
            fakes::prev_cell(oc967_fake_rand),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc967_fake_table_obj,
            fakes::prev_cell(oc967_fake_table_obj),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F9D8),
            oc967_fake_obj_next,
            fakes::prev_cell(oc967_fake_obj_next),
        );
    }

    /// Gate on: the fakes answer from the mailbox in this machine.
    fn oc967_gate(c: &mut Cpu) {
        oc967_install();
        c.bus.write32(OC967_GATE, OC967_MAGIC);
    }

    fn oc967_answer(c: &mut Cpu, k: u32, v: u32) {
        c.bus.write32(OC967_ANSWER + 4 * k, v);
    }

    /// The gate, the replay-length chain and the answers every test shares.
    fn oc967_setup(c: &mut Cpu, replay: u32) {
        oc967_gate(c);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(WORLD + 0x84, P1);
        c.bus.write32(P1 + 0x28, P2);
        c.bus.write32(P2, replay);
        oc967_answer(c, K_BE, BE);
        oc967_answer(c, K_INTERFACE, IFACE);
        oc967_answer(c, K_HUMAN_RIDER, 0);
        oc967_answer(c, K_INBOX, INBOX);
    }

    /// The character ids the riders may have.
    const CHARS: [u32; 4] = [3, 7, 8, 5];

    #[test]
    fn message_center_post_bc_finish_message_b_matches() {
        check(
            ee::s04::f_001e2828__generated,
            message_center_post_bc_finish_message_b,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_2828, 0);
                oc967_setup(c, [0, 5, 20][(seed / 5 % 3) as usize]);
                oc967_answer(c, K_RIDER_CHAR, CHARS[(seed / 120 % 4) as usize]);
                oc967_answer(
                    c,
                    K_FLAG_MASK,
                    if seed / 30 % 2 == 0 { 0 } else { u32::MAX },
                );
                c.g[A0][0] = [0xe, 0xf, 0x10, 3, 0][(seed % 5) as usize] as u64;
                c.g[A1][0] = (seed / 60 % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_check_aggression_message_matches() {
        check(
            ee::s04::f_001e2a08__generated,
            message_center_check_aggression_message,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_2a08, 0);
                oc967_setup(c, [0, 5, 20][(seed / 7 % 3) as usize]);
                for id in 0..8u32 {
                    let entry = AGGRESSION_TABLE + 0x14 * id;
                    c.bus.write32(entry + 4, 0x100 + id);
                    c.bus.write32(entry + 8, 1 + id % 3);
                }
                oc967_answer(c, K_RIDER_CHAR, CHARS[(seed % 4) as usize]);
                oc967_answer(c, K_CHAR_FLAG, seed / 4 % 8);
                oc967_answer(
                    c,
                    K_FLAG_MASK,
                    if seed / 32 % 2 == 0 { 0 } else { u32::MAX },
                );
                oc967_answer(c, K_RAND, seed * 37 % 256);
                c.g[A0][0] = CHARS[(seed / 2 % 4) as usize] as u64;
                c.g[A1][0] = [2, 10, 16, 30][(seed / 9 % 4) as usize] as u64;
            },
        );
    }

    #[test]
    fn message_center_get_group_message_id_matches() {
        check(
            ee::s04::f_001e2c80__generated,
            message_center_get_group_message_id,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_2c80, 0);
                oc967_gate(c);
                oc967_answer(
                    c,
                    K_FLAG_MASK,
                    if seed / 48 % 2 == 0 { 0 } else { u32::MAX },
                );
                for i in 0..61u32 {
                    let entry = AGGRESSION_TABLE + 0x14 * i;
                    c.bus.write32(entry, i % 4);
                    c.bus.write32(entry + 4, 0x100 + i);
                    c.bus.write32(entry + 8, 1 + (i + seed) % 3);
                    c.bus.write32(entry + 0x10, (i + seed) % 2);
                }
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = (seed / 4 % 6) as u64;
                c.g[A2][0] = (seed / 24 % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_post_message_matches() {
        check(
            ee::s04::f_001e2fe0__generated,
            message_center_post_message,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_2fe0, 0);
                oc967_setup(c, 0);
                // The world's table object: absent, or present with a vtable whose
                // slot calls the FAKE miss hook.
                c.bus.write32(P1 + 0x48, if seed % 3 == 0 { 0 } else { P3 });
                oc967_answer(c, K_TABLE_OBJ, OBJ);
                oc967_answer(c, K_OBJ_NEXT, if seed % 5 == 0 { 0 } else { OBJ2 });
                c.bus.write32(OBJ2 + 8, VT);
                c.bus.write16(VT + 0xc0, 0);
                c.bus.write32(VT + 0xc4, FAKE);
                // Inbox counts: none, a few, or the 0x19 case that removes one.
                c.bus
                    .write8(INBOX + 0xcc, [0, 3, 25][(seed % 3) as usize] as u8);
                oc967_answer(c, K_PICK_VARIANT, 2 + seed % 3);
                let msg = [0x7f, 0x80, 0x19, 0x1a, 0x1b, 0x05, 0x83][(seed % 7) as usize];
                // The message table entry's word at +8 selects the PickVariant path.
                c.bus.write32(MSG_TABLE + 0x18 * msg + 8, seed / 2 % 2);
                c.g[A0][0] = msg as u64;
                c.g[A1][0] = (seed / 7 % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_pick_variant_matches() {
        check(
            ee::s04::f_001e3100__generated,
            message_center_pick_variant,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_3100, 0);
                oc967_gate(c);
                // Types 3, 5, 8 and 4 present or not, in every combination.
                let mask = ((seed & 1) << 3)
                    | (((seed >> 1) & 1) << 5)
                    | (((seed >> 2) & 1) << 8)
                    | (((seed >> 3) & 1) << 4);
                oc967_answer(c, K_OF_TYPE_MASK, mask);
                oc967_answer(c, K_RAND, seed * 37 % 251 + 1);
                c.g[A0][0] = (seed % 7) as u64;
            },
        );
    }

    #[test]
    fn message_center_get_message_count_matches() {
        check(
            ee::s04::f_001e33e8__generated,
            message_center_get_message_count,
            50,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_33e8, 0);
                oc967_gate(c);
                oc967_answer(c, K_INBOX, INBOX);
                c.bus.write8(INBOX + 0xcc, seed as u8);
            },
        );
    }

    #[test]
    fn message_center_check_cash_messages_matches() {
        check(
            ee::s04::f_001e3760__generated,
            message_center_check_cash_messages,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_3760, 0);
                oc967_setup(c, 0);
                oc967_answer(c, K_PEAK_LOCKED, seed / 2 % 2);
                oc967_answer(c, K_MONEY, seed % 7);
                oc967_answer(c, K_MONEY_CMP, seed / 7 % 5);
                oc967_answer(
                    c,
                    K_FLAG_MASK,
                    if seed / 35 % 2 == 0 { 0 } else { u32::MAX },
                );
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    (1 + seed % 4) as u64
                };
                c.g[A1][0] = (seed % 50) as u64;
            },
        );
    }

    #[test]
    fn message_center_check_peak_messages_matches() {
        check(
            ee::s04::f_001e38b8__generated,
            message_center_check_peak_messages,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e_38b8, 0);
                oc967_setup(c, 0);
                oc967_answer(c, K_RIDER_CHAR, CHARS[(seed % 4) as usize]);
                oc967_answer(c, K_PEAK_LOCKED, seed / 2 % 2);
                oc967_answer(c, K_CMP_157, seed / 6 % 2);
                oc967_answer(
                    c,
                    K_FLAG_MASK,
                    if seed / 12 % 2 == 0 { 0 } else { u32::MAX },
                );
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    (1 + seed % 4) as u64
                };
            },
        );
    }
}
