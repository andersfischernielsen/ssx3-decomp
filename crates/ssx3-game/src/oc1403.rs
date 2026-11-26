//! Staging module oc1403 (wave av-1): hand-written AV player init and
//! subtitle parser replacements, each with its differential test against the
//! generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1403`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub(crate) use crate::game::world::wscriptcache::SUBTITLE_RECORD;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_dec8, av_player_init, "AV_Player_Init"),
    (0x003a_f7f0, av_parse_subtitles, "AV_ParseSubtitles"),
];

/// Direct callees, wrapped as generated aliases. The aliases dispatch
/// through their override slots, so both sides of a staging diff run the
/// same (generated or already-installed) body, and after this lane installs
/// the subtitle leaf/parser ports the same call sites route to them with no
/// edit. `wscriptcache`'s `mod callee` was checked first: it holds no SND
/// const, so `SND_IS_INITIALIZED` lives here in the same shape.
pub(crate) mod callee {
    use super::{ee, Callee};

    pub(crate) const SND_IS_INITIALIZED: Callee = Callee {
        addr: 0x003b_5440,
        f: ee::s13::SND_IsInitialized,
    };
    pub(crate) const AV_FIND_TOKEN: Callee = Callee {
        addr: 0x003a_f670,
        f: ee::s13::AV_FindToken,
    };
    pub(crate) const AV_PARSE_INT_FIELD: Callee = Callee {
        addr: 0x003a_f560,
        f: ee::s13::AV_ParseIntField,
    };
    pub(crate) const AV_PARSE_JUSTIFY: Callee = Callee {
        addr: 0x003a_f428,
        f: ee::s13::AV_ParseJustify,
    };
    pub(crate) const AV_SUBTITLE_INIT: Callee = Callee {
        addr: 0x003b_0340,
        f: ee::s13::AV_SubtitleInit,
    };
    pub(crate) const AV_SUBTITLE_SET_X: Callee = Callee {
        addr: 0x003b_03a8,
        f: ee::s13::AV_SubtitleSetX,
    };
    pub(crate) const AV_SUBTITLE_SET_Y: Callee = Callee {
        addr: 0x003b_03b0,
        f: ee::s13::AV_SubtitleSetY,
    };
    pub(crate) const AV_SUBTITLE_SET_WIDTH: Callee = Callee {
        addr: 0x003b_03b8,
        f: ee::s13::AV_SubtitleSetWidth,
    };
    pub(crate) const AV_SUBTITLE_SET_HEIGHT: Callee = Callee {
        addr: 0x003b_03c0,
        f: ee::s13::AV_SubtitleSetHeight,
    };
    pub(crate) const AV_SUBTITLE_SET_STRING_ID: Callee = Callee {
        addr: 0x003b_03c8,
        f: ee::s13::AV_SubtitleSetStringId,
    };
    pub(crate) const AV_SUBTITLE_SET_FRAME: Callee = Callee {
        addr: 0x003b_03d0,
        f: ee::s13::AV_SubtitleSetFrame,
    };
    pub(crate) const AV_SUBTITLE_SET_JUSTIFY_X: Callee = Callee {
        addr: 0x003b_03d8,
        f: ee::s13::AV_SubtitleSetJustifyX,
    };
    pub(crate) const AV_SUBTITLE_SET_JUSTIFY_Y: Callee = Callee {
        addr: 0x003b_03e0,
        f: ee::s13::AV_SubtitleSetJustifyY,
    };
    pub(crate) const AV_SUBTITLE_SET_SHAPE: Callee = Callee {
        addr: 0x003b_03e8,
        f: ee::s13::AV_SubtitleSetShape,
    };
    pub(crate) const AV_SUBTITLE_SET_COLOR0: Callee = Callee {
        addr: 0x003b_03f0,
        f: ee::s13::AV_SubtitleSetColor0,
    };
    pub(crate) const AV_SUBTITLE_SET_COLOR1: Callee = Callee {
        addr: 0x003b_03f8,
        f: ee::s13::AV_SubtitleSetColor1,
    };
    pub(crate) const AV_SUBTITLE_SET_COLOR2: Callee = Callee {
        addr: 0x003b_0400,
        f: ee::s13::AV_SubtitleSetColor2,
    };
    pub(crate) const AV_SUBTITLE_SET_COLOR3: Callee = Callee {
        addr: 0x003b_0408,
        f: ee::s13::AV_SubtitleSetColor3,
    };
}

