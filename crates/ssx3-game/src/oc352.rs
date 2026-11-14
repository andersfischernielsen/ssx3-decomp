//! Staging module oc352 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc352`.

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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0029_ab40,
    ssx_audio_system_update_boost_meter_sounds,
    "SSXAudioSystem_UpdateBoostMeterSounds",
)];

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateBoostMeterSounds

/// `SSXAudioSystem`: the last boost-meter level (0..10) per rider slot
/// (`int[2]`).
pub(crate) const BOOST_METER_LAST_LEVEL: u32 = 0x5fe0;
/// The pitch word of the next meter tick per rider index (`u16[2]`, +0x100
/// per tick).
pub(crate) const BOOST_METER_TICK_PITCH: u32 = 0x5fe8;
/// The last `floor(rider->f2F0 * 2)` per rider slot (`int[2]`).
pub(crate) const BOOST_METER_LAST_STEP: u32 = 0x5fec;

/// Rider fields: the human / local flags, the value whose half steps
/// trigger the step sound, the value that must be negative for it, the
/// boost-meter owner chain (`+0x790 -> +0x1b0 -> +0x3a8 {_, max, cur}`), the
/// pitch slot index.
pub(crate) const BOOST_METER_RIDER_HUMAN: u32 = 0x874;
pub(crate) const BOOST_METER_RIDER_LOCAL: u32 = 0x87c;
pub(crate) const BOOST_METER_RIDER_STEP: u32 = 0x2f0;
pub(crate) const BOOST_METER_RIDER_SIGN: u32 = 0x470;
pub(crate) const BOOST_METER_RIDER_OWNER: u32 = 0x790;
pub(crate) const BOOST_METER_OWNER_METER: u32 = 0x1b0;
pub(crate) const BOOST_METER_METER_VALUES: u32 = 0x3a8;
pub(crate) const BOOST_METER_RIDER_PITCH_SLOT: u32 = 0x870;

/// The split-screen mode byte (2: two riders).
pub(crate) const BOOST_METER_PLAYER_MODE: u32 = 0x0053_5c11;
/// The race flags (`gp - 0x6a0`; bit 0 suppresses the step sound).
pub(crate) const BOOST_METER_RACE_FLAGS_GP: u32 = 0x6a0;
/// The game module's race state (`+0x214`; 4 = racing).
pub(crate) const BOOST_METER_MODULE_STATE: u32 = 0x214;

const BOOST_METER_IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;
const BOOST_METER_GET_RACE_MAN: u32 = 0x0028_b1d8;
const BOOST_METER_GET_GAME_MODULE: u32 = 0x0028_b1c8;
const BOOST_METER_REPLAY_BUSY: u32 = 0x0027_0280;
const BOOST_METER_GET_CHANNEL_GAIN: u32 = 0x0028_7968;
const BOOST_METER_ADD_TIMER: Callee = Callee {
    addr: 0x002a_dca0,
    f: ee::s08::BankMonitor_AddTimer,
};
/// The 64-bit timer callback key of the meter tick (`.rodata`).
const BOOST_METER_TIMER_KEY: u32 = 0x004a_36c0;

