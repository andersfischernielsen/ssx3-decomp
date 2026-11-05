//! Staging wave 205 (oc239): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002e_2260,
        dynamic_spray_fx_update_emitter9,
        "cDynamicSprayFX_UpdateEmitter9",
    ),
    (
        0x0030_9118,
        wscript_man_process_pending_request,
        "WScriptMan_ProcessPendingRequest",
    ),
    (
        0x0028_87f8,
        ssx_audio_system_update_mix_ramp,
        "SSXAudioSystem_UpdateMixRamp",
    ),
    (
        0x0040_d758,
        expf,
        "expf",
    ),
    (
        0x003c_08f8,
        snd_restore_vu0_context,
        "SND_RestoreVu0Context",
    ),
    (
        0x003c_7388,
        snd_parse_next_tag,
        "SND_ParseNextTag",
    ),
    (
        0x003f_fbc0,
        sce_pad_get_state,
        "scePadGetState",
    ),
    (
        0x0035_6b30,
        mod_obj_save,
        "ModObj_Save",
    ),
    (
        0x0041_1f70,
        subdf3,
        "__subdf3",
    ),
];

// ---------------------------------------------------------------------------
// cDynamicSprayFX_UpdateEmitter9 (0x2e2260)

/// Field offsets of the spray effect used by emitter 9.
mod spray9 {
    /// The rider object.
    pub const OBJ: u32 = 0x0;
    /// The spray level (seconds left).
    pub const LEVEL: u32 = 0x4;
    /// The emitter block (slot 9 lives at +0x1290).
    pub const P28: u32 = 0x28;
    /// The curve table (0x900: velocity scale).
    pub const P2C: u32 = 0x2c;
    /// Ring index into the bone list (0..30).
    pub const RING: u32 = 0x78;
    /// The bone list the ring walks.
    pub const BONES: u32 = 0x7c;
    /// The particle colour.
    pub const COL: u32 = 0x90;
    /// The speed compared against the limit.
    pub const SPEED: u32 = 0xb8;
    /// The spray mode (2: the level does not fade).
    pub const MODE: u32 = 0xd4;
    /// Emitter slot 9 inside the emitter block.
    pub const SLOT: u32 = 0x1290;
}

/// `void cDynamicSprayFX_UpdateEmitter9(cDynamicSprayFX* fx)` (0x2e2260):
/// while the spray has a level and the rider is fast enough, spawn one
/// particle at the next bone of a 30-entry ring (the bone's matrix
/// translation), coloured with alpha `clamp(1.5 level, 0, 1)` and thrown
/// along the rider's velocity (`obj + 0x1e0`) times the curve's scale; then
/// fade the level by 1/60 (not below 0) unless the mode is 2. Otherwise slot
/// 9 is only kept alive at the rider's bone `obj + 0x89c`.
pub fn dynamic_spray_fx_update_emitter9(c: &mut Cpu) {
    use spray9::*;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 20, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 21, 0x68);
        let fx = r32(c, S0);
        let gp = r32(c, GP);
        c.mtc1(0, 20); // 0.0
        c.lwc1(2, fx + LEVEL);
        c.g[V0][0] = c.lw(fx + P28);
        c.c_lt_s(20, 2);
        let has_level = c.fcc();
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(SLOT));
        let mut spawn = false;
        if has_level {
            c.cycles += 6;
            c.lwc1(1, gp.wrapping_sub(0x3a4c)); // speed limit
            c.lwc1(0, fx + SPEED);
            c.c_lt_s(1, 0);
            spawn = c.fcc();
            if !spawn {
                // (delay slot of the branch to the idle path)
                c.g[V0][0] = c.lw(fx + OBJ);
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw(fx + OBJ);
        }
        if spawn {
            // alpha = clamp(1.5 level, 0, 1) in f3.
            c.cycles += 9;
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(AT, 0);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.mul_s(0, 2, 0);
            c.c_le_s(20, 0);
            let positive = c.fcc();
            c.mov_s(3, 20);
            if positive {
                c.cycles += 1;
                c.min_s(3, 0, 1);
            }

            // col (alpha in w) -> sp+0; bone translation -> sp+0x10;
            // velocity * scale -> sp+0x20.
            c.cycles += 36;
            let sp0 = sp(c);
            c.lq(V0, fx + COL);
            c.g[A1][0] = sx(sp0.wrapping_add(0x10));
            c.g[T1][0] = c.lw(fx + RING);
            c.g[A2][0] = sx(sp0.wrapping_add(0x20));
            c.sq(V0, sp0);
            c.g[A3][0] = c.g[SP][0];
            c.swc1(3, sp0.wrapping_add(0xc));
            c.g[T1][0] = sx(r32(c, T1) << 2);
            c.lwc1(21, gp.wrapping_sub(0x3a48)); // 1/60
            c.g[T0][0] = 1;
            c.g[V0][0] = c.lw(fx + BONES);
            c.g[T2][0] = c.lw(fx + OBJ);
            c.mov_s(12, 21);
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            c.g[T4][0] = c.lw(fx + P2C);
            c.g[T3][0] = c.lw(r32(c, T2).wrapping_add(0x780));
            c.g[V0][0] = c.lw(r32(c, T1));
            c.g[V1][0] = c.lw(r32(c, T3).wrapping_add(0x30));
            c.g[V0][0] = sx(r32(c, V0) << 6);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.lq(T1, r32(c, V1).wrapping_add(0x30));
            c.sq(T1, sp0.wrapping_add(0x10));
            c.lwc1(0, r32(c, T4).wrapping_add(0x900));
            c.mfc1(V1, 0);
            c.lqc2(4, r32(c, T2).wrapping_add(0x1e0));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0.wrapping_add(0x30));
            c.lq(V0, sp0.wrapping_add(0x30));
            c.sq(V0, sp0.wrapping_add(0x20));
            call_ptr(c, 0x0037_17c0, 0x002e_2350)?;

            // Advance the ring (wrap at 30).
            c.g[V0][0] = c.lw(fx + RING);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V1][0] = ((c.g[V0][0] as i64) < 30) as u64;
            let in_range = c.g[V1][0] != 0;
            c.sw(fx + RING, c.g[V0][0]);
            if !in_range {
                c.cycles += 1;
                c.sw(fx + RING, 0);
            }

            // Fade the level unless the mode holds it.
            c.cycles += 4;
            c.g[V1][0] = c.lw(fx + MODE);
            c.g[V0][0] = 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 5;
                c.lwc1(0, fx + LEVEL);
                c.sub_s(0, 0, 21);
                c.max_s(0, 0, 20);
                c.swc1(0, fx + LEVEL);
                c.cycles += 1;
            }
            frame.restore(c, S0, 0x50);
        } else {
            // Idle: keep slot 9 alive at the rider's bone obj+0x89c.
            c.cycles += 13;
            let sp0 = sp(c);
            let obj = r32(c, V0);
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(12, gp.wrapping_sub(0x3a44)); // 1/60
            c.g[A2][0] = 0;
            c.g[T0][0] = c.lw(obj.wrapping_add(0x780));
            c.g[A3][0] = 0;
            c.g[T1][0] = c.lw(obj.wrapping_add(0x89c));
            c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(0x30));
            c.g[T1][0] = sx(r32(c, T1) << 6);
            c.g[T0][0] = 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T1)));
            c.lq(V0, r32(c, V1).wrapping_add(0x30));
            c.sq(V0, sp0);
            call_ptr(c, 0x0037_17c0, 0x002e_23c4)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_ProcessPendingRequest (0x309118)

