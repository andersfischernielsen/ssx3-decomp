//! Work package A3 camera (docs/investigation/phase2-candidates.md, layouts
//! in docs/investigation/structs/camera.md).
//!
//! The game camera (`GameCamera`) holds an eye position and an orientation
//! quaternion; every frame the active controller (manual / chase / script)
//! blends the views of its camera algorithms into them, and
//! `CameraBase_UpdateMatrices` turns them into the world-to-camera matrix.
//! This module has that matrix builder, the camera constructor, the shake
//! bookkeeping and a handful of small accessors.
//!
//! Every replacement reproduces the original's observable state exactly: the
//! memory writes (including dead stack scratch), the registers a caller or
//! the differential test can see (v0/v1, f0/f1, the FPU condition bit, the
//! VU0 registers), and the EE cycle charge of each executed block.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_d928, update_matrices, "CameraBase_UpdateMatrices"),
    (0x0015_d698, camera_base_ctor, "CameraBase_ctor"),
    (0x0016_73a0, chase_get_view, "ChaseCameraAlgorithm_GetView"),
    (
        0x0016_2b80,
        chase_set_look_at_from_rider,
        "ChaseCameraAlgorithm_SetLookAtFromRider",
    ),
    (
        0x0016_2138,
        chase_controller_reset_state,
        "ChaseCamController_ResetState",
    ),
    (0x0015_e360, add_shake, "GameCamera_AddShake"),
    (0x0015_dfd8, set_controller, "GameCamera_SetController"),
    (
        0x0015_e2a8,
        init_shake_presets,
        "GameCamera_InitShakePresets",
    ),
    (0x0016_03f0, wobble_angle, "CameraWobble_GetAngle"),
    (
        0x0016_2310,
        rider_algo_set_target,
        "RiderCameraAlgorithm_SetTarget",
    ),
    (
        0x0016_94b0,
        script_algo_get_script_obj,
        "ScriptCameraAlgo_GetScriptObj",
    ),
    (
        0x0016_94a8,
        script_algo_set_script_obj,
        "ScriptCameraAlgo_SetScriptObj",
    ),
    (
        0x0016_c390,
        trigger_man_reset_header,
        "CameraTriggerMan_ResetHeader",
    ),
];

// ---------------------------------------------------------------- globals

/// The world (Z-up) to camera axis permutation, a 4x4 matrix written at boot
/// (`0x1673f8`); the view matrix starts from it.
const AXIS_REMAP: Addr = Addr(0x004c_53a0);
/// Default `GameCamera::forward` (static data).
const DEFAULT_FORWARD: Addr = Addr(0x004f_f140);
/// Shake presets: 5 entries of [`ShakePreset`].
const SHAKE_PRESETS: Addr = Addr(0x004c_53e0);
/// One int per shake preset (0 for preset 0, 1 for the rest).
const SHAKE_PRESET_FLAGS: Addr = Addr(0x004c_5430);
/// `GameModule*` (gp-0x848).
const GAME_MODULE_PTR: Addr = Addr(0x004a_28a8);
/// The twelve float constants the shake presets are initialised from
/// (gp-0x6cb0 .. gp-0x6c84).
const SHAKE_DEFAULTS: Addr = Addr(0x0049_c440);
/// Wobble angle scale (gp-0x6c1c).
const WOBBLE_SCALE: Addr = Addr(0x0049_c4d4);
/// The CameraBase vtable (`GameCamera_ctor` later installs 0x45b888).
const CAMERA_BASE_VTABLE: u32 = 0x0045_b8b0;

// ------------------------------------------------------------ typed views

/// `GameCamera` (0x4b0 bytes; the first 0xc0 are `CameraBase`).
#[derive(Clone, Copy, Debug)]
pub struct GameCamera(pub Addr);

impl GameCamera {
    /// Camera position (vec4).
    pub fn eye(self) -> Addr {
        self.0.off(0x20)
    }
    /// Orientation quaternion (x, y, z, w).
    pub fn rot(self) -> Addr {
        self.0.off(0x30)
    }
    /// World-to-camera matrix: rotation rows +0x40/+0x50/+0x60, translation
    /// row +0x70.
    pub fn view(self) -> Addr {
        self.0.off(0x40)
    }
    /// View direction (vec4).
    pub fn forward(self) -> Addr {
        self.0.off(0x80)
    }
    pub fn vptr(self) -> Addr {
        self.0.off(0x90)
    }
    /// Active `CamController*`.
    pub fn active_controller(self) -> Addr {
        self.0.off(0xa0)
    }
    /// The three controllers, by `GameCamera_SetController` mode 1/2/3.
    pub fn manual_controller(self) -> Addr {
        self.0.off(0xa4)
    }
    pub fn chase_controller(self) -> Addr {
        self.0.off(0xa8)
    }
    pub fn script_controller(self) -> Addr {
        self.0.off(0xac)
    }
    /// Index of the strongest shake requested this frame (i32).
    pub fn shake_preset(self) -> Addr {
        self.0.off(0x450)
    }
    /// Its strength (f32).
    pub fn shake_strength(self) -> Addr {
        self.0.off(0x454)
    }
    /// Set when a shake was requested this frame (i32).
    pub fn shake_requested(self) -> Addr {
        self.0.off(0x458)
    }
}

