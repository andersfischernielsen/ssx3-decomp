//! Work package D1: the rider's per-frame "doors" (the small functions
//! `AI_Update` / `AI_Render` call once per rider per frame) and the state
//! machine's thin handlers (`docs/investigation/phase2-candidates-2.md`, D1;
//! field names from `docs/investigation/structs/rider.md`).
//!
//! A rider is two objects: the gameplay `Rider` (physics, animation, FX;
//! vptr at +0x6c0) and its owner, the `RiderBase` (`Rider+0x77c`), which runs
//! the two-level state machine: a **main state** at +0xde0 (Ground, Air,
//! Wipeout, Static, Railslide, Handplant) and a **sub state** at +0xde4
//! (Cruise, BoardPress, Prewind, ... FinishLine). The state objects live
//! inside the `RiderBase` (Wipeout at +0x30, Handplant at +0x110, the Cruise
//! sub state at +0x1c0, BoardPress at +0x1d0, Spin at +0x230, Reset at +0x350).
//!
//! Every replacement follows the call protocol of [`crate::call`]: the same
//! stack frame, the registers the original leaves at each call, the unwind
//! check after it, and the EE cycles charged block by block. Incidental
//! register values the original leaves behind (`v0`/`v1` scratch, FPU
//! leftovers, the FPU condition bit) are reproduced in marked lines.

use crate::call::{call_ptr, guest, tail_wrapper, Callee, Frame, Target, VSlot};
use crate::game::ai::rider::rate_step_60;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_eb60, rider_tick_animation, "Rider_TickAnimation"),
    (0x0012_1700, rider_update_animation, "Rider_UpdateAnimation"),
    (0x0012_1728, rider_update_pose, "Rider_UpdatePose"),
    (0x0012_17f8, rider_update_model, "Rider_UpdateModel"),
    (0x0012_25f0, rider_render_shadow, "Rider_RenderShadow"),
    (0x0012_25c0, rider_prepare_lighting, "Rider_PrepareLighting"),
    (
        0x0012_2638,
        rider_render_attached_fx,
        "Rider_RenderAttachedFX",
    ),
    (
        0x0012_0e88,
        rider_update_attached_fx,
        "Rider_UpdateAttachedFX",
    ),
    (
        0x0012_18d0,
        rider_update_surface_fx,
        "Rider_UpdateSurfaceFX",
    ),
    (0x0012_1818, rider_update_progress, "Rider_UpdateProgress"),
    (
        0x0011_12b8,
        rider_base_set_main_state,
        "RiderBase_SetMainState",
    ),
    (
        0x0011_1538,
        rider_base_set_sub_state,
        "RiderBase_SetSubState",
    ),
    (
        0x0013_6e98,
        rider_wipeout_state_update,
        "RiderWipeoutState_Update",
    ),
    (
        0x0013_6ee0,
        rider_wipeout_state_post_update,
        "RiderWipeoutState_PostUpdate",
    ),
    (
        0x0013_9528,
        rider_handplant_state_post_update,
        "RiderHandplantState_PostUpdate",
    ),
    (
        0x0013_1cc0,
        rider_cruise_sub_check_airborne,
        "RiderCruiseSub_CheckAirborne",
    ),
    (0x0012_31a8, rider_is_on_ground, "Rider_IsOnGround"),
    (
        0x0011_6378,
        rider_try_enter_finish_line,
        "Rider_TryEnterFinishLine",
    ),
    (0x0011_6120, rider_check_reset_path, "Rider_CheckResetPath"),
    (
        0x0011_62c8,
        rider_try_enter_prewind,
        "Rider_TryEnterPrewind",
    ),
    (0x0012_0ed8, rider_refresh_pose, "Rider_RefreshPose"),
    (
        0x0012_2088,
        rider_find_nearby_lights,
        "Rider_FindNearbyLights",
    ),
    (
        0x0012_2658,
        rider_clear_nearby_lights,
        "Rider_ClearNearbyLights",
    ),
    (
        0x0012_0090,
        rider_set_anim_time_scale,
        "Rider_SetAnimTimeScale",
    ),
    (
        0x0013_60c8,
        rider_spin_sub_get_hud_angle,
        "RiderSpinSub_GetHUDAngle",
    ),
];

// ---------------------------------------------------------------------------
// State ids (`RiderBase+0xde0` / `+0xde4`).

/// Main states.
#[allow(dead_code)]
pub mod main_state {
    pub const GROUND: u64 = 0;
    pub const AIR: u64 = 1;
    pub const WIPEOUT: u64 = 2;
    pub const STATIC: u64 = 3;
    pub const RAILSLIDE: u64 = 4;
    pub const HANDPLANT: u64 = 5;
}

/// Sub states.
#[allow(dead_code)]
pub mod sub_state {
    pub const CRUISE: u64 = 0;
    pub const BOARD_PRESS: u64 = 1;
    pub const PREWIND: u64 = 2;
    pub const NATURAL_AIR: u64 = 4;
    pub const RESET: u64 = 9;
    pub const FINISH_LINE: u64 = 0xa;
}

// ---------------------------------------------------------------------------
// Fields.

/// `Rider` field offsets.
mod rider {
    /// `+0x2e8 powerUpBoostTimer`, `+0x2ec powerUpTrickTimer` (seconds).
    pub const POWER_UP_BOOST_TIMER: u32 = 0x2e8;
    pub const POWER_UP_TRICK_TIMER: u32 = 0x2ec;
    /// `+0x300 timeScale`: rubber-band time scale, also the animation dt.
    pub const TIME_SCALE: u32 = 0x300;
    /// `+0x360 jumpRearm`: 1.0 while the jump button is up.
    pub const JUMP_REARM: u32 = 0x360;
    /// `+0x430 groundPolyId` (-1 none), `+0x434 groundMaterial`.
    pub const GROUND_POLY_ID: u32 = 0x430;
    pub const GROUND_MATERIAL: u32 = 0x434;
    /// `+0x460` ground contact point (x, y).
    pub const GROUND_CONTACT: u32 = 0x460;
    /// `+0x470 finishTimer`: -1 while racing, >= 0 after the finish line.
    pub const FINISH_TIMER: u32 = 0x470;
    /// `+0x6c0` the `Rider` vptr.
    pub const VPTR: u32 = 0x6c0;
    /// `+0x77c owner` (`RiderBase*`).
    pub const OWNER: u32 = 0x77c;
    /// `+0x780 model` (`RiderModel*`), `+0x784 anim` (`AnimController*`).
    pub const MODEL: u32 = 0x780;
    pub const ANIM: u32 = 0x784;
    /// `+0x790 metrix` (`RiderMetrix*`).
    pub const METRIX: u32 = 0x790;
    /// `+0x794` the eight nearest light handles.
    pub const LIGHTS: u32 = 0x794;
    /// `+0x860` the overlap list the light search uses.
    pub const OVERLAP_LIST: u32 = 0x860;
    /// `+0x86c index`: rider slot 0..5.
    pub const INDEX: u32 = 0x86c;
    /// `+0x88c AttachedEmitterFX*`, `+0x890 cBoltOnFXInterface*`.
    pub const ATTACHED_FX: u32 = 0x88c;
    pub const BOLT_ON_FX: u32 = 0x890;
    /// `+0x9e0` / `+0xa60`: the records passed to object triggers.
    pub const TRIGGER_A: u32 = 0x9e0;
    pub const TRIGGER_B: u32 = 0xa60;
    /// `+0xa30` the object the rider is attached to (grind object).
    pub const ATTACHED_OBJ: u32 = 0xa30;
    /// `+0xac4 isPuppet`: replay/script control, skips physics and anim.
    pub const IS_PUPPET: u32 = 0xac4;
    /// `+0xb18` gates the shadow and lighting (unmapped; "visible/loaded").
    pub const SHADOW_FLAG: u32 = 0xb18;
    /// `+0xb1c` the flag handed to the animation controller (unmapped).
    pub const ANIM_FLAG: u32 = 0xb1c;
}

/// `RiderBase` field offsets.
mod base {
    /// The Wipeout state's first word: 0 on the ground, else airborne.
    pub const WIPEOUT_AIRBORNE: u32 = 0x30;
    /// The BoardPress sub state object.
    pub const BOARD_PRESS_SUB: u32 = 0x1d0;
    /// The Reset sub state object's first word (`reason != 0`).
    pub const RESET_SUB_FLAG: u32 = 0x350;
    /// `cCheatCodeDetector`.
    pub const CHEAT_DETECTOR: u32 = 0xd20;
    /// The emitter object of the trick power-up FX.
    pub const POWER_UP_EMITTER: u32 = 0xd30;
    pub const MAIN_STATE: u32 = 0xde0;
    pub const SUB_STATE: u32 = 0xde4;
}

