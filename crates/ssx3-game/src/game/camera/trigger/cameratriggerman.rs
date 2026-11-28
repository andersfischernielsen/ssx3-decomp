//! `src/camera/trigger/cameratriggerman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::camera::script::scriptcontroller::copy_matrix as copy_matrix_2;
pub(crate) use crate::game::camera::script::scriptcontroller::mat_vec;
pub(crate) use crate::game::camera::script::scriptcontroller::quad_copy;
pub(crate) use crate::game::camera::script::scriptcontroller::rot_axis as rot_axis_2;
pub(crate) use crate::game::camera::script::scriptcontroller::AXIS_X;
pub(crate) use crate::game::camera::script::scriptcontroller::AXIS_Y;
pub(crate) use crate::game::camera::script::scriptcontroller::AXIS_Z;
pub(crate) use crate::game::camera::script::scriptcontroller::IDENTITY_MAT;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::copy_matrix;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::mul_matrix_vec;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::rot_axis;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::ssxapp::oc429_at;
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
        0x0016_c9b0,
        camera_trigger_man_unload,
        "CameraTriggerMan_Unload",
    ),
    (
        0x0016_cf18,
        camera_trigger_man_empty_global_call,
        "CameraTriggerMan_EmptyGlobalCall",
    ),
    (
        0x0016_d210,
        camera_trigger_stack_ctor,
        "CameraTriggerStack_ctor",
    ),
    (
        0x0016_d238,
        camera_trigger_man_push_trigger_stack,
        "CameraTriggerMan_PushTriggerStack",
    ),
    (
        0x0016_d260,
        camera_trigger_man_pop_trigger_stack,
        "CameraTriggerMan_PopTriggerStack",
    ),
    (
        0x0016_d320,
        camera_trigger_man_update_for_rider,
        "CameraTriggerMan_UpdateForRider",
    ),
    (
        0x0017_0240,
        camera_line_bound_world_start,
        "CameraLineBoundObj_GetWorldStart",
    ),
    (
        0x0017_07b0,
        camera_line_bound_world_end,
        "CameraLineBoundObj_GetWorldEnd",
    ),
    (
        0x0017_0d20,
        camera_bound_obj_get_world_point,
        "CameraBoundObj_GetWorldPoint",
    ),
    (
        0x0016_cbe0,
        camera_trigger_man_remove_all_triggers,
        "CameraTriggerMan_RemoveAllTriggers",
    ),
    (
        0x0016_cf48,
        camera_trigger_man_update_triggers,
        "CameraTriggerMan_UpdateTriggers",
    ),
    (
        0x0016_ca10,
        camera_trigger_man_empty_hook,
        "CameraTriggerMan_EmptyHook",
    ),
    (
        0x0016_ce30,
        oc436_camera_trigger_man_reload,
        "CameraTriggerMan_Reload",
    ),
    (
        0x0016_cec8,
        camera_trigger_man_reload_if_global,
        "CameraTriggerMan_ReloadIf_Global",
    ),
    (
        0x0016_cef0,
        camera_trigger_man_reload_if,
        "CameraTriggerMan_ReloadIf",
    ),
    (
        0x0016_d1b8,
        camera_trigger_man_update_triggers_mask2,
        "CameraTriggerMan_UpdateTriggersMask2",
    ),
    (
        0x0016_d200,
        oc436_camera_trigger_stack_clear,
        "CameraTriggerStack_Clear",
    ),
    (
        0x0016_f068,
        camera_box_bound_obj_clamp_point,
        "CameraBoxBoundObj_ClampPoint",
    ),
    (
        0x0016_cdc8,
        camera_trigger_man_load,
        "CameraTriggerMan_Load",
    ),
    (
        0x0016_cea8,
        camera_trigger_man_remove_trigger_regions_wrapper,
        "CameraTriggerMan_RemoveTriggerRegions_Wrapper",
    ),
    (
        0x0016_cf40,
        camera_trigger_man_empty_stub,
        "CameraTriggerMan_EmptyStub",
    ),
    (
        0x0016_d1d8,
        camera_trigger_man_update_triggers_mask1,
        "CameraTriggerMan_UpdateTriggersMask1",
    ),
    (
        0x0016_e560,
        camera_ellipse_bound_obj_clamp_point,
        "CameraEllipseBoundObj_ClampPoint",
    ),
    (
        0x0016_fbb8,
        camera_line_bound_obj_clamp_point,
        "CameraLineBoundObj_ClampPoint",
    ),
    (
        0x0016_ca18,
        camera_trigger_man_remove_trigger_regions,
        "CameraTriggerMan_RemoveTriggerRegions",
    ),
    (
        0x0016_e1d8,
        camera_trigger_apply_action,
        "CameraTrigger_ApplyAction",
    ),
    (
        0x0016_c428,
        camera_trigger_man_read_trigger,
        "CameraTriggerMan_ReadTrigger",
    ),
    (
        0x0016_c4c8,
        camera_trigger_man_read_trigger_volume,
        "CameraTriggerMan_ReadTriggerVolume",
    ),
    (
        0x0016_c4f0,
        camera_trigger_man_read_bound_obj_action,
        "CameraTriggerMan_ReadBoundObjAction",
    ),
    (
        0x0016_c518,
        camera_trigger_man_read_trigger_action,
        "CameraTriggerMan_ReadTriggerAction",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CAMERA_TRIGGER_STACK_CLEAR;

    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;

    pub const CLEAR_FLAGS: Callee = Callee {
        addr: 0x0016_c678,
        f: ee::s02::f_0016c678,
    };

    pub const FIND: Callee = Callee {
        addr: 0x0016_c6b0,
        f: ee::s02::f_0016c6b0,
    };

    pub const ADD: Callee = Callee {
        addr: 0x0016_c718,
        f: ee::s02::f_0016c718,
    };

    pub const FIND_BY_ID: Callee = Callee {
        addr: 0x0016_c6e0,
        f: ee::s02::f_0016c6e0,
    };

    pub const REMOVE: Callee = Callee {
        addr: 0x0016_c778,
        f: ee::s02::f_0016c778,
    };

    pub const POP_TRIGGER: Callee = Callee {
        addr: 0x0016_d260,
        f: ee::s02::f_0016d260,
    };

    pub const PUSH_TRIGGER: Callee = Callee {
        addr: 0x0016_d238,
        f: ee::s02::f_0016d238,
    };

    pub const APPLY_ACTION: Callee = Callee {
        addr: 0x0016_e1d8,
        f: ee::s02::f_0016e1d8,
    };

    pub const COLLECT_NODE: Callee = Callee {
        addr: 0x0017_6890,
        f: ee::s02::f_00176890,
    };

    pub const COLLECT_BOX: Callee = Callee {
        addr: 0x0017_4848,
        f: ee::s02::f_00174848,
    };

    pub const CLASSIFY_AABB: u32 = 0x0032_8360;
}

/// Return addresses of the four `GetView` bodies: the addresses their calls
/// come back to, relative to the function's entry.
pub(crate) mod ret {
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::LINE_ROT_X;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::LINE_ROT_Y;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::LINE_ROT_Z;
}

/// Fields of a line-bound object (the matrix source).
pub(crate) mod line_bound {

    /// Translation (x, y, z) at +0, the three rotation angles and the scale
    /// (x, y, z) at +0x18, then the local start point at +0x2c and the end
    /// point at +0x38.
    pub const TRANSLATION: u32 = 0x0;

    pub const ROT_Z: u32 = 0xc;

    pub const ROT_X: u32 = 0x10;

    pub const ROT_Y: u32 = 0x14;

    pub const SCALE: u32 = 0x18;
}

// ---------------------------------------------------------------------------
// Camera triggers.

/// `void CameraTriggerMan_PushTriggerStack(CameraTriggerMan *man, uint value)`
/// (0x16d238): write `value` into `man->stack[man->count]` and at +0x4, then
/// bump the count.
pub fn camera_trigger_man_push_trigger_stack(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let value = c.g[A1][0];
    c.cycles += 9;
    let count = man.lw(c) as u32;
    let slot = man.0.wrapping_add(count << 2);
    c.g[V0][0] = sx(slot);
    Addr(slot).off(0x8).sw(c, value);
    man.off(0x4).sw(c, value);
    let count = man.lw(c) as u32;
    c.g[V0][0] = sx(count.wrapping_add(1));
    man.sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera bound object.

/// `void CameraBoundObj_GetWorldPoint(Vec4* out, void* src, int corner)`
/// (0x170d20): writes the world-space position of one of the object's eight
/// corners (`src + 0x28 + corner*12`) through the matrix built from `src`'s
/// translation, the three angles at `+0x10`, `+0x14`, `+0xc` and the scale at
/// `+0x18`.
pub fn camera_bound_obj_get_world_point(c: &mut Cpu) {
    guest(c, camera_bound_obj_get_world_point_body)
}

pub(crate) fn camera_bound_obj_get_world_point_body(c: &mut Cpu) -> Flow {
    c.cycles += 355;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
    let sp = c.g[SP][0] as u32;
    let out = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    let corner_index = abi::arg(c, 2);

    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    c.sq(16, sp.wrapping_add(0x140));
    c.sq(18, sp.wrapping_add(0x120));
    c.g[S2][0] = sx(corner_index);
    c.sq(20, sp.wrapping_add(0x100));
    c.sq(17, sp.wrapping_add(0x130));
    c.sq(19, sp.wrapping_add(0x110));
    c.sd(sp.wrapping_add(0xf0), c.g[RA][0]);
    c.swc1(20, sp.wrapping_add(0x150));

    // m = identity, then m = m * translate(src->pos).
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, sp);
    c.sqc2(2, sp.wrapping_add(0x10));
    c.sqc2(3, sp.wrapping_add(0x20));
    c.sqc2(4, sp.wrapping_add(0x30));
    c.lwc1(0, src);
    c.lwc1(1, src.wrapping_add(0x4));
    c.lwc1(2, src.wrapping_add(0x8));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.swc1(0, sp.wrapping_add(0x40));
    c.swc1(1, sp.wrapping_add(0x44));
    c.swc1(2, sp.wrapping_add(0x48));
    c.swc1(20, sp.wrapping_add(0x4c));
    mul_matrix_vec(c, sp.wrapping_add(0x30), sp, sp.wrapping_add(0x40));

    // Three axis rotations: x, then y, then z (angles not negated here).
    rot_axis(
        c,
        false,
        sp.wrapping_add(0x50),
        src,
        0x10,
        0x004f_f140,
        sp.wrapping_add(0xd0),
        sp.wrapping_add(0xd4),
        0x0017_0dd4,
    )?;
    rot_axis(
        c,
        false,
        sp.wrapping_add(0x40),
        src,
        0x14,
        0x004f_f150,
        sp.wrapping_add(0xd8),
        sp.wrapping_add(0xdc),
        0x0017_0f1c,
    )?;
    rot_axis(
        c,
        false,
        sp.wrapping_add(0x40),
        src,
        0xc,
        0x004f_f160,
        sp.wrapping_add(0xe0),
        sp.wrapping_add(0xe4),
        0x0017_1064,
    )?;

    // m = m * scale(src->scale).
    let work = sp.wrapping_add(0x90);
    c.lwc1(2, src.wrapping_add(0x18));
    c.lwc1(1, src.wrapping_add(0x1c));
    c.lwc1(0, src.wrapping_add(0x20));
    c.swc1(2, sp.wrapping_add(0x80));
    c.swc1(1, sp.wrapping_add(0x84));
    c.swc1(0, sp.wrapping_add(0x88));
    c.swc1(20, sp.wrapping_add(0x8c));
    c.lqc2(1, sp.wrapping_add(0x80));
    c.lqc2(4, sp);
    c.lqc2(5, sp.wrapping_add(0x10));
    c.lqc2(6, sp.wrapping_add(0x20));
    c.lqc2(7, sp.wrapping_add(0x30));
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    c.sqc2(8, work);
    c.sqc2(9, work.wrapping_add(0x10));
    c.sqc2(10, work.wrapping_add(0x20));
    c.sqc2(11, work.wrapping_add(0x30));
    copy_matrix(c, sp.wrapping_add(0x40), work);
    copy_matrix(c, sp, sp.wrapping_add(0x40));

    // out = m * (src + 0x28 + corner*12), a 128-bit store.
    c.g[V1][0] = 0xc;
    c.g[V0][0] = sx(out);
    c.mult(0, 4, 18, 3);
    c.lq(17, sp.wrapping_add(0x130));
    c.lq(19, sp.wrapping_add(0x110));
    c.g[RA][0] = c.ld(sp.wrapping_add(0xf0));
    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(src));
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
    c.lq(16, sp.wrapping_add(0x140));
    let corner = c.g[S2][0] as u32;
    c.lwc1(0, corner);
    c.swc1(0, sp.wrapping_add(0x40));
    c.lwc1(1, corner.wrapping_add(0x4));
    c.swc1(1, sp.wrapping_add(0x44));
    c.lwc1(0, corner.wrapping_add(0x8));
    c.swc1(20, sp.wrapping_add(0x4c));
    c.swc1(0, sp.wrapping_add(0x48));
    mul_matrix_vec(c, sp.wrapping_add(0x50), sp, sp.wrapping_add(0x40));
    c.lq(18, sp.wrapping_add(0x120));
    c.lq(3, sp.wrapping_add(0x50));
    c.lwc1(20, sp.wrapping_add(0x150));
    c.sq(3, out);
    c.lq(20, sp.wrapping_add(0x100));
    c.g[SP][0] = sx(sp.wrapping_add(0x160));
    abi::ret(c);
    Ok(())
}

/// `m = m * diag(scale)`: each column of `m` scaled by one component of the
/// scale vector, through the stack scratch at `tmp`, then copied to `via`
/// and back into `m` (the original's double copy).
pub(crate) fn scale_columns(c: &mut Cpu, m: u32, scale: u32, tmp: u32, via: u32) {
    c.lqc2(1, scale);
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    for i in 0..4u32 {
        c.sqc2(8 + i as usize, tmp + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, tmp + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, via + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, via + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, m + 0x10 * i);
    }
}

// ---------------------------------------------------------------------------
// Line-bound endpoints.

/// Shared body of `CameraLineBoundObj_GetWorldStart/End`: build the bound's
/// transform (translation, then rotations about X, Z and Y, then a
/// non-uniform scale) and transform `point` into world space at `out`.
pub(crate) fn camera_line_bound_world(
    c: &mut Cpu,
    out: u64,
    bound: u32,
    point: u32,
    base: u32,
) -> Flow {
    c.cycles += 348;
    let frame = Frame::open(c, 0x150);
    frame.save(c, S0, 0x130);
    frame.save(c, S1, 0x120);
    frame.save(c, S2, 0x110);
    frame.save(c, S3, 0x100);
    frame.save_ra(c, 0xf0);
    frame.save_fpr(c, 20, 0x140);

    let m = sp(c);
    copy_matrix_2(c, m, IDENTITY_MAT);

    // Translation: (x, y, z, 1) from the bound's first three words.
    c.f[0] = Addr(bound + line_bound::TRANSLATION).read_fpu(c).0;
    c.f[1] = Addr(bound + line_bound::TRANSLATION + 4).read_fpu(c).0;
    c.f[2] = Addr(bound + line_bound::TRANSLATION + 8).read_fpu(c).0;
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.mtc1(1, 20);
    Addr(m + 0x40).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x44).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x48).write_fpu(c, FpuF32(c.f[2]));
    Addr(m + 0x4c).write_fpu(c, FpuF32::ONE);
    mat_vec(c, m, m + 0x40, m + 0x30);

    let a = Addr(bound + line_bound::ROT_X).read_fpu(c);
    rot_axis_2(
        c,
        m,
        m + 0x50,
        AXIS_X,
        a,
        m + 0xd0,
        m + 0xd4,
        base + ret::LINE_ROT_X,
    )?;
    let b = Addr(bound + line_bound::ROT_Y).read_fpu(c);
    rot_axis_2(
        c,
        m,
        m + 0x40,
        AXIS_Z,
        b,
        m + 0xd8,
        m + 0xdc,
        base + ret::LINE_ROT_Z,
    )?;
    let cc = Addr(bound + line_bound::ROT_Z).read_fpu(c);
    rot_axis_2(
        c,
        m,
        m + 0x40,
        AXIS_Y,
        cc,
        m + 0xe0,
        m + 0xe4,
        base + ret::LINE_ROT_Y,
    )?;

    // Scale: (sx, sy, sz, 1) multiplies the matrix columns.
    c.f[2] = Addr(bound + line_bound::SCALE).read_fpu(c).0;
    c.f[1] = Addr(bound + line_bound::SCALE + 4).read_fpu(c).0;
    c.f[0] = Addr(bound + line_bound::SCALE + 8).read_fpu(c).0;
    Addr(m + 0x80).write_fpu(c, FpuF32(c.f[2]));
    Addr(m + 0x84).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x88).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x8c).write_fpu(c, FpuF32::ONE);
    scale_columns(c, m, m + 0x80, m + 0x90, m + 0x40);

    // Transform the point (x, y, z, 1) and copy the result to `out`.
    c.f[2] = Addr(bound + point + 8).read_fpu(c).0;
    c.g[V0][0] = out;
    c.f[0] = Addr(bound + point).read_fpu(c).0;
    c.f[1] = Addr(bound + point + 4).read_fpu(c).0;
    Addr(m + 0x4c).write_fpu(c, FpuF32::ONE);
    Addr(m + 0x40).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x44).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x48).write_fpu(c, FpuF32(c.f[2]));
    mat_vec(c, m, m + 0x40, m + 0x50);
    frame.restore(c, S0, 0x130);
    quad_copy(c, m + 0x50, out as u32);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S3, 0x100);
    frame.restore(c, S2, 0x110);
    frame.restore_ra(c, 0xf0);
    frame.restore_fpr(c, 20, 0x140);
    frame.close(c);
    Ok(())
}

/// `void CameraLineBoundObj_GetWorldStart(vec4 *out, LineBound *b)` (0x170240).
pub fn camera_line_bound_world_start(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let bound = abi::arg(c, 1);
        camera_line_bound_world(c, out, bound, 0x2c, 0x0017_0240)
    })
}

/// `void CameraLineBoundObj_GetWorldEnd(vec4 *out, LineBound *b)` (0x1707b0).
pub fn camera_line_bound_world_end(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let bound = abi::arg(c, 1);
        camera_line_bound_world(c, out, bound, 0x38, 0x0017_07b0)
    })
}

// ---------------------------------------------------------------------------
// CameraTriggerMan_EmptyGlobalCall.

/// `void CameraTriggerMan_EmptyGlobalCall(CameraTriggerMan* m)` (0x16cf18):
/// `CameraTriggerMan_EmptyStub(D_004C5830, m)`.
pub fn camera_trigger_man_empty_global_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5830));
        callee::CAMERA_TRIGGER_EMPTY_STUB.call(c, 0x0016_cf30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera trigger stack and array element destructors.

/// `cCameraTriggerStack* CameraTriggerStack_ctor(cCameraTriggerStack* self)`
/// (0x16d210): clear the stack and return it.
pub fn camera_trigger_stack_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_d224)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_Unload(CameraTriggerMan* m)` (0x16c9b0): purge both
/// active lists and both stacks, remove every trigger, clear the data and
/// reset the chase controllers.
pub fn camera_trigger_man_unload(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
    if callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR
        .call(c, 0x0016_c9c8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
    if callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR
        .call(c, 0x0016_c9d0)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
    if callee::CAMERA_TRIGGER_STACK_CLEAR_2
        .call(c, 0x0016_c9d8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
    if callee::CAMERA_TRIGGER_STACK_CLEAR_2
        .call(c, 0x0016_c9e0)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS
        .call(c, 0x0016_c9e8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if callee::CAMERA_TRIGGER_MAN_CLEAR_DATA
        .call(c, 0x0016_c9f0)
        .is_err()
    {
        return;
    }
    if call_ptr(c, 0x0015_dc18, 0x0016_c9f8).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void CameraTriggerMan_streamIn(self, stream)` (0x16ce30): clean up, parse
