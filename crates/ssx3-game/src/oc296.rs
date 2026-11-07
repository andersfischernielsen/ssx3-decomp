//! Staging wave 211 (oc296): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::guest::Addr;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_0560,
        ssx_audio_system_rider_say_207d,
        "SSXAudioSystem_RiderSay207D",
    ),
    (0x003e_0c88, stream_file_queue_range, "StreamFile_QueueRange"),
    (
        0x0030_a298,
        wscript_man_run_obj_script1,
        "WScriptMan_RunObjScript1",
    ),
    (0x0041_6404, strchr, "strchr"),
    (
        0x0034_e448,
        anim_object_alloc_node_info,
        "AnimObject_AllocNodeInfo",
    ),
    (0x0013_1c30, rider_cruise_sub_exit, "RiderCruiseSub_Exit"),
    (0x003c_8468, snd_ee_voice_stop, "SND_EeVoiceStop"),
    (
        0x0010_bfa8,
        computer_actor_rider_plan_air_tricks,
        "ComputerActorRider_PlanAirTricks",
    ),
    (
        0x002a_77a8,
        crowd_audio_pitch_bend_callback,
        "CrowdAudio_PitchBendCallback",
    ),
    (0x003b_c570, snd_midi_to_volume, "SND_MidiToVolume"),
];

// ---------------------------------------------------------------------------
// 0x002a0560 SSXAudioSystem_RiderSay207D

/// Rider fields read by [`ssx_audio_system_rider_say_207d`].
mod rs207d {
    /// `float` along-track progress; speech only while it is negative.
    pub const PROGRESS: u32 = 0x470;
    /// `float` heading angle (radians) of the passed rider.
    pub const HEADING: u32 = 0x4cc;
    /// The rider's embedded scripting object (its vtable pointer).
    pub const OBJ: u32 = 0x6c0;
    /// "Local" flag (0: not the local player).
    pub const LOCAL: u32 = 0x87c;
    /// Object vtable slots (`{i16 delta; pad; fn}`): direction vector,
    /// position, "is AI".
    pub const VT_DIR: u32 = 0x10;
    pub const VT_POS: u32 = 0x28;
    pub const VT_IS_AI: u32 = 0x48;
    /// Audio system: the speech system and the last pass time.
    pub const SPEECH: u32 = 0x5560;
    pub const LAST_PASS: u32 = 0x582c;
    /// Race manager: the race time.
    pub const RACE_TIME: u32 = 0x8;
    /// gp: speech settings (`+0x550` speech mode), the post hook, the
    /// listener radius and the three distance limits.
    pub const SPEECH_SETTINGS_GP: u32 = 0x84c;
    pub const SPEECH_MODE: u32 = 0x550;
    pub const POST_HOOK_GP: u32 = 0x173c;
    pub const RADIUS_GP: u32 = 0x5e8;
    pub const MIN_GAP_GP: u32 = 0x456c;
    pub const MAX_GAP_GP: u32 = 0x4568;
    pub const FAR_GAP_GP: u32 = 0x4564;
}

const RS207D_IS_SPEECH_VOLUME_ON: Callee = Callee {
    addr: 0x0029_f0a0,
    f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
};
const RS207D_IS_INSTANCE_READY: Callee = Callee {
    addr: 0x002b_0e28,
    f: ee::s08::SpeechSystem_IsInstanceReady,
};
const RS207D_GET_RIDER_SPEECH_MASK: Callee = Callee {
    addr: 0x0029_f330,
    f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
};
const RS207D_GET_RIDER_VOICE_MASK: Callee = Callee {
    addr: 0x002a_1d48,
    f: ee::s08::SSXAudioSystem_GetRiderVoiceMask,
};
const RS207D_GET_RIDER_RIVALRY: Callee = Callee {
    addr: 0x002a_1820,
    f: ee::s08::SSXAudioSystem_GetRiderRivalry,
};
const RS207D_SET_EVENT_CONTEXT: Callee = Callee {
    addr: 0x002b_1458,
    f: ee::s08::SpeechSystem_SetEventContext,
};

