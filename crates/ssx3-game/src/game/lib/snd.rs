//! `src/lib/snd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
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
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
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
    use crate::difftest::fakes::{self};
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
}