/// the trigger list from the stream, clear each trigger's fired flag and
/// refresh the in-game mask.
pub fn camera_trigger_man_reload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::CAMERA_TRIGGER_MAN_UNLOAD.call(c, 0x0016_ce4c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::CAMERA_TRIGGER_MAN_PARSE.call(c, 0x0016_ce58)?;
        c.g[V1][0] = lw_r(c, S1, 0x4);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = Addr(low(c, S1)).lw(c);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx(low(c, A1) << 2);
                c.g[A1][0] = sx(low(c, A1).wrapping_add(1));
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
                c.g[V1][0] = Addr(low(c, V0)).lw(c);
                sw_r(c, V1, 0x20, 0);
                c.g[V0][0] = lw_r(c, S1, 0x4);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(low(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0016_ce68);
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2.call(c, 0x0016_ce94)?;
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `CameraTriggerMan_PopTriggerStack` (0x0016d260).
//
// `int CameraTriggerMan::PopTriggerStack(TriggerStack* s, int key, int* changed,
//  int* out)`: remove `key` from the stack; report whether the top changed.

pub fn camera_trigger_man_pop_trigger_stack(c: &mut Cpu) {
    c.cycles += 3;
    let stack = c.g[A0][0] as u32;
    let key = c.g[A1][0];
    let changed = c.g[A2][0] as u32;
    let out = c.g[A3][0] as u32;
    let mut t1 = lw(c, stack, 0);
    c.g[T1][0] = t1;
    c.g[T0][0] = 0;
    if t1 != 0 {
        c.cycles += 3;
        let mut i: u32 = 0;
        let mut v1 = stack.wrapping_add(8);
        let t2 = v1;
        c.g[V1][0] = sx(v1);
        c.g[T2][0] = sx(t2);
        loop {
            c.cycles += 3;
            let v0 = lw(c, v1, 0);
            if v0 != key {
                t1 = lw(c, stack, 0);
                c.g[T1][0] = t1;
            } else {
                c.cycles += 4;
                let mut j = i.wrapping_add(1);
                c.g[A1][0] = sx(j);
                let in_range = (j as u64) < t1;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = sx(j << 2);
                if in_range {
                    c.cycles += 1;
                    v1 = (j << 2).wrapping_add(t2);
                    c.g[V1][0] = sx(v1);
                    loop {
                        c.cycles += 7;
                        let v = lw(c, v1, 0);
                        j = j.wrapping_add(1);
                        c.g[A1][0] = sx(j);
                        sw(c, v1.wrapping_sub(4), 0, v);
                        c.g[V0][0] = lw(c, stack, 0);
                        c.g[V0][0] = ((j as u64) < c.g[V0][0]) as u64;
                        let more = c.g[V0][0] != 0;
                        v1 = v1.wrapping_add(4);
                        c.g[V1][0] = sx(v1);
                        if !more {
                            break;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0016_d298);
                        }
                    }
                }
                c.g[A1][0] = sx(i.wrapping_add(1));
                c.cycles += 4;
                c.g[V0][0] = lw(c, stack, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                sw(c, stack, 0, c.g[V0][0]);
                break;
            }
            c.cycles += 4;
            c.g[T0][0] = sx(i.wrapping_add(1));
            i = i.wrapping_add(1);
            c.g[V0][0] = ((i as u64) < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(v1.wrapping_add(4));
            v1 = v1.wrapping_add(4);
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0016_d278);
            }
        }
    }
    c.cycles += 3;
    let count = lw(c, stack, 0);
    let v0 = sx((count as u32).wrapping_sub(1));
    if count == 0 {
        c.cycles += 3;
        sw(c, changed, 0, 0);
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    let top = lw(c, stack, 4);
    c.cycles += 6;
    let slot = (v0 as u32) << 2;
    let addr = (stack as u64).wrapping_add(slot as u64);
    let item = lw(c, addr as u32, 8);
    c.g[A0][0] = item;
    c.g[V1][0] = top;
    c.g[V0][0] = 1;
    if item != top {
        c.cycles += 2;
        sw(c, changed, 0, 1);
        sw(c, out, 0, item);
    } else {
        c.cycles += 2;
        sw(c, changed, 0, 0);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Small helpers.

#[inline]
pub(crate) fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// Where a value comes from at an instruction of the original: a register
/// or a word on the frame (`lw(off(sp))`).
#[derive(Clone, Copy)]
pub(crate) enum Src2 {
    Reg(usize),
    Stack(u32),
}

impl Src2 {
    fn get(self, c: &mut Cpu) -> u64 {
        match self {
            Src2::Reg(r) => c.g[r][0],
            Src2::Stack(off) => Addr((c.g[SP][0] as u32).wrapping_add(off)).lw(c),
        }
    }
}

/// What a pass leaves behind when it does not process its list.
#[derive(Clone, Copy)]
pub(crate) enum End {
    /// `a0 = *(sp + 0x44)` (trigger count).
    Count,
    /// `v0 = *(s6 + off)` (next category's flag).
    Field(u32),
    /// `v1 = *(sp + 0x48)` (count * 4).
    Stack48,
}

pub(crate) fn apply_end(c: &mut Cpu, end: End) {
    match end {
        End::Count => c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c),
        End::Field(off) => c.g[V0][0] = Addr(r32(c, S6).wrapping_add(off)).lw(c),
        End::Stack48 => c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c),
    }
}

/// One category pass of `CameraTriggerMan_UpdateForRider`: test the
/// category's spatial key against the rider box, walk the trigger list
/// marking matching nodes (`*(node + 0xc) = marker`), then collect the
/// octree node's eight child trigger lists.
///
/// The original keeps its state in s0..s7/a0..a3/v0/v1; this helper does the
/// same (it is passed the descriptor and reads/writes those registers) so
/// the callees see exactly the registers the generated body leaves for
/// them, and so the event back edge can be delivered at `resume_pc`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn category_pass(
    c: &mut Cpu,
    presence_gate: bool,
    desc: Src2,
    s0: Option<u32>,
    s1: Option<u32>,
    gate_cycles: u64,
    gate_ret: u32,
    box_a0: Src2,
    box_t1: Src2,
    child_rets: &[u32; 8],
    marker_reg: usize,
    type_reg: usize,
    loop_cycles: u64,
    resume_pc: u32,
    end: End,
    box_ret: u32,
    vtable_rets: [u32; 4],
    setup: Option<(usize, u64)>,
    setup_cycles: u64,
) -> Flow {
    // The first category tests the list head itself rather than a flag.
    if presence_gate {
        let d = desc.get(c);
        c.g[S5][0] = Addr((d as u32).wrapping_add(0x10)).lw(c);
        if c.g[S5][0] == 0 {
            c.cycles += 1;
            apply_end(c, end);
            return Ok(());
        }
    }

    c.cycles += gate_cycles;
    if let Some(off) = s0 {
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(off));
    }
    if let Some(off) = s1 {
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(off));
    }
    let d = desc.get(c);
    if !presence_gate {
        c.g[S5][0] = Addr((d as u32).wrapping_add(0x10)).lw(c);
    }
    c.g[A0][0] = d;
    c.g[RA][0] = sx(gate_ret);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    call_ptr(c, callee::CLASSIFY_AABB, gate_ret)?;

    // 1 = outside, 2 = overlapping/partial, 0 = inside.
    c.g[V1][0] = c.g[V0][0];
    let nonzero = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if nonzero {
        c.cycles += 2;
        let differs = c.g[V1][0] != c.g[V0][0];
        apply_end(c, end);
        if !differs {
            c.cycles += 7;
            let a0 = box_a0.get(c);
            let t1 = box_t1.get(c);
            c.g[A0][0] = a0;
            c.g[T0][0] = c.g[S5][0];
            c.g[T1][0] = t1;
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[RA][0] = sx(box_ret);
            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            callee::COLLECT_BOX.call(c, box_ret)?;
            // `break` to the block after the box call: one cycle and the
            // category's end value (A0/V0/V1).
            c.cycles += 1;
            apply_end(c, end);
        }
        return Ok(());
    }

    // The list of trigger objects attached to this octree node.
    c.cycles += 3;
    c.g[S2][0] = Addr(r32(c, S5).wrapping_add(0x28)).lw(c);
    if c.g[S2][0] == 0 {
        c.g[A0][0] = Addr(r32(c, S5)).lw(c);
    } else {
        c.cycles += setup_cycles;
        if let Some((reg, val)) = setup {
            c.g[reg][0] = val;
        }
        if marker_reg == S6 {
            // The last category compares against 4 in s7 and marks with s6.
            c.g[S7][0] = 4;
        }
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
        loop {
            c.cycles += loop_cycles;
            c.g[type_reg][0] = 4;
            if c.g[V0][0] != c.g[type_reg][0] {
                c.g[S2][0] = Addr(r32(c, S2)).lw(c);
            } else {
                c.cycles += 18;
                c.g[S3][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(4)).lw(c);
                c.g[S0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
                c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x6c0)).lw(c);
                c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x28)).lh(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x38)).lh(c);
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x2c)).lw(c);
                c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
                let target = r32(c, V0);
                c.g[RA][0] = sx(vtable_rets[0]);
                c.g[S0][0] = add32(c.g[S0][0], c.g[A1][0]);
                call_ptr(c, target, vtable_rets[0])?;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x3c)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let target = r32(c, V1);
                c.g[RA][0] = sx(vtable_rets[1]);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, vtable_rets[1])?;
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = Addr(r32(c, S2)).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(r32(c, S4)).lw(c);
                    c.g[RA][0] = sx(vtable_rets[2]);
                    c.g[A1][0] = c.g[S3][0];
                    callee::FIND.call(c, vtable_rets[2])?;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = Addr(r32(c, S4)).lw(c);
                        c.cycles += 2;
                        c.g[RA][0] = sx(vtable_rets[3]);
                        c.g[A1][0] = c.g[S3][0];
                        callee::ADD.call(c, vtable_rets[3])?;
                    } else {
                        c.cycles += 2;
                        c.sw(r32(c, V0).wrapping_add(0xc), c.g[marker_reg][0]);
                    }
                    c.cycles += 1;
                    c.g[S2][0] = Addr(r32(c, S2)).lw(c);
                }
            }
            c.cycles += 2;
            if c.g[S2][0] != 0 {
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, resume_pc);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S5)).lw(c);
            break;
        }
    }

    // The node's eight child octree entries.
    for (i, ret) in child_rets.iter().enumerate() {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += if i == 7 { 4 } else { 3 };
            c.g[RA][0] = sx(*ret);
            c.g[A1][0] = c.g[S4][0];
            callee::COLLECT_NODE.call(c, *ret)?;
        }
        if i < 7 {
            c.g[A0][0] = Addr(r32(c, S5).wrapping_add(4 * (i as u32 + 1))).lw(c);
        }
    }
    apply_end(c, end);
    Ok(())
}

// ---------------------------------------------------------------------------
// CameraTriggerMan_UpdateForRider.

/// `void CameraTriggerMan_UpdateForRider(CameraTriggerMan*, Rider*)`
/// (0x16d320): clear the trigger flags for this rider's slot, classify the
/// rider's bounding box against each of the eight camera-trigger categories,
/// mark the triggers that contain it, collect the octree child lists, then
/// pop/apply the triggers the rider just left and push/apply the ones it
/// entered.
pub fn camera_trigger_man_update_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x100);
        frame.save(c, FP, 0x70);
        frame.save(c, S0, 0xf0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save_ra(c, 0x60);
        c.sw(frame.at(c, 0x40), c.g[A0][0]);

        c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x874)).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !skip {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x87c)).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }

        c.cycles += 2;
        let inactive = c.g[V1][0] == 0;
        c.g[S1][0] = sx(r32(c, FP).wrapping_add(0x6c0));
        if inactive {
            c.cycles += 12;
            frame.restore(c, S0, 0xf0);
            frame.restore(c, S1, 0xe0);
            frame.restore(c, S2, 0xd0);
            frame.restore(c, S3, 0xc0);
            frame.restore(c, S4, 0xb0);
            frame.restore(c, S5, 0xa0);
            frame.restore(c, S6, 0x90);
            frame.restore(c, S7, 0x80);
            frame.restore(c, FP, 0x70);
            frame.restore_ra(c, 0x60);
            frame.close(c);
            return Ok(());
        }

        // Clear this rider's flag words: this + 0x14 + count * 4.
        c.cycles += 33;
        c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x870)).lw(c);
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[S0][0] = sx((r32(c, V0)) << 2);
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x14));
        c.g[RA][0] = sx(0x0016_d38c);
        c.g[A0][0] = add32(c.g[V1][0], c.g[A0][0]);
        callee::CLEAR_FLAGS.call(c, 0x0016_d38c)?;

        // CameraTriggerMan from world->game->trigger manager.
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[S0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.g[A1][0] = Addr(r32(c, FP).wrapping_add(0x6c0)).lw(c);
        c.g[S6][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x14));
        c.sw(frame.at(c, 0), c.g[S0][0]);
        c.sw(frame.at(c, 4), c.g[FP][0]);

        // Two virtual calls on the rider's pose object (the two quadwords
        // they return are the rider's bounding box, copied to the frame).
        c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x2c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0016_d3c0);
        c.g[A0][0] = add32(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0016_d3c0)?;
        c.g[V1][0] = Addr(r32(c, FP).wrapping_add(0x6c0)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0016_d3d8);
        c.g[A0][0] = add32(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0016_d3d8)?;
        c.lq(3, r32(c, S0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(4, r32(c, V0));
        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S5][0] = Addr(r32(c, S6).wrapping_add(0x10)).lw(c);
        let no_head = c.g[S5][0] == 0;
        c.g[S4][0] = c.g[SP][0];

        // Category 0: the manager's first list (no flag; the head is the
        // test), its box collected with the manager as the second key.
        if !no_head {
            category_pass(
                c,
                true,
                Src2::Reg(S6),
                Some(0x30),
                None,
                7,
                0x0016_d404,
                Src2::Reg(S0),
                Src2::Reg(S6),
                &[
                    0x0016_d4c0,
                    0x0016_d4d4,
                    0x0016_d4e8,
                    0x0016_d4fc,
                    0x0016_d510,
                    0x0016_d524,
                    0x0016_d538,
                    0x0016_d54c,
                ],
                S7,
                V1,
                3,
                0x0016_d428,
                End::Count,
                0x0016_d578,
                [0x0016_d464, 0x0016_d474, 0x0016_d488, 0x0016_d4a0],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        }

        // Cache the remaining category descriptors and flag fields.
        c.cycles += 16;
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x3c));
        c.g[V0][0] = Addr(r32(c, S6).wrapping_add(0x24)).lw(c);
        c.g[S0][0] = sx(r32(c, S6).wrapping_add(0x14));
        c.g[A0][0] = sx(r32(c, A0) << 2);
        c.sw(frame.at(c, 0x4c), c.g[V1][0]);
        c.sw(frame.at(c, 0x48), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x64));
        c.g[A0][0] = sx(r32(c, S6).wrapping_add(0x50));
        c.sw(frame.at(c, 0x54), c.g[V1][0]);
        c.sw(frame.at(c, 0x50), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x8c));
        c.g[A0][0] = sx(r32(c, S6).wrapping_add(0x78));
        c.g[S7][0] = sx(r32(c, S6).wrapping_add(0x28));
        c.sw(frame.at(c, 0x58), c.g[A0][0]);
        let skip = c.g[V0][0] == 0;
        c.sw(frame.at(c, 0x5c), c.g[V1][0]);
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(S0),
                None,
                Some(0x31),
                8,
                0x0016_d5d0,
                Src2::Reg(S1),
                Src2::Reg(S0),
                &[
                    0x0016_d68c,
                    0x0016_d6a0,
                    0x0016_d6b4,
                    0x0016_d6c8,
                    0x0016_d6dc,
                    0x0016_d6f0,
                    0x0016_d704,
                    0x0016_d718,
                ],
                V1,
                A0,
                3,
                0x0016_d5f0,
                End::Field(0x38),
                0x0016_d744,
                [0x0016_d62c, 0x0016_d63c, 0x0016_d650, 0x0016_d66c],
                None,
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x38));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x32));
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(S7),
                None,
                None,
                7,
                0x0016_d760,
                Src2::Reg(S0),
                Src2::Reg(S7),
                &[
                    0x0016_d81c,
                    0x0016_d830,
                    0x0016_d844,
                    0x0016_d858,
                    0x0016_d86c,
                    0x0016_d880,
                    0x0016_d894,
                    0x0016_d8a4,
                ],
                S7,
                A0,
                3,
                0x0016_d780,
                End::Field(0x4c),
                0x0016_d8d0,
                [0x0016_d7bc, 0x0016_d7cc, 0x0016_d7e0, 0x0016_d7f8],
                Some((S7, 1)),
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x4c));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4c)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(V0),
                Some(0x33),
                None,
                8,
                0x0016_d8f0,
                Src2::Reg(S0),
                Src2::Stack(0x4c),
                &[
                    0x0016_d9a8,
                    0x0016_d9bc,
                    0x0016_d9d0,
                    0x0016_d9e4,
                    0x0016_d9f8,
                    0x0016_da0c,
                    0x0016_da20,
                    0x0016_da34,
                ],
                S7,
                V1,
                3,
                0x0016_d910,
                End::Field(0x60),
                0x0016_da60,
                [0x0016_d94c, 0x0016_d95c, 0x0016_d970, 0x0016_d988],
                Some((S7, 1)),
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x60));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x50)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(A0),
                Some(0x34),
                None,
                7,
                0x0016_da7c,
                Src2::Reg(S0),
                Src2::Stack(0x50),
                &[
                    0x0016_db38,
                    0x0016_db4c,
                    0x0016_db60,
                    0x0016_db74,
                    0x0016_db88,
                    0x0016_db9c,
                    0x0016_dbb0,
                    0x0016_dbc4,
                ],
                S7,
                V1,
                3,
                0x0016_daa0,
                End::Field(0x74),
                0x0016_dbf0,
                [0x0016_dadc, 0x0016_daec, 0x0016_db00, 0x0016_db18],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x74));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x54)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(A0),
                Some(0x35),
                None,
                7,
                0x0016_dc0c,
                Src2::Reg(S0),
                Src2::Stack(0x54),
                &[
                    0x0016_dcc8,
                    0x0016_dcdc,
                    0x0016_dcf0,
                    0x0016_dd04,
                    0x0016_dd18,
                    0x0016_dd2c,
                    0x0016_dd40,
                    0x0016_dd54,
                ],
                S7,
                V1,
                3,
                0x0016_dc30,
                End::Field(0x88),
                0x0016_dd80,
                [0x0016_dc6c, 0x0016_dc7c, 0x0016_dc90, 0x0016_dca8],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x88));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x58)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(A0),
                Some(0x36),
                None,
                7,
                0x0016_dd9c,
                Src2::Reg(S0),
                Src2::Stack(0x58),
                &[
                    0x0016_de58,
                    0x0016_de6c,
                    0x0016_de80,
                    0x0016_de94,
                    0x0016_dea8,
                    0x0016_debc,
                    0x0016_ded0,
                    0x0016_dee4,
                ],
                S7,
                V1,
                3,
                0x0016_ddc0,
                End::Field(0x9c),
                0x0016_df10,
                [0x0016_ddfc, 0x0016_de0c, 0x0016_de20, 0x0016_de38],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x9c));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x5c)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src2::Reg(A0),
                Some(0x37),
                None,
                7,
                0x0016_df2c,
                Src2::Reg(S0),
                Src2::Stack(0x5c),
                &[
                    0x0016_dfe8,
                    0x0016_dff8,
                    0x0016_e00c,
                    0x0016_e020,
                    0x0016_e034,
                    0x0016_e048,
                    0x0016_e05c,
                    0x0016_e070,
                ],
                S6,
                S7,
                2,
                0x0016_df50,
                End::Stack48,
                0x0016_e09c,
                [0x0016_df88, 0x0016_df98, 0x0016_dfac, 0x0016_dfc4],
                Some((S6, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Stack48);
        }

        // Triggers that just became inactive: pop from the stack box, apply
        // the action, or apply the matching trigger's action, then unlink.
        c.cycles += 5;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) << 5);
            c.g[S2][0] = sx(r32(c, A0).wrapping_add(0x14));
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x1c));
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            loop {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
                } else {
                    c.cycles += 9;
                    c.g[S1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c));
                    c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                    c.g[RA][0] = sx(0x0016_e0ec);
                    c.g[A0][0] = add32(c.g[V0][0], c.g[S3][0]);
                    callee::POP_TRIGGER.call(c, 0x0016_e0ec)?;
                    let popped = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x38)).lw(c);
                    if !popped {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[RA][0] = sx(0x0016_e100);
                        c.g[A1][0] = c.g[FP][0];
                        callee::APPLY_ACTION.call(c, 0x0016_e100)?;
                        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    } else {
                        c.cycles += 2;
                        let no_entry = c.g[V0][0] == 0;
                        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                        if !no_entry {
                            c.cycles += 5;
                            c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x3c)).lw(c);
                            c.g[RA][0] = sx(0x0016_e11c);
                            c.g[A0][0] = add32(c.g[V1][0], c.g[S2][0]);
                            callee::FIND_BY_ID.call(c, 0x0016_e11c)?;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                                c.g[RA][0] = sx(0x0016_e130);
                                c.g[A1][0] = c.g[FP][0];
                                callee::APPLY_ACTION.call(c, 0x0016_e130)?;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0016_e140);
                    c.g[A0][0] = add32(c.g[V0][0], c.g[S2][0]);
                    callee::REMOVE.call(c, 0x0016_e140)?;
                    c.g[S0][0] = c.g[V0][0];
                }
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_e0c8);
                    }
                    continue;
                }
                break;
            }
        }

        // Triggers that just became active: push the stack box and apply the
        // trigger's action.
        c.cycles += 6;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[S1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let empty = c.g[S1][0] == 0;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V1) << 5);
            c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x1c));
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            loop {
                c.cycles += 2;
                let no_entry = c.g[V0][0] == 0;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                if !no_entry {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[RA][0] = sx(0x0016_e188);
                    c.g[A0][0] = add32(c.g[V1][0], c.g[S0][0]);
                    callee::PUSH_TRIGGER.call(c, 0x0016_e188)?;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0016_e198);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                    callee::APPLY_ACTION.call(c, 0x0016_e198)?;
                }
                c.cycles += 3;
                c.g[S1][0] = Addr(r32(c, S1).wrapping_add(4)).lw(c);
                if c.g[S1][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_e170);
                    }
                    continue;
                }
                break;
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `__builtin_delete(void*)`.
pub(crate) const OC429_DELETE: u32 = 0x0031_7e50;

/// `void CameraTriggerMan_UpdateTriggers(CameraTriggerMan* self, uint mask)`
/// (0x16cf48): walk the trigger list (`self->+0` array, `+4` count). A
/// trigger whose mask (`+4`) matches `mask` and that is not registered yet
/// (`+0x20 == 0`) registers itself (`+0x20 = trigger`) in the world's
/// spatial structure: the shape's bounds (virtual `+0x30`/`+0x34` of the
/// shape at `+8`) are converted (`0x328f28`) and inserted (`0x328c20`). A
/// registered trigger whose mask no longer matches is removed from it
/// (`0x3284b8`); once the removed node's entries (`+0x10` list, 8 words at
/// `+0..0x1c` then up to 3 more) are all empty the node is freed
/// (`__builtin_delete`, then `+0x10 = 0`), and the trigger is unregistered.
///
/// A dispatcher-shaped function (its generated body is a `match pc` loop):
/// the port handles the normal entry; any other `pc` (a resume after an
/// unwound callee or event) runs the generated body (at install: its
/// `__generated` alias).
pub fn camera_trigger_man_update_triggers(c: &mut Cpu) {
    if c.pc != 0x0016_cf48 {
        return camera_trigger_man_update_triggers_resume(c);
    }
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, 22, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S7, 0x40);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S7][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            c.g[S5][0] = u64::MAX;
            c.g[V0][0] = c.lw(c.g[S6][0] as u32);
            loop {
                // 0x16cf90: trigger i.
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
                c.g[S2][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x4));
                c.g[V0][0] &= c.g[S7][0];
                let no_match = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x20));
                // `true`: fall to 0x16d174 (`cycles += 1; v0 = count`) next;
                // `false`: the insert path already loaded the count.
                let reload_count;
                if !no_match {
                    c.cycles += 2;
                    let registered = c.g[V0][0] != 0;
                    c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                    if registered {
                        reload_count = true;
                    } else {
                        // Register: bounds -> box, insert into the world.
                        c.cycles += 33;
                        c.sw(oc429_at(c.g[S2][0], 0x20), c.g[S2][0]);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A3][0] = c.lw(oc429_at(c.g[S2][0], 0x8));
                        c.g[S1][0] = sx(oc429_at(c.g[S2][0], 0x14));
                        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
                        c.g[V1][0] = c.lw(oc429_at(c.g[A3][0], 0x24));
                        c.g[A2][0] = c.lw(oc429_at(c.g[V0][0], 0x84));
                        c.g[A1][0] = c.lh(oc429_at(c.g[V1][0], 0x30));
                        c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x34));
                        c.g[S0][0] = c.lw(oc429_at(c.g[A2][0], 0x20));
                        let bounds = c.g[V0][0] as u32;
                        c.g[A1][0] = sx(oc429_at(c.g[A3][0], c.g[A1][0] as u32));
                        call_ptr(c, bounds, 0x0016_cfe4)?;
                        c.g[A0][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x0032_8f28, 0x0016_cff0)?;
                        oc429_pick_node(c);
                        c.g[A3][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        c.g[A1][0] = 2;
                        call_ptr(c, 0x0032_8c20, 0x0016_d030)?; // insert
                        c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x4));
                        reload_count = false;
                    }
                } else {
                    c.cycles += 2;
                    let unregistered = c.g[V0][0] == 0;
                    c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                    if unregistered {
                        reload_count = true;
                    } else {
                        // Remove from the world.
                        c.cycles += 39;
                        c.g[A3][0] = c.lw(oc429_at(c.g[S2][0], 0x8));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[V1][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
                        c.g[S1][0] = sx(oc429_at(c.g[S2][0], 0x14));
                        c.g[V0][0] = c.lw(oc429_at(c.g[A3][0], 0x24));
                        c.g[A2][0] = c.lw(oc429_at(c.g[V1][0], 0x84));
                        c.g[A1][0] = c.lh(oc429_at(c.g[V0][0], 0x30));
                        c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x34));
                        c.g[S0][0] = c.lw(oc429_at(c.g[A2][0], 0x20));
                        let bounds = c.g[V1][0] as u32;
                        c.g[A1][0] = sx(oc429_at(c.g[A3][0], c.g[A1][0] as u32));
                        call_ptr(c, bounds, 0x0016_d06c)?;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0xa0));
                        c.g[A0][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        c.g[A1][0] = c.g[SP][0];
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], 1));
                        c.sw(oc429_at(c.g[S0][0], 0xa0), c.g[V0][0]);
                        call_ptr(c, 0x0032_8f28, 0x0016_d084)?;
                        // (The node pick keeps the node in s0 and the id in a2.)
                        c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 0x24));
                        c.g[V1][0] = sx(oc429_at(c.g[S0][0], 0x50));
                        c.g[A0][0] = c.lw(oc429_at(c.g[SP][0], 0x28));
                        c.g[A2][0] = c.g[S1][0];
                        c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.g[T0][0] = c.lw(oc429_at(c.g[SP][0], 0x2c));
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = c.g[S0][0];
                        }
                        c.g[A0][0] = ((c.g[S5][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        c.g[V0][0] = sx(oc429_at(c.g[V1][0], 0x28));
                        c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                        if c.g[A0][0] != 0 {
                            c.g[V0][0] = c.g[V1][0];
                        }
                        c.g[A1][0] = 2;
                        c.g[S0][0] = sx(oc429_at(c.g[V0][0], 0x14));
                        c.g[A3][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        if c.g[T0][0] != 0 {
                            c.g[S0][0] = c.g[V0][0];
                        }
                        c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                        c.g[T0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_84b8, 0x0016_d0cc)?; // remove
                        c.g[V1][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        // Is the node's entry list empty? `found`: a non-empty
                        // word was seen (keep the node).
                        let mut found = false;
                        'probe: {
                            if c.g[V0][0] != 0 {
                                c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                                found = true;
                                break 'probe;
                            }
                            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18] {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], off));
                                if c.g[V0][0] != 0 {
                                    c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                                    found = true;
                                    break 'probe;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x1c));
                            if c.g[V0][0] != 0 {
                                c.cycles += 1;
                                c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                                found = true;
                                break 'probe;
                            }
                            c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0x20));
                            c.cycles += 1;
                            c.g[A0][0] = 0;
                            loop {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                let nonzero = c.g[V0][0] != 0;
                                c.g[A0][0] = sx(oc429_at(c.g[A0][0], 1));
                                if nonzero {
                                    if back_edge_unwinds(c, 0x0016_d134) {
                                        return Ok(());
                                    }
                                    found = true;
                                    break 'probe;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[V1][0] = sx(oc429_at(c.g[V1][0], 4));
                                if more {
                                    if back_edge_unwinds(c, 0x0016_d140) {
                                        return Ok(());
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 1;
                                break 'probe;
                            }
                        }
                        if found {
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                        }
                        // 0x16d15c: free the emptied node.
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                            call_ptr(c, OC429_DELETE, 0x0016_d16c)?;
                            c.sw(oc429_at(c.g[S0][0], 0x10), 0);
                        }
                        c.sw(oc429_at(c.g[S2][0], 0x20), 0);
                        reload_count = true;
                    }
                }
                if reload_count {
                    // 0x16d174
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x4));
                }
                // 0x16d178: next trigger.
                c.cycles += 4;
                c.g[S3][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    if back_edge_unwinds(c, 0x0016_cf90) {
                        return Ok(());
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, 22, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Native resume of `camera_trigger_man_update_triggers` (converted from the `generated` body): any entry
/// other than 0x0016cf48, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn camera_trigger_man_update_triggers_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0016cf48 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = 0u64;
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S6][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S7][0] = c.g[A1][0];
                    if t {
                        pc = 0x0016d188;
                        continue;
                    }
                    pc = 0x0016cf84;
                    continue;
                }
                0x0016cf84 => {
                    c.cycles += 3;
                    c.g[S5][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    pc = 0x0016cf90;
                    continue;
                }
                0x0016cf90 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S2][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x0016d038;
                        continue;
                    }
                    pc = 0x0016cfac;
                    continue;
                }
                0x0016cfac => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0016d174;
                        continue;
                    }
                    pc = 0x0016cfb4;
                    continue;
                }
                0x0016cfb4 => {
                    c.cycles += 33;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x24));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0016cfe4);
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0016cfe4)?;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x0016cff0);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00328f28, 0x0016cff0)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A1][0] != 0 {
                        c.g[A0][0] = c.g[V1][0];
                    }
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0016d030);
                    if c.g[T0][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    call_ptr(c, 0x00328c20, 0x0016d030)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0016d178;
                        continue;
                    }
                    pc = 0x0016d038;
                    continue;
                }
                0x0016d038 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0016d174;
                        continue;
                    }
                    pc = 0x0016d040;
                    continue;
                }
                0x0016d040 => {
                    c.cycles += 39;
                    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x24));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0016d06c);
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0016d06c)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x0016d084);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x0016d084)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S5][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0016d0cc);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0016d0cc)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d0dc;
                    continue;
                }
                0x0016d0dc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d0e8;
                    continue;
                }
                0x0016d0e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d0f4;
                    continue;
                }
                0x0016d0f4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d100;
                    continue;
                }
                0x0016d100 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d10c;
                    continue;
                }
                0x0016d10c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d118;
                    continue;
                }
                0x0016d118 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d124;
                    continue;
                }
                0x0016d124 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        pc = 0x0016d13c;
                        continue;
                    }
                    pc = 0x0016d130;
                    continue;
                }
                0x0016d130 => {
                    c.cycles += 1;
                    c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x0016d134;
                    continue;
                }
                0x0016d134 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016d15c;
                        continue;
                    }
                    pc = 0x0016d13c;
                    continue;
                }
                0x0016d13c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x0016d140;
                    continue;
                }
                0x0016d140 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0016d134) {
                            return Ok(());
                        }
                        pc = 0x0016d134;
                        continue;
                    }
                    pc = 0x0016d14c;
                    continue;
                }
                0x0016d14c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0016d140) {
                            return Ok(());
                        }
                        pc = 0x0016d140;
                        continue;
                    }
                    pc = 0x0016d158;
                    continue;
                }
                0x0016d158 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0016d15c;
                    continue;
                }
                0x0016d15c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x20), 0u64);
                        pc = 0x0016d174;
                        continue;
                    }
                    pc = 0x0016d164;
                    continue;
                }
                0x0016d164 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0016d16c);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0016d16c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x20), 0u64);
                    pc = 0x0016d174;
                    continue;
                }
                0x0016d174 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                    pc = 0x0016d178;
                    continue;
                }
                0x0016d178 => {
                    c.cycles += 4;
                    c.g[S3][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                        if back_edge_unwinds(c, 0x0016cf90) {
                            return Ok(());
                        }
                        pc = 0x0016cf90;
                        continue;
                    }
                    pc = 0x0016d188;
                    continue;
                }
                0x0016d188 => {
                    c.cycles += 11;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016cf48, 0x0016d1b4);
                    return Ok(());
                }
            }
        }
    });
}

