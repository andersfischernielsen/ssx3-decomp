//! Staging wave 212 (oc306): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_6948, sndstrm_calc_data_rate, "SNDSTRM_CalcDataRate"),
    (0x0026_d760, replay_record_snapshot, "Replay_RecordSnapshot"),
    (0x0035_3118, modifier_block_free_uv_scroll, "ModifierBlock_FreeUVScroll"),
    (0x0023_40b8, game_state_finish_save_state, "GameStateFinish_SaveState"),
    (
        0x0028_8940,
        ssx_audio_is_closer_to_second_view,
        "SSXAudioSystem_IsCloserToSecondView",
    ),
    (0x0013_5b30, rider_spin_sub_set_spin_direction, "RiderSpinSub_SetSpinDirection"),
    (0x0028_8ac0, ssx_audio_get_view_pos, "SSXAudioSystem_GetViewPos"),
    (0x0011_7638, rider_metrix_add_combo_points, "RiderMetrix_AddComboPoints"),
    (0x0011_9068, rider_metrix_add_trick_history, "RiderMetrix_AddTrickHistory"),
    (
        0x002a_02d8,
        ssx_audio_speech_wipeout_recover,
        "SSXAudioSystem_SpeechWipeoutRecover",
    ),
];

// ------------------------------------------------------ SNDSTRM_CalcDataRate

/// Stream header fields read by `SNDSTRM_CalcDataRate`.
pub(crate) mod sndstrm_rate {
    /// +0 (u16) sample rate, +2 (u8) channel count, +3 (u8) codec type.
    pub const HDR_RATE: u32 = 0x0;
    pub const HDR_CHANNELS: u32 = 0x2;
    pub const HDR_CODEC: u32 = 0x3;
}

/// `int SNDSTRM_CalcDataRate(const StreamHeader* h)` (0x3b6948): the bytes
/// per second a sound stream consumes: `rate * channels * k >> 8` with `k`
/// the codec's bytes per 256 samples (5: 0x92, 10: 0x88, 4: 0x33, 22: 0x66,
/// 8: 0x200, anything else 0); codecs 16 and 23 return `channels * 8`.
/// LO/HI keep the last multiply (`rate * channels`, then `* k`).
pub fn sndstrm_calc_data_rate(c: &mut Cpu) {
    use sndstrm_rate as k;
    c.cycles += 7;
    let h = Addr(abi::arg(c, 0));
    c.g[A1][0] = h.off(k::HDR_CHANNELS).lbu(c);
    c.g[A2][0] = 0;
    c.g[V1][0] = h.off(k::HDR_CODEC).lbu(c);
    c.g[A0][0] = h.off(k::HDR_RATE).lhu(c);
    c.mult(0, A0, A0, A1);
    let codec = c.g[V1][0];
    // the compare chain: bytes per 256 samples, or None for `channels * 8`
    let per_256 = match codec {
        5 => {
            c.g[V0][0] = 0x5;
            c.cycles += 2;
            Some(0x92)
        }
        10 => {
            c.g[V0][0] = 0xa;
            c.cycles += 3 + 2;
            Some(0x88)
        }
        4 => {
            c.g[V0][0] = 0x4;
            c.cycles += 3 + 2 + 2;
            Some(0x33)
        }
        0x16 => {
            c.g[V0][0] = 0x16;
            c.cycles += 3 + 2 + 2 + 2;
            Some(0x66)
        }
        8 => {
            c.g[V0][0] = 0x8;
            c.cycles += 3 + 2 + 2 + 2 + 2;
            Some(0x200)
        }
        0x10 => {
            c.cycles += 3 + 2 + 2 + 2 + 2;
            None
        }
        0x17 => {
            c.cycles += 3 + 2 + 2 + 2 + 2 + 2;
            None
        }
        _ => {
            c.cycles += 3 + 2 + 2 + 2 + 2 + 2;
            Some(0)
        }
    };
    match per_256 {
        Some(bytes) => {
            c.g[A2][0] = bytes;
            c.cycles += 3;
            c.mult(0, V0, A0, A2);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
        }
        None => {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
        }
    }
    ret(c);
}

// ----------------------------------------------------- Replay_RecordSnapshot

/// Layouts and callees of `Replay_RecordSnapshot`.
pub(crate) mod record_snapshot {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// Replay manager: +0x3c8 the replay frame counter.
    pub const MGR_FRAME: u32 = 0x3c8;
    /// Snapshot: +0x1c the 4-byte frame descriptor, +0x30 the frame number.
    pub const SNAP_DESC: u32 = 0x1c;
    pub const SNAP_FRAME: u32 = 0x30;

    pub const ALLOC_SNAPSHOT: Callee = Callee {
        addr: 0x0027_0130,
        f: ee::s07::f_00270130,
    };
    pub const WORLD_STATE_WRITE: Callee = Callee {
        addr: 0x0026_d818,
        f: ee::s07::f_0026d818,
    };
    pub const SNAPSHOT_HOOK_A: Callee = Callee {
        addr: 0x0026_ed88,
        f: ee::s07::f_0026ed88,
    };
}