/// Fields and callees of the world-script manager's pending request.
mod wscript_pending {
    use super::*;
    /// The pending request kind (1..6; 0: none).
    pub const REQUEST: u32 = 0x2ac;
    /// The request's mission id (-1: none).
    pub const MISSION: u32 = 0x2b0;
    /// The id passed to the audio system's mission end.
    pub const AUDIO_ID: u32 = 0x2a0;
    /// The request jump table (`0x4898f0`, six entries).
    pub const JUMP_TABLE: u32 = 0x0048_98f0;
    pub const RUN_SETUP_SCRIPTS: Callee = Callee {
        addr: 0x0030_8db8,
        f: ee::s10::WScriptMan_RunMissionSetupScripts,
    };
    pub const MISSION_COMMAND: Callee = Callee {
        addr: 0x0030_a868,
        f: ee::s10::WScriptMan_MissionCommand,
    };
    /// `SSXAudioSystem* SSXAudioSystem_GetInstance()`.
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    /// `SSXAudioSystem_OnMissionEnd(audio, id)`.
    pub const AUDIO_ON_MISSION_END: u32 = 0x0029_d6e0;
    /// `SSXAudioSystem` FE confirmation sound (`audio, 1`).
    pub const AUDIO_FE_SOUND: u32 = 0x0029_dbb0;
    /// Replay hook on the world's replay object (`world->game->0x28`).
    pub const REPLAY_ON_MISSION: u32 = 0x0026_f7b8;
    /// Playback manager hook (`world->game->0x34`).
    pub const PLAYBACK_ON_MISSION: u32 = 0x0026_cc48;
}

