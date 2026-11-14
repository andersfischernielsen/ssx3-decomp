//! Staging module oc340 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc340`.

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
        0x0026_e7f8,
        replay_highlight_slot_detach,
        "Replay_HighlightSlotDetach",
    ),
    (
        0x0035_aad0,
        multi_spline_modifier_add_ref,
        "MultiSplineModifier_AddRef",
    ),
    (0x003c_cf40, snd_eaxa_reset, "SND_EaxaReset"),
    (
        0x0010_4b78,
        rider_anim_seq_end_type03,
        "RiderAnim_SeqEndType03",
    ),
    (
        0x0028_4ba0,
        ssx_audio_system_get_fade_mask,
        "SSXAudioSystem_GetFadeMask",
    ),
    (0x0010_01c8, crt_exit, "_exit"),
    (0x0010_01d8, crt_frame_dummy, "frame_dummy"),
    (
        0x0010_0250,
        computer_actor_rider_compute_steer_flat,
        "ComputerActorRider_ComputeSteerFlat",
    ),
    (
        0x0010_0b90,
        computer_actor_rider_cruise_chase_rival,
        "ComputerActorRider_CruiseChaseRival",
    ),
];

// ------------------------------------------------ Replay_HighlightSlotDetach

/// The replay highlight slot's attached object (`+0xb0`).
pub(crate) const REPLAY_HIGHLIGHT_SLOT_OBJECT: u32 = 0xb0;

/// `void Replay_HighlightSlotDetach(ReplayHighlightSlot* slot)` (0x26e7f8):
/// forget the slot's attached object (`slot->object = NULL`).
pub fn replay_highlight_slot_detach(c: &mut Cpu) {
    c.cycles += 2;
    let slot = Addr(abi::arg(c, 0));
    slot.off(REPLAY_HIGHLIGHT_SLOT_OBJECT).sw(c, 0);
    ret(c);
}

// ------------------------------------------------ MultiSplineModifier_AddRef

/// The modifier's reference count (`+0x34`).
pub(crate) const MULTI_SPLINE_MODIFIER_REFS: u32 = 0x34;

/// `int MultiSplineModifier_AddRef(MultiSplineModifier* m)` (0x35aad0):
/// one more reference; returns the new count.
pub fn multi_spline_modifier_add_ref(c: &mut Cpu) {
    c.cycles += 4;
    let refs = Addr(abi::arg(c, 0)).off(MULTI_SPLINE_MODIFIER_REFS);
    let n = abi::addu(refs.lw(c), 1);
    c.g[V0][0] = n;
    refs.sw(c, n);
    ret(c);
}

// ------------------------------------------------------------- SND_EaxaReset

/// `SndEaxa* SND_EaxaReset(SndEaxa* eaxa)` (0x3ccf40): clear the EA-XA
/// decoder's input pointer, sizes and both predictor histories; returns
/// `eaxa`.
pub fn snd_eaxa_reset(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.g[A0][0];
    let eaxa = Addr(abi::arg(c, 0));
    for off in [0xc, 0x8, 0x4, 0x98, 0x9c] {
        eaxa.off(off).sw(c, 0);
    }
    ret(c);
}

// ---------------------------------------------------- RiderAnim_SeqEndType03

