//! Staging module oc330 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc330`.

#![allow(unused_imports)]

use crate::abi;
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
    (
        0x002a_1820,
        ssx_audio_system_get_rider_rivalry,
        "SSXAudioSystem_GetRiderRivalry",
    ),
    (0x0029_0c10, audio_eval_curve_table, "Audio_EvalCurveTable"),
    (
        0x002a_0e70,
        ssx_audio_system_rider_say_2077,
        "SSXAudioSystem_RiderSay2077",
    ),
    (
        0x003c_8b40,
        snd_ee_voice_set_time_stretch,
        "SND_EeVoiceSetTimeStretch",
    ),
    (
        0x0036_12c0,
        intersect_info_reset_matrix,
        "IntersectInfo_ResetMatrix",
    ),
    (
        0x002b_0e28,
        speech_system_is_instance_ready,
        "SpeechSystem_IsInstanceReady",
    ),
    (
        0x0030_a5c0,
        wscript_man_run_kill_script,
        "WScriptMan_RunKillScript",
    ),
    (0x003c_d1d0, snd_utk_free, "SND_UtkFree"),
    (
        0x0029_f000,
        ssx_audio_system_get_speech_category_volume,
        "SSXAudioSystem_GetSpeechCategoryVolume",
    ),
    (
        0x003c_8598,
        snd_ee_voice_set_aux_send,
        "SND_EeVoiceSetAuxSend",
    ),
];

// ---------------------------------------------------------------------------
// Audio_EvalCurveTable (0x290c10)

/// The five 5-point curves `Audio_EvalCurveTable` indexes (`.rodata`):
/// `{ float t[5]; float v[5]; }`, 0x28 bytes each.
pub(crate) const AECT_TABLE: u32 = 0x0044_5898;

pub(crate) const AECT_CURVE_SIZE: u32 = 0x28;

/// `float Audio_EvalCurveTable(int i, float x)` (0x290c10): evaluate control
/// curve `i` (0x28 bytes each, `&D_00445898[i]`) at `x` through
/// `Audio_EvalCurve5`, in f12.
pub fn audio_eval_curve_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V1][0] = 0x28;
        c.g[V0][0] = 0x44_0000;
        c.mult(0, A0, A0, V1); // a0 = i * 0x28 (also LO/HI)
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x44_0000u32.wrapping_add(0x5898));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Callee {
            addr: 0x0029_0b58,
            f: ee::s07::Audio_EvalCurve5,
        }
        .call(c, 0x0029_0c30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetAuxSend (0x3c8598)

/// The voice table the sound library points at (`D_00515B40 + 0x1dc`).
pub(crate) const EVAS_VOICES_PTR: u32 = 0x0051_5d1c;

/// Per-voice stride, the aux-send float and the "send set" flag byte.
pub(crate) const EVAS_VOICE_SIZE: u32 = 0x60;

pub(crate) const EVAS_AUX_SEND: u32 = 0x38;

/// `void SND_EeVoiceSetAuxSend(int voice, int send, float level)` (0x3c8598):
/// `voices[voice].aux_send[send] = level` and mark the voice's sends dirty.
/// f12 carries `level`.
pub fn snd_ee_voice_set_aux_send(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = EVAS_VOICE_SIZE as u64;
    c.g[A2][0] = 0x51_0000;
    c.mult(0, A0, A0, V0); // a0 = voice * 0x60
    let base = 0x51_0000u32.wrapping_add(0x5b40);
    c.g[A2][0] = sx(base);
    c.g[V0][0] = Addr(base.wrapping_add(0x1dc)).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A3][0] = 0x1;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(EVAS_AUX_SEND));
    c.g[V1][0] = Addr(base.wrapping_add(0x1dc)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let ra = c.g[RA][0] as u32;
    // Delay slot of the return: flag the voice's sends as set.
    c.sb((c.g[A0][0] as u32).wrapping_add(1), c.g[A3][0]);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// IntersectInfo_ResetMatrix (0x3612c0)

/// The identity matrix copied into every `IntersectInfo` (`D_004FF1A0`).
pub(crate) const IIRM_IDENTITY: u32 = 0x004f_f1a0;

/// `IntersectInfo` matrix offset.
pub(crate) const IIRM_MATRIX: u32 = 0x90;

/// `void IntersectInfo_ResetMatrix(IntersectInfo* self)` (0x3612c0): copy the
/// identity quadwords at `D_004FF1A0` into `self->matrix` (VU0 macro-mode
/// `lqc2`/`sqc2`, vf1..vf4).
pub fn intersect_info_reset_matrix(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x50_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x90));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let src = c.g[V0][0] as u32;
    let dst = c.g[A0][0] as u32;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SpeechSystem_IsInstanceReady (0x2b0e28)

