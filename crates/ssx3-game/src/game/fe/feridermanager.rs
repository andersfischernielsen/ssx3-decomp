//! `src/fe/feridermanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::fe::debugfe::menu_sx_add;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw;
pub(crate) use crate::game::fe::fepopup::sb;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::sound::bankmonitor::o760_addi;
pub(crate) use crate::game::sound::bankmonitor::o760_ea;
pub(crate) use crate::game::sound::bankmonitor::o760_lw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_e680,
        fe_module_rider_preview_find_bones,
        "FEModule_RiderPreviewFindBones",
    ),
    (
        0x0019_eba0,
        fe_module_rider_preview_unload,
        "FEModule_RiderPreviewUnload",
    ),
    (
        0x0019_f3e8,
        femodule_rider_preview_set_transform,
        "FEModule_RiderPreviewSetTransform",
    ),
    (
        0x0019_f548,
        femodule_rider_preview_set_transform2,
        "FEModule_RiderPreviewSetTransform2",
    ),
    (
        0x0019_f908,
        fe_module_rider_preview_load_shape_textures,
        "FEModule_RiderPreviewLoadShapeTextures",
    ),
    (
        0x001a_03c0,
        fe_module_init_rider_previews,
        "FEModule_InitRiderPreviews",
    ),
    (
        0x001a_0498,
        femodule_set_rider_preview_indices,
        "FEModule_SetRiderPreviewIndices",
    ),
    (
        0x001a_04c8,
        fe_module_unload_rider_previews,
        "FEModule_UnloadRiderPreviews",
    ),
    (
        0x001a_0508,
        fe_module_request_preview_char,
        "FEModule_RequestPreviewChar",
    ),
    (
        0x001a_0538,
        femodule_get_rider_preview_2,
        "FEModule_GetRiderPreview_2",
    ),
    (
        0x001a_0548,
        femodule_get_rider_preview,
        "FEModule_GetRiderPreview",
    ),
    (
        0x001a_0570,
        fe_module_set_preview_visible,
        "FEModule_SetPreviewVisible",
    ),
    (
        0x001a_0598,
        fe_module_render_rider_previews,
        "FEModule_RenderRiderPreviews",
    ),
    (
        0x001a_0608,
        fe_module_update_rider_previews,
        "FEModule_UpdateRiderPreviews",
    ),
    (
        0x0038_9cb8,
        irradiance_add_camera_relative_light,
        "Irradiance_AddCameraRelativeLight",
    ),
    (
        0x0019_ee88,
        fe_module_rider_preview_render,
        "FEModule_RiderPreviewRender",
    ),
    (
        0x0019_f2d0,
        fe_module_rider_preview_apply_transform,
        "FEModule_RiderPreviewApplyTransform",
    ),
    (
        0x001d_cb70,
        fe_module_get_rider_preview_2_thunk,
        "FEModule_GetRiderPreview_2_Thunk",
    ),
    (
        0x001d_cb90,
        fe_module_get_rider_preview_thunk,
        "FEModule_GetRiderPreview_Thunk",
    ),
    (
        0x001a_0100,
        fe_module_rider_preview_poll_models,
        "FEModule_RiderPreviewPollModels",
    ),
    (
        0x0019_e960,
        fe_module_rider_preview_load_char,
        "FEModule_RiderPreviewLoadChar",
    ),
    (
        0x0019_ec68,
        fe_module_rider_preview_release_bolt_files,
        "FEModule_RiderPreviewReleaseBoltFiles",
    ),
    (
        0x0019_ed80,
        fe_module_rider_preview_setup_model,
        "FEModule_RiderPreviewSetupModel",
    ),
    (
        0x0019_f780,
        fe_module_rider_preview_apply_bolt_materials,
        "FEModule_RiderPreviewApplyBoltMaterials",
    ),
    (
        0x0019_f878,
        fe_module_rider_preview_find_texture_2,
        "FEModule_RiderPreviewFindTexture",
    ),
    (
        0x0019_fa78,
        fe_module_rider_preview_release_bolt_textures,
        "FEModule_RiderPreviewReleaseBoltTextures",
    ),
    (
        0x0019_fbe0,
        fe_module_rider_preview_request_bolt_textures,
        "FEModule_RiderPreviewRequestBoltTextures",
    ),
    (
        0x0019_fd58,
        fe_module_rider_preview_poll_textures,
        "FEModule_RiderPreviewPollTextures",
    ),
    (
        0x0019_ff00,
        fe_module_rider_preview_request_bolt_models,
        "FEModule_RiderPreviewRequestBoltModels",
    ),
    (
        0x001a_0358,
        fe_module_rider_preview_play_pending_speech,
        "FEModule_RiderPreviewPlayPendingSpeech",
    ),
    (
        0x001a_0420,
        fe_module_destroy_rider_previews_2,
        "FEModule_DestroyRiderPreviews",
    ),
    (
        0x0019_e7f0,
        femodule_rider_preview_request_extra_textures,
        "FEModule_RiderPreviewRequestExtraTextures",
    ),
    (
        0x0019_f418,
        femodule_rider_preview_move_local,
        "FEModule_RiderPreviewMoveLocal",
    ),
    (
        0x0019_f578,
        femodule_rider_preview_move_local2,
        "FEModule_RiderPreviewMoveLocal2",
    ),
    (
        0x0019_f740,
        femodule_rider_preview_get_anim_id,
        "FEModule_RiderPreviewGetAnimId",
    ),
    (
        0x0019_f760,
        femodule_rider_preview_get_anim_progress,
        "FEModule_RiderPreviewGetAnimProgress",
    ),
    (
        0x0019_f138,
        fe_rider_preview_update,
        "FEModule_RiderPreviewUpdate",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;

    pub(crate) use crate::game::fe::fepopup::callee::C0019F878;

    pub(crate) use crate::game::fe::fepopupmisc::callee::FE_MODULE_RIDER_PREVIEW_CTOR;

    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;

    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

/// `RiderPreview* FEModule_GetRiderPreview_2(FEModule *self, int index)`
/// (0x1a0538): `self + index*0xce0`, the preview record array.
pub fn femodule_get_rider_preview_2(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xce0;
    c.mult(0, A2, A1, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    abi::ret(c);
}

/// `RiderPreview* FEModule_GetRiderPreview(FEModule *self, int index)`
/// (0x1a0548): identical to [`femodule_get_rider_preview_2`].
pub fn femodule_get_rider_preview(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xce0;
    c.mult(0, A2, A1, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end module: preview visibility.

/// `void FEModule_SetPreviewVisible(self, int rider, int visible)`
/// (0x1a0570): forward to the rider preview object at `self + rider*0xce0`.
pub fn fe_module_set_preview_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0xce0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = sx(arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x001a_058c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RiderPreviewSetTransform(FEModule* self, sQuadRM* src)`
/// (0x19f3e8): copy the two quads into `+0xc30`/`+0xc40` and apply them.
pub fn femodule_rider_preview_set_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.lq(3, abi::arg(c, 1));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        let dst = c.g[A2][0] as u32;
        c.sq(3, at(dst, 0xc30));
        c.lq(2, at(abi::arg(c, 1), 0x10));
        c.sq(2, at(dst, 0xc40));
        callee::RIDER_PREVIEW_APPLY_TRANSFORM.call(c, 0x0019_f408)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_RiderPreviewSetTransform2(FEModule* self, sQuadRM* src)`
/// (0x19f548): the second pair of quads (`+0xc50`/`+0xc60`).
pub fn femodule_rider_preview_set_transform2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.lq(3, abi::arg(c, 1));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        let dst = c.g[A2][0] as u32;
        c.sq(3, at(dst, 0xc50));
        c.lq(2, at(abi::arg(c, 1), 0x10));
        c.sq(2, at(dst, 0xc60));
        callee::RIDER_PREVIEW_APPLY_TRANSFORM.call(c, 0x0019_f568)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_SetRiderPreviewIndices(FEModule* self)` (0x1a0498): number
/// the two rider preview slots (`FEModule` stride 0xce0, index at `+0x4`).
pub fn femodule_set_rider_preview_indices(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        let v = c.g[V1][0];
        Addr(at(a, 4)).sw(c, v);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(a.wrapping_add(0xce0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001a_04a0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front end: preview characters.

/// `FEModule_RequestPreviewChar(module, rider, a2, a3)` 0x1a0508: forward to
/// rider `idx`'s preview object (`module + idx * 0xce0`), passing `a2`/`a3` as
/// the request's character and flag.
pub fn fe_module_request_preview_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let module = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        // The arguments are moved, not truncated: `a1 = a2; a2 = a3`.
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        c.g[V0][0] = 0xce0;
        let frame = Frame::open(c, 0x10);
        // `mult v1, a1, v0`: the byte stride to the rider's preview object.
        let stride = abi::mult(c, idx, 0xce0);
        c.g[V1][0] = sx(stride);
        frame.save_ra(c, 0);
        c.g[A1][0] = a2;
        c.g[A2][0] = a3;
        c.g[A0][0] = sx(module.wrapping_add(stride));
        callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR.call(c, 0x001a_0528)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider previews: two slots 0xce0 bytes apart.

/// `void FEModule_UnloadRiderPreviews(FEModule* m)` (0x1a04c8): unload the
/// two rider preview slots at `+0x19c0` and `+0x26a0` (the loop the compiler
/// left, with an event check at the back edge).
pub fn fe_module_unload_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(crate::abi::r32(c, S0), 0x19c0));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_MODULE_RIDER_PREVIEW_UNLOAD.call(c, 0x001a_04e8)?;
            c.g[S0][0] = sx(p(crate::abi::r32(c, S0), 0xce0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_04e0);
            }
        }
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_UpdateRiderPreviews(FEModule* m)` (0x1a0608): the same
/// walk with `FEModule_RiderPreviewUpdate`.
pub fn fe_module_update_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(crate::abi::r32(c, S0), 0x19c0));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_MODULE_RIDER_PREVIEW_UPDATE.call(c, 0x001a_0628)?;
            c.g[S0][0] = sx(p(crate::abi::r32(c, S0), 0xce0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_0620);
            }
        }
        frame.close(c);
        Ok(())
    })
}

/// `void* FEModule_InitRiderPreviews(FEModule* self)` (0x1a03c0): construct
/// the two rider-preview objects 0xce0 bytes apart; returns `self`.
pub fn fe_module_init_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[S2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            callee::FE_MODULE_RIDER_PREVIEW_CTOR.call(c, 0x001a_03f4)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xce0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_03e8);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RenderRiderPreviews(FEModule* self)` (0x1a0598): point the
/// render targets at_2 the FE canvas and render both rider previews.
pub fn fe_module_render_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gpr(c, -0x854)).lw(c);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x2e;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2f;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(at_2(c, V0, 0x24)).sw(c, c.g[V1][0]);
        c.g[S1][0] = sx(at_2(c, S0, 0x19c0));
        c.g[A1][0] = Addr(at_2(c, V0, 0x1008)).lw(c);
        Addr(at_2(c, V0, 0x84)).sw(c, c.g[A2][0]);
        Addr(at_2(c, V0, 0x10a8)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(at_2(c, S0, 0xcc8)).lw(c);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = sx(at_2(c, S0, 0xce0));
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0019_ee88, 0x001a_05e0)?;
                c.g[S0][0] = sx(at_2(c, S0, 0xce0));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[V0][0] = Addr(at_2(c, S0, 0xcc8)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x001a_05d0);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// FE module teardown, text scroll and group focus.

// ---------------------------------------------------------------------------
// Front-end rider preview.

/// `FEModule_RiderPreviewUnload(this)` (0x19eba0): release the preview's
/// model and its three sub-objects, clear the handles and reset the state.
pub fn fe_module_rider_preview_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[A0][0] = Addr(this).off(8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR.call(c, 0x0019_ebc4)?;
        }
        c.g[A2][0] = Addr(this).off(0xc).lw(c);

        // The three sub-objects (offsets 0xc, 0xc74, 0xc70).
        c.cycles += 2;
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x58).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ebe4)?;
        }

        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0xc74).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xc4).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ec04)?;
        }

        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0xc70).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xa4).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ec24)?;
        }

        c.cycles += 16;
        Addr(this).off(8).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(this).off(0xc).sw(c, 0);
        Addr(this).off(0xc70).sw(c, 0);
        Addr(this).off(0xc74).sw(c, 0);
        callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES.call(c, 0x0019_ec3c)?;
        c.g[V0][0] = sx(0xffff_ffff_u32);
        Addr(this).off(0xcc8).sw(c, 0);
        Addr(this).off(0xcd4).sw(c, c.g[V0][0]);
        Addr(this).sw(c, c.g[V0][0]);
        Addr(this).off(4).sw(c, c.g[V0][0]);
        Addr(this).off(0xc78).sw(c, 0);

        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RiderModel_FindBone(RiderModel*, int bucket, const char *name)`.
pub(crate) const RIDER_MODEL_FIND_BONE: u32 = 0x0031_0c48;

// ---------------------------------------------------------------------------
// Rider preview bones.

/// The bone-name addresses `FEModule_RiderPreviewFindBones` looks up and the
/// second argument each is found with (`.rodata` strings).
pub(crate) const PREVIEW_BONE_NAMES: [(u32, u64); 14] = [
    (0x004a_1970, 0),
    (0x004a_1978, 0),
    (0x0046_0a00, 0),
    (0x0046_0a10, 0),
    (0x0046_0a20, 0),
    (0x0046_0a30, 0),
    (0x0046_0a40, 0),
    (0x0046_0a50, 0),
    (0x0046_0a60, 0),
    (0x0046_0a70, 0),
    (0x0046_0a80, 0),
    (0x0046_0a90, 0),
    (0x0046_0aa0, 1),
    (0x0046_0ab0, 1),
];

