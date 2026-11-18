//! `src/main/gamerender.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2 as sp;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_e730, game_view_man_dtor, "GameViewMan_dtor"),
    (
        0x0022_e7c8,
        game_view_man_reset_cameras,
        "GameViewMan_ResetCameras",
    ),
    (
        0x0022_e8b8,
        game_view_man_prefetch_views,
        "GameViewMan_PrefetchViews",
    ),
    (
        0x0022_e550,
        game_view_man_update_split_screen,
        "GameViewMan_UpdateSplitScreen",
    ),
    (0x0022_e3b8, game_view_man_ctor, "GameViewMan_ctor"),
    (0x0022_e968, game_module_ctor, "GameModule_ctor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
}

/// `void GameViewMan_ResetCameras(self)` (0x22e7c8): call the reset method of
/// each active view.
pub fn game_view_man_reset_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sq(16, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = lw_r(c, S2, 0x10);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x4));
            c.g[V1][0] = Addr(low(c, S0)).lw(c);
            loop {
                c.cycles += 12;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                c.g[A1][0] = lw_r(c, V1, 0xa0);
                c.g[V0][0] = lw_r(c, A1, 0x14);
                c.g[A0][0] = lh_r(c, V0, 0x20);
                c.g[V1][0] = lw_r(c, V0, 0x24);
                let target = low(c, V1);
                c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                call_ptr(c, target, 0x0022_e818)?;
                c.g[V0][0] = lw_r(c, S2, 0x10);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(low(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e7f8);
                }
            }
        }
        c.cycles += 6;
        c.lq(16, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `GameViewMan_dtor` (0x22e730): destroy every view in the manager's list
/// through its virtual destructor and optionally free the manager.
pub fn game_view_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        c.g[S3][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[S0][0] = sx(this.wrapping_add(4));
            loop {
                c.cycles += 3;
                let entry = Addr(r(c, S0)).lw(c);
                c.g[A2][0] = entry;
                c.g[A1][0] = 3;
                if entry != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(entry as u32, 0x90)).lw(c);
                    c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x0022_e780);
                    c.g[A0][0] = sx((entry as u32).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0022_e780)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S2)).lw(c);
                c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e760);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let free_self = c.g[V0][0] != 0;
        Addr(r(c, S2)).off(0x10).sw(c, 0);
        if free_self {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0022_e7a8);
            c.g[A0][0] = c.g[S2][0];
            callee::BUILTIN_DELETE.call(c, 0x0022_e7a8)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- GameViewMan_PrefetchViews

/// `GameViewMan.views` (`GameCamera*[]`, from +0x4) and `.count` (+0x10).
pub(crate) const GVM_VIEWS: u32 = 0x4;

pub(crate) const GVM_COUNT: u32 = 0x10;

