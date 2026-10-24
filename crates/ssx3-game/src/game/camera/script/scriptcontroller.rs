//! `src/camera/script/scriptcontroller.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::delete_if_flag;
pub(crate) use crate::game::ai::computer::DELETE;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::be::be::gp;
pub(crate) use crate::game::camera::cameraalgorithms::lh;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_2;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0016_8fe8,
        script_cam_controller_ctor,
        "ScriptCamController_ctor",
    ),
    (
        0x0016_90d0,
        script_cam_controller_dtor,
        "ScriptCamController_dtor",
    ),
    (
        0x0016_92d8,
        script_cam_controller_stop_script_camera,
        "ScriptCamController_StopScriptCamera",
    ),
    (
        0x0016_9340,
        script_cam_controller_push_algorithm,
        "ScriptCamController_PushAlgorithm",
    ),
    (
        0x0016_9418,
        script_camera_algo_ctor,
        "ScriptCameraAlgo_ctor",
    ),
    (
        0x0016_94b8,
        manual_script_camera_algo_ctor,
        "ManualScriptCameraAlgo_ctor",
    ),
    (
        0x0016_9518,
        manual_script_camera_algo_reset_for_rider,
        "ManualScriptCameraAlgo_ResetForRider",
    ),
    (
        0x0016_9540,
        manual_script_camera_algo_reset,
        "ManualScriptCameraAlgo_Reset",
    ),
    (
        0x0016_9570,
        manual_script_camera_algo_update,
        "ManualScriptCameraAlgo_Update",
    ),
    (
        0x0016_9828,
        manual_script_camera_get_view,
        "ManualScriptCameraAlgo_GetView",
    ),
    (
        0x0016_9cf8,
        target_script_camera_algo_ctor,
        "TargetScriptCameraAlgo_ctor",
    ),
    (
        0x0016_9d58,
        target_script_camera_algo_reset_for_rider,
        "TargetScriptCameraAlgo_ResetForRider",
    ),
    (
        0x0016_9d80,
        target_script_camera_algo_reset,
        "TargetScriptCameraAlgo_Reset",
    ),
    (
        0x0016_9db0,
        target_script_camera_algo_update,
        "TargetScriptCameraAlgo_Update",
    ),
    (
        0x0016_9f88,
        target_script_camera_get_view,
        "TargetScriptCameraAlgo_GetView",
    ),
    (
        0x0016_a458,
        subject_script_camera_algo_ctor,
        "SubjectScriptCameraAlgo_ctor",
    ),
    (
        0x0016_a4b8,
        subject_script_camera_algo_reset_for_rider,
        "SubjectScriptCameraAlgo_ResetForRider",
    ),
    (
        0x0016_a4e0,
        subject_script_camera_algo_reset,
        "SubjectScriptCameraAlgo_Reset",
    ),
    (
        0x0016_a868,
        subject_script_camera_get_view,
        "SubjectScriptCameraAlgo_GetView",
    ),
    (
        0x0016_ad38,
        algorithmic_script_camera_algo_ctor,
        "AlgorithmicScriptCameraAlgo_ctor",
    ),
    (
        0x0016_b150,
        algorithmic_script_camera_algo_reset,
        "AlgorithmicScriptCameraAlgo_Reset",
    ),
    (
        0x0016_b7b8,
        algorithmic_script_camera_get_view,
        "AlgorithmicScriptCameraAlgo_GetView",
    ),
    (
        0x0016_bef8,
        camera_trigger_list_dtor,
        "CameraTriggerList_dtor",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::Callee;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    use ssx3_ee as ee;

    pub const MATH_SIN_COS: u32 = 0x0031_be50;

    pub const MAT4_TO_POS_QUAT: u32 = 0x0031_b748;

    pub const CAMERA_WOBBLE_GET_OFFSET: u32 = 0x0016_0228;

    pub const CAMERA_WOBBLE_PHASE: u32 = 0x0016_03f0;
}

/// Return addresses of the four `GetView` bodies: the addresses their calls
/// come back to, relative to the function's entry.
pub(crate) mod ret {

    pub const WOBBLE: u32 = 0x6c;

    pub const PHASE: u32 = 0xbc;

    pub const ROT_X: u32 = 0xdc;

    pub const ROT_Z: u32 = 0x228;

    pub const ROT_Y: u32 = 0x368;

    pub const TO_POS_QUAT: u32 = 0x4a4;

    /// The two line-bound functions' call returns.
    pub const LINE_ROT_X: u32 = 0xac;

    pub const LINE_ROT_Z: u32 = 0x1f4;

    pub const LINE_ROT_Y: u32 = 0x33c;
}

/// Fields of the script camera both code paths read.
pub(crate) mod camera {

    /// The script's aim/rotation angles: pitch at +0x40, roll/yaw at +0x44
    /// and +0x48, world position at +0x30.
    pub const POSITION: u32 = 0x30;

    pub const ANGLE_PITCH: u32 = 0x40;

    pub const ANGLE_A: u32 = 0x44;