/// `void FEModule_RiderPreviewFindBones(FEModule *m)` (0x19e680): resolve the
/// preview rider's bones by name and cache the indices at `+0xc7c..+0xcb0`.
pub fn fe_module_rider_preview_find_bones(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 92;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;

        for (i, &(name, second)) in PREVIEW_BONE_NAMES.iter().enumerate() {
            c.g[A2][0] = name as u64;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A1][0] = second;
            call_ptr(c, RIDER_MODEL_FIND_BONE, 0x0019_e6a4 + 0x18 * i as u32)?;
            Addr(r32(c, S0).wrapping_add(0xc7c + 4 * i as u32)).sw(c, c.g[V0][0]);
        }

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_0019F908(void* self, sPkg* pkg, int replace)` (0x19f908): load a
/// rider preview's shape textures: for every entry of the package, bind the
/// data to the already-known model id (replace) or append the entry's name
/// and a fresh id to `self`'s table.
pub fn fe_module_rider_preview_load_shape_textures(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_19fa48: {
        c.cycles += 17;
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x60);
        c.g[S2][0] = 0u64;
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        sq(c, 30, at_3(c, SP, 0x20));
        frame.save_ra(c, 0x10);
        frame.save(c, S5, 0x50);
        c.g[S5][0] = lw(c, at_3(c, S4, 0x8));
        let t = (c.g[S5][0] as i64) <= 0;
        c.g[S7][0] = c.g[A2][0];
        if t {
            break 'b_19fa48;
        } else {
            c.cycles += 3;
            c.g[S6][0] = 0xc;
            c.g[S3][0] = sx(at_3(c, S4, 0x14));
            c.g[FP][0] = 0x4a0000;
            'l_19f958: loop {
                'b_19fa3c: {
                    'b_19fa38: {
                        'b_19f9dc: {
                            c.cycles += 21;
                            c.g[V0][0] = lw(c, r32(c, S3));
                            c.g[V1][0] = 0x4a0000;
                            c.g[A0][0] = sx(at_3(c, SP, 0x2));
                            c.g[A1][0] = 0u64;
                            c.g[S1][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                            c.g[T1][0] = sx(at_3(c, V1, 0x19a8));
                            c.g[V0][0] = lb(c, r32(c, T1));
                            c.g[A3][0] = lb(c, at_3(c, T1, 0x1));
                            sb(c, r32(c, SP), c.g[V0][0]);
                            sb(c, at_3(c, SP, 0x1), c.g[A3][0]);
                            c.g[RA][0] = sx(0x0019f988);
                            c.g[A2][0] = 0x6;
                            if call_ptr(c, 0x00416210, 0x0019f988).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x0019f998);
                            c.g[A2][0] = c.g[SP][0];
                            if call_ptr(c, 0x003b3d00, 0x0019f998).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0019f9a4);
                            c.g[A1][0] = c.g[SP][0];
                            if callee::C0019F878.call(c, 0x0019f9a4).is_err() {
                                return;
                            }
                            if (c.g[V0][0] as i64) < 0 {
                                c.g[A0][0] = lw(c, at_3(c, S0, 0x18));
                                break 'b_19f9dc;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[T0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                if t {
                                    break 'b_19fa38;
                                } else {
                                    c.cycles += 10;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = 0x1;
                                    c.g[V0][0] = lw(c, at_3(c, T0, 0x10d8));
                                    c.g[A0][0] = lh(c, at_3(c, V0, 0x180));
                                    c.g[V1][0] = lw(c, at_3(c, V0, 0x184));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019f9d4);
                                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x0019f9d4).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    c.g[S2][0] = sx(at_3(c, S2, 0x1));
                                    if t {
                                        break 'b_19fa3c;
                                    } else {
                                        break 'b_19f9dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 23;
                        c.g[A1][0] = c.g[SP][0];
                        c.mult(0, V0, A0, S6);
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                        c.g[RA][0] = sx(0x0019f9f0);
                        c.g[A0][0] = sx(at_3(c, A0, 0x20));
                        if call_ptr(c, 0x004166f4, 0x0019f9f0).is_err() {
                            return;
                        }
                        c.g[T2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = sx(at_3(c, FP, 0x19b0));
                        c.g[A3][0] = 0u64;
                        c.g[V1][0] = lw(c, at_3(c, T2, 0x10d8));
                        c.g[T0][0] = 0x1;
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = lh(c, at_3(c, V1, 0x170));
                        c.g[V0][0] = lw(c, at_3(c, V1, 0x174));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0019fa1c);
                        c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x0019fa1c).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at_3(c, S0, 0x18));
                        c.mult(0, A0, V1, S6);
                        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
                        sw(c, at_3(c, V1, 0x1c), c.g[V0][0]);
                        c.g[V0][0] = lw(c, at_3(c, S0, 0x18));
                        c.g[V0][0] = sx(at_3(c, V0, 0x1));
                        sw(c, at_3(c, S0, 0x18), c.g[V0][0]);
                        break 'b_19fa38;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = sx(at_3(c, S2, 0x1));
                    break 'b_19fa3c;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = sx(at_3(c, S3, 0x8));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019f958);
                    }
                    continue 'l_19f958;
                } else {
                    break 'b_19fa48;
                }
            }
        }
    }
    c.cycles += 12;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at_3(c, SP, 0x20));
    c.g[RA][0] = ld(c, at_3(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0xb0));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Irradiance_AddCameraRelativeLight (0x389cb8).

/// Light direction (Vec4) the sun's elevation is measured against.
pub(crate) const CRL_UP: u32 = 0x004f_f140;

/// Rotation axes of the azimuth and elevation rotations (Vec3 each).
pub(crate) const CRL_AXIS_1: u32 = 0x004f_f150;

pub(crate) const CRL_AXIS_2: u32 = 0x004f_f160;

/// A quadword shuffled through the stack around the two `Math_SinCos` calls.
pub(crate) const CRL_SWAP: u32 = 0x004f_f130;

/// The base orientation the angle rotations are applied to (4x4).
pub(crate) const CRL_BASE: u32 = 0x004f_f1a0;

/// The axis-swap matrix built once (4x4) and its "built" flag (`gp+0x28b4`).
pub(crate) const CRL_SWAP_MATRIX: u32 = 0x0050_0c60;

pub(crate) const CRL_SWAP_BUILT: i32 = 0x28b4;

/// The camera object (`gp+0x2a90`), its vtable (+0x10d8) and the
/// "get view matrix" slot (this-delta at +0x118, method at +0x11c).
pub(crate) const CRL_CAMERA: i32 = 0x2a90;

pub(crate) const CRL_CAMERA_VT: u32 = 0x10d8;

pub(crate) const CRL_VIEW_MATRIX_SLOT: u32 = 0x118;

/// `gp + off`.
#[inline(always)]
pub(crate) fn crl_gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `sp + off`.
#[inline(always)]
pub(crate) fn crl_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// Load a 4x4 matrix into vf[base]..vf[base+3].
#[inline(always)]
pub(crate) fn crl_load_mat(c: &mut Cpu, base: usize, m: u32) {
    for i in 0..4 {
        c.lqc2(base + i, m.wrapping_add(0x10 * i as u32));
    }
}

/// Store vf[base]..vf[base+3] as a 4x4 matrix.
#[inline(always)]
pub(crate) fn crl_store_mat(c: &mut Cpu, base: usize, m: u32) {
    for i in 0..4 {
        c.sqc2(base + i, m.wrapping_add(0x10 * i as u32));
    }
}

/// A matrix copy through vf1-vf4.
#[inline(always)]
pub(crate) fn crl_copy_mat(c: &mut Cpu, dst: u32, src: u32) {
    crl_load_mat(c, 1, src);
    crl_store_mat(c, 1, dst);
}

/// vf[out] = vf4 * v.x + vf5 * v.y + vf6 * v.z + vf7 * v.w (v = vf[v]).
#[inline(always)]
pub(crate) fn crl_mat_vec(c: &mut Cpu, out: u8, v: u8) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    c.vu0().mula(0b1111, 4, Src::Bc(v, X));
    c.vu0().madda(0b1111, 5, Src::Bc(v, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(v, Z));
    c.vu0().madd(0b1111, out, 7, Src::Bc(v, W));
}

/// `dst = a * b` (vf4-7 = a, vf8-11 = b, product rows in vf12-15).
#[inline(always)]
pub(crate) fn crl_mat_mul(c: &mut Cpu, dst: u32, a: u32, b: u32) {
    crl_load_mat(c, 4, a);
    crl_load_mat(c, 8, b);
    for i in 0..4 {
        crl_mat_vec(c, 12 + i, 8 + i);
    }
    crl_store_mat(c, 12, dst);
}

/// vf4.x = the four-lane dot product of vf[a] and vf[b] (vf6.x = 1).
#[inline(always)]
pub(crate) fn crl_dot4(c: &mut Cpu, a: u8, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, a, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// In-place transpose of the 4x4 matrix at `m` through GPRs: rows in
/// r+3, r+2, r+1, r; interleaves in r-1..r-4 (`pextlw/pextuw/pcpyld/pcpyud`).
#[inline(always)]
pub(crate) fn crl_transpose(c: &mut Cpu, m: u32, r: usize) {
    let rows = [r + 3, r + 2, r + 1, r];
    let t = [r - 1, r - 2, r - 3, r - 4];
    for (i, &row) in rows.iter().enumerate() {
        c.lq(row, m.wrapping_add(0x10 * i as u32));
    }
    c.pextlw(t[0], rows[1], rows[0]);
    c.pextuw(t[1], rows[1], rows[0]);
    c.pextlw(t[2], rows[3], rows[2]);
    c.pextuw(t[3], rows[3], rows[2]);
    c.pcpyld(rows[0], t[2], t[0]);
    c.pcpyud(rows[1], t[0], t[2]);
    c.pcpyld(rows[2], t[3], t[1]);
    c.pcpyud(rows[3], t[1], t[3]);
    for (i, &row) in rows.iter().enumerate() {
        c.sq(row, m.wrapping_add(0x10 * i as u32));
    }
}

/// The axis-angle rotation (Rodrigues) from `Math_SinCos`'s results at
/// `sp+sin_off` / `sp+cos_off` about the axis at `s1` (f20 = 1), written
/// as rows into the matrix at sp+0x120 (w lanes 0); sp+0x150 gets the
/// quadword from sp+0x1a0 (the row (0,0,0,1) stored first is overwritten).
pub(crate) fn crl_axis_rotation(c: &mut Cpu, sin_off: u32, cos_off: u32) {
    use crate::ps2f::FpuF32;
    let axis = r32(c, S1);
    let one = FpuF32(c.f[20]);
    c.lwc1(11, crl_sp(c, cos_off));
    let cs = FpuF32(c.f[11]);
    c.lwc1(3, axis.wrapping_add(8));
    let z = FpuF32(c.f[3]);
    c.lwc1(0, axis);
    let x = FpuF32(c.f[0]);
    c.lwc1(7, axis.wrapping_add(4));
    let y = FpuF32(c.f[7]);
    c.lwc1(8, crl_sp(c, sin_off));
    let sn = FpuF32(c.f[8]);
    let t = one - cs;
    let (tz, tx, ty) = (t * z, t * x, t * y);
    let (sz, sx_, sy) = (sn * z, sn * x, sn * y);
    c.lq(V0, crl_sp(c, 0x1a0));
    // Row 3 = (0, 0, 0, 1), then overwritten by the quadword just loaded.
    c.sw(crl_sp(c, 0x150), 0);
    c.sw(crl_sp(c, 0x154), 0);
    c.sw(crl_sp(c, 0x158), 0);
    c.swc1(20, crl_sp(c, 0x15c));
    c.sq(V0, crl_sp(c, 0x150));
    c.sw(crl_sp(c, 0x12c), 0);
    c.sw(crl_sp(c, 0x13c), 0);
    c.sw(crl_sp(c, 0x14c), 0);
    let m01 = ty * x - sz; // f0
    let m02 = tz * x + sy; // f5
    let m00 = tx * x + cs; // f6
    let m10 = tx * y + sz; // f4
    let m11 = ty * y + cs; // f1
    let m12 = tz * y - sx_; // f2
    let m20 = tx * z - sy; // f10
    let m21 = ty * z + sx_; // f3
    let m22 = tz * z + cs; // f9
    for (f, v) in [
        (0, m01),
        (1, m11),
        (2, m12),
        (3, m21),
        (4, m10),
        (5, m02),
        (6, m00),
        (8, sy),
        (9, m22),
        (10, m20),
        (12, sx_),
        (13, sz),
    ] {
        c.f[f] = v.0;
    }
    for (f, off) in [
        (5, 0x128),
        (0, 0x124),
        (2, 0x138),
        (9, 0x148),
        (6, 0x120),
        (4, 0x130),
        (10, 0x140),
        (1, 0x134),
        (3, 0x144),
    ] {
        let a = crl_sp(c, off);
        c.swc1(f, a);
    }
}

/// `void Irradiance_AddCameraRelativeLight(Irradiance* irr, const Vec4*
/// sunPos, float scale)` (0x389cb8): add the sun as a zonal SH light in
/// camera space. Builds a z-axis lobe at sp (w lanes of the 10 entries),
/// gets the camera's view matrix (virtual call), turns the sun position
/// into a camera-relative unit direction, derives its azimuth
/// (`Math_Atan`) and elevation (`Math_Asin` of the cosine against
/// `CRL_UP`, made an angle from the pole; mirrored for negative y),
/// rotates the base orientation by both angles (`Math_SinCos` + two
/// axis-angle rotations), transposes, applies the axis-swap matrix
/// (built on first use), rotates the lobe by the 3x3 part
/// (`Irradiance_Rotate`, order 3) and adds it to `irr` scaled by
/// `scale * k` (`Irradiance_AddScaledW`).
pub fn irradiance_add_camera_relative_light(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        // Prologue: frame 0x2e0, s0-s6 / ra / f20-f24 saved, the lobe's w
        // lanes (0, 0x10 ... 0x90).
        c.cycles += 128;
        let frame = Frame::open(c, 0x2e0);
        c.g[V0][0] = 0x004a_0000;
        frame.save(c, S0, 0x2a0);
        c.lwc1(3, 0x004a_43c0);
        c.g[S0][0] = c.g[A1][0];
        c.lwc1(2, crl_gp(c, -0x2700));
        c.lwc1(1, crl_gp(c, -0x26fc));
        c.lwc1(0, crl_gp(c, -0x26f8));
        frame.save(c, S6, 0x240);
        frame.save_fpr(c, 24, 0x2d0);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x290);
        c.f[24] = c.f[12];
        frame.save(c, S2, 0x280);
        frame.save(c, S3, 0x270);
        frame.save(c, S4, 0x260);
        frame.save(c, S5, 0x250);
        frame.save_ra(c, 0x230);
        frame.save_fpr(c, 23, 0x2c8);
        frame.save_fpr(c, 22, 0x2c0);
        frame.save_fpr(c, 21, 0x2b8);
        frame.save_fpr(c, 20, 0x2b0);
        for (f, off) in [(2, 0x0c), (3, 0x2c), (1, 0x3c), (0, 0x7c)] {
            let a = crl_sp(c, off);
            c.swc1(f, a);
        }
        for off in [0x1c, 0x4c, 0x5c, 0x6c, 0x8c, 0x9c] {
            let a = crl_sp(c, off);
            c.sw(a, 0);
        }

        // view = camera->GetViewMatrix() (virtual, this-adjusted).
        c.g[A1][0] = c.lw(crl_gp(c, CRL_CAMERA));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(CRL_CAMERA_VT));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(CRL_VIEW_MATRIX_SLOT));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(CRL_VIEW_MATRIX_SLOT + 4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0038_9d50)?;

        // Camera position: -(view^T * view.row3), w = 1 (sp+0xe0).
        let view = sp(c).wrapping_add(0xa0);
        c.g[V1][0] = sx(view);
        crl_copy_mat(c, view, r32(c, V0));
        c.lq(V0, crl_sp(c, 0xd0));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[21] = 0xbf80_0000;
        c.sq(V0, crl_sp(c, 0xf0));
        c.mfc1(A0, 21);
        c.lqc2(4, crl_sp(c, 0xf0));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, crl_sp(c, 0x100));
        c.lq(V0, crl_sp(c, 0x100));
        c.sq(V0, crl_sp(c, 0xe0));
        crl_transpose(c, view, 8);
        c.lqc2(8, crl_sp(c, 0xe0));
        crl_load_mat(c, 4, view);
        crl_mat_vec(c, 12, 8);
        c.sqc2(12, crl_sp(c, 0x100));
        c.g[AT][0] = 0x3f80_0000;
        c.f[22] = 0x3f80_0000;
        c.lq(V0, crl_sp(c, 0x100));
        c.sq(V0, crl_sp(c, 0xe0));
        c.sq(V0, crl_sp(c, 0xf0));
        c.swc1(22, crl_sp(c, 0xec));

        // dir = normalize(sunPos - camera) (sp+0xf0); f12 = dir.z, then
        // dir.z = 0 and f20 = |dir.xy|; f12 = dir.z / |dir.xy|.
        c.lqc2(3, r32(c, S0));
        c.lqc2(4, crl_sp(c, 0xe0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, crl_sp(c, 0x110));
        c.lq(V0, crl_sp(c, 0x110));
        c.sq(V0, crl_sp(c, 0x100));
        c.lqc2(3, crl_sp(c, 0x100));
        crl_dot4(c, 3, 3);
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, crl_sp(c, 0x110));
        c.lq(V0, crl_sp(c, 0x110));
        c.sq(V0, crl_sp(c, 0xf0));
        c.lwc1(12, crl_sp(c, 0xf8));
        c.sw(crl_sp(c, 0xf8), 0);
        c.lqc2(3, crl_sp(c, 0xf0));
        crl_dot4(c, 3, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.f[20] = c.g[V0][0] as u32;
        c.div_s(12, 12, 20);
        call_ptr(c, 0x0031_c228, 0x0038_9ea4)?; // Math_Atan

        // f23 = -azimuth; f21 = the elevation angle (0 without a
        // horizontal component).
        c.f[12] = 0;
        c.c_lt_s(12, 20);
        let horizontal = c.fcc();
        c.neg_s(23, 0);
        if !horizontal {
            c.cycles += 2;
            c.f[21] = c.f[12];
            c.g[V0][0] = 0x0050_0000;
        } else {
            c.cycles += 25;
            c.g[A0][0] = 0x0050_0000;
            // f1 = |dir| * |up|.
            c.lqc2(3, crl_sp(c, 0xf0));
            c.lqc2(5, crl_sp(c, 0xf0));
            crl_dot4(c, 3, 5);
            c.qmfc2(V0, 4);
            c.mtc1(V0, 0);
            c.lqc2(3, CRL_UP);
            c.lqc2(5, CRL_UP);
            crl_dot4(c, 3, 5);
            c.qmfc2(V1, 4);
            c.mtc1(V1, 1);
            c.lwc1(2, crl_gp(c, -0x26f4));
            c.mul_s(0, 0, 1);
            c.sqrt_s(1, 0);
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.cycles += 2;
                c.f[21] = c.f[12];
            } else {
                // cos = dir . up / f1, clamped to [-1 (f21), 1 (f22)];
                // f21 = k - asin(cos).
                c.cycles += 14;
                c.lqc2(3, crl_sp(c, 0xf0));
                c.lqc2(5, CRL_UP);
                crl_dot4(c, 3, 5);
                c.qmfc2(V0, 4);
                c.mtc1(V0, 0);
                c.div_s(0, 0, 1);
                c.c_le_s(21, 0);
                let above_min = c.fcc();
                c.f[12] = c.f[21];
                if above_min {
                    c.cycles += 1;
                    c.min_s(12, 0, 22);
                }
                c.cycles += 4;
                call_ptr(c, 0x0031_c128, 0x0038_9f68)?; // Math_Asin
                c.lwc1(1, crl_gp(c, -0x26f0));
                c.sub_s(21, 1, 0);
            }
            // Below the horizon (dir.y < 0): f21 = k - f21.
            c.cycles += 6;
            c.lwc1(1, crl_sp(c, 0xf4));
            c.f[0] = 0;
            c.c_lt_s(1, 0);
            let below = c.fcc();
            c.g[V0][0] = 0x0050_0000;
            if below {
                c.cycles += 3;
                c.lwc1(0, crl_gp(c, -0x26ec));
                c.sub_s(21, 0, 21);
            }
        }

        // orient (sp+0x120) = base with row 3 = base * camera (vf12).
        c.cycles += 234;
        let orient = sp(c).wrapping_add(0x120);
        c.g[S4][0] = sx(orient);
        c.g[V0][0] = sx(CRL_BASE);
        crl_copy_mat(c, orient, CRL_BASE);
        crl_load_mat(c, 4, orient);
        c.lqc2(8, crl_sp(c, 0xe0));
        crl_mat_vec(c, 12, 8);
        c.sqc2(12, orient.wrapping_add(0x30));

        // Rotate by the azimuth (f23) about the first axis.
        let prev = sp(c).wrapping_add(0x160);
        c.g[S0][0] = 0x0050_0000;
        c.neg_s(12, 23);
        c.g[S1][0] = sx(CRL_AXIS_1);
        c.g[S5][0] = sx(prev);
        c.g[S3][0] = c.g[S4][0];
        crl_copy_mat(c, prev, orient);
        c.g[S2][0] = 0x0050_0000;
        c.lq(V0, crl_sp(c, 0x190));
        c.g[S2][0] = sx(CRL_SWAP);
        c.g[A0][0] = sx(crl_sp(c, 0x220));
        c.lq(V1, CRL_SWAP);
        c.g[A1][0] = sx(crl_sp(c, 0x224));
        c.sq(V0, crl_sp(c, 0x1a0));
        c.sq(V1, crl_sp(c, 0x190));
        call_ptr(c, 0x0031_be50, 0x0038_a044)?; // Math_SinCos
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = 0x3f80_0000;
        crl_axis_rotation(c, 0x220, 0x224);
        crl_mat_mul(c, orient, orient, prev);

        // Rotate by the elevation (f21) about the second axis.
        c.g[S0][0] = 0x0050_0000;
        c.neg_s(12, 21);
        c.g[S1][0] = sx(CRL_AXIS_2);
        c.g[S3][0] = c.g[S4][0];
        crl_copy_mat(c, prev, orient);
        c.lq(V0, CRL_SWAP);
        c.g[A0][0] = sx(crl_sp(c, 0x228));
        c.lq(V1, crl_sp(c, 0x190));
        c.g[A1][0] = sx(crl_sp(c, 0x22c));
        c.sq(V0, crl_sp(c, 0x190));
        c.sq(V1, crl_sp(c, 0x1a0));
        call_ptr(c, 0x0031_be50, 0x0038_a1c8)?; // Math_SinCos
        crl_axis_rotation(c, 0x228, 0x22c);
        crl_mat_mul(c, orient, orient, prev);
        crl_transpose(c, orient, 7);

        // The axis-swap matrix rows (0,0,1,0), (-1,0,0,0), (0,1,0,0),
        // (0,0,0,1), built on first use.
        c.g[V0][0] = c.lw(crl_gp(c, CRL_SWAP_BUILT));
        let built = c.g[V0][0] != 0;
        c.g[A0][0] = 0x0050_0000;
        if !built {
            c.cycles += 22;
            c.f[0] = 0;
            c.g[V0][0] = sx(CRL_SWAP_MATRIX);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[1] = 0xbf80_0000;
            c.g[V1][0] = 1;
            let m = CRL_SWAP_MATRIX;
            c.swc1(20, m + 0x3c);
            c.swc1(1, m + 0x10);
            c.swc1(0, m + 0x2c);
            c.sw(crl_gp(c, CRL_SWAP_BUILT), 1);
            for (f, off) in [
                (0, 0x00),
                (0, 0x20),
                (0, 0x30),
                (0, 0x04),
                (0, 0x14),
                (20, 0x24),
                (0, 0x34),
                (20, 0x08),
                (0, 0x18),
                (0, 0x28),
                (0, 0x38),
                (0, 0x0c),
                (0, 0x1c),
            ] {
                c.swc1(f, m + off);
            }
        }

        // rot (sp+0x1b0, copied to sp+0x160) = swap * orient; its 3x3 part
        // packed at sp+0x1f0 and copied to sp+0x1a0.
        c.cycles += 100;
        let rot = sp(c).wrapping_add(0x1b0);
        c.g[V1][0] = sx(CRL_SWAP_MATRIX);
        c.g[V0][0] = sx(rot);
        c.g[A0][0] = c.g[S5][0];
        crl_mat_mul(c, rot, CRL_SWAP_MATRIX, orient);
        crl_copy_mat(c, prev, rot);
        for (f, off) in [
            (8, 0x160),
            (7, 0x164),
            (0, 0x168),
            (1, 0x170),
            (2, 0x174),
            (3, 0x178),
            (4, 0x180),
            (5, 0x184),
            (6, 0x188),
        ] {
            let a = crl_sp(c, off);
            c.lwc1(f, a);
        }
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(crl_sp(c, 0x1a0));
        c.g[A2][0] = 3;
        for (i, f) in [8, 7, 0, 1, 2, 3, 4, 5, 6].into_iter().enumerate() {
            let a = crl_sp(c, 0x1f0 + 4 * i as u32);
            c.swc1(f, a);
        }
        // 36-byte struct copy (ldl/ldr, sdl/sdr on aligned doublewords).
        for (reg, off) in [(V0, 0x00), (V1, 0x08), (T0, 0x10), (T1, 0x18)] {
            c.g[reg][0] = c.ld(crl_sp(c, 0x1f0 + off));
        }
        for (reg, off) in [(V0, 0x00), (V1, 0x08), (T0, 0x10), (T1, 0x18)] {
            let a = crl_sp(c, 0x1a0 + off);
            c.sd(a, c.g[reg][0]);
        }
        c.g[V0][0] = c.lw(crl_sp(c, 0x210));
        c.sw(crl_sp(c, 0x1c0), c.g[V0][0]);
        c.g[A3][0] = 3;
        Callee {
            addr: 0x0038_9840,
            f: ee::s12::Irradiance_Rotate,
        }
        .call(c, 0x0038_a4dc)?;

        // irr += lobe * (scale * k).
        c.lwc1(12, crl_gp(c, -0x26e8));
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[SP][0];
        c.mul_s(12, 24, 12);
        Callee {
            addr: 0x0038_9620,
            f: ee::s12::Irradiance_AddScaledW,
        }
        .call(c, 0x0038_a4f0)?;

        frame.restore(c, S0, 0x2a0);
        frame.restore(c, S1, 0x290);
        frame.restore(c, S2, 0x280);
        frame.restore(c, S3, 0x270);
        frame.restore(c, S4, 0x260);
        frame.restore(c, S5, 0x250);
        frame.restore(c, S6, 0x240);
        frame.restore_ra(c, 0x230);
        frame.restore_fpr(c, 24, 0x2d0);
        frame.restore_fpr(c, 23, 0x2c8);
        frame.restore_fpr(c, 22, 0x2c0);
        frame.restore_fpr(c, 21, 0x2b8);
        frame.restore_fpr(c, 20, 0x2b0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FEModule_RiderPreviewApplyTransform (0x19f2d0)

/// Preview fields: the rider model, the two pose halves (0x20 bytes each),
/// the two bones reset to the identity, the "single pose" flag, dirty flag.
pub(crate) const RIDER_PREVIEW_MODEL: u32 = 0x8;

pub(crate) const RIDER_PREVIEW_DIRTY: u32 = 0x14;

pub(crate) const RIDER_PREVIEW_POSE: u32 = 0xc30;

pub(crate) const RIDER_PREVIEW_BONES: [u32; 2] = [0xcac, 0xcb0];

pub(crate) const RIDER_PREVIEW_SINGLE_POSE: u32 = 0xccc;

/// `model->table[self->bone] = *(Quad*)src` (table at model + `table`).
pub(crate) fn rider_preview_reset_bone(
    c: &mut Cpu,
    bone: u32,
    table: u32,
    src_reg: usize,
    model_reg: usize,
    last: bool,
) {
    let s0 = c.g[S0][0] as u32;
    c.g[model_reg][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_MODEL));
    c.g[V0][0] = c.lw(s0.wrapping_add(bone));
    let tbl_reg = if model_reg == V1 { A0 } else { A2 };
    c.g[tbl_reg][0] = c.lw((c.g[model_reg][0] as u32).wrapping_add(table));
    if !last {
        c.lq(V1, c.g[src_reg][0] as u32);
    }
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    if last {
        c.lq(V1, c.g[src_reg][0] as u32);
    }
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[tbl_reg][0] as u32));
    c.sq(V1, c.g[V0][0] as u32);
}

