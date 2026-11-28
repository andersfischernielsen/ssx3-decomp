//! Staging module oc1600 (wave wave280): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1600`.

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
    (0x0010_0000, oc1600_crt0_jump, "lbl_00100000"),
    (
        0x0015_77e0,
        oc1600_is_peak_fully_completed,
        "BERewardInt_IsPeakFullyCompleted",
    ),
    (
        0x001e_58e0,
        oc1600_view_popup_show_next,
        "cOVState_ViewPopup_ShowNext",
    ),
    (
        0x001f_b0c0,
        oc1600_ingame_error_on_widget_action,
        "OVScreenInGameError_OnWidgetAction",
    ),
    (0x002a_82d8, oc1600_midi_banks_dtor, "MIDIBanks_dtor"),
    (
        0x002a_9128,
        oc1600_release_spu_block,
        "PS2SoundManager_ReleaseSpuBlock",
    ),
    (0x002a_a648, oc1600_stream_array_dtor, "StreamArray_dtor"),
];

/// `lbl_00100000` (0x00100000): the 8-byte stub in front of the program entry: charges two
/// cycles and jumps (tail call, no frame) to `entry` (0x00100008).
pub fn oc1600_crt0_jump(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = 0x0010_0008;
    ee::s00::f_00100008(c);
}

/// `bool BERewardInt_IsPeakFullyCompleted(self, peak, medal, index)` (0x001577e0): true when all three peak medals pass `BERewardInt_IsPeakGoldComplete` and the eight highlight record levels of the score interface sum to at least 24.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn oc1600_is_peak_fully_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S3][0];
        'l_157810: loop {
            'b_157884: {
                'b_157830: {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00157820);
                    c.g[A3][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00157920,
                        f: ee::s01::BERewardInt_IsPeakGoldComplete,
                    }
                    .call(c, 0x00157820)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_157830;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_157884;
                        } else {
                            break 'b_157830;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.g[S3][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x00157810);
                    }
                    continue 'l_157810;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00157844);
                    c.g[S1][0] = 0u64;
                    Callee {
                        addr: 0x0014dc80,
                        f: ee::s01::BELibrary_GetInstance,
                    }
                    .call(c, 0x00157844)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00157850);
                    c.g[A1][0] = 0x8;
                    Callee {
                        addr: 0x0014dd58,
                        f: ee::s01::BELibrary_GetInterface,
                    }
                    .call(c, 0x00157850)?;
                    c.g[S0][0] = 0u64;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    'l_157860: loop {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x00157868);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001552b0,
                            f: ee::s01::BEScoreInt_GetRecordLevel,
                        }
                        .call(c, 0x00157868)?;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 8i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00157860);
                            }
                            continue 'l_157860;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            break 'b_157884;
                        }
                    }
                }
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
        }
    });
}

