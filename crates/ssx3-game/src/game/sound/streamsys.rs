//! `src/sound/streamsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
#[cfg(test)]
pub(crate) use crate::game::lib::snd::SND_GETSTREAM_COUNT;
#[cfg(test)]
pub(crate) use crate::game::lib::snd::SND_GETSTREAM_TABLE;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::replay::replaycache::destroy_if_flag;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_9df0, streambuffer_dtor, "StreamBuffer_dtor"),
    (0x002a_9e78, stream_buffer_set_file, "StreamBuffer_SetFile"),
    (
        0x002a_9f30,
        stream_buffer_set_base_volume,
        "StreamBuffer_SetBaseVolume",
    ),
    (
        0x002a_a428,
        stream_buffer_get_state,
        "StreamBuffer_GetState",
    ),
    (
        0x002a_a7f0,
        stream_array_create_stream,
        "StreamArray_CreateStream",
    ),
    (
        0x002a_ab98,
        stream_array_set_stream_file,
        "StreamArray_SetStreamFile",
    ),
    (
        0x002a_abd0,
        stream_array_destroy_stream,
        "StreamArray_DestroyStream",
    ),
    (0x002a_ae40, stream_array_stop, "StreamArray_Stop"),
    (
        0x002a_b0d0,
        stream_array_set_volume_scale_ptr,
        "StreamArray_SetVolumeScalePtr",
    ),
    (
        0x002a_b128,
        stream_array_get_stream_handle,
        "StreamArray_GetStreamHandle",
    ),
    (
        0x002a_b188,
        stream_array_get_stream,
        "StreamArray_GetStream",
    ),
    (
        0x002a_b9e0,
        bank_monitor_instance_ctor,
        "BankMonitorInstance_ctor",
    ),
    (
        0x002a_ba18,
        bank_monitor_instance_dtor,
        "BankMonitorInstance_dtor",
    ),
    (
        0x002a_bca0,
        bank_monitor_calc_doppler_pitch,
        "BankMonitor_CalcDopplerPitch",
    ),
    (
        0x002a_bce8,
        sound_request_set_bank_sample,
        "SoundRequest_SetBankSample",
    ),
    (
        0x002a_cb30,
        bank_monitor_instance_resume,
        "BankMonitorInstance_Resume",
    ),
    (0x002a_de88, sound_manager_ref_ctor, "SoundManagerRef_ctor"),
    (0x002a_dee8, sound_manager_ref_dtor, "SoundManagerRef_dtor"),
    (
        0x002a_c7f0,
        bank_monitor_instance_update_range,
        "BankMonitorInstance_UpdateRange",
    ),
    (
        0x002a_9f80,
        stream_buffer_update_volume,
        "StreamBuffer_UpdateVolume",
    ),
    (
        0x002a_a020,
        stream_buffer_get_status,
        "StreamBuffer_GetStatus",
    ),
    (
        0x002a_ac28,
        stream_array_update_all,
        "StreamArray_UpdateAll",
    ),
    (
        0x002a_a068,
        stream_buffer_get_request_status,
        "StreamBuffer_GetRequestStatus",
    ),
    (0x002a_ae08, stream_array_get_state, "StreamArray_GetState"),
    (
        0x002a_a108,
        stream_buffer_get_time_remaining,
        "StreamBuffer_GetTimeRemaining",
    ),
    (
        0x002a_a1b8,
        stream_buffer_is_playing,
        "StreamBuffer_IsPlaying",
    ),
    (
        0x002a_ae78,
        stream_array_get_time_remaining,
        "StreamArray_GetTimeRemaining",
    ),
    (
        0x002a_b028,
        stream_array_is_playing,
        "StreamArray_IsPlaying",
    ),
    (
        0x002a_bb38,
        bank_monitor_instance_reset,
        "BankMonitorInstance_Reset",
    ),
    (
        0x002a_c180,
        bank_monitor_instance_fade_out,
        "BankMonitorInstance_FadeOut",
    ),
    (
        0x002a_bd48,
        bank_monitor_instance_update_distance,
        "BankMonitorInstance_UpdateDistance",
    ),
    (
        0x002a_c220,
        bank_monitor_instance_play,
        "BankMonitorInstance_Play",
    ),
    (
        0x002a_cb80,
        bank_monitor_instance_fade_volume,
        "BankMonitorInstance_FadeVolume",
    ),
    (0x002a_9ed8, stream_buffer_set_pan, "StreamBuffer_SetPan"),
    (
        0x002a_a210,
        stream_buffer_get_current_request,
        "StreamBuffer_GetCurrentRequest",
    ),
    (
        0x002a_b150,
        stream_array_get_current_request,
        "StreamArray_GetCurrentRequest",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::STREAM_BUFFER_DTOR;

    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4;

    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;

    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;

    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;

    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;

    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;

    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;

    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
}

/// Offset of the stream-array element table pointer.
pub(crate) const STREAM_ARRAY_TABLE: u32 = 0x8;

/// Offset of a stream's volume-scale pointer.
pub(crate) const STREAM_VOLUME_SCALE_PTR: u32 = 0x10;

/// `void StreamArray_SetVolumeScalePtr(StreamArray* a, int i, float* p)`
/// (0x002ab0d0): `streams[i]->volumeScalePtr = p` (no-op if slot `i` is null).
/// `a1` is left holding the stream pointer (or 0).
pub fn stream_array_set_volume_scale_ptr(c: &mut Cpu) {
    c.cycles += 6;
    let array = Addr(abi::arg(c, 0));
    c.g[V0][0] = array.off(STREAM_ARRAY_TABLE).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    if c.g[A1][0] != 0 {
        Addr(c.g[A1][0] as u32)
            .off(STREAM_VOLUME_SCALE_PTR)
            .sw(c, c.g[A2][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* StreamArray_GetStream(StreamArray* a, int i)` (0x002ab188): returns
/// `streams[i]`. `v0` first holds the table pointer, then the element; `a1` is
/// left holding the element address.
pub fn stream_array_get_stream(c: &mut Cpu) {
    c.cycles += 5;
    let array = Addr(abi::arg(c, 0));
    c.g[V0][0] = array.off(STREAM_ARRAY_TABLE).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
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

/// `int StreamBuffer_GetState(StreamBuffer *b)` (0x002aa428): fill a stack
/// status block and return its first word.
pub fn stream_buffer_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::STREAM_BUFFER_GET_STATUS, 0x002a_a438)?;
        let block = frame.at(c, 0);
        c.g[V0][0] = Addr(block).lw(c);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitorInstance_dtor(BankMonitorInstance *this, int flags)`
/// (0x2aba18).
pub fn bank_monitor_instance_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002a_ba30))
}

// ---------------------------------------------------------------------------
// Stream array / speech / song / music player.

/// `void StreamArray_SetStreamFile(StreamArray* a, int i, const char* file)`
/// (0x2aab98): call the setter of stream slot `i` if it exists.
///
/// Leaves the slot list base in `v0`.
pub fn stream_array_set_stream_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            callee::STREAM_BUFFER_SET_FILE.call(c, 0x002a_abc0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_Stop(StreamArray* a, int i)` (0x2aae40): stop stream slot
/// `i` if it exists. Leaves the slot list base in `v0`.
pub fn stream_array_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            callee::STREAM_BUFFER_STOP.call(c, 0x002a_ae68)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitorInstance_Resume(void* self)` (0x2acb30): resume a paused
/// bank monitor instance and restart its MIDI slot.
pub fn bank_monitor_instance_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::BANK_MONITOR_INSTANCE_POLL_FREE.call(c, 0x002a_cb44)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1000;
            callee::BANK_MONITOR_INSTANCE_SET_PITCH.call(c, 0x002a_cb58)?;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = Addr(this).off(4).lw(c);
                call_ptr(c, table::SND_MIDI_RESUME, 0x002a_cb70)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_DestroyStream(self, i)` (0x2aabd0): destroy and clear
/// entry `i` of the stream array.
pub fn stream_array_destroy_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V0));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_ac04);
            c.g[A1][0] = 3;
            callee::STREAM_BUFFER_DTOR.call(c, 0x002a_ac04)?;
            c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        } else {
            c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SoundManagerRef* SoundManagerRef_ctor(SoundManagerRef* self, void* a1)`
/// (0x2ade88): install the reference vtable, create the sound manager and
/// pass the argument on.
pub fn sound_manager_ref_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c00));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
        Addr(p(c.g[GP][0], 0x68c)).sw(c, c.g[S0][0]);
        call_ptr(c, 0x002a_8fa0, 0x002a_deb4)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = lw_2(c, c.g[V0][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V1][0], 8)).lh(c);
        c.g[A2][0] = lw_2(c, p(c.g[V1][0], 0xc));
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_ded0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Audio / streaming.