/// `SSXAudioSystem_UpdateBoostMeterSounds(SSXAudioSystem* self)`
/// (0x29ab40): per frame, for each local human rider (two in split
/// screen), a rising boost meter (`floor(cur / max * 10)` above the last
/// level and positive) queues a meter tick timer on the bank monitor (bank
/// 9, sound 0x96, pitch +0x100 per tick); and when `floor(f2F0 * 2)` just
/// dropped by one half step (1..12, not odd 5+), with `f470` negative,
/// no race flag, the replay not busy and the race running (state 4), pushes
/// and plays sound 0x68 of bank 9. Nothing while a replay plays.
pub fn ssx_audio_system_update_boost_meter_sounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        call_ptr(c, BOOST_METER_IS_REPLAY_PLAYING, 0x0029_ab74)?;
        let replaying = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x90);
        if !replaying {
            boost_meter_riders(c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The inlined `ffloor` of `f0` (`cvt.w.s` / `cvt.s.w`, one less when the
/// value is below the truncation): f1 = the floor; on the adjust path
/// at = f0 = 1.0f.
fn boost_meter_floor_f0(c: &mut Cpu) {
    let x = FpuF32(c.f[0]);
    let t = FpuF32::from_i32(x.to_i32());
    c.f[1] = t.0;
    let below = x.lt(t);
    abi::set_fcc(c, below);
    if below {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = 0x3f80_0000;
        c.f[1] = (t - FpuF32::ONE).0;
    }
}

/// `cvt.w.s f0, f1; mfc1 reg, f0`.
fn boost_meter_floor_to_int(c: &mut Cpu, reg: usize) {
    c.f[0] = FpuF32(c.f[1]).to_i32() as u32;
    c.g[reg][0] = sx(c.f[0]);
}

/// The rider loop: one or two slots (s5 = slot * 4, s7 = slots left,
/// fp = &self->tickPitch).
fn boost_meter_riders(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 13;
    // cBE_getBE()->getInterface(0) (result unused).
    call_ptr(c, 0x0014_dc80, 0x0029_ab84)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0029_ab90)?;
    c.g[V1][0] = 0x53_0000;
    c.g[A0][0] = 2;
    let mode = Addr(BOOST_METER_PLAYER_MODE).lb(c) ^ 2;
    c.g[V0][0] = mode;
    let slots = if mode == 0 { 2 } else { 1 };
    c.g[A1][0] = slots;
    c.g[S7][0] = slots;
    c.cycles += 2;
    c.g[FP][0] = sx(r32(c, S3).wrapping_add(BOOST_METER_TICK_PITCH));
    c.g[S5][0] = 0;
    loop {
        boost_meter_rider(c)?;
        c.cycles += 2;
        let more = c.g[S7][0] != 0;
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0029_abb8);
        }
    }
}

/// `raceMan->riders[slot]` (s5 = slot * 4): v0 = &riders[slot] - 0x28.
fn boost_meter_slot_rider(c: &mut Cpu, ret: u32) -> Result<u64, crate::call::Unwound> {
    call_ptr(c, BOOST_METER_GET_RACE_MAN, ret)?;
    let at = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S5)));
    c.g[V0][0] = at;
    Ok(Addr((at as u32).wrapping_add(0x28)).lw(c))
}

/// One rider slot.
fn boost_meter_rider(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 7;
    let rider = boost_meter_slot_rider(c, 0x0029_abc0)?;
    c.g[V1][0] = rider;
    let human = Addr((rider as u32).wrapping_add(BOOST_METER_RIDER_HUMAN)).lw(c);
    c.g[V0][0] = human;
    c.g[A0][0] = 0;
    if human != 0 {
        c.cycles += 2;
        let local = Addr((rider as u32).wrapping_add(BOOST_METER_RIDER_LOCAL)).lw(c);
        c.g[V0][0] = local;
        c.g[A0][0] = (0u64 < local) as u64;
    }
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.g[S7][0] = sx(r32(c, S7).wrapping_sub(1));
        return Ok(());
    }

    // The boost meter: level = floor(cur / max * 10).
    c.cycles += 20;
    c.g[S2][0] = boost_meter_slot_rider(c, 0x0029_abec)?;
    c.g[AT][0] = 0x4120_0000;
    c.f[2] = 0x4120_0000; // 10.0f
    let rider = r32(c, S2);
    let owner = Addr(rider.wrapping_add(BOOST_METER_RIDER_OWNER)).lw(c);
    c.g[V1][0] = owner;
    let meter = Addr((owner as u32).wrapping_add(BOOST_METER_OWNER_METER)).lw(c);
    c.g[A0][0] = meter;
    let values = if meter == 0 {
        0
    } else {
        sx((meter as u32).wrapping_add(BOOST_METER_METER_VALUES))
    };
    c.g[V0][0] = values;
    let max = Addr((values as u32).wrapping_add(4)).read_fpu(c);
    let cur = Addr((values as u32).wrapping_add(8)).read_fpu(c);
    c.f[0] = (cur.div(max) * FpuF32(0x4120_0000)).0;
    let this = r32(c, S3);
    c.g[V1][0] = sx(this.wrapping_add(BOOST_METER_LAST_LEVEL));
    boost_meter_floor_f0(c);

    c.cycles += 8;
    boost_meter_floor_to_int(c, S1);
    c.g[S4][0] = c.g[V1][0];
    let last_at = sx(r32(c, S4).wrapping_add(r32(c, S5)));
    c.g[V1][0] = last_at;
    let last = Addr(last_at as u32).lw(c);
    let rising = (last as i64) < (c.g[S1][0] as i64);
    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S5)));
    if rising {
        c.cycles += 2;
        c.g[A1][0] = 0x9;
        if (c.g[S1][0] as i64) > 0 {
            boost_meter_tick(c)?;
        }
    }

    // The step sound: s4 = floor(f2F0 * 2).
    c.cycles += 8;
    Addr(c.g[V0][0] as u32).sw(c, c.g[S1][0]);
    let step = Addr(r32(c, S2).wrapping_add(BOOST_METER_RIDER_STEP)).read_fpu(c);
    c.f[0] = (step + step).0;
    boost_meter_floor_f0(c);
    c.cycles += 5;
    boost_meter_floor_to_int(c, S4);
    let w = c.g[S4][0] as i64;
    c.g[S6][0] = sx(r32(c, S3).wrapping_add(BOOST_METER_LAST_STEP));
    'store: {
        'tail: {
            if w <= 0 {
                break 'tail;
            }
            c.cycles += 5;
            let last_at = sx(r32(c, S6).wrapping_add(r32(c, S5)));
            c.g[V1][0] = last_at;
            let prev = sx((Addr(last_at as u32).lw(c) as u32).wrapping_sub(1));
            c.g[V0][0] = sx(r32(c, S6).wrapping_add(r32(c, S5)));
            if c.g[S4][0] != prev {
                break 'store;
            }
            c.cycles += 3;
            let small = w < 13;
            c.g[V0][0] = c.g[S4][0] & 0x1;
            if !small {
                break 'tail;
            }
            c.cycles += 2;
            let odd = c.g[V0][0] != 0;
            c.g[V0][0] = (w < 5) as u64;
            if odd {
                c.cycles += 2;
                let low = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S6).wrapping_add(r32(c, S5)));
                if !low {
                    break 'store;
                }
            }
            boost_meter_step_sound(c)?;
        }
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S6).wrapping_add(r32(c, S5)));
    }
    c.cycles += 2;
    Addr(c.g[V0][0] as u32).sw(c, c.g[S4][0]);
    c.g[S7][0] = sx(r32(c, S7).wrapping_sub(1));
    Ok(())
}

