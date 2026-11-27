//! Staging module oc940 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc940`.

#![allow(unused_imports)]

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
        0x001c_fdd0,
        reward_gallery_find_texture_by_name,
        "cFEStateRewardGallery_FindTextureByName",
    ),
    (
        0x001c_fe60,
        reward_gallery_request_shape_load,
        "cFEStateRewardGallery_RequestShapeLoad",
    ),
    (
        0x001d_00c0,
        reward_gallery_release_textures,
        "cFEStateRewardGallery_ReleaseTextures",
    ),
    (
        0x001d_0180,
        reward_gallery_init_pages,
        "cFEStateRewardGallery_InitPages",
    ),
    (
        0x001d_0320,
        reward_gallery_purchase_item,
        "cFEStateRewardGallery_PurchaseItem",
    ),
    (
        0x001c_e2a8,
        keyboard_popup_build_keys,
        "cKeyboardPopup_BuildKeys",
    ),
    (
        0x001c_e5e0,
        keyboard_popup_append_quick_message,
        "cKeyboardPopup_AppendQuickMessage",
    ),
    (
        0x001c_bdb0,
        keyboard_popup_move_up_from_digit_row,
        "cKeyboardPopup_MoveUpFromDigitRow",
    ),
];

/// `cKeyboardPopup_IsKeyEnabled(self, key, &out)` (0x1ce560).
const KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e560,
    f: ee::s04::f_001ce560,
};

/// `cKeyboardPopup_FindEnabledInColumnFromBottom(self, row)` (0x1cc0e0).
const FIND_ENABLED_IN_COLUMN_FROM_BOTTOM: Callee = Callee {
    addr: 0x001c_c0e0,
    f: ee::s04::f_001cc0e0,
};

/// `cKeyboardPopup_SetKeyEnabled(self, key, enabled, flags)` (0x1ce3c8).
const SET_KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e3c8,
    f: ee::s04::f_001ce3c8,
};

/// `cKeyboardPopup_SetFunctionKeysEnabled(self, enabled)` (0x1ce468).
const SET_FUNCTION_KEYS_ENABLED: Callee = Callee {
    addr: 0x001c_e468,
    f: ee::s04::f_001ce468,
};

/// `cFEStateRewardGallery::FindTextureByName(this, name)` (0x1cfdd0): the
/// texture word of the first slot whose inline name matches, or -1. The
/// slots are 12-byte records from `this+0x7c` (name, 8 bytes; texture word at
/// +8) and the count is at `this+0x78`.
pub fn reward_gallery_find_texture_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let f = Frame::open(c, 0x60);
        f.save(c, 18, 0x30);
        f.save(c, 19, 0x20);
        c.g[S2][0] = 0;
        f.save(c, 20, 0x10);
        c.g[S3][0] = c.g[A0][0];
        f.save(c, 16, 0x50);
        f.save(c, 17, 0x40);
        f.save_ra(c, 0);
        let this = c.g[S3][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
        c.g[S4][0] = c.g[A1][0];
        'done: {
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break 'done;
            }
            c.cycles += 2;
            c.g[S1][0] = sx(this.wrapping_add(0x84));
            c.g[S0][0] = sx(this.wrapping_add(0x7c));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x0041_65a8, 0x001c_fe14)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    break 'done;
                }
                c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = u64::MAX;
                    break 'done;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001c_fe08);
                }
            }
        }
        c.cycles += 8;
        f.restore(c, 16, 0x50);
        f.restore(c, 17, 0x40);
        f.restore(c, 18, 0x30);
        f.restore(c, 19, 0x20);
        f.restore(c, 20, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `cFEStateRewardGallery::RequestShapeLoad(this)` (0x1cfe60): asks the
/// shape loader for the current item's shape, or, when the loader answers
/// with a handle, queues the load and records its state in `this+0x48`
/// (bits 2 and 3 set, bit 0 cleared; the handle at +0x8ec).
pub fn reward_gallery_request_shape_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let f = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        f.save(c, 16, 0x20);
        f.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc8)).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xcc)).lw(c);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_fe94)?;
        let loaded = c.g[V0][0] != 0;
        if !loaded {
            c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
        } else {
            c.cycles += 19;
            c.g[V1][0] = Addr(this.wrapping_add(0x8)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc8)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xcc)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001c_feb0);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_feb0)?;
            c.g[A1][0] = Addr(this.wrapping_add(0x50)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0x9;
            c.g[T0][0] = 0x1;
            c.g[T1][0] = 0;
            call_ptr(c, 0x0019_da10, 0x001c_fecc)?;
            c.g[V1][0] = Addr(this.wrapping_add(0x48)).lw(c);
            c.g[A0][0] = 0xffff_ffff_ffff_fff7;
            let handle = c.g[V0][0];
            Addr(this.wrapping_add(0x8ec)).sw(c, handle);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            c.g[V1][0] = c.g[V1][0] | 0x4;
            let state = c.g[V1][0];
            Addr(this.wrapping_add(0x48)).sw(c, state);
            return finish_request_shape_load(c, f);
        }
        c.cycles += 4;
        c.g[V1][0] = 0xffff_ffff_ffff_fffb;
        c.g[V0][0] = c.g[V0][0] | 0x8;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        let state = c.g[V0][0];
        Addr(this.wrapping_add(0x48)).sw(c, state);
        finish_request_shape_load(c, f)
    })
}

