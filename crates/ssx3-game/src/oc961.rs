//! Staging module oc961 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc961`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001e_1550,
        message_center_check_event_messages,
        "MessageCenter_CheckEventMessages",
    ),
    (
        0x001e_1eb8,
        message_center_process_pending_peak_flags,
        "MessageCenter_ProcessPendingPeakFlags",
    ),
    (
        0x001e_3ec0,
        ov_screen_message_center_on_widget_created,
        "OVScreenMessageCenter_OnWidgetCreated",
    ),
    (
        0x001e_48b0,
        ov_screen_message_center_refresh_list,
        "OVScreenMessageCenter_RefreshList",
    ),
    (
        0x001e_4c78,
        ov_screen_message_center_set_row,
        "OVScreenMessageCenter_SetRow",
    ),
];

/// `void MessageCenter_CheckEventMessages(void)` (0x1e1550): unless a replay
/// is running (`Game->Replay` state 1..9), look up the human rider's character
/// and, for each of the six rival-event slots (flags 1..3 for the first set of
/// three rivals, 4..6 for the second set), post a random one of three rival
/// taunt messages once the slot's rival event is unlocked and still unplayed
/// (a different group of messages for the rider's own character), setting the
/// slot's flag first so it fires only once. Only the first unset slot of each
/// triple is considered.
pub fn message_center_check_event_messages(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e1580: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e1580;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e1580;
            }
        }
        'b_1e1a1c: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1e1a1c;
            } else {
                'b_1e17dc: {
                    'b_1e1674: {
                        c.cycles += 17;
                        c.g[RA][0] = sx(0x001e1590);
                        call_ptr(c, 0x0014dc80, 0x001e1590)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e159c);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001e159c)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x001e15ac);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x00146e98, 0x001e15ac)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e15b8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00147398, 0x001e15b8)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e15c4);
                        c.g[A0][0] = 0x1;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e15c4)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_1e1674;
                        } else {
                            c.cycles += 7;
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = 0u64;
                            c.g[RA][0] = sx(0x001e15e0);
                            c.g[T0][0] = 0x1;
                            call_ptr(c, 0x001464d0, 0x001e15e0)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_1e17dc;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0xe;
                                c.g[RA][0] = sx(0x001e15f8);
                                c.g[A3][0] = 0x4;
                                call_ptr(c, 0x00145ef0, 0x001e15f8)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0x1;
                                if t {
                                    break 'b_1e17dc;
                                } else {
                                    'b_1e1644: {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x001e1608);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001e3a78,
                                            f: ee::s04::MessageCenter_SetFlag,
                                        }
                                        .call(c, 0x001e1608)?;
                                        c.g[S0][0] = 0x3;
                                        let t = c.g[S1][0] == c.g[S0][0];

                                        if t {
                                            break 'b_1e1644;
                                        } else {
                                            'b_1e1630: {
                                                c.cycles += 7;
                                                c.g[RA][0] = sx(0x001e161c);
                                                call_ptr(c, 0x003177f0, 0x001e161c)?;
                                                c.divu(0, 2, 16);
                                                c.g[A1][0] = 0u64;
                                                c.g[V0][0] = 0x3;
                                                if c.g[V0][0] == 0u64 {
                                                    break 'b_1e1630;
                                                } else {
                                                    break 'b_1e1630;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] = c.hi[0];
                                            c.g[RA][0] = sx(0x001e163c);
                                            Callee {
                                                addr: 0x001e2fe0,
                                                f: ee::s04::MessageCenter_PostMessage,
                                            }
                                            .call(c, 0x001e163c)?;
                                            let t = true;

                                            if t {
                                                break 'b_1e17dc;
                                            } else {
                                                break 'b_1e1644;
                                            }
                                        }
                                    }
                                    'b_1e1660: {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x001e164c);
                                        call_ptr(c, 0x003177f0, 0x001e164c)?;
                                        c.divu(0, 2, 17);
                                        c.g[A1][0] = 0u64;
                                        c.g[V0][0] = 0x3;
                                        if c.g[V0][0] == 0u64 {
                                            break 'b_1e1660;
                                        } else {
                                            break 'b_1e1660;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.hi[0];
                                    c.g[RA][0] = sx(0x001e166c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3));
                                    Callee {
                                        addr: 0x001e2fe0,
                                        f: ee::s04::MessageCenter_PostMessage,
                                    }
                                    .call(c, 0x001e166c)?;
                                    let t = true;

                                    if t {
                                        break 'b_1e17dc;
                                    } else {
                                        break 'b_1e1674;
                                    }
                                }
                            }
                        }
                    }
                    'b_1e172c: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e167c);
                        c.g[A0][0] = 0x2;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e167c)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_1e172c;
                        } else {
                            c.cycles += 7;
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = 0x1;
                            c.g[RA][0] = sx(0x001e1698);
                            c.g[T0][0] = 0x1;
                            call_ptr(c, 0x001464d0, 0x001e1698)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_1e17dc;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0xf;
                                c.g[RA][0] = sx(0x001e16b0);
                                c.g[A3][0] = 0x4;
                                call_ptr(c, 0x00145ef0, 0x001e16b0)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0x2;
                                if t {
                                    break 'b_1e17dc;
                                } else {
                                    'b_1e16fc: {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x001e16c0);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001e3a78,
                                            f: ee::s04::MessageCenter_SetFlag,
                                        }
                                        .call(c, 0x001e16c0)?;
                                        c.g[V0][0] = 0x7;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_1e16fc;
                                        } else {
                                            'b_1e16e8: {
                                                c.cycles += 7;
                                                c.g[RA][0] = sx(0x001e16d4);
                                                call_ptr(c, 0x003177f0, 0x001e16d4)?;
                                                c.g[V1][0] = 0x3;
                                                c.g[A1][0] = 0u64;
                                                c.divu(0, 2, 3);
                                                if c.g[V1][0] == 0u64 {
                                                    break 'b_1e16e8;
                                                } else {
                                                    break 'b_1e16e8;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] = c.hi[0];
                                            c.g[RA][0] = sx(0x001e16f4);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6));
                                            Callee {
                                                addr: 0x001e2fe0,
                                                f: ee::s04::MessageCenter_PostMessage,
                                            }
                                            .call(c, 0x001e16f4)?;
                                            let t = true;

                                            if t {
                                                break 'b_1e17dc;
                                            } else {
                                                break 'b_1e16fc;
                                            }
                                        }
                                    }
                                    'b_1e1718: {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x001e1704);
                                        call_ptr(c, 0x003177f0, 0x001e1704)?;
                                        c.g[V1][0] = 0x3;
                                        c.g[A1][0] = 0u64;
                                        c.divu(0, 2, 3);
                                        if c.g[V1][0] == 0u64 {
                                            break 'b_1e1718;
                                        } else {
                                            break 'b_1e1718;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.hi[0];
                                    c.g[RA][0] = sx(0x001e1724);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9));
                                    Callee {
                                        addr: 0x001e2fe0,
                                        f: ee::s04::MessageCenter_PostMessage,
                                    }
                                    .call(c, 0x001e1724)?;
                                    let t = true;

                                    if t {
                                        break 'b_1e17dc;
                                    } else {
                                        break 'b_1e172c;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e1734);
                    c.g[A0][0] = 0x3;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e1734)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_1e17dc;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = 0x2;
                        c.g[RA][0] = sx(0x001e1750);
                        c.g[T0][0] = 0x1;
                        call_ptr(c, 0x001464d0, 0x001e1750)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_1e17dc;
                        } else {
                            c.cycles += 6;
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0x10;
                            c.g[RA][0] = sx(0x001e1768);
                            c.g[A3][0] = 0x4;
                            call_ptr(c, 0x00145ef0, 0x001e1768)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = 0x3;
                            if t {
                                break 'b_1e17dc;
                            } else {
                                'b_1e17b4: {
                                    c.cycles += 5;
                                    c.g[RA][0] = sx(0x001e1778);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001e3a78,
                                        f: ee::s04::MessageCenter_SetFlag,
                                    }
                                    .call(c, 0x001e1778)?;
                                    c.g[V0][0] = 0x8;
                                    let t = c.g[S1][0] == c.g[V0][0];

                                    if t {
                                        break 'b_1e17b4;
                                    } else {
                                        'b_1e17a0: {
                                            c.cycles += 7;
                                            c.g[RA][0] = sx(0x001e178c);
                                            call_ptr(c, 0x003177f0, 0x001e178c)?;
                                            c.g[V1][0] = 0x3;
                                            c.g[A1][0] = 0u64;
                                            c.divu(0, 2, 3);
                                            if c.g[V1][0] == 0u64 {
                                                break 'b_1e17a0;
                                            } else {
                                                break 'b_1e17a0;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.hi[0];
                                        c.g[RA][0] = sx(0x001e17ac);
                                        c.g[A0][0] = c.g[A0][0] | 0xc;
                                        Callee {
                                            addr: 0x001e2fe0,
                                            f: ee::s04::MessageCenter_PostMessage,
                                        }
                                        .call(c, 0x001e17ac)?;
                                        let t = true;

                                        if t {
                                            break 'b_1e17dc;
                                        } else {
                                            break 'b_1e17b4;
                                        }
                                    }
                                }
                                'b_1e17d0: {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001e17bc);
                                    call_ptr(c, 0x003177f0, 0x001e17bc)?;
                                    c.g[V1][0] = 0x3;
                                    c.g[A1][0] = 0u64;
                                    c.divu(0, 2, 3);
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_1e17d0;
                                    } else {
                                        break 'b_1e17d0;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.hi[0];
                                c.g[RA][0] = sx(0x001e17dc);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf));
                                Callee {
                                    addr: 0x001e2fe0,
                                    f: ee::s04::MessageCenter_PostMessage,
                                }
                                .call(c, 0x001e17dc)?;
                                break 'b_1e17dc;
                            }
                        }
                    }
                }
                'b_1e189c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e17e4);
                    c.g[A0][0] = 0x4;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e17e4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_1e189c;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = 0u64;
                        c.g[RA][0] = sx(0x001e1800);
                        c.g[T0][0] = 0u64;
                        call_ptr(c, 0x001464d0, 0x001e1800)?;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1e1a1c;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0xe;
                            c.g[RA][0] = sx(0x001e181c);
                            c.g[A3][0] = 0x5;
                            call_ptr(c, 0x00145ef0, 0x001e181c)?;
                            let t = c.g[V0][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1e1a1c;
                            } else {
                                'b_1e186c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x4;
                                    c.g[RA][0] = sx(0x001e1830);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001e3a78,
                                        f: ee::s04::MessageCenter_SetFlag,
                                    }
                                    .call(c, 0x001e1830)?;
                                    c.g[S0][0] = 0x3;
                                    let t = c.g[S1][0] == c.g[S0][0];

                                    if t {
                                        break 'b_1e186c;
                                    } else {
                                        'b_1e1858: {
                                            c.cycles += 7;
                                            c.g[RA][0] = sx(0x001e1844);
                                            call_ptr(c, 0x003177f0, 0x001e1844)?;
                                            c.divu(0, 2, 16);
                                            c.g[A1][0] = 0u64;
                                            c.g[V0][0] = 0x3;
                                            if c.g[V0][0] == 0u64 {
                                                break 'b_1e1858;
                                            } else {
                                                break 'b_1e1858;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.hi[0];
                                        c.g[RA][0] = sx(0x001e1864);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12));
                                        Callee {
                                            addr: 0x001e2fe0,
                                            f: ee::s04::MessageCenter_PostMessage,
                                        }
                                        .call(c, 0x001e1864)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1e1a1c;
                                        } else {
                                            break 'b_1e186c;
                                        }
                                    }
                                }
                                'b_1e1888: {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001e1874);
                                    call_ptr(c, 0x003177f0, 0x001e1874)?;
                                    c.divu(0, 2, 17);
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = 0x3;
                                    if c.g[V0][0] == 0u64 {
                                        break 'b_1e1888;
                                    } else {
                                        break 'b_1e1888;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.hi[0];
                                c.g[RA][0] = sx(0x001e1894);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15));
                                Callee {
                                    addr: 0x001e2fe0,
                                    f: ee::s04::MessageCenter_PostMessage,
                                }
                                .call(c, 0x001e1894)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1e1a1c;
                                } else {
                                    break 'b_1e189c;
                                }
                            }
                        }
                    }
                }
                'b_1e195c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e18a4);
                    c.g[A0][0] = 0x5;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e18a4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_1e195c;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = 0x1;
                        c.g[RA][0] = sx(0x001e18c0);
                        c.g[T0][0] = 0u64;
                        call_ptr(c, 0x001464d0, 0x001e18c0)?;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1e1a1c;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0xf;
                            c.g[RA][0] = sx(0x001e18dc);
                            c.g[A3][0] = 0x5;
                            call_ptr(c, 0x00145ef0, 0x001e18dc)?;
                            let t = c.g[V0][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1e1a1c;
                            } else {
                                'b_1e192c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x5;
                                    c.g[RA][0] = sx(0x001e18f0);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001e3a78,
                                        f: ee::s04::MessageCenter_SetFlag,
                                    }
                                    .call(c, 0x001e18f0)?;
                                    c.g[V0][0] = 0x7;
                                    let t = c.g[S1][0] == c.g[V0][0];

                                    if t {
                                        break 'b_1e192c;
                                    } else {
                                        'b_1e1918: {
                                            c.cycles += 7;
                                            c.g[RA][0] = sx(0x001e1904);
                                            call_ptr(c, 0x003177f0, 0x001e1904)?;
                                            c.g[V1][0] = 0x3;
                                            c.g[A1][0] = 0u64;
                                            c.divu(0, 2, 3);
                                            if c.g[V1][0] == 0u64 {
                                                break 'b_1e1918;
                                            } else {
                                                break 'b_1e1918;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.hi[0];
                                        c.g[RA][0] = sx(0x001e1924);
                                        c.g[A0][0] = c.g[A0][0] | 0x18;
                                        Callee {
                                            addr: 0x001e2fe0,
                                            f: ee::s04::MessageCenter_PostMessage,
                                        }
                                        .call(c, 0x001e1924)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1e1a1c;
                                        } else {
                                            break 'b_1e192c;
                                        }
                                    }
                                }
                                'b_1e1948: {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001e1934);
                                    call_ptr(c, 0x003177f0, 0x001e1934)?;
                                    c.g[V1][0] = 0x3;
                                    c.g[A1][0] = 0u64;
                                    c.divu(0, 2, 3);
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_1e1948;
                                    } else {
                                        break 'b_1e1948;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.hi[0];
                                c.g[RA][0] = sx(0x001e1954);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b));
                                Callee {
                                    addr: 0x001e2fe0,
                                    f: ee::s04::MessageCenter_PostMessage,
                                }
                                .call(c, 0x001e1954)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1e1a1c;
                                } else {
                                    break 'b_1e195c;
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001e1964);
                c.g[A0][0] = 0x6;
                Callee {
                    addr: 0x001e3a30,
                    f: ee::s04::MessageCenter_IsFlagSet,
                }
                .call(c, 0x001e1964)?;
                let t = c.g[V0][0] != 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1e1a1c;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = 0x2;
                    c.g[RA][0] = sx(0x001e1984);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x001464d0, 0x001e1984)?;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1e1a1c;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x10;
                        c.g[RA][0] = sx(0x001e19a0);
                        c.g[A3][0] = 0x5;
                        call_ptr(c, 0x00145ef0, 0x001e19a0)?;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1e1a1c;
                        } else {
                            'b_1e19f0: {
                                c.cycles += 6;
                                c.g[A0][0] = 0x6;
                                c.g[RA][0] = sx(0x001e19b4);
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001e3a78,
                                    f: ee::s04::MessageCenter_SetFlag,
                                }
                                .call(c, 0x001e19b4)?;
                                c.g[V0][0] = 0x8;
                                let t = c.g[S1][0] == c.g[V0][0];

                                if t {
                                    break 'b_1e19f0;
                                } else {
                                    'b_1e19dc: {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x001e19c8);
                                        call_ptr(c, 0x003177f0, 0x001e19c8)?;
                                        c.g[V1][0] = 0x3;
                                        c.g[A1][0] = 0u64;
                                        c.divu(0, 2, 3);
                                        if c.g[V1][0] == 0u64 {
                                            break 'b_1e19dc;
                                        } else {
                                            break 'b_1e19dc;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.hi[0];
                                    c.g[RA][0] = sx(0x001e19e8);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e));
                                    Callee {
                                        addr: 0x001e2fe0,
                                        f: ee::s04::MessageCenter_PostMessage,
                                    }
                                    .call(c, 0x001e19e8)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1e1a1c;
                                    } else {
                                        break 'b_1e19f0;
                                    }
                                }
                            }
                            'b_1e1a0c: {
                                c.cycles += 7;
                                c.g[RA][0] = sx(0x001e19f8);
                                call_ptr(c, 0x003177f0, 0x001e19f8)?;
                                c.g[V1][0] = 0x3;
                                c.g[A1][0] = 0u64;
                                c.divu(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_1e1a0c;
                                } else {
                                    break 'b_1e1a0c;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.hi[0];
                            c.g[RA][0] = sx(0x001e1a18);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21));
                            Callee {
                                addr: 0x001e2fe0,
                                f: ee::s04::MessageCenter_PostMessage,
                            }
                            .call(c, 0x001e1a18)?;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_1e1a1c;
                        }
                    }
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