/// `jalr` to the scripting-object method at `slot` of the rider whose
/// object base (`rider + 0x6c0`) is `obj`, vtable `vt`: `a0 = obj + delta`.
fn rs207d_obj_call(c: &mut Cpu, obj: u64, vt: u64, slot: u32, ret_to: u32) -> crate::call::Flow {
    c.g[A0][0] = Addr((vt as u32).wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr((vt as u32).wrapping_add(slot + 4)).lw(c);
    let f = r32(c, V0);
    c.g[A0][0] = sx((obj as u32).wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret_to)
}

/// `dot(sp[0..4], *v)` in vf4.x (VU0 macro ops, vf6.x = 1.0).
fn rs207d_dot(c: &mut Cpu, v: u32) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, sp(c));
    c.lqc2(5, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// Post one speech event: register its context (`SetEventContext(speech,
/// 0, code, rider, Rider_GetCharacter(rider), 0, f20, 0)`, the rider in
/// s1/s0 as `rider_reg`) and, when accepted, call the post hook
/// (`gp+0x173c`) with `SPCH_MakeId(1, 0, code)` and the arguments
/// `hook_args` sets (`before_id` runs before `SPCH_MakeId`). Returns
/// whether the context was accepted.
fn rs207d_post(
    c: &mut Cpu,
    rider_reg: usize,
    code: u64,
    rets: [u32; 4],
    accept_cycles: u64,
    before_id: impl FnOnce(&mut Cpu),
    hook_args: impl FnOnce(&mut Cpu),
) -> Result<bool, crate::call::Unwound> {
    c.g[A0][0] = c.g[rider_reg][0];
    call_ptr(c, 0x0012_3128, rets[0])?; // Rider_GetCharacter
    c.g[A3][0] = c.g[rider_reg][0];
    c.g[T0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = code;
    c.mov_s(12, 20);
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    RS207D_SET_EVENT_CONTEXT.call(c, rets[1])?;
    let accepted = c.g[V0][0] != 0;
    c.g[A0][0] = 1;
    if !accepted {
        return Ok(false);
    }
    c.cycles += accept_cycles;
    c.g[A1][0] = 0;
    c.g[A2][0] = code;
    before_id(c);
    call_ptr(c, 0x003d_8008, rets[2])?; // SPCH_MakeId(1, 0, code)
    c.g[V1][0] = Addr(r32(c, GP).wrapping_add(rs207d::POST_HOOK_GP)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    hook_args(c);
    let hook = r32(c, V1);
    call_ptr(c, hook, rets[3])?;
    Ok(true)
}

/// `void SSXAudioSystem_RiderSay207D(SSXAudioSystem* sys, Rider* a, Rider*
/// b)` (0x2a0560): the "passed you" speech when rider `a` overtakes `b`.
/// Only with speech on (mode 2), outside replays, both riders' character
/// info clear, at least 600 ticks since the last pass, both still behind
/// the start line (progress < 0), one of them the followed rider unless
/// both are human/local, the speech system ready, both heard by the same
/// listener view, and their gap along `b`'s heading between the near and
/// far limits. Then, by rivalry (2: rival lines 0x2081/0x207f; 0/1: pass
/// lines 0x2080/0x207d), posts a line for each rider with its speech mask
/// and the gap class (1, or 2 beyond the "far" limit).
pub fn ssx_audio_system_rider_say_207d(c: &mut Cpu) {
    use rs207d::*;
    let frame = Frame { size: 0xc0 };
    guest(c, |c| {
        c.cycles += 17;
        Frame::open(c, 0xc0);
        frame.save(c, S3, 0x70);
        frame.save(c, S0, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xb0);
        RS207D_IS_SPEECH_VOLUME_ON.call(c, 0x002a_059c)?;
        // false: the plain exit; true: the exit through 0x2a09fc (+1 cycle).
        let late_exit = rs207d_body(c)?;
        if late_exit {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0xa0);
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`ssx_audio_system_rider_say_207d`] after the volume check:
/// `Ok(true)` when it leaves through the 0x2a09fc exit block.
fn rs207d_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use rs207d::*;
    let gp = r32(c, GP);
    let volume_off = c.g[V0][0] == 0;
    c.g[V1][0] = Addr(gp.wrapping_sub(SPEECH_SETTINGS_GP)).lw(c);
    if volume_off {
        return Ok(true);
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(SPEECH_MODE)).lw(c) ^ 2;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8ae0, 0x002a_05bc)?; // SSXAudioSystem_IsReplayPlaying
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0012_3168, 0x002a_05cc)?; // Rider_GetPlayerCharacterInfo
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0012_3168, 0x002a_05dc)?;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 9;
    call_ptr(c, 0x0028_b1c8, 0x002a_05ec)?; // SSXAudioSystem_GetGameModule
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(LAST_PASS)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RACE_TIME)).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 600) as u64;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    // Both riders still behind the line (0 <= progress: no speech).
    for (rider_reg, extra) in [(S1, 6u64), (S0, 5)] {
        c.cycles += extra;
        c.lwc1(0, r32(c, rider_reg).wrapping_add(PROGRESS));
        if extra == 6 {
            c.mtc1(0, 1);
        }
        c.c_le_s(1, 0);
        c.g[V0][0] = 1;
        if !c.fcc() {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            return Ok(false);
        }
    }
    // a: "AI or not local" (s4); its object base in s6.
    c.cycles += 10;
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(OBJ)).lw(c);
    c.g[A1][0] = sx(r32(c, S1).wrapping_add(OBJ));
    c.g[S6][0] = c.g[A1][0];
    c.g[S4][0] = 0;
    let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_IS_AI, 0x002a_066c)?;
    let mut set = c.g[V0][0] != 0;
    if !set {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(LOCAL)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            set = true;
        }
    }
    if set {
        c.g[S4][0] = 1;
        c.cycles += 1;
    }
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(OBJ)).lw(c);
    // b: the same (s2); its object base in s5.
    c.cycles += 9;
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(OBJ));
    c.g[S5][0] = c.g[A1][0];
    c.g[S2][0] = 0;
    let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_IS_AI, 0x002a_06a4)?;
    let mut set = c.g[V0][0] != 0;
    if !set {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(LOCAL)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            set = true;
        }
    }
    if set {
        c.g[S2][0] = 1;
        c.cycles += 1;
    }
    c.g[A0][0] = c.g[S3][0];
    // The followed rider must be a or b unless both are AI / not local.
    c.cycles += 4;
    c.g[A1][0] = u64::MAX;
    call_ptr(c, 0x0028_5d98, 0x002a_06c8)?; // SSXAudioSystem_FindRider(sys, -1)
    let a_flag = c.g[S4][0] != 0;
    c.g[S4][0] = sx(r32(c, S3).wrapping_add(SPEECH));
    let mut tail = true;
    if a_flag {
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 2;
            if c.g[S1][0] == c.g[V0][0] {
                tail = false;
            } else {
                c.cycles += 2;
                if c.g[S0][0] != c.g[V0][0] {
                    return Ok(false);
                }
                c.cycles += 1;
                c.g[S4][0] = sx(r32(c, S3).wrapping_add(SPEECH));
            }
        }
    }
    if tail {
        c.cycles += 1;
    }
    c.g[A1][0] = 0;
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    RS207D_IS_INSTANCE_READY.call(c, 0x002a_06f8)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    // Both riders must be heard by the same listener view.
    c.cycles += 12;
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(OBJ)).lw(c);
    let (obj, vt) = (c.g[S6][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_POS, 0x002a_0714)?;
    c.lwc1(12, gp.wrapping_add(RADIUS_GP));
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8b40, 0x002a_0724)?; // SSXAudioSystem_FindViewInRange
    c.g[S2][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) < 0 {
        return Ok(false);
    }
    c.cycles += 11;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(OBJ)).lw(c);
    let (obj, vt) = (c.g[S5][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_POS, 0x002a_0744)?;
    c.lwc1(12, gp.wrapping_add(RADIUS_GP));
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8b40, 0x002a_0754)?;
    if c.g[S2][0] != c.g[V0][0] {
        return Ok(false);
    }
    // The gap along b's heading: dot((cos, sin, 0, 0), dirA - dirB).
    c.cycles += 44;
    c.lwc1(12, r32(c, S0).wrapping_add(HEADING));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x14));
    call_ptr(c, 0x0031_be50, 0x002a_076c)?; // Math_SinCos(heading, &sin, &cos)
    let s = sp(c);
    c.lwc1(0, s.wrapping_add(0x14));
    c.lwc1(1, s.wrapping_add(0x10));
    c.swc1(0, s);
    c.swc1(1, s.wrapping_add(0x4));
    c.sw(s.wrapping_add(0x8), 0);
    c.sw(s.wrapping_add(0xc), 0);
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(OBJ)).lw(c);
    let (obj, vt) = (c.g[S6][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_DIR, 0x002a_0798)?;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(OBJ)).lw(c);
    let v = r32(c, V0);
    rs207d_dot(c, v);
    c.qmfc2(A0, 4);
    c.mtc1(A0, 20);
    let (obj, vt) = (c.g[S5][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_DIR, 0x002a_07d0)?;
    let v = r32(c, V0);
    rs207d_dot(c, v);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.lwc1(1, gp.wrapping_sub(MIN_GAP_GP));
    c.sub_s(20, 20, 0);
    c.c_lt_s(1, 20);
    if !c.fcc() {
        return Ok(false);
    }
    c.cycles += 5;
    c.lwc1(0, gp.wrapping_sub(MAX_GAP_GP));
    c.c_lt_s(20, 0);
    let too_far = !c.fcc();
    c.g[A0][0] = c.g[S1][0];
    if too_far {
        return Ok(true);
    }
    // Speech masks (s6: a, s7: b), b's voice mask (s2), gap class (s5).
    c.cycles += 14;
    c.g[S5][0] = 1;
    RS207D_GET_RIDER_SPEECH_MASK.call(c, 0x002a_0828)?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[S6][0] = c.g[V0][0];
    RS207D_GET_RIDER_VOICE_MASK.call(c, 0x002a_0838)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    RS207D_GET_RIDER_SPEECH_MASK.call(c, 0x002a_0844)?;
    c.lwc1(0, gp.wrapping_sub(FAR_GAP_GP));
    c.c_le_s(0, 20);
    let near = !c.fcc();
    c.g[S7][0] = c.g[V0][0];
    if !near {
        c.cycles += 1;
        c.g[S5][0] = 2;
    }
    // Rivalry: 2 the rival lines, 0/1 the pass lines.
    c.cycles += 8;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S0][0];
    RS207D_GET_RIDER_RIVALRY.call(c, 0x002a_086c)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    let rival = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[V1][0] < 2) as u64;
    if rival {
        // a: "rival passed" (0x2081), b: "passed by rival" (0x207f).
        c.cycles += 14;
        c.mtc1(0, 20);
        let rets = [0x002a_0888, 0x002a_08ac, 0x002a_08c0, 0x002a_08d8];
        rs207d_post(c, S1, 0x2081, rets, 9, |_| {}, |c| {
            c.g[A2][0] = c.g[S6][0];
            c.g[A1][0] = 2;
            c.g[A3][0] = c.g[S5][0];
        })?;
        c.cycles += 13;
        let rets = [0x002a_08e0, 0x002a_0904, 0x002a_0918, 0x002a_0930];
        let posted = rs207d_post(c, S0, 0x207f, rets, 11, |_| {}, |c| {
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[A1][0] = 2;
        })?;
        return Ok(!posted);
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    // a: "passed" (0x2080, with b's voice mask | 0x1000), b: "got
    // passed" (0x207d).
    c.cycles += 14;
    c.mtc1(0, 20);
    let rets = [0x002a_094c, 0x002a_0970, 0x002a_0988, 0x002a_09a4];
    let before = |c: &mut Cpu| c.g[S2][0] |= 0x1000;
    rs207d_post(c, S1, 0x2080, rets, 11, before, |c| {
        c.g[A2][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = c.g[S5][0];
    })?;
    c.cycles += 13;
    let rets = [0x002a_09ac, 0x002a_09d0, 0x002a_09e4, 0x002a_09fc];
    rs207d_post(c, S0, 0x207d, rets, 9, |_| {}, |c| {
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = c.g[S5][0];
        c.g[A1][0] = 2;
    })?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// 0x003e0c88 StreamFile_QueueRange

/// Stream-file and request fields used by [`stream_file_queue_range`].
mod sfqr {
    /// Stream file: its mutex, read state (0 idle), the "urgent" flag and
    /// the two priorities `StartNextRequest` takes.
    pub const MUTEX: u32 = 0x4;
    pub const READ_STATE: u32 = 0x38;
    pub const PRIORITY: u32 = 0x3c;
    pub const URGENT_PRIORITY: u32 = 0x40;
    pub const URGENT: u32 = 0x48;
    /// Request: kind (0: a file range), name (255 chars), offset, size.
    pub const REQ_KIND: u32 = 0x10;
    pub const REQ_NAME: u32 = 0x14;
    pub const REQ_OFFSET: u32 = 0x118;
    pub const REQ_SIZE: u32 = 0x11c;
}

const SFQR_VALIDATE: Callee = Callee {
    addr: 0x003d_fbd0,
    f: ee::s14::StreamFile_Validate,
};
const SFQR_ALLOC_REQUEST: Callee = Callee {
    addr: 0x003d_fcd8,
    f: ee::s14::StreamFile_AllocRequest,
};
const SFQR_ENQUEUE_REQUEST: Callee = Callee {
    addr: 0x003d_fd58,
    f: ee::s14::StreamFile_EnqueueRequest,
};
const SFQR_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};
const SFQR_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};
const SFQR_START_NEXT_REQUEST: Callee = Callee {
    addr: 0x003e_0270,
    f: ee::s14::StreamFile_StartNextRequest,
};

