//! `src/lib/snd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::Resume;
pub(crate) use crate::game::ai::ai::at as at_2;
use crate::game::lib::snd;
pub(crate) use crate::game::misc::SND_EVENT_TABLE;
use crate::game::visualfx::boardwakefx as bw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_5910, snd_add_tick_callback, "SND_AddTickCallback"),
    (0x003b_5a28, snd_add_event_callback, "SND_AddEventCallback"),
    (0x003b_5ba0, snd_mem_clamp_block, "SND_MemClampBlock"),
    (0x003b_5bd0, snd_mem_init_heap, "SND_MemInitHeap"),
    (0x003b_c5c8, snd_random, "SND_Random"),
    (0x003b_c930, sndenc_install_dts, "SNDENC_InstallDts"),
    (
        0x003c_0c18,
        snd_get_group_voice_range,
        "SND_GetGroupVoiceRange",
    ),
    (0x003c_2438, snd_tick_stub, "SND_TickStub"),
    (
        0x003c_25e0,
        snd_channel_apply_pitch,
        "SND_ChannelApplyPitch",
    ),
    (
        0x003c_2a50,
        snd_channel_apply_volume,
        "SND_ChannelApplyVolume",
    ),
    (0x003c_3500, snd_set_spu_reverb_mode, "SND_SetSpuReverbMode"),
    (0x003c_36c8, snd_set_spu_fx_volume, "SND_SetSpuFxVolume"),
    (0x003c_4af8, snd_spu_mem_clamp_block, "SND_SpuMemClampBlock"),
    (0x003c_4e50, sndi_spu_mem_set_base, "SNDI_SpuMemSetBase"),
    (
        0x003c_6ce0,
        sndstrm_stop_slot_unsupported,
        "SNDSTRM_StopSlotUnsupported",
    ),
    (
        0x003c_b538,
        snd_dsp_resample_set_rate,
        "SND_DspResample_SetRate",
    ),
    (
        0x003c_b540,
        snd_dsp_resample_process,
        "SND_DspResample_Process",
    ),
    (0x003c_ca08, snd_eaxa_decode_blocks, "SND_EaxaDecodeBlocks"),
    (0x003c_d190, snd_eaxa_set_history, "SND_EaxaSetHistory"),
    (
        0x003c_d2b0,
        snd_utk_decode_excitation,
        "SND_UtkDecodeExcitation",
    ),
    (0x003c_e0d8, snd_utk_set_mode, "SND_UtkSetMode"),
    (0x003c_e410, snd_utk_lpc_filter, "SND_UtkLpcFilter"),
    (0x003c_d878, snd_utk_decode_frame, "SND_UtkDecodeFrame"),
    (0x003c_de68, snd_utk_decode, "SND_UtkDecode"),
    (
        0x003b_7c40,
        sndstrm_get_request_status,
        "SNDSTRM_GetRequestStatus",
    ),
    (0x003b_a888, snd_cents_to_pitch, "SND_CentsToPitch"),
    (0x003c_6380, snd_umul32x32, "SND_UMul32x32"),
    (0x003c_85d0, snd_ee_mix_block, "SND_EeMixBlock"),
    (
        0x003c_9520,
        snd_dec_utk_decode_loop,
        "SND_DecUtk_DecodeLoop",
    ),
    (
        0x003c_a460,
        snd_dec_eaxa_decode_stream,
        "SND_DecEaxa_DecodeStream",
    ),
    (0x003c_cf90, snd_eaxa_decode, "SND_EaxaDecode"),
    (0x003b_55f0, snd_tick, "SND_Tick"),
    (0x003b_7b48, sndstrm_get_status, "SNDSTRM_GetStatus"),
    (
        0x003c_1298,
        snd_release_finished_voices,
        "SND_ReleaseFinishedVoices",
    ),
    (0x003c_38e0, snd_channel_apply_pan, "SND_ChannelApplyPan"),
    (0x003c_6de0, sndstrm_pop_block, "SNDSTRM_PopBlock"),
    (
        0x003b_6b10,
        sndstrm_consume_callback,
        "SNDSTRM_ConsumeCallback",
    ),
    (0x003b_71d8, sndstrm_service, "SNDSTRM_Service"),
    (0x003b_85f0, snd_set_volume, "SND_SetVolume"),
    (0x003c_64a8, snd_udiv64by32, "SND_UDiv64by32"),
    (0x003c_7010, sndstrm_release_bytes, "SNDSTRM_ReleaseBytes"),
    (0x003c_a6a8, snd_dsp_clip16_process, "SND_DspClip16_Process"),
    (
        0x003b_70f8,
        sndstrm_is_prebuffering,
        "SNDSTRM_IsPrebuffering",
    ),
    (0x003b_7a70, sndstrm_find_request, "SNDSTRM_FindRequest"),
    (0x003c_07a8, snd_restore_vu0_flags, "SND_RestoreVu0Flags"),
    (
        0x003c_08f8,
        snd_restore_vu0_context,
        "SND_RestoreVu0Context",
    ),
    (0x003c_7080, sndstrm_run_callbacks, "SNDSTRM_RunCallbacks"),
    (0x003c_7388, snd_parse_next_tag, "SND_ParseNextTag"),
    (0x003c_7828, snd_mix_add_ramp16, "SND_MixAddRamp16"),
    (0x003c_8968, snd_ee_mix, "SND_EeMix"),
    (
        0x003b_54e0,
        snd_run_change_callbacks,
        "SND_RunChangeCallbacks",
    ),
    (0x003b_6ea8, sndstrm_parse_data, "SNDSTRM_ParseData"),
    (0x003b_7e70, sndstrm_set_volume, "SNDSTRM_SetVolume"),
    (0x003b_7f18, sndstrm_get_volume, "SNDSTRM_GetVolume"),
    (0x003b_86c0, snd_get_volume, "SND_GetVolume"),
    (0x003b_a078, snd_is_voice_reserved, "SND_IsVoiceReserved"),
    (
        0x003c_60e0,
        snd_run_service_callbacks,
        "SND_RunServiceCallbacks",
    ),
    (0x003c_6ab8, sndstrm_push_block, "SNDSTRM_PushBlock"),
    (
        0x003c_6c28,
        sndstrm_get_free_blocks,
        "SNDSTRM_GetFreeBlocks",
    ),
    (0x003c_72f8, snd_read_big_endian, "SND_ReadBigEndian"),
    (0x003c_7ab0, snd_ee_voice_mix_ramp, "SND_EeVoiceMixRamp"),
    (0x003b_c4c0, snd_get_pan_gains, "SND_GetPanGains"),
    (
        0x003c_3ee0,
        snd_channel_start_voices,
        "SND_ChannelStartVoices",
    ),
    (
        0x003c_6c98,
        sndstrm_get_buffered_bytes,
        "SNDSTRM_GetBufferedBytes",
    ),
    (
        0x003b_5450,
        snd_timer_service_callback,
        "SND_TimerServiceCallback",
    ),
    (0x003b_7080, sndstrm_handle_chunk, "SNDSTRM_HandleChunk"),
    (
        0x003c_2440,
        snd_channel_stop_voices,
        "SND_ChannelStopVoices",
    ),
    (0x003c_27e0, snd_channel_apply_send, "SND_ChannelApplySend"),
    (
        0x003c_2ec8,
        snd_channel_apply_fx_enable,
        "SND_ChannelApplyFxEnable",
    ),
    (0x003c_8560, snd_ee_voice_set_volume, "SND_EeVoiceSetVolume"),
    (0x003c_cf60, snd_eaxa_set_input, "SND_EaxaSetInput"),
    (0x003d_85c8, spch_match_conditions, "SPCH_MatchConditions"),
    (0x003b_5f98, snd_fire_events, "SND_FireEvents"),
    (0x003b_6820, snd_play, "SND_Play"),
    (
        0x003b_6ad8,
        sndstrm_release_chunk_callback,
        "SNDSTRM_ReleaseChunkCallback",
    ),
    (0x003b_8218, snd_get_fx_params, "SND_GetFxParams"),
    (0x003b_8530, snd_fade_volume, "SND_FadeVolume"),
    (0x003b_a740, snd_channel_calc_send, "SND_ChannelCalcSend"),
    (
        0x003b_b820,
        snd_parse_stream_header,
        "SND_ParseStreamHeader",
    ),
    (0x003c_6158, snd_next_voice_group, "SND_NextVoiceGroup"),
    (0x003c_d6f0, snd_utk_parse_header, "SND_UtkParseHeader"),
    (0x003d_8780, spch_is_sample_allowed, "SPCH_IsSampleAllowed"),
    (0x003d_b4d0, spch_rand_next, "SPCH_RandNext"),
    (0x003d_b5c0, spch_random_no_repeat, "SPCH_RandomNoRepeat"),
    (0x003b_76e8, sndstrm_queue_request, "SNDSTRM_QueueRequest"),
    (0x003b_b0c8, snd_next_handle_serial, "SND_NextHandleSerial"),
    (0x003c_72a0, snd_random_range, "SND_RandomRange"),
    (0x003d_6e30, spch_history_find, "SPCH_HistoryFind"),
    (0x003b_c570, snd_midi_to_volume, "SND_MidiToVolume"),
    (0x003c_8158, snd_ee_voice_start, "SND_EeVoiceStart"),
    (0x003c_8468, snd_ee_voice_stop, "SND_EeVoiceStop"),
    (0x003c_8a88, snd_ee_voice_set_pitch, "SND_EeVoiceSetPitch"),
    (0x003c_dde8, snd_utk_set_input, "SND_UtkSetInput"),
    (0x003b_6948, sndstrm_calc_data_rate, "SNDSTRM_CalcDataRate"),
    (0x003b_6a70, sndstrm_retire_request, "SNDSTRM_RetireRequest"),
    (
        0x003c_3010,
        snd_channel_apply_time_stretch,
        "SND_ChannelApplyTimeStretch",
    ),
    (
        0x003c_3178,
        snd_channel_apply_lowpass,
        "SND_ChannelApplyLowpass",
    ),
    (0x003c_7218, snd_init_patch_params, "SND_InitPatchParams"),
    (
        0x003c_8c30,
        snd_ee_voice_set_highpass,
        "SND_EeVoiceSetHighpass",
    ),
    (
        0x003c_8d00,
        snd_ee_voice_set_lowpass,
        "SND_EeVoiceSetLowpass",
    ),
    (0x003c_9420, snd_dec_utk_init_mem, "SND_DecUtk_InitMem"),
    (0x003d_9ed0, spch_check_event_masks, "SPCH_CheckEventMasks"),
    (0x003c_6760, sndstrm_start_channel, "SNDSTRM_StartChannel"),
    (
        0x003d_9830,
        spch_notify_play_callback,
        "SPCH_NotifyPlayCallback",
    ),
    (
        0x003d_b838,
        spch_decode_sample_entry,
        "SPCH_DecodeSampleEntry",
    ),
];

/// Fields of the driver state block.
pub(crate) mod state {

    /// `u16`: sample rate the EE voices' pitch is relative to.
    pub const EE_RATE: u32 = 0x2a;

    /// `u16`: sample rate the IOP DSP voices' pitch is relative to.
    pub const IOP_RATE: u32 = 0x2c;

    /// `u8`: first EE voice number (subtracted, after `VOICE_BASE`).
    pub const EE_VOICE_BASE: u32 = 0x2f;

    /// `u8`: first voice number of the IOP block.
    pub const VOICE_BASE: u32 = 0x30;

    /// `u8`: speaker count (4 = quad).
    pub const SPEAKERS: u32 = 0x47;

    /// `u8`: auxiliary sends (outputs = speakers + sends).
    pub const AUX_SENDS: u32 = 0x48;

    /// `u8`: SPU output mode; 2 drives a second voice block (+0x18).
    pub const OUTPUT_MODE: u32 = 0x49;

    /// `SndChannel*`: the channel array.
    pub const CHANNELS: u32 = 0x1f0;
}

/// Fields of the second global block.
pub(crate) mod sync {

    /// `u8*`: the voice parameter blocks (SPU: 8 bytes per voice; IOP
    /// DSP: 16 bytes per voice at +0x1a4).
    pub const VOICE_PARAMS: u32 = 0xcb0;

    /// `float[voice][8]`: per-voice output gains (32 bytes per voice).
    pub const GAINS: u32 = 0xe98;
}

/// `SndChannel` (0x8c bytes).
pub(crate) mod chan {

    pub const SIZE: u32 = 0x8c;

    /// `s16[]`: the channel's voice numbers.
    pub const VOICES: u32 = 0x04;

    /// `u16`: pitch (4.12 relative to the output rate).
    pub const PITCH: u32 = 0x20;

    /// `u8`: number of voices.
    pub const VOICE_COUNT: u32 = 0x23;

    /// `u16`: flags.
    pub const FLAGS: u32 = 0x24;

    /// `float`: final volume.
    pub const VOLUME: u32 = 0x48;

    /// `s8`: pan.
    pub const PAN: u32 = 0x5f;

    /// `u16`: pitch scale.
    pub const PITCH_SCALE: u32 = 0x8a;

    /// Flag: the voices are SPU voices.
    pub const SPU: u64 = 0x8;

    /// Flag: the voices live in the IOP DSP block.
    pub const IOP: u64 = 0x100;
}

/// Addresses of `SND_UtkDecodeExcitation`.
pub(crate) mod exc {

    pub const START: u32 = 0x003c_d2b0;

    pub const END: u32 = 0x003c_d514;

    /// Loop head: decode the next symbol (v0 the peeked byte).
    pub const SYMBOL: u32 = 0x003c_d310;

    /// Loop head: write a run of zeros.
    pub const ZERO_RUN: u32 = 0x003c_d3d0;

    /// Loop head: one more bit of an escape value's unary magnitude.
    pub const ESCAPE: u32 = 0x003c_d400;

    /// Loop head: one pulse-mode sample.
    pub const PULSE: u32 = 0x003c_d488;

    /// `u8[state][256]`: the Huffman symbol of a peeked byte.
    pub const SYMBOLS: u32 = 0x0044_e9e8;

    /// `{ int next_state; int bits; float value; }[]` per symbol.
    pub const CODES: u32 = 0x0044_ebe8;

    /// `&CODES[0].value`.
    pub const CODE_VALUES: u32 = 0x0044_ebf0;

    /// Samples per excitation frame (the loops stop at 108).
    pub const SAMPLES: u64 = 108;
}

/// `EaxaState` fields.
pub(crate) mod eaxa {

    /// `int`: samples left to decode.
    pub const LEFT: u32 = 0x0;

    /// `float`: the newest decoded sample.
    pub const HIST1: u32 = 0x4;

    /// `float`: the one before.
    pub const HIST2: u32 = 0x8;

    /// `u8*`: the input.
    pub const INPUT: u32 = 0xc;

    /// `float*`: the output.
    pub const OUTPUT: u32 = 0x10;

    /// Decoded samples still waiting in the block buffer (stored negated
    /// after a partial block).
    pub const PENDING: u32 = 0x4;

    /// Samples left in the stream.
    pub const REMAINING: u32 = 0x8;

    /// Read pointer into the block buffer (floats).
    pub const READ: u32 = 0x10;

    /// One decoded block (28 floats).
    pub const BUF: u32 = 0x24;

    /// The sample count that `SND_EaxaDecodeBlocks` takes (by address).
    pub const COUNT: u32 = 0x94;

    /// Output pointer (floats).
    pub const OUT: u32 = 0xa4;

    /// Samples per EAXA block.
    pub const BLOCK: u64 = 28;
}

// ---------------------------------------------------------------------------
// Sound library (SND_*).

/// `void SND_AddTickCallback(void (*cb)())` (0x3b5910): append `cb` to the
/// per-tick callback table at 0x50a8e8 (only when the subsystem is enabled).
pub fn snd_add_tick_callback(c: &mut Cpu) {
    'b_3b5940: {
        c.cycles += 5;
        c.g[V1][0] = 0x51_0000;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x174).lb(c);
        if c.g[V0][0] == 0 {
            break 'b_3b5940;
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x177).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x177).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[A1][0] as u32).off(0x177).sb(c, c.g[V1][0]);
        break 'b_3b5940;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_AddEventCallback(void (*cb)())` (0x3b5a28): append `cb` to the
/// per-event callback table at 0x50a8e8.
pub fn snd_add_event_callback(c: &mut Cpu) {
    'b_3b5a58: {
        c.cycles += 5;
        c.g[V1][0] = 0x51_0000;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x174).lb(c);
        if c.g[V0][0] == 0 {
            break 'b_3b5a58;
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x179).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x179).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[A1][0] as u32).off(0x179).sb(c, c.g[V1][0]);
        break 'b_3b5a58;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_MemClampBlock(uint* addr, uint* size)` (0x3b5ba0): clamp `size`
/// so that `addr + size` does not pass the heap end.
pub fn snd_mem_clamp_block(c: &mut Cpu) {
    'b_3b5bc8: {
        c.cycles += 9;
        c.g[V0][0] = 0x51_0000;
        c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x54e4));
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if t {
            break 'b_3b5bc8;
        }
        c.cycles += 1;
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        break 'b_3b5bc8;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_MemInitHeap(uintptr_t base, uint size)` (0x3b5bd0): carve a block
/// header out of `[base, base+size)` and link it into the heap list.
pub fn snd_mem_init_heap(c: &mut Cpu) {
    c.cycles += 28;
    let base = abi::arg(c, 0);
    let size = abi::arg(c, 1);
    c.g[A2][0] = 0x51_0000;
    c.g[V0][0] = sx(base.wrapping_add(size));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5718));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
    c.sw((c.g[A2][0] as u32).wrapping_add(0x234), base as u64);
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.sw(base.wrapping_add(8), size as u64);
    c.g[A3][0] |= 0xfff0;
    c.g[A0][0] = sx(base.wrapping_add(0x18));
    c.g[A1][0] = sx(size.wrapping_sub(0xf));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20));
    c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[V0][0]);
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T0][0]);
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    abi::ret(c);
}

/// `uint SND_Random(void)` (0x3bc5c8): xorshift-style PRNG state update at
/// 0x50ac50; returns the new state.
pub fn snd_random(c: &mut Cpu) {
    'b_3bc600: {
        'b_3bc5f8: {
            c.cycles += 9;
            c.g[V1][0] = 0x51_0000;
            c.g[T2][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x53b0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
            let t = c.g[V0][0] != 0;
            c.g[T1][0] = 0;
            if t {
                break 'b_3bc5f8;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
            let t = c.g[V0][0] == 0;
            c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x53b0));
            if t {
                break 'b_3bc600;
            }
            break 'b_3bc5f8;
        }
        c.cycles += 2;
        c.g[T1][0] = 0x1;
        c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x53b0));
        break 'b_3bc600;
    }
    'b_3bc6ac: {
        c.cycles += 24;
        c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x53b0));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = (c.g[A3][0] < c.g[V1][0]) as u64;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[A3][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
        c.g[T1][0] = (c.g[A3][0] < c.g[A1][0]) as u64;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[T1][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        let t = c.g[V1][0] != 0;
        c.sw((c.g[T2][0] as u32).wrapping_sub(0x53b0), c.g[A3][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.sw((c.g[T2][0] as u32).wrapping_sub(0x53b0), c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        break 'b_3bc6ac;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

/// `void SNDENC_InstallDts(void)` (0x3bc930): publish the three default DTS
/// callbacks into the SND globals.
pub fn sndenc_install_dts(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x3c_0000;
    c.g[V1][0] = 0x3c_0000;
    c.g[A0][0] = 0x3c_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1360));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x12d0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf10));
    c.g[A1][0] = 0x51_0000;
    c.g[A2][0] = 0x51_0000;
    c.g[A3][0] = 0x51_0000;
    c.sw((c.g[A1][0] as u32).wrapping_sub(0x37c0), c.g[V0][0]);
    c.sw((c.g[A2][0] as u32).wrapping_sub(0x37bc), c.g[V1][0]);
    c.sw((c.g[A3][0] as u32).wrapping_sub(0x37b8), c.g[A0][0]);
    abi::ret(c);
}

/// `void SND_GetGroupVoiceRange(int group, int* lo, int* hi)` (0x3c0c18):
/// report the voice range of a group, or the global range for group 0.
pub fn snd_get_group_voice_range(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & 0x8;
    let t8 = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[A0][0] & 0x100;
    if !t8 {
        c.cycles += 5;
        c.sw(abi::arg(c, 1), 0);
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x56e8));
        c.sw(abi::arg(c, 2), c.g[V1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let t100 = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[A0][0] & 0x4;
    if !t100 {
        c.cycles += 9;
        c.g[V1][0] = 0x51_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lbu(c);
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x2f).lbu(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(abi::arg(c, 2), c.g[V0][0]);
        abi::ret(c);
        return;
    }
    'b_3c0c88: {
        c.cycles += 2;
        let t4 = c.g[V0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if t4 {
            break 'b_3c0c88;
        }
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x2f).lbu(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x17e).lh(c);
        c.sw(abi::arg(c, 2), c.g[A0][0]);
        break 'b_3c0c88;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_SetSpuReverbMode(int mode)` (0x3c3500): map an API reverb mode to
/// the SPU's internal reverb preset and program it.
pub fn snd_set_spu_reverb_mode(c: &mut Cpu) {
    'b_3c35a0: {
        'b_3c3518: {
            c.cycles += 4;
            c.g[A3][0] = c.g[A0][0];
            c.g[V0][0] = (c.g[A3][0] < 0x2u64) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = 0x5;
            if t {
                break 'b_3c3518;
            }
            c.cycles += 2;
            c.g[A3][0] = 0;
            break 'b_3c35a0;
        }
        'b_3c3528: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0xa;
            if t {
                break 'b_3c3528;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x1;
            break 'b_3c35a0;
        }
        'b_3c3538: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x14;
            if t {
                break 'b_3c3538;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x2;
            break 'b_3c35a0;
        }
        'b_3c3548: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x1e;
            if t {
                break 'b_3c3548;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x3;
            break 'b_3c35a0;
        }
        'b_3c3558: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x28;
            if t {
                break 'b_3c3558;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x4;
            break 'b_3c35a0;
        }
        'b_3c3568: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x32;
            if t {
                break 'b_3c3568;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x5;
            break 'b_3c35a0;
        }
        'b_3c3578: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x64;
            if t {
                break 'b_3c3578;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x6;
            break 'b_3c35a0;
        }
        'b_3c3588: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x6e;
            if t {
                break 'b_3c3588;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x9;
            break 'b_3c35a0;
        }
        'b_3c3598: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V1][0] = 0x7;
            if t {
                break 'b_3c3598;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x8;
            break 'b_3c35a0;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A3][0] ^ 0x78;
        if c.g[V0][0] == 0 {
            c.g[A3][0] = c.g[V1][0];
        }
        break 'b_3c35a0;
    }
    c.cycles += 22;
    c.g[V0][0] = 0x45_0000;
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22b8));
    c.g[A2][0] = 0x51_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = 0x20_0000;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5300));
    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
    c.g[V1][0] = 0x51_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 6) as u32);
    c.sh((c.g[A2][0] as u32).wrapping_add(0x1a9e), c.g[A0][0]);
    c.sb((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x224).lbu(c);
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
    c.sb((c.g[A0][0] as u32).wrapping_add(0x11), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x228).lbu(c);
    c.sb((c.g[A1][0] as u32).wrapping_add(0x12), c.g[V0][0]);
    abi::ret(c);
}

/// `void SND_SetSpuFxVolume(int flags)` (0x3c36c8): scale and store the SPU
/// effect volume (only when the global volume pointer is set and the effect
/// bit is present).
pub fn snd_set_spu_fx_volume(c: &mut Cpu) {
    'b_3c3720: {
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[A1][0] & 0x8;
        if t {
            break 'b_3c3720;
        }
        c.cycles += 2;
        let t = c.g[V0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if t {
            break 'b_3c3720;
        }
        c.cycles += 15;
        c.g[AT][0] = 0x46ff_0000;
        c.g[AT][0] |= 0xfe00;
        c.mtc1(AT, 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x54f8));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(2, 0);
        c.mfc1(V0, 2);
        c.sh((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(V0, 1);
        c.sh((c.g[A0][0] as u32).wrapping_add(0x16), c.g[V0][0]);
        break 'b_3c3720;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_SpuMemClampBlock(uint* addr, uint* size)` (0x3c4af8): clamp the
/// requested SPU-memory block against the three size limits, adjusting both
/// the address and the size.
pub fn snd_spu_mem_clamp_block(c: &mut Cpu) {
    'b_3c4b30: {
        c.cycles += 7;
        c.g[V0][0] = 0x51_0000;
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x1a9a).lhu(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
            break 'b_3c4b30;
        }
        c.cycles += 7;
        c.sw(abi::arg(c, 0), c.g[V1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        break 'b_3c4b30;
    }
    'b_3c4b48: {
        c.cycles += 5;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x1a9c).lhu(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        if !t {
            c.cycles += 1;
            c.sw(abi::arg(c, 1), c.g[V0][0]);
        }
        break 'b_3c4b48;
    }
    'b_3c4b68: {
        c.cycles += 7;
        c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x1a9e).lhu(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if !t {
            c.cycles += 1;
            c.sw(abi::arg(c, 1), c.g[V0][0]);
        }
        break 'b_3c4b68;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SNDI_SpuMemSetBase(int base, int end)` (0x3c4e50): program the SPU
/// memory window, deriving the clamped base from the requested address.
pub fn sndi_spu_mem_set_base(c: &mut Cpu) {
    'b_3c4eb4: {
        'b_3c4e78: {
            'b_3c4e68: {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(0x3f));
                if t {
                    break 'b_3c4e68;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x51_0000;
                c.g[A0][0] = 0x141;
                break 'b_3c4e78;
            }
            c.cycles += 4;
            c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V1][0] = ((c.g[A0][0] as i64) < 321) as u64;
            let t = c.g[V1][0] != 0;
            c.g[V0][0] = 0x51_0000;
            if t {
                break 'b_3c4eb4;
            }
            break 'b_3c4e78;
        }
        'b_3c4ec0: {
            'b_3c4e9c: {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = u64::MAX;
                let t = c.g[A1][0] != c.g[V0][0];
                c.sh((c.g[V1][0] as u32).wrapping_add(0x1a9a), c.g[A0][0]);
                if t {
                    break 'b_3c4e9c;
                }
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5300));
                c.g[V0][0] = 0x8000;
                c.sh((c.g[V1][0] as u32).wrapping_add(0x1a9c), c.g[V0][0]);
                break 'b_3c4ec0;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f));
            c.g[V0][0] = 0x8000;
            c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5300));
            if !t {
                break 'b_3c4eb4;
            }
            c.cycles += 1;
            c.sh((c.g[V0][0] as u32).wrapping_add(0x1a9c), c.g[A1][0]);
            break 'b_3c4ec0;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_fff8;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound driver globals.

/// `lui 0x51`: the high half the driver builds its global addresses from.
pub(crate) const STATE_HI: u64 = 0x0051_0000;

/// The driver state block (`0x510000 - 0x5718`).
pub(crate) const STATE: u32 = 0x0050_a8e8;

/// The driver's second global block (`0x510000 - 0x5300`).
pub(crate) const SYNC: u32 = 0x0050_ad00;

/// `SND_EeVoiceSetPitch(voice, rate)` (function table, cross-shard).
pub(crate) const EE_VOICE_SET_PITCH: u32 = 0x003c_8a88;

/// `SND_EeVoiceSetVolume(voice, output, gain)` (function table).
pub(crate) const EE_VOICE_SET_VOLUME: u32 = 0x003c_8560;

/// 127.0, the 7-bit volume scale.
pub(crate) const VOL_SCALE: FpuF32 = FpuF32(0x42fe_0000);

/// 1/127.
pub(crate) const INV_127: FpuF32 = FpuF32(0x3c01_0204);

// ---------------------------------------------------------------------------
// Register arithmetic as the compiled code does it.

/// `addiu`.
#[inline(always)]
pub(crate) fn addiu(a: u64, imm: i32) -> u64 {
    sx((a as u32).wrapping_add(imm as u32))
}

/// `subu`.
#[inline(always)]
pub(crate) fn subu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_sub(b as u32))
}

/// `sll`.
#[inline(always)]
pub(crate) fn sll(v: u64, n: u32) -> u64 {
    sx((v as u32) << n)
}

/// `slt` on whole registers.
#[inline(always)]
pub(crate) fn slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// The address `base + off` (32-bit).
#[inline(always)]
pub(crate) fn at(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// An FPU register.
#[inline(always)]
pub(crate) fn fpr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `cvt.w.s` then `mfc1`: the integer bits.
#[inline(always)]
pub(crate) fn cvt_w(v: FpuF32) -> u32 {
    v.to_i32() as u32
}

/// The channel `index` (`STATE->channels + index * 0x8c`); the product is
/// a `mult` (LO/HI set) whose low word lands in a0.
pub(crate) fn channel_at(c: &mut Cpu) -> u64 {
    let index = r32(c, A0);
    let offset = abi::mult(c, index, chan::SIZE);
    set_reg(c, A0, sx(offset));
    offset as u64
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyPitch.

/// `int SND_ChannelApplyPitch(int channel)` (0x3c25e0): push the channel's
/// pitch to its voices. Returns 0.
///
/// ```text
/// SPU voices:  rate = min(pitch * 4096 / 48000 * scale >> 12, 0x3fff)
///              params[v].pitch (+0x24) = rate   (and params[v + 0x18] in mode 2)
/// IOP voices:  rate = pitch * 4096 / iop_rate * scale >> 12
///              params[(v - base) * 2].dsp_pitch (+0x1a4) = rate
/// EE voices:   rate = pitch * 4096 / ee_rate * scale >> 8
///              SND_EeVoiceSetPitch(v - base - ee_base, rate)
/// ```
///
/// Frame 0x60 (s0 +0x50, s1 +0x40, s2 +0x30, s3 +0x20, s4 +0x10, ra +0).
pub fn snd_channel_apply_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        set_reg(c, V0, chan::SIZE as u64);
        frame.save(c, S2, 0x30);
        set_reg(c, V1, STATE_HI);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let offset = channel_at(c);
        set_reg(c, A3, sx(STATE));
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let channels = Addr(STATE).off(state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset as u32));
        set_reg(c, S2, ch);
        let flags = at(ch, chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & chan::IOP);
        if flags & chan::SPU != 0 {
            return pitch_spu(c, frame, ch);
        }
        c.cycles += 2;
        set_reg(c, S1, 0);
        if flags & chan::IOP != 0 {
            pitch_iop(c, frame, ch)
        } else {
            pitch_ee(c, frame, ch)
        }
    })
}

/// `pitch << 12 / rate_base * scale` as the original computes it (`div`
/// then `mult`, LO/HI left by the `mult`); leaves the scale in a0 and the
/// product in v0, returns the product's low word.
pub(crate) fn pitch_ratio(c: &mut Cpu, ch: u64, rate_base: u64) -> u32 {
    let pitch = at(ch, chan::PITCH).lhu(c);
    let scaled = sll(pitch, 12);
    let (ratio, _) = abi::div(c, scaled as u32, rate_base as u32);
    let scale = at(ch, chan::PITCH_SCALE).lhu(c);
    set_reg(c, A0, scale);
    let product = abi::mult(c, ratio, scale as u32);
    set_reg(c, V0, sx(product));
    product
}

/// SPU voices: the rate goes into each voice's parameter block.
pub(crate) fn pitch_spu(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 4 + 12;
    set_reg(c, A2, 0x3fff);
    let product = pitch_ratio(c, ch, 0xbb80);
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, A1, count);
    set_reg(c, S1, 0);
    let raw = sx(product >> 12);
    let in_range = (raw < 0x4000) as u64;
    set_reg(c, V1, in_range);
    set_reg(c, S3, if in_range != 0 { raw } else { 0x3fff });
    if count == 0 {
        return pitch_exit_early(c, frame);
    }
    c.cycles += 5;
    set_reg(c, V0, STATE_HI);
    set_reg(c, A2, 2);
    set_reg(c, A1, sx(SYNC));
    set_reg(c, A0, addiu(ch, chan::VOICES as i32));
    loop {
        // Loop head 0x3c2678 (a0 the voice slot, s1 the voices done).
        c.cycles += 8;
        let slot = reg(c, A0);
        let rate = reg(c, S3);
        let voice = at(slot, 0).lh(c);
        let entry = spu_voice_params(c, voice);
        at(entry, 0x24).sh(c, rate);
        let mode = Addr(STATE).off(state::OUTPUT_MODE).lbu(c);
        if mode == 2 {
            c.cycles += 7;
            let voice = at(slot, 0).lh(c);
            let entry = spu_voice_params(c, addiu(voice, 0x18));
            at(entry, 0x24).sh(c, rate);
        }
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        c.cycles += 4;
        let done = addiu(reg(c, S1), 1);
        set_reg(c, S1, done);
        let more = slt(done, count);
        set_reg(c, V0, more);
        set_reg(c, A0, addiu(slot, 2));
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            return pitch_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2678);
        }
    }
}

/// `params + voice * 8` (left in v1).
pub(crate) fn spu_voice_params(c: &mut Cpu, voice: u64) -> u64 {
    let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
    let entry = addu(params, sll(voice, 3));
    set_reg(c, V1, entry);
    entry
}

/// IOP DSP voices: the rate goes into the DSP block at +0x1a4.
pub(crate) fn pitch_iop(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 6 + 6;
    let rate_base = Addr(STATE).off(state::IOP_RATE).lhu(c);
    let product = pitch_ratio(c, ch, rate_base);
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, V1, count);
    set_reg(c, S3, sx(product >> 12));
    if count == 0 {
        return pitch_exit_early(c, frame);
    }
    c.cycles += 3;
    set_reg(c, V0, STATE_HI);
    set_reg(c, A1, addiu(ch, chan::VOICES as i32));
    set_reg(c, A2, sx(SYNC));
    loop {
        // Loop head 0x3c2710 (a1 the voice slot, s1 the voices done).
        c.cycles += 12;
        let slot = reg(c, A1);
        let rate = reg(c, S3);
        let voice = at(slot, 0).lh(c);
        let done = addiu(reg(c, S1), 1);
        set_reg(c, S1, done);
        let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
        set_reg(c, A0, base);
        let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
        let entry = addu(params, sll(subu(voice, base), 4));
        set_reg(c, V1, entry);
        at(entry, 0x1a4).sh(c, rate);
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let more = slt(done, count);
        set_reg(c, V0, more);
        set_reg(c, A1, addiu(slot, 2));
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            return pitch_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2710);
        }
    }
}

/// EE software voices: `SND_EeVoiceSetPitch(voice, rate)` per voice.
pub(crate) fn pitch_ee(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 6 + 6;
    let rate_base = Addr(STATE).off(state::EE_RATE).lhu(c);
    let product = pitch_ratio(c, ch, rate_base);
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, V1, count);
    set_reg(c, S3, sx(product >> 8));
    if count == 0 {
        return pitch_exit_early(c, frame);
    }
    c.cycles += 4;
    set_reg(c, S4, sx(STATE));
    set_reg(c, S0, addiu(ch, chan::VOICES as i32));
    let ee_base = Addr(STATE).off(state::EE_VOICE_BASE).lbu(c);
    set_reg(c, V0, ee_base);
    loop {
        // Loop head 0x3c2788 (s0 the voice slot, s1 the voices done, v0
        // the EE voice base).
        c.cycles += 12;
        let rate = reg(c, S3);
        set_reg(c, A1, rate);
        let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
        let done = addiu(reg(c, S1), 1);
        set_reg(c, S1, done);
        let slot = reg(c, S0);
        let voice = at(slot, 0).lh(c);
        set_reg(c, V1, voice);
        let ee_base = reg(c, V0);
        set_reg(c, S0, addiu(slot, 2));
        set_reg(c, A0, subu(voice, addu(base, ee_base)));
        call_ptr(c, EE_VOICE_SET_PITCH, 0x003c_27a8)?;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let more = slt(reg(c, S1), count);
        set_reg(c, V0, more);
        if more == 0 {
            return pitch_exit_early(c, frame);
        }
        let ee_base = Addr(STATE).off(state::EE_VOICE_BASE).lbu(c);
        set_reg(c, V0, ee_base);
        if event_due(c) {
            deliver_event_here(c, 0x003c_2788);
        }
    }
}

/// The early exit (`lq s0` in its own block).
pub(crate) fn pitch_exit_early(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x50);
    pitch_exit(c, frame)
}

/// The epilogue: return 0.
pub(crate) fn pitch_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    set_reg(c, V0, 0);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyVolume.

/// Callee-saved GPRs of `SND_ChannelApplyVolume`'s frame (s0 is restored
/// separately, in the exit blocks).
pub(crate) const VOLUME_SAVES: [(usize, u32); 7] = [
    (S2, 0x70),
    (S3, 0x60),
    (S4, 0x50),
    (S5, 0x40),
    (S6, 0x30),
    (S7, 0x20),
    (FP, 0x10),
];

/// `void SND_ChannelApplyVolume(int channel)` (0x3c2a50): push the
/// channel's volume to its voices.
///
/// ```text
/// SPU voices:  per voice, two (four in output mode 2) 7-bit levels
///              params[v].level{L,R} = (int)(gain[v][col] * volume * 127)
/// IOP voices:  g = volume * pan * (1/127)
///              dsp[(v - base) * 16 + k].level (+0x1a6) = (int)(g * gain[v][k] * 127)
///              for k < speakers + sends
/// EE voices:   SND_EeVoiceSetVolume(v - base - ee_base, k, g * gain[v][k])
/// ```
///
/// Frame 0xb0 (s0 +0x90, s1 +0x80 .. fp +0x10, ra +0, f20 +0xa0).
pub fn snd_channel_apply_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        set_reg(c, V0, chan::SIZE as u64);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        let offset = channel_at(c);
        set_reg(c, A3, STATE_HI);
        set_reg(c, A1, sx(STATE));
        for (r, off) in VOLUME_SAVES {
            frame.save(c, r, off);
        }
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0xa0);
        let channels = Addr(STATE).off(state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset as u32));
        set_reg(c, S1, ch);
        let flags = at(ch, chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & chan::IOP);
        if flags & chan::SPU != 0 {
            return volume_spu(c, frame, ch);
        }
        c.cycles += 2;
        set_reg(c, A2, 0);
        // The send gain shared by the IOP and EE paths.
        c.cycles += 14;
        let pan = at(ch, chan::PAN).lb(c);
        let volume = at(ch, chan::VOLUME).read_fpu(c);
        let pan_f = FpuF32::from_i32(pan as u32 as i32);
        let sends = Addr(STATE).off(state::AUX_SENDS).lbu(c);
        set_reg(c, A0, sends);
        let speakers = Addr(STATE).off(state::SPEAKERS).lbu(c);
        set_reg(c, V1, speakers);
        set_reg(c, AT, INV_127.0 as u64);
        let panned = volume * pan_f;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        set_reg(c, V0, count);
        set_reg(c, S4, addu(speakers, sends));
        let gain = panned * INV_127;
        abi::set_fpr(c, 0, panned);
        abi::set_fpr(c, 1, pan_f);
        abi::set_fpr(c, 2, INV_127);
        abi::set_fpr(c, 20, gain);
        if count == 0 {
            return volume_exit_early(c, frame);
        }
        if flags & chan::IOP != 0 {
            volume_iop(c, frame, ch)
        } else {
            volume_ee(c, frame, ch)
        }
    })
}

/// SPU voices: 7-bit levels into the voice parameter blocks.
pub(crate) fn volume_spu(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 3;
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, V0, count);
    set_reg(c, A2, 0);
    if count == 0 {
        return volume_exit_early(c, frame);
    }
    c.cycles += 11;
    set_reg(c, V0, STATE_HI);
    let state_reg = reg(c, A1);
    set_reg(c, T4, state_reg);
    set_reg(c, A3, sx(SYNC));
    set_reg(c, AT, VOL_SCALE.0 as u64);
    abi::set_fpr(c, 2, VOL_SCALE);
    set_reg(c, T0, sx(SYNC + sync::GAINS + 4));
    set_reg(c, T3, sx(SYNC + sync::GAINS));
    set_reg(c, T2, sx(SYNC + sync::GAINS + 0xc));
    set_reg(c, T1, sx(SYNC + sync::GAINS + 8));
    set_reg(c, A1, addiu(ch, chan::VOICES as i32));
    loop {
        // Loop head 0x3c2ae0 (a1 the voice slot, a2 the voices done).
        c.cycles += 4;
        let mode = Addr(STATE).off(state::OUTPUT_MODE).lbu(c);
        if mode != 2 {
            c.cycles += 21 + 11;
            spu_level(c, ch, 0x4, 0, 0x26);
            spu_level(c, ch, 0x0, 0, 0x27);
        } else {
            let speakers = Addr(STATE).off(state::SPEAKERS).lbu(c);
            c.cycles += 3;
            if speakers != 4 {
                c.cycles += 44 + 3 + 11;
                spu_level(c, ch, 0x10, 0, 0x26);
                spu_level(c, ch, 0x4, 0, 0x27);
                spu_level(c, ch, 0xc, 0x18, 0x26);
                spu_level(c, ch, 0x8, 0x18, 0x27);
            } else {
                c.cycles += 45 + 3 + 11;
                spu_level(c, ch, 0xc, 0, 0x26);
                spu_level(c, ch, 0x0, 0, 0x27);
                spu_level(c, ch, 0x8, 0x18, 0x26);
                spu_level(c, ch, 0x4, 0x18, 0x27);
            }
        }
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let done = addiu(reg(c, A2), 1);
        set_reg(c, A2, done);
        let more = slt(done, count);
        set_reg(c, V0, more);
        let slot = reg(c, A1);
        set_reg(c, A1, addiu(slot, 2));
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            return volume_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2ae0);
        }
    }
}

/// One SPU level: `params[voice + upper].byte = (int)(gain[voice][column] *
/// volume * 127)` (registers as the original leaves them: v1 the entry
/// offset, a0 the entry, v0 the level, f0 the float level, f1 its bits).
pub(crate) fn spu_level(c: &mut Cpu, ch: u64, column: u32, upper: i32, byte: u32) {
    let slot = reg(c, A1);
    let voice = at(slot, 0).lh(c);
    let volume = at(ch, chan::VOLUME).read_fpu(c);
    let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
    let gain = Addr(SYNC + sync::GAINS + column)
        .off((voice as u32) << 5)
        .read_fpu(c);
    let entry_off = sll(addiu(voice, upper), 3);
    set_reg(c, V1, entry_off);
    let entry = addu(params, entry_off);
    set_reg(c, A0, entry);
    let level = (gain * volume) * fpr(c, 2);
    let bits = cvt_w(level);
    abi::set_fpr(c, 0, level);
    c.f[1] = bits;
    set_reg(c, V0, sx(bits));
    at(entry, byte).sb(c, sx(bits));
}

/// IOP DSP voices: a level per output into the DSP block (+0x1a6).
pub(crate) fn volume_iop(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 5;
    let hi = reg(c, A3);
    set_reg(c, T5, hi);
    set_reg(c, T4, STATE_HI);
    set_reg(c, T3, addiu(ch, chan::VOICES as i32));
    set_reg(c, S0, 0);
    loop {
        // Loop head 0x3c2d48 (a2 the voice index).
        c.cycles += 2;
        let i = reg(c, A2);
        set_reg(c, S5, addiu(i, 1));
        if reg(c, S4) != 0 {
            c.cycles += 8;
            set_reg(c, V0, sll(i, 1));
            set_reg(c, T0, sx(SYNC));
            set_reg(c, AT, VOL_SCALE.0 as u64);
            abi::set_fpr(c, 1, VOL_SCALE);
            let slots = reg(c, T3);
            set_reg(c, A3, addu(slots, sll(i, 1)));
            set_reg(c, T2, sx(SYNC + sync::GAINS));
            let hi = reg(c, T5);
            set_reg(c, T1, addiu(hi, -0x5718));
            loop {
                // Loop head 0x3c2d70 (a3 the voice slot, s0 the output).
                c.cycles += 20;
                let k = reg(c, S0);
                let slot = reg(c, A3);
                let outputs = reg(c, S4);
                let voice = at(slot, 0).lh(c);
                let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
                set_reg(c, A1, base);
                let voice_off = sll(voice, 5);
                set_reg(c, A0, voice_off);
                let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
                let gain_at = addu(addu(sll(k, 2), voice_off), sx(SYNC + sync::GAINS));
                let gain = at(gain_at, 0).read_fpu(c);
                let dst = addu(params, addu(k, sll(subu(voice, base), 4)));
                set_reg(c, A2, dst);
                let next = addiu(k, 1);
                set_reg(c, S0, next);
                let level = (fpr(c, 20) * gain) * fpr(c, 1);
                let more = slt(next, outputs);
                set_reg(c, V1, more);
                let bits = cvt_w(level);
                abi::set_fpr(c, 0, level);
                c.f[2] = bits;
                set_reg(c, V0, sx(bits));
                at(dst, 0x1a6).sb(c, sx(bits));
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_2d70);
                }
            }
        }
        c.cycles += 5;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let next = reg(c, S5);
        set_reg(c, A2, next);
        let more = slt(next, count);
        set_reg(c, V0, more);
        set_reg(c, S0, 0);
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            return volume_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2d48);
        }
    }
}

/// EE software voices: `SND_EeVoiceSetVolume(voice, k, g * gain[v][k])` per
/// voice and output.
pub(crate) fn volume_ee(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 3;
    set_reg(c, S7, addiu(ch, chan::VOICES as i32));
    set_reg(c, FP, STATE_HI);
    set_reg(c, S0, 0);
    loop {
        // Loop head 0x3c2e20 (a2 the voice index).
        c.cycles += 2;
        let i = reg(c, A2);
        set_reg(c, S5, addiu(i, 1));
        if reg(c, S4) != 0 {
            c.cycles += 6;
            set_reg(c, V0, sll(i, 1));
            set_reg(c, V1, STATE_HI);
            set_reg(c, S2, sx(STATE));
            let slots = reg(c, S7);
            let slot = addu(slots, sll(i, 1));
            set_reg(c, S3, slot);
            let hi = reg(c, FP);
            set_reg(c, S6, addiu(hi, -0x4468));
            let voice = at(slot, 0).lh(c);
            set_reg(c, A3, voice);
            loop {
                // Loop head 0x3c2e40 (a3 the voice, s0 the output).
                c.cycles += 16;
                let k = reg(c, S0);
                let voice = reg(c, A3);
                let gains = reg(c, S6);
                let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
                set_reg(c, A1, k);
                let voice_off = sll(voice, 5);
                set_reg(c, V1, voice_off);
                let ee_base = Addr(STATE).off(state::EE_VOICE_BASE).lbu(c);
                set_reg(c, A2, ee_base);
                let gain_at = addu(addu(sll(k, 2), voice_off), gains);
                set_reg(c, V0, gain_at);
                set_reg(c, S0, addiu(k, 1));
                let gain = at(gain_at, 0).read_fpu(c);
                set_reg(c, A0, subu(voice, addu(base, ee_base)));
                let send = fpr(c, 20) * gain;
                abi::set_fpr(c, 12, send);
                call_ptr(c, EE_VOICE_SET_VOLUME, 0x003c_2e74)?;
                let more = slt(reg(c, S0), reg(c, S4));
                set_reg(c, V0, more);
                if more == 0 {
                    break;
                }
                let slot = reg(c, S3);
                let voice = at(slot, 0).lh(c);
                set_reg(c, A3, voice);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_2e40);
                }
            }
        }
        c.cycles += 5;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let next = reg(c, S5);
        set_reg(c, A2, next);
        let more = slt(next, count);
        set_reg(c, V0, more);
        set_reg(c, S0, 0);
        if more == 0 {
            return volume_exit_early(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2e20);
        }
    }
}

/// The early exit (`lq s0` in its own block).
pub(crate) fn volume_exit_early(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x90);
    volume_exit(c, frame)
}

/// The epilogue.
pub(crate) fn volume_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 12;
    frame.restore(c, S1, 0x80);
    for (r, off) in VOLUME_SAVES {
        frame.restore(c, r, off);
    }
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0xa0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// UTK synthesis filter.

/// `SND_UtkLpcFilter(int blocks, float a[12], float hist[12], float* x)`
/// (0x3ce410): the 12-tap all-pole synthesis filter, in place over
/// `blocks` (at least one) blocks of 12 samples:
///
/// ```text
/// y[n] = x[n] + sum_k a[k] * y[n - 1 - k]
/// ```
///
/// The history lives in f12..f23 as a ring (sample j of a block reads tap
/// k from register `12 + (k - j) mod 12` and writes `23 - j`); each sum is
/// accumulated with ADDA/MADDA in the original order (tap 0 first in a
/// block's first sample, taps 1..11 then 0 in the others). The final
/// history is stored back to `hist`. f0..f23, f30, f31 (zero) and ACC are
/// left as the original leaves them (it saves none of f20..f31).
pub fn snd_utk_lpc_filter(c: &mut Cpu) {
    c.cycles += 25;
    let coefs = Addr(r32(c, A1));
    let history = Addr(r32(c, A2));
    let mut a = [FpuF32::ZERO; 12];
    for (k, tap) in a.iter_mut().enumerate() {
        *tap = coefs.off(4 * k as u32).read_fpu(c);
    }
    let zero = FpuF32::ZERO;
    let mut h = [FpuF32::ZERO; 12];
    for (k, y) in h.iter_mut().enumerate() {
        *y = history.off(4 * k as u32).read_fpu(c);
    }
    for k in 0..12 {
        c.f[k] = a[k].0;
    }
    c.f[31] = zero.0;
    loop {
        // Loop head 0x3ce474 (a0 blocks left, a3 the block).
        c.cycles += 196;
        let x_at = Addr(r32(c, A3));
        let left = addiu(reg(c, A0), -1);
        set_reg(c, A0, left);
        let mut x = FpuF32::ZERO;
        let mut acc = FpuF32::ZERO;
        for j in 0..12 {
            x = x_at.off(4 * j as u32).read_fpu(c);
            acc = x + zero;
            for t in 0..12 {
                let k = if j == 0 { t } else { (t + 1) % 12 };
                acc = FpuF32::madd(acc, a[k], h[(k + 12 - j) % 12]);
            }
            let y = FpuF32::madd(acc, zero, zero);
            h[11 - j] = y;
            x_at.off(4 * j as u32).write_fpu(c, y);
        }
        set_reg(c, A3, addiu(x_at.0 as u64, 0x30));
        for k in 0..12 {
            c.f[12 + k] = h[k].0;
        }
        c.f[30] = x.0;
        c.facc = acc.0;
        if (left as i64) <= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_e474);
            for k in 0..12 {
                a[k] = fpr(c, k);
                h[k] = fpr(c, 12 + k);
            }
        }
    }
    c.cycles += 14;
    for (k, y) in h.iter().enumerate() {
        history.off(4 * k as u32).write_fpu(c, *y);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UTK excitation.

/// `SND_UtkReadBits(BitReader*, n)` / `SND_UtkSkipBits(BitReader*, n)`.
pub(crate) const UTK_READ_BITS: Callee = Callee {
    addr: 0x003c_d1f8,
    f: ee::s14::SND_UtkReadBits,
};

pub(crate) const UTK_SKIP_BITS: Callee = Callee {
    addr: 0x003c_d260,
    f: ee::s14::SND_UtkSkipBits,
};

/// `SND_UtkDecodeExcitation`'s frame: 0xb0 (s0 +0x90, s1 +0x80, s2 +0x70,
/// s3 +0x60, s4 +0x50, s5 +0x40, s6 +0x30, s7 +0x20, fp +0x10, ra +0, f21
/// +0xa8, f20 +0xa0).
pub(crate) const EXC_FRAME: Frame = Frame { size: 0xb0 };

/// Stages of `SND_UtkDecodeExcitation` (blocks of the original; the state
/// is in the registers throughout).
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Exc {
    Symbol,
    ZeroRunStart,
    ZeroRun,
    EscapeStart,
    EscapeMore,
    EscapeRead,
    Next,
    Tail,
    Pulse,
    Exit,
}

/// `void SND_UtkDecodeExcitation(BitReader* br, int huffman, float* out,
/// int stride)` (0x3cd2b0): decode the 108-sample excitation of a UTK
/// subframe into every `stride`-th sample of `out`.
///
/// ```text
/// huffman:  peek a byte; sym = SYMBOLS[state][byte]; skip CODES[sym].bits;
///           state = CODES[sym].next_state
///           sym >= 4: one sample CODES[sym].value
///           sym 2, 3: a run of (6 bits) + 7 zeros (clipped to the frame)
///           sym 0, 1: escape: magnitude 7 + count of 1 bits, then a sign
///                     bit (1 = positive)
/// pulse:    per sample, the low 2 bits: 1 -> -2.0, 3 -> +2.0 (2 bits),
///           otherwise 0.0 (1 bit)
/// ```
///
/// The original is a `match pc` dispatcher whose loops return on an unwind
/// request, so the loop heads are entry points too (the state is in s0..s7,
/// fp and the temporaries the original keeps there).
pub fn snd_utk_decode_excitation(c: &mut Cpu) {
    let mut stage = match c.pc {
        exc::START => exc_enter(c),
        exc::SYMBOL => Exc::Symbol,
        exc::ZERO_RUN => Exc::ZeroRun,
        exc::ESCAPE => Exc::EscapeMore,
        exc::PULSE => Exc::Pulse,
        pc => {
            c.jump_out(pc, exc::START, exc::END);
            return;
        }
    };
    loop {
        let next = match stage {
            Exc::Symbol => exc_symbol(c),
            Exc::ZeroRunStart => exc_zero_run_start(c),
            Exc::ZeroRun => exc_zero_run(c),
            Exc::EscapeStart => {
                c.cycles += 3;
                let pos = reg(c, S2);
                set_reg(c, S1, sll(pos, 2));
                let stride = reg(c, S3);
                set_reg(c, S2, addu(pos, stride));
                Resume::Go(Exc::EscapeRead)
            }
            Exc::EscapeMore => {
                c.cycles += 1;
                let n = reg(c, S0);
                set_reg(c, S0, addiu(n, 1));
                Resume::Go(Exc::EscapeRead)
            }
            Exc::EscapeRead => exc_escape_read(c),
            Exc::Next => {
                c.cycles += 1;
                let pos = reg(c, S2);
                set_reg(c, V0, slt(pos, exc::SAMPLES));
                Resume::Go(Exc::Tail)
            }
            Exc::Tail => exc_tail(c),
            Exc::Pulse => exc_pulse(c),
            Exc::Exit => {
                exc_exit(c);
                return;
            }
        };
        match next {
            Resume::Go(s) => stage = s,
            Resume::Done | Resume::Unwind => return,
        }
    }
}

/// The prologue and the mode set-up.
pub(crate) fn exc_enter(c: &mut Cpu) -> Exc {
    let frame = EXC_FRAME;
    c.cycles += 17;
    Frame::open(c, frame.size);
    frame.save(c, S3, 0x60);
    frame.save(c, S4, 0x50);
    let stride = reg(c, A3);
    set_reg(c, S3, stride);
    frame.save(c, S5, 0x40);
    let reader = reg(c, A0);
    set_reg(c, S4, reader);
    frame.save(c, S0, 0x90);
    let out = reg(c, A2);
    set_reg(c, S5, out);
    frame.save(c, S1, 0x80);
    frame.save(c, S2, 0x70);
    frame.save(c, S6, 0x30);
    frame.save(c, S7, 0x20);
    frame.save(c, FP, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 21, 0xa8);
    frame.save_fpr(c, 20, 0xa0);
    if reg(c, A1) == 0 {
        // Pulse mode: s1 the output, s5 the byte stride, f21/f20 = -2/+2.
        c.cycles += 10;
        set_reg(c, S1, out);
        set_reg(c, AT, 0x4000_0000);
        c.f[21] = 0xc000_0000;
        c.f[20] = 0x4000_0000;
        set_reg(c, S0, 0);
        set_reg(c, S7, 1);
        set_reg(c, S6, 3);
        set_reg(c, S5, sll(stride, 2));
        Exc::Pulse
    } else {
        // Huffman mode: fp the state, s7 the code table, s2 the position.
        c.cycles += 7;
        set_reg(c, FP, 0);
        set_reg(c, S7, sx(exc::CODES));
        set_reg(c, S2, 0);
        set_reg(c, S6, 1);
        let peek = at(reader, 4).lbu(c);
        set_reg(c, V0, peek);
        Exc::Symbol
    }
}

/// Decode one symbol (loop head 0x3cd310).
pub(crate) fn exc_symbol(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 17;
    let state = reg(c, FP);
    let peek = reg(c, V0);
    let reader = reg(c, S4);
    set_reg(c, A0, reader);
    let sym_at = addu(addu(peek, sll(state, 8)), sx(exc::SYMBOLS));
    let sym = at(sym_at, 0).lbu(c);
    set_reg(c, S1, sym);
    let entry_off = sx(abi::mult(c, sym as u32, 12));
    set_reg(c, S0, entry_off);
    let codes = reg(c, S7);
    let entry = addu(codes, entry_off);
    set_reg(c, V0, entry);
    set_reg(c, V1, entry);
    let bits = at(entry, 4).lw(c);
    set_reg(c, A1, bits);
    let next_state = at(entry, 0).lw(c);
    set_reg(c, FP, next_state);
    if UTK_SKIP_BITS.call(c, 0x003c_d348).is_err() {
        return Resume::Unwind;
    }
    let sym = reg(c, S1);
    set_reg(c, V0, slt(sym, 2));
    if slt(sym, 4) == 0 {
        // A value: out[pos] = CODES[sym].value.
        c.cycles += 8 + 1;
        let entry_off = reg(c, S0);
        let value_at = addu(entry_off, sx(exc::CODE_VALUES));
        set_reg(c, V0, value_at);
        let pos = reg(c, S2);
        let out = reg(c, S5);
        let out_at = addu(sll(pos, 2), out);
        set_reg(c, V1, out_at);
        let value = at(value_at, 0).read_fpu(c);
        abi::set_fpr(c, 0, value);
        let stride = reg(c, S3);
        set_reg(c, S2, addu(pos, stride));
        at(out_at, 0).write_fpu(c, value);
        return Resume::Go(Exc::Next);
    }
    c.cycles += 2;
    set_reg(c, S0, 7);
    Resume::Go(if sym < 2 {
        Exc::EscapeStart
    } else {
        Exc::ZeroRunStart
    })
}

/// A run of zeros: read its length, clip it to the frame.
pub(crate) fn exc_zero_run_start(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 9;
    let reader = reg(c, S4);
    set_reg(c, A0, reader);
    set_reg(c, A1, 6);
    if UTK_READ_BITS.call(c, 0x003c_d388).is_err() {
        return Resume::Unwind;
    }
    let run = addiu(reg(c, V0), 7);
    set_reg(c, A0, run);
    let stride = reg(c, S3);
    let pos = reg(c, S2);
    let span = sx(abi::mult(c, stride as u32, run as u32));
    set_reg(c, V1, span);
    let fits = slt(addu(span, pos), 109);
    set_reg(c, V0, 0x6c);
    let mut count = run;
    if fits == 0 {
        c.cycles += 2 + 4;
        let left = subu(0x6c, pos);
        let (q, _) = abi::div(c, left as u32, stride as u32);
        count = sx(q);
        set_reg(c, V0, count);
        set_reg(c, A0, count);
    }
    c.cycles += 2;
    set_reg(c, S0, 0);
    if (count as i64) <= 0 {
        return Resume::Go(Exc::Next);
    }
    c.cycles += 4;
    let first = sll(pos, 2);
    set_reg(c, V0, first);
    set_reg(c, A1, sll(stride, 2));
    let out = reg(c, S5);
    set_reg(c, V1, addu(first, out));
    Resume::Go(Exc::ZeroRun)
}

/// One zero of a run (loop head 0x3cd3d0: v1 the output, a1 the byte
/// stride, a0 the run, s0 the count).
pub(crate) fn exc_zero_run(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 7;
    let out = reg(c, V1);
    at(out, 0).sw(c, 0);
    let n = addiu(reg(c, S0), 1);
    set_reg(c, S0, n);
    let step = reg(c, A1);
    set_reg(c, V1, addu(out, step));
    let more = slt(n, reg(c, A0));
    set_reg(c, V0, more);
    let pos = addu(reg(c, S2), reg(c, S3));
    set_reg(c, S2, pos);
    if more != 0 {
        if back_edge_unwinds(c, exc::ZERO_RUN) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::ZeroRun);
    }
    c.cycles += 2;
    set_reg(c, V0, slt(pos, exc::SAMPLES));
    Resume::Go(Exc::Tail)
}

/// An escape value: count 1 bits onto the magnitude in s0, then the sign.
pub(crate) fn exc_escape_read(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 5;
    let reader = reg(c, S4);
    set_reg(c, A0, reader);
    set_reg(c, A1, 1);
    if UTK_READ_BITS.call(c, 0x003c_d410).is_err() {
        return Resume::Unwind;
    }
    let bit = reg(c, V0);
    set_reg(c, A0, reader);
    if bit == reg(c, S6) {
        if back_edge_unwinds(c, exc::ESCAPE) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::EscapeMore);
    }
    c.cycles += 4;
    set_reg(c, A1, 1);
    if UTK_READ_BITS.call(c, 0x003c_d420).is_err() {
        return Resume::Unwind;
    }
    let sign = reg(c, V0);
    let magnitude = reg(c, S0);
    let negated = subu(0, magnitude);
    set_reg(c, V0, negated);
    let slot = reg(c, S1);
    let out = reg(c, S5);
    let out_at = addu(slot, out);
    if sign != reg(c, S6) {
        c.cycles += 3 + 1;
        set_reg(c, V1, out_at);
        let value = FpuF32::from_i32(negated as u32 as i32);
        abi::set_fpr(c, 0, value);
        at(out_at, 0).write_fpu(c, value);
    } else {
        c.cycles += 5;
        let value = FpuF32::from_i32(magnitude as u32 as i32);
        abi::set_fpr(c, 0, value);
        set_reg(c, V0, out_at);
        at(out_at, 0).write_fpu(c, value);
    }
    Resume::Go(Exc::Next)
}

/// The Huffman loop's back edge (v0: more samples to go).
pub(crate) fn exc_tail(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 2;
    if reg(c, V0) != 0 {
        let reader = reg(c, S4);
        let peek = at(reader, 4).lbu(c);
        set_reg(c, V0, peek);
        if back_edge_unwinds(c, exc::SYMBOL) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::Symbol);
    }
    c.cycles += 2;
    EXC_FRAME.restore(c, S0, 0x90);
    Resume::Go(Exc::Exit)
}

/// One pulse-mode sample (loop head 0x3cd488: s1 the output, s0 the
/// position).
pub(crate) fn exc_pulse(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 4;
    let reader = reg(c, S4);
    let bits = at(reader, 4).lw(c);
    set_reg(c, V0, bits);
    let code = bits & 3;
    set_reg(c, S2, code);
    let out = reg(c, S1);
    let ret = if code == reg(c, S7) {
        c.cycles += 2 + 5;
        let minus_two = fpr(c, 21);
        at(out, 0).write_fpu(c, minus_two);
        set_reg(c, A1, 2);
        0x003c_d4b8
    } else if code != reg(c, S6) {
        c.cycles += 2 + 4;
        at(out, 0).sw(c, 0);
        set_reg(c, A1, 1);
        0x003c_d4cc
    } else {
        c.cycles += 2 + 1 + 5;
        let two = fpr(c, 20);
        at(out, 0).write_fpu(c, two);
        set_reg(c, A1, 2);
        0x003c_d4b8
    };
    set_reg(c, A0, reader);
    if UTK_SKIP_BITS.call(c, ret).is_err() {
        return Resume::Unwind;
    }
    let pos = addu(reg(c, S0), reg(c, S3));
    set_reg(c, S0, pos);
    c.cycles += 3;
    let more = slt(pos, exc::SAMPLES);
    set_reg(c, V0, more);
    let next = addu(reg(c, S1), reg(c, S5));
    set_reg(c, S1, next);
    if more != 0 {
        if back_edge_unwinds(c, exc::PULSE) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::Pulse);
    }
    c.cycles += 1;
    EXC_FRAME.restore(c, S0, 0x90);
    Resume::Go(Exc::Exit)
}

/// The epilogue (s0 already restored).
pub(crate) fn exc_exit(c: &mut Cpu) {
    let frame = EXC_FRAME;
    c.cycles += 13;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    frame.restore(c, FP, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 21, 0xa8);
    frame.restore_fpr(c, 20, 0xa0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// EA-XA.

/// `SND_EaxaDecodeRawBlock(EaxaState*)`: an uncompressed (0xee) block.
pub(crate) const EAXA_DECODE_RAW_BLOCK: Callee = Callee {
    addr: 0x003c_c938,
    f: ee::s14::SND_EaxaDecodeRawBlock,
};

/// The predictor coefficient table (`float c0[4..]`, `c1 = c0 + 4`;
/// `0x450000 - 0x1760`).
pub(crate) const EAXA_COEFS: u32 = 0x0044_e8a0;

/// Samples per block.
pub(crate) const EAXA_BLOCK_SAMPLES: i32 = 0x1c;

/// `void SND_EaxaDecodeBlocks(EaxaState* s)` (0x3cca08): decode 28-sample
/// blocks while `s->left > 0`. A block is a header byte (high nibble: the
/// predictor, low nibble: the shift) and 14 bytes of nibbles, high first:
///
/// ```text
/// y = (nibble << 28 >> (shift + 16)) + h1 * c0[p] + h2 * c1[p]
/// ```
///
/// (ADDA/MADDA/MADD: the nibble plus 0, then the two products, each
/// rounded). A 0xee header is a raw block (`SND_EaxaDecodeRawBlock`, with
/// a0..t4 kept in a 0x30 stack frame across the call). The state is written
/// back after every block. Frame 0x10 holding `ra` (`sw`, reloaded with
/// `lq`).
pub fn snd_eaxa_decode_blocks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let st = reg(c, A0);
        set_reg(c, V0, sx(EAXA_COEFS));
        set_reg(c, V1, st);
        set_reg(c, A1, sx(EAXA_COEFS));
        Frame::open(c, 0x10);
        let ra = reg(c, RA);
        Addr(sp(c)).sw(c, ra);
        let left = at(st, eaxa::LEFT).lw(c);
        set_reg(c, A2, left);
        c.f[10] = at(st, eaxa::HIST1).read_fpu(c).0;
        c.f[11] = at(st, eaxa::HIST2).read_fpu(c).0;
        let input = at(st, eaxa::INPUT).lw(c);
        set_reg(c, A3, input);
        let output = at(st, eaxa::OUTPUT).lw(c);
        set_reg(c, T0, output);
        loop {
            // Loop head 0x3cca38 (a2 left, a3 input, t0 output, f10/f11
            // the history).
            c.cycles += 2;
            let st = reg(c, A0);
            let left = reg(c, A2);
            if (left as i64) <= 0 {
                c.cycles += 9;
                at(st, eaxa::LEFT).sw(c, left);
                let (h1, h2) = (fpr(c, 10), fpr(c, 11));
                at(st, eaxa::HIST1).write_fpu(c, h1);
                at(st, eaxa::HIST2).write_fpu(c, h2);
                let (input, output) = (reg(c, A3), reg(c, T0));
                at(st, eaxa::INPUT).sw(c, input);
                at(st, eaxa::OUTPUT).sw(c, output);
                let frame_at = sp(c);
                c.lq(RA, frame_at);
                set_reg(c, SP, addiu(frame_at as u64, 0x10));
                c.pc = reg(c, RA) as u32;
                return Ok(());
            }
            c.cycles += 6;
            let input = reg(c, A3);
            let header = at(input, 0).lbu(c);
            set_reg(c, T1, header);
            set_reg(c, A2, addiu(left, -EAXA_BLOCK_SAMPLES));
            set_reg(c, T5, 0xee);
            let header2 = at(input, 0).lbu(c);
            set_reg(c, T6, header2);
            if header2 != 0xee {
                eaxa_decode_block(c);
            } else {
                eaxa_raw_block(c)?;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_ca38);
            }
        }
    })
}

/// The nibbles of input byte `p` as the original shifts them (t4 the high
/// sample, t3 the low one; returned converted to float).
pub(crate) fn eaxa_nibbles(
    c: &mut Cpu,
    input: u64,
    p: u32,
    hi_shift: u64,
    lo_shift: u64,
) -> (FpuF32, FpuF32) {
    let b = at(input, p).lb(c);
    let hi = sx(((b as u32) as i32 >> 4) as u32);
    let lo = sx((b as u32) << 28);
    let hi = sx((hi as u32) << ((hi_shift as u32) & 31));
    let lo = sx(((lo as u32) as i32 >> ((lo_shift as u32) & 31)) as u32);
    set_reg(c, T4, hi);
    set_reg(c, T3, lo);
    (
        FpuF32::from_i32(hi as u32 as i32),
        FpuF32::from_i32(lo as u32 as i32),
    )
}

/// One compressed block (256 cycles).
pub(crate) fn eaxa_decode_block(c: &mut Cpu) {
    c.cycles += 256;
    let st = reg(c, A0);
    let input = reg(c, A3);
    let out = reg(c, T0);
    let header = reg(c, T1);
    let predictor = sx((header as u32) >> 4);
    let shift = header & 0xf;
    let coefs = addu(sll(predictor, 2), reg(c, A1));
    let c0 = at(coefs, 0).read_fpu(c);
    let c1 = at(coefs, 0x10).read_fpu(c);
    let hi_shift = subu(0xc, shift);
    set_reg(c, T2, hi_shift);
    let lo_shift = addiu(shift, 0x10);
    set_reg(c, T1, lo_shift);
    let zero = FpuF32::ZERO;
    // The nibble pair of byte p + 1 is in (f2, f3) for even p, (f4, f5)
    // for odd p.
    let mut pairs = [
        eaxa_nibbles(c, input, 1, hi_shift, lo_shift),
        (fpr(c, 4), fpr(c, 5)),
    ];
    let mut h1 = fpr(c, 10);
    let mut h2 = fpr(c, 11);
    let mut acc = FpuF32::ZERO;
    for p in 0..14u32 {
        let (hi, lo) = pairs[(p % 2) as usize];
        acc = FpuF32::madd(hi + zero, h1, c0);
        h2 = FpuF32::madd(acc, h2, c1);
        let next = if p < 13 {
            Some(eaxa_nibbles(c, input, p + 2, hi_shift, lo_shift))
        } else {
            None
        };
        at(out, 8 * p).write_fpu(c, h2);
        acc = FpuF32::madd(lo + zero, h2, c0);
        h1 = FpuF32::madd(acc, h1, c1);
        if let Some(pair) = next {
            pairs[((p + 1) % 2) as usize] = pair;
        }
        at(out, 8 * p + 4).write_fpu(c, h1);
    }
    c.f[0] = c0.0;
    c.f[1] = c1.0;
    c.f[2] = pairs[0].0 .0;
    c.f[3] = pairs[0].1 .0;
    c.f[4] = pairs[1].0 .0;
    c.f[5] = pairs[1].1 .0;
    c.f[10] = h1.0;
    c.f[11] = h2.0;
    c.f[31] = zero.0;
    c.facc = acc.0;
    let input = addiu(input, 0xf);
    set_reg(c, A3, input);
    let out = addiu(out, 0x70);
    set_reg(c, T0, out);
    at(st, eaxa::HIST1).write_fpu(c, h1);
    at(st, eaxa::HIST2).write_fpu(c, h2);
    at(st, eaxa::INPUT).sw(c, input);
    at(st, eaxa::OUTPUT).sw(c, out);
}

/// Registers `SND_EaxaDecodeBlocks` keeps in a 0x30 frame across the raw
/// block call.
pub(crate) const EAXA_RAW_SAVES: [(u32, usize); 9] = [
    (0x20, A0),
    (0x1c, A1),
    (0x18, A2),
    (0x14, A3),
    (0x10, T0),
    (0xc, T1),
    (0x8, T2),
    (0x4, T3),
    (0x0, T4),
];

/// A raw block: `SND_EaxaDecodeRawBlock(s)`, then reload the state.
pub(crate) fn eaxa_raw_block(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    let frame = Frame::open(c, 0x30);
    for (off, r) in EAXA_RAW_SAVES {
        let v = reg(c, r);
        Addr(frame.at(c, off)).sw(c, v);
    }
    EAXA_DECODE_RAW_BLOCK.call(c, 0x003c_ca88)?;
    for (off, r) in EAXA_RAW_SAVES.iter().rev() {
        let v = Addr(frame.at(c, *off)).lw(c);
        set_reg(c, *r, v);
    }
    let top = addiu(reg(c, SP), 0x30);
    set_reg(c, SP, top);
    let st = reg(c, A0);
    c.f[10] = at(st, eaxa::HIST1).read_fpu(c).0;
    c.f[11] = at(st, eaxa::HIST2).read_fpu(c).0;
    let input = at(st, eaxa::INPUT).lw(c);
    set_reg(c, A3, input);
    let output = at(st, eaxa::OUTPUT).lw(c);
    set_reg(c, T0, output);
    Ok(())
}

/// `void SND_TickStub(void)` (0x3c2438): the audio system's per-frame tick
/// when no sequencer is installed.
pub fn snd_tick_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_DspResample_SetRate(SND_DspResample* r, int rate)` (0x3cb538):
/// store the resample rate at +0x1c.
pub fn snd_dsp_resample_set_rate(c: &mut Cpu) {
    c.cycles += 2;
    let r = abi::arg(c, 0);
    let rate = c.g[A1][0];
    Addr(r.wrapping_add(0x1c)).sw(c, rate);
    abi::ret(c);
}

/// `void SND_UtkSetMode(SND_Utk* u, int mode)` (0x3ce0d8): store the mode word
/// at +0xd64.
pub fn snd_utk_set_mode(c: &mut Cpu) {
    c.cycles += 2;
    let u = abi::arg(c, 0);
    let mode = c.g[A1][0];
    Addr(u.wrapping_add(0xd64)).sw(c, mode);
    abi::ret(c);
}

/// `int SNDSTRM_StopSlotUnsupported(void)` (0x3c6ce0): -15, the "not supported"
/// error.
pub fn sndstrm_stop_slot_unsupported(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((-15i32) as u32);
    abi::ret(c);
}

/// `void SND_EaxaSetHistory(SND_Eaxa* e, float* history)` (0x3cd190): copy the
/// first two history floats into the EAXA object.
pub fn snd_eaxa_set_history(c: &mut Cpu) {
    c.cycles += 5;
    let e = abi::arg(c, 0);
    let h = abi::arg(c, 1);
    c.lwc1(0, h);
    c.swc1(0, e.wrapping_add(0x98));
    c.lwc1(1, h.wrapping_add(0x4));
    let a = e.wrapping_add(0x9c);
    c.swc1(1, a);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SND_DspResample_Process: the hand-written EE DSP resampler.
//
// The caller sets up more than the standard ABI arguments: `s0`, `s1`, `s2`
// and `t7` are read as input pointers before the function ever writes them
// (this is hand-written assembly, and the generated body does not initialise
// them). The port therefore reads the same registers at the same points. The
// object (`a0`) holds the format at +0x1c, a "done" flag at +0x24, a sample
// count at +0x26 and a resample callback pointer at +0x8.

/// `int SND_DspResample_Process(DspObj *o, void *in, s32 *out, s32 end)`
/// (0x3cb540): copy `o->count` samples into the caller's output, or run the
/// MMI/VU0 block that resamples `o->count` samples into `[out, end)` at the
/// rate in `o->+0x20`.
pub fn snd_dsp_resample_process(c: &mut Cpu) {
    crate::call::guest(c, |c| {
        // Prologue (object at 0x30, input at 0x00, output cursor at 0x08;
        // 0x20 tracks the cursor for the fast path). `t2` keeps a2 while a2
        // receives a3.
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        let sp = c.g[SP][0] as u32;
        c.g[T2][0] = c.g[A2][0];
        c.sw(sp.wrapping_add(0x30), c.g[A0][0]);
        c.g[A2][0] = c.g[A3][0];
        c.sw(sp, c.g[A1][0]);
        c.g[V0][0] = 0x1_0000;
        c.sq(16, sp.wrapping_add(0xd0));
        c.sq(17, sp.wrapping_add(0xc0));
        c.g[T0][0] = c.g[A1][0];
        c.sq(18, sp.wrapping_add(0xb0));
        c.sq(19, sp.wrapping_add(0xa0));
        c.sq(20, sp.wrapping_add(0x90));
        c.sq(21, sp.wrapping_add(0x80));
        c.sq(22, sp.wrapping_add(0x70));
        c.sq(23, sp.wrapping_add(0x60));
        c.sq(30, sp.wrapping_add(0x50));
        c.sd(sp.wrapping_add(0x40), c.g[RA][0]);
        c.sw(sp.wrapping_add(0x4), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.sw(sp.wrapping_add(0x20), c.g[T2][0]);
        let other_format = c.g[V1][0] != c.g[V0][0];
        c.sw(sp.wrapping_add(0x8), c.g[A2][0]);

        if !other_format {
            // Fast path (format 0x10000): pass the input through.
            c.cycles += 3;
            let active = Addr(c.g[A0][0] as u32).off(0x24).lh(c);
            c.g[V0][0] = active;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
            if nonzero {
                // Copy lh(o->+0x26) samples from the object to the output.
                c.cycles += 7;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x26).lh(c);
                c.g[V0][0] = 0x4;
                c.g[A3][0] = Addr(c.g[A0][0] as u32).off(0x26).lhu(c);
                c.g[V0][0] = sx(4u32.wrapping_sub(c.g[V1][0] as u32));
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0;
                if !empty {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(sp.wrapping_add(0x30));
                    c.g[T1][0] = 0x4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28));
                    loop {
                        c.cycles += 17;
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 16);
                        c.g[T9][0] = c.lw(sp.wrapping_add(0x8));
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.swc1(0, c.g[T9][0] as u32);
                        c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x4));
                        c.sw(sp.wrapping_add(0x8), c.g[T9][0]);
                        c.g[FP][0] = c.lw(sp.wrapping_add(0x30));
                        c.g[V0][0] = Addr(c.g[FP][0] as u32).off(0x26).lh(c);
                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A3][0] = Addr(c.g[FP][0] as u32).off(0x26).lhu(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_b5e0);
                        }
                    }
                }
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x26).lh(c);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            }

            // Done check: no resample callback means the object is finished.
            c.cycles += 4;
            c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
            let no_callback = c.g[A0][0] == 0;
            c.g[V1][0] = c.lw(sp.wrapping_add(0x30));
            if !no_callback {
                c.cycles += 6;
                c.g[A2][0] = c.g[T2][0];
                let target = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = target;
                c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x1a).lbu(c);
                c.g[A3][0] = c.lw(sp.wrapping_add(0x8));
                crate::call::call_ptr(c, target as u32, 0x003c_b648)?;
                c.g[T0][0] = c.g[V0][0];
            }

            c.cycles += 2;
            let done = (c.g[T0][0] as i64) > 0;
            c.g[A2][0] = c.lw(sp.wrapping_add(0x30));
            if !done {
                c.cycles += 4;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x24).lh(c);
                let active = c.g[V0][0] != 0;
                c.g[A1][0] = c.lw(sp.wrapping_add(0x30));
                if !active {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh(Addr(c.g[A0][0] as u32).off(0x24).0, c.g[V0][0]);
                    c.g[A1][0] = c.lw(sp.wrapping_add(0x30));
                }
                c.cycles += 6;
                c.g[V0][0] = c.g[T0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x34), 0);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x28), 0);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), 0);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x30), 0);
                return epilogue(c);
            }

            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x24).lh(c);
            let finished = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw(sp);
            if finished {
                c.cycles += 4;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            } else {
                c.cycles += 9;
                c.g[V1][0] = 0x1;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x26).lh(c);
                c.sh(Addr(c.g[A2][0] as u32).off(0x24).0, c.g[V1][0]);
                c.g[T9][0] = c.lw(sp);
                c.g[FP][0] = c.lw(sp.wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[T9][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
            }
            c.cycles += 11;
            c.lwc1(1, c.g[A3][0] as u32);
            c.g[A1][0] = c.lw(sp.wrapping_add(0x30));
            c.g[V0][0] = c.lw(sp);
            c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x28));
            c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x2c));
            c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
            return epilogue(c);
        }

        // Second half: only reached when the format is not 0x10000.
        c.cycles += 8;
        c.g[T9][0] = c.lw(sp);
        c.g[FP][0] = c.lw(sp.wrapping_add(0x30));
        c.mult(0, 2, 3, 25);
        c.g[V1][0] = Addr(c.g[FP][0] as u32).off(0x20).lw(c);
        c.g[A0][0] = Addr(c.g[FP][0] as u32).off(0x24).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let active = c.g[A0][0] != 0;
        c.g[A1][0] = sx((c.g[V0][0] as u32) >> 16);
        if active {
            // Already running: copy the object's current 4-float frame out.
            c.cycles += 12;
            c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
            c.g[T9][0] = c.lw(sp.wrapping_add(0x20));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x28));
            c.swc1(0, c.g[T9][0] as u32);
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x2c));
            c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0x4));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
            c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x8));
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x34));
            c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0xc));
            c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x10));
            c.sw(sp.wrapping_add(0x20), c.g[T9][0]);
        } else {
            // Starting: `a1` (already `len + 4 - count`) samples to emit.
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[FP][0] as u32).off(0x26).lh(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0;
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if !empty {
                c.cycles += 2;
                c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw(sp.wrapping_add(0x30));
                    c.sw(sp.wrapping_add(0x20), c.g[V0][0]);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x26).lh(c);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
                    if !more {
                        c.cycles += 2;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_b730);
                    }
                }
            }
        }

        c.cycles += 2;
        let empty = (c.g[A1][0] as i64) <= 0;
        c.g[FP][0] = c.lw(sp.wrapping_add(0x30));
        if empty {
            c.cycles += 1;
            c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[FP][0] as u32).off(0x8).lw(c);
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
            } else {
                c.cycles += 7;
                c.g[T0][0] = Addr(c.g[FP][0] as u32).off(0x1a).lbu(c);
                let target = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = target;
                c.g[A3][0] = c.lw(sp.wrapping_add(0x20));
                crate::call::call_ptr(c, target as u32, 0x003c_b7b0)?;
                c.g[T0][0] = c.g[V0][0];
                let done = (c.g[T0][0] as i64) > 0;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[FP][0] as u32).off(0x24).lh(c);
                    let active = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
                    if !active {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        c.sh(Addr(c.g[FP][0] as u32).off(0x24).0, c.g[V0][0]);
                        c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
                    }
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0);
                    c.g[V0][0] = c.g[T0][0];
                    c.g[V1][0] = c.lw(sp.wrapping_add(0x30));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x28), 0);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x2c), 0);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x30), 0);
                    return epilogue(c);
                }
            }
        }

        // Emit the remaining samples.
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x24).lh(c);
        let active = c.g[V0][0] == 0;
        c.g[V0][0] = 0x4;
        if active {
            c.cycles += 3;
            c.g[A2][0] = c.lw(sp.wrapping_add(0x30));
            c.g[V0][0] = 0x1;
            c.sh(Addr(c.g[A2][0] as u32).off(0x24).0, c.g[V0][0]);
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x26).lh(c);
            c.g[A1][0] = c.lw(sp.wrapping_add(0x20));
            c.g[V0][0] = sx(4u32.wrapping_sub(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.sw(sp.wrapping_add(0x20), c.g[A1][0]);
        }

        // MMI/VU0 resample: 16-bit packed samples in, 4-float frames out.
        c.cycles += 29;
        c.g[T9][0] = c.lw(sp.wrapping_add(0x4));
        c.g[FP][0] = c.lw(sp);
        c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
        c.g[V1][0] = c.lw(sp.wrapping_add(0x8));
        c.g[A0][0] = c.g[T9][0];
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.g[V1][0];
        c.g[T2][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[T0][0] = sx((c.g[T2][0] as u32) << 2);
        c.pextlw(8, 8, 8);
        c.pextlw(8, 8, 8);
        c.g[T3][0] = sx((c.g[T2][0] as u32) << 1);
        c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.pextlw(9, 11, 0);
        c.pextlw(14, 12, 10);
        c.pextlw(9, 14, 9);
        c.paddw(10, 9, 8);
        c.paddw(11, 10, 8);
        c.paddw(12, 11, 8);
        c.psllw(8, 8, 2);
        c.g[T5][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
        c.pextlw(13, 13, 13);
        c.pextlw(13, 13, 13);
        c.g[T6][0] = 0x3780_0000;
        c.qmtc2(14, 1);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.mtsab(0, 4);
        loop {
            c.cycles += 129;
            c.paddw(14, 13, 9);
            c.paddw(15, 13, 10);
            c.paddw(16, 13, 11);
            c.paddw(17, 13, 12);
            c.paddw(13, 13, 8);
            c.pinteh(18, 0, 14);
            c.pinteh(19, 0, 15);
            c.pinteh(20, 0, 16);
            c.pinteh(21, 0, 17);
            c.qmtc2(18, 2);
            c.qmtc2(19, 3);
            c.qmtc2(20, 4);
            c.qmtc2(21, 5);
            c.vu0().itof(0, 0b1111, 2, 2);
            c.vu0().itof(0, 0b1111, 3, 3);
            c.pextlw(18, 6, 6);
            c.vu0().itof(0, 0b1111, 4, 4);
            c.vu0().itof(0, 0b1111, 5, 5);
            c.vu0().mul(0b1111, 2, 2, Src::Bc(1, X));
            c.vu0().mul(0b1111, 3, 3, Src::Bc(1, X));
            c.pextlw(18, 18, 18);
            c.vu0().mul(0b1111, 4, 4, Src::Bc(1, X));
            c.vu0().mul(0b1111, 5, 5, Src::Bc(1, X));
            c.psrlw(14, 14, 16);
            c.psrlw(15, 15, 16);
            c.psrlw(16, 16, 16);
            c.psrlw(17, 17, 16);
            c.psllw(14, 14, 2);
            c.psllw(15, 15, 2);
            c.psllw(16, 16, 2);
            c.psllw(17, 17, 2);
            c.paddw(14, 14, 18);
            c.paddw(15, 15, 18);
            c.paddw(16, 16, 18);
            c.paddw(17, 17, 18);
            c.qfsrv(18, 0, 14);
            c.g[S3][0] = Addr(c.g[T6][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[T6][0] as u32).off(0x4).lw(c);
            c.qfsrv(14, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[T6][0] >> 32;
            c.g[S7][0] = Addr(c.g[T6][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[T6][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 6);
            c.qmtc2(20, 10);
            c.qfsrv(18, 0, 15);
            c.g[S3][0] = Addr(c.g[T7][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[T7][0] as u32).off(0x4).lw(c);
            c.qfsrv(15, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[T7][0] >> 32;
            c.g[S7][0] = Addr(c.g[T7][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[T7][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 7);
            c.qmtc2(20, 11);
            c.qfsrv(18, 0, 16);
            c.g[S3][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            c.qfsrv(16, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[S0][0] >> 32;
            c.g[S7][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 8);
            c.qmtc2(20, 12);
            c.qfsrv(18, 0, 17);
            c.g[S3][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
            c.qfsrv(17, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[S1][0] >> 32;
            c.g[S7][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 9);
            c.qmtc2(20, 13);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
            c.vu0().sub(0b1111, 10, 10, Src::Vec(6));
            c.vu0().sub(0b1111, 11, 11, Src::Vec(7));
            c.vu0().sub(0b1111, 12, 12, Src::Vec(8));
            c.vu0().sub(0b1111, 13, 13, Src::Vec(9));
            c.vu0().mul(0b1111, 10, 10, Src::Vec(2));
            c.vu0().mul(0b1111, 11, 11, Src::Vec(3));
            c.vu0().mul(0b1111, 12, 12, Src::Vec(4));
            c.vu0().mul(0b1111, 13, 13, Src::Vec(5));
            c.vu0().add(0b1111, 10, 10, Src::Vec(6));
            c.vu0().add(0b1111, 11, 11, Src::Vec(7));
            c.vu0().add(0b1111, 12, 12, Src::Vec(8));
            c.vu0().add(0b1111, 13, 13, Src::Vec(9));
            c.sqc2(10, (c.g[A3][0] as u32).wrapping_sub(0x40));
            c.sqc2(11, (c.g[A3][0] as u32).wrapping_sub(0x30));
            c.sqc2(12, (c.g[A3][0] as u32).wrapping_sub(0x20));
            let more = c.g[A1][0] != c.g[A3][0];
            c.sqc2(13, (c.g[A3][0] as u32).wrapping_sub(0x10));
            if !more {
                c.cycles += 21;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[T5][0]);
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V0][0] = c.g[FP][0];
                c.g[A2][0] = c.lw(sp.wrapping_add(0x20));
                c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
                c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x26).lh(c);
                c.g[V1][0] = sx((c.g[A1][0] as u32) >> 16);
                c.g[T9][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lwc1(0, c.g[A3][0] as u32);
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x28));
                c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0x2c));
                c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x30));
                c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.sw((c.g[T9][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0x34));
                return epilogue(c);
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_b89c);
            }
        }
    })
}

/// The shared return: restore the callee-saved registers, drop the frame, go
/// back to `ra`. (The `v0` the caller reads is whatever the path left there.)
pub(crate) fn epilogue(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 12;
    let sp = c.g[SP][0] as u32;
    c.lq(16, sp.wrapping_add(0xd0));
    c.lq(17, sp.wrapping_add(0xc0));
    c.lq(18, sp.wrapping_add(0xb0));
    c.lq(19, sp.wrapping_add(0xa0));
    c.lq(20, sp.wrapping_add(0x90));
    c.lq(21, sp.wrapping_add(0x80));
    c.lq(22, sp.wrapping_add(0x70));
    c.lq(23, sp.wrapping_add(0x60));
    c.lq(30, sp.wrapping_add(0x50));
    c.g[RA][0] = c.ld(sp.wrapping_add(0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp.wrapping_add(0xe0));
    c.pc = ra;
    Ok(())
}

pub(crate) mod utk_frame {

    /// Non-zero: reduced bandwidth (only every other excitation sample is
    /// coded; the others are zeroed or interpolated).
    pub const REDUCED_BW: u32 = 0x0c;

    /// First RC index that makes the frame unvoiced (excitation decoded with
    /// the pulse model instead of the Huffman model).
    pub const VOICED_THRESHOLD: u32 = 0x10;

    /// `float[64]`: fixed-codebook gains.
    pub const FIXED_GAINS: u32 = 0x14;

    /// `float[12]`: reflection coefficients, stepped over the frame.
    pub const RC: u32 = 0x114;

    /// `float[324]`: adaptive codebook (the last 324 decoded samples).
    pub const ADAPT_CB: u32 = 0x174;

    /// `float[432]`: the frame's excitation (4 subframes of 108 samples).
    pub const DECODED: u32 = 0x684;

    /// `DECODED[108..]`: the 324 samples kept as the next adaptive codebook
    /// (copied 32 bytes at a time up to `KEEP_END`, then 16 more).
    pub const KEEP: u32 = 0x834;

    pub const KEEP_END: u32 = 0xd34;

    /// `D_0044E8E8`: the reflection-coefficient quantiser table (indices
    /// 0..63 for RC 0..3, 16..47 for RC 4..11).
    pub const RC_TABLE: u32 = 0x0044_e8e8;

    /// Frame (0x2e0) slots: the excitation scratch, the 12 RC deltas and
    /// the spilled pointers.
    pub const SIZE: u32 = 0x2e0;

    pub const EXC: u32 = 0x10;

    pub const DELTA: u32 = 0x1e0;

    pub const KEEP_PTR: u32 = 0x210;

    pub const RC_PTR: u32 = 0x214;

    pub const GAINS_PTR: u32 = 0x218;

    pub const DECODED_PTR: u32 = 0x21c;

    pub const KEEP_END_PTR: u32 = 0x220;
}

pub(crate) mod utk_frame_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const READ_BITS: Callee = Callee {
        addr: 0x003c_d1f8,
        f: ee::s14::SND_UtkReadBits,
    };

    pub const DECODE_EXCITATION: Callee = Callee {
        addr: 0x003c_d2b0,
        f: ee::s14::SND_UtkDecodeExcitation,
    };

    pub const INTERPOLATE: Callee = Callee {
        addr: 0x003c_d518,
        f: ee::s14::SND_UtkInterpolate,
    };

    pub const SYNTHESIZE: Callee = Callee {
        addr: 0x003c_d690,
        f: ee::s14::SND_UtkSynthesize,
    };
}

/// `addiu`/`addu` on a register value: 32-bit add, sign-extended.
#[inline(always)]
pub(crate) fn utk_add(v: u64, k: u32) -> u64 {
    sx((v as u32).wrapping_add(k))
}

/// `void SND_UtkDecodeFrame(SND_Utk* u)` (0x3cd878): decode one 432-sample
/// EA MicroTalk speech frame.
///
/// ```text
/// delta[j] = (RC_TABLE[bits(6 or 5)] - rc[j]) / 4         j = 0..11
/// voiced   = first index < voiced_threshold
/// 4 subframes i: lag = (i+2)*108 - bits(8); pitch = bits(4) / 15;
///   fixed = gains[bits(6)]; excitation (full, or every other sample with
///   the rest zeroed or interpolated and fixed halved);
///   decoded[i*108 + j] = pitch * adapt_cb[lag + j] + fixed * exc[j]
/// adapt_cb = decoded[108..432]
/// 4 times: rc += delta; synthesize 12 samples (the last time 33 blocks)
/// ```
///
/// Every register, every callee-saved value the callees spill and every
/// back-edge event check follows the generated body.
pub fn snd_utk_decode_frame(c: &mut Cpu) {
    guest(c, utk_decode_frame_body)
}

pub(crate) fn utk_decode_frame_body(c: &mut Cpu) -> crate::call::Flow {
    use crate::ps2f::FpuF32;
    use utk_frame as f;
    use utk_frame_callee as callee;

    let fmul = |a: u32, b: u32| (FpuF32(a) * FpuF32(b)).0;
    let fadd = |a: u32, b: u32| (FpuF32(a) + FpuF32(b)).0;
    let fsub = |a: u32, b: u32| (FpuF32(a) - FpuF32(b)).0;
    const QUARTER: u32 = 0x3e80_0000;

    // --- Prologue, first RC index.
    c.cycles += 38;
    let frame = Frame::open(c, f::SIZE);
    let u = c.g[A0][0];
    frame.save(c, S3, 0x290);
    frame.save(c, S0, 0x2c0);
    frame.save(c, S1, 0x2b0);
    frame.save(c, S2, 0x2a0);
    frame.save(c, S4, 0x280);
    frame.save(c, S6, 0x260);
    frame.save(c, FP, 0x240);
    frame.save_fpr(c, 20, 0x2d0);
    frame.save(c, S5, 0x270);
    frame.save(c, S7, 0x250);
    frame.save_ra(c, 0x230);
    frame.save_fpr(c, 21, 0x2d8);
    c.g[A1][0] = 6;
    c.g[S3][0] = u;
    let rc = utk_add(u, f::RC);
    c.g[V0][0] = rc;
    c.g[S2][0] = utk_add(u, f::RC + 4);
    c.g[S0][0] = 2;
    c.sw(frame.at(c, f::RC_PTR), rc);
    callee::READ_BITS.call(c, 0x003c_d8c8)?;

    // delta[0]; the first index also decides `voiced`.
    let first = c.g[V0][0];
    let table = sx(f::RC_TABLE);
    c.g[A0][0] = sx(((first as u32) << 2).wrapping_add(f::RC_TABLE));
    c.lwc1(2, utk_add(u, f::RC) as u32);
    c.g[AT][0] = QUARTER as u64;
    c.f[1] = QUARTER;
    c.lwc1(0, c.g[A0][0] as u32);
    c.g[S4][0] = table;
    let threshold = c.lw(utk_add(u, f::VOICED_THRESHOLD) as u32);
    c.g[V1][0] = threshold;
    c.g[FP][0] = sx(frame.at(c, f::DELTA));
    c.f[0] = fsub(c.f[0], c.f[2]);
    c.g[S1][0] = sx(frame.at(c, f::DELTA + 4));
    c.f[20] = c.f[1];
    c.g[S6][0] = ((first as i64) < (threshold as i64)) as u64;
    c.f[0] = fmul(c.f[0], c.f[1]);
    c.swc1(0, frame.at(c, f::DELTA));

    // delta[1..3]: 6-bit indices (s0 counts 2..0, s2 walks rc, s1 delta).
    loop {
        c.cycles += 14;
        c.g[A0][0] = u;
        c.g[A1][0] = 6;
        callee::READ_BITS.call(c, 0x003c_d91c)?;
        c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
        let entry = sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[S4][0] as u32));
        c.g[V0][0] = entry;
        let rc_j = c.g[S2][0];
        c.lwc1(1, rc_j as u32);
        c.g[S2][0] = utk_add(rc_j, 4);
        c.lwc1(0, entry as u32);
        c.f[0] = fmul(fsub(c.f[0], c.f[1]), c.f[20]);
        let delta_j = c.g[S1][0];
        c.swc1(0, delta_j as u32);
        c.g[S1][0] = utk_add(delta_j, 4);
        if (c.g[S0][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d910);
        }
    }

    // delta[4..11]: 5-bit indices into the table's second half.
    c.cycles += 18;
    c.g[V0][0] = 0x45_0000;
    c.g[A0][0] = utk_add(u, f::FIXED_GAINS);
    c.g[S4][0] = table;
    c.g[A1][0] = utk_add(u, f::DECODED);
    c.g[A2][0] = utk_add(u, f::KEEP);
    c.g[A3][0] = utk_add(u, f::KEEP_END);
    c.g[V0][0] = c.lw(frame.at(c, f::RC_PTR));
    c.g[S7][0] = utk_add(u, f::ADAPT_CB);
    c.g[AT][0] = QUARTER as u64;
    c.f[20] = QUARTER;
    c.g[S2][0] = utk_add(c.g[FP][0], 0x10);
    c.sw(frame.at(c, f::GAINS_PTR), c.g[A0][0]);
    c.g[S1][0] = utk_add(c.g[V0][0], 0x10);
    c.sw(frame.at(c, f::DECODED_PTR), c.g[A1][0]);
    c.g[S0][0] = 7;
    c.sw(frame.at(c, f::KEEP_PTR), c.g[A2][0]);
    c.sw(frame.at(c, f::KEEP_END_PTR), c.g[A3][0]);
    loop {
        c.cycles += 15;
        c.g[A0][0] = u;
        c.g[A1][0] = 5;
        callee::READ_BITS.call(c, 0x003c_d99c)?;
        c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
        let rc_j = c.g[S1][0];
        c.lwc1(1, rc_j as u32);
        c.g[S1][0] = utk_add(rc_j, 4);
        let entry =
            sx(((c.g[V0][0] as u32).wrapping_add(0x10) << 2).wrapping_add(c.g[S4][0] as u32));
        c.g[V0][0] = entry;
        c.lwc1(0, entry as u32);
        c.f[0] = fmul(fsub(c.f[0], c.f[1]), c.f[20]);
        let delta_j = c.g[S2][0];
        c.swc1(0, delta_j as u32);
        c.g[S2][0] = utk_add(delta_j, 4);
        if (c.g[S0][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d990);
        }
    }

    // --- Four subframes (s4 = i, s1 = i + 1 once decoded).
    c.cycles += 3;
    c.g[S4][0] = 0;
    c.g[A1][0] = 8;
    loop {
        // Pitch lag (s5 = (i + 2) * 108 - bits(8)) and pitch gain bits(4) / 15
        // (converted as unsigned: a value with the top bit set is halved,
        // converted and doubled).
        c.cycles += 11;
        c.g[A0][0] = u;
        callee::READ_BITS.call(c, 0x003c_d9e0)?;
        c.g[V1][0] = utk_add(c.g[S4][0], 2);
        c.g[A1][0] = 0x6c;
        c.mult(0, V1, V1, A1);
        c.g[A0][0] = u;
        c.g[A1][0] = 4;
        c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        callee::READ_BITS.call(c, 0x003c_d9fc)?;
        let gain_bits = c.g[V0][0];
        c.g[V1][0] = sx((gain_bits as u32) >> 1);
        if (gain_bits as i64) < 0 {
            c.cycles += 5;
            let halved = (gain_bits & 1) | c.g[V1][0];
            c.g[V0][0] = halved;
            let v = FpuF32::from_i32(halved as u32 as i32);
            c.f[1] = (v + v).0;
        } else {
            c.cycles += 4;
            c.f[1] = FpuF32::from_i32(gain_bits as u32 as i32).0;
        }

        // Fixed gain gains[bits(6)] in f20, pitch gain (* 1/15) in f21.
        c.cycles += 13;
        c.g[AT][0] = 0x3d88_8889;
        c.f[0] = 0x3d88_8889;
        c.g[A0][0] = u;
        c.g[A1][0] = 6;
        c.f[21] = fmul(c.f[1], c.f[0]);
        callee::READ_BITS.call(c, 0x003c_da44)?;
        let gains = c.lw(frame.at(c, f::GAINS_PTR));
        c.g[A2][0] = gains;
        let gain_at = sx(((c.g[V0][0] as u32) << 2).wrapping_add(gains as u32));
        c.g[V0][0] = gain_at;
        let reduced = c.lw(utk_add(u, f::REDUCED_BW) as u32);
        c.g[V1][0] = reduced;
        c.lwc1(20, gain_at as u32);

        if reduced == 0 {
            // Full excitation at EXC + 4.
            c.cycles += 7;
            c.g[A0][0] = u;
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = sx(frame.at(c, f::EXC + 4));
            c.g[A3][0] = 1;
            callee::DECODE_EXCITATION.call(c, 0x003c_da70)?;
            c.g[S1][0] = utk_add(c.g[S4][0], 1);
            c.cycles += 1;
        } else {
            // Every other sample (phase `align`), then zero or interpolate
            // the others.
            c.cycles += 17;
            c.g[A0][0] = u;
            c.g[A1][0] = 1;
            callee::READ_BITS.call(c, 0x003c_da84)?;
            let align = c.g[V0][0];
            c.g[S2][0] = align;
            c.g[A0][0] = u;
            c.g[A1][0] = 1;
            c.g[S1][0] = sx((align as u32) << 2);
            callee::READ_BITS.call(c, 0x003c_da98)?;
            let out = utk_add(c.g[S1][0], 0x14);
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = sx(sp(c).wrapping_add(out as u32));
            c.g[A0][0] = u;
            c.g[A1][0] = c.g[S6][0];
            c.g[A3][0] = 2;
            callee::DECODE_EXCITATION.call(c, 0x003c_dab4)?;
            c.g[V1][0] = utk_add(c.g[S2][0], 0xffff_fffa);
            if c.g[S0][0] != 0 {
                // Zero the uncoded samples (54, walking down).
                c.cycles += 5;
                c.g[S1][0] = utk_add(c.g[S4][0], 1);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[S0][0] = 0x6a;
                c.g[V0][0] = utk_add(sx(sp(c).wrapping_sub(c.g[V1][0] as u32)), 0x1a8);
                loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_fffe);
                    c.g[V0][0] = utk_add(c.g[V0][0], 0xffff_fff8);
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_dad0);
                    }
                }
                c.cycles += 2;
            } else {
                // Clear 5 samples on each side, interpolate, halve the gain.
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = 4;
                c.g[S1][0] = utk_add(c.g[S4][0], 1);
                c.g[V0][0] = sx(frame.at(c, f::EXC));
                loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
                    c.g[V0][0] = utk_add(c.g[V0][0], 0xffff_fffc);
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_db08);
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = sx(frame.at(c, 0x1c4));
                c.g[S0][0] = 4;
                loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
                    c.g[V0][0] = utk_add(c.g[V0][0], 4);
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_db30);
                    }
                }
                c.cycles += 6;
                let back = utk_add(c.g[A0][0], 0xffff_ffe8);
                c.g[A0][0] = sx(sp(c).wrapping_sub(back as u32));
                callee::INTERPOLATE.call(c, 0x003c_db58)?;
                c.g[AT][0] = 0x3f00_0000;
                c.f[0] = 0x3f00_0000;
                c.f[20] = fmul(c.f[20], c.f[0]);
                c.cycles += 1;
            }
        }
        c.g[A3][0] = 0x6c;

        // decoded[i*108 + j] = pitch * adapt_cb[lag + j] + fixed * exc[j].
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
        c.mult(0, V1, S4, A3);
        let decoded = c.lw(frame.at(c, f::DECODED_PTR));
        c.g[A2][0] = decoded;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
        c.g[A0][0] = sx(frame.at(c, f::EXC + 4));
        c.g[S0][0] = 0x6b;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(decoded as u32));
        loop {
            c.cycles += 11;
            let past = c.g[A1][0];
            let exc = c.g[A0][0];
            c.lwc1(0, past as u32);
            c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
            c.lwc1(1, exc as u32);
            c.g[A1][0] = utk_add(past, 4);
            c.f[0] = fmul(c.f[21], c.f[0]);
            c.g[A0][0] = utk_add(exc, 4);
            c.f[1] = fmul(c.f[20], c.f[1]);
            c.f[0] = fadd(c.f[0], c.f[1]);
            let out = c.g[V0][0];
            c.swc1(0, out as u32);
            c.g[V0][0] = utk_add(out, 4);
            if (c.g[S0][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_db88);
            }
        }
        c.cycles += 4;
        c.g[S4][0] = c.g[S1][0];
        let more = (c.g[S4][0] as i64) < 4;
        c.g[V0][0] = more as u64;
        c.g[A1][0] = 8;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d9d8);
        }
    }

    // --- adapt_cb = decoded[108..432] (0x510 bytes; doubleword copy when
    // both are 8-aligned, else ldl/ldr + sdl/sdr).
    c.cycles += 6;
    c.g[V1][0] = c.lw(frame.at(c, f::KEEP_PTR));
    c.g[A0][0] = c.g[S7][0];
    c.g[V0][0] = (c.g[V1][0] | c.g[A0][0]) & 7;
    if c.g[V0][0] == 0 {
        loop {
            c.cycles += 12;
            let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
            c.g[V0][0] = c.ld(src);
            c.g[A1][0] = c.ld(src.wrapping_add(8));
            c.g[A2][0] = c.ld(src.wrapping_add(0x10));
            c.g[A3][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[V0][0]);
            c.sd(dst.wrapping_add(8), c.g[A1][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x18), c.g[A3][0]);
            c.g[V0][0] = c.lw(frame.at(c, f::KEEP_END_PTR));
            c.g[V1][0] = utk_add(c.g[V1][0], 0x20);
            c.g[A0][0] = utk_add(c.g[A0][0], 0x20);
            if c.g[V1][0] == c.g[V0][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_dc34);
            }
        }
    } else {
        loop {
            c.cycles += 20;
            let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
            for (reg, off) in [(A3, 0u32), (V0, 8), (A1, 0x10), (A2, 0x18)] {
                c.ldl(reg, src.wrapping_add(off + 7));
                c.ldr(reg, src.wrapping_add(off));
            }
            for (reg, off) in [(A3, 0u32), (V0, 8), (A1, 0x10), (A2, 0x18)] {
                c.sdl(reg, dst.wrapping_add(off + 7));
                c.sdr(reg, dst.wrapping_add(off));
            }
            c.g[A3][0] = c.lw(frame.at(c, f::KEEP_END_PTR));
            c.g[V1][0] = utk_add(c.g[V1][0], 0x20);
            c.g[A0][0] = utk_add(c.g[A0][0], 0x20);
            if c.g[V1][0] == c.g[A3][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_dbdc);
            }
        }
        c.cycles += 2;
    }
    c.cycles += 11;
    let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
    for (reg, off) in [(A1, 0u32), (A2, 8)] {
        c.ldl(reg, src.wrapping_add(off + 7));
        c.ldr(reg, src.wrapping_add(off));
    }
    for (reg, off) in [(A1, 0u32), (A2, 8)] {
        c.sdl(reg, dst.wrapping_add(off + 7));
        c.sdr(reg, dst.wrapping_add(off));
    }

    // --- Four steps: rc += delta, then synthesize from sample `offset`.
    c.g[S0][0] = 0xb;
    c.g[V0][0] = c.lw(frame.at(c, f::RC_PTR));
    c.g[V1][0] = c.g[FP][0];
    const STEPS: [(u32, u64, u64, u32); 4] = [
        (0x003c_dc90, 0x00, 1, 0x003c_dcc4),
        (0x003c_dcd0, 0x0c, 1, 0x003c_dd04),
        (0x003c_dd10, 0x18, 1, 0x003c_dd44),
        (0x003c_dd50, 0x24, 0x21, 0x003c_dd80),
    ];
    for (k, &(head, offset, blocks, ret_to)) in STEPS.iter().enumerate() {
        loop {
            c.cycles += 8;
            let (rc_j, delta_j) = (c.g[V0][0], c.g[V1][0]);
            c.lwc1(0, rc_j as u32);
            c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
            c.lwc1(1, delta_j as u32);
            c.g[V1][0] = utk_add(delta_j, 4);
            c.f[0] = fadd(c.f[0], c.f[1]);
            c.swc1(0, rc_j as u32);
            c.g[V0][0] = utk_add(rc_j, 4);
            if (c.g[S0][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
        let last = k == STEPS.len() - 1;
        c.cycles += if last { 18 } else { 8 };
        c.g[A0][0] = u;
        c.g[A1][0] = offset;
        c.g[A2][0] = blocks;
        if !last {
            c.g[S0][0] = 0xb;
        }
        callee::SYNTHESIZE.call(c, ret_to)?;
        if !last {
            c.g[V0][0] = c.lw(frame.at(c, f::RC_PTR));
            c.g[V1][0] = c.g[FP][0];
        }
    }

    // --- Epilogue.
    frame.restore(c, S0, 0x2c0);
    frame.restore(c, S1, 0x2b0);
    frame.restore(c, S2, 0x2a0);
    frame.restore(c, S3, 0x290);
    frame.restore(c, S4, 0x280);
    frame.restore(c, S5, 0x270);
    frame.restore(c, S6, 0x260);
    frame.restore(c, S7, 0x250);
    frame.restore(c, FP, 0x240);
    frame.restore_ra(c, 0x230);
    frame.restore_fpr(c, 21, 0x2d8);
    frame.restore_fpr(c, 20, 0x2d0);
    frame.close(c);
    Ok(())
}

pub(crate) mod utk {

    /// Bit reader: next input byte, current byte, bits left in it.
    pub const BIT_PTR: u32 = 0x000;

    pub const BIT_BUF: u32 = 0x004;

    pub const BIT_COUNT: u32 = 0x008;

    /// `float[432]`: the last decoded frame.
    pub const DECODED: u32 = 0x684;

    /// Samples of `DECODED` not yet handed out (counted from its end).
    pub const AVAIL: u32 = 0xd44;

    /// The per-frame header after the frame's bits (mode 1).
    pub const HEADER: u32 = 0xd48;

    /// `float*`: where the next samples go.
    pub const OUT: u32 = 0xd4c;

    /// `s16` (stored big-endian in the stream): raw samples patched in.
    pub const PATCH_COUNT: u32 = 0xd50;

    /// `s16`: first `DECODED` index the raw samples replace.
    pub const PATCH_START: u32 = 0xd52;

    /// Samples left in the stream.
    pub const LEFT: u32 = 0xd54;

    /// Non-zero: the header carries a raw-sample patch (set by a 0xee tag).
    pub const PATCH_PENDING: u32 = 0xd60;

    /// 1: every frame is followed by a byte-aligned header.
    pub const MODE: u32 = 0xd64;

    /// Samples per frame.
    pub const FRAME: u32 = 0x1b0;
}

pub(crate) const UTK_DECODE_FRAME: Callee = Callee {
    addr: 0x003c_d878,
    f: ee::s14::SND_UtkDecodeFrame,
};

/// `memcpy` (called through the function table).
pub(crate) const MEMCPY: u32 = 0x0041_605c;

/// `int SND_UtkDecode(SND_Utk* u, float** out, int n)` (0x3cde68): hand out
/// up to `n` MicroTalk samples (no more than the stream has left): copy what
/// is left of the decoded frame to `*out`, decode further 432-sample frames
/// as needed. In mode 1 a byte-aligned header follows each frame: a pending
/// patch (`count` big-endian s16 samples replacing `DECODED[start..]`), then
/// a 0xee tag that announces a patch after the next frame; the bit reader
/// restarts on the byte after it. Returns the number of samples written
/// (0 when the stream is finished) and advances `LEFT`.
pub fn snd_utk_decode(c: &mut Cpu) {
    guest(c, utk_decode_body)
}

#[inline(always)]
pub(crate) fn fld(u: u64, off: u32) -> u32 {
    (u as u32).wrapping_add(off)
}

pub(crate) fn utk_decode_body(c: &mut Cpu) -> crate::call::Flow {
    use utk::*;

    c.cycles += 15;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S1, 0x90);
    frame.save(c, S0, 0xa0);
    let mut u = c.g[A0][0];
    c.g[S1][0] = u;
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, S7, 0x30);
    frame.save(c, FP, 0x20);
    frame.save_ra(c, 0x10);
    let left = c.lw(fld(u, LEFT));
    c.g[V0][0] = left;
    // s2: samples to take from the current frame.
    let mut take = c.lw(fld(u, AVAIL));
    c.g[S2][0] = take;

    if left == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        // s4 = min(n, left) (64-bit signed, as compared), s3 = still to write.
        c.cycles += 7;
        let mut total = left;
        let out = c.lw(c.g[A1][0] as u32);
        c.g[V1][0] = out;
        let n = c.g[A2][0];
        let lt = ((n as i64) < (total as i64)) as u64;
        c.g[V0][0] = lt;
        if lt != 0 {
            total = n;
        }
        c.g[S4][0] = total;
        c.sw(fld(u, OUT), out);
        let mut todo = total;
        c.g[S3][0] = todo;
        if (total as i64) > 0 {
            c.cycles += 6;
            c.g[S6][0] = FRAME as u64;
            c.g[S5][0] = 1;
            c.g[FP][0] = sx(fld(u, PATCH_START));
            c.g[S7][0] = sx(fld(u, PATCH_COUNT));
            c.g[A1][0] = c.lw(fld(u, AVAIL));
            loop {
                // Copy min(todo, avail) samples from the frame's tail.
                c.cycles += 18;
                let lt = ((todo as i64) < (take as i64)) as u64;
                c.g[V0][0] = lt;
                if lt != 0 {
                    take = todo;
                    c.g[S2][0] = take;
                }
                c.g[A0][0] = c.lw(fld(u, OUT));
                let first = FRAME.wrapping_sub(c.g[A1][0] as u32);
                let bytes = sx((take as u32) << 2);
                c.g[S0][0] = bytes;
                c.g[A2][0] = bytes;
                c.g[A1][0] = sx((first << 2).wrapping_add(u as u32).wrapping_add(DECODED));
                todo = sx((todo as u32).wrapping_sub(take as u32));
                c.g[S3][0] = todo;
                call_ptr(c, MEMCPY, 0x003c_df0c)?;
                let out = sx((c.lw(fld(u, OUT)) as u32).wrapping_add(bytes as u32));
                c.g[V1][0] = out;
                let avail = sx((c.lw(fld(u, AVAIL)) as u32).wrapping_sub(take as u32));
                c.g[V0][0] = avail;
                c.sw(fld(u, OUT), out);
                c.sw(fld(u, AVAIL), avail);
                if (todo as i64) <= 0 {
                    break;
                }

                // Decode the next frame.
                c.cycles += 6;
                c.g[A0][0] = u;
                take = FRAME as u64;
                c.g[S2][0] = take;
                UTK_DECODE_FRAME.call(c, 0x003c_df34)?;
                let mode = c.lw(fld(u, MODE));
                c.g[V0][0] = mode;
                c.sw(fld(u, AVAIL), c.g[S6][0]);
                if mode == c.g[S5][0] {
                    utk_frame_header(c, u);
                }

                c.cycles += 2;
                if (todo as i64) > 0 {
                    c.g[A1][0] = c.lw(fld(u, AVAIL));
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_dee0);
                        u = c.g[S1][0];
                        todo = c.g[S3][0];
                        take = c.g[S2][0];
                    }
                } else {
                    break;
                }
            }
            total = c.g[S4][0];
        }
        // LEFT -= written; return written.
        c.cycles += 4;
        let rest = sx((c.lw(fld(u, LEFT)) as u32).wrapping_sub(total as u32));
        c.g[V1][0] = rest;
        c.g[V0][0] = total;
        c.sw(fld(u, LEFT), rest);
    }

    c.cycles += 12;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore(c, FP, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// Mode 1, after a frame: apply a pending raw-sample patch from the header
/// at `BIT_PTR - 1`, check the 0xee tag after it and restart the bit reader
/// on the following byte.
pub(crate) fn utk_frame_header(c: &mut Cpu, u: u64) {
    use utk::*;

    c.cycles += 6;
    let ptr = c.lw(fld(u, BIT_PTR));
    c.g[A0][0] = ptr;
    let pending = c.lw(fld(u, PATCH_PENDING));
    c.g[V1][0] = pending;
    let header = sx((ptr as u32).wrapping_sub(1));
    c.g[V0][0] = header;
    c.g[A2][0] = header;
    c.sw(fld(u, HEADER), header);
    if pending != 0 {
        // Header: start (BE s16), count (BE s16), count BE s16 samples.
        c.cycles += 22;
        let p = ptr as u32;
        let b = c.lbu(p);
        c.g[V1][0] = b;
        c.g[A2][0] = sx(p.wrapping_add(3));
        c.sb(fld(u, PATCH_START), b);
        let b = c.lbu(p.wrapping_sub(1));
        c.g[V0][0] = b;
        c.sb(fld(u, PATCH_START + 1), b);
        let b = c.lbu(p.wrapping_add(2));
        c.g[V1][0] = b;
        c.sb(fld(u, PATCH_COUNT), b);
        let b = c.lbu(p.wrapping_add(1));
        c.g[V0][0] = b;
        c.sb(fld(u, PATCH_COUNT + 1), b);
        let count = c.lh(fld(u, PATCH_COUNT));
        c.g[A1][0] = count;
        let header = c.lw(fld(u, HEADER)) as u32;
        let start = c.lh(fld(u, PATCH_START));
        let tag = header.wrapping_add((count as u32) << 1);
        let next = sx(tag.wrapping_add(5));
        c.g[V0][0] = next;
        let dst_off = sx(((start as u32) << 2).wrapping_add(DECODED));
        c.g[A0][0] = dst_off;
        let tag = sx(tag.wrapping_add(4));
        c.g[V1][0] = tag;
        c.sw(fld(u, HEADER), tag);
        c.g[A3][0] = sx((u as u32).wrapping_add(dst_off as u32));
        c.sw(fld(u, BIT_PTR), next);
        if (count as i64) > 0 {
            c.cycles += 2;
            c.g[A1][0] = 0x10000;
            c.g[T0][0] = 0x10000;
            loop {
                // decoded[start + i] = (float)(s16 big-endian sample i);
                // a1 = (i + 1) << 16.
                c.cycles += 16;
                let src = c.g[A2][0] as u32;
                let lo = c.lbu(src.wrapping_add(1));
                c.g[V0][0] = lo;
                let k = c.g[A1][0] as u32;
                let i = sx(((k as i32) >> 16) as u32);
                c.g[A0][0] = i;
                c.g[A1][0] = sx(k.wrapping_add(c.g[T0][0] as u32));
                let tmp = sp(c);
                c.sb(tmp, lo);
                let hi = c.lbu(src);
                c.g[V1][0] = hi;
                c.g[A2][0] = sx(src.wrapping_add(2));
                c.sb(tmp.wrapping_add(1), hi);
                c.g[V0][0] = c.lh(tmp);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A3][0] as u32);
                let u = c.g[S1][0];
                let count = c.lh(fld(u, PATCH_COUNT));
                c.g[V0][0] = count;
                let more = ((i as i64) < (count as i64)) as u64;
                c.g[A0][0] = more;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_dfb8);
                }
            }
        }
        c.cycles += 1;
        c.sw(fld(c.g[S1][0], PATCH_PENDING), 0);
    }

    // A 0xee tag after the header: the next frame's header has a patch.
    c.cycles += 5;
    let header = c.lw(fld(u, HEADER));
    c.g[V0][0] = header;
    c.g[A0][0] = 0xee;
    let tag = c.lbu(header as u32);
    c.g[V1][0] = tag;
    if tag == 0xee {
        c.sw(fld(u, PATCH_PENDING), c.g[S5][0]);
    }

    // Restart the bit reader on the next byte.
    c.cycles += 7;
    c.g[V0][0] = 8;
    let p = c.lw(fld(u, BIT_PTR));
    c.g[V1][0] = p;
    c.sw(fld(u, BIT_COUNT), 8);
    let next = sx((p as u32).wrapping_add(1));
    c.g[A0][0] = next;
    let byte = c.lbu(p as u32);
    c.g[V0][0] = byte;
    c.sw(fld(u, BIT_PTR), next);
    c.sw(fld(u, BIT_BUF), byte);
}

pub(crate) mod mixer {

    pub const BASE: u32 = 0x0051_5b40;

    pub const VOICE_COUNT: u32 = 0x4;

    pub const BUS_COUNT: u32 = 0x5;

    /// The declick bus enable byte.
    pub const DECLICK_ON: u32 = 0xf;

    pub const POST_MIX: u32 = 0x10;

    pub const SCRATCH: u32 = 0xb0;

    pub const SAMPLES: u32 = 0xb4;

    /// Bus buffers, then their declick gains at +0x18 from each.
    pub const BUS_BUF: u32 = 0xd0;

    pub const BUS_DECLICK: u32 = 0xe8;

    /// Per bus: an effect and the output stage (relative to BUS_BUF).
    pub const BUS_FX: u32 = 0x4c;

    pub const BUS_OUT: u32 = 0x34;

    pub const DECLICK_GAIN: u32 = 0x100;

    pub const VOICES: u32 = 0x1dc;

    pub const MIX_FN: u32 = 0x1e0;

    /// The declick bus buffer.
    pub const DECLICK_BUF: u32 = 0x0051_5d80;

    /// Voices: 0x60 bytes; state byte (2 = playing), "gains changed" byte,
    /// send gains at +0x1c (per bus) and +0x38 (declick), last sample at
    /// +0x3c, the source object at +0x40.
    pub const VOICE_SIZE: u64 = 0x60;
}

// ---------------------------------------------------------------------------
// SND_DecEaxa_DecodeStream (0x3ca460, EA SND library)

pub(crate) const SNDSTRM_RELEASE_BYTES: u32 = 0x003c_7010;

pub(crate) const SNDSTRM_POP_BLOCK: u32 = 0x003c_6de0;

pub(crate) const MEMSET: u32 = 0x0041_6210;

pub(crate) const SND_EAXA_DECODE: Callee = Callee {
    addr: 0x003c_cf90,
    f: ee::s14::SND_EaxaDecode,
};

pub(crate) const SND_EAXA_SET_HISTORY: Callee = Callee {
    addr: 0x003c_d190,
    f: ee::s14::SND_EaxaSetHistory,
};

pub(crate) const SND_EAXA_SET_INPUT: Callee = Callee {
    addr: 0x003c_cf60,
    f: ee::s14::SND_EaxaSetInput,
};

/// The EA-XA stream decoder's fields: its ADPCM decoder, the current block
/// (sample data), the block's sample count, samples decoded from it, the
/// codec version (<= 2: blocks start with two history samples), the
/// stream and channel, and the samples not yet released to the stream.
pub(crate) const DX_DECODER: u32 = 0x1c;

pub(crate) const DX_BLOCK: u32 = 0x20;

pub(crate) const DX_BLOCK_SAMPLES: u32 = 0x24;

pub(crate) const DX_BLOCK_POS: u32 = 0x28;

pub(crate) const DX_VERSION: u32 = 0x2c;

pub(crate) const DX_STREAM: u32 = 0x30;

pub(crate) const DX_UNRELEASED: u32 = 0x34;

pub(crate) const DX_CHANNEL: u32 = 0x38;

/// `int SND_DecEaxa_DecodeStream(DecEaxa* d, int n, int, short** out)`
/// (0x3ca460, the output cursor is passed through the frame): releases the
/// samples consumed by the last call to the stream, then decodes `n`
/// samples: whatever the ADPCM decoder still holds, then block after block
/// popped from the stream (`SNDSTRM_PopBlock`; old-version blocks seed the
/// decoder history from their first two samples). When the stream runs dry
/// the rest of the output is cleared (only if something was decoded since
/// the last release). Returns the number of samples decoded.
pub fn snd_dec_eaxa_decode_stream(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0]; // d
        c.g[S1][0] = c.g[A1][0]; // n
        c.g[S2][0] = 0; // seed the history
        c.g[S3][0] = 0; // decoded
        let d = r32(c, S0);
        let at = |o: u32| Addr(d.wrapping_add(o));
        let out = frame.at(c, 0x10);
        c.g[A2][0] = at(DX_UNRELEASED).lw(c);
        let a3 = c.g[A3][0];
        Addr(out).sw(c, a3);
        if c.g[A2][0] != 0 {
            // SNDSTRM_ReleaseBytes(stream, channel, unreleased)
            c.cycles += 4;
            c.g[A0][0] = at(DX_STREAM).lw(c);
            c.g[A1][0] = at(DX_CHANNEL).lbu(c);
            call_ptr(c, SNDSTRM_RELEASE_BYTES, 0x003c_a4a4)?;
            at(DX_UNRELEASED).sw(c, 0);
        }

        c.cycles += 2;
        c.g[S4][0] = 1;
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A0][0] = at(DX_DECODER).lw(c);
            loop {
                // k = SND_EaxaDecode(decoder, &out, n)
                c.cycles += 17;
                c.g[A1][0] = sx(out);
                c.g[A2][0] = c.g[S1][0];
                SND_EAXA_DECODE.call(c, 0x003c_a4c4)?;
                let k = r32(c, V0);
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = Addr(out).lw(c);
                c.g[V0][0] = at(DX_BLOCK_POS).lw(c);
                c.g[A1][0] = sx(k << 2);
                c.g[V1][0] = at(DX_UNRELEASED).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(k << 2));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(k));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(k));
                let v = c.g[V0][0];
                at(DX_BLOCK_POS).sw(c, v);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(k));
                let a0 = c.g[A0][0];
                Addr(out).sw(c, a0);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let v1 = c.g[V1][0];
                at(DX_UNRELEASED).sw(c, v1);
                if c.g[V0][0] == 0 {
                    // Satisfied by the decoder alone.
                    c.cycles += 1;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(k));
                } else {
                    // The decoder ran dry: the next block.
                    c.cycles += 9;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(k));
                    c.g[A0][0] = at(DX_STREAM).lw(c);
                    c.g[A1][0] = at(DX_CHANNEL).lbu(c);
                    c.g[A2][0] = sx(d.wrapping_add(DX_BLOCK_SAMPLES));
                    c.g[A3][0] = sx(frame.at(c, 0x14));
                    call_ptr(c, SNDSTRM_POP_BLOCK, 0x003c_a514)?;
                    c.g[A2][0] = c.g[V0][0];
                    let block = c.g[A2][0];
                    at(DX_BLOCK).sw(c, block);
                    if block != 0 {
                        c.cycles += 6;
                        c.g[V1][0] = at(DX_VERSION).lw(c);
                        c.g[V0][0] = 2;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[S2][0] = c.g[S4][0];
                        }
                        c.g[A1][0] = c.g[SP][0];
                        if c.g[S2][0] != 0 {
                            // SND_EaxaSetHistory(decoder, {p[0], p[1]}); p += 2
                            c.cycles += 13;
                            let p = Addr(block as u32);
                            c.g[V0][0] = p.lh(c);
                            c.g[A0][0] = at(DX_DECODER).lw(c);
                            c.mtc1(V0, 0);
                            c.cvt_s_w(0, 0);
                            let a = frame.at(c, 0);
                            c.swc1(0, a);
                            c.g[V0][0] = p.off(2).lh(c);
                            c.mtc1(V0, 0);
                            c.cvt_s_w(0, 0);
                            let a = frame.at(c, 4);
                            c.swc1(0, a);
                            SND_EAXA_SET_HISTORY.call(c, 0x003c_a560)?;
                            c.g[V0][0] = at(DX_BLOCK).lw(c);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                            let v = c.g[V0][0];
                            at(DX_BLOCK).sw(c, v);
                        }
                        // SND_EaxaSetInput(decoder, p, samples * 4, samples)
                        c.cycles += 9;
                        c.g[A2][0] = at(DX_BLOCK_SAMPLES).lw(c);
                        at(DX_BLOCK_POS).sw(c, 0);
                        c.g[A3][0] = c.g[A2][0];
                        c.g[A0][0] = at(DX_DECODER).lw(c);
                        c.g[A1][0] = at(DX_BLOCK).lw(c);
                        c.g[A2][0] = sx(r32(c, A2) << 2);
                        SND_EAXA_SET_INPUT.call(c, 0x003c_a588)?;
                    } else {
                        // The stream ran dry: clear the rest, stop.
                        c.cycles += 2;
                        if (c.g[S1][0] as i64) > 0 {
                            c.cycles += 3;
                            c.g[V0][0] = at(DX_UNRELEASED).lw(c);
                            c.g[A0][0] = Addr(out).lw(c);
                            if c.g[V0][0] != 0 {
                                // memset(out, 0, n * 4)
                                c.cycles += 3;
                                c.g[A2][0] = sx(r32(c, S1) << 2);
                                c.g[A1][0] = 0;
                                call_ptr(c, MEMSET, 0x003c_a5b0)?;
                            }
                            c.cycles += 2;
                        }
                        c.g[S1][0] = 0;
                    }
                }
                c.cycles += 2;
                if (c.g[S1][0] as i64) <= 0 {
                    break;
                }
                c.g[A0][0] = at(DX_DECODER).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_a4b8);
                }
            }
        }

        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_DecUtk_DecodeLoop (0x3c9520, EA SND library)

pub(crate) const SND_UTK_DECODE: Callee = Callee {
    addr: 0x003c_de68,
    f: ee::s14::SND_UtkDecode,
};

pub(crate) const SND_UTK_SET_STATE: Callee = Callee {
    addr: 0x003c_e0b8,
    f: ee::s14::SND_UtkSetState,
};

pub(crate) const SND_UTK_SET_INPUT: Callee = Callee {
    addr: 0x003c_dde8,
    f: ee::s14::SND_UtkSetInput,
};

/// The looping MicroTalk decoder's fields: its UTK decoder, the sample
/// position, the loop start and end samples, the stream data.
pub(crate) const DU_DECODER: u32 = 0x1c;

pub(crate) const DU_POS: u32 = 0x20;

pub(crate) const DU_LOOP_START: u32 = 0x24;

pub(crate) const DU_LOOP_END: u32 = 0x28;

pub(crate) const DU_DATA: u32 = 0x2c;

/// `int SND_DecUtk_DecodeLoop(DecUtk* d, int n, int, float* out)`
/// (0x3c9520, the output cursor is passed through the frame): decodes `n`
/// samples of a looping MicroTalk sound. Whenever the decoder runs out
/// before the request is met, the position jumps back to the loop start,
/// the decoder state is reset (`SND_UtkSetState({0, 0, 1})`) and the loop's
/// `end - start + 1` samples are queued again (`SND_UtkSetInput`). Returns
/// 1.
pub fn snd_dec_utk_decode_loop(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0]; // d
        c.g[S1][0] = c.g[A1][0]; // n
        let d = r32(c, S0);
        let at = |o: u32| Addr(d.wrapping_add(o));
        let out = frame.at(c, 0x10);
        let a3 = c.g[A3][0];
        Addr(out).sw(c, a3);
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S2][0] = 1;
            c.g[A0][0] = at(DU_DECODER).lw(c);
            loop {
                // k = SND_UtkDecode(decoder, &out, n)
                c.cycles += 13;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx(out);
                SND_UTK_DECODE.call(c, 0x003c_955c)?;
                let k = r32(c, V0);
                c.g[A0][0] = Addr(out).lw(c);
                c.g[V1][0] = sx(k << 2);
                c.g[A1][0] = at(DU_POS).lw(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(k));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(k << 2));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(k));
                let a0 = c.g[A0][0];
                Addr(out).sw(c, a0);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let a1 = c.g[A1][0];
                at(DU_POS).sw(c, a1);
                if c.g[V0][0] != 0 {
                    // Back to the loop start: reset and requeue the loop.
                    c.cycles += 18;
                    c.g[V0][0] = at(DU_LOOP_START).lw(c);
                    c.g[A2][0] = 0;
                    c.g[A0][0] = at(DU_DECODER).lw(c);
                    c.g[A1][0] = c.g[SP][0];
                    let start = c.g[V0][0];
                    at(DU_POS).sw(c, start);
                    Addr(frame.at(c, 4)).sw(c, 0);
                    Addr(frame.at(c, 0)).sw(c, 0);
                    let one = c.g[S2][0];
                    Addr(frame.at(c, 8)).sw(c, one);
                    SND_UTK_SET_STATE.call(c, 0x003c_95a8)?;
                    c.g[A2][0] = at(DU_LOOP_END).lw(c);
                    c.g[V0][0] = at(DU_LOOP_START).lw(c);
                    c.g[A0][0] = at(DU_DECODER).lw(c);
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
                    c.g[A1][0] = at(DU_DATA).lw(c);
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                    c.g[A3][0] = c.g[A2][0];
                    c.g[A2][0] = sx(r32(c, A2) << 2);
                    SND_UTK_SET_INPUT.call(c, 0x003c_95cc)?;
                }
                c.cycles += 2;
                if (c.g[S1][0] as i64) <= 0 {
                    break;
                }
                c.g[A0][0] = at(DU_DECODER).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_9550);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x003ba888 SND_CentsToPitch

/// The pitch fraction table of [`snd_cents_to_pitch`]: 256 bytes, the
/// fraction of a semitone step (`(2^(i/256) - 1) * 256`), at_2 0x44c480.
pub const CENTS_PITCH_TABLE: u32 = 0x0045_0000 - 0x3b80;

/// `int SND_CentsToPitch(int cents)` (0x3ba888): an SPU pitch (0x1000 =
/// unchanged) for a detune in cents. Whole octaves (1200 cents) double or
/// halve the pitch; the rest is scaled to 1/256 octave steps (`* 0x369d >>
/// 16`, at_2 least -255) and looked up in the fraction table (`(pitch *
/// (0x100 + frac)) >> 8`, one more shift for negative steps).
pub fn snd_cents_to_pitch(c: &mut Cpu) {
    const OCTAVE: u32 = 0x4b0;
    c.cycles += 4;
    c.g[A2][0] = 0x1000;
    c.g[A3][0] = 0x45_0000;
    // Whole octaves up (the first test uses the full register).
    if (c.g[A0][0] as i64) >= 1200 {
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(OCTAVE));
            c.g[A2][0] = sx(r32(c, A2) << 1);
            let below = (c.g[A0][0] as i64) < 1200;
            c.g[V0][0] = below as u64;
            if below {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a898);
            }
        }
        c.cycles += 2;
    } else {
        c.cycles += 1;
    }
    // Whole octaves down.
    loop {
        let low = (c.g[A0][0] as i64) < -1199;
        c.g[V0][0] = low as u64;
        c.cycles += 5;
        if !low {
            break;
        }
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(OCTAVE));
        if event_due(c) {
            deliver_event_here(c, 0x003b_a8c0);
        }
        c.cycles += 1;
        c.g[A2][0] = sx(((r32(c, A2) as i32) >> 1) as u32);
        c.cycles += 1;
    }
    // The 1/256-octave step.
    c.cycles += 9;
    c.g[V1][0] = sx(0xffff_ff00);
    let cents = r32(c, A0);
    let scaled = abi::mult(c, cents, 0x369d);
    c.g[V0][0] = sx(scaled);
    c.g[A0][0] = sx(0xffff_ff01);
    c.g[A1][0] = sx(((scaled as i32) >> 16) as u32);
    let above_min = (c.g[V1][0] as i64) < (c.g[A1][0] as i64);
    c.g[V1][0] = above_min as u64;
    if !above_min {
        c.g[A1][0] = c.g[A0][0];
    }
    let step = c.g[A1][0];
    c.g[V0][0] = sx(CENTS_PITCH_TABLE);
    let (entry, shift) = if (step as i64) >= 0 {
        c.cycles += 5;
        (CENTS_PITCH_TABLE.wrapping_add(step as u32), 8)
    } else {
        c.cycles += 6;
        (
            CENTS_PITCH_TABLE
                .wrapping_add(step as u32)
                .wrapping_add(0x100),
            9,
        )
    };
    let base = CENTS_PITCH_TABLE.wrapping_add(step as u32);
    c.g[V0][0] = sx(base);
    let frac = Addr(entry).lbu(c);
    let pitch = r32(c, A2);
    let product = abi::mult(c, pitch, (frac as u32).wrapping_add(0x100));
    c.g[V1][0] = sx(product);
    c.g[A2][0] = sx(((product as i32) >> shift) as u32);
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

// ===========================================================================
// 0x003c6380 SND_UMul32x32

/// `__muldi3` (64-bit multiply), the partial-product helper.
pub const MULDI3: u32 = 0x0041_14d0;

/// One partial product of [`snd_umul32x32`]: `__muldi3(x, y)` on the
/// registers `x` and `y`, returning to `ret`.
pub(crate) fn umul_part(
    c: &mut Cpu,
    x: usize,
    y: usize,
    ret: u32,
) -> Result<u64, crate::call::Unwound> {
    c.g[A0][0] = c.g[x][0];
    c.g[A1][0] = c.g[y][0];
    call_ptr(c, MULDI3, ret)?;
    Ok(c.g[V0][0])
}

/// `u64 SND_UMul32x32(u32 a, u32 b)` (0x3c6380): the full 64-bit product of
/// two 32-bit values from four 16x16 partial products (each through
/// `__muldi3`): `lo = al*bl`, `m1 = al*bh`, `m2 = ah*bl`, `hi = ah*bh`;
/// the low word `lo + (m1 << 16) + (m2 << 16)`, the high word `hi + (m1 >>
/// 16) + (m2 >> 16)` plus the carry out of the middle 16 bits. The two
/// words meet in a doubleword on the stack (copied once) and come back in
/// v0.
pub fn snd_umul32x32(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 74;
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0x20);
        let (a, b) = (c.g[A0][0], c.g[A1][0]);
        // The 16-bit halves (s0 = al, s2 = bl, s3 = ah, s4 = bh).
        c.g[S0][0] = a & 0xffff;
        c.g[S2][0] = b & 0xffff;
        c.g[S3][0] = ((a as u32) >> 16) as u64;
        c.g[S4][0] = ((b as u32) >> 16) as u64;

        // The partial products (64-bit results in v0).
        let mul = umul_part;
        let lo = mul(c, S0, S2, 0x003c_63c8)?;
        c.g[S1][0] = lo;
        let m1 = mul(c, S0, S4, 0x003c_63d8)?;
        c.g[S0][0] = m1;
        let m2 = mul(c, S3, S2, 0x003c_63e8)?;
        c.g[S2][0] = m2;
        let hi = mul(c, S3, S4, 0x003c_63f8)?;

        // The carry out of bits 16..31 and the two words.
        c.g[A2][0] = m2 >> 16;
        let carry = ((lo >> 16)
            .wrapping_add(m1 & 0xffff)
            .wrapping_add(m2 & 0xffff))
            >> 16;
        let high = (m1 >> 16)
            .wrapping_add(m2 >> 16)
            .wrapping_add(hi)
            .wrapping_add(carry);
        let low = lo.wrapping_add(m1 << 16).wrapping_add(m2 << 16);
        let (high, low) = (sx(high as u32), sx(low as u32));
        c.g[S0][0] = m1 << 16;
        c.g[S1][0] = low;
        c.g[S2][0] = m2 << 16;
        let words = frame.at(c, 0);
        Addr(words).off(4).sw(c, high);
        Addr(words).sw(c, low);
        let both = Addr(words).read_u64_unaligned(c);
        Addr(words).off(0x10).write_u64_unaligned(c, both);
        let low_word = Addr(words).off(0x10).lwu(c);
        let high_word = Addr(words).off(0x14).lwu(c);
        c.g[A0][0] = low_word;
        c.g[A1][0] = 0xffff_ffff;
        c.g[V1][0] = high_word << 32;
        c.g[V0][0] = (hi & 0xffff_ffff_0000_0000 | low_word) & 0xffff_ffff | high_word << 32;

        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const SND_EAXA_DECODE_BLOCKS: Callee = Callee {
    addr: 0x003c_ca08,
    f: ee::s14::SND_EaxaDecodeBlocks,
};

/// `int SND_EaxaDecode(EaxaState* s, float** pout, int n)` (0x3ccf90,
/// `src/ealib/seg_2B4578.c`): decodes up to `n` samples of an EA ADPCM
/// stream to `*pout`; returns how many were written.
///
/// First the samples left over from the last partial block are copied
/// out, then whole 28-sample blocks are decoded straight to the output
/// (`SND_EaxaDecodeBlocks`), and a remainder is decoded as one block into
/// `s+0x24` and its first samples copied; the rest stays pending
/// (`s+0x4` negated, read pointer `s+0x10`). The stream's remaining count
/// (`s+0x8`) goes down by every sample delivered; when it runs out nothing
/// stays pending.
///
/// The state pointer lives in `s0`, the delivered count in `s3` and the
/// counters in `s1`/`s2` as in the original (`SND_EaxaDecodeBlocks` saves
/// them on its stack), and every block's cycle charge and scratch register
/// is reproduced.
pub fn snd_eaxa_decode(c: &mut Cpu) {
    use eaxa::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = 0;
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let s = r32(c, S0);
        let out = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = out;
        let remaining = Addr(s + REMAINING).lw(c);
        c.g[V1][0] = remaining;
        Addr(s + OUT).sw(c, out);
        if remaining == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            // m = min(n, remaining) (a1).
            c.cycles += 5;
            c.g[A1][0] = remaining;
            let n_lt = ((c.g[A2][0] as i64) < (remaining as i64)) as u64;
            c.g[V0][0] = n_lt;
            let pending = Addr(s + PENDING).lw(c);
            c.g[V1][0] = pending;
            if n_lt != 0 {
                c.g[A1][0] = c.g[A2][0];
            }
            if pending != 0 {
                // k = min(m, pending) (a0, s1): copy k pending samples.
                c.cycles += 5;
                let m_lt = ((c.g[A1][0] as i64) < (pending as i64)) as u64;
                c.g[V0][0] = m_lt;
                c.g[A0][0] = if m_lt != 0 { c.g[A1][0] } else { pending };
                c.g[S1][0] = c.g[A0][0];
                if (c.g[A0][0] as i64) > 0 {
                    c.cycles += 1;
                    loop {
                        c.cycles += 12;
                        let s = r32(c, S0);
                        let rd = Addr(s + READ).lw(c) as u32;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                        let dst = Addr(s + OUT).lw(c) as u32;
                        c.lwc1(0, rd);
                        c.swc1(0, dst);
                        let dst = Addr(s + OUT).lw(c) as u32;
                        let rd = Addr(s + READ).lw(c) as u32;
                        c.g[V0][0] = sx(dst.wrapping_add(4));
                        c.g[V1][0] = sx(rd.wrapping_add(4));
                        Addr(s + OUT).sw(c, c.g[V0][0]);
                        Addr(s + READ).sw(c, c.g[V1][0]);
                        if c.g[S1][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_cff8);
                        }
                    }
                }
                c.cycles += 8;
                let s = r32(c, S0);
                let k = r32(c, A0);
                let remaining = Addr(s + REMAINING).lw(c) as u32;
                c.g[S3][0] = c.g[A0][0];
                let pending = Addr(s + PENDING).lw(c) as u32;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(k));
                c.g[V1][0] = sx(remaining.wrapping_sub(k));
                c.g[V0][0] = sx(pending.wrapping_sub(k));
                Addr(s + REMAINING).sw(c, c.g[V1][0]);
                Addr(s + PENDING).sw(c, c.g[V0][0]);
            }
            // Whole blocks: s1 = m / 28 * 28, s2 = the remainder.
            c.cycles += 4;
            c.g[V0][0] = BLOCK;
            c.div(0, A1, V0);
            c.cycles += 5;
            c.g[V1][0] = c.lo[0];
            c.mult(0, S1, V1, V0);
            c.g[S2][0] = sx(r32(c, A1).wrapping_sub(r32(c, S1)));
            let s = r32(c, S0);
            Addr(s + COUNT).sw(c, c.g[S1][0]);
            if (c.g[S1][0] as i64) > 0 {
                c.cycles += 6;
                c.g[A0][0] = sx(s + COUNT);
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(r32(c, S1)));
                SND_EAXA_DECODE_BLOCKS.call(c, 0x003c_d078)?;
                let s = r32(c, S0);
                let remaining = Addr(s + REMAINING).lw(c) as u32;
                c.g[V0][0] = sx(remaining.wrapping_sub(r32(c, S1)));
                Addr(s + REMAINING).sw(c, c.g[V0][0]);
            }
            // The remainder: decode one block into the buffer, copy its
            // first s2 samples to the output, leave the rest pending.
            c.cycles += 2;
            let s = r32(c, S0);
            c.g[V0][0] = sx(s + BUF);
            if (c.g[S2][0] as i64) > 0 {
                c.cycles += 20;
                c.g[S4][0] = Addr(s + OUT).lw(c);
                Addr(s + COUNT).sw(c, c.g[S2][0]);
                c.g[A0][0] = sx(s + COUNT);
                Addr(s + OUT).sw(c, c.g[V0][0]);
                c.g[S1][0] = 0;
                SND_EAXA_DECODE_BLOCKS.call(c, 0x003c_d0a4)?;
                let s = r32(c, S0);
                let decoded = Addr(s + COUNT).lw(c) as u32;
                c.g[A0][0] = sx(decoded);
                c.g[A2][0] = sx(r32(c, S3).wrapping_add(r32(c, S2)));
                let out = Addr(s + OUT).lw(c) as u32;
                c.g[A1][0] = sx(out);
                let remaining = Addr(s + REMAINING).lw(c) as u32;
                c.g[V1][0] = sx(out.wrapping_add(decoded << 2));
                c.g[A0][0] = sx(0u32.wrapping_sub(decoded));
                c.g[V0][0] = sx(remaining.wrapping_sub(r32(c, S2)));
                c.g[A1][0] = sx(out.wrapping_sub(0x70));
                Addr(s + REMAINING).sw(c, c.g[V0][0]);
                Addr(s + READ).sw(c, c.g[V1][0]);
                Addr(s + PENDING).sw(c, c.g[A0][0]);
                Addr(s + OUT).sw(c, c.g[A1][0]);
                c.cycles += 1;
                c.g[A1][0] = c.g[S4][0];
                loop {
                    c.cycles += 9;
                    let s = r32(c, S0);
                    let block = Addr(s + OUT).lw(c) as u32;
                    c.g[V1][0] = sx(block);
                    let i = r32(c, S1);
                    c.g[S1][0] = sx(i.wrapping_add(1));
                    c.g[V0][0] = sx((i << 2).wrapping_add(block));
                    let more = (c.g[S1][0] as i64) < (c.g[S2][0] as i64);
                    c.g[A0][0] = more as u64;
                    c.lwc1(0, r32(c, V0));
                    c.swc1(0, r32(c, A1));
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_d0e0);
                    }
                }
                c.cycles += 1;
                c.g[S3][0] = c.g[A2][0];
            }
            // Stream finished: nothing stays pending.
            c.cycles += 3;
            let s = r32(c, S0);
            c.g[V0][0] = Addr(s + REMAINING).lw(c);
            if (c.g[V0][0] as i64) <= 0 {
                Addr(s + PENDING).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
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

pub(crate) const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

pub(crate) const SNDSTRM_FIND_REQUEST: Callee = Callee {
    addr: 0x003b_7a70,
    f: ee::s13::SNDSTRM_FindRequest,
};

pub(crate) const SND_UMUL32X32: Callee = Callee {
    addr: 0x003c_6380,
    f: ee::s13::SND_UMul32x32,
};

pub(crate) const SND_UDIV64BY32: Callee = Callee {
    addr: 0x003c_64a8,
    f: ee::s13::SND_UDiv64by32,
};

/// `int SNDSTRM_GetRequestStatus(int handle, SndStrmStatus* st)`
/// (0x3b7c40): fills `st` (four words, cleared first) for a stream request.
///
/// A negative handle, or one whose stream (low byte, `SNDSTRM_GetStream`)
/// does not exist, returns -8. Otherwise returns 0 with `st[0]` the
/// state: 3 when the request is unknown (`SNDSTRM_FindRequest`), 0 when it
/// is not active (`req+0x24`), 2 when it is the stream's current request
/// (`stream+0x11c`) and 1 when it is queued. For active requests `st[1]`
/// and `st[2]` are the start position (`req+0x14`) and the length
/// (`req+0x18 - req+0x14`) in milliseconds at the stream's rate (current:
/// `stream+0x14`, queued: `stream+0x18`; 64-bit `x * 1000 / rate`), and
/// `st[3]` is `req+0x1c * 1000 / rate`.
///
/// Registers as the original: the handle and then the request in `s0`,
/// the stream and then the rate in `s1`, `st` in `s2`, the 64-bit products
/// passed through the stack, LO/HI from the final `mult`/`divu`.
pub fn sndstrm_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let st = r32(c, S2);
        for i in 0..4 {
            Addr(st + 4 * i).sw(c, 0);
        }
        'done: {
            'bad: {
                if (c.g[S0][0] as i64) < 0 {
                    break 'bad;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0] & 0xff;
                SNDSTRM_GET_STREAM.call(c, 0x003b_7c78)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0 {
                    break 'bad;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                SNDSTRM_FIND_REQUEST.call(c, 0x003b_7c94)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 3;
                let st = r32(c, S2);
                'state: {
                    if c.g[S0][0] == 0 {
                        // Unknown request: state 3.
                        c.cycles += 1;
                        Addr(st).sw(c, c.g[V0][0]);
                        break 'state;
                    }
                    c.cycles += 3;
                    let req = r32(c, S0);
                    c.g[V0][0] = Addr(req + 0x24).lbu(c);
                    if c.g[V0][0] == 0 {
                        // Not active: state 0.
                        c.cycles += 2;
                        Addr(st).sw(c, 0);
                        break 'state;
                    }
                    let stream = r32(c, S1);
                    c.g[V0][0] = Addr(stream + 0x11c).lw(c);
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[S0][0] {
                        // The current request: state 2, the stream's rate.
                        c.cycles += 4;
                        c.g[V0][0] = 2;
                        Addr(st).sw(c, 2);
                        c.g[S1][0] = Addr(stream + 0x14).lhu(c);
                    } else {
                        // Queued: state 1, the next rate.
                        c.g[V0][0] = 1;
                        c.cycles += 2;
                        Addr(st).sw(c, 1);
                        c.g[S1][0] = Addr(stream + 0x18).lhu(c);
                    }
                    // Start and length in milliseconds.
                    c.cycles += 29;
                    c.g[A0][0] = Addr(req + 0x14).lw(c);
                    c.g[A1][0] = 1000;
                    SND_UMUL32X32.call(c, 0x003b_7ce0)?;
                    ms_from_product(c);
                    SND_UDIV64BY32.call(c, 0x003b_7d00)?;
                    let (st, req) = (r32(c, S2), r32(c, S0));
                    Addr(st + 4).sw(c, c.g[V0][0]);
                    c.g[A1][0] = 1000;
                    let end = Addr(req + 0x18).lw(c) as u32;
                    c.g[V0][0] = sx(end);
                    let start = Addr(req + 0x14).lw(c) as u32;
                    c.g[A0][0] = sx(end.wrapping_sub(start));
                    SND_UMUL32X32.call(c, 0x003b_7d18)?;
                    ms_from_product(c);
                    SND_UDIV64BY32.call(c, 0x003b_7d38)?;
                    let (st, req) = (r32(c, S2), r32(c, S0));
                    Addr(st + 8).sw(c, c.g[V0][0]);
                    c.g[V1][0] = 1000;
                    // st[3] = req+0x1c * 1000 / rate (32-bit).
                    c.cycles += 6;
                    c.g[V0][0] = Addr(req + 0x1c).lw(c);
                    c.mult(0, V0, V0, V1);
                    c.divu(0, V0, S1);
                    c.g[V0][0] = c.lo[0];
                    Addr(st + 0xc).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = (-8i64) as u64;
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

/// The 64-bit product in v0 split into words on the stack (`dsll32`/
/// `dsrl32`, two `sw`), reloaded as one doubleword into a0 with the rate
/// (s1) in a1: `SND_UDiv64by32`'s arguments.
pub(crate) fn ms_from_product(c: &mut Cpu) {
    let product = c.g[V0][0];
    c.g[V1][0] = product & 0xffff_ffff;
    c.g[V0][0] = product >> 32;
    let at = sp(c);
    Addr(at).sw(c, c.g[V1][0]);
    Addr(at + 4).sw(c, c.g[V0][0]);
    c.g[A0][0] = Addr(at).ld(c);
    c.g[A1][0] = c.g[S1][0];
}

// ---------------------------------------------------------------------------
// SND_EeMixBlock

/// `SND_MixDeclick16(float* gain, short* buf)` (0x3c7600).
pub(crate) const SND_MIX_DECLICK16: Callee = Callee {
    addr: 0x003c_7600,
    f: ee::s14::SND_MixDeclick16,
};

/// `int SND_EeVoiceMixRamp(Voice*)` (0x3c7ab0): mixes the 16-sample ramp of
/// a voice whose gains changed; returns the samples done.
pub(crate) const SND_EE_VOICE_MIX_RAMP: Callee = Callee {
    addr: 0x003c_7ab0,
    f: ee::s14::SND_EeVoiceMixRamp,
};

/// `SND_EeVoiceFinish(int voice)` (0x3c8430).
pub(crate) const SND_EE_VOICE_FINISH: Callee = Callee {
    addr: 0x003c_8430,
    f: ee::s14::SND_EeVoiceFinish,
};

/// One playing voice (s1): finish its gain ramp, render the rest of the
/// block from its source and mix it into every bus it sends to (and the
/// declick bus).
pub(crate) fn mix_voice(c: &mut Cpu) -> Flow {
    use mixer::*;
    let voice = r32(c, S1);
    let s = sp(c);
    c.cycles += 3;
    c.g[V0][0] = c.lb(voice.wrapping_add(1));
    c.g[S6][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[A1][0] = c.g[FP][0];
    } else {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        SND_EE_VOICE_MIX_RAMP.call(c, 0x003c_8728)?;
        c.g[S6][0] = c.g[V0][0];
        c.sb(voice.wrapping_add(1), 0);
        c.g[A1][0] = sx(r32(c, FP).wrapping_sub(r32(c, S6)));
    }
    c.cycles += 3;
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
    c.sw(s.wrapping_add(0x4), c.g[V0][0]);
    if c.g[A1][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    // Render: n = source->render(source, count, scratch, samples, 0).
    c.cycles += 11;
    c.g[A0][0] = c.lw(voice.wrapping_add(0x40));
    // (v1 keeps only the `lui` half of the mixer address here.)
    c.g[V1][0] = 0x51_0000;
    c.g[S3][0] = sx(BASE);
    c.g[T0][0] = 0;
    c.g[A2][0] = c.lw(BASE + SCRATCH);
    c.g[V0][0] = c.lw(r32(c, A0));
    let f = r32(c, V0);
    c.g[A3][0] = c.lw(BASE + SAMPLES);
    call_ptr(c, f, 0x003c_8768)?;
    c.g[S5][0] = c.g[V0][0];
    if (c.g[S5][0] as i64) < 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        SND_EE_VOICE_FINISH.call(c, 0x003c_877c)?;
        return Ok(());
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, S5) << 2);
    if c.g[S5][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    // Remember the last sample; mix into the buses with non-zero sends.
    c.cycles += 7;
    c.g[V1][0] = c.lw(BASE + SAMPLES);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.lwc1(0, r32(c, V0).wrapping_sub(4));
    c.swc1(0, voice.wrapping_add(0x3c));
    c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
    c.g[S2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.mtc1(0, 20);
        c.g[S7][0] = sx(r32(c, S6) << 2);
        c.g[S4][0] = sx(BASE + BUS_BUF);
        c.g[S0][0] = sx(voice.wrapping_add(0x1c));
        loop {
            c.cycles += 5;
            c.lwc1(12, r32(c, S0));
            c.c_eq_s(12, 20);
            if !c.fcc() {
                c.cycles += 7;
                c.g[A2][0] = c.lw(r32(c, S4));
                c.g[A0][0] = c.g[S5][0];
                c.g[V0][0] = c.lw(BASE + MIX_FN);
                c.g[A1][0] = c.lw(BASE + SAMPLES);
                let f = r32(c, V0);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, S7)));
                call_ptr(c, f, 0x003c_87e4)?;
            }
            c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
            c.cycles += 5;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(4));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_87b8);
            }
        }
    }
    c.cycles += 9;
    c.mtc1(0, 20);
    c.g[S3][0] = sx(BASE);
    c.g[S6][0] = sx(r32(c, S6) << 2);
    c.g[V0][0] = 0x51_0000;
    c.g[S7][0] = sx(BASE + DECLICK_ON);
    c.g[S4][0] = sx(DECLICK_BUF);
    c.g[S0][0] = sx(voice.wrapping_add(0x38));
    c.g[S2][0] = 0;
    loop {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S7)));
        c.g[V1][0] = c.lb(r32(c, V0));
        if c.g[V1][0] != 0 {
            c.cycles += 5;
            c.lwc1(12, r32(c, S0));
            c.c_eq_s(12, 20);
            if !c.fcc() {
                c.cycles += 7;
                c.g[V0][0] = c.lw(BASE + MIX_FN);
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.lw(BASE + SAMPLES);
                let f = r32(c, V0);
                c.g[A2][0] = sx(r32(c, S6).wrapping_add(DECLICK_BUF));
                call_ptr(c, f, 0x003c_8858)?;
                c.sw(DECLICK_BUF + 0x1800, 0);
            }
        }
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
        c.cycles += 2;
        let again = (c.g[S2][0] as i64) <= 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_8820);
        }
    }
    c.cycles += 1;
    Ok(())
}

/// `void SND_EeMixBlock(short** outs, int count)` (0x3c85d0): one block of
/// the EE software mixer: fade the declick bus, clear every bus buffer
/// (declicking it), mix each playing voice into its buses, run the post-mix
/// hook and send every bus through its effect (if any) to its output.
pub fn snd_ee_mix_block(c: &mut Cpu) {
    use mixer::*;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xc0);
        c.g[V1][0] = 0x51_0000;
        frame.save_fpr(c, 20, 0xb0);
        c.g[V0][0] = sx(BASE);
        for (reg, off) in [
            (S0, 0xa0),
            (S1, 0x90),
            (S2, 0x80),
            (S3, 0x70),
            (FP, 0x20),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x10);
        c.g[S0][0] = sx(BASE + DECLICK_GAIN);
        c.g[S1][0] = 0x51_0000;
        c.g[S2][0] = 0;
        c.g[S3][0] = sx(BASE + DECLICK_ON);
        c.g[FP][0] = c.g[A1][0];
        c.mtc1(0, 20);
        let s = sp(c);
        c.sw(s, c.g[A0][0]);

        // The declick bus fades (one bus).
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S3)));
            c.g[V1][0] = c.lb(r32(c, V0));
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                c.lwc1(0, r32(c, S0));
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(DECLICK_BUF);
                    SND_MIX_DECLICK16.call(c, 0x003c_8658)?;
                }
            }
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.cycles += 2;
            let again = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_8628);
            }
        }

        // Clear every bus buffer (count samples) and declick it.
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = sx(BASE);
        c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
        c.g[S2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[S1][0] = c.g[V1][0];
            c.mtc1(0, 20);
            c.g[S0][0] = sx(BASE + BUS_DECLICK);
            loop {
                c.cycles += 9;
                let gain = r32(c, S0);
                c.g[A0][0] = c.lw(gain.wrapping_sub(0x18));
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(r32(c, FP) << 2);
                call_ptr(c, MEMSET, 0x003c_8698)?;
                c.lwc1(0, gain);
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw(gain.wrapping_sub(0x18));
                    c.g[A0][0] = c.g[S0][0];
                    SND_MIX_DECLICK16.call(c, 0x003c_86b8)?;
                }
                c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
                c.cycles += 4;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(gain.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8688);
                }
            }
        }

        // The playing voices.
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = sx(BASE);
        c.g[V0][0] = c.lbu(BASE + VOICE_COUNT);
        c.g[S0][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = VOICE_SIZE;
            loop {
                c.cycles += 11;
                c.g[V0][0] = 0x51_0000;
                c.mult(0, V1, S0, V1);
                c.g[V0][0] = sx(BASE);
                c.g[A0][0] = c.lw(BASE + VOICES);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                c.sw(s.wrapping_add(0x4), c.g[V0][0]);
                c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = c.lb(r32(c, S1));
                let playing = c.g[V0][0] == 2;
                c.g[V1][0] = 0x51_0000;
                if playing {
                    mix_voice(c)?;
                    c.g[V1][0] = 0x51_0000;
                }
                c.cycles += 6;
                c.g[S0][0] = c.lw(s.wrapping_add(0x4));
                c.g[V1][0] = sx(BASE);
                c.g[V0][0] = c.lbu(BASE + VOICE_COUNT);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = VOICE_SIZE;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_86e8);
                }
            }
        }

        // The post-mix hook.
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[S0][0] = sx(BASE);
        c.g[V0][0] = c.lw(BASE + POST_MIX);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            let f = r32(c, V0);
            c.g[A0][0] = c.g[FP][0];
            call_ptr(c, f, 0x003c_88a0)?;
        }
        c.g[V0][0] = c.lbu(BASE + BUS_COUNT);

        // Each bus through its effect (if any) to its output.
        c.cycles += 2;
        c.g[S2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S3][0] = c.g[S0][0];
            c.g[S0][0] = c.lw(s);
            c.g[S1][0] = sx(BASE + BUS_BUF);
            loop {
                let bus = r32(c, S1);
                c.cycles += 3;
                c.g[V0][0] = c.lw(bus.wrapping_add(BUS_FX));
                c.g[A0][0] = c.g[V0][0];
                let ret = if c.g[V0][0] != 0 {
                    // fx->process(fx, count, bus, scratch, 0), then the output
                    // from the scratch buffer.
                    c.cycles += 15;
                    c.g[A2][0] = c.lw(bus);
                    c.g[A3][0] = c.lw(BASE + SCRATCH);
                    c.g[A1][0] = c.g[FP][0];
                    c.g[V0][0] = c.lw(r32(c, V0));
                    let f = r32(c, V0);
                    c.g[T0][0] = 0;
                    call_ptr(c, f, 0x003c_88dc)?;
                    c.g[A0][0] = c.lw(bus.wrapping_add(BUS_OUT));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A2][0] = c.lw(BASE + SCRATCH);
                    0x003c_88f8
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(bus.wrapping_add(BUS_OUT));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A2][0] = c.lw(bus);
                    0x003c_891c
                };
                // out->process(out, count, src, outs[i], 0).
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(r32(c, A0));
                let f = r32(c, V0);
                c.g[A3][0] = c.lw(r32(c, S0));
                call_ptr(c, f, ret)?;
                c.g[V0][0] = c.lbu(BASE + BUS_COUNT);
                c.cycles += 5;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(bus.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_88b8);
                }
            }
        }

        c.cycles += 13;
        for (reg, off) in [
            (S0, 0xa0),
            (S1, 0x90),
            (S2, 0x80),
            (S3, 0x70),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
            (FP, 0x20),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod pan {

    /// The driver state block (`snd::STATE`).
    pub const STATE: u32 = 0x0050_a8e8;

    /// `u8`: speaker count.
    pub const SPEAKERS: u32 = 0x47;

    /// `u8`: auxiliary sends.
    pub const AUX_SENDS: u32 = 0x48;

    /// `SndChannel*`: the channel array.
    pub const CHANNELS: u32 = 0x1f0;

    /// The second global block (`snd::SYNC`).
    pub const SYNC: u32 = 0x0050_ad00;

    /// `float[voice][8]`: per-voice output gains (`SYNC + 0xe98`).
    pub const GAINS: u32 = 0x0050_bb98;

    /// `{ s16 angle; s16 level; }[layout][6]`: the speaker layouts (0x18
    /// bytes each): per voice slot, a pan angle offset and a level.
    pub const LAYOUTS: u32 = 0x0050_ab20;

    /// The `level` halves of `LAYOUTS`.
    pub const LAYOUT_LEVELS: u32 = 0x0050_ab22;

    /// Bytes per layout.
    pub const LAYOUT_SIZE: u64 = 0x18;

    /// `SndChannel` size.
    pub const CHAN_SIZE: u64 = 0x8c;

    /// `s16[]`: the channel's voice numbers.
    pub const VOICES: u32 = 0x04;

    /// `u16`: the channel's pan angle.
    pub const ANGLE: u32 = 0x1c;

    /// `u8`: number of voices.
    pub const VOICE_COUNT: u32 = 0x23;

    /// `u16`: flags.
    pub const FLAGS: u32 = 0x24;

    /// `u16[]`: per-voice pan angles.
    pub const VOICE_ANGLES: u32 = 0x50;

    /// Flag: SPU voices (a centre-rear split in 5-speaker mode).
    pub const F_SPU: u64 = 0x8;

    /// Flag: pan by the smooth lookup table (instead of the gain tables).
    pub const F_SMOOTH: u64 = 0x40;

    /// `1/sqrt(2)`: the mono gain of both sides.
    pub const HALF_POWER: u32 = 0x3f35_04f4;

    /// `32767.0`.
    pub const FULL_SCALE: u32 = 0x46ff_fe00;

    /// `1.0`.
    pub const ONE: u32 = 0x3f80_0000;
}

pub(crate) mod snd_tick {

    /// The driver state (`0x510000 - 0x5718`).
    pub const STATE: u32 = 0x0050_a8e8;

    /// `s8`: number of tick callbacks.
    pub const TICK_CALLBACKS: u32 = 0x177;

    /// `s16`: number of voice channels.
    pub const CHANNEL_COUNT: u32 = 0x17e;

    /// `u32`: driver ticks run so far.
    pub const TICKS: u32 = 0x180;

    /// `void (*)()[]`: the tick callbacks.
    pub const CALLBACKS: u32 = 0x184;

    /// `SndChannel*`: the channel array (0x8c bytes each).
    pub const CHANNELS: u32 = 0x1f0;

    pub const CHANNEL_SIZE: u64 = 0x8c;

    /// Channel: volume sweep step per tick (0: no sweep).
    pub const SWEEP_STEP: u32 = 0x30;

    /// Channel: sweep target.
    pub const SWEEP_TARGET: u32 = 0x34;

    /// Channel: current sweep value.
    pub const SWEEP: u32 = 0x38;

    /// Channel: envelope level step per tick.
    pub const ENV_STEP: u32 = 0x3c;

    /// Channel: envelope level (16.16).
    pub const ENV_LEVEL: u32 = 0x40;

    /// Channel: ticks left in the envelope segment.
    pub const ENV_TICKS: u32 = 0x44;

    /// Channel: number of envelope segments (`s8`).
    pub const ENV_SEGMENTS: u32 = 0x5c;

    /// Channel: current envelope segment (`u8`).
    pub const ENV_SEGMENT: u32 = 0x5d;

    /// Channel: volume LFO length / position (`u8`).
    pub const VOL_LFO_LEN: u32 = 0x65;

    /// Channel: pitch LFO length (`u8`).
    pub const PITCH_LFO_LEN: u32 = 0x66;

    pub const VOL_LFO_POS: u32 = 0x67;

    pub const PITCH_LFO_POS: u32 = 0x68;

    /// Channel: allocated flag (`s8`, 1 when allocated).
    pub const ALLOCATED: u32 = 0x69;

    /// Channel: envelope segment table (`{ticks, level}` pairs).
    pub const ENV_TABLE: u32 = 0x6c;

    /// Channel: volume LFO table, or null.
    pub const VOL_LFO: u32 = 0x78;

    /// Channel: pitch LFO (modulation) table, or null.
    pub const PITCH_LFO: u32 = 0x7c;

    /// Channel: cached pitch (`u16`), cleared to force a recompute.
    pub const PITCH_CACHE: u32 = 0x86;
}

pub(crate) mod snd_tick_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const TICK_STUB: Callee = Callee {
        addr: 0x003c_2438,
        f: ee::s13::SND_TickStub,
    };

    pub const CALC_PITCH: Callee = Callee {
        addr: 0x003b_a938,
        f: ee::s13::SND_ChannelCalcPitch,
    };

    pub const APPLY_PITCH: Callee = Callee {
        addr: 0x003c_25e0,
        f: ee::s13::SND_ChannelApplyPitch,
    };

    pub const STOP: Callee = Callee {
        addr: 0x003b_89e0,
        f: ee::s13::SND_Stop,
    };

    pub const CALC_VOLUME: Callee = Callee {
        addr: 0x003b_a7b0,
        f: ee::s13::SND_ChannelCalcVolume,
    };

    pub const APPLY_VOLUME: Callee = Callee {
        addr: 0x003c_2a50,
        f: ee::s13::SND_ChannelApplyVolume,
    };
}

pub(crate) mod sndstrm {

    /// Stream: its stream file (`StreamFile_GetChannelBytes`).
    pub const FILE: u32 = 0x00;

    /// Stream: buffer slot index (`SNDSTRM_GetBufferedBytes`).
    pub const BUFFER: u32 = 0x08;

    /// Stream: playback rate in bytes per second (u16).
    pub const RATE: u32 = 0x14;

    /// Stream: the current request (list head).
    pub const REQUEST: u32 = 0x11c;

    /// Stream: state (0: idle).
    pub const STATE: u32 = 0x124;

    /// Request: its id, and its total byte count.
    pub const REQ_ID: u32 = 0x0c;

    pub const REQ_BYTES: u32 = 0x10;

    /// Status out: state, request id, buffered milliseconds.
    pub const OUT_STATE: u32 = 0x0;

    pub const OUT_REQUEST: u32 = 0x4;

    pub const OUT_MS: u32 = 0x8;
}

pub(crate) mod release {

    /// `lui 0x51`: the high half both blocks are addressed from.
    pub const HI: u64 = 0x0051_0000;

    /// The driver state (`HI - 0x5718`).
    pub const STATE: u32 = 0x0050_a8e8;

    /// The voice status block (`HI - 0x5300`).
    pub const VOICES: u32 = 0x0050_ad00;

    // State fields.
    /// Channel table pointer.
    pub const CHANNELS: u32 = 0x1f0;

    /// Number of hardware-voice channels (u8): the first group.
    pub const COUNT_A: u32 = 0x30;

    /// Number of channels of the second group (u8), after the first.
    pub const COUNT_B: u32 = 0x2f;

    /// Output mode (u8); 2 releases a voice's mirror (`+0x18`) as well.
    pub const MODE: u32 = 0x49;

    // Channel fields (0x8c bytes each).
    pub const CHANNEL_SIZE: u32 = 0x8c;

    /// Voice indices (s16 each).
    pub const CH_VOICES: u32 = 0x04;

    /// Number of voices (u8).
    pub const CH_VOICE_COUNT: u32 = 0x23;

    /// Timed-stop state (s16); -1 = no timed stop pending.
    pub const CH_STOP: u32 = 0x28;

    // Voice status block fields.
    /// Pointer to the status table the IOP fills in.
    pub const STATUS: u32 = 0xcb0;

    /// Last key seen per first-group voice (8-byte slots: key, remaining).
    pub const SEEN_A: u32 = 0x83c;

    /// Last key seen per second-group voice (8-byte slots: key, remaining).
    pub const SEEN_B: u32 = 0x9bc;

    // Status table entries.
    /// First group: key word, 8-byte stride.
    pub const KEY_A: u32 = 0x20;

    /// Second group: key word, 16-byte stride.
    pub const KEY_B: u32 = 0x1a0;

    /// Mode 2: the mirror voice of voice `h` is `h + 0x18`.
    pub const MIRROR: u32 = 0x18;

    /// Return addresses of the two `SND_ChannelRelease` calls.
    pub const RET_A: u32 = 0x003c_13b0;

    pub const RET_B: u32 = 0x003c_1504;

    /// Loop heads (event checks).
    pub const HEAD_A: u32 = 0x003c_12e8;

    pub const HEAD_A_VOICES: u32 = 0x003c_1388;

    pub const HEAD_B: u32 = 0x003c_1428;

    pub const HEAD_B_VOICES: u32 = 0x003c_14c8;

    pub const SND_CHANNEL_RELEASE: super::Callee = super::Callee {
        addr: 0x003b_a550,
        f: super::ee::s13::SND_ChannelRelease,
    };
}

pub(crate) const SND_CHANNEL_APPLY_VOLUME: Callee = Callee {
    addr: 0x003c_2a50,
    f: ee::s13::SND_ChannelApplyVolume,
};

pub(crate) const SND_PAN_LOOKUP: Callee = Callee {
    addr: 0x003c_3728,
    f: ee::s13::SND_PanLookup,
};

pub(crate) const SND_PAN_FIX_QUADRANT: Callee = Callee {
    addr: 0x003c_3780,
    f: ee::s13::SND_PanFixQuadrant,
};

pub(crate) const SND_GET_PAN_GAINS: Callee = Callee {
    addr: 0x003b_c4c0,
    f: ee::s13::SND_GetPanGains,
};

/// `addu`-style 32-bit add of a register and a value, sign-extended.
#[inline(always)]
pub(crate) fn add(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `sll r, 5` / `sll r, 2` / ...: 32-bit shift, sign-extended.
#[inline(always)]
pub(crate) fn shl(a: u64, n: u32) -> u64 {
    sx((a as u32) << n)
}

/// `void SND_ChannelApplyPan(int channel)` (0x3c38e0): spread a channel's
/// voices over the speakers. One speaker: every voice gets `1/sqrt 2` on
/// both sides. One voice: the channel's pan angle through the smooth lookup
/// (flag 0x40; result also kept as 16-bit levels) or the gain tables, with a
/// centre-rear split on 5 speakers (SPU flag). Several voices: pick a
/// speaker layout from the speaker/voice counts, pan each voice by its own
/// angle plus the layout's offset, scale by the layout's level, flip
/// out-of-range smooth pans, and route a 6th voice (LFE) to the aux send.
/// Then `SND_ChannelApplyVolume(channel)`.
pub fn snd_channel_apply_pan(c: &mut Cpu) {
    use pan::*;
    guest(c, |c| {
        // Prologue: the channel, its gain block and the speaker count.
        c.cycles += 28;
        let frame = Frame::open(c, 0xd0);
        let index_at = frame.at(c, 0x20);
        c.sw(index_at, c.g[A0][0]);
        c.g[A0][0] = CHAN_SIZE;
        frame.save(c, S0, 0xc0);
        c.g[V0][0] = c.lw(index_at);
        frame.save(c, S2, 0xa0);
        c.mult(0, A0, V0, A0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        c.g[A3][0] = sx(STATE);
        c.g[A2][0] = sx(GAINS);
        c.g[V0][0] = c.lw(index_at);
        c.g[A1][0] = c.lw(STATE + CHANNELS);
        c.g[V1][0] = shl(c.g[V0][0], 5);
        c.g[T0][0] = c.lbu(STATE + SPEAKERS);
        let ch = (c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32);
        c.g[S2][0] = sx(ch);
        c.g[S0][0] = add(c.g[V1][0], GAINS as u64);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0;
        let speakers = c.g[T0][0];

        if speakers == 1 {
            // Mono: both sides of every voice at half power.
            c.cycles += 3;
            c.g[V0][0] = c.lbu(ch + VOICE_COUNT);
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[A1][0] = sx(GAINS);
                c.g[AT][0] = HALF_POWER as u64;
                c.f[0] = HALF_POWER;
                c.g[A2][0] = sx(GAINS + 4);
                c.g[A0][0] = sx(ch + VOICES);
                loop {
                    c.cycles += 11;
                    let voice = c.lh(r32(c, A0));
                    c.g[S0][0] = add(c.g[S0][0], 1);
                    c.g[V0][0] = shl(voice, 5);
                    c.g[V1][0] = add(c.g[V0][0], (GAINS + 4) as u64);
                    c.swc1(0, r32(c, V1));
                    c.g[V0][0] = add(c.g[V0][0], GAINS as u64);
                    c.swc1(0, r32(c, V0));
                    let count = c.lbu(ch + VOICE_COUNT);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                    c.g[A0][0] = add(c.g[A0][0], 2);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3978);
                    }
                }
            }
            // A sixth voice (LFE) gets nothing on the front pair.
            c.cycles += 4;
            c.g[V1][0] = c.lbu(ch + VOICE_COUNT);
            c.g[V0][0] = 6;
            c.g[A0][0] = 0x51_0000;
            if c.g[V1][0] == 6 {
                c.cycles += 9;
                let lfe = shl(c.lh(ch + VOICES + 10), 5);
                c.g[A0][0] = sx(SYNC);
                c.g[V0][0] = add(lfe, SYNC as u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xe98), 0);
                c.g[V1][0] = shl(c.lh(ch + VOICES + 10), 5);
                c.g[A0][0] = add(c.g[A0][0], c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xe9c), 0);
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(index_at_now(c, frame));
            SND_CHANNEL_APPLY_VOLUME.call(c, 0x003c_39e0)?;
            frame.restore(c, S0, 0xc0);
            pan_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 4;
        let count = c.lbu(ch + VOICE_COUNT);
        c.g[V0][0] = count;
        c.g[V1][0] = count & 0xff;
        c.g[A2][0] = count;
        if count == 1 {
            pan_one_voice(c, frame, ch)?;
        } else {
            pan_voices(c, frame, ch, speakers)?;
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(index_at_now(c, frame));
        SND_CHANNEL_APPLY_VOLUME.call(c, 0x003c_3e74)?;
        frame.restore(c, S0, 0xc0);
        pan_epilogue(c, frame);
        Ok(())
    })
}

/// `sp + 0x20`, where the channel index was saved.
#[inline(always)]
pub(crate) fn index_at_now(c: &Cpu, frame: Frame) -> u32 {
    frame.at(c, 0x20)
}

/// Restore s1-s7, fp and ra and return.
pub(crate) fn pan_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore(c, S5, 0x70);
    frame.restore(c, S6, 0x60);
    frame.restore(c, S7, 0x50);
    frame.restore(c, FP, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

/// One voice: the channel's angle into its gain block (s0).
pub(crate) fn pan_one_voice(c: &mut Cpu, frame: Frame, ch: u32) -> crate::call::Flow {
    use pan::*;
    let gains = c.g[S0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
    if c.g[V0][0] != 0 {
        // Nothing on the first aux send.
        c.cycles += 5;
        c.g[V0][0] = shl(c.lbu(STATE + SPEAKERS), 2);
        c.g[V0][0] = add(c.g[S0][0], c.g[V0][0]);
        c.sw(r32(c, V0), 0);
    }
    c.g[V0][0] = c.lhu(ch + FLAGS);
    c.cycles += 3;
    c.g[V0][0] &= F_SMOOTH;
    c.g[A0][0] = c.lhu(ch + ANGLE);
    if c.g[V0][0] != 0 {
        c.cycles += 22;
        c.g[A1][0] = sx(gains);
        SND_PAN_LOOKUP.call(c, 0x003c_3a2c)?;
        c.g[A0][0] = c.lhu(ch + ANGLE);
        c.g[A1][0] = sx(gains + 4);
        c.g[A3][0] = c.lh(gains + 0x1c);
        c.g[A2][0] = sx(gains);
        c.g[T0][0] = c.lh(gains + 0x1e);
        SND_PAN_FIX_QUADRANT.call(c, 0x003c_3a44)?;
        // Keep the pair as 16-bit levels too.
        c.g[AT][0] = FULL_SCALE as u64;
        let scale = FpuF32(FULL_SCALE);
        let right = Addr(gains + 4).read_fpu(c) * scale;
        let left = Addr(gains).read_fpu(c) * scale;
        set_fpr(c, 1, left);
        let right_w = right.to_i32() as u32;
        let left_w = left.to_i32() as u32;
        c.f[2] = right_w;
        c.f[0] = left_w;
        c.g[V0][0] = sx(right_w);
        c.g[V1][0] = sx(left_w);
        c.sh(gains + 0x1c, c.g[V0][0]);
        c.sh(gains + 0x1e, c.g[V1][0]);
    } else {
        c.cycles += 2;
        c.g[A1][0] = sx(gains);
        SND_GET_PAN_GAINS.call(c, 0x003c_3a84)?;
    }
    c.cycles += 6;
    c.g[A0][0] = 5;
    c.g[V0][0] = sx(STATE);
    c.g[V1][0] = c.lbu(STATE + SPEAKERS);
    if c.g[V1][0] == 5 {
        c.cycles += 4;
        c.g[V0][0] = c.lhu(ch + FLAGS) & F_SPU;
        c.g[A1][0] = c.g[SP][0];
        if c.g[V0][0] != 0 {
            // SPU voices on 5 speakers: the opposite angle's front/rear
            // gains go to the centre and rear slots.
            c.cycles += 9;
            let angle = sx((c.lhu(ch + ANGLE) as u32).wrapping_sub(0x8000));
            c.g[A0][0] = angle & 0xffff;
            SND_GET_PAN_GAINS.call(c, 0x003c_3abc)?;
            let tmp = frame.at(c, 0);
            c.lwc1(0, tmp + 8);
            c.lwc1(1, tmp + 0xc);
            c.swc1(0, gains + 0x10);
            c.swc1(1, gains + 4);
        }
    }
    Ok(())
}

/// Where a layout decision continues (the original's join points).
pub(crate) enum LayoutJoin {
    /// `a1 = 0` if v0 is non-zero, then reload the voice count.
    ZeroIfV0,
    /// Reload the voice count.
    Reload,
    /// v0 already holds the voice count.
    Loaded,
}

/// Several voices: choose a speaker layout (a1), then pan every voice.
pub(crate) fn pan_voices(c: &mut Cpu, frame: Frame, ch: u32, speakers: u64) -> crate::call::Flow {
    use pan::*;
    // Layout by speaker/voice counts (registers: a0 = 1, a1 = 0,
    // v1 = a2 = voice count).
    let count = c.g[A2][0];
    c.cycles += 3;
    c.g[V0][0] = 5;
    let mut join = None;
    if speakers == 5 {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
        if c.g[V0][0] == 0 {
            // 5 speakers without aux sends: layout 1 for 6 voices.
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] ^ 6;
            c.g[A1][0] = c.g[A0][0];
            join = Some(LayoutJoin::ZeroIfV0);
        }
    }
    let join = match join {
        Some(j) => j,
        None => {
            c.g[A0][0] = 0x51_0000;
            c.cycles += 5;
            c.g[V1][0] = 4;
            c.g[V0][0] = sx(STATE);
            c.g[A0][0] = c.lbu(STATE + SPEAKERS);
            c.g[V0][0] = 2;
            if c.g[A0][0] == 4 {
                // Quad: layout 2 for 5-6 voices.
                c.cycles += 7;
                c.g[V0][0] = sx((count as u32).wrapping_sub(5)) & 0xff;
                c.g[V1][0] = (1 < c.g[V0][0]) as u64;
                c.g[A1][0] = if c.g[V1][0] != 0 { 0 } else { 2 };
                LayoutJoin::Reload
            } else {
                c.cycles += 2;
                if c.g[A0][0] != 2 {
                    c.g[V0][0] = c.lbu(ch + VOICE_COUNT);
                    LayoutJoin::Loaded
                } else {
                    // Stereo.
                    c.cycles += 4;
                    let smooth = c.lhu(ch + FLAGS) & F_SMOOTH != 0;
                    c.g[V0][0] = count & 0xff;
                    if smooth {
                        // Layout 1 for 6 voices.
                        c.cycles += 3;
                        c.g[V0][0] ^= 6;
                        c.g[A1][0] = (c.g[V0][0] < 1) as u64;
                        LayoutJoin::Reload
                    } else {
                        // Layout 3 for 5-6 voices, 5 for 4.
                        c.cycles += 5;
                        c.g[V0][0] = sx((count as u32).wrapping_sub(5)) & 0xff;
                        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                        c.g[A1][0] = 3;
                        if c.g[V0][0] != 0 {
                            LayoutJoin::Reload
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (count & 0xff) ^ 4;
                            c.g[A1][0] = 5;
                            LayoutJoin::ZeroIfV0
                        }
                    }
                }
            }
        }
    };
    if let LayoutJoin::ZeroIfV0 = join {
        c.cycles += 1;
        if c.g[V0][0] != 0 {
            c.g[A1][0] = 0;
        }
    }
    if !matches!(join, LayoutJoin::Loaded) {
        c.cycles += 1;
        c.g[V0][0] = c.lbu(ch + VOICE_COUNT);
    }
    c.cycles += 2;
    c.g[S0][0] = 0;
    if c.g[V0][0] == 0 {
        return Ok(());
    }

    // Per voice: s0 index, s7 the layout's byte offset, fp the voice angles.
    c.cycles += 8;
    c.g[V0][0] = LAYOUT_SIZE;
    c.g[A2][0] = sx(ch + VOICES);
    c.mult(0, S7, A1, V0);
    let voices_at = frame.at(c, 0x24);
    c.sw(voices_at, c.g[A2][0]);
    c.g[FP][0] = sx(ch + VOICE_ANGLES);
    let layout_at = frame.at(c, 0x28);
    c.sw(layout_at, c.g[S7][0]);
    c.g[V0][0] = 5;
    loop {
        pan_voice(c, frame, ch)?;
        // Next voice.
        c.cycles += 5;
        let count = c.lbu(ch + VOICE_COUNT);
        c.g[S0][0] = c.g[S1][0];
        let more = (c.g[S0][0] as i64) < (count as i64);
        c.g[V0][0] = 5;
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_3b98);
        }
    }
}

/// Where a voice's panning continues before the level scaling.
pub(crate) enum VoiceJoin {
    /// `a2 = 0x510000`, then as `State`.
    Hi,
    /// `a2 = STATE`.
    State,
    /// a2 already holds `STATE`.
    Done,
}

/// Pan voice s0 of a multi-voice channel.
pub(crate) fn pan_voice(c: &mut Cpu, frame: Frame, ch: u32) -> crate::call::Flow {
    use pan::*;
    let i = c.g[S0][0];
    let join = 'pan: {
        c.cycles += 2;
        if i == 5 {
            // The sixth voice (LFE): all to the first aux send if any.
            c.cycles += 5;
            c.g[A0][0] = 0x51_0000;
            c.g[V1][0] = sx(STATE);
            c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(STATE + SPEAKERS);
                c.g[S3][0] = shl(i, 1);
                c.g[S5][0] = sx(ch + VOICES);
                c.g[S1][0] = add(i, 1);
                let voice_at = add(c.g[S5][0], c.g[S3][0]) as u32;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A2][0] = sx(voice_at);
                    c.g[V0][0] = 0x51_0000;
                    c.g[T0][0] = sx(GAINS);
                    loop {
                        c.cycles += 9;
                        let voice = c.lh(voice_at);
                        c.g[V1][0] = shl(c.g[A1][0], 2);
                        c.g[A1][0] = add(c.g[A1][0], 1);
                        c.g[V0][0] = shl(voice, 5);
                        c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        c.g[V1][0] = add(add(c.g[V1][0], c.g[V0][0]), GAINS as u64);
                        c.sw(r32(c, V1), 0);
                        if c.g[A0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_3bd8);
                        }
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = sx(voice_at);
                c.g[V1][0] = shl(c.g[A1][0], 2);
                let voice = c.lh(voice_at);
                c.g[AT][0] = ONE as u64;
                c.f[0] = ONE;
                c.g[V0][0] = shl(voice, 5);
                c.g[A0][0] = sx(GAINS);
                c.g[V1][0] = add(add(c.g[V1][0], c.g[V0][0]), GAINS as u64);
                c.swc1(0, r32(c, V1));
                break 'pan VoiceJoin::Hi;
            }
            c.cycles += 1;
        }
        let flags = c.lhu(ch + FLAGS);
        c.cycles += 3;
        c.g[V0][0] = shl(i, 2);
        let voices = c.lw(frame.at(c, 0x24));
        if flags & F_SMOOTH != 0 {
            // Smooth: angle + layout offset through the lookup table.
            c.cycles += 23;
            c.g[A0][0] = voices;
            c.g[V1][0] = shl(i, 1);
            c.g[V0][0] = add(c.g[V0][0], c.g[S7][0]);
            c.g[A1][0] = add(c.g[FP][0], c.g[V1][0]);
            c.g[A3][0] = add(voices, c.g[V1][0]);
            c.g[V0][0] = add(c.g[V0][0], LAYOUTS as u64);
            let angle = c.lhu(r32(c, A1));
            let offset = c.lhu(r32(c, V0));
            c.g[S3][0] = c.g[V1][0];
            let voice = c.lh(r32(c, A3));
            c.g[S5][0] = sx(ch + VOICES);
            c.g[S1][0] = add(i, 1);
            c.g[A2][0] = sx(GAINS);
            c.g[A0][0] = add(angle, offset) & 0xffff;
            c.g[A1][0] = add(shl(voice, 5), GAINS as u64);
            SND_PAN_LOOKUP.call(c, 0x003c_3c90)?;
            c.g[A2][0] = 0x51_0000;
            break 'pan VoiceJoin::State;
        }
        // Gain tables: angle + layout offset.
        c.cycles += 25;
        c.g[A0][0] = voices;
        c.g[A3][0] = shl(i, 1);
        c.g[V1][0] = add(voices, c.g[A3][0]);
        c.g[V0][0] = add(add(c.g[V0][0], c.g[S7][0]), LAYOUTS as u64);
        let voice = c.lh(r32(c, V1));
        c.g[S3][0] = c.g[A3][0];
        c.g[V1][0] = c.lh(r32(c, V0));
        c.g[A2][0] = add(c.g[FP][0], c.g[A3][0]);
        c.g[V0][0] = 0x51_0000;
        let angle = c.lhu(r32(c, A2));
        c.g[S6][0] = sx(GAINS);
        c.g[A0][0] = add(angle, c.g[V1][0]);
        c.g[A1][0] = add(shl(voice, 5), GAINS as u64);
        SND_GET_PAN_GAINS.call(c, 0x003c_3ce0)?;
        c.g[S5][0] = sx(ch + VOICES);
        let count = c.lbu(ch + VOICE_COUNT);
        c.g[V1][0] = 0x51_0000;
        c.g[S4][0] = sx(STATE);
        c.g[V0][0] = (count < 5) as u64;
        c.g[S1][0] = add(i, 1);
        if c.g[V0][0] != 0 {
            break 'pan VoiceJoin::Hi;
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        c.g[A2][0] = 0x51_0000;
        if i != 1 {
            break 'pan VoiceJoin::State;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lbu(STATE + SPEAKERS);
        c.g[A0][0] = 5;
        c.g[A2][0] = sx(STATE);
        if c.g[V0][0] != 5 {
            break 'pan VoiceJoin::Done;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lhu(ch + FLAGS) & F_SPU;
        c.g[A0][0] = 0x8000;
        if c.g[V0][0] == 0 {
            break 'pan VoiceJoin::Hi;
        }
        // SPU voices on 5 speakers: the second voice's centre and rear
        // slots get the opposite angle's gains.
        c.cycles += 12;
        c.g[A1][0] = c.g[SP][0];
        SND_GET_PAN_GAINS.call(c, 0x003c_3d30)?;
        let tmp = frame.at(c, 0);
        let second = c.lh(ch + VOICES + 2);
        c.lwc1(0, tmp + 8);
        c.g[V1][0] = add(c.g[S6][0], shl(second, 5));
        c.lwc1(1, tmp + 0xc);
        c.swc1(0, r32(c, V1).wrapping_add(0x10));
        let second = c.lh(ch + VOICES + 2);
        c.g[V0][0] = add(c.g[S6][0], shl(second, 5));
        c.swc1(1, r32(c, V0).wrapping_add(4));
        VoiceJoin::Hi
    };
    match join {
        VoiceJoin::Hi => {
            c.cycles += 2;
            c.g[A2][0] = sx(STATE);
        }
        VoiceJoin::State => {
            c.cycles += 1;
            c.g[A2][0] = sx(STATE);
        }
        VoiceJoin::Done => {}
    }

    // Scale the voice's outputs by the layout's level / 32767.
    c.cycles += 5;
    let outputs = add(c.lbu(STATE + SPEAKERS), c.lbu(STATE + AUX_SENDS));
    c.g[V1][0] = outputs;
    c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
    c.g[A1][0] = 0;
    if outputs != 0 {
        c.cycles += 21;
        c.g[A0][0] = c.lw(frame.at(c, 0x28));
        let level_at = add(add(shl(i, 2), c.g[A0][0]), LAYOUT_LEVELS as u64);
        let level = c.lh(level_at as u32);
        c.g[V0][0] = level;
        c.g[A3][0] = add(c.g[S5][0], c.g[S3][0]);
        c.g[A0][0] = c.lbu(STATE + SPEAKERS);
        c.g[V1][0] = c.lbu(STATE + AUX_SENDS);
        let level = FpuF32::from_i32(level as i32);
        set_fpr(c, 1, level);
        c.g[AT][0] = FULL_SCALE as u64;
        let scale = FpuF32(FULL_SCALE);
        c.f[2] = FULL_SCALE;
        c.g[T0][0] = sx(GAINS);
        c.g[A2][0] = add(c.g[A0][0], c.g[V1][0]);
        let voice_at = r32(c, A3);
        loop {
            c.cycles += 12;
            let voice = c.lh(voice_at);
            c.g[V0][0] = shl(c.g[A1][0], 2);
            c.g[A1][0] = add(c.g[A1][0], 1);
            c.g[V1][0] = shl(voice, 5);
            c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.g[V0][0] = add(add(c.g[V0][0], c.g[V1][0]), GAINS as u64);
            let gain = Addr(r32(c, V0));
            let g = (gain.read_fpu(c) * level).div(scale);
            set_fpr(c, 0, g);
            gain.write_fpu(c, g);
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_3dc8);
            }
        }
    }

    // Smooth pans: a summed angle in the back half flips the first gain.
    c.cycles += 4;
    let smooth = c.lhu(ch + FLAGS) & F_SMOOTH != 0;
    c.g[V0][0] = 0;
    c.g[V1][0] = add(c.g[FP][0], c.g[S3][0]);
    if smooth {
        c.cycles += 12;
        c.g[A0][0] = add(add(shl(i, 2), c.g[S7][0]), LAYOUTS as u64);
        let angle = c.lhu(r32(c, V1));
        let offset = c.lh(r32(c, A0));
        c.g[V1][0] = offset;
        let back = sx((angle as u32)
            .wrapping_add(offset as u32)
            .wrapping_sub(0x4001))
            < 0x7fff;
        c.g[V0][0] = back as u64;
        c.g[V1][0] = add(c.g[S5][0], c.g[S3][0]);
        if back {
            c.cycles += 8;
            c.g[A0][0] = sx(GAINS);
            let voice = c.lh(r32(c, V1));
            c.g[V0][0] = add(shl(voice, 5), GAINS as u64);
            let gain = Addr(r32(c, V0));
            let g = -gain.read_fpu(c);
            set_fpr(c, 0, g);
            gain.write_fpu(c, g);
        }
    }
    Ok(())
}

/// `void SND_Tick(void)` (0x3b55f0): the sound driver's tick. Counts the
/// tick, runs the registered tick callbacks, then steps every allocated,
/// playing channel: the pitch LFO (recomputing and applying the pitch), the
/// volume LFO, the volume sweep (toward its target), and the envelope (the
/// level ramps by its step; at the end of a segment the next one is loaded,
/// or the voice is stopped after the last one, or when the sweep goes
/// negative). A channel whose volume inputs moved gets its volume recomputed
/// and applied.
pub fn snd_tick(c: &mut Cpu) {
    use snd_tick::*;
    use snd_tick_callee as callee;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0051_0000;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(STATE);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.lw(STATE + TICKS) as u32).wrapping_add(1));
        c.sw(STATE + TICKS, c.g[V1][0]);
        callee::TICK_STUB.call(c, 0x003b_562c)?;
        // The tick callbacks.
        c.g[V0][0] = c.lb(STATE + TICK_CALLBACKS);
        c.g[S2][0] = c.g[S0][0];
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 2;
            c.g[S0][0] = sx(STATE + CALLBACKS);
            c.g[V1][0] = c.lw(r32(c, S0));
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let callback = r32(c, V1);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                call_ptr(c, callback, 0x003b_564c)?;
                c.g[V0][0] = c.lb(STATE + TICK_CALLBACKS);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = c.lw(r32(c, S0));
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5640);
                }
            }
        }
        // The channels.
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, S3).wrapping_sub(0x5718));
        c.g[V0][0] = c.lh(r32(c, V1) + CHANNEL_COUNT);
        c.g[S1][0] = 0;
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 3;
            c.g[S2][0] = c.g[V1][0];
            c.g[V1][0] = CHANNEL_SIZE;
            loop {
                snd_tick_channel(c)?;
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = CHANNEL_SIZE;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5678);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// One channel of [`snd_tick`]: `s1` is the index, `s2` the driver state;
/// leaves the channel count in `v0` (`lh`) for the loop test.
pub(crate) fn snd_tick_channel(c: &mut Cpu) -> crate::call::Flow {
    use snd_tick::*;
    use snd_tick_callee as callee;
    let state = r32(c, S2);
    let count = |c: &mut Cpu| c.g[V0][0] = c.lh(state + CHANNEL_COUNT);
    c.cycles += 7;
    c.g[A0][0] = c.lw(state + CHANNELS);
    c.mult(0, 3, 17, 3); // index * 0x8c
    c.g[A1][0] = 1;
    c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let ch = r32(c, S0);
    c.g[V0][0] = c.lb(bw::p(ch, ALLOCATED));
    if c.g[V0][0] != c.g[A1][0] {
        count(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(ch);
    if (c.g[V0][0] as i64) < 0 {
        count(c);
        return Ok(());
    }
    // Pitch LFO: step its position (wrapping at the length) and re-pitch.
    c.cycles += 3;
    c.g[V0][0] = c.lw(bw::p(ch, PITCH_LFO));
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(bw::p(ch, PITCH_LFO_POS));
        c.g[A0][0] = c.lbu(bw::p(ch, PITCH_LFO_LEN));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = c.g[V0][0] & 0xff;
        c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        let inside = c.g[V1][0] != 0;
        c.sb(bw::p(ch, PITCH_LFO_POS), c.g[V0][0]);
        if !inside {
            c.cycles += 1;
            c.sb(bw::p(ch, PITCH_LFO_POS), 0);
        }
        c.cycles += 6;
        c.sh(bw::p(ch, PITCH_CACHE), 0);
        c.g[A0][0] = c.g[S1][0];
        callee::CALC_PITCH.call(c, 0x003b_56d8)?;
        c.g[A0][0] = c.g[S1][0];
        callee::APPLY_PITCH.call(c, 0x003b_56e0)?;
    }
    c.g[V0][0] = c.lw(bw::p(ch, VOL_LFO));
    // Volume LFO: step its position; a1 = volume changed.
    c.cycles += 2;
    c.g[A1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 8;
        c.g[V0][0] = c.lbu(bw::p(ch, VOL_LFO_POS));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lbu(bw::p(ch, VOL_LFO_LEN));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = c.g[V0][0] & 0xff;
        c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        let inside = c.g[V1][0] != 0;
        c.sb(bw::p(ch, VOL_LFO_POS), c.g[V0][0]);
        if !inside {
            c.cycles += 1;
            c.sb(bw::p(ch, VOL_LFO_POS), 0);
        }
    }
    'volume: {
        'next_segment: {
            'stop: {
                'envelope: {
                    // Sweep toward the target; a negative sweep stops.
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(bw::p(ch, SWEEP_STEP));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.lw(bw::p(ch, ENV_TICKS));
                        break 'envelope;
                    }
                    'reached: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(bw::p(ch, SWEEP));
                        c.g[A1][0] = 1;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        let rising = (c.g[V1][0] as i64) >= 0;
                        c.sw(bw::p(ch, SWEEP), c.g[V0][0]);
                        if !rising {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(bw::p(ch, SWEEP_TARGET));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            break 'reached;
                        }
                        c.cycles += 2;
                        c.g[V1][0] = c.lw(bw::p(ch, SWEEP_TARGET));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lw(bw::p(ch, SWEEP));
                    } else {
                        // Target reached: snap and end the sweep.
                        c.cycles += 3;
                        c.sw(bw::p(ch, SWEEP), c.g[V1][0]);
                        c.sw(bw::p(ch, SWEEP_STEP), 0);
                        c.g[V0][0] = c.lw(bw::p(ch, SWEEP));
                    }
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) < 0 {
                        break 'stop;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(bw::p(ch, ENV_TICKS));
                }
                // Envelope: count the segment down and ramp the level.
                c.cycles += 4;
                c.g[V1][0] = c.lw(bw::p(ch, ENV_STEP));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                let flat = c.g[V1][0] == 0;
                c.sw(bw::p(ch, ENV_TICKS), c.g[V0][0]);
                if !flat {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(bw::p(ch, ENV_LEVEL));
                    c.g[A1][0] = 1;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.sw(bw::p(ch, ENV_LEVEL), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(bw::p(ch, ENV_TICKS));
                if c.g[V0][0] != 0 {
                    break 'volume;
                }
                // Segment over: the next one, or stop after the last.
                c.cycles += 8;
                c.g[V0][0] = c.lbu(bw::p(ch, ENV_SEGMENT));
                c.g[V1][0] = c.lb(bw::p(ch, ENV_SEGMENTS));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, V0) << 24);
                c.g[A0][0] = sx(((r32(c, A0) as i32) >> 24) as u32);
                c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.sb(bw::p(ch, ENV_SEGMENT), c.g[V0][0]);
                if more {
                    break 'next_segment;
                }
                break 'stop;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(ch);
            callee::STOP.call(c, 0x003b_57b8)?;
            count(c);
            return Ok(());
        }
        // Load segment a0: {ticks (negative: forever), level << 16}; the
        // step spreads the level change over the ticks.
        c.cycles += 6;
        c.g[V0][0] = c.lw(bw::p(ch, ENV_TABLE));
        c.g[V1][0] = sx(r32(c, A0) << 3);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A0][0] = c.lw(r32(c, V1));
        let finite = (c.g[A0][0] as i64) >= 0;
        c.sw(bw::p(ch, ENV_TICKS), c.g[A0][0]);
        if !finite {
            c.cycles += 3;
            c.g[V0][0] = 0x7fff_ffff;
            c.sw(bw::p(ch, ENV_TICKS), c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(bw::p(r32(c, V1), 4));
        c.g[A0][0] = c.lw(bw::p(ch, ENV_LEVEL));
        c.g[V1][0] = c.lw(bw::p(ch, ENV_TICKS));
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
        c.div(0, 2, 3);
        c.cycles += 2;
        c.g[V0][0] = c.lo[0];
        c.sw(bw::p(ch, ENV_STEP), c.g[V0][0]);
    }
    // Volume inputs moved: recompute and apply.
    c.cycles += 2;
    if c.g[A1][0] == 0 {
        count(c);
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    callee::CALC_VOLUME.call(c, 0x003b_581c)?;
    c.g[V0][0] = c.lw(ch);
    if (c.g[V0][0] as i64) < 0 {
        count(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S1][0];
    callee::APPLY_VOLUME.call(c, 0x003b_5830)?;
    count(c);
    Ok(())
}

/// `SNDSTRM_GetStream(id)` (0x3b6a38).
pub(crate) const SNDSTRM_GET_STREAM_2: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::f_003b6a38,
};

/// `SNDSTRM_GetBufferedBytes(slot)` (0x3c6c98).
pub(crate) const SNDSTRM_GET_BUFFERED_BYTES: Callee = Callee {
    addr: 0x003c_6c98,
    f: ee::s13::f_003c6c98,
};

/// `SNDSTRM_FindRequest(id)` (0x3b7a70).
pub(crate) const SNDSTRM_FIND_REQUEST_2: Callee = Callee {
    addr: 0x003b_7a70,
    f: ee::s13::f_003b7a70,
};

/// `StreamFile_GetChannelBytes(file)` (0x3e14c8).
pub(crate) const STREAM_FILE_GET_CHANNEL_BYTES: u32 = 0x003e_14c8;

/// `int SNDSTRM_GetStatus(int id, SNDSTRMStatus* out)` (0x3b7b48): fill
/// `out` with stream `id`'s state, current request id and how many
/// milliseconds of audio are buffered (`bytes * 1000 / rate`; when nothing
/// is buffered yet, the request's channel bytes, capped at 0x3d0900, times
/// 1000 over the request's total). Returns -8 for an unknown stream, else 0.
pub fn sndstrm_get_status(c: &mut Cpu) {
    use sndstrm::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        let out = Addr(r32(c, A1));
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        out.off(OUT_MS).sw(c, 0);
        out.off(OUT_REQUEST).sw(c, 0);
        out.off(OUT_STATE).sw(c, 0);
        SNDSTRM_GET_STREAM_2.call(c, 0x003b_7b74)?;
        c.g[S1][0] = c.g[V0][0];
        let stream = Addr(r32(c, V0));
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = (-8i64) as u64;
        // false: the shared "return 0" (one more cycle).
        let mut done = true;
        if found {
            done = false;
            c.cycles += 3;
            c.g[V0][0] = stream.off(STATE).lw(c);
            out.off(OUT_STATE).sw(c, c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                let req = stream.off(REQUEST).lw(c);
                c.g[S0][0] = req;
                c.g[V0][0] = Addr((req as u32).wrapping_add(REQ_ID)).lw(c);
                out.off(OUT_REQUEST).sw(c, c.g[V0][0]);
                c.g[V1][0] = stream.off(RATE).lhu(c);
                c.g[V0][0] = 0;
                if c.g[V1][0] == 0 {
                    done = true;
                } else {
                    // Buffered bytes in milliseconds.
                    c.cycles += 7;
                    c.g[A0][0] = stream.off(BUFFER).lw(c);
                    c.g[S3][0] = 1000;
                    SNDSTRM_GET_BUFFERED_BYTES.call(c, 0x003b_7bb0)?;
                    let scaled = abi::mult(c, r32(c, V0), 1000);
                    c.g[V0][0] = sx(scaled);
                    c.g[V1][0] = stream.off(RATE).lhu(c);
                    c.cycles += 4;
                    let (ms, _) = abi::divu(c, scaled, r32(c, V1));
                    c.g[V0][0] = sx(ms);
                    out.off(OUT_MS).sw(c, c.g[V0][0]);
                    if ms == 0 {
                        // Nothing buffered: from the request's file instead.
                        c.cycles += 6;
                        c.g[A0][0] = out.off(OUT_REQUEST).lw(c);
                        SNDSTRM_FIND_REQUEST_2.call(c, 0x003b_7bd8)?;
                        c.g[S0][0] = c.g[V0][0];
                        let req = Addr(r32(c, V0));
                        let total = req.off(REQ_BYTES).lw(c);
                        c.g[V0][0] = 0;
                        if total == 0 {
                            done = true;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = stream.off(FILE).lw(c);
                            call_ptr(c, STREAM_FILE_GET_CHANNEL_BYTES, 0x003b_7bf0)?;
                            const CAP: u64 = 0x003d_0900;
                            c.g[A1][0] = req.off(REQ_BYTES).lw(c);
                            c.cycles += 6;
                            let over = CAP < c.g[V0][0];
                            c.g[A0][0] = over as u64;
                            c.g[V1][0] = if over { CAP } else { c.g[V0][0] };
                            let scaled = abi::mult(c, r32(c, V1), 1000);
                            let (ms, _) = abi::divu(c, scaled, r32(c, A1));
                            c.g[V1][0] = sx(ms);
                            out.off(OUT_MS).sw(c, c.g[V1][0]);
                        }
                    }
                }
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// SNDSTRM_PopBlock (0x3c6de0)

/// The stream manager (`D_005157B8`): +0 event count, +4 events (8 bytes:
/// u16 type, u16 stream id, u32 value), +0x304 queue pointer per stream.
pub(crate) const SNDSTRM_MAN_HI: u64 = 0x51_0000;

pub(crate) const SNDSTRM_MAN: u32 = 0x0051_57b8;

pub(crate) const SNDSTRM_QUEUES: u32 = 0x304;

/// Stream queue fields.
pub(crate) const Q_WR: u32 = 0x8;

// blocks written (consumed by every reader)
pub(crate) const Q_RD: u32 = 0xc;

// blocks read by all readers
pub(crate) const Q_POS: u32 = 0x10;

// u32 per reader: blocks read
pub(crate) const Q_RIDX: u32 = 0x28;

// u16 per reader: ring read index
pub(crate) const Q_CNT: u32 = 0x34;

// u16 per reader: blocks pending
pub(crate) const Q_SIZE: u32 = 0x40;

// u16 ring size
pub(crate) const Q_WIDX: u32 = 0x44;

// u16 ring write index
pub(crate) const Q_BYTES_A: u32 = 0x48;

pub(crate) const Q_BYTES_B: u32 = 0x4c;

pub(crate) const Q_NOTIFY: u32 = 0x54;

pub(crate) const Q_READERS: u32 = 0x62;

// u8
/// Ring entries (0x20 bytes) start at +0x7c: +4 length (bit 31: flag),
/// +8 value per reader.
pub(crate) const Q_ENTRIES: u32 = 0x7c;

/// `u32 SNDSTRM_PopBlock(int id, int reader, int* len, int* flag)`
/// (0x3c6de0): if every reader has passed the write mark, advance it (and
/// post a "block done" event when the queue asks for one); then pop the
/// reader's next block: its length and flag go to `*len`/`*flag`, the reader
/// advances, and once all readers have passed the read mark the shared byte
/// counters move. Returns the block's value for this reader (0 becomes -1),
/// or 0 when the reader has nothing pending. Kept register-shaped: the two
/// loops check events at their back edges with every register live.
pub fn sndstrm_pop_block(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = SNDSTRM_MAN_HI;
    c.g[T5][0] = c.g[A0][0];
    c.g[T3][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[T5][0] as u32) << 2);
    c.g[V0][0] = sx(SNDSTRM_MAN);
    c.g[T2][0] = 1; // all readers past the write mark
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[T0][0] = Addr(c.g[V0][0] as u32).off(SNDSTRM_QUEUES).lw(c);
    let q = Addr(c.g[T0][0] as u32);
    c.g[V0][0] = q.off(Q_READERS).lbu(c);
    c.g[T4][0] = 1; // all readers past the read mark
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[T1][0] = c.g[V0][0];
        c.g[A0][0] = addu(c.g[T0][0], Q_POS as u64);
        loop {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[T1][0] = addu(c.g[T1][0], (-1i64) as u64);
            c.g[V0][0] = q.off(Q_WR).lw(c);
            c.g[A0][0] = addu(c.g[A0][0], 4);
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                c.g[T2][0] = 0;
            }
            if c.g[T1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_6e18);
            }
        }
    }
    c.cycles += 2;
    c.g[V1][0] = addu(c.g[T0][0], Q_CNT as u64);
    if c.g[T2][0] != 0 {
        // Advance the write mark.
        c.cycles += 7;
        c.g[V0][0] = q.off(Q_WIDX).lhu(c);
        c.g[V1][0] = q.off(Q_NOTIFY).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 11) as u32);
        c.g[V0][0] = addu(c.g[V0][0], Q_ENTRIES as u64);
        c.g[T1][0] = addu(c.g[T0][0], c.g[V0][0]);
        if c.g[V1][0] != 0 {
            // Post event {type 1, id, value of reader 0}.
            c.cycles += 13;
            let man = Addr((c.g[T3][0] as u32).wrapping_add(0x57b8));
            c.g[V1][0] = man.lw(c);
            c.g[V0][0] = sx(man.0);
            c.g[V0][0] = addu(c.g[V0][0], 4);
            c.g[A0][0] = 1;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
            let ev = Addr(c.g[V1][0] as u32);
            ev.sh(c, c.g[A0][0]);
            ev.off(2).sh(c, c.g[T5][0]);
            c.g[A0][0] = Addr(c.g[T1][0] as u32).off(8).lw(c);
            ev.off(4).sw(c, c.g[A0][0]);
            c.g[V0][0] = man.lw(c);
            c.g[V0][0] = addu(c.g[V0][0], 1);
            man.sw(c, c.g[V0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = q.off(Q_WIDX).lhu(c);
        c.g[V0][0] = addu(c.g[V0][0], 1);
        q.off(Q_WIDX).sh(c, c.g[V0][0]);
        c.g[V1][0] = q.off(Q_WR).lw(c);
        c.g[V1][0] = addu(c.g[V1][0], 1);
        q.off(Q_WR).sw(c, c.g[V1][0]);
        c.g[A0][0] = q.off(Q_WIDX).lhu(c);
        c.g[V0][0] = q.off(Q_SIZE).lhu(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[A0][0] == 0 {
            q.off(Q_WIDX).sh(c, 0);
        }
        c.cycles += 1;
        c.g[V1][0] = addu(c.g[T0][0], Q_CNT as u64);
    }
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[T9][0] = c.g[V1][0];
    c.g[T5][0] = c.g[V0][0];
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lhu(c);
    let pending = c.g[A0][0] != 0;
    c.g[A0][0] = addu(c.g[T0][0], Q_RIDX as u64);
    if !pending {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    // Pop the reader's block.
    c.cycles += 28;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[T7][0] = c.g[A0][0];
    c.g[V0][0] = addu(c.g[T0][0], Q_POS as u64);
    c.g[A0][0] = addu(c.g[A0][0], c.g[T5][0]);
    c.g[T8][0] = c.g[A1][0];
    let ridx = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = ridx.lhu(c);
    c.g[A1][0] = addu(c.g[V0][0], c.g[A1][0]);
    c.g[T2][0] = c.g[V0][0];
    c.g[T1][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[T6][0] = 1;
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 11) as u32);
    c.g[V1][0] = addu(c.g[V1][0], Q_ENTRIES as u64);
    c.g[T3][0] = addu(c.g[T0][0], c.g[V1][0]);
    let entry = Addr(c.g[T3][0] as u32);
    c.g[V0][0] = entry.off(4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    Addr(c.g[A2][0] as u32).sw(c, c.g[V0][0]); // *len
    c.g[V1][0] = entry.off(4).lw(c);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 31) as u32);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]); // *flag
    c.g[V0][0] = ridx.lhu(c);
    c.g[V0][0] = addu(c.g[V0][0], 1);
    ridx.sh(c, c.g[V0][0]);
    let pos = Addr(c.g[A1][0] as u32);
    c.g[V1][0] = pos.lw(c);
    c.g[V1][0] = addu(c.g[V1][0], 1);
    pos.sw(c, c.g[V1][0]);
    // Have all readers passed the read mark? (stops at the first that has not)
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(c.g[T2][0] as u32).lw(c);
        c.g[T1][0] = addu(c.g[T1][0], 1);
        c.g[V0][0] = q.off(Q_RD).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] == 0 {
            c.g[T4][0] = 0;
        }
        let all_so_far = c.g[T4][0] == c.g[T6][0];
        c.g[T2][0] = addu(c.g[T2][0], 4);
        if !all_so_far {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = q.off(Q_READERS).lbu(c);
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_6f58);
        }
    }
    c.cycles += 2;
    c.g[A0][0] = addu(c.g[T7][0], c.g[T5][0]);
    if c.g[T4][0] != 0 {
        // Advance the read mark; move the block's bytes from A to B.
        c.cycles += 16;
        c.g[V0][0] = q.off(Q_RD).lw(c);
        c.g[V0][0] = addu(c.g[V0][0], 1);
        q.off(Q_RD).sw(c, c.g[V0][0]);
        c.g[V1][0] = entry.off(4).lw(c);
        c.g[A0][0] = q.off(Q_BYTES_A).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        q.off(Q_BYTES_A).sw(c, c.g[A0][0]);
        c.g[V0][0] = entry.off(4).lw(c);
        c.g[V1][0] = q.off(Q_BYTES_B).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
        q.off(Q_BYTES_B).sw(c, c.g[V1][0]);
        c.g[A0][0] = addu(c.g[T7][0], c.g[T5][0]);
    }
    // Wrap the read index.
    c.cycles += 7;
    let ridx = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = ridx.lhu(c);
    c.g[V0][0] = q.off(Q_SIZE).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        ridx.sh(c, 0);
    }
    c.cycles += 10;
    c.g[A0][0] = addu(c.g[T9][0], c.g[T5][0]);
    c.g[A1][0] = addu(c.g[T3][0], c.g[T8][0]);
    let cnt = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = cnt.lhu(c);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = addu(c.g[V0][0], (-1i64) as u64);
    cnt.sh(c, c.g[V0][0]);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    ret(c);
}

/// `slt_2`: signed 64-bit compare of two registers.
#[inline]
pub(crate) fn slt_2(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// `void SND_ReleaseFinishedVoices(void)` (0x3c1298): walks both channel
/// groups of the SND driver and releases the voices of every channel whose
/// hardware voice has finished. For each channel `i` with a non-zero key in
/// the IOP status table and no timed stop pending (`stop == -1`):
///
/// - first group: when the key changed since last time, the remaining
///   count is reloaded from the key's low 23 bits; when it is zero, each of
///   the channel's voices gets its status key cleared and
///   `SND_ChannelRelease(voice)` (in mode 2 the mirror voice `+0x18` too);
/// - second group: when the key changed, the remaining word is cleared;
///   when it is -1, each voice's status key is cleared and
///   `SND_ChannelRelease(voice)`.
pub fn snd_release_finished_voices(c: &mut Cpu) {
    use release::*;
    guest(c, |c| {
        let st = |o: u32| Addr(STATE + o);
        let vo = |o: u32| Addr(VOICES + o);
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        set_reg(c, A0, HI);
        frame.save(c, S2, 0x70);
        set_reg(c, V0, sx(STATE));
        frame.save(c, FP, 0x10);
        set_reg(c, S2, 0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0);
        let count_a = st(COUNT_A).lbu(c);
        set_reg(c, V1, count_a);
        set_reg(c, FP, HI);

        // First group.
        if count_a != 0 {
            c.cycles += 3;
            set_reg(c, V0, HI);
            set_reg(c, S7, sx(VOICES));
            let mut i: u32 = 0;
            loop {
                c.cycles += 6;
                let status = vo(STATUS).lw(c);
                let slot = sx(i << 3);
                set_reg(c, A2, slot);
                let entry = addu(status, slot);
                set_reg(c, V0, entry);
                let key = Addr((entry as u32).wrapping_add(KEY_A)).lw(c);
                set_reg(c, A1, key);
                set_reg(c, S6, sx(i.wrapping_add(1)));
                // true: through the one-cycle block that reloads v1.
                let short = 'ch: {
                    if key == 0 {
                        break 'ch true;
                    }
                    c.cycles += 9;
                    set_reg(c, V0, CHANNEL_SIZE as u64);
                    set_reg(c, T0, sx(STATE));
                    let off = sx(abi::mult(c, i, CHANNEL_SIZE));
                    set_reg(c, A3, off);
                    let chans = st(CHANNELS).lw(c);
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let stop = Addr((ch as u32).wrapping_add(CH_STOP)).lh(c);
                    set_reg(c, V0, u64::MAX);
                    set_reg(c, V1, sx(STATE));
                    if stop != u64::MAX {
                        break 'ch false;
                    }
                    c.cycles += 5;
                    let seen_key = sx(VOICES + SEEN_A);
                    set_reg(c, T1, seen_key);
                    let at = addu(slot, seen_key);
                    set_reg(c, V0, at);
                    let seen = Addr(at as u32).lw(c);
                    set_reg(c, V1, seen);
                    let seen_left = sx(VOICES + SEEN_A + 4);
                    set_reg(c, A0, seen_left);
                    if seen != key {
                        c.cycles += 5;
                        let at = addu(slot, seen_left);
                        set_reg(c, V1, at);
                        let left = key & 0x7f_ffff;
                        set_reg(c, V0, left);
                        Addr(at as u32).sw(c, left);
                    }
                    c.cycles += 4;
                    let at = addu(slot, seen_left);
                    set_reg(c, V0, at);
                    let left = Addr(at as u32).lw(c);
                    set_reg(c, V1, sx(STATE));
                    if left != 0 {
                        break 'ch false;
                    }
                    c.cycles += 5;
                    let chans = st(CHANNELS).lw(c);
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                    set_reg(c, V1, n);
                    set_reg(c, S1, 0);
                    if n == 0 {
                        break 'ch true;
                    }
                    c.cycles += 6;
                    set_reg(c, S3, sx(STATE));
                    set_reg(c, S2, off);
                    set_reg(c, S4, sx(VOICES));
                    set_reg(c, S5, 2);
                    let mut chans = st(CHANNELS).lw(c);
                    set_reg(c, V0, chans);
                    let mut j: u32 = 0;
                    loop {
                        c.cycles += 13;
                        let half = sx(j << 1);
                        set_reg(c, V1, half);
                        let status = vo(STATUS).lw(c);
                        let at = addu(addu(off, chans), half);
                        set_reg(c, V0, at);
                        let voice = Addr((at as u32).wrapping_add(CH_VOICES)).lh(c);
                        set_reg(c, S0, voice);
                        let v8 = sx((voice as u32) << 3);
                        set_reg(c, V1, v8);
                        set_reg(c, A0, voice);
                        let e = addu(status, v8);
                        set_reg(c, A1, e);
                        Addr((e as u32).wrapping_add(KEY_A)).sw(c, 0);
                        SND_CHANNEL_RELEASE.call(c, RET_A)?;
                        let mode = st(MODE).lbu(c);
                        set_reg(c, V0, mode);
                        if mode == 2 {
                            c.cycles += 6;
                            let status = vo(STATUS).lw(c);
                            let m8 = sx((voice as u32).wrapping_add(MIRROR) << 3);
                            set_reg(c, V0, m8);
                            let e = addu(status, m8);
                            set_reg(c, V1, e);
                            Addr((e as u32).wrapping_add(KEY_A)).sw(c, 0);
                        }
                        let chans_now = st(CHANNELS).lw(c);
                        c.cycles += 6;
                        j = j.wrapping_add(1);
                        set_reg(c, S1, sx(j));
                        let ch = addu(off, chans_now);
                        set_reg(c, V1, ch);
                        let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                        let more = slt_2(sx(j), n);
                        set_reg(c, V0, more);
                        if more == 0 {
                            break 'ch true;
                        }
                        chans = st(CHANNELS).lw(c);
                        set_reg(c, V0, chans);
                        if event_due(c) {
                            deliver_event_here(c, HEAD_A_VOICES);
                        }
                    }
                };
                if short {
                    c.cycles += 1;
                    set_reg(c, V1, sx(STATE));
                }
                c.cycles += 5;
                i = i.wrapping_add(1);
                set_reg(c, S2, sx(i));
                let count = st(COUNT_A).lbu(c);
                let more = slt_2(sx(i), count);
                set_reg(c, V0, more);
                set_reg(c, A0, HI);
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD_A);
                }
            }
        }

        // Second group: channels COUNT_A.. (status slots from 0).
        c.cycles += 5;
        set_reg(c, A0, HI);
        set_reg(c, V0, sx(STATE));
        let count_b = st(COUNT_B).lbu(c);
        set_reg(c, V1, count_b);
        set_reg(c, S2, 0);
        if count_b != 0 {
            c.cycles += 4;
            set_reg(c, V0, HI);
            set_reg(c, S7, sx(VOICES));
            let mut status = vo(STATUS).lw(c);
            set_reg(c, V0, status);
            let mut i: u32 = 0;
            loop {
                c.cycles += 5;
                let slot16 = sx(i << 4);
                set_reg(c, V1, slot16);
                let entry = addu(status, slot16);
                set_reg(c, V0, entry);
                let key = Addr((entry as u32).wrapping_add(KEY_B)).lw(c);
                set_reg(c, A3, key);
                set_reg(c, S6, sx(i.wrapping_add(1)));
                let short = 'ch: {
                    if key == 0 {
                        break 'ch true;
                    }
                    c.cycles += 11;
                    set_reg(c, T0, sx(STATE));
                    set_reg(c, T2, CHANNEL_SIZE as u64);
                    let first = st(COUNT_A).lbu(c);
                    let chans = st(CHANNELS).lw(c);
                    set_reg(c, V1, chans);
                    let idx = addu(sx(i), first);
                    let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let stop = Addr((ch as u32).wrapping_add(CH_STOP)).lh(c);
                    set_reg(c, A2, stop);
                    set_reg(c, V0, u64::MAX);
                    set_reg(c, A0, HI);
                    if stop != u64::MAX {
                        break 'ch false;
                    }
                    c.cycles += 6;
                    let slot = sx(i << 3);
                    set_reg(c, A1, slot);
                    let seen_key = sx(VOICES + SEEN_B);
                    set_reg(c, T1, seen_key);
                    let at = addu(slot, seen_key);
                    set_reg(c, V1, at);
                    let seen = Addr(at as u32).lw(c);
                    set_reg(c, V0, seen);
                    let seen_left = sx(VOICES + SEEN_B + 4);
                    set_reg(c, A0, seen_left);
                    if seen != key {
                        c.cycles += 2;
                        let at = addu(slot, seen_left);
                        set_reg(c, V0, at);
                        Addr(at as u32).sw(c, 0);
                    }
                    c.cycles += 4;
                    let at = addu(slot, seen_left);
                    set_reg(c, V0, at);
                    let left = Addr(at as u32).lw(c);
                    set_reg(c, V1, left);
                    set_reg(c, A0, HI);
                    if left != stop {
                        break 'ch false;
                    }
                    c.cycles += 8;
                    let first = st(COUNT_A).lbu(c);
                    let chans = st(CHANNELS).lw(c);
                    set_reg(c, A0, chans);
                    let idx = addu(sx(i), first);
                    let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                    let ch = addu(off, chans);
                    set_reg(c, V0, ch);
                    let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                    set_reg(c, V1, n);
                    set_reg(c, S1, 0);
                    if n == 0 {
                        break 'ch true;
                    }
                    c.cycles += 4;
                    set_reg(c, S3, sx(STATE));
                    set_reg(c, S5, sx(VOICES));
                    set_reg(c, S4, CHANNEL_SIZE as u64);
                    let mut first = st(COUNT_A).lbu(c);
                    set_reg(c, V1, first);
                    let mut j: u32 = 0;
                    loop {
                        c.cycles += 24;
                        let half = sx(j << 1);
                        set_reg(c, A2, half);
                        let chans = st(CHANNELS).lw(c);
                        set_reg(c, A0, chans);
                        j = j.wrapping_add(1);
                        set_reg(c, S1, sx(j));
                        let status = vo(STATUS).lw(c);
                        let idx = addu(sx(i), first);
                        let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                        let at = addu(addu(off, chans), half);
                        set_reg(c, V0, at);
                        let voice = Addr((at as u32).wrapping_add(CH_VOICES)).lh(c);
                        set_reg(c, S0, voice);
                        let v16 = sx((voice as u32).wrapping_sub(first as u32) << 4);
                        set_reg(c, V1, v16);
                        set_reg(c, A0, voice);
                        let e = addu(status, v16);
                        set_reg(c, A1, e);
                        Addr((e as u32).wrapping_add(KEY_B)).sw(c, 0);
                        SND_CHANNEL_RELEASE.call(c, RET_B)?;
                        let first_now = st(COUNT_A).lbu(c);
                        let chans = st(CHANNELS).lw(c);
                        set_reg(c, A0, chans);
                        let idx = addu(sx(i), first_now);
                        let off = sx(abi::mult(c, idx as u32, CHANNEL_SIZE));
                        let ch = addu(off, chans);
                        set_reg(c, V0, ch);
                        let n = Addr((ch as u32).wrapping_add(CH_VOICE_COUNT)).lbu(c);
                        let more = slt_2(sx(j), n);
                        set_reg(c, V1, more);
                        if more == 0 {
                            break 'ch true;
                        }
                        first = st(COUNT_A).lbu(c);
                        set_reg(c, V1, first);
                        if event_due(c) {
                            deliver_event_here(c, HEAD_B_VOICES);
                        }
                    }
                };
                if short {
                    c.cycles += 1;
                    set_reg(c, A0, HI);
                }
                c.cycles += 6;
                i = i.wrapping_add(1);
                set_reg(c, S2, sx(i));
                set_reg(c, V1, sx(STATE));
                let count = st(COUNT_B).lbu(c);
                let more = slt_2(sx(i), count);
                set_reg(c, V0, more);
                if more == 0 {
                    break;
                }
                status = vo(STATUS).lw(c);
                set_reg(c, V0, status);
                if event_due(c) {
                    deliver_event_here(c, HEAD_B);
                }
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x90);
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

pub(crate) mod snd_req {

    /// The stream's active-request list head.
    pub const STREAM_ACTIVE: u32 = 0x11c;

    /// Owner id (stream index in the low byte).
    pub const OWNER: u32 = 0xc;

    /// Consumed position, end position, bytes left.
    pub const POS: u32 = 0x14;

    pub const END: u32 = 0x18;

    pub const LEFT: u32 = 0x1c;
}

pub(crate) mod set_volume {

    pub const START: u32 = 0x003b_85f0;

    pub const END: u32 = 0x003b_86c0;

    /// Loop head (the per-channel body), the resume point after an unwind.
    pub const HEAD: u32 = 0x003b_8638;
}

pub(crate) mod snd_vol {

    /// `lui 0x51` (the state block's high half, left in v0).
    pub const STATE_HI: u64 = 0x51_0000;

    pub const STATE: u32 = 0x0050_a8e8;

    /// `+0x1f0` the channel table.
    pub const CHANNELS: u32 = 0x1f0;

    pub const CHANNEL_SIZE: u64 = 0x8c;

    /// Channel `+0x4` the channel the send applies to (`s16`).
    pub const SEND_CHANNEL: u32 = 0x4;

    /// Channel `+0x30` cleared on a volume change.
    pub const RAMP: u32 = 0x30;

    /// Channel `+0x38` the handle's volume (`volume << 16`).
    pub const VOLUME: u32 = 0x38;

    /// Channel `+0x62` effect send level (`s16`, > 0: apply the send).
    pub const SEND: u32 = 0x62;
}

pub(crate) mod sndstrm_service {
    use super::ee;
    use super::Callee;

    /// `Stream*[]` and the sound state (`+0x46` the stream count).
    pub const STREAMS: u32 = 0x0050_a0a0;

    pub const SND_STATE: u32 = 0x0050_a8e8;

    pub const STREAM_COUNT: u32 = 0x46;

    /// `Stream`: its stream file, playback slot, state (signed byte: 1
    /// feeding, 2 playing), the open flag, the prebuffer record.
    pub const FILE: u32 = 0x0;

    pub const SLOT: u32 = 0x8;

    pub const STATE: u32 = 0x10;

    pub const OPEN: u32 = 0x124;

    pub const STATE_FEEDING: u64 = 1;

    pub const STATE_PLAYING: u64 = 2;

    /// Chunks taken per pass when not feeding.
    pub const CHUNKS_PER_PASS: u64 = 8;

    pub const SND_LOCK: Callee = Callee {
        addr: 0x003b_58a0,
        f: ee::s13::SND_Lock,
    };

    pub const SND_UNLOCK: Callee = Callee {
        addr: 0x003b_58d8,
        f: ee::s13::SND_Unlock,
    };

    pub const GET_BUFFERED_BYTES: Callee = Callee {
        addr: 0x003c_6c98,
        f: ee::s13::SNDSTRM_GetBufferedBytes,
    };

    pub const STOP_SLOT: Callee = Callee {
        addr: 0x003c_6ce8,
        f: ee::s13::SNDSTRM_StopSlot,
    };

    pub const START_PLAYBACK: Callee = Callee {
        addr: 0x003b_68b8,
        f: ee::s13::SNDSTRM_StartPlayback,
    };

    pub const IS_PREBUFFERING: Callee = Callee {
        addr: 0x003b_70f8,
        f: ee::s13::SNDSTRM_IsPrebuffering,
    };

    pub const GET_FREE_BLOCKS: Callee = Callee {
        addr: 0x003c_6c28,
        f: ee::s13::SNDSTRM_GetFreeBlocks,
    };

    pub const HANDLE_CHUNK: Callee = Callee {
        addr: 0x003b_7080,
        f: ee::s13::SNDSTRM_HandleChunk,
    };

    /// `StreamFile_GetChunk(file)` (cross-shard).
    pub const STREAM_FILE_GET_CHUNK: u32 = 0x003e_12e0;
}

pub(crate) mod sndstrm_release {

    /// Release queue: count at 0x5157b8, 8-byte entries {0, slot, bytes}
    /// from 0x5157c0; slot buffers `Buffer*[]` at 0x5157b8 + 0x304.
    pub const QUEUE: u32 = 0x0051_57b8;

    #[cfg(test)]
    pub const BUFFERS: u32 = QUEUE + 0x304;

    /// Buffer: the reader id (signed byte), bytes held, release
    /// notification wanted.
    pub const READER: u32 = 0x42;

    pub const HELD: u32 = 0x4c;

    pub const NOTIFY: u32 = 0x58;
}

/// `void SNDSTRM_ConsumeCallback(?, unsigned n, SndStream *stream)`
/// (0x3b6b10): consume `n` bytes from the stream's active requests: advance
/// the head request (at most its bytes left; the rest carries over to the
/// next head), retiring it (`SNDSTRM_RetireRequest`) once its position
/// reaches its end. After 200 rounds the head is retired unconditionally.
pub fn sndstrm_consume_callback(c: &mut Cpu) {
    use snd_req::*;
    const RETIRE: Callee = Callee {
        addr: 0x003b_6a70,
        f: ee::s13::f_003b6a70,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0; // the carried-over remainder
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0; // rounds
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let stream = r32(c, S2);
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 201) as u64;
            c.g[A0][0] = Addr(stream.wrapping_add(STREAM_ACTIVE)).lw(c);
            let req = r32(c, A0);
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = Addr(req.wrapping_add(OWNER)).lw(c);
                RETIRE.call(c, 0x003b_6b4c)?;
                frame.restore(c, S0, 0x30);
                break;
            }
            // Take at most the request's bytes left; keep the rest.
            c.cycles += 4;
            c.g[V0][0] = Addr(req.wrapping_add(LEFT)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(req.wrapping_add(LEFT)).lw(c);
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            }
            c.cycles += 11;
            let n = c.g[A1][0] as u32;
            c.g[V0][0] = sx((Addr(req.wrapping_add(POS)).lw(c) as u32).wrapping_add(n));
            Addr(req.wrapping_add(POS)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx((Addr(req.wrapping_add(LEFT)).lw(c) as u32).wrapping_sub(n));
            Addr(req.wrapping_add(LEFT)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(req.wrapping_add(POS)).lw(c);
            c.g[V1][0] = Addr(req.wrapping_add(END)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(req.wrapping_add(OWNER)).lw(c);
                RETIRE.call(c, 0x003b_6ba8)?;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                break;
            }
            c.cycles += 2;
            c.g[S0][0] = 0;
            if event_due(c) {
                deliver_event_here(c, 0x003b_6b30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 SND_UDiv64by32(u64 x /*a0*/, u32 d /*a1*/)` (0x3c64a8): the
/// quotient of x / d by 32 steps of shift-and-subtract long division (x
/// goes through the stack to split it into lo/hi words; the remainder stays
/// in v1).
pub fn snd_udiv64by32(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    let s = sp(c);
    c.g[A2][0] = 0; // quotient
    let x = c.g[A0][0];
    c.sd(s, x);
    c.g[A3][0] = 0x1f; // bit counter
    c.g[A0][0] = c.lw(s); // lo
    c.g[V1][0] = c.lw(s.wrapping_add(4)); // hi (the running remainder)
    c.g[V0][0] = sx(r32(c, A0) >> 31);
    loop {
        c.cycles += 6;
        // (hi:lo) <<= 1, q <<= 1
        c.g[V1][0] = sx((r32(c, V1) << 1).wrapping_add(r32(c, V0)));
        c.g[A2][0] = sx(r32(c, A2) << 1);
        c.g[V0][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
        c.g[A0][0] = sx(r32(c, A0) << 1);
        if c.g[V0][0] == 0 {
            // hi >= d: subtract, set the quotient bit.
            c.cycles += 2;
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A1)));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        }
        c.cycles += 3;
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, A0) >> 31);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_64c8);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0];
    frame.close(c);
}

pub(crate) const SND_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};

pub(crate) const SND_CHANNEL_CALC_VOLUME: Callee = Callee {
    addr: 0x003b_a7b0,
    f: ee::s13::SND_ChannelCalcVolume,
};

pub(crate) const SND_NEXT_HANDLE_CHANNEL: Callee = Callee {
    addr: 0x003b_aa00,
    f: ee::s13::SND_NextHandleChannel,
};

pub(crate) const SND_CHANNEL_APPLY_VOLUME_2: Callee = Callee {
    addr: 0x003c_2a50,
    f: ee::s13::SND_ChannelApplyVolume,
};

pub(crate) const SND_CHANNEL_APPLY_SEND: Callee = Callee {
    addr: 0x003c_27e0,
    f: ee::s13::SND_ChannelApplySend,
};

/// `int SND_SetVolume(int handle, unsigned volume)` (0x3b85f0): set the
/// volume of every voice channel of a handle and recompute and apply it
/// (and the effect send where the channel has one). Returns the handle's
/// first channel (or the error from `SND_HandleToChannel`), but 0 at once
/// when a channel already has this volume (the rest are not visited).
///
/// The original is a `match pc` dispatcher whose loop returns on an unwind
/// request, so the loop head is an entry point too (registers: s1 the first
/// channel, s2 the volume, s3 0x8c, s4 the state block, a1 the cursor; the
/// cursor also at `0(sp)`).
pub fn snd_set_volume(c: &mut Cpu) {
    use set_volume::*;
    use snd_vol::*;
    let frame = Frame { size: 0x70 };
    let resumed = match c.pc {
        START => false,
        HEAD => true,
        pc => {
            c.jump_out(pc, START, END);
            return;
        }
    };
    guest(c, |c| {
        if !resumed {
            c.cycles += 12;
            Frame::open(c, frame.size);
            frame.save(c, S2, 0x40);
            frame.save(c, S1, 0x50);
            frame.save(c, S0, 0x60);
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S2][0] = c.g[A1][0];
            SND_HANDLE_TO_CHANNEL.call(c, 0x003b_8614)?;
            let first = c.g[V0][0];
            c.g[S1][0] = first;
            c.g[V1][0] = u64::MAX;
            if (first as i64) < 0 {
                return set_volume_exit(c, frame);
            }
            // The cursor starts at -1; the first step finds the first
            // channel.
            c.cycles += 5;
            c.g[V0][0] = STATE_HI;
            c.g[S4][0] = sx(STATE);
            c.sw(sp(c), u64::MAX);
            c.g[S3][0] = CHANNEL_SIZE;
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            if !set_volume_next(c)? {
                return set_volume_exit(c, frame);
            }
            if back_edge_unwinds(c, HEAD) {
                return Ok(());
            }
        }
        loop {
            // Loop head: the cursor's channel (a1).
            c.cycles += 7;
            let volume = sx((c.g[S2][0] as u32) << 16);
            c.g[A2][0] = volume;
            let table = c.lw((c.g[S4][0] as u32).wrapping_add(CHANNELS));
            c.g[V1][0] = table;
            c.mult(0, V0, A1, S3);
            let ch = sx((table as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = ch;
            let ch = ch as u32;
            let current = c.lw(ch.wrapping_add(VOLUME));
            c.g[A0][0] = current;
            c.sw(ch.wrapping_add(RAMP), 0);
            if current == volume {
                c.cycles += 2;
                c.g[V0][0] = 0;
                return set_volume_epilogue(c, frame);
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            c.sw(ch.wrapping_add(VOLUME), volume);
            SND_CHANNEL_CALC_VOLUME.call(c, 0x003b_8668)?;
            c.g[A0][0] = c.lw(sp(c));
            SND_CHANNEL_APPLY_VOLUME_2.call(c, 0x003b_8670)?;
            let send = c.lh(ch.wrapping_add(SEND));
            c.g[V0][0] = send;
            c.g[A0][0] = c.g[S1][0];
            if (send as i64) > 0 {
                c.cycles += 3;
                c.g[A0][0] = c.lh(ch.wrapping_add(SEND_CHANNEL));
                c.g[A1][0] = 0;
                SND_CHANNEL_APPLY_SEND.call(c, 0x003b_8688)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
            }
            if !set_volume_next(c)? {
                return set_volume_exit(c, frame);
            }
            if back_edge_unwinds(c, HEAD) {
                return Ok(());
            }
        }
    })
}

/// `SND_NextHandleChannel(first, sp)` for [`snd_set_volume`]: true when
/// there is another channel (its index then in a1).
pub(crate) fn set_volume_next(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[A1][0] = c.g[SP][0];
    SND_NEXT_HANDLE_CHANNEL.call(c, 0x003b_8694)?;
    if c.g[V0][0] != 0 {
        c.g[A1][0] = c.lw(sp(c));
        return Ok(true);
    }
    Ok(false)
}

/// Return the first channel (s1).
pub(crate) fn set_volume_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 1;
    c.g[V0][0] = c.g[S1][0];
    set_volume_epilogue(c, frame)
}

pub(crate) fn set_volume_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 8;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void SNDSTRM_Service(void)` (0x3b71d8): under the sound lock, for each
/// open stream: a playing stream whose buffer ran dry restarts (the queued
/// request at_2 +0x84..+0xec moves down to +0x1c, +0x18 to +0x14; stop the
/// slot, start playback); then, unless it is still prebuffering, feed it
/// chunks from its stream file: as many as the slot has free blocks when
/// feeding, else up to 8, stopping at_2 the first missing or unhandled one.
/// Frame 0x90 (s0-s7).
pub fn sndstrm_service(c: &mut Cpu) {
    use sndstrm_service::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        for (reg, off) in [
            (S3, 0x50),
            (S7, 0x10),
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0);
        c.g[S3][0] = 0;
        SND_LOCK.call(c, 0x003b_7208)?;
        c.g[S7][0] = 0x51_0000;
        c.g[V1][0] = c.lbu(SND_STATE + STREAM_COUNT);
        c.g[V0][0] = 0x51_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[S6][0] = STATE_PLAYING;
            c.g[S5][0] = sx(STREAMS);
            c.g[V0][0] = sx(r32(c, S3) << 2);
            // 0x3b7230: stream s3 (s4 = s3 + 1).
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                c.g[S0][0] = c.lw(r32(c, V0));
                c.g[S4][0] = sx(r32(c, S3).wrapping_add(1));
                service_stream(c)?;
                // 0x3b73c4: next stream.
                c.cycles += 5;
                c.g[S3][0] = c.g[S4][0];
                c.g[V0][0] = c.lbu(at_2(c, V1, STREAM_COUNT));
                let more = (c.g[S3][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = sx(r32(c, S3) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_7230);
                }
            }
        }
        c.cycles += 13;
        SND_UNLOCK.call(c, 0x003b_73e0)?;
        for (reg, off) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`sndstrm_service`] for the stream in s0 (s3 its index);
/// ends with v1 = the sound state, as every path of the original does.
pub(crate) fn service_stream(c: &mut Cpu) -> crate::call::Flow {
    use sndstrm_service::*;
    let state_addr = sx(r32(c, S7).wrapping_sub(0x5718));
    let stream = r32(c, S0);
    if stream == 0 {
        c.cycles += 1;
        c.g[V1][0] = state_addr;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(stream.wrapping_add(OPEN));
    c.g[V1][0] = state_addr;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = sx(c.lbu(stream.wrapping_add(STATE)) as u8 as i8 as i32 as u32);
    if c.g[V0][0] == c.g[S6][0] {
        // Playing: restart once the buffer ran dry.
        c.cycles += 5;
        c.g[A0][0] = c.lw(stream.wrapping_add(SLOT));
        GET_BUFFERED_BYTES.call(c, 0x003b_726c)?;
        let buffered = (c.g[V0][0] as i64) > 0;
        c.g[V1][0] = state_addr;
        if buffered {
            return Ok(());
        }
        restart_stream(c, stream)?;
    }

    // 0x3b7340
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    IS_PREBUFFERING.call(c, 0x003b_7348)?;
    let prebuffering = c.g[V0][0] != 0;
    c.g[V1][0] = state_addr;
    if prebuffering {
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = sx(c.lbu(stream.wrapping_add(STATE)) as u8 as i8 as i32 as u32);
    c.g[V1][0] = STATE_FEEDING;
    c.g[S1][0] = CHUNKS_PER_PASS;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 6;
        c.g[A0][0] = c.lw(stream.wrapping_add(SLOT));
        GET_FREE_BLOCKS.call(c, 0x003b_7374)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = state_addr;
        if (c.g[S1][0] as i64) <= 0 {
            return Ok(());
        }
        c.cycles += 2;
    } else {
        c.cycles += 2;
    }
    c.g[S2][0] = 0;
    c.cycles += 1;
    c.g[A0][0] = c.lw(stream.wrapping_add(FILE));
    // 0x3b7394: s1 chunks at_2 most.
    loop {
        c.cycles += 4;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
        call_ptr(c, STREAM_FILE_GET_CHUNK, 0x003b_739c)?;
        c.g[A1][0] = c.g[V0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            HANDLE_CHUNK.call(c, 0x003b_73ac)?;
            c.g[S2][0] = c.g[V0][0];
        }
        c.cycles += 2;
        c.g[V1][0] = state_addr;
        if c.g[S2][0] == 0 {
            return Ok(());
        }
        c.cycles += 2;
        if (c.g[S1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V1][0] = state_addr;
            return Ok(());
        }
        c.g[A0][0] = c.lw(stream.wrapping_add(FILE));
        if event_due(c) {
            deliver_event_here(c, 0x003b_7394);
        }
    }
}

/// 0x3b7280: a playing stream ran dry: move the queued request (0x60
/// bytes at_2 +0x84, then 8 at_2 +0xe4) down to +0x1c and +0x18 to +0x14
/// (unaligned copies when either end is not 8-byte aligned), clear +0x9c,
/// stop the slot and start playback again.
pub(crate) fn restart_stream(c: &mut Cpu, stream: u32) -> crate::call::Flow {
    use sndstrm_service::*;
    c.cycles += 10;
    c.g[A0][0] = sx(stream.wrapping_add(0x1c));
    c.g[V1][0] = sx(stream.wrapping_add(0x84));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.lwl(A1, stream.wrapping_add(0x1b));
    c.lwr(A1, stream.wrapping_add(0x18));
    c.swl(A1, stream.wrapping_add(0x17));
    c.swr(A1, stream.wrapping_add(0x14));
    let aligned = c.g[V0][0] & 7 == 0;
    c.g[V0][0] = sx(stream.wrapping_add(0xe4));
    if !aligned {
        // 0x3b729c
        loop {
            c.cycles += 19;
            let (src, dst) = (r32(c, V1), r32(c, A0));
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                let o = 8 * k as u32;
                c.ldl(reg, src.wrapping_add(o + 7));
                c.ldr(reg, src.wrapping_add(o));
            }
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                let o = 8 * k as u32;
                c.sdl(reg, dst.wrapping_add(o + 7));
                c.sdr(reg, dst.wrapping_add(o));
            }
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_729c);
            }
        }
    } else {
        // 0x3b72f0
        loop {
            c.cycles += 11;
            let (src, dst) = (r32(c, V1), r32(c, A0));
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                c.g[reg][0] = c.ld(src.wrapping_add(8 * k as u32));
            }
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                c.sd(dst.wrapping_add(8 * k as u32), c.g[reg][0]);
            }
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_72f0);
            }
        }
    }
    // 0x3b731c
    c.cycles += 9;
    c.ldl(V0, at_2(c, V1, 7));
    c.ldr(V0, r32(c, V1));
    c.sdl(V0, at_2(c, A0, 7));
    c.sdr(V0, r32(c, A0));
    c.g[A0][0] = c.lw(stream.wrapping_add(SLOT));
    c.sw(stream.wrapping_add(0x9c), 0);
    STOP_SLOT.call(c, 0x003b_7338)?;
    c.g[A0][0] = c.g[S0][0];
    START_PLAYBACK.call(c, 0x003b_7340)?;
    Ok(())
}

/// `void SNDSTRM_ReleaseBytes(int slot, int reader, int bytes)`
/// (0x3c7010): when `reader` owns slot `slot`'s buffer, take `bytes` off
/// the bytes it holds and, if the buffer wants notifications, queue a
/// release entry {0, slot, bytes}. A leaf.
pub fn sndstrm_release_bytes(c: &mut Cpu) {
    use sndstrm_release::*;
    c.cycles += 10;
    c.g[T0][0] = 0x51_0000;
    c.g[V1][0] = sx(r32(c, A0) << 2);
    c.g[T1][0] = sx(QUEUE);
    c.g[V1][0] = sx(QUEUE.wrapping_add(r32(c, V1)));
    let buf = c.lw(at_2(c, V1, 0x304));
    c.g[A3][0] = buf;
    let buf = buf as u32;
    c.g[V0][0] = sx(c.lbu(buf.wrapping_add(READER)) as u8 as i8 as i32 as u32);
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 6;
        c.g[V0][0] = c.lw(buf.wrapping_add(HELD));
        c.g[V1][0] = c.lw(buf.wrapping_add(NOTIFY));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A2)));
        c.sw(buf.wrapping_add(HELD), c.g[V0][0]);
        let notify = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(QUEUE);
        if notify {
            c.cycles += 9;
            c.g[V1][0] = sx(QUEUE + 4);
            let entry = (r32(c, V0) << 3).wrapping_add(QUEUE + 4);
            c.g[V0][0] = sx(entry);
            c.sw(entry.wrapping_add(4), c.g[A2][0]);
            c.sh(entry.wrapping_add(2), c.g[A0][0]);
            c.sh(entry, 0);
            c.g[V1][0] = c.lw(QUEUE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            c.sw(QUEUE, c.g[V1][0]);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ------------------------------------------------- SND_DspClip16_Process

/// `int SND_DspClip16_Process(DspClip16* o, int count, float* in, s16* out)`
/// (0x3ca6a8): the last stage of a sound DSP chain. If the object has an
/// upstream stage (`o->+0x8`), its process method (first word of the stage)
/// is called first as `(stage, count, out, in)` with `t0 = o->+0x1a`; a
/// result `<= 0` is returned as is. Then `count` floats from `in` are
/// clamped to [-32768, 32767] and packed as 16-bit samples into `out`, 16
/// samples (four VU0 vectors) per step. Returns `count`.
pub fn snd_dsp_clip16_process(c: &mut Cpu) {
    /// The upstream DSP stage (0: none).
    const UPSTREAM: u32 = 0x8;
    /// The byte passed to the upstream stage in t0.
    const CHANNEL: u32 = 0x1a;
    /// Return address of the upstream call.
    const RET_UPSTREAM: u32 = 0x003c_a6e8;
    /// The clamp-and-pack loop head.
    const LOOP: u32 = 0x003c_a714;

    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        let stage = Addr(c.g[A0][0] as u32).off(UPSTREAM).lw(c);
        c.g[V0][0] = stage;
        c.g[S2][0] = c.g[A2][0];

        let mut clip = true;
        if stage != 0 {
            // Registers at the call: (stage, count, out, in), t0 = channel.
            c.cycles += 8;
            c.g[T0][0] = Addr(c.g[A0][0] as u32).off(CHANNEL).lbu(c);
            c.g[A0][0] = stage;
            c.g[A2][0] = c.g[S1][0];
            let method = Addr(stage as u32).lw(c);
            c.g[V0][0] = method;
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, method as u32, RET_UPSTREAM)?;
            clip = (c.g[V0][0] as i64) > 0;
        }

        if clip {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0]; // floats left
            c.g[A1][0] = c.g[S2][0]; // input cursor
            c.g[A2][0] = c.g[S1][0]; // output cursor
            c.g[T0][0] = 0xffff_ffff_ffff_8000;
            c.g[T1][0] = 0x7fff;
            // vf1.x = -32768.0, vf2.x = 32767.0
            c.qmtc2(T0, 1);
            c.qmtc2(T1, 2);
            c.vu0().itof(0, 0b1111, 1, 1);
            c.vu0().itof(0, 0b1111, 2, 2);
            loop {
                c.cycles += 28;
                let input = c.g[A1][0] as u32;
                c.lqc2(3, input);
                c.lqc2(4, input.wrapping_add(0x10));
                c.lqc2(5, input.wrapping_add(0x20));
                c.lqc2(6, input.wrapping_add(0x30));
                for vf in 3..=6u8 {
                    c.vu0().max(0b1111, vf, vf, Src::Bc(1, X));
                }
                for vf in 3..=6u8 {
                    c.vu0().mini(0b1111, vf, vf, Src::Bc(2, X));
                }
                for vf in 3..=6u8 {
                    c.vu0().ftoi(0, 0b1111, vf, vf);
                }
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                c.g[A1][0] = sx(input.wrapping_add(0x40));
                c.qmfc2(A3, 3);
                c.qmfc2(T0, 4);
                c.qmfc2(T1, 5);
                c.qmfc2(T2, 6);
                c.ppach(A3, T0, A3);
                c.ppach(T1, T2, T1);
                let out = c.g[A2][0] as u32;
                c.sq(A3, out);
                c.sq(T1, out.wrapping_add(0x10));
                if (c.g[A0][0] as i64) <= 0 {
                    break;
                }
                c.g[A2][0] = sx(out.wrapping_add(0x20));
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod prebuffering {

    /// Entry, the "prebuffer done" resume point (an event check branches
    /// there), and the end of the function.
    pub const ENTRY: u32 = 0x003b_70f8;

    pub const RESET: u32 = 0x003b_7184;

    pub const END: u32 = 0x003b_71d4;

    /// `SNDSTRM`: the stream file handle at +0, the queued-request count at
    /// +0x130, the prebuffer state at +0x134.
    pub const STREAM_FILE: u32 = 0x0;

    pub const QUEUED: u32 = 0x130;

    pub const PREBUFFER: u32 = 0x134;

    /// Prebuffer state: data rate (bytes per second) at +0x10, the target
    /// in milliseconds at +0x20 (negative: wait forever, 0: done).
    pub const RATE: u32 = 0x10;

    pub const TARGET_MS: u32 = 0x20;

    /// `StreamFile_GetChannelBytes(file)`, `StreamFile_GetReadState(file)`
    /// (other shard, through the table); read state 2 = at end of file.
    pub const GET_CHANNEL_BYTES: u32 = 0x003e_14c8;

    pub const GET_READ_STATE: u32 = 0x003e_14f8;

    pub const READ_AT_END: u64 = 2;

    /// The buffered byte count is capped at 4,000,000 before `* 1000`.
    pub const BYTES_CAP: u64 = 0x003d_0900;
}

pub(crate) mod snd_tags {
    use super::*;

    /// The read pointer.
    pub const P: u32 = 0x0;

    /// The current tag byte.
    pub const TAG: u32 = 0x4;

    /// The tag's value when its payload is at most 4 bytes (big endian).
    pub const VALUE: u32 = 0x8;

    /// The payload pointer.
    pub const PAYLOAD: u32 = 0xc;

    /// The payload length.
    pub const LEN: u32 = 0x10;

    /// `u32 SND_ReadBigEndian(const u8* p, int n)` (0x3c72f8).
    pub const READ_BIG_ENDIAN: Callee = Callee {
        addr: 0x003c_72f8,
        f: ee::s13::SND_ReadBigEndian,
    };
}

pub(crate) mod sndstrm_cb {

    /// `int`: queued callback events.
    pub const COUNT: u32 = 0x0051_57b8;

    /// The events (8 bytes: `u16 kind`, `u16 stream`, `u32 value`).
    pub const EVENTS: u32 = 0x0051_57bc;

    pub const EVENT_SIZE: u32 = 8;

    pub const EV_KIND: u32 = 0;

    pub const EV_STREAM: u32 = 2;

    pub const EV_VALUE: u32 = 4;

    /// `Stream*` per stream id, after the events.
    pub const STREAMS: u32 = EVENTS + 0x300;

    /// Stream: `value_cb(value, user)`, `stream_cb(id, value, user)`, user.
    pub const VALUE_CB: u32 = 0x54;

    pub const STREAM_CB: u32 = 0x58;

    pub const USER: u32 = 0x5c;
}

/// Where `SNDSTRM_IsPrebuffering` goes after its checks.
pub(crate) enum PrebufferNext {
    /// The epilogue (v0 holds the result).
    Done,
    /// Prebuffering finished: clear the target, return 0.
    Reset,
    /// An event at the branch to `RESET` asked the thread to unwind.
    Unwind,
}

/// The checks of `SNDSTRM_IsPrebuffering` from its entry (prologue
/// included) up to the epilogue or the reset.
pub(crate) fn sndstrm_prebuffer_checks(c: &mut Cpu) -> Result<PrebufferNext, crate::call::Unwound> {
    use prebuffering::*;
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    let strm = c.g[A0][0];
    c.g[S1][0] = strm;
    frame.save(c, S0, 0x20);
    let strm = strm as u32;
    let pre = c.lw(strm.wrapping_add(PREBUFFER));
    c.g[S0][0] = pre;
    c.g[V0][0] = 0;
    if pre == 0 {
        return Ok(PrebufferNext::Done);
    }
    let pre = pre as u32;
    c.cycles += 3;
    let rate = c.lw(pre.wrapping_add(RATE));
    c.g[V0][0] = 0;
    if rate == 0 {
        return Ok(PrebufferNext::Done);
    }
    c.cycles += 3;
    let target = c.lw(pre.wrapping_add(TARGET_MS));
    c.g[V0][0] = 1;
    if (target as i64) < 0 {
        return Ok(PrebufferNext::Done);
    }
    c.cycles += 3;
    let target = c.lw(pre.wrapping_add(TARGET_MS));
    c.g[V0][0] = 0;
    if target == 0 {
        return Ok(PrebufferNext::Done);
    }

    // Buffered milliseconds: min(bytes, 4e6) * 1000 / rate.
    c.cycles += 10;
    c.g[A0][0] = c.lw(strm.wrapping_add(STREAM_FILE));
    call_ptr(c, GET_CHANNEL_BYTES, 0x003b_7144)?;
    c.g[A2][0] = 1000;
    let rate = c.lw(pre.wrapping_add(RATE));
    c.g[A1][0] = rate;
    c.g[V1][0] = BYTES_CAP;
    let over = BYTES_CAP < c.g[V0][0];
    c.g[A0][0] = over as u64;
    c.cycles += 8;
    if !over {
        c.g[V1][0] = c.g[V0][0];
    }
    c.mult(0, V1, V1, A2);
    c.g[V0][0] = c.lw(pre.wrapping_add(TARGET_MS));
    // (The divisor check `teq a1, zero` never traps: rate is non-zero.)
    c.divu(0, V1, A1);
    let short = c.lo[0] < c.g[V0][0];
    c.g[V1][0] = short as u64;
    if !short {
        return Ok(PrebufferNext::Reset);
    }

    // Not enough yet: done anyway when the file has been read to its end.
    c.cycles += 5;
    c.g[A0][0] = c.lw(strm.wrapping_add(STREAM_FILE));
    call_ptr(c, GET_READ_STATE, 0x003b_7198)?;
    c.g[V1][0] = READ_AT_END;
    if c.g[V0][0] == READ_AT_END {
        return Ok(if back_edge_unwinds(c, RESET) {
            PrebufferNext::Unwind
        } else {
            PrebufferNext::Reset
        });
    }
    // Or when requests are queued (> 0) still prebuffering; else done when
    // the file is idle (read state 0).
    c.cycles += 3;
    let queued = c.lw(strm.wrapping_add(QUEUED));
    c.g[V0][0] = 1;
    if (queued as i64) > 0 {
        return Ok(PrebufferNext::Done);
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(strm.wrapping_add(STREAM_FILE));
    call_ptr(c, GET_READ_STATE, 0x003b_71b8)?;
    let idle = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if idle {
        return Ok(if back_edge_unwinds(c, RESET) {
            PrebufferNext::Unwind
        } else {
            PrebufferNext::Reset
        });
    }
    Ok(PrebufferNext::Done)
}

/// `int SNDSTRM_IsPrebuffering(SNDSTRM* s)` (0x3b70f8): whether a sound
/// stream is still prebuffering. No prebuffer state, a zero data rate or a
/// zero target: no (0); a negative target: yes (1). Else it compares the
/// buffered time (`min(bytes, 4e6) * 1000 / rate` ms) with the target:
/// reached, or the file read to its end, or (with no queued requests) the
/// file idle, clears the target and answers 0; otherwise 1. Resumable at
/// 0x3b7184 (the event check on the way to clearing the target). Frame 0x30
/// (s0, s1).
pub fn sndstrm_is_prebuffering(c: &mut Cpu) {
    use prebuffering::*;
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let next = match c.pc {
            ENTRY => sndstrm_prebuffer_checks(c)?,
            RESET => PrebufferNext::Reset,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        match next {
            PrebufferNext::Unwind => return Ok(()),
            PrebufferNext::Reset => {
                c.cycles += 3;
                let pre = c.g[S0][0] as u32;
                c.sw(pre.wrapping_add(TARGET_MS), 0);
                c.g[V0][0] = 0;
            }
            PrebufferNext::Done => {}
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
// SND_RestoreVu0Context (0x3c08f8)

/// `SND_RestoreVu0Flags(int status)` (0x3c07a8).
pub(crate) const SND_RESTORE_VU0_FLAGS: Callee = Callee {
    addr: 0x003c_07a8,
    f: ee::s13::SND_RestoreVu0Flags,
};

/// `void SND_RestoreVu0Context(Vu0Context* ctx)` (0x3c08f8): the inverse of
/// `SND_SaveVu0Context`: reload ACC from the saved slot (`+0x1f0`, through
/// `ACC = vf1 + vf0.x`), vf1-vf6, the MAC/status flags (`+0x200`, rebuilt by
/// `SND_RestoreVu0Flags`), vf7-vf31 and the status register (`+0x204` into
/// vi16).
pub fn snd_restore_vu0_context(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[T0][0] = c.g[S0][0];
        let ctx = r32(c, S0);
        c.lqc2(1, ctx.wrapping_add(0x1f0));
        c.vu0().adda(0b1111, 1, Src::Bc(0, X));
        for vf in 1..=6u32 {
            c.lqc2(vf as usize, ctx.wrapping_add((vf - 1) * 0x10));
        }
        c.g[A0][0] = c.lw(ctx.wrapping_add(0x200));
        SND_RESTORE_VU0_FLAGS.call(c, 0x003c_0934)?;
        c.g[T0][0] = c.g[S0][0];
        for vf in 7..=12u32 {
            c.lqc2(vf as usize, ctx.wrapping_add((vf - 1) * 0x10));
        }
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x204));
        c.lqc2(13, ctx.wrapping_add(0xc0));
        c.lqc2(14, ctx.wrapping_add(0xd0));
        c.ctc2(T1, 16);
        for vf in 15..=31u32 {
            c.lqc2(vf as usize, ctx.wrapping_add((vf - 1) * 0x10));
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SND_ParseNextTag(TagStream* s)` (0x3c7388): skip 0xfc padding, read
/// the next tag byte; 0xff ends the stream (returns 0); 0xfd and 0xfe are
/// bare markers; otherwise read the length byte (0xff: a 4-byte big-endian
/// length follows), note the payload, decode payloads of up to 4 bytes as a
/// big-endian value, and skip past the payload (returns 1).
pub fn snd_parse_next_tag(c: &mut Cpu) {
    use snd_tags::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xfc;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = r32(c, S0);
        c.g[A1][0] = c.lw(s + P);
        c.g[V0][0] = c.lbu(r32(c, A1));
        if c.g[V0][0] == c.g[V1][0] {
            // Skip the 0xfc padding.
            c.cycles += 1;
            c.g[A2][0] = 0xfc;
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[A1][0];
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(1));
                c.sw(s + P, c.g[V1][0]);
                c.g[V0][0] = c.lbu(r32(c, A0).wrapping_add(1));
                let more = c.g[V0][0] == c.g[A2][0];
                c.g[A1][0] = c.g[V1][0];
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_73b0);
                }
            }
        }
        c.g[A0][0] = c.lw(s + P);

        c.cycles += 5;
        c.g[A2][0] = 0xff;
        let p = r32(c, A0);
        c.g[A1][0] = c.lbu(p);
        c.sw(s + TAG, c.g[A1][0]);
        c.g[V0][0] = 0;
        'done: {
            if c.g[A1][0] == c.g[A2][0] {
                break 'done; // end of stream
            }
            c.cycles += 5;
            c.g[V0][0] = sx(p.wrapping_add(1));
            c.g[V1][0] = 0xfd;
            c.sw(s + P, c.g[V0][0]);
            let tag = c.g[A1][0];
            c.g[V0][0] = 1;
            if tag == 0xfd {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 0xfe;
            if tag == 0xfe {
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }
            // The length byte (0xff: a 4-byte length follows).
            c.g[V0][0] = c.lbu(p.wrapping_add(1));
            c.cycles += 2;
            let long = c.g[V0][0] == c.g[A2][0];
            c.sw(s + LEN, c.g[V0][0]);
            if long {
                c.cycles += 7;
                c.g[A0][0] = sx(p.wrapping_add(2));
                c.g[A1][0] = 4;
                READ_BIG_ENDIAN.call(c, 0x003c_7420)?;
                c.g[V1][0] = c.lw(s + P);
                c.sw(s + LEN, c.g[V0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                c.sw(s + P, c.g[V1][0]);
            }
            // The payload.
            c.cycles += 7;
            c.g[V0][0] = c.lw(s + P);
            c.g[A1][0] = c.lw(s + LEN);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(s + P, c.g[A0][0]);
            c.g[V0][0] = (c.g[A1][0] < 5) as u64;
            let short = c.g[V0][0] != 0;
            c.sw(s + PAYLOAD, c.g[A0][0]);
            if short {
                c.cycles += 3;
                READ_BIG_ENDIAN.call(c, 0x003c_7454)?;
                c.sw(s + VALUE, c.g[V0][0]);
            }
            c.cycles += 5;
            c.g[V1][0] = c.lw(s + P);
            c.g[V0][0] = 1;
            c.g[A0][0] = c.lw(s + LEN);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.sw(s + P, c.g[V1][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SNDSTRM_FindRequest (0x3b7a70)

pub(crate) const SNDSTRM_GET_STREAM_3: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

/// Stream: head of its request list.
pub(crate) const STREAM_REQUESTS: u32 = 0x11c;

/// Request: next request in the list (+0) and its handle (+0xc).
pub(crate) const REQUEST_NEXT: u32 = 0x0;

pub(crate) const REQUEST_HANDLE: u32 = 0xc;

/// `SndStrmRequest* SNDSTRM_FindRequest(int handle)` (0x3b7a70): the request
/// with this handle in the request list of its stream (`handle & 0xff`,
/// `SNDSTRM_GetStream`), or 0 for a negative handle, a missing stream or an
/// unknown request. The handle stays in `s0` (64-bit compare with the
/// sign-extended handle words).
pub fn sndstrm_find_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let handle = c.g[S0][0];
        'done: {
            'none: {
                if (handle as i64) < 0 {
                    break 'none;
                }
                c.cycles += 4;
                c.g[A0][0] = handle & 0xff;
                SNDSTRM_GET_STREAM_3.call(c, 0x003b_7a8c)?;
                let stream = r32(c, V0);
                if stream == 0 {
                    break 'none;
                }
                c.g[V1][0] = Addr(stream).off(STREAM_REQUESTS).lw(c);
                loop {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    if c.g[V1][0] == 0 {
                        break 'done;
                    }
                    c.cycles += 3;
                    let req = r32(c, V1);
                    c.g[V0][0] = Addr(req).off(REQUEST_HANDLE).lw(c);
                    if c.g[V0][0] == c.g[S0][0] {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[V1][0];
                        break 'done;
                    }
                    c.g[V1][0] = Addr(req).off(REQUEST_NEXT).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_7aa0);
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_MixAddRamp16 (0x3c7828)

/// `1/17` (0x3d70f0f1): the ramp step over a 16-sample block.
pub(crate) const RAMP16_STEP_SCALE: FpuF32 = FpuF32(0x3d70_f0f1);

/// `void SND_MixAddRamp16(const float* src, float* dst, float g0, float g1)`
/// (0x3c7828): mixes 16 samples into `dst` with a linear gain ramp,
/// `dst[i] += src[i] * (g0 + step * (i + 1))`, `step = (g1 - g0) / 17`
/// (gain 2 as `step + step`, gains 3..16 as `step * k`).
///
/// The original keeps the gains and partly mixed samples in f1..f25 (f20..f25
/// saved on a 0x30 stack frame) and interleaves the loads and stores; the
/// memory order is kept (`dst[0..12]` are read before the first stores, each
/// `src[k]` after `dst[k-1]` is written), and so are the registers it leaves.
pub fn snd_mix_add_ramp16(c: &mut Cpu) {
    c.cycles += 159;
    let src = Addr(abi::arg(c, 0));
    let dst = Addr(abi::arg(c, 1));
    let g0 = FpuF32(c.f[12]);
    let step = (FpuF32(c.f[13]) - g0) * RAMP16_STEP_SCALE;
    let k = |n: u32| FpuF32((n as f32).to_bits());
    let mut gain = [FpuF32::ZERO; 17];
    gain[1] = g0 + step;
    gain[2] = g0 + (step + step);
    for n in 3..=16 {
        gain[n as usize] = g0 + step * k(n);
    }
    // dst[i] = d + src[i] * gain; returns (new dst[i], the product).
    let mix = |c: &mut Cpu, d: FpuF32, i: u32| -> (FpuF32, FpuF32) {
        let product = src.off(4 * i).read_fpu(c) * gain[i as usize + 1];
        let out = d + product;
        dst.off(4 * i).write_fpu(c, out);
        (out, product)
    };

    let s0 = src.read_fpu(c);
    let d0 = dst.read_fpu(c);
    // Save f20..f25 (restored below).
    let frame = Frame::open(c, 0x30);
    for (i, f) in (20..=25).enumerate() {
        frame.save_fpr(c, f, 8 * i as u32);
    }
    let mut d = [FpuF32::ZERO; 16];
    d[0] = d0;
    for i in 1..4 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    let out0 = d0 + s0 * gain[1];
    dst.write_fpu(c, out0);
    for i in 4..8 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    let (out1, _) = mix(c, d[1], 1);
    for i in 8..12 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    let mut out = [FpuF32::ZERO; 16];
    out[0] = out0;
    out[1] = out1;
    for i in 2..12 {
        out[i] = mix(c, d[i], i as u32).0;
    }
    for (i, f) in (20..=25).enumerate() {
        frame.restore_fpr(c, f, 8 * i as u32);
    }
    // The last four: src[12] first, then dst[12..16].
    let s12 = src.off(0x30).read_fpu(c);
    for i in 12..16 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    out[12] = d[12] + s12 * gain[13];
    dst.off(0x30).write_fpu(c, out[12]);
    let mut last_product = FpuF32::ZERO;
    for i in 13..16 {
        (out[i], last_product) = mix(c, d[i], i as u32);
    }

    // Registers as the original leaves them.
    c.g[AT][0] = 0x4180_0000;
    let set = |c: &mut Cpu, f: usize, v: FpuF32| c.f[f] = v.0;
    set(c, 0, last_product);
    set(c, 1, out[12]);
    set(c, 2, out[13]);
    set(c, 3, out[14]);
    set(c, 4, out[15]);
    set(c, 5, gain[4]);
    set(c, 6, gain[11]);
    set(c, 7, gain[12]);
    set(c, 8, gain[14]);
    set(c, 9, gain[5]);
    set(c, 10, gain[6]);
    set(c, 11, gain[7]);
    set(c, 12, gain[16]);
    set(c, 13, step * k(16));
    set(c, 14, gain[8]);
    set(c, 15, gain[15]);
    set(c, 16, out[2]);
    set(c, 17, out[11]);
    set(c, 18, out[8]);
    set(c, 19, out[9]);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// SND_EeMix (0x3c8968)

/// The EE software mixer state (`D_00515B40`).
pub(crate) const MIXER: u32 = 0x0051_5b40;

/// Number of output buses (one output pointer each).
pub(crate) const MIXER_BUS_COUNT: u32 = 0x5;

/// Bytes each output pointer advances per mixed block.
pub(crate) const MIXER_OUT_STEP: u32 = 0xc;

/// Samples per mixed block.
pub(crate) const MIX_BLOCK: u32 = 0x200;

pub(crate) const SND_EE_MIX_BLOCK: Callee = Callee {
    addr: 0x003c_85d0,
    f: ee::s14::SND_EeMixBlock,
};

/// `void SND_EeMix(int** outs, int len)` (0x3c8968): mix `len` samples in
/// blocks of 0x200. Copies the bus output pointers into a local array, then
/// per block calls `SND_EeMixBlock(array, min(len, 0x200))` and advances
/// every output pointer by the mixer's step (+0xc).
pub fn snd_ee_mix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        set_reg(c, V0, 0x51_0000);
        frame.save(c, S0, 0x60);
        let buses = Addr(MIXER + MIXER_BUS_COUNT).lbu(c);
        set_reg(c, V0, buses);
        set_reg(c, S0, reg(c, A1));
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);

        // outs[0..buses] -> the local array at sp.
        if buses != 0 {
            c.cycles += 2;
            set_reg(c, A2, buses);
            set_reg(c, V1, reg(c, SP));
            loop {
                c.cycles += 7;
                let (src, dst) = (r32(c, A0), r32(c, V1));
                let p = Addr(src).lw(c);
                set_reg(c, V0, p);
                set_reg(c, A2, addu(reg(c, A2), u32::MAX as u64));
                set_reg(c, A0, sx(src.wrapping_add(4)));
                Addr(dst).sw(c, p);
                let more = reg(c, A2) != 0;
                set_reg(c, V1, sx(dst.wrapping_add(4)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8998);
                }
            }
        }

        // Blocks of up to 0x200 samples.
        c.cycles += 2;
        set_reg(c, V0, 0x51_0000);
        if reg(c, S0) as i64 > 0 {
            c.cycles += 5;
            set_reg(c, S3, MIX_BLOCK as u64);
            set_reg(c, S2, sx(MIXER));
            set_reg(c, S1, sx(MIXER));
            set_reg(c, V0, ((reg(c, S0) as i64) < 513) as u64);
            loop {
                c.cycles += 7;
                let n = if reg(c, V0) != 0 {
                    reg(c, S0)
                } else {
                    reg(c, S3)
                };
                set_reg(c, A1, n);
                set_reg(c, A0, reg(c, SP));
                SND_EE_MIX_BLOCK.call(c, 0x003c_89e0)?;
                let buses = Addr(r32(c, S2).wrapping_add(MIXER_BUS_COUNT)).lbu(c);
                set_reg(c, V0, buses);
                set_reg(c, A1, sx(r32(c, S0).wrapping_sub(MIX_BLOCK)));
                if buses != 0 {
                    // Advance every output pointer.
                    c.cycles += 3;
                    let mixer = r32(c, S1);
                    let step = Addr(mixer.wrapping_add(MIXER_OUT_STEP)).lhu(c);
                    set_reg(c, A0, step);
                    set_reg(c, V1, reg(c, SP));
                    let count = Addr(mixer.wrapping_add(MIXER_BUS_COUNT)).lbu(c);
                    set_reg(c, A2, count);
                    loop {
                        c.cycles += 7;
                        let at = r32(c, V1);
                        let p = Addr(at).lw(c);
                        set_reg(c, A2, addu(reg(c, A2), u32::MAX as u64));
                        let p = addu(p, reg(c, A0));
                        set_reg(c, V0, p);
                        Addr(at).sw(c, p);
                        let more = reg(c, A2) != 0;
                        set_reg(c, V1, sx(at.wrapping_add(4)));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_89f8);
                        }
                    }
                }
                c.cycles += 3;
                set_reg(c, S0, reg(c, A1));
                let more = reg(c, S0) as i64 > 0;
                set_reg(c, V0, ((reg(c, S0) as i64) < 513) as u64);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_89d0);
                }
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void SNDSTRM_RunCallbacks(void)` (0x3c7080): deliver the queued stream
/// callback events: kind 0 calls the stream's `stream_cb(id, value, user)`,
/// any other kind its `value_cb(value, user)`; then empty the queue.
pub fn sndstrm_run_callbacks(c: &mut Cpu) {
    use sndstrm_cb::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = 0x51_0000;
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = 0;
        c.g[V0][0] = Addr(COUNT).lw(c);
        c.g[S4][0] = c.g[S2][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(EVENTS);
            c.g[S3][0] = sx(STREAMS);
            c.g[S0][0] = c.g[V0][0];
            loop {
                c.cycles += 6;
                let ev = r32(c, S0);
                c.g[A0][0] = Addr(ev + EV_STREAM).lhu(c);
                c.g[V1][0] = Addr(ev + EV_KIND).lhu(c);
                c.g[V0][0] = sx((r32(c, A0) << 2).wrapping_add(r32(c, S3)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let stream = r32(c, V0);
                if c.g[V1][0] == 0 {
                    c.cycles += 6;
                    c.g[A2][0] = Addr(stream + USER).lw(c);
                    c.g[V0][0] = Addr(stream + STREAM_CB).lw(c);
                    c.g[A1][0] = Addr(ev + EV_VALUE).lw(c);
                    call_ptr(c, r32(c, V0), 0x003c_70e8)?;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = Addr(stream + USER).lw(c);
                    c.g[V0][0] = Addr(stream + VALUE_CB).lw(c);
                    c.g[A0][0] = Addr(ev + EV_VALUE).lw(c);
                    call_ptr(c, r32(c, V0), 0x003c_7100)?;
                }
                c.g[V0][0] = Addr(COUNT).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(EVENT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_70c0);
                }
            }
        }
        c.cycles += 9;
        Addr(COUNT).sw(c, 0);
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

// ---------------------------------------------------------------------------
// SND_RestoreVu0Flags (0x3c07a8)

/// `void SND_RestoreVu0Flags(int status)` (0x3c07a8): rebuild the VU0 MAC /
/// status flags saved by the sound mixer by multiplying two vectors built
/// from `status` (one lane per bit, shifted each lane): `a` is 0 / 1.0
/// (no overflow/underflow bit 0x8800) or 1e-30 / 1e30 (bit 8 selects the
/// small value), `b` is `a` negated where bit 0x80 is set. VF1 and VF2 are
/// kept across the `vmul` (`qmfc2`/`qmtc2` through t0/t1).
pub fn snd_restore_vu0_flags(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0; // lane
    c.g[A3][0] = sx(sp(c).wrapping_add(0x10)); // b
    c.g[A1][0] = c.g[SP][0]; // &a[lane]
    loop {
        c.cycles += 3;
        let saturated = c.g[A0][0] & 0x8800 != 0;
        c.g[V0][0] = c.g[A0][0] & 0x8;
        let small = c.g[V0][0] != 0;
        // `tail`: the 1-cycle block that tests the sign bit.
        let tail = if !saturated {
            c.cycles += 3;
            c.mtc1(0, 0); // 0.0
            c.g[V1][0] = sx(r32(c, A2) << 2);
            if small {
                true
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000; // 1.0
                c.mtc1(AT, 0);
                c.g[V0][0] = c.g[A0][0] & 0x80;
                false
            }
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x0da2_4260; // 1e-30
            c.mtc1(AT, 0);
            c.g[V1][0] = sx(r32(c, A2) << 2);
            if !small {
                c.cycles += 3;
                c.g[AT][0] = 0x7149_f2ca; // 1e30
                c.mtc1(AT, 0);
            }
            true
        };
        if tail {
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0] & 0x80;
        }
        c.cycles += 2;
        c.swc1(0, r32(c, A1));
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.neg_s(0, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        c.swc1(0, r32(c, V0));
        c.g[A0][0] = sx(r32(c, A0) << 1);
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_07b8);
        }
    }
    c.cycles += 10;
    c.qmfc2(T0, 1);
    c.qmfc2(T1, 2);
    c.lqc2(1, sp(c));
    c.lqc2(2, sp(c).wrapping_add(0x10));
    c.vu0().mul(0b1111, 1, 1, Src::Vec(2));
    c.qmtc2(T0, 1);
    c.qmtc2(T1, 2);
    c.vu0().vnop();
    frame.close(c);
}

pub(crate) mod ee_mix {

    pub const STATE: u32 = 0x0051_5b40;

    pub const COUNT: u32 = 0x5;

    pub const DRY_ON: u32 = 0xf;

    pub const FORMAT: u32 = 0xb0;

    pub const BLOCK: u32 = 0xb4;

    pub const SENDS: u32 = 0xd0;

    pub const SET: u32 = 0x1e0;

    /// The dry mix buffer.
    #[cfg(test)]
    pub const DRY_BUFFER: u32 = 0x0051_5d80;

    /// `SND_MixAddRamp16`.
    pub const MIX_ADD_RAMP16: u32 = 0x003c_7828;
}

// ---------------------------------------------------------------------------
// SND_RunChangeCallbacks (0x3b54e0, seg_2B4578.c)

/// The sound library's globals (0x50a8e8): change-callback list (+0x1b4)
/// and the change counter (+0x1c0); the counter value last seen (0x44c468).
pub(crate) const SND_GLOBALS: u32 = 0x0051_0000 - 0x5718;

pub(crate) const SND_CB_LIST: u32 = 0x1b4;

pub(crate) const SND_CHANGE_COUNT: u32 = 0x1c0;

pub(crate) const SND_SEEN_HI: u32 = 0x0045_0000;

pub(crate) const SND_SEEN_LO: u32 = 0x3b98;

/// `void SND_RunChangeCallbacks()` (0x3b54e0): when the sound library's
/// change counter moved since the last call, remember it and call every
/// registered callback (`node->fn(node->arg)`, `{next, ?, fn, arg}`).
pub fn snd_run_change_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = SND_SEEN_HI as u64;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(SND_GLOBALS);
        let seen = Addr(SND_SEEN_HI - SND_SEEN_LO);
        c.g[V1][0] = seen.lw(c);
        c.g[A0][0] = Addr(SND_GLOBALS + SND_CHANGE_COUNT).lw(c);
        let unchanged = c.g[A0][0] == c.g[V1][0];
        frame.restore(c, S0, 0x10);
        if !unchanged {
            c.cycles += 3;
            c.g[S0][0] = Addr(SND_GLOBALS + SND_CB_LIST).lw(c);
            let count = c.g[A0][0];
            seen.sw(c, count);
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                loop {
                    c.cycles += 5;
                    let f = r32(c, V0);
                    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    call_ptr(c, f, 0x003b_5520)?;
                    c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_5518);
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_GetVolume (0x3b7f18, seg_2B4578.c)

/// Stream fields: the sound handle, the volume it was started with.
pub(crate) const STRM_HANDLE: u32 = 0x4;

pub(crate) const STRM_VOLUME: u32 = 0xec;

/// SND error code: invalid handle.
pub(crate) const SND_ERR_HANDLE: u64 = -8i64 as u64;

pub(crate) const SNDSTRM_GET_STREAM_4: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

pub(crate) const SND_GET_VOLUME: Callee = Callee {
    addr: 0x003b_86c0,
    f: ee::s13::SND_GetVolume,
};

/// `int SNDSTRM_GetVolume(int stream)` (0x3b7f18): the volume of the
/// stream's playing sound, or the volume it was started with (signed byte
/// at +0xec) when the sound is no longer playing; -8 for an unknown
/// stream.
pub fn sndstrm_get_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        SNDSTRM_GET_STREAM_4.call(c, 0x003b_7f2c)?;
        c.g[S0][0] = c.g[V0][0];
        let stream = r32(c, S0);
        let none = c.g[S0][0] == 0;
        c.g[V0][0] = SND_ERR_HANDLE;
        if !none {
            c.cycles += 5;
            c.g[A0][0] = Addr(stream.wrapping_add(STRM_HANDLE)).lw(c);
            SND_GET_VOLUME.call(c, 0x003b_7f44)?;
            if (c.g[V0][0] as i64) < 0 {
                c.g[V0][0] = Addr(stream.wrapping_add(STRM_VOLUME)).lb(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------- SNDSTRM_ParseData

/// Stream decoder state (`self + 0x134`): byte-swap flag (+0x25, set for
/// big-endian data), "block pushed" flag (+0x24), the sample clock (+0x1c)
/// and the block tag (+0xc).
pub(crate) const SS_STATE: u32 = 0x134;

pub(crate) const SS_SWAP: u32 = 0x25;

pub(crate) const SS_PUSHED: u32 = 0x24;

pub(crate) const SS_CLOCK: u32 = 0x1c;

pub(crate) const SS_TAG: u32 = 0xc;

/// Stream: file handle (+0), stream index (+0x8), channel count (+0x16).
pub(crate) const SS_CHANNELS: u32 = 0x16;

/// Data chunk: sample count (+0x8), per-channel offsets from +0xc.
pub(crate) const SS_CHUNK_SAMPLES: u32 = 0x8;

pub(crate) const SS_CHUNK_OFFSETS: u32 = 0xc;

/// `SNDSTRM_PushBlock` (0x3c6ab8).
pub(crate) const SS_PUSH_BLOCK: Callee = Callee {
    addr: 0x003c_6ab8,
    f: ee::s13::f_003c6ab8,
};

/// `void SNDSTRM_ParseData(SndStream* self, Chunk* chunk)` (0x3b6ea8): turn
/// an `SCDl` data chunk into a block descriptor on the stack (a 64-bit word
/// at sp+0: low 32 bits kept, bits 32..62 the sample count, bit 63 "first
/// block pushed"; then a pointer per channel at sp+0xc..): the sample count
/// and each channel's offset (byte-swapped when the stream is big-endian)
/// relative to the end of the offset table. An empty chunk is released
/// back to the file (`StreamFile_ReleaseChunk`); otherwise the chunk is
/// remembered just before the first channel's data, its first word gets the
/// stream's tag, the clock advances by the sample count and the block goes
/// to `SNDSTRM_PushBlock(stream index, &block)`. Frame 0x60 (s0, ra at
/// +0x40).
pub fn sndstrm_parse_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[T4][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[T3][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        let this = r32(c, T4);
        let chunk = r32(c, T3);
        let state = Addr(this.wrapping_add(SS_STATE)).lw(c);
        c.g[S0][0] = state;
        let state = state as u32;
        let swap = Addr(state.wrapping_add(SS_SWAP)).lbu(c);
        c.g[V0][0] = swap;
        let s = sp(c);
        c.g[A0][0] = sx(s.wrapping_add(0x34));
        // The low 31 bits / the rest of the block descriptor word.
        const LOW31: u64 = 0x7fff_ffff;
        const KEEP: u64 = 0x8000_0000_ffff_ffff;
        if swap != 0 {
            // Byte-swap the sample count through sp+0x34 -> sp+0x30.
            c.cycles += 26;
            let n = Addr(chunk.wrapping_add(SS_CHUNK_SAMPLES)).lw(c);
            c.g[V0][0] = n;
            c.g[A1][0] = sx(s.wrapping_add(0x30));
            c.g[A3][0] = LOW31;
            Addr(s.wrapping_add(0x34)).sw(c, n);
            c.g[A2][0] = KEEP;
            let b3 = Addr(s.wrapping_add(0x37)).lbu(c);
            Addr(s.wrapping_add(0x30)).sb(c, b3);
            let b2 = Addr(s.wrapping_add(0x36)).lbu(c);
            c.g[V1][0] = b2;
            Addr(s.wrapping_add(0x31)).sb(c, b2);
            let b1 = Addr(s.wrapping_add(0x35)).lbu(c);
            c.g[V0][0] = b1;
            Addr(s.wrapping_add(0x32)).sb(c, b1);
            let b0 = Addr(s.wrapping_add(0x34)).lbu(c);
            c.g[V1][0] = b0;
            Addr(s.wrapping_add(0x33)).sb(c, b0);
            let count = Addr(s.wrapping_add(0x30)).lwu(c) & LOW31;
            let word = Addr(s).ld(c) & KEEP;
            c.g[V0][0] = count << 32;
            let word = word | count << 32;
            c.g[V1][0] = word;
            Addr(s).sd(c, word);
        } else {
            c.cycles += 14;
            let count = Addr(chunk.wrapping_add(SS_CHUNK_SAMPLES)).lwu(c) & LOW31;
            c.g[A1][0] = LOW31;
            c.g[A0][0] = KEEP;
            c.g[V1][0] = count << 32;
            let word = (Addr(s).ld(c) & KEEP) | count << 32;
            c.g[V0][0] = word;
            Addr(s).sd(c, word);
        }

        // Channel pointers: data start (after the offset table) + offset.
        c.cycles += 5;
        let channels = Addr(this.wrapping_add(SS_CHANNELS)).lbu(c);
        c.g[V1][0] = channels;
        let offsets = chunk.wrapping_add(SS_CHUNK_OFFSETS);
        c.g[A0][0] = sx(offsets);
        c.g[V0][0] = sx((channels as u32) << 2);
        let data = offsets.wrapping_add((channels as u32) << 2);
        c.g[T2][0] = sx(data);
        if channels != 0 {
            c.cycles += 6;
            c.g[A2][0] = sx(offsets);
            c.g[T1][0] = channels;
            c.g[T0][0] = sx(s.wrapping_add(0x30));
            c.g[A3][0] = sx(s.wrapping_add(0x38));
            c.g[A1][0] = sx(s.wrapping_add(0xc));
            loop {
                c.cycles += 3;
                let s = sp(c);
                let swap = Addr(r32(c, S0).wrapping_add(SS_SWAP)).lbu(c);
                let mut off = Addr(r32(c, A2)).lw(c);
                c.g[V0][0] = off;
                if swap != 0 {
                    // Byte-swap through sp+0x38 -> sp+0x30.
                    c.cycles += 10;
                    Addr(s.wrapping_add(0x38)).sw(c, off);
                    let b3 = Addr(s.wrapping_add(0x3b)).lbu(c);
                    Addr(s.wrapping_add(0x30)).sb(c, b3);
                    let b2 = Addr(s.wrapping_add(0x3a)).lbu(c);
                    c.g[V0][0] = b2;
                    Addr(s.wrapping_add(0x31)).sb(c, b2);
                    let b1 = Addr(s.wrapping_add(0x39)).lbu(c);
                    c.g[V1][0] = b1;
                    Addr(s.wrapping_add(0x32)).sb(c, b1);
                    let b0 = Addr(s.wrapping_add(0x38)).lbu(c);
                    c.g[A0][0] = b0;
                    Addr(s.wrapping_add(0x33)).sb(c, b0);
                    off = Addr(s.wrapping_add(0x30)).lw(c);
                }
                c.cycles += 6;
                let ptr = sx(r32(c, T2).wrapping_add(off as u32));
                c.g[V0][0] = ptr;
                Addr(r32(c, A1)).sw(c, ptr);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                if c.g[T1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_6f98);
                }
            }
        }

        c.cycles += 6;
        let s = sp(c);
        let count = Addr(s.wrapping_add(4)).lw(c) & LOW31;
        c.g[V0][0] = count;
        c.g[A2][0] = LOW31;
        let first = Addr(s.wrapping_add(0xc)).lw(c);
        c.g[A0][0] = first;
        if count == 0 {
            // StreamFile_ReleaseChunk(file, chunk)
            c.cycles += 5;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[A1][0] = c.g[T3][0];
            call_ptr(c, 0x003e_13e8, 0x003b_7008)?;
        } else {
            c.cycles += 24;
            c.g[A3][0] = u64::MAX >> 1;
            c.g[A1][0] = c.g[SP][0];
            // The chunk, just before the first channel's data.
            let chunk_reg = c.g[T3][0];
            Addr((first as u32).wrapping_sub(4)).sw(c, chunk_reg);
            let tag = Addr(state.wrapping_add(SS_TAG)).lw(c);
            c.g[V1][0] = tag;
            let count = Addr(s.wrapping_add(4)).lwu(c);
            Addr(chunk).sw(c, tag);
            let count = (((count << 32) as i64) >> 32) as u64 & LOW31;
            c.g[V0][0] = count;
            let clock = Addr(state.wrapping_add(SS_CLOCK)).lw(c);
            let pushed = Addr(state.wrapping_add(SS_PUSHED)).lbu(c);
            let clock = sx((clock as u32).wrapping_add(count as u32));
            c.g[V1][0] = clock;
            Addr(state.wrapping_add(SS_CLOCK)).sw(c, clock);
            c.g[A2][0] = pushed << 63;
            let word = (Addr(s).ld(c) & (u64::MAX >> 1)) | pushed << 63;
            c.g[V0][0] = word;
            c.g[A0][0] = Addr(this.wrapping_add(0x8)).lw(c);
            Addr(s).sd(c, word);
            SS_PUSH_BLOCK.call(c, 0x003b_7064)?;
            c.g[V1][0] = 1;
            Addr(r32(c, S0).wrapping_add(SS_PUSHED)).sb(c, 1);
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 3;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------- SND_RunServiceCallbacks

/// Sound driver globals (0x50a8e8): service callbacks enabled (s8, +0x174),
/// their count (s8, +0x178) and the function table (+0x19c).
pub(crate) const SND_GLOBALS_2: u32 = 0x0050_a8e8;

pub(crate) const SND_SERVICE_ON: u32 = 0x174;

pub(crate) const SND_SERVICE_COUNT: u32 = 0x178;

pub(crate) const SND_SERVICE_FNS: u32 = 0x19c;

/// `void SND_RunServiceCallbacks(void)` (0x3c60e0), from the sound service
/// tick: when enabled, call each registered service callback in order (the
/// count is re-read after every call). Frame 0x40 (s0 = index, s1 = table
/// cursor, s2 = globals); s0 is reloaded once right at the start (the
/// original's delay slot) and again at the end.
pub fn snd_run_service_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0051_0000;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(SND_GLOBALS_2);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let on = Addr(SND_GLOBALS_2 + SND_SERVICE_ON).lb(c);
        c.g[V1][0] = on;
        frame.restore(c, S0, 0x30);
        if on != 0 {
            c.cycles += 3;
            let count = Addr(SND_GLOBALS_2 + SND_SERVICE_COUNT).lb(c);
            c.g[V0][0] = count;
            c.g[S0][0] = 0;
            if (count as i64) > 0 {
                c.cycles += 3;
                c.g[S2][0] = c.g[A0][0];
                c.g[S1][0] = sx(SND_GLOBALS_2 + SND_SERVICE_FNS);
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                loop {
                    c.cycles += 7;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    let callback = r32(c, V1);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    call_ptr(c, callback, 0x003c_612c)?;
                    let count = Addr(r32(c, S2).wrapping_add(SND_SERVICE_COUNT)).lb(c);
                    let more = (c.g[S0][0] as i64) < (count as i64);
                    c.g[V0][0] = more as u64;
                    if !more {
                        break;
                    }
                    c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_6120);
                    }
                }
            }
            c.cycles += 1;
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

// ---------------------------------------------------------------------------
// 0x003ba078 SND_IsVoiceReserved

/// The reserved-voice table (`short[]`) `SND_IsVoiceReserved` scans.
pub(crate) const SND_RESERVED_VOICES: u32 = 0x0050_ac20;

/// `int SND_IsVoiceReserved(int voice, int count)` (0x3ba078): 1 if `voice`
/// is among the first `count` entries of the reserved-voice table, else 0.
pub fn snd_is_voice_reserved(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0;
    if (c.g[A1][0] as i64) > 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x0051_0000;
        c.g[A2][0] = sx(SND_RESERVED_VOICES);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A2)).lh(c);
            let differs = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !differs {
                c.cycles += 2;
                c.g[V0][0] = 1;
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a088);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x003c6c28 SNDSTRM_GetFreeBlocks

/// SNDSTRM's per-slot state pointers (`+0x304 + 4 * slot`).
pub(crate) const SNDSTRM_SLOT_BASE: u32 = 0x0051_57b8;

pub(crate) const SNDSTRM_SLOT_STATES: u32 = 0x304;

/// `int SNDSTRM_GetFreeBlocks(int slot)` (0x3c6c28): the slot's block count
/// (`+0x40`) minus one minus the highest block index in use by its channels
/// (`+0x62` channels, a signed halfword each at `+0x34`, at least 0).
pub fn sndstrm_get_free_blocks(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x0051_0000;
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[V0][0] = sx(SNDSTRM_SLOT_BASE);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    let state = Addr(r32(c, V0).wrapping_add(SNDSTRM_SLOT_STATES)).lw(c);
    c.g[A2][0] = state;
    let state = state as u32;
    c.g[V0][0] = Addr(state.wrapping_add(0x62)).lbu(c);
    c.g[A1][0] = 0; // the highest index so far
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = c.g[V0][0]; // channels left
        c.g[V1][0] = sx(state.wrapping_add(0x34));
        loop {
            c.cycles += 6;
            let index = Addr(r32(c, V1)).lhu(c) as u32 as i16 as i64;
            c.g[V0][0] = (index < c.g[A1][0] as i64) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V0][0] = sx((index as u32) << 16);
                c.g[A1][0] = index as u64;
            }
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            c.cycles += 2;
            let more = c.g[A0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_6c50);
            }
        }
    }
    c.cycles += 6;
    let blocks = Addr(state.wrapping_add(0x40)).lhu(c) as u32 as i16 as i32 as u32;
    c.g[V0][0] = sx(blocks.wrapping_sub(r32(c, A1)).wrapping_sub(1));
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x003c6ab8 SNDSTRM_PushBlock

/// `int SNDSTRM_PushBlock(int slot, Block* block)` (0x3c6ab8): queue a
/// stream block on the slot's ring. Fails with -13 when some channel's
/// position (`+0x34`, one signed halfword per channel) has reached the last
/// ring entry (`+0x40` entries). Otherwise writes the ring entry at the
/// write index (`+0x46`, 0x20 bytes each from `+0x7c`): the sequence number
/// (`+0x4`), the block's size word (low 31 bits) with its top bit kept as
/// the block's flag, and one word per channel from the block (`+0xc`),
/// advancing every channel's position; adds the size to the queued bytes
/// (`+0x48`), bumps the sequence number and the write index (wrapping at the
/// ring size). Returns the sequence number the block got.
pub fn sndstrm_push_block(c: &mut Cpu) {
    const ERR_FULL: u64 = 0xffff_ffff_ffff_fff3;
    const SIZE_MASK: u64 = 0x7fff_ffff;
    let lh16 = |c: &mut Cpu, a: u32| Addr(a).lhu(c) as u32 as i16 as i64 as u64;
    c.cycles += 9;
    c.g[V0][0] = 0x0051_0000;
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[V0][0] = sx(SNDSTRM_SLOT_BASE);
    c.g[T2][0] = c.g[A1][0]; // the block
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.g[A3][0] = Addr(r32(c, V0).wrapping_add(SNDSTRM_SLOT_STATES)).lw(c);
    let st = r32(c, A3);
    c.g[V0][0] = Addr(st.wrapping_add(0x62)).lbu(c);
    c.g[T0][0] = 0;
    'done: {
        // Room check: every channel's position below the last entry.
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[V0][0]; // channels
            c.g[A0][0] = sx(st.wrapping_add(0x34));
            loop {
                c.cycles += 10;
                let pos_at = r32(c, A0);
                c.g[V1][0] = lh16(c, pos_at);
                c.g[V0][0] = lh16(c, st.wrapping_add(0x40));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                if c.g[V1][0] == 0 {
                    c.cycles += 2;
                    c.g[A1][0] = ERR_FULL;
                    break 'done;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_6ae8);
                }
            }
        }
        // The ring entry at the write index.
        c.cycles += 26;
        c.g[V0][0] = Addr(st.wrapping_add(0x46)).lhu(c);
        c.g[A1][0] = SIZE_MASK;
        let block = r32(c, T2);
        c.g[A0][0] = Addr(block.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[A2][0] = 0xffff_ffff_8000_0000;
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 11) as u32);
        c.g[A0][0] &= c.g[A1][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7c));
        c.g[T0][0] = 0;
        let entry = st.wrapping_add(r32(c, V0));
        c.g[T1][0] = sx(entry);
        c.g[V1][0] = Addr(entry.wrapping_add(4)).lw(c);
        c.g[V1][0] &= c.g[A2][0];
        c.g[V1][0] |= c.g[A0][0];
        Addr(entry.wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V1][0] &= c.g[A1][0];
        c.g[V0][0] = Addr(block.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) >> 31);
        c.g[V0][0] = sx(r32(c, V0) << 31);
        c.g[V1][0] |= c.g[V0][0];
        Addr(entry.wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(st.wrapping_add(4)).lw(c);
        Addr(entry).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(st.wrapping_add(0x62)).lbu(c);
        c.g[A1][0] = sx(entry.wrapping_add(8));
        // One word per channel; each channel's position advances.
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V1][0] = sx(st.wrapping_add(0x34));
            c.g[A0][0] = sx(block.wrapping_add(0xc));
            loop {
                c.cycles += 12;
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                Addr(r32(c, V1)).sh(c, c.g[V0][0]);
                c.g[V0][0] = Addr(st.wrapping_add(0x62)).lbu(c);
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_6b98);
                }
            }
        }
        // Queued bytes, sequence number, write index.
        c.cycles += 21;
        c.g[A0][0] = Addr(block.wrapping_add(4)).lw(c);
        c.g[V0][0] = SIZE_MASK;
        c.g[V1][0] = Addr(st.wrapping_add(0x48)).lw(c);
        c.g[A0][0] &= c.g[V0][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        Addr(st.wrapping_add(0x48)).sw(c, c.g[V1][0]);
        c.g[A1][0] = Addr(st.wrapping_add(4)).lw(c);
        c.g[V0][0] = Addr(st.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(st.wrapping_add(0x46)).lhu(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        Addr(st.wrapping_add(0x46)).sh(c, c.g[V1][0]);
        c.g[A0][0] = Addr(st.wrapping_add(0x46)).lhu(c);
        c.g[V0][0] = Addr(st.wrapping_add(0x40)).lhu(c);
        c.g[A0][0] = sx(r32(c, A0) << 16);
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[A0][0] == 0 {
            Addr(st.wrapping_add(0x46)).sh(c, 0);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_SetVolume (0x3b7e70)

pub(crate) const SNDSTRM_GET_STREAM_5: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

pub(crate) const SND_SET_VOLUME: Callee = Callee {
    addr: 0x003b_85f0,
    f: ee::s13::SND_SetVolume,
};

/// A sound stream: its sound handle (+4) and volume byte (+0xec).
pub(crate) const STREAM_SND_HANDLE: u32 = 0x4;

pub(crate) const STREAM_VOLUME: u32 = 0xec;

/// `int SNDSTRM_SetVolume(int stream /*a0*/, int volume /*a1*/)`
/// (0x3b7e70): store the stream's volume byte and set the volume of its
/// sound handle (`SND_SetVolume`); 0, or -8 when the stream does not exist.
/// Frame 0x20 (ra at 0, s0 = volume at 0x10).
pub fn sndstrm_set_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        SNDSTRM_GET_STREAM_5.call(c, 0x003b_7e84)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(-8i32 as u32);
        } else {
            c.cycles += 5;
            let stream = r32(c, V0);
            c.g[A0][0] = c.lw(stream.wrapping_add(STREAM_SND_HANDLE));
            c.g[A1][0] = c.g[S0][0];
            c.sb(stream.wrapping_add(STREAM_VOLUME), c.g[S0][0]);
            SND_SET_VOLUME.call(c, 0x003b_7e9c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// One send of [`snd_ee_voice_mix_ramp`] (blocks 0x3c7b9c/0x3c7c28): with
/// the wanted level at `s0` and the current one at `s0 - back`, ramps from
/// the current to the wanted level over the block (`SND_MixAddRamp16`) and
/// stores it, or (no change) adds at the steady level through the state's
/// `set` callback unless it is 0. `dry` picks the buffer: the dry buffer
/// (`s4 + 0x5d80`) or the send buffer `*s3`. True when a call was made
/// (the generated body's join block with its extra cycle).
pub(crate) fn mix_ramp_send(
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

// ---------------------------------------------------------------------------
// SND_GetVolume (0x3b86c0)

/// `SND_HandleToChannel` (0x3ba6e8).
pub(crate) const SND_HANDLE_TO_CHANNEL_2: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};

/// The pointer to the sound channel parameter array (0x8c bytes per
/// channel) and the channel's volume (s16).
pub(crate) const SND_CHANNEL_PARAMS: u32 = 0x0050_aad8;

pub(crate) const SND_CHANNEL_PARAM_SIZE: u32 = 0x8c;

pub(crate) const SND_CHANNEL_VOLUME: u32 = 0x3a;

/// `short SND_GetVolume(int handle)` (0x3b86c0): the volume of the channel
/// playing `handle`, or -8 when the handle is not playing.
pub fn snd_get_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        SND_HANDLE_TO_CHANNEL_2.call(c, 0x003b_86d0)?;
        let channel = c.g[V0][0];
        c.g[V1][0] = SND_CHANNEL_PARAM_SIZE as u64;
        if (channel as i64) < 0 {
            c.cycles += 1;
            c.g[V0][0] = (-8i64) as u64;
        } else {
            c.cycles += 6;
            c.g[A0][0] = 0x51_0000;
            let off = abi::mult(c, channel as u32, SND_CHANNEL_PARAM_SIZE);
            let params = Addr(SND_CHANNEL_PARAMS).lw(c);
            let entry = (params as u32).wrapping_add(off);
            c.g[V1][0] = sx(entry);
            c.g[V0][0] = Addr(entry).off(SND_CHANNEL_VOLUME).lh(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_ChannelStartVoices (0x3c3ee0)

/// The sound driver state block (`0x510000 - 0x5718`).
pub(crate) const SV_STATE: u32 = 0x0050_a8e8;

/// The second global block (`0x510000 - 0x5300`): voice start tables.
pub(crate) const SV_SYNC: u32 = 0x0050_ad00;

/// State: `SndChannel*` channel array.
pub(crate) const SV_STATE_CHANNELS: u32 = 0x1f0;

/// State `u8`: first EE voice number (after `SV_STATE_VOICE_BASE`).
pub(crate) const SV_STATE_EE_VOICE_BASE: u32 = 0x2f;

/// State `u8`: first voice number of the IOP DSP block.
pub(crate) const SV_STATE_VOICE_BASE: u32 = 0x30;

/// State `u8`: SPU output mode; 2 also drives the second voice block (+0x18).
pub(crate) const SV_STATE_OUTPUT_MODE: u32 = 0x49;

/// Sync: key-on sequence counter (`+= 0x800000` per started voice).
pub(crate) const SV_SYNC_KEY_SEQ: u32 = 0x1ab8;

/// Sync: `u8*` voice parameter blocks (SPU 8 bytes, IOP 16 bytes per voice).
pub(crate) const SV_SYNC_VOICE_PARAMS: u32 = 0xcb0;

/// Sync: per-voice start addresses (32 bytes per voice).
pub(crate) const SV_SYNC_START_ADDRS: u32 = 0xeb0;

/// Sync: SPU start-address shadow (8 bytes per voice).
pub(crate) const SV_SYNC_SPU_SHADOW: u32 = 0x840;

/// Sync: IOP DSP voice shadow (8 bytes per voice).
pub(crate) const SV_SYNC_IOP_SHADOW: u32 = 0x9c0;

/// `SndChannel` size.
pub(crate) const SV_CHAN_SIZE: u32 = 0x8c;

/// Channel `s16[]`: voice numbers.
pub(crate) const SV_CHAN_VOICES: u32 = 0x04;

/// Channel word passed to `SND_EeVoiceStart` (8th argument).
pub(crate) const SV_CHAN_EE_PARAM: u32 = 0x14;

/// Channel `u8`: voice count.
pub(crate) const SV_CHAN_VOICE_COUNT: u32 = 0x23;

/// Channel `u16`: flags (8: SPU voices, 0x100: IOP DSP voices, else EE).
pub(crate) const SV_CHAN_FLAGS: u32 = 0x24;

/// Start descriptor `s8` passed to `SND_EeVoiceStart` (13th argument).
pub(crate) const SV_DESC_PRIORITY: u32 = 0x02;

/// Start descriptor `u8`: codec.
pub(crate) const SV_DESC_CODEC: u32 = 0x13;

/// Start descriptor words passed to `SND_EeVoiceStart` (9th/10th args).
pub(crate) const SV_DESC_ARG5C: u32 = 0x5c;

pub(crate) const SV_DESC_ARG60: u32 = 0x60;

/// Start descriptor: pointer to the sample base address.
pub(crate) const SV_DESC_BASE_PTR: u32 = 0x64;

/// Start descriptor `int[]`: per-voice sample offsets.
pub(crate) const SV_DESC_OFFSETS: u32 = 0x68;

/// Start descriptor `int[]`: per-voice loop offsets.
pub(crate) const SV_DESC_LOOPS: u32 = 0x80;

/// Start descriptor `int[]`: per-voice stretch parameters.
pub(crate) const SV_DESC_STRETCH: u32 = 0x98;

pub(crate) const SV_APPLY_PITCH: Callee = Callee {
    addr: 0x003c_25e0,
    f: ee::s13::SND_ChannelApplyPitch,
};

pub(crate) const SV_APPLY_PAN: Callee = Callee {
    addr: 0x003c_38e0,
    f: ee::s13::SND_ChannelApplyPan,
};

pub(crate) const SV_APPLY_FX_ENABLE: Callee = Callee {
    addr: 0x003c_2ec8,
    f: ee::s13::SND_ChannelApplyFxEnable,
};

pub(crate) const SV_APPLY_SEND: Callee = Callee {
    addr: 0x003c_27e0,
    f: ee::s13::SND_ChannelApplySend,
};

pub(crate) const SV_APPLY_TIME_STRETCH: Callee = Callee {
    addr: 0x003c_3010,
    f: ee::s13::SND_ChannelApplyTimeStretch,
};

pub(crate) const SV_APPLY_DISTORTION: Callee = Callee {
    addr: 0x003c_30c8,
    f: ee::s13::SND_ChannelApplyDistortion,
};

pub(crate) const SV_APPLY_LOWPASS: Callee = Callee {
    addr: 0x003c_3178,
    f: ee::s13::SND_ChannelApplyLowpass,
};

pub(crate) const SV_APPLY_HIGHPASS: Callee = Callee {
    addr: 0x003c_3250,
    f: ee::s13::SND_ChannelApplyHighpass,
};

/// `SND_EeVoiceStart` (cross-shard: through the function table).
pub(crate) const SV_EE_VOICE_START: u32 = 0x003c_8158;

/// `SND_EeVoiceActivate` (cross-shard: through the function table).
pub(crate) const SV_EE_VOICE_ACTIVATE: u32 = 0x003c_83a0;

/// `int SND_ChannelStartVoices(StartDesc* d, u8* base, int ch, int stretch,
/// int distortion, int lowpass, int highpass)` (0x3c3ee0): start the voices
/// of sound channel `ch` (`state.channels + ch * 0x8c`).
///
/// - SPU channels (flag 8): apply pitch, pan, effect enable and send, then
///   per voice write the start address (`*d->base_ptr + d->offsets[i] -
///   d->offsets[0]`), bump the key-on sequence and store the tagged address
///   into the voice parameters and the SPU shadow (and the second block in
///   output mode 2).
/// - IOP DSP channels (flag 0x100): pitch, pan, send, then the same start
///   address into the IOP parameter block (+0x1a0) and a cleared shadow.
/// - EE channels: `SND_EeVoiceStart` per voice, then pitch, pan, send,
///   time stretch, distortion, low/high pass on the first voice, then
///   `SND_EeVoiceActivate` per voice.
///
/// Returns 0.
pub fn snd_channel_start_voices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xe0);
        c.g[V1][0] = SV_CHAN_SIZE as u64;
        frame.save(c, S0, 0xd0);
        c.g[V0][0] = 0x51_0000;
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        let ch_id = r32(c, S0);
        let ch_off = abi::mult(c, ch_id, SV_CHAN_SIZE);
        c.g[V1][0] = sx(ch_off);
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x80);
        c.g[S4][0] = sx(SV_STATE);
        frame.save(c, S7, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, FP, 0x50);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S1, 0xc0);
        c.g[FP][0] = c.g[T0][0];
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x40);
        let chans = Addr(SV_STATE + SV_STATE_CHANNELS).lw(c);
        c.g[V0][0] = chans;
        let lowpass = c.g[T1][0];
        Addr(sp(c) + 0x30).sw(c, lowpass);
        let ch = (chans as u32).wrapping_add(ch_off);
        c.g[S2][0] = sx(ch);
        let flags = Addr(ch + SV_CHAN_FLAGS).lhu(c);
        c.g[V1][0] = flags;
        c.g[V0][0] = flags & 0x8;
        let highpass = c.g[T2][0];
        Addr(sp(c) + 0x34).sw(c, highpass);

        if flags & 0x8 != 0 {
            // SPU voices.
            c.cycles += 13;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = 0;
            SV_APPLY_PITCH.call(c, 0x003c_3f58)?;
            c.g[A0][0] = c.g[S0][0];
            SV_APPLY_PAN.call(c, 0x003c_3f60)?;
            c.g[A0][0] = c.g[S0][0];
            SV_APPLY_FX_ENABLE.call(c, 0x003c_3f68)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            SV_APPLY_SEND.call(c, 0x003c_3f74)?;
            let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
            c.g[V1][0] = n;
            c.g[V0][0] = 0x51_0000;
            if n == 0 {
                sv_epilogue(c, frame, 1);
                return Ok(());
            }
            c.cycles += 8;
            c.g[T6][0] = c.g[S4][0];
            c.g[T0][0] = sx(SV_SYNC);
            c.g[T5][0] = 0x80_0000;
            c.g[T1][0] = sx(SV_SYNC + SV_SYNC_START_ADDRS);
            c.g[T3][0] = sx(SV_SYNC + SV_SYNC_SPU_SHADOW);
            c.g[T4][0] = 0x2;
            c.g[T2][0] = sx(r32(c, S3).wrapping_add(SV_DESC_OFFSETS));
            c.g[A3][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            loop {
                c.cycles += 31;
                let desc = r32(c, S3);
                let sync = r32(c, T0);
                let starts = r32(c, T1);
                let shadow = r32(c, T3);
                let voice_at = Addr(r32(c, A3));
                // Start address: *base_ptr + (offsets[i] - offsets[0]).
                let base_ptr = Addr(desc + SV_DESC_BASE_PTR).lw(c);
                let off0 = Addr(desc + SV_DESC_OFFSETS).lw(c);
                let voice = voice_at.lh(c) as u32;
                let off_i = Addr(r32(c, T2)).lw(c);
                let base = Addr(base_ptr as u32).lw(c);
                let start = (base as u32).wrapping_add((off_i as u32).wrapping_sub(off0 as u32));
                Addr((voice << 5).wrapping_add(starts)).sw(c, sx(start));
                // Next key-on sequence tag.
                let seq = (Addr(sync + SV_SYNC_KEY_SEQ).lw(c) as u32).wrapping_add(r32(c, T5));
                let params = Addr(sync + SV_SYNC_VOICE_PARAMS).lw(c) as u32;
                Addr(sync + SV_SYNC_KEY_SEQ).sw(c, sx(seq));
                let voice = voice_at.lh(c) as u32;
                let addr = Addr((voice << 5).wrapping_add(starts)).lw(c) as u32;
                let param_at = params.wrapping_add(voice << 3);
                Addr(param_at + 0x20).sw(c, sx(addr | seq));
                let voice = voice_at.lh(c) as u32;
                let mode = Addr(r32(c, T6) + SV_STATE_OUTPUT_MODE).lbu(c);
                let e_at = (voice << 5).wrapping_add(starts);
                let addr = Addr(e_at).lw(c);
                let shadow_at = (voice << 3).wrapping_add(shadow);
                Addr(shadow_at).sw(c, addr);
                if mode != c.g[T4][0] {
                    c.g[A0][0] = sx(e_at);
                    c.g[A1][0] = mode;
                    c.g[A2][0] = sx(param_at);
                    c.g[V1][0] = sx(shadow_at);
                } else {
                    // Output mode 2: the second voice block (+0x18) too.
                    c.cycles += 19;
                    let voice = voice_at.lh(c) as u32;
                    let params = Addr(sync + SV_SYNC_VOICE_PARAMS).lw(c) as u32;
                    let seq = Addr(sync + SV_SYNC_KEY_SEQ).lw(c);
                    let addr = Addr((voice << 5).wrapping_add(starts)).lw(c);
                    let param2_at = params.wrapping_add(voice.wrapping_add(0x18) << 3);
                    Addr(param2_at + 0x20).sw(c, addr | seq);
                    let voice = voice_at.lh(c) as u32;
                    let e_at = (voice << 5).wrapping_add(starts);
                    let addr = Addr(e_at).lw(c);
                    let shadow2_at = (voice.wrapping_add(0x18) << 3).wrapping_add(shadow);
                    Addr(shadow2_at).sw(c, addr);
                    c.g[A0][0] = addr;
                    c.g[A1][0] = sx(param2_at);
                    c.g[A2][0] = seq;
                    c.g[V1][0] = sx(e_at);
                    c.g[V0][0] = sx(shadow2_at);
                }
                c.cycles += 6;
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(4));
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_3fa0);
                }
            }
            sv_epilogue(c, frame, 2);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0x100;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] != 0 {
            // IOP DSP voices.
            c.cycles += 10;
            c.g[S1][0] = 0;
            SV_APPLY_PITCH.call(c, 0x003c_409c)?;
            c.g[A0][0] = c.g[S0][0];
            SV_APPLY_PAN.call(c, 0x003c_40a4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            SV_APPLY_SEND.call(c, 0x003c_40b0)?;
            let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
            c.g[V1][0] = n;
            c.g[V0][0] = 0x51_0000;
            if n == 0 {
                sv_epilogue(c, frame, 1);
                return Ok(());
            }
            c.cycles += 7;
            c.g[T4][0] = c.g[S4][0];
            c.g[T2][0] = sx(SV_SYNC);
            c.g[T6][0] = 0x80_0000;
            c.g[T3][0] = sx(SV_SYNC + SV_SYNC_START_ADDRS);
            c.g[T5][0] = sx(SV_SYNC + SV_SYNC_IOP_SHADOW);
            c.g[T1][0] = sx(r32(c, S3).wrapping_add(SV_DESC_OFFSETS));
            c.g[T0][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            loop {
                c.cycles += 36;
                let desc = r32(c, S3);
                let sync = r32(c, T2);
                let starts = r32(c, T3);
                let state = r32(c, T4);
                let voice_at = Addr(r32(c, T0));
                let base_ptr = Addr(desc + SV_DESC_BASE_PTR).lw(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let voice = voice_at.lh(c) as u32;
                let off_i = Addr(r32(c, T1)).lw(c);
                let off0 = Addr(desc + SV_DESC_OFFSETS).lw(c);
                let base = Addr(base_ptr as u32).lw(c);
                let first = Addr(state + SV_STATE_VOICE_BASE).lbu(c);
                let start = (base as u32).wrapping_add((off_i as u32).wrapping_sub(off0 as u32));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(4));
                Addr((voice << 5).wrapping_add(starts)).sw(c, sx(start));
                let seq = (Addr(sync + SV_SYNC_KEY_SEQ).lw(c) as u32).wrapping_add(r32(c, T6));
                let params = Addr(sync + SV_SYNC_VOICE_PARAMS).lw(c) as u32;
                Addr(sync + SV_SYNC_KEY_SEQ).sw(c, sx(seq));
                let voice = voice_at.lh(c) as u32;
                let e_at = (voice << 5).wrapping_add(starts);
                let addr = Addr(e_at).lw(c) as u32;
                let param_at = params.wrapping_add(voice.wrapping_sub(first as u32) << 4);
                Addr(param_at + 0x1a0).sw(c, sx(addr | seq));
                let voice = voice_at.lh(c) as u32;
                let first2 = Addr(state + SV_STATE_VOICE_BASE).lbu(c) as u32;
                let shadow_at = (voice.wrapping_sub(first2) << 3).wrapping_add(r32(c, T5));
                Addr(shadow_at).sw(c, 0);
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[A0][0] = sx(e_at);
                c.g[A1][0] = sx(seq);
                c.g[A2][0] = sx(param_at);
                c.g[A3][0] = first;
                c.g[V1][0] = sx(shadow_at);
                c.g[V0][0] = more as u64;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_40d8);
                }
            }
            sv_epilogue(c, frame, 2);
            return Ok(());
        }

        // EE voices: start each one.
        c.cycles += 3;
        let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
        c.g[T2][0] = n;
        c.g[S1][0] = 0;
        if n != 0 {
            c.cycles += 5;
            c.g[S6][0] = c.g[S4][0];
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(SV_DESC_STRETCH));
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            c.g[A0][0] = Addr(r32(c, S3) + SV_DESC_ARG60).lw(c);
            loop {
                c.cycles += 31;
                let desc = r32(c, S3);
                let state = r32(c, S6);
                let base = r32(c, S5);
                let stretch_at = r32(c, S0);
                let s = sp(c);
                c.g[A2][0] = 0x1;
                let priority = Addr(desc + SV_DESC_PRIORITY).lb(c);
                let arg5c = Addr(desc + SV_DESC_ARG5C).lw(c);
                let voice = Addr(r32(c, S4)).lh(c);
                let index = c.g[S1][0];
                Addr(s + 0x28).sw(c, index);
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(2));
                Addr(s).sw(c, arg5c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let arg60 = c.g[A0][0];
                Addr(s + 0x8).sw(c, arg60);
                Addr(s + 0x20).sw(c, priority);
                let ee_first = Addr(state + SV_STATE_EE_VOICE_BASE).lbu(c) as u32;
                let first = Addr(state + SV_STATE_VOICE_BASE).lbu(c) as u32;
                let off0 = Addr(desc + SV_DESC_OFFSETS).lw(c) as u32;
                Addr(s + 0x10).sw(c, 0);
                Addr(s + 0x18).sw(c, 0);
                // s0 walks the stretch words; the loop and sample offsets
                // of the same voice sit 0x18 and 0x30 bytes before.
                let loop_off =
                    Addr(stretch_at.wrapping_sub(SV_DESC_STRETCH - SV_DESC_LOOPS)).lw(c) as u32;
                let off_i =
                    Addr(stretch_at.wrapping_sub(SV_DESC_STRETCH - SV_DESC_OFFSETS)).lw(c) as u32;
                let stretch = Addr(stretch_at).lw(c);
                let codec = Addr(desc + SV_DESC_CODEC).lbu(c);
                let ee_param = Addr(r32(c, S2) + SV_CHAN_EE_PARAM).lw(c);
                // Registers at the call.
                c.g[V0][0] = sx(loop_off);
                c.g[V1][0] = arg5c;
                c.g[A0][0] = sx((voice as u32).wrapping_sub(first.wrapping_add(ee_first)));
                c.g[A1][0] = codec;
                c.g[A3][0] = sx(base.wrapping_add(off_i));
                c.g[T0][0] = sx(base.wrapping_add(off0).wrapping_add(loop_off));
                c.g[T1][0] = stretch;
                c.g[T3][0] = ee_param;
                c.g[S0][0] = sx(stretch_at.wrapping_add(4));
                call_ptr(c, SV_EE_VOICE_START, 0x003c_41fc)?;
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                c.g[T2][0] = n;
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S3) + SV_DESC_ARG60).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_4190);
                }
            }
        }

        // Channel parameters on the first voice.
        c.cycles += 23;
        let first_voice = Addr(r32(c, S2) + SV_CHAN_VOICES);
        c.g[A0][0] = first_voice.lh(c);
        c.g[S1][0] = 0;
        SV_APPLY_PITCH.call(c, 0x003c_4218)?;
        c.g[A0][0] = first_voice.lh(c);
        SV_APPLY_PAN.call(c, 0x003c_4220)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = 0;
        SV_APPLY_SEND.call(c, 0x003c_422c)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = c.g[S7][0];
        SV_APPLY_TIME_STRETCH.call(c, 0x003c_4238)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = c.g[FP][0];
        SV_APPLY_DISTORTION.call(c, 0x003c_4244)?;
        c.g[A1][0] = Addr(sp(c) + 0x30).lw(c);
        c.g[A0][0] = first_voice.lh(c);
        SV_APPLY_LOWPASS.call(c, 0x003c_4250)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = Addr(sp(c) + 0x34).lw(c);
        SV_APPLY_HIGHPASS.call(c, 0x003c_425c)?;
        let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
        c.g[V0][0] = 0x51_0000;
        if n != 0 {
            // Activate each voice.
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            c.g[S3][0] = sx(SV_STATE);
            c.g[V0][0] = Addr(SV_STATE + SV_STATE_EE_VOICE_BASE).lbu(c);
            loop {
                c.cycles += 11;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let state = r32(c, S3);
                let first = Addr(state + SV_STATE_VOICE_BASE).lbu(c) as u32;
                let voice = Addr(r32(c, S0)).lh(c);
                c.g[V1][0] = voice;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(2));
                let bases = first.wrapping_add(r32(c, V0));
                c.g[A0][0] = sx((voice as u32).wrapping_sub(bases));
                call_ptr(c, SV_EE_VOICE_ACTIVATE, 0x003c_4294)?;
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S3) + SV_STATE_EE_VOICE_BASE).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_4278);
                }
            }
        }
        sv_epilogue(c, frame, 1);
        Ok(())
    });
}

/// `SND_ChannelStartVoices`' exit: the `lq s0` block (`pre` cycles), then
/// v0 = 0 and the callee-saved registers back.
pub(crate) fn sv_epilogue(c: &mut Cpu, frame: Frame, pre: u64) {
    c.cycles += pre;
    frame.restore(c, S0, 0xd0);
    c.cycles += 12;
    c.g[V0][0] = 0;
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    frame.restore(c, FP, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_GetBufferedBytes (0x3c6c98)

/// The sound streamer's per-slot stream table (`D_005157B8 + 0x304`).
pub(crate) const SNDSTRM_SLOT_TABLE: u32 = 0x0051_57b8 + 0x304;

/// Stream fields: bytes buffered in the two halves of the ring.
pub(crate) const SNDSTRM_BUFFERED_A: u32 = 0x48;

pub(crate) const SNDSTRM_BUFFERED_B: u32 = 0x4c;

/// `int SNDSTRM_GetBufferedBytes(int slot)` (0x3c6c98): how many bytes the
/// stream in `slot` has buffered (the sum of its two buffer counts).
pub fn sndstrm_get_buffered_bytes(c: &mut Cpu) {
    c.cycles += 9;
    let index = sx((c.g[A0][0] as u32) << 2);
    c.g[A0][0] = index;
    let stream = Addr(SNDSTRM_SLOT_TABLE.wrapping_add(index as u32)).lw(c);
    c.g[V1][0] = stream;
    let a = Addr((stream as u32).wrapping_add(SNDSTRM_BUFFERED_A)).lw(c);
    let b = Addr((stream as u32).wrapping_add(SNDSTRM_BUFFERED_B)).lw(c);
    c.g[A0][0] = a;
    c.g[V0][0] = sx((a as u32).wrapping_add(b as u32));
    ret(c);
}

// ------------------------------------------------------------ SND_GetPanGains

/// The sound library's pan state: `u8 channels` at +0x47.
pub(crate) const SND_PAN_STATE: u32 = 0x0050_a8e8;

pub(crate) const SND_PAN_CHANNELS: u32 = 0x47;

/// Per output channel a 256-entry `u8` gain table, indexed by `pan >> 8`.
pub(crate) const SND_PAN_TABLES: u32 = 0x0050_ac38;

/// `void SND_GetPanGains(u16 pan, float* gains)` (0x3bc4c0): one gain per
/// output channel, `table[ch][pan >> 8] / 255.0`.
pub fn snd_get_pan_gains(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A0][0] = sx(((c.g[A0][0] & 0xffff) as u32) >> 8);
    c.g[V1][0] = sx(SND_PAN_STATE);
    c.g[V0][0] = Addr(SND_PAN_STATE + SND_PAN_CHANNELS).lbu(c);
    c.g[A2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[AT][0] = 0x437f_0000;
        let k255 = FpuF32(0x437f_0000);
        c.f[1] = k255.0;
        c.g[T0][0] = c.g[V1][0];
        c.g[A3][0] = sx(SND_PAN_TABLES);
        loop {
            c.cycles += 13;
            let table = Addr(r32(c, A3)).lw(c);
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
            c.g[V1][0] = sx((table as u32).wrapping_add(r32(c, A0)));
            let level = Addr(r32(c, V1)).lbu(c);
            let gain = FpuF32::from_i32(level as i32).div(k255);
            c.f[0] = gain.0;
            Addr(r32(c, A1)).write_fpu(c, gain);
            let channels = Addr(r32(c, T0).wrapping_add(SND_PAN_CHANNELS)).lbu(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (channels as i64)) as u64;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_c4f0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

pub(crate) mod spch_match {

    pub const COND_COUNT: u32 = 0x4;

    pub const COND_KIND: u32 = 0x8;

    pub const COND_OFFSET: u32 = 0x10;

    pub const COND_MASKS: u32 = 0x14;

    pub const KIND_ALWAYS: u64 = 0;

    pub const KIND_ALWAYS2: u64 = 0xfe;

    pub const KIND_DATA: u64 = 0xff;

    /// Context (a0): two size bytes (+4, +6) placing its value table.
    pub const CTX_SIZE_A: u32 = 0x4;

    pub const CTX_SIZE_B: u32 = 0x6;

    /// The loop head (event checks).
    pub const LOOP: u32 = 0x003d_8630;
}

/// `bool SPCH_MatchConditions(Ctx* ctx, Cond* cond, u32* values, u8*
/// actual)` (0x3d85c8): true when every condition `i < count` (count at
/// most 4, else false) accepts its actual value `actual[i]` (< 32, else
/// false): either the condition does not care (mask bit clear, kind 0 or
/// 0xfe), or bit `actual[i]` is set in the condition's value word
/// (`values[kind]`, or for kind 0xff a word of the context's table (offset
/// bit 7) or the bank data byte at the offset, looked up with
/// `SPCH_FindBankDataPtr(*values, &data)`, which must equal `actual[i]`).
/// Frame 0xe0 (s0-s7, fp, ra as quadwords; a2/t0 kept around the call).
pub fn spch_match_conditions(c: &mut Cpu) {
    use spch_match::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, FP, 0xc0);
        frame.save(c, S7, 0xb0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, RA, 0xd0);
        c.g[S1][0] = 1;
        frame.save(c, S6, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x40);
        let count = Addr(r32(c, FP).wrapping_add(COND_COUNT)).lb(c);
        c.g[T0][0] = count;
        c.g[V0][0] = ((count as i64) < 5) as u64;
        let actual_at = frame.at(c, 0x14);
        let actual = c.g[A3][0];
        Addr(actual_at).sw(c, actual);
        if (count as i64) >= 5 {
            c.cycles += 2;
            c.g[S1][0] = 0;
        } else {
            c.cycles += 2;
            c.g[S3][0] = 0;
            if (count as i64) > 0 {
                c.cycles += 4;
                c.g[S6][0] = sx(0xffff_fffc);
                c.g[S5][0] = sx(r32(c, FP).wrapping_add(COND_MASKS));
                loop {
                    if !spch_match_condition(c, frame)? {
                        // Actual value out of range: fail at once.
                        break;
                    }
                    c.cycles += 2;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                    if c.g[S1][0] == 0 {
                        break;
                    }
                    c.cycles += 3;
                    let more = (c.g[S3][0] as i64) < (c.g[T0][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, LOOP);
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, RA, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// One condition of `SPCH_MatchConditions` (index s3, mask word at s5):
/// leaves s1 = 1 if it accepts the actual value, else 0, up to the loop
/// tail (block 0x3d8738). `false`: the actual value is out of range and the
/// function returns at once (s1 = 0, no tail).
pub(crate) fn spch_match_condition(
    c: &mut Cpu,
    frame: Frame,
) -> Result<bool, crate::call::Unwound> {
    use spch_match::*;
    c.cycles += 6;
    let actual = Addr(frame.at(c, 0x14)).lw(c);
    c.g[V0][0] = actual;
    let at = sx((actual as u32).wrapping_add(r32(c, S3)));
    c.g[V1][0] = at;
    let value = Addr(at as u32).lbu(c);
    c.g[S2][0] = value;
    c.g[V0][0] = (value < 0x20) as u64;
    c.g[S1][0] = 0;
    if value >= 0x20 {
        return Ok(false);
    }
    c.cycles += 7;
    c.g[S4][0] = 1;
    let mask = Addr(r32(c, S5)).lw(c);
    let bit = sx(1u32 << (value as u32 & 31));
    c.g[A1][0] = bit;
    let cond = sx(r32(c, FP).wrapping_add(r32(c, S3)));
    c.g[V0][0] = cond;
    c.g[V1][0] = bit & mask;
    let kind = Addr((cond as u32).wrapping_add(COND_KIND)).lbu(c);
    c.g[A0][0] = kind;
    if bit & mask == 0 {
        return Ok(true);
    }
    c.cycles += 2;
    c.g[V0][0] = KIND_ALWAYS2;
    let always = if kind == KIND_ALWAYS {
        true
    } else {
        c.cycles += 2;
        c.g[V0][0] = KIND_DATA;
        kind == KIND_ALWAYS2
    };
    if always {
        c.cycles += 2;
        c.g[S1][0] = 1;
        return Ok(true);
    }
    c.cycles += 2;
    let index_off = sx((kind as u32) << 2);
    c.g[V1][0] = index_off;
    let word = if kind != KIND_DATA {
        // values[kind]
        c.cycles += 3;
        c.g[S1][0] = c.g[S4][0];
        let at = sx((index_off as u32).wrapping_add(r32(c, A2)));
        c.g[V1][0] = at;
        Addr(at as u32).lw(c)
    } else {
        c.cycles += 5;
        let cond = sx(r32(c, FP).wrapping_add(r32(c, S3)));
        c.g[V0][0] = cond;
        let offset = Addr((cond as u32).wrapping_add(COND_OFFSET)).lbu(c);
        c.g[S0][0] = offset;
        c.g[V1][0] = offset & 0x80;
        c.g[A0][0] = c.g[SP][0];
        if offset & 0x80 == 0 {
            // The bank data byte at `offset` must equal the actual value.
            c.cycles += 11;
            let values = r32(c, A2);
            let sp_ = sp(c);
            c.lwl(V1, values.wrapping_add(3));
            c.lwr(V1, values);
            c.swl(V1, sp_.wrapping_add(3));
            c.swr(V1, sp_);
            c.g[A1][0] = sx(sp_.wrapping_add(0x10));
            frame.save(c, A2, 0x20);
            frame.save(c, T0, 0x30);
            // SPCH_FindBankDataPtr(&id, &data)
            Callee {
                addr: 0x003d_83b8,
                f: ee::s14::f_003d83b8,
            }
            .call(c, 0x003d_86fc)?;
            frame.restore(c, A2, 0x20);
            frame.restore(c, T0, 0x30);
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                let data = Addr(frame.at(c, 0x10)).lw(c);
                c.g[S1][0] = c.g[S4][0];
                let at = sx((data as u32).wrapping_add(r32(c, S0)));
                c.g[V1][0] = at;
                let byte = Addr(at as u32).lbu(c);
                let diff = c.g[S2][0] ^ byte;
                c.g[V0][0] = diff;
                if diff != 0 {
                    c.g[S1][0] = 0;
                }
            }
            return Ok(true);
        }
        // A word of the context's value table.
        c.cycles += 17;
        let ctx = r32(c, S7);
        let size_a = Addr(ctx.wrapping_add(CTX_SIZE_A)).lbu(c);
        let offset = offset & 0x7f;
        c.g[S0][0] = offset;
        let size_b = Addr(ctx.wrapping_add(CTX_SIZE_B)).lbu(c);
        c.g[S1][0] = c.g[S4][0];
        let s6 = c.g[S6][0];
        let a = sx(((size_a as u32 >> 2).wrapping_add(3) as u64 & s6) as u32);
        let b = sx(((size_b as u32) << 2).wrapping_add(3) as u64 as u32) & s6;
        c.g[V0][0] = b;
        let table = sx(ctx
            .wrapping_add((a as u32).wrapping_add(8))
            .wrapping_add(b as u32));
        c.g[V1][0] = table;
        let at = sx(((offset as u32) << 2).wrapping_add(table as u32));
        c.g[A0][0] = at;
        Addr(at as u32).lw(c)
    };
    c.g[V0][0] = word;
    c.cycles += 2;
    let hit = c.g[A1][0] & word;
    c.g[V0][0] = hit;
    if hit == 0 {
        c.g[S1][0] = 0;
    }
    Ok(true)
}

// ---------------------------------------------------------------------------
// SND_ChannelApplySend (0x3c27e0).

/// `SndChannel` field: `s16[]` send levels, one per aux send (index 0 is
/// the SPU effect on/off switch).
pub(crate) const SEND_LEVELS: u32 = 0x62;

/// `1/32767 * 127` (the IOP DSP send byte scale).
pub(crate) const SEND_SCALE_IOP: FpuF32 = FpuF32(0x3b7e_01fc);

/// `32767.0` (the EE send divisor).
pub(crate) const SEND_DIVISOR_EE: FpuF32 = FpuF32(0x46ff_fe00);

/// `SND_EeVoiceSetAuxSend(int voice, int send, float level)`.
pub(crate) const EE_VOICE_SET_AUX_SEND: u32 = 0x003c_8598;

/// IOP DSP voice block: the send bytes (`+ (v - base) * 16 + send`).
pub(crate) const IOP_SEND_BYTES: u32 = 0x1ac;

/// `int SND_ChannelApplySend(int channel, int send)` (0x3c27e0): push the
/// channel's aux send level to its voices. Returns 0.
///
/// ```text
/// SPU voices:  effect mask (u64 at *params) bit v (and v + 0x18 in output
///              mode 2) = send_levels[0] != 0
/// IOP voices:  params[(v - base) * 16 + send + 0x1ac] =
///                  (int)(volume * send_levels[send] * 127/32767)
/// EE voices:   SND_EeVoiceSetAuxSend(v - base - ee_base, send,
///                  volume * send_levels[send] / 32767)
/// ```
///
/// Frame 0x70 (s0 +0x50, s1 +0x40, s2 +0x30, s3 +0x20, s4 +0x10, ra +0,
/// f20 +0x60).
pub fn snd_channel_apply_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        set_reg(c, V0, snd::chan::SIZE as u64);
        frame.save(c, S1, 0x40);
        set_reg(c, V1, snd::STATE_HI);
        frame.save(c, S4, 0x10);
        let offset = snd::channel_at(c);
        frame.save(c, S0, 0x50);
        set_reg(c, A3, sx(snd::STATE));
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        let channels = Addr(snd::STATE).off(snd::state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset as u32));
        set_reg(c, S1, ch);
        let flags = snd::at(ch, snd::chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & snd::chan::SPU);
        let send = reg(c, A1);
        set_reg(c, S4, send);
        if flags & snd::chan::SPU != 0 {
            return snd_send_spu(c, frame, ch);
        }
        c.cycles += 3;
        let level_off = snd::sll(send, 1);
        set_reg(c, V0, level_off);
        if flags & snd::chan::IOP != 0 {
            snd_send_iop(c, frame, ch, level_off)
        } else {
            snd_send_ee(c, frame, ch, level_off)
        }
    })
}

/// `volume * (float)send_levels[send]` (f1; f0 the converted level, AT and
/// f2 the scale constant, v1 the level, v0 the voice count, s2 = 0).
/// Returns (product, voice count).
pub(crate) fn snd_send_level(c: &mut Cpu, ch: u64, level_off: u64, scale: FpuF32) -> (FpuF32, u64) {
    let volume = snd::at(ch, snd::chan::VOLUME).read_fpu(c);
    let level = snd::at(addu(ch, level_off), SEND_LEVELS).lh(c);
    set_reg(c, AT, scale.0 as u64);
    abi::set_fpr(c, 2, scale);
    set_reg(c, V1, level);
    let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
    set_reg(c, V0, count);
    let level_f = FpuF32::from_i32(level as u32 as i32);
    abi::set_fpr(c, 0, level_f);
    let product = volume * level_f;
    abi::set_fpr(c, 1, product);
    set_reg(c, S2, 0);
    (product, count)
}

/// SPU voices: set or clear each voice's bit in the effect mask.
pub(crate) fn snd_send_spu(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 3;
    let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
    set_reg(c, V0, count);
    set_reg(c, S2, 0);
    if count == 0 {
        return snd_send_exit(c, frame, 1);
    }
    c.cycles += 9;
    set_reg(c, V0, snd::STATE_HI);
    let state_ptr = reg(c, A3);
    set_reg(c, T4, state_ptr);
    set_reg(c, T0, sx(snd::SYNC));
    set_reg(c, A3, 1);
    set_reg(c, T3, 2);
    set_reg(c, T2, 0);
    set_reg(c, T1, 0);
    set_reg(c, A2, snd::addiu(ch, snd::chan::VOICES as i32));
    loop {
        // Loop head 0x3c2858 (a2 the voice slot, s2 the voices done, t1/t2
        // the slot offset).
        c.cycles += 3 + 6;
        let on = snd::at(ch, SEND_LEVELS).lh(c);
        let slots = snd::addiu(ch, snd::chan::VOICES as i32);
        set_reg(c, A1, slots);
        let mask_at = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
        set_reg(c, V0, mask_at);
        let voice = snd::at(reg(c, A2), 0).lh(c);
        let mask = snd::at(mask_at, 0).ld(c);
        let bit = 1u64 << (voice & 63);
        let (v1, new_mask) = if on != 0 {
            (bit, mask | bit)
        } else {
            (!bit, mask & !bit)
        };
        set_reg(c, V1, v1);
        set_reg(c, A0, new_mask);
        c.cycles += 4;
        snd::at(mask_at, 0).sd(c, new_mask);
        let mode = snd::at(reg(c, T4), snd::state::OUTPUT_MODE).lbu(c);
        set_reg(c, V0, mode);
        if mode == 2 {
            // The second voice block (bits + 0x18).
            c.cycles += 3;
            let on = snd::at(ch, SEND_LEVELS).lh(c);
            set_reg(c, V0, on);
            let slot_off = reg(c, T1);
            set_reg(c, A0, addu(slots, slot_off));
            let (mask_at, v0, v1);
            if on != 0 {
                c.cycles += 7;
                mask_at = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let voice = snd::at(reg(c, A0), 0).lh(c);
                let mask = snd::at(mask_at, 0).ld(c);
                v0 = 1u64 << (snd::addiu(voice, 0x18) & 63);
                v1 = mask | v0;
            } else {
                c.cycles += 8;
                let slot_off = reg(c, T2);
                set_reg(c, A0, addu(slots, slot_off));
                let voice = snd::at(reg(c, A0), 0).lh(c);
                mask_at = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let mask = snd::at(mask_at, 0).ld(c);
                v0 = !(1u64 << (snd::addiu(voice, 0x18) & 63));
                v1 = mask & v0;
            }
            set_reg(c, A1, mask_at);
            set_reg(c, V0, v0);
            set_reg(c, V1, v1);
            c.cycles += 2;
            snd::at(mask_at, 0).sd(c, v1);
        }
        let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
        c.cycles += 6;
        let done = snd::addiu(reg(c, S2), 1);
        set_reg(c, S2, done);
        let t2 = reg(c, T2);
        set_reg(c, T2, snd::addiu(t2, 2));
        let t1 = reg(c, T1);
        set_reg(c, T1, snd::addiu(t1, 2));
        let more = snd::slt(done, count);
        set_reg(c, V0, more);
        let slot = reg(c, A2);
        set_reg(c, A2, snd::addiu(slot, 2));
        if more == 0 {
            return snd_send_exit(c, frame, 2);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2858);
        }
    }
}

/// IOP DSP voices: one send byte per voice.
pub(crate) fn snd_send_iop(c: &mut Cpu, frame: Frame, ch: u64, level_off: u64) -> Flow {
    c.cycles += 15;
    let (product, count) = snd_send_level(c, ch, level_off, SEND_SCALE_IOP);
    let scaled = product * SEND_SCALE_IOP;
    abi::set_fpr(c, 1, scaled);
    let bits = snd::cvt_w(scaled);
    c.f[0] = bits;
    set_reg(c, A2, sx(bits));
    if count == 0 {
        return snd_send_exit(c, frame, 1);
    }
    c.cycles += 5;
    set_reg(c, V0, snd::STATE_HI);
    let state_ptr = reg(c, A3);
    set_reg(c, T0, state_ptr);
    set_reg(c, A3, sx(snd::SYNC));
    set_reg(c, A1, snd::addiu(ch, snd::chan::VOICES as i32));
    loop {
        // Loop head 0x3c2970 (a1 the voice slot, s2 the voices done).
        c.cycles += 13;
        let slot = reg(c, A1);
        let voice = snd::at(slot, 0).lh(c);
        let done = snd::addiu(reg(c, S2), 1);
        set_reg(c, S2, done);
        let base = snd::at(reg(c, T0), snd::state::VOICE_BASE).lbu(c);
        set_reg(c, A0, base);
        let params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
        let off = addu(reg(c, S4), snd::sll(snd::subu(voice, base), 4));
        set_reg(c, V0, off);
        let dst = addu(params, off);
        set_reg(c, V1, dst);
        let level = reg(c, A2);
        snd::at(dst, IOP_SEND_BYTES).sb(c, level);
        let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
        let more = snd::slt(done, count);
        set_reg(c, V0, more);
        set_reg(c, A1, snd::addiu(slot, 2));
        if more == 0 {
            return snd_send_exit(c, frame, 2);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2970);
        }
    }
}

/// EE software voices: `SND_EeVoiceSetAuxSend(voice, send, level)` per
/// voice.
pub(crate) fn snd_send_ee(c: &mut Cpu, frame: Frame, ch: u64, level_off: u64) -> Flow {
    c.cycles += 13;
    let (product, count) = snd_send_level(c, ch, level_off, SEND_DIVISOR_EE);
    abi::set_fpr(c, 20, product.div(SEND_DIVISOR_EE));
    if count == 0 {
        return snd_send_exit(c, frame, 1);
    }
    c.cycles += 4;
    let state_ptr = reg(c, A3);
    set_reg(c, S3, state_ptr);
    set_reg(c, S0, snd::addiu(ch, snd::chan::VOICES as i32));
    let ee_base = snd::at(state_ptr, snd::state::EE_VOICE_BASE).lbu(c);
    set_reg(c, V0, ee_base);
    loop {
        // Loop head 0x3c29f0 (s0 the voice slot, s2 the voices done, v0 the
        // EE voice base).
        c.cycles += 13;
        let send = reg(c, S4);
        set_reg(c, A1, send);
        let base = snd::at(reg(c, S3), snd::state::VOICE_BASE).lbu(c);
        c.f[12] = c.f[20];
        let slot = reg(c, S0);
        let voice = snd::at(slot, 0).lh(c);
        set_reg(c, V1, voice);
        let done = snd::addiu(reg(c, S2), 1);
        set_reg(c, S2, done);
        let first = addu(base, reg(c, V0));
        set_reg(c, S0, snd::addiu(slot, 2));
        set_reg(c, A0, snd::subu(voice, first));
        call_ptr(c, EE_VOICE_SET_AUX_SEND, 0x003c_2a14)?;
        let count = snd::at(reg(c, S1), snd::chan::VOICE_COUNT).lbu(c);
        let more = snd::slt(reg(c, S2), count);
        set_reg(c, V0, more);
        if more == 0 {
            return snd_send_exit(c, frame, 1);
        }
        let ee_base = snd::at(reg(c, S3), snd::state::EE_VOICE_BASE).lbu(c);
        set_reg(c, V0, ee_base);
        if event_due(c) {
            deliver_event_here(c, 0x003c_29f0);
        }
    }
}

/// `lq s0` (its block costs `cycles`), then the epilogue: return 0.
pub(crate) fn snd_send_exit(c: &mut Cpu, frame: Frame, cycles: u64) -> Flow {
    c.cycles += cycles;
    frame.restore(c, S0, 0x50);
    c.cycles += 9;
    set_reg(c, V0, 0);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x60);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SNDSTRM_HandleChunk (0x3b7080).

/// `SndStream*[]`: the open streams by slot.
pub(crate) const SNDSTRM_STREAMS: u32 = 0x0050_a0a0;

/// Chunk tag `SCDl` (sample data).
pub(crate) const SNDSTRM_TAG_DATA: u64 = 0x6c44_4353;

/// Chunk tag `SCHl` (stream header).
pub(crate) const SNDSTRM_TAG_HEADER: u64 = 0x6c48_4353;

/// `SNDSTRM_ParseData(stream, chunk)`.
pub(crate) const SNDSTRM_PARSE_DATA: Callee = Callee {
    addr: 0x003b_6ea8,
    f: ee::s13::SNDSTRM_ParseData,
};

/// `SNDSTRM_ParseHeader(slot, chunk)`.
pub(crate) const SNDSTRM_PARSE_HEADER: Callee = Callee {
    addr: 0x003b_6bd0,
    f: ee::s13::SNDSTRM_ParseHeader,
};

/// `StreamFile_ReleaseChunk(file)`.
pub(crate) const SNDSTRM_RELEASE_CHUNK: u32 = 0x003e_13e8;

/// `int SNDSTRM_HandleChunk(int slot, Chunk* chunk)` (0x3b7080): dispatch
/// a chunk read for stream `slot` by its tag: `SCDl` data goes to
/// `SNDSTRM_ParseData(stream, chunk)` (returns 1), an `SCHl` header to
/// `SNDSTRM_ParseHeader(slot, chunk)` (returns 0), anything else is
/// released back to the stream's file (returns 1). Frame 0x10 (ra +0).
pub fn sndstrm_handle_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let slot = reg(c, A0);
        set_reg(c, A3, slot);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let entry = sx(((slot as u32) << 2).wrapping_add(SNDSTRM_STREAMS));
        set_reg(c, V1, entry);
        set_reg(c, V0, SNDSTRM_TAG_DATA);
        let tag = Addr(r32(c, A1)).lw(c);
        set_reg(c, A2, tag);
        let stream = Addr(entry as u32).lw(c);
        set_reg(c, A0, stream);
        if tag == SNDSTRM_TAG_DATA {
            c.cycles += 4;
            SNDSTRM_PARSE_DATA.call(c, 0x003b_70b8)?;
            set_reg(c, V0, 1);
        } else {
            c.cycles += 4;
            set_reg(c, V0, SNDSTRM_TAG_HEADER);
            if tag == SNDSTRM_TAG_HEADER {
                c.cycles += 4;
                let slot = reg(c, A3);
                set_reg(c, A0, slot);
                SNDSTRM_PARSE_HEADER.call(c, 0x003b_70d8)?;
                set_reg(c, V0, 0);
            } else {
                c.cycles += 3;
                let file = Addr(r32(c, A0)).lw(c);
                set_reg(c, A0, file);
                call_ptr(c, SNDSTRM_RELEASE_CHUNK, 0x003b_70e8)?;
                set_reg(c, V0, 1);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_TimerServiceCallback (0x3b5450).

/// `SND_RunServiceCallbacks()`.
pub(crate) const SND_TIMER_RUN_SERVICE_CALLBACKS: Callee = Callee {
    addr: 0x003c_60e0,
    f: ee::s13::SND_RunServiceCallbacks,
};

/// `int SND_TimerServiceCallback()` (0x3b5450): the sound timer's service
/// tick: run the registered service callbacks, return 0. Frame 0x10 (ra +0).
pub fn snd_timer_service_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        SND_TIMER_RUN_SERVICE_CALLBACKS.call(c, 0x003b_5460)?;
        frame.restore_ra(c, 0);
        set_reg(c, V0, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyFxEnable (0x3c2ec8).

/// `SndChannel` field: `s8` effect enable (the SPU path tests it, as `lb`).
pub(crate) const FX_ENABLE: u32 = 0x5f;

/// Offset of the SPU effect-enable mask (`u64`) in the voice parameter
/// block.
pub(crate) const FX_SPU_MASK: u32 = 0x8;

/// `SND_ChannelApplyVolume(channel)`.
pub(crate) const FX_APPLY_VOLUME: Callee = Callee {
    addr: 0x003c_2a50,
    f: ee::s13::SND_ChannelApplyVolume,
};

/// `void SND_ChannelApplyFxEnable(int channel)` (0x3c2ec8): push the
/// channel's effect-enable switch to its voices. SPU voices get their bit
/// (and bit + 0x18 in output mode 2) set or cleared in the effect mask
/// (`u64` at params + 8); IOP and EE voices take it through their levels:
/// `SND_ChannelApplyVolume(channel)`.
///
/// Frame 0x10 (ra +0).
pub fn snd_channel_apply_fx_enable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let index = reg(c, A0);
        set_reg(c, A1, index);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        set_reg(c, V0, snd::STATE_HI);
        let offset = abi::mult(c, index as u32, snd::chan::SIZE);
        set_reg(c, A0, sx(offset));
        set_reg(c, A3, sx(snd::STATE));
        let channels = Addr(snd::STATE).off(snd::state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset));
        set_reg(c, A2, ch);
        let flags = snd::at(ch, snd::chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & snd::chan::IOP);
        if flags & snd::chan::SPU != 0 {
            c.cycles += 3;
            let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
            set_reg(c, V0, count);
            set_reg(c, T2, 0);
            if count == 0 {
                c.cycles += 1;
            } else {
                fx_enable_spu(c, ch);
                c.cycles += 2;
            }
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            let index = reg(c, A1);
            set_reg(c, A0, index);
            if flags & snd::chan::IOP != 0 {
                c.cycles += 4;
                FX_APPLY_VOLUME.call(c, 0x003c_2ff4)?;
            } else {
                c.cycles += 2;
                FX_APPLY_VOLUME.call(c, 0x003c_3004)?;
                c.cycles += 1;
            }
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// The SPU voices' mask loop (0x3c2f28; a3 the voice slot, t2 the voices
/// done, t3/t4 the slot offset).
pub(crate) fn fx_enable_spu(c: &mut Cpu, ch: u64) {
    c.cycles += 8;
    set_reg(c, V0, snd::STATE_HI);
    let state_ptr = reg(c, A3);
    set_reg(c, T6, state_ptr);
    set_reg(c, T1, sx(snd::SYNC));
    set_reg(c, T0, 1);
    set_reg(c, T5, 2);
    set_reg(c, T4, 0);
    set_reg(c, T3, 0);
    set_reg(c, A3, snd::addiu(ch, snd::chan::VOICES as i32));
    loop {
        c.cycles += 3 + 6;
        let on = snd::at(ch, FX_ENABLE).lb(c);
        let slots = snd::addiu(ch, snd::chan::VOICES as i32);
        set_reg(c, A1, slots);
        let params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
        set_reg(c, V0, params);
        let voice = snd::at(reg(c, A3), 0).lh(c);
        let mask = snd::at(params, FX_SPU_MASK).ld(c);
        let bit = 1u64 << (voice & 63);
        let (v1, new_mask) = if on != 0 {
            (bit, mask | bit)
        } else {
            (!bit, mask & !bit)
        };
        set_reg(c, V1, v1);
        set_reg(c, A0, new_mask);
        c.cycles += 4;
        snd::at(params, FX_SPU_MASK).sd(c, new_mask);
        let mode = snd::at(reg(c, T6), snd::state::OUTPUT_MODE).lbu(c);
        set_reg(c, V0, mode);
        if mode == 2 {
            // The second voice block (bits + 0x18).
            c.cycles += 3;
            let on = snd::at(ch, FX_ENABLE).lb(c);
            set_reg(c, V0, on);
            let slot_off = reg(c, T3);
            set_reg(c, A0, addu(slots, slot_off));
            let (params, v0, v1);
            if on != 0 {
                c.cycles += 7;
                params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let voice = snd::at(reg(c, A0), 0).lh(c);
                let mask = snd::at(params, FX_SPU_MASK).ld(c);
                v0 = 1u64 << (snd::addiu(voice, 0x18) & 63);
                v1 = mask | v0;
            } else {
                c.cycles += 8;
                let slot_off = reg(c, T4);
                set_reg(c, A0, addu(slots, slot_off));
                let voice = snd::at(reg(c, A0), 0).lh(c);
                params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let mask = snd::at(params, FX_SPU_MASK).ld(c);
                v0 = !(1u64 << (snd::addiu(voice, 0x18) & 63));
                v1 = mask & v0;
            }
            set_reg(c, A1, params);
            set_reg(c, V0, v0);
            set_reg(c, V1, v1);
            c.cycles += 2;
            snd::at(params, FX_SPU_MASK).sd(c, v1);
        }
        let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
        c.cycles += 6;
        let done = snd::addiu(reg(c, T2), 1);
        set_reg(c, T2, done);
        let t4 = reg(c, T4);
        set_reg(c, T4, snd::addiu(t4, 2));
        let t3 = reg(c, T3);
        set_reg(c, T3, snd::addiu(t3, 2));
        let more = snd::slt(done, count);
        set_reg(c, V0, more);
        let slot = reg(c, A3);
        set_reg(c, A3, snd::addiu(slot, 2));
        if more == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2f28);
        }
    }
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetVolume (0x3c8560)

/// The sound driver state (0x515b40) and its voice table pointer (+0x1dc).
pub(crate) const SND_SET_VOL_STATE: u32 = 0x0051_5b40;

pub(crate) const SND_SET_VOL_VOICES: u32 = 0x1dc;

/// A voice entry: 0x60 bytes; dirty flag byte at +1, volumes (floats) at +0x1c.
pub(crate) const SND_SET_VOL_VOICE_SIZE: u32 = 0x60;

pub(crate) const SND_SET_VOL_DIRTY: u32 = 0x1;

pub(crate) const SND_SET_VOL_VOLUMES: u32 = 0x1c;

/// `void SND_EeVoiceSetVolume(int voice, int channel, float volume)`
/// (0x3c8560): store `volume` as the voice's channel volume and mark the
/// voice dirty.
pub fn snd_ee_voice_set_volume(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = SND_SET_VOL_VOICE_SIZE as u64;
    let voice_index = r32(c, A0);
    let off = abi::mult(c, voice_index, SND_SET_VOL_VOICE_SIZE);
    c.g[A0][0] = sx(off);
    c.g[A2][0] = sx(SND_SET_VOL_STATE);
    let voices = Addr(SND_SET_VOL_STATE + SND_SET_VOL_VOICES).lw(c);
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[A3][0] = 1;
    let slot = off.wrapping_add(voices as u32).wrapping_add(r32(c, A1));
    c.g[V0][0] = sx(slot);
    let volume = FpuF32(c.f[12]);
    Addr(slot.wrapping_add(SND_SET_VOL_VOLUMES)).write_fpu(c, volume);
    c.g[V1][0] = Addr(SND_SET_VOL_STATE + SND_SET_VOL_VOICES).lw(c);
    let voice = off.wrapping_add(r32(c, V1));
    c.g[A0][0] = sx(voice);
    Addr(voice.wrapping_add(SND_SET_VOL_DIRTY)).sb(c, 1);
    ret(c);
}

/// `int SND_EaxaSetInput(EaxaDecoder* dec, void* data, int size, int mode)`
/// (0x3ccf60): give an idle EA-XA decoder its input (data at `+0xa0`, size
/// at `+0xc`, mode at `+0x8`) and return 0; -1 when `data` is null or the
/// decoder is still busy (mode non-zero).
pub fn snd_eaxa_set_input(c: &mut Cpu) {
    const MODE: u32 = 0x8;
    const SIZE: u32 = 0xc;
    const DATA_PTR: u32 = 0xa0;
    c.cycles += 2;
    let dec = Addr(c.g[A0][0] as u32);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        return ret(c);
    }
    let busy = dec.off(MODE).lw(c) != 0;
    c.cycles += 2;
    if busy {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
    } else {
        c.cycles += 4;
        c.g[V0][0] = 0;
        let (data, size, mode) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
        dec.off(DATA_PTR).sw(c, data);
        dec.off(MODE).sw(c, mode);
        dec.off(SIZE).sw(c, size);
    }
    ret(c);
}

/// `int SND_ChannelStopVoices(int channel)` (0x3c2440): silence and release
/// every voice of a sound channel. SPU voices (flag 0x8) get their key word
/// in the voice-parameter table cleared (in output mode 2 also the mirror
/// voice `+0x18`); IOP DSP voices (flag 0x100) the word at `+0x1a0` of their
/// 16-byte block (numbered from the IOP voice base); EE voices are stopped
/// with `SND_EeVoiceStop` (numbered from the IOP + EE voice bases). Each
/// voice is then given back with `SND_ChannelRelease`. Returns 0.
pub fn snd_channel_stop_voices(c: &mut Cpu) {
    const STATE: u32 = 0x0050_a8e8;
    const SYNC: u32 = 0x0050_ad00;
    const CHANNELS: u32 = 0x1f0; // STATE+: SndChannel[]
    const CHANNEL_SIZE: u64 = 0x8c;
    const EE_VOICE_BASE: u32 = 0x2f; // STATE+: u8
    const VOICE_BASE: u32 = 0x30; // STATE+: u8
    const OUTPUT_MODE: u32 = 0x49; // STATE+: u8
    const VOICE_PARAMS: u32 = 0xcb0; // SYNC+
    const CH_VOICES: u32 = 0x04; // s16[]
    const CH_VOICE_COUNT: u32 = 0x23; // u8
    const CH_FLAGS: u32 = 0x24; // u16
    const MIRROR_VOICE: u32 = 0x18;
    const SND_CHANNEL_RELEASE: Callee = Callee {
        addr: 0x003b_a550,
        f: ee::s13::SND_ChannelRelease,
    };
    const SND_EE_VOICE_STOP: Callee = Callee {
        addr: 0x003c_8468,
        f: ee::s14::f_003c8468,
    };
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = CHANNEL_SIZE;
        frame.save(c, S2, 0x40);
        c.mult(0, A0, A0, V0);
        frame.save(c, S0, 0x60);
        c.g[V1][0] = 0x0051_0000;
        frame.save(c, S1, 0x50);
        c.g[V1][0] = sx(STATE);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let chans = Addr(STATE + CHANNELS).lw(c);
        c.g[V0][0] = chans;
        c.g[S2][0] = sx((chans as u32).wrapping_add(c.g[A0][0] as u32));
        let ch = c.g[S2][0] as u32;
        let flags = Addr(ch + CH_FLAGS).lhu(c);
        c.g[A0][0] = flags;
        c.g[V0][0] = flags & 0x100;
        let voice_count = |c: &mut Cpu| Addr(c.g[S2][0] as u32).off(CH_VOICE_COUNT).lbu(c);
        if flags & 0x8 != 0 {
            // SPU voices.
            c.cycles += 3;
            c.g[V0][0] = voice_count(c);
            c.g[S1][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x0051_0000;
                c.g[S5][0] = c.g[V1][0];
                c.g[S3][0] = sx(SYNC);
                c.g[S4][0] = 2;
                c.g[S0][0] = sx(ch.wrapping_add(CH_VOICES));
                loop {
                    c.cycles += 10;
                    let slot = Addr(c.g[S0][0] as u32);
                    let voice = slot.lh(c);
                    let params = Addr((c.g[S3][0] as u32).wrapping_add(VOICE_PARAMS)).lw(c);
                    c.g[V0][0] = sx((voice as u32) << 3);
                    c.g[V1][0] = sx((params as u32).wrapping_add(c.g[V0][0] as u32));
                    Addr(c.g[V1][0] as u32).off(0x20).sw(c, 0);
                    c.g[A0][0] = slot.lh(c);
                    SND_CHANNEL_RELEASE.call(c, 0x003c_24c4)?;
                    c.g[V0][0] = Addr(c.g[S5][0] as u32).off(OUTPUT_MODE).lbu(c);
                    if c.g[V0][0] == c.g[S4][0] {
                        c.cycles += 7;
                        let voice = Addr(c.g[S0][0] as u32).lh(c);
                        let params = Addr((c.g[S3][0] as u32).wrapping_add(VOICE_PARAMS)).lw(c);
                        let v = sx((voice as u32).wrapping_add(MIRROR_VOICE));
                        c.g[V0][0] = sx((v as u32) << 3);
                        c.g[V1][0] = sx((params as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).off(0x20).sw(c, 0);
                    }
                    c.g[V0][0] = voice_count(c);
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                    if !more {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x60);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_24a8);
                    }
                }
            }
        } else {
            c.cycles += 2;
            let iop = c.g[V0][0] != 0;
            c.g[V0][0] = voice_count(c);
            if iop {
                // IOP DSP voices.
                c.cycles += 2;
                c.g[S1][0] = 0;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x0051_0000;
                    c.g[S4][0] = c.g[V1][0];
                    c.g[S3][0] = sx(SYNC);
                    c.g[S0][0] = sx(ch.wrapping_add(CH_VOICES));
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lh(c);
                    loop {
                        c.cycles += 14;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        let base = Addr(c.g[S4][0] as u32).off(VOICE_BASE).lbu(c);
                        c.g[A0][0] = base;
                        let params = Addr((c.g[S3][0] as u32).wrapping_add(VOICE_PARAMS)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(base as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V1][0] = sx((params as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).off(0x1a0).sw(c, 0);
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                        SND_CHANNEL_RELEASE.call(c, 0x003c_2550)?;
                        let count = Addr(c.g[S2][0] as u32).off(CH_VOICE_COUNT).lbu(c);
                        let more = (c.g[S1][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            c.cycles += 2;
                            frame.restore(c, S0, 0x60);
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lh(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_2528);
                        }
                    }
                }
            } else {
                // EE voices.
                c.cycles += 2;
                c.g[S1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[S3][0] = c.g[V1][0];
                    c.g[S0][0] = sx(ch.wrapping_add(CH_VOICES));
                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(EE_VOICE_BASE).lbu(c);
                    loop {
                        c.cycles += 13;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        let base = Addr(c.g[S3][0] as u32).off(VOICE_BASE).lbu(c);
                        let voice = Addr(c.g[S0][0] as u32).lh(c);
                        c.g[V1][0] = voice;
                        let first = sx((base as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = sx((voice as u32).wrapping_sub(first as u32));
                        SND_EE_VOICE_STOP.call(c, 0x003c_2598)?;
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                        SND_CHANNEL_RELEASE.call(c, 0x003c_25a4)?;
                        let count = Addr(c.g[S2][0] as u32).off(CH_VOICE_COUNT).lbu(c);
                        let more = (c.g[S1][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(EE_VOICE_BASE).lbu(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_2580);
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 0;
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
// SND_ParseStreamHeader (0x003bb820)

/// The sound library's global state block (`D_0050A8E8`).
pub(crate) const PSH_SND_STATE: u32 = 0x0050_a8e8;

/// `u8` in the state block: the output channel layout (2 = stereo).
pub(crate) const PSH_STATE_OUT_CHANNELS: u32 = 0x47;

/// `u16 [channels][6]` in the state block: per-layout channel offsets.
pub(crate) const PSH_STATE_CHAN_TABLE: u32 = 0x2e0;

/// Stream header (`hdr`, s3): `u16 rate`, `u8 channels`, `u8 codec`.
pub(crate) const PSH_HDR_RATE: u32 = 0x0;

pub(crate) const PSH_HDR_CHANNELS: u32 = 0x2;

/// Patch params (`out`, s0): `u16 flags` at +8, channel offsets (`u16`) at
/// +0xc, marker tables at +0x48 / +0x58.
pub(crate) const PSH_OUT_FLAGS: u32 = 0x8;

pub(crate) const PSH_OUT_CHAN_OFF: u32 = 0xc;

pub(crate) const PSH_OUT_MARKER_DATA: u32 = 0x48;

pub(crate) const PSH_OUT_MARKER_SIZE: u32 = 0x58;

/// Extra info (`extra`, s2): word 0, then per-channel data offsets.
pub(crate) const PSH_EXTRA_OFFS: u32 = 0x4;

/// The tag record `SND_ParseNextTag` fills (at sp): read pointer, id, value,
/// data pointer, data size.
pub(crate) const PSH_REC_ID: u32 = 0x4;

pub(crate) const PSH_REC_VALUE: u32 = 0x8;

pub(crate) const PSH_REC_DATA: u32 = 0xc;

pub(crate) const PSH_REC_SIZE: u32 = 0x10;

/// The six per-channel base offsets (`vals`) on the stack.
pub(crate) const PSH_VALS: u32 = 0x20;

/// Where a tag's value goes.
#[derive(Clone, Copy)]
pub(crate) enum PshTag {
    /// `sb` of the value's low byte to `base + off`.
    Byte(usize, u32),
    /// `sh` of the value's low half.
    Half(usize, u32),
    /// `sw` of the value.
    Word(usize, u32),
    /// A copied data block: `out[data] = SND_MemAlloc(size)`, copy, `out[size]
    /// = size`; the block's address (for the return addresses).
    Data(u32, u32, u32),
    /// The reference offset pointer (`ref`, s5 = the data pointer).
    Ref,
    /// The output flags (s7 = the value).
    Flags,
    /// A marker (data pointer, size) appended to the marker tables (s6 = n).
    Marker,
}

/// The tag ids in the order the compiled compare chain tests them.
pub(crate) const PSH_TAGS: [(u64, PshTag); 29] = [
    (0xa0, PshTag::Byte(S3, 0x3)),
    (0x9c, PshTag::Word(SP, PSH_VALS)),
    (0x9d, PshTag::Word(SP, PSH_VALS + 0x4)),
    (0x9e, PshTag::Word(SP, PSH_VALS + 0x8)),
    (0x9f, PshTag::Word(SP, PSH_VALS + 0xc)),
    (0xa6, PshTag::Word(SP, PSH_VALS + 0x10)),
    (0xa7, PshTag::Word(SP, PSH_VALS + 0x14)),
    (0x98, PshTag::Data(0x18, 0x30, 0x003b_b9f4)),
    (0x99, PshTag::Data(0x1c, 0x34, 0x003b_ba24)),
    (0x9a, PshTag::Data(0x20, 0x38, 0x003b_ba54)),
    (0x9b, PshTag::Data(0x24, 0x3c, 0x003b_ba84)),
    (0xa4, PshTag::Data(0x28, 0x40, 0x003b_bab4)),
    (0xa5, PshTag::Data(0x2c, 0x44, 0x003b_bae4)),
    (0x80, PshTag::Byte(S0, 0x7)),
    (0x82, PshTag::Byte(S3, PSH_HDR_CHANNELS)),
    (0x84, PshTag::Half(S3, PSH_HDR_RATE)),
    (0x85, PshTag::Word(S2, 0x0)),
    (0x8a, PshTag::Ref),
    (0x13, PshTag::Byte(S0, 0x5)),
    (0x0a, PshTag::Byte(S0, 0x6)),
    (0x06, PshTag::Byte(S0, 0x2)),
    (0x8c, PshTag::Flags),
    (0x88, PshTag::Word(S2, PSH_EXTRA_OFFS)),
    (0x89, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x4)),
    (0x94, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x8)),
    (0x95, PshTag::Word(S2, PSH_EXTRA_OFFS + 0xc)),
    (0xa2, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x10)),
    (0xa3, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x14)),
    (0x14, PshTag::Marker),
];

/// `addu` of two registers' low words.
#[inline]
pub(crate) fn psh_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// Stages of [`snd_parse_stream_header`] (labels of the original).
#[derive(Clone, Copy)]
pub(crate) enum PshStage {
    /// 0x3bbc54: parse the next tag.
    NextTag,
    /// 0x3bb958: store the tag in v1 (loop head: a resume entry).
    Dispatch,
    /// 0x3bbc64: tags done, fix up the channels.
    Channels,
    /// 0x3bbcb0: one channel, counting down in a1 (a resume entry).
    Channel,
    /// 0x3bbd6c: epilogue.
    Exit,
}

/// `void SND_ParseStreamHeader(int base, const u8* src, StreamHdr* hdr,
/// PatchParams* out, StreamExtra* extra, u8* isGstr)` (0x003bb820): parse
/// an EA stream header ("SCHl"-style tag list after a 4-byte magic) into
/// the header, the patch params and the extra info. A "GSTR" header (not
/// "PT") goes to `SND_ParsePtHeader` instead and sets `*isGstr`. After the
/// tags, every channel gets its output offset from the layout table, and
/// its data offset made absolute (`base + `) or relative to the reference
/// offset (`*ref`), depending on the output flags.
///
/// Dispatcher-shaped: an event at the tag loop's or the channel loop's back
/// edge may unwind; the trampoline re-enters at 0x3bb958 / 0x3bbcb0 with
/// the loop state in the registers.
pub fn snd_parse_stream_header(c: &mut Cpu) {
    const START: u32 = 0x003b_b820;
    let frame = Frame { size: 0xe0 };
    guest(c, |c| {
        let mut stage = match c.pc {
            START => psh_prologue(c, frame)?,
            0x003b_b958 => PshStage::Dispatch,
            0x003b_bcb0 => PshStage::Channel,
            pc => {
                c.jump_out(pc, START, 0x003b_bd9c);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                PshStage::NextTag => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[SP][0];
                    SND_PARSE_NEXT_TAG.call(c, 0x003b_bc5c)?;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = Addr(sp(c)).off(PSH_REC_ID).lw(c);
                    if !more {
                        PshStage::Channels
                    } else if back_edge_unwinds(c, 0x003b_b958) {
                        return Ok(());
                    } else {
                        PshStage::Dispatch
                    }
                }
                PshStage::Dispatch => {
                    psh_store_tag(c)?;
                    PshStage::NextTag
                }
                PshStage::Channels => {
                    c.cycles += 5;
                    let out = r32(c, S0);
                    let flags = c.g[S7][0];
                    Addr(out).off(PSH_OUT_FLAGS).sh(c, flags);
                    let channels = Addr(r32(c, S3)).off(PSH_HDR_CHANNELS).lbu(c);
                    let ch = sx((channels as u32).wrapping_sub(1));
                    c.g[A1][0] = ch;
                    c.g[V0][0] = 0x51_0000;
                    if (ch as i64) < 0 {
                        PshStage::Exit
                    } else {
                        // Loop registers: a1 = ch, a3 = &vals[ch], a2 =
                        // &extra->offs[ch], t1 = &out->chanOff[ch], t2 = the
                        // state block, t6 = its channel table.
                        c.cycles += 14;
                        c.g[A0][0] = sx((ch as u32) << 2);
                        c.g[T2][0] = sx(PSH_SND_STATE);
                        c.g[V1][0] = sx((ch as u32) << 1);
                        c.g[V0][0] = psh_addu(c.g[A0][0], 4);
                        c.g[V1][0] = psh_addu(c.g[V1][0], PSH_OUT_CHAN_OFF as u64);
                        c.g[A0][0] = psh_addu(c.g[A0][0], c.g[SP][0]);
                        c.g[T6][0] = sx(PSH_SND_STATE + PSH_STATE_CHAN_TABLE);
                        c.g[A3][0] = psh_addu(c.g[A0][0], PSH_VALS as u64);
                        c.g[A2][0] = psh_addu(c.g[V0][0], c.g[S2][0]);
                        c.g[T1][0] = psh_addu(c.g[V1][0], c.g[S0][0]);
                        c.g[T5][0] = 2;
                        c.g[T4][0] = 0xc;
                        c.g[T3][0] = psh_addu(c.g[S2][0], PSH_EXTRA_OFFS as u64);
                        PshStage::Channel
                    }
                }
                PshStage::Channel => {
                    psh_channel(c);
                    c.cycles += 4;
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(2));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                    let more = (c.g[A1][0] as i64) >= 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(4));
                    if !more {
                        PshStage::Exit
                    } else if back_edge_unwinds(c, 0x003b_bcb0) {
                        return Ok(());
                    } else {
                        PshStage::Channel
                    }
                }
                PshStage::Exit => {
                    c.cycles += 12;
                    frame.restore(c, S0, 0xd0);
                    frame.restore(c, S1, 0xc0);
                    frame.restore(c, S2, 0xb0);
                    frame.restore(c, S3, 0xa0);
                    frame.restore(c, S4, 0x90);
                    frame.restore(c, S5, 0x80);
                    frame.restore(c, S6, 0x70);
                    frame.restore(c, S7, 0x60);
                    frame.restore(c, FP, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.close(c);
                    return Ok(());
                }
            };
        }
    });
}

/// `memset` (0x416210, through the function table).
pub(crate) const PSH_MEMSET: u32 = 0x0041_6210;

pub(crate) const SND_PARSE_NEXT_TAG: Callee = Callee {
    addr: 0x003c_7388,
    f: ee::s13::SND_ParseNextTag,
};

pub(crate) const SND_PARSE_PT_HEADER: Callee = Callee {
    addr: 0x003b_bda0,
    f: ee::s13::SND_ParsePtHeader,
};

pub(crate) const SND_INIT_PATCH_PARAMS: Callee = Callee {
    addr: 0x003c_7218,
    f: ee::s13::SND_InitPatchParams,
};

pub(crate) const SND_MEM_ALLOC: Callee = Callee {
    addr: 0x003b_5c60,
    f: ee::s13::SND_MemAlloc,
};

pub(crate) const SND_MEM_COPY: Callee = Callee {
    addr: 0x003c_51e8,
    f: ee::s13::SND_MemCopy,
};

/// Frame, clears, the magic check (GSTR goes to `SND_ParsePtHeader`) and
/// the header defaults; returns where the tag loop starts.
pub(crate) fn psh_prologue(c: &mut Cpu, frame: Frame) -> Result<PshStage, crate::call::Unwound> {
    c.cycles += 40;
    Frame::open(c, frame.size);
    frame.save(c, S1, 0xc0);
    frame.save(c, S3, 0xa0);
    frame.save(c, FP, 0x50);
    frame.save(c, S0, 0xd0);
    frame.save(c, S2, 0xb0);
    frame.save(c, S4, 0x90);
    frame.save(c, S5, 0x80);
    frame.save(c, S6, 0x70);
    frame.save(c, S7, 0x60);
    frame.save_ra(c, 0x40);
    c.g[S1][0] = c.g[A1][0]; // src
    c.g[S3][0] = c.g[A2][0]; // hdr
    c.g[FP][0] = c.g[A0][0]; // base
    c.g[S0][0] = c.g[A3][0]; // out
    c.g[S2][0] = c.g[T0][0]; // extra
    c.g[S4][0] = c.g[T1][0]; // isGstr
                             // memset(vals, 0, 0x18); memset(hdr, 0, 4); memset(out, 0, 0x68);
                             // memset(extra, 0, 0x1c). n = 0, flags = 8, ref = 0.
    c.g[A0][0] = psh_addu(c.g[SP][0], PSH_VALS as u64);
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x18;
    call_ptr(c, PSH_MEMSET, 0x003b_b874)?;
    c.g[S6][0] = 0;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x4;
    c.g[S7][0] = 0x8;
    call_ptr(c, PSH_MEMSET, 0x003b_b88c)?;
    c.g[S5][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x68;
    call_ptr(c, PSH_MEMSET, 0x003b_b8a0)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x1c;
    call_ptr(c, PSH_MEMSET, 0x003b_b8b0)?;

    // "PT" is a stream header; "GSTR" (checked as the compiled chain does)
    // is a PT header.
    let src = Addr(r32(c, S1));
    let b0 = src.lbu(c);
    c.g[A0][0] = b0;
    c.g[V0][0] = 0x47;
    let gstr = 'magic: {
        if b0 == b'P' as u64 {
            c.cycles += 4;
            let b1 = src.off(1).lbu(c);
            c.g[V1][0] = b1;
            if b1 == b'T' as u64 {
                break 'magic false;
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x53;
        if b0 != b'G' as u64 {
            break 'magic false;
        }
        for (i, (want, next)) in [(b'S', 0x54u64), (b'T', 0x52), (b'R', 0)]
            .into_iter()
            .enumerate()
        {
            c.cycles += 3;
            let b = src.off(1 + i as u32).lbu(c);
            c.g[V1][0] = b;
            if i < 2 {
                c.g[V0][0] = next;
            } else {
                c.g[A1][0] = c.g[S1][0];
            }
            if b != want as u64 {
                break 'magic false;
            }
        }
        true
    };
    if gstr {
        c.cycles += 7;
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A0][0] = 0;
        SND_PARSE_PT_HEADER.call(c, 0x003b_b910)?;
        c.g[V0][0] = 1;
        if c.g[S4][0] != 0 {
            c.cycles += 2;
            Addr(r32(c, S4)).sb(c, 1);
        }
        return Ok(PshStage::Exit);
    }
    c.cycles += 2;
    if c.g[S4][0] != 0 {
        Addr(r32(c, S4)).sb(c, 0);
    }
    c.cycles += 12;
    c.g[A0][0] = c.g[S0][0];
    SND_INIT_PATCH_PARAMS.call(c, 0x003b_b930)?;
    // Defaults: 22050 Hz, mono, codec 5; the tag reader starts after the
    // magic; extra->f0 = 0.
    c.g[V1][0] = 0x5622;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x5;
    let hdr = Addr(r32(c, S3));
    hdr.off(PSH_HDR_RATE).sh(c, 0x5622);
    hdr.off(PSH_HDR_CHANNELS).sb(c, 1);
    let p = psh_addu(c.g[S1][0], 4);
    c.g[A1][0] = p;
    hdr.off(0x3).sb(c, 5);
    Addr(sp(c)).sw(c, p);
    Addr(r32(c, S2)).sw(c, 0);
    Ok(PshStage::NextTag)
}

/// The compare chain on the tag id (v1) and the store it selects (0x3bb958).
pub(crate) fn psh_store_tag(c: &mut Cpu) -> crate::call::Flow {
    let id = c.g[V1][0];
    c.g[A0][0] = id;
    let rec = Addr(sp(c));
    for (k, &(tag, action)) in PSH_TAGS.iter().enumerate() {
        c.cycles += if k < 2 { 3 } else { 2 };
        // The delay slot loads the next id (the first block's moves a0).
        c.g[V0][0] = match k {
            0 => 0xa0,
            28 => sx((c.g[S6][0] as u32) << 2),
            _ => PSH_TAGS[k + 1].0,
        };
        if id != tag {
            continue;
        }
        match action {
            PshTag::Byte(base, off) => {
                c.cycles += 3;
                let v = rec.off(PSH_REC_VALUE).lbu(c);
                c.g[V0][0] = v;
                Addr(r32(c, base)).off(off).sb(c, v);
            }
            PshTag::Half(base, off) => {
                c.cycles += 3;
                let v = rec.off(PSH_REC_VALUE).lhu(c);
                c.g[V0][0] = v;
                Addr(r32(c, base)).off(off).sh(c, v);
            }
            PshTag::Word(base, off) => {
                c.cycles += 3;
                let v = rec.off(PSH_REC_VALUE).lw(c);
                c.g[V0][0] = v;
                Addr(r32(c, base)).off(off).sw(c, v);
            }
            PshTag::Data(data, size, block) => {
                c.cycles += 10;
                c.g[A0][0] = rec.off(PSH_REC_SIZE).lw(c);
                SND_MEM_ALLOC.call(c, block + 0x8)?;
                let rec = Addr(sp(c));
                c.g[A1][0] = rec.off(PSH_REC_DATA).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = rec.off(PSH_REC_SIZE).lw(c);
                let mem = c.g[V0][0];
                Addr(r32(c, S0)).off(data).sw(c, mem);
                SND_MEM_COPY.call(c, block + 0x1c)?;
                let n = Addr(sp(c)).off(PSH_REC_SIZE).lw(c);
                c.g[V0][0] = n;
                Addr(r32(c, S0)).off(size).sw(c, n);
            }
            PshTag::Ref => {
                c.cycles += 2;
                c.g[S5][0] = rec.off(PSH_REC_DATA).lw(c);
            }
            PshTag::Flags => {
                c.cycles += 2;
                c.g[S7][0] = rec.off(PSH_REC_VALUE).lw(c);
            }
            PshTag::Marker => {
                c.cycles += 7;
                let data = rec.off(PSH_REC_DATA).lw(c);
                c.g[A0][0] = data;
                let entry = psh_addu(c.g[S0][0], c.g[V0][0]);
                c.g[V1][0] = entry;
                let size = rec.off(PSH_REC_SIZE).lw(c);
                c.g[A1][0] = size;
                Addr(entry as u32).off(PSH_OUT_MARKER_DATA).sw(c, data);
                c.g[V0][0] = entry;
                Addr(entry as u32).off(PSH_OUT_MARKER_SIZE).sw(c, size);
                c.g[S6][0] = psh_addu(c.g[S6][0], 1);
            }
        }
        return Ok(());
    }
    Ok(())
}

/// One channel (a1 = ch) of the fix-up loop, 0x3bbcb0..0x3bbd5c (without
/// the counter step): the output offset from the layout table plus the
/// channel's base (zeroed for stereo output of a stereo stream), and the
/// data offset made absolute or relative to `*ref`.
pub(crate) fn psh_channel(c: &mut Cpu) {
    let state = Addr(r32(c, T2));
    let hdr = Addr(r32(c, S3));
    let out = Addr(r32(c, S0));
    c.cycles += 3;
    let layout = state.off(PSH_STATE_OUT_CHANNELS).lbu(c);
    c.g[V1][0] = layout;
    c.g[T0][0] = sx((c.g[A1][0] as u32) << 2);
    if layout == c.g[T5][0] {
        c.cycles += 3;
        let channels = hdr.off(PSH_HDR_CHANNELS).lbu(c);
        c.g[V0][0] = channels;
        if channels == layout {
            Addr(r32(c, A3)).sw(c, 0);
        }
    }
    // out->chanOff[ch] = table[channels - 1][ch] + (u16)vals[ch]
    c.cycles += 14;
    c.g[V0][0] = hdr.off(PSH_HDR_CHANNELS).lbu(c);
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A0][0] = Addr(r32(c, A3)).lhu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.mult(0, V0, V0, T4);
    c.g[V1][0] = psh_addu(c.g[V1][0], c.g[V0][0]);
    c.g[V1][0] = psh_addu(c.g[V1][0], c.g[T6][0]);
    c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
    c.g[V0][0] = psh_addu(c.g[V0][0], c.g[A0][0]);
    let off = c.g[V0][0];
    Addr(r32(c, T1)).sh(c, off);
    let flags = out.off(PSH_OUT_FLAGS).lhu(c);
    c.g[V1][0] = flags;
    c.g[V0][0] = flags & 0x14;
    // Absolute offsets (flags & 0x14, or no flags): offs[ch] += base.
    let absolute = if c.g[V0][0] != 0 {
        c.g[V1][0] = psh_addu(c.g[T3][0], c.g[T0][0]);
        true
    } else {
        c.cycles += 2;
        if flags != 0 {
            c.g[V0][0] = out.off(PSH_OUT_FLAGS).lhu(c);
            false
        } else {
            c.cycles += 1;
            c.g[V1][0] = psh_addu(c.g[T3][0], c.g[T0][0]);
            true
        }
    };
    if absolute {
        c.cycles += 4;
        let slot = Addr(r32(c, V1));
        let v = psh_addu(c.g[FP][0], slot.lw(c));
        c.g[V0][0] = v;
        slot.sw(c, v);
        c.g[V0][0] = out.off(PSH_OUT_FLAGS).lhu(c);
    }
    // Relative to the reference (flags & 0x308, ref set).
    c.cycles += 3;
    c.g[V0][0] &= 0x308;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        if c.g[S5][0] != 0 {
            c.cycles += 2;
            let extra = Addr(r32(c, S2));
            let reference = Addr(r32(c, S5));
            if c.g[A1][0] != 0 {
                c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                c.cycles += 5;
                c.g[A0][0] = extra.off(PSH_EXTRA_OFFS).lw(c);
                c.g[V1][0] = reference.lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[V1][0] = psh_addu(c.g[V1][0], c.g[V0][0]);
                let v = c.g[V1][0];
                Addr(r32(c, A2)).sw(c, v);
            } else {
                c.cycles += 3;
                let v = reference.lw(c);
                c.g[V0][0] = v;
                extra.off(PSH_EXTRA_OFFS).sw(c, v);
            }
            c.cycles += 1;
        }
    }
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
}

// ---------------------------------------------------------------------------
// SND_FadeVolume (0x003b8530)

/// The sound state block and its channel table (`+0x1f0`, 0x8c bytes per
/// channel: `+0x30` volume step, `+0x34` target volume, `+0x38` volume, all
/// `volume << 16`).
pub(crate) const FADE_SND_STATE: u32 = 0x0050_a8e8;

pub(crate) const FADE_CHANNELS: u32 = 0x1f0;

pub(crate) const FADE_CHANNEL_SIZE: u64 = 0x8c;

pub(crate) const FADE_STEP: u32 = 0x30;

pub(crate) const FADE_TARGET: u32 = 0x34;

pub(crate) const FADE_VOLUME: u32 = 0x38;

pub(crate) const FADE_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};

pub(crate) const FADE_NEXT_HANDLE_CHANNEL: Callee = Callee {
    addr: 0x003b_aa00,
    f: ee::s13::SND_NextHandleChannel,
};

/// `int SND_FadeVolume(int handle, int frames, unsigned volume)`
/// (0x003b8530): start a volume fade on every channel of a handle: the
/// target becomes `volume << 16` and the per-frame step `(target - volume)
/// / frames` (at least one frame). Returns the handle's first channel, or
/// the error from `SND_HandleToChannel`.
///
/// Dispatcher-shaped: an unwind at the channel loop's back edge re-enters
/// at 0x3b8588 (registers: s0 frames, s1 target, s2 first channel, s3
/// 0x8c, s4 the state block, v1 the channel; the cursor also at `0(sp)`).
pub fn snd_fade_volume(c: &mut Cpu) {
    const START: u32 = 0x003b_8530;
    const HEAD: u32 = 0x003b_8588;
    let frame = Frame { size: 0x70 };
    let resumed = match c.pc {
        START => false,
        HEAD => true,
        pc => {
            c.jump_out(pc, START, 0x003b_85ec);
            return;
        }
    };
    guest(c, |c| {
        let mut more = true;
        if !resumed {
            c.cycles += 13;
            Frame::open(c, frame.size);
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S0][0] = c.g[A1][0]; // frames
            c.g[S1][0] = c.g[A2][0]; // volume
            FADE_HANDLE_TO_CHANNEL.call(c, 0x003b_8558)?;
            c.g[S2][0] = c.g[V0][0];
            let positive = 0 < c.g[S0][0] as i64;
            c.g[V1][0] = positive as u64;
            if (c.g[S2][0] as i64) < 0 {
                more = false;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 1;
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = 0x51_0000;
                c.g[S4][0] = sx(FADE_SND_STATE);
                if !positive {
                    c.g[S0][0] = 1;
                }
                c.sw(sp(c), u64::MAX); // cursor = -1
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 16);
                c.g[S3][0] = FADE_CHANNEL_SIZE;
                more = fade_next_channel(c)?;
                if more && back_edge_unwinds(c, HEAD) {
                    return Ok(());
                }
            }
        }
        while more {
            // The cursor's channel (v1): target and step.
            c.cycles += 8;
            let table = Addr(r32(c, S4)).off(FADE_CHANNELS).lw(c);
            c.mult(0, V1, V1, S3);
            let ch = sx((table as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = ch;
            let ch = Addr(ch as u32);
            let volume = ch.off(FADE_VOLUME).lw(c);
            let target = c.g[S1][0];
            ch.off(FADE_TARGET).sw(c, target);
            c.g[V0][0] = sx((target as u32).wrapping_sub(volume as u32));
            c.cycles += 3;
            c.div(0, V0, S0);
            c.g[V0][0] = c.lo[0];
            let step = c.g[V0][0];
            ch.off(FADE_STEP).sw(c, step);
            more = fade_next_channel(c)?;
            if more && back_edge_unwinds(c, HEAD) {
                return Ok(());
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SND_NextHandleChannel(first, &cursor)` for [`snd_fade_volume`]: true
/// when there is another channel (its index then in v1).
pub(crate) fn fade_next_channel(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[SP][0];
    FADE_NEXT_HANDLE_CHANNEL.call(c, 0x003b_85c0)?;
    let more = c.g[V0][0] != 0;
    c.g[V1][0] = Addr(sp(c)).lw(c);
    Ok(more)
}

// ---------------------------------------------------------------------------
// SND_ChannelCalcSend (0x003ba740)

/// The channel table pointer (`SND state + 0x1f0`, 0x8c bytes per channel:
/// `+0x24` voice flags, per send slot `+0x60/+0x61` signed send levels and
/// `+0x62` the combined level, `u16` per slot).
pub(crate) const CALC_SEND_CHANNELS: u32 = 0x0050_a8e8 + 0x1f0;

pub(crate) const CALC_SEND_FLAGS: u32 = 0x24;

pub(crate) const SND_GET_FX_PARAMS: Callee = Callee {
    addr: 0x003b_8218,
    f: ee::s13::SND_GetFxParams,
};

/// `void SND_ChannelCalcSend(int slot, int channel)` (0x003ba740): the
/// effect send level of send slot `slot` of a channel: the product of the
/// channel's two signed send bytes for that slot and the effect's own
/// level (`SND_GetFxParams(slot, flags)[2]`), divided by 64, stored as the
/// slot's `s16` send.
pub fn snd_channel_calc_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[V0][0] = 0x8c;
        let frame = Frame::open(c, 0x30);
        c.mult(0, A1, A1, V0); // channel * 0x8c
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S1, 0x10);
        let table = Addr(CALC_SEND_CHANNELS).lw(c);
        c.g[S1][0] = c.g[A0][0]; // slot
        frame.save_ra(c, 0);
        let ch = sx((table as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S0][0] = ch;
        c.g[A1][0] = Addr(ch as u32).off(CALC_SEND_FLAGS).lhu(c);
        SND_GET_FX_PARAMS.call(c, 0x003b_a770)?;
        // The slot's two send bytes (channel + slot).
        let sends = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A2][0] = sends;
        frame.restore_ra(c, 0);
        let b = Addr(sends as u32).off(0x61).lb(c);
        c.g[A1][0] = b;
        let a = Addr(sends as u32).off(0x60).lb(c);
        c.g[V1][0] = a;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
        c.g[A0][0] = Addr(r32(c, V0)).off(2).lb(c); // effect level
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.mult(0, V1, V1, A1);
        frame.restore(c, S1, 0x10);
        c.mult(0, V1, V1, A0);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
        let level = c.g[V1][0];
        Addr(r32(c, S0)).off(0x62).sh(c, level);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// Number of registered handlers (signed byte).
pub(crate) const SND_EVENT_HANDLER_COUNT: u32 = 0x179;

/// The handler function pointers.
pub(crate) const SND_EVENT_HANDLERS: u32 = 0x1c8;

/// Queued event count of an event source (signed byte).
pub(crate) const SND_SOURCE_PENDING: u32 = 0x17;

/// Queued first words (`a[]`) and second words (`b[]`).
pub(crate) const SND_SOURCE_QUEUE_A: u32 = 0x34;

pub(crate) const SND_SOURCE_QUEUE_B: u32 = 0x44;

/// `void SND_FireEvents(Source* s, int arg)` (0x3b5f98): pops the source's
/// queued events from the back; for each one builds a message
/// `{arg, a[n], b[n]}` on the stack and hands it to every registered handler
/// of the sound event table, in order.
pub fn snd_fire_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S6, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        let source = Addr(r32(c, S2));
        let pending = source.off(SND_SOURCE_PENDING).lb(c);
        c.g[V0][0] = pending;
        c.g[V1][0] = source.off(SND_SOURCE_PENDING).lbu(c);
        if pending as i64 > 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x51_0000;
            c.g[S5][0] = sx(r32(c, S2).wrapping_add(SND_SOURCE_QUEUE_A));
            c.g[FP][0] = 0x51_0000;
            c.g[S7][0] = sx(SND_EVENT_TABLE);
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(SND_SOURCE_QUEUE_B));
            loop {
                // Pop one event and build the message at 0(sp).
                c.cycles += 14;
                let n = r32(c, V1).wrapping_sub(1);
                c.g[V1][0] = sx(n);
                let msg = sp(c);
                let arg = c.g[S6][0];
                Addr(msg).sw(c, arg);
                Addr(r32(c, S2)).off(SND_SOURCE_PENDING).sb(c, sx(n));
                let index = ((n << 24) as i32 >> 22) as u32;
                c.g[S1][0] = 0;
                let b_at = r32(c, S4).wrapping_add(index);
                let a_at = r32(c, S5).wrapping_add(index);
                c.g[V1][0] = sx(b_at);
                c.g[A1][0] = Addr(b_at).lw(c);
                c.g[A0][0] = Addr(a_at).lw(c);
                let handlers = Addr(r32(c, S7)).off(SND_EVENT_HANDLER_COUNT).lb(c);
                c.g[V0][0] = handlers;
                let (a, b) = (c.g[A0][0], c.g[A1][0]);
                Addr(msg).off(4).sw(c, a);
                Addr(msg).off(8).sw(c, b);
                if handlers as i64 > 0 {
                    // Call every handler with the message.
                    c.cycles += 4;
                    c.g[S3][0] = sx(r32(c, FP).wrapping_sub(0x5718));
                    c.g[S0][0] = sx(r32(c, S3).wrapping_add(SND_EVENT_HANDLERS));
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    loop {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                        let handler = r32(c, V1);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                        call_ptr(c, handler, 0x003b_6048)?;
                        let count = Addr(r32(c, S3)).off(SND_EVENT_HANDLER_COUNT).lb(c);
                        let more = (c.g[S1][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            break;
                        }
                        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_6038);
                        }
                    }
                }
                c.cycles += 3;
                let source = Addr(r32(c, S2));
                let pending = source.off(SND_SOURCE_PENDING).lb(c);
                c.g[V0][0] = pending;
                c.g[V1][0] = source.off(SND_SOURCE_PENDING).lbu(c);
                if pending as i64 <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5ff0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_GetFxParams

/// `SndFxParams* SND_GetFxParams(int voice, unsigned flags)` (0x3b8218):
/// the 20-byte parameter record of `voice` in the table of the effect the
/// flags select (`flags & 0x71c`: 8, 4 or 0x100), or null for any other
/// effect.
pub fn snd_get_fx_params(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] &= 0x71c;
    let kind = c.g[A1][0];
    c.g[V1][0] = 0x14; // record size
                       // The record of `voice` (a0) in the table at `0x510000 - neg`.
    fn snd_fx_record(c: &mut Cpu, neg: u32) {
        c.mult(0, 3, 4, 3);
        c.g[V0][0] = sx(0x0051_0000u32.wrapping_sub(neg));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    }
    if kind == 0x8 {
        c.cycles += 5;
        snd_fx_record(c, 0x54f8);
    } else {
        c.cycles += 3;
        if (kind as i64) < 9 {
            c.cycles += 2;
            if kind == 0x4 {
                c.cycles += 4;
                snd_fx_record(c, 0x5520);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 3;
            if kind == 0x100 {
                c.cycles += 5;
                snd_fx_record(c, 0x550c);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// SPCH_IsSampleAllowed

/// `int SPCH_IsSampleAllowed(SpchEvent* ev, SpchBank* bank, int choice)`
/// (0x3d8780): whether `choice` may be played for the speech event. The
/// bank header holds the bits per choice (+5), a flags byte (+4: bit 7 =
/// per-slot allow masks present, low nibble = slot count), an extra-bytes
/// count (+6) and the sample count (+0xa); the bit arrays follow the
/// header at `(slots + 2) * 4 + 0xf & 0x3ffc`. With a sample selected
/// (`ev+2 >= 0`) the choice bit of that sample's row must be set; then, if
/// the bank has slot masks, the choice bit of the slot's mask decides.
/// Out-of-range choices are refused.
pub fn spch_is_sample_allowed(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.g[A1][0]; // bank
    c.g[T2][0] = c.g[A2][0]; // choice
    let bank = c.g[T0][0] as u32;
    c.g[A3][0] = Addr(bank).off(0x5).lbu(c);
    let out_of_range = (c.g[A3][0] as i64) < (c.g[T2][0] as i64);
    c.g[A1][0] = 0x1;
    'done: {
        if out_of_range {
            c.cycles += 2;
            c.g[A1][0] = 0;
            break 'done;
        }
        'slots: {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x2).lh(c); // sample
            c.g[A2][0] = u64::MAX;
            if c.g[V1][0] == c.g[A2][0] {
                c.g[A2][0] = Addr(bank).off(0x4).lbu(c);
                break 'slots;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(bank).off(0xa).lhu(c);
            let in_range = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = in_range as u64;
            c.g[A0][0] = 0;
            if in_range {
                // The sample's row of choice bits.
                c.cycles += 14;
                c.mult(0, 2, 7, 3); // v0 = bits per row * sample
                c.g[A0][0] = Addr(bank).off(0x4).lbu(c);
                c.g[T4][0] = 0;
                c.g[V1][0] = c.g[A0][0] & 0xf;
                c.g[A0][0] &= 0x80;
                c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                c.mult(0, 2, 7, 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                c.g[V0][0] &= 0x3ffc;
                c.g[V0][0] = sx(bank.wrapping_add(c.g[V0][0] as u32));
                let masks = c.g[A0][0] != 0;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if masks {
                    // Skip the slot masks: (bits + 7) / 8 + 1 bytes.
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xe));
                    c.g[A0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V1][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(bank).off(0x6).lbu(c);
                c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T4][0] as u32));
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                }
                // Test bit t1 of the bit array at a3.
                c.cycles += 11;
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[T1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[T1][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lbu(c);
                c.g[V1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V1][0] &= 0xff;
                c.g[A0][0] &= c.g[V1][0];
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[A0][0];
            if c.g[A1][0] == 0 {
                break 'done;
            }
            c.cycles += 1;
            c.g[A2][0] = Addr(bank).off(0x4).lbu(c);
        }
        // The slot masks (flags bit 7): the choice bit of slot `flags & 15`.
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0] & 0x80;
        c.g[A2][0] &= 0xf;
        if c.g[V0][0] == 0 {
            break 'done;
        }
        c.cycles += 21;
        c.g[V0][0] = Addr(bank).off(0x5).lbu(c);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2));
        c.g[V1][0] = u64::MAX;
        c.mult(0, 2, 2, 4);
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x7));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[T2][0];
        }
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[V0][0] &= 0x3ffc;
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(bank.wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A1][0] &= 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1).lbu(c);
        c.g[A1][0] &= c.g[V1][0];
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// SPCH_RandomNoRepeat (0x3db5c0)

/// The speech picker's history ring: 32 entries of {u16 value, u16 key} at
/// 0x450b88, the newest at index `gp+0x1768` (0x4a4858).
pub(crate) const SPCH_HISTORY: u32 = 0x0045_0b88;

pub(crate) const SPCH_HISTORY_KEY: u32 = 0x0045_0b8a;

pub(crate) const SPCH_HISTORY_POS: u32 = 0x1768;

pub(crate) const SPCH_RAND_NEXT: Callee = Callee {
    addr: 0x003d_b4d0,
    f: ee::s14::SPCH_RandNext,
};

/// `int SPCH_RandomNoRepeat(int range, int key)` (0x3db5c0): a random
/// value in `[0, range)` (`(SPCH_RandNext() >> 16) * range >> 16`) that
/// avoids the values recently picked for `key`. With `key == -1` the first
/// draw is returned as is. Otherwise up to 32 draws are made; each is
/// looked up among the last `min(range / 2 clamped to 10, 32)` history
/// entries with the same key (newest first): an unseen value is taken at
/// once, else the one seen longest ago wins. The pick is pushed into the
/// history ring (value, key); the return value is the last draw.
///
/// Register roles, as the original keeps them: s4 range, s5 key (masked to
/// 16 bits), s3 how far back to look, s0 tries, s1 done, s2 pick, fp the
/// oldest hit so far; inner loop t0 limit, t1 same-key entries seen, t2
/// draw, t3 steps, t4 hit position, a1 ring index, a2 stop.
pub fn spch_random_no_repeat(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xa0);
        for (reg, off) in [
            (S4, 0x40),
            (S3, 0x30),
            (S5, 0x50),
            (FP, 0x80),
            (S2, 0x20),
            (S1, 0x10),
            (S0, 0x00),
            (RA, 0x90),
            (S7, 0x70),
            (S6, 0x60),
        ] {
            frame.save(c, reg, off);
        }
        c.g[V1][0] = 0xa;
        c.g[S4][0] = c.g[A0][0];
        let range = r32(c, S4);
        // half = range / 2 (rounded toward zero), at most 10.
        let half = ((range.wrapping_add(range >> 31)) as i32 >> 1) as u32;
        c.g[S3][0] = sx(half);
        c.g[V0][0] = ((half as i32) < 11) as u64;
        c.g[S5][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.g[S3][0] = 0xa;
        }
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        SPCH_RAND_NEXT.call(c, 0x003d_b618)?;
        c.g[FP][0] = u64::MAX;
        c.g[V0][0] = sx(r32(c, V0) >> 16);
        c.mult(0, V0, V0, S4);
        c.g[S2][0] = sx(r32(c, V0) >> 16);
        if c.g[S5][0] == u64::MAX {
            c.cycles += 2;
            c.g[V0][0] = c.g[S2][0];
            return spch_random_no_repeat_epilogue(c, frame);
        }

        c.cycles += 3;
        c.g[S5][0] &= 0xffff;
        c.g[S7][0] = u64::MAX;
        c.g[S6][0] = 1;
        loop {
            // One draw.
            c.cycles += 16;
            SPCH_RAND_NEXT.call(c, 0x003d_b648)?;
            c.g[V0][0] = sx(r32(c, V0) >> 16);
            c.g[T0][0] = c.g[S3][0];
            c.mult(0, V0, V0, S4);
            c.g[V1][0] = ((c.g[T0][0] as i64) < 33) as u64;
            c.g[A0][0] = 0x20;
            let gp = r32(c, GP);
            c.g[A1][0] = Addr(gp).off(SPCH_HISTORY_POS).lw(c);
            c.g[T5][0] = sx(r32(c, S0).wrapping_add(1));
            if c.g[V1][0] == 0 {
                c.g[T0][0] = 0x20;
            }
            c.g[T4][0] = u64::MAX;
            c.g[T1][0] = 0;
            c.g[T3][0] = 0;
            c.g[T2][0] = sx(r32(c, V0) >> 16);
            c.g[A2][0] = 0;

            // Search the history ring backwards for this draw.
            loop {
                c.cycles += 3;
                let in_window = (c.g[T1][0] as i64) < (c.g[T0][0] as i64);
                c.g[V0][0] = sx(0x0045_0000);
                if !in_window {
                    c.cycles += 1;
                    c.g[A2][0] = 1;
                    c.cycles += 1;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                } else {
                    c.cycles += 8;
                    let at = r32(c, A1) << 2;
                    c.g[A0][0] = sx(at);
                    c.g[A3][0] = sx(SPCH_HISTORY);
                    c.g[V0][0] = sx(at.wrapping_add(SPCH_HISTORY_KEY));
                    c.g[V1][0] = Addr(at.wrapping_add(SPCH_HISTORY_KEY)).lhu(c);
                    if c.g[V1][0] != c.g[S5][0] {
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = sx(at.wrapping_add(SPCH_HISTORY));
                        c.g[V0][0] = Addr(at.wrapping_add(SPCH_HISTORY)).lhu(c);
                        let same = c.g[V0][0] == c.g[T2][0];
                        c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                        if same {
                            c.cycles += 3;
                            c.g[A2][0] = 1;
                            c.g[T4][0] = sx(r32(c, T1).wrapping_sub(1));
                        }
                        c.cycles += 1;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    }
                }
                c.cycles += 7;
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x20));
                c.g[V1][0] = ((c.g[S7][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = ((c.g[T3][0] as i64) < 33) as u64;
                if c.g[A0][0] == 0 {
                    c.g[A2][0] = c.g[S6][0];
                }
                let again = c.g[A2][0] == 0;
                if c.g[V1][0] == 0 {
                    c.g[A1][0] = c.g[V0][0];
                }
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_b680);
                }
            }

            // Unseen: take it; seen longer ago than the best so far: keep it.
            c.cycles += 2;
            c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[T4][0] as i64)) as u64;
            if c.g[T4][0] == c.g[S7][0] {
                c.cycles += 3;
                c.g[S2][0] = c.g[T2][0];
                c.g[S1][0] = 1;
                c.cycles += 1;
                c.g[S0][0] = c.g[T5][0];
            } else {
                c.cycles += 2;
                c.g[S0][0] = c.g[T5][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[FP][0] = c.g[T4][0];
                    c.g[S2][0] = c.g[T2][0];
                    c.cycles += 1;
                    c.g[S0][0] = c.g[T5][0];
                }
            }
            c.cycles += 4;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 32) as u64;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = c.g[S6][0];
            }
            let more = c.g[S1][0] == 0;
            let gp = r32(c, GP);
            c.g[V0][0] = Addr(gp).off(SPCH_HISTORY_POS).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_b640);
            }
        }

        // Push the pick into the history ring.
        c.cycles += 4;
        let gp = r32(c, GP);
        let pos = Addr(gp).off(SPCH_HISTORY_POS);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < 32) as u64;
        let next = c.g[V0][0];
        pos.sw(c, next);
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            pos.sw(c, 0);
        }
        c.cycles += 9;
        let at = (pos.lw(c) as u32) << 2;
        c.g[V1][0] = sx(at);
        c.g[A1][0] = sx(at.wrapping_add(SPCH_HISTORY));
        c.g[A0][0] = sx(SPCH_HISTORY.wrapping_add(at));
        c.g[V0][0] = c.g[T2][0];
        let (pick, key) = (c.g[S2][0], c.g[S5][0]);
        Addr(SPCH_HISTORY.wrapping_add(at)).sh(c, pick);
        Addr(SPCH_HISTORY.wrapping_add(at)).off(2).sh(c, key);
        spch_random_no_repeat_epilogue(c, frame)
    })
}

/// SPCH_RandomNoRepeat's epilogue: `lq` ra, fp, s7..s0, pop the frame.
pub(crate) fn spch_random_no_repeat_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 12;
    for (reg, off) in [
        (RA, 0x90),
        (FP, 0x80),
        (S7, 0x70),
        (S6, 0x60),
        (S5, 0x50),
        (S4, 0x40),
        (S3, 0x30),
        (S2, 0x20),
        (S1, 0x10),
        (S0, 0x00),
    ] {
        frame.restore(c, reg, off);
    }
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SPCH_RandNext (0x3db4d0)

/// The speech RNG state: six words at 0x450b70 (a 160-bit sum chain plus a
/// counter in word 5).
pub(crate) const SPCH_RNG_BASE: u32 = 0x0045_0000;

pub(crate) const SPCH_RNG: u32 = 0x0045_0b70;

/// `u32 SPCH_RandNext(void)` (0x3db4d0): the speech system's random number
/// generator. Adds word 5 into word 4 and ripples the sum with carries
/// through words 3..0 (each word becomes the running sum), increments the
/// counter (word 5), and when the counter wraps to 0 increments words 4..0
/// in turn until one does not wrap. Returns the new word 0.
pub fn spch_rand_next(c: &mut Cpu) {
    use crate::guest::Addr;
    let w = |i: u32| Addr(SPCH_RNG + 4 * i);
    // 64-bit `addu` / `sltu` on sign-extended words, as the original.
    let add = |a: u64, b: u64| sx((a as u32).wrapping_add(b as u32));
    c.cycles += 9;
    c.g[T2][0] = sx(SPCH_RNG_BASE);
    c.g[V1][0] = sx(SPCH_RNG);
    let w5 = w(5).lw(c);
    let w4 = w(4).lw(c);
    let mut sum = add(w5, w4);
    let mut carry = 0u64;
    if sum < w5 {
        c.cycles += 2;
        carry = 1;
    } else {
        c.cycles += 3;
        if sum < w4 {
            c.cycles += 2;
            carry = 1;
        }
    }
    c.g[T0][0] = sx(SPCH_RNG);

    c.cycles += 24;
    let w0 = w(0).lw(c);
    let w3 = w(3).lw(c);
    let w2 = w(2).lw(c);
    w(4).sw(c, sum);
    sum = add(add(sum, w3), carry);
    let w1 = w(1).lw(c);
    carry = (sum < w3) as u64;
    w(3).sw(c, sum);
    sum = add(add(sum, w2), carry);
    let counter = add(w(5).lw(c), 1);
    carry = (sum < w2) as u64;
    w(2).sw(c, sum);
    sum = add(add(sum, w1), carry);
    w(5).sw(c, counter);
    w(1).sw(c, sum);
    let a0 = add(sum, w0);
    carry = (sum < w1) as u64;
    sum = add(a0, carry);
    w(0).sw(c, sum);
    c.g[V1][0] = counter;
    c.g[A0][0] = a0;
    c.g[A1][0] = w2;
    c.g[A2][0] = w1;
    c.g[T1][0] = carry;

    if counter == 0 {
        // The counter wrapped: increment words 4, 3, 2, 1 while they wrap.
        let mut wrapped = true;
        for i in [4u32, 3, 2, 1] {
            c.cycles += 4;
            let v = add(w(i).lw(c), 1);
            w(i).sw(c, v);
            c.g[V0][0] = v;
            if v != 0 {
                wrapped = false;
                break;
            }
        }
        if wrapped {
            c.cycles += 3;
            sum = add(sum, 1);
            w(0).sw(c, sum);
        }
    }
    c.cycles += 2;
    c.g[A3][0] = sum;
    c.g[V0][0] = sum;
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_UtkParseHeader (0x3cd6f0, ealib/seg_2B4578.c)

/// UTK decoder state: the byte cursor (+0), the bit buffer (+4), the bits
/// in it (+8), the header flag (+0xc), the excitation bit count (+0x10),
/// the fixed-codebook gain (+0x14), the 64 reflection gains (+0x18..), and
/// the two 12-word adaptive histories (+0x114, +0x144) and the 324-word
/// excitation buffer ending at +0x680.
pub(crate) const UTK_CURSOR: u32 = 0x0;

pub(crate) const UTK_BITS: u32 = 0x4;

pub(crate) const UTK_BIT_COUNT: u32 = 0x8;

pub(crate) const UTK_FLAG: u32 = 0xc;

pub(crate) const UTK_EXC_BITS: u32 = 0x10;

pub(crate) const UTK_FCB_GAIN: u32 = 0x14;

pub(crate) const UTK_GAINS: u32 = 0x18;

pub(crate) const UTK_HIST_A: u32 = 0x114;

pub(crate) const UTK_HIST_B: u32 = 0x144;

pub(crate) const UTK_EXCITATION: u32 = 0x174;

pub(crate) const UTK_READ_BITS_2: Callee = Callee {
    addr: 0x003c_d1f8,
    f: ee::s14::SND_UtkReadBits,
};

/// `cvt.s.w` of v0 as an unsigned word: halve, convert, double when the
/// value is "negative" (the original's unsigned-to-float idiom; v1 holds
/// `v0 >> 1`).
pub(crate) fn utk_unsigned_to_float(c: &mut Cpu, reg: usize) {
    c.cycles += 5;
    c.g[V0][0] = (c.g[V0][0] & 1) | c.g[V1][0];
    let f = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
    abi::set_fpr(c, reg, f + f);
}

/// `void SND_UtkParseHeader(UtkHeader* h, uint8_t* data, UtkState* st, int
/// reset)` (0x3cd6f0): start the UTK bit reader at `data` (first byte
/// preloaded, 8 bits). When `reset`, read the frame header: a 1-bit flag,
/// the 4-bit excitation size (stored as 32 - n), the 4-bit codebook gain
/// index (gain = (n + 1) * 8), the 6-bit gain index from which the 64
/// reflection gains grow geometrically by `1.04 + n * 0.001`, and clear the
/// histories and the excitation buffer.
pub fn snd_utk_parse_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 8;
        frame.save(c, S0, 0x10);
        let data = r32(c, A1);
        c.g[V1][0] = sx(data.wrapping_add(1));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        let st = r32(c, S0);
        Addr(st.wrapping_add(UTK_BIT_COUNT)).sw(c, 8);
        let next = c.g[V1][0];
        Addr(st.wrapping_add(UTK_CURSOR)).sw(c, next);
        c.g[V0][0] = Addr(data).lbu(c);
        let first = c.g[V0][0];
        Addr(st.wrapping_add(UTK_BITS)).sw(c, first);
        if c.g[A3][0] != 0 {
            c.cycles += 16;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            UTK_READ_BITS_2.call(c, 0x003c_d728)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            let flag = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(UTK_FLAG)).sw(c, flag);
            UTK_READ_BITS_2.call(c, 0x003c_d738)?;
            c.g[V1][0] = sx(0x20u32.wrapping_sub(r32(c, V0)));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            let exc_bits = c.g[V1][0];
            Addr(r32(c, S0).wrapping_add(UTK_EXC_BITS)).sw(c, exc_bits);
            UTK_READ_BITS_2.call(c, 0x003c_d750)?;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V1][0] = sx(r32(c, V0) >> 1);
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 4;
                abi::set_fpr(c, 1, FpuF32::from_i32(c.g[V0][0] as u32 as i32));
            } else {
                utk_unsigned_to_float(c, 1);
            }
            c.cycles += 9;
            c.g[AT][0] = 0x4100_0000;
            let eight = FpuF32(0x4100_0000);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 6;
            let gain = FpuF32(c.f[1]) * eight;
            abi::set_fpr(c, 0, gain);
            Addr(r32(c, S0).wrapping_add(UTK_FCB_GAIN)).write_fpu(c, gain);
            UTK_READ_BITS_2.call(c, 0x003c_d79c)?;
            if (c.g[V0][0] as i64) < 0 {
                c.g[V1][0] = sx(r32(c, V0) >> 1);
                utk_unsigned_to_float(c, 2);
            } else {
                c.cycles += 4;
                abi::set_fpr(c, 2, FpuF32::from_i32(c.g[V0][0] as u32 as i32));
            }
            c.cycles += 14;
            let step = FpuF32(0x3a83_126f);
            let st = r32(c, S0);
            c.g[V1][0] = sx(st.wrapping_add(UTK_GAINS));
            c.g[AT][0] = 0x3f85_1eb8;
            c.g[A3][0] = sx(st.wrapping_add(UTK_HIST_A));
            let scaled = FpuF32(c.f[2]) * step;
            c.g[A1][0] = sx(st.wrapping_add(UTK_HIST_B));
            c.g[A0][0] = sx(st.wrapping_add(UTK_EXCITATION));
            c.g[V0][0] = 0x3e;
            let ratio = scaled + FpuF32(0x3f85_1eb8);
            abi::set_fpr(c, 0, scaled);
            abi::set_fpr(c, 1, ratio);
            // Gains: each the previous one times the ratio.
            loop {
                c.cycles += 6;
                let at = r32(c, V1);
                let g = Addr(at.wrapping_sub(4)).read_fpu(c) * ratio;
                abi::set_fpr(c, 0, g);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                Addr(at).write_fpu(c, g);
                c.g[V1][0] = sx(at.wrapping_add(4));
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d800);
                }
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = 0xb;
            loop {
                c.cycles += 7;
                Addr(r32(c, V1)).sw(c, 0);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                Addr(r32(c, A1)).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d820);
                }
            }
            c.cycles += 3;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x50c));
            c.g[V0][0] = 0x143;
            loop {
                c.cycles += 7;
                Addr(r32(c, A0)).sw(c, 0);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d848);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_NextVoiceGroup (0x3c6158)
// ---------------------------------------------------------------------------

/// The voice-group table `D_0050A8E8`: `u8 count` at +0x4c, `u16 masks[]` at
/// +0x4e (one capability mask per voice group).
pub(crate) const SND_VOICE_GROUPS: u32 = 0x0050_a8e8;

pub(crate) const SND_VOICE_GROUP_COUNT: u32 = 0x4c;

pub(crate) const SND_VOICE_GROUP_MASKS: u32 = 0x4e;

/// The request's wanted-capability mask (`e->f18`) and its byte at +3
/// (`e->b3`: 1 allows groups with the 0x80 bit).
pub(crate) const SND_REQ_FLAGS: u32 = 0x18;

pub(crate) const SND_REQ_B3: u32 = 0x3;

/// Loop head (back-edge event check).
pub(crate) const SND_NEXT_VOICE_GROUP_LOOP: u32 = 0x003c_6188;

/// `int SND_NextVoiceGroup(int* idx, Request* e)` (0x3c6158): walks the
/// voice-group table from `*idx`, returning the first group's mask that
/// matches the request (the 0x71c type bits, then the 0xe0 bits; groups with
/// 0x80 only when `e->b3 == 1`) and advancing `*idx` past it; 0 when the
/// table is exhausted. Register-faithful: v0/v1/a0/a2/a3/t0..t3 as the
/// original leaves them.
pub fn snd_next_voice_group(c: &mut Cpu) {
    c.cycles += 8;
    c.g[T0][0] = c.g[A0][0];
    c.g[A0][0] = sx(SND_VOICE_GROUPS);
    let mut idx_ptr = Addr(c.g[T0][0] as u32);
    let mut v1 = idx_ptr.lw(c);
    let count = Addr(SND_VOICE_GROUPS + SND_VOICE_GROUP_COUNT).lbu(c);
    let mut a2: u64 = 0;
    if (v1 as i64) < (count as i64) {
        c.cycles += 4;
        c.g[T2][0] = sx(SND_VOICE_GROUPS);
        c.g[T3][0] = 1;
        c.g[T1][0] = sx(SND_VOICE_GROUPS + SND_VOICE_GROUP_MASKS);
        let mut v0 = sx((v1 as u32) << 1);
        let mut a0: u64;
        let mut a3: u64;
        let mut masks = c.g[T1][0] as u32;
        let mut table = c.g[T2][0] as u32;
        let mut one = c.g[T3][0];
        let mut req = c.g[A1][0] as u32;
        loop {
            // Type bits (0x71c).
            c.cycles += 7;
            let flags = Addr(req.wrapping_add(SND_REQ_FLAGS)).lhu(c);
            v0 = sx((v0 as u32).wrapping_add(masks));
            let group = Addr(v0 as u32).lhu(c);
            a3 = flags;
            v1 = flags & 0x71c;
            a0 = group & 0x71c;
            let mut accepted = true;
            if v1 == 0 {
                c.cycles += 2;
                a2 = a0;
                v1 = idx_ptr.lw(c);
            } else {
                c.cycles += 4;
                v0 = a3 & 0xffff;
                a2 = v0 & a0;
                if a2 != 0 {
                    v1 = idx_ptr.lw(c);
                } else {
                    c.cycles += 2;
                    a3 = idx_ptr.lw(c);
                    accepted = false;
                }
            }
            if accepted {
                // Second-stage bits (0xe0).
                c.cycles += 7;
                a0 = a3 & 0xe0;
                v0 = sx(((v1 as u32) << 1).wrapping_add(masks));
                a3 = v1;
                v1 = Addr(v0 as u32).lhu(c) & 0xe0;
                if a0 == 0 {
                    c.cycles += 1;
                    a2 |= v1;
                } else {
                    c.cycles += 4;
                    v0 = Addr(req.wrapping_add(SND_REQ_FLAGS)).lhu(c) & v1;
                    a2 |= v0;
                    if v0 == 0 {
                        c.cycles += 2;
                        a2 = 0;
                        accepted = false;
                    }
                }
            }
            if accepted {
                // Groups with 0x80 only for b3 == 1.
                c.cycles += 3;
                let b3 = Addr(req.wrapping_add(SND_REQ_B3)).lb(c);
                v0 = a2 & 0x80;
                if b3 != one {
                    c.cycles += 2;
                    if v0 != 0 {
                        a2 = 0;
                        accepted = false;
                    }
                }
            }
            if accepted {
                c.cycles += 2;
                v0 = sx((a3 as u32).wrapping_add(1));
                if a2 != 0 {
                    // Found: *idx = idx + 1, return the mask.
                    c.cycles += 2;
                    idx_ptr.sw(c, v0);
                    break;
                }
            }
            // Next group.
            c.cycles += 6;
            v1 = sx((a3 as u32).wrapping_add(1));
            idx_ptr.sw(c, v1);
            v0 = Addr(table.wrapping_add(SND_VOICE_GROUP_COUNT)).lbu(c);
            let more = (v1 as i64) < (v0 as i64);
            v0 = sx((v1 as u32) << 1);
            if !more {
                break;
            }
            if event_due(c) {
                c.g[V0][0] = v0;
                c.g[V1][0] = v1;
                c.g[A0][0] = a0;
                c.g[A2][0] = a2;
                c.g[A3][0] = a3;
                deliver_event_here(c, SND_NEXT_VOICE_GROUP_LOOP);
                v0 = c.g[V0][0];
                idx_ptr = Addr(c.g[T0][0] as u32);
                masks = c.g[T1][0] as u32;
                table = c.g[T2][0] as u32;
                one = c.g[T3][0];
                req = c.g[A1][0] as u32;
            }
        }
        c.g[A0][0] = a0;
        c.g[A3][0] = a3;
    }
    c.cycles += 2;
    c.g[V1][0] = v1;
    c.g[A2][0] = a2;
    c.g[V0][0] = a2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_ReleaseChunkCallback (0x3b6ad8)
// ---------------------------------------------------------------------------

/// The sound streams' stream-file handles, indexed by stream number
/// (`D_0050A0A0`: pointers to `StreamFile*`).
pub(crate) const SNDSTRM_FILES: u32 = 0x0050_a0a0;

/// `StreamFile_ReleaseChunk(file, chunk)` (cross-shard, through the table).
pub(crate) const SNDSTRM_RELEASE_CHUNK_2: u32 = 0x003e_13e8;

/// `void SNDSTRM_ReleaseChunkCallback(void* data)` (0x3b6ad8): the chunk
/// header sits in the word before the data; its first byte is the stream
/// number. Releases the chunk to that stream's file
/// (`StreamFile_ReleaseChunk(*files[*chunk], chunk)`).
pub fn sndstrm_release_chunk_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let chunk = Addr(r32(c, A0).wrapping_sub(4)).lw(c);
        c.g[A1][0] = chunk;
        let stream = Addr(chunk as u32).lbu(c);
        let slot = sx(((stream as u32) << 2).wrapping_add(SNDSTRM_FILES));
        let handle = Addr(slot as u32).lw(c);
        // Registers at the call.
        c.g[V0][0] = slot;
        c.g[V1][0] = handle;
        c.g[A0][0] = Addr(handle as u32).lw(c);
        call_ptr(c, SNDSTRM_RELEASE_CHUNK_2, 0x003b_6b04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_Play (0x3b6820)
// ---------------------------------------------------------------------------

/// The loaded banks (`D_0050AADC`: 16-byte entries, the bank pointer
/// first); a bank's patch count is the `u16` at +6.
pub(crate) const SND_PLAY_BANKS: u32 = 0x0050_aadc;

pub(crate) const SND_PLAY_BANK_PATCHES: u32 = 0x6;

/// SND error: bad bank or patch index.
pub(crate) const SND_PLAY_EBADARG: u64 = 0xffff_ffff_ffff_fff8;

pub(crate) const SND_PLAY_VALIDATE: Callee = Callee {
    addr: 0x003b_64d8,
    f: ee::s13::SND_BankValidate,
};

pub(crate) const SND_PLAY_GET_PATCH: Callee = Callee {
    addr: 0x003c_6540,
    f: ee::s13::SND_BankGetPatch,
};

pub(crate) const SND_PLAY_PLAY_PATCH: Callee = Callee {
    addr: 0x003b_b588,
    f: ee::s13::SND_PlayPatch,
};

/// `int SND_Play(int bank, int patch, PlayParams* params)` (0x3b6820):
/// validates the bank and the patch index (-8 when bad), then plays the
/// bank's patch (`SND_PlayPatch(bank, SND_BankGetPatch(bank, patch), bank
/// index, patch index, params)`) and returns its handle.
pub fn snd_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        SND_PLAY_VALIDATE.call(c, 0x003b_6848)?;
        let valid = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = 0x0051_0000;
        let mut ok = false;
        if valid {
            c.cycles += 5;
            let banks = Addr(SND_PLAY_BANKS).lw(c);
            c.g[A0][0] = banks;
            let entry = sx((r32(c, S2) << 4).wrapping_add(banks as u32));
            c.g[V1][0] = entry;
            let bank = Addr(entry as u32).lw(c);
            c.g[S0][0] = bank;
            if (c.g[S1][0] as i64) >= 0 {
                c.cycles += 4;
                let count = Addr((bank as u32).wrapping_add(SND_PLAY_BANK_PATCHES)).lhu(c);
                let inside = (c.g[S1][0] as i64) < (count as i64);
                c.g[V0][0] = inside as u64;
                c.g[A0][0] = bank;
                ok = inside;
            }
        }
        if ok {
            c.cycles += 8;
            c.g[A1][0] = c.g[S1][0];
            SND_PLAY_GET_PATCH.call(c, 0x003b_6884)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S3][0];
            SND_PLAY_PLAY_PATCH.call(c, 0x003b_689c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = SND_PLAY_EBADARG;
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

pub(crate) mod spch_hist_f {

    /// Flags: low nibble = extra word count, bit 7 = a bitmap follows.
    pub const FLAGS: u32 = 0x4;

    /// Width (bits per bitmap row).
    pub const WIDTH: u32 = 0x5;

    /// History length (ring size).
    pub const COUNT: u32 = 0x6;
}

/// `int SPCH_HistoryFind(SpchHistory* h, int key)` (0x3d6e30): look `key`
/// up in the speech history ring, newest first; returns its age + 1, or 0
/// when it is not there. The ring (a write-position byte, then `count`
/// entries) lies past the header tables, whose size follows from the
/// flags/width bytes. A leaf.
pub fn spch_history_find(c: &mut Cpu) {
    use spch_hist_f::*;
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    let hist = r32(c, A2);
    c.g[T0][0] = Addr(hist.wrapping_add(COUNT)).lbu(c);
    c.g[T3][0] = 0; // result: not found
    if c.g[T0][0] as i64 > 0 {
        c.cycles += 13;
        c.g[A0][0] = Addr(hist.wrapping_add(FLAGS)).lbu(c);
        c.g[T1][0] = 0; // bitmap bytes
        c.g[A3][0] = Addr(hist.wrapping_add(WIDTH)).lbu(c);
        c.g[V1][0] = c.g[A0][0] & 0xf;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(2));
        c.mult(0, V0, A3, V0);
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xf)) & 0x3ffc;
        c.g[V0][0] = sx(hist.wrapping_add(r32(c, V0)));
        c.g[A0][0] &= 0x80;
        c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        if c.g[A0][0] != 0 {
            // A bitmap of (width + 7) / 8 bytes plus one.
            c.cycles += 7;
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(7));
            c.g[V1][0] = ((-1i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V0][0] = if c.g[V1][0] != 0 {
                c.g[A0][0]
            } else {
                sx(r32(c, A3).wrapping_add(0xe))
            };
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
            c.g[T1][0] = sx(r32(c, V0).wrapping_add(1));
        }
        c.cycles += 8;
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, T1)));
        c.g[A3][0] = 0; // age
        c.g[A2][0] = Addr(r32(c, A0)).lbu(c); // write position
        c.g[T1][0] = sx(r32(c, T0).wrapping_sub(1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1)); // entries
        c.g[T2][0] = u64::MAX;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        loop {
            c.cycles += 6;
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T1][0]; // wrap to the last entry
            }
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
            c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
            if c.g[V0][0] == c.g[A1][0] {
                c.cycles += 2;
                c.g[T3][0] = sx(r32(c, A3).wrapping_add(1));
                break;
            }
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_6eb0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T3][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x003c72a0 SND_RandomRange

/// `int SND_RandomRange(int range)` (0x3c72a0): a random value in about
/// `[-range, range)`: `range` is clamped to `0..=0x10000`, then
/// `((SND_Random() & 0x7fff) - 0x4000) * range >> 14`.
pub fn snd_random_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let range = c.g[S0][0] as i64;
        c.g[V0][0] = (0x10000 < range) as u64;
        if range > 0x10000 {
            c.cycles += 2;
            c.g[S0][0] = 0x10000;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (-1 < range) as u64;
            if range <= -1 {
                c.g[S0][0] = 0;
            }
        }
        c.cycles += 10;
        Callee {
            addr: 0x003b_c5c8,
            f: ee::s13::SND_Random,
        }
        .call(c, 0x003c_72d8)?;
        c.g[V0][0] &= 0x7fff;
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4000));
        c.mult(0, V0, V0, S0);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 14) as u32);
        frame.close(c);
        Ok(())
    })
}

/// The last handle serial handed out (a byte, 0x44c588 = 0x450000 - 0x3a78).
pub(crate) const SND_SERIAL_HI: u32 = 0x0045_0000;

pub(crate) const SND_SERIAL_LO: u32 = 0x3a78;

/// The sound voice table (0x50a8e8): voice count (i16) at +0x17e, voice
/// array at +0x1f0, 0x8c bytes per voice; a voice's in-use flag (i8) at
/// +0x69 and its handle serial (u8) at +0x26.
pub(crate) const SND_VOICES: u32 = 0x0051_0000 - 0x5718;

pub(crate) const SND_VOICE_COUNT: u32 = 0x17e;

pub(crate) const SND_VOICE_ARRAY: u32 = 0x1f0;

pub(crate) const SND_VOICE_SIZE: u32 = 0x8c;

pub(crate) const SND_VOICE_IN_USE: u32 = 0x69;

pub(crate) const SND_VOICE_SERIAL: u32 = 0x26;

/// `u8 SND_NextHandleSerial(void)` (0x3bb0c8): advances the 8-bit handle
/// serial (skipping 0) until no voice in use carries it, and returns it.
/// Both loops' back edges check for events (the inner loop at 0x3bb110, the
/// retry at 0x3bb0cc); the loop state lives in registers throughout.
pub fn snd_next_handle_serial(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = sx(SND_SERIAL_HI);
    'retry: loop {
        // Bump the serial; 0 is skipped.
        c.cycles += 6;
        let serial = Addr(r32(c, V0).wrapping_sub(SND_SERIAL_LO));
        c.g[V1][0] = serial.lbu(c);
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[V1][0] = c.g[A0][0] & 0xff;
        serial.sb(c, c.g[A0][0]);
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            serial.sb(c, c.g[V0][0]);
        }
        // Scan the voices for one in use with this serial.
        c.cycles += 5;
        c.g[V0][0] = sx(SND_VOICES);
        c.g[A1][0] = Addr(SND_VOICES).off(SND_VOICE_COUNT).lh(c);
        let none = (c.g[A1][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[A2][0] = c.g[A1][0];
            c.g[V1][0] = Addr(SND_VOICES).off(SND_VOICE_ARRAY).lw(c);
            c.g[A1][0] = Addr(r32(c, A3).wrapping_sub(SND_SERIAL_LO)).lbu(c);
            loop {
                c.cycles += 3;
                let voice = Addr(r32(c, V1));
                c.g[V0][0] = voice.off(SND_VOICE_IN_USE).lb(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = voice.off(SND_VOICE_SERIAL).lbu(c);
                    let taken = c.g[V0][0] == c.g[A1][0];
                    c.g[V0][0] = sx(SND_SERIAL_HI);
                    if taken {
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_b0cc);
                        }
                        continue 'retry;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(SND_VOICE_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_b110);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A3).wrapping_sub(SND_SERIAL_LO)).lbu(c);
        ret(c);
        return;
    }
}

/// `SNDSTRM` stream: file handle at +0, request id counter at +0xc,
/// queued requests list at +0x11c, free request list at +0x128 (its count
/// at +0x130). A request node: data at +0x8, id | flags at +0xc, user
/// word at +0x20 (0x28 bytes, cleared on reuse).
pub(crate) const SNDSTRM_QUEUE_HANDLE: u32 = 0x0;

pub(crate) const SNDSTRM_QUEUE_NEXT_ID: u32 = 0xc;

pub(crate) const SNDSTRM_QUEUE_QUEUED: u32 = 0x11c;

pub(crate) const SNDSTRM_QUEUE_FREE: u32 = 0x128;

pub(crate) const SNDSTRM_QUEUE_FREE_COUNT: u32 = 0x130;

pub(crate) const SNDSTRM_QUEUE_REQ_DATA: u32 = 0x8;

pub(crate) const SNDSTRM_QUEUE_REQ_ID: u32 = 0xc;

pub(crate) const SNDSTRM_QUEUE_REQ_USER: u32 = 0x20;

pub(crate) const SNDSTRM_QUEUE_REQ_SIZE: u64 = 0x28;

/// The stream-file owner tag `'lECS'` passed to the file queue.
pub(crate) const SNDSTRM_QUEUE_TAG: u64 = 0x6c45_4353;

/// `StreamFile_QueueRange`, `StreamFile_QueueFile`, `memset`.
pub(crate) const SNDSTRM_QUEUE_RANGE: u32 = 0x003e_0c88;

pub(crate) const SNDSTRM_QUEUE_FILE: u32 = 0x003e_0d80;

pub(crate) const SNDSTRM_QUEUE_MEMSET: u32 = 0x0041_6210;

/// `int SNDSTRM_QueueRequest(int stream, int user, int a2, int a3, int
/// mode)` (0x3b76e8): queue a request on sound stream `stream`. `mode` 0
/// queues the file range (a2, a3), mode 1 the whole file a2, any other
/// mode takes a3 as the already-queued file request. Takes a free request
/// node, links it into the queued list with the next request id (steps of
/// 0x100, back to 0 past 2^31, or'ed with `stream`) and returns that id;
/// -8 for no stream, -13 for no free node, -1 when the file queue failed.
pub fn sndstrm_queue_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S1, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S5, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A1][0];
        Callee {
            addr: 0x003b_6a38,
            f: ee::s13::SNDSTRM_GetStream,
        }
        .call(c, 0x003b_7720)?;
        c.g[S0][0] = c.g[V0][0];
        'done: {
            c.g[V0][0] = (-8i64) as u64;
            if c.g[S0][0] == 0 {
                break 'done;
            }
            c.cycles += 3;
            let free = Addr((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_FREE_COUNT)).lw(c);
            c.g[V0][0] = (-13i64) as u64;
            if free == 0 {
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[S1][0] == 0 {
                c.cycles += 8;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_HANDLE)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A3][0] = SNDSTRM_QUEUE_TAG;
                call_ptr(c, SNDSTRM_QUEUE_RANGE, 0x003b_7758)?;
                c.g[S2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S3][0];
                if c.g[S1][0] == c.g[V0][0] {
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_HANDLE)).lw(c);
                    c.g[A2][0] = 0;
                    c.g[A3][0] = SNDSTRM_QUEUE_TAG;
                    call_ptr(c, SNDSTRM_QUEUE_FILE, 0x003b_777c)?;
                    c.g[S2][0] = c.g[V0][0];
                }
            }
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            if c.g[S2][0] == 0 {
                break 'done;
            }

            // Take a free node, clear it, queue it.
            c.cycles += 19;
            Callee {
                addr: 0x003b_58a0,
                f: ee::s13::SND_Lock,
            }
            .call(c, 0x003b_7790)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_FREE));
            Callee {
                addr: 0x003c_62e8,
                f: ee::s13::SND_ListPopFront,
            }
            .call(c, 0x003b_7798)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = SNDSTRM_QUEUE_REQ_SIZE;
            call_ptr(c, SNDSTRM_QUEUE_MEMSET, 0x003b_77ac)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_QUEUED));
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x003c_62b8,
                f: ee::s13::SND_ListPushBack,
            }
            .call(c, 0x003b_77b8)?;
            let stream = c.g[S0][0] as u32;
            let req = c.g[S1][0] as u32;
            let data = c.g[S2][0];
            Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_DATA)).sw(c, data);
            let next = Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).lw(c);
            let next = sx((next as u32).wrapping_add(0x100));
            c.g[V0][0] = next;
            Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).sw(c, next);
            let next = Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).lw(c);
            c.g[V1][0] = next;
            if (next as i64) < 0 {
                Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).sw(c, 0);
            }

            c.cycles += 8;
            let id = Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).lw(c) | c.g[S4][0];
            c.g[V0][0] = id;
            Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_ID)).sw(c, id);
            let user = c.g[S5][0];
            Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_USER)).sw(c, user);
            c.g[S0][0] = Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_ID)).lw(c);
            Callee {
                addr: 0x003b_58d8,
                f: ee::s13::SND_Unlock,
            }
            .call(c, 0x003b_77f0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
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

pub(crate) mod voice_start_callee {
    use super::*;

    /// `SND_MemAlloc` (other shard: through the function table).
    pub const MEM_ALLOC: u32 = 0x003b_5c60;

    pub const DSP_INSERT_SORTED: Callee = Callee {
        addr: 0x003c_8f48,
        f: ee::s14::SND_DspInsertSorted,
    };

    pub const DSP_STRETCH_INIT: Callee = Callee {
        addr: 0x003c_b280,
        f: ee::s14::SND_DspStretch_Init,
    };
}

// ---------------------------------------------------------------------------
// 0x003c8468 SND_EeVoiceStop

/// The EE voice mixer (0x515b40): channel count (byte +5), the per-channel
/// and the master level sums, the voice table pointer.
pub(crate) const SEVS_MIXER: u32 = 0x0051_5b40;

pub(crate) const SEVS_CHANNELS: u32 = 0x5;

pub(crate) const SEVS_CHANNEL_SUMS: u32 = 0xe8;

pub(crate) const SEVS_MASTER_SUM: u32 = 0x100;

pub(crate) const SEVS_VOICES: u32 = 0x1dc;

/// A voice (0x60 bytes): active byte, channel levels, master level,
/// scale, node list.
pub(crate) const SEVS_VOICE_SIZE: u32 = 0x60;

pub(crate) const SEVS_LEVELS: u32 = 0x4;

pub(crate) const SEVS_MASTER_LEVEL: u32 = 0x34;

pub(crate) const SEVS_SCALE: u32 = 0x3c;

pub(crate) const SEVS_NODES: u32 = 0x40;

/// `void SND_EeVoiceStop(int voice)` (0x3c8468): take the voice's scaled
/// master level and channel levels back out of the mixer sums (adding the
/// stored negated contributions), run and free each of its nodes (`node->
/// stop(node)` when set, `SND_MemFree`), then mark the voice inactive.
pub fn snd_ee_voice_stop(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        c.cycles += 14;
        Frame::open(c, 0x30);
        c.g[V1][0] = SEVS_VOICE_SIZE as u64;
        c.g[V0][0] = 0x0051_0000;
        frame.save(c, S1, 0x10);
        c.mult(0, A0, A0, V1);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = SEVS_MIXER as u64;
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr(SEVS_MIXER).off(SEVS_VOICES).lw(c);
        c.g[V0][0] = (SEVS_MIXER + SEVS_MASTER_SUM) as u64;
        c.g[S1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        let voice = r32(c, S1);
        c.g[V1][0] = sx(voice.wrapping_add(SEVS_MASTER_LEVEL));
        // The master sum (a one-element loop: a1 0 -> -1).
        c.cycles += 10;
        c.lwc1(0, r32(c, V1));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        c.lwc1(2, voice.wrapping_add(SEVS_SCALE));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        c.lwc1(1, r32(c, V0));
        c.mul_s(0, 0, 2);
        c.add_s(1, 1, 0);
        c.swc1(1, r32(c, V0));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        // The channel sums.
        c.cycles += 4;
        c.g[V1][0] = sx(SEVS_MIXER);
        c.g[V0][0] = Addr(SEVS_MIXER).off(SEVS_CHANNELS).lbu(c);
        c.g[A1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A2][0] = c.g[V1][0];
            c.g[A0][0] = sx(voice.wrapping_add(SEVS_LEVELS));
            c.g[V1][0] = sx(SEVS_MIXER + SEVS_CHANNEL_SUMS);
            loop {
                c.cycles += 12;
                c.lwc1(1, r32(c, A0));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.lwc1(2, voice.wrapping_add(SEVS_SCALE));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                c.lwc1(0, r32(c, V1));
                c.mul_s(1, 1, 2);
                c.add_s(0, 0, 1);
                c.swc1(0, r32(c, V1));
                c.g[V0][0] = Addr(r32(c, A2)).off(SEVS_CHANNELS).lbu(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_84e8);
                }
            }
        }
        // Stop and free the nodes.
        loop {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, S1)).off(SEVS_NODES).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(4).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                let stop = r32(c, V0);
                call_ptr(c, stop, 0x003c_8530)?;
            }
            c.g[V0][0] = Addr(r32(c, S1)).off(SEVS_NODES).lw(c);
            c.cycles += 5;
            c.g[S0][0] = Addr(r32(c, V0)).off(8).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003b_5e98, 0x003c_8540)?; // SND_MemFree
            let next = c.g[S0][0];
            Addr(r32(c, S1)).off(SEVS_NODES).sw(c, next);
            if next == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_8518);
            }
        }
        c.cycles += 6;
        c.sb(r32(c, S1), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003bc570 SND_MidiToVolume

/// The MIDI-to-volume table (0x44c590, 128 halfwords).
pub(crate) const SMTV_TABLE: u32 = 0x0044_c590;

/// `u16 SND_MidiToVolume(int midi)` (0x3bc570): the volume for a MIDI
/// level, `table[midi]` (no range check).
pub fn snd_midi_to_volume(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = sx(SMTV_TABLE);
    let at = (r32(c, A0) << 1).wrapping_add(SMTV_TABLE);
    c.g[A0][0] = sx(at);
    c.g[V0][0] = c.lhu(at);
    ret(c);
}

/// The sound driver's state block (`D_00515B40`): the voice entries
/// (`+0x1dc`, 0x60 bytes each), and per sample format (6 rows of 3) the
/// DSP start functions (`+0x18`) and object sizes (`+0x60`).
pub(crate) const VOICE_STATE: u32 = 0x0051_5b40;

pub(crate) const VOICE_STATE_ENTRIES: u32 = 0x1dc;

pub(crate) const VOICE_STATE_START_FNS: u32 = 0x18;

pub(crate) const VOICE_STATE_SIZES: u32 = 0x60;

/// The global flag byte passed in the start parameters (`D_0050AA62`).
pub(crate) const VOICE_START_FLAG: u32 = 0x0050_aa62;

/// Voice entry fields.
pub(crate) const VOICE_ACTIVE: u32 = 0x0;

pub(crate) const VOICE_DSP_LIST: u32 = 0x40;

pub(crate) const VOICE_RESULT: u32 = 0x44;

pub(crate) const VOICE_OBJ: u32 = 0x48;

pub(crate) const VOICE_STREAM: u32 = 0x4c;

/// `void SND_EeVoiceStart(int id, int fmt, int ch, int a, int b, int strm,
/// int c, int g, int d, int f, ..., int e, int prio)` (0x3c8158): reset voice
/// `id`'s DSP chain; pick the start function for the sample format (8, 10,
/// 4, 22, 16, 23 -> rows 0..5) and channel mode (mono: 0/1 by `f >= 1`;
/// stereo 2; other 0), and if there is one allocate its DSP object
/// (`SND_MemAlloc(size)`, priority 0xf0 / `prio`), call it with the start
/// parameters on the stack and insert it into the chain; with a stream,
/// allocate a time-stretch object (0x1828 bytes, priority 0xc8), initialise
/// it for the stream (voice -1 unless mono-0) and insert it too. Marks the
/// voice active.
pub fn snd_ee_voice_start(c: &mut Cpu) {
    use voice_start_callee as f;
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save(c, S2, 0xa0);
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A0][0]; // id
        c.g[V1][0] = 0x60;
        c.mult(0, V1, S1, V1); // id * sizeof(entry)
        c.g[T5][0] = 0x51_0000;
        c.g[T4][0] = 1;
        c.g[S3][0] = c.g[A2][0]; // ch
        c.g[S6][0] = c.g[A3][0];
        c.g[S7][0] = c.g[T0][0];
        c.g[FP][0] = c.g[T1][0]; // strm
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        let entries = Addr(VOICE_STATE + VOICE_STATE_ENTRIES).lw(c);
        c.g[S0][0] = sx((entries as u32).wrapping_add(c.g[V1][0] as u32));
        let ent = r32(c, S0);
        Addr(ent + VOICE_DSP_LIST).sw(c, 0);
        Addr(ent + VOICE_OBJ).sw(c, 0);
        Addr(ent + 0x50).sw(c, 0);
        Addr(ent + 0x58).sw(c, 0);
        Addr(ent + 0x5c).sw(c, 0);
        Addr(ent + VOICE_STREAM).sw(c, 0);
        Addr(sp(c) + 0x18).sw(c, 1); // p.one
        Addr(sp(c) + 0x28).sw(c, 0); // p.result

        // The format row (a0; -1 for an unknown format).
        let fmt = c.g[A1][0];
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = 8;
        if fmt == 8 {
            c.cycles += 2;
            c.g[A0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            if fmt == 0xa {
                c.cycles += 2;
                c.g[A0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x16;
                if fmt == 4 {
                    c.cycles += 2;
                    c.g[A0][0] = 2;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x10;
                    if fmt == 0x16 {
                        c.cycles += 2;
                        c.g[A0][0] = 3;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = fmt ^ 0x17;
                        if fmt == 0x10 {
                            c.cycles += 2;
                            c.g[A0][0] = 4;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 5;
                            if c.g[V1][0] == 0 {
                                c.g[A0][0] = 5;
                            }
                        }
                    }
                }
            }
        }

        // The channel column (v1).
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[S3][0] == 1 {
            c.cycles += 5;
            let f_arg = Addr(sp(c) + 0xd8).lw(c);
            c.g[V0][0] = ((f_arg as i64) < 1) as u64;
            c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { c.g[S3][0] };
        } else {
            c.cycles += 1;
            c.g[V1][0] = if c.g[S3][0] != 0 { 0 } else { 2 };
        }

        // The format's start function.
        c.cycles += 10;
        c.g[V0][0] = 0xc;
        c.g[A1][0] = sx(VOICE_STATE);
        c.mult(0, V0, A0, V0); // row * 12
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = sx(VOICE_STATE + VOICE_STATE_START_FNS);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let start_fn = Addr(r32(c, S2)).lw(c);
        c.g[V0][0] = sx(VOICE_STATE.wrapping_add(c.g[V1][0] as u32));
        if start_fn != 0 {
            c.cycles += 34;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(VOICE_STATE_SIZES)).lw(c);
            call_ptr(c, f::MEM_ALLOC, 0x003c_8294)?;
            let ent = r32(c, S0);
            Addr(ent + VOICE_OBJ).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xf0;
            c.g[V0][0] = 0x51_0000;
            let p = sp(c);
            Addr(p).sw(c, c.g[S6][0]); // p.a
            c.g[V1][0] = Addr(VOICE_START_FLAG).lbu(c);
            c.g[A1][0] = c.g[SP][0]; // &p
            c.g[V0][0] = Addr(p + 0xf0).lw(c);
            Addr(p + 0x4).sw(c, c.g[S7][0]); // p.b
            Addr(p + 0x24).sw(c, c.g[V0][0]); // p.e
            c.g[V0][0] = Addr(p + 0xd0).lw(c);
            Addr(p + 0x8).sw(c, c.g[S4][0]); // p.c
            Addr(p + 0x10).sw(c, c.g[V0][0]); // p.d
            c.g[V0][0] = Addr(p + 0xd8).lw(c);
            Addr(p + 0xc).sw(c, c.g[S5][0]); // p.g
            Addr(p + 0x14).sw(c, c.g[V0][0]); // p.f
            Addr(p + 0x20).sw(c, c.g[V1][0]); // p.flag
            Addr(p + 0x1c).sw(c, c.g[S1][0]); // p.id
            c.g[V0][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(r32(c, V0) + 0x4).sw(c, 0);
            c.g[V1][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(r32(c, V1) + 0x18).sh(c, 0xf0);
            c.g[V1][0] = Addr(p + 0xf8).lw(c); // prio
            c.g[V0][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(r32(c, V0) + 0x1a).sb(c, c.g[V1][0]);
            c.g[V1][0] = Addr(r32(c, S2)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = Addr(ent + VOICE_OBJ).lw(c);
            call_ptr(c, target, 0x003c_8300)?;
            let ent = r32(c, S0);
            c.g[V0][0] = Addr(sp(c) + 0x28).lw(c);
            c.g[A0][0] = sx(ent + VOICE_DSP_LIST);
            c.g[A1][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(ent + VOICE_RESULT).sw(c, c.g[V0][0]);
            f::DSP_INSERT_SORTED.call(c, 0x003c_8314)?;
        }

        // The stream's time-stretch stage.
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        if c.g[FP][0] != 0 {
            c.cycles += 18;
            c.g[A0][0] = 0x1828;
            if c.g[S3][0] != 0 {
                c.g[S1][0] = c.g[V0][0]; // voice -1
            }
            call_ptr(c, f::MEM_ALLOC, 0x003c_8328)?;
            let ent = r32(c, S0);
            Addr(ent + VOICE_STREAM).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc8;
            Addr(r32(c, V0) + 0x4).sw(c, 0);
            c.g[A1][0] = c.g[FP][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[V0][0] = Addr(ent + VOICE_STREAM).lw(c);
            Addr(r32(c, V0) + 0x18).sh(c, 0xc8);
            c.g[V0][0] = Addr(sp(c) + 0xf8).lw(c); // prio
            c.g[V1][0] = Addr(ent + VOICE_STREAM).lw(c);
            Addr(r32(c, V1) + 0x1a).sb(c, c.g[V0][0]);
            c.g[A0][0] = Addr(ent + VOICE_STREAM).lw(c);
            f::DSP_STRETCH_INIT.call(c, 0x003c_8358)?;
            let ent = r32(c, S0);
            c.g[A1][0] = Addr(ent + VOICE_STREAM).lw(c);
            c.g[A0][0] = sx(ent + VOICE_DSP_LIST);
            f::DSP_INSERT_SORTED.call(c, 0x003c_8364)?;
        }

        c.cycles += 14;
        c.g[V0][0] = 1;
        Addr(r32(c, S0) + VOICE_ACTIVE).sb(c, 1);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkSetInput (0x3cdde8).

/// UTK decoder state: `+0xd44` bit position, `+0xd48` the input, `+0xd54`
/// the input's size (non-zero: busy), `+0xd58` the user word, `+0xd5c`
/// the header reset flag, `+0xd60` "starts with the 0xee marker", `+0xd64`
/// the stream format (1: marker byte first).
pub(crate) const UTK_BIT_POS: u32 = 0xd44;

pub(crate) const UTK_INPUT: u32 = 0xd48;

pub(crate) const UTK_INPUT_SIZE: u32 = 0xd54;

pub(crate) const UTK_USER: u32 = 0xd58;

pub(crate) const UTK_RESET: u32 = 0xd5c;

pub(crate) const UTK_MARKED: u32 = 0xd60;

pub(crate) const UTK_FORMAT: u32 = 0xd64;

pub(crate) const UTK_PARSE_HEADER: Callee = Callee {
    addr: 0x003c_d6f0,
    f: ee::s14::SND_UtkParseHeader,
};

/// `int SND_UtkSetInput(UtkState* st, u8* data, int user, int size)`
/// (0x3cdde8): attach a new input buffer to an idle decoder (-1 if `data`
/// is null or an input is still attached), note whether a format-1 stream
/// starts with the 0xee marker (skipped), and parse its header. Returns 0.
pub fn snd_utk_set_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = u64::MAX;
        'out: {
            if c.g[A1][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            let st = Addr(r32(c, A0));
            c.g[V0][0] = st.off(UTK_INPUT_SIZE).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if busy {
                break 'out;
            }
            c.cycles += 7;
            c.g[T0][0] = st.off(UTK_FORMAT).lw(c);
            c.g[V0][0] = 1;
            st.off(UTK_INPUT_SIZE).sw(c, c.g[A3][0]);
            st.off(UTK_USER).sw(c, c.g[A2][0]);
            st.off(UTK_BIT_POS).sw(c, 0);
            st.off(UTK_INPUT).sw(c, c.g[A1][0]);
            if c.g[T0][0] == 1 {
                c.cycles += 4;
                let data = r32(c, A1);
                c.g[V1][0] = Addr(data).lbu(c);
                c.g[V0][0] = 0xee;
                if c.g[V1][0] != 0xee {
                    st.off(UTK_MARKED).sw(c, 0);
                } else {
                    c.cycles += 1;
                    st.off(UTK_MARKED).sw(c, c.g[T0][0]);
                }
                c.cycles += 6;
                c.g[A3][0] = st.off(UTK_RESET).lw(c);
                c.g[A1][0] = sx(data.wrapping_add(1));
                c.g[A2][0] = c.g[A0][0];
                UTK_PARSE_HEADER.call(c, 0x003c_de44)?;
            } else {
                c.cycles += 4;
                c.g[A3][0] = st.off(UTK_RESET).lw(c);
                c.g[A2][0] = c.g[A0][0];
                UTK_PARSE_HEADER.call(c, 0x003c_de58)?;
            }
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetPitch (0x3c8a88)

/// The sound driver's state block (`D_00515B40`).
pub(crate) const PITCH_SND_STATE: u32 = 0x0051_5b40;

/// State -> the voice entries (0x60 bytes each).
pub(crate) const PITCH_STATE_VOICES: u32 = 0x1dc;

/// State -> output sample rate code (>= 50: high-quality resampling).
pub(crate) const PITCH_STATE_RATE_CODE: u32 = 0xe;

/// Resampler interpolation setting byte (`D_0050AA62`).
pub(crate) const PITCH_RESAMPLE_MODE: u32 = 0x0050_aa62;

/// Bytes per voice entry.
pub(crate) const PITCH_VOICE_SIZE: u64 = 0x60;

/// Voice -> its DSP chain (sorted by priority).
pub(crate) const PITCH_VOICE_DSP_CHAIN: u32 = 0x40;

/// Voice -> its resampler DSP (0x38 bytes; priority half +0x18).
pub(crate) const PITCH_VOICE_RESAMPLER: u32 = 0x50;

/// Highest pitch ratio (2.0 in 16.16).
pub(crate) const PITCH_MAX: u64 = 0x2_0000;

pub(crate) const PITCH_RESAMPLE_INIT: Callee = Callee {
    addr: 0x003c_bb28,
    f: ee::s14::SND_DspResample_Init,
};

pub(crate) const PITCH_DSP_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};

pub(crate) const PITCH_RESAMPLE_SET_RATE: Callee = Callee {
    addr: 0x003c_b538,
    f: ee::s14::SND_DspResample_SetRate,
};

/// `void SND_EeVoiceSetPitch(int voice, int pitch)` (0x3c8a88): sets the
/// voice's resampling rate to `min(pitch, 0x20000)`, first creating its
/// resampler DSP if it has none (`SND_MemAlloc(0x38)`, priority 0xa0,
/// `SND_DspResample_Init(dsp, mode, rate code >= 50)`, inserted into the
/// voice's DSP chain). Frame 0x40 (s0 = voice entry, s1 = rate, s2 =
/// driver state); the entry offset uses `mult`.
pub fn snd_ee_voice_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = PITCH_VOICE_SIZE;
        frame.save(c, S0, 0x30);
        c.mult(0, A0, A0, V0);
        frame.save(c, S1, 0x20);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S2, 0x10);
        c.g[V0][0] = PITCH_MAX;
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(PITCH_SND_STATE);
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = Addr(PITCH_SND_STATE + PITCH_STATE_VOICES).lw(c);
        c.g[V0][0] = ((PITCH_MAX as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[A1][0];
        }
        c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let voice = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(voice.wrapping_add(PITCH_VOICE_RESAMPLER)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[A0][0] = c.g[V0][0];
        } else {
            // Create the resampler and insert it into the DSP chain.
            c.cycles += 20;
            c.g[A0][0] = 0x38;
            call_ptr(c, 0x003b_5c60, 0x003c_8ad8)?; // SND_MemAlloc
            let voice = Addr(c.g[S0][0] as u32);
            let dsp = c.g[V0][0];
            voice.off(PITCH_VOICE_RESAMPLER).sw(c, dsp);
            c.g[A0][0] = 0xa0;
            c.g[A3][0] = 0x51_0000;
            let state = c.g[S2][0] as u32;
            c.g[A2][0] = Addr(state.wrapping_add(PITCH_STATE_RATE_CODE)).lbu(c);
            Addr((dsp as u32).wrapping_add(4)).sw(c, 0);
            c.g[A2][0] = (c.g[A2][0] < 0x32) as u64;
            c.g[V1][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            c.g[A2][0] ^= 1;
            let prio = c.g[A0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sh(c, prio);
            c.g[V0][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(0x1a)).sb(c, 0);
            c.g[A1][0] = Addr(PITCH_RESAMPLE_MODE).lbu(c);
            c.g[A0][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            PITCH_RESAMPLE_INIT.call(c, 0x003c_8b10)?;
            let voice = Addr(c.g[S0][0] as u32);
            c.g[A1][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            c.g[A0][0] = sx(voice.0.wrapping_add(PITCH_VOICE_DSP_CHAIN));
            PITCH_DSP_INSERT_SORTED.call(c, 0x003c_8b1c)?;
            let voice = Addr(c.g[S0][0] as u32);
            c.g[A0][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        PITCH_RESAMPLE_SET_RATE.call(c, 0x003c_8b28)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod sndstrm_rate {

    /// +0 (u16) sample rate, +2 (u8) channel count, +3 (u8) codec type.
    pub const HDR_RATE: u32 = 0x0;

    pub const HDR_CHANNELS: u32 = 0x2;

    pub const HDR_CODEC: u32 = 0x3;
}

/// `int SNDSTRM_CalcDataRate(const StreamHeader* h)` (0x3b6948): the bytes
/// per second a sound stream consumes: `rate * channels * k >> 8` with `k`
/// the codec's bytes per 256 samples (5: 0x92, 10: 0x88, 4: 0x33, 22: 0x66,
/// 8: 0x200, anything else 0); codecs 16 and 23 return `channels * 8`.
/// LO/HI keep the last multiply (`rate * channels`, then `* k`).
pub fn sndstrm_calc_data_rate(c: &mut Cpu) {
    use sndstrm_rate as k;
    c.cycles += 7;
    let h = Addr(abi::arg(c, 0));
    c.g[A1][0] = h.off(k::HDR_CHANNELS).lbu(c);
    c.g[A2][0] = 0;
    c.g[V1][0] = h.off(k::HDR_CODEC).lbu(c);
    c.g[A0][0] = h.off(k::HDR_RATE).lhu(c);
    c.mult(0, A0, A0, A1);
    let codec = c.g[V1][0];
    // the compare chain: bytes per 256 samples, or None for `channels * 8`
    let per_256 = match codec {
        5 => {
            c.g[V0][0] = 0x5;
            c.cycles += 2;
            Some(0x92)
        }
        10 => {
            c.g[V0][0] = 0xa;
            c.cycles += 3 + 2;
            Some(0x88)
        }
        4 => {
            c.g[V0][0] = 0x4;
            c.cycles += 3 + 2 + 2;
            Some(0x33)
        }
        0x16 => {
            c.g[V0][0] = 0x16;
            c.cycles += 3 + 2 + 2 + 2;
            Some(0x66)
        }
        8 => {
            c.g[V0][0] = 0x8;
            c.cycles += 3 + 2 + 2 + 2 + 2;
            Some(0x200)
        }
        0x10 => {
            c.cycles += 3 + 2 + 2 + 2 + 2;
            None
        }
        0x17 => {
            c.cycles += 3 + 2 + 2 + 2 + 2 + 2;
            None
        }
        _ => {
            c.cycles += 3 + 2 + 2 + 2 + 2 + 2;
            Some(0)
        }
    };
    match per_256 {
        Some(bytes) => {
            c.g[A2][0] = bytes;
            c.cycles += 3;
            c.mult(0, V0, A0, A2);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
        }
        None => {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
        }
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetLowpass (0x3c8d00)

/// The EE voice table pointer (`SND mixer 0x515b40 + 0x1dc`).
pub(crate) const LOWPASS_VOICES: u32 = 0x0051_5d1c;

/// A voice entry is 0x60 bytes: DSP chain head at +0x40, its lowpass DSP
/// node at +0x58.
pub(crate) const LOWPASS_VOICE_SIZE: u64 = 0x60;

pub(crate) const LOWPASS_VOICE_DSP_CHAIN: u32 = 0x40;

pub(crate) const LOWPASS_VOICE_FILTER: u32 = 0x58;

/// The SND driver state and its output sample rate (u16 at +0x2a).
pub(crate) const LOWPASS_SND_STATE: u32 = 0x0050_a8e8;

pub(crate) const LOWPASS_STATE_RATE: u32 = 0x2a;

pub(crate) const LOWPASS_INIT: Callee = Callee {
    addr: 0x003c_a870,
    f: ee::s14::SND_DspLowpass_Init,
};

pub(crate) const LOWPASS_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};

pub(crate) const LOWPASS_SET_PARAMS: Callee = Callee {
    addr: 0x003c_a8a0,
    f: ee::s14::SND_DspLowpass_SetParams,
};

pub(crate) const LOWPASS_DSP_REMOVE: Callee = Callee {
    addr: 0x003c_8fb0,
    f: ee::s14::SND_DspRemove,
};

/// `void SND_EeVoiceSetLowpass(int voice, float cutoff)` (0x3c8d00): a
/// cutoff below 1.0 (a fraction of the output rate) gives the voice a
/// lowpass DSP (created on first use: `SND_MemAlloc(0x28)`, priority 0x28,
/// `SND_DspLowpass_Init`, inserted into the voice's DSP chain) and sets it
/// to `round(cutoff * rate) << 7` over `rate << 8` (gain 0x100); a cutoff
/// of 1.0 or more removes and frees the voice's lowpass DSP, if any.
/// Frame 0x40 (s0 = voice entry, f20 = cutoff); the entry offset uses
/// `mult`.
pub fn snd_ee_voice_set_lowpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = LOWPASS_VOICE_SIZE;
        c.mult(0, A0, A0, V0);
        frame.save_fpr(c, 20, 0x30);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0;
        c.f[20] = c.f[12];
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = Addr(LOWPASS_VOICES).lw(c);
        let below_one = FpuF32(c.f[20]).lt(FpuF32::ONE);
        abi::set_fcc(c, below_one);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        if below_one {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[V0][0] = Addr(voice.wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
            c.g[V1][0] = 0x51_0000;
            if c.g[V0][0] == 0 {
                // Create the lowpass DSP and insert it into the chain.
                c.cycles += 15;
                c.g[A0][0] = 0x28;
                call_ptr(c, 0x003b_5c60, 0x003c_8d4c)?; // SND_MemAlloc
                let filter = Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER));
                let dsp = c.g[V0][0];
                filter.sw(c, dsp);
                c.g[A0][0] = 0x28;
                Addr((dsp as u32).wrapping_add(4)).sw(c, 0);
                c.g[V0][0] = filter.lw(c);
                Addr(r32(c, V0).wrapping_add(0x18)).sh(c, 0x28);
                c.g[V1][0] = filter.lw(c);
                Addr(r32(c, V1).wrapping_add(0x1a)).sb(c, 0);
                c.g[A0][0] = filter.lw(c);
                LOWPASS_INIT.call(c, 0x003c_8d70)?;
                let voice = r32(c, S0);
                c.g[A1][0] = Addr(voice.wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
                c.g[A0][0] = sx(voice.wrapping_add(LOWPASS_VOICE_DSP_CHAIN));
                LOWPASS_INSERT_SORTED.call(c, 0x003c_8d7c)?;
                c.g[V1][0] = 0x51_0000;
            }
            // params = { round(cutoff * rate) << 7, rate << 8, 0x100 }.
            c.cycles += 14;
            c.f[1] = 0;
            c.g[V1][0] = sx(LOWPASS_SND_STATE);
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = 0x3f00_0000;
            let rate = Addr(LOWPASS_SND_STATE + LOWPASS_STATE_RATE).lhu(c);
            c.g[A0][0] = rate;
            let rate_f = FpuF32::from_i32(rate as i32);
            c.f[0] = rate_f.0;
            c.g[V0][0] = sx((rate as u32) << 8);
            let scaled = FpuF32(c.f[20]) * rate_f;
            c.f[12] = scaled.0;
            let negative = scaled.lt(FpuF32::ZERO);
            abi::set_fcc(c, negative);
            let params = sp(c);
            let rate_word = c.g[V0][0];
            Addr(params + 4).sw(c, rate_word);
            if negative {
                c.cycles += 2;
                c.g[AT][0] = 0xffff_ffff_bf00_0000;
                c.f[2] = 0xbf00_0000;
            }
            c.cycles += 12;
            let rounded = FpuF32(c.f[2]) + scaled;
            c.f[2] = rounded.0;
            c.f[4] = rounded.to_i32() as u32;
            let cut = sx(c.f[4] << 7);
            c.g[V0][0] = cut;
            c.g[V1][0] = 0x100;
            Addr(params).sw(c, cut);
            c.g[A1][0] = sx(params);
            Addr(params + 8).sw(c, 0x100);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
            LOWPASS_SET_PARAMS.call(c, 0x003c_8de8)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[A1][0] = Addr(voice.wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
            if c.g[A1][0] != 0 {
                // Remove the lowpass DSP from the chain and free it.
                c.cycles += 6;
                c.g[A0][0] = sx(voice.wrapping_add(LOWPASS_VOICE_DSP_CHAIN));
                LOWPASS_DSP_REMOVE.call(c, 0x003c_8e04)?;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
                call_ptr(c, 0x003b_5e98, 0x003c_8e0c)?; // SND_MemFree
                Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER)).sw(c, 0);
            }
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyTimeStretch (0x3c3010)

/// The SND driver state: the channel table pointer (+0x1f0) and the two
/// voice-number bases (bytes +0x2f, +0x30) subtracted from a channel's
/// voice numbers.
pub(crate) const STRETCH_SND_STATE: u32 = 0x0050_a8e8;

pub(crate) const STRETCH_STATE_CHANNELS: u32 = 0x1f0;

pub(crate) const STRETCH_STATE_BASE_A: u32 = 0x2f;

pub(crate) const STRETCH_STATE_BASE_B: u32 = 0x30;

/// A channel (0x8c bytes): voice numbers (`s16`) from +4, voice count
/// (byte +0x23), flags (u16 +0x24; bit 2: EE voices).
pub(crate) const STRETCH_CHANNEL_SIZE: u64 = 0x8c;

pub(crate) const STRETCH_CHANNEL_VOICES: u32 = 0x4;

pub(crate) const STRETCH_CHANNEL_COUNT: u32 = 0x23;

pub(crate) const STRETCH_CHANNEL_FLAGS: u32 = 0x24;

/// `int SND_ChannelApplyTimeStretch(int channel, int ratio)` (0x3c3010):
/// for a channel of EE voices (flag 4), sets the time-stretch ratio of each
/// of its voices (`SND_EeVoiceSetTimeStretch(voice - (base_a + base_b),
/// ratio)`); returns 0. Frame 0x60 (s0 = voice cursor, s1 = index, s2 =
/// channel, s3 = driver state, s4 = ratio); the channel offset uses `mult`.
pub fn snd_channel_apply_time_stretch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = STRETCH_CHANNEL_SIZE;
        frame.save(c, S2, 0x30);
        c.mult(0, A0, A0, V0);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[V1][0] = sx(STRETCH_SND_STATE);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(STRETCH_SND_STATE + STRETCH_STATE_CHANNELS).lw(c);
        c.g[S2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        let channel = r32(c, S2);
        c.g[V0][0] = Addr(channel.wrapping_add(STRETCH_CHANNEL_FLAGS)).lhu(c) & 4;
        c.g[S4][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(channel.wrapping_add(STRETCH_CHANNEL_COUNT)).lbu(c);
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx(channel.wrapping_add(STRETCH_CHANNEL_VOICES));
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(STRETCH_STATE_BASE_A)).lbu(c);
                loop {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S4][0];
                    let state = r32(c, S3);
                    let base_b = Addr(state.wrapping_add(STRETCH_STATE_BASE_B)).lbu(c);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    let cursor = r32(c, S0);
                    c.g[V1][0] = Addr(cursor).lh(c);
                    let base = (base_b as u32).wrapping_add(r32(c, V0));
                    c.g[S0][0] = sx(cursor.wrapping_add(2));
                    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(base));
                    call_ptr(c, 0x003c_8b40, 0x003c_3090)?; // SND_EeVoiceSetTimeStretch
                    let count = Addr(r32(c, S2).wrapping_add(STRETCH_CHANNEL_COUNT)).lbu(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (count as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(STRETCH_STATE_BASE_A)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3070);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_RetireRequest (0x3b6a70)

/// The sound-stream table (`0x50a0a0`, one pointer per stream index).
pub(crate) const SNDSTRM_RETIRE_REQUEST_STREAMS: u32 = 0x0050_a0a0;

/// Stream: the active request list (+0x11c), the free list (+0x128) and the
/// request being played (+0x134).
pub(crate) const SNDSTRM_RETIRE_REQUEST_ACTIVE: u32 = 0x11c;

pub(crate) const SNDSTRM_RETIRE_REQUEST_FREE: u32 = 0x128;

pub(crate) const SNDSTRM_RETIRE_REQUEST_PLAYING: u32 = 0x134;

/// `void SNDSTRM_RetireRequest(int handle)` (0x3b6a70): finds the request
/// for `handle` on its stream (stream index = low byte), moves it from the
/// active list to the front of the free list, and clears the stream's
/// playing request if it was that one.
pub fn sndstrm_retire_request(c: &mut Cpu) {
    const FIND_REQUEST: Callee = Callee {
        addr: 0x003b_7a70,
        f: ee::s13::SNDSTRM_FindRequest,
    };
    const LIST_REMOVE: Callee = Callee {
        addr: 0x003c_6320,
        f: ee::s13::SND_ListRemove,
    };
    const LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x003c_6288,
        f: ee::s13::SND_ListPushFront,
    };
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        let slot = sx((c.g[A0][0] as u32 & 0xff) << 2);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(SNDSTRM_RETIRE_REQUEST_STREAMS);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx((slot as u32).wrapping_add(SNDSTRM_RETIRE_REQUEST_STREAMS));
        c.g[S1][0] = Addr(r32(c, V0)).lw(c); // stream
        FIND_REQUEST.call(c, 0x003b_6a9c)?;
        c.g[S0][0] = c.g[V0][0]; // request
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(SNDSTRM_RETIRE_REQUEST_ACTIVE));
        c.g[A1][0] = c.g[S0][0];
        LIST_REMOVE.call(c, 0x003b_6aac)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(SNDSTRM_RETIRE_REQUEST_FREE));
        c.g[A1][0] = c.g[S0][0];
        LIST_PUSH_FRONT.call(c, 0x003b_6ab8)?;
        let playing = Addr(r32(c, S1)).off(SNDSTRM_RETIRE_REQUEST_PLAYING);
        c.g[V0][0] = playing.lw(c);
        if c.g[V0][0] == c.g[S0][0] {
            playing.sw(c, 0);
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
// 0x003c9420 SND_DecUtk_InitMem

/// The UTK decoder object: callbacks (+0, +4), sample count (+0x1c), a
/// cleared word (+0x20), mode (+0x24), the UTK state (+0x28).
pub(crate) const DEC_UTK_COUNT: u32 = 0x1c;

pub(crate) const DEC_UTK_MODE: u32 = 0x24;

pub(crate) const DEC_UTK_STATE: u32 = 0x28;

/// The configuration: input (+0), count (+0xc), mode (+0x24), and the
/// callback slot the decoder fills (+0x28).
pub(crate) const DEC_UTK_CFG_COUNT: u32 = 0xc;

/// `void SND_DecUtk_InitMem(DecUtk* self, DecUtkCfg* cfg)` (0x3c9420): set
/// up a UTK (MicroTalk) decoder in memory: its callbacks, count and mode
/// from `cfg`; allocate (`SND_UtkAlloc(0xd68)`) and reset the UTK state,
/// set its "running" flag, mode 0 for decoder modes below 3 else 1, and
/// point it at the input (`SND_UtkSetInput(state, cfg->input, count * 4,
/// count)`).
pub fn snd_dec_utk_init_mem(c: &mut Cpu) {
    guest(c, |c| {
        const ALLOC: Callee = Callee {
            addr: 0x003c_d1a8,
            f: ee::s14::SND_UtkAlloc,
        };
        const RESET: Callee = Callee {
            addr: 0x003c_ddb8,
            f: ee::s14::SND_UtkReset,
        };
        const GET_STATE: Callee = Callee {
            addr: 0x003c_e078,
            f: ee::s14::SND_UtkGetState,
        };
        const SET_STATE: Callee = Callee {
            addr: 0x003c_e0b8,
            f: ee::s14::SND_UtkSetState,
        };
        const SET_MODE: Callee = Callee {
            addr: 0x003c_e0d8,
            f: ee::s14::SND_UtkSetMode,
        };
        const SET_INPUT: Callee = Callee {
            addr: 0x003c_dde8,
            f: ee::s14::SND_UtkSetInput,
        };
        c.cycles += 44;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let this = c.g[S0][0] as u32;
        let cfg = c.g[S1][0] as u32;
        c.g[V0][0] = 0x003c_9360; // decode callback
        c.sw(this, c.g[V0][0]);
        c.g[A1][0] = 0x003c_9358; // the cfg's callback
        c.g[V1][0] = 0x003c_93f8; // second callback
        c.g[A2][0] = c.lw(cfg.wrapping_add(DEC_UTK_CFG_COUNT));
        c.sw(this.wrapping_add(0x20), 0);
        c.g[A0][0] = 0xd68;
        c.sw(this.wrapping_add(DEC_UTK_COUNT), c.g[A2][0]);
        c.g[V0][0] = c.lw(cfg.wrapping_add(0x24));
        c.sw(this.wrapping_add(DEC_UTK_MODE), c.g[V0][0]);
        c.sw(cfg.wrapping_add(0x28), c.g[A1][0]);
        c.sw(this.wrapping_add(0x4), c.g[V1][0]);
        ALLOC.call(c, 0x003c_9478)?;
        c.g[A0][0] = c.g[V0][0];
        RESET.call(c, 0x003c_9480)?;
        // state = GetState(utk); state.running = 1; SetState(utk, &state, 0).
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
        c.sw(this.wrapping_add(DEC_UTK_STATE), c.g[V0][0]);
        GET_STATE.call(c, 0x003c_9490)?;
        let s = sp(c);
        c.g[A0][0] = c.lw(this.wrapping_add(DEC_UTK_STATE));
        c.g[V0][0] = 0x1;
        c.ldl(3, s.wrapping_add(0x17));
        c.ldr(3, s.wrapping_add(0x10));
        c.g[A1][0] = c.lw(s.wrapping_add(0x18));
        c.sdl(3, s.wrapping_add(0x7));
        c.sdr(3, s);
        c.sw(s.wrapping_add(0x8), c.g[A1][0]);
        c.sw(s.wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0;
        SET_STATE.call(c, 0x003c_94c0)?;
        c.g[V0][0] = c.lw(this.wrapping_add(DEC_UTK_MODE));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
        c.g[A0][0] = c.lw(this.wrapping_add(DEC_UTK_STATE));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = 0;
            SET_MODE.call(c, 0x003c_94d8)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x1;
            SET_MODE.call(c, 0x003c_94e8)?;
        }
        c.g[A2][0] = c.lw(this.wrapping_add(DEC_UTK_COUNT));
        c.cycles += 10;
        c.g[A0][0] = c.lw(this.wrapping_add(DEC_UTK_STATE));
        c.g[A1][0] = c.lw(cfg);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        SET_INPUT.call(c, 0x003c_9500)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_InitPatchParams (0x3c7218)

/// Patch parameter block fields (`SND_InitPatchParams`).
pub(crate) const SND_PATCH_FLAGS: u32 = 0x0;

pub(crate) const SND_PATCH_B2: u32 = 0x2;

pub(crate) const SND_PATCH_VOLUME: u32 = 0x3;

pub(crate) const SND_PATCH_PAN: u32 = 0x4;

pub(crate) const SND_PATCH_RANGE: u32 = 0x8;

/// Six halfwords at +0x0c and six words at +0x18.
pub(crate) const SND_PATCH_HALVES: u32 = 0xc;

pub(crate) const SND_PATCH_WORDS: u32 = 0x18;

/// Four words at +0x48 and four at +0x58.
pub(crate) const SND_PATCH_A48: u32 = 0x48;

pub(crate) const SND_PATCH_A58: u32 = 0x58;

/// `int SND_InitPatchParams(Patch* p)` (0x3c7218): reset a sound patch's
/// parameter block to defaults: volume 0x7f, pan 0x40 (centre), range 8,
/// everything else (flags, bytes 2/5/6/7, the six halfword/word modulation
/// slots at +0x0c/+0x18 and the two word arrays at +0x48/+0x58) zero.
/// Returns 0. Leaf; both clearing loops check events at their back edges.
pub fn snd_init_patch_params(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A1][0] = c.g[A0][0];
    let patch = Addr(c.g[A1][0] as u32);
    c.g[V0][0] = 0x7f;
    c.g[V1][0] = 0x40;
    c.g[A0][0] = 0x8;
    patch.off(SND_PATCH_VOLUME).sb(c, 0x7f);
    patch.off(SND_PATCH_PAN).sb(c, 0x40);
    patch.off(SND_PATCH_RANGE).sh(c, 0x8);
    patch.off(SND_PATCH_B2).sb(c, 0);
    patch.off(SND_PATCH_FLAGS).sh(c, 0);
    for b in 5..8 {
        patch.off(b).sb(c, 0);
    }

    // a48[i] = a58[i] = 0 for i in 0..4 (v0 walks a58, v1 counts down).
    c.g[V0][0] = sx(patch.off(SND_PATCH_A58).0);
    c.g[V1][0] = 0x3;
    loop {
        c.cycles += 7;
        let at = c.g[V0][0] as u32;
        Addr(at.wrapping_sub(SND_PATCH_A58 - SND_PATCH_A48)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        Addr(at).sw(c, 0);
        c.g[V0][0] = sx(at.wrapping_add(4));
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_7250);
        }
    }

    // sC[i] = 0 (halfwords), i18[i] = 0 (words) for i in 0..6.
    c.cycles += 3;
    let patch = Addr(c.g[A1][0] as u32);
    c.g[V0][0] = sx(patch.off(SND_PATCH_HALVES).0);
    c.g[A0][0] = sx(patch.off(SND_PATCH_WORDS).0);
    c.g[V1][0] = 0x5;
    loop {
        c.cycles += 7;
        let half = c.g[V0][0] as u32;
        let word = c.g[A0][0] as u32;
        Addr(half).sh(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        Addr(word).sw(c, 0);
        c.g[V0][0] = sx(half.wrapping_add(2));
        c.g[A0][0] = sx(word.wrapping_add(4));
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_7278);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// SPCH_CheckEventMasks (0x3d9ed0)

/// Speech sentence header: byte 6 (length of the leading word table in
/// halfwords), byte 7 (high nibble: signed count of event conditions; low
/// nibble: number of 3-byte entries before the conditions).
pub(crate) const SPCH_SENT_B6: u32 = 0x6;

pub(crate) const SPCH_SENT_B7: u32 = 0x7;

/// The event block's byte 4: size of its header in quarter words.
pub(crate) const SPCH_EVENT_B4: u32 = 0x4;

/// `bool SPCH_CheckEventMasks(Sentence* s, EventBlock* ev, u32* masks)`
/// (0x3d9ed0): whether every event condition of a speech sentence holds.
/// Condition `i` is a 3-byte record `{a, slot, b}` in the sentence (after the
/// word table and the entries counted by byte 7's low nibble) paired with
/// the mask word `i` of the event block (after its header). A zero mask
/// always holds; otherwise `masks[slot] & mask` must be non-zero. The first
/// failing condition stops the scan; the scan's last record (`a`, `slot`,
/// `b`) is spilled to the frame (dead stores the original makes).
pub fn spch_check_event_masks(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A3][0] = c.g[A0][0];
    let frame = Frame::open(c, 0x10);
    let sent = Addr(c.g[A3][0] as u32);
    let b7 = sent.off(SPCH_SENT_B7).lbu(c);
    c.g[A0][0] = b7;
    c.g[T0][0] = c.g[A1][0];
    c.g[T5][0] = c.g[A2][0];
    c.g[V0][0] = sx((b7 as u32) << 24);
    // The high nibble as a signed count.
    let count = (((b7 as u32) << 24) as i32 >> 28) as u32;
    c.g[T1][0] = sx(count);
    c.g[T4][0] = 1;
    if count != 0 {
        c.cycles += 22;
        let skip = b7 & 0xf;
        c.g[A2][0] = skip;
        let b6 = sent.off(SPCH_SENT_B6).lbu(c);
        let ev = c.g[T0][0] as u32;
        let b4 = Addr(ev.wrapping_add(SPCH_EVENT_B4)).lbu(c) as u32;
        // Word table, rounded to a word, after the 0xc-byte header.
        let words = (((b6 as u32) << 1) + 3) & !3;
        let table_end = sent.0.wrapping_add(words + 0xc);
        let masks_off = (((b4 >> 2) + 3) & !3) + 8;
        c.g[A1][0] = sx(ev.wrapping_add(masks_off));
        let records = table_end.wrapping_add((skip as u32 * 3 + 3) & 0x7c);
        c.g[V1][0] = sx(table_end);
        c.g[A0][0] = sx(masks_off);
        c.g[V0][0] = sx(records);
        c.g[A3][0] = 0;
        if (count as i32) > 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[V0][0];
            // a0: condition record, a1: its mask word, a3: conditions held.
            loop {
                c.cycles += 5;
                let rec = c.g[A0][0] as u32;
                let mask = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A2][0] = mask;
                c.g[T3][0] = Addr(rec).lbu(c);
                c.g[T0][0] = Addr(rec.wrapping_add(1)).lbu(c);
                c.g[T2][0] = Addr(rec.wrapping_add(2)).lbu(c);
                if mask == 0 {
                    c.cycles += 1;
                } else {
                    c.cycles += 6;
                    let at = ((c.g[T0][0] as u32) << 2).wrapping_add(c.g[T5][0] as u32);
                    c.g[V0][0] = sx(at);
                    c.g[V1][0] = Addr(at).lw(c) & mask;
                    if c.g[V1][0] == 0 {
                        c.cycles += 2;
                        c.g[T4][0] = 0;
                        break;
                    }
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                let more = (c.g[A3][0] as i64) < (c.g[T1][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A0][0] = sx(rec.wrapping_add(3));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_9f50);
                }
            }
            c.cycles += 3;
            let (t2, t0, t3) = (c.g[T2][0], c.g[T0][0], c.g[T3][0]);
            Addr(frame.at(c, 8)).sw(c, t2);
            Addr(frame.at(c, 4)).sw(c, t0);
            Addr(frame.at(c, 0)).sw(c, t3);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T4][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyLowpass (0x3c3178)

/// The sound system state (`0x50a8e8`): the channel array (+0x1f0), the
/// output rate (+0x2a, u16) and the two voice index bases (+0x2f, +0x30).
pub(crate) const SND_STATE: u32 = 0x0050_a8e8;

pub(crate) const SND_STATE_CHANNELS: u32 = 0x1f0;

pub(crate) const SND_STATE_RATE: u32 = 0x2a;

pub(crate) const SND_STATE_VOICE_BASE_A: u32 = 0x2f;

pub(crate) const SND_STATE_VOICE_BASE_B: u32 = 0x30;

/// Channel (0x8c bytes): its voices (+4, s16 each), voice count (+0x23),
/// flags (+0x24; bit 2: filtered).
pub(crate) const SND_CHANNEL_SIZE: u32 = 0x8c;

pub(crate) const SND_CHANNEL_VOICES: u32 = 0x4;

pub(crate) const SND_CHANNEL_VOICE_COUNT: u32 = 0x23;

pub(crate) const SND_CHANNEL_FLAGS: u32 = 0x24;

/// `void SND_ChannelApplyLowpass(int channel, int cutoff)` (0x3c3178): for
/// a filtered channel (flags bit 2), set every voice's lowpass to
/// `cutoff / (rate * 0.5)` (`SND_EeVoiceSetLowpass(voice - bases, f)`).
pub fn snd_channel_apply_lowpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = SND_CHANNEL_SIZE as u64;
        frame.save(c, S2, 0x20);
        let channel = c.g[A0][0] as u32;
        let off = abi::mult(c, channel, SND_CHANNEL_SIZE);
        c.g[A0][0] = sx(off);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x50);
        let snd = Addr(SND_STATE);
        c.g[V1][0] = sx(snd.0);
        c.g[A2][0] = snd.off(SND_STATE_CHANNELS).lw(c);
        let chan = Addr((c.g[A2][0] as u32).wrapping_add(off));
        c.g[S2][0] = sx(chan.0);
        let flags = chan.off(SND_CHANNEL_FLAGS).lhu(c);
        c.g[V0][0] = flags & 4;
        c.g[S1][0] = 0;
        if flags & 4 != 0 {
            c.cycles += 14;
            let rate = snd.off(SND_STATE_RATE).lhu(c);
            c.g[V0][0] = rate;
            let amount = FpuF32::from_i32(c.g[A1][0] as u32 as i32);
            c.f[3] = amount.0;
            let half = FpuF32(0x3f00_0000);
            c.f[2] = half.0;
            let rate_f = FpuF32::from_i32(rate as u32 as i32);
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            c.f[1] = FpuF32::ONE.0;
            let count = chan.off(SND_CHANNEL_VOICE_COUNT).lbu(c);
            c.g[V0][0] = count;
            let nyquist = rate_f * half;
            c.f[0] = nyquist.0;
            let inv = FpuF32::ONE.div(nyquist);
            c.f[1] = inv.0;
            c.f[20] = (amount * inv).0;
            if count != 0 {
                c.cycles += 3;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx(chan.off(SND_CHANNEL_VOICES).0);
                c.g[V0][0] = snd.off(SND_STATE_VOICE_BASE_A).lbu(c);
                // s0: next voice, s1: voices done; v0 holds base A.
                loop {
                    c.cycles += 12;
                    c.f[12] = c.f[20];
                    let state = c.g[S3][0] as u32;
                    let base_b = Addr(state.wrapping_add(SND_STATE_VOICE_BASE_B)).lbu(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    let at = c.g[S0][0] as u32;
                    c.g[V1][0] = Addr(at).lh(c);
                    let bases = (base_b as u32).wrapping_add(c.g[V0][0] as u32);
                    c.g[S0][0] = sx(at.wrapping_add(2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(bases));
                    call_ptr(c, 0x003c_8d00, 0x003c_3220)?;
                    let chan = c.g[S2][0] as u32;
                    let count = Addr(chan.wrapping_add(SND_CHANNEL_VOICE_COUNT)).lbu(c);
                    let more = (c.g[S1][0] as i64) < (count as i64);
                    c.g[V0][0] = more as u64;
                    if !more {
                        break;
                    }
                    let state = c.g[S3][0] as u32;
                    c.g[V0][0] = Addr(state.wrapping_add(SND_STATE_VOICE_BASE_A)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3200);
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetHighpass (0x3c8c30)

/// The EE voice table pointer; a voice entry is 0x60 bytes: DSP chain head
/// at +0x40, its highpass (FIR) DSP node at +0x5c.
pub(crate) const HIGHPASS_VOICES: u32 = 0x0051_5d1c;

pub(crate) const HIGHPASS_VOICE_SIZE: u32 = 0x60;

pub(crate) const HIGHPASS_VOICE_DSP_CHAIN: u32 = 0x40;

pub(crate) const HIGHPASS_VOICE_FILTER: u32 = 0x5c;

/// The DSP node: its size (0x120), priority (+0x18, 0x50).
pub(crate) const HIGHPASS_NODE_SIZE: u64 = 0x120;

pub(crate) const HIGHPASS_PRIORITY: u64 = 0x50;

pub(crate) const HIGHPASS_INIT: Callee = Callee {
    addr: 0x003c_a990,
    f: ee::s14::SND_DspFirHighpass_Init,
};

pub(crate) const HIGHPASS_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};

pub(crate) const HIGHPASS_SET_CUTOFF: Callee = Callee {
    addr: 0x003c_a9d8,
    f: ee::s14::SND_DspFirHighpass_SetCutoff,
};

pub(crate) const HIGHPASS_DSP_REMOVE: Callee = Callee {
    addr: 0x003c_8fb0,
    f: ee::s14::SND_DspRemove,
};

/// `void SND_EeVoiceSetHighpass(int voice, int cutoff)` (0x3c8c30): a
/// positive cutoff (Hz) gives the voice a FIR highpass DSP (created on
/// first use: `SND_MemAlloc(0x120)`, priority 0x50,
/// `SND_DspFirHighpass_Init`, inserted into the voice's DSP chain) and sets
/// it to `{cutoff << 8, rate << 8}`; zero or negative removes and frees the
/// voice's highpass DSP, if any. Frame 0x40 (s0 = voice entry, s1 =
/// cutoff); the entry offset uses `mult`.
pub fn snd_ee_voice_set_highpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = HIGHPASS_VOICE_SIZE as u64;
        c.g[V1][0] = 0x51_0000;
        let voice_index = c.g[A0][0] as u32;
        let off = abi::mult(c, voice_index, HIGHPASS_VOICE_SIZE);
        c.g[A0][0] = sx(off);
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = Addr(HIGHPASS_VOICES).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(off));
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[V0][0] = Addr(voice.wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
            let present = c.g[V0][0] != 0;
            c.g[V0][0] = 0x51_0000;
            if !present {
                // Create the highpass DSP and insert it into the chain.
                c.cycles += 15;
                c.g[A0][0] = HIGHPASS_NODE_SIZE;
                call_ptr(c, 0x003b_5c60, 0x003c_8c70)?; // SND_MemAlloc
                let filter = Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER));
                let dsp = c.g[V0][0];
                filter.sw(c, dsp);
                c.g[A0][0] = HIGHPASS_PRIORITY;
                Addr((dsp as u32).wrapping_add(4)).sw(c, 0);
                c.g[V0][0] = filter.lw(c);
                Addr(r32(c, V0).wrapping_add(0x18)).sh(c, HIGHPASS_PRIORITY);
                c.g[V1][0] = filter.lw(c);
                Addr(r32(c, V1).wrapping_add(0x1a)).sb(c, 0);
                c.g[A0][0] = filter.lw(c);
                HIGHPASS_INIT.call(c, 0x003c_8c94)?;
                let voice = r32(c, S0);
                c.g[A1][0] = Addr(voice.wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
                c.g[A0][0] = sx(voice.wrapping_add(HIGHPASS_VOICE_DSP_CHAIN));
                HIGHPASS_INSERT_SORTED.call(c, 0x003c_8ca0)?;
                c.g[V0][0] = 0x51_0000;
            }
            // params = { cutoff << 8, rate << 8 } on the stack.
            c.cycles += 10;
            let cutoff = sx(r32(c, S1) << 8);
            c.g[A0][0] = cutoff;
            let rate = Addr(SND_STATE + SND_STATE_RATE).lhu(c);
            let params = sp(c);
            c.g[A1][0] = c.g[SP][0];
            Addr(params).sw(c, cutoff);
            c.g[V1][0] = sx((rate as u32) << 8);
            let rate_word = c.g[V1][0];
            Addr(params + 4).sw(c, rate_word);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
            HIGHPASS_SET_CUTOFF.call(c, 0x003c_8cc4)?;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[A1][0] = Addr(voice.wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
            if c.g[A1][0] != 0 {
                // Remove the highpass DSP from the chain and free it.
                c.cycles += 6;
                c.g[A0][0] = sx(voice.wrapping_add(HIGHPASS_VOICE_DSP_CHAIN));
                HIGHPASS_DSP_REMOVE.call(c, 0x003c_8ce0)?;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
                call_ptr(c, 0x003b_5e98, 0x003c_8ce8)?; // SND_MemFree
                Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER)).sw(c, 0);
            }
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_DecodeSampleEntry (0x3db838).

/// Speech bank header: low nibble = extra bytes per entry.
pub(crate) const SPCH_DECODE_FLAGS: u32 = 0x4;

/// Speech bank header: number of entries.
pub(crate) const SPCH_DECODE_COUNT: u32 = 0x5;

/// Speech bank header: block size in 256-byte units, minus one.
pub(crate) const SPCH_DECODE_BLOCK: u32 = 0x7;

/// Speech bank header: (u16) end block of the last entry.
pub(crate) const SPCH_DECODE_END: u32 = 0x8;

/// Speech bank: first entry. Each is `2 + extra` bytes: a big-endian u16
/// start block, then the extra bytes; bit 7 of the first byte marks an
/// alias whose second byte is the index of the entry it shares.
pub(crate) const SPCH_DECODE_ENTRIES: u32 = 0xc;

/// Output record: length (word).
pub(crate) const SPCH_DECODE_OUT_LEN: u32 = 0x0;

/// Output record: start offset (word).
pub(crate) const SPCH_DECODE_OUT_START: u32 = 0x4;

/// Output record: number of extra bytes (word).
pub(crate) const SPCH_DECODE_OUT_EXTRA_N: u32 = 0x8;

/// Output record: the extra bytes (4 reset to 0xff, up to 15 copied).
pub(crate) const SPCH_DECODE_OUT_EXTRA: u32 = 0xc;

/// `bool SPCH_DecodeSampleEntry(SpchBank* bank, int index, SpchEntry* out)`
/// (0x3db838), a leaf: fills `out` with entry `index`'s start offset (start
/// block * block size), its length (up to the next entry that is not an
/// alias, else the bank's end) and its extra bytes. Returns 0 (with `out`
/// reset to len 0 / start -1 / extras 0xff) when `index` is not below the
/// entry count, else 1. The multiplies leave LO/HI as the original's; the
/// body works in the original's registers (t2 bank, a1 index, a3 entry,
/// t5 entry size, t0/t1 cursor/limit, t4 end, t6 found, a0 result).
pub fn spch_decode_sample_entry(c: &mut Cpu) {
    c.cycles += 16;
    let out = c.g[A2][0] as u32;
    c.g[V1][0] = 0xff;
    c.g[V0][0] = u64::MAX;
    Addr(out).off(0xf).sb(c, 0xff);
    c.g[T2][0] = c.g[A0][0];
    Addr(out).off(SPCH_DECODE_OUT_START).sw(c, u64::MAX);
    c.g[A0][0] = 0;
    Addr(out).off(SPCH_DECODE_OUT_LEN).sw(c, 0);
    c.g[T6][0] = 0;
    Addr(out).off(SPCH_DECODE_OUT_EXTRA_N).sw(c, 0);
    for i in 0..3 {
        Addr(out).off(SPCH_DECODE_OUT_EXTRA + i).sb(c, 0xff);
    }
    let count = Addr(c.g[T2][0] as u32).off(SPCH_DECODE_COUNT).lbu(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (count as i64)) as u64;
    c.g[T4][0] = 0;
    if c.g[V0][0] != 0 {
        spch_decode_sample_entry_found(c, out);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// The in-range part of [`spch_decode_sample_entry`].
pub(crate) fn spch_decode_sample_entry_found(c: &mut Cpu, out: u32) {
    c.cycles += 10;
    let bank = c.g[T2][0] as u32;
    c.g[V0][0] = Addr(bank).off(SPCH_DECODE_FLAGS).lbu(c);
    c.g[A3][0] = sx(bank.wrapping_add(SPCH_DECODE_ENTRIES));
    c.g[T7][0] = c.g[A3][0];
    c.g[T0][0] = 0;
    c.g[T1][0] = c.g[V0][0] & 0xf;
    c.g[T5][0] = sx((c.g[T1][0] as u32).wrapping_add(2));
    Addr(out).off(SPCH_DECODE_OUT_EXTRA_N).sw(c, c.g[T1][0]);
    c.mult(0, V0, A1, T5); // index * entry size
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if c.g[T1][0] != 0 {
        // Copy the extra bytes.
        c.cycles += 2;
        c.g[T3][0] = sx(out.wrapping_add(SPCH_DECODE_OUT_EXTRA));
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
            Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_b8a8);
            }
        }
    }
    // An alias: the start block comes from the entry it names.
    c.cycles += 4;
    c.g[V0][0] = Addr(c.g[A3][0] as u32).lbu(c) & 0x80;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(1)).lbu(c);
        c.mult(0, V0, A1, T5);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T7][0] as u32));
    }
    c.g[V1][0] = Addr(c.g[A3][0] as u32).lbu(c);
    // start = (hi << 8 | lo) * ((block + 1) << 8).
    c.cycles += 13;
    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[V0][0] = Addr(bank).off(SPCH_DECODE_BLOCK).lbu(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(1)).lbu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A3][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.mult(0, A0, A0, A3);
    Addr(out).off(SPCH_DECODE_OUT_START).sw(c, c.g[A0][0]);
    c.g[T1][0] = Addr(bank).off(SPCH_DECODE_COUNT).lbu(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T1][0] as i64)) as u64;
    // The end: the next non-alias entry's start, else the bank's end.
    loop {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(bank).off(SPCH_DECODE_END).lhu(c);
            c.g[T6][0] = 1;
            c.mult(0, T4, V0, A3);
        } else {
            c.g[V0][0] = Addr(c.g[T0][0] as u32).lbu(c);
            c.cycles += 3;
            c.g[V0][0] &= 0x80;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            } else {
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lbu(c);
                c.cycles += 5;
                c.g[T6][0] = 1;
                c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(1)).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.mult(0, T4, T4, A3);
            }
        }
        c.cycles += 2;
        if c.g[T6][0] != 0 {
            break;
        }
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T1][0] as i64)) as u64;
        if event_due(c) {
            deliver_event_here(c, 0x003d_b918);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    Addr(out).off(SPCH_DECODE_OUT_LEN).sw(c, c.g[V0][0]);
    c.g[A0][0] = 1;
}

// ---------------------------------------------------------------------------
// SPCH_NotifyPlayCallback (0x3d9830)

/// The speech system's "sample started" callback (`void (*)(PlayInfo*)`,
/// 0 when nobody listens).
pub(crate) const SPCH_PLAY_CALLBACK: u32 = 0x0044_ff90;

/// The sentence slot table: 8-byte entries `{ i16 bank, i16 variation
/// (-1: none), u16 sample, u16 }`.
pub(crate) const SPCH_SENTENCE_SLOTS: u32 = 0x0045_070c;

/// gp offsets: the bank pointer table, the "mark played variations" flag and
/// the total length the percentage is taken of.
pub(crate) const SPCH_GP_BANKS: u32 = 0x1734;

pub(crate) const SPCH_GP_MARK_PLAYED: u32 = 0x1740;

pub(crate) const SPCH_GP_TOTAL_LEN: u32 = 0x1748;

/// Fields of a speech bank: flags (0x80 = has variation bits, low nibble =
/// header words), sample count, block multiplier, block size, id.
pub(crate) const SPCH_BANK_ID: u32 = 0x2;

pub(crate) const SPCH_BANK_FLAGS: u32 = 0x4;

pub(crate) const SPCH_BANK_COUNT: u32 = 0x5;

pub(crate) const SPCH_BANK_BLOCKS: u32 = 0x7;

pub(crate) const SPCH_BANK_BLOCK_SIZE: u32 = 0x8;

pub(crate) const SPCH_FIND_BANK_TYPE: Callee = Callee {
    addr: 0x003d_8330,
    f: ee::s14::f_003d8330,
};

pub(crate) const SPCH_FIND_BANK_USER_DATA: Callee = Callee {
    addr: 0x003d_81f0,
    f: ee::s14::f_003d81f0,
};

pub(crate) const SPCH_DECODE_SAMPLE_ENTRY: Callee = Callee {
    addr: 0x003d_b838,
    f: ee::s14::f_003db838,
};

pub(crate) const SPCH_HISTORY_PUSH: Callee = Callee {
    addr: 0x003d_6ee8,
    f: ee::s14::SPCH_HistoryPush,
};

/// `int SPCH_NotifyPlayCallback(Sentence* sentence (a0), BankKey* key (a1),
/// int slot (a2))` (0x3d9830): when a play callback is installed, look up the
/// key's bank type and user data, then for each of the sentence's
/// `flags[4] >> 2` slot entries (from `slot`): clear the sample's
/// "variation pending" bit (when the bank has variation bits and marking is
/// on), decode the sample entry, and if it decodes, report `{bank, start
/// offset (+ variation * block bytes), length, key, user data, bank id,
/// type}` to the callback and push the sample into the bank's history.
/// Returns the summed length in percent of the total (gp+0x1748), or 0.
pub fn spch_notify_play_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 0x450000;
        frame.save(c, FP, 0xc0);
        frame.save(c, S7, 0xb0);
        c.g[FP][0] = 0x450000;
        frame.save(c, S5, 0x90);
        c.g[S7][0] = 0; // summed length
        frame.save(c, S1, 0x50);
        c.g[S5][0] = c.g[A1][0]; // key
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A2][0]; // slot
        c.g[V1][0] = Addr(SPCH_PLAY_CALLBACK).lw(c);
        c.g[S0][0] = c.g[A0][0]; // sentence
        frame.save(c, RA, 0xd0);
        c.g[V0][0] = 0;
        frame.save(c, S6, 0xa0);
        frame.save(c, S4, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S2, 0x60);
        let user_data = frame.at(c, 0x34);
        Addr(user_data).sw(c, 0);
        if c.g[V1][0] != 0 {
            c.cycles += 10;
            c.g[A1][0] = sx(frame.at(c, 0x30));
            c.g[A0][0] = c.g[S5][0];
            SPCH_FIND_BANK_TYPE.call(c, 0x003d_9890)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = sx(frame.at(c, 0x34));
            SPCH_FIND_BANK_USER_DATA.call(c, 0x003d_989c)?;
            c.g[V1][0] = Addr(r32(c, S0)).off(SPCH_BANK_FLAGS).lbu(c);
            c.g[A0][0] = sx((c.g[V1][0] as u32) >> 2);
            c.g[V0][0] = 0x450000;
            if c.g[A0][0] != 0 {
                c.cycles += 5;
                c.g[V1][0] = sx(r32(c, S1) << 3);
                c.g[V0][0] = sx(SPCH_SENTENCE_SLOTS);
                c.g[S4][0] = c.g[A0][0]; // entries left
                c.g[S2][0] = sx(SPCH_SENTENCE_SLOTS.wrapping_add(r32(c, V1))); // entry
                c.g[S6][0] = u64::MAX; // -1
                loop {
                    spch_notify_mark_played(c);
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[SP][0];
                    SPCH_DECODE_SAMPLE_ENTRY.call(c, 0x003d_9958)?;
                    if c.g[V0][0] != 0 {
                        spch_notify_report(c)?;
                    }
                    c.g[S4][0] = sx(r32(c, S4).wrapping_sub(1));
                    c.cycles += 2;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(8));
                    if c.g[S4][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_98c0);
                    }
                }
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(SPCH_GP_TOTAL_LEN)).lw(c);
            c.g[V0][0] = 0;
            if c.g[V1][0] != 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x64;
                c.mult(0, V0, S7, V0); // total * 100
                c.div(0, V0, V1);
                c.g[V0][0] = c.lo[0];
            }
        }
        c.cycles += 12;
        frame.restore(c, RA, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Loop head of [`spch_notify_play_callback`]: load the entry's bank (s0)
/// and sample (s1); when the bank keeps variation bits (flags 0x80) and
/// marking is on, clear the sample's bit in the bank's bit array (after the
/// header: `(count * (words + 2) + 15) & 0x3ffc + words * 4 + 1`).
pub(crate) fn spch_notify_mark_played(c: &mut Cpu) {
    c.cycles += 9;
    let entry = Addr(r32(c, S2));
    c.g[S3][0] = entry.lh(c);
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(SPCH_GP_BANKS)).lw(c);
    c.g[V1][0] = sx(r32(c, S3) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[S0][0] = Addr(r32(c, V1)).lw(c);
    let bank = Addr(r32(c, S0));
    c.g[A2][0] = bank.off(SPCH_BANK_FLAGS).lbu(c);
    c.g[V0][0] = c.g[A2][0] & 0x80;
    c.g[S1][0] = entry.off(4).lhu(c);
    if c.g[V0][0] == 0 {
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(SPCH_GP_MARK_PLAYED)).lw(c);
    c.g[A2][0] &= 0xf;
    if c.g[V0][0] == 0 {
        return;
    }
    c.cycles += 22;
    c.g[V1][0] = bank.off(SPCH_BANK_COUNT).lbu(c);
    c.g[A1][0] = sx(r32(c, A2).wrapping_add(2));
    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S1][0] as i64)) as u64;
    c.mult(0, V1, V1, A1); // count * (words + 2)
                           // sample / 8 (rounded toward zero, as `(s < 0 ? s + 7 : s) >> 3`)
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(7));
    if c.g[V0][0] != 0 {
        c.g[A0][0] = c.g[S1][0];
    }
    c.g[A2][0] = sx(r32(c, A2) << 2);
    c.g[A0][0] = sx((r32(c, A0) as i32 >> 3) as u32);
    c.g[A1][0] = 1;
    c.g[V0][0] = sx(r32(c, A0) << 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xf));
    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(r32(c, V0))); // bit index
    c.g[V1][0] &= 0x3ffc;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
    c.g[A1][0] = sx(r32(c, A1) << (r32(c, V0) & 31));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    c.g[A1][0] = !c.g[A1][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    let bits = Addr(r32(c, V1));
    c.g[V0][0] = bits.lbu(c) & c.g[A1][0];
    bits.sb(c, c.g[V0][0]);
}

/// A decoded entry of [`spch_notify_play_callback`]: build the play info at
/// sp+0x10 and hand it to the callback, then push the sample into the bank's
/// history. Decode output at sp: `{ length, start, ... }`.
pub(crate) fn spch_notify_report(c: &mut Cpu) -> crate::call::Flow {
    let sp = sp(c);
    c.cycles += 8;
    let bank = Addr(r32(c, S0));
    c.g[V0][0] = bank.off(SPCH_BANK_BLOCKS).lbu(c);
    c.g[V1][0] = bank.off(SPCH_BANK_BLOCK_SIZE).lhu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.g[A0][0] = Addr(r32(c, S2)).off(2).lh(c); // variation
    c.g[V0][0] = sx(r32(c, V0) << 8);
    c.g[A1][0] = Addr(sp.wrapping_add(4)).lw(c); // start
    let no_variation = c.g[A0][0] == c.g[S6][0];
    c.mult(0, V1, V1, V0); // block bytes
    if !no_variation {
        c.cycles += 2;
        c.mult(0, V0, A0, V1);
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    }
    c.cycles += 22;
    c.g[V0][0] = Addr(sp).lw(c); // length
    c.g[A0][0] = sx(sp.wrapping_add(0x10));
    Addr(sp.wrapping_add(0x14)).sw(c, c.g[A1][0]);
    Addr(sp.wrapping_add(0x10)).sw(c, c.g[S3][0]);
    c.g[S7][0] = sx(r32(c, S7).wrapping_add(r32(c, V0)));
    Addr(sp.wrapping_add(0x18)).sw(c, c.g[V0][0]);
    // The key (4 bytes, maybe unaligned) to sp+0x1c.
    let key = r32(c, S5);
    c.lwl(V1, key.wrapping_add(3));
    c.lwr(V1, key);
    c.swl(V1, sp.wrapping_add(0x1f));
    c.swr(V1, sp.wrapping_add(0x1c));
    c.g[V0][0] = Addr(r32(c, FP).wrapping_sub(0x70)).lw(c); // the callback
    c.g[V1][0] = Addr(sp.wrapping_add(0x30)).lw(c); // bank type
    c.g[A1][0] = Addr(sp.wrapping_add(0x34)).lw(c); // user data
    Addr(sp.wrapping_add(0x28)).sw(c, c.g[V1][0]);
    Addr(sp.wrapping_add(0x20)).sw(c, c.g[A1][0]);
    c.g[V1][0] = bank.off(SPCH_BANK_ID).lhu(c);
    let callback = r32(c, V0);
    Addr(sp.wrapping_add(0x24)).sw(c, c.g[V1][0]);
    call_ptr(c, callback, 0x003d_99d0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    SPCH_HISTORY_PUSH.call(c, 0x003d_99dc)
}

// ---------------------------------------------------------------------------
// SNDSTRM_StartChannel (0x3c6760)

/// The sound streams' channel records (`StreamChannel*[]`, by stream index).
#[cfg(test)]
pub(crate) const SNDSTRM_CHANNELS: u32 = 0x0051_5abc;

/// The sound system's state block (`D_0050A8E8`; +0x1f0: the voice/channel
/// array, 0x8c bytes per channel).
pub(crate) const SNDSTRM_SND_STATE: u32 = 0x0050_a8e8;

pub(crate) const SNDSTRM_SND_CHANNELS: u32 = 0x1f0;

pub(crate) const SNDSTRM_SND_CHANNEL_SIZE: u32 = 0x8c;

/// Stream channel fields: result/handle word, per-voice stream offsets
/// (+0x10 words, +0x28 and +0x34 halves), voice count byte, max voice, the
/// copied stream header word and per-voice stream parameters (+0x64 words).
pub(crate) const SNDSTRM_CH_RESULT: u32 = 0x0;

pub(crate) const SNDSTRM_CH_MAX_VOICE: u32 = 0x42;

pub(crate) const SNDSTRM_CH_HEADER: u32 = 0x60;

pub(crate) const SNDSTRM_CH_VOICES: u32 = 0x62;

pub(crate) const SNDSTRM_CH_PARAMS: u32 = 0x64;

/// Stack frame slots: the voice-group request (sp+0: b3 = voice count,
/// +0x18 = capability flags), the group voice range (0xd0, 0xd4), the
/// allocation handle (0xd8), the group index (0xdc), the stream index (0xe0).
pub(crate) const SNDSTRM_START_FRAME: u32 = 0x190;

pub(crate) const SNDSTRM_GET_GROUP_VOICE_RANGE: Callee = Callee {
    addr: 0x003c_0c18,
    f: ee::s13::SND_GetGroupVoiceRange,
};

pub(crate) const SNDSTRM_ALLOC_VOICES: Callee = Callee {
    addr: 0x003b_a0b0,
    f: ee::s13::SND_AllocVoices,
};

pub(crate) const SNDSTRM_FIND_MAX_VOICE: Callee = Callee {
    addr: 0x003c_6580,
    f: ee::s13::SND_ChannelFindMaxVoice,
};

pub(crate) const SNDSTRM_CALC_PITCH: Callee = Callee {
    addr: 0x003b_a938,
    f: ee::s13::SND_ChannelCalcPitch,
};

pub(crate) const SNDSTRM_CALC_VOLUME: Callee = Callee {
    addr: 0x003b_a7b0,
    f: ee::s13::SND_ChannelCalcVolume,
};

pub(crate) const SNDSTRM_CALC_SEND: Callee = Callee {
    addr: 0x003b_a740,
    f: ee::s13::SND_ChannelCalcSend,
};

pub(crate) const SNDSTRM_CHANNEL_RELEASE: Callee = Callee {
    addr: 0x003b_a550,
    f: ee::s13::SND_ChannelRelease,
};

pub(crate) const SNDSTRM_NEXT_VOICE_GROUP: Callee = Callee {
    addr: 0x003c_6158,
    f: ee::s13::SND_NextVoiceGroup,
};

/// `SND_ChannelStartStreamVoices` (called through the function table).
pub(crate) const SNDSTRM_START_STREAM_VOICES: u32 = 0x003c_e1b0;

/// `reg + off` as an address.
#[inline]
pub(crate) fn sndstrm_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(r32(c, reg).wrapping_add(off))
}

/// `int SNDSTRM_StartChannel(int stream (a0), StreamHeader* hdr (a1),
/// StreamInfo* info (a2), StreamParams* params (a3))` (0x3c6760): reset the
/// stream's channel record (copy the header word, clear the per-voice
/// offsets, copy the per-voice parameters from `info + 0x18`), then walk the
/// voice groups that suit the request: allocate `hdr[2]` voices in the
/// group's range (-9 into the record when that fails, then try the next
/// group); on success set up the sound channel (pitch, volume, pan, send,
/// envelope fields from `info` and `params`), calculate pitch, volume and
/// send, and start the stream voices; if that fails release the voices and
/// store the error. Returns the record's result word.
///
/// The original is dispatcher-shaped: its loops' event checks can unwind,
/// and the function is re-entered at the loop heads 0x3c6818 (record reset),
/// 0x3c6858 (next voice group), 0x3c6988 (envelope copy) and 0x3c6a40
/// (voice release); all loop state is in registers.
pub fn sndstrm_start_channel(c: &mut Cpu) {
    let frame = Frame {
        size: SNDSTRM_START_FRAME,
    };
    let mut pc = c.pc;
    loop {
        match pc {
            0x003c_6760 => {
                c.cycles += 41;
                Frame::open(c, SNDSTRM_START_FRAME);
                c.g[V0][0] = 0x510000;
                Addr(frame.at(c, 0xe0)).sw(c, c.g[A0][0]);
                c.g[V0][0] = sx(0x0051_57b8);
                frame.save(c, S0, 0x180);
                frame.save(c, S5, 0x130);
                c.g[V1][0] = sx(r32(c, A0) << 2);
                frame.save(c, S6, 0x120);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                frame.save(c, S7, 0x110);
                c.g[S6][0] = c.g[A1][0]; // header
                frame.save(c, S1, 0x170);
                c.g[S7][0] = c.g[A2][0]; // info
                frame.save(c, S2, 0x160);
                c.g[S5][0] = c.g[A3][0]; // params
                frame.save(c, S3, 0x150);
                c.g[S0][0] = 0;
                frame.save(c, S4, 0x140);
                frame.save(c, FP, 0x100);
                frame.save_ra(c, 0xf0);
                c.g[A0][0] = sndstrm_at(c, S6, 2).lbu(c);
                c.g[A1][0] = sndstrm_at(c, S7, 8).lhu(c);
                c.g[S3][0] = sndstrm_at(c, V0, 0x304).lw(c); // the channel record
                c.g[FP][0] = sndstrm_at(c, S6, 2).lbu(c); // voice count
                                                          // The voice-group request at sp.
                Addr(frame.at(c, 0x3)).sb(c, c.g[A0][0]);
                Addr(frame.at(c, 0x18)).sh(c, c.g[A1][0]);
                Addr(frame.at(c, 0xdc)).sw(c, 0);
                // The header word (maybe unaligned) into the record.
                let (hdr, rec) = (r32(c, S6), r32(c, S3));
                c.lwl(V0, hdr.wrapping_add(3));
                c.lwr(V0, hdr);
                c.swl(V0, rec.wrapping_add(SNDSTRM_CH_HEADER + 3));
                c.swr(V0, rec.wrapping_add(SNDSTRM_CH_HEADER));
                let rec = Addr(rec);
                rec.off(0x4).sw(c, 0);
                rec.off(0x48).sw(c, 0);
                rec.off(0x4c).sw(c, 0);
                rec.off(0xc).sw(c, 0);
                rec.off(0x8).sw(c, 0);
                rec.off(0x46).sh(c, 0);
                c.g[V0][0] = rec.off(SNDSTRM_CH_VOICES).lbu(c);
                rec.off(0x44).sh(c, 0);
                c.g[A3][0] = sx(rec.0.wrapping_add(SNDSTRM_CH_PARAMS));
                if c.g[V0][0] == 0 {
                    pc = 0x003c_6a68;
                } else {
                    c.cycles += 5;
                    c.g[A2][0] = sx(r32(c, S7).wrapping_add(0x18));
                    c.g[A1][0] = sx(rec.0.wrapping_add(0x28));
                    c.g[A0][0] = sx(rec.0.wrapping_add(0x34));
                    c.g[V1][0] = sx(rec.0.wrapping_add(0x10));
                    pc = 0x003c_6818;
                }
            }
            // Record reset loop: clear the voice's offsets, copy its
            // parameter word.
            0x003c_6818 => {
                c.cycles += 14;
                sndstrm_at(c, V1, 0).sw(c, 0);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                sndstrm_at(c, A0, 0).sh(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                sndstrm_at(c, A1, 0).sh(c, 0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(2));
                c.g[V0][0] = sndstrm_at(c, A2, 0).lw(c);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                sndstrm_at(c, A3, 0).sw(c, c.g[V0][0]);
                c.g[V0][0] = sndstrm_at(c, S3, SNDSTRM_CH_VOICES).lbu(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, 0x003c_6818) {
                        return;
                    }
                    pc = 0x003c_6818;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = sx(frame.at(c, 0xdc));
                    pc = 0x003c_6a6c;
                }
            }
            // A voice group (s2): its voice range, then allocate.
            0x003c_6858 => {
                c.cycles += 12;
                c.g[A2][0] = sx(frame.at(c, 0xd4));
                c.g[A0][0] = c.g[S2][0];
                if SNDSTRM_GET_GROUP_VOICE_RANGE.call(c, 0x003c_6864).is_err() {
                    return;
                }
                c.g[A3][0] = Addr(frame.at(c, 0xd0)).lw(c);
                c.g[A0][0] = c.g[FP][0];
                c.g[T0][0] = Addr(frame.at(c, 0xd4)).lw(c);
                c.g[A1][0] = 0x65; // priority
                c.g[A2][0] = sx(frame.at(c, 0xd8));
                if SNDSTRM_ALLOC_VOICES.call(c, 0x003c_687c).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0]; // the sound channel
                c.g[V0][0] = Addr(frame.at(c, 0xd8)).lw(c); // its handle
                if (c.g[S1][0] as i64) >= 0 {
                    c.cycles += 59;
                    if sndstrm_setup_channel(c).is_err() {
                        return;
                    }
                    pc = if c.g[V0][0] == 0 {
                        0x003c_69a8
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = sx(r32(c, T1).wrapping_add(0x50));
                        0x003c_6988
                    };
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = (-9i64) as u64;
                    sndstrm_at(c, S3, SNDSTRM_CH_RESULT).sw(c, c.g[V0][0]);
                    c.g[A0][0] = sx(frame.at(c, 0xdc));
                    pc = 0x003c_6a6c;
                }
            }
            // Envelope copy loop: info+0xc halves into channel+0x50.
            0x003c_6988 => {
                c.cycles += 8;
                c.g[V0][0] = sndstrm_at(c, V1, 0).lhu(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
                sndstrm_at(c, A0, 0).sh(c, c.g[V0][0]);
                c.g[V0][0] = sndstrm_at(c, S6, 2).lbu(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, 0x003c_6988) {
                        return;
                    }
                    pc = 0x003c_6988;
                } else {
                    pc = 0x003c_69a8;
                }
            }
            0x003c_69a8 => {
                c.cycles += 30;
                if sndstrm_start_voices(c, frame).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                if (c.g[S2][0] as i64) >= 0 {
                    pc = 0x003c_6a80;
                } else {
                    // Failed: release the allocated voices.
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                    if c.g[FP][0] == 0 {
                        pc = 0x003c_6a64;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = SNDSTRM_SND_CHANNEL_SIZE as u64;
                        c.mult(0, S1, S1, V0);
                        c.g[V0][0] = 0x510000;
                        c.g[S4][0] = sx(SNDSTRM_SND_STATE);
                        c.g[V0][0] = sndstrm_at(c, S4, SNDSTRM_SND_CHANNELS).lw(c);
                        pc = 0x003c_6a40;
                    }
                }
            }
            // Release loop: SND_ChannelRelease(channel->voices[i]).
            0x003c_6a40 => {
                c.cycles += 9;
                c.g[V1][0] = sx(r32(c, S0) << 1);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = sndstrm_at(c, V0, 4).lh(c);
                if SNDSTRM_CHANNEL_RELEASE.call(c, 0x003c_6a58).is_err() {
                    return;
                }
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = sndstrm_at(c, S4, SNDSTRM_SND_CHANNELS).lw(c);
                    if back_edge_unwinds(c, 0x003c_6a40) {
                        return;
                    }
                    pc = 0x003c_6a40;
                } else {
                    pc = 0x003c_6a64;
                }
            }
            0x003c_6a64 => {
                c.cycles += 1;
                sndstrm_at(c, S3, SNDSTRM_CH_RESULT).sw(c, c.g[S2][0]);
                pc = 0x003c_6a68;
            }
            0x003c_6a68 => {
                c.cycles += 1;
                c.g[A0][0] = sx(frame.at(c, 0xdc));
                pc = 0x003c_6a6c;
            }
            // Next voice group for the request at sp.
            0x003c_6a6c => {
                c.cycles += 5;
                c.g[A1][0] = c.g[SP][0];
                if SNDSTRM_NEXT_VOICE_GROUP.call(c, 0x003c_6a74).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                if c.g[S2][0] != 0 {
                    c.g[A1][0] = sx(frame.at(c, 0xd0));
                    if back_edge_unwinds(c, 0x003c_6858) {
                        return;
                    }
                    pc = 0x003c_6858;
                } else {
                    pc = 0x003c_6a80;
                }
            }
            0x003c_6a80 => {
                c.cycles += 13;
                c.g[V0][0] = sndstrm_at(c, S3, SNDSTRM_CH_RESULT).lw(c);
                frame.restore(c, S0, 0x180);
                frame.restore(c, S1, 0x170);
                frame.restore(c, S2, 0x160);
                frame.restore(c, S3, 0x150);
                frame.restore(c, S4, 0x140);
                frame.restore(c, S5, 0x130);
                frame.restore(c, S6, 0x120);
                frame.restore(c, S7, 0x110);
                frame.restore(c, FP, 0x100);
                frame.restore_ra(c, 0xf0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003c_6760, 0x003c_6ab4);
                return;
            }
        }
    }
}

/// Allocation succeeded (s1 = sound channel, v0 = handle): store the handle,
/// find the channel's max voice, and fill the channel's fields from `info`
/// (s7) and `params` (s5). Leaves t1 = the channel, v1 = info + 0xc and
/// v0 = the voice count (whether the envelope copy runs).
pub(crate) fn sndstrm_setup_channel(c: &mut Cpu) -> crate::call::Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = 0;
    sndstrm_at(c, S3, SNDSTRM_CH_RESULT).sw(c, c.g[V0][0]);
    c.g[A1][0] = sndstrm_at(c, S6, 2).lbu(c);
    SNDSTRM_FIND_MAX_VOICE.call(c, 0x003c_68ac)?;
    c.g[A3][0] = SNDSTRM_SND_CHANNEL_SIZE as u64;
    sndstrm_at(c, S3, SNDSTRM_CH_MAX_VOICE).sb(c, c.g[V0][0]);
    c.mult(0, V1, S1, A3); // channel * 0x8c
    c.g[A1][0] = u64::MAX;
    c.g[A3][0] = 0x510000;
    c.g[A0][0] = 0x7fff0000;
    c.g[A3][0] = sx(SNDSTRM_SND_STATE);
    c.g[A2][0] = 0x7f0000;
    c.g[V0][0] = sndstrm_at(c, A3, SNDSTRM_SND_CHANNELS).lw(c);
    c.g[A0][0] |= 0xffff;
    c.g[A3][0] = 1;
    c.g[T0][0] = 0x64;
    c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    let ch = Addr(r32(c, T1));
    let info = Addr(r32(c, S7));
    let params = Addr(r32(c, S5));
    ch.off(0x4c).sh(c, 0);
    c.g[V1][0] = params.off(0x8).lhu(c);
    ch.off(0x12).sh(c, c.g[A1][0]);
    ch.off(0x1c).sh(c, c.g[V1][0]);
    c.g[V0][0] = info.lhu(c);
    ch.off(0x84).sh(c, c.g[V0][0]); // base pitch (cents)
    c.g[V1][0] = params.off(0xc).lhu(c);
    ch.off(0x30).sw(c, 0);
    ch.off(0x88).sh(c, c.g[V1][0]);
    c.g[V0][0] = params.lb(c);
    ch.off(0x44).sw(c, c.g[A0][0]);
    c.g[V0][0] = sx(r32(c, V0) << 16);
    ch.off(0x40).sw(c, c.g[A2][0]);
    ch.off(0x38).sw(c, c.g[V0][0]); // volume << 16
    ch.off(0x3c).sw(c, 0);
    c.g[V0][0] = info.off(0x3).lbu(c);
    ch.off(0x2b).sb(c, c.g[V0][0]);
    c.g[V1][0] = params.off(0x1).lbu(c);
    ch.off(0x5c).sb(c, c.g[A3][0]);
    ch.off(0x64).sb(c, c.g[V1][0]);
    ch.off(0x5d).sb(c, 0);
    ch.off(0x5e).sb(c, 0);
    c.g[V0][0] = info.off(0x5).lbu(c);
    ch.off(0x60).sb(c, c.g[V0][0]);
    c.g[V1][0] = params.off(0x4).lbu(c);
    ch.off(0x5f).sb(c, c.g[V1][0]);
    c.g[V0][0] = params.off(0x5).lbu(c);
    ch.off(0x61).sb(c, c.g[V0][0]);
    c.g[V1][0] = info.off(0x6).lb(c);
    ch.off(0x26).sb(c, 0);
    c.mult(0, V1, V1, T0); // pitch range * 100
    ch.off(0x27).sb(c, 0);
    ch.off(0x82).sh(c, c.g[V1][0]);
    for off in [0x6c, 0x70, 0x74, 0x78, 0x7c] {
        ch.off(off).sw(c, 0);
    }
    c.g[V0][0] = sndstrm_at(c, S6, 2).lbu(c);
    c.g[V1][0] = sx(info.0.wrapping_add(0xc));
    Ok(())
}

/// Finish the channel (t1) setup, calculate pitch, volume and send, and
/// start its stream voices; v0 = the result.
pub(crate) fn sndstrm_start_voices(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let ch = Addr(r32(c, T1));
    let hdr = Addr(r32(c, S6));
    c.g[V0][0] = u64::MAX;
    ch.off(0x14).sw(c, 0);
    ch.off(0x18).sw(c, c.g[V0][0]);
    c.g[A0][0] = c.g[S1][0];
    c.g[V0][0] = hdr.lhu(c);
    ch.off(0x20).sh(c, c.g[V0][0]);
    c.g[V1][0] = hdr.off(0x3).lbu(c);
    ch.off(0x22).sb(c, c.g[V1][0]);
    c.g[V0][0] = hdr.off(0x2).lbu(c);
    ch.off(0x24).sh(c, c.g[S2][0]); // voice group
    ch.off(0x23).sb(c, c.g[V0][0]);
    ch.off(0x86).sh(c, 0);
    SNDSTRM_CALC_PITCH.call(c, 0x003c_69dc)?;
    c.g[A0][0] = c.g[S1][0];
    SNDSTRM_CALC_VOLUME.call(c, 0x003c_69e4)?;
    c.g[A0][0] = 0;
    c.g[A1][0] = c.g[S1][0];
    SNDSTRM_CALC_SEND.call(c, 0x003c_69f0)?;
    let params = Addr(r32(c, S5));
    c.g[A2][0] = params.off(0xe).lhu(c);
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(frame.at(c, 0xe0)).lw(c); // stream
    c.g[T2][0] = c.g[S6][0];
    c.g[A3][0] = params.off(0x12).lhu(c);
    c.g[T3][0] = c.g[S7][0];
    c.g[T0][0] = params.off(0x14).lhu(c);
    c.g[T1][0] = params.off(0x16).lhu(c);
    call_ptr(c, SNDSTRM_START_STREAM_VOICES, 0x003c_6a14)
}

#[cfg(test)]
pub(crate) mod tests {
    crate::difftest::events::driven!(oc238_gen_prebuffering, ee::s13::f_003b70f8__generated);
    crate::difftest::events::driven!(oc238_hand_prebuffering, sndstrm_is_prebuffering);
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    #[test]
    fn snd_add_tick_callback_matches() {
        check(
            ssx3_ee::s13::f_003b5910,
            snd_add_tick_callback,
            200,
            &|c, seed| {
                let mut r = Rng(0x1001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let base = 0x0050_a8e8u32;
                c.bus
                    .write8(base + 0x174, if seed % 3 == 0 { 0 } else { 1 });
                let idx = if seed % 2 == 0 {
                    r.next_u32() & 0x1f
                } else {
                    r.next_u32() | 0x80
                };
                c.bus.write8(base + 0x177, idx as u8);
            },
        );
    }

    #[test]
    fn snd_add_event_callback_matches() {
        check(
            ssx3_ee::s13::f_003b5a28,
            snd_add_event_callback,
            200,
            &|c, seed| {
                let mut r = Rng(0x1002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let base = 0x0050_a8e8u32;
                c.bus
                    .write8(base + 0x174, if seed % 3 == 0 { 0 } else { 1 });
                let idx = if seed % 2 == 0 {
                    r.next_u32() & 0x1f
                } else {
                    r.next_u32() | 0x80
                };
                c.bus.write8(base + 0x179, idx as u8);
            },
        );
    }

    #[test]
    fn snd_mem_clamp_block_matches() {
        check(
            ssx3_ee::s13::f_003b5ba0,
            snd_mem_clamp_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x1003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 4) as u64;
                let a0 = r.next_u32() & 0x7fff_ffff;
                let sz = r.next_u32() & 0x7fff_ffff;
                c.bus.write32(DATA, a0);
                c.bus.write32(DATA + 4, sz);
                let st = DATA + 0x100;
                c.bus.write32(0x0050_ab1c, st);
                let lim = if seed % 2 == 0 {
                    a0.wrapping_add(sz).wrapping_sub(1)
                } else {
                    a0.wrapping_add(sz).wrapping_add(1)
                };
                c.bus.write32(st + 0xc, lim);
            },
        );
    }

    #[test]
    fn snd_mem_init_heap_matches() {
        check(
            ssx3_ee::s13::f_003b5bd0,
            snd_mem_init_heap,
            200,
            &|c, seed| {
                let mut r = Rng(0x1004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
                c.bus.write32(0x0050_ab1c, DATA + 0x400);
            },
        );
    }

    #[test]
    fn snd_random_matches() {
        check(ssx3_ee::s13::f_003bc5c8, snd_random, 200, &|c, seed| {
            let mut r = Rng(0x1005 ^ seed as u64);
            scramble(c, &mut r);
            let t0 = 0x0050_ac50u32;
            c.bus.write32(t0, r.next_u32());
            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14] {
                c.bus.write32(t0 + off, r.next_u32());
            }
            if seed % 4 == 0 {
                // Force a carry in the low word sum.
                c.bus.write32(t0 + 0x10, 0xffff_fff0);
                c.bus.write32(t0 + 0x14, 0xffff_fff0);
            }
            if seed % 3 == 0 {
                // Force every counter-increment branch to wrap.
                for off in [0x4u32, 0x8, 0xc, 0x10, 0x14] {
                    c.bus.write32(t0 + off, 0xffff_ffff);
                }
            }
        });
    }

    #[test]
    fn sndenc_install_dts_matches() {
        check(
            ssx3_ee::s13::f_003bc930,
            sndenc_install_dts,
            50,
            &|c, seed| {
                let mut r = Rng(0x1006 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_get_group_voice_range_matches() {
        check(
            ssx3_ee::s13::f_003c0c18,
            snd_get_group_voice_range,
            200,
            &|c, seed| {
                let mut r = Rng(0x1007 ^ seed as u64);
                scramble(c, &mut r);
                let group = match seed % 8 {
                    0 => 0u32,
                    1 => 8,
                    2 => 4,
                    3 => 0x100,
                    4 => 0x104,
                    5 => 0x108,
                    6 => 0x10c,
                    _ => r.next_u32(),
                };
                c.g[A0][0] = group as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (DATA + 0x304) as u64;
                let base = 0x0050_a8e8u32;
                c.bus.write8(base + 0x2f, r.next_u32() as u8);
                c.bus.write8(base + 0x30, r.next_u32() as u8);
                c.bus.write16(base + 0x17e, r.next_u32() as u16);
                c.bus.write8(0x0050_a918, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn snd_set_spu_reverb_mode_matches() {
        check(
            ssx3_ee::s13::f_003c3500,
            snd_set_spu_reverb_mode,
            200,
            &|c, seed| {
                let mut r = Rng(0x1008 ^ seed as u64);
                scramble(c, &mut r);
                let mode =
                    [0u32, 1, 5, 10, 20, 30, 40, 50, 100, 110, 120, 7, 99][(seed % 13) as usize];
                c.g[A0][0] = mode as u64;
                for k in 0..130u32 {
                    c.bus.write32(0x0044_dd48 + k * 4, r.next_u32());
                }
                c.bus.write32(0x0050_b9b0, DATA + 0x400);
                c.bus.write8(0x0050_a8e8 + 0x224, r.next_u32() as u8);
                c.bus.write8(0x0050_a8e8 + 0x228, r.next_u32() as u8);
                c.bus.write16(0x0050_ad00 + 0x1a9e, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn snd_set_spu_fx_volume_matches() {
        check(
            ssx3_ee::s13::f_003c36c8,
            snd_set_spu_fx_volume,
            200,
            &|c, seed| {
                let mut r = Rng(0x1009 ^ seed as u64);
                scramble(c, &mut r);
                let flags = if seed % 3 == 0 {
                    r.next_u32() & !8
                } else {
                    r.next_u32() | 8
                };
                c.g[A1][0] = flags as u64;
                let ptr = if seed % 5 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(0x0050_b9b0, ptr);
                c.bus.write32(0x0050_ab08 + 0xc, r.edge_f32());
                c.bus.write32(0x0050_ab08 + 0x10, r.edge_f32());
            },
        );
    }

    #[test]
    fn snd_spu_mem_clamp_block_matches() {
        check(
            ssx3_ee::s13::f_003c4af8,
            snd_spu_mem_clamp_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x100a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 4) as u64;
                let a0 = r.next_u32() & 0xffff;
                let sz = r.next_u32() & 0xffff;
                c.bus.write32(DATA, a0);
                c.bus.write32(DATA + 4, sz);
                c.bus.write16(0x0050_ad00 + 0x1a9a, r.next_u32() as u16);
                c.bus.write16(0x0050_ad00 + 0x1a9c, r.next_u32() as u16);
                c.bus.write16(0x0050_ad00 + 0x1a9e, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn sndi_spu_mem_set_base_matches() {
        check(
            ssx3_ee::s13::f_003c4e50,
            sndi_spu_mem_set_base,
            200,
            &|c, seed| {
                let mut r = Rng(0x100b ^ seed as u64);
                scramble(c, &mut r);
                let a0 = match seed % 5 {
                    0 => 0xffff_ffff_ffff_ffffu64,
                    1 => 0,
                    2 => 0x1234,
                    3 => 0x7fff,
                    _ => (r.next_u32() as u64) | ((r.next_u32() as u64) << 32),
                };
                c.g[A0][0] = a0;
                let a1 = match seed % 4 {
                    0 => 0xffff_ffff_ffff_ffffu64,
                    1 => 0,
                    2 => 0x1_0000,
                    _ => (r.next_u32() as u64) | ((r.next_u32() as u64) << 32),
                };
                c.g[A1][0] = a1;
                c.bus.write16(0x0050_ad00 + 0x1a9a, r.next_u32() as u16);
                c.bus.write16(0x0050_ad00 + 0x1a9c, r.next_u32() as u16);
            },
        );
    }

    /// `SND_EeVoiceSetPitch`/`SetVolume`'s voice array pointer (`0x515b40 +
    /// 0x1dc`).
    pub(crate) const EE_VOICES_PTR: u32 = 0x0051_5d1c;

    /// A channel with `flags` and the driver globals the channel functions
    /// read (voice numbers 6..23, so every voice index stays positive).
    pub(crate) fn setup_channel(c: &mut Cpu, r: &mut Rng, seed: u32, flags: u16) {
        let index = r.next_u32() % 3;
        c.g[A0][0] = index as u64;
        c.bus.write32(STATE + state::CHANNELS, DATA);
        let ch = DATA + index * chan::SIZE;
        c.bus.write16(ch + chan::FLAGS, flags);
        c.bus.write16(ch + chan::PITCH, r.next_u32() as u16);
        c.bus.write16(ch + chan::PITCH_SCALE, r.next_u32() as u16);
        c.bus
            .write8(ch + chan::VOICE_COUNT, ((seed / 12) % 5) as u8);
        for i in 0..4 {
            c.bus
                .write16(ch + chan::VOICES + 2 * i, (6 + r.next_u32() % 18) as u16);
        }
        let volume = if seed % 9 == 0 {
            r.edge_f32()
        } else {
            r.f32(2.0)
        };
        c.bus.write32(ch + chan::VOLUME, volume);
        c.bus.write8(ch + chan::PAN, r.next_u32() as u8);
        let mode = if seed % 2 == 0 {
            2
        } else {
            r.next_u32() as u8 | 1
        };
        c.bus.write8(STATE + state::OUTPUT_MODE, mode);
        c.bus
            .write8(STATE + state::SPEAKERS, if seed % 4 < 2 { 4 } else { 2 });
        c.bus
            .write8(STATE + state::AUX_SENDS, (r.next_u32() % 3) as u8);
        let rate = |r: &mut Rng| {
            if seed % 7 == 0 {
                0
            } else {
                r.next_u32() as u16
            }
        };
        let (iop, ee_rate) = (rate(r), rate(r));
        c.bus.write16(STATE + state::IOP_RATE, iop);
        c.bus.write16(STATE + state::EE_RATE, ee_rate);
        c.bus
            .write8(STATE + state::VOICE_BASE, (r.next_u32() % 4) as u8);
        c.bus
            .write8(STATE + state::EE_VOICE_BASE, (r.next_u32() % 3) as u8);
        c.bus.write32(SYNC + sync::VOICE_PARAMS, DATA + 0x1_0000);
        for i in 0..32 * 8 {
            c.bus.write32(SYNC + sync::GAINS + 4 * i, r.f32(1.0));
        }
        // EE voices with a resampler (SND_EeVoiceSetPitch takes the
        // SetRate path).
        c.bus.write32(EE_VOICES_PTR, DATA + 0x2_0000);
        for v in 0..32 {
            c.bus.write32(
                DATA + 0x2_0000 + v * 0x60 + 0x50,
                DATA + 0x3_0000 + v * 0x40,
            );
        }
    }

    /// Flags selecting the SPU / IOP / EE path by `seed % 3`, other bits
    /// random.
    pub(crate) fn channel_flags(r: &mut Rng, seed: u32) -> u16 {
        let other = r.next_u32() as u16 & !0x108;
        match seed % 3 {
            0 => other | 0x8 | (r.next_u32() as u16 & 0x100),
            1 => other | 0x100,
            _ => other,
        }
    }

    #[test]
    fn snd_channel_apply_pitch_matches() {
        check(
            ee::s13::f_003c25e0__generated,
            snd_channel_apply_pitch,
            240,
            &|c, seed| {
                let mut r = Rng(0x7a11_0001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let flags = channel_flags(&mut r, seed);
                setup_channel(c, &mut r, seed, flags);
            },
        );
    }

    #[test]
    fn snd_channel_apply_volume_matches() {
        check(
            ee::s13::f_003c2a50__generated,
            snd_channel_apply_volume,
            240,
            &|c, seed| {
                let mut r = Rng(0x7a11_0002 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let flags = channel_flags(&mut r, seed);
                setup_channel(c, &mut r, seed, flags);
            },
        );
    }

    #[test]
    fn snd_utk_lpc_filter_matches() {
        check(
            ee::s14::f_003ce410__generated,
            snd_utk_lpc_filter,
            200,
            &|c, seed| {
                let mut r = Rng(0x7a11_0003 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let blocks = (seed % 4) as i32 - (seed % 9 == 0) as i32;
                c.g[A0][0] = sx(blocks as u32);
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = sx(DATA + 0x100);
                c.g[A3][0] = sx(DATA + 0x200);
                let edge = seed % 5 == 0;
                for i in 0..12 {
                    let v = if edge { r.edge_f32() } else { r.f32(0.5) };
                    c.bus.write32(DATA + 4 * i, v);
                    let h = if edge { r.edge_f32() } else { r.f32(1000.0) };
                    c.bus.write32(DATA + 0x100 + 4 * i, h);
                }
                for i in 0..48 {
                    let x = if edge { r.edge_f32() } else { r.f32(1000.0) };
                    c.bus.write32(DATA + 0x200 + 4 * i, x);
                }
            },
        );
    }

    #[test]
    fn snd_utk_decode_excitation_matches() {
        check(
            ee::s14::f_003cd2b0__generated,
            snd_utk_decode_excitation,
            200,
            &|c, seed| {
                let mut r = Rng(0x7a11_0004 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.pc = exc::START;
                let reader = DATA;
                let input = DATA + 0x100;
                fill(c, &mut r, input, 0x100);
                c.bus.write32(reader, input);
                let buf = r.next_u32();
                c.bus.write32(reader + 4, buf);
                c.bus.write32(reader + 8, 8 + r.next_u32() % 24);
                c.g[A0][0] = sx(reader);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.g[A2][0] = sx(DATA + 0x1000);
                c.g[A3][0] = 1 + (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn snd_eaxa_decode_blocks_matches() {
        check(
            ee::s14::f_003cca08__generated,
            snd_eaxa_decode_blocks,
            240,
            &|c, seed| {
                let mut r = Rng(0x7a11_0005 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                for i in 0..20 {
                    c.bus.write32(EAXA_COEFS + 4 * i, r.f32(2.0));
                }
                let st = DATA;
                c.g[A0][0] = sx(st);
                let blocks = (seed % 4) as i32;
                let left = blocks * EAXA_BLOCK_SAMPLES - 3 * (seed % 5 == 0) as i32;
                c.bus.write32(st + eaxa::LEFT, left as u32);
                let (h1, h2) = if seed % 7 == 0 {
                    (r.edge_f32(), r.edge_f32())
                } else {
                    (r.f32(30000.0), r.f32(30000.0))
                };
                c.bus.write32(st + eaxa::HIST1, h1);
                c.bus.write32(st + eaxa::HIST2, h2);
                let input = DATA + 0x100;
                fill(c, &mut r, input, 0x100);
                c.bus.write32(st + eaxa::INPUT, input);
                c.bus.write32(st + eaxa::OUTPUT, DATA + 0x1000);
                let mut p = input;
                for b in 0..4 {
                    if (seed / 4 + b) % 3 == 0 {
                        c.bus.write8(p, 0xee);
                        p += 61;
                    } else {
                        let h = r.next_u32() as u8;
                        c.bus.write8(p, if h == 0xee { 0x12 } else { h });
                        p += 15;
                    }
                }
            },
        );
    }

    #[test]
    fn snd_tick_stub_matches() {
        leaf(ssx3_ee::s13::f_003c2438__generated, snd_tick_stub, 4);
    }

    #[test]
    fn snd_dsp_resample_set_rate_matches() {
        leaf(
            ssx3_ee::s14::f_003cb538__generated,
            snd_dsp_resample_set_rate,
            25,
        );
    }

    #[test]
    fn snd_utk_set_mode_matches() {
        leaf(ssx3_ee::s14::f_003ce0d8__generated, snd_utk_set_mode, 26);
    }

    #[test]
    fn sndstrm_stop_slot_unsupported_matches() {
        leaf(
            ssx3_ee::s13::f_003c6ce0__generated,
            sndstrm_stop_slot_unsupported,
            27,
        );
    }

    #[test]
    fn snd_eaxa_set_history_matches() {
        leaf(
            ssx3_ee::s14::f_003cd190__generated,
            snd_eaxa_set_history,
            28,
        );
    }

    #[test]
    fn snd_dsp_resample_process_matches() {
        use crate::difftest::fakes;
        check(
            ssx3_ee::s14::f_003cb540__generated,
            snd_dsp_resample_process,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x3cb5_40, 2);
                let o = DATA;
                // Format at +0x1c selects the pass-through half (0x10000) or
                // the resampling half (anything else).
                let fast = seed % 2 == 0;
                c.bus
                    .write32(o + 0x1c, if fast { 0x1_0000 } else { 0x2_0000 });
                c.bus.write16(o + 0x24, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write16(o + 0x26, (seed % 3) as u16);
                c.bus.write32(o + 0x20, 0x1_0000);
                c.bus.write8(o + 0x1a, 0);
                // The resample callback: `o+0x8` points at a one-word slot
                // holding a fake target (outside the function table, so the
                // call lands in the `miss` hook).
                c.bus.write32(o + 0x8, DATA + 0x300);
                c.bus.write32(DATA + 0x300, fakes::FAKE);
                for i in 0..4u32 {
                    c.bus.write32(o + 0x28 + 4 * i, 0x3f80_0000 + i);
                }
                // The hand-written routine's extra input pointers (s0, s1,
                // s2, t7) and the mult operands used for the sample count.
                for (i, reg) in [S0, S1, S2, T7].into_iter().enumerate() {
                    c.g[reg][0] = (DATA + 0x2000 + 0x100 * i as u32) as u64;
                    c.bus.write32(DATA + 0x2000 + 0x100 * i as u32, 0);
                    c.bus.write32(DATA + 0x2004 + 0x100 * i as u32, 0);
                }
                // Arguments: the object, the number of floats to emit (16, so
                // the MMI loop is one 0x40-byte step), the cursor buffer and
                // the output start.
                c.g[A0][0] = o as u64;
                c.g[A1][0] = 16;
                c.g[A2][0] = (DATA + 0x2600) as u64;
                c.g[A3][0] = (DATA + 0x2400) as u64;
                // Unwind through the callback on the first two seeds.
                if seed < 2 {
                    c.bus.write32(fakes::LOG_UNWIND_AT, 0);
                }
            },
        );
    }

    // ----------------------------------------------- SND_UtkDecodeFrame

    /// `SND_UtkDecodeFrame` against the generated body, with the real
    /// callees (bit reader, excitation decoder, interpolation, synthesis)
    /// decoding random speech bits from the game's tables. Seeds alternate
    /// the state's 8-byte alignment (the adaptive-codebook copy's
    /// doubleword loop or its ldl/ldr loop) and full vs reduced bandwidth;
    /// the random bits pick voiced/unvoiced frames and, in reduced frames,
    /// both the zeroing and the interpolation of the uncoded samples. Every
    /// seventh seed uses edge-case floats in the decoder state.
    #[test]
    fn snd_utk_decode_frame_matches() {
        use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};
        use utk_frame as f;

        /// `float[12]` synthesis history after the reflection coefficients.
        const SYNTH: u32 = 0x144;

        check(
            ee::s14::f_003cd878__generated,
            snd_utk_decode_frame,
            120,
            &|c, seed| {
                let mut r = Rng(0x003c_d878 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.pc = 0x003c_d878;
                // 4 mod 8 makes both copy pointers 8-aligned.
                let u = DATA + 0x1000 + 4 * (seed % 2);
                let input = DATA + 0x8000;
                fill(c, &mut r, input, 0x800);
                // Bit reader {ptr, buf, count}.
                c.bus.write32(u, input + r.next_u32() % 8);
                c.bus.write32(u + 4, r.next_u32());
                c.bus.write32(u + 8, 8 + r.next_u32() % 25);
                let reduced = if (seed / 2) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(u + f::REDUCED_BW, reduced);
                c.bus.write32(u + f::VOICED_THRESHOLD, r.next_u32() % 72);
                let edge = seed % 7 == 0;
                let flt = |r: &mut Rng, range: f32| if edge { r.edge_f32() } else { r.f32(range) };
                for i in 0..64 {
                    let v = flt(&mut r, 2.0);
                    c.bus.write32(u + f::FIXED_GAINS + 4 * i, v);
                }
                for i in 0..12 {
                    let v = flt(&mut r, 0.9);
                    c.bus.write32(u + f::RC + 4 * i, v);
                    let h = flt(&mut r, 100.0);
                    c.bus.write32(u + SYNTH + 4 * i, h);
                }
                for i in 0..324 {
                    let v = flt(&mut r, 1000.0);
                    c.bus.write32(u + f::ADAPT_CB + 4 * i, v);
                }
                fill(c, &mut r, u + f::DECODED, 432);
                c.g[A0][0] = sx(u);
            },
        );
    }

    // ---------------------------------------------------- SND_UtkDecode

    /// `SND_UtkDecode` against the generated body, with the real callees
    /// (memcpy, `SND_UtkDecodeFrame` and its bit reader and synthesis on the
    /// game's tables). `seed % 8` picks the case:
    ///
    /// - 0: the stream is finished (`LEFT == 0`): returns 0;
    /// - 1: nothing to write (`n <= 0`, or `LEFT < 0`);
    /// - 2: the request fits in what is left of the frame (one copy);
    /// - 3: mode != 1, up to three frames decoded from random bits;
    /// - 4: mode 1 without a pending patch (random bits; the header byte
    ///   may be a 0xee tag by chance), one frame;
    /// - 5..7: mode 1 with a pending patch: a zero bit stream, run once
    ///   through the real frame decoder first to find where the frame ends,
    ///   then a header planted there: 5 a count <= 0, 6/7 1..24 samples at
    ///   a start that keeps them inside the frame, 7 followed by a 0xee tag.
    ///
    /// `n` against `LEFT` alternates which one bounds the write, and the
    /// state's 8-byte alignment alternates (the frame decoder's copy loops).
    #[test]
    fn snd_utk_decode_matches() {
        use super::utk::*;
        use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};
        use crate::game::lib::snd::utk_frame as f;

        check(
            ee::s14::f_003cde68__generated,
            snd_utk_decode,
            96,
            &|c, seed| {
                let case = seed % 8;
                let mut r = Rng(0x003c_de68 ^ (seed as u64) << 8);
                elf_image(c);
                let u = DATA + 0x1000 + 4 * ((seed / 16) % 2);
                let input = DATA + 0x8000;
                let out_slot = DATA + 0x400;
                let out = DATA + 0x1_0000;
                let patch = case >= 5;

                // The decoder state (bit reader, mode fields, float history),
                // written from its own generator so it can be written twice.
                let state_seed = r.next_u32() as u64;
                let state = |c: &mut Cpu| {
                    let mut s = Rng(state_seed | 1);
                    c.bus.write32(u + BIT_PTR, input + s.next_u32() % 8);
                    c.bus.write32(u + BIT_BUF, s.next_u32());
                    c.bus.write32(u + BIT_COUNT, 8 + s.next_u32() % 25);
                    let reduced = if s.next_u32() % 2 == 0 {
                        0
                    } else {
                        s.next_u32() | 1
                    };
                    c.bus.write32(u + f::REDUCED_BW, reduced);
                    c.bus.write32(u + f::VOICED_THRESHOLD, s.next_u32() % 72);
                    for i in 0..64 {
                        let v = s.f32(2.0);
                        c.bus.write32(u + f::FIXED_GAINS + 4 * i, v);
                    }
                    for i in 0..12 {
                        let v = s.f32(0.9);
                        c.bus.write32(u + f::RC + 4 * i, v);
                        let h = s.f32(100.0);
                        c.bus.write32(u + 0x144 + 4 * i, h);
                    }
                    for i in 0..324 {
                        let v = s.f32(1000.0);
                        c.bus.write32(u + f::ADAPT_CB + 4 * i, v);
                    }
                    fill(c, &mut s, u + DECODED, 432);
                };
                if !patch {
                    fill(c, &mut r, input, 0x800);
                }
                state(c);

                if patch {
                    // Where does the frame end on this (zero) bit stream?
                    c.g[A0][0] = sx(u);
                    c.g[RA][0] = sx(crate::difftest::RA);
                    c.pc = 0x003c_d878;
                    ee::s14::f_003cd878__generated(c);
                    let end = c.bus.read32(u + BIT_PTR);
                    state(c);
                    // Header at end - 1 (that byte is consumed: 0, the start's
                    // high byte): start, count (BE), samples, tag.
                    let count: i32 = if case == 5 {
                        -((r.next_u32() % 50) as i32)
                    } else {
                        1 + (r.next_u32() % 24) as i32
                    };
                    // start <= 255 and count <= 24: the patch stays in DECODED.
                    let start = r.next_u32() % 256;
                    c.bus.write8(end, start as u8);
                    c.bus.write8(end + 1, (count >> 8) as u8);
                    c.bus.write8(end + 2, count as u8);
                    for i in 0..2 * count.max(0) as u32 {
                        let b = r.next_u32() as u8;
                        c.bus.write8(end + 3 + i, b);
                    }
                    // The tag after the samples (a count <= 0 points it back into
                    // the consumed stream: left alone there).
                    if count > 0 {
                        let tag = if case == 7 {
                            0xee
                        } else {
                            r.next_u32() as u8 & 0x7f
                        };
                        c.bus.write8(end + 3 + 2 * count as u32, tag);
                    }
                }

                scramble(c, &mut r);
                c.pc = 0x003c_de68;
                c.g[RA][0] = sx(crate::difftest::RA);

                let avail = match case {
                    2 => 1 + r.next_u32() % 432,
                    _ => r.next_u32() % 433,
                };
                let frames = if case == 3 { 3 } else { 1 };
                let n: i32 = match case {
                    0 | 2 => 1 + (r.next_u32() % avail.max(1)) as i32,
                    1 => -((r.next_u32() % 3) as i32),
                    _ => (avail + 1 + r.next_u32() % (432 * frames)) as i32,
                };
                let left: i32 = match case {
                    0 => 0,
                    1 if seed % 16 < 8 => -1 - (r.next_u32() % 100) as i32,
                    1 => 1 + (r.next_u32() % 100) as i32,
                    2 => n + (r.next_u32() % 100) as i32,
                    _ if seed % 16 < 8 => n + (r.next_u32() % 100) as i32,
                    _ => avail as i32 + 1 + (r.next_u32() % (n as u32 - avail)) as i32,
                };
                let mode = match case {
                    3 => {
                        if seed % 16 < 8 {
                            0
                        } else {
                            2 + r.next_u32() % 4
                        }
                    }
                    _ => 1,
                };
                let pending = if patch { 1 + r.next_u32() % 7 } else { 0 };
                c.bus.write32(u + AVAIL, avail);
                c.bus.write32(u + LEFT, left as u32);
                c.bus.write32(u + MODE, mode);
                c.bus.write32(u + PATCH_PENDING, pending);
                c.bus.write32(u + HEADER, r.next_u32());
                c.bus.write32(u + OUT, r.next_u32());
                c.bus.write32(u + PATCH_COUNT, r.next_u32());
                c.bus.write32(out_slot, out);
                c.g[A0][0] = sx(u);
                c.g[A1][0] = sx(out_slot);
                c.g[A2][0] = sx(n as u32);
            },
        );
    }

    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::ai::ai::ai_render;
    pub(crate) use crate::game::ai::ai::rider_prewind_sub_handle_input;
    pub(crate) use crate::game::ai::ai::rider_spin_sub_update_trick;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_ground_state_compute_boost_accel;
    pub(crate) use crate::game::ai::rider::rider_begin_frame;

    /// SND_DecEaxa_DecodeStream: an empty ADPCM decoder that is refilled
    /// from a real stream (`SNDSTRM_PopBlock` on a one-stream fixture whose
    /// channel queue holds 0-2 blocks of 28 or 56 samples), old and new
    /// codec versions (history seeding), requests satisfied mid-block and
    /// requests the stream cannot satisfy (cleared tail with and without
    /// unreleased samples), and a pending release on entry.
    #[test]
    fn snd_dec_eaxa_decode_stream_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{fill, DATA};
        const SELF: u32 = DATA;
        const DECODER: u32 = DATA + 0x200;
        const STREAM: u32 = DATA + 0x1000;
        const BLOCKS: u32 = DATA + 0x2000;
        const OUT: u32 = DATA + 0x8000;
        /// The stream table (`0x5157b8 + 0x304 + 4 * stream`).
        const STREAMS: u32 = 0x0051_57b8 + 0x304;
        crate::difftest::check(
            ee::s14::f_003ca460__generated,
            snd_dec_eaxa_decode_stream,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ca4_60, 0);
                c.pc = 0x003c_a460;
                c.bus.write32(STREAMS, STREAM);
                // The decoder: empty (no samples left, nothing buffered).
                for k in 0..0x30 {
                    c.bus.write32(DECODER + 4 * k, 0);
                }
                c.bus.write32(SELF + 0x1c, DECODER);
                c.bus.write32(SELF + 0x20, 0);
                c.bus.write32(SELF + 0x24, 0);
                c.bus.write32(SELF + 0x28, r.next_u32() % 100);
                c.bus.write32(SELF + 0x2c, r.next_u32() % 5);
                c.bus.write32(SELF + 0x30, 0);
                let pending = if seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 64
                };
                c.bus.write32(SELF + 0x34, pending);
                c.bus.write8(SELF + 0x38, 0);
                // The stream: no extra channels, channel 0 is 0x42's, a ring of
                // four block descriptors.
                for k in 0..0x40 {
                    c.bus.write32(STREAM + 4 * k, 0);
                }
                c.bus
                    .write8(STREAM + 0x42, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write16(STREAM + 0x40, 4);
                c.bus.write16(STREAM + 0x44, (r.next_u32() % 4) as u16);
                let queued = seed % 3;
                c.bus.write16(STREAM + 0x34, queued as u16);
                c.bus.write16(STREAM + 0x28, 0);
                c.bus.write32(STREAM + 0x4c, 0x1000);
                for d in 0..4u32 {
                    let desc = STREAM + 0x7c + 0x20 * d;
                    let samples = if r.next_u32() % 2 == 0 { 28 } else { 56 };
                    c.bus
                        .write32(desc + 4, samples | (r.next_u32() & 0x8000_0000));
                    c.bus.write32(desc + 8, BLOCKS + 0x100 * d);
                }
                fill(c, &mut r, BLOCKS, 0x100);
                let n = match seed % 5 {
                    0 => 0,
                    1 => 10,
                    2 => 28,
                    _ => 20 + r.next_u32() % 100,
                };
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = n as u64;
                c.g[A3][0] = sx(OUT);
            },
        );
    }

    /// SND_DecUtk_DecodeLoop with the real MicroTalk decoder (game image
    /// tables): requests met by the decoded frame alone, and requests that
    /// run the stream dry (LEFT below n, or 0) so the loop restarts
    /// (`SND_UtkSetState`, `SND_UtkSetInput` with a 20..320-sample loop and
    /// header parsing on random bits) once or several times; n <= 0.
    #[test]
    fn snd_dec_utk_decode_loop_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{elf_image, fill, DATA};
        use crate::game::lib::snd::utk::*;
        use crate::game::lib::snd::utk_frame as f;
        const SELF: u32 = DATA;
        const U: u32 = DATA + 0x1000;
        const INPUT: u32 = DATA + 0x8000;
        const OUT: u32 = DATA + 0x1_0000;
        crate::difftest::check(
            ee::s14::f_003c9520__generated,
            snd_dec_utk_decode_loop,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c95_20, 0);
                elf_image(c);
                c.pc = 0x003c_9520;
                fill(c, &mut r, INPUT, 0x800);
                // The decoder (as the SND_UtkDecode test builds it), mode 0.
                c.bus.write32(U + BIT_PTR, INPUT + r.next_u32() % 8);
                c.bus.write32(U + BIT_BUF, r.next_u32());
                c.bus.write32(U + BIT_COUNT, 8 + r.next_u32() % 25);
                c.bus
                    .write32(U + f::REDUCED_BW, if r.next_u32() % 2 == 0 { 0 } else { 1 });
                c.bus.write32(U + f::VOICED_THRESHOLD, r.next_u32() % 72);
                for i in 0..64 {
                    let v = r.f32(2.0);
                    c.bus.write32(U + f::FIXED_GAINS + 4 * i, v);
                }
                for i in 0..12 {
                    let v = r.f32(0.9);
                    c.bus.write32(U + f::RC + 4 * i, v);
                    let h = r.f32(100.0);
                    c.bus.write32(U + 0x144 + 4 * i, h);
                }
                for i in 0..324 {
                    let v = r.f32(1000.0);
                    c.bus.write32(U + f::ADAPT_CB + 4 * i, v);
                }
                fill(c, &mut r, U + DECODED, 432);
                c.bus.write32(U + MODE, 0);
                let avail = r.next_u32() % 433;
                c.bus.write32(U + AVAIL, avail);
                let n: i32 = match seed % 6 {
                    0 => -((r.next_u32() % 3) as i32),
                    1 => (r.next_u32() % 50) as i32 + 1,
                    _ => 50 + (r.next_u32() % 400) as i32,
                };
                let left = match seed % 4 {
                    0 => 0,
                    1 => avail.min(n.max(0) as u32 / 2),
                    _ => 2000,
                };
                c.bus.write32(U + LEFT, left);
                // The loop: samples 0..len-1 of the data.
                let start = r.next_u32() % 100;
                let len = 20 + r.next_u32() % 300;
                c.bus.write32(SELF + 0x1c, U);
                c.bus.write32(SELF + 0x20, r.next_u32() % 1000);
                c.bus.write32(SELF + 0x24, start);
                c.bus.write32(SELF + 0x28, start + len - 1);
                c.bus
                    .write32(SELF + 0x2c, INPUT + 0x400 + r.next_u32() % 16);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(n as u32);
                c.g[A3][0] = sx(OUT);
            },
        );
    }

    // -- 0x001352a8 RiderSpinSub_UpdateTrick ---------------------------------

    pub(crate) mod update_trick_fixture {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, DATA, GAME_GP};

        /// The objects (zeroed RAM except the fields written below).
        const THIS: u32 = DATA + 0x5_0000;
        const RIDER: u32 = DATA + 0x5_1000;
        const CTRL: u32 = DATA + 0x5_3000;
        const CTRL_VT: u32 = DATA + 0x5_3100;
        const SEQ_LISTS: u32 = DATA + 0x5_3200;
        const SEQ1: u32 = DATA + 0x5_3300;
        const SEQ2: u32 = DATA + 0x5_3400;
        const METRIX: u32 = DATA + 0x5_5000;
        const BE_LIBRARY: u32 = DATA + 0x5_7000;
        const BE_OBJ: u32 = DATA + 0x5_7100;
        const BE_VT: u32 = DATA + 0x5_7200;
        const AUDIO: u32 = DATA + 0x5_9000;
        /// The static animation table (28-byte entries; `+0` the kind
        /// `AnimController_GetLayerAnimEntry` returns).
        const ANIM_TABLE: u32 = 0x0044_6990;
        /// The BAGT trick table (0x14-byte entries of halfword fields).
        const TRICK_TABLE: u32 = 0x0045_aeb8;
        /// The behaviour library's interface objects (`BELibrary_GetInterface`).
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// Fake methods: the behaviour interface getter, and the controller's
        /// "current animation kind" (answers 0x207: `PlayAnim` stops there).
        const FAKE_BE: u32 = FAKE;
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;
        /// Animation ids of layer 2 by kind: a wind-up (0x12), hold loop
        /// (0x13), chain point (0x14), an ended one (2), anything else (0).
        const KINDS: [u32; 5] = [0x12, 0x13, 0x14, 2, 0];

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        /// The fake hook: the animation-kind method answers 0x207.
        fn miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_52a8, 8);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);

            // The behaviour library: the singleton exists, interface 0 is an
            // object whose getter is a fake; the character tables give a
            // divisor (character 0 of player 0).
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE_BE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            // The audio system (the grab sound returns at_2 once: the rider
            // has no sound slot).
            w(c, GAME_GP + 0x410, AUDIO);

            // The rider: `Rider_AddBoost` returns at_2 once (state 3).
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x874, 0);
            w(c, RIDER + 0xb34, 1 + r.next_u32() % 5);
            w(c, RIDER + 0x2f4, r.next_u32() % 10); // uber level
            w(
                c,
                RIDER + 0x2f0,
                if r.next_u32() % 2 == 0 { 0 } else { f(1.0) },
            );
            w(c, RIDER + 0xb2c, r.next_u32() % 4); // uber flags
            w(c, RIDER + 0x784, CTRL);
            w(c, RIDER + 0x790, METRIX);
            w(c, RIDER + 0x86c, 0);
            w(c, METRIX + 0x1ac, RIDER);

            // The animation controller: layer 1 and 2 sequences with random
            // event flags; layer 2 plays an animation of a random kind (or
            // none, 0x1b6).
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            w(c, SEQ_LISTS + 0xc, SEQ1);
            w(c, SEQ_LISTS + 0x14, SEQ2);
            c.bus.write64(SEQ1 + 0xb0, (r.next_u32() & 0xf) as u64);
            let flags = match seed % 5 {
                0 => 0,
                1 => 0xf,
                _ => r.next_u32() & 0xf,
            };
            c.bus.write64(SEQ2 + 0xb0, flags as u64);
            w(c, CTRL + 4, 0x10d + r.next_u32() % 0x1c);
            let id = if r.next_u32() % 8 == 0 {
                0x1b6
            } else {
                0x10d + r.next_u32() % 0x1c
            };
            w(c, CTRL + 8, id);
            if id != 0x1b6 {
                let kind = KINDS[(r.next_u32() % 5) as usize];
                w(c, ANIM_TABLE + id * 0x1c, kind);
            }

            // The trick table: tricks 0..7, each field an animation id or
            // "none" (0x1b6).
            for t in 0..8 {
                for k in 0..6 {
                    let v = if r.next_u32() % 3 == 0 {
                        0x1b6
                    } else {
                        0x10d + r.next_u32() % 0x1c
                    };
                    c.bus.write16(TRICK_TABLE + t * 0x14 + k * 2, v as u16);
                }
            }

            // The spin sub-state: every trick state (6: none of them), the
            // current trick (or -1), the request the same, another or none,
            // held or not.
            let current = if r.next_u32() % 4 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 8
            };
            let trick = match (seed / 5) % 4 {
                0 => u32::MAX,
                1 | 2 => current,
                _ => r.next_u32() % 8,
            };
            let held = if (seed / 3) % 2 == 0 {
                0
            } else {
                1 + r.next_u32() as u64
            };
            w(c, THIS + 0x4, (seed / 7) % 7);
            w(c, THIS + 0x8, current);
            w(c, THIS + 0x58, RIDER);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(trick);
            c.g[A2][0] = held;
            // The generated body (no override slot yet) dispatches on `pc`:
            // enter at_2 the start.
            c.pc = 0x0013_52a8;
        }

        /// `RiderSpinSub_UpdateTrick`: every trick state, the trick
        /// requested, changed or dropped, held or released, the layer's
        /// event flags and animation kinds, trick-table entries with and
        /// without animations, and fake callees that unwind.
        #[test]
        fn rider_spin_sub_update_trick_matches() {
            check(
                ssx3_ee::s01::f_001352a8__generated,
                rider_spin_sub_update_trick,
                300,
                &setup,
            );
        }
    }

    // -- 0x0013c948 RiderGroundState_ComputeBoostAccel -----------------------

    pub(crate) mod boost_accel_fixture {
        use super::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

        const THIS: u32 = DATA + 0x6_0000;
        const RIDER: u32 = DATA + 0x6_1000;
        const PARAMS: u32 = DATA + 0x6_2000;
        const CTRL: u32 = DATA + 0x6_3000;
        const BE_LIBRARY: u32 = DATA + 0x6_7000;
        const BE_OBJ: u32 = DATA + 0x6_7100;
        const BE_VT: u32 = DATA + 0x6_7200;
        /// The behaviour library's interface objects (`BELibrary_GetInterface`).
        const BE_INTERFACES: u32 = 0x0053_08a8;

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        /// A float that is 0 a third of the time, else in [-range, range).
        fn maybe0(r: &mut Rng, range: f32) -> u32 {
            if r.next_u32() % 3 == 0 {
                0
            } else {
                r.f32(range)
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_c948, 2);
            let r = &mut r;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);

            // The behaviour library (interface getter: a fake).
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);

            w(c, THIS + 0x18, RIDER);
            // The rider: on the ground (`+0x214` zero) and racing most of
            // the time; materials below 11, 11..13, 14..16 and >= 17.
            w(c, RIDER + 0x434, r.next_u32() % 20);
            w(c, RIDER + 0x214, if seed % 8 == 0 { r.f32(2.0) } else { 0 });
            w(
                c,
                RIDER + 0x470,
                if seed % 7 == 0 { f(1.0) } else { f(-1.0) },
            );
            c.bus.write16(RIDER + 0x2d4, (r.next_u32() & 0x30) as u16);
            // The heading: either component 0 at_2 times, any quadrant.
            put_vec(
                c,
                RIDER + 0x1b0,
                [maybe0(r, 2.0), maybe0(r, 2.0), r.f32(2.0), r.f32(2.0)],
            );
            w(c, RIDER + 0x4cc, r.f32(4.0));
            w(c, RIDER + 0x2e8, maybe0(r, 2.0));
            w(c, RIDER + 0x2fc, r.f32(2.0));
            w(c, RIDER + 0x1f0, r.f32(1.5));
            w(c, RIDER + 0x3a8, r.f32(3.0));
            w(c, RIDER + 0x220, r.f32(2.0));
            w(c, RIDER + 0x378, r.f32(2.0));
            let speed = if seed % 2 == 0 { 2.0 } else { 60.0 };
            put_vec(
                c,
                RIDER + 0x1e0,
                [r.f32(speed), r.f32(speed), r.f32(speed), r.f32(1.0)],
            );
            w(c, RIDER + 0x86c, 0);
            w(c, RIDER + 0xb34, 1 + r.next_u32() % 5);
            w(c, RIDER + 0x784, CTRL);
            w(c, CTRL + 8, if r.next_u32() % 2 == 0 { 0x16 } else { 0x20 });
            w(c, PARAMS + 0x20, r.f32(3.0));
            w(c, PARAMS + 0x24, r.f32(3.0));

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PARAMS);
            c.pc = 0x0013_c948;
        }

        /// `RiderGroundState_ComputeBoostAccel`: off the ground, finished,
        /// flagged, every heading quadrant and axis case, every material
        /// band, under and over the speed cap, the boost pose.
        #[test]
        fn rider_ground_state_compute_boost_accel_matches() {
            check(
                ssx3_ee::s01::f_0013c948__generated,
                rider_ground_state_compute_boost_accel,
                300,
                &setup,
            );
        }
    }

    // -- 0x003ba888 SND_CentsToPitch -----------------------------------------

    /// `SND_CentsToPitch`: octaves up and down (several), positive and
    /// negative steps, the -255 clamp, a full 64-bit register on entry.
    #[test]
    fn snd_cents_to_pitch_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng};
        check_lohi(
            ssx3_ee::s13::f_003ba888__generated,
            snd_cents_to_pitch,
            300,
            &|c, seed| {
                let mut r = Rng(0x003b_a888 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                fill(c, &mut r, CENTS_PITCH_TABLE, 0x40);
                let cents: i32 = match seed % 6 {
                    0 => (r.next_u32() % 1200) as i32 - 600,
                    1 => 1200 * ((r.next_u32() % 6) as i32 - 3),
                    2 => -1199 - (r.next_u32() % 3) as i32,
                    _ => (r.next_u32() % 20000) as i32 - 10000,
                };
                c.g[A0][0] = if seed % 10 == 9 {
                    (r.next_u32() as u64) << 32 | (cents as u32) as u64
                } else {
                    sx(cents as u32)
                };
                c.pc = 0x003b_a888;
            },
        );
    }

    // -- 0x003c6380 SND_UMul32x32 --------------------------------------------

    /// `SND_UMul32x32`: random and edge operands (zero, all ones, carries
    /// out of the middle bits), junk in the upper register halves.
    #[test]
    fn snd_umul32x32_matches() {
        use crate::difftest::{check_lohi, scramble, Rng};
        check_lohi(
            ssx3_ee::s13::f_003c6380__generated,
            snd_umul32x32,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_6380 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                let pick = |r: &mut Rng| match r.next_u32() % 5 {
                    0 => 0,
                    1 => u32::MAX,
                    2 => 0xffff,
                    3 => 0xffff_0000 | r.next_u32() & 0xffff,
                    _ => r.next_u32(),
                };
                let (a, b) = (pick(&mut r), pick(&mut r));
                c.g[A0][0] = (r.next_u32() as u64) << 32 | a as u64;
                c.g[A1][0] = if seed % 2 == 0 { sx(b) } else { b as u64 };
                c.pc = 0x003c_6380;
            },
        );
    }

    // -- 0x00120f20 Rider_BeginFrame -----------------------------------------

    pub(crate) mod begin_frame_fixture {
        use super::*;
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{DATA, GAME_GP};
        use crate::game::ai::ridermetrix::tests::target_speed_fixture;

        /// The rider of `Rider_UpdateTargetSpeed`'s fixture (random fields,
        /// a course, the back end), plus the objects below (zeroed RAM
        /// except the fields written).
        const RIDER: u32 = target_speed_fixture::RIDER;
        const FRAMES: u32 = DATA + 0x24_0000;
        const TREE2: u32 = DATA + 0x24_0100;
        const CONTROL: u32 = DATA + 0x24_0200;
        const LIGHTS: u32 = DATA + 0x24_0300;
        const MODEL: u32 = DATA + 0x24_1000;
        const BONES: u32 = DATA + 0x24_2000;
        const LIST: u32 = DATA + 0x24_4000;
        const TREE: u32 = DATA + 0x24_5000;
        const PROBE_VT: u32 = DATA + 0x24_6000;

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = target_speed_fixture::setup(c, seed, 3);
            let r = &mut r;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // The game module the fixture's world points at_2: the frame
            // counter, the spatial tree's stamp, the camera control, the
            // light manager.
            let app = c.bus.read32(GAME_GP - 0x848);
            let module = c.bus.read32(app + 0x84);
            let ai = c.bus.read32(RIDER + 0x86c);
            // The rider's turn on every third seed.
            let frame = ai + 3 * (r.next_u32() % 100) + [0, 1, 2][(seed % 3) as usize];
            w(c, module + 0xc, FRAMES);
            w(c, FRAMES + 8, frame);
            w(c, module + 0x20, TREE2);
            let stamp = r.next_u32();
            w(c, TREE2 + 0xa0, stamp);
            w(c, module + 0x34, CONTROL);
            w(c, CONTROL + 0x70, (seed / 3) % 4);
            w(c, module + 0x90, LIGHTS);
            // The overlap list (empty tree), stamped as the world's or not.
            w(c, RIDER + 0x860, LIST);
            w(c, LIST, TREE);
            w(
                c,
                LIST + 4,
                if (seed / 12) % 2 == 0 {
                    stamp
                } else {
                    stamp ^ 1
                },
            );
            w(c, LIST + 8, 0);
            // The model and its bones (hips and board root bone 0..3).
            w(c, RIDER + 0x780, MODEL);
            w(c, MODEL + 0x2c, BONES);
            w(c, RIDER + 0x89c, r.next_u32() % 4);
            w(c, RIDER + 0x8a0, r.next_u32() % 4);
            // A scripted scene on every other seed (probe at_2 the bone or
            // the position); the probe's shape method is a fake.
            w(c, RIDER + 0xac4, (seed / 24) % 2);
            w(c, RIDER + 0xafc, r.next_u32() % 2);
            w(c, RIDER + 0xb00, r.next_u32() % 2);
            w(c, RIDER + 0x6c0, PROBE_VT);
            w(c, PROBE_VT + 0x10, 0);
            w(c, PROBE_VT + 0x14, FAKE);
            // Racing or finished; the finish clock's step.
            w(
                c,
                RIDER + 0x470,
                if seed % 5 == 0 {
                    r.f32(100.0) & 0x7fff_ffff
                } else {
                    (-1.0f32).to_bits()
                },
            );
            w(c, GAME_GP - 0x7920, (1.0f32 / 60.0).to_bits());
            c.g[A0][0] = sx(RIDER);
            c.pc = 0x0012_0f20;
        }

        /// `Rider_BeginFrame`: the rider's turn or not, the list stamped as
        /// the world's or not, every camera control mode, in a scripted
        /// scene (probe at_2 the bone or the position) or not, racing or
        /// finished; real callees (the target speed's fakes may unwind).
        #[test]
        fn rider_begin_frame_matches() {
            check_lohi(
                ssx3_ee::s00::f_00120f20__generated,
                rider_begin_frame,
                300,
                &setup,
            );
        }
    }

    // -- 0x0012e9b8 RiderPrewindSub_HandleInput ------------------------------

    pub(crate) mod prewind_input_fixture {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

        /// The objects (zeroed RAM except the fields written below).
        const THIS: u32 = DATA + 0x7_0000;
        const PAD: u32 = DATA + 0x7_0100;
        const RIDER: u32 = DATA + 0x7_1000;
        const OWNER: u32 = DATA + 0x7_2000;
        const CTRL: u32 = DATA + 0x7_3000;
        const CTRL_VT: u32 = DATA + 0x7_3100;
        const SEQ_LISTS: u32 = DATA + 0x7_3200;
        const SEQ: u32 = DATA + 0x7_3300;
        const MODEL: u32 = DATA + 0x7_4000;
        const BONES: u32 = DATA + 0x7_4100;
        const LIST: u32 = DATA + 0x7_5000;
        const LIST_TREE: u32 = DATA + 0x7_5400;
        const METRIX: u32 = DATA + 0x7_6000;
        const AUDIO: u32 = DATA + 0x7_7000;
        const BE_LIBRARY: u32 = DATA + 0x7_8000;
        const BE_OBJ: u32 = DATA + 0x7_8100;
        const BE_VT: u32 = DATA + 0x7_8200;
        /// The static animation table (28-byte entries; `+0` the kind).
        const ANIM_TABLE: u32 = 0x0044_6990;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// The controller's "current animation kind" method: answers 0x207
        /// (`PlayAnim` stops there).
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;
        /// Layer-2 animation ids: a kind-10 one, id 0x15, an ordinary one.
        const ID_KIND10: u32 = 0x30;
        const ID_PLAIN: u32 = 0x31;

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        fn miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        /// A 6-bit signed axis: centred, inside or outside the dead zone.
        fn axis(r: &mut Rng) -> u32 {
            match r.next_u32() % 4 {
                0 => 0,
                1 => 3,
                2 => 0x3d,
                _ => r.next_u32() & 0x3f,
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0012_e9b8, 8);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);

            // The back end: the library exists, interface getters are fakes.
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            w(c, GAME_GP + 0x410, AUDIO);

            // The pad: never the reset bit; held (bit 13) on two of three
            // seeds; the axes centred, small or large.
            let held = seed % 3 != 0;
            let mut buttons = r.next_u32() & !0x0000_3000;
            buttons &= !(0x3f << 15 | 0x3f << 21);
            buttons |= axis(r) << 15 | axis(r) << 21;
            if held {
                buttons |= 0x2000;
            }
            w(c, PAD, buttons);
            w(c, PAD + 4, (r.next_u32() & !0xfff) | axis(r) | axis(r) << 6);

            w(c, THIS, RIDER);
            // The rider: racing (finished on every 16th seed), main state 2
            // (no rail search), 1 (airborne: no launch) or 0.
            w(
                c,
                RIDER + 0x470,
                if seed % 16 == 15 { f(1.0) } else { f(-1.0) },
            );
            w(c, RIDER + 0x77c, OWNER);
            w(c, OWNER + 0xde0, [2, 2, 1, 0][((seed / 3) % 4) as usize]);
            w(c, RIDER + 0x328, (seed / 12) % 5);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x780, MODEL);
            w(c, MODEL + 0x2c, BONES);
            w(c, RIDER + 0x860, LIST);
            w(c, LIST, LIST_TREE);
            w(c, RIDER + 0x790, METRIX);
            w(c, METRIX + 0x1ac, RIDER);
            for off in [0x220, 0x2a4, 0x2b0] {
                w(c, RIDER + off, r.f32(2.0));
            }
            w(
                c,
                RIDER + 0x2dc,
                if r.next_u32() % 2 == 0 { 0 } else { r.f32(1.0) },
            );
            put_vec(c, RIDER + 0x1e0, [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0]);
            for off in [0x100, 0x110, 0x120, 0x130] {
                put_vec(c, RIDER + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), f(1.0)]);
            }

            // The animation controller: layer 2 plays a kind-10 animation,
            // id 0x15 or an ordinary one.
            w(c, RIDER + 0x784, CTRL);
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            for k in 0..4 {
                w(c, SEQ_LISTS + 8 * k + 4, SEQ);
            }
            w(c, ANIM_TABLE + ID_KIND10 * 0x1c, 10);
            w(c, ANIM_TABLE + ID_PLAIN * 0x1c, 0);
            w(c, ANIM_TABLE + 0x15 * 0x1c, 0);
            let id = [ID_KIND10, 0x15, ID_PLAIN, ID_PLAIN][(seed % 4) as usize];
            w(c, CTRL + 8, id);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            c.pc = 0x0012_e9b8;
        }

        /// `RiderPrewindSub_HandleInput`: finished or not, released (with
        /// and without the launch, every prewind direction) or held (kind
        /// 10, anim 0x15, turn-around, the trick stick's dead zone), real
        /// callees with fakes that unwind.
        #[test]
        fn rider_prewind_sub_handle_input_matches() {
            check(
                ssx3_ee::s00::f_0012e9b8__generated,
                rider_prewind_sub_handle_input,
                300,
                &setup,
            );
        }
    }

    // -- 0x001291e0 AI_Render -------------------------------------------------

    pub(crate) mod ai_render_fixture {
        use super::*;
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check_lohi, Rng, DATA, GAME_GP};

        const THIS: u32 = DATA + 0xa_0000;
        const ACTORS: u32 = DATA + 0xa_1000;
        const RIDERS: u32 = DATA + 0xa_4000;
        const OWNERS: u32 = DATA + 0xb_0000;
        const GFX: u32 = DATA + 0xc_0000;
        const GFX_VT: u32 = DATA + 0xc_2000;
        const PACKET: u32 = DATA + 0xc_3000;
        const APP: u32 = DATA + 0xc_4000;
        const MODULE: u32 = DATA + 0xc_4100;
        const FRAMES: u32 = DATA + 0xc_4200;
        const CONTROL: u32 = DATA + 0xc_4300;
        /// Rider `i` (0x1000 bytes each) and its owner (0x1000 bytes each,
        /// zeroed: every effect is idle).
        fn rider(i: u32) -> u32 {
            RIDERS + 0x1000 * i
        }
        fn actor(i: u32) -> u32 {
            ACTORS + 0x40 * i
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0012_91e0, 4);
            let r = &mut r;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // The graphics manager: a packet buffer, the light-slot
            // allocator and the trail batch's draw method are fakes.
            w(c, GAME_GP - 0x854, GFX);
            w(c, GFX + 0xe84, PACKET);
            w(c, GFX + 0x10d8, GFX_VT);
            for (slot, k) in [(0x390, 0), (0x2c8, 1)] {
                c.bus.write16(GFX_VT + slot, 0);
                w(c, GFX_VT + slot + 4, FAKE + 0x10 * k);
            }
            // The world: the frame counter object (no pulse), the camera
            // control (mode 0 or not, or none).
            w(c, GAME_GP - 0x848, APP);
            w(c, APP + 0x84, MODULE);
            w(c, MODULE + 0xc, FRAMES);
            let control = match r.next_u32() % 3 {
                0 => 0,
                _ => CONTROL,
            };
            w(c, MODULE + 0x34, control);
            w(c, CONTROL + 0x70, r.next_u32() % 2);

            // Eight riders, none visible (`Rider_UpdateVisibility` gives
            // 0): an owner or none, a light slot or not yet (-1).
            for i in 0..8 {
                let p = rider(i);
                let owner = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    OWNERS + 0x1000 * i
                };
                w(c, p + 0x77c, owner);
                w(
                    c,
                    p + 0x888,
                    if r.next_u32() % 2 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    },
                );
                w(c, p + 0x880, r.next_u32() % 8);
                // Every eighth rider has the visible flag, but not state 7.
                w(c, p + 0x884, (r.next_u32() % 8 == 0) as u32);
                if c.bus.read32(p + 0x880) == 7 {
                    w(c, p + 0x880, 6);
                }
            }
            // Actors (some null) pointing at_2 riders.
            let mut next = 0;
            let mut actor_ptr = |c: &mut Cpu, r: &mut Rng| {
                let i = next;
                next += 1;
                if r.next_u32() % 5 == 0 {
                    0
                } else {
                    w(c, actor(i) + 0x18, rider(i % 8));
                    actor(i)
                }
            };

            // The manager: the light-slot list, the human / computer /
            // other actors and their counts (at_2 most 8 riders in all).
            let listed = seed % 5;
            w(c, THIS + 0x78, listed);
            for i in 0..listed {
                w(c, THIS + 0x28 + 4 * i, rider(i));
            }
            let humans = (seed / 5) % 3;
            w(c, THIS + 0x7c, humans);
            for i in 0..2 {
                let a = actor_ptr(c, r);
                w(c, THIS + 0x40 + 4 * i, a);
            }
            let viewed = actor_ptr(c, r);
            w(c, THIS + 0x70, viewed);
            w(c, THIS + 0x88, (seed / 15) % 2);
            w(c, THIS + 0x14, r.next_u32() % 2);
            w(c, THIS + 0x80, r.next_u32() % 4);
            for i in 0..3 {
                let a = actor_ptr(c, r);
                w(c, THIS + 0x48 + 4 * i, a);
            }
            w(c, THIS + 0x8c, r.next_u32() % 2);
            w(c, THIS + 0x84, r.next_u32() % 4);
            for i in 0..3 {
                let a = actor_ptr(c, r);
                w(c, THIS + 0x5c + 4 * i, a);
            }

            c.g[A0][0] = sx(THIS);
            c.pc = 0x0012_91e0;
        }

        /// `AI_Render` with no rider visible: light slots handed out, every
        /// way the drawn riders are chosen (single view by the camera mode,
        /// humans, computers, others, null actors), the per-rider passes on
        /// idle effects, the trail batch; fakes that unwind.
        #[test]
        fn ai_render_matches() {
            check_lohi(ssx3_ee::s00::f_001291e0__generated, ai_render, 300, &setup);
        }
    }

    // ---- SND_EaxaDecode (0x3ccf90) ----

    /// A decoder state with nothing left, or with 0..300 samples left;
    /// pending samples none, some, or negative (a partial block); a request
    /// of 0..200 samples (whole blocks and a remainder); random ADPCM bytes
    /// for the real `SND_EaxaDecodeBlocks` (coefficient table from the ELF).
    pub(crate) fn snd_eaxa_decode_setup(c: &mut Cpu, seed: u32) {
        const S: u32 = 0x00b0_0000;
        const POUT: u32 = S + 0x1000;
        const OUT: u32 = S + 0x1100;
        const READ: u32 = S + 0x2000;
        const SRC: u32 = S + 0x3000;
        crate::difftest::elf_image(c);
        let mut r = Rng(0x3ccf_9000 ^ seed as u64);
        scramble(c, &mut r);
        let remaining = if seed % 6 == 0 { 0 } else { r.next_u32() % 300 };
        let pending = match seed % 4 {
            0 => 0,
            1 => 0u32.wrapping_sub(r.next_u32() % 28),
            _ => 1 + r.next_u32() % 27,
        };
        c.bus.write32(S + 0x4, pending);
        c.bus.write32(S + 0x8, remaining);
        c.bus.write32(S + 0x10, READ);
        c.bus.write32(S + 0x98, r.f32(1.0));
        c.bus.write32(S + 0x9c, r.f32(1.0));
        c.bus.write32(S + 0xa0, SRC);
        fill(c, &mut r, READ, 32);
        fill(c, &mut r, SRC, 0x100);
        c.bus.write32(POUT, OUT);
        c.g[A0][0] = S as u64;
        c.g[A1][0] = POUT as u64;
        c.g[A2][0] = (r.next_u32() % 200) as u64;
        c.pc = 0x003c_cf90;
    }

    #[test]
    fn snd_eaxa_decode_matches() {
        check_lohi(
            ee::s14::f_003ccf90__generated,
            snd_eaxa_decode,
            300,
            &snd_eaxa_decode_setup,
        );
    }

    // ---- SNDSTRM_GetRequestStatus (0x3b7c40) ----

    /// Four stream slots (0x50a0a0, count byte 0x50a92e), some empty; each
    /// stream with a two-request list (+0x11c) and non-zero rates
    /// (+0x14/+0x18); handles that are negative, name a missing stream,
    /// an unknown request, or the current or queued request, active or
    /// not, with random positions.
    pub(crate) fn sndstrm_get_request_status_setup(c: &mut Cpu, seed: u32) {
        const B: u32 = 0x00b0_0000;
        const STREAM_COUNT: u32 = 0x0050_a92e;
        const STREAMS: u32 = 0x0050_a0a0;
        let mut r = Rng(0x003b_7c40 ^ (seed as u64) << 5);
        scramble(c, &mut r);
        c.bus.write8(STREAM_COUNT, 4);
        for i in 0..4u32 {
            let stream = B + 0x1000 * i;
            c.bus
                .write32(STREAMS + 4 * i, if i == 3 { 0 } else { stream });
            c.bus
                .write16(stream + 0x14, (1 + r.next_u32() % 48000) as u16);
            c.bus
                .write16(stream + 0x18, (1 + r.next_u32() % 48000) as u16);
            // Two requests: handles i | 0x100 and i | 0x200.
            let (first, second) = (stream + 0x200, stream + 0x300);
            c.bus.write32(stream + 0x11c, first);
            c.bus.write32(first, second);
            c.bus.write32(second, 0);
            for (req, h) in [(first, 0x100u32), (second, 0x200)] {
                c.bus.write32(req + 0xc, i | h);
                c.bus.write8(req + 0x24, (r.next_u32() % 3 != 0) as u8);
                c.bus.write32(req + 0x14, r.next_u32() % 0x10_0000);
                c.bus.write32(req + 0x18, r.next_u32() % 0x10_0000);
                c.bus.write32(req + 0x1c, r.next_u32() % 0x1_0000);
            }
        }
        let stream = r.next_u32() % 4;
        let handle = match seed % 5 {
            0 => 0x8000_0000 | r.next_u32(),
            1 => stream | 0x300,
            2 => stream | 0x100,
            _ => stream | 0x200,
        };
        c.g[A0][0] = handle as i32 as i64 as u64;
        c.g[A1][0] = (B + 0x8000) as u64;
        fill(c, &mut r, B + 0x8000, 4);
        c.pc = 0x003b_7c40;
    }

    #[test]
    fn sndstrm_get_request_status_matches() {
        check_lohi(
            ee::s13::f_003b7c40__generated,
            sndstrm_get_request_status,
            300,
            &sndstrm_get_request_status_setup,
        );
    }

    /// SND_EeMixBlock: the mixer at 0x515b40 with 0-2 buses (with or
    /// without an effect, declick gain zero or not), the declick bus on or
    /// off, 0-2 voices (playing or not, gains changed or not) whose sources
    /// render through a fake (small sample counts forced), return 0
    /// (`Rider_ReturnZero_25b8`) or -1 (`cFEStateRewardGallery_v26`, so the
    /// voice is finished), sends zero or not, and the post-mix hook set or
    /// not. All mix / effect / output functions are fakes.
    #[test]
    fn snd_ee_mix_block_matches() {
        covered(
            ee::s14::f_003c85d0__generated,
            snd_ee_mix_block,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c85d0, 4);
                c.pc = 0x003c_85d0;
                let m = 0x0051_5b40u32;
                let fake = |k: u32| fakes::FAKE + 0x10 * k;
                for (i, v) in [2u32, 1, 3, 2, 1, 2, 3, 1].iter().enumerate() {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * i as u32, *v);
                }
                let objs = DATA + 0x6000;
                let obj = |c: &mut Cpu, k: u32, f: u32| -> u32 {
                    let o = objs + 0x10 * k;
                    c.bus.write32(o, f);
                    o
                };
                let buses = r.next_u32() % 3;
                let voices = r.next_u32() % 3;
                let gain = |r: &mut Rng| {
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        0.5f32.to_bits()
                    }
                };
                c.bus.write8(m + 4, voices as u8);
                c.bus.write8(m + 5, buses as u8);
                c.bus.write8(m + 0xf, (r.next_u32() % 2) as u8);
                c.bus.write32(m + 8, fake(7));
                c.bus.write32(m + 0x100, gain(&mut r));
                c.bus
                    .write32(m + 0x10, if r.next_u32() % 2 == 0 { 0 } else { fake(6) });
                c.bus.write32(m + 0xb0, DATA + 0x2000);
                c.bus.write32(m + 0xb4, DATA + 0x3000);
                for k in 0..8 {
                    c.bus.write32(DATA + 0x3000 + 4 * k, r.f32(1.0));
                }
                c.bus.write32(m + 0x1e0, fake(5));
                for i in 0..buses {
                    c.bus.write32(m + 0xd0 + 4 * i, DATA + 0x1000 + 0x400 * i);
                    c.bus.write32(m + 0xe8 + 4 * i, gain(&mut r));
                    let fx = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        obj(c, i, fake(1))
                    };
                    c.bus.write32(m + 0x11c + 4 * i, fx);
                    let out = obj(c, 2 + i, fake(2));
                    c.bus.write32(m + 0x104 + 4 * i, out);
                }
                let table = DATA + 0x4000;
                c.bus.write32(m + 0x1dc, table);
                for v in 0..voices {
                    let e = table + 0x60 * v;
                    c.bus.write8(e, if r.next_u32() % 4 == 0 { 1 } else { 2 });
                    let dirty = buses < 2 && v == 0 && r.next_u32() % 2 == 0;
                    c.bus.write8(e + 1, dirty as u8);
                    for k in 0..3 {
                        c.bus.write32(e + 0x1c + 4 * k, gain(&mut r));
                        c.bus.write32(e + 0x4 + 4 * k, gain(&mut r));
                    }
                    c.bus.write32(e + 0x38, gain(&mut r));
                    let src = match r.next_u32() % 4 {
                        0 => 0x0011_25b8,
                        1 => 0x001d_faa0,
                        _ => fake(3),
                    };
                    let o = obj(c, 4 + v, src);
                    c.bus.write32(e + 0x40, o);
                }
                let outs = DATA + 0x5000;
                for i in 0..2 {
                    c.bus.write32(outs + 4 * i, DATA + 0x5100 + 0x200 * i);
                }
                c.g[A0][0] = sx(outs);
                c.g[A1][0] = [16u64, 32][(seed % 2) as usize];
            },
        );
    }

    /// `SND_PanLookup`'s sine table (`0x450000 - 0x2288`).
    pub(crate) const PAN_SINES: u32 = 0x0044_dd78;

    /// `SND_GetPanGains`' per-speaker byte tables (pointers).
    pub(crate) const PAN_TABLES: u32 = 0x0050_ac38;

    /// A channel (snd's fixture) with 1-6 speakers, 0-2 aux sends, 0-7
    /// voices, random smooth/SPU flags, angles, layouts and pan tables.
    pub(crate) fn apply_pan_setup(c: &mut Cpu, seed: u32) {
        use crate::game::lib::snd::tests::{channel_flags, setup_channel};
        let mut r = Rng(0x38e0_0001 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.pc = 0x003c_38e0;
        let flags = channel_flags(&mut r, seed);
        setup_channel(c, &mut r, seed, flags);
        let ch = DATA + (c.g[A0][0] as u32) * 0x8c;
        let speakers = [1u8, 2, 4, 5, 3, 6][(seed % 6) as usize];
        c.bus.write8(pan::STATE + pan::SPEAKERS, speakers);
        c.bus
            .write8(pan::STATE + pan::AUX_SENDS, ((seed / 6) % 3) as u8);
        c.bus.write8(ch + pan::VOICE_COUNT, ((seed / 18) % 8) as u8);
        let mut f = flags & !0x48;
        if r.next_u32() & 1 != 0 {
            f |= 0x40;
        }
        if r.next_u32() & 1 != 0 {
            f |= 0x8;
        }
        c.bus.write16(ch + pan::FLAGS, f);
        for i in 0..8 {
            c.bus
                .write16(ch + pan::VOICES + 2 * i, (6 + r.next_u32() % 18) as u16);
            c.bus
                .write16(ch + pan::VOICE_ANGLES + 2 * i, r.next_u32() as u16);
        }
        c.bus.write16(ch + pan::ANGLE, r.next_u32() as u16);
        fill(c, &mut r, pan::LAYOUTS, 0x30);
        fill(c, &mut r, PAN_SINES, 0x102);
        for i in 0..8 {
            c.bus
                .write32(PAN_TABLES + 4 * i, DATA + 0x4_0000 + 0x100 * i);
        }
        fill(c, &mut r, DATA + 0x4_0000, 0x200);
    }

    /// `SND_ChannelApplyPan`: every speaker layout, the mono, one-voice
    /// and LFE paths, smooth and table pans, the SPU centre/rear split.
    #[test]
    fn snd_channel_apply_pan_matches() {
        check(
            ee::s13::f_003c38e0__generated,
            snd_channel_apply_pan,
            300,
            &apply_pan_setup,
        );
    }

    /// `SND_Tick` over 0-3 tick callbacks (fakes) and 0-4 channels with
    /// random LFO, sweep and envelope states: unallocated / stopped
    /// channels, LFO wrap, sweep up/down to the target or below zero (stop),
    /// envelope ramps, the next segment (finite or forever) and the last
    /// segment (stop), with the real pitch/volume/stop callees (no voices).
    #[test]
    fn snd_tick_matches() {
        use snd_tick::*;
        check_lohi(ee::s13::f_003b55f0__generated, snd_tick, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b55, 3);
            let channels = DATA + 0x1_0000;
            let tables = DATA + 0x2_0000;
            fill(c, &mut r, tables, 0x400);
            c.bus.write32(STATE + TICKS, r.next_u32());
            let callbacks = r.next_u32() % 4;
            c.bus.write8(STATE + TICK_CALLBACKS, callbacks as u8);
            for i in 0..4 {
                c.bus.write32(STATE + CALLBACKS + 4 * i, FAKE + 0x20 * i);
            }
            let n = (seed / 4) % 5;
            c.bus.write16(STATE + CHANNEL_COUNT, n as u16);
            c.bus.write32(STATE + CHANNELS, channels);
            let small = |r: &mut Rng, m: u32| (r.next_u32() % (2 * m + 1)) as i32 - m as i32;
            for i in 0..n {
                let ch = channels + i * CHANNEL_SIZE as u32;
                fill(c, &mut r, ch, CHANNEL_SIZE as u32 / 4);
                let alloc = if r.next_u32() % 5 != 0 {
                    1
                } else {
                    r.next_u32() as u8 % 3
                };
                c.bus.write8(ch + ALLOCATED, alloc);
                let h = if r.next_u32() % 5 != 0 {
                    (r.next_u32() << 8 | i) & 0x7fff_ffff
                } else {
                    r.next_u32() | 0x8000_0000
                };
                c.bus.write32(ch, h);
                c.bus.write8(ch + 0x23, 0); // no voices
                c.bus.write32(
                    ch + 0x74,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        tables + 0x80
                    },
                );
                let lfo = |r: &mut Rng| {
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        tables + 0x100
                    }
                };
                let (pl, vl) = (lfo(&mut r), lfo(&mut r));
                c.bus.write32(ch + PITCH_LFO, pl);
                c.bus.write32(ch + VOL_LFO, vl);
                for off in [PITCH_LFO_POS, PITCH_LFO_LEN, VOL_LFO_POS, VOL_LFO_LEN] {
                    c.bus.write8(ch + off, (r.next_u32() % 6) as u8);
                }
                let step = match r.next_u32() % 3 {
                    0 => 0,
                    1 => small(&mut r, 40),
                    _ => -(1 + r.next_u32() as i32 % 40).abs(),
                };
                c.bus.write32(ch + SWEEP_STEP, step as u32);
                c.bus.write32(ch + SWEEP, small(&mut r, 60) as u32);
                c.bus.write32(ch + SWEEP_TARGET, small(&mut r, 60) as u32);
                c.bus.write32(
                    ch + ENV_STEP,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    },
                );
                c.bus.write32(
                    ch + ENV_TICKS,
                    [1u32, 1, 2, 30][(r.next_u32() % 4) as usize],
                );
                c.bus.write8(ch + ENV_SEGMENTS, (r.next_u32() % 4) as u8);
                c.bus.write8(ch + ENV_SEGMENT, (r.next_u32() % 4) as u8);
                c.bus.write32(ch + ENV_TABLE, tables + 0x200);
            }
            // Segment table: {ticks (some forever / zero), level}.
            for k in 0..8u32 {
                let ticks = match r.next_u32() % 4 {
                    0 => u32::MAX,
                    1 => 0,
                    _ => 1 + r.next_u32() % 100,
                };
                c.bus.write32(tables + 0x200 + 8 * k, ticks);
                c.bus.write32(tables + 0x204 + 8 * k, r.next_u32() % 0x8000);
            }
            c.pc = 0x003b_55f0;
        });
    }

    /// Stream status: unknown / negative stream id; state idle or not;
    /// rate 0 or not; buffered bytes zero or not; the request found or not
    /// (negative request id), its total zero or not; the stream file valid
    /// ("MRTS") or not, channel bytes below or above the 0x3d0900 cap.
    #[test]
    fn sndstrm_get_status_matches() {
        use sndstrm::*;
        check(
            ee::s13::f_003b7b48__generated,
            sndstrm_get_status,
            300,
            &|c, seed| {
                let mut r = Rng(0x7b48 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003b_7b48;
                const STREAM_COUNT: u32 = 0x0050_a92e;
                const STREAM_TABLE: u32 = 0x0050_a0a0;
                const BUFFER_TABLE: u32 = 0x0051_57b8 + 0x304;
                let out = DATA;
                let stream = DATA + 0x1000;
                let req = DATA + 0x2000;
                let buf = DATA + 0x3000;
                let file = DATA + 0x4000;
                let header = DATA + 0x5000;
                fill(c, &mut r, out, 4);
                c.bus.write8(STREAM_COUNT, 4);
                let id: u32 = match seed % 10 {
                    0 => 4 + r.next_u32() % 100,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => r.next_u32() % 4,
                };
                for k in 0..4 {
                    c.bus.write32(STREAM_TABLE + 4 * k, stream);
                }
                let state = if seed % 7 == 2 {
                    0
                } else {
                    1 + r.next_u32() % 5
                };
                c.bus.write32(stream + STATE, state);
                c.bus.write32(stream + REQUEST, req);
                // The request id names this stream in its low byte (or is negative).
                let req_id = if seed % 9 == 4 {
                    r.next_u32() | 0x8000_0000
                } else {
                    (id & 0xff) | (r.next_u32() % 16) << 8
                };
                c.bus.write32(req + REQ_ID, req_id);
                c.bus.write32(req, 0);
                let rate = if seed % 11 == 3 {
                    0
                } else {
                    1 + r.next_u32() % 48000
                };
                c.bus.write16(stream + RATE, rate as u16);
                let slot = r.next_u32() % 4;
                c.bus.write32(stream + BUFFER, slot);
                c.bus.write32(BUFFER_TABLE + 4 * slot, buf);
                let (b0, b1) = if seed % 2 == 0 {
                    (0, 0)
                } else {
                    (r.next_u32() % 0x10000, r.next_u32() % 0x10000)
                };
                c.bus.write32(buf + 0x48, b0);
                c.bus.write32(buf + 0x4c, b1);
                let total = if seed % 13 == 6 {
                    0
                } else {
                    1 + r.next_u32() % 0x80_0000
                };
                c.bus.write32(req + REQ_BYTES, total);
                c.bus.write32(stream + FILE, file);
                c.bus.write32(file, header);
                let magic = if seed % 5 == 1 {
                    r.next_u32()
                } else {
                    0x4d52_5453
                };
                c.bus.write32(header, magic);
                let channel = if r.next_u32() % 2 == 0 {
                    r.next_u32() % 0x40_0000
                } else {
                    r.next_u32()
                };
                c.bus.write32(file + 8, channel);
                c.g[A0][0] = sx(id);
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | out as u64;
            },
        );
    }

    /// SNDSTRM_PopBlock: readers behind/past the write and read marks, the
    /// notify event, ring wrap of both indices, nothing pending, value 0.
    #[test]
    fn sndstrm_pop_block_matches() {
        check(
            ee::s13::f_003c6de0__generated,
            sndstrm_pop_block,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c6d_e000 ^ seed as u64);
                scramble(c, &mut r);
                let id = r.next_u32() % 4;
                let readers = if seed % 11 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                let reader = r.next_u32() % readers.max(1);
                c.g[A0][0] = id as u64;
                c.g[A1][0] = reader as u64;
                c.g[A2][0] = sx(DATA + 0x1000);
                c.g[A3][0] = sx(DATA + 0x1004);
                let q = DATA;
                fill(c, &mut r, q, 0x80);
                put_words(c, SNDSTRM_MAN, &[r.next_u32() % 6]);
                c.bus.write32(SNDSTRM_MAN + SNDSTRM_QUEUES + 4 * id, q);
                c.bus.write8(q + Q_READERS, readers as u8);
                let size = 2 + r.next_u32() % 6;
                c.bus.write16(q + Q_SIZE, size as u16);
                // Write index at the end of the ring now and then (wraps).
                let widx = if seed % 4 == 0 {
                    size - 1
                } else {
                    r.next_u32() % size
                };
                c.bus.write16(q + Q_WIDX, widx as u16);
                let wr = 10 + r.next_u32() % 4;
                let rd = 10 + r.next_u32() % 4;
                c.bus.write32(q + Q_WR, wr);
                c.bus.write32(q + Q_RD, rd);
                let ahead = seed % 3 == 0;
                for i in 0..4 {
                    let p = if ahead {
                        14 + r.next_u32() % 2
                    } else {
                        9 + r.next_u32() % 7
                    };
                    c.bus.write32(q + Q_POS + 4 * i, p);
                    let ri = if seed % 5 == 0 {
                        size - 1
                    } else {
                        r.next_u32() % size
                    };
                    c.bus.write16(q + Q_RIDX + 2 * i, ri as u16);
                    let cnt = if seed % 7 == 0 { 0 } else { r.next_u32() % 3 };
                    c.bus.write16(q + Q_CNT + 2 * i, cnt as u16);
                }
                c.bus
                    .write32(q + Q_NOTIFY, if seed % 2 == 0 { 0 } else { 1 });
                for e in 0..8 {
                    let entry = q + Q_ENTRIES + 0x20 * e;
                    if r.next_u32() % 4 == 0 {
                        c.bus.write32(entry + 8 + 4 * reader, 0);
                    }
                }
            },
        );
    }

    /// SND_ReleaseFinishedVoices over random channel groups: keys zero or
    /// not, timed stops pending or not, seen keys equal or changed (with a
    /// key whose low 23 bits are zero), remaining 0 / -1 or not, 0-3 voices
    /// per channel, mode 2 or not; the real `SND_ChannelRelease` runs on
    /// the same channel table.
    #[test]
    fn snd_release_finished_voices_matches() {
        use super::release::*;
        check_lohi(
            ee::s13::f_003c1298__generated,
            snd_release_finished_voices,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3c_1298, 0);
                c.pc = 0x003c_1298;
                let count_a = r.next_u32() % 5;
                let count_b = r.next_u32() % 4;
                let total = (count_a + count_b).max(1);
                let chans = DATA;
                let status = DATA + 0x4000;
                c.bus.write32(STATE + CHANNELS, chans);
                c.bus.write8(STATE + COUNT_A, count_a as u8);
                c.bus.write8(STATE + COUNT_B, count_b as u8);
                let mode = if r.next_u32() % 2 == 0 {
                    2
                } else {
                    r.next_u32() as u8
                };
                c.bus.write8(STATE + MODE, mode);
                c.bus
                    .write16(crate::sound::snd::CHANNEL_COUNT.0, total as u16);
                c.bus.write32(VOICES + STATUS, status);
                fill(c, &mut r, status - 0x100, 0x500 / 4);
                for i in 0..total {
                    let ch = chans + i * CHANNEL_SIZE;
                    fill(c, &mut r, ch, CHANNEL_SIZE / 4);
                    let stop = if r.next_u32() % 4 != 0 {
                        0xffff
                    } else {
                        r.next_u32() as u16
                    };
                    c.bus.write16(ch + CH_STOP, stop);
                    let n = r.next_u32() % 4;
                    c.bus.write8(ch + CH_VOICE_COUNT, n as u8);
                    for j in 0..4 {
                        c.bus
                            .write16(ch + CH_VOICES + 2 * j, (r.next_u32() % total) as u16);
                    }
                    // SND_ChannelRelease's fields: group, allocated state.
                    c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
                    c.bus.write8(ch + 0x69, (r.next_u32() % 3) as u8);
                }
                let key = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0,
                    1 => 0x0080_0000 * (r.next_u32() % 4),
                    _ => r.next_u32(),
                };
                for i in 0..count_a {
                    let k = key(&mut r);
                    c.bus.write32(status + KEY_A + 8 * i, k);
                    let seen = VOICES + SEEN_A + 8 * i;
                    c.bus.write32(
                        seen,
                        if r.next_u32() % 2 == 0 {
                            k
                        } else {
                            r.next_u32()
                        },
                    );
                    c.bus.write32(
                        seen + 4,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32()
                        },
                    );
                }
                for i in 0..count_b {
                    let k = key(&mut r);
                    c.bus.write32(status + KEY_B + 16 * i, k);
                    let seen = VOICES + SEEN_B + 8 * i;
                    c.bus.write32(
                        seen,
                        if r.next_u32() % 2 == 0 {
                            k
                        } else {
                            r.next_u32()
                        },
                    );
                    c.bus.write32(
                        seen + 4,
                        if r.next_u32() % 2 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32()
                        },
                    );
                }
            },
        );
    }

    use crate::difftest::events::{check_events, schedule, Events};

    use ssx3_ee_rt::Cpu;

    /// SNDSTRM_ConsumeCallback: 1..3 queued requests of the stream, `n`
    /// smaller or larger than the head's bytes left (carrying over to the
    /// next request), positions reaching their end or not (the real
    /// SNDSTRM_RetireRequest moves the head to the free list); sometimes a
    /// 200-round cap is hit (a zero-left request chain with a huge `n`).
    #[test]
    fn sndstrm_consume_callback_matches() {
        check(
            ee::s13::f_003b6b10__generated,
            sndstrm_consume_callback,
            300,
            &|c, seed| {
                let mut r = Rng(0x3b6b10 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003b_6b10;
                let id = r.next_u32() % 8;
                let stream = DATA;
                c.bus.write8(0x0050_a92e, 8); // stream count
                c.bus.write32(0x0050_a0a0 + 4 * id, stream);
                fill(c, &mut r, stream + 0x11c, 7);
                c.g[A2][0] = sx(stream);
                let nreq = 1 + r.next_u32() % 3;
                let looping = seed % 10 == 0;
                let mut prev = 0u32;
                for k in (0..nreq).rev() {
                    let q = DATA + 0x400 + 0x40 * k;
                    fill(c, &mut r, q, 0x10);
                    // A looping list (the last points back to the first).
                    let next = if k == nreq - 1 && looping {
                        DATA + 0x400
                    } else {
                        prev
                    };
                    c.bus.write32(q, next);
                    c.bus.write32(q + 4, 0);
                    c.bus.write32(
                        q + snd_req::OWNER,
                        if looping { 0x8000_0000 | id } else { id },
                    );
                    let pos = r.next_u32() % 0x1000;
                    c.bus.write32(q + snd_req::POS, pos);
                    c.bus.write32(q + snd_req::END, pos + r.next_u32() % 0x800);
                    c.bus.write32(
                        q + snd_req::LEFT,
                        if looping { 0 } else { r.next_u32() % 0x400 },
                    );
                    prev = q;
                }
                c.bus.write32(stream + snd_req::STREAM_ACTIVE, DATA + 0x400);
                c.bus.write32(stream + 0x120, prev);
                c.bus.write32(stream + 0x124, nreq);
                c.bus.write32(stream + 0x128, 0);
                c.bus.write32(stream + 0x12c, 0);
                c.bus.write32(stream + 0x130, 0);
                c.g[A1][0] = sx(match seed % 3 {
                    0 => r.next_u32() % 0x100,
                    1 => r.next_u32() % 0x800,
                    _ => r.next_u32(),
                });
            },
        );
    }

    /// SND_UDiv64by32: random 64-bit dividends against small, large,
    /// sign-extended (upper half set) and zero divisors.
    #[test]
    fn snd_udiv64by32_matches() {
        check(
            ee::s13::f_003c64a8__generated,
            snd_udiv64by32,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_64a8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x003c_64a8;
                let x = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A0][0] = match seed % 4 {
                    0 => x >> 20,
                    _ => x,
                };
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => (r.next_u32() % 1000) as u64 + 1,
                    2 => sx(r.next_u32() | 0x8000_0000),
                    3 => r.next_u32() as u64,
                    _ => c.g[A0][0] >> 32 | 1,
                };
            },
        );
    }

    /// SND_SetVolume fixture: `n` channels in the driver state block (some
    /// sharing a handle group), with short voice lists so the volume and
    /// send callees take their quick paths; a handle that is mostly one of
    /// the channels'.
    pub(crate) fn set_volume_setup(c: &mut Cpu, r: &mut Rng) {
        use super::snd_vol::*;
        let channels = DATA + 0x4_0000;
        let n = r.next_u32() % 12;
        c.bus.write32(STATE + CHANNELS, channels);
        c.bus.write16(STATE + 0x17e, n as u16);
        let volume = r.next_u32() & 0xffff;
        for i in 0..n {
            let ch = channels + i * CHANNEL_SIZE as u32;
            for off in (0..CHANNEL_SIZE as u32).step_by(4) {
                c.bus.write32(ch + off, r.next_u32());
            }
            let h = if r.next_u32() % 5 != 0 {
                (r.next_u32() << 8 | i) & 0x7fff_ffff
            } else {
                r.next_u32()
            };
            c.bus.write32(ch, h);
            c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
            c.bus
                .write8(ch + 0x69, if r.next_u32() % 4 != 0 { 1 } else { 0 });
            // Voice flags with the "voices listed" bit and no voices.
            c.bus.write16(ch + 0x24, 0x8);
            c.bus.write8(ch + 0x23, 0);
            c.bus.write32(ch + 0x78, 0);
            c.bus.write16(ch + SEND_CHANNEL, (r.next_u32() % n) as u16);
            c.bus.write16(
                ch + SEND,
                [0, 1, 0x40, 0xffff][(r.next_u32() % 4) as usize] as u16,
            );
            let v = if r.next_u32() % 6 == 0 {
                volume
            } else {
                r.next_u32() & 0xffff
            };
            c.bus.write32(ch + VOLUME, v << 16);
        }
        let h = if n > 0 && r.next_u32() % 5 != 0 {
            c.bus
                .read32(channels + (r.next_u32() % n) * CHANNEL_SIZE as u32)
        } else {
            r.next_u32()
        };
        c.g[A0][0] = h as i32 as u64;
        c.g[A1][0] = volume as u64;
    }

    crate::difftest::events::driven!(gen_set_volume, ee::s13::f_003b85f0__generated);

    crate::difftest::events::driven!(hand_set_volume, snd_set_volume);

    /// SND_SetVolume: bad handles, single and multi-channel handles, an
    /// unchanged volume (early 0), channels with and without a send; and
    /// with events at the loop's back edge (delivered or unwinding, then
    /// resumed at the loop head).
    #[test]
    fn snd_set_volume_matches() {
        use crate::difftest::events::{check_events, schedule};
        check(
            ee::s13::f_003b85f0__generated,
            snd_set_volume,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x85f0, 0);
                c.pc = 0x003b_85f0;
                set_volume_setup(c, &mut r);
            },
        );
        check_events(gen_set_volume, hand_set_volume, 600, &|c, seed| {
            let mut r = machine(c, seed, 0x85f1, 0);
            set_volume_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_85f0)
        });
    }

    // ---------------------------------------------------------- SNDSTRM_Service

    /// SNDSTRM_Service: zero to four streams (empty slots, closed, open);
    /// states playing (buffer not dry), feeding (no / some free blocks) and
    /// other; prebuffering (no record, record idle, record waiting) or not;
    /// the stream file null (no chunk). The lock and unlock hooks are fakes
    /// (may unwind). Not reached: the dry-buffer restart (StopSlot /
    /// StartPlayback run the whole channel machinery) and HandleChunk (a
    /// chunk needs a live stream file).
    #[test]
    fn sndstrm_service_matches() {
        use sndstrm_service::*;
        const STREAM0: u32 = 0x00a0_0000;
        const BUF0: u32 = 0x00a1_0000;
        const BUFFER_TABLE: u32 = 0x0051_57b8 + 0x304;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x3b_71d8, 6);
            c.bus.write32(0x0044_c46c, FAKE);
            c.bus.write32(0x0044_c470, FAKE + 0x10);
            let n = [0, 1, 2, 3, 4, 4][(seed % 6) as usize];
            c.bus.write8(SND_STATE + STREAM_COUNT, n);
            c.bus.write8(SND_STATE + 0x176, r.next_u32() as u8);
            for i in 0..4u32 {
                let st = STREAM0 + 0x200 * i;
                let present = r.next_u32() % 5 != 0;
                c.bus.write32(STREAMS + 4 * i, if present { st } else { 0 });
                fill(c, &mut r, st, 0x200 / 4);
                c.bus.write32(st + FILE, 0);
                c.bus.write32(st + OPEN, (r.next_u32() % 4 != 0) as u32);
                let state = [1u8, 1, 2, 0, 3, 0xff][(r.next_u32() % 6) as usize];
                c.bus.write8(st + STATE, state);
                let slot = r.next_u32() % 4;
                c.bus.write32(st + SLOT, slot);
                let buf = BUF0 + 0x100 * slot;
                c.bus.write32(BUFFER_TABLE + 4 * slot, buf);
                // Buffered bytes always positive (no restart).
                c.bus.write32(buf + 0x48, 1 + r.next_u32() % 0x1000);
                c.bus.write32(buf + 0x4c, r.next_u32() % 0x1000);
                c.bus.write8(buf + 0x62, (r.next_u32() % 3) as u8);
                for k in 0..4 {
                    c.bus.write16(buf + 0x34 + 2 * k, r.next_u32() as u16);
                }
                // The prebuffer record: none, idle, or waiting (-1).
                let rec = st + 0x180;
                let (ptr, busy, wait) = match r.next_u32() % 3 {
                    0 => (0, 0, 0),
                    1 => (rec, 0, 0),
                    _ => (rec, 1, u32::MAX),
                };
                c.bus.write32(st + 0x134, ptr);
                c.bus.write32(rec + 0x10, busy);
                c.bus.write32(rec + 0x20, wait);
            }
            schedule(c, &mut r, seed, 0x003b_71d8)
        };
        check_events(ee::s13::f_003b71d8__generated, sndstrm_service, 300, &setup);
    }

    // ------------------------------------------------------ SNDSTRM_ReleaseBytes

    /// SNDSTRM_ReleaseBytes: the reader owning the slot or not (negative
    /// ids too), notification wanted or not, a queue already holding 0..7
    /// entries.
    #[test]
    fn sndstrm_release_bytes_matches() {
        use sndstrm_release::*;
        check(
            ee::s13::f_003c7010__generated,
            sndstrm_release_bytes,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c_7010 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x003c_7010;
                let slot = r.next_u32() % 4;
                let buf = DATA + 0x100 * slot;
                c.bus.write32(BUFFERS + 4 * slot, buf);
                fill(c, &mut r, buf, 0x40);
                let reader = [0u8, 1, 2, 0xff][(r.next_u32() % 4) as usize];
                c.bus.write8(buf + READER, reader);
                c.bus
                    .write32(buf + NOTIFY, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(QUEUE, r.next_u32() % 8);
                fill(c, &mut r, QUEUE + 4, 0x10);
                let asked = if seed % 4 == 0 {
                    r.next_u32() % 3
                } else {
                    reader as i8 as u32
                };
                c.g[A0][0] = slot as u64;
                c.g[A1][0] = sx(asked);
                c.g[A2][0] = sx(r.next_u32() % 0x10000);
            },
        );
    }

    /// `SND_DspClip16_Process`: with and without an upstream stage (a fake
    /// that returns a positive count or unwinds, or a leaf returning 0), and 16..64 floats
    /// with values both inside and beyond the 16-bit range.
    #[test]
    fn snd_dsp_clip16_process_matches() {
        check(
            ee::s14::f_003ca6a8__generated,
            snd_dsp_clip16_process,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3ca6a8, 1);
                c.pc = 0x003c_a6a8;
                let o = DATA;
                let stage = DATA + 0x100;
                let input = DATA + 0x1000;
                let out = DATA + 0x2000;
                c.bus
                    .write32(o + 0x8, if seed % 3 == 0 { 0 } else { stage });
                c.bus.write8(o + 0x1a, r.next_u32() as u8);
                // The upstream's method: a fake (returns a positive pointer, or
                // unwinds) or a real `return 0` leaf (0x3ce198: nothing to clip).
                let method = if seed % 3 == 2 {
                    0x003c_e198
                } else {
                    fakes::FAKE
                };
                c.bus.write32(stage, method);
                let count = 16 * (1 + seed % 4);
                for i in 0..count {
                    let v = match r.next_u32() % 4 {
                        0 => r.f32(100.0),
                        1 => r.f32(60000.0),
                        2 => r.edge_f32(),
                        _ => r.f32(1.0e9),
                    };
                    c.bus.write32(input + 4 * i, v);
                }
                fill(c, &mut r, out, 0x40);
                c.g[A0][0] = o as u64;
                c.g[A1][0] = count as u64;
                c.g[A2][0] = input as u64;
                c.g[A3][0] = out as u64;
            },
        );
    }

    /// The SNDSTRM_IsPrebuffering fixture: a stream at DATA with its file
    /// handle (StreamFile magic, or a bad one so the real getters answer 0),
    /// channel bytes below and above the 4e6 cap, read states 0..3, queued
    /// counts negative / zero / positive, and prebuffer state absent or with
    /// zero / random rate and negative / zero / small / large targets. Some
    /// seeds enter at the reset resume point (inside the frame, s0 = the
    /// prebuffer state). `evented` biases toward the two event-checked
    /// branches (large target, read state 2 or 0).
    pub(crate) fn oc238_prebuffer_setup(c: &mut Cpu, seed: u32, evented: bool) -> u32 {
        let mut r = machine(c, seed, 0x3b_70f8, 0);
        let strm = DATA;
        let pre = DATA + 0x400;
        let file = DATA + 0x800;
        let state = DATA + 0xc00;
        c.bus.write32(strm, file);
        let queued = match r.next_u32() % 3 {
            0 => 0,
            1 => 1 + r.next_u32() % 4,
            _ => (1 + r.next_u32() % 4).wrapping_neg(),
        };
        c.bus.write32(strm + 0x130, queued);
        let has_pre = evented || seed % 11 != 0;
        c.bus.write32(strm + 0x134, if has_pre { pre } else { 0 });
        let rate = if !evented && seed % 13 == 0 {
            0
        } else {
            1 + r.next_u32() % 200_000
        };
        c.bus.write32(pre + 0x10, rate);
        let target = if evented {
            0x4000_0000
        } else {
            match r.next_u32() % 6 {
                0 => 0,
                1 => (1 + r.next_u32() % 100).wrapping_neg(),
                2 => 0x7fff_0000,
                _ => r.next_u32() % 5000,
            }
        };
        c.bus.write32(pre + 0x20, target);
        c.bus.write32(file, state);
        c.bus.write32(file + 8, r.next_u32() % 0x50_0000);
        let magic = if !evented && seed % 9 == 0 {
            0x1234_5678
        } else {
            0x4d52_5453
        };
        c.bus.write32(state, magic);
        let read = if evented {
            [0, 2][(r.next_u32() % 2) as usize]
        } else {
            r.next_u32() % 4
        };
        c.bus.write32(state + 0x38, read);
        c.g[A0][0] = strm as u64;
        if seed % 10 == 7 {
            // Resumed at the reset: inside the frame, s0 = the state.
            c.g[ssx3_ee_rt::cpu::reg::SP][0] = (crate::difftest::SP - 0x30) as u64;
            fill(c, &mut r, crate::difftest::SP - 0x30, 12);
            c.g[S0][0] = pre as u64;
            // The saved ra (sp+0) returns to the test's sentinel.
            c.bus
                .write64(crate::difftest::SP - 0x30, crate::difftest::RA as u64);
            0x003b_7184
        } else {
            0x003b_70f8
        }
    }

    /// SNDSTRM_IsPrebuffering: every branch (real StreamFile getters), the
    /// resume entry, and events delivered or unwinding at the two
    /// event-checked branches to the reset.
    #[test]
    fn sndstrm_is_prebuffering_matches() {
        check(
            ee::s13::f_003b70f8__generated,
            sndstrm_is_prebuffering,
            300,
            &|c, seed| {
                let entry = oc238_prebuffer_setup(c, seed, false);
                c.pc = entry;
            },
        );
        crate::difftest::events::check_events(
            oc238_gen_prebuffering,
            oc238_hand_prebuffering,
            120,
            &|c, seed| {
                let entry = oc238_prebuffer_setup(c, seed, true);
                let mut r = Rng(0x3b_70f8 ^ seed as u64);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, entry);
                c.next_event = c.cycles + 4;
                mode
            },
        );
    }

    // ------------------------------------------------- SND_RestoreVu0Context

    /// `SND_RestoreVu0Context`: a saved context of random vectors and random
    /// flag/status words (the real `SND_RestoreVu0Flags` runs).
    #[test]
    fn snd_restore_vu0_context_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ee::s13::f_003c08f8__generated,
            snd_restore_vu0_context,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_08f8, 0);
                c.pc = 0x003c_08f8;
                let ctx = DATA + (r.next_u32() & 0xff0);
                fill(c, &mut r, ctx, 0x82);
                for k in 0..32u32 {
                    c.bus
                        .write32(ctx + 0x10 * k + 4 * (r.next_u32() % 4), r.edge_f32());
                }
                c.g[A0][0] = sx(ctx);
            },
        );
    }

    // ------------------------------------------------------ SND_ParseNextTag

    /// `SND_ParseNextTag`: 0..3 bytes of 0xfc padding, then the end tag, a
    /// bare marker (0xfd / 0xfe) or a data tag with a short (0..4), long
    /// (5..) or extended (0xff + 4-byte big-endian) length.
    #[test]
    fn snd_parse_next_tag_matches() {
        use super::snd_tags::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ee::s13::f_003c7388__generated,
            snd_parse_next_tag,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_7388, 0);
                c.pc = 0x003c_7388;
                let (s, data) = (DATA, DATA + 0x100);
                fill(c, &mut r, s, 8);
                fill(c, &mut r, data, 0x40);
                let mut at = data;
                for _ in 0..seed % 4 {
                    c.bus.write8(at, 0xfc);
                    at += 1;
                }
                let tag = match (seed / 4) % 5 {
                    0 => 0xff,
                    1 => 0xfd,
                    2 => 0xfe,
                    _ => (r.next_u32() % 0xfc) as u8,
                };
                c.bus.write8(at, tag);
                let len = match (seed / 20) % 3 {
                    0 => (r.next_u32() % 5) as u8,
                    1 => 5 + (r.next_u32() % 20) as u8,
                    _ => 0xff,
                };
                c.bus.write8(at + 1, len);
                if len == 0xff {
                    let long = r.next_u32() % 9;
                    c.bus.write8(at + 2, 0);
                    c.bus.write8(at + 3, 0);
                    c.bus.write8(at + 4, 0);
                    c.bus.write8(at + 5, long as u8);
                }
                c.bus.write32(s + P, data);
                c.g[A0][0] = sx(s);
            },
        );
    }

    /// SNDSTRM_FindRequest: four stream slots (one empty), each with a
    /// two-request list; handles that are negative, name the empty slot or an
    /// unknown request, or match the first or second request.
    #[test]
    fn sndstrm_find_request_matches() {
        check(
            ee::s13::f_003b7a70__generated,
            sndstrm_find_request,
            300,
            &|c, seed| {
                const B: u32 = 0x00b0_0000;
                const STREAM_COUNT: u32 = 0x0050_a92e;
                const STREAMS: u32 = 0x0050_a0a0;
                let mut r = Rng(0x003b_7a70 ^ (seed as u64) << 5);
                scramble(c, &mut r);
                c.bus.write8(STREAM_COUNT, 4);
                for i in 0..4u32 {
                    let stream = B + 0x1000 * i;
                    c.bus
                        .write32(STREAMS + 4 * i, if i == 3 { 0 } else { stream });
                    let (first, second) = (stream + 0x200, stream + 0x300);
                    c.bus.write32(stream + STREAM_REQUESTS, first);
                    c.bus.write32(first, second);
                    c.bus.write32(second, 0);
                    c.bus.write32(first + REQUEST_HANDLE, i | 0x100);
                    c.bus.write32(second + REQUEST_HANDLE, i | 0x200);
                }
                let stream = r.next_u32() % 4;
                let handle = match seed % 5 {
                    0 => 0x8000_0000 | r.next_u32(),
                    1 => stream | 0x300,
                    2 => stream | 0x100,
                    _ => stream | 0x200,
                };
                c.g[A0][0] = handle as i32 as i64 as u64;
                c.pc = 0x003b_7a70;
            },
        );
    }

    /// SND_MixAddRamp16: 16 random (edge-case or ordinary) source and
    /// destination samples in separate buffers, or the same buffer (in-place
    /// mixing), and random start/end gains.
    #[test]
    fn snd_mix_add_ramp16_matches() {
        check(
            ee::s14::f_003c7828__generated,
            snd_mix_add_ramp16,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_7828 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                let src = DATA;
                let dst = if seed % 5 == 0 { DATA } else { DATA + 0x100 };
                for i in 0..16 {
                    let v = if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(src + 4 * i, v);
                    if dst != src {
                        let v = if seed % 3 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(2.0)
                        };
                        c.bus.write32(dst + 4 * i, v);
                    }
                }
                c.g[A0][0] = sx(src);
                c.g[A1][0] = sx(dst);
                c.f[12] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.0)
                };
                c.f[13] = if seed % 7 == 1 {
                    r.edge_f32()
                } else {
                    r.f32(1.0)
                };
                c.pc = 0x003c_7828;
            },
        );
    }

    /// `SND_EeMix`: the mixer at 0x515b40 with 0-2 buses (output pointers
    /// copied, then advanced by a random step after every block), no voices,
    /// the declick bus off, declick gains zero, the post-mix hook set or not,
    /// each bus with or without an effect and an output stage (fakes that log
    /// and sometimes unwind); `SND_EeMixBlock` runs for real. Lengths -3..0x700
    /// (no block, one partial, several full and a partial).
    #[test]
    fn snd_ee_mix_matches() {
        covered(
            ee::s14::f_003c8968__generated,
            snd_ee_mix,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c8968, 4);
                c.pc = 0x003c_8968;
                let m = MIXER;
                let fake = |k: u32| FAKE + 0x10 * k;
                let objs = DATA + 0x6000;
                let obj = |c: &mut Cpu, k: u32, f: u32| -> u32 {
                    let o = objs + 0x10 * k;
                    c.bus.write32(o, f);
                    o
                };
                let buses = seed % 3;
                c.bus.write8(m + 4, 0);
                c.bus.write8(m + MIXER_BUS_COUNT, buses as u8);
                c.bus.write16(m + MIXER_OUT_STEP, r.next_u32() as u16);
                c.bus.write8(m + 0xf, 0);
                c.bus.write32(m + 8, fake(7));
                c.bus.write32(m + 0x100, 0);
                c.bus
                    .write32(m + 0x10, if r.next_u32() % 2 == 0 { 0 } else { fake(6) });
                c.bus.write32(m + 0xb0, DATA + 0x2000);
                c.bus.write32(m + 0xb4, DATA + 0x3000);
                c.bus.write32(m + 0x1e0, fake(5));
                c.bus.write32(m + 0x1dc, DATA + 0x4000);
                for i in 0..buses {
                    c.bus.write32(m + 0xd0 + 4 * i, DATA + 0x1000 + 0x800 * i);
                    c.bus.write32(m + 0xe8 + 4 * i, 0);
                    let fx = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        obj(c, i, fake(1))
                    };
                    c.bus.write32(m + 0x11c + 4 * i, fx);
                    let out = obj(c, 2 + i, fake(2));
                    c.bus.write32(m + 0x104 + 4 * i, out);
                }
                let outs = DATA + 0x5000;
                fill(c, &mut r, outs, 4);
                c.g[A0][0] = sx(outs);
                let len = match seed % 5 {
                    0 => (r.next_u32() % 4) as i32 - 3,
                    1 => 0x200,
                    2 => 1 + (r.next_u32() % 0x200) as i32,
                    _ => (r.next_u32() % 0x700) as i32,
                };
                c.g[A1][0] = len as i64 as u64;
            },
        );
    }

    /// `SNDSTRM_RunCallbacks`: zero, negative and 1-6 queued events of both
    /// kinds on random streams (ids 0-7) whose callbacks are fakes (some
    /// unwind).
    #[test]
    fn sndstrm_run_callbacks_matches() {
        use super::sndstrm_cb::*;
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s13::f_003c7080__generated,
            sndstrm_run_callbacks,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_7080, 4);
                c.pc = 0x003c_7080;
                let count = match seed % 5 {
                    0 => 0,
                    1 => (r.next_u32() % 3 + 1).wrapping_neg(),
                    _ => 1 + r.next_u32() % 6,
                };
                c.bus.write32(COUNT, count);
                for i in 0..8u32 {
                    let stream = DATA + 0x100 * i;
                    c.bus.write32(STREAMS + 4 * i, stream);
                    c.bus.write32(stream + VALUE_CB, FAKE + 0x10);
                    c.bus.write32(stream + STREAM_CB, FAKE + 0x20);
                    c.bus.write32(stream + USER, r.next_u32());
                }
                for i in 0..6u32 {
                    let ev = EVENTS + EVENT_SIZE * i;
                    let kind = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 0xffff
                    };
                    c.bus.write16(ev + EV_KIND, kind as u16);
                    c.bus.write16(ev + EV_STREAM, (r.next_u32() % 8) as u16);
                    c.bus.write32(ev + EV_VALUE, r.next_u32());
                }
            },
        );
    }

    /// `SND_RestoreVu0Flags` with random status words (every combination of
    /// the 0x8800 / 8 / 0x80 bits over the four lanes) and random VF1/VF2.
    #[test]
    fn snd_restore_vu0_flags_matches() {
        crate::difftest::check(
            ee::s13::f_003c07a8__generated,
            snd_restore_vu0_flags,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_07a8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_07a8;
                c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    /// SND_RunChangeCallbacks: counter unchanged, changed with an empty
    /// list, and changed with 1-4 callbacks (fakes).
    #[test]
    fn snd_run_change_callbacks_matches() {
        use crate::difftest::fakes::FAKE;
        const NODES: u32 = DATA + 0x20_0000;
        check(
            ee::s13::f_003b54e0__generated,
            snd_run_change_callbacks,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_54e0, 5);
                c.pc = 0x003b_54e0;
                let count = r.next_u32();
                c.bus.write32(SND_GLOBALS + SND_CHANGE_COUNT, count);
                let seen = if seed % 4 == 0 { count } else { r.next_u32() };
                c.bus.write32(SND_SEEN_HI - SND_SEEN_LO, seen);
                let n = (seed / 4) % 5;
                let mut head = 0;
                for i in (0..n).rev() {
                    let node = NODES + 0x20 * i;
                    c.bus.write32(node, head);
                    c.bus.write32(node + 8, FAKE + 0x10 * i);
                    c.bus.write32(node + 0xc, r.next_u32());
                    head = node;
                }
                c.bus.write32(SND_GLOBALS + SND_CB_LIST, head);
            },
        );
    }

    /// SNDSTRM_GetVolume: stream ids out of range / negative / unused
    /// (-8), and a stream whose handle is negative, out of the channel
    /// range, on an inactive channel, on a channel playing another handle
    /// (fallback to the start volume) or playing it (the channel volume).
    #[test]
    fn sndstrm_get_volume_matches() {
        use crate::difftest::{scramble, Rng};
        const STREAMS: u32 = DATA + 0x1000;
        const CHANNELS: u32 = DATA + 0x2000;
        const VOLUMES: u32 = DATA + 0x4000;
        check(
            ee::s13::f_003b7f18__generated,
            sndstrm_get_volume,
            300,
            &|c, seed| {
                let mut r = Rng(0x3b_7f18 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.pc = 0x003b_7f18;
                // Four stream slots, slot 3 unused.
                c.bus.write8(0x0051_0000 - 0x56d2, 4);
                for k in 0..4 {
                    let p = if k == 3 { 0 } else { STREAMS + 0x100 * k };
                    c.bus.write32(0x0051_0000 - 0x5f60 + 4 * k, p);
                }
                let id = (seed % 7) as i64 - 1; // -1..5
                c.g[A0][0] = id as u64;
                // Six channels.
                let globals = SND_GLOBALS;
                c.bus.write16(globals + 0x17e, 6);
                c.bus.write32(globals + 0x1f0, CHANNELS);
                c.bus.write32(0x0051_0000 - 0x5528, VOLUMES);
                for k in 0..3 {
                    let st = STREAMS + 0x100 * k;
                    let ch = r.next_u32() % 8;
                    let handle = match (seed / 7 + k) % 4 {
                        0 => 0x8000_0000 | ch,
                        1 => (r.next_u32() << 8) | ch,
                        _ => 0x100 * (k + 1) | ch,
                    };
                    c.bus.write32(st + STRM_HANDLE, handle);
                    c.bus.write8(st + STRM_VOLUME, r.next_u32() as u8);
                }
                for ch in 0..8 {
                    let cp = CHANNELS + 0x8c * ch;
                    c.bus.write8(cp + 0x69, (r.next_u32() % 3 != 0) as u8);
                    let k = r.next_u32() % 3;
                    c.bus.write32(
                        cp,
                        if r.next_u32() % 4 == 0 {
                            r.next_u32()
                        } else {
                            0x100 * (k + 1) | ch
                        },
                    );
                    c.bus
                        .write16(VOLUMES + 0x8c * ch + 0x3a, r.next_u32() as u16);
                }
            },
        );
    }

    /// SNDSTRM_ParseData: little- and big-endian streams (byte-swapped sample
    /// counts and offsets), 0..4 channels, empty chunks (count 0, or only
    /// the top bit set) released through the real StreamFile_ReleaseChunk
    /// (null file handle: the validation fails at once), non-empty ones
    /// pushed through the real SNDSTRM_PushBlock (no queued blocks), with
    /// the "pushed" flag set or clear.
    #[test]
    fn sndstrm_parse_data_matches() {
        const STREAM: u32 = DATA;
        const STATE: u32 = DATA + 0x200;
        const CHUNK: u32 = DATA + 0x400;
        const BLOCKS: u32 = DATA + 0x800;
        check(
            ee::s13::f_003b6ea8__generated,
            sndstrm_parse_data,
            300,
            &|c, seed| {
                // Scrambled registers, game gp, random bytes around the frame.
                let mut r = machine(c, seed, 0x3b_6ea8, 0);
                c.pc = 0x003b_6ea8;
                let swap = seed % 2 == 1;
                let bytes = |v: u32| if swap { v.swap_bytes() } else { v };
                let count = match seed % 7 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => 1 + r.next_u32() % 0x4000 | (r.next_u32() & 0x8000_0000),
                };
                let channels = if count & 0x7fff_ffff == 0 && seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                let idx = r.next_u32() % 4;
                fill(c, &mut r, STREAM, 0x10);
                c.bus.write32(STREAM, 0);
                c.bus.write32(STREAM + 0x8, idx);
                c.bus.write8(STREAM + SS_CHANNELS, channels as u8);
                c.bus.write32(STREAM + SS_STATE, STATE);
                fill(c, &mut r, STATE, 0x10);
                c.bus.write8(STATE + SS_SWAP, swap as u8);
                c.bus.write8(STATE + SS_PUSHED, (r.next_u32() % 2) as u8);
                fill(c, &mut r, CHUNK, 0x10);
                c.bus.write32(CHUNK + SS_CHUNK_SAMPLES, bytes(count));
                for i in 0..channels {
                    let off = 4 + 0x40 * i + (r.next_u32() % 0x10) * 4;
                    c.bus.write32(CHUNK + SS_CHUNK_OFFSETS + 4 * i, bytes(off));
                }
                // SNDSTRM_PushBlock's stream table and stream (no queue).
                for i in 0..4 {
                    let b = BLOCKS + 0x100 * i;
                    c.bus.write32(0x0051_57b8 + 0x304 + 4 * i, b);
                    fill(c, &mut r, b, 0x40);
                    c.bus.write8(b + 0x62, 0);
                    c.bus.write16(b + 0x46, (r.next_u32() % 4) as u16);
                    c.bus.write16(b + 0x40, 1 + (r.next_u32() % 4) as u16);
                }
                c.g[A0][0] = sx(STREAM);
                c.g[A1][0] = sx(CHUNK);
            },
        );
    }

    /// SND_RunServiceCallbacks: disabled, enabled with a count of 0, a
    /// negative count, or 1..5 fake callbacks (each may unwind).
    #[test]
    fn snd_run_service_callbacks_matches() {
        covered(
            ee::s13::f_003c60e0__generated,
            snd_run_service_callbacks,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_60e0, 5);
                c.pc = 0x003c_60e0;
                let on = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x7f
                };
                c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_ON, on as u8);
                let count: i8 = match seed % 7 {
                    0 => 0,
                    1 => -3,
                    _ => (1 + r.next_u32() % 5) as i8,
                };
                c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_COUNT, count as u8);
                for i in 0..6 {
                    c.bus
                        .write32(SND_GLOBALS_2 + SND_SERVICE_FNS + 4 * i, FAKE + 0x10 * i);
                }
            },
        );
    }

    // -- 0x003ba078 SND_IsVoiceReserved ----------------------------------------

    /// `SND_IsVoiceReserved` against the generated body: counts from
    /// negative to 24, voices that are in the scanned part of the table, past
    /// it, or nowhere (and a voice with junk upper bits that never matches).
    #[test]
    fn snd_is_voice_reserved_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(
            ee::s13::f_003ba078__generated,
            snd_is_voice_reserved,
            300,
            &|c, seed| {
                let mut r = Rng(0x003b_a078 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x003b_a078;
                // A table of distinct signed halfwords.
                for i in 0..32u32 {
                    let v = (i as i16 * 37 - 300) as u16;
                    c.bus.write16(SND_RESERVED_VOICES + 2 * i, v);
                }
                let count: i64 = match seed % 6 {
                    0 => -((r.next_u32() % 4) as i64),
                    _ => (r.next_u32() % 25) as i64,
                };
                c.g[A1][0] = count as u64;
                let k = r.next_u32() % 32;
                c.g[A0][0] = match seed % 4 {
                    0 => r.next_u32() as u64 | 1 << 40,
                    _ => (k as i16 * 37 - 300) as i64 as u64,
                };
            },
        );
    }

    // -- 0x003c6c28 SNDSTRM_GetFreeBlocks --------------------------------------

    /// `SNDSTRM_GetFreeBlocks` against the generated body: slots 0..7 with
    /// 0..8 channels whose block indices are positive, negative (ignored:
    /// the maximum starts at 0) or equal, and signed block counts.
    #[test]
    fn sndstrm_get_free_blocks_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        check(
            ee::s13::f_003c6c28__generated,
            sndstrm_get_free_blocks,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_6c28 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x003c_6c28;
                let slot = r.next_u32() % 8;
                let state = DATA + 0x100 * slot;
                c.g[A0][0] = slot as u64 | (r.next_u32() as u64) << 32;
                c.bus
                    .write32(SNDSTRM_SLOT_BASE + SNDSTRM_SLOT_STATES + 4 * slot, state);
                c.bus.write8(state + 0x62, (r.next_u32() % 9) as u8);
                for k in 0..8 {
                    let v = match r.next_u32() % 4 {
                        0 => r.next_u32() as u16,
                        1 => 7,
                        _ => (r.next_u32() % 40) as u16,
                    };
                    c.bus.write16(state + 0x34 + 2 * k, v);
                }
                let blocks = if seed % 5 == 0 {
                    r.next_u32() as u16
                } else {
                    (r.next_u32() % 64) as u16
                };
                c.bus.write16(state + 0x40, blocks);
            },
        );
    }

    // -- 0x003c6ab8 SNDSTRM_PushBlock ------------------------------------------

    /// `SNDSTRM_PushBlock` against the generated body: slots 0..7 with 0..8
    /// channels, positions below the last ring entry or at/after it (full),
    /// write indices before the end, at the last entry (wrap) and past it,
    /// size words with the top bit set or clear.
    #[test]
    fn sndstrm_push_block_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(
            ee::s13::f_003c6ab8__generated,
            sndstrm_push_block,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_6ab8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x003c_6ab8;
                let slot = r.next_u32() % 8;
                let state = DATA + 0x1000 * slot;
                let block = DATA + 0x2_0000;
                c.g[A0][0] = slot as u64 | (r.next_u32() as u64) << 32;
                c.g[A1][0] = sx(block);
                c.bus
                    .write32(SNDSTRM_SLOT_BASE + SNDSTRM_SLOT_STATES + 4 * slot, state);
                // The ring: 1..16 entries (sometimes a signed junk count), the
                // write index inside, at the end, or past it.
                let entries = if seed % 11 == 0 {
                    r.next_u32() as u16
                } else {
                    (1 + r.next_u32() % 16) as u16
                };
                c.bus.write16(state + 0x40, entries);
                let write = match r.next_u32() % 4 {
                    0 => entries.wrapping_sub(1),
                    1 => entries.wrapping_add((r.next_u32() % 3) as u16),
                    _ => (r.next_u32() % entries.max(1) as u32) as u16,
                };
                c.bus.write16(state + 0x46, write);
                let channels = (r.next_u32() % 9) as u8;
                c.bus.write8(state + 0x62, channels);
                for k in 0..8 {
                    let pos = if r.next_u32() % 8 == 0 {
                        entries.wrapping_sub(1)
                    } else {
                        (r.next_u32() % (entries.max(2) as u32 - 1)) as u16
                    };
                    c.bus.write16(state + 0x34 + 2 * k, pos);
                }
                c.bus.write32(state + 4, r.next_u32());
                c.bus.write32(state + 0x48, r.next_u32());
                fill(c, &mut r, block, 12);
            },
        );
    }

    // ------------------------------------------------------- SNDSTRM_SetVolume

    /// `SNDSTRM_SetVolume`: stream indices in range (one slot empty), past
    /// the count and negative; the stream's sound handle and the channels
    /// come from `SND_SetVolume`'s own fixture (bad handles, unchanged
    /// volume, one or more channels), which runs for real.
    #[test]
    fn sndstrm_set_volume_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        use crate::game::lib::snd::tests::set_volume_setup;
        const STREAM_COUNT: u32 = 0x0050_a92e;
        const STREAMS: u32 = 0x0050_a0a0;
        check(
            ee::s13::f_003b7e70__generated,
            sndstrm_set_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_7e70, 0);
                c.pc = 0x003b_7e70;
                set_volume_setup(c, &mut r);
                let handle = r32(c, A0);
                c.bus.write8(STREAM_COUNT, 4);
                for i in 0..4u32 {
                    let stream = DATA + 0x8_0000 + 0x200 * i;
                    c.bus
                        .write32(STREAMS + 4 * i, if i == 2 { 0 } else { stream });
                    c.bus.write32(stream + STREAM_SND_HANDLE, handle);
                    c.bus.write8(stream + STREAM_VOLUME, r.next_u32() as u8);
                }
                c.g[A0][0] = match seed % 8 {
                    0 => sx(0x8000_0000 | r.next_u32()),
                    1 => 4 + (r.next_u32() % 100) as u64,
                    _ => (r.next_u32() % 4) as u64,
                };
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

    /// SND_GetVolume through the real SND_HandleToChannel: negative handle,
    /// channel out of range, inactive channel, handle mismatch, or a
    /// playing channel whose volume is read.
    #[test]
    fn snd_get_volume_matches() {
        crate::difftest::check_lohi(
            ee::s13::f_003b86c0__generated,
            snd_get_volume,
            240,
            &|c, seed| {
                let mut r = Rng(0x003b_86c0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003b_86c0;
                // The SND state: channel count (+0x17e) and channel array (+0x1f0,
                // 0x8c bytes per channel: handle at +0, active byte at +0x69).
                let snd = 0x0050_a8e8;
                let channels = DATA + 0x1000;
                let params = DATA + 0x4000;
                c.bus.write16(snd + 0x17e, 8);
                c.bus.write32(snd + 0x1f0, channels);
                c.bus.write32(SND_CHANNEL_PARAMS, params);
                let ch = r.next_u32() % 10; // 8, 9: out of range
                let handle = (r.next_u32() & 0x7fff_ff00) | ch;
                c.g[A0][0] = match seed % 6 {
                    0 => (r.next_u32() | 0x8000_0000) as i32 as i64 as u64,
                    _ => handle as u64,
                };
                if ch < 8 {
                    let at = channels + ch * 0x8c;
                    c.bus.write8(at + 0x69, if seed % 6 == 1 { 0 } else { 1 });
                    c.bus.write32(
                        at,
                        if seed % 6 == 2 {
                            handle ^ 0x100
                        } else {
                            handle
                        },
                    );
                    c.bus
                        .write16(params + ch * 0x8c + 0x3a, r.next_u32() as u16);
                }
            },
        );
    }

    /// `SND_ChannelStartVoices`: SPU / IOP DSP / EE channels (snd's channel
    /// fixture, `seed % 3`) with 0-4 voices, output mode 2 or not, a random
    /// start descriptor; the real apply/start/activate callees run (the EE
    /// codec table and stretch words are zero, so `SND_EeVoiceStart` takes
    /// its short path).
    #[test]
    fn snd_channel_start_voices_matches() {
        use crate::difftest::fill;
        use crate::game::lib::snd::tests::{channel_flags, setup_channel};
        /// `SND_GetPanGains`' per-speaker byte tables, the speaker layouts
        /// and the pan sine table (as snd's pan fixture fills them).
        const PAN_TABLES: u32 = 0x0050_ac38;
        const PAN_LAYOUTS: u32 = 0x0050_ab20;
        const PAN_SINES: u32 = 0x0044_dd78;
        const DESC: u32 = DATA + 0x5_0000;
        const SAMPLE_BASE: u32 = DATA + 0x5_1000;
        check(
            ee::s13::f_003c3ee0__generated,
            snd_channel_start_voices,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_3ee0, 0);
                c.pc = 0x003c_3ee0;
                let flags = channel_flags(&mut r, seed);
                setup_channel(c, &mut r, seed, flags);
                let ch_id = c.g[A0][0];
                fill(c, &mut r, PAN_LAYOUTS, 0x30);
                fill(c, &mut r, PAN_SINES, 0x102);
                for i in 0..8 {
                    c.bus
                        .write32(PAN_TABLES + 4 * i, DATA + 0x4_0000 + 0x100 * i);
                }
                fill(c, &mut r, DATA + 0x4_0000, 0x200);
                // The start descriptor: random words, the sample base pointer,
                // no stretch (SND_EeVoiceStart would allocate a stretcher).
                fill(c, &mut r, DESC, 0x30);
                c.bus.write32(DESC + SV_DESC_BASE_PTR, SAMPLE_BASE);
                c.bus.write32(SAMPLE_BASE, r.next_u32());
                for i in 0..6 {
                    c.bus.write32(DESC + SV_DESC_STRETCH + 4 * i, 0);
                }
                c.bus.write32(SV_SYNC + SV_SYNC_KEY_SEQ, r.next_u32());
                c.g[A0][0] = sx(DESC);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = ch_id;
            },
        );
    }

    /// Slot table entries point at streams in DATA with random buffer counts.
    #[test]
    fn sndstrm_get_buffered_bytes_matches() {
        check(
            ee::s13::f_003c6c98__generated,
            sndstrm_get_buffered_bytes,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c6c_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_6c98;
                let slot = r.next_u32() % 8;
                // Upper bits of a0 are ignored by the `sll`.
                c.g[A0][0] = (r.next_u32() as u64) << 32 | slot as u64;
                let stream = DATA + 0x100 * (slot + 1);
                c.bus.write32(SNDSTRM_SLOT_TABLE + 4 * slot, stream);
                c.bus.write32(stream + SNDSTRM_BUFFERED_A, r.next_u32());
                c.bus.write32(stream + SNDSTRM_BUFFERED_B, r.next_u32());
            },
        );
    }

    /// `SND_GetPanGains`: 0..6 output channels, random tables and pans
    /// (the high pan bits are masked off).
    #[test]
    fn snd_get_pan_gains_matches() {
        check(
            ssx3_ee::s13::f_003bc4c0__generated,
            snd_get_pan_gains,
            300,
            &|c, seed| {
                let mut r = Rng(0x3bc4_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003b_c4c0;
                let channels = (seed % 7) as u8;
                c.bus.write8(SND_PAN_STATE + SND_PAN_CHANNELS, channels);
                for ch in 0..6u32 {
                    let table = DATA + 0x1000 * (ch + 1);
                    c.bus.write32(SND_PAN_TABLES + 4 * ch, table);
                    fill(c, &mut r, table, 0x40);
                }
                c.g[A0][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.g[A1][0] = sx(DATA + 0x100);
            },
        );
    }

    use crate::game::lib::snd::tests as snd_tests;

    // -- 0x003d85c8 SPCH_MatchConditions ---------------------------------------

    /// `SPCH_MatchConditions` against the generated body: condition counts
    /// -1..5, actual values in and out of range, masks that accept or not,
    /// kinds 0/0xfe (always), value-table indices, and 0xff (a context table
    /// word, or a bank data byte via the real `SPCH_FindBankDataPtr` over
    /// the eight bank slots at 0x44ffb8, found or not).
    #[test]
    fn spch_match_conditions_matches() {
        use super::spch_match::*;
        use crate::difftest::{check, scramble, Rng, DATA};
        const CTX: u32 = DATA + 0x1000;
        const COND: u32 = DATA + 0x2000;
        const VALUES: u32 = DATA + 0x3000;
        const ACTUAL: u32 = DATA + 0x4000;
        const BANKS: u32 = DATA + 0x5000;
        const BANK_SLOTS: u32 = 0x0045_0000 - 0x48;
        check(
            ee::s14::f_003d85c8__generated,
            spch_match_conditions,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_85c8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_85c8;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Context: two size bytes, then its value table.
                c.bus.write8(CTX + CTX_SIZE_A, (r.next_u32() % 0x40) as u8);
                c.bus.write8(CTX + CTX_SIZE_B, (r.next_u32() % 0x20) as u8);
                for i in 0..0x80 {
                    let v = if r.next_u32() % 3 == 0 {
                        r.next_u32()
                    } else {
                        0xf
                    };
                    w(c, CTX + 8 + 4 * i, v);
                }
                let count = [1, 2, 3, 4, 4, 4, 0, 5, 0xff][(r.next_u32() % 9) as usize];
                c.bus.write8(COND + COND_COUNT, count);
                for i in 0..4u32 {
                    let kind =
                        [0, 0xfe, 0xff, 0xff, 0xff, r.next_u32() % 8][(r.next_u32() % 6) as usize];
                    c.bus.write8(COND + COND_KIND + i, kind as u8);
                    c.bus.write8(
                        COND + COND_OFFSET + i,
                        ((r.next_u32() % 2) << 7 | r.next_u32() % 8) as u8,
                    );
                    let mask = if r.next_u32() % 5 == 0 {
                        r.next_u32()
                    } else {
                        u32::MAX
                    };
                    w(c, COND + COND_MASKS + 4 * i, mask);
                    let actual = if r.next_u32() % 10 == 0 {
                        0x20 + r.next_u32() % 0xe0
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write8(ACTUAL + i, actual as u8);
                }
                // Values: the bank id (bytes 2, 3) overlapping values[0].
                for i in 0..8 {
                    let v = if r.next_u32() % 3 == 0 {
                        r.next_u32()
                    } else {
                        0xf
                    };
                    w(c, VALUES + 4 * i, v);
                }
                let id = c.bus.read32(VALUES);
                for j in 0..8u32 {
                    let bank = BANKS + 0x100 * j;
                    w(
                        c,
                        BANK_SLOTS + 8 * j,
                        if r.next_u32() % 3 == 0 { 0 } else { bank },
                    );
                    let hit = r.next_u32() % 4 == 0;
                    c.bus.write8(
                        bank + 9,
                        if hit {
                            (id >> 16) as u8
                        } else {
                            r.next_u32() as u8
                        },
                    );
                    c.bus.write8(
                        bank + 8,
                        if hit {
                            (id >> 24) as u8
                        } else {
                            r.next_u32() as u8
                        },
                    );
                    c.bus.write16(bank + 0x10, (r.next_u32() % 16) as u16);
                    for k in 0..0x40 {
                        c.bus.write8(bank + 0x18 + k, (r.next_u32() % 4) as u8);
                    }
                }
                c.g[A0][0] = sx(CTX);
                c.g[A1][0] = sx(COND);
                c.g[A2][0] = sx(VALUES);
                c.g[A3][0] = sx(ACTUAL);
            },
        );
    }

    /// `SND_ChannelApplySend` over SPU channels (output mode 2 or not, send
    /// level 0 set or clear), IOP and EE channels, 0..4 voices.
    #[test]
    fn snd_channel_apply_send_matches() {
        check(
            ee::s13::f_003c27e0__generated,
            snd_channel_apply_send,
            240,
            &|c, seed| {
                let mut r = Rng(0x5e4d_0001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let flags = snd_tests::channel_flags(&mut r, seed);
                snd_tests::setup_channel(c, &mut r, seed, flags);
                let ch = DATA + (c.g[A0][0] as u32) * snd::chan::SIZE;
                for i in 0..4 {
                    let level = if (seed / 5 + i) % 3 == 0 {
                        0
                    } else {
                        r.next_u32() as u16
                    };
                    c.bus.write16(ch + SEND_LEVELS + 2 * i, level);
                }
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.pc = 0x003c_27e0;
            },
        );
    }

    /// `SNDSTRM_HandleChunk` with `SCDl` chunks (the real
    /// `SNDSTRM_ParseData`, fixture as in its own test: empty chunks
    /// released through a null file handle, others pushed through the real
    /// `SNDSTRM_PushBlock`) and other tags (the real
    /// `StreamFile_ReleaseChunk`, null file handle). The `SCHl` branch (the
    /// real `SNDSTRM_ParseHeader`, a large parser) is not reached.
    #[test]
    fn sndstrm_handle_chunk_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::fill;
        use crate::game::lib::snd::{
            SS_CHANNELS, SS_CHUNK_OFFSETS, SS_CHUNK_SAMPLES, SS_PUSHED, SS_STATE, SS_SWAP,
        };
        const STREAM: u32 = DATA;
        const STATE: u32 = DATA + 0x200;
        const CHUNK: u32 = DATA + 0x400;
        const BLOCKS: u32 = DATA + 0x800;
        check(
            ee::s13::f_003b7080__generated,
            sndstrm_handle_chunk,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_7080, 0);
                c.pc = 0x003b_7080;
                let slot = r.next_u32() % 4;
                for i in 0..4 {
                    c.bus.write32(
                        SNDSTRM_STREAMS + 4 * i,
                        if i == slot { STREAM } else { r.next_u32() },
                    );
                }
                let swap = seed % 2 == 1;
                let bytes = |v: u32| if swap { v.swap_bytes() } else { v };
                let count = match seed % 7 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => 1 + r.next_u32() % 0x4000 | (r.next_u32() & 0x8000_0000),
                };
                let channels = if count & 0x7fff_ffff == 0 && seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                let idx = r.next_u32() % 4;
                fill(c, &mut r, STREAM, 0x10);
                c.bus.write32(STREAM, 0);
                c.bus.write32(STREAM + 0x8, idx);
                c.bus.write8(STREAM + SS_CHANNELS, channels as u8);
                c.bus.write32(STREAM + SS_STATE, STATE);
                fill(c, &mut r, STATE, 0x10);
                c.bus.write8(STATE + SS_SWAP, swap as u8);
                c.bus.write8(STATE + SS_PUSHED, (r.next_u32() % 2) as u8);
                fill(c, &mut r, CHUNK, 0x10);
                let tag = match seed % 3 {
                    0 => r.next_u32() & !0x0400_0000,
                    _ => SNDSTRM_TAG_DATA as u32,
                };
                c.bus.write32(CHUNK, tag);
                c.bus.write32(CHUNK + SS_CHUNK_SAMPLES, bytes(count));
                for i in 0..channels {
                    let off = 4 + 0x40 * i + (r.next_u32() % 0x10) * 4;
                    c.bus.write32(CHUNK + SS_CHUNK_OFFSETS + 4 * i, bytes(off));
                }
                // SNDSTRM_PushBlock's stream table and stream (no queue).
                for i in 0..4 {
                    let b = BLOCKS + 0x100 * i;
                    c.bus.write32(0x0051_57b8 + 0x304 + 4 * i, b);
                    fill(c, &mut r, b, 0x40);
                    c.bus.write8(b + 0x62, 0);
                    c.bus.write16(b + 0x46, (r.next_u32() % 4) as u16);
                    c.bus.write16(b + 0x40, 1 + (r.next_u32() % 4) as u16);
                }
                c.g[A0][0] = slot as u64;
                c.g[A1][0] = sx(CHUNK);
            },
        );
    }

    /// `SND_TimerServiceCallback` through the real `SND_RunServiceCallbacks`:
    /// service disabled, a zero or negative count, or 1..5 fake callbacks
    /// (each may unwind).
    #[test]
    fn snd_timer_service_callback_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::game::lib::snd::{
            SND_GLOBALS_2, SND_SERVICE_COUNT, SND_SERVICE_FNS, SND_SERVICE_ON,
        };
        covered(
            ee::s13::f_003b5450__generated,
            snd_timer_service_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_5450, 5);
                c.pc = 0x003b_5450;
                let on = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x7f
                };
                c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_ON, on as u8);
                let count: i8 = match seed % 7 {
                    0 => 0,
                    1 => -3,
                    _ => (1 + r.next_u32() % 5) as i8,
                };
                c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_COUNT, count as u8);
                for i in 0..6 {
                    c.bus
                        .write32(SND_GLOBALS_2 + SND_SERVICE_FNS + 4 * i, FAKE + 0x10 * i);
                }
            },
        );
    }

    /// `SND_ChannelApplyFxEnable` over SPU channels (output mode 2 or not,
    /// the switch on or off, 0..4 voices), and IOP and EE channels (the
    /// real `SND_ChannelApplyVolume`, its EE voices through the real
    /// `SND_EeVoiceSetVolume`).
    #[test]
    fn snd_channel_apply_fx_enable_matches() {
        check(
            ee::s13::f_003c2ec8__generated,
            snd_channel_apply_fx_enable,
            240,
            &|c, seed| {
                let mut r = Rng(0x5e4d_0002 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let flags = snd_tests::channel_flags(&mut r, seed);
                snd_tests::setup_channel(c, &mut r, seed, flags);
                let ch = DATA + (c.g[A0][0] as u32) * snd::chan::SIZE;
                let on = if (seed / 5) % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u8 | 1
                };
                c.bus.write8(ch + FX_ENABLE, on);
                c.pc = 0x003c_2ec8;
            },
        );
    }

    /// SND_EeVoiceSetVolume: a voice table in test memory, random voice,
    /// channel and volume.
    #[test]
    fn snd_ee_voice_set_volume_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        check_lohi(
            ee::s14::f_003c8560__generated,
            snd_ee_voice_set_volume,
            200,
            &|c, seed| {
                let mut r = Rng(0x003c_8560 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x003c_8560;
                c.bus.write32(SND_SET_VOL_STATE + SND_SET_VOL_VOICES, DATA);
                c.g[A0][0] = (r.next_u32() % 48) as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    /// SND_EaxaSetInput: null or non-null data, idle or busy decoder.
    #[test]
    fn snd_eaxa_set_input_matches() {
        check(
            ee::s14::f_003ccf60__generated,
            snd_eaxa_set_input,
            200,
            &|c, seed| {
                let mut r = Rng(0x003c_cf60 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let dec = DATA + 0x40;
                fill(c, &mut r, dec, 0x30);
                c.bus
                    .write32(dec + 0x8, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                c.g[A0][0] = sx(dec);
                if seed % 5 == 0 {
                    c.g[A1][0] = 0;
                }
                c.pc = 0x003c_cf60;
            },
        );
    }

    /// SND_ChannelStopVoices: SPU (output mode 2 or not), IOP DSP and EE
    /// channels with 0..4 voices; the real SND_ChannelRelease runs on a
    /// table of 10 channels, the real SND_EeVoiceStop on EE voices whose
    /// callback list holds one element (callback none or a fake that
    /// unwinds on some seeds) and SND_MemFree takes its empty-heap exit.
    #[test]
    fn snd_channel_stop_voices_matches() {
        const STATE: u32 = 0x0050_a8e8;
        const SYNC: u32 = 0x0050_ad00;
        const EE_VOICES: u32 = 0x0051_5b40;
        const CHANNEL_SIZE: u32 = 0x8c;
        check(
            ee::s13::f_003c2440__generated,
            snd_channel_stop_voices,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3c_2440, 1);
                let chans = DATA;
                let count = 10u32;
                c.bus.write32(STATE + 0x1f0, chans);
                c.bus.write16(STATE + 0x17e, count as u16);
                for i in 0..count {
                    let ch = chans + i * CHANNEL_SIZE;
                    fill(c, &mut r, ch, CHANNEL_SIZE / 4);
                    c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
                    c.bus.write8(ch + 0x69, (r.next_u32() % 3) as u8);
                }
                let voice_base = r.next_u32() % 4;
                let ee_base = r.next_u32() % 3;
                c.bus.write8(STATE + 0x30, voice_base as u8);
                c.bus.write8(STATE + 0x2f, ee_base as u8);
                let mode = if seed % 2 == 0 {
                    2
                } else {
                    r.next_u32() as u8 | 1
                };
                c.bus.write8(STATE + 0x49, mode);
                c.bus.write32(SYNC + 0xcb0, DATA + 0x1_0000);
                // SND_MemFree's heap: no blocks (it returns at once).
                c.bus.write32(STATE + 0x234, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x14, 0);
                // EE voices (0x60 bytes): random gains, one callback element.
                c.bus.write32(EE_VOICES + 0x1dc, DATA + 0x2_0000);
                c.bus.write8(EE_VOICES + 5, (r.next_u32() % 3) as u8);
                for i in 0..0x50 {
                    c.bus.write32(EE_VOICES + 0xe8 + 4 * i, r.f32(1.0));
                }
                for v in 0..4 {
                    let voice = DATA + 0x2_0000 + 0x60 * v;
                    for i in 0..0x18 {
                        c.bus.write32(voice + 4 * i, r.f32(1.0));
                    }
                    let elem = DATA + 0x3_0000 + 0x20 * v;
                    c.bus.write32(voice + 0x40, elem);
                    c.bus.write32(
                        elem + 4,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            fakes::FAKE
                        },
                    );
                    c.bus.write32(elem + 8, 0);
                }
                // The channel: SPU / IOP / EE by seed % 3, 0..4 voices.
                let index = r.next_u32() % 3;
                let ch = chans + index * CHANNEL_SIZE;
                let other = r.next_u32() as u16 & !0x108;
                let flags = match seed % 3 {
                    0 => other | 0x8 | (r.next_u32() as u16 & 0x100),
                    1 => other | 0x100,
                    _ => other,
                };
                c.bus.write16(ch + 0x24, flags);
                c.bus.write8(ch + 0x23, (seed / 3 % 5) as u8);
                for i in 0..4 {
                    let voice = if seed % 3 == 2 {
                        // Distinct EE voices (a stopped voice's list is empty).
                        voice_base + ee_base + (i + seed / 15) % 4
                    } else {
                        r.next_u32() % count
                    };
                    c.bus.write16(ch + 4 + 2 * i, voice as u16);
                }
                c.g[A0][0] = index as u64;
                c.pc = 0x003c_2440;
            },
        );
    }

    use crate::difftest::events::driven;

    /// The sound library's state block for SND_ParseStreamHeader: an empty
    /// SND heap (`STATE+0x234`) for the data tags, the output layout byte and
    /// the per-layout channel offset table.
    pub(crate) fn psh_sound_state(c: &mut Cpu, r: &mut Rng, seed: u32) {
        const HEAP: u32 = DATA + 0x8000;
        put_words(c, PSH_SND_STATE + 0x234, &[HEAP]);
        // base, block table (empty, grows down), limit, low water, lowest.
        put_words(
            c,
            HEAP,
            &[DATA + 0x20_0000, DATA + 0x9000, 0, 0x4000, 0x40, 0],
        );
        let layout = if seed % 3 == 0 { r.next_u32() % 4 } else { 2 };
        c.bus
            .write8(PSH_SND_STATE + PSH_STATE_OUT_CHANNELS, layout as u8);
        for i in 0..36 {
            c.bus.write16(
                PSH_SND_STATE + PSH_STATE_CHAN_TABLE + 2 * i,
                r.next_u32() as u16,
            );
        }
    }

    /// A stream header at `src`: the magic, then random tags of the
    /// parser's (id, length, big-endian value / data) form, padding (0xfc)
    /// and length-less ids (0xfd/0xfe), ended by 0xff.
    ///
    /// `events`: a plain "SCHl" stream of at least 8 tags without the data
    /// tags, so no callee that can unwind on its own (memset/
    /// `SND_ParsePtHeader`/`SND_MemCopy` dispatchers) runs once the event is
    /// due; the test harness re-enters only the function under test.
    pub(crate) fn psh_stream(c: &mut Cpu, r: &mut Rng, seed: u32, src: u32, events: bool) {
        let mut b: Vec<u8> = Vec::new();
        match if events { 7 } else { seed % 8 } {
            0 => b.extend_from_slice(b"GSTR\0\0\0\0"),
            1 => b.extend_from_slice(b"PTxx"),
            2 => b.extend_from_slice(b"GSTx"),
            3 => b.extend_from_slice(b"GSxR"),
            4 => b.extend_from_slice(b"PxTR"),
            _ => b.extend_from_slice(b"SCHl"),
        }
        let ids: Vec<u8> = PSH_TAGS.iter().map(|t| t.0 as u8).collect();
        let n = if events {
            8 + r.next_u32() % 8
        } else {
            r.next_u32() % 14
        };
        let mut markers = 0;
        for _ in 0..n {
            match r.next_u32() % 12 {
                0 => b.push(0xfc),
                1 => b.push(0xfd + (r.next_u32() % 2) as u8),
                _ => {
                    let mut id = if r.next_u32() % 6 == 0 {
                        r.next_u32() as u8 % 0xf0
                    } else {
                        ids[(r.next_u32() as usize) % ids.len()]
                    };
                    if events && matches!(id, 0x98..=0x9b | 0xa4 | 0xa5) {
                        id = 0x88;
                    }
                    if id == 0x14 {
                        markers += 1;
                        if markers > 4 {
                            id = 0x13;
                        }
                    }
                    let len = 1 + r.next_u32() % 4;
                    let mut data: Vec<u8> = (0..len).map(|_| r.next_u32() as u8).collect();
                    match id {
                        // Channels: 1..6, sometimes 0 (no channel loop).
                        0x82 => {
                            data = vec![(r.next_u32() % 7) as u8];
                        }
                        // Output flags: the bits the channel loop tests.
                        0x8c => {
                            let f = [0u16, 0x14, 0x10, 0x4, 0x308, 0x100, 0x8, 0x1c];
                            let v = f[(r.next_u32() as usize) % f.len()];
                            data = vec![(v >> 8) as u8, v as u8];
                        }
                        _ => {}
                    }
                    b.push(id);
                    b.push(data.len() as u8);
                    b.extend_from_slice(&data);
                }
            }
        }
        b.push(0xff);
        for (i, &x) in b.iter().enumerate() {
            c.bus.write8(src + i as u32, x);
        }
    }

    /// Arguments of SND_ParseStreamHeader(base, src, hdr, out, extra, isGstr).
    pub(crate) fn psh_setup(c: &mut Cpu, r: &mut Rng, seed: u32, events: bool) {
        psh_sound_state(c, r, seed);
        let src = DATA + 0x1000;
        psh_stream(c, r, seed, src, events);
        c.g[A0][0] = sx(r.next_u32() % 0x10_0000);
        c.g[A1][0] = sx(src);
        c.g[A2][0] = sx(DATA + 0x200);
        c.g[A3][0] = sx(DATA + 0x300);
        c.g[T0][0] = sx(DATA + 0x400);
        c.g[T1][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x500) };
        c.pc = 0x003b_b820;
    }

    /// SND_ParseStreamHeader: random tag streams (every tag of the chain,
    /// unknown ids, padding), the PT / GSTR / near-miss magics, channel
    /// counts 0..6, stereo and other layouts, and the absolute / relative
    /// offset flags.
    #[test]
    fn snd_parse_stream_header_matches() {
        check(
            ee::s13::f_003bb820__generated,
            snd_parse_stream_header,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3bb820, 0);
                psh_setup(c, &mut r, seed, false);
            },
        );
    }

    driven!(psh_gen_driven, ee::s13::f_003bb820__generated);

    driven!(psh_hand_driven, snd_parse_stream_header);

    /// SND_ParseStreamHeader with events at the tag loop's and the channel
    /// loop's back edges (delivered, or unwinding and re-entered there).
    #[test]
    fn snd_parse_stream_header_events_match() {
        check_events(psh_gen_driven, psh_hand_driven, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x3bb821, 0);
            psh_setup(c, &mut r, seed, true);
            let mode = schedule(c, &mut r, seed, 0x003b_b820);
            // The first due event lands after the prologue's memsets (~250
            // cycles), in the tag loop (its own resume head 0x3bb958) or the
            // non-unwinding InitPatchParams / ParseNextTag loops; a memset
            // that unwinds would resume at a return address, which neither
            // the original nor the port has as an entry.
            if mode != Events::Off {
                c.next_event += 400;
            }
            mode
        });
    }

    /// SND_FadeVolume setup: SND_SetVolume's channel table and handle
    /// (bad handles, one or more channels), then `frames` (positive, zero
    /// or negative: one frame) and the target volume.
    pub(crate) fn fade_setup(c: &mut Cpu, r: &mut Rng) {
        crate::game::lib::snd::tests::set_volume_setup(c, r);
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = match r.next_u32() % 4 {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 50) as i64)) as u64,
            _ => (1 + r.next_u32() % 300) as u64,
        };
    }

    driven!(fade_gen_driven, ee::s13::f_003b8530__generated);

    driven!(fade_hand_driven, snd_fade_volume);

    /// SND_FadeVolume: plain runs, and events at the channel loop's back
    /// edge (delivered, or unwinding and re-entered at the loop head).
    #[test]
    fn snd_fade_volume_matches() {
        check(
            ee::s13::f_003b8530__generated,
            snd_fade_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b8530, 0);
                fade_setup(c, &mut r);
                c.pc = 0x003b_8530;
            },
        );
        check_events(fade_gen_driven, fade_hand_driven, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b8531, 0);
            fade_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_8530)
        });
    }

    /// SND_ChannelCalcSend: a random channel table (flags selecting the
    /// different effect parameter blocks, signed send bytes), send slots
    /// 0..3 and channels 0..7.
    #[test]
    fn snd_channel_calc_send_matches() {
        check(
            ee::s13::f_003ba740__generated,
            snd_channel_calc_send,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ba740, 0);
                // Effect parameter blocks in the state area, then the table.
                for i in 0..0x100u32 {
                    c.bus.write32(0x0050_aa00 + 4 * i, r.next_u32());
                }
                let table = DATA + 0x2_0000;
                put_words(c, CALC_SEND_CHANNELS, &[table]);
                for i in 0..8 * 0x8c / 4 {
                    c.bus.write32(table + 4 * i, r.next_u32());
                }
                for ch in 0..8u32 {
                    let flags = [0x8u16, 0x100, 0x4, 0x10, 0x200, r.next_u32() as u16];
                    c.bus.write16(
                        table + ch * 0x8c + CALC_SEND_FLAGS,
                        flags[(r.next_u32() % 6) as usize],
                    );
                }
                c.g[A0][0] = (r.next_u32() % 4) as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.pc = 0x003b_a740;
            },
        );
    }

    /// `SND_FireEvents`: a source with 0..4 queued events (or a negative
    /// count: nothing to do) and random queue words; the event table has
    /// -1..3 handlers, all fakes that log the message pointer and sometimes
    /// unwind.
    #[test]
    fn snd_fire_events_matches() {
        const SOURCE: u32 = DATA + 0x100;
        covered(
            ee::s13::f_003b5f98__generated,
            snd_fire_events,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_5f98, 8);
                c.pc = 0x003b_5f98;
                fill(c, &mut r, SOURCE, 0x20);
                let pending = [0u8, 1, 2, 3, 4, 0x80, 0xff][seed as usize % 7];
                c.bus.write8(SOURCE + SND_SOURCE_PENDING, pending);
                let handlers = [1u8, 2, 3, 0, 0xff][(seed / 7) as usize % 5];
                c.bus
                    .write8(SND_EVENT_TABLE + SND_EVENT_HANDLER_COUNT, handlers);
                for i in 0..4 {
                    c.bus.write32(
                        SND_EVENT_TABLE + SND_EVENT_HANDLERS + 4 * i,
                        FAKE + 0x10 * i,
                    );
                }
                c.g[A0][0] = sx(SOURCE);
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }

    /// `SND_GetFxParams`: each effect kind (8, 4, 0x100), other kinds below
    /// and above 9, random high flag bits, any voice.
    #[test]
    fn snd_get_fx_params_matches() {
        use crate::difftest::check_lohi;
        check_lohi(
            ee::s13::f_003b8218__generated,
            snd_get_fx_params,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_8218, 0);
                c.pc = 0x003b_8218;
                let kind = [0x8u32, 0x4, 0x100, 0x0, 0x10, 0x18, 0x200, 0x71c][(seed % 8) as usize];
                let noise = if seed % 3 == 0 {
                    r.next_u32() & !0x71c
                } else {
                    0
                };
                c.g[A1][0] = sx(kind | noise);
                c.g[A0][0] = if seed % 5 == 0 {
                    r.next_u32() as u64
                } else {
                    (r.next_u32() % 64) as u64
                };
            },
        );
    }

    /// `SPCH_IsSampleAllowed`: choices in and out of range (negative too),
    /// no sample selected (-1), samples in and out of range, flags with and
    /// without slot masks, extra bytes or none, random bit arrays.
    #[test]
    fn spch_is_sample_allowed_matches() {
        use crate::difftest::check_lohi;
        const EV: u32 = DATA + 0x100;
        const BANK: u32 = DATA + 0x1000;
        check_lohi(
            ee::s14::f_003d8780__generated,
            spch_is_sample_allowed,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_8780, 0);
                c.pc = 0x003d_8780;
                fill(c, &mut r, BANK, 0x100);
                let bits = r.next_u32() % 24;
                c.bus.write8(BANK + 5, bits as u8);
                let flags = (r.next_u32() % 16) | if seed % 2 == 0 { 0x80 } else { 0 };
                c.bus.write8(BANK + 4, flags as u8);
                c.bus.write8(
                    BANK + 6,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (r.next_u32() % 5) as u8
                    },
                );
                let samples = r.next_u32() % 8;
                c.bus.write16(BANK + 0xa, samples as u16);
                let sample: u16 = match seed % 5 {
                    0 => 0xffff,
                    1 => (samples + r.next_u32() % 3) as u16,
                    _ => (r.next_u32() % samples.max(1)) as u16,
                };
                c.bus.write16(EV + 2, sample);
                let choice = match seed % 7 {
                    0 => bits + 1 + r.next_u32() % 4,
                    1 => 0u32.wrapping_sub(1 + r.next_u32() % 8), // small negative
                    _ => r.next_u32() % (bits + 1),
                };
                c.g[A0][0] = sx(EV);
                c.g[A1][0] = sx(BANK);
                c.g[A2][0] = sx(choice);
            },
        );
    }

    /// The speech history ring at 0x450b88 filled with keys matching the
    /// request (or not) and small values, so draws hit, miss and wrap the
    /// ring; RNG state at 0x450b70; some seeds pass key -1. The real
    /// SPCH_RandNext runs.
    #[test]
    fn spch_random_no_repeat_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, GAME_GP};
        check_lohi(
            ee::s14::f_003db5c0__generated,
            spch_random_no_repeat,
            300,
            &|c, seed| {
                let mut r = Rng(0x3db5c0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_b5c0;
                c.g[GP][0] = GAME_GP as u64;
                for i in 0..6 {
                    c.bus.write32(0x0045_0b70 + 4 * i, r.next_u32());
                }
                let range = match seed % 6 {
                    0 => r.next_u32() % 4 + 1,
                    1 => r.next_u32() % 12,
                    2 => r.next_u32() % 100,
                    3 => (r.next_u32() % 50) as i32 as u32 ^ 0x8000_0000,
                    4 => 2,
                    _ => r.next_u32() % 70,
                };
                let key = r.next_u32() & 0xffff;
                c.g[A0][0] = sx(range);
                c.g[A1][0] = match seed % 7 {
                    0 => u64::MAX,
                    1 => (r.next_u32() as u64) << 32 | key as u64,
                    _ => key as u64,
                };
                for i in 0..32 {
                    let k = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xffff
                    } else {
                        key
                    };
                    let v = r.next_u32() % (range.min(64) + 1);
                    c.bus.write16(SPCH_HISTORY + 4 * i, v as u16);
                    c.bus.write16(SPCH_HISTORY + 4 * i + 2, k as u16);
                }
                c.bus.write32(GAME_GP + SPCH_HISTORY_POS, r.next_u32() % 32);
            },
        );
    }

    /// SPCH_RandNext on random RNG states, with words near the top so the
    /// carries happen, and the counter at -1 (wraps) with some of words
    /// 4..1 at 0 (they become -1 after the sum) so each step of the
    /// increment chain runs.
    #[test]
    fn spch_rand_next_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(
            ee::s14::f_003db4d0__generated,
            spch_rand_next,
            400,
            &|c, seed| {
                let mut r = Rng(0x3db4d0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_b4d0;
                let wrap = seed % 3 == 0;
                let chain = seed / 3 % 6; // how many of words 4..1 are -1
                for i in 0..6u32 {
                    let v = match r.next_u32() % 3 {
                        0 => r.next_u32(),
                        1 => 0xffff_ffff - (r.next_u32() % 4),
                        _ => 0x8000_0000 ^ r.next_u32() % 16,
                    };
                    c.bus.write32(SPCH_RNG + 4 * i, v);
                }
                if wrap {
                    c.bus.write32(SPCH_RNG + 0x14, u32::MAX);
                    for k in 0..chain.min(4) {
                        c.bus.write32(SPCH_RNG + 4 * (4 - k), 0);
                    }
                }
            },
        );
    }

    /// SND_UtkParseHeader: without and with the header read (the real
    /// SND_UtkReadBits on random stream bytes; its mask table comes from the
    /// ELF image), random state contents to be overwritten.
    #[test]
    fn snd_utk_parse_header_matches() {
        check(
            ee::s14::f_003cd6f0__generated,
            snd_utk_parse_header,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x3c_d6f0, 0);
                c.pc = 0x003c_d6f0;
                let data = DATA + 0x100 + seed % 4;
                let st = DATA + 0x1000;
                crate::difftest::fill(c, &mut r, DATA + 0x100, 8);
                crate::difftest::fill(c, &mut r, st, 0x6a0 / 4);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = sx(st);
                c.g[A3][0] = if seed % 4 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) << 32 | 1
                };
            },
        );
    }

    /// SND_NextVoiceGroup: random group masks and request flags (both stage
    /// bits zero or set, b3 = 1 or not), start index before, inside or past
    /// the table; a third of the seeds deliver events at the loop head.
    #[test]
    fn snd_next_voice_group_matches() {
        use crate::difftest::events::schedule;
        use crate::difftest::{check, scramble, Rng, DATA, RA as SENTINEL};
        check(
            ssx3_ee::s13::f_003c6158__generated,
            snd_next_voice_group,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c61_5800 ^ seed as u64);
                scramble(c, &mut r);
                let idx_ptr = DATA;
                let req = DATA + 0x200;
                let count = 1 + r.next_u32() % 12;
                c.bus
                    .write8(SND_VOICE_GROUPS + SND_VOICE_GROUP_COUNT, count as u8);
                for i in 0..16 {
                    let m = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.next_u32() & 0x71c,
                        2 => r.next_u32() & 0xe0,
                        _ => r.next_u32(),
                    };
                    c.bus
                        .write16(SND_VOICE_GROUPS + SND_VOICE_GROUP_MASKS + 2 * i, m as u16);
                }
                let start = match seed % 5 {
                    0 => count + r.next_u32() % 3,
                    1 => (r.next_u32() % 3).wrapping_neg(),
                    _ => r.next_u32() % count,
                };
                c.bus.write32(idx_ptr, start);
                let flags = match (seed / 5) % 4 {
                    0 => 0,
                    1 => r.next_u32() & 0x71c,
                    2 => r.next_u32() & 0xe0,
                    _ => r.next_u32(),
                };
                c.bus.write16(req + SND_REQ_FLAGS, flags as u16);
                c.bus.write8(
                    req + SND_REQ_B3,
                    if seed % 2 == 0 { 1 } else { r.next_u32() as u8 },
                );
                c.g[A0][0] = sx(idx_ptr);
                c.g[A1][0] = sx(req);
                c.pc = 0x003c_6158;
                if seed % 7 < 3 {
                    c.g[RA][0] = SENTINEL as u64;
                    schedule(c, &mut r, seed, 0x003c_6158);
                }
            },
        );
    }

    /// SNDSTRM_ReleaseChunkCallback: chunk header before the data, stream
    /// number byte, handle table; the real StreamFile_ReleaseChunk takes its
    /// short paths (null file, bad magic, chunk outside the buffer, chunk
    /// already released).
    #[test]
    fn sndstrm_release_chunk_callback_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        check(
            ssx3_ee::s13::f_003b6ad8__generated,
            sndstrm_release_chunk_callback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b6ad8, 0);
                c.pc = 0x003b_6ad8;
                let data = DATA + 0x10;
                let chunk = DATA + 0x100;
                let case = seed % 4;
                // The stream byte is also the low byte of the chunk's status
                // word: 0xfe with case 3 makes the status "released" (-2).
                let stream = if case == 3 { 0xfe } else { r.next_u32() % 8 };
                let handle_ptr = DATA + 0x200 + 0x10 * (stream & 7);
                let handle = DATA + 0x400;
                let header = DATA + 0x1000;
                c.bus.write32(data - 4, chunk);
                c.bus.write32(SNDSTRM_FILES + 4 * stream, handle_ptr);
                c.bus
                    .write32(handle_ptr, if case == 0 { 0 } else { handle });
                c.bus.write32(handle, header);
                let magic = 0x4d52_5453;
                c.bus.write32(
                    header,
                    if case == 1 {
                        magic ^ (1 + r.next_u32() % 0xff)
                    } else {
                        magic
                    },
                );
                // Buffer: past the chunk (case 2) or around it with the chunk
                // already released (case 3).
                let (start, end) = if case == 2 {
                    (DATA + 0x2000, DATA + 0x3000)
                } else {
                    (DATA, DATA + 0x3000)
                };
                c.bus.write32(header + 0x30, start);
                c.bus.write32(header + 0x34, end);
                c.bus.write32(chunk, 0xffff_ff00 | stream);
                c.g[A0][0] = sx(data);
            },
        );
    }

    /// SND_Play: bank index negative / past the count / unloaded / loading
    /// (the real SND_BankValidate), patch index negative, past the bank's
    /// count or inside it; inside, the real SND_BankGetPatch finds an empty
    /// patch slot and the real SND_PlayPatch returns -8 at once.
    #[test]
    fn snd_play_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const STATE: u32 = 0x0050_a8e8;
        const VALIDATE_TABLE: u32 = DATA + 0x1000;
        const BANKS: u32 = DATA + 0x2000;
        const BANK: u32 = DATA + 0x3000;
        check(
            ssx3_ee::s13::f_003b6820__generated,
            snd_play,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_6820, 0);
                c.pc = 0x003b_6820;
                let nbanks = 1 + r.next_u32() % 6;
                c.bus.write16(STATE + 0x28, nbanks as u16);
                c.bus.write32(STATE + 0x1f4, VALIDATE_TABLE);
                c.bus.write32(SND_PLAY_BANKS, BANKS);
                let bank = match seed % 6 {
                    0 => (r.next_u32() % 3).wrapping_neg(),
                    1 => nbanks + r.next_u32() % 3,
                    _ => r.next_u32() % nbanks,
                };
                for i in 0..8u32 {
                    let e = VALIDATE_TABLE + 16 * i;
                    c.bus
                        .write32(e, if seed % 6 == 2 { 0 } else { 1 + r.next_u32() });
                    c.bus.write8(
                        e + 0xc,
                        if seed % 6 == 3 {
                            1 + r.next_u32() as u8 % 4
                        } else {
                            0
                        },
                    );
                    c.bus.write32(BANKS + 16 * i, BANK);
                }
                let npatches = r.next_u32() % 6;
                c.bus.write16(BANK + SND_PLAY_BANK_PATCHES, npatches as u16);
                let patch = match (seed / 6) % 4 {
                    0 => (1 + r.next_u32() % 3).wrapping_neg(),
                    1 => npatches + r.next_u32() % 3,
                    _ => r.next_u32() % npatches.max(1),
                };
                c.g[A0][0] = sx(bank);
                c.g[A1][0] = sx(patch);
                c.g[A2][0] = sx(DATA + 0x4000);
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    /// `SPCH_HistoryFind`: empty or 1..12-entry histories with random
    /// flags/width (bitmap or not), write positions inside the ring (and 0:
    /// wrap at once), keys present (some several times) or absent.
    #[test]
    fn spch_history_find_matches() {
        use crate::difftest::check_lohi;
        use spch_hist_f::*;
        check_lohi(
            ee::s14::f_003d6e30__generated,
            spch_history_find,
            300,
            &|c, seed| {
                let mut r = Rng(0x3d6e_30c7 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003d_6e30;
                let hist = DATA;
                fill(c, &mut r, hist, 0x40);
                let count = if seed % 9 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 12
                };
                c.bus.write8(hist + COUNT, count as u8);
                c.bus.write8(hist + FLAGS, r.next_u32() as u8);
                c.bus.write8(hist + WIDTH, r.next_u32() as u8);
                // The ring and everything after the header: small values so keys hit.
                for i in 8..0x400 {
                    c.bus.write8(hist + i, (r.next_u32() % 8) as u8);
                }
                // Write positions: mostly within 0..=count (some hit the wrap).
                let flags = c.bus.read8(hist + FLAGS) as u32;
                let width = c.bus.read8(hist + WIDTH) as u32;
                let t = flags & 0xf;
                let mut ring = hist + ((width * (t + 2) + 0xf) & 0x3ffc) + t * 4;
                if flags & 0x80 != 0 {
                    ring += (width + 7) / 8 + 1;
                }
                c.bus.write8(ring, (r.next_u32() % (count + 1)) as u8);
                c.g[A0][0] = sx(hist);
                c.g[A1][0] = if seed % 11 == 0 {
                    sx(r.next_u32())
                } else {
                    (r.next_u32() % 9) as u64
                };
            },
        );
    }

    /// SND_RandomRange: ranges around the clamps (negative, 0, inside,
    /// 0x10000, above, 64-bit values); the real SND_Random over a random
    /// generator state.
    #[test]
    fn snd_random_range_matches() {
        use crate::difftest::{fill, scramble, Rng};
        check(
            ee::s13::f_003c72a0__generated,
            snd_random_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_72a0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_72a0;
                // The generator state (0x50ac50).
                fill(c, &mut r, 0x0050_ac50, 0x10);
                let range: u64 = match seed % 8 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 0x10000,
                    3 => 0x10001,
                    4 => sx(r.next_u32()),
                    5 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => (r.next_u32() % 0x10000) as u64,
                };
                c.g[A0][0] = range;
            },
        );
    }

    /// The voice table with `count` voices (some negative/zero counts), each
    /// in use or not, with serials just above the current one so that the
    /// search retries (including the wrap through 0).
    pub(crate) fn oc286_snd_voices(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let serial = if seed % 5 == 0 {
            0xff
        } else {
            r.next_u32() as u8
        };
        c.bus.write8(SND_SERIAL_HI - SND_SERIAL_LO, serial);
        let count: i16 = match seed % 7 {
            0 => -((r.next_u32() % 3) as i16),
            _ => (r.next_u32() % 10) as i16,
        };
        c.bus.write16(SND_VOICES + SND_VOICE_COUNT, count as u16);
        let voices = DATA + 0x4000;
        c.bus.write32(SND_VOICES + SND_VOICE_ARRAY, voices);
        for i in 0..10u32 {
            let v = voices + i * SND_VOICE_SIZE;
            let used = if r.next_u32() % 3 == 0 {
                0
            } else {
                r.next_u32() as u8 | 1
            };
            c.bus.write8(v + SND_VOICE_IN_USE, used);
            let ser = serial.wrapping_add(1 + (r.next_u32() % 4) as u8);
            c.bus.write8(v + SND_VOICE_SERIAL, ser);
        }
    }

    /// Serial bump (with the wrap that skips 0) and the retry search over the
    /// voices in use.
    #[test]
    fn snd_next_handle_serial_matches() {
        check(
            ee::s13::f_003bb0c8__generated,
            snd_next_handle_serial,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_b0c8, 0);
                oc286_snd_voices(c, &mut r, seed);
                c.pc = 0x003b_b0c8;
            },
        );
    }

    /// The same with events due at the back edges (delivered or unwinding).
    #[test]
    fn snd_next_handle_serial_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s13::f_003bb0c8__generated,
            snd_next_handle_serial,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_b0c9, 0);
                oc286_snd_voices(c, &mut r, seed | 1);
                // Enough voices for several back edges.
                let count = 3 + r.next_u32() % 7;
                c.bus.write16(SND_VOICES + SND_VOICE_COUNT, count as u16);
                let mode = schedule(c, &mut r, seed, 0x003b_b0c8);
                c.next_event = c.cycles + 10;
                mode
            },
        );
    }

    /// Stream table at 0x50a0a0 (count byte at 0x50a92e), one stream with
    /// a free list of 0..2 nodes; the SND lock/unlock hooks (0x44c46c,
    /// 0x44c470) are fakes; the stream file handle is null or has a bad
    /// magic, so the real `StreamFile_Queue*` take their short path (0).
    pub(crate) fn sndstrm_queue_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        let mut r = machine(c, seed, 0x3b76_e800, 2);
        let streams = 0x0050_a0a0;
        let stream = DATA;
        c.bus.write8(0x0050_a92e, 4);
        let index = match seed % 9 {
            0 => 5,
            1 => 2,
            _ => 1,
        };
        for k in 0..4 {
            c.bus
                .write32(streams + 4 * k, if k == 1 { stream } else { 0 });
        }
        c.bus.write32(0x0044_c46c, FAKE);
        c.bus.write32(0x0044_c470, FAKE + 0x10);
        // File handle: null, or an object whose header lacks 'MRTS'.
        let handle = if seed % 2 == 0 { 0 } else { DATA + 0x800 };
        c.bus.write32(DATA + 0x800, DATA + 0x900);
        c.bus.write32(DATA + 0x900, r.next_u32());
        c.bus.write32(stream + SNDSTRM_QUEUE_HANDLE, handle);
        let id = match seed % 4 {
            0 => 0x7fff_ff80u32,
            _ => r.next_u32() & 0x7fff_ff00,
        };
        c.bus.write32(stream + SNDSTRM_QUEUE_NEXT_ID, id);
        // Free list: 0, 1 or 2 nodes.
        let nodes = if seed % 7 == 3 { 0 } else { 1 + seed % 2 };
        let n0 = DATA + 0x1000;
        let n1 = DATA + 0x1100;
        crate::difftest::fill(c, &mut r, n0, 0x80);
        c.bus
            .write32(stream + SNDSTRM_QUEUE_FREE, if nodes > 0 { n0 } else { 0 });
        c.bus.write32(
            stream + SNDSTRM_QUEUE_FREE + 4,
            if nodes == 2 {
                n1
            } else if nodes == 1 {
                n0
            } else {
                0
            },
        );
        c.bus.write32(stream + SNDSTRM_QUEUE_FREE_COUNT, nodes);
        c.bus.write32(n0, if nodes == 2 { n1 } else { 0 });
        c.bus.write32(n1, 0);
        // Queued list: empty or one node.
        let q = DATA + 0x1200;
        let queued = seed % 3 == 0;
        c.bus
            .write32(stream + SNDSTRM_QUEUE_QUEUED, if queued { q } else { 0 });
        c.bus.write32(
            stream + SNDSTRM_QUEUE_QUEUED + 4,
            if queued { q } else { 0 },
        );
        c.bus
            .write32(stream + SNDSTRM_QUEUE_QUEUED + 8, queued as u32);
        c.bus.write32(q, 0);
        c.g[A0][0] = index;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = match seed % 5 {
            0 => 0,
            _ => sx(r.next_u32() | 1),
        };
        c.g[T0][0] = match seed % 6 {
            0 => 0,
            1 => 1,
            _ => (2 + r.next_u32() % 4) as u64,
        };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003b_76e8;
    }

    /// `SNDSTRM_QueueRequest`: no stream, no free node, file queue
    /// failure (modes 0 and 1), a queued request (id wrap), fakes
    /// unwinding at the lock or unlock.
    #[test]
    fn sndstrm_queue_request_matches() {
        check(
            ee::s13::f_003b76e8__generated,
            sndstrm_queue_request,
            300,
            &sndstrm_queue_fixture,
        );
    }

    /// `SND_EeVoiceStop`: voices 0..7 with random levels and scale, 0..5
    /// mixer channels, 1..4 nodes whose stop callback is a fake (some
    /// unwind) or none; `SND_MemFree` runs for real on an empty SND heap
    /// (unknown pointers are ignored).
    #[test]
    fn snd_ee_voice_stop_matches() {
        const VOICES: u32 = DATA + 0x20_0000;
        const NODES: u32 = DATA + 0x20_1000;
        const HEAP: u32 = DATA + 0x20_2000;
        check(
            ee::s14::f_003c8468__generated,
            snd_ee_voice_stop,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8468, 4);
                c.pc = 0x003c_8468;
                let voice = seed % 8;
                c.g[A0][0] = voice as u64 | (r.next_u32() as u64) << 32;
                c.bus.write32(SEVS_MIXER + SEVS_VOICES, VOICES);
                c.bus
                    .write8(SEVS_MIXER + SEVS_CHANNELS, (seed / 8 % 6) as u8);
                for i in 0..8 {
                    c.bus
                        .write32(SEVS_MIXER + SEVS_CHANNEL_SUMS + 4 * i, r.f32(4.0));
                }
                let v = VOICES + SEVS_VOICE_SIZE * voice;
                for i in 0..0x18 {
                    c.bus.write32(v + 4 * i, r.f32(2.0));
                }
                c.bus.write8(v, 1);
                // The node list.
                let n = 1 + r.next_u32() % 4;
                for k in 0..n {
                    let node = NODES + 0x20 * k;
                    let stop = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * k
                    };
                    c.bus.write32(node + 4, stop);
                    c.bus
                        .write32(node + 8, if k + 1 == n { 0 } else { node + 0x20 });
                }
                c.bus.write32(v + SEVS_NODES, NODES);
                // SND heap: no blocks (SND_MemFree ignores the pointer).
                c.bus.write32(0x0050_a8e8 + 0x234, HEAP);
                c.bus.write32(HEAP, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0x14, 0);
            },
        );
    }

    /// `SND_MidiToVolume`: levels 0..127 and a few out of range (the
    /// table read is unchecked), with junk in the upper bits.
    #[test]
    fn snd_midi_to_volume_matches() {
        check(
            ee::s13::f_003bc570__generated,
            snd_midi_to_volume,
            300,
            &|c, seed| {
                let mut r = Rng(0x3b_c570 ^ (seed as u64) << 16);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x003b_c570;
                for i in 0..0x200u32 {
                    c.bus.write16(SMTV_TABLE + 2 * i, r.next_u32() as u16);
                }
                let level = if seed % 10 == 0 {
                    128 + r.next_u32() % 128
                } else {
                    seed % 128
                };
                c.g[A0][0] = (r.next_u32() as u64) << 32 | level as u64;
            },
        );
    }

    /// `SND_EeVoiceStart`: every sample format (and an unknown one), every
    /// channel mode (mono with `f` below or above 1, stereo, other), start
    /// functions present (fakes, unwinding on some seeds) or not, with and
    /// without a stream. The real `SND_MemAlloc` allocates from an empty
    /// heap at `DATA + 0x20_0000`; the real time-stretch init reads a random
    /// stream header.
    #[test]
    fn snd_ee_voice_start_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        use crate::difftest::SP as STACK;
        covered(
            ee::s14::f_003c8158__generated,
            snd_ee_voice_start,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8158, 1);
                // The voice entries.
                let entries = DATA + 0x3000;
                for i in 0..0x60 * 8 / 4 {
                    c.bus.write32(entries + 4 * i, r.next_u32());
                }
                c.bus.write32(VOICE_STATE + VOICE_STATE_ENTRIES, entries);
                // The driver heap (`STATE + 0x234`): empty, room for both blocks.
                let heap = DATA + 0x100;
                c.bus.write32(VOICE_STATE + 0x234, heap);
                c.bus.write32(heap, DATA + 0x20_0000); // base
                c.bus.write32(heap + 0x4, DATA + 0x2000); // blocks
                c.bus.write32(heap + 0xc, 0x8_0000); // limit
                c.bus.write32(heap + 0x10, 0x100); // low water
                c.bus.write32(heap + 0x14, 0); // lowest: empty
                                               // Object sizes, then start functions (row -1 reads below them).
                for i in 0..0x48 / 4 + 3 {
                    c.bus
                        .write32(VOICE_STATE + 0x54 + 4 * i, 0x20 + r.next_u32() % 0x200);
                }
                for i in 0..0x48 / 4 + 3 {
                    let fun = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * (i % 4)
                    };
                    c.bus.write32(VOICE_STATE + 0xc + 4 * i, fun);
                }
                c.bus.write8(VOICE_START_FLAG, r.next_u32() as u8);
                // The stream header.
                let strm = DATA + 0x5000;
                for i in 0..0x10 {
                    c.bus.write32(strm + 4 * i, r.next_u32());
                }
                // Stack arguments: d, f, e, prio.
                let f_arg: i32 = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => -1,
                    _ => (r.next_u32() % 10) as i32,
                };
                c.bus.write32(STACK, r.next_u32());
                c.bus.write32(STACK + 0x8, f_arg as u32);
                c.bus.write32(STACK + 0x20, r.next_u32());
                c.bus.write32(STACK + 0x28, r.next_u32());

                let formats = [8u64, 10, 4, 22, 16, 23, 5];
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                c.g[A1][0] = formats[(seed / 2 % 7) as usize];
                c.g[A2][0] = match seed / 3 % 4 {
                    0 => 0,
                    1 | 2 => 1,
                    _ => 2,
                };
                c.g[T1][0] = if seed % 5 < 2 { 0 } else { sx(strm) };
                c.pc = 0x003c_8158;
            },
        );
    }

    /// SND_UtkSetInput: null data, a busy decoder, format 1 with and
    /// without the 0xee marker, other formats; the real SND_UtkParseHeader
    /// (with or without its header read; its mask table from the ELF image)
    /// on random stream bytes.
    #[test]
    fn snd_utk_set_input_matches() {
        use crate::difftest::elf_image;
        check(
            ee::s14::f_003cdde8__generated,
            snd_utk_set_input,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = fakes::machine(c, seed, 0x3cdde8, 0);
                let st = DATA + 0x1000;
                let data = DATA + 0x100 + seed % 4;
                fill(c, &mut r, DATA + 0x100, 8);
                fill(c, &mut r, st, 0xd70 / 4);
                c.bus.write32(
                    st + UTK_INPUT_SIZE,
                    if seed % 5 == 1 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(
                    st + UTK_FORMAT,
                    if seed % 3 == 0 { r.next_u32() % 3 } else { 1 },
                );
                if seed % 2 == 0 {
                    c.bus.write8(data, 0xee);
                }
                let reset = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() | 1,
                };
                c.bus.write32(st + UTK_RESET, reset);
                c.g[A0][0] = sx(st);
                c.g[A1][0] = if seed % 7 == 3 { 0 } else { sx(data) };
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.pc = 0x003c_dde8;
            },
        );
    }

    /// SND_EeVoiceSetPitch: pitches below, at and above 0x20000 (and
    /// negative), a voice that already has a resampler, or none (the real
    /// SND_MemAlloc allocates from an empty driver heap at DATA+0x20_0000),
    /// inserted into an empty DSP chain or one of 1-3 nodes with random
    /// priorities; both rate-code sides and several resampler modes.
    #[test]
    fn snd_ee_voice_set_pitch_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::machine;
        check_lohi(
            ee::s14::f_003c8a88__generated,
            snd_ee_voice_set_pitch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8a88, 0);
                // The voice entries.
                let entries = DATA + 0x3000;
                fill(c, &mut r, entries, 0x60 * 8 / 4);
                c.bus.write32(PITCH_SND_STATE + PITCH_STATE_VOICES, entries);
                let voice_index = r.next_u32() % 8;
                let voice = entries + 0x60 * voice_index;
                let existing = DATA + 0x6000;
                c.bus.write32(
                    voice + PITCH_VOICE_RESAMPLER,
                    if seed % 3 == 0 { existing } else { 0 },
                );
                // The DSP chain: empty or 1-3 nodes {next +8, priority +0x18}.
                let nodes = DATA + 0x7000;
                let n = seed / 3 % 4;
                c.bus.write32(
                    voice + PITCH_VOICE_DSP_CHAIN,
                    if n == 0 { 0 } else { nodes },
                );
                for i in 0..n {
                    let node = nodes + 0x40 * i;
                    c.bus
                        .write32(node + 8, if i + 1 == n { 0 } else { node + 0x40 });
                    c.bus
                        .write16(node + 0x18, (0x40 + 0x30 * i + r.next_u32() % 0x40) as u16);
                }
                // The driver heap (`STATE + 0x234`): empty, room for the block.
                let heap = DATA + 0x100;
                c.bus.write32(PITCH_SND_STATE + 0x234, heap);
                c.bus.write32(heap, DATA + 0x20_0000); // base
                c.bus.write32(heap + 0x4, DATA + 0x2000); // blocks
                c.bus.write32(heap + 0xc, 0x8_0000); // limit
                c.bus.write32(heap + 0x10, 0x100); // low water
                c.bus.write32(heap + 0x14, 0); // lowest: empty
                c.bus.write8(
                    PITCH_SND_STATE + PITCH_STATE_RATE_CODE,
                    (r.next_u32() % 100) as u8,
                );
                c.bus.write8(PITCH_RESAMPLE_MODE, (r.next_u32() % 3) as u8);
                c.g[A0][0] = voice_index as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => PITCH_MAX,
                    1 => PITCH_MAX + 1 + (r.next_u32() % 0x1_0000) as u64,
                    2 => sx(0u32.wrapping_sub(r.next_u32() % 0x100)),
                    _ => (r.next_u32() % 0x2_0000) as u64,
                };
                c.pc = 0x003c_8a88;
            },
        );
    }

    use crate::difftest::check_paths_lohi;

    /// Every codec of the chain, plus unknown ones, with random rates and
    /// channel counts.
    #[test]
    fn sndstrm_calc_data_rate_matches() {
        const CODECS: [u8; 10] = [5, 10, 4, 22, 8, 16, 23, 0, 0xff, 9];
        check_paths_lohi(
            ee::s13::f_003b6948__generated,
            sndstrm_calc_data_rate,
            200,
            &[12, 15, 17, 19, 21, 20, 22, 23],
            &|c, seed| {
                let mut r = Rng(0x3b69_4800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003b_6948;
                c.g[A0][0] = sx(DATA);
                c.bus.write16(DATA, r.next_u32() as u16);
                c.bus.write8(DATA + 2, r.next_u32() as u8);
                c.bus.write8(DATA + 3, CODECS[(seed % 10) as usize]);
            },
        );
    }

    /// `SND_EeVoiceSetLowpass`: cutoffs below 1.0 (positive and negative,
    /// so both rounding signs) with the voice's lowpass DSP present or
    /// created by the real `SND_MemAlloc` from an empty heap and inserted
    /// into a 0-3 node chain; cutoffs of 1.0 or more with a DSP to remove
    /// (the real `SND_DspRemove`, `SND_MemFree` on an empty heap) or none.
    #[test]
    fn snd_ee_voice_set_lowpass_matches() {
        const VOICES: u32 = DATA + 0x20_0000;
        const NODES: u32 = DATA + 0x20_1000;
        const HEAP: u32 = DATA + 0x20_2000;
        crate::difftest::check_lohi(
            ee::s14::f_003c8d00__generated,
            snd_ee_voice_set_lowpass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8d00, 0);
                c.pc = 0x003c_8d00;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.bus.write32(LOWPASS_VOICES, VOICES);
                fill(c, &mut r, VOICES, 0x60 * 8 / 4);
                let voice = VOICES + 0x60 * index;
                // The DSP chain: 0-3 nodes {next +8, priority +0x18}; the
                // lowpass DSP (when present) is one of them or not.
                let n = seed / 8 % 4;
                for i in 0..n {
                    let node = NODES + 0x40 * i;
                    fill(c, &mut r, node, 0x10);
                    c.bus
                        .write32(node + 8, if i + 1 == n { 0 } else { node + 0x40 });
                    c.bus
                        .write16(node + 0x18, (0x10 + 0x20 * i + r.next_u32() % 0x20) as u16);
                }
                c.bus.write32(
                    voice + LOWPASS_VOICE_DSP_CHAIN,
                    if n == 0 { 0 } else { NODES },
                );
                let filter = match seed % 3 {
                    0 => 0,
                    1 if n > 0 => NODES + 0x40 * (r.next_u32() % n),
                    _ => NODES + 0x200,
                };
                c.bus.write32(voice + LOWPASS_VOICE_FILTER, filter);
                // The driver heap: empty, room for the block.
                for state in [0x0051_5b40u32, LOWPASS_SND_STATE] {
                    c.bus.write32(state + 0x234, HEAP);
                }
                c.bus.write32(HEAP, DATA + 0x20_4000);
                c.bus.write32(HEAP + 0x4, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0xc, 0x8_0000);
                c.bus.write32(HEAP + 0x10, 0x100);
                c.bus.write32(HEAP + 0x14, 0);
                c.bus.write16(
                    LOWPASS_SND_STATE + LOWPASS_STATE_RATE,
                    [48000u32, 44100, 24000, r.next_u32() % 0x1_0000][(seed / 3 % 4) as usize]
                        as u16,
                );
                c.f[12] = match seed / 5 % 6 {
                    0 => 0x3f80_0000,
                    1 => r.edge_f32(),
                    2 => r.f32(1.0) | 0x8000_0000,
                    3 => 0x4000_0000 + (r.next_u32() & 0xff_ffff),
                    _ => r.f32(1.0) & 0x7fff_ffff,
                };
            },
        );
    }

    /// `SND_ChannelApplyTimeStretch`: channels with and without the EE-voice
    /// flag, 0-6 voices whose numbers map (after the two bases) onto voices
    /// 0-7 of a zeroed voice table (no stretch DSP: the real
    /// `SND_EeVoiceSetTimeStretch` returns at once).
    #[test]
    fn snd_channel_apply_time_stretch_matches() {
        const CHANNELS: u32 = DATA + 0x1000;
        const VOICES: u32 = DATA + 0x20_0000;
        crate::difftest::check_lohi(
            ee::s13::f_003c3010__generated,
            snd_channel_apply_time_stretch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_3010, 0);
                c.pc = 0x003c_3010;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.bus
                    .write32(STRETCH_SND_STATE + STRETCH_STATE_CHANNELS, CHANNELS);
                fill(c, &mut r, CHANNELS, (0x8c * 8) / 4);
                let base_a = r.next_u32() % 0x40;
                let base_b = r.next_u32() % 0x40;
                c.bus
                    .write8(STRETCH_SND_STATE + STRETCH_STATE_BASE_A, base_a as u8);
                c.bus
                    .write8(STRETCH_SND_STATE + STRETCH_STATE_BASE_B, base_b as u8);
                let ch = CHANNELS + 0x8c * index;
                let flags = r.next_u32() as u16;
                c.bus.write16(
                    ch + STRETCH_CHANNEL_FLAGS,
                    if seed % 4 == 0 { flags & !4 } else { flags | 4 },
                );
                let count = seed / 4 % 7;
                c.bus.write8(ch + STRETCH_CHANNEL_COUNT, count as u8);
                for i in 0..count {
                    c.bus.write16(
                        ch + STRETCH_CHANNEL_VOICES + 2 * i,
                        (base_a + base_b + r.next_u32() % 8) as u16,
                    );
                }
                c.bus.write32(0x0051_5d1c, VOICES);
                for i in 0..0x60 * 8 / 4 {
                    c.bus.write32(VOICES + 4 * i, 0);
                }
            },
        );
    }

    /// SNDSTRM_RetireRequest: four streams with a three-request active list
    /// and a one-entry free list; the handle names the first, middle or
    /// last request, which is or is not the playing one (the real
    /// FindRequest / ListRemove / ListPushFront run).
    #[test]
    fn sndstrm_retire_request_matches() {
        use crate::difftest::{check, scramble, Rng};
        const B: u32 = 0x00b0_0000;
        const STREAM_COUNT: u32 = 0x0050_a92e;
        check(
            ee::s13::f_003b6a70__generated,
            sndstrm_retire_request,
            200,
            &|c, seed| {
                let mut r = Rng(0x003b_6a70 ^ (seed as u64) << 5);
                scramble(c, &mut r);
                c.pc = 0x003b_6a70;
                c.bus.write8(STREAM_COUNT, 4);
                for i in 0..4u32 {
                    let stream = B + 0x1000 * i;
                    c.bus
                        .write32(SNDSTRM_RETIRE_REQUEST_STREAMS + 4 * i, stream);
                    // Active list {head, tail, count}: three requests
                    // {next, prev, ?, handle}.
                    let req = |k: u32| stream + 0x200 + 0x40 * k;
                    for k in 0..3 {
                        c.bus.write32(req(k), if k == 2 { 0 } else { req(k + 1) });
                        c.bus
                            .write32(req(k) + 4, if k == 0 { 0 } else { req(k - 1) });
                        c.bus.write32(req(k) + 8, r.next_u32());
                        c.bus.write32(req(k) + 0xc, i | (k + 1) << 8);
                    }
                    let active = stream + SNDSTRM_RETIRE_REQUEST_ACTIVE;
                    c.bus.write32(active, req(0));
                    c.bus.write32(active + 4, req(2));
                    c.bus.write32(active + 8, 3);
                    // Free list: one spare request, or empty.
                    let free = stream + SNDSTRM_RETIRE_REQUEST_FREE;
                    let spare = stream + 0x400;
                    let has_spare = r.next_u32() % 2 == 0;
                    c.bus.write32(spare, 0);
                    c.bus.write32(spare + 4, 0);
                    c.bus.write32(free, if has_spare { spare } else { 0 });
                    c.bus.write32(free + 4, if has_spare { spare } else { 0 });
                    c.bus.write32(free + 8, has_spare as u32);
                    let playing = match r.next_u32() % 3 {
                        0 => 0,
                        _ => req(r.next_u32() % 3),
                    };
                    c.bus
                        .write32(stream + SNDSTRM_RETIRE_REQUEST_PLAYING, playing);
                }
                let stream = r.next_u32() % 4;
                let k = 1 + seed % 3;
                c.g[A0][0] = sx(stream | k << 8);
            },
        );
    }

    /// `SND_DecUtk_InitMem`: the allocation hook (0x517604) is a fake that
    /// returns the test's UTK state block (or unwinds); the real
    /// `SND_UtkReset` / `GetState` / `SetState` / `SetMode` / `SetInput`
    /// run, for decoder modes below and from 3 and inputs with and without
    /// the 0xee marker byte.
    #[test]
    fn snd_dec_utk_init_mem_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{fill, DATA};
        const THIS: u32 = DATA + 0x100;
        const CFG: u32 = DATA + 0x200;
        const INPUT: u32 = DATA + 0x400;
        const UTK: u32 = DATA + 0x2000;
        covered(
            ee::s14::f_003c9420__generated,
            snd_dec_utk_init_mem,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_9420, 1);
                c.pc = 0x003c_9420;
                c.bus.write32(0x0051_7604, FAKE); // SND_UtkAlloc's allocator hook
                c.bus.write32(LOG_RESULTS, UTK);
                fill(c, &mut r, THIS, 0x10);
                fill(c, &mut r, CFG, 0x10);
                fill(c, &mut r, UTK + 0xd40, 0x0a);
                c.bus.write32(UTK + 4, r.next_u32());
                c.bus.write32(CFG, INPUT);
                c.bus.write32(CFG + DEC_UTK_CFG_COUNT, r.next_u32() % 0x400);
                c.bus.write32(
                    CFG + 0x24,
                    if seed % 7 == 0 {
                        (r.next_u32() % 4).wrapping_neg()
                    } else {
                        r.next_u32() % 6
                    },
                );
                c.bus.write8(
                    INPUT,
                    if seed % 3 == 0 {
                        0xee
                    } else {
                        r.next_u32() as u8
                    },
                );
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(CFG);
            },
        );
    }

    /// SND_InitPatchParams on a block of random bytes; event seeds deliver
    /// (or unwind at) the back edges of both clearing loops.
    #[test]
    fn snd_init_patch_params_matches() {
        check_events(
            ee::s13::f_003c7218__generated,
            snd_init_patch_params,
            120,
            &|c, seed| {
                let mut r = Rng(0x3c72_1800 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x20);
                let hi = if seed % 4 == 0 {
                    (r.next_u32() as u64) << 32
                } else {
                    0
                };
                c.g[A0][0] = hi | DATA as u64;
                let mode = schedule(c, &mut r, seed, 0x003c_7218);
                if mode != Events::Off && seed % 2 == 0 {
                    // Due late enough to land in the second loop.
                    c.next_event = c.cycles + 14 + 28 + (r.next_u32() % 40) as u64;
                }
                mode
            },
        );
    }

    /// SPCH_CheckEventMasks: condition counts -8..7 (none, negative, some),
    /// random table/entry sizes, zero masks, all-matching or sparse slot
    /// masks (an early failure); event seeds deliver at the loop's back edge.
    #[test]
    fn spch_check_event_masks_matches() {
        check_events(
            ee::s14::f_003d9ed0__generated,
            spch_check_event_masks,
            200,
            &|c, seed| {
                let mut r = Rng(0x3d9e_d000 ^ seed as u64);
                scramble(c, &mut r);
                let (sent, ev, slots) = (DATA, DATA + 0x1000, DATA + 0x2000);
                fill(c, &mut r, sent, 0x100);
                fill(c, &mut r, ev, 0x40);
                // Event seeds (seed % 3 != 0, see `schedule`) mostly get 4..7
                // conditions that all hold, so the loop runs its back edges.
                let evented = seed % 3 != 0 && seed % 5 != 0;
                let count = if evented {
                    4 + r.next_u32() % 4
                } else {
                    match seed % 4 {
                        0 => 0,
                        1 => 8 + r.next_u32() % 8,
                        _ => 1 + r.next_u32() % 7,
                    }
                };
                c.bus
                    .write8(sent + SPCH_SENT_B7, (count << 4 | r.next_u32() & 0xf) as u8);
                let b4 = c.bus.read8(ev + SPCH_EVENT_B4) as u32;
                let masks = ev + ((((b4 >> 2) + 3) & !3) + 8);
                for i in 0..8 {
                    let m = match r.next_u32() % 3 {
                        0 => 0,
                        _ => 1 << (r.next_u32() % 32),
                    };
                    c.bus.write32(masks + 4 * i, m);
                }
                let all = seed % 3 == 0 || evented;
                for i in 0..0x100 {
                    let w = if all {
                        u32::MAX
                    } else {
                        r.next_u32() | r.next_u32()
                    };
                    c.bus.write32(slots + 4 * i, w);
                }
                c.g[A0][0] = sx(sent);
                c.g[A1][0] = sx(ev);
                c.g[A2][0] = sx(slots);
                let mode = schedule(c, &mut r, seed, 0x003d_9ed0);
                if mode != Events::Off {
                    // Due by the first or second back edge (entry 9 + 22 + 1,
                    // then 5..13 + 4 cycles per condition).
                    c.next_event = c.cycles + 40 + (r.next_u32() % 12) as u64;
                }
                mode
            },
        );
    }

    /// SND_ChannelApplyLowpass: channels filtered or not, 0..12 voices,
    /// random voice numbers and index bases; cutoffs at or above half the
    /// rate, so the real SND_EeVoiceSetLowpass takes its "filter off, none
    /// installed" short path (the voice array is empty RAM). Event seeds
    /// deliver at the loop's back edge.
    #[test]
    fn snd_channel_apply_lowpass_matches() {
        check_events(
            ee::s13::f_003c3178__generated,
            snd_channel_apply_lowpass,
            150,
            &|c, seed| {
                let mut r = Rng(0x003c_3178 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let (chans, voices) = (DATA, DATA + 0x4_0000);
                c.bus.write32(SND_STATE + SND_STATE_CHANNELS, chans);
                c.bus.write32(0x0051_5d1c, voices);
                let rate = r.next_u32() % 0xc000;
                c.bus.write16(SND_STATE + SND_STATE_RATE, rate as u16);
                c.bus.write8(
                    SND_STATE + SND_STATE_VOICE_BASE_A,
                    (r.next_u32() % 16) as u8,
                );
                c.bus.write8(
                    SND_STATE + SND_STATE_VOICE_BASE_B,
                    (r.next_u32() % 16) as u8,
                );
                let index = r.next_u32() % 4;
                let chan = chans + index * SND_CHANNEL_SIZE;
                fill(c, &mut r, chan, SND_CHANNEL_SIZE / 4);
                let flags = r.next_u32() as u16 & !4 | if seed % 5 == 0 { 0 } else { 4 };
                c.bus.write16(chan + SND_CHANNEL_FLAGS, flags);
                let count = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 12
                };
                c.bus.write8(chan + SND_CHANNEL_VOICE_COUNT, count as u8);
                for i in 0..12 {
                    c.bus.write16(
                        chan + SND_CHANNEL_VOICES + 2 * i,
                        (r.next_u32() % 48) as u16,
                    );
                }
                c.g[A0][0] = index as u64 | (r.next_u32() as u64) << 32;
                c.g[A1][0] = (rate / 2 + 1 + r.next_u32() % 0x10000) as u64;
                let mode = schedule(c, &mut r, seed, 0x003c_3178);
                if mode != Events::Off {
                    c.next_event = c.cycles + 40 + (r.next_u32() % 200) as u64;
                }
                mode
            },
        );
    }

    /// `SND_EeVoiceSetHighpass`: positive cutoffs with the voice's highpass
    /// DSP present or created by the real `SND_MemAlloc` from an empty heap
    /// and inserted into a 0-3 node chain, then the real FIR design; zero or
    /// negative cutoffs with a DSP to remove (the real `SND_DspRemove`,
    /// `SND_MemFree` on an empty heap) or none.
    #[test]
    fn snd_ee_voice_set_highpass_matches() {
        use crate::difftest::fakes::machine;
        const VOICES: u32 = DATA + 0x20_0000;
        const NODES: u32 = DATA + 0x20_1000;
        const HEAP: u32 = DATA + 0x20_2000;
        crate::difftest::check_lohi(
            ee::s14::f_003c8c30__generated,
            snd_ee_voice_set_highpass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8c30, 0);
                c.pc = 0x003c_8c30;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.bus.write32(HIGHPASS_VOICES, VOICES);
                fill(c, &mut r, VOICES, HIGHPASS_VOICE_SIZE * 8 / 4);
                let voice = VOICES + HIGHPASS_VOICE_SIZE * index;
                // The DSP chain: 0-3 nodes {next +8, priority +0x18} (room
                // for a whole FIR node each); the highpass DSP (when
                // present) is one of them or not.
                let n = seed / 8 % 4;
                for i in 0..n {
                    let node = NODES + 0x200 * i;
                    fill(c, &mut r, node, 0x48);
                    c.bus
                        .write32(node + 8, if i + 1 == n { 0 } else { node + 0x200 });
                    c.bus
                        .write16(node + 0x18, (0x10 + 0x40 * i + r.next_u32() % 0x40) as u16);
                }
                c.bus.write32(
                    voice + HIGHPASS_VOICE_DSP_CHAIN,
                    if n == 0 { 0 } else { NODES },
                );
                let filter = match seed % 3 {
                    0 => 0,
                    1 if n > 0 => NODES + 0x200 * (r.next_u32() % n),
                    _ => NODES + 0x800,
                };
                if filter == NODES + 0x800 {
                    fill(c, &mut r, filter, 0x48);
                }
                c.bus.write32(voice + HIGHPASS_VOICE_FILTER, filter);
                // The driver heap: empty, room for the block.
                for state in [0x0051_5b40u32, SND_STATE] {
                    c.bus.write32(state + 0x234, HEAP);
                }
                c.bus.write32(HEAP, DATA + 0x20_4000);
                c.bus.write32(HEAP + 0x4, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0xc, 0x8_0000);
                c.bus.write32(HEAP + 0x10, 0x100);
                c.bus.write32(HEAP + 0x14, 0);
                c.bus.write16(
                    SND_STATE + SND_STATE_RATE,
                    [48000u32, 44100, 24000, r.next_u32() % 0x1_0000][(seed / 3 % 4) as usize]
                        as u16,
                );
                c.g[A1][0] = match seed / 5 % 5 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    2 => (r.next_u32() as u64) << 32 | (r.next_u32() % 20000) as u64,
                    _ => 20 + (r.next_u32() % 20000) as u64,
                };
            },
        );
    }

    /// A speech bank at `DATA` with random entries (some aliases), an index
    /// in range, past the end, or -1, and the output record at `DATA+0x800`.
    pub(crate) fn spch_decode_sample_entry_setup(
        c: &mut Cpu,
        r: &mut crate::difftest::Rng,
        seed: u32,
    ) {
        let bank = DATA;
        let extra = if seed % 5 == 0 { 0 } else { r.next_u32() % 16 };
        let count = 1 + r.next_u32() % 12;
        c.bus.write8(
            bank + SPCH_DECODE_FLAGS,
            ((r.next_u32() & 0xf0) | extra) as u8,
        );
        c.bus.write8(bank + SPCH_DECODE_COUNT, count as u8);
        c.bus.write8(bank + SPCH_DECODE_BLOCK, r.next_u32() as u8);
        c.bus.write16(bank + SPCH_DECODE_END, r.next_u32() as u16);
        let size = extra + 2;
        for i in 0..count + 2 {
            let e = bank + SPCH_DECODE_ENTRIES + i * size;
            for k in 0..size {
                c.bus.write8(e + k, r.next_u32() as u8);
            }
            if r.next_u32() % 3 == 0 {
                // An alias of another entry.
                c.bus.write8(e, (0x80 | r.next_u32()) as u8);
                c.bus.write8(e + 1, (r.next_u32() % (count + 1)) as u8);
            } else {
                c.bus.write8(e, (r.next_u32() & 0x7f) as u8);
            }
        }
        let index = match seed % 9 {
            0 => count as u64 + (r.next_u32() % 3) as u64,
            1 => u64::MAX,
            _ => (r.next_u32() % count) as u64,
        };
        c.g[A0][0] = sx(bank);
        c.g[A1][0] = index;
        c.g[A2][0] = sx(DATA + 0x800);
        c.pc = 0x003d_b838;
    }

    #[test]
    fn spch_decode_sample_entry_matches() {
        use crate::difftest::{scramble, Rng};
        check_lohi(
            ee::s14::f_003db838__generated,
            spch_decode_sample_entry,
            300,
            &|c, seed| {
                let mut r = Rng(0x3db8_3800 ^ seed as u64);
                scramble(c, &mut r);
                spch_decode_sample_entry_setup(c, &mut r, seed);
            },
        );
    }

    /// Events delivered (and unwinds requested) at both loops' back edges.
    #[test]
    fn spch_decode_sample_entry_events_match() {
        use crate::difftest::events::{check_events, schedule};
        use crate::difftest::{scramble, Rng};
        check_events(
            ee::s14::f_003db838__generated,
            spch_decode_sample_entry,
            300,
            &|c, seed| {
                let mut r = Rng(0x3db8_3801 ^ seed as u64);
                scramble(c, &mut r);
                spch_decode_sample_entry_setup(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x003d_b838)
            },
        );
    }

    /// `SPCH_NotifyPlayCallback`: a sentence of 0-4 slot entries over four
    /// random banks (with and without variation bits, samples in and out of
    /// range so the decode fails or succeeds, variation -1 or not), the
    /// marking flag on/off, total 0 or not, no callback; the callback is a
    /// fake that may unwind.
    #[test]
    fn spch_notify_play_callback_matches() {
        covered(
            ee::s14::f_003d9830__generated,
            spch_notify_play_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d98_30, 4);
                c.pc = 0x003d_9830;
                let gp = GAME_GP;
                let callback = if seed % 9 == 0 { 0 } else { FAKE };
                c.bus.write32(SPCH_PLAY_CALLBACK, callback);
                // The key {u16, id lo, id hi} and the bank-type table at
                // 0x44ffb8 (8 x {bank*, user data}); some keys match entry 3.
                let key = DATA + 1 + (seed & 2); // maybe unaligned
                fill(c, &mut r, DATA, 4);
                for i in 0..8u32 {
                    let e = 0x0044_ffb8 + i * 8;
                    let p = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x400 + i * 0x10
                    };
                    c.bus.write32(e, p);
                    c.bus.write32(e + 4, r.next_u32());
                    if p != 0 {
                        fill(c, &mut r, p, 4);
                    }
                }
                if seed % 2 == 0 {
                    let p = DATA + 0x400 + 3 * 0x10;
                    c.bus.write32(0x0044_ffb8 + 3 * 8, p);
                    let (id_lo, id_hi) = (c.bus.read8(key + 2), c.bus.read8(key + 3));
                    c.bus.write8(p + 9, id_lo);
                    c.bus.write8(p + 8, id_hi);
                }
                // Banks: four pointers at gp+0x1734.
                let banks = DATA + 0x800;
                c.bus.write32(gp + SPCH_GP_BANKS, banks);
                for k in 0..4u32 {
                    let b = DATA + 0x10000 + k * 0x1000;
                    c.bus.write32(banks + 4 * k, b);
                    fill(c, &mut r, b, 0x200);
                    let flags = (r.next_u32() & 0x80) as u8 | (r.next_u32() % 4) as u8;
                    c.bus.write8(b + SPCH_BANK_FLAGS, flags);
                    c.bus
                        .write8(b + SPCH_BANK_COUNT, (1 + r.next_u32() % 8) as u8);
                    c.bus.write8(b + SPCH_BANK_BLOCKS, (r.next_u32() % 4) as u8);
                }
                c.bus
                    .write32(gp + SPCH_GP_MARK_PLAYED, if seed % 3 == 0 { 0 } else { 1 });
                let total = if seed % 5 == 0 {
                    0
                } else {
                    r.next_u32() % 5000 + 1
                };
                c.bus.write32(gp + SPCH_GP_TOTAL_LEN, total);
                // The sentence: flags byte at +4 (entry count << 2).
                let sentence = DATA + 0x100;
                let count = if seed % 11 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                c.bus
                    .write8(sentence + 4, (count << 2 | r.next_u32() & 3) as u8);
                let slot = r.next_u32() % 4;
                for i in 0..count {
                    let e = SPCH_SENTENCE_SLOTS + (slot + i) * 8;
                    c.bus.write16(e, (r.next_u32() % 4) as u16);
                    let variation = if r.next_u32() % 2 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write16(e + 2, variation as u16);
                    c.bus.write16(e + 4, (r.next_u32() % 10) as u16);
                }
                c.g[A0][0] = sx(sentence);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = slot as u64;
            },
        );
    }

    /// `SNDSTRM_StartChannel` fixture: a stream channel record (0-4 voice
    /// parameter words), a header with 1-3 voices (maybe unaligned), random
    /// info/params, and a voice-group table of 0-3 groups whose masks are 0
    /// (rejected), 0x8 (voices [0, n8)) or 0x100 (voices [n8, n8 + n100))
    /// over up to 8 sound channels, each free or busy with a priority the
    /// allocator will not steal (so SND_Stop is never reached). Groups with
    /// too few free voices fail (-9, next group); the first that fits sets
    /// the channel up. SND_ChannelStartStreamVoices always returns 0, so the
    /// release path (0x3c6a20..0x3c6a64) is not reachable with real callees.
    pub(crate) fn sndstrm_start_channel_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        let mut r = machine(c, seed, 0x3c67_60, 0);
        c.pc = 0x003c_6760;
        c.g[RA][0] = crate::difftest::RA as u64;
        let stream = r.next_u32() % 4;
        let rec = DATA + 0x100;
        fill(c, &mut r, rec, 0x40);
        c.bus.write32(SNDSTRM_CHANNELS + 4 * stream, rec);
        c.bus
            .write8(rec + SNDSTRM_CH_VOICES, (r.next_u32() % 5) as u8);
        let hdr = DATA + 0x400 + (seed & 3);
        fill(c, &mut r, DATA + 0x400, 4);
        let voices = 1 + r.next_u32() % 3;
        c.bus.write8(hdr + 2, voices as u8);
        let info = DATA + 0x500;
        fill(c, &mut r, info, 0x10);
        c.bus.write16(info + 8, 0); // request flags: any group
        c.bus.write16(info, (r.next_u32() % 4000) as u16); // pitch cents
        let params = DATA + 0x600;
        fill(c, &mut r, params, 0x10);
        // The voice groups.
        let state = SNDSTRM_SND_STATE;
        let groups = r.next_u32() % 4;
        c.bus.write8(state + 0x4c, groups as u8);
        for i in 0..4 {
            let m = match r.next_u32() % 4 {
                0 => 0,
                1 => 0x100,
                _ => 0x8,
            } | (r.next_u32() & 0x60);
            let m = if m & 0x108 == 0 { 0 } else { m };
            c.bus.write16(state + 0x4e + 2 * i, m as u16);
        }
        let n8 = r.next_u32() % 5;
        let n100 = r.next_u32() % 4;
        c.bus.write8(state + 0x30, n8 as u8);
        c.bus.write8(state + 0x2f, n100 as u8);
        c.bus.write8(state + 0x3c, r.next_u32() as u8);
        c.bus.write32(0x0044_c478, r.next_u32() & 0x7fff_ff00);
        // The sound channels.
        let chans = DATA + 0x10000;
        c.bus.write32(state + SNDSTRM_SND_CHANNELS, chans);
        for k in 0..8 {
            let ch = chans + k * SNDSTRM_SND_CHANNEL_SIZE;
            fill(c, &mut r, ch, SNDSTRM_SND_CHANNEL_SIZE / 4);
            let busy = r.next_u32() % 3 == 0;
            c.bus.write8(ch + 0x69, busy as u8);
            if busy {
                c.bus.write8(ch + 0x2a, 0x70);
            }
        }
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = sx(hdr);
        c.g[A2][0] = sx(info);
        c.g[A3][0] = sx(params);
        r
    }

    crate::difftest::events::driven!(sndstrm_start_channel_gen, ee::s13::f_003c6760__generated);

    crate::difftest::events::driven!(sndstrm_start_channel_hand, sndstrm_start_channel);

    #[test]
    fn sndstrm_start_channel_matches() {
        crate::difftest::check_lohi(
            ee::s13::f_003c6760__generated,
            sndstrm_start_channel,
            300,
            &|c, seed| {
                sndstrm_start_channel_setup(c, seed);
            },
        );
        crate::difftest::events::check_events(
            sndstrm_start_channel_gen,
            sndstrm_start_channel_hand,
            150,
            &|c, seed| {
                let mut r = sndstrm_start_channel_setup(c, seed);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, 0x003c_6760);
                // The first due event must land on this function's own back
                // edge: callees (SND_ChannelCalcPitch, ...) are dispatchers
                // whose unwind would resume at a pc that is not an entry of
                // ours. With 2-4 record words the record reset loop runs
                // before any call, and its first back edge (0x3c6818, ~60
                // cycles in) takes the event; delivering it reschedules the
                // next one far past this function.
                let record = DATA + 0x100;
                c.bus
                    .write8(record + SNDSTRM_CH_VOICES, (2 + r.next_u32() % 3) as u8);
                if mode != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles + 1;
                }
                mode
            },
        );
    }
}
