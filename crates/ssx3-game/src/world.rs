//! Work package D5: world, sections & process lists
//! (`docs/investigation/phase2-candidates-2.md`, structs in
//! `docs/investigation/structs/world.md`).
//!
//! The per-frame drivers of the course: the GameProcess priority lists
//! (update / post-update / render passes over one bucket, and the sorted
//! merge of newly created processes), the modifier blocks of world objects
//! (`ModObj`), the world streaming tick, the visible-model and Bézier-patch
//! render passes, and a few world queries (Nash spatial hash, logical
//! sections, the AI quadtree, fractal noise).
//!
//! The replacements follow the call protocol of [`crate::call`] (frame,
//! registers at each call, unwind check, cycles between calls) and the
//! structured-loop event rule of [`crate::events`]: every loop here is a
//! structured loop in the generated code, so a back edge delivers a due
//! event with `pc` at the loop head and carries on with the registers as
//! the event left them. Loop state therefore lives in the guest registers
//! the original uses (callee-saved `s*` across calls), read back after each
//! check.

use crate::abi::{addu, mult, r32, ret, set_word};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_50a0, process_list_render_priority, "ProcessList_RenderPriority"),
    (0x0035_4f98, process_list_update_priority, "ProcessList_UpdatePriority"),
    (0x0035_5028, process_list_post_update_priority, "ProcessList_PostUpdatePriority"),
    (0x0035_4ef8, process_list_next, "ProcessList_Next"),
    (0x0035_4f48, process_list_pending_first, "ProcessList_PendingFirst"),
    (0x0035_4c98, process_list_merge_pending, "ProcessList_MergePending"),
    (0x0023_0640, game_module_render_process_lists, "GameModule_RenderProcessLists"),
    (0x0022_c078, game_module_render_models, "GameModule_RenderModels"),
    (0x0022_c620, game_module_render_bezier_patches, "GameModule_RenderBezierPatches"),
    (0x0022_c3b8, game_module_end_bezier_pass, "GameModule_EndBezierPass"),
    (0x0035_2dd0, modifier_block_render, "ModifierBlock_Render"),
    (0x0035_2d20, modifier_block_post_update, "ModifierBlock_PostUpdate"),
    (0x0035_2c70, modifier_block_update, "ModifierBlock_Update"),
    (0x0035_2b88, modifier_block_get_transform_bounds, "ModifierBlock_GetTransformBounds"),
    (0x0035_6198, mod_obj_update, "ModObj_Update"),
    (0x0035_5748, mod_obj_post_update_modifiers, "ModObj_PostUpdateModifiers"),
    (0x0035_56f8, mod_obj_update_modifiers, "ModObj_UpdateModifiers"),
    (0x003a_6928, world_update, "World_Update"),
    (0x003a_8290, c_world_cache_update, "cWorldCache_Update"),
    (0x002d_1be0, world_get_nash, "World_GetNash"),
    (0x002d_1c70, world_get_seconds_per_tick, "World_GetSecondsPerTick"),
    (0x0022_e0e0, section_man_find_logical_section, "SectionMan_FindLogicalSection"),
    (0x002c_1cd8, quad_tree_find_leaf, "QuadTree_FindLeaf"),
    (0x002d_1928, noise_fractal_1d, "Noise_Fractal1D"),
    (0x0022_a5a0, nash_collect_instances, "Nash_CollectInstances"),
];

// ---------------------------------------------------------------------------
// Small register helpers.

/// `lw` through the CPU load path (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A gcc 2.9x virtual call `obj->vt[slot](obj + delta, ...)`: reads the
/// slot at `vt + slot` (`lh` delta, `lw` function), leaves the function in
/// `func_reg` and the adjusted `this` in a0, as the originals do, and calls
/// through the function table.
#[inline]
fn virtual_call(c: &mut Cpu, obj: u64, vt: u64, slot: u32, func_reg: usize, ret: u32) -> Flow {
    let m = VSlot::read(c, vt, slot);
    c.g[func_reg][0] = m.func;
    c.g[A0][0] = m.this(obj);
    call_ptr(c, m.func as u32, ret)
}

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const PROCESS_LIST_FIRST: Callee = Callee { addr: 0x0035_4ea8, f: ee::s11::ProcessList_First };
    pub const PROCESS_LIST_LAST: Callee = Callee { addr: 0x0035_4ed0, f: ee::s11::ProcessList_Last };
    pub const PROCESS_LIST_NEXT: Callee = Callee { addr: 0x0035_4ef8, f: ee::s11::ProcessList_Next };
    pub const PROCESS_LIST_PREV: Callee = Callee { addr: 0x0035_4f20, f: ee::s11::ProcessList_Prev };
    pub const PROCESS_LIST_PENDING_FIRST: Callee =
        Callee { addr: 0x0035_4f48, f: ee::s11::ProcessList_PendingFirst };
    pub const PROCESS_LIST_PENDING_NEXT: Callee =
        Callee { addr: 0x0035_4f70, f: ee::s11::ProcessList_PendingNext };

    pub const MODIFIER_BLOCK_UPDATE: Callee = Callee { addr: 0x0035_2c70, f: ee::s11::ModifierBlock_Update };
    pub const MODIFIER_BLOCK_POST_UPDATE: Callee =
        Callee { addr: 0x0035_2d20, f: ee::s11::ModifierBlock_PostUpdate };
    pub const MODIFIER_BLOCK_HAS_TRANSFORM_CHANGED: Callee =
        Callee { addr: 0x0035_2e50, f: ee::s11::ModifierBlock_HasTransformChanged };
    pub const MODIFIER_BLOCK_IS_TRANSFORM_FINISHED: Callee =
        Callee { addr: 0x0035_2ed0, f: ee::s11::ModifierBlock_IsTransformFinished };
    pub const MODIFIER_BLOCK_CLEAR_TRANSFORM_FINISHED: Callee =
        Callee { addr: 0x0035_2f08, f: ee::s11::ModifierBlock_ClearTransformFinished };
    pub const MODIFIER_BLOCK_FREE_BOOST: Callee =
        Callee { addr: 0x0035_3188, f: ee::s11::ModifierBlock_FreeBoost };
    pub const MOD_OBJ_UPDATE_MODIFIERS: Callee = Callee { addr: 0x0035_56f8, f: ee::s11::ModObj_UpdateModifiers };
    pub const MOD_OBJ_POST_UPDATE_MODIFIERS: Callee =
        Callee { addr: 0x0035_5748, f: ee::s11::ModObj_PostUpdateModifiers };
    /// Unnamed `ModObj` test at 0x34fce0 (non-zero skips the modifiers).
    pub const MOD_OBJ_34FCE0: Callee = Callee { addr: 0x0034_fce0, f: ee::s11::f_0034fce0 };

    pub const C_WORLD_CACHE_UPDATE: Callee = Callee { addr: 0x003a_8290, f: ee::s13::cWorldCache_Update };
    pub const C_WORLD_CACHE_STREAMER_UPDATE: Callee =
        Callee { addr: 0x003a_9258, f: ee::s13::cWorldCache_StreamerUpdate };
    pub const C_WORLD_CACHE_UPDATE_HULL_LRU: Callee =
        Callee { addr: 0x003a_8668, f: ee::s13::cWorldCache_UpdateHullLRU };
    pub const C_STREAM_MAN_UPDATE: Callee = Callee { addr: 0x003a_7098, f: ee::s13::cStreamMan_Update };

    pub const GAME_MODULE_BEGIN_BEZIER_PASS: Callee =
        Callee { addr: 0x0022_c1b0, f: ee::s05::GameModule_BeginBezierPass };
    pub const GAME_MODULE_DRAW_BEZIER_PATCH: Callee =
        Callee { addr: 0x0022_c410, f: ee::s05::GameModule_DrawBezierPatch };
    pub const GAME_MODULE_END_BEZIER_PASS: Callee =
        Callee { addr: 0x0022_c3b8, f: ee::s05::GameModule_EndBezierPass };

    pub const NOISE_SAMPLE_1D: Callee = Callee { addr: 0x002d_18b0, f: ee::s09::Noise_Sample1D };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
mod table_callee {
    pub const PROCESS_LIST_RENDER_PRIORITY: u32 = 0x0035_50a0;
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
    pub const UV_SCROLL_MODIFIER_UPDATE: u32 = 0x0035_f7d0;
    pub const TEX_FLIP_MODIFIER_UPDATE: u32 = 0x0035_f410;
}

// ---------------------------------------------------------------------------
// Globals.

/// gp offset (negative) of `g_App` (`SSXApp*`, 0x4a28a8).
const APP_GP: u32 = 0x848;
/// `SSXApp` +0x84: the `GameModule*`; +0x14: seconds per tick (f32).
const APP_GAME_MODULE: u32 = 0x84;
const APP_SECONDS_PER_TICK: u32 = 0x14;
/// `GameModule` +0x20: the Nash spatial hash.
const GAME_MODULE_NASH: u32 = 0x20;
/// gp offset (negative) of the graphics object (`PS2GraphicsMan*`, 0x4a289c).
const GRAPHICS_GP: u32 = 0x854;
/// gp offset of the `PS2BezierMan` instance pointer.
const BEZIER_MAN_GP: u32 = 0x1158;
/// gp offset of `g_ProcessList` (the GameProcess priority list).
const PROCESS_LIST_GP: u32 = 0x2898;
/// The six priorities `GameModule_RenderProcessLists` renders, in order.
const RENDER_PRIORITIES: u32 = 0x0044_28d8;
/// SectionMan's logical-section table, 0x10 bytes per entry; the search
/// starts at +4 of the first entry (`0x442168 + 4`).
const LOGICAL_SECTIONS: u32 = 0x0044_2168;
const LOGICAL_SECTION_COUNT: u64 = 50;

