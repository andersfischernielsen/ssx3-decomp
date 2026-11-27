//! `src/sound/streamsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
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
pub(crate) use crate::game::sound::bankmonitor::SPEECH_STREAM_HANDLE;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_1ec0, fe_module_load, "FEModule_Load"),
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
    (
        0x002a_9f38,
        stream_buffer_set_pitch,
        "StreamBuffer_SetPitch",
    ),
    (0x002a_a320, stream_buffer_queue, "StreamBuffer_Queue"),
    (0x002a_ad78, stream_array_queue, "StreamArray_Queue"),
    (
        0x002a_bcf8,
        sound_request_set_stream,
        "SoundRequest_SetStream",
    ),
    (
        0x002a_b1a0,
        speech_system_clear_prebuffer_callback,
        "SpeechSystem_ClearPrebufferCallback",
    ),
    (0x002a_99d8, stream_buffer_ctor, "StreamBuffer_ctor"),
    (0x002a_a408, stream_buffer_stop, "StreamBuffer_Stop"),
    (0x002a_a490, stream_array_ctor, "StreamArray_ctor"),
    (
        0x002a_a910,
        stream_array_create_stream_default_params,
        "StreamArray_CreateStreamDefaultParams",
    ),
    (
        0x002a_b200,
        ssx_audio_system_base_ctor,
        "SSXAudioSystem_BaseCtor",
    ),
    (0x002a_9c00, stream_buffer_ctor_2, "StreamBuffer_ctor_2"),
    (
        0x002a_a0c0,
        stream_buffer_get_request_position,
        "StreamBuffer_GetRequestPosition",
    ),
    (
        0x002a_a150,
        stream_buffer_has_buffered,
        "StreamBuffer_HasBuffered",
    ),
    (
        0x002a_a188,
        stream_buffer_is_request_pending,
        "StreamBuffer_IsRequestPending",
    ),
    (
        0x002a_a1e8,
        stream_buffer_clear_prebuffer,
        "StreamBuffer_ClearPrebuffer",
    ),
    (
        0x002a_a240,
        stream_buffer_clear_prebuffer_handle,
        "StreamBuffer_ClearPrebufferHandle",
    ),
    (
        0x002a_a260,
        stream_buffer_get_request_length,
        "StreamBuffer_GetRequestLength",
    ),
    (0x002a_a2b0, stream_buffer_pause, "StreamBuffer_Pause"),
    (0x002a_a2e8, stream_buffer_resume, "StreamBuffer_Resume"),
    (
        0x002a_a398,
        stream_buffer_queue_from_big,
        "StreamBuffer_QueueFromBig",
    ),
    (
        0x002a_a448,
        stream_buffer_set_stream_param,
        "StreamBuffer_SetStreamParam",
    ),
    (
        0x002a_a870,
        stream_array_create_stream_ex,
        "StreamArray_CreateStreamEx",
    ),
    (0x002a_ac98, stream_array_pause_all, "StreamArray_PauseAll"),
    (
        0x002a_ad08,
        stream_array_resume_all,
        "StreamArray_ResumeAll",
    ),
    (
        0x002a_adc0,
        stream_array_queue_from_big,
        "StreamArray_QueueFromBig",
    ),
    (
        0x002a_aec0,
        stream_array_get_request_length,
        "StreamArray_GetRequestLength",
    ),
    (
        0x002a_af08,
        stream_array_has_buffered,
        "StreamArray_HasBuffered",
    ),
    (
        0x002a_af40,
        stream_array_is_request_pending,
        "StreamArray_IsRequestPending",
    ),
    (
        0x002a_af78,
        stream_array_is_request_done,
        "StreamArray_IsRequestDone",
    ),
    (
        0x002a_afd0,
        stream_array_is_request_playing,
        "StreamArray_IsRequestPlaying",
    ),
    (
        0x002a_b060,
        stream_array_clear_prebuffer,
        "StreamArray_ClearPrebuffer",
    ),
    (
        0x002a_b098,
        stream_array_clear_prebuffer_handle,
        "StreamArray_ClearPrebufferHandle",
    ),
    (
        0x002a_b0f0,
        stream_array_set_stream_volume,
        "StreamArray_SetStreamVolume",
    ),
    (
        0x002a_b1c8,
        stream_array_set_stream_param,
        "StreamArray_SetStreamParam",
    ),
    (
        0x002a_bd38,
        sound_request_set_bank_sample_2,
        "SoundRequest_SetBankSample_2",
    ),
    (
        0x002a_cac8,
        bank_monitor_instance_pause,
        "BankMonitorInstance_Pause",
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

    pub(crate) use crate::game::lib::snd::callee::STRCPY;
    pub(crate) use crate::game::lib::snd::callee::STREAM_BUFFER_QUEUE;
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

// ---------------------------------------------------------------------------
// StreamBuffer_Queue (0x2aa320)

/// The stream's big-file handle (`self+0x74`).
pub const STREAM_BUFFER_BIG: u32 = 0x74;

/// The stream handle (`self+8`).
pub const STREAM_BUFFER_HANDLE: u32 = 0x8;

/// The stream's inline request buffer (`self+0x14`).
pub const STREAM_BUFFER_INLINE: u32 = 0x14;

/// `BigFile_FindEntry` (0x3e2ce0).
pub const BIG_FILE_FIND_ENTRY: u32 = 0x003e_2ce0;

/// `SNDSTRM_Queue` (0x3b78d8).
pub const OC331_SNDSTRM_QUEUE: u32 = 0x003b_78d8;

/// `int StreamBuffer_Queue(void* self, int handle, void* src, int off)`
/// (0x2aa320): queue a stream request. When the stream has a big-file
/// handle, look the entry up first (its size at `sp`) and add it to the
/// offset; then `SNDSTRM_Queue(stream, handle, buffer, off)`, where the
/// buffer is `src` for the big-file case and the caller's `src` otherwise.
pub fn stream_buffer_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(STREAM_BUFFER_BIG)).lw(c);
        let has_big = c.g[A0][0] != 0;
        c.g[S2][0] = c.g[A3][0];
        if has_big {
            c.cycles += 10;
            c.g[A1][0] = c.g[A2][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, BIG_FILE_FIND_ENTRY, 0x002a_a364)?;
            c.g[A3][0] = Addr(c.g[SP][0] as u32).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(STREAM_BUFFER_HANDLE)).lw(c);
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(STREAM_BUFFER_INLINE));
            c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        } else {
            c.cycles += 2;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(STREAM_BUFFER_HANDLE)).lw(c);
        }
        c.cycles += 8;
        call_ptr(c, OC331_SNDSTRM_QUEUE, 0x002a_a380)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int StreamArray_Queue(void* self, int i, int a, int b, int c)` (0x2aad78):
/// queue the `i`-th entry of the array at `self+8`; -1 when it is null.
pub fn stream_array_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A3][0];
        c.g[V0][0] = Addr(r32(c, A0)).off(0x8).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let entry = c.g[A0][0];
        c.g[A3][0] = c.g[T0][0];
        if entry != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            c.g[A2][0] = c.g[V1][0];
            callee::STREAM_BUFFER_QUEUE.call(c, 0x002a_adb0)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int StreamBuffer_SetPitch(void* self, int pitch)` (0x2a9f38): set the
/// stream's pitch under the sound lock.
pub fn stream_buffer_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x003b_58a0, 0x002a_9f54)?;
        c.g[A0][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x003b_7fb8, 0x002a_9f60)?;
        call_ptr(c, 0x003b_58d8, 0x002a_9f68)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SoundRequest_SetStream(SoundRequest* self, int a1, char* name,
/// int a3)` (0x2abcf8): store the id, copy the stream name to `+0x10` and the
/// last argument to `+0x50`.
pub fn sound_request_set_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        Addr(r32(c, S0)).off(0xc).sw(c, c.g[A1][0]);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
        c.g[A1][0] = c.g[A2][0];
        callee::STRCPY.call(c, 0x002a_bd20)?;
        Addr(r32(c, S0)).off(0x50).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SpeechSystem_ClearPrebufferCallback(void* unused, SpeechStream* s)`
/// (0x2ab1a0): set the stream's prebuffer to 0
/// (`SNDSTRM_SetPrebuffer(s->handle, 0)`) and return 1.
pub fn speech_system_clear_prebuffer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(SPEECH_STREAM_HANDLE));
        c.g[A1][0] = 0;
        call_ptr(c, 0x003b_7d80, 0x002a_b1b4)?; // SNDSTRM_SetPrebuffer
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_CreateStreamDefaultParams(StreamSys** self, a1, a2, a3, t0)`
/// (0x2aa910): pushes a fresh bank-monitor instance (`mInst` at +0x1f4 of the
/// sys object, parameter arrays at the `+0x1d8` block), copies the default
/// descriptor from `sys+0x1dc` (24 bytes), clears its per-instance fields,
/// sets the delay to 100/90/50, the volume map to 127 and close-on-end to 1,
/// stores `a2` as the volume-control pointer, calls `StreamArray_CreateStream`
/// (0x2aa7f0) with `(self, a1, a3, t0)` and pops the instance again.
pub fn stream_array_create_stream_default_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 162;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[T1][0] = 0x18;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[T0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S4][0] = c.g[A3][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S6][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A1][0] = 0u64;
        c.g[S0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = 0x8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 3, 3, 9);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 9);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V1][0] = c.lb(c.g[A0][0] as u32);
        c.sw(c.g[A3][0] as u32, c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = sx(0x002aaa1c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x002aaa1c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T1][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[T2][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T3][0] = 0x7f;
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[T4][0] = 0x1;
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A3][0] = c.g[S3][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[T0][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T2][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[T3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[T4][0]);
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[RA][0] = sx(0x002aab60);
        c.sw(c.g[V0][0] as u32, c.g[S6][0]);
        call_ptr(c, 0x002aa7f0, 0x002aab60)?;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `SSXAudioSystem_BaseCtor(this, embedded, maxBanks, maxStreams, heap)`
/// (0x2ab200, frame 0x100): with `embedded` set, builds the sound system, bank
/// manager and stream system inside `this` (vtable/sub-object pointers at
/// +0x0, +0x118, +0x5560, +0x5908, +0x6200) and runs their ctors; always runs
/// the crowd, music and speech system ctors, rewrites the sub-objects'
/// vtables (0x4837d8, 0x483760, 0x4837a0), when not embedded copies them to
/// the stack rebased to `this`, and installs the crowd/music/speech vtables.
/// Returns `this`.
pub fn ssx_audio_system_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ab28c: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S1][0] = c.g[T0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S3][0] = c.g[A3][0];
            c.g[S4][0] = c.g[A1][0];
            let t = c.g[S4][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            if t {
                break 'b_2ab28c;
            } else {
                c.cycles += 22;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5730));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5728));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6200));
                c.g[A0][0] = c.g[A2][0];
                c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6200), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5908), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x118), c.g[V0][0]);
                c.g[RA][0] = sx(0x002ab260);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5560), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x002ab260)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6200));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x002ab274);
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0028b9b8, 0x002ab274)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5560));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x002ab28c);
                c.g[T0][0] = c.g[S1][0];
                call_ptr(c, 0x002aa490, 0x002ab28c)?;
                break 'b_2ab28c;
            }
        }
        'b_2ab42c: {
            c.cycles += 35;
            c.g[A2][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002ab2a4);
            c.g[A3][0] = c.g[S3][0];
            call_ptr(c, 0x002a69a0, 0x002ab2a4)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002ab2bc);
            c.g[A3][0] = c.g[S3][0];
            call_ptr(c, 0x002b28c0, 0x002ab2bc)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
            c.g[RA][0] = sx(0x002ab2d4);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002b07f8, 0x002ab2d4)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x37d8));
            c.g[A3][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[T7][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3760));
            c.g[S2][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x37a0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[T7][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[S4][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[S1][0]);
            if t {
                break 'b_2ab42c;
            } else {
                c.cycles += 69;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x37d8));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3760));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T4][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T2][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[T1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                c.g[A3][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A2][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5728));
                c.g[A1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T5][0]);
                c.g[T4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lhu((c.g[T7][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw(c.g[T4][0] as u32);
                c.g[V0][0] = c.lhu((c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5730));
                c.g[T3][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x37a0));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[A3][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T0][0]);
                c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw(c.g[T4][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[T6][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_2ab42c;
            }
        }
        c.cycles += 19;
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38f8));
        c.g[A0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5558), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3870));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x37f8));
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x571c), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5720), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t;
        return Ok(());
    });
}

