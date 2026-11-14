//! Staging module oc373 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc373`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0029_1c88,
    ssx_audio_system_start_board_surface_loop,
    "SSXAudioSystem_StartBoardSurfaceLoop",
)];

/// Rider fields `SSXAudioSystem_StartBoardSurfaceLoop` reads: listener
/// index, is-human, is-local, and the sound source (`&rider->mPosition`).
pub(crate) const SBSL_RIDER_VIEW: u32 = 0x870;
pub(crate) const SBSL_RIDER_HUMAN: u32 = 0x874;
pub(crate) const SBSL_RIDER_LOCAL: u32 = 0x87c;
pub(crate) const SBSL_RIDER_SOURCE: u32 = 0x110;

/// `SSXAudioSystem` fields: the bank monitor, the push bank / sample
/// tables, the sound root (`*root` = the monitor that plays).
const SBSL_SYS_MONITOR: u32 = 0x0;
const SBSL_SYS_PUSH_BANK: u32 = 0x4;
const SBSL_SYS_PUSH_SAMPLE: u32 = 0x8;
const SBSL_SYS_SND_ROOT: u32 = 0x118;

/// Bank monitor: the push queue (`+0x1d8`) and its entry count (`+0x1f4`).
const SBSL_PUSH_QUEUE: u32 = 0x1d8;
const SBSL_PUSH_COUNT: u32 = 0x1f4;

/// The audio mode byte at 0x535c11 (2: record the replay listener).
pub(crate) const SBSL_AUDIO_MODE: u32 = 0x0053_5c11;