/// The meter tick: `BankMonitor_AddTimer(&(*self->sndRoot)->queue, 0x96,
/// key, GetChannelGainPtr(self, 9, 0), 0, 0x65, 0x7f, pitch[rider->slot])`,
/// then pitch += 0x100. Leaves v0 = &lastLevel[slot].
fn boost_meter_tick(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 27;
    let this = r32(c, S3);
    let root = Addr(this.wrapping_add(0x118)).lw(c);
    c.g[V0][0] = root;
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 0;
    let monitor = Addr(root as u32).lw(c);
    c.g[S0][0] = sx((monitor as u32).wrapping_add(0x1d8));
    call_ptr(c, BOOST_METER_GET_CHANNEL_GAIN, 0x0029_ac80)?;
    let slot = Addr(r32(c, S2).wrapping_add(BOOST_METER_RIDER_PITCH_SLOT)).lw(c);
    c.g[A1][0] = 0x4a_0000;
    c.ldl(A2, BOOST_METER_TIMER_KEY + 7);
    c.ldr(A2, BOOST_METER_TIMER_KEY);
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = c.g[V0][0];
    let pitch_at = sx(r32(c, FP).wrapping_add((slot as u32) << 1));
    c.g[V1][0] = pitch_at;
    c.g[A1][0] = 0x96;
    c.g[T3][0] = Addr(pitch_at as u32).lhu(c);
    c.g[T0][0] = 0;
    c.g[T1][0] = 0x65;
    c.g[T2][0] = 0x7f;
    BOOST_METER_ADD_TIMER.call(c, 0x0029_acb8)?;
    let slot = Addr(r32(c, S2).wrapping_add(BOOST_METER_RIDER_PITCH_SLOT)).lw(c);
    let pitch_at = sx(r32(c, FP).wrapping_add((slot as u32) << 1));
    c.g[V1][0] = pitch_at;
    let pitch = sx((Addr(pitch_at as u32).lhu(c) as u32).wrapping_add(0x100));
    c.g[V0][0] = pitch;
    Addr(pitch_at as u32).sh(c, pitch);
    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S5)));
    Ok(())
}