/// `Snapshot* Replay_RecordSnapshot(ReplayMan* self, const FrameDesc* desc)`
/// (0x26d760): take a free snapshot (`Replay_AllocSnapshot`), copy the
/// 4-byte frame descriptor into it (unaligned, `+0x1c`), write the world
/// state into it, stamp it with the manager's frame counter (`+0x30`), run
/// the (empty) snapshot hook and return it.
pub fn replay_record_snapshot(c: &mut Cpu) {
    use record_snapshot as k;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        k::ALLOC_SNAPSHOT.call(c, 0x0026_d780)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        let desc = abi::r32(c, S2);
        let snap = abi::r32(c, S0);
        c.lwl(2, desc.wrapping_add(3));
        c.lwr(2, desc);
        c.swl(2, snap.wrapping_add(k::SNAP_DESC + 3));
        c.swr(2, snap.wrapping_add(k::SNAP_DESC));
        c.g[A1][0] = c.g[S0][0];
        k::WORLD_STATE_WRITE.call(c, 0x0026_d7a0)?;
        let mgr = abi::r32(c, S1);
        c.g[V0][0] = Addr(mgr.wrapping_add(k::MGR_FRAME)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        let snap = abi::r32(c, S0);
        Addr(snap.wrapping_add(k::SNAP_FRAME)).sw(c, c.g[V0][0]);
        k::SNAPSHOT_HOOK_A.call(c, 0x0026_d7b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ----------------------------------------------- ModifierBlock_FreeUVScroll

/// ModifierBlock: +0x4 the UV-scroll modifier (a plain heap block).
pub(crate) const MODIFIER_BLOCK_UV_SCROLL: u32 = 0x4;

/// `void ModifierBlock_FreeUVScroll(ModifierBlock* self)` (0x353118): when
/// the block has a UV-scroll modifier, free it (`__builtin_delete`) and
/// clear the slot.
pub fn modifier_block_free_uv_scroll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let block = abi::r32(c, S0);
        c.g[A0][0] = Addr(block.wrapping_add(MODIFIER_BLOCK_UV_SCROLL)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e50, 0x0035_313c)?;
            let block = abi::r32(c, S0);
            Addr(block.wrapping_add(MODIFIER_BLOCK_UV_SCROLL)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------ GameStateFinish_SaveState

pub(crate) const FINISH_BASE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateFinish_SaveState(GameStateFinish* self, Stream* s)`
/// (0x2340b8): the base state's save, then the words at `+0x10` and
/// `+0x14`, each written with the stream's virtual `Write(s, p, 4)`
/// (vtable slot `+8`).
pub fn game_state_finish_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        FINISH_BASE_SAVE_STATE.call(c, 0x0023_40d4)?;
        for (field, ret) in [(0x10, 0x0023_40f0), (0x14, 0x0023_410c)] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(8).lh(c);
            c.g[V0][0] = Addr(vt).off(0xc).lw(c);
            c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
            let write = r32(c, V0);
            call_ptr(c, write, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------- SSXAudioSystem_IsCloserToSecondView

/// Callees of `SSXAudioSystem_IsCloserToSecondView`.
pub(crate) mod closer_view {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const GET_LISTENER_COUNT: Callee = Callee {
        addr: 0x0028_8ce8,
        f: ee::s07::SSXAudioSystem_GetListenerCount,
    };
    pub const GET_VIEW_POS: Callee = Callee {
        addr: 0x0028_8ac0,
        f: ee::s07::SSXAudioSystem_GetViewPos,
    };
}

/// `bool SSXAudioSystem_IsCloserToSecondView(void* self, sSndVec4* pos)`
/// (0x288940): in split screen (two or more listeners), whether `pos` is
/// at least as close to the second view as to the first (squared 4D
/// distances from VU0: `!(d0 < d1)`); false with a single listener.
pub fn ssx_audio_is_closer_to_second_view(c: &mut Cpu) {
    use closer_view as k;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        k::GET_LISTENER_COUNT.call(c, 0x0028_895c)?;
        let single = (c.g[V0][0] as i64) < 2;
        c.g[V0][0] = 0;
        if !single {
            c.cycles += 41;
            // d0 = view0 - pos at sp+0x10, then copied to sp+0
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            k::GET_VIEW_POS.call(c, 0x0028_8974)?;
            c.lqc2(3, r32(c, V0));
            c.lqc2(4, r32(c, S1));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x10));
            c.g[A0][0] = c.g[S0][0];
            c.lq(V0, frame.at(c, 0x10));
            c.g[A1][0] = 1;
            c.sq(V0, frame.at(c, 0));
            // d1 = view1 - pos at sp+0x20, then copied to sp+0x10
            k::GET_VIEW_POS.call(c, 0x0028_8998)?;
            c.lqc2(3, r32(c, V0));
            c.lqc2(4, r32(c, S1));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x20));
            c.lq(V1, frame.at(c, 0x20));
            c.g[V0][0] = 0;
            c.sq(V1, frame.at(c, 0x10));
            // f1 = dot(d0, d0)
            closer_view_dot(c, frame.at(c, 0), A0);
            c.mtc1(A0, 1);
            // f0 = dot(d1, d1)
            closer_view_dot(c, frame.at(c, 0x10), V1);
            c.mtc1(V1, 0);
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });

    /// `dot(d, d)` of the vector at `at` into vf4.x (vf6.x = 1.0), moved
    /// to GPR `gpr`.
    fn closer_view_dot(c: &mut Cpu, at: u32, gpr: usize) {
        c.lqc2(3, at);
        c.lqc2(5, at);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(gpr, 4);
    }
}

// -------------------------------------------- RiderSpinSub_SetSpinDirection

/// Layout and literals of `RiderSpinSub_SetSpinDirection`.
pub(crate) mod spin_direction {
    /// RiderSpinSub: +0x10 spin rate (±2π or 0), +0x14 flip rate (±π or 0),
    /// +0x18/+0x1c/+0x20/+0x24 accumulated angles, +0x28/+0x2c pending
    /// spin/flip deltas (folded in and cleared).
    pub const SPIN_RATE: u32 = 0x10;
    pub const FLIP_RATE: u32 = 0x14;
    pub const FLIP_ACC: u32 = 0x18;
    pub const SPIN_ACC: u32 = 0x1c;
    pub const SPIN_ACC2: u32 = 0x20;
    pub const FLIP_ACC2: u32 = 0x24;
    pub const PENDING_SPIN: u32 = 0x28;
    pub const PENDING_FLIP: u32 = 0x2c;
    /// `gp-0x734c` π, `gp-0x7348` -π, `gp-0x7344` 2π, `gp-0x7340` -2π.
    pub const GP_PI: u32 = 0x734c;
    pub const GP_NEG_PI: u32 = 0x7348;
    pub const GP_TWO_PI: u32 = 0x7344;
    pub const GP_NEG_TWO_PI: u32 = 0x7340;
}

/// `void RiderSpinSub_SetSpinDirection(RiderSpinSub* self, float spin,
/// float flip)` (0x135b30): set the flip rate from the sign of `flip`
/// (π, -π or 0) and the spin rate from the sign of `spin` (2π, -2π or 0),
/// then fold the pending spin/flip deltas into the accumulated angles and
/// clear them.
pub fn rider_spin_sub_set_spin_direction(c: &mut Cpu) {
    use spin_direction as k;
    let this = abi::arg(c, 0);
    let gp = r32(c, GP);
    // the sign of f13 (flip) picks the flip rate, f12 (spin) the spin rate
    for (arg, field, pos, neg) in [
        (13, k::FLIP_RATE, k::GP_PI, k::GP_NEG_PI),
        (12, k::SPIN_RATE, k::GP_TWO_PI, k::GP_NEG_TWO_PI),
    ] {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, arg);
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(pos));
        } else {
            c.cycles += 4;
            c.c_lt_s(arg, 0);
            if c.fcc() {
                c.cycles += 2;
                c.lwc1(0, gp.wrapping_sub(neg));
            }
        }
        c.swc1(0, this.wrapping_add(field));
    }
    c.cycles += 17;
    c.lwc1(4, this.wrapping_add(k::PENDING_SPIN));
    c.lwc1(5, this.wrapping_add(k::PENDING_FLIP));
    c.lwc1(1, this.wrapping_add(k::SPIN_ACC));
    c.lwc1(2, this.wrapping_add(k::FLIP_ACC));
    c.lwc1(3, this.wrapping_add(k::SPIN_ACC2));
    c.add_s(1, 1, 4);
    c.lwc1(0, this.wrapping_add(k::FLIP_ACC2));
    c.add_s(2, 2, 5);
    c.add_s(3, 3, 4);
    c.sw(this.wrapping_add(k::PENDING_FLIP), 0);
    c.add_s(0, 0, 5);
    c.swc1(1, this.wrapping_add(k::SPIN_ACC));
    c.swc1(2, this.wrapping_add(k::FLIP_ACC));
    c.swc1(3, this.wrapping_add(k::SPIN_ACC2));
    c.swc1(0, this.wrapping_add(k::FLIP_ACC2));
    c.sw(this.wrapping_add(k::PENDING_SPIN), 0);
    ret(c);
}

// ------------------------------------------------ SSXAudioSystem_GetViewPos

pub(crate) const VIEW_POS_GET_VIEW: Callee = Callee {
    addr: 0x0028_b210,
    f: ee::s07::SSXAudioSystem_GetView,
};

/// View: +0x20 the camera position (a 4-vector).
pub(crate) const VIEW_POS_OFFSET: u32 = 0x20;

/// `sSndVec4* SSXAudioSystem_GetViewPos(void* self, int index)` (0x288ac0):
/// the position of listener view `index` (`SSXAudioSystem_GetView(index)
/// + 0x20`).
pub fn ssx_audio_get_view_pos(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        VIEW_POS_GET_VIEW.call(c, 0x0028_8ad0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(VIEW_POS_OFFSET));
        frame.close(c);
        Ok(())
    });
}

