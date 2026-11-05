//! Staging wave 206 (oc249): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `App_VSyncTick(App* app)` (0x317348, appman.cpp): the application
/// manager's per-vsync tick. Bumps the vsync counter (+0x18), adds the step
/// (+0x24, ticks per vsync) to the fractional accumulator (+0x28), keeps the
/// fraction and calls the app's `Tick` method (vtable +0x5c, slot 0x38) once
/// per whole tick crossed; then calls the current sub-object's (+0x8) slot
/// 0x28 (its per-vsync update).
///
/// The tick count is `trunc(acc + step) - trunc(acc)`; the loop runs while
/// the count, decremented first, is not -1.
pub fn app_vsync_tick(c: &mut Cpu) {
    const COUNTER: u32 = 0x18;
    const STEP: u32 = 0x24;
    const ACCUM: u32 = 0x28;
    const VTABLE: u32 = 0x5c;
    const SUB: u32 = 0x8;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = u64::MAX;
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let app = Addr(r32(c, S1));
        let accum = app.off(ACCUM).read_fpu(c);
        let step = app.off(STEP).read_fpu(c);
        let whole_before = accum.to_i32();
        c.f[2] = whole_before as u32;
        c.g[A0][0] = sx(whole_before as u32);
        let count = app.off(COUNTER).lw(c);
        let count = sx((count as u32).wrapping_add(1));
        c.g[V0][0] = count;
        app.off(COUNTER).sw(c, count);
        let sum = accum + step;
        let whole_after = sum.to_i32();
        c.f[1] = whole_after as u32;
        c.g[V1][0] = sx(whole_after as u32);
        let ticks = (whole_after as u32).wrapping_sub(whole_before as u32);
        let ticks_f = FpuF32::from_i32(ticks as i32);
        c.f[1] = ticks_f.0;
        let frac = sum - ticks_f;
        abi::set_fpr(c, 0, frac);
        c.g[S0][0] = sx(ticks.wrapping_sub(1));
        app.off(ACCUM).write_fpu(c, frac);
        if c.g[S0][0] != c.g[A1][0] {
            // One `Tick` call per whole tick crossed.
            c.cycles += 2;
            c.g[S2][0] = u64::MAX;
            c.g[V0][0] = Addr(r32(c, S1)).off(VTABLE).lw(c);
            loop {
                c.cycles += 7;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                let vt = Addr(r32(c, V0));
                let delta = vt.off(0x38).lh(c);
                c.g[A0][0] = delta;
                let method = vt.off(0x3c).lw(c);
                c.g[V1][0] = method;
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(delta as u32));
                call_ptr(c, method as u32, 0x0031_73c4)?;
                if c.g[S0][0] == c.g[S2][0] {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S1)).off(VTABLE).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_73b0);
                }
            }
        }
        // The sub-object's per-vsync update (its vtable at +0x8, slot 0x28).
        c.cycles += 12;
        let sub = Addr(r32(c, S1)).off(SUB).lw(c);
        c.g[A1][0] = sub;
        let vt = Addr(sub as u32).off(0x8).lw(c);
        c.g[V0][0] = vt;
        let delta = Addr(vt as u32).off(0x28).lh(c);
        c.g[A0][0] = delta;
        let method = Addr(vt as u32).off(0x2c).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = sx((sub as u32).wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x0031_73e4)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatSetVolume(int path, int volume)` (0x2af6c0, bankmonitor.cpp):
/// sets the volume of a playing path-music stream. `paths[path]` (the table
/// at 0x4d3e98) is the stream record: +0x0 stream handle, +0x8 pending
/// volume (-1: none), +0xc current volume, +0x64 "is a sound, not a stream"
/// flag (its channel handle at `*(+0x6c) + 4`), +0x65 status dirty flag.
/// Returns 0 when the audio system is off (`+0x6280` of the audio system),
/// -8 when a different volume change is still pending; else clears the
/// pending volume and, if the volume changed and both are non-negative,
/// sets it under `SND_Lock` and records it.
pub fn path_plat_set_volume(c: &mut Cpu) {
    const PATHS: u32 = 0x004d_3e98;
    const HANDLE: u32 = 0x0;
    const PENDING: u32 = 0x8;
    const CURRENT: u32 = 0xc;
    const IS_SOUND: u32 = 0x64;
    const DIRTY: u32 = 0x65;
    const SOUND: u32 = 0x6c;
    const AUDIO_ENABLED: u32 = 0x6280;
    /// Where the generated body leaves the function.
    enum Exit {
        /// 0x2af784: `v0 = s2` first.
        Result,
        /// 0x2af788: restore s0, then the rest.
        RestoreS0,
        /// 0x2af78c: s0 already restored.
        Rest,
    }
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        frame.save(c, 18, 0x10);
        c.g[V0][0] = sx(PATHS);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(PATHS));
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = Addr(r32(c, A0)).lw(c);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0028_b180, 0x002a_f6f4)?; // SSXAudioSystem_GetInstance
        c.g[V1][0] = Addr(r32(c, V0)).off(AUDIO_ENABLED).lw(c);
        c.g[V0][0] = 0;
        let exit = if c.g[V1][0] == 0 {
            Exit::RestoreS0
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).off(DIRTY).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x002a_efa8,
                    f: ee::s08::f_002aefa8,
                }
                .call(c, 0x002a_f714)?;
            }
            let path = Addr(r32(c, S0));
            let pending = path.off(PENDING).lw(c);
            c.g[V0][0] = pending;
            c.cycles += 2;
            let current = path.off(CURRENT).lw(c);
            c.g[V1][0] = current;
            let busy = (pending as i64) >= 0 && {
                c.cycles += 2;
                pending != current
            };
            if busy {
                c.g[S2][0] = (-8i64) as u64;
                Exit::Result
            } else {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                path.off(PENDING).sw(c, u64::MAX);
                if current == c.g[S1][0] {
                    Exit::Result
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    if (c.g[S1][0] as i64) < 0 {
                        Exit::RestoreS0
                    } else {
                        c.cycles += 2;
                        if (current as i64) < 0 {
                            frame.restore(c, 16, 0x30);
                            Exit::Rest
                        } else {
                            c.cycles += 5;
                            call_ptr(c, 0x003b_58a0, 0x002a_f74c)?; // SND_Lock
                            c.g[V0][0] = Addr(r32(c, S0)).off(IS_SOUND).lb(c);
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] != 0 {
                                c.cycles += 5;
                                let sound = Addr(r32(c, S0)).off(SOUND).lw(c);
                                c.g[V0][0] = sound;
                                c.g[A0][0] = Addr(sound as u32).off(4).lw(c);
                                call_ptr(c, 0x003b_85f0, 0x002a_f764)?; // SND_SetVolume
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = Addr(r32(c, S0)).off(HANDLE).lw(c);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x003b_7e70, 0x002a_f778)?; // SNDSTRM_SetVolume
                            }
                            c.cycles += 3;
                            call_ptr(c, 0x003b_58d8, 0x002a_f780)?; // SND_Unlock
                            let volume = c.g[S1][0];
                            Addr(r32(c, S0)).off(CURRENT).sw(c, volume);
                            Exit::Result
                        }
                    }
                }
            }
        };
        if let Exit::Result = exit {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        if !matches!(exit, Exit::Rest) {
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The EE voice mixer's global state (`0x515b40`): +0x5 number of extra
/// sends `n` (u8), +0xf the dry send's "on" byte, +0xb0 the mix format, +0xb4
/// the voice's sample block, +0xd0 the `n` send buffers, +0x1e0 the "set"
/// callback `set(0x10, block, buffer, level)`.
mod ee_mix {
    pub const STATE: u32 = 0x0051_5b40;
    pub const COUNT: u32 = 0x5;
    pub const DRY_ON: u32 = 0xf;
    pub const FORMAT: u32 = 0xb0;
    pub const BLOCK: u32 = 0xb4;
    pub const SENDS: u32 = 0xd0;
    pub const SET: u32 = 0x1e0;
    /// The dry mix buffer.
    pub const DRY_BUFFER: u32 = 0x0051_5d80;
    /// `SND_MixAddRamp16`.
    pub const MIX_ADD_RAMP16: u32 = 0x003c_7828;
}

/// One send of [`snd_ee_voice_mix_ramp`] (blocks 0x3c7b9c/0x3c7c28): with
/// the wanted level at `s0` and the current one at `s0 - back`, ramps from
/// the current to the wanted level over the block (`SND_MixAddRamp16`) and
/// stores it, or (no change) adds at the steady level through the state's
/// `set` callback unless it is 0. `dry` picks the buffer: the dry buffer
/// (`s4 + 0x5d80`) or the send buffer `*s3`. True when a call was made
/// (the generated body's join block with its extra cycle).
fn mix_ramp_send(
    c: &mut Cpu,
    back: u32,
    dry: bool,
    rets: (u32, u32),
) -> Result<bool, crate::call::Unwound> {
    let buffer = |c: &mut Cpu| {
        if dry {
            sx(r32(c, S4).wrapping_add(0x5d80))
        } else {
            Addr(r32(c, S3)).lw(c)
        }
    };
    c.cycles += 5;
    let cur = Addr(r32(c, S0).wrapping_sub(back)).read_fpu(c);
    let want = Addr(r32(c, S0)).read_fpu(c);
    c.f[12] = cur.0;
    c.f[13] = want.0;
    let same = cur.eq_s(want);
    abi::set_fcc(c, same);
    if !same {
        c.cycles += 6;
        c.g[A0][0] = Addr(r32(c, S2)).off(ee_mix::BLOCK).lw(c);
        c.g[A1][0] = buffer(c);
        Callee {
            addr: ee_mix::MIX_ADD_RAMP16,
            f: ee::s14::f_003c7828,
        }
        .call(c, rets.0)?;
        let want = Addr(r32(c, S0)).read_fpu(c);
        c.f[0] = want.0;
        Addr(r32(c, S0).wrapping_sub(back)).write_fpu(c, want);
        return Ok(true);
    }
    c.cycles += 4;
    let off = FpuF32(c.f[13]).eq_s(FpuF32(c.f[20]));
    abi::set_fcc(c, off);
    if off {
        return Ok(false);
    }
    c.cycles += 6;
    let set = Addr(r32(c, S2)).off(ee_mix::SET).lw(c);
    c.g[V0][0] = set;
    c.f[12] = c.f[13];
    c.g[A1][0] = Addr(r32(c, S2)).off(ee_mix::BLOCK).lw(c);
    c.g[A0][0] = 0x10;
    c.g[A2][0] = buffer(c);
    call_ptr(c, set as u32, rets.1)?;
    Ok(true)
}

/// `int SND_EeVoiceMixRamp(EeVoice* v)` (0x3c7ab0, seg_2B4578.c): mixes one
/// EE voice's block into the dry buffer and its sends. The voice's source
/// (`v+0x40`, method 0) renders the block first; when it renders nothing
/// (<= 0) the levels jump to their targets (dry +0x34 <- +0x38, sends
/// +0x4.. <- +0x1c..) and it returns 0. Otherwise it copies the block's
/// header word (+0x3c), mixes the dry send (if on) and the `n` sends, each
/// ramping from the current to the wanted level, and returns 0x10.
pub fn snd_ee_voice_mix_ramp(c: &mut Cpu) {
    use ee_mix::*;
    const SOURCE: u32 = 0x40;
    const DRY_CUR: u32 = 0x34;
    const DRY_WANT: u32 = 0x38;
    const HEADER: u32 = 0x3c;
    const SEND_CUR: u32 = 0x4;
    const SEND_WANT: u32 = 0x1c;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, 16, 0x70);
        c.g[A1][0] = 0x10;
        frame.save(c, 19, 0x40);
        c.g[S0][0] = sx(STATE);
        frame.save(c, 22, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 17, 0x60);
        c.g[S6][0] = 0x51_0000;
        frame.save(c, 18, 0x50);
        c.g[T0][0] = 0;
        frame.save(c, 20, 0x30);
        frame.save(c, 21, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x80);
        let source = Addr(r32(c, S3)).off(SOURCE).lw(c);
        c.g[A0][0] = source;
        c.g[A2][0] = Addr(STATE).off(FORMAT).lw(c);
        let render = Addr(source as u32).lw(c);
        c.g[V0][0] = render;
        c.g[A3][0] = Addr(STATE).off(BLOCK).lw(c);
        call_ptr(c, render as u32, 0x003c_7b04)?;
        if (c.g[V0][0] as i64) > 0 {
            let block = Addr(r32(c, S0)).off(BLOCK).lw(c);
            c.g[V0][0] = block;
            c.cycles += 9;
            c.g[S2][0] = c.g[S0][0];
            c.f[20] = 0;
            c.g[S5][0] = sx(r32(c, S2).wrapping_add(DRY_ON));
            let header = Addr(block as u32).off(HEADER).read_fpu(c);
            c.f[0] = header.0;
            c.g[S1][0] = 0;
            c.g[S4][0] = 0x51_0000;
            let voice = r32(c, S3);
            c.g[S0][0] = sx(voice.wrapping_add(DRY_WANT));
            Addr(voice).off(HEADER).write_fpu(c, header);
            // The dry send (a loop over one entry).
            c.cycles += 4;
            let on_at = r32(c, S1).wrapping_add(r32(c, S5));
            c.g[V0][0] = sx(on_at);
            let on = Addr(on_at).lb(c);
            c.g[V1][0] = on;
            if on != 0 && mix_ramp_send(c, 4, true, (0x003c_7bc0, 0x003c_7bf4))? {
                c.cycles += 1;
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.cycles += 2;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            // The sends.
            c.cycles += 4;
            let state = sx(r32(c, S6).wrapping_add(0x5b40));
            c.g[V1][0] = state;
            let n = Addr(state as u32).off(COUNT).lbu(c);
            c.g[V0][0] = n;
            c.g[S1][0] = 0;
            if n != 0 {
                c.cycles += 4;
                c.g[S2][0] = state;
                c.g[S0][0] = sx(r32(c, S3).wrapping_add(SEND_WANT));
                c.f[20] = 0;
                c.g[S3][0] = sx((state as u32).wrapping_add(SENDS));
                loop {
                    if mix_ramp_send(c, 0x18, false, (0x003c_7c40, 0x003c_7c74))? {
                        c.cycles += 1;
                    }
                    let n = Addr(r32(c, S2)).off(COUNT).lbu(c);
                    c.cycles += 5;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
                    let more = (c.g[S1][0] as i64) < (n as i64);
                    c.g[V0][0] = more as u64;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_7c20);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x10;
        } else {
            // Nothing rendered: the levels jump to their targets.
            c.cycles += 3;
            let voice = r32(c, S3);
            c.cycles += 7;
            let want = Addr(voice).off(DRY_WANT).read_fpu(c);
            c.f[0] = want.0;
            c.g[S1][0] = u64::MAX;
            Addr(voice).off(DRY_CUR).write_fpu(c, want);
            c.g[V0][0] = sx(voice.wrapping_add(DRY_WANT + 4));
            c.g[V1][0] = sx(voice.wrapping_add(DRY_CUR + 4));
            c.cycles += 4;
            let state = sx(r32(c, S6).wrapping_add(0x5b40));
            c.g[V1][0] = state;
            let n = Addr(state as u32).off(COUNT).lbu(c);
            c.g[V0][0] = n;
            c.g[S1][0] = 0;
            if n != 0 {
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(SEND_CUR));
                loop {
                    c.cycles += 7;
                    let cur = r32(c, A0);
                    let want = Addr(cur).off(SEND_WANT - SEND_CUR).read_fpu(c);
                    c.f[0] = want.0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    Addr(cur).write_fpu(c, want);
                    let n = Addr(r32(c, V1)).off(COUNT).lbu(c);
                    let more = (c.g[S1][0] as i64) < (n as i64);
                    c.g[V0][0] = more as u64;
                    c.g[A0][0] = sx(cur.wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_7b48);
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 11;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void Nash_ClassifyInstances(Cell* cell, List* lists, int count)`
/// (0x22a368, ssxapp.cpp): runs `Nash_ClassifyInstance(cell, node,
/// list->frustum)` over every node (linked through +0x0) of each list's
/// source (`list->src + 0x20` is the head; `+0x4` the frustum; lists are 8
/// bytes apart), then once more with a null node if the cell's instance
/// head (+0x50a4) is set.
pub fn nash_classify_instances(c: &mut Cpu) {
    const CLASSIFY: Callee = Callee {
        addr: 0x0022_9fc8,
        f: ee::s05::f_00229fc8,
    };
    const HEAD: u32 = 0x50a4;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x40);
        frame.save(c, 20, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 18, 0x30);
        frame.save(c, 19, 0x20);
        frame.save_ra(c, 0);
        if (c.g[A2][0] as i64) > 0 {
            loop {
                c.cycles += 5;
                let list = r32(c, S1);
                let src = Addr(list).lw(c);
                c.g[V0][0] = src;
                c.g[S3][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[S0][0] = Addr(src as u32).off(0x20).lw(c);
                c.g[S2][0] = sx(list.wrapping_add(8));
                if c.g[S0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = Addr(list).off(4).lw(c);
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S4][0];
                        CLASSIFY.call(c, 0x0022_a3b4)?;
                        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[A2][0] = Addr(r32(c, S1)).off(4).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_a3a8);
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S3][0];
                c.g[S1][0] = c.g[S2][0];
                if (c.g[A2][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_a390);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S4)).off(HEAD).lw(c);
        c.g[A0][0] = c.g[S4][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            CLASSIFY.call(c, 0x0022_a3e4)?;
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_ReadBigEndian(const u8* p, int n)` (0x3c72f8, seg_2B4578.c):
/// reads an `n`-byte big-endian integer (a sound header tag field); 1-, 2-
/// and 3-byte fields are sign-extended, 4-byte ones taken as they are.
pub fn snd_read_big_endian(c: &mut Cpu) {
    c.cycles += 3;
    let n = c.g[A1][0];
    c.g[A3][0] = 0;
    c.g[A2][0] = sx(r32(c, A1).wrapping_sub(1));
    if n != 0 {
        c.cycles += 1;
        c.g[T0][0] = u64::MAX;
        loop {
            c.cycles += 7;
            let p = r32(c, A0);
            let byte = Addr(p).lbu(c);
            c.g[V1][0] = byte;
            c.g[V0][0] = sx(r32(c, A3) << 8);
            c.g[A0][0] = sx(p.wrapping_add(1));
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(byte as u32));
            if c.g[A2][0] == c.g[T0][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_7308);
            }
        }
    }
    'done: {
        'extend: {
            c.cycles += 3;
            if n == 1 {
                c.cycles += 3;
                if (c.g[A3][0] as i64) >= 128 {
                    c.cycles += 2;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x100));
                    break 'done;
                }
            }
            c.cycles += 2;
            if n == 2 {
                c.cycles += 4;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                if 0x7fff < (c.g[A3][0] as i64) {
                    break 'extend;
                }
                c.cycles += 1;
            }
            c.cycles += 2;
            if n != 3 {
                break 'done;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xffff_ffff_ff00_0000;
            if 0x7f_ffff >= (c.g[A3][0] as i64) {
                break 'done;
            }
        }
        c.cycles += 1;
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// `int BENewPlayerInt_CharProfileCall523e8(self, int player, int flag)`
/// (0x145e68, beintplayer.cpp): tests a character-profile flag of a
/// player's character: `BE_TestCharProfileFlag(&profiles[profile][char],
/// flag)` with the rider profile index (`BE_GetRiderProfile`), the player's
/// slot (`BENewPlayerInt_GetSlot`) and the slot's character id (signed byte
/// +0x11 of the 0x1c-byte player record at 0x534fe0); the profile table is
/// at 0x4a6ca8 (0x9b50 bytes per profile, 0xf88 per character).
pub fn be_new_player_int_char_profile_call523e8(c: &mut Cpu) {
    const PLAYERS: u32 = 0x0053_4fe0;
    const PROFILES: u32 = 0x004a_6ca8;
    const GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };
    const GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::f_00147798,
    };
    const TEST_FLAG: Callee = Callee {
        addr: 0x0015_23e8,
        f: ee::s01::f_001523e8,
    };
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        GET_RIDER_PROFILE.call(c, 0x0014_5e8c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        GET_SLOT.call(c, 0x0014_5e98)?;
        c.g[V1][0] = 0x1c;
        let slot = r32(c, V0);
        let record = abi::mult(c, slot, 0x1c);
        c.g[V0][0] = sx(record);
        c.g[A3][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        let profile_index = r32(c, S1);
        let profile = abi::mult(c, profile_index, 0x9b50);
        c.g[S1][0] = sx(profile);
        c.g[A2][0] = sx(PROFILES);
        c.g[A1][0] = c.g[S2][0];
        let record = record.wrapping_add(PLAYERS);
        c.g[V0][0] = sx(record);
        let character = Addr(record).off(0x11).lb(c);
        let character = abi::mult(c, character as u32, 0xf88);
        c.g[A0][0] = sx(profile.wrapping_add(character.wrapping_add(PROFILES)));
        TEST_FLAG.call(c, 0x0014_5ed8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_GetRiderRatingTier(self, Rider* rider)` (0x289c18,
/// ssxAudio.cpp): the rider's overall rating tier for speech selection:
/// asks the rider's controller (`rider + 0x6c0`, vtable slot 0x38) for its
/// character, rates it with `BECharInt_GetOverallRating` on the BE library's
/// interface 3, and returns 0 below 4, 1 below 8, else 2.
pub fn ssx_audio_system_get_rider_rating_tier(c: &mut Cpu) {
    const CONTROLLER: u32 = 0x6c0;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0028_9c30)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0014_dd58, 0x0028_9c3c)?; // BELibrary_GetInterface
        let controller = r32(c, S0).wrapping_add(CONTROLLER);
        let vt = Addr(controller).lw(c);
        c.g[V1][0] = vt;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(controller);
        let delta = Addr(vt as u32).off(0x38).lh(c);
        let method = Addr(vt as u32).off(0x3c).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx(controller.wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x0028_9c58)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_8950, 0x0028_9c64)?; // BECharInt_GetOverallRating
        let rating = c.g[V0][0] as i64;
        c.g[A0][0] = c.g[V0][0];
        if rating < 4 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            let below8 = rating < 8;
            c.g[A0][0] = below8 as u64;
            c.g[V1][0] = 1;
            c.g[V0][0] = if below8 { 1 } else { 2 };
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailModifier_v03(RailModifier* self)` (0x35c4e0, railmodifier.cpp):
/// the rail modifier's per-frame update: recomputes its bounds (+0x10 min,
/// +0x20 max) with `RailModifier_UpdateBounds` and moves its entry (id at
/// +0x44) in the world's spatial hash from the old box (copied to the stack
/// first) to the new one: `SpatialHash_Update(World_GetNash(), 2, id,
/// &self->box, &old)`.
pub fn rail_modifier_v03(c: &mut Cpu) {
    const BOX: u32 = 0x10;
    const ID: u32 = 0x44;
    const UPDATE_BOUNDS: Callee = Callee {
        addr: 0x0035_c040,
        f: ee::s11::f_0035c040,
    };
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let this = r32(c, S0);
        c.g[S1][0] = sx(this.wrapping_add(BOX));
        // The old box, through v1/v0, to sp+0 / sp+0x10.
        c.lq(V1, this.wrapping_add(BOX));
        c.lq(V0, this.wrapping_add(BOX + 0x10));
        let old = frame.at(c, 0);
        c.sq(V1, old);
        c.sq(V0, old.wrapping_add(0x10));
        UPDATE_BOUNDS.call(c, 0x0035_c50c)?;
        call_ptr(c, 0x002d_1be0, 0x0035_c514)?; // World_GetNash
        c.g[A2][0] = Addr(r32(c, S0)).off(ID).lw(c);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_91e0, 0x0035_c52c)?; // SpatialHash_Update
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_Update(SpeechSystem* self)` (0x2b0c78,
/// bankmonitor.cpp): updates both speech instances (`self->instances[i]`,
/// the array at +0x4; empty slots skipped) with `SpeechInstance_Update(inst,
/// i)`, then ticks the event contexts (`SpeechSystem_TickEventContexts`).
pub fn speech_system_update(c: &mut Cpu) {
    const INSTANCES: u32 = 0x4;
    const INSTANCE_UPDATE: Callee = Callee {
        addr: 0x002a_fc88,
        f: ee::s08::f_002afc88,
    };
    const TICK_EVENT_CONTEXTS: Callee = Callee {
        addr: 0x002b_1720,
        f: ee::s08::f_002b1720,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(r32(c, S1)).off(INSTANCES).lw(c);
        loop {
            c.cycles += 5;
            let at = (r32(c, S0) << 2).wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(at);
            c.g[A0][0] = Addr(at).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = c.g[S0][0];
                INSTANCE_UPDATE.call(c, 0x002b_0cb4)?;
            }
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.cycles += 3;
            let more = (c.g[S0][0] as i64) < 2;
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[V1][0] = Addr(r32(c, S1)).off(INSTANCES).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002b_0c98);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        TICK_EVENT_CONTEXTS.call(c, 0x002b_0ccc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_7348, app_vsync_tick, "App_VSyncTick"),
    (0x002a_f6c0, path_plat_set_volume, "PATH_PlatSetVolume"),
    (0x003c_7ab0, snd_ee_voice_mix_ramp, "SND_EeVoiceMixRamp"),
    (
        0x0022_a368,
        nash_classify_instances,
        "Nash_ClassifyInstances",
    ),
    (0x003c_72f8, snd_read_big_endian, "SND_ReadBigEndian"),
    (
        0x0014_5e68,
        be_new_player_int_char_profile_call523e8,
        "BENewPlayerInt_CharProfileCall523e8",
    ),
    (
        0x0028_9c18,
        ssx_audio_system_get_rider_rating_tier,
        "SSXAudioSystem_GetRiderRatingTier",
    ),
    (0x0035_c4e0, rail_modifier_v03, "RailModifier_v03"),
    (0x002b_0c78, speech_system_update, "SpeechSystem_Update"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{Rng, DATA};

    /// `App_VSyncTick`: an app with an accumulator 0..1 and a step 0..3.5
    /// (0..4 ticks), counter random; its vtable's `Tick` (slot 0x38) and the
    /// sub-object's slot 0x28 are fakes (logged, sometimes unwinding). Every
    /// fourth seed has a zero step (no tick). Deltas are random small
    /// even numbers.
    #[test]
    fn app_vsync_tick_matches() {
        const APP: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x2000;
        const SUBOBJ: u32 = DATA + 0x3000;
        const SUBVT: u32 = DATA + 0x4000;
        covered(ssx3_ee::s10::f_00317348__generated, app_vsync_tick, 300, &|c, seed| {
            let mut r: Rng = machine(c, seed, 0x317348, 6);
            c.pc = 0x0031_7348;
            c.g[A0][0] = sx(APP);
            let acc = (r.next_u32() % 1000) as f32 / 1000.0;
            let step = if seed % 4 == 0 {
                0.0
            } else {
                (r.next_u32() % 3500) as f32 / 1000.0
            };
            c.bus.write32(APP + 0x18, r.next_u32());
            c.bus.write32(APP + 0x24, step.to_bits());
            c.bus.write32(APP + 0x28, acc.to_bits());
            c.bus.write32(APP + 0x5c, VT);
            c.bus.write32(APP + 0x8, SUBOBJ);
            c.bus.write16(
                VT + 0x38,
                ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1,
            );
            c.bus.write32(VT + 0x3c, FAKE);
            c.bus.write32(SUBOBJ + 0x8, SUBVT);
            c.bus.write16(
                SUBVT + 0x28,
                ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1,
            );
            c.bus.write32(SUBVT + 0x2c, FAKE + 0x10);
        });
    }

    /// `PATH_PlatSetVolume`: path slot 0..23 of the table at 0x4d3e98 (the
    /// other slots empty) points at a record; the audio system (gp+0x410)
    /// is off on every sixth seed; the dirty flag is set on a third of the
    /// seeds (then the real `PATH_PlatRefreshStatus` runs its short path:
    /// stream handle negative); pending is -1, equal to or different from
    /// the current volume; the new volume is negative, equal to the
    /// current one or different; the current one is sometimes negative.
    /// `SND_Lock`/`SND_Unlock` run for real with their hook pointers
    /// (0x44c46c/0x44c470) at fakes; `SND_SetVolume` gets a negative channel
    /// handle and `SNDSTRM_GetStream` finds no stream (count 0), so both
    /// return early.
    #[test]
    fn path_plat_set_volume_matches() {
        use crate::difftest::GAME_GP;
        const PATH: u32 = DATA + 0x1000;
        const SOUND: u32 = DATA + 0x2000;
        const AUDIO: u32 = DATA + 0x10000;
        covered(
            ssx3_ee::s08::f_002af6c0__generated,
            path_plat_set_volume,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x2af6c0, 4);
                c.pc = 0x002a_f6c0;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0x410, AUDIO);
                c.bus.write32(
                    AUDIO + 0x6280,
                    if seed % 6 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let slot = r.next_u32() % 24;
                for i in 0..24 {
                    c.bus
                        .write32(0x004d_3e98 + 4 * i, if i == slot { PATH } else { 0 });
                }
                c.g[A0][0] = slot as u64 | (r.next_u32() as u64) << 32;
                // SND_Lock/SND_Unlock hooks, stream count.
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write8(0x0050_a92e, 0);
                let current = match seed % 5 {
                    0 => (r.next_u32() | 0x8000_0000) as i32,
                    _ => (r.next_u32() % 128) as i32,
                };
                let pending = match (seed / 5) % 3 {
                    0 => -1,
                    1 => current,
                    _ => current.wrapping_add(1 + (r.next_u32() % 50) as i32),
                };
                let volume = match (seed / 3) % 4 {
                    0 => -1 - (r.next_u32() % 100) as i32,
                    1 => current,
                    _ => (r.next_u32() % 128) as i32,
                };
                c.g[A1][0] = volume as i64 as u64;
                c.bus.write32(PATH, r.next_u32() | 0x8000_0000);
                c.bus.write32(PATH + 0x8, pending as u32);
                c.bus.write32(PATH + 0xc, current as u32);
                c.bus.write8(
                    PATH + 0x64,
                    (seed % 2) as u8 * (1 + r.next_u32() % 200) as u8,
                );
                c.bus.write8(PATH + 0x65, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(PATH + 0x6c, SOUND);
                c.bus.write32(SOUND + 4, r.next_u32() | 0x8000_0000);
            },
        );
    }

    /// `SND_EeVoiceMixRamp`: a voice whose source's render method is a
    /// fake (renders: positive pointer) or the real `return 0` leaf at
    /// 0x1406c0 (every fourth seed); the mixer state at 0x515b40 with 0..6
    /// sends, the dry send on or off, a 16-float sample block and buffers
    /// in DATA, the `set` callback a fake. Levels per send: changed
    /// (the real `SND_MixAddRamp16` ramps), unchanged non-zero (`set`),
    /// unchanged zero (nothing).
    #[test]
    fn snd_ee_voice_mix_ramp_matches() {
        const VOICE: u32 = DATA + 0x1000;
        const SOURCE: u32 = DATA + 0x1100;
        const BLOCK: u32 = DATA + 0x1200;
        const BUFFERS: u32 = DATA + 0x2000;
        covered(
            ssx3_ee::s14::f_003c7ab0__generated,
            snd_ee_voice_mix_ramp,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3c7ab0, 8);
                c.pc = 0x003c_7ab0;
                c.g[A0][0] = sx(VOICE);
                c.bus.write32(VOICE + 0x40, SOURCE);
                c.bus
                    .write32(SOURCE, if seed % 4 == 0 { 0x0014_06c0 } else { FAKE });
                let state = ee_mix::STATE;
                let n = r.next_u32() % 7;
                c.bus.write8(state + 0x5, n as u8);
                c.bus.write8(state + 0xf, (r.next_u32() % 3) as u8);
                c.bus.write32(state + 0xb0, r.next_u32());
                c.bus.write32(state + 0xb4, BLOCK);
                c.bus.write32(state + 0x1e0, FAKE + 0x10);
                for i in 0..7 {
                    c.bus
                        .write32(state + 0xd0 + 4 * i, BUFFERS + 0x100 * (i + 1));
                }
                for i in 0..16 {
                    c.bus.write32(BLOCK + 4 * i, r.f32(1.0));
                    c.bus.write32(ee_mix::DRY_BUFFER + 4 * i, r.f32(100.0));
                    for k in 1..8 {
                        c.bus.write32(BUFFERS + 0x100 * k + 4 * i, r.f32(100.0));
                    }
                }
                // Levels: (cur, want) pairs for the dry send and 6 sends.
                let level = |r: &mut Rng| -> (u32, u32) {
                    match r.next_u32() % 4 {
                        0 => (0, 0),
                        1 => {
                            let v = r.f32(1.0);
                            (v, v)
                        }
                        2 => (0x8000_0000, 0),
                        _ => (r.f32(1.0), r.f32(1.0)),
                    }
                };
                let (cur, want) = level(&mut r);
                c.bus.write32(VOICE + 0x34, cur);
                c.bus.write32(VOICE + 0x38, want);
                for i in 0..6 {
                    let (cur, want) = level(&mut r);
                    c.bus.write32(VOICE + 0x4 + 4 * i, cur);
                    c.bus.write32(VOICE + 0x1c + 4 * i, want);
                }
            },
        );
    }

    /// `Nash_ClassifyInstances`: 0..4 lists (or a negative count), each
    /// source with 0..4 nodes; the real `Nash_ClassifyInstance` runs its
    /// short paths: node type words not ending in 3 (a miss), or ending in
    /// 3 with a heap slot word that is not 6 (a miss after the heap look-up);
    /// the trailing null-node call (instance head at +0x50a4 set on half the
    /// seeds) reads VU0 VI1 = 5..7 and stores the null node at the head.
    #[test]
    fn nash_classify_instances_matches() {
        use crate::difftest::check;
        const CELL: u32 = DATA;
        const HEAPPTR: u32 = DATA + 0x6000;
        const HEAP: u32 = DATA + 0x6100;
        const LISTS: u32 = DATA + 0x8000;
        const SRCS: u32 = DATA + 0x8100;
        const NODES: u32 = DATA + 0x9000;
        check(
            ssx3_ee::s05::f_0022a368__generated,
            nash_classify_instances,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x22a368, 0);
                c.pc = 0x0022_a368;
                c.bus.plat.write_ctrl(1, 5 + seed % 3);
                c.g[A0][0] = sx(CELL);
                c.bus.write32(
                    CELL + 0x50a4,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(CELL + 0x90, HEAPPTR);
                c.bus.write32(HEAPPTR, HEAP);
                for i in 0..0x40 {
                    c.bus.write32(HEAP + 4 * i, r.next_u32() & !2);
                }
                let n = r.next_u32() % 5;
                c.g[A1][0] = sx(LISTS);
                c.g[A2][0] = if seed % 13 == 5 {
                    sx(0xffff_fffd)
                } else {
                    n as u64
                };
                let mut node_at = NODES;
                for i in 0..n {
                    let src = SRCS + 0x40 * i;
                    c.bus.write32(LISTS + 8 * i, src);
                    c.bus.write32(LISTS + 8 * i + 4, r.next_u32());
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 5 {
                        let node = node_at;
                        node_at += 0x100;
                        c.bus.write32(node, next);
                        let kind = r.next_u32();
                        c.bus
                            .write32(node + 8, if kind % 3 == 0 { kind | 3 } else { kind & !1 });
                        c.bus.write8(node + 0x7d, (r.next_u32() % 6) as u8);
                        next = node;
                    }
                    c.bus.write32(src + 0x20, next);
                }
            },
        );
    }

    /// `SND_ReadBigEndian`: 0..4 random bytes (high bit set or not, so the
    /// sign extension of 1..3-byte fields goes both ways).
    #[test]
    fn snd_read_big_endian_matches() {
        use crate::difftest::{check, scramble};
        check(
            ssx3_ee::s13::f_003c72f8__generated,
            snd_read_big_endian,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c72_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_72f8;
                let at = DATA + (r.next_u32() & 0xff);
                c.g[A0][0] = sx(at);
                c.g[A1][0] = (seed % 5) as u64;
                for i in 0..4 {
                    let b = r.next_u32() as u8;
                    c.bus.write8(
                        at + i,
                        if (seed / 5) % 2 == 0 {
                            b & 0x7f
                        } else {
                            b | 0x80
                        },
                    );
                }
            },
        );
    }

    /// `BENewPlayerInt_CharProfileCall523e8`: player 0..7 whose slot (table
    /// 0x5305b0) is 0..3; the slot's rider record (0x535b20) is unused
    /// (+0xc = -1: profile 2) or used (profile = +0x10 & 1); the character
    /// byte 0..9; flag index 0..2 into the mask table at 0x45a7c0; random
    /// profile flag words. All three callees are real leaves.
    #[test]
    fn be_new_player_int_char_profile_call523e8_matches() {
        use crate::difftest::{check, scramble};
        check(
            ssx3_ee::s01::f_00145e68__generated,
            be_new_player_int_char_profile_call523e8,
            300,
            &|c, seed| {
                let mut r = Rng(0x145e_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0014_5e68;
                let player = r.next_u32() % 8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = player as u64;
                c.g[A2][0] = (seed % 3) as u64;
                for i in 0..8 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 4);
                }
                for slot in 0..4 {
                    let rec = 0x0053_5b20 + 0x1c * slot;
                    c.bus.write32(
                        rec + 0xc,
                        if r.next_u32() % 3 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32() >> 1
                        },
                    );
                    c.bus.write32(rec + 0x10, r.next_u32());
                    c.bus
                        .write8(0x0053_4fe0 + 0x1c * slot + 0x11, (r.next_u32() % 10) as u8);
                }
                for i in 0..3 {
                    c.bus.write32(0x0045_a7c0 + 4 * i, 1 << (r.next_u32() % 32));
                }
                for profile in 0..3 {
                    for ch in 0..10 {
                        c.bus.write32(
                            0x004a_6ca8 + 0x9b50 * profile + 0xf88 * ch + 0xacc,
                            r.next_u32(),
                        );
                    }
                }
            },
        );
    }

    /// `SSXAudioSystem_GetRiderRatingTier`: the BE library dummy at gp-0x1f04
    /// (so `BELibrary_GetInstance` returns it), interface table 0x5308a8
    /// indexed by gp-0x1efc whose object's `GetInterface` (+0x8) and the
    /// rider controller's slot 0x38 are fakes (the latter forced to return a
    /// rider 1..7); the real `BECharInt_GetOverallRating` on the rider
    /// tables of its own test (ratings below 4, 4..7 and 8 up).
    #[test]
    fn ssx_audio_system_get_rider_rating_tier_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::{fill, GAME_GP};
        const BELIB: u32 = DATA + 0x1000;
        const IFACE_OBJ: u32 = DATA + 0x1100;
        const IFACE_VT: u32 = DATA + 0x1200;
        const RIDER: u32 = DATA + 0x2000;
        const CTRL_VT: u32 = DATA + 0x3000;
        covered(
            ssx3_ee::s07::f_00289c18__generated,
            ssx_audio_system_get_rider_rating_tier,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x289c18, 2);
                c.pc = 0x0028_9c18;
                c.bus.write32(GAME_GP - 0x1f04, BELIB);
                let idx = r.next_u32() % 3;
                c.bus.write32(GAME_GP - 0x1efc, idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, IFACE_OBJ);
                c.bus.write32(IFACE_OBJ + 4, IFACE_VT);
                c.bus.write16(IFACE_VT + 0x8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(IFACE_VT + 0xc, FAKE);
                c.g[A0][0] = sx(DATA + 0x5000);
                c.g[A1][0] = sx(RIDER);
                c.bus.write32(RIDER + 0x6c0, CTRL_VT);
                c.bus.write16(CTRL_VT + 0x38, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(CTRL_VT + 0x3c, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS + 4, 1 + r.next_u32() % 7);
                // BECharInt_GetOverallRating's tables.
                for i in 0..8u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                for i in 0..8u32 {
                    let p = 0x0053_5b20 + 0x1c * i;
                    fill(c, &mut r, p, 7);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(p + 0xc, 0xffff_ffff);
                    }
                    c.bus.write8(p + 0x11, (r.next_u32() % 10) as u8);
                }
                let scale = [3u8, 7, 15, 31][(seed % 4) as usize];
                for i in 0..0x100u32 {
                    let v = r.next_u32() as u8 % (scale + 1);
                    c.bus.write8(0x0053_5538 + i, v);
                }
            },
        );
    }

    /// `RailModifier_v03`: the real `RailModifier_UpdateBounds` asks the
    /// rail object's vtable (slot 0x160: the real `return 0` leaf on every
    /// third seed, so the box stays, else a fake) for the new box (slot
    /// 0x168, a fake forced to return a point box P); the old box is
    /// P -/+ 100 (integer coordinates, exact), so the grown new box equals it
    /// and the real `SpatialHash_Update` (nash from the world chain
    /// gp-0x848 -> +0x84 -> +0x20) finds the keys unchanged; the key's grid
    /// constants at gp-0x2df8/-0x2df4 are set as in its own test.
    #[test]
    fn rail_modifier_v03_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::{put_vec, GAME_GP};
        const THIS: u32 = DATA;
        const RAIL: u32 = DATA + 0x100;
        const OBJ: u32 = DATA + 0x200;
        const VT: u32 = DATA + 0x400;
        const POINT: u32 = DATA + 0x800;
        const APP: u32 = DATA + 0x900;
        const GAME: u32 = DATA + 0xa00;
        covered(
            ssx3_ee::s11::f_0035c4e0__generated,
            rail_modifier_v03,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x35c4e0, 3);
                c.pc = 0x0035_c4e0;
                c.g[A0][0] = sx(THIS);
                c.bus.write32(THIS + 0x40, RAIL);
                c.bus.write32(THIS + 0x44, r.next_u32());
                c.bus.write32(RAIL + 0xc, OBJ);
                c.bus.write32(OBJ + 0xc, VT);
                c.bus.write16(VT + 0x160, (r.next_u32() & 0x7c) as u16);
                c.bus
                    .write32(VT + 0x164, if seed % 3 == 0 { 0x0014_06c0 } else { FAKE });
                c.bus.write16(VT + 0x168, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT + 0x16c, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS + 4, POINT);
                let p: Vec<f32> = (0..3)
                    .map(|_| (r.next_u32() % 2000) as f32 - 1000.0)
                    .collect();
                let one = 1.0f32.to_bits();
                put_vec(
                    c,
                    POINT,
                    [p[0].to_bits(), p[1].to_bits(), p[2].to_bits(), one],
                );
                put_vec(
                    c,
                    POINT + 0x10,
                    [p[0].to_bits(), p[1].to_bits(), p[2].to_bits(), one],
                );
                let lo = |v: f32| (v - 100.0).to_bits();
                let hi = |v: f32| (v + 100.0).to_bits();
                put_vec(c, THIS + 0x10, [lo(p[0]), lo(p[1]), lo(p[2]), one]);
                put_vec(c, THIS + 0x20, [hi(p[0]), hi(p[1]), hi(p[2]), one]);
                c.bus.write32(GAME_GP - 0x848, APP);
                c.bus.write32(APP + 0x84, GAME);
                c.bus.write32(GAME + 0x20, r.next_u32());
                // `SpatialKey_FromAABB`'s grid constants (scale 1.0, slack 0.25);
                // zero RAM there makes its level search never end (pc 0x329070).
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x2df8), 1.0f32.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x2df4), 0.25f32.to_bits());
            },
        );
    }

    /// `SpeechSystem_Update`: two instance slots, each empty or an instance
    /// (stream index 0..3, "started" flag 0..3) for the real
    /// `SpeechInstance_Update`: the speech listener (gp+0x6c4) has a stream
    /// array with no buffers (state 0: the instance is ready), its vtable
    /// slot 0x20 is a fake, `SPCH_ServiceChannel` is busy (gp+0x175c set)
    /// and `SPCH_DequeueChannel` finds an empty channel table (0x44fff8), so
    /// all return early; the real `SpeechSystem_TickEventContexts` counts
    /// down ten random contexts.
    #[test]
    fn speech_system_update_matches() {
        use crate::difftest::GAME_GP;
        const SYS: u32 = DATA;
        const SLOTS: u32 = DATA + 0x400;
        const INST: u32 = DATA + 0x1000;
        const LISTENER: u32 = DATA + 0x4000;
        const ARRAY: u32 = DATA + 0x4400;
        const BUFFERS: u32 = DATA + 0x4500;
        const LVT: u32 = DATA + 0x4600;
        covered(
            ssx3_ee::s08::f_002b0c78__generated,
            speech_system_update,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x2b0c78, 2);
                c.pc = 0x002b_0c78;
                c.g[A0][0] = sx(SYS);
                c.bus.write32(SYS + 4, SLOTS);
                for i in 0..2u32 {
                    let inst = INST + 0x1000 * i;
                    let present = (seed >> i) % 4 != 0;
                    c.bus.write32(SLOTS + 4 * i, if present { inst } else { 0 });
                    c.bus.write32(inst + 4, r.next_u32() % 4);
                    c.bus.write32(inst + 0x900, r.next_u32() % 4);
                    c.bus.write32(0x0044_fff8 + 4 * i, 0);
                }
                c.bus.write32(GAME_GP + 0x6c4, LISTENER);
                c.bus.write32(GAME_GP + 0x175c, 1);
                c.bus.write32(LISTENER, ARRAY);
                c.bus.write32(ARRAY + 8, BUFFERS);
                for k in 0..4 {
                    c.bus.write32(BUFFERS + 4 * k, 0);
                }
                c.bus.write32(LISTENER + 0x1bc, LVT);
                c.bus.write16(LVT + 0x20, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(LVT + 0x24, FAKE);
                for k in 0..10 {
                    let ctx = SYS + 0x18 + 0x20 * k;
                    c.bus.write32(ctx, r.next_u32() % 2);
                    c.bus.write32(ctx + 4, (r.next_u32() % 4).wrapping_sub(1));
                }
            },
        );
    }
}
