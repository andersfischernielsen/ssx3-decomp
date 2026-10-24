//! `src/lib/snd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
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
}
