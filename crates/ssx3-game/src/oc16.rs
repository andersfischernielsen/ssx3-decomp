//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements (wave 12): script movie / command queue, SSX audio
//! system, sound bank, crowd audio, INI parser, PS2 sound manager, stream array
//! and bank monitor. Each copies the generated body's logic, cycle charging and
//! observable register / FPU / LO-HI results exactly, gated by a differential
//! test.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_3798, script_movie_ctor, "ScriptMovie_ctor"),
    (0x0028_3c20, script_cmd_queue_clear, "ScriptCmdQueue_Clear"),
    (0x0028_3c60, script_cmd_queue_next_index, "ScriptCmdQueue_NextIndex"),
    (0x0028_3d70, script_cmd_queue_at, "ScriptCmdQueue_At"),
    (0x0028_3da0, script_cmd_queue_is_empty, "ScriptCmdQueue_IsEmpty"),
    (0x0028_73d8, ssx_audio_system_level_to_volume, "SSXAudioSystem_LevelToVolume"),
    (0x0028_7920, ssx_audio_system_get_channel_volume, "SSXAudioSystem_GetChannelVolume"),
    (0x0028_7930, ssx_audio_system_get_channel_gain, "SSXAudioSystem_GetChannelGain"),
    (0x0028_b248, snd_bank_ctor, "SndBank_ctor"),
    (0x0028_b6c8, snd_bank_link, "SndBank_Link"),
    (0x002a_72c0, crowd_audio_decode_source_handle, "CrowdAudio_DecodeSourceHandle"),
    (0x002a_78e0, ini_parser_get_char, "IniParser_GetChar"),
    (0x002a_8c20, ps2_sound_manager_ctor, "PS2SoundManager_ctor"),
    (0x002a_92b8, ps2_sound_manager_pop_free_spu_block, "PS2SoundManager_PopFreeSpuBlock"),
    (0x002a_b128, stream_array_get_stream_handle, "StreamArray_GetStreamHandle"),
    (0x002a_b9e0, bank_monitor_instance_ctor, "BankMonitorInstance_ctor"),
    (0x002a_bca0, bank_monitor_calc_doppler_pitch, "BankMonitor_CalcDopplerPitch"),
    (0x002a_d848, bank_monitor_timer_init, "BankMonitor_TimerInit"),
    (0x002a_d930, bank_monitor_timer_is_free, "BankMonitor_TimerIsFree"),
];

// ---------------------------------------------------------------------------
// Script movie.

/// `ScriptMovie_ctor(ScriptMovie *self)` (0x283798): clear the movie handle and
/// state, and install the vtable `0x482228` at +0x18. Returns `self` in v0.
pub fn script_movie_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x0048_2228; // 0x480000 + 0x2228
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = sx(VTABLE);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(0x18).sw(c, sx(VTABLE));
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script command queue (a 4-entry ring of 28-byte entries).

const CMD_READ: u32 = 0x8c;
const CMD_WRITE: u32 = 0x90;
const CMD_FULL: u32 = 0x94;
const CMD_ENTRY_SIZE: u32 = 0x1c;
const CMD_RING_SIZE: u64 = 4;

/// `void ScriptCmdQueue_Clear(ScriptCmdQueue *q)` (0x283c20): reset the read
/// and write cursors and clear the full flag.
pub fn script_cmd_queue_clear(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    q.off(CMD_FULL).sw(c, 0);
    q.off(CMD_READ).sw(c, 0);
    q.off(CMD_WRITE).sw(c, 0);
    abi::ret(c);
}

/// `int ScriptCmdQueue_NextIndex(int index)` (0x283c60): `index + 1`, wrapping
/// back to 0 after the last slot (index 4).
pub fn script_cmd_queue_next_index(c: &mut Cpu) {
    c.cycles += 3;
    let index = c.g[A1][0];
    let next = sx((index as u32).wrapping_add(1));
    c.cycles += 2;
    c.g[V0][0] = if index == CMD_RING_SIZE { 0 } else { next };
    abi::ret(c);
}