    pub const ANGLE_B: u32 = 0x48;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

pub(crate) const SCRIPT_CAMERA_ALGO_CTOR: Callee = Callee {
    addr: 0x0016_9418,
    f: ee::s02::f_00169418,
};

pub(crate) const CAMERA_WOBBLE_INIT: u32 = 0x0015_fff0;

/// `AlgorithmicScriptCameraAlgo *AlgorithmicScriptCameraAlgo_ctor(...)`
/// (0x16ad38): construct the `ScriptCameraAlgo` base, initialise the camera
/// wobble at +0x70 and store the default state quad from 0x4ff130.
pub fn algorithmic_script_camera_algo_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        SCRIPT_CAMERA_ALGO_CTOR.call(c, 0x0016_ad4c)?;
        c.g[V1][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx(0x46_0000u32.wrapping_sub(0x38e8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        call_ptr(c, CAMERA_WOBBLE_INIT, 0x0016_ad60)?;
        c.g[V1][0] = 0x50_0000;
        c.g[A1][0] = 0x59;
        c.g[V1][0] = sx(0x50_0000u32.wrapping_sub(0xed0));
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, c.g[V1][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sq(4, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x44), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AlgorithmicScriptCameraAlgo_Reset(AlgorithmicScriptCameraAlgo *this)`
/// (0x16b150): call the vtable slot at `this+0x10 -> +0x18/+0x1c` with
/// `arg = 0`.
pub fn algorithmic_script_camera_algo_reset(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        let vt_holder = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = vt_holder;
        c.g[V1][0] = Addr((vt_holder as u32).wrapping_add(0x18)).lh(c);
        c.g[A2][0] = Addr((vt_holder as u32).wrapping_add(0x1c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, c.g[A2][0] as u32, 0x0016_b170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The identity matrix and the three world axes, in `.bss` at boot.
pub(crate) const IDENTITY_MAT: u32 = 0x004f_f1a0;

pub(crate) const AXIS_X: u32 = 0x004f_f140;

pub(crate) const AXIS_Z: u32 = 0x004f_f150;

pub(crate) const AXIS_Y: u32 = 0x004f_f160;

// ---------------------------------------------------------------------------
// Small helpers.

/// A 16-byte aligned quadword copy `lq`/`sq` through GPR 3.
#[inline]
pub(crate) fn quad_copy(c: &mut Cpu, from: u32, to: u32) {
    c.lq(3, from);
    c.sq(3, to);
}

/// `m = *(m0)` as the original copies the identity: four `lqc2`/`sqc2`
/// pairs through vf1..vf4.
pub(crate) fn copy_matrix(c: &mut Cpu, dst: u32, src: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, src + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, dst + 0x10 * i);
    }
}

/// `out = m * v`, the VU0 "sum of scaled columns" product: vf4..vf7 are the
/// matrix columns, vf8 the vector, and the result lands in vf12 (left in the
/// accumulator as the original leaves it).
pub(crate) fn mat_vec(c: &mut Cpu, m: u32, v: u32, out: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, out);
}

/// `m = m * r` for two column-major 4x4 matrices, in place, as the original
/// emits it: four independent `MULA/MADDA/MADDA/MADD` chains into
/// vf12..vf15.
pub(crate) fn mat_mul(c: &mut Cpu, m: u32, r: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, r + 0x10 * i);
    }
    for j in 0..4u32 {
        let s = (8 + j) as u8;
        let d = (12 + j) as u8;
        c.vu0().mula(0b1111, 4, Src::Bc(s, X));
        c.vu0().madda(0b1111, 5, Src::Bc(s, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(s, Z));
        c.vu0().madd(0b1111, d, 7, Src::Bc(s, W));
    }
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, m + 0x10 * i);
    }
}

/// One `ctmRotAxis` step: `sincos(angle)` into `sin`/`cos`, build the
/// axis-angle rotation matrix at `r`, then `m = m * r`.
///
/// The matrix is the classic `a = 1 - cos` form, but with the products in
/// the original's order (`(a*z)*x` and `(a*x)*z` stay separate expressions
/// because they round differently):
///
/// ```text
/// [ a*xx+c   a*xy-sz  a*xz+sy ]
/// [ a*xy+sz  a*yy+c   a*yz-sx ]
/// [ a*xz-sy  a*yz+sx  a*zz+c  ]
/// ```
///
/// The 4th column is `(0, 0, 0, 1)`. Leaves `f0` and `f1` as the original's
/// last two results (`f0 = a*x*y - s*z`, `f1 = a*y*y + cos`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn rot_axis(
    c: &mut Cpu,
    m: u32,
    r: u32,
    axis: u32,
    angle: FpuF32,
    sin: u32,
    cos: u32,
    ret: u32,
) -> Flow {
    c.f[12] = angle.0;
    c.g[A0][0] = sx(sin);
    c.g[A1][0] = sx(cos);
    call_ptr(c, callee::MATH_SIN_COS, ret)?;
    let cos_v = Addr(cos).read_fpu(c);
    let s = Addr(sin).read_fpu(c);
    let x = Addr(axis).read_fpu(c);
    let y = Addr(axis + 4).read_fpu(c);
    let z = Addr(axis + 8).read_fpu(c);
    let a = FpuF32::ONE - cos_v;

    let az = a * z;
    let ax = a * x;
    let ay = a * y;
    let sz = s * z;
    let sx_ = s * x;
    let azz = az * z;
    let axz = ax * z;
    let azx = az * x;
    let axx = ax * x;
    let ayz = ay * z;
    let axy = ay * x;
    let sy = s * y;
    let azy = az * y;
    let axy2 = ax * y;
    let ayy = ay * y;

    let m00 = axx + cos_v;
    let m01 = axy - sz;
    let m02 = azx + sy;
    let m10 = axy2 + sz;
    let m11 = ayy + cos_v;
    let m12 = azy - sx_;
    let m20 = axz - sy;
    let m21 = ayz + sx_;
    let m22 = azz + cos_v;
    let one = FpuF32::ONE;

    Addr(r + 0x00).write_fpu(c, m00);
    Addr(r + 0x04).write_fpu(c, m01);
    Addr(r + 0x08).write_fpu(c, m02);
    Addr(r + 0x0c).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x10).write_fpu(c, m10);
    Addr(r + 0x14).write_fpu(c, m11);
    Addr(r + 0x18).write_fpu(c, m12);
    Addr(r + 0x1c).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x20).write_fpu(c, m20);
    Addr(r + 0x24).write_fpu(c, m21);
    Addr(r + 0x28).write_fpu(c, m22);
    Addr(r + 0x2c).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x30).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x34).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x38).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x3c).write_fpu(c, one);

    // The last two values the original leaves in the FPU file.
    c.f[0] = m01.0;
    c.f[1] = m11.0;
    mat_mul(c, m, r);
    Ok(())
}

// ---------------------------------------------------------------------------
// Script camera views.

/// Shared body of the four `*ScriptCameraAlgo_GetView` functions: `out` is
/// the object whose view (position at +0, quaternion at +0x10) is written,
/// `cam` the script camera, and `wobble` the offset of its wobble sub-object.
pub(crate) fn script_camera_get_view(
    c: &mut Cpu,
    out: u64,
    cam: u32,
    wobble: u32,
    base: u32,
) -> Flow {
    c.cycles += 308;
    let frame = Frame::open(c, 0x150);
    frame.save(c, S0, 0x130);
    frame.save(c, S1, 0x120);
    frame.save(c, S2, 0x110);
    frame.save(c, S3, 0x100);
    frame.save(c, S4, 0xf0);
    frame.save(c, S5, 0xe0);
    frame.save_ra(c, 0xd0);
    frame.save_fpr(c, 20, 0x140);

    let m = sp(c); // the matrix under construction, sp+0..sp+0x40
    copy_matrix(c, m, IDENTITY_MAT);

    // Registers at the calls: the original keeps the camera in s2, the result
    // object in s5 and the wobble sub-object in s0. Callees spill the
    // callee-saved registers into their own frames, so these have to hold the
    // original's values (the rest stay as the caller left them).
    c.g[S2][0] = c.g[A1][0];
    c.g[S5][0] = c.g[A0][0];
    c.g[S0][0] = sx(wobble);

    // Translate by (position + wobble offset): the wobble comes back as a
    // vector at sp+0x50, which the VU0 adds lane-wise to the position and
    // then pushes through the matrix as its new 4th column.
    c.f[13] = Addr(cam + camera::ANGLE_B).read_fpu(c).0;
    c.f[12] = Addr(cam + camera::ANGLE_A).read_fpu(c).0;
    c.g[A0][0] = sx(m + 0x50);
    c.g[A1][0] = sx(wobble);
    call_ptr(c, callee::CAMERA_WOBBLE_GET_OFFSET, base + ret::WOBBLE)?;
    c.lqc2(3, cam + camera::POSITION);
    c.lqc2(4, m + 0x50);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, m + 0x60);
    quad_copy(c, m + 0x60, m + 0x40);
    mat_vec(c, m, m + 0x40, m + 0x30);

    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.mtc1(1, 20);
    c.g[A0][0] = sx(wobble);
    call_ptr(c, callee::CAMERA_WOBBLE_PHASE, base + ret::PHASE)?;

    c.g[S1][0] = 0x0050_0000;
    c.g[S3][0] = sx(m);

    let phase = FpuF32(c.f[0]);
    let pitch = Addr(cam + camera::ANGLE_PITCH).read_fpu(c) + phase;
    c.g[S0][0] = sx(AXIS_X);
    rot_axis(
        c,
        m,
        m + 0x70,
        AXIS_X,
        pitch,
        m + 0xb0,
        m + 0xb4,
        base + ret::ROT_X,
    )?;
    let roll = -Addr(cam + camera::ANGLE_B).read_fpu(c);
    c.g[S0][0] = 0x0050_0000;
    c.g[S1][0] = sx(AXIS_Z);
    c.g[S3][0] = sx(m);
    rot_axis(
        c,
        m,
        m + 0x40,
        AXIS_Z,
        roll,
        m + 0xb8,
        m + 0xbc,
        base + ret::ROT_Z,
    )?;
    let yaw = Addr(cam + camera::ANGLE_A).read_fpu(c);
    c.g[S0][0] = sx(m);
    c.g[S4][0] = sx(m + 0x40);
    rot_axis(
        c,
        m,
        m + 0x40,
        AXIS_Y,
        yaw,
        m + 0xc0,
        m + 0xc4,
        base + ret::ROT_Y,
    )?;

    // Incidental registers the original leaves in the 3rd rotation step.
    c.g[V0][0] = sx(0x0050_0000);
    c.g[V1][0] = sx(AXIS_Y);

    c.g[A0][0] = out;
    c.g[A1][0] = sx(m);
    call_ptr(c, callee::MAT4_TO_POS_QUAT, base + ret::TO_POS_QUAT)?;
    c.g[V0][0] = out;

    frame.restore(c, S0, 0x130);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S2, 0x110);
    frame.restore(c, S3, 0x100);
    frame.restore(c, S4, 0xf0);
    frame.restore(c, S5, 0xe0);
    frame.restore_ra(c, 0xd0);
    frame.restore_fpr(c, 20, 0x140);
    frame.close(c);
    Ok(())
}