/// gp offsets.
mod gp {
    /// `g_App` (`SSXApp*`, `gp-0x848`); `+0x84` is the `GameModule`.
    pub const APP_NEG: u32 = 0x848;
    /// The `WScriptMan` instance (`gp+0xce8`).
    pub const WSCRIPT_MAN: u32 = 0xce8;
    /// `-99999.0`: "no height" for `WorldPainter_UpdateView` (`gp-0x7910`).
    pub const NO_HEIGHT_NEG: u32 = 0x7910;
    /// The time-scale slew step 0.05/60 (`gp-0x795c`).
    pub const TIME_SCALE_STEP_NEG: u32 = 0x795c;
    /// `1e30`: "stick centred" (`gp-0x7308`).
    pub const STICK_CENTRED_NEG: u32 = 0x7308;
}

/// `GameModule` field offsets.
mod game_module {
    pub const OF_APP: u32 = 0x84;
    pub const REPLAY: u32 = 0x28;
    pub const SECTION_MAN: u32 = 0x78;
    pub const LIGHTS: u32 = 0x90;
}

/// `lw base+off` (sign-extended, through the CPU load path).
#[inline(always)]
fn lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `addiu rd, base, off` on a pointer (sign-extended 32-bit result).
#[inline(always)]
fn ptr_add(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `gp + off` / `gp - off`.
#[inline(always)]
fn gp_minus(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}
#[inline(always)]
fn gp_plus(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// `c.le.s a, b` with the FCC bit latched.
#[inline(always)]
fn c_le_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    crate::abi::set_fcc(c, r);
    r
}

/// `c.eq.s a, b` with the FCC bit latched.
#[inline(always)]
fn c_eq_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.eq_s(b);
    crate::abi::set_fcc(c, r);
    r
}

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const RIDER_TICK_ANIMATION: Callee = Callee {
        addr: 0x0011_eb60,
        f: ee::s00::Rider_TickAnimation,
    };
    pub const RIDER_UPDATE_SKELETON: Callee = Callee {
        addr: 0x0011_eb98,
        f: ee::s00::Rider_UpdateSkeleton,
    };
    pub const RIDER_DRAW_SHADOW: Callee = Callee {
        addr: 0x0012_2898,
        f: ee::s00::Rider_DrawShadow,
    };
    pub const RIDER_BUILD_LIGHT_SET: Callee = Callee {
        addr: 0x0012_20d8,
        f: ee::s00::Rider_BuildLightSet,
    };
    pub const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
        addr: 0x0011_e098,
        f: ee::s00::Rider_BuildWorldMatrix,
    };
    pub const RIDER_GET_HIPS_POSITION: Callee = Callee {
        addr: 0x0011_ff48,
        f: ee::s00::Rider_GetHipsPosition,
    };
    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    pub const RIDER_GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };
    pub const RIDER_SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::Rider_SetMainState,
    };
    pub const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    pub const RIDER_BASE_EXIT_MAIN_STATE: Callee = Callee {
        addr: 0x0011_12f8,
        f: ee::s00::RiderBase_ExitMainState,
    };
    pub const RIDER_BASE_ENTER_MAIN_STATE: Callee = Callee {
        addr: 0x0011_1380,
        f: ee::s00::RiderBase_EnterMainState,
    };
    pub const RIDER_BASE_EXIT_SUB_STATE: Callee = Callee {
        addr: 0x0011_1578,
        f: ee::s00::RiderBase_ExitSubState,
    };
    pub const RIDER_BASE_ENTER_SUB_STATE: Callee = Callee {
        addr: 0x0011_1630,
        f: ee::s00::RiderBase_EnterSubState,
    };
    pub const RIDER_METRIX_SHOW_WRONG_WAY: Callee = Callee {
        addr: 0x0011_a088,
        f: ee::s00::RiderMetrix_ShowWrongWay,
    };
    /// The BoardPress sub state's release (adds the press boost).
    pub const RIDER_BOARD_PRESS_SUB_RELEASE: Callee = Callee {
        addr: 0x0013_1348,
        f: ee::s00::f_00131348,
    };
    /// An empty per-frame hook (`jr ra`).
    pub const RIDER_PROGRESS_HOOK: Callee = Callee {
        addr: 0x0012_5ad0,
        f: ee::s00::f_00125ad0,
    };
    pub const RIDER_UPDATE_RACE_PROGRESS: Callee = Callee {
        addr: 0x0011_2338,
        f: ee::s00::Rider_UpdateRaceProgress,
    };
    pub const RIDER_UPDATE_AI_PATH_PROGRESS: Callee = Callee {
        addr: 0x0011_25c0,
        f: ee::s00::Rider_UpdateAIPathProgress,
    };
    pub const RIDER_CHECK_RACE_FINISH: Callee = Callee {
        addr: 0x0012_5228,
        f: ee::s00::Rider_CheckRaceFinish,
    };
    pub const RIDER_METRIX_UPDATE: Callee = Callee {
        addr: 0x0011_7c28,
        f: ee::s00::RiderMetrix_Update,
    };
    pub const RIDER_UPDATE_FOCUS_BOX: Callee = Callee {
        addr: 0x0012_0e30,
        f: ee::s00::Rider_UpdateFocusBox,
    };
    pub const WIPEOUT_UPDATE_ROOT_POSE: Callee = Callee {
        addr: 0x0013_6f30,
        f: ee::s01::RiderWipeoutState_UpdateRootPose,
    };
    pub const WIPEOUT_UPDATE_GROUND: Callee = Callee {
        addr: 0x0013_7d18,
        f: ee::s01::RiderWipeoutState_UpdateGround,
    };
    pub const WIPEOUT_UPDATE_AIR: Callee = Callee {
        addr: 0x0013_7750,
        f: ee::s01::RiderWipeoutState_UpdateAir,
    };
    pub const WIPEOUT_POST_UPDATE_ROOT_POSE: Callee = Callee {
        addr: 0x0013_7138,
        f: ee::s01::RiderWipeoutState_PostUpdateRootPose,
    };
    pub const WIPEOUT_POST_UPDATE_GROUND: Callee = Callee {
        addr: 0x0013_8640,
        f: ee::s01::RiderWipeoutState_PostUpdateGround,
    };
    pub const WIPEOUT_POST_UPDATE_AIR: Callee = Callee {
        addr: 0x0013_7860,
        f: ee::s01::RiderWipeoutState_PostUpdateAir,
    };
}

/// Cross-shard direct calls the compiler emitted as `call_indirect`.
mod table {
    pub const ANIM_CONTROLLER_UPDATE: u32 = 0x0031_23c0;
    pub const ANIM_CONTROLLER_DISPATCH_END_EVENTS: u32 = 0x0031_2490;
    pub const RIDER_MODEL_BUILD_BONE_MATRICES: u32 = 0x0031_03f0;
    pub const ATTACHED_EMITTER_FX_RENDER: u32 = 0x002f_2088;
    pub const ATTACHED_EMITTER_FX_UPDATE: u32 = 0x002f_1a08;
    pub const BOLT_ON_FX_UPDATE: u32 = 0x002e_cf78;
    pub const SECTION_MAN_FIND_LOGICAL_SECTION: u32 = 0x0022_e0e0;
    pub const WORLD_PAINTER_UPDATE_VIEW: u32 = 0x002e_d490;
    /// Clears the power-up emitter's state.
    pub const EMITTER_RESET: u32 = 0x0039_2d18;
    pub const WSCRIPT_MAN_ON_OBJECT_TRIGGER: u32 = 0x0030_a060;
    pub const RIDER_UPDATE_BOUNDS: u32 = 0x0011_e150;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const SSX_AUDIO_SYSTEM_ON_RIDER_RESET: u32 = 0x0029_a220;
    pub const REPLAY_ON_RIDER_RESET: u32 = 0x0027_0970;
    pub const CHEAT_CODE_DETECTOR_POST_EVENT: u32 = 0x002f_6ac8;
    pub const WORLD_LIGHT_MAN_FIND_STRONGEST_LIGHTS: u32 = 0x002f_5b68;
    pub const EA_MEM_SET: u32 = 0x003e_6448;
}