/// The register shuffle both paths of `CameraTriggerMan_UpdateTriggers` do
/// after the box conversion (`0x328f28`) of the "register" path: the box
/// extents in `sp + 0x24..0x2c` select the nodes `s0 + 0x50`, `s0` or
/// `+ 0x28`; the chosen node + 0x14 (or the node itself) ends in `a0`, the
/// trigger's `+0x14` in `a2` (conditional moves, as compiled).
pub(crate) fn oc429_pick_node(c: &mut Cpu) {
    c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 0x24));
    c.g[V1][0] = sx(oc429_at(c.g[S0][0], 0x50));
    c.g[A1][0] = c.lw(oc429_at(c.g[SP][0], 0x28));
    c.g[A2][0] = c.g[S1][0];
    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[T0][0] = c.lw(oc429_at(c.g[SP][0], 0x2c));
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[S0][0];
    }
    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(oc429_at(c.g[V1][0], 0x28));
    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
    if c.g[A1][0] != 0 {
        c.g[A0][0] = c.g[V1][0];
    }
    c.g[A3][0] = sx(oc429_at(c.g[SP][0], 0x20));
    c.g[V0][0] = sx(oc429_at(c.g[A0][0], 0x14));
    if c.g[T0][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
}

/// `void CameraTriggerMan_RemoveAllTriggers(CameraTriggerMan* self)` (0x0016cbe0): removes every registered trigger from the spatial hash (bounds through the shape vtable, `SpatialKey_FromAABB`, `SpatialHash_Remove`; frees the cell when all its 11 words are empty), then resets the two trigger stacks, the trigger manager, both active lists and the chase controller switches. Dispatcher-shaped like the original (resumable at its block heads after an event unwind).
pub fn camera_trigger_man_remove_all_triggers(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0016cbe0 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x0016cd74;
                        continue;
                    }
                    pc = 0x0016cc10;
                    continue;
                }
                0x0016cc10 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x0016cc18;
                    continue;
                }
                0x0016cc18 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0016cd60;
                        continue;
                    }
                    pc = 0x0016cc30;
                    continue;
                }
                0x0016cc30 => {
                    c.cycles += 39;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x24));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0016cc5c)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x0016cc74)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S5][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0016ccbc)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cccc;
                    continue;
                }
                0x0016cccc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016ccd8;
                    continue;
                }
                0x0016ccd8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cce4;
                    continue;
                }
                0x0016cce4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016ccf0;
                    continue;
                }
                0x0016ccf0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016ccfc;
                    continue;
                }
                0x0016ccfc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cd08;
                    continue;
                }
                0x0016cd08 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cd14;
                    continue;
                }
                0x0016cd14 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        pc = 0x0016cd2c;
                        continue;
                    }
                    pc = 0x0016cd20;
                    continue;
                }
                0x0016cd20 => {
                    c.cycles += 1;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x0016cd24;
                    continue;
                }
                0x0016cd24 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016cd4c;
                        continue;
                    }
                    pc = 0x0016cd2c;
                    continue;
                }
                0x0016cd2c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x0016cd30;
                    continue;
                }
                0x0016cd30 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0016cd24) {
                            return Ok(());
                        }
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cd3c;
                    continue;
                }
                0x0016cd3c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0016cd30) {
                            return Ok(());
                        }
                        pc = 0x0016cd30;
                        continue;
                    }
                    pc = 0x0016cd48;
                    continue;
                }
                0x0016cd48 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0016cd4c;
                    continue;
                }
                0x0016cd4c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                        pc = 0x0016cd64;
                        continue;
                    }
                    pc = 0x0016cd54;
                    continue;
                }
                0x0016cd54 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0016cd5c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    pc = 0x0016cd60;
                    continue;
                }
                0x0016cd60 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    pc = 0x0016cd64;
                    continue;
                }
                0x0016cd64 => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                        if back_edge_unwinds(c, 0x0016cc18) {
                            return Ok(());
                        }
                        pc = 0x0016cc18;
                        continue;
                    }
                    pc = 0x0016cd74;
                    continue;
                }
                0x0016cd74 => {
                    c.cycles += 21;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cd7c)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cd84)?;
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0016c098,
                        f: ee::s02::CameraTriggerMan_Destroy,
                    }
                    .call(c, 0x0016cd8c)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                    Callee {
                        addr: 0x0016c628,
                        f: ee::s02::ActiveCameraTriggerList_Clear,
                    }
                    .call(c, 0x0016cd94)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x18));
                    Callee {
                        addr: 0x0016c628,
                        f: ee::s02::ActiveCameraTriggerList_Clear,
                    }
                    .call(c, 0x0016cd9c)?;
                    call_ptr(c, 0x0015dc18, 0x0016cda4)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016cbe0, 0x0016cdc8);
                    return Ok(());
                }
            }
        }
    })
}

/// `CameraTriggerMan.triggers` (array of trigger pointers at +0) and
/// `.count` (+4); a trigger's mask at +0x20.
pub(crate) const OC436_CTM_TRIGGERS: u32 = 0x0;

pub(crate) const OC436_CTM_COUNT: u32 = 0x4;

pub(crate) const OC436_TRIGGER_MASK: u32 = 0x20;

/// `void CameraTriggerMan_Reload(CameraTriggerMan* self, data)` (0x16ce30):
/// `Unload(self, data)`, `Parse(self, data)`, clear the mask (+0x20) of every
/// trigger in the new array, then `UpdateTriggersMask2(self)`.
pub fn oc436_camera_trigger_man_reload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0016_c9b0, 0x0016_ce4c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0016_c228, 0x0016_ce58)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_COUNT));
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_TRIGGERS));
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw((c.g[V1][0] as u32).wrapping_add(OC436_TRIGGER_MASK), 0);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_COUNT));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_TRIGGERS));
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_ce68);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0016_d1b8, 0x0016_ce94)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerStack_Clear(stack)` (0x16d200): zero +0 and +4.
pub fn oc436_camera_trigger_stack_clear(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A0][0] as u32).wrapping_add(4), 0);
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, 0);
    c.pc = t;
}

/// `void CameraTriggerMan_ReloadIf_Global(void* file, int flag)` (0x16cec8):
/// `CameraTriggerMan_ReloadIf(&gCameraTriggerMan (0x4c5830), file, flag)`.
pub fn camera_trigger_man_reload_if_global(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A2][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x4c_0000;
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5830));
        RELOAD_IF.call(c, 0x0016_cee4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan_ReloadIf` (0x16cef0).
pub(crate) const RELOAD_IF: Callee = Callee {
    addr: 0x0016_cef0,
    f: ee::s02::CameraTriggerMan_ReloadIf,
};