/// `void ManualScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)` (0x169828).
pub fn manual_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x74), 0x0016_9828)
    })
}

/// `void TargetScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)` (0x169f88).
pub fn target_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x74), 0x0016_9f88)
    })
}

/// `void SubjectScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)` (0x16a868).
pub fn subject_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x70), 0x0016_a868)
    })
}

/// `void AlgorithmicScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)`
/// (0x16b7b8).
pub fn algorithmic_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x70), 0x0016_b7b8)
    })
}

/// `void CameraTriggerList_dtor(CameraTriggerList* l, int flags)` (0x16bef8).
pub fn camera_trigger_list_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0016_bf10))
}

/// `void ScriptCamController_PushAlgorithm(ScriptCamController* c, void* algo)`
/// (0x169340): push `algo` onto the list at `c->+8` with weight 1.0f.
pub fn script_cam_controller_push_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x8));
        call_ptr(c, 0x0015_c988, 0x0016_9358)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script camera algorithms: the reset hook forwards to `self->v05()`.

/// `void ManualScriptCameraAlgo_ResetForRider(ScriptCamAlgo* self)`
/// (0x169518), `Target...` (0x169d58) and `Subject...` (0x16a4b8): call the
/// algorithm's `v05` virtual method with the object.
pub(crate) fn script_camera_algo_reset_for_rider(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let obj = arg(c, 0);
    let vptr = Addr(obj.wrapping_add(0x10)).lw(c);
    c.g[V0][0] = vptr;
    let delta = Addr((vptr as u32).wrapping_add(0x28)).lh(c);
    c.g[V1][0] = delta;
    let method = Addr((vptr as u32).wrapping_add(0x2c)).lw(c);
    c.g[A1][0] = method;
    c.g[A0][0] = sx(obj.wrapping_add(delta as u32));
    call_ptr(c, method as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The manual script camera algorithm's `ResetForRider`.
pub fn manual_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| script_camera_algo_reset_for_rider(c, 0x0016_9534))
}

/// The target script camera algorithm's `ResetForRider`.
pub fn target_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| script_camera_algo_reset_for_rider(c, 0x0016_9d74))
}

/// The subject script camera algorithm's `ResetForRider`.
pub fn subject_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| script_camera_algo_reset_for_rider(c, 0x0016_a4d4))
}

/// One of the scripted camera algorithms' `Reset` (0x169540, 0x169d80,
/// 0x16a4e0): call the algorithm's `v03` (slot at `+0x18`, method at `+0x1c`
/// of the vtable at `self->+0x10`) with `a1 = 0`.
pub(crate) fn script_camera_algo_reset(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 0x10)).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(vt, 0x18)).lh(c);
        c.g[A2][0] = Addr(at(vt, 0x1c)).lw(c);
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ManualScriptCameraAlgo_Reset(ManualScriptCameraAlgo* self)`
/// (0x169540).
pub fn manual_script_camera_algo_reset(c: &mut Cpu) {
    script_camera_algo_reset(c, 0x0016_9560);
}

/// `void TargetScriptCameraAlgo_Reset(TargetScriptCameraAlgo* self)`
/// (0x169d80).
pub fn target_script_camera_algo_reset(c: &mut Cpu) {
    script_camera_algo_reset(c, 0x0016_9da0);
}

/// `void SubjectScriptCameraAlgo_Reset(SubjectScriptCameraAlgo* self)`
/// (0x16a4e0).
pub fn subject_script_camera_algo_reset(c: &mut Cpu) {
    script_camera_algo_reset(c, 0x0016_a500);
}

/// `void ManualScriptCameraAlgo_ctor(ManualScriptCameraAlgo* self)` (0x1694b8).
pub fn manual_script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0016_9418, 0x0016_94cc)?;
        c.g[V1][0] = 0x45_c850;
        c.g[A0][0] = sx(at_2(c, S0, 0x74));
        c.g[V1][0] = sx(0x0045_c850);
        Addr(at_2(c, S0, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0015_fff0, 0x0016_94e0)?;
        c.g[V1][0] = 0x4f_f130;
        c.g[A1][0] = 0x56;
        c.g[V1][0] = sx(0x004f_f130);
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, r32_2(c, V1));
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[A1][0]);
        c.sq(4, at_2(c, S0, 0x30));
        Addr(at_2(c, S0, 0x44)).sw(c, 0);
        Addr(at_2(c, S0, 0x48)).sw(c, 0);
        Addr(at_2(c, S0, 0x40)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TargetScriptCameraAlgo_ctor(TargetScriptCameraAlgo* self)` (0x169cf8).
