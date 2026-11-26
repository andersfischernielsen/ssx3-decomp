//! Staging module oc848 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc848`.

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
    0x0019_2d80,
    cfe_state_bragging_rights_init,
    "cFEStateBraggingRights_Init",
)];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cFEStateBraggingRights_SetRowsVisible` (0x192740).
    pub(crate) const SET_ROWS_VISIBLE: Callee = Callee {
        addr: 0x0019_2740,
        f: ee::s02::f_00192740,
    };
    /// `cFEStateBraggingRights_SetRowMask` (0x192968).
    pub(crate) const SET_ROW_MASK: Callee = Callee {
        addr: 0x0019_2968,
        f: ee::s02::f_00192968,
    };
    /// `cFEStateBraggingRights_HideUnusedRows` (0x192970).
    pub(crate) const HIDE_UNUSED_ROWS: Callee = Callee {
        addr: 0x0019_2970,
        f: ee::s02::f_00192970,
    };
    /// `cFEStateBraggingRights_GetRowDefault` (0x193f38).
    pub(crate) const GET_ROW_DEFAULT: Callee = Callee {
        addr: 0x0019_3f38,
        f: ee::s02::f_00193f38,
    };
}

/// `void cFEStateBraggingRights_Init(cFEStateBraggingRights* self)` (0x192d80).
///
/// Opens the "bragging rights" screen (`self+0x40`), hides the two player
/// menus when the attribute bit 4 is set, picks the row table (`self+0x738`)
/// and the row count of the current game event (jump table 0x45fd20, six
/// events), then for every row sets its label text and fills the two list
/// boxes (No/Yes, Off/5s..20s or Off/3000..15000 entries) with the current
/// back-end value as selection. Finally shows or hides the net-play widgets
/// by the net flag at `gp - 0x1680`.
pub fn cfe_state_bragging_rights_init(c: &mut Cpu) {
    if c.pc != 0x0019_2d80 {
        return ee::s02::f_00192d80__entries(c);
    }
    guest(c, |c| 'l_192d80: loop {
        'b_192dec: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A0][0] = 0x460000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x338));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00192dc0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00192dd4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_192dec;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00192dec)?;
                break 'b_192dec;
            }
        }
        'b_192e1c: {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x704));
            if t {
                break 'b_192e1c;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = 0u64;
                callee::SET_ROWS_VISIBLE.call(c, 0x00192e10)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0u64;
                callee::SET_ROWS_VISIBLE.call(c, 0x00192e1c)?;
                break 'b_192e1c;
            }
        }
        'b_192ea8: {
            c.cycles += 10;
            c.g[S5][0] = 0u64;
            call_ptr(c, 0x0014dc80, 0x00192e24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00192e30)?;
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
            c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
            let t = c.g[V1][0] == 0u64;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_192ea8;
            } else {
                'b_192ea4: {
                    'b_192e9c: {
                        'b_192e90: {
                            'b_192e84: {
                                'b_192e78: {
                                    'b_192e6c: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2e0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00192d80 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00192d80);
                                                }
                                                continue 'l_192d80;
                                            }
                                            0x00192e60 => {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[GP][0] as u32).wrapping_sub(0x19e0));
                                                let t = true;
                                                c.g[S5][0] = 0x6;
                                                if t {
                                                    break 'b_192ea4;
                                                } else {
                                                    break 'b_192e6c;
                                                }
                                            }
                                            0x00192e6c => {
                                                break 'b_192e6c;
                                            }
                                            0x00192e78 => {
                                                break 'b_192e78;
                                            }
                                            0x00192e84 => {
                                                break 'b_192e84;
                                            }
                                            0x00192e90 => {
                                                break 'b_192e90;
                                            }
                                            0x00192e9c => {
                                                break 'b_192e9c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00192d80, 0x0019330c);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19d8));
                                    let t = true;
                                    c.g[S5][0] = 0x7;
                                    if t {
                                        break 'b_192ea4;
                                    } else {
                                        break 'b_192e78;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19d0));
                                let t = true;
                                c.g[S5][0] = 0x7;
                                if t {
                                    break 'b_192ea4;
                                } else {
                                    break 'b_192e84;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19c8));
                            let t = true;
                            c.g[S5][0] = 0x7;
                            if t {
                                break 'b_192ea4;
                            } else {
                                break 'b_192e90;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19c0));
                        let t = true;
                        c.g[S5][0] = 0x5;
                        if t {
                            break 'b_192ea4;
                        } else {
                            break 'b_192e9c;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x19b8));
                    c.g[S5][0] = 0x7;
                    break 'b_192ea4;
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x738), c.g[V0][0]);
                break 'b_192ea8;
            }
        }
        'b_192ed4: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_192ed4;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0x1;
                'l_192eb8: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00192eb8);
                        }
                        continue 'l_192eb8;
                    } else {
                        break 'b_192ed4;
                    }
                }
            }
        }
        'b_193190: {
            c.cycles += 21;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SET_ROW_MASK.call(c, 0x00192ee0)?;
            c.g[S2][0] = 0u64;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S4][0];
            callee::SET_ROW_MASK.call(c, 0x00192ef0)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.g[S3][0];
            callee::HIDE_UNUSED_ROWS.call(c, 0x00192efc)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.g[S4][0];
            callee::HIDE_UNUSED_ROWS.call(c, 0x00192f08)?;
            c.g[V0][0] = 0x4a0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14f8));
            c.g[V1][0] = c.lb(c.g[A2][0] as u32);
            c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
            c.sb(c.g[SP][0] as u32, c.g[V1][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
            let t = c.g[S5][0] == 0u64;
            c.g[A0][0] = 0x4a0000;
            if t {
                break 'b_193190;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                'l_192f30: loop {
                    'b_192f88: {
                        c.cycles += 9;
                        c.g[A0][0] = c.g[SP][0];
                        c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00192f3c)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00192f48)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_192f88;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                            c.g[V0][0] = 0x440000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1108));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            call_ptr(c, 0x00317670, 0x00192f78)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00192f84)?;
                            c.g[S3][0] = 0u64;
                            break 'b_192f88;
                        }
                    }
                    c.cycles += 6;
                    c.g[S6][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S4][0] = 0x460000;
                    c.g[FP][0] = 0x4a0000;
                    c.g[V0][0] = 0x34;
                    'l_192fa0: loop {
                        'b_193170: {
                            'b_1930d4: {
                                'b_19309c: {
                                    'b_193030: {
                                        'b_192ff4: {
                                            'b_192fe0: {
                                                c.cycles += 10;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                                                c.mult(0, 4, 19, 2);
                                                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S2][0] as u32));
                                                c.g[A1][0] = c.lb(c.g[V1][0] as u32);
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                                c.g[V1][0] = ((c.g[A1][0] as i64) < 7i64) as u64;
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S6][0] as u32));
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x6d8));
                                                if t {
                                                    break 'b_192fe0;
                                                } else {
                                                    c.cycles += 2;
                                                    if (c.g[A1][0] as i64) > 0 {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_192ff4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;

                                                        if t {
                                                            break 'b_193030;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[S3][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_193170;
                                                            } else {
                                                                break 'b_192fe0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x7;
                                            let t = c.g[A1][0] == c.g[V0][0];

                                            if t {
                                                break 'b_1930d4;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_193170;
                                                } else {
                                                    break 'b_192ff4;
                                                }
                                            }
                                        }
                                        c.cycles += 15;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                        call_ptr(c, 0x00317670, 0x00192ffc)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039a568, 0x0019300c)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                        call_ptr(c, 0x00317670, 0x00193018)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = 0x1;
                                        call_ptr(c, 0x0039a568, 0x00193028)?;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                                        if t {
                                            break 'b_19309c;
                                        } else {
                                            break 'b_193030;
                                        }
                                    }
                                    c.cycles += 27;
                                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x20c0));
                                    call_ptr(c, 0x00317670, 0x00193038)?;
                                    c.g[A2][0] = 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0039a568, 0x00193048)?;
                                    c.g[A2][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x16e0));
                                    c.g[A2][0] = 0x5;
                                    call_ptr(c, 0x0039a3f8, 0x0019305c)?;
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16d8));
                                    c.g[A2][0] = 0xa;
                                    call_ptr(c, 0x0039a3f8, 0x00193070)?;
                                    c.g[V1][0] = 0x4a0000;
                                    c.g[A2][0] = 0xf;
                                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1758));
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0039a3f8, 0x00193084)?;
                                    c.g[A2][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1760));
                                    c.g[A2][0] = 0x14;
                                    call_ptr(c, 0x0039a3f8, 0x00193098)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                                    break 'b_19309c;
                                }
                                c.cycles += 14;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[A2][0] = c.lb(c.g[V1][0] as u32);
                                callee::GET_ROW_DEFAULT.call(c, 0x001930b0)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                call_ptr(c, 0x0039a768, 0x001930c0)?;
                                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0039a7a8, 0x001930cc)?;
                                let t = true;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_193170;
                                } else {
                                    break 'b_1930d4;
                                }
                            }
                            c.cycles += 39;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x20c0));
                            call_ptr(c, 0x00317670, 0x001930dc)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x0039a568, 0x001930ec)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1768));
                            c.g[A2][0] = 0xbb8;
                            call_ptr(c, 0x0039a3f8, 0x001930fc)?;
                            c.g[V0][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1770));
                            c.g[A2][0] = 0x1388;
                            call_ptr(c, 0x0039a3f8, 0x00193110)?;
                            c.g[V1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1778));
                            c.g[A2][0] = 0x2710;
                            call_ptr(c, 0x0039a3f8, 0x00193124)?;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1780));
                            c.g[A2][0] = 0x3a98;
                            call_ptr(c, 0x0039a3f8, 0x00193138)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x738));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S3][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[A2][0] = c.lb(c.g[V1][0] as u32);
                            callee::GET_ROW_DEFAULT.call(c, 0x00193150)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            call_ptr(c, 0x0039a768, 0x00193160)?;
                            c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0039a7a8, 0x0019316c)?;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            break 'b_193170;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x34;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00192fa0);
                            }
                            continue 'l_192fa0;
                        } else {
                            c.cycles += 4;
                            c.g[S2][0] = c.g[S7][0];
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00192f30);
                                }
                                continue 'l_192f30;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = 0x4a0000;
                                break 'b_193190;
                            }
                        }
                    }
                }
            }
        }
        'b_193260: {
            'b_19325c: {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1580));
                call_ptr(c, 0x00317670, 0x00193198)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001931a4)?;
                c.g[A2][0] = c.g[V0][0];
                let t = c.g[A2][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                if t {
                    break 'b_19325c;
                } else {
                    'b_193248: {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_193248;
                        } else {
                            'b_193208: {
                                c.cycles += 14;
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001931d0)?;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x318));
                                call_ptr(c, 0x00317670, 0x001931dc)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001931e8)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_193208;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00193208)?;
                                    break 'b_193208;
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x308));
                            call_ptr(c, 0x00317670, 0x00193214)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00193220)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_19325c;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00193240)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                                if t {
                                    break 'b_193260;
                                } else {
                                    break 'b_193248;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019325c)?;
                    break 'b_19325c;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
            break 'b_193260;
        }
        'b_1932e0: {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            let t = c.g[V0][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_1932e0;
            } else {
                'b_1932a4: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1588));
                    call_ptr(c, 0x00317670, 0x00193278)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00193284)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1932a4;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001932a4)?;
                        break 'b_1932a4;
                    }
                }
                'b_1932dc: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f8));
                    call_ptr(c, 0x00317670, 0x001932b0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001932bc)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1932dc;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001932dc)?;
                        break 'b_1932dc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                break 'b_1932e0;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{elf_image, put_words, DATA, GAME_GP};

    pub(crate) const OC848_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC848_SELF: u32 = DATA + 0x1_0000;
    pub(crate) const OC848_SCREEN: u32 = DATA + 0x2_0000;
    pub(crate) const OC848_OBJ: u32 = DATA + 0x3_0000;
    pub(crate) const OC848_VT: u32 = DATA + 0x4_0000;
    pub(crate) const OC848_LISTS: u32 = DATA + 0x5_0000;
    pub(crate) const OC848_ZEROS: u32 = DATA + 0x6_0000;
    pub(crate) const OC848_ENGINE: u32 = DATA + 0x7_0000;

    /// The miss hook of the fixture: logs every call like the plain fake and
    /// gives `addNewScreen` a screen (or none) and `UIScreen_FindObject` a
    /// widget with a vtable (or none), so the body's pointer chasing stays in
    /// the fixture's data.
    fn oc848_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let salt = c.bus.read32(OC848_CTRL) as u64;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x0039_7cc0 => {
                c.g[V0][0] = if salt & 1 == 1 {
                    0
                } else {
                    OC848_SCREEN as u64
                };
            }
            0x0039_d860 => {
                c.g[V0][0] = if fakes::mix(salt.wrapping_add(n as u64)) >> 7 & 3 == 0 {
                    0
                } else {
                    (OC848_OBJ + (n % 8) * 0x40) as u64
                };
            }
            _ => {}
        }
    }

    pub(crate) const OC848_GATE: u32 = OC848_CTRL + 4;
    pub(crate) const OC848_MAGIC: u32 = 0x4f43_3834;

    macro_rules! oc848_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC848_GATE) != OC848_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc848_hook(c, $addr);
                }
            )*
            /// Gated fakes of the callees (and of the file-load request they
            /// would reach), active only when the fixture writes the magic word.
            pub(crate) fn oc848_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc848_fakes!(
        (
            oc848_fk_00397cc0,
            OC848_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397CC0
        ),
        (
            oc848_fk_0039c870,
            OC848_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039C870
        ),
        (
            oc848_fk_0039d860,
            OC848_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039D860
        ),
        (
            oc848_fk_003a1000,
            OC848_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003A1000
        ),
        (
            oc848_fk_0039a568,
            OC848_PREV_0039A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039A568
        ),
        (
            oc848_fk_0039a3f8,
            OC848_PREV_0039A3F8,
            ee::s13::OVERRIDE_0039A3F8,
            ee::s13::f_0039a3f8__generated,
            0x0039A3F8
        ),
        (
            oc848_fk_0039a768,
            OC848_PREV_0039A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039A768
        ),
        (
            oc848_fk_0039a7a8,
            OC848_PREV_0039A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039A7A8
        ),
        (
            oc848_fk_00317670,
            OC848_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc848_fk_0014dc80,
            OC848_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014DC80
        ),
        (
            oc848_fk_0014dd58,
            OC848_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014DD58
        ),
        (
            oc848_fk_003de670,
            OC848_PREV_003DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003DE670
        ),
    );

    /// `cFEStateBraggingRights_Init`: every game event (the six jump table
    /// cases, out-of-range events and a corrupt table entry), with and
    /// without a screen, the attribute bit that hides the menus, found and
    /// missing widgets, net play on and off, and the unwinds of its calls.
    #[test]
    fn cfe_state_bragging_rights_init_matches() {
        let table: [u32; 6] = [
            0x0019_2e60,
            0x0019_2e6c,
            0x0019_2e84,
            0x0019_2e78,
            0x0019_2e90,
            0x0019_2e9c,
        ];
        oc848_install_fakes();
        crate::difftest::check(
            ee::s02::f_00192d80__generated,
            cfe_state_bragging_rights_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0848_192d, 90);
                c.hooks.miss = oc848_hook;
                c.bus.write32(OC848_GATE, OC848_MAGIC);
                elf_image(c);
                c.pc = 0x0019_2d80;
                c.bus.write32(OC848_CTRL, r.next_u32());
                if seed % 5 == 1 {
                    let salt = c.bus.read32(OC848_CTRL) | 1;
                    c.bus.write32(OC848_CTRL, salt);
                } else {
                    let salt = c.bus.read32(OC848_CTRL) & !1;
                    c.bus.write32(OC848_CTRL, salt);
                }
                c.g[A0][0] = sx(OC848_SELF);
                c.bus.write32(OC848_SELF + 0x10, OC848_ENGINE);
                c.bus
                    .write32(OC848_SELF + 0x1c, if seed % 3 == 0 { 0x10 } else { 0 });
                // The two player menus: child list heads in zeroed memory.
                for menu in [0x6d0u32, 0x704] {
                    c.bus.write32(OC848_SELF + menu + 8, OC848_ZEROS);
                }
                // The list box pointers (`self + 0x6d8 + player * 0x34 + row * 4`).
                for i in 0..0x24u32 {
                    c.bus
                        .write32(OC848_SELF + 0x6d8 + 4 * i, OC848_LISTS + 0x10 * i);
                }
                // Vtables of the widgets the fake `FindObject` returns.
                for n in 0..8u32 {
                    c.bus.write32(OC848_OBJ + n * 0x40 + 8, OC848_VT);
                }
                for (off, i) in [(0x40u32, 0u32), (0x48, 1)] {
                    c.bus.write16(OC848_VT + off, 0);
                    c.bus.write32(OC848_VT + off + 4, fakes::FAKE + 0x10 * i);
                }
                put_words(c, 0x0045_fd20, &table);
                let event = seed % 8;
                c.bus.write8(0x0053_5c12, event as u8);
                if seed % 29 == 0 && event < 6 {
                    c.bus.write32(0x0045_fd20 + 4 * event, 0x0019_2e00);
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x1680), (seed / 8) % 2);
            },
        );
    }
}
