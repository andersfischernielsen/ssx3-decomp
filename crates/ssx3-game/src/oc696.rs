//! Staging module oc696 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc696`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003c_92f8,
        snd_register_utk_decoders,
        "SND_RegisterUtkDecoders",
    ),
    (0x003c_9618, snd_dec_utk_init_loop, "SND_DecUtk_InitLoop"),
    (0x003c_a360, snd_dec_eaxa_free_loop, "SND_DecEaxa_FreeLoop"),
    (0x003c_a388, snd_dec_eaxa_init_loop, "SND_DecEaxa_InitLoop"),
    (0x003c_a7a0, snd_dsp_clip16_init, "SND_DspClip16_Init"),
    (0x003c_bb78, snd_ee_mix_scaled_vu0, "SND_EeMixScaledVu0"),
    (0x003c_c848, snd_ee_fx_disable, "SND_EeFxDisable"),
    (0x003c_c918, snd_dec_set_alloc_func, "SND_DecSetAllocFunc"),
    (0x003c_c928, snd_dec_set_free_func, "SND_DecSetFreeFunc"),
    (0x003c_d138, snd_eaxa_get_history, "SND_EaxaGetHistory"),
];

const OC696_UTK_ALLOC: Callee = Callee {
    addr: 0x003c_d1a8,
    f: ee::s14::SND_UtkAlloc,
};
const OC696_UTK_RESET: Callee = Callee {
    addr: 0x003c_ddb8,
    f: ee::s14::SND_UtkReset,
};
const OC696_UTK_GET_STATE: Callee = Callee {
    addr: 0x003c_e078,
    f: ee::s14::SND_UtkGetState,
};
const OC696_UTK_SET_STATE: Callee = Callee {
    addr: 0x003c_e0b8,
    f: ee::s14::SND_UtkSetState,
};
const OC696_UTK_SET_INPUT: Callee = Callee {
    addr: 0x003c_dde8,
    f: ee::s14::SND_UtkSetInput,
};
const OC696_EAXA_ALLOC: Callee = Callee {
    addr: 0x003c_cef0,
    f: ee::s14::SND_EaxaAlloc,
};
const OC696_EAXA_FREE: Callee = Callee {
    addr: 0x003c_cf18,
    f: ee::s14::SND_EaxaFree,
};
const OC696_EAXA_RESET: Callee = Callee {
    addr: 0x003c_cf40,
    f: ee::s14::SND_EaxaReset,
};
const OC696_EAXA_SET_INPUT: Callee = Callee {
    addr: 0x003c_cf60,
    f: ee::s14::SND_EaxaSetInput,
};
const OC696_EAXA_DECODE: Callee = Callee {
    addr: 0x003c_cf90,
    f: ee::s14::SND_EaxaDecode,
};
const OC696_EAXA_SET_HISTORY: Callee = Callee {
    addr: 0x003c_d190,
    f: ee::s14::SND_EaxaSetHistory,
};
const OC696_EAXA_GET_HISTORY: Callee = Callee {
    addr: 0x003c_d138,
    f: ee::s14::SND_EaxaGetHistory,
};
const OC696_EE_FX_FREE: Callee = Callee {
    addr: 0x003c_bec8,
    f: ee::s14::SND_EeFxFree,
};

/// The EA-XA decoder's fields (`DecEaxa`, the looping variant): its ADPCM
/// decoder, the sample position, the loop start and end samples, the loop
/// data (codec version 3 and up), the codec version, the stream data and
/// the old-codec (version < 3) state: "history seeded" flag, "primed"
/// flag, loop start and end sample, the two saved history samples.
#[cfg(test)]
pub(crate) mod oc696_dl {
    pub(crate) const DECODER: u32 = 0x1c;
    pub(crate) const POS: u32 = 0x20;
    pub(crate) const LOOP_START: u32 = 0x24;
    pub(crate) const LOOP_END: u32 = 0x28;
    pub(crate) const LOOP_DATA: u32 = 0x2c;
    pub(crate) const VERSION: u32 = 0x30;
    pub(crate) const DATA: u32 = 0x34;
    pub(crate) const HISTORY_SEEDED: u32 = 0x3c;
    pub(crate) const PRIMED: u32 = 0x40;
    pub(crate) const START_SAMPLE: u32 = 0x44;
    pub(crate) const END_SAMPLE: u32 = 0x48;
    pub(crate) const HISTORY0: u32 = 0x4c;
    pub(crate) const HISTORY1: u32 = 0x50;
}

