//! Staging module oc337 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc337`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_95f0, snd_dec_utk_free_loop, "SND_DecUtk_FreeLoop"),
    (
        0x002a_f8a8,
        speech_system_spch_event_callback,
        "SpeechSystem_SpchEventCallback",
    ),
    (0x0034_6120, halo_modifier_ctor, "HaloModifier_ctor"),
    (
        0x0034_54e8,
        spline_modifier_get_path_length,
        "SplineModifier_GetPathLength",
    ),
    (
        0x003c_e378,
        sndstrm_find_by_channel,
        "SNDSTRM_FindByChannel",
    ),
    (
        0x003c_a610,
        snd_dec_eaxa_init_stream,
        "SND_DecEaxa_InitStream",
    ),
    (0x003b_68b8, sndstrm_start_playback, "SNDSTRM_StartPlayback"),
    (
        0x0029_f330,
        ssx_audio_system_get_rider_speech_mask,
        "SSXAudioSystem_GetRiderSpeechMask",
    ),
    (
        0x003c_e118,
        snd_ee_voice_get_sub_index,
        "SND_EeVoiceGetSubIndex",
    ),
    (
        0x0034_5720,
        modifier_list_push_back,
        "ModifierList_PushBack",
    ),
];

// ---------------------------------------------------------------------------
// SND_DecUtk_FreeLoop (0x3c95f0).

/// `SND_UtkFree(void*)` (0x3cd1d0).
const DEC_UTK_FREE_LOOP_UTK_FREE: Callee = Callee {
    addr: 0x003c_d1d0,
    f: ee::s14::SND_UtkFree,
};

/// The UTK decoder's work buffer.
pub(crate) const DEC_UTK_FREE_LOOP_BUFFER: u32 = 0x1c;