/// `int StreamFile_QueueRange(StreamFile* file, const char* name, int
/// offset, int size)` (0x3e0c88): queue a read of `size` bytes at `offset`
/// of file `name` on a valid ("STRM") stream file. Takes a free request
/// (0 when none or the file is invalid), fills it, appends it to the queue
/// and, if the stream was idle, marks it busy and starts the next request
/// (at the urgent priority when the urgent flag is set). Returns the
/// request's id.
pub fn stream_file_queue_range(c: &mut Cpu) {
    use sfqr::*;
    let frame = Frame { size: 0x60 };
    guest(c, |c| {
        c.cycles += 14;
        Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0x10);
        // Validate(file, &stream (sp+0), &file (sp+4)).
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        SFQR_VALIDATE.call(c, 0x003e_0cb8)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            sfqr_queue(c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The stream (the validated header) saved at `sp+0`.
fn sfqr_stream(c: &mut Cpu) -> u64 {
    Addr(sp(c)).lw(c)
}

/// [`stream_file_queue_range`] after validation: the request, the queue
/// and the idle-stream start; leaves the return value in v0.
fn sfqr_queue(c: &mut Cpu) -> crate::call::Flow {
    use sfqr::*;
    c.cycles += 5;
    c.g[A0][0] = sfqr_stream(c);
    SFQR_ALLOC_REQUEST.call(c, 0x003e_0cc8)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(());
    }
    let req = r32(c, S0);
    Addr(req).off(REQ_KIND).sw(c, 0);
    c.cycles += 16;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 0xff;
    c.g[A0][0] = sx(req.wrapping_add(REQ_NAME));
    call_ptr(c, 0x0041_6ce0, 0x003e_0cec)?; // strncpy(req->name, name, 255)
    let req = r32(c, S0);
    let offset = c.g[S2][0];
    Addr(req).off(REQ_OFFSET).sw(c, offset);
    c.g[A1][0] = c.g[S0][0];
    let size = c.g[S3][0];
    Addr(req).off(REQ_SIZE).sw(c, size);
    c.g[A0][0] = sfqr_stream(c);
    SFQR_ENQUEUE_REQUEST.call(c, 0x003e_0d00)?;
    c.g[A0][0] = sx((sfqr_stream(c) as u32).wrapping_add(MUTEX));
    SFQR_MUTEX_LOCK.call(c, 0x003e_0d0c)?;
    // Was the stream idle? Then mark it busy.
    c.g[V1][0] = sfqr_stream(c);
    let stream = r32(c, V1);
    c.g[S1][0] = Addr(stream).off(READ_STATE).lw(c);
    if c.g[S1][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr(stream).off(READ_STATE).sw(c, 1);
    }
    c.g[A0][0] = sfqr_stream(c);
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MUTEX));
    SFQR_MUTEX_UNLOCK.call(c, 0x003e_0d30)?;
    if c.g[S1][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = sfqr_stream(c);
        let stream = r32(c, A0);
        c.g[V0][0] = Addr(stream).off(URGENT).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(stream).off(URGENT_PRIORITY).lw(c);
            SFQR_START_NEXT_REQUEST.call(c, 0x003e_0d50)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(stream).off(PRIORITY).lw(c);
            SFQR_START_NEXT_REQUEST.call(c, 0x003e_0d60)?;
        }
    }
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x0030a298 WScriptMan_RunObjScript1

/// Script manager fields: the world, the object whose script runs.
const WSR1_WORLD: u32 = 0x28c;
const WSR1_RUNNING: u32 = 0x290;

const WSR1_CALL_FUNC: Callee = Callee {
    addr: 0x0030_9c88,
    f: ee::s10::WScriptMan_CallFunc,
};

/// `void WScriptMan_RunObjScript1(WScriptMan* self, WObj* obj)` (0x30a298):
/// with `obj` as the running object, look up its script function 1 in the
/// world's script cache and call it when there is one (the handle on the
/// stack), then clear the running object.
pub fn wscript_man_run_obj_script1(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        c.cycles += 12;
        Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        let obj = c.g[V0][0];
        Addr(this).off(WSR1_RUNNING).sw(c, obj);
        c.g[A2][0] = 1;
        c.g[A0][0] = Addr(this).off(WSR1_WORLD).lw(c);
        call_ptr(c, 0x003a_6b78, 0x0030_a2bc)?; // World_GetObjectScriptFunc(world, obj, 1)
        c.g[V1][0] = !c.g[V0][0];
        let handle = c.g[V0][0];
        Addr(sp(c)).sw(c, handle);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            WSR1_CALL_FUNC.call(c, 0x0030_a2d4)?;
        }
        c.cycles += 5;
        Addr(r32(c, S0)).off(WSR1_RUNNING).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00416404 strchr

/// 0x0101..01 and 0x8080..80 as newlib builds them in a register.
const STRCHR_ONES: u64 = 0x0101_0101_0101_0101;
const STRCHR_HIGHS: u64 = 0x8080_8080_8080_8080;

/// `pxor`.
fn strchr_pxor(a: crate::mmi::Gpr, b: crate::mmi::Gpr) -> crate::mmi::Gpr {
    [a[0] ^ b[0], a[1] ^ b[1]]
}

/// `char* strchr(const char* s, int ch)` (0x416404, newlib): the first
/// `(char)ch` in `s` (the terminating NUL counts), or null.
///
/// From an 8-byte aligned `s` it skips whole blocks holding neither a NUL
/// nor `ch`: 16 bytes per iteration (MMI, per-byte lanes) when `s` is
/// 16-byte aligned, else 8 bytes (`(x - ONES) & !x & HIGHS` on `x` and on
/// `x ^ pattern`); the block with a hit, and unaligned strings, go byte by
/// byte. The loop back edges check events (no unwind).
pub fn strchr(c: &mut Cpu) {
    use crate::mmi::{lq, pand, pcpyld, pcpyud, pnot, psubb};
    const QUAD_HEAD: u32 = 0x0041_6468;
    const DOUBLE_HEAD: u32 = 0x0041_653c;
    const BYTE_HEAD: u32 = 0x0041_6578;
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & 7;
    c.g[A1][0] &= 0xff;
    // The wide paths end with v0 = the byte at a0 (the block with a hit).
    'bytes: {
        if c.g[V0][0] != 0 {
            // Unaligned: straight to the byte loop.
            c.cycles += 1;
            c.g[V0][0] = c.lbu(r32(c, A0));
            break 'bytes;
        }
        // The constants: a2 = ONES, t0 = HIGHS, a3 = ch in every byte.
        c.cycles += 22;
        let ch = c.g[A1][0];
        c.g[A2][0] = STRCHR_ONES;
        c.g[T0][0] = STRCHR_HIGHS;
        let pair = (ch << 8).wrapping_add(ch);
        c.g[V1][0] = c.g[A0][0] & 0xf;
        let quad = (pair << 16).wrapping_add(pair);
        c.g[V0][0] = quad;
        c.g[T2][0] = quad << 32;
        c.g[A3][0] = quad.wrapping_add(quad << 32);
        if c.g[V1][0] == 0 {
            // 16 bytes per iteration.
            loop {
                c.cycles += 12;
                c.g[T1] = lq(c, r32(c, A0));
                c.g[T2] = pcpyld(c.g[A2], c.g[A2]);
                c.g[V1] = pnot(c.g[T1]);
                c.g[V0] = psubb(c.g[T1], c.g[T2]);
                c.g[A2] = pcpyld(c.g[T0], c.g[T0]);
                c.g[V0] = pand(c.g[V0], c.g[V1]);
                c.g[T0] = pcpyld(c.g[A3], c.g[A3]);
                c.g[V0] = pand(c.g[V0], c.g[A2]);
                c.g[V1] = pcpyud(c.g[V0], c.g[A3]);
                c.g[V1][0] |= c.g[V0][0];
                if c.g[V1][0] != 0 {
                    // A NUL in this block.
                    c.g[V0][0] = c.lbu(r32(c, A0));
                    break 'bytes;
                }
                c.cycles += 23;
                c.g[V0] = strchr_pxor(c.g[T1], c.g[T0]);
                c.g[V1] = psubb(c.g[V0], c.g[T2]);
                c.g[V0] = pnot(c.g[V0]);
                c.g[T0][0] = STRCHR_HIGHS;
                c.g[V1] = pand(c.g[V1], c.g[V0]);
                c.g[V1] = pand(c.g[V1], c.g[A2]);
                c.g[A2][0] = STRCHR_ONES;
                c.g[V0] = pcpyud(c.g[V1], c.g[A1]);
                c.g[V1][0] |= c.g[V0][0];
                if c.g[V1][0] != 0 {
                    // `ch` in this block.
                    c.cycles += 2;
                    c.g[V0][0] = c.lbu(r32(c, A0));
                    break 'bytes;
                }
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                if event_due(c) {
                    deliver_event_here(c, QUAD_HEAD);
                }
            }
        }
        // The first 8 bytes.
        c.cycles += 7;
        c.g[T1][0] = c.ld(r32(c, A0));
        let x = c.g[T1][0];
        c.g[V1][0] = !x;
        c.g[V0][0] = x.wrapping_sub(c.g[A2][0]) & !x & c.g[T0][0];
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lbu(r32(c, A0));
            break 'bytes;
        }
        c.cycles += 7;
        let y = x ^ c.g[A3][0];
        c.g[V1][0] = y.wrapping_sub(c.g[A2][0]) & !y & c.g[T0][0];
        c.g[V0][0] = !y;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.lbu(r32(c, A0));
            break 'bytes;
        }
        // 8 bytes per iteration (t1 = ONES, a2 = the block).
        c.cycles += 2;
        c.g[T1][0] = c.g[A2][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        loop {
            c.cycles += 7;
            c.g[A2][0] = c.ld(r32(c, A0));
            let x = c.g[A2][0];
            c.g[V1][0] = !x;
            let nul = x.wrapping_sub(c.g[T1][0]) & !x & c.g[T0][0];
            c.g[V0][0] = x ^ c.g[A3][0];
            if nul != 0 {
                c.cycles += 1;
                c.g[V0][0] = c.lbu(r32(c, A0));
                break 'bytes;
            }
            c.cycles += 6;
            let y = c.g[V0][0];
            c.g[V1][0] = !y;
            c.g[V0][0] = y.wrapping_sub(c.g[T1][0]) & !y & c.g[T0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lbu(r32(c, A0));
                break 'bytes;
            }
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
            if event_due(c) {
                deliver_event_here(c, DOUBLE_HEAD);
            }
        }
    }
    // Byte by byte: v0 is the byte at a0.
    loop {
        c.cycles += 3;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(r32(c, A0));
            break;
        }
        if event_due(c) {
            deliver_event_here(c, BYTE_HEAD);
        }
        c.cycles += 2;
        if c.g[V0][0] == c.g[A1][0] {
            c.g[V1][0] = c.lbu(r32(c, A0));
            break;
        }
        c.cycles += 2;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        c.g[V0][0] = c.lbu(r32(c, A0));
    }
    c.cycles += 4;
    c.g[V1][0] ^= c.g[A1][0];
    c.g[V0][0] = if c.g[V1][0] == 0 { c.g[A0][0] } else { 0 };
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x0034e448 AnimObject_AllocNodeInfo

