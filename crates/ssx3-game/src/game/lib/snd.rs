//! `src/lib/snd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_fcc;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::Resume;
pub(crate) use crate::game::ai::ai::at as at_2;
use crate::game::misc::STATUS_EIE;
use crate::game::visualfx::boardwakefx as bw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
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
    (0x003b_54e0, snd_run_change_callbacks, "SND_RunChangeCallbacks"),
    (0x003b_6ea8, sndstrm_parse_data, "SNDSTRM_ParseData"),
    (0x003b_7e70, sndstrm_set_volume, "SNDSTRM_SetVolume"),
    (0x003b_7f18, sndstrm_get_volume, "SNDSTRM_GetVolume"),
    (0x003b_86c0, snd_get_volume, "SND_GetVolume"),
    (0x003b_a078, snd_is_voice_reserved, "SND_IsVoiceReserved"),
    (0x003c_60e0, snd_run_service_callbacks, "SND_RunServiceCallbacks"),
    (0x003c_6ab8, sndstrm_push_block, "SNDSTRM_PushBlock"),
    (0x003c_6c28, sndstrm_get_free_blocks, "SNDSTRM_GetFreeBlocks"),
    (0x003c_72f8, snd_read_big_endian, "SND_ReadBigEndian"),
    (0x003c_7ab0, snd_ee_voice_mix_ramp, "SND_EeVoiceMixRamp"),
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

/// `addiu_2`-style 32-bit add of an immediate, sign-extended.
#[inline]
pub(crate) fn addiu_2(a: u64, k: i32) -> u64 {
    sx((a as u32).wrapping_add(k as u32))
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
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::lib::sif::tests::SEEDS;

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
    use super::*;

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
    use super::*;

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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::fakes::{ptr, FAKE, LOG_RESULTS};
    use crate::difftest::fresh;

    pub(crate) use crate::game::ai::ai::ai_render;
    pub(crate) use crate::game::ai::ai::rider_prewind_sub_handle_input;
    pub(crate) use crate::game::ai::ai::rider_spin_sub_update_trick;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_ground_state_compute_boost_accel;
    pub(crate) use crate::game::ai::rider::rider_begin_frame;
    pub(crate) use crate::game::camera::camera::tests::f;

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
        use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

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
        use crate::difftest::{Rng, DATA, GAME_GP};
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
        use crate::difftest::fakes::{machine, vtable, FAKE};
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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::check_paths;
    use crate::game::visualfx::boardwakefx::tests::{gp_at, put_spray_slot};

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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::require_paths;
    use crate::difftest::SP;
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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;

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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;


    /// SND_RunChangeCallbacks: counter unchanged, changed with an empty
    /// list, and changed with 1-4 callbacks (fakes).
    #[test]
    fn snd_run_change_callbacks_matches() {
        use crate::difftest::fakes::FAKE;
        const NODES: u32 = DATA + 0x20_0000;
        check(ee::s13::f_003b54e0__generated, snd_run_change_callbacks, 240, &|c, seed| {
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
        });
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
        check(ee::s13::f_003b7f18__generated, sndstrm_get_volume, 300, &|c, seed| {
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
                c.bus.write32(cp, if r.next_u32() % 4 == 0 { r.next_u32() } else { 0x100 * (k + 1) | ch });
                c.bus.write16(VOLUMES + 0x8c * ch + 0x3a, r.next_u32() as u16);
            }
        });
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
        check(ee::s13::f_003b6ea8__generated, sndstrm_parse_data, 300, &|c, seed| {
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
        });
    }


    /// SND_RunServiceCallbacks: disabled, enabled with a count of 0, a
    /// negative count, or 1..5 fake callbacks (each may unwind).
    #[test]
    fn snd_run_service_callbacks_matches() {
        covered(ee::s13::f_003c60e0__generated, snd_run_service_callbacks, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3c_60e0, 5);
            c.pc = 0x003c_60e0;
            let on = if seed % 5 == 0 { 0 } else { 1 + r.next_u32() % 0x7f };
            c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_ON, on as u8);
            let count: i8 = match seed % 7 {
                0 => 0,
                1 => -3,
                _ => (1 + r.next_u32() % 5) as i8,
            };
            c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_COUNT, count as u8);
            for i in 0..6 {
                c.bus.write32(SND_GLOBALS_2 + SND_SERVICE_FNS + 4 * i, FAKE + 0x10 * i);
            }
        });
    }

    // -- 0x003ba078 SND_IsVoiceReserved ----------------------------------------

    /// `SND_IsVoiceReserved` against the generated body: counts from
    /// negative to 24, voices that are in the scanned part of the table, past
    /// it, or nowhere (and a voice with junk upper bits that never matches).
    #[test]
    fn snd_is_voice_reserved_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(ee::s13::f_003ba078__generated, snd_is_voice_reserved, 300, &|c, seed| {
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
        });
    }

    // -- 0x003c6c28 SNDSTRM_GetFreeBlocks --------------------------------------

    /// `SNDSTRM_GetFreeBlocks` against the generated body: slots 0..7 with
    /// 0..8 channels whose block indices are positive, negative (ignored:
    /// the maximum starts at 0) or equal, and signed block counts.
    #[test]
    fn sndstrm_get_free_blocks_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        check(ee::s13::f_003c6c28__generated, sndstrm_get_free_blocks, 300, &|c, seed| {
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
            let blocks = if seed % 5 == 0 { r.next_u32() as u16 } else { (r.next_u32() % 64) as u16 };
            c.bus.write16(state + 0x40, blocks);
        });
    }

    // -- 0x003c6ab8 SNDSTRM_PushBlock ------------------------------------------

    /// `SNDSTRM_PushBlock` against the generated body: slots 0..7 with 0..8
    /// channels, positions below the last ring entry or at/after it (full),
    /// write indices before the end, at the last entry (wrap) and past it,
    /// size words with the top bit set or clear.
    #[test]
    fn sndstrm_push_block_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(ee::s13::f_003c6ab8__generated, sndstrm_push_block, 300, &|c, seed| {
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
            let entries = if seed % 11 == 0 { r.next_u32() as u16 } else { (1 + r.next_u32() % 16) as u16 };
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
        });
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
        check(ee::s13::f_003b7e70__generated, sndstrm_set_volume, 300, &|c, seed| {
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
        });
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
        crate::difftest::check_lohi(ee::s13::f_003b86c0__generated, snd_get_volume, 240, &|c, seed| {
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
                c.bus.write32(at, if seed % 6 == 2 { handle ^ 0x100 } else { handle });
                c.bus.write16(params + ch * 0x8c + 0x3a, r.next_u32() as u16);
            }
        });
    }
}
