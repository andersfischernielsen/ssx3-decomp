//! Staging module oc924 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc924`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void cFEStateNetConfig_ScanNextCard(this)` (0x1b4470): one step of the
/// memory card scan. With a card present (`IsAvailable`) and formatted it sets
/// the screen state to 4. A first pass (counter `gp-0x1570` zero) shows the
/// checking-card popup (state 2); the next pass reads the scan result
/// (`this+0x70c`): anything but 1 goes to `HandleLoadError` and flips the
/// counter, 1 hides the list and shows the card-error popup (0x840 / 0x841).
pub fn c_fe_state_net_config_scan_next_card(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b45dc: {
            'b_1b45d4: {
                'b_1b4528: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001b4498);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001b4498)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                    if t {
                        break 'b_1b4528;
                    } else {
                        'b_1b44c8: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                            if c.g[V1][0] != 0u64 {
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70c));
                                break 'b_1b44c8;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = 0x1;
                                c.g[A0][0] = c.g[S0][0];
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1570), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001b44c0);
                                c.g[A1][0] = 0x2;
                                Callee {
                                    addr: 0x001b47c0,
                                    f: ee::s03::cFEStateNetConfig_ShowCheckingCardPopup,
                                }
                                .call(c, 0x001b44c0)?;
                                let t = true;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_1b45d4;
                                } else {
                                    break 'b_1b44c8;
                                }
                            }
                        }
                        'b_1b4500: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x708), 0u64);
                            if t {
                                break 'b_1b4500;
                            } else {
                                c.cycles += 11;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 31);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                c.g[RA][0] = sx(0x001b44f8);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1570), c.g[V1][0]);
                                Callee {
                                    addr: 0x001b4fa0,
                                    f: ee::s03::cFEStateNetConfig_HandleLoadError,
                                }
                                .call(c, 0x001b44f8)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_1b45dc;
                                } else {
                                    break 'b_1b4500;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001b450c);
                        c.g[A1][0] = 0xffffffffffffffff;
                        Callee {
                            addr: 0x001b5068,
                            f: ee::s03::cFEStateNetConfig_SetListVisible,
                        }
                        .call(c, 0x001b450c)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x840;
                        c.g[A2][0] = 0x2;
                        c.g[RA][0] = sx(0x001b4520);
                        c.g[A3][0] = 0xffffffffffffffff;
                        Callee {
                            addr: 0x001b4888,
                            f: ee::s03::cFEStateNetConfig_ShowCardErrorPopup,
                        }
                        .call(c, 0x001b4520)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1b45dc;
                        } else {
                            break 'b_1b4528;
                        }
                    }
                }
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001b453c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b453c)?;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = 0x4;
                    break 'b_1b45d4;
                } else {
                    'b_1b4574: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                        if c.g[V1][0] != 0u64 {
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70c));
                            break 'b_1b4574;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = 0xffffffffffffff38;
                            c.g[V1][0] = 0x1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x70c), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1570), c.g[V1][0]);
                            c.g[RA][0] = sx(0x001b456c);
                            c.g[A1][0] = 0x2;
                            Callee {
                                addr: 0x001b47c0,
                                f: ee::s03::cFEStateNetConfig_ShowCheckingCardPopup,
                            }
                            .call(c, 0x001b456c)?;
                            let t = true;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1b45d4;
                            } else {
                                break 'b_1b4574;
                            }
                        }
                    }
                    'b_1b45c4: {
                        'b_1b45ac: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x708), 0u64);
                            if t {
                                break 'b_1b45ac;
                            } else {
                                c.cycles += 11;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 31);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                c.g[RA][0] = sx(0x001b45a4);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1570), c.g[V1][0]);
                                Callee {
                                    addr: 0x001b4fa0,
                                    f: ee::s03::cFEStateNetConfig_HandleLoadError,
                                }
                                .call(c, 0x001b45a4)?;
                                let t = true;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_1b45c4;
                                } else {
                                    break 'b_1b45ac;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x841;
                        c.g[A2][0] = 0x14;
                        c.g[RA][0] = sx(0x001b45c0);
                        c.g[A3][0] = 0x1;
                        Callee {
                            addr: 0x001b4888,
                            f: ee::s03::cFEStateNetConfig_ShowCardErrorPopup,
                        }
                        .call(c, 0x001b45c0)?;
                        c.g[A0][0] = c.g[S0][0];
                        break 'b_1b45c4;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b45cc);
                    c.g[A1][0] = 0xffffffffffffffff;
                    Callee {
                        addr: 0x001b5068,
                        f: ee::s03::cFEStateNetConfig_SetListVisible,
                    }
                    .call(c, 0x001b45cc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1b45dc;
                    } else {
                        break 'b_1b45d4;
                    }
                }
            }
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x708), c.g[V0][0]);
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1b45dc;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateNetConfig_StartConnect(this)` (0x1b45e8): with no card
/// available it shows the "memory card not detected" popup (text built with
/// `UStr_Sprintf` from the card slot number and "Your Network"); with a card
/// it shows the "connecting to ISP" popup with a Cancel option and starts the
/// connection for the selected setting (`Net_StartConnect`).
pub fn c_fe_state_net_config_start_connect(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b4798: {
            'b_1b471c: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S6][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001b4628);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b4628)?;
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_1b471c;
                } else {
                    c.cycles += 59;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x708), 0u64);
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x001b4644);
                    c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x71c));
                    Callee {
                        addr: 0x001b5068,
                        f: ee::s03::cFEStateNetConfig_SetListVisible,
                    }
                    .call(c, 0x001b4644)?;
                    c.g[RA][0] = sx(0x001b464c);
                    c.g[A0][0] = c.g[S6][0];
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001b464c)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f08));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001b4674);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001b4674)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = c.g[V0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001b4684);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, t, 0x001b4684)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f20));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001b46a8);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001b46a8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001b46b8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001b46b8)?;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001b46d0);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002c26d0, 0x001b46d0)?;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc4), c.g[S4][0]);
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x820));
                    c.g[RA][0] = sx(0x001b46ec);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001b46ec)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b46fc);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x001c57c0,
                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                    }
                    .call(c, 0x001b46fc)?;
                    c.g[V0][0] = 0x843;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x164), 0u64);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x001b4714);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a9bc0,
                        f: ee::s03::NetFEState_ShowPopup,
                    }
                    .call(c, 0x001b4714)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    if t {
                        break 'b_1b4798;
                    } else {
                        break 'b_1b471c;
                    }
                }
            }
            c.cycles += 31;
            c.g[RA][0] = sx(0x001b4724);
            c.g[A0][0] = c.g[S6][0];
            Callee {
                addr: 0x001a9c50,
                f: ee::s03::NetFEState_GetPopup,
            }
            .call(c, 0x001b4724)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x460000;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f38));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
            c.g[RA][0] = sx(0x001b4740);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x00317670, 0x001b4740)?;
            c.g[V1][0] = 0x3;
            c.g[A1][0] = 0x80e;
            c.g[A0][0] = 0x460000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A1][0]);
            c.g[RA][0] = sx(0x001b4760);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001b4760)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001b4770);
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x001c57c0,
                f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
            }
            .call(c, 0x001b4770)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001b477c);
            c.g[A0][0] = c.g[S6][0];
            Callee {
                addr: 0x001a9bc0,
                f: ee::s03::NetFEState_ShowPopup,
            }
            .call(c, 0x001b477c)?;
            c.g[RA][0] = sx(0x001b4784);
            call_ptr(c, 0x002591b8, 0x001b4784)?;
            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x6d0));
            c.g[RA][0] = sx(0x001b4790);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002636a8, 0x001b4790)?;
            c.sw((c.g[S6][0] as u32).wrapping_add(0x708), 0u64);
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1b4798;
        }
        c.cycles += 9;
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
    });
}

/// `void cFEStateNetConfig_OnWidgetCreated(this, item)` (0x1b52c8): the
/// screen's widgets are matched by name hash (`item+0x38`) against the ten
/// known names; each match sets the item's misc data, visibility/enable flags
/// through its vtable, or stores the widget in `this` (`0x6e4..0x6fc`).
pub fn c_fe_state_net_config_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b54a4: {
            'b_1b53bc: {
                'b_1b53a4: {
                    'b_1b5328: {
                        c.cycles += 13;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S2][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[S1][0] = c.g[A1][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[RA][0] = sx(0x001b52f4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f98));
                        call_ptr(c, 0x00317670, 0x001b52f4)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b5328;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001b531c);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b531c)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            let t = true;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1b53a4;
                            } else {
                                break 'b_1b5328;
                            }
                        }
                    }
                    'b_1b538c: {
                        'b_1b5380: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[RA][0] = sx(0x001b5334);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1520));
                            call_ptr(c, 0x00317670, 0x001b5334)?;
                            let t = c.g[S0][0] == c.g[V0][0];
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_1b5380;
                            } else {
                                'b_1b5368: {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x460000;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[RA][0] = sx(0x001b534c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3fb0));
                                    call_ptr(c, 0x00317670, 0x001b534c)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1b5368;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                        let t = true;
                                        c.g[A1][0] = 0u64;
                                        if t {
                                            break 'b_1b538c;
                                        } else {
                                            break 'b_1b5368;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                c.g[RA][0] = sx(0x001b5374);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3fc0));
                                call_ptr(c, 0x00317670, 0x001b5374)?;
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1b53bc;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    break 'b_1b5380;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[A1][0] = 0x1;
                        break 'b_1b538c;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001b539c);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001b539c)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    break 'b_1b53a4;
                }
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001b53b4);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b53b4)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1b54a4;
                } else {
                    break 'b_1b53bc;
                }
            }
            'b_1b54a0: {
                'b_1b545c: {
                    'b_1b5440: {
                        'b_1b53e4: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[RA][0] = sx(0x001b53c8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f88));
                            call_ptr(c, 0x00317670, 0x001b53c8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1b53e4;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x64;
                                c.g[A1][0] = 0x1;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6e8), c.g[S1][0]);
                                if t {
                                    break 'b_1b5440;
                                } else {
                                    break 'b_1b53e4;
                                }
                            }
                        }
                        'b_1b5400: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[RA][0] = sx(0x001b53f0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3fd8));
                            call_ptr(c, 0x00317670, 0x001b53f0)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1b5400;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6f8), c.g[S1][0]);
                                if t {
                                    break 'b_1b54a0;
                                } else {
                                    break 'b_1b5400;
                                }
                            }
                        }
                        'b_1b541c: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[RA][0] = sx(0x001b540c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ff0));
                            call_ptr(c, 0x00317670, 0x001b540c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1b541c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6fc), c.g[S1][0]);
                                if t {
                                    break 'b_1b54a0;
                                } else {
                                    break 'b_1b541c;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[RA][0] = sx(0x001b5428);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4000));
                        call_ptr(c, 0x00317670, 0x001b5428)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b545c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x65;
                            c.g[A1][0] = 0x1;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6e4), c.g[S1][0]);
                            break 'b_1b5440;
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001b5454);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001b5454)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1b54a4;
                    } else {
                        break 'b_1b545c;
                    }
                }
                'b_1b548c: {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001b5468);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4010));
                    call_ptr(c, 0x00317670, 0x001b5468)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        break 'b_1b548c;
                    } else {
                        c.cycles += 7;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6f4), c.g[S1][0]);
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = c.g[V0][0] | 0x80;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        if t {
                            break 'b_1b54a0;
                        } else {
                            break 'b_1b548c;
                        }
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001b5498);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4020));
                call_ptr(c, 0x00317670, 0x001b5498)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), c.g[S1][0]);
                    break 'b_1b54a0;
                } else {
                    break 'b_1b54a0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b54a4;
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

/// `void cFEStateNetConfig_OnConnectingPopupEvent(this, popup, event)`
/// (0x1b55b0): events 0xF and 0x14 cancel the connection (buddy and lobby
/// manager disconnect, `Net_DisconnectAll`, pair-settings widget made
/// non-static); 0x16 pushes the auth popup unless one is pending; everything
/// else is forwarded to `NetFEState_OnChildEvent`.
pub fn c_fe_state_net_config_on_connecting_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b5728: {
            'b_1b571c: {
                'b_1b5718: {
                    'b_1b56a8: {
                        'b_1b560c: {
                            'b_1b5608: {
                                c.cycles += 10;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                c.g[V0][0] = 0x14;
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[S0][0] = c.g[A0][0];
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[S1][0] = c.g[A2][0];
                                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[S2][0] = c.g[A1][0];
                                if t {
                                    break 'b_1b5608;
                                } else {
                                    'b_1b55f4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[S1][0] < 0x15u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0xf;
                                        if t {
                                            break 'b_1b55f4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1b560c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_1b5718;
                                                } else {
                                                    break 'b_1b55f4;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x16;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_1b56a8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A1][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1b571c;
                                        } else {
                                            break 'b_1b5608;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_1b560c;
                        }
                        'b_1b5640: {
                            c.cycles += 11;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6ec));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001b562c);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b562c)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_1b5640;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001b5640);
                                call_ptr(c, 0x00260f80, 0x001b5640)?;
                                break 'b_1b5640;
                            }
                        }
                        'b_1b565c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b5648);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x00261460, 0x001b5648)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1b565c;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x001b5658);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x002613c8, 0x001b5658)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1b565c;
                            }
                        }
                        'b_1b566c: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_1b566c;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001b566c);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                call_ptr(c, 0x0025b800, 0x001b566c)?;
                                break 'b_1b566c;
                            }
                        }
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x001b5674);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x00261408, 0x001b5674)?;
                        c.g[RA][0] = sx(0x001b567c);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025b800, 0x001b567c)?;
                        c.g[RA][0] = sx(0x001b5684);
                        call_ptr(c, 0x002591b8, 0x001b5684)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001b5690);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x002636f0, 0x001b5690)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001b56a0);
                        c.g[A2][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001a97b8,
                            f: ee::s03::NetFEState_OnChildEvent,
                        }
                        .call(c, 0x001b56a0)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1b5728;
                        } else {
                            break 'b_1b56a8;
                        }
                    }
                    'b_1b5700: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                        if c.g[V0][0] != 0u64 {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
                            break 'b_1b5700;
                        } else {
                            c.cycles += 19;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x700;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4080));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x001b56cc);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001b56cc)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001b56dc);
                            c.g[A2][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001b6288,
                                f: ee::s03::NetConfigAuthPopup_ctor,
                            }
                            .call(c, 0x001b56dc)?;
                            c.g[V1][0] = 0x81a;
                            c.g[A1][0] = c.g[V0][0];
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001b56fc);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f290, 0x001b56fc)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
                            break 'b_1b5700;
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001b5710);
                    c.g[A2][0] = 0x16;
                    Callee {
                        addr: 0x001a97b8,
                        f: ee::s03::NetFEState_OnChildEvent,
                    }
                    .call(c, 0x001b5710)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1b5728;
                    } else {
                        break 'b_1b5718;
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[S2][0];
                break 'b_1b571c;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b5724);
            c.g[A2][0] = c.g[S1][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b5724)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b5728;
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

/// `void cFEStateNetConfig_OnDisconnectPopupEvent(this, popup, event)`
/// (0x1b5740): events 0xF and 0x14 reset the session (buddy reconnect when
/// the buddy manager is not connected, lobby manager and buddy disconnect,
/// `Net_DisconnectAll`), then every event is forwarded to
/// `NetFEState_OnChildEvent`.
pub fn c_fe_state_net_config_on_disconnect_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b581c: {
            'b_1b580c: {
                'b_1b5774: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0xf;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[S2][0] = c.g[A1][0];
                    if t {
                        break 'b_1b5774;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x14;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_1b580c;
                        } else {
                            break 'b_1b5774;
                        }
                    }
                }
                'b_1b5788: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1b5788;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001b5788);
                        call_ptr(c, 0x00260f80, 0x001b5788)?;
                        break 'b_1b5788;
                    }
                }
                'b_1b57a4: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b5790);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x00261460, 0x001b5790)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    if t {
                        break 'b_1b57a4;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x001b57a0);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x002613c8, 0x001b57a0)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        break 'b_1b57a4;
                    }
                }
                'b_1b57b4: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1b57b4;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001b57b4);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025b800, 0x001b57b4)?;
                        break 'b_1b57b4;
                    }
                }
                c.cycles += 22;
                c.g[RA][0] = sx(0x001b57bc);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261408, 0x001b57bc)?;
                c.g[RA][0] = sx(0x001b57c4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001b57c4)?;
                c.g[RA][0] = sx(0x001b57cc);
                call_ptr(c, 0x002591b8, 0x001b57cc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001b57d8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x002636f0, 0x001b57d8)?;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001b57f4);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b57f4)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001b5804);
                c.g[A2][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b5804)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1b581c;
                } else {
                    break 'b_1b580c;
                }
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001b5818);
            c.g[A2][0] = c.g[S0][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b5818)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b581c;
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

/// `void cFEStateNetConfig_OnCreateConfigPopupEvent(this, popup, event)`
/// (0x1b5bd8): event 0xF forwards it, purges the front-end audio, runs
/// `func_00256088`, greys out the "create config" text and sets the screen
/// mode to 7; the other events are only forwarded.
pub fn c_fe_state_net_config_on_create_config_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b5ce8: {
            'b_1b5ce4: {
                'b_1b5cdc: {
                    'b_1b5ccc: {
                        c.cycles += 7;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.g[V0][0] = 0x10;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S1][0] = c.g[A0][0];
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                        if t {
                            break 'b_1b5ccc;
                        } else {
                            'b_1b5c24: {
                                'b_1b5c10: {
                                    c.cycles += 3;
                                    c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0xf;
                                    if t {
                                        break 'b_1b5c10;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[A2][0] == c.g[V0][0] {
                                            c.g[A2][0] = 0xf;
                                            break 'b_1b5c24;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_1b5cdc;
                                            } else {
                                                break 'b_1b5c10;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x14;
                                let t = c.g[A2][0] == c.g[V0][0];

                                if t {
                                    break 'b_1b5ccc;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1b5cdc;
                                    } else {
                                        break 'b_1b5c24;
                                    }
                                }
                            }
                            'b_1b5cb4: {
                                c.cycles += 17;
                                c.g[RA][0] = sx(0x001b5c2c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001a97b8,
                                    f: ee::s03::NetFEState_OnChildEvent,
                                }
                                .call(c, 0x001b5c2c)?;
                                c.g[RA][0] = sx(0x001b5c34);
                                call_ptr(c, 0x0028b180, 0x001b5c34)?;
                                c.g[RA][0] = sx(0x001b5c3c);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x00286200, 0x001b5c3c)?;
                                c.g[RA][0] = sx(0x001b5c44);
                                call_ptr(c, 0x00256088, 0x001b5c44)?;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x001b5c50);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f98));
                                call_ptr(c, 0x00317670, 0x001b5c50)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001b5c5c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001b5c5c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1b5cb4;
                                } else {
                                    c.cycles += 19;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b5c7c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b5c7c)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[AT][0] = 0x3f000000;
                                    c.mtc1(1, 0);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 1);
                                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x58));
                                    c.swc1(1, (c.g[SP][0] as u32));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001b5cb4);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b5cb4)?;
                                    break 'b_1b5cb4;
                                }
                            }
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V1][0] = 0xffffffffffffc0ff;
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            if t {
                                break 'b_1b5ce4;
                            } else {
                                break 'b_1b5ccc;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b5cd4);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001a97b8,
                        f: ee::s03::NetFEState_OnChildEvent,
                    }
                    .call(c, 0x001b5cd4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1b5ce8;
                    } else {
                        break 'b_1b5cdc;
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001b5ce4);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b5ce4)?;
                break 'b_1b5ce4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b5ce8;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_4470,
        c_fe_state_net_config_scan_next_card,
        "cFEStateNetConfig_ScanNextCard",
    ),
    (
        0x001b_45e8,
        c_fe_state_net_config_start_connect,
        "cFEStateNetConfig_StartConnect",
    ),
    (
        0x001b_52c8,
        c_fe_state_net_config_on_widget_created,
        "cFEStateNetConfig_OnWidgetCreated",
    ),
    (
        0x001b_55b0,
        c_fe_state_net_config_on_connecting_popup_event,
        "cFEStateNetConfig_OnConnectingPopupEvent",
    ),
    (
        0x001b_5740,
        c_fe_state_net_config_on_disconnect_popup_event,
        "cFEStateNetConfig_OnDisconnectPopupEvent",
    ),
    (
        0x001b_5bd8,
        c_fe_state_net_config_on_create_config_popup_event,
        "cFEStateNetConfig_OnCreateConfigPopupEvent",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control block (gate, forced-zero bits) and the fixture objects, away
    /// from the fake call results (`DATA..DATA+0x40000`) and the call log.
    const OC924_CTRL: u32 = DATA + 0x20_0000;
    const OC924_MAGIC: u32 = 0x0c92_4a11;
    const OC924_THIS: u32 = OC924_CTRL + 0x1000;
    const OC924_OBJ: u32 = OC924_CTRL + 0x2000;
    const OC924_CHILD: u32 = OC924_CTRL + 0x2400;
    const OC924_CARD: u32 = OC924_CTRL + 0x2800;
    const OC924_VT: u32 = OC924_CTRL + 0x3000;
    const OC924_OBJV: u32 = OC924_CTRL + 0x3800;
    /// `StrHash`' string addresses the widget-name chain of `OnWidgetCreated`
    /// compares against, in generated order.
    const OC924_NAMES: [u32; 10] = [
        0x0046_3f98,
        0x0046_1520,
        0x0046_3fb0,
        0x0046_3fc0,
        0x0046_3f88,
        0x0046_3fd8,
        0x0046_3ff0,
        0x0046_4000,
        0x0046_4010,
        0x0046_4020,
    ];

    /// The fake `StrHash`: a hash that depends on the string address only.
    fn oc924_hash(s: u32) -> u32 {
        s.wrapping_mul(0x9e37_79b1) | 1
    }

    /// One fake call (logs every register, results as `fakes::fake`), then the
    /// test's own results: v0 forced to zero for the calls the variant picks
    /// (virtual methods, `BuddyMan_IsConnected`, `UIScreen_FindObject`), the
    /// name hash, and the objects the callers dereference.
    fn oc924_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let a0 = c.g[A0][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let zeroable = addr < 0x1_0000 || addr == 0x0026_1460 || addr == 0x0039_d860;
        if zeroable && n < 16 && c.bus.read32(OC924_CTRL + 0x100 + 4 * n) != 0 {
            c.g[V0][0] = 0;
            return;
        }
        let v0 = c.g[V0][0] as u32;
        match addr {
            0x0031_7670 => c.g[V0][0] = sx(oc924_hash(a0)),
            // The popup the constructor returns gets a list-entry pointer (the
            // real UIStateStack push takes its short path).
            0x001b_6288 => c.bus.write32(v0.wrapping_add(0x20), OC924_CTRL + 0x4000),
            0x0039_d860 => c.bus.write32(v0.wrapping_add(8), OC924_OBJV),
            _ => {}
        }
    }

    macro_rules! oc924_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC924_CTRL) != OC924_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc924_fake_call(c, $addr);
                }
            )*
            fn oc924_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc924_fakes!(
        (
            oc924_fk_001b47c0,
            OC924_PREV_001B47C0,
            ee::s03::OVERRIDE_001B47C0,
            ee::s03::f_001b47c0__generated,
            0x001b_47c0
        ),
        (
            oc924_fk_001b4fa0,
            OC924_PREV_001B4FA0,
            ee::s03::OVERRIDE_001B4FA0,
            ee::s03::f_001b4fa0__generated,
            0x001b_4fa0
        ),
        (
            oc924_fk_001b5068,
            OC924_PREV_001B5068,
            ee::s03::OVERRIDE_001B5068,
            ee::s03::f_001b5068__generated,
            0x001b_5068
        ),
        (
            oc924_fk_001b4888,
            OC924_PREV_001B4888,
            ee::s03::OVERRIDE_001B4888,
            ee::s03::f_001b4888__generated,
            0x001b_4888
        ),
        (
            oc924_fk_001c57c0,
            OC924_PREV_001C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc924_fk_001a9c50,
            OC924_PREV_001A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc924_fk_001a9bc0,
            OC924_PREV_001A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc924_fk_001a97b8,
            OC924_PREV_001A97B8,
            ee::s03::OVERRIDE_001A97B8,
            ee::s03::f_001a97b8__generated,
            0x001a_97b8
        ),
        (
            oc924_fk_001b6288,
            OC924_PREV_001B6288,
            ee::s03::OVERRIDE_001B6288,
            ee::s03::f_001b6288__generated,
            0x001b_6288
        ),
        (
            oc924_fk_0025b800,
            OC924_PREV_0025B800,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800
        ),
        (
            oc924_fk_00260f80,
            OC924_PREV_00260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80
        ),
        (
            oc924_fk_002613c8,
            OC924_PREV_002613C8,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8
        ),
        (
            oc924_fk_00261408,
            OC924_PREV_00261408,
            ee::s06::OVERRIDE_00261408,
            ee::s06::f_00261408__generated,
            0x0026_1408
        ),
        (
            oc924_fk_00261460,
            OC924_PREV_00261460,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460
        ),
        (
            oc924_fk_002636a8,
            OC924_PREV_002636A8,
            ee::s07::OVERRIDE_002636A8,
            ee::s07::f_002636a8__generated,
            0x0026_36a8
        ),
        (
            oc924_fk_002636f0,
            OC924_PREV_002636F0,
            ee::s07::OVERRIDE_002636F0,
            ee::s07::f_002636f0__generated,
            0x0026_36f0
        ),
        (
            oc924_fk_00286200,
            OC924_PREV_00286200,
            ee::s07::OVERRIDE_00286200,
            ee::s07::f_00286200__generated,
            0x0028_6200
        ),
        (
            oc924_fk_0028b180,
            OC924_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc924_fk_002c26d0,
            OC924_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc924_fk_00317670,
            OC924_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc924_fk_00317d70,
            OC924_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc924_fk_0039d860,
            OC924_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc924_fk_00402450,
            OC924_PREV_00402450,
            ee::s15::OVERRIDE_00402450,
            ee::s15::f_00402450__generated,
            0x0040_2450
        ),
        (
            oc924_fk_00401718,
            OC924_PREV_00401718,
            ee::s15::OVERRIDE_00401718,
            ee::s15::f_00401718__generated,
            0x0040_1718
        ),
        (
            oc924_fk_00425fb8,
            OC924_PREV_00425FB8,
            ee::s15::OVERRIDE_00425FB8,
            ee::s15::f_00425fb8__generated,
            0x0042_5fb8
        ),
        (
            oc924_fk_0042c6a0,
            OC924_PREV_0042C6A0,
            ee::s15::OVERRIDE_0042C6A0,
            ee::s15::f_0042c6a0__generated,
            0x0042_c6a0
        ),
    );

    fn oc924_bit(var: u32, k: u32) -> u32 {
        (fakes::mix(var as u64 * 64 + k as u64) & 1) as u32
    }

    /// A vtable slot `{delta, method}` (the method is a fake address).
    fn oc924_slot(c: &mut Cpu, vt: u32, off: u32, n: u32) {
        c.bus.write16(vt + off, 0x10 * (n as u16 + 1));
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * n);
    }

    /// Machine for one seed: gated fakes, forced-zero results and objects from
    /// `var`, `pc` at the entry.
    fn oc924_setup(c: &mut Cpu, seed: u32, entry: u32, calls: u32, var: u32) -> Rng {
        oc924_install();
        let r = fakes::machine(c, seed, entry as u64, calls);
        c.hooks.miss = oc924_fake_call;
        c.hooks.jump_out = |c, pc, _, _| oc924_fake_call(c, pc);
        c.bus.write32(OC924_CTRL, OC924_MAGIC);
        for n in 0..16 {
            c.bus.write32(OC924_CTRL + 0x100 + 4 * n, oc924_bit(var, n));
        }
        // World -> +0x8c -> +4: the string table object with its method at +0x24;
        // +0x7c: the net module base the leaf `0x2591b8` adds to.
        let world = OC924_CTRL + 0x6000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x7c, OC924_CTRL + 0x6400);
        c.bus.write32(world + 0x8c, OC924_CTRL + 0x6200);
        c.bus.write32(OC924_CTRL + 0x6204, OC924_CTRL + 0x6300);
        c.bus.write16(OC924_CTRL + 0x6320, 8);
        c.bus.write32(OC924_CTRL + 0x6324, fakes::FAKE + 0x40);
        // The card manager (gp-0x1578): vtable methods at +0x1a8 and +0x1b8.
        c.bus.write32(GAME_GP.wrapping_sub(0x1578), OC924_CARD);
        c.bus.write32(OC924_CARD, OC924_VT);
        oc924_slot(c, OC924_VT, 0x1a8, 0);
        oc924_slot(c, OC924_VT, 0x1b8, 1);
        let counter = (fakes::mix(var as u64 + 99) % 4) as u32;
        c.bus.write32(GAME_GP.wrapping_sub(0x1570), counter);
        // Buddy manager (gp+0x238) present or not, lobby manager (gp-0xc8).
        let buddy = if oc924_bit(var, 20) != 0 {
            OC924_CTRL + 0x5000
        } else {
            0
        };
        c.bus.write32(GAME_GP.wrapping_add(0x238), buddy);
        c.bus
            .write32(GAME_GP.wrapping_sub(0xc8), OC924_CTRL + 0x5100);
        // `ssx_LoadExecPS2` (behind `func_00256088`): gp+0x2a74 -> +0x5c method.
        c.bus
            .write32(GAME_GP.wrapping_add(0x2a74), OC924_CTRL + 0x5c00);
        c.bus.write32(OC924_CTRL + 0x5c5c, OC924_CTRL + 0x5d00);
        oc924_slot(c, OC924_CTRL + 0x5d00, 0x10, 5);
        // Child objects (window vtable methods at +0x30/0x40/0x48/0x58/0xc0).
        for (i, off) in [0x30u32, 0x40, 0x48, 0x58, 0xc0].into_iter().enumerate() {
            oc924_slot(c, OC924_OBJV, off, 2 + i as u32);
        }
        c.bus.write32(OC924_OBJ + 8, OC924_OBJV);
        c.bus.write32(OC924_CHILD + 8, OC924_OBJV);
        // The state object.
        c.bus.write32(OC924_THIS + 0x10, OC924_CTRL + 0x5800);
        c.bus.write32(OC924_THIS + 0x40, OC924_CTRL + 0x5900);
        c.bus.write32(OC924_CTRL + 0x59d0, OC924_CTRL + 0x5a00);
        c.bus.write32(OC924_CTRL + 0x5a10, OC924_CTRL + 0x5b00);
        c.bus.write32(OC924_THIS + 0x6ec, OC924_OBJ);
        c.bus.write32(OC924_THIS + 0x6d8, oc924_bit(var, 21));
        let result = [1, 0, 0xffff_ff38, 5][(fakes::mix(var as u64 + 7) % 4) as usize];
        c.bus.write32(OC924_THIS + 0x70c, result);
        c.bus.write32(OC924_THIS + 0x708, 0x77);
        c.bus.write32(OC924_THIS + 0x6d0, 0x1234);
        c.pc = entry;
        r
    }

    fn oc924_run(
        generated: GuestFn,
        hand: GuestFn,
        entry: u32,
        cases: u32,
        calls: u32,
        args: &dyn Fn(&mut Cpu, &mut Rng, u32),
    ) {
        check(generated, hand, cases * 24, &|c, seed| {
            let (case, var) = (seed % cases, seed / cases);
            let mut r = oc924_setup(c, seed, entry, calls, var);
            args(c, &mut r, case);
        });
    }

    /// The popup-event arguments: `this`, a popup pointer, a 64-bit event.
    fn oc924_event_args(c: &mut Cpu, r: &mut Rng, event: u64) {
        c.g[A0][0] = OC924_THIS as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = event;
    }

    /// Memory card scan step: both vtable queries, the counter, the result
    /// word and the four helper calls.
    #[test]
    fn c_fe_state_net_config_scan_next_card_matches() {
        oc924_run(
            ee::s03::f_001b4470__generated,
            c_fe_state_net_config_scan_next_card,
            0x001b_4470,
            1,
            5,
            &|c, _, _| c.g[A0][0] = OC924_THIS as u64,
        );
    }

    /// Connect step: card missing (not-detected popup) or present (ISP popup
    /// and `Net_StartConnect`).
    #[test]
    fn c_fe_state_net_config_start_connect_matches() {
        oc924_run(
            ee::s03::f_001b45e8__generated,
            c_fe_state_net_config_start_connect,
            0x001b_45e8,
            1,
            9,
            &|c, _, _| c.g[A0][0] = OC924_THIS as u64,
        );
    }

    /// Widget-name chain: each of the ten names, and none.
    #[test]
    fn c_fe_state_net_config_on_widget_created_matches() {
        oc924_run(
            ee::s03::f_001b52c8__generated,
            c_fe_state_net_config_on_widget_created,
            0x001b_52c8,
            11,
            4,
            &|c, _, case| {
                let hash = match OC924_NAMES.get(case as usize) {
                    Some(&name) => oc924_hash(name),
                    None => 0x1234_5678,
                };
                c.bus.write32(OC924_CHILD + 0x38, hash);
                c.bus.write32(OC924_CHILD + 0x14, 0x12);
                c.g[A0][0] = OC924_THIS as u64;
                c.g[A1][0] = OC924_CHILD as u64;
            },
        );
    }

    /// Connecting popup: cancel events (0xF, 0x14), the auth popup (0x16, with
    /// and without a pending one) and forwarded events.
    #[test]
    fn c_fe_state_net_config_on_connecting_popup_event_matches() {
        const EVENTS: [u64; 10] = [
            0xf,
            0x14,
            0x16,
            0x16,
            0x10,
            0x0,
            0x17,
            0xffff_ffff_ffff_fff0,
            0x15,
            0x1_0000_000f,
        ];
        oc924_run(
            ee::s03::f_001b55b0__generated,
            c_fe_state_net_config_on_connecting_popup_event,
            0x001b_55b0,
            10,
            12,
            &|c, r, case| oc924_event_args(c, r, EVENTS[case as usize]),
        );
    }

    /// Disconnect popup: 0xF and 0x14 reset the session, others forward.
    #[test]
    fn c_fe_state_net_config_on_disconnect_popup_event_matches() {
        const EVENTS: [u64; 7] = [
            0xf,
            0x14,
            0x10,
            0x16,
            0x0,
            0x1_0000_000f,
            0xffff_ffff_ffff_fff0,
        ];
        oc924_run(
            ee::s03::f_001b5740__generated,
            c_fe_state_net_config_on_disconnect_popup_event,
            0x001b_5740,
            7,
            10,
            &|c, r, case| oc924_event_args(c, r, EVENTS[case as usize]),
        );
    }

    /// Create-config popup: 0xF greys the text (with and without the widget),
    /// the other events forward.
    #[test]
    fn c_fe_state_net_config_on_create_config_popup_event_matches() {
        const EVENTS: [u64; 8] = [0xf, 0x10, 0x14, 0x11, 0x0, 0x16, 0xffff_fff0, 0x1_0000_000f];
        oc924_run(
            ee::s03::f_001b5bd8__generated,
            c_fe_state_net_config_on_create_config_popup_event,
            0x001b_5bd8,
            8,
            9,
            &|c, r, case| oc924_event_args(c, r, EVENTS[case as usize]),
        );
    }
}