/// `lw base+off` on whole registers.
#[inline(always)]
fn sbsl_lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `root = sys->sndRoot; mon = *root; table = mon->table[mon->count] =
/// value`, in the registers the compiled code uses.
fn sbsl_root_store(
    c: &mut Cpu,
    (root, mon, idx, tab): (usize, usize, usize, usize),
    table: u32,
    value: u64,
) {
    c.g[root][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[mon][0] = sbsl_lw(c, c.g[root][0], 0);
    c.g[idx][0] = sbsl_lw(c, c.g[mon][0], SBSL_PUSH_COUNT);
    c.g[tab][0] = sbsl_lw(c, c.g[mon][0], table);
    c.g[idx][0] = sx((c.g[idx][0] as u32) << 2);
    c.g[idx][0] = sx((c.g[idx][0] as u32).wrapping_add(c.g[tab][0] as u32));
    Addr(c.g[idx][0] as u32).sw(c, value);
}

/// `monitor->queue.table[count] (scaled by shift) + extra = value`, with
/// `s0` = monitor and `s1` = its queue; `ri`/`rt` hold index and table.
fn sbsl_queue_store(
    c: &mut Cpu,
    (ri, rt): (usize, usize),
    table: u32,
    shift: u32,
    extra: u32,
    value: u64,
) {
    c.g[ri][0] = sbsl_lw(c, c.g[S0][0], SBSL_PUSH_COUNT);
    c.g[rt][0] = sbsl_lw(c, c.g[S1][0], table);
    c.g[ri][0] = sx((c.g[ri][0] as u32) << shift);
    c.g[ri][0] = sx((c.g[ri][0] as u32).wrapping_add(c.g[rt][0] as u32));
    Addr((c.g[ri][0] as u32).wrapping_add(extra)).sw(c, value);
}

/// `void SSXAudioSystem_StartBoardSurfaceLoop(SSXAudioSystem* sys, Rider*
/// rider, int surface)` (0x291c88): start the looping board-on-snow sound
/// of a surface. A new entry goes on the bank monitor's push queue: bank
/// `GetSurfaceBank(surface)`, sample `surface * 8 + 4` for a human local
/// rider (`+ 6` for others), volume 100 clamped to 0..127, pan 90, 50, 127,
/// gain channel 6, the rider as the source, radius 100 / vanish -1, and a
/// loop key (0x4a3690 local, 0x4a3698 remote). For the local human it also
/// sets the listener (`GetRiderListenerIndex`) and, in audio mode 2 outside
/// replays, the replay flag and the rider's listener index. Then
/// `Audio_PlayPushedSound(sys)`.
pub fn ssx_audio_system_start_board_surface_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0;
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = c.g[A2][0]; // surface
        frame.save(c, S6, 0x30);
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A1][0]; // rider
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = sbsl_lw(c, c.g[S6][0], SBSL_RIDER_HUMAN);
        c.g[S5][0] = c.g[A0][0]; // sys
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sbsl_lw(c, c.g[S6][0], SBSL_RIDER_LOCAL);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let local_human = c.g[V1][0] != 0;
        c.g[A0][0] = c.g[S5][0];
        if local_human {
            sbsl_local(c)?;
        } else {
            sbsl_remote(c)?;
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 10;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The local human rider's loop: s7 = record the replay listener (audio
/// mode 2 and no replay playing).
fn sbsl_local(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 10;
    c.g[S7][0] = 0;
    // BELibrary_GetInterface(BELibrary_GetInstance(), 0)
    call_ptr(c, 0x0014_dc80, 0x0029_1ce0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0029_1cec)?;
    c.g[V1][0] = 0x53_0000;
    c.g[A0][0] = 0x2;
    c.g[V0][0] = Addr(SBSL_AUDIO_MODE).lb(c);
    if c.g[V0][0] != c.g[A0][0] {
        c.g[A0][0] = c.g[S5][0];
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S5][0];
        // SSXAudioSystem_IsReplayPlaying(sys)
        call_ptr(c, 0x0028_8ae0, 0x0029_1d08)?;
        c.g[S7][0] = (c.g[V0][0] < 1) as u64;
        c.g[A0][0] = c.g[S5][0];
    }
    c.cycles += 159;
    sbsl_push_entry(c, 4, [0x0029_1d18, 0x0029_1df8])?;

    c.cycles += 69;
    Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
    // monitor->gain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 6, 0)
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = 0x6;
    c.g[S0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    call_ptr(c, 0x0028_7968, 0x0029_1fe8)?;
    c.g[A3][0] = sbsl_lw(c, c.g[S0][0], 0);
    c.g[T0][0] = sx((c.g[S6][0] as u32).wrapping_add(SBSL_RIDER_SOURCE));
    c.g[T1][0] = 0x4a_0000;
    c.g[A0][0] = c.g[S5][0];
    c.g[V1][0] = sbsl_lw(c, c.g[A3][0], SBSL_PUSH_COUNT);
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = sbsl_lw(c, c.g[A3][0], 0x214);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // monitor->source[count] = &rider->position (inline array at +0x1fc).
    c.g[A2][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[V1][0] = sbsl_lw(c, c.g[A2][0], 0);
    c.g[V0][0] = sbsl_lw(c, c.g[V1][0], SBSL_PUSH_COUNT);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr((c.g[V1][0] as u32).wrapping_add(0x1fc)).sw(c, c.g[T0][0]);
    // queue->key[count] = the local loop key (8 bytes at 0x4a3690).
    c.g[V0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[A2][0] = sbsl_lw(c, c.g[V0][0], 0);
    c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x3690));
    sbsl_copy_key(c, T0, A2, V0, V0);
    // monitor->object[count] = rider
    let rider = c.g[S6][0];
    sbsl_root_store(c, (A2, V1, V0, A2), 0x238, rider);
    // monitor->listener[count] = SSXAudioSystem_GetRiderListenerIndex(sys, rider)
    c.g[S0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    call_ptr(c, 0x0028_8a20, 0x0029_2088)?;
    c.g[A0][0] = sbsl_lw(c, c.g[S0][0], 0);
    c.g[AT][0] = 0x42c8_0000; // 100.0
    c.mtc1(AT, 12);
    c.g[V1][0] = sbsl_lw(c, c.g[A0][0], SBSL_PUSH_COUNT);
    c.g[A1][0] = sbsl_lw(c, c.g[A0][0], 0x23c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 13);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    sbsl_root_store(c, (A0, V1, V0, A0), 0x254, 0);
    // SndPlayParams_SetRadiusVanish(&root->queue, 100.0, -1.0)
    c.g[V1][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[A0][0] = sbsl_lw(c, c.g[V1][0], 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SBSL_PUSH_QUEUE));
    call_ptr(c, 0x002a_9988, 0x0029_20dc)?;
    c.g[A1][0] = 0x1;
    if c.g[S7][0] != 0 {
        // Live play of the local human in mode 2: replay flag and listener.
        c.cycles += 15;
        sbsl_root_store(c, (A0, V1, V0, A0), 0x258, 1);
        c.g[A1][0] = sbsl_lw(c, c.g[S6][0], SBSL_RIDER_VIEW);
        let view = c.g[A1][0];
        sbsl_root_store(c, (A0, V1, V0, A0), 0x25c, view);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S5][0];
    // Audio_PlayPushedSound(sys)
    call_ptr(c, 0x0029_06b8, 0x0029_2128)
}

/// Another rider's loop (no listener, no replay record).
fn sbsl_remote(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 159;
    sbsl_push_entry(c, 6, [0x0029_2138, 0x0029_2218])?;

    c.cycles += 60;
    Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
    // monitor->gain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 6, 0)
    c.g[A1][0] = 0x6;
    c.g[A0][0] = c.g[S5][0];
    c.g[A2][0] = 0;
    c.g[S0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    call_ptr(c, 0x0028_7968, 0x0029_2408)?;
    c.g[A1][0] = sbsl_lw(c, c.g[S0][0], 0);
    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(SBSL_RIDER_SOURCE));
    c.g[A3][0] = 0x4a_0000;
    c.g[AT][0] = 0x42c8_0000; // 100.0
    c.mtc1(AT, 12);
    c.g[V1][0] = sbsl_lw(c, c.g[A1][0], SBSL_PUSH_COUNT);
    c.g[A0][0] = sbsl_lw(c, c.g[A1][0], 0x214);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 13);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // monitor->source[count] = &rider->position (inline array at +0x1fc).
    c.g[A0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[V1][0] = sbsl_lw(c, c.g[A0][0], 0);
    c.g[V0][0] = sbsl_lw(c, c.g[V1][0], SBSL_PUSH_COUNT);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr((c.g[V1][0] as u32).wrapping_add(0x1fc)).sw(c, c.g[A2][0]);
    // queue->key[count] = the remote loop key (8 bytes at 0x4a3698).
    c.g[V0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[A0][0] = sbsl_lw(c, c.g[V0][0], 0);
    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3698));
    sbsl_copy_key(c, A2, A0, V0, A3);
    sbsl_root_store(c, (A0, V1, V0, A0), 0x254, 0);
    let rider = c.g[S6][0];
    sbsl_root_store(c, (V1, A0, V0, V1), 0x238, rider);
    // SndPlayParams_SetRadiusVanish(&root->queue, 100.0, -1.0)
    c.g[V1][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_SND_ROOT);
    c.g[A0][0] = sbsl_lw(c, c.g[V1][0], 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SBSL_PUSH_QUEUE));
    call_ptr(c, 0x002a_9988, 0x0029_24d4)?;
    c.g[A0][0] = c.g[S5][0];
    // Audio_PlayPushedSound(sys)
    call_ptr(c, 0x0029_06b8, 0x0029_24dc)
}