/// `void StreamBuffer_SetFile(StreamBuffer* buf, const char* name)`
/// (0x2a9e78): look the name up and register it on the buffer.
pub fn stream_buffer_set_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let buf = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[S1][0] = buf;
        c.g[S2][0] = name;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7f38, 0x002a_9e9c)?;
        let missing = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S1, 0x14));
        if !missing {
            c.cycles += 7;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x002a_9eb0)?;
            c.g[A1][0] = Addr(at(c, S1, 0x7c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_1ec8, 0x002a_9ebc)?;
            Addr(at(c, S1, 0x74)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_SetBaseVolume(StreamBuffer* b, u8 volume)` (0x2a9f30):
/// store the volume byte at +0xc.
pub fn stream_buffer_set_base_volume(c: &mut Cpu) {
    c.cycles += 2;
    let b = abi::arg(c, 0);
    let v = c.g[A1][0];
    Addr(b.wrapping_add(0xc)).sb(c, v);
    abi::ret(c);
}

/// `void SoundRequest_SetBankSample(SoundRequest* r, int bank, int sample)`
/// (0x2abce8): store the bank at +0xc and the sample at +0x10.
pub fn sound_request_set_bank_sample(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    let sample = c.g[A2][0];
    Addr(r.wrapping_add(0x10)).sw(c, sample);
    let bank = c.g[A1][0];
    Addr(r.wrapping_add(0xc)).sw(c, bank);
    abi::ret(c);
}

/// `SoundManagerRef_dtor(self, flags)` (0x2adee8): destroy the referenced
/// sound manager and free the reference.
pub fn sound_manager_ref_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c00));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Addr(p_2(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x18)).lh(c);
        c.g[V1][0] = Addr(p_2(c.g[V0][0] as u32, 0x1c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_df24)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        call_ptr(c, 0x0031_7e50, 0x002a_df2c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let no_delete = c.g[S0][0] == 0;
        Addr(gp(c, 0x68c)).sw(c, 0);
        if !no_delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002a_df40)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* StreamArray_CreateStream(self, idx, heap, extra)` (0x002aa7f0):
/// allocate a stream buffer and store it in slot `idx`.
pub fn stream_array_create_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(word(c, A1) << 2);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = sx(0x480000);
        c.g[V0][0] = Addr(at_2(c, S1, 0x8)).lw(c);
        c.g[A1][0] = sx(0x480000u32.wrapping_add(0x3228));
        c.g[A0][0] = 0x84;
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(word(c, S0).wrapping_add(word(c, V0)));
        callee::MEM_ALLOC_NAMED.call(c, 0x002a_a834)?;
        c.g[A1][0] = Addr(at_2(c, S1, 0)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(word(c, A1).wrapping_add(0x1d8));
        callee::STREAM_BUFFER_CTOR.call(c, 0x002a_a84c)?;
        Addr(at_2(c, S0, 0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_dtor(self, flags)` (0x2a9df0): close the stream, free
/// its buffer and the buffer object, then the object itself when
/// `flags & 1`.
pub fn streambuffer_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, c.g[S0][0] as u32);
        let no_stream = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_stream {
            c.cycles += 5;
            c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 8));
            call_ptr(c, callee::SNDSTRM_CLOSE.addr, 0x002a_9e18)?;
            c.g[V1][0] = lw_at(c, p_3(r32(c, S0), 4));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 0x74));
            } else {
                c.cycles += 3;
                c.g[A0][0] = lw_at(c, c.g[S0][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x002a_9e38)?;
                    c.cycles += 1;
                }
                c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 0x74));
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 0x74));
        }
        c.cycles += 2;
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::MEM_FREE_3, 0x002a_9e4c)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE_4.addr, 0x002a_9e60)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod bank_range {
    use super::*;

    /// `BankMonitorInstance`: playing flag, audible range (0: always on).
    pub const PLAYING: u32 = 0x8;

    pub const RANGE: u32 = 0x8c;

    pub const BANK_MONITOR_INSTANCE_FADE_OUT: Callee = Callee {
        addr: 0x002a_c180,
        f: ee::s08::f_002ac180,
    };

    pub const BANK_MONITOR_INSTANCE_PLAY: Callee = Callee {
        addr: 0x002a_c220,
        f: ee::s08::f_002ac220,
    };
}

/// `bool BankMonitorInstance_UpdateRange(BankMonitorInstance* this, float
/// dist)` (0x2ac7f0): starts or fades a positional sound by distance.
/// Without a range: 1. Playing: beyond the range it fades out (0), else 1.
/// Stopped: inside the range it starts playing (1), else 0.
pub fn bank_monitor_instance_update_range(c: &mut Cpu) {
    use bank_range::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 8;
        frame.save_ra(c, 0);
        let this = Addr(abi::arg(c, 0));
        let dist = abi::farg(c, 0);
        let range = this.off(RANGE).read_fpu(c);
        abi::set_fpr(c, 0, FpuF32::ZERO);
        abi::set_fpr(c, 1, range);
        let no_range = range.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_range);
        c.g[V0][0] = 0x1;
        if !no_range {
            c.cycles += 3;
            c.g[V0][0] = this.off(PLAYING).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let beyond = abi::c_lt_s(c, range, dist);
                c.g[V0][0] = 0x1;
                if beyond {
                    c.cycles += 4;
                    BANK_MONITOR_INSTANCE_FADE_OUT.call(c, 0x002a_c834)?;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 4;
                let inside = abi::c_lt_s(c, dist, range);
                c.g[V0][0] = 0;
                if inside {
                    c.cycles += 3;
                    BANK_MONITOR_INSTANCE_PLAY.call(c, 0x002a_c854)?;
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetStatus (0x2aa020, streamsys.cpp)

pub(crate) const SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const SNDSTRM_GET_STATUS: u32 = 0x003b_7b48;

/// StreamBuffer field: the SND stream id.
pub(crate) const SB_STREAM: u32 = 0x8;

/// `int StreamBuffer_GetStatus(StreamBuffer* self, SndStreamStatus* out)`
/// (0x2aa020): `SNDSTRM_GetStatus(self->stream, out)` under the sound
/// library lock; returns its result.
pub fn stream_buffer_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        call_ptr(c, SND_LOCK, 0x002a_a03c)?;
        c.g[A0][0] = Addr(this.wrapping_add(SB_STREAM)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SNDSTRM_GET_STATUS, 0x002a_a048)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SND_UNLOCK, 0x002a_a050)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_UpdateAll (0x2aac28, streamsys.cpp)

/// StreamArray fields: buffer count, buffer pointer array.
pub(crate) const SA_COUNT: u32 = 0x4;

pub(crate) const SA_BUFFERS: u32 = 0x8;

pub(crate) const STREAM_BUFFER_UPDATE_VOLUME: Callee = Callee {
    addr: 0x002a_9f80,
    f: ee::s08::StreamBuffer_UpdateVolume,
};

/// `void StreamArray_UpdateAll(StreamArray* self)` (0x2aac28): updates the
/// volume of every non-null stream buffer (`StreamBuffer_UpdateVolume`).
pub fn stream_array_update_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SA_COUNT)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SA_BUFFERS)).lw(c);
            loop {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, S0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    STREAM_BUFFER_UPDATE_VOLUME.call(c, 0x002a_ac6c)?;
                }
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SA_COUNT)).lw(c);
                c.cycles += 4;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SA_BUFFERS)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002a_ac50);
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

// ---------------------------------------------------------------------------
// StreamBuffer_UpdateVolume (0x2a9f80)

/// StreamBuffer fields: the SNDSTRM stream handle, the volume byte, the
/// pointer to the volume scale (a float) and the starvation counter.
pub(crate) const SB_HANDLE: u32 = 0x8;

pub(crate) const SB_VOLUME: u32 = 0xc;

pub(crate) const SB_SCALE_PTR: u32 = 0x10;

pub(crate) const SB_STARVED: u32 = 0x80;

/// `StreamBuffer_Stop` (0x2aa408).
pub(crate) const STREAM_BUFFER_STOP: Callee = Callee {
    addr: 0x002a_a408,
    f: ee::s08::StreamBuffer_Stop,
};

/// `void StreamBuffer_UpdateVolume(StreamBuffer *sb)` (0x2a9f80): stops the
/// stream when its starvation counter (+0x80) has passed two seconds of
/// audio frames (and clears it); otherwise, with a volume scale (+0x10),
/// sets the stream's volume to `volume byte * scale` clamped to 0..127,
/// under the sound lock.
pub fn stream_buffer_update_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0028_b240, 0x002a_9f98)?; // Audio_GetFrameRate

        let sb = Addr(r32(c, S1));
        let starved = sb.off(SB_STARVED).lw(c);
        c.g[V1][0] = starved;
        let limit = sx(r32(c, V0) << 1);
        c.g[V0][0] = ((limit as i64) < (starved as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(sb.0);
            STREAM_BUFFER_STOP.call(c, 0x002a_9fb4)?;
            Addr(r32(c, S1)).off(SB_STARVED).sw(c, 0);
        } else {
            let scale_ptr = sb.off(SB_SCALE_PTR).lw(c);
            c.g[V1][0] = scale_ptr;
            c.cycles += 2;
            c.g[A0][0] = 0x7f;
            if scale_ptr != 0 {
                c.cycles += 18;
                let volume = sb.off(SB_VOLUME).lb(c);
                c.g[V0][0] = volume;
                let scale = Addr(scale_ptr as u32).read_fpu(c);
                c.f[1] = scale.0;
                let vol_f = FpuF32::from_i32(volume as i32);
                let prod = vol_f * scale;
                c.f[0] = prod.0;
                let mut v = prod.to_i32();
                c.f[1] = v as u32;
                let below = v < 128;
                c.g[V0][0] = below as u64;
                if !below {
                    v = 0x7f;
                }
                let nonneg = -1 < v;
                c.g[V1][0] = nonneg as u64;
                if !nonneg {
                    v = 0;
                }
                c.g[S0][0] = sx(v as u32);
                call_ptr(c, 0x003b_58a0, 0x002a_9ff8)?; // SND_Lock
                c.g[A0][0] = Addr(r32(c, S1)).off(SB_HANDLE).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003b_7e70, 0x002a_a004)?; // SNDSTRM_SetVolume
                call_ptr(c, 0x003b_58d8, 0x002a_a00c)?; // SND_Unlock
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

// ---------------------------------------------------------------------------
// StreamBuffer_GetRequestStatus (0x2aa068, streamsys.cpp)

/// `SND_Lock` / `SND_Unlock` (sound library lock, through the table).
pub(crate) const SBRS_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SBRS_SND_UNLOCK: u32 = 0x003b_58d8;

/// `SNDSTRM_GetRequestStatus(int request, SndRequestStatus* out)` (another shard).
pub(crate) const SBRS_SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;

/// `StreamBuffer_GetStatus(self, SndStreamStatus* out)`.
pub(crate) const SBRS_STREAM_BUFFER_GET_STATUS: Callee = Callee {
    addr: 0x002a_a020,
    f: ee::s08::StreamBuffer_GetStatus,
};

/// SndStreamStatus field: the id of the stream's current request.
pub(crate) const SBRS_STATUS_REQUEST: u32 = 0x4;

/// `int StreamBuffer_GetRequestStatus(StreamBuffer* self, SndRequestStatus*
/// out, SndStreamStatus* scratch)` (0x2aa068): reads the stream status into
/// `scratch` (or a frame-local buffer when null), then, under the sound
/// library lock, `SNDSTRM_GetRequestStatus(status.request, out)`; returns its
/// result.
pub fn stream_buffer_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        // movn: the caller's status buffer, else the frame's own.
        c.g[S0][0] = if c.g[A2][0] != 0 {
            c.g[A2][0]
        } else {
            c.g[SP][0]
        };
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[S0][0];
        SBRS_STREAM_BUFFER_GET_STATUS.call(c, 0x002a_a08c)?;
        call_ptr(c, SBRS_SND_LOCK, 0x002a_a094)?;
        let status = r32(c, S0);
        c.g[A0][0] = Addr(status.wrapping_add(SBRS_STATUS_REQUEST)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SBRS_SNDSTRM_GET_REQUEST_STATUS, 0x002a_a0a0)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SBRS_SND_UNLOCK, 0x002a_a0a8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_GetState (0x2aae08, streamsys.cpp)

/// StreamArray: pointer to its array of StreamBuffer pointers.
pub(crate) const OC264_SA_BUFFERS: u32 = 0x8;

pub(crate) const OC264_STREAM_BUFFER_GET_STATE: Callee = Callee {
    addr: 0x002a_a428,
    f: ee::s08::StreamBuffer_GetState,
};

/// `int StreamArray_GetState(StreamArray* self, int i)` (0x2aae08): the
/// play state of stream buffer `i` (`StreamBuffer_GetState`), or 0 when
/// that slot is empty.
pub fn stream_array_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let slot_off = r32(c, A1) << 2;
        c.g[A1][0] = sx(slot_off);
        frame.save_ra(c, 0);
        let buffers = Addr(r32(c, A0).wrapping_add(OC264_SA_BUFFERS)).lw(c);
        c.g[V0][0] = buffers;
        let slot = slot_off.wrapping_add(buffers as u32);
        c.g[A1][0] = sx(slot);
        c.g[A0][0] = Addr(slot).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            OC264_STREAM_BUFFER_GET_STATE.call(c, 0x002a_ae30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_IsPlaying (0x2aa1b8)
// ---------------------------------------------------------------------------

/// SndRequestStatus: the request's state (2 = playing).
pub(crate) const SBIP_REQUEST_STATE_PLAYING: u64 = 2;

/// `bool StreamBuffer_IsPlaying(StreamBuffer* self)` (0x2aa1b8): whether the
/// stream's current request is playing (`StreamBuffer_GetRequestStatus` into
/// a frame-local status, state == 2).
pub fn stream_buffer_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x002a_a068,
            f: ee::s08::f_002aa068,
        }
        .call(c, 0x002a_a1cc)?;
        let state = Addr(sp(c)).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = (state == SBIP_REQUEST_STATE_PLAYING) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_Reset

/// `BankMonitorInstance` fields cleared by `Reset`.
pub(crate) const BMI_RESET_KIND: u32 = 0x0;

pub(crate) const BMI_RESET_FADING: u32 = 0x54;

pub(crate) const BMI_RESET_TIMER: u32 = 0x58;

pub(crate) const BMI_RESET_PRIORITY: u32 = 0xb0;

pub(crate) const BMI_RESET_KEY: u32 = 0xb4;

pub(crate) const BMI_RESET_ID: u32 = 0xb8;

/// `void BankMonitorInstance_Reset(BankMonitorInstance* self)` (0x2abb38):
/// fade the instance out, then free it: kind 0, not fading, no timer,
/// priority 0x8000, key -1, id 0xffff.
pub fn bank_monitor_instance_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002a_c180,
            f: ee::s08::BankMonitorInstance_FadeOut,
        }
        .call(c, 0x002a_bb4c)?;
        let inst = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = 0x8000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = 0xffff;
        inst.off(BMI_RESET_PRIORITY).sw(c, c.g[V1][0]);
        inst.off(BMI_RESET_ID).sw(c, c.g[V0][0]);
        inst.off(BMI_RESET_KEY).sw(c, c.g[A0][0]);
        inst.off(BMI_RESET_KIND).sw(c, 0);
        inst.off(BMI_RESET_FADING).sw(c, 0);
        inst.off(BMI_RESET_TIMER).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_FadeOut (0x2ac180, streamsys.cpp)

/// Bank monitor instance fields: kind (+0; 1 = patch voice, 2 = stream), the
/// sound handle (+4), the playing flag (+8) and the fading flag (+0x54).
pub(crate) const BMI_KIND: u32 = 0x0;

pub(crate) const BMI_HANDLE: u32 = 0x4;

pub(crate) const BMI_PLAYING: u32 = 0x8;

pub(crate) const BMI_FADING: u32 = 0x54;

/// Fade length and target volume passed to the fade calls.
pub(crate) const BMI_FADE_TICKS: u64 = 0x19;

/// `SND_Lock` / `SND_Unlock` and the two fade functions (cross-shard
/// direct calls, emitted as table calls).
pub(crate) const BMI_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const BMI_SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const BMI_FADE_VOICE: u32 = 0x003b_8530;

pub(crate) const BMI_FADE_STREAM: u32 = 0x003b_9f38;

/// `void BankMonitorInstance_FadeOut(BankMonitorInstance* self)`
/// (0x2ac180): once only (the fading flag), clears the playing flag and
/// fades the instance's voice (kind 1, `SND_FadeVolume`) or stream (kind 2)
/// to silence over 25 ticks, under the sound lock. Frame 0x20 (ra at 0, s0
/// at 0x10).
pub fn bank_monitor_instance_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let inst = r32(c, S0);
        c.g[V0][0] = c.lw(inst.wrapping_add(BMI_FADING));
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[V1][0] = c.lw(inst.wrapping_add(BMI_KIND));
            c.sw(inst.wrapping_add(BMI_PLAYING), 0);
            c.sw(inst.wrapping_add(BMI_FADING), 1);
            // (fade function, its three return addresses)
            let fade = if c.g[V1][0] == 1 {
                c.cycles += 10;
                Some((BMI_FADE_VOICE, [0x002a_c1cc, 0x002a_c1dc, 0x002a_c1e4]))
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if c.g[V1][0] == 2 {
                    c.cycles += 9;
                    Some((BMI_FADE_STREAM, [0x002a_c1f4, 0x002a_c204, 0x002a_c20c]))
                } else {
                    c.cycles += 2;
                    None
                }
            };
            if let Some((target, rets)) = fade {
                call_ptr(c, BMI_SND_LOCK, rets[0])?;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(BMI_HANDLE));
                c.g[A1][0] = BMI_FADE_TICKS;
                c.g[A2][0] = u64::MAX;
                call_ptr(c, target, rets[1])?;
                call_ptr(c, BMI_SND_UNLOCK, rets[2])?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_GetTimeRemaining (0x2aae78, sound/streamsys.cpp)

/// StreamArray: pointer to its array of StreamBuffer pointers.
pub(crate) const SAT_BUFFERS: u32 = 0x8;

/// `StreamBuffer_GetTimeRemaining` (0x2aa108).
pub(crate) const SAT_STREAM_BUFFER_GET_TIME_REMAINING: Callee = Callee {
    addr: 0x002a_a108,
    f: ee::s08::StreamBuffer_GetTimeRemaining,
};

/// `float StreamArray_GetTimeRemaining(StreamArray* self, int i)` (0x2aae78):
/// the time left in stream buffer `i` (`StreamBuffer_GetTimeRemaining`), or
/// 0.0 when `i` is negative or that slot is empty.
pub fn stream_array_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mut present = false;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 6;
            let buffers = Addr(r32(c, A0).wrapping_add(SAT_BUFFERS)).lw(c);
            c.g[V1][0] = buffers;
            let slot = (r32(c, A1) << 2).wrapping_add(buffers as u32);
            c.g[V0][0] = sx(slot);
            c.g[A0][0] = Addr(slot).lw(c);
            present = c.g[A0][0] != 0;
        }
        if present {
            c.cycles += 3;
            SAT_STREAM_BUFFER_GET_TIME_REMAINING.call(c, 0x002a_aeb0)?;
        } else {
            c.cycles += 3;
            c.f[0] = 0;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetTimeRemaining (0x2aa108)
// ---------------------------------------------------------------------------

/// `StreamBuffer_GetRequestStatus(self, status, scratch)`.
pub(crate) const STREAM_BUFFER_GET_REQUEST_STATUS: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::StreamBuffer_GetRequestStatus,
};