/// The speech system's instance array (at `speech + 4`).
pub(crate) const SIR_INSTANCES: u32 = 0x4;

/// `bool SpeechSystem_IsInstanceReady(SpeechSystem* self, int i)`
/// (0x2b0e28): whether instance `i` of the speech system's array exists and
/// its stream is ready (`SpeechInstance_IsStreamReady`), false when the slot
/// is empty.
pub fn speech_system_is_instance_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SIR_INSTANCES)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002b_0290,
                f: ee::s08::SpeechInstance_IsStreamReady,
            }
            .call(c, 0x002b_0e50)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_RunKillScript (0x30a5c0)

/// Script-manager fields: the running "kill" script handle and the script
/// object, plus the world's script-function lookup.
pub(crate) const RKS_RUNNING: u32 = 0x290;

pub(crate) const RKS_SCRIPT: u32 = 0x28c;

pub(crate) const RKS_GET_OBJECT_SCRIPT_FUNC: u32 = 0x003a_6b78;

pub(crate) const RKS_CALL_FUNC: Callee = Callee {
    addr: 0x0030_9c88,
    f: ee::s10::WScriptMan_CallFunc,
};

/// `void WScriptMan_RunKillScript(WScriptMan* self, int id)` (0x30a5c0): look
/// `id` up as an object script function (slot 4) of the manager's script
/// object and, when found (`!= -1`), call it with the handle; the handle is
/// published in `self+0x290` for the duration.
pub fn wscript_man_run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.sw(r32(c, S0).wrapping_add(RKS_RUNNING), c.g[V0][0]);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(RKS_SCRIPT)).lw(c);
        call_ptr(c, RKS_GET_OBJECT_SCRIPT_FUNC, 0x0030_a5e4)?;
        c.g[V1][0] = !(0u64 | c.g[V0][0]);
        let not_found = c.g[V1][0] == 0;
        // Delay slot: the handle is the argument for the call.
        c.sw(sp(c), c.g[V0][0]);
        if !not_found {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            RKS_CALL_FUNC.call(c, 0x0030_a5fc)?;
        }
        c.cycles += 5;
        c.sw(r32(c, S0).wrapping_add(RKS_RUNNING), 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkFree (0x3cd1d0)

/// The sound-library allocator's free hook (`D_00517608`).
pub(crate) const UTK_FREE_HOOK: u32 = 0x0051_7608;

/// `void SND_UtkFree(void* p)` (0x3cd1d0): hand `a0` to the library's free
/// hook (a function pointer in `D_00517608`).
pub fn snd_utk_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x51_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(0x51_0000u32.wrapping_add(0x7608)).lw(c);
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003c_d1e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetSpeechCategoryVolume (0x29f000)

/// Speech category types handled specially by
/// [`ssx_audio_system_get_speech_category_volume`]: 2 (the "fade to default"
/// curve) and 0xB (the category gain table at `self + 0x5730`).
pub(crate) const SCV_FADE_TYPE: u64 = 2;

pub(crate) const SCV_TABLE_TYPE: u64 = 0xb;

/// `float SSXAudioSystem_GetSpeechCategoryVolume(SSXAudioSystem* self,
/// int voice, int type, int channel)` (0x29f000): a non-zero `voice` means
/// "no speech category" (0). Type 2 sets the fade mode from the two constants
/// and then looks the channel gain up (`SSXAudioSystem_GetChannelGainPtr`),
/// type 0xB returns the category table (`self + 0x5730`), and every other
/// type goes straight to the channel gain lookup.
pub fn ssx_audio_system_get_speech_category_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0]; // type
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // self
        frame.save_ra(c, 0);
        let voice_set = c.g[A1][0] != 0;
        c.g[S2][0] = c.g[A3][0]; // channel
        if voice_set {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[V0][0] = SCV_FADE_TYPE;
            let not_fade = c.g[S0][0] != c.g[V0][0];
            c.g[V0][0] = SCV_TABLE_TYPE;
            if !not_fade {
                c.cycles += 5;
                c.lwc1(12, r32(c, GP).wrapping_sub(0x457c));
                c.lwc1(13, r32(c, GP).wrapping_sub(0x4578));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0028_7f00, 0x0029_f040)?;
                c.g[V0][0] = SCV_TABLE_TYPE;
            }
            c.cycles += 2;
            let not_table = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if not_table {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0028_7968, 0x0029_f060)?;
                c.lq(16, frame.at(c, 0x30));
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5730));
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x30));
            }
        }
        c.cycles += 5;
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetTimeStretch (0x3c8b40)

