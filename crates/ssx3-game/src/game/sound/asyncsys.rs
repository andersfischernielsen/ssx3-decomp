//! `src/sound/asyncsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_4 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp_3;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
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

    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;

    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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
    pub(crate) use crate::game::dirtysock::tags2::tests::common;

    pub(crate) use crate::game::dirtysock::tags2::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_3;
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
}
