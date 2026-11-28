//! Staging module oc1548 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1548`.

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
    (0x002b_3e20, bank_monitor_fade_step, "func_002B3E20"),
    (
        0x002b_6950,
        world_trigger_manager_set_enabled,
        "WorldTriggerManager_SetEnabled",
    ),
    (
        0x002b_9338,
        bank_monitor_init_current_play_params,
        "BankMonitor_InitCurrentPlayParams",
    ),
    (
        0x002b_9e70,
        snd_load_queue_finish_current_ok,
        "SndLoadQueue_FinishCurrentOk",
    ),
    (
        0x002b_9e90,
        snd_load_queue_finish_current_fail,
        "SndLoadQueue_FinishCurrentFail",
    ),
    (
        0x002b_9f08,
        snd_bank_man_is_bank_in_use,
        "SndBankMan_IsBankInUse",
    ),
    (
        0x002b_9f88,
        snd_bank_man_get_bank_handle,
        "SndBankMan_GetBankHandle",
    ),
    (
        0x002b_add0,
        world_sound_collect_subtree,
        "cWorldSound_CollectSubtree",
    ),
];

/// Fade record fields read by `bank_monitor_fade_step` (`self` in a0).
const BMFADE_SONG: u32 = 0x408; // mSongInstance
const BMFADE_STEP: u32 = 0x40c; // volume change per tick (float)
const BMFADE_CUR: u32 = 0x410; // current volume (float, written every tick)
const BMFADE_TARGET: u32 = 0x414; // target volume (int)
/// The song instance's volume byte.
const BMFADE_SONG_VOLUME: u32 = 0x40;

/// `void func_002B3E20(BankMonitorFade* self)`-shaped fade step (0x2b3e20,
/// bankmonitor.cpp): moves the current volume one step towards the target,
/// stores it (as an int) in the song instance and returns true once the
/// target is reached (then the target itself is stored, as a byte). The
/// generated body keeps the float compares as `c_lt_s` + `fcc`, and so does
/// this port.
pub fn bank_monitor_fade_step(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = c.g[A0][0];
    let this = r32(c, V1);
    c.mtc1(0, 3);
    c.lwc1(1, this.wrapping_add(BMFADE_STEP));
    c.lwc1(0, this.wrapping_add(BMFADE_CUR));
    c.c_lt_s(3, 1);
    c.add_s(2, 0, 1);
    let step_not_positive = !c.fcc();
    c.swc1(2, this.wrapping_add(BMFADE_CUR));
    // Still fading (the false return) when the new value has not reached the
    // target yet, on either side of zero.
    let mut fading = false;
    if !step_not_positive {
        c.cycles += 6;
        c.lwc1(0, this.wrapping_add(BMFADE_TARGET));
        c.cvt_s_w(0, 0);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.lwc1(0, this.wrapping_add(BMFADE_CUR));
            fading = true;
        }
    }
    if !fading {
        c.cycles += 4;
        c.c_lt_s(1, 3);
        if !c.fcc() {
            c.g[A0][0] = c.lw(this.wrapping_add(BMFADE_SONG));
        } else {
            c.cycles += 6;
            c.lwc1(0, this.wrapping_add(BMFADE_TARGET));
            c.cvt_s_w(0, 0);
            c.c_lt_s(0, 2);
            if !c.fcc() {
                c.g[A0][0] = c.lw(this.wrapping_add(BMFADE_SONG));
            } else {
                c.cycles += 1;
                c.lwc1(0, this.wrapping_add(BMFADE_CUR));
                fading = true;
            }
        }
    }
    if fading {
        // Return false: the song volume is the current value, truncated.
        c.cycles += 6;
        c.g[V0][0] = 0;
        c.g[A0][0] = c.lw(this.wrapping_add(BMFADE_SONG));
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1);
        let song_volume = r32(c, A0).wrapping_add(BMFADE_SONG_VOLUME);
        let value = c.g[V1][0];
        c.sb(song_volume, value);
        ret(c);
        return;
    }
    // Return true: the target is the final song volume.
    c.cycles += 4;
    c.g[V0][0] = 0x1;
    c.g[V1][0] = c.lb(this.wrapping_add(BMFADE_TARGET));
    let song_volume = r32(c, A0).wrapping_add(BMFADE_SONG_VOLUME);
    let value = c.g[V1][0];
    c.sb(song_volume, value);
    ret(c);
}