/// `void SND_DecUtk_FreeLoop(UtkDecoder* dec)` (0x3c95f0): give the
/// decoder's work buffer (`+0x1c`) back with `SND_UtkFree`, if it has one.
pub fn snd_dec_utk_free_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dec = r32(c, A0);
        c.g[A0][0] = Addr(dec.wrapping_add(DEC_UTK_FREE_LOOP_BUFFER)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            DEC_UTK_FREE_LOOP_UTK_FREE.call(c, 0x003c_960c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_SpchEventCallback (0x2af8a8).

/// `int SpeechSystem_OnSpchEvent(SpeechSystem*, SpchEvent*)` (0x2b0f48).
const SPCH_EVENT_CB_ON_SPCH_EVENT: Callee = Callee {
    addr: 0x002b_0f48,
    f: ee::s08::SpeechSystem_OnSpchEvent,
};

/// gp-relative: the flag that makes the callback refuse events.
pub(crate) const SPCH_EVENT_CB_BLOCKED: u32 = 0x6cc;

/// gp-relative: the speech system the events go to.
pub(crate) const SPCH_EVENT_CB_SYSTEM: u32 = 0x6c4;

/// `int SpeechSystem_SpchEventCallback(SpchEvent* ev)` (0x2af8a8): the
/// speech library's event callback. Refused (-1) while the flag at
/// `gp+0x6cc` is set; otherwise the event goes to
/// `SpeechSystem_OnSpchEvent(system at gp+0x6c4, ev)` and its result is
/// returned.
pub fn speech_system_spch_event_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(SPCH_EVENT_CB_BLOCKED)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(gp.wrapping_add(SPCH_EVENT_CB_SYSTEM)).lw(c);
            SPCH_EVENT_CB_ON_SPCH_EVENT.call(c, 0x002a_f8c4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HaloModifier_ctor (0x346120).

/// `ParticleModifier_BaseCtor(ParticleModifier*, void* owner)` (0x345798).
const HALO_MODIFIER_CTOR_BASE_CTOR: Callee = Callee {
    addr: 0x0034_5798,
    f: ee::s11::ParticleModifier_BaseCtor,
};

/// The HaloModifier class vtable.
pub(crate) const HALO_MODIFIER_CTOR_VTABLE: u32 = 0x0049_1220;

/// gp-relative: the live HaloModifier count (`D_004A4000`).
pub(crate) const HALO_MODIFIER_CTOR_COUNT: u32 = 0xf10;

/// `HaloModifier* HaloModifier_ctor(HaloModifier* self, HaloDesc* desc,
/// void* owner)` (0x346120): base particle-modifier constructor on `owner`,
/// this class's vtable, then copy the description: words `+4`, `+8` to
/// `+0x24`, `+0x28`; the quaternion stored `(w, x, y, z)` at `+0xc..+0x1c`
/// to `(x, y, z, w)` at `+0x10` (through a 16-byte stack temporary); `+0x1c`
/// to `+0x20`, `+0x20` to `+0x2c`; `+0x30 = 0`. Bumps the live count.
pub fn halo_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        HALO_MODIFIER_CTOR_BASE_CTOR.call(c, 0x0034_6140)?;
        let gp = r32(c, GP);
        let this = r32(c, S1);
        let desc = r32(c, S0);
        c.g[A1][0] = Addr(gp.wrapping_add(HALO_MODIFIER_CTOR_COUNT)).lw(c);
        c.g[V1][0] = sx(HALO_MODIFIER_CTOR_VTABLE);
        c.g[V0][0] = c.g[S1][0];
        Addr(this.wrapping_add(8)).sw(c, sx(HALO_MODIFIER_CTOR_VTABLE));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        frame.restore_ra(c, 0x10);
        c.g[V1][0] = Addr(desc.wrapping_add(4)).lw(c);
        let v1 = c.g[V1][0];
        Addr(this.wrapping_add(0x24)).sw(c, v1);
        c.g[A0][0] = Addr(desc.wrapping_add(8)).lw(c);
        let a0 = c.g[A0][0];
        Addr(this.wrapping_add(0x28)).sw(c, a0);
        // The quaternion, reordered through the stack.
        c.lwc1(0, desc.wrapping_add(0x10));
        c.lwc1(1, desc.wrapping_add(0x14));
        c.lwc1(2, desc.wrapping_add(0x18));
        c.lwc1(3, desc.wrapping_add(0xc));
        let tmp = frame.at(c, 0);
        c.swc1(0, tmp);
        c.swc1(1, tmp.wrapping_add(4));
        c.swc1(2, tmp.wrapping_add(8));
        c.swc1(3, tmp.wrapping_add(0xc));
        c.lq(V1, tmp);
        c.sq(V1, this.wrapping_add(0x10));
        c.lwc1(0, desc.wrapping_add(0x1c));
        c.swc1(0, this.wrapping_add(0x20));
        c.g[V1][0] = Addr(desc.wrapping_add(0x20)).lw(c);
        Addr(this.wrapping_add(0x30)).sw(c, 0);
        let v1 = c.g[V1][0];
        Addr(this.wrapping_add(0x2c)).sw(c, v1);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        let count = c.g[A1][0];
        Addr(gp.wrapping_add(HALO_MODIFIER_CTOR_COUNT)).sw(c, count);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SplineModifier_GetPathLength (0x3454e8).

/// `float SplineModifier_GetPathLength(SplineModifier* m)` (0x3454e8)
/// (`cSpline_calcLength`): the spline (`m->owner(+8)->spline(+0x68)`) has
/// `count` (`+0x20`) nodes linked from `+0x24` through `+0x64`; walk to the
/// last one and return its start distance (`+0x84`) plus its length
/// (`+0xc`).
pub fn spline_modifier_get_path_length(c: &mut Cpu) {
    c.cycles += 7;
    let m = r32(c, A0);
    c.g[V0][0] = Addr(m.wrapping_add(8)).lw(c);
    c.g[A0][0] = 1;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x68)).lw(c);
    let spline = r32(c, V1);
    c.g[A1][0] = Addr(spline.wrapping_add(0x20)).lw(c);
    c.g[V0][0] = (1i64 < c.g[A1][0] as i64) as u64;
    let walk = c.g[V0][0] != 0;
    c.g[V1][0] = Addr(spline.wrapping_add(0x24)).lw(c);
    if walk {
        c.cycles += 1;
        c.g[A0][0] = sx(r32(c, A1).wrapping_sub(1));
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            let more = c.g[A0][0] != 0;
            c.g[V1][0] = Addr(r32(c, V1).wrapping_add(0x64)).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_5508);
            }
        }
    }
    c.cycles += 4;
    let node = r32(c, V1);
    c.lwc1(1, node.wrapping_add(0xc));
    c.lwc1(0, node.wrapping_add(0x84));
    let length = FpuF32(c.f[0]) + FpuF32(c.f[1]);
    c.f[0] = length.0;
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_FindByChannel (0x3ce378).

/// The sound library's state (`D_0050A8E8`): the channel count byte at
/// `+0x2f`/`+0x30` offsets, the stream count at `+0x46`, the channel array
/// pointer at `+0x1f0` (0x8c bytes per channel).
pub(crate) const SND_STATE_OC337: u32 = 0x0050_a8e8;

/// The stream count byte in [`SND_STATE_OC337`].
pub(crate) const SND_STATE_OC337_STREAM_COUNT: u32 = 0x46;

/// The stream slot table (`D_005157B8 + 0x304`, one pointer per stream).
pub(crate) const SNDSTRM_FIND_BY_CHANNEL_SLOTS: u32 = 0x0051_5abc;