/// `void CameraTriggerMan_ReloadIf(CameraTriggerMan* man, void* file, int flag)`
/// (0x16cef0): `CameraTriggerMan_Reload(man, flag)` when `flag != 0`.
pub fn camera_trigger_man_reload_if(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        let skip = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !skip {
            c.cycles += 2;
            Callee {
                addr: 0x0016_ce30,
                f: ee::s02::CameraTriggerMan_Reload,
            }
            .call(c, 0x0016_cf08)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_UpdateTriggersMask2(CameraTriggerMan* man)`
/// (0x16d1b8): `CameraTriggerMan_UpdateTriggers(man, 2)`.
pub fn camera_trigger_man_update_triggers_mask2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2;
        Callee {
            addr: 0x0016_cf48,
            f: ee::s02::CameraTriggerMan_UpdateTriggers,
        }
        .call(c, 0x0016_d1c8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_EmptyHook()` (0x16ca10): does nothing.
pub fn camera_trigger_man_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraBoxBoundObj_ClampPoint(box *t, vec4 *point)` (0x16f068): clamps
/// `*point` into the camera trigger box `t`. The box holds `pos` (+0x00),
/// `rot[3]` (+0x0c, angles about Z, Y, X in the order rot[0], rot[2], rot[1]
/// for the inverse) and `size` (+0x18, half extents). The point is taken
/// into the box's frame (inverse rotation, then minus `pos`), clipped against
/// the three axis slabs (plane normals cached at 0x4c5450 on the first call,
/// flag at gp+0x1cf8), and taken back out with the forward rotation.
/// Every stack slot, VU0 and FPU step follows the generated body, so the
/// stack scratch and the register state at return match it too.
pub fn camera_box_bound_obj_clamp_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 347;
        let frame = Frame::open(c, 0x1d0);
        'b_16f5fc: {
            c.g[V1][0] = 0x500000;
            c.sq(22, frame.at(c, 0x150));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe60));
            c.g[S6][0] = c.g[A0][0];
            c.sw(frame.at(c, 0x110), c.g[A1][0]);
            c.sq(16, frame.at(c, 0x1b0));
            c.sq(17, frame.at(c, 0x1a0));
            c.sq(18, frame.at(c, 0x190));
            c.sq(19, frame.at(c, 0x180));
            c.sq(20, frame.at(c, 0x170));
            c.sq(21, frame.at(c, 0x160));
            c.sq(23, frame.at(c, 0x140));
            c.sq(30, frame.at(c, 0x130));
            c.sd(frame.at(c, 0x120), c.g[RA][0]);
            c.swc1(20, frame.at(c, 0x1c0));
            c.lqc2(1, c.g[V0][0] as u32);
            c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
            c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sqc2(1, frame.at(c, 0));
            c.sqc2(2, frame.at(c, 0x10));
            c.sqc2(3, frame.at(c, 0x20));
            c.sqc2(4, frame.at(c, 0x30));
            c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = c.g[SP][0];
            c.g[A0][0] = sx(frame.at(c, 0xe0));
            c.g[A1][0] = sx(frame.at(c, 0xe4));
            c.neg_s(12, 12);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.g[S3][0] = 0x500000;
            c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xea0));
            call_ptr(c, 0x0031_be50, 0x0016f0f0)?;
            c.lwc1(11, frame.at(c, 0xe4));
            c.g[S2][0] = sx(frame.at(c, 0x40));
            c.lwc1(3, (c.g[S5][0] as u32).wrapping_add(0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, (c.g[S3][0] as u32).wrapping_sub(0xea0));
            c.lwc1(6, (c.g[S5][0] as u32).wrapping_add(0x4));
            c.lwc1(8, frame.at(c, 0xe0));
            c.mul_s(2, 1, 3);
            c.sw(frame.at(c, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(frame.at(c, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(frame.at(c, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(frame.at(c, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(frame.at(c, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(frame.at(c, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, frame.at(c, 0x7c));
            c.mul_s(5, 2, 0);
            c.mul_s(7, 4, 0);
            c.mul_s(3, 1, 3);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(4, 4, 6);
            c.mul_s(1, 1, 6);
            c.add_s(5, 5, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(5, frame.at(c, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, frame.at(c, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, frame.at(c, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, frame.at(c, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, frame.at(c, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, frame.at(c, 0x50));
            c.swc1(10, frame.at(c, 0x60));
            c.swc1(1, frame.at(c, 0x54));
            c.swc1(3, frame.at(c, 0x64));
            c.lqc2(4, c.g[S0][0] as u32);
            c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
            c.lqc2(8, c.g[S2][0] as u32);
            c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
            c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
            c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.vu0().mula(0b1111, 4, Src::Bc(9, X));
            c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
            c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
            c.vu0().mula(0b1111, 4, Src::Bc(10, X));
            c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
            c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
            c.vu0().mula(0b1111, 4, Src::Bc(11, X));
            c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
            c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
            c.sqc2(12, c.g[S0][0] as u32);
            c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
            c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
            c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0x14));
            c.g[S0][0] = c.g[SP][0];
            c.g[A0][0] = sx(frame.at(c, 0xe8));
            c.g[A1][0] = sx(frame.at(c, 0xec));
            c.neg_s(12, 12);
            c.g[S1][0] = 0x500000;
            c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xeb0));
            call_ptr(c, 0x0031_be50, 0x0016f23c)?;
            c.lwc1(11, frame.at(c, 0xec));
            c.lwc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, (c.g[S1][0] as u32).wrapping_sub(0xeb0));
            c.lwc1(6, (c.g[FP][0] as u32).wrapping_add(0x4));
            c.lwc1(8, frame.at(c, 0xe8));
            c.mul_s(2, 1, 3);
            c.sw(frame.at(c, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(frame.at(c, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(frame.at(c, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(frame.at(c, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(frame.at(c, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(frame.at(c, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, frame.at(c, 0x7c));
            c.mul_s(5, 2, 0);
            c.mul_s(7, 4, 0);
            c.mul_s(3, 1, 3);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(4, 4, 6);
            c.mul_s(1, 1, 6);
            c.add_s(5, 5, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(5, frame.at(c, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, frame.at(c, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, frame.at(c, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, frame.at(c, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, frame.at(c, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, frame.at(c, 0x50));
            c.swc1(10, frame.at(c, 0x60));
            c.swc1(1, frame.at(c, 0x54));
            c.swc1(3, frame.at(c, 0x64));
            c.lqc2(4, c.g[S0][0] as u32);
            c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
            c.lqc2(8, c.g[S2][0] as u32);
            c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
            c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
            c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.vu0().mula(0b1111, 4, Src::Bc(9, X));
            c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
            c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
            c.vu0().mula(0b1111, 4, Src::Bc(10, X));
            c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
            c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
            c.vu0().mula(0b1111, 4, Src::Bc(11, X));
            c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
            c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
            c.sqc2(12, c.g[S0][0] as u32);
            c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
            c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
            c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[S4][0] = c.g[SP][0];
            c.g[A0][0] = sx(frame.at(c, 0xf0));
            c.g[A1][0] = sx(frame.at(c, 0xf4));
            c.neg_s(12, 12);
            c.g[S0][0] = 0x500000;
            c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
            call_ptr(c, 0x0031_be50, 0x0016f384)?;
            c.lwc1(11, frame.at(c, 0xf4));
            c.lwc1(3, (c.g[S7][0] as u32).wrapping_add(0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xec0));
            c.lwc1(6, (c.g[S7][0] as u32).wrapping_add(0x4));
            c.lwc1(8, frame.at(c, 0xf0));
            c.mul_s(2, 1, 3);
            c.sw(frame.at(c, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(frame.at(c, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(frame.at(c, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(frame.at(c, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(frame.at(c, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(frame.at(c, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, frame.at(c, 0x7c));
            c.mul_s(5, 2, 0);
            c.mul_s(7, 4, 0);
            c.mul_s(3, 1, 3);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(4, 4, 6);
            c.mul_s(1, 1, 6);
            c.add_s(5, 5, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(5, frame.at(c, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, frame.at(c, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, frame.at(c, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, frame.at(c, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, frame.at(c, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, frame.at(c, 0x50));
            c.swc1(10, frame.at(c, 0x60));
            c.swc1(1, frame.at(c, 0x54));
            c.swc1(3, frame.at(c, 0x64));
            c.lqc2(4, c.g[S4][0] as u32);
            c.lqc2(5, (c.g[S4][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[S4][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[S4][0] as u32).wrapping_add(0x30));
            c.lqc2(8, c.g[S2][0] as u32);
            c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
            c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
            c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.vu0().mula(0b1111, 4, Src::Bc(9, X));
            c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
            c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
            c.vu0().mula(0b1111, 4, Src::Bc(10, X));
            c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
            c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
            c.vu0().mula(0b1111, 4, Src::Bc(11, X));
            c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
            c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
            c.sqc2(12, c.g[S4][0] as u32);
            c.sqc2(13, (c.g[S4][0] as u32).wrapping_add(0x10));
            c.sqc2(14, (c.g[S4][0] as u32).wrapping_add(0x20));
            c.sqc2(15, (c.g[S4][0] as u32).wrapping_add(0x30));
            c.lwc1(0, c.g[S6][0] as u32);
            c.lwc1(1, (c.g[S6][0] as u32).wrapping_add(0x4));
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x8));
            c.neg_s(0, 0);
            c.neg_s(1, 1);
            c.swc1(20, frame.at(c, 0x4c));
            c.neg_s(2, 2);
            c.swc1(0, frame.at(c, 0x40));
            c.swc1(1, frame.at(c, 0x44));
            c.swc1(2, frame.at(c, 0x48));
            c.swc1(0, frame.at(c, 0x50));
            c.swc1(1, frame.at(c, 0x54));
            c.swc1(2, frame.at(c, 0x58));
            c.lqc2(4, frame.at(c, 0));
            c.lqc2(5, frame.at(c, 0x10));
            c.lqc2(6, frame.at(c, 0x20));
            c.lqc2(7, frame.at(c, 0x30));
            c.lqc2(8, frame.at(c, 0x40));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, frame.at(c, 0x30));
            c.g[V0][0] = c.lw(frame.at(c, 0x110));
            c.g[T0][0] = 0x500000;
            c.lqc2(8, c.g[V0][0] as u32);
            c.lqc2(4, frame.at(c, 0));
            c.lqc2(5, frame.at(c, 0x10));
            c.lqc2(6, frame.at(c, 0x20));
            c.lqc2(7, frame.at(c, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, frame.at(c, 0x50));
            c.g[S1][0] = c.g[S0][0];
            c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x18));
            c.lwc1(1, (c.g[S6][0] as u32).wrapping_add(0x1c));
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x20));
            c.neg_s(0, 0);
            c.lq(2, frame.at(c, 0x50));
            c.neg_s(1, 1);
            c.neg_s(2, 2);
            c.swc1(20, frame.at(c, 0x6c));
            c.sq(2, frame.at(c, 0x40));
            c.swc1(0, frame.at(c, 0x70));
            c.swc1(1, frame.at(c, 0x74));
            c.swc1(2, frame.at(c, 0x78));
            c.swc1(0, frame.at(c, 0x60));
            c.swc1(1, frame.at(c, 0x64));
            c.swc1(2, frame.at(c, 0x68));
            c.swc1(20, frame.at(c, 0x4c));
            c.lqc2(3, frame.at(c, 0x40));
            c.lqc2(4, frame.at(c, 0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x80));
            c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x18));
            c.lwc1(1, (c.g[S6][0] as u32).wrapping_add(0x1c));
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x20));
            c.lq(2, frame.at(c, 0x80));
            c.swc1(0, frame.at(c, 0x70));
            c.sq(2, frame.at(c, 0x50));
            c.swc1(1, frame.at(c, 0x74));
            c.swc1(2, frame.at(c, 0x78));
            c.swc1(20, frame.at(c, 0x7c));
            c.lqc2(3, frame.at(c, 0x40));
            c.lqc2(4, frame.at(c, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x80));
            c.lq(2, frame.at(c, 0x80));
            c.sq(2, frame.at(c, 0x60));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1cf8));
            let t = c.g[V1][0] != 0u64;
            c.g[V0][0] = 0x4c0000;
            if t {
                break 'b_16f5fc;
            } else {
                c.cycles += 10;
                c.lq(4, c.g[S7][0] as u32);
                c.lq(5, c.g[FP][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5450));
                c.lq(3, c.g[S5][0] as u32);
                c.g[A2][0] = 0x1;
                c.sq(4, c.g[V0][0] as u32);
                c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.sq(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1cf8), c.g[A2][0]);
                c.g[V0][0] = 0x4c0000;
                break 'b_16f5fc;
            }
        }
        c.cycles += 13;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 2);
        c.mtc1(0, 1);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5450));
        c.g[A0][0] = sx(frame.at(c, 0xf8));
        c.g[A1][0] = sx(frame.at(c, 0xfc));
        c.g[S2][0] = sx(frame.at(c, 0xa0));
        c.g[S4][0] = sx(frame.at(c, 0x100));
        c.g[S7][0] = sx(frame.at(c, 0x104));
        c.g[FP][0] = sx(frame.at(c, 0x70));
        c.g[S3][0] = sx(frame.at(c, 0x108));
        c.g[S5][0] = sx(frame.at(c, 0x10c));
        c.g[A3][0] = 0x2;
        'l_16f630: loop {
            'b_16f6a8: {
                c.cycles += 19;
                c.mfc1(2, 2);
                c.lqc2(4, c.g[A2][0] as u32);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(3, frame.at(c, 0x80));
                c.sq(3, frame.at(c, 0x70));
                c.lqc2(3, frame.at(c, 0x70));
                c.lqc2(5, frame.at(c, 0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                if t {
                    break 'b_16f6a8;
                } else {
                    c.cycles += 11;
                    c.mfc1(3, 0);
                    c.lqc2(4, frame.at(c, 0x70));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x90));
                    c.lq(2, frame.at(c, 0x90));
                    c.sq(2, frame.at(c, 0x80));
                    c.lqc2(3, frame.at(c, 0x40));
                    c.lqc2(4, frame.at(c, 0x80));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x40));
                    break 'b_16f6a8;
                }
            }
            'b_16f70c: {
                c.cycles += 13;
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(5, frame.at(c, 0x60));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    break 'b_16f70c;
                } else {
                    c.cycles += 12;
                    c.mfc1(3, 0);
                    c.lqc2(4, c.g[A2][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x90));
                    c.lq(2, frame.at(c, 0x90));
                    c.sq(2, frame.at(c, 0x80));
                    c.lqc2(3, frame.at(c, 0x40));
                    c.lqc2(4, frame.at(c, 0x80));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x40));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    break 'b_16f70c;
                }
            }
            c.cycles += 2;
            let t = (c.g[A3][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0016f630);
                }
                continue 'l_16f630;
            } else {
                c.cycles += 296;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0xe60));
                c.lqc2(1, c.g[V0][0] as u32);
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, frame.at(c, 0));
                c.sqc2(2, frame.at(c, 0x10));
                c.sqc2(3, frame.at(c, 0x20));
                c.sqc2(4, frame.at(c, 0x30));
                c.lwc1(0, c.g[S6][0] as u32);
                c.lwc1(1, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.swc1(0, frame.at(c, 0x70));
                c.swc1(1, frame.at(c, 0x74));
                c.swc1(2, frame.at(c, 0x78));
                c.swc1(20, frame.at(c, 0x7c));
                c.lqc2(4, frame.at(c, 0));
                c.lqc2(5, frame.at(c, 0x10));
                c.lqc2(6, frame.at(c, 0x20));
                c.lqc2(7, frame.at(c, 0x30));
                c.lqc2(8, frame.at(c, 0x70));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, frame.at(c, 0x30));
                c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_be50, 0x0016f790)?;
                c.lwc1(11, frame.at(c, 0xfc));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xec0));
                c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sub_s(1, 20, 11);
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_sub(0xec0));
                c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.lwc1(8, frame.at(c, 0xf8));
                c.mul_s(2, 1, 3);
                c.sw(frame.at(c, 0xd0), 0u64);
                c.mul_s(4, 1, 0);
                c.sw(frame.at(c, 0xd4), 0u64);
                c.mul_s(1, 1, 6);
                c.sw(frame.at(c, 0xd8), 0u64);
                c.mul_s(13, 8, 3);
                c.sw(frame.at(c, 0xac), 0u64);
                c.mul_s(12, 8, 0);
                c.sw(frame.at(c, 0xbc), 0u64);
                c.mul_s(9, 2, 3);
                c.sw(frame.at(c, 0xcc), 0u64);
                c.mul_s(10, 4, 3);
                c.swc1(20, frame.at(c, 0xdc));
                c.mul_s(5, 2, 0);
                c.mul_s(7, 4, 0);
                c.mul_s(3, 1, 3);
                c.mul_s(0, 1, 0);
                c.mul_s(8, 8, 6);
                c.mul_s(2, 2, 6);
                c.mul_s(4, 4, 6);
                c.mul_s(1, 1, 6);
                c.add_s(5, 5, 8);
                c.sub_s(0, 0, 13);
                c.sub_s(2, 2, 12);
                c.add_s(9, 9, 11);
                c.swc1(5, frame.at(c, 0xa8));
                c.add_s(7, 7, 11);
                c.swc1(0, frame.at(c, 0xa4));
                c.add_s(4, 4, 13);
                c.swc1(2, frame.at(c, 0xb8));
                c.sub_s(10, 10, 8);
                c.swc1(9, frame.at(c, 0xc8));
                c.add_s(1, 1, 11);
                c.swc1(7, frame.at(c, 0xa0));
                c.add_s(3, 3, 12);
                c.swc1(4, frame.at(c, 0xb0));
                c.swc1(10, frame.at(c, 0xc0));
                c.swc1(1, frame.at(c, 0xb4));
                c.swc1(3, frame.at(c, 0xc4));
                c.lqc2(4, c.g[S0][0] as u32);
                c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, c.g[S2][0] as u32);
                c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, c.g[S0][0] as u32);
                c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S7][0];
                c.g[S0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_be50, 0x0016f8d0)?;
                c.lwc1(11, frame.at(c, 0x104));
                c.g[V1][0] = 0x500000;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_sub(0xeb0));
                c.sub_s(1, 20, 11);
                c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.lwc1(8, frame.at(c, 0x100));
                c.mul_s(2, 1, 3);
                c.sw(frame.at(c, 0xa0), 0u64);
                c.mul_s(4, 1, 0);
                c.sw(frame.at(c, 0xa4), 0u64);
                c.mul_s(1, 1, 6);
                c.sw(frame.at(c, 0xa8), 0u64);
                c.mul_s(13, 8, 3);
                c.sw(frame.at(c, 0x7c), 0u64);
                c.mul_s(12, 8, 0);
                c.sw(frame.at(c, 0x8c), 0u64);
                c.mul_s(9, 2, 3);
                c.sw(frame.at(c, 0x9c), 0u64);
                c.mul_s(10, 4, 3);
                c.swc1(20, frame.at(c, 0xac));
                c.mul_s(5, 2, 0);
                c.mul_s(7, 4, 0);
                c.mul_s(3, 1, 3);
                c.mul_s(0, 1, 0);
                c.mul_s(8, 8, 6);
                c.mul_s(2, 2, 6);
                c.mul_s(4, 4, 6);
                c.mul_s(1, 1, 6);
                c.add_s(5, 5, 8);
                c.sub_s(0, 0, 13);
                c.sub_s(2, 2, 12);
                c.add_s(9, 9, 11);
                c.swc1(5, frame.at(c, 0x78));
                c.add_s(7, 7, 11);
                c.swc1(0, frame.at(c, 0x74));
                c.add_s(4, 4, 13);
                c.swc1(2, frame.at(c, 0x88));
                c.sub_s(10, 10, 8);
                c.swc1(9, frame.at(c, 0x98));
                c.add_s(1, 1, 11);
                c.swc1(7, frame.at(c, 0x70));
                c.add_s(3, 3, 12);
                c.swc1(4, frame.at(c, 0x80));
                c.swc1(10, frame.at(c, 0x90));
                c.swc1(1, frame.at(c, 0x84));
                c.swc1(3, frame.at(c, 0x94));
                c.lqc2(4, c.g[S0][0] as u32);
                c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, c.g[FP][0] as u32);
                c.lqc2(9, (c.g[FP][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[FP][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[FP][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, c.g[S0][0] as u32);
                c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[S0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_be50, 0x0016fa14)?;
                c.lwc1(11, frame.at(c, 0x10c));
                c.g[V1][0] = 0x500000;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_sub(0xea0));
                c.sub_s(1, 20, 11);
                c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.lwc1(8, frame.at(c, 0x108));
                c.mul_s(2, 1, 3);
                c.sw(frame.at(c, 0xa0), 0u64);
                c.mul_s(4, 1, 0);
                c.sw(frame.at(c, 0xa4), 0u64);
                c.mul_s(1, 1, 6);
                c.sw(frame.at(c, 0xa8), 0u64);
                c.mul_s(13, 8, 3);
                c.sw(frame.at(c, 0x7c), 0u64);
                c.mul_s(12, 8, 0);
                c.sw(frame.at(c, 0x8c), 0u64);
                c.mul_s(9, 2, 3);
                c.sw(frame.at(c, 0x9c), 0u64);
                c.mul_s(10, 4, 3);
                c.swc1(20, frame.at(c, 0xac));
                c.mul_s(5, 2, 0);
                c.mul_s(7, 4, 0);
                c.mul_s(3, 1, 3);
                c.mul_s(0, 1, 0);
                c.mul_s(8, 8, 6);
                c.mul_s(2, 2, 6);
                c.mul_s(4, 4, 6);
                c.mul_s(1, 1, 6);
                c.add_s(5, 5, 8);
                c.sub_s(0, 0, 13);
                c.sub_s(2, 2, 12);
                c.add_s(9, 9, 11);
                c.swc1(5, frame.at(c, 0x78));
                c.add_s(7, 7, 11);
                c.swc1(0, frame.at(c, 0x74));
                c.add_s(4, 4, 13);
                c.swc1(2, frame.at(c, 0x88));
                c.sub_s(10, 10, 8);
                c.swc1(9, frame.at(c, 0x98));
                c.add_s(1, 1, 11);
                c.swc1(7, frame.at(c, 0x70));
                c.add_s(3, 3, 12);
                c.swc1(4, frame.at(c, 0x80));
                c.swc1(10, frame.at(c, 0x90));
                c.swc1(1, frame.at(c, 0x84));
                c.swc1(3, frame.at(c, 0x94));
                c.lqc2(4, c.g[S0][0] as u32);
                c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, c.g[FP][0] as u32);
                c.lqc2(9, (c.g[FP][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[FP][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[FP][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, c.g[S0][0] as u32);
                c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0));
                c.lqc2(5, frame.at(c, 0x10));
                c.lqc2(6, frame.at(c, 0x20));
                c.lqc2(7, frame.at(c, 0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, frame.at(c, 0x80));
                c.lq(16, frame.at(c, 0x1b0));
                c.lq(2, frame.at(c, 0x80));
                c.g[V1][0] = c.lw(frame.at(c, 0x110));
                c.lq(17, frame.at(c, 0x1a0));
                c.sq(2, c.g[V1][0] as u32);
                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.lq(18, frame.at(c, 0x190));
                c.lq(19, frame.at(c, 0x180));
                c.lq(20, frame.at(c, 0x170));
                c.lq(21, frame.at(c, 0x160));
                c.lq(22, frame.at(c, 0x150));
                c.lq(23, frame.at(c, 0x140));
                c.lq(30, frame.at(c, 0x130));
                c.g[RA][0] = c.ld(frame.at(c, 0x120));
                c.lwc1(20, frame.at(c, 0x1c0));
                c.sq(2, frame.at(c, 0x70));
                frame.close(c);
                return Ok(());
            }
        }
    });
}

/// `CameraTriggerMan_LoadFile`.
pub(crate) const CAMERA_TRIGGER_MAN_LOAD_FILE: u32 = 0x0016_c170;

/// `CameraTriggerMan::Load(this)` (0x16cdc8): loads the trigger file; when
/// that returns 0, clears the `+0x20` word of each trigger region in the
/// list at `this + 0` (`count` at `this + 4`), and returns 0.
pub fn camera_trigger_man_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: CAMERA_TRIGGER_MAN_LOAD_FILE,
            f: ee::s02::f_0016c170,
        }
        .call(c, 0x0016_cddc)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                loop {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    Addr(r32(c, V1).wrapping_add(0x20)).sw(c, 0);
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_cdf8);
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn ecp_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`).
#[inline(always)]
pub(crate) fn ecp_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `void CameraEllipseBoundObj_ClampPoint(cCTBoundObjEllipse* src, Vec4* point)`
/// (0x16e560): a point outside the ellipsoid `src` is moved onto its surface.
/// The point is mapped into the ellipsoid's local frame (inverse scale,
/// rotations from `Math_SinCos`, then the translation by `-Pos`, all as VU0
/// and FPU operations on the stack copies of the matrices). If that local
/// point lies inside the unit sphere, nothing changes. Otherwise it is
/// projected onto the unit sphere, mapped back with the forward transform
/// (scale, rotations, translation), and `point->w` is set to 1.
pub fn camera_ellipse_bound_obj_clamp_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 361;
        c.g[SP][0] = sx(ecp_back(c, SP, 0x1f0));
        c.g[V0][0] = 0x500000;
        c.sq(18, ecp_at(c, SP, 0x1b0));
        c.g[V0][0] = sx(ecp_back(c, V0, 0xe60));
        c.sq(23, ecp_at(c, SP, 0x160));
        // this and the point: s2 and s7 (the callee-saved registers are in the frame).
        c.g[S2][0] = c.g[A0][0];
        c.g[S7][0] = c.g[A1][0];
        c.sq(16, ecp_at(c, SP, 0x1d0));
        c.sq(17, ecp_at(c, SP, 0x1c0));
        c.sq(19, ecp_at(c, SP, 0x1a0));
        c.sq(20, ecp_at(c, SP, 0x190));
        c.sq(21, ecp_at(c, SP, 0x180));
        c.sq(22, ecp_at(c, SP, 0x170));
        c.sq(30, ecp_at(c, SP, 0x150));
        c.sd(ecp_at(c, SP, 0x140), c.g[RA][0]);
        c.swc1(20, ecp_at(c, SP, 0x1e0));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, ecp_at(c, V0, 0x10));
        c.lqc2(3, ecp_at(c, V0, 0x20));
        c.lqc2(4, ecp_at(c, V0, 0x30));
        c.sqc2(1, r32(c, SP));
        c.sqc2(2, ecp_at(c, SP, 0x10));
        c.sqc2(3, ecp_at(c, SP, 0x20));
        c.sqc2(4, ecp_at(c, SP, 0x30));
        c.lwc1(2, ecp_at(c, S2, 0x18));
        c.g[S0][0] = sx(ecp_at(c, SP, 0x40));
        c.lwc1(0, ecp_at(c, S2, 0x1c));
        c.g[S6][0] = sx(ecp_at(c, SP, 0x90));
        c.lwc1(1, ecp_at(c, S2, 0x20));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.div_s(2, 20, 2);
        c.swc1(20, ecp_at(c, SP, 0x8c));
        c.div_s(0, 20, 0);
        c.div_s(1, 20, 1);
        c.swc1(2, ecp_at(c, SP, 0x80));
        c.swc1(0, ecp_at(c, SP, 0x84));
        c.swc1(1, ecp_at(c, SP, 0x88));
        c.lqc2(1, ecp_at(c, SP, 0x80));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, ecp_at(c, SP, 0x10));
        c.lqc2(6, ecp_at(c, SP, 0x20));
        c.lqc2(7, ecp_at(c, SP, 0x30));
        c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
        c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
        c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
        c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
        c.sqc2(8, r32(c, S6));
        c.sqc2(9, ecp_at(c, S6, 0x10));
        c.sqc2(10, ecp_at(c, S6, 0x20));
        c.sqc2(11, ecp_at(c, S6, 0x30));
        c.lqc2(1, r32(c, S6));
        c.lqc2(2, ecp_at(c, S6, 0x10));
        c.lqc2(3, ecp_at(c, S6, 0x20));
        c.lqc2(4, ecp_at(c, S6, 0x30));
        c.sqc2(1, r32(c, S0));
        c.sqc2(2, ecp_at(c, S0, 0x10));
        c.sqc2(3, ecp_at(c, S0, 0x20));
        c.sqc2(4, ecp_at(c, S0, 0x30));
        c.lqc2(1, r32(c, S0));
        c.lqc2(2, ecp_at(c, S0, 0x10));
        c.lqc2(3, ecp_at(c, S0, 0x20));
        c.lqc2(4, ecp_at(c, S0, 0x30));
        c.sqc2(1, r32(c, SP));
        c.sqc2(2, ecp_at(c, SP, 0x10));
        c.sqc2(3, ecp_at(c, SP, 0x20));
        c.sqc2(4, ecp_at(c, SP, 0x30));
        // Rotation 1: Math_SinCos(-field +0xc) into the first axis.
        c.lwc1(12, ecp_at(c, S2, 0xc));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(ecp_at(c, SP, 0x110));
        c.g[A1][0] = sx(ecp_at(c, SP, 0x114));
        c.neg_s(12, 12);
        call_ptr(c, 0x0031be50, 0x0016e684)?;
        c.lwc1(11, ecp_at(c, SP, 0x114));
        c.g[V0][0] = 0x500000;
        c.lwc1(0, ecp_back(c, V0, 0xea0));
        c.sub_s(1, 20, 11);
        c.g[V0][0] = sx(ecp_back(c, V0, 0xea0));
        c.lwc1(3, ecp_at(c, V0, 0x8));
        c.lwc1(6, ecp_at(c, V0, 0x4));
        c.mul_s(2, 1, 3);
        c.lwc1(8, ecp_at(c, SP, 0x110));
        c.mul_s(4, 1, 0);
        c.sw(ecp_at(c, SP, 0x70), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(ecp_at(c, SP, 0x74), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(ecp_at(c, SP, 0x78), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(ecp_at(c, SP, 0x4c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(ecp_at(c, SP, 0x5c), 0u64);
        c.mul_s(10, 4, 3);
        c.sw(ecp_at(c, SP, 0x6c), 0u64);
        c.mul_s(5, 2, 0);
        c.swc1(20, ecp_at(c, SP, 0x7c));
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, ecp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, ecp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, ecp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, ecp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, ecp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, ecp_at(c, SP, 0x50));
        c.swc1(10, ecp_at(c, SP, 0x60));
        c.swc1(1, ecp_at(c, SP, 0x54));
        c.swc1(3, ecp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, ecp_at(c, S1, 0x10));
        c.lqc2(6, ecp_at(c, S1, 0x20));
        c.lqc2(7, ecp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, ecp_at(c, S0, 0x10));
        c.lqc2(10, ecp_at(c, S0, 0x20));
        c.lqc2(11, ecp_at(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, ecp_at(c, S1, 0x10));
        c.sqc2(14, ecp_at(c, S1, 0x20));
        c.sqc2(15, ecp_at(c, S1, 0x30));
        // Rotation 2: Math_SinCos(-field +0x14) into the second axis.
        c.lwc1(12, ecp_at(c, S2, 0x14));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(ecp_at(c, SP, 0x118));
        c.g[A1][0] = sx(ecp_at(c, SP, 0x11c));
        c.neg_s(12, 12);
        c.g[FP][0] = 0x500000;
        c.g[S5][0] = sx(ecp_back(c, FP, 0xeb0));
        call_ptr(c, 0x0031be50, 0x0016e7d4)?;
        c.lwc1(11, ecp_at(c, SP, 0x11c));
        c.lwc1(0, ecp_back(c, FP, 0xeb0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, ecp_at(c, S5, 0x8));
        c.lwc1(6, ecp_at(c, S5, 0x4));
        c.lwc1(8, ecp_at(c, SP, 0x118));
        c.mul_s(2, 1, 3);
        c.sw(ecp_at(c, SP, 0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(ecp_at(c, SP, 0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(ecp_at(c, SP, 0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(ecp_at(c, SP, 0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(ecp_at(c, SP, 0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(ecp_at(c, SP, 0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, ecp_at(c, SP, 0x7c));
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, ecp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, ecp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, ecp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, ecp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, ecp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, ecp_at(c, SP, 0x50));
        c.swc1(10, ecp_at(c, SP, 0x60));
        c.swc1(1, ecp_at(c, SP, 0x54));
        c.swc1(3, ecp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, ecp_at(c, S1, 0x10));
        c.lqc2(6, ecp_at(c, S1, 0x20));
        c.lqc2(7, ecp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, ecp_at(c, S0, 0x10));
        c.lqc2(10, ecp_at(c, S0, 0x20));
        c.lqc2(11, ecp_at(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, ecp_at(c, S1, 0x10));
        c.sqc2(14, ecp_at(c, S1, 0x20));
        c.sqc2(15, ecp_at(c, S1, 0x30));
        // Rotation 3: Math_SinCos(-field +0x10) into the third axis.
        c.lwc1(12, ecp_at(c, S2, 0x10));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(ecp_at(c, SP, 0x120));
        c.g[A1][0] = sx(ecp_at(c, SP, 0x124));
        c.neg_s(12, 12);
        c.g[S4][0] = 0x500000;
        c.g[S3][0] = sx(ecp_back(c, S4, 0xec0));
        call_ptr(c, 0x0031be50, 0x0016e91c)?;
        c.lwc1(11, ecp_at(c, SP, 0x124));
        c.lwc1(3, ecp_at(c, S3, 0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, ecp_back(c, S4, 0xec0));
        c.lwc1(6, ecp_at(c, S3, 0x4));
        c.lwc1(8, ecp_at(c, SP, 0x120));
        c.mul_s(2, 1, 3);
        c.sw(ecp_at(c, SP, 0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(ecp_at(c, SP, 0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(ecp_at(c, SP, 0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(ecp_at(c, SP, 0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(ecp_at(c, SP, 0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(ecp_at(c, SP, 0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, ecp_at(c, SP, 0x7c));
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, ecp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, ecp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, ecp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, ecp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, ecp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, ecp_at(c, SP, 0x50));
        c.swc1(10, ecp_at(c, SP, 0x60));
        c.swc1(1, ecp_at(c, SP, 0x54));
        c.swc1(3, ecp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, ecp_at(c, S1, 0x10));
        c.lqc2(6, ecp_at(c, S1, 0x20));
        c.lqc2(7, ecp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, ecp_at(c, S0, 0x10));
        c.lqc2(10, ecp_at(c, S0, 0x20));
        c.lqc2(11, ecp_at(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, ecp_at(c, S1, 0x10));
        c.sqc2(14, ecp_at(c, S1, 0x20));
        c.sqc2(15, ecp_at(c, S1, 0x30));
        // Translation by -Pos (the first three floats of the ellipsoid).
        c.lwc1(0, r32(c, S2));
        c.lwc1(1, ecp_at(c, S2, 0x4));
        c.lwc1(2, ecp_at(c, S2, 0x8));
        c.neg_s(0, 0);
        c.neg_s(1, 1);
        c.swc1(20, ecp_at(c, SP, 0x4c));
        c.neg_s(2, 2);
        c.swc1(0, ecp_at(c, SP, 0x40));
        c.swc1(1, ecp_at(c, SP, 0x44));
        c.swc1(2, ecp_at(c, SP, 0x48));
        c.swc1(0, ecp_at(c, SP, 0x50));
        c.swc1(1, ecp_at(c, SP, 0x54));
        c.swc1(2, ecp_at(c, SP, 0x58));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, ecp_at(c, SP, 0x10));
        c.lqc2(6, ecp_at(c, SP, 0x20));
        c.lqc2(7, ecp_at(c, SP, 0x30));
        c.lqc2(8, ecp_at(c, SP, 0x40));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, ecp_at(c, SP, 0x30));
        c.lqc2(8, r32(c, S7));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, ecp_at(c, SP, 0x10));
        c.lqc2(6, ecp_at(c, SP, 0x20));
        c.lqc2(7, ecp_at(c, SP, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, ecp_at(c, SP, 0x50));
        c.lq(2, ecp_at(c, SP, 0x50));
        c.sq(2, ecp_at(c, SP, 0x40));
        c.sw(ecp_at(c, SP, 0x4c), 0u64);
        c.lwc1(0, ecp_at(c, SP, 0x40));
        c.lwc1(2, ecp_at(c, SP, 0x44));
        c.mul_s(0, 0, 0);
        c.lwc1(1, ecp_at(c, SP, 0x48));
        c.mul_s(2, 2, 2);
        c.mul_s(1, 1, 1);
        c.add_s(0, 0, 2);
        c.add_s(0, 0, 1);
        c.c_lt_s(0, 20);
        let inside = c.fcc();
        c.lq(16, ecp_at(c, SP, 0x1d0));
        if !inside {
            // Outside: project the local point onto the unit sphere and map it back.
            c.cycles += 333;
            c.lqc2(3, ecp_at(c, SP, 0x40));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, ecp_at(c, SP, 0x60));
            c.lq(2, ecp_at(c, SP, 0x60));
            c.sq(2, ecp_at(c, SP, 0x40));
            c.sq(2, ecp_at(c, SP, 0x50));
            c.g[V0][0] = 0x500000;
            c.swc1(20, ecp_at(c, SP, 0x4c));
            c.g[V0][0] = sx(ecp_back(c, V0, 0xe60));
            c.lqc2(1, r32(c, V0));
            c.lqc2(2, ecp_at(c, V0, 0x10));
            c.lqc2(3, ecp_at(c, V0, 0x20));
            c.lqc2(4, ecp_at(c, V0, 0x30));
            c.sqc2(1, r32(c, SP));
            c.sqc2(2, ecp_at(c, SP, 0x10));
            c.sqc2(3, ecp_at(c, SP, 0x20));
            c.sqc2(4, ecp_at(c, SP, 0x30));
            c.lwc1(0, r32(c, S2));
            c.lwc1(1, ecp_at(c, S2, 0x4));
            c.lwc1(2, ecp_at(c, S2, 0x8));
            c.swc1(0, ecp_at(c, SP, 0x50));
            c.swc1(1, ecp_at(c, SP, 0x54));
            c.swc1(2, ecp_at(c, SP, 0x58));
            c.swc1(20, ecp_at(c, SP, 0x5c));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, ecp_at(c, SP, 0x10));
            c.lqc2(6, ecp_at(c, SP, 0x20));
            c.lqc2(7, ecp_at(c, SP, 0x30));
            c.lqc2(8, ecp_at(c, SP, 0x50));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, ecp_at(c, SP, 0x30));
            // Map back: the rotations in reverse order, without the sign flip.
            c.lwc1(12, ecp_at(c, S2, 0x10));
            c.g[S1][0] = c.g[SP][0];
            c.g[A0][0] = sx(ecp_at(c, SP, 0x128));
            c.g[A1][0] = sx(ecp_at(c, SP, 0x12c));
            call_ptr(c, 0x0031be50, 0x0016ebbc)?;
            c.lwc1(11, ecp_at(c, SP, 0x12c));
            c.g[S0][0] = sx(ecp_at(c, SP, 0x50));
            c.lwc1(3, ecp_at(c, S3, 0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, ecp_back(c, S4, 0xec0));
            c.lwc1(6, ecp_at(c, S3, 0x4));
            c.lwc1(8, ecp_at(c, SP, 0x128));
            c.mul_s(2, 1, 3);
            c.sw(ecp_at(c, SP, 0x80), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(ecp_at(c, SP, 0x84), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(ecp_at(c, SP, 0x88), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(ecp_at(c, SP, 0x5c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(ecp_at(c, SP, 0x6c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(ecp_at(c, SP, 0x7c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, ecp_at(c, SP, 0x8c));
            c.mul_s(5, 2, 0);
            c.mul_s(7, 4, 0);
            c.mul_s(3, 1, 3);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(4, 4, 6);
            c.mul_s(1, 1, 6);
            c.add_s(5, 5, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(5, ecp_at(c, SP, 0x58));
            c.add_s(7, 7, 11);
            c.swc1(0, ecp_at(c, SP, 0x54));
            c.add_s(4, 4, 13);
            c.swc1(2, ecp_at(c, SP, 0x68));
            c.sub_s(10, 10, 8);
            c.swc1(9, ecp_at(c, SP, 0x78));
            c.add_s(1, 1, 11);
            c.swc1(7, ecp_at(c, SP, 0x50));
            c.add_s(3, 3, 12);
            c.swc1(4, ecp_at(c, SP, 0x60));
            c.swc1(10, ecp_at(c, SP, 0x70));
            c.swc1(1, ecp_at(c, SP, 0x64));
            c.swc1(3, ecp_at(c, SP, 0x74));
            c.lqc2(4, r32(c, S1));
            c.lqc2(5, ecp_at(c, S1, 0x10));
            c.lqc2(6, ecp_at(c, S1, 0x20));
            c.lqc2(7, ecp_at(c, S1, 0x30));
            c.lqc2(8, r32(c, S0));
            c.lqc2(9, ecp_at(c, S0, 0x10));
            c.lqc2(10, ecp_at(c, S0, 0x20));
            c.lqc2(11, ecp_at(c, S0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.vu0().mula(0b1111, 4, Src::Bc(9, X));
            c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
            c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
            c.vu0().mula(0b1111, 4, Src::Bc(10, X));
            c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
            c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
            c.vu0().mula(0b1111, 4, Src::Bc(11, X));
            c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
            c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
            c.sqc2(12, r32(c, S1));
            c.sqc2(13, ecp_at(c, S1, 0x10));
            c.sqc2(14, ecp_at(c, S1, 0x20));
            c.sqc2(15, ecp_at(c, S1, 0x30));
            c.lwc1(12, ecp_at(c, S2, 0x14));
            c.g[S1][0] = c.g[SP][0];
            c.g[A0][0] = sx(ecp_at(c, SP, 0x130));
            c.g[A1][0] = sx(ecp_at(c, SP, 0x134));
            call_ptr(c, 0x0031be50, 0x0016ecfc)?;
            c.lwc1(11, ecp_at(c, SP, 0x134));
            c.lwc1(0, ecp_back(c, FP, 0xeb0));
            c.sub_s(1, 20, 11);
            c.lwc1(3, ecp_at(c, S5, 0x8));
            c.lwc1(6, ecp_at(c, S5, 0x4));
            c.lwc1(8, ecp_at(c, SP, 0x130));
            c.mul_s(2, 1, 3);
            c.sw(ecp_at(c, SP, 0x80), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(ecp_at(c, SP, 0x84), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(ecp_at(c, SP, 0x88), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(ecp_at(c, SP, 0x5c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(ecp_at(c, SP, 0x6c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(ecp_at(c, SP, 0x7c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, ecp_at(c, SP, 0x8c));
            c.mul_s(5, 2, 0);
            c.mul_s(7, 4, 0);
            c.mul_s(3, 1, 3);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(4, 4, 6);
            c.mul_s(1, 1, 6);
            c.add_s(5, 5, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(5, ecp_at(c, SP, 0x58));
            c.add_s(7, 7, 11);
            c.swc1(0, ecp_at(c, SP, 0x54));
            c.add_s(4, 4, 13);
            c.swc1(2, ecp_at(c, SP, 0x68));
            c.sub_s(10, 10, 8);
            c.swc1(9, ecp_at(c, SP, 0x78));
            c.add_s(1, 1, 11);
            c.swc1(7, ecp_at(c, SP, 0x50));
            c.add_s(3, 3, 12);
            c.swc1(4, ecp_at(c, SP, 0x60));
            c.swc1(10, ecp_at(c, SP, 0x70));
            c.swc1(1, ecp_at(c, SP, 0x64));
            c.swc1(3, ecp_at(c, SP, 0x74));
            c.lqc2(4, r32(c, S1));
            c.lqc2(5, ecp_at(c, S1, 0x10));
            c.lqc2(6, ecp_at(c, S1, 0x20));
            c.lqc2(7, ecp_at(c, S1, 0x30));
            c.lqc2(8, r32(c, S0));
            c.lqc2(9, ecp_at(c, S0, 0x10));
            c.lqc2(10, ecp_at(c, S0, 0x20));
            c.lqc2(11, ecp_at(c, S0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.vu0().mula(0b1111, 4, Src::Bc(9, X));
            c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
            c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
            c.vu0().mula(0b1111, 4, Src::Bc(10, X));
            c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
            c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
            c.vu0().mula(0b1111, 4, Src::Bc(11, X));
            c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
            c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
            c.sqc2(12, r32(c, S1));
            c.sqc2(13, ecp_at(c, S1, 0x10));
            c.sqc2(14, ecp_at(c, S1, 0x20));
            c.sqc2(15, ecp_at(c, S1, 0x30));
            c.lwc1(12, ecp_at(c, S2, 0xc));
            c.g[S1][0] = c.g[SP][0];
            c.g[A0][0] = sx(ecp_at(c, SP, 0x138));
            c.g[A1][0] = sx(ecp_at(c, SP, 0x13c));
            call_ptr(c, 0x0031be50, 0x0016ee38)?;
            c.lwc1(11, ecp_at(c, SP, 0x13c));
            c.g[V0][0] = 0x500000;
            c.lwc1(0, ecp_back(c, V0, 0xea0));
            c.sub_s(1, 20, 11);
            c.g[V0][0] = sx(ecp_back(c, V0, 0xea0));
            c.lwc1(3, ecp_at(c, V0, 0x8));
            c.lwc1(6, ecp_at(c, V0, 0x4));
            c.mul_s(2, 1, 3);
            c.lwc1(8, ecp_at(c, SP, 0x138));
            c.mul_s(4, 1, 0);
            c.sw(ecp_at(c, SP, 0x80), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(ecp_at(c, SP, 0x84), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(ecp_at(c, SP, 0x88), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(ecp_at(c, SP, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(ecp_at(c, SP, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.sw(ecp_at(c, SP, 0x7c), 0u64);
            c.mul_s(5, 2, 0);
            c.swc1(20, ecp_at(c, SP, 0x8c));
            c.mul_s(7, 4, 0);
            c.mul_s(3, 1, 3);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(4, 4, 6);
            c.mul_s(1, 1, 6);
            c.add_s(5, 5, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(5, ecp_at(c, SP, 0x58));
            c.add_s(7, 7, 11);
            c.swc1(0, ecp_at(c, SP, 0x54));
            c.add_s(4, 4, 13);
            c.swc1(2, ecp_at(c, SP, 0x68));
            c.sub_s(10, 10, 8);
            c.swc1(9, ecp_at(c, SP, 0x78));
            c.add_s(1, 1, 11);
            c.swc1(7, ecp_at(c, SP, 0x50));
            c.add_s(3, 3, 12);
            c.swc1(4, ecp_at(c, SP, 0x60));
            c.swc1(10, ecp_at(c, SP, 0x70));
            c.swc1(1, ecp_at(c, SP, 0x64));
            c.swc1(3, ecp_at(c, SP, 0x74));
            c.lqc2(4, r32(c, S1));
            c.lqc2(5, ecp_at(c, S1, 0x10));
            c.lqc2(6, ecp_at(c, S1, 0x20));
            c.lqc2(7, ecp_at(c, S1, 0x30));
            c.lqc2(8, r32(c, S0));
            c.lqc2(9, ecp_at(c, S0, 0x10));
            c.lqc2(10, ecp_at(c, S0, 0x20));
            c.lqc2(11, ecp_at(c, S0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.vu0().mula(0b1111, 4, Src::Bc(9, X));
            c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
            c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
            c.vu0().mula(0b1111, 4, Src::Bc(10, X));
            c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
            c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
            c.vu0().mula(0b1111, 4, Src::Bc(11, X));
            c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
            c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
            c.sqc2(12, r32(c, S1));
            c.sqc2(13, ecp_at(c, S1, 0x10));
            c.sqc2(14, ecp_at(c, S1, 0x20));
            c.sqc2(15, ecp_at(c, S1, 0x30));
            // Store the mapped point; point->w is set to 1 here.
            c.lwc1(2, ecp_at(c, S2, 0x20));
            c.g[V0][0] = sx(ecp_at(c, SP, 0xd0));
            c.lwc1(1, ecp_at(c, S2, 0x18));
            c.lwc1(0, ecp_at(c, S2, 0x1c));
            c.swc1(1, ecp_at(c, SP, 0x90));
            c.swc1(0, ecp_at(c, SP, 0x94));
            c.swc1(2, ecp_at(c, SP, 0x98));
            c.swc1(20, ecp_at(c, SP, 0x9c));
            c.lqc2(1, r32(c, S6));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, ecp_at(c, SP, 0x10));
            c.lqc2(6, ecp_at(c, SP, 0x20));
            c.lqc2(7, ecp_at(c, SP, 0x30));
            c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
            c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
            c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
            c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
            c.sqc2(8, r32(c, V0));
            c.sqc2(9, ecp_at(c, V0, 0x10));
            c.sqc2(10, ecp_at(c, V0, 0x20));
            c.sqc2(11, ecp_at(c, V0, 0x30));
            c.lqc2(1, r32(c, V0));
            c.lqc2(2, ecp_at(c, V0, 0x10));
            c.lqc2(3, ecp_at(c, V0, 0x20));
            c.lqc2(4, ecp_at(c, V0, 0x30));
            c.sqc2(1, r32(c, S0));
            c.sqc2(2, ecp_at(c, S0, 0x10));
            c.sqc2(3, ecp_at(c, S0, 0x20));
            c.sqc2(4, ecp_at(c, S0, 0x30));
            c.lqc2(1, r32(c, S0));
            c.lqc2(2, ecp_at(c, S0, 0x10));
            c.lqc2(3, ecp_at(c, S0, 0x20));
            c.lqc2(4, ecp_at(c, S0, 0x30));
            c.sqc2(1, r32(c, SP));
            c.sqc2(2, ecp_at(c, SP, 0x10));
            c.sqc2(3, ecp_at(c, SP, 0x20));
            c.sqc2(4, ecp_at(c, SP, 0x30));
            c.lqc2(8, ecp_at(c, SP, 0x40));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, ecp_at(c, SP, 0x10));
            c.lqc2(6, ecp_at(c, SP, 0x20));
            c.lqc2(7, ecp_at(c, SP, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, ecp_at(c, SP, 0x60));
            c.lq(2, ecp_at(c, SP, 0x60));
            c.sq(2, r32(c, S7));
            c.swc1(20, ecp_at(c, S7, 0xc));
            c.sq(2, ecp_at(c, SP, 0x50));
            c.lq(16, ecp_at(c, SP, 0x1d0));
        }
        c.cycles += 12;
        c.lq(17, ecp_at(c, SP, 0x1c0));
        c.lq(18, ecp_at(c, SP, 0x1b0));
        c.lq(19, ecp_at(c, SP, 0x1a0));
        c.lq(20, ecp_at(c, SP, 0x190));
        c.lq(21, ecp_at(c, SP, 0x180));
        c.lq(22, ecp_at(c, SP, 0x170));
        c.lq(23, ecp_at(c, SP, 0x160));
        c.lq(30, ecp_at(c, SP, 0x150));
        c.g[RA][0] = c.ld(ecp_at(c, SP, 0x140));
        c.lwc1(20, ecp_at(c, SP, 0x1e0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(ecp_at(c, SP, 0x1f0));
        c.pc = ra;
        Ok(())
    });
}

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn lbcp_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`).
#[inline(always)]
pub(crate) fn lbcp_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `void CameraLineBoundObj_ClampPoint(cCTBoundObjLine* src, Vec4* point)`
/// (0x16fbb8; C++ `cCTBoundObjLine_limitPoint`). The line's two end points
/// (`Point[0]`, `Point[1]`) are mapped by the object's transform (translate
/// by `Pos`, rotate by pitch, roll and heading, scale), and `point` is
/// replaced by its projection onto the line through them: `pts[0] + dir * t`,
/// with `t = dot(point - pts[0], dir) / dot(dir, dir)`. The generated body has
/// no clamp to the segment, so `t` is not limited to [0, 1]. The transform
/// is built as VU0 products on stack copies of the identity matrix.
pub fn camera_line_bound_obj_clamp_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 417;
        // Copy the identity matrix to the stack, then load the object (src) and
        // its transform inputs; the C++ builds the matrix from them.
        c.g[SP][0] = sx(lbcp_back(c, SP, 0x150));
        c.g[V0][0] = 0x500000;
        c.sq(18, lbcp_at(c, SP, 0x110));
        c.g[V0][0] = sx(lbcp_back(c, V0, 0xe60));
        c.sq(19, lbcp_at(c, SP, 0x100));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, lbcp_at(c, SP, 0x130));
        c.g[S3][0] = c.g[A1][0];
        c.sq(17, lbcp_at(c, SP, 0x120));
        c.sd(lbcp_at(c, SP, 0xf0), c.g[RA][0]);
        c.swc1(20, lbcp_at(c, SP, 0x140));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, lbcp_at(c, V0, 0x10));
        c.lqc2(3, lbcp_at(c, V0, 0x20));
        c.lqc2(4, lbcp_at(c, V0, 0x30));
        c.sqc2(1, r32(c, SP));
        c.sqc2(2, lbcp_at(c, SP, 0x10));
        c.sqc2(3, lbcp_at(c, SP, 0x20));
        c.sqc2(4, lbcp_at(c, SP, 0x30));
        c.lwc1(0, r32(c, S2));
        c.lwc1(1, lbcp_at(c, S2, 0x4));
        c.lwc1(2, lbcp_at(c, S2, 0x8));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.swc1(0, lbcp_at(c, SP, 0x40));
        c.swc1(1, lbcp_at(c, SP, 0x44));
        c.swc1(2, lbcp_at(c, SP, 0x48));
        c.swc1(20, lbcp_at(c, SP, 0x4c));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.lqc2(8, lbcp_at(c, SP, 0x40));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, lbcp_at(c, SP, 0x30));
        c.lwc1(12, lbcp_at(c, S2, 0x10));
        c.g[A0][0] = sx(lbcp_at(c, SP, 0xd0));
        c.g[S0][0] = c.g[SP][0];
        c.g[A1][0] = sx(lbcp_at(c, SP, 0xd4));
        // fSinCos (0x31be50) on each angle: the rotation matrices.
        call_ptr(c, 0x0031be50, 0x0016fc64)?;
        c.lwc1(11, lbcp_at(c, SP, 0xd4));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(lbcp_back(c, V0, 0xec0));
        c.lwc1(0, lbcp_back(c, V0, 0xec0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, lbcp_at(c, V1, 0x8));
        c.lwc1(6, lbcp_at(c, V1, 0x4));
        c.g[A0][0] = sx(lbcp_at(c, SP, 0x50));
        c.lwc1(8, lbcp_at(c, SP, 0xd0));
        c.mul_s(2, 1, 3);
        c.sw(lbcp_at(c, SP, 0x80), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(lbcp_at(c, SP, 0x84), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(lbcp_at(c, SP, 0x88), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(lbcp_at(c, SP, 0x5c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(lbcp_at(c, SP, 0x6c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(lbcp_at(c, SP, 0x7c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, lbcp_at(c, SP, 0x8c));
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, lbcp_at(c, SP, 0x58));
        c.add_s(7, 7, 11);
        c.swc1(0, lbcp_at(c, SP, 0x54));
        c.add_s(4, 4, 13);
        c.swc1(2, lbcp_at(c, SP, 0x68));
        c.sub_s(10, 10, 8);
        c.swc1(9, lbcp_at(c, SP, 0x78));
        c.add_s(1, 1, 11);
        c.swc1(7, lbcp_at(c, SP, 0x50));
        c.add_s(3, 3, 12);
        c.swc1(4, lbcp_at(c, SP, 0x60));
        c.swc1(10, lbcp_at(c, SP, 0x70));
        c.swc1(1, lbcp_at(c, SP, 0x64));
        c.swc1(3, lbcp_at(c, SP, 0x74));
        c.lqc2(4, r32(c, S0));
        c.lqc2(5, lbcp_at(c, S0, 0x10));
        c.lqc2(6, lbcp_at(c, S0, 0x20));
        c.lqc2(7, lbcp_at(c, S0, 0x30));
        c.lqc2(8, r32(c, A0));
        c.lqc2(9, lbcp_at(c, A0, 0x10));
        c.lqc2(10, lbcp_at(c, A0, 0x20));
        c.lqc2(11, lbcp_at(c, A0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r32(c, S0));
        c.sqc2(13, lbcp_at(c, S0, 0x10));
        c.sqc2(14, lbcp_at(c, S0, 0x20));
        c.sqc2(15, lbcp_at(c, S0, 0x30));
        c.lwc1(12, lbcp_at(c, S2, 0x14));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(lbcp_at(c, SP, 0xd8));
        c.g[A1][0] = sx(lbcp_at(c, SP, 0xdc));
        // fSinCos (0x31be50) on the next angle.
        call_ptr(c, 0x0031be50, 0x0016fdac)?;
        c.lwc1(11, lbcp_at(c, SP, 0xdc));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(lbcp_back(c, V0, 0xeb0));
        c.lwc1(0, lbcp_back(c, V0, 0xeb0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, lbcp_at(c, V1, 0x8));
        c.lwc1(6, lbcp_at(c, V1, 0x4));
        c.g[S0][0] = sx(lbcp_at(c, SP, 0x40));
        c.lwc1(8, lbcp_at(c, SP, 0xd8));
        c.mul_s(2, 1, 3);
        c.sw(lbcp_at(c, SP, 0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(lbcp_at(c, SP, 0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(lbcp_at(c, SP, 0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(lbcp_at(c, SP, 0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(lbcp_at(c, SP, 0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(lbcp_at(c, SP, 0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, lbcp_at(c, SP, 0x7c));
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, lbcp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, lbcp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, lbcp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, lbcp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, lbcp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, lbcp_at(c, SP, 0x50));
        c.swc1(10, lbcp_at(c, SP, 0x60));
        c.swc1(1, lbcp_at(c, SP, 0x54));
        c.swc1(3, lbcp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, lbcp_at(c, S1, 0x10));
        c.lqc2(6, lbcp_at(c, S1, 0x20));
        c.lqc2(7, lbcp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, lbcp_at(c, S0, 0x10));
        c.lqc2(10, lbcp_at(c, S0, 0x20));
        c.lqc2(11, lbcp_at(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, lbcp_at(c, S1, 0x10));
        c.sqc2(14, lbcp_at(c, S1, 0x20));
        c.sqc2(15, lbcp_at(c, S1, 0x30));
        c.lwc1(12, lbcp_at(c, S2, 0xc));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(lbcp_at(c, SP, 0xe0));
        c.g[A1][0] = sx(lbcp_at(c, SP, 0xe4));
        // fSinCos (0x31be50) on the next angle.
        call_ptr(c, 0x0031be50, 0x0016fef4)?;
        c.lwc1(11, lbcp_at(c, SP, 0xe4));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(lbcp_back(c, V0, 0xea0));
        c.lwc1(0, lbcp_back(c, V0, 0xea0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, lbcp_at(c, V1, 0x8));
        c.lwc1(6, lbcp_at(c, V1, 0x4));
        c.lwc1(8, lbcp_at(c, SP, 0xe0));
        c.mul_s(2, 1, 3);
        c.sw(lbcp_at(c, SP, 0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(lbcp_at(c, SP, 0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(lbcp_at(c, SP, 0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(lbcp_at(c, SP, 0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(lbcp_at(c, SP, 0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(lbcp_at(c, SP, 0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, lbcp_at(c, SP, 0x7c));
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, lbcp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, lbcp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, lbcp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, lbcp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, lbcp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, lbcp_at(c, SP, 0x50));
        c.swc1(10, lbcp_at(c, SP, 0x60));
        c.swc1(1, lbcp_at(c, SP, 0x54));
        c.swc1(3, lbcp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, lbcp_at(c, S1, 0x10));
        c.lqc2(6, lbcp_at(c, S1, 0x20));
        c.lqc2(7, lbcp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, lbcp_at(c, S0, 0x10));
        c.lqc2(10, lbcp_at(c, S0, 0x20));
        c.lqc2(11, lbcp_at(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, lbcp_at(c, S1, 0x10));
        c.sqc2(14, lbcp_at(c, S1, 0x20));
        c.sqc2(15, lbcp_at(c, S1, 0x30));
        c.lwc1(2, lbcp_at(c, S2, 0x18));
        c.g[V0][0] = sx(lbcp_at(c, SP, 0x90));
        c.lwc1(1, lbcp_at(c, S2, 0x1c));
        c.lwc1(0, lbcp_at(c, S2, 0x20));
        c.swc1(2, lbcp_at(c, SP, 0x80));
        c.swc1(1, lbcp_at(c, SP, 0x84));
        c.swc1(0, lbcp_at(c, SP, 0x88));
        c.swc1(20, lbcp_at(c, SP, 0x8c));
        c.lqc2(1, lbcp_at(c, SP, 0x80));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
        c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
        c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
        c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
        c.sqc2(8, r32(c, V0));
        c.sqc2(9, lbcp_at(c, V0, 0x10));
        c.sqc2(10, lbcp_at(c, V0, 0x20));
        c.sqc2(11, lbcp_at(c, V0, 0x30));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, lbcp_at(c, V0, 0x10));
        c.lqc2(3, lbcp_at(c, V0, 0x20));
        c.lqc2(4, lbcp_at(c, V0, 0x30));
        c.sqc2(1, r32(c, S0));
        c.sqc2(2, lbcp_at(c, S0, 0x10));
        c.sqc2(3, lbcp_at(c, S0, 0x20));
        c.sqc2(4, lbcp_at(c, S0, 0x30));
        c.lqc2(1, r32(c, S0));
        c.lqc2(2, lbcp_at(c, S0, 0x10));
        c.lqc2(3, lbcp_at(c, S0, 0x20));
        c.lqc2(4, lbcp_at(c, S0, 0x30));
        c.sqc2(1, r32(c, SP));
        c.sqc2(2, lbcp_at(c, SP, 0x10));
        c.sqc2(3, lbcp_at(c, SP, 0x20));
        c.sqc2(4, lbcp_at(c, SP, 0x30));
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = 0x1;
        c.lwc1(0, lbcp_at(c, S2, 0x2c));
        c.lwc1(1, lbcp_at(c, S2, 0x30));
        c.lwc1(2, lbcp_at(c, S2, 0x34));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 3);
        c.swc1(0, lbcp_at(c, SP, 0x70));
        c.swc1(1, lbcp_at(c, SP, 0x74));
        c.swc1(2, lbcp_at(c, SP, 0x78));
        c.swc1(3, lbcp_at(c, SP, 0x7c));
        c.lqc2(8, lbcp_at(c, SP, 0x70));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, lbcp_at(c, SP, 0x80));
        c.lwc1(2, lbcp_at(c, S2, 0x40));
        c.lwc1(0, lbcp_at(c, S2, 0x38));
        c.lwc1(1, lbcp_at(c, S2, 0x3c));
        c.lq(2, lbcp_at(c, SP, 0x80));
        c.swc1(0, lbcp_at(c, SP, 0x70));
        c.sq(2, lbcp_at(c, SP, 0x60));
        c.swc1(1, lbcp_at(c, SP, 0x74));
        c.swc1(2, lbcp_at(c, SP, 0x78));
        c.swc1(3, lbcp_at(c, SP, 0x7c));
        c.sq(2, lbcp_at(c, SP, 0x40));
        c.lqc2(8, lbcp_at(c, SP, 0x70));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, lbcp_at(c, SP, 0x80));
        c.lq(2, lbcp_at(c, SP, 0x80));
        c.sq(2, lbcp_at(c, SP, 0x60));
        c.sq(2, lbcp_at(c, SP, 0x50));
        c.lqc2(3, lbcp_at(c, SP, 0x50));
        c.lqc2(4, lbcp_at(c, SP, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, lbcp_at(c, SP, 0x70));
        c.lq(2, lbcp_at(c, SP, 0x70));
        c.sq(2, lbcp_at(c, SP, 0x60));
        c.lqc2(3, r32(c, S3));
        c.lqc2(4, lbcp_at(c, SP, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, lbcp_at(c, SP, 0x80));
        c.lq(2, lbcp_at(c, SP, 0x80));
        c.lq(16, lbcp_at(c, SP, 0x130));
        c.sq(2, lbcp_at(c, SP, 0x70));
        c.lqc2(3, lbcp_at(c, SP, 0x60));
        c.lqc2(5, lbcp_at(c, SP, 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.lqc2(3, lbcp_at(c, SP, 0x70));
        c.lqc2(5, lbcp_at(c, SP, 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.div_s(0, 0, 1);
        c.lq(17, lbcp_at(c, SP, 0x120));
        c.mfc1(3, 0);
        c.lqc2(4, lbcp_at(c, SP, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, lbcp_at(c, SP, 0x90));
        c.lq(18, lbcp_at(c, SP, 0x110));
        c.lq(2, lbcp_at(c, SP, 0x90));
        c.g[RA][0] = c.ld(lbcp_at(c, SP, 0xf0));
        c.sq(2, lbcp_at(c, SP, 0x80));
        c.lqc2(3, lbcp_at(c, SP, 0x40));
        c.lqc2(4, lbcp_at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, lbcp_at(c, SP, 0x90));
        c.lwc1(20, lbcp_at(c, SP, 0x140));
        c.lq(2, lbcp_at(c, SP, 0x90));
        c.sq(2, r32(c, S3));
        c.lq(19, lbcp_at(c, SP, 0x100));
        c.sq(2, lbcp_at(c, SP, 0x70));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(lbcp_at(c, SP, 0x150));
        c.pc = ra;
        Ok(())
    });
}

/// `void CameraTriggerMan_RemoveTriggerRegions_Wrapper(CameraTriggerMan* man)`
/// (0x16cea8): calls `CameraTriggerMan_RemoveTriggerRegions(man)` (0x16ca18)
/// and returns to its own caller.
pub fn camera_trigger_man_remove_trigger_regions_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0016_ca18,
            f: ee::s02::f_0016ca18,
        }
        .call(c, 0x0016_ceb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_EmptyStub()` (0x16cf40): does nothing.
pub fn camera_trigger_man_empty_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void CameraTriggerMan_UpdateTriggersMask1(CameraTriggerMan* man)`
/// (0x16d1d8): `CameraTriggerMan_UpdateTriggers(man, 1)`.
pub fn camera_trigger_man_update_triggers_mask1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x0016_cf48,
            f: ee::s02::CameraTriggerMan_UpdateTriggers,
        }
        .call(c, 0x0016_d1e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan_RemoveTriggerRegions` (0x0016ca18): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn camera_trigger_man_remove_trigger_regions(c: &mut Cpu) {
    if c.pc != 0x0016ca18 {
        return camera_trigger_man_remove_trigger_regions_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0016ca18;
        loop {
            match pc {
                0x0016ca18 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x0016cb9c;
                        continue;
                    }
                    pc = 0x0016ca44;
                    continue;
                }
                0x0016ca44 => {
                    c.cycles += 3;
                    c.g[S4][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x0016ca50;
                    continue;
                }
                0x0016ca50 => {
                    c.cycles += 42;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0016ca88);
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0016ca88)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x0016caa0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x0016caa0)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0016cae8);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0016cae8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016caf8;
                    continue;
                }
                0x0016caf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb04;
                    continue;
                }
                0x0016cb04 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb10;
                    continue;
                }
                0x0016cb10 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb1c;
                    continue;
                }
                0x0016cb1c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb28;
                    continue;
                }
                0x0016cb28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb34;
                    continue;
                }
                0x0016cb34 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016cb74;
                        continue;
                    }
                    pc = 0x0016cb40;
                    continue;
                }
                0x0016cb40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        pc = 0x0016cb54;
                        continue;
                    }
                    pc = 0x0016cb4c;
                    continue;
                }
                0x0016cb4c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016cb74;
                        continue;
                    }
                    pc = 0x0016cb54;
                    continue;
                }
                0x0016cb54 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x0016cb58;
                    continue;
                }
                0x0016cb58 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0016cb4c) {
                            return Ok(());
                        }
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb64;
                    continue;
                }
                0x0016cb64 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0016cb58) {
                            return Ok(());
                        }
                        pc = 0x0016cb58;
                        continue;
                    }
                    pc = 0x0016cb70;
                    continue;
                }
                0x0016cb70 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0016cb74;
                    continue;
                }
                0x0016cb74 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                        pc = 0x0016cb8c;
                        continue;
                    }
                    pc = 0x0016cb7c;
                    continue;
                }
                0x0016cb7c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0016cb84);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0016cb84)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    pc = 0x0016cb8c;
                    continue;
                }
                0x0016cb8c => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        if back_edge_unwinds(c, 0x0016ca50) {
                            return Ok(());
                        }
                        pc = 0x0016ca50;
                        continue;
                    }
                    pc = 0x0016cb9c;
                    continue;
                }
                0x0016cb9c => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x0016cba4);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cba4)?;
                    c.g[RA][0] = sx(0x0016cbac);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cbac)?;
                    c.g[RA][0] = sx(0x0016cbb4);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0016bf40,
                        f: ee::s02::CameraTriggerMan_FreeTriggers,
                    }
                    .call(c, 0x0016cbb4)?;
                    c.g[RA][0] = sx(0x0016cbbc);
                    call_ptr(c, 0x0015dc18, 0x0016cbbc)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016ca18, 0x0016cbdc);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `camera_trigger_man_remove_trigger_regions` (converted from the `generated` body): any entry
/// other than 0x0016ca18, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn camera_trigger_man_remove_trigger_regions_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0016ca18 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x0016cb9c;
                        continue;
                    }
                    pc = 0x0016ca44;
                    continue;
                }
                0x0016ca44 => {
                    c.cycles += 3;
                    c.g[S4][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x0016ca50;
                    continue;
                }
                0x0016ca50 => {
                    c.cycles += 42;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0016ca88);
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0016ca88)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x0016caa0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x0016caa0)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0016cae8);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0016cae8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016caf8;
                    continue;
                }
                0x0016caf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb04;
                    continue;
                }
                0x0016cb04 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb10;
                    continue;
                }
                0x0016cb10 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb1c;
                    continue;
                }
                0x0016cb1c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb28;
                    continue;
                }
                0x0016cb28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb34;
                    continue;
                }
                0x0016cb34 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016cb74;
                        continue;
                    }
                    pc = 0x0016cb40;
                    continue;
                }
                0x0016cb40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        pc = 0x0016cb54;
                        continue;
                    }
                    pc = 0x0016cb4c;
                    continue;
                }
                0x0016cb4c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016cb74;
                        continue;
                    }
                    pc = 0x0016cb54;
                    continue;
                }
                0x0016cb54 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x0016cb58;
                    continue;
                }
                0x0016cb58 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0016cb4c) {
                            return Ok(());
                        }
                        pc = 0x0016cb4c;
                        continue;
                    }
                    pc = 0x0016cb64;
                    continue;
                }
                0x0016cb64 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0016cb58) {
                            return Ok(());
                        }
                        pc = 0x0016cb58;
                        continue;
                    }
                    pc = 0x0016cb70;
                    continue;
                }
                0x0016cb70 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0016cb74;
                    continue;
                }
                0x0016cb74 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                        pc = 0x0016cb8c;
                        continue;
                    }
                    pc = 0x0016cb7c;
                    continue;
                }
                0x0016cb7c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0016cb84);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0016cb84)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    pc = 0x0016cb8c;
                    continue;
                }
                0x0016cb8c => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        if back_edge_unwinds(c, 0x0016ca50) {
                            return Ok(());
                        }
                        pc = 0x0016ca50;
                        continue;
                    }
                    pc = 0x0016cb9c;
                    continue;
                }
                0x0016cb9c => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x0016cba4);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cba4)?;
                    c.g[RA][0] = sx(0x0016cbac);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cbac)?;
                    c.g[RA][0] = sx(0x0016cbb4);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0016bf40,
                        f: ee::s02::CameraTriggerMan_FreeTriggers,
                    }
                    .call(c, 0x0016cbb4)?;
                    c.g[RA][0] = sx(0x0016cbbc);
                    call_ptr(c, 0x0015dc18, 0x0016cbbc)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016ca18, 0x0016cbdc);
                    return Ok(());
                }
            }
        }
    });
}

