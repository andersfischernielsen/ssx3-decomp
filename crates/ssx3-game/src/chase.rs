//! Work package D3 chase camera (docs/investigation/phase2-candidates-2.md,
//! layouts in docs/investigation/structs/camera.md).
//!
//! The race camera is a `ChaseCameraAlgorithm`: every frame its leaf class's
//! `Update` gathers the rider's state and runs a fixed pipeline of steps
//! (follow distance, camera height, FOV, target height, jump curves, rail
//! pull-back, low-speed hold, ...), each of which nudges the look-at point
//! (+0x20) or the desired eye (+0x40) and ends with
//! `ChaseCameraAlgorithm_CheckDistanceReset`, which snaps the camera back
//! behind the rider when the eye got too close or too far. The
//! `ChaseCamController` picks which algorithm runs (auto, override, script,
//! spoke mode) and `GameViewMan` updates one `GameCamera` per view.
//!
//! The steps are VU0 macro code mixed with FPU code. Besides the usual
//! contract (memory incl. dead stack slots, hard registers, FCC, cycles per
//! block, calls), they use the VU0 accumulator and read Q back into a GPR:
//!
//! - [`Acc`] follows the accumulator op by op (ADDA/MULA/MADDA/MSUBA, and
//!   MADD/MSUB reading it), with the VU's one-rounding MADD semantics, and
//!   writes its final lanes back (VU0 context savers copy ACC to memory).
//! - Q: in macro mode DIV/SQRT/RSQRT results land in Q at once (each macro
//!   op flushes the pipeline, `waitq` is a no-op), so `cfc2 rt, vi22` reads
//!   the value just computed: [`q_sqrt`] / [`q_div`] compute it, store it in
//!   Q and return it; the GPR gets it sign-extended ([`cfc2_q`]). The
//!   invalid/divide-by-zero status bits they set are soft state.
//!
//! VU0 registers are written back at the end of each replacement (or before
//! a call, so the callee sees them) with the lanes the original wrote.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame, VSlot};
use crate::camera::{ChaseCamController, ChaseCameraAlgorithm};
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_8150, check_distance_reset, "ChaseCameraAlgorithm_CheckDistanceReset"),
    (0x0016_6550, init_from_rider, "ChaseCameraAlgorithm_InitFromRider"),
    (0x0015_d880, camera_base_get_pitch, "CameraBase_GetPitch"),
    (0x0016_3010, apply_follow_distance, "ChaseCameraAlgorithm_ApplyFollowDistance"),
    (0x0016_2b90, apply_target_height, "ChaseCameraAlgorithm_ApplyTargetHeight"),
    (0x0016_3270, apply_camera_height, "ChaseCameraAlgorithm_ApplyCameraHeight"),
    (0x0016_33b0, apply_fov, "ChaseCameraAlgorithm_ApplyFov"),
    (0x0016_3450, apply_handplant_hold, "ChaseCameraAlgorithm_ApplyHandplantHold"),
    (0x0016_2998, update_handplant_fade, "ChaseCameraAlgorithm_UpdateHandplantFade"),
    (0x0015_d020, is_script_camera_active, "ChaseCamController_IsScriptCameraActive"),
    (0x0016_2060, set_override_algorithm, "ChaseCamController_SetOverrideAlgorithm"),
    (0x0022_e840, game_view_man_update_cameras, "GameViewMan_UpdateCameras"),
    (0x0022_e920, game_view_man_all_views_ready, "GameViewMan_AllViewsReady"),
    (0x0015_e030, game_camera_reset_default_chase, "GameCamera_ResetDefaultChase"),
    (0x0016_3158, apply_side_offset, "ChaseCameraAlgorithm_ApplySideOffset"),
    (0x0016_41c0, apply_rail_pullback, "ChaseCameraAlgorithm_ApplyRailPullback"),
    (0x0016_46a0, apply_rail_align, "ChaseCameraAlgorithm_ApplyRailAlign"),
    (0x0016_43a8, apply_low_speed_hold, "ChaseCameraAlgorithm_ApplyLowSpeedHold"),
    (0x0016_2a20, update_air_transitions, "ChaseCameraAlgorithm_UpdateAirTransitions"),
    (0x0016_6640, compute_angles, "ChaseCameraAlgorithm_ComputeAngles"),
    (0x0016_6228, post_update, "ChaseCameraAlgorithm_PostUpdate"),
    (0x0016_1ab0, update_auto_mode, "ChaseCamController_UpdateAutoMode"),
    (0x0015_e460, game_camera_update_shake, "GameCamera_UpdateShake"),
    (0x0016_1bb8, chase_cam_controller_update, "ChaseCamController_Update"),
];

// ---------------------------------------------------------------- VU0 helpers

/// Lane masks in the VU `dest` order.
pub const X: u8 = 0b1000;
pub const XYZW: u8 = 0b1111;

/// All four lanes set to `v` (a broadcast operand `ft.field`).
#[inline]
pub fn splat(v: VuF32) -> Vec4 {
    Vec4 { x: v, y: v, z: v, w: v }
}

#[inline]
fn lanes(v: Vec4) -> [VuF32; 4] {
    [v.x, v.y, v.z, v.w]
}

#[inline]
fn from_lanes(l: [VuF32; 4]) -> Vec4 {
    Vec4 { x: l[0], y: l[1], z: l[2], w: l[3] }
}

/// Lane-wise `f(a, b)`.
#[inline]
fn zip(a: Vec4, b: Vec4, f: impl Fn(VuF32, VuF32) -> VuF32) -> Vec4 {
    let (a, b) = (lanes(a), lanes(b));
    from_lanes(std::array::from_fn(|i| f(a[i], b[i])))
}

/// `vadd`, `vsub`, `vmul` on all lanes.
#[inline]
pub fn vadd(a: Vec4, b: Vec4) -> Vec4 {
    zip(a, b, |x, y| x + y)
}
#[inline]
pub fn vsub(a: Vec4, b: Vec4) -> Vec4 {
    zip(a, b, |x, y| x - y)
}
#[inline]
pub fn vmul(a: Vec4, b: Vec4) -> Vec4 {
    zip(a, b, |x, y| x * y)
}

/// `new` in the lanes of `dest`, `old` elsewhere (a masked VU write).
#[inline]
pub fn masked(old: Vec4, new: Vec4, dest: u8) -> Vec4 {
    let (o, n) = (lanes(old), lanes(new));
    from_lanes(std::array::from_fn(|i| if dest & (8 >> i) != 0 { n[i] } else { o[i] }))
}

/// A VF register's current value.
#[inline]
pub fn vf(c: &Cpu, reg: usize) -> Vec4 {
    let v = c.bus.plat.vf(reg);
    Vec4 { x: VuF32(v[0]), y: VuF32(v[1]), z: VuF32(v[2]), w: VuF32(v[3]) }
}

/// `qmtc2 gpr, vf`: the 128-bit GPR image as four lanes.
#[inline]
pub fn qmtc2(g: [u64; 2]) -> Vec4 {
    Vec4::from_bits(g[0] as u128 | (g[1] as u128) << 64)
}

/// The VU0 accumulator ACC, followed op by op. Every op rounds like the
/// VU instruction (MADDA/MSUBA/MADD/MSUB: one rounding of the product, one
/// of the sum, as `VuF32::madd`). Load it with [`Acc::load`] at the first
/// accumulator op and [`Acc::store`] it where the original's last one ran
/// (lanes outside an op's `dest` keep their value).
#[derive(Clone, Copy, Debug)]
pub struct Acc(pub Vec4);

impl Acc {
    /// The machine's ACC.
    pub fn load(c: &Cpu) -> Acc {
        let v = c.bus.plat.acc();
        Acc(Vec4 { x: VuF32(v[0]), y: VuF32(v[1]), z: VuF32(v[2]), w: VuF32(v[3]) })
    }
    /// Write all four lanes back.
    pub fn store(self, c: &mut Cpu) {
        vu0_scratch::set_acc(c, XYZW, self.0);
    }
    /// `ADDA`: ACC = a + b.
    pub fn adda(&mut self, dest: u8, a: Vec4, b: Vec4) {
        self.0 = masked(self.0, vadd(a, b), dest);
    }
    /// `SUBA`: ACC = a - b.
    pub fn suba(&mut self, dest: u8, a: Vec4, b: Vec4) {
        self.0 = masked(self.0, vsub(a, b), dest);
    }
    /// `MULA`: ACC = a * b.
    pub fn mula(&mut self, dest: u8, a: Vec4, b: Vec4) {
        self.0 = masked(self.0, vmul(a, b), dest);
    }
    /// `MADDA`: ACC += a * b.
    pub fn madda(&mut self, dest: u8, a: Vec4, b: Vec4) {
        let r = self.madd(a, b);
        self.0 = masked(self.0, r, dest);
    }
    /// `MSUBA`: ACC -= a * b.
    pub fn msuba(&mut self, dest: u8, a: Vec4, b: Vec4) {
        let r = self.msub(a, b);
        self.0 = masked(self.0, r, dest);
    }
    /// `OPMULA.xyz ACC, fs, ft`: the first half of the cross product
    /// `fs × ft` (ACC.x = fs.y·ft.z, ACC.y = fs.z·ft.x, ACC.z = fs.x·ft.y).
    pub fn opmula(&mut self, fs: Vec4, ft: Vec4) {
        self.0.x = fs.y * ft.z;
        self.0.y = fs.z * ft.x;
        self.0.z = fs.x * ft.y;
    }
    /// `OPMSUB.xyz fd, fs, ft`: the second half (`ACC − fs.yzx · ft.zxy`);
    /// returns x, y, z (w is not written: take it from fd).
    pub fn opmsub(&self, fs: Vec4, ft: Vec4) -> Vec4 {
        Vec4 {
            x: VuF32::msub(self.0.x, fs.y, ft.z),
            y: VuF32::msub(self.0.y, fs.z, ft.x),
            z: VuF32::msub(self.0.z, fs.x, ft.y),
            w: VuF32::ZERO,
        }
    }
    /// `MADD` (the result, all lanes; mask it with [`masked`]): ACC + a * b.
    pub fn madd(&self, a: Vec4, b: Vec4) -> Vec4 {
        let (acc, a, b) = (lanes(self.0), lanes(a), lanes(b));
        from_lanes(std::array::from_fn(|i| VuF32::madd(acc[i], a[i], b[i])))
    }
    /// `MSUB`: ACC - a * b.
    pub fn msub(&self, a: Vec4, b: Vec4) -> Vec4 {
        let (acc, a, b) = (lanes(self.0), lanes(a), lanes(b));
        from_lanes(std::array::from_fn(|i| VuF32::msub(acc[i], a[i], b[i])))
    }
}

/// `vsqrt q, ft.f; vwaitq`: Q = sqrt(x), written to the machine's Q (macro
/// mode: available at once). Returns it.
pub fn q_sqrt(c: &mut Cpu, x: VuF32) -> VuF32 {
    let q = VuF32::sqrt(x);
    vu0_scratch::set_q(c, q);
    q
}

/// `vdiv q, fs.f, ft.f; vwaitq`: Q = a / b.
pub fn q_div(c: &mut Cpu, a: VuF32, b: VuF32) -> VuF32 {
    let q = VuF32::div(a, b);
    vu0_scratch::set_q(c, q);
    q
}

/// `cfc2 rt, vi22`: the GPR image of Q (sign-extended).
#[inline]
pub fn cfc2_q(q: VuF32) -> u64 {
    sx(q.0)
}

/// The game's four-lane dot product idiom (VU0 macro code from the vector
/// library): with the operands in vf3 and vf5 (vf3 twice for a length),
/// `vadd.x vf6, vf0, vf0w` (1.0), `vmul vf4, vf3, vf5`,
/// `vadda.x ACC, vf4, vf4y`, `vmadda.x ACC, vf6, vf4z`,
/// `vmadd.x vf4, vf6, vf4w`: a·b over x, y, z **and** w in vf4.x. Writes
/// vf4, vf6.x and ACC.x; returns the whole vf4 image (the original
/// sometimes reads it back with `qmfc2`).
pub fn dot4(c: &mut Cpu, a: Vec4, b: Vec4) -> Vec4 {
    let one = VuF32::ONE; // vf0.x + vf0.w
    vu0_scratch::set_vf(c, 6, X, splat(VuF32::ZERO + one));
    let prod = vmul(a, b);
    let mut acc = Acc::load(c);
    acc.adda(X, prod, splat(prod.y));
    acc.madda(X, splat(one), splat(prod.z));
    let sum = acc.madd(splat(one), splat(prod.w));
    acc.store(c);
    let vf4 = masked(prod, sum, X);
    vu0_scratch::set_vf(c, 4, XYZW, vf4);
    vf4
}

/// `|v|²` by [`dot4`] (`vmul vf4, vf3, vf3`): returns vf4.x.
pub fn length4_sq(c: &mut Cpu, v: Vec4) -> VuF32 {
    dot4(c, v, v).x
}

/// [`length4_sq`] followed by `vsqrt Q, vf4x; vwaitq`: |v|, also left in Q.
pub fn length4(c: &mut Cpu, v: Vec4) -> VuF32 {
    let sq = length4_sq(c, v);
    q_sqrt(c, sq)
}

/// `vrsqrt Q, vf0w, ft.f; vwaitq`: Q = 1 / sqrt(x).
pub fn q_rsqrt(c: &mut Cpu, x: VuF32) -> VuF32 {
    let q = VuF32::rsqrt(VuF32::ONE, x);
    vu0_scratch::set_q(c, q);
    q
}

// ---------------------------------------------------------------- views and globals

/// World up (static vec4 at 0x4ff160), used to raise the eye.
const WORLD_UP: Addr = Addr(0x004f_f160);

/// `RiderCameraTarget` (8 bytes): vptr, `Rider*`. Its vtable slots are
/// gcc 2.9x entries at `8 * slot`.
#[derive(Clone, Copy, Debug)]
pub struct RiderCameraTarget(pub Addr);

impl RiderCameraTarget {
    pub fn vptr(self) -> Addr {
        self.0
    }
    pub fn rider(self) -> Addr {
        self.0.off(4)
    }
}

/// The chase algorithm's fields this package uses (on top of
/// `camera::ChaseCameraAlgorithm`).
trait ChaseFields {
    fn desired_eye(self) -> Addr;
    fn reset_forward(self) -> Addr;
    fn needs_reset(self) -> Addr;
}

impl ChaseFields for ChaseCameraAlgorithm {
    /// Desired eye (vec4, +0x40).
    fn desired_eye(self) -> Addr {
        self.0.off(0x40)
    }
    /// Rider forward at reset / chase heading (vec4, +0x80).
    fn reset_forward(self) -> Addr {
        self.0.off(0x80)
    }
    /// "Needs reset" flag (i32, +0x2f0; vtable slot 11).
    fn needs_reset(self) -> Addr {
        self.0.off(0x2f0)
    }
}

/// gp-relative float constant (`lwc1 f, -off(gp)`).
fn gp_const(c: &mut Cpu, off: u32) -> FpuF32 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(off)).read_fpu(c)
}

/// `c.le.s a, b`: the compare result, also latched in FCC.
fn c_le_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let le = a.le(b);
    abi::set_fcc(c, le);
    le
}

/// `lw` through the CPU path (sign-extended).
#[inline]
fn lw(c: &mut Cpu, a: Addr) -> u64 {
    a.lw(c)
}

mod callee {
    use super::{ee, Callee};

    pub const CHASE_CAM_CONTROLLER_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::ChaseCamController_SetAlgorithm };
    pub const SET_DEFAULT_ALGORITHM: Callee =
        Callee { addr: 0x0016_1ef0, f: ee::s01::ChaseCamController_SetDefaultAlgorithm };
    pub const ENTER_SPOKE_MODE: Callee =
        Callee { addr: 0x0016_21a8, f: ee::s01::ChaseCamController_EnterSpokeMode };
    pub const EXIT_SPOKE_MODE: Callee =
        Callee { addr: 0x0016_2218, f: ee::s01::ChaseCamController_ExitSpokeMode };
    pub const GAME_CAMERA_ADD_SHAKE: Callee = Callee { addr: 0x0015_e360, f: ee::s01::GameCamera_AddShake };
    pub const START_SHAKE: Callee = Callee { addr: 0x0016_56b0, f: ee::s01::ChaseCameraAlgorithm_StartShake };
    pub const APPLY_SHAKE: Callee = Callee { addr: 0x0016_5938, f: ee::s01::ChaseCameraAlgorithm_ApplyShake };
    pub const UPDATE_AUTO_MODE: Callee =
        Callee { addr: 0x0016_1ab0, f: ee::s01::ChaseCamController_UpdateAutoMode };
    pub const ALGORITHM_LIST_REMOVE: Callee = Callee { addr: 0x0015_ca50, f: ee::s01::AlgorithmList_Remove };
    pub const SET_OVERRIDE_ALGORITHM: Callee =
        Callee { addr: 0x0016_2060, f: ee::s01::ChaseCamController_SetOverrideAlgorithm };
    pub const COMPUTE_ANGLES: Callee =
        Callee { addr: 0x0016_6640, f: ee::s02::ChaseCameraAlgorithm_ComputeAngles };
    /// `FUN_001662a0(algo, f12..f15)`: applies the four tuning floats.
    pub const APPLY_TUNING: Callee = Callee { addr: 0x0016_62a0, f: ee::s02::f_001662a0 };
    pub const FUN_00166530: Callee = Callee { addr: 0x0016_6530, f: ee::s02::f_00166530 };
    pub const BUILD_OUTPUT_VIEW: Callee =
        Callee { addr: 0x0016_6f90, f: ee::s02::ChaseCameraAlgorithm_BuildOutputView };
}

/// Calls the generated code makes through the function table.
mod table_callee {
    pub const CHASE_CAMERA_ALGORITHM_RESET: u32 = 0x0016_6f28;
    pub const MATH_ASIN: u32 = 0x0031_c128;
    pub const MATH_ATAN: u32 = 0x0031_c228;
    pub const CHECK_DISTANCE_RESET: u32 = 0x0016_8150;
    pub const AI_GET_FRAME_COUNTER: u32 = 0x0012_98c8;
    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;
}

// ---------------------------------------------------------------- distance reset

/// Distance below which the eye counts as too close (20.0).
const MIN_EYE_DISTANCE: FpuF32 = FpuF32(0x41a0_0000);
/// gp offset of the distance above which the eye counts as too far.
const MAX_EYE_DISTANCE_GP: u32 = 0x6778;
/// How far behind / above the rider a reset puts the eye (200, 50).
const RESET_BACK: FpuF32 = FpuF32(0x4348_0000);
const RESET_UP: FpuF32 = FpuF32(0x4248_0000);

