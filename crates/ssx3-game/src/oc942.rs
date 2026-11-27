//! Staging module oc942 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc942`.

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
        0x001d_2150,
        cfe_state_preview_reward_request_image,
        "cFEStatePreviewReward_RequestImage",
    ),
    (
        0x001d_2198,
        cfe_state_preview_reward_poll_image_load,
        "cFEStatePreviewReward_PollImageLoad",
    ),
    (
        0x001d_23e0,
        cfe_state_video_player_init,
        "cFEStateVideoPlayer_Init",
    ),
    (
        0x001d_2990,
        cfe_state_mountain_room_init,
        "cFEStateMountainRoom_Init",
    ),
    (
        0x001d_31c0,
        cfe_state_mountain_room_poll_image_load,
        "cFEStateMountainRoom_PollImageLoad",
    ),
    (
        0x001d_3780,
        cfe_state_peak_room_ctor,
        "cFEStatePeakRoom_ctor",
    ),
    (
        0x001d_3c60,
        cfe_state_peak_room_on_activate,
        "cFEStatePeakRoom_OnActivate",
    ),
];

/// `lw` of the world pointer at `gp - 0x848`.
fn fe_state_world(c: &mut Cpu) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848))
}

// Offsets of the preview-reward state (`self`): the bolt bank and handle of its
// texture, the texture id, the load state (0 equal, 1 b < a, 2 a < b, which
// also indexes the two per-state arrays at +0x48 and +0x54).
const RWD_BANK: u32 = 0x64;
const RWD_HANDLE: u32 = 0x6c;
const RWD_TEX: u32 = 0x70;
const RWD_STATE: u32 = 0x74;
const RWD_SLOTS: u32 = 0x48;
const RWD_OBJS: u32 = 0x54;

/// `void cFEStatePreviewReward_RequestImage(self, id)` (0x1d2150): ask the
/// bolt manager of the world (`world->bolt + 0x1a70`) for bank `self->bank`
/// item `id`; the handle goes to `self->handle`.
pub fn cfe_state_preview_reward_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = fe_state_world(c);
        frame.save(c, S0, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = 9;
        c.g[T0][0] = 1;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[T1][0] = 0;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019_da10, 0x001d_2184)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `'b_1d2224` part of `cFEStatePreviewReward_PollImageLoad`: fetch the
/// entry of the loaded item and store the state at `self + 0x74`: 1 when the
/// second 16-bit value is below the first, 2 when the first is below the
/// second, 0 when they are equal.
fn rwd_poll_state(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE));
    call_ptr(c, 0x0019_e2b0, 0x001d_21e4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let b_below_a = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if b_below_a {
        c.cycles += 2;
        c.sw((c.g[S0][0] as u32).wrapping_add(RWD_STATE), c.g[V0][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let a_below_b = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if a_below_b {
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(RWD_STATE), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(RWD_STATE), 0);
        }
    }
    Ok(())
}

/// The `'b_1d22d4` part of `cFEStatePreviewReward_PollImageLoad`: create the
/// render texture for the entry (3b3d00 fills the name, the render vtable
/// call makes the texture), release the bolt handle and attach the texture
/// to the per-state object, if there is one.
fn rwd_poll_attach(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    c.g[A2][0] = c.g[SP][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x003b_3d00, 0x001d_2230)?;
    c.sb((c.g[SP][0] as u32).wrapping_add(0x4), 0);
    c.g[A2][0] = c.g[SP][0];
    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.g[A3][0] = 0;
    c.g[T0][0] = 1;
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_2260)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE));
    c.sw((c.g[S0][0] as u32).wrapping_add(RWD_TEX), c.g[V0][0]);
    call_ptr(c, 0x0019_dc20, 0x001d_2274)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_STATE));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.sw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RWD_SLOTS));
    let no_slot = c.g[A0][0] == 0;
    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(RWD_OBJS));
    if no_slot {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    let no_obj = c.g[V1][0] == 0;
    c.g[A1][0] = 1;
    if no_obj {
        return Ok(());
    }
    c.cycles += 12;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_TEX));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x7c), 0);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_STATE));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x001d_22d4)?;
    Ok(())
}

