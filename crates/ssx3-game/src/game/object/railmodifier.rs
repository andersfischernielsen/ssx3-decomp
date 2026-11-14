//! `src/object/railmodifier.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::bx::bxstringctor::gp_4 as gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_4;
pub(crate) use crate::game::fe::fepopup::sh;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::splashmanager::oc230_gp;
pub(crate) use crate::game::intersect::aifwddiff::oc230_add;
pub(crate) use crate::game::intersect::aifwddiff::oc230_sll;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::lightman::addu as addu_2;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::sound::asyncsys::lw;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
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
        0x0035_cff0,
        roller_modifier_set_angular_momentum,
        "RollerModifier_SetAngularMomentum",
    ),
    (
        0x0035_d288,
        roller_modifier_update_momentum,
        "RollerModifier_UpdateMomentum",
    ),
    (
        0x0035_da00,
        roller_modifier_save_body,
        "RollerModifier_SaveBody",
    ),
    (
        0x0035_da38,
        roller_modifier_load_body,
        "RollerModifier_LoadBody",
    ),
    (0x0035_e1e0, roller_modifier_save, "RollerModifier_Save"),
    (
        0x0035_e770,
        rollermodifier_get_pose,
        "RollerModifier_GetPose",
    ),
    (0x0035_e850, roller_modifier_update, "RollerModifier_Update"),
    (
        0x0035_ed90,
        roller_modifier_translate,
        "RollerModifier_Translate",
    ),
    (0x0035_f0b8, tex_flip_modifier_ctor, "TexFlipModifier_ctor"),
    (
        0x0035_f1a8,
        tex_flip_modifier_ctor_2,
        "TexFlipModifier_ctor_2",
    ),
    (
        0x0035_f378,
        tex_flip_modifier_set_frame,
        "TexFlipModifier_SetFrame",
    ),
    (
        0x0035_f3a8,
        tex_flip_modifier_apply_frame,
        "TexFlipModifier_ApplyFrame",
    ),
    (
        0x0035_f410,
        tex_flip_modifier_update,
        "TexFlipModifier_Update",
    ),
    (
        0x0035_f598,
        texflip_modifier_handle_event,
        "TexFlipModifier_HandleEvent",
    ),
    (0x0035_f688, tex_flip_modifier_save, "TexFlipModifier_Save"),
    (
        0x0035_f6e8,
        uv_scroll_modifier_ctor,
        "UVScrollModifier_ctor",
    ),
    (
        0x0035_f788,
        uv_scroll_modifier_ctor_2,
        "UVScrollModifier_ctor_2",
    ),
    (
        0x0035_f7d0,
        uv_scroll_modifier_update,
        "UVScrollModifier_Update",
    ),
    (
        0x0035_fb30,
        uv_scroll_modifier_set_param,
        "UVScrollModifier_SetParam",
    ),
    (0x0036_0768, game_process_v11, "GameProcess_v11"),
    (0x0036_07a0, typed_process_is_type, "TypedProcess_IsType"),
    (
        0x0036_0800,
        sync_process_id_less_equal,
        "SyncProcess_IdLessEqual",
    ),
    (0x0036_08e8, world_obj_update, "WorldObj_Update"),
    (0x0036_0b00, roller_modifier_dtor, "RollerModifier_dtor"),
    (
        0x0036_0c28,
        boost_modifier_base_dtor,
        "BoostModifier_BaseDtor",
    ),
    (
        0x0036_0c98,
        modifier_list_node_dtor,
        "ModifierListNode_dtor",
    ),
    (
        0x0036_0d40,
        rail_modifier_list_node_dtor,
        "RailModifierListNode_dtor",
    ),
    (0x0036_0da0, mod_obj_v03, "ModObj_v03"),
    (0x0036_0dd8, mod_obj_v51, "ModObj_v51"),
    (0x0036_1068, anim_object_v30, "AnimObject_v30"),
    (0x0036_10e0, mod_obj_get_matrix_at, "ModObj_GetMatrixAt"),
    (0x0036_1158, anim_object_v53, "AnimObject_v53"),
    (0x0036_1170, anim_object_v31, "AnimObject_v31"),
    (0x0036_11d8, intersect_info_dtor, "IntersectInfo_dtor"),
    (0x0036_1340, intersect_info_dtor_3, "IntersectInfo_dtor_3"),
    (0x0036_1400, anim_teeter_dtor, "AnimTeeter_dtor"),
    (0x0036_1460, anim_teeter_v31, "AnimTeeter_v31"),
    (0x0036_1488, boost_modifier_v01, "BoostModifier_v01"),
    (0x0036_15a8, float_rail_v49, "FloatRail_v49"),
    (0x0036_15c0, spring_rail_v49, "SpringRail_v49"),
    (0x0036_1778, floating_v45, "Floating_v45"),
    (0x0036_17b8, floating_v27, "Floating_v27"),
    (0x0036_1800, floating_v29, "Floating_v29"),
    (0x0036_1858, floating_v51, "Floating_v51"),
    (0x0036_18c8, position_modifier_v01, "PositionModifier_v01"),
    (
        0x0036_1918,
        position_modifier_set_transform_bounds,
        "PositionModifier_SetTransformBounds",
    ),
    (0x0036_1988, parent_modifier_v01, "ParentModifier_v01"),
    (0x0036_19e0, magnet_modifier_v01, "MagnetModifier_v01"),
    (
        0x0036_1a30,
        magnet_modifier_set_transform_bounds,
        "MagnetModifier_SetTransformBounds",
    ),
    (0x0036_1a60, magnet_modifier_v18, "MagnetModifier_v18"),
    (
        0x0036_1aa8,
        ava_spline_modifier_v01,
        "AvaSplineModifier_v01",
    ),
    (
        0x0036_1ae0,
        ava_spline_modifier_set_transform_bounds,
        "AvaSplineModifier_SetTransformBounds",
    ),
    (
        0x0036_1b60,
        spline_modifier_set_transform_bounds,
        "SplineModifier_SetTransformBounds",
    ),
    (0x0036_1b90, spline_modifier_v18, "SplineModifier_v18"),
    (
        0x0036_1bf0,
        multi_spline_modifier_set_transform_bounds,
        "MultiSplineModifier_SetTransformBounds",
    ),
    (0x0036_1c78, roller_modifier_v01, "RollerModifier_v01"),
    (
        0x0036_1cb0,
        roller_modifier_is_transform_finished,
        "RollerModifier_IsTransformFinished",
    ),
    (
        0x0036_1d00,
        roller_modifier_set_transform_bounds,
        "RollerModifier_SetTransformBounds",
    ),
    (0x0036_1d78, rail_man_v11, "RailMan_v11"),
    (0x0036_1d90, anim_combo_v31, "AnimCombo_v31"),
    (0x0036_1de0, anim_delta_dtor, "AnimDelta_dtor"),
    (
        0x0036_1e30,
        intc_table_install_vif_vu_handlers,
        "IntcTable_InstallVifVuHandlers",
    ),
    (
        0x0036_1e80,
        intc_table_remove_vif_vu_handlers,
        "IntcTable_RemoveVifVuHandlers",
    ),
    (0x0036_1eb8, intctable_init, "IntcTable_Init"),
    (
        0x0036_1f90,
        intc_table_end_frame_stub,
        "IntcTable_EndFrameStub",
    ),
    (0x0036_1f98, intc_table_add_handler, "IntcTable_AddHandler"),
    (
        0x0036_2008,
        intc_table_remove_handler,
        "IntcTable_RemoveHandler",
    ),
    (
        0x0036_2340,
        intc_table_vif1_handler,
        "IntcTable_Vif1Handler",
    ),
    (0x0036_23a8, intc_table_vu1_handler, "IntcTable_Vu1Handler"),
    (0x0036_27a8, gs_pkt_finish, "GsPkt_Finish"),
    (
        0x0036_4240,
        defer_man_compute_sort_key,
        "DeferMan_ComputeSortKey",
    ),
    (
        0x0036_4360,
        defer_man_emit_default_state,
        "DeferMan_EmitDefaultState",
    ),
    (
        0x0036_45b8,
        defer_man_intern_matrix_record,
        "DeferMan_InternMatrixRecord",
    ),
    (0x0036_4b88, defer_man_set_scissor, "DeferMan_SetScissor"),
    (
        0x0036_6548,
        vram_heap_free_unlocked,
        "VramHeap_FreeUnlocked",
    ),
    (0x0036_6618, vramheap_free_all, "VramHeap_FreeAll"),
    (0x0036_6fe8, tex_man_ctor, "TexMan_ctor"),
    (0x0036_7078, texman_reset_vram, "TexMan_ResetVram"),
    (0x0036_70e0, tex_man_init_table, "TexMan_InitTable"),
    (
        0x0035_fc20,
        uv_scroll_modifier_get_matrix,
        "UVScrollModifier_GetMatrix",
    ),
    (0x0036_4050, defer_man_sort_buckets, "DeferMan_SortBuckets"),
    (
        0x0035_c040,
        rail_modifier_update_bounds,
        "RailModifier_UpdateBounds",
    ),
    (
        0x0036_1940,
        position_modifier_get_matrix,
        "PositionModifier_GetMatrix",
    ),
    (
        0x0036_66f8,
        tex_man_reset_usage_table,
        "TexMan_ResetUsageTable",
    ),
    (0x0035_c4e0, rail_modifier_v03, "RailModifier_v03"),
    (0x0036_1098, mod_obj_get_matrices, "ModObj_GetMatrices"),
    (
        0x0036_5f68,
        vram_heap_nodes_to_scratchpad,
        "VramHeap_NodesToScratchpad",
    ),
    (
        0x0036_5fa8,
        vram_heap_nodes_from_scratchpad,
        "VramHeap_NodesFromScratchpad",
    ),
    (
        0x0035_fd98,
        uv_scroll_modifier_save,
        "UVScrollModifier_Save",
    ),
    (
        0x0035_30d0,
        modifier_block_free_transform,
        "ModifierBlock_FreeTransform",
    ),
    (
        0x0035_3188,
        modifier_block_free_boost,
        "ModifierBlock_FreeBoost",
    ),
    (
        0x0035_31d0,
        modifier_block_free_particles,
        "ModifierBlock_FreeParticles",
    ),
    (0x0036_0840, sync_process_id_equals, "SyncProcess_IdEquals"),
    (
        0x0035_3118,
        modifier_block_free_uv_scroll,
        "ModifierBlock_FreeUVScroll",
    ),
    (
        0x0035_3150,
        modifier_block_free_tex_flip,
        "ModifierBlock_FreeTexFlip",
    ),
    (0x0035_c540, rail_modifier_v02, "RailModifier_v02"),
    (
        0x0035_c0e8,
        rail_modifier_build_rail_nodes,
        "RailModifier_BuildRailNodes",
    ),
    (0x0036_1038, object_v01, "Object_v01"),
    (0x0036_1290, intersect_info_dtor_2, "IntersectInfo_dtor_2"),
    (
        0x0036_12c0,
        intersect_info_reset_matrix,
        "IntersectInfo_ResetMatrix",
    ),
    (0x0036_1d50, particle_v01, "Particle_v01"),
];

/// The matrix record table of `DeferMan_InternMatrixRecord`.
pub(crate) mod matrix {

    /// `man + BASE`: `+FOUND` the last lookup's index (-1: appended),
    /// `+COUNT` the number of records.
    pub const BASE: u32 = 0x6_8000;

    pub const FOUND: u32 = 0x1cc0;

    pub const COUNT: u32 = 0x1cc8;

    /// `man + RECORDS + i * SIZE`: the records.
    pub const RECORDS: u32 = 0x6_9cd0;

    pub const SIZE: u32 = 0x180;

    /// The compared part of a record, as the original's unrolled compare
    /// chains: (offset, floats).
    pub const KEY_GROUPS: [(u32, u32); 4] = [(0xe0, 16), (0x120, 16), (0x160, 4), (0x170, 4)];

    /// How the original copies a new record: (offset, quadwords), the
    /// count re-read for each piece.
    pub const PIECES: [(u32, u32); 9] = [
        (0x00, 4),
        (0x40, 4),
        (0x80, 4),
        (0xc0, 1),
        (0xd0, 1),
        (0xe0, 4),
        (0x120, 4),
        (0x160, 1),
        (0x170, 1),
    ];
}

// ---------------------------------------------------------------- UV scroll

/// `UVScrollModifier` fields.
pub(crate) mod uv {

    /// Mode: 2 / 6 reverse the scroll when a cycle ends; 6 scrolls with a
    /// linear ramp up and down over the cycle.
    pub const MODE: u32 = 0x0;

    /// Time in the current cycle and the cycle length.
    pub const TIME: u32 = 0x4;

    pub const CYCLE: u32 = 0x8;

    /// Delay before the scroll (re)starts.
    pub const DELAY: u32 = 0xc;

    /// A phase accumulator and its step (wrapped by the global at gp-0x29cc).
    pub const PHASE: u32 = 0x10;

    pub const PHASE_STEP: u32 = 0x14;

    /// UV offset and velocity.
    pub const U: u32 = 0x30;

    pub const V: u32 = 0x34;

    pub const DU: u32 = 0x38;

    pub const DV: u32 = 0x3c;

    pub const RUNNING: u32 = 0x40;
}

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::ai::ai::callee::INTC_RESTORE_REGS;
    pub(crate) use crate::game::ai::ai::callee::INTC_SAVE_REGS;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;

    pub(crate) use crate::game::fe::fepopup::callee::C003625C0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626D8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627A8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364CD0;

    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;

    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;

    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;

    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;

    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;

    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;

    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;

    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;

    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;

    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
}

// ---------------------------------------------------------------------------
// UV scroll modifier.

/// `UVScrollModifier* UVScrollModifier_ctor(UVScrollModifier *m, void *desc)`
/// (0x35f6e8): build the modifier from its descriptor.
pub fn uv_scroll_modifier_ctor(c: &mut Cpu) {
    c.cycles += 40;
    let this = Addr(abi::arg(c, 0));
    let desc = Addr(abi::arg(c, 1));
    c.g[V0][0] = 0x1;
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    this.off(0x40).sw(c, c.g[V0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = 0x3f80_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = desc.off(4).lw(c);
    this.sw(c, c.g[V1][0]);
    c.f[0] = desc.off(8).read_fpu(c).0;
    this.off(0x30).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0xc).read_fpu(c).0;
    this.off(0x34).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = desc.off(0x18).read_fpu(c).0;
    this.off(8).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0x1c).read_fpu(c).0;
    this.off(4).sw(c, 0);
    this.off(0xc).write_fpu(c, FpuF32(c.f[1]));
    this.off(0x10).sw(c, 0);
    c.f[1] = Addr(c.g[A2][0] as u32).off(0x10).read_fpu(c).0;
    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
    c.f[0] = desc.off(0x20).read_fpu(c).0;
    let sp = c.g[ssx3_ee_rt::cpu::reg::SP][0] as u32;
    Addr(sp).off(0xc).sw(c, 0);
    c.f[2] = FpuF32(c.f[2]).div(FpuF32(c.f[1])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
    this.off(0x14).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0x24).read_fpu(c).0;
    c.f[0] = desc.off(0x28).read_fpu(c).0;
    c.f[2] = desc.off(0x2c).read_fpu(c).0;
    Addr(sp).write_fpu(c, FpuF32(c.f[1]));
    Addr(sp + 4).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp + 8).write_fpu(c, FpuF32(c.f[2]));
    let quad = Addr(sp).read_vec4(c);
    abi::set_gpr128(c, V1, quad.to_bits());
    this.off(0x20).write_vec4(c, quad);
    c.f[0] = desc.off(0x10).read_fpu(c).0;
    this.off(0x38).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0x14).read_fpu(c).0;
    this.off(0x3c).write_fpu(c, FpuF32(c.f[1]));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

