//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! This wave is front-end / audio / memcard bookkeeping: the script
//! sequencer field readers (`ScriptCam_*`, `ScriptAudio_ReadFields`), the
//! memcard UI callbacks, the popup/overlay handlers and a few audio system
//! helpers. Most of them are thin: read a field from the object they are
//! given, call one or two virtual methods, store the result.
//!
//! The replacements keep the guest registers the originals leave behind
//! (callers can read `v0`/`v1` scratch values), follow the call protocol of
//! [`crate::call`], and issue the same cycles at the same points.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_0228, camera_wobble_get_offset, "CameraWobble_GetOffset"),
    (0x0016_2318, chase_camera_algorithm_ctor, "ChaseCameraAlgorithm_ctor"),(0x001c_17f8, net_quick_play_popup_set_options_enabled, "cFEStateNetQuickPlayPopup_SetOptionsEnabled"),
    (0x0023_9d88, gm_handler9_reset, "GMHandler9_Reset"),
    (0x0027_c510, script_cam_read_subject_fields, "ScriptCam_ReadSubjectFields"),
    (0x0027_c6f8, script_cam_read_algorithmic_fields, "ScriptCam_ReadAlgorithmicFields"),
    (0x0027_c0f0, script_cam_read_manual_fields, "ScriptCam_ReadManualFields"),
    (0x0027_c300, script_cam_read_target_fields, "ScriptCam_ReadTargetFields"),
    (0x0027_cc28, script_audio_read_fields, "ScriptAudio_ReadFields"),
    (0x001f_b458, in_game_error_highlight_continue, "OVScreenInGameError_HighlightContinue"),
    (0x001f_b588, in_game_error_highlight_cancel, "OVScreenInGameError_HighlightCancel"),(0x0039_c558, ui_screen_dtor, "UIScreen_dtor"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    /// `InSeq_Channel* InSeq_ChannelGetTrack(InSeq_Channel*, int index)`.
    pub const IN_SEQ_CHANNEL_GET_TRACK: Callee =
        Callee { addr: 0x0027_42c8, f: ee::s07::f_002742c8 };

    /// `int CameraTrigger_ReadActionTypeN(CameraTrigger*, Action*)`.
    pub const TYPE0: Callee = Callee { addr: 0x0017_15a0, f: ee::s02::f_001715a0 };
    pub const TYPE1: Callee = Callee { addr: 0x0017_14e8, f: ee::s02::f_001714e8 };
    pub const TYPE2: Callee = Callee { addr: 0x0017_15f8, f: ee::s02::f_001715f8 };
    pub const TYPE3: Callee = Callee { addr: 0x0017_1680, f: ee::s02::f_00171680 };

    /// `UIScreen` helpers.
    pub const UI_LIST_DELETE_ALL: Callee = Callee { addr: 0x0039_7948, f: ee::s13::f_00397948 };
    pub const UI_LIST_NODE_DTOR_BODY: Callee =
        Callee { addr: 0x003a_4868, f: ee::s13::f_003a4868 };
    pub const UI_OBJECT_DTOR: Callee = Callee { addr: 0x0039_fc48, f: ee::s13::f_0039fc48 };

    /// `void RiderCameraAlgorithm_ctor(RiderCameraAlgorithm*)`.
    pub const RIDER_CAMERA_ALGORITHM_CTOR: Callee =
        Callee { addr: 0x0016_22b0, f: ee::s01::f_001622b0 };
}