/// `void cFEStatePreviewReward_PollImageLoad(self)` (0x1d2198): while the
/// texture request is pending, ask the bolt manager whether it has finished
/// (0x19e238). When it has, read the entry's state, attach the texture, and
/// clear the handle.
pub fn cfe_state_preview_reward_poll_image_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = fe_state_world(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE));
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0019_e238, 0x001d_21cc)?;
        if c.g[V0][0] != 0 {
            rwd_poll_state(c)?;
            rwd_poll_attach(c)?;
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateVideoPlayer_Init(self)` (0x1d23e0): open the movie player
/// (0x317d70 for the 0x44-byte allocation, 0x2533c8 for the player object),
/// store the player at `+0x248`, and start playback.
pub fn cfe_state_video_player_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x1320));
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x44;
        call_ptr(c, 0x0031_7d70, 0x001d_2408)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0025_33c8, 0x001d_2410)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x248), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_25e8, 0x001d_241c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The slot-setup block shared by the three `cFEStateMountainRoom_Init`
/// blocks: fetch the slot's pointer (0x317670), look its object up (0x39d860)
/// and, when there is one, run the slot's init (0x3a0c50). `self_pc` is the
/// block's own return address.
fn mr_init_slot(
    c: &mut Cpu,
    a0_off: u32,
    ret_fetch: u32,
    ret_find: u32,
    a1_off: u32,
    self_pc: u32,
) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(a0_off));
    call_ptr(c, 0x0031_7670, ret_fetch)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)?;
    let found = c.g[V0][0] != 0;
    c.g[A1][0] = 0x004a_0000;
    if found {
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(a1_off));
        call_ptr(c, 0x003a_0c50, self_pc)?;
    }
    Ok(())
}

/// `void cFEStateMountainRoom_Init(self)` (0x1d2990): store the room's
/// texture object (0x39 7cc0) and its first load, then run the three slot
/// setups and the two finishing calls (0x28b180, 0x28f140).
pub fn cfe_state_mountain_room_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x7228));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x001d_29b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001d_29c8)?;
        let stored = c.g[V0][0] != 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if stored {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001d_29e0)?;
        }
        mr_init_slot(c, 0x1f08, 0x001d_29ec, 0x001d_29f8, 0x14f8, 0x001d_2a0c)?;
        mr_init_slot(c, 0x1f10, 0x001d_2a18, 0x001d_2a24, 0x1500, 0x001d_2a38)?;
        mr_init_slot(c, 0x1f18, 0x001d_2a44, 0x001d_2a50, 0x1508, 0x001d_2a64)?;
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x001d_2a6c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 7;
        call_ptr(c, 0x0028_f140, 0x001d_2a78)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `'b_1d3284` and `'b_1d32a4` blocks of `cFEStateMountainRoom_PollImageLoad`:
/// the chain that leaves a0 = the bit (`(x >> 6) & 1`) of the first nested
/// object's field, or 0 when a link is missing.
fn mr_image_bit_chain(c: &mut Cpu) {
    c.cycles += 2;
    let v1_missing = c.g[V1][0] == 0;
    c.g[V0][0] = 0;
    if !v1_missing {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[V0][0] &= 1;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[A0][0] = c.g[V0][0] & 1;
        }
    }
}

/// The `'b_1d3268` block of `cFEStateMountainRoom_PollImageLoad`: ask 0x19e2b0
/// for the item's entry, start the texture load through the render vtable,
/// store the texture id in the room and the object, and set a0 for the chain.
fn mr_image_state(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
    call_ptr(c, 0x0019_e2b0, 0x001d_320c)?;
    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.g[A2][0] = sx(0x004a_1f40);
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
    c.g[A3][0] = 0;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T0][0] = 1;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x001d_3240)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
    let missing = c.g[V1][0] == 0;
    c.g[A0][0] = 0;
    if !missing {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[A0][0] = c.g[V0][0] & 1;
    }
    Ok(())
}