/// `void UVScrollModifier_SetParam(UVScrollModifier *m, int param, float v)`
/// (0x35fb30): store `v` into the field selected by `param`, clamped to the
/// parameter's range.
pub fn uv_scroll_modifier_set_param(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 4;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xc8));
    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xca));
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xcc));
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            // param 0xcc/0xcd: field 0x34/0x30, range [-4, 4]
            c.cycles += 5;
            c.g[AT][0] = 0xffff_ffff_c080_0000;
            c.f[0] = 0xc080_0000;
            let cc = FpuF32(c.f[0]).le(FpuF32(c.f[12]));
            abi::set_fcc(c, cc);
            if cc {
                c.cycles += 6;
                c.g[AT][0] = 0x4080_0000;
                c.f[0] = 0x4080_0000;
                let cc2 = FpuF32(c.f[12]).le(FpuF32(c.f[0]));
                abi::set_fcc(c, cc2);
                c.g[V0][0] = 0xcc;
                if cc2 {
                    c.cycles += 2;
                    if abi::arg(c, 1) != 0xcc {
                        this.off(0x34).write_fpu(c, FpuF32(c.f[12]));
                    } else {
                        c.cycles += 2;
                        this.off(0x30).write_fpu(c, FpuF32(c.f[12]));
                        abi::ret(c);
                        return;
                    }
                }
            }
        } else {
            // param 0xca/0xcb: field 0xc/0x8, range [0, 60]
            c.cycles += 4;
            c.f[0] = 0;
            let cc = FpuF32(c.f[0]).le(FpuF32(c.f[12]));
            abi::set_fcc(c, cc);
            if cc {
                c.cycles += 6;
                c.g[AT][0] = 0x4270_0000;
                c.f[0] = 0x4270_0000;
                let cc2 = FpuF32(c.f[12]).le(FpuF32(c.f[0]));
                abi::set_fcc(c, cc2);
                c.g[V0][0] = 0xca;
                if cc2 {
                    c.cycles += 2;
                    if abi::arg(c, 1) != 0xca {
                        c.cycles += 2;
                        this.off(0xc).write_fpu(c, FpuF32(c.f[12]));
                        abi::ret(c);
                        return;
                    }
                    c.cycles += 2;
                    this.off(8).write_fpu(c, FpuF32(c.f[12]));
                    abi::ret(c);
                    return;
                }
            }
        }
    } else {
        // param 0xc8/0xc9: field 0x3c/0x38, range [-1, 1]
        c.cycles += 5;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[0] = 0xbf80_0000;
        let cc = FpuF32(c.f[0]).le(FpuF32(c.f[12]));
        abi::set_fcc(c, cc);
        if cc {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            let cc2 = FpuF32(c.f[12]).le(FpuF32(c.f[0]));
            abi::set_fcc(c, cc2);
            c.g[V0][0] = 0xc9;
            if cc2 {
                c.cycles += 2;
                if abi::arg(c, 1) != 0xc9 {
                    c.cycles += 2;
                    this.off(0x3c).write_fpu(c, FpuF32(c.f[12]));
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
                this.off(0x38).write_fpu(c, FpuF32(c.f[12]));
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AvaSplineModifier_v01(AvaSplineModifier *this, int flags)`
/// (0x361aa8): scalar deleting destructor: set the vtable and, if `flags & 1`,
/// free `this`.
pub fn ava_spline_modifier_v01(c: &mut Cpu) {
    c.cycles += 7;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x0049_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let vtable = 0x0049_0000u32.wrapping_add(0x13f8);
    c.g[V0][0] = sx(vtable);
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    let flags = c.g[A1][0];
    c.g[A1][0] = flags & 1;
    let free = c.g[A1][0] == 0;
    c.sw(this, vtable as u64);
    if !free {
        c.cycles += 2;
        call_ptr(c, 0x0031_7e50, 0x0036_1acc).unwrap_or(());
        if c.pc != 0x0036_1acc {
            return;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// AnimObject / AnimDelta destructors.

/// `void AnimDelta_dtor(AnimDelta *this, int flags)` (0x361de0): set the
/// vtable then run `AnimObject_dtor(this, flags)`.
pub fn anim_delta_dtor(c: &mut Cpu) {
    c.cycles += 10;
    let this = abi::arg(c, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = 0x0049_0000;
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[V1][0] = this as u64;
    let vtable = 0x0049_0000u32.wrapping_sub(0x1590);
    c.g[V0][0] = sx(vtable);
    c.g[RA][0] = sx(0x0036_1dfc);
    c.sw(this.wrapping_add(0x3c), vtable as u64);
    call_ptr(c, 0x0034_1cf0, 0x0036_1dfc).unwrap_or(());
    if c.pc != 0x0036_1dfc {
        return;
    }
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `void DeferMan_InternMatrixRecord(DeferMan *man, const Record *rec)`
/// (0x3645b8): find the matrix record whose key part (+0xe0..+0x180, 40
/// floats compared with `c.eq.s`) equals `rec`'s and leave its index in
/// `man+0x69cc0`; if there is none, append a copy of all 0x180 bytes and
/// leave the new index there.
pub fn defer_man_intern_matrix_record(c: &mut Cpu) {
    c.cycles += 10;
    let man = abi::arg(c, 0);
    let base = man.wrapping_add(matrix::BASE);
    c.g[T3][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A1][0];
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = sx(base);
    c.g[A1][0] = 0;
    let count = Addr(base.wrapping_add(matrix::COUNT)).lw(c);
    Addr(base.wrapping_add(matrix::FOUND)).sw(c, u64::MAX);
    if (count as i64) > 0 {
        c.cycles += 4;
        c.g[A3][0] = sx(base);
        c.g[A2][0] = matrix::SIZE as u64;
        let first = abi::mult(c, 0, matrix::SIZE);
        c.g[V0][0] = sx(first);
        matrix_search(c);
    }
    matrix_tail(c);
}

/// Compare `n` floats of a stored record (`stored`) with the new one
/// (`new`), stopping at the first difference, as the original's unrolled
/// chain: it leaves the last pair in f0/f1 (stored in f0 for the first
/// compare, in f1 after), the condition bit, and costs 10 cycles + 5 per
/// further compare (+1 for the last). True when all are equal.
pub(crate) fn floats_equal(c: &mut Cpu, stored: u32, new: u32, n: u32) -> bool {
    let mut k = 0;
    loop {
        let a = Addr(stored.wrapping_add(4 * k)).read_u32(c);
        let b = Addr(new.wrapping_add(4 * k)).read_u32(c);
        let same = FpuF32(a).eq_s(FpuF32(b));
        let last = k + 1 == n;
        if !same || last {
            if k == 0 {
                (c.f[0], c.f[1]) = (a, b);
            } else {
                (c.f[1], c.f[0]) = (a, b);
            }
            abi::set_fcc(c, same);
            c.cycles += 10 + 5 * k as u64 + last as u64;
            return same;
        }
        k += 1;
    }
}

/// The search loop of `DeferMan_InternMatrixRecord`. Registers: t3 the
/// man, t1 the new record, a1 the index, v0 = index * 0x180, a2 = 0x180,
/// a3 the table base.
pub(crate) fn matrix_search(c: &mut Cpu) {
    loop {
        let man = r32(c, T3);
        let new = r32(c, T1);
        let index = c.g[A1][0];
        let stride = r32(c, A2);
        let rec = r32(c, V0).wrapping_add(man).wrapping_add(matrix::RECORDS);
        let mut group_at = rec;
        let mut same = true;
        for (g, &(off, n)) in matrix::KEY_GROUPS.iter().enumerate() {
            group_at = rec.wrapping_add(off);
            same = floats_equal(c, group_at, new.wrapping_add(off), n);
            if !same {
                c.cycles += if g < 3 { 3 } else { 2 };
                break;
            }
            c.cycles += 2;
        }
        c.g[V1][0] = sx(group_at);
        if same {
            c.cycles += 2;
            abi::mult(c, index as u32, stride);
            c.g[A0][0] = 1;
            let found_at = r32(c, A3).wrapping_add(matrix::FOUND);
            Addr(found_at).sw(c, index);
            return;
        }
        c.cycles += 4;
        let count = Addr(r32(c, A3).wrapping_add(matrix::COUNT)).lw(c);
        let next = sx((index as u32).wrapping_add(1));
        let more = (next as i64) < (count as i64);
        let next_off = abi::mult(c, next as u32, stride);
        c.g[A0][0] = 0;
        c.g[A1][0] = next;
        c.g[V0][0] = sx(next_off);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0036_45f0);
        }
    }
}

/// The end of `DeferMan_InternMatrixRecord`: append the record when the
/// search left -1.
pub(crate) fn matrix_tail(c: &mut Cpu) {
    c.cycles += 6;
    let man = r32(c, T3);
    let base = man.wrapping_add(matrix::BASE);
    c.g[V0][0] = matrix::BASE as u64;
    c.g[T2][0] = sx(base);
    let found = Addr(base.wrapping_add(matrix::FOUND)).lw(c);
    c.g[V1][0] = found;
    c.g[A3][0] = matrix::SIZE as u64;
    if (found as i64) < 0 {
        c.cycles += 110;
        let new = r32(c, T1);
        matrix_append(c, man, base, new);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// Copy the new record to `records[count]` piece by piece (the count is
/// re-read for each piece, as the original does), then `found = count`,
/// `count + 1`. Leaves the registers of the original's copy sequence: the
/// last four-quadword piece in VF1-VF4, the last `lq`s in a0/a1.
pub(crate) fn matrix_append(c: &mut Cpu, man: u32, base: u32, new: u32) {
    let count_at = base.wrapping_add(matrix::COUNT);
    let mut vf = [0u128; 4];
    let mut a0_q = 0u128;
    let mut a1_hi = 0u64;
    let mut rec_off = 0u32;
    for &(off, n) in matrix::PIECES.iter() {
        let count = Addr(count_at).lw(c) as u32;
        rec_off = abi::mult(c, count, matrix::SIZE);
        let dst = rec_off
            .wrapping_add(man)
            .wrapping_add(matrix::RECORDS + off);
        let src = new.wrapping_add(off);
        let mut q = [0u128; 4];
        for (i, v) in q.iter_mut().enumerate().take(n as usize) {
            *v = c.bus.read128(src.wrapping_add(0x10 * i as u32));
        }
        for (i, v) in q.iter().enumerate().take(n as usize) {
            c.bus.write128(dst.wrapping_add(0x10 * i as u32), *v);
        }
        match off {
            0x120 => vf = q,
            0x160 => a1_hi = (q[0] >> 64) as u64,
            0x170 => a0_q = q[0],
            _ => {}
        }
    }
    for (i, q) in vf.iter().enumerate() {
        let lanes = [
            *q as u32,
            (*q >> 32) as u32,
            (*q >> 64) as u32,
            (*q >> 96) as u32,
        ];
        c.bus.plat.set_vf(i + 1, lanes);
    }
    let count = Addr(count_at).lw(c);
    let next = sx((count as u32).wrapping_add(1));
    Addr(base.wrapping_add(matrix::FOUND)).sw(c, count);
    Addr(count_at).sw(c, next);
    c.g[V0][0] = next;
    c.g[V1][0] = count;
    abi::set_gpr128(c, A0, a0_q);
    c.g[A1] = [sx(rec_off), a1_hi];
    c.g[A2][0] = (matrix::RECORDS + 0xe0) as u64;
    c.g[T0][0] = sx(new.wrapping_add(0xe0));
}

/// The scroll velocity (`+0x38`/`+0x3c`, a UV offset per 60 Hz frame) as the
/// per-tick step at the game's rate: `60 / rate` (exactly 1 at 60, where
/// nothing changes). The time step itself already follows the rate (it is
/// `1 / (*(gp+0x2a74)+0x10)`).
#[inline]
fn uv_velocity_scale(c: &mut Cpu) -> FpuF32 {
    let r = crate::tick::rate(c);
    if r == 60 {
        FpuF32::ONE
    } else {
        FpuF32::from_f32(60.0f32 / r as f32)
    }
}

/// `void UVScrollModifier_Update(UVScrollModifier* m)` (0x35f7d0): advance
/// a texture scroll by one frame (1 / the frame rate at `*(gp+0x2a74)+0x10`).
/// While stopped it waits out the delay; while running it advances the
/// phase, ends the cycle after `CYCLE` (reversing the velocity in modes 2
/// and 6), and moves the UV offset by the velocity (mode 6: by the area
/// under a triangular speed ramp), wrapping it into [-1, 1].
pub fn uv_scroll_modifier_update(c: &mut Cpu) {
    const ZERO: FpuF32 = FpuF32::ZERO;
    const ONE: FpuF32 = FpuF32::ONE;
    const MINUS_ONE: FpuF32 = FpuF32(0xbf80_0000);
    const HALF: FpuF32 = FpuF32(0x3f00_0000);
    let m = Addr(abi::arg(c, 0));
    let gp = c.g[GP][0] as u32;
    c.cycles += 6;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x40);
    c.g[SP][0] = sx(sp);
    let stack = Addr(sp);
    // the registers a caller can see afterwards
    let mut f0: FpuF32;
    let mut f1 = FpuF32(c.f[1]);
    let mut v0 = c.g[V0][0];
    let mut v1 = c.g[V1][0];
    let cycle = m.off(uv::CYCLE).read_fpu(c);
    let has_cycle = ZERO.lt(cycle);
    abi::set_fcc(c, has_cycle);
    'out: {
        if !has_cycle {
            c.cycles += 4;
            f0 = m.off(uv::DELAY).read_fpu(c);
            let has_delay = ZERO.lt(f0);
            abi::set_fcc(c, has_delay);
            if !has_delay {
                break 'out;
            }
        }
        c.cycles += 11;
        let clock = Addr(gp.wrapping_add(0x2a74)).lw(c);
        c.g[AT][0] = ONE.0 as u64;
        f1 = FpuF32::from_i32(Addr(clock as u32).off(0x10).read_u32(c) as i32);
        let old_time = m.off(uv::TIME).read_fpu(c);
        v0 = m.off(uv::RUNNING).lw(c);
        f0 = ONE.div(f1);
        let time = old_time + f0;
        m.off(uv::TIME).write_fpu(c, time);
        if v0 == 0 {
            // stopped: start once the delay has passed
            c.cycles += 4;
            f0 = m.off(uv::DELAY).read_fpu(c);
            let waited = f0.le(time);
            abi::set_fcc(c, waited);
            if waited {
                c.cycles += 4;
                let start = ZERO.lt(cycle);
                abi::set_fcc(c, start);
                m.off(uv::TIME).write_fpu(c, ZERO);
                if start {
                    c.cycles += 2;
                    v0 = 1;
                    m.off(uv::RUNNING).sw(c, 1);
                }
            }
            break 'out;
        }
        // the phase accumulator
        c.cycles += 5;
        f1 = m.off(uv::PHASE_STEP).read_fpu(c);
        let still = f1.eq_s(ZERO);
        abi::set_fcc(c, still);
        if !still {
            c.cycles += 7;
            let step = f1;
            f0 = m.off(uv::PHASE).read_fpu(c) + step;
            let wrap = Addr(gp.wrapping_sub(0x29cc)).read_fpu(c);
            let over = wrap.le(step);
            abi::set_fcc(c, over);
            m.off(uv::PHASE).write_fpu(c, f0);
            let wrapped = if over {
                c.cycles += 2;
                Some(step - wrap)
            } else {
                c.cycles += 4;
                let under = step.le(wrap);
                abi::set_fcc(c, under);
                if under {
                    c.cycles += 1;
                    Some(step + wrap)
                } else {
                    None
                }
            };
            if let Some(s) = wrapped {
                c.cycles += 2;
                m.off(uv::PHASE_STEP).write_fpu(c, s);
            }
        }
        f1 = m.off(uv::TIME).read_fpu(c);
        // end of a cycle
        c.cycles += 5;
        f0 = m.off(uv::CYCLE).read_fpu(c);
        let ended = f0.le(f1);
        abi::set_fcc(c, ended);
        if ended {
            c.cycles += 7;
            m.off(uv::TIME).sw(c, 0);
            f1 = m.off(uv::DELAY).read_fpu(c);
            f0 = m.off(uv::TIME).read_fpu(c);
            let pause = f0.lt(f1);
            abi::set_fcc(c, pause);
            if pause {
                m.off(uv::RUNNING).sw(c, 0);
            }
            c.cycles += 4;
            let mode = m.off(uv::MODE).lw(c);
            v1 = mode;
            let reverse = mode == 2 || {
                c.cycles += 2;
                mode == 6
            };
            if reverse {
                // velocity = -velocity, through a stack doubleword
                c.cycles += 11;
                f0 = -m.off(uv::DU).read_fpu(c);
                f1 = -m.off(uv::DV).read_fpu(c);
                stack.write_fpu(c, f0);
                stack.off(4).write_fpu(c, f1);
                let pair = stack.read_u64_unaligned(c);
                m.off(uv::DU).write_u64_unaligned(c, pair);
            }
        }
        v0 = m.off(uv::RUNNING).lw(c);
        c.cycles += 2;
        let running = v0 != 0;
        v0 = 6;
        if !running {
            break 'out;
        }
        // move the UV offset
        c.cycles += 3;
        let mode = m.off(uv::MODE).lw(c);
        v1 = mode;
        if mode != 6 {
            c.cycles += 7;
            // The velocity is authored per 60 Hz frame; at another rate a
            // tick moves `60/rate` of it, so the texture scrolls at the same
            // speed in wall time (1.0 and bit-identical at 60).
            let scale = uv_velocity_scale(c);
            let u = m.off(uv::U).read_fpu(c) + m.off(uv::DU).read_fpu(c) * scale;
            let v = m.off(uv::V).read_fpu(c) + m.off(uv::DV).read_fpu(c) * scale;
            m.off(uv::U).write_fpu(c, u);
            m.off(uv::V).write_fpu(c, v);
        } else {
            // ramp: the speed rises linearly over the first half of the
            // cycle (counted from the end when the velocity is negative)
            c.cycles += 6;
            let du = m.off(uv::DU).read_fpu(c);
            let negative = du.lt(ZERO);
            abi::set_fcc(c, negative);
            let cycle = m.off(uv::CYCLE).read_fpu(c);
            let (t, spill_up, spill_down) = if negative {
                c.cycles += 8;
                (m.off(uv::TIME).read_fpu(c), 0x10, 0x20)
            } else {
                c.cycles += 9;
                (cycle - m.off(uv::TIME).read_fpu(c), 0x30, 0x10)
            };
            let first_half = t.lt(cycle * HALF);
            abi::set_fcc(c, first_half);
            let dv = m.off(uv::DV).read_fpu(c);
            let rate = ONE.div(cycle);
            let (du_t, dv_t) = if first_half {
                // offset += velocity * t / cycle
                c.cycles += 17;
                let (du_t, dv_t) = (du * t, dv * t);
                stack.off(spill_up).write_fpu(c, du_t);
                stack.off(spill_up + 4).write_fpu(c, dv_t);
                (du_t, dv_t)
            } else {
                // offset += velocity * (cycle - t) / cycle
                c.cycles += (if negative { 11 } else { 10 }) + 6;
                let rest = cycle - t;
                let (du_t, dv_t) = (du * rest, dv * rest);
                stack.off(spill_down + 4).write_fpu(c, dv_t);
                stack.off(spill_down).write_fpu(c, du_t);
                (du_t, dv_t)
            };
            // The ramp's area is a per-60-Hz-frame step too.
            let scale = uv_velocity_scale(c);
            let (step_u, step_v) = (du_t * rate * scale, dv_t * rate * scale);
            let v = m.off(uv::V).read_fpu(c) + step_v;
            let u = m.off(uv::U).read_fpu(c) + step_u;
            stack.off(4).write_fpu(c, step_v);
            stack.write_fpu(c, step_u);
            c.cycles += 3;
            m.off(uv::V).write_fpu(c, v);
            m.off(uv::U).write_fpu(c, u);
        }
        // wrap the offset into [-1, 1]
        c.cycles += 6;
        f1 = m.off(uv::U).read_fpu(c);
        let over = ONE.lt(f1);
        abi::set_fcc(c, over);
        let wrapped = if over {
            c.cycles += 2;
            Some(f1 - ONE)
        } else {
            c.cycles += 6;
            let under = f1.lt(MINUS_ONE);
            abi::set_fcc(c, under);
            if under {
                c.cycles += 1;
                Some(f1 + ONE)
            } else {
                None
            }
        };
        if let Some(w) = wrapped {
            c.cycles += 2;
            m.off(uv::U).write_fpu(c, w);
        }
        f1 = m.off(uv::V).read_fpu(c);
        c.cycles += 5;
        let over = ONE.lt(f1);
        abi::set_fcc(c, over);
        if over {
            c.cycles += 3;
            f0 = f1 - ONE;
            m.off(uv::V).write_fpu(c, f0);
        } else {
            c.cycles += 5;
            f0 = MINUS_ONE;
            let under = f1.lt(MINUS_ONE);
            abi::set_fcc(c, under);
            if under {
                c.cycles += 3;
                f0 = f1 + ONE;
                m.off(uv::V).write_fpu(c, f0);
            }
        }
    }
    c.cycles += 2;
    c.f[0] = f0.0;
    c.f[1] = f1.0;
    c.g[V0][0] = v0;
    c.g[V1][0] = v1;
    c.g[SP][0] = sx(sp.wrapping_add(0x40));
    abi::ret(c);
}

/// `void ModObj_v03(ModObj* o)` (0x360da0): a tail call to the node at
/// `*(o+0xc)`'s slot 0x190 method.
pub fn mod_obj_v03(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let this = abi::arg64(c, 0);
    c.g[V0][0] = Addr(this as u32).off(0xc).lw(c);
    let node = c.g[V0][0];
    c.g[V1][0] = Addr(node as u32).off(0x190).lh(c);
    c.g[A1][0] = Addr(node as u32).off(0x194).lw(c);
    let target = c.g[A1][0] as u32;
    c.g[RA][0] = sx(0x0036_0dbc);
    c.g[A0][0] = abi::addu(this, c.g[V1][0]);
    if call_ptr(c, target, 0x0036_0dbc).is_err() {
        return;
    }
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// World objects.

/// `void WorldObj_Update(WorldObj* obj)` (0x3608e8): call the object's
/// virtual update at `*(*(obj + 0xc) + 0x7c)` with `this + (short)
/// *((obj + 0xc) + 0x78)` as the argument (the virtual body sits at that
/// offset inside the embedded rail object, so the adjustment is added to the
/// `this` pointer). No value returned.
pub fn world_obj_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(0xc).lw(c);
        let virt = c.g[V1][0] as u32;
        c.g[A1][0] = Addr(virt).off(0x78).lh(c);
        let target = Addr(virt).off(0x7c).lw(c);
        c.g[V0][0] = target;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target as u32, 0x0036_0904)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Direct callees of `RollerModifier_Update`.
mod roller_callee {
    use super::{ee, Callee};
    pub const TRANSLATE: Callee = Callee {
        addr: 0x0035_ed90,
        f: ee::s11::RollerModifier_Translate,
    };
    pub const GET_POSE: Callee = Callee {
        addr: 0x0035_e770,
        f: ee::s11::RollerModifier_GetPose,
    };
    pub const INTEGRATE: Callee = Callee {
        addr: 0x0035_d340,
        f: ee::s11::RollerModifier_Integrate,
    };
    pub const UPDATE_INERTIA: Callee = Callee {
        addr: 0x0035_e248,
        f: ee::s11::RollerModifier_UpdateInertia,
    };
    pub const UPDATE_MOMENTUM: Callee = Callee {
        addr: 0x0035_d288,
        f: ee::s11::RollerModifier_UpdateMomentum,
    };
    pub const COLLIDE_WORLD: Callee = Callee {
        addr: 0x0035_edc8,
        f: ee::s11::RollerModifier_CollideWorld,
    };
    /// `SphereTree_SetTransform` (through the function table).
    pub const SPHERE_TREE_SET_TRANSFORM: u32 = 0x0032_c648;
}

/// `RollerModifier` fields used by the update.
mod roller {
    /// Running average of the kinetic energy (below `gp-0x29e0` = 10000 the
    /// roller stops).
    pub const ENERGY_AVG: u32 = 0x4;
    /// State timer (s): 10 stopped, 11..16 waiting, 16 reset.
    pub const TIMER: u32 = 0x20;
    /// Velocity damping (decays per tick, `tick::PER_TICK`).
    pub const DAMPING: u32 = 0x28;
    /// Linear and angular momentum, mass, linear/angular velocity.
    pub const MOMENTUM: u32 = 0x50;
    pub const ANGULAR: u32 = 0x60;
    pub const MASS: u32 = 0x70;
    pub const VELOCITY: u32 = 0xb0;
    pub const SPIN: u32 = 0xc0;
    /// The collision sphere tree and its transform.
    pub const TREE: u32 = 0xe0;
    pub const TRANSFORM: u32 = 0x240;
}

/// The VU0 sequence that turns the pose at `pose` (quaternion at `+0`,
/// position at `+0x10`) into the 4x4 transform at `out` (rotation rows
/// `+0..+0x20`, position `+0x30`).
fn roller_pose_matrix(c: &mut Cpu, pose: u32, out: u32) {
    c.lqc2(4, pose.wrapping_add(0x10));
    c.lqc2(3, pose);
    c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
    c.vu0().add(0b1110, 5, 4, Src::Vec(4));
    c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
    c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
    c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
    c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
    c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
    c.vu0().opmula(5, 4);
    c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
    c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
    c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
    c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
    c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
    c.vu0().suba(0b0010, 1, Src::Bc(6, X));
    c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
    c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
    c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
    c.vu0().adda(0b0010, 0, Src::Bc(8, X));
    c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
    c.vu0().adda(0b0100, 0, Src::Bc(8, X));
    c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
    c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
    c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
    c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
    c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
    c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
    c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
    c.sqc2(3, out.wrapping_add(0x30));
    c.sqc2(10, out);
    c.sqc2(11, out.wrapping_add(0x10));
    c.sqc2(12, out.wrapping_add(0x20));
}

/// `w = dot(a, b)` of two quadwords (VU0, the `.w` lane of vf4), moved to
/// `f{fr}` through v0.
fn roller_dot(c: &mut Cpu, a: u32, b: u32, fr: usize) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    c.mtc1(V0, fr);
}

/// `void RollerModifier_Update(RollerModifier* self)` (0x35e850): one tick
/// of a rolling object (a boulder, a log). The state timer at `+0x20`
/// advances by the step; from 11 s it waits, at 16 s it is put back 10 units
/// lower and its collision tree moved there. Otherwise the kinetic energy
/// (`v.p + w.L`, times the mass) feeds a running average (`0.95 old + 0.05
/// new` per tick); below 10000, or from 10 s on, the roller stops (timer
/// 11, momenta and velocities cleared). Else the damping decays, the body
/// integrates one step with a velocity-dependent drag, its inertia and
/// momentum are updated, the collision tree follows the pose, the world
/// collision runs, and the transform is rebuilt.
///
/// Tick rate: the average is a per-tick blend, so above 60 its kept share
/// is `k' = 0.95^(60/rate)` and the new share `1 - k'` (the same time
/// constant). The timer and the integration step load `.lit4` 1/60 (follow
/// the rate) and the 0.8333 damping decay is in `tick::PER_TICK`. Nothing
/// changes at 60.
pub fn roller_modifier_update(c: &mut Cpu) {
    use roller::*;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(0, gp(c, -0x29ec));
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 20, 0x90);
        let this = r32(c, S2);
        c.g[AT][0] = 0x4130_0000; // 11.0
        c.mtc1(AT, 2);
        c.lwc1(1, this.wrapping_add(TIMER));
        c.add_s(1, 1, 0);
        c.c_le_s(2, 1);
        let waiting = c.fcc();
        c.swc1(1, this.wrapping_add(TIMER));
        if waiting {
            c.cycles += 6;
            c.g[AT][0] = 0x4180_0000; // 16.0
            c.mtc1(AT, 0);
            c.c_le_s(0, 1);
            let reset = c.fcc();
            frame.restore(c, S0, 0x80);
            if reset {
                // Put it back 10 below and move its collision tree there.
                c.cycles += 51;
                let s = sp(c);
                c.g[AT][0] = 0xffff_ffff_c120_0000; // -10.0
                c.mtc1(AT, 0);
                c.g[A1][0] = sx(s);
                Addr(s).sw(c, 0);
                c.swc1(0, s.wrapping_add(8));
                Addr(s).off(4).sw(c, 0);
                Addr(s).off(0xc).sw(c, 0);
                roller_callee::TRANSLATE.call(c, 0x0035_e8c8)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
                roller_callee::GET_POSE.call(c, 0x0035_e8d4)?;
                c.g[A1][0] = sx(r32(c, S2).wrapping_add(TRANSFORM));
                roller_pose_matrix(c, sp(c).wrapping_add(0x10), r32(c, A1));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(TREE));
                call_ptr(c, roller_callee::SPHERE_TREE_SET_TRANSFORM, 0x0035_e96c)?;
                frame.restore(c, S0, 0x80);
            }
        } else {
            roller_update_body(c, frame)?;
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The energy average and then the stop or the simulation step (0x35e974 to
/// 0x35ed70; `s0` is restored at the end).
fn roller_update_body(c: &mut Cpu, frame: Frame) -> Flow {
    use roller::*;
    let this = r32(c, S2);
    c.cycles += 24;
    c.lwc1(3, this.wrapping_add(MASS));
    roller_dot(
        c,
        this.wrapping_add(VELOCITY),
        this.wrapping_add(MOMENTUM),
        1,
    );
    c.mtc1(ZERO, 2);
    roller_dot(c, this.wrapping_add(SPIN), this.wrapping_add(ANGULAR), 0);
    c.c_eq_s(3, 2);
    let massless = c.fcc();
    c.add_s(5, 1, 0);
    if massless {
        c.cycles += 2;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
        c.mtc1(AT, 4);
    } else {
        c.cycles += 5;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.div_s(4, 0, 3);
    }

    // avg = avg * 0.95 + energy * 0.05 (per tick).
    c.cycles += 12;
    let hz = crate::tick::rate(c);
    let kept = if hz == 60 {
        None
    } else {
        let entry = gp(c, -0x29e4);
        let k = f32::from_bits(c.bus.read32(entry));
        let k = k.powf(60.0 / hz as f32);
        Some((FpuF32::from_f32(k), FpuF32::from_f32(1.0 - k)))
    };
    c.lwc1(1, gp(c, -0x29e8));
    if let Some((_, w)) = kept {
        c.f[1] = w.0;
    }
    c.lwc1(0, this.wrapping_add(ENERGY_AVG));
    c.mul_s(1, 5, 1);
    c.lwc1(2, gp(c, -0x29e4));
    if let Some((k, _)) = kept {
        c.f[2] = k.0;
    }
    c.lwc1(3, gp(c, -0x29e0));
    c.mul_s(0, 0, 2);
    c.div_s(1, 1, 4);
    c.add_s(0, 0, 1);
    c.c_lt_s(0, 3);
    let slow = c.fcc();
    c.swc1(0, this.wrapping_add(ENERGY_AVG));
    let mut stop = slow;
    if !slow {
        c.cycles += 7;
        c.lwc1(3, this.wrapping_add(TIMER));
        c.g[AT][0] = 0x4120_0000; // 10.0
        c.mtc1(AT, 5);
        c.c_le_s(5, 3);
        stop = c.fcc();
        if !stop {
            c.lwc1(0, this.wrapping_add(DAMPING));
        }
    }
    if stop {
        roller_stop(c, this);
    } else {
        roller_step(c, this)?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x80);
    Ok(())
}

/// The roller stops: timer 11, momenta cleared, velocities = momenta / mass
/// (0x35ea3c).
fn roller_stop(c: &mut Cpu, this: u32) {
    use roller::*;
    c.cycles += 59;
    let s = sp(c);
    for k in 0..4 {
        Addr(s).off(4 * k).sw(c, 0);
    }
    c.g[AT][0] = 0x4130_0000; // 11.0
    c.mtc1(AT, 0);
    c.lq(V0, s);
    c.swc1(0, this.wrapping_add(TIMER));
    c.sq(V0, this.wrapping_add(MOMENTUM));
    c.lwc1(0, this.wrapping_add(MASS));
    c.mfc1(V0, 0);
    c.lqc2(4, s);
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x20));
    for k in 0..4 {
        Addr(s).off(4 * k).sw(c, 0);
    }
    c.lq(V0, s);
    c.lwc1(3, this.wrapping_add(0x80));
    c.sq(V0, this.wrapping_add(ANGULAR));
    c.lwc1(9, this.wrapping_add(0x8c));
    c.lwc1(2, this.wrapping_add(0x60));
    c.lwc1(4, this.wrapping_add(0x64));
    c.lwc1(0, this.wrapping_add(0x84));
    c.mul_s(3, 3, 2);
    c.lwc1(7, this.wrapping_add(0x90));
    c.mul_s(9, 9, 4);
    c.lwc1(1, this.wrapping_add(0x88));
    c.mul_s(0, 0, 2);
    c.lwc1(8, this.wrapping_add(0x94));
    c.mul_s(7, 7, 4);
    c.lwc1(6, this.wrapping_add(0x68));
    c.mul_s(1, 1, 2);
    c.lwc1(5, this.wrapping_add(0x98));
    c.mul_s(8, 8, 4);
    c.lwc1(2, this.wrapping_add(0x9c));
    c.add_s(3, 3, 9);
    c.lwc1(4, this.wrapping_add(0xa0));
    c.mul_s(5, 5, 6);
    c.add_s(0, 0, 7);
    c.lq(V0, s.wrapping_add(0x20));
    c.mul_s(2, 2, 6);
    c.add_s(1, 1, 8);
    c.sq(V0, this.wrapping_add(VELOCITY));
    c.mul_s(4, 4, 6);
    c.add_s(3, 3, 5);
    c.add_s(0, 0, 2);
    c.add_s(1, 1, 4);
    c.swc1(3, s.wrapping_add(0x20));
    c.swc1(0, s.wrapping_add(0x24));
    c.swc1(1, s.wrapping_add(0x28));
    c.lwc1(0, this.wrapping_add(0x6c));
    c.swc1(0, s.wrapping_add(0x2c));
    c.lq(V0, s.wrapping_add(0x20));
    c.sq(V0, this.wrapping_add(SPIN));
    c.sq(V0, s.wrapping_add(0x10));
}

/// One simulation step (0x35eb28 to 0x35ed6c; `f0` = the damping, `f5` =
/// 10.0).
fn roller_step(c: &mut Cpu, this: u32) -> Flow {
    use roller::*;
    // The damping decays; the drag scale is min(damping * 10, 2) (0 when
    // negative).
    c.cycles += 13;
    c.lwc1(1, gp(c, -0x29dc));
    c.mtc1(ZERO, 4);
    c.mul_s(0, 0, 1);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 20);
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(AT, 2);
    c.mov_s(6, 4);
    c.mul_s(1, 0, 5);
    c.c_le_s(4, 1);
    let positive = c.fcc();
    c.swc1(0, this.wrapping_add(DAMPING));
    if positive {
        c.cycles += 1;
        c.min_s(6, 1, 2);
    }

    // Forces: gravity (-980 / mass on z) and drag on both momenta, then
    // integrate one step.
    c.cycles += 131;
    let s = sp(c);
    c.lwc1(2, this.wrapping_add(MASS));
    c.add_s(0, 3, 3);
    c.g[AT][0] = 0xffff_ffff_c475_0000; // -980.0
    c.mtc1(AT, 1);
    c.g[S0][0] = sx(this.wrapping_add(0x30));
    c.swc1(4, s.wrapping_add(0x10));
    c.g[A1][0] = sx(s);
    c.div_s(1, 1, 2);
    c.swc1(4, s.wrapping_add(0x14));
    c.add_s(0, 0, 20);
    c.swc1(4, s.wrapping_add(0x1c));
    c.lwc1(12, gp(c, -0x29d8));
    c.g[A2][0] = sx(s.wrapping_add(0x10));
    c.g[A0][0] = c.g[S0][0];
    c.g[S1][0] = sx(this.wrapping_add(TRANSFORM));
    c.mul_s(0, 6, 0);
    c.swc1(1, s.wrapping_add(0x18));
    c.mfc1(V1, 0);
    c.lqc2(4, this.wrapping_add(MOMENTUM));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lq(V0, s.wrapping_add(0x30));
    c.sq(V0, s.wrapping_add(0x20));
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lq(V0, s.wrapping_add(0x30));
    c.swc1(4, s.wrapping_add(0x2c));
    c.sq(V0, s);
    c.swc1(4, s.wrapping_add(0x20));
    c.swc1(4, s.wrapping_add(0x24));
    c.swc1(4, s.wrapping_add(0x28));
    c.mfc1(V1, 0);
    c.lqc2(4, this.wrapping_add(ANGULAR));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s.wrapping_add(0x30));
    c.lqc2(3, s.wrapping_add(0x20));
    c.lqc2(4, s.wrapping_add(0x30));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s.wrapping_add(0x10));
    roller_callee::INTEGRATE.call(c, 0x0035_ec20)?;
    c.g[A0][0] = c.g[S2][0];
    roller_callee::UPDATE_INERTIA.call(c, 0x0035_ec28)?;
    c.g[A0][0] = c.g[S0][0];
    roller_callee::UPDATE_MOMENTUM.call(c, 0x0035_ec30)?;

    // The collision tree follows the pose; collide; rebuild the transform.
    c.g[S0][0] = sx(sp(c).wrapping_add(0x20));
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    roller_callee::GET_POSE.call(c, 0x0035_ec40)?;
    roller_pose_matrix(c, sp(c).wrapping_add(0x20), r32(c, S1));
    c.mov_s(12, 20);
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(TREE));
    call_ptr(c, roller_callee::SPHERE_TREE_SET_TRANSFORM, 0x0035_ecd4)?;
    c.g[A0][0] = c.g[S2][0];
    roller_callee::COLLIDE_WORLD.call(c, 0x0035_ecdc)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    roller_callee::GET_POSE.call(c, 0x0035_ece8)?;
    roller_pose_matrix(c, sp(c).wrapping_add(0x20), r32(c, S1));
    Ok(())
}

/// `void DeferMan_SetScissor(DeferMan* self /*a0*/, const ScissorKey* k
/// /*a1*/)` (0x364b88): find `k`'s four halfwords among `self`'s `count`
/// 8-byte keys (base `+0x6aed4`, count `+0x6aed0`); store the index in
/// `found` (`+0x69cc4`), and if absent append `k` at `count` and set
/// `found = count`, `count++`.
pub fn defer_man_set_scissor(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    let key = abi::arg(c, 1);
    c.cycles += 8;
    c.g[V0][0] = 0x60000;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] |= 0x8000;
    c.g[A3][0] = 0;
    c.g[A2][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
    let none = (c.g[T0][0] as i64) <= 0;
    Addr(c.g[A2][0] as u32).off(0x1cc4).sw(c, c.g[V1][0]);
    if !none {
        c.cycles += 4;
        c.g[V0][0] = 0x60000;
        c.g[T1][0] = c.g[A2][0];
        c.g[V0][0] |= 0xaeda;
        c.g[A2][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        let mut found = false;
        loop {
            c.cycles += 4;
            let entry = c.g[A2][0] as u32;
            c.g[V1][0] = Addr(entry.wrapping_sub(0x6)).lhu(c);
            c.g[V0][0] = Addr(key).lhu(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(entry.wrapping_sub(0x4)).lhu(c);
                c.g[V0][0] = Addr(key).off(2).lhu(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(entry.wrapping_sub(0x2)).lhu(c);
                    c.g[V0][0] = Addr(key).off(4).lhu(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(entry).lhu(c);
                        c.g[V0][0] = Addr(key).off(6).lhu(c);
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 2;
                            Addr(c.g[T1][0] as u32).off(0x1cc4).sw(c, c.g[A3][0]);
                            found = true;
                        }
                    }
                }
            }
            if found {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_4bb8);
            }
        }
    }
    // Not found: append the key at `count` and record its index.
    c.cycles += 6;
    c.g[V0][0] = 0x60000;
    c.g[V0][0] |= 0x8000;
    c.g[A2][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x1cc4).lw(c);
    if (c.g[V1][0] as i64) < 0 {
        c.cycles += 24;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x2ed4).sh(c, c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).off(2).lhu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(0x2ed6).sh(c, c.g[A0][0]);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).off(4).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x2ed8).sh(c, c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).off(6).lhu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(0x2eda).sh(c, c.g[A0][0]);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        Addr(c.g[A2][0] as u32).off(0x1cc4).sw(c, c.g[V0][0]);
        Addr(c.g[A2][0] as u32).off(0x2ed0).sw(c, c.g[V1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `DeferMan_ComputeSortKey`'s class table (`.rodata` at 0x492010).
pub(crate) const DEFER_CLASS_TABLE: u32 = 0x0049_2010;

// ---------------------------------------------------------------------------
// SyncProcess_IdLessEqual.

/// `bool SyncProcess_IdLessEqual(SyncProcess* self, SyncProcessObj* obj)`
/// (0x360800): `obj->GetId()` (slot at `obj->vt + 0x40`) `>= self->id`
/// (`self + 0x14`), i.e. the negation of `id < self->id`.
pub fn sync_process_id_less_equal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0036_0824)?;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DeferMan_ComputeSortKey.

/// `u32 DeferMan_ComputeSortKey(DeferEntry* e, u32 key)` (0x364240): the
/// packet's sort key, the bitwise complement of a packed
/// `{31 - layer(e+8), class(e+4), category(e+0), key}`. `class` is
/// `e[1] & 3` (0 → 0x3ff, 1 → 0x3fe, 2 → `(e[2] & 0x1ffffc00) >> 10`, 3 → 0);
/// `category` comes from the nine-entry table at 0x492010 indexed by
/// `(e[0] & 0x3c0) >> 6`; the key contributes `(short)key` when it is
/// non-negative and `(key >> 16) & 0xf` shifted to bits 6..9 when the high
/// half is non-negative.
pub fn defer_man_compute_sort_key(c: &mut Cpu) {
    c.cycles += 10;
    let entry = abi::arg(c, 0);
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = 0x1f;
    c.g[T0][0] = lw(c, entry.wrapping_add(0x8));
    c.g[A0][0] = 0x1;
    c.g[V1][0] = lw(c, entry.wrapping_add(0x4));
    c.g[V0][0] = c.g[T0][0] & 0x3e0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
    c.g[V1][0] = c.g[V1][0] & 0x3;
    let sel_one = c.g[V1][0] == c.g[A0][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    if sel_one {
        // b = 0x3fe
        c.cycles += 2;
        c.g[T0][0] = 0x3fe;
    } else {
        c.cycles += 3;
        if c.g[V1][0] == 0 {
            // b = 0x3ff
            c.cycles += 2;
            c.g[T0][0] = 0;
            c.cycles += 2;
            c.g[T0][0] = 0x3ff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x1fff_0000;
            if c.g[V1][0] == 2 {
                // b = (e[2] & 0x1ffffc00) >> 10
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] | 0xfc00;
                c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32) >> 10);
            } else {
                // b = 0
                c.cycles += 2;
                c.g[T0][0] = 0;
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = lw(c, entry);
    c.cycles += 5;
    c.g[V0][0] = c.g[V0][0] & 0x3c0;
    c.g[A0][0] = sx((c.g[V0][0] as u32) >> 6);
    c.g[V1][0] = (c.g[A0][0] < 0x9u64) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x49_0000;
    if in_table {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        // (0x490000 + 0x2010 is the class table at 0x492010.)
        c.g[V0][0] = sx(DEFER_CLASS_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let slot = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(slot).lw(c);
        let target = c.g[A0][0] as u32;
        let (cost, class) = match target {
            0x0036_42e0 => (2, 6),
            0x0036_42e8 => (2, 5),
            0x0036_42f0 => (2, 4),
            0x0036_42f8 => (2, 3),
            0x0036_4300 => (2, 2),
            0x0036_4308 => (2, 1),
            0x0036_4310 => (1, 0),
            _ => {
                c.jump_out(target, 0x0036_4240, 0x0036_4360);
                return;
            }
        };
        c.cycles += cost;
        c.g[A0][0] = class;
    } else {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }

    // The key: low half when non-negative, then the high nibble at bits 6..9.
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
    c.g[A3][0] = 0;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
    c.g[V1][0] = ((c.g[V0][0] as i64) < 0) as u64;
    let hi_negative = (c.g[A1][0] as i64) < 0;
    if c.g[V1][0] == 0 {
        c.g[A3][0] = c.g[V0][0];
    }
    if !hi_negative {
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0] & 0xf;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
    }
    c.cycles += 9;
    c.g[V0][0] = sx((c.g[T0][0] as u32) << 16);
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 26);
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 13);
    c.g[A3][0] = c.g[A3][0] & 0x3ff;
    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
    c.g[V0][0] = !(c.g[V0][0] | c.g[V1][0]);
    abi::ret(c);
}

/// `RailModifier *SplineModifier_v18(RailModifier *self)` (0x00361b90):
/// rebuild the rail when `+0x58` is set (`vt[3](this)`), then return
/// `this + 0x60`.
pub fn spline_modifier_v18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x58));
        let rebuild = c.g[V0][0] != 0;
        c.g[V0][0] = sx(off(c.g[S0][0] as u32, 0x60));
        if rebuild {
            c.cycles += 6;
            c.g[V0][0] = c.ldw(c.g[S0][0] as u32);
            c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
            c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = addu_2(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0036_1bc0)?;
            c.g[V0][0] = sx(off(c.g[S0][0] as u32, 0x60));
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `RollerModifier_dtor` (0x360b00): vtable at `+0`.
pub fn roller_modifier_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0049_13f8, 0x0036_0b24);
}

/// `BoostModifier_BaseDtor` (0x360c28): vtable at `+0x4`.
pub fn boost_modifier_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0049_13b8, 0x0036_0c4c);
}

/// `IntersectInfo_dtor` (0x3611d8): vtable at `+0x84`.
pub fn intersect_info_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x84, 0x0049_0af0, 0x0036_11fc);
}

/// `IntersectInfo_dtor_3` (0x361340): vtable at `+0x84`.
pub fn intersect_info_dtor_3(c: &mut Cpu) {
    deleting_dtor(c, 0x84, 0x0049_0af0, 0x0036_1364);
}

/// `BoostModifier_v01` (0x361488): vtable at `+0x4`.
pub fn boost_modifier_v01(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0049_13b8, 0x0036_14ac);
}

/// `TexFlipModifier_SetFrame(TexFlipModifier* m, int frame)` (0x35f378):
/// clamp `frame` to `< count` (else `count-1`), store it at `+0x14` and apply
/// it to the flip textures.
pub fn tex_flip_modifier_set_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr(c.g[A2][0] as u32).off(0x20).lw(c);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A1][0];
        }
        let this = c.g[A2][0] as u32;
        let v = c.g[V0][0];
        Addr(this).off(0x14).sw(c, v);
        call_ptr(c, 0x0035_f3a8, 0x0035_f39c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The "reset the vtable, then delete when asked" destructor shape.

/// `*(void**)((char*)self + DISP) = VTABLE; if (flags & 1) operator delete(self);`
/// — the body of the seventeen object/table destructors below. The `0x490000`
/// page is loaded into `v0` first (the literal the compiler shared across the
/// object files), then the final vtable address.
pub(crate) fn reset_vtable_and_delete(c: &mut Cpu, ret: u32, vtable: u32, disp: u32) -> Flow {
    c.cycles += 7;
    c.g[V0][0] = 0x0049_0000;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 0x1;
    let keep = c.g[A1][0] == 0;
    Addr(r32(c, A0).wrapping_add(disp)).sw(c, c.g[V0][0]);
    if !keep {
        c.cycles += 2;
        call_ptr(c, DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `PositionModifier` destroying destructor (0x3618c8): re-publish
/// `D_004913F8` (the shared modifier vtable) and free when asked.
pub fn position_modifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0036_18ec, 0x0049_13f8, 0)
    })
}

/// `ParentModifier` destroying destructor (0x361988).
pub fn parent_modifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0036_19ac, 0x0049_13f8, 0)
    })
}

/// `MagnetModifier` destroying destructor (0x3619e0).
pub fn magnet_modifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0036_1a04, 0x0049_13f8, 0)
    })
}

/// `RollerModifier` destroying destructor (0x361c78).
pub fn roller_modifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0036_1c9c, 0x0049_13f8, 0)
    })
}

// ---------------------------------------------------------------------------
// IntcTable_Vu1Handler.

