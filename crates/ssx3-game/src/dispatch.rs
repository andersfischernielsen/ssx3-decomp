//! Tier B: small functions with calls: thin wrappers, virtual dispatch and
//! forwarding doors into the big state machines
//! (`docs/investigation/phase2-candidates.md`, groups B2/B3).
//!
//! Replacements here follow the call protocol of [`crate::call`] (stack
//! frame, registers at each call, unwind check, cycles between calls).

use crate::abi::addu;
use crate::call::{call_ptr, guest, tail_wrapper, Callee, Flow, Frame, Target, Unwound, VSlot};
use crate::guest::Addr;
use crate::ps2f::{FpuF32, Vec4};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_dd58,
        be_library_get_interface,
        "BELibrary_GetInterface",
    ),
    (0x003b_58a0, snd_lock, "SND_Lock"),
    (0x003b_58d8, snd_unlock, "SND_Unlock"),
    (
        0x003d_cf70,
        eafile_set_request_callback,
        "EAFile_SetRequestCallback",
    ),
    (0x0015_df98, game_camera_update, "GameCamera_Update"),
    (
        0x0015_edc8,
        game_camera_set_target_rider,
        "GameCamera_SetTargetRider",
    ),
    (
        0x0015_cfe8,
        manual_cam_controller_update,
        "ManualCamController_Update",
    ),
    (0x0016_24e8, chase_camera_tick, "ChaseCameraAlgorithm_Tick"),
    (
        0x0016_6f28,
        chase_camera_reset,
        "ChaseCameraAlgorithm_Reset",
    ),
    (
        0x0015_f5b0,
        rider_camera_target_get_velocity,
        "RiderCameraTarget_GetVelocity",
    ),
    (
        0x0015_f568,
        rider_camera_target_get_position,
        "RiderCameraTarget_GetPosition",
    ),
    (0x0016_8060, camera_base_update, "CameraBase_Update"),
    (0x0015_d780, camera_base_get_yaw, "CameraBase_GetYaw"),
    (
        0x0012_16e0,
        rider_update_main_state,
        "Rider_UpdateMainState",
    ),
    (
        0x0012_1750,
        rider_post_update_main_state,
        "Rider_PostUpdateMainState",
    ),
    (
        0x0012_1068,
        rider_process_control_input,
        "Rider_ProcessControlInput",
    ),
    (0x0011_fe78, rider_set_main_state, "Rider_SetMainState"),
    (0x0011_fec8, rider_set_sub_state, "Rider_SetSubState"),
    (
        0x0013_6958,
        rider_static_state_update,
        "RiderStaticState_Update",
    ),
    (0x0010_6538, rider_translate, "Rider_Translate"),
    (
        0x0031_28e8,
        anim_controller_play_anim,
        "AnimController_PlayAnim",
    ),
    (0x0031_7e30, operator_new_named, "operator_new_named"),
    (0x0031_7d70, ea_mem_alloc_named, "EA_MemAllocNamed"),
    (0x0031_7e50, builtin_delete, "__builtin_delete"),
    (
        0x0039_5370,
        ps2_graphics_man_set_viewport_rect,
        "PS2GraphicsMan_SetViewportRect",
    ),
    (
        0x0037_6938,
        ps2_graphics_man_set_scissor_clamped,
        "PS2GraphicsMan_SetScissorClamped",
    ),
];

/// `lw` (sign-extended, through the CPU load path).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const RIDER_BASE_UPDATE_MAIN_STATE: Callee = Callee {
        addr: 0x0011_1408,
        f: ee::s00::RiderBase_UpdateMainState,
    };
    pub const RIDER_BASE_POST_UPDATE_MAIN_STATE: Callee = Callee {
        addr: 0x0011_14a0,
        f: ee::s00::RiderBase_PostUpdateMainState,
    };
    pub const RIDER_BASE_SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_12b8,
        f: ee::s00::RiderBase_SetMainState,
    };
    pub const RIDER_BASE_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_1538,
        f: ee::s00::RiderBase_SetSubState,
    };
    pub const RIDER_BASE_HANDLE_SUB_STATE_INPUT: Callee = Callee {
        addr: 0x0011_1728,
        f: ee::s00::RiderBase_HandleSubStateInput,
    };
    pub const GAME_CAMERA_COMPUTE_VIEW: Callee = Callee {
        addr: 0x0015_e668,
        f: ee::s01::GameCamera_ComputeView,
    };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };
    pub const MEM_ALLOC_INTERNAL: Callee = Callee {
        addr: 0x0031_9a90,
        f: ee::s10::MEM_AllocInternal,
    };
    pub const MEM_FREE_INTERNAL: Callee = Callee {
        addr: 0x0031_9b48,
        f: ee::s10::MEM_FreeInternal,
    };
    pub const ANIM_TABLE_GET_ENTRY: Callee = Callee {
        addr: 0x0031_15c8,
        f: ee::s10::AnimTable_GetEntry,
    };
    pub const ANIM_CONTROLLER_PLAY_ANIM_IMPL: Callee = Callee {
        addr: 0x0031_1f00,
        f: ee::s10::AnimController_PlayAnimImpl,
    };
}

/// Cross-shard direct calls the compiler emitted as `call_indirect(c, addr)`:
/// they go through the function table like the original.
mod table_callee {
    pub const RIDER_GET_HIPS_POSITION: u32 = 0x0011_ff48;
    pub const RIDER_BUILD_WORLD_MATRIX: u32 = 0x0011_e098;
    pub const CAMERA_BASE_UPDATE_MATRICES: u32 = 0x0015_d928;
    pub const RIDER_MODEL_SET_POSITION: u32 = 0x0031_0530;
    pub const MATH_ATAN: u32 = 0x0031_c228;
    pub const EA_MUTEX_LOCK: u32 = 0x003e_5700;
    pub const EA_MUTEX_UNLOCK: u32 = 0x003e_5760;
}

// ---------------------------------------------------------------------------
// Globals.

/// `g_beLibrary`: the back end's interface table, indexed by the current
/// library number (`gp-0x1efc`).
const BE_LIBRARY: u32 = 0x0053_08a8;
/// gp offset of the current back-end library index.
const BE_LIBRARY_INDEX_GP: u32 = 0x1efc;
/// The sound lock / unlock hooks (`SND_DefaultLock` ...).
const SND_LOCK_HOOK: u32 = 0x0044_c46c;
const SND_UNLOCK_HOOK: u32 = 0x0044_c470;
/// The sound globals block; the lock nesting byte is at +0x176.
const SND_GLOBALS: u32 = 0x0050_a8e8;
const SND_LOCK_DEPTH: u32 = SND_GLOBALS + 0x176;
/// `EAFile` request table pointer (0x30-byte records).
const EAFILE_REQUESTS: u32 = 0x0051_9ad8;
/// The vec4 a rider's model position is compared with (`Rider_PostUpdate`).
const RIDER_MODEL_ORIGIN: u32 = 0x004f_f120;
/// The allocator's mutex (`EAMutex`).
const MEM_MUTEX: u32 = 0x0053_8b00;
/// The game's `gp`-relative float constants used by `CameraBase_GetYaw`
/// (gp-0x6cc0 .. gp-0x6cb4: the yaw for a zero/negative cos, and the
/// quadrant corrections).
const YAW_CONST_GP: [u32; 4] = [0x6cc0, 0x6cbc, 0x6cb8, 0x6cb4];
/// gp offset of the animation tables pointer (`AnimController_PlayAnim`).
const ANIM_TABLES_GP: u32 = 0xd8c;
/// Per-anim-type blend parameter table (`AnimController_PlayAnim`).
const ANIM_TYPE_PARAMS: u32 = 0x0048_d808;