// ----------------------------------------------- RiderMetrix_AddComboPoints

/// Layout and callee of `RiderMetrix_AddComboPoints`.
pub(crate) mod combo_points {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// RiderMetrix: +0x9c combo count, +0xa0 combo score, +0xa4 peak (-1:
    /// reset), +0x1b0 the HUD effects object.
    pub const COMBO_COUNT: u32 = 0x9c;
    pub const COMBO_SCORE: u32 = 0xa0;
    pub const PEAK: u32 = 0xa4;
    pub const HUD: u32 = 0x1b0;
    /// HUD effects: +0x1d4 the combo-number effect (its first word the
    /// state, 0x34 while showing), +0x1e8 its current value.
    pub const HUD_COMBO_FX: u32 = 0x1d4;
    pub const HUD_COMBO_VALUE: u32 = 0x1e8;
    pub const FX_SHOWING: u64 = 0x34;
    /// `gp-0x7b14`: 0.05, the multiplier step per combo.
    pub const GP_STEP: u32 = 0x7b14;

    pub const SHOW_NUMBER_DELAYED: Callee = Callee {
        addr: 0x0011_71a8,
        f: ee::s00::cHUDFX_ShowNumberDelayed,
    };
}

/// `void RiderMetrix_AddComboPoints(RiderMetrix* self, int points)`
/// (0x117638): for positive points, bump the combo count `n`, add `points *
/// clamp((n + 10) * 0.05, 0.5, 2.0)` (truncated) to the combo score, reset
/// the peak to -1, and show the combo count on the HUD (type 3, with the
/// effect's value unless it is already showing, no delay).
pub fn rider_metrix_add_combo_points(c: &mut Cpu) {
    use combo_points as k;
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 18;
            let this = abi::arg(c, 0);
            let gp = r32(c, GP);
            // f0 = (n + 10) * 0.05 with n the new combo count (a3)
            c.g[V0][0] = Addr(this.wrapping_add(k::COMBO_COUNT)).lw(c);
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 3);
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(1));
            c.lwc1(2, gp.wrapping_sub(k::GP_STEP));
            c.mtc1(A3, 0);
            c.cvt_s_w(0, 0);
            c.g[AT][0] = 0x3f00_0000; // 0.5
            c.mtc1(AT, 1);
            Addr(this.wrapping_add(k::COMBO_COUNT)).sw(c, c.g[A3][0]);
            c.g[AT][0] = 0x4000_0000; // 2.0
            c.mtc1(AT, 4);
            c.add_s(0, 0, 3);
            c.mul_s(0, 0, 2);
            // f5 = clamp(f0, 0.5, 2.0)
            c.c_le_s(1, 0);
            c.mov_s(5, 1);
            if c.fcc() {
                c.cycles += 1;
                c.min_s(5, 0, 4);
            }
            // score += (int)(points * f5); peak = -1
            c.cycles += 13;
            c.mtc1(A1, 0);
            c.cvt_s_w(0, 0);
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
            c.mtc1(AT, 1);
            c.g[V1][0] = Addr(this.wrapping_add(k::COMBO_SCORE)).lw(c);
            c.swc1(1, this.wrapping_add(k::PEAK));
            c.mul_s(0, 0, 5);
            c.g[A1][0] = Addr(this.wrapping_add(k::HUD)).lw(c);
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            Addr(this.wrapping_add(k::COMBO_SCORE)).sw(c, c.g[V1][0]);
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                let hud = r32(c, A1);
                c.g[V0][0] = Addr(hud.wrapping_add(k::HUD_COMBO_FX)).lw(c) ^ k::FX_SHOWING;
                c.g[A2][0] = 0;
                c.g[A0][0] = sx(hud.wrapping_add(k::HUD_COMBO_FX));
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = Addr(hud.wrapping_add(k::HUD_COMBO_VALUE)).lw(c);
                }
                c.cycles += 3;
                c.mtc1(0, 12);
                c.g[A1][0] = 3;
                k::SHOW_NUMBER_DELAYED.call(c, 0x0011_76e8)?;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------- RiderMetrix_AddTrickHistory

