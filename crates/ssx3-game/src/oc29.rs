//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the renderer's deferral tables (`DeferMan`), the
//! modifier-object render path (`ModObj`) and the spline node visibility
//! test, each with a differential test against the generated body.
//!
//! - **Deferred state records.** `DeferMan` interns draw state so equal
//!   states are shared: per-pass hash chains of buckets (`FindBucket`, chain
//!   heads at +0x678a0) and of 0x18-byte state records (`InternState`, heads
//!   at +0x674a0, pool at +0x51480), and a linear table of 0x180-byte matrix
//!   records (`InternMatrixRecord`, +0x69cd0) keyed by their last 0xa0 bytes.
//!   A hit is moved to the front of its chain.
//! - **Modifier objects.** `ModObj_Render` decides whether a modifier
//!   object draws this frame (layer and pass switches, the model's flags,
//!   the camera visibility test, occlusion) and hands it to the graphics
//!   manager's draw method; `ModObj_v50` refreshes the object's bounds in
//!   the world's spatial hash.
//!
//! The loops are structured loops in the generated code (the `event()`
//! result is ignored); their state lives in the registers the original uses
//! and is read back from them after each back-edge check.
//!
//! Not here: `BELibrary_GetInstance` (0x14dc80) is already written in
//! `crate::ai` (`be_library_get_instance`); its build path (the character
//! database from disc) cannot run on a test machine, so it stays generated.

use crate::abi::{self, addu, r32};
use crate::call::{call_ptr, guest, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_5000, defer_man_find_bucket, "DeferMan_FindBucket"),
    (0x0039_4ed0, defer_man_intern_state, "DeferMan_InternState"),
    (0x0036_45b8, defer_man_intern_matrix_record, "DeferMan_InternMatrixRecord"),
    (0x0034_5638, spline_modifier_is_node_visible, "SplineModifier_IsNodeVisible"),
    (0x0035_68b0, mod_obj_v50, "ModObj_v50"),
    (0x0035_6298, mod_obj_render, "ModObj_Render"),
];

// ---------------------------------------------------------------------------
// Callees and globals.

/// Direct calls (same shard, through the generated function and its
/// override slot).
mod callee {
    use super::{ee, Callee};
    pub const GET_TRANSFORM_BOUNDS: Callee =
        Callee { addr: 0x0035_2b88, f: ee::s11::ModifierBlock_GetTransformBounds };
    pub const SET_TRANSFORM_BOUNDS: Callee =
        Callee { addr: 0x0035_2bc0, f: ee::s11::ModifierBlock_SetTransformBounds };
    pub const GET_TRANSFORM_RADIUS: Callee =
        Callee { addr: 0x0035_2bf8, f: ee::s11::ModifierBlock_GetTransformRadius };
    pub const MODIFIER_BLOCK_RENDER: Callee =
        Callee { addr: 0x0035_2dd0, f: ee::s11::ModifierBlock_Render };
}

/// Calls the original makes through the function table (other shards).
const WORLD_GET_NASH: u32 = 0x002d_1be0;
const WORLD_IS_OCCLUDED: u32 = 0x002d_2058;
const SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;
const UV_SCROLL_GET_MATRIX: u32 = 0x0035_fc20;

/// `*(gp + GFX_MAN)`: the graphics manager; its camera vtable is at +0x10d8.
const GFX_MAN: u32 = 0x2a90;
/// `**(gp + LAYER_TABLE)`: the render layer switches (8 bytes per layer,
/// mode at +0x24; 0x18 bytes per pass from +0x3f0, mode at +0).
const LAYER_TABLE: u32 = 0x16c8;
/// The identity matrix `ModObj_Render` passes when an object has its own.
const IDENTITY_MATRIX: u32 = 0x004f_f1a0;
/// 1.0f.
const ONE: u32 = 0x3f80_0000;

