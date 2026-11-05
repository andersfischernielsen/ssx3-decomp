//! Staging wave 205 (oc244): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
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
    (
        0x003d_0d70,
        path_set_track_control_value,
        "PATH_SetTrackControlValue",
    ),
    (0x0042_c0c0, e_intr, "EIntr"),
    (0x002e_3338, lens_fx_prepare_view, "LensFX_PrepareView"),
    (
        0x0010_c0a8,
        computer_actor_rider_set_cruise_crouch,
        "ComputerActorRider_SetCruiseCrouch",
    ),
    (0x003c_7080, sndstrm_run_callbacks, "SNDSTRM_RunCallbacks"),
    (0x003c_07a8, snd_restore_vu0_flags, "SND_RestoreVu0Flags"),
    (0x0026_6ba8, c_usb_keyboard_get_key, "cUsbKeyboard_GetKey"),
    (0x0021_ea00, hud_draw_panel_segment, "HUD_DrawPanelSegment"),
    (
        0x0013_6f30,
        rider_wipeout_state_update_root_pose,
        "RiderWipeoutState_UpdateRootPose",
    ),
    (0x0022_a770, nash_collect_effects, "Nash_CollectEffects"),
];

// ---------------------------------------------------------------------------
// PATH_SetTrackControlValue (0x3d0d70)

/// PATH interactive-music globals and path-map layout used here.
mod path_ctl {
    /// `u8`: the PATH player is initialised.
    pub const ENABLED: u32 = 0x0044_f42c;
    /// `PathMap*` of the selected map (its 24 track slots at +0x58).
    pub const CURRENT_MAP: u32 = 0x0044_f420;
    /// `int` id of the selected map (0 = none selected).
    pub const CURRENT_ID: u32 = 0x0044_f430;
    /// `&maps[0].loaded` (+0x34) and `&maps[0].id` (+0x30): the loop walks
    /// both with a stride of one map.
    pub const MAP0_LOADED: u32 = 0x0051_7644;
    pub const MAP0_ID: u32 = 0x0051_7640;
    pub const MAP_SIZE: u64 = 0x928;
    pub const SLOT0: u32 = 0x58;
    /// Track bytes: the control value and its previous value.
    pub const VALUE: u32 = 0x1;
    pub const PREV_VALUE: u32 = 0x2;
}

const PATH_SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::PATH_SelectMap,
};

/// `int PATH_SetTrackControlValue(uint mask, uint value)` (0x3d0d70): set
/// the control byte of every track of every loaded path map whose id
/// matches `mask` in both its top and second nibbles (the old value moves
/// to the previous-value byte). Fails with -18 unless the player is on, no
/// map is selected and the `PATH_SelectMap(-1)` query succeeds; values of
/// 0x80 and above change nothing. Clears the selection and returns 0.
pub fn path_set_track_control_value(c: &mut Cpu) {
    use path_ctl::*;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(ENABLED).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        let mut ready = false;
        let mut busy = false;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = 0x45_0000;
            busy = Addr(CURRENT_ID).lw(c) != 0;
            c.g[V0][0] = sx(-18i32 as u32);
            if !busy {
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                PATH_SELECT_MAP.call(c, 0x003d_0dc8)?;
                ready = c.g[V0][0] != 0;
                c.g[V0][0] = (c.g[S1][0] < 0x80) as u64;
            }
        }
        if !busy && !ready {
            c.cycles += 2;
            c.g[V0][0] = sx(-18i32 as u32);
        } else if ready {
            c.cycles += 2;
            c.g[S5][0] = 0x45_0000;
            if c.g[V0][0] != 0 {
                set_values(c)?;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            Addr(CURRENT_MAP).sw(c, 0);
            Addr(CURRENT_ID).sw(c, 0);
            c.g[V0][0] = 0;
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The map loop of `PATH_SetTrackControlValue`: a0 = map index, a1 = its
/// byte offset (`mult`), s1 = value, s2 = mask.
fn set_values(c: &mut Cpu) -> crate::call::Flow {
    use path_ctl::*;
    c.cycles += 8;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = 0;
    c.g[S3][0] = sx(MAP0_LOADED);
    c.g[S7][0] = MAP_SIZE;
    c.g[FP][0] = sx(MAP0_ID);
    c.g[S6][0] = 0xffff_ffff_f000_0000;
    c.g[S4][0] = 0x0f00_0000;
    c.mult(0, A1, A0, S7);
    loop {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S3)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(1));
        // `direct`: the second-nibble test failed; skip the 1-cycle tail.
        let mut direct = false;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(r32(c, FP))).lw(c);
            let top = c.g[V1][0] & c.g[S6][0] & c.g[S2][0];
            c.g[V0][0] = c.g[V1][0] & c.g[S4][0];
            if top != 0 {
                c.cycles += 3;
                c.g[V0][0] &= c.g[S2][0];
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[S0][0];
                    direct = true;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] &= 0xff;
                    PATH_SELECT_MAP.call(c, 0x003d_0e3c)?;
                    let selected = c.g[V0][0] != 0;
                    c.g[A0][0] = 0;
                    if selected {
                        set_map_tracks(c);
                    }
                }
            }
        }
        if !direct {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.mult(0, A1, A0, S7);
        if event_due(c) {
            deliver_event_here(c, 0x003d_0e00);
        }
    }
}

/// Write the value (s1) into the 24 track slots of the selected map.
fn set_map_tracks(c: &mut Cpu) {
    use path_ctl::*;
    c.cycles += 3;
    c.g[A1][0] = 0x45_0000;
    c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
    loop {
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SLOT0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            let track = Addr(r32(c, V1));
            let (value, prev) = (c.g[S1][0], track.off(VALUE).lbu(c));
            c.g[V0][0] = prev;
            track.off(VALUE).sb(c, value);
            track.off(PREV_VALUE).sb(c, prev);
        }
        c.cycles += 3;
        let more = (c.g[A0][0] as i64) < 24;
        c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_0e50);
        }
    }
}

// ---------------------------------------------------------------------------
// EIntr (0x42c0c0)

use crate::game::misc::STATUS_EIE;

/// `int EIntr(void)` (0x42c0c0): enable interrupts (`ei`); returns 1 if
/// they were already enabled.
pub fn e_intr(c: &mut Cpu) {
    c.cycles += 6;
    c.mfc0(V0, 12);
    c.g[V1][0] = STATUS_EIE;
    c.g[V0][0] &= c.g[V1][0];
    c.ei();
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    ret(c);
}

// ---------------------------------------------------------------------------
// LensFX_PrepareView (0x2e3338)

/// Layout used by `LensFX_PrepareView`.
mod lens_prep {
    use super::*;

    /// The camera / graphics object (`*(gp + 0x2a90)`), its interface's
    /// vtable pointer at +0x10d8.
    pub const CAMERA_GP: u32 = 0x2a90;
    pub const CAM_VT: u32 = 0x10d8;
    /// Interface slots: `GetViewRect(out)` (16-byte struct returned through
    /// a hidden pointer), `GetViewMatrix()` (returns a 4x4 matrix pointer),
    /// `GetFov(&x, &y, &z)`.
    pub const VT_VIEW_RECT: u32 = 0xe0;
    pub const VT_VIEW_MATRIX: u32 = 0x118;
    pub const VT_FOV: u32 = 0xf0;