// ---------------------------------------------------------------------------
// Templates.

/// The 0x10-frame "door" gcc emits for `if (flag) call(this)`: `+5` (load
/// the flag), `+3` on the call arm, then `tail` cycles (2, or 3 when the
/// door returns `true` in v0). `call_when_set` picks the polarity; `prepare`
/// sets extra argument registers on the call arm.
fn guarded_door(
    c: &mut Cpu,
    flag: u32,
    call_when_set: bool,
    prepare: impl FnOnce(&mut Cpu),
    target: Callee,
    ret: u32,
    tail: u64,
    returns_true: bool,
) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        let v = lw(c, this, flag);
        c.g[V0][0] = v;
        frame.restore_ra(c, 0);
        if (v != 0) == call_when_set {
            c.cycles += 3;
            prepare(c);
            target.call(c, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += tail;
        if returns_true {
            c.g[V0][0] = 1;
        }
        frame.close(c);
        Ok(())
    })
}

/// `RiderBase::SetMainState`/`SetSubState`: `Exit(this, new)`, swap the
/// state word, `Enter(this, old)`. One 16-cycle block, frame 0x30.
fn set_state(c: &mut Cpu, field: u32, exit: Callee, enter: Callee, rets: [u32; 2]) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let new = c.g[A1][0];
        c.g[S1][0] = new;
        exit.call(c, rets[0])?;
        let old = lw(c, this, field);
        c.g[A1][0] = old;
        c.g[A0][0] = this;
        Addr((this as u32).wrapping_add(field)).sw(c, new);
        enter.call(c, rets[1])?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The Wipeout state's Update/PostUpdate shape: `root(s)`, then `ground(s)`
/// while the wipeout's first word says "on the ground", else `air(s)`.
/// Blocks `+8`, `+4` (ground) or `+3` (air), `+3`; frame 0x20.
fn wipeout_dispatch(c: &mut Cpu, root: Callee, ground: Callee, air: Callee, rets: [u32; 3]) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let state = c.g[A0][0];
        c.g[S0][0] = state;
        root.call(c, rets[0])?;
        let airborne = lw(c, state, 0);
        c.g[V0][0] = airborne;
        c.g[A0][0] = state;
        if airborne == 0 {
            c.cycles += 4;
            ground.call(c, rets[1])?;
        } else {
            c.cycles += 3;
            air.call(c, rets[2])?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared prologue of the 0x20 frames holding `ra` at +0 and `s0 = this`
/// at +0x10. Returns `this`.
fn open_s0_frame(c: &mut Cpu) -> (Frame, u64) {
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let this = c.g[A0][0];
    c.g[S0][0] = this;
    (frame, this)
}

/// Epilogue matching [`open_s0_frame`].
fn close_s0_frame(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Per-frame doors (AI_Update / AI_Render).

/// `Rider_TickAnimation(rider, dt /*f12*/)` (0x11eb60): advance the
/// animation controller by `dt` (`f12` passes through untouched) and
/// dispatch its end-of-animation events. 13 cycles.
pub fn rider_tick_animation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let (frame, rider) = open_s0_frame(c);
        c.g[A1][0] = lw(c, rider, rider::ANIM_FLAG);
        c.g[A0][0] = lw(c, rider, rider::ANIM);
        call_ptr(c, table::ANIM_CONTROLLER_UPDATE, 0x0011_eb7c)?;
        c.g[A0][0] = lw(c, rider, rider::ANIM);
        call_ptr(c, table::ANIM_CONTROLLER_DISPATCH_END_EVENTS, 0x0011_eb84)?;
        close_s0_frame(c, frame);
        Ok(())
    })
}

/// `Rider_UpdateAnimation(rider)` (0x121700): unless the rider is a puppet,
/// tick its animation by its time scale.
pub fn rider_update_animation(c: &mut Cpu) {
    let prepare = |c: &mut Cpu| {
        let rider = c.g[A0][0];
        c.f[12] = Addr((rider as u32).wrapping_add(rider::TIME_SCALE)).read_u32(c);
    };
    guarded_door(
        c,
        rider::IS_PUPPET,
        false,
        prepare,
        callee::RIDER_TICK_ANIMATION,
        0x0012_171c,
        2,
        false,
    )
}

/// `Rider_UpdatePose(rider)` (0x121728): unless the rider is a puppet,
/// evaluate its skeleton.
pub fn rider_update_pose(c: &mut Cpu) {
    guarded_door(
        c,
        rider::IS_PUPPET,
        false,
        |_| {},
        callee::RIDER_UPDATE_SKELETON,
        0x0012_1744,
        2,
        false,
    )
}

/// `Rider_RenderShadow(rider)` (0x1225f0): draw the shadow when
/// `+0xb18` is set.
pub fn rider_render_shadow(c: &mut Cpu) {
    guarded_door(
        c,
        rider::SHADOW_FLAG,
        true,
        |_| {},
        callee::RIDER_DRAW_SHADOW,
        0x0012_260c,
        2,
        false,
    )
}

/// `Rider_PrepareLighting(rider)` (0x1225c0): build the light set when
/// `+0xb18` is set. Returns 1.
pub fn rider_prepare_lighting(c: &mut Cpu) {
    guarded_door(
        c,
        rider::SHADOW_FLAG,
        true,
        |_| {},
        callee::RIDER_BUILD_LIGHT_SET,
        0x0012_25dc,
        3,
        true,
    )
}

/// `Rider_UpdateModel(rider)` (0x1217f8):
/// `RiderModel_BuildBoneMatrices(rider->model)`.
pub fn rider_update_model(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(rider::MODEL),
        Target::Table(table::RIDER_MODEL_BUILD_BONE_MATRICES),
        0x0012_1808,
    )
}

/// `Rider_RenderAttachedFX(rider)` (0x122638):
/// `AttachedEmitterFX_Render(rider->attachedFx)`.
pub fn rider_render_attached_fx(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(rider::ATTACHED_FX),
        Target::Table(table::ATTACHED_EMITTER_FX_RENDER),
        0x0012_2648,
    )
}

/// `Rider_UpdateAttachedFX(rider)` (0x120e88): update the attached emitter
/// FX, then the bolt-on FX. 12 cycles.
pub fn rider_update_attached_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let (frame, rider) = open_s0_frame(c);
        c.g[A0][0] = lw(c, rider, rider::ATTACHED_FX);
        call_ptr(c, table::ATTACHED_EMITTER_FX_UPDATE, 0x0012_0ea0)?;
        c.g[A0][0] = lw(c, rider, rider::BOLT_ON_FX);
        call_ptr(c, table::BOLT_ON_FX_UPDATE, 0x0012_0ea8)?;
        close_s0_frame(c, frame);
        Ok(())
    })
}

/// `Rider_UpdateSurfaceFX(rider)` (0x1218d0): look up the material of the
/// ground polygon (when there is one), point the world painter at the
/// ground contact for this rider's slot, and reset the power-up emitter.
pub fn rider_update_surface_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let (frame, rider) = open_s0_frame(c);
        // `nor`: zero exactly when the (sign-extended) id is -1.
        let not_id = !lw(c, rider, rider::GROUND_POLY_ID);
        c.g[V0][0] = not_id;
        c.g[V1][0] = Addr(gp_minus(c, gp::APP_NEG)).lw(c);
        if not_id != 0 {
            c.cycles += 5;
            // The section lookup takes the id's low byte.
            c.g[A1][0] = Addr((rider as u32).wrapping_add(rider::GROUND_POLY_ID)).lbu(c);
            let module = lw(c, c.g[V1][0], game_module::OF_APP);
            c.g[V0][0] = module;
            c.g[A0][0] = lw(c, module, game_module::SECTION_MAN);
            call_ptr(c, table::SECTION_MAN_FIND_LOGICAL_SECTION, 0x0012_1900)?;
            let material = c.g[V0][0];
            Addr((rider as u32).wrapping_add(rider::GROUND_MATERIAL)).sw(c, material);
        }
        c.cycles += 19;
        // index = this->GetIndex() (Rider vtable slot 7).
        let vt = lw(c, rider, rider::VPTR);
        c.g[V1][0] = vt;
        let vbase = ptr_add(rider, rider::VPTR);
        c.g[A1][0] = vbase;
        let m = VSlot::read(c, vt, 0x38);
        c.g[V0][0] = m.func;
        c.g[A0][0] = m.this(vbase);
        call_ptr(c, m.func as u32, 0x0012_191c)?;
        let contact = Addr((rider as u32).wrapping_add(rider::GROUND_CONTACT));
        c.f[12] = contact.read_u32(c);
        c.g[A0][0] = c.g[V0][0];
        c.f[14] = Addr(gp_minus(c, gp::NO_HEIGHT_NEG)).read_u32(c);
        c.f[13] = contact.off(4).read_u32(c);
        call_ptr(c, table::WORLD_PAINTER_UPDATE_VIEW, 0x0012_1930)?;
        let owner = lw(c, rider, rider::OWNER);
        c.g[V0][0] = owner;
        c.g[A0][0] = if owner == 0 {
            0
        } else {
            ptr_add(owner, base::POWER_UP_EMITTER)
        };
        call_ptr(c, table::EMITTER_RESET, 0x0012_1940)?;
        close_s0_frame(c, frame);
        Ok(())
    })
}