#[inline(always)]
fn gp_minus(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

// ---------------------------------------------------------------------------
// ProcessList.

/// `ProcessList { int nPriorities; Bucket* buckets; }`; a bucket is 0x44
/// bytes: flags, then four sentinel `GameProcess`es framing the active list
/// (head +0x04, tail +0x14) and the pending list (head +0x24, tail +0x34).
mod process_list {
    pub const BUCKETS: u32 = 0x04;
    pub const BUCKET_SIZE: u32 = 0x44;
    pub const ACTIVE_HEAD: u32 = 0x04;
    pub const ACTIVE_TAIL: u32 = 0x14;
    pub const PENDING_HEAD: u32 = 0x24;
    pub const PENDING_TAIL: u32 = 0x34;
}

/// `GameProcess` links and vtable pointer.
mod game_process {
    pub const NEXT: u32 = 0x04;
    pub const PREV: u32 = 0x08;
    pub const VPTR: u32 = 0x0c;
    /// vtable entries (offsets of the gcc 2.9x `{delta, fn}` slots).
    pub const SLOT_DTOR: u32 = 0x08;
    pub const SLOT_UPDATE: u32 = 0x10;
    pub const SLOT_POST_UPDATE: u32 = 0x18;
    pub const SLOT_RENDER: u32 = 0x20;
    pub const SLOT_ID_LESS_EQUAL: u32 = 0x30;
    pub const SLOT_ID_EQUALS: u32 = 0x38;
}

/// `ProcessList_Next(list, process, priority)` (0x354ef8): the process
/// after `process` in the priority's active list, or null at the tail
/// sentinel. 9 cycles; leaves `priority * 0x44` in a2 (and LO/HI) and the
/// sentinel test (`next ^ tail`) in v1.
pub fn process_list_next(c: &mut Cpu) {
    use process_list::*;
    c.cycles += 9;
    let buckets = lw(c, at(c, A0, BUCKETS));
    let bucket_off = mult(c, r32(c, A2), BUCKET_SIZE);
    set_word(c, A2, bucket_off);
    let next = lw(c, at(c, A1, game_process::NEXT));
    let tail = addu(addu(buckets, bucket_off as u64), ACTIVE_TAIL as u64);
    let test = next ^ tail;
    c.g[V1][0] = test;
    c.g[V0][0] = if test == 0 { 0 } else { next };
    ret(c);
}

/// `ProcessList_PendingFirst(list, priority)` (0x354f48): the first
/// process waiting in the priority's pending list, or null. 9 cycles;
/// leaves the buckets pointer in v1, the sentinel test in a1, LO/HI.
pub fn process_list_pending_first(c: &mut Cpu) {
    use process_list::*;
    c.cycles += 9;
    let buckets = lw(c, at(c, A0, BUCKETS));
    c.g[V1][0] = buckets;
    let bucket_off = mult(c, r32(c, A1), BUCKET_SIZE);
    let bucket = addu(bucket_off as u64, buckets) as u32;
    let first = lw(c, bucket.wrapping_add(PENDING_HEAD + game_process::NEXT));
    let test = first ^ sx(bucket.wrapping_add(PENDING_TAIL));
    c.g[A1][0] = test;
    c.g[V0][0] = if test == 0 { 0 } else { first };
    ret(c);
}

/// Return addresses and loop head of one "call a method on every process"
/// pass.
struct PassAddrs {
    after_first: u32,
    after_method: u32,
    after_next: u32,
    head: u32,
}

/// The shape of `ProcessList_PostUpdatePriority` / `_RenderPriority`:
/// call vtable entry `slot` on each process of the priority's active list,
/// then step with `ProcessList_Next`. 0x40 frame (s0-s2).
fn call_on_each_process(c: &mut Cpu, slot: u32, at_: &PassAddrs) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A1][0]; // priority
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A0][0]; // list
    callee::PROCESS_LIST_FIRST.call(c, at_.after_first)?;
    c.g[S0][0] = c.g[V0][0]; // process
    if c.g[S0][0] == 0 {
        frame.restore(c, S0, 0x30);
    } else {
        c.cycles += 1;
        c.g[V0][0] = lw(c, at(c, S0, game_process::VPTR));
        loop {
            c.cycles += 11;
            let (process, vt) = (c.g[S0][0], c.g[V0][0]);
            virtual_call(c, process, vt, slot, V1, at_.after_method)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            callee::PROCESS_LIST_NEXT.call(c, at_.after_next)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                break;
            }
            c.g[V0][0] = lw(c, at(c, S0, game_process::VPTR));
            if event_due(c) {
                deliver_event(c, at_.head);
            }
        }
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `ProcessList_PostUpdatePriority(list, priority)` (0x355028): calls
/// `PostUpdate` (vtable slot 3) on every active process of the priority.
pub fn process_list_post_update_priority(c: &mut Cpu) {
    guest(c, |c| {
        call_on_each_process(
            c,
            game_process::SLOT_POST_UPDATE,
            &PassAddrs { after_first: 0x0035_5048, after_method: 0x0035_5068, after_next: 0x0035_5078, head: 0x0035_5058 },
        )
    })
}

/// `ProcessList_RenderPriority(list, priority)` (0x3550a0): calls
/// `Render` (vtable slot 4) on every active process of the priority.
pub fn process_list_render_priority(c: &mut Cpu) {
    guest(c, |c| {
        call_on_each_process(
            c,
            game_process::SLOT_RENDER,
            &PassAddrs { after_first: 0x0035_50c0, after_method: 0x0035_50e0, after_next: 0x0035_50f0, head: 0x0035_50d0 },
        )
    })
}

