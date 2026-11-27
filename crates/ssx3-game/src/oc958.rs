//! Staging module oc958 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc958`.

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
        0x001e1a30,
        message_center_match_character,
        "MessageCenter_MatchCharacter",
    ),
    (
        0x001e1dd0,
        message_center_set_peak_event_flag,
        "MessageCenter_SetPeakEventFlag",
    ),
    (
        0x001e2b38,
        message_center_get_character_flag_id,
        "MessageCenter_GetCharacterFlagId",
    ),
    (
        0x001e2ea0,
        message_center_get_sender_name_hash,
        "MessageCenter_GetSenderNameHash",
    ),
];

/// `MessageCenter_MatchCharacter` (0x001e1a30): does rider (character) id `a0`
/// (0..9, a jump table) match the id `a1`? Ids past 9 give 0; each case is
/// `a1 == n` for its own `n` (case 0 is `a1 == 0`).
pub fn message_center_match_character(c: &mut Cpu) {
    if c.pc != 0x001e1a30 {
        return ee::s04::f_001e1a30__generated(c);
    }
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A0][0] < 0xau64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x470000;
        if t {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0u64;
            c.pc = t;
            return Ok(());
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2230));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
            let t = c.g[A0][0] as u32;
            match t {
                0x001e1a54 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1a5c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x1;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1a68 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1a74 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x3;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1a80 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x4;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1a8c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x5;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1a98 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x6;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1aa4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x7;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1ab0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x8;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                0x001e1abc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] ^ 0x9;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(t, 0x001e1a30, 0x001e1ad0);
                    return Ok(());
                }
            }
        }
    });
}

/// `MessageCenter_SetPeakEventFlag` (0x001e1dd0): unless the BE peak-progress
/// counter (`world+0x84 -> +0x28 -> *`) is already 1..9, maps the peak event
/// `a1` (6..11 through a jump table) to a message flag id (0x28 + 2k) when its
/// flag (0x22 + k) is not set, and sets the flag (`+1` when `a2` is non-zero)
/// with value 1.
pub fn message_center_set_peak_event_flag(c: &mut Cpu) {
    if c.pc != 0x001e1dd0 {
        return ee::s04::f_001e1dd0__generated(c);
    }
    guest(c, |c| {
        'b_1e1dfc: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A2][0];
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_1e1dfc;
            } else {
                c.cycles += 1;
                c.g[A2][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e1dfc;
            }
        }
        'b_1e1eac: {
            c.cycles += 2;
            if c.g[A2][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_1e1eac;
            } else {
                'b_1e1e94: {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6));
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0u64;
                    if t {
                        break 'b_1e1e94;
                    } else {
                        'b_1e1e8c: {
                            'b_1e1e80: {
                                'b_1e1e70: {
                                    'b_1e1e60: {
                                        'b_1e1e50: {
                                            'b_1e1e40: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x470000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x21f0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001e1e30 => {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001e1e38);
                                                        c.g[A0][0] = 0x22;
                                                        Callee {
                                                            addr: 0x001e3a30,
                                                            f: ee::s04::MessageCenter_IsFlagSet,
                                                        }
                                                        .call(c, 0x001e1e38)?;
                                                        let t = true;
                                                        c.g[V1][0] = 0x28;
                                                        if t {
                                                            break 'b_1e1e8c;
                                                        } else {
                                                            break 'b_1e1e40;
                                                        }
                                                    }
                                                    0x001e1e40 => {
                                                        break 'b_1e1e40;
                                                    }
                                                    0x001e1e50 => {
                                                        break 'b_1e1e50;
                                                    }
                                                    0x001e1e60 => {
                                                        break 'b_1e1e60;
                                                    }
                                                    0x001e1e70 => {
                                                        break 'b_1e1e70;
                                                    }
                                                    0x001e1e80 => {
                                                        break 'b_1e1e80;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001e1dd0, 0x001e1eb8);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001e1e48);
                                            c.g[A0][0] = 0x24;
                                            Callee {
                                                addr: 0x001e3a30,
                                                f: ee::s04::MessageCenter_IsFlagSet,
                                            }
                                            .call(c, 0x001e1e48)?;
                                            let t = true;
                                            c.g[V1][0] = 0x2c;
                                            if t {
                                                break 'b_1e1e8c;
                                            } else {
                                                break 'b_1e1e50;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001e1e58);
                                        c.g[A0][0] = 0x26;
                                        Callee {
                                            addr: 0x001e3a30,
                                            f: ee::s04::MessageCenter_IsFlagSet,
                                        }
                                        .call(c, 0x001e1e58)?;
                                        let t = true;
                                        c.g[V1][0] = 0x30;
                                        if t {
                                            break 'b_1e1e8c;
                                        } else {
                                            break 'b_1e1e60;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001e1e68);
                                    c.g[A0][0] = 0x23;
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::MessageCenter_IsFlagSet,
                                    }
                                    .call(c, 0x001e1e68)?;
                                    let t = true;
                                    c.g[V1][0] = 0x2a;
                                    if t {
                                        break 'b_1e1e8c;
                                    } else {
                                        break 'b_1e1e70;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001e1e78);
                                c.g[A0][0] = 0x25;
                                Callee {
                                    addr: 0x001e3a30,
                                    f: ee::s04::MessageCenter_IsFlagSet,
                                }
                                .call(c, 0x001e1e78)?;
                                let t = true;
                                c.g[V1][0] = 0x2e;
                                if t {
                                    break 'b_1e1e8c;
                                } else {
                                    break 'b_1e1e80;
                                }
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001e1e88);
                            c.g[A0][0] = 0x27;
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::MessageCenter_IsFlagSet,
                            }
                            .call(c, 0x001e1e88)?;
                            c.g[V1][0] = 0x32;
                            break 'b_1e1e8c;
                        }
                        c.cycles += 2;
                        c.g[A2][0] = 0u64;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = c.g[V1][0];
                        }
                        break 'b_1e1e94;
                    }
                }
                'b_1e1ea8: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_1e1ea8;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[RA][0] = sx(0x001e1ea8);
                        if c.g[S0][0] == 0 {
                            c.g[A0][0] = c.g[A2][0];
                        }
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e1ea8)?;
                        break 'b_1e1ea8;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_1e1eac;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `MessageCenter_GetCharacterFlagId` (0x001e2b38): message flag id of