/// `void WorldTriggerManager_SetEnabled(WorldTriggerManager* self, int a, int b)`
/// (0x2b6950): stores `a` as the manager's enabled word (+0x334) when `a` or
/// `b` is zero. Otherwise it calls `func_00289400(func_0028B180())`: the
/// first call gets `self` in a0, the second gets the result in a0 and `a1`
/// as the first call left it.
pub fn world_trigger_manager_set_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mut store = c.g[A1][0] == 0;
        if !store {
            c.cycles += 2;
            store = c.g[A2][0] == 0;
        }
        if store {
            c.cycles += 2;
            let value = c.g[A1][0];
            c.sw(r32(c, A0).wrapping_add(0x334), value);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0028_b180, 0x002b_6974)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0028_9400, 0x002b_697c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_InitCurrentPlayParams(BankMonitor* self)` (0x2b9338): the
/// playing-params table at `self+0x20`, indexed by the count at `self+0x1c`
/// (24-byte entries), is passed to 0x3ba020 (the reset).
pub fn bank_monitor_init_current_play_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x18;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x1c));
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x20));
        c.mult(0, 2, 2, 3);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        call_ptr(c, 0x003b_a020, 0x002b_9358)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `SndLoadQueue_FinishCurrentOk` / `..._Fail`: one call of
/// `SndLoadQueue_FinishCurrent(self, flush)` (0x289df0) with `a1 = flush`,
/// returning to `ret`.
fn snd_load_queue_finish_with(c: &mut Cpu, flush: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = flush;
        call_ptr(c, 0x0028_9df0, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndLoadQueue_FinishCurrentOk(SndLoadQueue* a)` (0x2b9e70): finishes
/// the current load without flushing the queue (`flush = 0`).
pub fn snd_load_queue_finish_current_ok(c: &mut Cpu) {
    snd_load_queue_finish_with(c, 0, 0x002b_9e80);
}

/// `void SndLoadQueue_FinishCurrentFail(SndLoadQueue* a)` (0x2b9e90): the
/// same with `flush = 1` (the queue is flushed after the failed load).
pub fn snd_load_queue_finish_current_fail(c: &mut Cpu) {
    snd_load_queue_finish_with(c, 1, 0x002b_9ea0);
}