/// The epilogue of `RequestShapeLoad`.
fn finish_request_shape_load(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 5;
    f.restore(c, 16, 0x20);
    f.restore(c, 17, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `cFEStateRewardGallery::ReleaseTextures(this)` (0x1d00c0): releases every
/// texture slot (the loaded word at `+0x84 + 12*i` is cleared to -1 and the
/// slot's name byte is zeroed), then the shape handle at +0x8ec, and drops
/// the bit-3 flag of `this+0x48`.
pub fn reward_gallery_release_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let f = Frame::open(c, 0x50);
        f.save(c, 17, 0x30);
        f.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        f.save(c, 16, 0x40);
        f.save(c, 19, 0x10);
        f.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        'b_1d012c: {
            c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if empty {
                break 'b_1d012c;
            }
            c.cycles += 2;
            c.g[S3][0] = u64::MAX;
            c.g[S0][0] = sx(this.wrapping_add(0x84));
            loop {
                'b_1d0110: {
                    c.cycles += 3;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    let negative = (c.g[A1][0] as i64) < 0;
                    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                    if negative {
                        break 'b_1d0110;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10d8)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x190)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x194)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_0110)?;
                    break 'b_1d0110;
                }
                c.cycles += 7;
                let s0 = c.g[S0][0] as u32;
                let slot = c.g[S3][0];
                Addr(s0).sw(c, slot);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                Addr(s0.wrapping_sub(0x8)).sb(c, 0);
                c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !more {
                    break 'b_1d012c;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_00f0);
                }
            }
        }
        'b_1d0150: {
            c.cycles += 3;
            c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x8ec).lw(c);
            let pending = (c.g[A2][0] as i64) < 0;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if !pending {
                c.cycles += 6;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x50).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
                c.g[RA][0] = sx(0x001d_0148);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                call_ptr(c, 0x0019_dc20, 0x001d_0148)?;
                c.g[V0][0] = u64::MAX;
                Addr(c.g[S1][0] as u32).off(0x8ec).sw(c, u64::MAX);
            }
        }
        c.cycles += 11;
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_fff7;
        f.restore(c, 16, 0x40);
        c.g[V0][0] &= c.g[V1][0];
        f.restore(c, 18, 0x20);
        let flags = c.g[V0][0];
        Addr(this.wrapping_add(0x48)).sw(c, flags);
        f.restore(c, 17, 0x30);
        f.restore(c, 19, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `cFEStateRewardGallery::InitPages(this)` (0x1d0180): (re)creates the
/// page layout; when the layout call answers non-zero, the pages are set up
/// (bit 1 set, bit 0 cleared) and the page count is divided into `+0x60`.
pub fn reward_gallery_init_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(0x001d_0198);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        call_ptr(c, 0x0014_dc80, 0x001d_0198)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_01a4);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_01a4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_01b0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0015_6a38, 0x001d_01b0)?;
        let laid_out = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
        if !laid_out {
            c.cycles += 8;
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x1;
            c.g[RA][0] = sx(0x001d_01d0);
            let state = c.g[V0][0];
            Addr(this.wrapping_add(0x48)).sw(c, state);
            call_ptr(c, 0x0015_6a10, 0x001d_01d0)?;
            f.restore(c, 16, 0x20);
            return init_pages_epilogue(c, f);
        }
        // The layout call answered non-zero: second setup, then the divide.
        c.cycles += 13;
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        c.g[A1][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x2;
        let state = c.g[V0][0];
        Addr(this.wrapping_add(0x48)).sw(c, state);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0xe0).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xe4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001d_01fc);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_01fc)?;
        c.g[V1][0] = Addr(this.wrapping_add(0x5c)).lw(c);
        c.g[V0][0] = Addr(this.wrapping_add(0x68)).lw(c);
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.div(0, V0, V1);
        c.g[V0][0] = c.lo[0];
        Addr(this.wrapping_add(0x60)).sw(c, c.g[V0][0]);
        f.restore(c, 16, 0x20);
        init_pages_epilogue(c, f)
    })
}

