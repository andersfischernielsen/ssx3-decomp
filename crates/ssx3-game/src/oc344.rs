//! Staging module oc344 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc344`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0010_4c80,
        rider_anim_seq_end_type0a,
        "RiderAnim_SeqEndType0A",
    ),
    (
        0x0010_4cc8,
        rider_anim_play_event_sound,
        "RiderAnim_PlayEventSound",
    ),
    (0x0010_4de0, rider_anim_load_state, "RiderAnim_LoadState"),
    (
        0x0010_7578,
        rider_try_start_handplant,
        "Rider_TryStartHandplant",
    ),
    (
        0x0010_8c80,
        computer_actor_rider_ctor,
        "ComputerActorRider_ctor",
    ),
    (
        0x0010_8e88,
        computer_actor_rider_init,
        "ComputerActorRider_Init",
    ),
    (
        0x0010_a8e8,
        computer_actor_rider_load_state,
        "ComputerActorRider_LoadState",
    ),
    (
        0x0010_ad78,
        computer_actor_rider_think_board_press,
        "ComputerActorRider_ThinkBoardPress",
    ),
    (
        0x0010_aed8,
        computer_actor_rider_think_railslide,
        "ComputerActorRider_ThinkRailslide",
    ),
    (
        0x0010_bc98,
        computer_actor_rider_has_path_event_0f,
        "ComputerActorRider_HasPathEvent0F",
    ),
];

/// `rN + off` (32-bit address arithmetic on a register).
#[inline(always)]
fn oc344_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `gp - off` (a small-data constant).
#[inline(always)]
fn oc344_gp(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

/// `lui/ori at, bits; mtc1 at, f` (a float literal through `at`).
#[inline(always)]
fn oc344_lit(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(AT, f);
}

/// The compiled VU0 vector length: `VF4.x = |VF3|` into Q (dot product,
/// `vsqrt`, `vwaitq`).
#[inline(always)]
fn oc344_vu0_length_vf3(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

// ---------------------------------------------------------------------------
// RiderAnim sequence-end handler 0A, event sound, LoadState
// (0x104c80, 0x104cc8, 0x104de0; src/seg/seg_1218.cpp)

/// `RiderAnim_ClearFlagsAndPlayAnim` (0x312bd0, through the function table).
pub(crate) const RIDER_ANIM_SEQ_END_0A_PLAY: u32 = 0x0031_2bd0;

/// The animation the type-0A sequence end plays.
pub(crate) const RIDER_ANIM_SEQ_END_0A_ANIM: u64 = 0x1b2;

/// `void RiderAnim_SeqEndType0A(RiderAnim* a, AnimSeqList* list, AnimSeq*
/// seq)` (0x104c80): the sequence-end handler for state type 0A; plays
/// animation 0x1b2 through `RiderAnim_ClearFlagsAndPlayAnim(a, list, seq,
/// 0x1b2)`.
pub fn rider_anim_seq_end_type0a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = RIDER_ANIM_SEQ_END_0A_ANIM;
        call_ptr(c, RIDER_ANIM_SEQ_END_0A_PLAY, 0x0010_4c90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The rider's sound owner (`RiderAnim+0x60`).
pub(crate) const RIDER_ANIM_SOUND_OWNER: u32 = 0x60;

/// `SSXAudioSystem_OnAnimSoundEvent` (0x289b18, through the function table).
pub(crate) const RIDER_ANIM_ON_SOUND_EVENT: u32 = 0x0028_9b18;

/// `void RiderAnim_PlayEventSound(RiderAnim* a, u16 event)` (0x104cc8): when
/// the animation has a sound owner (`+0x60`), pass the event to the audio
/// system (`gp+0x410`): `OnAnimSoundEvent(audio, owner, event & 0xffff)`.
pub fn rider_anim_play_event_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc344_at(c, A0, RIDER_ANIM_SOUND_OWNER));
        c.g[A2][0] = c.g[A1][0] & 0xffff;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(oc344_at(c, GP, 0x410));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, RIDER_ANIM_ON_SOUND_EVENT, 0x0010_4ce8)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The six animation sequence lists (`RiderAnim+0x50`, 8 bytes each).
pub(crate) const RIDER_ANIM_SEQ_LISTS: u32 = 0x50;

/// `AnimSeqList_Clear` (0x314668) and `AnimSeqList_LoadState` (0x314880),
/// both through the function table.
pub(crate) const RIDER_ANIM_LIST_CLEAR: u32 = 0x0031_4668;
pub(crate) const RIDER_ANIM_LIST_LOAD: u32 = 0x0031_4880;

/// `void RiderAnim_LoadState(RiderAnim* self, Stream* s)` (0x104de0): read
/// the first 0x50 bytes of the animation state (`s->Read(self, 0x50)`, slot
/// 0x10 of the stream's vtable), then clear and reload each of the six
/// sequence lists at `*(self+0x50) + 8*i`.
pub fn rider_anim_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x50;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = c.lw(r32(c, S3));
        let vt = c.g[V0][0];
        let read = VSlot::read(c, vt, 0x10);
        c.g[V1][0] = read.func;
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(read.delta as u32));
        call_ptr(c, read.func as u32, 0x0010_4e20)?;
        c.g[A0][0] = c.lw(oc344_at(c, S2, RIDER_ANIM_SEQ_LISTS));
        loop {
            c.cycles += 11;
            c.g[S0][0] = sx(r32(c, S1) << 3);
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
            call_ptr(c, RIDER_ANIM_LIST_CLEAR, 0x0010_4e38)?;
            c.g[A0][0] = c.lw(oc344_at(c, S2, RIDER_ANIM_SEQ_LISTS));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
            call_ptr(c, RIDER_ANIM_LIST_LOAD, 0x0010_4e48)?;
            let more = (c.g[S1][0] as i64) < 6;
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[A0][0] = c.lw(oc344_at(c, S2, RIDER_ANIM_SEQ_LISTS));
            if event_due(c) {
                deliver_event_here(c, 0x0010_4e28);
            }
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
// Rider_TryStartHandplant (0x107578; handplantcontrol)

/// Rider fields read by the handplant start test.
pub(crate) const HANDPLANT_POS: u32 = 0x110;
pub(crate) const HANDPLANT_UP: u32 = 0x180;
pub(crate) const HANDPLANT_FWD: u32 = 0x1b0;
pub(crate) const HANDPLANT_VEL: u32 = 0x1e0;
pub(crate) const HANDPLANT_HEIGHT: u32 = 0x1e8;
pub(crate) const HANDPLANT_AI: u32 = 0x77c;
pub(crate) const HANDPLANT_ANIM: u32 = 0x784;
pub(crate) const HANDPLANT_OVERLAPS: u32 = 0x860;

/// The rail search radius (300.0).
pub(crate) const HANDPLANT_RADIUS: u32 = 0x4396_0000;

const HANDPLANT_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::f_0011fe98,
};
const HANDPLANT_SET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::f_0011fec8,
};
const HANDPLANT_SET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe78,
    f: ee::s00::f_0011fe78,
};