/// `ChaseCameraAlgorithm_CheckDistanceReset(algo)` (0x168150): when the
/// desired eye is closer than 20 or farther than the gp limit from the
/// look-at point (four-lane distance), snap the camera: look-at = the
/// target's position (slot 1), eye = position − forward × 200 + up × 50,
/// and raise the "needs reset" flag.
///
/// Frame 0xa0: diff at +0x00, forward/diff at +0x10, the position out
/// parameter at +0x20, the intermediates at +0x30..+0x70.
pub fn check_distance_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xa0);
        c.g[AT][0] = MIN_EYE_DISTANCE.0 as u64;
        c.f[0] = MIN_EYE_DISTANCE.0;
        frame.save(c, S0, 0x90);
        frame.save_ra(c, 0x80);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        c.g[S0][0] = c.g[A0][0];
        let sp = Addr(frame.at(c, 0));

        // How far the desired eye is from the look-at point.
        let diff = vsub(algo.look_at().read_vec4(c), algo.desired_eye().read_vec4(c));
        sp.off(0x10).write_vec4(c, diff);
        sp.write_vec4(c, diff);
        abi::set_gpr128(c, V0, diff.to_bits());
        vu0_scratch::set_vf(c, 3, XYZW, diff);
        vu0_scratch::set_vf(c, 5, XYZW, diff);
        let dist = FpuF32(length4(c, diff).0);
        c.g[V1][0] = cfc2_q(VuF32(dist.0));
        c.f[1] = dist.0;

        let too_close = abi::c_lt_s(c, dist, MIN_EYE_DISTANCE);
        let reset = too_close || {
            c.cycles += 5;
            let max = gp_const(c, MAX_EYE_DISTANCE_GP);
            c.f[0] = max.0;
            abi::c_lt_s(c, max, dist)
        };
        if reset {
            if !too_close {
                c.cycles += 1;
            }
            c.cycles += 47;
            reset_behind_rider(c, algo, sp)?;
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 3;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// The reset of [`check_distance_reset`] (block 0x1681d0).
fn reset_behind_rider(c: &mut Cpu, algo: ChaseCameraAlgorithm, sp: Addr) -> Flow {
    let target = RiderCameraTarget(Addr(lw(c, algo.target()) as u32));
    c.g[A1][0] = sx(target.0 .0);
    c.g[A0][0] = sx(sp.off(0x20).0);
    let rider = Addr(lw(c, target.rider()) as u32);
    let forward = rider.off(0x1b0).read_vec4(c);
    sp.off(0x10).write_vec4(c, forward);
    let vt = lw(c, target.vptr());
    let get_position = VSlot::read(c, vt, 0x8);
    // Registers at the call: v0 keeps the forward vector's upper half.
    abi::set_gpr128(c, V0, forward.to_bits());
    c.g[V0][0] = get_position.delta;
    c.g[V1][0] = vt;
    c.g[A2][0] = get_position.func;
    c.g[A1][0] = get_position.this(c.g[A1][0]);
    call_ptr(c, get_position.func as u32, 0x0016_81f4)?;

    c.g[AT][0] = RESET_BACK.0 as u64;
    c.f[0] = RESET_BACK.0;
    c.g[A0][0] = 1;
    let back = splat(VuF32(RESET_BACK.0));
    let behind = vmul(forward, back);
    sp.off(0x60).write_vec4(c, behind);
    c.g[AT][0] = RESET_UP.0 as u64;
    c.f[1] = RESET_UP.0;
    sp.off(0x50).write_vec4(c, behind);
    let position = sp.off(0x20).read_vec4(c);
    let back_point = vsub(position, behind);
    sp.off(0x60).write_vec4(c, back_point);
    sp.off(0x40).write_vec4(c, back_point);
    c.g[V0][0] = sx(WORLD_UP.0);
    c.g[V1][0] = sx(RESET_UP.0);
    let lift = vmul(WORLD_UP.read_vec4(c), splat(VuF32(RESET_UP.0)));
    sp.off(0x70).write_vec4(c, lift);
    sp.off(0x60).write_vec4(c, lift);
    let eye = vadd(back_point, lift);
    sp.off(0x70).write_vec4(c, eye);
    abi::set_gpr128(c, V1, eye.to_bits());
    abi::set_gpr128(c, V0, position.to_bits());
    algo.needs_reset().write_u32(c, 1);
    algo.desired_eye().write_vec4(c, eye);
    algo.look_at().write_vec4(c, position);
    sp.off(0x30).write_vec4(c, eye);
    vu0_scratch::set_vf(c, 3, XYZW, back_point);
    vu0_scratch::set_vf(c, 4, XYZW, lift);
    vu0_scratch::set_vf(c, 5, XYZW, eye);
    Ok(())
}

// ---------------------------------------------------------------- reset from rider

/// `ChaseCameraAlgorithm_InitFromRider(algo, back f12, down f13)` (0x166550):
/// chase heading +0x80 = normalize(forward × back − up × down), with the
/// rider's forward from the target (slot 4). One 59-cycle block; f20/f21
/// hold the two arguments across the call.
pub fn init_from_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x30);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        c.g[S0][0] = c.g[A0][0];
        let sp = Addr(frame.at(c, 0));
        sp.off(0x58).write_u32(c, c.f[21]);
        c.g[A0][0] = sx(sp.off(0x10).0);
        sp.off(0x50).write_u32(c, c.f[20]);
        let (back, down) = (abi::farg(c, 0), abi::farg(c, 1));
        c.f[21] = down.0;
        c.f[20] = back.0;

        // forward = target->GetForward() (slot 4) into sp+0x10.
        let target = RiderCameraTarget(Addr(lw(c, algo.target()) as u32));
        c.g[A2][0] = sx(target.0 .0);
        let vt = lw(c, target.vptr());
        c.g[V0][0] = vt;
        let get_forward = VSlot::read(c, vt, 0x20);
        c.g[V1][0] = get_forward.func;
        c.g[A1][0] = get_forward.this(c.g[A2][0]);
        call_ptr(c, get_forward.func as u32, 0x0016_658c)?;

        c.g[A0][0] = abi::mfc1(back);
        let forward = sp.off(0x10).read_vec4(c);
        let ahead = vmul(forward, splat(VuF32(back.0)));
        sp.off(0x20).write_vec4(c, ahead);
        frame.restore_ra(c, 0x30);
        c.f[20] = sp.off(0x50).read_u32(c);
        algo.reset_forward().write_vec4(c, ahead);
        sp.write_vec4(c, ahead);
        let drop = vmul(WORLD_UP.read_vec4(c), splat(VuF32(down.0)));
        sp.off(0x10).write_vec4(c, drop);
        abi::set_gpr128(c, V0, drop.to_bits());
        c.f[21] = sp.off(0x58).read_u32(c);
        sp.write_vec4(c, drop);
        let heading = vsub(ahead, drop);
        algo.reset_forward().write_vec4(c, heading);
        vu0_scratch::set_vf(c, 3, XYZW, heading);
        let len = length4(c, heading);
        c.g[V0][0] = cfc2_q(len);
        c.f[0] = len.0;
        c.g[V1][0] = cfc2_q(len);
        let vf3 = qmtc2(c.g[V1]);
        let inv = q_div(c, VuF32::ONE, vf3.x);
        let unit = vmul(heading, splat(inv));
        algo.reset_forward().write_vec4(c, unit);
        vu0_scratch::set_vf(c, 3, XYZW, vf3);
        vu0_scratch::set_vf(c, 4, XYZW, heading);
        vu0_scratch::set_vf(c, 5, XYZW, unit);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- camera pitch

/// `CameraBase_GetPitch(cam)` (0x15d880): the pitch of the camera
/// quaternion (x, y, z, w at +0x30): the rotated axis
/// `a = (2(yx − wz), 1 − 2(z² + x²), 2(yz + wx), 0)` is normalised by its
/// VU0 length and `asin(a.z / |a|)` returned (`Math_Asin`). FPU code
/// building the vector in the frame, then the VU0 length idiom. 41 cycles.
pub fn camera_base_get_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x20);
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        frame.save_ra(c, 0x10);
        let q = Addr(abi::arg(c, 0)).off(0x30);
        let (x, y, z, w) = (q.read_fpu(c), q.off(4).read_fpu(c), q.off(8).read_fpu(c), q.off(0xc).read_fpu(c));
        let zz = z * z;
        let xx = x * x;
        let yz = y * z;
        let sp = Addr(frame.at(c, 0));
        sp.off(0xc).sw(c, 0);
        let wx = w * x;
        let zz_xx = zz + xx;
        let yx = y * x;
        let wz = w * z;
        let up_half = yz + wx;
        let zz_xx2 = zz_xx + zz_xx;
        let side_half = yx - wz;
        let up = up_half + up_half;
        let fwd = one - zz_xx2;
        let side = side_half + side_half;
        sp.off(8).write_fpu(c, up);
        sp.off(4).write_fpu(c, fwd);
        sp.write_fpu(c, side);
        // FPU registers as the original leaves them.
        for (f, v) in [(0, side), (1, up), (2, x), (3, wz), (4, zz_xx2), (5, z), (6, wx), (7, fwd), (8, xx)] {
            c.f[f] = v.0;
        }

        let axis = sp.read_vec4(c);
        vu0_scratch::set_vf(c, 3, XYZW, axis);
        let len = length4(c, axis);
        c.g[V0][0] = cfc2_q(len);
        c.f[12] = up.div(FpuF32(len.0)).0;
        call_ptr(c, table_callee::MATH_ASIN, 0x0015_d918)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- controller

/// `GameCamera_ResetDefaultChase(cam)` (0x15e030): reset the embedded
/// blend algorithm (`ChaseCameraAlgorithm_Reset(cam + 0xc0)`). 7 cycles.
pub fn game_camera_reset_default_chase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xc0));
        call_ptr(c, table_callee::CHASE_CAMERA_ALGORITHM_RESET, 0x0015_e040)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Flag bits of `ChaseCamController::flags`.
const FLAG_OVERRIDE: u64 = 1;
const FLAG_SCRIPT: u64 = 2;
/// Algorithm ids with special meaning to the controller.
const ALGO_CLEAR_OVERRIDE: u64 = 0x4c;
const ALGO_TIMED_OVERRIDE: u64 = 0x5d;
const ALGO_DEFAULT_2_ALT: u64 = 0x3c;

