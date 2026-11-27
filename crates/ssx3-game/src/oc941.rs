//! Staging module oc941 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc941`.

#![allow(unused_imports)]

use crate::abi::{r32, ret, set_fpr};
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
        0x001d_03f8,
        fe_state_reward_gallery_view_item,
        "cFEStateRewardGallery_ViewItem",
    ),
    (
        0x001d_0d30,
        fe_state_reward_gallery_update_item_info,
        "cFEStateRewardGallery_UpdateItemInfo",
    ),
    (
        0x001d_0f30,
        fe_state_reward_gallery_update_selection,
        "cFEStateRewardGallery_UpdateSelection",
    ),
    (
        0x001d_12c8,
        fe_state_reward_gallery_trading_cards_ctor,
        "cFEStateRewardGalleryTradingCards_ctor",
    ),
    (
        0x001d_1460,
        fe_state_reward_gallery_concept_arts_ctor,
        "cFEStateRewardGalleryConceptArts_ctor",
    ),
    (
        0x001d_15f8,
        fe_state_reward_gallery_videos_ctor,
        "cFEStateRewardGalleryVideos_ctor",
    ),
    (
        0x001d_1790,
        fe_state_reward_gallery_toys_ctor,
        "cFEStateRewardGalleryToys_ctor",
    ),
    (
        0x001d_1928,
        fe_state_reward_gallery_cheat_chars_ctor,
        "cFEStateRewardGalleryCheatChars_ctor",
    ),
];

/// `s0` (the gallery object), read from the register each time: a callee
/// can change it, so no Rust copy is kept across a call.
#[inline]
fn reward_gallery_self(c: &Cpu) -> u32 {
    r32(c, S0)
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_ViewItem (0x1d03f8).

/// `void cFEStateRewardGallery_ViewItem(cFEStateRewardGallery* self)`
/// (0x1d03f8, `cFEStateRewardGalleryBase::showReward`): open the viewer of the
/// selected reward. Types 3, 4, 5 and 7 (posters, cards, art, toys) get a
/// `cFEStatePreviewReward`, type 6 a `cFEStateVideoPlayer` (640 bytes); any
/// other type opens nothing. A new state is pushed as a popup on the engine's
/// state stack.
pub fn fe_state_reward_gallery_view_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);

        // vt[34]: the reward's data, `vt[34].fn(this + delta)`.
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x110)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x114)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0428)?;
        c.g[S2][0] = c.g[V0][0];

        // The reward type decides the state.
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x4c)).lw(c);
        let ty = c.g[V1][0];
        c.g[V0][0] = 6;
        let is_video = ty == c.g[V0][0];
        c.g[V0][0] = ((ty as i64) < 7) as u64;
        if is_video {
            view_item_video(c)?;
        } else if (ty as i64) < 7 {
            c.cycles += 4;
            if (ty as i64) < 3 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                view_item_preview(c)?;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 7;
            if ty == 7 {
                c.cycles += 1;
                view_item_preview(c)?;
            }
        }

        // 'b_1d04f8: push the new state as a popup when there is one.
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001d_04f8)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The video branch of ViewItem: a `cFEStateVideoPlayer` with its movie set
/// from the reward data in `s2` (the movie name is at +4).
fn view_item_video(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A0][0] = 0x280;
    c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x1d8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001d_04bc)?;
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = Addr(reward_gallery_self(c).wrapping_add(0x44)).lb(c);
    c.g[A2][0] = c.g[S0][0];
    call_ptr(c, 0x001d_22f0, 0x001d_04d0)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x001d_2598, 0x001d_04e4)?;
    Ok(())
}

/// The preview branch of ViewItem: a `cFEStatePreviewReward` (120 bytes)
/// showing the reward data in `s2`, with the reward type.
fn view_item_preview(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A0][0] = 0x78;
    c.g[A1][0] = sx(0x0046_6f78);
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001d_0474)?;
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = Addr(reward_gallery_self(c).wrapping_add(0x44)).lb(c);
    c.g[A2][0] = c.g[S0][0];
    call_ptr(c, 0x001d_1aa0, 0x001d_0488)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(0x4c)).lw(c);
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x001d_20d8, 0x001d_049c)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_UpdateItemInfo (0x1d0d30).