/// `CameraTrigger_ApplyAction` (0x0016e1d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn camera_trigger_apply_action(c: &mut Cpu) {
    if c.pc != 0x0016e1d8 {
        return camera_trigger_apply_action_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0016e1d8;
        loop {
            match pc {
                0x0016e1d8 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A2][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x870));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A3][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = c.g[A3][0] == c.g[A1][0];
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                    if t {
                        pc = 0x0016e27c;
                        continue;
                    }
                    pc = 0x0016e210;
                    continue;
                }
                0x0016e210 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2i64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x2;
                        pc = 0x0016e22c;
                        continue;
                    }
                    pc = 0x0016e21c;
                    continue;
                }
                0x0016e21c => {
                    c.cycles += 2;
                    let t = c.g[A3][0] == 0u64;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e2c4;
                        continue;
                    }
                    pc = 0x0016e224;
                    continue;
                }
                0x0016e224 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e22c;
                    continue;
                }
                0x0016e22c => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e234;
                    continue;
                }
                0x0016e234 => {
                    c.cycles += 5;
                    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.mtc1(0, 0);
                    c.c_eq_s(2, 0);
                    let t = c.fcc();

                    if t {
                        pc = 0x0016e260;
                        continue;
                    }
                    pc = 0x0016e248;
                    continue;
                }
                0x0016e248 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6448));
                    c.div_s(0, 0, 2);
                    let t = true;
                    c.mul_s(12, 0, 1);
                    if t {
                        pc = 0x0016e268;
                        continue;
                    }
                    pc = 0x0016e260;
                    continue;
                }
                0x0016e260 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    pc = 0x0016e268;
                    continue;
                }
                0x0016e268 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T0][0];
                    c.g[RA][0] = sx(0x0016e274);
                    c.g[A1][0] = 0x5c;
                    call_ptr(c, 0x00162060, 0x0016e274)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e27c;
                    continue;
                }
                0x0016e27c => {
                    c.cycles += 5;
                    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.mtc1(0, 0);
                    c.c_eq_s(2, 0);
                    let t = c.fcc();

                    if t {
                        pc = 0x0016e2a8;
                        continue;
                    }
                    pc = 0x0016e290;
                    continue;
                }
                0x0016e290 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6444));
                    c.div_s(0, 0, 2);
                    let t = true;
                    c.mul_s(12, 0, 1);
                    if t {
                        pc = 0x0016e2b0;
                        continue;
                    }
                    pc = 0x0016e2a8;
                    continue;
                }
                0x0016e2a8 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    pc = 0x0016e2b0;
                    continue;
                }
                0x0016e2b0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T0][0];
                    c.g[RA][0] = sx(0x0016e2bc);
                    c.g[A1][0] = 0x5b;
                    call_ptr(c, 0x00162060, 0x0016e2bc)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e2c4;
                    continue;
                }
                0x0016e2c4 => {
                    c.cycles += 6;
                    c.mtc1(0, 12);
                    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.c_eq_s(2, 12);
                    let t = !c.fcc();
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x0016e2ec;
                        continue;
                    }
                    pc = 0x0016e2dc;
                    continue;
                }
                0x0016e2dc => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    let t = true;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x0016e304;
                        continue;
                    }
                    pc = 0x0016e2ec;
                    continue;
                }
                0x0016e2ec => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6440));
                    c.div_s(0, 0, 2);
                    c.mul_s(12, 0, 1);
                    c.g[A1][0] = c.g[V0][0];
                    pc = 0x0016e304;
                    continue;
                }
                0x0016e304 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x4cu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x0016e540;
                        continue;
                    }
                    pc = 0x0016e310;
                    continue;
                }
                0x0016e310 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4080));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x0016e328 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x4;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e330;
                    continue;
                }
                0x0016e330 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e338;
                    continue;
                }
                0x0016e338 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e340;
                    continue;
                }
                0x0016e340 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e348;
                    continue;
                }
                0x0016e348 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x7;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e350;
                    continue;
                }
                0x0016e350 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x5;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e358;
                    continue;
                }
                0x0016e358 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x6;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e360;
                    continue;
                }
                0x0016e360 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x8;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e368;
                    continue;
                }
                0x0016e368 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x9;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e370;
                    continue;
                }
                0x0016e370 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xa;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e378;
                    continue;
                }
                0x0016e378 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xb;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e380;
                    continue;
                }
                0x0016e380 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xc;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e388;
                    continue;
                }
                0x0016e388 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xd;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e390;
                    continue;
                }
                0x0016e390 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xe;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e398;
                    continue;
                }
                0x0016e398 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xf;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3a0;
                    continue;
                }
                0x0016e3a0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x10;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3a8;
                    continue;
                }
                0x0016e3a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x11;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3b0;
                    continue;
                }
                0x0016e3b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x12;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3b8;
                    continue;
                }
                0x0016e3b8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x13;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3c0;
                    continue;
                }
                0x0016e3c0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x14;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3c8;
                    continue;
                }
                0x0016e3c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x15;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3d0;
                    continue;
                }
                0x0016e3d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x16;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3d8;
                    continue;
                }
                0x0016e3d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x17;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3e0;
                    continue;
                }
                0x0016e3e0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x18;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3e8;
                    continue;
                }
                0x0016e3e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x19;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3f0;
                    continue;
                }
                0x0016e3f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1a;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3f8;
                    continue;
                }
                0x0016e3f8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1b;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e400;
                    continue;
                }
                0x0016e400 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e408;
                    continue;
                }
                0x0016e408 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1d;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e410;
                    continue;
                }
                0x0016e410 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1e;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e418;
                    continue;
                }
                0x0016e418 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1f;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e420;
                    continue;
                }
                0x0016e420 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x20;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e428;
                    continue;
                }
                0x0016e428 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x29;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e430;
                    continue;
                }
                0x0016e430 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e438;
                    continue;
                }
                0x0016e438 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2a;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e440;
                    continue;
                }
                0x0016e440 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2b;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e448;
                    continue;
                }
                0x0016e448 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e450;
                    continue;
                }
                0x0016e450 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2d;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e458;
                    continue;
                }
                0x0016e458 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2e;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e460;
                    continue;
                }
                0x0016e460 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2f;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e468;
                    continue;
                }
                0x0016e468 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x30;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e470;
                    continue;
                }
                0x0016e470 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x31;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e478;
                    continue;
                }
                0x0016e478 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x32;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e480;
                    continue;
                }
                0x0016e480 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x33;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e488;
                    continue;
                }
                0x0016e488 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x34;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e490;
                    continue;
                }
                0x0016e490 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x35;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e498;
                    continue;
                }
                0x0016e498 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x36;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4a0;
                    continue;
                }
                0x0016e4a0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x37;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4a8;
                    continue;
                }
                0x0016e4a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x38;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4b0;
                    continue;
                }
                0x0016e4b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x39;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4b8;
                    continue;
                }
                0x0016e4b8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3a;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4c0;
                    continue;
                }
                0x0016e4c0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3b;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4c8;
                    continue;
                }
                0x0016e4c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4d0;
                    continue;
                }
                0x0016e4d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3d;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4d8;
                    continue;
                }
                0x0016e4d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3e;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4e0;
                    continue;
                }
                0x0016e4e0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3f;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4e8;
                    continue;
                }
                0x0016e4e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x40;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4f0;
                    continue;
                }
                0x0016e4f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x41;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4f8;
                    continue;
                }
                0x0016e4f8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x42;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e500;
                    continue;
                }
                0x0016e500 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x43;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e508;
                    continue;
                }
                0x0016e508 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x44;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e510;
                    continue;
                }
                0x0016e510 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x45;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e518;
                    continue;
                }
                0x0016e518 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x46;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e520;
                    continue;
                }
                0x0016e520 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x47;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e528;
                    continue;
                }
                0x0016e528 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x48;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e530;
                    continue;
                }
                0x0016e530 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x49;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e538;
                    continue;
                }
                0x0016e538 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x4c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e540;
                    continue;
                }
                0x0016e540 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0u64;
                    pc = 0x0016e544;
                    continue;
                }
                0x0016e544 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[T0][0];
                    c.g[RA][0] = sx(0x0016e550);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00162060, 0x0016e550)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = 0x0016e554;
                    continue;
                }
                0x0016e554 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016e1d8, 0x0016e55c);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `camera_trigger_apply_action` (converted from the `generated` body): any entry