/// `ProcessList_UpdatePriority(list, priority)` (0x354f98): calls `Update`
/// (vtable slot 2) on every active process of the priority. The successor
/// is fetched before the call, so a process may unlink itself. Ends with a
/// `ProcessList_First` whose result is unused. 0x50 frame (s0-s3).
pub fn process_list_update_priority(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x0035_4fd0;
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0]; // list
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0]; // priority
        callee::PROCESS_LIST_FIRST.call(c, 0x0035_4fbc)?;
        c.g[S1][0] = c.g[V0][0]; // process
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 12;
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                callee::PROCESS_LIST_NEXT.call(c, 0x0035_4fdc)?;
                let vt = lw(c, at(c, S1, game_process::VPTR));
                c.g[V1][0] = vt;
                c.g[S0][0] = c.g[V0][0]; // next
                let process = c.g[S1][0];
                virtual_call(c, process, vt, game_process::SLOT_UPDATE, V0, 0x0035_4ff4)?;
                c.g[S1][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if c.g[S1][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event(c, HEAD);
                }
            }
        }
        c.g[A0][0] = c.g[S2][0];
        c.cycles += 9;
        c.g[A1][0] = c.g[S3][0];
        callee::PROCESS_LIST_FIRST.call(c, 0x0035_500c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ProcessList_MergePending(list, priority)` (0x354c98): moves every
/// pending process of the priority into the active list, sorted by sync
/// id. Each one is unlinked from the pending list, then the active list is
/// walked backwards from the tail: the first process `q` with
/// `p->IdLessEqual(q)` (slot 6) gets `p` after it, unless `p->IdEquals(q)`
/// (slot 7), in which case `p` takes `q`'s place and `q` is deleted
/// (slot 1 with flag 3). A process that sorts before all is put at the
/// head. 0x70 frame (s0-s5).
pub fn process_list_merge_pending(c: &mut Cpu) {
    use game_process::*;
    guest(c, |c| {
        const OUTER_HEAD: u32 = 0x0035_4cd8;
        const INNER_HEAD: u32 = 0x0035_4d20;
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0]; // list
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0]; // priority
        callee::PROCESS_LIST_PENDING_FIRST.call(c, 0x0035_4cc4)?;
        c.g[S1][0] = c.g[V0][0]; // p: the pending process
        c.g[V0][0] = process_list::BUCKET_SIZE as u64;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            let bucket_off = mult(c, r32(c, S3), process_list::BUCKET_SIZE);
            set_word(c, S4, bucket_off);
            c.g[A2][0] = c.g[S3][0];
            loop {
                c.cycles += 16;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                callee::PROCESS_LIST_PENDING_NEXT.call(c, 0x0035_4ce4)?;
                c.g[S5][0] = c.g[V0][0]; // the next pending process
                // Unlink p from the pending list.
                let prev = lw(c, at(c, S1, PREV));
                c.g[A2][0] = prev;
                let next = lw(c, at(c, S1, NEXT));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                c.sw((prev as u32).wrapping_add(NEXT), next);
                let next = lw(c, at(c, S1, NEXT));
                c.g[V1][0] = next;
                let prev = lw(c, at(c, S1, PREV));
                c.g[V0][0] = prev;
                c.sw((next as u32).wrapping_add(PREV), prev);
                callee::PROCESS_LIST_LAST.call(c, 0x0035_4d0c)?;
                c.g[S0][0] = c.g[V0][0]; // q: candidate predecessor
                let at_head = if c.g[S0][0] == 0 {
                    true
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = lw(c, at(c, S1, VPTR));
                    loop {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S0][0];
                        let (p, vt) = (c.g[S1][0], c.g[V1][0]);
                        virtual_call(c, p, vt, SLOT_ID_LESS_EQUAL, V0, 0x0035_4d34)?;
                        let after_q = c.g[V0][0] != 0;
                        c.g[A1][0] = c.g[S0][0];
                        if after_q {
                            c.cycles += 7;
                            let vt = lw(c, at(c, S1, VPTR));
                            c.g[V1][0] = vt;
                            let p = c.g[S1][0];
                            virtual_call(c, p, vt, SLOT_ID_EQUALS, V0, 0x0035_4d50)?;
                            let same_id = c.g[V0][0] != 0;
                            let (p, q) = (r32(c, S1), r32(c, S0));
                            c.g[V0][0] = lw(c, q.wrapping_add(NEXT));
                            if same_id {
                                // p replaces q, which is deleted.
                                c.cycles += 10;
                                c.sw(p.wrapping_add(NEXT), c.g[V0][0]);
                                c.g[V1][0] = lw(c, q.wrapping_add(NEXT));
                                c.sw((c.g[V1][0] as u32).wrapping_add(PREV), c.g[S1][0]);
                                c.g[V0][0] = lw(c, q.wrapping_add(PREV));
                                c.sw(p.wrapping_add(PREV), c.g[V0][0]);
                                c.g[V1][0] = lw(c, q.wrapping_add(PREV));
                                c.sw((c.g[V1][0] as u32).wrapping_add(NEXT), c.g[S1][0]);
                                c.sw(q.wrapping_add(NEXT), 0);
                                c.sw(q.wrapping_add(PREV), 0);
                                // (q is never null here.)
                                c.cycles += 8;
                                let vt = lw(c, q.wrapping_add(VPTR));
                                c.g[V0][0] = vt;
                                c.g[A1][0] = 3;
                                let q = c.g[S0][0];
                                virtual_call(c, q, vt, SLOT_DTOR, V1, 0x0035_4d98)?;
                            } else {
                                // Insert p after q.
                                c.cycles += 6;
                                c.sw(p.wrapping_add(NEXT), c.g[V0][0]);
                                c.g[V1][0] = lw(c, q.wrapping_add(NEXT));
                                c.sw((c.g[V1][0] as u32).wrapping_add(PREV), c.g[S1][0]);
                                c.sw(p.wrapping_add(PREV), c.g[S0][0]);
                                c.sw(q.wrapping_add(NEXT), c.g[S1][0]);
                            }
                            break;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        callee::PROCESS_LIST_PREV.call(c, 0x0035_4dc4)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[V1][0] = lw(c, at(c, S1, VPTR));
                        if event_due(c) {
                            deliver_event(c, INNER_HEAD);
                        }
                    }
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        false
                    } else {
                        c.cycles += 1;
                        true
                    }
                };
                if at_head {
                    // Insert p at the head of the active list.
                    // (The bucket address is recomputed from the list each time.)
                    use process_list::ACTIVE_HEAD;
                    let p = c.g[S1][0];
                    let pa = p as u32;
                    let buckets = lw(c, at(c, S2, process_list::BUCKETS));
                    c.cycles += 15;
                    let bucket = addu(c.g[S4][0], buckets) as u32;
                    c.g[V0][0] = sx(bucket);
                    let first = lw(c, bucket.wrapping_add(ACTIVE_HEAD + NEXT));
                    c.g[V1][0] = first;
                    c.sw(pa.wrapping_add(NEXT), first);
                    let bucket = bucket_of(c, S4);
                    c.g[V0][0] = sx(bucket);
                    let first = lw(c, bucket.wrapping_add(ACTIVE_HEAD + NEXT));
                    c.g[V1][0] = first;
                    c.sw((first as u32).wrapping_add(PREV), p);
                    let head = bucket_of(c, S4).wrapping_add(ACTIVE_HEAD);
                    c.g[V0][0] = sx(head);
                    c.sw(pa.wrapping_add(PREV), sx(head));
                    let bucket = bucket_of(c, S4);
                    c.g[V1][0] = sx(bucket);
                    c.sw(bucket.wrapping_add(ACTIVE_HEAD + NEXT), p);
                }
                c.g[S1][0] = c.g[S5][0];
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[A2][0] = c.g[S3][0];
                if event_due(c) {
                    deliver_event(c, OUTER_HEAD);
                }
            }
        }
        c.cycles += 9;
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

/// `buckets + bucket_off` with the buckets pointer re-read from the list
/// in s2 and the offset in `off_reg` (the merge re-reads it each time).
#[inline]
fn bucket_of(c: &mut Cpu, off_reg: usize) -> u32 {
    let buckets = lw(c, at(c, S2, process_list::BUCKETS));
    addu(c.g[off_reg][0], buckets) as u32
}

/// `GameModule_RenderProcessLists()` (0x230640): `ProcessList_RenderPriority`
/// on `g_ProcessList` for each of the six priorities in the render-order
/// table. 0x40 frame (s0-s2).
pub fn game_module_render_process_lists(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x0023_0668;
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0044_0000;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 5; // remaining after this one
        frame.save(c, S2, 0x10);
        c.g[S1][0] = RENDER_PRIORITIES as u64;
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(r32(c, GP).wrapping_add(PROCESS_LIST_GP));
        c.g[A1][0] = lw(c, r32(c, S1));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            call_ptr(c, table_callee::PROCESS_LIST_RENDER_PRIORITY, 0x0023_0678)?;
            if (c.g[S0][0] as i64) < 0 {
                break;
            }
            c.g[A1][0] = lw(c, r32(c, S1));
            if event_due(c) {
                deliver_event(c, HEAD);
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
// World streaming and world queries.

/// `World_Update(world)` (0x3a6928): ticks the world cache
/// (`cWorldCache_Update(world->cache)`). 7 cycles.
pub fn world_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, r32(c, A0));
        callee::C_WORLD_CACHE_UPDATE.call(c, 0x003a_6938)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWorldCache_Update(cache)` (0x3a8290): the per-frame streaming tick:
/// the section streamer (+0x10), the hull-page LRU, and the stream manager
/// (+0x300). 14 cycles.
pub fn c_world_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(at(c, S0, 0x10));
        callee::C_WORLD_CACHE_STREAMER_UPDATE.call(c, 0x003a_82a8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_WORLD_CACHE_UPDATE_HULL_LRU.call(c, 0x003a_82b0)?;
        c.g[A0][0] = sx(at(c, S0, 0x300));
        callee::C_STREAM_MAN_UPDATE.call(c, 0x003a_82b8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `World_GetNash()` (0x2d1be0): `g_App->gameModule->nash`. 4 cycles;
/// leaves the GameModule in v1.
pub fn world_get_nash(c: &mut Cpu) {
    c.cycles += 4;
    let app = lw(c, gp_minus(c, APP_GP));
    c.g[V0][0] = app;
    let module = lw(c, (app as u32).wrapping_add(APP_GAME_MODULE));
    c.g[V1][0] = module;
    c.g[V0][0] = lw(c, (module as u32).wrapping_add(GAME_MODULE_NASH));
    ret(c);
}

/// `World_GetSecondsPerTick()` (0x2d1c70): `g_App->secondsPerTick` in f0.
/// 3 cycles; leaves `g_App` in v0.
pub fn world_get_seconds_per_tick(c: &mut Cpu) {
    c.cycles += 3;
    let app = lw(c, gp_minus(c, APP_GP));
    c.g[V0][0] = app;
    c.f[0] = c.bus.read32((app as u32).wrapping_add(APP_SECONDS_PER_TICK));
    ret(c);
}

/// `SectionMan_FindLogicalSection(this, worldSection)` (0x22e0e0): the
/// logical id (0..49) whose table entry holds `worldSection` (compared as
/// a whole 64-bit register), or 50. Leaves the index in v1 and the entry
/// pointer in a0.
pub fn section_man_find_logical_section(c: &mut Cpu) {
    const HEAD: u32 = 0x0022_e0f0;
    c.cycles += 4;
    c.g[V0][0] = sx(LOGICAL_SECTIONS);
    c.g[V1][0] = 0;
    c.g[A0][0] = sx(LOGICAL_SECTIONS + 4);
    loop {
        c.cycles += 3;
        c.g[V0][0] = lw(c, r32(c, A0));
        if c.g[A1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            break;
        }
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        c.cycles += 3;
        let more = (c.g[V1][0] as i64) < LOGICAL_SECTION_COUNT as i64;
        c.g[V0][0] = more as u64;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = LOGICAL_SECTION_COUNT;
            break;
        }
        if event_due(c) {
            deliver_event(c, HEAD);
        }
    }
    ret(c);
}

/// `QuadTree_FindLeaf(tree, x, z)` (0x2c1cd8, x/z in f12/f13): the leaf
/// node containing the point, or `&tree->outside` (+0x18) when the point
/// maps outside the 15-bit grid. Cell = `(int)((p - origin) * invCell)`
/// (origin +4/+8, inverse cell size +0); nodes (8 bytes, `u16` flags/child
/// index, bit 0 = has children) are at `tree->nodes` (+0x20), the root at
/// index `tree->root` (+0x14). Each level picks one of four children from
/// the next bit of the cell coordinates. Returns null if the final node
/// is not a leaf.
pub fn quad_tree_find_leaf(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_1d50;
    c.cycles += 15;
    let tree = r32(c, A0);
    let origin_x = FpuF32(c.bus.read32(tree.wrapping_add(4)));
    c.f[1] = origin_x.0;
    c.g[A2][0] = 0x7fff;
    let origin_z = FpuF32(c.bus.read32(tree.wrapping_add(8)));
    c.f[0] = origin_z.0;
    let x = FpuF32(c.f[12]) - origin_x;
    let inv_cell = FpuF32(c.bus.read32(tree));
    c.f[2] = inv_cell.0;
    let z = FpuF32(c.f[13]) - origin_z;
    let x = x * inv_cell;
    let z = z * inv_cell;
    c.f[12] = x.0;
    c.f[13] = z.0;
    let cell_x = x.to_i32() as u32;
    c.g[V1][0] = sx(cell_x);
    let cell_z = z.to_i32() as u32;
    c.f[0] = cell_z;
    c.g[A1][0] = sx(cell_z);
    // Unsigned 64-bit compares of the sign-extended cells: negatives fail.
    let x_out = 0x7fff < c.g[V1][0];
    let z_out = 0x7fff < c.g[A1][0];
    c.g[V0][0] = z_out as u64;
    if x_out || {
        c.cycles += 2;
        z_out
    } {
        c.cycles += 2;
        c.g[V0][0] = sx(tree.wrapping_add(0x18));
        ret(c);
        return;
    }
    let root = c.lhu(tree.wrapping_add(0x14));
    c.cycles += 10;
    let mut bits_x = (cell_x << 1) & 0xffff;
    c.g[V1][0] = sx(cell_x << 1);
    let mut nodes = lw(c, tree.wrapping_add(0x20));
    c.g[T0][0] = nodes;
    c.g[A3][0] = bits_x as u64;
    c.g[V0][0] = sx((root as u32) << 3);
    c.g[A0][0] = sx(cell_z << 1);
    let mut node = (nodes as u32).wrapping_add((root as u32) << 3);
    c.g[A2][0] = sx(node);
    let flags = c.lhu(node);
    c.g[V0][0] = flags & 1;
    let mut bits_z = (cell_z << 1) & 0xffff;
    c.g[A1][0] = bits_z as u64;
    if flags & 1 != 0 {
        c.cycles += 1;
        loop {
            c.cycles += 17;
            let quadrant = (((bits_x >> 15) << 1) | (bits_z >> 15)) << 1;
            c.g[V0][0] = sx(bits_z >> 15);
            c.g[A0][0] = sx(bits_x << 1);
            bits_z <<= 1;
            bits_x = (bits_x << 1) & 0xfffc;
            c.g[A3][0] = bits_x as u64;
            let link = node.wrapping_add(quadrant);
            c.g[V1][0] = sx(link);
            let child = c.lhu(link) as u32;
            c.g[V0][0] = sx((child >> 1) << 3);
            node = (nodes as u32).wrapping_add((child >> 1) << 3);
            c.g[A2][0] = sx(node);
            let flags = c.lhu(node) & 1;
            c.g[V1][0] = flags;
            bits_z &= 0xfffc;
            c.g[A1][0] = bits_z as u64;
            if flags == 0 {
                break;
            }
            if event_due(c) {
                deliver_event(c, HEAD);
                bits_x = r32(c, A3);
                bits_z = r32(c, A1);
                node = r32(c, A2);
                nodes = c.g[T0][0];
            }
        }
    }
    c.cycles += 5;
    let leaf = c.lhu(node) & 1;
    c.g[V1][0] = leaf;
    c.g[V0][0] = if leaf == 0 { c.g[A2][0] } else { 0 };
    ret(c);
}

/// `Noise_Fractal1D(octaves, x)` (0x2d1928, x in f12): the sum over
/// octaves of `Noise_Sample1D(x * 2^i) * 0.5^i`, returned in f0. (The
/// last octave's sample is added once more at the end, as the original
/// does.) FPU arithmetic; f20-f24 saved in the 0x50 frame.
pub fn noise_fractal_1d(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x002d_1970;
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        let save_f = |c: &mut Cpu, f: usize, off: u32| {
            let a = frame.at(c, off);
            c.bus.write32(a, c.f[f]);
        };
        save_f(c, 22, 0x30);
        save_f(c, 21, 0x28);
        c.f[22] = 0; // sum
        c.g[AT][0] = 0x3f80_0000;
        c.f[21] = FpuF32::ONE.0; // amplitude
        save_f(c, 23, 0x38);
        c.f[0] = c.f[22];
        save_f(c, 20, 0x20);
        c.f[23] = c.f[12]; // x
        frame.save(c, S0, 0x10);
        c.f[20] = c.f[21]; // frequency
        frame.save_ra(c, 0);
        save_f(c, 24, 0x40);
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[AT][0] = 0x3f00_0000;
            c.f[24] = 0x3f00_0000; // 0.5
            c.g[S0][0] = c.g[A0][0];
            loop {
                c.cycles += 8;
                c.f[12] = (FpuF32(c.f[23]) * FpuF32(c.f[20])).0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                c.f[20] = (FpuF32(c.f[20]) + FpuF32(c.f[20])).0;
                callee::NOISE_SAMPLE_1D.call(c, 0x002d_1980)?;
                c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[21])).0;
                c.f[21] = (FpuF32(c.f[21]) * FpuF32(c.f[24])).0;
                c.f[22] = (FpuF32(c.f[22]) + FpuF32(c.f[0])).0;
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event(c, HEAD);
                }
            }
        }
        c.cycles += 10;
        c.f[0] = (FpuF32(c.f[22]) + FpuF32(c.f[0])).0;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        for (f, off) in [(24, 0x40), (23, 0x38), (22, 0x30), (21, 0x28), (20, 0x20)] {
            let a = frame.at(c, off);
            c.f[f] = c.bus.read32(a);
        }
        frame.close(c);
        Ok(())
    })
}