// ---------------------------------------------------------------------------
// Back end, sound, files.

/// `BELibrary_GetInterface()` (0x14dd58): the current back-end library's
/// interface: `lib = g_beLibrary[index]; return lib->vt[1](lib)` (vtable
/// slot at +8). The game's most-called function. 16 cycles.
pub fn be_library_get_interface(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let index = lw(c, (c.g[GP][0] as u32).wrapping_sub(BE_LIBRARY_INDEX_GP));
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let entry = ((index as u32) << 2).wrapping_add(BE_LIBRARY);
        let lib = lw(c, entry);
        let vt = lw(c, (lib as u32).wrapping_add(4));
        let m = VSlot::read(c, vt, 8);
        // Registers at the call.
        c.g[V0][0] = m.func;
        c.g[V1][0] = vt;
        c.g[A2][0] = lib;
        c.g[A0][0] = m.this(lib);
        call_ptr(c, m.func as u32, 0x0014_dd8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SND_Lock()` (0x3b58a0): call the lock hook, then increment the lock
/// nesting byte. 14 cycles.
pub fn snd_lock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = 0x0045_0000;
        let frame = Frame::open(c, 0x10);
        let hook = lw(c, SND_LOCK_HOOK);
        c.g[A0][0] = hook;
        frame.save_ra(c, 0);
        call_ptr(c, hook as u32, 0x003b_58b8)?;
        c.g[V1][0] = SND_GLOBALS as u64;
        frame.restore_ra(c, 0);
        let depth = sx((c.lbu(SND_LOCK_DEPTH) as u32).wrapping_add(1));
        c.g[V0][0] = depth;
        c.sb(SND_LOCK_DEPTH, depth);
        frame.close(c);
        Ok(())
    })
}

/// `SND_Unlock()` (0x3b58d8): decrement the lock nesting byte, then call
/// the unlock hook. 14 cycles.
pub fn snd_unlock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let depth = sx((c.lbu(SND_LOCK_DEPTH) as u32).wrapping_sub(1));
        let hook = lw(c, SND_UNLOCK_HOOK);
        c.sb(SND_LOCK_DEPTH, depth);
        // Registers at the call.
        c.g[V1][0] = SND_GLOBALS as u64;
        c.g[A0][0] = 0x0045_0000;
        c.g[V0][0] = depth;
        c.g[A1][0] = hook;
        call_ptr(c, hook as u32, 0x003b_5904)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `EAFile_SetRequestCallback(id, cb)` (0x3dcf70): store `cb` in request
/// `id >> 24` (+0x28); if the request has already completed (+8 non-zero),
/// call `cb(id, req->result(+8), req->user(+0x14))` at once.
pub fn eafile_set_request_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0x30;
        // The id goes through the stack to pick out its top byte.
        let id_slot = frame.at(c, 0);
        c.sw(id_slot, c.g[A0][0]);
        c.g[A2][0] = 0x0052_0000;
        let table = lw(c, EAFILE_REQUESTS);
        let cb = c.g[A1][0];
        c.g[T0][0] = cb;
        let slot = c.lbu(id_slot.wrapping_add(3));
        frame.save_ra(c, 0x10);
        let off = crate::abi::mult(c, slot as u32, 0x30);
        c.g[V0][0] = sx(off);
        let req = addu(table, sx(off));
        c.g[V1][0] = req;
        let done = lw(c, (req as u32).wrapping_add(8));
        c.g[A1][0] = done;
        c.sw((req as u32).wrapping_add(0x28), cb);
        if done != 0 {
            c.cycles += 4;
            c.g[A2][0] = lw(c, (req as u32).wrapping_add(0x14));
            c.g[A1][0] = lw(c, (req as u32).wrapping_add(8));
            call_ptr(c, cb as u32, 0x003d_cfb4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera.

/// `GameCamera_Update(cam)` (0x15df98): update the active controller
/// (`cam+0xa0`, vtable slot +0x10), then `GameCamera_ComputeView(cam)`.
pub fn game_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let cam = c.g[A0][0];
        c.g[S0][0] = cam;
        let ctrl = lw(c, (cam as u32).wrapping_add(0xa0));
        let vt = lw(c, (ctrl as u32).wrapping_add(0x14));
        let m = VSlot::read(c, vt, 0x10);
        // Registers at the call.
        c.g[A1][0] = ctrl;
        c.g[V0][0] = vt;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(ctrl);
        call_ptr(c, m.func as u32, 0x0015_dfc0)?;
        c.g[A0][0] = cam;
        callee::GAME_CAMERA_COMPUTE_VIEW.call(c, 0x0015_dfc8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameCamera_SetTargetRider(cam, rider)` (0x15edc8): store the rider in
/// the chase state (`cam->chase(+0xa8)->state(+0x20)->rider(+4)`) and tell
/// the chase controller (vtable slot +0x20).
pub fn game_camera_set_target_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let cam = c.g[A0][0] as u32;
        let chase = lw(c, cam.wrapping_add(0xa8));
        let state = lw(c, (chase as u32).wrapping_add(0x20));
        c.sw((state as u32).wrapping_add(4), c.g[A1][0]);
        // Reloaded after the store (it could alias).
        let chase = lw(c, cam.wrapping_add(0xa8));
        let vt = lw(c, (chase as u32).wrapping_add(0x14));
        let m = VSlot::read(c, vt, 0x20);
        // Registers at the call.
        c.g[A1][0] = chase;
        c.g[V0][0] = vt;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(chase);
        call_ptr(c, m.func as u32, 0x0015_edf4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ManualCamController_Update(ctrl)` (0x15cfe8): forward to the selected
/// algorithm's update (`**ctrl->algos(+8)`, vtable at +0x10, slot +0x28).
pub fn manual_cam_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let algos = lw(c, (c.g[A0][0] as u32).wrapping_add(8));
        let sel = lw(c, algos as u32);
        let algo = lw(c, sel as u32);
        let vt = lw(c, (algo as u32).wrapping_add(0x10));
        let m = VSlot::read(c, vt, 0x28);
        // Registers at the call.
        c.g[A1][0] = algo;
        c.g[V0][0] = vt;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(algo);
        call_ptr(c, m.func as u32, 0x0015_d010)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCameraAlgorithm_Tick(this, arg)` (0x1624e8): if `this+0x2f0` is
/// set, call vtable slot +0x20; then slots +0x28 and +0x40(arg). The
/// vtable pointer is at `this+0x10`.
pub fn chase_camera_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let pending = lw(c, (this as u32).wrapping_add(0x2f0));
        c.g[V0][0] = pending;
        let arg = c.g[A1][0];
        c.g[S1][0] = arg;
        let vcall = |c: &mut Cpu, off: u32, ret: u32| -> Flow {
            let vt = lw(c, (this as u32).wrapping_add(0x10));
            let m = VSlot::read(c, vt, off);
            // Registers at the call.
            c.g[V0][0] = vt;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(this);
            call_ptr(c, m.func as u32, ret)
        };
        if pending != 0 {
            c.cycles += 5;
            vcall(c, 0x20, 0x0016_251c)?;
        }
        c.cycles += 16;
        vcall(c, 0x28, 0x0016_2530)?;
        c.g[A1][0] = arg;
        vcall(c, 0x40, 0x0016_2548)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCameraAlgorithm_Reset(this)` (0x166f28): ask the target rider's
/// camera interface (`this->target(+0x30)->rider(+4)+0x6c0`, slot +0x38) for
/// a value and pass it to this algorithm's vtable slot +0x18.
pub fn chase_camera_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x10);
        let vt = lw(c, (this as u32).wrapping_add(0x10));
        c.g[S1][0] = vt;
        let target = lw(c, (this as u32).wrapping_add(0x30));
        let reset = VSlot::read(c, vt, 0x18);
        c.g[A2][0] = reset.delta;
        let rider = lw(c, (target as u32).wrapping_add(4));
        let self_this = reset.this(this);
        c.g[S0][0] = self_this;
        let iface = addu(rider, 0x6c0);
        let ivt = lw(c, iface as u32);
        let m = VSlot::read(c, ivt, 0x38);
        // Registers at the call.
        c.g[A1][0] = ivt;
        c.g[V1][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = m.this(iface);
        call_ptr(c, m.func as u32, 0x0016_6f68)?;
        let func = lw(c, (vt as u32).wrapping_add(0x1c));
        // Registers at the call.
        c.g[V1][0] = func;
        c.g[A0][0] = self_this;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, func as u32, 0x0016_6f78)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderCameraTarget_GetVelocity(out, this)` (0x15f5b0): call the rider's
/// camera interface (`this->rider(+4)+0x6c0`, slot +0x10) and copy the
/// vec4 it returns to `out`. Returns `out`.
pub fn rider_camera_target_get_velocity(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let out = c.g[A0][0];
        c.g[S0][0] = out;
        let rider = lw(c, (c.g[A1][0] as u32).wrapping_add(4));
        let iface = addu(rider, 0x6c0);
        let ivt = lw(c, iface as u32);
        let m = VSlot::read(c, ivt, 0x10);
        // Registers at the call.
        c.g[A1][0] = ivt;
        c.g[V1][0] = iface;
        c.g[V0][0] = m.func;
        c.g[A0][0] = m.this(iface);
        call_ptr(c, m.func as u32, 0x0015_f5dc)?;
        c.lq(V1, c.g[V0][0] as u32);
        c.g[V0][0] = out;
        frame.restore_ra(c, 0);
        c.sq(V1, out as u32);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `RiderCameraTarget_GetPosition(out, this)` (0x15f568):
/// `Rider_GetHipsPosition(out, this->rider(+4))`; returns `out`.
pub fn rider_camera_target_get_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let out = c.g[A0][0];
        c.g[S0][0] = out;
        c.g[A1][0] = lw(c, (c.g[A1][0] as u32).wrapping_add(4));
        call_ptr(c, table_callee::RIDER_GET_HIPS_POSITION, 0x0015_f580)?;
        c.g[V0][0] = out;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `CameraBase_Update(cam)` (0x168060): `CameraBase_UpdateMatrices(cam)`.
pub fn camera_base_update(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        table_target(table_callee::CAMERA_BASE_UPDATE_MATRICES),
        0x0016_8070,
    )
}

// ---------------------------------------------------------------------------
// Rider.

const fn table_target(addr: u32) -> Target {
    Target::Table(addr)
}

/// `Rider.owner`: the `RiderBase` state machine behind a rider.
const RIDER_OWNER: u32 = 0x77c;

/// `Rider_UpdateMainState(rider)` (0x1216e0):
/// `RiderBase_UpdateMainState(rider->owner)`.
pub fn rider_update_main_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(RIDER_OWNER),
        Target::Direct(callee::RIDER_BASE_UPDATE_MAIN_STATE),
        0x0012_16f0,
    )
}

/// `Rider_SetMainState(rider, s)` (0x11fe78):
/// `RiderBase_SetMainState(rider->owner, s)`.
pub fn rider_set_main_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(RIDER_OWNER),
        Target::Direct(callee::RIDER_BASE_SET_MAIN_STATE),
        0x0011_fe88,
    )
}

