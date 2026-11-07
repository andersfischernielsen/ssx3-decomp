//! `src/fe/feridermanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::dirtysock::tags2::arg;
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
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
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
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;

    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;

    pub(crate) use crate::game::fe::fepopup::callee::C0019F878;

    pub(crate) use crate::game::fe::fepopup3::callee::FE_MODULE_RIDER_PREVIEW_CTOR;

    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;

    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;

    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;

    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
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

/// `void FEModule_DestroyRiderPreviews(self, flags)` (0x1a0420): unload the
/// previews, destroy the two rider-preview objects at `+0xce0`/`+0x19c0` and
/// free the module when `flags & 1`.
pub fn fe_module_destroy_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS.call(c, 0x001a_0440)?;
        let null = c.g[S1][0] == 0;
        c.g[S0][0] = sx(low(c, S1).wrapping_add(0x19c0));
        let mut extra = true;
        let mut result = low(c, S2) & 1;
        if !null {
            c.cycles += 2;
            let last = c.g[S1][0] == c.g[S0][0];
            result = low(c, S2) & 1;
            if !last {
                c.cycles += 2;
                c.g[S0][0] = sx(low(c, S0).wrapping_sub(0xce0));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    callee::FE_MODULE_RIDER_PREVIEW_DTOR.call(c, 0x001a_0464)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx(low(c, S0).wrapping_sub(0xce0));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_0458);
                    }
                }
            } else {
                extra = false;
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V0][0] = result as u64;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE_2.addr, 0x001a_0480)?;
            c.lq(16, sp_2(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end rider preview.

/// `int FEModule_RiderPreviewFindTexture(RiderPreview* self, char* name)`
/// (0x19f878): the id of the first preview entry whose name matches, else -1.
pub fn fe_module_rider_preview_find_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S3), 0x18)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[S1][0] = sx(p(r32(c, S3), 0x1c));
            c.g[S0][0] = sx(p(r32(c, S3), 0x20));
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
                c.g[V0][0] = Addr(p(r32(c, S3), 0x18)).lw(c);
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
        frame.restore(c, S0, 0x10);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopup3::tests::setup;
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
}