/// `Rider_UpdateProgress(rider)` (0x121818): fire the trigger of the object
/// the rider is attached to (its own handler, else the script manager's),
/// then the race bookkeeping: race progress (not for puppets), AI path
/// progress, the finish check, the metrics (not for puppets) and the focus
/// box.
pub fn rider_update_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let (frame, rider) = open_s0_frame(c);
        let obj = lw(c, rider, rider::ATTACHED_OBJ);
        c.g[A0][0] = obj;
        if obj != 0 {
            c.cycles += 3;
            let trigger = lw(c, obj, 0xc);
            c.g[T0][0] = trigger;
            c.g[A1][0] = ptr_add(rider, rider::TRIGGER_B);
            if trigger != 0 {
                // trigger->OnTrigger(&rider->recB, &rider->recA, vbase).
                c.cycles += 9;
                let vt = lw(c, trigger, 0xc);
                c.g[V0][0] = vt;
                c.g[A2][0] = ptr_add(rider, rider::TRIGGER_A);
                c.g[A3][0] = ptr_add(rider, rider::VPTR);
                let m = VSlot::read(c, vt, 0x140);
                c.g[V1][0] = m.func;
                c.g[A0][0] = m.this(trigger);
                call_ptr(c, m.func as u32, 0x0012_185c)?;
            } else {
                c.cycles += 4;
                c.g[A0][0] = Addr(gp_plus(c, gp::WSCRIPT_MAN)).lw(c);
                c.g[A2][0] = ptr_add(rider, rider::TRIGGER_A);
                c.g[A3][0] = ptr_add(rider, rider::VPTR);
                call_ptr(c, table::WSCRIPT_MAN_ON_OBJECT_TRIGGER, 0x0012_1874)?;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = rider;
        callee::RIDER_PROGRESS_HOOK.call(c, 0x0012_187c)?;
        let puppet = lw(c, rider, rider::IS_PUPPET);
        c.g[V0][0] = puppet;
        if puppet == 0 {
            c.cycles += 2;
            c.g[A0][0] = rider;
            callee::RIDER_UPDATE_RACE_PROGRESS.call(c, 0x0012_1890)?;
        }
        c.cycles += 7;
        c.g[A0][0] = rider;
        callee::RIDER_UPDATE_AI_PATH_PROGRESS.call(c, 0x0012_1898)?;
        c.g[A0][0] = rider;
        callee::RIDER_CHECK_RACE_FINISH.call(c, 0x0012_18a0)?;
        let puppet = lw(c, rider, rider::IS_PUPPET);
        c.g[V1][0] = puppet;
        if puppet == 0 {
            c.cycles += 2;
            c.g[A0][0] = lw(c, rider, rider::METRIX);
            callee::RIDER_METRIX_UPDATE.call(c, 0x0012_18b4)?;
        }
        c.cycles += 6;
        c.g[A0][0] = rider;
        callee::RIDER_UPDATE_FOCUS_BOX.call(c, 0x0012_18bc)?;
        close_s0_frame(c, frame);
        Ok(())
    })
}

