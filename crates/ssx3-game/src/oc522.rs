//! Staging module oc522 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc522`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_3c30, cfe_load_screen_ctor, "cFELoadScreen_ctor"),
    (0x0024_4770, gm_handler_call_hook78, "GMHandler_CallHook78"),
    (
        0x0024_4880,
        game_state_finish_clear_flag40,
        "GameStateFinish_ClearFlag40",
    ),
    (
        0x0024_4888,
        game_module_wait_views_ready,
        "GameModule_WaitViewsReady",
    ),
    (
        0x0024_5118,
        cgame_comm_update_if_active,
        "cGameComm_UpdateIfActive",
    ),
    (0x0024_5140, netconn_idle_if_active, "NetConn_IdleIfActive"),
    (
        0x0024_5168,
        app_module_poll_exit_request,
        "AppModule_PollExitRequest",
    ),
    (
        0x0024_51b0,
        game_module_reset_all_riders,
        "GameModule_ResetAllRiders",
    ),
    (
        0x0024_5228,
        cfe_state_progressive_dtor,
        "cFEStateProgressive_dtor",
    ),
];

/// `cFELoadScreen::cFELoadScreen(this)` (0x243c30): runs the AppModule base
/// constructor, then stores the vtable word 0x47d740 at +0x0 and clears the
/// fields at +0xc, +0x10 and +0x14.
pub fn cfe_load_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0023_1cd0,
            f: ee::s06::f_00231cd0,
        }
        .call(c, 0x0024_3c44)?;
        let this = Addr(c.g[S0][0] as u32);
        let vtable = sx(0x0048_0000u32.wrapping_sub(0x28c0));
        c.g[V1][0] = vtable;
        c.g[V0][0] = c.g[S0][0];
        this.off(0xc).sw(c, 0);
        this.sw(c, vtable);
        this.off(0x10).sw(c, 0);
        this.off(0x14).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GMHandler::CallHook78(this)` (0x244770): tail-calls the function pointer
/// stored at +0x78 of `this`, returning to its own caller.
pub fn gm_handler_call_hook78(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let target = Addr(c.g[A0][0] as u32).off(0x78).lw(c);
        c.g[V0][0] = target;
        call_ptr(c, target as u32, 0x0024_4784)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameStateFinish::ClearFlag40(this)` (0x244880): clears the word at +0x40.
pub fn game_state_finish_clear_flag40(c: &mut Cpu) {
    c.cycles += 2;
    Addr(c.g[A0][0] as u32).off(0x40).sw(c, 0);
    ret(c);
}

/// `GameModule::WaitViewsReady(this)` (0x244888): asks the view manager to
/// prefetch (`0x22e8b8` on the view list at +0x84), then returns whether the
/// views are ready (`GameViewMan_AllViewsReady` at +0x84, unless the
/// prefetch flag at +0x9c is set). When they are ready, both flags at +0x9c
/// and +0xa0 are set.
pub fn game_module_wait_views_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(c.g[S0][0] as u32);
        c.g[A0][0] = this.off(0x84).lw(c);
        call_ptr(c, 0x0022_e8b8, 0x0024_48a0)?;
        c.g[V0][0] = this.off(0x9c).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[A0][0] = this.off(0x84).lw(c);
            call_ptr(c, 0x0022_e920, 0x0024_48b4)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 4;
                c.g[V1][0] = 1;
                c.g[V0][0] = 0;
                this.off(0xa0).sw(c, 1);
                this.off(0x9c).sw(c, 1);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cGameComm::UpdateIfActive()` (0x245118): runs `cGameComm_Update` on the
/// object at gp-0x204 when there is one.
pub fn cgame_comm_update_if_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let comm = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        c.g[A0][0] = comm;
        frame.save_ra(c, 0);
        if comm != 0 {
            c.cycles += 2;
            Callee {
                addr: 0x0025_70d8,
                f: ee::s06::f_002570d8,
            }
            .call(c, 0x0024_5130)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetConn::IdleIfActive()` (0x245140): calls `NetConnIdle` through the