/// `Nash_CollectInstances(this, nodes, count)` (0x22a5a0): for each of the
/// `count` visited octree nodes (`{Node* node; int clipped}` pairs), walks
/// the node's instance list (+0x20; instance: next +0, kind +8, section
/// byte +0x7d, logical section +0x7e, bounding sphere +0x50) and appends
/// the visible instances to the render list at `this + 0x20a0` (count
/// +0x209c). Instances of kind 3 must belong to an active section
/// (section 0xff = always; else the section's state +0x24 must be 6 and
/// the logical section's state +0x3f0 must be 3; tables from
/// `*(this + 0x90)`). Instances of clipped nodes are culled by the VU0
/// sphere test microprogram at 0xef0 (`vi1 == 2` = outside).
pub fn nash_collect_instances(c: &mut Cpu) {
    const NODE_HEAD: u32 = 0x0022_a5a8;
    const INSTANCE_HEAD: u32 = 0x0022_a5d8;
    c.cycles += 2;
    if (c.g[A2][0] as i64) > 0 {
        loop {
            c.cycles += 5;
            let node = lw(c, r32(c, A1));
            c.g[V0][0] = node;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[T0][0] = lw(c, (node as u32).wrapping_add(0x20)); // instance
            c.g[T3][0] = sx(at(c, A1, 8));
            if c.g[T0][0] != 0 {
                c.cycles += 7;
                c.g[T7][0] = 3;
                c.g[T6][0] = 0xff;
                c.g[T5][0] = 0x18;
                c.g[T4][0] = 2;
                c.g[T2][0] = sx(at(c, A0, 0x20a0));
                c.g[V0][0] = lw(c, at(c, T0, 8));
                loop {
                    collect_instance(c);
                    c.cycles += 2;
                    if c.g[T0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = lw(c, at(c, T0, 8));
                    if event_due(c) {
                        deliver_event(c, INSTANCE_HEAD);
                    }
                }
            }
            c.cycles += 2;
            let more = (c.g[A2][0] as i64) > 0;
            c.g[A1][0] = c.g[T3][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event(c, NODE_HEAD);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// One instance of `Nash_CollectInstances` (instance in t0, its kind word
/// in v0): the section filter, the cull test and the append; leaves t0 at
/// the next instance.
fn collect_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] &= 3;
    let sectioned = c.g[V0][0] == c.g[T7][0];
    c.g[T1][0] = lw(c, at(c, A1, 4)); // the node's "clipped" flag
    if sectioned {
        c.cycles += 5;
        let tables = lw(c, at(c, A0, 0x90));
        c.g[V0][0] = tables;
        let section = c.lbu(at(c, T0, 0x7d));
        c.g[V1][0] = section;
        c.g[A3][0] = lw(c, tables as u32);
        c.g[V0][0] = 1;
        if section != c.g[T6][0] {
            c.cycles += 7;
            let section = c.lbu(at(c, T0, 0x7d));
            c.g[V0][0] = sx((section as u32) << 3);
            let entry = addu(c.g[A3][0], c.g[V0][0]);
            c.g[V0][0] = entry;
            let state = lw(c, (entry as u32).wrapping_add(0x24)) ^ 6;
            c.g[V1][0] = state;
            if state != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                let logical = c.lh(at(c, T0, 0x7e));
                c.g[V0][0] = logical;
                let off = mult(c, logical as u32, 0x18);
                set_word(c, V1, off);
                let entry = addu(c.g[V1][0], c.g[A3][0]);
                c.g[V0][0] = entry;
                let state = lw(c, (entry as u32).wrapping_add(0x3f0)) ^ 3;
                c.g[V1][0] = state;
                c.g[V0][0] = (state < 1) as u64;
            }
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        let clipped = c.g[T1][0] != 0;
        c.g[V0][0] = sx(at(c, T0, 0x50));
        let culled = clipped && {
            c.cycles += 7;
            c.lqc2(22, r32(c, V0));
            c.ctc2(T1, 14);
            c.vcallms(0xef0);
            c.cfc2(V1, 1);
            c.g[V1][0] == c.g[T4][0]
        };
        if !culled {
            c.cycles += 7;
            let n = lw(c, at(c, A0, 0x209c));
            c.g[V0][0] = sx((n as u32) << 2);
            c.g[V0][0] = addu(c.g[T2][0], c.g[V0][0]);
            c.g[V1][0] = sx((n as u32).wrapping_add(1));
            c.sw(r32(c, V0), c.g[T0][0]);
            c.sw(at(c, A0, 0x209c), c.g[V1][0]);
        }
    }
    c.g[T0][0] = lw(c, r32(c, T0));
}

// ---------------------------------------------------------------------------
// Modifier blocks (`ModObj` world objects).

/// `ModifierBlock`: the modifiers of a `ModObj`. +0 transform modifier
/// (vptr at +0), +4 UV scroll, +8 texture flip, +0xc boost (vptr at +4),
/// +0x10 particle-modifier list, +0x1c rail-modifier list (nodes: next at
/// +0, vptr at +8).
mod modifier_block {
    pub const TRANSFORM: u32 = 0x00;
    pub const UV_SCROLL: u32 = 0x04;
    pub const TEX_FLIP: u32 = 0x08;
    pub const BOOST: u32 = 0x0c;
    pub const PARTICLES: u32 = 0x10;
    pub const RAILS: u32 = 0x1c;
    /// List node: next at +0, vptr at +8.
    pub const NODE_VPTR: u32 = 0x08;
}

/// Walk a modifier list (head pointer already in s0 and a1), calling
/// vtable entry `slot` on each node; the successor is read before the call.
/// `enter` = the extra cycles of the non-empty entry block.
fn call_on_modifier_list(c: &mut Cpu, enter: u64, slot: u32, ret_addr: u32, head: u32) -> Flow {
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += enter;
    c.g[V0][0] = lw(c, at(c, A1, modifier_block::NODE_VPTR));
    loop {
        c.cycles += 8;
        c.g[S0][0] = lw(c, r32(c, A1));
        let (node, vt) = (c.g[A1][0], c.g[V0][0]);
        virtual_call(c, node, vt, slot, V1, ret_addr)?;
        c.g[A1][0] = c.g[S0][0];
        if c.g[A1][0] == 0 {
            return Ok(());
        }
        c.g[V0][0] = lw(c, at(c, A1, modifier_block::NODE_VPTR));
        if event_due(c) {
            deliver_event(c, head);
        }
    }
}

/// The transform modifier call that ends `Render`/`PostUpdate`: if the
/// block has one, call its vtable entry `slot` (s0 is restored first, in
/// the branch delay slot, and again after the call).
fn call_transform_tail(c: &mut Cpu, frame: Frame, slot: u32, ret_addr: u32) -> Flow {
    c.cycles += 3;
    c.g[A1][0] = lw(c, at(c, S1, modifier_block::TRANSFORM));
    frame.restore(c, S0, 0x20);
    if c.g[A1][0] != 0 {
        c.cycles += 6;
        let vt = lw(c, r32(c, A1));
        c.g[V0][0] = vt;
        let transform = c.g[A1][0];
        virtual_call(c, transform, vt, slot, V1, ret_addr)?;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `ModifierBlock_Render(block)` (0x352dd0): `Render` (slot 6) on each
/// particle modifier, then the transform modifier's slot 11.
pub fn modifier_block_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, at(c, S1, modifier_block::PARTICLES));
        c.g[A1][0] = c.g[S0][0];
        call_on_modifier_list(c, 2, 0x30, 0x0035_2e0c, 0x0035_2df8)?;
        call_transform_tail(c, frame, 0x58, 0x0035_2e38)
    })
}

/// `ModifierBlock_PostUpdate(block)` (0x352d20): `Update` (slot 5) on each
/// particle modifier, slot 3 on each rail modifier, then the transform
/// modifier's slot 4.
pub fn modifier_block_post_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, at(c, S1, modifier_block::PARTICLES));
        c.g[A1][0] = c.g[S0][0];
        call_on_modifier_list(c, 2, 0x28, 0x0035_2d5c, 0x0035_2d48)?;
        c.cycles += 3;
        c.g[S0][0] = lw(c, at(c, S1, modifier_block::RAILS));
        c.g[A1][0] = c.g[S0][0];
        call_on_modifier_list(c, 1, 0x18, 0x0035_2d8c, 0x0035_2d78)?;
        call_transform_tail(c, frame, 0x20, 0x0035_2db8)
    })
}