/// Request status record (on the stack): state (2 = playing) and the
/// remaining time at +8.
pub(crate) const SB_REQ_STATE: u32 = 0x0;

pub(crate) const SB_REQ_REMAINING: u32 = 0x8;

pub(crate) const SB_REQ_PLAYING: u64 = 2;

/// `float StreamBuffer_GetTimeRemaining(StreamBuffer* self)` (0x2aa108):
/// asks for the current request's status (into a stack record); while it
/// is playing (state 2) returns its remaining time as a float, else 0.
pub fn stream_buffer_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        STREAM_BUFFER_GET_REQUEST_STATUS.call(c, 0x002a_a11c)?;
        let state = Addr(frame.at(c, SB_REQ_STATE)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = SB_REQ_PLAYING;
        frame.restore_ra(c, 0x10);
        if state == SB_REQ_PLAYING {
            c.cycles += 3;
            let remaining = Addr(frame.at(c, SB_REQ_REMAINING)).lw(c);
            c.g[V0][0] = remaining;
            abi::set_fpr(c, 0, FpuF32::from_i32(remaining as i32));
        } else {
            c.cycles += 3;
            abi::set_fpr(c, 0, FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_IsPlaying.

/// `StreamArray::buffers`: the array of `StreamBuffer*` slots.
pub(crate) const STREAM_ARRAY_BUFFERS: u32 = 0x8;

/// `bool StreamArray_IsPlaying(StreamArray* self, int i)` (0x2ab028): whether
/// stream slot `i` holds a buffer that is playing (`StreamBuffer_IsPlaying`);
/// false for an empty slot.
pub fn stream_array_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(STREAM_ARRAY_BUFFERS)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002a_a1b8,
                f: ee::s08::f_002aa1b8,
            }
            .call(c, 0x002a_b050)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_UpdateDistance (0x2abd48)

/// Listener object -> its position-source vtable (slot 0x30/0x34: distance²).
pub(crate) const LISTENER_VTABLE: u32 = 0x8d8;

/// BankMonitorInstance -> distance to the listener.
pub(crate) const BANK_MONITOR_DISTANCE: u32 = 0x5c;

/// `void BankMonitorInstance::UpdateDistance(Listener* l)` (0x2abd48): asks
/// the listener's virtual (vtable slot 0x30/0x34, `this` adjusted by the
/// slot's delta) for the squared distance to this instance (with a 0x20-byte
/// scratch buffer on the stack) and stores its square root at `self+0x5c`.
pub fn bank_monitor_instance_update_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        let listener = c.g[A3][0] as u32;
        c.g[V1][0] = Addr(listener.wrapping_add(LISTENER_VTABLE)).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(0x30).lh(c);
        c.g[V0][0] = vt.off(0x34).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(listener.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002a_bd78)?;
        let dist = crate::ps2f::FpuF32(c.f[0]).sqrt();
        c.f[0] = dist.0;
        frame.restore_ra(c, 0x20);
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(BANK_MONITOR_DISTANCE)).write_fpu(c, dist);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_Play (0x2ac220, streamsys.cpp).

/// Play-state stack the bank monitor plays through (`a1`): the current
/// play descriptor, a depth index and per-depth parameter arrays.
pub(crate) const BANK_PLAY_CUR_DESC: u32 = 0x04;

// 24-byte descriptor
pub(crate) const BANK_PLAY_DEPTH: u32 = 0x1c;

pub(crate) const BANK_PLAY_DESCS: u32 = 0x20;

// -> descriptors, 0x18 each
pub(crate) const BANK_PLAY_F24: u32 = 0x24;

// inline words
pub(crate) const BANK_PLAY_F34: u32 = 0x34;

// -> words
pub(crate) const BANK_PLAY_VOLUME: u32 = 0x38;

// -> words (clamped to 0..127)
pub(crate) const BANK_PLAY_F3C: u32 = 0x3c;

// -> words
pub(crate) const BANK_PLAY_F40: u32 = 0x40;

// inline 8-byte entries
pub(crate) const BANK_PLAY_ENV: u32 = 0x70;

// -> 16-byte envelopes
/// Monitor instance fields (`a0`).
pub(crate) const BANK_MON_KIND: u32 = 0x00;

// 1: bank patch, 2: MIDI, 3: none
pub(crate) const BANK_MON_HANDLE: u32 = 0x04;

pub(crate) const BANK_MON_PLAYING: u32 = 0x08;

pub(crate) const BANK_MON_BANK: u32 = 0x0c;

pub(crate) const BANK_MON_PATCH: u32 = 0x10;

pub(crate) const BANK_MON_F54: u32 = 0x54;

pub(crate) const BANK_MON_F60: u32 = 0x60;

pub(crate) const BANK_PLAY_MEMSET: u32 = 0x0041_6210;

pub(crate) const BANK_PLAY_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const BANK_PLAY_SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const BANK_PLAY_SND_PLAY_PATCH: u32 = 0x003b_6820;

pub(crate) const BANK_PLAY_SNDMIDI_PLAY: u32 = 0x003b_9a00;

/// `lw ra_, depth(st); lw rb, field(st); sll ra_, shift; addu ra_, rb;
/// sw val, extra(ra_)`: one per-depth array store, in the original's
/// registers.
pub(crate) fn bank_play_slot_store(
    c: &mut Cpu,
    ra_: usize,
    rb: usize,
    field: u32,
    shift: u32,
    extra: u32,
    val: u64,
) {
    let st = c.g[S1][0] as u32;
    c.g[ra_][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[rb][0] = c.lw(st.wrapping_add(field));
    c.g[ra_][0] = sx((c.g[ra_][0] as u32) << shift);
    c.g[ra_][0] = sx((c.g[ra_][0] as u32).wrapping_add(c.g[rb][0] as u32));
    c.sw((c.g[ra_][0] as u32).wrapping_add(extra), val);
}

/// `v0 = &volume[depth]` (`lw v0, depth; sll 2; addu a2`).
pub(crate) fn bank_play_volume_at(c: &mut Cpu) -> u32 {
    let st = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] as u32
}

/// The shared head of both play kinds: push a new depth (copy of the
/// current descriptor), reset its per-depth state, clear its 8-byte entry
/// (memset, returning to `memset_ret`), set the default envelope (100, 90,
/// 50), volume 127, flag 1, take the instance's +0x60 word, and clamp the
/// pushed descriptor's first byte to 0..127 from `volume[depth]`. Leaves
/// `v0` = the clamped byte and `a0` = its address (the `sb` happens in the
/// caller's next block).
pub(crate) fn bank_play_push_depth(c: &mut Cpu, memset_ret: u32) -> crate::call::Flow {
    c.cycles += 142;
    let st = c.g[S1][0] as u32;
    // ++depth; descs[depth] = cur (unaligned 24-byte copy).
    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[S0][0] = 0x18;
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
    c.g[A1][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[A2][0] = 8;
    c.sw(st.wrapping_add(BANK_PLAY_DEPTH), c.g[V1][0]);
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.mult(0, V0, V0, S0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let src = st.wrapping_add(BANK_PLAY_CUR_DESC);
    c.ldl(V1, src.wrapping_add(7));
    c.ldr(V1, src);
    c.ldl(A0, src.wrapping_add(0xf));
    c.ldr(A0, src.wrapping_add(8));
    c.ldl(A3, src.wrapping_add(0x17));
    c.ldr(A3, src.wrapping_add(0x10));
    let dst = c.g[V0][0] as u32;
    c.sdl(V1, dst.wrapping_add(7));
    c.sdr(V1, dst);
    c.sdl(A0, dst.wrapping_add(0xf));
    c.sdr(A0, dst.wrapping_add(8));
    c.sdl(A3, dst.wrapping_add(0x17));
    c.sdr(A3, dst.wrapping_add(0x10));
    // f24[depth] (inline) = 0.
    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx(st.wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(BANK_PLAY_F24), 0);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_F34, 2, 0, 0);
    // volume[depth] = descs[depth].c[0] (signed byte).
    c.g[A3][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[T0][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S0);
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_VOLUME));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_F3C, 2, 0, 0);
    // memset(&f40[depth], 0, 8).
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx(st.wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(BANK_PLAY_F40));
    call_ptr(c, BANK_PLAY_MEMSET, memset_ret)?;

    // Per-depth defaults.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    bank_play_slot_store(c, V1, V0, 0x60, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x64, 2, 0, 0);
    bank_play_slot_store(c, V1, V0, 0x68, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x6c, 2, 0, 0);
    bank_play_slot_store(c, V1, V0, BANK_PLAY_ENV, 4, 4, 0);
    let (a0, a1, a2, a3, s3) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[S3][0]);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_ENV, 4, 0, a0);
    bank_play_slot_store(c, V1, V0, BANK_PLAY_ENV, 4, 8, a1);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_ENV, 4, 0xc, a2);
    bank_play_slot_store(c, V1, V0, 0x74, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x80, 2, 0, 0);
    bank_play_slot_store(c, V1, V0, 0x84, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x78, 2, 0, a3);
    bank_play_slot_store(c, V1, V0, 0x7c, 2, 0, s3);
    // f3c[depth] = instance->f60; volume[depth] = 0.
    let inst = c.g[S2][0] as u32;
    let f60 = c.lw(inst.wrapping_add(BANK_MON_F60));
    // (the original loads depth and the pointer before this word; loads only)
    let st = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_F3C));
    c.g[V1][0] = f60;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_VOLUME, 2, 0, 0);

    // descs[depth].c[0] = clamp(volume[depth], 0, 127).
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[A2][0] = c.lw(st.wrapping_add(BANK_PLAY_VOLUME));
    c.mult(0, A0, A0, S0);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let below_max = c.g[V0][0] != 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    if below_max {
        c.cycles += 6;
        let at = bank_play_volume_at(c);
        c.g[V1][0] = c.lw(at);
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
        if negative {
            return Ok(());
        }
    }
    c.cycles += 7;
    let at = bank_play_volume_at(c);
    c.g[V1][0] = c.lw(at);
    c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0x7f;
    } else {
        c.cycles += 4;
        let at = bank_play_volume_at(c);
        c.g[V0][0] = c.lbu(at);
    }
    Ok(())
}