/// `DeferMan` layout (offsets from the `DeferMan*`).
mod defer {
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

/// The matrix record table of `DeferMan_InternMatrixRecord`.
mod matrix {
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
    pub const PIECES: [(u32, u32); 9] =
        [(0x00, 4), (0x40, 4), (0x80, 4), (0xc0, 1), (0xd0, 1), (0xe0, 4), (0x120, 4), (0x160, 1), (0x170, 1)];
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
fn intern_state_find(c: &mut Cpu) -> bool {
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
fn intern_state_insert(c: &mut Cpu) {
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
fn floats_equal(c: &mut Cpu, stored: u32, new: u32, n: u32) -> bool {
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
fn matrix_search(c: &mut Cpu) {
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
fn matrix_tail(c: &mut Cpu) {
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
fn matrix_append(c: &mut Cpu, man: u32, base: u32, new: u32) {
    let count_at = base.wrapping_add(matrix::COUNT);
    let mut vf = [0u128; 4];
    let mut a0_q = 0u128;
    let mut a1_hi = 0u64;
    let mut rec_off = 0u32;
    for &(off, n) in matrix::PIECES.iter() {
        let count = Addr(count_at).lw(c) as u32;
        rec_off = abi::mult(c, count, matrix::SIZE);
        let dst = rec_off.wrapping_add(man).wrapping_add(matrix::RECORDS + off);
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
        let lanes = [*q as u32, (*q >> 32) as u32, (*q >> 64) as u32, (*q >> 96) as u32];
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

// ---------------------------------------------------------------------------
// Spline modifier.

/// `bool SplineModifier_IsNodeVisible(SplineNode *node)` (0x345638): build
/// the node's box `{min (+0x6c), 1; max (+0x78), 1}` on the stack, ask the
/// camera (graphics manager method 0x158) for its view data, then run the
/// visibility test (method 0x2e8 on the box and the view data); true unless
/// the test answered exactly 1.
pub fn spline_modifier_is_node_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0xb0);
        let gfx = Addr(r32(c, GP).wrapping_add(GFX_MAN)).lw(c);
        c.g[A2][0] = gfx;
        frame.save(c, S2, 0x80);
        let node = c.g[A0][0];
        c.g[V0][0] = node;
        frame.save(c, S3, 0x70);
        let (min, max, view) = (frame.at(c, 0), frame.at(c, 0x10), frame.at(c, 0x20));
        c.g[S2][0] = sx(view);
        frame.save_ra(c, 0x60);
        c.g[S3][0] = sx(max);
        frame.save(c, S0, 0xa0);
        c.g[A0][0] = sx(view);
        frame.save(c, S1, 0x90);
        let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
        c.g[S1][0] = cam_vt;
        let n = Addr(node as u32);
        let corner = [
            n.off(0x6c).read_u32(c),
            n.off(0x70).read_u32(c),
            n.off(0x74).read_u32(c),
            n.off(0x78).read_u32(c),
            n.off(0x7c).read_u32(c),
            n.off(0x80).read_u32(c),
        ];
        c.f[..5].copy_from_slice(&corner[..5]);
        c.f[6] = corner[5];
        c.g[AT][0] = ONE as u64;
        c.f[5] = ONE;
        let test_delta = Addr((cam_vt as u32).wrapping_add(0x2e8)).lh(c);
        c.g[S0][0] = addu(gfx, test_delta);
        for (i, &w) in corner[..3].iter().enumerate() {
            Addr(min).off(4 * i as u32).write_u32(c, w);
            Addr(max).off(4 * i as u32).write_u32(c, corner[3 + i]);
        }
        Addr(min).off(0xc).write_u32(c, ONE);
        Addr(max).off(0xc).write_u32(c, ONE);
        // The camera's view data, into the buffer at sp+0x20.
        let vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
        c.g[V0][0] = vt;
        let query = VSlot::read(c, vt, 0x158);
        c.g[A1][0] = query.this(gfx);
        c.g[V1][0] = query.func;
        call_ptr(c, query.func as u32, 0x0034_56c4)?;
        // test(this, box, max, view).
        let test = Addr((cam_vt as u32).wrapping_add(0x2ec)).lw(c);
        c.g[V0][0] = test;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, test as u32, 0x0034_56dc)?;
        let answer = c.g[V0][0] ^ 1;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        c.g[V0][0] = (answer != 0) as u64;
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Modifier objects.

/// `void ModObj_v50(ModObj *this)` (0x3568b0, vtable slot 0x50): when the
/// object has a modifier block with an implementation, rebuild its box
/// around its position (method 0xc0, +0x30) with the block's radius, store
/// it as the block's bounds and move the object (`this+0x18`) in the
/// world's spatial hash from the old bounds to the new; then notify the
/// implementation (its method 0xc0).
pub fn mod_obj_v50(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0x80);
        let field = |off: u32| Addr((this as u32).wrapping_add(off));
        let block = field(0x1c).lw(c);
        c.g[V0][0] = block;
        let mut imp = 0;
        if block != 0 {
            c.cycles += 1;
            imp = Addr(block as u32).lw(c);
        }
        c.g[S1][0] = imp;
        c.cycles += 2;
        if imp != 0 {
            c.cycles += 57;
            // The old bounds (two quadwords) to sp+0x20.
            c.g[A0][0] = field(0x1c).lw(c);
            callee::GET_TRANSFORM_BOUNDS.call(c, 0x0035_68e4)?;
            let bounds = r32(c, V0);
            c.lq(V1, bounds);
            c.g[A0][0] = field(0x1c).lw(c);
            let old_min = frame.at(c, 0x20);
            c.sq(V1, old_min);
            c.lq(A1, bounds.wrapping_add(0x10));
            let old_max = frame.at(c, 0x30);
            c.sq(A1, old_max);
            // The radius vector (r, r, r, 0) at sp+0x40.
            callee::GET_TRANSFORM_RADIUS.call(c, 0x0035_68fc)?;
            let radius = c.f[0];
            let rad = Addr(frame.at(c, 0x40));
            rad.off(0xc).sw(c, 0);
            rad.off(8).write_u32(c, radius);
            rad.write_u32(c, radius);
            rad.off(4).write_u32(c, radius);
            // New min = position - radius (sp, sp+0x50).
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            let position = VSlot::read(c, vt, 0xc0);
            c.g[A0][0] = position.this(this);
            c.g[V0][0] = position.func;
            call_ptr(c, position.func as u32, 0x0035_6920)?;
            let pos = r32(c, V0);
            offset_by_radius(c, frame, pos, false);
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            for off in [0, 0x50] {
                let at = frame.at(c, off);
                c.sq(V0, at);
            }
            // New max = position + radius (sp+0x10, sp+0x50).
            let position = VSlot::read(c, vt, 0xc0);
            c.g[A0][0] = position.this(this);
            c.g[V0][0] = position.func;
            call_ptr(c, position.func as u32, 0x0035_6958)?;
            let pos = r32(c, V0);
            offset_by_radius(c, frame, pos, true);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = field(0x1c).lw(c);
            for off in [0x10, 0x50] {
                let at = frame.at(c, off);
                c.sq(V0, at);
            }
            callee::SET_TRANSFORM_BOUNDS.call(c, 0x0035_6988)?;
            // SpatialHash_Update(World_GetNash(), 0, obj, new, old).
            let obj = field(0x18).lw(c);
            c.g[S0][0] = obj;
            call_ptr(c, WORLD_GET_NASH, 0x0035_6990)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = obj;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(old_min);
            c.g[A1][0] = 0;
            call_ptr(c, SPATIAL_HASH_UPDATE, 0x0035_69a8)?;
            let vt = Addr(imp as u32).lw(c);
            c.g[V0][0] = vt;
            let notify = VSlot::read(c, vt, 0xc0);
            c.g[A0][0] = notify.this(imp);
            c.g[V1][0] = notify.func;
            call_ptr(c, notify.func as u32, 0x0035_69bc)?;
        }
        frame.restore(c, S0, 0xa0);
        c.cycles += 4;
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `ModObj_v50`'s box corner: the position quadword at `pos + 0x30`
/// (through v1 and sp+0x60) plus or minus the radius vector at sp+0x40 in
/// VU0 (VF3 ± VF4 → VF5), stored to sp+0x70 and loaded into v0.
fn offset_by_radius(c: &mut Cpu, frame: Frame, pos: u32, add: bool) {
    c.lq(V1, pos.wrapping_add(0x30));
    let staged = frame.at(c, 0x60);
    c.sq(V1, staged);
    let p = Vec4::from_bits(c.bus.read128(staged));
    let rad_at = frame.at(c, 0x40);
    let r = Vec4::from_bits(c.bus.read128(rad_at));
    let corner = if add {
        Vec4 { x: p.x + r.x, y: p.y + r.y, z: p.z + r.z, w: p.w + r.w }
    } else {
        Vec4 { x: p.x - r.x, y: p.y - r.y, z: p.z - r.z, w: p.w - r.w }
    };
    vu0_scratch::set_vf(c, 3, 0b1111, p);
    vu0_scratch::set_vf(c, 4, 0b1111, r);
    vu0_scratch::set_vf(c, 5, 0b1111, corner);
    let out = frame.at(c, 0x70);
    c.bus.write128(out, corner.to_bits());
    c.lq(V0, out);
}

/// `block && *(block + off) != 0`, as `ModObj_Render` tests the modifier
/// block's optional parts (+2 cycles when there is a block).
fn block_has(c: &mut Cpu, block: u64, off: u32) -> bool {
    if block == 0 {
        return false;
    }
    c.cycles += 2;
    Addr((block as u32).wrapping_add(off)).lw(c) != 0
}

/// `void ModObj_Render(ModObj *this)` (0x356298). Draws the object unless:
/// its model's layer (+0x7d) is switched off (layer mode != 6, or the
/// pass (+0x7e) mode != 3), the model's flags (+8) lack bit 2, the object
/// is hidden (+0x28), the camera test (graphics manager method 0x2e8 on
/// the object's own bounds, method 0x168, or the model's box at +0x60)
/// answers 1, or (model box only) the world occludes it. First it renders
/// the modifier block (`ModifierBlock_Render`). The draw call is the
/// graphics manager's method 0x2f8 with the model, the object's matrix
/// (method 0xc0, or method 0xd8's with the identity matrix when 0xd0 says
/// it has one), the UV-scroll matrix, the block's +8 part (or the model's
/// +0x94), the draw flags (0x2000: the block implementation's method 0xd0
/// answered 7; 0x400: camera-tested; 0x20: the test answered neither 0 nor
/// 1; 2: UV scroll) and f12 = `this+0x24`.
pub fn mod_obj_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x110);
        let layers = Addr(r32(c, GP).wrapping_add(LAYER_TABLE)).lw(c);
        c.g[V0][0] = layers;
        frame.save(c, S4, 0xc0);
        frame.save(c, S0, 0x100);
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        frame.save(c, S1, 0xf0);
        c.g[A0][0] = 0xff;
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        let field = |off: u32| Addr((this as u32).wrapping_add(off));
        // Layer and pass switches.
        let model = field(0x18).lw(c);
        c.g[A1][0] = model;
        let table = Addr(layers as u32).lw(c);
        c.g[A2][0] = table;
        let layer = Addr(model as u32).off(0x7d).lbu(c);
        c.g[V1][0] = layer;
        let visible: u64 = if layer == 0xff {
            1
        } else {
            c.cycles += 7;
            let layer = Addr(model as u32).off(0x7d).lbu(c) as u32;
            let mode = Addr((layer << 3).wrapping_add(table as u32)).off(0x24).lw(c) ^ 6;
            c.g[V1][0] = mode;
            if mode != 0 {
                0
            } else {
                c.cycles += 7;
                let pass = Addr(model as u32).off(0x7e).lh(c);
                c.g[A0][0] = 0x18;
                let pass_off = abi::mult(c, pass as u32, 0x18);
                let mode = Addr(pass_off.wrapping_add(table as u32)).off(0x3f0).lw(c) ^ 3;
                c.g[V1][0] = mode;
                (mode == 0) as u64
            }
        };
        c.g[V0][0] = visible;
        c.cycles += 2;
        c.g[S6][0] = 0;
        // How many of s0/s1 the path already restored (the epilogue's entry).
        let restored: u8 = 'body: {
            if visible == 0 {
                break 'body 0;
            }
            c.cycles += 3;
            let block = field(0x1c).lw(c);
            c.g[A0][0] = block;
            let gfx = Addr(r32(c, GP).wrapping_add(GFX_MAN)).lw(c);
            c.g[S5][0] = gfx;
            if block == 0 {
                c.cycles += 1;
            } else {
                c.cycles += 6;
                callee::MODIFIER_BLOCK_RENDER.call(c, 0x0035_6334)?;
                let block = field(0x1c).lw(c);
                c.g[V0][0] = block;
                let imp = Addr(block as u32).lw(c);
                c.g[A1][0] = imp;
                if imp != 0 {
                    c.cycles += 8;
                    let vt = Addr(imp as u32).lw(c);
                    c.g[V1][0] = vt;
                    let kind = VSlot::read(c, vt, 0xd0);
                    c.g[A0][0] = kind.this(imp);
                    c.g[V0][0] = kind.func;
                    call_ptr(c, kind.func as u32, 0x0035_6358)?;
                    let kind = c.g[V0][0] ^ 7;
                    c.g[V0][0] = kind;
                    c.g[V1][0] = 0x2000;
                    if kind == 0 {
                        c.g[S6][0] = 0x2000;
                    }
                    c.cycles += 1;
                }
            }
            let model = field(0x18).lw(c);
            c.g[V1][0] = model;
            c.cycles += 4;
            let drawn = Addr(model as u32).off(8).lw(c) & 4;
            c.g[V0][0] = drawn;
            frame.restore(c, S0, 0x100);
            if drawn == 0 {
                break 'body 1;
            }
            c.cycles += 3;
            let hidden = field(0x28).lw(c);
            c.g[V0][0] = hidden;
            frame.restore(c, S1, 0xf0);
            if hidden != 0 {
                break 'body 2;
            }
            // The camera test, on the object's own bounds or the model's box.
            c.cycles += 8;
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            let own = VSlot::read(c, vt, 0x160);
            c.g[A0][0] = own.this(this);
            c.g[V0][0] = own.func;
            call_ptr(c, own.func as u32, 0x0035_6398)?;
            let own_bounds = c.g[V0][0];
            c.g[S7][0] = own_bounds;
            if own_bounds == 0 {
                let model = field(0x18).lw(c);
                c.g[V0][0] = model;
                c.cycles += 32;
                let (min, max, view) = (frame.at(c, 0), frame.at(c, 0x10), frame.at(c, 0x40));
                c.g[S2][0] = sx(view);
                let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
                c.g[S1][0] = cam_vt;
                c.g[S3][0] = sx(max);
                let m = Addr(model as u32);
                let corner = [
                    m.off(0x60).read_u32(c),
                    m.off(0x64).read_u32(c),
                    m.off(0x68).read_u32(c),
                    m.off(0x6c).read_u32(c),
                    m.off(0x70).read_u32(c),
                    m.off(0x74).read_u32(c),
                ];
                c.g[A0][0] = sx(view);
                let test_delta = Addr((cam_vt as u32).wrapping_add(0x2e8)).lh(c);
                c.g[S0][0] = addu(gfx, test_delta);
                c.g[AT][0] = ONE as u64;
                (c.f[0], c.f[1], c.f[2]) = (corner[5], corner[4], ONE);
                for (i, &w) in corner[..3].iter().enumerate() {
                    Addr(min).off(4 * i as u32).write_u32(c, w);
                    Addr(max).off(4 * i as u32).write_u32(c, corner[3 + i]);
                }
                Addr(min).off(0xc).write_u32(c, ONE);
                Addr(max).off(0xc).write_u32(c, ONE);
                let vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
                c.g[V0][0] = vt;
                let query = VSlot::read(c, vt, 0x158);
                c.g[A1][0] = query.this(gfx);
                c.g[V1][0] = query.func;
                call_ptr(c, query.func as u32, 0x0035_6464)?;
                c.g[V0][0] = Addr((cam_vt as u32).wrapping_add(0x2ec)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
            } else {
                c.cycles += 21;
                let vt = field(0xc).lw(c);
                c.g[V1][0] = vt;
                let get = VSlot::read(c, vt, 0x168);
                c.g[A0][0] = get.this(this);
                c.g[V0][0] = get.func;
                call_ptr(c, get.func as u32, 0x0035_63b8)?;
                let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
                c.g[S1][0] = cam_vt;
                let bounds = c.g[V0][0];
                c.g[S2][0] = bounds;
                c.g[A0][0] = c.g[SP][0];
                c.g[S3][0] = sx((bounds as u32).wrapping_add(0x10));
                let test_delta = Addr((cam_vt as u32).wrapping_add(0x2e8)).lh(c);
                let query = VSlot::read(c, cam_vt, 0x158);
                c.g[S0][0] = addu(gfx, test_delta);
                c.g[A1][0] = query.this(gfx);
                c.g[V0][0] = query.func;
                call_ptr(c, query.func as u32, 0x0035_63e0)?;
                c.g[V0][0] = Addr((cam_vt as u32).wrapping_add(0x2ec)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[SP][0];
            }
            c.cycles += 6;
            let test = r32(c, V0);
            call_ptr(c, test, 0x0035_6480)?;
            let answer = c.g[V0][0];
            c.g[V1][0] = answer;
            c.g[V0][0] = 1;
            c.g[S6][0] |= 0x400;
            if answer == 1 {
                break 'body 0;
            }
            c.cycles += 3;
            let partly = c.g[S6][0] | 0x20;
            c.g[V0][0] = partly;
            if answer != 0 {
                c.g[S6][0] = partly;
            }
            if own_bounds == 0 {
                c.cycles += 5;
                let model = field(0x18).lw(c);
                c.g[A0][0] = sx((model as u32).wrapping_add(0x50));
                call_ptr(c, WORLD_IS_OCCLUDED, 0x0035_64a8)?;
                frame.restore(c, S0, 0x100);
                if c.g[V0][0] != 0 {
                    break 'body 1;
                }
            }
            // The object's matrix (s2) and its own extra matrix (s1).
            c.cycles += 8;
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            c.g[S0][0] = 0;
            let has = VSlot::read(c, vt, 0xd0);
            c.g[A0][0] = has.this(this);
            c.g[V0][0] = has.func;
            call_ptr(c, has.func as u32, 0x0035_64c8)?;
            let has_own_matrix = c.g[V0][0] != 0;
            let vt = field(0xc).lw(c);
            c.g[V1][0] = vt;
            if has_own_matrix {
                c.cycles += 8;
                let get = VSlot::read(c, vt, 0xd8);
                c.g[A0][0] = get.this(this);
                c.g[V0][0] = get.func;
                call_ptr(c, get.func as u32, 0x0035_64e0)?;
                c.g[V1][0] = 0x50_0000;
                c.g[S1][0] = c.g[V0][0];
                c.g[S2][0] = sx(IDENTITY_MATRIX);
            } else {
                c.cycles += 6;
                c.g[S1][0] = 0;
                let get = VSlot::read(c, vt, 0xc0);
                c.g[A0][0] = get.this(this);
                c.g[V0][0] = get.func;
                call_ptr(c, get.func as u32, 0x0035_6504)?;
                c.g[S2][0] = c.g[V0][0];
            }
            // The UV-scroll matrix of the modifier block (+4) into s0.
            c.cycles += 3;
            let mut block = field(0x1c).lw(c);
            c.g[A0][0] = block;
            let has_uv = block_has(c, block, 4);
            c.cycles += 2;
            if has_uv {
                c.cycles += 2;
                let has_uv = block_has(c, block, 4);
                c.cycles += 2;
                c.g[S0][0] = 0;
                if has_uv {
                    c.cycles += 3;
                    let uv = Addr((block as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = uv;
                    c.g[V0][0] = 0;
                    if uv != 0 {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, UV_SCROLL_GET_MATRIX, 0x0035_6550)?;
                    } else {
                        c.cycles += 1;
                    }
                    c.g[S0][0] = c.g[V0][0];
                    c.cycles += 1;
                    block = field(0x1c).lw(c);
                    c.g[A0][0] = block;
                }
                c.cycles += 1;
                c.g[S6][0] |= 2;
            }
            // The block's +8 part (+0x24 into it), else the model's +0x94.
            c.cycles += 2;
            let has_extra = block_has(c, block, 8);
            c.cycles += 2;
            let model_extra = |c: &mut Cpu| {
                let model = field(0x18).lw(c);
                c.cycles += 1;
                Addr((model as u32).wrapping_add(0x94)).lw(c)
            };
            let extra = if !has_extra {
                model_extra(c)
            } else {
                c.cycles += 2;
                let has_extra = block_has(c, block, 8);
                c.cycles += 2;
                if !has_extra {
                    model_extra(c)
                } else {
                    c.cycles += 3;
                    let part = Addr((block as u32).wrapping_add(8)).lw(c);
                    c.g[A0][0] = part;
                    if part != 0 {
                        c.cycles += 2;
                        sx((part as u32).wrapping_add(0x24))
                    } else {
                        0
                    }
                }
            };
            c.g[A2][0] = extra;
            // draw(gfx, model, extra, matrix, own matrix, uv, flags; alpha).
            c.cycles += 11;
            let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
            c.g[V0][0] = cam_vt;
            c.g[A3][0] = c.g[S2][0];
            let alpha = field(0x24).read_u32(c);
            c.f[12] = alpha;
            c.g[T0][0] = c.g[S1][0];
            let draw = VSlot::read(c, cam_vt, 0x2f8);
            c.g[T1][0] = c.g[S0][0];
            c.g[V1][0] = draw.func;
            c.g[T2][0] = c.g[S6][0];
            c.g[A1][0] = field(0x18).lw(c);
            c.g[A0][0] = draw.this(gfx);
            call_ptr(c, draw.func as u32, 0x0035_65d8)?;
            0
        };
        if restored < 1 {
            c.cycles += 1;
            frame.restore(c, S0, 0x100);
        }
        if restored < 2 {
            c.cycles += 1;
            frame.restore(c, S1, 0xf0);
        }
        c.cycles += 9;
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, fill, put_vec, scramble, Rng, DATA, GAME_GP};

    /// A real guest function that returns 0 (`UIState_TransitionIn`), for
    /// vtable slots whose 0 answer a fake cannot give.
    const RETURN_ZERO: u32 = 0x003a_5ab0;

    /// Vtable slot `off` = `{delta, func}`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// A random `this` adjustment.
    fn delta(r: &mut Rng) -> u32 {
        (r.next_u32() as i16 as i32 & !3) as u32
    }

    // ------------------------------------------------------------ DeferMan

    /// The chain of a random pass for `DeferMan_FindBucket`: `n` nodes from
    /// DATA+0x1000 (0x40 apart); the searched keys in a1/a2/t0. Node `hit`
    /// matches; the others differ in one key, are skipped (+0x14 set) or
    /// are random.
    fn bucket_chain(c: &mut Cpu, r: &mut Rng, n: u32, hit: Option<u32>) {
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
            c.bus.write32(at + defer::NODE_NEXT, if i + 1 < n { node(i + 1) } else { 0 });
        }
    }

    #[test]
    fn defer_man_find_bucket_matches() {
        check(ssx3_ee::s13::f_00395000__generated, defer_man_find_bucket, 300, &|c, seed| {
            let mut r = Rng(0x3950_0000 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let n = [0, 1, 2, 3, 5, 8][(seed % 6) as usize];
            let hit = if n == 0 || seed % 7 == 0 { None } else { Some(r.next_u32() % n) };
            bucket_chain(c, &mut r, n, hit);
        });
        check_events(ssx3_ee::s13::f_00395000__generated, defer_man_find_bucket, 90, &|c, seed| {
            let mut r = Rng(0x3950_e000 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let events = schedule(c, &mut r, seed, 0x0039_5000);
            let n = 4 + r.next_u32() % 6;
            let hit = if seed % 4 == 0 { None } else { Some(n - 1 - r.next_u32() % 2) };
            bucket_chain(c, &mut r, n, hit);
            events
        });
    }

    /// The state chain of a random pass for `DeferMan_InternState`: `n`
    /// records from DATA+0x1000 (0x20 apart), the state at DATA+0x800 or
    /// +0x804 (unaligned doublewords), the pool count random. Record `hit`
    /// equals the state; others share a prefix of 0..3 words or are random.
    fn state_chain(c: &mut Cpu, r: &mut Rng, n: u32, hit: Option<u32>) {
        let man = DATA;
        let pass = r.next_u32();
        let state = DATA + 0x800 + 4 * (r.next_u32() % 2);
        fill(c, r, state, 5);
        c.g[A0][0] = sx(man);
        c.g[A1][0] = sx(state);
        c.g[A2][0] = (r.next_u32() as u64) << 32 | pass as u64;
        let count = r.next_u32() % 64;
        c.bus.write32(man + defer::STATE_POOL + defer::STATE_POOL_COUNT, count);
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
            c.bus.write32(at + defer::STATE_NEXT, if i + 1 < n { rec(i + 1) } else { 0 });
        }
    }

    #[test]
    fn defer_man_intern_state_matches() {
        check_lohi(ssx3_ee::s13::f_00394ed0__generated, defer_man_intern_state, 300, &|c, seed| {
            let mut r = Rng(0x394e_d000 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let n = [0, 1, 2, 3, 5, 8][(seed % 6) as usize];
            let hit = if n == 0 || seed % 7 == 0 { None } else { Some(r.next_u32() % n) };
            state_chain(c, &mut r, n, hit);
        });
        check_events(ssx3_ee::s13::f_00394ed0__generated, defer_man_intern_state, 90, &|c, seed| {
            let mut r = Rng(0x394e_e000 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let events = schedule(c, &mut r, seed, 0x0039_4ed0);
            let n = 3 + r.next_u32() % 5;
            let hit = if seed % 4 == 0 { None } else { Some(n - 1 - r.next_u32() % 2) };
            state_chain(c, &mut r, n, hit);
            events
        });
    }

    /// A matrix table of `count` records (the man at DATA, the new record
    /// at DATA+0x10000). Record `hit` has the new record's key part; about
    /// half of the others have it with one float changed.
    fn matrix_table(c: &mut Cpu, r: &mut Rng, count: i32, hit: Option<u32>) {
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
        check_lohi(ssx3_ee::s12::f_003645b8__generated, defer_man_intern_matrix_record, 300, &|c, seed| {
            let mut r = Rng(0x3645_b800 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            // 0 and -1: no search (-1 appends over the table's own header).
            let count = match seed % 8 {
                0 => 0,
                1 => -1,
                k => k as i32 - 1,
            };
            let hit = if count > 0 && seed % 3 != 0 { Some(r.next_u32() % count as u32) } else { None };
            matrix_table(c, &mut r, count, hit);
        });
        check_events(ssx3_ee::s12::f_003645b8__generated, defer_man_intern_matrix_record, 90, &|c, seed| {
            let mut r = Rng(0x3645_e800 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let events = schedule(c, &mut r, seed, 0x0036_45b8);
            let count = 3 + r.next_u32() % 4;
            let hit = if seed % 4 == 0 { None } else { Some(count - 1) };
            matrix_table(c, &mut r, count as i32, hit);
            events
        });
    }

    // ------------------------------------------------------ spline modifier

    #[test]
    fn spline_modifier_is_node_visible_matches() {
        covered(ssx3_ee::s11::f_00345638__generated, spline_modifier_is_node_visible, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3456_3800, 2);
            let (gfx, cam_vt, node) = (DATA + 0x2000, DATA + 0x3000, DATA + 0x100);
            c.bus.write32(GAME_GP + GFX_MAN, gfx);
            c.bus.write32(gfx + 0x10d8, cam_vt);
            slot(c, cam_vt, 0x158, delta(&mut r), FAKE);
            slot(c, cam_vt, 0x2e8, delta(&mut r), FAKE + 0x10);
            fill(c, &mut r, node, 0x30);
            c.g[A0][0] = sx(node);
            // The test answers 1 (hidden) or a pointer.
            c.bus.write32(LOG_RESULTS + 4, if seed % 2 == 0 { 1 } else { 0 });
        });
    }

    // ------------------------------------------------------ modifier objects

    /// `ModObj_v50`: block / implementation present or not; the radius
    /// slot is the real `ModifierBlock_GetTransformRadius` on an object
    /// without implementation (radius 0), and the position (fakes 1, 2)
    /// equals both corners of the old bounds (fake 0), so the real
    /// `SpatialHash_Update` finds the keys unchanged.
    #[test]
    fn mod_obj_v50_matches() {
        covered(ssx3_ee::s11::f_003568b0__generated, mod_obj_v50, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3568_b000, 5);
            let this = DATA;
            let (block, imp, imp_vt, this_vt) = (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x400);
            let (app, game, old, pos) = (DATA + 0x600, DATA + 0x700, DATA + 0x1000, DATA + 0x1100);
            fill(c, &mut r, this, 0x10);
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0xc, this_vt);
            c.bus.write32(this + 0x18, DATA + 0x500);
            c.bus.write32(this + 0x1c, if seed % 6 == 0 { 0 } else { block });
            c.bus.write32(block, if seed % 6 == 1 { 0 } else { imp });
            c.bus.write32(imp, imp_vt);
            c.bus.write32(imp + 0x10, 0);
            slot(c, imp_vt, 0x60, delta(&mut r), FAKE);
            slot(c, imp_vt, 0x68, delta(&mut r), FAKE + 0x10);
            slot(c, imp_vt, 0x70, 0x10, callee::GET_TRANSFORM_RADIUS.addr);
            slot(c, imp_vt, 0xc0, delta(&mut r), FAKE + 0x20);
            slot(c, this_vt, 0xc0, delta(&mut r), FAKE + 0x30);
            // World_GetNash: *(gp - 0x848) -> +0x84 -> +0x20.
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x20, r.next_u32());
            let p = [r.f32(1000.0), r.f32(1000.0), r.f32(1000.0), ONE];
            put_vec(c, old, p);
            put_vec(c, old + 0x10, p);
            put_vec(c, pos + 0x30, p);
            c.bus.write32(LOG_RESULTS, old);
            c.bus.write32(LOG_RESULTS + 4, pos);
            c.bus.write32(LOG_RESULTS + 8, pos);
        });
    }

    /// `ModObj_Render`: random layer / pass switches, flags and hidden
    /// word; modifier block parts present or not; the object's and the
    /// camera's methods fakes or the real `return 0`; fake answers 1, 7 or
    /// a pointer; `World_IsOccluded` with no occluder or one whose VU0 test
    /// leaves VI1 as set here (0 or 1: occluded); UV scroll through the
    /// real `UVScrollModifier_GetMatrix`.
    #[test]
    fn mod_obj_render_matches() {
        covered(ssx3_ee::s11::f_00356298__generated, mod_obj_render, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3562_9800, 8);
            let this = DATA;
            let (model, this_vt) = (DATA + 0x1000, DATA + 0x2000);
            let (block, imp, imp_vt, uv) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3180, DATA + 0x3200);
            let (gfx, cam_vt, layers, table) = (DATA + 0x4000, DATA + 0x4800, DATA + 0x5000, DATA + 0x6000);
            let (app, game, occl, occluder) = (DATA + 0x7000, DATA + 0x7100, DATA + 0x7200, DATA + 0x7400);
            fill(c, &mut r, this, 0x10);
            fill(c, &mut r, model, 0x40);
            c.g[A0][0] = sx(this);
            // Layer and pass switches.
            c.bus.write32(GAME_GP + LAYER_TABLE, layers);
            c.bus.write32(layers, table);
            let layer = if r.next_u32() % 3 == 0 { 0xff } else { r.next_u32() % 4 };
            c.bus.write8(model + 0x7d, layer as u8);
            c.bus.write16(model + 0x7e, (r.next_u32() % 4) as u16);
            for i in 0..4 {
                let layer_mode = if r.next_u32() % 4 == 0 { r.next_u32() } else { 6 };
                c.bus.write32(table + i * 8 + 0x24, layer_mode);
                let pass_mode = if r.next_u32() % 4 == 0 { r.next_u32() } else { 3 };
                c.bus.write32(table + i * 0x18 + 0x3f0, pass_mode);
            }
            let flags = r.next_u32();
            c.bus.write32(model + 8, if r.next_u32() % 6 == 0 { flags & !4 } else { flags | 4 });
            let hidden = if r.next_u32() % 6 == 0 { r.next_u32() | 1 } else { 0 };
            c.bus.write32(this + 0x28, hidden);
            // The modifier block: implementation, UV scroll, +8 part.
            c.bus.write32(this + 0xc, this_vt);
            c.bus.write32(this + 0x18, model);
            c.bus.write32(this + 0x1c, if r.next_u32() % 4 == 0 { 0 } else { block });
            c.bus.write32(block + 0x10, 0);
            c.bus.write32(block, if r.next_u32() % 3 == 0 { 0 } else { imp });
            c.bus.write32(block + 4, if r.next_u32() % 3 == 0 { 0 } else { uv });
            let part = r.next_u32();
            c.bus.write32(block + 8, if r.next_u32() % 3 == 0 { 0 } else { part });
            c.bus.write32(imp, imp_vt);
            slot(c, imp_vt, 0x58, delta(&mut r), FAKE);
            slot(c, imp_vt, 0xd0, delta(&mut r), FAKE + 0x10);
            fill(c, &mut r, uv, 0x10);
            if r.next_u32() % 2 == 0 {
                c.bus.write32(uv + 0x10, 0);
            }
            // The object's methods.
            let zero_or = |r: &mut Rng, fake: u32| if r.next_u32() % 2 == 0 { RETURN_ZERO } else { fake };
            let (d, f) = (delta(&mut r), zero_or(&mut r, FAKE + 0x20));
            slot(c, this_vt, 0x160, d, f);
            slot(c, this_vt, 0x168, delta(&mut r), FAKE + 0x30);
            let (d, f) = (delta(&mut r), zero_or(&mut r, FAKE + 0x40));
            slot(c, this_vt, 0xd0, d, f);
            slot(c, this_vt, 0xd8, delta(&mut r), FAKE + 0x50);
            slot(c, this_vt, 0xc0, delta(&mut r), FAKE + 0x60);
            // The graphics manager's camera methods.
            c.bus.write32(GAME_GP + GFX_MAN, gfx);
            c.bus.write32(gfx + 0x10d8, cam_vt);
            slot(c, cam_vt, 0x158, delta(&mut r), FAKE + 0x70);
            let (d, f) = (delta(&mut r), zero_or(&mut r, FAKE + 0x80));
            slot(c, cam_vt, 0x2e8, d, f);
            slot(c, cam_vt, 0x2f8, delta(&mut r), FAKE + 0x90);
            // World_IsOccluded: *(gp - 0x848) -> +0x84 -> +0x8c occluders.
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x8c, occl);
            c.bus.write32(occl + 0x10c, r.next_u32() % 2);
            c.bus.write32(occl + 0x110, 1);
            c.bus.write32(occl + 8, occluder);
            fill(c, &mut r, occluder + 0x50, 0x14);
            c.bus.plat.write_ctrl(1, r.next_u32() % 2);
            // Fake answers: a pointer, 1 (hidden) or 7 (UV-scroll block).
            for n in 0..8 {
                c.bus.write32(LOG_RESULTS + 4 * n, [0, 1, 7][(r.next_u32() % 3) as usize]);
            }
        });
    }
}