/// `void MessageCenter_ProcessPendingPeakFlags(void)` (0x1e1eb8): unless a
/// replay is running, turn the pending peak flags 40..51 into peak messages:
/// flags 40/41, 44/45, 48/49 post `PostPeakMessageA(peak, 0/1)` and flags
/// 42/43, 46/47, 50/51 post `PostPeakMessageB(peak, 0/1)` (peaks 0..2), each
/// flag cleared once handled.
pub fn message_center_process_pending_peak_flags(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e1edc: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e1edc;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e1edc;
            }
        }
        'b_1e20c8: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            if t {
                break 'b_1e20c8;
            } else {
                'b_1e1f34: {
                    'b_1e1f10: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1eec);
                        c.g[A0][0] = 0x28;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e1eec)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_1e1f10;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001e1efc);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e2220,
                                f: ee::s04::MessageCenter_PostPeakMessageA,
                            }
                            .call(c, 0x001e1efc)?;
                            c.g[A0][0] = 0x28;
                            c.g[RA][0] = sx(0x001e1f08);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e3a78,
                                f: ee::s04::MessageCenter_SetFlag,
                            }
                            .call(c, 0x001e1f08)?;
                            let t = true;

                            if t {
                                break 'b_1e1f34;
                            } else {
                                break 'b_1e1f10;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e1f18);
                    c.g[A0][0] = 0x29;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e1f18)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_1e1f34;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e1f28);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001e2220,
                            f: ee::s04::MessageCenter_PostPeakMessageA,
                        }
                        .call(c, 0x001e1f28)?;
                        c.g[A0][0] = 0x29;
                        c.g[RA][0] = sx(0x001e1f34);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e1f34)?;
                        break 'b_1e1f34;
                    }
                }
                'b_1e1f84: {
                    'b_1e1f60: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1f3c);
                        c.g[A0][0] = 0x2c;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e1f3c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x1;
                        if t {
                            break 'b_1e1f60;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001e1f4c);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e2220,
                                f: ee::s04::MessageCenter_PostPeakMessageA,
                            }
                            .call(c, 0x001e1f4c)?;
                            c.g[A0][0] = 0x2c;
                            c.g[RA][0] = sx(0x001e1f58);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e3a78,
                                f: ee::s04::MessageCenter_SetFlag,
                            }
                            .call(c, 0x001e1f58)?;
                            let t = true;

                            if t {
                                break 'b_1e1f84;
                            } else {
                                break 'b_1e1f60;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e1f68);
                    c.g[A0][0] = 0x2d;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e1f68)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x1;
                    if t {
                        break 'b_1e1f84;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e1f78);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001e2220,
                            f: ee::s04::MessageCenter_PostPeakMessageA,
                        }
                        .call(c, 0x001e1f78)?;
                        c.g[A0][0] = 0x2d;
                        c.g[RA][0] = sx(0x001e1f84);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e1f84)?;
                        break 'b_1e1f84;
                    }
                }
                'b_1e1fd4: {
                    'b_1e1fb0: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1f8c);
                        c.g[A0][0] = 0x30;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e1f8c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x2;
                        if t {
                            break 'b_1e1fb0;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001e1f9c);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e2220,
                                f: ee::s04::MessageCenter_PostPeakMessageA,
                            }
                            .call(c, 0x001e1f9c)?;
                            c.g[A0][0] = 0x30;
                            c.g[RA][0] = sx(0x001e1fa8);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e3a78,
                                f: ee::s04::MessageCenter_SetFlag,
                            }
                            .call(c, 0x001e1fa8)?;
                            let t = true;

                            if t {
                                break 'b_1e1fd4;
                            } else {
                                break 'b_1e1fb0;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e1fb8);
                    c.g[A0][0] = 0x31;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e1fb8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x2;
                    if t {
                        break 'b_1e1fd4;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e1fc8);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001e2220,
                            f: ee::s04::MessageCenter_PostPeakMessageA,
                        }
                        .call(c, 0x001e1fc8)?;
                        c.g[A0][0] = 0x31;
                        c.g[RA][0] = sx(0x001e1fd4);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e1fd4)?;
                        break 'b_1e1fd4;
                    }
                }
                'b_1e2024: {
                    'b_1e2000: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1fdc);
                        c.g[A0][0] = 0x2a;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e1fdc)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_1e2000;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001e1fec);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e20d0,
                                f: ee::s04::MessageCenter_PostPeakMessageB,
                            }
                            .call(c, 0x001e1fec)?;
                            c.g[A0][0] = 0x2a;
                            c.g[RA][0] = sx(0x001e1ff8);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e3a78,
                                f: ee::s04::MessageCenter_SetFlag,
                            }
                            .call(c, 0x001e1ff8)?;
                            let t = true;

                            if t {
                                break 'b_1e2024;
                            } else {
                                break 'b_1e2000;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e2008);
                    c.g[A0][0] = 0x2b;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e2008)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_1e2024;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e2018);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001e20d0,
                            f: ee::s04::MessageCenter_PostPeakMessageB,
                        }
                        .call(c, 0x001e2018)?;
                        c.g[A0][0] = 0x2b;
                        c.g[RA][0] = sx(0x001e2024);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e2024)?;
                        break 'b_1e2024;
                    }
                }
                'b_1e2074: {
                    'b_1e2050: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e202c);
                        c.g[A0][0] = 0x2e;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e202c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x1;
                        if t {
                            break 'b_1e2050;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001e203c);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e20d0,
                                f: ee::s04::MessageCenter_PostPeakMessageB,
                            }
                            .call(c, 0x001e203c)?;
                            c.g[A0][0] = 0x2e;
                            c.g[RA][0] = sx(0x001e2048);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e3a78,
                                f: ee::s04::MessageCenter_SetFlag,
                            }
                            .call(c, 0x001e2048)?;
                            let t = true;

                            if t {
                                break 'b_1e2074;
                            } else {
                                break 'b_1e2050;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e2058);
                    c.g[A0][0] = 0x2f;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e2058)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x1;
                    if t {
                        break 'b_1e2074;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e2068);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001e20d0,
                            f: ee::s04::MessageCenter_PostPeakMessageB,
                        }
                        .call(c, 0x001e2068)?;
                        c.g[A0][0] = 0x2f;
                        c.g[RA][0] = sx(0x001e2074);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e2074)?;
                        break 'b_1e2074;
                    }
                }
                'b_1e20a0: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e207c);
                    c.g[A0][0] = 0x32;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e207c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x2;
                    if t {
                        break 'b_1e20a0;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001e208c);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e20d0,
                            f: ee::s04::MessageCenter_PostPeakMessageB,
                        }
                        .call(c, 0x001e208c)?;
                        c.g[A0][0] = 0x32;
                        c.g[RA][0] = sx(0x001e2098);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e2098)?;
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_1e20c8;
                        } else {
                            break 'b_1e20a0;
                        }
                    }
                }
                'b_1e20c4: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e20a8);
                    c.g[A0][0] = 0x33;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e20a8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x2;
                    if t {
                        break 'b_1e20c4;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e20b8);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001e20d0,
                            f: ee::s04::MessageCenter_PostPeakMessageB,
                        }
                        .call(c, 0x001e20b8)?;
                        c.g[A0][0] = 0x33;
                        c.g[RA][0] = sx(0x001e20c4);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e3a78,
                            f: ee::s04::MessageCenter_SetFlag,
                        }
                        .call(c, 0x001e20c4)?;
                        break 'b_1e20c4;
                    }
                }
                c.cycles += 1;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                break 'b_1e20c8;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenMessageCenter_OnWidgetCreated(self, widget)` (0x1e3ec0):