/// `void cOVState_ViewPopup_ShowNext(self)` (0x001e58e0): for the current popup message group, takes the next message in the group (or the first unread one of the inbox, marking it read) and shows it; otherwise calls the widget method at +0x8c of the owner (close/refresh).
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn oc1600_view_popup_show_next(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e5a04: {
            'b_1e59e4: {
                'b_1e59d4: {
                    'b_1e5984: {
                        c.cycles += 21;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.g[V1][0] = 0x18;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A1][0] = 0x4c0000;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S1][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c08));
                        c.g[A3][0] = 0x14;
                        c.g[A0][0] = 0x440000;
                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
                        c.g[A2][0] = 0x4;
                        c.mult(0, 3, 8, 3);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.mult(0, 2, 2, 7);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        let t = c.g[V1][0] != c.g[A2][0];

                        if t {
                            break 'b_1e5984;
                        } else {
                            'b_1e5974: {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x001e593c);
                                c.g[A0][0] = c.g[T0][0];
                                Callee {
                                    addr: 0x001e34d8,
                                    f: ee::s04::MessageCenter_GetNextInGroup,
                                }
                                .call(c, 0x001e593c)?;
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffffffffffffffff;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    break 'b_1e5974;
                                } else {
                                    c.cycles += 10;
                                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = 0x14;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001e596c);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001e596c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1e5a04;
                                    } else {
                                        break 'b_1e5974;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S1][0];
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                            let t = true;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_1e59d4;
                            } else {
                                break 'b_1e5984;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e598c);
                    Callee {
                        addr: 0x001e3408,
                        f: ee::s04::MessageCenter_GetUnreadCount,
                    }
                    .call(c, 0x001e598c)?;
                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        break 'b_1e59e4;
                    } else {
                        c.cycles += 16;
                        c.g[RA][0] = sx(0x001e599c);
                        Callee {
                            addr: 0x001e39f8,
                            f: ee::s04::MessageCenter_GetInbox,
                        }
                        .call(c, 0x001e599c)?;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x001e59a8);
                        c.g[S0][0] = c.g[V0][0];
                        Callee {
                            addr: 0x001e3460,
                            f: ee::s04::MessageCenter_GetFirstUnread,
                        }
                        .call(c, 0x001e59a8)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                        c.g[V1][0] = 0x1;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.g[RA][0] = sx(0x001e59c8);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                        Callee {
                            addr: 0x001e33e8,
                            f: ee::s04::MessageCenter_GetMessageCount,
                        }
                        .call(c, 0x001e59c8)?;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                        break 'b_1e59d4;
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001e59dc);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                Callee {
                    addr: 0x001e5428,
                    f: ee::s04::cOVState_ViewPopup_ShowMessage,
                }
                .call(c, 0x001e59dc)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1e5a04;
                } else {
                    break 'b_1e59e4;
                }
            }
            c.cycles += 8;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x14;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001e5a00);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001e5a00)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1e5a04;
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

/// `int OVScreenInGameError_OnWidgetAction(self, UIWidget* w, int action)` (0x001fb0c0): action 5 selects (`HandleSelect`), action 1 (focus) moves the highlight between the Cancel and Continue buttons by comparing the widget hash with the two button names.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn oc1600_ingame_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fb16c: {
            'b_1fb168: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                let t = c.g[S1][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1fb168;
                } else {
                    'b_1fb104: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_1fb104;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1fb16c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001fb0fc);
                                c.g[A2][0] = 0x5;
                                Callee {
                                    addr: 0x001fb2b8,
                                    f: ee::s04::OVScreenInGameError_HandleSelect,
                                }
                                .call(c, 0x001fb0fc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1fb16c;
                                } else {
                                    break 'b_1fb104;
                                }
                            }
                        }
                    }
                    'b_1fb13c: {
                        c.cycles += 6;
                        c.g[A0][0] = 0x470000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[RA][0] = sx(0x001fb114);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                        call_ptr(c, 0x00317670, 0x001fb114)?;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fb13c;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fb128);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001fb588,
                                f: ee::s04::OVScreenInGameError_HighlightCancel,
                            }
                            .call(c, 0x001fb128)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fb134);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001fb458,
                                f: ee::s04::OVScreenInGameError_HighlightContinue,
                            }
                            .call(c, 0x001fb134)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1fb16c;
                            } else {
                                break 'b_1fb13c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001fb148);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4b8));
                    call_ptr(c, 0x00317670, 0x001fb148)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1fb16c;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001fb15c);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001fb458,
                            f: ee::s04::OVScreenInGameError_HighlightContinue,
                        }
                        .call(c, 0x001fb15c)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001fb168);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001fb588,
                            f: ee::s04::OVScreenInGameError_HighlightCancel,
                        }
                        .call(c, 0x001fb168)?;
                        break 'b_1fb168;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1fb16c;
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