/// `int IntcTable_Vu1Handler(void)` (0x3623a8): the VU1 interrupt handler —
/// save the full register file to a 0x100-byte stack buffer, restore it, and
/// re-enable interrupts. Always returns 1 (the "handled" code).
pub fn intc_table_vu1_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x110);
        frame.save_ra(c, 0x100);
        c.g[A0][0] = c.g[SP][0];
        callee::INTC_SAVE_REGS.call(c, 0x0036_23b8)?;
        c.g[A0][0] = c.g[SP][0];
        callee::INTC_RESTORE_REGS.call(c, 0x0036_23c0)?;
        c.ei();
        frame.restore_ra(c, 0x100);
        c.g[V0][0] = 0x1;
        frame.close(c);
        Ok(())
    })
}

/// `int Floating_v45(Floating* self)` (0x361778): the floating transform's id,
/// either from the bound modifier block or the cached word.
pub fn floating_v45(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[V0][0] = this;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(this as u32).off(0x1c).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            callee::GET_TRANSFORM_BOUNDS.call(c, 0x0036_17a0)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(this as u32).off(0x78).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RollerModifier_SaveBody(RollerModifier* mod, sWScriptStream* stream)`
/// (0x35da00).
pub fn roller_modifier_save_body(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0035_da28, 8, 0x50, None))
}

/// `void RollerModifier_LoadBody(RollerModifier* mod, sWScriptStream* stream)`
/// (0x35da38).
pub fn roller_modifier_load_body(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_arg_vtable(c, 0x0035_da60, 0x10, 0x50, None)
    })
}

/// `void IntcTable_RemoveVifVuHandlers(IntcTable* table)` (0x361e80): remove
/// the VIF1 (5) and VU1 (7) interrupt handlers.
pub fn intc_table_remove_vif_vu_handlers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 5;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let table = c.g[A0][0];
        c.g[S0][0] = table;
        callee::INTC_REMOVE_HANDLER.call(c, 0x0036_1e98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 7;
        callee::INTC_REMOVE_HANDLER.call(c, 0x0036_1ea4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SphereTree_Translate` (0x32c630).
pub(crate) const SPHERE_TREE_TRANSLATE: u32 = 0x0032_c630;

/// `void RollerModifier_Translate(sRmBody* self, const sRmVec4* b)`
/// (0x35ed90): add `*b` to the body position at `+0x30` (VU0, keeping the MAC
/// flags) and translate the body's bounding volume at `+0xe0` as well.
pub fn roller_modifier_translate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A1][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe0));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.lqc2(4, c.g[V1][0] as u32);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x30));
        call_ptr(c, SPHERE_TREE_TRANSLATE, 0x0035_edbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* UVScrollModifier_ctor_2(UVScrollModifier* m, MbStream* s)` (0x35f788):
/// run the stream's slot-2 constructor on the object with size 0x50, return
/// the object.
pub fn uv_scroll_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x50;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = c.lw(r32(c, A3));
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
        let ctor = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, ctor, 0x0035_f7b8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Vec4* MagnetModifier_v18(MagnetModifier* m)` (0x361a60): update the
/// modifier when it has a target (`+0x50`) and return its magnet vector
/// (`+0x60`).
pub fn magnet_modifier_v18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x50));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x60));
        if !idle {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x18));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x1c));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0036_1a90)?;
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x60));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Floating_v27(self)` (0x3617b8): call the rail modifier's flush slot
/// (0x198/0x19c) when bit 0 of `+0x12` is set, then return `+0x74`.
pub fn floating_v27(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0x12).lhu(c);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x198).lh(c);
            c.g[V1][0] = Addr(vt).off(0x19c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0036_17ec);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0036_17ec)?;
        }
        c.g[V0][0] = Addr(this).off(0x74).lw(c);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Interrupts and textures.

/// `void IntcTable_InstallVifVuHandlers(IntcTable* self)` (0x361e30):
/// install the two 0x362340/0x3623a8 handlers for the VIF0 and VU0 interrupts.
pub fn intc_table_install_vif_vu_handlers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x360000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = sx(0x0036_2340);
        let _this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[A3][0] = 0;
        callee::INTC_TABLE_ADD_HANDLER.call(c, 0x0036_1e54)?;
        c.g[A2][0] = 0x360000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(0x0036_23a8);
        c.g[A1][0] = 7;
        c.g[A3][0] = 0;
        callee::INTC_TABLE_ADD_HANDLER.call(c, 0x0036_1e6c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierListNode_dtor(ModifierListNode* self, int flags)`
/// (0x360c98): install the vtable, unlink the node and maybe delete it.
pub fn modifier_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_1340; // D_00491340
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::MODIFIER_LIST_UNLINK.call(c, 0x0036_0cc0)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0036_0cd4)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailModifierListNode_dtor(RailModifierListNode* self, int flags)`
/// (0x360d40): install the vtable, unlink the node and maybe delete it.
pub fn rail_modifier_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_1200; // D_00491200
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::RAIL_MODIFIER_LIST_UNLINK.call(c, 0x0036_0d68)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0036_0d7c)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* IntcTable_RemoveHandler(IntcTable* self, int handler)`
/// (0x362008): disable the interrupt, clear its table slot and return the
/// old handler.
pub fn intc_table_remove_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let handler = r32(c, A1);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::DISABLE_INTC.call(c, 0x0036_2028)?;
        c.g[V1][0] = sx(handler << 2);
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(this.wrapping_add(r32(c, V1)));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        callee::REMOVE_INTC_HANDLER.call(c, 0x0036_2044)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics (s12).

/// `void AnimTeeter_dtor(AnimTeeter* self, int flags)` (0x361400): tear down
/// the anim object and free the block when `flags & 1`.
pub fn anim_teeter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_08f8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Addr(r32(c, S1)).off(0x50).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0036_1430);
        c.g[A1][0] = 0;
        callee::ANIM_OBJECT_BASE_DTOR.call(c, 0x0036_1430)?;
        c.g[S0][0] &= 0x1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0036_1444);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0036_1444)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Elem* Floating_v29(Floating* self, int index)` (0x361800): wake the
/// element through its virtual if the float flag is set, and return
/// `&self->elems[index]`.
pub fn floating_v29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x12).lhu(c);
        c.g[V0][0] &= 0x1;
        let idle = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !idle {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x198).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x19c).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0036_1838);
            c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0036_1838)?;
        }
        c.cycles += 8;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x74).lw(c);
        c.g[V0][0] = sx(r32(c, S1) << 6);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1 fN, a`: load the 32-bit pattern into FPU register `n`.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    c.f[n] = v.0;
    v
}

/// `li at, bits; mtc1 at, fN` (as gcc materialises a float literal).
#[inline(always)]
pub(crate) fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    c.f[n] = bits;
    FpuF32(bits)
}

/// The FPU condition bit after a `c.*.s` compare; returns the condition.
#[inline(always)]
pub(crate) fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `x >= 0.0f` as the original tests it: `li at, 0; mtc1 at, f0; c.le.s f0, f1`
/// then a `bc1t`.
#[inline(always)]
pub(crate) fn non_negative(c: &mut Cpu, x: FpuF32) -> bool {
    c.f[0] = 0;
    fcc(c, FpuF32::ZERO.le(x))
}

// ---------------------------------------------------------------------------
// Processes, rail modifiers and anim objects.

/// `int GameProcess_v11(GameProcess* this)` (0x360768): a process vtable slot
/// that always returns -1 (no game process).
pub fn game_process_v11(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    abi::ret(c);
}

/// `int TypedProcess_IsType(TypedProcess* this, int type)` (0x3607a0):
/// whether the process's type tag (`this+0x10`, a signed short) equals `type`.
pub fn typed_process_is_type(c: &mut Cpu) {
    c.cycles += 4;
    let tag = Addr(abi::arg(c, 0)).off(0x10).lh(c);
    c.g[V0][0] = tag ^ c.g[A1][0];
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `void ModObj_v51(ModObj* this)` (0x360dd8): clear the low flag bit (bit 0)
/// of the 16-bit word at `+0x12`.
pub fn mod_obj_v51(c: &mut Cpu) {
    c.cycles += 4;
    let flags = Addr(abi::arg(c, 0)).off(0x12);
    let v = flags.lhu(c) & 0xfffe;
    c.g[V0][0] = v;
    flags.sh(c, v);
    abi::ret(c);
}

/// `float AnimObject_v30(AnimObject* this)` (0x361068): the anim object's
/// input at `+0x0` scaled by 30 (the literal is materialised through `at`).
pub fn anim_object_v30(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let input = ldf(c, 1, this);
    let scale = li_f(c, 0, 0x41f0_0000); // 30.0f
    c.f[0] = (input * scale).0;
    abi::ret(c);
}

/// `float AnimObject_v53(AnimObject* this)` (0x361158): the "finished" word
/// at `+0x8` in v0; f0 = the time at `+0x10` when that word is zero, else 0.
pub fn anim_object_v53(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    let finished = Addr(this.wrapping_add(8)).lw(c);
    c.g[V0][0] = finished;
    c.f[0] = 0;
    if finished == 0 {
        c.f[0] = Addr(this.wrapping_add(0x10)).read_fpu(c).0;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int AnimObject_v31(AnimObject* this)` (0x361170): 1 when the time at
/// `+0x10` is >= 0, else -1.
pub fn anim_object_v31(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let x = ldf(c, 1, this.wrapping_add(0x10));
    c.g[V0][0] = u64::MAX;
    if non_negative(c, x) {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int AnimTeeter_v31(AnimTeeter* this)` (0x361460): 1 when the time at
/// `+0x4` is >= 0, else -1.
pub fn anim_teeter_v31(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let x = ldf(c, 1, this.wrapping_add(4));
    c.g[V0][0] = u64::MAX;
    if non_negative(c, x) {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int FloatRail_v49(FloatRail* this, int index)` (0x3615a8): the rail
/// entry `(*(this+0x70))[index]`'s field at `+0x44` (stride 0x50).
pub fn float_rail_v49(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let index = c.g[A1][0] as u32;
    let entries = Addr(this.wrapping_add(0x70)).lw(c);
    c.g[V1][0] = entries;
    c.g[V0][0] = 0x50;
    let entry_off = abi::mult(c, index, 0x50);
    c.g[A1][0] = sx(entry_off);
    let entry = entry_off.wrapping_add(entries as u32);
    c.g[A1][0] = sx(entry);
    c.g[V0][0] = Addr(entry.wrapping_add(0x44)).lw(c);
    abi::ret(c);
}

/// `int SpringRail_v49(SpringRail* this, int index)` (0x3615c0): the rail
/// entry's field at `+0x54` (stride 0x60).
pub fn spring_rail_v49(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let index = c.g[A1][0] as u32;
    let entries = Addr(this.wrapping_add(0x70)).lw(c);
    c.g[V1][0] = entries;
    c.g[V0][0] = 0x60;
    let entry_off = abi::mult(c, index, 0x60);
    c.g[A1][0] = sx(entry_off);
    let entry = entry_off.wrapping_add(entries as u32);
    c.g[A1][0] = sx(entry);
    c.g[V0][0] = Addr(entry.wrapping_add(0x54)).lw(c);
    abi::ret(c);
}

/// One 128-bit transform copy: `sq` the two quadwords at `src` to `dst` and
/// `dst+0x10` (through GPRs 2 and 3, as the original does, so v0/v1 end as
/// the copied halves).
pub(crate) fn copy_transform_bounds(c: &mut Cpu, dst: u32, src: u32) {
    c.lq(2, src);
    c.sq(2, dst.wrapping_add(0x10));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, dst.wrapping_add(0x20));
}

/// `void PositionModifier_SetTransformBounds(PositionModifier* this, void* xf)`
/// (0x361918): copy the transform into `this+0x10`/`+0x20`.
pub fn position_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    copy_transform_bounds(c, this, src);
    abi::ret(c);
}

/// `void MagnetModifier_SetTransformBounds(...)` (0x361a30): copy into
/// `this+0x20`/`+0x30`.
pub fn magnet_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.lq(2, src);
    c.sq(2, this.wrapping_add(0x20));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, this.wrapping_add(0x30));
    abi::ret(c);
}

/// `void AvaSplineModifier_SetTransformBounds(...)` (0x361ae0): copy into
/// `this+0x10`/`+0x20`.
pub fn ava_spline_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    copy_transform_bounds(c, this, src);
    abi::ret(c);
}

/// `void SplineModifier_SetTransformBounds(...)` (0x361b60): copy into
/// `this+0x10`/`+0x20`.
pub fn spline_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    copy_transform_bounds(c, this, src);
    abi::ret(c);
}

/// `void MultiSplineModifier_SetTransformBounds(MultiSplineModifier* this, void* xf)`
/// (0x361bf0): copy the transform into the sub-object at `*(this+0x3c)`
/// (`+0x30`/`+0x40`).
pub fn multi_spline_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    let sub = Addr(this.wrapping_add(0x3c)).lw(c);
    c.g[A2][0] = sub;
    let sub = sub as u32;
    c.lq(2, src);
    c.sq(2, sub.wrapping_add(0x30));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, sub.wrapping_add(0x40));
    abi::ret(c);
}

/// `int RollerModifier_IsTransformFinished(RollerModifier* this)`
/// (0x361cb0): 1 when the distance at `+0x20` is >= 17, else 0.
pub fn roller_modifier_is_transform_finished(c: &mut Cpu) {
    c.cycles += 7;
    let this = abi::arg(c, 0);
    let x = ldf(c, 1, this.wrapping_add(0x20));
    let threshold = li_f(c, 0, 0x4188_0000); // 17.0f
    c.g[V0][0] = 1;
    if !fcc(c, threshold.le(x)) {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RollerModifier_SetTransformBounds(...)` (0x361d00): copy into
/// `this+0x220`/`+0x230`.
pub fn roller_modifier_set_transform_bounds(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.lq(2, src);
    c.sq(2, this.wrapping_add(0x220));
    c.lq(3, src.wrapping_add(0x10));
    c.sq(3, this.wrapping_add(0x230));
    abi::ret(c);
}

/// `int RailMan_v11(RailMan* this)` (0x361d78): a rail-man vtable slot that
/// always returns -1.
pub fn rail_man_v11(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    abi::ret(c);
}

/// `int AnimCombo_v31(AnimCombo* this)` (0x361d90): 1 when the time chosen by
/// the flag byte at `+0x14` (`+0x28` when zero, `+0x0` otherwise) is >= 0,
/// else -1.
pub fn anim_combo_v31(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    let flag = Addr(this.wrapping_add(0x14)).lb(c);
    c.g[V0][0] = flag;
    if flag == 0 {
        let x = ldf(c, 1, this.wrapping_add(0x28));
        c.cycles += 5;
        c.g[V0][0] = u64::MAX;
        if non_negative(c, x) {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
    } else {
        let x = ldf(c, 1, this);
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        if non_negative(c, x) {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Objects, audio and script.

/// `void* ModObj_GetMatrixAt(ModObj* self, int index)` (0x3610E0): the matrix
/// element array at `+0x44`, or the transform modifier's matrix when bit 0 of
/// the flags at `+0x26` is set.
pub fn mod_obj_get_matrix_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lhu(ea(c.g[S0][0], 0x26));
        c.g[V0][0] &= 1;
        let has_modifier = c.g[V0][0] != 0;
        c.g[A1][0] = sx(ea(c.g[S0][0], 0x14));
        if has_modifier {
            c.cycles += 5;
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x20));
            c.g[A0][0] = c.lh(ea(c.g[V0][0], 0x198));
            c.g[V1][0] = c.lw(ea(c.g[V0][0], 0x19c));
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx(ea(c.g[A1][0], c.g[A0][0] as u32));
            call_ptr(c, method, 0x0036_111c)?;
        }
        c.cycles += 8;
        c.g[V1][0] = c.lw(ea(c.g[S0][0], 0x44));
        c.g[V0][0] = sx((c.g[S1][0] as u32) << 6);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx(ea(c.g[V1][0], c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VramHeap_FreeTextureBlocks(VramHeap* self, int i)` (0x366e30): free
/// both texture blocks a VRAM slot owns.
pub fn vram_heap_free_texture_blocks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_2(c, p_2(c.g[S0][0], 0x23e8));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S1][0] = lw_2(c, c.g[A1][0] as u32);
        c.g[A1][0] = lw_2(c, p_2(c.g[S1][0], 0x30));
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 3;
            call_ptr(c, 0x0036_63d8, 0x0036_6e68)?;
        }
        c.g[A1][0] = lw_2(c, p_2(c.g[S1][0], 0x34));
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0036_63d8, 0x0036_6e7c)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Texture flip modifier.

/// `void TexFlipModifier_Save(TexFlipModifier* self, Stream* stream)`
/// (0x35f688): write the modifier's 0x18-byte record and its texture handle.
pub fn tex_flip_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x18;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = lw_2(c, c.g[S0][0] as u32);
        c.g[A0][0] = Addr(p_2(c.g[V1][0], 8)).lh(c);
        c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0xc));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f6bc)?;
        c.g[V1][0] = lw_2(c, c.g[S0][0] as u32);
        c.g[A1][0] = lw_2(c, p_2(c.g[S1][0], 0x18));
        c.g[A0][0] = Addr(p_2(c.g[V1][0], 0x28)).lh(c);
        c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x2c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f6d4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Interrupts, VRAM and graphics.

/// `int IntcTable_Vif1Handler(void)` (0x362340): save the interrupted
/// registers, bump the VIF1 completion count and acknowledge the VIF1 IRQ.
pub fn intc_table_vif1_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sd(p(c.g[SP][0] as u32, 0x100), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0036_2120, 0x0036_2350)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[V0][0] = Addr(p(c.g[V0][0] as u32, 0x3c00)).lw(c);
        c.g[V1][0] = 0x0050_0000;
        c.g[V0][0] = c.g[V0][0] & 0x400;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x59d8));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x1000_0000;
            c.g[V1][0] = 0x8;
            c.g[V0][0] = c.g[V0][0] | 0x3c10;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x80)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sw(p(c.g[A1][0] as u32, 0x80), c.g[V0][0]);
        call_ptr(c, 0x0036_2230, 0x0036_238c)?;
        c.ei();
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x100));
        c.g[V0][0] = 1;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = ra;
        Ok(())
    })
}

/// `void TexMan_ResetVram(TexMan* self)` (0x367078): free both VRAM arenas and
/// reset the two page tables and the owner word.
pub fn texman_reset_vram(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x4350));
        call_ptr(c, 0x0036_6618, 0x0036_7090)?;
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x1f60));
        call_ptr(c, 0x0036_6618, 0x0036_7098)?;
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = 0x1;
        c.g[V0][0] = sx(p(c.g[S0][0] as u32, 0x1f50));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0036_70a8);
                }
                continue;
            }
            c.cycles += 6;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            c.sw(p(c.g[S0][0] as u32, 0x1f54), c.g[V0][0]);
            frame.restore(c, S0, 0x10);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = ra;
            return Ok(());
        }
    })
}

/// `void IntcTable_EndFrameStub(void)` (0x361f90): interrupt table end-of-frame
/// hook, not installed.
pub fn intc_table_end_frame_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `RollerModifier_Save(mod, obj)` 0x35e1e0: serialize the modifier body and
/// forward `obj`'s two save hooks (the packed 0x28-byte record and the current
/// value).
pub fn roller_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg64(c, 0);
        let obj64 = abi::arg64(c, 1);
        let obj = obj64 as u32;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = obj64;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(self_ as u32, 0x30));
        call_ptr(c, 0x0035_da00, 0x0035_e200)?;
        c.g[V0][0] = Addr(obj).lw(c);
        c.g[A1][0] = sx(p(self_ as u32, 4));
        c.g[A2][0] = 0x28;
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(p(vt, 8)).lh(c);
        c.g[V1][0] = lw_at(c, vt, 0xc);
        let func = r32(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0035_e21c)?;
        c.g[V0][0] = Addr(obj).lw(c);
        c.g[A1][0] = lw_at(c, self_ as u32, 0xd0);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(p(vt, 0x28)).lh(c);
        c.g[V1][0] = lw_at(c, vt, 0x2c);
        let func = r32(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0035_e234)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay, audio, INI, mission scan, lobby and widget hooks.

/// `TexFlipModifier_ApplyFrame(mod)` 0x35f3a8: for each entry, copy the
/// current flip table's 16-bit value into the modifier's output record.
pub fn tex_flip_modifier_apply_frame(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[A2][0] = sx(this);
    c.g[V0][0] = lw_at(c, this, 0x1c);
    c.g[A1][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[A3][0] = sx(p(this, 0x48));
        loop {
            c.cycles += 8;
            c.g[V1][0] = lw_at(c, this, 0x18);
            c.g[A0][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x94)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(p(r32(c, V0), 4)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V1), 0x10)).lw(c);
            if c.g[V1][0] == 0 {
                c.g[V0][0] = lw_at(c, this, 0x1c);
            } else {
                c.cycles += 6;
                c.g[V0][0] = lw_at(c, this, 0x14);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(p(r32(c, V0), 4)).lhu(c);
                Addr(r32(c, A3)).sh(c, c.g[V1][0]);
                c.g[V0][0] = lw_at(c, this, 0x1c);
            }
            c.cycles += 4;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_f3c0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `TexMan_InitTable(self)` (0x3670e0): zero the table header and link the
/// 499 free-list entries.
pub fn tex_man_init_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x1f40;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 8));
        Addr(p(c.g[S0][0] as u32, 4)).sw(c, 0);
        call_ptr(c, 0x003e_6448, 0x0036_7108)?;
        c.g[V0][0] = sx(p(c.g[S0][0] as u32, 0x1778));
        c.g[A1][0] = 0;
        Addr(p(c.g[S0][0] as u32, 0x1f48)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x4;
        loop {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x1f48)).lw(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 499i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7118);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Modifiers and misc leaf-ish helpers.

/// `TexFlipModifier_ctor_2(self, obj)` (0x35f1a8): read the flip mode and
/// frame count through `obj`'s vtable, store the count and finish the base
/// modifier construction.
pub fn tex_flip_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x18;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x10)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f1dc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x18)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f1f0)?;
        Addr(p(c.g[S1][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_f218, 0x0035_f1fc)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Floating_v51(self)` (0x361858): copy the transformed value through the
/// object's two vtable entries and clear the "floating" flag bit.
pub fn floating_v51(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(p(c.g[S2][0] as u32, 0xc)).lw(c);
        c.g[S1][0] = Addr(p(c.g[S0][0] as u32, 0x108)).lh(c);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0xc0)).lh(c);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xc4)).lw(c);
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0036_188c)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x10c)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(p(c.g[S2][0] as u32, 0x74)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0036_18a0)?;
        c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 0x12)).lhu(c);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] & 0xfffe;
        frame.restore(c, S1, 0x20);
        Addr(p(c.g[S2][0] as u32, 0x12)).sh(c, c.g[V0][0]);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IntcTable_AddHandler(IntcTable* self, int cause, void* handler,
/// void* arg)` (0x361f98): register an interrupt handler under the table
/// lock.
pub fn intc_table_add_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0042_4880, 0x0036_1fc8)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0042_3a80, 0x0036_1fd8)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(at(c, S1, 0)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0042_48e8, 0x0036_1fec)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IntcTable_Init(table)` (0x361eb8): fill the 16 interrupt slots and
/// their enable masks with -1, then clear the slots and install the VIF/VU
/// handlers.
pub fn intctable_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xf;
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_1ed8);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 0xf;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7c));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_1f00);
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::INTC_TABLE_CLEAR_SLOTS.addr, 0x0036_1f24)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(
            c,
            callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS.addr,
            0x0036_1f2c,
        )?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TexMan* TexMan_ctor(TexMan* self)` (0x366fe8): reset the table, the free
/// list and the two VRAM heaps.
pub fn tex_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(17, at_2(sp0, 0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at_2(sp0, 0x20));
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S1][0];
        c.sw(at_2(this as u32, 0x1f58), 0);
        callee::TEX_MAN_INIT_TABLE.call(c, 0x0036_7004)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx(at_2(this as u32, 0x1f50));
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            c.sw(slot, c.g[A0][0]);
            c.g[V1][0] = sx(at_2(c.g[V1][0] as u32, 0xffff_ffff));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(at_2(slot, 0xffff_fffc));
            if !more {
                c.cycles += 18;
                c.g[S0][0] = sx(at_2(this as u32, 8));
                c.g[V0][0] = u64::MAX;
                c.sw(at_2(this as u32, 0x1f54), c.g[V0][0]);
                c.g[A0][0] = sx(at_2(this as u32, 0x1f60));
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x400000;
                callee::VRAM_HEAP_INIT.call(c, 0x0036_704c)?;
                c.g[A0][0] = sx(at_2(this as u32, 0x4350));
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x1000;
                callee::VRAM_HEAP_INIT.call(c, 0x0036_7060)?;
                c.lq(16, at_2(sp0, 0x20));
                c.lq(17, at_2(sp0, 0x10));
                c.g[RA][0] = c.ld(sp0);
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at_2(sp0, 0x30));
                c.pc = ra;
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7010);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Roller modifier.

/// `void RollerModifier_UpdateMomentum(RollerModifier* m)` (0x35d288): one
/// Verlet-style momentum step for the roller coaster's board trail. The
/// node's linear velocity (`+0x54`..`+0x70`) is scaled by the frame delta
/// (`+0x30`,`+0x34`), damping (`+0x38`) and an acceleration (`+0x3c`); the
/// result is summed with the node's accumulated force (`+0x58`..`+0x6c`) and
/// the previous position vector (`+0x20`, copied into a VU0 register) is
/// added before the trail's current positions (`+0x80`, `+0x90`) are
/// written.
pub fn roller_modifier_update_momentum(c: &mut Cpu) {
    c.cycles += 46;
    let m = Addr(r(c, A0));
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x20));

    // vf5 = (m+0x20).xyz * (m+0x40), stored to the stack scratch vector.
    c.lwc1(0, p(m.0, 0x40));
    c.mfc1(2, 0);
    c.lqc2(4, p(m.0, 0x20));
    c.qmtc2(2, 3);
    c.vu0().mul(
        0b1111,
        5,
        4,
        ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X),
    );
    let v = p(r(c, SP), 0x10);
    c.sqc2(5, v);

    // velocity += force * delta and velocity += accel * damping.
    c.lwc1(4, p(m.0, 0x30));
    c.lwc1(5, p(m.0, 0x34));
    c.lwc1(2, p(m.0, 0x50));
    c.lwc1(8, p(m.0, 0x5c));
    c.lwc1(1, p(m.0, 0x54));
    c.mul_s(2, 2, 4);
    c.lwc1(6, p(m.0, 0x60));
    c.mul_s(8, 8, 5);
    c.lwc1(0, p(m.0, 0x58));
    c.mul_s(1, 1, 4);
    c.lwc1(3, p(m.0, 0x64));
    c.mul_s(6, 6, 5);
    c.mul_s(0, 0, 4);
    c.lwc1(7, p(m.0, 0x38));
    c.mul_s(3, 3, 5);
    c.lwc1(9, p(m.0, 0x68));
    c.lwc1(5, p(m.0, 0x6c));
    c.add_s(2, 2, 8);
    c.lwc1(4, p(m.0, 0x70));
    c.add_s(1, 1, 6);
    c.add_s(0, 0, 3);
    c.lwc1(6, p(m.0, 0x3c));
    c.mul_s(4, 4, 7);
    c.lq(2, v);
    c.mul_s(9, 9, 7);
    c.swc1(6, p(r(c, SP), 0x1c));
    c.mul_s(5, 5, 7);
    c.sq(2, p(m.0, 0x80));
    c.add_s(0, 0, 4);
    c.add_s(2, 2, 9);
    c.add_s(1, 1, 5);
    c.swc1(0, p(r(c, SP), 0x18));
    c.swc1(2, p(r(c, SP), 0x10));
    c.swc1(1, p(r(c, SP), 0x14));
    c.lq(2, p(r(c, SP), 0x10));
    c.sq(2, p(m.0, 0x90));
    c.sq(2, r(c, SP));

    let ra = r(c, RA);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x20));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Heap, parser and engine teardown.

/// `void VramHeap_FreeUnlocked(VramHeap *self)` (0x366548): walk the block
/// index from `+0x23c0`, coalescing and freeing every unlocked block.
pub fn vram_heap_free_unlocked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p_3(c.g[S1][0], 0x23c0)).lw(c);
        let index = c.g[A1][0];
        c.g[S2][0] = 0x1c;
        if (index as i64) < 0 {
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        c.mult(0, V1, A1, S2);
        loop {
            // Outer walk: locate the bucket containing `index`.
            c.cycles += 5;
            c.g[V0][0] = Addr(p_3(c.g[S1][0], 0x1ff0)).lw(c);
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = Addr(p_3(c.g[A3][0], 0x10)).lw(c);
            let negative = (c.g[S0][0] as i64) < 0;
            c.g[T0][0] = c.g[V0][0];
            if negative {
                c.cycles += 1;
                c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            } else {
                c.cycles += 6;
                c.mult(0, V0, S0, S2);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                if c.g[V0][0] & 1 != 0 {
                    c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[A0][0] = 0x1c;
                    c.mult(0, V1, S0, A0);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[S0][0] = Addr(p_3(c.g[V0][0], 0x10)).lw(c);
                        let negative = (c.g[S0][0] as i64) < 0;
                        c.mult(0, V0, S0, A0);
                        if negative {
                            c.cycles += 1;
                            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
                            break;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        let free = (c.g[V0][0] & 1) == 0;
                        c.mult(0, V1, S0, A0);
                        if free {
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_65a8);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
                        break;
                    }
                }
            }
            // Coalesce or free the block.
            c.cycles += 3;
            let head = (c.g[V1][0] & 1) == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
            if head {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[S0][0];
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0036_63d8, 0x0036_65f0)?;
                    c.g[A1][0] = c.g[S0][0];
                }
            }
            c.cycles += 2;
            let more = (c.g[A1][0] as i64) >= 0;
            c.mult(0, V1, A1, S2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_6570);
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

// ---------------------------------------------------------------------------
// Roller modifier.

/// `sRmXform RollerModifier_GetPose(self, sRmXform* in)` (0x35e770): compose
/// the roller's rotation quaternion with the incoming transform (VU0), scale
/// it by -1 and add the translation, writing the result through a0.
pub fn rollermodifier_get_pose(c: &mut Cpu) {
    c.cycles += 55;
    c.lq(3, p(r32(c, A1), 0x40));
    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
    c.lq(6, p(r32(c, A1), 0x30));
    c.g[V0][0] = c.g[A0][0];
    c.sq(3, p(r32(c, SP), 0x10));
    c.sq(6, r32(c, SP));
    c.lqc2(4, p(r32(c, A1), 0x40));
    c.lqc2(5, p(r32(c, A1), 0x2b0));
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
    c.sqc2(8, p(r32(c, SP), 0x40));
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 0);
    c.lq(3, p(r32(c, SP), 0x40));
    c.sq(3, p(r32(c, SP), 0x30));
    c.mfc1(5, 0);
    c.lqc2(4, p(r32(c, SP), 0x30));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(r32(c, SP), 0x40));
    c.lq(3, p(r32(c, SP), 0x40));
    c.sq(3, p(r32(c, SP), 0x20));
    c.lqc2(4, p(r32(c, SP), 0x10));
    c.lqc2(5, p(r32(c, SP), 0x20));
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
    c.sqc2(8, p(r32(c, SP), 0x60));
    c.lq(3, p(r32(c, SP), 0x60));
    c.sq(3, p(r32(c, SP), 0x50));
    c.lqc2(3, r32(c, SP));
    c.lqc2(4, p(r32(c, SP), 0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, SP));
    c.lq(3, r32(c, SP));
    c.lq(5, p(r32(c, SP), 0x10));
    c.sq(3, r32(c, A0));
    c.sq(5, p(r32(c, A0), 0x10));
    let ra = r32(c, RA);
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x70));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Vram heap.