/// `void FEModule_RiderPreviewApplyTransform(FERiderPreview* self)`
/// (0x19f2d0): pose the preview rider. With the single-pose flag both halves
/// of the pose get the first one; otherwise the two halves are the preview's
/// two poses and its two marked bones are reset to the identity quads at
/// 0x4ff130 / 0x4ff220 first. Then `RiderModel_ApplyPose` and
/// `RiderModel_BuildBoneMatrices`; marks the preview dirty.
pub fn fe_module_rider_preview_apply_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[A0][0];
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_SINGLE_POSE));
        c.g[A1][0] = c.g[SP][0];
        let sp_ = sp(c);
        if c.g[V0][0] != 0 {
            c.cycles += 12;
            c.lq(V1, s0.wrapping_add(RIDER_PREVIEW_POSE));
            c.lq(V0, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x10));
            c.g[A0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_MODEL));
            c.sq(V1, sp_.wrapping_add(0x20));
            c.sq(V0, sp_.wrapping_add(0x30));
            c.sq(V1, sp_);
            c.sq(V0, sp_.wrapping_add(0x10));
            call_ptr(c, 0x0031_0200, 0x0019_f30c)?; // RiderModel_ApplyPose(model, pose)
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            call_ptr(c, 0x0031_03f0, 0x0019_f314)?; // RiderModel_BuildBoneMatrices
        } else {
            c.cycles += 45;
            c.lq(A0, s0.wrapping_add(RIDER_PREVIEW_POSE));
            c.g[A3][0] = 0x50_0000;
            c.lq(A2, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x10));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xed0));
            c.lq(V0, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x30));
            c.g[T0][0] = 0x50_0000;
            c.lq(V1, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x20));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0xde0));
            c.sq(A0, sp_);
            c.sq(A2, sp_.wrapping_add(0x10));
            c.sq(V0, sp_.wrapping_add(0x30));
            c.sq(V1, sp_.wrapping_add(0x20));
            for (k, bone) in RIDER_PREVIEW_BONES.into_iter().enumerate() {
                rider_preview_reset_bone(c, bone, 0x24, A3, if k == 0 { V1 } else { A0 }, false);
                rider_preview_reset_bone(c, bone, 0x28, T0, A0, k == 1);
            }
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            call_ptr(c, 0x0031_0200, 0x0019_f3c4)?; // RiderModel_ApplyPose(model, pose)
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            call_ptr(c, 0x0031_03f0, 0x0019_f3cc)?; // RiderModel_BuildBoneMatrices
        }
        c.g[V0][0] = 0x1;
        c.cycles += 5;
        frame.restore_ra(c, 0x40);
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_DIRTY),
            c.g[V0][0],
        );
        frame.restore(c, S0, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FEModule_RiderPreviewRender (0x19ee88)

/// Preview fields read by the render: the rider type, a pending load, the
/// "outfit" override, the draw layer and the attached effects.
pub(crate) const RIDER_PREVIEW_TYPE: u32 = 0x0;

pub(crate) const RIDER_PREVIEW_LAYER: u32 = 0xc2c;

pub(crate) const RIDER_PREVIEW_FX: u32 = 0xc70;

pub(crate) const RIDER_PREVIEW_OUTFIT: u32 = 0xcd0;

pub(crate) const RIDER_PREVIEW_PENDING: u32 = 0xcd4;

/// `void FEModule_RiderPreviewRender(FERiderPreview* self)` (0x19ee88): draw
/// the front-end rider preview once it is loaded and posed: build its light
/// set (the irradiance entry for its type, types 7/8 and outfit-overridden
/// 0/7/8 using type 6's, plus a light relative to the root bone), set it on
/// the graphics manager, draw the model (at the preview's layer, if any) and
/// its attached effects, then restore the draw state's layer field.
pub fn fe_module_rider_preview_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xb0);
        'exit: {
            let s0 = c.g[S0][0] as u32;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_PENDING));
            if (c.g[V0][0] as i64) >= 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_MODEL));
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_DIRTY));
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0038_9260, 0x0019_eec8)?; // Irradiance_Clear(&lights)
                                                    // The light type (a0).
            let s0 = c.g[S0][0] as u32;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_OUTFIT));
            let outfit = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_TYPE));
            let lt = |c: &mut Cpu, n: i64| ((c.g[A0][0] as i64) < n) as u64;
            // 0: from 0x19eef8 (v0 = type < 9), 1: 0x19eefc (v0 set), 2: 0x19ef0c.
            let mut stage = 0;
            if outfit {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = 0x6;
                } else {
                    c.cycles += 2;
                    let negative = (c.g[A0][0] as i64) < 0;
                    c.g[V0][0] = lt(c, 9);
                    if !negative {
                        c.cycles += 2;
                        let big = c.g[V0][0] == 0;
                        c.g[V0][0] = lt(c, 7);
                        if big {
                            stage = 2;
                        } else {
                            c.cycles += 2;
                            let small = c.g[V0][0] != 0;
                            c.g[V0][0] = lt(c, 9);
                            if small {
                                stage = 1;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = 0x6;
                            }
                        }
                    }
                }
            }
            if stage == 0 {
                c.cycles += 1;
                c.g[V0][0] = lt(c, 9);
                stage = 1;
            }
            if stage == 1 {
                c.cycles += 2;
                let big = c.g[V0][0] == 0;
                c.g[V1][0] = lt(c, 7);
                if !big {
                    c.cycles += 2;
                    c.g[V0][0] = 0x6;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                }
            }
            // The type's irradiance entry (jump table at 0x460b40).
            c.cycles += 3;
            c.g[V0][0] = (c.g[A0][0] < 0xa) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x46_0000;
            if in_table {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let (cycles, name) = match c.g[A0][0] as u32 {
                    0x0019_ef30 => (5, 0x1620),
                    0x0019_ef44 => (5, 0x1628),
                    0x0019_ef58 => (5, 0x1980),
                    0x0019_ef6c => (5, 0x1638),
                    0x0019_ef80 => (5, 0x1640),
                    0x0019_ef94 => (5, 0x1988),
                    0x0019_efa8 => (5, 0x1650),
                    0x0019_efbc => (5, 0x1990),
                    0x0019_efd0 => (5, 0x1660),
                    0x0019_efe4 => (4, 0x1998),
                    t => {
                        c.jump_out(t, 0x0019_ee88, 0x0019_f134);
                        return Ok(());
                    }
                };
                c.cycles += cycles;
                c.g[A0][0] = 0x50_0000;
                c.g[A1][0] = 0x4a_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(name));
                c.cycles += 11;
                rider_preview_add_light(c, 0x0019_effc, 0x0019_f018)?;
            } else {
                c.cycles += 13;
                c.g[A0][0] = 0x50_0000;
                c.g[A1][0] = 0x4a_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x19a0));
                rider_preview_add_light(c, 0x0019_f034, 0x0019_f050)?;
            }
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            // A light at the root bone.
            c.cycles += 30;
            c.g[A2][0] = 0x4a_0000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1978));
            c.g[A1][0] = 0;
            call_ptr(c, 0x0031_0c48, 0x0019_f064)?; // RiderModel_GetBoneSkelIndex(model, 0, name)
            let sp_ = sp(c);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
            c.g[A1][0] = sx(sp_.wrapping_add(0xa0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.lq(V1, c.g[V0][0] as u32);
            c.sq(V1, sp_.wrapping_add(0xa0));
            call_ptr(c, 0x0038_9cb8, 0x0019_f090)?; // Irradiance_AddCameraRelativeLight
                                                    // GrMan->SetLights(&lights, 1) (vtable slot 0x228).
            let gp = c.g[GP][0] as u32;
            c.g[A3][0] = c.lw(gp.wrapping_sub(0x854));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x228));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x22c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_f0b0)?;
            // The draw state's layer field (bits 5..9): keep the old one in s1.
            let gp = c.g[GP][0] as u32;
            c.g[V0][0] = c.lw(gp.wrapping_sub(0x854));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_LAYER));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.g[A0][0] & 0x3e0;
            let no_layer = (c.g[V1][0] as i64) < 0;
            c.g[S1][0] = sx((c.g[V0][0] as u32) >> 5);
            if !no_layer {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = 0xffff_ffff_ffff_fc1f;
                c.g[V0][0] &= c.g[A0][0];
                c.g[V1][0] &= 0x3e0;
                c.g[V0][0] |= c.g[V1][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            }
            c.cycles += 16;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_0640, 0x0019_f0f8)?; // RiderModel_Render(model, 0, 0, 0)
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_FX));
            call_ptr(c, 0x002f_2088, 0x0019_f100)?; // AttachedEmitterFX_Render
            let gp = c.g[GP][0] as u32;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x854));
            c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
            c.g[A2][0] = sx((c.g[S1][0] as u32) << 5);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] &= c.g[A1][0];
            c.g[V0][0] |= c.g[A2][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        }
        frame.restore(c, S0, 0xd0);
        c.cycles += 4;
        frame.restore(c, S1, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `Irradiance_AddScaled(&lights, Irradiance_GetEntryByName(set, name),
/// 0x504810, 1.0)` (a0/a1 = set, name are set).
pub(crate) fn rider_preview_add_light(
    c: &mut Cpu,
    ret_get: u32,
    ret_add: u32,
) -> crate::call::Flow {
    call_ptr(c, 0x0038_ac20, ret_get)?; // Irradiance_GetEntryByName
    c.g[A2][0] = 0x50_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 12);
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4810));
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0038_9590, ret_add)?; // Irradiance_AddScaled
    Ok(())
}

/// `FEModule_GetRiderPreview_2_Thunk` (0x1dcb70): `cFERiderManager_getHuman(this + 0xb0)`,
/// tail-returning its result.
pub fn fe_module_get_rider_preview_2_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = menu_sx_add(c.g[A0][0], 0xb0);
        call_ptr(c, 0x001a_0538, 0x001d_cb80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEModule_GetRiderPreview_Thunk` (0x1dcb90): `cFERiderManager_getHuman1(this + 0xb0)`,
/// tail-returning its result.
pub fn fe_module_get_rider_preview_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = menu_sx_add(c.g[A0][0], 0xb0);
        call_ptr(c, 0x001a_0548, 0x001d_cba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int FEModule_RiderPreviewPollModels(RiderPreview* self)` (the C++ `cFERider_isDoneLoadingModels`): once per poll, checks that every bolt file holding one of the rider's profile parts is loaded; when all are, builds each part's model from the bolt's MDF archive, adds it to the preview's anim model, releases the file and clears the slot. Returns 1 when done (or already done, +0xcbc set), else 0.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn fe_module_rider_preview_poll_models(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x001a0100);
    'b_1a032c: {
        'b_1a0328: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xcbc));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_1a0328;
            } else {
                'b_1a01cc: {
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x001a0144);
                    c.pc = 0x0014dc80;
                    call_indirect(c, 0x0014dc80);
                    if c.pc != 0x001a0144 {
                        return;
                    }
                    c.g[V1][0] = 0x1;
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[RA][0] = sx(0x001a0158);
                    c.g[A1][0] = 0x9;
                    c.pc = 0x0014dd58;
                    call_indirect(c, 0x0014dd58);
                    if c.pc != 0x001a0158 {
                        return;
                    }
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0168);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                    c.pc = 0x0014ad28;
                    call_indirect(c, 0x0014ad28);
                    if c.pc != 0x001a0168 {
                        return;
                    }
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28c));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
                    if t {
                        break 'b_1a01cc;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[S0][0] = c.g[S6][0];
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        'l_1a0190: loop {
                            'b_1a01c0: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[RA][0] = sx(0x001a01a0);
                                c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                c.pc = 0x0019cbe0;
                                ee::s03::f_0019cbe0(c);
                                if c.pc != 0x001a01a0 {
                                    return;
                                }
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[A2][0] = c.g[V0][0];
                                if t {
                                    break 'b_1a01c0;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[RA][0] = sx(0x001a01b4);
                                    c.g[A0][0] = c.g[S7][0];
                                    c.pc = 0x0019e238;
                                    ee::s03::f_0019e238(c);
                                    if c.pc != 0x001a01b4 {
                                        return;
                                    }
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                    if c.g[V0][0] == 0 {
                                        c.g[V1][0] = 0u64;
                                    }
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                    break 'b_1a01c0;
                                }
                            }
                            c.cycles += 3;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001a0190);
                                }
                                continue 'l_1a0190;
                            } else {
                                break 'b_1a01cc;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                if t {
                    break 'b_1a032c;
                } else {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_1a0328;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        'l_1a01e8: loop {
                            'b_1a0318: {
                                c.cycles += 11;
                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                c.g[S1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                c.g[RA][0] = sx(0x001a0208);
                                c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.pc = 0x0019cbe0;
                                ee::s03::f_0019cbe0(c);
                                if c.pc != 0x001a0208 {
                                    return;
                                }
                                c.g[S4][0] = c.g[V0][0];
                                if (c.g[S4][0] as i64) < 0 {
                                    c.g[S0][0] = c.g[FP][0];
                                    break 'b_1a0318;
                                } else {
                                    'b_1a0240: {
                                        c.cycles += 9;
                                        c.g[RA][0] = sx(0x001a021c);
                                        c.g[A0][0] = c.g[SP][0];
                                        c.pc = 0x00314978;
                                        call_indirect(c, 0x00314978);
                                        if c.pc != 0x001a021c {
                                            return;
                                        }
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                        c.g[A0][0] = c.g[S7][0];
                                        c.g[RA][0] = sx(0x001a022c);
                                        c.g[A2][0] = c.g[S4][0];
                                        c.pc = 0x0019e2b0;
                                        ee::s03::f_0019e2b0(c);
                                        if c.pc != 0x001a022c {
                                            return;
                                        }
                                        c.g[S5][0] = c.g[V0][0];
                                        let t = c.g[S5][0] == 0u64;
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_1a0240;
                                        } else {
                                            c.cycles += 2;
                                            c.g[RA][0] = sx(0x001a0240);
                                            c.g[A1][0] = c.g[S5][0];
                                            c.pc = 0x00314a40;
                                            call_indirect(c, 0x00314a40);
                                            if c.pc != 0x001a0240 {
                                                return;
                                            }
                                            break 'b_1a0240;
                                        }
                                    }
                                    'b_1a030c: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_1a030c;
                                        } else {
                                            c.cycles += 1;
                                            c.g[S3][0] = sx((c.g[S6][0] as u32)
                                                .wrapping_sub((c.g[S0][0] as u32)));
                                            'l_1a0250: loop {
                                                'b_1a0300: {
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1788));
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                    c.g[RA][0] = sx(0x001a0260);
                                                    c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                                    c.pc = 0x0019cbe0;
                                                    ee::s03::f_0019cbe0(c);
                                                    if c.pc != 0x001a0260 {
                                                        return;
                                                    }
                                                    if c.g[V0][0] != c.g[S4][0] {
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        break 'b_1a0300;
                                                    } else {
                                                        'b_1a02dc: {
                                                            c.cycles += 2;
                                                            if c.g[S5][0] == 0u64 {
                                                                c.g[A1][0] =
                                                                    c.lw((c.g[S2][0] as u32));
                                                                break 'b_1a02dc;
                                                            } else {
                                                                'b_1a02ac: {
                                                                    'b_1a029c: {
                                                                        c.cycles += 9;
                                                                        c.g[RA][0] = sx(0x001a0278);
                                                                        c.pc = 0x0014bdb8;
                                                                        call_indirect(
                                                                            c, 0x0014bdb8,
                                                                        );
                                                                        if c.pc != 0x001a0278 {
                                                                            return;
                                                                        }
                                                                        c.g[S0][0] = c.g[V0][0];
                                                                        c.g[A1][0] =
                                                                            c.lw(
                                                                                (c.g[S2][0] as u32)
                                                                            );
                                                                        c.g[A2][0] =
                                                                            c.lh(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        c.g[RA][0] = sx(0x001a028c);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        c.pc = 0x0014d448;
                                                                        call_indirect(
                                                                            c, 0x0014d448,
                                                                        );
                                                                        if c.pc != 0x001a028c {
                                                                            return;
                                                                        }
                                                                        let t = (c.g[V0][0] as i64)
                                                                            >= 0;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        if t {
                                                                            break 'b_1a029c;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.g[S0][0] = 0u64;
                                                                            if t {
                                                                                break 'b_1a02ac;
                                                                            } else {
                                                                                break 'b_1a029c;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = 0x38;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.mult(0, 2, 4, 2);
                                                                    c.g[S0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[V0][0] as u32),
                                                                        ));
                                                                    break 'b_1a02ac;
                                                                }
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                if c.g[A1][0] == 0u64 {
                                                                    c.g[A1][0] =
                                                                        c.lw((c.g[S2][0] as u32));
                                                                    break 'b_1a02dc;
                                                                } else {
                                                                    'b_1a02d8: {
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x001a02c0);
                                                                        c.g[A0][0] = c.g[SP][0];
                                                                        c.pc = 0x00314c00;
                                                                        call_indirect(
                                                                            c, 0x00314c00,
                                                                        );
                                                                        if c.pc != 0x001a02c0 {
                                                                            return;
                                                                        }
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A2][0] = c.g[V0][0];
                                                                        if t {
                                                                            break 'b_1a02d8;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.g[A3][0] = c.lb((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x1));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[RA][0] =
                                                                                sx(0x001a02d8);
                                                                            c.g[A1][0] = 0u64;
                                                                            c.pc = 0x0030d8b8;
                                                                            call_indirect(
                                                                                c, 0x0030d8b8,
                                                                            );
                                                                            if c.pc != 0x001a02d8 {
                                                                                return;
                                                                            }
                                                                            break 'b_1a02d8;
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[A1][0] =
                                                                        c.lw((c.g[S2][0] as u32));
                                                                    break 'b_1a02dc;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 9;
                                                        c.g[A0][0] = c.g[S7][0];
                                                        c.g[RA][0] = sx(0x001a02e8);
                                                        c.g[A2][0] = c.g[S4][0];
                                                        c.pc = 0x0019dc20;
                                                        ee::s03::f_0019dc20(c);
                                                        if c.pc != 0x001a02e8 {
                                                            return;
                                                        }
                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1788));
                                                        c.g[A3][0] = 0xffffffffffffffff;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[RA][0] = sx(0x001a02fc);
                                                        c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                                        c.pc = 0x0019cc30;
                                                        ee::s03::f_0019cc30(c);
                                                        if c.pc != 0x001a02fc {
                                                            return;
                                                        }
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        break 'b_1a0300;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S3][0] != 0u64;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001a0250);
                                                    }
                                                    continue 'l_1a0250;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    break 'b_1a030c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001a0314);
                                    c.g[A1][0] = 0x2;
                                    c.pc = 0x00314988;
                                    call_indirect(c, 0x00314988);
                                    if c.pc != 0x001a0314 {
                                        return;
                                    }
                                    c.g[S0][0] = c.g[FP][0];
                                    break 'b_1a0318;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001a01e8);
                                }
                                continue 'l_1a01e8;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_1a0328;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        break 'b_1a032c;
    }
    c.cycles += 11;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.pc = t;
    return;
}

#[inline]
pub(crate) fn o760_lh(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lh(c)
}