// ---------------------------------------------------------------------------
// Small helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// A virtual call: `vt = *(obj + 4)`, `fn = *(vt + slot + 4)`,
/// `this = obj + *(short *)(vt + slot)`, leaving `fn` in `a1` (the register
/// the original issues it from) and returning to `ret`.
#[inline]
fn call_slot(c: &mut Cpu, obj: u64, slot: u32, ret: u32) -> Flow {
    let vt = Addr((obj as u32).wrapping_add(4)).lw(c);
    c.g[V1][0] = vt;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(slot)).lh(c);
    c.g[A1][0] = Addr((vt as u32).wrapping_add(slot + 4)).lw(c);
    let func = c.g[A1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

/// Read track `index` of the sequencer channel `chan`.
#[inline]
fn channel_track(c: &mut Cpu, chan: u64, index: u32, ret: u32) -> Flow {
    c.g[A0][0] = chan;
    c.g[A1][0] = index as u64;
    callee::IN_SEQ_CHANNEL_GET_TRACK.call(c, ret)
}

// ---------------------------------------------------------------------------
// Script sequencer field readers.

/// `void ScriptCam_ReadSubjectFields(ScriptCam* self, float* out)`
/// (0x27c510): ten subject values; `out[i] = *track_i->v04()`.
pub fn script_cam_read_subject_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c544;
        c.cycles += 100;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..9u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void ScriptCam_ReadAlgorithmicFields(ScriptCam* self, float* out)`
/// (0x27c6f8): the same ten subject values as
/// [`script_cam_read_subject_fields`] but for the algorithmic camera.
pub fn script_cam_read_algorithmic_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c72c;
        c.cycles += 100;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..9u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void ScriptCam_ReadManualFields(ScriptCam* self, float* out)`
/// (0x27c0f0): ten manual-camera values.
pub fn script_cam_read_manual_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c110;
        c.cycles += 110;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..10u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void ScriptCam_ReadTargetFields(ScriptCam* self, float* out)`
/// (0x27c300): ten target-camera values.
pub fn script_cam_read_target_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c320;
        c.cycles += 110;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..10u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void ScriptAudio_ReadFields(ScriptAudio* self, int* out)` (0x27cc28):
/// five tracks, two ids each: `out[2i] = *track_i->v04()`,
/// `out[2i+1] = track_i->v05()`.
pub fn script_audio_read_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_cc48;
        c.cycles += 105;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..5u32 {
            let out = c.g[S1][0] as u32;
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.g[V1][0] = c.lw(src);
            c.sw(out.wrapping_add(8 * i), c.g[V1][0]);
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x28, RET)?;
            let v = c.g[V0][0];
            c.sw(out.wrapping_add(8 * i + 4), v);
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_ParseHeader(ChunkResolve* self, char* buf, void* ctx)`
/// (0x327da8): copy a chunk header into the resolve block, lay the payload
/// pointers out after it, and return the header size.
pub fn c_ps2_chunk_resolve_parse_header(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0032_7df0;
        /// A bulk copy (`func_003E6574(dst, src, n)`).
        const COPY: u32 = 0x003e_6574;
        c.cycles += 8;
        let f = Frame::open(c, 0x50);
        f.save(c, S0, 0x40);
        f.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        let buf = c.g[S1][0];
        c.g[V0][0] = 0;
        if buf != 0 {
            c.cycles += 61;
            let obj = c.g[S0][0] as u32;
            let blk = obj.wrapping_add(0xc0);
            let ctx = c.g[A2][0];
            c.g[V0][0] = sx(blk);
            c.g[V1][0] = 1;
            c.sw(blk, ctx);
            c.g[A0][0] = sx(f.at(c, 0x10));
            c.sw(obj.wrapping_add(0xc4), 0);
            c.g[A1][0] = c.g[S1][0];
            c.sw(obj.wrapping_add(0x98), c.g[V0][0]);
            c.g[A2][0] = 4;
            c.g[RA][0] = sx(RET);
            c.sw(obj.wrapping_add(0xd0), c.g[V1][0]);
            c.pc = COPY;
            ssx3_ee_rt::funcs::call_indirect(c, COPY);
            if c.pc != RET {
                return Err(crate::call::Unwound);
            }
            // The two lengths go to the stack first ...
            c.g[A0][0] = sx(f.at(c, 0x10));
            c.g[A1][0] = sx(buf as u32);
            c.g[A2][0] = 4;
            call_ptr(c, COPY, 0x0032_7e00)?;
            c.g[A0][0] = sx(f.at(c, 0x14));
            c.g[A1][0] = sx((buf as u32).wrapping_add(4));
            c.g[A2][0] = 4;
            call_ptr(c, COPY, 0x0032_7e14)?;
            // ... then the fixed arrays into the resolve block.
            for (dst_off, src_off, n, ret) in [
                (8u32, 8u32, 4u64, 0x0032_7e14u32),
                (0xc, 0xc, 4, 0x0032_7e28),
                (0x14, 0x10, 0xc, 0x0032_7e3c),
                (0x2c, 0x1c, 0xc, 0x0032_7e50),
                (0x38, 0x28, 0x24, 0x0032_7e64),
                (0x5c, 0x4c, 0x24, 0x0032_7e78),
            ] {
                let b = Addr(obj.wrapping_add(0x98)).lw(c);
                c.g[A0][0] = sx((b as u32).wrapping_add(dst_off));
                c.g[A1][0] = sx((buf as u32).wrapping_add(src_off));
                c.g[A2][0] = n;
                call_ptr(c, COPY, ret)?;
            }
            c.g[V0][0] = Addr(obj.wrapping_add(0x98)).lw(c);
            c.g[A1][0] = sx((buf as u32).wrapping_add(0x70));
            c.g[A0][0] = 0xc;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
            c.g[V1][0] = Addr(obj.wrapping_add(0x98)).lw(c);
            let blkp = c.g[V1][0] as u32;
            c.g[V0][0] = sx((Addr(blkp.wrapping_add(0xc)).lw(c) as u32).wrapping_add(1));
            c.mult(0, A2, V0, A0); // a2 = (count + 1) * 12 (a0 = 12 at this point)
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sw(blkp.wrapping_add(0x24), c.g[A1][0]);
            c.g[A2][0] = Addr(obj.wrapping_add(0x98)).lw(c);
            let len1 = Addr(f.at(c, 0x14)).lw(c);
            let len0 = Addr(f.at(c, 0x10)).lw(c);
            c.g[A0][0] = len0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((len1 as u32)));
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            let t = c.g[V1][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((len0 as u32)));
            let blkp = c.g[A2][0] as u32;
            if t {
                c.cycles += 2;
                c.g[V0][0] = Addr(blkp.wrapping_add(0x24)).lw(c);
                c.sw(blkp.wrapping_add(0x28), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sw(blkp.wrapping_add(0x28), 0);
            }
            c.cycles += 13;
            let blkp = Addr(obj.wrapping_add(0x98)).lw(c) as u32;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[S1][0] as u32)));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.lwc1(0, blkp.wrapping_add(0x14));
            c.swc1(0, f.at(c, 0));
            c.lwc1(1, blkp.wrapping_add(0x18));
            c.swc1(1, f.at(c, 4));
            c.lwc1(0, blkp.wrapping_add(0x1c));
            c.swc1(2, f.at(c, 0xc));
            c.swc1(0, f.at(c, 8));
            c.lq(3, f.at(c, 0));
            c.sq(3, obj.wrapping_add(0x80));
        }
        c.cycles += 5;
        c.lq(16, f.at(c, 0x40));
        c.lq(17, f.at(c, 0x30));
        f.restore_ra(c, 0x20);
        f.close(c);
        Ok(())
    })
}

/// `ChaseCameraAlgorithm* ChaseCameraAlgorithm_ctor(ChaseCameraAlgorithm*)`
/// (0x162318): the rider camera base class plus the chase-specific fields
/// (distance, speed windows, damping constants and the two curves).
pub fn chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0016_2330;
        /// `void func_0031D5E8(vec*)` (a three-float vector constructor).
        const VEC_CTOR: u32 = 0x0031_d5e8;
        c.cycles += 79;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(RET);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_CAMERA_ALGORITHM_CTOR.call(c, RET)?;
        let obj = c.g[S0][0] as u32;
        c.g[S1][0] = sx(obj.wrapping_add(0x340));
        c.g[V1][0] = 0x0046_0000;
        c.g[A0][0] = sx(obj.wrapping_add(0x304));
        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x4440));
        c.g[RA][0] = sx(RET);
        c.sw(obj.wrapping_add(0x10), c.g[V1][0]);
        for off in [0x304u32, 0x310, 0x31c, 0x328, 0x334] {
            c.g[RA][0] = sx(RET);
            c.g[A0][0] = sx(obj.wrapping_add(off));
            call_ptr(c, VEC_CTOR, RET)?;
        }
        call_ptr(c, 0x0016_7d88, RET)?;
        c.swc1(0, obj.wrapping_add(0x380));
        call_ptr(c, 0x0016_7d98, RET)?;
        c.swc1(0, obj.wrapping_add(0x384));
        call_ptr(c, 0x0016_7da8, RET)?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(RET);
        c.swc1(0, obj.wrapping_add(0x388));
        call_ptr(c, 0x0016_7db8, RET)?;
        c.g[V0][0] = 0x0050_0000;
        c.g[A0][0] = 0x54;
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0xed0));
        c.g[A1][0] = 0x0050_0000;
        c.lq(3, c.g[V0][0] as u32);
        c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xee0));
        c.sw(obj.wrapping_add(0x54), 0);
        c.g[A2][0] = 0x1388;
        c.sw(obj.wrapping_add(0xc), c.g[A0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sq(3, obj.wrapping_add(0x40));
        c.sw(obj.wrapping_add(0x50), 0);
        c.sw(obj.wrapping_add(0x58), 0);
        c.lwc1(3, obj.wrapping_add(0x54));
        c.lq(3, c.g[A1][0] as u32);
        c.g[AT][0] = 0x4348_0000;
        c.mtc1(1, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 2);
        c.swc1(0, obj.wrapping_add(0x1d0));
        c.swc1(1, obj.wrapping_add(0x1dc));
        c.swc1(2, obj.wrapping_add(0x1e0));
        c.sq(3, obj.wrapping_add(0x90));
        c.sw(obj.wrapping_add(0x2c0), c.g[A2][0]);
        c.sw(obj.wrapping_add(0x1d4), 0);
        c.swc1(3, obj.wrapping_add(0x1d8));
        c.swc1(3, obj.wrapping_add(0x1e4));
        c.sw(obj.wrapping_add(0x2d0), 0);
        c.sw(obj.wrapping_add(0x2d8), 0);
        c.lwc1(0, gp(c, -0x6b08));
        c.lq(3, c.g[A1][0] as u32);
        c.lwc1(1, gp(c, -0x6b04));
        c.sq(3, obj.wrapping_add(0xc0));
        c.lwc1(2, gp(c, -0x6b00));
        c.lq(3, c.g[A1][0] as u32);
        f.restore(c, S1, 0x10);
        c.sq(3, obj.wrapping_add(0xd0));
        f.restore_ra(c, 0);
        c.lq(4, c.g[A1][0] as u32);
        c.swc1(0, obj);
        c.sq(4, obj.wrapping_add(0xe0));
        c.swc1(1, obj.wrapping_add(0x388));
        c.swc1(3, obj.wrapping_add(0x384));
        c.swc1(2, obj.wrapping_add(0x380));
        f.restore(c, S0, 0x20);
        f.close(c);
        Ok(())
    })
}

/// `float* CameraWobble_GetOffset(CameraWobble* self, float* out)` (0x160228):
/// the camera-wobble offset for the current time: a rotated frame from the
/// wobble's amplitude/phase fields, copied out; the identity offset when the
/// wobble is inactive.
pub fn camera_wobble_get_offset(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0016_028c;
        const SIN: u32 = 0x0031_c040;
        const COS: u32 = 0x0031_bf60;
        c.cycles += 18;
        let f = Frame::open(c, 0xe0);
        f.save(c, S0, 0xa0);
        f.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A1][0];
        f.save_fpr(c, 24, 0xd0);
        c.g[S1][0] = c.g[A0][0];
        f.save_fpr(c, 23, 0xc8);
        f.save_fpr(c, 22, 0xc0);
        c.mov_s(23, 12);
        f.save_ra(c, 0x80);
        f.save_fpr(c, 21, 0xb8);
        f.save_fpr(c, 20, 0xb0);
        c.mtc1(0, 24);
        let obj = c.g[S0][0] as u32;
        c.lwc1(0, obj.wrapping_add(0x18));
        c.c_eq_s(0, 24);
        let fcc = c.fcc();
        c.mov_s(22, 13);
        c.cycles += if !fcc { 80 } else { 5 };
        if fcc {
            // Inactive: copy the identity offset.
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0xee0));
            c.lq(3, c.g[V0][0] as u32);
            c.sq(3, c.g[S1][0] as u32);
        } else {
            let out = c.g[S1][0] as u32;
            c.mov_s(12, 22);
            call_ptr(c, SIN, RET)?;
            c.mov_s(21, 0);
            c.mov_s(12, 23);
            call_ptr(c, SIN, RET)?;
            c.mul_s(21, 21, 0);
            c.mov_s(12, 22);
            call_ptr(c, SIN, RET)?;
            c.mov_s(20, 0);
            c.mov_s(12, 23);
            call_ptr(c, COS, RET)?;
            c.mul_s(20, 20, 0);
            c.mov_s(12, 22);
            call_ptr(c, COS, RET)?;
            c.neg_s(0, 0);
            c.swc1(21, f.at(c, 0));
            c.swc1(20, f.at(c, 4));
            c.mov_s(12, 23);
            c.swc1(24, f.at(c, 0xc));
            c.g[RA][0] = sx(RET);
            c.swc1(0, f.at(c, 8));
            c.pc = COS;
            ssx3_ee_rt::funcs::call_indirect(c, COS);
            if c.pc != RET {
                return Err(crate::call::Unwound);
            }
            c.mov_s(20, 0);
            c.mov_s(12, 23);
            call_ptr(c, SIN, RET)?;
            c.neg_s(0, 0);
            c.swc1(20, f.at(c, 0x10));
            c.swc1(24, f.at(c, 0x1c));
            c.swc1(24, f.at(c, 0x18));
            c.swc1(0, f.at(c, 0x14));
            // Cross product of the two axes, the rotation, then the offset.
            c.lqc2(4, f.at(c, 0));
            c.lqc2(5, f.at(c, 0x10));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, ssx3_ee_rt::vu0::Src::Vec(6));
            c.sqc2(6, f.at(c, 0x30));
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 5);
            c.lwc1(2, obj.wrapping_add(0x28));
            c.lwc1(0, obj.wrapping_add(0x20));
            c.lwc1(3, obj);
            c.div_s(0, 0, 2);
            c.lwc1(1, obj.wrapping_add(0xc));
            c.lq(2, f.at(c, 0x30));
            c.sub_s(1, 1, 3);
            c.lwc1(2, obj.wrapping_add(0x10));
            c.lwc1(3, obj.wrapping_add(4));
            c.lwc1(4, obj.wrapping_add(0x18));
            c.sq(2, f.at(c, 0x20));
            c.sub_s(2, 2, 3);
            c.mul_s(1, 0, 1);
            c.mfc1(2, 1);
            c.lqc2(4, f.at(c, 0x10));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X));
            c.sqc2(5, f.at(c, 0x60));
            c.mul_s(4, 4, 5);
            c.lq(2, f.at(c, 0x60));
            c.mul_s(0, 0, 2);
            c.sq(2, f.at(c, 0x50));
            c.mfc1(3, 0);
            c.lqc2(4, f.at(c, 0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X));
            c.sqc2(5, f.at(c, 0x70));
            c.lq(2, f.at(c, 0x70));
            c.sq(2, f.at(c, 0x60));
            c.lqc2(3, f.at(c, 0x50));
            c.lqc2(4, f.at(c, 0x60));
            c.vu0().add(0b1111, 5, 3, ssx3_ee_rt::vu0::Src::Vec(4));
            c.sqc2(5, f.at(c, 0x70));
            c.lq(2, f.at(c, 0x70));
            c.sq(2, f.at(c, 0x40));
            c.mfc1(3, 4);
            c.lqc2(4, f.at(c, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X));
            c.sqc2(5, f.at(c, 0x70));
            c.lq(2, f.at(c, 0x70));
            c.sq(2, out);
            c.sq(2, f.at(c, 0x30));
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S1][0];
        f.restore(c, S0, 0xa0);
        f.restore(c, S1, 0x90);
        f.restore_ra(c, 0x80);
        f.restore_fpr(c, 24, 0xd0);
        f.restore_fpr(c, 23, 0xc8);
        f.restore_fpr(c, 22, 0xc0);
        f.restore_fpr(c, 21, 0xb8);
        f.restore_fpr(c, 20, 0xb0);
        f.close(c);
        Ok(())
    })
}

/// `int CameraTrigger_ReadAction(CameraTrigger* self, Action** out)`
/// (0x1712b0): read an action type word, allocate the matching action and
/// let it read its own fields; `*out` is the new action and the result is
/// the number of bytes consumed.
pub fn camera_trigger_read_action(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0017_12dc;
        const ALLOC: u32 = 0x0031_7d70;
        c.cycles += 16;
        let f = Frame::open(c, 0x70);
        f.save(c, S3, 0x30);
        f.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        f.save(c, S1, 0x50);
        f.save(c, S2, 0x40);
        f.save(c, S0, 0x60);
        f.save_ra(c, 0x10);
        c.g[RA][0] = sx(RET);
        c.g[A1][0] = sx(f.at(c, 0));
        call_ptr(c, 0x002d_2558, RET)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[S1][0] = Addr(f.at(c, 0)).lw(c);
        c.g[V0][0] = 1;
        let t1 = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
        if t1 {
            // Type 1: the long action (0x34 bytes).
            c.cycles += 41;
            c.g[A1][0] = 0x0046_0000;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x34;
            c.g[RA][0] = sx(RET);
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f38));
            call_ptr(c, ALLOC, 0x0017_1388)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 3);
            c.g[V1][0] = 0x0046_0000;
            c.lwc1(4, gp(c, -0x643c));
            c.g[S0][0] = c.g[V0][0];
            c.g[AT][0] = 0x43c8_0000;
            c.mtc1(1, 2);
            c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3e30));
            c.g[AT][0] = 0x41c8_0000;
            c.mtc1(1, 1);
            c.g[V0][0] = 0x0050_0000;
            c.g[AT][0] = 0x428c_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.g[S3][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
            c.g[A1][0] = c.g[S0][0];
            c.sw(c.g[S0][0] as u32, c.g[S1][0]);
            c.swc1(3, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x14));
            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x10));
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x1c));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
            c.sw((c.g[S0][0] as u32).wrapping_add(8), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(7));
            c.ldr(3, c.g[T0][0] as u32);
            c.g[A2][0] = Addr((c.g[T0][0] as u32).wrapping_add(8)).lw(c);
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x2f));
            c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x28));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.g[RA][0] = sx(RET);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
            callee::TYPE1.call(c, 0x0017_140c)?;
            c.sw(c.g[S4][0] as u32, c.g[S0][0]);
        } else {
            c.cycles += 2;
            let ge2 = c.g[V0][0] == 0;
            c.g[V0][0] = 2;
            if ge2 {
                c.cycles += 2;
                let is2 = c.g[S1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    // Type 2: 0x20 bytes.
                    c.cycles += 27;
                    c.g[A1][0] = 0x0046_0000;
                    c.g[A0][0] = 0x20;
                    c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f20));
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(RET);
                    c.g[A3][0] = 0;
                    call_ptr(c, ALLOC, 0x0017_142c)?;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.g[V1][0] = 0x0046_0000;
                    c.lwc1(2, gp(c, -0x6438));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[AT][0] = 0x4080_0000;
                    c.mtc1(1, 0);
                    c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3e48));
                    c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), 0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
                    c.g[RA][0] = sx(RET);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
                    callee::TYPE2.call(c, 0x0017_1478)?;
                    c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                } else {
                    c.cycles += 2;
                    let is3 = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if is3 {
                        // Type 3: the 8-byte action.
                        c.cycles += 15;
                        c.g[A1][0] = 0x0046_0000;
                        c.g[A0][0] = 8;
                        c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f08));
                        c.g[A2][0] = 0;
                        c.g[RA][0] = sx(RET);
                        c.g[A3][0] = 0;
                        call_ptr(c, ALLOC, 0x0017_1498)?;
                        c.g[V1][0] = 0x0046_0000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3d18));
                        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(RET);
                        c.g[A1][0] = c.g[S0][0];
                        callee::TYPE3.call(c, 0x0017_14b8)?;
                        c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S2][0];
                    }
                }
            } else {
                c.cycles += 2;
                let is0 = c.g[S1][0] == 0;
                c.g[V0][0] = c.g[S2][0];
                if is0 {
                    // Type 0: the 0x10-byte action.
                    c.cycles += 20;
                    c.g[A1][0] = 0x0046_0000;
                    c.g[A0][0] = 0x10;
                    c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f50));
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(RET);
                    c.g[A3][0] = 0;
                    call_ptr(c, ALLOC, 0x0017_1338)?;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[V1][0] = 0x0046_0000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3d18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.g[S0][0];
                    c.sw(c.g[S0][0] as u32, 0);
                    c.g[RA][0] = sx(RET);
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), 0);
                    callee::TYPE0.call(c, 0x0017_1368)?;
                    c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 7;
        f.restore(c, S1, 0x50);
        f.restore(c, S2, 0x40);
        f.restore(c, S3, 0x30);
        f.restore(c, S4, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

/// `void cFEStateNetQuickPlayPopup_SetOptionsEnabled(Popup* self, int value)`
/// (0x1c17f8): pass `value` to the setter (slot 0x40/0x44 of the object's
/// vtable) of the fifteen option widgets at +0x6f8..
pub fn net_quick_play_popup_set_options_enabled(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x001c_1828;
        /// `set(value)` through `[[self + off] + 8]` slot 0x40.
        fn set_option(c: &mut Cpu, self_: u32, off: u32, value: u64) -> Flow {
            c.g[A2][0] = Addr(self_.wrapping_add(off)).lw(c);
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
            let func = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = value;
            call_ptr(c, func, RET)
        }
        c.cycles += 115;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let obj = c.g[S0][0] as u32;
        let value = c.g[S1][0];
        for i in 0..15u32 {
            set_option(c, obj, 0x6f8 + 4 * i, value)?;
        }
        f.restore(c, S0, 0x20);
        f.restore(c, S1, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void UIScreen_dtor(UIScreen* self, uint flags)` (0x39c558): free the
/// screen's name/child blocks, tear down its lists and node bodies, then
/// release the object itself if `flags & 1`.
pub fn ui_screen_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let f = Frame::open(c, 0x80);
        c.g[V0][0] = 0x0049_0000;
        f.save(c, S5, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4670));
        f.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A0][0];
        f.save(c, S0, 0x70);
        f.save(c, S1, 0x60);
        f.save(c, S2, 0x50);
        f.save(c, S3, 0x40);
        f.save(c, S4, 0x30);
        f.save_ra(c, 0);
        c.sw((c.g[S5][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x3c)).lw(c);
        let t = c.g[A0][0] == 0;
        c.g[S6][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0039c5a0)?;
        }
        // The two name/child blocks, freed when the flag bit is clear.
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[S1][0] = 0x0049_0000;
        let empty = c.g[A0][0] == 0;
        if empty {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x14)).lw(c) & 2;
            let skip = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0049_0000;
            if !skip {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x0039c5c4)?;
                c.cycles += 1;
            }
        }
        c.g[V0][0] = 0x0049_0000;
        // Teardown of the embedded lists and nodes.
        c.cycles += 35;
        let obj = c.g[S5][0] as u32;
        c.g[S1][0] = sx(0x0049_0000u32.wrapping_add(0x4c98));
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x49e8));
        c.g[S2][0] = sx(obj.wrapping_add(0xb4));
        c.sw(obj.wrapping_add(0xcc), c.g[S1][0]);
        c.sw(obj.wrapping_add(0x48), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x0039c5e4)?;
        c.g[S4][0] = sx(obj.wrapping_add(0x40));
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(obj.wrapping_add(0xc0));
        c.g[S0][0] = sx(0x0049_0000u32.wrapping_add(0x4cc0));
        c.g[S3][0] = sx(obj.wrapping_add(0x18));
        c.sw(obj.wrapping_add(0xc8), c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c600)?;
        c.sw(obj.wrapping_add(0xbc), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c60c)?;
        c.g[A1][0] = 2;
        c.g[A0][0] = c.g[S4][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039c618)?;
        c.sw(obj.wrapping_add(0x30), c.g[S1][0]);
        c.g[A0][0] = c.g[S3][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x0039c624)?;
        c.sw(obj.wrapping_add(0x2c), c.g[S0][0]);
        c.g[A0][0] = sx(obj.wrapping_add(0x24));
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c630)?;
        c.sw(obj.wrapping_add(0x20), c.g[S0][0]);
        c.g[A0][0] = c.g[S3][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c63c)?;
        c.sw(obj.wrapping_add(8), c.g[S0][0]);
        c.g[A0][0] = c.g[S5][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c648)?;
        c.g[V0][0] = c.g[S6][0] & 1;
        let t = c.g[V0][0] == 0;
        f.restore(c, S0, 0x70);
        if !t {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0039c65c)?;
        }
        c.cycles += 9;
        f.restore(c, S1, 0x60);
        f.restore(c, S2, 0x50);
        f.restore(c, S3, 0x40);
        f.restore(c, S4, 0x30);
        f.restore(c, S5, 0x20);
        f.restore(c, S6, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The two in-game error highlight handlers (0x1fb458 / 0x1fb588): look up
/// the two popup buttons and the message node; with `on` set the highlighted
/// button gets four 1.0 floats, otherwise both buttons are cleared and the
/// node's setter is called.
fn in_game_error_highlight(c: &mut Cpu, names: [u32; 3], rets: [u32; 10]) {
    guest(c, |c| {
        c.cycles += 30;
        let f = Frame::open(c, 0x60);
        f.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, S2, 0x30);
        c.g[A0][0] = 0x0047_0000;
        f.save(c, S1, 0x40);
        f.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        f.save_ra(c, 0x10);
        // Three name -> widget lookups on the screen.
        let hash_ret = [rets[0], rets[2], rets[4]];
        let find_ret = [rets[1], rets[3], rets[5]];
        for (i, name) in names.iter().enumerate() {
            c.g[RA][0] = sx(hash_ret[i]);
            c.g[A0][0] = sx(*name);
            call_ptr(c, 0x0031_7670, hash_ret[i])?;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            c.g[RA][0] = sx(find_ret[i]);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, find_ret[i])?;
            match i {
                0 => c.g[S1][0] = c.g[V0][0],
                1 => c.g[S3][0] = c.g[V0][0],
                _ => c.g[S0][0] = c.g[V0][0],
            }
        }
        // The button: all four floats 1.0 when highlighted, only the first
        // (alpha) otherwise. The four words go through the stack.
        let on = c.g[S2][0] != 0;
        c.cycles += if on { 14 } else { 13 };
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(8)).lw(c);
        c.g[A1][0] = sx(f.at(c, 0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
        c.swc1(0, f.at(c, if on { 0xc } else { 0 }));
        if on {
            c.swc1(0, f.at(c, 0));
            c.swc1(0, f.at(c, 4));
            c.swc1(0, f.at(c, 8));
        } else {
            c.sw(f.at(c, 4), 0);
            c.sw(f.at(c, 8), 0);
            c.sw(f.at(c, 0xc), 0);
        }
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
        let func = c.g[V0][0] as u32;
        let ret = if on { rets[6] } else { rets[7] };
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, ret)?;
        c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(8)).lw(c);
        // Then the node's setter, and the node widget's own setter.
        c.cycles += 18;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(rets[8]);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, rets[8])?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(rets[9]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, rets[9])?;
        f.restore(c, S0, 0x50);
        f.restore(c, S1, 0x40);
        f.restore(c, S2, 0x30);
        f.restore(c, S3, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

/// `void OVScreenInGameError_HighlightContinue(OVScreen*, int on)` (0x1fb458).
pub fn in_game_error_highlight_continue(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fb48, 0x0046_fad8, 0x0046_fb00],
        [
            0x001f_b484, 0x001f_b490, 0x001f_b4a0, 0x001f_b4ac, 0x001f_b4bc, 0x001f_b4c8, 0x001f_b500,
            0x001f_b538, 0x001f_b550, 0x001f_b568,
        ],
    );
}