/// `void SND_RegisterUtkDecoders(void)` (0x3c92f8): fill in the UTK codec's
/// entries of the decoder table at 0x515b40: the function pointers
/// 0x3c9420 (`SND_DecUtk_InitMem`), 0x3c9960 and 0x3c9618
/// (`SND_DecUtk_InitLoop`) with the object sizes 0x2c, 0x3c and 0x30, each
/// stored twice (+0x30.. and +0x78..). A leaf: only stores.
pub fn snd_register_utk_decoders(c: &mut Cpu) {
    c.cycles += 24;
    c.g[V0][0] = 0x510000;
    c.g[V1][0] = 0x3d0000;
    c.g[A0][0] = 0x3d0000;
    c.g[A1][0] = 0x3d0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b40));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6be0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x66a0));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x69e8));
    c.g[A2][0] = 0x2c;
    c.g[A3][0] = 0x3c;
    c.g[T0][0] = 0x30;
    let table = c.g[V0][0] as u32;
    c.sw(table.wrapping_add(0x3c), c.g[V1][0]);
    c.sw(table.wrapping_add(0x88), c.g[T0][0]);
    c.sw(table.wrapping_add(0x84), c.g[A2][0]);
    c.sw(table.wrapping_add(0x44), c.g[A0][0]);
    c.sw(table.wrapping_add(0x8c), c.g[A3][0]);
    c.sw(table.wrapping_add(0x40), c.g[A1][0]);
    c.sw(table.wrapping_add(0x30), c.g[V1][0]);
    c.sw(table.wrapping_add(0x78), c.g[A2][0]);
    c.sw(table.wrapping_add(0x38), c.g[A0][0]);
    c.sw(table.wrapping_add(0x80), c.g[A3][0]);
    c.sw(table.wrapping_add(0x34), c.g[A1][0]);
    c.sw(table.wrapping_add(0x7c), c.g[T0][0]);
    ret(c);
}