/// `ChaseCamController_IsScriptCameraActive(ctl)` (0x15d020): a script
/// camera runs (flag bit 1) and the override id (+0x2c) is neither the
/// timed override 0x5d nor 0x3c. Leaf.
pub fn is_script_camera_active(c: &mut Cpu) {
    c.cycles += 4;
    let ctl = ChaseCamController(Addr(abi::arg(c, 0)));
    let flags = lw(c, ctl.flags());
    c.g[V0][0] = flags & FLAG_SCRIPT;
    c.g[V1][0] = 0;
    if flags & FLAG_SCRIPT != 0 {
        c.cycles += 4;
        let id = lw(c, ctl.0.off(0x2c));
        c.g[A0][0] = id;
        c.g[V0][0] = id ^ ALGO_DEFAULT_2_ALT;
        if id != ALGO_TIMED_OVERRIDE {
            c.cycles += 1;
            c.g[V1][0] = (id != ALGO_DEFAULT_2_ALT) as u64;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `ChaseCamController_SetOverrideAlgorithm(ctl, id, rate)` (0x162060):
/// id 0x4c clears the override (back to the saved id +0x24), any other id
/// becomes the override (+0x28, blend rate +0x34). The algorithm is
/// switched at once unless a script camera is running (then only the timed
/// override 0x5d passes).
pub fn set_override_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let ctl = ChaseCamController(Addr(abi::arg(c, 0)));
        c.g[V0][0] = ALGO_CLEAR_OVERRIDE;
        frame.save_ra(c, 0);
        let flags = if c.g[A1][0] == ALGO_CLEAR_OVERRIDE {
            c.cycles += 5;
            let flags = lw(c, ctl.flags());
            c.g[V1][0] = !FLAG_OVERRIDE;
            c.g[A1][0] = lw(c, ctl.restore_algorithm());
            flags & !FLAG_OVERRIDE
        } else {
            c.cycles += 4;
            let flags = lw(c, ctl.flags());
            ctl.algorithm_0x28().sw(c, c.g[A1][0]);
            ctl.override_blend_rate().sw(c, c.g[A2][0]);
            flags | FLAG_OVERRIDE
        };
        c.cycles += 5;
        ctl.flags().sw(c, flags);
        let flags = lw(c, ctl.flags());
        let switch = if flags & FLAG_SCRIPT == 0 {
            1
        } else {
            c.cycles += 3;
            (lw(c, ctl.0.off(0x2c)) == ALGO_TIMED_OVERRIDE) as u64
        };
        c.g[V0][0] = switch;
        c.cycles += 2;
        frame.restore_ra(c, 0);
        if switch != 0 {
            c.cycles += 3;
            callee::CHASE_CAM_CONTROLLER_SET_ALGORITHM.call(c, 0x0016_20c4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- views

/// `GameViewMan` (0x38, see camera.md): the camera loops read their count
/// at +0x10 and the `GameCamera*` array from +0x04.
#[derive(Clone, Copy, Debug)]
pub struct GameViewMan(pub Addr);

impl GameViewMan {
    /// Number of cameras to update (i32 at +0x10).
    pub fn camera_count(self) -> Addr {
        self.0.off(0x10)
    }
    /// `GameCamera*` i (array from +0x04).
    pub fn camera(self, i: u32) -> Addr {
        self.0.off(4 + 4 * i)
    }
}

/// `GameViewMan_AllViewsReady(vm)` (0x22e920): 1 when every camera's
/// +0xb4 flag is set (or there are none), else 0. Structured loop with an
/// event check at the back edge (head 0x22e938).
pub fn game_view_man_all_views_ready(c: &mut Cpu) {
    const HEAD: u32 = 0x0022_e938;
    c.cycles += 3;
    let vm = GameViewMan(Addr(abi::arg(c, 0)));
    let count = lw(c, vm.camera_count());
    c.g[V1][0] = count;
    c.g[A1][0] = 0;
    if count != 0 {
        c.cycles += 3;
        c.g[A2][0] = count;
        c.g[A0][0] = sx(vm.camera(0).0);
        loop {
            c.cycles += 4;
            let slot = Addr(c.g[A0][0] as u32);
            let cam = lw(c, slot);
            c.g[V0][0] = cam;
            let ready = lw(c, Addr(cam as u32).off(0xb4));
            c.g[V1][0] = ready;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if ready == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            let more = c.g[A1][0] < c.g[A2][0];
            c.g[V0][0] = more as u64;
            c.g[A0][0] = sx(slot.0.wrapping_add(4));
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `GameViewMan_UpdateCameras(vm)` (0x22e840): call every camera's
/// `Update` (GameCamera vtable at +0x90, slot +0x18). Structured loop with
/// a call and an event check at the back edge (head 0x22e870).
pub fn game_view_man_update_cameras(c: &mut Cpu) {
    const HEAD: u32 = 0x0022_e870;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let vm = GameViewMan(Addr(abi::arg(c, 0)));
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let count = lw(c, vm.camera_count());
        c.g[V0][0] = count;
        c.g[S1][0] = 0;
        if count != 0 {
            c.cycles += 3;
            c.g[S0][0] = sx(vm.camera(0).0);
            c.g[A1][0] = lw(c, Addr(c.g[S0][0] as u32));
            loop {
                c.cycles += 11;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                let cam = c.g[A1][0];
                let vt = lw(c, Addr(cam as u32).off(0x90));
                c.g[V0][0] = vt;
                let update = VSlot::read(c, vt, 0x18);
                c.g[V1][0] = update.func;
                c.g[A0][0] = update.this(cam);
                call_ptr(c, update.func as u32, 0x0022_e88c)?;
                let count = lw(c, vm.camera_count());
                let more = c.g[S1][0] < count;
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[A1][0] = lw(c, Addr(c.g[S0][0] as u32));
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, HEAD);
                }
            }
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

// ---------------------------------------------------------------- tuning steps

/// `RiderCamState`, the stack record `GatherRiderState` fills and the
/// steps read (camera.md).
#[derive(Clone, Copy, Debug)]
pub struct RiderCamState(pub Addr);

impl RiderCamState {
    /// Rider position (vec4).
    pub fn rider_position(self) -> Addr {
        self.0.off(0x10)
    }
    /// Chase direction (vec4): look-at to eye, unnormalised.
    pub fn chase_dir(self) -> Addr {
        self.0.off(0x30)
    }
    /// Rider main state (i32).
    pub fn main_state(self) -> Addr {
        self.0.off(0x40)
    }
    /// Speed (f32, cm/s).
    pub fn speed(self) -> Addr {
        self.0.off(0x44)
    }
    /// Handplant fade weight (f32).
    pub fn handplant_fade(self) -> Addr {
        self.0.off(0x48)
    }
    /// Boost strength (f32, target slot 17).
    pub fn boost(self) -> Addr {
        self.0.off(0x4c)
    }
    /// Target slot 18 (`Rider+0x220`, f32).
    pub fn slope_term(self) -> Addr {
        self.0.off(0x54)
    }
    /// "Is Wipeout / Handplant / Air / Railslide" flags (i32).
    pub fn is_handplant(self) -> Addr {
        self.0.off(0x5c)
    }
    pub fn is_air(self) -> Addr {
        self.0.off(0x60)
    }
    pub fn is_railslide(self) -> Addr {
        self.0.off(0x64)
    }
}

/// Smoothed chase fields (`ChaseCameraAlgorithm`, camera.md).
trait ChaseFilters {
    fn fov(self) -> Addr;
    fn eye_drop(self) -> Addr;
    fn follow_dist(self) -> Addr;
    fn fov_scale(self) -> Addr;
    fn look_at_height(self) -> Addr;
    fn eye_slope_bonus(self) -> Addr;
    fn look_at_slope_term(self) -> Addr;
}

impl ChaseFilters for ChaseCameraAlgorithm {
    /// Current FOV (f32, radians, header +0x00).
    fn fov(self) -> Addr {
        self.0
    }
    /// +0x1d4: smoothed amount the eye is lowered along world up.
    fn eye_drop(self) -> Addr {
        self.0.off(0x1d4)
    }
    /// +0x1d0: smoothed eye distance behind the look-at point.
    fn follow_dist(self) -> Addr {
        self.0.off(0x1d0)
    }
    /// +0x1dc: smoothed FOV scale.
    fn fov_scale(self) -> Addr {
        self.0.off(0x1dc)
    }
    /// +0x1e4: smoothed look-at lift along world up.
    fn look_at_height(self) -> Addr {
        self.0.off(0x1e4)
    }
    /// +0x204: speed-and-slope eye bonus, held while airborne.
    fn eye_slope_bonus(self) -> Addr {
        self.0.off(0x204)
    }
    /// +0x208: slope term (slot 18), held while airborne.
    fn look_at_slope_term(self) -> Addr {
        self.0.off(0x208)
    }
}

/// gp offsets of the step constants.
const SPEED_50_KMH_GP: u32 = 0x6ae8; // 1388.9 cm/s
const TARGET_SPEED_SCALE_GP: u32 = 0x6ae4; // 5.15e-4
const FOV_SPEED_SCALE_GP: u32 = 0x6ab0; // 1e-7
const FOV_SMOOTHING_GROUND_GP: u32 = 0x6aac; // 0.999
const QUARTER_PI_GP: u32 = 0x6aa8; // π/4

/// The tail every step ends with: `ChaseCameraAlgorithm_CheckDistanceReset
/// (algo)` returning to `ret`.
fn check_distance(c: &mut Cpu, ret: u32) -> Flow {
    call_ptr(c, table_callee::CHECK_DISTANCE_RESET, ret)
}

/// `look_at`/`eye` `+= dir × k` through the stack, as the steps do it:
/// `vmul vf5, vf4(dir), vf3(k).x` → sp+0x10 → (GPR) → sp+0x00, then
/// `vadd vf5, vf3(point), vf4(offset)` (or `vsub`) written to `point`.
/// `k_gpr` is the GPR image moved into vf3 (`qmtc2`); returns the new point.
fn offset_point(c: &mut Cpu, sp: Addr, point: Addr, dir: Vec4, k_gpr: [u64; 2], sub: bool) -> Vec4 {
    let k = qmtc2(k_gpr);
    let off = vmul(dir, splat(k.x));
    sp.off(0x10).write_vec4(c, off);
    sp.write_vec4(c, off);
    let p = point.read_vec4(c);
    let moved = if sub { vsub(p, off) } else { vadd(p, off) };
    point.write_vec4(c, moved);
    vu0_scratch::set_vf(c, 3, XYZW, p);
    vu0_scratch::set_vf(c, 4, XYZW, off);
    vu0_scratch::set_vf(c, 5, XYZW, moved);
    abi::set_gpr128(c, V0, off.to_bits());
    moved
}

/// `ChaseCameraAlgorithm_ApplyTargetHeight(algo, st, base f12, speedGain
/// f13, slope f14, boost f15, smoothing f16)` (0x162b90): step 10, lift the
/// look-at point along world up by the smoothed
/// `base + clamp01((v − 1388.9) × 5.15e-4) × speedGain + boost × f15 +
/// slope × slot18` (slot 18 held while airborne, +0x208).
pub fn apply_target_height(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        let speed_50 = gp_const(c, SPEED_50_KMH_GP);
        frame.save_ra(c, 0x20);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        c.g[A2][0] = c.g[A0][0];
        let zero = FpuF32(c.g[0][0] as u32);
        let excess = st.speed().read_fpu(c) - speed_50;
        c.f[1] = excess.0;
        // over = max(excess, 0) by a `c.le.s` branch.
        let positive = zero.le(excess);
        abi::set_fcc(c, positive);
        let over = if positive {
            c.cycles += 1;
            excess
        } else {
            zero
        };

        c.cycles += 7;
        let one = FpuF32::ONE;
        let ramp = (over * gp_const(c, TARGET_SPEED_SCALE_GP)).min(one);
        let air = lw(c, st.is_air());
        c.g[V0][0] = air;
        let slope = if air == 0 {
            c.cycles += 1;
            st.slope_term().read_fpu(c)
        } else {
            c.cycles += 2;
            algo.look_at_slope_term().read_fpu(c)
        };

        c.cycles += 35;
        let (base, speed_gain, slope_gain, boost_gain, smoothing) =
            (abi::farg(c, 0), abi::farg(c, 1), abi::farg(c, 2), abi::farg(c, 3), abi::farg(c, 4));
        algo.look_at_slope_term().write_fpu(c, slope);
        let speed_lift = ramp * speed_gain;
        let slope_lift = slope_gain * slope;
        c.g[AT][0] = one.0 as u64;
        let boost = st.boost().read_fpu(c);
        let lift = base + speed_lift;
        let old = algo.look_at_height().read_fpu(c);
        let boost_lift = boost * boost_gain;
        let keep = one - smoothing;
        c.g[A0][0] = c.g[A2][0];
        let old_part = smoothing * old;
        let lift = lift + boost_lift;
        let lift = lift + slope_lift;
        let new_part = keep * lift;
        let height = old_part + new_part;
        algo.look_at_height().write_fpu(c, height);
        for (f, v) in [(0, height), (1, lift), (2, boost_lift), (3, new_part), (4, slope_lift)] {
            c.f[f] = v.0;
        }
        c.g[V1][0] = abi::mfc1(height);
        let up = WORLD_UP.read_vec4(c);
        let sp = Addr(frame.at(c, 0));
        offset_point(c, sp, algo.look_at(), up, c.g[V1], false);
        check_distance(c, 0x0016_2c68)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCameraAlgorithm_ApplyFov(algo, st, speedSqGain f12, base f13,
/// boostWiden f14, smoothing f15)` (0x1633b0): step 8, the FOV scale
/// `t = speedSqGain × (v × 1e-7) × v + 1 + (boostWiden − 1) × boost` is
/// smoothed into +0x1dc (held in Air, 0.999 outside Ground/Air) and
/// `fov = base × π/4 × fovScale`.
pub fn apply_fov(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        let scale = gp_const(c, FOV_SPEED_SCALE_GP);
        frame.save_ra(c, 0);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        c.g[V1][0] = c.g[A0][0];
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        c.g[V0][0] = 1;
        let speed = st.speed().read_fpu(c);
        let widen = abi::farg(c, 2) - one;
        let boost = st.boost().read_fpu(c);
        let scaled = speed * scale;
        let state = lw(c, st.main_state());
        c.g[A2][0] = state;
        let widen = widen * boost;
        let t = abi::farg(c, 0) * scaled;
        let t = t * speed;
        let t = t + one;
        let t = t + widen;
        c.f[14] = widen.0;
        c.f[12] = t.0;
        c.f[1] = boost.0;
        c.f[2] = speed.0;
        c.f[3] = one.0;
        c.f[0] = scaled.0;
        // Smoothing: hold in Air, 0.999 unless on the ground.
        c.cycles += 2;
        if state == 1 {
            c.f[15] = one.0;
        } else if state != 0 {
            c.cycles += 1;
            c.f[15] = gp_const(c, FOV_SMOOTHING_GROUND_GP).0;
        }

        c.cycles += 17;
        let smoothing = abi::farg(c, 3);
        c.g[A0][0] = c.g[V1][0];
        let old = algo.fov_scale().read_fpu(c);
        let keep = one - smoothing;
        let quarter_pi = gp_const(c, QUARTER_PI_GP);
        let old_part = smoothing * old;
        let base = abi::farg(c, 1) * quarter_pi;
        let new_part = keep * t;
        let fov_scale = old_part + new_part;
        let fov = base * fov_scale;
        algo.fov_scale().write_fpu(c, fov_scale);
        algo.fov().write_fpu(c, fov);
        c.f[0] = new_part.0;
        c.f[1] = fov_scale.0;
        c.f[2] = fov.0;
        check_distance(c, 0x0016_3444)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const DIST_SPEED_SCALE_GP: u32 = 0x6ac4; // 1e-4

/// More boost-distance fields (`ChaseCameraAlgorithm` +0x1e8 / +0x1ec).
fn boost_dist_bonus(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x1e8)
}
fn last_boost(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x1ec)
}

/// `ChaseCameraAlgorithm_ApplyFollowDistance(algo, st, base f12, gain
/// f13, offset f14, boostGain f15, boostDecay f16, smoothGrow f17,
/// smoothShrink f18)` (0x163010): step 6, the follow distance
/// `t = min(5 × base, max(base + gain × ((v − offset) × 1e-4) × √(v − offset),
/// base)) + boostBonus` is smoothed into +0x1d0 (f18 when shrinking, f17
/// when growing; picked through a pointer to their stack slots) and the eye
/// placed that far behind the look-at point along the chase direction:
/// `eye = look_at − dir × (followDist / |dir|)`. The boost bonus is
/// `boostGain × √boost` when the boost rose, then decays by `boostDecay`.
pub fn apply_follow_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        let zero = FpuF32(0);
        c.f[6] = 0;
        frame.save_ra(c, 0x40);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        c.g[V1][0] = c.g[A0][0];
        let [base, gain, offset, boost_gain, boost_decay, grow, shrink] =
            std::array::from_fn(|i| abi::farg(c, i));
        c.f[4] = base.0;
        let dir = st.chase_dir().read_vec4(c);
        abi::set_gpr128(c, V0, dir.to_bits());
        let sp = Addr(frame.at(c, 0));
        sp.off(0x34).write_fpu(c, grow);
        sp.off(0x30).write_fpu(c, shrink);
        let excess = st.speed().read_fpu(c) - offset;
        let negative = abi::c_lt_s(c, excess, zero);
        sp.write_vec4(c, dir);
        let excess = if negative {
            c.cycles += 1;
            zero
        } else {
            excess
        };

        // Distance from speed, clamped to [base, 5 × base].
        c.cycles += 15;
        let scale = gp_const(c, DIST_SPEED_SCALE_GP);
        let root = excess.sqrt();
        let five = FpuF32(0x40a0_0000);
        c.g[AT][0] = five.0 as u64;
        let scaled = excess * scale;
        let boost = st.boost().read_fpu(c);
        let last = last_boost(algo).read_fpu(c);
        let max_dist = base * five;
        let term = gain * scaled;
        let rose = abi::c_lt_s(c, last, boost);
        let term = term * root;
        let raw = base + term;
        let at_least_base = raw.max(base);
        let mut dist = max_dist.min(at_least_base);
        if rose {
            c.cycles += 3;
            let bonus = boost_gain * boost.sqrt();
            boost_dist_bonus(algo).write_fpu(c, bonus);
        }
        c.f[1] = term.0;
        c.f[2] = max_dist.0;
        c.f[3] = root.0;
        c.f[5] = boost.0;

        // Boost bonus decay; pick the smoothing factor.
        c.cycles += 12;
        let bonus = boost_dist_bonus(algo).read_fpu(c) - boost_decay;
        let mut factor_slot = sp.off(0x34);
        c.g[V0][0] = sx(factor_slot.0);
        let old = algo.follow_dist().read_fpu(c);
        let boost_now = st.boost().read_fpu(c);
        last_boost(algo).write_fpu(c, boost_now);
        let bonus = bonus.max(zero);
        dist = dist + bonus;
        let shrinking = abi::c_lt_s(c, dist, old);
        boost_dist_bonus(algo).write_fpu(c, bonus);
        if shrinking {
            c.cycles += 1;
            factor_slot = sp.off(0x30);
            c.g[V0][0] = sx(factor_slot.0);
        }

        c.cycles += 36;
        let s = factor_slot.read_fpu(c);
        c.g[A0][0] = c.g[V1][0];
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        vu0_scratch::set_vf(c, 3, XYZW, dir);
        let len = length4(c, dir);
        c.g[V0][0] = cfc2_q(len);
        let old_part = s * old;
        let new_part = (one - s) * dist;
        let follow = old_part + new_part;
        algo.follow_dist().write_fpu(c, follow);
        let k = follow.div(FpuF32(len.0));
        c.g[V0][0] = abi::mfc1(k);
        let back = vmul(dir, splat(VuF32(k.0)));
        sp.write_vec4(c, back);
        let look = algo.look_at().read_vec4(c);
        let eye = vsub(look, back);
        sp.off(0x20).write_vec4(c, eye);
        abi::set_gpr128(c, V0, eye.to_bits());
        algo.desired_eye().write_vec4(c, eye);
        sp.off(0x10).write_vec4(c, eye);
        vu0_scratch::set_vf(c, 3, XYZW, look);
        vu0_scratch::set_vf(c, 4, XYZW, back);
        vu0_scratch::set_vf(c, 5, XYZW, eye);
        for (f, v) in [(0, new_part), (1, follow), (2, k), (3, s), (4, old), (6, zero), (12, dist)] {
            c.f[f] = v.0;
        }
        check_distance(c, 0x0016_314c)?;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

const SPEED_30_KMH_GP: u32 = 0x6abc; // 833.3 cm/s
const SPEED_60_KMH_GP: u32 = 0x6ab8; // 1666.7 cm/s
const SPEED_50_KMH_HEIGHT_GP: u32 = 0x6ac0; // 1388.9 cm/s
const HEIGHT_SPEED_SCALE_GP: u32 = 0x6ab4; // 1e-4

/// `ChaseCameraAlgorithm_ApplyCameraHeight(algo, st, gain f12, offset f13,
/// boostGain f14, slope f15, smoothing f16)` (0x163270): step 7, lower the
/// eye along world up by the smoothed
/// `drop = gain × ((v − 1388.9) × 1e-4) × √(v − 1388.9) − offset +
/// max(slopeBonus, boostGain × √boost)`, where the slope bonus
/// `ramp² × slope × (slot18 − 0.5)` (ramp: 0 below 30 km/h, 1 above 60)
/// is held while airborne (+0x204). f15 is overwritten with it.
pub fn apply_camera_height(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        let speed_50 = gp_const(c, SPEED_50_KMH_HEIGHT_GP);
        frame.save_ra(c, 0x20);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        c.g[A2][0] = c.g[A0][0];
        let zero = FpuF32(0);
        let speed = st.speed().read_fpu(c);
        let excess = speed - speed_50;
        let positive = zero.le(excess);
        abi::set_fcc(c, positive);
        let over = if positive {
            c.cycles += 1;
            excess
        } else {
            zero
        };

        // Speed ramp between 30 and 60 km/h.
        c.cycles += 5;
        let speed_30 = gp_const(c, SPEED_30_KMH_GP);
        let fast = speed_30.le(speed);
        abi::set_fcc(c, fast);
        let ramp = if !fast {
            c.cycles += 1;
            zero
        } else {
            c.cycles += 4;
            let speed_60 = gp_const(c, SPEED_60_KMH_GP);
            let below_60 = speed.le(speed_60);
            abi::set_fcc(c, below_60);
            c.cycles += 4;
            if below_60 {
                (speed - speed_30).div(speed_30)
            } else {
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                FpuF32::ONE
            }
        };
        let air = lw(c, st.is_air());
        c.g[V0][0] = air;

        // Slope bonus (held while airborne).
        c.cycles += 2;
        let ramp_sq = ramp * ramp;
        let slope_bonus = if air != 0 {
            c.cycles += 2;
            algo.eye_slope_bonus().read_fpu(c)
        } else {
            c.cycles += 6;
            let slope = st.slope_term().read_fpu(c);
            let k = ramp_sq * abi::farg(c, 3);
            let half = FpuF32(0x3f00_0000);
            c.g[AT][0] = half.0 as u64;
            k * (slope - half)
        };
        c.f[15] = slope_bonus.0;

        c.cycles += 39;
        let scale = gp_const(c, HEIGHT_SPEED_SCALE_GP);
        let root = over.sqrt();
        algo.eye_slope_bonus().write_fpu(c, slope_bonus);
        let scaled = over * scale;
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        let boost_root = st.boost().read_fpu(c).sqrt();
        let (gain, offset, boost_gain, smoothing) = (abi::farg(c, 0), abi::farg(c, 1), abi::farg(c, 2), abi::farg(c, 4));
        let keep = one - smoothing;
        let speed_drop = gain * scaled;
        let old = algo.eye_drop().read_fpu(c);
        let boost_drop = boost_gain * boost_root;
        c.g[A0][0] = c.g[A2][0];
        let old_part = smoothing * old;
        let speed_drop = speed_drop * root;
        let bonus = slope_bonus.max(boost_drop);
        let drop = speed_drop - offset;
        let drop = drop + bonus;
        let new_part = keep * drop;
        let eye_drop = old_part + new_part;
        algo.eye_drop().write_fpu(c, eye_drop);
        for (f, v) in [(0, bonus), (1, eye_drop), (2, drop), (3, new_part), (4, root), (5, over)] {
            c.f[f] = v.0;
        }
        c.g[V1][0] = abi::mfc1(eye_drop);
        let up = WORLD_UP.read_vec4(c);
        let sp = Addr(frame.at(c, 0));
        offset_point(c, sp, algo.desired_eye(), up, c.g[V1], true);
        check_distance(c, 0x0016_33a0)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Frames after a handplant during which the fade weight stays 1.
const HANDPLANT_FADE_HOLD: i64 = 120;
/// Where `framesSinceHandplant` stops counting.
const HANDPLANT_FADE_MAX: i64 = 820;
const HANDPLANT_FADE_RATE_GP: u32 = 0x6af4; // 1/700
const FRAME_TIME_GP: u32 = 0x6aa4; // 1/60

/// Handplant fields of `ChaseCameraAlgorithm`.
fn handplant_offset(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0xb0)
}
fn handplant_rider_pos(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0xa0)
}
fn handplant_blend_time(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x1e0)
}
fn handplant_lift(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2b8)
}
fn frames_since_handplant(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2c0)
}

/// `ChaseCameraAlgorithm_UpdateHandplantFade(algo, st)` (0x162998): step
/// 2, count the frames since the last handplant (+0x2c0, up to 820) and
/// set the fade weight `st+0x48`: 1 during and for 120 frames after a
/// handplant, then falling by 1/700 per frame down to 0. Leaf.
pub fn update_handplant_fade(c: &mut Cpu) {
    c.cycles += 3;
    let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
    let st = RiderCamState(Addr(abi::arg(c, 1)));
    let one = FpuF32::ONE;
    let handplant = lw(c, st.is_handplant());
    c.g[V0][0] = handplant;
    let fade = if handplant != 0 {
        frames_since_handplant(algo).sw(c, 0);
        c.cycles += 2;
        c.g[AT][0] = one.0 as u64;
        one
    } else {
        c.cycles += 4;
        let frames = lw(c, frames_since_handplant(algo));
        c.g[V1][0] = frames;
        c.g[V0][0] = sx((frames as u32).wrapping_add(1));
        if (frames as i64) < HANDPLANT_FADE_MAX {
            c.cycles += 1;
            frames_since_handplant(algo).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        let frames = lw(c, frames_since_handplant(algo));
        c.g[A0][0] = frames;
        c.g[AT][0] = one.0 as u64;
        let holding = (frames as i64) < HANDPLANT_FADE_HOLD;
        c.g[V0][0] = holding as u64;
        let mut fade = one;
        if !holding {
            c.cycles += 8;
            let hold = FpuF32(0x42f0_0000); // 120.0
            c.g[AT][0] = hold.0 as u64;
            let rate = gp_const(c, HANDPLANT_FADE_RATE_GP);
            let elapsed = FpuF32::from_i32(frames as i32) - hold;
            let drop = elapsed * rate;
            fade = one - drop;
            c.f[1] = hold.0;
            c.f[2] = rate.0;
            c.f[0] = drop.0;
        }
        c.cycles += 5;
        let zero = FpuF32(0);
        c.f[0] = 0;
        if abi::c_lt_s(c, fade, zero) {
            c.cycles += 3;
            zero
        } else {
            c.cycles += 2;
            fade
        }
    };
    if handplant != 0 {
        c.cycles += 3;
    }
    c.f[3] = fade.0;
    st.handplant_fade().write_fpu(c, fade);
    abi::ret(c);
}

/// `ChaseCameraAlgorithm_ApplyHandplantHold(algo, st, blendTime f15,
/// height f16, smoothing f17, decay f18)` (0x163450): step 9. During a
/// handplant the look-at → eye offset is saved (+0xb0) and the blend timer
/// (+0x1e0) cleared; afterwards the timer advances by 1/60 (while < 5) and,
/// for `blendTime` seconds, the eye eases from `look_at − saved` back to
/// where the other steps put it (`e = min(3u² − 2u³, 1)`). The rider
/// position is kept in +0xa0, and an extra eye height +0x2b8 (smoothed
/// toward `height` during a handplant, decaying by `decay` otherwise) is
/// added to the eye's z.
pub fn apply_handplant_hold(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x60);
        frame.save_ra(c, 0x50);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        let sp = Addr(frame.at(c, 0));
        let handplant = lw(c, st.is_handplant());
        c.g[V0][0] = handplant;
        c.g[A2][0] = c.g[A0][0];
        let one = FpuF32::ONE;
        // Eye blending happens: the quad last in v0 goes to sp+0.
        let mut spill = None;
        if handplant != 0 {
            c.cycles += 8;
            let look = algo.look_at().read_vec4(c);
            let eye = algo.desired_eye().read_vec4(c);
            let offset = vsub(look, eye);
            sp.off(0x10).write_vec4(c, offset);
            handplant_blend_time(algo).sw(c, 0);
            handplant_offset(algo).write_vec4(c, offset);
            vu0_scratch::set_vf(c, 3, XYZW, look);
            vu0_scratch::set_vf(c, 4, XYZW, eye);
            vu0_scratch::set_vf(c, 5, XYZW, offset);
            spill = Some(offset);
        } else {
            c.cycles += 7;
            let t = handplant_blend_time(algo).read_fpu(c);
            let five = FpuF32(0x40a0_0000);
            c.g[AT][0] = five.0 as u64;
            c.f[0] = five.0;
            c.f[2] = t.0;
            if abi::c_lt_s(c, t, five) {
                c.cycles += 6;
                let step = gp_const(c, FRAME_TIME_GP);
                c.f[0] = step.0;
                let t = t + step;
                c.f[2] = t.0;
                let blend = abi::farg(c, 3);
                let blending = abi::c_lt_s(c, t, blend);
                handplant_blend_time(algo).write_fpu(c, t);
                if blending {
                    spill = Some(blend_eye(c, algo, sp, t.div(blend)));
                } else {
                    c.cycles += 1;
                }
            }
        }
        if let Some(q) = spill {
            c.cycles += 2;
            sp.write_vec4(c, q);
        }
        let rider = st.rider_position().read_vec4(c);
        abi::set_gpr128(c, V0, rider.to_bits());

        c.cycles += 4;
        handplant_rider_pos(algo).write_vec4(c, rider);
        let handplant = lw(c, st.is_handplant());
        c.g[V1][0] = handplant;
        let old = handplant_lift(algo).read_fpu(c);
        let (height, smoothing, decay) = (abi::farg(c, 4), abi::farg(c, 5), abi::farg(c, 6));
        let lift = if handplant == 0 {
            c.cycles += 1;
            c.f[0] = old.0;
            decay * old
        } else {
            c.cycles += 8;
            c.g[AT][0] = one.0 as u64;
            let new_part = (one - smoothing) * height;
            c.f[0] = new_part.0;
            smoothing * old + new_part
        };

        c.cycles += 9;
        let z = algo.desired_eye().off(8);
        c.g[A0][0] = c.g[A2][0];
        handplant_lift(algo).write_fpu(c, lift);
        let ez = z.read_fpu(c) + lift;
        z.write_fpu(c, ez);
        c.f[0] = ez.0;
        c.f[1] = lift.0;
        check_distance(c, 0x0016_35b0)?;
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// The blend of [`apply_handplant_hold`] (blocks 0x1634f0..): with
/// `u = t / blendTime`, `e = min(3u² − 2u³, 1)`,
/// `eye = (look_at − saved) × (1 − e) + eye × e`. Returns the quad the
/// original leaves in v0 (the new eye).
fn blend_eye(c: &mut Cpu, algo: ChaseCameraAlgorithm, sp: Addr, u: FpuF32) -> Vec4 {
    c.cycles += 14;
    let (three, one) = (FpuF32(0x4040_0000), FpuF32::ONE);
    let uu = u * u;
    let uu2 = uu + uu;
    let uu3 = uu * three;
    let uuu2 = uu2 * u;
    let mut e = uu3 - uuu2;
    if abi::c_lt_s(c, one, e) {
        e = one;
    }

    c.cycles += 27;
    let look = algo.look_at().read_vec4(c);
    let saved = handplant_offset(algo).read_vec4(c);
    let from = vsub(look, saved);
    sp.off(0x30).write_vec4(c, from);
    let rest = one - e;
    sp.off(0x20).write_vec4(c, from);
    let a = vmul(from, splat(VuF32(rest.0)));
    sp.off(0x30).write_vec4(c, a);
    sp.off(0x10).write_vec4(c, a);
    let eye = algo.desired_eye().read_vec4(c);
    let b = vmul(eye, splat(VuF32(e.0)));
    sp.off(0x40).write_vec4(c, b);
    sp.off(0x30).write_vec4(c, b);
    let blended = vadd(a, b);
    sp.off(0x40).write_vec4(c, blended);
    algo.desired_eye().write_vec4(c, blended);
    vu0_scratch::set_vf(c, 3, XYZW, a);
    vu0_scratch::set_vf(c, 4, XYZW, b);
    vu0_scratch::set_vf(c, 5, XYZW, blended);
    c.g[AT][0] = one.0 as u64;
    for (f, v) in [(0, rest), (1, uuu2), (2, e), (3, three), (4, one)] {
        c.f[f] = v.0;
    }
    c.g[V1][0] = abi::mfc1(e);
    blended
}

/// `ChaseCameraAlgorithm_ApplySideOffset(algo, side f12, lookAtFactor f13)`
/// (0x163158): shift the camera sideways: with `d = look_at − eye` and the
/// side axis `s = up × d` (VU0 outer product), when `|s| > 0` the eye moves
/// by `o = s × (side × |d| / |s|)` and the look-at point by
/// `o × lookAtFactor`. The outer product leaves ACC.y/z behind (they
/// survive the closing `CheckDistanceReset`); f12 is overwritten by `|s|`.
pub fn apply_side_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        let zero = FpuF32(0);
        c.f[0] = 0;
        frame.save_ra(c, 0x40);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let (side, look_factor) = (abi::farg(c, 0), abi::farg(c, 1));
        c.f[1] = side.0;
        let sp = Addr(frame.at(c, 0));
        let look = algo.look_at().read_vec4(c);
        let d = vsub(look, algo.desired_eye().read_vec4(c));
        sp.off(0x10).write_vec4(c, d);
        sp.write_vec4(c, d);
        let up = WORLD_UP.read_vec4(c);
        let mut acc = Acc::load(c);
        acc.opmula(up, d);
        let old6 = vf(c, 6);
        let cross = Vec4 { w: old6.w - old6.w, ..acc.opmsub(d, up) };
        acc.store(c);
        sp.off(0x20).write_vec4(c, cross);
        sp.off(0x10).write_vec4(c, cross);
        abi::set_gpr128(c, V0, cross.to_bits());
        vu0_scratch::set_vf(c, 6, XYZW, cross);
        vu0_scratch::set_vf(c, 3, XYZW, cross);
        vu0_scratch::set_vf(c, 5, XYZW, d);
        let side_len = FpuF32(length4(c, cross).0);
        c.g[V1][0] = cfc2_q(VuF32(side_len.0));
        c.f[12] = side_len.0;
        if abi::c_lt_s(c, zero, side_len) {
            c.cycles += 32;
            vu0_scratch::set_vf(c, 3, XYZW, d);
            let dist = length4(c, d);
            let k = (side * FpuF32(dist.0)).div(side_len);
            c.f[0] = k.0;
            let o = vmul(cross, splat(VuF32(k.0)));
            sp.off(0x10).write_vec4(c, o);
            let eye = algo.desired_eye().read_vec4(c);
            algo.desired_eye().write_vec4(c, vadd(eye, o));
            c.g[V1][0] = abi::mfc1(look_factor);
            let shift = vmul(o, splat(VuF32(look_factor.0)));
            sp.off(0x30).write_vec4(c, shift);
            sp.off(0x20).write_vec4(c, shift);
            abi::set_gpr128(c, V0, shift.to_bits());
            let look = algo.look_at().read_vec4(c);
            let moved = vadd(look, shift);
            algo.look_at().write_vec4(c, moved);
            vu0_scratch::set_vf(c, 3, XYZW, look);
            vu0_scratch::set_vf(c, 4, XYZW, shift);
            vu0_scratch::set_vf(c, 5, XYZW, moved);
        }
        c.cycles += 5;
        check_distance(c, 0x0016_3264)?;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCameraAlgorithm_ApplyRailPullback(algo, side f12, back f13, up
/// f14, rate f15)` (0x1641c0): step 14. The rail weight +0x214 follows the
/// target's rail flag (slot 24) with factor `rate`; the eye is then pushed
/// out along the look-at → eye direction `n` by `o = n × (w × back)` (plus
/// `w × up` on z) and sideways along `s = up × o`, scaled to `w × side`
/// when non-zero. The arguments live in f20..f23 across the calls. ACC.y/z
/// of the outer product survive the closing `CheckDistanceReset`.
pub fn apply_rail_pullback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x60);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let sp = Addr(frame.at(c, 0));
        c.g[S0][0] = c.g[A0][0];
        for (f, off) in [(23, 0x88), (22, 0x80), (21, 0x78), (20, 0x70)] {
            sp.off(off).write_u32(c, c.f[f]);
        }
        let (side, back, up_push, rate) = (abi::farg(c, 0), abi::farg(c, 1), abi::farg(c, 2), abi::farg(c, 3));
        c.f[23] = side.0;
        c.f[22] = up_push.0;
        c.f[21] = back.0;
        frame.save_ra(c, 0x50);
        c.f[20] = rate.0;

        // rail = target->IsRailSliding() (slot 24).
        let target = RiderCameraTarget(Addr(lw(c, algo.target()) as u32));
        c.g[A1][0] = sx(target.0 .0);
        let vt = lw(c, target.vptr());
        c.g[V1][0] = vt;
        let is_rail = VSlot::read(c, vt, 0xc0);
        c.g[V0][0] = is_rail.func;
        c.g[A0][0] = is_rail.this(c.g[A1][0]);
        call_ptr(c, is_rail.func as u32, 0x0016_4208)?;
        let rail = c.g[V0][0] != 0;

        let eye = algo.desired_eye().read_vec4(c);
        let look = algo.look_at().read_vec4(c);
        let d = vsub(eye, look);
        sp.off(0x10).write_vec4(c, d);
        abi::set_gpr128(c, V1, d.to_bits());
        sp.write_vec4(c, d);
        let target_weight = if rail {
            c.cycles += 2;
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            FpuF32::ONE
        } else {
            FpuF32(0)
        };

        c.cycles += 55;
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        let old = rail_weight(algo).read_fpu(c);
        let w = rate * old + (one - rate) * target_weight;
        rail_weight(algo).write_fpu(c, w);
        let push = w * back;
        // n = d / |d| (RSQRT), o = n × push.
        vu0_scratch::set_vf(c, 3, XYZW, d);
        let len_sq = length4_sq(c, d);
        let inv = q_rsqrt(c, len_sq);
        let n = vmul(d, splat(inv));
        sp.off(0x30).write_vec4(c, n);
        let lift = w * up_push;
        sp.off(0x20).write_vec4(c, n);
        c.g[V1][0] = abi::mfc1(push);
        let o = vmul(n, splat(VuF32(push.0)));
        sp.off(0x30).write_vec4(c, o);
        sp.off(0x10).write_vec4(c, o);
        let oz = sp.off(0x18).read_fpu(c) + lift;
        sp.off(0x18).write_fpu(c, oz);
        // s = up × o (outer product), |s|² without the square root.
        let o = sp.off(0x10).read_vec4(c);
        let up = WORLD_UP.read_vec4(c);
        let mut acc = Acc::load(c);
        acc.opmula(up, o);
        let old6 = vf(c, 6);
        let cross = Vec4 { w: old6.w - old6.w, ..acc.opmsub(o, up) };
        acc.store(c);
        vu0_scratch::set_vf(c, 6, XYZW, cross);
        sp.off(0x30).write_vec4(c, cross);
        sp.off(0x20).write_vec4(c, cross);
        vu0_scratch::set_vf(c, 3, XYZW, cross);
        vu0_scratch::set_vf(c, 5, XYZW, cross);
        let cross_sq = FpuF32(length4_sq(c, cross).0);
        abi::set_gpr128(c, V1, vf(c, 4).to_bits());
        let zero = FpuF32(0);
        let mut f0 = cross_sq;
        let mut f1 = lift;
        if abi::c_lt_s(c, zero, cross_sq) {
            c.cycles += 17;
            let len = length4(c, cross);
            let k = (w * side).div(FpuF32(len.0));
            f0 = k;
            f1 = FpuF32(len.0);
            let scaled = vmul(cross, splat(VuF32(k.0)));
            sp.off(0x20).write_vec4(c, scaled);
        }

        c.cycles += 21;
        let o = sp.off(0x10).read_vec4(c);
        let s = sp.off(0x20).read_vec4(c);
        let offset = vadd(o, s);
        sp.off(0x40).write_vec4(c, offset);
        c.g[A0][0] = c.g[S0][0];
        abi::set_gpr128(c, V0, offset.to_bits());
        sp.off(0x30).write_vec4(c, offset);
        let eye = algo.desired_eye().read_vec4(c);
        let moved = vadd(eye, offset);
        algo.desired_eye().write_vec4(c, moved);
        vu0_scratch::set_vf(c, 3, XYZW, eye);
        vu0_scratch::set_vf(c, 4, XYZW, offset);
        vu0_scratch::set_vf(c, 5, XYZW, moved);
        for (f, v) in [(0, f0), (1, f1), (2, zero), (3, target_weight), (15, w)] {
            c.f[f] = v.0;
        }
        check_distance(c, 0x0016_4384)?;
        frame.restore(c, S0, 0x60);
        frame.restore_ra(c, 0x50);
        for (f, off) in [(23, 0x88), (22, 0x80), (21, 0x78), (20, 0x70)] {
            c.f[f] = Addr(frame.at(c, off)).read_u32(c);
        }
        frame.close(c);
        Ok(())
    })
}

/// +0x214: rail weight (target slot 24 low-passed).
fn rail_weight(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x214)
}

/// The static zero vector (0x4ff120).
const ZERO_VEC: Addr = Addr(0x004f_f120);

/// +0xc0: smoothed rail alignment offset added to the eye.
fn rail_align_offset(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0xc0)
}

/// `ChaseCameraAlgorithm_ApplyRailAlign(algo, st, up f12, smoothing f13)`
/// (0x1646a0): step 13. When moving, the aligned eye would sit at
/// `p = look_at − (dir × |d| / |dir| − up × f12)` (`d = look_at − eye`,
/// `dir = st+0x20`; two VU0 lengths); while rail sliding the correction
/// `p − eye` (else zero) is smoothed into +0xc0, which is added to the eye.
pub fn apply_rail_align(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x90);
        let zero = FpuF32(0);
        c.f[1] = 0;
        frame.save_ra(c, 0x80);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        let sp = Addr(frame.at(c, 0));
        let speed = st.speed().read_fpu(c);
        c.f[0] = speed.0;
        let moving = abi::c_lt_s(c, zero, speed);
        c.g[A2][0] = c.g[A0][0];
        let p = if moving {
            c.cycles += 55;
            let look = algo.look_at().read_vec4(c);
            let d = vsub(look, algo.desired_eye().read_vec4(c));
            sp.off(0x20).write_vec4(c, d);
            let dir = st.0.off(0x20).read_vec4(c);
            abi::set_gpr128(c, V1, dir.to_bits());
            sp.off(0x10).write_vec4(c, d);
            vu0_scratch::set_vf(c, 3, XYZW, d);
            let dist = length4(c, d);
            c.g[A0][0] = cfc2_q(dist);
            sp.off(0x10).write_vec4(c, dir);
            vu0_scratch::set_vf(c, 3, XYZW, dir);
            let dir_len = length4(c, dir);
            c.f[1] = dir_len.0;
            let k = FpuF32(dist.0).div(FpuF32(dir_len.0));
            c.g[V0][0] = abi::mfc1(k);
            let along = vmul(dir, splat(VuF32(k.0)));
            sp.off(0x10).write_vec4(c, along);
            let up_arg = abi::farg(c, 0);
            c.g[V1][0] = abi::mfc1(up_arg);
            let lift = vmul(WORLD_UP.read_vec4(c), splat(VuF32(up_arg.0)));
            sp.off(0x30).write_vec4(c, lift);
            sp.off(0x20).write_vec4(c, lift);
            let back = vsub(along, lift);
            sp.off(0x10).write_vec4(c, back);
            let p = vsub(look, back);
            sp.off(0x30).write_vec4(c, p);
            sp.write_vec4(c, p);
            sp.off(0x20).write_vec4(c, p);
            c.f[0] = k.0;
            p
        } else {
            c.cycles += 2;
            let eye = algo.desired_eye().read_vec4(c);
            sp.write_vec4(c, eye);
            eye
        };

        // The correction toward p while rail sliding, else zero.
        c.cycles += 9;
        let correction = vsub(p, algo.desired_eye().read_vec4(c));
        sp.off(0x20).write_vec4(c, correction);
        let railslide = lw(c, st.is_railslide());
        c.g[V1][0] = railslide;
        sp.off(0x10).write_vec4(c, correction);
        let mut source = sp.off(0x10);
        if railslide == 0 {
            c.cycles += 2;
            source = ZERO_VEC;
        }

        c.cycles += 37;
        let target = source.read_vec4(c);
        c.g[A0][0] = c.g[A2][0];
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        sp.off(0x20).write_vec4(c, target);
        let smoothing = abi::farg(c, 1);
        let kept = vmul(rail_align_offset(algo).read_vec4(c), splat(VuF32(smoothing.0)));
        sp.off(0x60).write_vec4(c, kept);
        let rest = one - smoothing;
        sp.off(0x50).write_vec4(c, kept);
        c.g[V1][0] = abi::mfc1(rest);
        let added = vmul(target, splat(VuF32(rest.0)));
        sp.off(0x70).write_vec4(c, added);
        sp.off(0x60).write_vec4(c, added);
        let offset = vadd(kept, added);
        sp.off(0x70).write_vec4(c, offset);
        abi::set_gpr128(c, V0, offset.to_bits());
        rail_align_offset(algo).write_vec4(c, offset);
        sp.off(0x40).write_vec4(c, offset);
        sp.off(0x30).write_vec4(c, offset);
        let eye = algo.desired_eye().read_vec4(c);
        let moved = vadd(eye, offset);
        algo.desired_eye().write_vec4(c, moved);
        vu0_scratch::set_vf(c, 3, XYZW, eye);
        vu0_scratch::set_vf(c, 4, XYZW, offset);
        vu0_scratch::set_vf(c, 5, XYZW, moved);
        c.f[0] = rest.0;
        check_distance(c, 0x0016_4858)?;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

const KMH_PER_CM_S_GP: u32 = 0x692c; // 0.036
const LOW_SPEED_IN_STEP_GP: u32 = 0x6928; // 1/60
const LOW_SPEED_OUT_STEP_GP: u32 = 0x6924; // 1/60

/// `lowSpeedState` (+0x300) values.
const LOW_SPEED_BLEND_IN: u64 = 0;
const LOW_SPEED_BLEND_OUT: u64 = 1;
const LOW_SPEED_HELD: u64 = 2;
const LOW_SPEED_OFF: u64 = 3;

fn low_speed_state(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x300)
}
fn low_speed_blend(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x248)
}
/// +0x1a0: look-at → eye offset saved when the low-speed hold starts.
fn low_speed_offset(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x1a0)
}