/// RiderMetrix: +0x60 the three most recent trick ids (0: empty).
pub(crate) const TRICK_HISTORY: u32 = 0x60;
/// The loop head of `RiderMetrix_AddTrickHistory` (the back-edge event pc).
pub(crate) const TRICK_HISTORY_LOOP: u32 = 0x0011_908c;

/// `void RiderMetrix_AddTrickHistory(RiderMetrix* self, int trick)`
/// (0x119068): record `trick` in the first empty one of the three recent
/// slots at `+0x60`; a repeat of the last recorded trick (the slot holding
/// it is followed by an empty one) is stored over itself instead. A full
/// history is left unchanged.
pub fn rider_metrix_add_trick_history(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this.wrapping_add(TRICK_HISTORY)).lw(c);
    c.g[T1][0] = 0;
    if c.g[V0][0] == 0 {
        // empty history: the trick goes into the first slot
        c.cycles += 2;
        Addr(this.wrapping_add(TRICK_HISTORY)).sw(c, c.g[A1][0]);
        ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = sx(this.wrapping_add(TRICK_HISTORY));
    // t1 = slot index, t0 = the history, a3 = &history[t1], a2 = t1 + 1
    loop {
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, T1) << 2);
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, A3)).lw(c);
        c.g[A2][0] = sx(r32(c, T1).wrapping_add(1));
        if c.g[V1][0] == c.g[A1][0] {
            // the same trick again: if the next slot is empty, store over it
            c.cycles += 3;
            let not_last = (c.g[T1][0] as i64) < 2;
            c.g[V0][0] = sx(r32(c, A2) << 2);
            if not_last {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V1][0] == 0 {
                    c.cycles += 2;
                    Addr(r32(c, A3)).sw(c, c.g[A1][0]);
                    ret(c);
                    return;
                }
            }
        }
        c.cycles += 3;
        let more = (c.g[A2][0] as i64) < 3;
        c.g[V0][0] = more as u64;
        c.g[T1][0] = c.g[A2][0];
        if !more {
            break;
        }
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, A2) << 2));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        if c.g[V1][0] == 0 {
            // the next slot is free: record the trick there
            c.cycles += 1;
            Addr(r32(c, V0)).sw(c, c.g[A1][0]);
            break;
        }
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(TRICK_HISTORY));
        if event_due(c) {
            deliver_event_here(c, TRICK_HISTORY_LOOP);
        }
    }
    c.cycles += 2;
    ret(c);
}

// --------------------------------------- SSXAudioSystem_SpeechWipeoutRecover