/// `void FEModule_RiderPreviewReleaseBoltFiles(FEModule* self)` (0x19ec68):
/// when the preview rider has a bolt file (`self+4` = rider index, `self+0`
/// = bolt file id, both >= 0), release every texture and model of the rider's
/// bolt-file index entry from the BoltFile and mark the slots free (-1).
pub fn fe_module_rider_preview_release_bolt_files(c: &mut Cpu) {
    guest(c, |c| {
        'out: {
            c.cycles += 10;
            let frame = Frame::open(c, 0x50);
            frame.save(c, 16, 0x40);
            frame.save(c, 17, 0x30);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, 18, 0x20);
            frame.save(c, 19, 0x10);
            frame.save_ra(c, 0);
            c.g[V0][0] = o760_lw(c, S0, 0x4);
            if (c.g[V0][0] as i64) < 0 {
                frame.restore(c, 16, 0x40);
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = o760_lw(c, S0, 0);
            if (c.g[V0][0] as i64) < 0 {
                frame.restore(c, 16, 0x40);
                break 'out;
            }
            'released: {
                c.cycles += 15;
                call_ptr(c, 0x0014_dc80, 0x0019_eca4)?; // cBE_getBE
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x9;
                call_ptr(c, 0x0014_dd58, 0x0019_ecb0)?;
                c.g[V1][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x848));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = o760_lw(c, S0, 0x4);
                c.g[V0][0] = o760_lw(c, V1, 0x7c);
                c.g[A2][0] = o760_lw(c, S0, 0);
                c.g[S3][0] = o760_addi(c, V0, 0x1a70); // the BoltFile
                call_ptr(c, 0x0014_ad28, 0x0019_eccc)?;
                c.g[V1][0] = o760_lw(c, V0, 0x28c); // entry count
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[S1][0] = o760_addi(c, V0, 0x290); // entries, 4 bytes each
                if empty {
                    break 'released;
                }
                c.cycles += 2;
                c.g[S2][0] = c.g[V1][0];
                loop {
                    // Texture slot of this entry.
                    c.cycles += 6;
                    c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                    c.g[A1][0] = o760_lw(c, S0, 0x4);
                    c.g[A2][0] = o760_lh(c, S1, 0);
                    Callee {
                        addr: 0x0019_cce8,
                        f: ee::s03::BoltFileIndexManager_GetTextureSlot,
                    }
                    .call(c, 0x0019_ecf0)?;
                    let none = (c.g[V0][0] as i64) < 0;
                    c.g[A2][0] = c.g[V0][0];
                    if !none {
                        c.cycles += 8;
                        c.g[A1][0] = o760_lw(c, S0, 0);
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0019_dc20,
                            f: ee::s03::BoltFile_Release,
                        }
                        .call(c, 0x0019_ed04)?;
                        c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                        c.g[A1][0] = o760_lw(c, S0, 0x4);
                        c.g[A2][0] = o760_lh(c, S1, 0);
                        Callee {
                            addr: 0x0019_cd38,
                            f: ee::s03::BoltFileIndexManager_SetTextureSlot,
                        }
                        .call(c, 0x0019_ed18)?;
                    }
                    // Model slot of this entry.
                    c.cycles += 6;
                    c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                    c.g[A1][0] = o760_lw(c, S0, 0x4);
                    c.g[A2][0] = o760_lh(c, S1, 0);
                    Callee {
                        addr: 0x0019_cbe0,
                        f: ee::s03::BoltFileIndexManager_GetModelSlot,
                    }
                    .call(c, 0x0019_ed28)?;
                    if (c.g[V0][0] as i64) < 0 {
                        c.g[S2][0] = o760_addi(c, S2, 0u32.wrapping_sub(1));
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = o760_lw(c, S0, 0);
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0019_dc20,
                            f: ee::s03::BoltFile_Release,
                        }
                        .call(c, 0x0019_ed40)?;
                        c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                        c.g[A1][0] = o760_lw(c, S0, 0x4);
                        c.g[A2][0] = o760_lh(c, S1, 0);
                        Callee {
                            addr: 0x0019_cc30,
                            f: ee::s03::BoltFileIndexManager_SetModelSlot,
                        }
                        .call(c, 0x0019_ed54)?;
                        c.g[S2][0] = o760_addi(c, S2, 0u32.wrapping_sub(1));
                    }
                    c.cycles += 2;
                    let more = c.g[S2][0] != 0;
                    c.g[S1][0] = o760_addi(c, S1, 0x4);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_ece0);
                        }
                        continue;
                    }
                    break 'released;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x40);
        }
        // Epilogue.
        let frame = Frame { size: 0x50 };
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int FEModule_RiderPreviewPollTextures(FEModule* self)` (0x19fd58): the rider
/// preview screen's texture loading poll. Returns 1 at once when the textures
/// are already loaded (`+0xcb8`). Otherwise asks the BE for the rider's profile
/// bolt list (`+0x28c` count, entries from `+0x290`, 4 bytes each: id halfword
/// and flags halfword), checks every bolt's texture file is loaded (`BoltFile_IsLoaded`);
/// when all are, hands each file's shape data to
/// `FEModule_RiderPreviewLoadShapeTextures` once (a 256-entry "used" table on the
/// stack, bit 4 of the flags selects the variant), releases the file and
/// clears the bolt's texture slot. Returns whether all were loaded.
pub fn fe_module_rider_preview_poll_textures(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x490 };
        'b_19fed0: {
            c.cycles += 14;
            Frame::open(c, 0x490);
            frame.save(c, S1, 0x470);
            frame.save(c, S0, 0x480);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S2, 0x460);
            frame.save(c, S3, 0x450);
            frame.save(c, S4, 0x440);
            frame.save(c, S5, 0x430);
            frame.save(c, S6, 0x420);
            frame.save(c, S7, 0x410);
            frame.save_ra(c, 0x400);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xcb8));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_19fed0;
            } else {
                'b_19fe0c: {
                    c.cycles += 15;
                    c.g[S6][0] = 0x1;
                    call_ptr(c, 0x0014dc80, 0x0019fd98)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019fda4)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    call_ptr(c, 0x0014ad28, 0x0019fdb4)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[S3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28c));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
                    if t {
                        break 'b_19fe0c;
                    } else {
                        c.cycles += 3;
                        c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
                        c.g[S0][0] = c.g[S3][0];
                        'l_19fdd8: loop {
                            'b_19fe00: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.lh(c.g[S2][0] as u32);
                                call_ptr(c, 0x0019cce8, 0x0019fde8)?;
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[A2][0] = c.g[V0][0];
                                if t {
                                    break 'b_19fe00;
                                } else {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x0019e238, 0x0019fdfc)?;
                                    if c.g[V0][0] == 0 {
                                        c.g[S6][0] = 0u64;
                                    }
                                    break 'b_19fe00;
                                }
                            }
                            c.cycles += 3;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] != 0u64;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019fdd8);
                                }
                                continue 'l_19fdd8;
                            } else {
                                break 'b_19fe0c;
                            }
                        }
                    }
                }
                'b_19fecc: {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_19fecc;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x400;
                        call_ptr(c, 0x003e6448, 0x0019fe20)?;
                        let t = (c.g[S3][0] as i64) <= 0;
                        c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
                        if t {
                            break 'b_19fecc;
                        } else {
                            c.cycles += 2;
                            c.g[S7][0] = 0x1;
                            c.g[S5][0] = c.g[S3][0];
                            'l_19fe30: loop {
                                'b_19fec0: {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = c.lh(c.g[S2][0] as u32);
                                    call_ptr(c, 0x0019cce8, 0x0019fe40)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let t = (c.g[S0][0] as i64) < 0;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_19fec0;
                                    } else {
                                        'b_19fe9c: {
                                            'b_19fe94: {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[A2][0] = c.g[S0][0];
                                                c.g[S3][0] = sx((c.g[S0][0] as u32) << 2);
                                                call_ptr(c, 0x0019e2b0, 0x0019fe5c)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = sx((c.g[SP][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                                if t {
                                                    break 'b_19fe94;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                    if t {
                                                        break 'b_19fe9c;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[A2][0] = c.g[S0][0];
                                                        c.g[A0][0] = c.g[S4][0];
                                                        call_ptr(c, 0x0019e2b0, 0x0019fe7c)?;
                                                        c.g[A2][0] = c.lhu(
                                                            (c.g[S2][0] as u32).wrapping_add(0x2),
                                                        );
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[A2][0] = c.g[A2][0] & 0x10;
                                                        c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
                                                        call_ptr(c, 0x0019f908, 0x0019fe94)?;
                                                        break 'b_19fe94;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[V0][0] =
                                                sx((c.g[SP][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                            break 'b_19fe9c;
                                        }
                                        c.cycles += 9;
                                        c.g[A2][0] = c.g[S0][0];
                                        c.g[A0][0] = c.g[S4][0];
                                        c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                                        call_ptr(c, 0x0019dc20, 0x0019feac)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                        c.g[A3][0] = 0xffffffffffffffff;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[A2][0] = c.lh(c.g[S2][0] as u32);
                                        call_ptr(c, 0x0019cd38, 0x0019fec0)?;
                                        break 'b_19fec0;
                                    }
                                }
                                c.cycles += 3;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                let t = c.g[S5][0] != 0u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019fe30);
                                    }
                                    continue 'l_19fe30;
                                } else {
                                    break 'b_19fecc;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S6][0];
                break 'b_19fed0;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x480);
        frame.restore(c, S1, 0x470);
        frame.restore(c, S2, 0x460);
        frame.restore(c, S3, 0x450);
        frame.restore(c, S4, 0x440);
        frame.restore(c, S5, 0x430);
        frame.restore(c, S6, 0x420);
        frame.restore(c, S7, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        return Ok(());
    });
}

/// `int FEModule_RiderPreviewFindTexture(RiderPreview* self, char* name)`
/// (0x19f878): the id (`+0x1c + 0xc*i`) of the first of the `self+0x18`
/// entries whose name (`+0x20 + 0xc*i`) equals `name` (`strcmp`), else -1.
pub fn fe_module_rider_preview_find_texture_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A1][0];
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[S1][0] = sx(r32(c, S3).wrapping_add(0x1c));
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(0x20));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_65a8, 0x0019_f8bc)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    found = true;
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_f8b0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
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

pub(crate) const PREVIEW_UNLOAD: Callee = Callee {
    addr: 0x0019_eba0,
    f: ee::s03::f_0019eba0,
};

pub(crate) const PREVIEW_REQUEST_BOLT_MODELS: Callee = Callee {
    addr: 0x0019_ff00,
    f: ee::s03::f_0019ff00,
};

pub(crate) const PREVIEW_REQUEST_EXTRA_TEXTURES: Callee = Callee {
    addr: 0x0019_e7f0,
    f: ee::s03::f_0019e7f0,
};

pub(crate) const PREVIEW_REQUEST_BOLT_TEXTURES: Callee = Callee {
    addr: 0x0019_fbe0,
    f: ee::s03::f_0019fbe0,
};

pub(crate) const PREVIEW_UPDATE: Callee = Callee {
    addr: 0x0019_f138,
    f: ee::s03::f_0019f138,
};

/// `int FEModule_RiderPreviewLoadChar(Preview* this, int profile, int character, int slot)`
/// (0x19e960) (reference: `cFERider_init`).
///
/// Loads the front-end rider preview for `character` in outfit `slot`
/// (`profile` is the rider profile). Returns 1 at once when that character
/// and slot are already loaded; otherwise unloads the current one (if any),
/// resets the preview fields (character `+0`, profile `+4`, slot `+0x10`,
/// the dirty flag `+0xcc4`, ...), builds its effect objects and anim model,
/// requests the models and textures (the extra textures for slots other than
/// 0), updates the preview, and records the character's bit mask (`+0xcd8`,
/// from the jump table at 0x460b00 for characters 0..=9). Returns 1.
pub fn fe_module_rider_preview_load_char(c: &mut Cpu) {
    if c.pc != 0x0019_e960 {
        return fe_module_rider_preview_load_char_resume(c);
    }
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0019_e990)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x0014_dd58, 0x0019_e99c)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        if c.g[V0][0] == c.g[S2][0] {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT));
            let same_slot = c.g[V0][0] == c.g[S1][0];
            c.g[V0][0] = 0x1;
            if same_slot {
                // Already loaded.
                return preview_epilogue(c, frame);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        }
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            PREVIEW_UNLOAD.call(c, 0x0019_e9cc)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
        }
        // Reset the preview fields (the dirty flag keeps its two sources).
        c.cycles += 68;
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
        c.g[A1][0] = 0x460000;
        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT), c.g[S1][0]);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S3][0]);
        c.g[V1][0] |= c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc28), 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.g[A3][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc8), 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xac0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb4), 0);
        c.g[A0][0] = 0xa8;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb8), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc24), c.g[T0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc4), c.g[V1][0]);
        // "AttachedEmitterFX" (168 bytes) and its constructor.
        call_ptr(c, 0x0031_7d70, 0x0019_ea2c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002f_1890, 0x0019_ea38)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xad8));
        c.g[A0][0] = 0xc8;
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc70), c.g[V0][0]);
        // "cBoltOnFXInterface" (200 bytes).
        call_ptr(c, 0x0031_7d70, 0x0019_ea54)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_ca68, 0x0019_ea64)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xaf0));
        c.g[A2][0] = 0;
        c.g[A0][0] = 0x160;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc74), c.g[V0][0]);
        // "AnimModel" (352 bytes), constructor and init(100, 15).
        call_ptr(c, 0x0031_7d70, 0x0019_ea80)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0030_d4b8, 0x0019_ea88)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0xf;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x0030_d840, 0x0019_ea9c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x002f_19f0, 0x0019_eaa8)?;
        // The rider's model list, then request models and textures.
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0014_ad28, 0x0019_eab8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        c.g[T0][0] = 0x1;
        PREVIEW_REQUEST_BOLT_MODELS.call(c, 0x0019_ead4)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT));
        let slot_zero = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !slot_zero {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            PREVIEW_REQUEST_EXTRA_TEXTURES.call(c, 0x0019_eae8)?;
            c.g[A1][0] = c.g[S1][0];
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A3][0] = 0x1;
        PREVIEW_REQUEST_BOLT_TEXTURES.call(c, 0x0019_eafc)?;
        c.g[A0][0] = c.g[S0][0];
        PREVIEW_UPDATE.call(c, 0x0019_eb04)?;
        // The character's bit mask, by the jump table at 0x460b00.
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = (c.g[V1][0] < 0xa) as u64;
        let unknown = c.g[V0][0] == 0;
        c.g[V0][0] = 0x460000;
        if !unknown {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb00));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            // (mask, cycles) of each case of the jump table.
            let (mask, cycles) = match target {
                0x0019_eb2c => (0x1, 2),
                0x0019_eb34 => (0x4, 2),
                0x0019_eb3c => (0x20, 2),
                0x0019_eb44 => (0x2, 2),
                0x0019_eb4c => (0x8, 2),
                0x0019_eb54 => (0x100, 2),
                0x0019_eb5c => (0x200, 2),
                0x0019_eb64 => (0x80, 2),
                0x0019_eb6c => (0x10, 2),
                0x0019_eb74 => (0x40, 1),
                _ => {
                    c.jump_out(target, 0x0019_e960, 0x0019_eba0);
                    return Ok(());
                }
            };
            c.cycles += cycles;
            c.g[V0][0] = mask;
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xcd8), c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        preview_epilogue(c, frame)
    })
}

/// Native resume of `fe_module_rider_preview_load_char` (converted from the `entries` body): any entry
/// other than 0x0019e960, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn fe_module_rider_preview_load_char_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0019e960 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A3][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x0019e990);
                    c.g[S3][0] = c.g[A1][0];
                    call_ptr(c, 0x0014dc80, 0x0019e990)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0019e99c);
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019e99c)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != c.g[S2][0];

                    if t {
                        pc = 0x0019e9bc;
                        continue;
                    }
                    pc = 0x0019e9ac;
                    continue;
                }
                0x0019e9ac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == c.g[S1][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0019eb80;
                        continue;
                    }
                    pc = 0x0019e9b8;
                    continue;
                }
                0x0019e9b8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    pc = 0x0019e9bc;
                    continue;
                }
                0x0019e9bc => {
                    c.cycles += 2;

                    if (c.g[V0][0] as i64) < 0 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
                        pc = 0x0019e9d0;
                        continue;
                    }
                    pc = 0x0019e9c4;
                    continue;
                }
                0x0019e9c4 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019e9cc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0019eba0,
                        f: ee::s03::FEModule_RiderPreviewUnload,
                    }
                    .call(c, 0x0019e9cc)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
                    pc = 0x0019e9d0;
                    continue;
                }
                0x0019e9d0 => {
                    c.cycles += 68;
                    c.g[T0][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                    c.g[A1][0] = 0x460000;
                    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.sw(c.g[S0][0] as u32, c.g[S2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc28), 0u64);
                    c.g[A2][0] = 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcc8), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xac0));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcb4), 0u64);
                    c.g[A0][0] = 0xa8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcb8), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcc0), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc24), c.g[T0][0]);
                    c.g[RA][0] = sx(0x0019ea2c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcc4), c.g[V1][0]);
                    call_ptr(c, 0x00317d70, 0x0019ea2c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0019ea38);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x002f1890, 0x0019ea38)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xad8));
                    c.g[A0][0] = 0xc8;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0019ea54);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc70), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x0019ea54)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0019ea64);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x002eca68, 0x0019ea64)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xaf0));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = 0x160;
                    c.g[RA][0] = sx(0x0019ea80);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc74), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x0019ea80)?;
                    c.g[RA][0] = sx(0x0019ea88);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0030d4b8, 0x0019ea88)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0xf;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0019ea9c);
                    c.g[A1][0] = 0x64;
                    call_ptr(c, 0x0030d840, 0x0019ea9c)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
                    c.g[RA][0] = sx(0x0019eaa8);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x002f19f0, 0x0019eaa8)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0019eab8);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x0014ad28, 0x0019eab8)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0019ead4);
                    c.g[T0][0] = 0x1;
                    Callee {
                        addr: 0x0019ff00,
                        f: ee::s03::FEModule_RiderPreviewRequestBoltModels,
                    }
                    .call(c, 0x0019ead4)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0019eaec;
                        continue;
                    }
                    pc = 0x0019eae0;
                    continue;
                }
                0x0019eae0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019eae8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0019e7f0,
                        f: ee::s03::FEModule_RiderPreviewRequestExtraTextures,
                    }
                    .call(c, 0x0019eae8)?;
                    c.g[A1][0] = c.g[S1][0];
                    pc = 0x0019eaec;
                    continue;
                }
                0x0019eaec => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0019eafc);
                    c.g[A3][0] = 0x1;
                    Callee {
                        addr: 0x0019fbe0,
                        f: ee::s03::FEModule_RiderPreviewRequestBoltTextures,
                    }
                    .call(c, 0x0019eafc)?;
                    c.g[RA][0] = sx(0x0019eb04);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0019f138,
                        f: ee::s03::FEModule_RiderPreviewUpdate,
                    }
                    .call(c, 0x0019eb04)?;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x0019eb7c;
                        continue;
                    }
                    pc = 0x0019eb14;
                    continue;
                }
                0x0019eb14 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb00));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x0019eb2c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb34;
                    continue;
                }
                0x0019eb34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb3c;
                    continue;
                }
                0x0019eb3c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x20;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb44;
                    continue;
                }
                0x0019eb44 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb4c;
                    continue;
                }
                0x0019eb4c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x8;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb54;
                    continue;
                }
                0x0019eb54 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x100;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb5c;
                    continue;
                }
                0x0019eb5c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x200;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb64;
                    continue;
                }
                0x0019eb64 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x80;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb6c;
                    continue;
                }
                0x0019eb6c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x10;
                    if t {
                        pc = 0x0019eb78;
                        continue;
                    }
                    pc = 0x0019eb74;
                    continue;
                }
                0x0019eb74 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x40;
                    pc = 0x0019eb78;
                    continue;
                }
                0x0019eb78 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xcd8), c.g[V0][0]);
                    pc = 0x0019eb7c;
                    continue;
                }
                0x0019eb7c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0019eb80;
                    continue;
                }
                0x0019eb80 => {
                    c.cycles += 8;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0019e960, 0x0019eba0);
                    return Ok(());
                }
            }
        }
    });
}

/// `FEModule_RiderPreviewLoadChar` fields.
pub(crate) const PREVIEW_SLOT: u32 = 0x10;

/// The epilogue of `fe_module_rider_preview_load_char`.
pub(crate) fn preview_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void FEModule_RiderPreviewRequestBoltModels(FERider* self, BoltList* list,
/// BoltEntry* prev, int force, int keepLoaded)` (0x19ff00, frame 0xb0;
/// the reference's `cFERider::loadModels`). `self+0x4` is the profile index,
/// `self` the character id; the list has `count` (+0x28c) 4-byte entries from
/// +0x290 (id halfword, flags halfword). Pass 1 (when `force` is 0): entries
/// not flagged 0x10 whose model slot is mapped release their Bolt-file slot
/// (`BoltFile_Release`) and are unmapped (`SetModelSlot(-1)`). Pass 2: for
/// unmapped entries, when `force` or (flag 0x10 and `prev` unset / not flagged)
/// it looks up the part entry (`func_0014D448`, 0x38-byte records) and
/// acquires its file (`BoltFile_Acquire`, `keepLoaded` from the stack),
/// storing the slot and clearing `self+0xcb4/0xcbc`, or stores -1 when the
/// part has no file; `prev` advances per unmapped entry. Ends with
/// `BoltFile_PrioritizeModels`.
pub fn fe_module_rider_preview_request_bolt_models(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ffd4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = c.g[A2][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A1][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[FP][0] = c.g[A3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S4][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28c));
            c.g[RA][0] = sx(0x0019ff48);
            c.sw(c.g[SP][0] as u32, c.g[T0][0]);
            call_ptr(c, 0x0014bdb8, 0x0019ff48)?;
            c.g[S7][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
            if t {
                break 'b_19ffd4;
            } else {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
                c.g[S2][0] = c.g[S4][0];
                'l_19ff68: loop {
                    'b_19ffcc: {
                        c.cycles += 2;
                        if c.g[FP][0] != 0u64 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_19ffcc;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            if c.g[V0][0] != 0u64 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                break 'b_19ffcc;
                            } else {
                                'b_19ffc8: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x0019ff90);
                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                    call_ptr(c, 0x0019cbe0, 0x0019ff90)?;
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    if t {
                                        break 'b_19ffc8;
                                    } else {
                                        c.cycles += 12;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[RA][0] = sx(0x0019ffa4);
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        call_ptr(c, 0x0019cbe0, 0x0019ffa4)?;
                                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x0019ffb4);
                                        c.g[A0][0] = c.g[S6][0];
                                        call_ptr(c, 0x0019dc20, 0x0019ffb4)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                        c.g[A3][0] = 0xffffffffffffffff;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[RA][0] = sx(0x0019ffc8);
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        call_ptr(c, 0x0019cc30, 0x0019ffc8)?;
                                        break 'b_19ffc8;
                                    }
                                }
                                c.cycles += 1;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                break 'b_19ffcc;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019ff68);
                        }
                        continue 'l_19ff68;
                    } else {
                        break 'b_19ffd4;
                    }
                }
            }
        }
        'b_1a00c0: {
            c.cycles += 2;
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
            if t {
                break 'b_1a00c0;
            } else {
                c.cycles += 1;
                c.g[S2][0] = c.g[S4][0];
                'l_19ffe0: loop {
                    'b_1a00b8: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x0019fff0);
                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                        call_ptr(c, 0x0019cbe0, 0x0019fff0)?;
                        if (c.g[V0][0] as i64) >= 0 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_1a00b8;
                        } else {
                            'b_1a00b0: {
                                'b_1a002c: {
                                    c.cycles += 2;
                                    if c.g[FP][0] != 0u64 {
                                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                        break 'b_1a002c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                                        c.g[V0][0] = c.g[V0][0] & 0x10;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_1a00b0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S3][0] == 0u64 {
                                                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                break 'b_1a002c;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S3][0] as u32).wrapping_add(0x2));
                                                c.g[V0][0] = c.g[V0][0] & 0x10;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_1a00b0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                    break 'b_1a002c;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_1a0058: {
                                    'b_1a0048: {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S7][0];
                                        c.g[RA][0] = sx(0x001a0038);
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        call_ptr(c, 0x0014d448, 0x001a0038)?;
                                        let t = (c.g[V0][0] as i64) >= 0;
                                        c.g[A0][0] = c.g[V0][0];
                                        if t {
                                            break 'b_1a0048;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1a0058;
                                            } else {
                                                break 'b_1a0048;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = 0x38;
                                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4));
                                    c.mult(0, 2, 4, 2);
                                    c.g[V0][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    break 'b_1a0058;
                                }
                                'b_1a00ac: {
                                    'b_1a0098: {
                                        c.cycles += 3;
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                        let t = c.g[A2][0] == 0u64;
                                        c.g[A0][0] = c.g[S6][0];
                                        if t {
                                            break 'b_1a0098;
                                        } else {
                                            c.cycles += 13;
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[T0][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A3][0] = 0x8;
                                            c.g[RA][0] = sx(0x001a0078);
                                            c.g[T1][0] = 0x1;
                                            call_ptr(c, 0x0019da10, 0x001a0078)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                            c.g[A3][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                            c.g[RA][0] = sx(0x001a008c);
                                            c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                            call_ptr(c, 0x0019cc30, 0x001a008c)?;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0xcbc), 0u64);
                                            let t = true;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0xcb4), 0u64);
                                            if t {
                                                break 'b_1a00ac;
                                            } else {
                                                break 'b_1a0098;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    c.g[A3][0] = 0xffffffffffffffff;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x001a00ac);
                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                    call_ptr(c, 0x0019cc30, 0x001a00ac)?;
                                    break 'b_1a00ac;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                break 'b_1a00b0;
                            }
                            c.cycles += 2;
                            if c.g[S3][0] != 0 {
                                c.g[S3][0] = c.g[V0][0];
                            }
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_1a00b8;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019ffe0);
                        }
                        continue 'l_19ffe0;
                    } else {
                        break 'b_1a00c0;
                    }
                }
            }
        }
        c.cycles += 15;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[RA][0] = sx(0x001a00cc);
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x0019df20, 0x001a00cc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void FEModule_RiderPreviewRequestBoltTextures(FERider* self, BoltList* list,
/// ?, bool keepLoaded)` (0x19fbe0, frame 0x190): releases the rider's current
/// bolt textures, clears the two done flags (+0xcb8, +0xcc0), then for every
/// body-part entry of the character whose bolt has no texture slot yet and
/// carries flag 0x10, builds its file name, acquires the bolt file and records
/// the slot; finally prioritises the character's textures.
pub fn fe_module_rider_preview_request_bolt_textures(c: &mut Cpu) {
    guest(c, |c| {
        'b_19fd20: {
            c.cycles += 37;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.g[S1][0] = c.g[A0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[RA][0] = sx(0x0019fc18);
            c.g[S7][0] = c.g[A3][0];
            call_ptr(c, 0x0014dc80, 0x0019fc18)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019fc24);
            c.g[A1][0] = 0x9;
            call_ptr(c, 0x0014dd58, 0x0019fc24)?;
            c.g[RA][0] = sx(0x0019fc2c);
            c.g[S6][0] = c.g[V0][0];
            call_ptr(c, 0x0014bdb8, 0x0019fc2c)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[RA][0] = sx(0x0019fc48);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, 0x0019fa78, 0x0019fc48)?;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0xcb8), 0u64);
            c.g[RA][0] = sx(0x0019fc5c);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xcc0), 0u64);
            call_ptr(c, 0x0014d988, 0x0019fc5c)?;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[S3][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019fc6c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014d998, 0x0019fc6c)?;
            let empty = (c.g[S3][0] as i64) <= 0;
            c.g[S2][0] = c.g[V0][0];
            if empty {
                break 'b_19fd20;
            }
            c.cycles += 1;
            'l_19fc78: loop {
                'b_19fca4: {
                    'b_19fca0: {
                        c.cycles += 7;
                        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x288));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                        let neg = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if neg {
                            break 'b_19fca0;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        break 'b_19fca4;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0u64;
                }
                'b_19fd18: {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0019fcb4);
                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                    call_ptr(c, 0x0019cce8, 0x0019fcb4)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                        break 'b_19fd18;
                    }
                    'b_19fd14: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] &= 0x10;
                        let no_flag = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S6][0];
                        if no_flag {
                            break 'b_19fd14;
                        }
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A3][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0019fce0);
                        c.g[T0][0] = c.g[SP][0];
                        call_ptr(c, 0x0014b988, 0x0019fce0)?;
                        let no_name = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S5][0];
                        if no_name {
                            break 'b_19fd14;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = 0xb;
                        c.g[T0][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x0019fd00);
                        c.g[T1][0] = 0x1;
                        call_ptr(c, 0x0019da10, 0x0019fd00)?;
                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                        c.g[RA][0] = sx(0x0019fd14);
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x0019cd38, 0x0019fd14)?;
                    }
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                }
                c.cycles += 2;
                let more = c.g[S3][0] != 0u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x38));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019fc78);
                    }
                    continue 'l_19fc78;
                }
                break 'b_19fd20;
            }
        }
        c.cycles += 14;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[RA][0] = sx(0x0019fd2c);
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x0019e070, 0x0019fd2c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
        c.pc = t;
        Ok(())
    })
}

