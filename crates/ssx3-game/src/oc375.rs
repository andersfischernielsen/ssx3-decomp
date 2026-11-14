//! Staging module oc375 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc375`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_54f8,
        ssx_audio_system_play_stream,
        "SSXAudioSystem_PlayStream",
    ),
    (0x003d_88c8, spch_choose_sample, "SPCH_ChooseSample"),
    (
        0x0030_a868,
        wscript_man_mission_command,
        "WScriptMan_MissionCommand",
    ),
    (0x003d_9fb0, spch_build_sentence, "SPCH_BuildSentence"),
    (
        0x003d_9088,
        spch_choose_event_samples,
        "SPCH_ChooseEventSamples",
    ),
    (0x003d_9bd8, spch_play_sentence, "SPCH_PlaySentence"),
    (0x003d_7760, spch_select_request, "SPCH_SelectRequest"),
    (0x003d_7418, spch_queue_event_args, "SPCH_QueueEventArgs"),
    (
        0x002b_1268,
        speech_system_stream_start_callback,
        "SpeechSystem_StreamStartCallback",
    ),
    (0x0035_b708, rail_modifier_ctor, "RailModifier_ctor"),
    //@OVERRIDES@
];

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayStream
// ---------------------------------------------------------------------------

/// `SSXAudioSystem` field: the stream array handle (`StreamArray**`).
pub(crate) const PLAY_STREAM_ARRAY: u32 = 0x118;
/// `StreamArray` fields: the play-params stack (`+0x1d8`) and its top index.
pub(crate) const PLAY_STREAM_PARAMS: u32 = 0x1d8;
pub(crate) const PLAY_STREAM_TOP: u32 = 0x1f4;

/// `(x, y)`: `x = top << shift; y = params[field]; x += y` (the address of
/// element `top` of the per-level array at `params + field`), as the
/// compiled code computes it, `x` and `y` left in those registers.
fn play_stream_slot(c: &mut Cpu, x: usize, y: usize, field: u32, shift: u32) -> u32 {
    c.g[x][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[y][0] = c.lw(r32(c, S1).wrapping_add(field));
    c.g[x][0] = sx(r32(c, x) << shift);
    c.g[x][0] = sx(r32(c, x).wrapping_add(r32(c, y)));
    r32(c, x)
}

/// `int SSXAudioSystem_PlayStream(SSXAudioSystem* s, int id, const char*
/// name, int a3, int target, int a5, int a6, int engine, float radius,
/// int a8)` (0x2854f8): queues stream `id` on the stream array; if that
/// worked (handle >= 0), pushes a fresh play-params level (defaults: volume
/// 100/90/50, pan 0x7f, priority 1), sets the vanish radius (`radius`, or
/// 100 / the engine default when `radius <= 0`), records `a6` and the
/// target (and which view it is closer to), marks replay-mirrored engine
/// streams in two-player mode, adds a bank-monitor instance with the stream
/// attached, pops the level and returns the handle.
pub fn ssx_audio_system_play_stream(c: &mut Cpu) {
    guest(c, play_stream_body);
}

fn play_stream_body(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    let frame = Frame::open(c, 0xc0);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, S7, 0x30);
    frame.save(c, FP, 0x20);
    frame.save_fpr(c, 20, 0xb0);
    frame.save(c, S0, 0xa0);
    frame.save(c, S1, 0x90);
    frame.save_ra(c, 0x10);
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = c.g[T0][0];
    c.g[S5][0] = c.g[T3][0];
    c.g[S6][0] = c.g[A1][0];
    c.g[S7][0] = c.g[A2][0];
    c.g[FP][0] = c.g[A3][0];
    c.mov_s(20, 12);
    // StreamArray_Queue(*s->array, id, a5, name, a3); a6 on the stack.
    c.g[A2][0] = c.g[T1][0];
    c.g[A3][0] = c.g[S7][0];
    c.g[T0][0] = c.g[FP][0];
    c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
    let a6 = c.g[T2][0];
    c.sw(sp(c), a6);
    call_ptr(c, 0x002a_ad78, 0x0028_555c)?;
    c.g[S4][0] = c.g[V0][0];
    c.g[T1][0] = 0x18;
    if (c.g[S4][0] as i64) >= 0 {
        play_stream_push(c)?;
    }
    c.cycles += 13;
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
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
    Ok(())
}

/// The queued path: push a level, fill it, add the instance, pop.
fn play_stream_push(c: &mut Cpu) -> Flow {
    c.cycles += 126;
    // s0 = *s->array; s1 = &s0->params; top++.
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[S0][0] = c.lw(r32(c, V1));
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[S1][0] = sx(r32(c, S0).wrapping_add(PLAY_STREAM_PARAMS));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    let top = c.g[V0][0];
    c.sw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP), top);
    // params->entries[top] (24 bytes each, at params[0x20]) = the defaults
    // at params+4.
    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
    c.mult(0, V1, V1, T1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let s0 = r32(c, S0);
    c.ldl(V0, s0.wrapping_add(0x1e3));
    c.ldr(V0, s0.wrapping_add(0x1dc));
    c.ldl(A0, s0.wrapping_add(0x1eb));
    c.ldr(A0, s0.wrapping_add(0x1e4));
    c.ldl(A3, s0.wrapping_add(0x1f3));
    c.ldr(A3, s0.wrapping_add(0x1ec));
    let e = r32(c, V1);
    c.sdl(V0, e.wrapping_add(7));
    c.sdr(V0, e);
    c.sdl(A0, e.wrapping_add(0xf));
    c.sdr(A0, e.wrapping_add(8));
    c.sdl(A3, e.wrapping_add(0x17));
    c.sdr(A3, e.wrapping_add(0x10));
    // Target word of the level (s0[0x1fc + 4 top]) and [0x34][top] cleared.
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.sw(r32(c, V0).wrapping_add(0x1fc), 0);
    let a = play_stream_slot(c, V1, V0, 0x34, 2);
    c.sw(a, 0);
    // [0x38][top] = the entry's first byte (its volume, sign-extended).
    c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[T0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, T1);
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = c.lb(r32(c, A0));
    let vol = c.g[V1][0];
    c.sw(r32(c, A3), vol);
    let a = play_stream_slot(c, V0, V1, 0x3c, 2);
    c.sw(a, 0);
    // memset(&params[0x40 + 8 top], 0, 8).
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0028_563c)?;
    // The remaining per-level fields.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.mtc1(0, 0);
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    c.c_le_s(20, 0);
    c.g[T0][0] = 1;
    let a = play_stream_slot(c, V1, V0, 0x60, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V1, V0, 0x64, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V0, V1, 0x68, 2);
    c.swc1(0, a);
    let a = play_stream_slot(c, V1, V0, 0x6c, 2);
    c.swc1(0, a);
    let a = play_stream_slot(c, V0, V1, 0x70, 4);
    c.sw(a.wrapping_add(4), 0);
    let a = play_stream_slot(c, V1, V0, 0x70, 4);
    let v = c.g[A0][0];
    c.sw(a, v);
    let a = play_stream_slot(c, V0, V1, 0x70, 4);
    let v = c.g[A1][0];
    c.sw(a.wrapping_add(8), v);
    let a = play_stream_slot(c, V1, V0, 0x70, 4);
    let v = c.g[A2][0];
    c.sw(a.wrapping_add(0xc), v);
    let a = play_stream_slot(c, V0, V1, 0x74, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V1, V0, 0x80, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V0, V1, 0x84, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V1, V0, 0x78, 2);
    let v = c.g[A3][0];
    c.sw(a, v);
    let a = play_stream_slot(c, V0, V1, 0x7c, 2);
    let v = c.g[T0][0];
    c.sw(a, v);

    // SndPlayParams_SetRadiusVanish(params, radius, -1).
    if !c.fcc() {
        c.cycles += 8;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.mov_s(12, 20);
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.cycles += 3;
        if c.g[S5][0] == 0 {
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(AT, 12);
        } else {
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x465c));
        }
        c.cycles += 6;
    }
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 13);
    c.g[A0][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(PLAY_STREAM_PARAMS));
    let fell = c.fcc();
    call_ptr(c, 0x002a_9988, if fell { 0x0028_5790 } else { 0x0028_57b4 })?;
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));

    // params[0x60][top] = a6 (kept on the stack).
    c.cycles += 8;
    c.g[A0][0] = c.lw(r32(c, V1));
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(PLAY_STREAM_TOP));
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x238));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[V1][0] = c.lw(sp(c));
    let v = c.g[V1][0];
    c.sw(r32(c, V0), v);
    if c.g[S3][0] != 0 {
        // target word = target; params[0x64][top] = IsCloserToSecondView.
        c.cycles += 16;
        c.g[A2][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = c.lw(r32(c, A2));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PLAY_STREAM_TOP));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let target = c.g[S3][0];
        c.sw(r32(c, V1).wrapping_add(0x1fc), target);
        c.g[S0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        PLAY_STREAM_CLOSER_VIEW.call(c, 0x0028_5800)?;
        c.g[A0][0] = c.lw(r32(c, S0));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(PLAY_STREAM_TOP));
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x23c));
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        let v = c.g[V0][0];
        c.sw(r32(c, V1), v);
    }

    // Engine streams in two-player mode outside replays: mirrored.
    c.cycles += 7;
    call_ptr(c, 0x0014_dc80, 0x0028_5820)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0028_582c)?;
    let have = c.g[V0][0] != 0;
    c.g[V0][0] = 0x53_0000;
    'mode: {
        if have {
            c.cycles += 4;
            c.g[A0][0] = 2;
            c.g[V1][0] = c.lb(0x0053_5c11);
            if c.g[V1][0] != c.g[A0][0] {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
                break 'mode;
            }
            c.cycles += 2;
            if c.g[S5][0] == 0 {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
                break 'mode;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            PLAY_STREAM_REPLAY_PLAYING.call(c, 0x0028_5854)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
                break 'mode;
            }
            // params[0x80][top] = 1; params[0x84][top] = a8.
            c.cycles += 16;
            c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
            c.g[A1][0] = 1;
            c.g[V1][0] = c.lw(r32(c, A0));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PLAY_STREAM_TOP));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x258));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.sw(r32(c, V0), 1);
            c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
            c.g[A0][0] = c.lw(r32(c, V1));
            c.g[V1][0] = c.lw(sp(c).wrapping_add(0xc0));
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(PLAY_STREAM_TOP));
            c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x25c));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
            let v = c.g[V1][0];
            c.sw(r32(c, V0), v);
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
    }

    // BankMonitor_AddInstance(params, handle, 0x7f, 3); attach the stream.
    c.cycles += 9;
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = 0x7f;
    c.g[A3][0] = 3;
    c.g[A0][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(PLAY_STREAM_PARAMS));
    call_ptr(c, 0x002a_cfe0, 0x0028_58b8)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 9;
        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.g[A1][0] = c.g[S6][0];
        call_ptr(c, 0x002a_b188, 0x0028_58d0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = c.g[FP][0];
        call_ptr(c, 0x002a_bcf8, 0x0028_58e4)?;
    }
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));

    // Pop the level; return the handle.
    c.cycles += 5;
    c.g[V0][0] = c.g[S4][0];
    c.g[A1][0] = c.lw(r32(c, V1));
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(PLAY_STREAM_TOP));
    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
    let v = c.g[A0][0];
    c.sw(r32(c, A1).wrapping_add(PLAY_STREAM_TOP), v);
    Ok(())
}

const PLAY_STREAM_CLOSER_VIEW: Callee = Callee {
    addr: 0x0028_8940,
    f: ee::s07::SSXAudioSystem_IsCloserToSecondView,
};
const PLAY_STREAM_REPLAY_PLAYING: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
};

// ---------------------------------------------------------------------------
// SPCH_ChooseSample
// ---------------------------------------------------------------------------

/// Speech selection set: `+0x61` count of chosen samples, `+0x62` the
/// chosen sample indices (200 at most).
pub(crate) const SPCH_SET_COUNT: u32 = 0x61;
pub(crate) const SPCH_SET_LIST: u32 = 0x62;
pub(crate) const SPCH_SET_MAX: u64 = 200;
/// Condition block: `+4` signed count, `+8` condition ids (0xfe: "mark"
/// slot), `+0x10` mark targets (bit 7 set: mark bit `& 0x7f`).
pub(crate) const SPCH_COND_COUNT: u32 = 0x4;
pub(crate) const SPCH_COND_IDS: u32 = 0x8;
pub(crate) const SPCH_COND_MARKS: u32 = 0x10;
pub(crate) const SPCH_COND_MARK: u64 = 0xfe;

const SPCH_MATCH_CONDITIONS: Callee = Callee {
    addr: 0x003d_85c8,
    f: ee::s14::SPCH_MatchConditions,
};
const SPCH_IS_SAMPLE_ALLOWED: Callee = Callee {
    addr: 0x003d_8780,
    f: ee::s14::SPCH_IsSampleAllowed,
};
const SPCH_HISTORY_FIND: Callee = Callee {
    addr: 0x003d_6e30,
    f: ee::s14::SPCH_HistoryFind,
};

/// `int SPCH_ChooseSample(SelSet* set, Sel* sel, short* event, Cond* cond,
/// u32* tbl)` (0x3d88c8): finds the samples of event `*event` whose
/// conditions hold (`cond`'s ids against the event's mask words and `tbl`),
/// that match and are allowed, appends every one not in the history to
/// `set` (and, when `cond` has mark slots, sets the sample's mark bits in
/// `sel`); if none was new, appends the one played longest ago. Returns the
/// number appended.
pub fn spch_choose_sample(c: &mut Cpu) {
    guest(c, spch_choose_sample_body);
}

fn spch_choose_sample_body(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0xd0);
    c.g[V0][0] = u64::MAX;
    frame.save(c, S7, 0xa0);
    frame.save(c, S5, 0x80);
    frame.save(c, S4, 0x70);
    frame.save(c, S2, 0x50);
    frame.save(c, RA, 0xc0);
    frame.save(c, FP, 0xb0);
    frame.save(c, S6, 0x90);
    frame.save(c, S3, 0x60);
    frame.save(c, S1, 0x40);
    frame.save(c, S0, 0x30);
    c.g[S7][0] = 0;
    c.g[S5][0] = c.g[A1][0];
    c.g[S4][0] = c.g[A3][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[A1][0] = 0;
    c.g[A3][0] = c.lb(r32(c, S4).wrapping_add(SPCH_COND_COUNT));
    // Locals: event, tbl, sample count, best age, best index, entry stride,
    // "cond has mark slots", next entry, entries base.
    let s = sp(c);
    let (event, tbl) = (c.g[A2][0], c.g[T0][0]);
    c.sw(s, event);
    c.sw(s.wrapping_add(4), tbl);
    c.sw(s.wrapping_add(0xc), 0);
    c.sw(s.wrapping_add(0x10), u64::MAX);
    c.sw(s.wrapping_add(0x18), 0);
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1734));

    // Does the condition block have a mark slot (id 0xfe)?
    if (c.g[A3][0] as i64) > 0 {
        c.cycles += 4;
        c.g[V1][0] = c.lbu(r32(c, S4).wrapping_add(SPCH_COND_IDS));
        c.g[V0][0] = SPCH_COND_MARK;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        if c.g[V1][0] == SPCH_COND_MARK {
            c.cycles += 3;
            c.g[V1][0] = 1;
            c.sw(sp(c).wrapping_add(0x18), 1);
        } else {
            loop {
                c.cycles += 3;
                let more = (c.g[A1][0] as i64) < (c.g[A3][0] as i64);
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
                if !more {
                    break;
                }
                c.cycles += 4;
                c.g[A0][0] = SPCH_COND_MARK;
                c.g[V1][0] = c.lbu(r32(c, V0).wrapping_add(SPCH_COND_IDS));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                if c.g[V1][0] == SPCH_COND_MARK {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    c.sw(sp(c).wrapping_add(0x18), 1);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_8948);
                }
            }
        }
    }

    // s3 = the event's sample header; its condition mask words must all
    // overlap the table words of the condition ids.
    c.cycles += 17;
    c.g[V1][0] = c.lw(sp(c));
    c.g[T3][0] = 1;
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lh(r32(c, V1));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    c.g[S3][0] = c.lw(r32(c, V0));
    c.g[A0][0] = c.lbu(r32(c, S3).wrapping_add(5));
    let count = c.g[A0][0];
    c.sw(sp(c).wrapping_add(8), count);
    c.g[V1][0] = c.lbu(r32(c, S3).wrapping_add(4));
    c.g[A3][0] = c.g[V1][0] & 0xf;
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(2));
    c.mult(0, V0, A0, V0);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xf));
    c.g[V0][0] &= 0x3ffc;
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
    if c.g[A3][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[A0][0];
        c.g[T0][0] = sx(r32(c, S4).wrapping_add(SPCH_COND_IDS));
        c.g[T2][0] = 0xff;
        c.g[T1][0] = 0xfe;
        'masks: loop {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
            c.g[V0][0] = c.lbu(r32(c, V0));
            let id = c.g[V0][0];
            if id != 0 {
                c.cycles += 2;
                if id != 0xff {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(sp(c).wrapping_add(4));
                    if id != 0xfe {
                        c.cycles += 7;
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[A0][0] = c.lw(r32(c, A2));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        c.g[A0][0] &= c.g[V1][0];
                        if c.g[A0][0] == 0 {
                            c.g[T3][0] = 0;
                            break 'masks;
                        }
                    }
                    c.cycles += 1;
                }
            }
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.cycles += 3;
            let more = (c.g[A1][0] as i64) < (c.g[A3][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_89c0);
            }
        }
    }

    c.cycles += 2;
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(0xc));
    let tail = if c.g[T3][0] != 0 {
        spch_choose_sample_scan(c)?
    } else {
        true
    };
    if tail {
        c.cycles += 1;
        c.g[V0][0] = c.g[S7][0];
    }
    c.cycles += 12;
    for (reg, off) in [
        (RA, 0xc0),
        (FP, 0xb0),
        (S7, 0xa0),
        (S6, 0x90),
        (S5, 0x80),
        (S4, 0x70),
        (S3, 0x60),
        (S2, 0x50),
        (S1, 0x40),
        (S0, 0x30),
    ] {
        frame.restore(c, reg, off);
    }
    frame.close(c);
    Ok(())
}

/// Appends sample `idx` to the set if there is room; `a0` = appended (0/1).
fn spch_choose_sample_append(c: &mut Cpu, from_stack: bool) {
    c.cycles += 4;
    c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(SPCH_SET_COUNT));
    c.g[V0][0] = (c.g[V0][0] < SPCH_SET_MAX) as u64;
    c.g[A0][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += if from_stack { 8 } else { 7 };
        c.g[V1][0] = c.lbu(r32(c, S2).wrapping_add(SPCH_SET_COUNT));
        c.g[A0][0] = 1;
        let idx = if from_stack {
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
            c.g[V0][0]
        } else {
            c.g[S0][0]
        };
        c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, V1)));
        c.sb(r32(c, V1).wrapping_add(SPCH_SET_LIST), idx);
        c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(SPCH_SET_COUNT));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        let n = c.g[V0][0];
        c.sb(r32(c, S2).wrapping_add(SPCH_SET_COUNT), n);
    }
}

/// Sets the mark bits of the sample entry at `s1`: for each mark slot of
/// the condition block whose target has bit 7, `bits[target & 0x7f] |= 1 <<
/// entry[slot]` (`bits` at `t0`). `a3` = the condition count.
fn spch_choose_sample_mark(c: &mut Cpu, head: u32) {
    c.cycles += 3;
    c.g[T2][0] = 0xfe;
    c.g[T1][0] = 1;
    c.g[A1][0] = sx(r32(c, S4).wrapping_add(SPCH_COND_MARKS));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(r32(c, A1).wrapping_sub(8));
        if c.g[V0][0] == SPCH_COND_MARK {
            c.cycles += 4;
            c.g[A0][0] = c.lbu(r32(c, A1));
            c.g[V0][0] = c.g[A0][0] & 0x80;
            let marked = c.g[V0][0] != 0;
            c.g[A0][0] &= 0x7f;
            if marked {
                c.cycles += 8;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, A2)));
                c.g[A0][0] = sx(r32(c, A0) << 2);
                c.g[V1][0] = c.lbu(r32(c, V0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
                c.g[V0][0] = c.lw(r32(c, A0));
                c.g[V1][0] = sx(r32(c, T1) << (r32(c, V1) & 31));
                c.g[V0][0] |= c.g[V1][0];
                let w = c.g[V0][0];
                c.sw(r32(c, A0), w);
            }
            c.cycles += 1;
        }
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        c.cycles += 3;
        let more = (c.g[A2][0] as i64) < (c.g[A3][0] as i64);
        c.g[V0][0] = more as u64;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The conditions hold: walk the samples, then fall back to the oldest.
