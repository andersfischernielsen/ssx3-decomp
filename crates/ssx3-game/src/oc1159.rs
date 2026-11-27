//! Staging module oc1159 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1159`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_b280, snd_dsp_stretch_init, "SND_DspStretch_Init"),
    (
        0x003c_c938,
        snd_eaxa_decode_raw_block,
        "SND_EaxaDecodeRawBlock",
    ),
    (
        0x003c_e198,
        sndstrm_slot_mem_stub_return_zero,
        "SNDSTRM_SlotMemStub_ReturnZero",
    ),
    (
        0x003c_e1a0,
        sndstrm_alloc_slot_stub_return_zero,
        "SNDSTRM_AllocSlotStub_ReturnZero",
    ),
    (
        0x003c_e1a8,
        sndstrm_free_slot_stub_return_zero,
        "SNDSTRM_FreeSlotStub_ReturnZero",
    ),
    (0x003c_eb90, snd_fir_reset, "SND_FirReset"),
    (
        0x003c_ee08,
        snd_dsp_fir_lowpass_process,
        "SND_DspFirLowpass_Process",
    ),
    (0x003c_ee88, snd_dsp_fir_lowpass_init, "SND_DspFirLowpass_Init"),
];

/// Offsets of the time-stretch node that `SND_DspStretch_Init` fills in.
pub(crate) mod dsp_stretch_node {
    /// The filter function (`SFILTER_timestretch_B280`, `0x3d0000 - 0x4e58`).
    pub const FILTER_FN: u32 = 0x003d_0000 - 0x4e58;
    /// `void*`: the restore function (cleared).
    pub const RESTORE_FN: u32 = 0x4;
    /// `u8*`: the sideband, `header + 6`.
    pub const SIDEBAND: u32 = 0x1c;
    /// `int`: the packet instance of the voice's master channel, or -1.
    pub const PACKET_HANDLE: u32 = 0x20;
    /// `float`: the stretch ratio, 1.0.
    pub const STRETCH_RATIO: u32 = 0x24;
    /// `int`: the sample error, 0.
    pub const SAMPLE_ERROR: u32 = 0x28;
    /// `int`: `header[1] * 2`.
    pub const WINDOW_SIZE: u32 = 0x2c;
    /// `int`: `SNDI_getb(header + 2, 4)`.
    pub const FRAMES_LEFT: u32 = 0x30;
    /// `int`: the input sample counter, 0.
    pub const INPUT_SAMPLES: u32 = 0x34;
    /// `int`: the output sample counter, 0.
    pub const OUTPUT_SAMPLES: u32 = 0x38;
    /// `int`: the output start, 0.
    pub const OUTPUT_START: u32 = 0x3c;
}

/// Offsets of the low-pass FIR node (`SFILTER`) and its `SNDFIRSTATE`.
pub(crate) mod fir_node {
    /// The restore function (cleared).
    pub const RESTORE_FN: u32 = 0x4;
    /// `pfnnext` (the next filter object, `obj`).
    pub const PFN_NEXT: u32 = 0x8;
    /// `pfnnext2` (cleared).
    pub const PFN_NEXT2: u32 = 0xc;
    /// `pfnprev1` (cleared).
    pub const PFN_PREV1: u32 = 0x10;
    /// `pfnprev2` (cleared).
    pub const PFN_PREV2: u32 = 0x14;
    /// `clean` (a byte, cleared).
    pub const CLEAN: u32 = 0x1b;
    /// The `SNDFIRSTATE` the FIR routines work on.
    pub const FIR_STATE: u32 = 0x20;
    /// `SFILTER_lpfFIR8_x`: the filter function (`0x3d0000 - 0x11f8`).
    pub const FILTER_FN: u32 = 0x003d_0000 - 0x11f8;
}

/// `int SND_EeVoiceGetMasterChannel(int voice)` (0x3ce0e0).
pub(crate) const STRETCH_MASTER_CHANNEL: Callee = Callee {
    addr: 0x003c_e0e0,
    f: ee::s14::SND_EeVoiceGetMasterChannel,
};