/// The tail of `cFEStateMountainRoom_PollImageLoad` after the state block:
/// the `'b_1d32b8` block (show the image when the bit chain says so, else
/// keep the request), then the bolt release (0x19dc20) and the handle reset.
fn mr_image_tail(c: &mut Cpu) -> Flow {
    // 'b_1d32b4 is entered when a0 (the request flag) is non-zero.
    c.cycles += 2;
    let a0_zero = c.g[A0][0] == 0;
    c.g[A0][0] = 0;
    if !a0_zero {
        mr_image_bit_chain(c);
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
            return mr_image_release(c);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_32e0, 0x001d_32b4)?;
    }
    c.cycles += 1;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
    mr_image_release(c)
}

/// The final call of `cFEStateMountainRoom_PollImageLoad`: release the bolt
/// bank of the room (0x19dc20) and set its handle to -1.
fn mr_image_release(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
    call_ptr(c, 0x0019_dc20, 0x001d_32c4)?;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
    Ok(())
}

/// `void cFEStateMountainRoom_PollImageLoad(self)` (0x1d31c0): the room's
/// counterpart of the preview poll: when a request is pending and the bolt
/// manager reports it done, start the texture and attach it; a not-yet-done
/// request returns at once.
pub fn cfe_state_mountain_room_poll_image_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = fe_state_world(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        let idle = c.g[A0][0] == 0;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
        if !idle {
            c.cycles += 6;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb0));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x0019_e238, 0x001d_31f8)?;
            let ready = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if ready {
                mr_image_state(c)?;
                mr_image_tail(c)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePeakRoom_ctor(self, kind, ..., ...)` (0x1d3780): set the
/// room's type fields, load its textures (0x1a1cd0, 0x14dc80, 0x14dd58,
/// 0x147488) and clear the three slot tables with the two loops (each slot
/// word and the last field are zeroed).
pub fn cfe_state_peak_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(((c.g[A2][0] as u32) << 24) as u32);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x001d_37a8)?;
        c.g[V0][0] = 0x2c;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x6878));
        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sb((c.g[S2][0] as u32).wrapping_add(0x44), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        call_ptr(c, 0x001a_1cd0, 0x001d_37d0)?;
        c.sb((c.g[S2][0] as u32).wrapping_add(0x15), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x001d_37d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001d_37e4)?;
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7488, 0x001d_37f0)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0xdc), c.g[V0][0]);
        c.g[V1][0] = 3;
        c.sw((c.g[S2][0] as u32).wrapping_add(0xe0), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
        loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32), 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_3800);
            }
        }
        c.cycles += 7;
        c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa8));
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x94));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
        c.g[T0][0] = 4;
        loop {
            c.cycles += 14;
            c.sw((c.g[V0][0] as u32), 0);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.sw((c.g[V1][0] as u32), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.sw((c.g[A0][0] as u32), 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            c.sw((c.g[A1][0] as u32), 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.sw((c.g[A2][0] as u32), 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            c.sw((c.g[A3][0] as u32), 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            let more = (c.g[T0][0] as i64) >= 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_3838);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0xd0), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePeakRoom_OnActivate(self)` (0x1d3c60): a forwarding
/// wrapper around 0x39e4c0.
pub fn cfe_state_peak_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e4c0, 0x001d_3c70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes (their result cells follow it).
    const OC942_GATE: u32 = DATA + 0x20_0000;
    const OC942_MAGIC: u32 = 0x4f43_3942;
    /// One 32-bit result cell per fake, indexed by the fake's number.
    const OC942_RES: u32 = DATA + 0x20_0010;
    const WORLD: u32 = DATA + 0x1000;
    /// The bolt manager holder: the manager is `MGR + 0x1a70`.
    const MGR: u32 = DATA + 0x2000;
    /// `*(gp - 0x854)`: the render context, with its vtable at `+0x10d8`.
    const RENDER: u32 = DATA + 0x4000;
    const VT: u32 = DATA + 0x5000;
    const VT2: u32 = DATA + 0x6000;
    const OBJ2: u32 = DATA + 0x7000;
    /// The bolt entry returned by 0x19e2b0 (its table offset at +0x14).
    const ENTRY: u32 = DATA + 0x8000;
    const OBJ: u32 = DATA + 0x9000;
    const OBJ3: u32 = DATA + 0xa000;
    const SELF: u32 = DATA + 0x400;

    fn oc942_install_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc942_install();
        let r = machine_rt(c, seed, salt, 6);
        c.bus.write32(OC942_GATE, OC942_MAGIC);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x7c, MGR);
        c.bus.write32(GAME_GP - 0x854, RENDER);
        c.bus.write32(RENDER + 0x10d8, VT);
        c.bus.write32(VT + 0x174, FAKE);
        c.bus.write32(VT2 + 0x4c, FAKE);
        c.bus.write32(OBJ2 + 8, VT2);
        r
    }

    fn oc942_cell(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC942_RES + 4 * idx, v);
    }

    /// The fake callee body: log the call, then return the cell `idx` (only
    /// when it returns to its caller).
    fn oc942_call(c: &mut Cpu, target: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(OC942_RES + 4 * idx));
        }
    }

    macro_rules! oc942_fakes {
        ($(($fake:ident, $prev:ident, $slot:path, $gen:path, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC942_GATE) != OC942_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc942_call(c, $addr, $idx);
                }
            )*
            fn oc942_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc942_fakes!(
        (
            oc942_fk_0019da10,
            OC942_PREV_0019DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10,
            0
        ),
        (
            oc942_fk_0019e238,
            OC942_PREV_0019E238,
            ee::s03::OVERRIDE_0019E238,
            ee::s03::f_0019e238__generated,
            0x0019_e238,
            1
        ),
        (
            oc942_fk_0019e2b0,
            OC942_PREV_0019E2B0,
            ee::s03::OVERRIDE_0019E2B0,
            ee::s03::f_0019e2b0__generated,
            0x0019_e2b0,
            2
        ),
        (
            oc942_fk_0019dc20,
            OC942_PREV_0019DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20,
            3
        ),
        (
            oc942_fk_003b3d00,
            OC942_PREV_003B3D00,
            ee::s13::OVERRIDE_003B3D00,
            ee::s13::f_003b3d00__generated,
            0x003b_3d00,
            4
        ),
        (
            oc942_fk_00317670,
            OC942_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            5
        ),
        (
            oc942_fk_00397cc0,
            OC942_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0,
            6
        ),
        (
            oc942_fk_0039c870,
            OC942_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            7
        ),
        (
            oc942_fk_0039d860,
            OC942_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            8
        ),
        (
            oc942_fk_003a0c50,
            OC942_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            9
        ),
        (
            oc942_fk_0028b180,
            OC942_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180,
            10
        ),
        (
            oc942_fk_0028f140,
            OC942_PREV_0028F140,
            ee::s07::OVERRIDE_0028F140,
            ee::s07::f_0028f140__generated,
            0x0028_f140,
            11
        ),
        (
            oc942_fk_001d25e8,
            OC942_PREV_001D25E8,
            ee::s04::OVERRIDE_001D25E8,
            ee::s04::f_001d25e8__generated,
            0x001d_25e8,
            12
        ),
        (
            oc942_fk_001d32e0,
            OC942_PREV_001D32E0,
            ee::s04::OVERRIDE_001D32E0,
            ee::s04::f_001d32e0__generated,
            0x001d_32e0,
            13
        ),
        (
            oc942_fk_0014dc80,
            OC942_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            14
        ),
        (
            oc942_fk_0014dd58,
            OC942_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            15
        ),
        (
            oc942_fk_00147488,
            OC942_PREV_00147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488,
            16
        ),
        (
            oc942_fk_001a1cd0,
            OC942_PREV_001A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0,
            17
        ),
        (
            oc942_fk_0039e2a0,
            OC942_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0,
            18
        ),
        (
            oc942_fk_0039e4c0,
            OC942_PREV_0039E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0,
            19
        ),
        (
            oc942_fk_00317d70,
            OC942_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            20
        ),
        (
            oc942_fk_002533c8,
            OC942_PREV_002533C8,
            ee::s06::OVERRIDE_002533C8,
            ee::s06::f_002533c8__generated,
            0x0025_33c8,
            21
        ),
    );

    #[test]
    fn cfe_state_preview_reward_request_image_matches() {
        check(
            ee::s04::f_001d2150__generated,
            cfe_state_preview_reward_request_image,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_2150);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = (seed % 7) as u64;
                c.bus.write32(SELF + RWD_BANK, seed % 3);
                oc942_cell(c, 0, 0x55 + seed);
            },
        );
    }

    /// Fill the bolt entry the poll reads: the table offset at +0x14 and the
    /// two 16-bit values at +4 and +6 (the three orders of the state).
    fn oc942_entry(c: &mut Cpu, seed: u32) {
        c.bus.write32(ENTRY + 0x14, 0x40);
        let (a, b) = match seed % 3 {
            0 => (5u16, 2u16),
            1 => (2, 5),
            _ => (3, 3),
        };
        c.bus.write16(ENTRY + 0x40 + 4, a);
        c.bus.write16(ENTRY + 0x40 + 6, b);
    }

    #[test]
    fn cfe_state_preview_reward_poll_image_load_matches() {
        check(
            ee::s04::f_001d2198__generated,
            cfe_state_preview_reward_poll_image_load,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_2198);
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + RWD_BANK, seed % 3);
                c.bus.write32(SELF + RWD_HANDLE, 0x55 + seed);
                oc942_entry(c, seed);
                oc942_cell(c, 1, if seed % 4 == 0 { 0 } else { 1 });
                oc942_cell(c, 2, ENTRY);
                oc942_cell(c, 3, seed);
                for k in 0..3 {
                    let slot = k * 4;
                    if seed % 5 == 0 {
                        c.bus.write32(SELF + RWD_SLOTS + slot, 0);
                    } else {
                        c.bus.write32(SELF + RWD_SLOTS + slot, OBJ);
                        c.bus.write32(SELF + RWD_OBJS + slot, OBJ2);
                    }
                }
                c.bus.write32(SELF + RWD_STATE, seed % 3);
            },
        );
    }

    #[test]
    fn cfe_state_video_player_init_matches() {
        check(
            ee::s04::f_001d23e0__generated,
            cfe_state_video_player_init,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_23e0);
                c.g[A0][0] = sx(SELF);
                oc942_cell(c, 20, 0x0009_0000 + seed);
                oc942_cell(c, 21, 0x0009_1000 + seed);
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_init_matches() {
        check(
            ee::s04::f_001d2990__generated,
            cfe_state_mountain_room_init,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_2990);
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + 0x10, OBJ);
                oc942_cell(c, 5, OBJ3);
                oc942_cell(c, 6, if seed % 2 == 0 { 0 } else { OBJ3 + 0x40 });
                oc942_cell(c, 8, if seed % 3 == 0 { 0 } else { OBJ3 + 0x80 });
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_poll_image_load_matches() {
        check(
            ee::s04::f_001d31c0__generated,
            cfe_state_mountain_room_poll_image_load,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_31c0);
                c.g[A0][0] = sx(SELF);
                c.bus
                    .write32(SELF + 0x50, if seed % 5 == 0 { 0 } else { OBJ2 });
                c.bus.write32(SELF + 0xb0, 0x77 + seed);
                c.bus.write32(SELF + 0xbc, 0x88 + seed);
                c.bus.write32(SELF + 0x48, OBJ3);
                c.bus.write32(SELF + 0x4c, OBJ3);
                c.bus
                    .write32(OBJ3 + 0x14, if seed % 2 == 0 { 0x40 } else { 0 });
                c.bus.write32(OBJ2 + 0x78, 0);
                oc942_cell(c, 1, if seed % 3 == 0 { 0 } else { 1 });
                oc942_cell(c, 2, ENTRY);
                oc942_entry(c, seed);
                oc942_cell(c, 3, seed);
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_ctor_matches() {
        check(
            ee::s04::f_001d3780__generated,
            cfe_state_peak_room_ctor,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_3780);
                c.g[A0][0] = sx(SELF);
                c.g[A2][0] = (seed as u64) & 0xff;
                c.g[A3][0] = 0x10 * (seed as u64 % 4);
                c.bus.write32(SELF + 0x10, OBJ);
                oc942_cell(c, 14, 3 + seed);
                oc942_cell(c, 15, 4);
                oc942_cell(c, 16, 0x5 + seed);
                oc942_cell(c, 17, 0x6 + seed);
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_on_activate_matches() {
        check(
            ee::s04::f_001d3c60__generated,
            cfe_state_peak_room_on_activate,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_3c60);
                c.g[A0][0] = sx(SELF);
            },
        );
    }
}