/// True when the exit goes through the caller's `+1; v0 = s7` tail.
fn spch_choose_sample_scan(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 9;
    c.g[V0][0] = c.lbu(r32(c, S3).wrapping_add(4));
    c.g[S1][0] = sx(r32(c, S3).wrapping_add(0xe));
    c.g[V0][0] &= 0xf;
    let base = c.g[A0][0];
    c.sw(sp(c).wrapping_add(0x20), base);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
    let stride = c.g[V0][0];
    c.sw(sp(c).wrapping_add(0x14), stride);
    c.g[V0][0] = c.lw(sp(c).wrapping_add(8));
    c.g[S0][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S6][0] = 0xffff_ffff_ffff_fffc;
        loop {
            // MatchConditions(sel, cond, tbl, entry).
            c.cycles += 10;
            c.g[A2][0] = c.lw(sp(c).wrapping_add(4));
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A3][0] = c.g[S1][0];
            SPCH_MATCH_CONDITIONS.call(c, 0x003d_8a54)?;
            c.g[FP][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V1][0] = c.lw(sp(c).wrapping_add(0x14));
            c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
            let next = c.g[V1][0];
            c.sw(sp(c).wrapping_add(0x1c), next);
            // True: the count is already in v1 (skips the `+1` load).
            let loaded = 'sample: {
                if c.g[V0][0] == 0 {
                    break 'sample false;
                }
                // IsSampleAllowed(event, header, i).
                c.cycles += 6;
                c.g[A0][0] = c.lw(sp(c));
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S0][0];
                SPCH_IS_SAMPLE_ALLOWED.call(c, 0x003d_8a78)?;
                let allowed = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if !allowed {
                    break 'sample false;
                }
                // HistoryFind(header, i): > 0 is its age in the history.
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                SPCH_HISTORY_FIND.call(c, 0x003d_8a88)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = c.lw(sp(c).wrapping_add(0xc));
                if (c.g[V1][0] as i64) > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.lw(sp(c).wrapping_add(8));
                        break 'sample true;
                    }
                    // The oldest so far.
                    c.cycles += 3;
                    let (age, i) = (c.g[V1][0], c.g[S0][0]);
                    c.sw(sp(c).wrapping_add(0xc), age);
                    c.sw(sp(c).wrapping_add(0x10), i);
                    break 'sample false;
                }
                // New: append it, then set its mark bits.
                spch_choose_sample_append(c, false);
                c.cycles += 3;
                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x18));
                c.g[S7][0] = sx(r32(c, S7).wrapping_add(r32(c, A0)));
                if c.g[V0][0] == 0 {
                    break 'sample false;
                }
                c.cycles += 14;
                c.g[V0][0] = c.lbu(r32(c, S5).wrapping_add(4));
                c.g[A2][0] = 0;
                c.g[V1][0] = c.lbu(r32(c, S5).wrapping_add(6));
                c.g[V0][0] = sx(r32(c, V0) >> 2);
                c.g[A3][0] = c.lb(r32(c, S4).wrapping_add(SPCH_COND_COUNT));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] &= c.g[S6][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(3));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
                c.g[V1][0] &= c.g[S6][0];
                c.g[V0][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                if (c.g[A3][0] as i64) > 0 {
                    spch_choose_sample_mark(c, 0x003d_8b28);
                }
                false
            };
            if !loaded {
                c.cycles += 1;
                c.g[V1][0] = c.lw(sp(c).wrapping_add(8));
            }
            // s0 = i + 1; continue while s0 < count; s1 = the next entry.
            c.cycles += 4;
            c.g[S0][0] = c.g[FP][0];
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[S1][0] = c.lw(sp(c).wrapping_add(0x1c));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_8a40);
            }
        }
    }
    Ok(spch_choose_sample_fallback(c))
}

/// After the walk: none new but an old one seen: append the oldest (and
/// mark its bits). Returns whether the exit takes the caller's tail.
fn spch_choose_sample_fallback(c: &mut Cpu) -> bool {
    c.cycles += 2;
    c.g[V0][0] = c.g[S7][0];
    if c.g[S7][0] != 0 {
        return false;
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x10));
    let none = c.g[A0][0] == u64::MAX;
    c.g[V0][0] = c.g[S7][0];
    if none {
        return false;
    }
    spch_choose_sample_append(c, true);
    c.cycles += 3;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x18));
    c.g[S7][0] = c.g[A0][0];
    if c.g[V1][0] == 0 {
        return true;
    }
    c.cycles += 18;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x20));
    c.g[A2][0] = 0;
    c.g[V0][0] = c.lbu(r32(c, S5).wrapping_add(4));
    c.lo[0] = c.g[V1][0];
    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x14));
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x10));
    c.g[V0][0] = sx(r32(c, V0) >> 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
    c.g[A3][0] = c.lb(r32(c, S4).wrapping_add(SPCH_COND_COUNT));
    // s1 = base + best * stride (+2 below): the best entry.
    c.madd(0, S1, V1, A0);
    c.g[V0][0] &= 0x7c;
    c.g[V1][0] = c.lbu(r32(c, S5).wrapping_add(6));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
    c.g[V0][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(2));
    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    if (c.g[A3][0] as i64) > 0 {
        spch_choose_sample_mark(c, 0x003d_8c30);
    }
    true
}

// ---------------------------------------------------------------------------
// WScriptMan_MissionCommand
// ---------------------------------------------------------------------------

/// `WScriptMan` fields: `+0` the current context, `+0x1c4` the event queue,
/// `+0x2ac` the mission mode, `+0x2b4`/`+0x2b8` the process lists.
pub(crate) const MISSION_CMD_QUEUE: u32 = 0x1c4;
pub(crate) const MISSION_CMD_MODE: u32 = 0x2ac;
pub(crate) const MISSION_CMD_LIST_B: u32 = 0x2b4;
pub(crate) const MISSION_CMD_LIST_A: u32 = 0x2b8;
/// Mission object fields: `+0x20` its id, `+0x5c` its vtable.
pub(crate) const MISSION_CMD_ID: u32 = 0x20;
pub(crate) const MISSION_CMD_VT: u32 = 0x5c;
/// The two `switch (cmd)` jump tables (19 and 20 entries).
pub(crate) const MISSION_CMD_TABLE1: u32 = 0x0048_9930;
pub(crate) const MISSION_CMD_TABLE2: u32 = 0x0048_9980;

const MISSION_CMD_PROCESS: Callee = Callee {
    addr: 0x0030_a728,
    f: ee::s10::WScriptMan_ProcessCommand,
};
const MISSION_CMD_QUEUE_PUSH: Callee = Callee {
    addr: 0x0030_c7c0,
    f: ee::s10::WScriptMan_EventQueuePush,
};
const MISSION_CMD_CURRENT: Callee = Callee {
    addr: 0x0030_aeb8,
    f: ee::s10::WScriptMan_GetCurrentMission,
};
const MISSION_CMD_FIND: Callee = Callee {
    addr: 0x0030_b388,
    f: ee::s10::WScriptProcessList_FindMissionById,
};
const MISSION_CMD_TYPE2_ACTIVE: Callee = Callee {
    addr: 0x0030_b8c0,
    f: ee::s10::WScriptMan_IsMissionType2Active,
};

/// `int WScriptMan_MissionCommand(WScriptMan* m, int id, int cmd, int ctx)`
/// (0x30a868): mission command `cmd` on mission `id` (-1: the current
/// mission) under context `ctx` (-1: keep). Unless `cmd` is 15 or 19 it
/// needs the mission game mode and an active race. Commands 0-2/18 go to
/// ProcessCommand, 8/9/11 to the event queue (unless the queue is busy);
/// 3/4/5/17 call the mission's start/stop/abort/restart methods; 12-16/19
/// ask the BE mission interface (completed, unlocked, available, ...).
/// The context is put back; returns the answer (0/1).
pub fn wscript_man_mission_command(c: &mut Cpu) {
    guest(c, mission_command_body);
}

fn mission_command_body(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = 0xf;
    frame.save(c, S0, 0x70);
    frame.save(c, S1, 0x60);
    frame.save(c, S2, 0x50);
    frame.save(c, S3, 0x40);
    frame.save(c, S4, 0x30);
    frame.save(c, S5, 0x20);
    frame.save(c, S6, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = 0;
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A2][0];
    c.g[S3][0] = c.g[A0][0];
    c.g[S4][0] = c.g[A3][0];
    // s0 = forced (cmd 15 or 19).
    if c.g[S2][0] != 0xf {
        c.cycles += 3;
        c.g[V0][0] = 0x13;
        if c.g[S2][0] == 0x13 {
            c.cycles += 1;
            c.g[S0][0] = 1;
        }
    } else {
        c.cycles += 1;
        c.g[S0][0] = 1;
    }
    c.cycles += 7;
    call_ptr(c, 0x0014_dc80, 0x0030_a8c0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0030_a8cc)?;
    c.g[V0][0] = u64::MAX;
    'run: {
        if c.g[S0][0] == 0 {
            // Mission mode and a race running, or return 0.
            c.cycles += 5;
            c.g[A0][0] = 4;
            c.g[V1][0] = c.lb(0x0053_5c10);
            c.g[V0][0] = 0;
            if c.g[V1][0] != 4 {
                break 'run;
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x84));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x7c));
            c.g[V0][0] = 0;
            if c.g[A0][0] != 1 {
                break 'run;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        // Switch to the context (the old one in s6).
        c.cycles += 2;
        c.g[S6][0] = c.lw(r32(c, S3));
        if c.g[S4][0] != c.g[V0][0] {
            c.cycles += 1;
            let ctx = c.g[S4][0];
            c.sw(r32(c, S3), ctx);
        }
        mission_command_first(c)?;

        // obj = the current mission; s4 = the BE mission interface.
        c.cycles += 11;
        c.g[S5][0] = 0;
        MISSION_CMD_CURRENT.call(c, 0x0030_a994)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0030_a99c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x0014_dd58, 0x0030_a9a8)?;
        c.g[S4][0] = c.g[V0][0];
        let in_table = c.g[S2][0] < 0x14;
        c.g[V0][0] = 0x49_0000;
        let restore = if in_table {
            c.cycles += 6;
            let target = mission_command_table(c, MISSION_CMD_TABLE2);
            match target {
                0x0030_ac68 => true,
                0x0030_a9d0 => mission_command_method(c, 0x88)?,
                0x0030_aa3c => mission_command_method(c, 0x10)?,
                0x0030_aa70 => mission_command_method(c, 0x98)?,
                0x0030_aac4 => mission_command_method(c, 0xa0)?,
                0x0030_ab20 => {
                    mission_command_ask(c, 0x0015_3e28, 4, 0x0030_ab48, MissionAsk::NonZero)?
                }
                0x0030_ab50 => {
                    mission_command_ask(c, 0x0015_3dd0, 4, 0x0030_ab78, MissionAsk::NonZero)?
                }
                0x0030_ab80 => mission_command_available(c)?,
                0x0030_abd8 => {
                    mission_command_ask(c, 0x0015_3fd8, 5, 0x0030_ac00, MissionAsk::Last)?
                }
                0x0030_ac0c => {
                    mission_command_ask(c, 0x0015_3e80, 4, 0x0030_ac34, MissionAsk::NonZero)?
                }
                0x0030_ac3c => {
                    mission_command_ask(c, 0x0015_3d78, 3, 0x0030_ac64, MissionAsk::NonZero)?
                }
                _ => {
                    c.jump_out(target, 0x0030_a868, 0x0030_ac98);
                    return Ok(());
                }
            }
        } else {
            true
        };
        if restore {
            c.cycles += 1;
            let old = c.g[S6][0];
            c.sw(r32(c, S3), old);
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S5][0];
    }
    c.cycles += 10;
    for (reg, off) in [
        (S0, 0x70),
        (S1, 0x60),
        (S2, 0x50),
        (S3, 0x40),
        (S4, 0x30),
        (S5, 0x20),
        (S6, 0x10),
    ] {
        frame.restore(c, reg, off);
    }
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `a0 = table[cmd]` (the jump target, left in `a0` as the original does).
fn mission_command_table(c: &mut Cpu, table: u32) -> u32 {
    c.g[V1][0] = sx(r32(c, S2) << 2);
    c.g[V0][0] = sx(table);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = c.lw(r32(c, V1));
    r32(c, A0)
}

/// The first switch: ProcessCommand or the event queue.
fn mission_command_first(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    let in_table = c.g[S2][0] < 0x13;
    c.g[V0][0] = 0x49_0000;
    if in_table {
        c.cycles += 6;
        let target = mission_command_table(c, MISSION_CMD_TABLE1);
        match target {
            0x0030_a938 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                MISSION_CMD_PROCESS.call(c, 0x0030_a948)?;
                c.g[A0][0] = c.g[S3][0];
                return Ok(());
            }
            0x0030_a950 => {
                // Queue the command unless the queue is busy (1..9).
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x28));
                c.g[A0][0] = c.lw(r32(c, A0));
                c.g[A1][0] = 0;
                if c.g[A0][0] != 0 {
                    c.cycles += 1;
                    c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[A1][0] != 0 {
                    return Ok(());
                }
                c.cycles += 5;
                c.g[A3][0] = c.lw(r32(c, S3));
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_QUEUE));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                MISSION_CMD_QUEUE_PUSH.call(c, 0x0030_a988)?;
            }
            0x0030_a988 => {}
            _ => {
                c.jump_out(target, 0x0030_a868, 0x0030_ac98);
                return Err(crate::call::Unwound);
            }
        }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S3][0];
    Ok(())
}

/// Commands 3/4/5/17: find mission `id` (in list A, and for 3/17 then list
/// B; -1: the current one) and call its vtable method `slot`; 3 and 5 then
/// tell the sound system. Stores the context back itself (false: the
/// caller's tail does not).
fn mission_command_method(c: &mut Cpu, slot: u32) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    let current = c.g[S1][0] == c.g[V0][0];
    // The vtable already loaded into v0 (3/17's first lookup found it).
    let mut loaded = false;
    match slot {
        0x88 => {
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_A));
            if !current {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                MISSION_CMD_FIND.call(c, 0x0030_a9e8)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_VT));
                    loaded = true;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_B));
                    c.g[A1][0] = c.g[S1][0];
                    MISSION_CMD_FIND.call(c, 0x0030_aa00)?;
                    c.g[S0][0] = c.g[V0][0];
                }
            }
        }
        0xa0 => {
            if !current {
                c.cycles += 6;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_A));
                c.g[A1][0] = c.g[S1][0];
                MISSION_CMD_FIND.call(c, 0x0030_aae0)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_VT));
                    loaded = true;
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_B));
                    MISSION_CMD_FIND.call(c, 0x0030_aaf8)?;
                    c.g[S0][0] = c.g[V0][0];
                }
            }
        }
        _ => {
            c.g[A1][0] = c.g[S1][0];
            if !current {
                c.cycles += 3;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_A));
                MISSION_CMD_FIND.call(
                    c,
                    if slot == 0x10 {
                        0x0030_aa54
                    } else {
                        0x0030_aa88
                    },
                )?;
                c.g[S0][0] = c.g[V0][0];
            }
        }
    }
    if !loaded {
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            let old = c.g[S6][0];
            c.sw(r32(c, S3), old);
            return Ok(false);
        }
        c.cycles += match slot {
            0x10 => 4,
            0x98 => 12,
            _ => 1,
        };
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_VT));
    }
    // obj->method(): a0 = obj + delta.
    let ret = match slot {
        0x88 => {
            c.cycles += 11;
            0x0030_aa20
        }
        0x98 => 0x0030_aaa8,
        _ => {
            if slot == 0xa0 {
                c.cycles += 2;
            }
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(slot));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(slot + 4));
            c.cycles += 4;
            0x0030_ab18
        }
    };
    if slot == 0x88 || slot == 0x98 {
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(slot));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(slot + 4));
    }
    let method = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)?;
    match slot {
        0x88 => {
            // Sound: mission started (id).
            call_ptr(c, 0x0028_b180, 0x0030_aa28)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0029_d6e0, 0x0030_aa34)?;
        }
        0x98 => {
            // Sound: mission aborted.
            call_ptr(c, 0x0028_b180, 0x0030_aab0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0029_dbb0, 0x0030_aabc)?;
        }
        _ => {}
    }
    let old = c.g[S6][0];
    c.sw(r32(c, S3), old);
    Ok(false)
}

/// How a BE mission query's result becomes the answer.
#[derive(Clone, Copy, PartialEq)]
enum MissionAsk {
    /// `answer = result != 0`.
    NonZero,
    /// `answer = result == -1` (no next mission).
    Last,
}

/// `id` (-1: the current mission's id, if there is one); false (and v0
/// = -1, a0 = iface) when there is none.
fn mission_command_resolve_id(c: &mut Cpu) -> bool {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    c.g[A0][0] = c.g[S4][0];
    if c.g[S1][0] != c.g[V0][0] {
        return true;
    }
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_ID));
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S4][0];
    c.g[S1][0] != c.g[V0][0]
}

/// Commands 12/13/14/15/19: `answer = query(iface, id)`.
fn mission_command_ask(
    c: &mut Cpu,
    query: u32,
    cost: u64,
    ret: u32,
    how: MissionAsk,
) -> Result<bool, crate::call::Unwound> {
    if mission_command_resolve_id(c) {
        c.cycles += cost;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, query, ret)?;
        if how == MissionAsk::Last {
            c.g[V0][0] = !c.g[V0][0];
            c.g[S5][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.g[S5][0] = (c.g[V0][0] != 0) as u64;
        }
    }
    Ok(true)
}

/// Command 16: available (not in mode 1, no type-2 mission running).
fn mission_command_available(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    if c.g[S1][0] == c.g[V0][0] {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_ID));
        }
        c.cycles += 2;
        if c.g[S1][0] == c.g[V0][0] {
            let old = c.g[S6][0];
            c.sw(r32(c, S3), old);
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(MISSION_CMD_MODE));
    c.cycles += 3;
    c.g[S0][0] = 1;
    if c.g[V0][0] == c.g[S0][0] {
        let old = c.g[S6][0];
        c.sw(r32(c, S3), old);
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    MISSION_CMD_TYPE2_ACTIVE.call(c, 0x0030_abb8)?;
    if c.g[V0][0] != 0 {
        let old = c.g[S6][0];
        c.sw(r32(c, S3), old);
        return Ok(false);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0015_3ed8, 0x0030_abcc)?;
    c.g[S5][0] = 0;
    if c.g[V0][0] != 0 {
        c.g[S5][0] = c.g[S0][0];
    }
    Ok(true)
}

// ---------------------------------------------------------------------------
// SPCH_BuildSentence
// ---------------------------------------------------------------------------

/// Speech event header: `+0` event id, `+6` part count, `+7` packed
/// table sizes, `+8` signed repeat count, `+0xa` flags (bit 0: always use
/// the bank field, bit 3: argument masks), `+0xc` the part offsets.
pub(crate) const SPCH_EVENT_PARTS: u32 = 0x6;
pub(crate) const SPCH_EVENT_FLAGS: u32 = 0xa;
/// Per-bank tables: `{u16 event, int limit}` (8 bytes), the bank mode words.
pub(crate) const SPCH_BANK_LAST: u32 = 0x0045_0660;
pub(crate) const SPCH_BANK_MODE: u32 = 0x0045_06a0;

const SPCH_FIND_EVENT: Callee = Callee {
    addr: 0x003d_7110,
    f: ee::s14::SPCH_FindEvent,
};
const SPCH_FIND_BANK_USER_DATA: Callee = Callee {
    addr: 0x003d_81f0,
    f: ee::s14::SPCH_FindBankUserData,
};
const SPCH_FIND_BANK_FIELD14: Callee = Callee {
    addr: 0x003d_82b0,
    f: ee::s14::SPCH_FindBankField14,
};
const SPCH_EVAL_PART_CHOICE: Callee = Callee {
    addr: 0x003d_b208,
    f: ee::s14::SPCH_EvalPartChoice,
};
const SPCH_PICK_WEIGHTED: Callee = Callee {
    addr: 0x003d_8cb0,
    f: ee::s14::SPCH_PickWeighted,
};
const SPCH_RANDOM_NO_REPEAT: Callee = Callee {
    addr: 0x003d_b5c0,
    f: ee::s14::SPCH_RandomNoRepeat,
};
const SPCH_CHECK_EVENT_MASKS: Callee = Callee {
    addr: 0x003d_9ed0,
    f: ee::s14::SPCH_CheckEventMasks,
};
const SPCH_CHOOSE_EVENT_SAMPLES: Callee = Callee {
    addr: 0x003d_9088,
    f: ee::s14::SPCH_ChooseEventSamples,
};
const SPCH_RESOLVE_EVENT_PARTS: Callee = Callee {
    addr: 0x003d_95b8,
    f: ee::s14::SPCH_ResolveEventParts,
};
const SPCH_SENTENCE_APPEND: Callee = Callee {
    addr: 0x003d_9ac8,
    f: ee::s14::SPCH_SentenceAppend,
};

/// `int SPCH_BuildSentence(int* args)` (0x3d9fb0): finds the speech event
/// `args[0]` and its bank; unless the bank's last event repeats past its
/// limit, evaluates the argument masks (20 two-bit tests against
/// `args[1..]`), orders the event's parts by weight, and tries them in that
/// order: a part that passes its chance roll and masks gets its samples
/// chosen and resolved, then the sentence appended. Returns the first
/// resolution (0: none; -1: the event has no parts).
pub fn spch_build_sentence(c: &mut Cpu) {
    guest(c, build_sentence_body);
}