/// The voice table [`snd_ee_voice_set_time_stretch`] indexes, and the
/// per-voice stretch value at `+0x4c`.
pub(crate) const EVTS_VOICES_PTR: u32 = 0x0051_5d1c;

pub(crate) const EVTS_VOICE_SIZE: u32 = 0x60;

pub(crate) const EVTS_STRETCH: u32 = 0x4c;

/// `void SND_EeVoiceSetTimeStretch(int voice, float ratio)` (0x3c8b40): set
/// the voice's time-stretch ratio (`voices[voice] + 0x4c`) through
/// `SND_DspStretch_SetRatio` when it is set. f12 carries `ratio`.
pub fn snd_ee_voice_set_time_stretch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V1][0] = EVTS_VOICE_SIZE as u64;
        c.g[A2][0] = 0x51_0000;
        c.mult(0, A0, A0, V1); // a0 = voice * 0x60
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(0x51_0000u32.wrapping_add(0x5d1c)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(EVTS_STRETCH)).lw(c);
        let unset = c.g[A0][0] == 0;
        // Delay slot: restore ra before the branch.
        frame.restore_ra(c, 0);
        if !unset {
            c.cycles += 3;
            Callee {
                addr: 0x003c_b110,
                f: ee::s14::SND_DspStretch_SetRatio,
            }
            .call(c, 0x003c_8b70)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetRiderRivalry (0x2a1820)

/// Rider fields: "is human" and "is local".
pub(crate) const GRR_HUMAN: u32 = 0x874;

pub(crate) const GRR_LOCAL: u32 = 0x87c;

/// The rider's embedded scripting object (its vtable) and the `v07` slot.
pub(crate) const GRR_OBJECT: u32 = 0x6c0;

pub(crate) const GRR_V07: u32 = 0x38;

/// Audio system word 0x78: 2 while a "rivalry" track is playing, which makes
/// the rivalry 2 straight away.
pub(crate) const GRR_TRACK_MODE: u32 = 0x78;

/// `int SSXAudioSystem_GetRiderRivalry(SSXAudioSystem* self, Rider* a,
/// Rider* b)` (0x2a1820): 2 when the audio system is in track mode 2 or when
/// both riders are human and local; otherwise the BE aggression interface's
/// rival state of a against b, or 2 while either "level" is 2 or more.
pub fn ssx_audio_system_get_rider_rivalry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A1][0]; // a
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0]; // b
        call_ptr(c, 0x0028_b1d8, 0x002a_1854)?; // SSXAudioSystem_GetInstance
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(GRR_TRACK_MODE)).lw(c);
        c.g[V0][0] = 0x2;
        let track_mode = c.g[V1][0] == c.g[V0][0];
        // Delay slot: restore s0 before the branch.
        c.lq(16, frame.at(c, 0x80));
        if !track_mode {
            'exit: {
                // Is a human and local? (either flag alone is enough)
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(GRR_HUMAN)).lw(c);
                let a_not_human = c.g[V0][0] == 0;
                c.g[V1][0] = 0;
                if !a_not_human {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(GRR_LOCAL)).lw(c);
                    c.g[V1][0] = (c.g[V0][0] > 0) as u64;
                }
                c.cycles += 2;
                if c.g[V1][0] != 0 {
                    // ... and is b?
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S4).wrapping_add(GRR_HUMAN)).lw(c);
                    let b_not_human = c.g[V0][0] == 0;
                    c.g[V1][0] = 0;
                    if !b_not_human {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(r32(c, S4).wrapping_add(GRR_LOCAL)).lw(c);
                        c.g[V1][0] = (c.g[V0][0] > 0) as u64;
                    }
                    c.cycles += 2;
                    let both = c.g[V1][0] != 0;
                    c.g[V0][0] = 0x2;
                    if both {
                        break 'exit;
                    }
                }
                // The full comparison through the BE aggression interface.
                c.cycles += 41;
                c.g[S5][0] = sx(r32(c, S3).wrapping_add(GRR_OBJECT));
                call_ptr(c, 0x0015_5a50, 0x002a_18a4)?; // BEAggressionInt_GetThis
                c.g[V1][0] = Addr(r32(c, S3).wrapping_add(GRR_OBJECT)).lw(c);
                c.g[S1][0] = c.g[V0][0];
                c.g[S6][0] = sx(r32(c, S4).wrapping_add(GRR_OBJECT));
                c.g[S7][0] = 0x2;
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07 + 4)).lw(c);
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, S5).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, f, 0x002a_18c4)?; // a->v07()
                c.g[V1][0] = Addr(r32(c, S4).wrapping_add(GRR_OBJECT)).lw(c);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07 + 4)).lw(c);
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, S6).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, f, 0x002a_18dc)?; // b->v07()
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0015_5b50, 0x002a_18ec)?; // GetRivalLevel(this, a, b)
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0015_5a50, 0x002a_18f4)?; // BEAggressionInt_GetThis
                c.g[V1][0] = Addr(r32(c, S4).wrapping_add(GRR_OBJECT)).lw(c);
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07 + 4)).lw(c);
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, S6).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, f, 0x002a_190c)?; // b->v07()
                c.g[V1][0] = Addr(r32(c, S3).wrapping_add(GRR_OBJECT)).lw(c);
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07 + 4)).lw(c);
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, S5).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, f, 0x002a_1924)?; // a->v07()
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x0015_5b50, 0x002a_1934)?; // GetRivalLevel(this, b, a)
                c.g[S0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                let level_a_high = c.g[S0][0] == 0;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                if level_a_high {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S7][0];
                } else {
                    c.cycles += 2;
                    let level_b_high = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[S7][0];
                    if !level_b_high {
                        // Both levels below 2: ask the rival state.
                        c.cycles += 20;
                        call_ptr(c, 0x0015_5a50, 0x002a_1950)?; // GetThis
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V1][0] = Addr(r32(c, S3).wrapping_add(GRR_OBJECT)).lw(c);
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07)).lh(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07 + 4)).lw(c);
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(r32(c, S5).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x002a_1968)?; // a->v07()
                        c.g[V1][0] = Addr(r32(c, S4).wrapping_add(GRR_OBJECT)).lw(c);
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07)).lh(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GRR_V07 + 4)).lw(c);
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(r32(c, S6).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x002a_1980)?; // b->v07()
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x0015_5ab0, 0x002a_1990)?; // GetRivalState
                        let state_set = c.g[V0][0] != 0;
                        c.g[V1][0] = c.g[V0][0] ^ 0x1;
                        c.cycles += 2;
                        if !state_set {
                            c.g[S7][0] = 0;
                        } else {
                            c.g[V0][0] = 0x1;
                            if c.g[V1][0] == 0 {
                                c.g[S7][0] = c.g[V0][0];
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S7][0];
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x80));
        }
        c.cycles += 10;
        c.lq(17, frame.at(c, 0x70));
        c.lq(18, frame.at(c, 0x60));
        c.lq(19, frame.at(c, 0x50));
        c.lq(20, frame.at(c, 0x40));
        c.lq(21, frame.at(c, 0x30));
        c.lq(22, frame.at(c, 0x20));
        c.lq(23, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_RiderSay2077 (0x2a0e70)

/// Rider: along-track progress (speech only while negative), the two
/// "human"/"local" flags, the scripting object and the `v05`/`v09` slots.
pub(crate) const RS2077_PROGRESS: u32 = 0x470;

pub(crate) const RS2077_HUMAN: u32 = 0x874;

pub(crate) const RS2077_LOCAL: u32 = 0x87c;

pub(crate) const RS2077_OBJECT: u32 = 0x6c0;

pub(crate) const RS2077_V05: u32 = 0x28;

pub(crate) const RS2077_V09: u32 = 0x48;

/// Audio system: the speech system, the post hook (`gp + 0x173c`) and the
/// listener radius.
pub(crate) const RS2077_SPEECH: u32 = 0x5560;

pub(crate) const RS2077_POST_HOOK_GP: u32 = 0x173c;

pub(crate) const RS2077_RADIUS_GP: u32 = 0x5e8;

/// The spoken code.
pub(crate) const RS2077_CODE: u64 = 0x2077;

/// `void SSXAudioSystem_RiderSay2077(SSXAudioSystem* sys, Rider* rider,
/// int play)` (0x2a0e70): the "wipeout" line. Silent / replaying / AI-or-
/// followed-elsewhere / still on course riders get nothing; then, with the
/// speech system's instance ready, the rider's "surface" method decides
/// whether a grunt plays (a negative listener distance means nothing is
/// played) and, when `play` is set, the rider is human and local, the line is
/// posted for its character with its speech mask via the speech context and
/// the post hook.
pub fn ssx_audio_system_rider_say_2077(c: &mut Cpu) {
    let frame = Frame { size: 0x70 };
    guest(c, |c| {
        c.cycles += 14;
        Frame::open(c, 0x70);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0]; // sys
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A1][0]; // rider
        frame.save(c, S2, 0x30);
        c.g[S4][0] = c.g[A2][0]; // play
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        Callee {
            addr: 0x0029_f0a0,
            f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
        }
        .call(c, 0x002a_0ea0)?;
        'exit: {
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x50);
                break 'exit;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0028_8ae0,
                f: ee::s07::SSXAudioSystem_IsReplayPlaying,
            }
            .call(c, 0x002a_0eb0)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x50);
                break 'exit;
            }
            // The rider's v09 (behind the start / AI?) or a non-local rider
            // means "check the followed rider".
            c.cycles += 10;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(RS2077_OBJECT)).lw(c);
            c.g[A1][0] = sx(r32(c, S0).wrapping_add(RS2077_OBJECT));
            c.g[S3][0] = c.g[A1][0];
            c.g[S2][0] = 0;
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(RS2077_V09)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RS2077_V09 + 4)).lw(c);
            let f = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, f, 0x002a_0ed8)?;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = 0x1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RS2077_LOCAL)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = 0x1;
                }
            }
            // Unless the rider is AI or not local, it must be the followed one.
            c.cycles += 2;
            let check_followed = c.g[S2][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !check_followed {
                c.cycles += 4;
                c.g[A1][0] = u64::MAX;
                call_ptr(c, 0x0028_5d98, 0x002a_0efc)?; // FindRider(sys, -1)
                if c.g[V0][0] != c.g[S0][0] {
                    frame.restore(c, S0, 0x50);
                    break 'exit;
                }
            }
            // Still behind the start line?
            c.cycles += 6;
            c.lwc1(0, r32(c, S0).wrapping_add(RS2077_PROGRESS));
            c.mtc1(0, 20);
            c.c_le_s(20, 0);
            let ahead = c.fcc();
            c.g[V0][0] = 0x1;
            if !ahead {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x50);
                break 'exit;
            }
            c.cycles += 6;
            c.g[S2][0] = sx(r32(c, S1).wrapping_add(RS2077_SPEECH));
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x002b_0e28,
                f: ee::s08::SpeechSystem_IsInstanceReady,
            }
            .call(c, 0x002a_0f38)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x50);
                break 'exit;
            }
            'late: {
                c.cycles += 11;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(RS2077_OBJECT)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(RS2077_V05)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RS2077_V05 + 4)).lw(c);
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, f, 0x002a_0f54)?; // rider->v05()
                c.lwc1(12, r32(c, GP).wrapping_add(RS2077_RADIUS_GP));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0028_8b40, 0x002a_0f64)?; // FindViewInRange
                let far = (c.g[V0][0] as i64) < 0;
                c.g[A0][0] = c.g[S1][0];
                if far {
                    break 'late;
                }
                c.cycles += 6;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x2;
                c.g[A3][0] = 0;
                Callee {
                    addr: 0x0029_f660,
                    f: ee::s08::SSXAudioSystem_PlayRiderGrunt,
                }
                .call(c, 0x002a_0f7c)?;
                if c.g[S4][0] == 0 {
                    frame.restore(c, S0, 0x50);
                    break 'exit;
                }
                // Human and local only.
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RS2077_HUMAN)).lw(c);
                let not_human = c.g[V0][0] == 0;
                c.g[V1][0] = 0;
                if !not_human {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RS2077_LOCAL)).lw(c);
                    c.g[V1][0] = (c.g[V0][0] > 0) as u64;
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    frame.restore(c, S0, 0x50);
                    break 'exit;
                }
                c.cycles += 16;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0029_f330,
                    f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
                }
                .call(c, 0x002a_0fa8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0012_3128, 0x002a_0fb4)?; // Rider_GetCharacter
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[T0][0] = c.g[V0][0];
                c.mov_s(12, 20);
                c.g[A1][0] = 0;
                c.g[A2][0] = RS2077_CODE;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                Callee {
                    addr: 0x002b_1458,
                    f: ee::s08::SpeechSystem_SetEventContext,
                }
                .call(c, 0x002a_0fd8)?;
                let accepted = c.g[V0][0] != 0;
                c.g[A0][0] = 0x1;
                if accepted {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = RS2077_CODE;
                    call_ptr(c, 0x003d_8008, 0x002a_0fec)?; // SPCH_MakeId(1, 0, code)
                    c.g[V1][0] = Addr(r32(c, GP).wrapping_add(RS2077_POST_HOOK_GP)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    let hook = c.g[V1][0] as u32;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, hook, 0x002a_1000)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, put_vec, scramble, Rng, DATA, GAME_GP};
    use crate::game::sound::soundsys::tests::rs207d_world;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Magic word that turns the gated test fakes on. It lives in each test
    /// machine's own RAM (`DATA + 0xf0000`, below the fakes' call log), so the
    /// rest of the suite reaches the fakes' chained-to bodies instead.
    const OC330_CTRL: u32 = DATA + 0x0f_0000;
    const OC330_MAGIC: u32 = 0x4f43_3330;

    /// The bodies the slots held before a test fake was installed.
    static OC330_PREV: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];

    /// Replace a generated slot with `fake` (once), remembering what it held.
    fn oc330_install(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), idx: usize) {
        crate::difftest::fakes::hook(slot, fake, &OC330_PREV[idx]);
    }

    /// Run the body a slot held before our fake (if any).
    fn oc330_chain(c: &mut Cpu, idx: usize, mine: fn(&mut Cpu)) {
        let p = OC330_PREV[idx].load(Ordering::Relaxed);
        if p != 0 && p != mine as usize {
            let f: GuestFn = unsafe { std::mem::transmute(p) };
            f(c);
        }
    }

    fn oc330_active(c: &mut Cpu) -> bool {
        c.bus.read32(OC330_CTRL) == OC330_MAGIC
    }

    // The world's addresses (the same ones `rs207d_world` uses).
    const OC330_SYS: u32 = DATA + 0x20_0000;
    const OC330_RIDER_A: u32 = DATA + 0x21_0000;
    const OC330_RIDER_B: u32 = DATA + 0x21_1000;

    // -- 0x290c10 Audio_EvalCurveTable --------------------------------------

    /// Index 0..4 into the five curves (times 0,1,2,3,4 at `i*0x28`, values
    /// 0,10,20,30,40 at `+0x14`), `x` in, below, inside and above the range,
    /// with f12 as the argument.
    #[test]
    fn audio_eval_curve_table_matches() {
        check(
            ee::s07::f_00290c10__generated,
            audio_eval_curve_table,
            100,
            &|c, seed| {
                let mut r = Rng(0x2900_c10 ^ (seed as u64) << 7);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let i = r.next_u32() % 5;
                c.g[A0][0] = i as u64;
                let x = match seed % 6 {
                    0 => -1.0f32,
                    1 => 0.0,
                    2 => 2.5,
                    3 => 3.99,
                    4 => 4.0,
                    _ => 9.0,
                };
                c.f[12] = x.to_bits();
                let base = AECT_TABLE + i * AECT_CURVE_SIZE;
                for k in 0..5u32 {
                    c.bus.write32(base + 4 * k, (k as f32).to_bits());
                    c.bus
                        .write32(base + 0x14 + 4 * k, (10.0 * k as f32).to_bits());
                }
            },
        );
    }

    // -- 0x3c8598 SND_EeVoiceSetAuxSend ------------------------------------

    /// Aux sends 0..3 of voices 0..7, `level` from the edge cases.
    #[test]
    fn snd_ee_voice_set_aux_send_matches() {
        check(
            ee::s14::f_003c8598__generated,
            snd_ee_voice_set_aux_send,
            100,
            &|c, seed| {
                let mut r = Rng(0x3c85_98 ^ (seed as u64) << 5);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(EVAS_VOICES_PTR, DATA + 0x2000);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    // -- 0x3612c0 IntersectInfo_ResetMatrix --------------------------------

    /// The identity quadwords at `D_004FF1A0` copied over a random matrix.
    #[test]
    fn intersect_info_reset_matrix_matches() {
        check(
            ee::s12::f_003612c0__generated,
            intersect_info_reset_matrix,
            60,
            &|c, seed| {
                let mut r = Rng(0x3612_c0 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                elf_image(c);
                for k in 0..4u32 {
                    put_vec(
                        c,
                        IIRM_IDENTITY + 0x10 * k,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                let this = DATA + 0x1000;
                for k in 0..4u32 {
                    put_vec(
                        c,
                        this + IIRM_MATRIX + 0x10 * k,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    // -- 0x2b0e28 SpeechSystem_IsInstanceReady ------------------------------

    /// The speech system of the 2077 world (its stream array), slot 0 with a
    /// stream (ready or not) and the empty slots 1..3.
    #[test]
    fn speech_system_is_instance_ready_matches() {
        check(
            ee::s08::f_002b0e28__generated,
            speech_system_is_instance_ready,
            60,
            &|c, seed| {
                rs207d_world(c, seed);
                c.pc = 0x002b_0e28;
                c.g[A0][0] = (OC330_SYS + 0x5560) as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    // -- 0x30a5c0 WScriptMan_RunKillScript ---------------------------------

    /// A gated fake for the world's script-function lookup: `OC330_CTRL+4` chooses
    /// "not found" (-1) or the handle at `OC330_CTRL+8`.
    fn oc330_fake_get_object_script_func(c: &mut Cpu) {
        if !oc330_active(c) {
            return oc330_chain(c, 0, oc330_fake_get_object_script_func);
        }
        let missing = c.bus.read32(OC330_CTRL + 4) != 0;
        c.g[V0][0] = if missing { u64::MAX } else { 5 };
        c.g[V1][0] = 0;
        c.cycles += 6;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    /// A gated fake for the script call: records the handle argument in `v0`.
    fn oc330_fake_call_func(c: &mut Cpu) {
        if !oc330_active(c) {
            return oc330_chain(c, 1, oc330_fake_call_func);
        }
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    /// The handle published in `self+0x290`, the script object lookup's
    /// "found" and "not found" branches, and the script call.
    #[test]
    fn wscript_man_run_kill_script_matches() {
        oc330_install(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003A6B78),
            oc330_fake_get_object_script_func,
            0,
        );
        oc330_install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00309C88),
            oc330_fake_call_func,
            1,
        );
        check(
            ee::s10::f_0030a5c0__generated,
            wscript_man_run_kill_script,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_a5c0, 1);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(this + RKS_SCRIPT, 0);
                c.bus.write32(OC330_CTRL, OC330_MAGIC);
                c.bus.write32(OC330_CTRL + 4, (seed % 2 == 0) as u32);
                c.bus.write32(OC330_CTRL + 8, 0);
                c.pc = 0x0030_a5c0;
            },
        );
    }

    // -- 0x3cd1d0 SND_UtkFree ----------------------------------------------

    /// The free hook is a fake (the miss hook), reached with the pointer in
    /// `a0`; some seeds unwind inside it.
    #[test]
    fn snd_utk_free_matches() {
        check(ee::s14::f_003cd1d0__generated, snd_utk_free, 60, &|c, seed| {
            machine(c, seed, 0x003c_d1d0, 1);
            let p = DATA + 0x2000 + 0x40 * (seed % 8);
            c.g[A0][0] = p as u64;
            c.bus
                .write32(UTK_FREE_HOOK, if seed % 3 == 0 { 0 } else { FAKE });
            c.pc = 0x003c_d1d0;
        });
    }

    // -- 0x29f000 SSXAudioSystem_GetSpeechCategoryVolume -------------------

    /// A voice argument set and clear; types 0, 2, 0xB and others; the
    /// channel in a3. The fade-mode and gain-pointer callees are real.
    #[test]
    fn ssx_audio_system_get_speech_category_volume_matches() {
        check(
            ee::s08::f_0029f000__generated,
            ssx_audio_system_get_speech_category_volume,
            100,
            &|c, seed| {
                let mut r = Rng(0x29f0_00 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 4 == 0 { 1 } else { 0 };
                let ty = [0u32, 1, 2, 0xb, 0x2077][(seed / 2 % 5) as usize];
                c.g[A2][0] = ty as u64;
                c.g[A3][0] = (r.next_u32() % 4) as u64;
                // The gain table the 0xB category returns.
                c.bus.write32(this + 0x5730, r.next_u32());
                c.bus.write32(this + 0x5734, r.next_u32());
            },
        );
    }

    // -- 0x3c8b40 SND_EeVoiceSetTimeStretch --------------------------------

    /// Voices 0..7 with and without a stretch value; the real
    /// `SND_DspStretch_SetRatio` on the voice.
    #[test]
    fn snd_ee_voice_set_time_stretch_matches() {
        check(
            ee::s14::f_003c8b40__generated,
            snd_ee_voice_set_time_stretch,
            100,
            &|c, seed| {
                let mut r = Rng(0x3c8b_40 ^ (seed as u64) << 13);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let voices = DATA + 0x2000;
                c.bus.write32(EVTS_VOICES_PTR, voices);
                let voice = r.next_u32() % 8;
                c.g[A0][0] = voice as u64;
                c.f[12] = if seed % 5 == 0 {
                    1.0f32.to_bits()
                } else {
                    r.f32(4.0)
                };
                let stretch = if seed % 3 == 0 {
                    0
                } else {
                    voices + EVTS_VOICE_SIZE * voice
                };
                c.bus
                    .write32(voices + EVTS_VOICE_SIZE * voice + EVTS_STRETCH, stretch);
                // The stretch object's ratio field at +0x24.
                if stretch != 0 {
                    c.bus.write32(stretch + 0x24, r.edge_f32());
                }
            },
        );
    }

    // -- 0x2a1820 SSXAudioSystem_GetRiderRivalry ---------------------------

    /// The 2077 world (BE aggression singletons, player records, riders) with
    /// the audio instance's track mode 2 or not, and the riders' human/local
    /// flags from the world; the BE interface leaves run for real.
    #[test]
    fn ssx_audio_system_get_rider_rivalry_matches() {
        check(
            ee::s08::f_002a1820__generated,
            ssx_audio_system_get_rider_rivalry,
            150,
            &|c, seed| {
                let mut r = rs207d_world(c, seed);
                c.pc = 0x002a_1820;
                c.g[A0][0] = OC330_SYS as u64;
                c.g[A1][0] = OC330_RIDER_A as u64;
                c.g[A2][0] = OC330_RIDER_B as u64;
                // The audio instance and its track mode (2 for some seeds).
                let audio = DATA + 0x21_c000;
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(
                    audio + GRR_TRACK_MODE,
                    if seed % 4 == 0 { 2 } else { r.next_u32() % 3 },
                );
                // Extra variety in the two "human"/"local" flags.
                for rider in [OC330_RIDER_A, OC330_RIDER_B] {
                    c.bus.write32(rider + GRR_HUMAN, r.next_u32() % 2);
                    c.bus.write32(rider + GRR_LOCAL, r.next_u32() % 2);
                }
            },
        );
    }

    // -- 0x2a0e70 SSXAudioSystem_RiderSay2077 ------------------------------

    /// The 2077 world with a listener radius that puts the riders in range:
    /// speech off, replaying, AI / followed / local riders, on-course riders,
    /// an instance that is not ready, the view check, the grunt (short path)
    /// and the masked line with accepted and rejected speech contexts.
    #[test]
    fn ssx_audio_system_rider_say_2077_matches() {
        check(
            ee::s08::f_002a0e70__generated,
            ssx_audio_system_rider_say_2077,
            120,
            &|c, seed| {
                let mut r = rs207d_world(c, seed);
                c.pc = 0x002a_0e70;
                c.g[A0][0] = OC330_SYS as u64;
                c.g[A1][0] = OC330_RIDER_A as u64;
                c.g[A2][0] = (seed % 2) as u64;
                // A radius that hears the riders at (1,2,3).
                c.bus.write32(GAME_GP + RS2077_RADIUS_GP, 10.0f32.to_bits());
                // The grunt checks read the rider's character index.
                c.bus.write32(OC330_RIDER_A + 0xec, r.next_u32() % 4);
            },
        );
    }
}
