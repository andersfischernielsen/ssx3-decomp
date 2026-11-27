//! `src/sound/asyncsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_4 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::sound::bankmonitor::oc767_addiu;
pub(crate) use crate::game::sound::bankmonitor::oc767_addu;
pub(crate) use crate::game::sound::bankmonitor::oc767_lw;
pub(crate) use crate::game::sound::bankmonitor::oc767_sll;
pub(crate) use crate::game::sound::bankmonitor::oc767_sw;
pub(crate) use crate::game::sound::bankmonitor::oc767_swr;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp_3;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_b180,
        ssx_audio_system_get_instance,
        "SSXAudioSystem_GetInstance",
    ),
    (
        0x0028_b1b0,
        ssx_audio_system_is_in_game,
        "SSXAudioSystem_IsInGame",
    ),
    (
        0x0028_b1c0,
        ssx_audio_system_get_app,
        "SSXAudioSystem_GetApp",
    ),
    (
        0x0028_b1e8,
        ssx_audio_system_get_be_library,
        "SSXAudioSystem_GetBELibrary",
    ),
    (
        0x0028_b1f8,
        ssx_audio_system_get_view_count,
        "SSXAudioSystem_GetViewCount",
    ),
    (
        0x0028_b210,
        ssx_audio_system_get_view,
        "SSXAudioSystem_GetView",
    ),
    (0x0028_b240, audio_get_frame_rate, "Audio_GetFrameRate"),
    (0x0028_b248, snd_bank_ctor, "SndBank_ctor"),
    (0x0028_b278, snd_bank_dtor, "SndBank_dtor"),
    (0x0028_b2d0, snd_bank_unload_linked, "SndBank_UnloadLinked"),
    (0x0028_b528, snd_bank_unload, "SndBank_Unload"),
    (0x0028_aaf8, snd_load_queue_update, "SndLoadQueue_Update"),
    (
        0x0028_a298,
        snd_load_queue_start_next,
        "SndLoadQueue_StartNext",
    ),
    (
        0x0028_a728,
        snd_load_queue_cancel_bank,
        "SndLoadQueue_CancelBank",
    ),
    (0x0028_b320, snd_bank_load, "SndBank_Load"),
    (
        0x0028_a1e8,
        snd_load_queue_alloc_async_buffer,
        "SndLoadQueue_AllocAsyncBuffer",
    ),
    (0x0028_a558, snd_load_queue_push, "SndLoadQueue_Push"),
    (
        0x0028_b0f0,
        snd_bank_man_notify_queue_heads,
        "SndBankMan_NotifyQueueHeads",
    ),
];

/// The audio app / view objects: `world + 0x84` is the audio system, its
/// `+0x84` the app, and the app's `+0x10` the view count.
pub(crate) mod audio {

    pub const APP: u32 = 0x84;

    pub const LISTENER_COUNT: u32 = 0x10;
}

// ---------------------------------------------------------------------------
// Callees (same-shard direct calls; cross-shard ones stay `call_ptr`s).

pub(crate) mod callee {

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_UNLOAD_2;

    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
}