/// One shake preset (16 bytes) in the table at [`SHAKE_PRESETS`].
#[derive(Clone, Copy, Debug)]
pub struct ShakePreset(pub Addr);

impl ShakePreset {
    /// Preset `i` (unchecked, like the game).
    pub fn nth(i: u32) -> ShakePreset {
        ShakePreset(SHAKE_PRESETS.off(i << 4))
    }
    /// Strength multiplier (f32).
    pub fn amplitude(self) -> Addr {
        self.0.off(0x8)
    }
    /// Distance at which the shake fades out; <= 0 means no fade (f32).
    pub fn range(self) -> Addr {
        self.0.off(0xc)
    }
}

/// `ChaseCameraAlgorithm` (0x390+ bytes).
#[derive(Clone, Copy, Debug)]
pub struct ChaseCameraAlgorithm(pub Addr);

impl ChaseCameraAlgorithm {
    /// Look-at point (vec4).
    pub fn look_at(self) -> Addr {
        self.0.off(0x20)
    }
    /// `RiderCameraTarget*` (`RiderCameraAlgorithm` and subclasses).
    pub fn target(self) -> Addr {
        self.0.off(0x30)
    }
    /// Output eye (vec4) and orientation (quat).
    pub fn out_eye(self) -> Addr {
        self.0.off(0x60)
    }
    pub fn out_rot(self) -> Addr {
        self.0.off(0x70)
    }
}

/// The `RiderCamState` record `ChaseCameraAlgorithm_GatherRiderState` builds.
#[derive(Clone, Copy, Debug)]
pub struct RiderCamState(pub Addr);

impl RiderCamState {
    pub fn rider_position(self) -> Addr {
        self.0.off(0x10)
    }
}

/// `CameraView`, the output of `CameraAlgorithm::GetView`.
#[derive(Clone, Copy, Debug)]
pub struct CameraView(pub Addr);

impl CameraView {
    pub fn eye(self) -> Addr {
        self.0
    }
    pub fn rot(self) -> Addr {
        self.0.off(0x10)
    }
}

/// `ChaseCamController` (0x44 bytes).
#[derive(Clone, Copy, Debug)]
pub struct ChaseCamController(pub Addr);

impl ChaseCamController {
    /// Algorithm id to restore after an override (i32).
    pub fn restore_algorithm(self) -> Addr {
        self.0.off(0x24)
    }
    /// Second copy of an algorithm id (i32, meaning unclear).
    pub fn algorithm_0x28(self) -> Addr {
        self.0.off(0x28)
    }
    /// Flags: bit 0 override active, bit 1 script camera, bit 2 spoke mode.
    pub fn flags(self) -> Addr {
        self.0.off(0x30)
    }
    /// Blend rate of the pending override (f32).
    pub fn override_blend_rate(self) -> Addr {
        self.0.off(0x34)
    }
    /// Spoke mode (i32).
    pub fn spoke_mode(self) -> Addr {
        self.0.off(0x3c)
    }
    /// Zeroed on reset, otherwise unused (i32).
    pub fn unused_0x40(self) -> Addr {
        self.0.off(0x40)
    }
}

/// `ScriptCameraAlgo`.
#[derive(Clone, Copy, Debug)]
pub struct ScriptCameraAlgo(pub Addr);

impl ScriptCameraAlgo {
    /// `ScriptObjCam*`.
    pub fn script_obj(self) -> Addr {
        self.0.off(0x14)
    }
}

/// A camera wobble (field meanings from `CameraWobble_GetAngle` only).
#[derive(Clone, Copy, Debug)]
pub struct CameraWobble(pub Addr);

impl CameraWobble {
    /// Angle the wobble is measured from (f32).
    pub fn base(self) -> Addr {
        self.0.off(0x08)
    }
    /// Current value (f32).
    pub fn current(self) -> Addr {
        self.0.off(0x14)
    }
    /// Amplitude; 0 disables the wobble (f32).
    pub fn amplitude(self) -> Addr {
        self.0.off(0x1c)
    }
    /// Elapsed time and period (f32).
    pub fn time(self) -> Addr {
        self.0.off(0x24)
    }
    pub fn period(self) -> Addr {
        self.0.off(0x2c)
    }
}

/// `CameraTriggerMan` header.
#[derive(Clone, Copy, Debug)]
pub struct CameraTriggerMan(pub Addr);

// ---------------------------------------------------------------- helpers

fn lanes(v: Vec4) -> [VuF32; 4] {
    [v.x, v.y, v.z, v.w]
}