/// `Rider_SetSubState(rider, s)` (0x11fec8):
/// `RiderBase_SetSubState(rider->owner, s)`.
pub fn rider_set_sub_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(RIDER_OWNER),
        Target::Direct(callee::RIDER_BASE_SET_SUB_STATE),
        0x0011_fed8,
    )
}

/// `RiderStaticState_Update(state)` (0x136958):
/// `Rider_BuildWorldMatrix(state->rider(+0))`.
pub fn rider_static_state_update(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(0),
        table_target(table_callee::RIDER_BUILD_WORLD_MATRIX),
        0x0013_6968,
    )
}

/// `Rider_ProcessControlInput(rider)` (0x121068): fill a control record on
/// the stack from the owner's controller (`owner+0xde8` sub-object, slot
/// +8), then `RiderBase_HandleSubStateInput(owner, &record)`.
pub fn rider_process_control_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let record = c.g[SP][0];
        c.g[A1][0] = record;
        frame.save_ra(c, 0x10);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        let owner = lw(c, (rider as u32).wrapping_add(RIDER_OWNER));
        let vt = lw(c, (owner as u32).wrapping_add(0xde8));
        let m = VSlot::read(c, vt, 8);
        // Registers at the call.
        c.g[A2][0] = owner;
        c.g[V0][0] = vt;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(owner);
        call_ptr(c, m.func as u32, 0x0012_1094)?;
        c.g[A0][0] = lw(c, (rider as u32).wrapping_add(RIDER_OWNER));
        c.g[A1][0] = record;
        callee::RIDER_BASE_HANDLE_SUB_STATE_INPUT.call(c, 0x0012_10a0)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_PostUpdateMainState(rider)` (0x121750):
/// `RiderBase_PostUpdateMainState(owner)`; then, if the rider's model
/// position (`+0x9d0`) differs from the vec4 at 0x4ff120 (four `c.eq.s`),
/// `RiderModel_SetPosition(rider->model(+0x780), &rider->model_pos)`.
pub fn rider_post_update_main_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.g[A0][0] = lw(c, (rider as u32).wrapping_add(RIDER_OWNER));
        callee::RIDER_BASE_POST_UPDATE_MAIN_STATE.call(c, 0x0012_1768)?;

        let r = rider as u32;
        let pos = |i: u32| r.wrapping_add(0x9d0 + 4 * i);
        let origin = |i: u32| RIDER_MODEL_ORIGIN + 4 * i;
        c.g[V1][0] = RIDER_MODEL_ORIGIN as u64;
        // Compare lane by lane, leaving f0/f1 and the FCC bit as the
        // original does (the w lane loads its operands the other way round).
        let lane = |c: &mut Cpu, f1: u32, f0: u32| -> bool {
            c.f[1] = Addr(f1).read_u32(c);
            c.f[0] = Addr(f0).read_u32(c);
            let eq = FpuF32(c.f[1]).eq_s(FpuF32(c.f[0]));
            crate::abi::set_fcc(c, eq);
            eq
        };
        let changed = if !lane(c, pos(0), origin(0)) {
            c.cycles += 1;
            true
        } else {
            c.cycles += 6;
            if !lane(c, pos(1), origin(1)) {
                true
            } else {
                c.cycles += 6;
                if !lane(c, pos(2), origin(2)) {
                    true
                } else {
                    c.cycles += 5;
                    let eq = lane(c, origin(3), pos(3));
                    if !eq {
                        c.cycles += 1;
                    }
                    !eq
                }
            }
        };
        c.g[V0][0] = changed as u64;

        c.cycles += 2;
        if changed {
            c.cycles += 4;
            c.g[A0][0] = lw(c, r.wrapping_add(0x780));
            c.g[A1][0] = addu(rider, 0x9d0);
            call_ptr(c, table_callee::RIDER_MODEL_SET_POSITION, 0x0012_17e4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_Translate(rider, delta)` (0x106538): add the vec4 `*delta` (VU0
/// `vadd`) to the position (+0x110) and model position (+0x9d0), notify
/// the collision object (`FUN_00329b40(rider+0xaa0 ptr)`), then to the
/// bounds (+0x400, +0x410).
pub fn rider_translate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_ra(c, 0);
        let delta = c.g[A1][0];
        c.g[S1][0] = delta;
        let r = rider as u32;
        let shift = |c: &mut Cpu, off: u32| {
            let v = Addr(r.wrapping_add(off)).read_vec4(c);
            let d = Addr(delta as u32).read_vec4(c);
            let s = vadd(v, d);
            Addr(r.wrapping_add(off)).write_vec4(c, s);
            // VU0 scratch as the original leaves it.
            vf_set(c, 3, v);
            vf_set(c, 4, d);
            vf_set(c, 5, s);
        };
        shift(c, 0x110);
        shift(c, 0x9d0);
        c.g[A0][0] = lw(c, r.wrapping_add(0xaa0));
        call_ptr(c, 0x0032_9b40, 0x0010_6578)?;
        shift(c, 0x400);
        frame.restore_ra(c, 0);
        shift(c, 0x410);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

fn vadd(a: Vec4, b: Vec4) -> Vec4 {
    Vec4 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w,
    }
}