/// `void WScriptMan_ProcessPendingRequest(WScriptMan* self)` (0x309118):
/// carry out the mission request queued in `self+0x2ac` (1: start — run the
/// setup scripts, command 1, end the audio mission, poke the replay; 2:
/// command 3; 3 and 4: command 0x12 / 5, rerun the setup scripts, play the
/// FE sound; 5: poke the playback manager, command 1 (forced), rerun the
/// setup scripts; 6: command 0x12 and rerun the setup scripts), then clear
/// the request (kind 0, mission -1). The kind dispatches through the
/// compiled jump table at 0x4898f0.
pub fn wscript_man_process_pending_request(c: &mut Cpu) {
    use wscript_pending::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = c.lw(this + REQUEST);
        c.g[A0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V1][0] = (c.g[A0][0] < 6) as u64;
        c.g[V0][0] = 0x0049_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(JUMP_TABLE));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            // Mission command `cmd` on the queued mission (a3 = forced).
            let command = |c: &mut Cpu, a2: u64, ret: u32| {
                c.g[A1][0] = c.lw(this + MISSION);
                c.g[A0][0] = sx(this);
                c.g[A2][0] = a2;
                c.g[A3][0] = 0;
                MISSION_COMMAND.call(c, ret)
            };
            let setup_scripts = |c: &mut Cpu, ret: u32| {
                c.g[A0][0] = sx(this);
                RUN_SETUP_SCRIPTS.call(c, ret)
            };
            match target {
                // 1: start the mission.
                0x0030_9154 => {
                    c.cycles += 18;
                    setup_scripts(c, 0x0030_915c)?;
                    command(c, 1, 0x0030_9170)?;
                    call_ptr(c, AUDIO_GET_INSTANCE, 0x0030_9178)?;
                    c.g[A1][0] = c.lw(this + AUDIO_ID);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, AUDIO_ON_MISSION_END, 0x0030_9184)?;
                    let gp = r32(c, GP);
                    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x28));
                    call_ptr(c, REPLAY_ON_MISSION, 0x0030_9194)?;
                }
                // 2: command 3.
                0x0030_919c => {
                    c.cycles += 7;
                    command(c, 3, 0x0030_91b0)?;
                }
                // 3 / 4: command 0x12 / 5, setup scripts, FE sound.
                0x0030_91b8 | 0x0030_91c4 => {
                    if target == 0x0030_91b8 {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw(this + MISSION);
                        c.g[A2][0] = 0x12;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.lw(this + MISSION);
                        c.g[A2][0] = 5;
                    }
                    c.cycles += 12;
                    c.g[A3][0] = 0;
                    c.g[A0][0] = sx(this);
                    MISSION_COMMAND.call(c, 0x0030_91d8)?;
                    setup_scripts(c, 0x0030_91e0)?;
                    call_ptr(c, AUDIO_GET_INSTANCE, 0x0030_91e8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, AUDIO_FE_SOUND, 0x0030_91f4)?;
                }
                // 5: playback hook, forced command 1, setup scripts.
                0x0030_91fc => {
                    c.cycles += 13;
                    let gp = r32(c, GP);
                    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x34));
                    call_ptr(c, PLAYBACK_ON_MISSION, 0x0030_920c)?;
                    c.g[A1][0] = c.lw(this + MISSION);
                    c.g[A0][0] = sx(this);
                    c.g[A2][0] = 1;
                    c.g[A3][0] = 1;
                    MISSION_COMMAND.call(c, 0x0030_9220)?;
                    setup_scripts(c, 0x0030_9228)?;
                }
                // 6: command 0x12, setup scripts.
                0x0030_9230 => {
                    c.cycles += 7;
                    command(c, 0x12, 0x0030_9244)?;
                    setup_scripts(c, 0x0030_924c)?;
                }
                _ => {
                    c.jump_out(target, 0x0030_9118, 0x0030_926c);
                    return Err(crate::call::Unwound);
                }
            }
        }
        // Clear the request.
        c.cycles += 8;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sw(this + REQUEST, 0);
        c.g[V0][0] |= 0xffff;
        frame.restore_ra(c, 0);
        c.sw(this + MISSION, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateMixRamp (0x2887f8)

/// Fields of the audio system's mix ramp.
mod mix_ramp {
    /// Non-zero while a ramp is in progress.
    pub const DIRTY: u32 = 0x6410;
    /// Eleven per-channel ramp steps (0: idle).
    pub const DELTA: u32 = 0x6414;
    /// Eleven current levels.
    pub const CUR: u32 = 0x63e4;
    /// Eleven targets (`CUR + 0x5c`).
    pub const TARGET_FROM_CUR: u32 = 0x5c;
    /// Eleven channel pairs `{base, scaled}` (16 bytes apart; `scaled` at
    /// 0x62c8 + 16 i, `base` 4 bytes before it).
    pub const CHAN_SCALED: u32 = 0x62c8;
    /// The master level.
    pub const MASTER: u32 = 0x6308;
    /// Ten outputs: master times the gp table at 0x43c..0x460.
    pub const OUT: u32 = 0x636c;
}

/// `void SSXAudioSystem_UpdateMixRamp(SSXAudioSystem* self)` (0x2887f8):
/// while a mix ramp is running, step each of the eleven channel levels
/// toward its target (clamping and stopping the step on arrival, keeping
/// the ramp running otherwise), rescale each stepping channel by its level,
/// and recompute the ten master outputs from the gp gain table.
pub fn ssx_audio_system_update_mix_ramp(c: &mut Cpu) {
    use mix_ramp::*;
    c.cycles += 3;
    let this = r32(c, A0);
    c.g[V0][0] = c.lw(this + DIRTY);
    c.g[T1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.sw(this + DIRTY, 0);
        c.mtc1(0, 2); // 0.0
        c.g[A2][0] = sx(this + CHAN_SCALED);
        c.g[V1][0] = sx(this + CUR);
        c.g[A1][0] = sx(this + DELTA);
        c.g[T0][0] = 0;
        c.g[A3][0] = 10;
        loop {
            // Loop state lives in a1 (delta), v1 (cur), a2 (channel), t0
            // (byte offset), a3 (count down).
            let this = r32(c, A0);
            let delta = r32(c, A1);
            let cur = r32(c, V1);
            c.cycles += 5;
            c.lwc1(1, delta);
            c.c_eq_s(1, 2);
            if c.fcc() {
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            } else {
                // cur += delta (f1).
                c.cycles += 8;
                c.lwc1(0, cur);
                c.add_s(1, 0, 1);
                c.swc1(1, cur);
                c.lwc1(0, delta);
                c.c_lt_s(0, 2);
                let falling = c.fcc();
                c.g[V0][0] = sx(this + CUR);
                let arrived;
                if falling {
                    c.cycles += 5;
                    c.lwc1(0, cur + TARGET_FROM_CUR);
                    c.c_le_s(1, 0);
                    arrived = c.fcc();
                    if !arrived {
                        c.cycles += 2;
                    }
                } else {
                    c.cycles += 5;
                    c.lwc1(0, cur + TARGET_FROM_CUR);
                    c.c_le_s(0, 1);
                    arrived = c.fcc();
                    if arrived {
                        c.cycles += 1;
                    }
                }
                if arrived {
                    // Clamp to the target, stop stepping.
                    c.swc1(0, cur);
                    c.cycles += 1;
                    c.sw(delta, 0);
                } else {
                    c.sw(this + DIRTY, c.g[T1][0]);
                }
                // scaled = base * cur.
                c.cycles += 6;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                let chan = r32(c, A2);
                c.lwc1(0, chan.wrapping_sub(4));
                c.lwc1(1, r32(c, V0));
                c.mul_s(0, 0, 1);
                c.swc1(0, chan);
                c.g[A2][0] = sx(chan.wrapping_add(0x10));
            }
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            let more = (c.g[A3][0] as i64) >= 0;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_8820);
            }
        }
        // Outputs: master * gain[k].
        c.cycles += 31;
        let gp = r32(c, GP);
        let this = r32(c, A0);
        c.lwc1(8, gp + 0x43c);
        c.lwc1(9, gp + 0x440);
        c.lwc1(10, gp + 0x444);
        c.lwc1(6, gp + 0x448);
        c.lwc1(7, gp + 0x44c);
        c.lwc1(5, gp + 0x450);
        c.lwc1(4, gp + 0x454);
        c.lwc1(2, gp + 0x458);
        c.lwc1(1, gp + 0x45c);
        c.lwc1(0, this + MASTER);
        c.lwc1(3, gp + 0x460);
        c.mul_s(8, 0, 8);
        c.mul_s(9, 0, 9);
        c.mul_s(10, 0, 10);
        c.mul_s(6, 0, 6);
        c.swc1(8, this + OUT);
        c.mul_s(7, 0, 7);
        c.swc1(9, this + OUT + 0x4);
        c.mul_s(5, 0, 5);
        c.swc1(10, this + OUT + 0x8);
        c.mul_s(4, 0, 4);
        c.swc1(6, this + OUT + 0xc);
        c.mul_s(2, 0, 2);
        c.swc1(7, this + OUT + 0x10);
        c.mul_s(1, 0, 1);
        c.swc1(5, this + OUT + 0x14);
        c.mul_s(0, 0, 3);
        c.swc1(4, this + OUT + 0x18);
        c.swc1(2, this + OUT + 0x1c);
        c.swc1(1, this + OUT + 0x20);
        c.swc1(0, this + OUT + 0x24);
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// expf (0x40d758)

/// newlib's math-error machinery used by the `w_*f` wrappers.
mod libm_wrap {
    use super::*;
    /// `_LIB_VERSION` (-1 `_IEEE_`, 0 `_SVID_`, 2 `_POSIX_`), at 0x499688.
    pub const LIB_VERSION: u32 = 0x0049_9688;
    /// The name string "expf" the exception record points at.
    pub const EXPF_NAME: u32 = 0x0049_9088;
    /// `HUGE` (SVID overflow result, a double).
    pub const SVID_HUGE: u32 = 0x0049_9090;
    /// `HUGE_VAL` (a double).
    pub const HUGE_VAL: u32 = 0x0049_9680;
    /// `struct exception` on the frame: type, name, arg1, arg2, retval, err.
    pub const EXC_TYPE: u32 = 0x0;
    pub const EXC_NAME: u32 = 0x4;
    pub const EXC_ARG1: u32 = 0x8;
    pub const EXC_ARG2: u32 = 0x10;
    pub const EXC_RETVAL: u32 = 0x18;
    pub const EXC_ERR: u32 = 0x20;
    pub const IEEE754_EXPF: Callee = Callee {
        addr: 0x0040_e048,
        f: ee::s15::__ieee754_expf,
    };
    pub const FINITEF: Callee = Callee {
        addr: 0x0040_f8c0,
        f: ee::s15::finitef,
    };
    pub const EXTENDSFDF2: Callee = Callee {
        addr: 0x0041_3af8,
        f: ee::s15::__extendsfdf2,
    };
    pub const MATHERR: Callee = Callee {
        addr: 0x0040_f878,
        f: ee::s15::matherr,
    };
    pub const GETREENT: Callee = Callee {
        addr: 0x0041_5fb8,
        f: ee::s15::__getreent,
    };
    pub const TRUNCDFSF2: Callee = Callee {
        addr: 0x0041_27a8,
        f: ee::s15::__truncdfsf2,
    };
}

/// `float expf(float x)` (0x40d758), newlib's `w_expf.c` wrapper: compute
/// `__ieee754_expf(x)`; unless `_LIB_VERSION` is `_IEEE_` or `x` is not
/// finite, report overflow (`x > 88.72284`, type 3) or underflow (`x <
/// -103.97208`, type 4) through `matherr` / `errno = ERANGE` (POSIX skips
/// `matherr`) and return the exception's (double) result as a float.
pub fn expf(c: &mut Cpu) {
    use libm_wrap::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save_fpr(c, 20, 0x60);
        c.mov_s(20, 12);
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S1][0]);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S0][0]);
        frame.save_fpr(c, 21, 0x68);
        frame.save_ra(c, 0x50);
        c.g[S1][0] = 0x004a_0000;
        IEEE754_EXPF.call(c, 0x0040_d77c)?;
        c.g[S0][0] = c.lw(LIB_VERSION);
        c.g[V0][0] = u64::MAX;
        let ieee = c.g[S0][0] == c.g[V0][0];
        c.mov_s(21, 0); // the result
        'checked: {
            if ieee {
                break 'checked;
            }
            c.cycles += 4;
            c.mov_s(12, 20);
            FINITEF.call(c, 0x0040_d794)?;
            let finite = c.g[V0][0] != 0;
            c.mov_s(0, 21);
            if !finite {
                break 'checked;
            }
            let version = c.g[S0][0];
            // The error path: the exception record is filled, then
            // matherr (unless POSIX) and errno.
            let posix;
            c.cycles += 7;
            c.g[AT][0] = 0x42b1_7180; // 88.72284
            c.mtc1(AT, 0);
            c.c_lt_s(0, 20);
            let overflow = c.fcc();
            c.g[V0][0] = 0x004a_0000;
            if overflow {
                c.cycles += 10;
                c.g[V1][0] = 3;
                c.g[V0][0] = sx(EXPF_NAME);
                let sp0 = sp(c);
                c.sw(sp0 + EXC_TYPE, c.g[V1][0]);
                c.sw(sp0 + EXC_NAME, c.g[V0][0]);
                c.mov_s(12, 20);
                c.sw(sp0 + EXC_ERR, 0);
                EXTENDSFDF2.call(c, 0x0040_d7d4)?;
                let sp0 = sp(c);
                c.sd(sp0 + EXC_ARG1, c.g[V0][0]);
                c.sd(sp0 + EXC_ARG2, c.g[V0][0]);
                if version != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x004a_0000;
                    c.g[V1][0] = c.ld(HUGE_VAL);
                    c.sd(sp0 + EXC_RETVAL, c.g[V1][0]);
                } else {
                    c.cycles += 4;
                    c.g[AT][0] = 0x004a_0000;
                    c.g[V0][0] = c.ld(SVID_HUGE);
                    c.sd(sp0 + EXC_RETVAL, c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_sub(0x6978));
                c.g[V0][0] = 2;
                posix = c.g[V1][0] == c.g[V0][0];
                if !posix {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 7;
                c.g[AT][0] = 0xffff_ffff_c2cf_f1b5; // -103.97208
                c.mtc1(AT, 0);
                c.c_lt_s(20, 0);
                let underflow = c.fcc();
                c.g[V1][0] = 4;
                if !underflow {
                    c.cycles += 1;
                    c.mov_s(0, 21);
                    break 'checked;
                }
                c.cycles += 12;
                c.g[V0][0] = sx(EXPF_NAME);
                let sp0 = sp(c);
                c.sw(sp0 + EXC_TYPE, c.g[V1][0]);
                c.sw(sp0 + EXC_NAME, c.g[V0][0]);
                c.mov_s(12, 20);
                c.sw(sp0 + EXC_ERR, 0);
                EXTENDSFDF2.call(c, 0x0040_d848)?;
                let sp0 = sp(c);
                c.g[V1][0] = 0;
                c.sd(sp0 + EXC_ARG1, c.g[V0][0]);
                c.sd(sp0 + EXC_RETVAL, c.g[V1][0]);
                c.g[V1][0] = 2;
                posix = c.g[S0][0] == c.g[V1][0];
                c.sd(sp0 + EXC_ARG2, c.g[V0][0]);
            }
            let mut set_errno = posix;
            if !posix {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                MATHERR.call(c, 0x0040_d868)?;
                let handled = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(sp(c) + EXC_ERR);
                set_errno = !handled;
            }
            if set_errno {
                // errno = ERANGE.
                c.cycles += 5;
                GETREENT.call(c, 0x0040_d878)?;
                c.g[V1][0] = 0x22;
                c.sw(r32(c, V0), c.g[V1][0]);
                c.g[V0][0] = c.lw(sp(c) + EXC_ERR);
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                // errno = exc.err.
                c.cycles += 4;
                GETREENT.call(c, 0x0040_d894)?;
                c.g[V1][0] = c.lw(sp(c) + EXC_ERR);
                c.sw(r32(c, V0), c.g[V1][0]);
            }
            c.cycles += 4;
            c.g[A0][0] = c.ld(sp(c) + EXC_RETVAL);
            TRUNCDFSF2.call(c, 0x0040_d8a4)?;
            frame.restore_ra(c, 0x50);
            return finish(c, frame);
        }
        c.cycles += 1;
        frame.restore_ra(c, 0x50);
        finish(c, frame)
    });

    /// The shared epilogue (ra already reloaded).
    fn finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 6;
        let a = frame.at(c, 0x40);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S0][0] = c.ld(a);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// SND_RestoreVu0Context (0x3c08f8)