fn vec4(l: [VuF32; 4]) -> Vec4 {
    Vec4 {
        x: l[0],
        y: l[1],
        z: l[2],
        w: l[3],
    }
}

/// Row vector times matrix as the VU0 chain MULA/MADDA/MADDA/MADD:
/// `m0*v.x + m1*v.y + m2*v.z + m3*v.w`, accumulating left to right.
fn vec_times_mat(v: Vec4, m: &[Vec4; 4]) -> Vec4 {
    let (m0, m1, m2, m3) = (lanes(m[0]), lanes(m[1]), lanes(m[2]), lanes(m[3]));
    vec4(std::array::from_fn(|l| {
        let acc = m0[l] * v.x;
        let acc = VuF32::madd(acc, m1[l], v.y);
        let acc = VuF32::madd(acc, m2[l], v.z);
        VuF32::madd(acc, m3[l], v.w)
    }))
}

/// A quaternion as a 3x3 rotation (row-vector convention, w lanes 0), in
/// the VU0 operation order the game uses (shared with
/// `Rider_BuildWorldMatrix`): diagonal terms are (1 - 2a²) - 2b², off
/// diagonals (0 + 2ab) ± 2cw, each MADD/MSUB rounding once.
fn quat_rows(q: Vec4) -> [Vec4; 3] {
    let (one, zero) = (VuF32::ONE, VuF32::ZERO);
    let (x2, y2, z2) = (q.x + q.x, q.y + q.y, q.z + q.z);
    let (sx, sy, sz) = (x2 * q.x, y2 * q.y, z2 * q.z);
    let (xw, yw, zw) = (x2 * q.w, y2 * q.w, z2 * q.w);
    // OPMULA, then MADD by vf0*vf0 (adds zero).
    let yz = VuF32::madd(y2 * q.z, zero, zero);
    let zx = VuF32::madd(z2 * q.x, zero, zero);
    let xy = VuF32::madd(x2 * q.y, zero, zero);
    [
        Vec4 {
            x: VuF32::msub(one - sy, one, sz),
            y: VuF32::madd(zero + xy, one, zw),
            z: VuF32::msub(zero + zx, one, yw),
            w: zero,
        },
        Vec4 {
            x: VuF32::msub(zero + xy, one, zw),
            y: VuF32::msub(one - sz, one, sx),
            z: VuF32::madd(zero + yz, one, xw),
            w: zero,
        },
        Vec4 {
            x: VuF32::madd(zero + zx, one, yw),
            y: VuF32::msub(zero + yz, one, xw),
            z: VuF32::msub(one - sx, one, sy),
            w: zero,
        },
    ]
}

/// VU0's constant register vf0.
const VF0: Vec4 = Vec4 {
    x: VuF32::ZERO,
    y: VuF32::ZERO,
    z: VuF32::ZERO,
    w: VuF32::ONE,
};

fn read_mat(c: &mut Cpu, a: Addr) -> [Vec4; 4] {
    std::array::from_fn(|i| a.off(0x10 * i as u32).read_vec4(c))
}

fn write_mat(c: &mut Cpu, a: Addr, m: &[Vec4; 4]) {
    for (i, row) in m.iter().enumerate() {
        a.off(0x10 * i as u32).write_vec4(c, *row);
    }
}

// ------------------------------------------------------------ replacements