/// Copy the 8-byte loop key at `src` (through the stack temp at sp+0) into
/// `queue.key[count]` (`monitor + 0x1d8 + 0x40 + count * 8`); `mon` holds
/// the monitor, `tmp` carries the first copy, `out` the second.
fn sbsl_copy_key(c: &mut Cpu, src: usize, mon: usize, tmp: usize, out: usize) {
    let from = c.g[src][0] as u32;
    let stack = sp(c);
    c.ldl(tmp, from.wrapping_add(0x7));
    c.ldr(tmp, from);
    c.sdl(tmp, stack.wrapping_add(0x7));
    c.sdr(tmp, stack);
    c.g[V1][0] = sx((c.g[mon][0] as u32).wrapping_add(SBSL_PUSH_QUEUE));
    c.g[V0][0] = sbsl_lw(c, c.g[mon][0], SBSL_PUSH_COUNT);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(out, stack.wrapping_add(0x7));
    c.ldr(out, stack);
    let key = c.g[V1][0] as u32;
    c.sdl(out, key.wrapping_add(0x47));
    c.sdr(out, key.wrapping_add(0x40));
}

/// The push-queue entry both paths share: `s4` = GetSurfaceBank(surface),
/// `s2` = surface * 8 + `sample`; count += 1, copy the 24-byte template,
/// clear the entry's tables, memset its 8-byte slot, set volume 100 and pan
/// 90, 50, 127, flag 1, store bank and sample in the system's tables, then
/// the root monitor's volume 0 and the clamped volume byte. Leaves v0 = the
/// byte and a0 = where it goes.
fn sbsl_push_entry(c: &mut Cpu, sample: u32, rets: [u32; 2]) -> crate::call::Flow {
    c.g[A1][0] = c.g[S2][0];
    // SSXAudioSystem_GetSurfaceBank(sys, surface)
    call_ptr(c, 0x0029_17b8, rets[0])?;
    c.g[S3][0] = 0x18; // record size
    c.g[S0][0] = sbsl_lw(c, c.g[S5][0], SBSL_SYS_MONITOR);
    c.g[S4][0] = c.g[V0][0];
    c.g[S2][0] = sx((c.g[S2][0] as u32) << 3);
    c.g[A1][0] = 0;
    let monitor = c.g[S0][0] as u32;
    c.g[V0][0] = Addr(monitor).off(SBSL_PUSH_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(SBSL_PUSH_QUEUE));
    let queue = c.g[S1][0] as u32;
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(sample));
    c.g[A2][0] = 0x8;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(monitor).off(SBSL_PUSH_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(SBSL_PUSH_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, V1, V1, S3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(V0, monitor.wrapping_add(0x1e3));
    c.ldr(V0, monitor.wrapping_add(0x1dc));
    c.ldl(A0, monitor.wrapping_add(0x1eb));
    c.ldr(A0, monitor.wrapping_add(0x1e4));
    c.ldl(A3, monitor.wrapping_add(0x1f3));
    c.ldr(A3, monitor.wrapping_add(0x1ec));
    let record = c.g[V1][0] as u32;
    c.sdl(V0, record.wrapping_add(0x7));
    c.sdr(V0, record);
    c.sdl(A0, record.wrapping_add(0xf));
    c.sdr(A0, record.wrapping_add(0x8));
    c.sdl(A3, record.wrapping_add(0x17));
    c.sdr(A3, record.wrapping_add(0x10));
    // monitor->source[count] = 0 (inline array at +0x1fc).
    c.g[V0][0] = Addr(monitor).off(SBSL_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
    Addr((c.g[V0][0] as u32).wrapping_add(0x1fc)).sw(c, 0);
    sbsl_queue_store(c, (V1, V0), 0x34, 2, 0, 0);
    // table 0x38 [count] = the record's first byte (sign-extended).
    c.g[A3][0] = Addr(monitor).off(SBSL_PUSH_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(SBSL_PUSH_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S3);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    sbsl_queue_store(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8)
    c.g[A0][0] = Addr(monitor).off(SBSL_PUSH_COUNT).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, rets[1])?;

    // The entry's parameters: volume 100, pan 90, 50, 127, flag 1.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    c.g[T0][0] = 0x1;
    sbsl_queue_store(c, (V1, V0), 0x60, 2, 0, 0);
    sbsl_queue_store(c, (V0, V1), 0x64, 2, 0, 0);
    sbsl_queue_store(c, (V1, V0), 0x68, 2, 0, 0);
    sbsl_queue_store(c, (V0, V1), 0x6c, 2, 0, 0);
    sbsl_queue_store(c, (V1, V0), 0x70, 4, 0x4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    sbsl_queue_store(c, (V0, V1), 0x70, 4, 0x0, a0);
    sbsl_queue_store(c, (V1, V0), 0x70, 4, 0x8, a1);
    sbsl_queue_store(c, (V0, V1), 0x70, 4, 0xc, a2);
    sbsl_queue_store(c, (V1, V0), 0x74, 2, 0, 0);
    sbsl_queue_store(c, (V0, V1), 0x80, 2, 0, 0);
    sbsl_queue_store(c, (V1, V0), 0x84, 2, 0, 0);
    sbsl_queue_store(c, (V0, V1), 0x78, 2, 0, a3);
    sbsl_queue_store(c, (V1, V0), 0x7c, 2, 0, t0);

    // sys->pushBank[count] = bank (s4); sys->pushSample[count] = sample (s2).
    let sys = c.g[S5][0];
    c.g[A0][0] = sbsl_lw(c, sys, SBSL_SYS_MONITOR);
    c.g[V1][0] = sbsl_lw(c, sys, SBSL_SYS_PUSH_BANK);
    c.g[V0][0] = sbsl_lw(c, c.g[A0][0], SBSL_PUSH_COUNT);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S4][0]);
    c.g[V1][0] = sbsl_lw(c, sys, SBSL_SYS_MONITOR);
    c.g[A0][0] = sbsl_lw(c, sys, SBSL_SYS_PUSH_SAMPLE);
    c.g[V0][0] = sbsl_lw(c, c.g[V1][0], SBSL_PUSH_COUNT);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S2][0]);

    // Root monitor: volume[count] = 0, then the record's volume byte =
    // clamp(volume[count], 0, 127).
    c.g[V1][0] = sbsl_lw(c, sys, SBSL_SYS_SND_ROOT);
    c.g[A1][0] = sbsl_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = sbsl_lw(c, c.g[A1][0], SBSL_PUSH_COUNT);
    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(SBSL_PUSH_QUEUE));
    c.g[V1][0] = sbsl_lw(c, c.g[A2][0], 0x38);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, 0);
    c.g[A0][0] = sbsl_lw(c, c.g[A1][0], SBSL_PUSH_COUNT);
    c.g[V1][0] = sbsl_lw(c, c.g[A2][0], 0x1c);
    c.g[A3][0] = sbsl_lw(c, c.g[A2][0], 0x38);
    c.mult(0, A0, A0, S3);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = sbsl_lw(c, c.g[A2][0], 0x20);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = sbsl_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    // The current volume word: queue->volume[queue->count].
    let vol_at = |c: &mut Cpu| -> u64 {
        let n = sbsl_lw(c, c.g[A2][0], 0x1c);
        sx(((n as u32) << 2).wrapping_add(c.g[A3][0] as u32))
    };
    let mut clamp_top = true;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = sbsl_lw(c, c.g[V0][0], 0);
        c.g[V0][0] = 0;
        clamp_top = (c.g[V1][0] as i64) >= 0;
    }
    if clamp_top {
        c.cycles += 7;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = sbsl_lw(c, c.g[V0][0], 0);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = vol_at(c);
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `SSXAudioSystem_StartBoardSurfaceLoop` on the shared audio world of
    /// the soundsys tests (real BELibrary lookup with a fake interface
    /// method, IsReplayPlaying, GetSurfaceBank, memset, gain lookup,
    /// GetRiderListenerIndex, SetRadiusVanish and `Audio_PlayPushedSound`
    /// with no bank). Covers: human local rider or not (both loop paths),
    /// audio mode 2 or not, in / out of replays (the replay record), in game
    /// or not, surfaces -1..20 (both banks), 0..2 entries already queued;
    /// some seeds unwind at a fake call.
    #[test]
    fn ssx_audio_system_start_board_surface_loop_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::{FAKE, LOG_UNWIND_AT};
        use crate::difftest::{DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::audio;
        const OC373_SBSL_IFACE: u32 = DATA + 0x26_1000;
        const OC373_SBSL_IFACE_VT: u32 = DATA + 0x26_1100;
        check_lohi(
            ee::s07::f_00291c88__generated,
            ssx_audio_system_start_board_surface_loop,
            200,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_1c88, 0x0029_1c88);
                // BELibrary_GetInterface: interface table entry 0 with a
                // fake method.
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, OC373_SBSL_IFACE);
                c.bus.write32(OC373_SBSL_IFACE + 4, OC373_SBSL_IFACE_VT);
                c.bus.write16(OC373_SBSL_IFACE_VT + 8, 0);
                c.bus.write32(OC373_SBSL_IFACE_VT + 0xc, FAKE + 0x40);
                c.bus
                    .write8(SBSL_AUDIO_MODE, if seed % 5 < 3 { 2 } else { 1 });
                c.bus
                    .write32(audio::RIDER + SBSL_RIDER_VIEW, r.next_u32() % 3);
                c.g[A2][0] = sx((r.next_u32() % 22).wrapping_sub(1));
                if seed % 10 == 7 {
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 3);
                }
            },
        );
    }
}