/// `void BankMonitorInstance_Play(BankMonitorInstance* self, PlayStack* st)`
/// (0x2ac220): mark the instance playing, then by its kind push a play
/// depth with default parameters and start it under the sound lock: kind 1
/// plays a bank patch (`SND_PlayPatch(bank, patch, desc)`), kind 2 a MIDI
/// stream (`SNDMIDI_Play(desc, bank, patch, -1)`); the handle goes to
/// `self+4` and the depth is popped again. Other kinds do nothing.
pub fn bank_monitor_instance_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 2;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = 1;
        frame.save_ra(c, 0);
        let inst = c.g[S2][0] as u32;
        c.sw(inst.wrapping_add(BANK_MON_PLAYING), c.g[S3][0]);
        c.g[V1][0] = c.lw(inst.wrapping_add(BANK_MON_KIND));
        c.sw(inst.wrapping_add(BANK_MON_F54), 0);
        let kind = c.g[V1][0];

        if kind == 2 {
            // MIDI stream: SNDMIDI_Play(&descs[depth], bank, patch, -1).
            bank_play_push_depth(c, 0x002a_c5e4)?;
            c.cycles += 11;
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            call_ptr(c, BANK_PLAY_SND_LOCK, 0x002a_c79c)?;
            let st = c.g[S1][0] as u32;
            let inst = c.g[S2][0] as u32;
            c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
            c.g[V0][0] = 0x18;
            c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
            c.g[A3][0] = u64::MAX;
            c.mult(0, A0, A0, V0);
            c.g[A1][0] = c.lw(inst.wrapping_add(BANK_MON_BANK));
            c.g[A2][0] = c.lw(inst.wrapping_add(BANK_MON_PATCH));
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, BANK_PLAY_SNDMIDI_PLAY, 0x002a_c7c0)?;
            bank_play_finish(c, frame)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((kind as i64) < 3) as u64;
            frame.restore(c, S0, 0x40);
            if c.g[V0][0] == 0 {
                // Kind 3 and up: nothing to play.
                c.cycles += 1;
                frame.restore(c, S1, 0x30);
            } else {
                c.cycles += 2;
                if kind != c.g[S3][0] {
                    frame.restore(c, S1, 0x30);
                } else {
                    // Bank patch: SND_PlayPatch(bank, patch, &descs[depth]).
                    bank_play_push_depth(c, 0x002a_c338)?;
                    c.cycles += 12;
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    call_ptr(c, BANK_PLAY_SND_LOCK, 0x002a_c4f0)?;
                    let st = c.g[S1][0] as u32;
                    let inst = c.g[S2][0] as u32;
                    c.g[A2][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
                    c.g[V0][0] = 0x18;
                    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
                    c.mult(0, A2, A2, V0);
                    c.g[A0][0] = c.lw(inst.wrapping_add(BANK_MON_BANK));
                    c.g[A1][0] = c.lw(inst.wrapping_add(BANK_MON_PATCH));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    call_ptr(c, BANK_PLAY_SND_PLAY_PATCH, 0x002a_c510)?;
                    bank_play_finish(c, frame)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Common tail of both play kinds: store the handle, `SND_Unlock`, pop
/// the depth, restore s0/s1.
pub(crate) fn bank_play_finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    let inst = c.g[S2][0] as u32;
    c.sw(inst.wrapping_add(BANK_MON_HANDLE), c.g[V0][0]);
    call_ptr(c, BANK_PLAY_SND_UNLOCK, 0x002a_c7c8)?;
    let st = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.sw(st.wrapping_add(BANK_PLAY_DEPTH), c.g[V0][0]);
    frame.restore(c, S0, 0x40);
    c.cycles += 1;
    frame.restore(c, S1, 0x30);
    Ok(())
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_FadeVolume (0x2acb80)

/// Bank-monitor instance: the voice kind (+0: 1 sound, 2 MIDI, 3 stream
/// buffer), the sound / MIDI handle (+4), the stream buffer (+0xc), the
/// "stopping" flag (+0x54) and two "volume touched" flags (+0x58, +0x9c).
pub(crate) const FADE_VOLUME_HANDLE: u32 = 0x4;

pub(crate) const FADE_VOLUME_BUFFER: u32 = 0xc;

pub(crate) const FADE_VOLUME_STOPPING: u32 = 0x54;

pub(crate) const FADE_VOLUME_TOUCHED: u32 = 0x58;

pub(crate) const FADE_VOLUME_TOUCHED_2: u32 = 0x9c;

/// `void BankMonitorInstance_FadeVolume(Instance* self, int volume, int
/// touch, float seconds)` (0x2acb80): unless the instance is already
/// stopping, fades its voice to `volume` (clamped to 0..127) over
/// `seconds` (in centiseconds to the driver; 0 = set at once), under the
/// sound lock for sound and MIDI voices. Volume -1 means stop: the
/// instance is marked stopping and a stream buffer is stopped. `touch`
/// sets the two volume-touched flags first.
pub fn bank_monitor_instance_fade_volume(c: &mut Cpu) {
    const STOP: Callee = Callee {
        addr: 0x002a_a408,
        f: ee::s08::StreamBuffer_Stop,
    };
    const SET_BASE_VOLUME: Callee = Callee {
        addr: 0x002a_9f30,
        f: ee::s08::StreamBuffer_SetBaseVolume,
    };
    let frame = Frame { size: 0x40 };
    guest(c, |c| {
        c.cycles += 9;
        Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // volume
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12]; // seconds
        if c.g[A2][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let me = Addr(r32(c, S1));
            me.off(FADE_VOLUME_TOUCHED_2).sw(c, 1);
            me.off(FADE_VOLUME_TOUCHED).sw(c, 1);
        }

        c.cycles += 3;
        let me = Addr(r32(c, S1));
        c.g[V0][0] = me.off(FADE_VOLUME_STOPPING).lw(c);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            // Volume: -1 = stop, else clamped to 0..127.
            c.cycles += 3;
            c.g[A0][0] = u64::MAX;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
            if c.g[S0][0] == u64::MAX {
                c.cycles += 3;
                c.g[V0][0] = 1;
                me.off(FADE_VOLUME_STOPPING).sw(c, 1);
            } else {
                c.cycles += 4;
                c.g[V1][0] = 0x7f;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 0x7f;
                }
                c.g[A0][0] = (-1i64 < c.g[S0][0] as i64) as u64;
                if c.g[A0][0] == 0 {
                    c.g[S0][0] = 0;
                }
            }

            c.cycles += 4;
            let kind = me.lw(c);
            c.g[V1][0] = kind;
            c.g[V0][0] = ((kind as i64) < 3) as u64;
            if kind == 2 {
                // MIDI: SNDMIDI_SetVolume / SNDMIDI_FadeVolume.
                fade_volume_driver(
                    c,
                    0x002a_cc7c,
                    (0x003b_9e48, 0x002a_ccc0, 2),
                    (0x003b_9f38, 0x002a_ccb0),
                )?;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                let below_3 = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if !below_3 {
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    if kind == 3 {
                        // Stream buffer: stop, or set its base volume.
                        c.g[V0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_STOPPING).lw(c);
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_BUFFER).lw(c);
                            c.cycles += 3;
                            c.g[A1][0] = c.g[S0][0];
                            SET_BASE_VOLUME.call(c, 0x002a_ccf0)?;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_BUFFER).lw(c);
                            STOP.call(c, 0x002a_cce0)?;
                        }
                    } else {
                        c.cycles += 2;
                    }
                } else {
                    c.cycles += 2;
                    if kind == 1 {
                        // Sound: SND_SetVolume / SND_FadeVolume.
                        fade_volume_driver(
                            c,
                            0x002a_cc28,
                            (0x003b_85f0, 0x002a_cc6c, 4),
                            (0x003b_8530, 0x002a_cc5c),
                        )?;
                    } else {
                        c.cycles += 2;
                    }
                }
                frame.restore(c, S0, 0x20);
            }
        }

        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The sound / MIDI arm of [`bank_monitor_instance_fade_volume`]: under
