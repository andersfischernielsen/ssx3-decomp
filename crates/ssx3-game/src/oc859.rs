//! Staging module oc859 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc859`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_d738, bolt_file_evict_bytes, "BoltFile_EvictBytes"),
    (
        0x0019_e7f0,
        femodule_rider_preview_request_extra_textures,
        "FEModule_RiderPreviewRequestExtraTextures",
    ),
    (
        0x0019_f418,
        femodule_rider_preview_move_local,
        "FEModule_RiderPreviewMoveLocal",
    ),
    (
        0x0019_f578,
        femodule_rider_preview_move_local2,
        "FEModule_RiderPreviewMoveLocal2",
    ),
    (
        0x0019_f740,
        femodule_rider_preview_get_anim_id,
        "FEModule_RiderPreviewGetAnimId",
    ),
    (
        0x0019_f760,
        femodule_rider_preview_get_anim_progress,
        "FEModule_RiderPreviewGetAnimProgress",
    ),
];

/// Size of one `BoltFileCache` slot record.
#[cfg(test)]
pub(crate) const OC859_SLOT_SIZE: u32 = 0x120;
/// Slot offsets read by `BoltFile_EvictBytes`: data pointer, timer, size and
/// eviction priority (the state word is at +0).
pub(crate) const OC859_SLOT_DATA: u32 = 0x104;
pub(crate) const OC859_SLOT_TIMER: u32 = 0x114;
pub(crate) const OC859_SLOT_SIZE_OFF: u32 = 0x118;
pub(crate) const OC859_SLOT_PRIORITY: u32 = 0x11c;