/// `int RiderAnim_SeqEndType03(RiderAnim* a, cAnimSequencer* b, AnimNode* n,
/// ...)` (0x104b78): the sequence-end handler for state type 03: replace the
/// sequence with animation 5 (`AnimController_ReplaceSequence(a, b, n, 5)`).
pub fn rider_anim_seq_end_type03(c: &mut Cpu) {
    use crate::game::animation::rideranim::SEQ_END_REPLACE;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0x5;
        call_ptr(c, SEQ_END_REPLACE, 0x0010_4b88)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ SSXAudioSystem_GetFadeMask

/// `int SSXAudioSystem_GetFadeMask(int quiet)` (0x284ba0): the mask of sound
/// groups that fade (0x173), or none when `quiet` is set.
pub fn ssx_audio_system_get_fade_mask(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = if c.g[A0][0] != 0 { 0 } else { 0x173 };
    ret(c);
}

// --------------------------------------------------------------------- _exit

/// `Exit` (0x42c6e8): kernel exit (`InitTLB`, then the `_Exit` syscall).
pub(crate) const CRT_EXIT_KERNEL: u32 = 0x0042_c6e8;

/// `void _exit(int)` (0x1001c8): the crt0 exit stub, `Exit(0)` (a tail
/// call; the status argument is ignored).
pub fn crt_exit(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = 0;
    c.pc = CRT_EXIT_KERNEL;
    ssx3_ee_rt::funcs::call_indirect(c, CRT_EXIT_KERNEL);
}

// --------------------------------------------------------------- frame_dummy

/// `__register_frame_info_table` (0x414728, cross-shard).
pub(crate) const FRAME_DUMMY_REGISTER: u32 = 0x0041_4728;
/// The `.eh_frame` table (`0x440000 - 0x3200`) and its static object.
pub(crate) const FRAME_DUMMY_EH_FRAME: u32 = 0x0044_0000 - 0x3200;
pub(crate) const FRAME_DUMMY_OBJECT: u32 = 0x004a_0000 + 0x5d48;

/// `static void frame_dummy(void)` (0x1001d8): crtbegin's init step,
/// `__register_frame_info_table(__EH_FRAME_BEGIN__, &object)`, with the
/// frame-pointer frame gcc gives it (`fp` saved, `sp` restored from `fp`).
pub fn crt_frame_dummy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[SP][0];
        c.g[A0][0] = sx(FRAME_DUMMY_EH_FRAME);
        c.g[A1][0] = sx(FRAME_DUMMY_OBJECT);
        call_ptr(c, FRAME_DUMMY_REGISTER, 0x0010_0200)?;
        c.g[SP][0] = c.g[FP][0];
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------- ComputerActorRider_ComputeSteerFlat

/// `.lit4` scale of the steering when the flatness factor is not 1
/// (0.12667641, at `gp - 0x7ff0`).
pub(crate) const STEER_FLAT_LIT_SCALE: u32 = 0x7ff0;

/// `float ComputerActorRider_ComputeSteerFlat(ComputerActorRider* self,
/// const Vec4* v)` (0x100250): the steering for direction `v` on flat
/// ground. With the model's (`self+0x18`) forward axis `a` (`+0x1a0`) and
/// side axis `b` (`+0x1c0`): `u = normalize(v - a * dot(v, a))`, steer
/// `r = -dot(u, b)`; returned as is when the flatness factor
/// (`self+0xdf4`) is 1, else `r * 0.12667641 * factor`.
///
/// The VU0 macro sequence is the original's (scratch vectors on a 0x50
/// stack frame).
pub fn computer_actor_rider_compute_steer_flat(c: &mut Cpu) {
    use crate::game::ai::computer::{v_dot, v_normalize};
    use ssx3_ee_rt::vu0::{Src, X};
    c.cycles += 55;
    let this = r32(c, A0);
    c.g[A2][0] = Addr(this).off(0x18).lw(c);
    let frame = Frame::open(c, 0x50);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1); // 1.0
    let model = r32(c, A2);
    let s = sp(c);
    c.lq(V0, model.wrapping_add(0x1a0));
    c.sq(V0, s);
    c.lq(V1, model.wrapping_add(0x1c0));
    c.sq(V1, s.wrapping_add(0x10));
    let v = r32(c, A1);
    // v - a * dot(v, a)
    v_dot(c, v, s);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    c.mfc1(V1, 0);
    c.lqc2(4, s);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s.wrapping_add(0x30));
    c.lqc2(3, v);
    c.lqc2(4, s.wrapping_add(0x30));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s.wrapping_add(0x20));
    // normalized, then -dot(u, b)
    v_normalize(c, s.wrapping_add(0x20), s.wrapping_add(0x40));
    c.lwc1(2, this.wrapping_add(0xdf4)); // flatness factor
    c.lq(V0, s.wrapping_add(0x40));
    c.c_eq_s(2, 1);
    c.sq(V0, s.wrapping_add(0x30));
    v_dot(c, s.wrapping_add(0x30), s.wrapping_add(0x10));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    let unit = c.fcc();
    c.neg_s(1, 0);
    if unit {
        c.cycles += 1;
        c.mov_s(0, 1);
    } else {
        c.cycles += 4;
        let a = r32(c, GP).wrapping_sub(STEER_FLAT_LIT_SCALE);
        c.lwc1(0, a);
        c.mul_s(0, 1, 0);
        c.mul_s(0, 0, 2);
    }
    c.cycles += 2;
    frame.close(c);
}