/// `SND_Lock`, `set(handle, volume)` when `seconds` (f20) is 0, else
/// `fade(handle, (int)(seconds * 100), volume)`; then `SND_Unlock`. `set`
/// is `(address, return address, cycles before the call)`.
pub(crate) fn fade_volume_driver(
    c: &mut Cpu,
    lock_ret: u32,
    set: (u32, u32, u64),
    fade: (u32, u32),
) -> crate::call::Flow {
    c.cycles += 7;
    call_ptr(c, 0x003b_58a0, lock_ret)?; // SND_Lock
    c.f[0] = 0;
    let seconds = FpuF32(c.f[20]);
    let at_once = seconds.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, at_once);
    if at_once {
        c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_HANDLE).lw(c);
        c.cycles += set.2;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, set.0, set.1)?;
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x42c8_0000;
        c.g[A2][0] = c.g[S0][0];
        let centis = seconds * FpuF32(0x42c8_0000);
        abi::set_fpr(c, 0, centis);
        c.f[1] = centis.to_i32() as u32;
        c.g[A1][0] = sx(c.f[1]);
        c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_HANDLE).lw(c);
        call_ptr(c, fade.0, fade.1)?;
    }
    c.cycles += 4;
    call_ptr(c, 0x003b_58d8, 0x002a_ccc8) // SND_Unlock
}

// ---------------------------------------------------------------------------
// StreamArray_GetCurrentRequest (0x2ab150)
// ---------------------------------------------------------------------------

/// The array of stream buffers, at `self + 8`.
pub(crate) const STREAM_ARRAY_GET_CURRENT_REQUEST_BUFFERS: u32 = 0x8;

/// `int StreamArray_GetCurrentRequest(self, int i)` (0x2ab150): the current
/// request of stream `i`, or `-1` when that slot is empty.
pub fn stream_array_get_current_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] =
            c.lw((c.g[A0][0] as u32).wrapping_add(STREAM_ARRAY_GET_CURRENT_REQUEST_BUFFERS));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = u64::MAX; // -1
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002a_a210,
                f: ee::s08::StreamBuffer_GetCurrentRequest,
            }
            .call(c, 0x002a_b178)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_SetPan (0x2a9ed8)
// ---------------------------------------------------------------------------

/// The stream handle a `StreamBuffer` forwards to, at `+8`.
pub(crate) const STREAM_BUFFER_SET_PAN_HANDLE: u32 = 0x8;

/// The sound lock/unlock routine pointers (`D_0044FFxx`).
#[cfg(test)]
pub(crate) const STREAM_BUFFER_SET_PAN_LOCK_SLOT: u32 = 0x0044_c46c;

#[cfg(test)]
pub(crate) const STREAM_BUFFER_SET_PAN_UNLOCK_SLOT: u32 = 0x0044_c470;

/// `void StreamBuffer_SetPan(self, int left, int right)` (0x2a9ed8): set the
/// pan of the buffer's stream under the sound lock.
pub fn stream_buffer_set_pan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x003b_58a0, 0x002a_9efc)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STREAM_BUFFER_SET_PAN_HANDLE));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x003b_7f60, 0x002a_9f0c)?;
        call_ptr(c, 0x003b_58d8, 0x002a_9f14)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002aa210 StreamBuffer_GetCurrentRequest