/// `SND_RestoreVu0Flags(int status)` (0x3c07a8).
const SND_RESTORE_VU0_FLAGS: Callee = Callee {
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

// ---------------------------------------------------------------------------
// SND_ParseNextTag (0x3c7388)

/// The tag-stream parser state of the sound library.
mod snd_tags {
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
// scePadGetState (0x3ffbc0)

/// libpad's per-port/slot table and the DMA buffer fields read here.
mod libpad {
    use super::*;
    /// The port/slot table (`0x526250 + port * 0x70 + slot * 0x1c`).
    pub const PORTS: u32 = 0x0052_6250;
    pub const PORT_STRIDE: u32 = 0x70;
    pub const SLOT_STRIDE: u32 = 0x1c;
    /// Non-zero while the port is open.
    pub const OPEN: u32 = 0x10;
    /// The DMA buffer's state byte (6: stable) and request state (2: busy).
    pub const STATE: u32 = 0x70;
    pub const REQ_STATE: u32 = 0x71;
    /// `u8* libpad_GetDmaBuffer(int port, int slot)` (0x3ff9a8).
    pub const GET_DMA_BUFFER: Callee = Callee {
        addr: 0x003f_f9a8,
        f: ee::s15::libpad_GetDmaBuffer,
    };
}

/// `int scePadGetState(int port, int slot)` (0x3ffbc0): 0x63 when the
/// port is not open; otherwise the DMA buffer's state byte, except that a
/// stable pad (6) with a request still running (2) reports 5 (`scePadStateExecCmd`).
pub fn sce_pad_get_state(c: &mut Cpu) {
    use libpad::*;
    guest(c, |c| {
        c.cycles += 14;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A2, V1); // port * 0x70
        c.mult(0, A0, A1, A0); // slot * 0x1c
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(PORTS);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(PORTS.wrapping_add(r32(c, A0)));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(OPEN));
        let open = c.g[V1][0] != 0;
        c.g[V0][0] = 0x63;
        if !open {
            c.cycles += 1;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[A2][0];
            GET_DMA_BUFFER.call(c, 0x003f_fc00)?;
            c.g[A0][0] = c.g[V0][0];
            let buf = r32(c, A0);
            c.g[V1][0] = 6;
            c.g[V0][0] = c.lbu(buf + STATE);
            let stable = c.g[V0][0] == c.g[V1][0];
            if stable {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(buf + REQ_STATE);
                c.g[V0][0] = 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = c.lbu(buf + STATE);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 5;
                }
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// ModObj_Save (0x356b30)

/// Fields and callees of `ModObj_Save`.
mod mod_obj {
    use super::*;
    /// The three saved words.
    pub const WORDS: [u32; 3] = [0x20, 0x24, 0x28];
    /// The modifier block (0: none).
    pub const BLOCK: u32 = 0x1c;
    /// The stream's `Write(stream, data, size)` slot (`{delta, fn}` at 8).
    pub const WRITE_SLOT: u32 = 0x8;
    pub const WORLD_OBJ_SAVE: Callee = Callee {
        addr: 0x0034_fe90,
        f: ee::s11::WorldObj_Save,
    };
    pub const MODIFIER_BLOCK_SAVE: Callee = Callee {
        addr: 0x0035_3448,
        f: ee::s11::ModifierBlock_Save,
    };
}

/// `void ModObj_Save(ModObj* self, Stream* stream)` (0x356b30): save the
/// world object, then write the words at +0x20, +0x24, +0x28 and a
/// "has modifier block" flag (4 bytes each) with `stream->Write`, then let
/// the modifier block (+0x1c) save itself.
pub fn mod_obj_save(c: &mut Cpu) {
    use mod_obj::*;
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        WORLD_OBJ_SAVE.call(c, 0x0035_6b4c)?;
        let this = r32(c, S0);
        let stream = r32(c, S1);
        // stream->Write(&self->word, 4) for each word.
        for (word, ret) in WORDS.into_iter().zip([0x0035_6b68u32, 0x0035_6b84, 0x0035_6ba0]) {
            c.g[V1][0] = c.lw(stream);
            c.g[A1][0] = sx(this.wrapping_add(word));
            c.g[A2][0] = 4;
            let vt = r32(c, V1);
            c.g[A0][0] = c.lh(vt.wrapping_add(WRITE_SLOT));
            c.g[V0][0] = c.lw(vt.wrapping_add(WRITE_SLOT + 4));
            let target = r32(c, V0);
            c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
            call_ptr(c, target, ret)?;
        }
        // The flag, on the frame.
        c.g[A3][0] = c.lw(stream);
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(this.wrapping_add(BLOCK));
        c.g[A2][0] = 4;
        let vt = r32(c, A3);
        c.g[A0][0] = c.lh(vt.wrapping_add(WRITE_SLOT));
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        c.g[V1][0] = c.lw(vt.wrapping_add(WRITE_SLOT + 4));
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        let target = r32(c, V1);
        c.sw(sp(c), c.g[V0][0]);
        call_ptr(c, target, 0x0035_6bc8)?;
        c.g[A0][0] = c.lw(this.wrapping_add(BLOCK));
        let has_block = c.g[A0][0] != 0;
        frame.restore(c, S0, 0x30);
        if has_block {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            MODIFIER_BLOCK_SAVE.call(c, 0x0035_6bdc)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// __subdf3 (0x411f70)

/// libgcc's soft-float helpers (`fp-bit.c`).
mod softfp {
    use super::*;
    pub const UNPACK_D: Callee = Callee {
        addr: 0x0041_1c38,
        f: ee::s15::__unpack_d,
    };
    pub const FPADD_PARTS_D: Callee = Callee {
        addr: 0x0041_1cd8,
        f: ee::s15::_fpadd_parts_d,
    };
    pub const PACK_D: Callee = Callee {
        addr: 0x0041_1b08,
        f: ee::s15::__pack_d,
    };
    /// The frame: unpacked a at 0, unpacked b at 0x20, the sum at 0x40,
    /// the packed arguments at 0x60 / 0x68.
    pub const A_PARTS: u32 = 0x0;
    pub const B_PARTS: u32 = 0x20;
    pub const SUM_PARTS: u32 = 0x40;
    pub const A_PACKED: u32 = 0x60;
    pub const B_PACKED: u32 = 0x68;
    /// The sign word of an unpacked number.
    pub const SIGN: u32 = 0x4;
}

/// `double __subdf3(double a /*a0*/, double b /*a1*/)` (0x411f70):
/// libgcc's soft-float `a - b`: unpack both, flip b's sign, add the parts,
/// pack the result (v0).
pub fn subdf3(c: &mut Cpu) {
    use softfp::*;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x90);
        let sp0 = sp(c);
        c.sd(sp0 + A_PACKED, c.g[A0][0]);
        c.sd(sp0 + B_PACKED, c.g[A1][0]);
        c.g[A0][0] = sx(sp0 + A_PACKED);
        c.sd(sp0 + 0x70, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        UNPACK_D.call(c, 0x0041_1f90)?;
        let sp0 = sp(c);
        c.g[S0][0] = sx(sp0 + B_PARTS);
        c.g[A0][0] = sx(sp0 + B_PACKED);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D.call(c, 0x0041_1fa0)?;
        // b.sign ^= 1
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(sp0 + B_PARTS + SIGN);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp0 + SUM_PARTS);
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] ^= 1;
        c.sw(sp0 + B_PARTS + SIGN, c.g[V0][0]);
        FPADD_PARTS_D.call(c, 0x0041_1fbc)?;
        c.g[A0][0] = c.g[V0][0];
        PACK_D.call(c, 0x0041_1fc4)?;
        frame.restore_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------- cDynamicSprayFX_UpdateEmitter9

    /// `cDynamicSprayFX_UpdateEmitter9`: level zero / positive, speed below /
    /// above the limit, alpha clamped or not, the ring wrapping at 30 or not,
    /// mode 2 (no fade) or not; the real slot-9 emitter spawn runs.
    #[test]
    fn dynamic_spray_fx_update_emitter9_matches() {
        use super::spray9::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::put_spray_slot;
        let f = |x: f32| x.to_bits();
        check(
            ee::s09::f_002e2260__generated,
            dynamic_spray_fx_update_emitter9,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e22, 0);
                elf_image(c);
                c.pc = 0x002e_2260;
                let (fx, obj, em, curve, bones, holder, mats) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x2000,
                    DATA + 0x5600,
                    DATA + 0x5800,
                    DATA + 0x5900,
                    DATA + 0x9000,
                );
                let (world, g, players) = (DATA + 0x6000, DATA + 0x6200, DATA + 0x6400);
                c.bus.write32(fx + OBJ, obj);
                c.bus.write32(fx + P28, em);
                c.bus.write32(fx + P2C, curve);
                c.bus.write32(fx + BONES, bones);
                c.bus.write32(
                    fx + LEVEL,
                    match seed % 5 {
                        0 => 0,
                        1 => f(0.25),
                        2 => f(0.9),
                        3 => f(0.01),
                        _ => f(-0.5),
                    },
                );
                c.bus.write32(
                    fx + SPEED,
                    f([100.0f32, 300.0, 2000.0][((seed / 5) % 3) as usize]),
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x3a4c), f(250.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x3a48), f(1.0 / 60.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x3a44), f(1.0 / 60.0));
                c.bus.write32(
                    fx + RING,
                    [0u32, 28, 29, r.next_u32() % 30][((seed / 15) % 4) as usize],
                );
                c.bus.write32(fx + MODE, if seed % 7 == 0 { 2 } else { r.next_u32() % 4 });
                for k in 0..4u32 {
                    c.bus.write32(fx + COL + 4 * k, r.f32(10.0));
                    c.bus.write32(obj + 0x1e0 + 4 * k, r.f32(100.0));
                }
                c.bus.write32(curve + 0x900, r.f32(3.0));
                for k in 0..30u32 {
                    c.bus.write32(bones + 4 * k, r.next_u32() % 16);
                }
                c.bus.write32(obj + 0x780, holder);
                c.bus.write32(holder + 0x30, mats);
                c.bus.write32(obj + 0x89c, r.next_u32() % 16);
                for k in 0..16u32 {
                    for j in 0..4u32 {
                        c.bus.write32(mats + 0x40 * k + 0x30 + 4 * j, r.f32(50.0));
                    }
                }
                put_spray_slot(c, &mut r, em + SLOT, DATA + 0x8000);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, 0);
                c.bus.write32(GAME_GP + 0xa0c, r.next_u32());
                c.g[A0][0] = sx(fx);
            },
        );
    }

    // ------------------------------------------- WScriptMan_ProcessPendingRequest

    /// `WScriptMan_ProcessPendingRequest`: every request kind 0..7 (0 and 7
    /// out of the jump table). The real callees take their short paths: no
    /// world sections (setup scripts), mission commands that stop at the
    /// game-state check (`0x535c10 != 4`, behind a fake back-end interface
    /// call that may unwind), an audio system whose mission end and FE
    /// sound find nothing playing, an idle replay and a busy playback.
    #[test]
    fn wscript_man_process_pending_request_matches() {
        use super::wscript_pending::*;
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, elf_image, DATA, GAME_GP};
        const BE_LIBRARY: u32 = DATA + 0x8_6000;
        const BE_IFACE: u32 = DATA + 0x8_7000;
        const BE_VTABLE: u32 = DATA + 0x8_7100;
        check(
            ee::s10::f_00309118__generated,
            wscript_man_process_pending_request,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_9118, 2);
                elf_image(c);
                c.pc = 0x0030_9118;
                let (this, view, sections) = (DATA, DATA + 0x400, DATA + 0x500);
                let (audio, world, game, riders, rider) =
                    (DATA + 0x1_0000, DATA + 0x2000, DATA + 0x2100, DATA + 0x2200, DATA + 0x2300);
                let (replay, playback) = (DATA + 0x3000, DATA + 0x3100);
                c.bus.write32(this + REQUEST, seed % 8);
                c.bus.write32(this + MISSION, r.next_u32());
                c.bus.write32(this + AUDIO_ID, r.next_u32());
                // Setup scripts: a world view without sections.
                c.bus.write32(this + 0x28c, view);
                c.bus.write32(view, sections);
                c.bus.write32(sections + 0x14, 0);
                // Mission command: the back end's interface (a fake call),
                // then not in game state 4.
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VTABLE);
                c.bus.write16(BE_VTABLE + 8, 0);
                c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
                c.bus.write8(0x0053_5c10, (r.next_u32() % 4) as u8);
                // The audio system: nothing playing.
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(audio + 0x5fd4, 0);
                c.bus.write32(audio + 0x5fdc, 0xffff_ffff);
                c.bus.write32(audio + 0x118 + 0x408, 0);
                c.bus.write32(audio + 0x608c, 2);
                // The world: rider without wipeout loops, replay and playback.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, riders);
                c.bus.write32(riders + 0x28, rider);
                c.bus.write32(rider + 0x774, 0xffff_ffff);
                c.bus.write32(rider + 0x778, 0xffff_ffff);
                c.bus.write32(game + 0x28, replay);
                c.bus.write32(replay, 0);
                c.bus.write32(game + 0x34, playback);
                c.bus.write32(playback + 0x70, 1);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------- SSXAudioSystem_UpdateMixRamp

    /// `SSXAudioSystem_UpdateMixRamp`: idle or ramping; per channel an idle
    /// step, a rising or falling step that arrives (clamped) or not, with
    /// edge-case floats mixed in.
    #[test]
    fn ssx_audio_system_update_mix_ramp_matches() {
        use super::mix_ramp::*;
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        let f = |x: f32| x.to_bits();
        check(
            ee::s07::f_002887f8__generated,
            ssx_audio_system_update_mix_ramp,
            300,
            &|c, seed| {
                let mut r = Rng(0x2887_f800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0028_87f8;
                let this = DATA;
                c.bus.write32(this + DIRTY, if seed % 9 == 0 { 0 } else { r.next_u32() | 1 });
                for i in 0..11u32 {
                    let cur = (r.next_u32() % 200) as f32 * 0.01;
                    let target = (r.next_u32() % 200) as f32 * 0.01;
                    let delta = match r.next_u32() % 6 {
                        0 => 0.0,
                        1 => 0.05,
                        2 => -0.05,
                        3 => 1.0,
                        4 => -1.0,
                        _ => f32::from_bits(r.edge_f32()),
                    };
                    c.bus.write32(this + CUR + 4 * i, f(cur));
                    c.bus.write32(this + CUR + TARGET_FROM_CUR + 4 * i, f(target));
                    c.bus.write32(this + DELTA + 4 * i, f(delta));
                    c.bus.write32(this + CHAN_SCALED - 4 + 0x10 * i, r.f32(2.0));
                    c.bus.write32(this + CHAN_SCALED + 0x10 * i, r.next_u32());
                }
                c.bus.write32(this + MASTER, r.f32(1.0));
                for k in 0..10u32 {
                    c.bus.write32(GAME_GP + 0x43c + 4 * k, r.f32(1.0));
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------ expf

    /// `expf`: arguments in range, overflowing, underflowing, at the
    /// thresholds and not finite, under each `_LIB_VERSION` (IEEE, SVID,
    /// XOPEN, POSIX); the real libm helpers (`matherr` returns 0).
    #[test]
    fn expf_matches() {
        use super::libm_wrap::*;
        use crate::difftest::{check, elf_image, scramble, Rng, GAME_GP};
        let f = |x: f32| x.to_bits();
        check(
            ee::s15::f_0040d758__generated,
            expf,
            300,
            &|c, seed| {
                let mut r = Rng(0x40d7_5800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0040_d758;
                let version = [0xffff_ffffu32, 0, 1, 2][(seed % 4) as usize];
                c.bus.write32(LIB_VERSION, version);
                let x = match (seed / 4) % 9 {
                    0 => f(1.5),
                    1 => f(-2.25),
                    2 => f(88.8),
                    3 => f(500.0),
                    4 => f(-104.5),
                    5 => f(-2000.0),
                    6 => f(88.72284),
                    7 => r.edge_f32(),
                    _ => r.f32(120.0),
                };
                c.f[12] = x;
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
                    c.bus.write32(ctx + 0x10 * k + 4 * (r.next_u32() % 4), r.edge_f32());
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

    // -------------------------------------------------------- scePadGetState

    /// `scePadGetState`: ports and slots 0..3, the port closed or open, a
    /// DMA buffer (either half, by the real `libpad_GetDmaBuffer`) whose
    /// state is 6 or another value, request state 2 or another value.
    #[test]
    fn sce_pad_get_state_matches() {
        use super::libpad::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check_lohi, elf_image, DATA};
        check_lohi(
            ee::s15::f_003ffbc0__generated,
            sce_pad_get_state,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003f_fbc0, 0);
                elf_image(c);
                c.pc = 0x003f_fbc0;
                let (port, slot) = (r.next_u32() % 2, r.next_u32() % 4);
                let entry = PORTS + port * PORT_STRIDE + slot * SLOT_STRIDE;
                let buf = DATA + 0x100 * (r.next_u32() % 8);
                c.bus.write32(entry, buf);
                c.bus.write32(entry + OPEN, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(buf + 0x58, r.next_u32() % 4);
                c.bus.write32(buf + 0xd8, r.next_u32() % 4);
                for half in [buf, buf + 0x80] {
                    let state = if r.next_u32() % 2 == 0 { 6 } else { r.next_u32() % 8 };
                    let req = if r.next_u32() % 2 == 0 { 2 } else { r.next_u32() % 4 };
                    c.bus.write8(half + STATE, state as u8);
                    c.bus.write8(half + REQ_STATE, req as u8);
                }
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
            },
        );
    }

    // ----------------------------------------------------------- ModObj_Save

    /// `ModObj_Save`: with and without a modifier block (an empty one: only
    /// its header is written); the stream's write slots are fakes, some of
    /// which unwind; the real `WorldObj_Save` and `ModifierBlock_Save` run.
    #[test]
    fn mod_obj_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{fill, DATA};
        covered(
            ee::s11::f_00356b30__generated,
            mod_obj_save,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_6b30, 7);
                c.pc = 0x0035_6b30;
                let (this, stream, block) = (DATA + 0x100, DATA + 0x400, DATA + 0x800);
                fill(c, &mut r, this, 0x10);
                c.bus.write32(this + 0x1c, if seed % 3 == 0 { 0 } else { block });
                let vt = vtable(c, &mut r, &[8, 0x30]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    // -------------------------------------------------------------- __subdf3

    /// `__subdf3`: random doubles and edge cases (zeros of both signs,
    /// infinities, NaNs, denormals, equal operands) through the real
    /// soft-float helpers.
    #[test]
    fn subdf3_matches() {
        use crate::difftest::{check, scramble, Rng};
        let edge = |r: &mut Rng| -> u64 {
            let sign = ((r.next_u32() & 1) as u64) << 63;
            let mant = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & ((1u64 << 52) - 1);
            sign | match r.next_u32() % 8 {
                0 => 0,
                1 => mant,
                2 => 0x7ff0_0000_0000_0000,
                3 => 0x7ff0_0000_0000_0000 | mant | 1,
                4 => 1.0f64.to_bits(),
                _ => (((r.next_u32() % 200 + 923) as u64) << 52) | mant,
            }
        };
        check(
            ee::s15::f_00411f70__generated,
            subdf3,
            300,
            &|c, seed| {
                let mut r = Rng(0x0041_1f70 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0041_1f70;
                let a = edge(&mut r);
                let b = match seed % 5 {
                    0 => a,
                    1 => a ^ (1 << 63),
                    _ => edge(&mut r),
                };
                c.g[A0][0] = a;
                c.g[A1][0] = b;
            },
        );
    }
}