    /// LensFX: view rect copy at +0, view matrix copy at +0x10, the three
    /// FOV outputs at +0x50/+0x54/+0x58, per-view source counts at +0x60,
    /// per-view flare arrays (0x3000 bytes, 0x30 per flare) at +0x68, per-view
    /// sun state (0x210 bytes) at +0x6070.
    pub const MATRIX: u32 = 0x10;
    pub const FOV_X: u32 = 0x50;
    pub const FOV_Y: u32 = 0x54;
    pub const FOV_Z: u32 = 0x58;
    pub const COUNTS: u32 = 0x60;
    pub const FLARES: u32 = 0x68;
    pub const FLARE_VIEW_SIZE: u64 = 0x3000;
    pub const FLARE_SIZE: u32 = 0x30;
    pub const SUN: u32 = 0x6070;
    pub const SUN_SIZE: u64 = 0x210;

    pub const UPDATE_SUN_POSITION: Callee = Callee {
        addr: 0x002f_4db8,
        f: ee::s09::LensFX_UpdateSunPosition,
    };
    pub const PROJECT_SOURCE: Callee = Callee {
        addr: 0x002e_2b00,
        f: ee::s09::LensFX_ProjectSource,
    };
}

/// `void LensFX_PrepareView(LensFX* fx, int view)` (0x2e3338): per view,
/// before the flares are drawn: copy the camera's view rect, view matrix
/// (VU0 quadword copy) and FOV values into the lens FX, update the sun
/// position for this view, then project every flare source of the view.
pub fn lens_fx_prepare_view(c: &mut Cpu) {
    use crate::call::VSlot;
    use lens_prep::*;
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A0][0]; // fx
        frame.save(c, S2, 0x30);
        c.g[S3][0] = c.g[A1][0]; // view
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        let gp = r32(c, GP);
        c.g[S0][0] = Addr(gp.wrapping_add(CAMERA_GP)).lw(c);
        let cam = c.g[S0][0];

        // The view rect, returned into sp+0 and copied (unaligned) to fx+0.
        c.g[V0][0] = Addr((cam as u32).wrapping_add(CAM_VT)).lw(c);
        let vt = c.g[V0][0];
        let slot = VSlot::read(c, vt, VT_VIEW_RECT);
        c.g[V1][0] = slot.func;
        c.g[A1][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, 0x002e_3374)?;
        let (stack, fx) = (sp(c), r32(c, S1));
        c.ldl(V0, stack.wrapping_add(0x7));
        c.ldr(V0, stack);
        c.ldl(V1, stack.wrapping_add(0xf));
        c.ldr(V1, stack.wrapping_add(0x8));
        c.sdl(V0, fx.wrapping_add(0x7));
        c.sdr(V0, fx);
        c.sdl(V1, fx.wrapping_add(0xf));
        c.sdr(V1, fx.wrapping_add(0x8));

        // The view matrix.
        c.g[V1][0] = Addr((cam as u32).wrapping_add(CAM_VT)).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_VIEW_MATRIX);
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, 0x002e_33a8)?;
        let src = r32(c, V0);
        let dst = fx.wrapping_add(MATRIX);
        c.g[V1][0] = sx(dst);
        for row in 0..4u32 {
            c.lqc2(1 + row as usize, src.wrapping_add(0x10 * row));
        }
        for row in 0..4u32 {
            c.sqc2(1 + row as usize, dst.wrapping_add(0x10 * row));
        }

        // The FOV values.
        c.g[V1][0] = Addr((cam as u32).wrapping_add(CAM_VT)).lw(c);
        c.g[A3][0] = sx(fx.wrapping_add(FOV_Z));
        c.g[A1][0] = sx(fx.wrapping_add(FOV_X));
        c.g[A2][0] = sx(fx.wrapping_add(FOV_Y));
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_FOV);
        c.g[S2][0] = 0;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, 0x002e_33f0)?;

        // The sun of this view.
        c.g[A0][0] = SUN_SIZE;
        c.g[V0][0] = sx(fx.wrapping_add(SUN));
        c.mult(0, A0, S3, A0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        UPDATE_SUN_POSITION.call(c, 0x002e_340c)?;

        // Project the view's flare sources.
        c.g[A0][0] = sx(r32(c, S3) << 2);
        c.g[V0][0] = sx(fx.wrapping_add(COUNTS));
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, A3)).lw(c);
        c.g[V0][0] = FLARE_VIEW_SIZE;
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V1][0] = sx(fx.wrapping_add(FLARES));
            c.mult(0, V0, S3, V0);
            c.g[S3][0] = c.g[A3][0]; // &counts[view]
            c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                PROJECT_SOURCE.call(c, 0x002e_3444)?;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLARE_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_3438);
                }
            }
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

// ---------------------------------------------------------------------------
// ComputerActorRider_SetCruiseCrouch (0x10c0a8)

/// Layout used by `ComputerActorRider_SetCruiseCrouch`.
mod cruise {
    /// Computer actor: the rider (`Rider*` at +0x18; its velocity vector at
    /// +0x1e0) and the rider's speed at +0xdf8.
    pub const RIDER: u32 = 0x18;
    pub const RIDER_VELOCITY: u32 = 0x1e0;
    pub const SPEED: u32 = 0xdf8;
    /// `.lit4`: the fast-speed threshold (833.33) and the cruise crouch
    /// amount (0.97).
    pub const LIT_FAST: u32 = 0x7de4;
    pub const LIT_CROUCH: u32 = 0x7de0;
    /// The virtual pad word: crouch amount (0..31) in its top 6 bits.
    pub const PAD_CROUCH_KEEP: u64 = 0x03ff_ffff;
    pub const PAD_CROUCH_SHIFT: u32 = 26;
}