/// `CameraBase_UpdateMatrices(GameCamera*)` (0x15d928): rebuild the
/// world-to-camera matrix from the eye and orientation.
///
/// view = R(rot) · AXIS_REMAP, then the translation row becomes
/// (-eye, 1) · view. The original is VU0 macro code (102 cycles, one block)
/// that passes the rotation rows and the (-eye, 1) vector through a 0x70-byte
/// stack frame; those dead stack bytes, the VU0 scratch registers and the
/// FPU/GPR scratch are reproduced.
pub fn update_matrices(c: &mut Cpu) {
    c.cycles += 102;
    let cam = GameCamera(Addr(abi::arg(c, 0)));
    let frame = Addr((c.g[SP][0] as u32).wrapping_sub(0x70));
    let view = cam.view();

    // Start from the axis permutation.
    let remap = read_mat(c, AXIS_REMAP);
    write_mat(c, view, &remap);

    // Rotation rows (and vf0 as the fourth row) go to the stack frame.
    let q = cam.rot().read_vec4(c);
    let r = quat_rows(q);
    frame.off(0x30).write_vec4(c, VF0);
    for (i, row) in r.iter().enumerate() {
        frame.off(0x10 * i as u32).write_vec4(c, *row);
    }

    // view = R · view, every row through the MULA/MADDA chain.
    let m = read_mat(c, view);
    let rs = read_mat(c, frame);
    let product: [Vec4; 4] = std::array::from_fn(|i| vec_times_mat(rs[i], &m));
    write_mat(c, view, &product);

    // Translation: (-eye, 1) · view replaces row 3. The vector is assembled
    // on the stack by FPU stores (with a copy of eye next to it).
    let e = cam.eye();
    let (ex, ey, ez) = (e.read_fpu(c), e.off(4).read_fpu(c), e.off(8).read_fpu(c));
    let one = FpuF32::ONE;
    frame.off(0x40).write_fpu(c, -ex);
    frame.off(0x44).write_fpu(c, -ey);
    frame.off(0x48).write_fpu(c, -ez);
    frame.off(0x4c).write_fpu(c, one);
    frame.off(0x50).write_fpu(c, -ex);
    frame.off(0x54).write_fpu(c, -ey);
    frame.off(0x58).write_fpu(c, -ez);
    frame.off(0x60).write_fpu(c, ex);
    frame.off(0x64).write_fpu(c, ey);
    frame.off(0x68).write_fpu(c, ez);
    let m = read_mat(c, view);
    let neg_eye = frame.off(0x40).read_vec4(c);
    let translation = vec_times_mat(neg_eye, &m);
    view.off(0x30).write_vec4(c, translation);

    // Scratch state as the original leaves it (its earlier intermediates in
    // vf4..vf8 were overwritten by the reloads).
    let rows = read_mat(c, frame);
    let ones = Vec4 {
        x: VuF32::ONE,
        y: VuF32::ONE,
        z: VuF32::ONE,
        w: remap[0].w,
    };
    vu0_scratch::set_vf(c, 1, 0b1111, ones);
    vu0_scratch::set_vf(c, 2, 0b1111, remap[1]);
    vu0_scratch::set_vf(c, 3, 0b1111, remap[2]);
    for (i, row) in m.iter().enumerate() {
        vu0_scratch::set_vf(c, 4 + i, 0b1111, *row);
    }
    vu0_scratch::set_vf(c, 8, 0b1111, neg_eye);
    // ACC as the final MULA/MADDA/MADDA chain leaves it: the translation
    // before its w term (m0*-ex + m1*-ey + m2*-ez, one rounding per step).
    let lane = |i: usize, v: Vec4| [v.x, v.y, v.z, v.w][i];
    let acc: Vec<VuF32> = (0..4)
        .map(|i| {
            let a = lane(i, m[0]) * neg_eye.x;
            let a = VuF32::madd(a, lane(i, m[1]), neg_eye.y);
            VuF32::madd(a, lane(i, m[2]), neg_eye.z)
        })
        .collect();
    vu0_scratch::set_acc(
        c,
        0b1111,
        Vec4 {
            x: acc[0],
            y: acc[1],
            z: acc[2],
            w: acc[3],
        },
    );
    vu0_scratch::set_vf(c, 9, 0b1111, rows[1]);
    vu0_scratch::set_vf(c, 10, 0b1111, rows[2]);
    vu0_scratch::set_vf(c, 11, 0b1111, rows[3]);
    vu0_scratch::set_vf(c, 12, 0b1111, translation);
    vu0_scratch::set_vf(c, 13, 0b1111, product[1]);
    vu0_scratch::set_vf(c, 14, 0b1111, product[2]);
    vu0_scratch::set_vf(c, 15, 0b1111, product[3]);
    c.f[0] = ex.0;
    c.f[1] = one.0;
    c.f[2] = (-ey).0;
    c.f[3] = ey.0;
    c.f[4] = (-ex).0;
    c.f[5] = ez.0;
    c.f[6] = (-ez).0;
    c.g[V0][0] = sx(AXIS_REMAP.0);
    c.g[V1][0] = sx(view.0);
    c.g[AT][0] = sx(one.0);
    abi::ret(c);
}

/// `CameraBase_ctor(GameCamera*)` (0x15d698): install the CameraBase
/// vtable, eye = 0, rot = (0, 1, 0, 1), forward = the default direction.
/// Returns the camera.
///
/// The (0, 1, 0, 1) quadword is built in a 16-byte stack slot below sp
/// (zeroed first, then the two 1.0 words), which stays behind. 25 cycles.
pub fn camera_base_ctor(c: &mut Cpu) {
    c.cycles += 25;
    let cam = GameCamera(Addr(abi::arg(c, 0)));
    let slot = Addr((c.g[SP][0] as u32).wrapping_sub(0x10));
    let one = FpuF32::ONE;
    for o in [0, 4, 8, 0xc] {
        slot.off(o).write_u32(c, 0);
    }
    let zero = slot.read_vec4(c);
    slot.off(4).write_fpu(c, one);
    slot.off(0xc).write_fpu(c, one);
    slot.write_u32(c, 0);
    slot.off(8).write_u32(c, 0);
    cam.vptr().write_u32(c, CAMERA_BASE_VTABLE);
    let up = slot.read_vec4(c);
    cam.eye().write_vec4(c, zero);
    cam.rot().write_vec4(c, up);
    let fwd = DEFAULT_FORWARD.read_vec4(c);
    cam.forward().write_vec4(c, fwd);

    c.g[AT][0] = sx(one.0);
    c.f[0] = one.0;
    abi::set_gpr128(c, A1, up.to_bits());
    c.g[A2][0] = sx(DEFAULT_FORWARD.0);
    abi::set_gpr128(c, A3, zero.to_bits());
    abi::set_gpr128(c, V1, fwd.to_bits());
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `ChaseCameraAlgorithm_GetView(CameraView *out, algo)` (0x1673a0, vtable
/// slot 2): copy the output eye and orientation. Returns `out`. 6 cycles.
pub fn chase_get_view(c: &mut Cpu) {
    c.cycles += 6;
    let out = CameraView(Addr(abi::arg(c, 0)));
    let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 1)));
    c.g[V0][0] = c.g[A0][0];
    let eye = algo.out_eye().read_vec4(c);
    out.eye().write_vec4(c, eye);
    let rot = algo.out_rot().read_vec4(c);
    out.rot().write_vec4(c, rot);
    abi::set_gpr128(c, A2, eye.to_bits());
    abi::set_gpr128(c, V1, rot.to_bits());
    abi::ret(c);
}