/// The epilogue of `InitPages` (the `s0` pair is restored by the caller's
/// path already: only `s1` / the return address remain).
fn init_pages_epilogue(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 4;
    f.restore(c, 17, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `cFEStateRewardGallery::PurchaseItem(this, item)` (0x1d0320): refreshes
/// the reward's labels, prices and icons through the menu helpers, with the
/// item index in `a1`.
pub fn reward_gallery_purchase_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 53;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        f.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[RA][0] = sx(0x001d_033c);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x001d_033c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0348);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_0348)?;
        c.g[A3][0] = Addr(this.wrapping_add(0x4c)).lw(c);
        c.g[T0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(this.wrapping_add(0x50)).lw(c);
        c.g[T1][0] = 0x1;
        c.g[A1][0] = Addr(this.wrapping_add(0x44)).lb(c);
        c.g[RA][0] = sx(0x001d_0364);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_7f20, 0x001d_0364)?;
        c.g[RA][0] = sx(0x001d_036c);
        call_ptr(c, 0x0014_dc80, 0x001d_036c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0378);
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001d_0378)?;
        c.g[A2][0] = Addr(this.wrapping_add(0x50)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0388);
        c.g[A1][0] = Addr(this.wrapping_add(0x44)).lb(c);
        call_ptr(c, 0x0015_0928, 0x001d_0388)?;
        Addr(this.wrapping_add(0x54)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x150)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x154)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03a0);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03a0)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x108)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03b8);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03b8)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x158)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x15c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03cc);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03cc)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x168)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x16c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03e0);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03e0)?;
        f.restore(c, 16, 0x20);
        f.restore(c, 17, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// Where `BuildKeys` continues after one key: the `'*'`/`0x1c` or `0x45/0x4f/0x29`
/// key paths, the default path, or the strlen-checked `0x4b` key.
#[derive(Clone, Copy)]
enum KeyExit {
    /// Sets A1 = key, then A2 = 0, then the call.
    Star,
    /// Sets A2 = 0, then the call.
    Zero,
    /// The call with the A2 the path chose.
    Call,
    /// The default path: the call with the enabled flag 1 (its own cycles).
    Default,
}

/// `cKeyboardPopup::BuildKeys(this)` (0x1ce2a8): walks the 0x54 keys of the
/// key table at `0x441390`, enabling or disabling each through
/// `SetKeyEnabled(this, key, enabled, 2)`; key `0x4b` is enabled only when its
/// text and value are set. Then `SetFunctionKeysEnabled(this, 0)`.
pub fn keyboard_popup_build_keys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let f = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x0044_0000;
        f.save(c, 16, 0x90);
        f.save(c, 17, 0x80);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 18, 0x70);
        c.g[S1][0] = 0;
        f.save(c, 19, 0x60);
        c.g[S2][0] = 0x4b;
        f.save(c, 20, 0x50);
        c.g[S3][0] = 0x29;
        f.save(c, 21, 0x40);
        c.g[S4][0] = 0x4f;
        f.save(c, 22, 0x30);
        c.g[S5][0] = 0x45;
        f.save(c, 23, 0x20);
        c.g[S6][0] = 0x1c;
        f.save(c, 30, 0x10);
        c.g[S7][0] = 0x2a;
        f.save_ra(c, 0);
        c.g[FP][0] = sx(0x0044_0000u32.wrapping_add(0x1390));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
        let this = c.g[S0][0] as u32;
        loop {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
            let star = c.g[V1][0] == c.g[S7][0];
            c.g[A0][0] = c.g[S0][0];
            let exit = if star {
                KeyExit::Star
            } else {
                c.cycles += 2;
                let at_1c = c.g[S1][0] == c.g[S6][0];
                c.g[A1][0] = c.g[S1][0];
                if at_1c {
                    KeyExit::Zero
                } else {
                    c.cycles += 2;
                    let at_45 = c.g[S1][0] == c.g[S5][0];
                    c.g[A2][0] = 0;
                    if at_45 {
                        KeyExit::Call
                    } else {
                        c.cycles += 2;
                        if c.g[S1][0] == c.g[S4][0] {
                            KeyExit::Call
                        } else {
                            c.cycles += 2;
                            if c.g[S1][0] == c.g[S3][0] {
                                KeyExit::Call
                            } else {
                                c.cycles += 2;
                                let not_4b = c.g[S1][0] != c.g[S2][0];
                                c.g[A2][0] = 1;
                                if not_4b {
                                    KeyExit::Default
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001c_e33c);
                                    c.g[A0][0] = sx(this.wrapping_add(0x74));
                                    call_ptr(c, 0x0041_6810, 0x001c_e33c)?;
                                    let no_name = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S0][0];
                                    if no_name {
                                        KeyExit::Star
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = Addr(this.wrapping_add(0x58)).lw(c);
                                        let no_value = c.g[V0][0] == 0;
                                        c.g[A1][0] = 0x4b;
                                        if no_value {
                                            KeyExit::Star
                                        } else {
                                            c.cycles += 2;
                                            c.g[A2][0] = 1;
                                            KeyExit::Call
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match exit {
                KeyExit::Star => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S1][0];
                    c.cycles += 1;
                    c.g[A2][0] = 0;
                    set_key_enabled_call(c)?;
                }
                KeyExit::Zero => {
                    c.cycles += 1;
                    c.g[A2][0] = 0;
                    set_key_enabled_call(c)?;
                }
                KeyExit::Call => set_key_enabled_call(c)?,
                KeyExit::Default => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001c_e378);
                    c.g[A3][0] = 0x2;
                    SET_KEY_ENABLED.call(c, 0x001c_e378)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 84) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e300);
            }
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c_e394);
        c.g[A1][0] = 0;
        SET_FUNCTION_KEYS_ENABLED.call(c, 0x001c_e394)?;
        f.restore(c, 16, 0x90);
        f.restore(c, 17, 0x80);
        f.restore(c, 18, 0x70);
        f.restore(c, 19, 0x60);
        f.restore(c, 20, 0x50);
        f.restore(c, 21, 0x40);
        f.restore(c, 22, 0x30);
        f.restore(c, 23, 0x20);
        f.restore(c, 30, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The shared call of `BuildKeys`' per-key paths: `SetKeyEnabled(this, key, A2, 2)`
/// with its own cycles, then the key advances.
fn set_key_enabled_call(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[RA][0] = sx(0x001c_e368);
    c.g[A3][0] = 0x2;
    SET_KEY_ENABLED.call(c, 0x001c_e368)?;
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
    Ok(())
}

/// `cKeyboardPopup::AppendQuickMessage(this, key)` (0x1ce5e0): fills the
/// quick-message buffer on the stack with the option's text (or the default
/// quick-key message for the key), trims it to the room left after the
/// current text, appends it to `this+0x74`, and records the length in
/// `this+0x134` (clamped to `this+0x60 - 1`).
pub fn keyboard_popup_append_quick_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let f = Frame::open(c, 0x90);
        f.save(c, 18, 0x60);
        f.save(c, 17, 0x70);
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save(c, 16, 0x80);
        f.save(c, 19, 0x50);
        f.save_ra(c, 0x40);
        let this = c.g[S2][0] as u32;
        c.g[RA][0] = sx(0x001c_e608);
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        call_ptr(c, 0x0041_6810, 0x001c_e608)?;
        c.g[V1][0] = Addr(this.wrapping_add(0x60)).lw(c);
        c.g[RA][0] = sx(0x001c_e614);
        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        call_ptr(c, 0x0014_dc80, 0x001c_e614)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001c_e620);
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x001c_e620)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001c_e630);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f6a8, 0x001c_e630)?;
        let buf = sp(c);
        if c.g[V0][0] != 0 {
            c.g[A1][0] = c.g[V0][0];
            // The option text: copy it into the buffer.
            c.cycles += 2;
            c.g[RA][0] = sx(0x001c_e690);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0041_66f4, 0x001c_e690)?;
            c.cycles += 1;
            c.g[V0][0] = sx(buf.wrapping_add(c.g[S3][0] as u32));
        } else {
            // The default quick-key message of the key.
            c.cycles += 14;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
            c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[RA][0] = sx(0x001c_e658);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x0014_f678, 0x001c_e658)?;
            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_e668);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x001c_e668)?;
            c.g[A1][0] = c.g[V0][0];
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                Addr(buf).sb(c, 0);
                c.cycles += 1;
                c.g[V0][0] = sx(buf.wrapping_add(c.g[S3][0] as u32));
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001c_e678);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x003d_cb20, 0x001c_e678)?;
                c.g[V0][0] = sx(buf.wrapping_add(c.g[S3][0] as u32));
            }
        }
        // Trim: the room is the text's cursor, the text is cut at the room.
        c.cycles += 15;
        c.g[S0][0] = sx(this.wrapping_add(0x74));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x001c_e6a8);
        Addr(c.g[V0][0] as u32).sb(c, 0);
        call_ptr(c, 0x0041_62d0, 0x001c_e6a8)?;
        c.g[RA][0] = sx(0x001c_e6b0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x001c_e6b0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(this.wrapping_add(0x60)).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let len_minus_1 = c.g[V0][0];
        Addr(this.wrapping_add(0x134)).sw(c, len_minus_1);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            let len = c.g[A0][0];
            Addr(this.wrapping_add(0x134)).sw(c, len);
        }
        c.cycles += 7;
        f.restore(c, 16, 0x80);
        f.restore(c, 17, 0x70);
        f.restore(c, 18, 0x60);
        f.restore(c, 19, 0x50);
        f.restore_ra(c, 0x40);
        f.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup::MoveUpFromDigitRow(this, dir)` (0x1cbdb0): the key-state
/// machine behind moving the cursor up from the number row. The current
/// state is at `this+0x140`; each branch checks the keys of the states it
/// may move to and stores the new state. The `dir == 0` flag (`S1 < 1`) goes
/// to `this+0x144`.
pub fn keyboard_popup_move_up_from_digit_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let f = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        f.save(c, 16, 0x30);
        f.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0x10);
        c.g[A1][0] = 0;
        let this = c.g[S0][0] as u32;
        KEY_ENABLED.call(c, 0x001c_bdd4)?;
        let enabled = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(this.wrapping_add(0x140)).lw(c);
        if enabled {
            move_up_enabled(c, this)?;
        } else {
            move_up_disabled(c, this)?;
        }
        c.cycles += 5;
        f.restore(c, 16, 0x30);
        f.restore(c, 17, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

/// The `KeyEnabled` answer was non-zero: states 0x10/0x11 go to 4, state 0x12
/// to 5 (both return 1); any other state returns 0.
fn move_up_enabled(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let in_10_11 = c.g[V0][0] != 0;
    c.g[V0][0] = 0x12;
    if !in_10_11 {
        c.cycles += 2;
        let differs = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0;
        if differs {
            return Ok(());
        }
        c.cycles += 1;
        c.g[V1][0] = 5;
    } else {
        c.cycles += 2;
        c.g[V1][0] = 4;
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    let state = c.g[V1][0];
    Addr(this.wrapping_add(0x140)).sw(c, state);
    Ok(())
}

/// The `KeyEnabled` answer was zero: the states 0x0f..=0x16, 0x17, 0x18/0x19
/// and 0x1a each check one key, and the other states return 0.
fn move_up_disabled(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf));
    c.g[V0][0] = (c.g[V0][0] < 8) as u64;
    let in_0f_16 = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if in_0f_16 {
        return move_up_key_0x4b(c, this);
    }
    // 0x17 checks key 0x4c; 0x1a checks 0x53; 0x18/0x19 check 0x4f.
    c.cycles += 3;
    c.g[V0][0] = 0x17;
    let not_17 = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0x1a;
    if !not_17 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4c;
        c.g[RA][0] = sx(0x001c_be84);
        c.g[A2][0] = c.g[SP][0];
        KEY_ENABLED.call(c, 0x001c_be84)?;
        let key_on = c.g[V0][0] != 0;
        c.g[V0][0] = 0x4c;
        if key_on {
            c.cycles += 2;
            Addr(this.wrapping_add(0x140)).sw(c, 0x4c);
            return finish_move_up(c);
        }
        return move_up_key_failed(c, this);
    }
    c.cycles += 2;
    let not_1a = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x18));
    if !not_1a {
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x53;
        c.g[RA][0] = sx(0x001c_beac);
        c.g[A2][0] = c.g[SP][0];
        KEY_ENABLED.call(c, 0x001c_beac)?;
        let key_on = c.g[V0][0] != 0;
        c.g[V0][0] = 0x53;
        if key_on {
            c.cycles += 2;
            Addr(this.wrapping_add(0x140)).sw(c, 0x53);
            return finish_move_up(c);
        }
        return move_up_key_failed(c, this);
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0]) < 2) as u64;
    let in_18_19 = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if !in_18_19 {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 5;
    c.g[A1][0] = 0x4f;
    c.g[RA][0] = sx(0x001c_bed4);
    c.g[A2][0] = c.g[SP][0];
    KEY_ENABLED.call(c, 0x001c_bed4)?;
    let key_on = c.g[V0][0] != 0;
    c.g[V0][0] = 0x4f;
    if key_on {
        c.cycles += 2;
        Addr(this.wrapping_add(0x140)).sw(c, 0x4f);
        return finish_move_up(c);
    }
    move_up_key_failed(c, this)
}

/// States 0x0f..=0x16: key 0x4b, then the state-0x14 check.
fn move_up_key_0x4b(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = 0x4b;
    c.g[RA][0] = sx(0x001c_be28);
    c.g[A2][0] = c.g[SP][0];
    KEY_ENABLED.call(c, 0x001c_be28)?;
    let key_on = c.g[V0][0] != 0;
    c.g[V0][0] = 0x4b;
    if key_on {
        c.cycles += 2;
        Addr(this.wrapping_add(0x140)).sw(c, 0x4b);
        return finish_move_up(c);
    }
    flag_store_move_up(c, this);
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x140)).lw(c);
    c.g[V0][0] = 0x14;
    let not_14 = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    if not_14 {
        return find_minus_one_move_up(c);
    }
    c.cycles += 4;
    c.g[RA][0] = sx(0x001c_be60);
    c.g[A1][0] = 0x3e;
    FIND_ENABLED_IN_COLUMN_FROM_BOTTOM.call(c, 0x001c_be60)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// The key check of 0x17, 0x1a or 0x18/0x19 failed: store the flag, then
/// `FindEnabledInColumnFromBottom(this, -1)`.
fn move_up_key_failed(c: &mut Cpu, this: u32) -> Flow {
    flag_store_move_up(c, this);
    c.cycles += 1;
    c.g[A0][0] = c.g[S0][0];
    find_minus_one_move_up(c)
}

/// `this+0x144 = (a1 < 1)` when the key's out-value is non-zero.
fn flag_store_move_up(c: &mut Cpu, this: u32) {
    c.cycles += 3;
    c.g[V0][0] = Addr(sp(c)).lw(c);
    let no_out = c.g[V0][0] == 0;
    c.g[V0][0] = (c.g[S1][0] < 1) as u64;
    if !no_out {
        c.cycles += 1;
        let flag = c.g[V0][0];
        Addr(this.wrapping_add(0x144)).sw(c, flag);
    }
}

/// `FindEnabledInColumnFromBottom(this, -1)` and the return with 1.
fn find_minus_one_move_up(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[RA][0] = sx(0x001c_bf00);
    c.g[A1][0] = u64::MAX;
    FIND_ENABLED_IN_COLUMN_FROM_BOTTOM.call(c, 0x001c_bf00)?;
    finish_move_up(c)
}

/// The return with 1 after the state store (`+2` cycles).
fn finish_move_up(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, machine_rt, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, DATA, GAME_GP};

    /// The world pointer the reward gallery code reads at `gp - 0x848`.
    const WORLD: u32 = DATA + 0x2000;

    #[test]
    fn reward_gallery_find_texture_by_name_matches() {
        check(
            ee::s04::f_001cfdd0__generated,
            reward_gallery_find_texture_by_name,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1c_fdd0, 0);
                let this = DATA + 0x100;
                let count = seed % 5;
                c.bus.write32(this + 0x78, count);
                for i in 0..count {
                    let entry = this + 0x7c + 12 * i;
                    c.bus.write8(entry, b'a' + i as u8);
                    c.bus.write8(entry + 1, 0);
                    c.bus.write32(entry + 8, 0x1000 + i);
                }
                c.bus.write8(DATA + 0x400, b'a' + (seed % 6) as u8);
                c.bus.write8(DATA + 0x401, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn reward_gallery_request_shape_load_matches() {
        check(
            ee::s04::f_001cfe60__generated,
            reward_gallery_request_shape_load,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1c_fe60, 3);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x7c, DATA + 0x3000);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                c.bus.write32(this + 0x8, obj);
                c.bus.write32(this + 0x48, seed.wrapping_mul(0x9e37));
                c.bus.write16(obj + 0xc8, 0);
                // Seeds pick the leaf (returns 0) or a fake (returns non-zero).
                let target = if seed % 2 == 0 { 0x0014_06c0 } else { FAKE };
                c.bus.write32(obj + 0xcc, target);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn reward_gallery_release_textures_matches() {
        check(
            ee::s04::f_001d00c0__generated,
            reward_gallery_release_textures,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1d_00c0, 3);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x7c, DATA + 0x3000);
                let ctx = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), ctx);
                let vt = DATA + 0x5100;
                c.bus.write32(ctx + 0x10d8, vt);
                c.bus.write16(vt + 0x190, 0);
                c.bus.write32(vt + 0x194, FAKE + 0x20);
                let this = DATA + 0x100;
                let count = seed % 4;
                c.bus.write32(this + 0x78, count);
                for i in 0..count {
                    // Negative words skip the release call, others make it.
                    let word = if (seed + i) % 2 == 0 {
                        u32::MAX
                    } else {
                        0x1000 + i
                    };
                    c.bus.write32(this + 0x84 + 12 * i, word);
                }
                c.bus
                    .write32(this + 0x8ec, if seed % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(this + 0x48, 0xf);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn reward_gallery_init_pages_matches() {
        check(
            ee::s04::f_001d0180__generated,
            reward_gallery_init_pages,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1d_0180, 3);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x48, seed.wrapping_mul(0x51));
                c.bus.write32(this + 0x5c, seed % 7);
                c.bus.write32(this + 0x68, 3 + seed % 5);
                c.bus.write32(this + 0x8, DATA + 0x200);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn reward_gallery_purchase_item_matches() {
        check(
            ee::s04::f_001d0320__generated,
            reward_gallery_purchase_item,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1d_0320, 3);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                c.bus.write32(this + 0x8, obj);
                c.bus.write32(this + 0x44, seed & 0xff);
                c.bus.write32(this + 0x4c, seed.wrapping_mul(7));
                c.bus.write32(this + 0x50, seed.wrapping_mul(11));
                for off in [0x150u32, 0x154, 0x108, 0x10c, 0x158, 0x15c, 0x168, 0x16c] {
                    c.bus.write32(obj + off, FAKE + 0x30);
                }
                c.bus.write16(obj + 0x150, 0);
                c.bus.write16(obj + 0x108, 0);
                c.bus.write16(obj + 0x158, 0);
                c.bus.write16(obj + 0x168, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn keyboard_popup_build_keys_matches() {
        check(
            ee::s04::f_001ce2a8__generated,
            keyboard_popup_build_keys,
            60,
            &|c, seed| {
                elf_image(c);
                let _ = machine_rt(c, seed, 0x1c_e2a8, 0);
                let this = DATA + 0x100;
                // The name text (+0x74) is empty for some seeds, set for others.
                if seed % 3 != 0 {
                    c.bus.write8(this + 0x74, b'x');
                    c.bus.write8(this + 0x75, 0);
                }
                c.bus.write32(this + 0x58, seed % 2);
                c.bus.write32(this + 0x60, 0x40);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn keyboard_popup_append_quick_message_matches() {
        check(
            ee::s04::f_001ce5e0__generated,
            keyboard_popup_append_quick_message,
            40,
            &|c, seed| {
                elf_image(c);
                let _ = machine_rt(c, seed, 0x1c_e5e0, 2);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                let this = DATA + 0x100;
                c.bus.write8(this + 0x74, b'o');
                c.bus.write8(this + 0x75, 0);
                c.bus.write32(this + 0x60, 0x20 + seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x54) as u64;
            },
        );
    }

    #[test]
    fn keyboard_popup_move_up_from_digit_row_matches() {
        check(
            ee::s04::f_001cbdb0__generated,
            keyboard_popup_move_up_from_digit_row,
            120,
            &|c, seed| {
                elf_image(c);
                let _ = machine_rt(c, seed, 0x1c_bdb0, 2);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x140, 0x0c + seed % 16);
                c.bus.write32(this + 0x144, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
