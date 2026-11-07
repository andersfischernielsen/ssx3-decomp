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
#[cfg(test)]
use ssx3_ee as ee;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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
}