/// `int SNDSTRM_FindByChannel(int channel)` (0x3ce378): the index of the
/// first stream slot whose stream's handle (`*slot`) maps to `channel`
/// (`SND_HandleToChannel`), or -1.
pub fn sndstrm_find_by_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x51_0000;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = sx(SND_STATE_OC337);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        c.g[V0][0] = Addr(SND_STATE_OC337 + SND_STATE_OC337_STREAM_COUNT).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        'found: {
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[V0][0] = sx(0x0051_57b8);
                c.g[S1][0] = sx(SNDSTRM_FIND_BY_CHANNEL_SLOTS);
                loop {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                    if c.g[A0][0] != 0 {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
                        call_ptr(c, 0x003b_a6e8, 0x003c_e3d0)?;
                        if c.g[V0][0] == c.g[S2][0] {
                            c.cycles += 2;
                            c.g[V0][0] = c.g[S0][0];
                            break 'found;
                        }
                    }
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SND_STATE_OC337_STREAM_COUNT)).lbu(c);
                    c.cycles += 4;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_e3b8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
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
// SND_DecEaxa_InitStream (0x3ca610).

/// `int SND_EeVoiceGetMasterChannel(int voice)` (0x3ce0e0).
const DEC_EAXA_INIT_MASTER_CHANNEL: Callee = Callee {
    addr: 0x003c_e0e0,
    f: ee::s14::SND_EeVoiceGetMasterChannel,
};

/// `int SNDSTRM_FindByChannel(int channel)` (0x3ce378).
const DEC_EAXA_INIT_FIND_BY_CHANNEL: Callee = Callee {
    addr: 0x003c_e378,
    f: ee::s14::SNDSTRM_FindByChannel,
};

/// `int SND_EeVoiceGetSubIndex(int voice)` (0x3ce118).
const DEC_EAXA_INIT_SUB_INDEX: Callee = Callee {
    addr: 0x003c_e118,
    f: ee::s14::SND_EeVoiceGetSubIndex,
};

/// `void* SND_EaxaAlloc(int size)` (0x3ccef0).
const DEC_EAXA_INIT_ALLOC: Callee = Callee {
    addr: 0x003c_cef0,
    f: ee::s14::SND_EaxaAlloc,
};

/// `EaxaState* SND_EaxaReset(EaxaState*)` (0x3ccf40).
const DEC_EAXA_INIT_RESET: Callee = Callee {
    addr: 0x003c_cf40,
    f: ee::s14::SND_EaxaReset,
};

/// The EA-XA decoder's two methods (`+0`, `+4`).
pub(crate) const DEC_EAXA_INIT_METHOD0: u32 = 0x003c_a460;

pub(crate) const DEC_EAXA_INIT_METHOD1: u32 = 0x003c_a5e8;

/// `void SND_DecEaxa_InitStream(EaxaDecoder* dec, StreamDesc* desc)`
/// (0x3ca610): set up an EA-XA stream decoder for the voice `desc+0x1c`:
/// its methods (`+0`, `+4`), the stream of the voice's master channel
/// (`+0x30`), the voice's sub index (byte `+0x38`), `desc+0x24` to `+0x2c`,
/// `desc+0xc` to `+0x24`, cleared counters (`+0x20`, `+0x28`, `+0x34`) and a
/// fresh 0xa8-byte EA-XA state (`+0x1c`).
pub fn snd_dec_eaxa_init_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x3d_0000;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x3d_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(DEC_EAXA_INIT_METHOD1);
        c.g[V0][0] = sx(DEC_EAXA_INIT_METHOD0);
        let dec = r32(c, S0);
        Addr(dec.wrapping_add(4)).sw(c, sx(DEC_EAXA_INIT_METHOD1));
        Addr(dec).sw(c, sx(DEC_EAXA_INIT_METHOD0));
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        DEC_EAXA_INIT_MASTER_CHANNEL.call(c, 0x003c_a648)?;
        c.g[A0][0] = c.g[V0][0];
        DEC_EAXA_INIT_FIND_BY_CHANNEL.call(c, 0x003c_a650)?;
        let v0 = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0x30)).sw(c, v0);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        DEC_EAXA_INIT_SUB_INDEX.call(c, 0x003c_a65c)?;
        let (dec, desc) = (r32(c, S0), r32(c, S1));
        let sub = c.g[V0][0];
        Addr(dec.wrapping_add(0x38)).sb(c, sub);
        c.g[A0][0] = 0xa8;
        c.g[V0][0] = Addr(desc.wrapping_add(0x24)).lw(c);
        Addr(dec.wrapping_add(0x20)).sw(c, 0);
        let v0 = c.g[V0][0];
        Addr(dec.wrapping_add(0x2c)).sw(c, v0);
        c.g[V1][0] = Addr(desc.wrapping_add(0xc)).lw(c);
        Addr(dec.wrapping_add(0x28)).sw(c, 0);
        Addr(dec.wrapping_add(0x34)).sw(c, 0);
        let v1 = c.g[V1][0];
        Addr(dec.wrapping_add(0x24)).sw(c, v1);
        DEC_EAXA_INIT_ALLOC.call(c, 0x003c_a684)?;
        c.g[A0][0] = c.g[V0][0];
        DEC_EAXA_INIT_RESET.call(c, 0x003c_a68c)?;
        let state = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0x1c)).sw(c, state);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_StartPlayback (0x3b68b8).