/// Fields used by [`anim_object_alloc_node_info`].
mod aoani {
    /// AnimObject: its resource, the mesh count, the mesh instances, the
    /// per-node references.
    pub const RES: u32 = 0x2c;
    pub const MESH_COUNT: u32 = 0x40;
    pub const MESHES: u32 = 0x48;
    pub const REFS: u32 = 0x4c;
    /// Resource: the node definitions.
    pub const DEF: u32 = 0x80;
    /// Node definitions: count and the 16-byte entries (`+8` the mesh
    /// source, or 0).
    pub const DEF_COUNT: u32 = 0x4;
    pub const DEF_ENTRIES: u32 = 0x8;
    pub const ENTRY_SIZE: u32 = 0x10;
    pub const ENTRY_SRC: u32 = 0x8;
    /// A mesh instance (`IntersectInfo`, 0xd0 bytes) and its vtable field.
    pub const MESH_SIZE: u32 = 0xd0;
    pub const MESH_VTABLE: u32 = 0x84;
    /// Allocation names and the `IntersectInfo` vtables (base, then final).
    pub const MESH_NAME: u64 = 0x0048_e860;
    pub const REF_NAME: u64 = 0x0048_e870;
    pub const BASE_VTABLE: u64 = 0x0049_0af0;
    pub const VTABLE: u64 = 0x0049_0ad0;
}

const AOANI_CLEAR: Callee = Callee {
    addr: 0x0035_13d0,
    f: ee::s11::IntersectInfo_Clear,
};
const AOANI_SET_SOURCE: Callee = Callee {
    addr: 0x0035_1398,
    f: ee::s11::IntersectInfo_SetSource,
};

/// `void AnimObject_AllocNodeInfo(AnimObject* self)` (0x34e448): count the
/// node definitions that have a mesh source, allocate that many mesh
/// instances (`new IntersectInfo[n]`: a count word, then each one cleared
/// and given its identity matrix) and one 32-byte reference per node, then
/// point each mesh instance at its node's source.
pub fn anim_object_alloc_node_info(c: &mut Cpu) {
    use aoani::*;
    let frame = Frame { size: 0xb0 };
    guest(c, |c| {
        c.cycles += 19;
        Frame::open(c, 0xb0);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S0, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, S3);
        c.g[V0][0] = Addr(this).off(RES).lw(c);
        c.g[S4][0] = Addr(r32(c, V0)).off(DEF).lw(c);
        let def = r32(c, S4);
        c.g[S0][0] = Addr(def).off(DEF_ENTRIES).lw(c);
        Addr(this).off(MESH_COUNT).sw(c, 0);
        c.g[V0][0] = Addr(def).off(DEF_COUNT).lw(c);
        c.g[S1][0] = 0;
        // Count the nodes with a mesh source.
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 1;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0)).off(ENTRY_SRC).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    let n = Addr(this).off(MESH_COUNT).lw(c);
                    c.g[V0][0] = sx((n as u32).wrapping_add(1));
                    let n = c.g[V0][0];
                    Addr(this).off(MESH_COUNT).sw(c, n);
                }
                c.g[V0][0] = Addr(def).off(DEF_COUNT).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(ENTRY_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_e498);
                }
            }
        }
        // The mesh instances (the refs pointer's address at sp+0).
        c.cycles += 3;
        c.g[S1][0] = Addr(this).off(MESH_COUNT).lw(c);
        c.g[A0][0] = MESH_SIZE as u64;
        if c.g[S1][0] != 0 {
            c.cycles += 15;
            c.g[V0][0] = sx(this.wrapping_add(REFS));
            c.mult(0, A0, S1, A0);
            c.g[A1][0] = MESH_NAME;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            let refs_at = c.g[V0][0];
            Addr(sp(c)).sw(c, refs_at);
            c.g[FP][0] = sx(this.wrapping_add(MESHES));
            c.g[S2][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            call_ptr(c, 0x0031_7e30, 0x0034_e4fc)?; // operator_new_named
            c.g[S5][0] = sx(r32(c, V0).wrapping_add(0x10));
            let count = c.g[S1][0];
            Addr(r32(c, V0)).sw(c, count);
            c.g[S0][0] = c.g[S5][0];
            if c.g[S1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = 0x0049_0000;
                c.g[V1][0] = 0x0049_0000;
                c.g[S7][0] = BASE_VTABLE;
                c.g[S6][0] = VTABLE;
                c.g[S1][0] = u64::MAX;
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S0][0];
                    let mesh = r32(c, S0);
                    let base_vt = c.g[S7][0];
                    Addr(mesh).off(MESH_VTABLE).sw(c, base_vt);
                    AOANI_CLEAR.call(c, 0x0034_e52c)?;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                    let mesh = r32(c, S0);
                    let vt = c.g[S6][0];
                    Addr(mesh).off(MESH_VTABLE).sw(c, vt);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0036_12c0, 0x0034_e53c)?; // IntersectInfo_ResetMatrix
                    let more = c.g[S2][0] != c.g[S1][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(MESH_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_e520);
                    }
                }
            }
            c.cycles += 2;
            let meshes = c.g[S5][0];
            Addr(r32(c, FP)).sw(c, meshes);
        } else {
            c.cycles += 3;
            Addr(this).off(MESHES).sw(c, 0);
            c.g[V1][0] = sx(this.wrapping_add(REFS));
            let refs_at = c.g[V1][0];
            Addr(sp(c)).sw(c, refs_at);
        }
        // The node references (32 bytes each).
        c.cycles += 9;
        c.g[S0][0] = Addr(r32(c, S4)).off(DEF_COUNT).lw(c);
        c.g[A1][0] = REF_NAME;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx(r32(c, S0) << 5);
        call_ptr(c, 0x0031_7e30, 0x0034_e574)?; // operator_new_named
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 6;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[S1][0] = 0;
        let refs = c.g[V0][0];
        Addr(r32(c, V1)).sw(c, refs);
        let def = r32(c, S4);
        c.g[V0][0] = Addr(def).off(DEF_COUNT).lw(c);
        c.g[S0][0] = Addr(def).off(DEF_ENTRIES).lw(c);
        // Point each mesh instance at its node's source.
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 3;
                c.g[A1][0] = Addr(r32(c, S0)).off(ENTRY_SRC).lw(c);
                if c.g[A1][0] != 0 {
                    c.cycles += 5;
                    let meshes = Addr(r32(c, S3)).off(MESHES).lw(c);
                    c.g[A0][0] = sx((meshes as u32).wrapping_add(r32(c, S2)));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(MESH_SIZE));
                    AOANI_SET_SOURCE.call(c, 0x0034_e5bc)?;
                }
                c.g[V0][0] = Addr(r32(c, S4)).off(DEF_COUNT).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(ENTRY_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_e5a0);
                }
            }
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
    })
}