/// `void BoltFile_EvictBytes(BoltFileCache* self, int skip_table, int bytes)`
/// (0x19d738): free idle slots until `bytes` bytes are released. Pass 1
/// frees every idle slot of each table before the last (table `skip_table` is
/// skipped); pass 2 repeatedly frees the idle slot with the lowest priority
/// (+0x11c) of the table at `skip_table`, until enough is freed.
pub fn bolt_file_evict_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0;
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0);
        'done: {
            'pass1: loop {
                'next_table: {
                    c.cycles += 2;
                    let same = c.g[S3][0] == c.g[S6][0];
                    c.g[V1][0] = 0x10000;
                    if !same {
                        c.cycles += 2;
                        c.g[S1][0] = 0;
                        c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, S5)));
                        'scan: loop {
                            'next_slot: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC859_SLOT_DATA));
                                if c.g[V0][0] == 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                    break 'next_slot;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC859_SLOT_TIMER));
                                if c.g[V0][0] != 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                    break 'next_slot;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(r32(c, S0));
                                if c.g[V0][0] != 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                    break 'next_slot;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC859_SLOT_SIZE_OFF));
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
                                call_ptr(c, 0x0019_d4b8, 0x0019_d7bc)?; // BoltFile_FreeSlot
                                let done = (c.g[S2][0] as i64) <= 0;
                                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                if done {
                                    break 'done;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 256) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x120));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019_d780);
                                }
                                continue 'scan;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = 0x10000;
                            break 'next_table;
                        }
                    }
                }
                c.cycles += 5;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[V1][0] |= 0x2000;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 10) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_d770);
                    }
                    continue 'pass1;
                }
                break 'pass1;
            }
            // Pass 2: the idle slot with the lowest priority, repeatedly.
            c.cycles += 6;
            c.mult(0, S0, S6, V1);
            c.g[S5][0] = 0x120;
            c.g[S3][0] = sx(r32(c, S4).wrapping_add(0x118));
            c.g[S1][0] = c.g[S0][0];
            c.g[A2][0] = u64::MAX;
            'pass2: loop {
                c.cycles += 4;
                c.g[A3][0] = 0;
                c.g[A1][0] = 0;
                c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, S1)));
                'pick: loop {
                    'next_slot: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(OC859_SLOT_DATA));
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            break 'next_slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A0));
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            break 'next_slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(OC859_SLOT_TIMER));
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            break 'next_slot;
                        }
                        'take: {
                            c.cycles += 2;
                            let none_yet = (c.g[A2][0] as i64) < 0;
                            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(OC859_SLOT_PRIORITY));
                            if !none_yet {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                                    break 'next_slot;
                                }
                            }
                            break 'take;
                        }
                        c.cycles += 3;
                        c.g[A3][0] = c.g[V1][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x120));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d810);
                        }
                        continue 'pick;
                    }
                    c.cycles += 2;
                    let none = (c.g[A2][0] as i64) < 0;
                    c.mult(0, V0, A2, S5);
                    if none {
                        break 'done;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
                    call_ptr(c, 0x0019_d4b8, 0x0019_d884)?; // BoltFile_FreeSlot
                    let more = (c.g[S2][0] as i64) > 0;
                    c.g[A2][0] = u64::MAX;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d800);
                        }
                        continue 'pass2;
                    }
                    break 'done;
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_RiderPreviewRequestExtraTextures(FEModule* self)`
/// (0x19e7f0): for the character's bolt entries of type 6, request the
/// texture of each one that has no texture slot yet (`BoltFile_Acquire` +
/// `SetTextureSlot`).
pub fn femodule_rider_preview_request_extra_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x190);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S0, 0x180);
        frame.save(c, S1, 0x170);
        frame.save(c, S2, 0x160);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x140);
        frame.save(c, S5, 0x130);
        frame.save(c, S6, 0x120);
        frame.save(c, S3, 0x150);
        frame.save(c, S7, 0x110);
        frame.save_ra(c, 0x100);
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x7c));
        c.g[S6][0] = sx(r32(c, V1).wrapping_add(0x1a70));
        call_ptr(c, 0x0014_dc80, 0x0019_e82c)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, 0x0014_dd58, 0x0019_e838)?; // BELibrary_GetInterface
        c.g[S5][0] = c.g[V0][0];
        call_ptr(c, 0x0014_bdb8, 0x0019_e840)?; // Bolts_GetDB
        c.g[A2][0] = c.lw(r32(c, S1));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x0014_ad28, 0x0019_e854)?; // BE_GetCharProfileData
        c.g[A1][0] = c.lw(r32(c, S1));
        c.g[A0][0] = c.g[S0][0];
        c.g[S4][0] = c.g[V0][0];
        call_ptr(c, 0x0014_d998, 0x0019_e864)?; // Bolts_GetCharBolts
        c.g[A1][0] = c.lw(r32(c, S1));
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_d988, 0x0019_e874)?; // Bolts_GetCharBoltCount
        let empty = (c.g[V0][0] as i64) <= 0;
        c.sw(r32(c, S1).wrapping_add(0xcb8), 0);
        if !empty {
            c.cycles += 3;
            c.g[S3][0] = c.g[V0][0];
            c.g[S7][0] = 6;
            loop {
                'next_bolt: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(r32(c, S2).wrapping_add(3));
                    if c.g[V0][0] != c.g[S7][0] {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        break 'next_bolt;
                    }
                    'have_tex: {
                        'no_tex: {
                            c.cycles += 7;
                            c.g[V0][0] = c.lh(r32(c, S2).wrapping_add(4));
                            c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(0x288));
                            c.g[V0][0] = sx(r32(c, V0) << 1);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                            c.g[V0][0] = c.lh(r32(c, V0));
                            let neg = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            if neg {
                                break 'no_tex;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x290));
                            c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                            break 'have_tex;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = 0;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1788));
                    c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
                    c.g[A2][0] = c.lh(r32(c, S0));
                    call_ptr(c, 0x0019_cce8, 0x0019_e8d0)?; // GetTextureSlot
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        break 'next_bolt;
                    }
                    'request: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A2][0] = c.lw(r32(c, S1));
                        c.g[A3][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[SP][0];
                        call_ptr(c, 0x0014_b988, 0x0019_e8f0)?; // GetBoltTextureName
                        let none = c.g[V0][0] == 0;
                        c.g[A0][0] = c.g[S6][0];
                        if none {
                            break 'request;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = c.lw(r32(c, S1));
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = 0xb;
                        c.g[T0][0] = 1;
                        c.g[T1][0] = 1;
                        call_ptr(c, 0x0019_da10, 0x0019_e910)?; // BoltFile_Acquire
                        c.g[A2][0] = c.lh(r32(c, S0));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1788));
                        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
                        call_ptr(c, 0x0019_cd38, 0x0019_e924)?; // SetTextureSlot
                    }
                    c.cycles += 1;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                }
                c.cycles += 2;
                let more = c.g[S3][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_e888);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x180);
        frame.restore(c, S1, 0x170);
        frame.restore(c, S2, 0x160);
        frame.restore(c, S3, 0x150);
        frame.restore(c, S4, 0x140);
        frame.restore(c, S5, 0x130);
        frame.restore(c, S6, 0x120);
        frame.restore(c, S7, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// The shared body of the two `MoveLocal` functions: rotate the offset
/// quad `vf5` by the quad at `rot` (a cross-product pair, `w` lanes from the
/// scalar parts), add it to the position quad at `pos` and re-apply the
/// transform. `ra` is the return address of the `ApplyTransform` call.
#[inline(always)]
fn rider_preview_move_local_body(c: &mut Cpu, rot: u32, pos: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 27;
    let frame = Frame::open(c, 0x30);
    c.g[V1][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.lqc2(4, r32(c, V1).wrapping_add(rot));
    c.lqc2(5, r32(c, A1));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    let a = frame.at(c, 0x10);
    c.sqc2(8, a);
    c.lq(2, a);
    let a0 = frame.at(c, 0);
    c.sq(2, a0);
    c.lqc2(3, r32(c, V1).wrapping_add(pos));
    c.lqc2(4, a0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, V1).wrapping_add(pos));
    call_ptr(c, 0x0019_f2d0, ret)?; // FEModule_RiderPreviewApplyTransform
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void FEModule_RiderPreviewMoveLocal(FEModule* self, sQuad* delta)`
/// (0x19f418): move the first preview transform (+0xc30) by `delta` rotated
/// by the rotation quad at +0xc40.
pub fn femodule_rider_preview_move_local(c: &mut Cpu) {
    guest(c, |c| {
        rider_preview_move_local_body(c, 0xc40, 0xc30, 0x0019_f478)
    });
}

