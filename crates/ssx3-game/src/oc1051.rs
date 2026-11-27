//! Staging module oc1051 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1051`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void MusicPlayer_LoopBankIdleCallback(MusicPlayer* p)` (0x2b47c0): under
/// the sound lock, checks the loop bank of the current song. When the bank's
/// state word in the bank table (`sndbankman+0xacc`, 0x60 bytes a bank) reads
/// 1, asks the bank manager for the bank handle and whether that bank is in
/// use; a bank that is not in use counts as idle. Unless the bank is idle,
/// clears the song's loops-active word at +0x4c.
pub fn music_player_loop_bank_idle_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S3][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x408).lw(c);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[S2][0] = Addr(c.g[V0][0] as u32).off(0x34).lw(c);
        c.g[S1][0] = Addr(c.g[V1][0] as u32).lw(c);
        call_ptr(c, 0x003b_58a0, 0x002b_47f4)?; // SND_Lock
        c.g[V1][0] = 0x60;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xacc)).lw(c);
        c.mult(0, V1, S2, V1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] ^= 1;
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] == 0 {
            // The bank's state word is 1: is the bank in use?
            c.cycles += 6;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x002b_9f88, 0x002b_481c)?; // SndBankMan_GetBankHandle
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003b_65d0, 0x002b_4824)?; // SND_BankIsInUse
            c.g[V0][0] ^= 1;
            c.g[S3][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 4;
        call_ptr(c, 0x003b_58d8, 0x002b_4834)?; // SND_Unlock
        let idle = c.g[S3][0] != 0;
        c.g[V0][0] = 0;
        if !idle {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x408).lw(c);
            c.g[V0][0] = 1;
            Addr(c.g[V1][0] as u32).off(0x4c).sw(c, 0);
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

/// `int MusicPlayer_GetSedValue(MusicPlayer* p)` (0x2b48c8): -1 when the
/// player has no `+0x418` word; otherwise the SED value of the current song
/// (`+0x408`), or the song pointer's own result when it is null.
pub fn music_player_get_sed_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x418).lw(c);
        let missing = c.g[V0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if missing {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x408).lw(c);
            let no_song = c.g[A0][0] == 0;
            frame.restore_ra(c, 0);
            if !no_song {
                c.cycles += 4;
                call_ptr(c, 0x002b_48b8, 0x002b_48f0)?; // SongInstance_GetSedValue
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldTriggerManager::~cWorldTriggerManager(int flags)` (0x2b5988):
/// destroys the trigger instances of the manager's array (`+0` pointer, the
/// count in the word before it, 0x70 bytes an instance), frees the array,
/// and deletes the manager itself when bit 0 of `flags` is set.
pub fn world_trigger_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = 0x70;
        if c.g[A1][0] != 0 {
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x70));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_4be0, 0x002b_59d4)?; // WorldTriggerInstance_dtor
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x70));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_59c8);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x002b_59ec)?; // EA_MemFree
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        let delete_self = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if delete_self {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002b_5a00)?; // __builtin_delete
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldTriggerManager_FreeTriggerInfo(TriggerManager* m)` (0x2b5b40):
/// frees the trigger info block at `+0x338` (and the second block at
/// `+0x33c`, when set), then clears the `+0x340` word.
pub fn world_trigger_manager_free_trigger_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x338)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            call_ptr(c, 0x0025_23a8, 0x002b_5b64)?; // MEM_Free
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x33c)).lw(c);
            let empty = c.g[A0][0] == 0;
            Addr((c.g[S0][0] as u32).wrapping_add(0x338)).sw(c, 0);
            if !empty {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x002b_5b78)?; // EA_MemFree
            }
            c.cycles += 2;
            Addr((c.g[S0][0] as u32).wrapping_add(0x340)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldTriggerManager_StopAllWithOwner(TriggerManager* m, int owner)`
/// (0x2b5ed8): walks the 40 trigger slots of the manager (0x70 bytes each).
/// An active slot (state 1) whose owner word at +0xc is `owner` is marked
/// idle and its trigger instance is stopped.
pub fn world_trigger_manager_stop_all_with_owner(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = 0x27;
        c.g[S3][0] = c.g[A1][0];
        c.g[S4][0] = 0x1;
        loop {
            'slot: {
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let active = c.g[V0][0] == c.g[S4][0];
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                if !active {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                if c.g[V0][0] != c.g[S3][0] {
                    break 'slot;
                }
                c.cycles += 4;
                Addr(c.g[V1][0] as u32).sw(c, 0);
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                call_ptr(c, 0x002b_5758, 0x002b_5f38)?; // WorldTriggerInstance_Stop
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5f08);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char* WorldTriggerManager_GetSpecialTriggerName(void)` (0x2b6628): always
/// null; the special-trigger name table is not in this build.
pub fn world_trigger_manager_get_special_trigger_name(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `bool WorldTriggerManager_IsType3(TriggerManager* m, ...)` (0x2b6918):
/// asks `GetTriggerInfo` for the type of trigger 2 and returns whether that
/// type is 3.
pub fn world_trigger_manager_is_type3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[T1][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = c.g[SP][0] | 0x4;
        c.g[T0][0] = c.g[SP][0] | 0x8;
        c.g[T2][0] = 0x2;
        call_ptr(c, 0x002b_63d0, 0x002b_6938)?; // WorldTriggerManager_GetTriggerInfo
        c.g[V0][0] = Addr(sp(c)).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] ^= 0x3;
        c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldSound_UnlinkEmitters(void* manager, Emitters* e)` (0x2b6c20):
/// for each of the `count` emitter records at `e+0x10` (0x18 bytes each),
/// unlinks the ones that have an emitter. Returns 1.
pub fn c_world_sound_unlink_emitters(c: &mut Cpu) {
    guest(c, |c| {
        let a1 = c.g[A1][0] as u32;
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = sx(a1.wrapping_add(0x10));
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(a1.wrapping_add(0xc)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = c.g[A0][0];
        if !none {
            c.cycles += 3;
            c.g[S0][0] = c.g[V1][0];
            c.g[S1][0] = c.g[V0][0];
            loop {
                'emitter: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
                    let empty = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if empty {
                        break 'emitter;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_7410, 0x002b_6c64)?; // cWorldSound_UnlinkEmitter
                }
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                let more = c.g[S1][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_6c50);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_47c0,
        music_player_loop_bank_idle_callback,
        "MusicPlayer_LoopBankIdleCallback",
    ),
    (
        0x002b_48c8,
        music_player_get_sed_value,
        "MusicPlayer_GetSedValue",
    ),
    (
        0x002b_5988,
        world_trigger_manager_dtor,
        "WorldTriggerManager_dtor",
    ),
    (
        0x002b_5b40,
        world_trigger_manager_free_trigger_info,
        "WorldTriggerManager_FreeTriggerInfo",
    ),
    (
        0x002b_5ed8,
        world_trigger_manager_stop_all_with_owner,
        "WorldTriggerManager_StopAllWithOwner",
    ),
    (
        0x002b_6628,
        world_trigger_manager_get_special_trigger_name,
        "WorldTriggerManager_GetSpecialTriggerName",
    ),
    (
        0x002b_6918,
        world_trigger_manager_is_type3,
        "WorldTriggerManager_IsType3",
    ),
    (
        0x002b_6c20,
        c_world_sound_unlink_emitters,
        "cWorldSound_UnlinkEmitters",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes: the fakes only record the
    /// call while [`OC1051_MAGIC`] is in RAM here, otherwise they run the body
    /// they replaced (so other tests see the real callees).
    pub(crate) const OC1051_GATE: u32 = DATA + 0x20_0000;
    pub(crate) const OC1051_MAGIC: u32 = 0x1051_0a0a;

    fn gate_on(c: &mut Cpu) {
        c.bus.write32(OC1051_GATE, OC1051_MAGIC);
    }

    /// A gated fake for one callee: logs the call and returns to `ra` (or
    /// unwinds when the machine asks for it).
    macro_rules! oc1051_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1051_GATE) != OC1051_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                fakes::fake_call(c, $addr)
            }
        };
    }

    oc1051_fake!(
        oc1051_fake_003b58a0,
        OC1051_PREV_003B58A0,
        0x003b_58a0,
        ssx3_ee::s13::f_003b58a0__generated
    );
    oc1051_fake!(
        oc1051_fake_003b58d8,
        OC1051_PREV_003B58D8,
        0x003b_58d8,
        ssx3_ee::s13::f_003b58d8__generated
    );
    oc1051_fake!(
        oc1051_fake_003b65d0,
        OC1051_PREV_003B65D0,
        0x003b_65d0,
        ssx3_ee::s13::f_003b65d0__generated
    );
    oc1051_fake!(
        oc1051_fake_002b9f88,
        OC1051_PREV_002B9F88,
        0x002b_9f88,
        ssx3_ee::s08::f_002b9f88__generated
    );
    oc1051_fake!(
        oc1051_fake_002b4be0,
        OC1051_PREV_002B4BE0,
        0x002b_4be0,
        ssx3_ee::s08::f_002b4be0__generated
    );
    oc1051_fake!(
        oc1051_fake_002b5758,
        OC1051_PREV_002B5758,
        0x002b_5758,
        ssx3_ee::s08::f_002b5758__generated
    );
    oc1051_fake!(
        oc1051_fake_002b7410,
        OC1051_PREV_002B7410,
        0x002b_7410,
        ssx3_ee::s08::f_002b7410__generated
    );
    oc1051_fake!(
        oc1051_fake_00317e98,
        OC1051_PREV_00317E98,
        0x0031_7e98,
        ssx3_ee::s10::f_00317e98__generated
    );
    oc1051_fake!(
        oc1051_fake_00317e50,
        OC1051_PREV_00317E50,
        0x0031_7e50,
        ssx3_ee::s10::f_00317e50__generated
    );
    oc1051_fake!(
        oc1051_fake_002523a8,
        OC1051_PREV_002523A8,
        0x0025_23a8,
        ssx3_ee::s06::f_002523a8__generated
    );

    /// `GetTriggerInfo` also writes the trigger type through its out pointer
    /// (`a2`, the frame slot the caller reads back); the type comes from the
    /// gate block at `OC1051_GATE + 4`.
    pub(crate) static OC1051_PREV_002B63D0: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn oc1051_fake_002b63d0(c: &mut Cpu) {
        if c.bus.read32(OC1051_GATE) != OC1051_MAGIC {
            return fakes::fall_back(
                c,
                &OC1051_PREV_002B63D0,
                ssx3_ee::s08::f_002b63d0__generated,
            );
        }
        let out = c.g[A2][0] as u32;
        let ty = c.bus.read32(OC1051_GATE + 4);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x002b_63d0);
        if c.pc == ra {
            c.bus.write32(out, ty);
        }
    }

    /// Hook every callee fake into its override slot (once per process).
    pub(crate) fn oc1051_install() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003B58A0),
            oc1051_fake_003b58a0,
            &OC1051_PREV_003B58A0,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003B58D8),
            oc1051_fake_003b58d8,
            &OC1051_PREV_003B58D8,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003B65D0),
            oc1051_fake_003b65d0,
            &OC1051_PREV_003B65D0,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s08::OVERRIDE_002B9F88),
            oc1051_fake_002b9f88,
            &OC1051_PREV_002B9F88,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s08::OVERRIDE_002B4BE0),
            oc1051_fake_002b4be0,
            &OC1051_PREV_002B4BE0,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s08::OVERRIDE_002B5758),
            oc1051_fake_002b5758,
            &OC1051_PREV_002B5758,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s08::OVERRIDE_002B7410),
            oc1051_fake_002b7410,
            &OC1051_PREV_002B7410,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s08::OVERRIDE_002B63D0),
            oc1051_fake_002b63d0,
            &OC1051_PREV_002B63D0,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s10::OVERRIDE_00317E98),
            oc1051_fake_00317e98,
            &OC1051_PREV_00317E98,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s10::OVERRIDE_00317E50),
            oc1051_fake_00317e50,
            &OC1051_PREV_00317E50,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s06::OVERRIDE_002523A8),
            oc1051_fake_002523a8,
            &OC1051_PREV_002523A8,
        );
    }

    #[test]
    fn music_player_loop_bank_idle_callback_matches() {
        oc1051_install();
        check(
            ssx3_ee::s08::f_002b47c0__generated,
            music_player_loop_bank_idle_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1051_47c0, 6);
                gate_on(c);
                let this = DATA + 0x1000;
                let mgr = DATA + 0x3000;
                let song = DATA + 0x4000;
                // this->p (p->mgr), mgr->mgr: s1 = **this.
                c.bus.write32(this, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, mgr);
                c.bus.write32(this + 0x408, song);
                // song->+0x34 is the bank index 3: bank table entry at 0x120.
                c.bus.write32(song + 0x34, 3);
                c.bus.write32(mgr + 0xacc, DATA + 0x9000 - 0x120);
                let state = if seed % 3 == 0 { 2 } else { 1 };
                c.bus.write32(DATA + 0x9000, state);
                c.bus.write32(song + 0x4c, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn music_player_get_sed_value_matches() {
        check(
            ssx3_ee::s08::f_002b48c8__generated,
            music_player_get_sed_value,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b48_c8, 2);
                let object = DATA + 0x1000;
                let mode = seed % 3;
                c.bus
                    .write32(object + 0x418, if mode == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(object + 0x408, if mode == 1 { 0 } else { DATA + 0x4000 });
                c.bus.write32(DATA + 0x4000, DATA + 0x5000);
                c.bus.write32(DATA + 0x5098, r.next_u32());
                c.g[A0][0] = sx(object);
            },
        );
    }

    #[test]
    fn world_trigger_manager_dtor_matches() {
        oc1051_install();
        check(
            ssx3_ee::s08::f_002b5988__generated,
            world_trigger_manager_dtor,
            300,
            &|c, seed| {
                let _r = machine(c, seed, 0x2b59_88, 6);
                gate_on(c);
                let this = DATA + 0x1000;
                let array = DATA + 0x2000;
                let count = seed % 4;
                c.bus.write32(this, if seed % 5 == 0 { 0 } else { array });
                c.bus.write32(array - 0x10, count);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn world_trigger_manager_free_trigger_info_matches() {
        oc1051_install();
        check(
            ssx3_ee::s08::f_002b5b40__generated,
            world_trigger_manager_free_trigger_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b5b_40, 4);
                gate_on(c);
                let this = DATA + 0x1000;
                c.bus
                    .write32(this + 0x338, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus
                    .write32(this + 0x33c, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
                c.bus.write32(this + 0x340, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn world_trigger_manager_stop_all_with_owner_matches() {
        oc1051_install();
        check(
            ssx3_ee::s08::f_002b5ed8__generated,
            world_trigger_manager_stop_all_with_owner,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b5e_d8, 8);
                gate_on(c);
                let this = DATA + 0x1000;
                let base = DATA + 0x2000;
                c.bus.write32(this, base);
                for i in 0..40u32 {
                    let e = base + 0x70 * i;
                    c.bus.write32(e, r.next_u32() % 3);
                    c.bus
                        .write32(e + 0xc, if r.next_u32() % 2 == 0 { 5 } else { 7 });
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(5);
            },
        );
    }

    #[test]
    fn world_trigger_manager_get_special_trigger_name_matches() {
        check(
            ssx3_ee::s08::f_002b6628__generated,
            world_trigger_manager_get_special_trigger_name,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b66_28, 0);
            },
        );
    }

    #[test]
    fn world_trigger_manager_is_type3_matches() {
        oc1051_install();
        check(
            ssx3_ee::s08::f_002b6918__generated,
            world_trigger_manager_is_type3,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b69_18, 1);
                gate_on(c);
                c.bus.write32(OC1051_GATE + 4, seed % 5);
            },
        );
    }

    #[test]
    fn c_world_sound_unlink_emitters_matches() {
        oc1051_install();
        check(
            ssx3_ee::s08::f_002b6c20__generated,
            c_world_sound_unlink_emitters,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b6c_20, 4);
                gate_on(c);
                let obj = DATA + 0x1000;
                c.bus.write32(obj + 0xc, seed % 4);
                for i in 0..4u32 {
                    let rec = obj + 0x10 + 0x18 * i;
                    let emitter = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x5000 + 0x40 * i
                    };
                    c.bus.write32(rec + 0xc, emitter);
                }
                c.g[A0][0] = sx(DATA + 0x6000);
                c.g[A1][0] = sx(obj);
            },
        );
    }
}