// ----------------------------------------- ComputerActorRider_CruiseChaseRival

/// `.lit4` constants of `ComputerActorRider_CruiseChaseRival` (offsets below
/// `gp`): 1/2pi, 2pi and the angle thresholds 90, 50 and 15 degrees.
pub(crate) const CHASE_RIVAL_LIT_INV_2PI: u32 = 0x7fbc;
pub(crate) const CHASE_RIVAL_LIT_2PI: u32 = 0x7fb8;
pub(crate) const CHASE_RIVAL_LIT_BEHIND: u32 = 0x7fb4;
pub(crate) const CHASE_RIVAL_LIT_WIDE: u32 = 0x7fb0;
pub(crate) const CHASE_RIVAL_LIT_NARROW: u32 = 0x7fac;

/// `void ComputerActorRider_CruiseChaseRival(ComputerActorRider* self,
/// ControlInput* pad)` (0x100b90): the "chase the rival" cruise step. The
/// rival is AI rider slot `self+0xe70`; its record in the model
/// (`model + slot*0x24`: distance +8, heading +0xc) gives the heading
/// difference, wrapped to [-pi, pi]; own speed from the model velocity,
/// the rival's from its path interface (`+0x6c0`, vtable +0x10).
///
/// - behind (|angle| > 90 deg): steer at the path; when the rival is
///   farther than 200 and slower, set the pad's low 6 bits of `+4` to 0x1f;
/// - otherwise aim 300 units ahead of the rival (its position, vtable
///   +0x28, plus its normalized velocity * 300): within 15 deg crouch and
///   boost, within 50 deg or slower than the rival cruise-crouch, faster
///   set the 0x1f bits; then steer at that point;
/// - pack the steering (`* 31`, 6 bits at bit 20) into the pad flags, then
///   resolve the jump state: a jump marker (descriptor 0, rival slot -1),
///   no target rider (descriptor 1), or a target the rival is not busy
///   with (vtable +0x40; descriptor 2).
///
/// The original's blocks are laid out in address order and entered by
/// falling out of the previous one; each `'b_…` label is one of them.
pub fn computer_actor_rider_cruise_chase_rival(c: &mut Cpu) {
    use crate::game::ai::computer::{cruise, jump_desc_copy, v_normalize, vu0_length_sq};
    use ssx3_ee_rt::vu0::{Src, X};
    /// Rider fields.
    const R_MODEL: u32 = 0x18;
    const R_MARKER_DIST: u32 = 0xdf0;
    const R_TARGET: u32 = 0xe70;
    const R_JUMP_STATE: u32 = 0xf38;
    /// Rider-slot record in the model: `0x24` bytes, distance and heading.
    const REC_SIZE: u64 = 0x24;
    const REC_DIST: u32 = 0x8;
    const REC_HEADING: u32 = 0xc;
    /// Model fields: heading, velocity.
    const M_HEADING: u32 = 0x4cc;
    const M_VELOCITY: u32 = 0x1e0;
    /// The rival's path interface (`{delta, fn}` slots).
    const E_IFACE: u32 = 0x6c0;
    const IFACE_GET_VELOCITY: u32 = 0x10;
    const IFACE_GET_POSITION: u32 = 0x28;
    const IFACE_BUSY: u32 = 0x40;
    /// The jump descriptors (`0x440000 - 0x30e0`).
    const JUMP_DESCS: u32 = 0x0044_0000 - 0x30e0;
    let gp_lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0xf0);
        let a = gp_lit(c, CHASE_RIVAL_LIT_INV_2PI);
        c.lwc1(0, a); // 1 / 2pi
        frame.save(c, S0, 0xc0);
        frame.save(c, S3, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x80);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0xb0);
        c.g[S4][0] = REC_SIZE;
        frame.save(c, S2, 0xa0);
        frame.save(c, S5, 0x70);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 23, 0xe8);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        let this = r32(c, S0);
        c.g[V0][0] = Addr(gp_lit(c, 0x848)).lw(c);
        c.g[V1][0] = Addr(this).off(R_TARGET).lw(c);
        c.g[A2][0] = Addr(this).off(R_MODEL).lw(c);
        c.mult(0, V1, V1, S4);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 3); // 0.5
        c.lwc1(2, r32(c, A2).wrapping_add(M_HEADING));
        c.g[A0][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        c.g[V0][0] = Addr(this).off(R_TARGET).lw(c);
        c.g[V1][0] = abi::addu(c.g[A2][0], c.g[V1][0]);
        c.g[S5][0] = Addr(r32(c, A0)).off(0xc).lw(c); // the AI
        c.lwc1(1, r32(c, V1).wrapping_add(REC_HEADING));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = abi::addu(c.g[S5][0], c.g[V0][0]);
        c.sub_s(1, 1, 2);
        // The rival.
        c.g[S1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
        // floor(diff / 2pi + 0.5)
        c.mul_s(0, 1, 0);
        c.add_s(0, 0, 3);
        c.cvt_w_s(2, 0);
        c.cvt_s_w(2, 2);
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.sub_s(2, 2, 0);
        }

        'b_100e3c: {
            'b_100d2c: {
                c.cycles += 35;
                let a = gp_lit(c, CHASE_RIVAL_LIT_2PI);
                c.lwc1(0, a); // 2pi
                c.lqc2(3, r32(c, A2).wrapping_add(M_VELOCITY));
                vu0_length_sq(c);
                c.cfc2(V0, 22);
                c.mtc1(V0, 23); // own speed
                let rival = r32(c, S1);
                c.g[V1][0] = Addr(rival).off(E_IFACE).lw(c);
                c.g[S2][0] = sx(rival.wrapping_add(E_IFACE));
                c.mul_s(0, 2, 0);
                let a = gp_lit(c, CHASE_RIVAL_LIT_BEHIND);
                c.lwc1(20, a); // pi/2
                let vt = r32(c, V1);
                c.g[A0][0] = Addr(vt).off(IFACE_GET_VELOCITY).lh(c);
                c.g[V0][0] = Addr(vt).off(IFACE_GET_VELOCITY + 4).lw(c);
                c.sub_s(0, 1, 0); // the wrapped angle
                c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                c.abs_s(21, 0);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0010_0c90)?;
                c.c_lt_s(20, 21);
                c.lqc2(3, r32(c, V0));
                vu0_length_sq(c);
                c.cfc2(V1, 22);
                c.mtc1(V1, 22); // rival speed
                if !c.fcc() {
                    c.g[V1][0] = Addr(r32(c, S1)).off(E_IFACE).lw(c);
                    break 'b_100d2c;
                }
                // Behind: steer at the path.
                c.cycles += 17;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                cruise::GET_STEER_TARGET.call(c, 0x0010_0cd0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                cruise::COMPUTE_STEER.call(c, 0x0010_0cdc)?;
                c.g[V1][0] = Addr(r32(c, S0)).off(R_TARGET).lw(c);
                c.mov_s(2, 0);
                c.g[V0][0] = Addr(r32(c, S0)).off(R_MODEL).lw(c);
                c.mult(0, V1, V1, S4);
                c.g[AT][0] = 0x4348_0000; // 200.0
                c.mtc1(AT, 1);
                c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
                c.lwc1(0, r32(c, V0).wrapping_add(REC_DIST));
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    break 'b_100e3c;
                }
                c.cycles += 4;
                c.c_lt_s(22, 23);
                c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
                if !c.fcc() {
                    break 'b_100e3c;
                }
                c.cycles += 5;
                let pad = Addr(r32(c, S3)).off(4);
                c.g[V0][0] = (pad.lw(c) & c.g[V1][0]) | 0x1f;
                let w = c.g[V0][0];
                pad.sw(c, w);
                break 'b_100e3c;
            }
            // Ahead: aim 300 units ahead of the rival.
            'b_100e30: {
                'b_100e0c: {
                    'b_100df8: {
                        'b_100dec: {
                            c.cycles += 39;
                            let vt = r32(c, V1);
                            c.g[A0][0] = Addr(vt).off(IFACE_GET_VELOCITY).lh(c);
                            c.g[V0][0] = Addr(vt).off(IFACE_GET_VELOCITY + 4).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                            call_ptr(c, target, 0x0010_0d3c)?;
                            let s = sp(c);
                            let vel = r32(c, V0);
                            v_normalize(c, vel, s.wrapping_add(0x10));
                            c.lq(V0, s.wrapping_add(0x10));
                            c.sq(V0, s);
                            c.g[V1][0] = Addr(r32(c, S1)).off(E_IFACE).lw(c);
                            let vt = r32(c, V1);
                            c.g[A0][0] = Addr(vt).off(IFACE_GET_POSITION).lh(c);
                            c.g[V0][0] = Addr(vt).off(IFACE_GET_POSITION + 4).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                            call_ptr(c, target, 0x0010_0d80)?;
                            // position + direction * 300
                            let s = sp(c);
                            c.g[AT][0] = 0x4396_0000; // 300.0
                            c.mtc1(AT, 1);
                            c.mfc1(A0, 1);
                            c.lqc2(4, s);
                            c.qmtc2(A0, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, s.wrapping_add(0x30));
                            let a = gp_lit(c, CHASE_RIVAL_LIT_WIDE);
                            c.lwc1(0, a); // 50 degrees
                            c.lq(V1, s.wrapping_add(0x30));
                            c.c_lt_s(21, 0);
                            c.sq(V1, s.wrapping_add(0x20));
                            c.lqc2(3, r32(c, V0));
                            c.lqc2(4, s.wrapping_add(0x20));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, s.wrapping_add(0x30));
                            c.lq(V0, s.wrapping_add(0x30));
                            let wide = c.fcc();
                            c.sq(V0, s.wrapping_add(0x10));
                            if !wide {
                                break 'b_100dec;
                            }
                            c.cycles += 5;
                            let a = gp_lit(c, CHASE_RIVAL_LIT_NARROW);
                            c.lwc1(0, a); // 15 degrees
                            c.c_lt_s(21, 0);
                            c.g[A0][0] = c.g[S0][0];
                            if !c.fcc() {
                                break 'b_100df8;
                            }
                            c.cycles += 4;
                            c.g[A1][0] = c.g[S3][0];
                            cruise::CROUCH_AND_BOOST.call(c, 0x0010_0de4)?;
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_100e30;
                        }
                        // Outside 50 degrees: crouch when slower.
                        c.cycles += 3;
                        c.c_lt_s(23, 22);
                        if !c.fcc() {
                            break 'b_100e0c;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S3][0];
                    cruise::SET_CRUISE_CROUCH.call(c, 0x0010_0e04)?;
                    c.g[A0][0] = c.g[S0][0];
                    break 'b_100e30;
                }
                // Not slower: faster sets the 0x1f bits.
                c.cycles += 4;
                c.c_lt_s(22, 23);
                c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
                if c.fcc() {
                    c.cycles += 4;
                    let pad = Addr(r32(c, S3)).off(4);
                    c.g[V0][0] = (pad.lw(c) & c.g[V1][0]) | 0x1f;
                    let w = c.g[V0][0];
                    pad.sw(c, w);
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 3;
            c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
            cruise::COMPUTE_STEER.call(c, 0x0010_0e38)?;
            c.mov_s(2, 0);
        }

        // Pack the steering, then the jump state.
        'b_100f54: {
            c.cycles += 26;
            c.g[AT][0] = 0x41f8_0000; // 31.0
            c.mtc1(AT, 0);
            c.g[V0][0] = 0xffff_ffff_fc0f_0000;
            let pad = r32(c, S3);
            c.g[V1][0] = Addr(pad).lw(c);
            c.g[V0][0] |= 0xffff;
            c.mul_s(0, 2, 0);
            c.g[S2][0] = 1;
            c.g[V1][0] &= c.g[V0][0];
            c.g[S1][0] = sx(r32(c, S0).wrapping_add(R_MARKER_DIST));
            let s = sp(c);
            Addr(s).off(0x54).sw(c, 1);
            c.g[A0][0] = c.g[S0][0];
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[A2][0] = sx(s.wrapping_add(0x40));
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = sx(s.wrapping_add(0x44));
            c.g[T0][0] = sx(s.wrapping_add(0x48));
            c.g[T1][0] = sx(s.wrapping_add(0x4c));
            c.g[V0][0] &= 0x3f;
            c.g[T2][0] = sx(s.wrapping_add(0x50));
            c.g[V0][0] = sx(r32(c, V0) << 20);
            c.g[V1][0] |= c.g[V0][0];
            let flags = c.g[V1][0];
            Addr(pad).sw(c, flags);
            cruise::FIND_JUMP_MARKER.call(c, 0x0010_0e9c)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !none {
                c.cycles += 9;
                c.g[V1][0] = 0x44_0000;
                Addr(r32(c, S0)).off(R_TARGET).sw(c, u64::MAX);
                c.g[A2][0] = sx(JUMP_DESCS);
                jump_desc_copy(c, V0, JUMP_DESCS, 7, 0);
                Addr(r32(c, S0)).off(R_JUMP_STATE).sh(c, 0);
                break 'b_100f54;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(sp(c).wrapping_add(0x54));
            cruise::SELECT_TARGET_RIDER.call(c, 0x0010_0ed4)?;
            let selected = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(sp(c)).off(0x54).lw(c);
            if !selected {
                c.cycles += 8;
                c.g[V0][0] = sx(JUMP_DESCS);
                jump_desc_copy(c, A3, JUMP_DESCS, 0xf, 0x8);
                let state = c.g[S2][0];
                Addr(r32(c, S0)).off(R_JUMP_STATE).sh(c, state);
                break 'b_100f54;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                return finish(c, frame, false);
            }
            // Is the rival busy?
            c.cycles += 12;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x80).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = abi::addu(c.g[S5][0], c.g[V0][0]);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x28).lw(c);
            c.g[V1][0] = Addr(r32(c, A0)).off(E_IFACE).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(E_IFACE));
            let vt = r32(c, V1);
            c.g[A1][0] = Addr(vt).off(IFACE_BUSY).lh(c);
            c.g[V0][0] = Addr(vt).off(IFACE_BUSY + 4).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
            call_ptr(c, target, 0x0010_0f2c)?;
            if c.g[V0][0] != 0 {
                return finish(c, frame, false);
            }
            c.cycles += 8;
            c.g[V0][0] = 0x44_0000;
            c.g[V1][0] = 2;
            c.g[V0][0] = sx(JUMP_DESCS);
            jump_desc_copy(c, A2, JUMP_DESCS, 0x17, 0x10);
            let state = c.g[V1][0];
            Addr(r32(c, S0)).off(R_JUMP_STATE).sh(c, state);
        }
        finish(c, frame, true)
    });

    /// The epilogue (`fall` = entered by falling into it: one more cycle).
    fn finish(c: &mut Cpu, frame: Frame, fall: bool) -> crate::call::Flow {
        if fall {
            c.cycles += 1;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 23, 0xe8);
        frame.restore_fpr(c, 22, 0xe0);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    /// `Replay_HighlightSlotDetach` on a slot with a random object.
    #[test]
    fn replay_highlight_slot_detach_matches() {
        check(
            ee::s07::f_0026e7f8__generated,
            replay_highlight_slot_detach,
            64,
            &|c, seed| {
                let mut r = Rng(0x0026_e7f8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0026_e7f8;
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus
                    .write32(DATA + 0x100 + REPLAY_HIGHLIGHT_SLOT_OBJECT, r.next_u32());
            },
        );
    }

    /// `MultiSplineModifier_AddRef` with counts around 0, -1 and the wrap.
    #[test]
    fn multi_spline_modifier_add_ref_matches() {
        check(
            ee::s11::f_0035aad0__generated,
            multi_spline_modifier_add_ref,
            64,
            &|c, seed| {
                let mut r = Rng(0x0035_aad0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0035_aad0;
                c.g[A0][0] = sx(DATA + 0x200);
                let n = match seed % 4 {
                    0 => 0,
                    1 => u32::MAX,
                    2 => 0x7fff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x200 + MULTI_SPLINE_MODIFIER_REFS, n);
            },
        );
    }

    /// `SND_EaxaReset` on a decoder full of random words.
    #[test]
    fn snd_eaxa_reset_matches() {
        check(
            ee::s14::f_003ccf40__generated,
            snd_eaxa_reset,
            32,
            &|c, seed| {
                let mut r = Rng(0x003c_cf40 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_cf40;
                c.g[A0][0] = sx(DATA + 0x300);
                crate::difftest::fill(c, &mut r, DATA + 0x300, 0x30);
            },
        );
    }

    /// `RiderAnim_SeqEndType03` over the real `AnimController_ReplaceSequence`
    /// chain on its short paths (`RiderAnim_SeqEndType02`'s fixture).
    #[test]
    fn rider_anim_seq_end_type03_matches() {
        use crate::game::animation::rideranim::tests::replace_sequence_setup;
        check(
            ee::s00::f_00104b78__generated,
            rider_anim_seq_end_type03,
            200,
            &|c, seed| {
                replace_sequence_setup(c, seed, 0x0010_4b78);
                c.pc = 0x0010_4b78;
            },
        );
    }

    /// `SSXAudioSystem_GetFadeMask` with `quiet` zero, small and with only
    /// upper bits set (a 64-bit test).
    #[test]
    fn ssx_audio_system_get_fade_mask_matches() {
        check(
            ee::s07::f_00284ba0__generated,
            ssx_audio_system_get_fade_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x0028_4ba0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0028_4ba0;
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 1 << 40,
                    _ => c.g[A0][0],
                };
            },
        );
    }

    /// `_exit` through the real `Exit` / `InitTLB` chain with fake syscalls:
    /// `GetMemorySize` answers a hash (never 32 MB), so `_InitTLB` runs and
    /// `_Exit` returns to the caller; some seeds unwind in a syscall.
    #[test]
    fn crt_exit_matches() {
        check(ee::s00::f_001001c8__generated, crt_exit, 64, &|c, seed| {
            machine(c, seed, 0x0010_01c8, 3);
            c.pc = 0x0010_01c8;
        });
    }

    /// `frame_dummy` over the real `__register_frame_info_table` (it links
    /// the static object into the list at 0x52ade8).
    #[test]
    fn crt_frame_dummy_matches() {
        check(
            ee::s00::f_001001d8__generated,
            crt_frame_dummy,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_01d8, 0);
                c.pc = 0x0010_01d8;
                c.bus.write32(0x0052_ade8, r.next_u32());
            },
        );
    }

    /// `ComputerActorRider_ComputeSteerFlat`: random axes and directions
    /// (some zero), flatness factor 1 or not, the real `.lit4` scale.
    #[test]
    fn computer_actor_rider_compute_steer_flat_matches() {
        check(
            ee::s00::f_00100250__generated,
            computer_actor_rider_compute_steer_flat,
            300,
            &|c, seed| {
                let mut r = Rng(0x0010_0250 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0010_0250;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - STEER_FLAT_LIT_SCALE, 0x3e01_b776);
                let (me, model, v) = (DATA, DATA + 0x1000, DATA + 0x2000);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(v);
                c.bus.write32(me + 0x18, model);
                let vec = |r: &mut Rng| {
                    if seed % 11 == 0 {
                        [0, 0, 0, 0]
                    } else {
                        [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(2.0)]
                    }
                };
                let (a, b, d) = (vec(&mut r), vec(&mut r), vec(&mut r));
                put_vec(c, model + 0x1a0, a);
                put_vec(c, model + 0x1c0, b);
                put_vec(c, v, d);
                let f = match seed % 3 {
                    0 => 1.0f32.to_bits(),
                    1 => r.f32(2.0),
                    _ => r.edge_f32(),
                };
                c.bus.write32(me + 0xdf4, f);
            },
        );
    }

    /// `ComputerActorRider_CruiseChaseRival` on the cruise fixture of
    /// `ComputerActorRider_CruiseFollowPath` (same steering, crouch, boost,
    /// jump-marker and target-rider callees, all real): the rival is AI slot
    /// 1..3, whose path interface returns a random velocity and position
    /// (fakes, forced pointers) and is busy or not (fake / the real zero
    /// leaf); headings over the whole circle (behind, within 15 and 50
    /// degrees, outside), record distances below and above 200, the real
    /// `.lit4` constants.
    #[test]
    fn computer_actor_rider_cruise_chase_rival_matches() {
        use crate::game::ai::computer::tests::{
            cruise_setup, MODEL, PMF, RIDER_2, RIDER_ENTRY, WORLD,
        };
        check(
            ee::s00::f_00100b90__generated,
            computer_actor_rider_cruise_chase_rival,
            240,
            &|c, seed| {
                cruise_setup(c, seed);
                let mut r = Rng(0x0010_0b90 ^ (seed as u64) << 20);
                c.pc = 0x0010_0b90;
                // The `.lit4` constants: 1/2pi, 2pi, 90/50/15 degrees.
                for (off, v) in [
                    (CHASE_RIVAL_LIT_INV_2PI, 0x3e22_f983),
                    (CHASE_RIVAL_LIT_2PI, 0x40c9_0fdb),
                    (CHASE_RIVAL_LIT_BEHIND, 0x3fc9_0fdc),
                    (CHASE_RIVAL_LIT_WIDE, 0x3f5f_66f4),
                    (CHASE_RIVAL_LIT_NARROW, 0x3e86_0a93),
                ] {
                    c.bus.write32(GAME_GP - off, v);
                }
                // The rival: AI slot 1..3 (`ai + 0x28 + 4*slot`).
                let slot = 1 + seed % 3;
                let ai = WORLD + 0x200;
                c.bus.write32(RIDER_2 + 0xe70, slot);
                for i in 1..4u32 {
                    c.bus
                        .write32(ai + 0x28 + 4 * i, RIDER_ENTRY + 0x100 * (i - 1));
                }
                let entry = RIDER_ENTRY + 0x100 * (slot - 1);
                c.bus.write32(entry + 0x6c0, PMF);
                // Its path interface: velocity, position (fakes whose forced
                // results point at random vectors) and "busy".
                let (vel, pos) = (DATA + 0x9_0000, DATA + 0x9_0100);
                c.bus.write16(PMF + 0x10, 0);
                c.bus.write32(PMF + 0x14, FAKE + 0x30);
                c.bus.write16(PMF + 0x28, 0);
                c.bus.write32(PMF + 0x2c, FAKE + 0x50);
                if seed % 4 == 0 {
                    c.bus.write32(PMF + 0x44, 0x0014_06c0);
                }
                put_words(c, LOG_RESULTS, &[vel, vel, pos]);
                put_vec(c, vel, [r.f32(150.0), r.f32(150.0), r.f32(150.0), 0]);
                put_vec(c, pos, [r.f32(500.0), r.f32(500.0), r.f32(500.0), 0]);
                // Headings and the slot record (distance, heading).
                c.bus.write32(MODEL + 0x4cc, r.f32(3.2));
                let rec = MODEL + 0x24 * slot;
                c.bus.write32(
                    rec + 0x8,
                    match seed % 5 {
                        0 => 0,
                        1 => r.f32(400.0) & 0x7fff_ffff,
                        2 => 200.0f32.to_bits(),
                        3 => r.f32(150.0),
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(rec + 0xc, r.f32(6.4));
            },
        );
    }
}
