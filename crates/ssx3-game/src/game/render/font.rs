//! `src/render/font.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tags2::p as p_2;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::object::railmodifier::DELETE;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::particle::copy_mat;
pub(crate) use crate::game::render::particle::p_2 as p;
pub(crate) use crate::game::wscript::wscriptman::p as p_3;
use crate::guest::Addr;
use crate::ps2f::vu0_scratch;
use crate::ps2f::FpuF32;
use crate::ps2f::Vec4;
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
    (0x0039_1c48, font_draw_text_f, "Font_DrawTextF"),
    (
        0x0039_29f8,
        font_draw_text_centered_f,
        "Font_DrawTextCenteredF",
    ),
    (0x0039_2a60, font_draw_text_right, "Font_DrawTextRight"),
    (0x0039_2b40, font_draw_text_right_f, "Font_DrawTextRightF"),
    (0x0039_2ba8, font_get_char_width, "Font_GetCharWidth"),
    (0x0039_2c60, font_get_char_width_w, "Font_GetCharWidthW"),
    (0x0039_2d18, fx_trail_reset, "FXTrail_Reset"),
    (0x0039_2d90, fxtrail_add_point, "FXTrail_AddPoint"),
    (
        0x0039_2df0,
        renderer_build_sine_table,
        "Renderer_BuildSineTable",
    ),
    (
        0x0039_3fb8,
        graphics_man_set_screen_offset,
        "GraphicsMan_SetScreenOffset",
    ),
    (
        0x0039_3fd8,
        graphics_man_get_screen_offset_x,
        "GraphicsMan_GetScreenOffsetX",
    ),
    (
        0x0039_3ff0,
        graphics_man_get_screen_offset_y,
        "GraphicsMan_GetScreenOffsetY",
    ),
    (0x0039_42a0, graphics_man_v58, "GraphicsMan_v58"),
    (
        0x0039_4b38,
        grid_mesh_base_base_dtor,
        "GridMeshBase_BaseDtor",
    ),
    (0x0039_4b98, grid_mesh_base_call_v22, "GridMeshBase_CallV22"),
    (
        0x0039_4c98,
        light_man_init_instances,
        "LightMan_InitInstances",
    ),
    (
        0x0039_4ce8,
        light_man_free_instances,
        "LightMan_FreeInstances",
    ),
    (0x0039_4d18, light_man_next_id, "LightMan_NextId"),
    (0x0039_4ed0, defer_man_intern_state, "DeferMan_InternState"),
    (0x0039_5000, defer_man_find_bucket, "DeferMan_FindBucket"),
    (
        0x0039_50c0,
        ps2_graphics_man_copy_view_state,
        "PS2GraphicsMan_CopyViewState",
    ),
    (
        0x0039_5330,
        ps2_graphics_man_get_screen_width,
        "PS2GraphicsMan_GetScreenWidth",
    ),
    (
        0x0039_5350,
        ps2_graphics_man_get_screen_height,
        "PS2GraphicsMan_GetScreenHeight",
    ),
    (
        0x0039_54d0,
        ps2_graphics_man_set_lighting,
        "PS2GraphicsMan_SetLighting",
    ),
    (
        0x0039_56e8,
        ps2_graphics_man_push_matrix,
        "PS2GraphicsMan_PushMatrix",
    ),
    (
        0x0039_5c38,
        ps2_graphics_man_load_matrix,
        "PS2GraphicsMan_LoadMatrix",
    ),
    (
        0x0039_5c70,
        ps2_graphics_man_mult_matrix,
        "PS2GraphicsMan_MultMatrix",
    ),
    (
        0x0039_5cf0,
        ps2_graphics_man_load_identity,
        "PS2GraphicsMan_LoadIdentity",
    ),
    (
        0x0039_5d28,
        ps2_graphics_man_translate,
        "PS2GraphicsMan_Translate",
    ),
    (
        0x0039_5d60,
        ps2_graphics_man_rotate_axis,
        "PS2GraphicsMan_RotateAxis",
    ),
    (
        0x0039_6110,
        c_ps2_grid_mesh_set_scale,
        "cPS2GridMesh_SetScale",
    ),
    (
        0x0039_61a8,
        vfx_render_toggles_dtor,
        "VfxRenderToggles_dtor",
    ),
    (0x0039_64c8, vfx_plants_dtor, "VfxPlants_dtor"),
    (0x0039_6838, vfx_tube_dtor, "VfxTube_dtor"),
    (0x0039_68d0, rail_tube_set_point, "RailTube_SetPoint"),
    (0x0039_68e8, path_arrow_v01, "PathArrow_v01"),
    (0x0039_6928, detail_system_v01, "DetailSystem_v01"),
    (
        0x0039_6a00,
        vfx_visual_effects_test_dtor,
        "VfxVisualEffectsTest_dtor",
    ),
    (0x0039_6d30, vfx_screen_tint_dtor, "VfxScreenTint_dtor"),
];

/// `DeferMan` layout (offsets from the `DeferMan*`).
pub(crate) mod defer {

    /// Bucket chain heads, one word per pass (`FindBucket`).
    pub const BUCKETS: u32 = 0x6_78a0;

    /// State record chain heads, one word per pass (`InternState`).
    pub const STATE_HEADS: u32 = 0x6_74a0;

    /// The state record pool: `+0x1480` count, records from `+0x1484`.
    pub const STATE_POOL: u32 = 0x5_0000;

    pub const STATE_POOL_COUNT: u32 = 0x1480;

    pub const STATE_POOL_RECORDS: u32 = 0x5_1484;

    pub const STATE_RECORD_SIZE: u32 = 0x18;

    /// Bucket node: keys at +0/+4/+8, "not a leaf" at +0x14, next at +0x1c.
    pub const NODE_SKIP: u32 = 0x14;

    pub const NODE_NEXT: u32 = 0x1c;

    /// State record: four compared words, a fifth, next at +0x14.
    pub const STATE_NEXT: u32 = 0x14;
}

/// One saved view (`sPtView`).
pub(crate) mod vw {
    pub(crate) use crate::game::render::particle::vw::DEPTH;
    pub(crate) use crate::game::render::particle::vw::F24;
    pub(crate) use crate::game::render::particle::vw::F28;
    pub(crate) use crate::game::render::particle::vw::F2C;
    pub(crate) use crate::game::render::particle::vw::KEY;
    pub(crate) use crate::game::render::particle::vw::M120;
    pub(crate) use crate::game::render::particle::vw::M180;
    pub(crate) use crate::game::render::particle::vw::M1C0;
    pub(crate) use crate::game::render::particle::vw::M60;
    pub(crate) use crate::game::render::particle::vw::MA0;
    pub(crate) use crate::game::render::particle::vw::ME0;
    pub(crate) use crate::game::render::particle::vw::Q160;
    pub(crate) use crate::game::render::particle::vw::Q170;
    pub(crate) use crate::game::render::particle::vw::Q200;
    pub(crate) use crate::game::render::particle::vw::Q210;
    pub(crate) use crate::game::render::particle::vw::Q30;
    pub(crate) use crate::game::render::particle::vw::Q40;
    pub(crate) use crate::game::render::particle::vw::Q50;
    pub(crate) use crate::game::render::particle::vw::SCISSOR;
}

/// `FXTrail` (font flash trail) fields: the count, the `{x,y,z}` cursor, and
/// the two endpoint times.
pub(crate) mod trail {

    pub const COUNT: u32 = 0x8;

    pub const START: u32 = 0xa0;