/// `void ComputerActorRider_SetCruiseCrouch(ComputerActor* self, Pad* pad)`
/// (0x10c0a8): the AI's crouch while cruising: full (1.0) below the fast
/// speed (|velocity| < 833.33, VU0 length), 0.97 when faster, none below
/// speed 20; written as `crouch * 31` into the pad word's top 6 bits.
pub fn computer_actor_rider_set_cruise_crouch(c: &mut Cpu) {
    use cruise::*;
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.cycles += 17;
    let gp = r32(c, GP);
    let me = abi::arg(c, 0);
    c.lwc1(1, gp.wrapping_sub(LIT_FAST));
    c.g[V0][0] = Addr(me.wrapping_add(RIDER)).lw(c);
    c.lqc2(3, r32(c, V0).wrapping_add(RIDER_VELOCITY));
    // |velocity| (x, y, z) into Q, then f0.
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    c.lwc1(2, gp.wrapping_sub(LIT_CROUCH));
    c.c_lt_s(0, 1);
    let slow = c.fcc();
    c.lwc1(1, me.wrapping_add(SPEED));
    if slow {
        c.cycles += 2;
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(AT, 2);
    }
    c.cycles += 5;
    c.g[AT][0] = 0x41a0_0000; // 20.0
    c.mtc1(AT, 0);
    c.c_lt_s(1, 0);
    if c.fcc() {
        c.cycles += 1;
        c.mtc1(0, 2);
    }
    c.cycles += 13;
    c.g[AT][0] = 0x41f8_0000; // 31.0
    c.mtc1(AT, 0);
    c.g[V1][0] = PAD_CROUCH_KEEP;
    let pad = Addr(abi::arg(c, 1));
    c.g[A0][0] = pad.lw(c);
    c.mul_s(0, 2, 0);
    c.g[A0][0] &= c.g[V1][0];
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.g[V0][0] = sx(r32(c, V0) << PAD_CROUCH_SHIFT);
    c.g[A0][0] |= c.g[V0][0];
    let word = c.g[A0][0];
    pad.sw(c, word);
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_RunCallbacks (0x3c7080)

/// The sound-stream callback queue.
mod sndstrm_cb {
    /// `int`: queued callback events.
    pub const COUNT: u32 = 0x0051_57b8;
    /// The events (8 bytes: `u16 kind`, `u16 stream`, `u32 value`).
    pub const EVENTS: u32 = 0x0051_57bc;
    pub const EVENT_SIZE: u32 = 8;
    pub const EV_KIND: u32 = 0;
    pub const EV_STREAM: u32 = 2;
    pub const EV_VALUE: u32 = 4;
    /// `Stream*` per stream id, after the events.
    pub const STREAMS: u32 = EVENTS + 0x300;
    /// Stream: `value_cb(value, user)`, `stream_cb(id, value, user)`, user.
    pub const VALUE_CB: u32 = 0x54;
    pub const STREAM_CB: u32 = 0x58;
    pub const USER: u32 = 0x5c;
}

/// `void SNDSTRM_RunCallbacks(void)` (0x3c7080): deliver the queued stream
/// callback events: kind 0 calls the stream's `stream_cb(id, value, user)`,
/// any other kind its `value_cb(value, user)`; then empty the queue.
pub fn sndstrm_run_callbacks(c: &mut Cpu) {
    use sndstrm_cb::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = 0x51_0000;
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = 0;
        c.g[V0][0] = Addr(COUNT).lw(c);
        c.g[S4][0] = c.g[S2][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(EVENTS);
            c.g[S3][0] = sx(STREAMS);
            c.g[S0][0] = c.g[V0][0];
            loop {
                c.cycles += 6;
                let ev = r32(c, S0);
                c.g[A0][0] = Addr(ev + EV_STREAM).lhu(c);
                c.g[V1][0] = Addr(ev + EV_KIND).lhu(c);
                c.g[V0][0] = sx((r32(c, A0) << 2).wrapping_add(r32(c, S3)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let stream = r32(c, V0);
                if c.g[V1][0] == 0 {
                    c.cycles += 6;
                    c.g[A2][0] = Addr(stream + USER).lw(c);
                    c.g[V0][0] = Addr(stream + STREAM_CB).lw(c);
                    c.g[A1][0] = Addr(ev + EV_VALUE).lw(c);
                    call_ptr(c, r32(c, V0), 0x003c_70e8)?;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = Addr(stream + USER).lw(c);
                    c.g[V0][0] = Addr(stream + VALUE_CB).lw(c);
                    c.g[A0][0] = Addr(ev + EV_VALUE).lw(c);
                    call_ptr(c, r32(c, V0), 0x003c_7100)?;
                }
                c.g[V0][0] = Addr(COUNT).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(EVENT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_70c0);
                }
            }
        }
        c.cycles += 9;
        Addr(COUNT).sw(c, 0);
        frame.restore(c, S0, 0x50);
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
// SND_RestoreVu0Flags (0x3c07a8)

/// `void SND_RestoreVu0Flags(int status)` (0x3c07a8): rebuild the VU0 MAC /
/// status flags saved by the sound mixer by multiplying two vectors built
/// from `status` (one lane per bit, shifted each lane): `a` is 0 / 1.0
/// (no overflow/underflow bit 0x8800) or 1e-30 / 1e30 (bit 8 selects the
/// small value), `b` is `a` negated where bit 0x80 is set. VF1 and VF2 are
/// kept across the `vmul` (`qmfc2`/`qmtc2` through t0/t1).
pub fn snd_restore_vu0_flags(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0; // lane
    c.g[A3][0] = sx(sp(c).wrapping_add(0x10)); // b
    c.g[A1][0] = c.g[SP][0]; // &a[lane]
    loop {
        c.cycles += 3;
        let saturated = c.g[A0][0] & 0x8800 != 0;
        c.g[V0][0] = c.g[A0][0] & 0x8;
        let small = c.g[V0][0] != 0;
        // `tail`: the 1-cycle block that tests the sign bit.
        let tail = if !saturated {
            c.cycles += 3;
            c.mtc1(0, 0); // 0.0
            c.g[V1][0] = sx(r32(c, A2) << 2);
            if small {
                true
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000; // 1.0
                c.mtc1(AT, 0);
                c.g[V0][0] = c.g[A0][0] & 0x80;
                false
            }
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x0da2_4260; // 1e-30
            c.mtc1(AT, 0);
            c.g[V1][0] = sx(r32(c, A2) << 2);
            if !small {
                c.cycles += 3;
                c.g[AT][0] = 0x7149_f2ca; // 1e30
                c.mtc1(AT, 0);
            }
            true
        };
        if tail {
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0] & 0x80;
        }
        c.cycles += 2;
        c.swc1(0, r32(c, A1));
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.neg_s(0, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        c.swc1(0, r32(c, V0));
        c.g[A0][0] = sx(r32(c, A0) << 1);
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_07b8);
        }
    }
    c.cycles += 10;
    c.qmfc2(T0, 1);
    c.qmfc2(T1, 2);
    c.lqc2(1, sp(c));
    c.lqc2(2, sp(c).wrapping_add(0x10));
    c.vu0().mul(0b1111, 1, 1, Src::Vec(2));
    c.qmtc2(T0, 1);
    c.qmtc2(T1, 2);
    c.vu0().vnop();
    frame.close(c);
}

// ---------------------------------------------------------------------------
// cUsbKeyboard_GetKey (0x266ba8)

/// Layout used by `cUsbKeyboard_GetKey`.
mod usb_kb {
    /// Per-keyboard channels (0x8c bytes): queued key count at +0, read
    /// index at +4, a 64-entry ring of key pairs (two bytes) at +8.
    pub const CHANNEL_SIZE: u64 = 0x8c;
    pub const COUNT: u32 = 0x0;
    pub const READ: u32 = 0x4;
    pub const RING: u32 = 0x8;
    /// Keyboard: non-zero once opened.
    pub const ACTIVE: u32 = 0x460;
    /// `int UsbKb_GetMaxKeyboards(void)`.
    pub const GET_MAX_KEYBOARDS: u32 = 0x003f_6348;
}

/// `int cUsbKeyboard_GetKey(cUsbKeyboard* kb, int ch, int* key, int* mod)`
/// (0x266ba8): pop the next queued key pair of keyboard channel `ch`.
/// `*key` and `*mod` are -1 unless the popped pair's byte is non-zero.
/// Returns 1 if a pair was popped, 0 if the keyboard is closed, `ch` is out
/// of range or the queue is empty.
pub fn c_usb_keyboard_get_key(c: &mut Cpu) {
    use usb_kb::*;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A2][0]; // key
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0]; // kb
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0]; // mod
        let (key, modifier, kb) = (Addr(r32(c, S3)), Addr(r32(c, S4)), r32(c, S0));
        key.sw(c, u64::MAX);
        modifier.sw(c, u64::MAX);
        c.g[V0][0] = Addr(kb + ACTIVE).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0]; // ch
        let mut popped = false;
        if active {
            c.cycles += 5;
            call_ptr(c, GET_MAX_KEYBOARDS, 0x0026_6bf0)?;
            let in_range = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = (in_range) as u64;
            c.g[V0][0] = CHANNEL_SIZE;
            if in_range {
                c.cycles += 9;
                c.g[V1][0] = sx(kb.wrapping_add(READ));
                c.mult(0, A1, S1, V0);
                let off = r32(c, A1);
                c.g[A3][0] = sx(r32(c, V1).wrapping_add(off));
                c.g[A0][0] = sx(kb.wrapping_add(off));
                let read_at = Addr(r32(c, A3));
                c.g[A2][0] = read_at.lw(c);
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                let queued = (c.g[A2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = queued as u64;
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if queued {
                    // The ring slot `read % 64` (C remainder), two bytes each.
                    c.cycles += 12;
                    let read = r32(c, A2);
                    c.g[V0][0] = if c.g[V1][0] != 0 {
                        c.g[A2][0]
                    } else {
                        sx(read.wrapping_add(0x3f))
                    };
                    c.g[A0][0] = sx(kb.wrapping_add(RING));
                    let rounded = ((r32(c, V0) as i32 >> 6) as u32) << 6;
                    let slot = read.wrapping_sub(rounded) << 1;
                    c.g[V0][0] = sx(slot);
                    c.g[V1][0] = sx(slot.wrapping_add(off));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
                    if c.g[V0][0] != 0 {
                        let k = c.g[V0][0];
                        key.sw(c, k);
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(kb.wrapping_add(RING + 1).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    c.g[V0][0] = c.g[V1][0];
                    if c.g[V1][0] != 0 {
                        c.cycles += 1;
                        let m = c.g[V0][0];
                        modifier.sw(c, m);
                    }
                    c.cycles += 5;
                    c.g[V1][0] = read_at.lw(c);
                    c.g[V0][0] = 1;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                    let next = c.g[V1][0];
                    read_at.sw(c, next);
                    popped = true;
                }
            }
        }
        if !popped {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
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
// HUD_DrawPanelSegment (0x21ea00)

/// Layout used by `HUD_DrawPanelSegment`.
mod hud_seg {
    /// The renderer (`*(gp - 0x854)`): interface vtable at +0x10d8, slot
    /// 0x238 = `DrawStrip(verts, count, flags)`.
    pub const RENDERER_GP: u32 = 0x854;
    pub const RENDERER_VT: u32 = 0x10d8;
    pub const VT_DRAW_STRIP: u32 = 0x238;
    /// Strip vertex (0x30 bytes, on the stack): u, v, q at +0, colour words
    /// at +0x10..+0x1c, position vector at +0x20.
    pub const VERTEX_SIZE: u32 = 0x30;
    pub const V_Q: u32 = 0x8;
    pub const V_COLOUR: u32 = 0x10;
    /// Taller panels get 36-pixel end caps (8 vertices instead of 4).
    pub const CAP_MIN_HEIGHT: u32 = 0x4290_0000; // 72.0
}

/// `void HUD_DrawPanelSegment(void*, const float colour[4], const float
/// pos[2], float w, float h, float u0, float v0, float u1, float v1)`
/// (0x21ea00): draw a textured HUD panel piece as one vertex strip through
/// the renderer: 4 vertices, or 8 when `h > 72` (36-pixel caps at the top
/// and bottom, the middle rows at the texture's middle v). Colour channels
/// are scaled by 128 to integers; q is 1. The vertices are built on the
/// stack as the original does (vector temporaries at sp+0x180 / sp+0xc0).
pub fn hud_draw_panel_segment(c: &mut Cpu) {
    use crate::call::VSlot;
    use hud_seg::*;
    guest(c, |c| {
        c.cycles += 8;
        c.g[AT][0] = CAP_MIN_HEIGHT as u64;
        c.mtc1(AT, 0);
        let frame = Frame::open(c, 0x1a0);
        frame.save_ra(c, 0x190);
        c.c_lt_s(0, 13);
        let capped = c.fcc();
        c.g[T1][0] = c.g[A2][0]; // pos
        let (count, ctor_head, vert_head) = if capped {
            c.cycles += 4;
            c.add_s(7, 15, 17); // v0 + v1 (halved below: the middle v)
            c.g[V0][0] = 7;
            (8u32, 0x0021_ea30, 0x0021_eaa0)
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            (4u32, 0x0021_ebf8, 0x0021_ec68)
        };
        c.g[V1][0] = u64::MAX;
        // The vertex array's (empty) constructor loop.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[A0][0] = 0;
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ctor_head);
            }
        }
        // Colour channels * 128 as integers: t0, a3, a1, a2 = r, g, b, a.
        c.cycles += 21;
        let colour = r32(c, A1);
        c.g[AT][0] = 0x4300_0000; // 128.0
        c.mtc1(AT, 1);
        c.lwc1(0, colour + 0x8);
        c.g[V1][0] = c.g[SP][0];
        c.lwc1(4, colour + 0xc);
        c.lwc1(2, colour);
        c.mul_s(0, 0, 1);
        c.lwc1(3, colour + 0x4);
        c.mul_s(4, 4, 1);
        c.mul_s(2, 2, 1);
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(AT, 5);
        c.mul_s(3, 3, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(A1, 1);
        c.cvt_w_s(1, 2);
        c.mfc1(T0, 1);
        c.cvt_w_s(1, 3);
        c.mfc1(A3, 1);
        c.cvt_w_s(0, 4);
        c.mfc1(A2, 0);
        // Colour, q = 1 and u (u0 for even vertices, u1 for odd ones).
        loop {
            c.cycles += 7;
            let v = r32(c, V1);
            c.sw(v + V_COLOUR + 0xc, c.g[T0][0]);
            c.g[V0][0] = c.g[A0][0] & 1;
            c.sw(v + V_COLOUR, c.g[A3][0]);
            c.sw(v + V_COLOUR + 0x4, c.g[A1][0]);
            c.sw(v + V_COLOUR + 0x8, c.g[A2][0]);
            c.swc1(5, v + V_Q);
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.swc1(16, v);
            } else {
                c.cycles += 1;
                c.swc1(14, v);
            }
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[V0][0] = ((c.g[A0][0] as i64) < count as i64) as u64;
            c.g[V1][0] = sx(v.wrapping_add(VERTEX_SIZE));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, vert_head);
            }
        }
        // v coordinates and positions (x, y, 0, 1) built through a vector
        // temporary; a1 = count, a2 = the vertices, a3 = 0.
        let s = sp(c);
        let pos = r32(c, T1);
        if capped {
            c.cycles += 69;
            c.lwc1(0, pos + 0x4);
            c.g[A1][0] = 0x8;
            c.lwc1(3, pos);
            c.g[A2][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.add_s(6, 0, 13);
            c.swc1(3, s + 0x180);
            c.add_s(5, 3, 12);
            c.sw(s + 0x188, 0u64);
            c.g[A3][0] = 0u64;
            c.swc1(0, s + 0x184);
            c.swc1(1, s + 0x18c);
            c.g[AT][0] = 0x42100000;
            c.mtc1(1, 2);
            c.lq(8, s + 0x180);
            c.swc1(5, s + 0x180);
            c.add_s(4, 0, 2);
            c.sw(s + 0x188, 0u64);
            c.sub_s(2, 6, 2);
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 0);
            c.lq(4, s + 0x180);
            c.swc1(3, s + 0x180);
            c.mul_s(0, 7, 0);
            c.sw(s + 0x188, 0u64);
            c.swc1(4, s + 0x184);
            c.sq(8, s + 0x20);
            c.lq(2, s + 0x180);
            c.swc1(5, s + 0x180);
            c.sw(s + 0x188, 0u64);
            c.sq(4, s + 0x50);
            c.lq(3, s + 0x180);
            c.sq(2, s + 0x80);
            c.swc1(15, s + 0x34);
            c.swc1(0, s + 0xf4);
            c.swc1(17, s + 0x154);
            c.swc1(15, s + 0x4);
            c.swc1(0, s + 0x64);
            c.swc1(0, s + 0x94);
            c.swc1(0, s + 0xc4);
            c.swc1(17, s + 0x124);
            c.sq(3, s + 0xb0);
            c.sw(s + 0x188, 0u64);
            c.swc1(3, s + 0x180);
            c.swc1(2, s + 0x184);
            c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            c.lq(2, s + 0x180);
            c.swc1(5, s + 0x180);
            c.sw(s + 0x188, 0u64);
            c.sq(2, s + 0xe0);
            c.lq(3, s + 0x180);
            c.swc1(3, s + 0x180);
            c.sw(s + 0x188, 0u64);
            c.swc1(6, s + 0x184);
            c.sq(3, s + 0x110);
            c.lq(4, s + 0x180);
            c.swc1(5, s + 0x180);
            c.sw(s + 0x188, 0u64);
            c.sq(4, s + 0x140);
            c.lq(2, s + 0x180);
            c.sq(2, s + 0x170);
            let vt = Addr(r32(c, T0).wrapping_add(RENDERER_VT)).lw(c);
            c.g[V1][0] = vt;
            let slot = VSlot::read(c, vt, VT_DRAW_STRIP);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(c.g[T0][0]);
            call_ptr(c, slot.func as u32, 0x0021_ebe4)?;
        } else {
            c.cycles += 39;
            c.lwc1(0, pos + 0x4);
            c.g[A1][0] = 0x4;
            c.lwc1(1, pos);
            c.g[A2][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            c.add_s(4, 0, 13);
            c.swc1(1, s + 0xc0);
            c.add_s(3, 1, 12);
            c.sw(s + 0xc8, 0u64);
            c.g[A3][0] = 0u64;
            c.swc1(0, s + 0xc4);
            c.swc1(2, s + 0xcc);
            c.swc1(15, s + 0x34);
            c.lq(2, s + 0xc0);
            c.swc1(3, s + 0xc0);
            c.sw(s + 0xc8, 0u64);
            c.sq(2, s + 0x20);
            c.lq(4, s + 0xc0);
            c.swc1(1, s + 0xc0);
            c.sw(s + 0xc8, 0u64);
            c.swc1(4, s + 0xc4);
            c.swc1(17, s + 0x94);
            c.lq(3, s + 0xc0);
            c.swc1(3, s + 0xc0);
            c.sw(s + 0xc8, 0u64);
            c.sq(4, s + 0x50);
            c.lq(2, s + 0xc0);
            c.sq(3, s + 0x80);
            c.sq(2, s + 0xb0);
            c.swc1(15, s + 0x4);
            c.swc1(17, s + 0x64);
            c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            let vt = Addr(r32(c, T0).wrapping_add(RENDERER_VT)).lw(c);
            c.g[V0][0] = vt;
            let slot = VSlot::read(c, vt, VT_DRAW_STRIP);
            c.g[V1][0] = slot.func;
            c.g[A0][0] = slot.this(c.g[T0][0]);
            call_ptr(c, slot.func as u32, 0x0021_ed38)?;
        }
        frame.restore_ra(c, 0x190);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_UpdateRootPose (0x136f30)

/// Layout used by `RiderWipeoutState_UpdateRootPose`.
mod wipeout_pose {
    /// Wipeout state: tumble velocity (vec4) at +0x10, angular velocity at
    /// +0x20, the rider at +0x40.
    pub const VEL: u32 = 0x10;
    pub const ANG_VEL: u32 = 0x20;
    pub const RIDER: u32 = 0x40;
    /// Rider: root position (vec4) at +0x130, root rotation quaternion at
    /// +0x140, active flag at +0x150, time scale at +0x300.
    pub const R_POS: u32 = 0x130;
    pub const R_QUAT: u32 = 0x140;
    pub const R_ACTIVE: u32 = 0x150;
    pub const R_RATE: u32 = 0x300;
    /// `.lit4`: 1/60 (seconds per tick), -0.2 (horizontal drag), 3333.33
    /// (top tumble speed).
    pub const LIT_TICK: u32 = 0x72b8;
    pub const LIT_DRAG: u32 = 0x72b4;
    pub const LIT_MAX_SPEED: u32 = 0x72b0;
}

/// `void RiderWipeoutState_UpdateRootPose(RiderWipeoutState* self)`
/// (0x136f30): integrate the tumbling rider's root while wiped out (if the
/// rider is active), with `dt = rate / 60`: position += vel * dt; vel +=
/// (vel.x * -0.2, vel.y * -0.2, -1800, 0) * dt (drag and gravity), clamped
/// to length 3333.33; angular velocity *= 1 - dt / 2; the root quaternion
/// += its derivative from the angular velocity (VU0 quaternion product) *
/// dt, then renormalised (`vrsqrt`). VU0 macro ops as in the original; the
/// vector temporaries live on the stack.
pub fn rider_wipeout_state_update_root_pose(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    use wipeout_pose::*;
    let gp = r32(c, GP);
    c.cycles += 5;
    c.g[A1][0] = c.g[A0][0];
    let me = r32(c, A1);
    c.g[V1][0] = c.lw(me + RIDER);
    c.g[V0][0] = c.lw(r32(c, V1) + R_ACTIVE);
    let active = c.g[V0][0] != 0;
    let frame = Frame::open(c, 0x30);
    let s = sp(c);
    if active {
        // dt (f4) = rate / 60; position += vel * dt.
        c.cycles += 51;
        c.lwc1(1, r32(c, V1) + R_RATE);
        c.lwc1(0, gp.wrapping_sub(LIT_TICK));
        c.lwc1(2, gp.wrapping_sub(LIT_DRAG));
        c.mul_s(4, 1, 0);
        c.g[AT][0] = 0xffffffffc4e10000;
        c.mtc1(1, 3);
        c.mfc1(2, 4);
        c.lqc2(4, me + VEL);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x10);
        c.lwc1(5, gp.wrapping_sub(LIT_MAX_SPEED));
        c.lq(2, s + 0x10);
        c.g[V1][0] = c.lw(me + RIDER);
        c.sq(2, s);
        c.lqc2(3, r32(c, V1) + R_POS);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, V1) + R_POS);
        c.lwc1(0, me + VEL);
        c.lwc1(1, me + VEL + 4);
        c.mul_s(0, 0, 2);
        c.swc1(3, s + 0x18);
        c.mul_s(1, 1, 2);
        c.sw(s + 0x1c, 0u64);
        c.swc1(0, s + 0x10);
        c.swc1(1, s + 0x14);
        c.mfc1(3, 4);
        c.lqc2(4, s + 0x10);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x20);
        c.lq(2, s + 0x20);
        c.sq(2, s);
        c.lqc2(3, me + VEL);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, me + VEL);
        c.lqc2(3, me + VEL);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.c_lt_s(5, 0);
        if c.fcc() {
            // Faster than the top speed: vel *= max / |vel|.
            c.cycles += 6;
            c.div_s(0, 5, 0);
            c.mfc1(2, 0);
            c.lqc2(4, me + VEL);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, me + VEL);
        }
        // Angular damping, the quaternion derivative and renormalisation.
        c.cycles += 65;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.g[A0][0] = 0x3f000000;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.mul_s(1, 4, 1);
        c.sub_s(0, 0, 1);
        c.mfc1(2, 0);
        c.lqc2(4, me + ANG_VEL);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, me + ANG_VEL);
        c.g[V1][0] = c.lw(me + RIDER);
        c.qmtc2(4, 1);
        c.lqc2(4, r32(c, V1) + R_QUAT);
        c.lqc2(5, me + ANG_VEL);
        c.vu0().suba(0b0001, 0, Src::Vec(0));
        c.vu0().msub(0b0001, 2, 0, Src::Bc(1, X));
        c.vu0().mul(0b1110, 7, 4, Src::Vec(5));
        c.vu0().mul(0b1110, 3, 5, Src::Bc(4, W));
        c.vu0().opmula(5, 4);
        c.vu0().opmsub(6, 4, 5);
        c.vu0().mula(0b0001, 2, Src::Bc(7, X));
        c.vu0().madda(0b0001, 2, Src::Bc(7, Y));
        c.vu0().madd(0b0001, 8, 2, Src::Bc(7, Z));
        c.vu0().mula(0b1110, 3, Src::Bc(1, X));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(1, X));
        c.sqc2(8, s + 0x20);
        c.lq(2, s + 0x20);
        c.sq(2, s + 0x10);
        c.mfc1(3, 4);
        c.lqc2(4, s + 0x10);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x20);
        c.lq(2, s + 0x20);
        c.g[V1][0] = c.lw(me + RIDER);
        c.sq(2, s);
        c.lwc1(0, s);
        c.lwc1(1, r32(c, V1) + R_QUAT);
        c.lwc1(2, r32(c, V1) + R_QUAT + 4);
        c.add_s(1, 1, 0);
        c.lwc1(3, r32(c, V1) + R_QUAT + 8);
        c.lwc1(4, r32(c, V1) + R_QUAT + 0xc);
        c.swc1(1, r32(c, V1) + R_QUAT);
        c.lwc1(0, s + 0x4);
        c.add_s(2, 2, 0);
        c.swc1(2, r32(c, V1) + R_QUAT + 4);
        c.lwc1(0, s + 0x8);
        c.add_s(3, 3, 0);
        c.swc1(3, r32(c, V1) + R_QUAT + 8);
        c.lwc1(0, s + 0xc);
        c.add_s(4, 4, 0);
        c.swc1(4, r32(c, V1) + R_QUAT + 0xc);
        c.g[V0][0] = c.lw(me + RIDER);
        c.lqc2(3, r32(c, V0) + R_QUAT);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, r32(c, V0) + R_QUAT);
    }
    c.cycles += 2;
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Nash_CollectEffects (0x22a770)