/// other than 0x0016e1d8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn camera_trigger_apply_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0016e1d8 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A2][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x870));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A3][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = c.g[A3][0] == c.g[A1][0];
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                    if t {
                        pc = 0x0016e27c;
                        continue;
                    }
                    pc = 0x0016e210;
                    continue;
                }
                0x0016e210 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2i64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x2;
                        pc = 0x0016e22c;
                        continue;
                    }
                    pc = 0x0016e21c;
                    continue;
                }
                0x0016e21c => {
                    c.cycles += 2;
                    let t = c.g[A3][0] == 0u64;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e2c4;
                        continue;
                    }
                    pc = 0x0016e224;
                    continue;
                }
                0x0016e224 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e22c;
                    continue;
                }
                0x0016e22c => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e234;
                    continue;
                }
                0x0016e234 => {
                    c.cycles += 5;
                    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.mtc1(0, 0);
                    c.c_eq_s(2, 0);
                    let t = c.fcc();

                    if t {
                        pc = 0x0016e260;
                        continue;
                    }
                    pc = 0x0016e248;
                    continue;
                }
                0x0016e248 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6448));
                    c.div_s(0, 0, 2);
                    let t = true;
                    c.mul_s(12, 0, 1);
                    if t {
                        pc = 0x0016e268;
                        continue;
                    }
                    pc = 0x0016e260;
                    continue;
                }
                0x0016e260 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    pc = 0x0016e268;
                    continue;
                }
                0x0016e268 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T0][0];
                    c.g[RA][0] = sx(0x0016e274);
                    c.g[A1][0] = 0x5c;
                    call_ptr(c, 0x00162060, 0x0016e274)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e27c;
                    continue;
                }
                0x0016e27c => {
                    c.cycles += 5;
                    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.mtc1(0, 0);
                    c.c_eq_s(2, 0);
                    let t = c.fcc();

                    if t {
                        pc = 0x0016e2a8;
                        continue;
                    }
                    pc = 0x0016e290;
                    continue;
                }
                0x0016e290 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6444));
                    c.div_s(0, 0, 2);
                    let t = true;
                    c.mul_s(12, 0, 1);
                    if t {
                        pc = 0x0016e2b0;
                        continue;
                    }
                    pc = 0x0016e2a8;
                    continue;
                }
                0x0016e2a8 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    pc = 0x0016e2b0;
                    continue;
                }
                0x0016e2b0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T0][0];
                    c.g[RA][0] = sx(0x0016e2bc);
                    c.g[A1][0] = 0x5b;
                    call_ptr(c, 0x00162060, 0x0016e2bc)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0016e554;
                        continue;
                    }
                    pc = 0x0016e2c4;
                    continue;
                }
                0x0016e2c4 => {
                    c.cycles += 6;
                    c.mtc1(0, 12);
                    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.c_eq_s(2, 12);
                    let t = !c.fcc();
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x0016e2ec;
                        continue;
                    }
                    pc = 0x0016e2dc;
                    continue;
                }
                0x0016e2dc => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    let t = true;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x0016e304;
                        continue;
                    }
                    pc = 0x0016e2ec;
                    continue;
                }
                0x0016e2ec => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6440));
                    c.div_s(0, 0, 2);
                    c.mul_s(12, 0, 1);
                    c.g[A1][0] = c.g[V0][0];
                    pc = 0x0016e304;
                    continue;
                }
                0x0016e304 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x4cu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x0016e540;
                        continue;
                    }
                    pc = 0x0016e310;
                    continue;
                }
                0x0016e310 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4080));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x0016e328 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x4;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e330;
                    continue;
                }
                0x0016e330 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e338;
                    continue;
                }
                0x0016e338 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e340;
                    continue;
                }
                0x0016e340 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e348;
                    continue;
                }
                0x0016e348 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x7;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e350;
                    continue;
                }
                0x0016e350 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x5;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e358;
                    continue;
                }
                0x0016e358 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x6;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e360;
                    continue;
                }
                0x0016e360 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x8;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e368;
                    continue;
                }
                0x0016e368 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x9;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e370;
                    continue;
                }
                0x0016e370 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xa;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e378;
                    continue;
                }
                0x0016e378 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xb;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e380;
                    continue;
                }
                0x0016e380 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xc;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e388;
                    continue;
                }
                0x0016e388 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xd;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e390;
                    continue;
                }
                0x0016e390 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xe;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e398;
                    continue;
                }
                0x0016e398 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0xf;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3a0;
                    continue;
                }
                0x0016e3a0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x10;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3a8;
                    continue;
                }
                0x0016e3a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x11;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3b0;
                    continue;
                }
                0x0016e3b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x12;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3b8;
                    continue;
                }
                0x0016e3b8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x13;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3c0;
                    continue;
                }
                0x0016e3c0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x14;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3c8;
                    continue;
                }
                0x0016e3c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x15;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3d0;
                    continue;
                }
                0x0016e3d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x16;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3d8;
                    continue;
                }
                0x0016e3d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x17;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3e0;
                    continue;
                }
                0x0016e3e0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x18;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3e8;
                    continue;
                }
                0x0016e3e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x19;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3f0;
                    continue;
                }
                0x0016e3f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1a;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e3f8;
                    continue;
                }
                0x0016e3f8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1b;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e400;
                    continue;
                }
                0x0016e400 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e408;
                    continue;
                }
                0x0016e408 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1d;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e410;
                    continue;
                }
                0x0016e410 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1e;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e418;
                    continue;
                }
                0x0016e418 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x1f;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e420;
                    continue;
                }
                0x0016e420 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x20;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e428;
                    continue;
                }
                0x0016e428 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x29;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e430;
                    continue;
                }
                0x0016e430 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e438;
                    continue;
                }
                0x0016e438 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2a;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e440;
                    continue;
                }
                0x0016e440 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2b;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e448;
                    continue;
                }
                0x0016e448 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e450;
                    continue;
                }
                0x0016e450 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2d;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e458;
                    continue;
                }
                0x0016e458 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2e;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e460;
                    continue;
                }
                0x0016e460 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x2f;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e468;
                    continue;
                }
                0x0016e468 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x30;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e470;
                    continue;
                }
                0x0016e470 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x31;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e478;
                    continue;
                }
                0x0016e478 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x32;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e480;
                    continue;
                }
                0x0016e480 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x33;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e488;
                    continue;
                }
                0x0016e488 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x34;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e490;
                    continue;
                }
                0x0016e490 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x35;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e498;
                    continue;
                }
                0x0016e498 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x36;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4a0;
                    continue;
                }
                0x0016e4a0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x37;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4a8;
                    continue;
                }
                0x0016e4a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x38;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4b0;
                    continue;
                }
                0x0016e4b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x39;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4b8;
                    continue;
                }
                0x0016e4b8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3a;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4c0;
                    continue;
                }
                0x0016e4c0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3b;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4c8;
                    continue;
                }
                0x0016e4c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4d0;
                    continue;
                }
                0x0016e4d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3d;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4d8;
                    continue;
                }
                0x0016e4d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3e;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4e0;
                    continue;
                }
                0x0016e4e0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x3f;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4e8;
                    continue;
                }
                0x0016e4e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x40;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4f0;
                    continue;
                }
                0x0016e4f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x41;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e4f8;
                    continue;
                }
                0x0016e4f8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x42;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e500;
                    continue;
                }
                0x0016e500 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x43;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e508;
                    continue;
                }
                0x0016e508 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x44;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e510;
                    continue;
                }
                0x0016e510 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x45;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e518;
                    continue;
                }
                0x0016e518 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x46;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e520;
                    continue;
                }
                0x0016e520 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x47;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e528;
                    continue;
                }
                0x0016e528 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x48;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e530;
                    continue;
                }
                0x0016e530 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x49;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e538;
                    continue;
                }
                0x0016e538 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0x4c;
                    if t {
                        pc = 0x0016e544;
                        continue;
                    }
                    pc = 0x0016e540;
                    continue;
                }
                0x0016e540 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0u64;
                    pc = 0x0016e544;
                    continue;
                }
                0x0016e544 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[T0][0];
                    c.g[RA][0] = sx(0x0016e550);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00162060, 0x0016e550)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = 0x0016e554;
                    continue;
                }
                0x0016e554 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016e1d8, 0x0016e55c);
                    return Ok(());
                }
            }
        }
    });
}