/// `FEModule_Load(FEModule* self)` (0x1a1ec0, frame 0x70): the front-end load
/// sequence: load timer progress 3.5/20/50/75/99, locale and texture init,
/// the UI file, character animations, bolt file manager, rider manager, the
/// front-end camera pose (0,0,-195)/(0,1,0), then the input, custom and
/// sound objects of the UI (alloc via 0x317d70), the irradiance database
/// and `cSSXAudio_FrontEndLoad`.
pub fn fe_module_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 154;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = c.g[A0][0];
        c.g[AT][0] = 0x40600000;
        c.mtc1(1, 12);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[RA][0] = sx(0x001a1ee8);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317530, 0x001a1ee8)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a1efc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a1efc)?;
        c.g[RA][0] = sx(0x001a1f04);
        call_ptr(c, 0x0014dc80, 0x001a1f04)?;
        c.g[A1][0] = 0x9;
        c.g[RA][0] = sx(0x001a1f10);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014dd58, 0x001a1f10)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a1f1c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014bd78, 0x001a1f1c)?;
        c.g[RA][0] = sx(0x001a1f24);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014af10, 0x001a1f24)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[RA][0] = sx(0x001a1f30);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
        call_ptr(c, 0x00195be0, 0x001a1f30)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x42480000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a1f44);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a1f44)?;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001a1f60);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001a1f60)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = 0x460000;
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1278));
        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[T0][0] as u32);
        c.g[A2][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[A3][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x9));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[SP][0] as u32);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x9), c.g[A3][0]);
        c.g[V1][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0xa));
        c.sb((c.g[SP][0] as u32).wrapping_add(0xa), c.g[V1][0]);
        c.g[RA][0] = sx(0x001a1f9c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1278));
        call_ptr(c, 0x00397c40, 0x001a1f9c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x42960000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a1fb0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a1fb0)?;
        c.g[RA][0] = sx(0x001a1fb8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a2b00, 0x001a1fb8)?;
        c.g[RA][0] = sx(0x001a1fc0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019ce68, 0x001a1fc0)?;
        c.g[RA][0] = sx(0x001a1fc8);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a0498, 0x001a1fc8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[AT][0] = 0xffffffffc3430000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), 0u64);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001a2034);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001a2034)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x42c60000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a2048);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a2048)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1288));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001a2060);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x001a2060)?;
        c.g[V1][0] = 0x470000;
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e30));
        c.sb((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x460000;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
        c.g[A2][0] = 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1298));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0u64);
        c.g[A0][0] = 0x14;
        c.g[RA][0] = sx(0x001a20a0);
        c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x001a20a0)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[RA][0] = sx(0x001a20ac);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001a0720, 0x001a20ac)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0x460000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12a8));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[RA][0] = sx(0x001a20cc);
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x00317d70, 0x001a20cc)?;
        c.g[RA][0] = sx(0x001a20d4);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001a2e20, 0x001a20d4)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = 0x500000;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x88));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[RA][0] = sx(0x001a2104);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        call_ptr(c, 0x0038acd0, 0x001a2104)?;
        c.g[RA][0] = sx(0x001a210c);
        call_ptr(c, 0x0028b180, 0x001a210c)?;
        c.g[RA][0] = sx(0x001a2114);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00285fb0, 0x001a2114)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `cStreamInstance* StreamBuffer_ctor(cStreamInstance* self, SndBankMonitor* p,
/// int heap, int extra)` (0x2a99d8, frame 0x50): resets the instance fields
/// (+0x74/+0x78/+0x80 = 0, handle -1, heap type +0x7c), picks up the volume
/// control pointer of the current monitor instance, and, when set, rescales
/// the instance's base volume by it (clamped to 0..127, `cvt.w.s`) and mirrors
/// the clamped value into the current-volume array. Then allocates the stream
/// buffer (`SNDSTRM_overhead(8, 300) + extra` bytes through
/// `operator_new_named`, 0x317e30), creates the stream (`SNDSTRM_create`) and
/// stores the handle at +8. Returns `self`.
pub fn stream_buffer_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a9b84: {
            c.cycles += 29;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0xffffffffffffffff;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A3][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.sw((c.g[S2][0] as u32).wrapping_add(0x74), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x78), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[S3][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x80), 0u64);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[A1][0] == 0u64;
            c.sb((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            if t {
                break 'b_2a9b84;
            } else {
                'b_2a9b0c: {
                    'b_2a9b08: {
                        'b_2a9ab8: {
                            c.cycles += 16;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.lwc1(2, c.g[A1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[AT][0] = 0x42fe0000;
                            c.mtc1(1, 3);
                            c.lwc1(0, c.g[V0][0] as u32);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            if t {
                                break 'b_2a9ab8;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                c.mtc1(0, 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.lwc1(0, c.g[V0][0] as u32);
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 0, 2);
                                c.c_le_s(1, 0);
                                let t = !c.fcc();
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_2a9b08;
                                } else {
                                    break 'b_2a9ab8;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();
                        c.g[V0][0] = 0x7f;
                        if t {
                            break 'b_2a9b08;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.lwc1(0, c.g[V0][0] as u32);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            let t = true;
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_2a9b0c;
                            } else {
                                break 'b_2a9b08;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    break 'b_2a9b0c;
                }
                'b_2a9b80: {
                    'b_2a9b54: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x18;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.mult(0, 4, 4, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if t {
                            break 'b_2a9b54;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_2a9b80;
                            } else {
                                break 'b_2a9b54;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        break 'b_2a9b80;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        break 'b_2a9b80;
                    }
                }
                c.cycles += 1;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                break 'b_2a9b84;
            }
        }
        c.cycles += 31;
        c.g[A1][0] = 0x12c;
        c.g[RA][0] = sx(0x002a9b90);
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x003b7b00, 0x002a9b90)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3770));
        c.g[RA][0] = sx(0x002a9bac);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9bac)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V1][0] = 0x18;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[A1][0] = 0x8;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A2][0] = 0x12c;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 4, 4, 3);
        c.g[RA][0] = sx(0x002a9bdc);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x003b7818, 0x002a9bdc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------------------
// Irradiance_Load (0x0038acd0): already hand-written in `game/render/irradiance.rs`
// (deliberately not installed there: FILE_load waits on the IOP queue). Skipped.
// PATH_StopTracks (0x003d2a68): a `loop { match pc` block dispatcher (resumable
// at its back-edge heads) with no `__entries` twin. Skipped for the orchestrator's
// `port_dispatcher.py`.

/// `cStreamArray* StreamArray_ctor(cStreamArray* this, int inchrg, int banks,
/// int count, heap)` (0x2aa490, frame 0xc0): with `inchrg` set, builds the
/// sound manager reference and bank manager inside `this` (sub-object
/// pointers at +0x0 and +0x1ec) and runs their ctors; rewrites the sound
/// system and bank manager vtables (0x483998, 0x4839b8); when not in charge,
/// copies both vtables to the stack rebased to `this`; stores `count` at +4,
/// allocates the `count`-entry stream pointer array at +8 (named
/// `StreamArray`) and clears it. Returns `this`.
pub fn stream_array_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2aa4f4: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = c.g[T0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            let t = c.g[S3][0] == 0u64;
            c.g[S4][0] = c.g[A3][0];
            if t {
                break 'b_2aa4f4;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S1][0];
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x002aa4e0);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1ec), c.g[V1][0]);
                call_ptr(c, 0x002ade88, 0x002aa4e0)?;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002aa4f4);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0028b9b8, 0x002aa4f4)?;
                break 'b_2aa4f4;
            }
        }
        'b_2aa5cc: {
            c.cycles += 14;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3998));
            c.g[A0][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[A3][0] = 0x480000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x39b8));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2aa5cc;
            } else {
                c.cycles += 40;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3998));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x39b8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_2aa5cc;
            }
        }
        'b_2aa620: {
            c.cycles += 11;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = sx((c.g[S4][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3218));
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x002aa5e8);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x002aa5e8)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0u64;
            let t = (c.g[V1][0] as i64) <= 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            if t {
                break 'b_2aa620;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                'l_2aa600: loop {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x002aa600);
                        }
                        continue 'l_2aa600;
                    } else {
                        break 'b_2aa620;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// Offset of the sound stream handle in a `StreamBuffer`.
pub(crate) const STREAM_BUFFER_STOP_HANDLE: u32 = 0x8;

/// `void StreamBuffer_Stop(StreamBuffer* sb)` (0x2aa408,
/// `cStreamInstance_Purge`): `SNDSTRM_Stop(sb->handle)` (handle at +0x8).
pub fn stream_buffer_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(STREAM_BUFFER_STOP_HANDLE));
        call_ptr(c, 0x003b_78f8, 0x002a_a418)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `StreamBuffer_GetStatus` (0x2aa020): fills the 3-word status record.
pub(crate) const GET_STATUS_OF_BUFFER: Callee = Callee {
    addr: 0x002a_a020,
    f: ee::s08::f_002aa020,
};

/// `StreamBuffer_GetRequestStatus` (0x2aa068): the current request's status.
pub(crate) const GET_REQUEST_STATUS_OF_BUFFER: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::f_002aa068,
};

/// `SNDSTRM_SetPrebuffer` (0x3b7d80), called through the function table.
pub(crate) const SNDSTRM_SET_PREBUFFER_ADDR: u32 = 0x003b_7d80;

/// `SndRequestStatus.state` of a request that is playing.
pub(crate) const REQUEST_PLAYING: u64 = 2;