/// `void VramHeap_FreeAll(VramHeap* self)` (0x366618): walk the free list and
/// release every marked block.
pub fn vramheap_free_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(r32(c, S1), 0x23c0)).lw(c);
        let done = (c.g[A1][0] as i64) < 0;
        c.g[S2][0] = 0x1c;
        if done {
            c.cycles += 7;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[S3][0] = 0xffff_ffff_ffff_fffd;
        c.mult(0, 3, 5, 18);
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S1), 0x1ff0)).lw(c);
            c.g[A3][0] = sx(p(r32(c, V0), r32(c, V1)));
            c.g[S0][0] = Addr(p(r32(c, A3), 0x10)).lw(c);
            let leaf = (c.g[S0][0] as i64) < 0;
            c.g[T0][0] = c.g[V0][0];
            let mut need_next = false;
            if !leaf {
                c.cycles += 6;
                c.mult(0, 2, 16, 18);
                c.g[V1][0] = sx(p(r32(c, V0), r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 1;
                if c.g[V0][0] != 0 {
                    // break out with V1 already loaded from A3 below
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[A0][0] = 0x1c;
                    c.mult(0, 3, 16, 4);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = sx(p(r32(c, V1), r32(c, A2)));
                        c.g[S0][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
                        let end = (c.g[S0][0] as i64) < 0;
                        c.mult(0, 2, 16, 4);
                        if end {
                            need_next = true;
                            break;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = sx(p(r32(c, V0), r32(c, T0)));
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = c.g[V0][0] & 1;
                        let more = c.g[V0][0] == 0;
                        c.mult(0, 3, 16, 4);
                        if !more {
                            need_next = true;
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0036_6680);
                        }
                    }
                }
                if need_next {
                    c.cycles += 1;
                }
                c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            } else {
                c.cycles += 1;
                c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            let clean = c.g[V0][0] == 0;
            c.g[V0][0] = sx(((r32(c, V1) as i32) >> 1) as u32);
            if !clean {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let no_clear = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & c.g[S3][0];
                if !no_clear {
                    c.cycles += 1;
                    Addr(r32(c, A3)).sw(c, c.g[V0][0]);
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0036_63d8, 0x0036_66cc)?;
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            let more = (c.g[A1][0] as i64) >= 0;
            c.mult(0, 3, 5, 18);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_6648);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tex-flip modifier.

/// `void TexFlipModifier_HandleEvent(TexFlipModifier* self, int id, void* v)`
/// (0x35f598): event 300 stores the speed, 301 selects a frame from a float
/// (rounding up) and 302 advances to the next frame cyclically.
pub fn texflip_modifier_handle_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = 0x12d;
        let is301 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);

        'outer: {
            if is301 {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_le_s(0, 12);
                if !c.fcc() {
                    break 'outer;
                }
                c.cycles += 5;
                c.g[AT][0] = 0x4110_0000;
                c.mtc1(1, 0);
                c.c_lt_s(12, 0);
                if !c.fcc() {
                    break 'outer;
                }
                'round: {
                    c.cycles += 5;
                    c.cvt_w_s(1, 12);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(1, 12);
                    if !c.fcc() {
                        break 'round;
                    }
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.add_s(1, 1, 0);
                }
                c.cycles += 8;
                c.cvt_w_s(0, 1);
                c.mfc1(2, 0);
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = c.g[V0][0];
                c.sw(Addr(r(c, A2)).off(0x14).0, c.g[V0][0]);
                callee::TEXFLIP_SET_FRAME.call(c, 0x0035_f644)?;
                break 'outer;
            }

            'switch: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 302) as u64;
                let large = c.g[V0][0] == 0;
                c.g[V0][0] = 0x12c;
                if !large {
                    c.cycles += 2;
                    let is300 = c.g[A1][0] == c.g[V0][0];
                    frame.restore_ra(c, 0);
                    if !is300 {
                        c.cycles += 2;
                        break 'outer;
                    }
                    // event 300: speed = v
                    c.cycles += 2;
                    // The stored speed is the per-tick timer step; the script
                    // value is per 60 Hz frame, so at a higher rate the same
                    // flips per second need a smaller step (60 is unchanged).
                    let hz = crate::tick::rate(c);
                    if hz != 60 {
                        let k = FpuF32::from_f32(60.0 / hz as f32);
                        c.f[12] = (FpuF32(c.f[12]) * k).0;
                    }
                    c.swc1(12, Addr(r(c, A2)).off(0x8).0);
                    break 'switch;
                }

                c.cycles += 3;
                c.g[V0][0] = 0x12e;
                let is302 = c.g[A1][0] == c.g[V0][0];
                frame.restore_ra(c, 0);
                if !is302 {
                    c.cycles += 2;
                    break 'outer;
                }

                // event 302: next frame, cycling at count.
                {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r(c, A2)).off(0x14).lw(c);
                    c.g[V1][0] = Addr(r(c, A2)).off(0x20).lw(c);
                    c.g[V0][0] = sx(p(r(c, V0), 0x1));
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let inside = c.g[V1][0] != 0;
                    c.sw(Addr(r(c, A2)).off(0x14).0, c.g[V0][0]);
                    if !inside {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        c.sw(Addr(r(c, A2)).off(0x14).0, c.g[V0][0]);
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = Addr(r(c, A0)).off(0x14).lw(c);
                callee::TEXFLIP_SET_FRAME.call(c, 0x0035_f678)?;
                break 'switch;
            }
            c.cycles += 1;
        }

        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Functions.