/// `void FEModule_RiderPreviewReleaseBoltTextures(FEModule* self, list)` (0x19fa78,
/// frame 0x90): when the preview's profile index (+4) and character id (+0)
/// are valid, walks the character's bolt part entries (0x38 bytes each, count
/// from the profile DB), and for each one whose bolt texture slot is
/// assigned and whose part type is not 6 releases the bolt file texture and
/// clears the slot to -1; a type 6 entry keeps its texture and clears the
/// done flag. Stores the flag at +0xcb8 (`mDoneLoadingTextures`).
pub fn fe_module_rider_preview_release_bolt_textures(c: &mut Cpu) {
    guest(c, |c| {
        'b_19fbb8: {
            'b_19fbb4: {
                c.cycles += 24;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x0019faac);
                c.g[S4][0] = c.g[A1][0];
                call_ptr(c, 0x0014dc80, 0x0019faac)?;
                c.g[A1][0] = 0x9;
                c.g[RA][0] = sx(0x0019fab8);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0019fab8)?;
                c.g[RA][0] = sx(0x0019fac0);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014bdb8, 0x0019fac0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                let t = (c.g[A1][0] as i64) < 0;
                c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
                if t {
                    break 'b_19fbb4;
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    if (c.g[A2][0] as i64) < 0 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        break 'b_19fbb8;
                    } else {
                        'b_19fafc: {
                            c.cycles += 2;
                            if c.g[S4][0] != 0u64 {
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                break 'b_19fafc;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0019faf4);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0014ad28, 0x0019faf4)?;
                                c.g[S4][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                break 'b_19fafc;
                            }
                        }
                        'b_19fbb0: {
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x0019fb08);
                            c.g[S5][0] = 0x1;
                            call_ptr(c, 0x0014d988, 0x0019fb08)?;
                            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x0019fb18);
                            c.g[S0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014d998, 0x0019fb18)?;
                            let t = (c.g[S0][0] as i64) <= 0;
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_19fbb0;
                            } else {
                                c.cycles += 2;
                                c.g[S3][0] = c.g[S0][0];
                                c.g[S7][0] = 0x6;
                                'l_19fb28: loop {
                                    'b_19fb54: {
                                        'b_19fb50: {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x288));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            if t {
                                                break 'b_19fb50;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                                let t = true;
                                                c.g[S0][0] = sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_19fb54;
                                                } else {
                                                    break 'b_19fb50;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S0][0] = 0u64;
                                        break 'b_19fb54;
                                    }
                                    'b_19fba8: {
                                        c.cycles += 7;
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                        c.g[RA][0] = sx(0x0019fb64);
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        Callee {
                                            addr: 0x0019cce8,
                                            f: ee::s03::BoltFileIndexManager_GetTextureSlot,
                                        }
                                        .call(c, 0x0019fb64)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        if (c.g[A2][0] as i64) < 0 {
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                            break 'b_19fba8;
                                        } else {
                                            'b_19fba4: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lb((c.g[S1][0] as u32).wrapping_add(0x3));
                                                if c.g[V0][0] == c.g[S7][0] {
                                                    c.g[S5][0] = 0u64;
                                                    break 'b_19fba4;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                                    c.g[RA][0] = sx(0x0019fb88);
                                                    c.g[A0][0] = c.g[S6][0];
                                                    Callee {
                                                        addr: 0x0019dc20,
                                                        f: ee::s03::BoltFile_Release,
                                                    }
                                                    .call(c, 0x0019fb88)?;
                                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                                    c.g[A3][0] = 0xffffffffffffffff;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1788));
                                                    c.g[RA][0] = sx(0x0019fb9c);
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                    Callee { addr: 0x0019cd38, f: ee::s03::BoltFileIndexManager_SetTextureSlot }.call(c, 0x0019fb9c)?;
                                                    let t = true;
                                                    c.g[S3][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                                    if t {
                                                        break 'b_19fba8;
                                                    } else {
                                                        break 'b_19fba4;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                            break 'b_19fba8;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S3][0] != 0u64;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0019fb28);
                                        }
                                        continue 'l_19fb28;
                                    } else {
                                        break 'b_19fbb0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xcb8), c.g[S5][0]);
                        break 'b_19fbb4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_19fbb8;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `FEModule_RiderPreviewSetupModel(self)` (0x0019ed80): flag the model as
/// compiled (`+0xcb4 = 1`), set up the texture files and refs, compile the
/// anim model (`+8`), allocate the 0x70-byte rider anim (`new` with the
/// "RiderAnim" name), give it the cRiderAnim vtable (0x458458, slot `+0x58`),
/// link it to the rider (`+0xc`), play animation 434, set it up and transform
/// the anim model with an identity pose copied to the stack, then find the
/// bones.
pub fn fe_module_rider_preview_setup_model(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x50);
        c.g[A1][0] = 0;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x40);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xcb4), c.g[V0][0]);
        call_ptr(c, 0x0036_9890, 0x0019_edac)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0036_9690, 0x0019_edbc)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        call_ptr(c, 0x0030_dbd0, 0x0019_edc8)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A3][0] = 0;
        c.g[A2][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb28));
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0019_ede0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_18d0, 0x0019_edec)?;
        c.g[V1][0] = 0x46_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7ba8));
        c.g[A0][0] = c.g[S0][0];
        let anim = c.g[S0][0] as u32;
        c.sw(anim.wrapping_add(0x58), c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(anim.wrapping_add(0x60), 0);
        c.sw(anim.wrapping_add(0x64), 0);
        call_ptr(c, 0x0010_4d50, 0x0019_ee10)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x1b2;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_28e8, 0x0019_ee28)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0031_2598, 0x0019_ee34)?;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 1;
        c.g[V0][0] = 0x50_0000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xdd0));
        c.g[A1][0] = c.g[SP][0];
        // The identity pose (two 0x20-byte halves) copied from 0x4ff230 to
        // the stack, twice.
        let pose = c.g[V0][0] as u32;
        c.lq(6, pose.wrapping_add(0x10));
        c.lq(3, pose);
        frame.save(c, 6, 0x10);
        frame.save(c, 3, 0);
        frame.save(c, 3, 0x20);
        frame.save(c, 6, 0x30);
        call_ptr(c, 0x0031_0200, 0x0019_ee6c)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0019_e680,
            f: ee::s03::FEModule_RiderPreviewFindBones,
        }
        .call(c, 0x0019_ee74)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `int FEModule_RiderPreviewApplyBoltMaterials(RiderPreview* p)` (0x19f780).
///
/// Front-end rider preview: looks up the rider's model (`BE_GetRiderCharDBField`
/// style lookup through the BE with slot 9), and for every entry of its part
/// list whose flag `0x10` is set, finds the matching texture/material record
/// (0x14d448) and, when that record's variant byte (+0x10) is not negative,
/// applies it as a model part variant (0x30ec18) to the preview model
/// (`p+8`). Finally stores 1 through the pointer at `p+0xc74` and returns 1.
pub fn fe_module_rider_preview_apply_bolt_materials(c: &mut Cpu) {
    guest(c, |c| {
        'b_19f848: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x0019f7a4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x9;
            call_ptr(c, 0x0014dd58, 0x0019f7b0)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014bdb8, 0x0019f7b8)?;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.lw(c.g[S2][0] as u32);
            c.g[S3][0] = c.g[V0][0];
            call_ptr(c, 0x0014ad28, 0x0019f7cc)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28c));
            call_ptr(c, 0x0030eb60, 0x0019f7dc)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[S4][0] = 0x38;
            if t {
                break 'b_19f848;
            } else {
                c.cycles += 1;
                'l_19f7e8: loop {
                    'b_19f840: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] = c.g[V0][0] & 0x10;
                        if c.g[V0][0] == 0u64 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            break 'b_19f840;
                        } else {
                            'b_19f824: {
                                'b_19f818: {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                    call_ptr(c, 0x0014d448, 0x0019f808)?;
                                    let t = (c.g[V0][0] as i64) >= 0;
                                    c.g[V1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_19f818;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_19f824;
                                        } else {
                                            break 'b_19f818;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.mult(0, 3, 3, 20);
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                break 'b_19f824;
                            }
                            c.cycles += 3;
                            c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x10));
                            if (c.g[A1][0] as i64) < 0 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                break 'b_19f840;
                            } else {
                                c.cycles += 4;
                                c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                call_ptr(c, 0x0030ec18, 0x0019f83c)?;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                break 'b_19f840;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S1][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019f7e8);
                        }
                        continue 'l_19f7e8;
                    } else {
                        break 'b_19f848;
                    }
                }
            }
        }
        c.cycles += 11;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc74));
        c.g[V0][0] = 0x1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        ret(c);
        return Ok(());
    })
}

/// `void FEModule_DestroyRiderPreviews(FEModule* self, int flags)` (0x1a0420):
/// unloads the rider previews, runs the destructor on the two previews at
/// `self+0x19c0-0xce0` and `self+0xce0`/`self+0` (0xce0 bytes each, last to
/// first; nothing for a null `self`), and deletes `self` when `flags & 1`.
pub fn fe_module_destroy_rider_previews_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001a0440);
        c.g[S2][0] = c.g[A1][0];
        Callee {
            addr: 0x001a04c8,
            f: ee::s03::f_001a04c8,
        }
        .call(c, 0x001a0440)?;
        let null = c.g[S1][0] == 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c0));
        // Skipping straight to the flag test (no `+1` block charge) happens
        // only when `self + 0x19c0 == self`; it cannot, but keep the shape.
        let mut skip_charge = false;
        if !null {
            c.cycles += 2;
            let same = c.g[S1][0] == c.g[S0][0];
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if same {
                skip_charge = true;
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xce0));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[RA][0] = sx(0x001a0464);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0019e498,
                        f: ee::s03::f_0019e498,
                    }
                    .call(c, 0x001a0464)?;
                    let t = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xce0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001a0458);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
        if !skip_charge {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let t = c.g[V0][0] == 0;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001a0480);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00317e50, 0x001a0480)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

pub(crate) const FE_MODULE_PENDING_SPEECH_A: u32 = 0xc1c;

pub(crate) const FE_MODULE_PENDING_SPEECH_B: u32 = 0xc20;

/// `void FEModule_RiderPreviewPlayPendingSpeech(FEModule* this)` (0x1a0358).
///
/// Two pending speech handles (`this+0xc1c`, `this+0xc20`): the first one,
/// if set, is passed with `*this` to 0x2a1778 and cleared; otherwise the
/// second one, if set, is passed with `*this` to 0x2a16b0 and cleared. Both
/// start with a 0x28b180 call that yields the sound system.
pub fn fe_module_rider_preview_play_pending_speech(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_A));
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x001a_037c)?;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002a_1778, 0x001a_0388)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_A),
                0,
            );
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_B));
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 6;
                call_ptr(c, 0x0028_b180, 0x001a_03a0)?;
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a_16b0, 0x001a_03ac)?;
                c.sw(
                    (c.g[S0][0] as u32).wrapping_add(FE_MODULE_PENDING_SPEECH_B),
                    0,
                );
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RiderPreviewRequestExtraTextures(FEModule* self)`
/// (0x19e7f0): for the character's bolt entries of type 6, request the
/// texture of each one that has no texture slot yet (`BoltFile_Acquire` +
/// `SetTextureSlot`).
pub fn femodule_rider_preview_request_extra_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x190);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S0, 0x180);
        frame.save(c, S1, 0x170);
        frame.save(c, S2, 0x160);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x140);
        frame.save(c, S5, 0x130);
        frame.save(c, S6, 0x120);
        frame.save(c, S3, 0x150);
        frame.save(c, S7, 0x110);
        frame.save_ra(c, 0x100);
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x7c));
        c.g[S6][0] = sx(r32(c, V1).wrapping_add(0x1a70));
        call_ptr(c, 0x0014_dc80, 0x0019_e82c)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, 0x0014_dd58, 0x0019_e838)?; // BELibrary_GetInterface
        c.g[S5][0] = c.g[V0][0];
        call_ptr(c, 0x0014_bdb8, 0x0019_e840)?; // Bolts_GetDB
        c.g[A2][0] = c.lw(r32(c, S1));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x0014_ad28, 0x0019_e854)?; // BE_GetCharProfileData
        c.g[A1][0] = c.lw(r32(c, S1));
        c.g[A0][0] = c.g[S0][0];
        c.g[S4][0] = c.g[V0][0];
        call_ptr(c, 0x0014_d998, 0x0019_e864)?; // Bolts_GetCharBolts
        c.g[A1][0] = c.lw(r32(c, S1));
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_d988, 0x0019_e874)?; // Bolts_GetCharBoltCount
        let empty = (c.g[V0][0] as i64) <= 0;
        c.sw(r32(c, S1).wrapping_add(0xcb8), 0);
        if !empty {
            c.cycles += 3;
            c.g[S3][0] = c.g[V0][0];
            c.g[S7][0] = 6;
            loop {
                'next_bolt: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(r32(c, S2).wrapping_add(3));
                    if c.g[V0][0] != c.g[S7][0] {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        break 'next_bolt;
                    }
                    'have_tex: {
                        'no_tex: {
                            c.cycles += 7;
                            c.g[V0][0] = c.lh(r32(c, S2).wrapping_add(4));
                            c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(0x288));
                            c.g[V0][0] = sx(r32(c, V0) << 1);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                            c.g[V0][0] = c.lh(r32(c, V0));
                            let neg = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            if neg {
                                break 'no_tex;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x290));
                            c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                            break 'have_tex;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = 0;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1788));
                    c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
                    c.g[A2][0] = c.lh(r32(c, S0));
                    call_ptr(c, 0x0019_cce8, 0x0019_e8d0)?; // GetTextureSlot
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        break 'next_bolt;
                    }
                    'request: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A2][0] = c.lw(r32(c, S1));
                        c.g[A3][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[SP][0];
                        call_ptr(c, 0x0014_b988, 0x0019_e8f0)?; // GetBoltTextureName
                        let none = c.g[V0][0] == 0;
                        c.g[A0][0] = c.g[S6][0];
                        if none {
                            break 'request;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = c.lw(r32(c, S1));
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = 0xb;
                        c.g[T0][0] = 1;
                        c.g[T1][0] = 1;
                        call_ptr(c, 0x0019_da10, 0x0019_e910)?; // BoltFile_Acquire
                        c.g[A2][0] = c.lh(r32(c, S0));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1788));
                        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(4));
                        call_ptr(c, 0x0019_cd38, 0x0019_e924)?; // SetTextureSlot
                    }
                    c.cycles += 1;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                }
                c.cycles += 2;
                let more = c.g[S3][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_e888);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x180);
        frame.restore(c, S1, 0x170);
        frame.restore(c, S2, 0x160);
        frame.restore(c, S3, 0x150);
        frame.restore(c, S4, 0x140);
        frame.restore(c, S5, 0x130);
        frame.restore(c, S6, 0x120);
        frame.restore(c, S7, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// The shared body of the two `MoveLocal` functions: rotate the offset
/// quad `vf5` by the quad at `rot` (a cross-product pair, `w` lanes from the
/// scalar parts), add it to the position quad at `pos` and re-apply the
/// transform. `ra` is the return address of the `ApplyTransform` call.
#[inline(always)]
pub(crate) fn rider_preview_move_local_body(
    c: &mut Cpu,
    rot: u32,
    pos: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 27;
    let frame = Frame::open(c, 0x30);
    c.g[V1][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.lqc2(4, r32(c, V1).wrapping_add(rot));
    c.lqc2(5, r32(c, A1));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    let a = frame.at(c, 0x10);
    c.sqc2(8, a);
    c.lq(2, a);
    let a0 = frame.at(c, 0);
    c.sq(2, a0);
    c.lqc2(3, r32(c, V1).wrapping_add(pos));
    c.lqc2(4, a0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, V1).wrapping_add(pos));
    call_ptr(c, 0x0019_f2d0, ret)?; // FEModule_RiderPreviewApplyTransform
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void FEModule_RiderPreviewMoveLocal(FEModule* self, sQuad* delta)`
/// (0x19f418): move the first preview transform (+0xc30) by `delta` rotated
/// by the rotation quad at +0xc40.
pub fn femodule_rider_preview_move_local(c: &mut Cpu) {
    guest(c, |c| {
        rider_preview_move_local_body(c, 0xc40, 0xc30, 0x0019_f478)
    });
}

/// `void FEModule_RiderPreviewMoveLocal2(FEModule* self, sQuad* delta)`
/// (0x19f578): the same for the second transform (+0xc50, rotation +0xc60).
pub fn femodule_rider_preview_move_local2(c: &mut Cpu) {
    guest(c, |c| {
        rider_preview_move_local_body(c, 0xc60, 0xc50, 0x0019_f5d8)
    });
}