/// The step-sound conditions (s2 = rider): `f470 < 0`, race flag bit 0
/// clear, the replay not busy (0x270280 false), module state 4; then the
/// push and play of sound 0x68.
fn boost_meter_step_sound(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 7;
    let sign = Addr(r32(c, S2).wrapping_add(BOOST_METER_RIDER_SIGN)).read_fpu(c);
    c.f[1] = sign.0;
    c.g[S0][0] = 0x1;
    c.f[0] = 0;
    let non_negative = FpuF32::ZERO.le(sign);
    abi::set_fcc(c, non_negative);
    c.g[V1][0] = 0x1;
    if !non_negative {
        c.cycles += 1;
        c.g[V1][0] = 0;
    }
    c.cycles += 4;
    let flags = Addr(r32(c, GP).wrapping_sub(BOOST_METER_RACE_FLAGS_GP)).lw(c) & 0x1;
    c.g[V0][0] = flags;
    if c.g[V1][0] != 0 {
        c.g[S0][0] = 0;
    }
    let mut blocked = flags != 0;
    if !blocked {
        c.cycles += 6;
        call_ptr(c, BOOST_METER_GET_GAME_MODULE, 0x0029_ad7c)?;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        call_ptr(c, BOOST_METER_REPLAY_BUSY, 0x0029_ad84)?;
        blocked = c.g[V0][0] != 0;
    }
    if blocked {
        c.cycles += 1;
        c.g[S0][0] = 0;
    }
    c.cycles += 7;
    call_ptr(c, BOOST_METER_GET_GAME_MODULE, 0x0029_ad98)?;
    let state = Addr((c.g[V0][0] as u32).wrapping_add(BOOST_METER_MODULE_STATE)).lw(c) ^ 0x4;
    c.g[V1][0] = state;
    if state != 0 {
        c.g[S0][0] = 0;
    }
    c.g[A0][0] = 0x18;
    if c.g[S0][0] != 0 {
        boost_meter_push_and_play(c)?;
    }
    Ok(())
}

/// `sum = lw(arrays + table) + (cursor << shift)` with the cursor re-read
/// from `monitor + 0x1f4` (s0 = monitor, s1 = arrays = monitor + 0x1d8),
/// the sum in `sum` and the table base in `base`; then `sw value` at
/// `sum + field`.
fn boost_meter_store_entry(
    c: &mut Cpu,
    (sum, base): (usize, usize),
    table: u32,
    shift: u32,
    field: u32,
    value: u64,
) {
    let cursor = Addr(r32(c, S0).wrapping_add(0x1f4)).lw(c);
    c.g[sum][0] = cursor;
    let at = Addr(r32(c, S1).wrapping_add(table)).lw(c);
    c.g[base][0] = at;
    let entry = sx(((cursor as u32) << shift).wrapping_add(at as u32));
    c.g[sum][0] = entry;
    Addr((entry as u32).wrapping_add(field)).sw(c, value);
}