/// `void SND_DecUtk_InitLoop(DecUtk* self, DecUtkCfg* cfg)` (0x3c9618): set
/// up a looping UTK (MicroTalk) decoder: its callbacks (decode 0x3c9520,
/// free 0x3c95f0), the loop start (cfg+0x10) and end (cfg+0x14) samples and
/// the stream data (cfg+4); allocate (`SND_UtkAlloc(0xd68)`) and reset the
/// UTK state, mark it running (`SND_UtkGetState`, +8 = 1,
/// `SND_UtkSetState`), then queue the input (`SND_UtkSetInput(state,
/// cfg->input, count * 4, count)` with `count` = cfg+0x10). The cfg's
/// callback slot (+0x28) gets 0x3c9518.
pub fn snd_dec_utk_init_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 53;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x3d0000;
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0x3d0000;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6ae0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6a10));
        let this = c.g[S0][0] as u32;
        c.sw(this, c.g[V0][0]);
        c.sw(this.wrapping_add(0x4), c.g[V1][0]);
        c.g[S1][0] = c.g[A1][0];
        let cfg = c.g[S1][0] as u32;
        c.g[A0][0] = 0xd68;
        c.g[V1][0] = c.lw(cfg.wrapping_add(0x4));
        c.sw(this.wrapping_add(0x20), 0);
        c.sw(this.wrapping_add(0x2c), c.g[V1][0]);
        c.g[V0][0] = c.lw(cfg.wrapping_add(0x10));
        c.sw(this.wrapping_add(0x24), c.g[V0][0]);
        c.g[V1][0] = c.lw(cfg.wrapping_add(0x14));
        c.sw(this.wrapping_add(0x28), c.g[V1][0]);
        OC696_UTK_ALLOC.call(c, 0x003c_966c)?;
        c.g[A0][0] = c.g[V0][0];
        OC696_UTK_RESET.call(c, 0x003c_9674)?;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = c.g[V0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x1c), c.g[V0][0]);
        OC696_UTK_GET_STATE.call(c, 0x003c_9684)?;
        // state = GetState(utk); state.running = 1; SetState(utk, &state, 0).
        let this = c.g[S0][0] as u32;
        let s = sp(c);
        c.g[A0][0] = c.lw(this.wrapping_add(0x1c));
        c.g[V0][0] = 0x1;
        c.ldl(3, s.wrapping_add(0x17));
        c.ldr(3, s.wrapping_add(0x10));
        c.g[A1][0] = c.lw(s.wrapping_add(0x18));
        c.sdl(3, s.wrapping_add(0x7));
        c.sdr(3, s);
        c.sw(s.wrapping_add(0x8), c.g[A1][0]);
        c.g[A1][0] = c.g[SP][0];
        c.sw(s.wrapping_add(0x8), c.g[V0][0]);
        c.g[A2][0] = 0;
        OC696_UTK_SET_STATE.call(c, 0x003c_96b4)?;
        let this = c.g[S0][0] as u32;
        let cfg = c.g[S1][0] as u32;
        c.g[A2][0] = c.lw(this.wrapping_add(0x24));
        c.g[A0][0] = c.lw(this.wrapping_add(0x1c));
        c.g[A1][0] = c.lw(cfg);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        OC696_UTK_SET_INPUT.call(c, 0x003c_96cc)?;
        c.g[V1][0] = 0x3d0000;
        frame.restore(c, S0, 0x40);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ae8));
        frame.restore_ra(c, 0x20);
        let cfg = c.g[S1][0] as u32;
        c.sw(cfg.wrapping_add(0x28), c.g[V1][0]);
        frame.restore(c, S1, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DecEaxa_FreeLoop(DecEaxa* d)` (0x3ca360): free the ADPCM
/// decoder (`SND_EaxaFree(d->decoder)`) if there is one.
pub fn snd_dec_eaxa_free_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            OC696_EAXA_FREE.call(c, 0x003c_a37c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DecEaxa_InitLoop(DecEaxa* self, DecEaxaCfg* cfg)` (0x3ca388):
/// set up a looping EA-XA decoder: its callbacks (decode 0x3ca0b8, free
/// 0x3ca360), the stream (cfg+4), the loop positions (cfg+0x10/+0x14) and
/// version (cfg+0x24); allocate (`SND_EaxaAlloc(0xa8)`) and reset the
/// ADPCM decoder; the cfg's callback slot (+0x28) gets 0x3ca358.
/// Version 3 and up queue the loop data (`SND_EaxaSetInput`); older codecs
/// keep the data pointer, size and sample range for the decode loop's
/// priming (+0x34..+0x48).
pub fn snd_dec_eaxa_init_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x3d0000;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x3d0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5f48));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ca0));
        let this = c.g[S0][0] as u32;
        c.sw(this, c.g[V0][0]);
        c.sw(this.wrapping_add(0x4), c.g[V1][0]);
        c.g[S1][0] = c.g[A1][0];
        let cfg = c.g[S1][0] as u32;
        c.g[A0][0] = 0xa8;
        c.g[V1][0] = c.lw(cfg.wrapping_add(0x4));
        c.sw(this.wrapping_add(0x20), 0);
        c.sw(this.wrapping_add(0x2c), c.g[V1][0]);
        c.g[V0][0] = c.lw(cfg.wrapping_add(0x24));
        c.sw(this.wrapping_add(0x30), c.g[V0][0]);
        c.g[V1][0] = c.lw(cfg.wrapping_add(0x10));
        c.sw(this.wrapping_add(0x24), c.g[V1][0]);
        c.g[V0][0] = c.lw(cfg.wrapping_add(0x14));
        c.sw(this.wrapping_add(0x28), c.g[V0][0]);
        OC696_EAXA_ALLOC.call(c, 0x003c_a3e4)?;
        c.g[A0][0] = c.g[V0][0];
        OC696_EAXA_RESET.call(c, 0x003c_a3ec)?;
        let this = c.g[S0][0] as u32;
        let cfg = c.g[S1][0] as u32;
        c.g[V1][0] = 0x3d0000;
        c.sw(this.wrapping_add(0x1c), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ca8));
        c.sw(cfg.wrapping_add(0x28), c.g[V1][0]);
        c.g[V0][0] = c.lw(this.wrapping_add(0x30));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
        if c.g[V0][0] == 0 {
            // Version 3 and up: queue the loop data.
            c.g[A2][0] = c.lw(this.wrapping_add(0x24));
            c.cycles += 5;
            c.g[A0][0] = c.lw(this.wrapping_add(0x1c));
            c.g[A3][0] = c.g[A2][0];
            c.g[A1][0] = c.lw(cfg);
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            OC696_EAXA_SET_INPUT.call(c, 0x003c_a44c)?;
        } else {
            // Older codecs: keep the stream's data and sample range.
            c.cycles += 11;
            c.g[V0][0] = c.lw(cfg);
            c.sw(this.wrapping_add(0x34), c.g[V0][0]);
            c.g[V1][0] = c.lw(cfg.wrapping_add(0xc));
            c.sw(this.wrapping_add(0x3c), 0);
            c.sw(this.wrapping_add(0x38), c.g[V1][0]);
            c.sw(this.wrapping_add(0x40), 0);
            c.g[V0][0] = c.lw(cfg.wrapping_add(0x10));
            c.sw(this.wrapping_add(0x44), c.g[V0][0]);
            c.g[V1][0] = c.lw(cfg.wrapping_add(0x14));
            c.sw(this.wrapping_add(0x48), c.g[V1][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspClip16_Init(DspClip16* self)` (0x3ca7a0): the 16-bit clip
/// DSP stage: callback slot 0 = `SND_DspClip16_Process` (0x3ca6a8), state
/// word 1 = 0.
pub fn snd_dsp_clip16_init(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x3d0000;
    let this = c.g[A0][0] as u32;
    c.sw(this.wrapping_add(0x4), 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5958));
    c.sw(this, c.g[V0][0]);
    ret(c);
}

/// `void SND_EeMixScaledVu0(int words, float* src, float* dst)` (0x3cbb78):
/// `dst[i] += src[i] * scale` over `words` floats, four quadwords (64 bytes)
/// per pass on VU0 (`scale` is the float argument f12, broadcast from vf1;
/// `words` must be a multiple of 16). Event check at the loop's back edge.
pub fn snd_ee_mix_scaled_vu0(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = c.g[A2][0];
    c.mfc1(8, 12);
    c.g[A0][0] = c.g[V1][0];
    c.qmtc2(8, 1);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[A2][0] = c.g[A1][0];
    c.g[A3][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    'l_3cbb9c: loop {
        c.cycles += 23;
        c.lqc2(2, (c.g[A2][0] as u32));
        c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.lqc2(5, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.lqc2(6, (c.g[A3][0] as u32));
        c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x10));
        c.lqc2(8, (c.g[A3][0] as u32).wrapping_add(0x20));
        c.lqc2(9, (c.g[A3][0] as u32).wrapping_add(0x30));
        c.vu0().mul(0b1111, 2, 2, Src::Bc(1, X));
        c.vu0().mul(0b1111, 3, 3, Src::Bc(1, X));
        c.vu0().mul(0b1111, 4, 4, Src::Bc(1, X));
        c.vu0().mul(0b1111, 5, 5, Src::Bc(1, X));
        c.vu0().add(0b1111, 6, 6, Src::Vec(2));
        c.vu0().add(0b1111, 7, 7, Src::Vec(3));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
        c.vu0().add(0b1111, 8, 8, Src::Vec(4));
        c.vu0().add(0b1111, 9, 9, Src::Vec(5));
        c.sqc2(6, (c.g[A3][0] as u32));
        c.sqc2(7, (c.g[A3][0] as u32).wrapping_add(0x10));
        c.sqc2(8, (c.g[A3][0] as u32).wrapping_add(0x20));
        c.sqc2(9, (c.g[A3][0] as u32).wrapping_add(0x30));
        let t = c.g[A0][0] != c.g[A2][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x003cbb9c);
            }
            continue 'l_3cbb9c;
        } else {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.pc = t;
            return;
        }
    }
}

/// `void SND_EeFxDisable(void)` (0x3cc848): clear the reverb-engine's
/// enable word (+0x10 of the mixer at 0x515b40), free the effects
/// (`SND_EeFxFree`) and clear the byte at +0xf.
pub fn snd_ee_fx_disable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = 0x510000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b40));
        let mixer = c.g[S0][0] as u32;
        c.sw(mixer.wrapping_add(0x10), 0);
        OC696_EE_FX_FREE.call(c, 0x003c_c864)?;
        let mixer = c.g[S0][0] as u32;
        c.sb(mixer.wrapping_add(0xf), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DecSetAllocFunc(void* (*alloc)(int))` (0x3cc918): the decoder
/// library's allocator hook (0x517604).
pub fn snd_dec_set_alloc_func(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x510000;
    let hook = (c.g[V0][0] as u32).wrapping_add(0x7604);
    c.sw(hook, c.g[A0][0]);
    ret(c);
}

/// `void SND_DecSetFreeFunc(void (*free)(void*))` (0x3cc928): the decoder
/// library's free hook (0x517608).
pub fn snd_dec_set_free_func(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x510000;
    let hook = (c.g[V0][0] as u32).wrapping_add(0x7608);
    c.sw(hook, c.g[A0][0]);
    ret(c);
}

/// `EaxaHistory SND_EaxaGetHistory(EaxaDecoder* dec)` (0x3cd138): the two
/// history samples (floats at +0x98 and +0x9c) of the ADPCM decoder, returned
/// by value in v0: the sample at +0x9c in the high word, +0x98 in the low
/// word. Goes through a 0x20-byte stack frame with unaligned doubleword
/// moves (`ldl/ldr/sdl/sdr`); the scratch `v1` is merged in as the original
/// does (its low word is masked away when the pair is assembled).
pub fn snd_eaxa_get_history(c: &mut Cpu) {
    c.cycles += 22;
    let dec = c.g[A0][0] as u32;
    c.lwc1(0, dec.wrapping_add(0x9c));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.lwc1(1, dec.wrapping_add(0x98));
    let s = sp(c);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] <<= 32;
    c.g[V1][0] &= c.g[V0][0];
    c.swc1(0, s.wrapping_add(0x4));
    c.swc1(1, s);
    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] >>= 32;
    c.ldl(2, s.wrapping_add(0x7));
    c.ldr(2, s);
    c.sdl(2, s.wrapping_add(0x17));
    c.sdr(2, s.wrapping_add(0x10));
    c.g[A0][0] = c.lwu(s.wrapping_add(0x10));
    c.g[V0][0] = c.lwu(s.wrapping_add(0x14));
    c.g[V1][0] |= c.g[A0][0];
    c.g[V1][0] &= c.g[A1][0];
    c.g[V0][0] <<= 32;
    c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// The decoder library's allocator and free hooks.
    const OC696_ALLOC_HOOK: u32 = 0x0051_7604;
    const OC696_FREE_HOOK: u32 = 0x0051_7608;

    /// `SND_RegisterUtkDecoders`: a leaf of stores into the decoder table.
    #[test]
    fn snd_register_utk_decoders_matches() {
        check(
            ee::s14::f_003c92f8__generated,
            snd_register_utk_decoders,
            8,
            &|c, seed| {
                let mut r = Rng(0x003c_92f8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_92f8;
                fill(c, &mut r, 0x0051_5b40, 0x40);
            },
        );
    }

    /// `SND_DspClip16_Init` on an object of random words.
    #[test]
    fn snd_dsp_clip16_init_matches() {
        check(
            ee::s14::f_003ca7a0__generated,
            snd_dsp_clip16_init,
            8,
            &|c, seed| {
                let mut r = Rng(0x003c_a7a0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_a7a0;
                fill(c, &mut r, DATA + 0x100, 4);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    /// `SND_DecSetAllocFunc`: any function pointer.
    #[test]
    fn snd_dec_set_alloc_func_matches() {
        check(
            ee::s14::f_003cc918__generated,
            snd_dec_set_alloc_func,
            8,
            &|c, seed| {
                let mut r = Rng(0x003c_c918 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_c918;
                c.g[A0][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    /// `SND_DecSetFreeFunc`: any function pointer.
    #[test]
    fn snd_dec_set_free_func_matches() {
        check(
            ee::s14::f_003cc928__generated,
            snd_dec_set_free_func,
            8,
            &|c, seed| {
                let mut r = Rng(0x003c_c928 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_c928;
                c.g[A0][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    /// `SND_EaxaGetHistory`: random history floats (edge values included)
    /// and a random `v1` (the scratch the result is merged with).
    #[test]
    fn snd_eaxa_get_history_matches() {
        check(
            ee::s14::f_003cd138__generated,
            snd_eaxa_get_history,
            64,
            &|c, seed| {
                let mut r = Rng(0x003c_d138 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_d138;
                let dec = DATA + 0x200;
                fill(c, &mut r, dec, 0x30);
                let a = r.edge_f32();
                let b = r.edge_f32();
                c.bus.write32(dec + 0x98, a);
                c.bus.write32(dec + 0x9c, b);
                c.g[A0][0] = sx(dec);
            },
        );
    }

    /// `SND_DecEaxa_FreeLoop`: with and without a decoder; the library's free
    /// hook (0x517608, called by the real `SND_EaxaFree`) is a fake, which
    /// unwinds for some seeds.
    #[test]
    fn snd_dec_eaxa_free_loop_matches() {
        covered(
            ee::s14::f_003ca360__generated,
            snd_dec_eaxa_free_loop,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_a360, 1);
                c.pc = 0x003c_a360;
                let dec = DATA + 0x100;
                fill(c, &mut r, dec, 0x10);
                let decoder = if seed % 3 == 0 {
                    0
                } else {
                    DATA + 0x2000 + 0x40 * (seed % 8)
                };
                c.bus.write32(dec + 0x1c, decoder);
                c.bus.write32(OC696_FREE_HOOK, FAKE + 0x10 * (seed % 4));
                c.g[A0][0] = sx(dec);
            },
        );
    }

    /// `SND_DecUtk_InitLoop`: the allocation hook (0x517604) is a fake that
    /// returns the test's UTK state block (or unwinds); the real
    /// `SND_UtkReset` / `GetState` / `SetState` / `SetInput` run, on inputs
    /// with and without the 0xee marker byte.
    #[test]
    fn snd_dec_utk_init_loop_matches() {
        const THIS: u32 = DATA + 0x100;
        const CFG: u32 = DATA + 0x200;
        const INPUT: u32 = DATA + 0x400;
        const UTK: u32 = DATA + 0x2000;
        covered(
            ee::s14::f_003c9618__generated,
            snd_dec_utk_init_loop,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_9618, 1);
                c.pc = 0x003c_9618;
                c.bus.write32(OC696_ALLOC_HOOK, FAKE);
                c.bus.write32(LOG_RESULTS, UTK);
                fill(c, &mut r, THIS, 0x10);
                fill(c, &mut r, CFG, 0x10);
                fill(c, &mut r, UTK + 0xd40, 0x0a);
                c.bus.write32(UTK + 4, r.next_u32());
                c.bus.write32(CFG, INPUT);
                c.bus.write32(CFG + 0x10, r.next_u32() % 0x400);
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

    /// `SND_DecEaxa_InitLoop`: the allocator hook (0x517604) is a fake
    /// returning a block (unwinding for some seeds), `SND_EaxaReset` and
    /// `SND_EaxaSetInput` are real; codec versions below and from 3, with and
    /// without a data pointer.
    #[test]
    fn snd_dec_eaxa_init_loop_matches() {
        covered(
            ee::s14::f_003ca388__generated,
            snd_dec_eaxa_init_loop,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_a388, 1);
                c.pc = 0x003c_a388;
                c.bus.write32(OC696_ALLOC_HOOK, FAKE);
                c.bus.write32(LOG_RESULTS, DATA + 0x3_0000);
                let this = DATA + 0x100;
                let cfg = DATA + 0x200;
                fill(c, &mut r, this, 0x14);
                fill(c, &mut r, cfg, 0x14);
                fill(c, &mut r, DATA + 0x3_0000, 0x30);
                c.bus
                    .write32(cfg, if seed % 5 == 0 { 0 } else { DATA + 0x400 });
                c.bus.write32(cfg + 0x10, r.next_u32() % 0x400);
                let version = if seed % 2 == 0 {
                    r.next_u32() % 3
                } else {
                    3 + r.next_u32() % 3
                };
                c.bus.write32(cfg + 0x24, version);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(cfg);
            },
        );
    }

    /// Gate for the `SND_MemFree` fake (the suite runs tests in parallel):
    /// it logs only while this word of the machine's RAM holds the magic.
    const OC696_CTRL: u32 = DATA + 0x20_0000;
    const OC696_MAGIC: u32 = 0x0c69_6a01;
    static OC696_PREV_MEM_FREE: AtomicUsize = AtomicUsize::new(0);

    fn oc696_fake_mem_free(c: &mut Cpu) {
        if c.bus.read32(OC696_CTRL) != OC696_MAGIC {
            return fall_back(c, &OC696_PREV_MEM_FREE, ee::s13::f_003b5e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x003b_5e98, ra);
    }

    /// The effects-engine fixture: the enable flag (0x44e898), the table of
    /// effect objects (0x515d44, sized by a big-endian word read through
    /// `SND_ReadBigEndian`), the slot array (0x515d48), and the scalars the
    /// teardown clears. Objects carry a stop callback (a fake) or none.
    fn oc696_fx_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        const BASE: u32 = DATA + 0x1000;
        const TABLE: u32 = DATA + 0x2000;
        const SLOTS: u32 = DATA + 0x4000;
        let mut r = machine(c, seed, entry as u64, 8);
        c.pc = entry;
        c.bus.write32(OC696_CTRL, OC696_MAGIC);
        c.bus.write32(0x0044_e898, (seed % 5 != 0) as u32);
        let words = 1 + r.next_u32() % 3;
        c.bus.write32(0x0051_5d54, words);
        c.bus.write32(0x0051_5d40, BASE);
        let n = match seed % 7 {
            0 => 0,
            1 => 0u32.wrapping_sub(1 + r.next_u32() % 3),
            _ => 1 + r.next_u32() % 3,
        };
        let size_at = BASE + 4 * words + 4;
        for (i, b) in n.to_be_bytes().iter().enumerate() {
            c.bus.write8(size_at + i as u32, *b);
        }
        // The table pointer: always set when objects are listed, sometimes
        // absent otherwise (so the table free is skipped).
        let table = if (n as i32) > 0 || seed % 3 != 0 {
            TABLE
        } else {
            0
        };
        c.bus.write32(0x0051_5d44, table);
        for i in 0..(n as i32).max(0) as u32 {
            let obj = DATA + 0x3000 + 0x40 * i;
            c.bus.write32(TABLE + 4 * i, obj);
            let callback = if r.next_u32() % 2 == 0 {
                0
            } else {
                FAKE + 0x10 * i
            };
            c.bus.write32(obj + 4, callback);
        }
        c.bus
            .write32(0x0051_5d48, if seed % 3 == 0 { 0 } else { SLOTS });
        for i in 0..6 {
            let v = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(SLOTS + 4 * i, v);
        }
        for a in [0x0051_75f8u32, 0x0051_75fc, 0x0051_5d50, 0x0051_5d4c] {
            c.bus.write32(a, r.next_u32());
        }
        r
    }

    fn oc696_fx_install() {
        hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B5E98),
            oc696_fake_mem_free,
            &OC696_PREV_MEM_FREE,
        );
    }

    driven!(oc696_gen_fx_free, ee::s14::f_003cbec8);

    /// `SND_EeFxFree`: engine off (nothing to do), no / negative / 1..3
    /// effect objects with and without stop callbacks, null table and slot
    /// array; `SND_ReadBigEndian` and `memset` are real, `SND_MemFree` and
    /// the callbacks are logging fakes that unwind for some seeds; event
    /// seeds deliver (or unwind at) the loops' back edges.

    /// `SND_EeFxDisable` around the same fixture.
    #[test]
    fn snd_ee_fx_disable_matches() {
        oc696_fx_install();
        covered(
            ee::s14::f_003cc848__generated,
            snd_ee_fx_disable,
            120,
            &|c, seed| {
                let mut r = oc696_fx_setup(c, seed, 0x003c_c848);
                fill(c, &mut r, 0x0051_5b40, 0x8);
            },
        );
    }

    driven!(oc696_gen_mix_scaled, ee::s14::f_003cbb78__generated);
    driven!(oc696_hand_mix_scaled, snd_ee_mix_scaled_vu0);

    /// `SND_EeMixScaledVu0`: 1..3 passes (16, 32 or 48 words) over random
    /// floats (edge values included) with a random scale; event seeds deliver
    /// (or unwind at) the loop's back edge.
    #[test]
    fn snd_ee_mix_scaled_vu0_matches() {
        const SRC: u32 = DATA + 0x1000;
        const DST: u32 = DATA + 0x2000;
        let setup = |c: &mut Cpu, seed: u32| -> Rng {
            let mut r = Rng(0x003c_bb78 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003c_bb78;
            for i in 0..0x30 {
                let a = r.edge_f32();
                let b = r.edge_f32();
                c.bus.write32(SRC + 4 * i, a);
                c.bus.write32(DST + 4 * i, b);
            }
            c.f[12] = r.edge_f32();
            c.g[A0][0] = 16 * (1 + seed as u64 % 3);
            c.g[A1][0] = sx(SRC);
            c.g[A2][0] = sx(DST);
            r
        };
        check(
            ee::s14::f_003cbb78__generated,
            snd_ee_mix_scaled_vu0,
            48,
            &|c, seed| {
                setup(c, seed);
            },
        );
        check_events(
            oc696_gen_mix_scaled,
            oc696_hand_mix_scaled,
            90,
            &|c, seed| {
                let mut r = setup(c, seed);
                schedule(c, &mut r, seed, 0x003c_bb78)
            },
        );
    }

    /// `SND_DecEaxa_DecodeLoop` fixture, with the real EA-XA decoder
    /// (coefficient table from the ELF): both codec generations, primed or
    /// not, loops of 1..100 samples, requests of 0 (nothing), negative and
    /// 1..150 samples so the decoder runs dry once or several times.
    fn oc696_decode_loop_setup(c: &mut Cpu, seed: u32) -> Rng {
        const SELF: u32 = DATA;
        const DECODER: u32 = DATA + 0x200;
        const INPUT: u32 = DATA + 0x2000;
        const OUT: u32 = DATA + 0x8000;
        use super::oc696_dl as dl;
        let mut r = Rng(0x003c_a0b8 ^ (seed as u64) << 8);
        elf_image(c);
        scramble(c, &mut r);
        c.pc = 0x003c_a0b8;
        fill(c, &mut r, INPUT, 0x400);
        for k in 0..0x30 {
            c.bus.write32(DECODER + 4 * k, 0);
        }
        let h0 = r.f32(1.0);
        let h1 = r.f32(1.0);
        c.bus.write32(DECODER + 0x98, h0);
        c.bus.write32(DECODER + 0x9c, h1);
        let modern = seed % 2 == 0;
        let len = 1 + r.next_u32() % 100;
        let start = r.next_u32() % 100;
        c.bus.write32(SELF + dl::DECODER, DECODER);
        c.bus.write32(SELF + dl::POS, r.next_u32() % 1000);
        c.bus.write32(SELF + dl::LOOP_START, start);
        c.bus.write32(SELF + dl::LOOP_END, start + len - 1);
        c.bus
            .write32(SELF + dl::LOOP_DATA, INPUT + 0x10 * (r.next_u32() % 16));
        let version = if modern {
            3 + r.next_u32() % 3
        } else {
            r.next_u32() % 3
        };
        c.bus.write32(SELF + dl::VERSION, version);
        c.bus.write32(SELF + dl::DATA, INPUT + 0x200);
        c.bus.write32(SELF + dl::HISTORY_SEEDED, r.next_u32() % 2);
        c.bus.write32(SELF + dl::PRIMED, (seed / 2) % 2);
        let s = r.next_u32() % 150;
        c.bus.write32(SELF + dl::START_SAMPLE, s);
        let e = s + 1 + r.next_u32() % 100;
        c.bus.write32(SELF + dl::END_SAMPLE, e);
        let a = r.f32(1.0);
        let b = r.f32(1.0);
        c.bus.write32(SELF + dl::HISTORY0, a);
        c.bus.write32(SELF + dl::HISTORY1, b);
        let n: i32 = match seed % 7 {
            0 => 0,
            1 => -((r.next_u32() % 3) as i32),
            _ => 1 + (r.next_u32() % 150) as i32,
        };
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(n as u32);
        c.g[A3][0] = sx(OUT);
        r
    }

    driven!(oc696_gen_decode_loop, ee::s14::f_003ca0b8);
}