/// `int SNDSTRM_StartChannel(int, void*, void*, void*)` (0x3c6760).
const START_PLAYBACK_START_CHANNEL: Callee = Callee {
    addr: 0x003c_6760,
    f: ee::s13::SNDSTRM_StartChannel,
};

/// `int SND_ApplyChannelParams(int handle, ChannelParams*)` (0x3b8cc8).
const START_PLAYBACK_APPLY_PARAMS: Callee = Callee {
    addr: 0x003b_8cc8,
    f: ee::s13::SND_ApplyChannelParams,
};

/// `void SNDSTRM_StartPlayback(Stream* s)` (0x3b68b8): start the stream's
/// channel (`SNDSTRM_StartChannel(s+8 word, s+0x14, s+0x1c, s+0xec)`, the
/// handle to `+4`), apply the parameter block `+0x104` when its flag
/// (`+0x114`) is set, and mark the stream playing (byte `+0x10`). The
/// C++ is a `do { ... } while (++i == 0)` loop: it runs once.
pub fn sndstrm_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        let strm = r32(c, S0);
        c.g[A1][0] = sx(strm.wrapping_add(0x14));
        frame.save(c, S3, 0x10);
        c.g[A2][0] = sx(strm.wrapping_add(0x1c));
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(strm.wrapping_add(0xec));
        c.g[S3][0] = 0;
        c.g[S2][0] = sx(strm.wrapping_add(0x104));
        c.g[A0][0] = Addr(strm.wrapping_add(8)).lw(c);
        c.g[S1][0] = sx(strm.wrapping_add(0x114));
        START_PLAYBACK_START_CHANNEL.call(c, 0x003b_68f4)?;
        let handle = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(4)).sw(c, handle);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
                c.g[A1][0] = c.g[S2][0];
                START_PLAYBACK_APPLY_PARAMS.call(c, 0x003b_6910)?;
            }
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x18));
            c.cycles += 3;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            let again = c.g[S3][0] == 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x18));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_68f8);
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x30);
        Addr(r32(c, S0).wrapping_add(0x10)).sb(c, 1);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetRiderSpeechMask (0x29f330).

/// `int SSXAudioSystem_CharacterToSpeechMask(int character, void* info)`
/// (0x29f2b0).
const RIDER_SPEECH_MASK_CHARACTER_TO_MASK: Callee = Callee {
    addr: 0x0029_f2b0,
    f: ee::s08::SSXAudioSystem_CharacterToSpeechMask,
};

/// `Rider_GetCharacter`, `Rider_GetPlayerCharacterInfo` (another shard:
/// through the function table).
pub(crate) const RIDER_SPEECH_MASK_GET_CHARACTER: u32 = 0x0012_3128;

pub(crate) const RIDER_SPEECH_MASK_GET_INFO: u32 = 0x0012_3168;

/// `int SSXAudioSystem_GetRiderSpeechMask(Rider* rider)` (0x29f330): the
/// speech mask of the rider's character
/// (`CharacterToSpeechMask(Rider_GetCharacter(r),
/// Rider_GetPlayerCharacterInfo(r))`).
pub fn ssx_audio_system_get_rider_speech_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, RIDER_SPEECH_MASK_GET_CHARACTER, 0x0029_f348)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, RIDER_SPEECH_MASK_GET_INFO, 0x0029_f354)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        RIDER_SPEECH_MASK_CHARACTER_TO_MASK.call(c, 0x0029_f360)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceGetSubIndex (0x3ce118).

/// Channel record size in the channel array (`SND_STATE_OC337 + 0x1f0`).
pub(crate) const SND_CHANNEL_SIZE_OC337: u32 = 0x8c;