/// character `a0` (0..10 -> 0xc..0x15 through a jump table); anything else
/// (and case 10) gives 0x3d.
pub fn message_center_get_character_flag_id(c: &mut Cpu) {
    if c.pc != 0x001e2b38 {
        return ee::s04::f_001e2b38__generated(c);
    }
    guest(c, |c| {
        'b_1e2bac: {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A0][0] < 0xbu64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x470000;
            if t {
                break 'b_1e2bac;
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x21d0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                match t {
                    0x001e2b5c => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0xc;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b64 => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0xd;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b6c => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0xe;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b74 => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0xf;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b7c => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0x10;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b84 => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0x11;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b8c => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0x12;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b94 => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0x13;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2b9c => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0x14;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2ba4 => {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0x15;
                        c.pc = t;
                        return Ok(());
                    }
                    0x001e2bac => {
                        break 'b_1e2bac;
                    }
                    _ => {
                        c.jump_out(t, 0x001e2b38, 0x001e2bb4);
                        return Ok(());
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0x3d;
        c.pc = t;
        return Ok(());
    });
}

/// `MessageCenter_GetSenderNameHash` (0x001e2ea0): `StrHash` of the sender
/// name string of sender `a0` (0..12 through a jump table), 0 past it.
pub fn message_center_get_sender_name_hash(c: &mut Cpu) {
    if c.pc != 0x001e2ea0 {
        return ee::s04::f_001e2ea0__generated(c);
    }
    guest(c, |c| {
        'b_1e2fd8: {
            'b_1e2fd0: {
                c.cycles += 4;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V0][0] = (c.g[A0][0] < 0xdu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1e2fd0;
                } else {
                    'b_1e2fbc: {
                        'b_1e2fa8: {
                            'b_1e2f94: {
                                'b_1e2f80: {
                                    'b_1e2f6c: {
                                        'b_1e2f58: {
                                            'b_1e2f44: {
                                                'b_1e2f30: {
                                                    'b_1e2f1c: {
                                                        'b_1e2f08: {
                                                            'b_1e2ef4: {
                                                                'b_1e2ee0: {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = 0x470000;
                                                                    c.g[V1][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x20d0));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[V0][0] as u32),
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[V1][0] as u32));
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x001e2ecc => {
                                                                            c.cycles += 5;
                                                                            c.g[A0][0] = 0x470000;
                                                                            c.g[RA][0] =
                                                                                sx(0x001e2ed8);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x21a0,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x00317670,
                                                                                0x001e2ed8,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[RA][0] = c
                                                                                .ld((c.g[SP][0]
                                                                                    as u32));
                                                                            if t {
                                                                                break 'b_1e2fd8;
                                                                            } else {
                                                                                break 'b_1e2ee0;
                                                                            }
                                                                        }
                                                                        0x001e2ee0 => {
                                                                            break 'b_1e2ee0;
                                                                        }
                                                                        0x001e2ef4 => {
                                                                            break 'b_1e2ef4;
                                                                        }
                                                                        0x001e2f08 => {
                                                                            break 'b_1e2f08;
                                                                        }
                                                                        0x001e2f1c => {
                                                                            break 'b_1e2f1c;
                                                                        }
                                                                        0x001e2f30 => {
                                                                            break 'b_1e2f30;
                                                                        }
                                                                        0x001e2f44 => {
                                                                            break 'b_1e2f44;
                                                                        }
                                                                        0x001e2f58 => {
                                                                            break 'b_1e2f58;
                                                                        }
                                                                        0x001e2f6c => {
                                                                            break 'b_1e2f6c;
                                                                        }
                                                                        0x001e2f80 => {
                                                                            break 'b_1e2f80;
                                                                        }
                                                                        0x001e2f94 => {
                                                                            break 'b_1e2f94;
                                                                        }
                                                                        0x001e2fa8 => {
                                                                            break 'b_1e2fa8;
                                                                        }
                                                                        0x001e2fbc => {
                                                                            break 'b_1e2fbc;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x001e2ea0,
                                                                                0x001e2fe0,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 5;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[RA][0] = sx(0x001e2eec);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x2190));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001e2eec,
                                                                )?;
                                                                let t = true;
                                                                c.g[RA][0] =
                                                                    c.ld((c.g[SP][0] as u32));
                                                                if t {
                                                                    break 'b_1e2fd8;
                                                                } else {
                                                                    break 'b_1e2ef4;
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[A0][0] = 0x470000;
                                                            c.g[RA][0] = sx(0x001e2f00);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x2180));
                                                            call_ptr(c, 0x00317670, 0x001e2f00)?;
                                                            let t = true;
                                                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                                            if t {
                                                                break 'b_1e2fd8;
                                                            } else {
                                                                break 'b_1e2f08;
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[A0][0] = 0x470000;
                                                        c.g[RA][0] = sx(0x001e2f14);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x2170));
                                                        call_ptr(c, 0x00317670, 0x001e2f14)?;
                                                        let t = true;
                                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                                        if t {
                                                            break 'b_1e2fd8;
                                                        } else {
                                                            break 'b_1e2f1c;
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x001e2f28);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x2160));
                                                    call_ptr(c, 0x00317670, 0x001e2f28)?;
                                                    let t = true;
                                                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                                    if t {
                                                        break 'b_1e2fd8;
                                                    } else {
                                                        break 'b_1e2f30;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = 0x470000;
                                                c.g[RA][0] = sx(0x001e2f3c);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x2150));
                                                call_ptr(c, 0x00317670, 0x001e2f3c)?;
                                                let t = true;
                                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                                if t {
                                                    break 'b_1e2fd8;
                                                } else {
                                                    break 'b_1e2f44;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] = 0x470000;
                                            c.g[RA][0] = sx(0x001e2f50);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x2140));
                                            call_ptr(c, 0x00317670, 0x001e2f50)?;
                                            let t = true;
                                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                            if t {
                                                break 'b_1e2fd8;
                                            } else {
                                                break 'b_1e2f58;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = 0x470000;
                                        c.g[RA][0] = sx(0x001e2f64);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2130));
                                        call_ptr(c, 0x00317670, 0x001e2f64)?;
                                        let t = true;
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                        if t {
                                            break 'b_1e2fd8;
                                        } else {
                                            break 'b_1e2f6c;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001e2f78);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2120));
                                    call_ptr(c, 0x00317670, 0x001e2f78)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                    if t {
                                        break 'b_1e2fd8;
                                    } else {
                                        break 'b_1e2f80;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x001e2f8c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2110));
                                call_ptr(c, 0x00317670, 0x001e2f8c)?;
                                let t = true;
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                if t {
                                    break 'b_1e2fd8;
                                } else {
                                    break 'b_1e2f94;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x001e2fa0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2100));
                            call_ptr(c, 0x00317670, 0x001e2fa0)?;
                            let t = true;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                            if t {
                                break 'b_1e2fd8;
                            } else {
                                break 'b_1e2fa8;
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001e2fb4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20f0));
                        call_ptr(c, 0x00317670, 0x001e2fb4)?;
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_1e2fd8;
                        } else {
                            break 'b_1e2fbc;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001e2fc8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20e0));
                    call_ptr(c, 0x00317670, 0x001e2fc8)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_1e2fd8;
                    } else {
                        break 'b_1e2fd0;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_1e2fd8;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, fresh, run_generated, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The fixture's control words: the gate of the fakes, the answer of the
    /// flag query, and the BE progress counter chain the peak flag reads.
    pub(crate) const OC958_CTRL: u32 = DATA + 0x28_0000;
    pub(crate) const OC958_GATE: u32 = OC958_CTRL;
    pub(crate) const OC958_FLAG: u32 = OC958_CTRL + 0x10;
    pub(crate) const OC958_MAGIC: u32 = 0x4f43_3938;
    pub(crate) const OC958_WORLD: u32 = DATA + 0x1000;
    pub(crate) const OC958_BE: u32 = DATA + 0x1100;
    pub(crate) const OC958_COUNTER_PTR: u32 = DATA + 0x1200;

    /// The fake callee: logs like the plain fake; `MessageCenter_IsFlagSet`
    /// answers with the fixture's flag word, `StrHash` with its argument.
    pub(crate) fn oc958_hook(c: &mut Cpu, target: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let flag = c.bus.read32(OC958_FLAG);
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x001e_3a30 => c.g[V0][0] = flag as u64,
            0x0031_7670 => c.g[V0][0] = a0,
            _ => {}
        }
    }

    macro_rules! oc958_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC958_GATE) != OC958_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc958_hook(c, $addr);
                }
            )*
            pub(crate) fn oc958_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc958_fakes!(
        (
            oc958_fk_1e3a30,
            OC958_PREV_1E3A30,
            ee::s04::OVERRIDE_001E3A30,
            ee::s04::f_001e3a30__generated,
            0x001e_3a30
        ),
        (
            oc958_fk_1e3a78,
            OC958_PREV_1E3A78,
            ee::s04::OVERRIDE_001E3A78,
            ee::s04::f_001e3a78__generated,
            0x001e_3a78
        ),
        (
            oc958_fk_317670,
            OC958_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
    );

    /// Distinct cycle totals of the generated body over `seeds` fixtures (no
    /// unwinds): at least `min` paths must be taken.
    fn oc958_paths(generated: fn(&mut Cpu), seeds: u32, min: usize, setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "only {} paths: {seen:?}", seen.len());
    }

    // ---------------------------------------------------------------------
    // MessageCenter_MatchCharacter
    // ---------------------------------------------------------------------

    fn oc958_match_fixture(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x1e1a30, 0);
        elf_image(c);
        c.pc = 0x001e_1a30;
        c.g[A0][0] = (seed % 14) as u64;
        c.g[A1][0] = ((seed / 14) % 12) as u64;
    }

    /// Every id (0..9 and past the table) against every compare value.
    #[test]
    fn message_center_match_character_matches() {
        check(
            ee::s04::f_001e1a30__generated,
            message_center_match_character,
            14 * 12,
            &oc958_match_fixture,
        );
    }

    #[test]
    fn message_center_match_character_paths() {
        oc958_paths(
            ee::s04::f_001e1a30__generated,
            14 * 12,
            3,
            &oc958_match_fixture,
        );
    }

    // ---------------------------------------------------------------------
    // MessageCenter_GetCharacterFlagId
    // ---------------------------------------------------------------------

    fn oc958_flag_id_fixture(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x1e2b38, 0);
        elf_image(c);
        c.pc = 0x001e_2b38;
        c.g[A0][0] = (seed % 14) as u64;
    }

    /// Every character (the 11 table cases, the last one falling through to
    /// the default) and ids past the table.
    #[test]
    fn message_center_get_character_flag_id_matches() {
        check(
            ee::s04::f_001e2b38__generated,
            message_center_get_character_flag_id,
            28,
            &oc958_flag_id_fixture,
        );
    }

    #[test]
    fn message_center_get_character_flag_id_paths() {
        oc958_paths(
            ee::s04::f_001e2b38__generated,
            28,
            2,
            &oc958_flag_id_fixture,
        );
    }

    // ---------------------------------------------------------------------
    // MessageCenter_GetSenderNameHash
    // ---------------------------------------------------------------------

    fn oc958_sender_fixture(c: &mut Cpu, seed: u32, gate: bool, calls: u32) {
        let _ = machine(c, seed, 0x1e2ea0, calls);
        elf_image(c);
        oc958_install_fakes();
        if gate {
            c.bus.write32(OC958_GATE, OC958_MAGIC);
        }
        c.pc = 0x001e_2ea0;
        c.g[A0][0] = (seed % 16) as u64;
    }

    /// The real `StrHash` of every sender name (and ids past the table).
    #[test]
    fn message_center_get_sender_name_hash_matches() {
        check(
            ee::s04::f_001e2ea0__generated,
            message_center_get_sender_name_hash,
            16,
            &|c, seed| oc958_sender_fixture(c, seed, false, 0),
        );
    }

    /// The same with the hash call faked, so unwinds inside it are compared.
    #[test]
    fn message_center_get_sender_name_hash_unwinds_match() {
        check(
            ee::s04::f_001e2ea0__generated,
            message_center_get_sender_name_hash,
            96,
            &|c, seed| oc958_sender_fixture(c, seed, true, 1),
        );
    }

    #[test]
    fn message_center_get_sender_name_hash_paths() {
        oc958_paths(ee::s04::f_001e2ea0__generated, 16, 3, &|c, seed| {
            oc958_sender_fixture(c, seed, true, 0)
        });
    }

    // ---------------------------------------------------------------------
    // MessageCenter_SetPeakEventFlag
    // ---------------------------------------------------------------------

    /// Progress counter 0 / 5 / 12 (proceed, skip, proceed), event 0..13
    /// (6..11 through the table), set-flag argument, flag query answer.
    fn oc958_peak_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let _ = machine(c, seed, 0x1e1dd0, calls);
        elf_image(c);
        oc958_install_fakes();
        c.bus.write32(OC958_GATE, OC958_MAGIC);
        c.bus.write32(OC958_FLAG, (seed / 84) % 2);
        c.bus.write32(GAME_GP_WORLD, OC958_WORLD);
        c.bus.write32(OC958_WORLD + 0x84, OC958_BE);
        c.bus.write32(OC958_BE + 0x28, OC958_COUNTER_PTR);
        c.bus
            .write32(OC958_COUNTER_PTR, [0, 5, 12][(seed % 3) as usize]);
        c.pc = 0x001e_1dd0;
        c.g[A1][0] = ((seed / 3) % 14) as u64;
        c.g[A2][0] = ((seed / 42) % 2) as u64;
    }

    const GAME_GP_WORLD: u32 = crate::difftest::GAME_GP.wrapping_sub(0x848);

    /// Skipped when the counter is 1..9; otherwise each peak event's flag
    /// query and the set-flag call, with and without the `+1`, plus unwinds
    /// inside both calls.
    #[test]
    fn message_center_set_peak_event_flag_matches() {
        check(
            ee::s04::f_001e1dd0__generated,
            message_center_set_peak_event_flag,
            336,
            &|c, seed| oc958_peak_fixture(c, seed, if seed % 5 < 2 { 2 } else { 0 }),
        );
    }

    #[test]
    fn message_center_set_peak_event_flag_paths() {
        oc958_paths(ee::s04::f_001e1dd0__generated, 336, 6, &|c, seed| {
            oc958_peak_fixture(c, seed, 0)
        });
    }
}