/// `void FEModule_RiderPreviewMoveLocal2(FEModule* self, sQuad* delta)`
/// (0x19f578): the same for the second transform (+0xc50, rotation +0xc60).
pub fn femodule_rider_preview_move_local2(c: &mut Cpu) {
    guest(c, |c| {
        rider_preview_move_local_body(c, 0xc60, 0xc50, 0x0019_f5d8)
    });
}

/// `int FEModule_RiderPreviewGetAnimId(FEModule* self)` (0x19f740): layer 2
/// of the rider animation controller (`self+0xc`).
pub fn femodule_rider_preview_get_anim_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 2;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0031_2aa0, 0x0019_f754)?; // AnimController_GetLayerAnimId
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `float FEModule_RiderPreviewGetAnimProgress(FEModule* self)` (0x19f760):
/// layer 2's progress.
pub fn femodule_rider_preview_get_anim_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 2;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0031_2ab0, 0x0019_f774)?; // AnimController_GetLayerProgress
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    fn oc859_clear(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// Gate and mailboxes of the oc859 fakes.
    const OC859_CTRL: u32 = DATA + 0x20_1000;
    const OC859_MAGIC: u32 = 0x0c85_9001;

    /// The fake callee `idx`: logs the call (all registers) and returns the
    /// next scripted result (`list[count]`: 0 = the fake's pointer,
    /// otherwise `value + 1` sign-extended).
    fn oc859_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC859_CTRL + 0x40 + 4 * idx);
        c.bus.write32(OC859_CTRL + 0x40 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC859_CTRL + 0x100 + 0x40 * idx + 4 * (n % 16));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = sx(mail.wrapping_sub(1));
        }
    }

    macro_rules! oc859_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC859_CTRL) != OC859_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc859_fake_call(c, $addr, $idx);
                }
            )*
            fn oc859_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc859_fakes!(
        (
            oc859_fake_14dc80,
            OC859_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            0
        ),
        (
            oc859_fake_14dd58,
            OC859_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            1
        ),
        (
            oc859_fake_14bdb8,
            OC859_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8,
            2
        ),
        (
            oc859_fake_14ad28,
            OC859_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28,
            3
        ),
        (
            oc859_fake_14d998,
            OC859_PREV_14D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998,
            4
        ),
        (
            oc859_fake_14d988,
            OC859_PREV_14D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988,
            5
        ),
        (
            oc859_fake_14b988,
            OC859_PREV_14B988,
            ee::s01::OVERRIDE_0014B988,
            ee::s01::f_0014b988__generated,
            0x0014_b988,
            6
        ),
        (
            oc859_fake_19da10,
            OC859_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10,
            7
        ),
        (
            oc859_fake_19cce8,
            OC859_PREV_19CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8,
            8
        ),
        (
            oc859_fake_19cd38,
            OC859_PREV_19CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38,
            9
        ),
    );

    /// Script the results of fake `idx`.
    fn oc859_script(c: &mut Cpu, idx: u32, results: &[u32]) {
        for (i, v) in results.iter().enumerate() {
            c.bus
                .write32(OC859_CTRL + 0x100 + 0x40 * idx + 4 * i as u32, *v);
        }
    }

    /// `BoltFile_EvictBytes`: tables of idle, busy (state/timer set) and
    /// empty slots with random sizes and priorities; the byte target spans
    /// "satisfied by the first free" to "never satisfied", and the skipped
    /// table is anywhere in 0..=10. `BoltFile_FreeSlot` runs for real with a
    /// fake `EA_MemFree`.
    #[test]
    fn bolt_file_evict_bytes_matches() {
        use crate::game::fe::festatecharequipdetail::tests::{
            bolt_fs_fake_mem_free, BOLT_FS_CTRL, BOLT_FS_MAGIC,
        };
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            bolt_fs_fake_mem_free,
            fakes::prev_cell(bolt_fs_fake_mem_free),
        );
        check(
            ee::s03::f_0019d738__generated,
            bolt_file_evict_bytes,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19d738, 0);
                c.pc = 0x0019_d738;
                c.bus.write32(BOLT_FS_CTRL, BOLT_FS_MAGIC);
                let this = DATA;
                let busy_percent = [0u32, 20, 60][(seed % 3) as usize];
                for table in 0..10u32 {
                    for _ in 0..(seed % 5) * 6 + 4 {
                        let index = if r.next_u32() % 8 == 0 {
                            255
                        } else {
                            r.next_u32() % 256
                        };
                        let slot = this + table * 0x12000 + index * OC859_SLOT_SIZE;
                        c.bus.write32(slot + OC859_SLOT_DATA, DATA + 0xd_0000);
                        c.bus
                            .write32(slot + OC859_SLOT_SIZE_OFF, 1 + r.next_u32() % 300);
                        c.bus.write32(slot + OC859_SLOT_PRIORITY, r.next_u32() % 40);
                        if r.next_u32() % 100 < busy_percent {
                            match r.next_u32() % 3 {
                                0 => c.bus.write32(slot, 1),
                                1 => c.bus.write32(slot + OC859_SLOT_TIMER, 5),
                                _ => c.bus.write32(slot + OC859_SLOT_DATA, 0),
                            }
                        }
                    }
                }
                c.bus.write32(this + 0xb_4000, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 11);
                c.g[A2][0] = sx(match seed % 4 {
                    0 => 1 + r.next_u32() % 100,
                    1 => 1 + r.next_u32() % 1000,
                    2 => 1 + r.next_u32() % 5000,
                    _ => 100_000,
                });
            },
        );
    }

    /// `RequestExtraTextures`: the bolt-info callees are scripted fakes.
    /// Zero, one or several entries of type 6 or other types, texture
    /// indices negative or not, textures already slotted (result >= 0) or
    /// not, and bolt texture names found or not.
    #[test]
    fn femodule_rider_preview_request_extra_textures_matches() {
        oc859_install_fakes();
        check(
            ee::s03::f_0019e7f0__generated,
            femodule_rider_preview_request_extra_textures,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19e7f0, 0);
                c.pc = 0x0019_e7f0;
                c.bus.write32(OC859_CTRL, OC859_MAGIC);
                let world = DATA + 0x500;
                let this = DATA + 0x1000;
                let prof = DATA + 0x2000;
                let table = DATA + 0x3000;
                let bolts = DATA + 0x4000;
                c.bus.write32(world + 0x7c, r.next_u32());
                c.bus.write32(this, r.next_u32());
                c.bus.write32(this + 4, r.next_u32());
                c.bus.write32(prof + 0x288, table);
                for i in 0..8u32 {
                    c.bus.write16(
                        table + 2 * i,
                        [0xffffu16, 0, 1, 2, 3, 0xffff, 1, 2][i as usize],
                    );
                    c.bus
                        .write16(prof + 0x290 + 4 * i, 0x100 + r.next_u32() as u16 % 0x400);
                }
                let count = [0u32, 1, 3, 4][(seed % 4) as usize];
                for i in 0..4u32 {
                    let e = bolts + 0x38 * i;
                    c.bus
                        .write8(e + 3, if r.next_u32() % 3 == 0 { 2 } else { 6 });
                    c.bus.write16(e + 4, (r.next_u32() % 8) as u16);
                }
                oc859_script(c, 3, &[prof + 1]);
                oc859_script(c, 4, &[bolts + 1]);
                oc859_script(c, 5, &[count + 1]);
                let mut slot_results = [0u32; 4];
                let mut name_results = [0u32; 4];
                for i in 0..4 {
                    // GetTextureSlot: -1 (+1 = 0 is "pointer", so use -2) or >= 0.
                    slot_results[i] = if r.next_u32() % 2 == 0 {
                        0xffff_ffff
                    } else {
                        6
                    };
                    name_results[i] = if r.next_u32() % 2 == 0 { 1 } else { 0x1234 };
                }
                oc859_script(c, 8, &slot_results);
                oc859_script(c, 6, &name_results);
                let mut rr = Rng(seed as u64 | 1);
                fill(c, &mut rr, crate::difftest::SP - 0x1c0, 0x40);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `MoveLocal`/`MoveLocal2`: random preview quads and delta; the real
    /// `ApplyTransform` runs on a cleared model pointer.
    fn oc859_move_local_fixture(c: &mut Cpu, seed: u32, addr: u32) {
        let mut r = machine(c, seed, addr as u64, 1);
        c.pc = addr;
        let this = DATA + 0x100;
        oc859_clear(c, this, 0x400);
        oc859_clear(c, DATA + 0x2000, 0x10);
        for off in [0xc30u32, 0xc40, 0xc50, 0xc60] {
            for i in 0..4 {
                c.bus.write32(this + off + 4 * i, r.f32(100.0));
            }
        }
        for i in 0..4 {
            c.bus.write32(DATA + 0x2000 + 4 * i, r.f32(100.0));
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(DATA + 0x2000);
    }

    #[test]
    fn femodule_rider_preview_move_local_matches() {
        check(
            ee::s03::f_0019f418__generated,
            femodule_rider_preview_move_local,
            120,
            &|c, seed| oc859_move_local_fixture(c, seed, 0x0019_f418),
        );
    }

    #[test]
    fn femodule_rider_preview_move_local2_matches() {
        check(
            ee::s03::f_0019f578__generated,
            femodule_rider_preview_move_local2,
            120,
            &|c, seed| oc859_move_local_fixture(c, seed, 0x0019_f578),
        );
    }

    /// The anim controller at `self+0xc`: a cleared anim list, so the real
    /// `GetLayerAnimId`/`GetLayerProgress` chains stay short.
    fn oc859_anim_fixture(c: &mut Cpu, seed: u32, addr: u32) {
        let mut r = machine(c, seed, addr as u64, 1);
        c.pc = addr;
        let this = DATA + 0x100;
        let ctl = DATA + 0x1000;
        oc859_clear(c, this, 0x20);
        oc859_clear(c, ctl, 0x100);
        oc859_clear(c, DATA + 0x2000, 0x100);
        c.bus.write32(this + 0xc, ctl);
        for i in 0..8u32 {
            c.bus.write32(ctl + 4 * i, r.next_u32());
        }
        c.bus.write32(ctl + 0x50, DATA + 0x2000);
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn femodule_rider_preview_get_anim_id_matches() {
        check(
            ee::s03::f_0019f740__generated,
            femodule_rider_preview_get_anim_id,
            60,
            &|c, seed| oc859_anim_fixture(c, seed, 0x0019_f740),
        );
    }

    #[test]
    fn femodule_rider_preview_get_anim_progress_matches() {
        check(
            ee::s03::f_0019f760__generated,
            femodule_rider_preview_get_anim_progress,
            60,
            &|c, seed| oc859_anim_fixture(c, seed, 0x0019_f760),
        );
    }
}