/// `void cFEStateRewardGallery_UpdateItemInfo(cFEStateRewardGallery* self)`
/// (0x1d0d30): refresh the two item texts (+0x908 the name, +0x90c the price)
/// for the selected grid cell. Without a grid both texts are blanked.
pub fn fe_state_reward_gallery_update_item_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);

        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        item_info_body(c, frame)?;

        // 'b_1d0e64: the epilogue.
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of UpdateItemInfo up to its epilogue (`a0` = the grid).
fn item_info_body(c: &mut Cpu, frame: Frame) -> Flow {
    if c.g[A0][0] != 0 {
        c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x5c)).lw(c);
        return item_info_grid(c, frame);
    }
    // No grid: blank the name and the price.
    c.cycles += 3;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x908)).lw(c);
    if c.g[A0][0] == 0 {
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
    } else {
        c.cycles += 4;
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
        call_ptr(c, 0x003a_0c50, 0x001d_0d6c)?;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
    }
    c.cycles += 2;
    let no_price = c.g[A0][0] == 0;
    frame.restore(c, S0, 0x30);
    if no_price {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
    call_ptr(c, 0x003a_0c50, 0x001d_0d84)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// The grid case of UpdateItemInfo: the item index, the item and its name,
/// then its price (or the empty text).
fn item_info_grid(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x64)).lw(c);
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.mult(0, 3, 3, 2);
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
    let vt = c.g[A1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x110)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x114)).lw(c);
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let target = c.g[V0][0] as u32;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, target, 0x001d_0db4)?;
    c.g[S1][0] = c.g[V0][0];

    // vt[37]: the name of the item, set on the name text.
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x908)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 10;
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x128)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x12c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0ddc)?;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x908)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_0c50, 0x001d_0de8)?;
    }
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);

    c.cycles += 2;
    if c.g[V0][0] == 0 {
        frame.restore(c, S0, 0x30);
        return Ok(());
    }

    // vt[35]: the price. A positive price the item can be bought for is
    // formatted; anything else (or a shown item) gets the empty text.
    c.cycles += 9;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x118)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x11c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_0e0c)?;
    c.g[S1][0] = c.g[V0][0];
    let not_positive = (c.g[S1][0] as i64) <= 0;
    c.g[A1][0] = 0x004a_0000;
    if !not_positive {
        c.cycles += 8;
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x130)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x134)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0e30)?;
        let shown = c.g[V0][0] != 0;
        c.g[A1][0] = 0x004a_0000;
        if !shown {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0019_8af0, 0x001d_0e40)?;
            c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001d_0e4c)?;
            frame.restore(c, S0, 0x30);
            return Ok(());
        }
    }

    // Blank the price text.
    c.cycles += 4;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
    call_ptr(c, 0x003a_0c50, 0x001d_0e60)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_UpdateSelection (0x1d0f30).

/// `void cFEStateRewardGallery_UpdateSelection(cFEStateRewardGallery* self)`
/// (0x1d0f30): move the selection's icon and the grid's selection state. The
/// icon is placed at the grid cell (offset of the grid's (x, y) plus the
/// icon's own (x, y) at +0x70/+0x74); a selected item sets the grid's
/// (+0x78, +0x7c) selection pair, and the grid's own vtable is told the
/// selection frame.
pub fn fe_state_reward_gallery_update_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);

        c.g[A2][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
        let no_icon = c.g[A2][0] == 0;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        if !no_icon {
            selection_icon(c)?;
        }
        selection_grid(c, frame)?;

        // Epilogue (after 'b_1d1098).
        c.cycles += 4;
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The icon half of UpdateSelection (`a0` = the grid, `a2` = the icon).
fn selection_icon(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        // No grid: the icon's slot 9 with 0.
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
        c.cycles += 6;
        c.g[A1][0] = 0;
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0fe0)?;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        return Ok(());
    }

    // The grid's cell: the grid's centre (a) plus the icon's offset (b).
    c.cycles += 29;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a_0290, 0x001d_0f60)?;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
    let s = sp(c);
    c.g[A1][0] = sx(s.wrapping_add(0x10));
    call_ptr(c, 0x003a_0290, 0x001d_0f6c)?;

    let f2 = Addr(reward_gallery_self(c).wrapping_add(0x74)).read_fpu(c);
    c.g[A1][0] = 1;
    let f3 = Addr(reward_gallery_self(c).wrapping_add(0x70)).read_fpu(c);
    let a_y = Addr(s.wrapping_add(4)).read_fpu(c);
    let a_x = Addr(s).read_fpu(c);
    let b_y = a_y + f2;
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
    let b_x = a_x + f3;
    set_fpr(c, 3, f3);
    set_fpr(c, 2, f2);
    set_fpr(c, 1, b_y);
    set_fpr(c, 0, b_x);
    Addr(s.wrapping_add(0x14)).write_fpu(c, b_y);
    Addr(s.wrapping_add(0x10)).write_fpu(c, b_x);

    // The 12-byte vector b is copied to the icon at +0x44.
    c.ldl(3, s.wrapping_add(0x17));
    c.ldr(3, s.wrapping_add(0x10));
    c.g[A0][0] = Addr(s.wrapping_add(0x18)).lw(c);
    let icon = c.g[V0][0] as u32;
    c.sdl(3, icon.wrapping_add(0x4b));
    c.sdr(3, icon.wrapping_add(0x44));
    let z = c.g[A0][0];
    Addr(icon.wrapping_add(0x4c)).sw(c, z);

    // vt[9] of the icon, with a 1.
    c.g[A2][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_0fc4)?;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
    Ok(())
}