/// `ScriptCmdQueueEntry *ScriptCmdQueue_At(ScriptCmdQueue *q, int index)`
/// (0x283d70): address of the ring element at `read + index` (mod 4), the
/// entries being 28 bytes each.
pub fn script_cmd_queue_at(c: &mut Cpu) {
    c.cycles += 5;
    let q = Addr(abi::arg(c, 0));
    let read = q.off(CMD_READ).lw(c) as u32;
    let at = read.wrapping_add(abi::arg(c, 1));
    let rem = abi::div(c, at, 5).1;
    let slot = abi::mult(c, rem, CMD_ENTRY_SIZE);
    c.cycles += 6;
    c.g[A2][0] = 5;
    c.g[A1][0] = sx(rem);
    c.g[V1][0] = sx(slot);
    c.g[V0][0] = sx(slot.wrapping_add(abi::arg(c, 0)));
    abi::ret(c);
}

/// `bool ScriptCmdQueue_IsEmpty(ScriptCmdQueue *q)` (0x283da0): true when the
/// read cursor still equals the write cursor and the queue is not marked full.
pub fn script_cmd_queue_is_empty(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    let write = q.off(CMD_WRITE).lw(c);
    let read = q.off(CMD_READ).lw(c);
    c.g[V1][0] = write;
    let mut empty = 0u64;
    if write == read {
        c.cycles += 2;
        empty = (q.off(CMD_FULL).lw(c) < 1) as u64;
    }
    c.cycles += 2;
    c.g[A1][0] = empty;
    c.g[V0][0] = empty;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SSX audio system.

/// `float SSXAudioSystem_LevelToVolume(SSXAudioSystem *self, int level)`
/// (0x2873d8): scales the integer level by the constant at `gp-0x462c`, then
/// clamps the result to `[0, 1]`.
pub fn ssx_audio_system_level_to_volume(c: &mut Cpu) {
    c.cycles += 11;
    let level = FpuF32::from_i32(c.g[A1][0] as u32 as i32);
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x462c)).read_fpu(c);
    let scaled = level * scale;
    c.f[2] = scaled.0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[3] = FpuF32::ONE.0;
    let zero = FpuF32::ZERO;
    c.f[1] = zero.0;
    abi::set_fcc(c, zero.le(scaled));
    if zero.le(scaled) {
        c.cycles += 1;
        c.f[1] = scaled.min(FpuF32::ONE).0;
    }
    c.cycles += 2;
    c.f[0] = c.f[1];
    abi::ret(c);
}

/// `float SSXAudioSystem_GetChannelVolume(SSXAudioSystem *self, int channel)`
/// (0x287920): the volume field at `self + channel*16 + 0x62bc`.
pub fn ssx_audio_system_get_channel_volume(c: &mut Cpu) {
    const VOLUME: u32 = 0x62bc;
    c.cycles += 4;
    let off = sx(abi::arg(c, 1) << 4);
    let addr = abi::arg(c, 0).wrapping_add(off as u32);
    c.g[A1][0] = off;
    c.g[A0][0] = sx(addr);
    c.f[0] = Addr(addr).off(VOLUME).read_fpu(c).0;
    abi::ret(c);
}