/// `ChaseCameraAlgorithm_SetLookAtFromRider(algo, RiderCamState*)`
/// (0x162b80): look-at point = rider position. 3 cycles.
pub fn chase_set_look_at_from_rider(c: &mut Cpu) {
    c.cycles += 3;
    let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
    let st = RiderCamState(Addr(abi::arg(c, 1)));
    let p = st.rider_position().read_vec4(c);
    algo.look_at().write_vec4(c, p);
    abi::set_gpr128(c, V0, p.to_bits());
    abi::ret(c);
}

/// `ChaseCamController_ResetState(ctl)` (0x162138): restore the saved
/// algorithm id, leave override/script/spoke mode. 14 cycles.
pub fn chase_controller_reset_state(c: &mut Cpu) {
    c.cycles += 14;
    let ctl = ChaseCamController(Addr(abi::arg(c, 0)));
    let flags = ctl.flags().read_u32(c) & !7;
    let restore = ctl.restore_algorithm().read_u32(c);
    ctl.override_blend_rate().write_u32(c, 0);
    ctl.algorithm_0x28().write_u32(c, restore);
    ctl.spoke_mode().write_u32(c, 0);
    ctl.flags().write_u32(c, flags);
    ctl.unused_0x40().write_u32(c, 0);
    c.g[V0][0] = sx(flags);
    c.g[V1][0] = sx(restore);
    c.g[A1][0] = !2;
    c.g[A2][0] = !1;
    c.g[A3][0] = !4;
    abi::ret(c);
}