    pub const END: u32 = 0xa4;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
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

// ---------------------------------------------------------------------------
// Deferred draw buckets.

/// `DeferManBucket *DeferMan_FindBucket(DeferMan *man, uint hash, uint key1,
/// uint pass, uint key2)` (0x395000): walk the chain of pass `pass & 0xff`
/// for the leaf bucket with keys `(hash, key1, key2)`; a hit is moved to the
/// head of the chain and returned, otherwise 0. Nodes with +0x14 set are
/// never matched.
pub fn defer_man_find_bucket(c: &mut Cpu) {
    c.cycles += 9;
    let man = abi::arg(c, 0);
    let slot_off = (abi::arg(c, 3) & 0xff) << 2;
    let head_slot = man.wrapping_add(defer::BUCKETS + slot_off);
    c.g[A3][0] = slot_off as u64;
    c.g[T3][0] = sx(head_slot);
    if Addr(head_slot).lw(c) == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = 1;
    c.g[A0][0] = sx(man.wrapping_add(defer::BUCKETS));
    loop {
        // t3: the link being followed; a1/a2/t0: the keys.
        let link = r32(c, T3);
        c.cycles += 5;
        let node = Addr(link).lw(c);
        let n = Addr(node as u32);
        let skip = n.off(defer::NODE_SKIP).lw(c);
        // (v1 as the compare chain leaves it, hit)
        let (v1, hit) = if skip != 0 {
            (node, false)
        } else {
            let key1 = n.off(4).lw(c);
            if key1 != c.g[A2][0] {
                c.cycles += 5;
                (0, false)
            } else {
                let key2 = n.off(8).lw(c);
                if key2 != c.g[T0][0] {
                    c.cycles += 7;
                    (0, false)
                } else {
                    c.cycles += 12;
                    let same = n.lw(c) == c.g[A1][0];
                    (same as u64, same)
                }
            }
        };
        c.cycles += 2;
        c.g[V1][0] = v1;
        c.g[T1][0] = node;
        c.g[T2][0] = hit as u64;
        if !hit {
            let next = n.off(defer::NODE_NEXT).lw(c);
            c.cycles += 2;
            c.g[T3][0] = sx((node as u32).wrapping_add(defer::NODE_NEXT));
            if next != 0 {
                if event_due(c) {
                    c.g[V0][0] = next;
                    deliver_event_here(c, 0x0039_5030);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        // Hit: unlink it and push it on the head of the chain.
        c.cycles += 4;
        let head_slot = r32(c, A0).wrapping_add(r32(c, A3));
        c.g[A3][0] = sx(head_slot);
        if node != Addr(head_slot).lw(c) {
            let next = n.off(defer::NODE_NEXT).lw(c);
            c.cycles += 6;
            c.g[A0][0] = next;
            Addr(link).sw(c, next);
            let old_head = Addr(head_slot).lw(c);
            c.g[V1][0] = old_head;
            n.off(defer::NODE_NEXT).sw(c, old_head);
            Addr(head_slot).sw(c, node);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = node;
        abi::ret(c);
        return;
    }
}

/// `DeferStateRecord *DeferMan_InternState(DeferMan *man, const u32 state[5],
/// int pass)` (0x394ed0): share equal draw states. Look in the chain of
/// pass `pass & 0xff` for a record whose first four words equal `state`;
/// a hit is moved to the head and returned. Otherwise take the next pool
/// record, copy the five words into it, push it on the chain and return it.
pub fn defer_man_intern_state(c: &mut Cpu) {
    c.cycles += 10;
    let man = abi::arg(c, 0);
    let pass = c.g[A2][0] & 0xff;
    let slot_off = (pass as u32) << 2;
    let first_link = man.wrapping_add(defer::STATE_HEADS + slot_off);
    c.g[A3][0] = pass;
    c.g[V1][0] = slot_off as u64;
    c.g[T2][0] = c.g[A0][0];
    c.g[T3][0] = c.g[A1][0];
    c.g[T1][0] = sx(first_link);
    if Addr(first_link).lw(c) != 0 {
        c.cycles += 4;
        c.g[T5][0] = slot_off as u64;
        c.g[T4][0] = sx(man.wrapping_add(defer::STATE_HEADS));
        if intern_state_find(c) {
            return;
        }
    }
    intern_state_insert(c);
}

/// The chain walk of `DeferMan_InternState`: true when it found (and
/// returned) a record. Registers: t1 the link, t3 the state, t4 + t5 the
/// head slot.
pub(crate) fn intern_state_find(c: &mut Cpu) -> bool {
    loop {
        c.cycles += 4;
        let link = r32(c, T1);
        let rec = Addr(link).lw(c);
        c.g[A2][0] = rec;
        // Compare the four state words: a0/a1 walk record and state, t0
        // counts (both cursors stop on a difference).
        let mut rec_at = rec;
        let mut state_at = c.g[T3][0];
        let mut count = 0u64;
        let mut word;
        let same = loop {
            c.cycles += 4;
            word = Addr(rec_at as u32).lw(c);
            let want = Addr(state_at as u32).lw(c);
            count = sx((count as u32).wrapping_add(1));
            if word != want {
                c.cycles += 2;
                break false;
            }
            c.cycles += 4;
            state_at = sx((state_at as u32).wrapping_add(4));
            rec_at = sx((rec_at as u32).wrapping_add(4));
            if count < 4 {
                if event_due(c) {
                    c.g[V0][0] = 1;
                    c.g[V1][0] = word;
                    c.g[T0][0] = count;
                    c.g[A0][0] = rec_at;
                    c.g[A1][0] = state_at;
                    deliver_event_here(c, 0x0039_4f18);
                    (count, rec_at, state_at) = (c.g[T0][0], c.g[A0][0], c.g[A1][0]);
                }
                continue;
            }
            c.cycles += 1;
            break true;
        };
        c.cycles += 2;
        let head_slot = r32(c, T4).wrapping_add(r32(c, T5));
        c.g[V1][0] = word;
        c.g[T0][0] = count;
        c.g[A0][0] = rec_at;
        c.g[A1][0] = sx(head_slot);
        let r = Addr(rec as u32);
        if !same {
            c.cycles += 3;
            let next = r.off(defer::STATE_NEXT).lw(c);
            c.g[V0][0] = next;
            c.g[T1][0] = sx((rec as u32).wrapping_add(defer::STATE_NEXT));
            if next != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_4f08);
                }
                continue;
            }
            return false;
        }
        // Hit: move it to the head of the chain.
        c.cycles += 3;
        if rec != Addr(head_slot).lw(c) {
            let next = r.off(defer::STATE_NEXT).lw(c);
            c.cycles += 6;
            c.g[A0][0] = next;
            Addr(link).sw(c, next);
            let old_head = Addr(head_slot).lw(c);
            c.g[V1][0] = old_head;
            r.off(defer::STATE_NEXT).sw(c, old_head);
            Addr(head_slot).sw(c, rec);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = rec;
        abi::ret(c);
        return true;
    }
}

/// The miss path of `DeferMan_InternState`: take pool record `count++`,
/// copy the five state words (the first four as two unaligned doublewords:
/// records are 0x18 bytes from +0x51484), push it on the chain.
pub(crate) fn intern_state_insert(c: &mut Cpu) {
    c.cycles += 31;
    let man = r32(c, T2);
    let state = r32(c, T3);
    let pool = man.wrapping_add(defer::STATE_POOL);
    let count_at = pool.wrapping_add(defer::STATE_POOL_COUNT);
    let count = Addr(count_at).lw(c);
    let rec_off = abi::mult(c, count as u32, defer::STATE_RECORD_SIZE);
    let slot_off = r32(c, A3) << 2;
    Addr(count_at).sw(c, sx((count as u32).wrapping_add(1)));
    let head_slot = man.wrapping_add(defer::STATE_HEADS).wrapping_add(slot_off);
    let rec = man.wrapping_add(rec_off.wrapping_add(defer::STATE_POOL_RECORDS));
    let s = Addr(state);
    let words01 = s.read_u64_unaligned(c);
    let words23 = s.off(8).read_u64_unaligned(c);
    let word4 = s.off(0x10).lw(c);
    Addr(rec).write_u64_unaligned(c, words01);
    Addr(rec).off(8).write_u64_unaligned(c, words23);
    Addr(rec).off(0x10).sw(c, word4);
    let old_head = Addr(head_slot).lw(c);
    Addr(rec).off(defer::STATE_NEXT).sw(c, old_head);
    Addr(head_slot).sw(c, sx(rec));
    c.g[V0][0] = sx(rec);
    c.g[V1][0] = old_head;
    c.g[A0][0] = sx(head_slot);
    c.g[A1][0] = sx(rec);
    c.g[A2][0] = word4;
    c.g[A3][0] = sx(slot_off);
    abi::ret(c);
}

/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
pub(crate) fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// `void PS2GraphicsMan_CopyViewState(sPtView* dst, const sPtView* src)`
/// (0x3950c0): copy one saved view (scissor/shorts, floats, matrices, quads,
/// key). Returns `dst` in v0 and the copied key in v1.
pub fn ps2_graphics_man_copy_view_state(c: &mut Cpu) {
    c.cycles += 106;
    let dst = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.g[A3][0] = sx(p(dst, vw::M60));
    c.g[A2][0] = sx(p(src, vw::M60));
    for &(off, fpr) in &[
        (0x0u32, 1usize),
        (0x4, 0),
        (0x8, 1),
        (0xc, 0),
        (0x10, 1),
        (0x14, 0),
        (0x18, 1),
        (0x1c, 0),
    ] {
        c.lwc1(fpr, p(src, off));
        c.swc1(fpr, p(dst, off));
    }
    c.g[V0][0] = Addr(src).off(0x20).lw(c);
    c.sw(p(dst, 0x20), c.g[V0][0]);
    for &(off, fpr) in &[(0x24u32, 0usize), (0x28, 1), (0x2c, 0)] {
        c.lwc1(fpr, p(src, off));
        c.swc1(fpr, p(dst, off));
    }
    copy_q(c, 2, p(src, vw::Q30), p(dst, vw::Q30));
    copy_q(c, 3, p(src, vw::Q40), p(dst, vw::Q40));
    copy_q(c, 2, p(src, vw::Q50), p(dst, vw::Q50));
    copy_mat(c, c.g[A2][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(dst, vw::MA0));
    c.g[V0][0] = sx(p(src, vw::MA0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A2][0] = sx(p(dst, vw::ME0));
    c.g[V0][0] = sx(p(src, vw::ME0));
    copy_mat(c, c.g[V0][0] as u32, c.g[A2][0] as u32);
    c.g[A3][0] = sx(p(dst, vw::M120));
    c.g[V0][0] = sx(p(src, vw::M120));
    copy_mat(c, c.g[V0][0] as u32, c.g[A3][0] as u32);
    c.lq(3, p(src, vw::Q160));
    c.g[A3][0] = sx(p(dst, vw::M180));
    c.g[A2][0] = sx(p(src, vw::M180));
    c.sq(3, p(dst, vw::Q160));
    c.lq(2, p(src, vw::Q170));
    c.sq(2, p(dst, vw::Q170));
    copy_mat(c, c.g[A2][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(dst, vw::M1C0));
    c.g[V0][0] = sx(p(src, vw::M1C0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.lq(6, p(src, vw::Q200));
    c.g[V0][0] = c.g[A0][0];
    c.sq(6, p(dst, vw::Q200));
    c.lq(3, p(src, vw::Q210));
    c.sq(3, p(dst, vw::Q210));
    c.ldl(3, p(src, vw::SCISSOR + 7));
    c.ldr(3, p(src, vw::SCISSOR));
    c.sdl(3, p(dst, vw::SCISSOR + 7));
    c.sdr(3, p(dst, vw::SCISSOR));
    c.g[V1][0] = Addr(src).off(vw::KEY).lw(c);
    c.sw(p(dst, vw::KEY), c.g[V1][0]);
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// Font / flash trails.

/// `void FXTrail_Reset(FXTrail *trail)` (0x392d18): clear the trail's count,
/// its `{x,y,z}` cursor and the two endpoint times.
pub fn fx_trail_reset(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x10);
    // The original builds a zeroed {x,y,z} on the stack and moves it to the
    // trail as one doubleword; keep the same stack traffic and registers.
    let tmp = frame.at(c, 0);
    Addr(tmp).sw(c, 0);
    Addr(tmp + 4).sw(c, 0);
    Addr(tmp + 8).sw(c, 0);
    let trail = abi::arg(c, 0);
    // (+0xc cleared too, before the copy.)
    c.sw(trail.wrapping_add(0xc), 0);
    c.ldl(V0, tmp + 7);
    c.ldr(V0, tmp);
    c.g[V1][0] = Addr(tmp + 8).lw(c);
    c.sdl(V0, trail.wrapping_add(7));
    c.sdr(V0, trail);
    c.sw(trail.wrapping_add(trail::COUNT), c.g[V1][0]);
    c.sw(trail.wrapping_add(trail::END), 0);
    c.sw(trail.wrapping_add(trail::START), 0);
    frame.close(c);
}

/// `void GridMeshBase_CallV22(GridMeshBase* self, ...)` (0x394b98): call slot
/// 0x16 of the mesh object at `+0x4` with the remaining arguments.
pub fn grid_mesh_base_call_v22(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T2][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let obj = c.g[T2][0] as u32;
        c.g[V1][0] = Addr(at(obj, 0xb0)).lh(c);
        c.g[V0][0] = Addr(at(obj, 0xb4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0039_4bb4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LightMan_FreeInstances(LightMan* self)` (0x394ce8): free the
/// instance array at `+0x4`, if any.
pub fn light_man_free_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let list = c.g[A0][0];
        frame.restore_ra(c, 0);
        if list != 0 {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0039_4d04)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `GridMeshBase` base-object destroying destructor (0x394b38): the base
/// vtable is the second word of the object.
pub fn grid_mesh_base_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_4b5c, 0x0049_3938, 0x4)
    })
}

/// `VfxRenderToggles` destroying destructor (0x3961a8).
pub fn vfx_render_toggles_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_61cc, 0x0048_8680, 0)
    })
}

/// `VfxPlants` destroying destructor (0x3964c8).
pub fn vfx_plants_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_64ec, 0x0048_8680, 0)
    })
}

/// `VfxTube` destroying destructor (0x396838).
pub fn vfx_tube_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_685c, 0x0048_8680, 0)
    })
}

/// `PathArrow` destroying destructor (0x3968e8).
pub fn path_arrow_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_690c, 0x0048_8680, 0)
    })
}

/// `DetailSystem` destroying destructor (0x396928).
pub fn detail_system_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_694c, 0x0048_8680, 0)
    })
}

/// `VfxVisualEffectsTest` destroying destructor (0x396a00).
pub fn vfx_visual_effects_test_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_6a24, 0x0048_8680, 0)
    })
}

/// `VfxScreenTint` destroying destructor (0x396d30).
pub fn vfx_screen_tint_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_6d54, 0x0048_8680, 0)
    })
}

// ---------------------------------------------------------------------------
// FX trail.

/// `void FXTrail_AddPoint(FXTrail* t, const Quat* rot, const Vec* pos)`
/// (0x392d90): append one point — 12 bytes at `t+0x10+12*count` and the
/// quaternion at `t+0x50+16*count` — then bump the count. 19 cycles.
pub fn fxtrail_add_point(c: &mut Cpu) {
    c.cycles += 19;
    let t = r32(c, A0);
    c.g[V0][0] = c.lw(p_2(t, 0xc));
    c.g[V1][0] = 0xc;
    c.mult(0, A3, V0, V1);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(t));
    let pos = r32(c, A2);
    c.ldl(8, p_2(pos, 7));
    c.ldr(8, pos);
    c.g[T1][0] = c.lw(p_2(pos, 8));
    let slot = r32(c, V0);
    c.sdl(8, p_2(slot, 0x17));
    c.sdr(8, p_2(slot, 0x10));
    c.sw(p_2(slot, 0x18), c.g[T1][0]);
    c.g[V1][0] = c.lw(p_2(t, 0xc));
    let rot = r32(c, A1);
    c.lq(6, rot);
    c.g[V1][0] = sx(r32(c, V1) << 4);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(t));
    c.sq(6, p_2(r32(c, V1), 0x50));
    c.g[V0][0] = c.lw(p_2(r32(c, A0), 0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.sw(p_2(t, 0xc), c.g[V0][0]);
    abi::ret(c);
}

/// `void LightMan_InitInstances(LightMan* self, int n)` (0x394c98): allocate
/// the `n + 0x32` instance table.
pub fn light_man_init_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x32));
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p_3(this, 0xc)).sw(c, 0);
        c.g[A1][0] = 0x49_1fa0; // D_00491FA0
        Addr(p_3(this, 0x10)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(r32(c, V1) << 2);
        Addr(p_3(this, 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0039_4cd4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0039_4cd4)?;
        Addr(p_3(this, 4)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics manager / font helpers.

/// `void GraphicsMan_SetScreenOffset(self, x, y)` (0x393fb8): store the
/// integer arguments as the screen-offset floats.
pub fn graphics_man_set_screen_offset(c: &mut Cpu) {
    c.cycles += 7;
    c.mtc1(5, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(6, 1);
    c.cvt_s_w(1, 1);
    let this = Addr(abi::arg(c, 0));
    c.swc1(0, this.0);
    c.swc1(1, this.off(0x4).0);
    abi::ret(c);
}

/// `int GraphicsMan_GetScreenOffsetX(self)` (0x393fd8): truncate the float at
/// `+0`.
pub fn graphics_man_get_screen_offset_x(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.lwc1(0, this.0);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    abi::ret(c);
}

/// `int GraphicsMan_GetScreenOffsetY(self)` (0x393ff0): truncate the float at
/// `+4`.
pub fn graphics_man_get_screen_offset_y(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.lwc1(0, this.off(0x4).0);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    abi::ret(c);
}

/// `self GraphicsMan_v58(self)` (0x3942a0): initialise a colour/scale object
/// to `{1.0f, 1.0f, 0, 1.0f}` and return it.
pub fn graphics_man_v58(c: &mut Cpu) {
    c.cycles += 8;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[V0][0] = c.g[A0][0];
    let this = Addr(abi::arg(c, 0));
    this.off(0x8).sw(c, 0);
    c.swc1(0, this.off(0xc).0);
    c.swc1(0, this.0);
    c.swc1(0, this.off(0x4).0);
    abi::ret(c);
}

/// `void* LightMan_NextId(self)` (0x394d18): `base + index` and increment the
/// index (`+0xc`).
pub fn light_man_next_id(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0xc).lw(c);
    c.g[V0][0] = this.off(0x8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    let next = c.g[V1][0];
    this.off(0xc).sw(c, next);
    abi::ret(c);
}

/// `int PS2GraphicsMan_GetScreenWidth(self)` (0x395330): the current font
/// entry's `+0x88` (entry size 0xa4).
pub fn ps2_graphics_man_get_screen_width(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x59c8).lw(c);
    c.g[A1][0] = 0xa4;
    c.g[V0][0] = this.off(0x59cc).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x88).lw(c);
    abi::ret(c);
}

/// `int PS2GraphicsMan_GetScreenHeight(self)` (0x395350): the current font
/// entry's `+0x8c`.
pub fn ps2_graphics_man_get_screen_height(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x59c8).lw(c);
    c.g[A1][0] = 0xa4;
    c.g[V0][0] = this.off(0x59cc).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    abi::ret(c);
}

/// `void PS2GraphicsMan_SetLighting(self, src)` (0x3954d0): copy the 0xa0-byte
/// lighting block to `self + 0x6bb0`, a quadword at a time.
pub fn ps2_graphics_man_set_lighting(c: &mut Cpu) {
    c.cycles += 2;
    let dst = (c.g[A0][0] as u32).wrapping_add(0x6bb0);
    c.g[A0][0] = sx(dst);
    let end = (c.g[A1][0] as u32).wrapping_add(0xa0);
    c.g[V0][0] = sx(end);
    loop {
        // Reload the loop state from the registers: an event delivered at
        // the back edge may have switched the guest thread.
        let src = c.g[A1][0] as u32;
        let dst = c.g[A0][0] as u32;
        c.cycles += 7;
        c.lq(3, src);
        c.lq(6, src.wrapping_add(0x10));
        c.sq(3, dst);
        c.sq(6, dst.wrapping_add(0x10));
        c.g[A1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_54d8);
        }
    }
}

/// `void PS2GraphicsMan_PushMatrix(self)` (0x3956e8): copy the current matrix
/// to `top + 0x40` and advance the stack (`+0x13e4`) and depth (`+0x13e0`).
pub fn ps2_graphics_man_push_matrix(c: &mut Cpu) {
    c.cycles += 17;
    let this = Addr(abi::arg(c, 0));
    c.g[A1][0] = this.off(0x13e4).lw(c);
    let src = c.g[A1][0] as u32;
    c.g[V0][0] = sx(src.wrapping_add(0x40));
    let dst = c.g[V0][0] as u32;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    c.g[V1][0] = this.off(0x13e4).lw(c);
    c.g[V0][0] = this.off(0x13e0).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    let top = c.g[V1][0];
    this.off(0x13e4).sw(c, top);
    let depth = c.g[V0][0];
    this.off(0x13e0).sw(c, depth);
    abi::ret(c);
}

/// `void PS2GraphicsMan_LoadMatrix(self, m)` (0x395c38): copy `m` over the
/// current matrix and drop the cached matrix flag (`+0x6b90`).
pub fn ps2_graphics_man_load_matrix(c: &mut Cpu) {
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    let m = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(1, m);
    c.lqc2(2, m.wrapping_add(0x10));
    c.lqc2(3, m.wrapping_add(0x20));
    c.lqc2(4, m.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_MultMatrix(self, m)` (0x395c70): current = current * m
/// (four VU0 dot products).
pub fn ps2_graphics_man_mult_matrix(c: &mut Cpu) {
    c.cycles += 31;
    let this = Addr(abi::arg(c, 0));
    let m = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(4, dst);
    c.lqc2(5, dst.wrapping_add(0x10));
    c.lqc2(6, dst.wrapping_add(0x20));
    c.lqc2(7, dst.wrapping_add(0x30));
    c.lqc2(8, m);
    c.lqc2(9, m.wrapping_add(0x10));
    c.lqc2(10, m.wrapping_add(0x20));
    c.lqc2(11, m.wrapping_add(0x30));
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
    c.sqc2(12, dst);
    c.sqc2(13, dst.wrapping_add(0x10));
    c.sqc2(14, dst.wrapping_add(0x20));
    c.sqc2(15, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_LoadIdentity(self)` (0x395cf0): copy the identity
/// matrix at `0x4ff1a0` over the current matrix.
pub fn ps2_graphics_man_load_identity(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x50_0000;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x13e4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    let dst = c.g[V1][0] as u32;
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_Translate(self, v)` (0x395d28): translate the current
/// matrix by the vector at `v` (VU0 dot product into row 3).
pub fn ps2_graphics_man_translate(c: &mut Cpu) {
    c.cycles += 13;
    let this = Addr(abi::arg(c, 0));
    let v = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(4, dst);
    c.lqc2(5, dst.wrapping_add(0x10));
    c.lqc2(6, dst.wrapping_add(0x20));
    c.lqc2(7, dst.wrapping_add(0x30));
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void cPS2GridMesh_SetScale(self, range)` (0x396110): store the range and
/// the reciprocal scale `32767 / range`.
pub fn c_ps2_grid_mesh_set_scale(c: &mut Cpu) {
    c.cycles += 5;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(0x254c));
    let this = Addr(abi::arg(c, 0));
    c.swc1(12, this.off(0x194).0);
    c.div_s(0, 0, 12);
    c.swc1(0, this.off(0x198).0);
    abi::ret(c);
}

/// `void RailTube_SetPoint(self, index, point)` (0x3968d0): copy the quadword
/// `point` into the tube's point array (`ptr + index * 0x10`).
pub fn rail_tube_set_point(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x8).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    let point = abi::arg(c, 2) as u32;
    c.lq(3, point);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let dst = c.g[A1][0] as u32;
    c.sq(3, dst);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Fonts and HUD.

/// `void Font_DrawTextF(void* font, float x, float y, const char* fmt, ...)`
/// (0x391c48).
pub fn font_draw_text_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p_4(c.g[SP][0] as u32, 0x20));
        c.sq(S1, p_4(c.g[SP][0] as u32, 0x10));
        c.g[S0][0] = 0x0050_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x0ca0));
        c.swc1(21, p_4(c.g[SP][0] as u32, 0x38));
        c.swc1(20, p_4(c.g[SP][0] as u32, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_86c8, 0x0039_1c7c)?;
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_1cb0, 0x0039_1c90)?;
        c.lq(S0, p_4(c.g[SP][0] as u32, 0x20));
        c.lq(S1, p_4(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, p_4(c.g[SP][0] as u32, 0x38));
        c.lwc1(20, p_4(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `Font_DrawTextCenteredF(font, fmt, ap, x, y)` 0x3929f8: format and draw a
/// centred string.
pub fn font_draw_text_centered_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let font = abi::arg64(c, 0);
        let fmt = abi::arg64(c, 1);
        let ap = abi::arg64(c, 2);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x20);
        c.swc1(21, p_5(sp(c), 0x38));
        c.g[S0][0] = font;
        c.swc1(20, p_5(sp(c), 0x30));
        c.g[A0][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A1][0] = fmt;
        c.g[A2][0] = ap;
        call_ptr(c, 0x0031_91c0, 0x0039_2a28)?;
        c.g[A1][0] = Addr(sp(c)).lw(c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_2908, 0x0039_2a3c)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0039_2a48)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(21, p_5(sp(c), 0x38));
        c.lwc1(20, p_5(sp(c), 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `Font_DrawTextRightF(font, fmt, ap, x, y)` 0x392b40: format and draw a
/// right-aligned string.
pub fn font_draw_text_right_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let font = abi::arg64(c, 0);
        let fmt = abi::arg64(c, 1);
        let ap = abi::arg64(c, 2);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x20);
        c.swc1(21, p_5(sp(c), 0x38));
        c.g[S0][0] = font;
        c.swc1(20, p_5(sp(c), 0x30));
        c.g[A0][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A1][0] = fmt;
        c.g[A2][0] = ap;
        call_ptr(c, 0x0031_91c0, 0x0039_2b70)?;
        c.g[A1][0] = Addr(sp(c)).lw(c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_2a60, 0x0039_2b84)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0039_2b90)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(21, p_5(sp(c), 0x38));
        c.lwc1(20, p_5(sp(c), 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rendering and fonts.

/// `Font_DrawTextRight(font, text, x, y)` (0x392a60): measure the string,
/// subtract its width from x and draw it.
pub fn font_draw_text_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 13);
        c.mov_s(20, 12);
        c.lwc1(12, p_6(c.g[S0][0] as u32, 0x38));
        c.lwc1(13, p_6(c.g[S0][0] as u32, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x0039_2a9c)?;
        c.sub_s(12, 20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(13, 21);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_1cb0, 0x0039_2ab0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `Renderer_BuildSineTable(self)` (0x392df0): fill 640 floats with
/// `sin(i * 2*pi/512)`.
pub fn renderer_build_sine_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x10);
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.lwc1(20, gp_2(c, -0x2694));
        loop {
            c.cycles += 9;
            c.mtc1(S1, 12);
            c.cvt_s_w(12, 12);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.mul_s(12, 12, 20);
            call_ptr(c, 0x0031_bf60, 0x0039_2e2c)?;
            c.swc1(0, c.g[S0][0] as u32);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 640i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_2e18);
                }
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.g[S2][0];
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x40);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Fonts.

/// `float Font_GetCharWidth(sGlyphTable* t, char c)` (0x392ba8): binary
/// search the glyph table for `c`; a found glyph's width, else 0.0f.
pub fn font_get_char_width(c: &mut Cpu) {
    c.cycles += 7;
    let font = c.g[A0][0];
    c.g[V1][0] = font;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A3][0] = Addr(p_7(font as u32, 0xc)).lw(c);
    c.g[A0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let below_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    'done: {
        if below_range {
            break 'done;
        }
        'search: {
            c.cycles += 4;
            c.g[A1][0] = Addr(p_7(font as u32, 0x10)).lw(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T0][0] = Addr(font as u32).lw(c);
                break 'search;
            }
            c.cycles += 7;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = Addr(p_7(font as u32, 4)).lw(c);
            c.mult(0, 4, 4, 2);
            c.mult(0, 2, 7, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            break 'done;
        }
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
        c.g[T2][0] = Addr(p_7(c.g[V1][0] as u32, 4)).lw(c);
        c.g[T1][0] = 0xc;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        'l: loop {
            c.cycles += 6;
            c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 6, 7, 9);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
            c.g[A2][0] = c.lhu(c.g[V0][0] as u32);
            let hit = c.g[A0][0] == c.g[A2][0];
            if hit {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[T0][0] = c.g[A3][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = Addr(p_7(c.g[V1][0] as u32, 8)).lw(c);
                break 'done;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_2c00);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(p_7(c.g[V0][0] as u32, 8)).lb(c);
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 3;
        c.mtc1(0, 0);
    }
    c.pc = c.g[RA][0] as u32;
}

/// `float Font_GetCharWidthW(sGlyphTable* t, unsigned short c)` (0x392c60):
/// the same lookup for wide characters.
pub fn font_get_char_width_w(c: &mut Cpu) {
    c.cycles += 6;
    let font = c.g[A0][0];
    c.g[V1][0] = font;
    c.g[A3][0] = Addr(p_7(font as u32, 0xc)).lw(c);
    c.g[A0][0] = c.g[A1][0] & 0xffff;
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let below_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    'done: {
        if below_range {
            break 'done;
        }
        'search: {
            c.cycles += 4;
            c.g[A2][0] = Addr(p_7(font as u32, 0x10)).lw(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(font as u32).lw(c);
                break 'search;
            }
            c.cycles += 7;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = Addr(p_7(font as u32, 4)).lw(c);
            c.mult(0, 4, 4, 2);
            c.mult(0, 2, 7, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            break 'done;
        }
        c.cycles += 5;
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
        c.g[T2][0] = Addr(p_7(c.g[V1][0] as u32, 4)).lw(c);
        c.g[T1][0] = 0xc;
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        'l: loop {
            c.cycles += 6;
            c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 6, 7, 9);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
            c.g[A2][0] = c.lhu(c.g[V0][0] as u32);
            let hit = c.g[A0][0] == c.g[A2][0];
            if hit {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[A3][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = Addr(p_7(c.g[V1][0] as u32, 8)).lw(c);
                break 'done;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_2cb8);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(p_7(c.g[V0][0] as u32, 8)).lb(c);
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 3;
        c.mtc1(0, 0);
    }
    c.pc = c.g[RA][0] as u32;
}

/// `void PS2GraphicsMan_RotateAxis(GraphicsMan* self, const float* ang)`
/// (0x395d60): build the rotation matrix for the three angles through
/// `Math_SinCos` and transform the manager's `+0x13e4` matrix in place, then
/// clear `+0x6b90`.
pub fn ps2_graphics_man_rotate_axis(c: &mut Cpu) {
    c.cycles += 94;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.sq(16, c.pa(SP, 0x80));
    c.sq(17, c.pa(SP, 0x70));
    c.g[S0][0] = c.g[A1][0];
    c.std(SP, 0x50, c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    c.sq(18, c.pa(SP, 0x60));
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S2][0] = c.ldw_2(S1, 0x13e4);
    c.g[RA][0] = sx(0x00395d8c);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x44));
    c.pc = 0x0031be50;
    call_indirect(c, 0x0031be50);
    if c.pc != 0x00395d8c {
        return;
    }
    c.lwc1(11, c.pa(SP, 0x44));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 5);
    c.lwc1(4, c.pa(S0, 0x8));
    c.sub_s(1, 5, 11);
    c.lwc1(0, c.pa(S0, 0));
    c.lwc1(6, c.pa(S0, 0x4));
    c.lwc1(8, c.pa(SP, 0x40));
    c.mul_s(2, 1, 4);
    c.swc1(5, c.pa(SP, 0x3c));
    c.mul_s(3, 1, 0);
    c.stw_2(SP, 0x30, 0u64);
    c.mul_s(1, 1, 6);
    c.stw_2(SP, 0x34, 0u64);
    c.mul_s(13, 8, 4);
    c.stw_2(SP, 0x38, 0u64);
    c.mul_s(12, 8, 0);
    c.stw_2(SP, 0xc, 0u64);
    c.mul_s(9, 2, 4);
    c.stw_2(SP, 0x1c, 0u64);
    c.mul_s(10, 3, 4);
    c.stw_2(SP, 0x2c, 0u64);
    c.mul_s(5, 2, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(4, 1, 4);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, c.pa(SP, 0x8));
    c.add_s(7, 7, 11);
    c.swc1(0, c.pa(SP, 0x4));
    c.add_s(3, 3, 13);
    c.swc1(2, c.pa(SP, 0x18));
    c.sub_s(10, 10, 8);
    c.swc1(9, c.pa(SP, 0x28));
    c.add_s(1, 1, 11);
    c.swc1(7, c.pa(SP, 0));
    c.add_s(4, 4, 12);
    c.swc1(3, c.pa(SP, 0x10));
    c.swc1(10, c.pa(SP, 0x20));
    c.swc1(1, c.pa(SP, 0x14));
    c.swc1(4, c.pa(SP, 0x24));
    c.lqc2(4, c.pa(S2, 0));
    c.lqc2(5, c.pa(S2, 0x10));
    c.lqc2(6, c.pa(S2, 0x20));
    c.lqc2(7, c.pa(S2, 0x30));
    c.lqc2(8, c.pa(SP, 0));
    c.lqc2(9, c.pa(SP, 0x10));
    c.lqc2(10, c.pa(SP, 0x20));
    c.lqc2(11, c.pa(SP, 0x30));
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
    c.sqc2(12, c.pa(S2, 0));
    c.sqc2(13, c.pa(S2, 0x10));
    c.sqc2(14, c.pa(S2, 0x20));
    c.sqc2(15, c.pa(S2, 0x30));
    c.stw_2(S1, 0x6b90, 0u64);
    c.lq(16, c.pa(SP, 0x80));
    c.lq(17, c.pa(SP, 0x70));
    c.lq(18, c.pa(SP, 0x60));
    c.g[RA][0] = c.ldd(SP, 0x50);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = ra;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::base_2 as base;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_4;
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

    // ------------------------------------------------------------ DeferMan

    /// The chain of a random pass for `DeferMan_FindBucket`: `n` nodes from
    /// DATA+0x1000 (0x40 apart); the searched keys in a1/a2/t0. Node `hit`
    /// matches; the others differ in one key, are skipped (+0x14 set) or
    /// are random.
    pub(crate) fn bucket_chain(c: &mut Cpu, r: &mut Rng, n: u32, hit: Option<u32>) {
        let man = DATA;
        let pass = r.next_u32();
        let (hash, key1, key2) = (r.next_u32(), r.next_u32(), r.next_u32());
        c.g[A0][0] = sx(man);
        c.g[A1][0] = sx(hash);
        c.g[A2][0] = sx(key1);
        c.g[A3][0] = (r.next_u32() as u64) << 32 | pass as u64;
        c.g[T0][0] = sx(key2);
        let node = |i: u32| DATA + 0x1000 + i * 0x40;
        let slot_at = man + defer::BUCKETS + (pass & 0xff) * 4;
        c.bus.write32(slot_at, if n == 0 { 0 } else { node(0) });
        for i in 0..n {
            let at = node(i);
            fill(c, r, at, 0x10);
            let (mut h, mut k1, mut k2, mut skip) = (hash, key1, key2, 0);
            if hit != Some(i) {
                match r.next_u32() % 5 {
                    0 => h ^= 1 << (r.next_u32() % 32),
                    1 => k1 = r.next_u32(),
                    2 => k2 = r.next_u32(),
                    3 => skip = r.next_u32() | 1,
                    _ => (h, k1, k2) = (r.next_u32(), r.next_u32(), r.next_u32()),
                }
            }
            c.bus.write32(at, h);
            c.bus.write32(at + 4, k1);
            c.bus.write32(at + 8, k2);
            c.bus.write32(at + defer::NODE_SKIP, skip);
            c.bus.write32(
                at + defer::NODE_NEXT,
                if i + 1 < n { node(i + 1) } else { 0 },
            );
        }
    }

    #[test]
    fn defer_man_find_bucket_matches() {
        check(
            ssx3_ee::s13::f_00395000__generated,
            defer_man_find_bucket,
            300,
            &|c, seed| {
                let mut r = Rng(0x3950_0000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let n = [0, 1, 2, 3, 5, 8][(seed % 6) as usize];
                let hit = if n == 0 || seed % 7 == 0 {
                    None
                } else {
                    Some(r.next_u32() % n)
                };
                bucket_chain(c, &mut r, n, hit);
            },
        );
        check_events(
            ssx3_ee::s13::f_00395000__generated,
            defer_man_find_bucket,
            90,
            &|c, seed| {
                let mut r = Rng(0x3950_e000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let events = schedule(c, &mut r, seed, 0x0039_5000);
                let n = 4 + r.next_u32() % 6;
                let hit = if seed % 4 == 0 {
                    None
                } else {
                    Some(n - 1 - r.next_u32() % 2)
                };
                bucket_chain(c, &mut r, n, hit);
                events
            },
        );
    }

    /// The state chain of a random pass for `DeferMan_InternState`: `n`
    /// records from DATA+0x1000 (0x20 apart), the state at DATA+0x800 or
    /// +0x804 (unaligned doublewords), the pool count random. Record `hit`
    /// equals the state; others share a prefix of 0..3 words or are random.
    pub(crate) fn state_chain(c: &mut Cpu, r: &mut Rng, n: u32, hit: Option<u32>) {
        let man = DATA;
        let pass = r.next_u32();
        let state = DATA + 0x800 + 4 * (r.next_u32() % 2);
        fill(c, r, state, 5);
        c.g[A0][0] = sx(man);
        c.g[A1][0] = sx(state);
        c.g[A2][0] = (r.next_u32() as u64) << 32 | pass as u64;
        let count = r.next_u32() % 64;
        c.bus
            .write32(man + defer::STATE_POOL + defer::STATE_POOL_COUNT, count);
        let rec = |i: u32| DATA + 0x1000 + i * 0x20;
        let slot_at = man + defer::STATE_HEADS + (pass & 0xff) * 4;
        c.bus.write32(slot_at, if n == 0 { 0 } else { rec(0) });
        for i in 0..n {
            let at = rec(i);
            fill(c, r, at, 8);
            if hit == Some(i) || r.next_u32() % 2 == 0 {
                let words = if hit == Some(i) { 4 } else { r.next_u32() % 4 };
                for w in 0..words {
                    let v = c.bus.read32(state + 4 * w);
                    c.bus.write32(at + 4 * w, v);
                }
            }
            c.bus.write32(
                at + defer::STATE_NEXT,
                if i + 1 < n { rec(i + 1) } else { 0 },
            );
        }
    }

    #[test]
    fn defer_man_intern_state_matches() {
        check_lohi(
            ssx3_ee::s13::f_00394ed0__generated,
            defer_man_intern_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x394e_d000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let n = [0, 1, 2, 3, 5, 8][(seed % 6) as usize];
                let hit = if n == 0 || seed % 7 == 0 {
                    None
                } else {
                    Some(r.next_u32() % n)
                };
                state_chain(c, &mut r, n, hit);
            },
        );
        check_events(
            ssx3_ee::s13::f_00394ed0__generated,
            defer_man_intern_state,
            90,
            &|c, seed| {
                let mut r = Rng(0x394e_e000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let events = schedule(c, &mut r, seed, 0x0039_4ed0);
                let n = 3 + r.next_u32() % 5;
                let hit = if seed % 4 == 0 {
                    None
                } else {
                    Some(n - 1 - r.next_u32() % 2)
                };
                state_chain(c, &mut r, n, hit);
                events
            },
        );
    }

    #[test]
    fn ps2_graphics_man_copy_view_state_matches() {
        check(
            ee::s13::f_003950c0__generated,
            ps2_graphics_man_copy_view_state,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x39_50c0);
                let dst = DATA;
                let src = DATA + 0x4000;
                fill(c, &mut r, src, 0x100);
                fill(c, &mut r, dst, 0x100);
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = sx(src);
            },
        );
    }

    // --------------------------------------------------------- font FX trails

    #[test]
    fn fx_trail_reset_matches() {
        check(
            ee::s13::f_00392d18__generated,
            fx_trail_reset,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0039_2d18 ^ seed as u64);
                scramble(c, &mut r);
                let trail = DATA + 0x1000;
                c.g[A0][0] = trail as u64;
                // Old contents the reset must clear: the {x,y,z} cursor, the two
                // endpoint times, and the scratch doubleword on the stack.
                fill(c, &mut r, trail, 3);
                fill(c, &mut r, trail + 0xa0, 2);
                fill(c, &mut r, SP - 0x10, 4);
            },
        );
    }

    #[test]
    fn grid_mesh_base_call_v22_matches() {
        covered(
            s13::f_00394b98__generated,
            grid_mesh_base_call_v22,
            120,
            &|c, seed| {
                machine(c, seed, 0x394b98, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x200);
                slot(c, VT, 0xb0);
                c.bus.write32(OBJ + 4, VT);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed as u32);
                c.g[A2][0] = sx(seed.wrapping_mul(3));
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn light_man_free_instances_matches() {
        check(
            s13::f_00394ce8__generated,
            light_man_free_instances,
            120,
            &|c, seed| {
                machine(c, seed, 0x394ce8, 1);
                clear(c, OBJ, 0x40);
                c.g[A0][0] = sx(OBJ);
                let list = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(OBJ + 4, list);
            },
        );
    }

    #[test]
    fn grid_mesh_base_base_dtor_matches() {
        check(
            ee::s13::f_00394b38__generated,
            grid_mesh_base_base_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x394b_38),
        );
    }

    #[test]
    fn vfx_render_toggles_dtor_matches() {
        check(
            ee::s13::f_003961a8__generated,
            vfx_render_toggles_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3961_a8),
        );
    }

    #[test]
    fn vfx_plants_dtor_matches() {
        check(
            ee::s13::f_003964c8__generated,
            vfx_plants_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3964_c8),
        );
    }

    #[test]
    fn vfx_tube_dtor_matches() {
        check(
            ee::s13::f_00396838__generated,
            vfx_tube_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3968_38),
        );
    }

    #[test]
    fn path_arrow_v01_matches() {
        check(
            ee::s13::f_003968e8__generated,
            path_arrow_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3968_e8),
        );
    }

    #[test]
    fn detail_system_v01_matches() {
        check(
            ee::s13::f_00396928__generated,
            detail_system_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3969_28),
        );
    }

    #[test]
    fn vfx_visual_effects_test_dtor_matches() {
        check(
            ee::s13::f_00396a00__generated,
            vfx_visual_effects_test_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x396a_00),
        );
    }

    #[test]
    fn vfx_screen_tint_dtor_matches() {
        check(
            ee::s13::f_00396d30__generated,
            vfx_screen_tint_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x396d_30),
        );
    }

    // ------------------------------------------------------------ FX trail

    #[test]
    fn fxtrail_add_point_matches() {
        check(
            ee::s13::f_00392d90__generated,
            fxtrail_add_point,
            80,
            &|c, seed| {
                let mut r = Rng(0x392d_7a11 ^ seed as u64);
                scramble(c, &mut r);
                let t = DATA;
                c.bus.write32(t + 0xc, r.next_u32() % 4);
                fill(c, &mut r, t + 0x10, 32);
                fill(c, &mut r, t + 0x50, 64);
                let rot = DATA + 0x200;
                let pos = DATA + 0x300;
                fill(c, &mut r, rot, 4);
                fill(c, &mut r, pos, 4);
                c.g[A0][0] = sx(t);
                c.g[A1][0] = sx(rot);
                c.g[A2][0] = sx(pos);
            },
        );
    }

    #[test]
    fn light_man_init_instances_matches() {
        check(
            ee::s13::f_00394c98__generated,
            light_man_init_instances,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x394c_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }

    /// Write `words` at `addr`.
    pub(crate) fn words(c: &mut Cpu, addr: u32, v: &[u32]) {
        for (i, w) in v.iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, *w);
        }
    }

    pub(crate) fn rand_vec(c: &mut Cpu, r: &mut Rng, addr: u32) {
        put_vec(
            c,
            addr,
            [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
        );
    }

    #[test]
    fn graphics_man_set_screen_offset_matches() {
        check(
            ssx3_ee::s13::f_00393fb8__generated,
            graphics_man_set_screen_offset,
            160,
            &|c, seed| {
                let mut r = Rng(0x393f_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                words(c, DATA, &[r.next_u32(), r.next_u32()]);
            },
        );
    }

    #[test]
    fn graphics_man_get_screen_offset_x_matches() {
        check(
            ssx3_ee::s13::f_00393fd8__generated,
            graphics_man_get_screen_offset_x,
            200,
            &|c, seed| {
                let mut r = Rng(0x393f_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.edge_f32());
            },
        );
    }

    #[test]
    fn graphics_man_get_screen_offset_y_matches() {
        check(
            ssx3_ee::s13::f_00393ff0__generated,
            graphics_man_get_screen_offset_y,
            200,
            &|c, seed| {
                let mut r = Rng(0x393f_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 4, r.edge_f32());
            },
        );
    }

    #[test]
    fn graphics_man_v58_matches() {
        check(
            ssx3_ee::s13::f_003942a0__generated,
            graphics_man_v58,
            160,
            &|c, seed| {
                let mut r = Rng(0x3942_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                words(
                    c,
                    DATA,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
            },
        );
    }

    #[test]
    fn light_man_next_id_matches() {
        check(
            ssx3_ee::s13::f_00394d18__generated,
            light_man_next_id,
            200,
            &|c, seed| {
                let mut r = Rng(0x394d_1800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 8, r.next_u32());
                c.bus.write32(DATA + 0xc, r.next_u32() % 100);
            },
        );
    }

    pub(crate) fn screen_entry(c: &mut Cpu, r: &mut Rng, off: u32) {
        c.g[A0][0] = DATA as u64;
        let idx = r.next_u32() % 3;
        let entry = DATA + 0x400;
        c.bus.write32(DATA + 0x59c8, idx);
        c.bus.write32(DATA + 0x59cc, entry);
        c.bus.write32(entry + idx * 0xa4 + off, r.next_u32());
        c.bus.write32(entry + idx * 0xa4 + 0x88, r.next_u32());
        c.bus.write32(entry + idx * 0xa4 + 0x8c, r.next_u32());
    }

    #[test]
    fn ps2_graphics_man_get_screen_width_matches() {
        check(
            ssx3_ee::s13::f_00395330__generated,
            ps2_graphics_man_get_screen_width,
            160,
            &|c, seed| {
                let mut r = Rng(0x3953_3000 ^ seed as u64);
                scramble(c, &mut r);
                screen_entry(c, &mut r, 0x88);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_screen_height_matches() {
        check(
            ssx3_ee::s13::f_00395350__generated,
            ps2_graphics_man_get_screen_height,
            160,
            &|c, seed| {
                let mut r = Rng(0x3953_5000 ^ seed as u64);
                scramble(c, &mut r);
                screen_entry(c, &mut r, 0x8c);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_lighting_matches() {
        check_events(
            ssx3_ee::s13::f_003954d0__generated,
            ps2_graphics_man_set_lighting,
            120,
            &|c, seed| {
                let mut r = Rng(0x3954_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let src = DATA + 0x800;
                c.g[A1][0] = src as u64;
                for i in 0..10 {
                    rand_vec(c, &mut r, src + i * 0x10);
                }
                let mode = schedule(c, &mut r, seed, 0x0039_54d0);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn ps2_graphics_man_push_matrix_matches() {
        check(
            ssx3_ee::s13::f_003956e8__generated,
            ps2_graphics_man_push_matrix,
            160,
            &|c, seed| {
                let mut r = Rng(0x3956_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x13e0, r.next_u32() % 16);
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                    rand_vec(c, &mut r, top + 0x40 + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_load_matrix_matches() {
        check(
            ssx3_ee::s13::f_00395c38__generated,
            ps2_graphics_man_load_matrix,
            160,
            &|c, seed| {
                let mut r = Rng(0x395c_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                let m = DATA + 0x2000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                    rand_vec(c, &mut r, m + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_mult_matrix_matches() {
        check(
            ssx3_ee::s13::f_00395c70__generated,
            ps2_graphics_man_mult_matrix,
            160,
            &|c, seed| {
                let mut r = Rng(0x395c_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                let m = DATA + 0x2000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                    rand_vec(c, &mut r, m + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_load_identity_matches() {
        check(
            ssx3_ee::s13::f_00395cf0__generated,
            ps2_graphics_man_load_identity,
            160,
            &|c, seed| {
                let mut r = Rng(0x395c_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                // The .lit4 identity the original copies.
                put_vec(c, 0x004f_f1a0, [0x3f80_0000, 0, 0, 0]);
                put_vec(c, 0x004f_f1b0, [0, 0x3f80_0000, 0, 0]);
                put_vec(c, 0x004f_f1c0, [0, 0, 0x3f80_0000, 0]);
                put_vec(c, 0x004f_f1d0, [0, 0, 0, 0x3f80_0000]);
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_translate_matches() {
        check(
            ssx3_ee::s13::f_00395d28__generated,
            ps2_graphics_man_translate,
            160,
            &|c, seed| {
                let mut r = Rng(0x395d_2800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                let v = DATA + 0x2000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                }
                rand_vec(c, &mut r, v);
            },
        );
    }

    #[test]
    fn c_ps2_grid_mesh_set_scale_matches() {
        check(
            ssx3_ee::s13::f_00396110__generated,
            c_ps2_grid_mesh_set_scale,
            200,
            &|c, seed| {
                let mut r = Rng(0x3961_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP - 0x254c, 32767.0f32.to_bits());
                c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(DATA + 0x194, r.next_u32());
                c.bus.write32(DATA + 0x198, r.next_u32());
            },
        );
    }

    #[test]
    fn rail_tube_set_point_matches() {
        check(
            ssx3_ee::s13::f_003968d0__generated,
            rail_tube_set_point,
            160,
            &|c, seed| {
                let mut r = Rng(0x3968_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 8, DATA + 0x1000);
                rand_vec(c, &mut r, DATA + 0x2000);
                for i in 0..4 {
                    rand_vec(c, &mut r, DATA + 0x1000 + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn font_draw_text_f_matches() {
        check(
            ee::s12::f_00391c48__generated,
            font_draw_text_f,
            4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_1c48, 2);
                be_library_2(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    #[test]
    fn font_draw_text_centered_f_matches() {
        check(
            ee::s13::f_003929f8__generated,
            font_draw_text_centered_f,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x3929f8, 0);
                globals(c);
                // The empty-string sentinel: the local BXString's destructor then
                // skips the refcount path.
                w(c, GAME_GP + 0xda0, 0x0048_dcd4);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0;
                w(c, OBJ2, 0);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xc000_0000
                };
                c.f[13] = 0x4040_0000;
            },
        );
    }

    #[test]
    fn font_draw_text_right_f_matches() {
        check(
            ee::s13::f_00392b40__generated,
            font_draw_text_right_f,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x392b40, 0);
                globals(c);
                w(c, GAME_GP + 0xda0, 0x0048_dcd4);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0;
                w(c, OBJ2, 0);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xc000_0000
                };
                c.f[13] = 0x4040_0000;
            },
        );
    }

    // --------------------------------------------------------- rendering

    #[test]
    fn font_draw_text_right_matches() {
        check(
            ee::s13::f_00392a60__generated,
            font_draw_text_right,
            4,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0039_2a60, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b't');
                c.bus.write8(DATA + 0x4001, 0);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                w_2(c, OBJ + 0x38, r.edge_f32());
                w_2(c, OBJ + 0x3c, r.edge_f32());
            },
        );
    }

    #[test]
    fn renderer_build_sine_table_matches() {
        check(
            ee::s13::f_00392df0__generated,
            renderer_build_sine_table,
            4,
            &|c, seed| {
                let mut r = Rng(0x392d_f0e1 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                w_2(c, GAME_GP.wrapping_sub(0x2694), 0x3c49_0fdb);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // -------------------------------------------------------------------- fonts

    /// A glyph table whose count/max fields let both the "outside the range"
    /// and the search paths run: `+0xc` count, `+0x10` max code, `+0` first
    /// glyph, `+4` glyph table pointer, `+8` width table pointer.
    pub(crate) fn font_fixture(c: &mut Cpu, seed: u32, salt: u64, wide: bool) {
        let _ = base_3(c, seed, salt, 0);
        c.g[A0][0] = OBJ as u64;
        let ch = if wide {
            (seed % 0x60) as u64
        } else {
            ((seed % 0x60) as i8 as i32) as u32 as u64
        };
        c.g[A1][0] = ch;
        w_3(c, OBJ + 0xc, if seed % 5 == 0 { 0 } else { 100 });
        w_3(c, OBJ + 0x10, if seed % 3 == 0 { 0x10 } else { 0x200 });
        w_3(c, OBJ, OBJ2_2);
        w_3(c, OBJ + 4, OBJ2_2);
        w_3(c, OBJ + 8, OBJ3);
        // A sorted little glyph table at OBJ2_2 (codes) and widths at OBJ3.
        for i in 0..0x20u32 {
            c.bus.write16(OBJ2_2 + i * 2, (i * 2) as u16);
            c.bus.write8(OBJ3 + i, (i as u8).wrapping_add(1));
        }
    }

    #[test]
    fn font_get_char_width_matches() {
        check(
            ee::s13::f_00392ba8__generated,
            font_get_char_width,
            12,
            &|c, seed| font_fixture(c, seed, 0x0039_2ba8, false),
        );
    }

    #[test]
    fn font_get_char_width_w_matches() {
        check(
            ee::s13::f_00392c60__generated,
            font_get_char_width_w,
            12,
            &|c, seed| font_fixture(c, seed, 0x0039_2c60, true),
        );
    }

    #[test]
    fn ps2_graphics_man_rotate_axis_matches() {
        same(
            ssx3_ee::s13::f_00395d60__generated,
            ps2_graphics_man_rotate_axis,
            200,
            &|c, seed| {
                let mut r = Rng(0x395d60 ^ seed as u64);
                scramble(c, &mut r);
                let m = DATA + 0x100;
                let man = DATA + 0x2000;
                c.g[A0][0] = man as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(man + 0x13e4, m);
                put_vec(
                    c,
                    m,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x10,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x20,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x30,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for i in 0..4 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.edge_f32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }
}