/// `TexFlipModifier_ctor(self, desc, owner)` (0x35f0b8): a texture-flip
/// modifier over a frame sequence: store the owner, the description's first
/// frame, period, frame count and scale, derive the playback rate from the
/// description's rate and the global frame time, choose a random start frame
/// when the description asks for a negative first frame, then initialise the
/// frame table.
pub fn tex_flip_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        let desc = abi::arg(c, 1);
        c.g[V1][0] = lw_3(c, gp(c, 0x2a74)); // D_004A5B64 (the clock)
        frame.save(c, 16, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(p(this, 0x18), c.g[A2][0]);
        c.sw(p(this, 0x14), 0);
        c.sw(p(this, 0x1c), 0);
        c.sw(p(this, 0x20), 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        let clock = c.g[V1][0] as u32;
        c.g[V0][0] = lw_3(c, p(desc, 4));
        c.sw(this, c.g[V0][0]);
        // rate = desc->rate / (float)clock->frameTicks
        c.lwc1(0, p(clock, 0x10));
        c.cvt_s_w(0, 0);
        c.lwc1(1, p(desc, 0x10));
        c.div_s(0, 20, 0);
        c.mul_s(1, 1, 0);
        c.swc1(1, p(this, 8));
        c.g[V0][0] = lw_3(c, p(desc, 0xc));
        c.swc1(1, p(this, 0xc));
        c.sw(p(this, 4), 0);
        c.sw(p(this, 0x10), c.g[V0][0]);
        c.lwc1(1, p(this, 4));
        c.g[V0][0] = lw_3(c, p(desc, 8));
        c.mtc1(2, 0); // f0 = frame count
        c.cvt_s_w(0, 0);
        c.c_lt_s(0, 1);
        if !c.fcc() {
            // the frame count is in range: start at the description's frame
            c.sw(p(this, 0x14), c.g[V0][0]);
        } else {
            // a negative start frame means "random": 1..=8 frames of jitter
            c.cycles += 18;
            call_ptr(c, 0x0031_77f0, 0x0035_f140)?; // BXrand()
            c.g[V1][0] = 0x7f_0000;
            c.g[A1][0] = 0x3f80_0000;
            c.g[V1][0] |= 0xffff;
            c.g[AT][0] = 0x4100_0000;
            c.mtc1(1, 1); // f1 = 8.0
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= c.g[A1][0];
            c.mtc1(2, 2); // f2 = 1.0 + rand/2^23
            c.sub_s(0, 2, 20);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.cvt_s_w(1, 1);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.add_s(1, 1, 20);
            }
            c.cycles += 2;
            c.cvt_w_s(0, 1);
            c.swc1(0, p(this, 0x14));
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x0035_f190);
        c.g[A0][0] = c.g[S0][0];
        c.pc = 0x0035_f218;
        ee::s12::TexFlipModifier_Init(c);
        if c.pc != 0x0035_f190 {
            return Err(Unwound);
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `RailModifier_GetWorldMatrix(self, out)` (0x35c5a0): ask the instance's
/// node matrix, then multiply the rail modifier's transform by it in VU0 and
/// store the result in `out` (also copying it back into the modifier's own
/// matrix).
pub fn railmodifier_get_world_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        let frame = Frame::open(c, 0xb0);
        let this = abi::arg(c, 0);
        let out = abi::arg(c, 1);
        frame.save(c, 16, 0xa0);
        frame.save(c, 17, 0x90);
        c.g[S0][0] = c.g[A0][0];
        c.sd(p(sp(c), 0x80), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = lw_3(c, p(this, 0x40));
        c.g[RA][0] = sx(0x0035_c5c8);
        c.g[A1][0] = lw_3(c, p(this, 0x34));
        c.pc = 0x0034_fed8;
        ee::s11::Instance_GetNodeMatrix(c);
        if c.pc != 0x0035_c5c8 {
            return Err(Unwound);
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx(p(sp(c), 0x40));
        c.lqc2(4, out);
        c.lqc2(5, p(out, 0x10));
        c.lqc2(6, p(out, 0x20));
        c.lqc2(7, p(out, 0x30));
        c.lqc2(8, c.g[S0][0] as u32);
        c.lqc2(9, p(c.g[S0][0] as u32, 0x10));
        c.lqc2(10, p(c.g[S0][0] as u32, 0x20));
        c.lqc2(11, p(c.g[S0][0] as u32, 0x30));
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
        c.sqc2(12, c.g[V0][0] as u32);
        c.sqc2(13, p(c.g[V0][0] as u32, 0x10));
        c.sqc2(14, p(c.g[V0][0] as u32, 0x20));
        c.sqc2(15, p(c.g[V0][0] as u32, 0x30));
        c.lqc2(1, c.g[V0][0] as u32);
        c.lqc2(2, p(c.g[V0][0] as u32, 0x10));
        c.lqc2(3, p(c.g[V0][0] as u32, 0x20));
        c.lqc2(4, p(c.g[V0][0] as u32, 0x30));
        c.sqc2(1, sp(c));
        c.sqc2(2, p(sp(c), 0x10));
        c.sqc2(3, p(sp(c), 0x20));
        c.sqc2(4, p(sp(c), 0x30));
        c.lqc2(1, sp(c));
        c.lqc2(2, p(sp(c), 0x10));
        c.lqc2(3, p(sp(c), 0x20));
        c.lqc2(4, p(sp(c), 0x30));
        c.sqc2(1, out);
        c.sqc2(2, p(out, 0x10));
        c.sqc2(3, p(out, 0x20));
        c.sqc2(4, p(out, 0x30));
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void GsPkt_Finish(GsPktWriter* w, void** pp, int mode)` (0x3627a8):
/// close the writer's current packet block. An empty writer does nothing.
/// Mode 1 emits the opening block and leaves the old cursor on `*pp`; modes
/// 0 and 2 emit a block at the static buffer (`D_00505A80` for mode 0) plus
/// the two-quadword DMA kick on `*pp`; any other mode is a no-op.
pub fn gs_pkt_finish(c: &mut Cpu) {
    'b_3627d8: {
        c.cycles += 4;
        c.g[T1][0] = c.g[A0][0];
        c.g[V1][0] = c.ldw_2(T1, 0);
        let t = c.g[V1][0] == 0u64;
        c.g[T2][0] = c.g[A1][0];
        if t {
            break 'b_3627d8;
        } else {
            'b_362938: {
                'b_3628e4: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[V0][0] = c.ldw_2(T1, 0x4);
                        c.cycles += 20;
                        c.g[V1][0] = 0x10000000;
                        c.g[A1][0] = 0x50000000;
                        c.g[A2][0] = 0x8000;
                        c.g[A2][0] = c.g[A2][0] << 45;
                        c.stw_2(T2, 0, c.g[V0][0]);
                        c.g[A3][0] = 0xe;
                        c.g[V0][0] = c.ldw_2(T1, 0);
                        c.g[A0][0] = c.ldw_2(T1, 0x8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.stw_2(T1, 0x4, c.g[A0][0]);
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.std(A0, 0, c.g[V0][0]);
                        c.g[V1][0] = c.ldw_2(T1, 0x4);
                        c.std(V1, 0x8, 0u64);
                        c.g[A0][0] = c.ldw_2(T1, 0x4);
                        c.std(A0, 0x10, 0u64);
                        c.g[V0][0] = c.ldw_2(T1, 0);
                        c.g[A0][0] = c.ldw_2(T1, 0x4);
                        let t = true;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_362938;
                        } else {
                            break 'b_3628e4;
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = 0x2;
                            c.cycles += 2;
                            if c.g[A2][0] == c.g[V0][0] {
                                c.g[V0][0] = c.ldw_2(T1, 0x4);
                                break 'b_3628e4;
                            } else {
                                c.cycles += 2;
                                let ra = c.g[RA][0] as u32;
                                c.pc = ra;
                                return;
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A0][0] = 0x500000;
                            if t {
                                c.cycles += 41;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a80));
                                c.g[V0][0] = 0x60000000;
                                c.stw_2(T1, 0x4, c.g[A1][0]);
                                c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
                                c.std(A0, 0x5a80, c.g[V0][0]);
                                c.g[A1][0] = c.g[A1][0] << 32;
                                c.g[V0][0] = 0x50000000;
                                c.g[A2][0] = 0x50000000;
                                c.g[V1][0] = c.ldw_2(T1, 0x4);
                                c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                c.g[A3][0] = 0x8000;
                                c.g[A3][0] = c.g[A3][0] << 45;
                                c.g[T0][0] = 0xe;
                                c.std(V1, 0x8, 0u64);
                                c.g[V1][0] = c.ldw_2(T1, 0x4);
                                c.std(V1, 0x10, 0u64);
                                c.g[V0][0] = c.ldw_2(T1, 0);
                                c.g[A0][0] = c.ldw_2(T1, 0x4);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                                c.g[V0][0] = c.g[V0][0] << 32;
                                c.std(A0, 0x18, c.g[V0][0]);
                                c.g[V1][0] = c.ldw_2(T1, 0);
                                c.g[V0][0] = c.ldw_2(T1, 0x4);
                                c.g[V1][0] = c.g[V1][0] | 0x8000;
                                c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
                                c.std(V0, 0x20, c.g[V1][0]);
                                c.g[A0][0] = c.ldw_2(T1, 0x4);
                                c.std(A0, 0x28, c.g[T0][0]);
                                c.g[V0][0] = c.ldw_2(T1, 0x4);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                                c.stw_2(T1, 0x4, c.g[V0][0]);
                                c.g[V1][0] = c.ldw_2(T2, 0);
                                c.std(V1, 0, c.g[A1][0]);
                                c.g[A0][0] = c.ldw_2(T2, 0);
                                c.std(A0, 0x8, 0u64);
                                c.g[V0][0] = c.ldw_2(T2, 0);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                let ra = c.g[RA][0] as u32;
                                c.stw_2(T2, 0, c.g[V0][0]);
                                c.pc = ra;
                                return;
                            } else {
                                break 'b_3627d8;
                            }
                        }
                    }
                }
                c.cycles += 21;
                c.g[V1][0] = 0x10000000;
                c.g[A1][0] = 0x6c000000;
                c.g[A2][0] = 0x8000;
                c.g[A2][0] = c.g[A2][0] << 45;
                c.stw_2(T2, 0, c.g[V0][0]);
                c.g[A1][0] = c.g[A1][0] | 0x3f2;
                c.g[A3][0] = 0xe;
                c.g[V0][0] = c.ldw_2(T1, 0);
                c.g[A0][0] = c.ldw_2(T1, 0x8);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                c.stw_2(T1, 0x4, c.g[A0][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.std(A0, 0, c.g[V0][0]);
                c.g[V1][0] = c.ldw_2(T1, 0x4);
                c.std(V1, 0x8, 0u64);
                c.g[A0][0] = c.ldw_2(T1, 0x4);
                c.std(A0, 0x10, 0u64);
                c.g[V0][0] = c.ldw_2(T1, 0);
                c.g[A0][0] = c.ldw_2(T1, 0x4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                break 'b_362938;
            }
            c.cycles += 15;
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] << 32;
            c.std(A0, 0x18, c.g[V0][0]);
            c.g[V1][0] = c.ldw_2(T1, 0);
            c.g[V0][0] = c.ldw_2(T1, 0x4);
            c.g[V1][0] = c.g[V1][0] | 0x8000;
            c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
            c.std(V0, 0x20, c.g[V1][0]);
            c.g[A0][0] = c.ldw_2(T1, 0x4);
            c.std(A0, 0x28, c.g[A3][0]);
            c.g[V0][0] = c.ldw_2(T1, 0x4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
            c.stw_2(T1, 0x4, c.g[V0][0]);
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `void TexFlipModifier_Update(FrameMod* self)` (0x35f410): advance the
/// flip timer; when it reaches the period, switch the texture page (picking
/// between the two page variants from the frame parity), re-arm the timer
/// and notify the owner.
pub fn tex_flip_modifier_update(c: &mut Cpu) {
    'b_35f584: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.mtc1(0, 0);
        c.sq(16, c.pa(SP, 0x20));
        c.sq(17, c.pa(SP, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.std(SP, 0, c.g[RA][0]);
        c.swc1(20, c.pa(SP, 0x30));
        c.lwc1(1, c.pa(S0, 0x8));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lq(16, c.pa(SP, 0x20));
            break 'b_35f584;
        } else {
            'b_35f580: {
                c.cycles += 8;
                c.lwc1(0, c.pa(S0, 0x4));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.add_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.swc1(0, c.pa(S0, 0x4));
                if t {
                    break 'b_35f580;
                } else {
                    'b_35f578: {
                        'b_35f574: {
                            'b_35f4b4: {
                                c.cycles += 4;
                                c.sub_s(0, 0, 20);
                                c.g[V1][0] = c.ldw_2(S0, 0);
                                let t = c.g[V1][0] != 0u64;
                                c.swc1(0, c.pa(S0, 0x4));
                                if t {
                                    break 'b_35f4b4;
                                } else {
                                    'b_35f498: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.ldw_2(S0, 0x10);
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.ldw_2(S0, 0x14);
                                        if t {
                                            break 'b_35f498;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] = c.ldw_2(S0, 0x20);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V1][0] != 0u64;
                                            c.stw_2(S0, 0x14, c.g[V0][0]);
                                            if t {
                                                break 'b_35f574;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.stw_2(S0, 0x14, 0u64);
                                                if t {
                                                    break 'b_35f574;
                                                } else {
                                                    break 'b_35f498;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[V0][0] as i64) >= 0;
                                    c.stw_2(S0, 0x14, c.g[V0][0]);
                                    if t {
                                        break 'b_35f574;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.ldw_2(S0, 0x20);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        let t = true;
                                        c.stw_2(S0, 0x14, c.g[V0][0]);
                                        if t {
                                            break 'b_35f574;
                                        } else {
                                            break 'b_35f4b4;
                                        }
                                    }
                                }
                            }
                            'b_35f504: {
                                c.cycles += 3;
                                c.g[S1][0] = 0x1;
                                let t = c.g[V1][0] != c.g[S1][0];
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_35f504;
                                } else {
                                    c.cycles += 15;
                                    c.g[V1][0] = c.ldw_2(GP, 0x2a74);
                                    c.g[A0][0] = 0x2;
                                    c.g[AT][0] = 0x41200000;
                                    c.mtc1(1, 0);
                                    c.lwc1(1, c.pa(V1, 0x10));
                                    c.cvt_s_w(1, 1);
                                    c.g[V0][0] = c.ldw_2(S0, 0x14);
                                    c.g[V1][0] = c.ldw_2(S0, 0x20);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.stw_2(S0, 0, c.g[A0][0]);
                                    c.div_s(0, 0, 1);
                                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    c.stw_2(S0, 0x14, c.g[V0][0]);
                                    let t = c.g[V1][0] != 0u64;
                                    c.swc1(0, c.pa(S0, 0x8));
                                    if t {
                                        break 'b_35f574;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.stw_2(S0, 0x14, 0u64);
                                        if t {
                                            break 'b_35f574;
                                        } else {
                                            break 'b_35f504;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_35f578;
                            } else {
                                c.cycles += 25;
                                c.g[RA][0] = sx(0x0035f514);
                                c.pc = 0x003177f0;
                                call_indirect(c, 0x003177f0);
                                if c.pc != 0x0035f514 {
                                    return;
                                }
                                c.g[V1][0] = 0x7f0000;
                                c.g[A1][0] = 0x3f800000;
                                c.g[V1][0] = c.g[V1][0] | 0xffff;
                                c.g[AT][0] = 0x3f400000;
                                c.mtc1(1, 2);
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                c.g[AT][0] = 0x3e800000;
                                c.mtc1(1, 3);
                                c.g[A0][0] = c.g[V0][0] | c.g[A1][0];
                                c.lwc1(1, c.pa(S0, 0xc));
                                c.mtc1(4, 4);
                                c.g[V0][0] = c.ldw_2(S0, 0x14);
                                c.sub_s(0, 4, 20);
                                c.g[A0][0] = c.ldw_2(S0, 0x20);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.stw_2(S0, 0, c.g[S1][0]);
                                c.g[A0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                c.stw_2(S0, 0x14, c.g[V0][0]);
                                c.mul_s(0, 0, 2);
                                c.add_s(0, 0, 3);
                                c.mul_s(1, 1, 0);
                                let t = c.g[A0][0] != 0u64;
                                c.swc1(1, c.pa(S0, 0x8));
                                if t {
                                    break 'b_35f574;
                                } else {
                                    c.cycles += 1;
                                    c.stw_2(S0, 0x14, 0u64);
                                    break 'b_35f574;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        break 'b_35f578;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0035f580);
                    c.g[A1][0] = c.ldw_2(A0, 0x14);
                    c.pc = 0x0035f378;
                    call_indirect(c, 0x0035f378);
                    if c.pc != 0x0035f580 {
                        return;
                    }
                    break 'b_35f580;
                }
            }
            c.cycles += 1;
            c.lq(16, c.pa(SP, 0x20));
            break 'b_35f584;
        }
    }
    c.cycles += 5;
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    c.lwc1(20, c.pa(SP, 0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// Rail roller modifier.

/// `func_0035CFF0(RollerModifier *self, const Vec4 *v)` (0x35cff0):
/// remember the incoming position, invert the modifier's 3x3 rotation
/// matrix and transform `self->pos` by it (world -> modifier space).
///
/// The original copies the unaligned 0x20-byte rotation at `self+0x50`
/// (plus the word at `+0x70`) to a stack temporary, inverts it with the
/// cofactor/determinant expansion, then applies the inverse to
/// `self+0x90..0x9c` and stores the result at `self+0x30`.
pub fn roller_modifier_set_angular_momentum(c: &mut Cpu) {
    c.cycles += 165;
    let obj = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    let base = |c: &Cpu| c.g[A0][0] as u32;
    c.lq(2, out);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xc0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 18); // f18 = 1.0
    c.sq(2, obj.wrapping_add(0x90));
    // Copy the unaligned rotation matrix `self+0x50..0x70` to `sp`.
    c.ldl(2, obj.wrapping_add(0x57));
    c.ldr(2, obj.wrapping_add(0x50));
    c.ldl(3, obj.wrapping_add(0x5f));
    c.ldr(3, obj.wrapping_add(0x58));
    c.ldl(5, obj.wrapping_add(0x67));
    c.ldr(5, obj.wrapping_add(0x60));
    c.ldl(6, obj.wrapping_add(0x6f));
    c.ldr(6, obj.wrapping_add(0x68));
    c.sdl(2, sp(c).wrapping_add(0x7));
    c.sdr(2, sp(c));
    c.sdl(3, sp(c).wrapping_add(0xf));
    c.sdr(3, sp(c).wrapping_add(0x8));
    c.sdl(5, sp(c).wrapping_add(0x17));
    c.sdr(5, sp(c).wrapping_add(0x10));
    c.sdl(6, sp(c).wrapping_add(0x1f));
    c.sdr(6, sp(c).wrapping_add(0x18));
    c.g[V0][0] = c.lw(base(c).wrapping_add(0x70));
    c.sw(sp(c).wrapping_add(0x20), c.g[V0][0]);
    // Invert the 3x3 part (cofactor expansion, determinant in f0).
    c.lwc1(12, sp(c).wrapping_add(0x10));
    c.lwc1(11, sp(c).wrapping_add(0x20));
    c.lwc1(4, sp(c).wrapping_add(0x14));
    c.lwc1(15, sp(c).wrapping_add(0x1c));
    c.mul_s(9, 12, 11);
    c.lwc1(10, sp(c).wrapping_add(0x8));
    c.lwc1(14, sp(c).wrapping_add(0x4));
    c.mul_s(0, 4, 15);
    c.mul_s(8, 15, 10);
    c.lwc1(3, sp(c).wrapping_add(0xc));
    c.mul_s(6, 11, 14);
    c.lwc1(1, sp(c));
    c.mul_s(7, 10, 12);
    c.lwc1(2, sp(c).wrapping_add(0x18));
    c.sub_s(9, 9, 0);
    c.mul_s(5, 14, 4);
    c.sub_s(8, 8, 6);
    c.mul_s(0, 1, 9);
    c.swc1(9, sp(c).wrapping_add(0x30));
    c.sub_s(5, 5, 7);
    c.mul_s(6, 3, 8);
    c.swc1(8, sp(c).wrapping_add(0x34));
    c.mul_s(16, 4, 1);
    c.mul_s(7, 2, 5);
    c.swc1(5, sp(c).wrapping_add(0x38));
    c.add_s(0, 0, 6);
    c.mul_s(13, 1, 11);
    c.mul_s(17, 15, 1);
    c.mul_s(6, 3, 10);
    c.add_s(0, 0, 7);
    c.mul_s(1, 1, 12);
    c.mul_s(7, 14, 3);
    c.mul_s(11, 11, 3);
    c.mul_s(10, 10, 2);
    c.mul_s(12, 12, 2);
    c.mul_s(4, 2, 4);
    c.mul_s(3, 3, 15);
    c.mul_s(2, 2, 14);
    c.div_s(0, 18, 0); // 1/det
    c.sub_s(4, 4, 11);
    c.sub_s(13, 13, 10);
    c.sub_s(6, 6, 16);
    c.sub_s(3, 3, 12);
    c.swc1(4, sp(c).wrapping_add(0x3c));
    c.sub_s(2, 2, 17);
    c.swc1(13, sp(c).wrapping_add(0x40));
    c.sub_s(1, 1, 7);
    c.swc1(6, sp(c).wrapping_add(0x44));
    c.swc1(3, sp(c).wrapping_add(0x48));
    c.mul_s(9, 9, 0);
    c.swc1(2, sp(c).wrapping_add(0x4c));
    c.mul_s(8, 8, 0);
    c.mul_s(7, 1, 0);
    c.swc1(1, sp(c).wrapping_add(0x50));
    c.mul_s(5, 5, 0);
    c.swc1(9, sp(c).wrapping_add(0x90));
    c.mul_s(4, 4, 0);
    c.swc1(8, sp(c).wrapping_add(0x94));
    c.mul_s(13, 13, 0);
    c.swc1(7, sp(c).wrapping_add(0xb0));
    c.mul_s(6, 6, 0);
    c.swc1(5, sp(c).wrapping_add(0x98));
    c.mul_s(3, 3, 0);
    c.swc1(4, sp(c).wrapping_add(0x9c));
    c.mul_s(2, 2, 0);
    c.swc1(13, sp(c).wrapping_add(0xa0));
    c.swc1(6, sp(c).wrapping_add(0xa4));
    c.swc1(3, sp(c).wrapping_add(0xa8));
    c.swc1(2, sp(c).wrapping_add(0xac));
    // Move the inverse to `sp+0x60` and its translation row to `sp+0x20`.
    c.ldl(2, sp(c).wrapping_add(0x97));
    c.ldr(2, sp(c).wrapping_add(0x90));
    c.ldl(3, sp(c).wrapping_add(0x9f));
    c.ldr(3, sp(c).wrapping_add(0x98));
    c.ldl(5, sp(c).wrapping_add(0xa7));
    c.ldr(5, sp(c).wrapping_add(0xa0));
    c.ldl(6, sp(c).wrapping_add(0xaf));
    c.ldr(6, sp(c).wrapping_add(0xa8));
    c.sdl(2, sp(c).wrapping_add(0x67));
    c.sdr(2, sp(c).wrapping_add(0x60));
    c.sdl(3, sp(c).wrapping_add(0x6f));
    c.sdr(3, sp(c).wrapping_add(0x68));
    c.sdl(5, sp(c).wrapping_add(0x77));
    c.sdr(5, sp(c).wrapping_add(0x70));
    c.sdl(6, sp(c).wrapping_add(0x7f));
    c.sdr(6, sp(c).wrapping_add(0x78));
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0xb0));
    c.sw(sp(c).wrapping_add(0x80), c.g[V0][0]);
    c.ldl(2, sp(c).wrapping_add(0x67));
    c.ldr(2, sp(c).wrapping_add(0x60));
    c.ldl(3, sp(c).wrapping_add(0x6f));
    c.ldr(3, sp(c).wrapping_add(0x68));
    c.ldl(5, sp(c).wrapping_add(0x77));
    c.ldr(5, sp(c).wrapping_add(0x70));
    c.ldl(6, sp(c).wrapping_add(0x7f));
    c.ldr(6, sp(c).wrapping_add(0x78));
    c.sdl(2, sp(c).wrapping_add(0x7));
    c.sdr(2, sp(c));
    c.sdl(3, sp(c).wrapping_add(0xf));
    c.sdr(3, sp(c).wrapping_add(0x8));
    c.sdl(5, sp(c).wrapping_add(0x17));
    c.sdr(5, sp(c).wrapping_add(0x10));
    c.sdl(6, sp(c).wrapping_add(0x1f));
    c.sdr(6, sp(c).wrapping_add(0x18));
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x80));
    c.sw(sp(c).wrapping_add(0x20), c.g[V0][0]);
    // out = inverse * self->pos, row by row.
    c.lwc1(1, base(c).wrapping_add(0x90));
    c.lwc1(7, base(c).wrapping_add(0x94));
    c.lwc1(3, sp(c));
    c.lwc1(9, sp(c).wrapping_add(0xc));
    c.lwc1(2, sp(c).wrapping_add(0x4));
    c.mul_s(3, 3, 1);
    c.lwc1(8, sp(c).wrapping_add(0x10));
    c.mul_s(9, 9, 7);
    c.lwc1(0, sp(c).wrapping_add(0x8));
    c.mul_s(2, 2, 1);
    c.lwc1(4, sp(c).wrapping_add(0x14));
    c.mul_s(8, 8, 7);
    c.mul_s(0, 0, 1);
    c.lwc1(5, base(c).wrapping_add(0x98));
    c.mul_s(4, 4, 7);
    c.lwc1(6, sp(c).wrapping_add(0x18));
    c.lwc1(7, sp(c).wrapping_add(0x1c));
    c.add_s(3, 3, 9);
    c.lwc1(1, sp(c).wrapping_add(0x20));
    c.add_s(2, 2, 8);
    c.add_s(0, 0, 4);
    c.lwc1(8, base(c).wrapping_add(0x9c));
    c.mul_s(1, 1, 5);
    c.mul_s(6, 6, 5);
    c.swc1(8, sp(c).wrapping_add(0x4c));
    c.mul_s(7, 7, 5);
    c.add_s(0, 0, 1);
    c.add_s(3, 3, 6);
    c.add_s(2, 2, 7);
    c.swc1(0, sp(c).wrapping_add(0x48));
    c.swc1(3, sp(c).wrapping_add(0x40));
    c.swc1(2, sp(c).wrapping_add(0x44));
    c.lq(2, sp(c).wrapping_add(0x40));
    c.sq(2, base(c).wrapping_add(0x30));
    c.sq(2, sp(c).wrapping_add(0x30));
    c.g[SP][0] = sx(sp(c).wrapping_add(0xc0));
    abi::ret(c);
}

/// `void func_00364360(void* self, void** pp)` (0x364360): emit the default
/// render state: a state packet from the GS state word, the alpha, clamp,
/// zbuffer and test packets for both passes, and the finishing packet.
pub fn defer_man_emit_default_state(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    c.cycles += 16;
    c.g[A2][0] = 0xffffffffffffffff;
    frame.save(c, S2, 0x60);
    c.g[V1][0] = sx(at_3(c, SP, 0x10));
    frame.save(c, S3, 0x50);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S4, 0x40);
    c.g[A1][0] = 0u64;
    frame.save(c, S1, 0x70);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    c.g[A0][0] = c.g[SP][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = sx(at_3(c, SP, 0x20));
    c.g[S3][0] = lw_4(c, at_3(c, GP, 0x2a90));
    'l_3643a0: loop {
        c.cycles += 7;
        sh(c, r32(c, V1), c.g[A2][0]);
        c.g[A1][0] = sx(at_3(c, A1, 0x1));
        c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[V1][0] = sx(at_3(c, V1, 0x2));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x003643a0);
            }
            continue 'l_3643a0;
        } else {
            c.cycles += 126;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x003643c8);
            c.g[A2][0] = 0x10;
            if call_ptr(c, 0x00416210, 0x003643c8).is_err() {
                return;
            }
            c.g[S0][0] = 0xf0000;
            c.g[A0][0] = lw_4(c, r32(c, S2));
            c.g[V0][0] = 0x500000;
            c.g[T0][0] = sx(at_3(c, V0, 0x1420));
            ldl(c, 5, r32(c, T0));
            ldl(c, 6, at_3(c, T0, 0x8));
            c.g[A3][0] = lw_4(c, at_3(c, T0, 0x10));
            sdl(c, 5, r32(c, SP));
            sdl(c, 6, at_3(c, SP, 0x8));
            sw(c, at_3(c, SP, 0x10), c.g[A3][0]);
            c.g[V1][0] = 0x10000000;
            c.g[V1][0] = c.g[V1][0] | 0x1;
            sd(c, r32(c, A0), c.g[V1][0]);
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[T1][0] = 0x3fff0000;
            c.g[T0][0] = c.g[S2][0];
            c.g[V1][0] = lw_4(c, r32(c, S2));
            c.g[T1][0] = c.g[T1][0] | 0xffff;
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0u64;
            sd(c, at_3(c, V1, 0x8), 0u64);
            c.g[S0][0] = c.g[S0][0] | 0xf000;
            c.g[V0][0] = lw_4(c, r32(c, S2));
            sd(c, at_3(c, V0, 0x10), 0u64);
            c.g[V1][0] = lw_4(c, r32(c, S2));
            sd(c, at_3(c, V1, 0x18), c.g[A1][0]);
            c.g[A1][0] = lw_4(c, r32(c, SP));
            c.g[V0][0] = lw_4(c, r32(c, S2));
            c.g[A3][0] = lw_4(c, at_3(c, SP, 0xc));
            c.g[A1][0] = c.g[A1][0] & 0x3c0;
            c.g[V0][0] = sx(at_3(c, V0, 0x20));
            c.g[A1][0] = sx(r32(c, A1) >> 6);
            c.g[A3][0] = c.g[A3][0] & c.g[T1][0];
            c.g[RA][0] = sx(0x00364468);
            sw(c, r32(c, S2), c.g[V0][0]);
            if callee::C00364CD0.call(c, 0x00364468).is_err() {
                return;
            }
            c.g[V1][0] = lw_4(c, r32(c, S2));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw_4(c, r32(c, SP));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = sx(at_3(c, V1, 0x30));
            sw(c, at_3(c, SP, 0x20), 0u64);
            c.g[A1][0] = c.g[A1][0] & 0xc;
            sw(c, at_3(c, SP, 0x24), c.g[V0][0]);
            sw(c, at_3(c, SP, 0x28), c.g[V1][0]);
            c.g[RA][0] = sx(0x00364494);
            c.g[A1][0] = sx(r32(c, A1) >> 2);
            if callee::C003625C0.call(c, 0x00364494).is_err() {
                return;
            }
            c.g[A1][0] = lw_4(c, at_3(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = lw_4(c, r32(c, SP));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.g[A1][0] & 0x7c;
            c.g[A2][0] = sx(r32(c, A2) >> 12);
            c.g[A1][0] = sx(r32(c, A1) >> 2);
            c.g[RA][0] = sx(0x003644b8);
            c.g[A2][0] = c.g[A2][0] & 0xff;
            if callee::C00362478.call(c, 0x003644b8).is_err() {
                return;
            }
            c.g[A3][0] = lw_4(c, at_3(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw_4(c, at_3(c, S3, 0x5a80));
            c.g[T0][0] = 0u64;
            c.g[A2][0] = lw_4(c, at_3(c, S3, 0x5a40));
            c.g[A3][0] = sx(r32(c, A3) >> 22);
            c.g[RA][0] = sx(0x003644d8);
            c.g[A3][0] = c.g[A3][0] & 0x1;
            if callee::C00362660.call(c, 0x003644d8).is_err() {
                return;
            }
            c.g[A2][0] = lw_4(c, at_3(c, SP, 0x4));
            c.g[T0][0] = 0u64;
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = c.g[A2][0] & c.g[S0][0];
            c.g[A1][0] = sx(r32(c, A2) >> 23);
            c.g[A2][0] = sx(r32(c, A2) >> 20);
            c.g[A3][0] = sx(r32(c, A3) >> 12);
            c.g[A1][0] = c.g[A1][0] & 0x3;
            c.g[RA][0] = sx(0x00364500);
            c.g[A2][0] = c.g[A2][0] & 0x3;
            if callee::C003626D8.call(c, 0x00364500).is_err() {
                return;
            }
            c.g[A1][0] = lw_4(c, r32(c, SP));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0x1;
            c.g[A1][0] = c.g[A1][0] & 0x30;
            c.g[RA][0] = sx(0x00364518);
            c.g[A1][0] = sx(r32(c, A1) >> 4);
            if callee::C003625C0.call(c, 0x00364518).is_err() {
                return;
            }
            c.g[A1][0] = lw_4(c, at_3(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = lw_4(c, r32(c, SP));
            c.g[A3][0] = 0x1;
            c.g[A1][0] = c.g[A1][0] & 0xf80;
            c.g[A2][0] = sx(r32(c, A2) >> 20);
            c.g[A1][0] = sx(r32(c, A1) >> 7);
            c.g[RA][0] = sx(0x0036453c);
            c.g[A2][0] = c.g[A2][0] & 0xff;
            if callee::C00362478.call(c, 0x0036453c).is_err() {
                return;
            }
            c.g[A3][0] = lw_4(c, at_3(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = lw_4(c, at_3(c, S3, 0x5a40));
            c.g[T0][0] = 0x1;
            c.g[A1][0] = lw_4(c, at_3(c, S3, 0x5a80));
            c.g[A3][0] = sx(r32(c, A3) >> 23);
            c.g[RA][0] = sx(0x0036455c);
            c.g[A3][0] = c.g[A3][0] & 0x1;
            if callee::C00362660.call(c, 0x0036455c).is_err() {
                return;
            }
            c.g[A2][0] = lw_4(c, at_3(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = 0x1;
            c.g[S0][0] = c.g[A2][0] & c.g[S0][0];
            c.g[A1][0] = sx(r32(c, A2) >> 25);
            c.g[A2][0] = sx(r32(c, A2) >> 22);
            c.g[A3][0] = sx(r32(c, S0) >> 12);
            c.g[A1][0] = c.g[A1][0] & 0x3;
            c.g[RA][0] = sx(0x00364584);
            c.g[A2][0] = c.g[A2][0] & 0x3;
            if callee::C003626D8.call(c, 0x00364584).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x00364594);
            c.g[A2][0] = 0x1;
            if callee::C003627A8.call(c, 0x00364594).is_err() {
                return;
            }
            frame.restore(c, S0, 0x80);
            frame.restore(c, S1, 0x70);
            frame.restore(c, S2, 0x60);
            frame.restore(c, S3, 0x50);
            frame.restore(c, S4, 0x40);
            c.g[RA][0] = ld(c, at_3(c, SP, 0x30));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at_3(c, SP, 0x90));
            c.pc = t;
            return;
        }
    }
}

pub(crate) mod uv_scroll {

    /// Rotation angle (radians); 0 means no rotation.
    pub const ANGLE: u32 = 0x10;

    /// Rotation axis x, y, z.
    pub const AXIS_X: u32 = 0x20;

    pub const AXIS_Y: u32 = 0x24;

    pub const AXIS_Z: u32 = 0x28;

    /// Translation (u, v), written into the matrix's last row.
    pub const TRANS_U: u32 = 0x30;

    pub const TRANS_V: u32 = 0x34;
}

/// The identity matrix in `.rodata` (0x4ff1a0) copied when there is no
/// rotation.
pub(crate) const IDENTITY_MATRIX: u32 = 0x004f_f1a0;

/// `Math_SinCos(float angle /* f12 */, float* sin, float* cos)`.
pub(crate) const MATH_SIN_COS: Callee = Callee {
    addr: 0x0031_be50,
    f: ee::s10::f_0031be50,
};

/// `float* UVScrollModifier_GetMatrix(UVScrollModifier* self, float m[4][4])`
/// (0x35fc20), hot during races (rail/surface UV animation): builds the texture
/// matrix of a scrolling UV modifier. With a non-zero angle, `m` is the
/// rotation by `angle` about the axis (the Rodrigues form
/// `t*a*a^T + c*I + s*[a]x` with `t = 1 - c`; rows 0..2, column 3 zero,
/// `m[3][3] = 1`); with angle 0 it is the identity (a VU0 quadword copy).
/// Then `m[3][0] = u`, `m[3][1] = v` (the UV scroll offset). Returns `m`.
pub fn uv_scroll_modifier_get_matrix(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let this = Addr(r32(c, S1));
        let zero = FpuF32(0);
        c.f[20] = zero.0;
        let angle = this.off(uv_scroll::ANGLE).read_fpu(c);
        c.f[12] = angle.0;
        let no_rotation = angle.eq_s(zero);
        crate::abi::set_fcc(c, no_rotation);
        c.g[S0][0] = c.g[A1][0];
        let m = Addr(r32(c, S0));

        if no_rotation {
            c.cycles += 10;
            c.g[V0][0] = sx(IDENTITY_MATRIX);
            for row in 0..4u32 {
                c.lqc2(1 + row as usize, IDENTITY_MATRIX + row * 0x10);
            }
            for row in 0..4u32 {
                c.sqc2(1 + row as usize, m.0.wrapping_add(row * 0x10));
            }
        } else {
            c.cycles += 60;
            // Math_SinCos(angle, &sin /* sp+0 */, &cos /* sp+4 */).
            let s = sp(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[SP][0] | 0x4;
            MATH_SIN_COS.call(c, 0x0035_fc5c)?;
            let ax_at = this.off(uv_scroll::AXIS_X);
            let ay_at = this.off(uv_scroll::AXIS_Y);
            let az_at = this.off(uv_scroll::AXIS_Z);

            let cos = Addr(s.wrapping_add(4)).read_fpu(c);
            c.g[AT][0] = 0x3f80_0000;
            let one = FpuF32(0x3f80_0000);
            let ax = ax_at.read_fpu(c);
            let t = one - cos;
            let sin = Addr(s).read_fpu(c);
            let ay = ay_at.read_fpu(c);
            let az = az_at.read_fpu(c);
            let tx = t * ax;
            let sy = sin * ay;
            let sz = sin * az;
            let m00 = tx * ax + cos;
            let sx_ = sin * ax;
            let ty = t * ay;
            let tz = t * az;
            m.write_fpu(c, m00);
            // The axis is re-read before each use (m may alias self).
            let m10 = tx * ay_at.read_fpu(c) + sz;
            m.off(0x10).write_fpu(c, m10);
            let az1 = az_at.read_fpu(c);
            m.off(0x30).write_fpu(c, zero);
            let m20 = tx * az1 - sy;
            m.off(0x20).write_fpu(c, m20);
            let m01 = ty * ax_at.read_fpu(c) - sz;
            m.off(0x04).write_fpu(c, m01);
            let m11 = ty * ay_at.read_fpu(c) + cos;
            m.off(0x14).write_fpu(c, m11);
            let az2 = az_at.read_fpu(c);
            m.off(0x34).write_fpu(c, zero);
            let m21 = ty * az2 + sx_;
            m.off(0x24).write_fpu(c, m21);
            let m02 = tz * ax_at.read_fpu(c) + sy;
            m.off(0x08).write_fpu(c, m02);
            let m12 = tz * ay_at.read_fpu(c) - sx_;
            m.off(0x18).write_fpu(c, m12);
            let az3 = az_at.read_fpu(c);
            m.off(0x2c).write_fpu(c, zero);
            let tz_az = tz * az3;
            m.off(0x3c).write_fpu(c, one);
            m.off(0x38).write_fpu(c, zero);
            m.off(0x0c).write_fpu(c, zero);
            let m22 = tz_az + cos;
            m.off(0x1c).write_fpu(c, zero);
            m.off(0x28).write_fpu(c, m22);

            // FPU registers as the original leaves them.
            c.f[0] = az3.0;
            c.f[1] = m12.0;
            c.f[2] = m20.0;
            c.f[3] = m21.0;
            c.f[4] = m22.0;
            c.f[5] = sx_.0;
            c.f[6] = az.0;
            c.f[7] = cos.0;
            c.f[8] = sz.0;
            c.f[9] = sy.0;
            c.f[10] = one.0;
        }

        // Translation into the last row, then return m.
        c.cycles += 11;
        let u = this.off(uv_scroll::TRANS_U).read_fpu(c);
        c.f[1] = u.0;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        m.off(0x30).write_fpu(c, u);
        frame.restore_fpr(c, 20, 0x40);
        let v = this.off(uv_scroll::TRANS_V).read_fpu(c);
        c.f[0] = v.0;
        frame.restore(c, S1, 0x20);
        m.off(0x34).write_fpu(c, v);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod oc230_defer {
    use crate::call::Callee;

    pub const SPR_STREAM_BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ssx3_ee::s12::SprStream_BeginBlock,
    };

    pub const SPR_STREAM_NEXT_INDEX: Callee = Callee {
        addr: 0x0038_f6a8,
        f: ssx3_ee::s12::SprStream_NextIndex,
    };

    /// `memset` (another shard: through the function table).
    pub const MEMSET: u32 = 0x003e_6448;

    /// The item count and the items (key, value: 8 bytes each).
    pub const COUNT: u32 = 0x6_7ca4;

    pub const ITEMS: u32 = 0x6_7ca8;

    /// The second item buffer (global).
    pub const SCRATCH: u32 = 0x0050_5b70;
}

/// `void DeferMan_SortBuckets(DeferMan* m)` (0x364050): LSD radix sort of
/// the deferred draw items (8 bytes, sorted by the 32-bit key) by bytes:
/// four 257-entry histograms (one per key byte) in a scratchpad stream
/// block (gp+0x1384), counted in one pass and prefix-summed, then four
/// scatter passes ping-ponging between the items and the global buffer
/// (an even number, so the result ends in the items); then the stream
/// moves to its next block.
pub fn defer_man_sort_buckets(c: &mut Cpu) {
    use oc230_defer as d;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = d::ITEMS as u64;
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[A1][0] = 0x50_0000;
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x6_0000));
        frame.save_ra(c, 0);
        c.g[S1][0] = oc230_add(c, A0, V1); // src = the items
        c.g[S4][0] = c.lw(oc230_gp(c, 0x1384)); // the scratchpad stream
        c.g[S3][0] = sx(d::SCRATCH); // dst
        frame.save(c, S2, 0x30);
        c.g[A1][0] = 0;
        c.g[S2][0] = c.lw(r32(c, V0).wrapping_add(d::COUNT - 0x6_0000)); // n
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = u64::MAX;
        c.g[A3][0] = 0;
        d::SPR_STREAM_BEGIN_BLOCK.call(c, 0x0036_40a4)?;
        c.g[S0][0] = c.g[V0][0]; // the histograms
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0x1010;
        call_ptr(c, d::MEMSET, 0x0036_40b8)?;
        let hist = r32(c, S0);

        // Count the four key bytes (histogram k at +0x404 * k, entry b + 1).
        let empty = (c.g[S2][0] as i64) <= 0;
        c.g[T0][0] = c.g[S1][0];
        if !empty {
            c.cycles += 2;
            c.g[A3][0] = c.g[S2][0];
            loop {
                c.cycles += 28;
                let item = r32(c, T0);
                c.g[A0][0] = c.lw(item);
                c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
                c.g[T0][0] = sx(item.wrapping_add(8));
                c.g[V1][0] = c.g[A0][0] & 0xff;
                c.g[A2][0] = sx(r32(c, A0) >> 6) & 0x3fc;
                c.g[V1][0] = oc230_sll(c, V1, 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(hist));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(hist));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(4));
                c.g[A1][0] = sx(r32(c, A0) >> 14) & 0x3fc;
                c.g[A0][0] = sx(r32(c, A0) >> 24);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(hist));
                let v = c.g[V0][0];
                c.sw(r32(c, V1).wrapping_add(4), v);
                c.g[A0][0] = oc230_sll(c, A0, 2);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(hist));
                for (base, off, reg) in [(A2, 0x408u32, V0), (A1, 0x80c, V1), (A0, 0xc10, V0)] {
                    let at = r32(c, base).wrapping_add(off);
                    c.g[reg][0] = c.lw(at);
                    c.g[reg][0] = sx(r32(c, reg).wrapping_add(1));
                    let v = c.g[reg][0];
                    c.sw(at, v);
                }
                if c.g[A3][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_40c8);
                }
            }
        }

        // Prefix sums: hist[k][i] += hist[k][i - 1] for i = 1..255.
        c.cycles += 2;
        c.g[T2][0] = sx(hist.wrapping_add(4));
        c.g[T3][0] = 0xfe;
        loop {
            c.cycles += 19;
            let at = r32(c, T2);
            c.g[V1][0] = c.lw(at);
            c.g[T3][0] = sx(r32(c, T3).wrapping_sub(1));
            c.g[A0][0] = c.lw(at.wrapping_add(0x404));
            c.g[A1][0] = c.lw(at.wrapping_add(0x808));
            c.g[A2][0] = c.lw(at.wrapping_add(0xc0c));
            c.g[V0][0] = c.lw(at.wrapping_sub(4));
            c.g[A3][0] = c.lw(at.wrapping_add(0x400));
            c.g[T0][0] = c.lw(at.wrapping_add(0x804));
            c.g[V1][0] = oc230_add(c, V1, V0);
            c.g[T1][0] = c.lw(at.wrapping_add(0xc08));
            c.g[A0][0] = oc230_add(c, A0, A3);
            c.g[A1][0] = oc230_add(c, A1, T0);
            let v = c.g[V1][0];
            c.sw(at, v);
            c.g[A2][0] = oc230_add(c, A2, T1);
            let v = c.g[A0][0];
            c.sw(at.wrapping_add(0x404), v);
            let v = c.g[A1][0];
            c.sw(at.wrapping_add(0x808), v);
            let v = c.g[A2][0];
            c.sw(at.wrapping_add(0xc0c), v);
            let more = (c.g[T3][0] as i64) >= 0;
            c.g[T2][0] = sx(at.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_4140);
            }
        }

        // Scatter by byte shift = 0, 8, 16, 24 (histogram shift / 8).
        c.cycles += 1;
        c.g[A3][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V1][0] = oc230_sll(c, A3, 5);
            c.g[V0][0] = sx(((r32(c, A3) as i32) >> 3) as u32);
            c.g[V1][0] = oc230_add(c, V1, V0);
            c.g[T1][0] = sx(r32(c, A3).wrapping_add(8));
            c.g[V1][0] = oc230_sll(c, V1, 2);
            let empty = (c.g[S2][0] as i64) <= 0;
            c.g[T0][0] = sx(hist.wrapping_add(r32(c, V1)));
            if !empty {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                loop {
                    // dst[h[(key >> shift) & 0xff]++] = *src++
                    c.cycles += 18;
                    let item = r32(c, A1);
                    c.g[V0][0] = c.lw(item);
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                    c.g[V0][0] = sx(r32(c, V0) >> (r32(c, A3) & 31)) & 0xff;
                    c.g[V0][0] = oc230_sll(c, V0, 2);
                    c.g[V0][0] = oc230_add(c, V0, T0);
                    let slot = r32(c, V0);
                    c.g[A0][0] = c.lw(slot);
                    c.g[V1][0] = c.g[A0][0];
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                    c.g[V1][0] = oc230_sll(c, V1, 3);
                    let v = c.g[A0][0];
                    c.sw(slot, v);
                    c.g[V1][0] = oc230_add(c, V1, S3);
                    let out = r32(c, V1);
                    c.ldl(V0, item.wrapping_add(7));
                    c.ldr(V0, item);
                    c.sdl(V0, out.wrapping_add(7));
                    c.sdr(V0, out);
                    let more = c.g[A2][0] != 0;
                    c.g[A1][0] = sx(item.wrapping_add(8));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_41b8);
                    }
                }
            }
            // Swap src and dst.
            c.cycles += 6;
            c.g[V1][0] = c.g[S3][0];
            c.g[A3][0] = c.g[T1][0];
            c.g[S3][0] = c.g[S1][0];
            c.g[V0][0] = ((c.g[A3][0] as i64) < 32) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = c.g[V1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_4190);
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S4][0];
        d::SPR_STREAM_NEXT_INDEX.call(c, 0x0036_4220)?;
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
// TexMan_ResetUsageTable (0x3666f8)

pub(crate) const SPR_STREAM_DOWNLOAD: Callee = Callee {
    addr: 0x0038_f738,
    f: ee::s12::SprStream_Download,
};

/// `InvalidDCache` (libkernel; a cross-shard call through the table).
pub(crate) const INVALID_DCACHE: u32 = 0x0042_47d8;

/// The texture usage table: 2000 8-byte entries.
pub(crate) const TEX_USAGE_TABLE: u32 = 0x0053_ab80;

pub(crate) const TEX_USAGE_TABLE_LAST: u32 = 0x0053_e9ff;

pub(crate) const TEX_USAGE_ENTRIES: i64 = 2000;

/// `gp`-relative: the scratchpad stream's free pointer (`0x7000_0000`-based;
/// the scratchpad ends at `0x7000_4000`).
pub(crate) const GP_SPR_FREE: u32 = 0x2a94;

pub(crate) const SPR_END: u32 = 0x7000_4000;

/// `void TexMan_ResetUsageTable(void)` (0x3666f8): sets all 2000 entries of
/// the texture usage table (0x53ab80) to `0xffffffff` words: invalidates its
/// data cache lines, fills the free scratchpad (from the stream's free
/// pointer to the end) with the pattern quadword, and DMAs that scratchpad
/// block over the table in chunks of `free / 8` entries
/// (`SprStream_Download`, waiting for each).
///
/// The pattern quadword is built on the stack and kept in s0; s1 = entries
/// done, s0 = chunk, s2 = 2000, s3 = the table.
pub fn tex_man_reset_usage_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xffff_ffff;
        let at = frame.at(c, 8);
        c.sd(at, c.g[V0][0]);
        c.g[A0][0] = 0x0054_0000;
        let at = frame.at(c, 0);
        c.sd(at, c.g[V0][0]);
        c.g[A1][0] = 0x0054_0000;
        frame.save(c, S0, 0x50);
        c.g[A0][0] = TEX_USAGE_TABLE as u64;
        frame.save(c, S1, 0x40);
        c.g[A1][0] = TEX_USAGE_TABLE_LAST as u64;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.restore(c, S0, 0);
        call_ptr(c, INVALID_DCACHE, 0x0036_6738)?;

        // Fill the free scratchpad with the pattern.
        let gp = r32(c, GP);
        c.g[V1][0] = Addr(gp).off(GP_SPR_FREE).lw(c);
        c.g[V0][0] = 0x7000_0000;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx(SPR_END.wrapping_sub(r32(c, V1)));
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0) >> 4);
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 7;
                let at = r32(c, V1);
                c.sq(S0, at);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_6760);
                }
            }
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(gp).off(GP_SPR_FREE).lw(c);

        // The free size in table entries per chunk.
        c.cycles += 2;
        c.g[V0][0] = 0x7000_0000;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[A1][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = SPR_END as u64;
            c.g[A1][0] = sx(SPR_END.wrapping_sub(r32(c, V1)));
        }
        c.cycles += 6;
        c.g[V0][0] = 0x0054_0000;
        c.g[S0][0] = sx(r32(c, A1) >> 3);
        c.g[S3][0] = TEX_USAGE_TABLE as u64;
        c.g[S1][0] = 0;
        c.g[S2][0] = TEX_USAGE_ENTRIES as u64;
        c.g[V1][0] = sx(r32(c, S2).wrapping_sub(r32(c, S1)));
        loop {
            c.cycles += 11;
            c.g[A0][0] = sx(r32(c, S1) << 3);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
            if c.g[V0][0] != 0 {
                // The last, shorter chunk.
                c.g[S0][0] = c.g[V1][0];
            }
            c.g[A2][0] = 2;
            c.g[A1][0] = sx(r32(c, S0) << 3);
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
            SPR_STREAM_DOWNLOAD.call(c, 0x0036_67d0)?;
            c.g[V0][0] = ((c.g[S1][0] as i64) < TEX_USAGE_ENTRIES) as u64;
            c.g[V1][0] = sx(r32(c, S2).wrapping_sub(r32(c, S1)));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_67b0);
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
// PositionModifier_GetMatrix (0x361940)

/// PositionModifier: non-zero when the matrix is stale.
pub(crate) const POSMOD_DIRTY: u32 = 0x44;

/// The matrix.
pub(crate) const POSMOD_MATRIX: u32 = 0x50;

/// Vtable slot that rebuilds the matrix.
pub(crate) const SLOT_POSMOD_UPDATE: u32 = 0x18;

/// `Matrix* PositionModifier::GetMatrix(PositionModifier* m)` (0x361940):
/// rebuilds the matrix through the virtual update (slot 0x18) when the
/// dirty word (+0x44) is set, and returns the matrix at +0x50.
pub fn position_modifier_get_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        set_reg(c, S0, reg(c, A0));
        let m = r32(c, S0);
        let dirty = Addr(m.wrapping_add(POSMOD_DIRTY)).lw(c);
        set_reg(c, V0, sx(m.wrapping_add(POSMOD_MATRIX)));
        if dirty != 0 {
            c.cycles += 6;
            let vt = Addr(m).lw(c);
            set_reg(c, V0, vt);
            let slot = VSlot::read(c, vt, SLOT_POSMOD_UPDATE);
            set_reg(c, V1, slot.func);
            set_reg(c, A0, slot.this(sx(m)));
            call_ptr(c, slot.func as u32, 0x0036_1970)?;
            set_reg(c, V0, sx(m.wrapping_add(POSMOD_MATRIX)));
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ RailModifier_UpdateBounds

/// `void RailModifier_UpdateBounds(RailModifier* self)` (0x35c040): if the
/// owner's object (`self+0x40 -> +0xc`) says it has bounds (virtual at
/// vtable +0x160), copy its bounding box (virtual at +0x168: min, max) to
/// `self+0x10` / `self+0x20`, grown by 100 units on x, y and z.
pub fn rail_modifier_update_bounds(c: &mut Cpu) {
    /// `RailModifier` fields.
    const OWNER: u32 = 0x40;
    const BOX_MIN: u32 = 0x10;
    const BOX_MAX: u32 = 0x20;
    /// The owner's object and its vtable slots `{delta: i16, method}`.
    const OWNER_OBJECT: u32 = 0xc;
    const VTABLE: u32 = 0xc;
    const HAS_BOUNDS: u32 = 0x160;
    const GET_BOUNDS: u32 = 0x168;
    /// 100.0, the margin.
    const MARGIN: u64 = 0x42c8_0000;
    /// `object->vtable[slot](object)`.
    fn call_object(c: &mut Cpu, this: u32, slot: u32, ret: u32) -> crate::call::Flow {
        let owner = Addr(this).off(OWNER).lw(c);
        c.g[V0][0] = owner;
        let object = Addr(owner as u32).off(OWNER_OBJECT).lw(c);
        c.g[A1][0] = object;
        let vt = Addr(object as u32).off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr(vt as u32).off(slot).lh(c);
        let method = Addr(vt as u32).off(slot + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = abi::addu(object, delta);
        call_ptr(c, method as u32, ret)
    }
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        call_object(c, this, HAS_BOUNDS, 0x0035_c06c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 26;
            call_object(c, this, GET_BOUNDS, 0x0035_c090)?;
            let bounds = r32(c, V0);
            c.lq(V1, bounds);
            c.g[AT][0] = MARGIN;
            c.f[0] = MARGIN as u32;
            c.sq(V1, this.wrapping_add(BOX_MIN));
            c.lq(A0, bounds.wrapping_add(0x10));
            // The margin vector (100, 100, 100, 0) on the stack.
            let pad = sp(c);
            Addr(pad).off(0x8).write_fpu(c, FpuF32(MARGIN as u32));
            c.sq(A0, this.wrapping_add(BOX_MAX));
            Addr(pad).write_fpu(c, FpuF32(MARGIN as u32));
            Addr(pad).off(0x4).write_fpu(c, FpuF32(MARGIN as u32));
            Addr(pad).off(0xc).sw(c, 0);
            c.lqc2(3, this.wrapping_add(BOX_MIN));
            c.lqc2(4, pad);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, this.wrapping_add(BOX_MIN));
            c.lqc2(3, this.wrapping_add(BOX_MAX));
            c.lqc2(4, pad);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, this.wrapping_add(BOX_MAX));
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void RailModifier_v03(RailModifier* self)` (0x35c4e0, railmodifier.cpp):
/// the rail modifier's per-frame update: recomputes its bounds (+0x10 min,
/// +0x20 max) with `RailModifier_UpdateBounds` and moves its entry (id at
/// +0x44) in the world's spatial hash from the old box (copied to the stack
/// first) to the new one: `SpatialHash_Update(World_GetNash(), 2, id,
/// &self->box, &old)`.
pub fn rail_modifier_v03(c: &mut Cpu) {
    const BOX: u32 = 0x10;
    const ID: u32 = 0x44;
    const UPDATE_BOUNDS: Callee = Callee {
        addr: 0x0035_c040,
        f: ee::s11::f_0035c040,
    };
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let this = r32(c, S0);
        c.g[S1][0] = sx(this.wrapping_add(BOX));
        // The old box, through v1/v0, to sp+0 / sp+0x10.
        c.lq(V1, this.wrapping_add(BOX));
        c.lq(V0, this.wrapping_add(BOX + 0x10));
        let old = frame.at(c, 0);
        c.sq(V1, old);
        c.sq(V0, old.wrapping_add(0x10));
        UPDATE_BOUNDS.call(c, 0x0035_c50c)?;
        call_ptr(c, 0x002d_1be0, 0x0035_c514)?; // World_GetNash
        c.g[A2][0] = Addr(r32(c, S0)).off(ID).lw(c);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_91e0, 0x0035_c52c)?; // SpatialHash_Update
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_GetMatrices (0x361098)

/// ModObj: the embedded transform modifier (`+0x14`, its vtable at `+0x20`),
/// the flags (`+0x26`, bit 0: the modifier is active) and the matrix array.
pub(crate) const MODOBJ_MODIFIER: u32 = 0x14;

pub(crate) const MODOBJ_MODIFIER_VT: u32 = 0x20;

pub(crate) const MODOBJ_FLAGS: u32 = 0x26;

pub(crate) const MODOBJ_MATRICES: u32 = 0x44;

/// The modifier's "update matrices" vtable slot.
pub(crate) const MODIFIER_UPDATE_SLOT: u32 = 0x198;

/// `Matrix* ModObj_GetMatrices(ModObj* self)` (0x361098): the object's
/// matrix array (`+0x44`), first letting the active transform modifier
/// update it (virtual slot 0x198 on the embedded modifier at `+0x14`).
pub fn mod_obj_get_matrices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_FLAGS)).lhu(c) & 1;
        c.g[A1][0] = sx(this.wrapping_add(MODOBJ_MODIFIER));
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_MODIFIER_VT)).lw(c);
            let vt = r32(c, V0);
            c.g[A0][0] = Addr(vt.wrapping_add(MODIFIER_UPDATE_SLOT)).lh(c);
            c.g[V1][0] = Addr(vt.wrapping_add(MODIFIER_UPDATE_SLOT + 4)).lw(c);
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0036_10cc)?;
        }
        c.cycles += 5;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_MATRICES)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VramHeap_NodesFromScratchpad (railmodifier.cpp).