fn vf_set(c: &mut Cpu, reg: usize, v: Vec4) {
    crate::ps2f::vu0_scratch::set_vf(c, reg, 0b1111, v);
}

// ---------------------------------------------------------------------------
// Graphics, memory.

/// `PS2GraphicsMan_SetViewportRect(gm, rect, arg)` (0x395370): pass the
/// float rect (x, y, w, h in f12..f15) and `arg` to the viewport
/// listener (`gm+0x10d8` vtable, slot +0xb8).
pub fn ps2_graphics_man_set_viewport_rect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        let rect = c.g[A1][0];
        c.g[V0][0] = rect;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        let gm = c.g[A0][0];
        let vt = lw(c, (gm as u32).wrapping_add(0x10d8));
        let m = VSlot::read(c, vt, 0xb8);
        let r = rect as u32;
        for (i, f) in [12usize, 13, 14, 15].into_iter().enumerate() {
            c.f[f] = Addr(r.wrapping_add(4 * i as u32)).read_u32(c);
        }
        // Registers at the call.
        c.g[V1][0] = vt;
        c.g[A2][0] = m.delta;
        c.g[A3][0] = m.func;
        c.g[A0][0] = m.this(gm);
        call_ptr(c, m.func as u32, 0x0039_53a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `operator_new_named(size, name, flags, ...)` (0x317e30):
/// `EA_MemAllocNamed(...)` with the arguments unchanged.
pub fn operator_new_named(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::EA_MEM_ALLOC_NAMED),
        0x0031_7e40,
    )
}

/// The block alignment `EA_MemAllocNamed` derives from its flags: the
/// explicit `4 << ((flags >> 24) & 0xf)` if that field is set, else 0x80
/// for flag 0x400, else 16 (8 for blocks under 16 bytes). Computed with the
/// original's branchless register sequence, whose leftovers (v0, v1, a0..a2)
/// are visible to the mutex call.
fn alloc_alignment(c: &mut Cpu, size: u64, flags: u64) -> u64 {
    let big_align = sx(((flags & 0x400) as u32) << 21);
    let big_mask = sx(((big_align as u32) as i32 >> 31) as u32); // -1 with 0x400
    let small = sx(((size as u32).wrapping_sub(0x10) as i32 >> 31) as u32); // -1 below 16
    let shift = sx((small as u32).wrapping_add(4)); // 3 or 4
    let field = sx(flags as u32 >> 24) & 0xf;
    let default = sx((big_mask as u32).wrapping_add(1) << (shift as u32 & 31)); // 0, 8 or 16
    let explicit = sx(4u32 << (field as u32 & 31));
    let no_field = sx(((field as u32).wrapping_sub(1) as i32 >> 31) as u32);
    let mut align = (big_mask & 0x80) | default;
    if no_field == 0 {
        align = explicit;
    }
    // Registers as the sequence leaves them.
    c.g[V0][0] = explicit;
    c.g[V1][0] = shift;
    c.g[A0][0] = field;
    c.g[A1][0] = default;
    c.g[A2][0] = no_field;
    align
}

/// `EA_MemAllocNamed(size, name, flags, extra)` (0x317d70): under the
/// allocator mutex, `MEM_AllocInternal(size, alignment(flags), flags,
/// extra)`. Returns the block. 48 cycles plus the callees.
pub fn ea_mem_alloc_named(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S3, 0x20);
        let flags = c.g[A2][0];
        c.g[S3][0] = flags;
        frame.save(c, S4, 0x10);
        let size = c.g[A0][0];
        c.g[S4][0] = size;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        let align = alloc_alignment(c, size, flags);
        c.g[S0][0] = align;
        c.g[S1][0] = MEM_MUTEX as u64;
        frame.save_ra(c, 0);
        let extra = c.g[A3][0];
        c.g[S2][0] = extra;

        c.g[A0][0] = MEM_MUTEX as u64;
        call_ptr(c, table_callee::EA_MUTEX_LOCK, 0x0031_7dec)?;
        c.g[A0][0] = size;
        c.g[A1][0] = align;
        c.g[A2][0] = flags;
        c.g[A3][0] = extra;
        callee::MEM_ALLOC_INTERNAL.call(c, 0x0031_7e00)?;
        let block = c.g[V0][0];
        c.g[S0][0] = block;
        c.g[A0][0] = MEM_MUTEX as u64;
        call_ptr(c, table_callee::EA_MUTEX_UNLOCK, 0x0031_7e0c)?;
        c.g[V0][0] = block;

        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `__builtin_delete(p)` (0x317e50): `MEM_FreeInternal(p)` under the