/// `bool Rider_TryStartHandplant(Rider* r, int on)` (0x107578): record the
/// handplant cheat input (`RiderBase_RecordCheatCodeInput(ai+0xd20, 2)`).
/// In the air (main state 1) the rider must be high enough (`+0x1e8`),
/// upright (`up.z >= 0`) and not in a handplant animation (layer-2 entries
/// 0x12..0x14). Then look for the nearest rail 300 units ahead along the
/// velocity (`OverlapList_FindNearestRail`): the rail must be flat enough,
/// within 300 of the probe point and ahead of the rider. On success start
/// the handplant control (`ai+0x110`, speed factor `clamp(0.5 / max(speed,
/// k), 0.5, 1.5) * 300`, the rail direction flipped to face the velocity),
/// arm its timer (`ai+0x370`), set sub states 0xd then 0xb and main state 5,
/// and return 1; otherwise return 0.
pub fn rider_try_start_handplant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x110);
        c.g[A1][0] = 0x2;
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        c.g[A0][0] = c.lw(oc344_at(c, S0, HANDPLANT_AI));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xd20));
        call_ptr(c, 0x002f_6ac8, 0x0010_75a4)?;
        c.g[A0][0] = c.g[S0][0];
        HANDPLANT_GET_MAIN_STATE.call(c, 0x0010_75ac)?;
        c.g[V1][0] = 0x1;
        'done: {
            if c.g[V0][0] == c.g[V1][0] {
                // In the air: high enough, upright, not already planting.
                c.cycles += 6;
                c.lwc1(1, oc344_gp(c, 0x7e94));
                c.lwc1(0, oc344_at(c, S0, HANDPLANT_HEIGHT));
                c.c_lt_s(0, 1);
                c.g[V0][0] = 0;
                if c.fcc() {
                    break 'done;
                }
                c.cycles += 8;
                c.lq(V0, oc344_at(c, S0, HANDPLANT_UP));
                c.mtc1(0, 1);
                c.sq(V0, sp(c));
                c.lwc1(0, sp(c).wrapping_add(0x8));
                c.c_lt_s(0, 1);
                c.g[V0][0] = 0;
                if c.fcc() {
                    break 'done;
                }
                c.cycles += 7;
                c.g[A0][0] = c.lw(oc344_at(c, S0, HANDPLANT_ANIM));
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0031_1ae8, 0x0010_75fc)?;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x12));
                let planting = c.g[V0][0] < 0x3;
                c.g[V0][0] = 0;
                if planting {
                    break 'done;
                }
            }
            // 0x10760c: speed, and the probe direction (normalised velocity
            // when moving, else the forward vector).
            c.cycles += 16;
            c.lqc2(3, oc344_at(c, S0, HANDPLANT_VEL));
            oc344_vu0_length_vf3(c);
            c.cfc2(V0, 22);
            c.mtc1(V0, 21);
            c.lwc1(0, oc344_gp(c, 0x7e90));
            c.mov_s(2, 21);
            c.c_lt_s(0, 2);
            if !c.fcc() {
                c.lq(V0, oc344_at(c, S0, HANDPLANT_FWD));
            } else {
                c.cycles += 8;
                c.mfc1(V1, 2);
                c.qmtc2(V1, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, oc344_at(c, S0, HANDPLANT_VEL));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, sp(c).wrapping_add(0xa0));
                c.lq(V0, sp(c).wrapping_add(0xa0));
            }
            // 0x10766c: probe = pos + dir * 300; nearest rail to the probe.
            c.cycles += 26;
            let s = sp(c);
            c.sq(V0, s);
            c.sq(V0, s.wrapping_add(0x90));
            oc344_lit(c, 20, HANDPLANT_RADIUS);
            c.g[S1][0] = sx(s.wrapping_add(0x10));
            c.mfc1(V0, 20);
            c.lqc2(4, s);
            c.qmtc2(V0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s.wrapping_add(0xb0));
            c.g[A1][0] = sx(s.wrapping_add(0x90));
            c.lq(V0, s.wrapping_add(0xb0));
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 0x2;
            c.mov_s(12, 20);
            c.sq(V0, s.wrapping_add(0xa0));
            c.lqc2(3, oc344_at(c, S0, HANDPLANT_POS));
            c.lqc2(4, s.wrapping_add(0xa0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xb0));
            c.lq(V0, s.wrapping_add(0xb0));
            c.g[A0][0] = c.lw(oc344_at(c, S0, HANDPLANT_OVERLAPS));
            c.sq(V0, s.wrapping_add(0x90));
            call_ptr(c, 0x0033_4680, 0x0010_76cc)?;
            let s = sp(c);
            let found = c.g[V0][0] != 0;
            c.lwc1(0, s.wrapping_add(0x28));
            let mut ok = false;
            if found {
                // The rail must be flat enough (|dir.z| <= k).
                c.cycles += 6;
                c.lwc1(1, oc344_gp(c, 0x7e8c));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                c.g[V0][0] = 0;
                if c.fcc() {
                    break 'done;
                }
                // Within 300 of the probe.
                c.cycles += 20;
                c.lqc2(3, s.wrapping_add(0x10));
                c.lqc2(4, s.wrapping_add(0x90));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xb0));
                c.lq(V0, s.wrapping_add(0xb0));
                c.sq(V0, s.wrapping_add(0xa0));
                c.lqc2(3, s.wrapping_add(0xa0));
                oc344_vu0_length_vf3(c);
                c.cfc2(V1, 22);
                c.mtc1(V1, 0);
                c.c_lt_s(20, 0);
                c.g[V0][0] = 0;
                if c.fcc() {
                    break 'done;
                }
                // Ahead of the rider: (rail - pos) . vel >= 0.
                c.cycles += 20;
                c.lqc2(3, s.wrapping_add(0x10));
                c.lqc2(4, oc344_at(c, S0, HANDPLANT_POS));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xc0));
                c.mtc1(0, 3);
                c.lq(V0, s.wrapping_add(0xc0));
                c.sq(V0, s.wrapping_add(0xa0));
                c.sq(V0, s.wrapping_add(0xb0));
                c.lqc2(3, oc344_at(c, S0, HANDPLANT_VEL));
                c.lqc2(5, s.wrapping_add(0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(V1, 4);
                c.mtc1(V1, 0);
                c.c_lt_s(0, 3);
                ok = !c.fcc();
            }
            if !ok {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'done;
            }
            // 0x107794: speed factor clamp(0.5 / max(speed, k), 0.5, 1.5).
            c.cycles += 12;
            c.lwc1(0, oc344_gp(c, 0x7e88));
            c.lwc1(1, oc344_gp(c, 0x7e84));
            c.max_s(2, 21, 0);
            oc344_lit(c, 12, 0x3f00_0000);
            c.div_s(1, 1, 2);
            oc344_lit(c, 0, 0x3fc0_0000);
            c.c_le_s(12, 1);
            let above = c.fcc();
            c.mov_s(2, 12);
            if above {
                c.cycles += 1;
                c.min_s(2, 1, 0);
            }
            // 0x1077c8: the rail direction, flipped to face the velocity.
            c.cycles += 13;
            let s = sp(c);
            c.lqc2(3, s.wrapping_add(0x20));
            c.lqc2(5, oc344_at(c, S0, HANDPLANT_VEL));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V0, 4);
            c.mtc1(V0, 0);
            c.c_lt_s(0, 3);
            let behind = c.fcc();
            c.mul_s(20, 2, 12);
            if behind {
                c.cycles += 7;
                oc344_lit(c, 0, 0xbf80_0000);
                c.mfc1(V0, 0);
                c.lqc2(4, s.wrapping_add(0x20));
                c.qmtc2(V0, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s.wrapping_add(0x20));
            }
            // 0x107818: start the handplant.
            c.cycles += 21;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xd;
            HANDPLANT_SET_SUB_STATE.call(c, 0x0010_7824)?;
            c.g[A0][0] = c.lw(oc344_at(c, S0, HANDPLANT_AI));
            c.mov_s(12, 20);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x110));
            call_ptr(c, 0x0013_8ba0, 0x0010_7838)?;
            c.g[A0][0] = c.lw(oc344_at(c, S0, HANDPLANT_AI));
            c.mov_s(12, 20);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.lw(oc344_at(c, A0, 0x1a4));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x370));
            call_ptr(c, 0x0013_29b0, 0x0010_7850)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xb;
            HANDPLANT_SET_SUB_STATE.call(c, 0x0010_785c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x5;
            HANDPLANT_SET_MAIN_STATE.call(c, 0x0010_7868)?;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider ctor / Init / LoadState (0x108c80, 0x108e88, 0x10a8e8;
// all_in_one_ai.cpp `cComputer`)

/// The fields of a `cComputer` that point at its virtual-base `cRider`
/// (`this+0xf50`), stored in this order.
pub(crate) const COMPUTER_CTOR_RIDER_LINKS: [u32; 31] = [
    0xd20, 0xc70, 0xb40, 0xb00, 0xaf0, 0xad0, 0x9c0, 0x610, 0x520, 0x470, 0x3b0, 0x3a0, 0x398,
    0x384, 0x364, 0x358, 0x340, 0x2b8, 0x2a4, 0x288, 0x224, 0x200, 0x1f0, 0x1e4, 0x1c0, 0x1b0,
    0x100, 0xa0, 0x70, 0x24, 0x18,
];

/// The rider pointer (`this+0x18`).
pub(crate) const COMPUTER_RIDER: u32 = 0x18;

const COMPUTER_RIDER_CTOR: Callee = Callee {
    addr: 0x0011_b698,
    f: ee::s00::f_0011b698,
};
const COMPUTER_RIDER_BASE_CTOR: Callee = Callee {
    addr: 0x0010_fcd8,
    f: ee::s00::f_0010fcd8,
};
const COMPUTER_INIT: Callee = Callee {
    addr: 0x0010_8e88,
    f: ee::s00::f_00108e88,
};

/// `cComputer* cComputer::cComputer(cComputer* this, bool inChrg, bool
/// isActor)` (0x108c80). As the most-derived object (`inChrg`) it points
/// every virtual-base link at its `cRider` (`this+0xf50`) and constructs it;
/// then the `cRiderHandler` base (`RiderBase_ctor(this, 0)`), and the rider's
/// three vtables (script object 0x458618, script rider 0x459b90, rider
/// 0x458660). As a base subobject it copies the script-object and rider
/// vtables to the stack and adjusts their `this` deltas by the subobject
/// offset (gcc 2.9x thunks). Then the AI vtable (`+0xde8` = 0x4585f0), the
/// actor flag (`rider+0x878`), `ComputerActorRider_Init(this)`; returns
/// `this`.
pub fn computer_actor_rider_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[S2][0] = c.g[A2][0];
        if c.g[S1][0] != 0 {
            c.cycles += 34;
            let me = r32(c, S0);
            c.g[V0][0] = sx(me.wrapping_add(0xf50));
            c.g[A0][0] = c.g[V0][0];
            for off in COMPUTER_CTOR_RIDER_LINKS {
                c.sw(me.wrapping_add(off), c.g[V0][0]);
            }
            COMPUTER_RIDER_CTOR.call(c, 0x0010_8d2c)?;
        }
        c.cycles += 19;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        COMPUTER_RIDER_BASE_CTOR.call(c, 0x0010_8d38)?;
        let me = r32(c, S0);
        c.g[V1][0] = c.lw(me.wrapping_add(COMPUTER_RIDER));
        c.g[V0][0] = 0x46_0000;
        c.g[A2][0] = sx(0x46_0000u32.wrapping_sub(0x79e8));
        c.g[A0][0] = 0x46_0000;
        c.sw(oc344_at(c, V1, 0x6e8), c.g[A2][0]);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_sub(0x6470));
        c.g[V1][0] = 0x46_0000;
        c.g[A3][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(me.wrapping_add(COMPUTER_RIDER));
        c.g[T3][0] = c.g[V1][0];
        c.g[V1][0] = sx(0x46_0000u32.wrapping_sub(0x79a0));
        c.g[T2][0] = 0x46_0000;
        c.sw(oc344_at(c, A1, 0x6d0), c.g[A0][0]);
        c.g[V0][0] = c.lw(me.wrapping_add(COMPUTER_RIDER));
        c.sw(oc344_at(c, V0, 0x6c0), c.g[V1][0]);
        if c.g[S1][0] == 0 {
            // A base subobject: the script-object vtable (0x48 bytes) to sp.
            c.cycles += 4;
            c.g[A1][0] = c.g[A2][0];
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x40));
            c.g[T1][0] = sx(sp(c).wrapping_add(0x50));
            loop {
                c.cycles += 11;
                let src = r32(c, A1);
                c.g[A0][0] = c.ld(src);
                c.g[A2][0] = c.ld(src.wrapping_add(0x8));
                c.g[T0][0] = c.ld(src.wrapping_add(0x10));
                c.g[T4][0] = c.ld(src.wrapping_add(0x18));
                let dst = r32(c, V1);
                c.sd(dst, c.g[A0][0]);
                c.sd(dst.wrapping_add(0x8), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x10), c.g[T0][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T4][0]);
                c.g[A1][0] = sx(src.wrapping_add(0x20));
                let more = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx(dst.wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_8d88);
                }
            }
            c.cycles += 14;
            c.g[V0][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            c.g[A0][0] = sx(r32(c, A3).wrapping_sub(0x79e8));
            c.g[T5][0] = c.ld(r32(c, A1));
            c.sd(r32(c, V1), c.g[T5][0]);
            c.sw(oc344_at(c, V0, 0x6e8), c.g[SP][0]);
            c.g[T0][0] = sx(r32(c, T3).wrapping_sub(0x79a0));
            c.g[V1][0] = c.lhu(oc344_at(c, A0, 0x8));
            c.g[A1][0] = sx(r32(c, T0).wrapping_add(0xa0));
            c.g[V0][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            c.g[A2][0] = c.g[T1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xf50));
            c.g[A3][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
            c.sh(sp(c).wrapping_add(0x8), c.g[V1][0]);
            // The rider vtable (0xb0 bytes) to sp+0x50.
            loop {
                c.cycles += 11;
                let src = r32(c, T0);
                c.g[T4][0] = c.ld(src);
                c.g[T5][0] = c.ld(src.wrapping_add(0x8));
                c.g[V0][0] = c.ld(src.wrapping_add(0x10));
                c.g[V1][0] = c.ld(src.wrapping_add(0x18));
                let dst = r32(c, A2);
                c.sd(dst, c.g[T4][0]);
                c.sd(dst.wrapping_add(0x8), c.g[T5][0]);
                c.sd(dst.wrapping_add(0x10), c.g[V0][0]);
                c.sd(dst.wrapping_add(0x18), c.g[V1][0]);
                c.g[T0][0] = sx(src.wrapping_add(0x20));
                let more = c.g[T0][0] != c.g[A1][0];
                c.g[A2][0] = sx(dst.wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_8dec);
                }
            }
            c.cycles += 13;
            c.g[A0][0] = sx(r32(c, T3).wrapping_sub(0x79a0));
            c.g[A1][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            c.g[V1][0] = c.lhu(oc344_at(c, A0, 0xa0));
            c.g[V0][0] = c.lhu(oc344_at(c, A0, 0x8));
            let src = r32(c, T0);
            c.g[A0][0] = c.ld(src);
            c.g[T3][0] = c.ld(src.wrapping_add(0x8));
            let dst = r32(c, A2);
            c.sd(dst, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
            c.sd(dst.wrapping_add(0x8), c.g[T3][0]);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.sw(oc344_at(c, A1, 0x6c0), c.g[T1][0]);
            c.sh(sp(c).wrapping_add(0x58), c.g[V0][0]);
            c.sh(sp(c).wrapping_add(0xf0), c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, T2).wrapping_sub(0x7a10));
        c.sw(oc344_at(c, S0, 0xde8), c.g[V0][0]);
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[V1][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            c.g[V0][0] = 0x1;
            c.sw(oc344_at(c, V1, 0x878), c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        COMPUTER_INIT.call(c, 0x0010_8e6c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x120);
        frame.restore(c, S0, 0x130);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The AI state words `ComputerActorRider_Init` zeroes, in store order.
pub(crate) const COMPUTER_INIT_ZEROED: [u32; 15] = [
    0xe10, 0xe14, 0xe18, 0xe20, 0xe24, 0xe28, 0xe1c, 0xe2c, 0xe38, 0xe3c, 0xe30, 0xe34, 0xdf0,
    0xe74, 0xe78,
];

/// The think-handler table (`0x43cf20`, 8-byte entries); `+0xf38` selects
/// the entry copied to `+0xf44`.
pub(crate) const COMPUTER_THINK_TABLE: u32 = 0x0043_cf20;

const COMPUTER_INIT_DIFFICULTY: Callee = Callee {
    addr: 0x0010_cab8,
    f: ee::s00::f_0010cab8,
};
const COMPUTER_RESET_TRICK_COUNTS: Callee = Callee {
    addr: 0x0010_d170,
    f: ee::s00::f_0010d170,
};

/// `void ComputerActorRider_Init(cComputer* this)` (0x108e88): reset the AI
/// state (`+0xe0c` and `+0xe70` = -1, the counters at 0xdf0..0xe78 = 0, the
/// two targets at `+0xe50`/`+0xe60` from the zero vector at 0x4ff130), set
/// the difficulty (`InitDifficulty_2`), the skill scale `+0xdf4` = 1.0, the
/// think handler (entry 1 of the table at 0x43cf20 into `+0xf44`, index
/// `+0xf38` = 1), clear the three 0x3c-byte trick plans (`+0xe7c`, `+0xeb8`,
/// `+0xef4`) and the trick counts.
pub fn computer_actor_rider_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 64;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x50_0000;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0xed0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX;
        let me = r32(c, S0);
        c.sw(me.wrapping_add(0xe0c), c.g[S1][0]);
        for off in COMPUTER_INIT_ZEROED {
            c.sw(me.wrapping_add(off), 0);
        }
        c.sh(me.wrapping_add(0xf38), 0);
        c.sw(me.wrapping_add(0xf30), 0);
        c.sw(me.wrapping_add(0xf34), 0);
        let zero = r32(c, A1);
        c.lq(V0, zero);
        c.sq(V0, me.wrapping_add(0xe50));
        c.lq(V1, zero);
        c.sw(me.wrapping_add(0xe40), 0);
        c.sq(V1, me.wrapping_add(0xe60));
        COMPUTER_INIT_DIFFICULTY.call(c, 0x0010_8f0c)?;
        oc344_lit(c, 0, 0x3f80_0000);
        c.g[V0][0] = 0x44_0000;
        c.g[V0][0] = sx(COMPUTER_THINK_TABLE);
        let me = r32(c, S0);
        c.sw(me.wrapping_add(0xe70), c.g[S1][0]);
        c.swc1(0, me.wrapping_add(0xdf4));
        c.g[V1][0] = 0x1;
        c.ldl(A0, COMPUTER_THINK_TABLE + 0xf);
        c.ldr(A0, COMPUTER_THINK_TABLE + 0x8);
        c.sdl(A0, me.wrapping_add(0xf4b));
        c.sdr(A0, me.wrapping_add(0xf44));
        c.sh(me.wrapping_add(0xf38), c.g[V1][0]);
        for (off, ret_to) in [
            (0xe7c, 0x0010_8f4c),
            (0xeb8, 0x0010_8f5c),
            (0xef4, 0x0010_8f6c),
        ] {
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(off));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x3c;
            call_ptr(c, 0x003e_6448, ret_to)?;
        }
        c.g[A0][0] = c.g[S0][0];
        COMPUTER_RESET_TRICK_COUNTS.call(c, 0x0010_8f74)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const COMPUTER_RIDER_BASE_LOAD_STATE: Callee = Callee {
    addr: 0x0011_1d98,
    f: ee::s00::f_00111d98,
};

/// `void ComputerActorRider_LoadState(cComputer* this, Stream* s)`
/// (0x10a8e8): the `cRiderHandler` state (`RiderBase_LoadState`), then the
/// AI block (`s->Read(this+0xdf0, 0x150)`), then re-point the think handler
/// (`+0xf44`) at the table entry the loaded index (`+0xf38`) names.
pub fn computer_actor_rider_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        COMPUTER_RIDER_BASE_LOAD_STATE.call(c, 0x0010_a904)?;
        let stream = r32(c, S0);
        c.g[V1][0] = c.lw(stream);
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0xdf0));
        c.g[A2][0] = 0x150;
        let vt = c.g[V1][0];
        let read = VSlot::read(c, vt, 0x10);
        c.g[V0][0] = read.func;
        c.g[A0][0] = sx(stream.wrapping_add(read.delta as u32));
        call_ptr(c, read.func as u32, 0x0010_a920)?;
        let me = r32(c, S1);
        c.g[A0][0] = c.lh(me.wrapping_add(0xf38));
        c.g[V1][0] = 0x44_0000;
        c.g[V1][0] = sx(COMPUTER_THINK_TABLE);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, A0) << 3);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        let entry = r32(c, V0);
        c.ldl(V1, entry.wrapping_add(0x7));
        c.ldr(V1, entry);
        c.sdl(V1, me.wrapping_add(0xf4b));
        c.sdr(V1, me.wrapping_add(0xf44));
        c.sh(me.wrapping_add(0xf38), c.g[A0][0]);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider think handlers (0x10ad78, 0x10aed8, 0x10bc98)