/// `int SND_EeVoiceGetSubIndex(int voice)` (0x3ce118): the voice's channel
/// index is `voice + state[0x30] + state[0x2f]`; if that channel has a
/// master (`+0x28` != -1), the 1-based slot (1..5) of the index among the
/// master's sub-voice halfwords (`+6..+0xe`), else 0.
pub fn snd_ee_voice_get_sub_index(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x51_0000;
    c.g[T0][0] = SND_CHANNEL_SIZE_OC337 as u64;
    c.g[V0][0] = sx(SND_STATE_OC337);
    c.g[A2][0] = u64::MAX;
    c.g[V1][0] = Addr(SND_STATE_OC337 + 0x30).lbu(c);
    c.g[A1][0] = Addr(SND_STATE_OC337 + 0x2f).lbu(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    c.g[A3][0] = Addr(SND_STATE_OC337 + 0x1f0).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    c.mult(0, V0, A0, T0);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lh(c);
    let has_master = c.g[V0][0] != c.g[A2][0];
    c.mult(0, V1, V0, T0);
    if !has_master {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return ret(c);
    }
    c.cycles += 4;
    c.g[A1][0] = 1;
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(6));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1)).lh(c);
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            return ret(c);
        }
        c.cycles += 4;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return ret(c);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_e170);
        }
    }
}

// ---------------------------------------------------------------------------
// ModifierList_PushBack (0x345720).