/// The grid half of UpdateSelection: the selection pair of the grid cell
/// (`a0` = the grid when this starts), then the grid's own vt[11] call.
fn selection_grid(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    selection_cell(c)?;

    // Tail: the grid's vt[11] with the selection frame colour (0x4c6808).
    c.cycles += 9;
    c.g[A2][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
    c.g[A1][0] = sx(0x004c_0000u32.wrapping_add(0x6808));
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x58)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x5c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_1094)?;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// The cell lookup of UpdateSelection: the item index, the selection pair
/// of the grid (+0x78, +0x7c), set from the item's vt[26] and vt[27].
fn selection_cell(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x5c)).lw(c);
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x64)).lw(c);
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.mult(0, 3, 3, 2);
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
    let vt = c.g[A1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x130)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x134)).lw(c);
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let target = c.g[V0][0] as u32;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, target, 0x001d_1018)?;
    let no_item = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[S1][0];
    if no_item {
        return Ok(());
    }

    // vt[26]: the item's row; a row at or after 0 is stored as is.
    c.cycles += 8;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0xd0)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0xd4)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_1034)?;
    c.g[V1][0] = c.g[V0][0];
    if (c.g[V1][0] as i64) >= 0 {
        c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        c.cycles += 2;
        let grid = c.g[V0][0] as u32;
        let row = c.g[V1][0];
        Addr(grid.wrapping_add(0x78)).sw(c, row);
        Addr(grid.wrapping_add(0x7c)).sw(c, 0);
        return Ok(());
    }

    // vt[27]: a negative row, the item's index is stored at +0x7c.
    c.cycles += 11;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0xd8)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0xdc)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_1058)?;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    let grid = c.g[V1][0] as u32;
    let count = c.g[V0][0];
    Addr(grid.wrapping_add(0x7c)).sw(c, count);
    Addr(grid.wrapping_add(0x78)).sw(c, c.g[A0][0]);
    Ok(())
}

// ---------------------------------------------------------------------------
// The five page constructors (0x1d12c8, 0x1d1460, 0x1d15f8, 0x1d1790, 0x1d1928).

/// What the gallery page constructors differ in.
struct RewardGalleryPage {
    /// The base constructor's page type (`t0`).
    kind: u64,
    /// The page's vtable is `0x470000 - vt_delta`.
    vt_delta: u32,
    /// Counts down from here: the item slots cleared are `last + 1`.
    last: u64,
    /// Offset of the first cleared slot (the slots are 4 bytes, going down).
    first: u32,
    /// Return address after the base constructor call.
    ret: u32,
    /// Loop head (the event check's pc).
    head: u32,
}

const RG_TRADING_CARDS: RewardGalleryPage = RewardGalleryPage {
    kind: 4,
    vt_delta: 0x5f38,
    last: 0x73,
    first: 0xbf4,
    ret: 0x001d_12e8,
    head: 0x001d_1300,
};

const RG_CONCEPT_ARTS: RewardGalleryPage = RewardGalleryPage {
    kind: 5,
    vt_delta: 0x60b8,
    last: 0x63,
    first: 0xbb4,
    ret: 0x001d_1480,
    head: 0x001d_1498,
};