pub fn target_script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0016_9418, 0x0016_9d0c)?;
        c.g[V1][0] = 0x45_c7e8;
        c.g[A0][0] = sx(at_2(c, S0, 0x74));
        c.g[V1][0] = sx(0x0045_c7e8);
        Addr(at_2(c, S0, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0015_fff0, 0x0016_9d20)?;
        c.g[V1][0] = 0x4f_f130;
        c.g[A1][0] = 0x57;
        c.g[V1][0] = sx(0x004f_f130);
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, r32_2(c, V1));
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[A1][0]);
        c.sq(4, at_2(c, S0, 0x30));
        Addr(at_2(c, S0, 0x40)).sw(c, 0);
        Addr(at_2(c, S0, 0x44)).sw(c, 0);
        Addr(at_2(c, S0, 0x48)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SubjectScriptCameraAlgo_ctor(SubjectScriptCameraAlgo* self)` (0x16a458).
pub fn subject_script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0016_9418, 0x0016_a46c)?;
        c.g[V1][0] = 0x45_c780;
        c.g[A0][0] = sx(at_2(c, S0, 0x70));
        c.g[V1][0] = sx(0x0045_c780);
        Addr(at_2(c, S0, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0015_fff0, 0x0016_a480)?;
        c.g[V1][0] = 0x4f_f130;
        c.g[A1][0] = 0x58;
        c.g[V1][0] = sx(0x004f_f130);
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, r32_2(c, V1));
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[A1][0]);
        c.sq(4, at_2(c, S0, 0x30));
        Addr(at_2(c, S0, 0x40)).sw(c, 0);
        Addr(at_2(c, S0, 0x44)).sw(c, 0);
        Addr(at_2(c, S0, 0x48)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCamController_StopScriptCamera(ctrl, algo)` (0x1692d8): walk
/// the algorithm list for the entry with `algo->key` and remove it; clear the
/// active flag when the list empties.
pub fn script_cam_controller_stop_script_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        let inactive = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[A1][0];
        if !inactive {
            c.cycles += 2;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
            loop {
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    break;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_9300);
                    }
                    continue;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0016_9320);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0015_ca50, 0x0016_9320)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                break;
            }
            c.cycles += 2;
            if (c.g[V0][0] as i64) <= 0 {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script camera algorithm.

/// `ScriptCameraAlgo* ScriptCameraAlgo_ctor(ScriptCameraAlgo* self)`
/// (0x169418): default FOV, camera constants and the rider's camera control
/// index.
pub fn script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4220));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V0][0]);
        callee::CAMERA_ALGORITHM_DEFAULT_FOV.call(c, 0x0016_9438)?;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 1);
        c.g[V1][0] = 0x55;
        c.lwc1(2, gp(c, -0x6548));
        c.g[V0][0] = 0x0046_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x3748));
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x0050_0000;
        c.swc1(0, r32(c, S0));
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0xed0));
        c.swc1(1, p(r32(c, S0), 4));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(2, p(r32(c, S0), 8));
        Addr(p(r32(c, S0), 0x14)).sw(c, 0);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[RA][0] = Addr(r32(c, SP)).ld(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x84)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, A0), 0x84)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0xac)).lw(c);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[A0][0]);
        c.lq(3, r32(c, A2));
        c.sq(3, p(r32(c, S0), 0x20));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCamController_Update(self)` (0x169368): blend every script
/// camera's weight towards 1.0 and update its algorithm.
pub fn script_cam_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.swc1(21, p(c.g[SP][0] as u32, 0x28));
        c.swc1(20, p(c.g[SP][0] as u32, 0x20));
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            c.g[AT][0] = 0x4040_0000;
            c.mtc1(1, 21);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            loop {
                c.cycles += 12;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, 0x0016_94b0, 0x0016_93a8)?;
                c.lwc1(2, p(c.g[V0][0] as u32, 0x18));
                c.mul_s(1, 2, 2);
                c.add_s(0, 1, 1);
                c.mul_s(1, 1, 21);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 1, 0);
                c.c_lt_s(20, 1);
                let above = !c.fcc();
                c.swc1(2, p(c.g[S0][0] as u32, 4));
                if above {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                }
                c.cycles += 10;
                c.swc1(1, p(c.g[S0][0] as u32, 0xc));
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x10)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x28)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x2c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0016_93f4)?;
                c.g[S0][0] = Addr(p(c.g[S0][0] as u32, 0x14)).lw(c);
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0016_93a0);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ScriptCamController_dtor` (0x001690d0).
//
// `void ScriptCamController::~ScriptCamController()`: install the class
// vtable, release and `delete` the two script elements, free the element
// array, then run the base destructor.

pub fn script_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = 0x460000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3bd8));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        'l_169108: loop {
            'b_169140: {
                c.cycles += 9;
                c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[RA][0] = sx(0x00169118);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x002832d8, 0x00169118)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S0][0] = c.lw((c.g[S0][0] as u32));
                let t = c.g[S0][0] == 0u64;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_169140;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00169140);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00169140)?;
                    break 'b_169140;
                }
            }
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x00169108);
                }
                continue 'l_169108;
            } else {
                'b_169168: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    if c.g[A0][0] == 0u64 {
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_169168;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x00169164);
                        call_ptr(c, 0x00317e98, 0x00169164)?;
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_169168;
                    }
                }
                c.cycles += 9;
                c.g[RA][0] = sx(0x00169170);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0015cc10, 0x00169170)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t;
                return Ok(());
            }
        }

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// The two-instruction virtual-method read the originals use:
/// `a0 = lh(vt+off)`, `func = lw(vt+off+4)`. The original leaves `func` in
/// `vreg` (v0, v1, a2 ... depending on the site), which later code and the
/// fakes can see, so the register is written here too. Returns `func`.
#[inline(always)]
pub(crate) fn vslot(c: &mut Cpu, vt: u64, off: u32, vreg: usize) -> u64 {
    let base = (vt as u32).wrapping_add(off);
    c.g[A0][0] = Addr(base).lh(c);
    let func = Addr(base.wrapping_add(4)).lw(c);
    c.g[vreg][0] = func;
    func
}

// ---------------------------------------------------------------------------
// Script camera.