/// `int StreamBuffer_GetCurrentRequest(StreamBuffer *self)` (0x2aa210): ask
/// the SND layer for the buffer's status; -1 when it has no request, else
/// the request's index (the second status word).
pub fn stream_buffer_get_current_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x002a_a020,
            f: ee::s08::StreamBuffer_GetStatus,
        }
        .call(c, 0x002a_a220)?;
        c.g[V1][0] = Addr(c.g[SP][0] as u32).lw(c);
        let no_request = c.g[V1][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffffu64;
        if !no_request {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[SP][0] as u32).off(4).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_init_6 as heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;

    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;

    pub(crate) use crate::game::replay::replaycache::tests::dtor_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn stream_array_set_volume_scale_ptr_matches() {
        check(
            ssx3_ee::s08::f_002ab0d0__generated,
            stream_array_set_volume_scale_ptr,
            300,
            &|c, seed| {
                let mut r = Rng(0x9abc_def0 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(DATA + STREAM_ARRAY_TABLE, table);
                fill(c, &mut r, table, 8);
                if seed % 2 == 0 {
                    for i in 0..4 {
                        c.bus.write32(table + i * 4, 0);
                    }
                } else {
                    for i in 0..4 {
                        c.bus.write32(table + i * 4, DATA + 0x300 + i * 0x20);
                    }
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
            },
        );
    }

    #[test]
    fn stream_array_get_stream_matches() {
        check(
            ssx3_ee::s08::f_002ab188__generated,
            stream_array_get_stream,
            300,
            &|c, seed| {
                let mut r = Rng(0xabcd_ef01 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.bus.write32(DATA + STREAM_ARRAY_TABLE, table);
                for i in 0..4 {
                    c.bus
                        .write32(table + i * 4, if seed % 3 == 0 { 0 } else { r.next_u32() });
                }
            },
        );
    }

    #[test]
    fn stream_array_get_stream_handle_matches() {
        check(
            ssx3_ee::s08::f_002ab128,
            stream_array_get_stream_handle,
            300,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn bank_monitor_instance_ctor_matches() {
        check(
            ssx3_ee::s08::f_002ab9e0,
            bank_monitor_instance_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x1111_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..48 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn bank_monitor_calc_doppler_pitch_matches() {
        check(
            ssx3_ee::s08::f_002abca0,
            bank_monitor_calc_doppler_pitch,
            300,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn stream_buffer_get_state_matches() {
        check(
            ee::s08::f_002aa428__generated,
            stream_buffer_get_state,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a428, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x1000 + 4 * i, r.next_u32() % 4);
                }
            },
        );
    }

    #[test]
    fn bank_monitor_instance_dtor_matches() {
        check(
            ee::s08::f_002aba18__generated,
            bank_monitor_instance_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2aba18),
        );
    }

    #[test]
    fn stream_array_set_stream_file_matches() {
        check(
            ee::s08::f_002aab98__generated,
            stream_array_set_stream_file,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aab_98, 0);
                let arr = DATA + 0x100;
                c.g[A0][0] = sx(arr);
                c.g[A1][0] = sx(seed % 16);
                c.g[A2][0] = sx(DATA + 0x300);
                // The slot list lives at arr + 8; the slots are arr + 0x200 + i*4.
                c.bus.write32(arr + 8, DATA + 0x200);
                let slot = DATA + 0x200 + 4 * (seed % 16);
                let entry = if seed % 3 == 0 { 0 } else { DATA + 0x320 };
                c.bus.write32(slot, entry);
                fill(c, &mut r, DATA + 0x300, 8);
            },
        );
    }

    #[test]
    fn stream_array_stop_matches() {
        check(
            ee::s08::f_002aae40__generated,
            stream_array_stop,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aae_40, 0);
                let arr = DATA + 0x100;
                c.g[A0][0] = sx(arr);
                c.g[A1][0] = sx(seed % 16);
                c.bus.write32(arr + 8, DATA + 0x200);
                let slot = DATA + 0x200 + 4 * (seed % 16);
                let entry = if seed % 3 == 0 { 0 } else { DATA + 0x340 };
                c.bus.write32(slot, entry);
                fill(c, &mut r, DATA + 0x340, 8);
            },
        );
    }

    #[test]
    fn bank_monitor_instance_resume_matches() {
        check(
            ee::s08::f_002acb30__generated,
            bank_monitor_instance_resume,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2acb_30, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
                // `*(self)` non-zero takes the "resume" path in PollFree.
                c.bus.write32(this, (seed % 3) as u32);
                c.bus.write32(this + 0x9c, 0);
                c.bus.write32(
                    this + 4,
                    match seed % 4 {
                        0 => u32::MAX,
                        n => n,
                    },
                );
            },
        );
    }

    #[test]
    fn stream_array_destroy_stream_matches() {
        check(
            ee::s08::f_002aabd0__generated,
            stream_array_destroy_stream,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x2aabd0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                let array = DATA + 0x2000;
                clear(c, this, 0x40);
                clear(c, array, 0x100);
                c.bus.write32(this + 8, array);
                let i = seed % 4;
                let buf = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x100)
                };
                if buf != 0 {
                    clear(c, buf, 0x40);
                }
                c.bus.write32(array + 4 * i, buf);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = i as u64;
            },
        );
    }

    #[test]
    fn sound_manager_ref_ctor_matches() {
        check(
            ee::s08::f_002ade88__generated,
            sound_manager_ref_ctor,
            32,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ade_88);
                // `PS2SoundManager_Create` allocates 0x320 bytes (heap path).
                heap_init(c);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, gp_at(0x68c), r.next_u32());
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------------- streaming

    #[test]
    fn stream_buffer_set_file_matches() {
        check(
            ssx3_ee::s08::f_002a9e78__generated,
            stream_buffer_set_file,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a9e78, 0);
                for i in 0..8u32 {
                    c.bus
                        .write8(OBJ2 + i, if i < 4 { b"data"[i as usize] } else { 0 });
                }
                w(c, OBJ + 0x7c, seed);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn stream_buffer_set_base_volume_matches() {
        leaf(
            ssx3_ee::s08::f_002a9f30__generated,
            stream_buffer_set_base_volume,
            23,
        );
    }

    #[test]
    fn sound_request_set_bank_sample_matches() {
        leaf(
            ssx3_ee::s08::f_002abce8__generated,
            sound_request_set_bank_sample,
            29,
        );
    }

    #[test]
    fn sound_manager_ref_dtor_matches() {
        check(
            ee::s08::f_002adee8__generated,
            sound_manager_ref_dtor,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002a_dee8, 2);
                c.g[A0][0] = OBJ_2 as u64;
                // Keep the reference's manager null and do not free the object
                // (a non-pool pointer would walk the test heap).
                c.g[A1][0] = 0;
                w_2(c, OBJ_2, 0);
                let vt = vtable(c, &mut r, &[0x18]);
                w_2(c, OBJ2_2, vt);
                let _ = seed;
            },
        );
    }

    // ----------------------------------------------------------------- streams

    #[test]
    fn stream_array_create_stream_matches() {
        check(
            ee::s08::f_002aa7f0__generated,
            stream_array_create_stream,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2aa7_f0, 4);
                let self_ = DATA;
                w_3(c, self_ + 0x8, DATA + 0x2000);
                w_3(c, self_, DATA + 0x2100);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn streambuffer_dtor_matches() {
        check(
            ee::s08::f_002a9df0__generated,
            streambuffer_dtor,
            SEEDS_4,
            &|c, seed| {
                base_4(c, seed, 0xa9df0, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed & 1) as u64;
                c.bus.write32(s, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(s + 4, 1);
                c.bus.write32(s + 8, 0);
                c.bus.write32(s + 0x74, 0);
            },
        );
    }

    // -- 0x002ac7f0 BankMonitorInstance_UpdateRange --------------------------

    /// `BankMonitorInstance_UpdateRange`: no range, playing inside and
    /// beyond, stopped inside and beyond (the fade-out and play callees on
    /// their short paths: already fading, or a state without a voice).
    #[test]
    fn bank_monitor_instance_update_range_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const THIS: u32 = DATA + 0x9_0000;
        check(
            ssx3_ee::s08::f_002ac7f0__generated,
            bank_monitor_instance_update_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_c7f0 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                let range = if seed % 5 == 0 {
                    0
                } else {
                    (10.0 + (r.next_u32() % 100) as f32).to_bits()
                };
                c.bus.write32(THIS + 0x8c, range);
                c.bus
                    .write32(THIS + 0x8, (seed / 5) % 2 * (1 + r.next_u32() % 4));
                c.bus.write32(THIS, [0, 5][(r.next_u32() % 2) as usize]);
                c.bus.write32(THIS + 0x54, r.next_u32() % 2);
                c.g[A0][0] = sx(THIS);
                c.f[12] = match r.next_u32() % 4 {
                    0 => r.edge_f32(),
                    1 => range,
                    _ => ((r.next_u32() % 200) as f32).to_bits(),
                };
                c.pc = 0x002a_c7f0;
            },
        );
    }

    /// StreamBuffer_GetStatus: an unknown stream (-8), a stream with no
    /// request (0x124 = 0) and one with a request but no rate (+0x14 = 0),
    /// the lock/unlock hooks fakes.
    #[test]
    fn stream_buffer_get_status_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x20_0000;
        const OUT: u32 = SELF + 0x100;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(
            ee::s08::f_002aa020__generated,
            stream_buffer_get_status,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a020, 2);
                c.pc = 0x002a_a020;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OUT);
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + SB_STREAM, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 3 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(REQ + 0xc, r.next_u32());
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// StreamArray_UpdateAll: counts -1..4, null and real buffers; each
    /// buffer runs the real StreamBuffer_UpdateVolume on its short paths
    /// (no volume source, or a volume set on an unknown SND stream; the
    /// lock hooks are fakes, the age stays below the stop limit).
    #[test]
    fn stream_array_update_all_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x20_0000;
        const ARRAY: u32 = SELF + 0x100;
        const BUFFERS: u32 = SELF + 0x1000;
        const VOLUME: u32 = SELF + 0x3000;
        check(
            ee::s08::f_002aac28__generated,
            stream_array_update_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ac28, 6);
                c.pc = 0x002a_ac28;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                c.bus.write8(0x0051_0000 - 0x56d2, 0); // no SND streams
                let n = (seed % 6) as i32 - 1;
                c.bus.write32(SELF + SA_COUNT, n as u32);
                c.bus.write32(SELF + SA_BUFFERS, ARRAY);
                c.bus.write32(VOLUME, r.f32(1.0));
                for i in 0..5u32 {
                    let b = BUFFERS + 0x100 * i;
                    c.bus
                        .write32(ARRAY + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { b });
                    c.bus.write32(b + 0x80, r.next_u32() % 121);
                    c.bus
                        .write32(b + 0x10, if r.next_u32() % 2 == 0 { 0 } else { VOLUME });
                    c.bus.write8(b + 0xc, r.next_u32() as u8);
                    c.bus.write32(b + 0x8, r.next_u32() % 4);
                }
            },
        );
    }

    /// StreamBuffer_UpdateVolume: starved (stop and clear), no scale
    /// pointer, or a volume set through the real SND lock / SNDSTRM path
    /// (fake lock hooks) with products below 0, inside and above 0..127.
    #[test]
    fn stream_buffer_update_volume_matches() {
        check(
            ee::s08::f_002a9f80__generated,
            stream_buffer_update_volume,
            240,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x2a9f80, 0);
                c.pc = 0x002a_9f80;
                let sb = DATA + 0x100;
                let scale = DATA + 0x400;
                c.g[A0][0] = sx(sb);
                c.bus.write32(sb + SB_HANDLE, r.next_u32() % 8);
                c.bus.write8(sb + SB_VOLUME, r.next_u32() as u8);
                // Audio_GetFrameRate is 60: the limit is 120.
                let starved = match seed % 4 {
                    0 => 121 + r.next_u32() % 100,
                    1 => 120,
                    _ => r.next_u32() % 120,
                };
                c.bus.write32(sb + SB_STARVED, starved);
                c.bus
                    .write32(sb + SB_SCALE_PTR, if seed % 5 == 1 { 0 } else { scale });
                let f = match seed % 3 {
                    0 => r.f32(4.0),
                    1 => r.edge_f32(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(scale, f);
            },
        );
    }

    /// StreamBuffer_GetRequestStatus: caller's status buffer or the frame's,
    /// an unknown SND stream (status zeroed: request 0) or a known one with
    /// a request id that is negative (-8 early out) or unknown; fake lock
    /// hooks, real StreamBuffer_GetStatus / SNDSTRM_GetRequestStatus.
    #[test]
    fn stream_buffer_get_request_status_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const OUT: u32 = SELF + 0x100;
        const SCRATCH: u32 = SELF + 0x200;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(
            ee::s08::f_002aa068__generated,
            stream_buffer_get_request_status,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a068, 4);
                c.pc = 0x002a_a068;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OUT);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(SCRATCH) };
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(
                    REQ + 0xc,
                    if seed % 4 < 2 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// StreamArray_GetState: empty and non-empty slots; the real
    /// StreamBuffer_GetState runs on a buffer whose stream ids are small.
    #[test]
    fn stream_array_get_state_matches() {
        check(
            ee::s08::f_002aae08__generated,
            stream_array_get_state,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aae_08, 0);
                c.pc = 0x002a_ae08;
                let arr = DATA + 0x100;
                let list = DATA + 0x200;
                let buffer = DATA + 0x1000;
                c.g[A0][0] = sx(arr);
                c.g[A1][0] = sx(seed % 16);
                c.bus.write32(arr + OC264_SA_BUFFERS, list);
                let entry = if seed % 3 == 0 { 0 } else { buffer };
                c.bus.write32(list + 4 * (seed % 16), entry);
                for i in 0..8u32 {
                    c.bus.write32(buffer + 4 * i, r.next_u32() % 4);
                }
            },
        );
    }

    /// `StreamBuffer_IsPlaying`: the fixture of
    /// `stream_buffer_get_request_status_matches` (fake lock hooks, one SND
    /// stream slot, known/unknown streams and requests), with the status
    /// word in the frame preset to 2 (playing) or junk where the real
    /// callees leave it alone.
    #[test]
    fn stream_buffer_is_playing_matches() {
        use crate::difftest::{check, SP};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(
            ee::s08::f_002aa1b8__generated,
            stream_buffer_is_playing,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a1b8, 4);
                c.pc = 0x002a_a1b8;
                c.g[A0][0] = sx(SELF);
                c.bus
                    .write32(SP - 0x20, if seed % 2 == 0 { 2 } else { r.next_u32() % 4 });
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(
                    REQ + 0xc,
                    if seed % 4 < 2 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// `BankMonitorInstance_Reset`: the `FadeOut` fixture (already fading,
    /// or kind 1 / 2 / other; null SND lock callbacks are fakes, some
    /// unwind; a handle that finds no channel), with the real `FadeOut`.
    #[test]
    fn bank_monitor_instance_reset_matches() {
        covered(
            ee::s08::f_002abb38__generated,
            bank_monitor_instance_reset,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_bb38, 4);
                c.pc = 0x002a_bb38;
                let inst = DATA;
                c.g[A0][0] = inst as u64;
                c.bus.write32(inst + 0x8, r.next_u32()); // playing
                c.bus.write32(
                    inst + 0x54,
                    if seed % 5 == 0 {
                        1 + r.next_u32() % 3
                    } else {
                        0
                    },
                );
                let kind = match seed % 4 {
                    0 | 1 => 1,
                    2 => 2,
                    _ => r.next_u32() % 6,
                };
                c.bus.write32(inst, kind);
                let handle = if r.next_u32() % 2 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(inst + 0x4, handle);
            },
        );
    }

    /// BankMonitorInstance_FadeOut: already fading, or kind 1 / 2 / other.
    /// SND_Lock/SND_Unlock's lock callbacks are null (fakes, some unwind);
    /// the handle is negative or out of range, so the fade calls find no
    /// channel and return at once.
    #[test]
    fn bank_monitor_instance_fade_out_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s08::f_002ac180__generated,
            bank_monitor_instance_fade_out,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ac180, 4);
                c.pc = 0x002a_c180;
                let inst = DATA;
                c.g[A0][0] = inst as u64;
                c.bus.write32(
                    inst + BMI_FADING,
                    if seed % 5 == 0 {
                        1 + r.next_u32() % 3
                    } else {
                        0
                    },
                );
                let kind = match seed % 4 {
                    0 | 1 => 1,
                    2 => 2,
                    _ => r.next_u32() % 6,
                };
                c.bus.write32(inst + BMI_KIND, kind);
                c.bus.write32(inst + BMI_PLAYING, r.next_u32());
                let handle = if r.next_u32() % 2 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(inst + BMI_HANDLE, handle);
            },
        );
    }

    /// StreamArray_GetTimeRemaining: negative index, empty slot, and a
    /// present buffer on which the real StreamBuffer_GetTimeRemaining runs
    /// (fixture as in `stream_buffer_get_request_status_matches`: one stream
    /// slot, SND lock/unlock and the request lookup on fakes).
    #[test]
    fn stream_array_get_time_remaining_matches() {
        const ARR: u32 = DATA + 0x20_0000;
        const LIST: u32 = ARR + 0x100;
        const BUF: u32 = ARR + 0x400;
        const STREAM: u32 = ARR + 0x1000;
        const REQ: u32 = ARR + 0x2000;
        check(
            ee::s08::f_002aae78__generated,
            stream_array_get_time_remaining,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ae78, 4);
                c.pc = 0x002a_ae78;
                let i = seed % 8;
                c.g[A0][0] = sx(ARR);
                c.g[A1][0] = if seed % 5 == 0 {
                    sx(0x8000_0000 | r.next_u32())
                } else {
                    i as u64
                };
                c.bus.write32(ARR + SAT_BUFFERS, LIST);
                c.bus
                    .write32(LIST + 4 * i, if seed % 5 == 1 { 0 } else { BUF });
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(BUF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 7 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(REQ + 0xc, r.next_u32());
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// StreamBuffer_GetTimeRemaining: the real StreamBuffer_GetRequestStatus
    /// chain (fake SND lock/unlock hooks, some unwinding) on one SND stream
    /// whose idle status hands request id 0 on: the request playing (head
    /// of the list: state 2, remaining time from its byte counters and the
    /// sample rate), queued behind another (state 1), inactive (state 0) or
    /// not found (state 3).
    #[test]
    fn stream_buffer_get_time_remaining_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const HEAD: u32 = SELF + 0x2000;
        const REQ: u32 = SELF + 0x2100;
        check(
            ssx3_ee::s08::f_002aa108__generated,
            stream_buffer_get_time_remaining,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a108, 4);
                c.pc = 0x002a_a108;
                c.g[A0][0] = sx(SELF);
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot (SND stream 0), idle: status 0, request id 0.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                c.bus.write32(SELF + 0x8, 0);
                c.bus.write32(STREAM + 0x124, 0);
                c.bus
                    .write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
                // Request list: REQ (id 0) first, behind HEAD (id 1), or absent.
                let case = seed % 4;
                let first = if case == 1 { HEAD } else { REQ };
                c.bus.write32(STREAM + 0x11c, first);
                c.bus.write32(HEAD, REQ);
                c.bus.write32(HEAD + 0xc, 1);
                c.bus.write32(REQ, 0);
                c.bus.write32(REQ + 0xc, if case == 3 { 7 } else { 0 });
                c.bus.write8(
                    REQ + 0x24,
                    if case == 2 {
                        0
                    } else {
                        1 + r.next_u32() as u8 % 4
                    },
                );
                let done = r.next_u32() & 0xf_ffff;
                c.bus.write32(REQ + 0x14, done);
                c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
                c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
            },
        );
    }

    /// StreamArray_IsPlaying: an empty slot (false) or a buffer whose real
    /// StreamBuffer_IsPlaying -> GetRequestStatus chain runs on one SND
    /// stream slot (unknown or known stream, request state random), with
    /// fake lock hooks that unwind on some seeds.
    #[test]
    fn stream_array_is_playing_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const ARRAY: u32 = DATA + 0x20_0000;
        const LIST: u32 = ARRAY + 0x40;
        const BUFFER: u32 = ARRAY + 0x400;
        const STREAM: u32 = ARRAY + 0x1000;
        const REQ: u32 = ARRAY + 0x2000;
        check(
            ee::s08::f_002ab028__generated,
            stream_array_is_playing,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b028, 4);
                c.pc = 0x002a_b028;
                let i = seed % 8;
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = i as u64;
                c.bus.write32(ARRAY + STREAM_ARRAY_BUFFERS, LIST);
                c.bus
                    .write32(LIST + 4 * i, if seed % 4 == 0 { 0 } else { BUFFER });
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(BUFFER + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(
                    REQ + 0xc,
                    if seed % 6 < 3 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// BankMonitorInstance_UpdateDistance: the virtual is a fake that
    /// returns a random f0 (sqrt of its bits) and sometimes unwinds.
    #[test]
    fn bank_monitor_instance_update_distance_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s08::f_002abd48__generated,
            bank_monitor_instance_update_distance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2abd48, 1);
                let this = DATA;
                let listener = DATA + 0x1000;
                let vt = DATA + 0x3000;
                fill(c, &mut r, this, 0x20);
                c.bus.write32(listener + LISTENER_VTABLE, vt);
                c.bus
                    .write16(vt + 0x30, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x34, FAKE);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(listener);
                c.pc = 0x002a_bd48;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BankMonitorInstance_Play.

    /// A monitor instance at DATA and its play stack at DATA+0x1000. Kinds
    /// 1 (bank patch) and 2 (MIDI) push a depth and play under the sound
    /// lock (`SND_Lock`/`SND_Unlock` call through pointers we aim at
    /// fakes, which log and sometimes unwind); 0, 3, -1 and 7 do nothing.
    /// Every other seed of the playing kinds aliases the stack's pointers
    /// so the clamp reads a chosen word (above 127, negative, in range):
    /// `f3c` points at the volume pointer, the instance's +0x60 word (stored
    /// there) points the zeroing store at the depth, and the clamp then
    /// reads a word of the current descriptor.
    pub(crate) fn bank_monitor_play_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{fill, DATA};
        let mut r = machine(c, seed, 0x2a_c220, 2);
        c.pc = 0x002a_c220;
        let inst = DATA;
        let st = DATA + 0x1000;
        let patch = DATA + 0x2000;
        c.g[A0][0] = sx(inst);
        c.g[A1][0] = sx(st);
        // SND_Lock / SND_Unlock interrupt hooks.
        c.bus.write32(0x0044_c46c, FAKE);
        c.bus.write32(0x0044_c470, FAKE + 0x10);

        let kinds = [1u32, 2, 1, 2, 0, 3, 0xffff_ffff, 7];
        let kind = kinds[(seed % 8) as usize];
        let alias = (seed / 8) % 2 == 1;
        fill(c, &mut r, inst, 0x20);
        c.bus.write32(inst + BANK_MON_KIND, kind);
        let bank = if r.next_u32() % 4 == 0 {
            0xffff_fff0
        } else {
            r.next_u32() % 8
        };
        c.bus.write32(inst + BANK_MON_BANK, bank);
        c.bus.write32(inst + BANK_MON_PATCH, patch);
        fill(c, &mut r, patch, 8);
        if r.next_u32() % 2 == 0 {
            c.bus.write16(patch, 0x444d);
        }

        fill(c, &mut r, st, 0x22);
        let depth = (r.next_u32() % 3).wrapping_sub(1); // -1..1, pushed to 0..2
        c.bus.write32(st + BANK_PLAY_DEPTH, depth);
        c.bus.write32(st + BANK_PLAY_DESCS, DATA + 0x3000);
        fill(c, &mut r, DATA + 0x3000, 0x20);
        c.bus.write32(st + BANK_PLAY_F34, DATA + 0x4000);
        c.bus.write32(st + BANK_PLAY_VOLUME, DATA + 0x4100);
        c.bus.write32(st + BANK_PLAY_F3C, DATA + 0x4200);
        for (i, off) in [0x60u32, 0x64, 0x68, 0x6c, 0x74, 0x78, 0x7c, 0x80, 0x84]
            .into_iter()
            .enumerate()
        {
            c.bus.write32(st + off, DATA + 0x4300 + 0x100 * i as u32);
        }
        c.bus.write32(st + BANK_PLAY_ENV, DATA + 0x5000);
        if alias && kind <= 2 {
            let d = 1 + r.next_u32() % 2; // pushed depth 1 or 2
            c.bus.write32(st + BANK_PLAY_DEPTH, d - 1);
            c.bus
                .write32(st + BANK_PLAY_F3C, st + BANK_PLAY_VOLUME - 4 * d);
            c.bus
                .write32(inst + BANK_MON_F60, st + BANK_PLAY_DEPTH - 4 * d);
            let words = [
                200u32,
                0xffff_fffb,
                50,
                127,
                126,
                0x8000_0000,
                0x7f,
                0x1234_5605,
            ];
            let w = words[(seed / 16 % 8) as usize];
            c.bus.write32(st + BANK_PLAY_DEPTH - 4 * d, w);
        }
    }

    /// Paths (kind 1 / 2 / none) and the clamp branches, with the sound
    /// lock fakes unwinding at either call.
    #[test]
    fn bank_monitor_instance_play_matches() {
        crate::difftest::fakes::covered(
            ee::s08::f_002ac220__generated,
            bank_monitor_instance_play,
            256,
            &bank_monitor_play_setup,
        );
        crate::difftest::check_lohi(
            ee::s08::f_002ac220__generated,
            bank_monitor_instance_play,
            128,
            &bank_monitor_play_setup,
        );
    }

    /// BankMonitorInstance_FadeVolume: kinds 0..4 (sound, MIDI, stream
    /// buffer, none), already stopping or not, volume -1 / out of range /
    /// in range, `touch` on or off, seconds 0, -0 or not; sound and MIDI
    /// handles that the real drivers reject (short paths), the lock hooks
    /// are fakes (unwinding for some seeds).
    #[test]
    fn bank_monitor_instance_fade_volume_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::DATA;
        const INST: u32 = DATA;
        const BUFFER: u32 = DATA + 0x200;
        covered(
            ee::s08::f_002acb80__generated,
            bank_monitor_instance_fade_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_cb80, 4);
                c.pc = 0x002a_cb80;
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write32(INST, seed % 5);
                c.bus.write32(INST + FADE_VOLUME_HANDLE, r.next_u32());
                c.bus.write32(INST + FADE_VOLUME_BUFFER, BUFFER);
                c.bus.write32(BUFFER + 8, r.next_u32() | 0x8000_0000);
                c.bus.write32(BUFFER + 0xc, r.next_u32());
                c.bus.write32(
                    INST + FADE_VOLUME_STOPPING,
                    if r.next_u32() % 4 == 0 { 1 } else { 0 },
                );
                c.bus.write32(INST + FADE_VOLUME_TOUCHED, r.next_u32());
                c.bus.write32(INST + FADE_VOLUME_TOUCHED_2, r.next_u32());
                let volume: u64 = match (seed / 5) % 6 {
                    0 => u64::MAX,
                    1 => 128 + (r.next_u32() % 1000) as u64,
                    2 => (-((1 + r.next_u32() % 1000) as i64)) as u64,
                    3 => 127,
                    _ => (r.next_u32() % 128) as u64,
                };
                c.g[A1][0] = volume;
                c.g[A2][0] = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.f[12] = match r.next_u32() % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.edge_f32(),
                    _ => r.f32(5.0),
                };
                c.g[A0][0] = sx(INST);
            },
        );
    }

    /// `StreamArray_GetCurrentRequest`: an empty slot (`-1`) or a real stream
    /// buffer whose status is read.
    #[test]
    fn stream_array_get_current_request_matches() {
        check(
            ee::s08::f_002ab150__generated,
            stream_array_get_current_request,
            200,
            &|c, seed| {
                let mut r = Rng(0x2ab1_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let i = seed % 4;
                c.g[A1][0] = i as u64;
                let buffers = DATA + 0x100;
                c.bus.write32(DATA + 8, buffers);
                for k in 0..4u32 {
                    let empty = (seed + k) % 3 == 0;
                    c.bus.write32(
                        buffers + 4 * k,
                        if empty { 0 } else { DATA + 0x200 + 0x40 * k },
                    );
                    c.bus.write32(DATA + 0x208 + 0x40 * k, k);
                }
                // The sound lock routines the status path calls, and an empty
                // stream table (the status call returns at once).
                c.bus.write32(STREAM_BUFFER_SET_PAN_LOCK_SLOT, 0x0022_6610);
                c.bus
                    .write32(STREAM_BUFFER_SET_PAN_UNLOCK_SLOT, 0x0022_6610);
                c.bus.write8(SND_GETSTREAM_COUNT, 0);
            },
        );
    }

    /// `StreamBuffer_SetPan`: the sound lock is taken, the buffer's stream
    /// gets the pan and the lock is released.
    #[test]
    fn stream_buffer_set_pan_matches() {
        check(
            ee::s08::f_002a9ed8__generated,
            stream_buffer_set_pan,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a9e_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let handle = seed % 4;
                c.bus.write32(DATA + STREAM_BUFFER_SET_PAN_HANDLE, handle);
                // The lock routines SND_Lock / SND_Unlock call, and the stream
                // table the pan call looks the handle up in.
                c.bus.write32(STREAM_BUFFER_SET_PAN_LOCK_SLOT, 0x0022_6610);
                c.bus
                    .write32(STREAM_BUFFER_SET_PAN_UNLOCK_SLOT, 0x0022_6610);
                c.bus.write8(SND_GETSTREAM_COUNT, ((seed % 4) + 1) as u8);
                for h in 0..4u32 {
                    c.bus
                        .write32(SND_GETSTREAM_TABLE + 4 * h, DATA + 0x2000 + 0x100 * h);
                    c.bus.write32(DATA + 0x2004 + 0x100 * h, h);
                }
            },
        );
    }

    /// A StreamBuffer with a known, unknown or empty SND stream, with and
    /// without a request; the SND lock / unlock hooks are fakes.
    #[test]
    fn stream_buffer_get_current_request_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x22_0000;
        const REQ: u32 = DATA + 0x24_0000;
        check(
            ee::s08::f_002aa210__generated,
            stream_buffer_get_current_request,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a210, 2);
                c.pc = 0x002a_a210;
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 3 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(REQ + 0xc, r.next_u32());
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
                c.g[A0][0] = sx(SELF);
            },
        );
    }
}