/// `ChaseCameraAlgorithm_ApplyLowSpeedHold(algo, st, kmh f12, inTime f13,
/// outTime f14)` (0x1643a8): step 15. Below `kmh` (speed × 0.036), not
/// airborne and not in a handplant, the camera blends (state +0x300,
/// weight +0x248 over `inTime`/`outTime` seconds) toward a hold: the
/// look-at point moves to the rider and the look-at → eye offset turns
/// toward the one saved at the start (+0x1a0), keeping the average of the
/// two lengths. State 3 = off.
pub fn apply_low_speed_hold(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        let to_kmh = gp_const(c, KMH_PER_CM_S_GP);
        frame.save_ra(c, 0x60);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        let sp = Addr(frame.at(c, 0));
        let kmh = st.speed().read_fpu(c) * to_kmh;
        c.f[0] = kmh.0;
        c.f[1] = to_kmh.0;
        let slow = abi::c_lt_s(c, kmh, abi::farg(c, 0));
        c.g[A2][0] = 0;
        // want = slow && !air && !handplant.
        let want = if !slow {
            c.cycles += 1;
            false
        } else {
            c.cycles += 3;
            let air = lw(c, st.is_air());
            c.g[V0][0] = air;
            if air != 0 {
                false
            } else {
                c.cycles += 3;
                let handplant = lw(c, st.is_handplant());
                c.g[V0][0] = handplant;
                handplant == 0
            }
        };
        c.g[A2][0] = want as u64;

        // State transitions.
        let state = lw(c, low_speed_state(algo));
        c.cycles += 3;
        match state {
            LOW_SPEED_OFF => {
                c.cycles += 2;
                if want {
                    c.cycles += 11;
                    low_speed_state(algo).sw(c, LOW_SPEED_BLEND_IN);
                    low_speed_blend(algo).sw(c, 0);
                    let look = algo.look_at().read_vec4(c);
                    let eye = algo.desired_eye().read_vec4(c);
                    let offset = vsub(look, eye);
                    sp.off(0x10).write_vec4(c, offset);
                    low_speed_offset(algo).write_vec4(c, offset);
                    sp.write_vec4(c, offset);
                    vu0_scratch::set_vf(c, 3, XYZW, look);
                    vu0_scratch::set_vf(c, 4, XYZW, eye);
                    vu0_scratch::set_vf(c, 5, XYZW, offset);
                } else {
                    c.cycles += 2 + 2 + 3;
                }
            }
            LOW_SPEED_HELD => {
                c.cycles += 2 + 2;
                if !want {
                    c.cycles += 5 + 1;
                    let one = FpuF32::ONE;
                    c.g[AT][0] = one.0 as u64;
                    c.f[0] = one.0;
                    low_speed_state(algo).sw(c, LOW_SPEED_BLEND_OUT);
                    low_speed_blend(algo).write_fpu(c, one);
                } else {
                    c.cycles += 2 + 3;
                }
            }
            LOW_SPEED_BLEND_IN => {
                c.cycles += 2 + 2 + 2 + 3;
                if !want {
                    c.cycles += 1;
                    low_speed_state(algo).sw(c, LOW_SPEED_BLEND_OUT);
                }
            }
            LOW_SPEED_BLEND_OUT => {
                c.cycles += 2 + 2 + 3 + 2 + 1;
                if want {
                    low_speed_state(algo).sw(c, LOW_SPEED_BLEND_IN);
                }
            }
            _ => c.cycles += 2 + 2 + 3,
        }

        // Advance the blend weight.
        let state = lw(c, low_speed_state(algo));
        c.cycles += 2;
        let (zero, one) = (FpuF32(0), FpuF32::ONE);
        if state == LOW_SPEED_BLEND_IN {
            c.cycles += 11;
            let step = gp_const(c, LOW_SPEED_IN_STEP_GP).div(abi::farg(c, 1));
            c.g[AT][0] = one.0 as u64;
            let w = (low_speed_blend(algo).read_fpu(c) + step).min(one);
            let full = w.eq_s(one);
            abi::set_fcc(c, full);
            low_speed_blend(algo).write_fpu(c, w);
            for (f, v) in [(0, step), (1, w), (2, one)] {
                c.f[f] = v.0;
            }
            if full {
                c.cycles += 3;
                low_speed_state(algo).sw(c, LOW_SPEED_HELD);
            }
            c.cycles += 1;
        } else {
            c.cycles += 2;
            if state == LOW_SPEED_BLEND_OUT {
                c.cycles += 10;
                let step = gp_const(c, LOW_SPEED_OUT_STEP_GP).div(abi::farg(c, 2));
                let w = (low_speed_blend(algo).read_fpu(c) - step).max(zero);
                let empty = w.eq_s(zero);
                abi::set_fcc(c, empty);
                low_speed_blend(algo).write_fpu(c, w);
                for (f, v) in [(0, step), (1, w), (2, zero)] {
                    c.f[f] = v.0;
                }
                if empty {
                    c.cycles += 2;
                    low_speed_state(algo).sw(c, LOW_SPEED_OFF);
                }
                c.cycles += 1;
            }
        }
        let state = lw(c, low_speed_state(algo));
        c.g[V1][0] = state;
        c.cycles += 3;
        c.g[V0][0] = LOW_SPEED_OFF;
        if state != LOW_SPEED_OFF {
            hold_blend(c, algo, st, sp);
        }
        c.cycles += 5;
        check_distance(c, 0x0016_4694)?;
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The blend of [`apply_low_speed_hold`] (blocks 0x164500..0x164688):
/// `e = min(3w² − 2w³, 1)`, offset `b = d × (1 − e) + saved × e` rescaled
/// to `(|d| + |b|) / 2`, look-at `= look_at × (1 − e) + rider × e`,
/// eye `= look-at − b`.
fn hold_blend(c: &mut Cpu, algo: ChaseCameraAlgorithm, st: RiderCamState, sp: Addr) {
    c.cycles += 14;
    let (three, one, half) = (FpuF32(0x4040_0000), FpuF32::ONE, FpuF32(0x3f00_0000));
    let w = low_speed_blend(algo).read_fpu(c);
    let ww = w * w;
    let ww2 = ww + ww;
    let ww3 = ww * three;
    let www2 = ww2 * w;
    let mut e = ww3 - www2;
    if abi::c_lt_s(c, one, e) {
        e = one;
    }

    c.cycles += 86;
    let look = algo.look_at().read_vec4(c);
    let d = vsub(look, algo.desired_eye().read_vec4(c));
    sp.off(0x10).write_vec4(c, d);
    let rest = one - e;
    c.g[AT][0] = half.0 as u64;
    sp.write_vec4(c, d);
    let from_d = vmul(d, splat(VuF32(rest.0)));
    sp.off(0x30).write_vec4(c, from_d);
    sp.off(0x20).write_vec4(c, from_d);
    let from_saved = vmul(low_speed_offset(algo).read_vec4(c), splat(VuF32(e.0)));
    sp.off(0x40).write_vec4(c, from_saved);
    sp.off(0x30).write_vec4(c, from_saved);
    let b = vadd(from_d, from_saved);
    sp.off(0x40).write_vec4(c, b);
    sp.off(0x10).write_vec4(c, b);
    vu0_scratch::set_vf(c, 3, XYZW, d);
    let d_len = FpuF32(length4(c, d).0);
    vu0_scratch::set_vf(c, 3, XYZW, b);
    let b_len = length4(c, b);
    c.g[V1][0] = cfc2_q(b_len);
    let b_len = FpuF32(b_len.0);
    let half_d = d_len * half;
    let half_b = b_len * half;
    let k = (half_d + half_b).div(b_len);
    let b = vmul(b, splat(VuF32(k.0)));
    sp.off(0x10).write_vec4(c, b);
    let kept = vmul(look, splat(VuF32(rest.0)));
    sp.off(0x40).write_vec4(c, kept);
    sp.off(0x30).write_vec4(c, kept);
    c.g[V1][0] = abi::mfc1(e);
    let to_rider = vmul(st.rider_position().read_vec4(c), splat(VuF32(e.0)));
    sp.off(0x50).write_vec4(c, to_rider);
    sp.off(0x40).write_vec4(c, to_rider);
    let new_look = vadd(kept, to_rider);
    sp.off(0x50).write_vec4(c, new_look);
    algo.look_at().write_vec4(c, new_look);
    sp.off(0x20).write_vec4(c, new_look);
    let eye = vsub(new_look, b);
    sp.off(0x30).write_vec4(c, eye);
    algo.desired_eye().write_vec4(c, eye);
    sp.off(0x20).write_vec4(c, eye);
    abi::set_gpr128(c, V0, eye.to_bits());
    vu0_scratch::set_vf(c, 3, XYZW, new_look);
    vu0_scratch::set_vf(c, 4, XYZW, b);
    vu0_scratch::set_vf(c, 5, XYZW, eye);
    for (f, v) in [(0, k), (1, half_b), (2, b_len), (3, rest), (4, e), (5, one)] {
        c.f[f] = v.0;
    }
}

const DROP_STEEPNESS_GP: u32 = 0x6af0; // 0.85
const DROP_GROUND_NORMAL_GP: u32 = 0x6aec; // 0.1
/// Frames on the ground before a takeoff may start the takeoff kick.
const TAKEOFF_MIN_GROUND_FRAMES: i64 = 31;
/// `takeoffCountdown` start value.
const TAKEOFF_COUNTDOWN: u64 = 15;

/// Air bookkeeping fields of `ChaseCameraAlgorithm`.
fn air_swing_time(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x22c)
}
fn takeoff_countdown(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2c4)
}
fn last_air_frame(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2cc)
}
fn is_drop(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2d0)
}
fn was_airborne(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2d4)
}
fn air_swing_set_up(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2d8)
}
fn air_curve_active(algo: ChaseCameraAlgorithm) -> Addr {
    algo.0.off(0x2e0)
}