/// `void OVScreenInGameError_HighlightCancel(OVScreen*, int on)` (0x1fb588).
pub fn in_game_error_highlight_cancel(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fa90, 0x0046_fac8, 0x0046_fae8],
        [
            0x001f_b5b4, 0x001f_b5c0, 0x001f_b5d0, 0x001f_b5dc, 0x001f_b5ec, 0x001f_b5f8, 0x001f_b630,
            0x001f_b668, 0x001f_b680, 0x001f_b698,
        ],
    );
}

/// `void GMHandler9_Reset(GMHandler9* self)` (0x239d88): clear the race
/// manager's player/handler counters and re-seat the two player interfaces.
pub fn gm_handler9_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let f = Frame::open(c, 0x40);
        f.save(c, S2, 0x10);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(0x0023_9da4);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_9da4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023_9db0);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_9db0)?;
        c.g[RA][0] = sx(0x0023_9db8);
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_9db8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023_9dc4);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_9dc4)?;
        // The handler's state block.
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[T0][0] = u64::MAX;
        c.g[A2][0] = 3;
        c.g[A3][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[T0][0]);
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[T0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x88), c.g[A3][0]);
        c.g[A1][0] = 3;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x74), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x98), c.g[A3][0]);
        c.sw(c.g[V1][0] as u32, c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x70), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0);
        c.g[RA][0] = sx(0x0023_9e1c);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), 0);
        call_ptr(c, 0x0014_54f8, 0x0023_9e1c)?;
        c.g[V1][0] = 0x3c;
        c.g[A3][0] = Addr(gp(c, -0x480)).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[A2][0] = 0x28;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x18));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x7c), 0);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8c), 0);
        c.g[A1][0] = u64::MAX;
        c.g[RA][0] = sx(0x0023_9e44);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        call_ptr(c, 0x0041_6210, 0x0023_9e44)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_9e58);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0023_9e58)?;
        // The two player-ID slots (a1 = 0, then 1) and their names.
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0023_9e64);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_7488, 0x0023_9e64)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x0023_9e78);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, 0x0014_7488, 0x0023_9e78)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0023_9e8c);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x0014_74c8, 0x0023_9e8c)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x0023_9ea0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        call_ptr(c, 0x0014_74c8, 0x0023_9ea0)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        f.restore(c, S0, 0x30);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        c.sh((c.g[S2][0] as u32).wrapping_add(6), 0);
        c.sh((c.g[S2][0] as u32).wrapping_add(4), 0);
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{elf_image, scramble, Rng, DATA, GAME_GP};

    /// A leaf that returns quickly and only reads its first argument: the
    /// stand-in for the virtual methods the originals call.
    const LEAF: u32 = 0x002c_c098;

    const BE_DUMMY: u32 = DATA + 0x7_0000;
    const WORLD: u32 = DATA + 0x1_0000;

    /// Scrambled machine with the game image loaded, the BELibrary singleton
    /// and the world pointer installed (real callees otherwise take long or
    /// runaway paths).
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(salt ^ 0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x1f04, BE_DUMMY);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        r
    }

    /// Compare hard state, caller-saved state and LO/HI.
    fn check3(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(g, h, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    /// A sequencer channel whose ten tracks each have a vtable with the two
    /// slots the field readers use, plus a destination buffer.
    fn seq_channel(c: &mut Cpu, r: &mut Rng) -> (u32, u32) {
        let chan = DATA;
        let table = DATA + 0x100;
        c.bus.write32(chan + 0x24, table);
        for i in 0..10u32 {
            let track = DATA + 0x1000 + 0x40 * i;
            let vt = DATA + 0x2000 + 0x40 * i;
            c.bus.write32(table + 4 * i, track);
            c.bus.write32(track + 4, vt);
            c.bus.write32(track, r.next_u32());
            c.bus.write32(track + 0x54, r.next_u32());
            for slot in [0x20u32, 0x28] {
                c.bus.write16(vt + slot, 0);
                c.bus.write32(vt + slot + 4, LEAF);
            }
        }
        (chan, DATA + 0x400)
    }

    #[test]
    fn script_cam_read_subject_fields_matches() {
        check3(ee::s07::f_0027c510__generated, script_cam_read_subject_fields, 40, &|c, seed| {
            let mut r = base(c, seed, 0xc510);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }

    #[test]
    fn script_cam_read_algorithmic_fields_matches() {
        check3(ee::s07::f_0027c6f8__generated, script_cam_read_algorithmic_fields, 40, &|c, seed| {
            let mut r = base(c, seed, 0xc6f8);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }

    #[test]
    fn script_cam_read_manual_fields_matches() {
        check3(ee::s07::f_0027c0f0__generated, script_cam_read_manual_fields, 40, &|c, seed| {
            let mut r = base(c, seed, 0xc0f0);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }

    #[test]
    fn script_cam_read_target_fields_matches() {
        check3(ee::s07::f_0027c300__generated, script_cam_read_target_fields, 40, &|c, seed| {
            let mut r = base(c, seed, 0xc300);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }

        #[test]
    fn chase_camera_algorithm_ctor_matches() {
        check3(ee::s01::f_00162318__generated, chase_camera_algorithm_ctor, 60, &|c, seed| {
            let _ = base(c, seed, 0x162318);
            c.g[A0][0] = sx(DATA);
        });
    }

    /// A screen whose child list matches the three names the highlight
    /// handlers look up (the hashes are computed by the game's own hash
    /// function, so the lookups really find the widgets).
    fn highlight_screen(c: &mut Cpu, names: [u32; 3]) {
        let obj = DATA;
        let screen = DATA + 0x100;
        let count = DATA + 0x200;
        let list = DATA + 0x300;
        c.bus.write32(obj + 0x40, screen);
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(count, 3);
        c.bus.write32(screen + 0x3c, list);
        for (i, name) in names.iter().enumerate() {
            c.g[A0][0] = sx(*name);
            ssx3_ee::s10::f_00317670(c);
            let hash = c.g[V0][0] as u32;
            let w = DATA + 0x1000 + 0x80 * i as u32;
            c.bus.write32(list + 4 * i as u32, w);
            c.bus.write32(w + 0x38, hash);
            c.bus.write32(w + 8, w + 0x40);
            for slot in [0x48u32, 0x58] {
                c.bus.write16(w + 0x40 + slot, 0);
                c.bus.write32(w + 0x40 + slot + 4, LEAF);
            }
            // The clear path also uses the third widget itself as a table.
            c.bus.write32(w + 0x4c, LEAF);
        }
    }

    #[test]
    fn in_game_error_highlight_continue_matches() {
        check3(ee::s04::f_001fb458__generated, in_game_error_highlight_continue, 40, &|c, seed| {
            let _ = base(c, seed, 0x1fb458);
            highlight_screen(c, [0x0046_fb48, 0x0046_fad8, 0x0046_fb00]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn in_game_error_highlight_cancel_matches() {
        check3(ee::s04::f_001fb588__generated, in_game_error_highlight_cancel, 40, &|c, seed| {
            let _ = base(c, seed, 0x1fb588);
            highlight_screen(c, [0x0046_fa90, 0x0046_fac8, 0x0046_fae8]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn gm_handler9_reset_matches() {
        check3(ee::s06::f_00239d88__generated, gm_handler9_reset, 40, &|c, seed| {
            let mut r = base(c, seed, 0x239d88);
            // The handler object with a vtable, and the global state block.
            let h = DATA;
            let vt = DATA + 0x100;
            c.bus.write32(h, vt);
            c.bus.write32(h + 4, r.next_u32());
            c.bus.write16(vt + 0x20, 0);
            c.bus.write32(vt + 0x24, LEAF);
            c.bus.write32(GAME_GP - 0x480, DATA + 0x800);
            c.g[A0][0] = sx(h);
        });
    }

    #[test]
    fn net_quick_play_popup_set_options_enabled_matches() {
        check3(ee::s03::f_001c17f8__generated, net_quick_play_popup_set_options_enabled, 40, &|c, seed| {
            let mut r = base(c, seed, 0x1c17f8);
            let obj = DATA;
            for i in 0..15u32 {
                let w = DATA + 0x2000 + 0x40 * i;
                c.bus.write32(obj + 0x6f8 + 4 * i, w);
                c.bus.write32(w + 8, w + 0x20);
                c.bus.write16(w + 0x20 + 0x40, 0);
                c.bus.write32(w + 0x20 + 0x44, LEAF);
            }
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }

    #[test]
    fn ui_screen_dtor_matches() {
        check3(ee::s13::f_0039c558__generated, ui_screen_dtor, 40, &|c, seed| {
            let mut r = base(c, seed, 0x39c558);
            let obj = DATA;
            // Zeroed screen: no names to free, empty lists.
            for i in 0..0x100u32 {
                c.bus.write32(obj + 4 * i, 0);
            }
            c.bus.write32(obj + 0x14, (r.next_u32() % 4) as u32);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

        #[test]
    fn camera_wobble_get_offset_matches() {
        check3(ee::s01::f_00160228__generated, camera_wobble_get_offset, 60, &|c, seed| {
            let mut r = base(c, seed, 0x160228);
            let obj = DATA + 0x100;
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = sx(obj);
            let inactive = seed % 3 == 0;
            c.bus.write32(obj + 0x18, if inactive { 0 } else { r.edge_f32() });
            for off in [0u32, 4, 0xc, 0x10, 0x20, 0x28] {
                c.bus.write32(obj + off, r.f32(10.0));
            }
            c.f[12] = r.f32(3.0);
            c.f[13] = r.f32(3.0);
        });
    }

    #[test]
    fn script_audio_read_fields_matches() {
        check3(ee::s07::f_0027cc28__generated, script_audio_read_fields, 40, &|c, seed| {
            let mut r = base(c, seed, 0xcc28);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }
}