/// `float StreamBuffer_GetRequestPosition(StreamBuffer* self)` (0x2aa0c0): the
/// current request's status into a frame-local record; while it is playing
/// (state 2), the record's `a` word as a float, else 0.0. The state test leaves
/// `v0` at 2 on the zero path, as the generated body does.
pub fn stream_buffer_get_request_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_REQUEST_STATUS_OF_BUFFER.call(c, 0x002a_a0d4)?;
        let state = Addr(frame.at(c, 0)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = REQUEST_PLAYING;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        if state == REQUEST_PLAYING {
            c.g[V0][0] = Addr(frame.at(c, 0x4)).lw(c);
            abi::set_fpr(c, 0, FpuF32::from_i32(c.g[V0][0] as u32 as i32));
        } else {
            abi::set_fpr(c, 0, FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamBuffer_HasBuffered(StreamBuffer* self, int minVal)` (0x2aa150):
/// the status record's `b` word is at least `minVal` (signed).
pub fn stream_buffer_has_buffered(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_STATUS_OF_BUFFER.call(c, 0x002a_a168)?;
        c.g[V0][0] = Addr(frame.at(c, 0x8)).lw(c);
        frame.restore_ra(c, 0x10);
        let below = (c.g[V0][0] as i64) < (c.g[S0][0] as i64);
        c.g[V0][0] = (below as u64) ^ 1;
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamBuffer_IsRequestPending(StreamBuffer* self)` (0x2aa188): the
/// current request's state is 1.
pub fn stream_buffer_is_request_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_REQUEST_STATUS_OF_BUFFER.call(c, 0x002a_a19c)?;
        let state = Addr(frame.at(c, 0)).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = ((state ^ 1) < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_ClearPrebuffer(StreamBuffer* self)` (0x2aa1e8): the
/// stream's status record gives the stream handle `a`; the prebuffer of that
/// handle is cleared (`SNDSTRM_SetPrebuffer(handle, 0)`).
pub fn stream_buffer_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_STATUS_OF_BUFFER.call(c, 0x002a_a1f8)?;
        c.g[A0][0] = Addr(frame.at(c, 0x4)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, SNDSTRM_SET_PREBUFFER_ADDR, 0x002a_a204)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_ClearPrebufferHandle(void* self, int handle)` (0x2aa240):
/// `SNDSTRM_SetPrebuffer(handle, 0)`.
pub fn stream_buffer_clear_prebuffer_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        call_ptr(c, SNDSTRM_SET_PREBUFFER_ADDR, 0x002a_a254)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `StreamBuffer* StreamBuffer_ctor_2(StreamBuffer *self, SsPlayer *p, char *mem,
/// int size, int n, int m)` (0x002a9c00): fills the stream fields from the
/// player's tables (float curve values for the volume and pitch), then creates the
/// SND stream. Converted from the generated labelled-block body: every register,
/// cycle and call kept.
pub fn stream_buffer_ctor_2(c: &mut Cpu) {
    if c.pc != 0x002a9c00 {
        return ee::s08::f_002a9c00__generated(c);
    }
    guest(c, |c| {
        'b_2a9d8c: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[T2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[T3][0] = c.g[A2][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[T4][0] = c.g[A3][0];
            c.g[A2][0] = c.g[T1][0];
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
            c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[A1][0] == 0u64;
            c.sb((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            if t {
                break 'b_2a9d8c;
            } else {
                'b_2a9d14: {
                    'b_2a9d10: {
                        'b_2a9cc0: {
                            c.cycles += 16;
                            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.lwc1(2, c.g[A1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[AT][0] = 0x42fe0000;
                            c.mtc1(1, 3);
                            c.lwc1(0, c.g[V0][0] as u32);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            if t {
                                break 'b_2a9cc0;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                                c.mtc1(0, 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.lwc1(0, c.g[V0][0] as u32);
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 0, 2);
                                c.c_le_s(1, 0);
                                let t = !c.fcc();
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_2a9d10;
                                } else {
                                    break 'b_2a9cc0;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();
                        c.g[V0][0] = 0x7f;
                        if t {
                            break 'b_2a9d10;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.lwc1(0, c.g[V0][0] as u32);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            let t = true;
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_2a9d14;
                            } else {
                                break 'b_2a9d10;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    break 'b_2a9d14;
                }
                'b_2a9d88: {
                    'b_2a9d5c: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x18;
                        c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[A3][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
                        c.mult(0, 4, 4, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if t {
                            break 'b_2a9d5c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_2a9d88;
                            } else {
                                break 'b_2a9d5c;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        break 'b_2a9d88;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        break 'b_2a9d88;
                    }
                }
                c.cycles += 1;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                break 'b_2a9d8c;
            }
        }
        c.cycles += 18;
        c.g[V0][0] = 0x1;
        c.sw(c.g[S0][0] as u32, c.g[T3][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = 0x18;
        c.g[A1][0] = c.g[T0][0];
        c.g[A3][0] = c.g[T3][0];
        c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
        c.g[T0][0] = c.g[T4][0];
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x20));
        c.mult(0, 4, 4, 3);
        c.g[RA][0] = sx(0x002a9dbc);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x003b7818, 0x002a9dbc)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetRequestLength (0x2aa260)

/// The request status record `StreamBuffer_GetRequestStatus` fills: the state
/// at +0 (2 = playing) and two lengths at +4 and +8.
pub(crate) const SRL_STATE: u32 = 0x0;

pub(crate) const SRL_LENGTH_A: u32 = 0x4;

pub(crate) const SRL_LENGTH_B: u32 = 0x8;

pub(crate) const SRL_PLAYING: u64 = 2;

/// `StreamBuffer_GetRequestStatus(self, out, scratch)` (0x2aa068).
pub(crate) const SRL_GET_REQUEST_STATUS: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::StreamBuffer_GetRequestStatus,
};

/// `float StreamBuffer_GetRequestLength(StreamBuffer* self)` (0x2aa260):
/// asks for the current request's status (into a stack record); while it is
/// playing (state 2) returns the sum of its two lengths as a float, else 0.
pub fn stream_buffer_get_request_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        SRL_GET_REQUEST_STATUS.call(c, 0x002a_a274)?;
        let state = Addr(frame.at(c, SRL_STATE)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = SRL_PLAYING;
        let playing = state == SRL_PLAYING;
        frame.restore_ra(c, 0x10);
        if playing {
            c.cycles += 5;
            c.g[V0][0] = Addr(frame.at(c, SRL_LENGTH_A)).lw(c);
            c.g[V1][0] = Addr(frame.at(c, SRL_LENGTH_B)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            let length = crate::ps2f::FpuF32::from_i32(c.g[V0][0] as u32 as i32);
            crate::abi::set_fpr(c, 0, length);
        } else {
            c.cycles += 3;
            crate::abi::set_fpr(c, 0, crate::ps2f::FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_Pause (0x2aa2b0) and StreamBuffer_Resume (0x2aa2e8)

/// `SND_Lock` / `SND_Unlock` (the sound library lock, through the table).
pub(crate) const SBP_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SBP_SND_UNLOCK: u32 = 0x003b_58d8;

/// `SNDSTRM_SetPitch(handle, pitch)` (0x3b7fb8): pause sets pitch 0.
pub(crate) const SBP_SNDSTRM_SET_PITCH: u32 = 0x003b_7fb8;

/// StreamBuffer field: the SND stream handle.
pub(crate) const SBP_HANDLE: u32 = 0x8;

/// The pitch `StreamBuffer_Resume` sets.
pub(crate) const SBP_RESUME_PITCH: u64 = 0x1000;

/// `void StreamBuffer_Pause(StreamBuffer* self)` (0x2aa2b0): under the sound
/// lock, `SNDSTRM_SetPitch(self->stream, 0)`.
pub fn stream_buffer_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SBP_SND_LOCK, 0x002a_a2c4)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SBP_HANDLE)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, SBP_SNDSTRM_SET_PITCH, 0x002a_a2d0)?;
        call_ptr(c, SBP_SND_UNLOCK, 0x002a_a2d8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_Resume(StreamBuffer* self)` (0x2aa2e8): as
/// `StreamBuffer_Pause`, with the pitch `SBP_RESUME_PITCH`.
pub fn stream_buffer_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SBP_SND_LOCK, 0x002a_a2fc)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SBP_HANDLE)).lw(c);
        c.g[A1][0] = SBP_RESUME_PITCH;
        call_ptr(c, SBP_SNDSTRM_SET_PITCH, 0x002a_a308)?;
        call_ptr(c, SBP_SND_UNLOCK, 0x002a_a310)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_QueueFromBig (0x2aa398)

/// `SNDSTRM` big-file decoder lookup, `BigFile_FindEntry` (0x3e2ce0).
pub(crate) const SQB_BIG_FIND_ENTRY: u32 = 0x003e_2ce0;

/// `SNDSTRM_Queue(handle, a, b, offset)` (0x3b78d8).
pub(crate) const SQB_SNDSTRM_QUEUE: u32 = 0x003b_78d8;

/// StreamBuffer fields: the big-file decoder, the SND stream handle and the
/// base of the queued offset.
pub(crate) const SQB_DECODER: u32 = 0x74;

pub(crate) const SQB_HANDLE: u32 = 0x8;

pub(crate) const SQB_OFFSET: u32 = 0x14;

/// `int StreamBuffer_QueueFromBig(StreamBuffer* self, int a, int b, int off)`
/// (0x2aa398): with a big-file decoder (+0x74), looks up the decoder entry
/// (its size goes to a stack slot) and queues the request on the SND stream
/// at `off` plus that size; without a decoder, returns 0.
pub fn stream_buffer_queue_from_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SQB_DECODER)).lw(c);
        let has_decoder = c.g[A0][0] != 0;
        c.g[S2][0] = c.g[A3][0];
        if has_decoder {
            c.cycles += 10;
            c.g[A3][0] = c.g[SP][0];
            c.g[A1][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, SQB_BIG_FIND_ENTRY, 0x002a_a3d8)?;
            c.g[A3][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SQB_HANDLE)).lw(c);
            c.g[A2][0] = sx(r32(c, S0).wrapping_add(SQB_OFFSET));
            c.g[A3][0] = sx(r32(c, S2).wrapping_add(r32(c, A3)));
            call_ptr(c, SQB_SNDSTRM_QUEUE, 0x002a_a3f0)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_SetStreamParam (0x2aa448)

/// The SND stream setter the other shard holds (0x3b80e0): `(handle, arg)`.
pub(crate) const SBSP_SNDSTRM_SET: u32 = 0x003b_80e0;

/// `int StreamBuffer_SetStreamParam(StreamBuffer* self, int arg)` (0x2aa448):
/// under the sound lock, `SNDSTRM_Set(self->stream, arg)`; returns its result.
pub fn stream_buffer_set_stream_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, SBP_SND_LOCK, 0x002a_a464)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SBP_HANDLE)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SBSP_SNDSTRM_SET, 0x002a_a470)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SBP_SND_UNLOCK, 0x002a_a478)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_CreateStreamEx (0x2aa870)

/// `cMemMan_alloc(size, name, heap, 0)` (0x317d70) and the stream object's
/// size and name string (0x483228).
pub(crate) const SCE_ALLOC: u32 = 0x0031_7d70;

pub(crate) const SCE_NAME: u32 = 0x0048_3228;

pub(crate) const SCE_OBJ_SIZE: u64 = 0x84;

/// StreamArray fields: the buffer pointer array (+8); the sound player the
/// object's constructor takes is at `*self + 0x1d8`.
pub(crate) const SCE_BUFFERS: u32 = 0x8;

pub(crate) const SCE_PLAYER_OFF: u32 = 0x1d8;

/// `StreamBuffer_ctor_2(obj, player, mem, size, n, m)` (0x2a9c00).
pub(crate) const SCE_CTOR: Callee = Callee {
    addr: 0x002a_9c00,
    f: ee::s08::StreamBuffer_ctor_2,
};

/// `StreamBuffer* StreamArray_CreateStreamEx(StreamArray* self, int idx,
/// void* heap, void* mem, int size, int n, int m)` (0x2aa870): allocates a
/// 0x84-byte stream buffer, constructs it on the player at `*self + 0x1d8`
/// and stores it in the array slot `idx`; returns the constructor's result.
pub fn stream_array_create_stream_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = sx(r32(c, A1) << 2);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[S4][0] = c.g[T2][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A3][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[A1][0] = sx(SCE_NAME);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SCE_BUFFERS)).lw(c);
        c.g[A0][0] = SCE_OBJ_SIZE;
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        call_ptr(c, SCE_ALLOC, 0x002a_a8c4)?;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S3][0];
        c.g[T1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(SCE_PLAYER_OFF));
        SCE_CTOR.call(c, 0x002a_a8e4)?;
        let slot = r32(c, S0);
        let stream = c.g[V0][0];
        Addr(slot).sw(c, stream);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_PauseAll (0x2aac98) and StreamArray_ResumeAll (0x2aad08)

/// StreamArray fields: the buffer count and the buffer pointer array.
pub(crate) const SAP_COUNT: u32 = 0x4;

pub(crate) const SAP_BUFFERS: u32 = 0x8;

/// `StreamBuffer_Pause` (0x2aa2b0) and `StreamBuffer_Resume` (0x2aa2e8).
pub(crate) const SAP_STREAM_BUFFER_PAUSE: Callee = Callee {
    addr: 0x002a_a2b0,
    f: ee::s08::StreamBuffer_Pause,
};

pub(crate) const SAP_STREAM_BUFFER_RESUME: Callee = Callee {
    addr: 0x002a_a2e8,
    f: ee::s08::StreamBuffer_Resume,
};

/// The loop `StreamArray_PauseAll` and `StreamArray_ResumeAll` share: for
/// each non-null buffer in the array (count at +4, pointers at +8), calls
/// `op` with the buffer in a0 (returning to `ret`); `head` is the loop head
/// for the back-edge event check.
pub(crate) fn sap_for_each_buffer(c: &mut Cpu, op: Callee, ret: u32, head: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SAP_COUNT)).lw(c);
    let count = c.g[V0][0] as i64;
    c.g[S0][0] = 0;
    if count > 0 {
        c.cycles += 2;
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SAP_BUFFERS)).lw(c);
        loop {
            c.cycles += 5;
            c.g[V0][0] = sx(r32(c, S0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                op.call(c, ret)?;
            }
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SAP_COUNT)).lw(c);
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SAP_BUFFERS)).lw(c);
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void StreamArray_PauseAll(StreamArray* self)` (0x2aac98): pauses every
/// non-null stream buffer.
pub fn stream_array_pause_all(c: &mut Cpu) {
    guest(c, |c| {
        sap_for_each_buffer(c, SAP_STREAM_BUFFER_PAUSE, 0x002a_acdc, 0x002a_acc0)
    })
}

/// `void StreamArray_ResumeAll(StreamArray* self)` (0x2aad08): resumes every
/// non-null stream buffer.
pub fn stream_array_resume_all(c: &mut Cpu) {
    guest(c, |c| {
        sap_for_each_buffer(c, SAP_STREAM_BUFFER_RESUME, 0x002a_ad4c, 0x002a_ad30)
    })
}

/// `StreamArray::buffers`: the pointer to the array of `StreamBuffer*` slots.
pub(crate) const SAQ_BUFFERS: u32 = 0x8;

/// SND lock, `SNDSTRM_GetRequestStatus` and SND unlock, as the request-state
/// wrappers call them (through the function table).
pub(crate) const SAIR_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SAIR_SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const SAIR_REQUEST_STATUS: u32 = 0x003b_7c40;

/// `int StreamArray_QueueFromBig(StreamArray* self, int idx, int a, int b, int c)`
/// (0x2aadc0): queue the request on the stream in slot `idx`
/// (`StreamBuffer_QueueFromBig`), or return -1 when the slot is empty.
pub fn stream_array_queue_from_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A3][0];
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let has_stream = c.g[A0][0] != 0;
        c.g[A3][0] = c.g[T0][0];
        if has_stream {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            c.g[A2][0] = c.g[V1][0];
            Callee {
                addr: 0x002a_a398,
                f: ee::s08::f_002aa398,
            }
            .call(c, 0x002a_adf8)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float StreamArray_GetRequestLength(StreamArray* self, int idx)` (0x2aaec0):
/// the length of the request on slot `idx` (`StreamBuffer_GetRequestLength`),
/// or 0.0 for a negative index or an empty slot. The result is in f0.
pub fn stream_array_get_request_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let index_negative = (c.g[A1][0] as i64) < 0;
        frame.save_ra(c, 0);
        let mut has_stream = false;
        if !index_negative {
            c.cycles += 6;
            c.g[V1][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            has_stream = c.g[A0][0] != 0;
        }
        c.cycles += 3;
        if has_stream {
            Callee {
                addr: 0x002a_a260,
                f: ee::s08::f_002aa260,
            }
            .call(c, 0x002a_aef8)?;
            frame.restore_ra(c, 0);
        } else {
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_HasBuffered(StreamArray* self, int idx, int min)` (0x2aaf08):
/// whether the stream in slot `idx` has at least `min` buffered
/// (`StreamBuffer_HasBuffered`); false for an empty slot.
pub fn stream_array_has_buffered(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_a150,
                f: ee::s08::f_002aa150,
            }
            .call(c, 0x002a_af30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_IsRequestPending(StreamArray* self, int idx)` (0x2aaf40):
/// whether the stream in slot `idx` has a request pending
/// (`StreamBuffer_IsRequestPending`); false for an empty slot.
pub fn stream_array_is_request_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002a_a188,
                f: ee::s08::f_002aa188,
            }
            .call(c, 0x002a_af68)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_IsRequestDone(StreamArray* self, int handle)` (0x2aaf78,
/// streamsys.cpp `cStreamSys_STREAM_IsRequestedDequeued`): true when the
/// request `handle` reports state 3 (done) or the status query failed. The
/// query runs under the SND lock: `SNDSTRM_GetRequestStatus(handle, st)`
/// fills `st[0]` with the state and returns the result in s0.
pub fn stream_array_is_request_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SAIR_SND_LOCK, 0x002a_af8c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, SAIR_REQUEST_STATUS, 0x002a_af98)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SAIR_SND_UNLOCK, 0x002a_afa0)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 3;
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0;
        c.cycles += 2;
        if done {
            c.g[A0][0] = 1;
        } else {
            // The status query's result: below zero means failure, which
            // also counts as done.
            let failed = (c.g[S0][0] as i64) < 0;
            if failed {
                c.cycles += 2;
                c.g[A0][0] = 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_IsRequestPlaying(StreamArray* self, int handle)`
/// (0x2aafd0): true when the request `handle` reports state 2 (playing) and
/// the status query did not fail. Same query as `StreamArray_IsRequestDone`.
pub fn stream_array_is_request_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SAIR_SND_LOCK, 0x002a_afe4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, SAIR_REQUEST_STATUS, 0x002a_aff0)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SAIR_SND_UNLOCK, 0x002a_aff8)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 2;
        c.g[A0][0] = 0;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 0) as u64;
            c.g[A0][0] = c.g[V0][0] ^ 0x1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_ClearPrebuffer(StreamArray* self, int idx)` (0x2ab060):
/// clear the prebuffer of the stream in slot `idx` (`StreamBuffer_ClearPrebuffer`);
/// nothing for an empty slot.
pub fn stream_array_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            Callee {
                addr: 0x002a_a1e8,
                f: ee::s08::f_002aa1e8,
            }
            .call(c, 0x002a_b088)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_ClearPrebufferHandle(StreamArray* self, int idx, int handle)`
/// (0x2ab098): clear the prebuffer of request `handle` on slot `idx`
/// (`StreamBuffer_ClearPrebufferHandle`); nothing for an empty slot.
pub fn stream_array_clear_prebuffer_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_a240,
                f: ee::s08::f_002aa240,
            }
            .call(c, 0x002a_b0c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_SetStreamVolume(StreamArray* a, int idx, int vol)`
/// (0x2ab0f0): the stream in slot `idx` of the table at `a+0x8` gets `vol`
/// as its base volume. An empty slot does nothing.
pub fn stream_array_set_stream_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_b118);
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_9f30,
                f: ee::s08::f_002a9f30,
            }
            .call(c, 0x002a_b118)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int StreamArray_SetStreamParam(StreamArray* a, int idx, int arg)`
/// (0x2ab1c8): as `StreamArray_SetStreamVolume`, but the stream's parameter
/// setter runs and its result is returned; -1 for an empty slot.
pub fn stream_array_set_stream_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !empty {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002a_b1f0);
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_a448,
                f: ee::s08::f_002aa448,
            }
            .call(c, 0x002a_b1f0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SoundRequest_SetBankSample_2(SoundRequest* r, int bank, int sample)`
/// (0x2abd38): the bank goes to +0xc and the sample to +0x10 (the same
/// stores as `SoundRequest_SetBankSample`, which the Ghidra pseudo-C shows).
pub fn sound_request_set_bank_sample_2(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    let sample = c.g[A2][0];
    Addr(r.wrapping_add(0x10)).sw(c, sample);
    let bank = c.g[A1][0];
    Addr(r.wrapping_add(0xc)).sw(c, bank);
    abi::ret(c);
}

/// `void BankMonitorInstance_Pause(BankMonitorInstance* self)` (0x2acac8):
/// unless the instance is free or already fading, reset its pitch and
/// volume and, for a type-2 instance, pause its sequence handle.
pub fn bank_monitor_instance_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002a_cadc);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002a_bb80,
            f: ee::s08::f_002abb80,
        }
        .call(c, 0x002a_cadc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x54).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002a_cafc);
                c.g[A1][0] = 0;
                Callee {
                    addr: 0x002a_be78,
                    f: ee::s08::f_002abe78,
                }
                .call(c, 0x002a_cafc)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002a_cb08);
                c.g[A1][0] = 0;
                Callee {
                    addr: 0x002a_bf60,
                    f: ee::s08::f_002abf60,
                }
                .call(c, 0x002a_cb08)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = 2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x002a_cb20);
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                    call_ptr(c, 0x003b_9d00, 0x002a_cb20)?;
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc779_install_fakes;

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

    pub(crate) use crate::game::lib::snd::tests::snd_world;

    /// `StreamBuffer_Queue`: the real `SNDSTRM_Queue` takes its "no such
    /// stream" short path (no stream table); handles positive and negative.
    #[test]
    fn stream_buffer_queue_matches() {
        check(
            ee::s08::f_002aa320__generated,
            stream_buffer_queue,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aa320, 0);
                let this = DATA;
                // (The big-file branch is not exercised: it runs the
                // `BigFile_FindEntry` dispatcher, whose zeroed-data path
                // could not be bounded.)
                c.bus.write32(this + STREAM_BUFFER_BIG, 0);
                let handle = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() % 0x100
                };
                c.bus.write32(this + STREAM_BUFFER_HANDLE, handle);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1_8000);
                c.g[A2][0] = sx(DATA + 0x1_9000);
                c.g[A3][0] = (r.next_u32() % 0x4000) as u64;
                c.pc = 0x002a_a320;
            },
        );
    }

    // ------------------------------------------------------ StreamArray_Queue

    #[test]
    fn stream_array_queue_matches() {
        check(
            ee::s08::f_002aad78__generated,
            stream_array_queue,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_ad78 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002a_ad78;
                let this = DATA;
                let array = DATA + 0x1000;
                let entry = DATA + 0x2000;
                let i = seed % 4;
                c.bus.write32(this + 8, array);
                c.bus
                    .write32(array + 4 * i, if seed % 2 == 0 { 0 } else { entry });
                // Keep the queued buffer on StreamBuffer_Queue's short path.
                c.bus.write32(entry + 0x74, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = i as u64;
                c.g[A2][0] = sx(DATA + 0x3000);
                c.g[A3][0] = sx(DATA + 0x4000);
                c.g[T0][0] = sx(DATA + 0x5000);
            },
        );
    }

    // ------------------------------------------------ StreamBuffer_SetPitch

    #[test]
    fn stream_buffer_set_pitch_matches() {
        covered(
            ee::s08::f_002a9f38__generated,
            stream_buffer_set_pitch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a9f, 1);
                c.pc = 0x002a_9f38;
                // No streams: the handle resolves to no stream and the pitch
                // setter takes its early return.
                snd_world(c, &mut r, false);
                let this = DATA;
                c.bus.write32(this + 8, 0xffff_ffff);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 {
                    r.edge_f32() as u64
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    // ------------------------------------------------ SoundRequest_SetStream

    #[test]
    fn sound_request_set_stream_matches() {
        check(
            ee::s08::f_002abcf8__generated,
            sound_request_set_stream,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_bcf8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002a_bcf8;
                let req = DATA;
                let src = DATA + 0x1000;
                let n = 1 + r.next_u32() % 24;
                for k in 0..n {
                    c.bus.write8(src + k, 1 + (r.next_u32() % 0x7e) as u8);
                }
                for k in n..n + 48 {
                    c.bus.write8(src + k, 0);
                }
                c.g[A0][0] = sx(req);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(src);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    pub(crate) use crate::game::sound::bankmonitor::tests::oc339_speech_stream;

    /// `SpeechSystem_ClearPrebufferCallback`: the real
    /// `SNDSTRM_SetPrebuffer` with a request found or not.
    #[test]
    fn speech_system_clear_prebuffer_callback_matches() {
        covered(
            ee::s08::f_002ab1a0__generated,
            speech_system_clear_prebuffer_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b1a0, 2);
                c.pc = 0x002a_b1a0;
                let stream = oc339_speech_stream(c, &mut r, seed);
                c.g[A1][0] = stream as u64;
            },
        );
    }

    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_gate;

    pub(crate) use crate::game::fe::uitemplatemap::tests::OC777_OBJ;
    pub(crate) use crate::game::fe::uitemplatemap::tests::OC777_UNWIND_ADDR;
    pub(crate) use crate::game::sound::soundsys::tests::oc779_audio_ctor_fixture;

    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and the fixture objects, clear of the fakes' call
    /// log (`DATA + 0x10_0000`) and pointer range (`DATA .. DATA + 0x4_0000`).
    pub(crate) const OC775_CTRL: u32 = DATA + 0x2e_0000;

    pub(crate) const OC775_MAGIC: u32 = 0x4f43_3737 + 5;

    pub(crate) const OC775_OBJ: u32 = DATA + 0x30_0000;

    /// Gated fakes for the callees that have an override slot.
    macro_rules! oc775_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC775_CTRL) != OC775_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc775_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc775_fakes!(
        (
            oc775_fake_002aa7f0,
            OC775_PREV_002AA7F0,
            ee::s08::OVERRIDE_002AA7F0,
            ee::s08::f_002aa7f0__generated,
            0x002a_a7f0
        ),
        (
            oc775_fake_002ade88,
            OC775_PREV_002ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc775_fake_00317e30,
            OC775_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc775_fake_003b5a28,
            OC775_PREV_003B5A28,
            ee::s13::OVERRIDE_003B5A28,
            ee::s13::f_003b5a28__generated,
            0x003b_5a28
        ),
        (
            oc775_fake_002948a0,
            OC775_PREV_002948A0,
            ee::s07::OVERRIDE_002948A0,
            ee::s07::f_002948a0__generated,
            0x0029_48a0
        ),
        (
            oc775_fake_002a4590,
            OC775_PREV_002A4590,
            ee::s08::OVERRIDE_002A4590,
            ee::s08::f_002a4590__generated,
            0x002a_4590
        ),
        (
            oc775_fake_00287968,
            OC775_PREV_00287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc775_fake_002b0e60,
            OC775_PREV_002B0E60,
            ee::s08::OVERRIDE_002B0E60,
            ee::s08::f_002b0e60__generated,
            0x002b_0e60
        ),
        (
            oc775_fake_00317530,
            OC775_PREV_00317530,
            ee::s10::OVERRIDE_00317530,
            ee::s10::f_00317530__generated,
            0x0031_7530
        ),
        (
            oc775_fake_00317550,
            OC775_PREV_00317550,
            ee::s10::OVERRIDE_00317550,
            ee::s10::f_00317550__generated,
            0x0031_7550
        ),
        (
            oc775_fake_0014dc80,
            OC775_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc775_fake_0014dd58,
            OC775_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc775_fake_0014bd78,
            OC775_PREV_0014BD78,
            ee::s01::OVERRIDE_0014BD78,
            ee::s01::f_0014bd78__generated,
            0x0014_bd78
        ),
        (
            oc775_fake_0014af10,
            OC775_PREV_0014AF10,
            ee::s01::OVERRIDE_0014AF10,
            ee::s01::f_0014af10__generated,
            0x0014_af10
        ),
        (
            oc775_fake_00397c40,
            OC775_PREV_00397C40,
            ee::s13::OVERRIDE_00397C40,
            ee::s13::f_00397c40__generated,
            0x0039_7c40
        ),
        (
            oc775_fake_001a2b00,
            OC775_PREV_001A2B00,
            ee::s03::OVERRIDE_001A2B00,
            ee::s03::f_001a2b00__generated,
            0x001a_2b00
        ),
        (
            oc775_fake_0019ce68,
            OC775_PREV_0019CE68,
            ee::s03::OVERRIDE_0019CE68,
            ee::s03::f_0019ce68__generated,
            0x0019_ce68
        ),
        (
            oc775_fake_001a0498,
            OC775_PREV_001A0498,
            ee::s03::OVERRIDE_001A0498,
            ee::s03::f_001a0498__generated,
            0x001a_0498
        ),
        (
            oc775_fake_00317d70,
            OC775_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc775_fake_001a0720,
            OC775_PREV_001A0720,
            ee::s03::OVERRIDE_001A0720,
            ee::s03::f_001a0720__generated,
            0x001a_0720
        ),
        (
            oc775_fake_001a2e20,
            OC775_PREV_001A2E20,
            ee::s03::OVERRIDE_001A2E20,
            ee::s03::f_001a2e20__generated,
            0x001a_2e20
        ),
        (
            oc775_fake_0028b180,
            OC775_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc775_fake_003de670,
            OC775_PREV_003DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003d_e670
        ),
        (
            oc775_fake_00391708,
            OC775_PREV_00391708,
            ee::s12::OVERRIDE_00391708,
            ee::s12::f_00391708__generated,
            0x0039_1708
        ),
    );

    /// `StreamArray_CreateStreamDefaultParams`: the sys object with its
    /// parameter-array pointers (`+0x1d8 ..`) pointing at distinct zero-filled
    /// areas, a random instance counter (so the arrays are indexed 1..3) and a
    /// random default descriptor; `StreamArray_CreateStream` is a gated fake
    /// (unwinding on some seeds), `memset` runs for real.
    #[test]
    fn stream_array_create_stream_default_params_matches() {
        oc775_install_fakes();
        check(
            ee::s08::f_002aa910__generated,
            stream_array_create_stream_default_params,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x775a, 1);
                c.bus.write32(OC775_CTRL, OC775_MAGIC);
                let this = OC775_OBJ;
                let sys = OC775_OBJ + 0x1000;
                c.bus.write32(this, sys);
                c.bus.write32(sys + 0x1f4, r.next_u32() % 3);
                for w in 0..6u32 {
                    c.bus.write32(sys + 0x1dc + 4 * w, r.next_u32());
                }
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    let area = OC775_OBJ + 0x4000 + 0x100 * k as u32;
                    c.bus.write32(sys + 0x1d8 + off, area);
                    for w in 0..0x40u32 {
                        c.bus.write32(area + 4 * w, r.next_u32());
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
            },
        );
    }

    /// Gp words every audio-system fixture needs (BELibrary dummy and world).
    pub(crate) fn oc775_gp_words(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), OC775_OBJ + 0x8000);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x848), OC775_OBJ + 0x9000);
    }

    /// `SSXAudioSystem_BaseCtor` with `embedded` 0 and 1, a zeroed 0x7780-byte
    /// object, gated fakes on the callees with slots (the same set the
    /// `SSXAudioSystem_ctor` test uses). The sound manager, bank manager,
    /// stream system and the crowd/music/speech ctors run for real; if one of
    /// them runs away on the fixture the entry is dropped.
    #[test]
    fn ssx_audio_system_base_ctor_matches() {
        oc775_install_fakes();
        check(
            ee::s08::f_002ab200__generated,
            ssx_audio_system_base_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x775b, 7);
                c.bus.write32(OC775_CTRL, OC775_MAGIC);
                oc775_gp_words(c);
                c.g[A0][0] = sx(OC775_OBJ);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = 0x1c;
                c.g[A3][0] = 3;
                c.g[T0][0] = (seed / 2 % 3) as u64;
            },
        );
    }

    /// `Font_ctor` (already hand-written in `lightman`): default UV transforms
    /// and colours into a random font object, then `Font_Load` (a gated
    /// fake, unwinding on some seeds).
    #[test]
    fn font_ctor_matches() {
        oc775_install_fakes();
        check(
            ee::s12::f_003915e8__generated,
            crate::game::render::lightman::font_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x775d, 1);
                c.bus.write32(OC775_CTRL, OC775_MAGIC);
                for w in 0..0x20u32 {
                    c.bus.write32(OC775_OBJ + 4 * w, r.next_u32());
                }
                c.g[A0][0] = sx(OC775_OBJ);
                c.g[A1][0] = sx(DATA + 0x20_0000);
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    /// Fixture of `stream_buffer_ctor`: a stream instance object, and the
    /// bank monitor `p` (current instance at +0x1c, current-descriptor array
    /// at +0x20 (24-byte entries), base-volume array at +0x38, volume-control
    /// pointer array at +0x3c). A quarter of the seeds have no volume
    /// control; its float is an edge value. `unwind_alloc` makes the buffer
    /// allocation (`operator_new_named`, 0x317e30) unwind, so the real
    /// `SNDSTRM_create` is not reached.
    pub(crate) fn oc777_stream_fixture(c: &mut Cpu, seed: u32, unwind_alloc: bool) {
        let mut r = machine(c, seed, 0x777a, 1);
        c.pc = 0x002a_99d8;
        oc777_gate(c);
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        if unwind_alloc {
            c.bus.write32(OC777_UNWIND_ADDR, 0x0031_7e30);
        }
        let this = OC777_OBJ;
        let p = OC777_OBJ + 0x1000;
        let ctl_ptrs = OC777_OBJ + 0x2000;
        let base_vols = OC777_OBJ + 0x2100;
        let current = OC777_OBJ + 0x2200;
        let ctl_cells = OC777_OBJ + 0x2300;
        let inst = r.next_u32() % 4;
        c.bus.write32(p + 0x1c, inst);
        c.bus.write32(p + 0x20, current);
        c.bus.write32(p + 0x38, base_vols);
        c.bus.write32(p + 0x3c, ctl_ptrs);
        for i in 0..4u32 {
            let ctl = if i == inst && seed % 4 == 0 {
                0
            } else {
                ctl_cells + 4 * i
            };
            c.bus.write32(ctl_ptrs + 4 * i, ctl);
            let bits = r.edge_f32();
            c.bus.write32(ctl_cells + 4 * i, bits);
            let vol = match r.next_u32() % 4 {
                0 => r.next_u32(),
                1 => 126 + r.next_u32() % 4,
                _ => (r.next_u32() % 400).wrapping_sub(100),
            };
            c.bus.write32(base_vols + 4 * i, vol);
        }
        for w in 0..0x20u32 {
            c.bus.write32(current + 4 * w, r.next_u32());
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(p);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32() % 0x400);
    }

    /// `StreamBuffer_ctor` up to the buffer allocation: field resets, the
    /// volume-control rescale and clamps (0..127, `cvt.w.s`), the current
    /// descriptor's volume byte, `SNDSTRM_overhead` (real), and the gated
    /// `operator_new_named` which unwinds.
    #[test]
    fn stream_buffer_ctor_matches() {
        oc777_install_fakes();
        check(
            ee::s08::f_002a99d8__generated,
            stream_buffer_ctor,
            80,
            &|c, seed| {
                oc777_stream_fixture(c, seed, true);
            },
        );
    }

    /// `StreamBuffer_ctor` through the real `SNDSTRM_create` on the
    /// allocated (zeroed) buffer. If the stream code runs away on the
    /// fixture, drop this test and keep the one above.
    #[test]
    fn stream_buffer_ctor_full_matches() {
        oc777_install_fakes();
        check(
            ee::s08::f_002a99d8__generated,
            stream_buffer_ctor,
            40,
            &|c, seed| {
                oc777_stream_fixture(c, seed, false);
            },
        );
    }

    /// `StreamArray_ctor` for in-charge 0/1 and stream counts 0, 1, 3, 4 (the
    /// count is the `a3` argument), unwinding at random fake calls.
    #[test]
    fn stream_array_ctor_matches() {
        oc779_install_fakes();
        check(
            ee::s08::f_002aa490__generated,
            stream_array_ctor,
            64,
            &|c, seed| {
                oc779_audio_ctor_fixture(c, seed, 0x779a, 4);
                c.g[A3][0] = [0u64, 1, 3, 4][(seed / 2 % 4) as usize];
            },
        );
    }

    /// `StreamBuffer_Stop`: passes the buffer's stream handle to the real
    /// `SNDSTRM_Stop` (no stream is registered, so it takes the not-found path).
    #[test]
    fn stream_buffer_stop_matches() {
        check(
            ssx3_ee::s08::f_002aa408__generated,
            stream_buffer_stop,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x799b, 0);
                c.pc = 0x002a_a408;
                let sb = DATA + 0x1_0000;
                c.bus.write32(sb + STREAM_BUFFER_STOP_HANDLE, r.next_u32());
                c.g[A0][0] = sx(sb);
            },
        );
    }

    /// Gate word of this test's callee fakes (in the machine's own RAM).
    const FL_CTRL: u32 = DATA + 0x38_0000;

    const FL_MAGIC: u32 = 0x0815_f1a0;

    /// Every callee of the function under test; all are faked (the real
    /// chains reach the disc loader and the state constructors, which run
    /// away on test data).
    const FL_ADDRS: [u32; 17] = [
        0x0031_7530,
        0x0031_7550,
        0x0014_dc80,
        0x0014_dd58,
        0x0014_bd78,
        0x0014_af10,
        0x0039_7c40,
        0x001a_2b00,
        0x0019_ce68,
        0x001a_0498,
        0x0031_7d70,
        0x001a_0720,
        0x001a_2e20,
        0x0038_acd0,
        0x0028_b180,
        0x0028_5fb0,
        0x002c_6e98,
    ];

    fn fl_idx(addr: u32) -> u32 {
        FL_ADDRS
            .iter()
            .position(|&a| a == addr)
            .expect("fake address") as u32
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns a
    /// small number (the allocator, constructors and library lookups keep
    /// the fake's pointer), or 0 for the calls the fixture's per-callee mask
    /// picks (bit n = nth call of that callee: a failed allocation, a
    /// missing object).
    fn fl_fake_call(c: &mut Cpu, addr: u32) {
        let i = fl_idx(addr);
        let (mask, count) = (FL_CTRL + 0x100 + 4 * i, FL_CTRL + 0x300 + 4 * i);
        let n = c.bus.read32(count);
        c.bus.write32(count, n.wrapping_add(1));
        let zm = c.bus.read32(mask);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let zero = n < 32 && zm >> n & 1 != 0;
        if zero {
            c.g[V0][0] = 0;
        } else if !matches!(
            addr,
            0x0014_dc80 | 0x0014_dd58 | 0x0031_7d70 | 0x0028_b180 | 0x001a_0720 | 0x001a_2e20
        ) {
            c.g[V0][0] >>= 4;
            c.g[V0][0] &= 3;
        }
    }

    macro_rules! fl_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FL_CTRL) != FL_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fl_fake_call(c, $addr);
                }
            )*
            fn fl_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    fl_fakes!(
        (
            fl_fk_00317530,
            FL_PREV_00317530,
            ee::s10::OVERRIDE_00317530,
            ee::s10::f_00317530__generated,
            0x0031_7530
        ),
        (
            fl_fk_00317550,
            FL_PREV_00317550,
            ee::s10::OVERRIDE_00317550,
            ee::s10::f_00317550__generated,
            0x0031_7550
        ),
        (
            fl_fk_0014dc80,
            FL_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            fl_fk_0014dd58,
            FL_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            fl_fk_0014bd78,
            FL_PREV_0014BD78,
            ee::s01::OVERRIDE_0014BD78,
            ee::s01::f_0014bd78__generated,
            0x0014_bd78
        ),
        (
            fl_fk_0014af10,
            FL_PREV_0014AF10,
            ee::s01::OVERRIDE_0014AF10,
            ee::s01::f_0014af10__generated,
            0x0014_af10
        ),
        (
            fl_fk_00397c40,
            FL_PREV_00397C40,
            ee::s13::OVERRIDE_00397C40,
            ee::s13::f_00397c40__generated,
            0x0039_7c40
        ),
        (
            fl_fk_001a2b00,
            FL_PREV_001A2B00,
            ee::s03::OVERRIDE_001A2B00,
            ee::s03::f_001a2b00__generated,
            0x001a_2b00
        ),
        (
            fl_fk_0019ce68,
            FL_PREV_0019CE68,
            ee::s03::OVERRIDE_0019CE68,
            ee::s03::f_0019ce68__generated,
            0x0019_ce68
        ),
        (
            fl_fk_001a0498,
            FL_PREV_001A0498,
            ee::s03::OVERRIDE_001A0498,
            ee::s03::f_001a0498__generated,
            0x001a_0498
        ),
        (
            fl_fk_00317d70,
            FL_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            fl_fk_001a0720,
            FL_PREV_001A0720,
            ee::s03::OVERRIDE_001A0720,
            ee::s03::f_001a0720__generated,
            0x001a_0720
        ),
        (
            fl_fk_001a2e20,
            FL_PREV_001A2E20,
            ee::s03::OVERRIDE_001A2E20,
            ee::s03::f_001a2e20__generated,
            0x001a_2e20
        ),
        (
            fl_fk_0038acd0,
            FL_PREV_0038ACD0,
            ee::s12::OVERRIDE_0038ACD0,
            ee::s12::f_0038acd0__generated,
            0x0038_acd0
        ),
        (
            fl_fk_0028b180,
            FL_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            fl_fk_00285fb0,
            FL_PREV_00285FB0,
            ee::s07::OVERRIDE_00285FB0,
            ee::s07::f_00285fb0__generated,
            0x0028_5fb0
        ),
        (
            fl_fk_002c6e98,
            FL_PREV_002C6E98,
            ee::s08::OVERRIDE_002C6E98,
            ee::s08::f_002c6e98__generated,
            0x002c_6e98
        ),
    );

    /// Arm the fakes and give each callee a random call mask (none, all, or
    /// random bits, by seed and callee).
    fn fl_arm(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fl_install_fakes();
        c.bus.write32(FL_CTRL, FL_MAGIC);
        for i in 0..FL_ADDRS.len() as u32 {
            let z = match (seed / 5 + i) % 5 {
                0 | 1 => 0,
                2 => u32::MAX,
                _ => r.next_u32(),
            };
            c.bus.write32(FL_CTRL + 0x100 + 4 * i, z);
        }
    }

    /// The fixture of `FEModule_Load`: the module object with a valid UI
    /// object at `+0xc`, the progress object (`gp+0x2a74`), the engine
    /// (`gp-0x854`) and the world (`gp-0x848`) with fake-method vtables, and
    /// the locale object (`world+0x8c`) whose language byte and loaded-table
    /// slots vary by seed (the real, unslotted `FELocale_LoadFETables` runs
    /// on it; its allocation and string table load are faked).
    fn fl_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        let mut r = machine(c, seed, salt, calls);
        crate::difftest::elf_image(c);
        fl_arm(c, &mut r, seed);
        let g = |off: u32| GAME_GP - off;
        let this = DATA + 0x30_0000;
        fill(c, &mut r, this, 0x40);
        let ui = DATA + 0x30_1000;
        fill(c, &mut r, ui, 0x20);
        c.bus.write32(this + 0xc, ui);
        c.bus.write32(GAME_GP + 0x2a74, DATA + 0x30_2000);
        let (engine, evt) = (DATA + 0x30_4000, DATA + 0x30_5000);
        c.bus.write32(g(0x854), engine);
        c.bus.write32(engine + 0x10d8, evt);
        c.bus.write16(evt + 0x160, (r.next_u32() & 0x70) as u16);
        c.bus.write32(evt + 0x164, FAKE);
        let (world, locale, cam, cam_vt) = (
            DATA + 0x31_0000,
            DATA + 0x32_0000,
            DATA + 0x31_1000,
            DATA + 0x31_2000,
        );
        c.bus.write32(g(0x848), world);
        c.bus.write32(world + 0x8c, locale);
        c.bus.write32(world + 0x88, r.next_u32());
        c.bus.write32(world + 0x7c, cam);
        c.bus.write32(cam + 0xa0, cam_vt);
        c.bus.write16(cam_vt + 0x18, 0);
        c.bus.write32(cam_vt + 0x1c, FAKE);
        for i in 0..8 {
            c.bus.write32(
                locale + 8 + 4 * i,
                if (seed / 5 + i) % 3 == 0 {
                    DATA + 0x32_1000
                } else {
                    0
                },
            );
        }
        c.bus
            .write8(locale + 1, [0u8, 1, 2, 3, 4, 9][(seed % 6) as usize]);
        c.g[A0][0] = sx(this);
        c.pc = 0x001a_1ec0;
    }

    /// `FEModule_Load`: every callee faked except the locale loader; the
    /// language byte (0..3 and others), loaded or missing locale tables,
    /// failing allocations and unwinds of its calls.
    #[test]
    fn fe_module_load_matches() {
        fakes::covered(
            ee::s03::f_001a1ec0__generated,
            fe_module_load,
            240,
            &|c, seed| fl_fixture(c, seed, 0x1a1ec0, 20),
        );
    }

    /// Several distinct paths (cycle totals) are taken over the seeds.
    #[test]
    fn fe_module_load_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = crate::difftest::fresh();
            fl_fixture(&mut c, seed, 0x1a1ec0, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s03::f_001a1ec0__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 5, "only {} paths: {seen:?}", seen.len());
    }

    use crate::difftest::fakes::machine_rt;
    use std::ptr::addr_of_mut;

    /// Gate of this module's fakes: a word in the machine's RAM, written by
    /// the fixtures that want the fakes to answer (else the previous body runs).
    pub(crate) const OC1036_CTRL: u32 = DATA + 0x2e_8000;

    pub(crate) const OC1036_MAGIC: u32 = 0x1036_c0de;

    pub(crate) static OC1036_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1036_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1036_PREV_SNDSTRM_CREATE: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemFree` (0x317e98): records the call and returns.
    pub(crate) fn oc1036_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1036_CTRL) != OC1036_MAGIC {
            return fakes::fall_back(c, &OC1036_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        fakes::fake_call(c, 0x0031_7e98);
    }

    /// `__builtin_delete` (0x317e50): records the call and returns.
    pub(crate) fn oc1036_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1036_CTRL) != OC1036_MAGIC {
            return fakes::fall_back(c, &OC1036_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `SNDSTRM_Create` (0x3b7818): records the call and returns.
    pub(crate) fn oc1036_fake_sndstrm_create(c: &mut Cpu) {
        if c.bus.read32(OC1036_CTRL) != OC1036_MAGIC {
            return fakes::fall_back(
                c,
                &OC1036_PREV_SNDSTRM_CREATE,
                ee::s13::f_003b7818__generated,
            );
        }
        fakes::fake_call(c, 0x003b_7818);
    }

    /// Install the gated fakes (once per process).
    pub(crate) fn oc1036_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1036_fake_free,
            &OC1036_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1036_fake_delete,
            &OC1036_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7818),
            oc1036_fake_sndstrm_create,
            &OC1036_PREV_SNDSTRM_CREATE,
        );
    }

    pub(crate) fn oc1036_gate_on(c: &mut Cpu) {
        c.bus.write32(OC1036_CTRL, OC1036_MAGIC);
    }

    /// The stream object of the status readers, its stream slot and request.
    pub(crate) const SELF: u32 = DATA + 0x20_0000;

    pub(crate) const STREAM: u32 = SELF + 0x1000;

    pub(crate) const REQ: u32 = SELF + 0x2000;

    /// The status readers' world (as the GetStatus / GetRequestStatus tests):
    /// SND lock hooks, one stream slot whose request list holds `REQ`. The
    /// request is playing on even seeds (state byte 2), else another state.
    pub(crate) fn stream_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
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
        c.bus.write32(REQ, 0);
        c.bus.write32(
            REQ + 0xc,
            if seed % 4 < 2 {
                0x8000_0000 | r.next_u32()
            } else {
                r.next_u32() & 0x7fff_ffff
            },
        );
        c.bus.write8(
            REQ + 0x24,
            if seed % 2 == 0 {
                2
            } else {
                1 + (r.next_u32() % 4) as u8
            },
        );
        c.bus
            .write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
        c.bus
            .write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
        let done = r.next_u32() & 0xf_ffff;
        c.bus.write32(REQ + 0x14, done);
        c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
        c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
        c.bus
            .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
    }

    #[test]
    fn stream_buffer_get_request_position_matches() {
        check(
            ee::s08::f_002aa0c0__generated,
            stream_buffer_get_request_position,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a0c0, 4);
                c.pc = 0x002a_a0c0;
                c.g[A0][0] = sx(SELF);
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_has_buffered_matches() {
        check(
            ee::s08::f_002aa150__generated,
            stream_buffer_has_buffered,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a150, 4);
                c.pc = 0x002a_a150;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(if seed % 2 == 0 {
                    r.next_u32() % 4
                } else {
                    r.next_u32()
                });
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_is_request_pending_matches() {
        check(
            ee::s08::f_002aa188__generated,
            stream_buffer_is_request_pending,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a188, 4);
                c.pc = 0x002a_a188;
                c.g[A0][0] = sx(SELF);
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_clear_prebuffer_matches() {
        oc1036_install();
        check(
            ee::s08::f_002aa1e8__generated,
            stream_buffer_clear_prebuffer,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a1e8, 4);
                oc1036_gate_on(c);
                c.pc = 0x002a_a1e8;
                c.g[A0][0] = sx(SELF);
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_clear_prebuffer_handle_matches() {
        oc1036_install();
        check(
            ee::s08::f_002aa240__generated,
            stream_buffer_clear_prebuffer_handle,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a240, 2);
                oc1036_gate_on(c);
                c.pc = 0x002a_a240;
                snd_world(c, &mut r, seed % 2 == 0);
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(r.next_u32()) };
            },
        );
    }

    #[test]
    fn stream_buffer_ctor_2_matches() {
        const P: u32 = SELF + 0x4000;
        const TABLE_A: u32 = SELF + 0x5000;
        const TABLE_B: u32 = SELF + 0x5100;
        oc1036_install();
        check(
            ee::s08::f_002a9c00__generated,
            stream_buffer_ctor_2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_9c00, 3);
                oc1036_gate_on(c);
                c.pc = 0x002a_9c00;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(P);
                c.g[A2][0] = sx(SELF + 0x6000);
                c.g[A3][0] = sx(r.next_u32() % 4096);
                c.g[T0][0] = sx(r.next_u32() % 8);
                c.g[T1][0] = sx(r.next_u32() % 8);
                c.bus.write32(P + 0x1c, r.next_u32() % 16);
                c.bus.write32(P + 0x20, r.next_u32() % 16);
                c.bus.write32(P + 0x38, TABLE_B);
                c.bus.write32(P + 0x3c, TABLE_A);
                for k in 0..64u32 {
                    let a = if k % 5 == 0 {
                        u32::MAX - r.next_u32() % 5
                    } else {
                        r.next_u32() % 300
                    };
                    c.bus.write32(TABLE_A + k * 4, a);
                    c.bus.write32(TABLE_B + k * 4, r.next_u32() % 300);
                }
            },
        );
    }

    /// StreamBuffer_GetRequestLength: the request status on one SND stream
    /// slot (as the time-remaining test sets it up), with the request's state
    /// 2 (playing) on about a quarter of the seeds and idle, unknown or
    /// zero-state requests on the others: the sum of the two lengths, or 0.
    #[test]
    fn stream_buffer_get_request_length_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2100;
        check(
            ee::s08::f_002aa260__generated,
            stream_buffer_get_request_length,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a260, 4);
                c.pc = 0x002a_a260;
                c.g[A0][0] = sx(SELF);
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot (SND stream 0).
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                c.bus.write32(SELF + SBP_HANDLE, 0);
                c.bus.write32(STREAM + 0x124, 0);
                c.bus
                    .write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
                c.bus.write32(STREAM + 0x11c, REQ);
                let case = seed % 4;
                c.bus.write32(REQ, 0);
                c.bus.write32(REQ + 0xc, if case == 3 { 7 } else { 0 });
                let state = match case {
                    1 => 2,
                    2 => 0,
                    _ => 1 + r.next_u32() as u8 % 4,
                };
                c.bus.write8(REQ + 0x24, state);
                let done = r.next_u32() & 0xf_ffff;
                c.bus.write32(REQ + 0x14, done);
                c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
                c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
            },
        );
    }

    /// StreamBuffer_Pause: the sound lock hooks are fakes, the stream table
    /// has no streams, so SNDSTRM_SetPitch takes its early return; the handle
    /// is unknown or random.
    #[test]
    fn stream_buffer_pause_matches() {
        covered(
            ee::s08::f_002aa2b0__generated,
            stream_buffer_pause,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a2b0, 2);
                c.pc = 0x002a_a2b0;
                snd_world(c, &mut r, false);
                let sb = DATA + 0x100;
                c.g[A0][0] = sx(sb);
                let handle = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(sb + SBP_HANDLE, handle);
            },
        );
    }

    /// StreamBuffer_Resume: as the pause test, with the pitch restore.
    #[test]
    fn stream_buffer_resume_matches() {
        covered(
            ee::s08::f_002aa2e8__generated,
            stream_buffer_resume,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a2e8, 2);
                c.pc = 0x002a_a2e8;
                snd_world(c, &mut r, false);
                let sb = DATA + 0x100;
                c.g[A0][0] = sx(sb);
                let handle = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(sb + SBP_HANDLE, handle);
            },
        );
    }

    /// StreamBuffer_QueueFromBig, the no-decoder path (+0x74 zero): returns 0
    /// after the frame is restored. The decoder path runs the big-file lookup
    /// (BigFile_FindEntry), which cannot be bounded on zeroed data (as in
    /// stream_buffer_queue_matches), so it is not exercised here.
    #[test]
    fn stream_buffer_queue_from_big_matches() {
        check(
            ee::s08::f_002aa398__generated,
            stream_buffer_queue_from_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a398, 0);
                c.pc = 0x002a_a398;
                let sb = DATA + 0x100;
                c.bus.write32(sb + SQB_DECODER, 0);
                c.bus.write32(sb + SQB_HANDLE, r.next_u32());
                c.bus.write32(sb + SQB_OFFSET, r.next_u32());
                c.g[A0][0] = sx(sb);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = (r.next_u32() % 0x4000) as u64;
            },
        );
    }

    /// StreamBuffer_SetStreamParam: the lock hooks are fakes, no SND streams
    /// (the setter's early return), a random handle and argument.
    #[test]
    fn stream_buffer_set_stream_param_matches() {
        covered(
            ee::s08::f_002aa448__generated,
            stream_buffer_set_stream_param,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a448, 2);
                c.pc = 0x002a_a448;
                snd_world(c, &mut r, false);
                let sb = DATA + 0x100;
                c.g[A0][0] = sx(sb);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(sb + SBP_HANDLE, r.next_u32());
            },
        );
    }

    /// StreamArray_CreateStreamEx: the allocator is the real pool allocator
    /// (machine_rt), the constructor runs on a zeroed player; each seed fills
    /// a different array slot, and the heap and size arguments are random.
    #[test]
    fn stream_array_create_stream_ex_matches() {
        check(
            ee::s08::f_002aa870__generated,
            stream_array_create_stream_ex,
            120,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2a_a870, 0);
                c.pc = 0x002a_a870;
                let this = DATA + 0x20_0000;
                let player = this + 0x1000;
                let buffers = this + 0x100;
                c.bus.write32(this, player);
                c.bus.write32(this + SCE_BUFFERS, buffers);
                c.bus.write32(buffers + 4 * (seed % 4), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.g[T1][0] = sx(r.next_u32());
                c.g[T2][0] = sx(r.next_u32());
            },
        );
    }

    /// StreamArray_PauseAll: counts -1..4, null and real buffers; each real
    /// buffer runs the real StreamBuffer_Pause (fake lock hooks, no streams).
    #[test]
    fn stream_array_pause_all_matches() {
        covered(
            ee::s08::f_002aac98__generated,
            stream_array_pause_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ac98, 3);
                c.pc = 0x002a_ac98;
                snd_world(c, &mut r, false);
                sap_fixture(c, &mut r, seed);
            },
        );
    }

    /// StreamArray_ResumeAll: the same fixture as PauseAll.
    #[test]
    fn stream_array_resume_all_matches() {
        covered(
            ee::s08::f_002aad08__generated,
            stream_array_resume_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ad08, 3);
                c.pc = 0x002a_ad08;
                snd_world(c, &mut r, false);
                sap_fixture(c, &mut r, seed);
            },
        );
    }

    /// The array fixture both PauseAll and ResumeAll tests use.
    pub(crate) fn sap_fixture(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        const ARRAY: u32 = DATA + 0x20_0000;
        const SLOTS: u32 = ARRAY + 0x100;
        const BUFS: u32 = ARRAY + 0x200;
        c.g[A0][0] = sx(ARRAY);
        let n = (seed % 6) as i32 - 1;
        c.bus.write32(ARRAY + SAP_COUNT, n as u32);
        c.bus.write32(ARRAY + SAP_BUFFERS, SLOTS);
        for i in 0..5u32 {
            let b = BUFS + 0x100 * i;
            c.bus
                .write32(SLOTS + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { b });
            c.bus.write32(b + SBP_HANDLE, r.next_u32());
        }
    }

    /// The stream array of every fixture (its list at `LIST`, the buffer
    /// every slot points at, one SND stream with one request).
    pub(crate) const ARRAY: u32 = DATA + 0x20_0000;

    pub(crate) const LIST: u32 = ARRAY + 0x40;

    pub(crate) const BUFFER: u32 = ARRAY + 0x400;

    pub(crate) const STREAM_2: u32 = ARRAY + 0x1000;

    pub(crate) const REQ_2: u32 = ARRAY + 0x2000;

    /// `StreamBuffer::stream` (the SND stream id) and `StreamBuffer::dec`
    /// (the decoder: null takes the short path of `QueueFromBig`).
    pub(crate) const BUFFER_STREAM: u32 = 0x8;

    pub(crate) const BUFFER_DEC: u32 = 0x74;

    /// The stream fixture of the `StreamBuffer_IsPlaying` test: the SND lock
    /// hooks are fakes, one live SND stream with one request (handle at
    /// `REQ + 0xc`), and every array slot holds `BUFFER` except `empty`.
    pub(crate) fn stream_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, empty: Option<u32>) {
        c.bus.write32(ARRAY + SAQ_BUFFERS, LIST);
        for i in 0..4u32 {
            let entry = if empty == Some(i) { 0 } else { BUFFER };
            c.bus.write32(LIST + 4 * i, entry);
        }
        let id = if seed % 3 == 0 {
            1 + r.next_u32() % 4
        } else {
            0
        };
        c.bus.write32(BUFFER + BUFFER_STREAM, id);
        c.bus.write32(BUFFER + BUFFER_DEC, 0);
        c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
        c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
        c.bus.write8(0x0051_0000 - 0x56d2, 1);
        c.bus.write32(0x0051_0000 - 0x5f60, STREAM_2);
        c.bus.write32(
            STREAM_2 + 0x124,
            if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
        );
        c.bus.write32(STREAM_2 + 0x11c, REQ_2);
        c.bus.write32(
            REQ_2 + 0xc,
            if seed % 6 < 3 {
                0x8000_0000 | r.next_u32()
            } else {
                r.next_u32() % 4
            },
        );
        c.bus.write16(STREAM_2 + 0x14, 0);
        c.bus
            .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
    }

    /// A request handle: the fixture's request (found) or a random one.
    pub(crate) fn handle(c: &mut Cpu, r: &mut Rng, seed: u32) -> u64 {
        let h = if seed % 4 == 0 {
            r.next_u32()
        } else {
            c.bus.read32(REQ_2 + 0xc)
        };
        h as i32 as i64 as u64
    }

    use crate::game::mem::memstd::tests::norm_miss;

    /// Our stream fixture (kept for the ported-libs tests below): plants
    /// the stream pointer at `SND_STREAMS` with handle-less-vs-backed
    /// (`+0x124`), request (`+0x11c`) and buffered (`+0x14`) steering.
    pub(crate) fn stream_fixture_2(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
        c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
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
        SELF
    }

    /// Our stream array of 4 slots over one live buffer.
    pub(crate) fn array_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        const ARRAY: u32 = DATA + 0x1000;
        const BUFS: u32 = DATA + 0x1100;
        let live = stream_fixture_2(c, r, seed);
        c.bus.write32(ARRAY + 8, BUFS);
        for i in 0..4u32 {
            c.bus
                .write32(BUFS + 4 * i, if (seed / 4 + i) & 1 == 0 { 0 } else { live });
        }
        ARRAY
    }

    /// Steer the request the status callees resolve.
    pub(crate) fn request_steering_2(c: &mut Cpu, r: &mut Rng, seed: u32) {
        const REQ: u32 = DATA + 0x20_0000 + 0x2000;
        c.bus.write32(
            REQ + 0xc,
            match seed % 3 {
                0 => 0,
                1 => 0x8000_0000 | r.next_u32(),
                _ => r.next_u32() & 0x7fff_ffff,
            },
        );
        c.bus.write8(REQ + 0x24, ((seed / 3) % 2) as u8 * 0x11);
        c.bus.write8(DATA + 0x5024, ((seed / 6) % 2) as u8 * 0x22);
    }

    /// Our queue-array fixture over one live buffer.
    pub(crate) fn queue_array_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        const ARRAY: u32 = DATA + 0x1000;
        const BUFS: u32 = DATA + 0x1100;
        let live = stream_fixture_2(c, r, seed);
        c.bus.write32(ARRAY + 8, BUFS);
        for i in 0..4u32 {
            c.bus
                .write32(BUFS + 4 * i, if (seed / 4 + i) & 1 == 0 { 0 } else { live });
        }
        c.bus.write32(live + 0x14, 0xaaaa_0000 + seed);
        c.bus.write32(
            live + 0x74,
            match seed % 3 {
                0 => 0,
                1 => 0xbbbb_0000 + seed,
                _ => 0,
            },
        );
        ARRAY
    }

    #[test]
    fn stream_array_queue_from_big_matches() {
        check(
            ee::s08::f_002aadc0__generated,
            stream_array_queue_from_big,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_adc0, 0);
                c.pc = 0x002a_adc0;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn stream_array_get_request_length_matches() {
        check(
            ee::s08::f_002aaec0__generated,
            stream_array_get_request_length,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_aec0, 0);
                c.pc = 0x002a_aec0;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = if seed % 7 == 0 { u64::MAX } else { idx as u64 };
            },
        );
    }

    #[test]
    fn stream_array_has_buffered_matches() {
        check(
            ee::s08::f_002aaf08__generated,
            stream_array_has_buffered,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_af08, 0);
                c.pc = 0x002a_af08;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = (r.next_u32() % 64) as u64;
            },
        );
    }

    #[test]
    fn stream_array_is_request_pending_matches() {
        check(
            ee::s08::f_002aaf40__generated,
            stream_array_is_request_pending,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_af40, 0);
                c.pc = 0x002a_af40;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn stream_array_is_request_done_matches() {
        check(
            ee::s08::f_002aaf78__generated,
            stream_array_is_request_done,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_af78, 4);
                c.pc = 0x002a_af78;
                stream_fixture(c, &mut r, seed, None);
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = handle(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_array_is_request_playing_matches() {
        check(
            ee::s08::f_002aafd0__generated,
            stream_array_is_request_playing,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_afd0, 4);
                c.pc = 0x002a_afd0;
                stream_fixture(c, &mut r, seed, None);
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = handle(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_array_clear_prebuffer_matches() {
        check(
            ee::s08::f_002ab060__generated,
            stream_array_clear_prebuffer,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b060, 0);
                c.pc = 0x002a_b060;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn stream_array_clear_prebuffer_handle_matches() {
        check(
            ee::s08::f_002ab098__generated,
            stream_array_clear_prebuffer_handle,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b098, 0);
                c.pc = 0x002a_b098;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = handle(c, &mut r, seed);
            },
        );
    }

    /// Gate word magic of the fakes below (written into the machine's RAM).
    pub(crate) const OC1045_MAGIC: u32 = 0x1045_c0de;

    /// Fake k keeps its gate, call counter and forced results in its own
    /// block of machine RAM (never in statics).
    pub(crate) const OC1045_BASE: u32 = DATA + 0x20_0000;

    pub(crate) fn oc1045_gate(k: u32) -> u32 {
        OC1045_BASE + k * 0x1000
    }

    /// Arm fake `k`: gate on, counter zero, forced results for its first calls.
    pub(crate) fn oc1045_arm(c: &mut Cpu, k: u32, results: &[u32]) {
        let g = oc1045_gate(k);
        c.bus.write32(g, OC1045_MAGIC);
        c.bus.write32(g + 4, 0);
        for (i, v) in results.iter().enumerate() {
            c.bus.write32(g + 0x100 + 4 * (i as u32), *v);
        }
    }

    /// Take fake `k`'s next call number and its forced result (0 after the
    /// armed results run out).
    pub(crate) fn oc1045_next(c: &mut Cpu, k: u32) -> u32 {
        let g = oc1045_gate(k);
        let n = c.bus.read32(g + 4);
        c.bus.write32(g + 4, n + 1);
        if n < 16 {
            c.bus.read32(g + 0x100 + 4 * n)
        } else {
            0
        }
    }

    /// A callee fake for an override slot. Gate off: the body the slot held
    /// when the fake was hooked. Gate on: a forced result in v0, return to ra.
    macro_rules! oc1045_fake {
        ($fake:ident, $prev:ident, $k:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(oc1045_gate($k)) != OC1045_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let v = oc1045_next(c, $k);
                c.g[V0][0] = sx(v);
                c.cycles += 4;
                c.pc = c.g[RA][0] as u32;
            }
        };
    }

    oc1045_fake!(
        oc1045_fake_stream_param,
        OC1045_PREV_STREAM_PARAM,
        0,
        ee::s08::f_002aa448__generated
    );

    oc1045_fake!(
        oc1045_fake_poll,
        OC1045_PREV_POLL,
        1,
        ee::s08::f_002abb80__generated
    );

    oc1045_fake!(
        oc1045_fake_pitch,
        OC1045_PREV_PITCH,
        2,
        ee::s08::f_002abe78__generated
    );

    oc1045_fake!(
        oc1045_fake_volume,
        OC1045_PREV_VOLUME,
        3,
        ee::s08::f_002abf60__generated
    );

    oc1045_fake!(
        oc1045_fake_seqpause,
        OC1045_PREV_SEQPAUSE,
        4,
        ee::s13::f_003b9d00__generated
    );

    oc1045_fake!(
        oc1045_fake_inst_dtor,
        OC1045_PREV_INST_DTOR,
        5,
        ee::s08::f_002aba18__generated
    );

    oc1045_fake!(
        oc1045_fake_base_dtor,
        OC1045_PREV_BASE_DTOR,
        6,
        ee::s08::f_002adac8__generated
    );

    oc1045_fake!(
        oc1045_fake_sm_dtor,
        OC1045_PREV_SM_DTOR,
        7,
        ee::s08::f_002adee8__generated
    );

    oc1045_fake!(
        oc1045_fake_table_a,
        OC1045_PREV_TABLE_A,
        8,
        ee::s10::f_00317e98__generated
    );

    oc1045_fake!(
        oc1045_fake_table_b,
        OC1045_PREV_TABLE_B,
        9,
        ee::s10::f_00317e50__generated
    );

    oc1045_fake!(
        oc1045_fake_find,
        OC1045_PREV_FIND,
        10,
        ee::s08::f_002ad550__generated
    );

    oc1045_fake!(
        oc1045_fake_fade,
        OC1045_PREV_FADE,
        11,
        ee::s08::f_002acb80__generated
    );

    /// Hook every fake once per process (gates are off unless a test arms them).
    pub(crate) fn oc1045_hook_all() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AA448),
            oc1045_fake_stream_param,
            &OC1045_PREV_STREAM_PARAM,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABB80),
            oc1045_fake_poll,
            &OC1045_PREV_POLL,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABE78),
            oc1045_fake_pitch,
            &OC1045_PREV_PITCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABF60),
            oc1045_fake_volume,
            &OC1045_PREV_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B9D00),
            oc1045_fake_seqpause,
            &OC1045_PREV_SEQPAUSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABA18),
            oc1045_fake_inst_dtor,
            &OC1045_PREV_INST_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADAC8),
            oc1045_fake_base_dtor,
            &OC1045_PREV_BASE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADEE8),
            oc1045_fake_sm_dtor,
            &OC1045_PREV_SM_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1045_fake_table_a,
            &OC1045_PREV_TABLE_A,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1045_fake_table_b,
            &OC1045_PREV_TABLE_B,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD550),
            oc1045_fake_find,
            &OC1045_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ACB80),
            oc1045_fake_fade,
            &OC1045_PREV_FADE,
        );
    }

    #[test]
    fn stream_array_set_stream_volume_matches() {
        check(
            ssx3_ee::s08::f_002ab0f0__generated,
            stream_array_set_stream_volume,
            300,
            &|c, seed| {
                let mut r = Rng(0x2ab0_f0 ^ seed as u64);
                scramble(c, &mut r);
                let array = DATA;
                let table = DATA + 0x100;
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(array + 8, table);
                for i in 0..4u32 {
                    let slot = if (seed >> i) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x300 + i * 0x20
                    };
                    c.bus.write32(table + 4 * i, slot);
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
            },
        );
    }

    #[test]
    fn stream_array_set_stream_param_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002ab1c8__generated,
            stream_array_set_stream_param,
            300,
            &|c, seed| {
                let mut r = Rng(0x2ab1_c8 ^ seed as u64);
                scramble(c, &mut r);
                let array = DATA;
                let table = DATA + 0x100;
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(array + 8, table);
                for i in 0..4u32 {
                    let slot = if (seed >> i) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x300 + i * 0x20
                    };
                    c.bus.write32(table + 4 * i, slot);
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
                oc1045_arm(c, 0, &[r.next_u32()]);
            },
        );
    }

    #[test]
    fn sound_request_set_bank_sample_2_matches() {
        check(
            ssx3_ee::s08::f_002abd38__generated,
            sound_request_set_bank_sample_2,
            30,
            &|c, seed| {
                let mut r = Rng(0x2abd_38 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn bank_monitor_instance_pause_matches() {
        oc1045_hook_all();
        check(
            ssx3_ee::s08::f_002acac8__generated,
            bank_monitor_instance_pause,
            300,
            &|c, seed| {
                let mut r = Rng(0x2aca_c8 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, if seed % 5 == 0 { 2 } else { 1 });
                c.bus.write32(this + 0x54, (seed % 4 == 1) as u32);
                c.bus.write32(this + 4, DATA + 0x2000);
                oc1045_arm(c, 1, &[(seed % 3 == 0) as u32]);
                oc1045_arm(c, 2, &[]);
                oc1045_arm(c, 3, &[]);
                oc1045_arm(c, 4, &[]);
            },
        );
    }
    // ---------- Stream coverage for the ported libs ----------
    // Our stream tests, renamed with `_live` (live-fixture stream), run
    // against main's bodies. Keepers pin paths or assert properties
    // main's tests do not cover.
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::require_paths;

    driven!(pause_live_gen, ee::s08::f_002aac98__generated);

    driven!(pause_live_hand, stream_array_pause_all);

    driven!(resume_live_gen, ee::s08::f_002aad08__generated);

    driven!(resume_live_hand, stream_array_resume_all);

    /// Gate of the `SNDSTRM_Create` fake for the ctor_2 test: with
    /// `CTOR2_MAGIC` at `CTOR2_CTRL` the call is logged by `fakes::fake_call`
    /// (results, scrambled temporaries, the chosen unwind) at a fixed 4 cycles.
    pub(crate) const CTOR2_CTRL: u32 = DATA + 0x1f60;

    pub(crate) const CTOR2_MAGIC: u32 = 0x5c7a_2c02;

    pub(crate) static CTOR2_PREV_CREATE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn ctor2_fake_create(c: &mut Cpu) {
        if c.bus.read32(CTOR2_CTRL) != CTOR2_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &CTOR2_PREV_CREATE,
                ee::s13::f_003b7818__generated,
            );
        }
        // A fixed cost, so each arm has one cycle total.
        let before = c.cycles;
        crate::difftest::fakes::fake_call(c, 0x003b_7818);
        c.cycles = before + 4;
    }

    /// The ctor_2 fixture. `P` (a1) holds the stream index at `+0x1c`, the
    /// volume-pointer array at `+0x3c`, the level array at `+0x38` and the
    /// 0x18-stride byte array at `+0x20`. Seeds pick the index, a null or
    /// real volume (0.25, 1, 1.5, -1) and a level (negative, 0..126,
    /// large), so the level times the volume lands below 0, inside 0..127 or
    /// at or above 127. One seed in three makes `SNDSTRM_Create` unwind.
    pub(crate) fn ctor2_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x002a_9c00, 1);
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B7818),
            ctor2_fake_create,
            &CTOR2_PREV_CREATE,
        );
        c.bus.write32(CTOR2_CTRL, CTOR2_MAGIC);
        const BUF: u32 = DATA + 0x100;
        const P: u32 = DATA + 0x400;
        const VOLS: u32 = DATA + 0x500;
        const FLOATS: u32 = DATA + 0x540;
        const LEVELS: u32 = DATA + 0x580;
        const BYTES: u32 = DATA + 0x600;
        crate::difftest::fill(c, &mut r, BUF, 0x10);
        crate::difftest::fill(c, &mut r, BYTES, 0x18);
        let index = seed % 3;
        let floats = [0x3e80_0000u32, 0x3f80_0000, 0x3fc0_0000, 0xbf80_0000];
        crate::difftest::put_words(c, FLOATS, &floats);
        for i in 0..3 {
            let vol = if (seed / 3 + i) % 5 == 4 {
                0
            } else {
                FLOATS + 4 * ((seed / 3 + i) % 4)
            };
            c.bus.write32(VOLS + 4 * i, vol);
            let level = match (seed / 15 + i) % 4 {
                0 => -(1 + (r.next_u32() % 200) as i32),
                1 => (r.next_u32() % 100) as i32,
                2 => 100 + (r.next_u32() % 60) as i32,
                _ => 0x1000 + (r.next_u32() % 0x1000) as i32,
            };
            c.bus.write32(LEVELS + 4 * i, level as u32);
        }
        c.bus.write32(P + 0x1c, index);
        c.bus.write32(P + 0x20, BYTES);
        c.bus.write32(P + 0x38, LEVELS);
        c.bus.write32(P + 0x3c, VOLS);
        c.g[A0][0] = BUF as u64;
        c.g[A1][0] = P as u64;
        c.g[A2][0] = 0x2222_0000 + seed as u64;
        c.g[A3][0] = 0x3333_0000 + seed as u64;
        c.g[T0][0] = 0x4444_0000 + seed as u64;
        c.g[T1][0] = 0x5555_0000 + seed as u64;
        c.pc = 0x002a_9c00;
    }

    /// `StreamBuffer_ctor_2`: every clamp of the level times the volume, the
    /// null-volume skip, and an unwinding `SNDSTRM_Create`.
    #[test]
    fn stream_buffer_ctor_2_live_matches() {
        check(
            ee::s08::f_002a9c00__generated,
            stream_buffer_ctor_2,
            90,
            &ctor2_fixture,
        );
        // Null volume 43; the three clamp arms (below 0, at or above 127, inside).
        require_paths(
            ee::s08::f_002a9c00__generated,
            90,
            &[43, 90, 101, 120],
            &ctor2_fixture,
        );
    }

    /// `StreamBuffer_QueueFromBig`: no entry reads 0; a live entry runs the
    /// real find-and-queue.
    #[test]
    fn stream_buffer_queue_from_big_live_matches() {
        check(
            ee::s08::f_002aa398__generated,
            stream_buffer_queue_from_big,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a398, 0);
                c.hooks.miss = norm_miss;
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.bus.write32(this + 0x14, 0xaaaa_0000 + seed);
                c.bus.write32(
                    this + 0x74,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xbbbb_0000 + seed,
                        _ => 0,
                    },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = [0u64, 0x100, 0xffff_ffff][seed as usize % 3];
                c.g[A2][0] = [0u64, 1, 0xabcd_0000][seed as usize % 3];
                c.g[A3][0] = (seed % 2) as u64;
                c.g[T0][0] = (seed % 3) as u64;
                c.pc = 0x002a_a398;
            },
        );
        require_paths(
            ee::s08::f_002aa398__generated,
            60,
            &[18, 291],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a398, 0);
                c.hooks.miss = norm_miss;
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.bus.write32(this + 0x14, 0xaaaa_0000 + seed);
                c.bus.write32(
                    this + 0x74,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xbbbb_0000 + seed,
                        _ => 0,
                    },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = [0u64, 0x100, 0xffff_ffff][seed as usize % 3];
                c.g[A2][0] = [0u64, 1, 0xabcd_0000][seed as usize % 3];
                c.g[A3][0] = (seed % 2) as u64;
                c.g[T0][0] = (seed % 3) as u64;
                c.pc = 0x002a_a398;
            },
        );
    }

    /// `StreamBuffer_GetRequestLength`: the summed words as a float when the
    /// status is 2, else the constant 2.
    #[test]
    fn stream_buffer_get_request_length_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_a260, 0);
            c.hooks.miss = norm_miss;
            snd_world(c, &mut r, true);
            // Plant the fixture stream after the world, so the
            // request steering below steers a live request.
            let this = stream_fixture_2(c, &mut r, seed);
            request_steering_2(c, &mut r, seed);
            c.g[A0][0] = this as u64;
            c.pc = 0x002a_a260;
        };
        check(
            ee::s08::f_002aa260__generated,
            stream_buffer_get_request_length,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002aa260__generated,
            100,
            &[200, 231, 237, 1338],
            &setup,
        );
    }

    /// `StreamBuffer_SetStreamParam`: the parameter crosses the SND lock to
    /// the real setter on a live or missing stream.
    #[test]
    fn stream_buffer_set_stream_param_live_matches() {
        check(
            ee::s08::f_002aa448__generated,
            stream_buffer_set_stream_param,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a448, 0);
                c.hooks.miss = norm_miss;
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = [0u64, 1, 0xffff_ffff][seed as usize % 3];
                c.g[A2][0] = 0xcc00_0000 + seed as u64;
                c.pc = 0x002a_a448;
            },
        );
        require_paths(
            ee::s08::f_002aa448__generated,
            60,
            &[72, 112],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a448, 0);
                c.hooks.miss = norm_miss;
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = [0u64, 1, 0xffff_ffff][seed as usize % 3];
                c.g[A2][0] = 0xcc00_0000 + seed as u64;
                c.pc = 0x002a_a448;
            },
        );
    }

    /// `StreamBuffer_GetRequestPosition`: status 2 reads the position word
    /// as a float, anything else reads 0.0.
    #[test]
    fn stream_buffer_get_request_position_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_a0c0, 0);
            c.hooks.miss = norm_miss;
            snd_world(c, &mut r, true);
            // Plant the fixture stream after the world, so the
            // request steering below steers a live request.
            let this = stream_fixture_2(c, &mut r, seed);
            request_steering_2(c, &mut r, seed);
            c.g[A0][0] = this as u64;
            c.pc = 0x002a_a0c0;
        };
        check(
            ee::s08::f_002aa0c0__generated,
            stream_buffer_get_request_position,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002aa0c0__generated,
            100,
            &[200, 231, 237, 1336],
            &setup,
        );
    }

    /// `StreamBuffer_Pause` and `StreamBuffer_Resume`: the pitch call runs
    /// for real on the fixture stream.
    #[test]
    fn stream_buffer_pause_resume_live_matches() {
        for (gen, hand, addr, salt) in [
            (
                ee::s08::f_002aa2b0__generated as fn(&mut Cpu),
                stream_buffer_pause as fn(&mut Cpu),
                0x002a_a2b0,
                0x002a_a2b0,
            ),
            (
                ee::s08::f_002aa2e8__generated as fn(&mut Cpu),
                stream_buffer_resume as fn(&mut Cpu),
                0x002a_a2e8,
                0x002a_a2e8,
            ),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt as u64, 0);
                c.hooks.miss = norm_miss;
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = this as u64;
                c.pc = addr;
            });
            require_paths(gen, 60, &[70, 112], &|c, seed| {
                let mut r = machine(c, seed, salt as u64, 0);
                c.hooks.miss = norm_miss;
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = this as u64;
                c.pc = addr;
            });
        }
    }

    /// `StreamBuffer_HasBuffered`: the buffered count crosses the steered
    /// threshold both ways.
    #[test]
    fn stream_buffer_has_buffered_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_a150, 0);
            c.hooks.miss = norm_miss;
            snd_world(c, &mut r, true);
            // Plant the fixture stream after the world.
            let this = stream_fixture_2(c, &mut r, seed);
            // Buffered halves for even seeds, so the live stream actually
            // buffers (else every threshold-1 seed reads 0).
            if seed % 2 == 0 {
                c.bus.write16(this + 0x1000 + 0x14, 0x20);
                c.bus.write16(this + 0x1000 + 0x18, 0x40);
            }
            c.g[A0][0] = this as u64;
            c.g[A1][0] = [0u64, 1, 0x7fff_ffff, 0xffff_ffff][seed as usize % 4];
            c.pc = 0x002a_a150;
        };
        check(
            ee::s08::f_002aa150__generated,
            stream_buffer_has_buffered,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002aa150__generated,
            100,
            &[95, 104, 109, 146, 157],
            &setup,
        );
    }

    /// `StreamBuffer_IsRequestPending`: status 0 and 2 both read
    /// not-pending (status 1 is unreached on this single-request fixture;
    /// see the branch table).
    #[test]
    fn stream_buffer_is_request_pending_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_a188, 0);
            snd_world(c, &mut r, true);
            // Plant the fixture stream after the world so the SND_STREAMS
            // pointer (0x50a0a0) is the fixture's live stream, not the
            // world's: the +0x124/+0x11c/+0x14 steering is actually read.
            let this = stream_fixture_2(c, &mut r, seed);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0xeeee_0000 + seed as u64;
            c.pc = 0x002a_a188;
        };
        check(
            ee::s08::f_002aa188__generated,
            stream_buffer_is_request_pending,
            100,
            &setup,
        );
        // The fixture stream must be live: handle-less seeds resolve no
        // request (status 3), while backed seeds resolve 0 or 2 (pre-fix
        // every seed reads the world's stream, whose node is always
        // found: only 0 or 2, never 3).
        const OUT: u32 = DATA + 0x30_0000;
        let mut statuses = std::collections::BTreeSet::new();
        for seed in 0..crate::difftest::seed_count(100) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            c.g[A1][0] = OUT as u64;
            c.g[A2][0] = 0;
            c.pc = 0x002a_a068;
            crate::difftest::run_generated(&mut c, stream_buffer_get_request_status);
            statuses.insert(c.bus.read32(OUT));
        }
        assert!(
            statuses.contains(&3),
            "request status never leaves 0/2: {statuses:?}"
        );
    }

    /// `StreamBuffer_ClearPrebuffer`: the real `GetStatus` reads the
    /// fixture stream, and the real `SetPrebuffer` clears (or refuses) the
    /// resulting handle, on both sides.
    #[test]
    fn stream_buffer_clear_prebuffer_live_matches() {
        check(
            ee::s08::f_002aa1e8__generated,
            stream_buffer_clear_prebuffer,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a1e8, 0);
                let this = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = this as u64;
                c.pc = 0x002a_a1e8;
            },
        );
    }

    /// `StreamBuffer_ClearPrebufferHandle`: handle 0 matches the fixture
    /// request (the prebuffer size is cleared), a negative handle matches
    /// nothing (`-8`).
    #[test]
    fn stream_buffer_clear_prebuffer_handle_live_matches() {
        check(
            ee::s08::f_002aa240__generated,
            stream_buffer_clear_prebuffer_handle,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a240, 0);
                snd_world(c, &mut r, true);
                c.g[A0][0] = 0xdead_beef;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    0xffff_ffff_ffff_ffff
                };
                c.pc = 0x002a_a240;
            },
        );
    }

    /// `StreamArray_CreateStreamEx`: the slot holds what the constructor
    /// returned on both sides.
    #[test]
    fn stream_array_create_stream_ex_live_matches() {
        check(
            ee::s08::f_002aa870__generated,
            stream_array_create_stream_ex,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_a870, 0);
                crate::difftest::fakes::machine_rt(c, seed, 0x002a_a870, 0);
                let array = array_fixture(c, &mut r, seed);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = 0xaaaa_0000 + seed as u64;
                c.g[A3][0] = 0xbbbb_0000 + seed as u64;
                c.g[T0][0] = 0xcccc_0000 + seed as u64;
                c.g[T1][0] = 0xdddd_0000 + seed as u64;
                c.g[T2][0] = 0xeeee_0000 + seed as u64;
                c.pc = 0x002a_a870;
            },
        );
        require_paths(ee::s08::f_002aa870__generated, 60, &[1522], &|c, seed| {
            let mut r = machine(c, seed, 0x002a_a870, 0);
            crate::difftest::fakes::machine_rt(c, seed, 0x002a_a870, 0);
            let array = array_fixture(c, &mut r, seed);
            c.g[A0][0] = array as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = 0xaaaa_0000 + seed as u64;
            c.g[A3][0] = 0xbbbb_0000 + seed as u64;
            c.g[T0][0] = 0xcccc_0000 + seed as u64;
            c.g[T1][0] = 0xdddd_0000 + seed as u64;
            c.g[T2][0] = 0xeeee_0000 + seed as u64;
            c.pc = 0x002a_a870;
        });
    }

    /// `StreamArray_PauseAll` and `StreamArray_ResumeAll`: counts of -1..4
    /// with null and live slots run the real pause/resume on each live one.
    #[test]
    fn stream_array_pause_resume_all_live_matches() {
        for (gen, hand, addr) in [
            (
                ee::s08::f_002aac98__generated as fn(&mut Cpu),
                stream_array_pause_all as fn(&mut Cpu),
                0x002a_ac98,
            ),
            (
                ee::s08::f_002aad08__generated as fn(&mut Cpu),
                stream_array_resume_all as fn(&mut Cpu),
                0x002a_ad08,
            ),
        ] {
            check(gen, hand, 100, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                const ARRAY: u32 = DATA + 0x1000;
                const BUFS: u32 = DATA + 0x1100;
                let live = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                let count = match seed % 6 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    _ => 1 + (seed % 4),
                };
                c.bus.write32(ARRAY + 4, count);
                c.bus.write32(ARRAY + 8, BUFS);
                for i in 0..4u32 {
                    c.bus
                        .write32(BUFS + 4 * i, if (seed / 6 + i) & 1 == 0 { 0 } else { live });
                }
                c.g[A0][0] = ARRAY as u64;
                c.pc = addr;
            });
            require_paths(
                gen,
                100,
                &[13, 24, 106, 139, 148, 157, 197, 272, 281],
                &|c, seed| {
                    let mut r = machine(c, seed, addr as u64, 0);
                    c.hooks.miss = norm_miss;
                    const ARRAY: u32 = DATA + 0x1000;
                    const BUFS: u32 = DATA + 0x1100;
                    let live = stream_fixture_2(c, &mut r, seed);
                    snd_world(c, &mut r, true);
                    let count = match seed % 6 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        _ => 1 + (seed % 4),
                    };
                    c.bus.write32(ARRAY + 4, count);
                    c.bus.write32(ARRAY + 8, BUFS);
                    for i in 0..4u32 {
                        c.bus
                            .write32(BUFS + 4 * i, if (seed / 6 + i) & 1 == 0 { 0 } else { live });
                    }
                    c.g[A0][0] = ARRAY as u64;
                    c.pc = addr;
                },
            );
        }
    }

    /// The pause/resume loops under delivered events. Resume's head
    /// (0x2aad30) is structured: the port must carry on, never resume
    /// there (the pause head is covered by
    /// `stream_array_pause_all_back_edge_carries_on`).
    #[test]
    fn stream_array_pause_resume_all_live_events() {
        for (gen, hand, addr, no_resume) in [
            (
                pause_live_gen as fn(&mut Cpu),
                pause_live_hand as fn(&mut Cpu),
                0x002a_ac98,
                None,
            ),
            (
                resume_live_gen as fn(&mut Cpu),
                resume_live_hand as fn(&mut Cpu),
                0x002a_ad08,
                Some(0x002a_ad30),
            ),
        ] {
            let resumes = check_events(gen, hand, 100, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                const ARRAY: u32 = DATA + 0x1000;
                const BUFS: u32 = DATA + 0x1100;
                let live = stream_fixture_2(c, &mut r, seed);
                snd_world(c, &mut r, true);
                let count = match seed % 6 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    _ => 1 + (seed % 4),
                };
                c.bus.write32(ARRAY + 4, count);
                c.bus.write32(ARRAY + 8, BUFS);
                for i in 0..4u32 {
                    c.bus
                        .write32(BUFS + 4 * i, if (seed / 6 + i) & 1 == 0 { 0 } else { live });
                }
                c.g[A0][0] = ARRAY as u64;
                schedule(c, &mut r, seed, addr)
            });
            if let Some(head) = no_resume {
                assert!(
                    resumes.get(&head).is_none(),
                    "unwound at structured head {head:#x}: {resumes:x?}"
                );
            }
        }
    }

    /// `StreamArray_IsRequestDone`: 1 for a negative handle, else 1 only
    /// when the status is 3 (unreached on this fixture; the status is 0 or
    /// 2 here).
    #[test]
    fn stream_array_is_request_done_live_matches() {
        check(
            ee::s08::f_002aaf78__generated,
            stream_array_is_request_done,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_af78, 0);
                c.hooks.miss = norm_miss;
                snd_world(c, &mut r, true);
                c.bus.write8(DATA + 0x5024, ((seed / 3) % 2) as u8 * 0x22);
                c.g[A0][0] = 0xdada_0000 + seed as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff - (seed % 100) as u64,
                    1 => 0,
                    _ => 1 + (seed % 100) as u64,
                };
                c.pc = 0x002a_af78;
            },
        );
        require_paths(
            ee::s08::f_002aaf78__generated,
            100,
            &[78, 90, 133, 1238],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_af78, 0);
                c.hooks.miss = norm_miss;
                snd_world(c, &mut r, true);
                c.bus.write8(DATA + 0x5024, ((seed / 3) % 2) as u8 * 0x22);
                c.g[A0][0] = 0xdada_0000 + seed as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff - (seed % 100) as u64,
                    1 => 0,
                    _ => 1 + (seed % 100) as u64,
                };
                c.pc = 0x002a_af78;
            },
        );
    }

    /// `StreamArray_IsRequestPlaying`: 1 for a non-negative handle with
    /// status 2, else 0.
    #[test]
    fn stream_array_is_request_playing_live_matches() {
        check(
            ee::s08::f_002aafd0__generated,
            stream_array_is_request_playing,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_afd0, 0);
                c.hooks.miss = norm_miss;
                snd_world(c, &mut r, true);
                c.bus.write8(DATA + 0x5024, ((seed / 3) % 2) as u8 * 0x22);
                c.g[A0][0] = 0xdbdb_0000 + seed as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff - (seed % 100) as u64,
                    1 => 0,
                    _ => 1 + (seed % 100) as u64,
                };
                c.pc = 0x002a_afd0;
            },
        );
        require_paths(
            ee::s08::f_002aafd0__generated,
            100,
            &[75, 87, 132, 1239],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_afd0, 0);
                c.hooks.miss = norm_miss;
                snd_world(c, &mut r, true);
                c.bus.write8(DATA + 0x5024, ((seed / 3) % 2) as u8 * 0x22);
                c.g[A0][0] = 0xdbdb_0000 + seed as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff - (seed % 100) as u64,
                    1 => 0,
                    _ => 1 + (seed % 100) as u64,
                };
                c.pc = 0x002a_afd0;
            },
        );
    }

    /// `StreamArray_QueueFromBig`: null slots read -1, zero-entry buffers
    /// read 0, live entries run the real queue.
    #[test]
    fn stream_array_queue_from_big_live_matches() {
        check(
            ee::s08::f_002aadc0__generated,
            stream_array_queue_from_big,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_adc0, 0);
                let array = queue_array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = [0u64, 0x100, 0xffff_ffff][seed as usize % 3];
                c.g[A3][0] = [0u64, 1, 0xabcd_0000][seed as usize % 3];
                c.g[T0][0] = (seed % 2) as u64;
                c.pc = 0x002a_adc0;
            },
        );
        require_paths(
            ee::s08::f_002aadc0__generated,
            60,
            &[14, 33, 306],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_adc0, 0);
                let array = queue_array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = [0u64, 0x100, 0xffff_ffff][seed as usize % 3];
                c.g[A3][0] = [0u64, 1, 0xabcd_0000][seed as usize % 3];
                c.g[T0][0] = (seed % 2) as u64;
                c.pc = 0x002a_adc0;
            },
        );
    }

    /// `StreamArray_GetRequestLength`: a negative index reads 0.0 without
    /// touching the table; null slots do the same after the lookup.
    #[test]
    fn stream_array_get_request_length_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_aec0, 0);
            snd_world(c, &mut r, true);
            // Plant the fixture array after the world.
            let array = queue_array_fixture(c, &mut r, seed);
            c.g[A0][0] = array as u64;
            c.g[A1][0] = match seed % 5 {
                0 => 0xffff_ffff_ffff_ffff,
                _ => (seed % 4) as u64,
            };
            c.g[A2][0] = 0xee00_0000 + seed as u64;
            c.pc = 0x002a_aec0;
        };
        check(
            ee::s08::f_002aaec0__generated,
            stream_array_get_request_length,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002aaec0__generated,
            100,
            &[8, 14, 217, 225, 250, 256],
            &setup,
        );
    }

    /// `StreamArray_HasBuffered`: null slots read 0; live slots run the
    /// real buffered check against the fixture stream with a steered
    /// threshold.
    #[test]
    fn stream_array_has_buffered_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_af08, 0);
            snd_world(c, &mut r, true);
            // Plant the fixture array after the world.
            let array = array_fixture(c, &mut r, seed);
            // Buffered halves for even seeds (stream at SELF + 0x1000).
            if seed % 2 == 0 {
                c.bus.write16(DATA + 0x20_1000 + 0x14, 0x20);
                c.bus.write16(DATA + 0x20_1000 + 0x18, 0x40);
            }
            c.g[A0][0] = array as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = [0, 1, 0x7fff_ffff, 0xffff_ffff][seed as usize % 4];
            c.pc = 0x002a_af08;
        };
        check(
            ee::s08::f_002aaf08__generated,
            stream_array_has_buffered,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002aaf08__generated,
            100,
            &[11, 112, 122, 128, 170, 176],
            &setup,
        );
    }

    /// `StreamArray_IsRequestPending`: null slots read 0; live slots run
    /// the real pending check.
    #[test]
    fn stream_array_is_request_pending_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_af40, 0);
            snd_world(c, &mut r, true);
            // Plant the fixture array after the world.
            let array = array_fixture(c, &mut r, seed);
            c.g[A0][0] = array as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = 0xaaaa_0000 + seed as u64;
            c.pc = 0x002a_af40;
        };
        check(
            ee::s08::f_002aaf40__generated,
            stream_array_is_request_pending,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002aaf40__generated,
            100,
            &[11, 213, 221, 246, 252],
            &setup,
        );
    }

    /// `StreamArray_ClearPrebuffer`: null slots are a no-op; live slots run
    /// the real clear.
    #[test]
    fn stream_array_clear_prebuffer_live_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x002a_ab60, 0);
            snd_world(c, &mut r, true);
            // Plant the fixture array after the world.
            let array = array_fixture(c, &mut r, seed);
            c.g[A0][0] = array as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = 0xbbbb_0000 + seed as u64;
            c.pc = 0x002a_b060;
        };
        check(
            ee::s08::f_002ab060__generated,
            stream_array_clear_prebuffer,
            100,
            &setup,
        );
        require_paths(
            ee::s08::f_002ab060__generated,
            100,
            &[10, 204, 212, 220],
            &setup,
        );
    }

    /// `StreamArray_ClearPrebufferHandle`: the handle either matches the
    /// fixture request (cleared) or matches nothing (`-8`), or the slot is
    /// null (0 without calling).
    #[test]
    fn stream_array_clear_prebuffer_handle_live_matches() {
        check(
            ee::s08::f_002ab098__generated,
            stream_array_clear_prebuffer_handle,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_ab98, 0);
                let array = array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    0xffff_ffff_ffff_ffff
                };
                c.pc = 0x002a_b098;
            },
        );
        require_paths(
            ee::s08::f_002ab098__generated,
            100,
            &[10, 96, 119],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_ab98, 0);
                let array = array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    0xffff_ffff_ffff_ffff
                };
                c.pc = 0x002a_b098;
            },
        );
    }

    /// `StreamArray_SetStreamVolume`: volumes whose low byte is 0 or not
    /// land at the buffer's volume slot; null slots store nothing.
    #[test]
    fn stream_array_set_stream_volume_live_matches() {
        check(
            ee::s08::f_002ab0f0__generated,
            stream_array_set_stream_volume,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_abf0, 0);
                let array = array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = [0u64, 1, 0x100, 0x1ab][seed as usize % 4];
                c.pc = 0x002a_b0f0;
            },
        );
        require_paths(
            ee::s08::f_002ab0f0__generated,
            100,
            &[10, 15],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_abf0, 0);
                let array = array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, true);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = [0u64, 1, 0x100, 0x1ab][seed as usize % 4];
                c.pc = 0x002a_b0f0;
            },
        );
    }

    /// `StreamArray_SetStreamParam`: the parameter crosses the SND lock to
    /// the real setter, or the slot is null (-1 without calling).
    #[test]
    fn stream_array_set_stream_param_live_matches() {
        check(
            ee::s08::f_002ab1c8__generated,
            stream_array_set_stream_param,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_ab1c, 0);
                let array = array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, seed % 3 != 2);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = [0u64, 1, 0xffff_ffff][seed as usize % 3];
                c.pc = 0x002a_b1c8;
            },
        );
        require_paths(
            ee::s08::f_002ab1c8__generated,
            100,
            &[11, 86, 89, 130],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_ab1c, 0);
                let array = array_fixture(c, &mut r, seed);
                snd_world(c, &mut r, seed % 3 != 2);
                c.g[A0][0] = array as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = [0u64, 1, 0xffff_ffff][seed as usize % 3];
                c.pc = 0x002a_b1c8;
            },
        );
    }
}