const RG_VIDEOS: RewardGalleryPage = RewardGalleryPage {
    kind: 6,
    vt_delta: 0x6238,
    last: 0x1,
    first: 0xa2c,
    ret: 0x001d_1618,
    head: 0x001d_1630,
};

const RG_TOYS: RewardGalleryPage = RewardGalleryPage {
    kind: 7,
    vt_delta: 0x63b8,
    last: 0x1b,
    first: 0xa94,
    ret: 0x001d_17b0,
    head: 0x001d_17c8,
};

const RG_CHEAT_CHARS: RewardGalleryPage = RewardGalleryPage {
    kind: 8,
    vt_delta: 0x6538,
    last: 0x1b,
    first: 0xa94,
    ret: 0x001d_1948,
    head: 0x001d_1960,
};

/// `cFEStateRewardGalleryTradingCards::ctor(self, owner, a, b)` (0x1d12c8).
pub fn fe_state_reward_gallery_trading_cards_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_TRADING_CARDS);
}

/// `cFEStateRewardGalleryConceptArts::ctor` (0x1d1460).
pub fn fe_state_reward_gallery_concept_arts_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_CONCEPT_ARTS);
}

/// `cFEStateRewardGalleryVideos::ctor` (0x1d15f8).
pub fn fe_state_reward_gallery_videos_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_VIDEOS);
}

/// `cFEStateRewardGalleryToys::ctor` (0x1d1790).
pub fn fe_state_reward_gallery_toys_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_TOYS);
}

/// `cFEStateRewardGalleryCheatChars::ctor` (0x1d1928).
pub fn fe_state_reward_gallery_cheat_chars_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_CHEAT_CHARS);
}

