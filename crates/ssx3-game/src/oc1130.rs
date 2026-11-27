//! Staging module oc1130 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1130`.

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
        0x0019_f138,
        fe_rider_preview_update,
        "FEModule_RiderPreviewUpdate",
    ),
    (0x003c_4450, snd_sif_dma_start_next, "SND_SifDmaStartNext"),
    (0x003c_4668, snd_sif_dma_queue, "SND_SifDmaQueue"),
];

/// `void FEModule_RiderPreviewUpdate(RiderPreview* self)` (0x19f138): per-frame
/// update of a front-end rider preview. While a character load is pending
/// (`+0xcd4 >= 0`) it waits until the vtable count passes the pending index and
/// then restarts the load (`FEModule_RiderPreviewLoadChar`); otherwise, while
/// `state < 10`, it steps through the load stages (models polled `+0xcbc`,
/// textures polled `+0xcb8`, model set up `+0xcb4`, bolt materials applied
/// `+0xcc0`). Once the preview is ready (`+0xcc8`) it ticks the animation
/// (`+0xc`), applies the transform and updates the two attached effects
/// (`+0xc70`, `+0xc74`). Converted from the generated labelled-block body
/// with every register, cycle and call kept; any other `pc` (a resume) runs
/// the generated body.
pub fn fe_rider_preview_update(c: &mut Cpu) {
    if c.pc != 0x0019f138 {
        return ee::s03::f_0019f138__generated(c);
    }
    guest(c, |c| {
        'b_19f2c4: {
            'b_19f27c: {
                'b_19f278: {
                    'b_19f198: {
                        c.cycles += 7;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[S0][0] = c.g[A0][0];
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcd4));
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if t {
                            break 'b_19f198;
                        } else {
                            c.cycles += 9;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0019f168);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0019f168)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcd4));
                            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_19f278;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc24));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc28));
                                c.g[RA][0] = sx(0x0019f190);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xcd4), c.g[V0][0]);
                                Callee {
                                    addr: 0x0019e960,
                                    f: ee::s03::FEModule_RiderPreviewLoadChar,
                                }
                                .call(c, 0x0019f190)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                if t {
                                    break 'b_19f27c;
                                } else {
                                    break 'b_19f198;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_19f2c4;
                    } else {
                        'b_19f1dc: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
                                break 'b_19f1dc;
                            } else {
                                'b_19f1cc: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019f1bc);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a0100,
                                        f: ee::s03::FEModule_RiderPreviewPollModels,
                                    }
                                    .call(c, 0x0019f1bc)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19f1cc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), c.g[V0][0]);
                                        if t {
                                            break 'b_19f278;
                                        } else {
                                            break 'b_19f1cc;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                    break 'b_19f27c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
                                    break 'b_19f1dc;
                                }
                            }
                        }
                        'b_19f214: {
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                                break 'b_19f214;
                            } else {
                                'b_19f210: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019f1ec);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x0019fd58,
                                        f: ee::s03::FEModule_RiderPreviewPollTextures,
                                    }
                                    .call(c, 0x0019f1ec)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19f210;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb4));
                                        let t = c.g[V1][0] != 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb8), c.g[V0][0]);
                                        if t {
                                            break 'b_19f278;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x0019f208);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x0019ed80,
                                                f: ee::s03::FEModule_RiderPreviewSetupModel,
                                            }
                                            .call(c, 0x0019f208)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                            if t {
                                                break 'b_19f27c;
                                            } else {
                                                break 'b_19f210;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                                break 'b_19f214;
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                            break 'b_19f27c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                break 'b_19f27c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb4));
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                    break 'b_19f27c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc0));
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                        break 'b_19f27c;
                                    } else {
                                        'b_19f274: {
                                            'b_19f26c: {
                                                c.cycles += 8;
                                                c.g[RA][0] = sx(0x0019f248);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee { addr: 0x0019f780, f: ee::s03::FEModule_RiderPreviewApplyBoltMaterials }.call(c, 0x0019f248)?;
                                                c.g[V0][0] = 0x1;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
                                                c.g[A1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                                c.g[A0][0] = 0u64;
                                                let t = c.g[V1][0] != 0u64;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xcc0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_19f26c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xcc8)
                                                        );
                                                    if c.g[V0][0] == 0u64 {
                                                        c.sw((c.g[A1][0] as u32), c.g[A0][0]);
                                                        break 'b_19f274;
                                                    } else {
                                                        break 'b_19f26c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = 0x1;
                                            c.sw((c.g[A1][0] as u32), c.g[A0][0]);
                                            break 'b_19f274;
                                        }
                                        c.cycles += 1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc4), 0u64);
                                        break 'b_19f278;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                break 'b_19f27c;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_19f2c4;
            } else {
                'b_19f2a8: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        break 'b_19f2a8;
                    } else {
                        c.cycles += 6;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 12);
                        c.g[RA][0] = sx(0x0019f2a0);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x003123c0, 0x0019f2a0)?;
                        c.g[RA][0] = sx(0x0019f2a8);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x00312490, 0x0019f2a8)?;
                        break 'b_19f2a8;
                    }
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x0019f2b0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0019f2d0,
                    f: ee::s03::FEModule_RiderPreviewApplyTransform,
                }
                .call(c, 0x0019f2b0)?;
                c.g[RA][0] = sx(0x0019f2b8);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
                call_ptr(c, 0x002f1a08, 0x0019f2b8)?;
                c.g[RA][0] = sx(0x0019f2c0);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc74));
                call_ptr(c, 0x002ecf78, 0x0019f2c0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_19f2c4;
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

/// Offsets of the SIF DMA queue block (`0x50ad00`, `0x510000 - 0x5300`) that
/// `SND_SifDmaStartNext` reads (see `SIFDMA_*` in `game::lib::snd`).
const OC1130_QUEUED: u32 = 0xcc0;
/// `u8`: index of the chosen queue entry.
const OC1130_CHOSEN: u32 = 0xcc1;
/// `u8`: set while a transfer is in flight.
const OC1130_BUSY: u32 = 0xcc2;
/// `u32`: audio tick at which the transfer started.
const OC1130_STARTED: u32 = 0xcc4;
/// 28 entries of 16 bytes: `{ u32 key, u32 src, u32 _, u16 size, u8 priority }`.
const OC1130_ENTRIES: u32 = 0xcc8;
/// `u32`: IOP destination address of the transfers.
const OC1130_IOP_DEST: u32 = 0xe8c;

/// `void SND_SifDmaStartNext(int nonblocking)` (0x3c4450): start the next
/// queued SIF DMA. Scans the 28 queue entries for the used one (key != 0)
/// with the highest priority byte (+0xe; ties go to the smaller key), stops
/// once `+0xcc0` used entries were seen, records its index (+0xcc1) and the
/// audio tick (+0xcc4), copies it to the IOP (`SND_SifSetDma(src, dest, size,
/// nonblocking)`), marks the queue busy (+0xcc2) and sends the 0x30-byte
/// command packet (`sceSifSendCmd`, or the interrupt-safe `isceSifSendCmd`
/// when `nonblocking != 0`).
pub fn snd_sif_dma_start_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x510000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4638));
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[T2][0] = sx(r32(c, V0).wrapping_sub(0xcc8));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V0][0];
        c.g[T1][0] = 0;
        c.g[A1][0] |= 0xffff;
        c.g[T0][0] = 0;
        c.g[A3][0] = 0;
        c.g[S3][0] = 0x510000;
        'scan: loop {
            'done: {
                'step: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(r32(c, A2));
                    if c.g[A0][0] == 0 {
                        // free entry
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                        break 'step;
                    }
                    'chosen: {
                        'compare: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lbu(r32(c, A2).wrapping_add(0xe));
                            c.g[V0][0] = (c.g[T0][0] < c.g[V1][0]) as u64;
                            let not_higher = c.g[V0][0] == 0;
                            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                            if not_higher {
                                break 'compare;
                            }
                            // higher priority than the best so far
                            c.cycles += 4;
                            let at = r32(c, T2).wrapping_add(OC1130_CHOSEN);
                            c.sb(at, c.g[A3][0]);
                            c.g[A1][0] = c.lw(r32(c, A2));
                            c.g[T0][0] = c.lbu(r32(c, A2).wrapping_add(0xe));
                            break 'chosen;
                        }
                        c.cycles += 2;
                        let other_priority = c.g[V1][0] != c.g[T0][0];
                        c.g[V0][0] = (c.g[A0][0] < c.g[A1][0]) as u64;
                        if other_priority {
                            break 'chosen;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break 'chosen;
                        }
                        // same priority, smaller key
                        c.cycles += 2;
                        let at = r32(c, T2).wrapping_add(OC1130_CHOSEN);
                        c.sb(at, c.g[A3][0]);
                        c.g[A1][0] = c.lw(r32(c, A2));
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lbu(r32(c, T2).wrapping_add(OC1130_QUEUED));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let all_seen = c.g[V0][0] == 0;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    if all_seen {
                        break 'done;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 28) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_4498);
                    }
                    continue 'scan;
                }
            }
            break 'scan;
        }
        c.cycles += 34;
        c.g[S1][0] = 0x510000;
        c.g[V0][0] = 0x510000;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x5300));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5718));
        c.g[S0][0] = c.lbu(r32(c, S1).wrapping_add(OC1130_CHOSEN));
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(OC1130_ENTRIES));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x180));
        c.g[A3][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 24);
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(OC1130_IOP_DEST));
        c.sw(r32(c, S1).wrapping_add(OC1130_STARTED), c.g[V1][0]);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 20) as u32);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(4));
        c.g[A2][0] = c.lhu(r32(c, S0).wrapping_add(0xc));
        Callee {
            addr: 0x003c_43c0,
            f: ee::s13::f_003c43c0,
        }
        .call(c, 0x003c_4540)?;
        c.g[V0][0] = 1;
        c.g[A3][0] = c.lw(r32(c, S1).wrapping_add(OC1130_IOP_DEST));
        c.sb(r32(c, S1).wrapping_add(OC1130_BUSY), c.g[V0][0]);
        c.g[A2][0] = 0x30;
        c.g[T1][0] = sx(r32(c, S3).wrapping_add(0x5780));
        c.g[V1][0] = 1;
        c.g[T0][0] = c.lbu(r32(c, S0).wrapping_add(0xe));
        c.g[V0][0] = c.lw(r32(c, S0));
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(8));
        c.g[A1][0] = c.lhu(r32(c, S0).wrapping_add(0xc));
        c.sb(r32(c, S3).wrapping_add(0x5780), c.g[A2][0]);
        // the command packet: { u8 size; ...; u32 +0x10 = 1, +0x14 = key,
        // +0x18 = IOP dest, +0x1c = third word, +0x20 = size, +0x22 = priority }
        c.sw(r32(c, T1).wrapping_add(0x10), c.g[V1][0]);
        c.sw(r32(c, T1).wrapping_add(0x14), c.g[V0][0]);
        c.sw(r32(c, T1).wrapping_add(0x18), c.g[A3][0]);
        c.sw(r32(c, T1).wrapping_add(0x1c), c.g[A0][0]);
        c.sh(r32(c, T1).wrapping_add(0x20), c.g[A1][0]);
        let blocking = c.g[S2][0] == 0;
        c.sh(r32(c, T1).wrapping_add(0x22), c.g[T0][0]);
        if blocking {
            c.cycles += 8;
            c.g[A1][0] = c.g[T1][0];
            c.g[A0][0] = 0;
            c.g[A2][0] = 0x30;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            call_ptr(c, 0x004261b0, 0x003c_45c8)?;
        } else {
            c.cycles += 9;
            c.g[A1][0] = c.g[T1][0];
            c.g[A0][0] = 0;
            c.g[A2][0] = 0x30;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            call_ptr(c, 0x004261f0, 0x003c_45a4)?;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SifDmaQueue(int key, src, dst, int size, u8 priority)` (0x3c4668):
/// queue one SIF DMA transfer. Rounds `size` up to 64 bytes, disables
/// interrupts, bumps the request counter (`+0xcbc`, wrapping to 1). Key 0x100
/// starts the transfer at once (`SND_SifSetDma`); any other key takes the first
/// free queue entry (`+0xcc8`, 28 of 16 bytes), counts it (`+0xcc0`) and kicks
/// the queue (`SND_SifDmaKick(0)`); returns the request id (0 when the queue is
/// full). Converted from the generated `match pc` body (back-edge events
/// kept); any other `pc` (a resume) runs the generated body.
pub fn snd_sif_dma_queue(c: &mut Cpu) {
    if c.pc != 0x003c4668 {
        return ee::s13::f_003c4668__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003c4668;
        loop {
            match pc {
                0x003c4668 => {
                    c.cycles += 5;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = c.g[A3][0] & 0x3f;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        pc = 0x003c468c;
                        continue;
                    }
                    pc = 0x003c467c;
                    continue;
                }
                0x003c467c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.g[V0][0] | 0xffc0;
                    c.g[A3][0] = c.g[A3][0] & c.g[V0][0];
                    pc = 0x003c468c;
                    continue;
                }
                0x003c468c => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x10000;
                    pc = 0x003c4690;
                    continue;
                }
                0x003c4690 => {
                    c.cycles += 7;
                    c.di();
                    c.mfc0(2, 12);
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x510000;
                    if t {
                        if back_edge_unwinds(c, 0x003c4690) {
                            return Ok(());
                        }
                        pc = 0x003c4690;
                        continue;
                    }
                    pc = 0x003c46ac;
                    continue;
                }
                0x003c46ac => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x100;
                    if t {
                        pc = 0x003c46d4;
                        continue;
                    }
                    pc = 0x003c46c8;
                    continue;
                }
                0x003c46c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), c.g[V0][0]);
                    c.g[V0][0] = 0x100;
                    pc = 0x003c46d4;
                    continue;
                }
                0x003c46d4 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xcc8));
                    if t {
                        pc = 0x003c4740;
                        continue;
                    }
                    pc = 0x003c46dc;
                    continue;
                }
                0x003c46dc => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[A1][0] = c.g[A2][0];
                    c.g[A2][0] = c.g[A3][0];
                    c.g[RA][0] = sx(0x003c46f0);
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x003c43c0,
                        f: ee::s13::SND_SifSetDma,
                    }
                    .call(c, 0x003c46f0)?;
                    c.ei();
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003c4778;
                        continue;
                    }
                    pc = 0x003c4700;
                    continue;
                }
                0x003c4700 => {
                    c.cycles += 16;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xcc8));
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xcc0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[S0][0] as u32).wrapping_add(0xcc0), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                    c.sb((c.g[T1][0] as u32).wrapping_add(0xe), c.g[T0][0]);
                    c.sw((c.g[T1][0] as u32), c.g[V1][0]);
                    c.sw((c.g[T1][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    c.sw((c.g[T1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sh((c.g[T1][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                    c.ei();
                    c.g[RA][0] = sx(0x003c4734);
                    c.g[A0][0] = 0u64;
                    Callee {
                        addr: 0x003c45e8,
                        f: ee::s13::SND_SifDmaKick,
                    }
                    .call(c, 0x003c4734)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003c4778;
                        continue;
                    }
                    pc = 0x003c4740;
                    continue;
                }
                0x003c4740 => {
                    c.cycles += 4;
                    c.g[T2][0] = 0u64;
                    c.g[S0][0] = c.g[V1][0];
                    c.g[T1][0] = c.g[V1][0];
                    pc = 0x003c4750;
                    continue;
                }
                0x003c4750 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] == 0u64;
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003c4700) {
                            return Ok(());
                        }
                        pc = 0x003c4700;
                        continue;
                    }
                    pc = 0x003c475c;
                    continue;
                }
                0x003c475c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[T2][0] as i64) < 28i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x003c4750) {
                            return Ok(());
                        }
                        pc = 0x003c4750;
                        continue;
                    }
                    pc = 0x003c476c;
                    continue;
                }
                0x003c476c => {
                    c.cycles += 3;
                    c.ei();
                    c.g[V0][0] = 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    pc = 0x003c4778;
                    continue;
                }
                0x003c4778 => {
                    c.cycles += 3;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003c4668, 0x003c4784);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the fakes below answer only while this magic is in the
    /// machine's RAM (the suite runs tests in parallel), else they run the
    /// body their slot held.
    const OC1130_CTRL: u32 = DATA + 0x2e_0000;
    const OC1130_MAGIC: u32 = 0x1130_c001;
    /// Forced results, one word per fake (`result + 3`; 0 = the fake's own
    /// pointer result).
    const OC1130_MAIL: u32 = DATA + 0x2f_0000;

    /// Make fake number `idx` return `v` in `v0`.
    fn oc1130_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC1130_MAIL + 4 * idx, v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result unless it unwound.
    fn oc1130_fake_body(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC1130_MAIL + 4 * idx);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if mail != 0 {
            c.g[V0][0] = sx(mail.wrapping_sub(3));
        }
    }

    macro_rules! oc1130_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1130_CTRL) != OC1130_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1130_fake_body(c, $addr, $idx);
                }
            )*
            fn oc1130_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1130_fakes!(
        (
            oc1130_fake_load_char,
            OC1130_PREV_LOAD_CHAR,
            ee::s03::OVERRIDE_0019E960,
            ee::s03::f_0019e960__generated,
            0x0019_e960,
            0
        ),
        (
            oc1130_fake_poll_models,
            OC1130_PREV_POLL_MODELS,
            ee::s03::OVERRIDE_001A0100,
            ee::s03::f_001a0100__generated,
            0x001a_0100,
            1
        ),
        (
            oc1130_fake_poll_textures,
            OC1130_PREV_POLL_TEXTURES,
            ee::s03::OVERRIDE_0019FD58,
            ee::s03::f_0019fd58__generated,
            0x0019_fd58,
            2
        ),
        (
            oc1130_fake_setup_model,
            OC1130_PREV_SETUP_MODEL,
            ee::s03::OVERRIDE_0019ED80,
            ee::s03::f_0019ed80__generated,
            0x0019_ed80,
            3
        ),
        (
            oc1130_fake_bolt_materials,
            OC1130_PREV_BOLT_MATERIALS,
            ee::s03::OVERRIDE_0019F780,
            ee::s03::f_0019f780__generated,
            0x0019_f780,
            4
        ),
        (
            oc1130_fake_apply_transform,
            OC1130_PREV_APPLY_TRANSFORM,
            ee::s03::OVERRIDE_0019F2D0,
            ee::s03::f_0019f2d0__generated,
            0x0019_f2d0,
            5
        ),
        (
            oc1130_fake_anim_update,
            OC1130_PREV_ANIM_UPDATE,
            ee::s10::OVERRIDE_003123C0,
            ee::s10::f_003123c0__generated,
            0x0031_23c0,
            6
        ),
        (
            oc1130_fake_anim_events,
            OC1130_PREV_ANIM_EVENTS,
            ee::s10::OVERRIDE_00312490,
            ee::s10::f_00312490__generated,
            0x0031_2490,
            7
        ),
        (
            oc1130_fake_emitter_update,
            OC1130_PREV_EMITTER_UPDATE,
            ee::s09::OVERRIDE_002F1A08,
            ee::s09::f_002f1a08__generated,
            0x002f_1a08,
            8
        ),
        (
            oc1130_fake_bolt_update,
            OC1130_PREV_BOLT_UPDATE,
            ee::s09::OVERRIDE_002ECF78,
            ee::s09::f_002ecf78__generated,
            0x002e_cf78,
            9
        ),
        (
            oc1130_fake_sif_set_dma,
            OC1130_PREV_SIF_SET_DMA,
            ee::s13::OVERRIDE_003C43C0,
            ee::s13::f_003c43c0__generated,
            0x003c_43c0,
            10
        ),
        (
            oc1130_fake_sif_send_cmd,
            OC1130_PREV_SIF_SEND_CMD,
            ee::s15::OVERRIDE_004261B0,
            ee::s15::f_004261b0__generated,
            0x0042_61b0,
            11
        ),
        (
            oc1130_fake_isif_send_cmd,
            OC1130_PREV_ISIF_SEND_CMD,
            ee::s15::OVERRIDE_004261F0,
            ee::s15::f_004261f0__generated,
            0x0042_61f0,
            12
        ),
    );

    /// `FEModule_RiderPreviewUpdate`: a pending character load (the vtable
    /// call's result above or below `pending`, or no load pending), the
    /// five load stages in every flag combination (models polled, textures
    /// polled, bolt models, materials applied, state >= 10), and the pose
    /// step with and without an animation. Callees are gated fakes; a random
    /// call unwinds.
    #[test]
    fn fe_rider_preview_update_matches() {
        oc1130_install();
        check(
            ee::s03::f_0019f138__generated,
            fe_rider_preview_update,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_f138, 8);
                c.pc = 0x0019_f138;
                c.bus.write32(OC1130_CTRL, OC1130_MAGIC);
                let this = DATA + 0x1000;
                let globals = DATA + 0x2000;
                let vtable = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x854, globals);
                c.bus.write32(globals + 0x10d8, vtable);
                c.bus
                    .write16(vtable + 0x390, (r.next_u32() % 0x40 * 4) as u16);
                c.bus.write32(vtable + 0x394, FAKE);
                let flags = r.next_u32();
                let bit = |n: u32| (flags >> n) & 1;
                let pending = match seed % 3 {
                    0 => u32::MAX,
                    1 => r.next_u32() % 3,
                    _ => 3 + r.next_u32() % 2,
                };
                c.bus.write32(this + 0xcd4, pending);
                // the vtable call is the first fake call
                c.bus.write32(LOG_RESULTS, 3);
                let state = if flags & 7 == 0 {
                    10 + r.next_u32() % 4
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(this, state);
                c.bus.write32(this + 4, r.next_u32());
                c.bus
                    .write32(this + 0xc, if bit(3) == 1 { DATA + 0x4000 } else { 0 });
                c.bus.write32(this + 0xc24, r.next_u32());
                c.bus.write32(this + 0xc28, r.next_u32());
                c.bus.write32(this + 0xc70, r.next_u32());
                c.bus.write32(this + 0xc74, r.next_u32());
                c.bus.write32(this + 0xcb4, bit(4) * (1 + r.next_u32() % 3));
                c.bus.write32(this + 0xcb8, bit(5) * (1 + r.next_u32() % 3));
                c.bus.write32(
                    this + 0xcbc,
                    if bit(6) == 1 || seed % 4 == 3 { 1 } else { 0 },
                );
                c.bus.write32(this + 0xcc0, bit(7) * (1 + r.next_u32() % 3));
                c.bus.write32(this + 0xcc4, bit(8) * (1 + r.next_u32() % 3));
                c.bus.write32(this + 0xcc8, bit(9) * (1 + r.next_u32() % 3));
                oc1130_force(c, 1, bit(10));
                oc1130_force(c, 2, bit(11));
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// The SIF DMA queue block and the audio tick for the sound tests.
    fn oc1130_queue(c: &mut Cpu, r: &mut crate::difftest::Rng, count: u8, dense: bool) {
        let base = 0x0050_ad00;
        c.bus.write8(base + OC1130_QUEUED, count);
        c.bus
            .write8(base + OC1130_CHOSEN, (r.next_u32() % 28) as u8);
        c.bus
            .write8(base + OC1130_BUSY, (r.next_u32() % 3 == 0) as u8);
        for i in 0..28u32 {
            let rec = base + OC1130_ENTRIES + 0x10 * i;
            let used = dense || r.next_u32() % 3 != 0;
            let key = match r.next_u32() % 8 {
                _ if !used => 0,
                0 => 0x8000_0000 | (r.next_u32() % 4) * 0x100,
                n => 0x100 * (1 + n % 4),
            };
            c.bus.write32(rec, key);
            c.bus.write32(rec + 4, DATA + 0x4000 + 0x100 * i);
            c.bus.write32(rec + 8, r.next_u32());
            c.bus.write16(rec + 0xc, (r.next_u32() % 0x1000) as u16);
            c.bus.write8(rec + 0xe, (r.next_u32() % 3) as u8);
        }
        c.bus.write32(base + OC1130_IOP_DEST, r.next_u32());
        c.bus.write32(base + OC1130_STARTED, r.next_u32());
        c.bus.write32(0x0050_a8e8 + 0x180, r.next_u32());
    }

    const OC1130_COUNTS: [u8; 10] = [0xff, 0, 1, 2, 3, 4, 6, 9, 28, 100];

    fn oc1130_start_next_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        oc1130_install();
        let mut r = machine(c, seed, 0x003c_4450, 3);
        c.pc = 0x003c_4450;
        c.bus.write32(OC1130_CTRL, OC1130_MAGIC);
        let count = OC1130_COUNTS[(seed % 10) as usize];
        oc1130_queue(c, &mut r, count, seed / 10 % 2 == 0);
        r
    }

    /// `SND_SifDmaStartNext`: queues with free and used entries (all used
    /// for half of the seeds), signed counts from -1 to 100, equal and
    /// different priorities and keys (also keys with the sign bit set),
    /// blocking and non-blocking; `SND_SifSetDma` and the command sends are
    /// gated fakes, and a random one of them unwinds.
    #[test]
    fn snd_sif_dma_start_next_matches() {
        check(
            ee::s13::f_003c4450__generated,
            snd_sif_dma_start_next,
            400,
            &|c, seed| {
                let mut r = oc1130_start_next_setup(c, seed);
                c.g[A0][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(1 + r.next_u32() % 3)
                };
            },
        );
    }

    /// `SND_SifDmaStartNext` with events delivered at the scan's back edge.
    #[test]
    fn snd_sif_dma_start_next_events() {
        check_events(
            ee::s13::f_003c4450__generated,
            snd_sif_dma_start_next,
            300,
            &|c, seed| {
                let mut r = oc1130_start_next_setup(c, seed);
                let ev = schedule(c, &mut r, seed, 0x003c_4450);
                c.g[A0][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(1 + r.next_u32() % 3)
                };
                ev
            },
        );
    }

    fn oc1130_queue_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        oc1130_install();
        let mut r = machine(c, seed, 0x003c_4668, 3);
        c.pc = 0x003c_4668;
        c.bus.write32(OC1130_CTRL, OC1130_MAGIC);
        c.cop0[12] = r.next_u32();
        // all entries used for every fifth seed: no free slot, return 0
        let count = (r.next_u32() % 4) as u8;
        oc1130_queue(c, &mut r, count, seed % 5 == 4);
        let counter = [u32::MAX, 0, 5, 0x7fff_ffff][(seed / 5 % 4) as usize];
        c.bus.write32(0x0050_ad00 + 0xcbc, counter);
        r
    }

    fn oc1130_queue_args(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        // key 0x100: start the transfer at once; any other key: queue it
        c.g[A0][0] = if seed % 3 == 0 {
            0x100
        } else {
            sx(r.next_u32())
        };
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(r.next_u32());
        // sizes with and without the 64-byte alignment step
        c.g[A3][0] = sx(if seed % 2 == 0 {
            r.next_u32() & !0x3f
        } else {
            r.next_u32()
        });
        c.g[T0][0] = sx(r.next_u32());
    }

    /// `SND_SifDmaQueue(key, src, dst, size, priority)`: the immediate transfer
    /// for key 0x100, else the first free queue entry gets the request and
    /// `SND_SifDmaKick(0)` runs (the real kick and start-next; its callees are
    /// gated fakes); counter wrap to 1, a full queue, aligned and unaligned
    /// sizes.
    #[test]
    fn snd_sif_dma_queue_matches() {
        check(
            ee::s13::f_003c4668__generated,
            snd_sif_dma_queue,
            500,
            &|c, seed| {
                let mut r = oc1130_queue_setup(c, seed);
                oc1130_queue_args(c, &mut r, seed);
            },
        );
    }

    /// `SND_SifDmaQueue` with events delivered at the interrupt-wait loop and
    /// the free-slot scan (the unwound call resumes in the generated body).
    #[test]
    fn snd_sif_dma_queue_events() {
        check_events(
            ee::s13::f_003c4668__generated,
            snd_sif_dma_queue,
            400,
            &|c, seed| {
                let mut r = oc1130_queue_setup(c, seed);
                let ev = schedule(c, &mut r, seed, 0x003c_4668);
                oc1130_queue_args(c, &mut r, seed);
                ev
            },
        );
    }
}