/// `void MIDIBanks_dtor(MIDIBanks* self, int flags)` (0x002a82d8): resets the vtables (flags 0: also the stack copies with the this-adjust fixed up), frees the banks block at +0x10, clears the global, shuts the MIDI player down, runs the base destructor, and per flag bit 1 / bit 0 destroys the bank manager and sound manager reference / frees self.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn oc1600_midi_banks_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a83d4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = 0x480000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a48));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[A3][0] = 0x480000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x39f0));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
            c.g[A0][0] = 0x480000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[T0][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3a10));
            c.g[S1][0] = c.g[A1][0];
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2a83d4;
            } else {
                c.cycles += 40;
                c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x39f0));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd((c.g[SP][0] as u32), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x3a10));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw((c.g[S0][0] as u32));
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_2a83d4;
            }
        }
        'b_2a83e8: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2a83e8;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002a83e8);
                call_ptr(c, 0x00317e98, 0x002a83e8)?;
                break 'b_2a83e8;
            }
        }
        'b_2a8428: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x002a83f0);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x654), 0u64);
            call_ptr(c, 0x003b9880, 0x002a83f0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002a83fc);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0028fea0, 0x002a83fc)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if t {
                break 'b_2a8428;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x002a8414);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0028bb10, 0x002a8414)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x002a8424);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                Callee {
                    addr: 0x002adee8,
                    f: ee::s08::SoundManagerRef_dtor,
                }
                .call(c, 0x002a8424)?;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                break 'b_2a8428;
            }
        }
        'b_2a843c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                break 'b_2a843c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a8438);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x002a8438)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                break 'b_2a843c;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// Resumable: any head `pc` of the generated body continues natively.
/// `void PS2SoundManager_ReleaseSpuBlock(self, int block)` (0x002a9128): marks the SPU block free, merges it with an adjacent free block of the same size through the size-sorted free list (head index at +0x318, entries of 0x18 bytes from +0xc), freeing the SPU memory as it goes.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn oc1600_release_spu_block(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002a9128 => {
                    c.cycles += 15;
                    c.g[A2][0] = 0x18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.mult(0, 5, 5, 6);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = 0x1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30c));

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x318));
                        pc = 0x002a91b8;
                        continue;
                    }
                    pc = 0x002a9164;
                    continue;
                }
                0x002a9164 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002a916c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002a92f8,
                        f: ee::s08::PS2SoundManager_FreeSpuBlock,
                    }
                    .call(c, 0x002a916c)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x318));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[S1][0] = 0x18;
                    if t {
                        pc = 0x002a923c;
                        continue;
                    }
                    pc = 0x002a9178;
                    continue;
                }
                0x002a9178 => {
                    c.cycles += 2;
                    c.mult(0, 2, 2, 17);
                    pc = 0x002a9180;
                    continue;
                }
                0x002a9180 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x002a923c;
                        continue;
                    }
                    pc = 0x002a9198;
                    continue;
                }
                0x002a9198 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x002a91a4);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x318), c.g[V0][0]);
                    Callee {
                        addr: 0x002a92f8,
                        f: ee::s08::PS2SoundManager_FreeSpuBlock,
                    }
                    .call(c, 0x002a91a4)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x318));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.mult(0, 2, 2, 17);
                    if t {
                        if back_edge_unwinds(c, 0x002a9180) {
                            return Ok(());
                        }
                        pc = 0x002a9180;
                        continue;
                    }
                    pc = 0x002a91b0;
                    continue;
                }
                0x002a91b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x002a9240;
                        continue;
                    }
                    pc = 0x002a91b8;
                    continue;
                }
                0x002a91b8 => {
                    c.cycles += 2;
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.mult(0, 2, 3, 6);
                    if t {
                        pc = 0x002a91ec;
                        continue;
                    }
                    pc = 0x002a91c0;
                    continue;
                }
                0x002a91c0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                    if t {
                        pc = 0x002a9220;
                        continue;
                    }
                    pc = 0x002a91c8;
                    continue;
                }
                0x002a91c8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    if t {
                        pc = 0x002a923c;
                        continue;
                    }
                    pc = 0x002a91d8;
                    continue;
                }
                0x002a91d8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    if t {
                        pc = 0x002a923c;
                        continue;
                    }
                    pc = 0x002a91ec;
                    continue;
                }
                0x002a91ec => {
                    c.cycles += 3;
                    c.g[A3][0] = c.g[A0][0];
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0x18;
                    pc = 0x002a91f8;
                    continue;
                }
                0x002a91f8 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                        pc = 0x002a922c;
                        continue;
                    }
                    pc = 0x002a9210;
                    continue;
                }
                0x002a9210 => {
                    c.cycles += 2;

                    if c.g[A2][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x002a91c8) {
                            return Ok(());
                        }
                        pc = 0x002a91c8;
                        continue;
                    }
                    pc = 0x002a9218;
                    continue;
                }
                0x002a9218 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                    pc = 0x002a9220;
                    continue;
                }
                0x002a9220 => {
                    c.cycles += 3;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x318), c.g[V0][0]);
                    if t {
                        pc = 0x002a923c;
                        continue;
                    }
                    pc = 0x002a922c;
                    continue;
                }
                0x002a922c => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.mult(0, 2, 2, 8);
                    if t {
                        if back_edge_unwinds(c, 0x002a91d8) {
                            return Ok(());
                        }
                        pc = 0x002a91d8;
                        continue;
                    }
                    pc = 0x002a9234;
                    continue;
                }
                0x002a9234 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = c.g[A0][0];
                    if t {
                        if back_edge_unwinds(c, 0x002a91f8) {
                            return Ok(());
                        }
                        pc = 0x002a91f8;
                        continue;
                    }
                    pc = 0x002a923c;
                    continue;
                }
                0x002a923c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x002a9240;
                    continue;
                }
                0x002a9240 => {
                    c.cycles += 4;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a9128, 0x002a9250);
                    return Ok(());
                }
            }
        }
    });
}