/// Layouts, globals and callees of `SSXAudioSystem_SpeechWipeoutRecover`.
pub(crate) mod wipeout_recover {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// Rider: +0x470 course progress (< 0 before the finish), +0x6c0 its
    /// scripting object (vtable `+0x28` position, `+0x48` "is AI"), +0x87c
    /// the local-human flag.
    pub const RIDER_PROGRESS: u32 = 0x470;
    pub const RIDER_OBJ: u32 = 0x6c0;
    pub const RIDER_LOCAL: u32 = 0x87c;
    pub const VT_POSITION: u32 = 0x28;
    pub const VT_IS_AI: u32 = 0x48;
    /// SSXAudioSystem: +0x5560 the speech system.
    pub const SYS_SPEECH: u32 = 0x5560;
    /// `gp+0x5e8`: the hearing radius (/100) for the listener check;
    /// `gp+0x173c`: the speech post hook.
    pub const GP_RADIUS: u32 = 0x5e8;
    pub const GP_POST_HOOK: u32 = 0x173c;
    /// The recover lines by track category (1, 2, anything else).
    pub const LINE_CAT1: u64 = 0x2088;
    pub const LINE_CAT2: u64 = 0x20a0;
    pub const LINE_OTHER: u64 = 0x20a1;

    pub const IS_SPEECH_VOLUME_ON: Callee = Callee {
        addr: 0x0029_f0a0,
        f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
    };
    pub const IS_INSTANCE_READY: Callee = Callee {
        addr: 0x002b_0e28,
        f: ee::s08::SpeechSystem_IsInstanceReady,
    };
    pub const GET_RIDER_SPEECH_MASK: Callee = Callee {
        addr: 0x0029_f330,
        f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
    };
    pub const IS_TRACK_GROUP5: Callee = Callee {
        addr: 0x002a_3f90,
        f: ee::s08::SSXAudioSystem_IsTrackGroup5,
    };
    pub const IS_EVENT_MODE4: Callee = Callee {
        addr: 0x002a_4040,
        f: ee::s08::SSXAudioSystem_IsEventMode4,
    };
    pub const TRACK_TO_CATEGORY: Callee = Callee {
        addr: 0x002a_1e20,
        f: ee::s08::SSXAudioSystem_TrackToCategory,
    };
    pub const SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };
    // Cross-shard direct calls (emitted as `call_indirect`).
    pub const IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;
    pub const FIND_RIDER: u32 = 0x0028_5d98;
    pub const FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;
    pub const GET_BE_LIBRARY: u32 = 0x0028_b1e8;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const RACE_GET_TRACK: u32 = 0x0014_4bc0;
    pub const RIDER_GET_CHARACTER: u32 = 0x0012_3128;
    pub const SPCH_MAKE_ID: u32 = 0x003d_8008;
}