fn build_sentence_body(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x280);
    frame.save(c, FP, 0x260);
    frame.save(c, S2, 0x200);
    frame.save(c, S1, 0x1f0);
    frame.save(c, RA, 0x270);
    frame.save(c, S7, 0x250);
    frame.save(c, S6, 0x240);
    frame.save(c, S5, 0x230);
    frame.save(c, S4, 0x220);
    frame.save(c, S3, 0x210);
    frame.save(c, S0, 0x1e0);
    c.g[FP][0] = c.g[A0][0];
    c.g[A0][0] = sx(sp(c).wrapping_add(0x1a0));
    // key = { args[0] }, part choice flag = 0.
    c.sb(sp(c).wrapping_add(0x1b0), 0);
    let args = r32(c, FP);
    c.lwl(V0, args.wrapping_add(3));
    c.lwr(V0, args);
    let s = sp(c);
    c.swl(V0, s.wrapping_add(0x1a3));
    c.swr(V0, s.wrapping_add(0x1a0));
    c.g[S1][0] = 0;
    SPCH_FIND_EVENT.call(c, 0x003d_a000)?;
    c.g[S2][0] = c.g[V0][0];
    let s = sp(c);
    c.sw(s.wrapping_add(0x1bc), 0);
    c.sw(s.wrapping_add(0x1c4), 0);
    c.sw(s.wrapping_add(0x1cc), 0);
    'out: {
        if c.g[S2][0] == 0 {
            break 'out;
        }
        c.cycles += 5;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x1a0));
        c.g[A1][0] = sx(sp(c).wrapping_add(0x1b4));
        SPCH_FIND_BANK_USER_DATA.call(c, 0x003d_a020)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(sp(c).wrapping_add(0x1b4));
        if !found {
            break 'out;
        }
        // Repeating the bank's last event within its limit: nothing.
        c.cycles += 9;
        c.g[V0][0] = 0x45_0000;
        c.g[T0][0] = sx(SPCH_BANK_LAST);
        c.g[A1][0] = c.lhu(r32(c, S2));
        c.g[A3][0] = sx(r32(c, A0) << 3);
        c.g[T1][0] = c.g[A0][0];
        c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, T0)));
        c.g[V0][0] = c.lhu(r32(c, V1));
        c.g[A2][0] = 1;
        if c.g[A1][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = c.lb(r32(c, S2).wrapping_add(8));
            c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, A3)));
            if (c.g[A0][0] as i64) > 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(4));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = 0;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x45_0000;
        if c.g[A2][0] == 0 {
            break 'out;
        }
        // s6 = the bank's mode (mode 1 only for events with flag bit 0).
        c.cycles += 7;
        c.g[V1][0] = sx(r32(c, T1) << 2);
        c.g[V0][0] = sx(SPCH_BANK_MODE);
        c.g[A0][0] = c.lbu(r32(c, S2).wrapping_add(SPCH_EVENT_FLAGS));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] &= 1;
        c.g[S6][0] = c.lw(r32(c, V1));
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[S6][0] ^ 1;
            if c.g[V0][0] == 0 {
                c.g[S6][0] = 0;
            }
        }
        c.cycles += 2;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x1a0));
        if c.g[S6][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x1a0));
            SPCH_FIND_BANK_FIELD14.call(c, 0x003d_a0a0)?;
            let v = c.g[V0][0];
            c.sw(sp(c).wrapping_add(0x1cc), v);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x1a0));
        }
        // Part choice, then the parts ordered by weight at sp+0.
        c.cycles += 11;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[FP][0];
        c.g[A3][0] = sx(sp(c).wrapping_add(0x1b0));
        SPCH_EVAL_PART_CHOICE.call(c, 0x003d_a0b8)?;
        let v = c.g[V0][0];
        c.sw(sp(c).wrapping_add(0x1c0), v);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        SPCH_PICK_WEIGHTED.call(c, 0x003d_a0c8)?;
        c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(SPCH_EVENT_PARTS));
        let parts = c.g[V0][0];
        c.sw(sp(c).wrapping_add(0x1b8), parts);
        if (parts as i64) <= 0 {
            c.cycles += 3;
            c.g[V1][0] = u64::MAX;
            c.sw(sp(c).wrapping_add(0x1bc), u64::MAX);
            break 'out;
        }
        // The argument mask (flag bit 3): bit i set when test i passes.
        c.cycles += 6;
        c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(SPCH_EVENT_FLAGS));
        c.g[T4][0] = 1;
        c.g[V0][0] = sx(r32(c, V0) >> 3);
        c.g[V0][0] &= 1;
        let masks = c.g[V0][0];
        c.sw(sp(c).wrapping_add(0x1c8), masks);
        if masks != 0 {
            build_sentence_arg_mask(c);
        }
        c.cycles += 2;
        c.sw(sp(c).wrapping_add(0x1bc), 0);
        if c.g[T4][0] == 0 {
            break 'out;
        }
        build_sentence_parts(c)?;
    }
    c.cycles += 13;
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x1bc));
    for (reg, off) in [
        (RA, 0x270),
        (FP, 0x260),
        (S7, 0x250),
        (S6, 0x240),
        (S5, 0x230),
        (S4, 0x220),
        (S3, 0x210),
        (S2, 0x200),
        (S1, 0x1f0),
        (S0, 0x1e0),
    ] {
        frame.restore(c, reg, off);
    }
    frame.close(c);
    Ok(())
}

/// The argument mask: test i (2 bits of the event's mask word) is 1:
/// `args[i + 1] != 0`, 2: `args[i + 1] != -1`; the result at sp+0x1c4.
fn build_sentence_arg_mask(c: &mut Cpu) {
    c.cycles += 28;
    c.g[A1][0] = c.lw(sp(c).wrapping_add(0x1b8));
    c.g[A2][0] = 0xffff_ffff_ffff_fffc;
    c.g[V1][0] = c.lbu(r32(c, S2).wrapping_add(7));
    c.g[T1][0] = 1;
    c.g[A0][0] = sx(r32(c, A1) << 1);
    c.g[T2][0] = 0;
    c.g[A1][0] = sx(r32(c, V1) << 24);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(3));
    c.g[V1][0] &= 0xf;
    c.g[A1][0] = sx(((r32(c, A1) as i32) >> 28) as u32);
    c.g[V0][0] = sx(r32(c, V1) << 1);
    c.g[A0][0] &= 0x3fc;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
    c.g[V1][0] = sx(r32(c, A1) << 1);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] &= c.g[A2][0];
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(3));
    c.g[V1][0] &= c.g[A2][0];
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    c.g[A3][0] = 0;
    c.g[A0][0] = c.lw(r32(c, A0));
    c.g[T0][0] = 1;
    c.g[T3][0] = 2;
    c.g[A2][0] = sx(r32(c, FP).wrapping_add(4));
    loop {
        c.cycles += 6;
        c.g[V0][0] = sx(r32(c, A3) << 1);
        c.g[A1][0] = c.lw(r32(c, A2));
        c.g[V0][0] = sx(r32(c, A0) >> (r32(c, V0) & 31));
        c.g[V0][0] &= 3;
        c.g[V1][0] = 0;
        if c.g[V0][0] == c.g[T0][0] {
            c.cycles += 3;
            c.g[V1][0] = c.g[T0][0];
            if c.g[A1][0] == 0 {
                c.g[V1][0] = 0;
            }
        } else {
            c.cycles += 2;
            let two = c.g[V0][0] == c.g[T3][0];
            c.g[V0][0] = !c.g[A1][0];
            c.cycles += 2;
            if two {
                c.g[V1][0] = c.g[T0][0];
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0;
                }
            }
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.g[T2][0] |= c.g[T1][0];
        }
        c.cycles += 6;
        c.g[V0][0] = sx(r32(c, T1) << 1);
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
        c.g[T1][0] = c.g[V0][0] & 0xfffe;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 20) as u64;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_a168);
        }
    }
    c.cycles += 1;
    let mask = c.g[T2][0];
    c.sw(sp(c).wrapping_add(0x1c4), mask);
}

/// Tries the parts in weight order until one resolves (sp+0x1bc != 0).
fn build_sentence_parts(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A2][0] = sx(r32(c, S2).wrapping_add(0xc));
    c.g[V0][0] = sx(sp(c).wrapping_add(0x70));
    let (offs, set) = (c.g[A2][0], c.g[V0][0]);
    c.sw(sp(c).wrapping_add(0x1d0), offs);
    c.sw(sp(c).wrapping_add(0x1d4), set);
    c.g[V0][0] = sx(sp(c).wrapping_add(r32(c, S1)));
    loop {
        // s3 = the next part index (in range, or done).
        c.cycles += 3;
        c.g[S3][0] = c.lbu(r32(c, V0));
        c.g[S0][0] = 1;
        c.cycles += 4;
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x1b8));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x1d0));
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        // s1 = the part (event + 4 offs[s3]); roll its chance.
        c.cycles += 15;
        c.g[S4][0] = sx(r32(c, S3) << 1);
        c.g[S7][0] = sx(r32(c, S1).wrapping_add(1));
        c.g[A0][0] = 0x64;
        c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lhu(r32(c, V1));
        c.g[A1][0] = u64::MAX;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[S1][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        SPCH_RANDOM_NO_REPEAT.call(c, 0x003d_a228)?;
        'part: {
            c.g[A0][0] = c.lbu(r32(c, S1).wrapping_add(1));
            c.g[V1][0] = c.g[S0][0];
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0;
            }
            c.g[A0][0] = c.g[S2][0];
            if c.g[V1][0] == 0 {
                break 'part;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[FP][0];
            SPCH_CHECK_EVENT_MASKS.call(c, 0x003d_a24c)?;
            let ok = c.g[V0][0] != 0;
            c.g[A2][0] = c.lw(sp(c).wrapping_add(0x1c8));
            if !ok {
                break 'part;
            }
            // With argument masks, the part's mask must equal ours.
            c.cycles += 2;
            let masks = c.g[A2][0] != 0;
            c.g[A2][0] = c.lw(sp(c).wrapping_add(0x1c4));
            if masks {
                c.cycles += 25;
                c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(7));
                c.g[V1][0] = c.lbu(r32(c, S2).wrapping_add(6));
                c.g[A1][0] = sx(r32(c, V0) << 24);
                c.g[V0][0] &= 0xf;
                c.g[A1][0] = sx(((r32(c, A1) as i32) >> 28) as u32);
                c.g[A0][0] = sx(r32(c, V0) << 1);
                c.g[V1][0] = sx(r32(c, V1) << 1);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(3));
                c.g[V0][0] = sx(r32(c, A1) << 1);
                c.g[V1][0] &= 0x3fc;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xc));
                c.g[A1][0] = 0xffff_ffff_ffff_fffc;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(3));
                c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, V1)));
                c.g[A0][0] &= c.g[A1][0];
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] &= c.g[A1][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S4)));
                c.g[V0][0] = c.lhu(r32(c, V1).wrapping_add(4));
                c.g[V0][0] ^= c.g[A2][0];
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = 0;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.lw(sp(c).wrapping_add(0x1d4));
            if c.g[S0][0] == 0 {
                break 'part;
            }
            // Clear the choice set (twelve 8-byte slots' flags and its
            // counts), choose the samples.
            c.cycles += 6;
            c.g[S0][0] = sx(sp(c).wrapping_add(0x70));
            let s = sp(c);
            c.sb(s.wrapping_add(0xd1), 0);
            c.g[V1][0] = 0xb;
            c.sb(s.wrapping_add(0xd0), 0);
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x5f));
            loop {
                c.cycles += 7;
                c.sb(r32(c, V0), 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_a2e0);
                }
            }
            c.cycles += 9;
            c.g[T0][0] = c.lbu(sp(c).wrapping_add(0x1b0));
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = c.lw(sp(c).wrapping_add(0x1c0));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[FP][0];
            c.g[T1][0] = c.g[S6][0];
            SPCH_CHOOSE_EVENT_SAMPLES.call(c, 0x003d_a318)?;
            let chosen = (c.g[V0][0] as i64) > 0;
            c.g[T0][0] = c.lw(sp(c).wrapping_add(0x1cc));
            if !chosen {
                break 'part;
            }
            c.cycles += 7;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x1a0));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S6][0];
            SPCH_RESOLVE_EVENT_PARTS.call(c, 0x003d_a334)?;
            let resolved = c.g[V0][0];
            c.sw(sp(c).wrapping_add(0x1bc), resolved);
            if resolved == 0 {
                break 'part;
            }
            c.cycles += 7;
            c.g[A1][0] = c.lw(sp(c).wrapping_add(0x1b4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[T1][0] = c.g[FP][0];
            SPCH_SENTENCE_APPEND.call(c, 0x003d_a358)?;
        }
        c.cycles += 3;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x1bc));
        c.g[S1][0] = c.g[S7][0];
        if c.g[A1][0] != 0 {
            return Ok(());
        }
        c.cycles += 4;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x1b8));
        c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[A2][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(sp(c).wrapping_add(r32(c, S1)));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_a1e8);
        }
    }
}

// ---------------------------------------------------------------------------
// SPCH_ChooseEventSamples
// ---------------------------------------------------------------------------

/// Speech choice set: `+0x60` the part's slot count; slots of 8 bytes from
/// `+0`: `+0` bank (-1: none), `+2` sample, `+4` set start, `+5` length,
/// `+7` done flag.
pub(crate) const SPCH_CHOICE_SLOTS: u32 = 0x60;

const SPCH_MATCH_FLAGS: Callee = Callee {
    addr: 0x003d_b3b8,
    f: ee::s14::SPCH_MatchFlags,
};
const SPCH_FIND_BANK_BY_ID: Callee = Callee {
    addr: 0x003d_6d50,
    f: ee::s14::SPCH_FindBankById,
};
const SPCH_FIND_BANK: Callee = Callee {
    addr: 0x003d_6db8,
    f: ee::s14::SPCH_FindBank,
};
const SPCH_BANK_HAS_SAMPLE: Callee = Callee {
    addr: 0x003d_6cf8,
    f: ee::s14::SPCH_BankHasSample,
};
const SPCH_CHOOSE_SAMPLE: Callee = Callee {
    addr: 0x003d_88c8,
    f: ee::s14::SPCH_ChooseSample,
};

/// `int SPCH_ChooseEventSamples(SelSet* set, u8* part, int* args, u8 a,
/// u8 b, int mode)` (0x3d9088): if the part has fewer than 13 slots, its
/// mode bits (`part[4] & 3`) suit `mode` and its flags match `(a, b)`:
/// clears the part's mark words and, up to 200 rounds, resolves each slot
/// whose conditions hold (its bank by id, by argument or by argument with a
/// sample check) and chooses its samples (SPCH_ChooseSample). Returns 1
/// when every slot resolved, else 0. Dispatcher-shaped like the original
/// (its loops can unwind and resume at their heads).
pub fn spch_choose_event_samples(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003d_9088 => {
                    c.cycles += 22;
                    let frame = Frame::open(c, 0xb0);
                    frame.save(c, FP, 0x90);
                    frame.save(c, S7, 0x80);
                    frame.save(c, S6, 0x70);
                    frame.save(c, S4, 0x50);
                    frame.save(c, RA, 0xa0);
                    frame.save(c, S5, 0x60);
                    frame.save(c, S3, 0x40);
                    frame.save(c, S2, 0x30);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S0, 0x10);
                    c.g[S7][0] = c.g[A0][0];
                    c.g[S4][0] = c.g[A1][0];
                    c.g[S6][0] = c.g[A2][0];
                    // Locals: ret = 1 (sp+0), tries = 200 (sp+8).
                    c.g[V1][0] = c.lbu(r32(c, S4).wrapping_add(4));
                    c.sw(sp(c), 1);
                    c.g[V0][0] = 0xc8;
                    c.g[FP][0] = sx(r32(c, V1) >> 2);
                    c.sw(sp(c).wrapping_add(8), 0xc8);
                    c.g[V0][0] = ((c.g[FP][0] as i64) < 13) as u64;
                    pc = if c.g[V0][0] == 0 {
                        0x003d_9128
                    } else {
                        0x003d_90e0
                    };
                }
                // Do the part's mode bits (part[4] & 3) suit `mode` (t1)?
                0x003d_90e0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(sp(c));
                    c.g[V1][0] &= 3;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 2;
                    pc = if t { 0x003d_90f8 } else { 0x003d_90f0 };
                }
                0x003d_90f0 => {
                    c.cycles += 2;
                    if c.g[T1][0] == c.g[V0][0] {
                        c.g[V0][0] = 1;
                        pc = 0x003d_910c;
                    } else {
                        pc = 0x003d_90f8;
                    }
                }
                0x003d_90f8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                    pc = if t { 0x003d_910c } else { 0x003d_9100 };
                }
                0x003d_9100 => {
                    c.cycles += 2;
                    let t = c.g[T1][0] != c.g[V1][0];
                    c.g[V0][0] = 1;
                    pc = if t { 0x003d_910c } else { 0x003d_9108 };
                }
                0x003d_9108 => {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                    pc = 0x003d_910c;
                }
                0x003d_910c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[A3][0] & 0xff;
                    pc = if t { 0x003d_9128 } else { 0x003d_9114 };
                }
                // SPCH_MatchFlags(a, b, part).
                0x003d_9114 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[T0][0] & 0xff;
                    c.g[A2][0] = c.g[S4][0];
                    SPCH_MATCH_FLAGS.call(c, 0x003d_9120)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(r32(c, S4).wrapping_add(4));
                        pc = 0x003d_9130;
                    } else {
                        pc = 0x003d_9128;
                    }
                }
                // ret = 0.
                0x003d_9128 => {
                    c.cycles += 2;
                    c.sw(sp(c), 0);
                    pc = 0x003d_938c;
                }
                // Clear the part's mark words.
                0x003d_9130 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0;
                    c.g[V1][0] = c.lbu(r32(c, S4).wrapping_add(6));
                    c.g[V0][0] = sx(r32(c, V0) >> 2);
                    c.g[A0][0] = c.lbu(r32(c, S4).wrapping_add(5));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] &= 0x7c;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
                    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                    let t = c.g[A0][0] == 0;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    if t {
                        pc = 0x003d_917c;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = c.g[V0][0];
                        pc = 0x003d_9160;
                    }
                }
                0x003d_9160 => {
                    c.cycles += 7;
                    c.sw(r32(c, V1), 0);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    c.g[V0][0] = c.lbu(r32(c, S4).wrapping_add(5));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                    if more {
                        if back_edge_unwinds(c, 0x003d_9160) {
                            return Ok(());
                        }
                        pc = 0x003d_9160;
                    } else {
                        pc = 0x003d_917c;
                    }
                }
                0x003d_917c => {
                    c.cycles += 3;
                    let n = c.g[FP][0];
                    c.sb(r32(c, S7).wrapping_add(SPCH_CHOICE_SLOTS), n);
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(8));
                    c.g[V1][0] = 1;
                    pc = 0x003d_9188;
                }
                // A round: all = 1 (sp+4), tries-- (sp+8), slot i = 0.
                0x003d_9188 => {
                    c.cycles += 5;
                    let all = c.g[V1][0];
                    c.sw(sp(c).wrapping_add(4), all);
                    c.g[S5][0] = 0;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                    let t = c.g[FP][0] == 0;
                    let tries = c.g[V0][0];
                    c.sw(sp(c).wrapping_add(8), tries);
                    if t {
                        pc = 0x003d_9368;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = sx(r32(c, S4).wrapping_add(8));
                        c.g[S2][0] = c.g[S7][0];
                        let list = c.g[V1][0];
                        c.sw(sp(c).wrapping_add(0xc), list);
                        pc = 0x003d_91a8;
                    }
                }
                // Slot i: s3 = the condition (part + 4 list[i]); check its
                // mark conditions (t1 = holds).
                0x003d_91a8 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
                    c.g[T1][0] = 1;
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                    c.g[V0][0] = c.lbu(r32(c, V1));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[S3][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                    c.g[T0][0] = c.lb(r32(c, S3).wrapping_add(4));
                    let t = (c.g[T0][0] as i64) <= 0;
                    c.g[A3][0] = 0;
                    if t {
                        pc = 0x003d_9258;
                    } else {
                        c.cycles += 5;
                        c.g[A2][0] = 0xffff_ffff_ffff_fffc;
                        c.g[T2][0] = 0xff;
                        c.g[A1][0] = sx(r32(c, S3).wrapping_add(0x10));
                        pc = 0x003d_91e0;
                    }
                }
                // Condition k: id 0xff with a marked target (bit 7) needs
                // that mark word of the selection to be set.
                0x003d_91e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(r32(c, A1).wrapping_sub(8));
                    if c.g[V0][0] != c.g[T2][0] {
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                        pc = 0x003d_924c;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lbu(r32(c, A1));
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[A0][0] & 0x80;
                    if t {
                        pc = 0x003d_9248;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                        pc = 0x003d_924c;
                        continue;
                    }
                    c.cycles += 13;
                    c.g[V0][0] = c.lbu(r32(c, S4).wrapping_add(4));
                    c.g[V1][0] = c.lbu(r32(c, S4).wrapping_add(6));
                    c.g[V0][0] = sx(r32(c, V0) >> 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] &= c.g[A2][0];
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(3));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
                    c.g[V1][0] &= c.g[A2][0];
                    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    let t = c.g[V1][0] == 0;
                    c.g[A0][0] &= 0x7f;
                    if t {
                        pc = 0x003d_9248;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(r32(c, A0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] == 0 {
                        c.g[T1][0] = 0;
                        pc = 0x003d_9258;
                    } else {
                        pc = 0x003d_9248;
                    }
                }
                0x003d_9248 => {
                    c.cycles += 1;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    pc = 0x003d_924c;
                }
                0x003d_924c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    if more {
                        if back_edge_unwinds(c, 0x003d_91e0) {
                            return Ok(());
                        }
                        pc = 0x003d_91e0;
                    } else {
                        pc = 0x003d_9258;
                    }
                }
                0x003d_9258 => {
                    c.cycles += 2;
                    if c.g[T1][0] == 0 {
                        // Not yet: all = 0.
                        c.cycles += 2;
                        c.sw(sp(c).wrapping_add(4), 0);
                        pc = 0x003d_9358;
                        continue;
                    }
                    c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(7));
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                        pc = 0x003d_935c;
                        continue;
                    }
                    // Resolve the slot: start = set count, bank/sample = -1;
                    // kind (cond+3): 0 bank by id, 1 bank of argument
                    // cond+2, 2 bank by id holding the argument's sample.
                    c.cycles += 10;
                    c.g[V0][0] = c.lbu(r32(c, S7).wrapping_add(0x61));
                    c.g[A2][0] = u64::MAX;
                    c.g[V1][0] = 1;
                    let start = c.g[V0][0];
                    c.sb(r32(c, S2).wrapping_add(4), start);
                    c.g[A0][0] = c.lhu(r32(c, S3));
                    c.g[A1][0] = c.lbu(r32(c, S3).wrapping_add(3));
                    c.g[S0][0] = c.lbu(r32(c, S3).wrapping_add(2));
                    c.sh(r32(c, S2).wrapping_add(2), u64::MAX);
                    c.sh(r32(c, S2), u64::MAX);
                    if c.g[A1][0] == c.g[V1][0] {
                        // Bank of argument s0 (< 20), else none.
                        c.cycles += 3;
                        let ok = c.g[S0][0] < 0x14;
                        c.g[V0][0] = sx(r32(c, S0) << 2);
                        if ok {
                            c.cycles += 5;
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                            c.g[A1][0] = c.lhu(r32(c, V0));
                            SPCH_FIND_BANK.call(c, 0x003d_92e0)?;
                            let bank = c.g[V0][0];
                            c.sh(r32(c, S2), bank);
                        } else {
                            c.cycles += 2;
                            c.sh(r32(c, S2), u64::MAX);
                        }
                        pc = 0x003d_931c;
                        continue;
                    }
                    c.cycles += 2;
                    let kind0 = c.g[A1][0] == 0;
                    c.g[V0][0] = 2;
                    if kind0 {
                        c.cycles += 4;
                        SPCH_FIND_BANK_BY_ID.call(c, 0x003d_92b8)?;
                        let bank = c.g[V0][0];
                        c.sh(r32(c, S2), bank);
                        pc = 0x003d_931c;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[A1][0] != c.g[V0][0] {
                        c.cycles += 2;
                        c.g[V1][0] = c.lh(r32(c, S2));
                        pc = 0x003d_9320;
                        continue;
                    }
                    // Kind 2: the bank, if it holds sample args[s0].
                    c.cycles += 10;
                    SPCH_FIND_BANK_BY_ID.call(c, 0x003d_92f0)?;
                    c.g[V1][0] = sx(r32(c, S0) << 2);
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, S6)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.lw(r32(c, S0));
                    SPCH_BANK_HAS_SAMPLE.call(c, 0x003d_9308)?;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = u64::MAX;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lhu(r32(c, S0));
                        let sample = c.g[V0][0];
                        c.sh(r32(c, S2).wrapping_add(2), sample);
                    }
                    c.cycles += 1;
                    let bank = c.g[S1][0];
                    c.sh(r32(c, S2), bank);
                    pc = 0x003d_931c;
                }
                0x003d_931c => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lh(r32(c, S2));
                    pc = 0x003d_9320;
                }
                // No bank: ret = 0. Else choose the slot's samples.
                0x003d_9320 => {
                    c.cycles += 4;
                    c.g[V0][0] = u64::MAX;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[S0][0] = 1;
                    if none {
                        if back_edge_unwinds(c, 0x003d_9128) {
                            return Ok(());
                        }
                        pc = 0x003d_9128;
                        continue;
                    }
                    c.cycles += 8;
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[T0][0] = c.g[S6][0];
                    SPCH_CHOOSE_SAMPLE.call(c, 0x003d_9348)?;
                    if c.g[V0][0] == 0 {
                        c.sw(sp(c), 0);
                        pc = 0x003d_938c;
                        continue;
                    }
                    c.cycles += 2;
                    let len = c.g[V0][0];
                    c.sb(r32(c, S2).wrapping_add(5), len);
                    let done = c.g[S0][0];
                    c.sb(r32(c, S2).wrapping_add(7), done);
                    pc = 0x003d_9358;
                }
                0x003d_9358 => {
                    c.cycles += 1;
                    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                    pc = 0x003d_935c;
                }
                0x003d_935c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[FP][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(8));
                    if more {
                        if back_edge_unwinds(c, 0x003d_91a8) {
                            return Ok(());
                        }
                        pc = 0x003d_91a8;
                    } else {
                        pc = 0x003d_9368;
                    }
                }
                // Another round while some slot waits and tries are left;
                // out of tries: ret = 0.
                0x003d_9368 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(sp(c).wrapping_add(4));
                    let all = c.g[V1][0] != 0;
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(8));
                    if !all {
                        c.cycles += 2;
                        let again = (c.g[V0][0] as i64) > 0;
                        c.g[V1][0] = 1;
                        if again {
                            if back_edge_unwinds(c, 0x003d_9188) {
                                return Ok(());
                            }
                            pc = 0x003d_9188;
                            continue;
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(sp(c));
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0;
                    }
                    let ret = c.g[V1][0];
                    c.sw(sp(c), ret);
                    pc = 0x003d_938c;
                }
                0x003d_938c => {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw(sp(c));
                    let frame = Frame { size: 0xb0 };
                    for (reg, off) in [
                        (RA, 0xa0),
                        (FP, 0x90),
                        (S7, 0x80),
                        (S6, 0x70),
                        (S5, 0x60),
                        (S4, 0x50),
                        (S3, 0x40),
                        (S2, 0x30),
                        (S1, 0x20),
                        (S0, 0x10),
                    ] {
                        frame.restore(c, reg, off);
                    }
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_9088, 0x003d_93c0);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SPCH_PlaySentence
// ---------------------------------------------------------------------------