/// `void GameViewMan_PrefetchViews(GameViewMan* self)` (0x22e8b8): tells the
/// world streamer where every active view camera looks
/// (`GameCamera_UpdateStreamView` for views 0..count).
pub fn game_view_man_prefetch_views(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(GVM_COUNT)).lw(c);
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(GVM_VIEWS));
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                call_ptr(c, 0x0015_ec98, 0x0022_e8f4)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(GVM_COUNT)).lw(c);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e8e8);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameViewMan_UpdateSplitScreen(GameViewMan *views)` (0x22e550): when
/// the game's view mode (`AI + 0x214`, 0..9) changes (and the game is not in
/// the 1..9 sized-session case read from `AI + 0x28`), stores it at `+0xc` and
/// lays out the views: the screen width and height come from the display
/// object (`gp - 0x854`, +0x10d8, methods +0x50/+0x54 and +0x58/+0x5c; the
/// unsigned int results are converted to float). Modes 0..2 and 6.. use one
/// full-screen view (far clip 30000), modes 3..5 two side-by-side views of half
/// the width (far clip 20000). Then the world's per-view slots (`+0x250`,
/// `+0x2a0` stride 0x50) of the unused views are cleared.
pub fn game_view_man_update_split_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, frame.at(c, 0x20));
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
        'done: {
            if c.g[V1][0] == c.g[A0][0] {
                frame.restore(c, S0, 0x10);
                break 'done;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'done;
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 1;
            let first_layout = c.g[V1][0] == c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A0][0]);
            if !first_layout {
                // Screen width -> f20 (unsigned int to float).
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0022_e5c0)?;
                let high_bit = (c.g[V0][0] as i64) < 0;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                if high_bit {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V0][0] &= 1;
                    c.g[V0][0] |= c.g[V1][0];
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    c.add_s(20, 20, 20);
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                }
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                // Screen height -> f2.
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0022_e604)?;
                let high_bit = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if high_bit {
                    c.cycles += 6;
                    c.g[V0][0] &= 1;
                    c.g[V0][0] |= c.g[V1][0];
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                    c.add_s(2, 2, 2);
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                }
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                // Lay out the views for the new mode (v1).
                c.cycles += 3;
                let below_6 = ((c.g[V1][0] as i64) < 6) as u64;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
                if below_6 != 0 {
                    c.cycles += 2;
                    let below_3 = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if !below_3 {
                        // Modes 3..5: two views side by side.
                        c.cycles += 20;
                        c.g[AT][0] = 0x3f00_0000; // 0.5
                        c.mtc1(1, 0);
                        c.g[V0][0] = 2;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                        c.mul_s(0, 20, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x51c8));
                        let this = c.g[S0][0] as u32;
                        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                        c.swc1(2, this.wrapping_add(0x34));
                        c.swc1(0, this.wrapping_add(0x30));
                        c.sw(this.wrapping_add(0x14), 0);
                        c.sw(this.wrapping_add(0x18), 0);
                        c.sw(this.wrapping_add(0x1c), 0);
                        c.swc1(0, this.wrapping_add(0x20));
                        c.swc1(2, this.wrapping_add(0x24));
                        c.swc1(0, this.wrapping_add(0x28));
                        c.sw(this.wrapping_add(0x2c), 0);
                        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = c.lw(this.wrapping_add(8));
                        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                    } else {
                        oc423_split_single_view(c);
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    oc423_split_single_view(c);
                }
                // Clear the unused views' world slots.
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 1;
                let two_views = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if two_views {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x250), 0);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x2a0), 0);
                } else {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                    c.g[A2][0] = 0x50;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.mult(0, A0, A0, A2);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x250), 0);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The one-view layout of `GameViewMan_UpdateSplitScreen` (v0 = 1 on entry):
/// view count 1, the full width and height, far clip `gp - 0x51c4`.
pub(crate) fn oc423_split_single_view(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x51c4));
    let this = c.g[S0][0] as u32;
    c.sw(this.wrapping_add(0x10), c.g[V0][0]);
    c.swc1(20, this.wrapping_add(0x20));
    c.swc1(2, this.wrapping_add(0x24));
    c.sw(this.wrapping_add(0x14), 0);
    c.sw(this.wrapping_add(0x18), 0);
    c.sw(this.wrapping_add(0x1c), 0);
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
}

/// `cGameModule* GameModule_ctor(cGameModule* self)` (0x22e968): constructs the
/// game module. Runs the base constructor (`0x231cd0`), points the vtable at
/// 0x47d110, clears `+0xc/+0x24/+0x48/+0x7c/+0x84`, runs the setup calls
/// (`0x244240(0)`, the sub-object constructors at `+0xb0`, `+0x100`, `+0x158`,
/// `+0x178`, `+0x1b8`), fills the embedded state-table entries (`+0xd0..+0x228`
/// with their vtables 0x47d... and ids 3/4/5/8/0xa/0xb/7/1) and clears the
/// eleven words from `+0x68` upward. Returns `self`.
///
/// This port keeps the generated register sequence (the callees see every
/// register) and only converts the call protocol (`call_ptr`, frame, events).
#[allow(
    unused_labels,
    unreachable_code,
    unused_assignments,
    unused_variables,
    clippy::all
)]
pub fn game_module_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 136;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x3;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = 0xa;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = 0x4;
        call_ptr(c, 0x00231cd0, 0x0022e998)?;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2ef0));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x24), 0u64);
        c.sw(c.g[S4][0] as u32, c.g[V0][0]);
        c.g[A0][0] = 0u64;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x48), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x7c), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), 0u64);
        call_ptr(c, 0x00244240, 0x0022e9c0)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x84), 0u64);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x00234188, 0x0022e9cc)?;
        c.g[S0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xd0), 0u64);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2918));
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xdc), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2d90));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xdc), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b28));
        c.sw((c.g[S4][0] as u32).wrapping_add(0xec), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xe0), 0u64);
        c.g[A1][0] = 0x5;
        c.sw((c.g[S4][0] as u32).wrapping_add(0xd4), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2b80));
        c.sw((c.g[S4][0] as u32).wrapping_add(0xd0), c.g[S1][0]);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x100));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xec), c.g[V1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xfc), c.g[S0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xf0), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xe4), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xe0), c.g[S3][0]);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xf4), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xf0), c.g[A1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
        call_ptr(c, 0x00233c10, 0x0022ea40)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x124), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x118), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2970));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[A0][0] = 0x8;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x124), c.g[V0][0]);
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x134), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2de8));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x128), 0u64);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x118), c.g[A0][0]);
        c.g[A2][0] = 0xb;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x11c), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x29c8));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x134), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2a78));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x128), c.g[A0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x144), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x158));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x138), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x12c), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x138), c.g[A2][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x154), c.g[S0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x148), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x144), c.g[V0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x13c), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x14c), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x148), c.g[S2][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x154), c.g[A1][0]);
        call_ptr(c, 0x00234ef8, 0x0022ead0)?;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x178));
        call_ptr(c, 0x00236868, 0x0022ead8)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x198), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x18c), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2e40));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[V1][0] = 0x7;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x198), c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x18c), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c30));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1a8), c.g[S0][0]);
        c.g[V0][0] = 0x1;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x19c), 0u64);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1b8));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x190), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1a0), c.g[S1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1b0), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1b4), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1a8), c.g[A1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x19c), c.g[V0][0]);
        call_ptr(c, 0x00236208, 0x0022eb30)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1e8), c.g[S0][0]);
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1dc), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2c88));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1f8), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e98));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1e8), c.g[V0][0]);
        c.g[A0][0] = 0xffffffffffffffff;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1ec), 0u64);
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x68));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
        c.g[A1][0] = 0x1;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1e0), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x488), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1f8), c.g[V1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1ec), c.g[S3][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x228), c.g[A0][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1f0), 0u64);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x224), 0u64);
        'l_22eb88: loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0xc), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V0][0] as u32, 0u64);
            let t = (c.g[A1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0022eb88);
                }
                continue 'l_22eb88;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.g[S4][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `GameViewMan* GameViewMan_ctor(GameViewMan* self, uint count)`: creates `count` (at most 2) game cameras (far clip 30000, or 20000 with two views), sets their controller, and stores the screen width and height.
/// (`GameViewMan_ctor`, 0x0022e3b8)
pub fn game_view_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_22e3f4: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = 0x1;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = 0xffffffffffffffff;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x51d4));
            let t = c.g[V0][0] != 0u64;
            c.sw(c.g[S2][0] as u32, c.g[A1][0]);
            if t {
                break 'b_22e3f4;
            } else {
                c.cycles += 1;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x51d0));
                break 'b_22e3f4;
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0u64;
        c.g[S3][0] = 0x480000;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
        'l_22e400: loop {
            'b_22e474: {
                'b_22e46c: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0x20000000;
                    if t {
                        break 'b_22e46c;
                    } else {
                        c.cycles += 23;
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x4b0;
                        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4ac8));
                        call_ptr(c, 0x00317d70, 0x0022e420)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0015dc70, 0x0022e42c)?;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[A1][0] = c.g[S1][0];
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x51cc));
                        c.mov_s(14, 20);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                        c.g[AT][0] = 0x41f00000;
                        c.mtc1(1, 13);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                        let t = c.g[A2][0] as u32;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0022e458)?;
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0015dfd8, 0x0022e464)?;
                        let t = true;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_22e474;
                        } else {
                            break 'b_22e46c;
                        }
                    }
                }
                c.cycles += 2;
                c.sw(c.g[S0][0] as u32, 0u64);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                break 'b_22e474;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 0x2u64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0022e400);
                }
                continue 'l_22e400;
            } else {
                'b_22e4dc: {
                    'b_22e4c0: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0022e4a8)?;
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if t {
                            break 'b_22e4c0;
                        } else {
                            c.cycles += 4;
                            c.mtc1(2, 20);
                            c.cvt_s_w(20, 20);
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                            if t {
                                break 'b_22e4dc;
                            } else {
                                break 'b_22e4c0;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    c.add_s(20, 20, 20);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    break 'b_22e4dc;
                }
                'b_22e51c: {
                    'b_22e504: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0022e4ec)?;
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                        if t {
                            break 'b_22e504;
                        } else {
                            c.cycles += 4;
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            let t = true;
                            c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_22e51c;
                            } else {
                                break 'b_22e504;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    c.add_s(0, 0, 0);
                    c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x20));
                    break 'b_22e51c;
                }
                c.cycles += 12;
                c.g[V0][0] = c.g[S2][0];
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x24));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), 0u64);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;
    use ssx3_ee as ee;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;

    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::main::gamemode::tests::be_library;

    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS;

    #[test]
    fn game_view_man_reset_cameras_matches() {
        check(
            ee::s05::f_0022e7c8__generated,
            game_view_man_reset_cameras,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22e7c8, 1);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                let n = seed % 3;
                w(c, obj + 0x10, n);
                for i in 0..n {
                    let v = DATA + 0x1000 + 0x100 * i;
                    let o = DATA + 0x1400 + 0x100 * i;
                    w(c, obj + 4 + 4 * i, v);
                    w(c, v + 0xa0, o);
                    w(c, o + 0x14, DATA + 0x3000);
                    slot(c, DATA + 0x3000, 0x20, 0x30, FAKE + 0x40 * i);
                    w(c, v + 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_view_man_dtor_matches() {
        check(
            ee::s05::f_0022e730__generated,
            game_view_man_dtor,
            SEEDS_2,
            &alloc_base_setup(|c, seed, r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x60) } else { DATA };
                let count = seed % 3;
                w(c, this, count);
                for i in 0..count {
                    let entry = if (seed >> (i + 1)) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x800 + 0x40 * i
                    };
                    if entry != 0 {
                        let vt = vtable(c, r, &[8]);
                        w(c, entry + 0x90, vt);
                    }
                    w(c, this + 4 + 4 * i, entry);
                }
                w(c, this + 0x10, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    use crate::difftest::elf_image;

    /// `GameViewMan_PrefetchViews`: 0..3 views, each running the real
    /// `GameCamera_UpdateStreamView` on a world cache with no streamer
    /// nodes (the camera test's fixture), race mode 1 or not.
    #[test]
    fn game_view_man_prefetch_views_matches() {
        check(
            ssx3_ee::s05::f_0022e8b8__generated,
            game_view_man_prefetch_views,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_e8b8, 0);
                elf_image(c);
                c.pc = 0x0022_e8b8;
                let app = DATA + 0x1000;
                let module = DATA + 0x1100;
                let race = DATA + 0x1200;
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x84, race);
                c.bus.write32(
                    race + 0x10,
                    if seed % 4 == 0 { 1 } else { r.next_u32() % 3 },
                );
                let cache_ptr = DATA + 0x2000;
                let cache = DATA + 0x3000;
                let nodes = DATA + 0x2100;
                let clock = DATA + 0x2200;
                c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
                c.bus.write32(cache_ptr, cache);
                c.bus.write32(cache + 0x10 + 4, nodes);
                c.bus.write32(nodes + 8, 0);
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus.write32(clock + 0x18, r.next_u32());
                let man = DATA + 0x400;
                let count = seed % 4;
                for i in 0..3 {
                    let cam = DATA + 0x8000 + 0x100 * i;
                    c.bus.write32(man + 4 + 4 * i, cam);
                    c.bus
                        .write32(cam + 0x8, ((r.next_u32() % 40000) as f32).to_bits());
                    for k in 0..4 {
                        c.bus.write32(cam + 0x20 + 4 * k, r.f32(5000.0));
                    }
                    c.bus.write32(cam + 0xb0, r.next_u32() % 4);
                    c.bus.write32(cam + 0xb4, r.next_u32());
                }
                c.bus.write32(man + 0x10, count);
                c.g[A0][0] = sx(man);
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w_5 as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_CTRL;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_MAGIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_RET;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::hook_serialize_read_u32;

    /// New view modes 0..8 (one view, two views, one view again), the width and
    /// height methods answered by the faked `Serialize_ReadU32` (so the
    /// unsigned-to-float high-bit branch is reachable through its scripted
    /// negative results).
    #[test]
    fn game_view_man_update_split_screen_matches() {
        hook_serialize_read_u32();
        fakes::covered(
            ee::s05::f_0022e550__generated,
            game_view_man_update_split_screen,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_e550, 2);
                w_2(c, OC423_CTRL, OC423_MAGIC);
                let sizes = [0x500u32, 0x2d0, 0xffff_ffff, 0x8000_0001, 0x280];
                w_2(c, OC423_RET, sizes[(seed / 5 % 5) as usize]);
                w_2(c, OC423_RET + 4, sizes[(seed / 3 % 5) as usize]);
                let views = DATA;
                let display = DATA + 0x2000;
                let display_vt = DATA + 0x4000;
                let world = DATA + 0x5000;
                let game = DATA + 0x5100;
                let slots_ptr = DATA + 0x5300;
                let table = DATA + 0x6000;
                let cam1 = DATA + 0x7000;
                w_2(c, GAME_GP - 0x848, world);
                w_2(c, GAME_GP - 0x854, display);
                w_2(c, GAME_GP - 0x51c8, 20000.0f32.to_bits());
                w_2(c, GAME_GP - 0x51c4, 30000.0f32.to_bits());
                w_2(c, world + 0x84, game);
                let mode = seed % 9;
                w_2(c, game + 0x214, mode);
                w_2(c, game + 0x28, DATA + 0x5200);
                w_2(
                    c,
                    DATA + 0x5200,
                    [0, 0, 20, 30, 5, 10, 0, 1][(seed / 7 % 8) as usize],
                );
                w_2(c, game + 0x10, slots_ptr);
                w_2(c, slots_ptr, table);
                for i in 0..0x30u32 {
                    w_2(c, table + 0x250 + 4 * i, r.next_u32() | 1);
                }
                w_2(c, display + 0x10d8, display_vt);
                c.bus.write16(display_vt + 0x50, 0x10);
                w_2(c, display_vt + 0x54, 0x002d_2510);
                c.bus.write16(display_vt + 0x58, 0x20);
                w_2(c, display_vt + 0x5c, 0x002d_2510);
                w_2(c, views, if seed % 4 == 0 { 1 } else { 0 });
                w_2(c, views + 4, DATA + 0x7800);
                w_2(c, views + 8, cam1);
                w_2(c, cam1 + 0xb0, seed / 2 % 2);
                w_2(
                    c,
                    views + 0xc,
                    if seed % 5 == 0 {
                        mode
                    } else {
                        r.next_u32() % 9
                    },
                );
                w_2(c, views + 0x10, r.next_u32());
                c.g[A0][0] = views as u64;
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;

    use crate::difftest::RA as SENTINEL;

    pub(crate) use crate::game::be::belibrary::tests::oc431_force;
    pub(crate) use crate::game::be::belibrary::tests::oc431_install;
    pub(crate) use crate::game::be::belibrary::tests::oc431_machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;

    /// `GameModule_ctor`: all seven constructor / setup callees are gated
    /// fakes (some unwind); the state-table stores and the final clearing loop
    /// run for real.
    #[test]
    fn game_module_ctor_matches() {
        oc430_install_fakes();
        covered(
            ee::s05::f_0022e968__generated,
            game_module_ctor,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0022_e968, 7);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0022_e968;
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    /// The same with event delivery at the clearing loop's back edge.
    #[test]
    fn game_module_ctor_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s05::f_0022e968__generated,
            game_module_ctor,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_e969, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                let mode = schedule(c, &mut r, seed, 0x0022_e968);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    /// Address of the vtable the fixture's camera (`GameCamera_ctor` puts
    /// `0x45b888` at +0x90) is called through.
    pub(crate) const OC431_CAMERA_VT: u32 = 0x0045_b888;

    /// `GameViewMan_ctor`: 0 to 3 views (far clip 20000 from two views on),
    /// real `GameCamera_ctor` over fake controller constructors, the camera's
    /// vtable method and the two display-size calls (positive and negative
    /// results, the latter through the halving path).
    #[test]
    fn game_view_man_ctor_matches() {
        oc431_install();
        covered(
            ee::s05::f_0022e3b8__generated,
            game_view_man_ctor,
            200,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x22e3b8, 26, 0x0022_e3b8);
                let (this, grman, disp) =
                    (OC431_BASE + 0x100, OC431_BASE + 0x1000, OC431_BASE + 0x1100);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(GAME_GP - 0x51d4, 30000.0f32.to_bits());
                c.bus.write32(GAME_GP - 0x51d0, 20000.0f32.to_bits());
                c.bus.write32(GAME_GP - 0x51cc, 0.785_398_2f32.to_bits());
                c.bus
                    .write16(OC431_CAMERA_VT + 0x10, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC431_CAMERA_VT + 0x14, FAKE);
                c.bus.write32(GAME_GP - 0x854, grman);
                c.bus.write32(grman + 0x10d8, disp);
                c.bus.write16(disp + 0x50, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(disp + 0x54, FAKE);
                c.bus.write16(disp + 0x58, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(disp + 0x5c, FAKE);
                // The two display-size calls are the last two fake calls (the
                // first return value of each camera is irrelevant).
                let cams = (seed % 4).min(2);
                let last = cams * 11;
                for k in 0..2 {
                    let v = match r.next_u32() % 3 {
                        0 => 640,
                        1 => 0xffff_fff1,
                        _ => 0x8000_0001,
                    };
                    oc431_force(c, last + k, v);
                }
            },
        );
    }
}
