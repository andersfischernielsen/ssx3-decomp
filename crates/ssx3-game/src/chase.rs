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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

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

/// The game's four-lane vector length idiom (VU0 macro code from the
/// vector library): with `v` in vf3,
/// `vadd.x vf6, vf0, vf0w` (1.0), `vmul vf4, vf3, vf3`,
/// `vadda.x ACC, vf4, vf4y`, `vmadda.x ACC, vf6, vf4z`,
/// `vmadd.x vf4, vf6, vf4w`, `vsqrt Q, vf4x`: |v| over x, y, z **and** w.
/// Writes vf4, vf6.x, ACC.x and Q; returns |v|.
pub fn length4(c: &mut Cpu, v: Vec4) -> VuF32 {
    let one = VuF32::ONE; // vf0.x + vf0.w
    vu0_scratch::set_vf(c, 6, X, splat(VuF32::ZERO + one));
    let sq = vmul(v, v);
    let mut acc = Acc::load(c);
    acc.adda(X, sq, splat(sq.y));
    acc.madda(X, splat(one), splat(sq.z));
    let sum = acc.madd(splat(one), splat(sq.w));
    acc.store(c);
    vu0_scratch::set_vf(c, 4, XYZW, masked(sq, sum, X));
    q_sqrt(c, sum.x)
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

/// `lw` through the CPU path (sign-extended).
#[inline]
fn lw(c: &mut Cpu, a: Addr) -> u64 {
    a.lw(c)
}

mod callee {
    use super::{ee, Callee};

    pub const CHASE_CAM_CONTROLLER_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::ChaseCamController_SetAlgorithm };
}

/// Calls the generated code makes through the function table.
mod table_callee {
    pub const CHASE_CAMERA_ALGORITHM_RESET: u32 = 0x0016_6f28;
    pub const MATH_ASIN: u32 = 0x0031_c128;
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
                crate::events::deliver_event(c, HEAD);
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
                    crate::events::deliver_event(c, HEAD);
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

#[cfg(test)]
mod tests {
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine, ptr, vtable, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, check_paths, fill, put_vec, require_paths, Rng, GAME_GP};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::funcs::GuestFn;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 400;

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
        let g = ssx3_ee::s02::f_00168150;
        covered(g, super::check_distance_reset, SEEDS, &distance_reset_setup);
        paths_no_unwind(g, SEEDS, &[34, 79, 92], &distance_reset_setup);
    }

    #[test]
    fn init_from_rider() {
        let g = ssx3_ee::s02::f_00166550;
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
        check(ssx3_ee::s01::f_0015d880, super::camera_base_get_pitch, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd303, 0);
            let cam = ptr(&mut r);
            c.g[A0][0] = cam as u64;
            let q = if seed % 4 == 0 { vec(&mut r, 2.0) } else { [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)] };
            put_vec(c, cam + 0x30, q);
        });
    }

    #[test]
    fn game_camera_reset_default_chase() {
        covered(ssx3_ee::s01::f_0015e030, super::game_camera_reset_default_chase, SEEDS, &|c, seed| {
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
        check_paths(ssx3_ee::s01::f_0015d020, super::is_script_camera_active, SEEDS, &[6, 10, 11], &|c, seed| {
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
        let g = ssx3_ee::s01::f_00162060;
        check(g, super::set_override_algorithm, SEEDS, &set_override_setup);
    }

    driven!(gen_all_views_ready, ssx3_ee::s05::f_0022e920);
    driven!(hand_all_views_ready, super::game_view_man_all_views_ready);
    driven!(gen_update_cameras, ssx3_ee::s05::f_0022e840);
    driven!(hand_update_cameras, super::game_view_man_update_cameras);

    /// A view manager with 0..6 cameras (each a vptr at +0x90 whose slot
    /// +0x18 is a fake, and a ready flag at +0xb4).
    fn views(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let vm = ptr(r);
        let n = [0, 1, 2, 3, 6][(seed / 3 % 5) as usize];
        c.bus.write32(vm + 0x10, n);
        let vt = vtable(c, r, &[0x18]);
        for i in 0..n {
            let cam = ptr(r);
            c.bus.write32(vm + 4 + 4 * i, cam);
            c.bus.write32(cam + 0x90, vt);
            c.bus.write32(cam + 0xb4, if r.next_u32() % 5 == 0 { 0 } else { r.next_u32() | 1 });
        }
        vm
    }

    #[test]
    fn game_view_man_all_views_ready() {
        check_events(gen_all_views_ready, hand_all_views_ready, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xd307, 0);
            let vm = views(c, &mut r, seed);
            c.g[A0][0] = vm as u64;
            schedule(c, &mut r, seed, 0x0022_e920)
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
}