/// `void SSXAudioSystem_SpeechWipeoutRecover(SSXAudioSystem* sys, Rider*
/// rider)` (0x2a02d8): the "back on the board" speech after a wipeout.
/// Only with speech volume on, outside replays, before the finish
/// (progress < 0), for the followed rider when the rider is an AI or not a
/// local human, with the speech instance ready and the rider heard by a
/// listener view (within `gp+0x5e8 * 100`). The line depends on the
/// track's category (1: 0x2088, 2: 0x20a0, else 0x20a1); when its event
/// context is accepted it is posted through the hook with the rider's
/// speech mask and priority 1 (track group 5 or event mode 4) or 3.
pub fn ssx_audio_speech_wipeout_recover(c: &mut Cpu) {
    use wipeout_recover as k;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        k::IS_SPEECH_VOLUME_ON.call(c, 0x002a_0304)?;
        // true: out through 0x2a053c (one more cycle)
        if wipeout_recover_body(c)? {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });

    /// Everything after the volume check (s1 = sys, s0 = rider): `Ok(true)`
    /// when it leaves through 0x2a053c.
    fn wipeout_recover_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
        use wipeout_recover as k;
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, k::IS_REPLAY_PLAYING, 0x002a_0314)?;
        if c.g[V0][0] != 0 {
            return Ok(false);
        }
        // finished (progress >= 0): nothing to say
        c.cycles += 6;
        c.lwc1(1, r32(c, S0).wrapping_add(k::RIDER_PROGRESS));
        c.mtc1(0, 0);
        c.c_le_s(0, 1);
        c.g[V0][0] = 1;
        if !c.fcc() {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            return Ok(false);
        }
        // s2 = the rider is an AI or not a local human: then it must be
        // the followed rider
        c.cycles += 10;
        let obj = r32(c, S0).wrapping_add(k::RIDER_OBJ);
        c.g[V1][0] = Addr(obj).lw(c);
        c.g[A1][0] = sx(obj);
        c.g[S5][0] = c.g[A1][0];
        c.g[S2][0] = 0;
        wipeout_recover_obj_call(c, k::VT_IS_AI, 0x002a_0360)?;
        if c.g[V0][0] != 0 {
            c.g[S2][0] = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(k::RIDER_LOCAL)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[S2][0] = 1;
            }
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S2][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = u64::MAX;
            call_ptr(c, k::FIND_RIDER, 0x002a_0384)?;
            if c.g[V0][0] != c.g[S0][0] {
                return Ok(false);
            }
        }
        // s2 = the speech system
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(k::SYS_SPEECH));
        c.g[A1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        k::IS_INSTANCE_READY.call(c, 0x002a_039c)?;
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
        // s3 = speech mask, s4 = priority
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        k::GET_RIDER_SPEECH_MASK.call(c, 0x002a_03ac)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        k::IS_TRACK_GROUP5.call(c, 0x002a_03b8)?;
        c.g[S4][0] = 1;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            k::IS_EVENT_MODE4.call(c, 0x002a_03c8)?;
            c.g[S4][0] = 1;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[S4][0] = 3;
            }
        }
        // heard by a listener view?
        c.cycles += 11;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(k::RIDER_OBJ)).lw(c);
        wipeout_recover_obj_call(c, k::VT_POSITION, 0x002a_03e8)?;
        c.lwc1(12, r32(c, GP).wrapping_add(k::GP_RADIUS));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, k::FIND_VIEW_IN_RANGE, 0x002a_03f8)?;
        if (c.g[V0][0] as i64) < 0 {
            return Ok(false);
        }
        // the track's category picks the line
        c.cycles += 14;
        call_ptr(c, k::GET_BE_LIBRARY, 0x002a_0408)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, k::BE_GET_INTERFACE, 0x002a_0414)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, k::RACE_GET_TRACK, 0x002a_041c)?;
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        k::TRACK_TO_CATEGORY.call(c, 0x002a_0428)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 2;
        if c.g[V1][0] == 1 {
            c.cycles += 13;
            if !wipeout_recover_context(c, k::LINE_CAT1, [0x002a_0440, 0x002a_0464])? {
                return Ok(true);
            }
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = k::LINE_CAT1;
            c.cycles += 10;
            wipeout_recover_post(c, [0x002a_04c4, 0x002a_04dc])?;
            return Ok(false);
        }
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 13;
            if !wipeout_recover_context(c, k::LINE_CAT2, [0x002a_0488, 0x002a_04ac])? {
                return Ok(true);
            }
            c.cycles += 2;
            c.g[A1][0] = 0;
            c.g[A2][0] = k::LINE_CAT2;
            c.cycles += 10;
            wipeout_recover_post(c, [0x002a_04c4, 0x002a_04dc])?;
            return Ok(false);
        }
        c.cycles += 13;
        if !wipeout_recover_context(c, k::LINE_OTHER, [0x002a_04ec, 0x002a_0510])? {
            return Ok(true);
        }
        c.cycles += 9;
        c.g[A1][0] = 0;
        c.g[A2][0] = k::LINE_OTHER;
        wipeout_recover_post(c, [0x002a_0524, 0x002a_053c])?;
        Ok(true)
    }

    /// `jalr` to the method at `slot` of the rider's scripting object
    /// (`s5`, vtable in v1): `a0 = obj + delta`.
    fn wipeout_recover_obj_call(c: &mut Cpu, slot: u32, ret_to: u32) -> Flow {
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
        call_ptr(c, f, ret_to)
    }

    /// `SetEventContext(speech, 0, line, rider, Rider_GetCharacter(rider),
    /// 0, 0.0, 0)`: whether it was accepted (a0 = 1 after).
    fn wipeout_recover_context(
        c: &mut Cpu,
        line: u64,
        rets: [u32; 2],
    ) -> Result<bool, crate::call::Unwound> {
        use wipeout_recover as k;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, k::RIDER_GET_CHARACTER, rets[0])?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = line;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        k::SET_EVENT_CONTEXT.call(c, rets[1])?;
        let accepted = c.g[V0][0] != 0;
        c.g[A0][0] = 1;
        Ok(accepted)
    }

    /// Post the line (a0 = 1, a1 = 0, a2 = line already set): the hook
    /// `gp+0x173c` with `(SPCH_MakeId(1, 0, line), 2, mask, priority)`.
    fn wipeout_recover_post(c: &mut Cpu, rets: [u32; 2]) -> Flow {
        use wipeout_recover as k;
        call_ptr(c, k::SPCH_MAKE_ID, rets[0])?;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(k::GP_POST_HOOK)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        let hook = r32(c, V1);
        c.g[A1][0] = 2;
        call_ptr(c, hook, rets[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, check_paths, check_paths_lohi, fill, put_words, scramble, Rng, DATA};

    /// Every codec of the chain, plus unknown ones, with random rates and
    /// channel counts.
    #[test]
    fn sndstrm_calc_data_rate_matches() {
        const CODECS: [u8; 10] = [5, 10, 4, 22, 8, 16, 23, 0, 0xff, 9];
        check_paths_lohi(
            ee::s13::f_003b6948__generated,
            sndstrm_calc_data_rate,
            200,
            &[12, 15, 17, 19, 21, 20, 22, 23],
            &|c, seed| {
                let mut r = Rng(0x3b69_4800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003b_6948;
                c.g[A0][0] = sx(DATA);
                c.bus.write16(DATA, r.next_u32() as u16);
                c.bus.write8(DATA + 2, r.next_u32() as u8);
                c.bus.write8(DATA + 3, CODECS[(seed % 10) as usize]);
            },
        );
    }

    /// A replay manager whose free list holds one snapshot; for some seeds
    /// the free count is 0 so `Replay_AllocSnapshot` first recycles (an
    /// empty used list: a short pass). The descriptor is unaligned for some
    /// seeds. Fixture as `world_state_save_matches` (fakes for the world
    /// state's virtual writers, the dummy world).
    #[test]
    fn replay_record_snapshot_matches() {
        use crate::game::fe::fepopup3::tests::dummy_world;
        use crate::game::fe::feasyncfile::tests::call_machine;
        check(
            ee::s07::f_0026d760__generated,
            replay_record_snapshot,
            40,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x26d7_60, 2);
                dummy_world(c);
                c.pc = 0x0026_d760;
                let this = DATA + 0x100;
                let snap = DATA + 0x800;
                for off in (0x3a0..0x3d0).step_by(4) {
                    c.bus.write32(this + off, 0);
                }
                // free list (head, tail, count) at +0x3bc; used list at +0x3ac
                c.bus.write32(this + 0x3bc, snap);
                c.bus.write32(this + 0x3c0, snap);
                c.bus.write32(this + 0x3c4, if seed % 4 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(this + 0x3c8, r.next_u32());
                for off in (0..0x40).step_by(4) {
                    c.bus.write32(snap + off, 0);
                }
                c.bus.write32(snap + 0x1c, 0xdead_beef);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x300 + (seed & 3));
                c.bus.write32(DATA + 0x300, r.next_u32());
                c.bus.write32(DATA + 0x304, r.next_u32());
            },
        );
    }

    /// No UV-scroll block, or one really allocated (a pool-sized one or a
    /// heap one) that `__builtin_delete` frees.
    #[test]
    fn modifier_block_free_uv_scroll_matches() {
        use crate::game::fe::feasyncfile::tests::{call_machine, run};
        check(
            ee::s11::f_00353118__generated,
            modifier_block_free_uv_scroll,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x35_3118, 0);
                let block = DATA + 0x100;
                c.bus.write32(block, r.next_u32());
                let uv = if seed % 3 == 0 {
                    0
                } else {
                    let size = if seed % 3 == 1 { 0x30 } else { 0x400 };
                    run(
                        c,
                        0x0031_7e30,
                        ee::s10::operator_new_named,
                        &[size, 0x0048_15a0, 0, 0],
                    ) as u32
                };
                c.bus.write32(block + MODIFIER_BLOCK_UV_SCROLL, uv);
                c.g[A0][0] = sx(block);
                c.pc = 0x0035_3118;
            },
        );
    }

    /// A stream whose `Write` slot is a fake (logs every argument; one of
    /// the three calls may unwind); the real base GameState_SaveState makes
    /// the first call.
    #[test]
    fn game_state_finish_save_state_matches() {
        use crate::difftest::fakes;
        fakes::covered(
            ee::s06::f_002340b8__generated,
            game_state_finish_save_state,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2340b8, 3);
                let stream = DATA + 0x200;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(DATA + 0x400 + (r.next_u32() & 0xfc));
                c.g[A1][0] = sx(stream);
                c.pc = 0x0023_40b8;
            },
        );
    }

    /// A world with the audio app and its listener list (+0x84): no list
    /// (one listener), 0..3 listeners, and two views at positions near or
    /// far from `pos` (so either view can be the closer one, and equal
    /// distances happen).
    #[test]
    fn ssx_audio_is_closer_to_second_view_matches() {
        use crate::difftest::{put_vec, GAME_GP};
        check(
            ee::s07::f_00288940__generated,
            ssx_audio_is_closer_to_second_view,
            300,
            &|c, seed| {
                let mut r = Rng(0x2889_4011 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0028_8940;
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA;
                let app = DATA + 0x100;
                let list = DATA + 0x200;
                let pos = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, app);
                let has_list = seed % 7 != 0;
                c.bus.write32(app + 0x84, if has_list { list } else { 0 });
                c.bus.write32(list + 0x10, seed % 4);
                let spread = if seed % 3 == 0 { 1.0 } else { 20.0 };
                for i in 0..3 {
                    let view = DATA + 0x1000 + i * 0x40;
                    c.bus.write32(list + 4 + i * 4, view);
                    put_vec(
                        c,
                        view + 0x20,
                        [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)],
                    );
                }
                if seed % 5 == 0 {
                    // both views at the same place: equal distances
                    for i in 0..4 {
                        let w = c.bus.read32(DATA + 0x1020 + 4 * i);
                        c.bus.write32(DATA + 0x1060 + 4 * i, w);
                    }
                }
                put_vec(
                    c,
                    pos,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)],
                );
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(pos);
            },
        );
    }

    /// Spin and flip of every sign (and zero, and the odd edge values), the
    /// rate literals at their gp slots, random angles and pending deltas.
    #[test]
    fn rider_spin_sub_set_spin_direction_matches() {
        use crate::difftest::GAME_GP;
        use spin_direction as k;
        check_paths(
            ee::s01::f_00135b30__generated,
            rider_spin_sub_set_spin_direction,
            300,
            &[4 + 3 + 4 + 3 + 17, 4 + 4 + 2 + 4 + 4 + 17, 4 + 4 + 4 + 4 + 17],
            &|c, seed| {
                let mut r = Rng(0x135b_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0013_5b30;
                c.g[GP][0] = sx(GAME_GP);
                for (slot, v) in [
                    (k::GP_PI, 0x4049_0fdbu32),
                    (k::GP_NEG_PI, 0xc049_0fdb),
                    (k::GP_TWO_PI, 0x40c9_0fdb),
                    (k::GP_NEG_TWO_PI, 0xc0c9_0fdb),
                ] {
                    c.bus.write32(GAME_GP - slot, v);
                }
                let sign = |r: &mut Rng, m: u32| match m % 4 {
                    0 => r.f32(10.0) & 0x7fff_ffff,
                    1 => r.f32(10.0) | 0x8000_0000,
                    2 => 0,
                    _ => r.edge_f32(),
                };
                c.f[12] = sign(&mut r, seed);
                c.f[13] = sign(&mut r, seed / 4);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                for off in (0..0x30).step_by(4) {
                    c.bus.write32(this + off, r.f32(100.0));
                }
            },
        );
    }

    /// The audio app's listener list with three views; index 0..2.
    #[test]
    fn ssx_audio_get_view_pos_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s07::f_00288ac0__generated,
            ssx_audio_get_view_pos,
            60,
            &|c, seed| {
                let mut r = Rng(0x288a_c0d1 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0028_8ac0;
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA;
                let app = DATA + 0x100;
                let list = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, app);
                c.bus.write32(app + 0x84, list);
                c.bus.write32(list + 0x10, 3);
                for i in 0..3 {
                    c.bus.write32(list + 4 + i * 4, DATA + 0x1000 + (r.next_u32() & 0xff0));
                }
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(seed % 3);
            },
        );
    }

    /// Zero/negative/positive points, combo counts on both sides of the
    /// clamp (multiplier below 0.5, inside, above 2.0), with no HUD, or a
    /// HUD whose combo effect is showing (0x34) or not.
    #[test]
    fn rider_metrix_add_combo_points_matches() {
        use crate::difftest::GAME_GP;
        use combo_points as k;
        check(
            ee::s00::f_00117638__generated,
            rider_metrix_add_combo_points,
            300,
            &|c, seed| {
                let mut r = Rng(0x1176_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_7638;
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP - k::GP_STEP, 0x3d4c_cccd); // 0.05
                let this = DATA + 0x100;
                let hud = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => sx(r.next_u32() | 0x8000_0000),
                    2 => sx(r.next_u32() % 100 + 1),
                    _ => sx(r.next_u32() % 100_000 + 1),
                };
                let count = match (seed / 5) % 4 {
                    0 => (r.next_u32() % 20) as i32 - 20, // multiplier < 0.5
                    1 => (r.next_u32() % 30) as i32,      // inside
                    2 => 30 + (r.next_u32() % 1000) as i32, // > 2.0
                    _ => r.next_u32() as i32,
                };
                c.bus.write32(this + k::COMBO_COUNT, count as u32);
                c.bus.write32(this + k::COMBO_SCORE, r.next_u32());
                c.bus.write32(this + k::PEAK, r.f32(10.0));
                c.bus.write32(this + k::HUD, if seed % 3 == 0 { 0 } else { hud });
                c.bus.write32(
                    hud + k::HUD_COMBO_FX,
                    if seed % 2 == 0 { 0x34 } else { r.next_u32() % 0x40 },
                );
                c.bus.write32(hud + k::HUD_COMBO_VALUE, r.next_u32());
            },
        );
    }

    /// Histories of 0..3 entries drawn from a small id set (so repeats of the
    /// trick, at the last entry or earlier, happen), and a full history.
    #[test]
    fn rider_metrix_add_trick_history_matches() {
        check(
            ee::s00::f_00119068__generated,
            rider_metrix_add_trick_history,
            300,
            &|c, seed| {
                let mut r = Rng(0x1190_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_9068;
                let this = DATA + 0x100;
                let filled = seed % 4;
                for i in 0..3 {
                    let v = if i < filled { 1 + r.next_u32() % 3 } else { 0 };
                    c.bus.write32(this + TRICK_HISTORY + 4 * i, v);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(1 + r.next_u32() % 3);
            },
        );
    }

    /// The same with events falling due at the loop's back edge (off,
    /// delivered, or a VBlank requesting an unwind): two or three entries,
    /// mostly a new trick so the walk reaches the back edges.
    #[test]
    fn rider_metrix_add_trick_history_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s00::f_00119068__generated,
            rider_metrix_add_trick_history,
            200,
            &|c, seed| {
                let mut r = Rng(0x1190_6801 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                for i in 0..3 {
                    let v = if i < 2 + (seed / 3) % 2 { 1 + r.next_u32() % 2 } else { 0 };
                    c.bus.write32(this + TRICK_HISTORY + 4 * i, v);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(if seed % 4 == 0 { 1 } else { 3 });
                schedule(c, &mut r, seed, 0x0011_9068)
            },
        );
    }

    /// The world of `SSXAudioSystem_RiderSay207D`'s test (audio system,
    /// riders, scripting vtables, speech settings, listener view, race
    /// manager, speech system, BE singletons; fakes for the post hook and
    /// the BE interface method, some unwinding) with rider a recovering:
    /// speech volume off, replay, progress after the finish (1.0, 0.0), AI
    /// or local-human combinations against the followed rider, speech
    /// instance not ready, heard or not (far rider, tiny radius), track
    /// group/event mode priorities, all three track categories, and lines
    /// already live (contexts rejected).
    #[test]
    fn ssx_audio_speech_wipeout_recover_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::sound::soundsys::tests::rs207d_world;
        const SYS: u32 = DATA + 0x20_0000;
        const RIDER_A: u32 = DATA + 0x21_0000;
        check(
            ee::s08::f_002a02d8__generated,
            ssx_audio_speech_wipeout_recover,
            300,
            &|c, seed| {
                let mut r = rs207d_world(c, seed);
                c.pc = 0x002a_02d8;
                let progress = match seed % 11 {
                    1 => 1.0f32,
                    8 => 0.0,
                    _ => -1.0 - (r.next_u32() % 100) as f32,
                };
                c.bus.write32(RIDER_A + 0x470, progress.to_bits());
                let radius = if seed % 17 == 9 { 0.0001f32 } else { 10.0 };
                c.bus.write32(GAME_GP + 0x5e8, radius.to_bits());
                // BENewRaceInt_GetTrack: race record 0's track id at
                // 0x43d950 (categories 1, 2, 4 and "none").
                c.bus.write32(0x0053_5c08, 0);
                c.bus
                    .write32(0x0043_d950, [0u32, 2, 4, 0x20][(seed / 7 % 4) as usize]);
                c.bus.write8(0x0053_5c10, if seed % 3 == 1 { 4 } else { 0 });
                if seed % 5 == 1 {
                    let speech = SYS + 0x5560;
                    for (i, code) in [0x2088u32, 0x20a0, 0x20a1].into_iter().enumerate() {
                        let slot = speech + 0x18 + 0x20 * i as u32;
                        c.bus.write32(slot, 1);
                        c.bus.write32(slot + 4, 50);
                        c.bus.write32(slot + 8, 0);
                        c.bus.write32(slot + 0xc, code);
                    }
                }
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = RIDER_A as u64;
            },
        );
    }
}
