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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

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
                    use process_list::ACTIVE_HEAD;
                    c.g[V0][0] = lw(c, at(c, S2, process_list::BUCKETS));
                    c.cycles += 15;
                    let p = c.g[S1][0];
                    let head = addu(c.g[S4][0], c.g[V0][0]) as u32 + ACTIVE_HEAD;
                    c.g[V0][0] = sx(head - ACTIVE_HEAD);
                    c.g[V1][0] = lw(c, head + NEXT);
                    c.sw(p as u32 + NEXT, c.g[V1][0]);
                    let head = bucket_of(c, S4) + ACTIVE_HEAD;
                    c.g[V0][0] = sx(head - ACTIVE_HEAD);
                    c.g[V1][0] = lw(c, head + NEXT);
                    c.sw((c.g[V1][0] as u32).wrapping_add(PREV), p);
                    let head = bucket_of(c, S4) + ACTIVE_HEAD;
                    c.g[V0][0] = sx(head);
                    c.sw(p as u32 + PREV, c.g[V0][0]);
                    let head = bucket_of(c, S4) + ACTIVE_HEAD;
                    c.g[V1][0] = sx(head - ACTIVE_HEAD);
                    c.sw(head + NEXT, p);
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
    let nodes = lw(c, tree.wrapping_add(0x20));
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