/// `void ModifierList_PushBack(Link* list, Link* node)` (0x345720): append
/// `node` after the last link of the list (`next` at `+0`, `prev` at `+4`;
/// the list head is a link itself).
pub fn modifier_list_push_back(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                break;
            }
            c.g[A0][0] = Addr(r32(c, A0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0034_5730);
            }
        }
    }
    let node = r32(c, A1);
    Addr(node).sw(c, 0);
    c.cycles += 3;
    let last = c.g[A0][0];
    Addr(node.wrapping_add(4)).sw(c, last);
    let node64 = c.g[A1][0];
    Addr(last as u32).sw(c, node64);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{covered, fake, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gated slot fakes (the suite runs tests in parallel): a fake runs only
    /// while its magic is at this address of the machine's RAM, else the body
    /// the slot held.
    pub(crate) const OC337_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC337_MAGIC_SPEECH: u32 = 0x0c33_7a01;

    pub(crate) const OC337_MAGIC_START_CHANNEL: u32 = 0x0c33_7a02;

    pub(crate) const OC337_MAGIC_RIDER: u32 = 0x0c33_7a03;

    /// Arm the fake whose magic is `magic`.
    fn oc337_arm(c: &mut Cpu, magic: u32) {
        c.bus.write32(OC337_CTRL, magic);
    }

    /// Run event seeds: after the fixture, schedule an event at the first
    /// back edge (for both bodies alike).
    fn oc337_events(c: &mut Cpu, seed: u32, entry: u32) -> Events {
        let mut r = Rng(entry as u64 ^ (seed as u64) << 12);
        let mode = schedule(c, &mut r, seed, entry);
        if mode != Events::Off {
            c.next_event = c.cycles + 4;
        }
        mode
    }

    // -- SND_DecUtk_FreeLoop ------------------------------------------------

    /// With and without a work buffer; the library's free hook is a fake
    /// (the miss hook), unwinding for some seeds.
    #[test]
    fn snd_dec_utk_free_loop_matches() {
        covered(
            ee::s14::f_003c95f0__generated,
            snd_dec_utk_free_loop,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_95f0, 1);
                c.pc = 0x003c_95f0;
                let dec = DATA + 0x100;
                fill(c, &mut r, dec, 0x10);
                let buf = if seed % 3 == 0 {
                    0
                } else {
                    DATA + 0x2000 + 0x40 * (seed % 8)
                };
                c.bus.write32(dec + DEC_UTK_FREE_LOOP_BUFFER, buf);
                c.bus.write32(0x0051_7608, FAKE);
                c.g[A0][0] = sx(dec);
            },
        );
    }

    // -- SpeechSystem_SpchEventCallback -------------------------------------

    static OC337_PREV_ON_SPCH_EVENT: AtomicUsize = AtomicUsize::new(0);

    fn oc337_fake_on_spch_event(c: &mut Cpu) {
        if c.bus.read32(OC337_CTRL) != OC337_MAGIC_SPEECH {
            return crate::difftest::fakes::fall_back(
                c,
                &OC337_PREV_ON_SPCH_EVENT,
                ee::s08::f_002b0f48__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x002b_0f48, ra);
    }

    /// Blocked (-1) or passed to a fake `SpeechSystem_OnSpchEvent`
    /// (sometimes unwinding).
    #[test]
    fn speech_system_spch_event_callback_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002B0F48),
            oc337_fake_on_spch_event,
            &OC337_PREV_ON_SPCH_EVENT,
        );
        covered(
            ee::s08::f_002af8a8__generated,
            speech_system_spch_event_callback,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_f8a8, 1);
                c.pc = 0x002a_f8a8;
                oc337_arm(c, OC337_MAGIC_SPEECH);
                let blocked = if seed % 3 == 0 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(GAME_GP + SPCH_EVENT_CB_BLOCKED, blocked);
                c.bus.write32(GAME_GP + SPCH_EVENT_CB_SYSTEM, DATA + 0x200);
                c.g[A0][0] = sx(DATA + 0x400 + 0x10 * (seed % 16));
            },
        );
    }

    // -- HaloModifier_ctor --------------------------------------------------

    /// A random description and modifier; the real base constructor.
    #[test]
    fn halo_modifier_ctor_matches() {
        check(
            ee::s11::f_00346120__generated,
            halo_modifier_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_6120, 0);
                c.pc = 0x0034_6120;
                let this = DATA + 0x400;
                let desc = DATA + 0x800;
                fill(c, &mut r, this, 0x10);
                fill(c, &mut r, desc, 0x10);
                for i in 0..4 {
                    c.bus.write32(desc + 0xc + 4 * i, r.edge_f32());
                }
                c.bus.write32(GAME_GP + 0xf04, r.next_u32());
                c.bus
                    .write32(GAME_GP + HALO_MODIFIER_CTOR_COUNT, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(desc);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    // -- SplineModifier_GetPathLength ---------------------------------------

    /// A spline of `count` nodes (-1..6, so also the no-walk counts 0 and 1)
    /// linked through `+0x64`, with edge-case lengths and distances.
    fn oc337_path_length_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x34_54e8, 0);
        c.pc = 0x0034_54e8;
        let m = DATA;
        let owner = DATA + 0x100;
        let spline = DATA + 0x200;
        let nodes = DATA + 0x1000;
        c.bus.write32(m + 8, owner);
        c.bus.write32(owner + 0x68, spline);
        let count = (r.next_u32() % 8) as i32 - 1;
        c.bus.write32(spline + 0x20, count as u32);
        c.bus.write32(spline + 0x24, nodes);
        for i in 0..8u32 {
            let n = nodes + 0x100 * i;
            c.bus.write32(n + 0x64, n + 0x100);
            c.bus.write32(n + 0xc, r.edge_f32());
            c.bus.write32(n + 0x84, r.edge_f32());
        }
        c.g[A0][0] = sx(m);
    }

    driven!(oc337_gen_path_length, ee::s11::f_003454e8__generated);

    driven!(oc337_hand_path_length, spline_modifier_get_path_length);

    #[test]
    fn spline_modifier_get_path_length_matches() {
        check(
            ee::s11::f_003454e8__generated,
            spline_modifier_get_path_length,
            200,
            &oc337_path_length_setup,
        );
        check_events(
            oc337_gen_path_length,
            oc337_hand_path_length,
            60,
            &|c, seed| {
                oc337_path_length_setup(c, seed);
                oc337_events(c, seed, 0x0034_54e8)
            },
        );
    }

    // -- Sound channel tables (FindByChannel, InitStream, GetSubIndex) ------

    /// The channel array (8 records of 0x8c bytes).
    pub(crate) const OC337_CHANNELS: u32 = DATA + 0x1000;

    /// Stream objects (their first word is a channel handle).
    pub(crate) const OC337_STREAMS: u32 = DATA + 0x2000;

    /// The sound state's channel tables: 8 channels with handles
    /// `i | k << 8` (k 0..2), a live byte (`+0x69`), a master (`+0x28`: -1 or
    /// 0..7) and five sub-voice halfwords (`+6..+0xe`); 0..6 stream slots
    /// (some empty) whose streams carry a live handle, a stale one (`k = 3`)
    /// or a negative one. `SND_HandleToChannel` runs for real on them.
    fn oc337_snd_tables(c: &mut Cpu, r: &mut Rng) {
        let st = SND_STATE_OC337;
        c.bus.write8(st + 0x30, (r.next_u32() % 2) as u8);
        c.bus.write8(st + 0x2f, (r.next_u32() % 2) as u8);
        c.bus.write32(st + 0x1f0, OC337_CHANNELS);
        c.bus.write16(st + 0x17e, 8);
        for i in 0..8u32 {
            let e = OC337_CHANNELS + SND_CHANNEL_SIZE_OC337 * i;
            c.bus.write32(e, i | (r.next_u32() % 3) << 8);
            c.bus.write8(e + 0x69, (r.next_u32() % 4 != 0) as u8);
            let master = if r.next_u32() % 3 == 0 {
                0xffff
            } else {
                r.next_u32() % 8
            };
            c.bus.write16(e + 0x28, master as u16);
            for k in 0..5 {
                c.bus.write16(e + 6 + 2 * k, (r.next_u32() % 9) as u16);
            }
        }
        let count = r.next_u32() % 7;
        c.bus.write8(st + SND_STATE_OC337_STREAM_COUNT, count as u8);
        for j in 0..6u32 {
            let s = OC337_STREAMS + 0x10 * j;
            let slot = if r.next_u32() % 5 == 0 { 0 } else { s };
            c.bus.write32(SNDSTRM_FIND_BY_CHANNEL_SLOTS + 4 * j, slot);
            let ch = r.next_u32() % 8;
            let handle = match r.next_u32() % 4 {
                0 => 0x8000_0000 | r.next_u32(),
                1 => ch | 0x300,
                _ => c.bus.read32(OC337_CHANNELS + SND_CHANNEL_SIZE_OC337 * ch),
            };
            c.bus.write32(s, handle);
        }
    }

    // -- SNDSTRM_FindByChannel ----------------------------------------------

    /// Channels 0..7, -8 (what `SND_HandleToChannel` answers for a dead
    /// handle, so dead streams match) and an unused 9.
    fn oc337_find_by_channel_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3c_e378, 0);
        c.pc = 0x003c_e378;
        oc337_snd_tables(c, &mut r);
        let channel: i32 = match seed % 6 {
            0 => -8,
            1 => 9,
            _ => (r.next_u32() % 8) as i32,
        };
        c.g[A0][0] = channel as i64 as u64;
    }

    driven!(oc337_gen_find_by_channel, ee::s14::f_003ce378__generated);

    driven!(oc337_hand_find_by_channel, sndstrm_find_by_channel);

    #[test]
    fn sndstrm_find_by_channel_matches() {
        check(
            ee::s14::f_003ce378__generated,
            sndstrm_find_by_channel,
            200,
            &oc337_find_by_channel_setup,
        );
        check_events(
            oc337_gen_find_by_channel,
            oc337_hand_find_by_channel,
            60,
            &|c, seed| {
                oc337_find_by_channel_setup(c, seed);
                oc337_events(c, seed, 0x003c_e378)
            },
        );
    }

    // -- SND_DecEaxa_InitStream ---------------------------------------------

    /// The real master-channel, stream and sub-index lookups on the channel
    /// tables; the EA-XA allocator hook (`0x517604`) is a fake returning a
    /// block (unwinding for some seeds), `SND_EaxaReset` is real.
    #[test]
    fn snd_dec_eaxa_init_stream_matches() {
        covered(
            ee::s14::f_003ca610__generated,
            snd_dec_eaxa_init_stream,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_a610, 1);
                c.pc = 0x003c_a610;
                oc337_snd_tables(c, &mut r);
                c.bus.write32(0x0051_7604, FAKE);
                c.bus.write32(LOG_RESULTS, DATA + 0x3_0000);
                let dec = DATA + 0x4000;
                let desc = DATA + 0x4100;
                fill(c, &mut r, dec, 0x10);
                fill(c, &mut r, desc, 0x10);
                c.bus.write32(desc + 0x1c, r.next_u32() % 6);
                c.g[A0][0] = sx(dec);
                c.g[A1][0] = sx(desc);
            },
        );
    }

    // -- SNDSTRM_StartPlayback ----------------------------------------------

    static OC337_PREV_START_CHANNEL: AtomicUsize = AtomicUsize::new(0);

    fn oc337_fake_start_channel(c: &mut Cpu) {
        if c.bus.read32(OC337_CTRL) != OC337_MAGIC_START_CHANNEL {
            return crate::difftest::fakes::fall_back(
                c,
                &OC337_PREV_START_CHANNEL,
                ee::s13::f_003c6760__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x003c_6760, ra);
    }

    /// A fake `SNDSTRM_StartChannel` (sometimes unwinding) answering a
    /// pointer or a negative handle; with the parameter flag set, the real
    /// `SND_ApplyChannelParams` finds no channel for it (no channels: count
    /// `+0x17e` is 0) and returns at once.
    #[test]
    fn sndstrm_start_playback_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003C6760),
            oc337_fake_start_channel,
            &OC337_PREV_START_CHANNEL,
        );
        covered(
            ee::s13::f_003b68b8__generated,
            sndstrm_start_playback,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_68b8, 1);
                c.pc = 0x003b_68b8;
                oc337_arm(c, OC337_MAGIC_START_CHANNEL);
                c.bus.write16(SND_STATE_OC337 + 0x17e, 0);
                let strm = DATA + 0x400;
                fill(c, &mut r, strm, 0x50);
                c.bus.write32(
                    strm + 0x114,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let handle = match seed % 3 {
                    0 => 0,
                    1 => 0xffff_fff8,
                    _ => 0x8000_0000 | r.next_u32(),
                };
                c.bus.write32(LOG_RESULTS, handle);
                c.g[A0][0] = sx(strm);
            },
        );
    }

    // -- SSXAudioSystem_GetRiderSpeechMask ----------------------------------

    static OC337_PREV_GET_CHARACTER: AtomicUsize = AtomicUsize::new(0);

    static OC337_PREV_GET_INFO: AtomicUsize = AtomicUsize::new(0);

    fn oc337_fake_get_character(c: &mut Cpu) {
        if c.bus.read32(OC337_CTRL) != OC337_MAGIC_RIDER {
            return crate::difftest::fakes::fall_back(
                c,
                &OC337_PREV_GET_CHARACTER,
                ee::s00::f_00123128__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fake(c, RIDER_SPEECH_MASK_GET_CHARACTER, ra);
    }

    fn oc337_fake_get_info(c: &mut Cpu) {
        if c.bus.read32(OC337_CTRL) != OC337_MAGIC_RIDER {
            return crate::difftest::fakes::fall_back(
                c,
                &OC337_PREV_GET_INFO,
                ee::s00::f_00123168__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fake(c, RIDER_SPEECH_MASK_GET_INFO, ra);
    }

    /// Fake rider lookups (either may unwind) answering a character 1..9 or
    /// a pointer, and an info pointer; the real `CharacterToSpeechMask`.
    #[test]
    fn ssx_audio_system_get_rider_speech_mask_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00123128),
            oc337_fake_get_character,
            &OC337_PREV_GET_CHARACTER,
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00123168),
            oc337_fake_get_info,
            &OC337_PREV_GET_INFO,
        );
        covered(
            ee::s08::f_0029f330__generated,
            ssx_audio_system_get_rider_speech_mask,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_f330, 2);
                c.pc = 0x0029_f330;
                oc337_arm(c, OC337_MAGIC_RIDER);
                c.bus.write32(LOG_RESULTS, seed % 10);
                c.bus.write32(LOG_RESULTS + 4, 0);
                c.g[A0][0] = sx(DATA + 0x100 + 0x10 * (r.next_u32() % 16));
            },
        );
    }

    // -- SND_EeVoiceGetSubIndex ---------------------------------------------

    /// Voices 0..5 on the channel tables; for half the seeds the channel's
    /// index is planted in one of its master's sub-voice slots.
    fn oc337_sub_index_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3c_e118, 0);
        c.pc = 0x003c_e118;
        oc337_snd_tables(c, &mut r);
        let voice = r.next_u32() % 6;
        let st = SND_STATE_OC337;
        let index = voice + c.bus.read8(st + 0x30) as u32 + c.bus.read8(st + 0x2f) as u32;
        let master = c
            .bus
            .read16(OC337_CHANNELS + SND_CHANNEL_SIZE_OC337 * index + 0x28);
        if seed % 2 == 0 && master != 0xffff {
            let slot = r.next_u32() % 5;
            c.bus.write16(
                OC337_CHANNELS + SND_CHANNEL_SIZE_OC337 * master as u32 + 6 + 2 * slot,
                index as u16,
            );
        }
        c.g[A0][0] = voice as u64;
    }

    driven!(oc337_gen_sub_index, ee::s14::f_003ce118__generated);

    driven!(oc337_hand_sub_index, snd_ee_voice_get_sub_index);

    #[test]
    fn snd_ee_voice_get_sub_index_matches() {
        check_lohi(
            ee::s14::f_003ce118__generated,
            snd_ee_voice_get_sub_index,
            200,
            &oc337_sub_index_setup,
        );
        check_events(oc337_gen_sub_index, oc337_hand_sub_index, 60, &|c, seed| {
            oc337_sub_index_setup(c, seed);
            oc337_events(c, seed, 0x003c_e118)
        });
    }

    // -- ModifierList_PushBack ----------------------------------------------

    /// A list of 0..5 links (the head at `DATA`) and a new node with junk
    /// in its link words.
    fn oc337_push_back_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x34_5720, 0);
        c.pc = 0x0034_5720;
        let head = DATA;
        let links = r.next_u32() % 6;
        let mut prev = head;
        for i in 0..links {
            let link = DATA + 0x100 + 0x20 * i;
            c.bus.write32(prev, link);
            c.bus.write32(link + 4, prev);
            prev = link;
        }
        c.bus.write32(prev, 0);
        let node = DATA + 0x400;
        fill(c, &mut r, node, 4);
        c.g[A0][0] = sx(head);
        c.g[A1][0] = sx(node);
    }

    driven!(oc337_gen_push_back, ee::s11::f_00345720__generated);

    driven!(oc337_hand_push_back, modifier_list_push_back);

    #[test]
    fn modifier_list_push_back_matches() {
        check(
            ee::s11::f_00345720__generated,
            modifier_list_push_back,
            200,
            &oc337_push_back_setup,
        );
        check_events(oc337_gen_push_back, oc337_hand_push_back, 60, &|c, seed| {
            oc337_push_back_setup(c, seed);
            oc337_events(c, seed, 0x0034_5720)
        });
    }
}