/// `InvalidDCache` (libkernel; called through the table).
pub(crate) const INVALID_DCACHE_207: u32 = 0x0042_47d8;

/// `SprStream_Download` (0x38f738).
pub(crate) const SPR_STREAM_DOWNLOAD_207: Callee = Callee {
    addr: 0x0038_f738,
    f: ee::s12::SprStream_Download,
};

/// `void VramHeap_NodesFromScratchpad(VramHeap* self)` (0x365fa8): bring the
/// heap's node block back from the scratchpad: invalidate the data cache
/// over `self..self+0x1fef`, DMA 0x1ff0 bytes from the scratchpad stream to
/// `self` (flags 2: wait), then store `self` in the block's trailing link
/// word (`+0x1ff0`).
pub fn vram_heap_nodes_from_scratchpad(c: &mut Cpu) {
    const BLOCK: u32 = 0x1ff0;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(BLOCK - 1));
        call_ptr(c, INVALID_DCACHE_207, 0x0036_5fc0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = BLOCK as u64;
        c.g[A2][0] = 0x2;
        SPR_STREAM_DOWNLOAD_207.call(c, 0x0036_5fd0)?;
        let heap = r32(c, S0);
        c.sw(heap.wrapping_add(BLOCK), c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ VramHeap_NodesToScratchpad

/// `SprStream_Upload` (0x38f708).
pub(crate) const VRAM_SPR_STREAM_UPLOAD: Callee = Callee {
    addr: 0x0038_f708,
    f: ee::s12::f_0038f708,
};

/// `SyncDCache` (0x424698, cross-shard).
pub(crate) const VRAM_SYNC_DCACHE: u32 = 0x0042_4698;

/// The heap's node table: 0x1ff0 bytes from +0, and the word after it
/// (+0x1ff0) where the table's current address is kept.
pub(crate) const VRAM_NODES_SIZE: u32 = 0x1ff0;

/// `void VramHeap_NodesToScratchpad(VramHeap* self)` (0x365f68): writes the
/// node table back from the data cache and DMAs it to the scratchpad
/// stream buffer (waiting for the transfer); `self->nodes` = the
/// scratchpad copy.
pub fn vram_heap_nodes_to_scratchpad(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(VRAM_NODES_SIZE - 1));
        call_ptr(c, VRAM_SYNC_DCACHE, 0x0036_5f80)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = VRAM_NODES_SIZE as u64;
        c.g[A2][0] = 2;
        VRAM_SPR_STREAM_UPLOAD.call(c, 0x0036_5f90)?;
        let nodes = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(VRAM_NODES_SIZE)).sw(c, nodes);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UVScrollModifier_Save (0x35fd98)

/// The stream's `write(data, size)` vtable slot (delta at +8, fn at +0xc).
pub(crate) const UV_SCROLL_SAVE_WRITE_SLOT: u32 = 0x8;

/// Bytes of UVScrollModifier state saved.
pub(crate) const UV_SCROLL_SAVE_SIZE: u64 = 0x50;

/// `void UVScrollModifier_Save(UVScrollModifier* self, Stream* out)`
/// (0x35fd98): write the modifier's first 0x50 bytes with `out->write(self,
/// 0x50)` (virtual slot 8).
pub fn uv_scroll_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = UV_SCROLL_SAVE_SIZE;
        let out = r32(c, A3);
        c.g[V1][0] = Addr(out).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt.wrapping_add(UV_SCROLL_SAVE_WRITE_SLOT)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(UV_SCROLL_SAVE_WRITE_SLOT + 4)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(out.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_fdc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ModifierBlock::FreeParticles()` (0x3531d0): deletes every link of
/// the block's particle list (+0x10; each link's `next` at +0, its vtable at
/// +8) through its deleting destructor (`link->~Link(3)`, vtable +8).
pub fn modifier_block_free_particles(c: &mut Cpu) {
    const PARTICLES: u32 = 0x10;
    const LINK_NEXT: u32 = 0x0;
    const LINK_VTABLE: u32 = 0x8;
    const DELETE: u32 = 0x8;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let head = c.lw((c.g[A0][0] as u32).wrapping_add(PARTICLES));
        c.g[A0][0] = head;
        frame.restore(c, S0, 0x10);
        if head != 0 {
            c.cycles += 2;
            c.g[A2][0] = head;
            loop {
                c.cycles += 2;
                let link = c.g[A2][0] as u32;
                c.g[S0][0] = c.lw(link.wrapping_add(LINK_NEXT));
                if link != 0 {
                    c.cycles += 6;
                    let vt = c.lw(link.wrapping_add(LINK_VTABLE));
                    c.g[V0][0] = vt;
                    c.g[A1][0] = 0x3;
                    let delta = c.lh((vt as u32).wrapping_add(DELETE));
                    let dtor = c.lw((vt as u32).wrapping_add(DELETE + 4));
                    c.g[V1][0] = dtor;
                    c.g[A0][0] = sx(link.wrapping_add(delta as u32));
                    call_ptr(c, dtor as u32, 0x0035_3210)?;
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0];
                frame.restore(c, S0, 0x10);
                if c.g[A2][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_31f0);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_FreeTransform (0x3530d0)

/// ModifierBlock: the transform modifier at +0x0.
pub(crate) const MODIFIER_BLOCK_TRANSFORM: u32 = 0x0;

/// `void ModifierBlock_FreeTransform(ModifierBlock* self)` (0x3530d0): when
/// the block has a transform modifier, deletes it through its virtual
/// destructor (vtable +0x8, flags 3: destroy and free) and clears the slot.
pub fn modifier_block_free_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let block = c.g[S0][0] as u32;
        let transform = c.lw(block.wrapping_add(MODIFIER_BLOCK_TRANSFORM));
        c.g[A2][0] = transform;
        c.g[A1][0] = 0x3;
        if transform != 0 {
            c.cycles += 6;
            let vt = c.lw(transform as u32);
            c.g[V0][0] = vt;
            c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((vt as u32).wrapping_add(0xc));
            let dtor = c.g[V1][0] as u32;
            c.g[A0][0] = sx((transform as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, dtor, 0x0035_3100)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(MODIFIER_BLOCK_TRANSFORM),
                0,
            );
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_FreeBoost (0x353188).

/// Modifier block: `+0xc` the boost object (vtable at `+4`, slot 1 its
/// deleting destructor).
pub(crate) const MOD_BLOCK_BOOST: u32 = 0xc;

/// `void ModifierBlock_FreeBoost(ModifierBlock* self)` (0x353188): delete
/// the boost object, if any (virtual destructor, flags 3), and clear it.
pub fn modifier_block_free_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(r32(c, S0)).off(MOD_BLOCK_BOOST).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            let boost = r32(c, A2);
            c.g[V0][0] = Addr(boost).off(4).lw(c);
            let vt = r32(c, V0);
            c.g[A0][0] = Addr(vt).off(8).lh(c);
            c.g[V1][0] = Addr(vt).off(0xc).lw(c);
            c.g[A0][0] = sx(boost.wrapping_add(r32(c, A0)));
            let dtor = r32(c, V1);
            call_ptr(c, dtor, 0x0035_31b8)?;
            Addr(r32(c, S0)).off(MOD_BLOCK_BOOST).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SyncProcess_IdEquals (0x360840)

/// `SyncProcess`: its sync id (`+0x14`).
pub(crate) const OC300_SYNC_ID: u32 = 0x14;

/// Object: its vtable (`+0xc`); the vtable's "get id" slot (`+0x40`).
pub(crate) const OC300_OBJ_VTABLE: u32 = 0xc;

pub(crate) const OC300_VT_GET_ID: u32 = 0x40;

/// `bool SyncProcess_IdEquals(SyncProcess* self, Object* obj)` (0x360840):
/// whether `self`'s sync id equals the id `obj` reports through its
/// virtual "get id" method (vtable slot `+0x40`).
pub fn sync_process_id_equals(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let obj = c.g[A1][0];
        let vt = Addr(obj as u32).off(OC300_OBJ_VTABLE).lw(c);
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, OC300_VT_GET_ID);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0036_0864)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(OC300_SYNC_ID).lw(c);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] ^= c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_FreeTexFlip (0x353150)

/// `ModifierBlock`: the owned texture-flip modifier (`+0x8`).
pub(crate) const OC305_MODIFIER_BLOCK_TEX_FLIP: u32 = 0x8;

/// `__builtin_delete` (0x317e50).
pub(crate) const OC305_BUILTIN_DELETE: u32 = 0x0031_7e50;

/// `void ModifierBlock_FreeTexFlip(ModifierBlock* self)` (0x353150): delete
/// the block's texture-flip modifier (`+0x8`), if any, and clear the field.
pub fn modifier_block_free_tex_flip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(OC305_MODIFIER_BLOCK_TEX_FLIP).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, OC305_BUILTIN_DELETE, 0x0035_3174)?;
            Addr(r32(c, S0)).off(OC305_MODIFIER_BLOCK_TEX_FLIP).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------- ModifierBlock_FreeUVScroll

/// ModifierBlock: +0x4 the UV-scroll modifier (a plain heap block).
pub(crate) const MODIFIER_BLOCK_UV_SCROLL: u32 = 0x4;

/// `void ModifierBlock_FreeUVScroll(ModifierBlock* self)` (0x353118): when
/// the block has a UV-scroll modifier, free it (`__builtin_delete`) and
/// clear the slot.
pub fn modifier_block_free_uv_scroll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let block = abi::r32(c, S0);
        c.g[A0][0] = Addr(block.wrapping_add(MODIFIER_BLOCK_UV_SCROLL)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e50, 0x0035_313c)?;
            let block = abi::r32(c, S0);
            Addr(block.wrapping_add(MODIFIER_BLOCK_UV_SCROLL)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Rail modifier: the saved block (0x30 bytes at +0x10) and the instance
/// pointer (+0x40) written to the stream.
pub(crate) const RAIL_MODIFIER_BLOCK: u32 = 0x10;

pub(crate) const RAIL_MODIFIER_INSTANCE: u32 = 0x40;

/// Stream vtable slots (`{i16 delta, fn}`): write bytes (+0x8) and write a
/// pointer (+0x28).
pub(crate) const STREAM_WRITE_BYTES: u32 = 0x8;

pub(crate) const STREAM_WRITE_PTR: u32 = 0x28;

/// A `jalr` through the stream's vtable slot `slot` (`{i16 delta, fn}`):
/// `a0 = stream + delta`, target the slot's function, return to `ret`.
/// Leaves v1 = vtable, v0 = target, as the compiled sequence does.
pub(crate) fn oc308_stream_call(
    c: &mut Cpu,
    stream_reg: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    let stream = r32(c, stream_reg);
    c.g[V1][0] = c.lw(stream);
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V0][0] = c.lw(vt.wrapping_add(slot + 4));
    c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
    let target = c.g[V0][0] as u32;
    call_ptr(c, target, ret)
}

/// `void RailModifier_v02(RailModifier* self, cStream* stream)` (0x35c540):
/// save the rail modifier: its 0x30-byte block at +0x10 through
/// `stream->WriteBytes`, then its instance pointer (+0x40) through
/// `stream->WritePtr`.
pub fn rail_modifier_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x30;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(RAIL_MODIFIER_BLOCK));
        oc308_stream_call(c, S0, STREAM_WRITE_BYTES, 0x0035_c574)?;
        // Registers at the call: v1 = vtable, a1 = instance, a0 = stream + delta.
        let vt = c.lw(r32(c, S0));
        c.g[V1][0] = vt;
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(RAIL_MODIFIER_INSTANCE));
        c.g[A0][0] = c.lh((vt as u32).wrapping_add(STREAM_WRITE_PTR));
        c.g[V0][0] = c.lw((vt as u32).wrapping_add(STREAM_WRITE_PTR + 4));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0035_c58c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RailModifier_BuildRailNodes (0x35c0e8)

/// `cRailModifier + 0x34`: the starting rail node index (< 0: none).
pub(crate) const RM_BUILD_NODE: u32 = 0x34;

/// `cRailModifier + 0x40`: the rail object (matrix at +0x10, node table
/// header at +0x80, scale at +0x84).
pub(crate) const RM_BUILD_RAIL: u32 = 0x40;

/// `cRailModifier + 0x44`: the allocated 0x10-byte link.
pub(crate) const RM_BUILD_LINK: u32 = 0x44;

/// `cRailModifier + 0x50`: the built transform (4 rows).
pub(crate) const RM_BUILD_XFORM: u32 = 0x50;

pub(crate) const RM_BUILD_RAIL_MATRIX: u32 = 0x10;

pub(crate) const RM_BUILD_RAIL_TABLE: u32 = 0x80;

pub(crate) const RM_BUILD_RAIL_SCALE: u32 = 0x84;

/// `EA_MemAllocNamed` / `cMemMan_alloc` (0x317d70, through the table).
pub(crate) const RM_BUILD_MEM_ALLOC: u32 = 0x0031_7d70;

/// `lqc2 vf1..vf4` from `src`, then `sqc2 vf1..vf4` to `dst` (a 4x4 matrix
/// copy through VU0).
#[inline]
pub(crate) fn rm_build_copy_matrix(c: &mut Cpu, src: u32, dst: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, src.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, dst.wrapping_add(0x10 * i));
    }
}

/// vf12..vf15 = rows vf8..vf11 times the matrix vf4..vf7 (the compiled
/// matrix multiply).
#[inline]
pub(crate) fn rm_build_mul_matrix(c: &mut Cpu) {
    for row in 0..4u8 {
        let r = 8 + row;
        c.vu0().mula(0b1111, 4, Src::Bc(r, X));
        c.vu0().madda(0b1111, 5, Src::Bc(r, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(r, Z));
        c.vu0().madd(0b1111, 12 + row, 7, Src::Bc(r, W));
    }
}

/// `lqc2 vf4..vf7` from `a`, `lqc2 vf8..vf11` from `b`, multiply, store
/// vf12..vf15 at `out`, then copy `out` -> `tmp` -> `b`.
#[inline]
pub(crate) fn rm_build_concat(c: &mut Cpu, a: u32, b: u32, out: u32, tmp: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, a.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, b.wrapping_add(0x10 * i));
    }
    rm_build_mul_matrix(c);
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, out.wrapping_add(0x10 * i));
    }
    rm_build_copy_matrix(c, out, tmp);
    rm_build_copy_matrix(c, tmp, b);
}

/// `void cRailModifier_buildXform(cRailModifier* self)` (0x35c0e8): builds
/// the modifier's transform at +0x50. Without a start node it is the rail's
/// own matrix; otherwise the start node's matrix (translation scaled by the
/// rail's scale), concatenated with each following node's matrix (same
/// scaling) down the node chain (`next == -1` ends it), then with the rail's
/// matrix. The 3x3 part is then transposed and the translation replaced by
/// `-t` through the transposed rows (an inverse rigid transform), and a
/// 0x10-byte link `{.., .., 2, self}` is allocated into +0x44.
pub fn rail_modifier_build_rail_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save_ra(c, 0x110);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(this.wrapping_add(RM_BUILD_RAIL)).lw(c);
        c.g[A0][0] = Addr(this.wrapping_add(RM_BUILD_NODE)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(RM_BUILD_RAIL_TABLE)).lw(c);
        c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        let s = sp(c);
        if (c.g[A0][0] as i64) < 0 {
            // No node: the rail's matrix.
            c.cycles += 13;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RM_BUILD_RAIL_MATRIX));
            c.g[V0][0] = sx(this.wrapping_add(RM_BUILD_XFORM));
            let (src, dst) = (c.g[V1][0] as u32, c.g[V0][0] as u32);
            rm_build_copy_matrix(c, src, dst);
            c.g[T1][0] = c.g[V0][0];
            c.g[T7][0] = 0x0049_0000;
        } else {
            c.cycles += 46;
            // sp+0x00: the scale vector {s, s, s, 1}.
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(RM_BUILD_RAIL_SCALE));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.swc1(0, s.wrapping_add(0x8));
            c.g[V1][0] = sx(this.wrapping_add(RM_BUILD_XFORM));
            c.swc1(1, s.wrapping_add(0xc));
            c.swc1(0, s);
            c.swc1(0, s.wrapping_add(0x4));
            let node = c.g[A0][0] as u32;
            c.g[V0][0] = Addr(node.wrapping_add(0xc)).lw(c);
            let (src, dst) = (c.g[V0][0] as u32, c.g[V1][0] as u32);
            rm_build_copy_matrix(c, src, dst);
            // Row 3 (translation) *= scale.
            c.lq(2, this.wrapping_add(0x80));
            c.g[T1][0] = c.g[V1][0];
            c.lwc1(0, s);
            c.g[T4][0] = sx(s.wrapping_add(0x50));
            c.sq(2, s.wrapping_add(0x20));
            c.g[T6][0] = sx(s.wrapping_add(0x10));
            c.lwc1(4, s.wrapping_add(0x4));
            c.g[V1][0] = u64::MAX;
            c.lwc1(1, s.wrapping_add(0x20));
            c.lwc1(2, s.wrapping_add(0x24));
            c.mul_s(1, 1, 0);
            c.lwc1(3, s.wrapping_add(0x28));
            c.mul_s(2, 2, 4);
            c.lwc1(5, s.wrapping_add(0x8));
            c.lwc1(0, s.wrapping_add(0x2c));
            c.lwc1(4, s.wrapping_add(0xc));
            c.mul_s(3, 3, 5);
            c.swc1(1, s.wrapping_add(0x10));
            c.mul_s(0, 0, 4);
            c.swc1(2, s.wrapping_add(0x14));
            c.swc1(3, s.wrapping_add(0x18));
            c.swc1(0, s.wrapping_add(0x1c));
            c.lq(2, s.wrapping_add(0x10));
            c.sq(2, this.wrapping_add(0x80));
            c.g[A0][0] = Addr(node).lw(c);
            c.g[T7][0] = 0x0049_0000;
            if c.g[A0][0] != c.g[V1][0] {
                c.cycles += 6;
                c.g[T0][0] = c.g[T1][0];
                c.g[A3][0] = sx(s.wrapping_add(0x30));
                c.g[A2][0] = sx(s.wrapping_add(0xd0));
                c.g[T3][0] = sx(s.wrapping_add(0x70));
                c.g[T5][0] = u64::MAX;
                c.g[A1][0] = sx((c.g[A0][0] as u32) << 4);
                loop {
                    // Next node: its matrix (sp+0x70) with the translation
                    // scaled (sp+0xa0), then xform = node * xform.
                    c.cycles += 78;
                    c.lwc1(6, s);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.lwc1(7, s.wrapping_add(0x4));
                    let node = c.g[A1][0] as u32;
                    c.g[A0][0] = Addr(node.wrapping_add(0xc)).lw(c);
                    c.lwc1(5, s.wrapping_add(0x8));
                    let m = c.g[A0][0] as u32;
                    c.lq(3, m);
                    c.lwc1(4, s.wrapping_add(0xc));
                    c.sq(3, s.wrapping_add(0xb0));
                    c.sq(3, s.wrapping_add(0x70));
                    c.lq(2, m.wrapping_add(0x10));
                    c.sq(2, s.wrapping_add(0xb0));
                    c.sq(2, s.wrapping_add(0x80));
                    c.lq(3, m.wrapping_add(0x20));
                    c.sq(3, s.wrapping_add(0xb0));
                    c.sq(3, s.wrapping_add(0x90));
                    c.lq(2, m.wrapping_add(0x30));
                    c.sq(2, s.wrapping_add(0xc0));
                    c.lwc1(1, s.wrapping_add(0xc0));
                    c.lwc1(2, s.wrapping_add(0xc4));
                    c.lwc1(3, s.wrapping_add(0xc8));
                    c.mul_s(1, 1, 6);
                    c.lwc1(0, s.wrapping_add(0xcc));
                    c.mul_s(2, 2, 7);
                    c.mul_s(3, 3, 5);
                    c.mul_s(0, 0, 4);
                    c.swc1(1, s.wrapping_add(0xb0));
                    c.swc1(2, s.wrapping_add(0xb4));
                    c.swc1(3, s.wrapping_add(0xb8));
                    c.swc1(0, s.wrapping_add(0xbc));
                    c.lq(2, s.wrapping_add(0xb0));
                    c.sq(2, s.wrapping_add(0xa0));
                    let (t3, t0, a2, a3) = (
                        c.g[T3][0] as u32,
                        c.g[T0][0] as u32,
                        c.g[A2][0] as u32,
                        c.g[A3][0] as u32,
                    );
                    rm_build_concat(c, t3, t0, a2, a3);
                    c.g[A0][0] = Addr(node).lw(c);
                    let more = c.g[A0][0] != c.g[T5][0];
                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 4);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_c210);
                    }
                }
            }
            // xform = rail matrix * xform.
            c.cycles += 47;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(RM_BUILD_RAIL)).lw(c);
            c.g[V1][0] = c.g[T6][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(RM_BUILD_RAIL_MATRIX));
            let (v0, t1, t4, v1) = (
                c.g[V0][0] as u32,
                c.g[T1][0] as u32,
                c.g[T4][0] as u32,
                c.g[V1][0] as u32,
            );
            rm_build_concat(c, v0, t1, t4, v1);
        }
        // Transpose the 3x3 part.
        c.cycles += 55;
        let this = c.g[S0][0] as u32;
        let s = sp(c);
        let f = |o: u32| this.wrapping_add(o);
        c.lwc1(3, f(0x54));
        c.g[A1][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x15a0));
        c.lwc1(2, f(0x60));
        c.g[A0][0] = 0x10;
        c.swc1(3, f(0x60));
        c.g[A2][0] = 0x2000_0000;
        c.lwc1(3, f(0x58));
        c.g[A3][0] = 0;
        c.lwc1(1, f(0x70));
        c.swc1(3, f(0x70));
        c.lq(2, f(0x80));
        c.swc1(2, f(0x54));
        c.lwc1(3, f(0x68));
        c.lwc1(0, f(0x74));
        c.sq(2, s.wrapping_add(0x10));
        c.swc1(0, f(0x68));
        c.swc1(1, f(0x58));
        c.swc1(3, f(0x74));
        // Translation: -t (sp+0x20), w = 0, through the transposed rows.
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(1, 0);
        c.mfc1(2, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lq(2, s.wrapping_add(0x20));
        c.sq(2, s);
        Addr(s.wrapping_add(0xc)).sw(c, 0);
        c.lqc2(8, s);
        let t1 = c.g[T1][0] as u32;
        for i in 0..4u32 {
            c.lqc2(4 + i as usize, t1.wrapping_add(0x10 * i));
        }
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, s.wrapping_add(0x20));
        c.lq(3, s.wrapping_add(0x20));
        c.sq(3, s);
        c.swc1(1, s.wrapping_add(0xc));
        c.sq(3, s.wrapping_add(0x10));
        c.lq(2, s);
        c.sq(2, f(0x80));
        // The link: EA_MemAllocNamed(0x10, "...", 0x20000000, 0).
        call_ptr(c, RM_BUILD_MEM_ALLOC, 0x0035_c4c0)?;
        c.g[V1][0] = 0x2;
        let (link, owner, kind) = (c.g[V0][0], c.g[S0][0], c.g[V1][0]);
        Addr((link as u32).wrapping_add(0xc)).sw(c, owner);
        Addr((link as u32).wrapping_add(0x8)).sw(c, kind);
        Addr((owner as u32).wrapping_add(RM_BUILD_LINK)).sw(c, link);
        frame.restore(c, S0, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Object_v01 (0x361038)

/// The vtable `Object_v01` installs at `+0xc` before chaining (`D_00490e80`).
pub(crate) const OBJECT_V01_VTABLE_ADDR: u32 = 0x0049_0e80;

/// The object's vtable pointer.
pub(crate) const OBJECT_V01_VTABLE: u32 = 0xc;

pub(crate) const OBJECT_V01_MOD_OBJ_DTOR: Callee = Callee {
    addr: 0x0035_53c0,
    f: ee::s11::ModObj_dtor,
};

/// `Object* Object_v01(Object* self, int flags)` (0x361038, the
/// `railmodifier.cpp` object's virtual destructor): reinstall its own vtable
/// (0x490e80) and chain to `ModObj_dtor(self, flags)` (flags passed
/// through untouched in a1). Leaves v1 = self.
pub fn object_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OBJECT_V01_VTABLE_ADDR);
        c.sw(r32(c, V1).wrapping_add(OBJECT_V01_VTABLE), c.g[V0][0]);
        OBJECT_V01_MOD_OBJ_DTOR.call(c, 0x0036_1054)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IntersectInfo_dtor_2 (0x361290)

/// The `cMeshAnimFrame` vtable-instance pointer written at `+0x84`
/// (`D_00490AF0`).
pub(crate) const OC320_MESH_ANIM_FRAME_VT: u32 = 0x0049_0af0;

/// `void cMeshAnimFrame::~cMeshAnimFrame(this, int flags)` (0x361290): the
/// deleting destructor variant: install the base vtable pointer at `+0x84`,
/// and when `flags & 1` is set, `operator delete(this)`.
pub fn intersect_info_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(OC320_MESH_ANIM_FRAME_VT);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let plain = c.g[A1][0] == 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x84), c.g[V0][0]);
        if !plain {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0036_12b4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IntersectInfo_ResetMatrix (0x3612c0)

#[cfg(test)]
/// The identity matrix copied into every `IntersectInfo` (`D_004FF1A0`).
pub(crate) const IIRM_IDENTITY: u32 = 0x004f_f1a0;

#[cfg(test)]
/// `IntersectInfo` matrix offset.
pub(crate) const IIRM_MATRIX: u32 = 0x90;

/// `void IntersectInfo_ResetMatrix(IntersectInfo* self)` (0x3612c0): copy the
/// identity quadwords at `D_004FF1A0` into `self->matrix` (VU0 macro-mode
/// `lqc2`/`sqc2`, vf1..vf4).
pub fn intersect_info_reset_matrix(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x50_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x90));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let src = c.g[V0][0] as u32;
    let dst = c.g[A0][0] as u32;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Particle_v01 (0x361d50)

/// The particle node's base destructor (0x3553c0), cross-shard.
pub(crate) const PARTICLE_BASE_DTOR: u32 = 0x0035_53c0;

/// `cParticleNode::~cParticleNode(this)` (0x361d50): install the
/// `cParticleNode` vtable and run the base destructor.
pub fn particle_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x11a0));
        c.sw(r32(c, V1).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, PARTICLE_BASE_DTOR, 0x0036_1d6c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;

    use crate::difftest::events::schedule;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as RET;

    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::generic;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_12;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;

    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_8;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::camera::cameracontroller::tests::heap_block;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_8;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_9;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_7;

    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;

    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_10;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2 as vtable_3;
    pub(crate) use crate::game::object::modifierblock::tests::RETURNS_ZERO;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::slot;
    pub(crate) use crate::game::render::particle::tests::vt;
    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_6;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::visualfx::worldlightman::tests::f;
    pub(crate) use crate::game::visualfx::worldlightman::tests::pick;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_5;
    use ssx3_ee as ee;

    #[test]
    fn uv_scroll_modifier_ctor_matches() {
        check(
            ssx3_ee::s12::f_0035f6e8,
            uv_scroll_modifier_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x102b ^ seed as u64);
                scramble(c, &mut r);
                let desc = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = desc as u64;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
                c.bus.write32(DATA + 0x300 + 0x10, r.next_u32());
                for off in [4u32, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
                    c.bus.write32(desc + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn uv_scroll_modifier_set_param_matches() {
        check(
            ssx3_ee::s12::f_0035fb30,
            uv_scroll_modifier_set_param,
            300,
            &|c, seed| {
                let mut r = Rng(0x102c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let p = match seed % 8 {
                    0 => 0xc8,
                    1 => 0xc9,
                    2 => 0xca,
                    3 => 0xcb,
                    4 => 0xcc,
                    5 => 0xcd,
                    _ => 0xc0 + (seed % 4),
                };
                c.g[A1][0] = p as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn ava_spline_modifier_v01_matches() {
        check(
            ssx3_ee::s12::f_00361aa8,
            ava_spline_modifier_v01,
            40,
            &|c, seed| {
                let mut r = Rng(0x361a_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() & !1) as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn anim_delta_dtor_matches() {
        check(ssx3_ee::s12::f_00361de0, anim_delta_dtor, 40, &|c, seed| {
            let mut r = Rng(0x361d_e000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            c.bus.write32(DATA + 0x44, 0);
        });
    }

    /// A matrix table of `count` records (the man at DATA, the new record
    /// at DATA+0x10000). Record `hit` has the new record's key part; about
    /// half of the others have it with one float changed.
    pub(crate) fn matrix_table(c: &mut Cpu, r: &mut Rng, count: i32, hit: Option<u32>) {
        let man = DATA;
        let new = DATA + 0x1_0000;
        fill(c, r, new, matrix::SIZE / 4);
        c.g[A0][0] = sx(man);
        c.g[A1][0] = sx(new);
        let base = man + matrix::BASE;
        c.bus.write32(base + matrix::COUNT, count as u32);
        c.bus.write32(base + matrix::FOUND, r.next_u32());
        for i in 0..count.max(0) as u32 {
            let rec = man + matrix::RECORDS + i * matrix::SIZE;
            fill(c, r, rec, matrix::SIZE / 4);
            if hit == Some(i) || r.next_u32() % 2 == 0 {
                for w in (0xe0..matrix::SIZE).step_by(4) {
                    let v = c.bus.read32(new + w);
                    c.bus.write32(rec + w, v);
                }
                if hit != Some(i) {
                    let w = 0xe0 + 4 * (r.next_u32() % 40);
                    let v = c.bus.read32(rec + w) ^ 1 << (r.next_u32() % 31);
                    c.bus.write32(rec + w, v);
                }
            }
        }
    }

    #[test]
    fn defer_man_intern_matrix_record_matches() {
        check_lohi(
            ssx3_ee::s12::f_003645b8__generated,
            defer_man_intern_matrix_record,
            300,
            &|c, seed| {
                let mut r = Rng(0x3645_b800 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                // 0 and -1: no search (-1 appends over the table's own header).
                let count = match seed % 8 {
                    0 => 0,
                    1 => -1,
                    k => k as i32 - 1,
                };
                let hit = if count > 0 && seed % 3 != 0 {
                    Some(r.next_u32() % count as u32)
                } else {
                    None
                };
                matrix_table(c, &mut r, count, hit);
            },
        );
        check_events(
            ssx3_ee::s12::f_003645b8__generated,
            defer_man_intern_matrix_record,
            90,
            &|c, seed| {
                let mut r = Rng(0x3645_e800 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let events = schedule(c, &mut r, seed, 0x0036_45b8);
                let count = 3 + r.next_u32() % 4;
                let hit = if seed % 4 == 0 { None } else { Some(count - 1) };
                matrix_table(c, &mut r, count as i32, hit);
                events
            },
        );
    }

    // ------------------------------------------------------------ UV scroll

    #[test]
    fn uv_scroll_modifier_update_matches() {
        check(
            ee::s12::f_0035f7d0__generated,
            uv_scroll_modifier_update,
            400,
            &|c, seed| {
                let mut r = Rng(0x0005_c011 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let clock = DATA + 0x400;
                c.bus.write32(GAME_GP + 0x2a74, clock);
                // The generated body is the reference at the game's own rate;
                // the per-tick velocity conversion only matches at 60.
                c.bus.write32(clock + 0x10, 60);
                c.bus.write32(GAME_GP - 0x29cc, f(1.0));
                // 4-aligned, sometimes not 8-aligned (the unaligned velocity store)
                let m = DATA + 4 * (seed % 2);
                fill(c, &mut r, m, 0x12);
                let cycle = pick(&mut r, &[0.0f32, 1.0, 0.5, 2.0, -1.0]);
                c.bus.write32(m, pick(&mut r, &[0u32, 2, 6, 6, 1]));
                c.bus.write32(m + 0x8, f(cycle));
                let time =
                    pick(&mut r, &[0.0f32, 0.99, 0.3, 0.7, 1.5, 0.45]) * cycle.abs().max(0.5);
                c.bus.write32(m + 0x4, f(time));
                c.bus
                    .write32(m + 0xc, f(pick(&mut r, &[0.0f32, 0.5, -1.0, 3.0, 0.01])));
                c.bus.write32(m + 0x10, r.f32(2.0));
                c.bus.write32(
                    m + 0x14,
                    f(pick(&mut r, &[0.0f32, 0.3, -0.3, 2.5, -2.5, 1.0])),
                );
                c.bus.write32(m + 0x30, r.f32(1.5));
                c.bus.write32(m + 0x34, r.f32(1.5));
                c.bus.write32(m + 0x38, r.f32(0.6));
                c.bus.write32(m + 0x3c, r.f32(0.6));
                c.bus.write32(m + 0x40, (seed % 3 != 0) as u32);
                c.g[A0][0] = sx(m);
            },
        );
    }

    /// The scroll velocity is a UV offset per 60 Hz frame: half a second of
    /// ticks moves the same offset at 60, 120 and 240 Hz (the time step already
    /// follows the rate).
    #[test]
    fn uv_scroll_speed_follows_the_rate() {
        let offset = |hz: u32, mode: u32, du: f32| -> f32 {
            let mut c = crate::difftest::fresh();
            c.g[GP][0] = GAME_GP as u64;
            let clock = DATA + 0x400;
            c.bus.write32(GAME_GP + 0x2a74, clock);
            c.bus.write32(clock + 0x10, hz);
            c.bus.write32(GAME_GP - 0x29cc, 1.0f32.to_bits());
            let m = DATA + 0x800;
            for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x30, 0x34, 0x38, 0x3c, 0x40] {
                c.bus.write32(m + off, 0);
            }
            c.bus.write32(m + 0x8, 10.0f32.to_bits()); // cycle: never ends
            c.bus.write32(m + 0x0, mode);
            c.bus.write32(m + 0x38, du.to_bits());
            c.bus.write32(m + 0x3c, (du * 2.0).to_bits());
            c.bus.write32(m + 0x40, 1); // running
            crate::tick::set_for_test(&mut c, hz);
            let ticks = hz / 2; // half a second
            for _ in 0..ticks {
                c.g[A0][0] = m as u64;
                uv_scroll_modifier_update(&mut c);
                assert_eq!(c.pc, RET, "{hz}: did not return");
            }
            let u = f32::from_bits(c.bus.read32(m + 0x30));
            let v = f32::from_bits(c.bus.read32(m + 0x34));
            assert!((v - 2.0 * u).abs() < 1e-3, "{hz}: v {v} vs u {u}");
            u
        };
        let u60 = offset(60, 0, 0.01);
        assert!((u60 - 0.3).abs() < 1e-4, "60 Hz: {u60}");
        for hz in [120u32, 240] {
            let u = offset(hz, 0, 0.01);
            assert!((u - u60).abs() < 1e-5, "{hz} Hz: {u} vs {u60}");
        }
        // The mode-6 ramp (a triangular speed) too.
        let r60 = offset(60, 6, 0.01);
        for hz in [120u32, 240] {
            let r = offset(hz, 6, 0.01);
            assert!((r - r60).abs() < 1e-3, "ramp {hz} Hz: {r} vs {r60}");
        }
    }

    #[test]
    fn mod_obj_v03_matches() {
        covered(
            ee::s12::f_00360da0__generated,
            mod_obj_v03,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x36_0da0, 1);
                let block = DATA;
                let node = DATA + 0x3100;
                slot(c, 0x190, if seed % 3 == 0 { RETURNS_ZERO } else { FAKE });
                c.bus.write32(node, vt());
                c.bus.write32(block + 0xc, node);
                c.g[A0][0] = sx(block);
            },
        );
    }

    /// Four halfwords (one scissor key).
    pub(crate) fn put_half4(c: &mut Cpu, at: u32, v: [u16; 4]) {
        for (i, x) in v.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *x);
        }
    }

    #[test]
    fn world_obj_update_matches() {
        covered(
            ee::s12::f_003608e8__generated,
            world_obj_update,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3608, 1);
                let this = DATA;
                c.bus.write32(this + 0xc, DATA + 0x100);
                c.bus
                    .write16(DATA + 0x178, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(DATA + 0x17c, FAKE);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn defer_man_set_scissor_matches() {
        check(
            ee::s12::f_00364b88__generated,
            defer_man_set_scissor,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x364b);
                let table = DATA;
                let key = DATA + 0x10000;
                let (count_addr, keys) = (table + 0x6aed0, table + 0x6aed4);
                let k = [
                    (r.next_u32() % 4) as u16,
                    (r.next_u32() % 4) as u16,
                    (r.next_u32() % 4) as u16,
                    (r.next_u32() % 4) as u16,
                ];
                put_half4(c, key, k);
                let mismatch = |v: [u16; 4], field: usize| {
                    let mut w = v;
                    w[field] ^= 1;
                    w
                };
                let (count, entries): (u32, Vec<[u16; 4]>) = match seed % 6 {
                    0 => (0, Vec::new()),
                    1 => (1, vec![k]),
                    2 => (3, vec![mismatch(k, 0), k, mismatch(k, 3)]),
                    3 => (2, vec![mismatch(k, 0), mismatch(k, 1)]),
                    4 => (2, vec![mismatch(k, 2), mismatch(k, 3)]),
                    _ => (1, vec![mismatch(k, 1)]),
                };
                for (i, e) in entries.iter().enumerate() {
                    put_half4(c, keys + 8 * i as u32, *e);
                }
                c.bus.write32(count_addr, count);
                c.g[A0][0] = sx(table);
                c.g[A1][0] = sx(key);
            },
        );
    }

    #[test]
    fn sync_process_id_less_equal_matches() {
        check(
            ssx3_ee::s12::f_00360800__generated,
            sync_process_id_less_equal,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3608_00, 1);
                let obj = DATA;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj.wrapping_add(0x14), r.next_u32());
                // The object's vtable: slot 8 (`+0x40`) with no `this` adjustment.
                c.bus.write32(obj + 0xc, DATA + 0x200);
                c.bus.write16(DATA + 0x200 + 0x40, 0);
                c.bus.write32(DATA + 0x200 + 0x44, FAKE);
            },
        );
    }

    #[test]
    fn defer_man_compute_sort_key_matches() {
        check(
            ssx3_ee::s12::f_00364240__generated,
            defer_man_compute_sort_key,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3642_40, 0);
                c.pc = 0x0036_4240;
                // The category switch is a jump table in .rodata.
                let mut table = [
                    0x0036_4310,
                    0x0036_42f0,
                    0x0036_42e8,
                    0x0036_42f8,
                    0x0036_4308,
                    0x0036_42e0,
                    0x0036_4300,
                    0x0036_4300,
                    0x0036_4308,
                ];
                // sel: 0..3 (one per seed class); category: in range, past the table
                // (the default class), and one seed class where the table holds a
                // value that is not a case block.
                let category = if seed % 16 == 0 {
                    table[0] = 0xdead_beef;
                    0
                } else if seed % 8 == 0 {
                    9 + seed % 7
                } else {
                    seed % 9
                };
                put_words(c, DEFER_CLASS_TABLE, &table);
                let sel = seed % 4;
                let e = DATA;
                c.g[A0][0] = sx(e);
                c.bus.write32(e, (category << 6) | (r.next_u32() & !0x3c0));
                c.bus.write32(e + 4, sel | (r.next_u32() & !3));
                c.bus.write32(e + 8, r.next_u32());
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn spline_modifier_v18_matches() {
        covered(
            ssx3_ee::s12::f_00361b90__generated,
            spline_modifier_v18,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b90, 1);
                let selfp = DATA;
                c.g[A0][0] = selfp as u64;
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(selfp, vt);
                c.bus.write32(selfp + 0x58, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn roller_modifier_dtor_matches() {
        dtor_case(
            ee::s12::f_00360b00__generated,
            roller_modifier_dtor,
            0x360b00,
        );
    }

    #[test]
    fn boost_modifier_base_dtor_matches() {
        dtor_case(
            ee::s12::f_00360c28__generated,
            boost_modifier_base_dtor,
            0x360c28,
        );
    }

    #[test]
    fn intersect_info_dtor_matches() {
        dtor_case(
            ee::s12::f_003611d8__generated,
            intersect_info_dtor,
            0x3611d8,
        );
    }

    #[test]
    fn intersect_info_dtor_3_matches() {
        dtor_case(
            ee::s12::f_00361340__generated,
            intersect_info_dtor_3,
            0x361340,
        );
    }

    #[test]
    fn boost_modifier_v01_matches() {
        dtor_case(ee::s12::f_00361488__generated, boost_modifier_v01, 0x361488);
    }

    #[test]
    fn tex_flip_modifier_set_frame_matches() {
        check(
            ee::s12::f_0035f378__generated,
            tex_flip_modifier_set_frame,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x35f378, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 7) as u64;
                // The frame count at +0x20 and no textures in the apply loop.
                c.bus.write32(DATA + 0x20, seed % 5);
                c.bus.write32(DATA + 0x1c, 0);
            },
        );
    }

    /// The shared destructor fixture: half the seeds free a null object (the
    /// `operator delete` path stays off the heap manager) and half keep a real
    /// pointer with the flag cleared.
    pub(crate) fn destructor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        if seed % 2 == 1 {
            c.g[A0][0] = 0;
            c.g[A1][0] = 1;
        } else {
            c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
            c.g[A1][0] = sx(r.next_u32() & !1);
        }
    }

    #[test]
    fn position_modifier_v01_matches() {
        check(
            ee::s12::f_003618c8__generated,
            position_modifier_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3618_c8),
        );
    }

    #[test]
    fn parent_modifier_v01_matches() {
        check(
            ee::s12::f_00361988__generated,
            parent_modifier_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3619_88),
        );
    }

    #[test]
    fn magnet_modifier_v01_matches() {
        check(
            ee::s12::f_003619e0__generated,
            magnet_modifier_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3619_e0),
        );
    }

    #[test]
    fn roller_modifier_v01_matches() {
        check(
            ee::s12::f_00361c78__generated,
            roller_modifier_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x361c_78),
        );
    }

    #[test]
    fn intc_table_vu1_handler_matches() {
        check(
            ee::s12::f_003623a8__generated,
            intc_table_vu1_handler,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x3623_a8, 2);
                be_library(c);
            },
        );
    }

    #[test]
    fn roller_modifier_save_body_matches() {
        check(
            ee::s11::f_0035da00__generated,
            roller_modifier_save_body,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x35da_00, 8);
            },
        );
    }

    #[test]
    fn roller_modifier_load_body_matches() {
        check(
            ee::s11::f_0035da38__generated,
            roller_modifier_load_body,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x35da_38, 0x10);
            },
        );
    }

    #[test]
    fn floating_v45_matches() {
        check(
            ee::s12::f_00361778__generated,
            floating_v45,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3617_78, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                let bound = seed % 3 != 0;
                c.bus
                    .write32(DATA + 0x100 + 0x1c, if bound { DATA + 0x300 } else { 0 });
                c.bus.write32(DATA + 0x100 + 0x78, r.next_u32());
                fill(c, &mut r, DATA + 0x300, 0x40);
                // The bound block's first word is its wrapped object: no object,
                // no virtual call.
                c.bus.write32(DATA + 0x300, 0);
            },
        );
    }

    #[test]
    fn intc_table_remove_vif_vu_handlers_matches() {
        check(
            ee::s12::f_00361e80__generated,
            intc_table_remove_vif_vu_handlers,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361e_80, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn roller_modifier_translate_matches() {
        check(
            ssx3_ee::s11::f_0035ed90__generated,
            roller_modifier_translate,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35ed_90, 1);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                for i in 0..0x30u32 / 4 {
                    c.bus.write32(DATA + 0x30 + i * 4, r.edge_f32());
                    c.bus.write32(DATA + 0x80 + i * 4, r.edge_f32());
                    c.bus.write32(DATA + 0x800 + i * 4, r.edge_f32());
                }
                for reg in 1..32 {
                    let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
                    c.bus.plat.set_vf(reg, v);
                }
            },
        );
    }

    #[test]
    fn uv_scroll_modifier_ctor_2_matches() {
        covered(
            ee::s12::f_0035f788__generated,
            uv_scroll_modifier_ctor_2,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_f788, 1);
                let obj = DATA + 0x1000;
                let stream = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    #[test]
    fn magnet_modifier_v18_matches() {
        covered(
            ee::s12::f_00361a60__generated,
            magnet_modifier_v18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_1a60, 1);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj + 0x50, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3617b8 Floating_v27.

    #[test]
    fn floating_v27_matches() {
        check(
            ee::s12::f_003617b8__generated,
            floating_v27,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3617_b8, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write16(this + 0x12, (r.next_u32() & 1) as u16);
                c.bus.write32(this + 0x74, r.next_u32());
                let vt = vtable(c, &mut r, &[0x198]);
                c.bus.write32(this + 0xc, vt);
            },
        );
    }

    // ------------------------------------------------------------------
    // Interrupts and textures.

    #[test]
    fn intc_table_install_vif_vu_handlers_matches() {
        check(
            ee::s12::f_00361e30__generated,
            intc_table_install_vif_vu_handlers,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x361e_30, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn modifier_list_node_dtor_matches() {
        check(
            ee::s12::f_00360c98__generated,
            modifier_list_node_dtor,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x360c_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn rail_modifier_list_node_dtor_matches() {
        check(
            ee::s12::f_00360d40__generated,
            rail_modifier_list_node_dtor,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x360d_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn intc_table_remove_handler_matches() {
        check(
            ee::s12::f_00362008__generated,
            intc_table_remove_handler,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3620_08, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                for i in 0..8 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Graphics.

    #[test]
    fn anim_teeter_dtor_matches() {
        check(
            ee::s12::f_00361400__generated,
            anim_teeter_dtor,
            SEEDS_6,
            &|c, seed| {
                let _ = common(c, seed, 0x361400, 2);
                alloc_ready_2(c);
                let this = heap_block(c, 0x100);
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn floating_v29_matches() {
        check(
            ee::s12::f_00361800__generated,
            floating_v29,
            SEEDS_6,
            &|c, seed| {
                let mut r = common(c, seed, 0x361800, 1);
                let this = DATA + 0x100;
                let vt = DATA + 0x300;
                clear(c, this, 0x100);
                clear(c, vt, 0x200);
                c.bus.write16(this + 0x12, (seed % 2) as u16);
                c.bus.write32(this + 0xc, vt);
                vtable_2(c, vt, &[(0x198, 0, FAKE)]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64 & 0x3f;
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 300;

    pub(crate) fn rng(tag: u32, seed: u32) -> Rng {
        Rng(((tag as u64) << 20) ^ (seed as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15))
    }

    #[test]
    fn game_process_v11_matches() {
        check(
            ee::s12::f_00360768__generated,
            game_process_v11,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_0768, seed);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn typed_process_is_type_matches() {
        check(
            ee::s12::f_003607a0__generated,
            typed_process_is_type,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_07a0, seed);
                scramble(c, &mut r);
                let tag = (r.next_u32() & 0xffff) as u16;
                c.bus.write16(DATA + 0x10, tag);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 {
                    sx(tag as i16 as i32 as u32)
                } else {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn mod_obj_v51_matches() {
        check(
            ee::s12::f_00360dd8__generated,
            mod_obj_v51,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_0dd8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write16(DATA + 0x12, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn anim_object_v30_matches() {
        check(
            ee::s12::f_00361068__generated,
            anim_object_v30,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1068, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(
                    DATA,
                    if seed % 2 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    },
                );
            },
        );
    }

    #[test]
    fn anim_object_v53_matches() {
        check(
            ee::s12::f_00361158__generated,
            anim_object_v53,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1158, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 8, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 0x10, r.edge_f32());
            },
        );
    }

    /// A float that exercises the `>= 0.0` compares: ±0, denormals, the
    /// largest exponent and ordinary values.
    pub(crate) fn put_float(c: &mut Cpu, seed: u32, addr: u32, r: &mut Rng) {
        let bits = match seed % 5 {
            0 => 0,
            1 => 0x8000_0000,
            2 => 0x7f80_0000,
            3 => 0x0000_0001,
            _ => r.edge_f32(),
        };
        c.bus.write32(addr, bits);
    }

    #[test]
    fn anim_object_v31_matches() {
        check(
            ee::s12::f_00361170__generated,
            anim_object_v31,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1170, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                put_float(c, seed, DATA + 0x10, &mut r);
            },
        );
    }

    #[test]
    fn anim_teeter_v31_matches() {
        check(
            ee::s12::f_00361460__generated,
            anim_teeter_v31,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1460, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                put_float(c, seed, DATA + 4, &mut r);
            },
        );
    }

    #[test]
    fn float_rail_v49_matches() {
        check_lohi(
            ee::s12::f_003615a8__generated,
            float_rail_v49,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_15a8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(DATA + 0x70, DATA + 0x5000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x5000 + i * 0x50 + 0x44, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn spring_rail_v49_matches() {
        check_lohi(
            ee::s12::f_003615c0__generated,
            spring_rail_v49,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_15c0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(DATA + 0x70, DATA + 0x5000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x5000 + i * 0x60 + 0x54, r.next_u32());
                }
            },
        );
    }

    /// The transform source (`a1`) and the object (`a0`) of a
    /// `SetTransformBounds` copy.
    pub(crate) fn put_transform(c: &mut Cpu, r: &mut Rng, dest: u32, sub: bool) {
        let src = DATA + 0x5000;
        for i in 0..0x30u32 {
            c.bus.write32(src + i * 4, r.next_u32());
            c.bus.write32(DATA + dest + i * 4, r.next_u32());
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(src);
        if sub {
            c.bus.write32(DATA + 0x3c, DATA + 0x6000);
        }
    }

    #[test]
    fn position_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361918__generated,
            position_modifier_set_transform_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1918, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x10, false);
            },
        );
    }

    #[test]
    fn magnet_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361a30__generated,
            magnet_modifier_set_transform_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1a30, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x20, false);
            },
        );
    }

    #[test]
    fn ava_spline_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361ae0__generated,
            ava_spline_modifier_set_transform_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1ae0, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x10, false);
            },
        );
    }

    #[test]
    fn spline_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361b60__generated,
            spline_modifier_set_transform_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1b60, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x10, false);
            },
        );
    }

    #[test]
    fn multi_spline_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361bf0__generated,
            multi_spline_modifier_set_transform_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1bf0, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x30, true);
            },
        );
    }

    #[test]
    fn roller_modifier_is_transform_finished_matches() {
        check(
            ee::s12::f_00361cb0__generated,
            roller_modifier_is_transform_finished,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1cb0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let bits = match seed % 4 {
                    0 => 0x4188_0000, // 17.0
                    1 => 0x4188_0001,
                    2 => 0x4187_ffff,
                    _ => r.edge_f32(),
                };
                c.bus.write32(DATA + 0x20, bits);
            },
        );
    }

    #[test]
    fn roller_modifier_set_transform_bounds_matches() {
        check(
            ee::s12::f_00361d00__generated,
            roller_modifier_set_transform_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1d00, seed);
                scramble(c, &mut r);
                put_transform(c, &mut r, 0x220, false);
            },
        );
    }

    #[test]
    fn rail_man_v11_matches() {
        check(
            ee::s12::f_00361d78__generated,
            rail_man_v11,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1d78, seed);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn anim_combo_v31_matches() {
        check(
            ee::s12::f_00361d90__generated,
            anim_combo_v31,
            SEEDS_7,
            &|c, seed| {
                let mut r = rng(0x0036_1d90, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(
                    DATA + 0x14,
                    if seed % 3 == 0 { 0 } else { r.next_u32() as u8 },
                );
                put_float(c, seed, DATA, &mut r);
                put_float(c, seed / 2, DATA + 0x28, &mut r);
            },
        );
    }

    pub(crate) fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    // ------------------------------------------------------- object transforms

    #[test]
    fn mod_obj_get_matrix_at_matches() {
        check(
            ee::s12::f_003610e0__generated,
            mod_obj_get_matrix_at,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_10e0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 64) as u64;
                h(c, DATA + 0x26, if seed % 2 == 0 { 0 } else { 1 });
                w(c, DATA + 0x44, DATA + 0x3000);
                let vt = vtable(c, &mut r, &[0x198]);
                w(c, DATA + 0x20, vt);
            },
        );
    }

    // ------------------------------------------------------------------
    // Misc.

    #[test]
    fn tex_flip_modifier_save_matches() {
        check(
            ee::s12::f_0035f688__generated,
            tex_flip_modifier_save,
            40,
            &|c, seed| {
                base_2(c, seed, 0x35f6_88);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                let stream = DATA + 0x300;
                clear(c, stream, 0x20);
                let vt = DATA + 0x400;
                vtable_3(
                    c,
                    vt,
                    &[
                        (0x8, FAKE + 0x10),
                        (0xc, FAKE + 0x20),
                        (0x28, FAKE + 0x30),
                        (0x2c, FAKE + 0x40),
                    ],
                );
                w_2(c, stream, vt as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    #[test]
    fn intc_table_vif1_handler_matches() {
        check(
            ee::s12::f_00362340__generated,
            intc_table_vif1_handler,
            16,
            &|c, seed| {
                machine(c, seed, 0x0036_2340, 0);
                be_library_2(c);
                c.bus
                    .write32(0x1000_3c00, if seed % 2 == 0 { 0 } else { 0x400 });
            },
        );
    }

    #[test]
    fn texman_reset_vram_matches() {
        check(
            ee::s12::f_00367078__generated,
            texman_reset_vram,
            8,
            &|c, seed| {
                machine(c, seed, 0x0036_7078, 2);
                be_library_2(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                // `VramHeap_FreeAll(self + 0x4350/0x1f60)` walks a list of rail
                // links: `+0x23c0` is the head index and `+0x1ff0` the link
                // array. A zeroed head (0) with a null array makes the inner
                // "find the next free link" walk read `[0].next` as 0 forever.
                // A negative head terminates both free-alls immediately; the
                // function under test only calls them and resets the counters.
                c.bus.write32(DATA + 0x4350 + 0x23c0, 0xffff_ffff);
                c.bus.write32(DATA + 0x1f60 + 0x23c0, 0xffff_ffff);
            },
        );
    }

    #[test]
    fn intc_table_end_frame_stub_matches() {
        leaf(
            ssx3_ee::s12::f_00361f90__generated,
            intc_table_end_frame_stub,
            6,
        );
    }

    #[test]
    fn roller_modifier_save_matches() {
        check(
            ee::s11::f_0035e1e0__generated,
            roller_modifier_save,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x35e1e0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w_3(c, OBJ2, VT);
                slot_2(c, VT, 0x8, 0, fakes::FAKE);
                slot_2(c, VT, 0x28, 0, fakes::FAKE);
                w_3(c, OBJ + 0xd0, if seed % 2 == 0 { 0 } else { 0x1234 });
            },
        );
    }
    // returns 1

    #[test]
    fn tex_flip_modifier_apply_frame_matches() {
        check(
            ee::s12::f_0035f3a8__generated,
            tex_flip_modifier_apply_frame,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x35f3a8, 0);
                c.g[A0][0] = OBJ as u64;
                let count: u32 = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2,
                };
                w_3(c, OBJ + 0x1c, count);
                w_3(c, OBJ + 0x18, OBJ2);
                w_3(c, OBJ + 0x14, 0);
                w_3(c, OBJ2 + 0x94, OBJ3);
                w_3(c, OBJ3 + 4, OBJ4);
                w_3(c, OBJ3 + 8, OBJ2 + 0x200);
                if seed % 2 == 0 {
                    w_3(c, OBJ4 + 0x10, OBJ3 + 0x400);
                    w_3(c, OBJ3 + 0x400 + 4, 0x1234);
                } else {
                    w_3(c, OBJ4 + 0x10, 0);
                    w_3(c, OBJ2 + 0x200 + 0x10, 0);
                }
            },
        );
    }

    #[test]
    fn tex_man_init_table_matches() {
        check(
            ee::s12::f_003670e0__generated,
            tex_man_init_table,
            4,
            &|c, seed| {
                let _ = machine(c, seed, 0x0036_70e0, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn tex_flip_modifier_ctor_2_matches() {
        covered(
            ee::s12::f_0035f1a8__generated,
            tex_flip_modifier_ctor_2,
            6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_f1a8, 2);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2_2 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x18]);
                w(c, OBJ2_2, vt);
                w(c, OBJ_2 + 0x18, 0);
                w(c, OBJ_2 + 0x14, 0);
            },
        );
    }

    #[test]
    fn floating_v51_matches() {
        covered(
            ee::s12::f_00361858__generated,
            floating_v51,
            6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_1858, 2);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = OBJ_2 as u64;
                let vt = vtable(c, &mut r, &[0xc0, 0x108]);
                w(c, OBJ_2 + 0xc, vt);
                w(c, OBJ_2 + 0x74, r.next_u32());
                c.bus.write16(OBJ_2 + 0x12, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn intc_table_add_handler_matches() {
        check(
            ee::s12::f_00361f98__generated,
            intc_table_add_handler,
            40,
            &|c, seed| {
                base_3(c, seed, 0x361f_98);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x3000);
                c.g[A3][0] = sx(DATA + 0x3400);
            },
        );
    }

    #[test]
    fn intctable_init_matches() {
        check(
            ee::s12::f_00361eb8__generated,
            intctable_init,
            SEEDS_10,
            &|c, seed| {
                base_4(c, seed, 0x61eb8, 0);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn tex_man_ctor_matches() {
        check(
            ee::s12::f_00366fe8__generated,
            tex_man_ctor,
            20,
            &|c, seed| {
                let _r = machine(c, seed, 0x366fe8, 0);
                c.g[A0][0] = sx(DATA);
                // The two VramHeap base pointers `VramHeap_Init` reads.
                c.bus.write32(DATA + 0x3f50, DATA + 0x8000);
                c.bus.write32(DATA + 0x6340, DATA + 0x9000);
            },
        );
    }

    // ------------------------------------------------------------- momentum

    #[test]
    fn roller_modifier_update_momentum_matches() {
        check(
            ee::s11::f_0035d288__generated,
            roller_modifier_update_momentum,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35d2, 0);
                c.g[A0][0] = DATA as u64;
                for off in (0x20..0x74).step_by(4) {
                    w(c, DATA + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn vram_heap_free_unlocked_matches() {
        check(
            ee::s12::f_00366548__generated,
            vram_heap_free_unlocked,
            20,
            &|c, seed| {
                let _r = base_5(c, seed, 0x3665);
                c.g[A0][0] = DATA as u64;
                // Start index -1: nothing to free.
                c.bus.write32(DATA + 0x23c0, (-1i32) as u32);
                c.bus.write32(DATA + 0x1ff0, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn rollermodifier_get_pose_matches() {
        check(
            ee::s11::f_0035e770__generated,
            rollermodifier_get_pose,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x35e770, 0);
                let dst = DATA;
                let src = DATA + 0x400;
                clear_2(c, dst, 0x100);
                clear_2(c, src, 0x400);
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                for k in 0..0x40u32 {
                    let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
                    put_vec(c, src + 4 * k, v);
                }
                c.bus
                    .plat
                    .set_acc([r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            },
        );
    }

    #[test]
    fn vramheap_free_all_matches() {
        check_lohi(
            ee::s12::f_00366618__generated,
            vramheap_free_all,
            8,
            &|c, seed| {
                let _ = base_6(c, seed, 0x366618, 2);
                let this = DATA;
                clear_2(c, this, 0x4000);
                c.g[A0][0] = this as u64;
                w(c, this + 0x1ff0, DATA + 0x3000);
                let i = if seed % 2 == 0 { 0 } else { 0xffff_fffe };
                w(c, this + 0x23c0, i);
                // one node: next = -1, flags clear_2
                let node = DATA + 0x3000 + i.wrapping_mul(0x1c);
                w(c, node + 0x10, 0xffff_ffff);
                w(c, node, if seed % 2 == 0 { 0 } else { 2 });
            },
        );
    }

    // -------------------------------------------------------- tex flip

    #[test]
    fn texflip_modifier_handle_event_matches() {
        strict(
            ee::s12::f_0035f598__generated,
            texflip_modifier_handle_event,
            90,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x35f5, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => 300,
                    1 => 301,
                    2 => 302,
                    3 => 299,
                    _ => 303,
                };
                c.f[12] = r.edge_f32();
                w(c, DATA + 0x8, 0);
                w(c, DATA + 0x14, (seed % 4) as u32);
                w(c, DATA + 0x20, 1 + (seed % 4) as u32);
            },
        );
    }

    /// Event 300's speed is the per-tick timer step: the same flip rate per
    /// second is `v * 60/rate` at another rate.
    #[test]
    fn texflip_modifier_handle_event_speed_follows_the_rate() {
        fn speed(hz: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 300;
            c.f[12] = 1.0f32.to_bits();
            texflip_modifier_handle_event(&mut c);
            f32::from_bits(c.bus.read32(DATA + 0x8))
        }
        assert_eq!(speed(60), 1.0);
        assert_eq!(speed(120), 0.5);
        assert_eq!(speed(240), 0.25);
    }

    #[test]
    fn tex_flip_modifier_ctor_matches() {
        strict_2(
            ee::s12::f_0035f0b8__generated,
            tex_flip_modifier_ctor,
            SEEDS_12,
            &|c, seed| {
                let mut r = base_8(c, seed, 0xf0b8, 0);
                let clock = DATA + 0x400;
                let desc = DATA + 0x1000;
                let this = DATA + 0x2000;
                let owner = DATA + 0x3000;
                w(c, GAME_GP.wrapping_add(0x2a74), clock);
                w(c, clock + 0x10, r.next_u32() | 1);
                w(c, desc + 4, r.next_u32());
                w(c, desc + 0xc, r.next_u32());
                // a negative first frame asks for a random start frame
                let n = if seed % 3 == 0 {
                    (r.next_u32() % 0x8000_0000) as i32
                } else {
                    -((r.next_u32() % 64) as i32) - 1
                };
                w(c, desc + 8, n as u32);
                put_vec(c, desc + 0x10, [r.edge_f32(), 0, 0, 0]);
                // the owner's entry table is empty: the init loop is skipped
                w(c, owner + 0x94, owner + 0x100);
                w(c, owner + 0x100, 0);
                c.bus.write32(this + 0x18, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(desc);
                c.g[A2][0] = sx(owner);
            },
        );
    }

    #[test]
    fn gs_pkt_finish_matches() {
        same(
            ssx3_ee::s12::f_003627a8__generated,
            gs_pkt_finish,
            200,
            &|c, seed| {
                let mut r = Rng(0x3627a8 ^ seed as u64);
                scramble(c, &mut r);
                let w = DATA + 0x100;
                let pp = DATA + 0x200;
                let base = DATA + 0x800;
                let cur = DATA + 0x900;
                c.g[A0][0] = w as u64;
                c.g[A1][0] = pp as u64;
                c.g[A2][0] = (if seed % 9 == 0 { 3 } else { seed % 4 }) as u64;
                let count = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 6
                };
                c.bus.write32(w, count);
                c.bus.write32(w + 4, cur);
                c.bus.write32(w + 8, base);
                c.bus.write32(pp, DATA + 0x1400);
                for i in 0..0x80 {
                    c.bus.write8(DATA + 0x1400 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn tex_flip_modifier_update_matches() {
        same(
            ssx3_ee::s12::f_0035f410__generated,
            tex_flip_modifier_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x35f410 ^ seed as u64);
                generic(c, seed, 0x20);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                for off in [0u32, 4, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
                    c.bus.write32(s + off, (r.next_u32() % 8) as u32);
                }
                c.bus.write32(s + 8, r.f32(1.0));
                let hz = DATA + 0x500;
                c.bus.write32(GAME_GP + 0x2a74, hz);
                c.bus.write32(hz + 0x10, 30 + (r.next_u32() % 120));
            },
        );
    }

    #[test]
    fn roller_set_angular_momentum_matches() {
        check(
            ssx3_ee::s11::f_0035cff0__generated,
            roller_modifier_set_angular_momentum,
            200,
            &|c, seed| {
                let mut r = Rng(0x35cf_f0d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                // rotation at +0x50..0x74, position at +0x90..0x9c
                for k in 0..0x30u32 {
                    c.bus.write32(DATA + 0x30 + 4 * k, r.next_u32());
                }
                for k in 0..0x10u32 {
                    c.bus.write32(DATA + 0x90 + 4 * k, r.next_u32());
                }
                for k in 0..8u32 {
                    c.bus.write32(DATA + 0x400 + 4 * k, r.next_u32());
                }
            },
        );
    }

    pub(crate) const DEFER: u32 = DATA + 0x1_0000;

    #[test]
    fn defer_man_emit_default_state_matches() {
        check_all(
            ee::s12::f_00364360__generated,
            defer_man_emit_default_state,
            20,
            &|c, seed| {
                let mut r = base_9(c, seed, 0x3643_60, 10);
                let sp = c.g[ssx3_ee_rt::cpu::reg::SP][0] as u32;
                let pp = DATA + 0x1_1000;
                c.g[A0][0] = sx(DEFER);
                c.g[A1][0] = sx(pp);
                c.bus.write32(pp, DATA + 0x1_2000);
                // The GS state words the packet is built from.
                put_words(
                    c,
                    0x0050_1420,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_words(c, DATA + 0x1_2000, &[0, 0, 0, 0, 0, 0, 0, 0]);
                put_words(c, 0x0050_1400, &[0, 0, 0, 0]);
                let _ = sp;
            },
        );
    }

    /// A roller at `DATA` for `RollerModifier_Update`: random body state, a
    /// collision tree of one sphere (radius 1) and a world whose collision
    /// set is empty (`Nash_FindClosestContact` finds nothing), a heap for the
    /// collision query's tree.
    fn roller_update_fixture(c: &mut Cpu, r: &mut Rng, timer: f32, avg: f32, mass: f32) {
        let this = DATA;
        for off in (0x30..0xe0u32).step_by(4) {
            c.bus.write32(this + off, r.f32(2.0));
        }
        c.bus.write32(this + 0x4, avg.to_bits());
        c.bus.write32(this + 0x20, timer.to_bits());
        c.bus.write32(this + 0x28, r.f32(1.0));
        c.bus.write32(this + 0x70, mass.to_bits());
        // The collision tree: +0x90 scale, +0x98 its sphere set (index 0 of
        // a radius table).
        let tree = this + 0xe0;
        for off in (0..0x160u32).step_by(4) {
            c.bus.write32(tree + off, 0);
        }
        c.bus.write32(tree + 0x90, 1.0f32.to_bits());
        let set = DATA + 0x7000;
        c.bus.write32(tree + 0x98, set);
        c.bus.write32(set + 0xc, 0);
        c.bus.write32(set + 0x20, DATA + 0x7100);
        c.bus.write32(DATA + 0x7100, 2.0f32.to_bits());
        // The world's collision set (World_GetNash): no root.
        let wld = DATA + 0x2_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), wld);
        c.bus.write32(wld + 0x84, wld + 0x100);
        c.bus.write32(wld + 0x100 + 0x20, wld + 0x200);
        c.bus.write32(wld + 0x200 + 0x10, 0);
        c.bus.write32(wld + 0x200 + 0xa4, 0);
    }

    #[test]
    fn roller_modifier_update_matches() {
        check(
            ee::s11::f_0035e850__generated,
            roller_modifier_update,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35e850, 0);
                elf_image(c);
                crate::game::wscript::wscriptman::tests::alloc_ready(c);
                // reset, waiting, stop by time, stop by energy, simulate x2.
                let (timer, avg) = match seed % 6 {
                    0 => (16.5, 50000.0),
                    1 => (12.0, 50000.0),
                    2 => (10.5, 50000.0),
                    3 => (1.0, 10.0),
                    _ => (1.0, 50000.0),
                };
                let mass = if seed % 7 == 0 {
                    0.0
                } else {
                    1.0 + (seed % 5) as f32
                };
                roller_update_fixture(c, &mut r, timer, avg, mass);
                if seed % 6 == 3 {
                    for off in [0x50u32, 0x60, 0xb0, 0xc0] {
                        for k in 0..4 {
                            c.bus.write32(DATA + off + 4 * k, 0);
                        }
                    }
                }
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// The energy average keeps its time constant: with the same energy each
    /// tick, one update at 60 and `rate/60` updates at 120 / 240 Hz give the
    /// same average (the timer is put back each tick so the blend runs on
    /// every call; the stop path runs and clears the momenta, which the test
    /// sets again).
    #[test]
    fn roller_modifier_update_follows_the_rate() {
        fn after(hz: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 5, 0x35e851, 0);
            elf_image(c);
            crate::tick::set_for_test(c, hz);
            roller_update_fixture(c, &mut r, 10.5, 50000.0, 2.0);
            for _ in 0..hz / 60 {
                c.bus.write32(DATA + 0x20, 10.5f32.to_bits());
                for off in [0x50u32, 0x60, 0xb0, 0xc0] {
                    for k in 0..4 {
                        c.bus.write32(DATA + off + 4 * k, 0);
                    }
                }
                c.bus.write32(DATA + 0x50, 100.0f32.to_bits());
                c.bus.write32(DATA + 0xb0, 1.0f32.to_bits());
                c.g[A0][0] = DATA as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
                roller_modifier_update(c);
                assert_eq!(c.pc, crate::difftest::RA);
            }
            f32::from_bits(c.bus.read32(DATA + 0x4))
        }
        let base = after(60);
        assert!((base - 47510.0).abs() < 0.5, "{base}");
        for hz in [120, 240] {
            let v = after(hz);
            assert!(
                (v - base).abs() < 0.1,
                "{hz} Hz: average {v} vs {base} at 60"
            );
        }
    }

    /// UVScrollModifier_GetMatrix: rotation (angle != 0, through the real
    /// Math_SinCos) and identity (angle == 0, also -0) paths, with edge-case
    /// axis and translation values, and an `m` that overlaps `self` for some
    /// seeds (the original re-reads the axis between stores).
    #[test]
    fn uv_scroll_modifier_get_matrix_matches() {
        use crate::difftest::{check_paths, fill, scramble, Rng, DATA, GAME_GP};
        const MATRIX: u32 = DATA + 0x100;
        check_paths(
            ee::s12::f_0035fc20__generated,
            uv_scroll_modifier_get_matrix,
            300,
            &[12 + 10 + 11],
            &|c, seed| {
                let mut r = Rng(0x35fc_2000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                // Math_SinCos's .lit4 constants and the identity matrix.
                fill(c, &mut r, GAME_GP - 0x2ec0, 8);
                for i in 0..16u32 {
                    let one = if i % 5 == 0 { 0x3f80_0000 } else { 0 };
                    c.bus.write32(IDENTITY_MATRIX + i * 4, one);
                }
                fill(c, &mut r, DATA, 0x20);
                let angle = match seed % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(6.3),
                };
                c.bus.write32(DATA + uv_scroll::ANGLE, angle);
                for off in [
                    uv_scroll::AXIS_X,
                    uv_scroll::AXIS_Y,
                    uv_scroll::AXIS_Z,
                    uv_scroll::TRANS_U,
                    uv_scroll::TRANS_V,
                ] {
                    let v = if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(DATA + off, v);
                }
                fill(c, &mut r, MATRIX, 16);
                c.g[A0][0] = sx(DATA);
                // Some seeds: m overlaps self's axis (aliasing order).
                c.g[A1][0] = sx(if seed % 7 == 5 { DATA + 0x10 } else { MATRIX });
                c.pc = 0x0035_fc20;
            },
        );
    }

    /// DeferMan_SortBuckets: 0..64 items with random keys (sometimes few
    /// distinct bytes), the scratchpad stream at gp+0x1384 with two slots
    /// whose blocks are in RAM; the real stream callees and memset run.
    #[test]
    fn defer_man_sort_buckets_matches() {
        check(
            ee::s12::f_00364050__generated,
            defer_man_sort_buckets,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_4050 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0036_4050;
                let (man, stream) = (DATA, DATA + 0x7_0000);
                c.bus.write32(GAME_GP + 0x1384, stream);
                let slot = r.next_u32() % 2;
                c.bus.write32(stream + 4, 2);
                c.bus.write32(stream + 8, slot);
                c.bus.write32(stream + 0xc, slot);
                c.bus.write32(stream + 0x10, 0x101);
                c.bus.write32(stream + 0x14, DATA + 0x7_1000);
                c.bus.write32(stream + 0x18, DATA + 0x7_3000);
                fill(c, &mut r, DATA + 0x7_1000, 0x404);
                fill(c, &mut r, DATA + 0x7_3000, 0x404);
                let n = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() % 65,
                };
                c.bus.write32(man + oc230_defer::COUNT, n);
                for i in 0..n {
                    let key = match seed % 3 {
                        0 => r.next_u32(),
                        1 => r.next_u32() & 0x0103_0107,
                        _ => r.next_u32() % 300,
                    };
                    c.bus.write32(man + oc230_defer::ITEMS + 8 * i, key);
                    let v = r.next_u32();
                    c.bus.write32(man + oc230_defer::ITEMS + 8 * i + 4, v);
                }
                fill(c, &mut r, oc230_defer::SCRATCH, 2 * 64);
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// TexMan_ResetUsageTable: the scratchpad stream's free pointer at
    /// `gp+0x2a94` leaving 0x400..0x4000 free bytes (16..2 chunks, the last
    /// one shorter when 2000 is not a multiple), random table and scratchpad
    /// contents. The real InvalidDCache and SprStream_Download run (DMA from
    /// the scratchpad, waiting). A null free pointer is not covered: the
    /// chunk size would be 0 and the original would loop forever.
    #[test]
    fn tex_man_reset_usage_table_matches() {
        check(
            ee::s12::f_003666f8__generated,
            tex_man_reset_usage_table,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_66f8 ^ (seed as u64) << 17);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                let free = 0x400 + (r.next_u32() % 0x3c1) * 0x10;
                let ptr = SPR_END - free;
                c.bus.write32(crate::difftest::GAME_GP + GP_SPR_FREE, ptr);
                fill(c, &mut r, TEX_USAGE_TABLE, 4000);
                for i in 0..0x1000 {
                    c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
                }
                c.pc = 0x0036_66f8;
            },
        );
    }

    /// `PositionModifier_GetMatrix`: the dirty word zero (no call) or not;
    /// the update slot is a fake (logs, sometimes unwinds).
    #[test]
    fn position_modifier_get_matrix_matches() {
        const M: u32 = DATA;
        const VT: u32 = DATA + 0x1000;
        covered(
            ee::s12::f_00361940__generated,
            position_modifier_get_matrix,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361940, 1);
                c.pc = 0x0036_1940;
                c.bus.write32(M, VT);
                c.bus.write32(
                    M + POSMOD_DIRTY,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus
                    .write16(VT + SLOT_POSMOD_UPDATE, (r.next_u32() as u16) & 0x00fc);
                c.bus.write32(VT + SLOT_POSMOD_UPDATE + 4, FAKE);
                c.g[A0][0] = sx(M);
            },
        );
    }

    // ------------------------------------------------ RailModifier_UpdateBounds

    /// `RailModifier_UpdateBounds`: "has bounds" is a fake (nonzero) or the
    /// real leaf at 0x1406c0 (returns 0); "get bounds" is a fake whose
    /// forced result points at a random box. Some calls unwind.
    #[test]
    fn rail_modifier_update_bounds_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::put_vec;
        check(
            ee::s11::f_0035c040__generated,
            rail_modifier_update_bounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_c040, 2);
                c.pc = 0x0035_c040;
                let this = DATA + 0x1000;
                let owner = DATA + 0x1100;
                let object = DATA + 0x1200;
                let vt = DATA + 0x1400;
                let bounds = DATA + 0x2000;
                c.bus.write32(this + 0x40, owner);
                c.bus.write32(owner + 0xc, object);
                c.bus.write32(object + 0xc, vt);
                for slot in [0x160, 0x168] {
                    c.bus
                        .write16(vt + slot, (r.next_u32() as i16 & 0x7c) as u16);
                }
                c.bus
                    .write32(vt + 0x164, if seed % 4 == 0 { 0x0014_06c0 } else { FAKE });
                c.bus.write32(vt + 0x16c, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS + 4, bounds);
                for k in 0..2 {
                    put_vec(
                        c,
                        bounds + 0x10 * k,
                        [r.f32(5000.0), r.f32(5000.0), r.f32(5000.0), r.next_u32()],
                    );
                    put_vec(
                        c,
                        this + 0x10 + 0x10 * k,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `RailModifier_v03`: the real `RailModifier_UpdateBounds` asks the
    /// rail object's vtable (slot 0x160: the real `return 0` leaf on every
    /// third seed, so the box stays, else a fake) for the new box (slot
    /// 0x168, a fake forced to return a point box P); the old box is
    /// P -/+ 100 (integer coordinates, exact), so the grown new box equals it
    /// and the real `SpatialHash_Update` (nash from the world chain
    /// gp-0x848 -> +0x84 -> +0x20) finds the keys unchanged; the key's grid
    /// constants at gp-0x2df8/-0x2df4 are set as in its own test.
    #[test]
    fn rail_modifier_v03_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::{put_vec, GAME_GP};
        const THIS: u32 = DATA;
        const RAIL: u32 = DATA + 0x100;
        const OBJ: u32 = DATA + 0x200;
        const VT: u32 = DATA + 0x400;
        const POINT: u32 = DATA + 0x800;
        const APP: u32 = DATA + 0x900;
        const GAME: u32 = DATA + 0xa00;
        covered(
            ssx3_ee::s11::f_0035c4e0__generated,
            rail_modifier_v03,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x35c4e0, 3);
                c.pc = 0x0035_c4e0;
                c.g[A0][0] = sx(THIS);
                c.bus.write32(THIS + 0x40, RAIL);
                c.bus.write32(THIS + 0x44, r.next_u32());
                c.bus.write32(RAIL + 0xc, OBJ);
                c.bus.write32(OBJ + 0xc, VT);
                c.bus.write16(VT + 0x160, (r.next_u32() & 0x7c) as u16);
                c.bus
                    .write32(VT + 0x164, if seed % 3 == 0 { 0x0014_06c0 } else { FAKE });
                c.bus.write16(VT + 0x168, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT + 0x16c, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS + 4, POINT);
                let p: Vec<f32> = (0..3)
                    .map(|_| (r.next_u32() % 2000) as f32 - 1000.0)
                    .collect();
                let one = 1.0f32.to_bits();
                put_vec(
                    c,
                    POINT,
                    [p[0].to_bits(), p[1].to_bits(), p[2].to_bits(), one],
                );
                put_vec(
                    c,
                    POINT + 0x10,
                    [p[0].to_bits(), p[1].to_bits(), p[2].to_bits(), one],
                );
                let lo = |v: f32| (v - 100.0).to_bits();
                let hi = |v: f32| (v + 100.0).to_bits();
                put_vec(c, THIS + 0x10, [lo(p[0]), lo(p[1]), lo(p[2]), one]);
                put_vec(c, THIS + 0x20, [hi(p[0]), hi(p[1]), hi(p[2]), one]);
                c.bus.write32(GAME_GP - 0x848, APP);
                c.bus.write32(APP + 0x84, GAME);
                c.bus.write32(GAME + 0x20, r.next_u32());
                // `SpatialKey_FromAABB`'s grid constants (scale 1.0, slack 0.25);
                // zero RAM there makes its level search never end (pc 0x329070).
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x2df8), 1.0f32.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x2df4), 0.25f32.to_bits());
            },
        );
    }

    /// `ModObj_GetMatrices`: modifier active or not; the modifier's update
    /// slot is a fake (logging the call, sometimes unwinding).
    #[test]
    fn mod_obj_get_matrices_matches() {
        use crate::difftest::fakes::vtable;
        check(
            ee::s12::f_00361098__generated,
            mod_obj_get_matrices,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_1098, 1);
                c.pc = 0x0036_1098;
                c.g[A0][0] = sx(DATA);
                let flags = (r.next_u32() & 0xfffe) as u16 | (seed % 2) as u16;
                c.bus.write16(DATA + MODOBJ_FLAGS, flags);
                c.bus.write32(DATA + MODOBJ_MATRICES, DATA + 0x3000);
                let vt = vtable(c, &mut r, &[MODIFIER_UPDATE_SLOT]);
                c.bus.write32(DATA + MODOBJ_MODIFIER_VT, vt);
            },
        );
    }

    // (fakes imported above)

    // ------------------------------------------------------- vram heap

    /// VramHeap_NodesFromScratchpad: the heap at a random 16-byte aligned
    /// spot, the scratchpad stream's pointer (`gp+0x2a94`) at a random
    /// quadword that leaves room for the 0x1ff0-byte block, random RAM and
    /// scratchpad. The real InvalidDCache and SprStream_Download run.
    #[test]
    fn vram_heap_nodes_from_scratchpad_matches() {
        check(
            ee::s12::f_00365fa8__generated,
            vram_heap_nodes_from_scratchpad,
            100,
            &|c, seed| {
                let mut r = Rng(0x0036_5fa8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0036_5fa8;
                fill(c, &mut r, DATA, 0x1000);
                for i in 0..0x1000 {
                    c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
                }
                let spr = 0x7000_0000 + (r.next_u32() % 0x200) * 0x10;
                c.bus.write32(GAME_GP + 0x2a94, spr);
                c.g[A0][0] = sx(DATA + (r.next_u32() % 0x100) * 0x10);
            },
        );
    }

    /// `VramHeap_NodesToScratchpad`: the real `SyncDCache` (interrupts
    /// enabled or not) and `SprStream_Upload` / `Dma_ToScratchpad`
    /// (waiting for the transfer) on a random node table, to scratchpad
    /// stream offsets 0..0x200.
    #[test]
    fn vram_heap_nodes_to_scratchpad_matches() {
        check(
            ssx3_ee::s12::f_00365f68__generated,
            vram_heap_nodes_to_scratchpad,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_5f68 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0036_5f68;
                let status = r.next_u32() & !0x1_0000;
                c.cop0[12] = if seed % 2 == 0 {
                    status | 0x1_0000
                } else {
                    status
                };
                c.cop0[28] = r.next_u32();
                let heap = DATA + 0x10 * (r.next_u32() % 0x40);
                fill(c, &mut r, heap, VRAM_NODES_SIZE / 4 + 4);
                for i in 0..0x1000 {
                    c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
                }
                c.bus
                    .write32(GAME_GP + 0x2a94, 0x10 * (r.next_u32() % 0x20));
                c.g[A0][0] = sx(heap);
            },
        );
    }

    /// UVScrollModifier_Save: the stream's write method is a fake (which may
    /// unwind).
    #[test]
    fn uv_scroll_modifier_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        covered(
            ee::s12::f_0035fd98__generated,
            uv_scroll_modifier_save,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_fd98, 1);
                c.pc = 0x0035_fd98;
                let out = DATA + 0x300;
                let vt = vtable(c, &mut r, &[UV_SCROLL_SAVE_WRITE_SLOT]);
                c.bus.write32(out, vt);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(out);
            },
        );
    }

    /// `ModifierBlock_FreeParticles`: an empty list, or 1..5 links (each
    /// `next` at +0, a vtable at +8 whose delete slot (+8) is a fake).
    #[test]
    fn modifier_block_free_particles_matches() {
        const BLOCK: u32 = DATA + 0x100;
        const LINKS: u32 = DATA + 0x1000;
        covered(
            ee::s11::f_003531d0__generated,
            modifier_block_free_particles,
            200,
            &|c, seed| {
                let links = seed % 6;
                let mut r = machine(c, seed, 0x3531d0, links);
                c.pc = 0x0035_31d0;
                let vt = DATA + 0x400;
                c.bus.write16(vt + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(vt + 0xc, FAKE);
                let link = |i: u32| if i < links { LINKS + 0x40 * i } else { 0 };
                fill(c, &mut r, BLOCK, 8);
                c.bus.write32(BLOCK + 0x10, link(0));
                for i in 0..links {
                    c.bus.write32(link(i), link(i + 1));
                    c.bus.write32(link(i) + 8, vt);
                }
                c.g[A0][0] = sx(BLOCK);
            },
        );
    }

    /// `ModifierBlock_FreeTransform`: no transform, or one whose destructor
    /// is a fake (sometimes unwinding).
    #[test]
    fn modifier_block_free_transform_matches() {
        covered(
            ee::s11::f_003530d0__generated,
            modifier_block_free_transform,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_30d0, 1);
                c.pc = 0x0035_30d0;
                let block = DATA + 0x20_0000 + 0x100;
                let transform = DATA + 0x20_0000 + 0x200;
                if seed % 4 == 0 {
                    c.bus.write32(block + MODIFIER_BLOCK_TRANSFORM, 0);
                } else {
                    c.bus.write32(block + MODIFIER_BLOCK_TRANSFORM, transform);
                    let vt = vtable(c, &mut r, &[0x8]);
                    c.bus.write32(transform, vt);
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    /// ModifierBlock_FreeBoost: no boost object, or one whose destructor
    /// slot is a fake (logs `this` and flags; some calls unwind).
    #[test]
    fn modifier_block_free_boost_matches() {
        fakes::covered(
            ee::s11::f_00353188__generated,
            modifier_block_free_boost,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x353188, 1);
                let block = DATA + 0x100;
                let boost = DATA + 0x200;
                fill(c, &mut r, block, 8);
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(boost + 4, vt);
                c.bus
                    .write32(block + 0xc, if seed % 4 == 0 { 0 } else { boost });
                c.g[A0][0] = sx(block);
                c.pc = 0x0035_3188;
            },
        );
    }

    /// The id method is a fake: its result equals the process's id (a
    /// non-negative one, as fakes return zero-extended values) or not; some
    /// seeds unwind in it.
    #[test]
    fn sync_process_id_equals_matches() {
        use crate::difftest::fakes::{covered, vtable, LOG_RESULTS};
        covered(
            ee::s12::f_00360840__generated,
            sync_process_id_equals,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_0840, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                let vt = vtable(c, &mut r, &[OC300_VT_GET_ID]);
                c.bus.write32(obj + OC300_OBJ_VTABLE, vt);
                let id = if seed % 3 == 0 {
                    r.next_u32()
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(this + OC300_SYNC_ID, id);
                if seed % 3 != 0 {
                    let result = if seed % 2 == 0 { id } else { id ^ 1 };
                    c.bus.write32(LOG_RESULTS, result);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.pc = 0x0036_0840;
            },
        );
    }

    /// `ModifierBlock_FreeTexFlip`: no modifier, or one allocated from a real
    /// dlmalloc arena and freed by the real `__builtin_delete`.
    #[test]
    fn modifier_block_free_tex_flip_matches() {
        use crate::game::object::animnode::tests::heap_block;
        use crate::game::wscript::wscriptman::tests::alloc_ready;
        crate::difftest::check(
            ee::s11::f_00353150__generated,
            modifier_block_free_tex_flip,
            200,
            &|c, seed| {
                let _ = crate::difftest::fakes::machine(c, seed, 0x3531_50, 0);
                alloc_ready(c);
                let this = crate::difftest::DATA + 0x100;
                for i in 0..4 {
                    c.bus.write32(this + 4 * i, 0x1111_0000 + i);
                }
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x20 + 0x10 * (seed % 4))
                };
                c.bus.write32(this + OC305_MODIFIER_BLOCK_TEX_FLIP, owned);
                c.g[A0][0] = sx(this);
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0035_3150;
            },
        );
    }

    /// No UV-scroll block, or one really allocated (a pool-sized one or a
    /// heap one) that `__builtin_delete` frees.
    #[test]
    fn modifier_block_free_uv_scroll_matches() {
        use crate::game::fe::feasyncfile::tests::{call_machine, run};
        check(
            ee::s11::f_00353118__generated,
            modifier_block_free_uv_scroll,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x35_3118, 0);
                let block = DATA + 0x100;
                c.bus.write32(block, r.next_u32());
                let uv = if seed % 3 == 0 {
                    0
                } else {
                    let size = if seed % 3 == 1 { 0x30 } else { 0x400 };
                    run(
                        c,
                        0x0031_7e30,
                        ee::s10::operator_new_named,
                        &[size, 0x0048_15a0, 0, 0],
                    ) as u32
                };
                c.bus.write32(block + MODIFIER_BLOCK_UV_SCROLL, uv);
                c.g[A0][0] = sx(block);
                c.pc = 0x0035_3118;
            },
        );
    }

    /// A stream object at `stream` whose vtable (at `vt`) has fake methods
    /// in the slots `slots` with random `this` deltas.
    pub(crate) fn oc308_fake_stream(
        c: &mut Cpu,
        r: &mut crate::difftest::Rng,
        stream: u32,
        vt: u32,
        slots: &[u32],
    ) {
        c.bus.write32(stream, vt);
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// Both stream calls are made (fakes log their arguments), and either may
    /// unwind.
    #[test]
    fn rail_modifier_v02_matches() {
        covered(
            ee::s11::f_0035c540__generated,
            rail_modifier_v02,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_c540, 2);
                let self_ = DATA;
                let stream = DATA + 0x1000;
                oc308_fake_stream(
                    c,
                    &mut r,
                    stream,
                    DATA + 0x2000,
                    &[STREAM_WRITE_BYTES, STREAM_WRITE_PTR],
                );
                c.bus.write32(self_ + RAIL_MODIFIER_INSTANCE, r.next_u32());
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0035_c540;
            },
        );
    }

    /// A float vector: mostly ordinary values, sometimes edge cases.
    pub(crate) fn oc313_vec(r: &mut Rng, edge: bool) -> [u32; 4] {
        let mut v = [0u32; 4];
        for x in v.iter_mut() {
            *x = if edge { r.edge_f32() } else { r.f32(50.0) };
        }
        v
    }

    /// Rail transform: no start node (rail matrix), a single node, and
    /// chains of up to four nodes; the link is allocated from the pools.
    #[test]
    fn rail_modifier_build_rail_nodes_matches() {
        check(
            ee::s11::f_0035c0e8__generated,
            rail_modifier_build_rail_nodes,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_c0e8, 0);
                c.pc = 0x0035_c0e8;
                crate::game::fe::debugfe::tests::pool_allocator(c);
                let this = DATA;
                let rail = DATA + 0x1000;
                let table = DATA + 0x2000;
                let nodes = DATA + 0x3000;
                let mats = DATA + 0x4000;
                let edge = seed % 9 == 8;
                for i in 0..0x24u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                for i in 0..4u32 {
                    let v = oc313_vec(&mut r, edge);
                    put_vec(c, rail + RM_BUILD_RAIL_MATRIX + 0x10 * i, v);
                    put_vec(
                        c,
                        this + RM_BUILD_XFORM + 0x10 * i,
                        oc313_vec(&mut r, false),
                    );
                }
                c.bus.write32(this + RM_BUILD_RAIL, rail);
                c.bus.write32(rail + RM_BUILD_RAIL_TABLE, table);
                c.bus.write32(rail + RM_BUILD_RAIL_SCALE, r.f32(4.0));
                c.bus.write32(table + 0x8, nodes);
                // Node chain: start, then `len` more, then -1.
                let len = seed % 5;
                let mut idx = 1 + r.next_u32() % 3;
                let start = idx;
                for k in 0..=len {
                    let next = if k == len {
                        u32::MAX
                    } else {
                        idx + 1 + r.next_u32() % 2
                    };
                    let node = nodes + 0x10 * idx;
                    c.bus.write32(node, next);
                    c.bus.write32(node + 0xc, mats + 0x40 * k);
                    for i in 0..4u32 {
                        let v = oc313_vec(&mut r, edge);
                        put_vec(c, mats + 0x40 * k + 0x10 * i, v);
                    }
                    idx = next;
                }
                let first = if seed % 4 == 0 {
                    u32::MAX - r.next_u32() % 4
                } else {
                    start
                };
                c.bus.write32(this + RM_BUILD_NODE, first);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `Object_v01`: the vtable store, then the real `ModObj_dtor` chain on
    /// the fixture of `mod_obj_dtor_matches` (no modifier block, or an empty
    /// one from a dlmalloc arena; WorldObj_dtor without the delete bit). The
    /// mutex syscalls are fakes; some seeds unwind at one of the calls.
    #[test]
    fn object_v01_matches() {
        use crate::difftest::fakes::{machine, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        use crate::game::bx::cubicspline::tests::oc238_run;
        use crate::game::object::movenode::MOD_OBJ_BLOCK;
        check(
            ee::s12::f_00361038__generated,
            object_v01,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_1038, 0);
                let obj = DATA + 0x1000;
                let inst = DATA + 0x1100;
                fill(c, &mut r, obj, 8);
                c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, 0);
                } else {
                    c.bus.write32(obj + 4, DATA + 0x1200);
                    c.bus
                        .write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x1300 });
                }
                c.bus
                    .write32(obj + 0x18, if seed % 7 == 0 { 0 } else { inst });
                c.bus.write32(inst + 0x8, r.next_u32());
                c.bus
                    .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
                let block = if seed % 4 == 0 {
                    0
                } else {
                    // The game heap over test memory, pools off.
                    c.bus.write32(GAME_GP + 0xdd8, 0);
                    oc238_run(
                        c,
                        0x0031_ff60,
                        ee::s10::f_0031ff60,
                        &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                    );
                    let b = oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[0x40, 0]) as u32;
                    for i in 0..0x10 {
                        c.bus.write32(b + 4 * i, 0);
                    }
                    b
                };
                c.bus.write32(obj + MOD_OBJ_BLOCK, block);
                // Unwind at one of the next calls (the setup made some).
                if seed % 3 == 1 {
                    let n = c.bus.read32(LOG_COUNT);
                    c.bus.write32(LOG_UNWIND_AT, n + r.next_u32() % 6);
                }
                c.pc = 0x0036_1038;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    // -- 0x00361290 IntersectInfo_dtor_2 -------------------------------------

    /// A `cMeshAnimFrame` at DATA; the deleting destructor variant (`flags
    /// & 1`) calls `operator delete`, the plain one does not. Other flag bits
    /// are masked away (and stay masked in `a1`).
    #[test]
    fn intersect_info_dtor_2_matches() {
        use crate::game::ai::aianimeventmap::tests::alloc_setup;
        check(
            ee::s12::f_00361290__generated,
            intersect_info_dtor_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_1290, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = match seed % 4 {
                    0 => 0u64,
                    1 => 1,
                    2 => 0xfffe | 1,
                    _ => (r.next_u32() & !1) as u64,
                };
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0036_1290;
            },
        );
    }

    pub(crate) use crate::game::animation::rideranim::tests::CTRL;
    pub(crate) use crate::game::animation::rideranim::tests::MAGIC;

    pub(crate) use crate::game::animation::rideranim::tests::active;
    pub(crate) use crate::game::animation::rideranim::tests::chain;
    pub(crate) use crate::game::animation::rideranim::tests::fake_ret;
    pub(crate) use crate::game::animation::rideranim::tests::install;

    // -- 0x3612c0 IntersectInfo_ResetMatrix --------------------------------

    /// The identity quadwords at `D_004FF1A0` copied over a random matrix.
    #[test]
    fn intersect_info_reset_matrix_matches() {
        check(
            ee::s12::f_003612c0__generated,
            intersect_info_reset_matrix,
            60,
            &|c, seed| {
                let mut r = Rng(0x3612_c0 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                elf_image(c);
                for k in 0..4u32 {
                    put_vec(
                        c,
                        IIRM_IDENTITY + 0x10 * k,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                let this = DATA + 0x1000;
                for k in 0..4u32 {
                    put_vec(
                        c,
                        this + IIRM_MATRIX + 0x10 * k,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    // -- Particle_v01 --------------------------------------------------------

    pub(crate) fn fake_particle_base_dtor(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 6, fake_particle_base_dtor);
        }
        fake_ret(c, 8);
    }

    #[test]
    fn particle_v01_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_003553C0),
            fake_particle_base_dtor,
            6,
        );
        check(
            ee::s12::f_00361d50__generated,
            particle_v01,
            200,
            &|c, seed| {
                machine(c, seed, 0x0036_1d50, 1);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x0036_1d50;
            },
        );
    }
}