/// `void AV_Player_Init(AV_Player* self, int a1, int a2, int a3, int a4,
/// int a5, int a6, int a7, int a8)` (0x3adec8): zero the player, record
/// the constructor arguments, and — when the allocator object is up —
/// build the stream pair: two allocator results, two `StreamFile_Create`
/// streams, priorities, channel filters, then (by the `type` in `a7`) an
/// `AsyncFile_Load` pair, a `StreamFile_QueueRange` pair or a
/// `StreamFile_QueueFile` pair. With no audio (`a4 == 0`) only one
/// allocator result is kept. A dead allocator reports busy and returns.
/// The sound flag at +0x1c is `a8`, forced to 1 while SND is down.
pub fn av_player_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[T3][0];
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S5, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = c.g[T2][0];
        frame.save(c, S4, 0x60);
        c.g[FP][0] = c.g[T1][0];
        c.sw(frame.at(c, 0), c.g[A2][0]);
        c.g[S0][0] = c.lw(frame.at(c, 0xb0));
        frame.save_ra(c, 0x10);
        'b_3adf90: {
            c.g[S4][0] = 0x1;
            callee::SND_IS_INITIALIZED.call(c, 0x003a_df20)?;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[S4][0];
            }
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x78;
            call_ptr(c, 0x003e6448, 0x003a_df34)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S1][0]);
            c.g[V1][0] = 0x510000;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[S0][0]);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x20), 0u64);
            c.g[S0][0] = c.g[V1][0];
            c.sw((c.g[S2][0] as u32).wrapping_add(0x28), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x24), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x44), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x64), 0u64);
            c.sw(c.g[S2][0] as u32, 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x70), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x74), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[S6][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S5][0]);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_3adf90;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S4][0];
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = 0u64;
                }
                break 'b_3adf90;
            }
        }
        'b_3ae2c8: {
            'b_3adfa8: {
                c.cycles += 2;
                let t = c.g[A0][0] != 0u64;

                if t {
                    break 'b_3adfa8;
                } else {
                    c.cycles += 4;
                    call_ptr(c, 0x003e62b0, 0x003a_dfa0)?;
                    let t = true;
                    frame.restore(c, S0, 0xa0);
                    if t {
                        break 'b_3ae2c8;
                    } else {
                        break 'b_3adfa8;
                    }
                }
            }
            'b_3ae2c4: {
                'b_3ae280: {
                    'b_3ae270: {
                        'b_3ae178: {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[A0][0] = 0x490000;
                            if t {
                                break 'b_3ae178;
                            } else {
                                'b_3ae0c0: {
                                    'b_3ae0a8: {
                                        c.cycles += 55;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S4][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5218));
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = 0u64;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, t, 0x003a_dfd0)?;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                        c.g[A0][0] = 0x490000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5230));
                                        c.g[A1][0] = c.g[S7][0];
                                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = 0u64;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, t, 0x003a_dff4)?;
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[T0][0] = c.g[S3][0];
                                        c.g[A0][0] = 0x2;
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0x1;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        call_ptr(c, 0x003e06d8, 0x003a_e010)?;
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[T0][0] = c.g[S7][0];
                                        c.g[A0][0] = 0x2;
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0x1;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                        call_ptr(c, 0x003e06d8, 0x003a_e02c)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                        c.g[A1][0] = 0x98;
                                        c.g[A2][0] = 0x34;
                                        call_ptr(c, 0x003e0ad8, 0x003a_e040)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x98;
                                        c.g[A2][0] = 0x34;
                                        call_ptr(c, 0x003e0ad8, 0x003a_e050)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x1;
                                        c.g[A2][0] = 0xff;
                                        c.g[A3][0] = 0x4d;
                                        c.g[T0][0] = 0x1;
                                        call_ptr(c, 0x003e0948, 0x003a_e068)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0u64;
                                        c.g[T0][0] = 0xfffffffffffffffe;
                                        call_ptr(c, 0x003e0948, 0x003a_e080)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        if t {
                                            break 'b_3ae0a8;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A1][0] = 0x1;
                                            c.g[A2][0] = 0xffff;
                                            c.g[A3][0] = 0x4353;
                                            c.g[T0][0] = 0x1;
                                            call_ptr(c, 0x003e0948, 0x003a_e0a0)?;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                            if t {
                                                break 'b_3ae0c0;
                                            } else {
                                                break 'b_3ae0a8;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    c.g[A2][0] = 0xffff;
                                    c.g[A3][0] = 0x4353;
                                    c.g[T0][0] = 0xffffffffffffffff;
                                    call_ptr(c, 0x003e0948, 0x003a_e0bc)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                    break 'b_3ae0c0;
                                }
                                'b_3ae100: {
                                    c.cycles += 9;
                                    c.g[A1][0] = 0x2;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    c.g[T0][0] = 0xfffffffffffffffe;
                                    call_ptr(c, 0x003e0948, 0x003a_e0d4)?;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                    if t {
                                        break 'b_3ae100;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        call_ptr(c, 0x003df690, 0x003a_e0f0)?;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                        c.g[A0][0] = c.g[S6][0];
                                        let t = true;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_3ae270;
                                        } else {
                                            break 'b_3ae100;
                                        }
                                    }
                                }
                                'b_3ae13c: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_3ae13c;
                                    } else {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        c.g[A1][0] = c.g[S5][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x003e0c88, 0x003a_e11c)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = c.g[S6][0];
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x003e0c88, 0x003a_e134)?;
                                        let t = true;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                                        if t {
                                            break 'b_3ae2c4;
                                        } else {
                                            break 'b_3ae13c;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                frame.restore(c, S0, 0xa0);
                                if t {
                                    break 'b_3ae2c8;
                                } else {
                                    c.cycles += 13;
                                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[A3][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    call_ptr(c, 0x003e0d80, 0x003a_e158)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                    c.g[A2][0] = c.g[FP][0];
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A3][0] = 0u64;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                                    call_ptr(c, 0x003e0d80, 0x003a_e170)?;
                                    let t = true;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                    if t {
                                        break 'b_3ae2c4;
                                    } else {
                                        break 'b_3ae178;
                                    }
                                }
                            }
                        }
                        'b_3ae25c: {
                            'b_3ae244: {
                                c.cycles += 44;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5218));
                                c.g[A1][0] = c.g[S3][0];
                                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                let t = c.g[V0][0] as u32;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, t, 0x003a_e198)?;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                                c.g[A3][0] = c.g[V0][0];
                                c.g[T0][0] = c.g[S3][0];
                                c.g[A2][0] = 0x2;
                                c.g[A0][0] = 0x2;
                                c.g[A1][0] = 0x3;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                call_ptr(c, 0x003e06d8, 0x003a_e1b8)?;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.g[V0][0];
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                call_ptr(c, 0x003e0bf8, 0x003a_e1c8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                c.g[A1][0] = 0x98;
                                c.g[A2][0] = 0x34;
                                call_ptr(c, 0x003e0ad8, 0x003a_e1dc)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x98;
                                c.g[A2][0] = 0x34;
                                call_ptr(c, 0x003e0ad8, 0x003a_e1ec)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0xff;
                                c.g[A3][0] = 0x4d;
                                c.g[T0][0] = 0x1;
                                call_ptr(c, 0x003e0948, 0x003a_e204)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x3;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.g[T0][0] = 0xfffffffffffffffe;
                                call_ptr(c, 0x003e0948, 0x003a_e21c)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_3ae244;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x2;
                                    c.g[A2][0] = 0xffff;
                                    c.g[A3][0] = 0x4353;
                                    c.g[T0][0] = 0x2;
                                    call_ptr(c, 0x003e0948, 0x003a_e23c)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_3ae25c;
                                    } else {
                                        break 'b_3ae244;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = 0x2;
                            c.g[A2][0] = 0xffff;
                            c.g[A3][0] = 0x4353;
                            c.g[T0][0] = 0xffffffffffffffff;
                            call_ptr(c, 0x003e0948, 0x003a_e258)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                            break 'b_3ae25c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                        if t {
                            break 'b_3ae280;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            break 'b_3ae270;
                        }
                    }
                    c.cycles += 4;
                    call_ptr(c, 0x003df690, 0x003a_e278)?;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    if t {
                        break 'b_3ae2c4;
                    } else {
                        break 'b_3ae280;
                    }
                }
                'b_3ae2a4: {
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_3ae2a4;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x003e0c88, 0x003a_e29c)?;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                        if t {
                            break 'b_3ae2c4;
                        } else {
                            break 'b_3ae2a4;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                frame.restore(c, S0, 0xa0);
                if t {
                    break 'b_3ae2c8;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                    c.g[A3][0] = 0u64;
                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x003e0d80, 0x003a_e2c0)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                    break 'b_3ae2c4;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
            break 'b_3ae2c8;
        }
        c.cycles += 11;
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
    })
}
/// `int AV_ParseSubtitles(SubtitleList* list, const char* text, int len)`
/// (0x3af7f0): copy the 13 subtitle token spellings to the stack, count
/// the entries, allocate the record array (`0x34` bytes each), then walk
/// the text: split lines, match `FRAME`/`STRINGID`/`X`/`Y`/`WIDTH`/
/// `HEIGHT`/`JUSTIFY_X`/`JUSTIFY_Y`/`SHAPE`/`A`/`R`/`G`/`B` through the
/// jump table, parse each value with `AV_ParseIntField`/`AV_ParseJustify`
/// and store it with the matching `AV_SubtitleSet*` setter. Empty input
/// (or no token on any line) stores count 0 and returns. `pc` is read
/// from the machine so a resume after an unwound event or callee
/// re-enters at its recorded arm; there is no separate resume twin.
pub fn av_parse_subtitles(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003af7f0 => {
                    c.cycles += 64;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x240));
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.g[S0][0] = c.g[SP][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.g[S1][0] = 0u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x200));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.g[S5][0] = c.g[A1][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.g[S4][0] = c.g[A2][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[V1][0]);
                    c.g[FP][0] = 0xffffffffffffffff;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.g[S7][0] = 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x1a0), c.g[RA][0]);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5340));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[SP][0] as u32);
                    c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x27));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x2f));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x28));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x1f));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x27));
                    c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x2f));
                    c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x118), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x11c), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x120), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x124), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x128), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x130), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x134), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x148), c.g[A0][0]);
                    c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    pc = 0x003af8f0;
                    continue;
                }
                0x003af8f0 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x00416810, 0x003a_f900)?;
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = (c.g[S1][0] < 0xdu64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x003a_f8f0) {
                            return Ok(());
                        }
                        pc = 0x003af8f0;
                        continue;
                    }
                    pc = 0x003af910;
                    continue;
                }
                0x003af910 => {
                    c.cycles += 5;
                    c.g[A2][0] = 0x1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x150), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A2][0]);
                    let t = c.g[S4][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003af9b4;
                        continue;
                    }
                    pc = 0x003af924;
                    continue;
                }
                0x003af924 => {
                    c.cycles += 3;
                    c.g[S6][0] = 0xa;
                    c.g[S0][0] = c.g[S5][0];
                    pc = 0x003af930;
                    continue;
                }
                0x003af930 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    c.g[A3][0] = 0x9;
                    let t = c.g[V0][0] == c.g[A3][0];
                    c.g[S2][0] = c.g[S0][0];
                    if t {
                        pc = 0x003af950;
                        continue;
                    }
                    pc = 0x003af940;
                    continue;
                }
                0x003af940 => {
                    c.cycles += 4;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    c.g[T1][0] = 0x1;

                    if c.g[T0][0] != c.g[T1][0] {
                        c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                        pc = 0x003af994;
                        continue;
                    }
                    pc = 0x003af950;
                    continue;
                }
                0x003af950 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[FP][0] != c.g[V0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    if t {
                        pc = 0x003af990;
                        continue;
                    }
                    pc = 0x003af95c;
                    continue;
                }
                0x003af95c => {
                    c.cycles += 13;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S2][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[T0][0] = 0xd;
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x88));
                    callee::AV_FIND_TOKEN.call(c, 0x003a_f984)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    if c.g[V0][0] != 0 {
                        c.g[FP][0] = c.g[S7][0];
                    }
                    pc = 0x003af990;
                    continue;
                }
                0x003af990 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                    pc = 0x003af994;
                    continue;
                }
                0x003af994 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != c.g[S6][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003af9a8;
                        continue;
                    }
                    pc = 0x003af99c;
                    continue;
                }
                0x003af99c => {
                    c.cycles += 3;
                    c.g[V1][0] = 0x1;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
                    pc = 0x003af9a8;
                    continue;
                }
                0x003af9a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003a_f930) {
                            return Ok(());
                        }
                        pc = 0x003af930;
                        continue;
                    }
                    pc = 0x003af9b4;
                    continue;
                }
                0x003af9b4 => {
                    c.cycles += 3;
                    c.g[S6][0] = 0xffffffffffffffff;
                    let t = c.g[FP][0] != c.g[S6][0];
                    c.g[T1][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[FP][0] as u32));
                    if t {
                        pc = 0x003af9d8;
                        continue;
                    }
                    pc = 0x003af9c0;
                    continue;
                }
                0x003af9c0 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                    let t = true;
                    c.sw(c.g[S3][0] as u32, 0u64);
                    if t {
                        pc = 0x003b030c;
                        continue;
                    }
                    pc = 0x003af9cc;
                    continue;
                }
                0x003af9cc => {
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                    if t {
                        pc = 0x003afb9c;
                        continue;
                    }
                    pc = 0x003af9d8;
                    continue;
                }
                0x003af9d8 => {
                    c.cycles += 18;
                    c.g[A1][0] = 0x34;
                    c.g[S7][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = 0x510000;
                    c.mult(0, 5, 23, 5);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5260));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] as u32;
                    c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x2));
                    call_ptr(c, t, 0x003a_fa10)?;
                    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                    let t = c.g[S0][0] == c.g[S6][0];
                    c.g[S1][0] = c.g[S2][0];
                    if t {
                        pc = 0x003afa3c;
                        continue;
                    }
                    pc = 0x003afa20;
                    continue;
                }
                0x003afa20 => {
                    c.cycles += 2;
                    c.g[S6][0] = 0xffffffffffffffff;
                    pc = 0x003afa28;
                    continue;
                }
                0x003afa28 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    callee::AV_SUBTITLE_INIT.call(c, 0x003a_fa34)?;
                    let t = c.g[S0][0] != c.g[S6][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
                    if t {
                        if back_edge_unwinds(c, 0x003a_fa28) {
                            return Ok(());
                        }
                        pc = 0x003afa28;
                        continue;
                    }
                    pc = 0x003afa3c;
                    continue;
                }
                0x003afa3c => {
                    c.cycles += 5;
                    c.sw(c.g[S3][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S7][0]);
                    let t = c.g[S4][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003afa8c;
                        continue;
                    }
                    pc = 0x003afa50;
                    continue;
                }
                0x003afa50 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lb(c.g[S5][0] as u32);
                    c.g[V0][0] = c.g[V0][0] ^ 0xa;
                    c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
                    let t = c.g[FP][0] == c.g[S2][0];
                    c.g[A2][0] = 0x1;
                    if t {
                        pc = 0x003afa90;
                        continue;
                    }
                    pc = 0x003afa64;
                    continue;
                }
                0x003afa64 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x003afa68;
                    continue;
                }
                0x003afa68 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        pc = 0x003afa8c;
                        continue;
                    }
                    pc = 0x003afa74;
                    continue;
                }
                0x003afa74 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lb(c.g[V0][0] as u32);
                    c.g[V1][0] = c.g[V0][0] ^ 0xa;
                    if c.g[V1][0] == 0 {
                        c.g[S2][0] = c.g[A0][0];
                    }

                    if c.g[FP][0] != c.g[S2][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003a_fa68) {
                            return Ok(());
                        }
                        pc = 0x003afa68;
                        continue;
                    }
                    pc = 0x003afa8c;
                    continue;
                }
                0x003afa8c => {
                    c.cycles += 1;
                    c.g[A2][0] = 0x1;
                    pc = 0x003afa90;
                    continue;
                }
                0x003afa90 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A2][0]);
                    if t {
                        pc = 0x003afb9c;
                        continue;
                    }
                    pc = 0x003afa9c;
                    continue;
                }
                0x003afa9c => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    pc = 0x003afaa0;
                    continue;
                }
                0x003afaa0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x9;
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        pc = 0x003afac0;
                        continue;
                    }
                    pc = 0x003afab0;
                    continue;
                }
                0x003afab0 => {
                    c.cycles += 4;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    c.g[V0][0] = 0x1;

                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lb(c.g[S2][0] as u32);
                        pc = 0x003afb80;
                        continue;
                    }
                    pc = 0x003afac0;
                    continue;
                }
                0x003afac0 => {
                    c.cycles += 15;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c), c.g[S2][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[T0][0] = 0xd;
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    callee::AV_FIND_TOKEN.call(c, 0x003a_faec)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = (c.g[V1][0] < 0xdu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afafc;
                    continue;
                }
                0x003afafc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003afb14 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x118), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb1c;
                    continue;
                }
                0x003afb1c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x11c), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb24;
                    continue;
                }
                0x003afb24 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x134), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb2c;
                    continue;
                }
                0x003afb2c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb34;
                    continue;
                }
                0x003afb34 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x120), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb3c;
                    continue;
                }
                0x003afb3c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x124), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb44;
                    continue;
                }
                0x003afb44 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x128), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb4c;
                    continue;
                }
                0x003afb4c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb54;
                    continue;
                }
                0x003afb54 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x130), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb5c;
                    continue;
                }
                0x003afb5c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb64;
                    continue;
                }
                0x003afb64 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb6c;
                    continue;
                }
                0x003afb6c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x144), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb74;
                    continue;
                }
                0x003afb74 => {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x148), c.g[S0][0]);
                    pc = 0x003afb78;
                    continue;
                }
                0x003afb78 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lb(c.g[S2][0] as u32);
                    pc = 0x003afb80;
                    continue;
                }
                0x003afb80 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xa;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x1;
                    if t {
                        if back_edge_unwinds(c, 0x003a_f9cc) {
                            return Ok(());
                        }
                        pc = 0x003af9cc;
                        continue;
                    }
                    pc = 0x003afb8c;
                    continue;
                }
                0x003afb8c => {
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x003a_faa0) {
                            return Ok(());
                        }
                        pc = 0x003afaa0;
                        continue;
                    }
                    pc = 0x003afb9c;
                    continue;
                }
                0x003afb9c => {
                    c.cycles += 4;
                    c.g[S0][0] = 0u64;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x003b030c;
                        continue;
                    }
                    pc = 0x003afbac;
                    continue;
                }
                0x003afbac => {
                    c.cycles += 19;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), 0u64);
                    c.g[S7][0] = 0xffffffffffffffff;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x160), 0u64);
                    c.g[S6][0] = 0x34;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x164), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x168), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x174), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x178), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x17c), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x180), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x184), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x188), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x190), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x194), 0u64);
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    pc = 0x003afbf8;
                    continue;
                }
                0x003afbf8 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x9;
                    c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    if t {
                        pc = 0x003afc14;
                        continue;
                    }
                    pc = 0x003afc08;
                    continue;
                }
                0x003afc08 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[T0][0] != c.g[V0][0];
                    c.g[V0][0] = 0xa;
                    if t {
                        pc = 0x003b01a8;
                        continue;
                    }
                    pc = 0x003afc14;
                    continue;
                }
                0x003afc14 => {
                    c.cycles += 8;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if c.g[T1][0] == 0 {
                        c.g[S1][0] = c.g[V0][0];
                    }
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[V0][0]);
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003afc64;
                        continue;
                    }
                    pc = 0x003afc34;
                    continue;
                }
                0x003afc34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        pc = 0x003afc50;
                        continue;
                    }
                    pc = 0x003afc3c;
                    continue;
                }
                0x003afc3c => {
                    c.cycles += 1;
                    pc = 0x003afc40;
                    continue;
                }
                0x003afc40 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003afc64;
                        continue;
                    }
                    pc = 0x003afc50;
                    continue;
                }
                0x003afc50 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003a_fc40) {
                            return Ok(());
                        }
                        pc = 0x003afc40;
                        continue;
                    }
                    pc = 0x003afc64;
                    continue;
                }
                0x003afc64 => {
                    c.cycles += 3;
                    c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[FP][0] == c.g[S4][0];
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if t {
                        pc = 0x003afc7c;
                        continue;
                    }
                    pc = 0x003afc70;
                    continue;
                }
                0x003afc70 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[V1][0]);
                    c.sb(c.g[V0][0] as u32, 0u64);
                    pc = 0x003afc7c;
                    continue;
                }
                0x003afc7c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x118));
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    if t {
                        pc = 0x003afce4;
                        continue;
                    }
                    pc = 0x003afc88;
                    continue;
                }
                0x003afc88 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x98));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x94));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fcc8)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x9c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_FRAME.call(c, 0x003a_fcdc)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afce4;
                    continue;
                }
                0x003afce4 => {
                    c.cycles += 3;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x11c));
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        pc = 0x003afd4c;
                        continue;
                    }
                    pc = 0x003afcf0;
                    continue;
                }
                0x003afcf0 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa4));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fd30)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa8));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    callee::AV_SUBTITLE_SET_STRING_ID.call(c, 0x003a_fd44)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afd4c;
                    continue;
                }
                0x003afd4c => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[T1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x124));
                    if t {
                        pc = 0x003afdb0;
                        continue;
                    }
                    pc = 0x003afd54;
                    continue;
                }
                0x003afd54 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xac), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb4), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fd94)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::AV_SUBTITLE_SET_WIDTH.call(c, 0x003a_fda8)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afdb0;
                    continue;
                }
                0x003afdb0 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x128));
                    if t {
                        pc = 0x003afe14;
                        continue;
                    }
                    pc = 0x003afdb8;
                    continue;
                }
                0x003afdb8 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xbc));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb8));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fdf8)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_HEIGHT.call(c, 0x003a_fe0c)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afe14;
                    continue;
                }
                0x003afe14 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x12c));
                    if t {
                        pc = 0x003afe78;
                        continue;
                    }
                    pc = 0x003afe1c;
                    continue;
                }
                0x003afe1c => {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    callee::AV_PARSE_JUSTIFY.call(c, 0x003a_fe28)?;
                    let t = c.g[V0][0] == c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x003afe48;
                        continue;
                    }
                    pc = 0x003afe30;
                    continue;
                }
                0x003afe30 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    callee::AV_SUBTITLE_SET_JUSTIFY_X.call(c, 0x003a_fe40)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afe48;
                    continue;
                }
                0x003afe48 => {
                    c.cycles += 12;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V1][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x180));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.mult(0, 3, 2, 22);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    callee::AV_SUBTITLE_SET_JUSTIFY_X.call(c, 0x003a_fe70)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afe78;
                    continue;
                }
                0x003afe78 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x003afedc;
                        continue;
                    }
                    pc = 0x003afe80;
                    continue;
                }
                0x003afe80 => {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    callee::AV_PARSE_JUSTIFY.call(c, 0x003a_fe8c)?;
                    let t = c.g[V0][0] == c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x003afeac;
                        continue;
                    }
                    pc = 0x003afe94;
                    continue;
                }
                0x003afe94 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x17c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_JUSTIFY_Y.call(c, 0x003a_fea4)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afeac;
                    continue;
                }
                0x003afeac => {
                    c.cycles += 12;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V1][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.mult(0, 3, 2, 22);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    callee::AV_SUBTITLE_SET_JUSTIFY_Y.call(c, 0x003a_fed4)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afedc;
                    continue;
                }
                0x003afedc => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
                    if t {
                        pc = 0x003aff40;
                        continue;
                    }
                    pc = 0x003afee4;
                    continue;
                }
                0x003afee4 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc4));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xcc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_ff24)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xcc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_SHAPE.call(c, 0x003a_ff38)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003aff40;
                    continue;
                }
                0x003aff40 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    if t {
                        pc = 0x003affa4;
                        continue;
                    }
                    pc = 0x003aff48;
                    continue;
                }
                0x003aff48 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd4));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_ff88)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    callee::AV_SUBTITLE_SET_X.call(c, 0x003a_ff9c)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003affa4;
                    continue;
                }
                0x003affa4 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[T1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
                    if t {
                        pc = 0x003b0008;
                        continue;
                    }
                    pc = 0x003affac;
                    continue;
                }
                0x003affac => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xdc), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xdc));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_ffec)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::AV_SUBTITLE_SET_Y.call(c, 0x003b_0000)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b0008;
                    continue;
                }
                0x003b0008 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                    if t {
                        pc = 0x003b006c;
                        continue;
                    }
                    pc = 0x003b0010;
                    continue;
                }
                0x003b0010 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xec), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xec));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe8));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x24));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_0050)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR0.call(c, 0x003b_0064)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b006c;
                    continue;
                }
                0x003b006c => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x144));
                    if t {
                        pc = 0x003b00d0;
                        continue;
                    }
                    pc = 0x003b0074;
                    continue;
                }
                0x003b0074 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf8), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf4), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xfc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x25));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_00b4)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xfc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR1.call(c, 0x003b_00c8)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b00d0;
                    continue;
                }
                0x003b00d0 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[T1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x148));
                    if t {
                        pc = 0x003b0134;
                        continue;
                    }
                    pc = 0x003b00d8;
                    continue;
                }
                0x003b00d8 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x104));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x108));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x26));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_0118)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR2.call(c, 0x003b_012c)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b0134;
                    continue;
                }
                0x003b0134 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b013c;
                    continue;
                }
                0x003b013c => {
                    c.cycles += 22;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x110), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10c));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x114));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x27));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x114), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_017c)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x114));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR3.call(c, 0x003b_0190)?;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    pc = 0x003b0194;
                    continue;
                }
                0x003b0194 => {
                    c.cycles += 2;
                    let t = c.g[FP][0] == c.g[S4][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x003b0300;
                        continue;
                    }
                    pc = 0x003b019c;
                    continue;
                }
                0x003b019c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    let t = true;
                    c.sb(c.g[V0][0] as u32, c.g[A3][0]);
                    if t {
                        pc = 0x003b0300;
                        continue;
                    }
                    pc = 0x003b01a8;
                    continue;
                }
                0x003b01a8 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003b0300;
                        continue;
                    }
                    pc = 0x003b01b0;
                    continue;
                }
                0x003b01b0 => {
                    c.cycles += 84;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                    c.mult(0, 6, 18, 22);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[T0][0]);
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[S2][0];
                    }
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                    c.mult(0, 2, 2, 22);
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[T0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[T1][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                    c.g[S0][0] = 0u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x34));
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x17c));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x180));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[A2][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x174), c.g[A3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x17c), c.g[T1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x180), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x178), c.g[T0][0]);
                    c.ldl(4, (c.g[A1][0] as u32).wrapping_sub(0x2d));
                    c.ldr(4, (c.g[A1][0] as u32).wrapping_sub(0x34));
                    c.ldl(6, (c.g[A1][0] as u32).wrapping_sub(0x25));
                    c.ldr(6, (c.g[A1][0] as u32).wrapping_sub(0x2c));
                    c.ldl(7, (c.g[A1][0] as u32).wrapping_sub(0x1d));
                    c.ldr(7, (c.g[A1][0] as u32).wrapping_sub(0x24));
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_sub(0x15));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_sub(0x1c));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[V0][0] as u32);
                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x18));
                    c.ldl(4, (c.g[A1][0] as u32).wrapping_sub(0xd));
                    c.ldr(4, (c.g[A1][0] as u32).wrapping_sub(0x14));
                    c.ldl(6, (c.g[A1][0] as u32).wrapping_sub(0x5));
                    c.ldr(6, (c.g[A1][0] as u32).wrapping_sub(0xc));
                    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x4));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x27));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                    c.g[A3][0] = 0x1;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x184), c.g[T1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x188), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x190), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x194), c.g[A2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A3][0]);
                    pc = 0x003b0300;
                    continue;
                }
                0x003b0300 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x003a_fbf8) {
                            return Ok(());
                        }
                        pc = 0x003afbf8;
                        continue;
                    }
                    pc = 0x003b030c;
                    continue;
                }
                0x003b030c => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x200));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1a0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003af7f0, 0x003b033c);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::{check, check_paths, compare, elf_image, fakes, require_paths, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `AV_Player` field offsets, used by the fixture to prefill and read
    /// back the struct the port initializes.
    const AV_PLAYER_A1: u32 = 0x0c;
    const AV_PLAYER_A4: u32 = 0x10;
    const AV_PLAYER_ALLOC_A: u32 = 0x04;
    const AV_PLAYER_ALLOC_B: u32 = 0x08;
    const AV_PLAYER_HAS_AUDIO: u32 = 0x14;
    const AV_PLAYER_TYPE: u32 = 0x18;
    const AV_PLAYER_SND_FLAG: u32 = 0x1c;
    const AV_PLAYER_ASYNC_A: u32 = 0x24;
    const AV_PLAYER_ASYNC_B: u32 = 0x2c;
    const AV_PLAYER_STREAM_A: u32 = 0x30;
    const AV_PLAYER_STREAM_B: u32 = 0x34;
    const AV_PLAYER_FILE_A: u32 = 0x38;
    const AV_PLAYER_FILE_B: u32 = 0x3c;

    /// Subtitle record size (one `0x34`-byte record per entry).
    const AV_PARSE_RECORD: u32 = 0x34;

    /// Gate of the stream/async callee fakes: a magic word in the machine's
    /// RAM turns a fake on (both sides of a diff get their own machine).
    /// With the gate off the fake falls back to the body the hook found.
    pub(crate) const AVTEST_MAGIC: u32 = 0x1403_1403;
    pub(crate) const AVTEST_CTRL: u32 = DATA + 0x20_0000;

    macro_rules! avtest_gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(AVTEST_CTRL) != AVTEST_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    avtest_gated_fake!(
        avtest_fake_stream_create,
        AVTEST_PREV_STREAM_CREATE,
        0x003e_06d8,
        ee::s14::f_003e06d8
    );
    avtest_gated_fake!(
        avtest_fake_get_channel,
        AVTEST_PREV_GET_CHANNEL,
        0x003e_0bf8,
        ee::s14::f_003e0bf8
    );
    avtest_gated_fake!(
        avtest_fake_set_priority,
        AVTEST_PREV_SET_PRIORITY,
        0x003e_0ad8,
        ee::s14::f_003e0ad8
    );
    avtest_gated_fake!(
        avtest_fake_set_channel_filter,
        AVTEST_PREV_SET_CHANNEL_FILTER,
        0x003e_0948,
        ee::s14::f_003e0948
    );
    avtest_gated_fake!(
        avtest_fake_queue_range,
        AVTEST_PREV_QUEUE_RANGE,
        0x003e_0c88,
        ee::s14::f_003e0c88
    );
    avtest_gated_fake!(
        avtest_fake_queue_file,
        AVTEST_PREV_QUEUE_FILE,
        0x003e_0d80,
        ee::s14::f_003e0d80
    );
    avtest_gated_fake!(
        avtest_fake_async_load,
        AVTEST_PREV_ASYNC_LOAD,
        0x003d_f690,
        ee::s14::f_003df690
    );

    /// Install the stream/async fakes once per process (later calls no-op).
    pub(crate) fn avplayer_hook_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E06D8),
            avtest_fake_stream_create,
            &AVTEST_PREV_STREAM_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0BF8),
            avtest_fake_get_channel,
            &AVTEST_PREV_GET_CHANNEL,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0AD8),
            avtest_fake_set_priority,
            &AVTEST_PREV_SET_PRIORITY,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0948),
            avtest_fake_set_channel_filter,
            &AVTEST_PREV_SET_CHANNEL_FILTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0C88),
            avtest_fake_queue_range,
            &AVTEST_PREV_QUEUE_RANGE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0D80),
            avtest_fake_queue_file,
            &AVTEST_PREV_QUEUE_FILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            avtest_fake_async_load,
            &AVTEST_PREV_ASYNC_LOAD,
        );
    }

    /// The player struct under test: 16-aligned (the memset cycle count
    /// assumes it) at a nonzero test address.
    pub(crate) const AVPLAYER_TEST: u32 = DATA + 0x100;

    /// Fixture for av_player_init: the struct prefilled nonzero, every init
    /// branch steered by an independent seed digit. EA_MemSet and
    /// EA_AssertNotBusy stay real (short bounded paths); the allocator
    /// vtable word points at the miss-hook fake; the stream/async callees
    /// hit the gated fakes above; the RAM comparison checks every stored
    /// field, and the two all-real arms pin exact cycle totals.
    pub(crate) fn avplayer_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1403, 0);
        let snd_on = seed % 2;
        let alloc = seed / 2 % 3;
        let has_audio = seed / 6 % 2;
        let flag = seed / 12 % 3;
        let ty = seed / 36 % 4;
        for i in 0..0x78 / 4 {
            c.bus
                .write32(AVPLAYER_TEST + 4 * i as u32, r.next_u32() | 1);
        }
        for (i, off) in [
            AV_PLAYER_A1,
            AV_PLAYER_A4,
            AV_PLAYER_ALLOC_A,
            AV_PLAYER_ALLOC_B,
            AV_PLAYER_HAS_AUDIO,
            AV_PLAYER_TYPE,
            AV_PLAYER_SND_FLAG,
            AV_PLAYER_ASYNC_A,
            AV_PLAYER_ASYNC_B,
            AV_PLAYER_STREAM_A,
            AV_PLAYER_STREAM_B,
            AV_PLAYER_FILE_A,
            AV_PLAYER_FILE_B,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus
                .write32(AVPLAYER_TEST + off, 0x5A5A_5A00 + i as u32 + 1);
        }
        // SND on/off: `SND_IsInitialized` samples `lb(0x510000 - 0x55a4)`.
        // The sound flag at +0x1c is a8, or forced to 1 while SND is down.
        c.bus.write8(0x0050_aa5c, if snd_on == 0 { 0 } else { 1 });
        // Allocator object absent / present-but-idle / present.
        c.bus
            .write32(0x0050_9430, if alloc == 0 { 0 } else { fakes::FAKE });
        c.bus
            .write32(0x0050_9434, if alloc == 2 { 0x1234_5678 } else { 0 });
        c.g[A0][0] = AVPLAYER_TEST as u64;
        c.g[A1][0] = (r.next_u32() | 1) as u64;
        c.g[A2][0] = (r.next_u32() | 1) as u64;
        c.g[A3][0] = (r.next_u32() | 1) as u64;
        c.g[T0][0] = if has_audio == 0 {
            0
        } else {
            (r.next_u32() | 1) as u64
        };
        c.g[T1][0] = (r.next_u32() | 1) as u64;
        c.g[T2][0] = (r.next_u32() | 1) as u64;
        c.g[T3][0] = ty as u64;
        let a8 = match flag {
            0 => 0,
            1 => 1,
            _ => 0x1A2B_3C4D,
        };
        // The eighth argument lives at the caller's sp (the port opens a
        // 0xb0 frame first, then reads it back at sp+0xb0).
        c.bus.write32(c.g[SP][0] as u32, a8);
        c.bus.write32(AVTEST_CTRL, AVTEST_MAGIC);
    }

    /// AV_Player_Init (0x3adec8): the struct ends up identical on both sides
    /// on every arm (checked word for word through dirty-page RAM
    /// comparison). Arms 161 (no allocator object) and 164 (idle allocator)
    /// stay fully real, so their totals are exact; the stream arms run
    /// through the gated fakes (whose cost varies), so they are
    /// equality-checked but not path-listed.
    #[test]
    fn av_player_init_matches() {
        avplayer_hook_fakes();
        // Equality on every arm (300 seeds over the five steering digits).
        check(
            ee::s13::f_003adec8__generated,
            av_player_init,
            300,
            &avplayer_setup,
        );
        // The all-real early exits have exact totals: 160 (no allocator
        // object) and 163 (idle allocator). The stream arms run through
        // call-logging fakes whose `3 + (h & 7)` cost varies per seed, so
        // their totals scatter and are equality-checked above instead.
        require_paths(
            ee::s13::f_003adec8__generated,
            300,
            &[160, 163],
            &avplayer_setup,
        );
        // Arm-distinguishing RAM asserts on the port, seed by seed.
        for seed in 0..300u32 {
            let mut c = crate::difftest::fresh();
            avplayer_setup(&mut c, seed);
            let snd = seed % 2;
            let alloc = seed / 2 % 3;
            let audio = seed / 6 % 2;
            let flag = seed / 12 % 3;
            let ty = seed / 36 % 4;
            let a8 = match flag {
                0 => 0,
                1 => 1,
                _ => 0x1A2B_3C4D,
            };
            av_player_init(&mut c);
            let p = AVPLAYER_TEST;
            // SND down forces the flag to 1; SND up keeps a8.
            assert_eq!(
                c.bus.read32(p + 0x1c),
                if snd == 0 { 1 } else { a8 },
                "seed {seed} +0x1c"
            );
            // The audio flag (a4) is recorded at +0x10 and tested for the
            // no-audio path below.
            if audio == 0 {
                assert_eq!(c.bus.read32(p + 0x10), 0, "seed {seed} +0x10");
            } else {
                assert_ne!(c.bus.read32(p + 0x10), 0, "seed {seed} +0x10");
            }
            if alloc == 2 {
                // Allocator present: both stream pairs are built.
                assert_ne!(c.bus.read32(p + 0x30), 0, "seed {seed} +0x30");
                assert_ne!(c.bus.read32(p + 0x34), 0, "seed {seed} +0x34");
                // The type digit selects the queue pair; the call log shows
                // which one ran.
                let mut targets = Vec::new();
                let mut addr = crate::difftest::fakes::LOG_START;
                for _ in 0..16 {
                    let t = c.bus.read32(addr);
                    if t == 0 {
                        break;
                    }
                    targets.push(t);
                    addr += 0x220;
                }
                match ty {
                    0 => assert!(
                        targets.contains(&0x003e_0c88),
                        "seed {seed} queue-range {targets:x?}"
                    ),
                    1 => assert!(
                        targets.contains(&0x003d_f690),
                        "seed {seed} async-load {targets:x?}"
                    ),
                    2 => assert!(
                        targets.contains(&0x003e_0d80),
                        "seed {seed} queue-file {targets:x?}"
                    ),
                    _ => assert!(
                        !targets.contains(&0x003e_0c88)
                            && !targets.contains(&0x003d_f690)
                            && !targets.contains(&0x003e_0d80),
                        "seed {seed} no queue {targets:x?}"
                    ),
                }
            } else {
                // No/idle allocator: the early exit builds no streams.
                assert_eq!(c.bus.read32(p + 0x30), 0, "seed {seed} +0x30");
                assert_eq!(c.bus.read32(p + 0x34), 0, "seed {seed} +0x34");
                assert_eq!(c.bus.read32(p + 0x38), 0, "seed {seed} +0x38");
                assert_eq!(c.bus.read32(p + 0x3c), 0, "seed {seed} +0x3c");
            }
        }
    }

    /// The subtitle list struct, text buffer and record array for the
    /// parser test. The record array is the forced result of the one
    /// allocator call (first miss-hook call, index 0).
    pub(crate) const AVPARSE_LIST: u32 = DATA + 0x200;
    pub(crate) const AVPARSE_TEXT: u32 = DATA + 0x1000;
    pub(crate) const AVPARSE_RECORDS: u32 = DATA + 0x6_0000;

    /// Real-shaped subtitle text by shape: empty, one full entry, many
    /// entries, overlong lines, token-free junk. Numbers vary with the seed
    /// (frames/timings); every buffer is NUL-terminated past its length.
    pub(crate) fn avparse_subtitle_text(shape: u32, seed: u32) -> Vec<u8> {
        match shape {
            0 => Vec::new(),
            1 => format!(
                "FRAME\t{}\nSTRINGID\t{}\nX\t{}\nY\t{}\nWIDTH\t{}\nHEIGHT\t{}\nJUSTIFY_X\t0\nJUSTIFY_Y\t0\nSHAPE\t1\nA\t255\nR\t{}\nG\t{}\nB\t{}\n",
                seed % 100000,
                seed % 997,
                100 + seed % 400,
                50 + seed % 300,
                200 + seed % 200,
                60,
                (seed % 256) as u8,
                ((seed / 7) % 256) as u8,
                ((seed / 13) % 256) as u8,
            )
            .into_bytes(),
            2 => {
                let mut v = Vec::new();
                for i in 0..8u32 {
                    v.extend_from_slice(
                        format!("FRAME\t{}\nSTRINGID\t{}\n", seed % 50000 + i, i)
                            .as_bytes(),
                    );
                }
                v
            }
            3 => {
                let mut v = b"FRAME\t".to_vec();
                v.extend(std::iter::repeat(b'9').take(200));
                v.extend_from_slice(b"\nX\t");
                v.extend(std::iter::repeat(b'Z').take(250));
                v.extend_from_slice(b"\nSTRINGID\t7\n");
                v
            }
            _ => b"hello world\n\t \n\x01\x02\tend\nno keywords here\n".to_vec(),
        }
    }

    /// Fixture for av_parse_subtitles: game image for the token table, jump
    /// table and strlen targets; bounded NUL-terminated text; a writable
    /// record array sized for the entry counts above. Every loop in the
    /// parser is bounded by the text length or the entry count, so every
    /// shape terminates on both sides. The RAM comparison checks the parsed
    /// tables (record count, per-record fields, strings), not only the
    /// return value.
    pub(crate) fn avparse_subtitle_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x03af, 0);
        elf_image(c);
        let text = avparse_subtitle_text(seed % 5, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * AV_PARSE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
        c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        c.g[A0][0] = AVPARSE_LIST as u64;
        c.g[A1][0] = AVPARSE_TEXT as u64;
        c.g[A2][0] = text.len() as u64;
        c.pc = 0x003a_f7f0;
    }

    #[test]
    fn av_parse_subtitles_matches() {
        check(
            ee::s13::f_003af7f0__generated,
            av_parse_subtitles,
            300,
            &avparse_subtitle_setup,
        );
    }

    /// Event delivery without unwinds: a Deliver-only schedule on a running
    /// guest thread, so every loop back edge (top-level and nested) delivers
    /// without ever unwinding. Unwind seeds are deliberately excluded here:
    /// an unwind inside a nested `AV_FindToken`/`AV_ParseJustify` call leaves
    /// `drive` re-entering the top function at the nested head, which the
    /// top-level `match pc` cannot resume (the gccrt-fix-1 precedent for
    /// `check_eh_spec`). Live unwinds land only in the shape-0 test below;
    /// every used-form head is also entered directly by the resume test, and
    /// the nested functions carry their own driven Unwind tests in oc1401.
    /// The schedules left behind must match too.
    #[test]
    fn av_parse_subtitles_events_matches() {
        driven!(avparse_gen_driven, ee::s13::f_003af7f0__generated);
        driven!(avparse_hand_driven, av_parse_subtitles);
        check_events(avparse_gen_driven, avparse_hand_driven, 96, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x03af_0000 + seed as u64, 0);
            elf_image(c);
            let text = avparse_subtitle_text(seed % 5, seed);
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(AVPARSE_TEXT + i as u32, *b);
            }
            c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
            for i in 0..64 * AV_PARSE_RECORD / 4 {
                c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
            }
            c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
            c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
            c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
            c.bus.write32(0x0050_9430, fakes::FAKE);
            c.g[A0][0] = AVPARSE_LIST as u64;
            c.g[A1][0] = AVPARSE_TEXT as u64;
            c.g[A2][0] = text.len() as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            // Stock schedule for the running thread, then force Deliver:
            // no VBlank, so events deliver without ever unwinding.
            let _ = schedule(c, &mut r, seed, 0x003a_f7f0);
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            c.next_event = c.cycles + (r.next_u32() % 48) as u64;
            crate::difftest::events::Events::Deliver
        });
    }

    /// The eight loop-event heads of av_parse_subtitles: the back edges the
    /// generated body checks (`c.pc = HEAD; if c.event() { return; }`). Each
    /// entry below is one resume head exercised by direct entry.
    pub(crate) const AVPARSE_RESUME_HEADS: [u32; 8] = [
        0x003a_f8f0, // token-table copy loop
        0x003a_f930, // first token scan
        0x003a_f9cc, // next entry
        0x003a_fa28, // subtitle-record init loop
        0x003a_fa68, // line token scan
        0x003a_faa0, // per-byte token scan
        0x003a_fbf8, // entry line scan
        0x003a_fc40, // end-of-line scan
    ];

    /// Fixture for one direct-entry resume head: the stack frame in place
    /// (token pointers and lengths copied, slots cleared, RA saved), the
    /// list/text/records of the normal fixture, and the registers the arm
    /// reads set to mid-loop values. Terminates on both sides like the
    /// normal fixture (every loop stays bounded by the text or the count).
    pub(crate) fn avparse_resume_setup(c: &mut Cpu, seed: u32, head: u32) {
        let mut r = fakes::machine(c, seed, 0x03af_1000 + head as u64, 0);
        elf_image(c);
        let text = avparse_subtitle_text(1, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * AV_PARSE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, AVPARSE_RECORDS + 0x10);
        c.bus.write32(AVPARSE_LIST + 4, 5);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        // Stack frame as the entry arm leaves it.
        let sp = crate::difftest::SP - 0x240;
        c.g[SP][0] = sp as u64;
        for i in 0..0x240 / 4 {
            c.bus.write32(sp + 4 * i as u32, r.next_u32() | 1);
        }
        for i in 0..13u32 {
            let ptr = c.bus.read32(0x0049_5340 + 4 * i);
            c.bus.write32(sp + 4 * i, ptr);
            let mut len = 0u32;
            while c.bus.read8(ptr + len) != 0 {
                len += 1;
            }
            c.bus.write32(sp + 0x40 + 4 * i, len);
        }
        c.bus.write32(sp + 0x154, sp + 0x40);
        let resume_len = c.bus.read32(0x0049_5340 + 0x30);
        c.bus.write32(sp + 0x30, resume_len);
        for (i, slot) in [
            0x118u32, 0x11c, 0x120, 0x124, 0x128, 0x12c, 0x130, 0x134, 0x138, 0x13c, 0x140, 0x144,
            0x148,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(sp + slot, i as u32);
        }
        for off in [
            0x15cu32, 0x160, 0x164, 0x168, 0x16c, 0x170, 0x174, 0x178, 0x17c, 0x180, 0x184, 0x188,
            0x18c, 0x190, 0x194,
        ] {
            c.bus.write32(sp + off, 0);
        }
        c.bus.write32(sp + 0x14c, 1);
        c.bus.write32(sp + 0x150, 0);
        c.bus.write32(sp + 0x158, 1);
        c.bus.write32(sp + 0x88, 0);
        c.bus.write32(sp + 0x1a0, crate::difftest::RA);
        c.g[RA][0] = crate::difftest::RA as u64;
        // Canonical mid-parse registers; each head below overrides its own.
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        c.g[S2][0] = 0;
        c.g[S3][0] = AVPARSE_LIST as u64;
        c.g[S4][0] = text.len() as u64;
        c.g[S5][0] = AVPARSE_TEXT as u64;
        c.g[S6][0] = 0xa;
        c.g[S7][0] = 0;
        c.g[FP][0] = u64::MAX;
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        match head {
            0x003a_f8f0 => {
                c.g[S0][0] = 0x0049_5340 + 0x18;
                c.g[S1][0] = 6;
                c.g[S2][0] = (sp + 0x40 + 0x18) as u64;
            }
            0x003a_f930 => {
                c.g[S0][0] = (AVPARSE_TEXT + 2) as u64;
                c.g[S1][0] = 2;
            }
            0x003a_f9cc => {
                c.g[S1][0] = 4;
                c.g[A0][0] = 0x003a_fb14;
            }
            0x003a_fa28 => {
                c.g[S1][0] = (AVPARSE_RECORDS + 0x10) as u64;
                c.g[S0][0] = 1;
                c.g[S6][0] = u64::MAX;
            }
            0x003a_fa68 => {
                c.g[S1][0] = 0;
            }
            0x003a_faa0 => {
                c.g[S1][0] = 2;
                c.g[V0][0] = (AVPARSE_TEXT + 2) as u64;
                c.g[S0][0] = 0;
                c.g[FP][0] = 0;
            }
            0x003a_fbf8 => {
                c.g[S0][0] = 0;
                c.g[S1][0] = 2;
                c.g[V0][0] = (AVPARSE_TEXT + 2) as u64;
            }
            _ => {
                // 0x3afc40: end-of-line scan from the line start.
                c.g[S1][0] = 0;
                c.g[A0][0] = 0;
                c.g[V1][0] = AVPARSE_TEXT as u64;
            }
        }
        c.pc = head;
    }

    /// One direct-entry resume per used-form loop head: the same frame and
    /// register state on both sides, compared to equality (registers,
    /// cycles, RAM). Together with the events test above this pins all
    /// eight back edges with and without delivery.
    #[test]
    fn av_parse_subtitles_resume_matches() {
        for head in AVPARSE_RESUME_HEADS {
            let r = compare(ee::s13::f_003af7f0__generated, av_parse_subtitles, &|c| {
                avparse_resume_setup(c, 0xC0DE, head)
            });
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
        }
    }

    /// Token-per-line subtitle entries: line 0 is a FRAME filler (so phase 1
    /// matches and allocates), line 1 holds each of the 13 known tokens, so
    /// the phase-2 forward search records its slot and phase 3 dispatches
    /// its field handler. JUSTIFY_X/Y use real justification names (hit
    /// path); the rest use distinctive numbers checked field by field below.
    pub(crate) const AVPARSE_TOKENS: [&str; 13] = [
        "FRAME",
        "STRINGID",
        "X",
        "Y",
        "WIDTH",
        "HEIGHT",
        "JUSTIFY_X",
        "JUSTIFY_Y",
        "SHAPE",
        "A",
        "R",
        "G",
        "B",
    ];

    pub(crate) fn avparse_token_first_text(which: usize, seed: u32) -> Vec<u8> {
        let num = 100 + (seed % 400);
        // JUSTIFY_X/Y alternate hit and miss names by seed so both the
        // known-name and default paths run.
        let val_of = |tok: &str| match tok {
            "JUSTIFY_X" if seed % 2 == 0 => "left".to_string(),
            "JUSTIFY_Y" if seed % 2 == 1 => "TOP".to_string(),
            "JUSTIFY_X" | "JUSTIFY_Y" => "BOGUS".to_string(),
            _ => format!("{num}"),
        };
        // Line 0 is FRAME (phase-1 match); line 1 is token `which`.
        let mut v = Vec::new();
        v.extend_from_slice(format!("FRAME\t{num}\n").as_bytes());
        v.extend_from_slice(
            format!(
                "{}\t{}\n",
                AVPARSE_TOKENS[which],
                val_of(AVPARSE_TOKENS[which])
            )
            .as_bytes(),
        );
        v
    }

    pub(crate) fn avparse_token_first_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x03af, 0);
        elf_image(c);
        // which = seed % 13 rotates the line-1 token through all 13 kinds.
        let text = avparse_token_first_text(seed as usize % 13, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * SUBTITLE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
        c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        c.g[A0][0] = AVPARSE_LIST as u64;
        c.g[A1][0] = AVPARSE_TEXT as u64;
        c.g[A2][0] = text.len() as u64;
        c.pc = 0x003a_f7f0;
    }

    #[test]
    fn av_parse_subtitles_token_first_matches() {
        check(
            ee::s13::f_003af7f0__generated,
            av_parse_subtitles,
            130,
            &avparse_token_first_setup,
        );
    }

    /// Field-by-field proof that the token-first entries dispatch the right
    /// handler: record 0 carries line 1's parsed value in its token's
    /// field (INIT defaults are 32/350/600/50/-1/-1/2/2/0 with zero colors).
    #[test]
    fn av_parse_subtitles_token_first_fields() {
        // (token index, record word, expected value). JUSTIFY_X hits on
        // even seeds (left=3), JUSTIFY_Y on odd seeds (TOP=1); other seeds
        // take the default path.
        for k in 0..13usize {
            let seed = match k {
                6 => 0u32,
                7 => 1u32,
                _ => 1u32,
            };
            let mut c = crate::difftest::fresh();
            avparse_subtitle_setup(&mut c, seed);
            let text = avparse_token_first_text(k, seed);
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(AVPARSE_TEXT + i as u32, *b);
            }
            c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
            c.g[A2][0] = text.len() as u64;
            av_parse_subtitles(&mut c);
            let rec = AVPARSE_RECORDS + 0x10;
            let mut w = |o: u32| c.bus.read32(rec + o);
            let num = 100 + (seed % 400);
            match k {
                0 => assert_eq!(w(0x14), num, "FRAME field"),
                1 => assert_eq!(w(0x10), num, "STRINGID field"),
                2 => assert_eq!(w(0x00), num, "X field"),
                3 => assert_eq!(w(0x04), num, "Y field"),
                4 => assert_eq!(w(0x08), num, "WIDTH field"),
                5 => assert_eq!(w(0x0c), num, "HEIGHT field"),
                6 => assert_eq!(w(0x18), 3, "JUSTIFY_X left"),
                7 => assert_eq!(w(0x1c), 1, "JUSTIFY_Y TOP"),
                8 => assert_eq!(w(0x20), num, "SHAPE field"),
                9 => assert_eq!(c.bus.read8(rec + 0x24), num as u8, "A color byte"),
                10 => assert_eq!(c.bus.read8(rec + 0x25), num as u8, "R color byte"),
                11 => assert_eq!(c.bus.read8(rec + 0x26), num as u8, "G color byte"),
                _ => assert_eq!(c.bus.read8(rec + 0x27), num as u8, "B color byte"),
            }
        }
    }

    /// Boundary shapes: no trailing LF (S1==S4 at the scan ends), a VT byte
    /// (whitespace class 0x0b), a leading LF (entry-scan loop iterates), and
    /// a tab-first line. Each pins a loop bound or byte class the five main
    /// shapes never hit.
    pub(crate) fn avparse_boundary_text(which: u32, seed: u32) -> Vec<u8> {
        let num = 100 + (seed % 400);
        match which % 4 {
            0 => format!("FRAME\t{num}\nX\t{num}").into_bytes(),
            1 => format!("FRAME\t{num}\nX\x0b{num}\n").into_bytes(),
            2 => format!("\nFRAME\t{num}\nX\t{num}\n").into_bytes(),
            _ => format!("\t{num}\nFRAME\t{num}\n").into_bytes(),
        }
    }

    pub(crate) fn avparse_boundary_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x03af, 0);
        elf_image(c);
        let text = avparse_boundary_text(seed % 4, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * SUBTITLE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
        c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        c.g[A0][0] = AVPARSE_LIST as u64;
        c.g[A1][0] = AVPARSE_TEXT as u64;
        c.g[A2][0] = text.len() as u64;
        c.pc = 0x003a_f7f0;
    }

    #[test]
    fn av_parse_subtitles_boundary_matches() {
        check(
            ee::s13::f_003af7f0__generated,
            av_parse_subtitles,
            120,
            &avparse_boundary_setup,
        );
    }

    /// First unwind on each of the eight loop-event heads: a tuned VBlank
    /// time per head (calibrated: f8f0 ~200, f930 ~1500, fa28 ~2600,
    /// fa68 ~3300 on leading-LF text, faa0 ~3100, f9cc ~3900, fc40 ~4000,
    /// fbf8 ~4500), with the per-head resume count asserted. VBlank is
    /// consumed by the first unwind, so each driven side resumes exactly
    /// once at that head (combined {head: 2}).
    #[test]
    fn av_parse_subtitles_first_unwind_per_head() {
        use crate::difftest::events::{compare_schedule, take_resumes};
        driven!(avparse_gen_first, ee::s13::f_003af7f0__generated);
        driven!(avparse_hand_first, av_parse_subtitles);
        // (head, VBlank offset, leading-LF text?).
        const CASES: [(u32, u64, bool); 8] = [
            (0x003a_f8f0, 200, false),
            (0x003a_f930, 1500, false),
            (0x003a_f9cc, 3900, false),
            (0x003a_fa28, 2600, false),
            (0x003a_fa68, 3300, true),
            (0x003a_faa0, 3100, false),
            (0x003a_fbf8, 4500, false),
            (0x003a_fc40, 4000, false),
        ];
        for (head, toff, leading_lf) in CASES {
            let setup = |c: &mut Cpu| {
                avparse_subtitle_setup(c, 1);
                if leading_lf {
                    c.bus.write8(AVPARSE_TEXT, b'\n');
                    let text = avparse_subtitle_text(1, 1);
                    for (i, b) in text.iter().enumerate() {
                        c.bus.write8(AVPARSE_TEXT + 1 + i as u32, *b);
                    }
                    c.bus.write8(AVPARSE_TEXT + 1 + text.len() as u32, 0);
                    c.g[A2][0] = (text.len() + 1) as u64;
                }
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x003a_f7f0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x003a_f7f0;
                c.cycles = 10000;
                c.next_event = 10000;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 10000 + toff;
            };
            take_resumes();
            let r = crate::difftest::compare_with(
                avparse_gen_first,
                avparse_hand_first,
                &setup,
                |a, b, hard| {
                    compare_schedule(a, b, hard);
                },
            );
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
            let resumes = take_resumes();
            let mut expected = std::collections::BTreeMap::new();
            expected.insert(head, 2);
            assert_eq!(resumes, expected, "head {head:#x}");
        }
    }
}