/// `bool SSXAudioSystem_IsInGame(void)` (0x0028b1b0): whether the game module
/// pointer (global at `gp-0x848`) has a non-zero app/module word at +0x84.
/// `v1` is left holding the game module pointer.
pub fn ssx_audio_system_is_in_game(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `BELibrary* SSXAudioSystem_GetBELibrary(void)` (0x0028b1e8): the library
/// pointer at game module +0x78. `v1` is left holding the game module pointer.
pub fn ssx_audio_system_get_be_library(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x78).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound bank (a doubly-linked list of banks).

/// `SndBank_ctor(SndBank *self)` (0x28b248): initialise an empty bank (`-1`
/// sentinels at +0x4 and +0xc, everything else zero).
pub fn snd_bank_ctor(c: &mut Cpu) {
    c.cycles += 12;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = c.g[A0][0];
    this.off(0xc).sw(c, u64::MAX);
    this.sw(c, 0);
    this.off(0x4).sw(c, u64::MAX);
    this.off(0x8).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    this.off(0x20).sb(c, 0);
    abi::ret(c);
}

/// `SSXAudioSystem` globals: the app singleton (`gp - 0x848`, the same word as
/// the world pointer) and the audio instance (`gp + 0x410`).
pub(crate) const AUDIO_APP_GP: i32 = -0x848;

pub(crate) const AUDIO_INSTANCE_GP: i32 = 0x410;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `lw` at `addr` (sign-extended into the 64-bit register).
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, addr: u32) -> u64 {
    c.lw(addr)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem singletons.

/// `void* SSXAudioSystem_GetInstance()` (0x28b180): the audio system object
/// stored in the game globals (`gp + 0x410`).
pub fn ssx_audio_system_get_instance(c: &mut Cpu) {
    c.cycles += 2;
    let a = gp(c, AUDIO_INSTANCE_GP);
    c.g[V0][0] = c.lw(a);
    abi::ret(c);
}

/// `void* SSXAudioSystem_GetApp()` (0x28b1c0): the app object (`gp - 0x848`,
/// the same global as the world pointer).
pub fn ssx_audio_system_get_app(c: &mut Cpu) {
    c.cycles += 2;
    let a = gp(c, AUDIO_APP_GP);
    c.g[V0][0] = c.lw(a);
    abi::ret(c);
}
// D_004A1208
pub(crate) const WORLD: i32 = -0x848;

// ---------------------------------------------------------------------------
// Audio system.

/// `int SSXAudioSystem_GetViewCount()` (0x28b1f8): `world->audio->app->count`.
pub fn ssx_audio_system_get_view_count(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr(gp_2(c, WORLD)).lw(c);
    c.g[V0][0] = world;
    let system = Addr(world as u32).off(audio::APP).lw(c);
    c.g[V1][0] = system;
    let app = Addr(system as u32).off(audio::APP).lw(c);
    c.g[A0][0] = app;
    let count = Addr(app as u32).off(audio::LISTENER_COUNT).lw(c);
    c.g[V0][0] = count;
    abi::ret(c);
}

/// `void *SSXAudioSystem_GetView(int i)` (0x0028b210): rider view `i` from the
/// rider table (world → `+0x84` → `+0x84`, entries of 4 bytes at `+0x4`).
pub fn ssx_audio_system_get_view(c: &mut Cpu) {
    c.cycles += 7;
    let world = Addr(gp_3(c, -0x848)).lw(c);
    c.g[A1][0] = world;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = cameras;
    let riders = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = sx((riders as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
    c.pc = ra;
}

/// `void SndBank_UnloadLinked(SndBank* self)` (0x28b2d0): walk the list of
/// banks linked from `+0x18` (terminated by `self`) and unload each.
pub fn snd_bank_unload_linked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let list = c.g[A0][0];
        c.g[S1][0] = list;
        frame.save_ra(c, 0);
        let mut p = Addr(list as u32).off(0x18).lw(c);
        c.g[S0][0] = p;
        let mut only_sentinel = false;
        if p != 0 {
            c.cycles += 2;
            if p == list {
                // the head is already the sentinel: nothing to unload
                only_sentinel = true;
            } else {
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = p;
                    callee::SND_BANK_UNLOAD.call(c, 0x0028_b300)?;
                    c.g[S0][0] = Addr(p as u32).off(0x18).lw(c);
                    p = c.g[S0][0];
                    if p == list {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_b2f8);
                    }
                }
            }
        }
        if !only_sentinel {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_dtor(self, flags)` (0x28b278): unload, unlink and free the
/// bank's memory, then free the bank when asked.
pub fn snd_bank_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0028_b294);
        callee::SND_BANK_UNLOAD_2.call(c, 0x0028_b294)?;
        c.g[RA][0] = sx(0x0028_b29c);
        c.g[A0][0] = c.g[S1][0];
        callee::SND_BANK_UNLINK.call(c, 0x0028_b29c)?;
        c.g[RA][0] = sx(0x0028_b2a4);
        c.g[A0][0] = c.g[S1][0];
        callee::SND_BANK_FREE_MEMORY.call(c, 0x0028_b2a4)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let free = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !free {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028_b2b8);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0028_b2b8)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound.

/// `void SndBank_Unload(SndBank* self)` (0x28b528): unload a sound bank once.
pub fn snd_bank_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, c.g[S0][0] as u32);
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 7;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 4));
            call_ptr(c, 0x003b_6300, 0x0028_b554)?;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = lw_2(c, p_2(c.g[S0][0], 8));
            let has_owner = c.g[V0][0] != 0;
            Addr(p_2(c.g[S0][0], 4)).sw(c, c.g[V1][0]);
            if !has_owner {
                c.cycles += 4;
                c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 0x10));
                call_ptr(c, 0x0025_23a8, 0x0028_b56c)?;
                Addr(p_2(c.g[S0][0], 0x14)).sw(c, 0);
                Addr(p_2(c.g[S0][0], 0x10)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Leaves: constants and field setters.

/// `int Audio_GetFrameRate(void)` (0x28b240): the audio system's fixed 60 Hz
/// frame rate (`0x3c`).
pub fn audio_get_frame_rate(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3c;
    abi::ret(c);
}

/// `void SndLoadQueue::Update()` (0x28aaf8, `src/sound/asyncsys.cpp`): polls
/// the sound load queue's current asynchronous file read.
///
/// Nothing to do unless a load is running (`self+0x4`). The kind of the
/// current request (ring at `+0x1ac`; rings are `{size, data, head, tail}`,
/// the others at `+0x18c` ids, `+0x19c` name slots of 64 bytes at `+0xc`,
/// `+0x1bc` parameters) picks what happens when `AsyncFile_IsDone(handle)`
/// reports completion: kinds 0 and 2 load a sound bank
/// (`AsyncFile_GetResult`, `SND_BankAdd`; a bank that is not resident is
/// removed, copied to memory the owner allocates through vtable `+0x18`
/// (`EA_MemCopy`) and added again; a resident one is moved there with
/// `SND_BankGetSize`/`SND_BankMove`), kind 1 hands the raw data over; the
/// owner is told through vtable `+0x28` (failures with -1 / 2), the four
/// rings advance and the next request starts (`SndLoadQueue_StartNext`).
/// A failed read (`IsDone < 0`) ends the request
/// (`SndLoadQueue_FinishCurrent(self, 0)`).
///
/// The body keeps the original's block structure (every cycle charge, the
/// ring reads written out at each use, the scratch registers each block
/// leaves), with the calls through the call protocol.
pub fn snd_load_queue_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_28b0d4: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28b0d4;
            } else {
                'b_28ab4c: {
                    'b_28ab38: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b8));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1ac));
                        if t {
                            break 'b_28ab38;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S1][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_28ab4c;
                            } else {
                                break 'b_28ab38;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S1][0] = c.lw(c.g[V0][0] as u32);
                    break 'b_28ab4c;
                }
                'b_28b0c8: {
                    'b_28b028: {
                        'b_28b01c: {
                            'b_28aeb8: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x1;
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                                if t {
                                    break 'b_28aeb8;
                                } else {
                                    'b_28ab78: {
                                        'b_28ab70: {
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V0][0] = 0x2;
                                                break 'b_28ab70;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == 0u64;

                                                if t {
                                                    break 'b_28ab78;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_28b0d4;
                                                    } else {
                                                        break 'b_28ab70;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S1][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            break 'b_28b0d4;
                                        } else {
                                            break 'b_28ab78;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    call_ptr(c, 0x003df980, 0x0028ab80)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    if c.g[V1][0] == 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        break 'b_28b0d4;
                                    } else {
                                        'b_28aea8: {
                                            'b_28aea4: {
                                                c.cycles += 2;
                                                let t = (c.g[V1][0] as i64) <= 0;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_28aea4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_28aea8;
                                                    } else {
                                                        'b_28ae1c: {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                                            c.g[A1][0] = c.g[SP][0];
                                                            c.sw(c.g[SP][0] as u32, 0u64);
                                                            c.g[A2][0] = c.g[SP][0] | 0x4;
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                0u64,
                                                            );
                                                            call_ptr(c, 0x003df9d8, 0x0028abb4)?;
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x4));
                                                            let t = (c.g[V1][0] as i64) <= 0;
                                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                                            if t {
                                                                break 'b_28ae1c;
                                                            } else {
                                                                'b_28ae0c: {
                                                                    'b_28acf8: {
                                                                        c.cycles += 5;
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        call_ptr(
                                                                            c, 0x003b6098,
                                                                            0x0028abc8,
                                                                        )?;
                                                                        c.g[V1][0] = 0x7;
                                                                        let t = c.g[V0][0]
                                                                            == c.g[V1][0];
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S0][0] as u32);
                                                                        if t {
                                                                            break 'b_28acf8;
                                                                        } else {
                                                                            'b_28ac14: {
                                                                                'b_28ac00: {
                                                                                    c.cycles += 9;
                                                                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18c));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003b6300,
                                                                                        0x0028abdc,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d4));
                                                                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                                                    c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                                                                                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
                                                                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                                                                                    let t = c.g[A1]
                                                                                        [0]
                                                                                        != c.g[A0]
                                                                                            [0];
                                                                                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                    if t {
                                                                                        break 'b_28ac00;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28ac14;
                                                                                        } else {
                                                                                            break 'b_28ac00;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                c.g[A1][0] = c
                                                                                    .lw(c.g[V0][0]
                                                                                        as u32);
                                                                                break 'b_28ac14;
                                                                            }
                                                                            'b_28ac44: {
                                                                                'b_28ac30: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1bc));
                                                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
                                                                                    let t = c.g[V1]
                                                                                        [0]
                                                                                        != c.g[V0]
                                                                                            [0];
                                                                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    if t {
                                                                                        break 'b_28ac30;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A3][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28ac44;
                                                                                        } else {
                                                                                            break 'b_28ac30;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                c.g[A3][0] = c
                                                                                    .lw(c.g[V0][0]
                                                                                        as u32);
                                                                                break 'b_28ac44;
                                                                            }
                                                                            'b_28aca8: {
                                                                                'b_28ac94: {
                                                                                    c.cycles += 18;
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        c.g[T1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    c.g[A0][0] =
                                                                                        c.g[T0][0];
                                                                                    call_ptr(
                                                                                        c, t,
                                                                                        0x0028ac50,
                                                                                    )?;
                                                                                    c.g[S2][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.g[A1][0] = c
                                                                                        .lw(
                                                                                        c.g[SP][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S2][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003e6574,
                                                                                        0x0028ac64,
                                                                                    )?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[A1][0] =
                                                                                        c.g[S2][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003b6098,
                                                                                        0x0028ac70,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d4));
                                                                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                                                    c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                                                                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c));
                                                                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                                                                                    let t = c.g[A1]
                                                                                        [0]
                                                                                        != c.g[A0]
                                                                                            [0];
                                                                                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                    if t {
                                                                                        break 'b_28ac94;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28aca8;
                                                                                        } else {
                                                                                            break 'b_28ac94;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                c.g[A1][0] = c
                                                                                    .lw(c.g[V0][0]
                                                                                        as u32);
                                                                                break 'b_28aca8;
                                                                            }
                                                                            'b_28acd4: {
                                                                                'b_28acc0: {
                                                                                    c.cycles += 4;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a4));
                                                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                                                                    let t = c.g[V1]
                                                                                        [0]
                                                                                        != c.g[V0]
                                                                                            [0];
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x19c));
                                                                                    if t {
                                                                                        break 'b_28acc0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[V0][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28acd4;
                                                                                        } else {
                                                                                            break 'b_28acc0;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                c.g[V0][0] = c
                                                                                    .lw(c.g[V0][0]
                                                                                        as u32);
                                                                                break 'b_28acd4;
                                                                            }
                                                                            c.cycles += 9;
                                                                            c.g[T0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 6);
                                                                            c.g[A0][0] = c.g[A3][0];
                                                                            c.g[V0][0] = c
                                                                                .lw(c.g[A2][0]
                                                                                    as u32);
                                                                            c.g[T0][0] =
                                                                                sx((c.g[T0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ));
                                                                            c.g[T2][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            c.g[T0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[T0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            c.g[A2][0] = c
                                                                                .lw(c.g[S0][0]
                                                                                    as u32);
                                                                            let t = true;
                                                                            c.g[T1][0] = c.g[S2][0];
                                                                            if t {
                                                                                break 'b_28ae0c;
                                                                            } else {
                                                                                break 'b_28acf8;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_28ad3c: {
                                                                        'b_28ad28: {
                                                                            c.cycles += 10;
                                                                            c.g[S1][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x18c,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x003b6528,
                                                                                0x0028ad00,
                                                                            )?;
                                                                            c.g[S3][0] = c.g[V0][0];
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x194,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1d4,
                                                                                ));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x198,
                                                                                ));
                                                                            c.g[V1][0] = c.lh((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            c.g[T0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1c,
                                                                                    ));
                                                                            let t = c.g[A1][0]
                                                                                != c.g[A0][0];
                                                                            c.g[A2][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[V1][0]
                                                                                            as u32,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28ad28;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[A1][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28ad3c;
                                                                                } else {
                                                                                    break 'b_28ad28;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A1][0] =
                                                                            c.lw(c.g[V0][0] as u32);
                                                                        break 'b_28ad3c;
                                                                    }
                                                                    'b_28ad68: {
                                                                        'b_28ad54: {
                                                                            c.cycles += 4;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c4,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c8,
                                                                                ));
                                                                            let t = c.g[V1][0]
                                                                                != c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1bc,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28ad54;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[A3][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28ad68;
                                                                                } else {
                                                                                    break 'b_28ad54;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A3][0] =
                                                                            c.lw(c.g[V0][0] as u32);
                                                                        break 'b_28ad68;
                                                                    }
                                                                    'b_28adc0: {
                                                                        'b_28adac: {
                                                                            c.cycles += 15;
                                                                            c.g[A0][0] = c.g[A2][0];
                                                                            c.g[V0][0] = c
                                                                                .lw(c.g[T0][0]
                                                                                    as u32);
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[A2][0] = c.g[S3][0];
                                                                            call_ptr(
                                                                                c, t, 0x0028ad78,
                                                                            )?;
                                                                            c.g[S2][0] = c.g[V0][0];
                                                                            c.g[A1][0] = c
                                                                                .lw(c.g[S0][0]
                                                                                    as u32);
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            call_ptr(
                                                                                c, 0x003b6548,
                                                                                0x0028ad88,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1d4,
                                                                                ));
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x194,
                                                                                ));
                                                                            c.g[V0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x28,
                                                                                ));
                                                                            c.g[A2][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2c,
                                                                                    ));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x198,
                                                                                ));
                                                                            let t = c.g[A1][0]
                                                                                != c.g[A0][0];
                                                                            c.g[A3][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28adac;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[A1][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28adc0;
                                                                                } else {
                                                                                    break 'b_28adac;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A1][0] =
                                                                            c.lw(c.g[V0][0] as u32);
                                                                        break 'b_28adc0;
                                                                    }
                                                                    'b_28adec: {
                                                                        'b_28add8: {
                                                                            c.cycles += 4;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1a4,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1a8,
                                                                                ));
                                                                            let t = c.g[V1][0]
                                                                                != c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x19c,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28add8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[V0][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28adec;
                                                                                } else {
                                                                                    break 'b_28add8;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V0][0] =
                                                                            c.lw(c.g[V0][0] as u32);
                                                                        break 'b_28adec;
                                                                    }
                                                                    c.cycles += 8;
                                                                    c.g[T0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 6);
                                                                    c.g[A0][0] = c.g[A3][0];
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[A2][0] as u32);
                                                                    c.g[T0][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0xc));
                                                                    c.g[A2][0] =
                                                                        c.lw(c.g[S0][0] as u32);
                                                                    c.g[T0][0] = sx((c.g[S0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[T0][0] as u32,
                                                                        ));
                                                                    c.g[T1][0] = c.g[S2][0];
                                                                    c.g[T2][0] = c.g[S3][0];
                                                                    break 'b_28ae0c;
                                                                }
                                                                c.cycles += 4;
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(c, t, 0x0028ae14)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x194));
                                                                if t {
                                                                    break 'b_28b028;
                                                                } else {
                                                                    break 'b_28ae1c;
                                                                }
                                                            }
                                                        }
                                                        'b_28ae58: {
                                                            'b_28ae44: {
                                                                c.cycles += 8;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1d4));
                                                                c.g[A2][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x18c));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x194));
                                                                c.g[V0][0] = c
                                                                    .lh((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                c.g[A3][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2c));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x198));
                                                                let t = c.g[A1][0] != c.g[V1][0];
                                                                c.g[A0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                if t {
                                                                    break 'b_28ae44;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[A1][0] = 0xffffffffffffffff;
                                                                    if t {
                                                                        break 'b_28ae58;
                                                                    } else {
                                                                        break 'b_28ae44;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                                            break 'b_28ae58;
                                                        }
                                                        'b_28ae84: {
                                                            'b_28ae70: {
                                                                c.cycles += 4;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1a4));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1a8));
                                                                let t = c.g[V1][0] != c.g[V0][0];
                                                                c.g[A2][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x19c));
                                                                if t {
                                                                    break 'b_28ae70;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V0][0] = 0xffffffffffffffff;
                                                                    if t {
                                                                        break 'b_28ae84;
                                                                    } else {
                                                                        break 'b_28ae70;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                                                            break 'b_28ae84;
                                                        }
                                                        c.cycles += 8;
                                                        c.g[T0][0] = sx((c.g[V0][0] as u32) << 6);
                                                        c.g[A2][0] = 0xffffffffffffffff;
                                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                        c.g[T0][0] =
                                                            sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[T0][0] as u32));
                                                        c.g[A3][0] = 0x2;
                                                        let t = true;
                                                        c.g[T1][0] = 0u64;
                                                        if t {
                                                            break 'b_28b01c;
                                                        } else {
                                                            break 'b_28aea4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A0][0] = c.g[S0][0];
                                            break 'b_28aea8;
                                        }
                                        c.cycles += 4;
                                        c.g[A1][0] = 0u64;
                                        Callee {
                                            addr: 0x00289df0,
                                            f: ee::s07::SndLoadQueue_FinishCurrent,
                                        }
                                        .call(c, 0x0028aeb0)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        if t {
                                            break 'b_28b0d4;
                                        } else {
                                            break 'b_28aeb8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            call_ptr(c, 0x003df980, 0x0028aec0)?;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                break 'b_28b0d4;
                            } else {
                                c.cycles += 2;
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_28b0c8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != c.g[S1][0];
                                    c.g[A1][0] = c.g[SP][0] | 0x8;
                                    if t {
                                        break 'b_28b0c8;
                                    } else {
                                        'b_28af98: {
                                            c.cycles += 8;
                                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                            c.g[A2][0] = c.g[SP][0] | 0xc;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                            call_ptr(c, 0x003df9d8, 0x0028aeec)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            let t = (c.g[V1][0] as i64) <= 0;
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                            if t {
                                                break 'b_28af98;
                                            } else {
                                                'b_28af3c: {
                                                    'b_28af28: {
                                                        c.cycles += 10;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x003b6098, 0x0028af00)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1d4));
                                                        c.g[A2][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18c));
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x194));
                                                        c.g[V1][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        c.g[T3][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x2c));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x198));
                                                        let t = c.g[A1][0] != c.g[A0][0];
                                                        c.g[A3][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[V1][0] as u32));
                                                        if t {
                                                            break 'b_28af28;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = 0xffffffffffffffff;
                                                            if t {
                                                                break 'b_28af3c;
                                                            } else {
                                                                break 'b_28af28;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                                    break 'b_28af3c;
                                                }
                                                'b_28af68: {
                                                    'b_28af54: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1a8));
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        c.g[A0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x19c));
                                                        if t {
                                                            break 'b_28af54;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = 0xffffffffffffffff;
                                                            if t {
                                                                break 'b_28af68;
                                                            } else {
                                                                break 'b_28af54;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                                                    break 'b_28af68;
                                                }
                                                c.cycles += 12;
                                                c.g[T0][0] = sx((c.g[V0][0] as u32) << 6);
                                                c.g[A0][0] = c.g[A3][0];
                                                c.g[T0][0] =
                                                    sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                                c.g[T1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.g[T2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[T0][0] as u32));
                                                c.g[V0][0] = c.lw(c.g[T3][0] as u32);
                                                c.g[A3][0] = 0u64;
                                                let t = c.g[V0][0] as u32;
                                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                call_ptr(c, t, 0x0028af90)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                if t {
                                                    break 'b_28b028;
                                                } else {
                                                    break 'b_28af98;
                                                }
                                            }
                                        }
                                        'b_28afd4: {
                                            'b_28afc0: {
                                                c.cycles += 8;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1d4));
                                                c.g[A2][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x18c));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                c.g[V0][0] =
                                                    c.lh((c.g[A0][0] as u32).wrapping_add(0x28));
                                                c.g[A3][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2c));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                                                let t = c.g[A1][0] != c.g[V1][0];
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_28afc0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A1][0] = 0xffffffffffffffff;
                                                    if t {
                                                        break 'b_28afd4;
                                                    } else {
                                                        break 'b_28afc0;
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                            break 'b_28afd4;
                                        }
                                        'b_28b000: {
                                            'b_28afec: {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1a4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[A2][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x19c));
                                                if t {
                                                    break 'b_28afec;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = 0xffffffffffffffff;
                                                    if t {
                                                        break 'b_28b000;
                                                    } else {
                                                        break 'b_28afec;
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                                            break 'b_28b000;
                                        }
                                        c.cycles += 7;
                                        c.g[T0][0] = sx((c.g[V0][0] as u32) << 6);
                                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[T0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        c.g[A3][0] = 0x2;
                                        break 'b_28b01c;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        let t = c.g[V0][0] as u32;
                        c.g[T2][0] = 0u64;
                        call_ptr(c, t, 0x0028b024)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                        break 'b_28b028;
                    }
                    'b_28b04c: {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                        if t {
                            break 'b_28b04c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x198), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x198), 0u64);
                                break 'b_28b04c;
                            } else {
                                break 'b_28b04c;
                            }
                        }
                    }
                    'b_28b070: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_28b070;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x19c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), 0u64);
                                break 'b_28b070;
                            } else {
                                break 'b_28b070;
                            }
                        }
                    }
                    'b_28b094: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b8));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_28b094;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ac));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1b8), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1b8), 0u64);
                                break 'b_28b094;
                            } else {
                                break 'b_28b094;
                            }
                        }
                    }
                    'b_28b0b8: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_28b0b8;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1bc));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), 0u64);
                                break 'b_28b0b8;
                            } else {
                                break 'b_28b0b8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0028a298,
                        f: ee::s07::SndLoadQueue_StartNext,
                    }
                    .call(c, 0x0028b0c0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_28b0d4;
                    } else {
                        break 'b_28b0c8;
                    }
                }
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x00289df0,
                    f: ee::s07::SndLoadQueue_FinishCurrent,
                }
                .call(c, 0x0028b0d0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28b0d4;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// The tail element of a circular queue (`Array[Tail]`): the queue header is
/// at the address in `q` (`Array` at +4, `Tail` at +0xc), the element lands in
/// `dst`. The queue is known to be non-empty here (5 cycles).
pub(crate) fn oc738_queue_peek(c: &mut Cpu, q: usize, dst: usize) {
    c.cycles += 5;
    let base = c.g[q][0] as u32;
    c.g[V0][0] = Addr(base.wrapping_add(0xc)).lw(c);
    c.g[V1][0] = Addr(base.wrapping_add(0x4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[dst][0] = Addr(c.g[V0][0] as u32).lw(c);
}

/// Loads the head queue's file into the main buffer: the file name index
/// (`mFilenameIndex` queue at +0x19c, -1 when empty) picks a 64-byte name
/// slot at +0xc; `ASYNCFILE_loadat(name, buffer (+0x1cc), size (+0x1d0))`
/// and its handle goes to `self->mHandle` (+0). The state machine's kind 0
/// and kind 2 only differ in the return address.
pub(crate) fn oc738_load_at_buffer(c: &mut Cpu, ret: u32) -> Flow {
    let this = c.g[S0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x1a8)).lw(c);
    let not_empty = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = sx(this.wrapping_add(0x19c));
    if not_empty {
        oc738_queue_peek(c, A0, V0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
    }
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[V0][0] as u32) << 6);
    c.g[A1][0] = Addr(this.wrapping_add(0x1cc)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = Addr(this.wrapping_add(0x1d0)).lw(c);
    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, 0x003d_f748, ret)?; // ASYNCFILE_loadat
    c.sw(this, c.g[V0][0]);
    Ok(())
}

/// Kind 1: the buffer is allocated through the object's virtual methods
/// (vtable at +0x1d4: entry 3 allocates `(handle, -1, flags)`, entry 4 gives
/// the size of `handle`); with a buffer the file is read into it
/// (`ASYNCFILE_loadat`), else it is loaded by `ASYNCFILE_load(name, flags)`.
pub(crate) fn oc738_load_allocated(c: &mut Cpu) -> Flow {
    let this = c.g[S0][0] as u32;
    // The file name (index queue +0x19c, -1 when empty).
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x1a4)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1a8)).lw(c);
    let not_empty = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = sx(this.wrapping_add(0x19c));
    if not_empty {
        oc738_queue_peek(c, A0, V0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
    }
    // The allocation handle (user handle queue, s2) and the vtable entry 3.
    c.cycles += 10;
    c.g[V1][0] = Addr(this.wrapping_add(0x1d4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = Addr(this.wrapping_add(0x194)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lh(c);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = Addr(this.wrapping_add(0x198)).lw(c);
    c.g[S4][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    let has_handle = c.g[A1][0] != c.g[V1][0];
    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
    if has_handle {
        oc738_queue_peek(c, S2, A1);
    } else {
        c.cycles += 2;
        c.g[A1][0] = u64::MAX;
    }
    // The flags (main memory flags queue +0x1bc, s1).
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x1c4)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1c8)).lw(c);
    let has_flags = c.g[V1][0] != c.g[V0][0];
    c.g[S1][0] = sx(this.wrapping_add(0x1bc));
    if has_flags {
        oc738_queue_peek(c, S1, A3);
    } else {
        c.cycles += 2;
        c.g[A3][0] = u64::MAX;
    }
    // buffer = vt[3].fn(this + delta, handle, -1, flags)
    c.cycles += 11;
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
    let alloc = c.g[V0][0] as u32;
    c.g[A2][0] = u64::MAX;
    call_ptr(c, alloc, 0x0028_a49c)?;
    c.g[S3][0] = c.g[V0][0];
    // size = vt[4].fn(this + delta, handle)
    c.g[A1][0] = Addr(this.wrapping_add(0x194)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1d4)).lw(c);
    c.g[A0][0] = Addr(this.wrapping_add(0x198)).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24));
    let has_handle = c.g[A1][0] != c.g[A0][0];
    c.g[A2][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
    if has_handle {
        oc738_queue_peek(c, S2, A1);
    } else {
        c.cycles += 2;
        c.g[A1][0] = u64::MAX;
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
    let size_fn = c.g[V0][0] as u32;
    c.g[A0][0] = c.g[A2][0];
    call_ptr(c, size_fn, 0x0028_a4e4)?;
    let no_buffer = c.g[S3][0] == 0;
    c.g[A0][0] = c.g[S4][0];
    if !no_buffer {
        c.cycles += 5;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x003d_f748, 0x0028_a4f8)?; // ASYNCFILE_loadat(name, buffer, size)
        c.sw(this, c.g[V0][0]);
        return Ok(());
    }
    // No buffer: ASYNCFILE_load(name, flags).
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x1c4)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1c8)).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
        c.cycles += 4;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
    } else {
        c.cycles += 2;
        c.g[A1][0] = u64::MAX;
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, 0x003d_f690, 0x0028_a530)?; // ASYNCFILE_load(name, flags)
    c.sw(this, c.g[V0][0]);
    Ok(())
}

/// `void SndLoadQueue_StartNext(SndLoadQueue* self)` (0x28a298), the C++
/// `cAsyncSys_ASYNCSYS_StartLoad`: when no load is in flight (+4) and the
/// user-handle queue (+0x18c) is not empty, marks the system busy, notifies
/// through the vtable (+0x1d4, entry 2) with the next handle and starts the
/// asynchronous load for the kind at the tail of the kind queue (+0x1ac):
/// 0 and 2 load a named file into the main buffer, 1 allocates a buffer
/// through the vtable first. The file handle is stored at `self+0`.
pub fn snd_load_queue_start_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x4)).lw(c);
        'exit: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x50);
                break 'exit;
            }
            'done: {
                c.cycles += 4;
                c.g[V1][0] = Addr(this.wrapping_add(0x194)).lw(c);
                c.g[V0][0] = Addr(this.wrapping_add(0x198)).lw(c);
                let empty = c.g[V1][0] == c.g[V0][0];
                c.g[S2][0] = sx(this.wrapping_add(0x18c));
                if empty {
                    break 'done;
                }
                // Busy; notify(handle at the tail of the user-handle queue).
                c.cycles += 16;
                c.g[V1][0] = 0x1;
                c.g[A1][0] = Addr(this.wrapping_add(0x1d4)).lw(c);
                c.sw(this.wrapping_add(0x4), c.g[V1][0]);
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lh(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let notify = c.g[A2][0] as u32;
                c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, notify, 0x0028_a304)?;
                // The kind at the tail of the kind queue (-1 when empty).
                c.g[V1][0] = Addr(this.wrapping_add(0x1b4)).lw(c);
                c.g[V0][0] = Addr(this.wrapping_add(0x1b8)).lw(c);
                let has_kind = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx(this.wrapping_add(0x1ac));
                if has_kind {
                    oc738_queue_peek(c, A0, V1);
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = u64::MAX;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let kind1 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                if kind1 {
                    oc738_load_allocated(c)?;
                    break 'done;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // Kind >= 2: only 2 loads.
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x50);
                        break 'exit;
                    }
                    c.g[V1][0] = Addr(this.wrapping_add(0x1a4)).lw(c);
                    oc738_load_at_buffer(c, 0x0028_a3ec)?;
                } else {
                    // Kind < 2: only 0 loads.
                    c.cycles += 2;
                    if c.g[V1][0] != 0 {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x50);
                        break 'exit;
                    }
                    c.g[V1][0] = Addr(this.wrapping_add(0x1a4)).lw(c);
                    oc738_load_at_buffer(c, 0x0028_a3a4)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SndLoadQueue_CancelBank(Queue* q, int bank)` (the C++ `cAsyncSys_ASYNCSYS_Cancel`): removes `bank` from the load queue's rings
/// (+0x18c, +0x19c, +0x1ac, +0x1bc, each with head/tail/count words), compacting the rest;
/// finishes the current load when it is the cancelled one and starts the next.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn snd_load_queue_cancel_bank(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0028a728);
    'b_28aaec: {
        'b_28a78c: {
            'b_28a788: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] == 0u64;
                c.g[T3][0] = c.g[A1][0];
                if t {
                    break 'b_28a788;
                } else {
                    'b_28a76c: {
                        'b_28a758: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18c));
                            if t {
                                break 'b_28a758;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_28a76c;
                                } else {
                                    break 'b_28a758;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                        break 'b_28a76c;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[T3][0] {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                        break 'b_28a78c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[A2][0];
                        c.g[RA][0] = sx(0x0028a780);
                        c.g[A1][0] = 0u64;
                        c.pc = 0x00289df0;
                        ee::s07::f_00289df0(c);
                        if c.pc != 0x0028a780 {
                            return;
                        }
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_28aaec;
                        } else {
                            break 'b_28a788;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
            break 'b_28a78c;
        }
        'b_28a7a4: {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            if c.g[V0][0] == 0u64 {
                break 'b_28a7a4;
            } else {
                break 'b_28a7a4;
            }
        }
        'b_28aad8: {
            c.cycles += 4;
            c.div(0, 3, 2);
            c.g[V1][0] = c.hi[0];
            if (c.g[V1][0] as i64) <= 0 {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                break 'b_28aad8;
            } else {
                c.cycles += 3;
                c.g[T2][0] = c.g[V1][0];
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18c));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                'l_28a7c0: loop {
                    'b_28a800: {
                        'b_28a7d4: {
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
                            if c.g[V0][0] != c.g[A1][0] {
                                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
                                break 'b_28a7d4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[A1][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_28a800;
                                } else {
                                    break 'b_28a7d4;
                                }
                            }
                        }
                        'b_28a7fc: {
                            c.cycles += 9;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[A2][0] as u32).wrapping_add(0x198), c.g[A1][0]);
                            if t {
                                break 'b_28a7fc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x198), 0u64);
                                break 'b_28a7fc;
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_28a800;
                    }
                    'b_28aacc: {
                        'b_28aac8: {
                            'b_28a87c: {
                                c.cycles += 2;
                                if c.g[A1][0] != c.g[T3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                                    break 'b_28a87c;
                                } else {
                                    'b_28a82c: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a8));
                                        let t = c.g[V0][0] == c.g[A0][0];
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_28a82c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                            c.sw(
                                                (c.g[A2][0] as u32).wrapping_add(0x1a8),
                                                c.g[V0][0],
                                            );
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.sw((c.g[A2][0] as u32).wrapping_add(0x1a8), 0u64);
                                                break 'b_28a82c;
                                            } else {
                                                break 'b_28a82c;
                                            }
                                        }
                                    }
                                    'b_28a850: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b8));
                                        let t = c.g[V0][0] == c.g[A0][0];
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_28a850;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                            c.sw(
                                                (c.g[A2][0] as u32).wrapping_add(0x1b8),
                                                c.g[V0][0],
                                            );
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.sw((c.g[A2][0] as u32).wrapping_add(0x1b8), 0u64);
                                                break 'b_28a850;
                                            } else {
                                                break 'b_28a850;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c8));
                                    let t = c.g[V0][0] == c.g[A0][0];
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_28aac8;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), c.g[V0][0]);
                                        c.g[V0][0] =
                                            ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), 0u64);
                                            break 'b_28aac8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                                            if t {
                                                break 'b_28aacc;
                                            } else {
                                                break 'b_28a87c;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_28a890: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28a890;
                                } else {
                                    break 'b_28a890;
                                }
                            }
                            'b_28a8d4: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                    break 'b_28a8d4;
                                } else {
                                    'b_28a8d0: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x194), c.g[V1][0]);
                                        if t {
                                            break 'b_28a8d0;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x194), 0u64);
                                            break 'b_28a8d0;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                    break 'b_28a8d4;
                                }
                            }
                            'b_28a928: {
                                'b_28a8f0: {
                                    c.cycles += 5;
                                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x19c));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a8));
                                    c.g[A0][0] = c.g[V1][0];
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_28a8f0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[T0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_28a928;
                                        } else {
                                            break 'b_28a8f0;
                                        }
                                    }
                                }
                                'b_28a91c: {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1a8), c.g[A1][0]);
                                    if t {
                                        break 'b_28a91c;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1a8), 0u64);
                                        break 'b_28a91c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a8));
                                break 'b_28a928;
                            }
                            'b_28a93c: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28a93c;
                                } else {
                                    break 'b_28a93c;
                                }
                            }
                            'b_28a97c: {
                                c.cycles += 3;
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[A1][0] {
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                    break 'b_28a97c;
                                } else {
                                    'b_28a978: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1a4), c.g[V1][0]);
                                        if t {
                                            break 'b_28a978;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x1a4), 0u64);
                                            break 'b_28a978;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                    break 'b_28a97c;
                                }
                            }
                            'b_28a9d0: {
                                'b_28a998: {
                                    c.cycles += 5;
                                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b8));
                                    c.g[A0][0] = c.g[V1][0];
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_28a998;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[T0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_28a9d0;
                                        } else {
                                            break 'b_28a998;
                                        }
                                    }
                                }
                                'b_28a9c4: {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1b8), c.g[A1][0]);
                                    if t {
                                        break 'b_28a9c4;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1b8), 0u64);
                                        break 'b_28a9c4;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b8));
                                break 'b_28a9d0;
                            }
                            'b_28a9e4: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28a9e4;
                                } else {
                                    break 'b_28a9e4;
                                }
                            }
                            'b_28aa24: {
                                c.cycles += 3;
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[A1][0] {
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                    break 'b_28aa24;
                                } else {
                                    'b_28aa20: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1b4), c.g[V1][0]);
                                        if t {
                                            break 'b_28aa20;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x1b4), 0u64);
                                            break 'b_28aa20;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                    break 'b_28aa24;
                                }
                            }
                            'b_28aa78: {
                                'b_28aa40: {
                                    c.cycles += 5;
                                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c8));
                                    c.g[A0][0] = c.g[V1][0];
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_28aa40;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[T0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_28aa78;
                                        } else {
                                            break 'b_28aa40;
                                        }
                                    }
                                }
                                'b_28aa6c: {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), c.g[A1][0]);
                                    if t {
                                        break 'b_28aa6c;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), 0u64);
                                        break 'b_28aa6c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c8));
                                break 'b_28aa78;
                            }
                            'b_28aa8c: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28aa8c;
                                } else {
                                    break 'b_28aa8c;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.hi[0];
                            if c.g[A0][0] == c.g[A1][0] {
                                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                                break 'b_28aacc;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x1c4), c.g[V1][0]);
                                if t {
                                    break 'b_28aac8;
                                } else {
                                    c.cycles += 1;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1c4), 0u64);
                                    break 'b_28aac8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                        break 'b_28aacc;
                    }
                    c.cycles += 2;
                    if c.g[T2][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                        if event_due(c) {
                            deliver_event_here(c, 0x0028a7c0);
                        }
                        continue 'l_28a7c0;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        break 'b_28aad8;
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        if t {
            break 'b_28aaec;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028aae8);
            c.g[A0][0] = c.g[A2][0];
            c.pc = 0x0028a298;
            ee::s07::f_0028a298(c);
            if c.pc != 0x0028aae8 {
                return;
            }
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_28aaec;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `void SndBank_Load(Bank* self, char* file, int align)` (the C++ `cBankInstance_Load`): unloads the bank, loads its data from disc (by allocation location: heap, preloaded buffer or stream memory), registers it with the sound driver and copies the header, then records the filename.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn snd_bank_load(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0028b320);
    'b_28b508: {
        'b_28b3ec: {
            'b_28b3c4: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x0028b350);
                c.g[S1][0] = 0u64;
                c.pc = 0x0028b528;
                ee::s07::f_0028b528(c);
                if c.pc != 0x0028b350 {
                    return;
                }
                c.g[RA][0] = sx(0x0028b358);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x0028b2d0;
                ee::s07::f_0028b2d0(c);
                if c.pc != 0x0028b358 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.sw((c.g[SP][0] as u32), 0u64);
                if t {
                    break 'b_28b3c4;
                } else {
                    'b_28b3a8: {
                        'b_28b3a0: {
                            'b_28b394: {
                                'b_28b384: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_28b384;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[A2][0] = c.g[S2][0];
                                        if t {
                                            break 'b_28b394;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if t {
                                                break 'b_28b3ec;
                                            } else {
                                                break 'b_28b384;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[A0][0] = c.g[S3][0];
                                if t {
                                    break 'b_28b3a0;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_28b3ec;
                                    } else {
                                        break 'b_28b394;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S3][0];
                            let t = true;
                            c.g[A1][0] = c.g[SP][0];
                            if t {
                                break 'b_28b3a8;
                            } else {
                                break 'b_28b3a0;
                            }
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = 0x100;
                        break 'b_28b3a8;
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0028b3b0);
                    c.pc = 0x003e1a70;
                    call_indirect(c, 0x003e1a70);
                    if c.pc != 0x0028b3b0 {
                        return;
                    }
                    c.g[S1][0] = c.g[V0][0];
                    if c.g[S1][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_28b508;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_28b3ec;
                        } else {
                            break 'b_28b3c4;
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0028b3d4);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.pc = 0x003e1bb8;
            call_indirect(c, 0x003e1bb8);
            if c.pc != 0x0028b3d4 {
                return;
            }
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_28b508;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                break 'b_28b3ec;
            }
        }
        'b_28b420: {
            c.cycles += 3;
            c.g[S4][0] = 0x2;
            if c.g[V0][0] != c.g[S4][0] {
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_28b420;
            } else {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x524));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0028b41c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.pc = t;
                call_indirect(c, t);
                if c.pc != 0x0028b41c {
                    return;
                }
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_28b420;
            }
        }
        'b_28b4d8: {
            'b_28b4c0: {
                'b_28b4b8: {
                    'b_28b4b4: {
                        'b_28b4a0: {
                            'b_28b448: {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x0028b428);
                                c.g[A1][0] = c.g[S1][0];
                                c.pc = 0x003b6098;
                                call_indirect(c, 0x003b6098);
                                if c.pc != 0x0028b428 {
                                    return;
                                }
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = 0x7;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x8;
                                if t {
                                    break 'b_28b448;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_28b4a0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_28b4c0;
                                        } else {
                                            break 'b_28b448;
                                        }
                                    }
                                }
                            }
                            'b_28b474: {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x0028b450);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.pc = 0x003b6528;
                                call_indirect(c, 0x003b6528);
                                if c.pc != 0x0028b450 {
                                    return;
                                }
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                let t = c.g[V0][0] != c.g[S4][0];
                                c.g[A2][0] = 0u64;
                                if t {
                                    break 'b_28b474;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x0028b46c);
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.pc = 0x003b6548;
                                    call_indirect(c, 0x003b6548);
                                    if c.pc != 0x0028b46c {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_28b4b8;
                                    } else {
                                        break 'b_28b474;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x0028b480);
                            c.g[A1][0] = c.g[S2][0];
                            c.pc = 0x00252fa0;
                            call_indirect(c, 0x00252fa0);
                            if c.pc != 0x0028b480 {
                                return;
                            }
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0028b490);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.pc = 0x003b6548;
                            call_indirect(c, 0x003b6548);
                            if c.pc != 0x0028b490 {
                                return;
                            }
                            c.g[RA][0] = sx(0x0028b498);
                            c.g[A0][0] = c.g[S1][0];
                            c.pc = 0x002523a8;
                            call_indirect(c, 0x002523a8);
                            if c.pc != 0x0028b498 {
                                return;
                            }
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S2][0]);
                            if t {
                                break 'b_28b4b4;
                            } else {
                                break 'b_28b4a0;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_28b4b8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            break 'b_28b4b4;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    break 'b_28b4b8;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                if t {
                    break 'b_28b4d8;
                } else {
                    break 'b_28b4c0;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S0][0] as u32), 0u64);
                break 'b_28b4d8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0028b4d4);
                c.g[A0][0] = c.g[S1][0];
                c.pc = 0x002523a8;
                call_indirect(c, 0x002523a8);
                if c.pc != 0x0028b4d4 {
                    return;
                }
                c.sw((c.g[S0][0] as u32), 0u64);
                break 'b_28b4d8;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x524));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0028b4f8);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x0028b4f8 {
            return;
        }
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = sx(0x0028b504);
        c.g[A1][0] = c.g[S3][0];
        c.pc = 0x004166f4;
        call_indirect(c, 0x004166f4);
        if c.pc != 0x0028b504 {
            return;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        break 'b_28b508;
    }
    c.cycles += 7;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t;
    return;
}