/// Layout used by `Nash_CollectEffects`.
mod nash_fx {
    /// Nash: emitter list (count at +0x78b4, entries from +0x78b8) and
    /// light list (count at +0x7bc0, entries from +0x7bc4).
    pub const EMITTER_COUNT: u32 = 0x78b4;
    pub const EMITTERS: u32 = 0x78b8;
    pub const LIGHT_COUNT: u32 = 0x7bc0;
    pub const LIGHTS: u32 = 0x7bc4;
    /// Visible-node pair (8 bytes): world node at +0, its clip word at +4
    /// (0 = fully inside, no sphere test).
    pub const PAIR_SIZE: u32 = 8;
    pub const PAIR_CLIP: u32 = 4;
    /// World node: effect list head at +0x28. Effect: next at +0, kind at
    /// +8, bounding sphere at +0x50.
    pub const NODE_EFFECTS: u32 = 0x28;
    pub const FX_KIND: u32 = 0x8;
    pub const FX_SPHERE: u32 = 0x50;
    pub const KIND_EMITTER: u64 = 7;
    pub const KIND_LIGHT: u64 = 8;
    /// The sphere test microprogram's result (in vi1) for "outside".
    pub const OUTSIDE: u64 = 2;
}

/// `void Nash_CollectEffects(Nash* this, NodePair* nodes, int count)`
/// (0x22a770): walk the effects of every visible world node: emitters
/// (kind 7) are appended to the emitter list unless the node is clipped
/// and the VU0 sphere test (microprogram 0xef0, clip word in vi14) puts the
/// emitter's sphere outside; lights (kind 8) are always appended to the
/// light list.
pub fn nash_collect_effects(c: &mut Cpu) {
    use nash_fx::*;
    c.cycles += 2;
    if (c.g[A2][0] as i64) > 0 {
        let me = r32(c, A0);
        loop {
            c.cycles += 5;
            let pair = r32(c, A1);
            c.g[V0][0] = Addr(pair).lw(c);
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A3][0] = Addr(r32(c, V0).wrapping_add(NODE_EFFECTS)).lw(c);
            c.g[T2][0] = sx(pair.wrapping_add(PAIR_SIZE));
            if c.g[A3][0] != 0 {
                c.cycles += 7;
                c.g[T5][0] = KIND_EMITTER;
                c.g[T4][0] = OUTSIDE;
                c.g[T1][0] = sx(me.wrapping_add(EMITTERS));
                c.g[T3][0] = KIND_LIGHT;
                c.g[T0][0] = sx(me.wrapping_add(LIGHTS));
                c.g[V0][0] = Addr(r32(c, A3).wrapping_add(FX_KIND)).lw(c);
                loop {
                    c.cycles += 2;
                    let fx = r32(c, A3);
                    // Some((count field, list)) when the effect is appended.
                    let append = if c.g[V0][0] == c.g[T5][0] {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(pair.wrapping_add(PAIR_CLIP)).lw(c);
                        c.g[V0][0] = sx(fx.wrapping_add(FX_SPHERE));
                        let culled = c.g[V1][0] != 0 && {
                            c.cycles += 7;
                            c.lqc2(22, r32(c, V0));
                            c.ctc2(V1, 14);
                            c.vcallms(0xef0);
                            c.cfc2(V1, 1);
                            c.g[V1][0] == c.g[T4][0]
                        };
                        if culled {
                            None
                        } else {
                            c.cycles += 7;
                            Some((EMITTER_COUNT, T1))
                        }
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[T3][0] {
                            None
                        } else {
                            c.cycles += 6;
                            Some((LIGHT_COUNT, T0))
                        }
                    };
                    if let Some((count, list)) = append {
                        let count = Addr(me.wrapping_add(count));
                        c.g[V1][0] = count.lw(c);
                        c.g[V0][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = sx(r32(c, list).wrapping_add(r32(c, V0)));
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                        let (slot, fx_word, n) = (Addr(r32(c, V0)), c.g[A3][0], c.g[V1][0]);
                        slot.sw(c, fx_word);
                        count.sw(c, n);
                        c.cycles += 1;
                    }
                    c.g[A3][0] = Addr(fx).lw(c);
                    c.cycles += 2;
                    if c.g[A3][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, A3).wrapping_add(FX_KIND)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_a7a8);
                    }
                }
            }
            c.cycles += 2;
            let more = (c.g[A2][0] as i64) > 0;
            c.g[A1][0] = c.g[T2][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_a778);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, fill, put_words, scramble, Rng, DATA};

    /// PATH_SetTrackControlValue: player on/off, a map already selected or
    /// not, values below and above 0x80, random masks (sometimes all ones)
    /// against four maps with random ids and loaded flags, each with 24
    /// slots that are empty or point at a random track; the real
    /// `PATH_SelectMap`.
    #[test]
    fn path_set_track_control_value_matches() {
        use super::path_ctl::*;
        check_lohi(
            ee::s14::f_003d0d70__generated,
            path_set_track_control_value,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_0d70 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_0d70;
                let enabled = if seed % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(ENABLED, enabled as u8);
                let id = if seed % 8 == 1 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(CURRENT_ID, id);
                c.bus.write32(CURRENT_MAP, r.next_u32());
                let mask = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                };
                c.g[A0][0] = mask;
                c.g[A1][0] = if seed % 5 == 0 {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                } else {
                    (r.next_u32() % 0x80) as u64
                };
                for m in 0..4u32 {
                    let map = MAP0_ID - 0x30 + m * MAP_SIZE as u32;
                    c.bus.write32(map + 0x30, r.next_u32());
                    c.bus
                        .write32(map + 0x34, if r.next_u32() % 3 == 0 { 0 } else { 1 });
                    for j in 0..24u32 {
                        let track = DATA + (m * 24 + j) * 0x10;
                        fill(c, &mut r, track, 4);
                        let p = if r.next_u32() % 4 == 0 { 0 } else { track };
                        c.bus.write32(map + SLOT0 + 4 * j, p);
                    }
                }
            },
        );
    }

    /// `EIntr` with interrupts on and off (random other Status bits).
    #[test]
    fn e_intr_matches() {
        crate::difftest::check(ee::s15::f_0042c0c0__generated, e_intr, 64, &|c, seed| {
            let mut r = Rng(0x42c0_c000 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let status = r.next_u32() & !(STATUS_EIE as u32);
            c.cop0[12] = if seed % 2 == 0 {
                status
            } else {
                status | STATUS_EIE as u32
            };
            c.pc = 0x0042_c0c0;
        });
    }

    /// `LensFX_PrepareView` for views 0 and 1: the camera interface's view
    /// rect / matrix / FOV slots and the projector's occlusion slot are
    /// fakes (the matrix call returns a random matrix; occlusion is always
    /// reported, so the real `LensFX_ProjectSource` takes its short path);
    /// the painter's sun texture is -1 (the real `LensFX_UpdateSunPosition`
    /// hides the sun); zero, negative and 1-4 flare sources; unwinds in the
    /// first four calls.
    #[test]
    fn lens_fx_prepare_view_matches() {
        use super::lens_prep::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::GAME_GP;
        use crate::game::visualfx::lensfx::tests::painter_sun;
        covered(
            ee::s09::f_002e3338__generated,
            lens_fx_prepare_view,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002e_3338, 4);
                c.pc = 0x002e_3338;
                let fx = DATA + 0x4_0000;
                let cam = DATA + 0x5_0000;
                let vt = DATA + 0x5_1000;
                let no_sun = DATA + 0x5_4000;
                let src = DATA + 0x5_5000;
                let matrix = DATA + 0x5_6000;
                let view = seed % 2;
                c.g[A0][0] = sx(fx);
                c.g[A1][0] = view as u64;
                // The camera and its interface.
                c.bus.write32(GAME_GP + CAMERA_GP, cam);
                c.bus.write32(cam + CAM_VT, vt);
                for (i, off) in [VT_VIEW_RECT, VT_VIEW_MATRIX, VT_FOV, 0x2d8]
                    .into_iter()
                    .enumerate()
                {
                    let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
                }
                fill(c, &mut r, matrix, 16);
                c.bus.write32(LOG_RESULTS + 4, matrix);
                // Painter views 6 and 7: sun texture -1.
                for v in [6, 7] {
                    painter_sun(c, &mut r, v, DATA + 0x5_2000, DATA + 0x5_3000, &[0x108]);
                }
                c.bus.write32(no_sun, u32::MAX);
                c.bus.write32(LOG_RESULTS + 12, no_sun);
                // The lens FX: header, counts, flares pointing at one source.
                fill(c, &mut r, fx, 0x18);
                fill(c, &mut r, src, 0x10);
                let count = match seed % 4 {
                    0 => 0,
                    1 => (r.next_u32() % 5 + 1).wrapping_neg(),
                    _ => 1 + r.next_u32() % 4,
                };
                c.bus.write32(fx + COUNTS + 4 * view, count);
                for k in 0..4 {
                    let flare = fx + FLARES + view * FLARE_VIEW_SIZE as u32 + k * FLARE_SIZE;
                    fill(c, &mut r, flare, 0xc);
                    c.bus.write32(flare + 0x20, src);
                }
            },
        );
    }

    /// `ComputerActorRider_SetCruiseCrouch`: rider velocities below and
    /// above the fast threshold, speeds below and above 20, the real
    /// `.lit4` constants; a random pad word (the low 26 bits kept).
    #[test]
    fn computer_actor_rider_set_cruise_crouch_matches() {
        use super::cruise::*;
        use crate::difftest::GAME_GP;
        crate::difftest::check(
            ee::s00::f_0010c0a8__generated,
            computer_actor_rider_set_cruise_crouch,
            300,
            &|c, seed| {
                let mut r = Rng(0x0010_c0a8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0010_c0a8;
                c.g[GP][0] = GAME_GP as u64;
                let (me, rider, pad) = (DATA, DATA + 0x1000, DATA + 0x2000);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(pad);
                c.bus.write32(GAME_GP - LIT_FAST, 833.333_4f32.to_bits());
                c.bus.write32(GAME_GP - LIT_CROUCH, 0.97f32.to_bits());
                c.bus.write32(me + RIDER, rider);
                let v = if seed % 2 == 0 { 400.0 } else { 1200.0 };
                for i in 0..4 {
                    c.bus.write32(rider + RIDER_VELOCITY + 4 * i, r.f32(v));
                }
                let speed = if seed % 3 == 0 {
                    r.f32(20.0)
                } else {
                    r.f32(100.0)
                };
                c.bus
                    .write32(me + SPEED, if seed % 5 == 0 { r.edge_f32() } else { speed });
                c.bus.write32(pad, r.next_u32());
            },
        );
    }

    /// `SNDSTRM_RunCallbacks`: zero, negative and 1-6 queued events of both
    /// kinds on random streams (ids 0-7) whose callbacks are fakes (some
    /// unwind).
    #[test]
    fn sndstrm_run_callbacks_matches() {
        use super::sndstrm_cb::*;
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s13::f_003c7080__generated,
            sndstrm_run_callbacks,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_7080, 4);
                c.pc = 0x003c_7080;
                let count = match seed % 5 {
                    0 => 0,
                    1 => (r.next_u32() % 3 + 1).wrapping_neg(),
                    _ => 1 + r.next_u32() % 6,
                };
                c.bus.write32(COUNT, count);
                for i in 0..8u32 {
                    let stream = DATA + 0x100 * i;
                    c.bus.write32(STREAMS + 4 * i, stream);
                    c.bus.write32(stream + VALUE_CB, FAKE + 0x10);
                    c.bus.write32(stream + STREAM_CB, FAKE + 0x20);
                    c.bus.write32(stream + USER, r.next_u32());
                }
                for i in 0..6u32 {
                    let ev = EVENTS + EVENT_SIZE * i;
                    let kind = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 0xffff
                    };
                    c.bus.write16(ev + EV_KIND, kind as u16);
                    c.bus.write16(ev + EV_STREAM, (r.next_u32() % 8) as u16);
                    c.bus.write32(ev + EV_VALUE, r.next_u32());
                }
            },
        );
    }

    /// `SND_RestoreVu0Flags` with random status words (every combination of
    /// the 0x8800 / 8 / 0x80 bits over the four lanes) and random VF1/VF2.
    #[test]
    fn snd_restore_vu0_flags_matches() {
        crate::difftest::check(
            ee::s13::f_003c07a8__generated,
            snd_restore_vu0_flags,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_07a8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_07a8;
                c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    /// `cUsbKeyboard_GetKey`: open and closed keyboards, channels in and
    /// out of range (negative too) of the real `UsbKb_GetMaxKeyboards`,
    /// empty and non-empty queues with negative and large read indices,
    /// zero and non-zero key / modifier bytes.
    #[test]
    fn c_usb_keyboard_get_key_matches() {
        use super::usb_kb::*;
        crate::difftest::check_lohi(
            ee::s07::f_00266ba8__generated,
            c_usb_keyboard_get_key,
            300,
            &|c, seed| {
                let mut r = Rng(0x0026_6ba8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0026_6ba8;
                let (kb, key, modifier) = (DATA, DATA + 0x800, DATA + 0x810);
                let max = 1 + r.next_u32() % 4;
                c.bus.write32(0x0052_6010, max);
                c.bus.write32(
                    kb + ACTIVE,
                    if seed % 6 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let ch = match seed % 7 {
                    0 => max + r.next_u32() % 3,
                    1 => (r.next_u32() % 3 + 1).wrapping_neg(),
                    _ => r.next_u32() % max,
                };
                c.g[A0][0] = sx(kb);
                c.g[A1][0] = sx(ch);
                c.g[A2][0] = sx(key);
                c.g[A3][0] = sx(modifier);
                for i in 0..4u32 {
                    let chan = kb + i * CHANNEL_SIZE as u32;
                    let read = match r.next_u32() % 4 {
                        0 => (r.next_u32() % 200).wrapping_neg(),
                        1 => r.next_u32() % 1000,
                        _ => r.next_u32() % 64,
                    };
                    let count = if r.next_u32() % 4 == 0 {
                        read
                    } else {
                        read.wrapping_add(1 + r.next_u32() % 5)
                    };
                    c.bus.write32(chan + COUNT, count);
                    c.bus.write32(chan + READ, read);
                    for b in 0..0x80u32 {
                        let v = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            r.next_u32() as u8
                        };
                        c.bus.write8(chan + RING + b, v);
                    }
                }
            },
        );
    }

    /// `HUD_DrawPanelSegment`: heights below and above 72 (4 or 8
    /// vertices), random colours, position and texture coordinates; the
    /// renderer's strip slot is a fake (some calls unwind).
    #[test]
    fn hud_draw_panel_segment_matches() {
        use super::hud_seg::*;
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::GAME_GP;
        covered(
            ee::s05::f_0021ea00__generated,
            hud_draw_panel_segment,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0021_ea00, 1);
                c.pc = 0x0021_ea00;
                let (colour, pos, renderer, vt) = (DATA, DATA + 0x20, DATA + 0x1000, DATA + 0x3000);
                c.bus.write32(GAME_GP - RENDERER_GP, renderer);
                c.bus.write32(renderer + RENDERER_VT, vt);
                let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                c.bus.write16(vt + VT_DRAW_STRIP, delta as u16);
                c.bus.write32(vt + VT_DRAW_STRIP + 4, FAKE);
                for i in 0..4 {
                    let ch = (r.next_u32() % 1000) as f32 / 500.0;
                    c.bus.write32(colour + 4 * i, ch.to_bits());
                }
                c.bus.write32(pos, r.f32(640.0));
                c.bus.write32(pos + 4, r.f32(480.0));
                c.g[A1][0] = sx(colour);
                c.g[A2][0] = sx(pos);
                c.f[12] = ((r.next_u32() % 400) as f32 / 2.0).to_bits();
                c.f[13] = match seed % 4 {
                    0 => 72.0f32.to_bits(),
                    1 => ((r.next_u32() % 72) as f32).to_bits(),
                    _ => (72.5 + (r.next_u32() % 300) as f32).to_bits(),
                };
                for f in 14..18 {
                    c.f[f] = r.f32(1.0);
                }
            },
        );
    }

    /// `RiderWipeoutState_UpdateRootPose`: active and inactive riders,
    /// tumble speeds below and above the 3333.33 clamp, random angular
    /// velocities, root positions and quaternions, rates around 1; the
    /// real `.lit4` constants.
    #[test]
    fn rider_wipeout_state_update_root_pose_matches() {
        use super::wipeout_pose::*;
        use crate::difftest::GAME_GP;
        crate::difftest::check(
            ee::s01::f_00136f30__generated,
            rider_wipeout_state_update_root_pose,
            300,
            &|c, seed| {
                let mut r = Rng(0x0013_6f30 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0013_6f30;
                c.g[GP][0] = GAME_GP as u64;
                put_words(
                    c,
                    GAME_GP - LIT_TICK,
                    &[
                        (1.0f32 / 60.0).to_bits(),
                        (-0.2f32).to_bits(),
                        3333.3335f32.to_bits(),
                    ],
                );
                let (me, rider) = (DATA, DATA + 0x1000);
                c.g[A0][0] = sx(me);
                c.bus.write32(me + RIDER, rider);
                c.bus.write32(
                    rider + R_ACTIVE,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let speed = if seed % 2 == 0 { 2000.0 } else { 6000.0 };
                for i in 0..4 {
                    c.bus.write32(me + VEL + 4 * i, r.f32(speed));
                    c.bus.write32(me + ANG_VEL + 4 * i, r.f32(10.0));
                    c.bus.write32(rider + R_POS + 4 * i, r.f32(5000.0));
                    c.bus.write32(rider + R_QUAT + 4 * i, r.f32(1.0));
                }
                c.bus.write32(
                    rider + R_RATE,
                    (0.5 + (r.next_u32() % 100) as f32 / 50.0).to_bits(),
                );
            },
        );
    }

    /// `Nash_CollectEffects`: zero, negative and 1-4 visible nodes (clipped
    /// or not), each with up to 5 effects of kind 7 (emitter), 8 (light) or
    /// other; the test sphere microprogram culls clip word 2.
    #[test]
    fn nash_collect_effects_matches() {
        use super::nash_fx::*;
        use crate::game::main::ssxapp::tests::put_cull_microprogram;
        crate::difftest::check(
            ee::s05::f_0022a770__generated,
            nash_collect_effects,
            300,
            &|c, seed| {
                let mut r = Rng(0x0022_a770 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                put_cull_microprogram(c);
                c.pc = 0x0022_a770;
                let (me, pairs) = (DATA, DATA + 0x9000);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(pairs);
                c.g[A2][0] = match seed % 6 {
                    0 => 0,
                    1 => sx((r.next_u32() % 3 + 1).wrapping_neg()),
                    _ => (1 + r.next_u32() % 4) as u64,
                };
                c.bus.write32(me + EMITTER_COUNT, r.next_u32() % 16);
                c.bus.write32(me + LIGHT_COUNT, r.next_u32() % 16);
                let mut fx = DATA + 0xa000;
                for p in 0..4u32 {
                    let node = DATA + 0x9100 + 0x40 * p;
                    let pair = pairs + PAIR_SIZE * p;
                    c.bus.write32(pair, node);
                    let clip = match r.next_u32() % 3 {
                        0 => 0,
                        1 => OUTSIDE as u32,
                        _ => r.next_u32() % 4,
                    };
                    c.bus.write32(pair + PAIR_CLIP, clip);
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 6 {
                        let kind = match r.next_u32() % 4 {
                            0 | 1 => KIND_EMITTER as u32,
                            2 => KIND_LIGHT as u32,
                            _ => r.next_u32() % 16,
                        };
                        c.bus.write32(fx, next);
                        c.bus.write32(fx + FX_KIND, kind);
                        fill(c, &mut r, fx + FX_SPHERE, 4);
                        next = fx;
                        fx += 0x60;
                    }
                    c.bus.write32(node + NODE_EFFECTS, next);
                }
            },
        );
    }
}