/// `GameCamera_AddShake(float intensity, float dist, GameCamera*, int
/// preset)` (0x15e360): request a camera shake for this frame; the
/// strongest request wins.
///
/// While the game state is 1..9 (or negative) presets 1..4 are ignored.
/// Otherwise strength = intensity * fade * preset.amplitude, where fade
/// falls from 1 to 0 as `dist` goes from half the preset's range to the
/// full range (1 when dist <= 0 or the preset has no range).
pub fn add_shake(c: &mut Cpu) {
    let intensity = abi::farg(c, 0);
    let dist = abi::farg(c, 1);
    let cam = GameCamera(Addr(abi::arg(c, 0)));
    let preset = abi::arg(c, 1);

    // The game state: *(GameModule->+0x84 (GameViewMan)->+0x28).
    c.cycles += 7;
    c.f[0] = dist.0;
    let module = GAME_MODULE_PTR.read_u32(c);
    let view_man = Addr(module).off(0x84).read_u32(c);
    let state_ptr = Addr(view_man).off(0x28).read_u32(c);
    let state = Addr(state_ptr).read_u32(c) as i32;
    c.g[V1][0] = sx(view_man);
    let busy = state != 0 && {
        c.cycles += 1;
        state < 10
    };
    c.cycles += 2;
    let skip = busy && {
        c.cycles += 3;
        preset.wrapping_sub(1) < 4
    };
    c.g[V0][0] = 1;
    if !skip {
        // Fade factor.
        c.cycles += 10;
        let p = ShakePreset::nth(preset);
        let zero = FpuF32::ZERO;
        let range = p.range().read_fpu(c);
        c.f[4] = zero.0;
        c.f[13] = range.0;
        let mut fade = None;
        if zero.lt(dist) {
            c.cycles += 3;
            if zero.lt(range) {
                c.cycles += 11;
                let two = FpuF32::from_f32(2.0);
                let t = two - (dist + dist).div(range);
                c.f[0] = t.0;
                c.f[1] = two.0;
                c.f[3] = FpuF32::ONE.0;
                // Clamp to [0, 1] by a chain of compares.
                fade = Some(if !t.le(FpuF32::ONE) {
                    FpuF32::ONE
                } else {
                    c.cycles += 4;
                    if !zero.le(t) {
                        zero
                    } else {
                        c.cycles += 4 + 2;
                        t
                    }
                });
            }
        }
        let fade = fade.unwrap_or_else(|| {
            c.cycles += 2;
            FpuF32::ONE
        });

        c.cycles += 10;
        let amp = p.amplitude().read_fpu(c);
        let strength = (intensity * fade) * amp;
        let current = cam.shake_strength().read_fpu(c);
        let wins = current.lt(strength);
        abi::set_fcc(c, wins);
        if wins {
            c.cycles += 2;
            cam.shake_preset().write_u32(c, preset);
            cam.shake_strength().write_fpu(c, strength);
        }
        c.cycles += 1;
        cam.shake_requested().write_u32(c, 1);
        c.f[0] = amp.0;
        c.f[1] = current.0;
        c.f[2] = strength.0;
        c.g[V1][0] = sx(preset << 4);
        c.g[A2][0] = sx(preset << 4);
        c.g[A3][0] = 0x4c_0000;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `GameCamera_SetController(cam, mode)` (0x15dfd8): make the manual (1),
/// chase (2) or script (3) controller active; other modes do nothing.
/// `mode` is compared as a 64-bit register.
pub fn set_controller(c: &mut Cpu) {
    let cam = GameCamera(Addr(abi::arg(c, 0)));
    let mode = abi::reg(c, A1) as i64;
    c.cycles += 3;
    let slot = if mode == 2 {
        c.cycles += 3;
        Some(cam.chase_controller())
    } else {
        c.cycles += 2;
        if mode >= 3 {
            c.cycles += 3 + 2;
            c.g[V0][0] = 3;
            (mode == 3).then(|| cam.script_controller())
        } else {
            c.cycles += 2 + 2;
            c.g[V0][0] = 1;
            (mode == 1).then(|| cam.manual_controller())
        }
    };
    if let Some(slot) = slot {
        let ctl = slot.read_u32(c);
        cam.active_controller().write_u32(c, ctl);
        c.g[V0][0] = sx(ctl);
    }
    abi::ret(c);
}

/// `GameCamera_InitShakePresets()` (0x15e2a8): fill the shake preset table
/// from the constants at [`SHAKE_DEFAULTS`]. Presets 1..3 share their first
/// two words; presets 1..4 get range -1 (no distance fade). 45 cycles.
pub fn init_shake_presets(c: &mut Cpu) {
    c.cycles += 45;
    let d: [u32; 12] = std::array::from_fn(|i| SHAKE_DEFAULTS.off(4 * i as u32).read_u32(c));
    let no_range = FpuF32::from_f32(-1.0).0;
    #[rustfmt::skip]
    let table: [[u32; 4]; 5] = [
        [d[0], d[3], d[4], d[5]],
        [d[1], d[2], d[6], no_range],
        [d[1], d[2], d[7], no_range],
        [d[1], d[2], d[8], no_range],
        [d[9], d[10], d[11], no_range],
    ];
    for (i, entry) in table.iter().enumerate() {
        for (j, w) in entry.iter().enumerate() {
            SHAKE_PRESETS
                .off(0x10 * i as u32 + 4 * j as u32)
                .write_u32(c, *w);
        }
    }
    for (i, flag) in [0u32, 1, 1, 1, 1].iter().enumerate() {
        SHAKE_PRESET_FLAGS.off(4 * i as u32).write_u32(c, *flag);
    }
    // FPU/GPR scratch: f0 = -1.0, f1..f12 hold the constants.
    c.f[0] = no_range;
    let f_of = [
        (3, 0),
        (2, 1),
        (1, 2),
        (6, 3),
        (4, 4),
        (5, 5),
        (7, 6),
        (8, 7),
        (9, 8),
        (10, 9),
        (11, 10),
        (12, 11),
    ];
    for (f, i) in f_of {
        c.f[f] = d[i];
    }
    c.g[AT][0] = sx(no_range);
    c.g[V0][0] = sx(SHAKE_PRESETS.0);
    c.g[V1][0] = 1;
    c.g[A0][0] = sx(SHAKE_PRESET_FLAGS.0);
    c.g[A1][0] = 0x4c_0000;
    c.g[A2][0] = 0x4c_0000;
    abi::ret(c);
}

/// `CameraWobble_GetAngle(const CameraWobble*)` (0x1603f0): 0 when the
/// amplitude is 0, else `amp * ((time / period) * K) * (current - base)`.
pub fn wobble_angle(c: &mut Cpu) {
    let w = CameraWobble(Addr(abi::arg(c, 0)));
    c.cycles += 5;
    let amp = w.amplitude().read_fpu(c);
    c.f[4] = amp.0;
    let off = amp.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, off);
    let angle = if off {
        c.cycles += 2;
        FpuF32::ZERO
    } else {
        c.cycles += 11;
        let k = WOBBLE_SCALE.read_fpu(c);
        let t = w.time().read_fpu(c).div(w.period().read_fpu(c));
        let base = w.base().read_fpu(c);
        let delta = w.current().read_fpu(c) - base;
        c.f[1] = delta.0;
        c.f[2] = base.0;
        c.f[3] = k.0;
        amp * (t * k) * delta
    };
    abi::set_fret(c, angle);
    abi::ret(c);
}

/// `RiderCameraAlgorithm_SetTarget(algo, RiderCameraTarget*)` (0x162310,
/// vtable slot 12). 2 cycles.
pub fn rider_algo_set_target(c: &mut Cpu) {
    c.cycles += 2;
    let algo = ChaseCameraAlgorithm(Addr(abi::arg(c, 0)));
    let target = abi::arg(c, 1);
    algo.target().write_u32(c, target);
    abi::ret(c);
}

/// `ScriptCameraAlgo_GetScriptObj(algo)` (0x1694b0). 2 cycles.
pub fn script_algo_get_script_obj(c: &mut Cpu) {
    c.cycles += 2;
    let algo = ScriptCameraAlgo(Addr(abi::arg(c, 0)));
    let obj = algo.script_obj().read_u32(c);
    abi::set_ret(c, obj);
    abi::ret(c);
}

/// `ScriptCameraAlgo_SetScriptObj(algo, obj)` (0x1694a8). 2 cycles.
pub fn script_algo_set_script_obj(c: &mut Cpu) {
    c.cycles += 2;
    let algo = ScriptCameraAlgo(Addr(abi::arg(c, 0)));
    let obj = abi::arg(c, 1);
    algo.script_obj().write_u32(c, obj);
    abi::ret(c);
}

/// `CameraTriggerMan_ResetHeader(m)` (0x16c390): +0x10 = 0, +0x0c = 7,
/// +0x04 = +0x08 = 0. 6 cycles.
pub fn trigger_man_reset_header(c: &mut Cpu) {
    c.cycles += 6;
    let m = CameraTriggerMan(Addr(abi::arg(c, 0)));
    m.0.off(0x10).write_u32(c, 0);
    m.0.off(0x0c).write_u32(c, 7);
    m.0.off(0x04).write_u32(c, 0);
    m.0.off(0x08).write_u32(c, 0);
    c.g[V0][0] = 7;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use crate::difftest::{check, fill, put_vec, Rng, DATA, SP};
    use ssx3_ee_rt::Cpu;

    const GP: u64 = crate::difftest::GAME_GP as u64;

    /// Random GPRs (except zero/gp/sp/ra), FPRs, FPU condition bit and VU0
    /// registers, so leftovers and missed scratch show up.
    fn scramble(c: &mut Cpu, r: &mut Rng) {
        for reg in 1..28 {
            c.g[reg] = [
                (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            ];
        }
        c.g[28][0] = GP;
        for f in 0..32 {
            c.f[f] = r.next_u32();
        }
        c.fcr31 = r.next_u32() & (1 << 23);
        for reg in 1..32 {
            let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
            c.bus.plat.set_vf(reg, v);
        }
    }

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt)
    }

    #[test]
    fn update_matrices() {
        check(
            ssx3_ee::s01::f_0015d928__generated,
            super::update_matrices,
            400,
            &|c, seed| {
                let mut r = rng(seed, 1);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                fill(c, &mut r, SP - 0x80, 0x20);
                fill(c, &mut r, DATA, 0x40);
                let range = if seed % 4 == 0 { 1e30 } else { 2.0 };
                for i in 0..4 {
                    put_vec(
                        c,
                        0x4c53a0 + 16 * i,
                        [r.f32(range), r.f32(range), r.f32(range), r.f32(range)],
                    );
                }
                let e = if seed % 3 == 0 { 0.0 } else { 1e5 };
                put_vec(c, DATA + 0x20, [r.f32(e), r.f32(e), r.f32(e), r.next_u32()]);
                put_vec(
                    c,
                    DATA + 0x30,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
                );
            },
        );
    }

    #[test]
    fn camera_base_ctor() {
        check(
            ssx3_ee::s01::f_0015d698__generated,
            super::camera_base_ctor,
            200,
            &|c, seed| {
                let mut r = rng(seed, 2);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                fill(c, &mut r, SP - 0x20, 0x10);
                fill(c, &mut r, DATA, 0x30);
                fill(c, &mut r, 0x4ff140, 4);
            },
        );
    }

    #[test]
    fn chase_get_view() {
        check(
            ssx3_ee::s02::f_001673a0__generated,
            super::chase_get_view,
            200,
            &|c, seed| {
                let mut r = rng(seed, 3);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                c.g[5][0] = (DATA + 0x100) as u64;
                fill(c, &mut r, DATA, 0x80);
            },
        );
    }

    #[test]
    fn chase_set_look_at_from_rider() {
        check(
            ssx3_ee::s01::f_00162b80__generated,
            super::chase_set_look_at_from_rider,
            200,
            &|c, seed| {
                let mut r = rng(seed, 4);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                c.g[5][0] = (DATA + 0x100) as u64;
                fill(c, &mut r, DATA, 0x80);
            },
        );
    }

    #[test]
    fn chase_controller_reset_state() {
        check(
            ssx3_ee::s01::f_00162138__generated,
            super::chase_controller_reset_state,
            200,
            &|c, seed| {
                let mut r = rng(seed, 5);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x20);
            },
        );
    }

    #[test]
    fn add_shake() {
        check(
            ssx3_ee::s01::f_0015e360__generated,
            super::add_shake,
            2000,
            &|c, seed| {
                let mut r = rng(seed, 6);
                scramble(c, &mut r);
                let cam = DATA;
                let (module, view_man, state) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                c.g[4][0] = cam as u64;
                fill(c, &mut r, cam + 0x440, 8);
                c.bus.write32(0x4a28a8, module);
                c.bus.write32(module + 0x84, view_man);
                c.bus.write32(view_man + 0x28, state);
                let states = [0, 1, 5, 9, 10, 11, -1, r.next_u32() as i32];
                c.bus.write32(state, states[(seed % 8) as usize] as u32);
                let preset =
                    [-2i64, -1, 0, 1, 2, 3, 4, 5, 6, 0x1_0000_0002][(seed / 8 % 10) as usize];
                c.g[5][0] = preset as u64;
                fill(c, &mut r, 0x4c53d0, 0x40);
                // Ranges: positive (fade), zero/negative (no fade); distances
                // around the fade window, zero or negative.
                for i in 0..8u32 {
                    let a = 0x4c53e0 + 16 * i;
                    c.bus.write32(a + 8, r.f32(4.0));
                    let range = match r.next_u32() % 4 {
                        0 => 0.0f32.to_bits(),
                        1 => r.f32(100.0),
                        _ => r.f32(100.0) & 0x7fff_ffff,
                    };
                    c.bus.write32(a + 0xc, range);
                }
                let range_at = 0x4c53ec_u32.wrapping_add((preset as u32) << 4);
                let range = f32::from_bits(c.bus.read32(range_at)).abs().max(1.0);
                c.f[12] = r.f32(3.0);
                c.f[13] = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.f32(100.0) | 0x8000_0000,
                    _ => (f32::from_bits(r.f32(1.0)).abs() * 1.5 * range).to_bits(),
                };
                c.bus
                    .write32(cam + 0x454, if seed % 5 == 0 { 0 } else { r.f32(4.0) });
            },
        );
    }

    #[test]
    fn set_controller() {
        check(
            ssx3_ee::s01::f_0015dfd8__generated,
            super::set_controller,
            400,
            &|c, seed| {
                let mut r = rng(seed, 7);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                fill(c, &mut r, DATA + 0xa0, 4);
                let modes = [
                    -1i64,
                    0,
                    1,
                    2,
                    3,
                    4,
                    7,
                    0x1_0000_0002,
                    0x1_0000_0001,
                    -0x1_0000_0000 + 3,
                ];
                c.g[5][0] = modes[(seed % 10) as usize] as u64;
            },
        );
    }

    #[test]
    fn init_shake_presets() {
        check(
            ssx3_ee::s01::f_0015e2a8__generated,
            super::init_shake_presets,
            200,
            &|c, seed| {
                let mut r = rng(seed, 8);
                scramble(c, &mut r);
                fill(c, &mut r, 0x49c440, 12);
                fill(c, &mut r, 0x4c53d0, 0x30);
            },
        );
    }

    #[test]
    fn wobble_angle() {
        check(
            ssx3_ee::s01::f_001603f0__generated,
            super::wobble_angle,
            600,
            &|c, seed| {
                let mut r = rng(seed, 9);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x10);
                for o in [0x08, 0x14, 0x24, 0x2c] {
                    c.bus.write32(DATA + o, r.f32(100.0));
                }
                let amp = match seed % 5 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => 0x0000_1234, // denormal: compares equal to 0
                    _ => r.f32(10.0),
                };
                c.bus.write32(DATA + 0x1c, amp);
                if seed % 7 == 0 {
                    c.bus.write32(DATA + 0x2c, 0);
                }
                c.bus.write32(0x49c4d4, r.f32(1.0));
            },
        );
    }

    #[test]
    fn small_accessors() {
        type F = ssx3_ee_rt::funcs::GuestFn;
        let pairs: [(F, F); 4] = [
            (
                ssx3_ee::s01::f_00162310__generated,
                super::rider_algo_set_target,
            ),
            (
                ssx3_ee::s02::f_001694b0__generated,
                super::script_algo_get_script_obj,
            ),
            (
                ssx3_ee::s02::f_001694a8__generated,
                super::script_algo_set_script_obj,
            ),
            (
                ssx3_ee::s02::f_0016c390__generated,
                super::trigger_man_reset_header,
            ),
        ];
        for (k, (g, h)) in pairs.into_iter().enumerate() {
            check(g, h, 200, &|c, seed| {
                let mut r = rng(seed, 10 + k as u64);
                scramble(c, &mut r);
                c.g[4][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x20);
            });
        }
    }
}