/// `bool SndBankMan_IsBankInUse(SndBankMan* self, int idx)` (0x2b9f08): true
/// when the bank entry `idx` (0x60 bytes, table at `self+0xacc`) has state 1
/// (the "in use" state is the one that asks the bank manager, through
/// `SND_BankIsInUse`, for its handle). Brackets the work with 0x3b58a0 and
/// 0x3b58d8.
pub fn snd_bank_man_is_bank_in_use(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x003b_58a0, 0x002b_9f2c)?;
        c.g[V1][0] = 0x60;
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xacc));
        c.mult(0, 3, 17, 3);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        c.g[V0][0] = c.lw(r32(c, V1));
        c.g[V0][0] ^= 1;
        let ok = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if !ok {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x002b_9f88,
                f: ee::s08::f_002b9f88,
            }
            .call(c, 0x002b_9f54)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003b_65d0, 0x002b_9f5c)?;
            c.g[V0][0] ^= 1;
            c.g[S2][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 9;
        call_ptr(c, 0x003b_58d8, 0x002b_9f6c)?;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SndBankMan_GetBankHandle(SndBankMan* self, int idx)` (0x2b9f88): the
/// handle stored in the bank entry `idx` (word +4) when its state is 1, else
/// -1. Leaves `a1` at the entry address and `v1` at the table base, as the
/// generated body does.
pub fn snd_bank_man_get_bank_handle(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x60;
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0xacc));
    c.mult(0, 5, 5, 2);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
    c.g[V0][0] = c.lw(r32(c, A1));
    c.g[V0][0] ^= 1;
    let not_ready = c.g[V0][0] != 0;
    c.g[V0][0] = u64::MAX;
    if not_ready {
        c.cycles += 2;
        ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x4));
        ret(c);
    }
}

/// One child step of `world_sound_collect_subtree`: recurse into the child in
/// a0 (when set), then load the next child (`next`, a byte offset in the
/// node) into a0. The callee returns to `ret`.
fn wsc_collect_child(c: &mut Cpu, ret: u32, next: u32) -> Flow {
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x002b_add0,
            f: ee::s08::f_002badd0,
        }
        .call(c, ret)?;
    }
    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(next));
    Ok(())
}

/// `void cWorldSound_CollectSubtree(Node* node, List** out)` (0x2badd0):
/// appends every item of type 5 in `node`'s item list (at +0x28), then
/// recurses into the eight children (pointers at +0x0..+0x1c), in order, into
/// the list `*out` (count at +0, items array at +4).
pub fn world_sound_collect_subtree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x28));
        if c.g[A1][0] == 0 {
            c.g[A0][0] = c.lw(r32(c, S1));
        } else {
            c.cycles += 3;
            c.g[A2][0] = 0x5;
            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x8));
            loop {
                c.cycles += 2;
                if c.g[V0][0] != c.g[A2][0] {
                    c.g[A1][0] = c.lw(r32(c, A1));
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(r32(c, S0));
                    c.g[V1][0] = c.lw(r32(c, A0));
                    c.g[V0][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1));
                    let slot = r32(c, V0).wrapping_add(0x4);
                    let item = c.g[A1][0];
                    c.sw(slot, item);
                    let count = c.g[V1][0];
                    c.sw(r32(c, A0), count);
                    c.g[A1][0] = c.lw(r32(c, A1));
                }
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x8));
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_ae00);
                    }
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(r32(c, S1));
                    break;
                }
            }
        }
        wsc_collect_child(c, 0x002b_ae44, 0x4)?;
        wsc_collect_child(c, 0x002b_ae58, 0x8)?;
        wsc_collect_child(c, 0x002b_ae6c, 0xc)?;
        wsc_collect_child(c, 0x002b_ae80, 0x10)?;
        wsc_collect_child(c, 0x002b_ae94, 0x14)?;
        wsc_collect_child(c, 0x002b_aea8, 0x18)?;
        wsc_collect_child(c, 0x002b_aebc, 0x1c)?;
        // The last child: no next pointer, and s0 is restored after it.
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            Callee {
                addr: 0x002b_add0,
                f: ee::s08::f_002badd0,
            }
            .call(c, 0x002b_aed0)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC1548_CTRL`.
    const OC1548_CTRL: u32 = DATA + 0xf_f000;
    const OC1548_MAGIC: u32 = 0x1548_c0de;

    /// Gated fakes of the callees (hooked once per process; with the gate off
    /// they run the body they replaced).
    macro_rules! oc1548_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1548_CTRL) != OC1548_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc1548_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1548_fakes! {
        oc1548_fake_28b180, OC1548_PREV_28B180, 0x0028_b180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated;
        oc1548_fake_289400, OC1548_PREV_289400, 0x0028_9400, ee::s07::OVERRIDE_00289400, ee::s07::f_00289400__generated;
        oc1548_fake_289df0, OC1548_PREV_289DF0, 0x0028_9df0, ee::s07::OVERRIDE_00289DF0, ee::s07::f_00289df0__generated;
        oc1548_fake_3ba020, OC1548_PREV_3BA020, 0x003b_a020, ee::s13::OVERRIDE_003BA020, ee::s13::f_003ba020__generated;
        oc1548_fake_3b58a0, OC1548_PREV_3B58A0, 0x003b_58a0, ee::s13::OVERRIDE_003B58A0, ee::s13::f_003b58a0__generated;
        oc1548_fake_3b58d8, OC1548_PREV_3B58D8, 0x003b_58d8, ee::s13::OVERRIDE_003B58D8, ee::s13::f_003b58d8__generated;
        oc1548_fake_3b65d0, OC1548_PREV_3B65D0, 0x003b_65d0, ee::s13::OVERRIDE_003B65D0, ee::s13::f_003b65d0__generated;
    }

    /// `bank_monitor_fade_step` against the generated body: fades up and down
    /// through the target, step zero, targets above and below the current
    /// volume, and both returns.
    #[test]
    fn bank_monitor_fade_step_matches() {
        check(
            ee::s08::f_002b3e20__generated,
            bank_monitor_fade_step,
            300,
            &|c, seed| {
                let mut r = Rng(0x002b_3e20 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                let song = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x408, song);
                let step = [0.0f32, 0.5, -0.5, 1.0, -2.0][(seed % 5) as usize];
                c.bus.write32(this + 0x40c, step.to_bits());
                c.bus.write32(this + 0x410, r.f32(4.0));
                c.bus.write32(this + 0x414, (r.next_u32() % 9).wrapping_sub(4));
            },
        );
    }

    /// `world_trigger_manager_set_enabled` against the generated body: the
    /// store path (a or b zero) and the two-call path, with both callees
    /// gated fakes.
    #[test]
    fn world_trigger_manager_set_enabled_matches() {
        oc1548_install_fakes();
        check(
            ee::s08::f_002b6950__generated,
            world_trigger_manager_set_enabled,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_6950, 0);
                c.bus.write32(OC1548_CTRL, OC1548_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.g[A2][0] = if seed % 4 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
            },
        );
    }

    /// `bank_monitor_init_current_play_params` against the generated body: the
    /// count times 24 (signed, from a random int) offsets the table, the
    /// reset callee is a gated fake.
    #[test]
    fn bank_monitor_init_current_play_params_matches() {
        oc1548_install_fakes();
        check(
            ee::s08::f_002b9338__generated,
            bank_monitor_init_current_play_params,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_9338, 0);
                c.bus.write32(OC1548_CTRL, OC1548_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x1c, (r.next_u32() % 64).wrapping_sub(16));
                c.bus.write32(this + 0x20, DATA + 0x2000);
            },
        );
    }

    /// The two `SndLoadQueue_FinishCurrent` wrappers against their generated
    /// bodies: the flush argument and the return address of the one call.
    #[test]
    fn snd_load_queue_finish_current_ok_matches() {
        oc1548_install_fakes();
        check(
            ee::s08::f_002b9e70__generated,
            snd_load_queue_finish_current_ok,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_9e70, 0);
                c.bus.write32(OC1548_CTRL, OC1548_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn snd_load_queue_finish_current_fail_matches() {
        oc1548_install_fakes();
        check(
            ee::s08::f_002b9e90__generated,
            snd_load_queue_finish_current_fail,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_9e90, 0);
                c.bus.write32(OC1548_CTRL, OC1548_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `snd_bank_man_is_bank_in_use` against the generated body: bank entries
    /// in state 1 (the call path) and in other states, both table indices of
    /// the fixture, and the three gated bracket and check callees. The bank
    /// handle lookup is the real `SndBankMan_GetBankHandle` body.
    #[test]
    fn snd_bank_man_is_bank_in_use_matches() {
        oc1548_install_fakes();
        check(
            ee::s08::f_002b9f08__generated,
            snd_bank_man_is_bank_in_use,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_9f08, 0);
                c.bus.write32(OC1548_CTRL, OC1548_MAGIC);
                let this = DATA + 0x1000;
                let table = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(this + 0xacc, table);
                for i in 0..4u32 {
                    let state = match (seed + i) % 3 {
                        0 => 1,
                        1 => 0,
                        _ => 2,
                    };
                    c.bus.write32(table + i * 0x60, state);
                    c.bus.write32(table + i * 0x60 + 4, r.next_u32());
                }
            },
        );
    }

    /// `snd_bank_man_get_bank_handle` against the generated body: state 1
    /// (the -1 result) and other states (the handle), every index of a
    /// six-entry table.
    #[test]
    fn snd_bank_man_get_bank_handle_matches() {
        check(
            ee::s08::f_002b9f88__generated,
            snd_bank_man_get_bank_handle,
            240,
            &|c, seed| {
                let mut r = Rng(0x002b_9f88 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                let table = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 6) as u64;
                c.bus.write32(this + 0xacc, table);
                for i in 0..6u32 {
                    let state = match (seed + i) % 3 {
                        0 => 1,
                        1 => 0,
                        _ => 2,
                    };
                    c.bus.write32(table + i * 0x60, state);
                    c.bus.write32(table + i * 0x60 + 4, r.next_u32());
                }
            },
        );
    }

    /// Node and item layout for `world_sound_collect_subtree`: nodes of
    /// 0x40 bytes (children at +0..+0x1c, item list at +0x28), items of 0x10
    /// bytes (next at +0, type at +8), the list (count at +0, items at +4) and
    /// the out cell holding its address. Children have higher indices, so the
    /// tree is acyclic. With `flat` no node has children.
    fn wsc_collect_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, flat: bool) {
        const NODES: u32 = DATA + 0x1_0000;
        const ITEMS: u32 = DATA + 0x2_0000;
        const LIST: u32 = DATA + 0x3_0000;
        const OUT: u32 = DATA + 0x3_8000;
        let count = 1 + seed % 6;
        for i in 0..count {
            let node = NODES + i * 0x40;
            for k in 0..8u32 {
                let child = if flat || i + 1 == count || r.next_u32() % 3 == 0 {
                    0
                } else {
                    NODES + (i + 1 + r.next_u32() % (count - i - 1)) * 0x40
                };
                c.bus.write32(node + 4 * k, child);
            }
            // Flat fixtures always have a few items, so the loop's back edges
            // (where the events are due) run.
            let items = if flat {
                2 + r.next_u32() % 3
            } else {
                r.next_u32() % 4
            };
            let mut head = 0;
            for k in (0..items).rev() {
                let item = ITEMS + (i * 4 + k) * 0x10;
                let kind = [5, 5, 2, 1][(r.next_u32() % 4) as usize];
                c.bus.write32(item, head);
                c.bus.write32(item + 8, kind);
                head = item;
            }
            c.bus.write32(node + 0x28, head);
        }
        c.bus.write32(OUT, LIST);
        c.bus.write32(LIST, r.next_u32() % 3);
        c.g[A0][0] = sx(NODES);
        c.g[A1][0] = sx(OUT);
    }

    /// `world_sound_collect_subtree` against the generated body on random
    /// trees: the item filter, the list growth, the children (recursing
    /// through the generated body) and the empty-list path.
    #[test]
    fn world_sound_collect_subtree_matches() {
        check(
            ee::s08::f_002badd0__generated,
            world_sound_collect_subtree,
            240,
            &|c, seed| {
                let mut r = Rng(0x002b_add0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                wsc_collect_fixture(c, &mut r, seed, false);
            },
        );
    }

    /// The same body on flat lists with events due at its back edges.
    #[test]
    fn world_sound_collect_subtree_events_match() {
        driven!(gen_collect, ee::s08::f_002badd0__generated);
        driven!(hand_collect, world_sound_collect_subtree);
        check_events(gen_collect, hand_collect, 96, &|c, seed| {
            let mut r = Rng(0x002b_add0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            wsc_collect_fixture(c, &mut r, seed, true);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, 0x002b_add0)
        });
    }
}