/// `ScriptCamController* ScriptCamController_ctor(self, info, script)`
/// (0x168fe8): construct the camera controller and allocate its two script
/// object slots (`+0x1c`), each a script camera bound to the script handle.
pub fn script_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S5][0] = 0x460000;
        call_ptr(c, 0x0015_cba0, 0x0016_9020)?;

        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 3;
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x3bd8));
        c.g[A1][0] = 0x460000;
        c.sw(Addr(r(c, S2)).off(0x14).0, c.g[V0][0]);
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x4178));
        c.sw(r(c, S2), c.g[V1][0]);
        c.g[A0][0] = 8;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0016_904c)?;
        c.sw(Addr(r(c, S2)).off(0x1c).0, c.g[V0][0]);
        c.g[S1][0] = Addr(r(c, S2)).off(0x1c).lw(c);

        loop {
            c.cycles += 19;
            c.g[S0][0] = sx(r(c, S3) << 2);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[S1][0] = sx(p(r(c, S1), r(c, S0)));
            c.g[A0][0] = 0x80;
            c.g[A1][0] = sx(r(c, S5).wrapping_sub(0x4160));
            c.g[S3][0] = sx(p(r(c, S3), 0x1));
            call_ptr(c, 0x0031_7d70, 0x0016_9078)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x0027_d5e8, 0x0016_9084)?;
            c.sw(r(c, S1), c.g[V0][0]);
            c.g[V0][0] = Addr(r(c, S2)).off(0x1c).lw(c);
            c.g[S0][0] = sx(p(r(c, S0), r(c, V0)));
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            call_ptr(c, 0x0028_3298, 0x0016_9098)?;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
            if r(c, V0) == 0 {
                break;
            }
            c.g[S1][0] = Addr(r(c, S2)).off(0x1c).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0016_9058);
            }
        }

        c.cycles += 11;
        c.g[V0][0] = c.g[S2][0];
        c.sw(Addr(r(c, S2)).off(0x18).0, 0);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00169DB0(void* p)` (0x169db0): update a target-script camera
/// algorithm: take the target camera's position and orientation, convert
/// degrees to radians, add the rig's yaw/pitch offsets, build the position
/// from the orbit matrix, aim at_3 the target point and apply the wobble
/// parameters.
pub fn target_script_camera_algo_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    c.cycles += 117;
    frame.save(c, S0, 0xa0);
    frame.save(c, S1, 0x90);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save_ra(c, 0x50);
    c.g[S4][0] = sx(at_3(c, S0, 0x74));
    frame.save(c, S2, 0x80);
    c.g[V1][0] = lw_2(c, at_3(c, S0, 0x14));
    c.g[V0][0] = lw_2(c, at_3(c, V1, 0x14));
    c.g[S2][0] = lw_2(c, at_3(c, V0, 0x8));
    c.g[RA][0] = sx(0x00169de8);
    c.g[A0][0] = c.g[S2][0];
    if call_ptr(c, 0x0027c2a8, 0x00169de8).is_err() {
        return;
    }
    c.g[S1][0] = c.g[V0][0];
    c.g[S3][0] = lw_2(c, at_3(c, S0, 0x14));
    c.lwc1(1, r32(c, S1));
    c.g[A0][0] = c.g[S2][0];
    c.g[S3][0] = sx(at_3(c, S3, 0x30));
    c.g[A1][0] = sx(at_3(c, S0, 0x4c));
    c.swc1(1, at_3(c, S0, 0x4));
    c.lwc1(0, at_3(c, S1, 0x4));
    c.g[RA][0] = sx(0x00169e10);
    c.swc1(0, at_3(c, S0, 0x8));
    if call_ptr(c, 0x0027c300, 0x00169e10).is_err() {
        return;
    }
    c.lwc1(1, r32(c, S1));
    c.g[A0][0] = sx(at_3(c, SP, 0x40));
    c.lwc1(0, at_3(c, S0, 0x68));
    c.g[A1][0] = sx(at_3(c, SP, 0x44));
    c.lwc1(4, r32(c, GP).wrapping_sub(0x6520));
    c.swc1(1, at_3(c, S0, 0x4));
    c.mul_s(0, 0, 4);
    c.lwc1(2, at_3(c, S0, 0x60));
    c.lwc1(1, at_3(c, S1, 0x4));
    c.g[V0][0] = lw_2(c, at_3(c, S0, 0x14));
    c.mul_s(2, 2, 4);
    c.swc1(1, at_3(c, S0, 0x8));
    c.swc1(0, r32(c, S0));
    c.lwc1(1, at_3(c, S0, 0x5c));
    c.lwc1(0, at_3(c, V0, 0x78));
    c.mul_s(1, 1, 4);
    c.lwc1(3, at_3(c, S0, 0x64));
    c.add_s(2, 2, 0);
    c.lwc1(5, at_3(c, S0, 0x4c));
    c.mul_s(3, 3, 4);
    c.lwc1(6, at_3(c, S0, 0x50));
    c.lwc1(7, at_3(c, S0, 0x54));
    c.swc1(2, at_3(c, S0, 0x44));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 4);
    c.lwc1(0, at_3(c, V0, 0x74));
    c.add_s(1, 1, 0);
    c.swc1(1, at_3(c, S0, 0x48));
    c.lwc1(0, at_3(c, V0, 0x70));
    c.swc1(5, at_3(c, SP, 0x10));
    c.add_s(3, 3, 0);
    c.swc1(6, at_3(c, SP, 0x14));
    c.swc1(7, at_3(c, SP, 0x18));
    c.swc1(4, at_3(c, SP, 0x1c));
    c.swc1(3, at_3(c, S0, 0x40));
    c.lqc2(8, at_3(c, SP, 0x10));
    c.lqc2(4, r32(c, S3));
    c.lqc2(5, at_3(c, S3, 0x10));
    c.lqc2(6, at_3(c, S3, 0x20));
    c.lqc2(7, at_3(c, S3, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, at_3(c, SP, 0x20));
    lq(c, 2, at_3(c, SP, 0x20));
    c.lwc1(12, at_3(c, S0, 0x44));
    sq(c, 2, at_3(c, S0, 0x20));
    c.g[RA][0] = sx(0x00169ed8);
    sq(c, 2, r32(c, SP));
    if call_ptr(c, 0x0031be50, 0x00169ed8).is_err() {
        return;
    }
    c.lwc1(12, at_3(c, S0, 0x48));
    c.g[A0][0] = sx(at_3(c, SP, 0x48));
    c.g[RA][0] = sx(0x00169ee8);
    c.g[A1][0] = sx(at_3(c, SP, 0x4c));
    if call_ptr(c, 0x0031be50, 0x00169ee8).is_err() {
        return;
    }
    c.lwc1(1, at_3(c, SP, 0x4c));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(0, at_3(c, SP, 0x44));
    c.lwc1(2, at_3(c, SP, 0x40));
    c.mul_s(0, 1, 0);
    c.lwc1(3, at_3(c, SP, 0x48));
    c.mul_s(1, 1, 2);
    sw_2(c, at_3(c, SP, 0xc), 0u64);
    c.swc1(3, at_3(c, SP, 0x8));
    c.swc1(0, r32(c, SP));
    c.swc1(1, at_3(c, SP, 0x4));
    c.lwc1(0, at_3(c, S0, 0x58));
    c.mfc1(3, 0);
    c.lqc2(4, r32(c, SP));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_3(c, SP, 0x30));
    lq(c, 2, at_3(c, SP, 0x30));
    sq(c, 2, at_3(c, SP, 0x20));
    c.lqc2(3, at_3(c, S0, 0x20));
    c.lqc2(4, at_3(c, SP, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_3(c, SP, 0x30));
    lq(c, 2, at_3(c, SP, 0x30));
    c.lwc1(12, at_3(c, S0, 0x6c));
    sq(c, 2, at_3(c, S0, 0x30));
    c.g[RA][0] = sx(0x00169f58);
    sq(c, 2, at_3(c, SP, 0x10));
    if call_ptr(c, 0x00160028, 0x00169f58).is_err() {
        return;
    }
    c.lwc1(12, at_3(c, S0, 0x70));
    c.g[RA][0] = sx(0x00169f64);
    c.g[A0][0] = c.g[S4][0];
    if call_ptr(c, 0x00160130, 0x00169f64).is_err() {
        return;
    }
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    c.g[RA][0] = ld(c, at_3(c, SP, 0x50));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `void func_00169570(void* p)` (0x169570): update a manual-script camera
/// algorithm: take the camera's offset and the rig matrix, place the target,
/// compute the eye position from the distance/angles and derive the yaw and
/// pitch from the view vector, then apply the wobble parameters.
pub fn manual_script_camera_algo_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_169744: {
        'b_16973c: {
            'b_169700: {
                c.cycles += 89;
                frame.save(c, S1, 0x50);
                frame.save(c, S2, 0x40);
                c.swc1(22, at_3(c, SP, 0x80));
                c.g[S2][0] = c.g[A0][0];
                c.swc1(21, at_3(c, SP, 0x78));
                c.swc1(20, at_3(c, SP, 0x70));
                frame.save(c, S0, 0x60);
                frame.save_ra(c, 0x30);
                c.mtc1(0, 22);
                c.g[V0][0] = lw_2(c, at_3(c, S2, 0x14));
                c.g[V1][0] = lw_2(c, at_3(c, V0, 0x14));
                c.g[S0][0] = lw_2(c, at_3(c, V1, 0x8));
                c.g[RA][0] = sx(0x001695ac);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x0027c098, 0x001695ac).is_err() {
                    return;
                }
                c.lwc1(1, r32(c, V0));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = lw_2(c, at_3(c, S2, 0x14));
                c.g[A1][0] = sx(at_3(c, S2, 0x4c));
                c.swc1(1, at_3(c, S2, 0x4));
                c.g[S1][0] = sx(at_3(c, S1, 0x30));
                c.lwc1(0, at_3(c, V0, 0x4));
                c.g[RA][0] = sx(0x001695d0);
                c.swc1(0, at_3(c, S2, 0x8));
                if call_ptr(c, 0x0027c0f0, 0x001695d0).is_err() {
                    return;
                }
                c.lwc1(0, at_3(c, S2, 0x68));
                c.lwc1(4, r32(c, GP).wrapping_sub(0x6544));
                c.lwc1(1, at_3(c, S2, 0x4c));
                c.lwc1(2, at_3(c, S2, 0x50));
                c.mul_s(0, 0, 4);
                c.lwc1(3, at_3(c, S2, 0x54));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 5);
                c.swc1(1, r32(c, SP));
                c.swc1(2, at_3(c, SP, 0x4));
                c.swc1(3, at_3(c, SP, 0x8));
                c.swc1(5, at_3(c, SP, 0xc));
                c.swc1(0, r32(c, S2));
                lq(c, 2, r32(c, SP));
                sq(c, 2, at_3(c, S2, 0x30));
                c.lqc2(8, at_3(c, S2, 0x30));
                c.lqc2(4, r32(c, S1));
                c.lqc2(5, at_3(c, S1, 0x10));
                c.lqc2(6, at_3(c, S1, 0x20));
                c.lqc2(7, at_3(c, S1, 0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, at_3(c, SP, 0x10));
                lq(c, 2, at_3(c, SP, 0x10));
                c.lwc1(0, at_3(c, S2, 0x64));
                sq(c, 2, at_3(c, S2, 0x30));
                sq(c, 2, r32(c, SP));
                c.mul_s(0, 0, 4);
                c.g[V0][0] = lw_2(c, at_3(c, S2, 0x14));
                c.lwc1(4, at_3(c, S2, 0x58));
                c.lwc1(1, at_3(c, V0, 0x70));
                c.lwc1(2, at_3(c, S2, 0x5c));
                c.add_s(0, 0, 1);
                c.lwc1(3, at_3(c, S2, 0x60));
                c.swc1(4, at_3(c, SP, 0x10));
                c.swc1(2, at_3(c, SP, 0x14));
                c.swc1(0, at_3(c, S2, 0x40));
                c.swc1(3, at_3(c, SP, 0x18));
                c.swc1(5, at_3(c, SP, 0x1c));
                c.lqc2(8, at_3(c, SP, 0x10));
                c.lqc2(4, r32(c, S1));
                c.lqc2(5, at_3(c, S1, 0x10));
                c.lqc2(6, at_3(c, S1, 0x20));
                c.lqc2(7, at_3(c, S1, 0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, at_3(c, SP, 0x20));
                lq(c, 2, at_3(c, SP, 0x20));
                sq(c, 2, r32(c, SP));
                sq(c, 2, at_3(c, S2, 0x20));
                c.lqc2(3, at_3(c, S2, 0x20));
                c.lqc2(4, at_3(c, S2, 0x30));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x10));
                lq(c, 2, at_3(c, SP, 0x10));
                sq(c, 2, r32(c, SP));
                c.lwc1(20, r32(c, SP));
                c.c_eq_s(20, 22);
                let t = !c.fcc();
                c.lwc1(21, at_3(c, SP, 0x4));
                if t {
                    break 'b_169700;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(12, 21);
                    if t {
                        break 'b_16973c;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(12, r32(c, GP).wrapping_sub(0x6540));
                        let t = !c.fcc();
                        c.lwc1(0, r32(c, SP));
                        if t {
                            break 'b_16973c;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, r32(c, GP).wrapping_sub(0x653c));
                            let t = true;
                            c.lwc1(1, at_3(c, SP, 0x4));
                            if t {
                                break 'b_169744;
                            } else {
                                break 'b_169700;
                            }
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            c.g[RA][0] = sx(0x0016970c);
            if call_ptr(c, 0x0031c228, 0x0016970c).is_err() {
                return;
            }
            c.c_lt_s(20, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_16973c;
            } else {
                'b_169734: {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();

                    if t {
                        break 'b_169734;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x6538));
                        let t = true;
                        c.add_s(12, 12, 0);
                        if t {
                            break 'b_16973c;
                        } else {
                            break 'b_169734;
                        }
                    }
                }
                c.cycles += 2;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x6534));
                c.sub_s(12, 12, 0);
                break 'b_16973c;
            }
        }
        c.cycles += 2;
        c.lwc1(0, r32(c, SP));
        c.lwc1(1, at_3(c, SP, 0x4));
        break 'b_169744;
    }
    'b_1697e8: {
        'b_1697e0: {
            'b_1697a0: {
                c.cycles += 12;
                c.mul_s(2, 0, 0);
                c.swc1(12, at_3(c, S2, 0x44));
                c.mul_s(3, 1, 1);
                c.swc1(0, at_3(c, SP, 0x10));
                c.mtc1(0, 22);
                c.swc1(1, at_3(c, SP, 0x14));
                c.add_s(2, 2, 3);
                c.lwc1(21, at_3(c, SP, 0x8));
                c.sqrt_s(20, 2);
                c.c_eq_s(20, 22);
                let t = !c.fcc();

                if t {
                    break 'b_1697a0;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(0, 21);
                    if t {
                        break 'b_1697e0;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x6530));
                        let t = !c.fcc();
                        c.g[S0][0] = sx(at_3(c, S2, 0x74));
                        if t {
                            break 'b_1697e0;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, r32(c, GP).wrapping_sub(0x652c));
                            let t = true;
                            c.lwc1(12, at_3(c, S2, 0x6c));
                            if t {
                                break 'b_1697e8;
                            } else {
                                break 'b_1697a0;
                            }
                        }
                    }
                }
            }
            'b_1697dc: {
                c.cycles += 7;
                c.div_s(12, 21, 20);
                c.g[RA][0] = sx(0x001697ac);
                if call_ptr(c, 0x0031c228, 0x001697ac).is_err() {
                    return;
                }
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_1697dc;
                } else {
                    'b_1697d4: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();

                        if t {
                            break 'b_1697d4;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, r32(c, GP).wrapping_sub(0x6528));
                            let t = true;
                            c.add_s(12, 12, 0);
                            if t {
                                break 'b_1697dc;
                            } else {
                                break 'b_1697d4;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, r32(c, GP).wrapping_sub(0x6524));
                    c.sub_s(12, 12, 0);
                    break 'b_1697dc;
                }
            }
            c.cycles += 1;
            c.mov_s(0, 12);
            break 'b_1697e0;
        }
        c.cycles += 2;
        c.g[S0][0] = sx(at_3(c, S2, 0x74));
        c.lwc1(12, at_3(c, S2, 0x6c));
        break 'b_1697e8;
    }
    c.cycles += 15;
    c.swc1(0, at_3(c, S2, 0x48));
    c.g[RA][0] = sx(0x001697f4);
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x00160028, 0x001697f4).is_err() {
        return;
    }
    c.lwc1(12, at_3(c, S2, 0x70));
    c.g[RA][0] = sx(0x00169800);
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x00160130, 0x00169800).is_err() {
        return;
    }
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    c.g[RA][0] = ld(c, at_3(c, SP, 0x30));
    c.lwc1(22, at_3(c, SP, 0x80));
    c.lwc1(21, at_3(c, SP, 0x78));
    c.lwc1(20, at_3(c, SP, 0x70));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x90));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    pub(crate) use crate::game::be::beintmission::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::be::beintstat::tests::call;
    pub(crate) use crate::game::be::beintstat::tests::pool_allocator;
    pub(crate) use crate::game::be::beintstat::tests::MEM_HEAP;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_3;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_2;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_2;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::W;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn algorithmic_script_camera_algo_ctor_matches() {
        check(
            ee::s02::f_0016ad38,
            algorithmic_script_camera_algo_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xad38, 0);
                c.g[A0][0] = DATA as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x180 / 4);
            },
        );
    }

    #[test]
    fn algorithmic_script_camera_algo_reset_matches() {
        covered(
            ee::s02::f_0016b150,
            algorithmic_script_camera_algo_reset,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb150, 1);
                let this = ptr(&mut r);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(this + 0x10, vt);
                c.g[A0][0] = this as u64;
            },
        );
    }

    pub(crate) const SEEDS_2: u32 = 250;

    pub(crate) const OUT: u32 = DATA;

    pub(crate) const CAM: u32 = DATA + 0x800;

    /// `1/(2 pi)`-style wobble angle scale in `.sdata`.
    pub(crate) const WOBBLE_SCALE: u32 = 0x0049_c4d4;

    /// The vector `CameraWobble_GetOffset` returns when the wobble is off.
    pub(crate) const NO_WOBBLE: u32 = 0x004f_f120;

    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// The boot-time matrices/axes and the wobble support globals.
    pub(crate) fn put_globals(c: &mut Cpu, r: &mut Rng) {
        put_vec(c, IDENTITY_MAT, [f(1.0), 0, 0, 0]);
        put_vec(c, IDENTITY_MAT + 0x10, [0, f(1.0), 0, 0]);
        put_vec(c, IDENTITY_MAT + 0x20, [0, 0, f(1.0), 0]);
        put_vec(c, IDENTITY_MAT + 0x30, [0, 0, 0, f(1.0)]);
        put_vec(c, AXIS_X, [f(1.0), 0, 0, 0]);
        put_vec(c, AXIS_Z, [0, 0, f(1.0), 0]);
        put_vec(c, AXIS_Y, [0, f(1.0), 0, 0]);
        put_vec(
            c,
            NO_WOBBLE,
            [r.f32(3.0), r.f32(3.0), r.f32(3.0), r.f32(3.0)],
        );
        c.bus.write32(WOBBLE_SCALE, r.f32(1.0));
    }

    /// A script camera: position at +0x30, script angles at +0x40..0x48 and
    /// a wobble sub-object at +0x70 (also covering the +0x74 variant).
    pub(crate) fn put_script_camera(c: &mut Cpu, r: &mut Rng, seed: u32) {
        put_vec(
            c,
            CAM + camera::POSITION,
            [r.f32(500.0), r.f32(500.0), r.f32(500.0), 0],
        );
        for off in [camera::ANGLE_PITCH, camera::ANGLE_A, camera::ANGLE_B] {
            c.bus.write32(CAM + off, r.f32(6.0));
        }
        for k in 0..12u32 {
            c.bus.write32(CAM + 0x70 + 4 * k, r.f32(4.0));
        }
        // Every fourth seed takes the wobble's early out (offset 0x18 is the
        // disabled flag); some seeds divide by a zero period.
        let zero = if seed % 4 == 0 { 0 } else { r.f32(2.0) };
        c.bus.write32(CAM + 0x70 + 0x18, zero);
        c.bus.write32(CAM + 0x74 + 0x18, zero);
        if seed % 7 == 0 {
            c.bus.write32(CAM + 0x70 + 0x2c, 0);
            c.bus.write32(CAM + 0x74 + 0x2c, 0);
        }
        if seed % 5 == 0 {
            c.bus.write32(CAM + 0x74 + 0x1c, 0);
        }
    }

    #[test]
    fn manual_script_camera_get_view_matches() {
        check(
            ee::s02::f_00169828__generated,
            manual_script_camera_get_view,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0016_9828 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                put_globals(c, &mut r);
                put_script_camera(c, &mut r, seed);
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(CAM);
            },
        );
    }

    #[test]
    fn target_script_camera_get_view_matches() {
        check(
            ee::s02::f_00169f88__generated,
            target_script_camera_get_view,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0016_9f88 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                put_globals(c, &mut r);
                put_script_camera(c, &mut r, seed);
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(CAM);
            },
        );
    }

    #[test]
    fn subject_script_camera_get_view_matches() {
        check(
            ee::s02::f_0016a868__generated,
            subject_script_camera_get_view,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0016_a868 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                put_globals(c, &mut r);
                put_script_camera(c, &mut r, seed);
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(CAM);
            },
        );
    }

    #[test]
    fn algorithmic_script_camera_get_view_matches() {
        check(
            ee::s02::f_0016b7b8__generated,
            algorithmic_script_camera_get_view,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0016_b7b8 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                put_globals(c, &mut r);
                put_script_camera(c, &mut r, seed);
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(CAM);
            },
        );
    }

    #[test]
    fn script_cam_controller_push_algorithm_matches() {
        crate::difftest::check(
            ee::s02::f_00169340__generated,
            script_cam_controller_push_algorithm,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1693_40, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                // The new algorithm; a null vtable makes its "notify" call go to
                // the fake hook (or be skipped).
                c.g[A1][0] = sx(DATA + 0x300);
                c.bus
                    .write32(DATA + 8, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
                c.bus.write32(
                    DATA + 0x300 + 0x10,
                    if seed % 3 == 0 { 0 } else { DATA + 0x600 },
                );
                c.bus.write32(DATA + 0x600 + 0x24, 0);
                c.bus.write32(DATA + 0x500 + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn camera_trigger_list_dtor_matches() {
        crate::difftest::check(
            ee::s02::f_0016bef8__generated,
            camera_trigger_list_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16be_f8, 2);
                be_library(c);
                if seed % 2 == 1 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = 1;
                } else {
                    c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                    c.g[A1][0] = sx(r.next_u32() & !1);
                }
            },
        );
    }

    // ---------------------------------------------------------------- camera

    /// A camera algorithm object at a random pointer whose `v05` slot calls
    /// the fake, or unwinds at it.
    pub(crate) fn algo_reset_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        let obj = ptr(&mut r);
        c.g[A0][0] = sx(obj);
        let vt = vtable(c, &mut r, &[0x28]);
        c.bus.write32(obj.wrapping_add(0x10), vt);
    }

    #[test]
    fn manual_script_camera_algo_reset_for_rider_matches() {
        covered(
            ssx3_ee::s02::f_00169518__generated,
            manual_script_camera_algo_reset_for_rider,
            120,
            &|c, seed| algo_reset_setup(c, seed, 0x11),
        );
    }

    #[test]
    fn target_script_camera_algo_reset_for_rider_matches() {
        covered(
            ssx3_ee::s02::f_00169d58__generated,
            target_script_camera_algo_reset_for_rider,
            120,
            &|c, seed| algo_reset_setup(c, seed, 0x12),
        );
    }

    #[test]
    fn subject_script_camera_algo_reset_for_rider_matches() {
        covered(
            ssx3_ee::s02::f_0016a4b8__generated,
            subject_script_camera_algo_reset_for_rider,
            120,
            &|c, seed| algo_reset_setup(c, seed, 0x13),
        );
    }

    #[test]
    fn manual_script_camera_algo_reset_matches() {
        covered(
            s02::f_00169540__generated,
            manual_script_camera_algo_reset,
            120,
            &|c, seed| {
                machine(c, seed, 0x169540, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x40);
                slot(c, VT, 0x18);
                c.bus.write32(OBJ + 0x10, VT);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn target_script_camera_algo_reset_matches() {
        covered(
            s02::f_00169d80__generated,
            target_script_camera_algo_reset,
            120,
            &|c, seed| {
                machine(c, seed, 0x169d80, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x40);
                slot(c, VT, 0x18);
                c.bus.write32(OBJ + 0x10, VT);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn subject_script_camera_algo_reset_matches() {
        covered(
            s02::f_0016a4e0__generated,
            subject_script_camera_algo_reset,
            120,
            &|c, seed| {
                machine(c, seed, 0x16a4e0, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x40);
                slot(c, VT, 0x18);
                c.bus.write32(OBJ + 0x10, VT);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn manual_script_camera_algo_ctor_matches() {
        check(
            ssx3_ee::s02::f_001694b8__generated,
            manual_script_camera_algo_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x1694b8, 0);
                c.bus.write32(OBJ_2, seed);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    #[test]
    fn target_script_camera_algo_ctor_matches() {
        check(
            ssx3_ee::s02::f_00169cf8__generated,
            target_script_camera_algo_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x169cf8, 0);
                c.bus.write32(OBJ_2, seed);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    #[test]
    fn subject_script_camera_algo_ctor_matches() {
        check(
            ssx3_ee::s02::f_0016a458__generated,
            subject_script_camera_algo_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x16a458, 0);
                c.bus.write32(OBJ_2, seed);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    /// Call a guest setup function and return `v0`.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena: the game
    /// heap, for the paths that allocate more than the pools hold.
    pub(crate) fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    #[test]
    fn script_cam_controller_stop_script_camera_matches() {
        check(
            ee::s02::f_001692d8__generated,
            script_cam_controller_stop_script_camera,
            40,
            &|c, seed| {
                base(c, seed, 0x16_92d8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0x1c, 0);
                c.bus
                    .write32(DATA + 0x18, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x8, DATA + 0x300);
                c.bus
                    .write32(DATA + 0x304, if seed % 3 == 2 { 1 } else { 4 });
                match seed % 3 {
                    0 => {
                        c.bus.write32(DATA + 0x300, 0);
                    }
                    1 => {
                        // No key match: the node at the head has key 0x77.
                        let node = DATA + 0x500;
                        c.bus.write32(node, 0x77);
                        c.bus.write32(node + 0x10, 0);
                        c.bus.write32(node + 0x14, 0);
                        c.bus.write32(DATA + 0x300, node);
                    }
                    _ => {
                        // A real heap block, so `AlgorithmList_Remove` can free it.
                        heap_init(c);
                        let node = run(
                            c,
                            0x0032_0058,
                            ee::s10::MEM_HeapMemalign,
                            &[0x004f_f250, 16, 0x40],
                        ) as u32;
                        for off in [0u32, 0x10, 0x14, 0x18] {
                            c.bus.write32(node + off, 0);
                        }
                        c.bus.write32(DATA + 0x300, node);
                    }
                }
            },
        );
    }

    #[test]
    fn script_camera_algo_ctor_matches() {
        check(
            ee::s02::f_00169418__generated,
            script_camera_algo_ctor,
            100,
            &|c, seed| {
                let _ = base_2(c, seed, 0x16_9418);
                globals(c);
                let game = WORLD + 0x1000;
                let list = WORLD + 0x2000;
                let ctrl = WORLD + 0x3000;
                w(c, WORLD + 0x84, game);
                w(c, game + 0x84, list);
                w(c, list + 4, ctrl);
                w(c, ctrl + 0xac, seed ^ 0x77);
                w(c, GAME_GP.wrapping_sub(0x6548), 0x42c8_0000);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn script_cam_controller_dtor_matches() {
        strict(
            ee::s02::f_001690d0__generated,
            script_cam_controller_dtor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1690, 4);
                let this = DATA;
                let array = DATA + 0x400;
                let vt = DATA + 0x600;
                c.bus.write32(this + 0x1c, array);
                for i in 0..2u32 {
                    let elem = DATA + 0x800 + 0x100 * i;
                    c.bus.write32(array + 4 * i, elem);
                    c.bus.write32(elem + 4, r.next_u32());
                    c.bus.write32(elem + 0xc, vt);
                }
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE);
                let se = DATA + 0x1000;
                let mgr = DATA + 0x1400;
                c.bus.write32(GAME_GP + 0x3ec, se);
                c.bus.write32(se + 0x2a8, mgr);
                c.bus.write16(mgr + 0x18, 0);
                c.bus.write32(mgr + 0x1c, FAKE);
                c.bus.write32(this + 8, 0);
                c.bus.write32(this + 0x14, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed as u64) & !1;
            },
        );
    }

    // ------------------------------------------------------- script camera

    #[test]
    fn script_cam_controller_ctor_matches() {
        strict_2(
            ee::s02::f_00168fe8__generated,
            script_cam_controller_ctor,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x168f, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed % 4) + 1) as u64;
            },
        );
    }

    pub(crate) const TARGET_CAM: u32 = DATA + 0x1_0000;

    pub(crate) const TARGET_RIG: u32 = DATA + 0x1_1000;

    pub(crate) const TARGET_ORBIT: u32 = DATA + 0x1_2000;

    #[test]
    fn target_script_camera_algo_update_matches() {
        check_all(
            ee::s02::f_00169db0__generated,
            target_script_camera_algo_update,
            20,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x169d_b0, 10);
                c.g[A0][0] = sx(TARGET_CAM);
                c.bus.write32(TARGET_CAM + 0x14, TARGET_RIG);
                c.bus.write32(TARGET_RIG + 0x14, TARGET_ORBIT);
                c.bus.write32(TARGET_ORBIT + 0x8, DATA + 0x1_3000);
                c.bus.write32(TARGET_RIG + 0x30, r.next_u32() % 0x40);
                for i in 0..0x10 {
                    c.bus.write32(TARGET_RIG + 0x30 + 4 * i, r.f32(1.0));
                }
                for i in 0..0x20 {
                    c.bus.write32(TARGET_CAM + 0x40 + 4 * i, r.f32(10.0));
                }
            },
        );
    }

    pub(crate) const MAN_CAM: u32 = DATA + 0x1_0000;

    pub(crate) const MAN_RIG: u32 = DATA + 0x1_1000;

    pub(crate) const MAN_ORBIT: u32 = DATA + 0x1_2000;

    #[test]
    fn manual_script_camera_algo_update_matches() {
        check_all(
            ee::s02::f_00169570__generated,
            manual_script_camera_algo_update,
            20,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x1695_70, 10);
                c.g[A0][0] = sx(MAN_CAM);
                c.bus.write32(MAN_CAM + 0x14, MAN_RIG);
                c.bus.write32(MAN_RIG + 0x14, MAN_ORBIT);
                c.bus.write32(MAN_ORBIT + 0x8, DATA + 0x1_3000);
                for i in 0..0x10 {
                    c.bus.write32(MAN_RIG + 0x30 + 4 * i, r.f32(1.0));
                }
                for i in 0..0x20 {
                    c.bus.write32(MAN_CAM + 0x40 + 4 * i, r.f32(10.0));
                }
            },
        );
    }
}