/// The shared body of the page constructors: the base gallery constructor
/// (its `a2` is sign-extended from a byte), the page's vtable, a cleared
/// pointer at +0xa24 and the item slots from `first` down, cleared to 0.
fn reward_gallery_page_ctor(c: &mut Cpu, p: &RewardGalleryPage) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);

        c.g[T0][0] = p.kind;
        call_ptr(c, 0x001c_eea0, p.ret)?;

        let this = reward_gallery_self(c);
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(p.vt_delta));
        Addr(this.wrapping_add(0xa24)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(8)).sw(c, vtable);
        c.g[A0][0] = p.last;
        c.g[V0][0] = sx(this.wrapping_add(p.first));
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            Addr(slot).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, p.head);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, fake, vtable, LOG_COUNT, LOG_RESULTS};
    use crate::difftest::{check, fill, Rng, DATA};

    /// The fake callee with sentinel results. Fakes zero-extend the forced
    /// results, so 0xffff_fffe forces 0 and 0xffff_ffff forces -1 here.
    fn oc941_sentinel_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(LOG_COUNT);
        let forced = if n < 8 {
            c.bus.read32(LOG_RESULTS + 4 * n)
        } else {
            0
        };
        fake(c, target, ra);
        if c.pc == ra {
            match forced {
                0xffff_fffe => c.g[V0][0] = 0,
                0xffff_ffff => c.g[V0][0] = u64::MAX,
                _ => {}
            }
        }
    }

    /// Forced results for the first eight fake calls, chosen by the seed:
    /// zero, -1, a hash (0 = not forced) or a data pointer.
    fn oc941_force_results(c: &mut Cpu, seed: u32) {
        for n in 0..8u32 {
            let v = match (seed >> (2 * n)) & 3 {
                0 => 0xffff_fffe,
                1 => 0xffff_ffff,
                2 => 0,
                _ => DATA + 0x4000,
            };
            c.bus.write32(LOG_RESULTS + 4 * n, v);
        }
    }

    /// The reward types the viewer branches on (6 = video, 7 = toy, -1 = other).
    const OC941_TYPES: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, (-1i32) as u32];

    #[test]
    fn fe_state_reward_gallery_view_item_matches() {
        check(
            ee::s04::f_001d03f8__generated,
            fe_state_reward_gallery_view_item,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1d03f8, 6);
                c.hooks.miss = oc941_sentinel_miss;
                oc941_force_results(c, seed);
                let this = DATA + 0x1000;
                let vt = vtable(c, &mut r, &[0x110, 0x128]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x10, DATA + 0x3000);
                c.bus.write8(this + 0x44, r.next_u32() as u8);
                c.bus
                    .write32(this + 0x4c, OC941_TYPES[(seed % 10) as usize]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_update_item_info_matches() {
        check(
            ee::s04::f_001d0d30__generated,
            fe_state_reward_gallery_update_item_info,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1d0d30, 6);
                c.hooks.miss = oc941_sentinel_miss;
                oc941_force_results(c, seed);
                let this = DATA + 0x1000;
                let grid = DATA + 0x5000;
                let vt = vtable(c, &mut r, &[0x110, 0x118, 0x128, 0x130]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x5c, r.next_u32() & 0xf);
                c.bus.write32(this + 0x64, r.next_u32() & 0xf);
                c.bus
                    .write32(this + 0x8fc, if seed % 4 == 0 { 0 } else { grid });
                c.bus
                    .write32(this + 0x908, if seed % 3 == 0 { 0 } else { DATA + 0x6000 });
                c.bus
                    .write32(this + 0x90c, if seed % 5 == 0 { 0 } else { DATA + 0x6100 });
                c.bus.write32(grid + 0x18, r.next_u32() & 0x1f);
                fill(c, &mut r, DATA + 0x6000, 0x10);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_update_selection_matches() {
        check(
            ee::s04::f_001d0f30__generated,
            fe_state_reward_gallery_update_selection,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1d0f30, 6);
                c.hooks.miss = oc941_sentinel_miss;
                oc941_force_results(c, seed);
                let this = DATA + 0x1000;
                let icon = DATA + 0x5000;
                let grid = DATA + 0x5400;
                let vt_self = vtable(c, &mut r, &[0xd0, 0xd8, 0x130]);
                let vt_icon = vtable(c, &mut r, &[0x48]);
                let vt_grid = vtable(c, &mut r, &[0x58]);
                c.bus.write32(this + 8, vt_self);
                c.bus.write32(this + 0x5c, r.next_u32() & 0xf);
                c.bus.write32(this + 0x64, r.next_u32() & 0xf);
                c.bus.write32(this + 0x70, r.f32(10.0));
                c.bus.write32(this + 0x74, r.f32(10.0));
                c.bus
                    .write32(this + 0x8f8, if seed % 4 == 0 { 0 } else { icon });
                c.bus
                    .write32(this + 0x8fc, if seed % 3 == 0 { 0 } else { grid });
                c.bus.write32(icon + 8, vt_icon);
                fill(c, &mut r, icon + 0x40, 4);
                c.bus.write32(grid + 8, vt_grid);
                c.bus.write32(grid + 0x18, r.next_u32() & 0x1f);
                c.bus.write32(grid + 0x78, r.next_u32());
                c.bus.write32(grid + 0x7c, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// The page constructors' fixture: a zeroed gallery object with random
    /// junk in the arguments (the sign-extended byte in `a2`).
    fn oc941_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = fakes::machine_rt(c, seed, salt, 6);
        c.hooks.miss = oc941_sentinel_miss;
        oc941_force_results(c, seed);
        let this = DATA + 0x2000;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(DATA + 0x3000);
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
    }

    #[test]
    fn fe_state_reward_gallery_trading_cards_ctor_matches() {
        check(
            ee::s04::f_001d12c8__generated,
            fe_state_reward_gallery_trading_cards_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d12c8),
        );
    }

    #[test]
    fn fe_state_reward_gallery_concept_arts_ctor_matches() {
        check(
            ee::s04::f_001d1460__generated,
            fe_state_reward_gallery_concept_arts_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d1460),
        );
    }

    #[test]
    fn fe_state_reward_gallery_videos_ctor_matches() {
        check(
            ee::s04::f_001d15f8__generated,
            fe_state_reward_gallery_videos_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d15f8),
        );
    }

    #[test]
    fn fe_state_reward_gallery_toys_ctor_matches() {
        check(
            ee::s04::f_001d1790__generated,
            fe_state_reward_gallery_toys_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d1790),
        );
    }

    #[test]
    fn fe_state_reward_gallery_cheat_chars_ctor_matches() {
        check(
            ee::s04::f_001d1928__generated,
            fe_state_reward_gallery_cheat_chars_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d1928),
        );
    }
}