/// `int SNDSTRM_FindByChannel(int channel)` (0x3ce378).
pub(crate) const STRETCH_FIND_BY_CHANNEL: Callee = Callee {
    addr: 0x003c_e378,
    f: ee::s14::SNDSTRM_FindByChannel,
};

/// `void SND_FirProcess(SNDFIRSTATE* s, int count, ...)` (0x3cebe8): the FIR run.
pub(crate) const FIR_PROCESS: Callee = Callee {
    addr: 0x003c_ebe8,
    f: ee::s14::SND_FirProcess,
};

/// `void SND_FirReset(SNDFIRSTATE* s)` (0x3ceb90).
pub(crate) const FIR_RESET: Callee = Callee {
    addr: 0x003c_eb90,
    f: ee::s14::SND_FirReset,
};

/// The FIR state words `SND_FirReset` clears (history and matrix words).
pub(crate) const FIR_RESET_WORDS: [u32; 20] = [
    0xb8, 0xc0, 0xc4, 0xc8, 0xcc, 0xd0, 0xd4, 0xd8, 0xdc, 0x4, 0x8, 0xc, 0x18, 0x1c, 0x2c, 0x90,
    0xa0, 0xa4, 0xb0, 0xb4,
];

/// One sample of `SND_EaxaDecodeRawBlock`: `(float)*(short*)swapped` where
/// `swapped[0] = *hi` and `swapped[1] = *lo` (the stack's two scratch bytes at
/// SP). Leaves the halfword in V0 and V1 as the original did, and stores the
/// float to `dst` (f0).
fn raw_sample_to(c: &mut Cpu, hi: u32, lo: u32, dst: u32) {
    let scratch = sp(c);
    let high = Addr(hi).lbu(c);
    c.g[V0][0] = high;
    Addr(scratch).sb(c, high);
    let low = Addr(lo).lbu(c);
    c.g[V1][0] = low;
    Addr(scratch.wrapping_add(1)).sb(c, low);
    c.g[V0][0] = Addr(scratch).lh(c);
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.swc1(0, dst);
}

/// The epilogue of `SND_DspFirLowpass_Process`: restore s1..s3 and ra from
/// the frame, pop it and return.
fn fir_process_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void SFILTER_timestretchinit(SFilter* node, const u8* header, int voice)`
/// (`SND_DspStretch_Init`, 0x3cb280): fill the time-stretch node from the
/// stream header. The packet instance is the voice's stream handle (-1 for
/// no voice); the sideband is `header + 6`; the window is `header[1] * 2`;
/// the frame count is `SNDI_getb(header + 2, 4)`. Returns 0.
pub fn snd_dsp_stretch_init(c: &mut Cpu) {
    guest(c, |c| {
        use dsp_stretch_node as n;
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[A2][0];
        c.g[V0][0] = sx(n::FILTER_FN);
        let filter = c.g[V0][0];
        Addr(r32(c, S0)).sw(c, filter);
        Addr(r32(c, S0).wrapping_add(n::RESTORE_FN)).sw(c, 0);
        // The voice's packet handle. A0 holds the voice.
        if (c.g[A0][0] as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            let handle = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(n::PACKET_HANDLE)).sw(c, handle);
        } else {
            c.cycles += 6;
            STRETCH_MASTER_CHANNEL.call(c, 0x003c_b2b8)?;
            c.g[A0][0] = c.g[V0][0];
            STRETCH_FIND_BY_CHANNEL.call(c, 0x003c_b2c0)?;
            let handle = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(n::PACKET_HANDLE)).sw(c, handle);
        }
        c.cycles += 22;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x6));
        let sideband = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(n::SIDEBAND)).sw(c, sideband);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x2));
        let ratio_at = r32(c, S0).wrapping_add(n::STRETCH_RATIO);
        c.swc1(0, ratio_at);
        c.g[A1][0] = 4;
        Addr(r32(c, S0).wrapping_add(n::SAMPLE_ERROR)).sw(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x1)).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        let window = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(n::WINDOW_SIZE)).sw(c, window);
        call_ptr(c, 0x003c_72f8, 0x003c_b300)?;
        let node = r32(c, S0);
        let frames = c.g[V0][0];
        Addr(node.wrapping_add(n::FRAMES_LEFT)).sw(c, frames);
        Addr(node.wrapping_add(n::OUTPUT_START)).sw(c, 0);
        c.g[V0][0] = 0;
        Addr(node.wrapping_add(n::INPUT_SAMPLES)).sw(c, 0);
        Addr(node.wrapping_add(n::OUTPUT_SAMPLES)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SND_EaxaDecodeRawBlock(EaxaState* s)` (0x3cc938): an uncompressed