/// function table when the connection at gp-0x238 exists.
pub fn netconn_idle_if_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let conn = Addr((c.g[GP][0] as u32).wrapping_sub(0x238)).lw(c);
        c.g[V0][0] = conn;
        frame.save_ra(c, 0);
        if conn != 0 {
            c.cycles += 2;
            call_ptr(c, 0x003f_42a0, 0x0024_5158)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AppModule::PollExitRequest(this)` (0x245168): advances the fade
/// (`AppModule_UpdateFade`), then reports 0 (and sets the flag at +0x8 to 1)
/// while the world at gp-0x848 has its flag at +4 set; otherwise 1.
pub fn app_module_poll_exit_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0023_1d18,
            f: ee::s06::f_00231d18,
        }
        .call(c, 0x0024_517c)?;
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V0][0] = world;
        let flag = Addr(world as u32).off(0x4).lw(c);
        c.g[V1][0] = flag;
        c.g[V1][0] = 1;
        if flag != 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            let one = c.g[V1][0];
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, one);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameModule::ResetAllRiders(this)` (0x2451b0): calls the reset slot at
/// +0x9c (through the rider at +0x6c0 of each entry at +0x28) for every
/// entry of the array at this+0xc, whose count is the word at +0x78.
pub fn game_module_reset_all_riders(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let list = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = list;
        let count = Addr(list as u32).off(0x78).lw(c);
        c.g[V1][0] = count;
        c.g[S0][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            loop {
                c.cycles += 14;
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let entry = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
                c.g[A1][0] = entry;
                let rider = Addr(entry as u32).off(0x6c0).lw(c);
                c.g[V1][0] = rider;
                c.g[A0][0] = Addr(rider as u32).off(0x98).lh(c);
                let slot = Addr(rider as u32).off(0x9c).lw(c);
                c.g[V0][0] = slot;
                c.g[A0][0] = sx((entry as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, slot as u32, 0x0024_51fc)?;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
                let count = Addr(c.g[V1][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0024_51d8);
                }
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

/// `cFEStateProgressive::~cFEStateProgressive()` (0x245228): calls the
/// `UIState` destructor (0x39e390) on the same object and returns.
pub fn cfe_state_progressive_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e390, 0x0024_5238)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the callee fakes below run only when it holds `MAGIC`.
    const CTRL: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x0522_0522;
    /// Forced `v0` results of the fakes, one word per fake index.
    const RESULTS: u32 = CTRL + 0x10;

    macro_rules! oc522_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $k:literal;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(CTRL) != MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                    if c.pc == ra {
                        let v = c.bus.read32(RESULTS + 4 * $k);
                        c.g[V0][0] = sx(v);
                    }
                }
            )*
            fn oc522_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev); )*
            }
        };
    }

    oc522_fakes! {
        oc522_f_231cd0 OC522_P_231CD0 0x0023_1cd0 s06 OVERRIDE_00231CD0 f_00231cd0__generated 0;
        oc522_f_22e8b8 OC522_P_22E8B8 0x0022_e8b8 s05 OVERRIDE_0022E8B8 f_0022e8b8__generated 1;
        oc522_f_22e920 OC522_P_22E920 0x0022_e920 s05 OVERRIDE_0022E920 f_0022e920__generated 2;
        oc522_f_3f42a0 OC522_P_3F42A0 0x003f_42a0 s14 OVERRIDE_003F42A0 f_003f42a0__generated 4;
        oc522_f_231d18 OC522_P_231D18 0x0023_1d18 s06 OVERRIDE_00231D18 f_00231d18__generated 5;
        oc522_f_39e390 OC522_P_39E390 0x0039_e390 s13 OVERRIDE_0039E390 f_0039e390__generated 6;
    }

    /// A machine with the fakes gated on and the fake results set.
    fn gated(c: &mut Cpu, seed: u32, salt: u64, results: &[u32]) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(CTRL, MAGIC);
        for (k, &v) in results.iter().enumerate() {
            c.bus.write32(RESULTS + 4 * k as u32, v);
        }
        r
    }

    #[test]
    fn cfe_load_screen_ctor_matches() {
        oc522_install_fakes();
        check(
            ee::s06::f_00243c30__generated,
            cfe_load_screen_ctor,
            200,
            &|c, seed| {
                gated(c, seed, 0x243c30, &[]);
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 5));
            },
        );
    }

    #[test]
    fn gm_handler_call_hook78_matches() {
        check(
            ee::s06::f_00244770__generated,
            gm_handler_call_hook78,
            200,
            &|c, seed| {
                // One call may unwind: the machine decides by seed.
                fakes::machine(c, seed, 0x244770, 1);
                c.g[A0][0] = sx(DATA);
                // Fake targets outside the function table (the miss hook).
                c.bus.write32(DATA + 0x78, fakes::FAKE + 0x10 * (seed % 3));
            },
        );
    }

    #[test]
    fn game_state_finish_clear_flag40_matches() {
        check(
            ee::s06::f_00244880__generated,
            game_state_finish_clear_flag40,
            50,
            &|c, seed| {
                let mut r = Rng(0x244880 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x40 * (seed % 4));
            },
        );
    }

    #[test]
    fn game_module_wait_views_ready_matches() {
        oc522_install_fakes();
        check(
            ee::s06::f_00244888__generated,
            game_module_wait_views_ready,
            200,
            &|c, seed| {
                gated(c, seed, 0x244888, &[0, seed % 2, (seed / 2) % 2]);
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x9c, (seed % 3 == 0) as u32);
            },
        );
    }

    /// The idle path only: `cGameComm_Update` is a resumable dispatcher with
    /// no override slot, so the active path cannot be faked here and its
    /// real body runs on test data (not bounded). That path is checked by
    /// the game's replay gate.
    #[test]
    fn cgame_comm_update_if_active_matches() {
        check(
            ee::s06::f_00245118__generated,
            cgame_comm_update_if_active,
            100,
            &|c, seed| {
                fakes::machine(c, seed, 0x245118, 0);
                c.bus.write32(GAME_GP - 0x204, 0);
            },
        );
    }

    #[test]
    fn netconn_idle_if_active_matches() {
        oc522_install_fakes();
        check(
            ee::s06::f_00245140__generated,
            netconn_idle_if_active,
            100,
            &|c, seed| {
                gated(c, seed, 0x245140, &[0, 0, 0, 0, 0]);
                let conn = if seed % 3 == 0 { 0 } else { DATA + 0x500 };
                c.bus.write32(GAME_GP - 0x238, conn);
            },
        );
    }

    #[test]
    fn app_module_poll_exit_request_matches() {
        oc522_install_fakes();
        check(
            ee::s06::f_00245168__generated,
            app_module_poll_exit_request,
            100,
            &|c, seed| {
                gated(c, seed, 0x245168, &[0, 0, 0, 0, 0, seed % 2]);
                c.g[A0][0] = sx(DATA + 0x400);
                c.bus.write32(GAME_GP - 0x848, DATA + 0x100);
                c.bus.write32(DATA + 0x104, seed % 2);
            },
        );
    }

    /// Builds the array of riders `ResetAllRiders` walks: `count` entries,
    /// each reached through the list at +0x28 and a rider at +0x6c0 whose
    /// reset slot (+0x9c) is a fake target outside the function table.
    fn riders_setup(c: &mut Cpu, seed: u32, count: u32) {
        let mgr = DATA + 0x100;
        let list = DATA + 0x1000;
        c.bus.write32(mgr + 0xc, list);
        c.bus.write32(list + 0x78, count);
        for i in 0..count {
            let entry = DATA + 0x2000 + 0x100 * i;
            let rider = DATA + 0x8000 + 0x100 * i;
            c.bus.write32(list + 0x28 + 4 * i, entry);
            c.bus.write32(entry + 0x6c0, rider);
            c.bus.write16(rider + 0x98, (seed * 8 + i * 4) as u16);
            c.bus
                .write32(rider + 0x9c, fakes::FAKE + 0x10 * ((seed + i) % 3));
        }
    }

    #[test]
    fn game_module_reset_all_riders_matches() {
        check(
            ee::s06::f_002451b0__generated,
            game_module_reset_all_riders,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x2451b0, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                riders_setup(c, seed, seed % 4);
            },
        );
    }

    #[test]
    fn cfe_state_progressive_dtor_matches() {
        oc522_install_fakes();
        check(
            ee::s06::f_00245228__generated,
            cfe_state_progressive_dtor,
            100,
            &|c, seed| {
                gated(c, seed, 0x245228, &[0, 0, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }
}