/// The built sentence (0x450700): `+0` its bank, `+4` part count; per part
/// (stride 0x5c): `+0x194` flags byte, `+0x18c` event, `+0x190` part
/// header, `+0x198` its 4-byte key; the chosen slots `{s16 bank, u16
/// sample}` at `+0xc` (stride 8).
pub(crate) const SPCH_SENTENCE: u32 = 0x0045_0700;
pub(crate) const SPCH_SENTENCE_PART: u32 = 0x5c;
/// The play callback (`0x44ff98`).
pub(crate) const SPCH_PLAY_CALLBACK: u32 = 0x0044_ff98;

const SPCH_NOTIFY_PART_CALLBACK: Callee = Callee {
    addr: 0x003d_b040,
    f: ee::s14::SPCH_NotifyPartCallback,
};
const SPCH_FIND_BANK_TYPE: Callee = Callee {
    addr: 0x003d_8330,
    f: ee::s14::SPCH_FindBankType,
};
const SPCH_GET_EVENT_PARAM: Callee = Callee {
    addr: 0x003d_b3e0,
    f: ee::s14::SPCH_GetEventParam,
};
const SPCH_DECODE_SAMPLE_ENTRY: Callee = Callee {
    addr: 0x003d_b838,
    f: ee::s14::SPCH_DecodeSampleEntry,
};
const SPCH_NOTIFY_PLAY_CALLBACK: Callee = Callee {
    addr: 0x003d_9830,
    f: ee::s14::SPCH_NotifyPlayCallback,
};
const SPCH_RESET_SENTENCE: Callee = Callee {
    addr: 0x003d_9a40,
    f: ee::s14::SPCH_ResetSentence,
};

/// `int SPCH_PlaySentence(int bank)` (0x3d9bd8): if the built sentence
/// belongs to `bank` and has parts, plays each part: notifies the part
/// callback; with a play callback set, decodes each chosen sample of each
/// slot (up to 4 per slot) and hands it to the callback; notifies the play
/// callback; the first part also records the bank's last event (counting
/// repeats). Resets the sentence and returns the number of slots played.
pub fn spch_play_sentence(c: &mut Cpu) {
    guest(c, play_sentence_body);
}

fn play_sentence_body(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0xf0);
    frame.save(c, S1, 0x60);
    frame.save(c, RA, 0xe0);
    frame.save(c, FP, 0xd0);
    frame.save(c, S7, 0xc0);
    frame.save(c, S6, 0xb0);
    frame.save(c, S5, 0xa0);
    frame.save(c, S4, 0x90);
    frame.save(c, S3, 0x80);
    frame.save(c, S2, 0x70);
    frame.save(c, S0, 0x50);
    c.g[S1][0] = sx(SPCH_SENTENCE);
    c.g[V1][0] = c.lw(SPCH_SENTENCE);
    let bank = c.g[A0][0];
    c.sw(sp(c).wrapping_add(0x24), bank);
    c.g[V0][0] = 0;
    // Locals: played slots (sp+0x28), part count (+0x2c), part i (+0x30),
    // key (sp+0, also the callbacks' argument: +0x34).
    c.sw(sp(c).wrapping_add(0x28), 0);
    if c.g[V1][0] == c.g[A0][0] {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.g[V0][0] = (0 < (c.g[V0][0] as i64)) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.sw(sp(c).wrapping_add(0x30), 0);
        let parts = c.g[V0][0];
        c.sw(sp(c).wrapping_add(0x2c), parts);
        if (parts as i64) > 0 {
            c.cycles += 2;
            let s = sp(c);
            c.sw(s.wrapping_add(0x34), s as u64);
            loop {
                play_sentence_part(c)?;
                c.cycles += 5;
                c.g[T0][0] = c.lw(sp(c).wrapping_add(0x40));
                c.g[T1][0] = c.lw(sp(c).wrapping_add(0x2c));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                let next = c.g[T0][0];
                c.sw(sp(c).wrapping_add(0x30), next);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_9c48);
                }
            }
        }
    }
    c.cycles += 15;
    SPCH_RESET_SENTENCE.call(c, 0x003d_9e98)?;
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x28));
    for (reg, off) in [
        (RA, 0xe0),
        (FP, 0xd0),
        (S7, 0xc0),
        (S6, 0xb0),
        (S5, 0xa0),
        (S4, 0x90),
        (S3, 0x80),
        (S2, 0x70),
        (S1, 0x60),
        (S0, 0x50),
    ] {
        frame.restore(c, reg, off);
    }
    frame.close(c);
    Ok(())
}

/// Part `i` (sp+0x30); `i + 1` left at sp+0x40.
fn play_sentence_part(c: &mut Cpu) -> Flow {
    c.cycles += 32;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x30));
    c.g[A0][0] = SPCH_SENTENCE_PART as u64;
    c.g[A1][0] = 0x45_0000;
    c.g[T0][0] = 0x45_0000;
    c.mult(0, S0, V1, A0);
    c.g[A1][0] = sx(SPCH_SENTENCE + 0x198);
    c.g[A3][0] = sx(r32(c, V1).wrapping_add(1));
    c.g[T0][0] = sx(SPCH_SENTENCE);
    let next = c.g[A3][0];
    c.sw(sp(c).wrapping_add(0x40), next);
    c.g[S1][0] = sx(r32(c, A1).wrapping_sub(0xc));
    c.g[A0][0] = c.g[SP][0];
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, A1)));
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
    c.g[A2][0] = c.lbu(r32(c, V0).wrapping_add(0x194));
    c.g[A1][0] = c.lw(r32(c, S1));
    c.g[A3][0] = c.g[V1][0];
    // key -> sp+0 (unaligned copy through t1).
    let key = r32(c, V1);
    c.lwl(T1, key.wrapping_add(3));
    c.lwr(T1, key);
    let s = sp(c);
    c.swl(T1, s.wrapping_add(3));
    c.swr(T1, s);
    SPCH_NOTIFY_PART_CALLBACK.call(c, 0x003d_9ca0)?;
    c.g[S1][0] = c.lw(r32(c, S1));
    c.g[T1][0] = sx(SPCH_SENTENCE + 0x190);
    c.g[T2][0] = sx(SPCH_PLAY_CALLBACK - 8);
    let event = c.g[S1][0];
    c.sw(sp(c).wrapping_add(0x38), event);
    c.g[S0][0] = sx(r32(c, T1).wrapping_add(r32(c, S0)));
    c.g[V0][0] = c.lw(SPCH_PLAY_CALLBACK);
    c.g[S3][0] = c.lw(r32(c, S0));
    if c.g[V0][0] != 0 {
        // The bank type (sp+0x20), then each slot j of the part.
        c.cycles += 7;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
        c.g[A0][0] = c.g[SP][0];
        SPCH_FIND_BANK_TYPE.call(c, 0x003d_9cd4)?;
        c.g[V1][0] = c.lbu(r32(c, S3).wrapping_add(4));
        c.g[FP][0] = sx(r32(c, V1) >> 2);
        c.g[A1][0] = 0;
        if c.g[FP][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(8));
            c.g[S7][0] = sx(sp(c).wrapping_add(0x1c));
            let list = c.g[V0][0];
            c.sw(sp(c).wrapping_add(0x3c), list);
            c.g[V1][0] = c.lw(sp(c).wrapping_add(0x3c));
            loop {
                play_sentence_slot(c)?;
                c.cycles += 4;
                c.g[A1][0] = c.g[S6][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[FP][0] as i64)) as u64;
                c.g[V1][0] = c.lw(sp(c).wrapping_add(0x3c));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_9cf8);
                }
            }
        }
    }
    // SPCH_NotifyPlayCallback(part header, key, played); count its slots.
    c.cycles += 18;
    c.g[V0][0] = sx(SPCH_SENTENCE + 0x190);
    c.g[V1][0] = SPCH_SENTENCE_PART as u64;
    c.g[A2][0] = c.lw(sp(c).wrapping_add(0x28));
    c.lo[0] = c.g[V0][0];
    c.g[A1][0] = c.g[SP][0];
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x30));
    c.madd(0, S0, V0, V1);
    c.g[A0][0] = c.lw(r32(c, S0));
    SPCH_NOTIFY_PLAY_CALLBACK.call(c, 0x003d_9dfc)?;
    c.g[V1][0] = c.lw(r32(c, S0));
    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x30));
    c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(4));
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x28));
    c.g[V0][0] = sx(r32(c, V0) >> 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let played = c.g[V1][0];
    c.sw(sp(c).wrapping_add(0x28), played);
    if c.g[A0][0] == 0 {
        // First part: the bank's last event = key (repeat count 1), or
        // one more repeat of it.
        c.cycles += 9;
        c.g[A3][0] = c.lw(sp(c).wrapping_add(0x24));
        c.g[V0][0] = sx(SPCH_BANK_LAST);
        c.g[A0][0] = c.lhu(sp(c));
        c.g[A1][0] = sx(r32(c, A3) << 3);
        c.g[A2][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lhu(r32(c, A2));
        c.g[T1][0] = 0x45_0000;
        if c.g[A0][0] == c.g[V1][0] {
            c.cycles += 7;
            c.g[T0][0] = sx(SPCH_BANK_LAST + 4);
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, T0)));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            let n = c.g[V0][0];
            c.sw(r32(c, V1), n);
        } else {
            c.cycles += 8;
            c.g[A3][0] = 1;
            c.g[T1][0] = sx(SPCH_BANK_LAST + 4);
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, T1)));
            let s = sp(c);
            c.lwl(T2, s.wrapping_add(3));
            c.lwr(T2, s);
            let at = r32(c, A2);
            c.swl(T2, at.wrapping_add(3));
            c.swr(T2, at);
            c.sw(r32(c, V0), 1);
        }
    }
    Ok(())
}

/// Slot j (a1) of the part: s5/s4 = its chosen bank/sample entry; plays
/// its (up to 4) samples (0xff: none). Leaves `s6` = j + 1.
fn play_sentence_slot(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    c.g[A3][0] = sx(SPCH_SENTENCE + 0xc);
    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x28));
    c.g[T0][0] = sx(SPCH_SENTENCE + 0x10);
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    c.g[A0][0] = c.lbu(r32(c, V0));
    c.g[V1][0] = sx(r32(c, V1) << 3);
    c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.g[S4][0] = sx(r32(c, V1).wrapping_add(r32(c, T0)));
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[S1][0] = 0;
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
    c.g[S6][0] = sx(r32(c, A1).wrapping_add(1));
    c.g[S2][0] = sx(r32(c, A0).wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S1)));
    loop {
        c.cycles += 4;
        c.g[T1][0] = 0xff;
        c.g[A1][0] = c.lbu(r32(c, V0));
        if c.g[A1][0] != 0xff {
            // param = GetEventParam(event, id); decode the sample entry
            // (bank table[slot bank], slot sample) into sp+0x10.
            c.cycles += 13;
            c.g[A0][0] = c.lw(sp(c).wrapping_add(0x38));
            SPCH_GET_EVENT_PARAM.call(c, 0x003d_9d58)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1734));
            c.g[V0][0] = c.lh(r32(c, S5));
            c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = c.lhu(r32(c, S4));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = c.lw(r32(c, V0));
            SPCH_DECODE_SAMPLE_ENTRY.call(c, 0x003d_9d7c)?;
            let decoded = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, S7).wrapping_add(r32(c, S1)));
            if decoded {
                // callback(key, param, 1 << type[k], bank type).
                c.cycles += 12;
                c.g[T2][0] = sx(SPCH_PLAY_CALLBACK - 8);
                c.g[V1][0] = c.lbu(r32(c, V0));
                c.g[V0][0] = 1;
                c.g[T0][0] = c.lw(SPCH_PLAY_CALLBACK);
                c.g[V1][0] = sx(1u32 << (r32(c, V1) & 31));
                c.g[A3][0] = c.lw(sp(c).wrapping_add(0x20));
                c.g[A0][0] = c.lw(sp(c).wrapping_add(0x34));
                c.g[A2][0] = sx(c.g[V1][0] as u32);
                c.g[A1][0] = c.g[S0][0];
                let callback = r32(c, T0);
                call_ptr(c, callback, 0x003d_9db4)?;
            }
            c.cycles += 1;
        }
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S1)));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_9d40);
        }
    }
}

// ---------------------------------------------------------------------------
// SPCH_SelectRequest
// ---------------------------------------------------------------------------

/// The speech request queue: 16 entries at 0x44fff8 + 0x60 i: `+0x64`
/// queued time, `+0x68` (u16) tie-break stamp, `+0x6a` in use, `+0x6b`
/// bank, `+0x6c` busy, `+0x70` its event (`+0` id, `+2` lifetime, `+4`
/// priority, `+0xa` flags), `+0x74` key.
pub(crate) const SPCH_QUEUE_BASE: u32 = 0x0044_fff8;
pub(crate) const SPCH_QUEUE_ENTRY: u32 = 0x60;

const SPCH_GET_TIME: Callee = Callee {
    addr: 0x003d_b9a0,
    f: ee::s14::SPCH_GetTime,
};
const SPCH_FREE_QUEUE_SLOT: Callee = Callee {
    addr: 0x003d_7c38,
    f: ee::s14::SPCH_FreeQueueSlot,
};