/// `int FEModule_RiderPreviewGetAnimId(FEModule* self)` (0x19f740): layer 2
/// of the rider animation controller (`self+0xc`).
pub fn femodule_rider_preview_get_anim_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 2;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0031_2aa0, 0x0019_f754)?; // AnimController_GetLayerAnimId
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `float FEModule_RiderPreviewGetAnimProgress(FEModule* self)` (0x19f760):
/// layer 2's progress.
pub fn femodule_rider_preview_get_anim_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 2;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0031_2ab0, 0x0019_f774)?; // AnimController_GetLayerProgress
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_RiderPreviewUpdate(RiderPreview* self)` (0x19f138): per-frame
/// update of a front-end rider preview. While a character load is pending
/// (`+0xcd4 >= 0`) it waits until the vtable count passes the pending index and
/// then restarts the load (`FEModule_RiderPreviewLoadChar`); otherwise, while
/// `state < 10`, it steps through the load stages (models polled `+0xcbc`,
/// textures polled `+0xcb8`, model set up `+0xcb4`, bolt materials applied
/// `+0xcc0`). Once the preview is ready (`+0xcc8`) it ticks the animation
/// (`+0xc`), applies the transform and updates the two attached effects
/// (`+0xc70`, `+0xc74`). Converted from the generated labelled-block body
/// with every register, cycle and call kept; any other `pc` (a resume) runs
/// the generated body.
pub fn fe_rider_preview_update(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_19f2c4: {
            'b_19f27c: {
                'b_19f278: {
                    'b_19f198: {
                        c.cycles += 7;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[S0][0] = c.g[A0][0];
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcd4));
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if t {
                            break 'b_19f198;
                        } else {
                            c.cycles += 9;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0019f168);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019f168)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcd4));
                            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_19f278;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc24));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc28));
                                c.g[RA][0] = sx(0x0019f190);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xcd4), c.g[V0][0]);
                                Callee {
                                    addr: 0x0019e960,
                                    f: ee::s03::FEModule_RiderPreviewLoadChar,
                                }
                                .call(c, 0x0019f190)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                if t {
                                    break 'b_19f27c;
                                } else {
                                    break 'b_19f198;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_19f2c4;
                    } else {
                        'b_19f1dc: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
                                break 'b_19f1dc;
                            } else {
                                'b_19f1cc: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019f1bc);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a0100,
                                        f: ee::s03::FEModule_RiderPreviewPollModels,
                                    }
                                    .call(c, 0x0019f1bc)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19f1cc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), c.g[V0][0]);
                                        if t {
                                            break 'b_19f278;
                                        } else {
                                            break 'b_19f1cc;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                    break 'b_19f27c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
                                    break 'b_19f1dc;
                                }
                            }
                        }
                        'b_19f214: {
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                                break 'b_19f214;
                            } else {
                                'b_19f210: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019f1ec);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x0019fd58,
                                        f: ee::s03::FEModule_RiderPreviewPollTextures,
                                    }
                                    .call(c, 0x0019f1ec)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19f210;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb4));
                                        let t = c.g[V1][0] != 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb8), c.g[V0][0]);
                                        if t {
                                            break 'b_19f278;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x0019f208);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x0019ed80,
                                                f: ee::s03::FEModule_RiderPreviewSetupModel,
                                            }
                                            .call(c, 0x0019f208)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                            if t {
                                                break 'b_19f27c;
                                            } else {
                                                break 'b_19f210;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcbc));
                                break 'b_19f214;
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                            break 'b_19f27c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                break 'b_19f27c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb4));
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                    break 'b_19f27c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc0));
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                        break 'b_19f27c;
                                    } else {
                                        'b_19f274: {
                                            'b_19f26c: {
                                                c.cycles += 8;
                                                c.g[RA][0] = sx(0x0019f248);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee { addr: 0x0019f780, f: ee::s03::FEModule_RiderPreviewApplyBoltMaterials }.call(c, 0x0019f248)?;
                                                c.g[V0][0] = 0x1;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
                                                c.g[A1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xcc8));
                                                c.g[A0][0] = 0u64;
                                                let t = c.g[V1][0] != 0u64;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xcc0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_19f26c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xcc8)
                                                        );
                                                    if c.g[V0][0] == 0u64 {
                                                        c.sw(c.g[A1][0] as u32, c.g[A0][0]);
                                                        break 'b_19f274;
                                                    } else {
                                                        break 'b_19f26c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = 0x1;
                                            c.sw(c.g[A1][0] as u32, c.g[A0][0]);
                                            break 'b_19f274;
                                        }
                                        c.cycles += 1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc4), 0u64);
                                        break 'b_19f278;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
                break 'b_19f27c;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_19f2c4;
            } else {
                'b_19f2a8: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        break 'b_19f2a8;
                    } else {
                        c.cycles += 6;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 12);
                        c.g[RA][0] = sx(0x0019f2a0);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x003123c0, 0x0019f2a0)?;
                        c.g[RA][0] = sx(0x0019f2a8);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x00312490, 0x0019f2a8)?;
                        break 'b_19f2a8;
                    }
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x0019f2b0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0019f2d0,
                    f: ee::s03::FEModule_RiderPreviewApplyTransform,
                }
                .call(c, 0x0019f2b0)?;
                c.g[RA][0] = sx(0x0019f2b8);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
                call_ptr(c, 0x002f1a08, 0x0019f2b8)?;
                c.g[RA][0] = sx(0x0019f2c0);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc74));
                call_ptr(c, 0x002ecf78, 0x0019f2c0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_19f2c4;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_install_fakes;
    use crate::game::lib::snd::tests::oc1130_install;
    pub(crate) use crate::game::lib::snd::tests::oc792_install_fakes;
    pub(crate) use crate::game::sound::bankmanager::tests::oc786_install_fakes;
    pub(crate) use crate::game::sound::bankmanager::tests::oc788_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc760_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc749_install;
    pub(crate) use crate::game::ui::uimenu::tests::oc776_install_plain_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2;

    pub(crate) use crate::game::main::debugmenu::tests::w;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    use ssx3_ee as ee;

    use ssx3_ee::s03;

    #[test]
    fn femodule_get_rider_preview_2_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a0538,
            femodule_get_rider_preview_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn femodule_get_rider_preview_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a0548,
            femodule_get_rider_preview,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn fe_module_set_preview_visible_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a0570__generated,
            fe_module_set_preview_visible,
            200,
            &|c, seed| {
                let mut r = rng(seed, 0x1a);
                scramble(c, &mut r);
                let index = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = index as u64;
                c.g[A2][0] = (seed % 3 != 0) as u64;
                let base = DATA + index * 0xce0;
                // Cover the early-out branches of the preview update.
                for off in [0xcb4u32, 0xcb8, 0xcc0, 0xcc4, 0xcc8] {
                    c.bus.write32(
                        base + off,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32()
                        },
                    );
                }
                c.bus.write32(base + 0xcd4, r.next_u32() | 0x8000_0000);
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_transform_matches() {
        check(
            s03::f_0019f3e8__generated,
            femodule_rider_preview_set_transform,
            120,
            &|c, seed| {
                machine(c, seed, 0x19f3e8, 1);
                clear(c, OBJ, 0x100);
                clear(c, CHILD, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                for i in 0..16u32 {
                    c.bus
                        .write32(CHILD + 4 * i, seed.wrapping_mul(0x0101_0101) ^ i);
                }
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_transform2_matches() {
        check(
            s03::f_0019f548__generated,
            femodule_rider_preview_set_transform2,
            120,
            &|c, seed| {
                machine(c, seed, 0x19f548, 1);
                clear(c, OBJ, 0x100);
                clear(c, CHILD, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                for i in 0..16u32 {
                    c.bus
                        .write32(CHILD + 4 * i, seed.wrapping_mul(0x0101_0101) ^ i);
                }
            },
        );
    }

    #[test]
    fn femodule_set_rider_preview_indices_matches() {
        check(
            s03::f_001a0498__generated,
            femodule_set_rider_preview_indices,
            200,
            &|c, seed| {
                let mut r = Rng(0x1a04_98aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x1c00);
                c.bus.write32(OBJ + 4, r.next_u32());
                c.bus.write32(OBJ + 0xce4, r.next_u32());
            },
        );
    }

    #[test]
    fn fe_module_request_preview_char_matches() {
        check(
            ssx3_ee::s03::f_001a0508__generated,
            fe_module_request_preview_char,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xa1, 1);
                globals(c);
                let idx = (seed % 3) as u32;
                // The callee compares the character it is asked for with
                // `*(preview)`; `*(preview + 0x10)` and `*(+0xcd4)` stay zero, so
                // both variants take one of the two call-free paths.
                let want = (seed % 2) as u32;
                let stored = if seed % 4 < 2 { want } else { want ^ 1 };
                c.bus.write32(DATA + idx * 0xce0, stored);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = want as u64;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn fe_module_unload_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a04c8__generated,
            fe_module_unload_rider_previews,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_module_update_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a0608__generated,
            fe_module_update_rider_previews,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_module_init_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a03c0__generated,
            fe_module_init_rider_previews,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1a03c0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn fe_module_render_rider_previews_matches() {
        check(
            ee::s03::f_001a0598__generated,
            fe_module_render_rider_previews,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1a05_98);
                let canvas = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x854, canvas);
                fill(c, &mut r, canvas, 0x400);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xcc8, 0);
                c.bus.write32(DATA + 0xce0 + 0xcc8, 0);
            },
        );
    }

    #[test]
    fn fe_module_rider_preview_unload_matches() {
        check(
            ee::s03::f_0019eba0__generated,
            fe_module_rider_preview_unload,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19eb, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 8, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0xcu32, 0xc74, 0xc70].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let slot = match *off {
                            0xc => 0x58,
                            0xc74 => 0xc4,
                            _ => 0xa4,
                        };
                        let vt = vtable(c, &mut r, &[slot]);
                        c.bus.write32(o + slot, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn fe_module_rider_preview_find_bones_matches() {
        check(
            ee::s03::f_0019e680__generated,
            fe_module_rider_preview_find_bones,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x19_e680, 14);
                c.g[A0][0] = DATA as u64;
                // A model with no bones: every lookup takes the not-found path.
                w(c, DATA + 8, DATA + 0x2000);
            },
        );
    }

    pub(crate) const RIDER_TABLE: u32 = DATA + 0x1_0000;

    pub(crate) const TEX_PKG: u32 = DATA + 0x1_1000;

    #[test]
    fn fe_module_rider_preview_load_shape_textures_matches() {
        check_all(
            ee::s03::f_0019f908__generated,
            fe_module_rider_preview_load_shape_textures,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x19f9_08, 10);
                c.g[A0][0] = sx(RIDER_TABLE);
                c.g[A1][0] = sx(TEX_PKG);
                c.g[A2][0] = sx(seed % 2);
                c.bus.write32(TEX_PKG + 0x8, (seed % 3) as u32); // entry count
                c.bus.write32(RIDER_TABLE + 0x4, seed % 3); // stored entry count
                if seed % 2 == 0 {
                    c.bus.write32(TEX_PKG + 0xc, 0x10); // first entry offset
                    c.bus.write32(TEX_PKG + 0x10, 0x20);
                }
                let _ = &mut r;
            },
        );
    }

    /// `Irradiance_AddCameraRelativeLight`: the camera's view matrix comes
    /// from a fake virtual method (call 0, unwinding on some seeds);
    /// `Math_Atan/Asin/SinCos`, `Irradiance_Rotate` and
    /// `Irradiance_AddScaledW` run for real on the ELF image's constants.
    /// Branches: a zero view and a sun straight above (no horizontal
    /// component), a zero `up` vector (degenerate length), a huge `up`
    /// (cosine outside [-1, 1]: both clamps), dir.y of either sign, and the
    /// axis-swap matrix built or not.
    #[test]
    fn irradiance_add_camera_relative_light_matches() {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{check, elf_image, fill, put_vec, Rng, DATA, GAME_GP, SP as STACK};
        const VIEW: u32 = DATA + 0x1000;
        const CAMERA: u32 = DATA + 0x2000;
        const VTABLE: u32 = DATA + 0x3000;
        const SUN: u32 = DATA + 0x4000;
        const IRR: u32 = DATA + 0x5000;
        fn vec(r: &mut Rng, range: f32) -> [u32; 4] {
            [r.f32(range), r.f32(range), r.f32(range), r.f32(range)]
        }
        fn matrix(c: &mut Cpu, r: &mut Rng, at: u32, range: f32) {
            for i in 0..4 {
                let v = vec(r, range);
                put_vec(c, at + 0x10 * i, v);
            }
        }
        check(
            ee::s12::f_00389cb8__generated,
            irradiance_add_camera_relative_light,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0038_9cb8, 1);
                c.pc = 0x0038_9cb8;
                // Stale stack under the frame (read before written).
                fill(c, &mut r, STACK - 0x300, 0xc0);
                // camera->GetViewMatrix() through the vtable, returning VIEW.
                c.bus.write32(GAME_GP + CRL_CAMERA as u32, CAMERA);
                c.bus.write32(CAMERA + CRL_CAMERA_VT, VTABLE);
                let delta = (r.next_u32() as i16 & !3) as u16;
                c.bus.write16(VTABLE + CRL_VIEW_MATRIX_SLOT, delta);
                c.bus.write32(VTABLE + CRL_VIEW_MATRIX_SLOT + 4, FAKE);
                c.bus.write32(LOG_RESULTS, VIEW);
                let overhead = seed % 5 == 0;
                if overhead {
                    for i in 0..4 {
                        put_vec(c, VIEW + 0x10 * i, [0; 4]);
                    }
                } else {
                    matrix(c, &mut r, VIEW, 2.0);
                }
                let mut sun = vec(&mut r, 100.0);
                if overhead {
                    // dir = (0, 0, z, 0): no horizontal part (w lane too).
                    sun[0] = 0;
                    sun[1] = 0;
                    sun[3] = 0x3f80_0000;
                }
                put_vec(c, SUN, sun);
                let up = match seed % 7 {
                    1 => [0; 4],
                    2 | 4 => vec(&mut r, 1e25),
                    _ => vec(&mut r, 1.0),
                };
                put_vec(c, CRL_UP, up);
                let a1 = vec(&mut r, 1.0);
                put_vec(c, CRL_AXIS_1, a1);
                let a2 = vec(&mut r, 1.0);
                put_vec(c, CRL_AXIS_2, a2);
                let q = vec(&mut r, 1.0);
                put_vec(c, CRL_SWAP, q);
                matrix(c, &mut r, CRL_BASE, 2.0);
                matrix(c, &mut r, CRL_SWAP_MATRIX, 2.0);
                c.bus.write32(GAME_GP + CRL_SWAP_BUILT as u32, seed & 1);
                for i in 0..40 {
                    let v = r.f32(2.0);
                    c.bus.write32(IRR + 4 * i, v);
                }
                c.g[A0][0] = sx(IRR);
                c.g[A1][0] = sx(SUN);
                c.f[12] = r.f32(4.0);
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::put_words;

    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    pub(crate) use crate::game::ai::rider::tests::oc396_fake;
    pub(crate) use crate::game::ai::rider::tests::oc396_fake_call;
    pub(crate) use crate::game::ai::rider::tests::OC396_CTRL;
    pub(crate) use crate::game::ai::rider::tests::OC396_MAGIC;
    pub(crate) use crate::game::ai::rider::tests::OC396_RET;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// A NUL-terminated string at `at`.
    pub(crate) fn put_str_oc396(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    oc396_fake!(
        oc396_fake_apply_pose,
        OC396_PREV_APPLY_POSE,
        0x0031_0200,
        ee::s10::f_00310200__generated
    );

    oc396_fake!(
        oc396_fake_bone_matrices,
        OC396_PREV_BONE_MATRICES,
        0x0031_03f0,
        ee::s10::f_003103f0__generated
    );

    pub(crate) fn oc396_install_model_fakes() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00310200),
            oc396_fake_apply_pose,
            &OC396_PREV_APPLY_POSE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003103F0),
            oc396_fake_bone_matrices,
            &OC396_PREV_BONE_MATRICES,
        );
    }

    /// Both pose modes, the model calls faked.
    #[test]
    fn fe_module_rider_preview_apply_transform_matches() {
        oc396_install_model_fakes();
        covered(
            ee::s03::f_0019f2d0__generated,
            fe_module_rider_preview_apply_transform,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_f2d0, 2);
                c.pc = 0x0019_f2d0;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                let this = DATA + 0x100;
                let model = DATA + 0x1000;
                c.bus.write32(this + RIDER_PREVIEW_MODEL, model);
                c.bus.write32(model + 0x24, DATA + 0x2000);
                c.bus.write32(model + 0x28, DATA + 0x3000);
                c.bus
                    .write32(this + RIDER_PREVIEW_SINGLE_POSE, (seed % 2) * r.next_u32());
                for b in RIDER_PREVIEW_BONES {
                    c.bus.write32(this + b, r.next_u32() % 16);
                }
                fill(c, &mut r, this + RIDER_PREVIEW_POSE, 16);
                fill(c, &mut r, 0x004f_f130, 4);
                fill(c, &mut r, 0x004f_f220, 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    oc396_fake!(
        oc396_fake_irr_clear,
        OC396_PREV_IRR_CLEAR,
        0x0038_9260,
        ee::s12::f_00389260__generated
    );

    oc396_fake!(
        oc396_fake_irr_entry,
        OC396_PREV_IRR_ENTRY,
        0x0038_ac20,
        ee::s12::f_0038ac20__generated
    );

    oc396_fake!(
        oc396_fake_irr_add,
        OC396_PREV_IRR_ADD,
        0x0038_9590,
        ee::s12::f_00389590__generated
    );

    oc396_fake!(
        oc396_fake_irr_cam,
        OC396_PREV_IRR_CAM,
        0x0038_9cb8,
        ee::s12::f_00389cb8__generated
    );

    oc396_fake!(
        oc396_fake_model_render,
        OC396_PREV_MODEL_RENDER,
        0x0031_0640,
        ee::s10::f_00310640__generated
    );

    oc396_fake!(
        oc396_fake_fx_render,
        OC396_PREV_FX_RENDER,
        0x002f_2088,
        ee::s09::f_002f2088__generated
    );

    /// The preview render: not ready (pending, no model, not posed) or every
    /// rider type with and without the outfit override; the root bone found
    /// or not (real lookup, `strcmp`); with and without a layer.
    #[test]
    fn fe_module_rider_preview_render_matches() {
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00389260),
            oc396_fake_irr_clear,
            &OC396_PREV_IRR_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038AC20),
            oc396_fake_irr_entry,
            &OC396_PREV_IRR_ENTRY,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00389590),
            oc396_fake_irr_add,
            &OC396_PREV_IRR_ADD,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00389CB8),
            oc396_fake_irr_cam,
            &OC396_PREV_IRR_CAM,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00310640),
            oc396_fake_model_render,
            &OC396_PREV_MODEL_RENDER,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F2088),
            oc396_fake_fx_render,
            &OC396_PREV_FX_RENDER,
        );
        covered(
            ee::s03::f_0019ee88__generated,
            fe_module_rider_preview_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_ee88, 7);
                c.pc = 0x0019_ee88;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                fill(c, &mut r, OC396_RET, 8);
                put_words(
                    c,
                    0x0046_0b40,
                    &[
                        0x19ef30, 0x19ef44, 0x19ef58, 0x19ef6c, 0x19ef80, 0x19ef94, 0x19efa8,
                        0x19efbc, 0x19efd0, 0x19efe4,
                    ],
                );
                let this = DATA + 0x100;
                let model = DATA + 0x1000;
                let ready = seed % 8 != 0;
                c.bus.write32(
                    this + RIDER_PREVIEW_PENDING,
                    if ready { u32::MAX } else { seed % 3 },
                );
                c.bus.write32(
                    this + RIDER_PREVIEW_MODEL,
                    if seed % 16 == 1 { 0 } else { model },
                );
                c.bus.write32(
                    this + RIDER_PREVIEW_DIRTY,
                    if seed % 16 == 2 { 0 } else { 1 },
                );
                c.bus
                    .write32(this + RIDER_PREVIEW_TYPE, (seed / 2 % 13).wrapping_sub(1));
                c.bus.write32(this + RIDER_PREVIEW_OUTFIT, seed % 2);
                c.bus.write32(
                    this + RIDER_PREVIEW_LAYER,
                    if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 32
                    },
                );
                c.bus.write32(this + RIDER_PREVIEW_FX, r.next_u32());
                // The model's skeleton: the root bone's name list.
                put_str_oc396(c, 0x004a_1978, b"root");
                c.bus.write32(model + 0x1c, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100, 0);
                c.bus.write32(model + 0xc, DATA + 0x1200);
                let skel = DATA + 0x1200;
                let n = r.next_u32() % 3;
                c.bus.write32(skel + 0x44, n);
                c.bus.write32(skel + 0x38, DATA + 0x1400);
                c.bus.write32(skel + 4, r.next_u32() % 4);
                for k in 0..2 {
                    let name: &[u8] = if r.next_u32() % 2 == 0 {
                        b"root"
                    } else {
                        b"spine"
                    };
                    put_str_oc396(c, DATA + 0x1400 + 0x50 * k, name);
                }
                c.bus.write32(model + 0x2c, DATA + 0x1800);
                fill(c, &mut r, DATA + 0x17e0, 0x30);
                // The graphics manager: vtable slot 0x228, draw state.
                let gr = DATA + 0x2000;
                c.bus.write32(GAME_GP - 0x854, gr);
                c.bus.write32(gr + 0x10d8, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 0x228, 0);
                c.bus
                    .write32(DATA + 0x2100 + 0x22c, crate::difftest::fakes::FAKE);
                c.bus.write32(gr + 0xe84, DATA + 0x2200);
                c.bus.write32(DATA + 0x2208, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::fe::debugfe::tests::OC506_THIS;

    pub(crate) use crate::game::fe::debugfe::tests::oc506_arm;
    pub(crate) use crate::game::fe::debugfe::tests::oc506_hook_all;

    #[test]
    fn fe_module_get_rider_preview_2_thunk_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcb70__generated,
            fe_module_get_rider_preview_2_thunk,
            200,
            &|c, seed| {
                oc506_arm(c, seed, 0x1dcb70, 2);
                c.g[A0][0] = OC506_THIS as u64;
            },
        );
    }

    #[test]
    fn fe_module_get_rider_preview_thunk_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcb90__generated,
            fe_module_get_rider_preview_thunk,
            200,
            &|c, seed| {
                oc506_arm(c, seed, 0x1dcb90, 2);
                c.g[A0][0] = OC506_THIS as u64;
            },
        );
    }

    pub(crate) use crate::game::sound::soundsys::tests::oc749_gate;
    pub(crate) use crate::game::sound::soundsys::tests::oc749_res;

    pub(crate) use crate::game::sound::soundsys::tests::OC749_FIND_BOLT;
    pub(crate) use crate::game::sound::soundsys::tests::OC749_GET_DATA;
    pub(crate) use crate::game::sound::soundsys::tests::OC749_IS_LOADED;

    pub(crate) use crate::game::sound::soundsys::tests::OC749_MODEL_SLOT;

    /// `FEModule_RiderPreviewPollModels`: the preview's bolt list (+0x28c count,
    /// ids at +0x290), the bolt-file model-slot table, the loaded flags (some
    /// files not loaded: returns 0), archives (with and without a package), the
    /// bolt DB entries (file 0 or not, index -1 or valid) and the real
    /// `BoltFile_Release` on slots with a high refcount.
    #[test]
    fn fe_module_rider_preview_poll_models_matches() {
        oc749_install();
        check_lohi(
            ee::s03::f_001a0100__generated,
            fe_module_rider_preview_poll_models,
            300,
            &|c, seed| {
                let mut r = Rng(0x001a_0100 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc749_gate(c, &mut r);
                let this = DATA + 0x100;
                let lst = DATA + 0x1000;
                let db = DATA + 0x2000;
                let arr = DATA + 0x2100;
                let s7 = DATA + 0x30000;
                let world = DATA + 0x200;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, r.next_u32() % 2);
                c.bus.write32(this + 4, r.next_u32() % 3);
                c.bus.write32(this + 8, DATA + 0x180);
                c.bus.write32(
                    this + 0xcbc,
                    if seed % 9 == 0 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, s7 - 0x1a70);
                c.bus.write32(GAME_GP.wrapping_sub(0x1788), DATA + 0x400);
                for file in 0..2u32 {
                    for slot in 0..4u32 {
                        c.bus.write32(s7 + 0x12000 * file + 0x120 * slot + 0x114, 9);
                    }
                }
                c.bus.write32(lst + 0x28c, r.next_u32() % 4);
                for i in 0..4u32 {
                    let word = (r.next_u32() & 0xffff_0000) | (r.next_u32() % 8);
                    c.bus.write32(lst + 0x290 + 4 * i, word);
                }
                for k in 0..8u32 {
                    let slot = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(OC749_MODEL_SLOT + 4 * k, slot);
                    c.bus
                        .write32(OC749_IS_LOADED + 4 * k, (r.next_u32() % 6 != 0) as u32);
                    let data = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        DATA + 0x6000
                    };
                    c.bus.write32(OC749_GET_DATA + 4 * k, data);
                    let idx = if r.next_u32() % 4 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write32(OC749_FIND_BOLT + 4 * k, idx);
                }
                oc749_res(c, 0x14dc80, DATA + 0x500);
                oc749_res(c, 0x14dd58, DATA + 0x510);
                oc749_res(c, 0x14ad28, lst);
                oc749_res(c, 0x14bdb8, db);
                c.bus.write32(db + 4, arr);
                for e in 0..3u32 {
                    c.bus.write8(arr + 0x38 * e + 1, r.next_u32() as u8);
                    let file = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(arr + 0x38 * e + 0x18, file);
                }
                let part = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    DATA + 0x7000
                };
                oc749_res(c, 0x314c00, part);
                oc749_res(c, 0x30d8b8, r.next_u32());
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;

    use crate::difftest::SP as TEST_SP;
    pub(crate) use crate::game::fe::debugfe::tests::oc793_machine;

    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_gate;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_index;

    pub(crate) use crate::game::fe::uitemplatemap::tests::OC777_OBJ;
    pub(crate) use crate::game::fe::uitemplatemap::tests::OC777_PAT;
    pub(crate) use crate::game::lib::snd::tests::oc792_arm;

    pub(crate) use crate::game::sound::bankmanager::tests::oc782_fixture;
    pub(crate) use crate::game::sound::bankmanager::tests::oc782_force;

    pub(crate) use crate::game::sound::bankmanager::tests::OC782_OBJ;
    pub(crate) use crate::game::sound::bankmanager::tests::OC786_CTRL;
    pub(crate) use crate::game::sound::bankmanager::tests::OC786_MAGIC;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc760_mail;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc768_machine;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc768_mail;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_BLOB;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_MAGIC;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_OBJ;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_RES;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC768_TABLES;

    pub(crate) use crate::game::sound::soundsys::tests::oc788_arm;
    pub(crate) use crate::game::sound::soundsys::tests::oc788_force;

    pub(crate) use crate::game::ui::uimenu::tests::oc776_force;

    pub(crate) use crate::game::ui::uimenu::tests::OC776_CTRL;
    pub(crate) use crate::game::ui::uimenu::tests::OC776_MAGIC;

    pub(crate) const OC760_WORLD: u32 = DATA + 0x23_0000;

    /// Release the bolt-file textures and models of the preview rider: covers
    /// the two early exits (negative rider / file), empty and full entry lists,
    /// present and absent texture/model slots, and unwinding callees.
    #[test]
    fn fe_module_rider_preview_release_bolt_files_matches() {
        check(
            ee::s03::f_0019ec68__generated,
            fe_module_rider_preview_release_bolt_files,
            200,
            &|c, seed| {
                oc760_install_fakes();
                let mut r = machine(c, seed, 0x19_ec68, 30);
                c.bus.write32(OC760_CTRL, OC760_MAGIC);
                let neg = 0x8000_0000 | r.next_u32();
                let (file, rider) = match seed % 6 {
                    0 => (5, neg),
                    1 => (neg, 5),
                    _ => (r.next_u32() & 0x3ff, r.next_u32() & 0x3ff),
                };
                c.bus.write32(OC760_OBJ, file);
                c.bus.write32(OC760_OBJ + 4, rider);
                c.g[A0][0] = sx(OC760_OBJ);
                c.bus.write32(GAME_GP - 0x848, OC760_WORLD);
                c.bus.write32(OC760_WORLD + 0x7c, r.next_u32() & 0xf_fff0);
                c.bus.write32(GAME_GP - 0x1788, r.next_u32());
                // The rider's index entry: a count (0, negative or 1..5) and
                // 4-byte entries holding a short index.
                let count = match (seed / 6) % 6 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    n => n,
                };
                c.bus.write32(OC760_BLOB + 0x28c, count);
                for i in 0..8 {
                    c.bus.write32(OC760_BLOB + 0x290 + 4 * i, r.next_u32());
                }
                oc760_mail(c, 0x0014_ad28, OC760_BLOB + 3);
                for i in 3..40 {
                    let m = [0, 2, 9][(r.next_u32() % 3) as usize];
                    c.bus.write32(OC760_RES + 4 * i, m);
                }
            },
        );
    }

    /// Make the fake for `addr` return `v` in `v0` (sign-extended).
    pub(crate) fn oc768_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc768_mail(addr), v.wrapping_add(3));
    }

    // -- FEModule_RiderPreviewPollTextures -------------------------------

    pub(crate) const POLL_THIS: u32 = DATA + 0x2000;

    pub(crate) const POLL_LIST: u32 = DATA + 0x3000;

    /// A rider with -1..5 bolts (ids that map to a slot, or -1, through the
    /// gated slot-table fake), all files loaded for two seeds in three, the
    /// "used" table on the stack pre-seeded (the memset fake does not clear
    /// it), a texture data pointer or none, and one seed in five already done.
    #[test]
    fn fe_module_rider_preview_poll_textures_matches() {
        covered(
            ee::s03::f_0019fd58__generated,
            fe_module_rider_preview_poll_textures,
            160,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0019_fd58, 14);
                let world = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, DATA + 0x1800);
                c.bus.write32(GAME_GP - 0x1788, r.next_u32());
                c.bus.write32(POLL_THIS, r.next_u32());
                c.bus.write32(POLL_THIS + 4, r.next_u32());
                let done = if seed % 5 == 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write32(POLL_THIS + 0xcb8, done);
                let n: i32 = [-1, 0, 1, 2, 3, 4, 5][(seed / 5 % 7) as usize];
                c.bus.write32(POLL_LIST + 0x28c, n as u32);
                for i in 0..5u32 {
                    let flags =
                        (r.next_u32() & 0xe0) | if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus
                        .write16(POLL_LIST + 0x290 + 4 * i, r.next_u32() as u16);
                    c.bus.write16(POLL_LIST + 0x292 + 4 * i, flags as u16);
                }
                oc768_force(c, 0x0014_ad28, POLL_LIST);
                for k in 0..8u32 {
                    let slot = if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(OC768_TABLES + 4 * k, slot);
                    let loaded = if seed % 3 != 0 { 1 } else { r.next_u32() & 1 };
                    c.bus.write32(OC768_TABLES + 0x20 + 4 * k, loaded);
                    let data = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x4000 + 0x10 * k
                    };
                    c.bus.write32(OC768_TABLES + 0x40 + 4 * k, data);
                    let used = (r.next_u32() % 3 == 0) as u32;
                    c.bus.write32(TEST_SP - 0x490 + 4 * k, used);
                }
                c.g[A0][0] = sx(POLL_THIS);
            },
        );
    }

    pub(crate) fn put_name(c: &mut Cpu, at: u32, name: &[u8]) {
        for i in 0..8u32 {
            let b = name.get(i as usize).copied().unwrap_or(0);
            c.bus.write8(at + i, b);
        }
    }

    /// `FEModule_RiderPreviewFindTexture`: 0 to 5 preview entries (also a
    /// negative count), with the wanted name at entry k, last, or absent.
    #[test]
    fn fe_module_rider_preview_find_texture_matches() {
        check(
            ee::s03::f_0019f878__generated,
            fe_module_rider_preview_find_texture_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x0019_f878 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                let name = DATA + 0x1000;
                let count = [0u32, 1, 2, 3, 5, 0xffff_fffe, 4][seed as usize % 7];
                c.bus.write32(this + 0x18, count);
                let names: [&[u8]; 5] = [b"rider0", b"rider1", b"abc", b"abd", b"rider12"];
                for i in 0..5u32 {
                    c.bus.write32(this + 0x1c + 0xc * i, 0x100 + 7 * i);
                    put_name(c, this + 0x20 + 0xc * i, names[i as usize]);
                }
                let want: &[u8] = match (seed / 7) % 6 {
                    0 => b"rider0",
                    1 => b"rider1",
                    2 => b"abc",
                    3 => b"abd",
                    4 => b"rider12",
                    _ => b"nope",
                };
                put_name(c, name, want);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
            },
        );
    }

    /// `FEModule_RiderPreviewLoadChar`: already loaded (same character and
    /// slot), a different slot of the same character, the first load
    /// (character -1: nothing to unload), a reload, characters 0..=9 (all
    /// bit masks) and out of range (the table's unknown case), slot 0 or not
    /// (extra textures), a corrupt table entry (jump out), and the unwinds
    /// of its calls. The real, unported request/update callees run on empty
    /// lists (`BE` counts forced to 0).
    #[test]
    fn fe_module_rider_preview_load_char_matches() {
        oc776_install_plain_fakes();
        let table: [u32; 10] = [
            0x19eb2c, 0x19eb34, 0x19eb3c, 0x19eb44, 0x19eb4c, 0x19eb54, 0x19eb5c, 0x19eb64,
            0x19eb6c, 0x19eb74,
        ];
        let zero_list = DATA + 0x6_0000;
        fakes::covered(
            ee::s03::f_0019e960__generated,
            fe_module_rider_preview_load_char,
            700,
            &|c, seed| {
                machine(c, seed, 0x7766, 12);
                c.pc = 0x0019_e960;
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                put_words(c, 0x0046_0b00, &table);
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                let character = (seed % 12) as u64;
                let slot = ((seed / 12) % 3) as u64;
                c.g[A1][0] = sx(0x77 + seed);
                c.g[A2][0] = character;
                c.g[A3][0] = slot;
                // The loaded character: the same (with the same or another slot),
                // none (-1), or another one.
                let old = match (seed / 36) % 4 {
                    0 | 1 => character as u32,
                    2 => u32::MAX,
                    _ => (seed % 5) + 3,
                };
                c.bus.write32(this, old);
                c.bus.write32(
                    this + PREVIEW_SLOT,
                    if (seed / 36) % 4 == 0 { slot as u32 } else { 9 },
                );
                c.bus.write32(this + 0xcc4, (seed % 3 == 0) as u32);
                c.bus.write32(this + 0xcc8, (seed % 5 == 0) as u32);
                // `PreviewUpdate` (real) leaves the model flow alone.
                c.bus.write32(this + 0xcd4, u32::MAX);
                oc776_force(c, 0x0014_bdb8, zero_list);
                oc776_force(c, 0x0014_d998, zero_list);
                oc776_force(c, 0x0014_d988, 0);
                oc776_force(c, 0x0014_ad28, zero_list);
                if seed % 41 == 0 && character < 10 {
                    c.bus
                        .write32(0x0046_0b00 + 4 * character as u32, 0x0019_e970);
                }
            },
        );
    }

    /// Script the results of the fake at `addr`: calls k with bit k of `mask`
    /// set return `value` in `v0`.
    pub(crate) fn oc777_pat(c: &mut Cpu, addr: u32, mask: u32, value: u32) {
        let idx = oc777_index(addr);
        c.bus.write32(OC777_PAT + 8 * idx, mask);
        c.bus.write32(OC777_PAT + 8 * idx + 4, value);
    }

    /// `FEModule_RiderPreviewRequestBoltModels`: a rider with 0..5 model
    /// entries (random ids and 0x10 flags), `force` and `prev` variants, the
    /// model-slot lookups scripted mapped/unmapped, the part database with
    /// parts that have a file or not, `BoltFile_PrioritizeModels` real (over a
    /// zeroed manager, its sort a gated fake).
    #[test]
    fn fe_module_rider_preview_request_bolt_models_matches() {
        oc777_install_fakes();
        check(
            ee::s03::f_0019ff00__generated,
            fe_module_rider_preview_request_bolt_models,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777f, 40);
                c.pc = 0x0019_ff00;
                oc777_gate(c);
                let this = OC777_OBJ + 0x1000;
                let list = OC777_OBJ + 0x1100;
                let prev = OC777_OBJ + 0x1300;
                let world = OC777_OBJ + 0x2000;
                let db = OC777_OBJ + 0x2100;
                let parts = OC777_OBJ + 0x2200;
                let fe = OC777_OBJ + 0x1_0000;
                c.bus.write32(this, r.next_u32() & 0xff);
                c.bus.write32(this + 4, r.next_u32() & 0xf);
                let n = r.next_u32() % 6;
                c.bus.write32(list + 0x28c, n);
                for i in 0..n {
                    c.bus
                        .write16(list + 0x290 + 4 * i, (r.next_u32() & 0x3f) as u16);
                    let flags =
                        (if r.next_u32() & 1 != 0 { 0x10u32 } else { 0 }) | (r.next_u32() & 0x3);
                    c.bus.write16(list + 0x292 + 4 * i, flags as u16);
                }
                c.bus
                    .write16(prev + 2, if r.next_u32() & 1 != 0 { 0x10 } else { 0 });
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, fe);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1788), OC777_OBJ + 0x2300);
                c.bus.write32(db + 4, parts);
                for i in 0..4u32 {
                    let file = if r.next_u32() & 1 != 0 {
                        r.next_u32() | 1
                    } else {
                        0
                    };
                    c.bus.write32(parts + 0x38 * i + 0x28, file);
                }
                // The part-database lookup returns an index 0..2, or -1 on
                // a third of the seeds; the slot lookup answers "unmapped"
                // (-1) for some of its calls.
                oc777_pat(c, 0x0014_bdb8, 0xffff_ffff, db);
                let idx = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    seed % 3 - 1 + (seed / 3) % 2
                };
                oc777_pat(c, 0x0014_d448, 0xffff_ffff, idx);
                oc777_pat(c, 0x0019_cbe0, r.next_u32(), 0xffff_ffff);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = if seed % 3 == 1 { 0 } else { sx(prev) };
                c.g[A3][0] = (seed % 4 == 0) as u64;
                c.g[T0][0] = sx(r.next_u32());
            },
        );
    }

    /// Gated fakes (the suite runs tests in parallel): a fake runs only while
    /// this magic is in the machine's RAM, else the body its slot held.
    pub(crate) const OC781_CTRL: u32 = DATA + 0x2b_0000;

    pub(crate) const OC781_MAGIC: u32 = 0x0781_c001;

    /// Per-callee forced results, one word per fake at `OC781_MAIL + (addr & 0xffffc)`.
    pub(crate) const OC781_MAIL: u32 = DATA + 0x2c_0000;

    /// Result table of the texture-slot fake (0x19cce8: 8 words, indexed by
    /// the id argument & 7).
    pub(crate) const OC781_TABLES: u32 = OC781_CTRL + 0x100;

    pub(crate) fn oc781_mail(addr: u32) -> u32 {
        OC781_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (sign-extended).
    pub(crate) fn oc781_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc781_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (else the fake's pointer) unless it unwound.
    pub(crate) fn oc781_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let mail = c.bus.read32(oc781_mail(addr));
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0019_cce8 => c.g[V0][0] = sx(c.bus.read32(OC781_TABLES + 4 * (a2 & 7))),
            _ if mail != 0 => c.g[V0][0] = sx(mail.wrapping_sub(3)),
            _ => {}
        }
    }

    macro_rules! oc781_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC781_CTRL) != OC781_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc781_fake_body(c, $addr);
                }
            )*
            pub(crate) fn oc781_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc781_fakes!(
        (
            oc781_fake_14dc80,
            OC781_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc781_fake_14dd58,
            OC781_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc781_fake_14bdb8,
            OC781_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc781_fake_14d988,
            OC781_PREV_14D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988
        ),
        (
            oc781_fake_14d998,
            OC781_PREV_14D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998
        ),
        (
            oc781_fake_14b988,
            OC781_PREV_14B988,
            ee::s01::OVERRIDE_0014B988,
            ee::s01::f_0014b988__generated,
            0x0014_b988
        ),
        (
            oc781_fake_19cce8,
            OC781_PREV_19CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8
        ),
        (
            oc781_fake_19da10,
            OC781_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc781_fake_19cd38,
            OC781_PREV_19CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38
        ),
        (
            oc781_fake_19dc20,
            OC781_PREV_19DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc781_fake_418ef8,
            OC781_PREV_418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
        (
            oc781_fake_417828,
            OC781_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc781_fake_317d70,
            OC781_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc781_fake_317e30,
            OC781_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc781_fake_2b4a20,
            OC781_PREV_2B4A20,
            ee::s08::OVERRIDE_002B4A20,
            ee::s08::f_002b4a20__generated,
            0x002b_4a20
        ),
        (
            oc781_fake_2b21e0,
            OC781_PREV_2B21E0,
            ee::s08::OVERRIDE_002B21E0,
            ee::s08::f_002b21e0__generated,
            0x002b_21e0
        ),
        (
            oc781_fake_3b5910,
            OC781_PREV_3B5910,
            ee::s13::OVERRIDE_003B5910,
            ee::s13::f_003b5910__generated,
            0x003b_5910
        ),
        (
            oc781_fake_3e57f8,
            OC781_PREV_3E57F8,
            ee::s14::OVERRIDE_003E57F8,
            ee::s14::f_003e57f8__generated,
            0x003e_57f8
        ),
    );

    /// Common start of a fixture: scrambled registers, the fake hooks, the
    /// gate word and the game's gp.
    pub(crate) fn oc781_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc781_install();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC781_CTRL, OC781_MAGIC);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    pub(crate) const BOLTTEX_THIS: u32 = DATA + 0x2000;

    pub(crate) const BOLTTEX_LIST: u32 = DATA + 0x3000;

    pub(crate) const BOLTTEX_MAP: u32 = DATA + 0x3800;

    pub(crate) const BOLTTEX_PARTS: u32 = DATA + 0x4000;

    /// A character with -1..5 body parts (the 0x19fa78 release and the 0x19e070
    /// prioritise run for real; the file manager, slot table and name builder
    /// are gated fakes), bolts mapped to list items or to none (-1), texture
    /// slots present or not, items with and without the 0x10 flag, names found
    /// or not, a negative char or profile index now and then (the release
    /// returns early) and a fake unwinding in some call.
    #[test]
    fn fe_module_rider_preview_request_bolt_textures_matches() {
        covered(
            ee::s03::f_0019fbe0__generated,
            fe_module_rider_preview_request_bolt_textures,
            200,
            &|c, seed| {
                let mut r = oc781_machine(c, seed, 0x0019_fbe0, 14);
                let world = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, DATA + 0x1800);
                c.bus.write32(GAME_GP - 0x1788, r.next_u32());
                let char_id = if seed % 10 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 8
                };
                let profile = if seed % 7 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 8
                };
                c.bus.write32(BOLTTEX_THIS, char_id);
                c.bus.write32(BOLTTEX_THIS + 4, profile);
                c.bus.write32(BOLTTEX_THIS + 0xcb8, r.next_u32());
                c.bus.write32(BOLTTEX_THIS + 0xcc0, r.next_u32());
                let n: i32 = [-1, 0, 1, 2, 3, 4, 5][(seed / 3 % 7) as usize];
                oc781_force(c, 0x0014_d988, n as u32);
                oc781_force(c, 0x0014_d998, BOLTTEX_PARTS);
                oc781_force(c, 0x0014_b988, (seed / 2) % 2);
                c.bus.write32(BOLTTEX_LIST + 0x288, BOLTTEX_MAP);
                for j in 0..5u32 {
                    let v: u16 = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        (r.next_u32() % 5) as u16
                    };
                    c.bus.write16(BOLTTEX_MAP + 2 * j, v);
                    let flags =
                        (r.next_u32() & 0xe0) | if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus
                        .write16(BOLTTEX_LIST + 0x290 + 4 * j, r.next_u32() as u16);
                    c.bus.write16(BOLTTEX_LIST + 0x292 + 4 * j, flags as u16);
                }
                for i in 0..5u32 {
                    let part = BOLTTEX_PARTS + 0x38 * i;
                    c.bus
                        .write8(part + 3, if r.next_u32() % 3 == 0 { 6 } else { 2 });
                    c.bus.write16(part + 4, (r.next_u32() % 5) as u16);
                }
                for k in 0..8u32 {
                    let slot = if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(OC781_TABLES + 4 * k, slot);
                }
                c.g[A0][0] = sx(BOLTTEX_THIS);
                c.g[A1][0] = sx(BOLTTEX_LIST);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `PATH_PlatInit` (hand-written in `bankmonitor.rs`): clears the 24 path
    /// records over random data, publishes the tables and callbacks and makes
    /// the two (faked) calls, one of which unwinds in some seeds.
    #[test]
    fn path_plat_init_matches() {
        covered(
            ee::s08::f_002ae048__generated,
            crate::game::sound::bankmonitor::path_plat_init,
            48,
            &|c, seed| {
                let mut r = oc781_machine(c, seed, 0x002a_e048, 2);
                for i in 0..24u32 {
                    c.bus.write32(0x004d_3e98 + 4 * i, r.next_u32());
                    for k in 0..10u32 {
                        c.bus.write32(0x004d_3ef8 + 0x28 * i + 4 * k, r.next_u32());
                    }
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x002a_e048;
            },
        );
    }

    pub(crate) const SONGCREATE_P: u32 = DATA + 0x5000;

    pub(crate) const SONGCREATE_Q: u32 = DATA + 0x5400;

    pub(crate) const SONGCREATE_R: u32 = DATA + 0x6000;

    pub(crate) const SONGCREATE_V: u32 = DATA + 0x7000;

    /// `SongInstance_Create` (hand-written in `bankmonitor.rs`): allocation
    /// (fake) and the real `SongInstance_ctor` with random volume tables,
    /// section index, option words, a stack argument that is zero or not (the
    /// update call) and a fake unwinding in some call.
    #[test]
    fn song_instance_create_matches() {
        covered(
            ee::s08::f_002b1b98__generated,
            crate::game::sound::bankmonitor::song_instance_create,
            120,
            &|c, seed| {
                let mut r = oc781_machine(c, seed, 0x002b_1b98, 6);
                elf_image(c);
                c.bus.write32(OC781_CTRL, OC781_MAGIC);
                c.bus.write32(GAME_GP + 0x6fc, DATA + 0x8000);
                c.bus.write32(DATA + 0x8000, DATA + 0x8100);
                c.bus.write32(DATA + 0x8100, DATA + 0x8200);
                for off in (0..0x100u32).step_by(4) {
                    c.bus.write32(SONGCREATE_P + off, r.next_u32());
                }
                c.bus.write32(SONGCREATE_P + 0x88, r.next_u32() % 256);
                c.bus.write32(SONGCREATE_P + 0xa4, seed % 2);
                c.bus.write32(SONGCREATE_Q + 0x1c, r.next_u32() % 4);
                c.bus.write32(SONGCREATE_Q + 0x20, SONGCREATE_R);
                c.bus.write32(SONGCREATE_Q + 0x38, SONGCREATE_V);
                for i in 0..0x40u32 {
                    c.bus.write32(SONGCREATE_R + 4 * i, r.next_u32());
                }
                for i in 0..4u32 {
                    let v: u32 =
                        [0, 1, 126, 127, 128, 200, 0xffff_ffff, 50][(r.next_u32() % 8) as usize];
                    c.bus.write32(SONGCREATE_V + 4 * i, v);
                }
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = sx(DATA + 0x9100);
                c.g[A2][0] = sx(SONGCREATE_P);
                c.g[A3][0] = sx(SONGCREATE_Q);
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.pc = 0x002b_1b98;
            },
        );
    }

    /// `FEModule_RiderPreviewReleaseBoltTextures`: profile index / character
    /// id valid or negative, bolt part entry lists of 1..3 entries (types 6 and
    /// others, slots into the profile's bolt map), texture slots found or not
    /// (the slot lookup returns -1 for some calls).
    #[test]
    fn fe_module_rider_preview_release_bolt_textures_matches() {
        fakes::covered(
            ee::s03::f_0019fa78__generated,
            fe_module_rider_preview_release_bolt_textures,
            128,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782b, 14);
                let this = OC782_OBJ + 0x1000;
                let list = OC782_OBJ + 0x2000;
                let map = OC782_OBJ + 0x3000;
                let ents = OC782_OBJ + 0x4000;
                let neg = |r: &mut Rng| {
                    if r.next_u32() % 5 == 0 {
                        0xffff_fff0
                    } else {
                        r.next_u32() & 0xf
                    }
                };
                let profile = neg(&mut r);
                let chr = neg(&mut r);
                c.bus.write32(this, chr);
                c.bus.write32(this + 4, profile);
                c.bus.write32(this + 0xcb8, r.next_u32());
                c.bus.write32(list + 0x288, map);
                let count = 1 + r.next_u32() % 3;
                for i in 0..4u32 {
                    let e = ents + 0x38 * i;
                    c.bus
                        .write8(e + 3, if r.next_u32() % 2 == 0 { 6 } else { 2 });
                    c.bus.write16(e + 4, i as u16);
                    let v = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        (r.next_u32() % 4) as u16
                    };
                    c.bus.write16(map + 2 * i, v);
                }
                let with_list = seed % 4 != 0;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if with_list { sx(list) } else { 0 };
                if with_list {
                    oc782_force(c, 3, count);
                    oc782_force(c, 4, ents);
                } else {
                    oc782_force(c, 3, list);
                    oc782_force(c, 4, count);
                    oc782_force(c, 5, ents);
                }
            },
        );
    }

    /// A game-like object area well above the fakes' pointer results.
    pub(crate) const OC786_OBJ: u32 = DATA + 0x8_0000;

    /// Arm the gated fakes in this machine and install them once.
    pub(crate) fn oc786_arm(c: &mut Cpu) {
        oc786_install_fakes();
        c.bus.write32(OC786_CTRL, OC786_MAGIC);
    }

    /// `FEModule_RiderPreviewSetupModel`: all callees faked (the rider anim comes from a fake `new`).
    #[test]
    fn fe_module_rider_preview_setup_model_matches() {
        fakes::covered(
            ee::s03::f_0019ed80__generated,
            fe_module_rider_preview_setup_model,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19ed80, 11);
                oc786_arm(c);
                c.g[A0][0] = sx(OC786_OBJ);
                c.bus.write32(OC786_OBJ + 0x8, OC786_OBJ + 0x1000);
                c.bus.write32(OC786_OBJ + 0x18, r.next_u32());
                // The fake `new` (call 3) returns the anim object.
                c.bus
                    .write32(fakes::LOG_RESULTS + 4 * 3, OC786_OBJ + 0x3000);
            },
        );
    }

    /// `FEModule_RiderPreviewApplyBoltMaterials`: empty and negative part
    /// counts, entries with and without the flag, a negative and non-negative
    /// texture index, a negative and non-negative variant byte, and unwinds.
    #[test]
    fn fe_module_rider_preview_apply_bolt_materials_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s03::f_0019f780__generated,
            fe_module_rider_preview_apply_bolt_materials,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788c, 14);
                oc788_arm(c);
                c.pc = 0x0019_f780;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                for off in [0, 4, 8] {
                    c.bus.write32(this + off, r.next_u32());
                }
                c.bus.write32(this + 0xc74, DATA + 0x1200 + 4 * (seed % 3));
                let rec = DATA + 0x2000;
                let count: u32 = [0, -1i32 as u32, 1, 3, 5][seed as usize % 5];
                c.bus.write32(rec + 0x28c, count);
                for i in 0..6 {
                    let w = r.next_u32();
                    c.bus.write16(rec + 0x290 + 4 * i, w as u16);
                    c.bus.write16(rec + 0x292 + 4 * i, (w >> 16) as u16);
                }
                let tbl = DATA + 0x3000;
                let elems = DATA + 0x4000;
                c.bus.write32(tbl + 4, elems);
                let idx: u32 = [-1i32 as u32, 0, 1, 3][(seed as usize / 5) % 4];
                if idx != u32::MAX {
                    let el = elems + idx * 0x38;
                    c.bus
                        .write8(el + 0x10, [0x00u8, 0xff, 0x05][(seed as usize / 20) % 3]);
                    c.bus.write8(el + 1, r.next_u32() as u8);
                }
                oc788_force(c, 0x0014_ad28, rec);
                oc788_force(c, 0x0014_bdb8, tbl);
                oc788_force(c, 0x0014_d448, idx);
            },
        );
    }

    /// The object the tests point their first argument at.
    pub(crate) const OC792_OBJ: u32 = DATA + 0x1_0000;

    /// `FEModule_DestroyRiderPreviews`: null and real `self`, flag bit 0 on and
    /// off; the unload, two destructors and the delete are fakes that may unwind.
    #[test]
    fn fe_module_destroy_rider_previews_matches() {
        oc792_install_fakes();
        fakes::covered(
            ee::s03::f_001a0420__generated,
            fe_module_destroy_rider_previews_2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x792c, 4);
                oc792_arm(c);
                c.pc = 0x001a_0420;
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(OC792_OBJ) };
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `FEModule_RiderPreviewPlayPendingSpeech`: first handle set, only the
    /// second set, none set.
    #[test]
    fn fe_module_rider_preview_play_pending_speech_matches() {
        covered(
            ee::s03::f_001a0358__generated,
            fe_module_rider_preview_play_pending_speech,
            120,
            &|c, seed| {
                let mut r = oc793_machine(c, seed, 0x001a_0358, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let (a, b) = match seed % 4 {
                    0 => (r.next_u32() | 1, r.next_u32() | 1),
                    1 => (0, r.next_u32() | 1),
                    2 => (0, 0),
                    _ => (r.next_u32() | 1, 0),
                };
                c.bus.write32(this + FE_MODULE_PENDING_SPEECH_A, a);
                c.bus.write32(this + FE_MODULE_PENDING_SPEECH_B, b);
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    pub(crate) fn oc859_clear(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// Gate and mailboxes of the oc859 fakes.
    pub(crate) const OC859_CTRL: u32 = DATA + 0x20_1000;

    pub(crate) const OC859_MAGIC: u32 = 0x0c85_9001;

    /// The fake callee `idx`: logs the call (all registers) and returns the
    /// next scripted result (`list[count]`: 0 = the fake's pointer,
    /// otherwise `value + 1` sign-extended).
    pub(crate) fn oc859_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC859_CTRL + 0x40 + 4 * idx);
        c.bus.write32(OC859_CTRL + 0x40 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC859_CTRL + 0x100 + 0x40 * idx + 4 * (n % 16));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = sx(mail.wrapping_sub(1));
        }
    }

    macro_rules! oc859_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC859_CTRL) != OC859_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc859_fake_call(c, $addr, $idx);
                }
            )*
            fn oc859_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc859_fakes!(
        (
            oc859_fake_14dc80,
            OC859_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            0
        ),
        (
            oc859_fake_14dd58,
            OC859_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            1
        ),
        (
            oc859_fake_14bdb8,
            OC859_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8,
            2
        ),
        (
            oc859_fake_14ad28,
            OC859_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28,
            3
        ),
        (
            oc859_fake_14d998,
            OC859_PREV_14D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998,
            4
        ),
        (
            oc859_fake_14d988,
            OC859_PREV_14D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988,
            5
        ),
        (
            oc859_fake_14b988,
            OC859_PREV_14B988,
            ee::s01::OVERRIDE_0014B988,
            ee::s01::f_0014b988__generated,
            0x0014_b988,
            6
        ),
        (
            oc859_fake_19da10,
            OC859_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10,
            7
        ),
        (
            oc859_fake_19cce8,
            OC859_PREV_19CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8,
            8
        ),
        (
            oc859_fake_19cd38,
            OC859_PREV_19CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38,
            9
        ),
    );

    /// Script the results of fake `idx`.
    pub(crate) fn oc859_script(c: &mut Cpu, idx: u32, results: &[u32]) {
        for (i, v) in results.iter().enumerate() {
            c.bus
                .write32(OC859_CTRL + 0x100 + 0x40 * idx + 4 * i as u32, *v);
        }
    }

    /// `RequestExtraTextures`: the bolt-info callees are scripted fakes.
    /// Zero, one or several entries of type 6 or other types, texture
    /// indices negative or not, textures already slotted (result >= 0) or
    /// not, and bolt texture names found or not.
    #[test]
    fn femodule_rider_preview_request_extra_textures_matches() {
        oc859_install_fakes();
        check(
            ee::s03::f_0019e7f0__generated,
            femodule_rider_preview_request_extra_textures,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19e7f0, 0);
                c.pc = 0x0019_e7f0;
                c.bus.write32(OC859_CTRL, OC859_MAGIC);
                let world = DATA + 0x500;
                let this = DATA + 0x1000;
                let prof = DATA + 0x2000;
                let table = DATA + 0x3000;
                let bolts = DATA + 0x4000;
                c.bus.write32(world + 0x7c, r.next_u32());
                c.bus.write32(this, r.next_u32());
                c.bus.write32(this + 4, r.next_u32());
                c.bus.write32(prof + 0x288, table);
                for i in 0..8u32 {
                    c.bus.write16(
                        table + 2 * i,
                        [0xffffu16, 0, 1, 2, 3, 0xffff, 1, 2][i as usize],
                    );
                    c.bus
                        .write16(prof + 0x290 + 4 * i, 0x100 + r.next_u32() as u16 % 0x400);
                }
                let count = [0u32, 1, 3, 4][(seed % 4) as usize];
                for i in 0..4u32 {
                    let e = bolts + 0x38 * i;
                    c.bus
                        .write8(e + 3, if r.next_u32() % 3 == 0 { 2 } else { 6 });
                    c.bus.write16(e + 4, (r.next_u32() % 8) as u16);
                }
                oc859_script(c, 3, &[prof + 1]);
                oc859_script(c, 4, &[bolts + 1]);
                oc859_script(c, 5, &[count + 1]);
                let mut slot_results = [0u32; 4];
                let mut name_results = [0u32; 4];
                for i in 0..4 {
                    // GetTextureSlot: -1 (+1 = 0 is "pointer", so use -2) or >= 0.
                    slot_results[i] = if r.next_u32() % 2 == 0 {
                        0xffff_ffff
                    } else {
                        6
                    };
                    name_results[i] = if r.next_u32() % 2 == 0 { 1 } else { 0x1234 };
                }
                oc859_script(c, 8, &slot_results);
                oc859_script(c, 6, &name_results);
                let mut rr = Rng(seed as u64 | 1);
                fill(c, &mut rr, crate::difftest::SP - 0x1c0, 0x40);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `MoveLocal`/`MoveLocal2`: random preview quads and delta; the real
    /// `ApplyTransform` runs on a cleared model pointer.
    pub(crate) fn oc859_move_local_fixture(c: &mut Cpu, seed: u32, addr: u32) {
        let mut r = machine(c, seed, addr as u64, 1);
        c.pc = addr;
        let this = DATA + 0x100;
        oc859_clear(c, this, 0x400);
        oc859_clear(c, DATA + 0x2000, 0x10);
        for off in [0xc30u32, 0xc40, 0xc50, 0xc60] {
            for i in 0..4 {
                c.bus.write32(this + off + 4 * i, r.f32(100.0));
            }
        }
        for i in 0..4 {
            c.bus.write32(DATA + 0x2000 + 4 * i, r.f32(100.0));
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(DATA + 0x2000);
    }

    #[test]
    fn femodule_rider_preview_move_local_matches() {
        check(
            ee::s03::f_0019f418__generated,
            femodule_rider_preview_move_local,
            120,
            &|c, seed| oc859_move_local_fixture(c, seed, 0x0019_f418),
        );
    }

    #[test]
    fn femodule_rider_preview_move_local2_matches() {
        check(
            ee::s03::f_0019f578__generated,
            femodule_rider_preview_move_local2,
            120,
            &|c, seed| oc859_move_local_fixture(c, seed, 0x0019_f578),
        );
    }

    /// The anim controller at `self+0xc`: a cleared anim list, so the real
    /// `GetLayerAnimId`/`GetLayerProgress` chains stay short.
    pub(crate) fn oc859_anim_fixture(c: &mut Cpu, seed: u32, addr: u32) {
        let mut r = machine(c, seed, addr as u64, 1);
        c.pc = addr;
        let this = DATA + 0x100;
        let ctl = DATA + 0x1000;
        oc859_clear(c, this, 0x20);
        oc859_clear(c, ctl, 0x100);
        oc859_clear(c, DATA + 0x2000, 0x100);
        c.bus.write32(this + 0xc, ctl);
        for i in 0..8u32 {
            c.bus.write32(ctl + 4 * i, r.next_u32());
        }
        c.bus.write32(ctl + 0x50, DATA + 0x2000);
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn femodule_rider_preview_get_anim_id_matches() {
        check(
            ee::s03::f_0019f740__generated,
            femodule_rider_preview_get_anim_id,
            60,
            &|c, seed| oc859_anim_fixture(c, seed, 0x0019_f740),
        );
    }

    #[test]
    fn femodule_rider_preview_get_anim_progress_matches() {
        check(
            ee::s03::f_0019f760__generated,
            femodule_rider_preview_get_anim_progress,
            60,
            &|c, seed| oc859_anim_fixture(c, seed, 0x0019_f760),
        );
    }

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    pub(crate) use crate::game::lib::snd::tests::OC1130_CTRL;
    pub(crate) use crate::game::lib::snd::tests::OC1130_MAGIC;
    pub(crate) use crate::game::lib::snd::tests::OC1130_MAIL;

    /// Make fake number `idx` return `v` in `v0`.
    pub(crate) fn oc1130_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC1130_MAIL + 4 * idx, v.wrapping_add(3));
    }

    /// `FEModule_RiderPreviewUpdate`: a pending character load (the vtable
    /// call's result above or below `pending`, or no load pending), the
    /// five load stages in every flag combination (models polled, textures
    /// polled, bolt models, materials applied, state >= 10), and the pose
    /// step with and without an animation. Callees are gated fakes; a random
    /// call unwinds.
    #[test]
    fn fe_rider_preview_update_matches() {
        oc1130_install();
        check(
            ee::s03::f_0019f138__generated,
            fe_rider_preview_update,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_f138, 8);
                c.pc = 0x0019_f138;
                c.bus.write32(OC1130_CTRL, OC1130_MAGIC);
                let this = DATA + 0x1000;
                let globals = DATA + 0x2000;
                let vtable = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x854, globals);
                c.bus.write32(globals + 0x10d8, vtable);
                c.bus
                    .write16(vtable + 0x390, (r.next_u32() % 0x40 * 4) as u16);
                c.bus.write32(vtable + 0x394, FAKE);
                let flags = r.next_u32();
                let bit = |n: u32| (flags >> n) & 1;
                let pending = match seed % 3 {
                    0 => u32::MAX,
                    1 => r.next_u32() % 3,
                    _ => 3 + r.next_u32() % 2,
                };
                c.bus.write32(this + 0xcd4, pending);
                // the vtable call is the first fake call
                c.bus.write32(LOG_RESULTS, 3);
                let state = if flags & 7 == 0 {
                    10 + r.next_u32() % 4
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(this, state);
                c.bus.write32(this + 4, r.next_u32());
                c.bus
                    .write32(this + 0xc, if bit(3) == 1 { DATA + 0x4000 } else { 0 });
                c.bus.write32(this + 0xc24, r.next_u32());
                c.bus.write32(this + 0xc28, r.next_u32());
                c.bus.write32(this + 0xc70, r.next_u32());
                c.bus.write32(this + 0xc74, r.next_u32());
                c.bus.write32(this + 0xcb4, bit(4) * (1 + r.next_u32() % 3));
                c.bus.write32(this + 0xcb8, bit(5) * (1 + r.next_u32() % 3));
                c.bus.write32(
                    this + 0xcbc,
                    if bit(6) == 1 || seed % 4 == 3 { 1 } else { 0 },
                );
                c.bus.write32(this + 0xcc0, bit(7) * (1 + r.next_u32() % 3));
                c.bus.write32(this + 0xcc4, bit(8) * (1 + r.next_u32() % 3));
                c.bus.write32(this + 0xcc8, bit(9) * (1 + r.next_u32() % 3));
                oc1130_force(c, 1, bit(10));
                oc1130_force(c, 2, bit(11));
                c.g[A0][0] = sx(this);
            },
        );
    }
}