/// `CameraTriggerMan_ReadTriggerAction(cursor, actionOut, cookie)` (0x16c518):
/// reads one action (`CameraTrigger_ReadAction`); result 6 if none. The
/// action's type word then picks the bound-object reader (type 1, at
/// `action + 8`, its result returned) or the spline reader (type 2, at
/// `action + 8`, result 0). Other types return 0.
pub fn camera_trigger_man_read_trigger_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        'a: {
            'b: {
                'c: {
                    Callee {
                        addr: 0x0017_12b0,
                        f: ee::s02::f_001712b0,
                    }
                    .call(c, 0x0016_c53c)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                        break 'c;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x6;
                    break 'b;
                }
                c.cycles += 4;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                'd: {
                    let not_bound = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if not_bound {
                        break 'd;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.g[S2][0];
                    Callee {
                        addr: 0x0016_c4f0,
                        f: ee::s02::f_0016c4f0,
                    }
                    .call(c, 0x0016_c56c)?;
                    frame.restore(c, S0, 0x30);
                    break 'a;
                }
                // Type 2 (spline), or another type: v0 still holds 2 here.
                c.cycles += 2;
                let other = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if other {
                    break 'b;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                Callee {
                    addr: 0x0017_1d58,
                    f: ee::s02::f_00171d58,
                }
                .call(c, 0x0016_c588)?;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera trigger file reading.