/// `int SPCH_SelectRequest(IdList* list, int bank)` (0x3d7760): picks the
/// queued request of `bank` to play: in use, not busy, its event in `list`
/// (if given), not past its lifetime (expired ones are freed), and in mode-1
/// banks not outranked by the bank entry for its key; highest priority
/// wins, then the oldest, then the larger stamp. Returns its index or -1.
/// Dispatcher-shaped like the original (its loops can unwind and resume at
/// their heads).
pub fn spch_select_request(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003d_7760 => {
                    c.cycles += 28;
                    let frame = Frame::open(c, 0xd0);
                    frame.save(c, S5, 0x80);
                    frame.save(c, S4, 0x70);
                    frame.save(c, FP, 0xb0);
                    frame.save(c, S7, 0xa0);
                    frame.save(c, S3, 0x60);
                    frame.save(c, S2, 0x50);
                    frame.save(c, S1, 0x40);
                    frame.save(c, S0, 0x30);
                    frame.save(c, RA, 0xc0);
                    frame.save(c, S6, 0x90);
                    c.g[S5][0] = c.g[A1][0];
                    c.g[S4][0] = c.g[A0][0];
                    // fp = 1; best priority s7 = 0; best s2 = -1; stride s1;
                    // best age s0 = 0xffffffff (sign-extended).
                    c.g[FP][0] = 1;
                    c.g[S7][0] = 0;
                    c.g[S2][0] = u64::MAX;
                    c.g[S1][0] = SPCH_QUEUE_ENTRY as u64;
                    c.g[S0][0] = u64::MAX;
                    SPCH_GET_TIME.call(c, 0x003d_77b0)?;
                    // now = time + offset (sp+0x14); i (t5) = 0, max pri t9.
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x174c));
                    c.g[A0][0] = 0x45_0000;
                    c.g[S3][0] = sx(SPCH_QUEUE_BASE);
                    c.g[T9][0] = 0;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[T5][0] = 0;
                    let now = c.g[V0][0];
                    c.sw(sp(c).wrapping_add(0x14), now);
                    c.mult(0, A0, T5, S1);
                    pc = 0x003d_77d0;
                }
                // Request i (t5; a0 = its offset): in use, this bank, idle?
                0x003d_77d0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                    c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x6a));
                    c.g[S6][0] = sx(r32(c, T5).wrapping_add(1));
                    if c.g[V0][0] == 0 {
                        pc = 0x003d_7a0c;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x6b));
                    if c.g[V0][0] != c.g[S5][0] {
                        c.g[T5][0] = c.g[S6][0];
                        pc = 0x003d_7a10;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(0x6c));
                    if c.g[V0][0] != 0 {
                        c.g[T5][0] = c.g[S6][0];
                        pc = 0x003d_7a10;
                        continue;
                    }
                    // t3 = its event; t7 expired = 0, t4 blocked = 0,
                    // t8 in list = 1.
                    c.cycles += 10;
                    c.g[A1][0] = 0x45_0000;
                    c.g[T7][0] = 0;
                    c.g[A1][0] = sx(SPCH_QUEUE_BASE + 0x70);
                    c.g[T4][0] = 0;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                    c.g[T8][0] = 1;
                    c.g[T3][0] = c.lw(r32(c, V0));
                    c.g[T6][0] = 0x45_0000;
                    c.g[T2][0] = sx(r32(c, S5) << 2);
                    if c.g[S4][0] == 0 {
                        pc = 0x003d_7880;
                        continue;
                    }
                    // Is the event id in the list?
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r32(c, S4));
                    c.g[T0][0] = 0;
                    c.g[A2][0] = c.lhu(r32(c, T3));
                    c.g[A0][0] = 0;
                    c.g[A3][0] = c.g[V0][0];
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x003d_787c;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(4));
                    c.g[V0][0] = c.lhu(r32(c, V1));
                    c.g[A1][0] = c.g[V1][0];
                    if c.g[V0][0] == c.g[A2][0] {
                        c.cycles += 2;
                        c.g[T0][0] = 1;
                        pc = 0x003d_787c;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                        pc = 0x003d_785c;
                    }
                }
                0x003d_785c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0) << 1);
                    if done {
                        pc = 0x003d_787c;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[V1][0] = c.lhu(r32(c, V0));
                    let other = c.g[V1][0] != c.g[A2][0];
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                    if other {
                        if back_edge_unwinds(c, 0x003d_785c) {
                            return Ok(());
                        }
                        pc = 0x003d_785c;
                    } else {
                        c.cycles += 1;
                        c.g[T0][0] = 1;
                        pc = 0x003d_787c;
                    }
                }
                0x003d_787c => {
                    c.cycles += 1;
                    c.g[T8][0] = c.g[T0][0];
                    pc = 0x003d_7880;
                }
                // age (t1) = now - queued; expired (t7) past a lifetime.
                0x003d_7880 => {
                    c.cycles += 9;
                    c.mult(0, A1, T5, S1);
                    c.g[V0][0] = 0x45_0000;
                    c.g[A2][0] = sx(SPCH_QUEUE_BASE);
                    c.g[A0][0] = c.lhu(r32(c, T3).wrapping_add(2));
                    c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x64));
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x14));
                    c.g[T1][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                    if c.g[A0][0] != 0 {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[A0][0];
                        c.g[T7][0] = c.g[FP][0];
                        c.g[V0][0] = (c.g[V0][0] < c.g[T1][0]) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[T7][0] = 0;
                        }
                    }
                    // Mode-1 bank and an event with flag bit 0: blocked?
                    c.cycles += 5;
                    c.g[V0][0] = sx(r32(c, T6).wrapping_add(0x6a0));
                    c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != c.g[FP][0] {
                        pc = 0x003d_7970;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lbu(r32(c, T3).wrapping_add(0xa));
                    c.g[V0][0] &= 1;
                    let plain = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A2).wrapping_add(0x74));
                    if plain {
                        pc = 0x003d_7970;
                        continue;
                    }
                    // Find the bank entry of the request's key (sp+0) in
                    // the 8-entry table at 0x44ffb8; its slot -> sp+0x10.
                    c.cycles += 14;
                    c.g[V1][0] = 0x45_0000;
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                    c.g[T0][0] = sx(0x0044_ffb8);
                    let key = r32(c, V0);
                    c.lwl(V1, key.wrapping_add(3));
                    c.lwr(V1, key);
                    let s = sp(c);
                    c.swl(V1, s.wrapping_add(3));
                    c.g[T2][0] = 0;
                    c.swr(V1, s);
                    c.g[T4][0] = sx(sp(c).wrapping_add(0x10));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[T6][0] = 0;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = c.g[T0][0];
                    pc = 0x003d_7910;
                }
                0x003d_7910 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(r32(c, T0));
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                    if c.g[A0][0] != 0 {
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu(r32(c, A1).wrapping_add(2));
                        c.g[V0][0] = c.lbu(r32(c, A0).wrapping_add(9));
                        if c.g[V0][0] == c.g[V1][0] {
                            c.cycles += 4;
                            c.g[V1][0] = c.lbu(r32(c, A0).wrapping_add(8));
                            c.g[V0][0] = c.lbu(r32(c, A1).wrapping_add(3));
                            if c.g[V1][0] == c.g[V0][0] {
                                let slot = c.g[A3][0];
                                c.sw(r32(c, T4), slot);
                                if back_edge_unwinds(c, 0x003d_7850) {
                                    return Ok(());
                                }
                                pc = 0x003d_7850;
                                continue;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(8));
                    if more {
                        if back_edge_unwinds(c, 0x003d_7910) {
                            return Ok(());
                        }
                        pc = 0x003d_7910;
                    } else {
                        pc = 0x003d_794c;
                    }
                }
                // Found (t6 = 1).
                0x003d_7850 => {
                    c.cycles += 2;
                    c.g[T6][0] = 1;
                    pc = 0x003d_794c;
                }
                // blocked (t4) = found and priority < the entry's.
                0x003d_794c => {
                    c.cycles += 2;
                    c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
                    if c.g[T6][0] != 0 {
                        c.cycles += 6;
                        c.g[T2][0] = c.g[FP][0];
                        c.g[A0][0] = c.lhu(r32(c, T3).wrapping_add(4));
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[V1][0] = c.lhu(r32(c, V0).wrapping_add(0x16));
                        c.g[A0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                        if c.g[A0][0] == 0 {
                            c.g[T2][0] = 0;
                        }
                    }
                    c.cycles += 1;
                    c.g[T4][0] = c.g[T2][0];
                    pc = 0x003d_7970;
                }
                0x003d_7970 => {
                    c.cycles += 2;
                    if c.g[T7][0] != 0 {
                        // Expired: free it (t9 kept on the stack).
                        c.cycles += 5;
                        let s = sp(c).wrapping_add(0x20);
                        c.sq(T9, s);
                        c.g[A0][0] = c.g[T5][0];
                        SPCH_FREE_QUEUE_SLOT.call(c, 0x003d_7984)?;
                        let s = sp(c).wrapping_add(0x20);
                        c.lq(T9, s);
                        pc = 0x003d_7a0c;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[T8][0] == 0 {
                        c.g[T5][0] = c.g[S6][0];
                        pc = 0x003d_7a10;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[T4][0] != 0 {
                        c.g[T5][0] = c.g[S6][0];
                        pc = 0x003d_7a10;
                        continue;
                    }
                    // Higher priority: the new best.
                    c.cycles += 4;
                    c.g[V1][0] = c.lhu(r32(c, T3).wrapping_add(4));
                    c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.cycles += 6;
                        c.g[S2][0] = c.g[T5][0];
                        c.g[S0][0] = c.g[T1][0];
                        c.mult(0, A0, S2, S1);
                        c.g[T9][0] = c.g[V1][0];
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                        pc = 0x003d_7a08;
                        continue;
                    }
                    // Same priority: younger, or as old with a larger stamp.
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[T9][0] {
                        c.g[T5][0] = c.g[S6][0];
                        pc = 0x003d_7a10;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[T1][0] < c.g[S0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        if c.g[T1][0] != c.g[S0][0] {
                            c.g[T5][0] = c.g[S6][0];
                            pc = 0x003d_7a10;
                            continue;
                        }
                        c.cycles += 6;
                        c.mult(0, V0, T5, S1);
                        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                        c.g[V0][0] = c.lhu(r32(c, V1).wrapping_add(0x68));
                        c.g[V0][0] = (c.g[S7][0] < c.g[V0][0]) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[T5][0] = c.g[S6][0];
                            pc = 0x003d_7a10;
                            continue;
                        }
                        c.cycles += 1;
                    }
                    c.g[S2][0] = c.g[T5][0];
                    c.cycles += 3;
                    c.g[S0][0] = c.g[T1][0];
                    c.mult(0, V1, S2, S1);
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
                    pc = 0x003d_7a08;
                }
                // The best's stamp (s7).
                0x003d_7a08 => {
                    c.cycles += 1;
                    c.g[S7][0] = c.lhu(r32(c, V0).wrapping_add(0x68));
                    c.cycles += 1;
                    c.g[T5][0] = c.g[S6][0];
                    pc = 0x003d_7a10;
                }
                0x003d_7a0c => {
                    c.cycles += 1;
                    c.g[T5][0] = c.g[S6][0];
                    pc = 0x003d_7a10;
                }
                0x003d_7a10 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T5][0] as i64) < 16) as u64;
                    let more = c.g[V0][0] != 0;
                    c.mult(0, A0, T5, S1);
                    if more {
                        if back_edge_unwinds(c, 0x003d_77d0) {
                            return Ok(());
                        }
                        pc = 0x003d_77d0;
                        continue;
                    }
                    c.cycles += 13;
                    c.g[V0][0] = c.g[S2][0];
                    let frame = Frame { size: 0xd0 };
                    for (reg, off) in [
                        (RA, 0xc0),
                        (FP, 0xb0),
                        (S7, 0xa0),
                        (S6, 0x90),
                        (S5, 0x80),
                        (S4, 0x70),
                        (S3, 0x60),
                        (S2, 0x50),
                        (S1, 0x40),
                        (S0, 0x30),
                    ] {
                        frame.restore(c, reg, off);
                    }
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_7760, 0x003d_7a50);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SPCH_QueueEventArgs
// ---------------------------------------------------------------------------

/// Per-bank queue state (0x44fff8 + 4 bank): `+0` queued count, `+0x20`
/// newest request, `+0x40` newest non-chained request.
pub(crate) const SPCH_BANK_QUEUED: u32 = 0x0044_fff8;

const SPCH_ALLOC_QUEUE_SLOT: Callee = Callee {
    addr: 0x003d_72a0,
    f: ee::s14::SPCH_AllocQueueSlot,
};

/// `int SPCH_QueueEventArgs(int* args)` (0x3d7418): queues speech event
/// `args[0]` with its 20 argument words (not re-entrantly: gp+0x1758).
/// Finds the event and the bank entry of its key; a plain event must pass
/// its chance roll and takes its own priority; a chained event (flag bit
/// 5) follows the bank's newest requests (both must be valid) at their
/// priority. Allocates a queue slot (a chained event frees the newest on
/// failure), stamps it (time, a per-tick sequence at gp+0x1754), copies the
/// arguments and updates the bank's newest/queued state. Returns 1 when
/// queued. Dispatcher-shaped like the original (its loops can unwind and
/// resume at their heads).
pub fn spch_queue_event_args(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003d_7418 => {
                    c.cycles += 15;
                    let frame = Frame::open(c, 0xa0);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1758));
                    frame.save(c, S6, 0x80);
                    frame.save(c, S5, 0x70);
                    frame.save(c, S4, 0x60);
                    frame.save(c, S3, 0x50);
                    frame.save(c, RA, 0x90);
                    frame.save(c, S2, 0x40);
                    frame.save(c, S1, 0x30);
                    frame.save(c, S0, 0x20);
                    // s6 newest = -1, s5 result = 0, s4 args, s3 chained = -1.
                    c.g[S6][0] = u64::MAX;
                    c.g[S5][0] = 0;
                    c.g[S4][0] = c.g[A0][0];
                    c.g[S3][0] = u64::MAX;
                    if c.g[V0][0] != 0 {
                        pc = 0x003d_76c0;
                        continue;
                    }
                    // Busy; key = args[0] at sp+0; find the event.
                    c.cycles += 11;
                    c.g[V0][0] = 1;
                    c.g[A0][0] = c.g[SP][0];
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x1758), 1);
                    let args = r32(c, S4);
                    c.lwl(V0, args.wrapping_add(3));
                    c.lwr(V0, args);
                    let s = sp(c);
                    c.swl(V0, s.wrapping_add(3));
                    c.swr(V0, s);
                    SPCH_FIND_EVENT.call(c, 0x003d_7474)?;
                    c.g[S2][0] = c.g[V0][0];
                    let none = c.g[S2][0] == 0;
                    c.g[V0][0] = 0x45_0000;
                    if none {
                        pc = 0x003d_76c0;
                        continue;
                    }
                    // Find the bank entry (8 at 0x44ffb8) of the key.
                    c.cycles += 8;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = sx(0x0044_ffb8);
                    c.g[T3][0] = sx(sp(c).wrapping_add(0x10));
                    c.g[T4][0] = 0;
                    c.g[T1][0] = sx(sp(c).wrapping_add(0x14));
                    c.g[T2][0] = 0;
                    c.g[A3][0] = 0;
                    c.g[T0][0] = c.g[A1][0];
                    pc = 0x003d_74a0;
                }
                0x003d_74a0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(r32(c, A1));
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    if c.g[A0][0] != 0 {
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu(r32(c, A2).wrapping_add(2));
                        c.g[V0][0] = c.lbu(r32(c, A0).wrapping_add(9));
                        if c.g[V0][0] == c.g[V1][0] {
                            c.cycles += 4;
                            c.g[V1][0] = c.lbu(r32(c, A0).wrapping_add(8));
                            c.g[V0][0] = c.lbu(r32(c, A2).wrapping_add(3));
                            if c.g[V1][0] == c.g[V0][0] {
                                // Found: its slot -> sp+0x14.
                                let slot = c.g[T0][0];
                                c.sw(r32(c, T1), slot);
                                c.cycles += 2;
                                c.g[T2][0] = 1;
                                if back_edge_unwinds(c, 0x003d_74dc) {
                                    return Ok(());
                                }
                                pc = 0x003d_74dc;
                                continue;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(8));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 8) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
                    if more {
                        if back_edge_unwinds(c, 0x003d_74a0) {
                            return Ok(());
                        }
                        pc = 0x003d_74a0;
                    } else {
                        pc = 0x003d_74dc;
                    }
                }
                0x003d_74dc => {
                    // Found: the bank index (slot word 1) -> sp+0x10.
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x14));
                    if c.g[T2][0] != 0 {
                        c.cycles += 3;
                        c.g[T4][0] = 1;
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(4));
                        let bank = c.g[V0][0];
                        c.sw(r32(c, T3), bank);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S5][0];
                    if c.g[T4][0] == 0 {
                        pc = 0x003d_76c4;
                        continue;
                    }
                    // s1 = chained (flag bit 5).
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(r32(c, S2).wrapping_add(0xa));
                    c.g[V0][0] = sx(r32(c, V0) >> 5);
                    c.g[S1][0] = c.g[V0][0] & 1;
                    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x10));
                    if c.g[S1][0] != 0 {
                        // The bank's newest (s6) and newest plain (s3)
                        // requests, both < 16; priority of the plain one.
                        c.cycles += 13;
                        c.g[V1][0] = 0x45_0000;
                        c.g[A3][0] = sx(SPCH_BANK_QUEUED);
                        c.g[V0][0] = sx(r32(c, A0) << 2);
                        c.g[A1][0] = c.g[A0][0];
                        c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
                        c.g[V0][0] = c.g[V1][0];
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x40));
                        c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x20));
                        c.g[V1][0] = (c.g[A0][0] < 0x10) as u64;
                        c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = 0;
                        }
                        let bad = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x60;
                        if bad {
                            pc = 0x003d_76c0;
                            continue;
                        }
                        c.cycles += 6;
                        c.g[S3][0] = c.g[A0][0];
                        c.mult(0, V1, S3, V0);
                        c.g[S6][0] = c.g[A2][0];
                        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
                        c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(0x70));
                        c.cycles += 2;
                        c.g[A0][0] = c.lhu(r32(c, V0).wrapping_add(4));
                    } else {
                        // The chance roll (event +9), own priority.
                        c.cycles += 7;
                        c.g[S0][0] = c.lb(r32(c, S2).wrapping_add(9));
                        c.g[A0][0] = 0x64;
                        c.g[A1][0] = u64::MAX;
                        SPCH_RANDOM_NO_REPEAT.call(c, 0x003d_7570)?;
                        c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let missed = c.g[S0][0] != 0;
                        c.g[V0][0] = c.g[S5][0];
                        if missed {
                            pc = 0x003d_76c4;
                            continue;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lhu(r32(c, S2).wrapping_add(4));
                        c.cycles += 1;
                        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
                    }
                    // slot = SPCH_AllocQueueSlot(priority, bank, chained).
                    c.cycles += 5;
                    c.g[A2][0] = c.g[S1][0];
                    SPCH_ALLOC_QUEUE_SLOT.call(c, 0x003d_7598)?;
                    c.g[S0][0] = c.g[V0][0];
                    if (c.g[S0][0] as i64) < 0 {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S5][0];
                        if c.g[S1][0] != 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S3][0];
                            SPCH_FREE_QUEUE_SLOT.call(c, 0x003d_75b4)?;
                            c.g[V0][0] = c.g[S5][0];
                        }
                        pc = 0x003d_76c4;
                        continue;
                    }
                    // Sequence within the same time stamp.
                    c.cycles += 6;
                    SPCH_GET_TIME.call(c, 0x003d_75c4)?;
                    c.g[T1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1750));
                    if c.g[T1][0] != c.g[V0][0] {
                        c.sh((c.g[GP][0] as u32).wrapping_add(0x1754), 0);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu((c.g[GP][0] as u32).wrapping_add(0x1754));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                        let seq = c.g[V0][0];
                        c.sh((c.g[GP][0] as u32).wrapping_add(0x1754), seq);
                    }
                    // Fill the request (t0 = slot * 0x60).
                    c.cycles += 19;
                    c.g[T3][0] = 0x60;
                    c.g[V0][0] = 0x45_0000;
                    c.mult(0, T0, S0, T3);
                    c.g[T2][0] = sx(SPCH_BANK_QUEUED);
                    c.g[A1][0] = c.lbu(sp(c).wrapping_add(0x10));
                    c.g[T4][0] = c.g[V0][0];
                    c.g[A2][0] = c.lhu((c.g[GP][0] as u32).wrapping_add(0x1754));
                    c.g[A3][0] = u64::MAX;
                    let now = c.g[T1][0];
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x1750), now);
                    c.g[V1][0] = sx(r32(c, T2).wrapping_add(r32(c, T0)));
                    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, T2)));
                    let (event, req) = (c.g[S2][0], r32(c, V0));
                    c.sw(r32(c, V1).wrapping_add(0x70), event);
                    c.g[A0][0] = c.g[V1][0];
                    let bank = c.g[A1][0];
                    c.sb(req.wrapping_add(0x6b), bank);
                    let seq = c.g[A2][0];
                    c.sh(req.wrapping_add(0x68), seq);
                    c.sw(r32(c, A0).wrapping_add(0x64), now);
                    c.sh(req.wrapping_add(0x6e), u64::MAX);
                    let chained = c.g[S1][0];
                    c.sh(req.wrapping_add(0x6c), chained);
                    if chained != 0 {
                        // The newest request chains to this one.
                        c.cycles += 3;
                        c.mult(0, V1, S6, T3);
                        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, T2)));
                        let slot = c.g[S0][0];
                        c.sh(r32(c, V0).wrapping_add(0x6e), slot);
                    }
                    // Copy the 20 argument words to +0x74.
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, T2).wrapping_add(0x74));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = 0x13;
                    pc = 0x003d_7648;
                }
                0x003d_7648 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(r32(c, A0));
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                    let w = c.g[V0][0];
                    c.sw(r32(c, V1), w);
                    let more = (c.g[A1][0] as i64) >= 0;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                    if more {
                        if back_edge_unwinds(c, 0x003d_7648) {
                            return Ok(());
                        }
                        pc = 0x003d_7648;
                        continue;
                    }
                    // Plain: one more queued; mark in use; newest (+plain).
                    c.cycles += 2;
                    c.g[V0][0] = 0x60;
                    if c.g[S1][0] == 0 {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                        c.g[A0][0] = sx(r32(c, T4).wrapping_sub(8));
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                        let n = c.g[V1][0];
                        c.sw(r32(c, V0), n);
                        c.g[V0][0] = 0x60;
                    }
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x10));
                    c.mult(0, V0, S0, V0);
                    c.g[A1][0] = sx(r32(c, T4).wrapping_sub(8));
                    c.g[A2][0] = sx(r32(c, V1) << 2);
                    c.g[A0][0] = 1;
                    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.sb(r32(c, V0).wrapping_add(0x6a), 1);
                    let slot = c.g[S0][0];
                    c.sw(r32(c, V1).wrapping_add(0x20), slot);
                    if c.g[S1][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[V1][0];
                        c.sw(r32(c, V0).wrapping_add(0x40), slot);
                    }
                    c.cycles += 1;
                    c.g[S5][0] = 1;
                    pc = 0x003d_76c0;
                }
                0x003d_76c0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S5][0];
                    pc = 0x003d_76c4;
                }
                0x003d_76c4 => {
                    c.cycles += 11;
                    let frame = Frame { size: 0xa0 };
                    for (reg, off) in [
                        (RA, 0x90),
                        (S6, 0x80),
                        (S5, 0x70),
                        (S4, 0x60),
                        (S3, 0x50),
                        (S2, 0x40),
                        (S1, 0x30),
                        (S0, 0x20),
                    ] {
                        frame.restore(c, reg, off);
                    }
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x1758), 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_7418, 0x003d_76f0);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_StreamStartCallback
// ---------------------------------------------------------------------------

/// Stream message fields: `+0x18` the speech entry (`+0` id, `+4` volume
/// slot, `+0x8d0` arg), `+0x1c` the request it was started for, `+0x20`
/// the stream slot (-1: none).
pub(crate) const STREAM_START_ENTRY: u32 = 0x18;
pub(crate) const STREAM_START_REQUEST: u32 = 0x1c;
pub(crate) const STREAM_START_SLOT: u32 = 0x20;

const STREAM_START_INSTANCE_SLOT: Callee = Callee {
    addr: 0x002b_1220,
    f: ee::s08::SpeechSystem_GetInstanceStreamSlot,
};
const STREAM_START_CURRENT_REQUEST: Callee = Callee {
    addr: 0x002a_b150,
    f: ee::s08::StreamArray_GetCurrentRequest,
};
const STREAM_START_ADD_TIMER: Callee = Callee {
    addr: 0x002a_dca0,
    f: ee::s08::BankMonitor_AddTimer,
};
const STREAM_START_VOLUME_SCALE: Callee = Callee {
    addr: 0x002a_b0d0,
    f: ee::s08::StreamArray_SetVolumeScalePtr,
};
const STREAM_START_CLEAR_PREBUFFER: Callee = Callee {
    addr: 0x002b_1248,
    f: ee::s08::SpeechSystem_ClearPrebuffer,
};