const BOARD_PRESS_FIND_MARKER: Callee = Callee {
    addr: 0x0010_bbf8,
    f: ee::s00::f_0010bbf8,
};
const BOARD_PRESS_GET_STEER_TARGET: Callee = Callee {
    addr: 0x0010_0610,
    f: ee::s00::f_00100610,
};
const BOARD_PRESS_COMPUTE_STEER: Callee = Callee {
    addr: 0x0010_0348,
    f: ee::s00::f_00100348,
};

/// The rider's up vector (`rider+0x1a0`).
pub(crate) const COMPUTER_RIDER_UP: u32 = 0x1a0;

/// `void ComputerActorRider_ThinkBoardPress(cComputer* this, Pad* pad)`
/// (0x10ad78): the CPU rider's input during a board press. Looks for a
/// board-press marker (`FindBoardPressMarker`, which also says whether to
/// press nose or tail: lean -1 or +1), steers (`ComputeSteer` towards
/// `GetSteerTarget`, x31 into pad bits 18..23). With a marker: lean into pad
/// word 1 bits 0..5 (x31), and, when `clamp(2 * up.z, -1, 1)` is larger
/// than a dead zone, that value x31 into pad bits 24..29; without one the
/// lean bits are cleared.
pub fn computer_actor_rider_think_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_fpr(c, 20, 0x60);
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        BOARD_PRESS_FIND_MARKER.call(c, 0x0010_ada0)?;
        oc344_lit(c, 20, 0x3f80_0000);
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[SP][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            oc344_lit(c, 20, 0xbf80_0000);
        }
        // 0x10adc0: steer.
        c.cycles += 19;
        c.g[A1][0] = c.g[S1][0];
        BOARD_PRESS_GET_STEER_TARGET.call(c, 0x0010_adc8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        BOARD_PRESS_COMPUTE_STEER.call(c, 0x0010_add4)?;
        oc344_lit(c, 3, 0x41f8_0000);
        c.g[V1][0] = 0xffff_ffff_ff03_0000;
        let pad = r32(c, S0);
        c.g[A0][0] = c.lw(pad);
        c.g[V1][0] |= 0xffff;
        c.mul_s(0, 0, 3);
        c.g[A1][0] = c.g[A0][0] & c.g[V1][0];
        c.cvt_w_s(1, 0);
        c.mfc1(V0, 1);
        c.g[V0][0] &= 0x3f;
        c.g[V0][0] = sx(r32(c, V0) << 18);
        c.g[A1][0] |= c.g[V0][0];
        c.sw(pad, c.g[A1][0]);
        if c.g[S2][0] == 0 {
            // No marker: clear the lean.
            c.cycles += 4;
            c.g[V0][0] = c.lw(pad.wrapping_add(0x4));
            c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
            c.g[V0][0] &= c.g[V1][0];
            c.sw(pad.wrapping_add(0x4), c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 22;
            c.g[A0][0] = c.lw(pad.wrapping_add(0x4));
            c.mul_s(0, 20, 3);
            c.g[V0][0] = 0xffff_ffff_ffff_ffc0;
            oc344_lit(c, 1, 0xbf80_0000);
            c.g[A0][0] &= c.g[V0][0];
            oc344_lit(c, 4, 0x3f80_0000);
            c.cvt_w_s(2, 0);
            c.mfc1(V0, 2);
            c.g[V0][0] &= 0x3f;
            c.g[A0][0] |= c.g[V0][0];
            c.sw(pad.wrapping_add(0x4), c.g[A0][0]);
            c.g[V1][0] = c.lw(oc344_at(c, S1, COMPUTER_RIDER));
            c.lq(V0, oc344_at(c, V1, COMPUTER_RIDER_UP));
            c.sq(V0, sp(c));
            c.lwc1(0, sp(c).wrapping_add(0x8));
            c.add_s(0, 0, 0);
            c.c_le_s(1, 0);
            let above = c.fcc();
            c.mov_s(2, 1);
            if above {
                c.cycles += 1;
                c.min_s(2, 0, 4);
            }
            c.cycles += 6;
            c.lwc1(1, oc344_gp(c, 0x7e2c));
            c.abs_s(0, 2);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 11;
                c.mul_s(0, 2, 3);
                c.g[V1][0] = 0xffff_ffff_c0ff_0000;
                c.g[V1][0] |= 0xffff;
                c.g[V1][0] &= c.g[A1][0];
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] &= 0x3f;
                c.g[V0][0] = sx(r32(c, V0) << 24);
                c.g[V1][0] |= c.g[V0][0];
                c.sw(pad, c.g[V1][0]);
                c.cycles += 1;
            }
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

const RAILSLIDE_FIND_JUMP_MARKER: Callee = Callee {
    addr: 0x0010_b980,
    f: ee::s00::f_0010b980,
};
const RAILSLIDE_PLAN_AIR_TRICKS: Callee = Callee {
    addr: 0x0010_bfa8,
    f: ee::s00::f_0010bfa8,
};
const RAILSLIDE_FIND_SPEED_MARKER: Callee = Callee {
    addr: 0x0010_bb18,
    f: ee::s00::f_0010bb18,
};

/// The rider's velocity (`rider+0x1e0`) and state (`rider+0x328`).
pub(crate) const COMPUTER_RIDER_VEL: u32 = 0x1e0;
pub(crate) const COMPUTER_RIDER_STATE: u32 = 0x328;

/// `void ComputerActorRider_ThinkRailslide(cComputer* this, Pad* pad)`
/// (0x10aed8): the CPU rider's input on a rail. Looks for a jump marker
/// (`FindJumpMarker`; its distance goes to `+0xe20`) and presses bits
/// 17..24; with a marker and a trick to plan, presses bits 13/14 too and
/// plans the air tricks (`PlanAirTricks`), done. Otherwise balances
/// (`clamp(2 * up.z, -1, 1)` x31 into bits 25..30 past a dead zone), leans
/// (+-31 into pad word 1 by the sign of `+0xe38`) unless the rider is in
/// state 3/4 or the layer-2 animation is 0xe, and, with a speed marker
/// (`FindSpeedMarker`) whose speed (`+0xdf0` - k) is above the rider's,
/// presses bit 16.
pub fn computer_actor_rider_think_railslide(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        c.g[A3][0] = sx(sp(c).wrapping_add(0x14));
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0xdf0));
        frame.save_fpr(c, 21, 0x68);
        c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
        frame.save_fpr(c, 20, 0x60);
        c.g[T0][0] = sx(sp(c).wrapping_add(0x18));
        c.g[T1][0] = sx(sp(c).wrapping_add(0x1c));
        c.g[T2][0] = sx(sp(c).wrapping_add(0x20));
        RAILSLIDE_FIND_JUMP_MARKER.call(c, 0x0010_af14)?;
        'body: {
            let s = sp(c);
            c.g[A3][0] = c.lw(s.wrapping_add(0x18));
            c.g[A0][0] = 0x1fe_0000;
            c.sw(oc344_at(c, S0, 0xe20), c.g[A3][0]);
            let pad = r32(c, S1);
            c.g[V1][0] = c.lw(pad);
            c.g[V1][0] |= c.g[A0][0];
            let found = c.g[V0][0] != 0;
            c.sw(pad, c.g[V1][0]);
            if found {
                c.cycles += 3;
                c.g[V0][0] = c.lw(s.wrapping_add(0x10));
                let no_trick = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] | 0x6000;
                if !no_trick {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(s.wrapping_add(0x1c));
                    c.g[A2][0] = c.lw(s.wrapping_add(0x20));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw(s.wrapping_add(0x14));
                    c.sw(pad, c.g[V0][0]);
                    RAILSLIDE_PLAN_AIR_TRICKS.call(c, 0x0010_af54)?;
                    frame.restore(c, S0, 0x50);
                    break 'body;
                }
            }
            // 0x10af5c: balance.
            c.cycles += 13;
            c.g[V1][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            oc344_lit(c, 20, 0xbf80_0000);
            c.lq(V0, oc344_at(c, V1, COMPUTER_RIDER_UP));
            oc344_lit(c, 21, 0x3f80_0000);
            c.sq(V0, sp(c));
            c.lwc1(0, sp(c).wrapping_add(0x8));
            c.add_s(0, 0, 0);
            c.c_le_s(20, 0);
            let above = c.fcc();
            c.mov_s(2, 20);
            if above {
                c.cycles += 1;
                c.min_s(2, 0, 21);
            }
            c.cycles += 6;
            c.lwc1(1, oc344_gp(c, 0x7e28));
            c.abs_s(0, 2);
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V1][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            } else {
                c.cycles += 14;
                oc344_lit(c, 0, 0x41f8_0000);
                c.g[V1][0] = 0xffff_ffff_81ff_0000;
                let pad = r32(c, S1);
                c.g[A0][0] = c.lw(pad);
                c.g[V1][0] |= 0xffff;
                c.mul_s(0, 2, 0);
                c.g[A0][0] &= c.g[V1][0];
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] &= 0x3f;
                c.g[V0][0] = sx(r32(c, V0) << 25);
                c.g[A0][0] |= c.g[V0][0];
                c.sw(pad, c.g[A0][0]);
                c.g[V1][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            }
            // 0x10afe4: lean, unless in state 3/4 or animation 0xe.
            c.cycles += 5;
            c.g[V0][0] = c.lw(oc344_at(c, V1, COMPUTER_RIDER_STATE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x3));
            let skip = c.g[V0][0] < 0x2;
            c.g[V0][0] = skip as u64;
            c.g[A0][0] = c.g[S0][0];
            if !skip {
                c.cycles += 6;
                c.g[A0][0] = c.lw(oc344_at(c, V1, 0x784));
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0031_1ae8, 0x0010_b004)?;
                c.g[V1][0] = 0xe;
                let grinding = c.g[V0][0] == c.g[V1][0];
                c.g[A0][0] = c.g[S0][0];
                if !grinding {
                    c.cycles += 6;
                    c.lwc1(1, oc344_at(c, S0, 0xe38));
                    c.mtc1(0, 0);
                    c.c_lt_s(1, 0);
                    let negative = c.fcc();
                    c.mov_s(2, 20);
                    if negative {
                        c.cycles += 1;
                        c.mov_s(2, 21);
                    }
                    c.cycles += 12;
                    oc344_lit(c, 0, 0x41f8_0000);
                    c.g[A0][0] = 0xffff_ffff_ffff_ffc0;
                    let pad = r32(c, S1);
                    c.g[V1][0] = c.lw(pad.wrapping_add(0x4));
                    c.mul_s(0, 2, 0);
                    c.g[V1][0] &= c.g[A0][0];
                    c.cvt_w_s(1, 0);
                    c.mfc1(V0, 1);
                    c.g[V0][0] &= 0x3f;
                    c.g[V1][0] |= c.g[V0][0];
                    c.sw(pad.wrapping_add(0x4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S0][0];
                }
            }
            // 0x10b05c: speed marker.
            c.cycles += 16;
            c.g[A1][0] = sx(r32(c, S0).wrapping_add(0xdf0));
            c.g[A2][0] = sx(sp(c).wrapping_add(0x24));
            RAILSLIDE_FIND_SPEED_MARKER.call(c, 0x0010_b068)?;
            c.g[V1][0] = c.lw(oc344_at(c, S0, COMPUTER_RIDER));
            c.lqc2(3, oc344_at(c, V1, COMPUTER_RIDER_VEL));
            oc344_vu0_length_vf3(c);
            c.cfc2(A0, 22);
            c.mtc1(A0, 2);
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x24));
            if found {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x50);
                    break 'body;
                }
                c.cycles += 7;
                c.lwc1(0, oc344_at(c, S0, 0xdf0));
                c.lwc1(1, oc344_gp(c, 0x7e24));
                c.sub_s(0, 0, 1);
                c.c_lt_s(2, 0);
                let slower = c.fcc();
                c.g[V1][0] = 0x1_0000;
                if slower {
                    c.cycles += 3;
                    let pad = r32(c, S1);
                    c.g[V0][0] = c.lw(pad);
                    c.g[V0][0] |= c.g[V1][0];
                    c.sw(pad, c.g[V0][0]);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The path event type `HasPathEvent0F` looks for.
pub(crate) const PATH_EVENT_0F: u64 = 0xf;

/// `bool ComputerActorRider_HasPathEvent0F(cComputer* this)` (0x10bc98):
/// query up to 12 track events around the rider's track position
/// (`track->Query(events, 12, dist, side)`, slot 8 of the track at
/// `rider+0xab8`, with `rider+0x4c0`/`+0x4c4`) and return whether one of
/// them (16-byte records on the stack) has type 0xf.
pub fn computer_actor_rider_has_path_event_0f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xd0);
        c.g[A2][0] = 0xc;
        frame.save_ra(c, 0xc0);
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(oc344_at(c, A0, COMPUTER_RIDER));
        c.g[A3][0] = c.lw(oc344_at(c, V0, 0xab8));
        c.lwc1(13, oc344_at(c, V0, 0x4c4));
        c.g[V1][0] = c.lw(oc344_at(c, A3, 0x34));
        c.lwc1(12, oc344_at(c, V0, 0x4c0));
        let vt = c.g[V1][0];
        let query = VSlot::read(c, vt, 0x8);
        c.g[V0][0] = query.func;
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(query.delta as u32));
        call_ptr(c, query.func as u32, 0x0010_bccc)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        let mut found = false;
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A2][0] = PATH_EVENT_0F;
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1));
                if c.g[V0][0] == c.g[A2][0] {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    found = true;
                    break;
                }
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.cycles += 3;
                let more = (c.g[A0][0] as i64) < (c.g[A1][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_bce0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0xc0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, vtable, FAKE};
    use crate::difftest::{
        elf_image, fill, put_vec, Rng, CYCLE_BUDGET, DATA, GAME_GP, SP as TEST_SP,
    };
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's test fakes (in each test machine's own RAM).
    const OC344_CTRL: u32 = DATA + 0x0f_0200;
    const OC344_MAGIC: u32 = 0x4f43_3434;
    /// Forced callee results: 8 records `{target, v0, f0, flags}` (flags:
    /// 1 = set v0 (sign-extended), 2 = set f0).
    const OC344_RESULTS: u32 = OC344_CTRL + 0x10;
    /// The record `OverlapList_FindNearestRail` writes to its `a2` (0x20 bytes).
    const OC344_RAIL_OUT: u32 = OC344_CTRL + 0x100;

    /// Restore the callee-saved registers a caller saved in its frame and
    /// return from it (a stand-in for a slot-less callee that would run
    /// away on test data; the generated callers return through their
    /// `c.pc` checks).
    fn oc344_pop_frame(c: &mut Cpu, size: u32, saved: &[(usize, u32)], ra_off: u32) {
        let s = sp(c);
        for &(reg, off) in saved {
            c.lq(reg, s.wrapping_add(off));
        }
        c.g[RA][0] = c.ld(s.wrapping_add(ra_off));
        c.g[SP][0] = sx(s.wrapping_add(size));
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }

    /// The logging fake, plus the results the fixture forced for `target`.
    fn oc344_fake_body(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        if target == 0x0014_0738 && ra == 0x0011_b6b8 {
            // Rider_InitData from Rider_ctor: return from Rider_ctor.
            oc344_pop_frame(c, 0x40, &[(S2, 0x10), (S0, 0x30), (S1, 0x20)], 0);
            return;
        }
        let out = r32(c, A2);
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for i in 0..8 {
            let e = OC344_RESULTS + 0x10 * i;
            let t = c.bus.read32(e);
            if t == 0 {
                break;
            }
            if t == target {
                let flags = c.bus.read32(e + 0xc);
                if flags & 1 != 0 {
                    c.g[V0][0] = sx(c.bus.read32(e + 4));
                }
                if flags & 2 != 0 {
                    c.f[0] = c.bus.read32(e + 8);
                }
            }
        }
        if target == 0x0033_4680 {
            for k in 0..8 {
                let w = c.bus.read32(OC344_RAIL_OUT + 4 * k);
                c.bus.write32(out.wrapping_add(4 * k), w);
            }
        }
    }

    /// The machine's `miss` hook: `RiderBase_LoadState`'s first stream read
    /// returns from `RiderBase_LoadState`; everything else is faked.
    fn oc344_miss(c: &mut Cpu, target: u32) {
        if c.g[RA][0] as u32 == 0x0011_1dcc {
            oc344_pop_frame(c, 0x50, &[(S0, 0x40), (S1, 0x30)], 0x20);
            return;
        }
        oc344_fake_body(c, target);
    }

    /// The event hook of the ctor test (`next_event` 0): the first back edge
    /// inside `RiderBase_ctor` returns from it (its sub-object ctors
    /// allocate).
    fn oc344_ctor_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        if (0x0010_fcd8..0x0011_12b4).contains(&c.pc) {
            oc344_pop_frame(
                c,
                0x1270,
                &[
                    (S0, 0x1260),
                    (S1, 0x1250),
                    (S2, 0x1240),
                    (S3, 0x1230),
                    (S4, 0x1220),
                    (S5, 0x1210),
                    (S6, 0x1200),
                    (S7, 0x11f0),
                    (30, 0x11e0),
                ],
                0x11d0,
            );
            c.next_event = CYCLE_BUDGET;
            return true;
        }
        false
    }

    macro_rules! oc344_gated {
        ($name:ident, $prev:ident, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC344_CTRL) != OC344_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                oc344_fake_body(c, target);
            }
        };
    }

    oc344_gated!(
        oc344_fake_289b18,
        OC344_PREV_289B18,
        ee::s07::f_00289b18__generated
    );
    oc344_gated!(
        oc344_fake_312bd0,
        OC344_PREV_312BD0,
        ee::s10::f_00312bd0__generated
    );
    oc344_gated!(
        oc344_fake_314668,
        OC344_PREV_314668,
        ee::s10::f_00314668__generated
    );
    oc344_gated!(
        oc344_fake_311ae8,
        OC344_PREV_311AE8,
        ee::s10::f_00311ae8__generated
    );
    oc344_gated!(
        oc344_fake_334680,
        OC344_PREV_334680,
        ee::s11::f_00334680__generated
    );
    oc344_gated!(
        oc344_fake_2f6ac8,
        OC344_PREV_2F6AC8,
        ee::s09::f_002f6ac8__generated
    );
    oc344_gated!(
        oc344_fake_140738,
        OC344_PREV_140738,
        ee::s01::f_00140738__generated
    );
    oc344_gated!(
        oc344_fake_10cab8,
        OC344_PREV_10CAB8,
        ee::s00::f_0010cab8__generated
    );
    oc344_gated!(
        oc344_fake_10d170,
        OC344_PREV_10D170,
        ee::s00::f_0010d170__generated
    );
    oc344_gated!(
        oc344_fake_11fe98,
        OC344_PREV_11FE98,
        ee::s00::f_0011fe98__generated
    );
    oc344_gated!(
        oc344_fake_11fec8,
        OC344_PREV_11FEC8,
        ee::s00::f_0011fec8__generated
    );
    oc344_gated!(
        oc344_fake_11fe78,
        OC344_PREV_11FE78,
        ee::s00::f_0011fe78__generated
    );
    oc344_gated!(
        oc344_fake_10bbf8,
        OC344_PREV_10BBF8,
        ee::s00::f_0010bbf8__generated
    );
    oc344_gated!(
        oc344_fake_100610,
        OC344_PREV_100610,
        ee::s00::f_00100610__generated
    );
    oc344_gated!(
        oc344_fake_100348,
        OC344_PREV_100348,
        ee::s00::f_00100348__generated
    );
    oc344_gated!(
        oc344_fake_10b980,
        OC344_PREV_10B980,
        ee::s00::f_0010b980__generated
    );
    oc344_gated!(
        oc344_fake_10bfa8,
        OC344_PREV_10BFA8,
        ee::s00::f_0010bfa8__generated
    );
    oc344_gated!(
        oc344_fake_10bb18,
        OC344_PREV_10BB18,
        ee::s00::f_0010bb18__generated
    );

    /// Put the gated fakes in their slots (once per process).
    #[allow(unused_unsafe)]
    fn oc344_install() {
        use std::ptr::addr_of_mut;
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00289B18),
                oc344_fake_289b18,
                &OC344_PREV_289B18,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00312BD0),
                oc344_fake_312bd0,
                &OC344_PREV_312BD0,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00314668),
                oc344_fake_314668,
                &OC344_PREV_314668,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
                oc344_fake_311ae8,
                &OC344_PREV_311AE8,
            );
            fakes::hook(
                addr_of_mut!(ee::s11::OVERRIDE_00334680),
                oc344_fake_334680,
                &OC344_PREV_334680,
            );
            fakes::hook(
                addr_of_mut!(ee::s09::OVERRIDE_002F6AC8),
                oc344_fake_2f6ac8,
                &OC344_PREV_2F6AC8,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00140738),
                oc344_fake_140738,
                &OC344_PREV_140738,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010CAB8),
                oc344_fake_10cab8,
                &OC344_PREV_10CAB8,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010D170),
                oc344_fake_10d170,
                &OC344_PREV_10D170,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FE98),
                oc344_fake_11fe98,
                &OC344_PREV_11FE98,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
                oc344_fake_11fec8,
                &OC344_PREV_11FEC8,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FE78),
                oc344_fake_11fe78,
                &OC344_PREV_11FE78,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010BBF8),
                oc344_fake_10bbf8,
                &OC344_PREV_10BBF8,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00100610),
                oc344_fake_100610,
                &OC344_PREV_100610,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00100348),
                oc344_fake_100348,
                &OC344_PREV_100348,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010B980),
                oc344_fake_10b980,
                &OC344_PREV_10B980,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010BFA8),
                oc344_fake_10bfa8,
                &OC344_PREV_10BFA8,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010BB18),
                oc344_fake_10bb18,
                &OC344_PREV_10BB18,
            );
        }
    }

    /// Scrambled machine, fakes on, this module's miss hook.
    fn oc344_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC344_CTRL, OC344_MAGIC);
        c.hooks.miss = oc344_miss;
        r
    }

    /// Force the result of calls to `target` (v0 sign-extended, and/or f0).
    fn oc344_force(c: &mut Cpu, target: u32, v0: Option<u32>, f0: Option<u32>) {
        for i in 0..8 {
            let e = OC344_RESULTS + 0x10 * i;
            if c.bus.read32(e) == 0 {
                c.bus.write32(e, target);
                c.bus.write32(e + 4, v0.unwrap_or(0));
                c.bus.write32(e + 8, f0.unwrap_or(0));
                c.bus
                    .write32(e + 0xc, v0.is_some() as u32 | (f0.is_some() as u32) << 1);
                return;
            }
        }
        panic!("too many forced results");
    }

    /// A random float vector with `w` 0.
    fn oc344_vec(r: &mut Rng, range: f32) -> [u32; 4] {
        [r.f32(range), r.f32(range), r.f32(range), 0]
    }

    /// `RiderAnim_SeqEndType0A`: animation 0x1b2 through the (faked)
    /// `ClearFlagsAndPlayAnim`, which unwinds on some seeds.
    #[test]
    fn rider_anim_seq_end_type0a_matches() {
        oc344_install();
        covered(
            ee::s00::f_00104c80__generated,
            rider_anim_seq_end_type0a,
            200,
            &|c, seed| {
                oc344_machine(c, seed, 0x0010_4c80, 1);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x3000);
                c.pc = 0x0010_4c80;
            },
        );
    }

    /// `RiderAnim_PlayEventSound`: with and without a sound owner.
    #[test]
    fn rider_anim_play_event_sound_matches() {
        oc344_install();
        covered(
            ee::s00::f_00104cc8__generated,
            rider_anim_play_event_sound,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_4cc8, 1);
                let anim = DATA + 0x1000;
                c.g[A0][0] = sx(anim);
                c.bus.write32(
                    anim + RIDER_ANIM_SOUND_OWNER,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(GAME_GP + 0x410, r.next_u32());
                c.pc = 0x0010_4cc8;
            },
        );
    }

    /// `RiderAnim_LoadState`: the stream read (faked through the vtable),
    /// then the six lists, cleared (faked) and reloaded by the real
    /// `AnimSeqList_LoadState` (count 0 from the stack, so no allocation).
    #[test]
    fn rider_anim_load_state_matches() {
        oc344_install();
        covered(
            ee::s00::f_00104de0__generated,
            rider_anim_load_state,
            120,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_4de0, 13);
                let anim = DATA + 0x1000;
                let stream = DATA + 0x2000;
                let lists = DATA + 0x3000;
                fill(c, &mut r, anim, 0x20);
                c.bus.write32(anim + RIDER_ANIM_SEQ_LISTS, lists);
                fill(c, &mut r, lists, 12);
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                // AnimSeqList_LoadState's count word (its sp+0).
                c.bus.write32(TEST_SP - 0x50 - 0x80, 0);
                c.g[A0][0] = sx(anim);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0010_4de0;
            },
        );
    }

    /// `Rider_TryStartHandplant`: every exit (not in the air or too low,
    /// upside down, already planting, no rail, steep rail, too far, behind)
    /// and the start, with the real handplant-control setup (0x138ba0) and
    /// timer (0x1329b0).
    #[test]
    fn rider_try_start_handplant_matches() {
        oc344_install();
        covered(
            ee::s00::f_00107578__generated,
            rider_try_start_handplant,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_7578, 6);
                elf_image(c);
                let rider = DATA + 0x1000;
                let ai = DATA + 0x4000;
                let anim = DATA + 0x8000;
                let overlaps = DATA + 0x8100;
                let model = DATA + 0x9000;
                let bones = DATA + 0xa000;
                for i in 0..0x240 {
                    let v = r.f32(200.0);
                    c.bus.write32(rider + 4 * i, v);
                }
                for i in 0..0x100 {
                    let v = r.f32(10.0);
                    c.bus.write32(ai + 4 * i, v);
                }
                for i in 0..0x40 {
                    let v = r.f32(200.0);
                    c.bus.write32(bones + 4 * i, v);
                }
                c.bus.write32(rider + HANDPLANT_AI, ai);
                c.bus.write32(rider + HANDPLANT_ANIM, anim);
                c.bus.write32(rider + HANDPLANT_OVERLAPS, overlaps);
                c.bus.write32(rider + 0x780, model);
                c.bus.write32(model + 0x2c, bones);
                c.bus.write32(rider + 0x8a8, r.next_u32() % 2);
                c.bus.write32(rider + 0x320, r.next_u32() % 8);
                // The handplant control's and the timer's rider.
                c.bus.write32(ai + 0x110 + 0xa0, rider);
                c.bus.write32(ai + 0x370 + 0x14, rider);
                c.bus.write32(ai + 0x1a4, r.next_u32() & 1);
                // Velocity: sometimes nearly still.
                let vel = if seed % 7 == 0 {
                    oc344_vec(&mut r, 0.01)
                } else {
                    oc344_vec(&mut r, 60.0)
                };
                put_vec(c, rider + HANDPLANT_VEL, vel);
                put_vec(c, rider + HANDPLANT_POS, oc344_vec(&mut r, 200.0));
                // The rail record: near the probe, a direction with a small z.
                let pos = rider + HANDPLANT_POS;
                for k in 0..3 {
                    let p = f32::from_bits(c.bus.read32(pos + 4 * k));
                    let off = f32::from_bits(r.f32(500.0));
                    c.bus.write32(OC344_RAIL_OUT + 4 * k, (p + off).to_bits());
                }
                c.bus.write32(OC344_RAIL_OUT + 0xc, 0);
                put_vec(
                    c,
                    OC344_RAIL_OUT + 0x10,
                    [r.f32(1.0), r.f32(1.0), r.f32(0.6), 0],
                );
                // The rail's (optional) owner the control reads (sp+0x68).
                let owner = if seed % 2 == 0 { 0 } else { DATA + 0xb000 };
                c.bus.write32(TEST_SP - 0x110 + 0x68, owner);
                c.bus.write32(DATA + 0xb01c, r.next_u32());
                let main_state = if seed % 3 == 0 { 2 } else { 1 };
                oc344_force(c, 0x0011_fe98, Some(main_state), None);
                oc344_force(c, 0x0031_1ae8, Some(0x12 + r.next_u32() % 6), None);
                let rail = (seed % 5 != 0) as u32;
                oc344_force(c, 0x0033_4680, Some(rail), None);
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = r.next_u32() as u64 & 1;
                c.pc = 0x0010_7578;
            },
        );
    }

    /// `ComputerActorRider_ctor`: most-derived (with the `cRider` links and
    /// `Rider_ctor`, returned from at its first call) and base-subobject
    /// (the vtable copies with adjusted deltas), actor or not.
    /// `RiderBase_ctor` returns at its first back edge (event hook);
    /// `ComputerActorRider_Init` runs for real with faked callees.
    #[test]
    fn computer_actor_rider_ctor_matches() {
        oc344_install();
        covered(
            ee::s00::f_00108c80__generated,
            computer_actor_rider_ctor,
            120,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_8c80, 2);
                let me = DATA + 0x1000;
                let rider = DATA + 0x4000;
                fill(c, &mut r, me, 0x400);
                // A base subobject's virtual-base links already point at the
                // rider (RiderBase_ctor stores through them).
                for off in COMPUTER_CTOR_RIDER_LINKS {
                    c.bus.write32(me + off, rider);
                }
                // The vtables the copies read, and Init's tables.
                fill(c, &mut r, 0x0045_85f0, 0x100);
                fill(c, &mut r, 0x0045_9b80, 0x8);
                fill(c, &mut r, 0x004f_f130, 0x4);
                fill(c, &mut r, 0x0043_cf20, 0x4);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { 1 };
                c.next_event = 0;
                c.hooks.event = oc344_ctor_event;
                c.pc = 0x0010_8c80;
            },
        );
    }

    /// `ComputerActorRider_Init`: the resets, faked difficulty and trick
    /// counts, the real memsets.
    #[test]
    fn computer_actor_rider_init_matches() {
        oc344_install();
        covered(
            ee::s00::f_00108e88__generated,
            computer_actor_rider_init,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_8e88, 2);
                let me = DATA + 0x1000;
                fill(c, &mut r, me + 0xdf0, 0x60);
                fill(c, &mut r, 0x004f_f130, 0x4);
                fill(c, &mut r, COMPUTER_THINK_TABLE, 0x4);
                c.g[A0][0] = sx(me);
                c.pc = 0x0010_8e88;
            },
        );
    }

    /// `ComputerActorRider_LoadState`: `RiderBase_LoadState` returns at its
    /// first stream read; the AI block read is faked; table indexes around
    /// the table (also negative).
    #[test]
    fn computer_actor_rider_load_state_matches() {
        oc344_install();
        covered(
            ee::s00::f_0010a8e8__generated,
            computer_actor_rider_load_state,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_a8e8, 1);
                let me = DATA + 0x1000;
                let stream = DATA + 0x3000;
                fill(c, &mut r, me + 0xf30, 0x8);
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                fill(c, &mut r, COMPUTER_THINK_TABLE - 0x20, 0x20);
                let index = (r.next_u32() % 9) as i32 - 2;
                c.bus.write16(me + 0xf38, index as u16);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0010_a8e8;
            },
        );
    }

    /// `ComputerActorRider_ThinkBoardPress`: marker or not, nose or tail,
    /// steer from the faked `ComputeSteer`, balance past the dead zone or
    /// not.
    #[test]
    fn computer_actor_rider_think_board_press_matches() {
        oc344_install();
        covered(
            ee::s00::f_0010ad78__generated,
            computer_actor_rider_think_board_press,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_ad78, 3);
                let me = DATA + 0x1000;
                let pad = DATA + 0x3000;
                let rider = DATA + 0x4000;
                c.bus.write32(me + COMPUTER_RIDER, rider);
                put_vec(
                    c,
                    rider + COMPUTER_RIDER_UP,
                    [r.f32(1.0), r.f32(1.0), r.edge_f32(), r.f32(1.0)],
                );
                fill(c, &mut r, pad, 2);
                c.bus.write32(GAME_GP - 0x7e2c, 0x3e80_0000);
                // FindBoardPressMarker's nose/tail word (sp+0x10).
                let side = if seed % 2 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(TEST_SP - 0x70 + 0x10, side);
                let marker = (seed % 3 != 0) as u32;
                oc344_force(c, 0x0010_bbf8, Some(marker), None);
                let steer = r.f32(1.5);
                oc344_force(c, 0x0010_0348, None, Some(steer));
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(pad);
                c.pc = 0x0010_ad78;
            },
        );
    }

    /// `ComputerActorRider_ThinkRailslide`: jump marker with or without a
    /// trick plan, balance, lean (state 3/4, animation 0xe, sign of
    /// `+0xe38`), speed marker with or without a speed, faster or slower.
    #[test]
    fn computer_actor_rider_think_railslide_matches() {
        oc344_install();
        covered(
            ee::s00::f_0010aed8__generated,
            computer_actor_rider_think_railslide,
            240,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_aed8, 4);
                let me = DATA + 0x1000;
                let pad = DATA + 0x3000;
                let rider = DATA + 0x4000;
                c.bus.write32(me + COMPUTER_RIDER, rider);
                c.bus.write32(me + 0xe38, r.edge_f32());
                c.bus.write32(me + 0xdf0, r.f32(80.0));
                put_vec(
                    c,
                    rider + COMPUTER_RIDER_UP,
                    [r.f32(1.0), r.f32(1.0), r.edge_f32(), r.f32(1.0)],
                );
                put_vec(c, rider + COMPUTER_RIDER_VEL, oc344_vec(&mut r, 60.0));
                c.bus
                    .write32(rider + COMPUTER_RIDER_STATE, r.next_u32() % 6);
                c.bus.write32(rider + 0x784, DATA + 0x8000);
                fill(c, &mut r, pad, 2);
                c.bus.write32(GAME_GP - 0x7e28, 0x3e80_0000);
                c.bus.write32(GAME_GP - 0x7e24, 0x40a0_0000);
                // FindJumpMarker's outputs (sp+0x10..0x20) and
                // FindSpeedMarker's (sp+0x24).
                let s = TEST_SP - 0x70;
                fill(c, &mut r, s + 0x10, 5);
                if seed % 3 == 0 {
                    c.bus.write32(s + 0x10, 0);
                }
                let speed = if seed % 4 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(s + 0x24, speed);
                oc344_force(c, 0x0010_b980, Some((seed % 5 < 2) as u32), None);
                oc344_force(c, 0x0031_1ae8, Some(0xd + r.next_u32() % 2), None);
                oc344_force(c, 0x0010_bb18, Some((seed % 7 != 0) as u32), None);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(pad);
                c.pc = 0x0010_aed8;
            },
        );
    }

    /// `ComputerActorRider_HasPathEvent0F`: no events (0 or negative), and
    /// up to 12 events with or without a type-0xf one.
    #[test]
    fn computer_actor_rider_has_path_event_0f_matches() {
        oc344_install();
        covered(
            ee::s00::f_0010bc98__generated,
            computer_actor_rider_has_path_event_0f,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_bc98, 1);
                let me = DATA + 0x1000;
                let rider = DATA + 0x2000;
                let track = DATA + 0x3000;
                c.bus.write32(me + COMPUTER_RIDER, rider);
                c.bus.write32(rider + 0xab8, track);
                c.bus.write32(rider + 0x4c0, r.f32(1000.0));
                c.bus.write32(rider + 0x4c4, r.f32(10.0));
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(track + 0x34, vt);
                let count = match seed % 6 {
                    0 => 0,
                    1 => u32::MAX,
                    _ => 1 + r.next_u32() % 12,
                };
                oc344_force(c, FAKE, Some(count), None);
                let events = TEST_SP - 0xd0;
                for i in 0..12 {
                    let ty = if r.next_u32() % 8 == 0 {
                        0xf
                    } else {
                        r.next_u32() % 0x20
                    };
                    c.bus.write32(events + 0x10 * i, ty);
                }
                c.g[A0][0] = sx(me);
                c.pc = 0x0010_bc98;
            },
        );
    }
}