/// `int CameraTriggerMan_ReadTrigger(void* buffer, void* trigger, int version)`
/// (0x16c428): reads one camera trigger. Two `get_uint` reads fill its first
/// two words from `*trigger` and `*trigger + 4`; then `ReadTriggerVolume` reads
/// `*trigger + 8` and, when that status is zero, `ReadTriggerAction` reads the
/// actions at `*trigger + 0xc` and `*trigger + 0x10`. Returns the first
/// non-zero status, or the status of the last read (0). Frame 0x40 (s0-s2).
pub fn camera_trigger_man_read_trigger(c: &mut Cpu) {
    const GET_UINT: u32 = 0x002d_2558;
    const READ_VOLUME: Callee = Callee {
        addr: 0x0016_c4c8,
        f: ee::s02::f_0016c4c8,
    };
    const READ_ACTION: Callee = Callee {
        addr: 0x0016_c518,
        f: ee::s02::f_0016c518,
    };
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0]; // the buffer
        c.g[S1][0] = c.g[A1][0]; // the trigger
        c.g[S2][0] = c.g[A2][0]; // the version
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        call_ptr(c, GET_UINT, 0x0016_c450)?;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
        call_ptr(c, GET_UINT, 0x0016_c460)?;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x8));
        READ_VOLUME.call(c, 0x0016_c474)?;
        'done: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 7;
            c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xc));
            READ_ACTION.call(c, 0x0016_c490)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 6;
            c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            READ_ACTION.call(c, 0x0016_c4ac)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan_ReadTriggerVolume` (0x16c4c8; the reference's `readVolume`):
/// reads the trigger's volume through `CameraTrigger_ReadSphereTrigger`
/// (0x172e18). Returns 0 when that read reports a volume, else 4.
pub fn camera_trigger_man_read_trigger_volume(c: &mut Cpu) {
    const RET: u32 = 0x0016_c4d8;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0017_2e18,
            f: ee::s02::f_00172e18,
        }
        .call(c, RET)?;
        c.g[V1][0] = 0x4;
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

/// `CameraTriggerMan::ReadBoundObjAction` (0x16c4f0): returns 5 when
/// `CameraTrigger_ReadBoundObj` gives nothing, else 0.
pub fn camera_trigger_man_read_bound_obj_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0017_1690,
            f: ee::s02::f_00171690,
        }
        .call(c, 0x0016_c500)?;
        c.g[V1][0] = 0x5;
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::compare;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::camera::camera::tests::camera_chain;

    pub(crate) use crate::game::camera::script::scriptcontroller::tests::put_globals;

    pub(crate) use crate::game::camera::script::scriptcontroller::tests::OUT;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::SEEDS_2 as SEEDS;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::axes;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    use ssx3_ee as ee;

    #[test]
    fn camera_trigger_man_push_trigger_stack_matches() {
        check(
            ssx3_ee::s02::f_0016d238__generated,
            camera_trigger_man_push_trigger_stack,
            200,
            &|c, seed| {
                let mut r = Rng(0xa1a1_a1a1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let n0 = r.next_u32() % 8;
                // Odd seeds set high bits that vanish in the 32-bit `<< 2`.
                let count = if seed % 2 == 0 { n0 } else { 0x4000_0000 | n0 };
                c.bus.write32(DATA, count);
            },
        );
    }

    #[test]
    fn camera_bound_obj_get_world_point_matches() {
        check(
            ssx3_ee::s02::f_00170d20__generated,
            camera_bound_obj_get_world_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x170d_2001 ^ seed as u64);
                scramble(c, &mut r);
                axes(c);
                let out = DATA + 0x200;
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = sx((seed % 8) as u32);
                // Position, three angles, scale, then the eight corners.
                for i in 0..8 {
                    c.bus.write32(DATA + 4 * i, r.edge_f32());
                }
                c.bus.write32(DATA + 0x18, r.f32(2.0));
                c.bus.write32(DATA + 0x1c, r.f32(2.0));
                c.bus.write32(DATA + 0x20, r.f32(2.0));
                for i in 0..8 * 3 {
                    c.bus.write32(DATA + 0x28 + 4 * i, r.edge_f32());
                }
                c.pc = 0x0017_0d20;
            },
        );
    }

    pub(crate) const BOUND: u32 = DATA + 0x1000;

    /// A line bound: translation, three angles, scale, start and end points.
    pub(crate) fn put_line_bound(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for k in 0..0x12u32 {
            c.bus.write32(BOUND + 4 * k, r.f32(3.0));
        }
        for k in 0..3u32 {
            let s = if seed % 5 == 0 && k == 1 {
                0
            } else {
                r.f32(2.0)
            };
            c.bus.write32(BOUND + line_bound::SCALE + 4 * k, s);
        }
    }

    #[test]
    fn camera_line_bound_world_start_matches() {
        check(
            ee::s02::f_00170240__generated,
            camera_line_bound_world_start,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0017_0240 ^ (seed as u64) << 7);
                scramble(c, &mut r);
                put_globals(c, &mut r);
                put_line_bound(c, &mut r, seed);
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(BOUND);
            },
        );
    }

    #[test]
    fn camera_line_bound_world_end_matches() {
        check(
            ee::s02::f_001707b0__generated,
            camera_line_bound_world_end,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0017_07b0 ^ (seed as u64) << 7);
                scramble(c, &mut r);
                put_globals(c, &mut r);
                put_line_bound(c, &mut r, seed);
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(BOUND);
            },
        );
    }

    #[test]
    fn camera_trigger_man_empty_global_call_matches() {
        crate::difftest::check(
            ee::s02::f_0016cf18__generated,
            camera_trigger_man_empty_global_call,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16cf_18, 1);
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(DATA + (r.next_u32() & 0xfff0))
                };
            },
        );
    }

    #[test]
    fn camera_trigger_stack_ctor_matches() {
        check(
            ssx3_ee::s02::f_0016d210__generated,
            camera_trigger_stack_ctor,
            200,
            &|c, seed| {
                let mut r = rng(seed, 0x14);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x2000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn camera_trigger_man_unload_matches() {
        check(
            ssx3_ee::s02::f_0016c9b0__generated,
            camera_trigger_man_unload,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x16c9b0, 6);
                // The camera chain is set up so the chase reset really runs;
                // an empty trigger list makes the purge loops skip.
                camera_chain(c, seed);
                let m = DATA;
                c.bus.write32(m + 4, 0);
                c.g[A0][0] = m as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_pop_trigger_stack_matches() {
        check(
            ee::s02::f_0016d260__generated,
            camera_trigger_man_pop_trigger_stack,
            150,
            &|c, seed| {
                let mut r = Rng(0x0016_d260 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let stack = DATA;
                let count = (seed % 4) as u32;
                c.bus.write32(stack, count);
                c.bus.write32(stack + 4, r.next_u32());
                for i in 0..4 {
                    c.bus.write32(stack + 8 + 4 * i, (r.next_u32() % 4) as u32);
                }
                c.g[A0][0] = sx(stack);
                c.g[A1][0] = (r.next_u32() % 5) as u64;
                c.g[A2][0] = sx(DATA + 0x100);
                c.g[A3][0] = sx(DATA + 0x200);
            },
        );
    }

    pub(crate) const SEEDS_2: u32 = 200;

    /// Hard, caller-saved (`soft`) and LO/HI must all match.
    pub(crate) fn strict(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    pub(crate) fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    pub(crate) fn w16(c: &mut Cpu, at: u32, v: u16) {
        c.bus.write16(at, v);
    }

    /// Rider at DATA+0x14000, its pose vtable object at +0x15c0, camera
    /// trigger manager at DATA+0x12000 (two spatial-key categories), the
    /// rider's trigger list at DATA+0x13014, and the two virtual pose calls
    /// forced to return the rider box at DATA+0x16000 / +0x17000.
    ///
    /// The box is min=max=(0, 2048, 0), so a category whose cell is
    /// [-2048, 2048]^3 classifies as 0 (inside; exercises the list walk) and
    /// a far-away cell as 1 (outside; exercises the skip/tail paths). The
    /// `v0 == 2` (box collection) path is not exercised: it would run the
    /// real octree box walker, whose fixture is much larger than this test's
    /// budget.
    pub(crate) fn setup(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x0016_d320, 0);
        let world = DATA + 0x1_0000;
        let game = world + 0x1000;
        let mgr = DATA + 0x12_000;
        let this = DATA + 0x13_000;
        let rider = DATA + 0x14_000;
        let vt = DATA + 0x15_000;
        let amin = DATA + 0x16_000;
        let amax = DATA + 0x17_000;

        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x11_000); // BELibrary dummy
        w(c, GAME_GP.wrapping_sub(0x848), world);
        w(c, world + 0x84, game);
        w(c, game + 0x20, mgr);

        // Rider slot count and the trigger list inside `this`.
        let count = seed % 3 % 2;
        w(c, this + 0x870, count);
        w(c, this + 0x14, 0);
        w(c, this + 0x18, 0);

        // Entry gate: 0x874 != 0 and 0x87c > 0 to do any work.
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(rider);
        w(c, rider + 0x874, if seed % 8 == 0 { 0 } else { 1 });
        w(
            c,
            rider + 0x87c,
            match seed % 8 {
                1 => 0,
                2 => 0xffff_ffff,
                _ => 1,
            },
        );

        // The rider's pose object; both virtual calls go to the fake hook.
        w(c, rider + 0x6c0, vt);
        w16(c, vt + 0x28, 0);
        w(c, vt + 0x2c, 0x5000);
        w(c, LOG_RESULTS, amin);
        w(c, LOG_RESULTS + 4, amax);
        for a in [amin, amax] {
            w(c, a, 0);
            w(c, a + 4, 0x4500_0000); // 2048.0
            w(c, a + 8, 0);
            w(c, a + 12, 0);
        }

        // Category 0 is the manager itself; its key is either inside or far
        // outside the rider box depending on the seed.
        let outside = !matches!(seed % 8, 3 | 4 | 5 | 6);
        let idx = if outside { -100i32 } else { -1i32 };
        w(c, mgr, 11);
        for off in [4u32, 8, 12] {
            w(c, mgr + off, idx as u32);
        }

        // The other seven categories (spatial key + list head at +0x10).
        for off in [0x14u32, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c] {
            w(c, mgr + off, 11);
            for k in [4u32, 8, 12] {
                w(c, mgr + off + k, (-100i32) as u32);
            }
            w(c, mgr + off + 0x10, 0);
        }

        // A small valid list node so category 0's head and the child slots
        // are real (all child pointers null, so no octree walk).
        let node = DATA + 0x18_000;
        let walk = DATA + 0x19_000;
        w(c, node + 0x28, 0);
        for off in [0u32, 4, 8, 12, 0x10, 0x14, 0x18, 0x1c] {
            w(c, node + off, 0);
        }

        match seed % 8 {
            3 => {
                w(c, mgr + 0x10, node);
            }
            4 => {
                // One non-matching list node: type != 4, end of list.
                w(c, walk, 0);
                w(c, walk + 8, 0);
                w(c, node + 0x28, walk);
                w(c, mgr + 0x10, node);
            }
            5 => {
                // One matching list node (type 4): two virtual calls; the
                // second is pointed at ActiveCameraTriggerList_Find with an
                // empty list, so it returns 0 and no list is modified.
                let trigger = DATA + 0x1a_000;
                let obj1 = DATA + 0x1b_000;
                let obj2 = DATA + 0x1c_000;
                w(c, walk, 0);
                w(c, walk + 8, 4);
                w(c, walk + 0xc, trigger);
                w(c, node + 0x28, walk);
                w(c, mgr + 0x10, node);
                w(c, trigger + 8, obj2);
                w(c, vt + 0x6c0, obj1);
                w16(c, obj1 + 0x28, 0);
                w(c, obj1 + 0x2c, 0x5000);
                w(c, obj2 + 0x24, obj2);
                w(c, obj2, 0);
                w16(c, obj2 + 0x38, 0);
                w(c, obj2 + 0x3c, 0x0016_c6b0);
            }
            6 => {}
            7 => {
                // Every category has a list head: all eight category tests
                // run (but their keys are outside the box, so none walks a
                // list or collects a box).
                let dummy = DATA + 0x1d_000;
                for off in [0x10u32, 0x24, 0x38, 0x4c, 0x60, 0x74, 0x88, 0x9c] {
                    w(c, mgr + off, dummy);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn camera_trigger_man_update_for_rider_matches() {
        strict(
            ee::s02::f_0016d320__generated,
            camera_trigger_man_update_for_rider,
            SEEDS_2,
            &setup,
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::SP as TEST_SP;

    pub(crate) use crate::game::be::belibrary::tests::oc431_install;
    pub(crate) use crate::game::be::belibrary::tests::oc431_machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    pub(crate) use crate::game::main::ssxapp::tests::oc429_gp;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    pub(crate) use crate::game::main::ssxapp::tests::OC429_FX;

    /// `CameraTriggerMan_UpdateTriggers`: 0..3 triggers, each matching the
    /// mask or not and registered or not (register / remove / skip), the
    /// four node choices from the box extents (random stack words), nodes
    /// with an empty or non-empty entry list (freed or kept), unwinds in
    /// each call.
    #[test]
    fn camera_trigger_man_update_triggers_matches() {
        covered(
            ee::s02::f_0016cf48__generated,
            camera_trigger_man_update_triggers,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x16cf48, 12, 0x0016_cf48);
                let this = OC429_FX + 0x400;
                let list = OC429_FX + 0x600;
                let world = OC429_FX + 0x300;
                let x = OC429_FX + 0x340;
                let y = OC429_FX + 0xa00;
                let shape_vt = OC429_FX + 0x740;
                let entries = OC429_FX + 0xc00;
                oc429_w(c, oc429_gp(-0x848), world);
                oc429_w(c, world + 0x84, x);
                oc429_w(c, x + 0x20, y);
                // The node lists of every node choice.
                for node in [0u32, 0x14, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c] {
                    oc429_w(c, y + node + 0x10, entries);
                }
                oc429_w(c, y + 0xa0, r.next_u32());
                // Entry list: all empty, or one word set.
                let k = r.next_u32() % 12;
                for i in 0..11u32 {
                    oc429_w(c, entries + 4 * i, (i == k) as u32 * 0x1234);
                }
                c.bus.write16(shape_vt + 0x30, (r.next_u32() as u16) & !3);
                oc429_w(c, shape_vt + 0x34, fakes::FAKE);
                let n = seed % 4;
                oc429_w(c, this, list);
                oc429_w(c, this + 4, n);
                let mask = 1 + r.next_u32() % 3;
                for i in 0..3u32 {
                    let t = OC429_FX + 0x800 + 0x100 * i;
                    oc429_w(c, list + 4 * i, t);
                    oc429_w(c, t + 4, r.next_u32() % 4);
                    oc429_w(c, t + 8, OC429_FX + 0x700 + 0x10 * i);
                    oc429_w(c, OC429_FX + 0x700 + 0x10 * i + 0x24, shape_vt);
                    oc429_w(c, t + 0x14, r.next_u32());
                    oc429_w(c, t + 0x20, if r.next_u32() & 1 == 0 { 0 } else { t });
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = mask as u64;
            },
        );
    }

    /// `CameraTriggerMan_RemoveAllTriggers`: for each registered trigger
    /// (bounds through the shape's vtable, spatial-hash removal in the chosen
    /// cell), a cell that ends up empty (11 zero words) is freed. Then the
    /// stacks, lists and chase controllers are reset (fakes).
    #[test]
    fn camera_trigger_man_remove_all_triggers_matches() {
        oc431_install();
        covered(
            ee::s02::f_0016cbe0__generated,
            camera_trigger_man_remove_all_triggers,
            200,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x16cbe0, 20, 0x0016_cbe0);
                let (this, list, world, w2, w3, mgr, vt, cells) = (
                    OC431_BASE + 0x100,
                    OC431_BASE + 0x200,
                    OC431_BASE + 0x300,
                    OC431_BASE + 0x400,
                    OC431_BASE + 0x500,
                    OC431_BASE + 0x600,
                    OC431_BASE + 0x1000,
                    OC431_BASE + 0x2000,
                );
                c.g[A0][0] = this as u64;
                let n = r.next_u32() % 4;
                c.bus.write32(this, list);
                c.bus.write32(this + 4, n);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, w2);
                c.bus.write32(w2 + 0x20, mgr);
                let _ = w3;
                c.bus.write32(mgr + 0xa0, r.next_u32());
                c.bus.write16(vt + 0x30, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x34, FAKE);
                for i in 0..4u32 {
                    let trig = OC431_BASE + 0x3000 + 0x80 * i;
                    let shape = OC431_BASE + 0x3800 + 0x40 * i;
                    c.bus.write32(list + 4 * i, trig);
                    c.bus.write32(trig + 8, shape);
                    c.bus.write32(trig + 0x20, (r.next_u32() % 3 != 0) as u32);
                    c.bus.write32(shape + 0x24, vt);
                    fill(c, &mut r, trig + 0x10, 4);
                }
                // The box the fake bounds call leaves on the stack decides the cell:
                // negative coordinates pick the lower cells.
                for k in 0..3 {
                    let v = if r.next_u32() & 1 == 0 {
                        u32::MAX - 2
                    } else {
                        1 + r.next_u32() % 5
                    };
                    c.bus.write32(TEST_SP - 0xa0 + 0x24 + 4 * k, v);
                }
                // Each candidate cell holds a pointer (+0x10) to 11 words that
                // are all zero, or zero up to a random position.
                for (i, off) in [0u32, 0x14, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c]
                    .into_iter()
                    .enumerate()
                {
                    let cell = cells + 0x40 * i as u32;
                    c.bus.write32(mgr + off + 0x10, cell);
                    for k in 0..11u32 {
                        c.bus.write32(cell + 4 * k, 0);
                    }
                    if r.next_u32() % 3 != 0 {
                        c.bus
                            .write32(cell + 4 * (r.next_u32() % 11), 1 + r.next_u32() % 9);
                    }
                }
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    pub(crate) use crate::game::ai::rider::tests::oc436_machine;

    pub(crate) use crate::game::camera::camera::tests::SEEDS_5 as SEEDS_3;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `CameraTriggerStack_Clear`: both words are cleared whatever they held.
    #[test]
    fn oc436_camera_trigger_stack_clear_matches() {
        check(
            ee::s02::f_0016d200__generated,
            oc436_camera_trigger_stack_clear,
            SEEDS_3,
            &|c, seed| {
                let mut r = Rng(0x16d2_00 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let stack = DATA + 0x100;
                c.g[A0][0] = sx(stack);
                let (a, b) = (r.next_u32(), r.next_u32());
                c.bus.write32(stack, a);
                c.bus.write32(stack + 4, b);
            },
        );
    }

    /// `CameraTriggerMan_Reload`: no triggers, negative count, 1-3 triggers
    /// (each has its mask word cleared); the callees are logged fakes
    /// (`UpdateTriggersMask2` runs for real on top of the `UpdateTriggers`
    /// fake).
    #[test]
    fn oc436_camera_trigger_man_reload_matches() {
        covered(
            ee::s02::f_0016ce30__generated,
            oc436_camera_trigger_man_reload,
            SEEDS_3,
            &|c, seed| {
                let mut r = oc436_machine(c, seed, 0x16ce30, 3);
                let man = DATA + 0x1000;
                let array = DATA + 0x1100;
                c.g[A0][0] = sx(man);
                c.g[A1][0] = sx(r.next_u32());
                let count = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    k => k - 1,
                };
                c.bus.write32(man + OC436_CTM_TRIGGERS, array);
                c.bus.write32(man + OC436_CTM_COUNT, count);
                for i in 0..4u32 {
                    let trigger = DATA + 0x1400 + 0x40 * i;
                    c.bus.write32(array + 4 * i, trigger);
                    let junk = r.next_u32() | 1;
                    c.bus.write32(trigger + OC436_TRIGGER_MASK, junk);
                }
            },
        );
    }

    /// Gate word of this module's callee fakes (in the machine's own RAM).
    pub(crate) const OC438_CTRL: u32 = DATA + 0xf_fe00;

    pub(crate) const OC438_MAGIC: u32 = 0x4f43_4438;

    macro_rules! oc438_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC438_CTRL) == OC438_MAGIC {
                    let ra = c.g[RA][0] as u32;
                    return fake(c, $addr, ra);
                }
                fall_back(c, &$prev, $gen)
            }
        };
    }

    oc438_gated!(
        oc438_fake_ui_state_dtor,
        OC438_PREV_UI_STATE_DTOR,
        0x0039_e390,
        ee::s13::f_0039e390__generated
    );

    oc438_gated!(
        oc438_fake_anim_ctrl_dtor,
        OC438_PREV_ANIM_CTRL_DTOR,
        0x0031_1958,
        ee::s10::f_00311958__generated
    );

    oc438_gated!(
        oc438_fake_read_u32,
        OC438_PREV_READ_U32,
        0x002d_2558,
        ee::s09::f_002d2558__generated
    );

    oc438_gated!(
        oc438_fake_bolts_load_db,
        OC438_PREV_BOLTS_LOAD_DB,
        0x0014_c6a0,
        ee::s01::f_0014c6a0__generated
    );

    oc438_gated!(
        oc438_fake_bolts_prepare,
        OC438_PREV_BOLTS_PREPARE,
        0x0014_d068,
        ee::s01::f_0014d068__generated
    );

    oc438_gated!(
        oc438_fake_update_triggers,
        OC438_PREV_UPDATE_TRIGGERS,
        0x0016_cf48,
        ee::s02::f_0016cf48__generated
    );

    oc438_gated!(
        oc438_fake_ctm_unload,
        OC438_PREV_CTM_UNLOAD,
        0x0016_c9b0,
        ee::s02::f_0016c9b0__generated
    );

    oc438_gated!(
        oc438_fake_ctm_parse,
        OC438_PREV_CTM_PARSE,
        0x0016_c228,
        ee::s02::f_0016c228__generated
    );

    pub(crate) fn oc438_install_fakes() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc438_fake_ui_state_dtor,
            &OC438_PREV_UI_STATE_DTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00311958),
            oc438_fake_anim_ctrl_dtor,
            &OC438_PREV_ANIM_CTRL_DTOR,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2558),
            oc438_fake_read_u32,
            &OC438_PREV_READ_U32,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C6A0),
            oc438_fake_bolts_load_db,
            &OC438_PREV_BOLTS_LOAD_DB,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014D068),
            oc438_fake_bolts_prepare,
            &OC438_PREV_BOLTS_PREPARE,
        );
        hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016CF48),
            oc438_fake_update_triggers,
            &OC438_PREV_UPDATE_TRIGGERS,
        );
        hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C9B0),
            oc438_fake_ctm_unload,
            &OC438_PREV_CTM_UNLOAD,
        );
        hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C228),
            oc438_fake_ctm_parse,
            &OC438_PREV_CTM_PARSE,
        );
    }

    /// Fixture of a call-making function: scrambled machine, gate on, `pc` at
    /// the entry; `calls` is how many callees it makes.
    pub(crate) fn oc438_machine(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC438_CTRL, OC438_MAGIC);
        r
    }

    /// A `CameraTriggerMan` at `man`: trigger count at +4 (zero, negative or
    /// 1-3), pointer array at +0 to objects in the data area.
    pub(crate) fn oc438_man(c: &mut Cpu, r: &mut Rng, man: u32) {
        let count = match r.next_u32() % 4 {
            0 => 0,
            1 => (-(r.next_u32() as i32 % 3)) as u32,
            _ => 1 + r.next_u32() % 3,
        };
        let array = DATA + 0x2000;
        c.bus.write32(man, array);
        c.bus.write32(man + 4, count);
        for k in 0..4 {
            c.bus.write32(array + 4 * k, DATA + 0x3000 + 0x100 * k);
            c.bus
                .write32(DATA + 0x3000 + 0x100 * k + 0x20, r.next_u32());
        }
    }

    /// `CameraTriggerMan_ReloadIf_Global`: reloads the global manager
    /// (0x4c5830) when the flag is non-zero; Unload, Parse and UpdateTriggers
    /// are faked.
    #[test]
    fn camera_trigger_man_reload_if_global_matches() {
        oc438_install_fakes();
        covered(
            ee::s02::f_0016cec8__generated,
            camera_trigger_man_reload_if_global,
            200,
            &|c, seed| {
                let mut r = oc438_machine(c, seed, 0x0016_cec8, 3);
                oc438_man(c, &mut r, 0x004c_5830);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }

    /// `CameraTriggerMan_ReloadIf`: the flag in a2 off (no call) or on
    /// (`Reload`: Unload, Parse, a loop over the triggers, UpdateTriggers).
    #[test]
    fn camera_trigger_man_reload_if_matches() {
        oc438_install_fakes();
        covered(
            ee::s02::f_0016cef0__generated,
            camera_trigger_man_reload_if,
            200,
            &|c, seed| {
                let mut r = oc438_machine(c, seed, 0x0016_cef0, 3);
                let man = DATA + 0x100;
                oc438_man(c, &mut r, man);
                c.g[A0][0] = man as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }

    /// `CameraTriggerMan_UpdateTriggersMask2`: `UpdateTriggers(man, 2)`.
    #[test]
    fn camera_trigger_man_update_triggers_mask2_matches() {
        oc438_install_fakes();
        covered(
            ee::s02::f_0016d1b8__generated,
            camera_trigger_man_update_triggers_mask2,
            200,
            &|c, seed| {
                let mut r = oc438_machine(c, seed, 0x0016_d1b8, 1);
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    /// `CameraTriggerMan_EmptyHook`: only the return.
    #[test]
    fn camera_trigger_man_empty_hook_matches() {
        check(
            ee::s02::f_0016ca10__generated,
            camera_trigger_man_empty_hook,
            10,
            &|c, seed| {
                let mut r = Rng(0x16ca_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_ca10;
            },
        );
    }

    use crate::difftest::put_vec;

    pub(crate) const ENTRY: u32 = 0x0016_f068;

    pub(crate) const BOX: u32 = DATA;

    pub(crate) const POINT: u32 = DATA + 0x100;

    pub(crate) const ONE: u32 = 0x3f80_0000;

    #[test]
    fn camera_box_bound_obj_clamp_point_matches() {
        check(
            ee::s02::f_0016f068__generated,
            camera_box_bound_obj_clamp_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x5eed_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(BOX);
                c.g[A1][0] = sx(POINT);
                c.pc = ENTRY;
                // Axis vectors (X, Y, Z at 0x4ff140/50/60) and the identity
                // matrix the function copies from (0x4ff1a0).
                put_vec(c, 0x004f_f140, [ONE, 0, 0, 0]);
                put_vec(c, 0x004f_f150, [0, ONE, 0, 0]);
                put_vec(c, 0x004f_f160, [0, 0, ONE, 0]);
                put_vec(c, 0x004f_f1a0, [ONE, 0, 0, 0]);
                put_vec(c, 0x004f_f1b0, [0, ONE, 0, 0]);
                put_vec(c, 0x004f_f1c0, [0, 0, ONE, 0]);
                put_vec(c, 0x004f_f1d0, [0, 0, 0, ONE]);
                // Box: pos, rot (radians), half size (non-negative).
                for i in 0..3 {
                    c.bus.write32(BOX + 4 * i, r.f32(60.0));
                }
                for i in 3..6 {
                    c.bus.write32(BOX + 4 * i, r.f32(3.2));
                }
                for i in 6..9 {
                    c.bus.write32(BOX + 4 * i, r.f32(60.0) & 0x7fff_ffff);
                }
                // Point (w is overwritten by the function).
                put_vec(c, POINT, [r.f32(80.0), r.f32(80.0), r.f32(80.0), ONE]);
                // Both sides of the one-time axis cache.
                c.bus
                    .write32(GAME_GP + 0x1cf8, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    use crate::difftest::elf_image;

    pub(crate) use crate::game::camera::camera::tests::OC492_CTRL;
    pub(crate) use crate::game::camera::camera::tests::OC492_MAGIC;

    pub(crate) use crate::game::camera::camera::tests::oc492_hooks;

    #[test]
    fn camera_trigger_man_load_matches() {
        oc492_hooks();
        check(
            ee::s02::f_0016cdc8__generated,
            camera_trigger_man_load,
            200,
            &|c, seed| {
                let mut r = Rng(0x492_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_cdc8;
                c.bus.write32(OC492_CTRL, OC492_MAGIC);
                // Result of CameraTriggerMan_LoadFile: nonzero on a third of the seeds.
                c.bus
                    .write32(OC492_CTRL + 4, if seed % 3 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(DATA + 0x1000);
                // The list: pointer array at +0, count at +4 (negative on some seeds).
                let count = match seed % 6 {
                    5 => u32::MAX,
                    k => k,
                };
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus.write32(DATA + 0x1004, count);
                for i in 0..4 {
                    c.bus
                        .write32(DATA + 0x2000 + 4 * i, DATA + 0x3000 + 0x40 * i);
                    c.bus.write32(DATA + 0x3000 + 0x40 * i + 0x20, r.next_u32());
                }
            },
        );
    }

    /// The `cCTBoundObjEllipse` (position, heading, roll, pitch, scale).
    pub(crate) const SRC: u32 = DATA;

    /// The `Vec4` the point is read from and written to.
    pub(crate) const POINT_2: u32 = DATA + 0x100;

    #[test]
    fn camera_ellipse_bound_obj_clamp_point_matches() {
        check(
            ee::s02::f_0016e560__generated,
            camera_ellipse_bound_obj_clamp_point,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0016_e560 ^ (seed as u64) << 20 ^ 0x2545_f491);
                scramble(c, &mut r);
                c.pc = 0x0016_e560;
                fill(c, &mut r, crate::difftest::SP - 0x200, 0x80);
                // A value in -m..m, as float bits.
                let sym = |r: &mut Rng, m: f32| {
                    (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
                };
                // Position in +-50, angles in +-7 rad, scale in 0.5..3.5.
                let pos: [f32; 3] = std::array::from_fn(|_| f32::from_bits(sym(&mut r, 50.0)));
                for (k, p) in pos.iter().enumerate() {
                    c.bus.write32(SRC + 4 * k as u32, p.to_bits());
                }
                for k in 0..3u32 {
                    c.bus.write32(SRC + 0xc + 4 * k, sym(&mut r, 7.0));
                }
                for k in 0..3u32 {
                    let s = 0.5 + (r.next_u32() as f32 / u32::MAX as f32) * 3.0;
                    c.bus.write32(SRC + 0x18 + 4 * k, s.to_bits());
                }
                // Even seeds put the point near the ellipsoid (inside), odd
                // seeds far away (outside, so the projection runs).
                let spread = if seed % 2 == 0 { 0.3 } else { 20.0 };
                let mut v = [0u32; 4];
                for k in 0..3 {
                    v[k] = (pos[k] + f32::from_bits(sym(&mut r, spread))).to_bits();
                }
                v[3] = r.next_u32();
                put_vec(c, POINT_2, v);
                c.g[A0][0] = sx(SRC);
                c.g[A1][0] = sx(POINT_2);
            },
        );
    }

    /// The `cCTBoundObjLine` the transform and the end points are read from.
    pub(crate) const LBCP_SRC: u32 = DATA;

    /// The `Vec4` the point is read from and written to.
    pub(crate) const LBCP_POINT: u32 = DATA + 0x100;

    #[test]
    fn camera_line_bound_obj_clamp_point_matches() {
        check(
            ee::s02::f_0016fbb8__generated,
            camera_line_bound_obj_clamp_point,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0016_fbb8 ^ (seed as u64) << 20 ^ 0x2545_f491);
                scramble(c, &mut r);
                c.pc = 0x0016_fbb8;
                fill(c, &mut r, crate::difftest::SP - 0x200, 0x80);
                // A float in -m..m, as bits.
                let sym = |r: &mut Rng, m: f32| {
                    (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
                };
                // Position, angles and end points in a small range.
                for k in 0..0x18u32 {
                    c.bus.write32(LBCP_SRC + 4 * k, sym(&mut r, 4.0));
                }
                // The three scale-like words are divided by: keep them positive.
                for k in 0..3u32 {
                    let s = 0.5 + (r.next_u32() as f32 / u32::MAX as f32) * 3.0;
                    c.bus.write32(LBCP_SRC + 0x18 + 4 * k, s.to_bits());
                }
                for k in 0..0x0cu32 {
                    c.bus.write32(LBCP_SRC + 0x24 + 4 * k, sym(&mut r, 4.0));
                }
                // The point: even seeds near the line, odd seeds far away.
                let spread = if seed % 2 == 0 { 0.3 } else { 20.0 };
                let mut v = [0u32; 4];
                for k in 0..3 {
                    v[k] = sym(&mut r, spread);
                }
                v[3] = r.next_u32();
                put_vec(c, LBCP_POINT, v);
                c.g[A0][0] = sx(LBCP_SRC);
                c.g[A1][0] = sx(LBCP_POINT);
            },
        );
    }

    /// `CameraTriggerMan_RemoveTriggerRegions_Wrapper` against the generated body.
    /// The count at +4 is zero, so the trigger loop is skipped and only the
    /// tail (stack and list resets) runs.
    #[test]
    fn camera_trigger_man_remove_trigger_regions_wrapper_matches() {
        check(
            ee::s02::f_0016cea8__generated,
            camera_trigger_man_remove_trigger_regions_wrapper,
            50,
            &|c, seed| {
                let mut r = Rng(0x497c_ea8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 4, 0);
                c.pc = 0x0016_cea8;
            },
        );
    }

    /// `CameraTriggerMan_EmptyStub`: only the return.
    #[test]
    fn camera_trigger_man_empty_stub_matches() {
        check(
            ee::s02::f_0016cf40__generated,
            camera_trigger_man_empty_stub,
            10,
            &|c, seed| {
                let mut r = Rng(0x497c_f40 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0016_cf40;
            },
        );
    }

    /// `CameraTriggerMan_UpdateTriggersMask1`: `UpdateTriggers(man, 1)`. The
    /// callee is the CameraTriggerMan fake from `cameratriggerman`'s tests,
    /// gated on that test module's machine magic (`oc438_machine`).
    #[test]
    fn camera_trigger_man_update_triggers_mask1_matches() {
        crate::game::camera::trigger::cameratriggerman::tests::oc438_install_fakes();
        crate::difftest::fakes::covered(
            ee::s02::f_0016d1d8__generated,
            camera_trigger_man_update_triggers_mask1,
            200,
            &|c, seed| {
                let mut r = crate::game::camera::trigger::cameratriggerman::tests::oc438_machine(
                    c,
                    seed,
                    0x0016_d1d8,
                    1,
                );
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_remove_trigger_regions_matches() {
        check(
            ee::s02::f_0016ca18__generated,
            camera_trigger_man_remove_trigger_regions,
            200,
            &|c, seed| {
                machine(c, seed, 0xca18, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0016ca18;
            },
        );
    }

    #[test]
    fn camera_trigger_apply_action_matches() {
        check(
            ee::s02::f_0016e1d8__generated,
            camera_trigger_apply_action,
            200,
            &|c, seed| {
                machine(c, seed, 0xe1d8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0016e1d8;
            },
        );
    }

    pub(crate) use crate::game::animation::rideranimbase::tests::OC1495_MAGIC;

    pub(crate) use crate::game::fe::ovstatereplay::tests::oc1493_install_fakes;

    pub(crate) use crate::game::fe::ovstatereplay::tests::OC1493_RT_GATE;
    pub(crate) use crate::game::fe::ovstatereplay::tests::OC1493_RT_MAGIC;

    // CameraTriggerMan_ReadTriggerAction fixture: the three callees are gated
    // fakes; the action's type word decides the path.
    pub(crate) const RT_GATE: u32 = DATA + 0x1_2000;

    pub(crate) const RT_ACTION: u32 = RT_GATE + 4;

    pub(crate) const RT_BOUND: u32 = RT_GATE + 8;

    pub(crate) const RT_MAGIC: u32 = 0x1492_0b0b;

    pub(crate) static OC1492_RT_PREV_ACTION: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1492_RT_PREV_BOUND: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1492_RT_PREV_SPLINE: AtomicUsize = AtomicUsize::new(0);

    /// `CameraTrigger_ReadAction`: the forced result in v0.
    pub(crate) fn oc1492_rt_action(c: &mut Cpu) {
        if c.bus.read32(RT_GATE) != RT_MAGIC {
            return fakes::fall_back(c, &OC1492_RT_PREV_ACTION, ee::s02::f_001712b0__generated);
        }
        let v = c.bus.read32(RT_ACTION);
        c.g[V0][0] = v as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `CameraTriggerMan_ReadBoundObjAction`: the forced result in v0.
    pub(crate) fn oc1492_rt_bound(c: &mut Cpu) {
        if c.bus.read32(RT_GATE) != RT_MAGIC {
            return fakes::fall_back(c, &OC1492_RT_PREV_BOUND, ee::s02::f_0016c4f0__generated);
        }
        let v = c.bus.read32(RT_BOUND);
        c.g[V0][0] = v as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `CameraTrigger_ReadSpline`: nothing to write in this fixture.
    pub(crate) fn oc1492_rt_spline(c: &mut Cpu) {
        if c.bus.read32(RT_GATE) != RT_MAGIC {
            return fakes::fall_back(c, &OC1492_RT_PREV_SPLINE, ee::s02::f_00171d58__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn camera_trigger_man_read_trigger_action_matches() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001712B0),
            oc1492_rt_action,
            &OC1492_RT_PREV_ACTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C4F0),
            oc1492_rt_bound,
            &OC1492_RT_PREV_BOUND,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00171D58),
            oc1492_rt_spline,
            &OC1492_RT_PREV_SPLINE,
        );
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x16_c518, 0);
            c.bus.write32(RT_GATE, RT_MAGIC);
            c.bus.write32(RT_ACTION, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(RT_BOUND, 0x7000 + seed);
            // *actionOut = the action; the action's type word picks the path.
            c.bus.write32(DATA + 0x100, DATA + 0x200);
            c.bus
                .write32(DATA + 0x200, [1, 2, 3][((seed / 3) % 3) as usize]);
            c.g[A0][0] = sx(DATA + 0x600);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = seed as u64;
        };
        check(
            ee::s02::f_0016c518__generated,
            camera_trigger_man_read_trigger_action,
            64,
            setup,
        );
    }

    /// `CameraTriggerMan_ReadTrigger`: the two get_uint reads, the volume and
    /// the two action reads are fakes (answers from the gate words); the
    /// statuses cover every early exit.
    #[test]
    fn camera_trigger_man_read_trigger_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s02::f_0016c428__generated,
            camera_trigger_man_read_trigger,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x16c428, 3);
                c.bus.write32(OC1493_RT_GATE, OC1493_RT_MAGIC);
                let nonzero = |r: &mut Rng| r.next_u32() | 1;
                c.bus.write32(
                    OC1493_RT_GATE + 0x8,
                    if seed % 3 == 0 { 0 } else { nonzero(&mut r) },
                );
                c.bus.write32(
                    OC1493_RT_GATE + 0xc,
                    if seed % 4 == 0 { 0 } else { nonzero(&mut r) },
                );
                c.bus.write32(
                    OC1493_RT_GATE + 0x10,
                    if seed % 5 == 0 { 0 } else { nonzero(&mut r) },
                );
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = sx(r.next_u32());
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// Gate and mailbox words of the test fakes (one pair per fake: a fake's
    /// gate must be its own word, see the writer brief).
    pub(crate) const OC1494_MAGIC: u32 = 0x1494_0c0d;

    pub(crate) const OC1494_SPHERE_GATE: u32 = DATA + 0x2000;

    pub(crate) const OC1494_SPHERE_RESULT: u32 = DATA + 0x2004;

    pub(crate) static OC1494_PREV_SPHERE: AtomicUsize = AtomicUsize::new(0);

    /// `CameraTrigger_ReadSphereTrigger` (0x172e18) with a forced "volume read"
    /// result: 0 or 1 in v0 (the real body allocates and reads the cursor).
    pub(crate) fn oc1494_fake_sphere(c: &mut Cpu) {
        if c.bus.read32(OC1494_SPHERE_GATE) != OC1494_MAGIC {
            return fakes::fall_back(c, &OC1494_PREV_SPHERE, ee::s02::f_00172e18__generated);
        }
        let ra = c.g[RA][0] as u32;
        let read = c.bus.read32(OC1494_SPHERE_RESULT) != 0;
        fakes::fake(c, 0x0017_2e18, ra);
        if c.pc == ra {
            c.g[V0][0] = read as u64;
        }
    }

    #[test]
    fn camera_trigger_man_read_trigger_volume_matches() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00172E18),
            oc1494_fake_sphere,
            &OC1494_PREV_SPHERE,
        );
        // calls = 1: the fake may unwind, which the wrapper must pass through.
        check(
            ee::s02::f_0016c4c8__generated,
            camera_trigger_man_read_trigger_volume,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x0016_c4c8, 1);
                c.bus.write32(OC1494_SPHERE_GATE, OC1494_MAGIC);
                c.bus.write32(OC1494_SPHERE_RESULT, seed & 1);
            },
        );
    }

    pub(crate) const OC1495_BOUND_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) static OC1495_PREV_BOUND_OBJ: AtomicUsize = AtomicUsize::new(0);

    /// `CameraTrigger_ReadBoundObj`: returns the mailbox word in `v0`.
    pub(crate) fn oc1495_fake_bound_obj(c: &mut Cpu) {
        if c.bus.read32(OC1495_BOUND_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_BOUND_OBJ, ee::s02::f_00171690__generated);
        }
        c.cycles += 3;
        c.g[V0][0] = c.bus.read32(OC1495_BOUND_CTRL + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc1495_install_bound_obj() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00171690),
            oc1495_fake_bound_obj,
            &OC1495_PREV_BOUND_OBJ,
        );
    }

    /// `CameraTriggerMan_ReadBoundObjAction`: the bound object result is 0 or
    /// not, both give the same return value.
    #[test]
    fn camera_trigger_man_read_bound_obj_action_matches() {
        check(
            ee::s02::f_0016c4f0__generated,
            camera_trigger_man_read_bound_obj_action,
            300,
            &|c, seed| {
                oc1495_install_bound_obj();
                let mut r = machine(c, seed, 0x0016_c4f0, 0);
                c.bus.write32(OC1495_BOUND_CTRL, OC1495_MAGIC);
                c.bus.write32(
                    OC1495_BOUND_CTRL + 4,
                    if seed % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }
}