/// `int SpeechSystem_StreamStartCallback(SpeechSystem* s, StreamMsg* m)`
/// (0x2b1268): a stream started. If its instance's current request is no
/// longer the message's, retries in 200 ticks (BankMonitor_AddTimer with
/// the message) and returns 1. Otherwise, with a stream slot, asks the
/// system's volume method (vtable `+0x18`) for a scale pointer and sets it
/// on the entry's volume slot; then clears the prebuffer and returns its
/// result.
pub fn speech_system_stream_start_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_ENTRY));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
        c.g[A1][0] = c.lw(r32(c, S0));
        STREAM_START_INSTANCE_SLOT.call(c, 0x002b_1294)?;
        c.g[A0][0] = c.lw(r32(c, S1));
        c.g[A1][0] = c.g[V0][0];
        STREAM_START_CURRENT_REQUEST.call(c, 0x002b_12a0)?;
        c.g[T0][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_REQUEST));
        let current = c.g[V0][0] == c.g[T0][0];
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
        if !current {
            // BankMonitor_AddTimer(params, 200, timer cb, entry, request,
            // slot, 0, 0).
            c.cycles += 14;
            c.ldl(A2, 0x004a_37e7);
            c.ldr(A2, 0x004a_37e0);
            c.g[A1][0] = 0xc8;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[T2][0] = 0;
            c.g[T1][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_SLOT));
            c.g[T3][0] = 0;
            c.g[A0][0] = c.lw(r32(c, V0));
            c.g[A3][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_ENTRY));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            STREAM_START_ADD_TIMER.call(c, 0x002b_12dc)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[A2][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_SLOT));
            c.g[V0][0] = u64::MAX;
            if c.g[A2][0] != c.g[V0][0] {
                // scale = s->vt[3](s, entry->id, slot, entry->arg).
                c.cycles += 9;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x1bc));
                c.g[A1][0] = c.lw(r32(c, S0));
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x18));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(0x8d0));
                let method = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x002b_1310)?;
                c.g[A2][0] = c.g[V0][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(4));
                    c.g[A0][0] = c.lw(r32(c, S1));
                    STREAM_START_VOLUME_SCALE.call(c, 0x002b_1324)?;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            STREAM_START_CLEAR_PREBUFFER.call(c, 0x002b_1330)?;
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

// ---------------------------------------------------------------------------
// RailModifier_ctor
// ---------------------------------------------------------------------------

/// `RailModifier` fields: `+0x10`/`+0x20` its bound box (min, max), `+0x30`
/// the rail reference (id), `+0x34` the descriptor value, `+0x40` the model,
/// `+0x44` its spatial-hash entry.
pub(crate) const RAIL_MOD_BOX: u32 = 0x10;
pub(crate) const RAIL_MOD_REF: u32 = 0x30;
pub(crate) const RAIL_MOD_MODEL: u32 = 0x40;
pub(crate) const RAIL_MOD_ENTRY: u32 = 0x44;
/// The `RailModifier` vtable.
pub(crate) const RAIL_MOD_VTABLE: u32 = 0x0049_11d0;

const RAIL_MOD_BUILD_NODES: Callee = Callee {
    addr: 0x0035_c0e8,
    f: ee::s11::RailModifier_BuildRailNodes,
};
const RAIL_MOD_UPDATE_BOUNDS: Callee = Callee {
    addr: 0x0035_c040,
    f: ee::s11::RailModifier_UpdateBounds,
};

/// `RailModifier* RailModifier_ctor(void* mem, RailDesc* desc, Model*
/// model)` (0x35b708): sets up the modifier (vtable, rail reference and
/// value from `desc`); unless the model's object says otherwise (vtable
/// `+0x160`), takes the model's box grown by 100 on each side; builds the
/// rail nodes and bounds, inserts its entry into the world's spatial hash,
/// then removes each segment of its rail track from the hash (freeing hash
/// nodes left empty). Returns `mem`. Dispatcher-shaped like the original
/// (its loops can unwind and resume at their heads).
pub fn rail_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0035_b708 => {
                    c.cycles += 32;
                    let frame = Frame::open(c, 0xc0);
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    frame.save(c, S3, 0x70);
                    c.g[V1][0] = 0x49_0000;
                    frame.save(c, S1, 0x90);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S0, 0xa0);
                    c.g[V1][0] = sx(RAIL_MOD_VTABLE);
                    frame.save(c, S2, 0x80);
                    c.g[V0][0] |= 0xffff;
                    frame.save(c, S4, 0x60);
                    c.g[S1][0] = sx(r32(c, S3).wrapping_add(RAIL_MOD_BOX));
                    frame.save(c, S5, 0x50);
                    frame.save_ra(c, 0x40);
                    frame.save_fpr(c, 20, 0xb0);
                    let s3 = r32(c, S3);
                    let (minus1, vt, model) = (c.g[V0][0], c.g[V1][0], c.g[A2][0]);
                    c.sw(s3.wrapping_add(RAIL_MOD_REF), minus1);
                    c.sw(s3.wrapping_add(8), vt);
                    c.sw(s3, 0);
                    c.sw(s3.wrapping_add(4), 0);
                    c.sw(s3.wrapping_add(RAIL_MOD_MODEL), model);
                    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
                    let id = c.g[V0][0];
                    c.sw(s3.wrapping_add(RAIL_MOD_REF), id);
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(8));
                    let value = c.g[V1][0];
                    c.sw(s3.wrapping_add(0x34), value);
                    // model->obj->vt[0x160](obj).
                    c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0xc));
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0xc));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x160));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x164));
                    let method = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0035_b780)?;
                    if c.g[V0][0] == 0 {
                        rail_mod_model_box(c);
                    }
                    // Nodes, bounds; insert the entry into the hash.
                    c.cycles += 39;
                    c.g[A0][0] = c.g[S3][0];
                    RAIL_MOD_BUILD_NODES.call(c, 0x0035_b824)?;
                    c.g[A0][0] = c.g[S3][0];
                    RAIL_MOD_UPDATE_BOUNDS.call(c, 0x0035_b82c)?;
                    call_ptr(c, 0x002d_1be0, 0x0035_b834)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S1][0] = c.lw(r32(c, S3).wrapping_add(RAIL_MOD_ENTRY));
                    call_ptr(c, 0x0032_8f28, 0x0035_b848)?;
                    // Child of the hash root for the key at sp+0.
                    let s = sp(c);
                    c.g[V1][0] = c.lw(s.wrapping_add(4));
                    c.g[A3][0] = u64::MAX;
                    c.g[A1][0] = c.lw(s.wrapping_add(8));
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x50));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A2][0] = c.lw(s.wrapping_add(0xc));
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x28));
                    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[A1][0] != 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x14));
                    c.g[A1][0] = 2;
                    if c.g[A3][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.g[A3][0] = c.g[SP][0];
                    call_ptr(c, 0x0032_8c20, 0x0035_b88c)?;
                    // s5 = the rail track (world rail table, by ref id).
                    call_ptr(c, 0x002d_1bd8, 0x0035_b894)?;
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[A1][0] = c.lw(r32(c, S3).wrapping_add(RAIL_MOD_REF));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(8));
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x44));
                        c.cycles += 7;
                        c.g[V0][0] = sx(r32(c, A1) >> 8);
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        c.g[V0][0] = sx(r32(c, V1) >> 8);
                        let some = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        if !some {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                        c.cycles += 1;
                        c.g[S5][0] = c.g[V0][0];
                    } else {
                        c.cycles += 2;
                        c.g[S5][0] = 0;
                    }
                    // Its segments (s1), count at +0x20.
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(0x20));
                    c.g[S2][0] = 0;
                    c.g[S1][0] = c.lw(r32(c, S5).wrapping_add(0x24));
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x0035_ba58;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 20);
                    c.g[S4][0] = u64::MAX;
                    pc = 0x0035_b900;
                }
                // Segment s1: its box -> sp; remove it from the hash node.
                0x0035_b900 => {
                    c.cycles += 44;
                    let (seg, s) = (r32(c, S1), sp(c));
                    c.lwc1(0, seg.wrapping_add(0x6c));
                    c.swc1(0, s);
                    c.lwc1(1, seg.wrapping_add(0x70));
                    c.swc1(1, s.wrapping_add(4));
                    c.lwc1(2, seg.wrapping_add(0x74));
                    c.swc1(20, s.wrapping_add(0xc));
                    c.swc1(2, s.wrapping_add(8));
                    c.lwc1(0, seg.wrapping_add(0x78));
                    c.swc1(0, s.wrapping_add(0x10));
                    c.lwc1(1, seg.wrapping_add(0x7c));
                    c.swc1(1, s.wrapping_add(0x14));
                    c.lwc1(0, seg.wrapping_add(0x80));
                    c.swc1(20, s.wrapping_add(0x1c));
                    c.swc1(0, s.wrapping_add(0x18));
                    call_ptr(c, 0x002d_1be0, 0x0035_b93c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    let n = c.g[V0][0];
                    c.sw(r32(c, S0).wrapping_add(0xa0), n);
                    call_ptr(c, 0x0032_8f28, 0x0035_b958)?;
                    let s = sp(c);
                    c.g[V0][0] = c.lw(s.wrapping_add(0x24));
                    c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x50));
                    c.g[A0][0] = c.lw(s.wrapping_add(0x28));
                    c.g[A1][0] = 2;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw(s.wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x14));
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x0032_84b8, 0x0035_b9a0)?;
                    // Is the node's bucket block empty (8 words, then the
                    // 3 lists)?
                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    let mut used = c.g[V0][0] != 0;
                    for off in [4u32, 8, 0xc, 0x10, 0x14] {
                        if used {
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(off));
                        used = c.g[V0][0] != 0;
                    }
                    if used {
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        pc = 0x0035_ba04;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    if c.g[V0][0] != 0 {
                        pc = 0x0035_ba04;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    if c.g[V0][0] != 0 {
                        pc = 0x0035_ba04;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
                    c.g[A0][0] = 0;
                    pc = 0x0035_ba18;
                }
                // Not empty.
                0x0035_ba04 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0035_ba34;
                }
                0x0035_ba18 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, V1));
                    let used = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                    if used {
                        if back_edge_unwinds(c, 0x0035_ba04) {
                            return Ok(());
                        }
                        pc = 0x0035_ba04;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                    if more {
                        if back_edge_unwinds(c, 0x0035_ba18) {
                            return Ok(());
                        }
                        pc = 0x0035_ba18;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    pc = 0x0035_ba34;
                }
                // Empty: free the block. Next segment.
                0x0035_ba34 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                        call_ptr(c, 0x0031_7e50, 0x0035_ba44)?;
                        c.sw(r32(c, S0).wrapping_add(0x10), 0);
                    }
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(0x20));
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = c.lw(r32(c, S1).wrapping_add(0x64));
                    if more {
                        if back_edge_unwinds(c, 0x0035_b900) {
                            return Ok(());
                        }
                        pc = 0x0035_b900;
                        continue;
                    }
                    pc = 0x0035_ba58;
                }
                0x0035_ba58 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.g[S3][0];
                    let frame = Frame { size: 0xc0 };
                    frame.restore(c, S0, 0xa0);
                    frame.restore(c, S1, 0x90);
                    frame.restore(c, S2, 0x80);
                    frame.restore(c, S3, 0x70);
                    frame.restore(c, S4, 0x60);
                    frame.restore(c, S5, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.restore_fpr(c, 20, 0xb0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0035_b708, 0x0035_ba84);
                    return Ok(());
                }
            }
        }
    })
}

/// The model's box (`+0x60..0x74`, w = 1) into the modifier's box, grown
/// by (100, 100, 100, 0) on each side (VU0 sub/add).
fn rail_mod_model_box(c: &mut Cpu) {
    c.cycles += 37;
    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(RAIL_MOD_MODEL));
    let (m, s, s3) = (r32(c, V0), sp(c), r32(c, S3));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 4);
    c.lwc1(1, m.wrapping_add(0x60));
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 3);
    c.swc1(1, s);
    c.lwc1(0, m.wrapping_add(0x64));
    c.swc1(0, s.wrapping_add(4));
    c.lwc1(1, m.wrapping_add(0x68));
    c.swc1(4, s.wrapping_add(0xc));
    c.swc1(1, s.wrapping_add(8));
    c.lwc1(2, m.wrapping_add(0x6c));
    c.lq(V1, s);
    c.swc1(2, s.wrapping_add(0x10));
    c.lwc1(0, m.wrapping_add(0x70));
    c.swc1(0, s.wrapping_add(0x14));
    c.lwc1(1, m.wrapping_add(0x74));
    c.swc1(4, s.wrapping_add(0x1c));
    c.swc1(1, s.wrapping_add(0x18));
    c.sq(V1, s.wrapping_add(0x20));
    c.lq(V0, s.wrapping_add(0x10));
    c.sq(V1, s3.wrapping_add(0x10));
    c.sq(V0, s3.wrapping_add(0x20));
    c.swc1(3, s.wrapping_add(0x28));
    c.sq(V0, s.wrapping_add(0x30));
    c.swc1(3, s.wrapping_add(0x20));
    c.swc1(3, s.wrapping_add(0x24));
    c.sw(s.wrapping_add(0x2c), 0);
    c.lqc2(3, s3.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s3.wrapping_add(0x10));
    c.lqc2(3, s3.wrapping_add(0x20));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s3.wrapping_add(0x20));
}