/// `Rider_RefreshPose(rider)` (0x120ed8): rebuild the world matrix; unless
/// the rider is a puppet, tick the animation by 1.0 and evaluate the
/// skeleton.
pub fn rider_refresh_pose(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let (frame, rider) = open_s0_frame(c);
        callee::RIDER_BUILD_WORLD_MATRIX.call(c, 0x0012_0eec)?;
        let puppet = lw(c, rider, rider::IS_PUPPET);
        c.g[V0][0] = puppet;
        if puppet == 0 {
            c.cycles += 7;
            // dt = 1.0 (through `at`).
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            c.f[12] = FpuF32::ONE.0;
            c.g[A0][0] = rider;
            callee::RIDER_TICK_ANIMATION.call(c, 0x0012_0f08)?;
            c.g[A0][0] = rider;
            callee::RIDER_UPDATE_SKELETON.call(c, 0x0012_0f10)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_FindNearbyLights(rider)` (0x122088): the eight strongest world
/// lights around the rider's hips (into `rider+0x794`). The hips position
/// is a 16-byte buffer at the bottom of the frame.
pub fn rider_find_nearby_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_ra(c, 0x10);
        let hips = c.g[SP][0];
        c.g[A0][0] = hips;
        c.g[A1][0] = rider;
        callee::RIDER_GET_HIPS_POSITION.call(c, 0x0012_20a4)?;
        let app = Addr(gp_minus(c, gp::APP_NEG)).lw(c);
        c.g[V1][0] = app;
        c.g[A3][0] = ptr_add(rider, rider::LIGHTS);
        c.g[A2][0] = lw(c, rider, rider::OVERLAP_LIST);
        c.g[A1][0] = hips;
        let module = lw(c, app, game_module::OF_APP);
        c.g[V0][0] = module;
        c.g[T0][0] = 8;
        c.g[A0][0] = lw(c, module, game_module::LIGHTS);
        call_ptr(c, table::WORLD_LIGHT_MAN_FIND_STRONGEST_LIGHTS, 0x0012_20c4)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_ClearNearbyLights(rider)` (0x122658): zero the eight light
/// handles (`EA_MemSet(rider+0x794, 0, 0x20)`). 9 cycles.
pub fn rider_clear_nearby_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = ptr_add(c.g[A0][0], rider::LIGHTS);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20;
        call_ptr(c, table::EA_MEM_SET, 0x0012_2670)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_SetAnimTimeScale(rider, target /*f12*/)` (0x120090): slew the
/// rubber-band time scale toward `target` by at most 0.05/60 per call
/// (CPU riders' catch-up; at another rate a call's step is `60/rate` of that,
/// so the slew takes the same wall-clock time). Leaf.
pub fn rider_set_anim_time_scale(c: &mut Cpu) {
    c.cycles += 7;
    let rider = c.g[A0][0] as u32;
    let raw = Addr(gp_minus(c, gp::TIME_SCALE_STEP_NEG)).read_fpu(c);
    let step = rate_step_60(c, raw);
    let scale = Addr(rider.wrapping_add(rider::TIME_SCALE));
    let cur = scale.read_fpu(c);
    let target = FpuF32(c.f[12]);
    let upper = target + step;
    c.f[2] = step.0;
    c.f[1] = cur.0;
    let result = if crate::abi::c_lt_s(c, upper, cur) {
        // Above the band: step down.
        c.f[0] = upper.0;
        c.cycles += 2;
        cur - step
    } else {
        let lower = target - step;
        c.f[0] = lower.0;
        c.cycles += 4;
        if crate::abi::c_lt_s(c, cur, lower) {
            // Below the band: step up.
            cur + step
        } else {
            target
        }
    };
    c.f[12] = result.0;
    c.cycles += 2;
    scale.write_fpu(c, result);
    crate::abi::ret(c);
}

// ---------------------------------------------------------------------------
// State machine.

/// `RiderBase_SetMainState(base, state)` (0x1112b8): leave the current main
/// state, store the new one, enter it (`Enter` gets the old state).
pub fn rider_base_set_main_state(c: &mut Cpu) {
    set_state(
        c,
        base::MAIN_STATE,
        callee::RIDER_BASE_EXIT_MAIN_STATE,
        callee::RIDER_BASE_ENTER_MAIN_STATE,
        [0x0011_12d4, 0x0011_12e4],
    )
}

/// `RiderBase_SetSubState(base, state)` (0x111538): the same for the sub
/// state.
pub fn rider_base_set_sub_state(c: &mut Cpu) {
    set_state(
        c,
        base::SUB_STATE,
        callee::RIDER_BASE_EXIT_SUB_STATE,
        callee::RIDER_BASE_ENTER_SUB_STATE,
        [0x0011_1554, 0x0011_1564],
    )
}

/// `RiderWipeoutState_Update(state)` (0x136e98): pose the root, then the
/// ground or air part of the wipeout.
pub fn rider_wipeout_state_update(c: &mut Cpu) {
    wipeout_dispatch(
        c,
        callee::WIPEOUT_UPDATE_ROOT_POSE,
        callee::WIPEOUT_UPDATE_GROUND,
        callee::WIPEOUT_UPDATE_AIR,
        [0x0013_6eac, 0x0013_6ec0, 0x0013_6ed0],
    )
}

/// `RiderWipeoutState_PostUpdate(state)` (0x136ee0): the same after the
/// physics step.
pub fn rider_wipeout_state_post_update(c: &mut Cpu) {
    wipeout_dispatch(
        c,
        callee::WIPEOUT_POST_UPDATE_ROOT_POSE,
        callee::WIPEOUT_POST_UPDATE_GROUND,
        callee::WIPEOUT_POST_UPDATE_AIR,
        [0x0013_6ef4, 0x0013_6f08, 0x0013_6f18],
    )
}

/// `RiderHandplantState_PostUpdate(state)` (0x139528):
/// `Rider_UpdateBounds(state->rider (+0xa0), 0)`. 8 cycles.
pub fn rider_handplant_state_post_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, c.g[A0][0], 0xa0);
        call_ptr(c, table::RIDER_UPDATE_BOUNDS, 0x0013_953c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderCruiseSub_CheckAirborne(sub)` (0x131cc0): when the rider
/// (`sub+0`) has left the ground (main state Air), switch to the
/// NaturalAir sub state and return 1; else 0.
pub fn rider_cruise_sub_check_airborne(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let (frame, sub) = open_s0_frame(c);
        c.g[A0][0] = lw(c, sub, 0);
        callee::RIDER_GET_MAIN_STATE.call(c, 0x0013_1cd8)?;
        c.g[V1][0] = 1;
        let airborne = c.g[V0][0] == main_state::AIR;
        c.g[V0][0] = 0;
        if airborne {
            c.cycles += 4;
            c.g[A0][0] = lw(c, sub, 0);
            c.g[A1][0] = sub_state::NATURAL_AIR;
            callee::RIDER_SET_SUB_STATE.call(c, 0x0013_1cf0)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        close_s0_frame(c, frame);
        Ok(())
    })
}

/// `Rider_IsOnGround(rider)` (0x1231a8): main state Ground, or a Wipeout
/// that is sliding on the ground. (Reads the main state twice.)
pub fn rider_is_on_ground(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::RIDER_GET_MAIN_STATE.call(c, 0x0012_31c4)?;
        let on_ground = if c.g[V0][0] == main_state::GROUND {
            true
        } else {
            c.cycles += 5;
            c.g[A0][0] = rider;
            callee::RIDER_GET_MAIN_STATE.call(c, 0x0012_31d4)?;
            c.g[V1][0] = 2;
            let wipeout = c.g[V0][0] == main_state::WIPEOUT;
            c.g[V0][0] = 0;
            if wipeout {
                c.cycles += 4;
                let owner = lw(c, rider, rider::OWNER);
                let airborne = lw(c, owner, base::WIPEOUT_AIRBORNE);
                c.g[V1][0] = airborne;
                c.g[V0][0] = 0;
                if airborne == 0 {
                    c.cycles += 1;
                }
                airborne == 0
            } else {
                false
            }
        };
        if on_ground {
            c.g[S1][0] = 1;
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_TryEnterFinishLine(rider)` (0x116378): once the finish timer has
/// started (`0 <= finishTimer`), switch to the FinishLine sub state and
/// return 1; else 0.
pub fn rider_try_enter_finish_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.f[1] = 0;
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        let timer = Addr((rider as u32).wrapping_add(rider::FINISH_TIMER)).read_fpu(c);
        c.f[0] = timer.0;
        let finished = c_le_s(c, FpuF32::ZERO, timer);
        c.g[V0][0] = 0;
        if finished {
            c.cycles += 3;
            c.g[A1][0] = sub_state::FINISH_LINE;
            callee::RIDER_SET_SUB_STATE.call(c, 0x0011_63a0)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_CheckResetPath(rider, force, reason)` (0x116120): put the rider
/// back on the course. Nothing happens when both `force` and `reason` are
/// 0. Otherwise clear the power-up timers, show "wrong way" for reasons 1
/// and 4, flag the Reset sub state with `reason != 0`, switch to sub state
/// Reset / main state Static, and tell the audio system and the replay.
/// Returns 1 when reset.
pub fn rider_check_reset_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_ra(c, 0);
        let force = c.g[A1][0] != 0;
        let reason = c.g[A2][0];
        c.g[S1][0] = reason;
        if !force {
            c.cycles += 2;
            c.g[V0][0] = 0;
            if reason == 0 {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        let at = |off| Addr((rider as u32).wrapping_add(off));
        at(rider::POWER_UP_BOOST_TIMER).sw(c, 0);
        c.g[V0][0] = 1;
        at(rider::POWER_UP_TRICK_TIMER).sw(c, 0);
        let wrong_way = if reason == 1 {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            reason == 4
        };
        if wrong_way {
            c.cycles += 3;
            c.g[A0][0] = lw(c, rider, rider::METRIX);
            callee::RIDER_METRIX_SHOW_WRONG_WAY.call(c, 0x0011_6168)?;
        }
        let owner = lw(c, rider, rider::OWNER);
        c.g[V1][0] = owner;
        c.cycles += 19;
        let flag = (reason != 0) as u64;
        c.g[V0][0] = flag;
        c.g[A0][0] = rider;
        c.g[A1][0] = sub_state::RESET;
        Addr((owner as u32).wrapping_add(base::RESET_SUB_FLAG)).sw(c, flag);
        callee::RIDER_SET_SUB_STATE.call(c, 0x0011_6180)?;
        c.g[A0][0] = rider;
        c.g[A1][0] = main_state::STATIC;
        callee::RIDER_SET_MAIN_STATE.call(c, 0x0011_618c)?;
        call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_6194)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = rider;
        call_ptr(c, table::SSX_AUDIO_SYSTEM_ON_RIDER_RESET, 0x0011_61a0)?;
        let app = Addr(gp_minus(c, gp::APP_NEG)).lw(c);
        c.g[V1][0] = app;
        c.g[A1][0] = lw(c, rider, rider::INDEX);
        let module = lw(c, app, game_module::OF_APP);
        c.g[V0][0] = module;
        c.g[A0][0] = lw(c, module, game_module::REPLAY);
        call_ptr(c, table::REPLAY_ON_RIDER_RESET, 0x0011_61b4)?;
        c.g[V0][0] = 1;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_TryEnterPrewind(rider, jumpPressed, cheatBit)` (0x1162c8): start
/// a jump (sub state Prewind) on a fresh jump press (`jumpRearm == 0` and
/// `jumpPressed`) or when `cheatBit` forces it (which is also posted to the
/// cheat-code detector); leaving BoardPress releases the press boost first.
/// Returns 1 when the prewind starts, else re-arms the jump (`1.0`) and
/// returns 0.
pub fn rider_try_enter_prewind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save(c, S2, 0x10);
        let cheat = c.g[A2][0];
        c.g[S1][0] = cheat;
        frame.save_ra(c, 0);
        let jump = c.g[A1][0];
        c.g[S2][0] = jump;
        if cheat != 0 {
            c.cycles += 4;
            let owner = lw(c, rider, rider::OWNER);
            c.g[A1][0] = 1;
            c.g[A0][0] = ptr_add(owner, base::CHEAT_DETECTOR);
            call_ptr(c, table::CHEAT_CODE_DETECTOR_POST_EVENT, 0x0011_62fc)?;
        }
        c.cycles += 5;
        let rearm = Addr((rider as u32).wrapping_add(rider::JUMP_REARM));
        let rearm_v = rearm.read_fpu(c);
        c.f[1] = rearm_v.0;
        c.f[0] = 0;
        let fresh_press = if c_eq_s(c, rearm_v, FpuF32::ZERO) {
            c.cycles += 2;
            jump != 0
        } else {
            false
        };
        let go = fresh_press || {
            c.cycles += 2;
            c.g[V0][0] = 0;
            cheat != 0
        };
        if go {
            c.cycles += 5;
            c.g[A0][0] = rider;
            callee::RIDER_GET_SUB_STATE.call(c, 0x0011_6328)?;
            c.g[V1][0] = 1;
            let from_press = c.g[V0][0] == sub_state::BOARD_PRESS;
            c.g[A0][0] = rider;
            if from_press {
                c.cycles += 4;
                let owner = lw(c, rider, rider::OWNER);
                c.g[A0][0] = ptr_add(owner, base::BOARD_PRESS_SUB);
                callee::RIDER_BOARD_PRESS_SUB_RELEASE.call(c, 0x0011_6340)?;
                c.g[A0][0] = rider;
            }
            c.cycles += 4;
            c.g[A1][0] = sub_state::PREWIND;
            callee::RIDER_SET_SUB_STATE.call(c, 0x0011_634c)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 3;
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            c.f[0] = FpuF32::ONE.0;
            rearm.write_fpu(c, FpuF32::ONE);
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

/// `RiderSpinSub_GetHUDAngle(sub, &centred)` (0x1360c8): the stick angle
/// for the spin HUD in f0, or 0.0 with `*centred = 1` while the stick is
/// centred (angle 1e30). Leaf.
pub fn rider_spin_sub_get_hud_angle(c: &mut Cpu) {
    c.cycles += 6;
    let sub = c.g[A0][0] as u32;
    let centred_mark = Addr(gp_minus(c, gp::STICK_CENTRED_NEG)).read_fpu(c);
    let angle = Addr(sub.wrapping_add(0x50));
    let a = angle.read_fpu(c);
    c.f[1] = centred_mark.0;
    let centred = c_eq_s(c, a, centred_mark);
    if !centred {
        c.cycles += 1;
    }
    c.g[V0][0] = centred as u64;
    c.cycles += 3;
    c.f[0] = 0;
    Addr(c.g[A1][0] as u32).sw(c, centred as u64);
    if !centred {
        c.cycles += 1;
        c.f[0] = angle.read_u32(c);
    }
    c.cycles += 2;
    crate::abi::ret(c);
}

#[cfg(test)]
mod tests {
    //! Differential tests against the generated functions.
    //!
    //! Callees run for real: both machines run the same generated callee
    //! from the same state. The objects they get are built so that the
    //! callees reach virtual calls through test vtables (or null ones),
    //! which land in the fakes of `difftest::fakes`: those log every
    //! register at the call and unwind at a chosen call index, so the
    //! unwind protocol is exercised through the real callees too. Branches
    //! are chosen by the seed (`seed % n`), so every path is taken by
    //! construction; with real callees a path's total cycle cost is not
    //! fixed, so `check_paths` is used for the two leaves only.
    use crate::difftest::fakes::*;
    use crate::difftest::{check, check_paths, fresh, put_words, Rng, GAME_GP, RA};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    /// `check`, plus how many seeds reached a fake and how many unwound
    /// (asserted against the minimums given).
    fn run(
        name: &str,
        gen: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        seeds: u32,
        min: (u32, u32),
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        check(gen, hand, seeds, setup);
        let (mut reached, mut unwound) = (0, 0);
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            reached += (c.bus.read32(LOG_COUNT) > 0) as u32;
            unwound += (c.pc != RA) as u32;
        }
        eprintln!("{name}: {seeds} seeds, {reached} reached a fake, {unwound} unwound");
        assert!(
            reached >= min.0 && unwound >= min.1,
            "{name}: reached {reached}, unwound {unwound}"
        );
    }

    /// The game's initialised data (the ELF's load segment: code, `.data`,
    /// `.sdata` constants), so that real callees deep in the call tree see
    /// the game's constants (step sizes, limits) instead of zeros.
    fn elf_image(c: &mut Cpu) {
        static IMAGE: std::sync::OnceLock<(u32, Vec<u8>)> = std::sync::OnceLock::new();
        let (vaddr, bytes) = IMAGE.get_or_init(|| {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
            let d = std::fs::read(path).expect("orig/SLUS_207.72");
            let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
            let ph = u32_at(0x1c) as usize;
            // The single PT_LOAD segment: offset, vaddr, filesz.
            let (off, vaddr, size) = (
                u32_at(ph + 4) as usize,
                u32_at(ph + 8),
                u32_at(ph + 16) as usize,
            );
            (vaddr, d[off..off + size].to_vec())
        });
        let at = *vaddr as usize;
        c.bus.ram[at..at + bytes.len()].copy_from_slice(bytes);
    }

    /// [`elf_image`] plus a constructed back-end library singleton whose
    /// interfaces (`BELibrary_GetInterface`, vtable slot +8) are fakes, so
    /// deep callees asking the back end for sound/effects services stop at
    /// a fake instead of constructing the real back end.
    fn game_world(c: &mut Cpu, r: &mut Rng) {
        elf_image(c);
        // The back end's lazily created singletons (gp-0x1f04, gp-0x1f28).
        for off in [0x1f04, 0x1f28] {
            let singleton = obj(r);
            c.bus.write32(GAME_GP - off, singleton);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        for i in 0..8 {
            let lib = obj(r);
            c.bus.write32(0x0053_08a8 + 4 * i, lib);
            let vt = vtable(c, r, &[8]);
            c.bus.write32(lib + 4, vt);
        }
    }

    /// A zeroed object.
    fn obj(r: &mut Rng) -> u32 {
        ptr(r)
    }

    /// A rider in a0; returns its address.
    fn rider(c: &mut Cpu, r: &mut Rng) -> u32 {
        let p = obj(r);
        c.g[A0][0] = p as u64;
        p
    }

    /// An animation controller with one sequence on layer 0 and a vtable
    /// whose slots (+0x10 update, +0x18 end event) are fakes.
    fn anim_controller(c: &mut Cpu, r: &mut Rng) -> u32 {
        let a = obj(r);
        let layers = obj(r);
        let node = obj(r);
        c.bus.write32(a + 0x50, layers);
        c.bus.write32(layers + 4, node);
        c.bus.write32(node + 0xc8, 0);
        let vt = vtable(c, r, &[0x10, 0x18]);
        c.bus.write32(a + 0x58, vt);
        a
    }

    /// `g_App` with a `GameModule` whose fields `offs` are zeroed objects.
    fn game_module(c: &mut Cpu, r: &mut Rng, offs: &[u32]) -> u32 {
        let app = obj(r);
        let module = obj(r);
        c.bus.write32(GAME_GP - 0x848, app);
        c.bus.write32(app + 0x84, module);
        for &o in offs {
            let x = obj(r);
            c.bus.write32(module + o, x);
        }
        module
    }

    /// A `RiderBase` owner for `rider` with the given main/sub states and
    /// a vtable at +0 (fakes) for the state hooks.
    fn owner(c: &mut Cpu, r: &mut Rng, rider: u32, main: u32, sub: u32) -> u32 {
        let o = obj(r);
        c.bus.write32(rider + 0x77c, o);
        c.bus.write32(o + 0xde0, main);
        c.bus.write32(o + 0xde4, sub);
        o
    }

    /// A state id past the end of the state tables (the game's Exit/Enter
    /// switches skip those), so state changes don't run the real state
    /// handlers on the test's empty objects.
    fn state(r: &mut Rng) -> u32 {
        if r.next_u32() % 2 == 0 {
            0xe + r.next_u32() % 8
        } else {
            r.next_u32() | 0x8000_0000
        }
    }

    #[test]
    fn tick_animation() {
        run(
            "tick_animation",
            ssx3_ee::s00::f_0011eb60__generated,
            super::rider_tick_animation,
            SEEDS,
            (SEEDS, 20),
            &|c, seed| {
                let mut r = machine(c, seed, 1, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                let a = anim_controller(c, &mut r);
                c.bus.write32(p + 0x784, a);
                c.bus.write32(p + 0xb1c, r.next_u32());
            },
        );
    }

    #[test]
    fn update_animation() {
        run(
            "update_animation",
            ssx3_ee::s00::f_00121700__generated,
            super::rider_update_animation,
            SEEDS,
            (SEEDS / 2, 10),
            &|c, seed| {
                let mut r = machine(c, seed, 2, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                let a = anim_controller(c, &mut r);
                c.bus.write32(p + 0x784, a);
                c.bus.write32(p + 0x300, r.edge_f32());
                c.bus
                    .write32(p + 0xac4, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            },
        );
    }

    #[test]
    fn update_pose() {
        run(
            "update_pose",
            ssx3_ee::s00::f_00121728__generated,
            super::rider_update_pose,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 3, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                c.bus
                    .write32(p + 0xac4, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                let model = obj(&mut r);
                c.bus.write32(p + 0x780, model);
                let a = anim_controller(c, &mut r);
                c.bus.write32(p + 0x784, a);
            },
        );
    }

    #[test]
    fn render_shadow_and_prepare_lighting() {
        for (name, gen, hand) in [
            (
                "render_shadow",
                ssx3_ee::s00::f_001225f0__generated as fn(&mut Cpu),
                super::rider_render_shadow as fn(&mut Cpu),
            ),
            (
                "prepare_lighting",
                ssx3_ee::s00::f_001225c0__generated,
                super::rider_prepare_lighting,
            ),
        ] {
            run(name, gen, hand, SEEDS, (0, 0), &|c, seed| {
                let mut r = machine(c, seed, 4, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                c.bus
                    .write32(p + 0xb18, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                let model = obj(&mut r);
                c.bus.write32(p + 0x780, model);
                game_module(c, &mut r, &[0x78, 0x90]);
            });
        }
    }

    #[test]
    fn update_model_and_attached_fx() {
        for (name, gen, hand) in [
            (
                "update_model",
                ssx3_ee::s00::f_001217f8__generated as fn(&mut Cpu),
                super::rider_update_model as fn(&mut Cpu),
            ),
            (
                "render_attached_fx",
                ssx3_ee::s00::f_00122638__generated,
                super::rider_render_attached_fx,
            ),
            (
                "update_attached_fx",
                ssx3_ee::s00::f_00120e88__generated,
                super::rider_update_attached_fx,
            ),
        ] {
            run(name, gen, hand, SEEDS, (0, 0), &|c, seed| {
                let mut r = machine(c, seed, 5, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                for off in [0x780, 0x88c, 0x890] {
                    let o = if seed % 4 == 0 { 0 } else { obj(&mut r) };
                    c.bus.write32(p + off, o);
                    if o != 0 {
                        let vt = vtable(c, &mut r, &[8, 0x10, 0x18, 0x20]);
                        c.bus.write32(o, vt);
                    }
                }
            });
        }
    }

    #[test]
    fn update_surface_fx() {
        run(
            "update_surface_fx",
            ssx3_ee::s00::f_001218d0__generated,
            super::rider_update_surface_fx,
            SEEDS,
            (SEEDS, 20),
            &|c, seed| {
                let mut r = machine(c, seed, 6, 3);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                game_module(c, &mut r, &[0x78]);
                let id = match seed % 3 {
                    0 => u32::MAX,
                    1 => r.next_u32() & 0xff,
                    _ => r.next_u32(),
                };
                c.bus.write32(p + 0x430, id);
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(p + 0x6c0, vt);
                c.bus.write32(p + 0x460, r.edge_f32());
                c.bus.write32(p + 0x464, r.edge_f32());
                put_words(c, GAME_GP - 0x7910, &[0xc7c3_4f80]);
                if seed % 2 == 0 {
                    obj_owner(c, &mut r, p);
                }
            },
        );
    }

    fn obj_owner(c: &mut Cpu, r: &mut Rng, rider: u32) -> u32 {
        let o = obj(r);
        c.bus.write32(rider + 0x77c, o);
        o
    }

    #[test]
    fn update_progress() {
        run(
            "update_progress",
            ssx3_ee::s00::f_00121818__generated,
            super::rider_update_progress,
            SEEDS,
            (SEEDS, 50),
            &|c, seed| {
                let mut r = machine(c, seed, 7, 3);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                game_module(c, &mut r, &[0x28, 0x78]);
                match seed % 3 {
                    0 => {}
                    1 => {
                        let o = obj(&mut r);
                        c.bus.write32(p + 0xa30, o);
                        let ws = obj(&mut r);
                        c.bus.write32(GAME_GP + 0xce8, ws);
                    }
                    _ => {
                        let o = obj(&mut r);
                        c.bus.write32(p + 0xa30, o);
                        let t = obj(&mut r);
                        c.bus.write32(o + 0xc, t);
                        let vt = vtable(c, &mut r, &[0x140]);
                        c.bus.write32(t + 0xc, vt);
                    }
                }
                c.bus.write32(
                    p + 0xac4,
                    if seed / 3 % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    },
                );
                obj_owner(c, &mut r, p);
                let m = obj(&mut r);
                c.bus.write32(p + 0x790, m);
            },
        );
    }

    #[test]
    fn refresh_pose() {
        run(
            "refresh_pose",
            ssx3_ee::s00::f_00120ed8__generated,
            super::rider_refresh_pose,
            SEEDS,
            (SEEDS / 2, 10),
            &|c, seed| {
                let mut r = machine(c, seed, 8, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                for i in 0..4 {
                    c.bus.write32(p + 0x110 + 4 * i, r.f32(1e4));
                    c.bus.write32(p + 0x120 + 4 * i, r.f32(1.0));
                }
                c.bus
                    .write32(p + 0xac4, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                let a = anim_controller(c, &mut r);
                c.bus.write32(p + 0x784, a);
                let model = obj(&mut r);
                c.bus.write32(p + 0x780, model);
            },
        );
    }

    #[test]
    fn find_nearby_lights() {
        run(
            "find_nearby_lights",
            ssx3_ee::s00::f_00122088__generated,
            super::rider_find_nearby_lights,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 9, 2);
                game_world(c, &mut r);
                let p = rider(c, &mut r);
                game_module(c, &mut r, &[0x90]);
                let model = obj(&mut r);
                c.bus.write32(p + 0x780, model);
                let list = obj(&mut r);
                c.bus.write32(p + 0x860, list);
            },
        );
    }

    #[test]
    fn clear_nearby_lights() {
        run(
            "clear_nearby_lights",
            ssx3_ee::s00::f_00122658__generated,
            super::rider_clear_nearby_lights,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 10, 1);
                let p = rider(c, &mut r);
                crate::difftest::fill(c, &mut r, p + 0x790, 12);
            },
        );
    }

    #[test]
    fn set_anim_time_scale() {
        check_paths(
            ssx3_ee::s00::f_00120090__generated,
            super::rider_set_anim_time_scale,
            2000,
            &[11, 13],
            &|c, seed| {
                let mut r = machine(c, seed, 11, 0);
                let p = rider(c, &mut r);
                let k = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    0x3a5a_740f
                };
                put_words(c, GAME_GP - 0x795c, &[k]);
                let target = r.edge_f32();
                c.f[12] = target;
                // Current scale near the target (within a few steps), or anywhere.
                let cur = match seed % 3 {
                    0 => r.edge_f32(),
                    _ => {
                        let d = (r.next_u32() % 7) as f32 - 3.0;
                        (f32::from_bits(target) + d * 0.0004).to_bits()
                    }
                };
                c.bus.write32(p + 0x300, cur);
            },
        );
    }

    /// `Rider_SetAnimTimeScale` (0x120090) slews by 0.000833 **per call**
    /// (gp-0x795c, 0.05/s at 60 Hz): half a second of calls must reach the same
    /// time scale at 120/240 Hz, so the rubber-band catch-up keeps its timing.
    #[test]
    fn rider_set_anim_time_scale_follows_the_rate() {
        use crate::tick;
        // The game's step at gp-0x795c (0.0008333333).
        const STEP_60: u32 = 0x3a5a_740f;
        let scale = |hz: u32, ticks: u32| -> f32 {
            let mut c = fresh();
            c.g[GP][0] = GAME_GP as u64;
            let p = 0x0080_0000u32;
            c.g[A0][0] = p as u64;
            c.bus.write32(p + 0x300, 1.0f32.to_bits()); // the time scale
            c.bus.write32(GAME_GP.wrapping_sub(0x795c), STEP_60);
            tick::set_for_test(&mut c, hz);
            for _ in 0..ticks {
                c.f[12] = 1.1f32.to_bits(); // the target (above the band)
                super::rider_set_anim_time_scale(&mut c);
            }
            f32::from_bits(c.bus.read32(p + 0x300))
        };
        // 30 ticks at 60 Hz = 0.025 up from 1.0.
        let at_60 = scale(60, 30);
        assert!((at_60 - 1.025).abs() < 1e-4, "{at_60}");
        for hz in [120u32, 240] {
            let at = scale(hz, 30 * hz / 60);
            assert!(
                (at - at_60).abs() < 1e-5,
                "{hz} Hz: {at} after half a second, 60 Hz: {at_60}"
            );
        }
    }

    #[test]
    fn rider_base_set_states() {
        for (name, gen, hand) in [
            (
                "set_main_state",
                ssx3_ee::s00::f_001112b8__generated as fn(&mut Cpu),
                super::rider_base_set_main_state as fn(&mut Cpu),
            ),
            (
                "set_sub_state",
                ssx3_ee::s00::f_00111538__generated,
                super::rider_base_set_sub_state,
            ),
        ] {
            run(name, gen, hand, SEEDS, (0, 0), &|c, seed| {
                let mut r = machine(c, seed, 12, 2);
                // Without the game image the state switches jump through empty
                // tables into the fakes; with it they skip the out-of-range states.
                if seed % 5 < 3 {
                    game_world(c, &mut r);
                }
                let p = obj(&mut r);
                // In-range states only without the image (their switches
                // then jump into the fakes).
                let pick = |r: &mut Rng| {
                    if seed % 5 < 3 {
                        state(r)
                    } else {
                        r.next_u32() % 6
                    }
                };
                let (m_, s_) = (pick(&mut r), pick(&mut r));
                let o = owner(c, &mut r, p, m_, s_);
                c.g[A0][0] = o as u64;
                c.g[A1][0] = (r.next_u32() as u64) << 32 | pick(&mut r) as u64;
            });
        }
    }

    #[test]
    fn wipeout_update_and_post_update() {
        for (name, gen, hand) in [
            (
                "wipeout_update",
                ssx3_ee::s01::f_00136e98__generated as fn(&mut Cpu),
                super::rider_wipeout_state_update as fn(&mut Cpu),
            ),
            (
                "wipeout_post_update",
                ssx3_ee::s01::f_00136ee0__generated,
                super::rider_wipeout_state_post_update,
            ),
        ] {
            run(name, gen, hand, SEEDS, (0, 0), &|c, seed| {
                let mut r = machine(c, seed, 13, 3);
                game_world(c, &mut r);
                let s = obj(&mut r);
                c.g[A0][0] = s as u64;
                c.bus
                    .write32(s, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                // The wipeout's rider (+0x40) with an air predictor (+0x788)
                // that is already settled (state 5: no simulation steps).
                let p = obj(&mut r);
                c.bus.write32(s + 0x40, p);
                let predictor = obj(&mut r);
                c.bus.write32(p + 0x788, predictor);
                c.bus.write32(predictor + 0xac, 5);
                c.bus.write32(p + 0x300, 0x3f80_0000);
            });
        }
    }

    #[test]
    fn handplant_post_update() {
        run(
            "handplant_post_update",
            ssx3_ee::s01::f_00139528__generated,
            super::rider_handplant_state_post_update,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 14, 1);
                let s = obj(&mut r);
                c.g[A0][0] = s as u64;
                let p = if seed % 4 == 0 { 0 } else { obj(&mut r) };
                c.bus.write32(s + 0xa0, p);
            },
        );
    }

    #[test]
    fn cruise_sub_check_airborne() {
        run(
            "check_airborne",
            ssx3_ee::s00::f_00131cc0__generated,
            super::rider_cruise_sub_check_airborne,
            SEEDS,
            (30, 30),
            &|c, seed| {
                let mut r = machine(c, seed, 15, 2);
                // Without the game image the state switches jump through empty
                // tables into the fakes; with it they skip the out-of-range states.
                if seed % 5 < 3 {
                    game_world(c, &mut r);
                }
                let sub = obj(&mut r);
                c.g[A0][0] = sub as u64;
                let p = obj(&mut r);
                c.bus.write32(sub, p);
                let main = if seed % 2 == 0 { 1 } else { state(&mut r) };
                let (m_, s_) = (main, state(&mut r));
                owner(c, &mut r, p, m_, s_);
            },
        );
    }

    #[test]
    fn is_on_ground() {
        run(
            "is_on_ground",
            ssx3_ee::s00::f_001231a8__generated,
            super::rider_is_on_ground,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 16, 0);
                let p = rider(c, &mut r);
                let main = match seed % 4 {
                    0 => 0,
                    1 | 2 => 2,
                    _ => state(&mut r),
                };
                let (m_, s_) = (main, state(&mut r));
                let o = owner(c, &mut r, p, m_, s_);
                c.bus
                    .write32(o + 0x30, if seed % 4 == 1 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn try_enter_finish_line() {
        run(
            "finish_line",
            ssx3_ee::s00::f_00116378__generated,
            super::rider_try_enter_finish_line,
            600,
            (60, 60),
            &|c, seed| {
                let mut r = machine(c, seed, 17, 2);
                // Without the game image the state switches jump through empty
                // tables into the fakes; with it they skip the out-of-range states.
                if seed % 5 < 3 {
                    game_world(c, &mut r);
                }
                let p = rider(c, &mut r);
                let t = match seed % 4 {
                    0 => 0xbf80_0000,
                    1 => (seed & 8) << 28,
                    _ => r.edge_f32(),
                };
                c.bus.write32(p + 0x470, t);
                let (m_, s_) = (state(&mut r), state(&mut r));
                owner(c, &mut r, p, m_, s_);
            },
        );
    }

    #[test]
    fn check_reset_path() {
        run(
            "check_reset_path",
            ssx3_ee::s00::f_00116120__generated,
            super::rider_check_reset_path,
            600,
            (100, 100),
            &|c, seed| {
                let mut r = machine(c, seed, 18, 6);
                // Without the game image the state switches jump through empty
                // tables into the fakes; with it they skip the out-of-range states.
                if seed % 5 < 3 {
                    game_world(c, &mut r);
                }
                let p = rider(c, &mut r);
                let force = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) << 32 | 1
                };
                let reason: u64 = match seed / 2 % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 4,
                    3 => (r.next_u32() as u64) << 32,
                    _ => r.next_u32() as u64,
                };
                c.g[A1][0] = force;
                c.g[A2][0] = reason;
                let (m_, s_) = (state(&mut r), state(&mut r));
                owner(c, &mut r, p, m_, s_);
                let m = obj(&mut r);
                c.bus.write32(p + 0x790, m);
                game_module(c, &mut r, &[0x28]);
                c.bus.write32(p + 0x86c, r.next_u32() % 6);
            },
        );
    }

    #[test]
    fn try_enter_prewind() {
        run(
            "try_enter_prewind",
            ssx3_ee::s00::f_001162c8__generated,
            super::rider_try_enter_prewind,
            600,
            (100, 100),
            &|c, seed| {
                let mut r = machine(c, seed, 19, 4);
                let p = rider(c, &mut r);
                let cheat = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) << 32 | (r.next_u32() | 1) as u64
                };
                let jump = if seed / 2 % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.g[A1][0] = jump;
                c.g[A2][0] = cheat;
                let rearm = match seed / 4 % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => 0x3f80_0000,
                    _ => r.edge_f32(),
                };
                c.bus.write32(p + 0x360, rearm);
                let sub = if seed / 16 % 2 == 0 { 1 } else { state(&mut r) };
                let (m_, s_) = (state(&mut r), sub);
                owner(c, &mut r, p, m_, s_);
            },
        );
    }

    #[test]
    fn spin_sub_get_hud_angle() {
        check_paths(
            ssx3_ee::s01::f_001360c8__generated,
            super::rider_spin_sub_get_hud_angle,
            1000,
            &[11, 13],
            &|c, seed| {
                let mut r = machine(c, seed, 20, 0);
                let s = obj(&mut r);
                c.g[A0][0] = s as u64;
                c.g[A1][0] = obj(&mut r) as u64 + 4;
                put_words(c, GAME_GP - 0x7308, &[0x7149_f2ca]);
                let a = match seed % 3 {
                    0 => 0x7149_f2ca,
                    _ => r.edge_f32(),
                };
                c.bus.write32(s + 0x50, a);
            },
        );
    }
}