/// The inlined push-and-play of sound 0x68 (a0 = 0x18 on entry): a new
/// entry on the bank monitor's push queue (`self->monitor + 0x1d8`) from
/// the 24-byte template (volume 100, pan 90, 50, 127, 1), bank 0 / sample
/// 0x68 in the system's tables, its gain clamped to 0..0x7f, the gain
/// pointer of bank 9, then `Audio_PlayPushedSound(self)`.
fn boost_meter_push_and_play(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 157;
    // cursor++; records[cursor] = template.
    let monitor = Addr(r32(c, S3)).lw(c);
    c.g[S0][0] = monitor;
    let monitor = monitor as u32;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x8;
    let count = Addr(monitor.wrapping_add(0x1f4)).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(0x1d8));
    let arrays = r32(c, S1);
    let count = sx((count as u32).wrapping_add(1));
    c.g[V0][0] = count;
    Addr(monitor.wrapping_add(0x1f4)).sw(c, count);
    c.g[V1][0] = Addr(monitor.wrapping_add(0x1f4)).lw(c);
    c.g[V0][0] = Addr(arrays.wrapping_add(0x20)).lw(c);
    c.mult(0, V1, V1, A0);
    let record = (c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32);
    c.g[V1][0] = sx(record);
    c.ldl(A3, monitor.wrapping_add(0x1e3));
    c.ldr(A3, monitor.wrapping_add(0x1dc));
    c.ldl(T0, monitor.wrapping_add(0x1eb));
    c.ldr(T0, monitor.wrapping_add(0x1e4));
    c.ldl(V0, monitor.wrapping_add(0x1f3));
    c.ldr(V0, monitor.wrapping_add(0x1ec));
    c.sdl(A3, record.wrapping_add(0x7));
    c.sdr(A3, record);
    c.sdl(T0, record.wrapping_add(0xf));
    c.sdr(T0, record.wrapping_add(0x8));
    c.sdl(V0, record.wrapping_add(0x17));
    c.sdr(V0, record.wrapping_add(0x10));
    // monitor->pushHandle[cursor] = 0 (inline at +0x1fc).
    let cursor = Addr(monitor.wrapping_add(0x1f4)).lw(c);
    let at = sx(monitor.wrapping_add((cursor as u32) << 2));
    c.g[V0][0] = at;
    Addr((at as u32).wrapping_add(0x1fc)).sw(c, 0);
    boost_meter_store_entry(c, (V1, V0), 0x34, 2, 0, 0);
    // table 0x38 [cursor] = (s8) records[cursor].vol.
    c.g[V0][0] = 0x18;
    c.g[A3][0] = Addr(monitor.wrapping_add(0x1f4)).lw(c);
    c.g[A0][0] = Addr(monitor.wrapping_add(0x1f4)).lw(c);
    c.g[T0][0] = Addr(arrays.wrapping_add(0x20)).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, V0);
    c.g[V0][0] = Addr(arrays.wrapping_add(0x38)).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    boost_meter_store_entry(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&arrays->params[cursor] (+0x40), 0, 8).
    let cursor = Addr(monitor.wrapping_add(0x1f4)).lw(c);
    c.g[A0][0] = sx(arrays.wrapping_add((cursor as u32) << 3).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0029_ae80)?;

    // The entry's parameters.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x1;
    boost_meter_store_entry(c, (V1, V0), 0x60, 2, 0, 0);
    c.g[T0][0] = 0x18;
    boost_meter_store_entry(c, (V0, V1), 0x64, 2, 0, 0);
    boost_meter_store_entry(c, (V1, V0), 0x68, 2, 0, 0);
    boost_meter_store_entry(c, (V0, V1), 0x6c, 2, 0, 0);
    boost_meter_store_entry(c, (V1, V0), 0x70, 4, 0x4, 0);
    let (a0, a1, a2, a3) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    boost_meter_store_entry(c, (V0, V1), 0x70, 4, 0x0, a0);
    boost_meter_store_entry(c, (V1, V0), 0x70, 4, 0x8, a1);
    boost_meter_store_entry(c, (V0, V1), 0x70, 4, 0xc, a2);
    boost_meter_store_entry(c, (V1, V0), 0x74, 2, 0, 0);
    boost_meter_store_entry(c, (V0, V1), 0x80, 2, 0, 0);
    boost_meter_store_entry(c, (V1, V0), 0x84, 2, 0, 0);
    boost_meter_store_entry(c, (V0, V1), 0x78, 2, 0, 0x7f);
    c.g[V1][0] = 0x7f;
    boost_meter_store_entry(c, (V1, V0), 0x7c, 2, 0, a3);
    c.g[A3][0] = 0x7f;

    // self->pushBank[cursor] = 0; self->pushSample[cursor] = 0x68.
    let this = r32(c, S3);
    c.g[A0][0] = Addr(this).lw(c);
    c.g[V1][0] = Addr(this.wrapping_add(4)).lw(c);
    let cursor = Addr((c.g[A0][0] as u32).wrapping_add(0x1f4)).lw(c);
    let at = sx(((cursor as u32) << 2).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = at;
    Addr(at as u32).sw(c, 0);
    c.g[V1][0] = Addr(this).lw(c);
    c.g[A0][0] = Addr(this.wrapping_add(8)).lw(c);
    let cursor = Addr((c.g[V1][0] as u32).wrapping_add(0x1f4)).lw(c);
    let at = sx(((cursor as u32) << 2).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = at;
    c.g[A0][0] = 0x68;
    Addr(at as u32).sw(c, 0x68);

    // (*self->sndRoot) queue: vals[cur] = 0x7f; ch[cur].vol = clamp(vals[cur]).
    let root = Addr(this.wrapping_add(0x118)).lw(c);
    c.g[V1][0] = root;
    let owner = Addr(root as u32).lw(c);
    c.g[A1][0] = owner;
    let owner = owner as u32;
    let cursor = Addr(owner.wrapping_add(0x1f4)).lw(c);
    let queue = owner.wrapping_add(0x1d8);
    c.g[A2][0] = sx(queue);
    let vals = Addr(queue.wrapping_add(0x38)).lw(c);
    c.g[V1][0] = vals;
    let at = sx(((cursor as u32) << 2).wrapping_add(vals as u32));
    c.g[V0][0] = at;
    Addr(at as u32).sw(c, c.g[A3][0]);
    c.g[A0][0] = Addr(owner.wrapping_add(0x1f4)).lw(c);
    c.g[V1][0] = Addr(queue.wrapping_add(0x1c)).lw(c);
    c.g[A3][0] = Addr(queue.wrapping_add(0x38)).lw(c);
    c.mult(0, A0, A0, T0);
    let vals = r32(c, A3);
    let val_at = |c: &mut Cpu| {
        let cur = Addr(queue.wrapping_add(0x1c)).lw(c);
        sx(((cur as u32) << 2).wrapping_add(vals))
    };
    c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(vals));
    c.g[A1][0] = Addr(queue.wrapping_add(0x20)).lw(c);
    let v = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = ((v as i64) < 127) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    'clamp: {
        if (v as i64) < 127 {
            c.cycles += 6;
            let at = val_at(c);
            c.g[V0][0] = at;
            let v = Addr(at as u32).lw(c);
            c.g[V1][0] = v;
            c.g[V0][0] = 0;
            if (v as i64) < 0 {
                break 'clamp;
            }
        }
        c.cycles += 7;
        let at = val_at(c);
        c.g[V0][0] = at;
        let v = Addr(at as u32).lw(c);
        let below = (v as i64) < 127;
        c.g[V1][0] = below as u64;
        if !below {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            let at = val_at(c);
            c.g[V0][0] = Addr(at as u32).lbu(c);
        }
    }
    c.cycles += 14;
    Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);

    // monitor->pushGain[cursor] = GetChannelGainPtr(self, 9, 0); play.
    c.g[A1][0] = 0x9;
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 0;
    c.g[S0][0] = Addr(this.wrapping_add(0x118)).lw(c);
    call_ptr(c, BOOST_METER_GET_CHANNEL_GAIN, 0x0029_b07c)?;
    let owner = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = owner;
    c.g[A0][0] = c.g[S3][0];
    let cursor = Addr((owner as u32).wrapping_add(0x1f4)).lw(c);
    let gains = Addr((owner as u32).wrapping_add(0x214)).lw(c);
    c.g[A2][0] = gains;
    let at = sx(((cursor as u32) << 2).wrapping_add(gains as u32));
    c.g[V1][0] = at;
    Addr(at as u32).sw(c, c.g[V0][0]);
    call_ptr(c, 0x0029_06b8, 0x0029_b09c)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `SSXAudioSystem_UpdateBoostMeterSounds` on the soundsys audio world
    /// (real IsReplayPlaying, cBE_getBE, GetRaceMan / GetGameModule, the
    /// replay check 0x270280 through its jump table, GetChannelGainPtr,
    /// BankMonitor_AddTimer on free timers, memset and
    /// `Audio_PlayPushedSound` with no bank; GetInterface's method is a
    /// fake). Replay states playing (early out) or not; one or two rider
    /// slots; riders human/local or not; a missing meter; meter levels
    /// rising or not, positive or not; step values 0..14 (and negative)
    /// equal to the last step - 1 or not; f470 negative or not, the race
    /// flag, module state 4 or not.
    #[test]
    fn ssx_audio_system_update_boost_meter_sounds_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{check_lohi, fill, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::audio;
        const RACE_MAN: u32 = DATA + 0x27_0000;
        const RIDER1: u32 = DATA + 0x27_1000;
        const OWNERS: u32 = DATA + 0x27_4000;
        const METERS: u32 = DATA + 0x27_6000;
        check_lohi(
            ee::s08::f_0029ab40__generated,
            ssx_audio_system_update_boost_meter_sounds,
            240,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_ab40, 0x0029_ab40);
                let sys = audio::SYS;
                // BELibrary_GetInterface(0): interface 0, a fake method.
                let iface = DATA + 0x26_1000;
                let ivt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, ivt);
                c.bus.write16(ivt + 8, 0);
                c.bus.write32(ivt + 0xc, FAKE + 0x40);
                // Always in game; the replay: playing (states 1..9) on some
                // seeds, else idle states the busy check reads.
                c.bus.write32(audio::GAME + 0x84, audio::MODULE);
                c.bus.write32(audio::MODULE + 0x28, audio::REPLAY);
                let replay = if seed % 11 == 3 {
                    5
                } else {
                    [0u32, 0, 12, 13, 0x20][(r.next_u32() % 5) as usize]
                };
                c.bus.write32(audio::REPLAY, replay);
                c.bus.write32(audio::REPLAY + 0x3cc, r.next_u32() % 4);
                c.bus.write32(audio::REPLAY + 0x484, r.next_u32() % 4);
                c.bus.write32(audio::REPLAY + 0x604, r.next_u32() % 2);
                c.bus.write32(
                    audio::MODULE + BOOST_METER_MODULE_STATE,
                    if seed % 5 == 4 { 3 } else { 4 },
                );
                c.bus.write32(
                    GAME_GP - BOOST_METER_RACE_FLAGS_GP,
                    if seed % 9 == 5 { 3 } else { r.next_u32() & !1 },
                );
                c.bus
                    .write8(BOOST_METER_PLAYER_MODE, [2u8, 2, 1, 0][(seed % 4) as usize]);
                // The race manager's two rider slots.
                c.bus.write32(audio::MODULE + 0xc, RACE_MAN);
                c.bus.write32(RACE_MAN + 0x28, audio::RIDER);
                c.bus.write32(RACE_MAN + 0x2c, RIDER1);
                for (i, rider) in [audio::RIDER, RIDER1].into_iter().enumerate() {
                    let i = i as u32;
                    let on = (seed + i) % 8 != 1;
                    c.bus.write32(
                        rider + BOOST_METER_RIDER_HUMAN,
                        if on { 1 + r.next_u32() % 2 } else { 0 },
                    );
                    c.bus.write32(
                        rider + BOOST_METER_RIDER_LOCAL,
                        if (seed + i) % 8 == 6 {
                            0
                        } else {
                            r.next_u32() | 1
                        },
                    );
                    let owner = OWNERS + 0x400 * i;
                    let meter = METERS + 0x1000 * i;
                    c.bus.write32(rider + BOOST_METER_RIDER_OWNER, owner);
                    c.bus.write32(
                        owner + BOOST_METER_OWNER_METER,
                        if (seed + i) % 23 == 7 { 0 } else { meter },
                    );
                    let values = meter + BOOST_METER_METER_VALUES;
                    let max = 0.5 + (r.next_u32() % 100) as f32 / 40.0;
                    let frac = match r.next_u32() % 6 {
                        0 => -0.17,
                        1 => 1.0,
                        _ => (r.next_u32() % 1000) as f32 / 1000.0,
                    };
                    c.bus.write32(values + 4, max.to_bits());
                    c.bus.write32(values + 8, (max * frac).to_bits());
                    c.bus
                        .write32(rider + BOOST_METER_RIDER_PITCH_SLOT, r.next_u32() % 2);
                    // Step value: half steps 0..14 (fractional, or negative).
                    let w = (r.next_u32() % 15) as i32;
                    let step = match r.next_u32() % 8 {
                        0 => -0.3,
                        1 => w as f32 / 2.0,
                        _ => (w as f32 + 0.25) / 2.0,
                    };
                    c.bus
                        .write32(rider + BOOST_METER_RIDER_STEP, step.to_bits());
                    let sign = if r.next_u32() % 4 == 0 { 0.5f32 } else { -1.5 };
                    c.bus
                        .write32(rider + BOOST_METER_RIDER_SIGN, sign.to_bits());
                    // Last level / step of this slot.
                    let level = (r.next_u32() % 12) as i32 - 1;
                    c.bus
                        .write32(sys + BOOST_METER_LAST_LEVEL + 4 * i, level as u32);
                    let last_step = if r.next_u32() % 3 == 0 {
                        r.next_u32() % 16
                    } else {
                        (w + 1) as u32
                    };
                    c.bus
                        .write32(sys + BOOST_METER_LAST_STEP + 4 * i, last_step);
                }
                fill(c, &mut r, sys + BOOST_METER_TICK_PITCH, 1);
            },
        );
    }
}