/// `SndLoadQueue_StartNext` (0x28a298).
pub(crate) const OC767_SND_LOAD_QUEUE_START_NEXT: Callee = Callee {
    addr: 0x0028_a298,
    f: ee::s07::f_0028a298,
};

/// Offsets in the async load system (`sAsync28A558`): the next free name
/// slot, four ring queues (`size`, `data`, `head`, `tail`) for bank ids,
/// slots, memory types and memory flags, and the 64-byte name slots.
pub(crate) const SND_LOAD_SLOT: u32 = 0x08;

pub(crate) const SND_LOAD_NAMES: u32 = 0x0c;

pub(crate) const SND_LOAD_Q_BANK: u32 = 0x18c;

pub(crate) const SND_LOAD_Q_SLOT: u32 = 0x19c;

pub(crate) const SND_LOAD_Q_TYPE: u32 = 0x1ac;

pub(crate) const SND_LOAD_Q_FLAGS: u32 = 0x1bc;

/// The ring-queue push of `SndLoadQueue_Push` once `hi` holds
/// `(head + 1) % size`: when that is not the tail (room left) store `value`
/// at `data[head]` and advance `head`, wrapping to 0 at `size`. `base` holds
/// the queue's address; the generated code loads the tail into `tail_reg`,
/// the remainder into `hi_reg`, and leaves `v0`/`v1` as scratch.
pub(crate) fn oc767_ring_push(
    c: &mut Cpu,
    base: usize,
    tail_reg: usize,
    hi_reg: usize,
    value: usize,
) {
    c.cycles += 4;
    c.g[tail_reg][0] = oc767_lw(c, base, 0xc);
    c.g[hi_reg][0] = c.hi[0];
    if c.g[hi_reg][0] == c.g[tail_reg][0] {
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = oc767_lw(c, base, 8);
    c.g[V1][0] = oc767_lw(c, base, 4);
    oc767_sll(c, V0, V0, 2);
    oc767_addu(c, V0, V0, V1);
    let v = c.g[value][0];
    oc767_sw(c, V0, 0, v);
    c.g[V1][0] = oc767_lw(c, base, 8);
    c.g[V0][0] = oc767_lw(c, base, 0);
    oc767_addiu(c, V1, V1, 1);
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let more = c.g[V0][0] != 0;
    let v = c.g[V1][0];
    oc767_sw(c, base, 8, v);
    if !more {
        c.cycles += 1;
        oc767_sw(c, base, 8, 0);
    }
    c.cycles += 1;
}

/// `void SndLoadQueue_Push(AsyncSys* self, int bankId, char* name, int
/// memType, int memFlags)` (0x28a558, `cAsyncSys::ASYNCSYS_QueueLoad`): queues
/// a bank load (bank id, memory type, memory flags and the next slot number
/// in four ring queues that drop the entry when full), copies the name into
/// the current 64-byte name slot, advances the slot (0..5), and starts the next
/// load.
pub fn snd_load_queue_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        oc767_addiu(c, T1, S0, SND_LOAD_Q_BANK);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_BANK + 8);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_Q_BANK);
        oc767_addiu(c, V0, V0, 1);
        c.div(0, V0, V1);

        // bank id
        oc767_ring_push(c, T1, V0, A0, A1);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_Q_TYPE + 8);

        // memory type
        c.cycles += 6;
        oc767_addiu(c, T1, S0, SND_LOAD_Q_TYPE);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_TYPE);
        oc767_addiu(c, V1, V1, 1);
        c.div(0, V1, V0);
        oc767_ring_push(c, T1, V1, A0, A3);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_Q_FLAGS + 8);

        // memory flags
        c.cycles += 6;
        oc767_addiu(c, A3, S0, SND_LOAD_Q_FLAGS);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_FLAGS);
        oc767_addiu(c, V1, V1, 1);
        c.div(0, V1, V0);
        oc767_ring_push(c, A3, V1, A0, T0);
        c.g[A0][0] = oc767_lw(c, S0, SND_LOAD_SLOT);

        // The name goes into names[slot] (64 bytes each).
        c.cycles += 16;
        c.g[A1][0] = c.g[A2][0];
        oc767_sll(c, A0, A0, 6);
        oc767_addiu(c, A0, A0, SND_LOAD_NAMES);
        oc767_addu(c, A0, S0, A0);
        call_ptr(c, 0x0041_66f4, 0x0028_a694)?; // strcpy
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_SLOT + 8);
        oc767_addiu(c, A2, S0, SND_LOAD_Q_SLOT);
        c.g[A0][0] = oc767_lw(c, S0, SND_LOAD_Q_SLOT);
        oc767_addiu(c, V0, V0, 1);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_SLOT);
        c.div(0, V0, A0);
        c.g[A3][0] = c.g[V1][0];
        oc767_addiu(c, V1, V1, 1);
        oc767_swr(c, S0, SND_LOAD_SLOT, V1);
        // slot number
        oc767_ring_push(c, A2, V0, A1, A3);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_SLOT);

        // Six name slots: wrap.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
        if c.g[V0][0] == 0 {
            oc767_sw(c, S0, SND_LOAD_SLOT, 0);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        OC767_SND_LOAD_QUEUE_START_NEXT.call(c, 0x0028_a718)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SndLoadQueue::AllocAsyncBuffer(this, size)` (0x28a1e8).
///
/// With a non-zero `size`, allocates the async load buffer with the named
/// allocator (`operator_new_named`, size, "...", 0, 0); the size goes to
/// `this+0x1d0` and the buffer to `this+0x1cc`. A zero size does nothing.
pub fn snd_load_queue_alloc_async_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_size = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !no_size {
            c.cycles += 7;
            c.g[A1][0] = 0x48_0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2988));
            let this = r32(c, S0);
            c.sw(this.wrapping_add(SNDLOADQUEUE_ASYNC_SIZE), c.g[V0][0]);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0028_a218)?;
            let this = r32(c, S0);
            c.sw(this.wrapping_add(SNDLOADQUEUE_ASYNC_BUFFER), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndLoadQueue`: the async buffer size and pointer.
pub(crate) const SNDLOADQUEUE_ASYNC_SIZE: u32 = 0x1d0;

pub(crate) const SNDLOADQUEUE_ASYNC_BUFFER: u32 = 0x1cc;

/// `void SndBankMan_NotifyQueueHeads(SndBankMan* self, int queue)` (0x28b0f0):
/// pick the head entries of the two queues at `self+0x18c` and `self+0x1bc`
/// (or -1 when a queue has none) and hand the first to the function pointer
/// stored in the queue's dispatch table, with the chosen entry as argument.
pub fn snd_bank_man_notify_queue_heads(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        let bank = c.g[A3][0] as u32;
        c.g[T0][0] = sx(bank.wrapping_add(0x18c));
        c.g[V0][0] = Addr(bank.wrapping_add(0x1d4)).lw(c);
        c.g[A1][0] = Addr(bank.wrapping_add(0x194)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = Addr(bank.wrapping_add(0x198)).lw(c);
        c.g[T1][0] = sx(bank.wrapping_add(c.g[V1][0] as u32));
        if c.g[A1][0] != c.g[A0][0] {
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        } else {
            c.cycles += 2;
            c.g[A1][0] = u64::MAX;
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(bank.wrapping_add(0x1c8)).lw(c);
        c.g[V0][0] = Addr(bank.wrapping_add(0x1c4)).lw(c);
        let differ = c.g[V0][0] != c.g[V1][0];
        c.g[A0][0] = sx(bank.wrapping_add(0x1bc));
        if differ {
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
        } else {
            c.cycles += 2;
            c.g[A3][0] = u64::MAX;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[T2][0] as u32).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = c.g[T1][0];
        call_ptr(c, target, 0x0028_b174)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc767_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc739_install;
    pub(crate) use crate::game::sound::soundsys::tests::oc749_install;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::camera::cameraalgorithms::tests::world_chain;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn ssx_audio_system_is_in_game_matches() {
        check(
            ssx3_ee::s07::f_0028b1b0__generated,
            ssx_audio_system_is_in_game,
            300,
            &|c, seed| {
                let mut r = Rng(0x4567_89ab ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x848, DATA);
                let v = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 0x8000_0000,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x84, v);
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_be_library_matches() {
        check(
            ssx3_ee::s07::f_0028b1e8__generated,
            ssx_audio_system_get_be_library,
            300,
            &|c, seed| {
                let mut r = Rng(0x5678_9abc ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x848, DATA);
                c.bus
                    .write32(DATA + 0x78, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn snd_bank_ctor_matches() {
        check(ssx3_ee::s07::f_0028b248, snd_bank_ctor, 50, &|c, seed| {
            let mut r = Rng(0x1111_0009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..10 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn ssx_audio_system_get_instance_matches() {
        check(
            ssx3_ee::s07::f_0028b180__generated,
            ssx_audio_system_get_instance,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b18_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP + 0x410,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_app_matches() {
        check(
            ssx3_ee::s07::f_0028b1c0__generated,
            ssx_audio_system_get_app,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b1c_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x848),
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    // ------------------------------------------------------------------- audio

    /// `world -> +0x84 -> +0x84 -> +0x10` (audio system, app, count).
    pub(crate) fn audio_world(c: &mut Cpu) -> u32 {
        let world = DATA + 0x1000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        world
    }

    #[test]
    fn ssx_audio_system_get_view_count_matches() {
        check(
            ee::s07::f_0028b1f8__generated,
            ssx_audio_system_get_view_count,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0028_b1f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = audio_world(c);
                c.bus.write32(world + 0x84, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_view_matches() {
        check(
            ee::s07::f_0028b210__generated,
            ssx_audio_system_get_view,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_b210, 0);
                world_chain(c, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x3000 + 4 + 4 * i, ptr(&mut r));
                }
            },
        );
    }

    #[test]
    fn snd_bank_unload_linked_matches() {
        check(
            ee::s07::f_0028b2d0__generated,
            snd_bank_unload_linked,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x28b2_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                let nodes = (seed % 4) as u32;
                let mut next = this;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x200 + 0x100 * i;
                    clear(c, node, 0x40);
                    c.bus.write32(node + 0x18, next);
                    next = node;
                }
                let head = if seed % 5 == 0 { this } else { next };
                c.bus.write32(this + 0x18, head);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn snd_bank_dtor_matches() {
        check(
            ee::s07::f_0028b278__generated,
            snd_bank_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x28b278, 0);
                alloc_ready(c);
                let this = heap_block(c, 0x400);
                clear(c, this, 0x100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Sound.

    #[test]
    fn snd_bank_unload_matches() {
        check(
            ee::s07::f_0028b528__generated,
            snd_bank_unload,
            SEEDS_4,
            &|c, seed| {
                let _ = base(c, seed, 0x28b5_28);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, s, if seed % 3 == 0 { 1 } else { 0 });
                w(c, s + 4, if seed % 2 == 0 { 0 } else { DATA + 0x300 });
                w(c, s + 8, if seed % 4 < 2 { 0 } else { 1 });
                w(c, s + 0x10, DATA + 0x400);
                clear(c, DATA + 0x300, 0x40);
                clear(c, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn audio_get_frame_rate_matches() {
        leaf(
            ssx3_ee::s07::f_0028b240__generated,
            audio_get_frame_rate,
            16,
        );
    }

    // ---- SndLoadQueue_Update (0x28aaf8) ----

    pub(crate) mod load_queue {
        const B: u32 = 0x00b0_0000;
        pub const QUEUE: u32 = B;
        pub const RINGS: u32 = B + 0x400;
        pub const FILES: u32 = B + 0x800;
        pub const BUF: u32 = B + 0x1000;
        /// `AsyncFile` handle table: count (0x519bf0) and table pointer
        /// (0x450c34, 0x30-byte entries: +0 handle, +0x8 size, +0x10
        /// error, +0x1c pending request, +0x2c end).
        pub const FILE_COUNT: u32 = 0x0051_9bf0;
        pub const FILE_TABLE: u32 = 0x0045_0c34;
    }

    /// A queue that is idle, or running a request of kind 0..3 or none;
    /// rings holding one entry or empty; a handle that is unknown (the read
    /// failed), still pending, or done with no data or a small zeroed
    /// buffer; the owner's allocate/notify slots are fakes. The mutex and
    /// thread syscalls land in the fakes too. One-entry rings keep
    /// `SndLoadQueue_StartNext` from opening another file.
    pub(crate) fn snd_load_queue_update_setup(c: &mut Cpu, seed: u32) {
        use load_queue::*;
        let mut r = machine(c, seed, 0x0028_aaf8, 6);
        let slot = r.next_u32() % 4;
        let handle = 0x100 | slot;
        c.bus
            .write32(QUEUE, if seed % 7 == 0 { 0x1ff } else { handle });
        c.bus.write32(QUEUE + 4, (seed % 9 != 0) as u32);
        // Rings: +0x18c ids, +0x19c name slots, +0x1ac kinds, +0x1bc params.
        for (i, off) in [0x18cu32, 0x19c, 0x1ac, 0x1bc].into_iter().enumerate() {
            let data = RINGS + 0x40 * i as u32;
            let empty = match off {
                0x1ac => seed % 11 == 0,
                0x18c => false,
                _ => r.next_u32() % 4 == 0,
            };
            c.bus.write32(QUEUE + off, 4);
            c.bus.write32(QUEUE + off + 4, data);
            let tail = r.next_u32() % 4;
            c.bus
                .write32(QUEUE + off + 8, if empty { tail } else { (tail + 1) % 4 });
            c.bus.write32(QUEUE + off + 0xc, tail);
            let value = match off {
                0x1ac => (seed / 2) % 4,
                0x19c => r.next_u32() % 4,
                _ => r.next_u32() % 64,
            };
            c.bus.write32(data + 4 * tail, value);
        }
        fill(c, &mut r, QUEUE + 0xc, 0x40);
        let vt = vtable(c, &mut r, &[0x18, 0x28]);
        c.bus.write32(QUEUE + 0x1d4, vt);
        // The file: pending, or done with 0 or 0x40 bytes.
        c.bus.write32(FILE_COUNT, 4);
        c.bus.write32(FILE_TABLE, FILES);
        let entry = FILES + 0x30 * slot;
        c.bus.write32(entry, handle);
        let size = if (seed / 3) % 4 == 0 { 0x40 } else { 0 };
        c.bus.write32(entry + 0x8, size);
        c.bus.write32(entry + 0x10, 0);
        c.bus.write32(entry + 0x1c, ((seed / 5) % 4 == 0) as u32);
        c.bus.write32(entry + 0x2c, BUF + size);
        c.g[A0][0] = QUEUE as u64;
        c.pc = 0x0028_aaf8;
    }

    #[test]
    fn snd_load_queue_update_matches() {
        check(
            ee::s07::f_0028aaf8__generated,
            snd_load_queue_update,
            300,
            &snd_load_queue_update_setup,
        );
    }

    use crate::difftest::fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc739_gate;
    pub(crate) use crate::game::sound::soundsys::tests::oc749_gate;
    pub(crate) use crate::game::sound::soundsys::tests::oc749_res;
    pub(crate) use crate::game::sound::soundsys::tests::OC739_CTRL;

    pub(crate) use crate::game::sound::soundsys::tests::OC749_LOAD_SIZE;

    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    pub(crate) const OC738_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC738_MAGIC: u32 = 0x4f43_3738;

    /// A fake for a callee's override slot, active only on machines that set
    /// the control word (the suite runs its tests in parallel).
    macro_rules! oc738_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC738_CTRL) != OC738_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc738_fake!(
        oc738_fake_get_be,
        OC738_PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );

    oc738_fake!(
        oc738_fake_get_interface,
        OC738_PREV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );

    oc738_fake!(
        oc738_fake_get_char_id,
        OC738_PREV_GET_CHAR_ID,
        ee::s01::f_0014a080__generated,
        0x0014_a080
    );

    oc738_fake!(
        oc738_fake_event_locked,
        OC738_PREV_EVENT_LOCKED,
        ee::s01::f_00146a70__generated,
        0x0014_6a70
    );

    oc738_fake!(
        oc738_fake_elem_pos,
        OC738_PREV_ELEM_POS,
        ee::s04::f_001e91f8__generated,
        0x001e_91f8
    );

    oc738_fake!(
        oc738_fake_elem_scale,
        OC738_PREV_ELEM_SCALE,
        ee::s04::f_001e9290__generated,
        0x001e_9290
    );

    oc738_fake!(
        oc738_fake_elem_color,
        OC738_PREV_ELEM_COLOR,
        ee::s04::f_001e91a8__generated,
        0x001e_91a8
    );

    oc738_fake!(
        oc738_fake_replay_save_snapshot,
        OC738_PREV_REPLAY_SAVE_SNAPSHOT,
        ee::s07::f_0026f7f8__generated,
        0x0026_f7f8
    );

    oc738_fake!(
        oc738_fake_replay_set_state,
        OC738_PREV_REPLAY_SET_STATE,
        ee::s07::f_0026f4a0__generated,
        0x0026_f4a0
    );

    oc738_fake!(
        oc738_fake_replay_seek,
        OC738_PREV_REPLAY_SEEK,
        ee::s07::f_002702f8__generated,
        0x0027_02f8
    );

    oc738_fake!(
        oc738_fake_replay_step,
        OC738_PREV_REPLAY_STEP,
        ee::s07::f_002703f0__generated,
        0x0027_03f0
    );

    oc738_fake!(
        oc738_fake_replay_instant_cams,
        OC738_PREV_REPLAY_INSTANT_CAMS,
        ee::s07::f_0026ef30__generated,
        0x0026_ef30
    );

    oc738_fake!(
        oc738_fake_replay_cams,
        OC738_PREV_REPLAY_CAMS,
        ee::s07::f_0026ee68__generated,
        0x0026_ee68
    );

    oc738_fake!(
        oc738_fake_audio_instance,
        OC738_PREV_AUDIO_INSTANCE,
        ee::s07::f_0028b180__generated,
        0x0028_b180
    );

    oc738_fake!(
        oc738_fake_audio_replay_begin,
        OC738_PREV_AUDIO_REPLAY_BEGIN,
        ee::s07::f_0028f200__generated,
        0x0028_f200
    );

    oc738_fake!(
        oc738_fake_list_add_choice,
        OC738_PREV_LIST_ADD_CHOICE,
        ee::s07::f_00278e50__generated,
        0x0027_8e50
    );

    oc738_fake!(
        oc738_fake_list_prepare,
        OC738_PREV_LIST_PREPARE,
        ee::s07::f_00278f38__generated,
        0x0027_8f38
    );

    oc738_fake!(
        oc738_fake_list_start,
        OC738_PREV_LIST_START,
        ee::s07::f_00278f68__generated,
        0x0027_8f68
    );

    oc738_fake!(
        oc738_fake_async_load_at,
        OC738_PREV_ASYNC_LOAD_AT,
        ee::s14::f_003df748__generated,
        0x003d_f748
    );

    oc738_fake!(
        oc738_fake_async_load,
        OC738_PREV_ASYNC_LOAD,
        ee::s14::f_003df690__generated,
        0x003d_f690
    );

    /// A fake whose result becomes `$val` when the control word at
    /// `OC738_CTRL + $flag` is non-zero (a result the generic fake cannot
    /// produce: -1, or zero).
    macro_rules! oc738_fake_val {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, $flag:expr, $val:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC738_CTRL) != OC738_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if c.pc == c.g[RA][0] as u32 && c.bus.read32(OC738_CTRL + $flag) != 0 {
                    c.g[V0][0] = $val;
                }
            }
        };
    }

    // `getTrackMedal` returns -1 (no medal).
    oc738_fake_val!(
        oc738_fake_medal,
        OC738_PREV_MEDAL,
        ee::s01::f_00158910__generated,
        0x0015_8910,
        4,
        u64::MAX
    );

    // `cSectionMan_isLoaded` returns 0.
    oc738_fake_val!(
        oc738_fake_is_loaded,
        OC738_PREV_IS_LOADED,
        ee::s05::f_0022d278__generated,
        0x0022_d278,
        8,
        0
    );

    // `IsActiveNISInTransition` returns 0.
    oc738_fake_val!(
        oc738_fake_nis_active,
        OC738_PREV_NIS_ACTIVE,
        ee::s07::f_00279298__generated,
        0x0027_9298,
        0xc,
        0
    );

    oc738_fake!(
        oc738_fake_nis_end_pos,
        OC738_PREV_NIS_END_POS,
        ee::s00::f_00123f38__generated,
        0x0012_3f38
    );

    oc738_fake!(
        oc738_fake_set_focus,
        OC738_PREV_SET_FOCUS,
        ee::s13::f_003a9658__generated,
        0x003a_9658
    );

    oc738_fake!(
        oc738_fake_get_test_focus,
        OC738_PREV_GET_TEST_FOCUS,
        ee::s13::f_003a9770__generated,
        0x003a_9770
    );

    oc738_fake!(
        oc738_fake_get_transition_man,
        OC738_PREV_GET_TRANSITION_MAN,
        ee::s06::f_00230698__generated,
        0x0023_0698
    );

    oc738_fake!(
        oc738_fake_end_transport,
        OC738_PREV_END_TRANSPORT,
        ee::s07::f_0027a9f0__generated,
        0x0027_a9f0
    );

    oc738_fake!(
        oc738_fake_section_activate,
        OC738_PREV_SECTION_ACTIVATE,
        ee::s05::f_0022d088__generated,
        0x0022_d088
    );

    pub(crate) fn oc738_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc738_fake_get_be,
            &OC738_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc738_fake_get_interface,
            &OC738_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014A080),
            oc738_fake_get_char_id,
            &OC738_PREV_GET_CHAR_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146A70),
            oc738_fake_event_locked,
            &OC738_PREV_EVENT_LOCKED,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E91F8),
            oc738_fake_elem_pos,
            &OC738_PREV_ELEM_POS,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E9290),
            oc738_fake_elem_scale,
            &OC738_PREV_ELEM_SCALE,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E91A8),
            oc738_fake_elem_color,
            &OC738_PREV_ELEM_COLOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D278),
            oc738_fake_is_loaded,
            &OC738_PREV_IS_LOADED,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00279298),
            oc738_fake_nis_active,
            &OC738_PREV_NIS_ACTIVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123F38),
            oc738_fake_nis_end_pos,
            &OC738_PREV_NIS_END_POS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9658),
            oc738_fake_set_focus,
            &OC738_PREV_SET_FOCUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9770),
            oc738_fake_get_test_focus,
            &OC738_PREV_GET_TEST_FOCUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            oc738_fake_get_transition_man,
            &OC738_PREV_GET_TRANSITION_MAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027A9F0),
            oc738_fake_end_transport,
            &OC738_PREV_END_TRANSPORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D088),
            oc738_fake_section_activate,
            &OC738_PREV_SECTION_ACTIVATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F7F8),
            oc738_fake_replay_save_snapshot,
            &OC738_PREV_REPLAY_SAVE_SNAPSHOT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            oc738_fake_replay_set_state,
            &OC738_PREV_REPLAY_SET_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002702F8),
            oc738_fake_replay_seek,
            &OC738_PREV_REPLAY_SEEK,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002703F0),
            oc738_fake_replay_step,
            &OC738_PREV_REPLAY_STEP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026EF30),
            oc738_fake_replay_instant_cams,
            &OC738_PREV_REPLAY_INSTANT_CAMS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026EE68),
            oc738_fake_replay_cams,
            &OC738_PREV_REPLAY_CAMS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc738_fake_audio_instance,
            &OC738_PREV_AUDIO_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028F200),
            oc738_fake_audio_replay_begin,
            &OC738_PREV_AUDIO_REPLAY_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278E50),
            oc738_fake_list_add_choice,
            &OC738_PREV_LIST_ADD_CHOICE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F38),
            oc738_fake_list_prepare,
            &OC738_PREV_LIST_PREPARE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F68),
            oc738_fake_list_start,
            &OC738_PREV_LIST_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF748),
            oc738_fake_async_load_at,
            &OC738_PREV_ASYNC_LOAD_AT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            oc738_fake_async_load,
            &OC738_PREV_ASYNC_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158910),
            oc738_fake_medal,
            &OC738_PREV_MEDAL,
        );
    }

    /// A scrambled machine with the fakes on and the gp of the game.
    pub(crate) fn oc738_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC738_CTRL, OC738_MAGIC);
        r
    }

    /// A circular queue header at `q` (`Array` +4, `Head` +8, `Tail` +0xc)
    /// holding `values` from tail 1 on: empty when `values` is empty.
    pub(crate) fn oc738_queue(c: &mut Cpu, q: u32, array: u32, values: &[u32]) {
        c.bus.write32(q + 4, array);
        if values.is_empty() {
            c.bus.write32(q + 8, 2);
            c.bus.write32(q + 0xc, 2);
            return;
        }
        c.bus.write32(q + 8, 5);
        c.bus.write32(q + 0xc, 1);
        for (i, v) in values.iter().enumerate() {
            c.bus.write32(array + 4 * (1 + i as u32), *v);
        }
    }

    /// `SndLoadQueue_StartNext`: busy; empty user queue; every queue empty or
    /// not; kinds 0, 1 (buffer allocated or not), 2 and invalid ones.
    #[test]
    fn snd_load_queue_start_next_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s07::f_0028a298__generated,
            snd_load_queue_start_next,
            600,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x28_a298, 4);
                let this = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.bus.write32(this + 4, (seed % 7 == 0) as u32);
                let handle = r.next_u32();
                let one_user = [handle];
                let users: &[u32] = if seed % 5 == 0 { &[] } else { &one_user };
                oc738_queue(c, this + 0x18c, DATA + 0x3000, users);
                let kind = [0u32, 1, 2, 3, u32::MAX, 1, 0, 2, 1][((seed / 7) % 9) as usize];
                let one_kind = [kind];
                let kinds: &[u32] = if (seed / 63) % 8 == 0 { &[] } else { &one_kind };
                oc738_queue(c, this + 0x1ac, DATA + 0x3100, kinds);
                let one_name = [r.next_u32() % 8];
                let names: &[u32] = if (seed / 3) % 6 == 0 { &[] } else { &one_name };
                oc738_queue(c, this + 0x19c, DATA + 0x3200, names);
                let one_flag = [r.next_u32()];
                let flags: &[u32] = if (seed / 4) % 4 == 0 { &[] } else { &one_flag };
                oc738_queue(c, this + 0x1bc, DATA + 0x3300, flags);
                c.bus.write32(this + 0x1cc, r.next_u32());
                c.bus.write32(this + 0x1d0, r.next_u32());
                c.bus.write32(this + 0x1d4, vt);
                for off in [0x10u32, 0x18, 0x20] {
                    c.bus
                        .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
                }
                c.bus.write32(vt + 0x14, fakes::FAKE);
                // Allocation: a fake (non-zero buffer) or the zero leaf.
                let alloc = if (seed / 2) % 2 == 0 {
                    fakes::FAKE + 0x10
                } else {
                    0x0014_06c0
                };
                c.bus.write32(vt + 0x1c, alloc);
                c.bus.write32(vt + 0x24, fakes::FAKE + 0x20);
                c.g[A0][0] = sx(this);
                c.pc = 0x0028_a298;
            },
        );
    }

    /// `SndLoadQueue_CancelBank`: four rings (size, data, head, tail at
    /// +0x18c/+0x19c/+0x1ac/+0x1bc), the busy word at +4. On `seed % 3 == 0`
    /// the current load is the cancelled bank and busy is set (finish path).
    #[test]
    fn snd_load_queue_cancel_bank_matches() {
        oc739_install();
        fakes::covered(
            ee::s07::f_0028a728__generated,
            snd_load_queue_cancel_bank,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28a728, 6);
                oc739_gate(c);
                for k in 0..8u32 {
                    let v = r.next_u32() % 2;
                    c.bus.write32(
                        OC739_CTRL + 0x20 + 4 * k,
                        if k < 2 { v } else { DATA + 0x5000 + 0x40 * k },
                    );
                }
                let this = DATA + 0x100;
                let id = 1 + r.next_u32() % 3;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id as u64;
                let finish = seed % 3 == 0;
                let busy = if finish { 1 } else { r.next_u32() % 2 };
                c.bus.write32(this, DATA + 0x5000);
                c.bus.write32(this + 4, busy);
                for k in 0..4u32 {
                    let base = this + 0x18c + 0x10 * k;
                    let size = 2 + r.next_u32() % 5;
                    let data = DATA + 0x1000 + 0x40 * k;
                    let head = r.next_u32() % size;
                    let mut tail = r.next_u32() % size;
                    if (k == 0 || k == 3) && finish && head == tail {
                        tail = (tail + 1) % size;
                    }
                    c.bus.write32(base, size);
                    c.bus.write32(base + 4, data);
                    c.bus.write32(base + 8, head);
                    c.bus.write32(base + 0xc, tail);
                    for i in 0..size {
                        c.bus.write32(data + 4 * i, 1 + r.next_u32() % 3);
                    }
                    if k == 0 && finish {
                        c.bus.write32(data + 4 * tail, id);
                    }
                    if k == 3 && finish {
                        // The finish path calls the node virtuals for kinds 1 and 2.
                        c.bus.write32(data + 4 * tail, 1 + r.next_u32() % 2);
                    }
                }
                c.bus.write32(this + 0x1cc, r.next_u32());
                c.bus.write32(this + 0x1d0, r.next_u32());
                let vt = DATA + 0x2000;
                c.bus.write32(this + 0x1d4, vt);
                for off in (0..0x40u32).step_by(4) {
                    let lh_slot = matches!(off, 0x10 | 0x18 | 0x20 | 0x28);
                    c.bus
                        .write32(vt + off, if lh_slot { 0 } else { fakes::FAKE });
                }
            },
        );
    }

    /// `SndBank_Load`: by allocation location (+8: 0 heap, 1 preloaded buffer,
    /// 2 stream memory, 3 none), the file loaders (failing sometimes), the
    /// bank-add result (7: header copy, 8: keep data, other: free) and the two
    /// bank-manager virtual calls.
    #[test]
    fn snd_bank_load_matches() {
        oc749_install();
        fakes::covered(
            ee::s07::f_0028b320__generated,
            snd_bank_load,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b320, 2);
                oc749_gate(c, &mut r);
                let this = DATA + 0x100;
                let bank_mgr = DATA + 0x600;
                let p1 = DATA + 0x700;
                let q = DATA + 0x710;
                let vt = DATA + 0x800;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(this + 4, r.next_u32());
                c.bus.write32(this + 8, seed % 4);
                c.bus.write32(this + 0xc, r.next_u32());
                c.bus.write32(this + 0x10, DATA + 0xb00);
                c.bus.write32(this + 0x14, r.next_u32() % 0x1000);
                c.bus.write32(GAME_GP + 0x524, bank_mgr);
                c.bus.write32(bank_mgr + 0x1d8, p1);
                c.bus.write32(p1, q);
                c.bus.write32(q, vt);
                c.bus
                    .write16(vt + 0x30, if r.next_u32() % 2 == 0 { 0 } else { 0x10 });
                c.bus.write32(vt + 0x34, fakes::FAKE);
                c.bus
                    .write16(vt + 0x38, if r.next_u32() % 2 == 0 { 0 } else { 0x20 });
                c.bus.write32(vt + 0x3c, fakes::FAKE + 0x10);
                let data = if r.next_u32() % 6 == 0 {
                    0
                } else {
                    DATA + 0x900
                };
                oc749_res(c, 0x3e1a70, data);
                c.bus.write32(OC749_LOAD_SIZE, r.next_u32() % 0x1000);
                oc749_res(c, 0x3e1bb8, (r.next_u32() % 6 != 0) as u32);
                oc749_res(c, 0x3b6098, [7, 8, 3][(seed as usize / 4) % 3]);
                oc749_res(c, 0x3b6528, r.next_u32() % 0x400);
                oc749_res(c, 0x252fa0, DATA + 0xa00);
            },
        );
    }

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::lib::snd::tests::OC794_THIS;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC767_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC767_MAGIC;

    pub(crate) const OC767_LOAD_OBJ: u32 = DATA + 0x1000;

    pub(crate) const OC767_LOAD_RINGS: u32 = DATA + 0x3000;

    /// `SndLoadQueue_Push`: four ring queues of size 1-5 with random head
    /// and tail (full queues drop the entry, size-1 queues are always full),
    /// the name slot 0-7 (wraps at 6); the `strcpy` and `StartNext` callees
    /// are fakes, some of which unwind.
    #[test]
    fn snd_load_queue_push_matches() {
        oc767_install_fakes();
        covered(
            ee::s07::f_0028a558__generated,
            snd_load_queue_push,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_a558, 2);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                fill(c, &mut r, OC767_LOAD_OBJ, 0x1e0 / 4);
                fill(c, &mut r, OC767_LOAD_RINGS, 0x100 / 4);
                for (k, q) in [
                    SND_LOAD_Q_BANK,
                    SND_LOAD_Q_SLOT,
                    SND_LOAD_Q_TYPE,
                    SND_LOAD_Q_FLAGS,
                ]
                .into_iter()
                .enumerate()
                {
                    let size = 1 + r.next_u32() % 5;
                    c.bus.write32(OC767_LOAD_OBJ + q, size);
                    c.bus
                        .write32(OC767_LOAD_OBJ + q + 4, OC767_LOAD_RINGS + 0x40 * k as u32);
                    c.bus.write32(OC767_LOAD_OBJ + q + 8, r.next_u32() % size);
                    c.bus.write32(OC767_LOAD_OBJ + q + 0xc, r.next_u32() % size);
                }
                c.bus
                    .write32(OC767_LOAD_OBJ + SND_LOAD_SLOT, r.next_u32() % 8);
                c.g[A0][0] = sx(OC767_LOAD_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(DATA + 0x800);
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.pc = 0x0028_a558;
            },
        );
    }

    /// The word that arms oc794's callee fakes in a machine's RAM.
    pub(crate) const OC794_CTRL: u32 = DATA + 0x32_0000;

    pub(crate) const OC794_MAGIC: u32 = 0x0794_f4ce;

    pub(crate) static OC794_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC794_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC794_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc794_fk_new(c: &mut Cpu) {
        if c.bus.read32(OC794_CTRL) != OC794_MAGIC {
            return fakes::fall_back(c, &OC794_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        fakes::fake_call(c, 0x0031_7e30);
    }

    pub(crate) fn oc794_fk_alloc(c: &mut Cpu) {
        if c.bus.read32(OC794_CTRL) != OC794_MAGIC {
            return fakes::fall_back(c, &OC794_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        fakes::fake_call(c, 0x0031_7d70);
    }

    pub(crate) fn oc794_fk_ctor(c: &mut Cpu) {
        if c.bus.read32(OC794_CTRL) != OC794_MAGIC {
            return fakes::fall_back(c, &OC794_PREV_CTOR, ee::s08::f_002b58d0__generated);
        }
        fakes::fake_call(c, 0x002b_58d0);
    }

    /// Hook the gated fakes (once per process) and arm them in this machine.
    pub(crate) fn oc794_arm(c: &mut Cpu) {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc794_fk_new,
            &OC794_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc794_fk_alloc,
            &OC794_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B58D0),
            oc794_fk_ctor,
            &OC794_PREV_CTOR,
        );
        c.bus.write32(OC794_CTRL, OC794_MAGIC);
    }

    /// `SndLoadQueue_AllocAsyncBuffer`: zero and non-zero sizes and the
    /// unwind of the allocator call.
    #[test]
    fn snd_load_queue_alloc_async_buffer_matches() {
        fakes::covered(
            ee::s07::f_0028a1e8__generated,
            snd_load_queue_alloc_async_buffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x794b, 1);
                oc794_arm(c);
                c.pc = 0x0028_a1e8;
                c.g[A0][0] = sx(OC794_THIS);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(r.next_u32() & 0xf_fff0 | 0x10)
                };
            },
        );
    }

    use crate::difftest::fakes::FAKE;

    #[test]
    fn snd_bank_man_notify_queue_heads_matches() {
        check(
            ee::s07::f_0028b0f0__generated,
            snd_bank_man_notify_queue_heads,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0028_b0f0, 0);
                let bank = DATA + 0x1000;
                let queue = DATA + 0x2000;
                let table = DATA + 0x3000;
                let idx = seed % 4;
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (seed as u64).wrapping_mul(7);
                // The queue record: the entry offset and the call target.
                c.bus.write32(bank + 0x1d4, queue);
                c.bus.write16(queue + 0x18, 0x40);
                c.bus.write32(queue + 0x1c, FAKE);
                // First queue: the array base and index at +0x190/+0x198;
                // the head is `a1` (+0x194) against `a0` (+0x198).
                c.bus.write32(bank + 0x190, table);
                c.bus.write32(bank + 0x198, idx);
                c.bus
                    .write32(bank + 0x194, if seed % 2 == 0 { idx } else { idx + 1 });
                // Second queue: the same at +0x1c0/+0x1c8 and +0x1c4.
                c.bus.write32(bank + 0x1c0, table);
                c.bus.write32(bank + 0x1c8, idx);
                c.bus.write32(
                    bank + 0x1c4,
                    if (seed / 2) % 2 == 0 { idx } else { idx + 1 },
                );
                c.bus.write32(table + 4 * idx, 0x1234 + seed);
            },
        );
    }
}