/// allocator mutex. 18 cycles plus the callees.
pub fn builtin_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = MEM_MUTEX as u64;
        let block = c.g[A0][0];
        c.g[S1][0] = block;
        frame.save_ra(c, 0);
        c.g[A0][0] = MEM_MUTEX as u64;
        call_ptr(c, table_callee::EA_MUTEX_LOCK, 0x0031_7e74)?;
        c.g[A0][0] = block;
        callee::MEM_FREE_INTERNAL.call(c, 0x0031_7e7c)?;
        c.g[A0][0] = MEM_MUTEX as u64;
        call_ptr(c, table_callee::EA_MUTEX_UNLOCK, 0x0031_7e84)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraBase_GetYaw(cam)` (0x15d780): the yaw of the camera quaternion
/// (x, y, z, w at +0x30): `s = 2(y·x − w·z)`, `k = 1 − 2(z² + x²)`, then
/// `atan(k / s)` with quadrant corrections, or a constant when `s` is 0.
/// FPU code; f20-f22 are saved in the frame, the three products in the
/// frame's first words.
pub fn camera_base_get_yaw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        c.f[8] = one.0;
        for (f, off) in [(22, 0x30), (21, 0x28), (20, 0x20)] {
            let a = frame.at(c, off);
            Addr(a).write_u32(c, c.f[f]);
        }
        let zero_slot = frame.at(c, 0xc);
        c.sw(zero_slot, 0);
        frame.save_ra(c, 0x10);
        let zero = Addr(zero_slot).read_fpu(c);

        let q = c.g[A0][0] as u32;
        let (x, y, z, w) = (
            Addr(q.wrapping_add(0x30)).read_fpu(c),
            Addr(q.wrapping_add(0x34)).read_fpu(c),
            Addr(q.wrapping_add(0x38)).read_fpu(c),
            Addr(q.wrapping_add(0x3c)).read_fpu(c),
        );
        let zz = z * z;
        let xx = x * x;
        let yx = y * x;
        let wz = w * z;
        let zz_xx = zz + xx;
        let yz = y * z;
        let wx = w * x;
        let s_half = yx - wz;
        let zz_xx2 = zz_xx + zz_xx;
        let third_half = yz + wx;
        let s = s_half + s_half;
        let k = one - zz_xx2;
        let third = third_half + third_half;
        // FPU registers as the original leaves them.
        c.f[3] = z.0;
        c.f[4] = x.0;
        c.f[5] = wx.0;
        c.f[6] = xx.0;
        c.f[7] = wz.0;
        c.f[2] = s_half.0;
        c.f[1] = zz_xx2.0;
        c.f[22] = zero.0;
        c.f[20] = s.0;
        c.f[21] = k.0;
        c.f[0] = third.0;
        let sp0 = frame.at(c, 0);
        Addr(sp0).write_fpu(c, s);
        Addr(sp0 + 4).write_fpu(c, k);
        Addr(sp0 + 8).write_fpu(c, third);

        let gp = c.g[GP][0] as u32;
        let konst = |c: &mut Cpu, i: usize| Addr(gp.wrapping_sub(YAW_CONST_GP[i])).read_fpu(c);
        let s_is_zero = s.eq_s(zero);
        crate::abi::set_fcc(c, s_is_zero);
        // The result is left in f0; `reload` is false when the last block
        // already reloaded ra and f22 itself.
        let mut reload = true;
        if s_is_zero {
            c.cycles += 4;
            let k_is_zero = k.eq_s(zero);
            crate::abi::set_fcc(c, k_is_zero);
            c.f[0] = k.0;
            if !k_is_zero {
                c.cycles += 4;
                let positive = zero.le(k);
                crate::abi::set_fcc(c, positive);
                c.f[0] = konst(c, 0).0;
                frame.restore_ra(c, 0x10);
                if positive {
                    c.cycles += 3;
                    c.f[0] = konst(c, 1).0;
                    let a = frame.at(c, 0x30);
                    c.f[22] = Addr(a).read_u32(c);
                    reload = false;
                }
            }
        } else {
            c.cycles += 7;
            c.f[12] = k.div(s).0;
            call_ptr(c, table_callee::MATH_ATAN, 0x0015_d834)?;
            let mut yaw = FpuF32(c.f[0]);
            let s_negative = s.lt(zero);
            crate::abi::set_fcc(c, s_negative);
            c.f[8] = yaw.0;
            if s_negative {
                c.cycles += 3;
                let k_positive = zero.lt(k);
                crate::abi::set_fcc(c, k_positive);
                if k_positive {
                    c.cycles += 3;
                    let fix = konst(c, 2);
                    c.f[0] = fix.0;
                    yaw = yaw + fix;
                } else {
                    c.cycles += 2;
                    let fix = konst(c, 3);
                    c.f[0] = fix.0;
                    yaw = yaw - fix;
                }
                c.f[8] = yaw.0;
            }
            c.cycles += 1;
            c.f[0] = yaw.0;
        }
        if reload {
            c.cycles += 2;
            frame.restore_ra(c, 0x10);
            let a = frame.at(c, 0x30);
            c.f[22] = Addr(a).read_u32(c);
        }
        c.cycles += 4;
        let (a21, a20) = (frame.at(c, 0x28), frame.at(c, 0x20));
        c.f[21] = Addr(a21).read_u32(c);
        c.f[20] = Addr(a20).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

/// `AnimController_PlayAnim(ctrl, anim, flags, blend f12)` (0x3128e8): pick
/// the layer for `anim` (vtable slot +0x20 of `ctrl+0x58`; 0x207 means
/// none), take the blend time from the anim table entry when `blend < 0`,
/// start it with `AnimController_PlayAnimImpl` and remember the result in
/// the controller's per-type slot (`ctrl + 4*type`).
pub fn anim_controller_play_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        let anim = c.g[A1][0];
        c.g[S2][0] = anim;
        frame.save(c, S4, 0x10);
        let ctrl = c.g[A0][0];
        c.g[S3][0] = ctrl;
        let f20_slot = frame.at(c, 0x60);
        Addr(f20_slot).write_u32(c, c.f[20]);
        let flags = c.g[A2][0];
        c.g[S4][0] = flags;
        frame.save(c, S1, 0x40);
        let mut blend = FpuF32(c.f[12]);
        c.f[20] = blend.0;
        frame.save_ra(c, 0);

        let vt = lw(c, (ctrl as u32).wrapping_add(0x58));
        let m = VSlot::read(c, vt, 0x20);
        // Registers at the call.
        c.g[V1][0] = vt;
        c.g[V0][0] = m.func;
        c.g[A0][0] = m.this(ctrl);
        call_ptr(c, m.func as u32, 0x0031_292c)?;
        let layer = c.g[V0][0];
        c.g[S0][0] = layer;
        c.g[V0][0] = 0x1b6;
        if layer != 0x207 {
            c.cycles += 5;
            c.f[0] = 0;
            let gp = c.g[GP][0] as u32;
            let use_default = blend.lt(FpuF32::ZERO);
            crate::abi::set_fcc(c, use_default);
            let mut tables = lw(c, gp.wrapping_add(ANIM_TABLES_GP));
            c.g[V1][0] = tables;
            if use_default {
                c.cycles += 4;
                c.g[A0][0] = anim;
                callee::ANIM_TABLE_GET_ENTRY.call(c, 0x0031_2958)?;
                blend = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).read_fpu(c);
                c.f[20] = blend.0;
                tables = lw(c, gp.wrapping_add(ANIM_TABLES_GP));
                c.g[V1][0] = tables;
            }
            c.cycles += 22;
            let layer_slot = addu(sx((layer as u32) << 2), tables);
            c.g[V0][0] = layer_slot;
            c.g[A0][0] = anim;
            let layer_obj = lw(c, (layer_slot as u32).wrapping_add(0x1030));
            c.g[S1][0] = layer_obj;
            callee::ANIM_TABLE_GET_ENTRY.call(c, 0x0031_2974)?;
            let ty = lw(c, (c.g[V0][0] as u32).wrapping_add(0xc));
            let seqs = lw(c, (ctrl as u32).wrapping_add(0x50));
            let type_off = sx((ty as u32) << 2);
            let param_addr = addu(type_off, ANIM_TYPE_PARAMS as u64);
            let param = lw(c, param_addr as u32);
            // Registers at the call.
            c.g[A2][0] = anim;
            c.g[V1][0] = seqs;
            c.g[S0][0] = type_off;
            c.g[V0][0] = param_addr;
            c.g[A1][0] = addu(seqs, sx((ty as u32) << 3));
            c.g[T1][0] = param;
            c.g[A3][0] = layer_obj;
            c.g[T0][0] = flags;
            c.f[12] = blend.0;
            c.g[A0][0] = ctrl;
            callee::ANIM_CONTROLLER_PLAY_ANIM_IMPL.call(c, 0x0031_29b0)?;
            let slot = addu(ctrl, type_off);
            c.g[S0][0] = slot;
            c.sw(slot as u32, c.g[V0][0]);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        let a = frame.at(c, 0x60);
        c.f[20] = Addr(a).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

/// The scissor rectangle a `PS2GraphicsMan` keeps (x, y, w, h halfwords).
const GM_SCISSOR: u32 = 0x5900;

/// `PS2GraphicsMan_SetScissorClamped(gm, x, y, w, h)` (0x376938; floats in
/// f12..f15): move a negative x/y origin onto the screen (shrinking w/h),
/// convert to integers (`cvt.w.s`), and store the rectangle clamped to the
/// screen size from the two virtual getters (slots +0x50 width, +0x58
/// height of the `gm+0x10d8` interface).
pub fn ps2_graphics_man_set_scissor_clamped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let gm = c.g[A0][0];
        c.g[S0][0] = gm;
        for (f, off) in [(23, 0x48), (22, 0x40), (21, 0x38), (20, 0x30)] {
            let a = frame.at(c, off);
            Addr(a).write_u32(c, c.f[f]);
        }
        let (mut x, mut y, mut w, mut h) = (
            FpuF32(c.f[12]),
            FpuF32(c.f[13]),
            FpuF32(c.f[14]),
            FpuF32(c.f[15]),
        );
        c.f[20] = x.0;
        c.f[21] = y.0;
        c.f[22] = w.0;
        c.f[23] = h.0;
        frame.save_ra(c, 0);

        let screen_dim = |c: &mut Cpu, off: u32, ret: u32| -> Result<u64, Unwound> {
            let vt = lw(c, (gm as u32).wrapping_add(0x10d8));
            let m = VSlot::read(c, vt, off);
            // Registers at the call.
            c.g[V1][0] = vt;
            c.g[V0][0] = m.func;
            c.g[A0][0] = m.this(gm);
            call_ptr(c, m.func as u32, ret)?;
            Ok(c.g[V0][0])
        };
        let width = screen_dim(c, 0x50, 0x0037_6980)?;
        c.g[S1][0] = width;
        let height = screen_dim(c, 0x58, 0x0037_6998)?;
        c.g[T1][0] = height;

        // A negative origin moves onto the screen, shrinking the size.
        c.f[0] = 0;
        let zero = FpuF32::ZERO;
        let x_neg = x.lt(zero);
        crate::abi::set_fcc(c, x_neg);
        if x_neg {
            c.cycles += 2;
            w = w + x;
            x = zero;
        }
        c.cycles += 3;
        let y_neg = y.lt(zero);
        crate::abi::set_fcc(c, y_neg);
        if y_neg {
            c.cycles += 2;
            h = h + y;
            y = zero;
        }
        c.f[20] = x.0;
        c.f[21] = y.0;
        c.f[22] = w.0;
        c.f[23] = h.0;

        // Integer rectangle, each part clamped (signed 64-bit compares).
        c.cycles += 17;
        let int = |v: FpuF32| sx(v.to_i32() as u32);
        let (xi, yi, wi) = (int(x), int(y), int(w));
        c.g[A2][0] = xi;
        c.g[A3][0] = yi;
        c.g[T0][0] = wi;
        let smaller = |a: u64, b: u64| (a as i64) < (b as i64);
        let max_x = sx((width as u32).wrapping_sub(1));
        let x_out = if smaller(xi, max_x) { xi } else { max_x };
        let max_y = sx((height as u32).wrapping_sub(1));
        let y_out = if smaller(yi, max_y) { yi } else { max_y };
        c.g[A1][0] = smaller(yi, max_y) as u64;
        let scissor = (gm as u32).wrapping_add(GM_SCISSOR);
        c.sh(scissor, x_out);
        c.sh(scissor + 2, y_out);
        c.g[A0][0] = y_out;
        let room_x = sx((width as u32).wrapping_sub((x_out & 0xffff) as u32));
        c.g[V0][0] = x_out & 0xffff;
        let w_out = if (wi as i64) < 0 {
            c.cycles += 1;
            0
        } else {
            c.cycles += 4;
            let less = smaller(room_x, wi);
            c.g[V0][0] = less as u64;
            if less {
                room_x
            } else {
                wi
            }
        };
        c.g[A1][0] = w_out;

        c.cycles += 6;
        let y_stored = c.lhu(scissor + 2);
        c.f[0] = h.to_i32() as u32;
        let hi = sx(c.f[0]);
        c.sh(scissor + 4, w_out);
        let room_y = sx((height as u32).wrapping_sub(y_stored as u32));
        c.g[V0][0] = y_stored;
        c.g[A0][0] = room_y;
        let h_out = if (hi as i64) < 0 {
            c.cycles += 1;
            0
        } else {
            c.cycles += 3;
            let less = smaller(room_y, hi);
            c.g[V0][0] = less as u64;
            if less {
                room_y
            } else {
                hi
            }
        };
        c.g[V1][0] = h_out;

        c.cycles += 10;
        c.sh(scissor + 6, h_out);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        for (f, off) in [(23, 0x48), (22, 0x40), (21, 0x38), (20, 0x30)] {
            let a = frame.at(c, off);
            c.f[f] = Addr(a).read_u32(c);
        }
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use crate::difftest::fakes::*;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    #[test]
    fn be_library_get_interface() {
        covered(
            ssx3_ee::s01::f_0014dd58__generated,
            super::be_library_get_interface,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 1, 1);
                let index = r.next_u32() % 8;
                c.bus.write32(GAME_GP - 0x1efc, index);
                let lib = ptr(&mut r);
                c.bus.write32(super::BE_LIBRARY + 4 * index, lib);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib + 4, vt);
            },
        );
    }

    #[test]
    fn snd_lock_unlock() {
        for (gen, hand) in [
            (
                ssx3_ee::s13::f_003b58a0__generated as fn(&mut Cpu),
                super::snd_lock as fn(&mut Cpu),
            ),
            (ssx3_ee::s13::f_003b58d8__generated, super::snd_unlock),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, 2, 1);
                c.bus.write32(super::SND_LOCK_HOOK, FAKE);
                c.bus.write32(super::SND_UNLOCK_HOOK, FAKE + 0x10);
                c.bus.write8(super::SND_LOCK_DEPTH, r.next_u32() as u8);
            });
        }
    }

    #[test]
    fn eafile_set_request_callback() {
        covered(
            ssx3_ee::s14::f_003dcf70__generated,
            super::eafile_set_request_callback,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 3, 1);
                let table = ptr(&mut r);
                c.bus.write32(super::EAFILE_REQUESTS, table);
                let slot = r.next_u32() & 0xff;
                c.g[A0][0] =
                    (r.next_u32() as u64) << 32 | (slot << 24 | r.next_u32() & 0xff_ffff) as u64;
                c.g[A1][0] = (r.next_u32() as u64) << 32 | FAKE as u64;
                let req = table + 0x30 * slot;
                crate::difftest::fill(c, &mut r, req, 12);
                if r.next_u32() % 2 == 0 {
                    c.bus.write32(req + 8, 0);
                }
            },
        );
    }

    /// A camera with an active controller whose vtable (at +0x14) points at
    /// fakes. GameCamera_ComputeView then runs for real on zeroed state.
    #[test]
    fn game_camera_update() {
        covered(
            ssx3_ee::s01::f_0015df98__generated,
            super::game_camera_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 4, 1);
                let cam = ptr(&mut r);
                c.g[A0][0] = cam as u64;
                let ctrl = ptr(&mut r);
                c.bus.write32(cam + 0xa0, ctrl);
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(ctrl + 0x14, vt);
            },
        );
    }

    #[test]
    fn game_camera_set_target_rider() {
        covered(
            ssx3_ee::s01::f_0015edc8__generated,
            super::game_camera_set_target_rider,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 5, 1);
                let cam = ptr(&mut r);
                c.g[A0][0] = cam as u64;
                let chase = ptr(&mut r);
                c.bus.write32(cam + 0xa8, chase);
                // Sometimes the state's rider field aliases cam+0xa8.
                let state = if r.next_u32() % 4 == 0 {
                    cam + 0xa4
                } else {
                    ptr(&mut r)
                };
                c.bus.write32(chase + 0x20, state);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(chase + 0x14, vt);
                if state == cam + 0xa4 {
                    // The rider written becomes the new "chase"; give it a vtable.
                    c.g[A1][0] = ptr(&mut r) as u64;
                    c.bus.write32(c.g[A1][0] as u32 + 0x14, vt);
                }
            },
        );
    }

    #[test]
    fn manual_cam_controller_update() {
        covered(
            ssx3_ee::s01::f_0015cfe8__generated,
            super::manual_cam_controller_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 6, 1);
                let ctrl = ptr(&mut r);
                c.g[A0][0] = ctrl as u64;
                let (algos, sel, algo) = (ptr(&mut r), ptr(&mut r), ptr(&mut r));
                c.bus.write32(ctrl + 8, algos);
                c.bus.write32(algos, sel);
                c.bus.write32(sel, algo);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(algo + 0x10, vt);
            },
        );
    }

    #[test]
    fn chase_camera_tick() {
        covered(
            ssx3_ee::s01::f_001624e8__generated,
            super::chase_camera_tick,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 7, 3);
                let this = ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.bus.write32(
                    this + 0x2f0,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32()
                    },
                );
                let vt = vtable(c, &mut r, &[0x20, 0x28, 0x40]);
                c.bus.write32(this + 0x10, vt);
            },
        );
    }

    #[test]
    fn chase_camera_reset() {
        covered(
            ssx3_ee::s02::f_00166f28__generated,
            super::chase_camera_reset,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 8, 2);
                let this = ptr(&mut r);
                c.g[A0][0] = this as u64;
                let vt = ptr(&mut r);
                c.bus.write16(vt + 0x18, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + 0x1c, FAKE + 0x20);
                c.bus.write32(this + 0x10, vt);
                let (target, rider) = (ptr(&mut r), ptr(&mut r));
                c.bus.write32(this + 0x30, target);
                c.bus.write32(target + 4, rider);
                let ivt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(rider + 0x6c0, ivt);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_velocity() {
        covered(
            ssx3_ee::s01::f_0015f5b0__generated,
            super::rider_camera_target_get_velocity,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 9, 1);
                c.g[A0][0] = ptr(&mut r) as u64;
                let this = ptr(&mut r);
                c.g[A1][0] = this as u64;
                let rider = ptr(&mut r);
                c.bus.write32(this + 4, rider);
                let ivt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(rider + 0x6c0, ivt);
                crate::difftest::fill(c, &mut r, DATA, 0x4_0000 / 4);
                c.bus.write32(this + 4, rider);
                c.bus.write32(rider + 0x6c0, ivt);
                c.bus.write32(ivt + 0x14, FAKE);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_position() {
        check(
            ssx3_ee::s01::f_0015f568__generated,
            super::rider_camera_target_get_position,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 10, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
                let this = ptr(&mut r);
                c.g[A1][0] = this as u64;
                let (rider, model, bones) = (ptr(&mut r), ptr(&mut r), ptr(&mut r));
                c.bus.write32(this + 4, rider);
                c.bus.write32(rider + 0x780, model);
                c.bus.write32(rider + 0x89c, r.next_u32() % 32);
                c.bus.write32(model + 0x2c, bones);
                crate::difftest::fill(c, &mut r, bones, 32 * 8);
            },
        );
    }

    #[test]
    fn camera_base_update() {
        check(
            ssx3_ee::s02::f_00168060__generated,
            super::camera_base_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 11, 0);
                let cam = ptr(&mut r);
                c.g[A0][0] = cam as u64;
                crate::difftest::fill(c, &mut r, cam, 0x40);
                for i in 0..16 {
                    c.bus.write32(cam + 0x20 + 4 * i, r.f32(4.0));
                }
            },
        );
    }

    /// The RiderBase state switches read their jump tables from empty RAM:
    /// the 0 entry lands in `jump_out`, i.e. the fake (which may unwind).
    fn rider_owner(c: &mut Cpu, r: &mut Rng) -> u32 {
        let rider = ptr(r);
        c.g[A0][0] = rider as u64;
        let owner = ptr(r);
        c.bus.write32(rider + super::RIDER_OWNER, owner);
        for off in [0xde0, 0xde4] {
            let s = if r.next_u32() % 4 == 0 {
                r.next_u32()
            } else {
                r.next_u32() % 6
            };
            c.bus.write32(owner + off, s);
        }
        rider
    }

    #[test]
    fn rider_update_main_state() {
        covered(
            ssx3_ee::s00::f_001216e0__generated,
            super::rider_update_main_state,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 12, 1);
                rider_owner(c, &mut r);
            },
        );
    }

    #[test]
    fn rider_set_main_and_sub_state() {
        for (gen, hand) in [
            (
                ssx3_ee::s00::f_0011fe78__generated as fn(&mut Cpu),
                super::rider_set_main_state as fn(&mut Cpu),
            ),
            (
                ssx3_ee::s00::f_0011fec8__generated,
                super::rider_set_sub_state,
            ),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, 13, 2);
                rider_owner(c, &mut r);
                c.g[A1][0] = (r.next_u32() % 7) as u64;
            });
        }
    }

    #[test]
    fn rider_static_state_update() {
        check(
            ssx3_ee::s01::f_00136958__generated,
            super::rider_static_state_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 14, 0);
                let state = ptr(&mut r);
                c.g[A0][0] = state as u64;
                let rider = ptr(&mut r);
                c.bus.write32(state, rider);
                for i in 0..4 {
                    c.bus.write32(rider + 0x110 + 4 * i, r.f32(1e4));
                    c.bus.write32(rider + 0x120 + 4 * i, r.f32(1.0));
                }
            },
        );
    }

    #[test]
    fn rider_process_control_input() {
        covered(
            ssx3_ee::s00::f_00121068__generated,
            super::rider_process_control_input,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 15, 2);
                let rider = rider_owner(c, &mut r);
                let owner = c.bus.read32(rider + super::RIDER_OWNER);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(owner + 0xde8, vt);
            },
        );
    }

    #[test]
    fn rider_post_update_main_state() {
        covered(
            ssx3_ee::s00::f_00121750__generated,
            super::rider_post_update_main_state,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 16, 1);
                let rider = rider_owner(c, &mut r);
                // An empty model (no nodes to move).
                let model = ptr(&mut r);
                c.bus.write32(rider + 0x780, model);
                c.bus.write32(model + 8, 0);
                // Equal up to a random lane (sometimes all equal), with -0/+0 and
                // denormal pairs that compare equal.
                let diff_from = r.next_u32() % 5;
                for i in 0..4 {
                    let v = r.edge_f32();
                    c.bus.write32(super::RIDER_MODEL_ORIGIN + 4 * i, v);
                    let w = if i < diff_from {
                        match r.next_u32() % 3 {
                            0 if v & 0x7f80_0000 == 0 => r.next_u32() & 0x807f_ffff,
                            _ => v,
                        }
                    } else {
                        r.edge_f32()
                    };
                    c.bus.write32(rider + 0x9d0 + 4 * i, w);
                }
            },
        );
    }

    #[test]
    fn rider_translate() {
        check(
            ssx3_ee::s00::f_00106538__generated,
            super::rider_translate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 17, 1);
                let rider = ptr(&mut r);
                c.g[A0][0] = rider as u64;
                let delta = if r.next_u32() % 8 == 0 {
                    rider + 0x110
                } else {
                    ptr(&mut r)
                };
                c.g[A1][0] = delta as u64;
                for off in [0x110, 0x9d0, 0x400, 0x410] {
                    for i in 0..4 {
                        c.bus.write32(rider + off + 4 * i, r.edge_f32());
                    }
                }
                if delta != rider + 0x110 {
                    for i in 0..4 {
                        c.bus.write32(delta + 4 * i, r.edge_f32());
                    }
                }
                // The collision tree: a root and up to three children.
                // (Away from the rider so its stores can't change the count.)
                let tree = 0x00a0_0000 + (r.next_u32() & 0xfff0);
                c.bus.write32(rider + 0xaa0, tree);
                for i in 0..32 {
                    c.bus.write32(tree + 0x10 + 4 * i, r.edge_f32());
                }
                c.bus.write32(tree + 0x2c, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_viewport_rect() {
        covered(
            ssx3_ee::s13::f_00395370__generated,
            super::ps2_graphics_man_set_viewport_rect,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 18, 1);
                let gm = ptr(&mut r);
                c.g[A0][0] = gm as u64;
                let rect = ptr(&mut r);
                c.g[A1][0] = rect as u64;
                crate::difftest::fill(c, &mut r, rect, 4);
                let vt = vtable(c, &mut r, &[0xb8]);
                c.bus.write32(gm + 0x10d8, vt);
            },
        );
    }
    fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.g[A0][0] = match r.next_u32() % 4 {
            0 => (r.next_u32() % 0x20) as u64,
            1 => (r.next_u32() % 0x400) as u64,
            2 => (r.next_u32() % 0x10_0000) as u64,
            _ => r.next_u32() as u64 | (r.next_u32() as u64) << 32,
        };
        let mut flags = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
        if r.next_u32() % 2 == 0 {
            flags &= !0x0f00_0000;
        }
        c.g[A2][0] = flags;
        if r.next_u32() % 2 == 0 {
            c.g[A3][0] = 0;
        }
        // Sometimes the mutex is already held by "this thread" (fake
        // GetThreadId result 0x42 for the first syscall).
        if r.next_u32() % 2 == 0 {
            c.bus.write32(super::MEM_MUTEX + 4, 0x42);
            c.bus.write32(LOG_RESULTS, 0x42);
            c.bus.write32(super::MEM_MUTEX + 8, r.next_u32() % 4);
        }
    }

    #[test]
    fn ea_mem_alloc_named() {
        for (gen, hand) in [
            (
                ssx3_ee::s10::f_00317d70__generated as fn(&mut Cpu),
                super::ea_mem_alloc_named as fn(&mut Cpu),
            ),
            (
                ssx3_ee::s10::f_00317e30__generated,
                super::operator_new_named,
            ),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, 19, 6);
                alloc_setup(c, &mut r);
            });
        }
    }

    #[test]
    fn builtin_delete() {
        covered(
            ssx3_ee::s10::f_00317e50__generated,
            super::builtin_delete,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 20, 6);
                alloc_setup(c, &mut r);
                c.g[A0][0] = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    ptr(&mut r) as u64
                };
            },
        );
    }

    #[test]
    fn camera_base_get_yaw() {
        check(
            ssx3_ee::s01::f_0015d780__generated,
            super::camera_base_get_yaw,
            1000,
            &|c, seed| {
                let mut r = machine(c, seed, 21, 1);
                let cam = ptr(&mut r);
                c.g[A0][0] = cam as u64;
                let mut q = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)];
                match seed % 6 {
                    // s = 0 with k = 1, k = 0, k = -1, k = 1 - 2z^2.
                    0 => (q[0], q[2]) = (0, 0),
                    1 => (q[0], q[2], q[3]) = (0x3f00_0000, 0x3f00_0000, q[1]),
                    2 => (q[0], q[2], q[3]) = (0, 0x3f80_0000, 0),
                    3 => (q[0], q[3]) = (0x8000_0000, 0),
                    4 => q = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    _ => {}
                }
                for (i, v) in q.into_iter().enumerate() {
                    c.bus.write32(cam + 0x30 + 4 * i as u32, v);
                }
                for off in super::YAW_CONST_GP {
                    c.bus.write32(GAME_GP - off, r.f32(4.0));
                }
            },
        );
    }

    #[test]
    fn anim_controller_play_anim() {
        covered(
            ssx3_ee::s10::f_003128e8__generated,
            super::anim_controller_play_anim,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 22, 2);
                let ctrl = ptr(&mut r);
                c.g[A0][0] = ctrl as u64;
                let anim = r.next_u32() % 64;
                c.g[A1][0] = anim as u64;
                c.f[12] = if r.next_u32() % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.0)
                };
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(ctrl + 0x58, vt);
                let layer = if r.next_u32() % 4 == 0 {
                    0x207
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write32(LOG_RESULTS, layer);
                if layer != 0x207 {
                    // PlayAnimImpl runs real code on this synthetic controller,
                    // which would not terminate; the first fake it reaches (the
                    // GetThreadId syscall of the allocator mutex, via
                    // AnimSeqList_PushNew -> operator new) unwinds. Its register
                    // dump and frame show the arguments PlayAnim passed.
                    c.bus.write32(LOG_UNWIND_AT, 1);
                }
                let tables = ptr(&mut r);
                c.bus.write32(GAME_GP + super::ANIM_TABLES_GP, tables);
                c.bus.write32(tables + 4 * layer + 0x1030, ptr(&mut r));
                // The anim table entry (0x1c bytes at 0x446990 + 0x1c*anim).
                let entry = 0x0044_6990 + 0x1c * anim;
                c.bus.write32(entry + 0xc, r.next_u32() % 4);
                c.bus.write32(entry + 0x10, r.f32(2.0));
                c.bus.write32(ctrl + 0x50, ptr(&mut r));
            },
        );
    }
    #[test]
    fn ps2_graphics_man_set_scissor_clamped() {
        covered(
            ssx3_ee::s12::f_00376938__generated,
            super::ps2_graphics_man_set_scissor_clamped,
            1000,
            &|c, seed| {
                let mut r = machine(c, seed, 23, 2);
                let gm = ptr(&mut r);
                c.g[A0][0] = gm as u64;
                let vt = vtable(c, &mut r, &[0x50, 0x58]);
                c.bus.write32(gm + 0x10d8, vt);
                // Screen sizes (sometimes 0 or negative).
                for i in 0..2 {
                    let v = match r.next_u32() % 4 {
                        0 => r.next_u32() % 3,
                        1 => r.next_u32(),
                        _ => 256 + r.next_u32() % 512,
                    };
                    c.bus.write32(LOG_RESULTS + 4 * i, v);
                }
                for f in 12..16 {
                    c.f[f] = match r.next_u32() % 4 {
                        0 => r.edge_f32(),
                        1 => r.f32(1e6),
                        _ => r.f32(700.0),
                    };
                }
                crate::difftest::fill(c, &mut r, gm + super::GM_SCISSOR, 2);
            },
        );
    }
}