/// `ModifierBlock_Update(block)` (0x352c70): the transform modifier's
/// `Update` (slot 2), the UV-scroll and texture-flip modifiers, then the
/// boost modifier's `Update` (slot 2); when its slot 4 (still active)
/// returns 0 the boost is freed (`ModifierBlock_FreeBoost`).
pub fn modifier_block_update(c: &mut Cpu) {
    use modifier_block::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, at(c, S0, TRANSFORM));
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            let vt = lw(c, r32(c, A1));
            c.g[V0][0] = vt;
            let transform = c.g[A1][0];
            virtual_call(c, transform, vt, 0x10, V1, 0x0035_2ca0)?;
        }
        c.g[A0][0] = lw(c, at(c, S0, UV_SCROLL));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, table_callee::UV_SCROLL_MODIFIER_UPDATE, 0x0035_2cb4)?;
        }
        c.g[A0][0] = lw(c, at(c, S0, TEX_FLIP));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, table_callee::TEX_FLIP_MODIFIER_UPDATE, 0x0035_2cc8)?;
        }
        c.g[A1][0] = lw(c, at(c, S0, BOOST));
        c.cycles += 2;
        if c.g[A1][0] != 0 {
            c.cycles += 13;
            let boost = c.g[A1][0];
            let vt = lw(c, (boost as u32).wrapping_add(4));
            c.g[V0][0] = vt;
            virtual_call(c, boost, vt, 0x10, V1, 0x0035_2ce8)?;
            let boost = lw(c, at(c, S0, BOOST));
            c.g[A1][0] = boost;
            let vt = lw(c, (boost as u32).wrapping_add(4));
            c.g[V1][0] = vt;
            virtual_call(c, boost, vt, 0x20, V0, 0x0035_2d00)?;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::MODIFIER_BLOCK_FREE_BOOST.call(c, 0x0035_2d10)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModifierBlock_GetTransformBounds(block)` (0x352b88): the transform
/// modifier's bounds (slot 12), or null without one.
pub fn modifier_block_get_transform_bounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, r32(c, A0));
        if c.g[A1][0] != 0 {
            let vt = lw(c, r32(c, A1));
            c.g[V1][0] = vt;
            c.cycles += 4;
            let transform = c.g[A1][0];
            virtual_call(c, transform, vt, 0x60, V0, 0x0035_2bb4)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModObj` fields: +0xc vptr, +0x12 dirty flags (`u16`, bit 0 =
/// transform changed), +0x1c `ModifierBlock*`, +0x20 countdown.
mod mod_obj {
    pub const VPTR: u32 = 0x0c;
    pub const DIRTY: u32 = 0x12;
    pub const MODIFIERS: u32 = 0x1c;
    pub const COUNTDOWN: u32 = 0x20;
}

/// `ModObj_UpdateModifiers(obj)` (0x3556f8): updates the modifier block
/// and marks the object dirty (+0x12 bit 0) when the transform changed.
pub fn mod_obj_update_modifiers(c: &mut Cpu) {
    use mod_obj::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = lw(c, at(c, S0, MODIFIERS));
        if c.g[A0][0] != 0 {
            c.cycles += 6;
            callee::MODIFIER_BLOCK_UPDATE.call(c, 0x0035_571c)?;
            c.g[A0][0] = lw(c, at(c, S0, MODIFIERS));
            callee::MODIFIER_BLOCK_HAS_TRANSFORM_CHANGED.call(c, 0x0035_5724)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let dirty = c.lhu(at(c, S0, DIRTY)) | 1;
                c.g[V0][0] = dirty;
                c.sh(at(c, S0, DIRTY), dirty);
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModObj_PostUpdateModifiers(obj)` (0x355748): `ModifierBlock_PostUpdate`
/// on the object's modifier block, if any.
pub fn mod_obj_post_update_modifiers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, at(c, A0, mod_obj::MODIFIERS));
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            callee::MODIFIER_BLOCK_POST_UPDATE.call(c, 0x0035_5764)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `ModObj_Update(obj)` (0x356198): counts the countdown (+0x20) down;
/// unless slot 20 says active, slot 23 is positive, or 0x34fce0 says so,
/// it only calls slot 35 (idle). Otherwise it updates the modifiers, and
/// when slot 15 returns non-zero post-updates them; a finished transform
/// is acknowledged and reported through slot 34 with 1.
pub fn mod_obj_update(c: &mut Cpu) {
    use mod_obj::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let countdown = lw(c, at(c, S0, COUNTDOWN));
        c.g[V0][0] = sx((countdown as u32).wrapping_sub(1));
        if (countdown as i64) > 0 {
            c.cycles += 1;
            c.sw(at(c, S0, COUNTDOWN), c.g[V0][0]);
        }
        let obj = c.g[S0][0];
        c.cycles += 7;
        let vt = lw(c, at(c, S0, VPTR));
        c.g[V1][0] = vt;
        virtual_call(c, obj, vt, 0xa0, V0, 0x0035_61cc)?;
        let mut update = true;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            let vt = lw(c, at(c, S0, VPTR));
            c.g[V1][0] = vt;
            virtual_call(c, obj, vt, 0xb8, V0, 0x0035_61e8)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 4;
                c.g[A0][0] = obj;
                callee::MOD_OBJ_34FCE0.call(c, 0x0035_61f8)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 7;
                    let vt = lw(c, at(c, S0, VPTR));
                    c.g[V0][0] = vt;
                    virtual_call(c, obj, vt, 0x118, V1, 0x0035_6214)?;
                    frame.restore(c, S0, 0x10);
                    update = false;
                }
            }
        }
        if update {
            c.cycles += 9;
            c.g[A0][0] = obj;
            callee::MOD_OBJ_UPDATE_MODIFIERS.call(c, 0x0035_6224)?;
            let vt = lw(c, at(c, S0, VPTR));
            c.g[V1][0] = vt;
            virtual_call(c, obj, vt, 0x78, V0, 0x0035_6238)?;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = obj;
                callee::MOD_OBJ_POST_UPDATE_MODIFIERS.call(c, 0x0035_6248)?;
                c.g[A0][0] = lw(c, at(c, S0, MODIFIERS));
                if c.g[A0][0] != 0 {
                    c.cycles += 4;
                    callee::MODIFIER_BLOCK_IS_TRANSFORM_FINISHED.call(c, 0x0035_625c)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 9;
                        c.g[A0][0] = lw(c, at(c, S0, MODIFIERS));
                        callee::MODIFIER_BLOCK_CLEAR_TRANSFORM_FINISHED.call(c, 0x0035_626c)?;
                        let vt = lw(c, at(c, S0, VPTR));
                        c.g[V0][0] = vt;
                        c.g[A1][0] = 1;
                        virtual_call(c, obj, vt, 0x110, V1, 0x0035_6284)?;
                    }
                }
            }
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Render passes.

/// `GameModule_RenderModels(this, visible)` (0x22c078): sets up the
/// graphics state (per-view fog/alpha words in the draw context), then
/// draws each model of the visible lists (+0x20a0, count +0x209c; +0x40a4,
/// count +0x40a0) with `PS2GraphicsMan` slot 96 (`DrawModelSimple(model,
/// 0, 0, flags)`). Flags are 0xc00 for one view, 0x400 for split screen;
/// the second list adds 0x20.
pub fn game_module_render_models(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x50);
        let gfx = lw(c, gp_minus(c, GRAPHICS_GP));
        c.g[A2][0] = gfx;
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0xffff_ffff_ffff_f07f;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0]; // visible lists
        frame.save(c, S0, 0x40);
        c.g[S3][0] = 0xc00; // draw flags
        frame.save(c, S1, 0x30);
        c.g[T0][0] = 0x400;
        frame.save_ra(c, 0);
        c.g[T1][0] = sx(at(c, S2, 0x20a0));
        let view = lw(c, (gfx as u32).wrapping_add(0x24));
        let ctx = lw(c, (gfx as u32).wrapping_add(0xe84));
        c.g[A1][0] = ctx;
        c.g[V1][0] = sx((view as u32) << 2);
        let entry = addu(gfx, c.g[V1][0]);
        c.g[A2][0] = entry;
        let value = lw(c, (entry as u32).wrapping_add(0xf50));
        c.g[V0][0] = value;
        c.sh((ctx as u32).wrapping_add(0x12), value);
        let gfx = lw(c, gp_minus(c, GRAPHICS_GP));
        c.g[A2][0] = gfx;
        let ctx = lw(c, (gfx as u32).wrapping_add(0xe84));
        c.g[A1][0] = ctx;
        let word = (lw(c, (ctx as u32).wrapping_add(4)) & c.g[A3][0]) | 0x880;
        c.g[V0][0] = word;
        c.sw((ctx as u32).wrapping_add(4), word);
        let ctx = lw(c, (gfx as u32).wrapping_add(0xe84));
        c.g[V1][0] = ctx;
        let word = lw(c, ctx as u32) | 0x30;
        c.g[V0][0] = word;
        c.sw(ctx as u32, word);
        let views = lw(c, at(c, A0, 0x84));
        c.g[V1][0] = views;
        let count = lw(c, at(c, S2, 0x209c));
        c.g[A1][0] = count;
        let one_view = lw(c, (views as u32).wrapping_add(0x10)) < 2;
        c.g[V0][0] = one_view as u64;
        if !one_view {
            c.g[S3][0] = c.g[T0][0];
        }
        if (count as i64) > 0 {
            c.cycles += 4;
            draw_models(c, 0, 0x0022_c13c, 0x0022_c110)?;
        }
        c.cycles += 3;
        let count = lw(c, at(c, S2, 0x40a0));
        c.g[V0][0] = count;
        c.g[T1][0] = sx(at(c, S2, 0x40a4));
        if (count as i64) > 0 {
            c.cycles += 4;
            draw_models(c, 0x20, 0x0022_c18c, 0x0022_c160)?;
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

/// One `RenderModels` list: the list in t1 and the count in a1/v0 (moved
/// to s1/s0), `DrawModelSimple(model, 0, 0, s3 | extra)` per entry.
fn draw_models(c: &mut Cpu, extra_flags: u64, ret_addr: u32, head: u32) -> Flow {
    c.g[S1][0] = c.g[T1][0];
    c.g[S0][0] = if extra_flags == 0 { c.g[A1][0] } else { c.g[V0][0] };
    c.g[T1][0] = lw(c, gp_minus(c, GRAPHICS_GP));
    loop {
        c.cycles += 13;
        c.g[A2][0] = 0;
        c.g[A1][0] = lw(c, r32(c, S1));
        c.g[A3][0] = 0;
        let gfx = c.g[T1][0];
        let vt = lw(c, (gfx as u32).wrapping_add(0x10d8));
        c.g[V1][0] = vt;
        c.g[T0][0] = c.g[S3][0] | extra_flags;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
        virtual_call(c, gfx, vt, 0x300, V0, ret_addr)?;
        let more = c.g[S0][0] != 0;
        c.g[T1][0] = lw(c, gp_minus(c, GRAPHICS_GP));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event(c, head);
        }
    }
}

/// `GameModule_RenderBezierPatches(this, visible)` (0x22c620): opens a
/// Bézier pass context on the stack, draws the visible patch lists
/// (+0x50ac, count +0x50a8, LOD set 0; +0x70b0, count +0x70ac, LOD set 1),
/// flushes the `PS2BezierMan` (slot 5) when the views' +0x14 is 0, and
/// closes the pass.
pub fn game_module_render_bezier_patches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S3, 0x40);
        frame.save(c, S2, 0x50);
        c.g[S3][0] = c.g[A0][0]; // this
        c.g[S2][0] = c.g[A1][0]; // visible lists
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0x30);
        c.g[A0][0] = c.g[SP][0]; // the pass context at sp+0
        callee::GAME_MODULE_BEGIN_BEZIER_PASS.call(c, 0x0022_c648)?;
        for (count_off, list_off, enter, lod, ret_addr, head) in [
            (0x50a8, 0x50ac, 3, 0, 0x0022_c670, 0x0022_c660),
            (0x70ac, 0x70b0, 4, 1, 0x0022_c6a8, 0x0022_c698),
        ] {
            if lod == 1 {
                c.cycles += 3;
            }
            let count = lw(c, at(c, S2, count_off));
            c.g[V0][0] = count;
            c.g[A1][0] = sx(at(c, S2, list_off));
            if (count as i64) <= 0 {
                continue;
            }
            c.cycles += enter;
            c.g[S1][0] = c.g[A1][0];
            c.g[S0][0] = count;
            c.g[A1][0] = lw(c, r32(c, S1));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = lod;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                callee::GAME_MODULE_DRAW_BEZIER_PATCH.call(c, ret_addr)?;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = lw(c, r32(c, S1));
                if event_due(c) {
                    deliver_event(c, head);
                }
            }
        }
        c.cycles += 4;
        let views = lw(c, at(c, S3, 0x84));
        c.g[V0][0] = views;
        c.g[V1][0] = lw(c, (views as u32).wrapping_add(0x14));
        if c.g[V1][0] == 0 {
            c.cycles += 7;
            let man = lw(c, r32(c, GP).wrapping_add(BEZIER_MAN_GP));
            c.g[A1][0] = man;
            let vt = lw(c, (man as u32).wrapping_add(4));
            c.g[V0][0] = vt;
            virtual_call(c, man, vt, 0x28, V1, 0x0022_c6dc)?;
        }
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 9;
        c.g[A1][0] = 2;
        callee::GAME_MODULE_END_BEZIER_PASS.call(c, 0x0022_c6e8)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `GameModule_EndBezierPass(ctx, flags)` (0x22c3b8): the pass context's
/// destructor: `PS2BezierMan` slot 18 (end pass) on `ctx->man`, then
/// `delete ctx` when `flags & 1`.
pub fn game_module_end_bezier_pass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // flags
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // ctx
        let man = lw(c, r32(c, S1));
        c.g[A1][0] = man;
        let vt = lw(c, (man as u32).wrapping_add(4));
        c.g[V0][0] = vt;
        virtual_call(c, man, vt, 0x90, V1, 0x0022_c3e8)?;
        let delete = c.g[S0][0] & 1;
        c.g[S0][0] = delete;
        frame.restore(c, S0, 0x20);
        if delete != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table_callee::BUILTIN_DELETE, 0x0022_c3fc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{covered, fake_call, machine, FAKE, LOG_COUNT, LOG_START};
    use crate::difftest::{check, check_lohi, fill, fresh, require_paths, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::{reg::*, sx};
    use ssx3_ee_rt::funcs::GuestFn;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    // ------------------------------------------------------------- fakes

    /// `fakes::fake_call`, then a varied result: null, negative or the
    /// fake's pointer (fakes alone never return 0, and these functions
    /// branch on zero / sign).
    fn fake_varied(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match (h >> 20) & 3 {
            0 => c.g[V0][0] = 0,
            1 => c.g[V0][0] = sx(h as u32 | 0x8000_0000),
            _ => {}
        }
    }

    /// One vtable whose every slot 0x00..0x140 is a distinct fake with a
    /// small random `this` delta.
    const VT: u32 = DATA + 0xf000;
    fn put_vtable(c: &mut Cpu, r: &mut Rng) -> u32 {
        for off in (0..0x340).step_by(8) {
            let delta = ((r.next_u32() as i16 as i32 % 64) & !3) as u16;
            c.bus.write16(VT + off, delta);
            c.bus.write32(VT + off + 4, FAKE + 2 * off);
        }
        VT
    }

    /// The return addresses the fakes were called with, over all seeds
    /// (read from the fakes' RAM log): proves each call site was reached.
    fn require_call_sites(gen: GuestFn, seeds: u32, sites: &[u32], setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            gen(&mut c);
            let n = c.bus.read32(LOG_COUNT);
            for i in 0..n {
                let p = LOG_START + 0x220 * i;
                seen.insert(c.bus.read32(p + 0x10 + 16 * 31));
            }
        }
        for s in sites {
            assert!(seen.contains(s), "call site returning to {s:#x} never reached (seen {seen:x?})");
        }
    }

    /// Calls compared with unwinding fakes (`covered`), the call sites
    /// reached, and (for loops) events delivered mid-loop.
    fn calls_and_events(
        gen: GuestFn,
        hand: GuestFn,
        entry: Option<u32>,
        salt: u64,
        calls: u32,
        sites: &[u32],
        setup: &dyn Fn(&mut Cpu, &mut Rng, u32),
    ) {
        let with_calls = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, calls);
            c.hooks.miss = fake_varied;
            setup(c, &mut r, seed);
        };
        covered(gen, hand, SEEDS, &with_calls);
        let no_unwind = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 0);
            c.hooks.miss = fake_varied;
            setup(c, &mut r, seed);
        };
        require_call_sites(gen, SEEDS, sites, &no_unwind);
        if let Some(entry) = entry {
            check_events(gen, hand, SEEDS, &|c, seed| -> Events {
                let mut r = machine(c, seed, salt ^ 0xe7e7, 0);
                c.hooks.miss = fake_varied;
                setup(c, &mut r, seed);
                schedule(c, &mut r, seed, entry)
            });
        }
    }

    // ------------------------------------------------------- process lists

    const LIST: u32 = DATA;
    const BUCKETS: u32 = DATA + 0x40;
    const PROCS: u32 = DATA + 0x1000;

    /// Link `items` between the sentinels `head` and `tail`.
    fn link(c: &mut Cpu, head: u32, tail: u32, items: &[u32]) {
        let mut prev = head;
        for &p in items {
            c.bus.write32(prev + 4, p);
            c.bus.write32(p + 8, prev);
            prev = p;
        }
        c.bus.write32(prev + 4, tail);
        c.bus.write32(tail + 8, prev);
        c.bus.write32(head + 8, 0);
        c.bus.write32(tail + 4, 0);
    }

    /// A 9-priority list at `list` (buckets at BUCKETS) with `active` and
    /// `pending` processes in priority `pri`; the others empty. Returns the
    /// active processes then the pending ones.
    fn process_lists(c: &mut Cpu, r: &mut Rng, list: u32, pri: u32, active: u32, pending: u32) -> Vec<u32> {
        put_vtable(c, r);
        c.bus.write32(list, 9);
        c.bus.write32(list + 4, BUCKETS);
        for p in 0..9 {
            let b = BUCKETS + p * 0x44;
            c.bus.write32(b, r.next_u32() & 1);
            for s in [4, 0x14, 0x24, 0x34] {
                c.bus.write32(b + s, r.next_u32());
                c.bus.write32(b + s + 0xc, 0x0049_1f00);
            }
            link(c, b + 4, b + 0x14, &[]);
            link(c, b + 0x24, b + 0x34, &[]);
        }
        let procs: Vec<u32> = (0..active + pending).map(|i| PROCS + 0x20 * i).collect();
        for &p in &procs {
            fill(c, r, p, 8);
            c.bus.write32(p + 0xc, VT);
        }
        let b = BUCKETS + pri * 0x44;
        let (a, q) = procs.split_at(active as usize);
        link(c, b + 4, b + 0x14, a);
        link(c, b + 0x24, b + 0x34, q);
        procs
    }

    /// A priority (low bits) in a register with random upper bits.
    fn pri_arg(r: &mut Rng, pri: u32) -> u64 {
        (r.next_u32() as u64) << 32 | pri as u64
    }

    #[test]
    fn process_list_next() {
        check_lohi(ssx3_ee::s11::f_00354ef8__generated, super::process_list_next, SEEDS, &|c, seed| {
            let mut r = Rng(0x6e65_7874 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let pri = r.next_u32() % 9;
            let n = 1 + r.next_u32() % 4;
            let procs = process_lists(c, &mut r, LIST, pri, n, 0);
            c.g[A0][0] = LIST as u64;
            c.g[A1][0] = procs[(r.next_u32() % n) as usize] as u64;
            c.g[A2][0] = pri_arg(&mut r, pri);
        });
    }

    #[test]
    fn process_list_pending_first() {
        check_lohi(ssx3_ee::s11::f_00354f48__generated, super::process_list_pending_first, SEEDS, &|c, seed| {
            let mut r = Rng(0x7066_6972 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let pri = r.next_u32() % 9;
            let (a, p) = (r.next_u32() % 2, r.next_u32() % 3);
            process_lists(c, &mut r, LIST, pri, a, p);
            c.g[A0][0] = LIST as u64;
            c.g[A1][0] = pri_arg(&mut r, pri);
        });
    }

    /// Setup for the per-priority passes: 0..12 active processes.
    fn pass_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let pri = r.next_u32() % 9;
        let n = [0, 1, 2, 5, 12][(seed % 5) as usize];
        let pending = r.next_u32() % 2;
        process_lists(c, r, LIST, pri, n, pending);
        c.g[A0][0] = LIST as u64;
        c.g[A1][0] = pri_arg(r, pri);
    }

    #[test]
    fn process_list_post_update_priority() {
        calls_and_events(
            ssx3_ee::s11::f_00355028__generated,
            super::process_list_post_update_priority,
            Some(0x0035_5028),
            0x5051,
            6,
            &[0x0035_5068],
            &pass_setup,
        );
    }

    #[test]
    fn process_list_render_priority() {
        calls_and_events(
            ssx3_ee::s11::f_003550a0__generated,
            super::process_list_render_priority,
            Some(0x0035_50a0),
            0x5052,
            6,
            &[0x0035_50e0],
            &pass_setup,
        );
    }

    #[test]
    fn process_list_update_priority() {
        calls_and_events(
            ssx3_ee::s11::f_00354f98__generated,
            super::process_list_update_priority,
            Some(0x0035_4f98),
            0x5053,
            6,
            &[0x0035_4ff4],
            &pass_setup,
        );
    }

    #[test]
    fn process_list_merge_pending() {
        calls_and_events(
            ssx3_ee::s11::f_00354c98__generated,
            super::process_list_merge_pending,
            Some(0x0035_4c98),
            0x5054,
            12,
            &[0x0035_4d34, 0x0035_4d50, 0x0035_4d98],
            &|c, r, seed| {
                let pri = r.next_u32() % 9;
                let active = [0, 1, 3, 6][(seed % 4) as usize];
                let pending = [0, 1, 2, 5][(seed / 4 % 4) as usize];
                process_lists(c, r, LIST, pri, active, pending);
                c.g[A0][0] = LIST as u64;
                c.g[A1][0] = pri_arg(r, pri);
            },
        );
    }

    #[test]
    fn game_module_render_process_lists() {
        calls_and_events(
            ssx3_ee::s06::f_00230640__generated,
            super::game_module_render_process_lists,
            Some(0x0023_0640),
            0x5055,
            8,
            &[0x0035_50e0],
            &|c, r, seed| {
                let list = GAME_GP + super::PROCESS_LIST_GP;
                let pri = r.next_u32() % 9;
                process_lists(c, r, list, pri, [0, 1, 3][(seed % 3) as usize], 0);
                for i in 0..6 {
                    let p = if r.next_u32() % 2 == 0 { pri } else { r.next_u32() % 9 };
                    c.bus.write32(super::RENDER_PRIORITIES + 4 * i, p);
                }
            },
        );
    }

    // ---------------------------------------------------- world queries

    #[test]
    fn world_get_nash_and_seconds_per_tick() {
        for (gen, hand) in [
            (ssx3_ee::s09::f_002d1be0__generated as GuestFn, super::world_get_nash as GuestFn),
            (ssx3_ee::s09::f_002d1c70__generated, super::world_get_seconds_per_tick),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = Rng(0x6e61_7368 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let app = DATA + 0x100 + 0x10 * (r.next_u32() % 64);
                let module = DATA + 0x2000 + 0x10 * (r.next_u32() % 64);
                fill(c, &mut r, DATA, 0x1000);
                c.bus.write32(GAME_GP - super::APP_GP, app);
                c.bus.write32(app + 0x84, module);
            });
        }
    }

    #[test]
    fn section_man_find_logical_section() {
        let setup = |c: &mut Cpu, r: &mut Rng| {
            scramble(c, r);
            for i in 0..50 {
                // Small values so that some repeat.
                c.bus.write32(super::LOGICAL_SECTIONS + 4 + 0x10 * i, r.next_u32() % 80);
            }
            let v = r.next_u32() % 100;
            // Upper bits: sign extension (can match) or garbage (cannot).
            c.g[A1][0] = if r.next_u32() % 4 == 0 { (r.next_u32() as u64) << 32 | v as u64 } else { v as u64 };
        };
        let gen = ssx3_ee::s05::f_0022e0e0__generated;
        let hand = super::section_man_find_logical_section;
        check_events(gen, hand, SEEDS, &|c, seed| {
            let mut r = Rng(0x7365_6374 ^ (seed as u64) << 20);
            setup(c, &mut r);
            schedule(c, &mut r, seed, 0x0022_e0e0)
        });
        require_paths(gen, SEEDS, &[9, 15, 306], &|c, seed| {
            let mut r = Rng(0x7365_6374 ^ (seed as u64) << 20);
            setup(c, &mut r);
        });
    }

    /// A quadtree at DATA: inverse cell size, origin, root index and a
    /// node array where children always have larger indices (so walks
    /// end) and the last nodes are leaves.
    fn quad_tree(c: &mut Cpu, r: &mut Rng) {
        let tree = DATA;
        let nodes = DATA + 0x100;
        let inv_cell = [1.0f32, 0.25, 2.0, 1.0 / 64.0][(r.next_u32() % 4) as usize];
        let ox = (r.next_u32() % 2000) as f32 - 1000.0;
        let oz = (r.next_u32() % 2000) as f32 - 1000.0;
        c.bus.write32(tree, inv_cell.to_bits());
        c.bus.write32(tree + 4, ox.to_bits());
        c.bus.write32(tree + 8, oz.to_bits());
        let count = 64u32;
        c.bus.write16(tree + 0x14, (r.next_u32() % 4) as u16);
        c.bus.write32(tree + 0x20, nodes);
        for k in 0..count {
            for e in 0..4 {
                let child = if k + 1 < count { k + 1 + r.next_u32() % (count - k - 1).min(3) } else { k };
                let branch = k + 4 < count && r.next_u32() % 10 < 9;
                c.bus.write16(nodes + 8 * k + 2 * e, (child << 1 | branch as u32) as u16);
            }
        }
        // Points: cells mostly in range, sometimes outside on either axis.
        let cell = |r: &mut Rng| -> f32 {
            match r.next_u32() % 8 {
                0 => -((r.next_u32() % 100) as f32) - 1.5,
                1 => 0x8000 as f32 + (r.next_u32() % 100) as f32,
                _ => (r.next_u32() % 0x8000) as f32 + 0.25,
            }
        };
        let (cx, cz) = (cell(r), cell(r));
        c.f[12] = (ox + cx / inv_cell).to_bits();
        c.f[13] = (oz + cz / inv_cell).to_bits();
        c.g[A0][0] = tree as u64;
    }

    #[test]
    fn quad_tree_find_leaf() {
        let gen = ssx3_ee::s08::f_002c1cd8__generated;
        let hand = super::quad_tree_find_leaf;
        check_events(gen, hand, 600, &|c, seed| {
            let mut r = Rng(0x7175_6164 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            quad_tree(c, &mut r);
            schedule(c, &mut r, seed, 0x002c_1cd8)
        });
        // x outside, z outside, root leaf, one and two levels.
        require_paths(gen, 600, &[17, 19, 32, 50, 67], &|c, seed| {
            let mut r = Rng(0x7175_6164 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            quad_tree(c, &mut r);
        });
    }

    #[test]
    fn noise_fractal_1d() {
        let gen = ssx3_ee::s09::f_002d1928__generated;
        let hand = super::noise_fractal_1d;
        let setup = |c: &mut Cpu, r: &mut Rng| {
            scramble(c, r);
            c.g[GP][0] = GAME_GP as u64;
            let octaves = [0u64, 1, 2, 4, 7, sx(0xffff_fffe), 3 | 0xffff_0000_0000_0000][(r.next_u32() % 7) as usize];
            c.g[A0][0] = octaves;
            c.f[12] = r.f32(100.0);
            // Noise_Sample1D's sample table (0x445ab0; negative x reads
            // before it).
            for i in 0..0x180u32 {
                c.bus.write32(0x0044_5ab0 - 0x300 + 4 * i, r.f32(1.0));
            }
        };
        check_events(gen, hand, SEEDS, &|c, seed| {
            let mut r = Rng(0x6e6f_6973 ^ (seed as u64) << 20);
            setup(c, &mut r);
            schedule(c, &mut r, seed, 0x002d_1928)
        });
        require_paths(gen, SEEDS, &[25], &|c, seed| {
            let mut r = Rng(0x6e6f_6973 ^ (seed as u64) << 20);
            setup(c, &mut r);
        });
    }

    /// The VU0 microprogram at 0xef0 for the tests: `vi1 = vi14` (the
    /// node's clip word), so a clip word of 2 culls.
    fn put_cull_microprogram(c: &mut Cpu) {
        // Hand-written microcode has no translation: run it on the
        // reference interpreter.
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0.fallback =
            Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000 + 0xef0;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E bit
        c.bus.write32(at + 8, 0x8000_033c); // NOP
        c.bus.write32(at + 12, 0x0000_02ff); // NOP
    }

    /// Nash collection input: `nodes` visited nodes with up to `per_node`
    /// instances each. Returns `this`.
    fn nash_setup(c: &mut Cpu, r: &mut Rng, nodes: u32, per_node: u32) -> u32 {
        put_cull_microprogram(c);
        let this = DATA + 0x400;
        let tables = DATA + 0x40000;
        c.bus.write32(this + 0x90, DATA + 0x100);
        c.bus.write32(DATA + 0x100, tables);
        for s in 0..16 {
            c.bus.write32(tables + 8 * s + 0x24, if r.next_u32() % 2 == 0 { 6 } else { r.next_u32() % 8 });
        }
        for l in 0..32u32 {
            let e = tables.wrapping_add((l as i32 - 8) as u32 * 0x18);
            c.bus.write32(e + 0x3f0, if r.next_u32() % 2 == 0 { 3 } else { r.next_u32() % 8 });
        }
        c.bus.write32(this + 0x209c, r.next_u32() % 10);
        let pairs = DATA + 0x200;
        let mut inst = DATA + 0x10000;
        for i in 0..nodes {
            let node = DATA + 0x8000 + 0x40 * i;
            c.bus.write32(pairs + 8 * i, node);
            c.bus.write32(pairs + 8 * i + 4, [0, 1, 2, 2, r.next_u32()][(r.next_u32() % 5) as usize]);
            let n = r.next_u32() % (per_node + 1);
            let mut head = 0;
            for _ in 0..n {
                fill(c, r, inst, 0x40);
                c.bus.write32(inst, head);
                if r.next_u32() % 2 == 0 {
                    c.bus.write32(inst + 8, r.next_u32() | 3);
                }
                c.bus.write8(inst + 0x7d, if r.next_u32() % 3 == 0 { 0xff } else { (r.next_u32() % 16) as u8 });
                c.bus.write16(inst + 0x7e, ((r.next_u32() % 32) as i32 - 8) as u16);
                head = inst;
                inst += 0x100;
            }
            c.bus.write32(node + 0x20, head);
        }
        c.g[A0][0] = this as u64;
        c.g[A1][0] = pairs as u64;
        c.g[A2][0] = if nodes == 0 && r.next_u32() % 2 == 0 { sx(0xffff_fff0) } else { nodes as u64 };
        this
    }

    #[test]
    fn nash_collect_instances() {
        let gen = ssx3_ee::s05::f_0022a5a0__generated;
        let hand = super::nash_collect_instances;
        check_events(gen, hand, 600, &|c, seed| {
            let mut r = Rng(0x6e61_7368 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let nodes = [0, 1, 3, 6][(seed / 3 % 4) as usize];
            nash_setup(c, &mut r, nodes, 6);
            schedule(c, &mut r, seed, 0x0022_a5a0)
        });
        // Every instance path (filter / clip / cull / append) for one
        // instance: distinct (cycles, appended) outcomes.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..600 {
            let mut c = fresh();
            let mut r = Rng(0x6e31 ^ (seed as u64) << 20);
            scramble(&mut c, &mut r);
            let this = nash_setup(&mut c, &mut r, 1, 1);
            let before = c.bus.read32(this + 0x209c);
            gen(&mut c);
            seen.insert((c.cycles, c.bus.read32(this + 0x209c) - before));
        }
        assert!(seen.len() >= 10, "instance paths seen: {seen:?}");
    }

    // ---------------------------------------------------- modifier blocks

    const BLOCK: u32 = DATA + 0x200;
    const MODS: u32 = DATA + 0x4000;

    /// A modifier block: each modifier present with probability 3/4 (UV
    /// scroll / texture flip objects are zeroed memory), particle and rail
    /// lists of 0..6 nodes.
    fn modifier_block(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        put_vtable(c, r);
        fill(c, r, BLOCK, 8);
        let mut next = MODS;
        let mut obj = |c: &mut Cpu, r: &mut Rng, vptr_at: Option<u32>| -> u32 {
            let p = next;
            next += 0x80;
            for i in 0..0x20 {
                c.bus.write32(p + 4 * i, 0);
            }
            if let Some(off) = vptr_at {
                fill(c, r, p, 4);
                c.bus.write32(p + off, VT);
            }
            p
        };
        for (field, vptr_at) in [(0u32, Some(0u32)), (4, None), (8, None), (0xc, Some(4))] {
            let present = r.next_u32() % 4 != 0;
            let p = obj(c, r, vptr_at);
            c.bus.write32(BLOCK + field, if present { p } else { 0 });
        }
        for (field, n) in [(0x10u32, [0, 1, 3, 6][(seed % 4) as usize]), (0x1c, [0, 2, 1, 4][(seed / 4 % 4) as usize])] {
            let mut head = 0;
            for _ in 0..n {
                let p = obj(c, r, Some(8));
                c.bus.write32(p, head);
                head = p;
            }
            c.bus.write32(BLOCK + field, head);
        }
        BLOCK
    }

    #[test]
    fn modifier_block_render() {
        calls_and_events(
            ssx3_ee::s11::f_00352dd0__generated,
            super::modifier_block_render,
            Some(0x0035_2dd0),
            0x4d42,
            8,
            &[0x0035_2e0c, 0x0035_2e38],
            &|c, r, seed| {
                c.g[A0][0] = modifier_block(c, r, seed) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_post_update() {
        calls_and_events(
            ssx3_ee::s11::f_00352d20__generated,
            super::modifier_block_post_update,
            Some(0x0035_2d20),
            0x4d43,
            12,
            &[0x0035_2d5c, 0x0035_2d8c, 0x0035_2db8],
            &|c, r, seed| {
                c.g[A0][0] = modifier_block(c, r, seed) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_update() {
        calls_and_events(
            ssx3_ee::s11::f_00352c70__generated,
            super::modifier_block_update,
            None,
            0x4d44,
            4,
            // Transform, boost update / active, FreeBoost's dtor call.
            &[0x0035_2ca0, 0x0035_2ce8, 0x0035_2d00, 0x0035_31b8],
            &|c, r, seed| {
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = modifier_block(c, r, seed) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_get_transform_bounds() {
        calls_and_events(
            ssx3_ee::s11::f_00352b88__generated,
            super::modifier_block_get_transform_bounds,
            None,
            0x4d45,
            1,
            &[0x0035_2bb4],
            &|c, r, seed| {
                c.g[A0][0] = modifier_block(c, r, seed) as u64;
            },
        );
    }

    /// A ModObj with a vptr, a countdown, an instance pointer (+0x18,
    /// zeroed memory) and a modifier block (or none).
    fn mod_obj(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let block = modifier_block(c, r, seed);
        let obj = DATA + 0x300;
        fill(c, r, obj, 0x10);
        c.bus.write32(obj + 0xc, VT);
        c.bus.write32(obj + 0x18, DATA + 0x6000);
        c.bus.write32(obj + 0x1c, if r.next_u32() % 5 == 0 { 0 } else { block });
        c.bus.write32(obj + 0x20, (r.next_u32() % 5) as i32 as u32 - 2);
        obj
    }

    #[test]
    fn mod_obj_update_modifiers() {
        calls_and_events(
            ssx3_ee::s11::f_003556f8__generated,
            super::mod_obj_update_modifiers,
            None,
            0x4d4f,
            6,
            // ModifierBlock_Update's transform call, HasTransformChanged's.
            &[0x0035_2ca0, 0x0035_2e78],
            &|c, r, seed| {
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = mod_obj(c, r, seed) as u64;
            },
        );
    }

    #[test]
    fn mod_obj_post_update_modifiers() {
        calls_and_events(
            ssx3_ee::s11::f_00355748__generated,
            super::mod_obj_post_update_modifiers,
            None,
            0x4d50,
            8,
            &[0x0035_2d5c],
            &|c, r, seed| {
                c.g[A0][0] = mod_obj(c, r, seed) as u64;
            },
        );
    }

    #[test]
    fn mod_obj_update() {
        calls_and_events(
            ssx3_ee::s11::f_00356198__generated,
            super::mod_obj_update,
            None,
            0x4d51,
            10,
            &[0x0035_61cc, 0x0035_61e8, 0x0035_6214, 0x0035_6238, 0x0035_6284],
            &|c, r, seed| {
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = mod_obj(c, r, seed) as u64;
            },
        );
    }

    // ------------------------------------------------------ world, render

    #[test]
    fn world_update() {
        for (gen, hand, is_world) in [
            (ssx3_ee::s13::f_003a6928__generated as GuestFn, super::world_update as GuestFn, true),
            (ssx3_ee::s13::f_003a8290__generated, super::c_world_cache_update, false),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                machine(c, seed, 0x776f, 0);
                c.hooks.miss = fake_varied;
                let cache = DATA + 0x8000;
                let world = DATA + 0x100;
                c.bus.write32(world, cache);
                c.g[A0][0] = if is_world { world } else { cache } as u64;
            });
        }
    }

    #[test]
    fn game_module_render_models() {
        calls_and_events(
            ssx3_ee::s05::f_0022c078__generated,
            super::game_module_render_models,
            Some(0x0022_c078),
            0x524d,
            8,
            &[0x0022_c13c, 0x0022_c18c],
            &|c, r, seed| {
                put_vtable(c, r);
                c.g[GP][0] = GAME_GP as u64;
                let gfx = DATA + 0x10000;
                fill(c, r, gfx, 0x400);
                c.bus.write32(GAME_GP - super::GRAPHICS_GP, gfx);
                c.bus.write32(gfx + 0x10d8, VT);
                c.bus.write32(gfx + 0x24, r.next_u32() % 4);
                let ctx = DATA + 0x200;
                fill(c, r, ctx, 8);
                c.bus.write32(gfx + 0xe84, ctx);
                let this = DATA + 0x300;
                let views = DATA + 0x380;
                c.bus.write32(this + 0x84, views);
                c.bus.write32(views + 0x10, r.next_u32() % 4);
                let lists = DATA + 0x20000;
                let n1 = [0u32, 1, 3, 9][(seed % 4) as usize];
                let n2 = [0u32, 2, 1, 7][(seed / 4 % 4) as usize];
                c.bus.write32(lists + 0x209c, if n1 == 0 && r.next_u32() % 2 == 0 { -1i32 as u32 } else { n1 });
                c.bus.write32(lists + 0x40a0, n2);
                fill(c, r, lists + 0x20a0, 16);
                fill(c, r, lists + 0x40a4, 16);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = lists as u64;
            },
        );
    }

    #[test]
    fn game_module_end_bezier_pass() {
        calls_and_events(
            ssx3_ee::s05::f_0022c3b8__generated,
            super::game_module_end_bezier_pass,
            None,
            0x4542,
            2,
            &[0x0022_c3e8],
            &|c, r, _seed| {
                put_vtable(c, r);
                let man = DATA + 0x400;
                c.bus.write32(man + 4, VT);
                let ctx = DATA + 0x500;
                fill(c, r, ctx, 8);
                c.bus.write32(ctx, man);
                c.g[A0][0] = ctx as u64;
                c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn game_module_render_bezier_patches() {
        calls_and_events(
            ssx3_ee::s05::f_0022c620__generated,
            super::game_module_render_bezier_patches,
            Some(0x0022_c620),
            0x4250,
            8,
            &[0x0022_c6dc],
            &|c, r, seed| {
                put_vtable(c, r);
                c.g[GP][0] = GAME_GP as u64;
                let man = DATA + 0x400;
                c.bus.write32(man + 4, VT);
                c.bus.write32(GAME_GP + super::BEZIER_MAN_GP, man);
                let this = DATA + 0x300;
                let views = DATA + 0x380;
                c.bus.write32(this + 0x84, views);
                c.bus.write32(views + 0x14, r.next_u32() % 2);
                let lists = DATA + 0x20000;
                let n1 = [0u32, 1, 3][(seed % 3) as usize];
                let n2 = [0u32, 2, 1][(seed / 3 % 3) as usize];
                c.bus.write32(lists + 0x50a8, n1);
                c.bus.write32(lists + 0x70ac, n2);
                for i in 0..4 {
                    c.bus.write32(lists + 0x50ac + 4 * i, DATA + 0x30000 + 0x200 * i);
                    c.bus.write32(lists + 0x70b0 + 4 * i, DATA + 0x31000 + 0x200 * i);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = lists as u64;
            },
        );
    }
}