/// matches the new widget's name hash (`widget+0x38`) against the message
/// center's widget names and stores it in the matching `self` field (+0x13c..
/// +0x160); the per-row widgets "name%d" (i = 0..7) go into the arrays at
/// +0x9c, +0xbc, +0xdc, +0xfc and +0x11c. Some kinds also get a vtable call
/// (slot 9 or 27) as they are registered.
pub fn ov_screen_message_center_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = 0x001e3ec0;
        loop {
            match pc {
                0x001e3ec0 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1048));
                    c.g[RA][0] = sx(0x001e3f00);
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001e3f00)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001e3f10;
                        continue;
                    }
                    pc = 0x001e3f08;
                    continue;
                }
                0x001e3f08 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14c), c.g[S2][0]);
                    if t {
                        pc = 0x001e3f20;
                        continue;
                    }
                    pc = 0x001e3f10;
                    continue;
                }
                0x001e3f10 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3f18);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20e0));
                    call_ptr(c, 0x00317670, 0x001e3f18)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x150), c.g[S2][0]);
                        pc = 0x001e3f20;
                        continue;
                    }
                    pc = 0x001e3f20;
                    continue;
                }
                0x001e3f20 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3f28);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1058));
                    call_ptr(c, 0x00317670, 0x001e3f28)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001e3f38;
                        continue;
                    }
                    pc = 0x001e3f30;
                    continue;
                }
                0x001e3f30 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x154), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3f38;
                    continue;
                }
                0x001e3f38 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3f40);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20e8));
                    call_ptr(c, 0x00317670, 0x001e3f40)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        pc = 0x001e3f50;
                        continue;
                    }
                    pc = 0x001e3f48;
                    continue;
                }
                0x001e3f48 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x158), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3f50;
                    continue;
                }
                0x001e3f50 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3f58);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2060));
                    call_ptr(c, 0x00317670, 0x001e3f58)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        pc = 0x001e3f68;
                        continue;
                    }
                    pc = 0x001e3f60;
                    continue;
                }
                0x001e3f60 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x13c), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3f68;
                    continue;
                }
                0x001e3f68 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3f70);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2048));
                    call_ptr(c, 0x00317670, 0x001e3f70)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        pc = 0x001e3f80;
                        continue;
                    }
                    pc = 0x001e3f78;
                    continue;
                }
                0x001e3f78 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x140), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3f80;
                    continue;
                }
                0x001e3f80 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3f88);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2030));
                    call_ptr(c, 0x00317670, 0x001e3f88)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        pc = 0x001e3f9c;
                        continue;
                    }
                    pc = 0x001e3f90;
                    continue;
                }
                0x001e3f90 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001e4000;
                        continue;
                    }
                    pc = 0x001e3f9c;
                    continue;
                }
                0x001e3f9c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3fa4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2020));
                    call_ptr(c, 0x00317670, 0x001e3fa4)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        pc = 0x001e3fb4;
                        continue;
                    }
                    pc = 0x001e3fac;
                    continue;
                }
                0x001e3fac => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x144), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3fb4;
                    continue;
                }
                0x001e3fb4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3fbc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2010));
                    call_ptr(c, 0x00317670, 0x001e3fbc)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001e3fcc;
                        continue;
                    }
                    pc = 0x001e3fc4;
                    continue;
                }
                0x001e3fc4 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x160), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3fcc;
                    continue;
                }
                0x001e3fcc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3fd4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20f0));
                    call_ptr(c, 0x00317670, 0x001e3fd4)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        pc = 0x001e3fe4;
                        continue;
                    }
                    pc = 0x001e3fdc;
                    continue;
                }
                0x001e3fdc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x15c), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e3fe4;
                    continue;
                }
                0x001e3fe4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e3fec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2000));
                    call_ptr(c, 0x00317670, 0x001e3fec)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x001e4074;
                        continue;
                    }
                    pc = 0x001e3ff4;
                    continue;
                }
                0x001e3ff4 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x148), c.g[S2][0]);
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x001e4000;
                    continue;
                }
                0x001e4000 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e4010);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e4010)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001e4138;
                        continue;
                    }
                    pc = 0x001e4018;
                    continue;
                }
                0x001e4018 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e4028;
                    continue;
                }
                0x001e4028 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e4038;
                    continue;
                }
                0x001e4038 => {
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), c.g[S2][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001e405c);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e405c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001e4138;
                        continue;
                    }
                    pc = 0x001e4064;
                    continue;
                }
                0x001e4064 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c), c.g[S2][0]);
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e4074;
                    continue;
                }
                0x001e4074 => {
                    c.cycles += 6;
                    c.g[FP][0] = 0x470000;
                    c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x9c));
                    c.g[S7][0] = 0x4a0000;
                    c.g[S6][0] = 0x470000;
                    let t = true;
                    c.g[S5][0] = 0x470000;
                    if t {
                        pc = 0x001e4104;
                        continue;
                    }
                    pc = 0x001e408c;
                    continue;
                }
                0x001e408c => {
                    c.cycles += 1;
                    pc = 0x001e4090;
                    continue;
                }
                0x001e4090 => {
                    c.cycles += 7;
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1fe0));
                    c.g[RA][0] = sx(0x001e409c);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x001e409c)?;
                    c.g[RA][0] = sx(0x001e40a4);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001e40a4)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if back_edge_unwinds(c, 0x001e4018) {
                            return Ok(());
                        }
                        pc = 0x001e4018;
                        continue;
                    }
                    pc = 0x001e40ac;
                    continue;
                }
                0x001e40ac => {
                    c.cycles += 7;
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x20f8));
                    c.g[RA][0] = sx(0x001e40b8);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x001e40b8)?;
                    c.g[RA][0] = sx(0x001e40c0);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001e40c0)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if back_edge_unwinds(c, 0x001e4028) {
                            return Ok(());
                        }
                        pc = 0x001e4028;
                        continue;
                    }
                    pc = 0x001e40c8;
                    continue;
                }
                0x001e40c8 => {
                    c.cycles += 7;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1fd0));
                    c.g[RA][0] = sx(0x001e40d4);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x001e40d4)?;
                    c.g[RA][0] = sx(0x001e40dc);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001e40dc)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1fc0));
                    if t {
                        if back_edge_unwinds(c, 0x001e4038) {
                            return Ok(());
                        }
                        pc = 0x001e4038;
                        continue;
                    }
                    pc = 0x001e40e4;
                    continue;
                }
                0x001e40e4 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001e40f0);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x001e40f0)?;
                    c.g[RA][0] = sx(0x001e40f8);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001e40f8)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001e4064) {
                            return Ok(());
                        }
                        pc = 0x001e4064;
                        continue;
                    }
                    pc = 0x001e4100;
                    continue;
                }
                0x001e4100 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x001e4104;
                    continue;
                }
                0x001e4104 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        pc = 0x001e4134;
                        continue;
                    }
                    pc = 0x001e4110;
                    continue;
                }
                0x001e4110 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1ff0));
                    c.g[RA][0] = sx(0x001e4120);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x001e4120)?;
                    c.g[RA][0] = sx(0x001e4128);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001e4128)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = c.g[SP][0];
                        if back_edge_unwinds(c, 0x001e4090) {
                            return Ok(());
                        }
                        pc = 0x001e4090;
                        continue;
                    }
                    pc = 0x001e4130;
                    continue;
                }
                0x001e4130 => {
                    c.cycles += 1;
                    c.sw((c.g[S4][0] as u32), c.g[S2][0]);
                    pc = 0x001e4134;
                    continue;
                }
                0x001e4134 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    pc = 0x001e4138;
                    continue;
                }
                0x001e4138 => {
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
                }
                _ => {
                    c.jump_out(pc, 0x001e3ec0, 0x001e4164);
                    return Ok(());
                }
            }
        }
    });
}