/// (0xee) EA-XA block. After the header byte come 30 big-endian halfwords,
/// each converted to float: the first two go to the state's `sample1` and
/// `sample2` (+4, +8), the other 28 to the output at `pdst` (+0x10). The input
/// pointer (+0xc) moves past them. A block with another header leaves the
/// state alone.
pub fn snd_eaxa_decode_raw_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A3][0] = c.g[A0][0];
        c.g[A2][0] = Addr(r32(c, A3).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = 0xee;
        c.g[V0][0] = Addr(r32(c, A2)).lbu(c);
        let raw = c.g[V0][0] == c.g[V1][0];
        let frame = Frame::open(c, 0x10);
        if raw {
            c.cycles += 24;
            let psrc = r32(c, A2);
            // psrc++ and the first sample (psrc[1], psrc[0]), then psrc += 2.
            c.g[V1][0] = sx(psrc.wrapping_add(1));
            c.g[A0][0] = sx(psrc.wrapping_add(3));
            let v1 = c.g[V1][0];
            Addr(r32(c, A3).wrapping_add(0xc)).sw(c, v1);
            c.g[A1][0] = sx(psrc.wrapping_add(5));
            c.g[T0][0] = 0;
            let first_dst = r32(c, A3).wrapping_add(0x4);
            raw_sample_to(c, psrc.wrapping_add(2), psrc.wrapping_add(1), first_dst);
            let a0 = c.g[A0][0];
            Addr(r32(c, A3).wrapping_add(0xc)).sw(c, a0);
            // The second sample.
            let src = r32(c, A0);
            let second_dst = r32(c, A3).wrapping_add(0x8);
            raw_sample_to(c, src.wrapping_add(1), src, second_dst);
            let a1 = c.g[A1][0];
            Addr(r32(c, A3).wrapping_add(0xc)).sw(c, a1);
            // The 28 samples: loop head 0x3cc9b0.
            loop {
                c.cycles += 19;
                c.g[A0][0] = Addr(r32(c, A3).wrapping_add(0xc)).lw(c);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[A1][0] = Addr(r32(c, A3).wrapping_add(0x10)).lw(c);
                c.g[A2][0] = (c.g[T0][0] < 0x1cu64) as u64;
                let src = r32(c, A0);
                let dst = r32(c, A1);
                raw_sample_to(c, src.wrapping_add(1), src, dst);
                c.g[V0][0] = Addr(r32(c, A3).wrapping_add(0x10)).lw(c);
                c.g[V1][0] = Addr(r32(c, A3).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
                let v0 = c.g[V0][0];
                Addr(r32(c, A3).wrapping_add(0x10)).sw(c, v0);
                let more = c.g[A2][0] != 0;
                let v1 = c.g[V1][0];
                Addr(r32(c, A3).wrapping_add(0xc)).sw(c, v1);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_c9b0);
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_SlotMemStub_ReturnZero` (0x3ce198): a stub that returns 0.
pub fn sndstrm_slot_mem_stub_return_zero(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `SNDSTRM_AllocSlotStub_ReturnZero` (0x3ce1a0): a stub that returns 0.
pub fn sndstrm_alloc_slot_stub_return_zero(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `SNDSTRM_FreeSlotStub_ReturnZero` (0x3ce1a8): a stub that returns 0.
pub fn sndstrm_free_slot_stub_return_zero(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `void SND_FirReset(SNDFIRSTATE* s)` (0x3ceb90): clear the FIR state's
/// history and matrix words (`s` in A0).
pub fn snd_fir_reset(c: &mut Cpu) {
    c.cycles += 21;
    let state = r32(c, A0);
    for off in FIR_RESET_WORDS {
        Addr(state.wrapping_add(off)).sw(c, 0);
    }
    ret(c);
}

/// `int SND_DspFirLowpass_Process(SFilter* node, int count, float* dst,
/// float* src, int requester)` (0x3cee08): run the chained filter object
/// first (`node->pfnnext`, its function returns the count it took); when
/// that is positive or there is no object, run the FIR on `node->fir`, and
/// return `count`. Keeps the caller's s0 and s1..s3 in the frame.
pub fn snd_dsp_fir_lowpass_process(c: &mut Cpu) {
    guest(c, |c| {
        use fir_node as n;
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(n::PFN_NEXT)).lw(c);
        c.g[S3][0] = c.g[A3][0];
        if c.g[A0][0] != 0 {
            c.cycles += 7;
            let vfn = Addr(r32(c, A0)).lw(c);
            c.g[V0][0] = vfn;
            c.g[T0][0] = Addr(r32(c, S0).wrapping_add(0x1a)).lbu(c);
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, vfn as u32, 0x003c_ee4c)?;
            if (c.g[V0][0] as i64) <= 0 {
                frame.restore(c, S0, 0x40);
                fir_process_epilogue(c, frame);
                return Ok(());
            }
        }
        c.cycles += 7;
        let fir = r32(c, S0).wrapping_add(n::FIR_STATE);
        c.g[A0][0] = sx(fir);
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        FIR_PROCESS.call(c, 0x003c_ee68)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        fir_process_epilogue(c, frame);
        Ok(())
    });
}

/// `void SND_DspFirLowpass_Init(SFilter* self)` (0x3cee88): set the node to
/// the low-pass FIR filter with every chain pointer and the clean flag
/// cleared, then reset its FIR state. Returns 0.
pub fn snd_dsp_fir_lowpass_init(c: &mut Cpu) {
    guest(c, |c| {
        use fir_node as n;
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[A0][0];
        let node = r32(c, V0);
        c.g[V1][0] = sx(n::FILTER_FN);
        c.g[A0][0] = sx(node.wrapping_add(n::FIR_STATE));
        let filter = c.g[V1][0];
        Addr(node).sw(c, filter);
        Addr(node.wrapping_add(n::PFN_NEXT)).sw(c, 0);
        Addr(node.wrapping_add(n::PFN_NEXT2)).sw(c, 0);
        Addr(node.wrapping_add(n::PFN_PREV1)).sw(c, 0);
        Addr(node.wrapping_add(n::PFN_PREV2)).sw(c, 0);
        Addr(node.wrapping_add(n::CLEAN)).sb(c, 0);
        Addr(node.wrapping_add(n::RESTORE_FN)).sw(c, 0);
        FIR_RESET.call(c, 0x003c_eec0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// The EA-XA raw-block fixture: the struct at DATA (psrc at +0xc, pdst at
    /// +0x10), the input at DATA+0x100. Raw (0xee) headers for seeds not
    /// divisible by 3, other headers otherwise.
    fn eaxa_raw_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        scramble(c, r);
        let st = DATA;
        let input = DATA + 0x100;
        c.g[A0][0] = sx(st);
        c.bus.write32(st + 0xc, input);
        c.bus.write32(st + 0x10, DATA + 0x1000);
        fill(c, r, input, 0x40);
        if seed % 3 == 0 {
            let h = r.next_u32() as u8;
            c.bus.write8(input, if h == 0xee { 0x12 } else { h });
        } else {
            c.bus.write8(input, 0xee);
        }
    }

    #[test]
    fn snd_dsp_stretch_init_matches() {
        check(
            ee::s14::f_003cb280__generated,
            snd_dsp_stretch_init,
            240,
            &|c, seed| {
                let mut r = Rng(0x003c_b280 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                // Voice -1 (no packet), or a voice whose stream is looked up.
                let voice: i64 = if seed % 4 == 0 {
                    -1
                } else {
                    (r.next_u32() % 40) as i64
                };
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = voice as u64;
                fill(c, &mut r, DATA + 0x100, 0x20);
            },
        );
    }

    #[test]
    fn snd_eaxa_decode_raw_block_matches() {
        check(
            ee::s14::f_003cc938__generated,
            snd_eaxa_decode_raw_block,
            240,
            &|c, seed| {
                let mut r = Rng(0x003c_c938 ^ (seed as u64) << 8);
                eaxa_raw_setup(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn snd_eaxa_decode_raw_block_events_matches() {
        check_events(
            ee::s14::f_003cc938__generated,
            snd_eaxa_decode_raw_block,
            600,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0xc938, 0);
                eaxa_raw_setup(c, seed, &mut r);
                schedule(c, &mut r, seed, 0x003c_c938)
            },
        );
    }

    #[test]
    fn sndstrm_slot_mem_stub_return_zero_matches() {
        check(
            ee::s14::f_003ce198__generated,
            sndstrm_slot_mem_stub_return_zero,
            40,
            &|c, seed| {
                let mut r = Rng(0x003c_e198 ^ (seed as u64) << 8);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn sndstrm_alloc_slot_stub_return_zero_matches() {
        check(
            ee::s14::f_003ce1a0__generated,
            sndstrm_alloc_slot_stub_return_zero,
            40,
            &|c, seed| {
                let mut r = Rng(0x003c_e1a0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn sndstrm_free_slot_stub_return_zero_matches() {
        check(
            ee::s14::f_003ce1a8__generated,
            sndstrm_free_slot_stub_return_zero,
            40,
            &|c, seed| {
                let mut r = Rng(0x003c_e1a8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_fir_reset_matches() {
        check(
            ee::s14::f_003ceb90__generated,
            snd_fir_reset,
            60,
            &|c, seed| {
                let mut r = Rng(0x003c_eb90 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn snd_dsp_fir_lowpass_process_matches() {
        check(
            ee::s14::f_003cee08__generated,
            snd_dsp_fir_lowpass_process,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0xee08, 0);
                let node = DATA;
                let obj = DATA + 0x400;
                // seed % 3: no chained object (FIR only); an object whose
                // function returns 0 (early exit, a real leaf); an object
                // whose function is the fake (positive, FIR after it).
                let vfn = match seed % 3 {
                    0 => 0,
                    1 => 0x0014_06c0,
                    _ => fakes::FAKE,
                };
                c.bus.write32(node + 0x8, if seed % 3 == 0 { 0 } else { obj });
                c.bus.write32(obj, vfn);
                c.bus.write8(node + 0x1a, r.next_u32() as u8);
                c.g[A0][0] = sx(node);
                // FirProcess steps 0x40 bytes per pass and ends only on pointer
                // equality: the count must be a multiple of 16.
                c.g[A1][0] = 16 * (1 + (seed % 3) as u64);
                c.g[A2][0] = sx(DATA + 0x1000);
                c.g[A3][0] = sx(DATA + 0x2000);
                fill(c, &mut r, DATA + 0x1000, 0x80);
            },
        );
    }

    #[test]
    fn snd_dsp_fir_lowpass_init_matches() {
        check(
            ee::s14::f_003cee88__generated,
            snd_dsp_fir_lowpass_init,
            60,
            &|c, seed| {
                let mut r = Rng(0x003c_ee88 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