//@BODIES@

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, put_words, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The gate of this module's fakes (in each test machine's RAM).
    const OC375_CTRL: u32 = DATA + 0x20_0000;
    const OC375_MAGIC: u32 = 0x0c37_5a7e;
    /// Scripted results: up to 16 `(target, v0)` pairs (v0 sign-extended).
    const OC375_TAB: u32 = OC375_CTRL + 0x10;

    /// A logged fake (may unwind) whose `v0` comes from the script.
    fn oc375_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for i in 0..16 {
            if c.bus.read32(OC375_TAB + 8 * i) == target {
                let v = c.bus.read32(OC375_TAB + 8 * i + 4);
                let h = c.g[V1][0];
                c.g[V0][0] = match v {
                    OC375_COIN => h >> 7 & 1,
                    OC375_SMALL => sx(((h >> 9) % 5) as u32).wrapping_sub(2),
                    _ => sx(v),
                };
            }
        }
    }

    /// Scripted `v0` markers: 0 or 1, or -2..2, from the call's hash.
    const OC375_COIN: u32 = 0x7fff_c014;
    const OC375_SMALL: u32 = 0x7fff_5a11;

    /// Script `v0` for calls to `target`.
    fn oc375_script(c: &mut Cpu, target: u32, v0: u32) {
        for i in 0..16 {
            let t = c.bus.read32(OC375_TAB + 8 * i);
            if t == 0 || t == target {
                c.bus.write32(OC375_TAB + 8 * i, target);
                c.bus.write32(OC375_TAB + 8 * i + 4, v0);
                return;
            }
        }
        panic!("oc375 script full");
    }

    /// Fake machine: the gate on, the fakes hooked.
    fn oc375_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc375_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC375_CTRL, OC375_MAGIC);
        r
    }

    macro_rules! oc375_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC375_CTRL) == OC375_MAGIC {
                    return oc375_fake(c, $addr);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    oc375_gated!(
        oc375_fake_stream_queue,
        OC375_PREV_STREAM_QUEUE,
        0x002a_ad78,
        ee::s08::f_002aad78__generated
    );
    oc375_gated!(
        oc375_fake_radius_vanish,
        OC375_PREV_RADIUS_VANISH,
        0x002a_9988,
        ee::s08::f_002a9988__generated
    );
    oc375_gated!(
        oc375_fake_closer_view,
        OC375_PREV_CLOSER_VIEW,
        0x0028_8940,
        ee::s07::f_00288940__generated
    );
    oc375_gated!(
        oc375_fake_get_be,
        OC375_PREV_GET_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    oc375_gated!(
        oc375_fake_get_interface,
        OC375_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    oc375_gated!(
        oc375_fake_replay_playing,
        OC375_PREV_REPLAY_PLAYING,
        0x0028_8ae0,
        ee::s07::f_00288ae0__generated
    );
    oc375_gated!(
        oc375_fake_bank_add,
        OC375_PREV_BANK_ADD,
        0x002a_cfe0,
        ee::s08::f_002acfe0__generated
    );
    oc375_gated!(
        oc375_fake_get_stream,
        OC375_PREV_GET_STREAM,
        0x002a_b188,
        ee::s08::f_002ab188__generated
    );
    oc375_gated!(
        oc375_fake_set_stream,
        OC375_PREV_SET_STREAM,
        0x002a_bcf8,
        ee::s08::f_002abcf8__generated
    );
    oc375_gated!(
        oc375_fake_match_conditions,
        OC375_PREV_MATCH_CONDITIONS,
        0x003d_85c8,
        ee::s14::f_003d85c8__generated
    );
    oc375_gated!(
        oc375_fake_sample_allowed,
        OC375_PREV_SAMPLE_ALLOWED,
        0x003d_8780,
        ee::s14::f_003d8780__generated
    );
    oc375_gated!(
        oc375_fake_history_find,
        OC375_PREV_HISTORY_FIND,
        0x003d_6e30,
        ee::s14::f_003d6e30__generated
    );
    oc375_gated!(
        oc375_fake_mc_queue_push,
        OC375_PREV_MC_QUEUE_PUSH,
        0x0030_c7c0,
        ee::s10::f_0030c7c0__generated
    );
    oc375_gated!(
        oc375_fake_mc_current,
        OC375_PREV_MC_CURRENT,
        0x0030_aeb8,
        ee::s10::f_0030aeb8__generated
    );
    oc375_gated!(
        oc375_fake_mc_find,
        OC375_PREV_MC_FIND,
        0x0030_b388,
        ee::s10::f_0030b388__generated
    );
    oc375_gated!(
        oc375_fake_mc_type2,
        OC375_PREV_MC_TYPE2,
        0x0030_b8c0,
        ee::s10::f_0030b8c0__generated
    );
    oc375_gated!(
        oc375_fake_mc_find_by_id,
        OC375_PREV_MC_FIND_BY_ID,
        0x0030_b320,
        ee::s10::f_0030b320__generated
    );
    oc375_gated!(
        oc375_fake_mc_sound_sys,
        OC375_PREV_MC_SOUND_SYS,
        0x0028_b180,
        ee::s07::f_0028b180__generated
    );
    oc375_gated!(
        oc375_fake_mc_abort_sound,
        OC375_PREV_MC_ABORT_SOUND,
        0x0029_dbb0,
        ee::s08::f_0029dbb0__generated
    );
    oc375_gated!(
        oc375_fake_mc_be_q1,
        OC375_PREV_MC_BE_Q1,
        0x0015_3e28,
        ee::s01::f_00153e28__generated
    );
    oc375_gated!(
        oc375_fake_mc_be_q2,
        OC375_PREV_MC_BE_Q2,
        0x0015_3dd0,
        ee::s01::f_00153dd0__generated
    );
    oc375_gated!(
        oc375_fake_mc_be_q3,
        OC375_PREV_MC_BE_Q3,
        0x0015_3ed8,
        ee::s01::f_00153ed8__generated
    );
    oc375_gated!(
        oc375_fake_mc_be_q4,
        OC375_PREV_MC_BE_Q4,
        0x0015_3fd8,
        ee::s01::f_00153fd8__generated
    );
    oc375_gated!(
        oc375_fake_mc_be_q5,
        OC375_PREV_MC_BE_Q5,
        0x0015_3e80,
        ee::s01::f_00153e80__generated
    );
    oc375_gated!(
        oc375_fake_mc_be_q6,
        OC375_PREV_MC_BE_Q6,
        0x0015_3d78,
        ee::s01::f_00153d78__generated
    );
    oc375_gated!(
        oc375_fake_find_event,
        OC375_PREV_FIND_EVENT,
        0x003d_7110,
        ee::s14::f_003d7110__generated
    );
    oc375_gated!(
        oc375_fake_find_bank_user_data,
        OC375_PREV_FIND_BANK_USER_DATA,
        0x003d_81f0,
        ee::s14::f_003d81f0__generated
    );
    oc375_gated!(
        oc375_fake_eval_part_choice,
        OC375_PREV_EVAL_PART_CHOICE,
        0x003d_b208,
        ee::s14::f_003db208__generated
    );
    oc375_gated!(
        oc375_fake_pick_weighted,
        OC375_PREV_PICK_WEIGHTED,
        0x003d_8cb0,
        ee::s14::f_003d8cb0__generated
    );
    oc375_gated!(
        oc375_fake_random_no_repeat,
        OC375_PREV_RANDOM_NO_REPEAT,
        0x003d_b5c0,
        ee::s14::f_003db5c0__generated
    );
    oc375_gated!(
        oc375_fake_check_event_masks,
        OC375_PREV_CHECK_EVENT_MASKS,
        0x003d_9ed0,
        ee::s14::f_003d9ed0__generated
    );
    oc375_gated!(
        oc375_fake_resolve_event_parts,
        OC375_PREV_RESOLVE_EVENT_PARTS,
        0x003d_95b8,
        ee::s14::f_003d95b8__generated
    );
    oc375_gated!(
        oc375_fake_sentence_append,
        OC375_PREV_SENTENCE_APPEND,
        0x003d_9ac8,
        ee::s14::f_003d9ac8__generated
    );
    oc375_gated!(
        oc375_fake_match_flags,
        OC375_PREV_MATCH_FLAGS,
        0x003d_b3b8,
        ee::s14::f_003db3b8__generated
    );
    oc375_gated!(
        oc375_fake_find_bank_by_id,
        OC375_PREV_FIND_BANK_BY_ID,
        0x003d_6d50,
        ee::s14::f_003d6d50__generated
    );
    oc375_gated!(
        oc375_fake_notify_part,
        OC375_PREV_NOTIFY_PART,
        0x003d_b040,
        ee::s14::f_003db040__generated
    );
    oc375_gated!(
        oc375_fake_find_bank_type,
        OC375_PREV_FIND_BANK_TYPE,
        0x003d_8330,
        ee::s14::f_003d8330__generated
    );
    oc375_gated!(
        oc375_fake_decode_sample,
        OC375_PREV_DECODE_SAMPLE,
        0x003d_b838,
        ee::s14::f_003db838__generated
    );
    oc375_gated!(
        oc375_fake_notify_play,
        OC375_PREV_NOTIFY_PLAY,
        0x003d_9830,
        ee::s14::f_003d9830__generated
    );
    oc375_gated!(
        oc375_fake_reset_sentence,
        OC375_PREV_RESET_SENTENCE,
        0x003d_9a40,
        ee::s14::f_003d9a40__generated
    );
    oc375_gated!(
        oc375_fake_get_time,
        OC375_PREV_GET_TIME,
        0x003d_b9a0,
        ee::s14::f_003db9a0__generated
    );
    oc375_gated!(
        oc375_fake_free_queue_slot,
        OC375_PREV_FREE_QUEUE_SLOT,
        0x003d_7c38,
        ee::s14::f_003d7c38__generated
    );
    oc375_gated!(
        oc375_fake_alloc_queue_slot,
        OC375_PREV_ALLOC_QUEUE_SLOT,
        0x003d_72a0,
        ee::s14::f_003d72a0__generated
    );
    oc375_gated!(
        oc375_fake_instance_slot,
        OC375_PREV_INSTANCE_SLOT,
        0x002b_1220,
        ee::s08::f_002b1220__generated
    );
    oc375_gated!(
        oc375_fake_current_request,
        OC375_PREV_CURRENT_REQUEST,
        0x002a_b150,
        ee::s08::f_002ab150__generated
    );
    oc375_gated!(
        oc375_fake_add_timer,
        OC375_PREV_ADD_TIMER,
        0x002a_dca0,
        ee::s08::f_002adca0__generated
    );
    oc375_gated!(
        oc375_fake_volume_scale,
        OC375_PREV_VOLUME_SCALE,
        0x002a_b0d0,
        ee::s08::f_002ab0d0__generated
    );
    oc375_gated!(
        oc375_fake_clear_prebuffer,
        OC375_PREV_CLEAR_PREBUFFER,
        0x002b_1248,
        ee::s08::f_002b1248__generated
    );
    oc375_gated!(
        oc375_fake_build_nodes,
        OC375_PREV_BUILD_NODES,
        0x0035_c0e8,
        ee::s11::f_0035c0e8__generated
    );
    oc375_gated!(
        oc375_fake_update_bounds,
        OC375_PREV_UPDATE_BOUNDS,
        0x0035_c040,
        ee::s11::f_0035c040__generated
    );
    oc375_gated!(
        oc375_fake_get_nash,
        OC375_PREV_GET_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated
    );
    oc375_gated!(
        oc375_fake_world_instance,
        OC375_PREV_WORLD_INSTANCE,
        0x002d_1bd8,
        ee::s09::f_002d1bd8__generated
    );
    oc375_gated!(
        oc375_fake_key_from_aabb,
        OC375_PREV_KEY_FROM_AABB,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );
    oc375_gated!(
        oc375_fake_hash_remove,
        OC375_PREV_HASH_REMOVE,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );
    oc375_gated!(
        oc375_fake_delete,
        OC375_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    oc375_gated!(
        oc375_fake_alloc_named,
        OC375_PREV_ALLOC_NAMED,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    //@FAKES@

    fn oc375_install_fakes() {
        use std::ptr::addr_of_mut;
        // SAFETY: override slots are plain fn pointers; `hook` serialises.
        {
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AAD78),
                oc375_fake_stream_queue,
                &OC375_PREV_STREAM_QUEUE,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002A9988),
                oc375_fake_radius_vanish,
                &OC375_PREV_RADIUS_VANISH,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00288940),
                oc375_fake_closer_view,
                &OC375_PREV_CLOSER_VIEW,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                oc375_fake_get_be,
                &OC375_PREV_GET_BE,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                oc375_fake_get_interface,
                &OC375_PREV_GET_INTERFACE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00288AE0),
                oc375_fake_replay_playing,
                &OC375_PREV_REPLAY_PLAYING,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002ACFE0),
                oc375_fake_bank_add,
                &OC375_PREV_BANK_ADD,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AB188),
                oc375_fake_get_stream,
                &OC375_PREV_GET_STREAM,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002ABCF8),
                oc375_fake_set_stream,
                &OC375_PREV_SET_STREAM,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D85C8),
                oc375_fake_match_conditions,
                &OC375_PREV_MATCH_CONDITIONS,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D8780),
                oc375_fake_sample_allowed,
                &OC375_PREV_SAMPLE_ALLOWED,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D6E30),
                oc375_fake_history_find,
                &OC375_PREV_HISTORY_FIND,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030C7C0),
                oc375_fake_mc_queue_push,
                &OC375_PREV_MC_QUEUE_PUSH,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030AEB8),
                oc375_fake_mc_current,
                &OC375_PREV_MC_CURRENT,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B388),
                oc375_fake_mc_find,
                &OC375_PREV_MC_FIND,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B8C0),
                oc375_fake_mc_type2,
                &OC375_PREV_MC_TYPE2,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B320),
                oc375_fake_mc_find_by_id,
                &OC375_PREV_MC_FIND_BY_ID,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0028B180),
                oc375_fake_mc_sound_sys,
                &OC375_PREV_MC_SOUND_SYS,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_0029DBB0),
                oc375_fake_mc_abort_sound,
                &OC375_PREV_MC_ABORT_SOUND,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00153E28),
                oc375_fake_mc_be_q1,
                &OC375_PREV_MC_BE_Q1,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00153DD0),
                oc375_fake_mc_be_q2,
                &OC375_PREV_MC_BE_Q2,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00153ED8),
                oc375_fake_mc_be_q3,
                &OC375_PREV_MC_BE_Q3,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00153FD8),
                oc375_fake_mc_be_q4,
                &OC375_PREV_MC_BE_Q4,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00153E80),
                oc375_fake_mc_be_q5,
                &OC375_PREV_MC_BE_Q5,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00153D78),
                oc375_fake_mc_be_q6,
                &OC375_PREV_MC_BE_Q6,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D7110),
                oc375_fake_find_event,
                &OC375_PREV_FIND_EVENT,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D81F0),
                oc375_fake_find_bank_user_data,
                &OC375_PREV_FIND_BANK_USER_DATA,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DB208),
                oc375_fake_eval_part_choice,
                &OC375_PREV_EVAL_PART_CHOICE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D8CB0),
                oc375_fake_pick_weighted,
                &OC375_PREV_PICK_WEIGHTED,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DB5C0),
                oc375_fake_random_no_repeat,
                &OC375_PREV_RANDOM_NO_REPEAT,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D9ED0),
                oc375_fake_check_event_masks,
                &OC375_PREV_CHECK_EVENT_MASKS,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D95B8),
                oc375_fake_resolve_event_parts,
                &OC375_PREV_RESOLVE_EVENT_PARTS,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D9AC8),
                oc375_fake_sentence_append,
                &OC375_PREV_SENTENCE_APPEND,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DB3B8),
                oc375_fake_match_flags,
                &OC375_PREV_MATCH_FLAGS,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D6D50),
                oc375_fake_find_bank_by_id,
                &OC375_PREV_FIND_BANK_BY_ID,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DB040),
                oc375_fake_notify_part,
                &OC375_PREV_NOTIFY_PART,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D8330),
                oc375_fake_find_bank_type,
                &OC375_PREV_FIND_BANK_TYPE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DB838),
                oc375_fake_decode_sample,
                &OC375_PREV_DECODE_SAMPLE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D9830),
                oc375_fake_notify_play,
                &OC375_PREV_NOTIFY_PLAY,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D9A40),
                oc375_fake_reset_sentence,
                &OC375_PREV_RESET_SENTENCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DB9A0),
                oc375_fake_get_time,
                &OC375_PREV_GET_TIME,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D7C38),
                oc375_fake_free_queue_slot,
                &OC375_PREV_FREE_QUEUE_SLOT,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D72A0),
                oc375_fake_alloc_queue_slot,
                &OC375_PREV_ALLOC_QUEUE_SLOT,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002B1220),
                oc375_fake_instance_slot,
                &OC375_PREV_INSTANCE_SLOT,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AB150),
                oc375_fake_current_request,
                &OC375_PREV_CURRENT_REQUEST,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
                oc375_fake_add_timer,
                &OC375_PREV_ADD_TIMER,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AB0D0),
                oc375_fake_volume_scale,
                &OC375_PREV_VOLUME_SCALE,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002B1248),
                oc375_fake_clear_prebuffer,
                &OC375_PREV_CLEAR_PREBUFFER,
            );
            fakes::hook(
                addr_of_mut!(ee::s11::OVERRIDE_0035C0E8),
                oc375_fake_build_nodes,
                &OC375_PREV_BUILD_NODES,
            );
            fakes::hook(
                addr_of_mut!(ee::s11::OVERRIDE_0035C040),
                oc375_fake_update_bounds,
                &OC375_PREV_UPDATE_BOUNDS,
            );
            fakes::hook(
                addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
                oc375_fake_get_nash,
                &OC375_PREV_GET_NASH,
            );
            fakes::hook(
                addr_of_mut!(ee::s09::OVERRIDE_002D1BD8),
                oc375_fake_world_instance,
                &OC375_PREV_WORLD_INSTANCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00328F28),
                oc375_fake_key_from_aabb,
                &OC375_PREV_KEY_FROM_AABB,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_003284B8),
                oc375_fake_hash_remove,
                &OC375_PREV_HASH_REMOVE,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00317E50),
                oc375_fake_delete,
                &OC375_PREV_DELETE,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00317D70),
                oc375_fake_alloc_named,
                &OC375_PREV_ALLOC_NAMED,
            );
            //@HOOKS@
        }
    }

    /// SSXAudioSystem_PlayStream: the queue fails or succeeds; radius
    /// positive, zero/negative with and without the engine flag; with or
    /// without a target; the two-player mirror check through each exit; the
    /// bank monitor returning an instance or none. All callees but memset
    /// are fakes.
    #[test]
    fn ssx_audio_system_play_stream_matches() {
        covered(
            ee::s07::f_002854f8__generated,
            ssx_audio_system_play_stream,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x0028_54f8, 8);
                c.pc = 0x0028_54f8;
                let sys = DATA + 0x100;
                let pp = DATA + 0x200;
                let arr = DATA + 0x1000;
                let params = arr + PLAY_STREAM_PARAMS;
                c.bus.write32(sys + PLAY_STREAM_ARRAY, pp);
                c.bus.write32(pp, arr);
                fill(c, &mut r, arr + 0x1d8, 0x30);
                c.bus.write32(arr + PLAY_STREAM_TOP, r.next_u32() % 3);
                // Per-level arrays.
                for (i, off) in [
                    0x34u32, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus
                        .write32(params + off, DATA + 0x2000 + 0x100 * i as u32);
                }
                c.bus.write32(params + 0x20, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                c.g[A0][0] = sx(sys);
                c.g[T0][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T3][0] = if seed % 2 == 0 { 0 } else { 1 };
                c.f[12] = match seed % 4 {
                    0 => 0,
                    1 => 0xbf80_0000,
                    _ => r.f32(500.0) & 0x7fff_ffff,
                };
                c.bus.write32(GAME_GP - 0x465c, 0x41ef_ff88);
                c.bus.write32(crate::difftest::SP, r.next_u32());
                c.bus.write8(0x0053_5c11, if seed % 5 == 0 { 1 } else { 2 });
                oc375_script(
                    c,
                    0x002a_ad78,
                    if seed % 7 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 50
                    },
                );
                oc375_script(c, 0x0014_dd58, (seed / 2 % 4 != 0) as u32);
                oc375_script(c, 0x0028_8ae0, (seed / 3 % 3 == 0) as u32);
                oc375_script(
                    c,
                    0x002a_cfe0,
                    if seed / 5 % 3 == 0 { 0 } else { DATA + 0x5000 },
                );
            },
        );
    }

    /// SPCH_ChooseSample: condition blocks with and without mark slots
    /// (none, -1 count), mask words that fail or pass, samples that match,
    /// are allowed, new or old (history age), a full or open set, and the
    /// fallback to the oldest. The three callees are fakes.
    #[test]
    fn spch_choose_sample_matches() {
        covered(
            ee::s14::f_003d88c8__generated,
            spch_choose_sample,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x003d_88c8, 12);
                c.pc = 0x003d_88c8;
                let set = DATA + 0x100;
                let sel = DATA + 0x4000;
                let event = DATA + 0x500;
                let table = DATA + 0x600;
                let cond = DATA + 0x700;
                let tbl = DATA + 0x2000;
                let header = DATA + 0x800;
                fill(c, &mut r, set, 0x80);
                c.bus
                    .write8(set + SPCH_SET_COUNT, (196 + r.next_u32() % 6) as u8);
                fill(c, &mut r, sel, 0x200);
                c.bus.write16(event, 1);
                c.bus.write32(GAME_GP + 0x1734, table);
                c.bus.write32(table + 4, header);
                // Header: cond count n (low nibble), sample count, entries
                // from +0xe (stride n + 2), the n mask words after them.
                fill(c, &mut r, header, 0x40);
                let n = if seed % 5 == 0 { 0 } else { r.next_u32() % 4 };
                c.bus.write8(header + 4, (0x30 | n) as u8);
                c.bus.write8(header + 5, (r.next_u32() % 6) as u8);
                // Condition ids: none, ignored (0, 0xff), mark slots, or ids.
                fill(c, &mut r, cond, 8);
                c.bus.write8(
                    cond + SPCH_COND_COUNT,
                    ((r.next_u32() % 6) as u8).wrapping_sub(1),
                );
                for i in 0..8 {
                    let id = match r.next_u32() % 5 {
                        0 => 0,
                        1 => 0xff,
                        2 => 0xfe,
                        _ => 1 + r.next_u32() % 0x20,
                    };
                    c.bus.write8(cond + SPCH_COND_IDS + i, id as u8);
                }
                fill(c, &mut r, tbl, 0x40);
                if seed % 4 == 0 {
                    for _ in 0..8 {
                        c.bus.write32(tbl + 4 * (r.next_u32() % 0x21), 0);
                    }
                }
                c.g[A0][0] = sx(set);
                c.g[A1][0] = sx(sel);
                c.g[A2][0] = sx(event);
                c.g[A3][0] = sx(cond);
                c.g[T0][0] = sx(tbl);
                oc375_script(c, 0x003d_85c8, if seed % 9 == 0 { 0 } else { OC375_COIN });
                oc375_script(c, 0x003d_8780, OC375_COIN);
                oc375_script(c, 0x003d_6e30, OC375_SMALL);
            },
        );
    }

    /// WScriptMan_MissionCommand: every command 0..21 (both jump tables are
    /// written), forced (15/19) or gated by the game mode and the race,
    /// with a context or -1, the event queue busy or free, mission id or
    /// -1 (current mission or none), lookups that find the mission or not,
    /// and the BE queries' answers. ProcessCommand runs for real (with no
    /// process to touch); cmd 3's sound call (0x29d6e0, no override slot)
    /// is never reached: the fake before it unwinds.
    #[test]
    fn wscript_man_mission_command_matches() {
        covered(
            ee::s10::f_0030a868__generated,
            wscript_man_mission_command,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x0030_a868, 10);
                c.pc = 0x0030_a868;
                put_words(
                    c,
                    MISSION_CMD_TABLE1,
                    &[
                        0x30a938, 0x30a938, 0x30a938, 0x30a988, 0x30a988, 0x30a988, 0x30a988,
                        0x30a988, 0x30a950, 0x30a950, 0x30a988, 0x30a950, 0x30a988, 0x30a988,
                        0x30a988, 0x30a988, 0x30a988, 0x30a988, 0x30a938,
                    ],
                );
                put_words(
                    c,
                    MISSION_CMD_TABLE2,
                    &[
                        0x30ac68, 0x30ac68, 0x30ac68, 0x30a9d0, 0x30aa3c, 0x30aa70, 0x30ac68,
                        0x30ac68, 0x30ac68, 0x30ac68, 0x30ac68, 0x30ac68, 0x30ab50, 0x30ab20,
                        0x30ac0c, 0x30ac3c, 0x30ab80, 0x30aac4, 0x30ac68, 0x30abd8,
                    ],
                );
                let man = DATA + 0x100;
                let world = DATA + 0x400;
                let p = DATA + 0x500;
                let q = DATA + 0x600;
                let busy = DATA + 0x700;
                let obj = DATA + 0x3000;
                c.bus.write32(man, r.next_u32());
                c.bus.write32(man + 0x2a4, 0);
                c.bus
                    .write32(man + MISSION_CMD_MODE, (seed / 7 % 3 == 0) as u32);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, p);
                c.bus.write32(p + 0xc, q);
                c.bus.write32(q + 0x7c, (seed % 11 != 0) as u32);
                c.bus.write32(p + 0x28, busy);
                c.bus.write32(busy, [0, 5, 20][(seed / 3 % 3) as usize]);
                c.bus
                    .write8(0x0053_5c10, if seed % 13 == 0 { 3 } else { 4 });
                let vt = fakes::vtable(c, &mut r, &[0x88, 0x10, 0x98, 0xa0]);
                c.bus.write32(obj + MISSION_CMD_VT, vt);
                c.bus.write32(
                    obj + MISSION_CMD_ID,
                    if seed / 5 % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 40
                    },
                );
                let cmd = seed % 22;
                let any_id = seed / 22 % 3 != 0;
                let current = seed / 2 % 4 != 0;
                let found = seed / 4 % 3 != 0;
                c.g[A0][0] = sx(man);
                c.g[A1][0] = if any_id {
                    (r.next_u32() % 40) as u64
                } else {
                    u64::MAX
                };
                c.g[A2][0] = cmd as u64;
                c.g[A3][0] = if seed % 2 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() % 8) as u64
                };
                oc375_script(c, 0x0030_aeb8, if current { obj } else { 0 });
                oc375_script(c, 0x0030_b388, if found { obj } else { 0 });
                oc375_script(c, 0x0030_b320, 0);
                oc375_script(c, 0x0030_b8c0, OC375_COIN);
                for query in [
                    0x0015_3e28,
                    0x0015_3dd0,
                    0x0015_3ed8,
                    0x0015_3e80,
                    0x0015_3d78,
                ] {
                    oc375_script(c, query, OC375_COIN);
                }
                oc375_script(c, 0x0015_3fd8, OC375_SMALL);
                // Cmd 3 with a mission: unwind at its method or the sound-system
                // getter, so the real 0x29d6e0 never runs.
                if cmd == 3 {
                    let at = (if any_id { 6 } else { 5 }) + seed / 3 % 2;
                    c.bus.write32(fakes::LOG_UNWIND_AT, at);
                }
            },
        );
    }

    /// A speech sample table for the real SPCH_ChooseSample: `gp+0x1734`
    /// -> table, entry 1 -> a header with no condition masks and 1-3
    /// samples (callee fakes decide match/allowed/history).
    fn oc375_sample_table(c: &mut Cpu, r: &mut Rng) {
        let table = DATA + 0x6000;
        let header = DATA + 0x6100;
        c.bus.write32(GAME_GP + 0x1734, table);
        c.bus.write32(table + 4, header);
        fill(c, r, header, 0x10);
        c.bus.write8(header + 4, 0x30);
        c.bus.write8(header + 5, (1 + r.next_u32() % 3) as u8);
    }

    /// SPCH_BuildSentence: no event / no bank, the bank's last event
    /// repeating within its limit or not, bank modes 0/1/2 (with and
    /// without flag bit 0), no parts, argument masks on (20 tests over
    /// args of 0, -1 and others) or off, parts whose roll, masks or mask
    /// word fail, and parts that go through the real ChooseEventSamples
    /// (returning 0 or a choice, then resolved or not). Callees with
    /// override slots are fakes; FindBankField14 runs on an empty table.
    #[test]
    fn spch_build_sentence_matches() {
        covered(
            ee::s14::f_003d9fb0__generated,
            spch_build_sentence,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x003d_9fb0, 14);
                c.pc = 0x003d_9fb0;
                let frame = crate::difftest::SP - 0x280;
                let args = DATA + 0x100;
                let ev = DATA + 0x1000;
                for i in 0..22 {
                    let v = match r.next_u32() % 3 {
                        0 => 0,
                        1 => u32::MAX,
                        _ => r.next_u32(),
                    };
                    c.bus.write32(args + 4 * i, v);
                }
                // Bank index, its last event and limit, its mode.
                let bank = r.next_u32() % 4;
                c.bus.write32(frame + 0x1b4, bank);
                let id = r.next_u32() % 3;
                c.bus.write16(
                    SPCH_BANK_LAST + 8 * bank,
                    (if seed % 3 == 0 { id } else { 7 }) as u16,
                );
                c.bus
                    .write32(SPCH_BANK_LAST + 8 * bank + 4, r.next_u32() % 4);
                c.bus.write32(SPCH_BANK_MODE + 4 * bank, r.next_u32() % 3);
                // The event: id, repeat count, flags, parts at +0xc.
                fill(c, &mut r, ev, 0x80);
                let parts = if seed % 17 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write16(ev, id as u16);
                c.bus.write8(ev + SPCH_EVENT_PARTS, parts as u8);
                c.bus
                    .write8(ev + 7, (r.next_u32() % 4 | (r.next_u32() % 4) << 4) as u8);
                c.bus.write8(ev + 8, (r.next_u32() % 5) as u8);
                c.bus
                    .write8(ev + SPCH_EVENT_FLAGS, (r.next_u32() % 16) as u8);
                for i in 0..4 {
                    c.bus.write16(ev + 0xc + 2 * i, (0x10 + 0x10 * i) as u16);
                }
                // Parts (ChooseEventSamples headers) at ev + 0x40 * (i + 1).
                for i in 0..4 {
                    let part = ev + 0x40 * (i + 1);
                    c.bus.write8(part + 1, (r.next_u32() % 101) as u8);
                    c.bus.write8(part + 4, (1 + r.next_u32() % 2) as u8 * 4);
                    c.bus.write8(part + 5, (r.next_u32() % 3) as u8);
                    c.bus.write8(part + 6, 0);
                    for k in 0..4 {
                        c.bus.write8(part + 8 + k, 4 + k as u8);
                    }
                    for k in 0..4 {
                        let e = part + 0x10 + 4 * k;
                        c.bus
                            .write8(e + 3, if r.next_u32() % 3 == 0 { 3 } else { 0 });
                        c.bus.write8(e + 4, 0);
                    }
                }
                // The weight order at sp+0.
                for i in 0..8 {
                    c.bus.write8(frame + i, (r.next_u32() % 5) as u8);
                }
                oc375_sample_table(c, &mut r);
                oc375_script(c, 0x003d_7110, if seed % 19 == 0 { 0 } else { ev });
                oc375_script(c, 0x003d_81f0, if seed % 23 == 0 { 0 } else { 1 });
                oc375_script(c, 0x003d_b5c0, r.next_u32() % 100);
                oc375_script(c, 0x003d_9ed0, OC375_COIN);
                oc375_script(c, 0x003d_95b8, OC375_COIN);
                oc375_script(c, 0x003d_b3b8, if seed % 5 == 0 { 0 } else { 1 });
                oc375_script(c, 0x003d_6d50, 1);
                oc375_script(c, 0x003d_85c8, 1);
                oc375_script(c, 0x003d_8780, 1);
                oc375_script(c, 0x003d_6e30, OC375_SMALL);
                c.g[A0][0] = sx(args);
            },
        );
    }

    /// SPCH_ChooseEventSamples: too many slots, each mode-bit/mode pair,
    /// flags that match or not; slots whose mark conditions fail (cleared
    /// mark words: rounds until the 200 tries run out) or hold, done or
    /// not; banks by id (found or none), by argument (FindBank, argument
    /// index in range or not) and by id with a sample check
    /// (BankHasSample), and the real SPCH_ChooseSample choosing samples
    /// or none. FindBank and BankHasSample run for real on a 3-bank table.
    #[test]
    fn spch_choose_event_samples_matches() {
        covered(
            ee::s14::f_003d9088__generated,
            spch_choose_event_samples,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x003d_9088, 10);
                c.pc = 0x003d_9088;
                let set = DATA + 0x100;
                let part = DATA + 0x400;
                let args = DATA + 0x800;
                // Banks: three sample headers (id 0x40+i, arg key i+5,
                // 3 samples by index), the count at gp+0x1738.
                let table = DATA + 0x6000;
                c.bus.write32(GAME_GP + 0x1734, table);
                c.bus.write32(GAME_GP + 0x1738, 3);
                for i in 0..3u32 {
                    let h = DATA + 0x6100 + 0x80 * i;
                    c.bus.write32(table + 4 * i, h);
                    fill(c, &mut r, h, 0x20);
                    c.bus.write16(h, (0x40 + i) as u16);
                    c.bus.write16(h + 2, (5 + i) as u16);
                    c.bus.write8(h + 4, (0x30 | r.next_u32() % 2) as u8);
                    c.bus.write8(h + 5, (1 + r.next_u32() % 3) as u8);
                    c.bus.write16(h + 0xa, 3);
                }
                fill(c, &mut r, set, 0x20);
                c.bus.write8(set + 0x61, (r.next_u32() % 5) as u8);
                for i in 0..22 {
                    let v = match r.next_u32() % 3 {
                        0 => 5 + r.next_u32() % 4,
                        1 => r.next_u32() % 6,
                        _ => r.next_u32(),
                    };
                    c.bus.write32(args + 4 * i, v);
                }
                // The part: slot count / mode bits, words to clear, its
                // conditions at part + 0x20 * (k + 1).
                fill(c, &mut r, part, 0x100);
                let n = if seed % 13 == 0 {
                    13 + r.next_u32() % 3
                } else {
                    r.next_u32() % 5
                };
                c.bus.write8(part + 4, (n << 2 | r.next_u32() % 4) as u8);
                c.bus.write8(part + 5, (r.next_u32() % 4) as u8);
                c.bus.write8(part + 6, (r.next_u32() % 3) as u8);
                for k in 0..4u32 {
                    c.bus.write8(part + 8 + k, (8 * (k + 1)) as u8);
                    let cond = part + 0x20 * (k + 1);
                    c.bus.write16(cond, (0x40 + r.next_u32() % 4) as u16);
                    c.bus.write8(
                        cond + 2,
                        (if r.next_u32() % 5 == 0 {
                            0x15
                        } else {
                            r.next_u32() % 22
                        }) as u8,
                    );
                    c.bus.write8(cond + 3, (r.next_u32() % 4) as u8);
                    c.bus
                        .write8(cond + 4, ((r.next_u32() % 4) as u8).wrapping_sub(1));
                    for j in 0..3 {
                        let id = if r.next_u32() % 2 == 0 {
                            0xff
                        } else {
                            r.next_u32() % 0x20
                        };
                        c.bus.write8(cond + 8 + j, id as u8);
                        let target =
                            r.next_u32() % 8 | (if r.next_u32() % 3 == 0 { 0 } else { 0x80 });
                        c.bus.write8(cond + 0x10 + j, target as u8);
                    }
                }
                c.g[A0][0] = sx(set);
                c.g[A1][0] = sx(part);
                c.g[A2][0] = sx(args);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = (r.next_u32() % 3) as u64;
                oc375_script(c, 0x003d_b3b8, if seed % 7 == 0 { 0 } else { 1 });
                oc375_script(c, 0x003d_6d50, OC375_SMALL);
                oc375_script(c, 0x003d_85c8, if seed % 4 == 0 { OC375_COIN } else { 1 });
                oc375_script(c, 0x003d_8780, 1);
                oc375_script(c, 0x003d_6e30, OC375_SMALL);
            },
        );
    }

    /// SPCH_PlaySentence: another bank's sentence or no parts; 1-3 parts
    /// with and without the play callback, part headers with 0-3 slots,
    /// sample ids 0xff (none) or decoded / not decoded, and the first
    /// part's key repeating the bank's last event or replacing it.
    /// GetEventParam runs for real; the callback is a fake address.
    #[test]
    fn spch_play_sentence_matches() {
        covered(
            ee::s14::f_003d9bd8__generated,
            spch_play_sentence,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x003d_9bd8, 12);
                c.pc = 0x003d_9bd8;
                let bank = r.next_u32() % 4;
                let event = DATA + 0x2000;
                fill(c, &mut r, event, 0x40);
                c.bus.write8(event + 6, (r.next_u32() % 4) as u8);
                c.bus
                    .write32(SPCH_SENTENCE, if seed % 9 == 0 { bank + 1 } else { bank });
                let parts = if seed % 11 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 4
                };
                c.bus.write32(SPCH_SENTENCE + 4, parts);
                // Chosen slots: small banks, any samples.
                for k in 0..16 {
                    c.bus
                        .write16(SPCH_SENTENCE + 0xc + 8 * k, (r.next_u32() % 3) as u16);
                    c.bus
                        .write16(SPCH_SENTENCE + 0x10 + 8 * k, r.next_u32() as u16);
                }
                for i in 0..3 {
                    let at = SPCH_SENTENCE + SPCH_SENTENCE_PART * i;
                    let header = DATA + 0x3000 + 0x100 * i;
                    c.bus.write8(at + 0x194, r.next_u32() as u8);
                    c.bus.write32(at + 0x18c, event);
                    c.bus.write32(at + 0x190, header);
                    c.bus.write32(
                        at + 0x198,
                        if r.next_u32() % 2 == 0 {
                            7
                        } else {
                            r.next_u32()
                        },
                    );
                    fill(c, &mut r, header, 0x20);
                    c.bus.write8(header + 4, ((r.next_u32() % 4) << 2) as u8);
                    for k in 0..4 {
                        c.bus.write8(header + 8 + k, (r.next_u32() % 4) as u8);
                    }
                    for k in 0..0x14 {
                        let id = if r.next_u32() % 3 == 0 {
                            0xff
                        } else {
                            r.next_u32() % 8
                        };
                        c.bus.write8(header + 0xc + k, id as u8);
                    }
                }
                c.bus.write16(SPCH_BANK_LAST + 8 * bank, 7);
                c.bus.write32(
                    SPCH_PLAY_CALLBACK,
                    if seed % 3 == 0 { 0 } else { fakes::FAKE },
                );
                let table = DATA + 0x6000;
                c.bus.write32(GAME_GP + 0x1734, table);
                for i in 0..3 {
                    c.bus.write32(table + 4 * i, DATA + 0x6100 + 0x80 * i);
                }
                oc375_script(c, 0x003d_b838, OC375_COIN);
                c.g[A0][0] = bank as u64;
            },
        );
    }

    /// SPCH_SelectRequest: 16 queued requests, free or in use, of this
    /// bank or another, busy or idle, with or without an id list (empty,
    /// containing the id or not), lifetimes that expire (freed) or not,
    /// mode-1 banks with flagged events whose key finds a bank entry of
    /// higher or lower priority, and ties in priority, age and stamp.
    #[test]
    fn spch_select_request_matches() {
        covered(
            ee::s14::f_003d7760__generated,
            spch_select_request,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x003d_7760, 6);
                c.pc = 0x003d_7760;
                let bank = r.next_u32() % 3;
                c.bus.write32(GAME_GP + 0x174c, 0);
                oc375_script(c, 0x003d_b9a0, 1000);
                c.bus
                    .write32(SPCH_BANK_MODE + 4 * bank, if seed % 3 == 0 { 0 } else { 1 });
                for i in 0..16u32 {
                    let e = SPCH_QUEUE_BASE + SPCH_QUEUE_ENTRY * i;
                    let ev = DATA + 0x2000 + 0x20 * i;
                    c.bus.write32(e + 0x64, 1000 - r.next_u32() % 4);
                    c.bus.write16(e + 0x68, (r.next_u32() % 3) as u16);
                    c.bus.write8(e + 0x6a, (r.next_u32() % 5 != 0) as u8);
                    c.bus.write8(
                        e + 0x6b,
                        (if r.next_u32() % 4 == 0 {
                            bank + 1
                        } else {
                            bank
                        }) as u8,
                    );
                    c.bus.write16(e + 0x6c, (r.next_u32() % 6 == 0) as u16);
                    c.bus.write32(e + 0x70, ev);
                    c.bus.write32(e + 0x74, 0x0101_0000 * (r.next_u32() % 3));
                    c.bus.write16(ev, (r.next_u32() % 6) as u16);
                    c.bus.write16(
                        ev + 2,
                        (if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32() % 5
                        }) as u16,
                    );
                    c.bus.write16(ev + 4, (r.next_u32() % 3) as u16);
                    c.bus.write8(ev + 0xa, r.next_u32() as u8);
                }
                // Bank entries for keys 0x0101 / 0x0202 (bytes 2/3).
                let table = 0x0044_ffb8;
                for k in 0..8u32 {
                    let ent = DATA + 0x3000 + 0x20 * k;
                    let used = r.next_u32() % 3 != 0;
                    c.bus.write32(table + 8 * k, if used { ent } else { 0 });
                    let b = (k % 3) as u8;
                    c.bus.write8(ent + 8, b);
                    c.bus.write8(ent + 9, b);
                    c.bus.write16(ent + 0x16, (r.next_u32() % 4) as u16);
                }
                // The id list.
                let list = DATA + 0x500;
                let ids = DATA + 0x540;
                c.bus.write32(list, r.next_u32() % 4);
                c.bus.write32(list + 4, ids);
                for k in 0..4 {
                    c.bus.write16(ids + 2 * k, (r.next_u32() % 6) as u16);
                }
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { sx(list) };
                c.g[A1][0] = bank as u64;
            },
        );
    }

    /// SPCH_QueueEventArgs: re-entered (busy flag), no event, no bank
    /// entry for the key, plain events passing or missing their roll,
    /// chained events whose bank has valid or invalid newest requests, the
    /// slot allocation failing (chained: the newest is freed) or not, the
    /// same or a new time stamp, and the argument copy.
    #[test]
    fn spch_queue_event_args_matches() {
        covered(
            ee::s14::f_003d7418__generated,
            spch_queue_event_args,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x003d_7418, 6);
                c.pc = 0x003d_7418;
                let args = DATA + 0x100;
                let ev = DATA + 0x1000;
                fill(c, &mut r, args, 20);
                let k = r.next_u32() % 3;
                c.bus.write8(args + 2, k as u8);
                c.bus.write8(args + 3, k as u8);
                c.bus.write32(GAME_GP + 0x1758, (seed % 17 == 0) as u32);
                c.bus.write32(GAME_GP + 0x1750, 1000);
                c.bus.write16(GAME_GP + 0x1754, r.next_u32() as u16);
                fill(c, &mut r, ev, 4);
                c.bus.write8(ev + 9, (r.next_u32() % 100) as u8);
                c.bus.write8(ev + 0xa, if seed % 2 == 0 { 0x20 } else { 0 });
                // Bank entries (key bytes k, k) and their bank indices.
                let table = 0x0044_ffb8;
                for i in 0..8u32 {
                    let ent = DATA + 0x3000 + 0x20 * i;
                    c.bus
                        .write32(table + 8 * i, if r.next_u32() % 3 == 0 { 0 } else { ent });
                    c.bus.write32(table + 8 * i + 4, r.next_u32() % 3);
                    c.bus.write8(ent + 8, (i % 4) as u8);
                    c.bus.write8(ent + 9, (i % 4) as u8);
                }
                for b in 0..3u32 {
                    let pick = |r: &mut Rng| {
                        if r.next_u32() % 5 == 0 {
                            16 + r.next_u32() % 4
                        } else {
                            r.next_u32() % 16
                        }
                    };
                    let a = pick(&mut r);
                    let p = pick(&mut r);
                    c.bus.write32(SPCH_BANK_QUEUED + 4 * b + 0x20, a);
                    c.bus.write32(SPCH_BANK_QUEUED + 4 * b + 0x40, p);
                    c.bus.write32(SPCH_BANK_QUEUED + 4 * b, r.next_u32() % 4);
                }
                for i in 0..16u32 {
                    c.bus
                        .write32(SPCH_QUEUE_BASE + SPCH_QUEUE_ENTRY * i + 0x70, ev);
                }
                oc375_script(c, 0x003d_7110, if seed % 13 == 0 { 0 } else { ev });
                oc375_script(c, 0x003d_b5c0, r.next_u32() % 100);
                oc375_script(
                    c,
                    0x003d_72a0,
                    if seed % 5 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 16
                    },
                );
                oc375_script(c, 0x003d_b9a0, 1000 + seed % 2);
                c.g[A0][0] = sx(args);
            },
        );
    }

    /// SpeechSystem_StreamStartCallback: the instance's current request
    /// is another one (retry timer) or this one; with no stream slot or a
    /// slot whose volume method returns a scale (a fake) or 0 (a real leaf
    /// returning 0).
    #[test]
    fn speech_system_stream_start_callback_matches() {
        covered(
            ee::s08::f_002b1268__generated,
            speech_system_stream_start_callback,
            200,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x002b_1268, 5);
                c.pc = 0x002b_1268;
                let sys = DATA + 0x100;
                let msg = DATA + 0x200;
                let entry = DATA + 0x400;
                let vt = DATA + 0x600;
                let p = DATA + 0x700;
                let q = DATA + 0x740;
                fill(c, &mut r, sys, 4);
                fill(c, &mut r, entry, 4);
                c.bus.write32(entry + 0x8d0, r.next_u32());
                c.bus.write32(sys + 0x1bc, vt);
                c.bus.write16(vt + 0x18, (r.next_u32() % 0x40) as u16 & !3);
                c.bus.write32(
                    vt + 0x1c,
                    if seed % 3 == 0 {
                        0x0014_06c0
                    } else {
                        fakes::FAKE
                    },
                );
                let request = r.next_u32();
                c.bus.write32(msg + STREAM_START_ENTRY, entry);
                c.bus.write32(msg + STREAM_START_REQUEST, request);
                c.bus.write32(
                    msg + STREAM_START_SLOT,
                    if seed % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(GAME_GP + 0x6c4, p);
                c.bus.write32(p, q);
                c.bus.write32(q, DATA + 0x800);
                c.bus.write32(0x004a_37e0, r.next_u32());
                c.bus.write32(0x004a_37e4, r.next_u32());
                oc375_script(
                    c,
                    0x002a_b150,
                    if seed % 5 == 0 { request ^ 1 } else { request },
                );
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(msg);
            },
        );
    }

    /// RailModifier_ctor: the model's object keeps its own box (a fake
    /// method) or not (a real leaf returning 0: the model box grown by 100,
    /// VU0); the entry inserted into an empty hash node by the real
    /// SpatialHash insert (allocation faked); no rail track, a track with
    /// no packed pointer, or 1-3 segments whose hash nodes' bucket blocks
    /// are empty (freed) or hold a word in each position checked.
    #[test]
    fn rail_modifier_ctor_matches() {
        covered(
            ee::s11::f_0035b708__generated,
            rail_modifier_ctor,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x0035_b708, 12);
                c.pc = 0x0035_b708;
                let frame = crate::difftest::SP - 0xc0;
                let me = DATA + 0x100;
                let desc = DATA + 0x200;
                let model = DATA + 0x300;
                let obj = DATA + 0x380;
                let vt = DATA + 0x3c0;
                let nash = DATA + 0x4000;
                fill(c, &mut r, me, 0x14);
                c.bus.write32(me + RAIL_MOD_ENTRY, DATA + 0x900);
                let track_ix = r.next_u32() % 3;
                let sub = r.next_u32() % 2;
                c.bus.write32(desc + 4, track_ix | sub << 8);
                c.bus.write32(desc + 8, r.next_u32());
                c.bus.write32(model + 0xc, obj);
                c.bus.write32(obj + 0xc, vt);
                c.bus.write16(vt + 0x160, 0);
                c.bus.write32(
                    vt + 0x164,
                    if seed % 2 == 0 {
                        0x0014_06c0
                    } else {
                        fakes::FAKE
                    },
                );
                for i in 0..6 {
                    c.bus
                        .write32(model + 0x60 + 4 * i, r.f32(500.0) & 0x7fff_ffff);
                }
                // The key words the original reads back from its stack.
                for off in [4u32, 8, 0xc] {
                    c.bus.write32(frame + off, r.next_u32() % 50);
                }
                for off in [0x24u32, 0x28, 0x2c] {
                    c.bus.write32(
                        frame + off,
                        if r.next_u32() % 2 == 0 {
                            5
                        } else {
                            (-5i32) as u32
                        },
                    );
                }
                // Hash nodes: the root's slot empty, the others' bucket blocks.
                for i in 0..0x30 {
                    c.bus.write32(nash + 4 * i, 0);
                }
                for (k, off) in [0x14u32, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c]
                    .into_iter()
                    .enumerate()
                {
                    let block = DATA + 0x4800 + 0x40 * k as u32;
                    c.bus.write32(nash + off + 0x10, block);
                    for w in 0..11 {
                        c.bus.write32(block + 4 * w, 0);
                    }
                    if r.next_u32() % 3 != 0 {
                        c.bus
                            .write32(block + 4 * (r.next_u32() % 11), r.next_u32() | 1);
                    }
                }
                // World -> rail table -> track -> packed segment list.
                let world = DATA + 0x600;
                let v = DATA + 0x640;
                let a = DATA + 0x680;
                let t = DATA + 0x6c0;
                let b = DATA + 0x700;
                let track = DATA + 0x5800;
                c.bus.write32(world, v);
                c.bus.write32(v + 8, a);
                for i in 0..3 {
                    c.bus.write32(a + 4 * i, if i == 2 { 0 } else { t });
                }
                c.bus.write32(t + 0x44, b);
                c.bus.write32(b, track << 6);
                c.bus.write32(b + 4, 0x40);
                let segs = r.next_u32() % 4;
                c.bus.write32(track + 0x20, segs);
                c.bus.write32(track + 0x24, DATA + 0x5900);
                for k in 0..3u32 {
                    let seg = DATA + 0x5900 + 0x100 * k;
                    c.bus.write32(seg + 0x64, seg + 0x100);
                    for i in 0..6 {
                        c.bus.write32(seg + 0x6c + 4 * i, r.f32(500.0));
                    }
                }
                oc375_script(c, 0x002d_1be0, nash);
                oc375_script(c, 0x002d_1bd8, world);
                oc375_script(c, 0x0031_7d70, DATA + 0x5000);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(desc);
                c.g[A2][0] = sx(model);
            },
        );
    }

    //@TESTS@
}