/// `void OVScreenMessageCenter_RefreshList(self)` (0x1e48b0): rebuilds the
/// message list: counts the local and online (FAQ) messages; with none it hides
/// the list widgets and shows "no messages", otherwise it shows them and
/// fills the eight visible rows (`SetRow` per message, hiding the unused
/// rows), then sets the scrollable menu's item count and scroll position.
pub fn ov_screen_message_center_refresh_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e48f8: {
            'b_1e48f0: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1e48f0;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = 0x4;
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_1e48f8;
                    } else {
                        break 'b_1e48f0;
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = 0x4;
            c.g[A1][0] = 0u64;
            break 'b_1e48f8;
        }
        'b_1e4b30: {
            'b_1e49b0: {
                c.cycles += 8;
                c.g[RA][0] = sx(0x001e4900);
                Callee {
                    addr: 0x001e2bb8,
                    f: ee::s04::MessageCenter_CountGroupMessages,
                }
                .call(c, 0x001e4900)?;
                c.g[RA][0] = sx(0x001e4908);
                c.g[S5][0] = c.g[V0][0];
                Callee {
                    addr: 0x001e33e8,
                    f: ee::s04::MessageCenter_GetMessageCount,
                }
                .call(c, 0x001e4908)?;
                c.g[S6][0] = c.g[V0][0];
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                let t = c.g[S0][0] != 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1e49b0;
                } else {
                    c.cycles += 38;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f30));
                    c.g[RA][0] = sx(0x001e4928);
                    c.g[S7][0] = 0u64;
                    call_ptr(c, 0x0039e8b8, 0x001e4928)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4938);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f20));
                    call_ptr(c, 0x0039e8b8, 0x001e4938)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4948);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f10));
                    call_ptr(c, 0x0039e8b8, 0x001e4948)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4958);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f00));
                    call_ptr(c, 0x0039e8b8, 0x001e4958)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4968);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ee8));
                    call_ptr(c, 0x0039e8b8, 0x001e4968)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4978);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ed0));
                    call_ptr(c, 0x0039e948, 0x001e4978)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x154));
                    c.g[RA][0] = sx(0x001e4984);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4984)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x158));
                    c.g[RA][0] = sx(0x001e4990);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4990)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x001e499c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e499c)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x150));
                    c.g[RA][0] = sx(0x001e49a8);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e49a8)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x144));
                    if t {
                        break 'b_1e4b30;
                    } else {
                        break 'b_1e49b0;
                    }
                }
            }
            c.cycles += 28;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f30));
            c.g[RA][0] = sx(0x001e49c0);
            c.g[S7][0] = c.g[S0][0];
            call_ptr(c, 0x0039e948, 0x001e49c0)?;
            c.g[S3][0] = 0u64;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001e49d4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f20));
            call_ptr(c, 0x0039e948, 0x001e49d4)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001e49e4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f10));
            call_ptr(c, 0x0039e948, 0x001e49e4)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001e49f4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1f00));
            call_ptr(c, 0x0039e948, 0x001e49f4)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001e4a04);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ed0));
            call_ptr(c, 0x0039e8b8, 0x001e4a04)?;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x154));
            c.g[RA][0] = sx(0x001e4a10);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x001e4878,
                f: ee::s04::OVScreenMessageCenter_ShowWidget,
            }
            .call(c, 0x001e4a10)?;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x158));
            c.g[RA][0] = sx(0x001e4a1c);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x001e4878,
                f: ee::s04::OVScreenMessageCenter_ShowWidget,
            }
            .call(c, 0x001e4a1c)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x174));
            'l_1e4a20: loop {
                'b_1e4b20: {
                    'b_1e4a68: {
                        c.cycles += 5;
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                        let t = (c.g[S1][0] as i64) < 0;
                        c.g[S4][0] = c.g[V1][0];
                        if t {
                            break 'b_1e4a68;
                        } else {
                            c.cycles += 13;
                            c.g[RA][0] = sx(0x001e4a3c);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001e3620,
                                f: ee::s04::MessageCenter_GetMessageId,
                            }
                            .call(c, 0x001e4a3c)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001e4a48);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001e3650,
                                f: ee::s04::MessageCenter_GetMessageVariant,
                            }
                            .call(c, 0x001e4a48)?;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[T0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001e4a60);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001e4c78,
                                f: ee::s04::OVScreenMessageCenter_SetRow,
                            }
                            .call(c, 0x001e4a60)?;
                            let t = true;
                            c.g[S3][0] = c.g[S4][0];
                            if t {
                                break 'b_1e4b20;
                            } else {
                                break 'b_1e4a68;
                            }
                        }
                    }
                    'b_1e4ac8: {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                        if t {
                            break 'b_1e4ac8;
                        } else {
                            'b_1e4a90: {
                                'b_1e4a88: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = !(0u64 | c.g[S1][0]);
                                    if t {
                                        break 'b_1e4a88;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x4;
                                        let t = true;
                                        c.g[A2][0] = 0u64;
                                        if t {
                                            break 'b_1e4a90;
                                        } else {
                                            break 'b_1e4a88;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x4;
                                c.g[A2][0] = 0x1;
                                break 'b_1e4a90;
                            }
                            c.cycles += 14;
                            c.g[RA][0] = sx(0x001e4a98);
                            Callee {
                                addr: 0x001e2c80,
                                f: ee::s04::MessageCenter_GetGroupMessageId,
                            }
                            .call(c, 0x001e4a98)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001e4aa8);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001e4b80,
                                f: ee::s04::OVScreenMessageCenter_GetGroupVariant,
                            }
                            .call(c, 0x001e4aa8)?;
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[T0][0] = c.g[V0][0] & 0xff;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001e4ac0);
                            c.g[A1][0] = 0xffffffffffffffff;
                            Callee {
                                addr: 0x001e4c78,
                                f: ee::s04::OVScreenMessageCenter_SetRow,
                            }
                            .call(c, 0x001e4ac0)?;
                            let t = true;
                            c.g[S3][0] = c.g[S4][0];
                            if t {
                                break 'b_1e4b20;
                            } else {
                                break 'b_1e4ac8;
                            }
                        }
                    }
                    c.cycles += 22;
                    c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[RA][0] = sx(0x001e4adc);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4adc)?;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4aec);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xbc));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4aec)?;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4afc);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdc));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4afc)?;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4b0c);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4b0c)?;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e4b1c);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xfc));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4b1c)?;
                    c.g[S3][0] = c.g[S4][0];
                    break 'b_1e4b20;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x174));
                    if event_due(c) {
                        deliver_event_here(c, 0x001e4a20);
                    }
                    continue 'l_1e4a20;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x144));
                    break 'b_1e4b30;
                }
            }
        }
        c.cycles += 20;
        c.g[A1][0] = c.g[S7][0] & 0xff;
        c.g[S0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
        c.g[RA][0] = sx(0x001e4b40);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039b760, 0x001e4b40)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x144));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sb((c.g[V0][0] as u32).wrapping_add(0x95), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x144));
        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x174));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sb((c.g[V1][0] as u32).wrapping_add(0x98), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenMessageCenter_SetRow(self, id, slot, sub)` (0x1e4c78): fills
/// one visible row of the message list from the message `id`: shows or hides
/// the row's widgets (read/unread bitmaps, group header tint), sets the row's
/// marker text ("+", "-", ">" or the index), the sender name, and the subject
/// (via `SetRowSubject` for type-1 messages).
pub fn ov_screen_message_center_set_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e4dd4: {
            'b_1e4d6c: {
                c.cycles += 32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                c.g[V1][0] = 0x18;
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = 0x4c0000;
                c.g[S7][0] = c.g[A3][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.mult(0, 3, 23, 3);
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T1][0] = 0x14;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A3][0] = 0x440000;
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1630));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[S2][0] = c.g[A1][0];
                c.g[S6][0] = c.g[A2][0];
                c.g[FP][0] = c.g[T0][0] & 0xff;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                c.mult(0, 2, 2, 9);
                c.g[RA][0] = sx(0x001e4ce8);
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                Callee {
                    addr: 0x001e39f8,
                    f: ee::s04::MessageCenter_GetInbox,
                }
                .call(c, 0x001e4ce8)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffffffffffffffff;
                if c.g[S2][0] != c.g[V0][0] {
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc8));
                    break 'b_1e4d6c;
                } else {
                    'b_1e4d44: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                        c.g[S3][0] = sx((c.g[S6][0] as u32) << 2);
                        let t = c.g[V0][0] == 0u64;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xbc));
                        if t {
                            break 'b_1e4d44;
                        } else {
                            'b_1e4d38: {
                                'b_1e4d34: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                    let t = c.g[S7][0] != c.g[V0][0];
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                                    if t {
                                        break 'b_1e4d34;
                                    } else {
                                        'b_1e4d2c: {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001e4d1c);
                                            c.g[A0][0] =
                                                c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                                            Callee {
                                                addr: 0x001e3a30,
                                                f: ee::s04::MessageCenter_IsFlagSet,
                                            }
                                            .call(c, 0x001e4d1c)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[S3][0] as u32)));
                                            if t {
                                                break 'b_1e4d2c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x164));
                                                if t {
                                                    break 'b_1e4d38;
                                                } else {
                                                    break 'b_1e4d2c;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x168));
                                        if t {
                                            break 'b_1e4d38;
                                        } else {
                                            break 'b_1e4d34;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
                                break 'b_1e4d38;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[A0][0]);
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[S2][0]);
                            break 'b_1e4d44;
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001e4d54);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4d54)?;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001e4d64);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x001e4878,
                        f: ee::s04::OVScreenMessageCenter_ShowWidget,
                    }
                    .call(c, 0x001e4d64)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x11c));
                    if t {
                        break 'b_1e4dd4;
                    } else {
                        break 'b_1e4d6c;
                    }
                }
            }
            'b_1e4dac: {
                c.cycles += 5;
                c.g[V0][0] = 0x1;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S2][0] as u32) & 31));
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                let t = c.g[V1][0] == 0u64;
                c.g[S0][0] = sx((c.g[S6][0] as u32) << 2);
                if t {
                    break 'b_1e4dac;
                } else {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[S3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001e4d94);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4d94)?;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001e4da4);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    Callee {
                        addr: 0x001e4840,
                        f: ee::s04::OVScreenMessageCenter_HideWidget,
                    }
                    .call(c, 0x001e4da4)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x11c));
                    if t {
                        break 'b_1e4dd4;
                    } else {
                        break 'b_1e4dac;
                    }
                }
            }
            c.cycles += 10;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[S3][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001e4dc0);
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
            Callee {
                addr: 0x001e4878,
                f: ee::s04::OVScreenMessageCenter_ShowWidget,
            }
            .call(c, 0x001e4dc0)?;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e4dd0);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
            Callee {
                addr: 0x001e4840,
                f: ee::s04::OVScreenMessageCenter_HideWidget,
            }
            .call(c, 0x001e4dd0)?;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x11c));
            break 'b_1e4dd4;
        }
        'b_1e4e98: {
            c.cycles += 5;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
            if t {
                break 'b_1e4e98;
            } else {
                'b_1e4e6c: {
                    'b_1e4e50: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                        if c.g[V0][0] == 0u64 {
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                            break 'b_1e4e50;
                        } else {
                            'b_1e4e3c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                let t = c.g[S7][0] != c.g[V0][0];
                                c.g[A1][0] = 0x4a0000;
                                if t {
                                    break 'b_1e4e3c;
                                } else {
                                    'b_1e4e28: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001e4e08);
                                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                                        Callee {
                                            addr: 0x001e3a30,
                                            f: ee::s04::MessageCenter_IsFlagSet,
                                        }
                                        .call(c, 0x001e4e08)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = 0x4a0000;
                                        if t {
                                            break 'b_1e4e28;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[SP][0];
                                            c.g[RA][0] = sx(0x001e4e20);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2100));
                                            call_ptr(c, 0x00417828, 0x001e4e20)?;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[S3][0] as u32)));
                                            if t {
                                                break 'b_1e4e6c;
                                            } else {
                                                break 'b_1e4e28;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[RA][0] = sx(0x001e4e34);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2108));
                                    call_ptr(c, 0x00417828, 0x001e4e34)?;
                                    let t = true;
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                                    if t {
                                        break 'b_1e4e6c;
                                    } else {
                                        break 'b_1e4e3c;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x001e4e48);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2110));
                            call_ptr(c, 0x00417828, 0x001e4e48)?;
                            let t = true;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                            if t {
                                break 'b_1e4e6c;
                            } else {
                                break 'b_1e4e50;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2118));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                    c.g[RA][0] = sx(0x001e4e68);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x001e4e68)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    break 'b_1e4e6c;
                }
                c.cycles += 11;
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001e4e78);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                call_ptr(c, 0x003a0c50, 0x001e4e78)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001e4e94);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001e4e94)?;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
                break 'b_1e4e98;
            }
        }
        'b_1e4ee4: {
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
            if c.g[V1][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                break 'b_1e4ee4;
            } else {
                c.cycles += 15;
                c.g[RA][0] = sx(0x001e4eb0);
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                Callee {
                    addr: 0x001e2ea0,
                    f: ee::s04::MessageCenter_GetSenderNameHash,
                }
                .call(c, 0x001e4eb0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x001e4ebc);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e4ebc)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001e4ed8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001e4ed8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S7][0]);
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                break 'b_1e4ee4;
            }
        }
        'b_1e4f44: {
            'b_1e4f0c: {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                if t {
                    break 'b_1e4f0c;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x001e4f04);
                    c.g[A3][0] = c.g[FP][0];
                    Callee {
                        addr: 0x001e4f70,
                        f: ee::s04::OVScreenMessageCenter_SetRowSubject,
                    }
                    .call(c, 0x001e4f04)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        break 'b_1e4f44;
                    } else {
                        break 'b_1e4f0c;
                    }
                }
            }
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
            if c.g[A0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1e4f44;
            } else {
                c.cycles += 10;
                c.g[RA][0] = sx(0x001e4f24);
                c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a1000, 0x001e4f24)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001e4f40);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001e4f40)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1e4f44;
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fresh, run_generated, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's own RAM) and
    /// its value; the mailbox words (`OC961_CTRL + 0x10 + 4 * k`) hold what the
    /// fakes answer.
    pub(crate) const OC961_CTRL: u32 = DATA + 0x2d_0000;
    pub(crate) const OC961_MAGIC: u32 = 0x4f43_3936 + 1;

    pub(crate) fn oc961_mail(c: &mut Cpu, k: u32, v: u32) {
        c.bus.write32(OC961_CTRL + 0x10 + 4 * k, v);
    }

    /// Gated fakes for the callees that have an override slot. `mode` 0:
    /// only logs the call; 1: returns bit `call index & 31` of mailbox word
    /// `k`; 2: returns mailbox word `k`; 3: a hash compare: returns word `k+1`
    /// on call index `word k`, else a different value.
    macro_rules! oc961_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mode:expr, $k:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC961_CTRL) != OC961_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let n = c.bus.read32(fakes::LOG_COUNT);
                    fakes::fake_call(c, $addr);
                    if c.pc >> 16 == 0xdead {
                        return;
                    }
                    let word = c.bus.read32(OC961_CTRL + 0x10 + 4 * $k);
                    if $mode == 1 {
                        c.g[V0][0] = ((word >> (n & 31)) & 1) as u64;
                    } else if $mode == 2 {
                        c.g[V0][0] = sx(word);
                    } else if $mode == 3 {
                        let h = c.bus.read32(OC961_CTRL + 0x14 + 4 * $k);
                        c.g[V0][0] = sx(if n == word { h } else { h ^ (n + 1) });
                    }
                }
            )*
            pub(crate) fn oc961_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc961_fakes!(
        (
            oc961_f_14dc80,
            OC961_P_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            0,
            0
        ),
        (
            oc961_f_14dd58,
            OC961_P_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            0,
            0
        ),
        (
            oc961_f_146e98,
            OC961_P_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98,
            0,
            0
        ),
        (
            oc961_f_147398,
            OC961_P_147398,
            ee::s01::OVERRIDE_00147398,
            ee::s01::f_00147398__generated,
            0x0014_7398,
            2,
            3
        ),
        (
            oc961_f_1464d0,
            OC961_P_1464D0,
            ee::s01::OVERRIDE_001464D0,
            ee::s01::f_001464d0__generated,
            0x0014_64d0,
            1,
            1
        ),
        (
            oc961_f_145ef0,
            OC961_P_145EF0,
            ee::s01::OVERRIDE_00145EF0,
            ee::s01::f_00145ef0__generated,
            0x0014_5ef0,
            1,
            2
        ),
        (
            oc961_f_3177f0,
            OC961_P_3177F0,
            ee::s10::OVERRIDE_003177F0,
            ee::s10::f_003177f0__generated,
            0x0031_77f0,
            0,
            0
        ),
        (
            oc961_f_317670,
            OC961_P_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            3,
            4
        ),
        (
            oc961_f_1e3a30,
            OC961_P_1E3A30,
            ee::s04::OVERRIDE_001E3A30,
            ee::s04::f_001e3a30__generated,
            0x001e_3a30,
            1,
            0
        ),
        (
            oc961_f_1e3a78,
            OC961_P_1E3A78,
            ee::s04::OVERRIDE_001E3A78,
            ee::s04::f_001e3a78__generated,
            0x001e_3a78,
            0,
            0
        ),
        (
            oc961_f_1e2fe0,
            OC961_P_1E2FE0,
            ee::s04::OVERRIDE_001E2FE0,
            ee::s04::f_001e2fe0__generated,
            0x001e_2fe0,
            0,
            0
        ),
        (
            oc961_f_1e2220,
            OC961_P_1E2220,
            ee::s04::OVERRIDE_001E2220,
            ee::s04::f_001e2220__generated,
            0x001e_2220,
            0,
            0
        ),
        (
            oc961_f_1e20d0,
            OC961_P_1E20D0,
            ee::s04::OVERRIDE_001E20D0,
            ee::s04::f_001e20d0__generated,
            0x001e_20d0,
            0,
            0
        ),
        (
            oc961_f_1e2bb8,
            OC961_P_1E2BB8,
            ee::s04::OVERRIDE_001E2BB8,
            ee::s04::f_001e2bb8__generated,
            0x001e_2bb8,
            2,
            6
        ),
        (
            oc961_f_1e2c80,
            OC961_P_1E2C80,
            ee::s04::OVERRIDE_001E2C80,
            ee::s04::f_001e2c80__generated,
            0x001e_2c80,
            0,
            0
        ),
        (
            oc961_f_1e2ea0,
            OC961_P_1E2EA0,
            ee::s04::OVERRIDE_001E2EA0,
            ee::s04::f_001e2ea0__generated,
            0x001e_2ea0,
            0,
            0
        ),
        (
            oc961_f_1e33e8,
            OC961_P_1E33E8,
            ee::s04::OVERRIDE_001E33E8,
            ee::s04::f_001e33e8__generated,
            0x001e_33e8,
            2,
            7
        ),
        (
            oc961_f_1e3620,
            OC961_P_1E3620,
            ee::s04::OVERRIDE_001E3620,
            ee::s04::f_001e3620__generated,
            0x001e_3620,
            0,
            0
        ),
        (
            oc961_f_1e3650,
            OC961_P_1E3650,
            ee::s04::OVERRIDE_001E3650,
            ee::s04::f_001e3650__generated,
            0x001e_3650,
            0,
            0
        ),
        (
            oc961_f_1e39f8,
            OC961_P_1E39F8,
            ee::s04::OVERRIDE_001E39F8,
            ee::s04::f_001e39f8__generated,
            0x001e_39f8,
            2,
            8
        ),
        (
            oc961_f_1e4840,
            OC961_P_1E4840,
            ee::s04::OVERRIDE_001E4840,
            ee::s04::f_001e4840__generated,
            0x001e_4840,
            0,
            0
        ),
        (
            oc961_f_1e4878,
            OC961_P_1E4878,
            ee::s04::OVERRIDE_001E4878,
            ee::s04::f_001e4878__generated,
            0x001e_4878,
            0,
            0
        ),
        (
            oc961_f_1e4b80,
            OC961_P_1E4B80,
            ee::s04::OVERRIDE_001E4B80,
            ee::s04::f_001e4b80__generated,
            0x001e_4b80,
            0,
            0
        ),
        (
            oc961_f_1e4c78,
            OC961_P_1E4C78,
            ee::s04::OVERRIDE_001E4C78,
            ee::s04::f_001e4c78__generated,
            0x001e_4c78,
            0,
            0
        ),
        (
            oc961_f_1e4f70,
            OC961_P_1E4F70,
            ee::s04::OVERRIDE_001E4F70,
            ee::s04::f_001e4f70__generated,
            0x001e_4f70,
            0,
            0
        ),
        (
            oc961_f_39b760,
            OC961_P_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760,
            0,
            0
        ),
        (
            oc961_f_39e8b8,
            OC961_P_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8,
            0,
            0
        ),
        (
            oc961_f_39e948,
            OC961_P_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948,
            0,
            0
        ),
        (
            oc961_f_3a0c50,
            OC961_P_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            0,
            0
        ),
        (
            oc961_f_3a1000,
            OC961_P_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            0,
            0
        ),
        (
            oc961_f_417828,
            OC961_P_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            0,
            0
        ),
    );

    /// A vtable at `DATA + 0x8000` whose slots 9 and 27 hold `{delta, fake}`
    /// (the widgets' `vt[9].fn` / `vt[27].fn` calls land in the miss hook).
    fn oc961_vtable(c: &mut Cpu, r: &mut Rng) -> u32 {
        let vt = DATA + 0x8000;
        for (i, off) in [0x48u32, 0xd8].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
        vt
    }

    /// The `Game->Replay` state word the message-center updaters check first:
    /// `gp-0x848` -> world, `+0x84` -> game, `+0x28` -> replay, `[0]`.
    fn oc961_replay(c: &mut Cpu, state: u32) {
        let world = DATA + 0x1000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, DATA + 0x1100);
        c.bus.write32(DATA + 0x1100 + 0x28, DATA + 0x1200);
        c.bus.write32(DATA + 0x1200, state);
    }

    /// Replay state by seed: idle (0, >= 10) in 5 of 8, busy (1..9, negative)
    /// otherwise.
    fn oc961_replay_state(seed: u32) -> u32 {
        [0, 0, 0, 0, 3, 9, 10, u32::MAX][(seed % 8) as usize]
    }

    /// Number of distinct cycle totals the generated body takes over the seeds.
    fn oc961_paths(f: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, f);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    fn oc961_check_event_messages_setup(c: &mut Cpu, seed: u32) {
        oc961_install_fakes();
        let mut r = machine(c, seed, 0x1e1550, 14);
        oc961_replay(c, oc961_replay_state(seed));
        c.bus.write32(OC961_CTRL, OC961_MAGIC);
        // Flag set (skip), rival locked, event played: about one in four.
        for k in 0..3 {
            let m = r.next_u32() & r.next_u32();
            oc961_mail(c, k, m);
        }
        // The human rider's character: 3, 7, 8 select the own-character groups.
        oc961_mail(c, 3, [3, 7, 8, 1, 0, 5][(seed / 8 % 6) as usize]);
        c.pc = 0x001e_1550;
    }

    /// `MessageCenter_CheckEventMessages`: the six rival-event slots, with the
    /// flag, lock and played-event answers and the character from mailboxes.
    #[test]
    fn message_center_check_event_messages_matches() {
        check(
            ee::s04::f_001e1550__generated,
            message_center_check_event_messages,
            300,
            &oc961_check_event_messages_setup,
        );
        assert!(
            oc961_paths(
                ee::s04::f_001e1550__generated,
                300,
                &oc961_check_event_messages_setup
            ) >= 12
        );
    }

    fn oc961_process_pending_peak_flags_setup(c: &mut Cpu, seed: u32) {
        oc961_install_fakes();
        let _r = machine(c, seed, 0x1e1eb8, 14);
        oc961_replay(c, oc961_replay_state(seed));
        c.bus.write32(OC961_CTRL, OC961_MAGIC);
        let mut r = Rng(0x1e1eb8 ^ (seed as u64) << 9);
        oc961_mail(c, 0, r.next_u32());
        c.pc = 0x001e_1eb8;
    }

    /// `MessageCenter_ProcessPendingPeakFlags`: flags 40..51 (pairs per peak,
    /// A and B kinds), each answered by a random mask bit.
    #[test]
    fn message_center_process_pending_peak_flags_matches() {
        check(
            ee::s04::f_001e1eb8__generated,
            message_center_process_pending_peak_flags,
            300,
            &oc961_process_pending_peak_flags_setup,
        );
        assert!(
            oc961_paths(
                ee::s04::f_001e1eb8__generated,
                300,
                &oc961_process_pending_peak_flags_setup
            ) >= 6
        );
    }

    pub(crate) const OC961_WIDGET_SELF: u32 = DATA + 0x2000;
    pub(crate) const OC961_WIDGET: u32 = DATA + 0x4000;

    fn oc961_on_widget_created_setup(c: &mut Cpu, seed: u32) {
        oc961_install_fakes();
        let mut r = machine(c, seed, 0x1e3ec0, 24);
        c.bus.write32(OC961_CTRL, OC961_MAGIC);
        let hash = r.next_u32() | 1;
        c.bus.write32(OC961_WIDGET + 0x38, hash);
        let vt = oc961_vtable(c, &mut r);
        c.bus.write32(OC961_WIDGET + 8, vt);
        // The hash call that matches: sweeps every name; 90..99 match none.
        oc961_mail(c, 4, seed % 100);
        oc961_mail(c, 5, hash);
        for off in (0x13c..0x13c + 0x100u32).step_by(4) {
            c.bus.write32(OC961_WIDGET_SELF + off, r.next_u32());
        }
        c.g[A0][0] = sx(OC961_WIDGET_SELF);
        c.g[A1][0] = sx(OC961_WIDGET);
        c.pc = 0x001e_3ec0;
    }

    /// `OVScreenMessageCenter_OnWidgetCreated`: the widget's name hash picks
    /// the `self` field (or row array slot) it is stored into; the hash
    /// compare fake matches the call number from the mailbox.
    #[test]
    fn ov_screen_message_center_on_widget_created_matches() {
        check(
            ee::s04::f_001e3ec0__generated,
            ov_screen_message_center_on_widget_created,
            200,
            &oc961_on_widget_created_setup,
        );
        assert!(
            oc961_paths(
                ee::s04::f_001e3ec0__generated,
                200,
                &oc961_on_widget_created_setup
            ) >= 12
        );
    }

    fn oc961_refresh_list_setup(c: &mut Cpu, seed: u32) {
        oc961_install_fakes();
        let mut r = machine(c, seed, 0x1e48b0, 40);
        c.bus.write32(OC961_CTRL, OC961_MAGIC);
        let me = OC961_WIDGET_SELF;
        // The online (FAQ) flag, the counts, the scroll offset.
        c.bus.write32(GAME_GP.wrapping_sub(0x102c), seed / 2 % 2);
        let (online, local) = if seed % 7 == 0 {
            (0, 0)
        } else {
            (r.next_u32() % 4, r.next_u32() % 12)
        };
        oc961_mail(c, 6, online);
        oc961_mail(c, 7, local);
        for off in (0x9c..0x180u32).step_by(4) {
            c.bus.write32(me + off, r.next_u32());
        }
        let menu = DATA + 0x5000;
        c.bus.write32(me + 0x144, menu);
        c.bus.write32(me + 0x174, r.next_u32() % 6);
        c.bus.write32(menu + 0x94, r.next_u32());
        c.g[A0][0] = sx(me);
        c.pc = 0x001e_48b0;
    }

    /// `OVScreenMessageCenter_RefreshList`: empty list (hide widgets, show
    /// "no messages"), or the eight rows filled from local messages, FAQ
    /// messages or hidden, then the menu's scroll state restored.
    #[test]
    fn ov_screen_message_center_refresh_list_matches() {
        check(
            ee::s04::f_001e48b0__generated,
            ov_screen_message_center_refresh_list,
            300,
            &oc961_refresh_list_setup,
        );
        assert!(
            oc961_paths(
                ee::s04::f_001e48b0__generated,
                300,
                &oc961_refresh_list_setup
            ) >= 6
        );
    }

    fn oc961_set_row_setup(c: &mut Cpu, seed: u32) {
        oc961_install_fakes();
        let mut r = machine(c, seed, 0x1e4c78, 12);
        c.bus.write32(OC961_CTRL, OC961_MAGIC);
        let me = OC961_WIDGET_SELF;
        let vt = oc961_vtable(c, &mut r);
        // Message and group tables: 0x18 and 0x14 bytes per entry.
        let id = r.next_u32() % 8;
        for i in 0..8u32 {
            let ent = 0x004c_6c08 + 0x18 * i;
            c.bus.write32(ent + 4, r.next_u32() % 3);
            c.bus
                .write32(ent + 8, [1, 0, 2, 1][(r.next_u32() % 4) as usize]);
            c.bus.write32(ent + 0xc, r.next_u32());
            c.bus.write32(ent + 0x10, r.next_u32());
        }
        let group = c.bus.read32(0x004c_6c08 + 0x18 * id + 4);
        for g in 0..3u32 {
            let ent = 0x0044_1630 + 0x14 * g;
            let first = if g == group && seed % 2 == 0 {
                id
            } else {
                r.next_u32() % 8
            };
            c.bus.write32(ent + 4, first);
            c.bus.write32(ent + 0x10, (r.next_u32() % 3 != 0) as u32);
        }
        // The row widgets: five arrays of eight (a9C, aBC, aDC, aFC, a11C);
        // the first two are never null, the others about one in four.
        for (arr, base) in [0x9cu32, 0xbc, 0xdc, 0xfc, 0x11c].into_iter().enumerate() {
            for i in 0..8u32 {
                let w = OC961_WIDGET + 0x80 * (8 * arr as u32 + i);
                let p = if arr >= 2 && r.next_u32() % 4 == 0 {
                    0
                } else {
                    w
                };
                c.bus.write32(me + base + 4 * i, p);
                c.bus.write32(w + 8, vt);
            }
        }
        for off in [0x164u32, 0x168, 0x16c, 0x174] {
            c.bus.write32(me + off, r.next_u32());
        }
        // The inbox (`GetInbox` result): the read-mask word at +0xc8.
        let inbox = DATA + 0x6000;
        c.bus.write32(inbox + 0xc8, r.next_u32());
        oc961_mail(c, 8, inbox);
        oc961_mail(c, 0, r.next_u32());
        let idx: u64 = if seed % 5 == 0 {
            u64::MAX
        } else {
            (r.next_u32() % 32) as u64
        };
        c.g[A0][0] = sx(me);
        c.g[A1][0] = idx;
        c.g[A2][0] = (r.next_u32() % 8) as u64;
        c.g[A3][0] = id as u64;
        c.g[T0][0] = r.next_u32() as u64;
        c.pc = 0x001e_4c78;
    }

    /// `OVScreenMessageCenter_SetRow`: one list row from message `id`: read /
    /// unread bitmaps or the group header tint, the row's marker text, sender
    /// name and subject, with the widgets of the row arrays (some null).
    #[test]
    fn ov_screen_message_center_set_row_matches() {
        check(
            ee::s04::f_001e4c78__generated,
            ov_screen_message_center_set_row,
            300,
            &oc961_set_row_setup,
        );
        assert!(oc961_paths(ee::s04::f_001e4c78__generated, 300, &oc961_set_row_setup) >= 8);
    }
}