// ---------------------------------------------------------------------------
// 0x00131c30 RiderCruiseSub_Exit

/// The rider's animation controller (`rider + 0x784`).
const RCSE_ANIM: u32 = 0x784;
/// Layer-1 anim types that the cruise state fades out on exit.
const RCSE_FADE_TYPES: [u64; 2] = [3, 0xd];

/// `void RiderCruiseSub_Exit(RiderSubState* self, int next)` (0x131c30):
/// unless the next sub-state is 4, when animation layer 1 plays a cruise
/// anim (type 3 or 13) fade layer 0 out over 0.1 s and set layer 1's
/// sequence weight (+0x90) to 1.
pub fn rider_cruise_sub_exit(c: &mut Cpu) {
    let frame = Frame { size: 0x20 };
    guest(c, |c| {
        c.cycles += 6;
        Frame::open(c, 0x20);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let to_4 = c.g[A1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        let mut late = true;
        if !to_4 {
            // Layer 1's anim type (asked again for the second type).
            let mut fade = false;
            for (i, (cycles, ret_to)) in [(7u64, 0x0013_1c58u32), (6, 0x0013_1c70)].into_iter().enumerate() {
                c.cycles += cycles;
                let rider = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = rider;
                if i == 0 {
                    c.g[A1][0] = 1;
                }
                c.g[A0][0] = Addr(rider as u32).off(RCSE_ANIM).lw(c);
                call_ptr(c, 0x0031_1ae8, ret_to)?; // AnimController_GetLayerAnimEntry
                c.g[V1][0] = RCSE_FADE_TYPES[i];
                let hit = c.g[V0][0] == c.g[V1][0];
                if i == 0 {
                    c.g[A1][0] = 1;
                }
                if hit {
                    fade = true;
                    break;
                }
            }
            if fade {
                c.cycles += 12;
                let rider = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = rider;
                c.g[A1][0] = 0;
                c.lwc1(12, r32(c, GP).wrapping_sub(0x75b0)); // 0.1f
                c.g[A0][0] = Addr(rider as u32).off(RCSE_ANIM).lw(c);
                call_ptr(c, 0x0031_1e88, 0x0013_1c90)?; // AnimController_FadeOutLayer
                let rider = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = rider;
                c.g[A1][0] = 1;
                c.g[A0][0] = Addr(rider as u32).off(RCSE_ANIM).lw(c);
                call_ptr(c, 0x0031_1b20, 0x0013_1ca0)?; // AnimController_GetLayerSequence
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.swc1(0, r32(c, V0).wrapping_add(0x90));
            } else {
                late = false;
            }
        }
        if late {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003c8468 SND_EeVoiceStop

/// The EE voice mixer (0x515b40): channel count (byte +5), the per-channel
/// and the master level sums, the voice table pointer.
const SEVS_MIXER: u32 = 0x0051_5b40;
const SEVS_CHANNELS: u32 = 0x5;
const SEVS_CHANNEL_SUMS: u32 = 0xe8;
const SEVS_MASTER_SUM: u32 = 0x100;
const SEVS_VOICES: u32 = 0x1dc;
/// A voice (0x60 bytes): active byte, channel levels, master level,
/// scale, node list.
const SEVS_VOICE_SIZE: u32 = 0x60;
const SEVS_LEVELS: u32 = 0x4;
const SEVS_MASTER_LEVEL: u32 = 0x34;
const SEVS_SCALE: u32 = 0x3c;
const SEVS_NODES: u32 = 0x40;

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
// 0x0010bfa8 ComputerActorRider_PlanAirTricks

/// Air-trick plan fields of a computer rider.
mod capat {
    pub const TRICK: u32 = 0xe0c;
    pub const PLANNED: u32 = 0xe10;
    pub const SKIP: u32 = 0xe14;
    pub const DONE: u32 = 0xe18;
    pub const ARG_T0: u32 = 0xe1c;
    pub const ARG_A3: u32 = 0xe20;
    /// Spin / flip directions (+-1.0 or 0).
    pub const SPIN: u32 = 0xe38;
    pub const FLIP: u32 = 0xe3c;
}

const CAPAT_ROLL_SKIP_TRICKS: Callee = Callee {
    addr: 0x0010_c1d0,
    f: ee::s00::ComputerActorRider_RollSkipTricks,
};

/// Store `1.0` (spin / flip wanted) or `0` at `off` of the rider in s0.
fn capat_axis(c: &mut Cpu, want: bool, off: u32) {
    c.cycles += 2;
    let at = r32(c, S0).wrapping_add(off);
    if !want {
        c.sw(at, 0);
    } else {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.swc1(0, at);
    }
}

/// `void ComputerActorRider_PlanAirTricks(ComputerActorRider* self, int
/// spin, int flip, int a3, int t0)` (0x10bfa8): reset the planned trick,
/// plan one if a spin or a flip is wanted, roll whether to skip tricks,
/// store the two parameters, and give the wanted axes a random direction
/// (+-1.0).
pub fn computer_actor_rider_plan_air_tricks(c: &mut Cpu) {
    use capat::*;
    let frame = Frame { size: 0x60 };
    guest(c, |c| {
        c.cycles += 17;
        Frame::open(c, 0x60);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        let this = r32(c, S0);
        c.g[V1][0] = sx(this.wrapping_add(PLANNED));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        c.sw(this.wrapping_add(TRICK), u64::MAX);
        c.g[S3][0] = c.g[T0][0];
        c.g[V0][0] = 0;
        // planned = spin || flip
        let mut planned = true;
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            planned = c.g[S4][0] != 0;
        }
        if planned {
            c.cycles += 2;
            c.g[V0][0] = 1;
        }
        let v = c.g[V0][0];
        c.sw(r32(c, V1), v);
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        CAPAT_ROLL_SKIP_TRICKS.call(c, 0x0010_c004)?;
        let this = r32(c, S0);
        c.g[V1][0] = c.lw(this.wrapping_add(PLANNED));
        let skip = c.g[V0][0];
        c.sw(this.wrapping_add(SKIP), skip);
        let a3 = c.g[S2][0];
        c.sw(this.wrapping_add(ARG_A3), a3);
        let t0 = c.g[S3][0];
        c.sw(this.wrapping_add(ARG_T0), t0);
        c.sw(this.wrapping_add(DONE), 0);
        let mut late = true;
        if c.g[V1][0] != 0 {
            let spin = c.g[S1][0] != 0;
            capat_axis(c, spin, SPIN);
            let flip = c.g[S4][0] != 0;
            capat_axis(c, flip, FLIP);
            // Random signs (AIrand() & 1 negates).
            for (i, (off, ret_to)) in [(SPIN, 0x0010_c04cu32), (FLIP, 0x0010_c06c)].into_iter().enumerate() {
                c.cycles += 5;
                call_ptr(c, 0x0031_7810, ret_to)?; // Rand_NextGame
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    let at = r32(c, S0).wrapping_add(off);
                    c.lwc1(0, at);
                    c.neg_s(0, 0);
                    c.swc1(0, at);
                } else if i == 1 {
                    late = false;
                }
            }
        }
        if late {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
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
// 0x002a77a8 CrowdAudio_PitchBendCallback

const CAPB_UPDATE_PITCH_BEND: Callee = Callee {
    addr: 0x002a_68b0,
    f: ee::s08::CrowdAudio_UpdatePitchBend,
};

/// `void CrowdAudio_PitchBendCallback(void* ctx, BankMonitorInstance*
/// voice, float dt)` (0x2a77a8): the bank monitor's per-voice callback:
/// `CrowdAudio_UpdatePitchBend(voice->owner (+0x80), voice, dt)`.
pub fn crowd_audio_pitch_bend_callback(c: &mut Cpu) {
    let frame = Frame { size: 0x10 };
    guest(c, |c| {
        c.cycles += 7;
        Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A1)).off(0x80).lw(c);
        CAPB_UPDATE_PITCH_BEND.call(c, 0x002a_77b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003bc570 SND_MidiToVolume

/// The MIDI-to-volume table (0x44c590, 128 halfwords).
const SMTV_TABLE: u32 = 0x0044_c590;

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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

    /// The world of [`super::ssx_audio_system_rider_say_207d`]: the audio
    /// system, riders a / b / c (c only as the followed rider), their
    /// scripting-object vtables (real leaves: `RiderScriptObj_GetSelf`
    /// returns the object + delta, 0x1406c0 returns 0), the speech
    /// settings, the game module with one listener view, the race manager,
    /// the speech system and its event contexts, and the BE singletons.
    fn rs207d_world(c: &mut ssx3_ee_rt::Cpu, seed: u32) -> Rng {
        const SYS: u32 = DATA + 0x20_0000;
        const RIDER_A: u32 = DATA + 0x21_0000;
        const RIDER_B: u32 = DATA + 0x21_1000;
        const RIDER_C: u32 = DATA + 0x21_2000;
        const VTABLES: u32 = DATA + 0x21_4000;
        const SETTINGS: u32 = DATA + 0x21_5000;
        const GAME: u32 = DATA + 0x21_6000;
        const MODULE: u32 = DATA + 0x21_7000;
        const VIEWS: u32 = DATA + 0x21_8000;
        const VIEW: u32 = DATA + 0x21_8100;
        const RACE_MAN: u32 = DATA + 0x21_9000;
        const STREAMS: u32 = DATA + 0x21_a000;
        const STREAM: u32 = DATA + 0x21_a100;
        const REPLAY: u32 = DATA + 0x21_a200;
        const BE_DUMMY: u32 = DATA + 0x21_b000;
        const IFACE: u32 = DATA + 0x21_b100;
        const IFACE_VT: u32 = DATA + 0x21_b200;
        const GET_SELF: u32 = 0x0014_0ce0;
        const RETURN_0: u32 = 0x0014_06c0;
        const F_ONE: u32 = 0x3f80_0000;

        let mut r = machine(c, seed, 0x2a_0560, 12);
        elf_image(c);
        c.pc = 0x002a_0560;
        let w = |c: &mut ssx3_ee_rt::Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let gp = |off: i32| (GAME_GP as i32 + off) as u32;
        // BE library: instance, interface 0 with a fake method, singletons.
        w(c, gp(-0x1f04), BE_DUMMY);
        w(c, gp(-0x1efc), 0);
        w(c, 0x0053_08a8, IFACE);
        w(c, IFACE + 4, IFACE_VT);
        w(c, IFACE_VT + 8, 0);
        w(c, IFACE_VT + 0xc, FAKE + 0x40);
        for (k, off) in [0x1f3ci32, 0x1f30, 0x1f2c, 0x1ea4].into_iter().enumerate() {
            w(c, gp(-off), BE_DUMMY + 0x400 + 0x40 * k as u32);
        }
        // Player slots 0/1/2 -> race records; a character-info byte set for
        // a (seed % 17 == 3) or b (== 4); random rivalry profiles.
        for p in 0..3u32 {
            w(c, 0x0053_05b0 + 4 * p, p);
            let rec = 0x0053_5b20 + 0x1c * p;
            w(c, rec + 0xc, if r.next_u32() % 3 == 0 { u32::MAX } else { r.next_u32() % 3 });
            w(c, rec + 0x10, r.next_u32());
            c.bus.write8(rec + 0x11, (r.next_u32() % 4) as u8);
        }
        c.bus.write8(0x0053_4fe0 + 0x12, (seed % 17 == 3) as u8);
        c.bus.write8(0x0053_4fe0 + 0x1c + 0x12, (seed % 17 == 4) as u8);
        // Speech volume (channel 4) on unless seed % 9 == 3; speech mode 2.
        w(c, SYS + 0x62bc + 0x40, if seed % 9 == 3 { 0 } else { F_ONE });
        w(c, gp(-0x84c), SETTINGS);
        w(c, SETTINGS + 0x550, if seed % 10 == 7 { 0 } else { 2 });
        // Game module, replay (playing for seed % 13 == 5), listener view.
        w(c, gp(-0x848), GAME);
        w(c, GAME + 0x84, MODULE);
        w(c, MODULE + 0x28, if seed % 13 == 5 { REPLAY } else { 0 });
        w(c, REPLAY, 3);
        w(c, MODULE + 0xc, RACE_MAN);
        w(c, MODULE + 0x84, VIEWS);
        w(c, VIEWS + 0x10, 1);
        w(c, VIEWS + 4, VIEW);
        put_vec(c, VIEW + 0x20, [0; 4]);
        // Race manager: time vs. the last pass, the followed-rider list,
        // rider count 2 (rival lines) or 3 (BE rivalry).
        w(c, RACE_MAN + 8, 1000);
        w(c, SYS + 0x582c, if seed % 19 == 5 { 900 } else { 0 });
        w(c, RACE_MAN + 0x78, if seed % 3 == 0 { 3 } else { 2 });
        w(c, RACE_MAN + 0x7c, 2);
        let followed = [RIDER_A, RIDER_B, RIDER_C, RIDER_A][(seed / 3 % 4) as usize];
        w(c, RACE_MAN + 0x28, followed);
        w(c, RACE_MAN + 0x2c, RIDER_C);
        // Speech system: an instance stream unless seed % 12 == 6, event
        // contexts free, or a's / b's line already live (rejected).
        let speech = SYS + 0x5560;
        w(c, speech + 4, STREAMS);
        w(c, STREAMS, if seed % 12 == 6 { 0 } else { STREAM });
        w(c, STREAM + 4, 0);
        w(c, gp(0x6c4), 0);
        let live: &[u32] = match seed % 5 {
            1 => &[0x2081, 0x2080],
            2 => &[0x207f, 0x207d],
            _ => &[],
        };
        for (i, &code) in live.iter().enumerate() {
            let slot = speech + 0x18 + 0x20 * i as u32;
            w(c, slot, 1);
            w(c, slot + 4, 50);
            w(c, slot + 8, 0);
            w(c, slot + 0xc, code);
        }
        w(c, gp(0x173c), FAKE + 0x80);
        // The riders.
        let gap = [500.0f32, 1000.0, 1500.0, 2000.0][(seed / 2 % 4) as usize];
        for (i, rider) in [RIDER_A, RIDER_B, RIDER_C].into_iter().enumerate() {
            let vt = VTABLES + 0x100 * i as u32;
            let bits = seed >> (2 * i);
            let progress = match (i, seed % 11) {
                (0, 1) => 1.0f32,
                (1, 2) => 0.0,
                _ => -1.0,
            };
            w(c, rider + 0x470, progress.to_bits());
            let heading = if seed % 7 == 0 { r.f32(3.0) } else { 0 };
            w(c, rider + 0x4cc, heading);
            w(c, rider + 0x6c0, vt);
            w(c, rider + 0x86c, i as u32);
            w(c, rider + 0x874, bits & 1);
            w(c, rider + 0x87c, (bits >> 1) & 1);
            w(c, rider + 0xec, r.next_u32() % 4);
            // Slots: direction (+0x7d0), position (+0x7c0), rival player
            // index (0), "is AI" (0 or non-zero).
            c.bus.write16(vt + 0x10, 0x110);
            w(c, vt + 0x14, GET_SELF);
            c.bus.write16(vt + 0x28, 0x100);
            w(c, vt + 0x2c, GET_SELF);
            c.bus.write16(vt + 0x38, 0);
            w(c, vt + 0x3c, RETURN_0);
            c.bus.write16(vt + 0x48, 0);
            w(c, vt + 0x4c, if (bits >> 4) & 1 == 0 { RETURN_0 } else { GET_SELF });
            // Heard by the view unless seed % 23 puts one far away.
            let far = (i == 0 && seed % 23 == 1) || (i == 1 && seed % 23 == 2);
            let x = if far { 5000.0f32 } else { 1.0 + i as f32 };
            put_vec(c, rider + 0x7c0, [x.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0]);
            let d = if i == 0 { gap } else { 0.0 };
            put_vec(c, rider + 0x7d0, [d.to_bits(), d.to_bits(), 0, 0]);
        }
        c.g[A0][0] = SYS as u64;
        c.g[A1][0] = RIDER_A as u64;
        c.g[A2][0] = RIDER_B as u64;
        r
    }

    /// `StreamFile_QueueRange`: an invalid file (null, or no "STRM"
    /// header), no free request, then a queued request on an empty or a
    /// non-empty queue, with the stream busy or idle (then
    /// `StartNextRequest` at either priority: the request's name matches
    /// the open file and `IssueRead` stalls at once). The mutex syscalls go
    /// to the fakes (some unwind).
    #[test]
    fn stream_file_queue_range_matches() {
        const FILE: u32 = DATA;
        const HDR: u32 = DATA + 0x1000;
        const REQ: u32 = DATA + 0x2000;
        const NEXT_FREE: u32 = DATA + 0x2400;
        const TAIL: u32 = DATA + 0x2800;
        const NAME: u32 = DATA + 0x3000;
        const RING: u32 = DATA + 0x4000;
        check(
            ee::s14::f_003e0c88__generated,
            stream_file_queue_range,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_0c88, 8);
                c.pc = 0x003e_0c88;
                let w = |c: &mut ssx3_ee_rt::Cpu, a: u32, v: u32| c.bus.write32(a, v);
                c.g[A0][0] = if seed % 11 == 0 { 0 } else { FILE as u64 };
                w(c, FILE, HDR);
                w(c, HDR, if seed % 13 == 1 { r.next_u32() } else { 0x4d52_5453 });
                // Mutex: owner (never the fake thread id), depth, semaphore.
                w(c, HDR + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
                w(c, HDR + 0xc, r.next_u32() & 3);
                w(c, HDR + 0x10, r.next_u32() & 0xff);
                // Free list: one request (or none), id counter.
                w(c, HDR + 0x68, if seed % 9 == 4 { 0 } else { REQ });
                w(c, REQ + 0xc, NEXT_FREE);
                w(c, REQ, r.next_u32());
                w(c, 0x0051_9c18, if seed % 7 == 2 { 0xffff_ff00 } else { r.next_u32() });
                // Queue: empty or one request.
                let queued = seed % 3 == 0;
                w(c, HDR + 0x64, if queued { TAIL } else { 0 });
                w(c, HDR + 0x5c, if queued { TAIL } else { 0 });
                w(c, HDR + 0x60, if queued { TAIL } else { 0 });
                w(c, TAIL + 4, 2);
                w(c, TAIL + 0xc, 0);
                // Busy or idle; the two priorities.
                w(c, HDR + 0x38, if seed % 2 == 0 { 0 } else { 1 + r.next_u32() % 2 });
                w(c, HDR + 0x48, (seed / 2) % 2);
                w(c, HDR + 0x3c, r.next_u32() % 8);
                w(c, HDR + 0x40, r.next_u32() % 8);
                // The file name, also the open file's name.
                for (i, b) in b"track01.big\0".iter().enumerate() {
                    c.bus.write8(NAME + i as u32, *b);
                    c.bus.write8(HDR + 0x6c + i as u32, *b);
                }
                c.g[A1][0] = NAME as u64;
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32() & 0xffff);
                // StreamFile_IssueRead stall path.
                w(c, HDR + 0x170, r.next_u32());
                w(c, HDR + 0x16c, 0);
                w(c, HDR + 0x30, RING);
                w(c, HDR + 0x34, DATA + 0x5000);
                w(c, HDR + 0x50, RING);
                w(c, HDR + 0x54, RING);
                w(c, HDR + 0x58, r.next_u32());
                w(c, HDR + 0x17c, 0x1000);
            },
        );
    }

    /// `WScriptMan_RunObjScript1`: an object without a script slot, a slot
    /// whose id is -1, and a found script (handle `id << 8 | section`) run
    /// by the real `WScriptMan_CallFunc` on the Luno VM (the fixture of
    /// `wscript_man_call_func_matches`).
    #[test]
    fn wscript_man_run_obj_script1_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        use crate::game::wscript::wscriptman::WSM_SECTION_TABLES;
        const SELF: u32 = DATA + 0x20_0000;
        const OBJ: u32 = SELF + 0x1000;
        const WORLD: u32 = SELF + 0x1100;
        const CACHE: u32 = SELF + 0x1200;
        const ENTRIES: u32 = SELF + 0x1300;
        const FUNCS: u32 = SELF + 0x1400;
        const SLOT: u32 = SELF + 0x1800;
        const OBJ_FUNCS: u32 = SELF + 0x1900;
        const SCRIPT: u32 = SELF + 0x2000;
        const TABLES: u32 = SELF + 0x3000;
        check(
            ee::s10::f_0030a298__generated,
            wscript_man_run_obj_script1,
            120,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x30_a298);
                c.pc = 0x0030_a298;
                let section = seed % 4;
                let id = (seed / 4) % 4;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OBJ);
                c.bus.write32(SELF + WSR1_WORLD, WORLD);
                c.bus.write32(SELF + WSR1_RUNNING, r.next_u32());
                c.bus.write32(WORLD + 4, CACHE);
                c.bus.write32(CACHE + 4, ENTRIES);
                for k in 0..4u32 {
                    let entry = ENTRIES + 0xc * k;
                    c.bus.write16(entry, 3);
                    c.bus.write32(entry + 8, FUNCS);
                }
                // The object's script slot: none, id -1, or function 1 of
                // object-script entry `k` = the handle.
                c.bus.write8(OBJ + 0x78, (r.next_u32() % 4) as u8);
                c.bus.write32(OBJ + 0x88, if seed % 5 == 0 { 0 } else { SLOT });
                let k = r.next_u32() % 3;
                c.bus.write32(SLOT + 8, if seed % 5 == 1 { u32::MAX } else { k << 8 | 0x5a });
                c.bus.write32(FUNCS + 0x1c, OBJ_FUNCS);
                c.bus.write32(OBJ_FUNCS + 0x18 * k + 4, id << 8 | section);
                // WScriptMan_CallFunc: the section function and its script.
                c.bus.write32(FUNCS + 0x3c, FUNCS + 0x40);
                c.bus.write32(FUNCS + 0x40 + 4 * id, SCRIPT);
                c.bus.write32(SCRIPT + 4, 0x10);
                c.bus.write32(SCRIPT + 0x14, 0);
                c.bus.write32(SCRIPT + 0x18, (seed / 16) % 4);
                c.bus.write32(SCRIPT + 0x1c, 0);
                for i in 0..4u32 {
                    let value = TABLES + 0x40 * i;
                    let table = TABLES + 0x400 + 0x40 * i;
                    c.bus.write32(SELF + WSM_SECTION_TABLES + 4 * i, value);
                    c.bus.write32(value, table);
                    c.bus.write32(table + 8, 100);
                }
                let d = 0x004c_9098;
                c.bus.write32(d, r.next_u32());
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, 2);
            },
        );
    }

    crate::difftest::events::driven!(strchr_gen, ee::s15::f_00416404__generated);
    crate::difftest::events::driven!(strchr_hand, strchr);

    /// `strchr`: strings of 0..~6000 bytes at every alignment (byte, 8-byte
    /// and 16-byte paths), searching for a byte of the string (early, late,
    /// in the first block), a byte not in it, or the NUL itself (`ch` with
    /// junk above the low byte); events delivered (or unwinding) inside the
    /// loops.
    #[test]
    fn strchr_matches() {
        use crate::difftest::events::{check_events, schedule};
        use crate::difftest::{fill, scramble};
        check_events(strchr_gen, strchr_hand, 600, &|c, seed| {
            let mut r = Rng(0x7374_7263_6872 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let s = DATA + 0x10 * (r.next_u32() % 32)
                + match r.next_u32() % 3 {
                    0 => 0,
                    1 => 8,
                    _ => r.next_u32() % 16,
                };
            let len = match r.next_u32() % 6 {
                0 => r.next_u32() % 8,
                1 => r.next_u32() % 40,
                2 => [7, 8, 9, 15, 16, 17, 31, 32, 33][(r.next_u32() % 9) as usize],
                3 => r.next_u32() % 300,
                4 => r.next_u32() % 2000,
                _ => 3000 + r.next_u32() % 3000,
            };
            // Bytes 1..=0xfe; 0xff never appears (the "absent" search).
            for i in 0..len {
                let b = match r.next_u32() % 6 {
                    0 => [1u8, 0x7f, 0x80, 0xfe, 2][(r.next_u32() % 5) as usize],
                    _ => (r.next_u32() as u8).clamp(1, 0xfe),
                };
                c.bus.write8(s + i, b);
            }
            c.bus.write8(s + len, 0);
            let ch = match r.next_u32() % 5 {
                0 => 0xff,
                1 => 0,
                _ if len == 0 => 0x41,
                2 => c.bus.read8(s + r.next_u32() % len.min(20)) as u32,
                _ => c.bus.read8(s + r.next_u32() % len) as u32,
            };
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | (r.next_u32() & 0xffff_ff00 | ch) as u64;
            schedule(c, &mut r, seed, 0x0041_6404)
        });
    }

    /// `AnimObject_AllocNodeInfo`: 0..6 node definitions, each with or
    /// without a mesh source (no meshes at all for some seeds); the real
    /// `operator_new_named` allocates from the test pools / heap, the real
    /// `IntersectInfo_Clear` / `ResetMatrix` / `SetSource` fill them.
    #[test]
    fn anim_object_alloc_node_info_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x20_0000;
        const RES: u32 = SELF + 0x100;
        const DEF: u32 = SELF + 0x200;
        const ENTRIES: u32 = SELF + 0x300;
        const SOURCES: u32 = SELF + 0x400;
        check(
            ee::s11::f_0034e448__generated,
            anim_object_alloc_node_info,
            200,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x34_e448);
                aoani_identity_matrix(c);
                c.pc = 0x0034_e448;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + 0x2c, RES);
                c.bus.write32(SELF + 0x40, r.next_u32());
                c.bus.write32(RES + 0x80, DEF);
                let count = seed % 7;
                c.bus.write32(DEF + 4, count);
                c.bus.write32(DEF + 8, ENTRIES);
                for i in 0..count {
                    let src = if seed % 5 == 0 || r.next_u32() % 3 == 0 {
                        0
                    } else {
                        SOURCES + 0x20 * i
                    };
                    c.bus.write32(ENTRIES + 0x10 * i + 8, src);
                    for k in 0..6 {
                        c.bus.write32(SOURCES + 0x20 * i + 4 * k, r.f32(100.0));
                    }
                }
            },
        );
    }

    /// The identity matrix `IntersectInfo_ResetMatrix` copies (0x4ff1a0).
    fn aoani_identity_matrix(c: &mut ssx3_ee_rt::Cpu) {
        for i in 0..4u32 {
            put_vec(c, 0x004f_f1a0 + 0x10 * i, {
                let mut v = [0u32; 4];
                v[i as usize] = 0x3f80_0000;
                v
            });
        }
    }

    /// `RiderCruiseSub_Exit`: next state 4 (nothing), layer 1 playing an
    /// anim of type 3, 13, another type or none (id 0x1b6); the fade runs
    /// the real `FadeOutLayer` (layer 0's sequence list empty) and
    /// `GetLayerSequence`.
    #[test]
    fn rider_cruise_sub_exit_matches() {
        const SUB: u32 = DATA + 0x20_0000;
        const RIDER: u32 = DATA + 0x20_1000;
        const CTRL: u32 = DATA + 0x20_2000;
        const LISTS: u32 = DATA + 0x20_3000;
        const SEQ: u32 = DATA + 0x20_4000;
        check(
            ee::s00::f_00131c30__generated,
            rider_cruise_sub_exit,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13_1c30, 0);
                c.pc = 0x0013_1c30;
                c.g[A0][0] = sx(SUB);
                c.g[A1][0] = if seed % 6 == 0 { 4 } else { (r.next_u32() % 8) as u64 };
                c.bus.write32(SUB, RIDER);
                c.bus.write32(RIDER + RCSE_ANIM, CTRL);
                c.bus.write32(GAME_GP - 0x75b0, 0x3dcc_cccd);
                // Layer anim ids 0..3 (or none) and their types.
                let id = if seed % 7 == 1 { 0x1b6 } else { seed / 2 % 4 };
                c.bus.write32(CTRL, r.next_u32() % 4);
                c.bus.write32(CTRL + 4, id);
                for (k, ty) in [3u32, 0xd, 5, 0].into_iter().enumerate() {
                    c.bus.write32(0x0044_6990 + 0x1c * k as u32, ty);
                }
                // Sequence lists: layer 0 empty; layer 1's head sequence.
                c.bus.write32(CTRL + 0x50, LISTS);
                c.bus.write32(LISTS + 4, 0);
                c.bus.write32(LISTS + 8 + 4, SEQ);
                c.bus.write32(SEQ + 0xc8, 0);
                c.bus.write32(SEQ + 0x90, r.next_u32());
            },
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
                c.bus.write8(SEVS_MIXER + SEVS_CHANNELS, (seed / 8 % 6) as u8);
                for i in 0..8 {
                    c.bus.write32(SEVS_MIXER + SEVS_CHANNEL_SUMS + 4 * i, r.f32(4.0));
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
                    let stop = if r.next_u32() % 3 == 0 { 0 } else { FAKE + 0x10 * k };
                    c.bus.write32(node + 4, stop);
                    c.bus.write32(node + 8, if k + 1 == n { 0 } else { node + 0x20 });
                }
                c.bus.write32(v + SEVS_NODES, NODES);
                // SND heap: no blocks (SND_MemFree ignores the pointer).
                c.bus.write32(0x0050_a8e8 + 0x234, HEAP);
                c.bus.write32(HEAP, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0x14, 0);
            },
        );
    }

    /// `ComputerActorRider_PlanAirTricks`: spin / flip wanted (1..3) or not
    /// (0), the real `RollSkipTricks` and `Rand_NextGame` on a random
    /// generator state (both signs of each axis).
    #[test]
    fn computer_actor_rider_plan_air_tricks_matches() {
        const RIDER: u32 = DATA + 0x20_0000;
        check(
            ee::s00::f_0010bfa8__generated,
            computer_actor_rider_plan_air_tricks,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_bfa8, 0);
                c.pc = 0x0010_bfa8;
                c.g[A0][0] = sx(RIDER);
                let pick = |r: &mut Rng, on: bool| -> u64 {
                    if on {
                        1 + (r.next_u32() % 3) as u64
                    } else {
                        0
                    }
                };
                c.g[A1][0] = pick(&mut r, seed % 2 == 1);
                c.g[A2][0] = pick(&mut r, seed / 2 % 2 == 1);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                for i in 0..0x100 {
                    c.bus.write32(RIDER + 0xd00 + 4 * i, r.next_u32());
                }
                c.bus.write32(RIDER + 0xdfc, r.f32(100.0));
                // Rand_NextGame's generator state.
                for i in 0..8 {
                    c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `CrowdAudio_PitchBendCallback`: the real `CrowdAudio_UpdatePitchBend`
    /// on the voice's owner (the fixture of its own test): not playing,
    /// idle, rising and falling bends; the SND lock hooks are fakes (some
    /// unwind).
    #[test]
    fn crowd_audio_pitch_bend_callback_matches() {
        use crate::difftest::fill;
        const SELF: u32 = DATA + 0x100;
        const VOICE: u32 = DATA + 0x400;
        const TABLE: u32 = DATA + 0x800;
        check(
            ee::s08::f_002a77a8__generated,
            crowd_audio_pitch_bend_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_77a8, 4);
                c.pc = 0x002a_77a8;
                fill(c, &mut r, SELF, 0x70);
                fill(c, &mut r, VOICE, 0x40);
                fill(c, &mut r, TABLE, 0x60);
                c.bus.write32(VOICE + 0x80, SELF);
                c.bus.write32(GAME_GP + 0x608, TABLE);
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write16(0x0050_a8e8 + 0x17e, 0);
                let playing = seed % 6 != 0;
                c.bus.write32(
                    VOICE + 4,
                    if playing {
                        r.next_u32() & 0x7fff_ffff
                    } else {
                        0x8000_0000 | r.next_u32()
                    },
                );
                c.bus.write32(VOICE + 0x54, 1);
                let curve = r.next_u32() % 4;
                c.bus.write32(SELF + 0x184, curve);
                let bend = TABLE + curve * 0x54 + 0x1c;
                let fall = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
                let rise = 0.25 + (r.next_u32() % 100) as f32 / 50.0;
                c.bus.write32(bend + 0x48, fall.to_bits());
                c.bus.write32(bend + 0x4c, rise.to_bits());
                c.bus.write32(bend + 0x50, r.f32(64.0));
                let time = match seed % 5 {
                    0 => -((r.next_u32() % 3) as f32),
                    _ => (r.next_u32() % 400) as f32 / 50.0,
                };
                c.bus.write32(SELF + 0x188, time.to_bits());
                c.f[12] = ((r.next_u32() % 100) as f32 / 60.0).to_bits();
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(VOICE);
            },
        );
    }

    /// `SND_MidiToVolume`: levels 0..127 and a few out of range (the
    /// table read is unchecked), with junk in the upper bits.
    #[test]
    fn snd_midi_to_volume_matches() {
        check(ee::s13::f_003bc570__generated, snd_midi_to_volume, 300, &|c, seed| {
            let mut r = Rng(0x3b_c570 ^ (seed as u64) << 16);
            crate::difftest::scramble(c, &mut r);
            c.pc = 0x003b_c570;
            for i in 0..0x200u32 {
                c.bus.write16(SMTV_TABLE + 2 * i, r.next_u32() as u16);
            }
            let level = if seed % 10 == 0 { 128 + r.next_u32() % 128 } else { seed % 128 };
            c.g[A0][0] = (r.next_u32() as u64) << 32 | level as u64;
        });
    }

    /// `SSXAudioSystem_RiderSay207D`: every early out (speech volume, mode,
    /// replay, character info, pass interval, progress, followed rider,
    /// speech instance, listener views, gap limits), the rival and pass
    /// lines with accepted and rejected contexts; the post hook and the BE
    /// interface method are fakes (some unwind).
    #[test]
    fn ssx_audio_system_rider_say_207d_matches() {
        check(
            ee::s08::f_002a0560__generated,
            ssx_audio_system_rider_say_207d,
            300,
            &|c, seed| {
                rs207d_world(c, seed);
            },
        );
    }
}