/// `float SSXAudioSystem_GetChannelGain(SSXAudioSystem *self, int channel,
/// int sub)` (0x287930): channel 4 returns a per-sub gain from +0x636c
/// (`sub*4`), everything else a per-channel gain from +0x62c8 (`channel*16`).
pub fn ssx_audio_system_get_channel_gain(c: &mut Cpu) {
    const GAIN: u32 = 0x62c8;
    const SUB_GAIN: u32 = 0x636c;
    c.cycles += 3;
    let base = abi::arg(c, 0);
    if c.g[A1][0] != 4 {
        c.g[V0][0] = sx(abi::arg(c, 1) << 4);
        c.cycles += 3;
        let addr = base.wrapping_add(c.g[V0][0] as u32);
        c.g[V0][0] = sx(addr);
        c.f[0] = Addr(addr).off(GAIN).read_fpu(c).0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    if (c.g[A2][0] as i64) < 10 {
        c.cycles += 3;
        let off = sx(abi::arg(c, 2) << 2);
        let addr = base.wrapping_add(off as u32);
        c.g[V0][0] = sx(addr);
        c.f[0] = Addr(addr).off(SUB_GAIN).read_fpu(c).0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = sx(abi::arg(c, 1) << 4);
    c.cycles += 3;
    let addr = base.wrapping_add(c.g[V0][0] as u32);
    c.g[V0][0] = sx(addr);
    c.f[0] = Addr(addr).off(GAIN).read_fpu(c).0;
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

/// `void SndBank_Link(SndBank *self, SndBank *other)` (0x28b6c8): splice
/// `self` into the doubly-linked list right after `other`, then adopt
/// `other`'s data pointers.
pub fn snd_bank_link(c: &mut Cpu) {
    let self_ = Addr(abi::arg(c, 0));
    let other = Addr(abi::arg(c, 1));
    c.cycles += 3;
    let other_next = other.off(0x18).lw(c);
    if other_next == 0 {
        other.off(0x18).sw(c, c.g[A0][0]);
        c.cycles += 3;
        other.off(0x1c).sw(c, c.g[A0][0]);
        self_.off(0x18).sw(c, c.g[A1][0]);
        self_.off(0x1c).sw(c, c.g[A1][0]);
    } else {
        c.cycles += 7;
        self_.off(0x18).sw(c, c.g[A1][0]);
        let next = other.off(0x1c).lw(c);
        self_.off(0x1c).sw(c, next);
        other.off(0x1c).sw(c, c.g[A0][0]);
        let back = self_.off(0x1c).lw(c);
        Addr(back as u32).off(0x18).sw(c, c.g[A0][0]);
    }
    c.cycles += 3;
    let data = other.off(0x8).lw(c);
    c.g[V0][0] = data;
    if data != 0 {
        c.cycles += 7;
        self_.off(0x8).sw(c, data);
        let p10 = other.off(0x10).lw(c);
        self_.off(0x10).sw(c, p10);
        let p14 = other.off(0x14).lw(c);
        c.g[V1][0] = p14;
        self_.off(0x14).sw(c, p14);
        let p0c = other.off(0xc).lw(c);
        self_.off(0xc).sw(c, p0c);
        c.g[V0][0] = p0c;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SndBank_Unlink(SndBank *self)` (0x28b730): remove `self` from the
/// doubly-linked list, keeping neighbours connected, and clear its links.
pub fn snd_bank_unlink(c: &mut Cpu) {
    let self_ = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let prev = self_.off(0x18).lw(c);
    c.g[V1][0] = prev;
    c.g[V0][0] = u64::MAX;
    if prev != 0 {
        c.cycles += 6;
        let next = self_.off(0x1c).lw(c);
        c.g[A1][0] = next;
        self_.off(0xc).sw(c, u64::MAX);
        self_.off(0x10).sw(c, 0);
        self_.off(0x14).sw(c, 0);
        self_.off(0x8).sw(c, 0);
        if prev != next {
            c.cycles += 4;
            Addr(prev as u32).off(0x18).sw(c, 0);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V0][0] as u32).off(0x1c).sw(c, 0);
            c.cycles += 4;
            Addr(prev as u32).off(0x1c).sw(c, next);
            c.g[V1][0] = self_.off(0x1c).lw(c);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            Addr(prev as u32).off(0x18).sw(c, 0);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V0][0] as u32).off(0x1c).sw(c, 0);
            c.g[V1][0] = prev;
        }
        c.cycles += 2;
        self_.off(0x18).sw(c, 0);
        self_.off(0x1c).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Crowd audio.

/// `void CrowdAudio_DecodeSourceHandle(uint handle, int *index, int *source)`
/// (0x2a72c0): split the packed handle into its low byte and its arithmetic
/// `>> 8` (sign-extended) high part.
pub fn crowd_audio_decode_source_handle(c: &mut Cpu) {
    c.cycles += 5;
    let handle = c.g[A1][0];
    let index = handle & 0xff;
    let source = sx(((handle as u32) as i32 >> 8) as u32);
    c.g[V0][0] = index;
    c.g[A1][0] = source;
    Addr(abi::arg(c, 2)).sw(c, index);
    Addr(abi::arg(c, 3)).sw(c, source);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// INI parser.

/// `int IniParser_GetChar(IniParser *self)` (0x2a78e0): read the sign-extended
/// byte at the cursor, then advance the cursor by one.
pub fn ini_parser_get_char(c: &mut Cpu) {
    const CURSOR: u32 = 0x8;
    c.cycles += 5;
    let parser = Addr(abi::arg(c, 0));
    let ptr = parser.off(CURSOR).lw(c);
    c.g[V1][0] = ptr;
    c.g[V0][0] = Addr(ptr as u32).lb(c);
    c.g[V1][0] = sx((ptr as u32).wrapping_add(1));
    parser.off(CURSOR).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PS2 sound manager and its SPU-block free list.

/// `PS2SoundManager_ctor(PS2SoundManager *self)` (0x2a8c20): install the
/// vtable `0x4835f8`, `-1` sentinels, and zero the state words.
pub fn ps2_sound_manager_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x0048_35f8; // 0x480000 + 0x35f8
    c.cycles += 12;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(VTABLE);
    c.g[V1][0] = u64::MAX;
    this.off(0x318).sw(c, u64::MAX);
    this.sw(c, sx(VTABLE));
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x30c).sw(c, u64::MAX);
    this.off(0x310).sw(c, u64::MAX);
    this.off(0x31c).sw(c, 0);
    abi::ret(c);
}

/// `void *PS2SoundManager_PopFreeSpuBlock(PS2SoundManager *self)` (0x2a92b8):
/// pop the head block off the free list at +0x310 (blocks 24 bytes each) and
/// return its address, or NULL when the list sentinel `-1` is reached.
pub fn ps2_sound_manager_pop_free_spu_block(c: &mut Cpu) {
    const FREE_HEAD: u32 = 0x310;
    const BLOCK_SIZE: u32 = 0x18;
    c.cycles += 4;
    let sm = Addr(abi::arg(c, 0));
    let head = sm.off(FREE_HEAD).lw(c);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = 0x18;
    if (head as i64) < 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 9;
    let lo = abi::mult(c, head as u32, BLOCK_SIZE);
    let block = sx(sm.0.wrapping_add(lo.wrapping_add(0xc)));
    c.g[V1][0] = block;
    c.g[V0][0] = block;
    let next = Addr(block as u32).off(0x14).lw(c);
    c.g[A0][0] = next;
    sm.off(FREE_HEAD).sw(c, next);
    Addr(block as u32).off(0x14).sw(c, u64::MAX);
    c.g[A1][0] = u64::MAX;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Stream array.

/// `int StreamArray_GetStreamHandle(StreamArray *self, int index)`
/// (0x2ab128): the +0x8 field of the stream at `self->streams[index]`, or -1
/// when the slot is empty.
pub fn stream_array_get_stream_handle(c: &mut Cpu) {
    const STREAMS: u32 = 0x8;
    c.cycles += 6;
    let arr = Addr(abi::arg(c, 0));
    let base = arr.off(STREAMS).lw(c);
    c.g[V0][0] = base;
    let slot = sx((abi::arg(c, 1) << 2).wrapping_add(base as u32));
    let entry = Addr(slot as u32).lw(c);
    c.g[A1][0] = entry;
    c.g[V0][0] = u64::MAX;
    if entry != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(entry as u32).off(STREAMS).lw(c);
    } else {
        c.cycles += 2;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bank monitor.

/// `BankMonitorInstance_ctor(BankMonitorInstance *self)` (0x2ab9e0): set the
/// default distance/gain sentinels and store `-1.0` at +0x90.
pub fn bank_monitor_instance_ctor(c: &mut Cpu) {
    c.cycles += 14;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x8000;
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // mtc1 f0 = -1.0
    c.f[0] = c.g[AT][0] as u32;
    c.g[V1][0] = u64::MAX;
    c.g[A1][0] = 0xffff_ffff_ffff_8ad0;
    this.off(0xb0).sw(c, 0x8000);
    this.off(0x90).write_fpu(c, FpuF32(c.f[0]));
    c.g[V0][0] = c.g[A0][0];
    this.off(0xb4).sw(c, u64::MAX);
    this.off(0xb8).sw(c, 0xffff_ffff_ffff_8ad0);
    this.sw(c, 0);
    this.off(0x54).sw(c, 0);
    this.off(0x58).sw(c, 0);
    abi::ret(c);
}

/// `float BankMonitor_CalcDopplerPitch(float ratio, float unused)` (0x2abca0):
/// clamp `f12 / f13` to `[gp-0x4560, gp-0x455c]`, then return
/// `(clamped + 8000.0) * gp-0x4558`.
pub fn bank_monitor_calc_doppler_pitch(c: &mut Cpu) {
    c.cycles += 6;
    let ratio = abi::farg(c, 0).div(abi::farg(c, 1));
    let lower = Addr((c.g[GP][0] as u32).wrapping_sub(0x4560)).read_fpu(c);
    let clamped;
    if abi::c_lt_s(c, ratio, lower) {
        clamped = lower;
    } else {
        c.cycles += 5;
        let upper = Addr((c.g[GP][0] as u32).wrapping_sub(0x455c)).read_fpu(c);
        if abi::c_lt_s(c, upper, ratio) {
            clamped = upper;
        } else {
            clamped = ratio;
        }
    }
    c.cycles += 6;
    c.g[AT][0] = 0x45fa_0000;
    let addend = FpuF32(c.g[AT][0] as u32); // mtc1 f0 = 8000.0
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x4558)).read_fpu(c);
    c.f[12] = clamped.0;
    c.f[1] = scale.0;
    c.f[0] = ((clamped + addend) * scale).0;
    abi::ret(c);
}

/// `void BankMonitor_TimerInit(BankMonitor_Timer *self)` (0x2ad848): clear the
/// active flag and set the period sentinel `-1`. Returns `self` in v0.
pub fn bank_monitor_timer_init(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    this.sw(c, 0);
    this.off(0x4).sw(c, u64::MAX);
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `bool BankMonitor_TimerIsFree(BankMonitor_Timer *self)` (0x2ad930): true
/// when the period field at +0x4 is the `-1` sentinel.
pub fn bank_monitor_timer_is_free(c: &mut Cpu) {
    c.cycles += 4;
    let v = !(0u64 | Addr(abi::arg(c, 0)).off(0x4).lw(c));
    c.g[V0][0] = (v < 1) as u64;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn script_movie_ctor_matches() {
        check(ssx3_ee::s07::f_00283798, script_movie_ctor, 50, &|c, seed| {
            let mut r = Rng(0x1111_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..9 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn script_cmd_queue_clear_matches() {
        check(ssx3_ee::s07::f_00283c20, script_cmd_queue_clear, 50, &|c, seed| {
            let mut r = Rng(0x1111_0002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..40 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn script_cmd_queue_next_index_matches() {
        check(ssx3_ee::s07::f_00283c60, script_cmd_queue_next_index, 100, &|c, seed| {
            let mut r = Rng(0x1111_0003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = if seed % 2 == 0 {
                4
            } else {
                ((r.next_u32() as u64) << 32) | r.next_u32() as u64
            };
        });
    }

    #[test]
    fn script_cmd_queue_at_matches() {
        check_lohi(ssx3_ee::s07::f_00283d70, script_cmd_queue_at, 200, &|c, seed| {
            let mut r = Rng(0x1111_0004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.bus.write32(DATA + CMD_READ, r.next_u32());
        });
    }

    #[test]
    fn script_cmd_queue_is_empty_matches() {
        check(ssx3_ee::s07::f_00283da0, script_cmd_queue_is_empty, 200, &|c, seed| {
            let mut r = Rng(0x1111_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let read = r.next_u32();
            let write = if seed % 3 == 0 { read } else { r.next_u32() };
            c.bus.write32(DATA + CMD_READ, read);
            c.bus.write32(DATA + CMD_WRITE, write);
            c.bus.write32(DATA + CMD_FULL, if seed % 4 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn ssx_audio_system_level_to_volume_matches() {
        check(ssx3_ee::s07::f_002873d8, ssx_audio_system_level_to_volume, 300, &|c, seed| {
            let mut r = Rng(0x1111_0006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let scale = match seed % 3 {
                0 => 1.0f32.to_bits(),
                1 => 2.0f32.to_bits(),
                _ => r.edge_f32(),
            };
            c.bus.write32(GAME_GP - 0x462c, scale);
            c.g[A1][0] = match seed % 4 {
                0 => (-5i32) as u32 as u64,
                1 => 0,
                2 => 5,
                _ => r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn ssx_audio_system_get_channel_volume_matches() {
        check(ssx3_ee::s07::f_00287920, ssx_audio_system_get_channel_volume, 100, &|c, seed| {
            let mut r = Rng(0x1111_0007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let idx = (seed % 8) as u64;
            c.g[A1][0] = idx;
            c.bus.write32(DATA + (idx as u32) * 0x10 + 0x62bc, r.edge_f32());
        });
    }

    #[test]
    fn ssx_audio_system_get_channel_gain_matches() {
        check(ssx3_ee::s07::f_00287930, ssx_audio_system_get_channel_gain, 300, &|c, seed| {
            let mut r = Rng(0x1111_0008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let a1 = (seed % 5) as u64; // 4 exercises the sub-channel path
            let a2 = match seed % 4 {
                0 => 3,
                1 => 12,
                2 => 0xffff_ffff_0000_0007, // negative i64 (< 10), low word stays small
                _ => (seed % 6) as u64,
            };
            c.g[A1][0] = a1;
            c.g[A2][0] = a2;
            c.bus.write32(DATA + (a1 as u32) * 0x10 + 0x62c8, r.edge_f32());
            c.bus.write32(DATA + ((a2 as u32) << 2) + 0x636c, r.edge_f32());
        });
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
    fn snd_bank_link_matches() {
        check(ssx3_ee::s07::f_0028b6c8, snd_bank_link, 300, &|c, seed| {
            let mut r = Rng(0x1111_000a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let other = DATA + 0x100;
            let prev = if seed % 2 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(other + 0x18, prev);
            c.bus.write32(other + 0x1c, DATA + 0x300);
            c.bus.write32(other + 0x8, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(other + 0x10, r.next_u32());
            c.bus.write32(other + 0x14, r.next_u32());
            c.bus.write32(other + 0xc, r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
            c.bus.write32(DATA + 0x200 + 0x18, r.next_u32());
            c.bus.write32(DATA + 0x300 + 0x18, r.next_u32());
            c.bus.write32(DATA + 0x300 + 0x1c, r.next_u32());
        });
    }

    #[test]
    fn snd_bank_unlink_matches() {
        check(ssx3_ee::s07::f_0028b730, snd_bank_unlink, 300, &|c, seed| {
            let mut r = Rng(0x1111_000b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            // 0: no previous; 1: prev == next; 2: prev != next.
            let (prev, next, write_next) = match seed % 3 {
                0 => (0, DATA + 0x200, false),
                1 => (DATA + 0x100, DATA + 0x100, false),
                _ => (DATA + 0x100, DATA + 0x200, true),
            };
            c.bus.write32(DATA + 0x18, prev);
            c.bus.write32(DATA + 0x1c, next);
            for off in [0x8u32, 0xc, 0x10, 0x14] {
                c.bus.write32(DATA + off, r.next_u32());
            }
            if prev != 0 {
                c.bus.write32(prev + 0x18, r.next_u32());
                c.bus.write32(prev + 0x1c, r.next_u32());
            }
            if write_next {
                c.bus.write32(DATA + 0x200 + 0x18, r.next_u32());
            }
        });
    }

    #[test]
    fn crowd_audio_decode_source_handle_matches() {
        check(ssx3_ee::s08::f_002a72c0, crowd_audio_decode_source_handle, 200, &|c, seed| {
            let mut r = Rng(0x1111_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = DATA as u64;
            c.g[A3][0] = (DATA + 4) as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn ini_parser_get_char_matches() {
        check(ssx3_ee::s08::f_002a78e0, ini_parser_get_char, 200, &|c, seed| {
            let mut r = Rng(0x1111_000d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x8, DATA + 0x100);
            let b = if seed % 2 == 0 {
                (r.next_u32() | 0x80) as u8
            } else {
                r.next_u32() as u8
            };
            c.bus.write8(DATA + 0x100, b);
        });
    }

    #[test]
    fn ps2_sound_manager_ctor_matches() {
        check(ssx3_ee::s08::f_002a8c20, ps2_sound_manager_ctor, 50, &|c, seed| {
            let mut r = Rng(0x1111_000e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..210 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_sound_manager_pop_free_spu_block_matches() {
        check_lohi(ssx3_ee::s08::f_002a92b8, ps2_sound_manager_pop_free_spu_block, 200, &|c, seed| {
            let mut r = Rng(0x1111_000f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let head: u32 = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000, // negative after lw sign extension
                _ => 2,
            };
            c.bus.write32(DATA + 0x310, head);
            let block = DATA.wrapping_add(head.wrapping_mul(0x18).wrapping_add(0xc));
            c.bus.write32(block + 0x14, r.next_u32());
        });
    }

    #[test]
    fn stream_array_get_stream_handle_matches() {
        check(ssx3_ee::s08::f_002ab128, stream_array_get_stream_handle, 300, &|c, seed| {
            let mut r = Rng(0x1111_0010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let idx = (seed % 8) as u64;
            c.g[A1][0] = idx;
            let base = DATA + 0x100;
            c.bus.write32(DATA + 8, base);
            let entry = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(base + (idx as u32) * 4, entry);
            c.bus.write32(DATA + 0x200 + 8, r.next_u32());
        });
    }

    #[test]
    fn bank_monitor_instance_ctor_matches() {
        check(ssx3_ee::s08::f_002ab9e0, bank_monitor_instance_ctor, 50, &|c, seed| {
            let mut r = Rng(0x1111_0011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..48 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn bank_monitor_calc_doppler_pitch_matches() {
        check(ssx3_ee::s08::f_002abca0, bank_monitor_calc_doppler_pitch, 300, &|c, seed| {
            let mut r = Rng(0x1111_0012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x4560, (-1.0f32).to_bits()); // lower clamp
            c.bus.write32(GAME_GP - 0x455c, (1.0f32).to_bits()); // upper clamp
            c.bus.write32(GAME_GP - 0x4558, r.edge_f32()); // scale
            match seed % 4 {
                0 => {
                    c.f[12] = (-2.0f32).to_bits(); // ratio < lower
                    c.f[13] = 1.0f32.to_bits();
                }
                1 => {
                    c.f[12] = 2.0f32.to_bits(); // ratio > upper
                    c.f[13] = 1.0f32.to_bits();
                }
                2 => {
                    c.f[12] = 0.5f32.to_bits(); // inside
                    c.f[13] = 1.0f32.to_bits();
                }
                _ => {
                    c.f[12] = r.edge_f32();
                    c.f[13] = r.edge_f32();
                }
            }
        });
    }

    #[test]
    fn bank_monitor_timer_init_matches() {
        check(ssx3_ee::s08::f_002ad848, bank_monitor_timer_init, 50, &|c, seed| {
            let mut r = Rng(0x1111_0013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn bank_monitor_timer_is_free_matches() {
        check(ssx3_ee::s08::f_002ad930, bank_monitor_timer_is_free, 100, &|c, seed| {
            let mut r = Rng(0x1111_0014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, match seed % 3 {
                0 => 0xffff_ffff,
                1 => 0,
                _ => r.next_u32(),
            });
        });
    }
}