/// `void StreamArray_dtor(StreamArray* self, int flags)` (0x002aa648): resets the vtables, destroys every stream buffer, frees the stream table, and per flag bit 1 / bit 0 destroys the bank manager and sound manager reference / frees self.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn oc1600_stream_array_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2aa740: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[A3][0] = 0x480000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3998));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A0][0] = 0x480000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[T0][0] = 0x480000;
            c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x39b8));
            c.g[T6][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
            let t = c.g[T6][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2aa740;
            } else {
                c.cycles += 40;
                c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3998));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd((c.g[SP][0] as u32), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x39b8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw((c.g[S1][0] as u32));
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_2aa740;
            }
        }
        'b_2aa788: {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[S3][0] = c.g[T6][0] & 0x1;
            c.g[S0][0] = 0u64;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.g[T6][0] & 0x2;
            if t {
                break 'b_2aa788;
            } else {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                'l_2aa758: loop {
                    'b_2aa778: {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                        if c.g[A0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            break 'b_2aa778;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x002aa774);
                            c.g[A1][0] = 0x3;
                            Callee {
                                addr: 0x002a9df0,
                                f: ee::s08::StreamBuffer_dtor,
                            }
                            .call(c, 0x002aa774)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            break 'b_2aa778;
                        }
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x002aa758);
                        }
                        continue 'l_2aa758;
                    } else {
                        break 'b_2aa788;
                    }
                }
            }
        }
        'b_2aa79c: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2aa79c;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002aa79c);
                call_ptr(c, 0x00317e98, 0x002aa79c)?;
                break 'b_2aa79c;
            }
        }
        'b_2aa7c0: {
            c.cycles += 2;
            let t = c.g[S2][0] == 0u64;

            if t {
                break 'b_2aa7c0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                c.g[RA][0] = sx(0x002aa7b0);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0028bb10, 0x002aa7b0)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x002aa7c0);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                Callee {
                    addr: 0x002adee8,
                    f: ee::s08::SoundManagerRef_dtor,
                }
                .call(c, 0x002aa7c0)?;
                break 'b_2aa7c0;
            }
        }
        'b_2aa7d4: {
            c.cycles += 2;
            let t = c.g[S3][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            if t {
                break 'b_2aa7d4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002aa7d0);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00317e50, 0x002aa7d0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_2aa7d4;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's RAM:
    /// other tests' machines run the real bodies). The words after it are the
    /// mailbox the fakes answer from.
    const OC1600_CTRL: u32 = DATA + 0xe_0000;
    const OC1600_MAGIC: u32 = 0x1600_f00d;
    /// Fixture objects, clear of the allocator pools and the call log.
    const OC1600_BASE: u32 = DATA + 0x8_0000;

    fn oc1600_ans_none(_c: &mut Cpu) -> Option<u32> {
        None
    }

    /// `IsPeakGoldComplete(self, a, b, i)`: bit `i` of mailbox word 0.
    fn oc1600_ans_gold(c: &mut Cpu) -> Option<u32> {
        let i = c.g[A3][0] as u32 & 31;
        Some((c.bus.read32(OC1600_CTRL + 0x10) >> i) & 1)
    }

    /// `BEScoreInt_GetRecordLevel(self, i)`: nibble `i` of mailbox word 1.
    fn oc1600_ans_level(c: &mut Cpu) -> Option<u32> {
        let i = c.g[A1][0] as u32 & 7;
        Some((c.bus.read32(OC1600_CTRL + 0x14) >> (4 * i)) & 0xf)
    }

    /// `MessageCenter_GetNextInGroup`: mailbox word 0.
    fn oc1600_ans_next_in_group(c: &mut Cpu) -> Option<u32> {
        Some(c.bus.read32(OC1600_CTRL + 0x10))
    }

    /// `MessageCenter_GetUnreadCount`: mailbox word 1.
    fn oc1600_ans_unread(c: &mut Cpu) -> Option<u32> {
        Some(c.bus.read32(OC1600_CTRL + 0x14))
    }

    /// `StrHash(name)`: mailbox word 0 for the Cancel name, word 1 for the Continue name.
    fn oc1600_ans_strhash(c: &mut Cpu) -> Option<u32> {
        let word = if c.g[A0][0] as u32 == 0x0046_fa90 {
            0x10
        } else {
            0x14
        };
        Some(c.bus.read32(OC1600_CTRL + word))
    }

    /// Gated fakes on the callee slots: with the gate word set they log the call like
    /// `fakes::fake_call` does and (optionally) force the result in v0; with the gate off
    /// they run whatever the slot held before.
    macro_rules! oc1600_gated {
        ($($name:ident, $addr:expr, $slot:path, $gen:path, $ans:expr;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1600_CTRL) != OC1600_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    let answer: Option<u32> = ($ans)(c);
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        if let Some(v) = answer {
                            c.g[V0][0] = sx(v);
                        }
                    }
                }
            )*
            fn oc1600_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1600_gated! {
        oc1600_fake_crt_entry, 0x0010_0008, ee::s00::OVERRIDE_00100008, ee::s00::f_00100008__generated, oc1600_ans_none;
        oc1600_fake_gold, 0x0015_7920, ee::s01::OVERRIDE_00157920, ee::s01::f_00157920__generated, oc1600_ans_gold;
        oc1600_fake_get_instance, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, oc1600_ans_none;
        oc1600_fake_get_interface, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, oc1600_ans_none;
        oc1600_fake_record_level, 0x0015_52b0, ee::s01::OVERRIDE_001552B0, ee::s01::f_001552b0__generated, oc1600_ans_level;
        oc1600_fake_next_in_group, 0x001e_34d8, ee::s04::OVERRIDE_001E34D8, ee::s04::f_001e34d8__generated, oc1600_ans_next_in_group;
        oc1600_fake_unread_count, 0x001e_3408, ee::s04::OVERRIDE_001E3408, ee::s04::f_001e3408__generated, oc1600_ans_unread;
        oc1600_fake_get_inbox, 0x001e_39f8, ee::s04::OVERRIDE_001E39F8, ee::s04::f_001e39f8__generated, oc1600_ans_none;
        oc1600_fake_first_unread, 0x001e_3460, ee::s04::OVERRIDE_001E3460, ee::s04::f_001e3460__generated, oc1600_ans_none;
        oc1600_fake_message_count, 0x001e_33e8, ee::s04::OVERRIDE_001E33E8, ee::s04::f_001e33e8__generated, oc1600_ans_none;
        oc1600_fake_show_message, 0x001e_5428, ee::s04::OVERRIDE_001E5428, ee::s04::f_001e5428__generated, oc1600_ans_none;
        oc1600_fake_handle_select, 0x001f_b2b8, ee::s04::OVERRIDE_001FB2B8, ee::s04::f_001fb2b8__generated, oc1600_ans_none;
        oc1600_fake_highlight_cancel, 0x001f_b588, ee::s04::OVERRIDE_001FB588, ee::s04::f_001fb588__generated, oc1600_ans_none;
        oc1600_fake_highlight_continue, 0x001f_b458, ee::s04::OVERRIDE_001FB458, ee::s04::f_001fb458__generated, oc1600_ans_none;
        oc1600_fake_str_hash, 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, oc1600_ans_strhash;
        oc1600_fake_mem_free, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, oc1600_ans_none;
        oc1600_fake_delete, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated, oc1600_ans_none;
        oc1600_fake_midi_shutdown, 0x003b_9880, ee::s13::OVERRIDE_003B9880, ee::s13::f_003b9880__generated, oc1600_ans_none;
        oc1600_fake_midi_base_dtor, 0x0028_fea0, ee::s07::OVERRIDE_0028FEA0, ee::s07::f_0028fea0__generated, oc1600_ans_none;
        oc1600_fake_bank_man_dtor, 0x0028_bb10, ee::s07::OVERRIDE_0028BB10, ee::s07::f_0028bb10__generated, oc1600_ans_none;
        oc1600_fake_sound_ref_dtor, 0x002a_dee8, ee::s08::OVERRIDE_002ADEE8, ee::s08::f_002adee8__generated, oc1600_ans_none;
        oc1600_fake_stream_buffer_dtor, 0x002a_9df0, ee::s08::OVERRIDE_002A9DF0, ee::s08::f_002a9df0__generated, oc1600_ans_none;
        oc1600_fake_free_spu_block, 0x002a_92f8, ee::s08::OVERRIDE_002A92F8, ee::s08::f_002a92f8__generated, oc1600_ans_none;
    }

    /// Machine with the fakes hooked, the gate open and a per-seed hash for the fixture.
    fn oc1600_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> u64 {
        oc1600_hook();
        fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1600_CTRL, OC1600_MAGIC);
        fakes::mix(seed as u64 ^ salt.wrapping_mul(0x9e37_79b9))
    }

    /// `lbl_00100000`: the tail jump to the program entry (2 cycles, `pc = 0x100008`).
    #[test]
    fn oc1600_crt0_jump_matches() {
        check(
            ee::s00::f_00100000__generated,
            oc1600_crt0_jump,
            8,
            &|c, seed| {
                oc1600_machine(c, seed, 0x1600_0, 1);
            },
        );
    }

    /// The three peak medals (mask bits), then the eight record levels (nibbles) that must
    /// sum to at least 24.
    #[test]
    fn oc1600_is_peak_fully_completed_matches() {
        check_reaching(
            ee::s01::f_001577e0__generated,
            oc1600_is_peak_fully_completed,
            300,
            &[0x0015_7820, 0x0015_7844, 0x0015_7850, 0x0015_7868],
            &|c, seed| {
                let h = oc1600_machine(c, seed, 0x1600_1, 14);
                let mask = if h & 3 != 0 { 7 } else { (h >> 4) as u32 & 7 };
                c.bus.write32(OC1600_CTRL + 0x10, mask);
                let mut levels = 0u32;
                for i in 0..8 {
                    levels |= (((h >> (8 + 3 * i)) % 6) as u32) << (4 * i);
                }
                c.bus.write32(OC1600_CTRL + 0x14, levels);
            },
        );
    }

    /// Next message of the popup's group (or none, then the owner's widget method), or
    /// the first unread message of the inbox (marked read) and then `ShowMessage`.
    #[test]
    fn oc1600_view_popup_show_next_matches() {
        check_reaching(
            ee::s04::f_001e58e0__generated,
            oc1600_view_popup_show_next,
            300,
            &[
                0x001e_593c,
                0x001e_596c,
                0x001e_598c,
                0x001e_599c,
                0x001e_59a8,
                0x001e_59c8,
                0x001e_59dc,
                0x001e_5a00,
            ],
            &|c, seed| {
                let h = oc1600_machine(c, seed, 0x1600_2, 8);
                let t0 = ((h & 3) % 3) as u32;
                let (obj, owner, vt) = (OC1600_BASE, OC1600_BASE + 0x400, OC1600_BASE + 0x800);
                c.g[A0][0] = obj as u64;
                c.bus.write32(obj + 0x68, t0);
                c.bus.write32(obj + 0x20, owner);
                c.bus.write32(owner + 8, vt);
                c.bus.write16(vt + 0x88, ((h >> 12) as u16) & 0x3fc);
                c.bus.write32(vt + 0x8c, fakes::FAKE + 0x10);
                // The message-state table: group t0 -> row t0 -> state (4 = a message group).
                c.bus.write32(0x004c_6c08 + 0x18 * t0 + 4, t0);
                let in_group = (h >> 3) % 3 != 0;
                c.bus
                    .write32(0x0044_1630 + 0x14 * t0, if in_group { 4 } else { 5 });
                c.bus.write32(
                    OC1600_CTRL + 0x10,
                    [0xffff_ffffu32, 3][((h >> 5) & 1) as usize],
                );
                c.bus.write32(
                    OC1600_CTRL + 0x14,
                    [0u32, 0xffff_fffd, 2][((h >> 6) % 3) as usize],
                );
            },
        );
    }

    /// Action 5 selects, action 1 (focus) compares the widget's hash with the two button
    /// names (mailbox words 0 and 1) and moves the highlight; anything else, or no
    /// widget, does nothing.
    #[test]
    fn oc1600_ingame_error_on_widget_action_matches() {
        check_reaching(
            ee::s04::f_001fb0c0__generated,
            oc1600_ingame_error_on_widget_action,
            300,
            &[
                0x001f_b0fc,
                0x001f_b114,
                0x001f_b128,
                0x001f_b134,
                0x001f_b148,
                0x001f_b15c,
                0x001f_b168,
            ],
            &|c, seed| {
                let h = oc1600_machine(c, seed, 0x1600_3, 6);
                let (me, item) = (OC1600_BASE, OC1600_BASE + 0x400);
                let (cancel, cont) = (0x1111_0000 + seed, 0x2222_0000 + seed);
                c.bus.write32(OC1600_CTRL + 0x10, cancel);
                c.bus.write32(OC1600_CTRL + 0x14, cont);
                c.g[A0][0] = me as u64;
                c.g[A1][0] = if h % 7 == 0 { 0 } else { item as u64 };
                c.g[A2][0] = [1u64, 1, 5, 2][((h >> 8) & 3) as usize];
                c.bus.write32(
                    item + 0x38,
                    [cancel, cont, 0x3333_0000][((h >> 24) % 3) as usize],
                );
            },
        );
    }

    /// Object layout shared by the two destructors: `*self` is the owner with the
    /// reference object at `+0x1d8`.
    fn oc1600_dtor_object(c: &mut Cpu, h: u64) -> u32 {
        let (obj, owner, sub) = (OC1600_BASE, OC1600_BASE + 0x1000, OC1600_BASE + 0x2000);
        c.bus.write32(obj, owner);
        c.bus.write32(owner + 0x1d8, sub);
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = (h & 3) as u64;
        obj
    }

    /// Vtable resets, the banks block free, MIDI shutdown, base destructor, and the
    /// flag bits (1: free self, 2: destroy the bank manager and sound manager ref).
    #[test]
    fn oc1600_midi_banks_dtor_matches() {
        check_reaching(
            ee::s08::f_002a82d8__generated,
            oc1600_midi_banks_dtor,
            200,
            &[
                0x002a_83e8,
                0x002a_83f0,
                0x002a_83fc,
                0x002a_8414,
                0x002a_8424,
                0x002a_8438,
            ],
            &|c, seed| {
                let h = oc1600_machine(c, seed, 0x1600_4, 6);
                let obj = oc1600_dtor_object(c, h >> 3);
                c.g[A1][0] = (seed & 3) as u64;
                let block = if (h >> 7) & 1 != 0 {
                    OC1600_BASE + 0x3000
                } else {
                    0
                };
                c.bus.write32(obj + 0x10, block);
            },
        );
    }

    /// Vtable resets, every stream buffer destroyed, the table freed, and the flag bits.
    #[test]
    fn oc1600_stream_array_dtor_matches() {
        check_reaching(
            ee::s08::f_002aa648__generated,
            oc1600_stream_array_dtor,
            300,
            &[
                0x002a_a774,
                0x002a_a79c,
                0x002a_a7b0,
                0x002a_a7c0,
                0x002a_a7d0,
            ],
            &|c, seed| {
                let h = oc1600_machine(c, seed, 0x1600_5, 8);
                let obj = oc1600_dtor_object(c, h >> 3);
                c.g[A1][0] = (seed & 3) as u64;
                let count = [0u32, 1, 2, 3, 0xffff_ffff][((h >> 9) % 5) as usize];
                let table = if count == 0 && (h >> 14) & 1 != 0 {
                    0
                } else {
                    OC1600_BASE + 0x3000
                };
                c.bus.write32(obj + 4, count);
                c.bus.write32(obj + 8, table);
                for i in 0..3u32 {
                    let entry = if (h >> (16 + i)) & 1 != 0 {
                        OC1600_BASE + 0x4000 + 0x40 * i
                    } else {
                        0
                    };
                    c.bus.write32(OC1600_BASE + 0x3000 + 4 * i, entry);
                }
            },
        );
    }

    /// Free-list insertion: the block at index `a1` is marked free and merged with equal-sized
    /// neighbours (freeing SPU memory) or inserted into the size-sorted list.
    #[test]
    fn oc1600_release_spu_block_matches() {
        check_reaching(
            ee::s08::f_002a9128__generated,
            oc1600_release_spu_block,
            400,
            &[0x002a_916c, 0x002a_91a4],
            &|c, seed| {
                let h = oc1600_machine(c, seed, 0x1600_6, 4);
                c.pc = 0x002a_9128;
                let me = OC1600_BASE;
                c.g[A0][0] = me as u64;
                c.g[A1][0] = (h >> 3) % 6;
                let entry = |i: u32| me + 0xc + 0x18 * i;
                for i in 0..6u32 {
                    c.bus
                        .write32(entry(i) + 4, 1 + ((h >> (6 + 3 * i)) % 4) as u32);
                    let next = if i == 5 || (h >> (30 + i)) & 1 != 0 {
                        0xffff_ffff
                    } else {
                        i + 1
                    };
                    c.bus.write32(entry(i) + 0x14, next);
                }
                c.bus.write32(me + 0x30c, 1 + ((h >> 44) % 4) as u32);
                let head = if (h >> 50) & 7 == 0 {
                    0xffff_ffff
                } else {
                    ((h >> 53) % 6) as u32
                };
                c.bus.write32(me + 0x318, head);
            },
        );
    }
}