/// `ChaseCameraAlgorithm_UpdateAirTransitions(algo, st)` (0x162a20): step
/// 3, takeoff and landing bookkeeping. At takeoff (Air and not yet
/// `wasAirborne`): a drop is flagged (+0x2d0, swing time +0x22c cleared)
/// when the normalised velocity's z exceeds 0.85 and the previous ground
/// normal (target slot 9) has z < 0.1; the takeoff countdown (+0x2c4) is
/// set to 15 when no air-arc curve runs and the rider was grounded for
/// more than 30 frames. On landing the air flags are cleared. While
/// airborne the AI frame counter is stamped into +0x2cc.
pub fn update_air_transitions(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        let st = RiderCamState(Addr(abi::arg(c, 1)));
        c.g[S1][0] = c.g[A1][0];
        let sp = Addr(frame.at(c, 0));
        sp.off(0x50).write_u32(c, c.f[20]);
        let air = lw(c, st.is_air());
        c.g[V0][0] = air;
        c.g[S0][0] = c.g[A0][0];

        let stamp = if air == 0 {
            // On the ground: clear the air flags after a landing.
            c.cycles += 3;
            let was = lw(c, was_airborne(algo));
            c.g[V0][0] = was;
            if was != 0 {
                c.cycles += 4;
                was_airborne(algo).sw(c, 0);
                is_drop(algo).sw(c, 0);
                air_swing_set_up(algo).sw(c, 0);
            }
            let air = lw(c, st.is_air());
            c.g[V0][0] = air;
            c.cycles += 2;
            air != 0
        } else {
            c.cycles += 3;
            let was = lw(c, was_airborne(algo));
            c.g[V0][0] = was;
            if was == 0 {
                takeoff(c, algo, st, sp)?;
                c.cycles += 1;
                let air = lw(c, st.is_air());
                c.g[V0][0] = air;
                c.cycles += 2;
                air != 0
            } else {
                true
            }
        };
        if stamp {
            c.cycles += 4;
            call_ptr(c, table_callee::AI_GET_FRAME_COUNTER, 0x0016_2b60)?;
            last_air_frame(algo).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        c.f[20] = sp.off(0x50).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

/// The takeoff part of [`update_air_transitions`] (blocks 0x162a6c..0x162b48).
fn takeoff(c: &mut Cpu, algo: ChaseCameraAlgorithm, st: RiderCamState, sp: Addr) -> Flow {
    c.cycles += 7;
    let zero = FpuF32(0);
    let speed = st.speed().read_fpu(c);
    c.f[0] = speed.0;
    c.f[20] = 0;
    let vel = st.0.off(0x20).read_vec4(c);
    abi::set_gpr128(c, V0, vel.to_bits());
    let moving = abi::c_lt_s(c, zero, speed);
    sp.write_vec4(c, vel);
    let mut checked_normal = false;
    if moving {
        c.cycles += 23;
        vu0_scratch::set_vf(c, 3, XYZW, vel);
        let len = length4(c, vel);
        c.g[V0][0] = cfc2_q(len);
        c.f[1] = len.0;
        let steep = gp_const(c, DROP_STEEPNESS_GP);
        c.f[2] = steep.0;
        let vf3 = qmtc2(c.g[V0]);
        vu0_scratch::set_vf(c, 3, XYZW, vf3);
        let inv = q_div(c, VuF32::ONE, vf3.x);
        let dir = vmul(vel, splat(inv));
        sp.write_vec4(c, dir);
        vu0_scratch::set_vf(c, 4, XYZW, vel);
        vu0_scratch::set_vf(c, 5, XYZW, dir);
        let dir_z = sp.off(8).read_fpu(c);
        c.f[0] = dir_z.0;
        let falling = abi::c_lt_s(c, steep, dir_z);
        c.g[A0][0] = sx(sp.off(0x10).0);
        if falling {
            // Previous ground normal (target slot 9) into sp+0x10.
            c.cycles += 12;
            let target = RiderCameraTarget(Addr(lw(c, algo.target()) as u32));
            c.g[A2][0] = sx(target.0 .0);
            let vt = lw(c, target.vptr());
            c.g[V0][0] = vt;
            let prev_normal = VSlot::read(c, vt, 0x48);
            c.g[V1][0] = prev_normal.func;
            c.g[A1][0] = prev_normal.this(c.g[A2][0]);
            call_ptr(c, prev_normal.func as u32, 0x0016_2ae0)?;
            let normal_z = sp.off(0x18).read_fpu(c);
            let limit = gp_const(c, DROP_GROUND_NORMAL_GP);
            c.f[1] = normal_z.0;
            c.f[0] = limit.0;
            let steep_ground = abi::c_lt_s(c, normal_z, limit);
            c.g[V0][0] = 1;
            checked_normal = true;
            if steep_ground {
                c.cycles += 2;
                air_swing_time(algo).write_u32(c, c.f[20]);
                is_drop(algo).sw(c, 1);
                c.cycles += 1;
            }
        }
    }
    if !checked_normal {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }

    // Mark airborne; start the takeoff countdown after a long ground run.
    c.cycles += 8;
    was_airborne(algo).sw(c, 1);
    call_ptr(c, table_callee::AI_GET_FRAME_COUNTER, 0x0016_2b0c)?;
    let last = lw(c, last_air_frame(algo));
    c.g[V1][0] = last;
    let curve = lw(c, air_curve_active(algo));
    c.g[A0][0] = curve;
    let ground_frames = sx((c.g[V0][0] as u32).wrapping_sub(last as u32));
    let long_ground = !((ground_frames as i64) < TAKEOFF_MIN_GROUND_FRAMES);
    c.g[V0][0] = long_ground as u64;
    if curve == 0 {
        c.cycles += 2;
        c.g[V0][0] = TAKEOFF_COUNTDOWN;
        if long_ground {
            c.cycles += 2;
            takeoff_countdown(algo).sw(c, TAKEOFF_COUNTDOWN);
        }
    }
    Ok(())
}

/// The static reference axes (0x4ff140 / 0x4ff150), written at boot.
const REF_AXIS_0: Addr = Addr(0x004f_f140);
const REF_AXIS_1: Addr = Addr(0x004f_f150);
const ANGLE_EPSILON_GP: u32 = 0x68b4;
const ANGLE_EPSILON_2_GP: u32 = 0x68ac;
const ANGLE_BASE_GP: [u32; 3] = [0x68b0, 0x68a8, 0x68a4];

/// `ChaseCameraAlgorithm_ComputeAngles(algo)` (0x166640): the view angles
/// of the eye → look-at direction `d`: pitch +0x54 = −atan(d.z / |d_xy|)
/// and heading +0x50 from the angle between the flattened direction and
/// the reference axes 0x4ff140/0x4ff150 (`asin` of the normalised dot
/// products, mapped to the full circle by the side of axis 1), 0 when the
/// direction is vertical. The dot products use the VU0 idiom ([`dot4`]),
/// read back with `qmfc2`.
pub fn compute_angles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let heading = algo.0.off(0x50);
        c.g[S1][0] = sx(heading.0);
        let sp = Addr(frame.at(c, 0));
        sp.off(0x50).write_u32(c, c.f[20]);
        let d = vsub(algo.look_at().read_vec4(c), algo.desired_eye().read_vec4(c));
        sp.off(0x10).write_vec4(c, d);
        abi::set_gpr128(c, V0, d.to_bits());
        let dz = sp.off(0x18).read_fpu(c);
        sp.write_vec4(c, d);
        sp.off(0x18).sw(c, 0);
        let flat = sp.off(0x10).read_vec4(c);
        vu0_scratch::set_vf(c, 3, XYZW, flat);
        vu0_scratch::set_vf(c, 5, XYZW, d);
        let flat_len = length4(c, flat);
        c.g[V0][0] = cfc2_q(flat_len);
        let flat_len = FpuF32(flat_len.0);
        c.f[20] = flat_len.0;
        c.f[12] = dz.div(flat_len).0;
        call_ptr(c, table_callee::MATH_ATAN, 0x0016_66b0)?;

        let zero = FpuF32(0);
        c.f[3] = 0;
        let pitch = -FpuF32(c.f[0]);
        c.f[0] = pitch.0;
        let level = abi::c_lt_s(c, zero, flat_len);
        algo.0.off(0x54).write_fpu(c, pitch);
        if !level {
            c.cycles += 1;
            heading.write_fpu(c, zero);
        } else {
            c.cycles += 15;
            c.g[V1][0] = sx(REF_AXIS_1.0);
            let axis1 = REF_AXIS_1.read_vec4(c);
            let side = dot4_ab(c, flat, axis1, V0);
            c.f[0] = side.0;
            let ahead = abi::c_lt_s(c, zero, FpuF32(side.0));
            c.g[A0][0] = 0x0050_0000;
            let angle = if ahead {
                c.cycles += 25;
                let lens = lens_product(c, flat, ANGLE_EPSILON_GP);
                c.f[0] = 0;
                if abi::c_lt_s(c, lens, FpuF32(c.f[2])) {
                    zero
                } else {
                    c.cycles += 16;
                    asin_of_ratio(c, flat, lens, 5, 0x0016_67bc)?;
                    let base = gp_const(c, ANGLE_BASE_GP[0]);
                    c.f[1] = base.0;
                    base - FpuF32(c.f[0])
                }
            } else {
                c.cycles += 24;
                let lens = lens_product(c, flat, ANGLE_EPSILON_2_GP);
                let a = if abi::c_lt_s(c, lens, FpuF32(c.f[2])) {
                    c.cycles += 2;
                    zero
                } else {
                    c.cycles += 16;
                    asin_of_ratio(c, flat, lens, 4, 0x0016_6884)?;
                    let base = gp_const(c, ANGLE_BASE_GP[1]);
                    base - FpuF32(c.f[0])
                };
                c.f[1] = a.0;
                c.cycles += 2;
                gp_const(c, ANGLE_BASE_GP[2]) - a
            };
            c.f[0] = angle.0;
            c.cycles += 2;
            heading.write_fpu(c, angle);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        c.f[20] = sp.off(0x50).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

/// [`dot4`] of `a` (vf3) and `b` (vf5), read back with `qmfc2 reg, vf4`.
fn dot4_ab(c: &mut Cpu, a: Vec4, b: Vec4, reg: usize) -> VuF32 {
    vu0_scratch::set_vf(c, 3, XYZW, a);
    vu0_scratch::set_vf(c, 5, XYZW, b);
    let vf4 = dot4(c, a, b);
    abi::set_gpr128(c, reg, vf4.to_bits());
    vf4.x
}

/// `sqrt(|flat|² × |axis0|²)` (FPU), with the epsilon in f2: f0 = the
/// product, f1 = the root.
fn lens_product(c: &mut Cpu, flat: Vec4, eps_gp: u32) -> FpuF32 {
    let flat_sq = FpuF32(dot4_ab(c, flat, flat, V0).0);
    let axis = REF_AXIS_0.read_vec4(c);
    let axis_sq = FpuF32(dot4_ab(c, axis, axis, V1).0);
    let eps = gp_const(c, eps_gp);
    c.f[2] = eps.0;
    let prod = flat_sq * axis_sq;
    let root = prod.sqrt();
    c.f[0] = prod.0;
    c.f[1] = root.0;
    root
}

/// `Math_Asin(clamp(flat·axis0 / lens, −1, 1))` returning to `ret`, with
/// `cycles` charged before the call (the result is in f0).
fn asin_of_ratio(c: &mut Cpu, flat: Vec4, lens: FpuF32, cycles: u64, ret: u32) -> Flow {
    let axis = REF_AXIS_0.read_vec4(c);
    let dot = FpuF32(dot4_ab(c, flat, axis, V0).0);
    c.f[0] = dot.0;
    let minus_one = FpuF32(0xbf80_0000);
    c.g[AT][0] = sx(minus_one.0);
    c.f[2] = minus_one.0;
    let ratio = dot.div(lens);
    let above = minus_one.le(ratio);
    abi::set_fcc(c, above);
    let ratio = if !above {
        minus_one
    } else {
        c.cycles += 3;
        let one = FpuF32::ONE;
        c.g[AT][0] = one.0 as u64;
        c.f[0] = one.0;
        ratio.min(one)
    };
    c.f[12] = ratio.0;
    c.cycles += cycles;
    call_ptr(c, table_callee::MATH_ASIN, ret)
}

/// `ChaseCameraAlgorithm_PostUpdate(algo)` (0x166228, vtable slot 8): get
/// the leaf class's tuning record (slot 7: four floats and a flag, in the
/// frame), compute the view angles, apply the tuning when its flag is set
/// (`FUN_001662a0(algo, f12..f15)`), then `FUN_00166530(algo)` and
/// `ChaseCameraAlgorithm_BuildOutputView(algo)`.
pub fn post_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        let sp = Addr(frame.at(c, 0));
        c.g[A1][0] = sx(sp.0);
        frame.save_ra(c, 0x20);
        let algo = c.g[A0][0];
        c.g[S0][0] = algo;
        let vt = lw(c, Addr(algo as u32).off(0x10));
        c.g[V0][0] = vt;
        let get_tuning = VSlot::read(c, vt, 0x38);
        c.g[V1][0] = get_tuning.func;
        c.g[A0][0] = get_tuning.this(algo);
        call_ptr(c, get_tuning.func as u32, 0x0016_6250)?;

        c.g[A0][0] = algo;
        c.g[A1][0] = 0;
        callee::COMPUTE_ANGLES.call(c, 0x0016_625c)?;
        let apply = lw(c, sp.off(0x10));
        c.g[V0][0] = apply;
        c.f[12] = sp.read_u32(c);
        if apply != 0 {
            c.cycles += 5;
            c.g[A0][0] = algo;
            c.f[13] = sp.off(4).read_u32(c);
            c.f[14] = sp.off(8).read_u32(c);
            c.f[15] = sp.off(0xc).read_u32(c);
            callee::APPLY_TUNING.call(c, 0x0016_627c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = algo;
        callee::FUN_00166530.call(c, 0x0016_6284)?;
        c.g[A0][0] = algo;
        callee::BUILD_OUTPUT_VIEW.call(c, 0x0016_628c)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The race default algorithm id (DEFAULT_3).
const ALGO_DEFAULT_3: u64 = 0x3d;
/// An algorithm id that keeps its own auto mode.
const ALGO_KEEP_AUTO: u64 = 0x4a;
/// Rider main state Handplant.
const RIDER_STATE_HANDPLANT: u64 = 5;
/// Spoke mode ends above this speed (km/h).
const SPOKE_EXIT_KMH: FpuF32 = FpuF32(0x420c_0000); // 35.0

/// `ChaseCamController` fields this package adds.
fn auto_algorithm(ctl: ChaseCamController) -> Addr {
    ctl.0.off(0x1c)
}
fn current_algorithm(ctl: ChaseCamController) -> Addr {
    ctl.0.off(0x18)
}
fn controller_target(ctl: ChaseCamController) -> Addr {
    ctl.0.off(0x20)
}

/// `ChaseCamController_UpdateAutoMode(ctl)` (0x161ab0): force the auto
/// algorithm back to DEFAULT_3 (switching to it unless it or 0x4a is
/// current), then manage spoke mode: enter it when the rider's main state
/// is Handplant, leave it above 35 km/h (`|velocity| × 0.036`, four-lane
/// VU0 length).
pub fn update_auto_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let ctl = ChaseCamController(Addr(abi::arg(c, 0)));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = ALGO_DEFAULT_3;
        let sp = Addr(frame.at(c, 0));
        sp.off(0x40).write_u32(c, c.f[20]);
        let auto = lw(c, auto_algorithm(ctl));
        c.g[V0][0] = auto;
        if auto != ALGO_DEFAULT_3 {
            c.cycles += 4;
            let current = lw(c, current_algorithm(ctl));
            c.g[V1][0] = current;
            c.g[V0][0] = ALGO_KEEP_AUTO;
            if current != ALGO_KEEP_AUTO {
                c.cycles += 3;
                c.g[A1][0] = ALGO_DEFAULT_3;
                callee::SET_DEFAULT_ALGORITHM.call(c, 0x0016_1af0)?;
            }
        }
        auto_algorithm(ctl).sw(c, ALGO_DEFAULT_3);

        // Rider speed (km/h) from the target's velocity (slot 3).
        c.cycles += 26;
        c.g[A0][0] = sx(sp.0);
        let target = RiderCameraTarget(Addr(lw(c, controller_target(ctl)) as u32));
        c.g[A2][0] = sx(target.0 .0);
        let vt = lw(c, target.vptr());
        c.g[V0][0] = vt;
        let get_velocity = VSlot::read(c, vt, 0x18);
        c.g[V1][0] = get_velocity.func;
        c.g[A1][0] = get_velocity.this(c.g[A2][0]);
        call_ptr(c, get_velocity.func as u32, 0x0016_1b10)?;
        let target = RiderCameraTarget(Addr(lw(c, controller_target(ctl)) as u32));
        c.g[V0][0] = sx(target.0 .0);
        let vel = sp.read_vec4(c);
        vu0_scratch::set_vf(c, 3, XYZW, vel);
        let speed = length4(c, vel);
        c.g[V1][0] = cfc2_q(speed);
        c.f[1] = speed.0;
        let to_kmh = gp_const(c, KMH_PER_CM_S_GP_AUTO);
        c.f[0] = to_kmh.0;
        c.g[A0][0] = lw(c, target.rider());
        let kmh = FpuF32(speed.0) * to_kmh;
        c.f[20] = kmh.0;
        call_ptr(c, table_callee::RIDER_GET_MAIN_STATE, 0x0016_1b4c)?;

        let handplant = (c.g[V0][0] ^ RIDER_STATE_HANDPLANT) == 0;
        let spoke = lw(c, ctl.spoke_mode());
        c.g[V1][0] = spoke;
        c.g[V0][0] = handplant as u64;
        if spoke == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            if handplant {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                ctl.spoke_mode().sw(c, 1);
                callee::ENTER_SPOKE_MODE.call(c, 0x0016_1b70)?;
            } else {
                c.cycles += 1;
            }
        } else {
            c.cycles += 6;
            c.g[AT][0] = SPOKE_EXIT_KMH.0 as u64;
            c.f[0] = SPOKE_EXIT_KMH.0;
            if abi::c_lt_s(c, SPOKE_EXIT_KMH, kmh) {
                c.cycles += 3;
                ctl.spoke_mode().sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                callee::EXIT_SPOKE_MODE.call(c, 0x0016_1b9c)?;
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        c.f[20] = sp.off(0x40).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

const KMH_PER_CM_S_GP_AUTO: u32 = 0x6b28; // 0.036

/// gp offset of a word whose bit 0 disables the boost shake.
const SHAKE_DISABLE_GP: u32 = 0x6a0;
const BOOST_SHAKE_MIN_GP: u32 = 0x6c80;
const SHAKE_SPEED_SCALE_GP: u32 = 0x6c7c;
/// Boost levels for shake presets 3 and 2 (lower: preset 1).
const BOOST_SHAKE_3_GP: u32 = 0x6c78;
const BOOST_SHAKE_2_GP: u32 = 0x6c74;
/// Shake presets (16 bytes: amplitude, frequency, ...) and their flags.
const SHAKE_PRESETS: Addr = Addr(0x004c_53e0);
const SHAKE_PRESET_FLAGS: Addr = Addr(0x004c_5430);

/// `GameCamera` shake fields (camera.md).
fn shake_preset(cam: Addr) -> Addr {
    cam.off(0x450)
}
fn shake_strength(cam: Addr) -> Addr {
    cam.off(0x454)
}
fn shake_requested(cam: Addr) -> Addr {
    cam.off(0x458)
}
fn shake_requested_prev(cam: Addr) -> Addr {
    cam.off(0x45c)
}

/// `GameCamera_UpdateShake(cam)` (0x15e460): while the rider boosts above
/// a threshold (target slot 17) request a shake (`GameCamera_AddShake`,
/// preset 3/2/1 by boost level, strength by speed: 0 below 60, 1 above 120
/// in the target's speed units), then start the strongest requested shake
/// on the embedded algorithm (`ChaseCameraAlgorithm_StartShake`, only on
/// the frame a request appears) and apply it (`ApplyShake`); the request
/// fields +0x454/+0x458 are cleared and +0x45c remembers this frame's.
pub fn game_camera_update_shake(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x30);
        let sp = Addr(frame.at(c, 0));
        sp.off(0x50).write_u32(c, c.f[20]);
        let cam = Addr(abi::arg(c, 0));
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        sp.off(0x60).write_u32(c, c.f[22]);
        sp.off(0x58).write_u32(c, c.f[21]);
        let ctl = lw(c, cam.off(0xa8));
        c.g[V0][0] = ctl;
        let target = RiderCameraTarget(Addr(lw(c, Addr(ctl as u32).off(0x20)) as u32));
        c.g[S0][0] = sx(target.0 .0);
        let vt = lw(c, target.vptr());
        c.g[V1][0] = vt;
        let get_boost = VSlot::read(c, vt, 0x88);
        c.g[V0][0] = get_boost.func;
        c.g[A0][0] = get_boost.this(c.g[S0][0]);
        call_ptr(c, get_boost.func as u32, 0x0015_e4a0)?;

        let disabled = lw(c, Addr((c.g[GP][0] as u32).wrapping_sub(SHAKE_DISABLE_GP))) & 1;
        c.g[V0][0] = disabled ^ 1;
        let boost = FpuF32(c.f[0]);
        c.f[20] = boost.0;
        // Whether the preset still has to be loaded (block 0x15e5b4).
        let mut reload_preset = true;
        if disabled == 0 {
            c.cycles += 5;
            let min = gp_const(c, BOOST_SHAKE_MIN_GP);
            c.f[0] = min.0;
            let boosting = abi::c_lt_s(c, min, boost);
            c.g[A0][0] = sx(sp.0);
            if boosting {
                let strength = shake_speed_factor(c, target, sp)?;
                c.f[12] = strength.0;
                reload_preset = request_boost_shake(c, cam, boost)?;
            }
        }
        if reload_preset {
            c.cycles += 1;
        }
        let preset = lw(c, shake_preset(cam));
        c.g[V0][0] = preset;

        // Start (on a new request) and apply the shake.
        c.cycles += 16;
        c.g[A1][0] = sx(SHAKE_PRESETS.0);
        let preset_off = (preset as u32) << 4;
        let flag_at = SHAKE_PRESET_FLAGS.off((preset as u32) << 2);
        c.g[V1][0] = sx(flag_at.0);
        c.g[A0][0] = sx(SHAKE_PRESET_FLAGS.0);
        let entry = SHAKE_PRESETS.off(preset_off);
        c.g[A2][0] = sx(entry.0);
        let flag = lw(c, flag_at);
        c.g[S2][0] = flag;
        c.g[V0][0] = sx(entry.0);
        let frequency = entry.off(4).read_fpu(c);
        let amplitude = entry.read_fpu(c);
        c.f[22] = frequency.0;
        c.f[21] = amplitude.0;
        let algo = cam.off(0xc0);
        c.g[S0][0] = sx(algo.0);
        let requested = lw(c, shake_requested(cam));
        c.g[V0][0] = requested;
        let strength = shake_strength(cam).read_fpu(c);
        c.f[20] = strength.0;
        let mut reload_request = true;
        if requested != 0 {
            c.cycles += 3;
            let prev = lw(c, shake_requested_prev(cam));
            c.g[V0][0] = prev;
            if prev != 0 {
                reload_request = false;
            } else {
                c.cycles += 5;
                c.g[A0][0] = sx(algo.0);
                c.f[12] = amplitude.0;
                c.f[13] = frequency.0;
                c.f[14] = strength.0;
                callee::START_SHAKE.call(c, 0x0015_e618)?;
            }
        }
        if reload_request {
            c.cycles += 1;
        }
        c.g[A1][0] = lw(c, shake_requested(cam));

        c.cycles += 19;
        c.g[A0][0] = sx(algo.0);
        c.g[A2][0] = flag;
        c.f[12] = amplitude.0;
        c.f[13] = frequency.0;
        c.f[14] = strength.0;
        callee::APPLY_SHAKE.call(c, 0x0015_e634)?;
        let requested = lw(c, shake_requested(cam));
        c.g[V0][0] = requested;
        shake_strength(cam).sw(c, 0);
        shake_requested_prev(cam).sw(c, requested);
        shake_requested(cam).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        c.f[22] = sp.off(0x60).read_u32(c);
        c.f[21] = sp.off(0x58).read_u32(c);
        c.f[20] = sp.off(0x50).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

/// Shake strength from the rider's speed (target slot 3, four-lane VU0
/// length × scale): 0 below 60, (v − 60) / 60 up to 120, 1 above
/// (blocks 0x15e4c4..0x15e554).
fn shake_speed_factor(c: &mut Cpu, target: RiderCameraTarget, sp: Addr) -> Result<FpuF32, crate::call::Unwound> {
    c.cycles += 22;
    let vt = lw(c, target.vptr());
    c.g[V0][0] = vt;
    let get_velocity = VSlot::read(c, vt, 0x18);
    c.g[V1][0] = get_velocity.func;
    c.g[A1][0] = get_velocity.this(c.g[S0][0]);
    call_ptr(c, get_velocity.func as u32, 0x0015_e4dc)?;
    let scale = gp_const(c, SHAKE_SPEED_SCALE_GP);
    c.f[0] = scale.0;
    let vel = sp.read_vec4(c);
    vu0_scratch::set_vf(c, 3, XYZW, vel);
    let len = length4(c, vel);
    c.g[V0][0] = cfc2_q(len);
    let sixty = FpuF32(0x4270_0000);
    c.g[AT][0] = sixty.0 as u64;
    let speed = FpuF32(len.0) * scale;
    c.f[1] = speed.0;
    let factor = if !sixty.le(speed) {
        abi::set_fcc(c, false);
        c.cycles += 1;
        FpuF32(0)
    } else {
        abi::set_fcc(c, true);
        c.cycles += 5;
        let full = FpuF32(0x42f0_0000);
        c.g[AT][0] = full.0 as u64;
        c.f[0] = full.0;
        let below = speed.le(full);
        abi::set_fcc(c, below);
        c.cycles += 4;
        if below {
            let over = speed - sixty;
            c.f[0] = over.0;
            over.div(sixty)
        } else {
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            FpuF32::ONE
        }
    };
    Ok(factor)
}

/// `GameCamera_AddShake(cam, preset, f12 strength, f13 0)` with preset 3 /
/// 2 / 1 by boost level (blocks 0x15e558..0x15e5b0). Returns whether the
/// preset reload of block 0x15e5b4 follows (preset 1), else the original
/// reloaded it in the call's block.
fn request_boost_shake(c: &mut Cpu, cam: Addr, boost: FpuF32) -> Result<bool, crate::call::Unwound> {
    c.cycles += 5;
    let level_3 = gp_const(c, BOOST_SHAKE_3_GP);
    c.f[0] = level_3.0;
    let strong = abi::c_lt_s(c, level_3, boost);
    c.g[A0][0] = sx(cam.0);
    let (preset, ret) = if strong {
        (3, 0x0015_e578)
    } else {
        c.cycles += 5;
        let level_2 = gp_const(c, BOOST_SHAKE_2_GP);
        c.f[0] = level_2.0;
        if abi::c_lt_s(c, level_2, boost) {
            (2, 0x0015_e5a0)
        } else {
            (1, 0x0015_e5b4)
        }
    };
    c.cycles += if preset == 1 { 3 } else { 5 };
    c.f[13] = 0;
    c.g[A1][0] = preset;
    callee::GAME_CAMERA_ADD_SHAKE.call(c, ret)?;
    Ok(preset == 1)
}

/// gp offset of the `GameModule*`.
const GAME_MODULE_GP: u32 = 0x848;
const OVERRIDE_TIMER_STEP_GP: u32 = 0x6b24; // 1/60
/// The override timer restores the saved algorithm after this (seconds).
const OVERRIDE_TIMEOUT: FpuF32 = FpuF32(0x4120_0000); // 10.0

/// `AlgorithmListNode` (0x1c): algorithm, weight, rate, eased weight,
/// prev, next.
#[derive(Clone, Copy, Debug)]
pub struct AlgorithmListNode(pub Addr);

impl AlgorithmListNode {
    pub fn algorithm(self) -> Addr {
        self.0
    }
    pub fn weight(self) -> Addr {
        self.0.off(4)
    }
    pub fn rate(self) -> Addr {
        self.0.off(8)
    }
    pub fn eased(self) -> Addr {
        self.0.off(0xc)
    }
    pub fn next(self) -> Addr {
        self.0.off(0x14)
    }
}

/// `3w² − 2w³` as the controller computes it (`w²·3 − (w²+w²)·w`), with
/// f1 = w², f0 = 2w², then the products; returns (f1 = 3w², f0 = 2w³).
fn smoothstep_terms(w: FpuF32, three: FpuF32) -> (FpuF32, FpuF32) {
    let ww = w * w;
    let ww2 = ww + ww;
    (ww * three, ww2 * w)
}

/// `CameraAlgorithm::Tick(arg)` (vtable slot 9) of `algo`, returning to
/// `ret`; a0..v1 as the original leaves them at the call.
fn tick(c: &mut Cpu, algo: u64, arg: u64, ret: u32) -> Flow {
    c.g[A1][0] = arg;
    let vt = lw(c, Addr(algo as u32).off(0x10));
    c.g[V0][0] = vt;
    let slot = VSlot::read(c, vt, 0x48);
    c.g[A0][0] = slot.delta;
    c.g[V1][0] = slot.func;
    c.g[A0][0] = slot.this(algo);
    call_ptr(c, slot.func as u32, ret)
}

/// `ChaseCamController_Update(ctl)` (0x161bb8, vtable slot 2): unless the
/// gp debug bit freezes it, update the auto mode, tick the head algorithm
/// of the blend list and move its weight toward 1 by its rate; every other
/// node's weight moves toward 0 by the head's rate (algorithm 0x4a drops
/// at once) and nodes that reach 0 are removed, the rest are ticked. The
/// eased weights `min(3w² − 2w³, 1)` go to +0x0c. Then the timed override
/// (0x5d) is counted (+0x38) and switches back to the saved algorithm
/// after 10 s. Structured loop over the nodes (event check at 0x161cb0).
pub fn chase_cam_controller_update(c: &mut Cpu) {
    const HEAD: u32 = 0x0016_1cb0;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        let frozen = lw(c, Addr((c.g[GP][0] as u32).wrapping_sub(SHAKE_DISABLE_GP))) & 1;
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let ctl = ChaseCamController(Addr(abi::arg(c, 0)));
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[V0][0] = frozen;
        frame.save_ra(c, 0);
        let sp = Addr(frame.at(c, 0));
        for (f, off) in [(22, 0x50), (21, 0x48), (20, 0x40)] {
            sp.off(off).write_u32(c, c.f[f]);
        }
        let module_slot = Addr((c.g[GP][0] as u32).wrapping_sub(GAME_MODULE_GP));
        let mut loaded_module = false;
        if frozen == 0 {
            c.cycles += 20;
            callee::UPDATE_AUTO_MODE.call(c, 0x0016_1bf0)?;
            let list = lw(c, ctl.0.off(8));
            c.g[V0][0] = list;
            let head = AlgorithmListNode(Addr(lw(c, Addr(list as u32)) as u32));
            c.g[S0][0] = sx(head.0 .0);
            let algo = lw(c, head.algorithm());
            c.g[A2][0] = algo;
            tick(c, algo, 0, 0x0016_1c14)?;

            // The head blends in: w += rate, up to 1.
            let (one, three) = (FpuF32::ONE, FpuF32(0x4040_0000));
            let rate = head.rate().read_fpu(c);
            c.f[20] = rate.0;
            c.g[AT][0] = one.0 as u64;
            c.f[1] = one.0;
            let w = head.weight().read_fpu(c);
            c.f[3] = w.0;
            let w_up = rate + one;
            let w = if abi::c_lt_s(c, w_up, w) {
                c.f[0] = w_up.0;
                c.cycles += 2;
                w - rate
            } else {
                let limit = one - rate;
                c.f[0] = limit.0;
                c.cycles += 4;
                if abi::c_lt_s(c, w, limit) {
                    c.cycles += 1;
                    w + rate
                } else {
                    one
                }
            };
            c.cycles += 13;
            let (ww3, www2) = smoothstep_terms(w, three);
            c.g[AT][0] = one.0 as u64;
            c.f[4] = three.0;
            c.f[3] = one.0;
            c.f[1] = ww3.0;
            c.f[2] = w.0;
            let mut eased = ww3 - www2;
            head.weight().write_fpu(c, w);
            if abi::c_lt_s(c, one, eased) {
                c.cycles += 1;
                eased = one;
            }
            c.f[0] = eased.0;

            c.cycles += 4;
            head.eased().write_fpu(c, eased);
            let next = lw(c, head.next());
            c.g[S0][0] = next;
            c.g[V1][0] = lw(c, module_slot);
            loaded_module = true;
            if next != 0 {
                c.cycles += 5;
                c.f[22] = three.0;
                c.f[21] = one.0;
                c.g[S2][0] = ALGO_KEEP_AUTO;
                c.f[1] = Addr(next as u32).off(4).read_u32(c);
                fade_out_nodes(c, ctl, HEAD)?;
                loaded_module = false;
            }
        }
        if !loaded_module {
            c.cycles += 1;
            c.g[V1][0] = lw(c, module_slot);
        }
        override_timer(c, ctl)?;
        c.cycles += 8;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        for (f, off) in [(22, 0x50), (21, 0x48), (20, 0x40)] {
            c.f[f] = Addr(frame.at(c, off)).read_u32(c);
        }
        frame.close(c);
        Ok(())
    })
}

/// The loop of [`chase_cam_controller_update`] over the non-head nodes
/// (from s0; f20 = the head's rate, f21 = 1, f22 = 3, f1 = the node's
/// weight): fade the weight out, ease it, remove or tick the node.
fn fade_out_nodes(c: &mut Cpu, ctl: ChaseCamController, head: u32) -> Flow {
    loop {
        let node = AlgorithmListNode(Addr(c.g[S0][0] as u32));
        let (rate, one, three) = (FpuF32(c.f[20]), FpuF32(c.f[21]), FpuF32(c.f[22]));
        let zero = FpuF32(0);
        let w = FpuF32(c.f[1]);
        c.cycles += 4;
        let w = if abi::c_lt_s(c, rate, w) {
            c.cycles += 2;
            w - rate
        } else {
            let floor = -rate;
            c.f[0] = floor.0;
            c.cycles += 5;
            if abi::c_lt_s(c, w, floor) {
                w + rate
            } else {
                zero
            }
        };
        c.cycles += 9;
        let (ww3, www2) = smoothstep_terms(w, three);
        c.f[0] = www2.0;
        c.f[2] = w.0;
        let mut eased = ww3 - www2;
        let over = abi::c_lt_s(c, one, eased);
        node.weight().write_fpu(c, w);
        if over {
            c.cycles += 2;
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            eased = FpuF32::ONE;
        }
        c.f[1] = eased.0;

        c.cycles += 5;
        node.eased().write_fpu(c, eased);
        let algo = lw(c, node.algorithm());
        c.g[V0][0] = algo;
        let id = lw(c, Addr(algo as u32).off(0xc));
        c.g[V1][0] = id;
        if id == c.g[S2][0] {
            node.weight().sw(c, 0);
        }

        c.cycles += 6;
        let w = node.weight().read_fpu(c);
        c.f[0] = w.0;
        c.f[1] = 0;
        let mut remove = c_le_s(c, w, zero);
        if !remove {
            c.cycles += 5;
            let eased = node.eased().read_fpu(c);
            c.f[0] = eased.0;
            remove = c_le_s(c, eased, zero);
            if remove {
                c.cycles += 1;
            }
        }
        if remove {
            c.cycles += 4;
            c.g[A0][0] = lw(c, ctl.0.off(8));
            c.g[A1][0] = c.g[S0][0];
            callee::ALGORITHM_LIST_REMOVE.call(c, 0x0016_1d54)?;
            c.g[S0][0] = c.g[V0][0];
        } else {
            c.cycles += 7;
            let algo = lw(c, node.algorithm());
            c.g[A2][0] = algo;
            tick(c, algo, 1, 0x0016_1d74)?;
            c.g[S0][0] = lw(c, node.next());
        }
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            return Ok(());
        }
        c.f[1] = Addr(c.g[S0][0] as u32).off(4).read_u32(c);
        if crate::events::event_due(c) {
            crate::events::deliver_event_here(c, head);
        }
    }
}

/// The tail of [`chase_cam_controller_update`] (blocks 0x161d84..0x161e34):
/// with the view manager's race state (`GameModule+0x84 → +0x28`), clear
/// the override timer unless a race runs (state 1..9; also while +0x484
/// is clear); count the timed override 0x5d and switch back to the saved
/// algorithm after 10 s. v1 holds the `GameModule*` on entry.
fn override_timer(c: &mut Cpu, ctl: ChaseCamController) -> Flow {
    c.cycles += 6;
    c.g[A1][0] = 0;
    let vm = lw(c, Addr(c.g[V1][0] as u32).off(0x84));
    c.g[V0][0] = vm;
    let race = Addr(lw(c, Addr(vm as u32).off(0x28)) as u32);
    c.g[V1][0] = sx(race.0);
    let state = lw(c, race);
    c.g[V0][0] = state;
    c.g[A0][0] = 0;
    if state != 0 {
        c.cycles += 1;
        c.g[A0][0] = ((state as i64) < 10) as u64;
    }
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        let v = lw(c, race.off(0x484));
        c.g[V0][0] = v;
        c.g[A1][0] = (v == 0) as u64;
    }
    c.cycles += 2;
    let timer = ctl.0.off(0x38);
    if c.g[A1][0] != 0 {
        timer.sw(c, 0);
    }
    c.cycles += 3;
    let state = lw(c, race);
    c.g[V0][0] = state;
    c.g[V1][0] = 0;
    if state != 0 {
        c.cycles += 1;
        c.g[V1][0] = ((state as i64) < 10) as u64;
    }
    c.cycles += 2;
    c.g[V0][0] = ALGO_TIMED_OVERRIDE;
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        timer.sw(c, 0);
    } else {
        c.cycles += 3;
        let id = lw(c, ctl.0.off(0x2c));
        c.g[V1][0] = id;
        if id != ALGO_TIMED_OVERRIDE {
            timer.sw(c, 0);
        } else {
            c.cycles += 4;
            let current = lw(c, current_algorithm(ctl));
            c.g[V0][0] = current;
            let restore = lw(c, ctl.restore_algorithm());
            c.g[A1][0] = restore;
            if current == restore {
                timer.sw(c, 0);
            } else {
                c.cycles += 9;
                let step = gp_const(c, OVERRIDE_TIMER_STEP_GP);
                c.f[1] = step.0;
                c.g[AT][0] = OVERRIDE_TIMEOUT.0 as u64;
                c.f[2] = OVERRIDE_TIMEOUT.0;
                let t = timer.read_fpu(c) + step;
                c.f[0] = t.0;
                let expired = abi::c_lt_s(c, OVERRIDE_TIMEOUT, t);
                timer.write_fpu(c, t);
                if expired {
                    c.cycles += 8;
                    timer.sw(c, 0);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[12] = FpuF32::ONE.0;
                    c.g[A2][0] = 0;
                    callee::SET_OVERRIDE_ALGORITHM.call(c, 0x0016_1e28)?;
                    restore_s0(c);
                    return Ok(());
                }
            }
        }
    }
    c.cycles += 1;
    restore_s0(c);
    Ok(())
}

/// `lq s0, 0x30(sp)` of [`chase_cam_controller_update`]'s frame.
fn restore_s0(c: &mut Cpu) {
    Frame { size: 0x60 }.restore(c, S0, 0x30);
}

#[cfg(test)]
mod tests {
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{covered, machine, vtable, FAKE, LOG_RESULTS, LOG_UNWIND_AT};
    use crate::difftest::{check, check_paths, fill, put_vec, require_paths, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::funcs::GuestFn;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 400;

    /// A random object pointer in the lower half of the test data area
    /// (`fakes::ptr` objects can reach into the fakes' log at 0x900000).
    fn ptr(r: &mut Rng) -> u32 {
        crate::difftest::DATA + (r.next_u32() & 0x7_fff0)
    }

    /// `require_paths` with the fakes never unwinding (so a path's total
    /// is its blocks plus the fakes' 3..10 cycles per call).
    fn paths_no_unwind(g: GuestFn, seeds: u32, paths: &[u64], setup: &dyn Fn(&mut Cpu, u32)) {
        require_paths(g, seeds, paths, &|c, s| {
            setup(c, s);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        });
    }

    /// A float in [-range, range) or an edge case.
    fn f(r: &mut Rng, range: f32) -> u32 {
        if r.next_u32() % 8 == 0 {
            r.edge_f32()
        } else {
            r.f32(range)
        }
    }

    fn vec(r: &mut Rng, range: f32) -> [u32; 4] {
        [f(r, range), f(r, range), f(r, range), f(r, range)]
    }

    /// A target (vptr with the given slots, rider) for `algo + 0x30`.
    fn target(c: &mut Cpu, r: &mut Rng, algo: u32, slots: &[u32]) -> (u32, u32) {
        let (t, rider) = (ptr(r), ptr(r));
        c.bus.write32(algo + 0x30, t);
        c.bus.write32(t + 4, rider);
        let vt = vtable(c, r, slots);
        c.bus.write32(t, vt);
        (t, rider)
    }

    fn distance_reset_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd301, 1);
        let algo = ptr(&mut r);
        c.g[A0][0] = algo as u64;
        let (_, rider) = target(c, &mut r, algo, &[0x8]);
        put_vec(c, rider + 0x1b0, vec(&mut r, 1.0));
        fill(c, &mut r, 0x4ff160, 4);
        let max = 400.0 + (r.next_u32() % 1000) as f32;
        c.bus.write32(GAME_GP - 0x6778, max.to_bits());
        let look = vec(&mut r, 1e4);
        put_vec(c, algo + 0x20, look);
        // Distances below 20, in range and beyond the limit.
        let d = match seed % 3 {
            0 => (r.next_u32() % 20) as f32,
            1 => 20.0 + (r.next_u32() % 380) as f32,
            _ => max + (r.next_u32() % 1000) as f32,
        } / 2.0;
        let l = |i: usize| f32::from_bits(look[i]);
        let eye = [(l(0) + d).to_bits(), (l(1) - d).to_bits(), (l(2) + d).to_bits(), (l(3) - d).to_bits()];
        put_vec(c, algo + 0x40, if seed % 17 == 0 { vec(&mut r, 1e30) } else { eye });
    }

    #[test]
    fn check_distance_reset() {
        let g = ssx3_ee::s02::f_00168150__generated;
        covered(g, super::check_distance_reset, SEEDS, &distance_reset_setup);
        paths_no_unwind(g, SEEDS, &[34, 79, 92], &distance_reset_setup);
    }

    #[test]
    fn init_from_rider() {
        let g = ssx3_ee::s02::f_00166550__generated;
        covered(g, super::init_from_rider, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd302, 1);
            let algo = ptr(&mut r);
            c.g[A0][0] = algo as u64;
            target(c, &mut r, algo, &[0x20]);
            fill(c, &mut r, 0x4ff160, 4);
            c.f[12] = f(&mut r, 500.0);
            c.f[13] = f(&mut r, 500.0);
            fill(c, &mut r, algo + 0x80, 4);
        });
    }

    #[test]
    fn camera_base_get_pitch() {
        check(ssx3_ee::s01::f_0015d880__generated, super::camera_base_get_pitch, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd303, 0);
            let cam = ptr(&mut r);
            c.g[A0][0] = cam as u64;
            let q = if seed % 4 == 0 { vec(&mut r, 2.0) } else { [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)] };
            put_vec(c, cam + 0x30, q);
        });
    }

    #[test]
    fn game_camera_reset_default_chase() {
        covered(ssx3_ee::s01::f_0015e030__generated, super::game_camera_reset_default_chase, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd304, 2);
            let cam = ptr(&mut r);
            c.g[A0][0] = cam as u64;
            let algo = cam + 0xc0;
            // What ChaseCameraAlgorithm_Reset reads (see dispatch tests).
            let vt = ptr(&mut r);
            c.bus.write16(vt + 0x18, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + 0x1c, FAKE + 0x20);
            c.bus.write32(algo + 0x10, vt);
            let (_, rider) = target(c, &mut r, algo, &[]);
            let ivt = vtable(c, &mut r, &[0x38]);
            c.bus.write32(rider + 0x6c0, ivt);
        });
    }

    #[test]
    fn is_script_camera_active() {
        check_paths(ssx3_ee::s01::f_0015d020__generated, super::is_script_camera_active, SEEDS, &[6, 10, 11], &|c, seed| {
            let mut r = machine(c, seed, 0xd305, 0);
            let ctl = ptr(&mut r);
            c.g[A0][0] = ctl as u64;
            c.bus.write32(ctl + 0x30, r.next_u32());
            let ids = [0x5d, 0x3c, 0x3d, r.next_u32(), 0xffff_ff3c];
            c.bus.write32(ctl + 0x2c, ids[(seed % 5) as usize]);
        });
    }

    fn set_override_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd306, 4);
        let ctl = ptr(&mut r);
        c.g[A0][0] = ctl as u64;
        fill(c, &mut r, ctl, 0x11);
        c.g[A1][0] = match seed % 4 {
            0 => 0x4c,
            1 => 0x1_0000_004c,
            _ => r.next_u32() as u64 % 0x70,
        };
        if seed % 3 == 0 {
            c.bus.write32(ctl + 0x2c, 0x5d);
        }
    }

    #[test]
    fn set_override_algorithm() {
        let g = ssx3_ee::s01::f_00162060__generated;
        check(g, super::set_override_algorithm, SEEDS, &set_override_setup);
        // The paths without the SetAlgorithm call (a script camera runs).
        require_paths(g, SEEDS, &[20, 21], &set_override_setup);
    }

    driven!(gen_all_views_ready, ssx3_ee::s05::f_0022e920__generated);
    driven!(hand_all_views_ready, super::game_view_man_all_views_ready);
    driven!(gen_update_cameras, ssx3_ee::s05::f_0022e840__generated);
    driven!(hand_update_cameras, super::game_view_man_update_cameras);

    /// A view manager with 0..3 cameras (each a vptr at +0x90 whose slot
    /// +0x18 is a fake, and a ready flag at +0xb4). A fourth camera pointer
    /// would overwrite the count at +0x10.
    fn views(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        // Separate blocks: a camera's fields must not overlap the list
        // (and the list is written after the vtable).
        let vt = vtable(c, r, &[0x18]);
        let vm = crate::difftest::DATA + 0x4_0000 + 0x40 * (r.next_u32() % 64);
        let n = [0, 1, 2, 3, 3][(seed / 3 % 5) as usize];
        c.bus.write32(vm + 0x10, n);
        for i in 0..n {
            let cam = crate::difftest::DATA + 0x5_0000 + 0x100 * i + 0x10 * (r.next_u32() % 8);
            c.bus.write32(vm + 4 + 4 * i, cam);
            c.bus.write32(cam + 0x90, vt);
            c.bus.write32(cam + 0xb4, if r.next_u32() % 25 == 0 { 0 } else { r.next_u32() | 1 });
        }
        vm
    }

    #[test]
    fn game_view_man_all_views_ready() {
        check_events(gen_all_views_ready, hand_all_views_ready, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd307, 0);
            let vm = views(c, &mut r, seed);
            c.g[A0][0] = vm as u64;
            let mode = schedule(c, &mut r, seed, 0x0022_e920);
            // The loop is short (at most two back edges): make events due early.
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 12) as u64;
            }
            mode
        });
    }

    #[test]
    fn game_view_man_update_cameras() {
        check_events(gen_update_cameras, hand_update_cameras, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd308, 0);
            let vm = views(c, &mut r, seed);
            c.g[A0][0] = vm as u64;
            schedule(c, &mut r, seed, 0x0022_e840)
        });
    }

    /// An algorithm the real `CheckDistanceReset` can run on (target with
    /// a fake slot 1, forward, world up, distance limit), plus a
    /// `RiderCamState` with plausible speeds and random flags.
    fn step_setup(c: &mut Cpu, r: &mut Rng) -> (u32, u32) {
        let algo = ptr(r);
        c.g[A0][0] = algo as u64;
        let (_, rider) = target(c, r, algo, &[0x8]);
        put_vec(c, rider + 0x1b0, vec(r, 1.0));
        fill(c, r, 0x4ff140, 12);
        c.bus.write32(GAME_GP - 0x6778, (300.0 + (r.next_u32() % 400) as f32).to_bits());
        fill(c, r, algo, 0xf0);
        put_vec(c, algo + 0x20, vec(r, 300.0));
        put_vec(c, algo + 0x40, vec(r, 300.0));
        let st = ptr(r);
        c.g[A1][0] = st as u64;
        fill(c, r, st, 0x1a);
        put_vec(c, st + 0x30, vec(r, 500.0));
        let speed = match r.next_u32() % 4 {
            0 => r.f32(500.0),
            1 => 1388.9f32.to_bits(),
            _ => r.f32(4000.0) & 0x7fff_ffff,
        };
        c.bus.write32(st + 0x44, speed);
        c.bus.write32(st + 0x40, r.next_u32() % 6);
        for o in [0x48, 0x4c, 0x54] {
            c.bus.write32(st + o, f(r, 2.0));
        }
        for o in [0x58, 0x5c, 0x60, 0x64] {
            c.bus.write32(st + o, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() % 3 });
        }
        for i in 0..8 {
            c.f[12 + i] = f(r, 100.0);
        }
        // gp constants of the steps.
        fill(c, r, GAME_GP - 0x6b00, 0x60);
        (algo, st)
    }

    #[test]
    fn apply_target_height() {
        let g = ssx3_ee::s01::f_00162b90__generated;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0xd310, 1);
            step_setup(c, &mut r);
            c.bus.write32(GAME_GP - 0x6ae8, 1388.9f32.to_bits());
            c.bus.write32(GAME_GP - 0x6ae4, 5.15e-4f32.to_bits());
            c.f[16] = r.f32(1.0);
        };
        covered(g, super::apply_target_height, SEEDS, &setup);
        paths_no_unwind(g, SEEDS, &[88, 89, 90], &|c, s| {
            setup(c, s);
            quiet_reset(c);
        });
    }

    #[test]
    fn apply_fov() {
        let g = ssx3_ee::s01::f_001633b0__generated;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0xd311, 1);
            let (_, st) = step_setup(c, &mut r);
            c.bus.write32(st + 0x40, [0, 1, 2, 5, 0xffff_ffff][(seed % 5) as usize]);
        };
        covered(g, super::apply_fov, SEEDS, &setup);
    }

    /// Make the closing `CheckDistanceReset` take its 34-cycle no-reset
    /// path (no limit, no world-up offsets, eye 128 from the look-at).
    fn quiet_reset(c: &mut Cpu) {
        let algo = c.g[A0][0] as u32;
        c.bus.write32(GAME_GP - 0x6778, 1e30f32.to_bits());
        put_vec(c, algo + 0x40, [0, 0, 0, 0x4300_0000]);
        put_vec(c, algo + 0x20, [0, 0, 0, 0]);
        put_vec(c, 0x4ff160, [0, 0, 0, 0]);
    }

    fn follow_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd312, 1);
        let (algo, st) = step_setup(c, &mut r);
        c.bus.write32(GAME_GP - 0x6ac4, 1e-4f32.to_bits());
        c.f[12] = (100.0 + (r.next_u32() % 400) as f32).to_bits();
        c.f[14] = r.f32(1500.0);
        c.f[16] = r.f32(3.0);
        c.f[17] = r.f32(1.0) & 0x7fff_ffff;
        c.f[18] = r.f32(1.0) & 0x7fff_ffff;
        c.bus.write32(algo + 0x1d0, (r.f32(800.0) & 0x7fff_ffff) | 0x4300_0000);
        c.bus.write32(st + 0x4c, r.f32(2.0) & 0x7fff_ffff);
        c.bus.write32(algo + 0x1ec, r.f32(2.0) & 0x7fff_ffff);
        c.bus.write32(algo + 0x1e8, r.f32(10.0));
    }

    #[test]
    fn apply_follow_distance() {
        let g = ssx3_ee::s01::f_00163010__generated;
        covered(g, super::apply_follow_distance, SEEDS, &follow_setup);
        paths_no_unwind(g, SEEDS, &[111, 112, 113, 114, 115, 116], &|c, s| {
            follow_setup(c, s);
            quiet_reset(c);
        });
    }

    fn camera_height_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd313, 1);
        step_setup(c, &mut r);
        for (off, v) in [(0x6ac0, 1388.9f32), (0x6abc, 833.3), (0x6ab8, 1666.7), (0x6ab4, 1e-4)] {
            c.bus.write32(GAME_GP - off, v.to_bits());
        }
        let st = c.g[A1][0] as u32;
        c.bus.write32(st + 0x4c, r.f32(2.0) & 0x7fff_ffff);
        c.f[16] = r.f32(1.0);
    }

    #[test]
    fn apply_camera_height() {
        let g = ssx3_ee::s01::f_00163270__generated;
        covered(g, super::apply_camera_height, SEEDS, &camera_height_setup);
        // 11 [+1 above 50 km/h] + (6 | 13) + 2 + (2 | 6) + 39, then 34.
        paths_no_unwind(g, SEEDS, &[94, 98, 101, 102, 105, 106], &|c, s| {
            camera_height_setup(c, s);
            quiet_reset(c);
        });
    }

    #[test]
    fn update_handplant_fade() {
        let g = ssx3_ee::s01::f_00162998__generated;
        check_paths(g, super::update_handplant_fade, SEEDS, &[8, 21, 28, 29, 30], &|c, seed| {
            let mut r = machine(c, seed, 0xd314, 0);
            let (algo, st) = (ptr(&mut r), ptr(&mut r) + 0x10_0000);
            c.g[A0][0] = algo as u64;
            c.g[A1][0] = st as u64;
            c.bus.write32(st + 0x5c, if seed % 4 == 0 { r.next_u32() | 1 } else { 0 });
            let frames = [0, 119, 120, 500, 819, 820, 821, 5000, 0xffff_fff0, r.next_u32()];
            c.bus.write32(algo + 0x2c0, frames[(seed / 4 % 10) as usize]);
            c.bus.write32(GAME_GP - 0x6af4, if seed % 3 == 0 { r.f32(0.01) } else { (1.0f32 / 700.0).to_bits() });
        });
    }

    fn handplant_hold_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd315, 1);
        let (algo, st) = step_setup(c, &mut r);
        c.bus.write32(GAME_GP - 0x6aa4, (1.0f32 / 60.0).to_bits());
        c.bus.write32(st + 0x5c, if seed % 4 == 0 { 1 } else { 0 });
        let t = match seed / 4 % 4 {
            0 => 5.0,
            1 => 7.0,
            _ => (r.next_u32() % 300) as f32 / 60.0,
        };
        c.bus.write32(algo + 0x1e0, t.to_bits());
        c.f[15] = ((r.next_u32() % 400) as f32 / 60.0).to_bits();
        put_vec(c, algo + 0xb0, vec(&mut r, 300.0));
        c.bus.write32(algo + 0x2b8, r.f32(100.0));
    }

    #[test]
    fn apply_handplant_hold() {
        let g = ssx3_ee::s01::f_00163450__generated;
        covered(g, super::apply_handplant_hold, SEEDS, &handplant_hold_setup);
        paths_no_unwind(g, SEEDS, &[36 + 34, 26 + 34, 33 + 34, 75 + 34], &|c, s| {
            handplant_hold_setup(c, s);
            quiet_reset(c);
        });
    }

    fn side_offset_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd316, 1);
        let (algo, _) = step_setup(c, &mut r);
        if seed % 5 == 0 {
            // d parallel to up: |s| = 0.
            let up = [0, 0, 0x3f80_0000, 0];
            put_vec(c, 0x4ff160, up);
            put_vec(c, algo + 0x20, [0, 0, 0x4300_0000, 0]);
            put_vec(c, algo + 0x40, [0, 0, 0, 0]);
        }
    }

    #[test]
    fn apply_side_offset() {
        let g = ssx3_ee::s01::f_00163158__generated;
        covered(g, super::apply_side_offset, SEEDS, &side_offset_setup);
        // 38 / 70 cycles, plus a `CheckDistanceReset` without reset.
        paths_no_unwind(g, SEEDS, &[72, 104], &side_offset_setup);
    }

    fn rail_pullback_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd317, 2);
        let (algo, _) = step_setup(c, &mut r);
        // Slot 24 (rail flag) next to slot 1 (CheckDistanceReset's).
        let t = c.bus.read32(algo + 0x30);
        let vt = c.bus.read32(t);
        c.bus.write16(vt + 0xc0, (r.next_u32() as i16 & !3) as u16);
        c.bus.write32(vt + 0xc4, FAKE + 0x40);
        c.bus.write32(LOG_RESULTS, if seed % 2 == 0 { 0 } else { 1 });
        if seed % 5 == 0 {
            // o parallel to up: |s| = 0.
            put_vec(c, 0x4ff160, [0, 0, 0x3f80_0000, 0]);
            put_vec(c, algo + 0x40, [0, 0, 0x4300_0000, 0]);
            put_vec(c, algo + 0x20, [0, 0, 0, 0]);
        }
    }

    #[test]
    fn apply_rail_pullback() {
        let g = ssx3_ee::s01::f_001641c0__generated;
        covered(g, super::apply_rail_pullback, SEEDS, &rail_pullback_setup);
    }

    fn rail_align_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd318, 1);
        let (_, st) = step_setup(c, &mut r);
        put_vec(c, st + 0x20, vec(&mut r, 500.0));
        if seed % 3 == 0 {
            c.bus.write32(st + 0x44, r.f32(100.0) | 0x8000_0000);
        }
        c.bus.write32(st + 0x64, if seed % 2 == 0 { 0 } else { 1 });
        fill(c, &mut r, 0x4ff120, 4);
    }

    #[test]
    fn apply_rail_align() {
        let g = ssx3_ee::s01::f_001646a0__generated;
        covered(g, super::apply_rail_align, SEEDS, &rail_align_setup);
        // 8 + (55 | 2) + 9 [+2] + 37, then 34.
        paths_no_unwind(g, SEEDS, &[90, 92, 143, 145], &|c, s| {
            rail_align_setup(c, s);
            quiet_reset(c);
            c.bus.write32(c.g[A0][0] as u32 + 0x44, 0x4300_0000);
            put_vec(c, 0x4ff120, [0; 4]);
            put_vec(c, c.g[A0][0] as u32 + 0xc0, [0; 4]);
        });
    }

    fn low_speed_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd319, 1);
        let (algo, st) = step_setup(c, &mut r);
        for (off, v) in [(0x692c, 0.036f32), (0x6928, 1.0 / 60.0), (0x6924, 1.0 / 60.0)] {
            c.bus.write32(GAME_GP - off, v.to_bits());
        }
        c.f[12] = 15.0f32.to_bits();
        c.f[13] = [1.5f32, 0.01, 0.5][(seed % 3) as usize].to_bits();
        c.f[14] = [1.5f32, 0.02, 0.5][(seed / 3 % 3) as usize].to_bits();
        let speed = if seed % 2 == 0 { r.f32(400.0) & 0x7fff_ffff } else { r.f32(4000.0) };
        c.bus.write32(st + 0x44, speed);
        c.bus.write32(st + 0x60, (r.next_u32() % 4 == 0) as u32);
        c.bus.write32(st + 0x5c, (r.next_u32() % 4 == 0) as u32);
        c.bus.write32(algo + 0x300, [0, 1, 2, 3, 3, 7][(seed / 9 % 6) as usize]);
        let w = [0.0f32, 1.0, 0.995, 0.005, 0.5];
        c.bus.write32(algo + 0x248, w[(r.next_u32() % 5) as usize].to_bits());
        put_vec(c, algo + 0x1a0, vec(&mut r, 300.0));
    }

    #[test]
    fn apply_low_speed_hold() {
        let g = ssx3_ee::s01::f_001643a8__generated;
        covered(g, super::apply_low_speed_hold, 1000, &low_speed_setup);
    }

    fn air_transitions_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd31a, 1);
        let (algo, st) = step_setup(c, &mut r);
        let t = c.bus.read32(algo + 0x30);
        let vt = c.bus.read32(t);
        c.bus.write16(vt + 0x48, (r.next_u32() as i16 & !3) as u16);
        c.bus.write32(vt + 0x4c, FAKE + 0x40);
        c.bus.write32(st + 0x60, (seed % 3 != 0) as u32);
        c.bus.write32(algo + 0x2d4, (seed / 3 % 3 == 0) as u32);
        c.bus.write32(algo + 0x2e0, (r.next_u32() % 3 == 0) as u32);
        c.bus.write32(algo + 0x2cc, r.next_u32() % 64);
        // Velocity: steeply up/down or random; speed sign.
        let v = match r.next_u32() % 3 {
            0 => [r.f32(10.0), r.f32(10.0), 0x44fa_0000, 0],
            _ => vec(&mut r, 500.0),
        };
        put_vec(c, st + 0x20, v);
        // The AI frame counter: *(gp-0x848) -> +0x84 -> +0x0c -> +0x08.
        let (module, x, ai) = (DATA + 0xa_0000, DATA + 0xa_0100, DATA + 0xa_0200);
        c.bus.write32(GAME_GP - 0x848, module);
        c.bus.write32(module + 0x84, x);
        c.bus.write32(x + 0xc, ai);
        c.bus.write32(ai + 8, r.next_u32() % 128);
        for (off, v) in [(0x6af0, 0.85f32), (0x6aec, 0.1)] {
            c.bus.write32(GAME_GP - off, v.to_bits());
        }
        // Fake stack for the previous-normal out parameter.
        fill(c, &mut r, crate::difftest::SP - 0x60, 0x18);
    }

    #[test]
    fn update_air_transitions() {
        let g = ssx3_ee::s01::f_00162a20__generated;
        covered(g, super::update_air_transitions, SEEDS, &air_transitions_setup);
        // Ground (19, 23), airborne (26), takeoff without / with a moving
        // check (50, 73) + countdown (2, 4).
        paths_no_unwind(g, SEEDS, &[19, 23, 26, 50, 52, 54, 73, 75, 77], &air_transitions_setup);
    }

    fn compute_angles_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd31b, 0);
        let algo = ptr(&mut r);
        c.g[A0][0] = algo as u64;
        fill(c, &mut r, algo, 0x18);
        put_vec(c, algo + 0x20, vec(&mut r, 500.0));
        let eye = match seed % 6 {
            // Vertical: the flattened direction is zero.
            0 => {
                let l = c.bus.read32(algo + 0x20);
                let l1 = c.bus.read32(algo + 0x24);
                let l3 = c.bus.read32(algo + 0x2c);
                [l, l1, r.f32(100.0), l3]
            }
            _ => vec(&mut r, 500.0),
        };
        put_vec(c, algo + 0x40, eye);
        let axes = if seed % 4 == 0 { [vec(&mut r, 1.0), vec(&mut r, 1.0)] } else {
            [[0x3f80_0000, 0, 0, 0], [0, 0x3f80_0000, 0, 0]]
        };
        put_vec(c, 0x4ff140, axes[0]);
        put_vec(c, 0x4ff150, if seed % 7 == 0 { [0; 4] } else { axes[1] });
        let eps = if seed % 5 == 0 { 1e9f32 } else { 1e-6 };
        c.bus.write32(GAME_GP - 0x68b4, eps.to_bits());
        c.bus.write32(GAME_GP - 0x68ac, eps.to_bits());
        for off in [0x68b0, 0x68a8, 0x68a4] {
            c.bus.write32(GAME_GP - off, r.f32(7.0));
        }
    }

    #[test]
    fn compute_angles() {
        let g = ssx3_ee::s02::f_00166640__generated;
        check(g, super::compute_angles, 1000, &compute_angles_setup);
    }

    fn post_update_setup(c: &mut Cpu, seed: u32) {
        compute_angles_setup(c, seed);
        let mut r = machine_keep(c, seed, 0xd31c, 1);
        let algo = c.g[A0][0] as u32;
        fill(c, &mut r, algo + 0x50, 0xe0);
        let vt = vtable(c, &mut r, &[0x38]);
        c.bus.write32(algo + 0x10, vt);
        // The tuning record the fake leaves in the frame (flag at +0x10).
        let sp = crate::difftest::SP - 0x40;
        fill(c, &mut r, sp, 4);
        c.bus.write32(sp + 0x10, (seed % 2) as u32);
    }

    /// `machine`'s unwind index and RNG without re-scrambling.
    fn machine_keep(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        let unwind = if calls > 0 && r.next_u32() % 3 == 0 { r.next_u32() % calls } else { u32::MAX };
        c.bus.write32(LOG_UNWIND_AT, unwind);
        r
    }

    #[test]
    fn post_update() {
        let g = ssx3_ee::s02::f_00166228__generated;
        covered(g, super::post_update, SEEDS, &post_update_setup);
    }

    fn auto_mode_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd31d, 3);
        let ctl = ptr(&mut r);
        c.g[A0][0] = ctl as u64;
        fill(c, &mut r, ctl, 0x11);
        c.bus.write32(ctl + 0x1c, [0x3d, 0x3d, 0x10, 0x4a][(seed % 4) as usize]);
        c.bus.write32(ctl + 0x18, [0x4a, 0x3d, 0x10][(seed / 4 % 3) as usize]);
        // Flags: override/script/spoke bits mostly set (no immediate switch).
        c.bus.write32(ctl + 0x30, r.next_u32() | if seed % 8 == 0 { 0 } else { 1 });
        c.bus.write32(ctl + 0x3c, (seed / 12 % 2) as u32);
        let (t, rider, st) = (ptr(&mut r), ptr(&mut r), DATA + 0xc_0000);
        c.bus.write32(ctl + 0x20, t);
        c.bus.write32(t + 4, rider);
        let vt = vtable(c, &mut r, &[0x18]);
        c.bus.write32(t, vt);
        c.bus.write32(rider + 0x77c, st);
        c.bus.write32(st + 0xde0, if r.next_u32() % 2 == 0 { 5 } else { r.next_u32() % 8 });
        c.bus.write32(rider + 0x870, r.next_u32() % 4);
        c.bus.write32(GAME_GP - 0x6b28, 0.036f32.to_bits());
        c.bus.write32(GAME_GP - 0x6b20, r.f32(10.0));
        // The velocity the fake leaves in the frame (sp+0).
        let sp = crate::difftest::SP - 0x50;
        let v = [r.f32(1500.0), r.f32(1500.0), r.f32(300.0), 0];
        put_vec(c, sp, v);
    }

    #[test]
    fn update_auto_mode() {
        let g = ssx3_ee::s01::f_00161ab0__generated;
        covered(g, super::update_auto_mode, SEEDS, &auto_mode_setup);
    }

    fn update_shake_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xd31e, 2);
        let cam = DATA + 0x2_0000;
        c.g[A0][0] = cam as u64;
        fill(c, &mut r, cam, 0x140);
        let (ctl, t) = (DATA + 0x3_0000, DATA + 0x3_1000);
        c.bus.write32(cam + 0xa8, ctl);
        c.bus.write32(ctl + 0x20, t);
        let vt = vtable(c, &mut r, &[0x88, 0x18]);
        c.bus.write32(t, vt);
        c.bus.write32(cam + 0x450, r.next_u32() % 5);
        c.bus.write32(cam + 0x458, (seed % 3 != 0) as u32);
        c.bus.write32(cam + 0x45c, (seed / 3 % 2) as u32);
        c.bus.write32(GAME_GP - 0x6a0, (seed % 7 == 0) as u32);
        // Boost thresholds around the fakes' f0 results (random bits).
        for off in [0x6c80, 0x6c78, 0x6c74] {
            c.bus.write32(GAME_GP - off, r.next_u32());
        }
        c.bus.write32(GAME_GP - 0x6c7c, 0.05f32.to_bits());
        let sp = crate::difftest::SP - 0x70;
        put_vec(c, sp, [r.f32(3000.0), r.f32(3000.0), r.f32(500.0), 0]);
        // GameCamera_AddShake: GameModule -> view manager -> state.
        let (module, vm, state) = (DATA + 0x4_0000, DATA + 0x4_0100, DATA + 0x4_0200);
        c.bus.write32(0x4a28a8, module);
        c.bus.write32(module + 0x84, vm);
        c.bus.write32(vm + 0x28, state);
        c.bus.write32(state, r.next_u32() % 12);
        fill(c, &mut r, 0x4c53d0, 0x80);
    }

    #[test]
    fn game_camera_update_shake() {
        let g = ssx3_ee::s01::f_0015e460__generated;
        covered(g, super::game_camera_update_shake, SEEDS, &update_shake_setup);
    }

    driven!(gen_controller_update, ssx3_ee::s01::f_00161bb8__generated);
    driven!(hand_controller_update, super::chase_cam_controller_update);

    /// A controller whose list holds 1..4 nodes with fake algorithms
    /// (Tick = slot 9 fake), and the auto-mode / race-state context.
    /// `expire`: the timed override runs out, which calls the real
    /// `ChaseCamController_SetAlgorithm` factory; its jump table is not in
    /// the test RAM, so those seeds stop at the jump (`stop_at_jump`).
    fn controller_update_setup(c: &mut Cpu, seed: u32, expire: bool) -> Events {
        auto_mode_setup(c, seed);
        let mut r = machine_keep(c, seed, 0xd31f, 0);
        let ctl = c.g[A0][0] as u32;
        // No immediate algorithm switch in the auto mode (factory).
        let flags = c.bus.read32(ctl + 0x30) | 1;
        c.bus.write32(ctl + 0x30, flags);
        c.bus.write32(GAME_GP - 0x6a0, (seed % 11 == 0) as u32);
        let list = DATA + 0xd_0000;
        c.bus.write32(ctl + 8, list);
        let n = 1 + seed / 3 % 4;
        c.bus.write32(list + 4, n);
        let vt = vtable(c, &mut r, &[0x48]);
        let rate = [0.05f32, 1.0, 0.5][(r.next_u32() % 3) as usize];
        for i in 0..n {
            let node = list + 0x100 + 0x40 * i;
            let algo = list + 0x1000 + 0x40 * i;
            if i == 0 {
                c.bus.write32(list, node);
            }
            c.bus.write32(node, algo);
            let w = [0.0f32, 1.0, 0.03, 0.97, 0.5, 2.0, -0.2, -2.0][(r.next_u32() % 8) as usize];
            c.bus.write32(node + 4, w.to_bits());
            c.bus.write32(node + 8, rate.to_bits());
            c.bus.write32(node + 0xc, r.f32(1.0));
            c.bus.write32(node + 0x14, if i + 1 < n { node + 0x40 } else { 0 });
            c.bus.write32(algo + 0xc, if r.next_u32() % 4 == 0 { 0x4a } else { 0x3d });
            c.bus.write32(algo + 0x10, vt);
        }
        // Race state: GameModule -> +0x84 -> +0x28 -> state, +0x484.
        let (module, vm, race) = (DATA + 0xe_0000, DATA + 0xe_0100, DATA + 0xe_1000);
        c.bus.write32(GAME_GP - 0x848, module);
        c.bus.write32(module + 0x84, vm);
        c.bus.write32(vm + 0x28, race);
        let states = [0, 1, 5, 10, 0xffff_ffff];
        c.bus.write32(race, if expire { 5 } else { states[(r.next_u32() % 5) as usize] });
        c.bus.write32(race + 0x484, (r.next_u32() % 2) as u32);
        c.bus.write32(ctl + 0x2c, if expire || seed % 2 == 0 { 0x5d } else { 0x3d });
        let current = c.bus.read32(ctl + 0x18);
        c.bus.write32(ctl + 0x24, if !expire && r.next_u32() % 4 == 0 { current } else { 0x10 });
        let timer = if expire { 12.0f32 } else { [9.0f32, 0.0][(r.next_u32() % 2) as usize] };
        c.bus.write32(ctl + 0x38, timer.to_bits());
        c.bus.write32(GAME_GP - 0x6b24, (1.0f32 / 60.0).to_bits());
        schedule(c, &mut r, seed, 0x0016_1bb8)
    }

    /// A `jump_out` hook that stops the thread (pc outside the code), so
    /// both runs return through their unwind checks.
    fn stop_at_jump(c: &mut Cpu, _pc: u32, _start: u32, _end: u32) {
        c.pc = 0xdead_0000;
    }

    #[test]
    fn chase_cam_controller_update() {
        check_events(gen_controller_update, hand_controller_update, SEEDS, &|c, seed| {
            controller_update_setup(c, seed, false)
        });
        // The override timeout and its SetOverrideAlgorithm call.
        check(ssx3_ee::s01::f_00161bb8__generated, super::chase_cam_controller_update, 200, &|c, seed| {
            controller_update_setup(c, seed * 11 + 1, true);
            c.hooks.jump_out = stop_at_jump;
            c.next_event = u64::MAX;
        });
    }
}
