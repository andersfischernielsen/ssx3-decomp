//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for a mixed bag of hot helpers and small dispatchers — the
//! thread/semaphore syscall stubs, rider/audio/replay predicates, the two
//! Nash octree-node classifiers, and the rider-distance getter — each with a
//! differential test against the generated body.
//!
//! Calls follow [`crate::call`]: the same registers at each call, the same
//! return address, the unwind check after it, and the same cycle charge per
//! block. The dispatchers (`SSXAudioSystem_GetSurfaceGroup`,
//! `SSXAudioSystem_GetSurfaceSoundType`, `Replay_IsPaused`, the Nash
//! classifiers) read their jump tables from guest memory and jump like the
//! original; their resume entries are the table targets and the blocks the
//! table can reach (a thread switch can re-enter them there).
//!
//! Loops that check events use [`event_due`]/[`deliver_event_here`] at the
//! same back edge.

use crate::abi::{self, r32};
use crate::call::{guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0011_3128,
        rider_get_dist_to_finish,
        "Rider_GetDistToFinish",
    ),
    (0x0042_3c90, get_thread_id, "GetThreadId"),
    (0x0042_3dc0, signal_sema, "SignalSema"),
    (0x0042_3dd0, i_signal_sema, "iSignalSema"),
    (0x0042_3de0, wait_sema, "WaitSema"),];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    /// `Rider_GetMainState(Rider*)`: `*(*(rider + 0x77c) + 0xde0)`.
    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
}

/// `Replay_IsPaused`'s state table (`0x480000 + 0x17c0`), indexed by the
/// replay object's first word (only states < 0x10 are looked up).
const REPLAY_TABLE: u32 = 0x0048_17c0;
/// `Nash_ClassifyPatch`'s table (`0x480000 - 0x4ca0`), indexed by the heap
/// handle (only handles < 5 are looked up).
const NASH_PATCH_TABLE: u32 = 0x0047_b360;
/// `Nash_ClassifyInstance`'s table (`0x480000 - 0x4cc0`).
const NASH_INSTANCE_TABLE: u32 = 0x0047_b340;
/// `SSXAudioSystem_GetSurfaceSoundType`'s table (`0x480000 + 0x2aa0`),
/// indexed by the surface type id + 1 (ids < 0x15).
const SURFACE_SOUND_TABLE: u32 = 0x0048_2aa0;
/// `SSXAudioSystem_GetSurfaceGroup`'s table (`0x480000 + 0x2b50`), indexed by
/// the group id (ids < 9).
const SURFACE_GROUP_TABLE: u32 = 0x0048_2b50;

// ---------------------------------------------------------------------------
// Trivial leaves and syscall stubs.

/// `float func_00113128(Rider *r)` (0x113128): the rider's distance to the
/// finish line, a plain field read at `+0x4d0`. (The reference has assembly
/// only; the generated body reads f0 and returns.)
pub fn rider_get_dist_to_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, r32(c, A0).wrapping_add(0x4d0));
    abi::ret(c);
}

/// `int GetThreadId(void)` (0x423c90): syscall 0x2f, then one event check
/// (the generated code tests for a due event before returning).
pub fn get_thread_id(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x2f;
    c.pc = 0x0042_3c98;
    c.syscall(0x0042_3c94);
    if c.pc != 0x0042_3c98 {
        return;
    }
    if event_due(c) {
        deliver_event_here(c, 0x0042_3c98);
    }
    abi::ret(c);
}

/// `int SignalSema(int sema)` (0x423dc0): syscall 0x42.
pub fn signal_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x42;
    c.pc = 0x0042_3dc8;
    c.syscall(0x0042_3dc4);
    if c.pc != 0x0042_3dc8 {
        return;
    }
    if event_due(c) {
        deliver_event_here(c, 0x0042_3dc8);
    }
    abi::ret(c);
}

/// `int iSignalSema(int sema)` (0x423dd0): syscall 0x43 (the interrupt-level
/// variant; the generated code leaves `v1` sign-extended from 0xffffffbd,
/// i.e. the 64-bit syscall word -0x43).
pub fn i_signal_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xffff_ffff_ffff_ffbd;
    c.pc = 0x0042_3dd8;
    c.syscall(0x0042_3dd4);
    if c.pc != 0x0042_3dd8 {
        return;
    }
    if event_due(c) {
        deliver_event_here(c, 0x0042_3dd8);
    }
    abi::ret(c);
}

/// `int WaitSema(int sema)` (0x423de0): syscall 0x44.
pub fn wait_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x44;
    c.pc = 0x0042_3de8;
    c.syscall(0x0042_3de4);
    if c.pc != 0x0042_3de8 {
        return;
    }
    if event_due(c) {
        deliver_event_here(c, 0x0042_3de8);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay / audio predicates.

/// `int Replay_IsPaused(Replay *r)` (0x270280): the replay state word at
/// `+0` picks a handler from the table at [`REPLAY_TABLE`]. States 1 and 7
/// compare two counters (`+0x484` against `+0x3cc`, and `+0x604` must be
/// zero); states 3 and 5 return true, state 4 returns `+0x484 >= +0x3cc`,
/// most states return false. States >= 0x10 return false without a lookup.
///
/// One block per table target, in the generated code's order. The function
/// makes no calls and checks no events, so its only entry is the top.
pub fn replay_is_paused(c: &mut Cpu) {
    c.cycles += 5;
    let replay = r32(c, A0);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = Addr(replay).lw(c);
    c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A3][0] = 0;
    if !out_of_range {
        c.cycles += 7;
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx(REPLAY_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            // State 1 / 7: a pause request is visible in the two counters.
            0x0027_02b0 => {
                c.cycles += 9;
                c.g[A0][0] = 0x1;
                c.g[A1][0] = Addr(replay).off(0x3cc).lw(c);
                c.g[V1][0] = Addr(replay).off(0x604).lw(c);
                c.g[A3][0] = c.g[A0][0];
                c.g[V0][0] = Addr(replay).off(0x484).lw(c);
                if c.g[V1][0] != 0 {
                    c.g[A3][0] = 0;
                }
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[A0][0];
                }
            }
            // State 4: not yet caught up.
            0x0027_02d4 => {
                c.cycles += 5;
                c.g[V1][0] = Addr(replay).off(0x3cc).lw(c);
                c.g[V0][0] = Addr(replay).off(0x484).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A3][0] = c.g[V0][0] ^ 0x1;
            }
            // States 3 and 5: paused.
            0x0027_02e8 => {
                c.cycles += 1;
                c.g[A3][0] = 0x1;
            }
            // Any other table entry keeps the zero result.
            0x0027_02ec => {}
            target => {
                c.jump_out(target, 0x0027_0280, 0x0027_02f4);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

/// `void Nash_ClassifyPatch(Cell *cell, Node *node, int frustum)` (0x22a128):
/// classify one octree node under the cell's heap. A null node leaves `v0` =
/// `heap < 5` (the tail is then skipped); otherwise the node's signed word at
/// `+0x156` is checked for a handle and its byte at `+0x155` indexes the
/// cell's heap (`heap + byte*8 + 0x24`, the slot word must be 6), and the
/// handle selects `heap + handle*0x18 + 0x3f0` whose word must be 3. A hit
/// runs the VU0 bounding-box/frustum classification and then, when the heap
/// handle is below five, inserts the node into one of the cell's two lists
/// (head at `+0x78b0`, array at `+0x50ac`/`+0x70ac`).
///
/// The function makes no calls and checks no events, so the only entry the
/// trampoline can ever use is the top (the generated `__entries` twin's table
/// targets are only reachable from inside).
pub fn nash_classify_patch(c: &mut Cpu) {
    c.cycles += 3;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let cell = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    let node = r32(c, A1);
    let heap;
    let mut classified;
    if node == 0 {
        c.cycles += 2;
        c.cfc2(4, 1);
        heap = r32(c, A0);
        classified = (c.g[A0][0] < 0x5) as u64;
    } else {
        // 0x22a134: the node's handle and slot.
        c.cycles += 5;
        c.g[V0][0] = Addr(cell).off(0x90).lw(c);
        c.g[T0][0] = Addr(node).off(0x156).lh(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        heap = r32(c, A0);
        c.g[V0][0] = 0;
        classified = 0;
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(node).off(0x155).lbu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(heap.wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
            c.g[V1][0] ^= 0x6;
            if c.g[V1][0] == 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x18;
                let i = abi::mult(c, r32(c, T0), 0x18);
                c.g[V1][0] = sx(i);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(heap));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3f0).lw(c);
                c.g[V1][0] ^= 0x3;
                c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
            } else {
                c.g[V0][0] = 0;
            }
            classified = c.g[V0][0];
        }
        c.cycles += 2;
        if classified != 0 {
            // 0x22a184: build the node's box on the stack, classify it with
            // the cell's VU0 routine and keep the handle test for the tail.
            c.cycles += 26;
            for i in 0..6u32 {
                c.lwc1(i as usize, node.wrapping_add(0x158 + 4 * i));
            }
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 6);
            c.swc1(0, sp(c));
            c.swc1(1, sp(c).wrapping_add(0x4));
            c.swc1(2, sp(c).wrapping_add(0x8));
            c.swc1(3, sp(c).wrapping_add(0x10));
            c.swc1(4, sp(c).wrapping_add(0x14));
            c.swc1(5, sp(c).wrapping_add(0x18));
            c.swc1(6, sp(c).wrapping_add(0x1c));
            c.swc1(6, sp(c).wrapping_add(0xc));
            c.cfc2(4, 1);
            c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
            c.g[V0][0] = sx(node.wrapping_add(0x140));
            c.lqc2(20, sp(c));
            c.lqc2(21, r32(c, V1));
            c.lqc2(22, r32(c, V0));
            c.ctc2(6, 14);
            c.vcallms(0xdb8);
            c.g[V0][0] = (heap < 0x5) as u64;
            classified = c.g[V0][0];
        }
        let _ = classified;
    }
    // 0x22a1f4: only a heap handle below five gets a list insert.
    c.cycles += 2;
    let skip = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0048_0000;
    if !skip {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(NASH_PATCH_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let mut insert = true;
        match c.g[A0][0] as u32 {
            0x0022_a214 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x78b0).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(cell).off(0x50a8).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x50ac).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x50a8).sw(c, c.g[V1][0]);
                }
            }
            0x0022_a260 => {}
            0x0022_a23c => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x78b0).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(cell).off(0x70ac).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x70b0).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x70ac).sw(c, c.g[V1][0]);
                }
            }
            target => {
                c.jump_out(target, 0x0022_a128, 0x0022_a26c);
                return;
            }
        }
        if insert {
            c.cycles += 1;
            Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// `void Nash_ClassifyInstance(Cell *cell, Node *node, int frustum)`
/// (0x229fc8): the same classification for the other node flavour. The node
/// is skipped unless its `+0x8` word ends in 3; its byte at `+0x7d` (0xff
/// means "no slot") indexes `heap + byte*8 + 0x24` (slot word must be 6) and
/// its signed halfword at `+0x7e` selects `heap + handle*0x18 + 0x3f0` (word
/// must be 3). A hit classifies the box at `+0x60` and inserts into
/// `+0x50a4`/`+0x20a0` or `+0x40a0`.
///
/// Like `Nash_ClassifyPatch` it makes no calls and checks no events, so the
/// top is its only entry.
pub fn nash_classify_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let cell = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    let node = r32(c, A1);
    let mut heap = 0u32;
    if node == 0 {
        c.cycles += 2;
        c.cfc2(4, 1);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    } else {
        // 0x229fd4: only nodes whose type word ends in 3 are classified.
        c.cycles += 5;
        c.g[V0][0] = Addr(node).off(0x8).lw(c);
        c.g[V1][0] = 0x3;
        c.g[V0][0] &= 0x3;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = 0;
        } else {
            // 0x229fe8: the node's slot byte indexes the cell's heap.
            c.cycles += 6;
            c.g[V0][0] = Addr(cell).off(0x90).lw(c);
            c.g[A0][0] = 0xff;
            c.g[V1][0] = Addr(node).off(0x7d).lbu(c);
            c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = 0x1;
            heap = r32(c, T0);
            if c.g[V1][0] != c.g[A0][0] {
                c.cycles += 7;
                c.g[V0][0] = Addr(node).off(0x7d).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
                c.g[V1][0] ^= 0x6;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(node).off(0x7e).lh(c);
                    c.g[A0][0] = 0x18;
                    let i = abi::mult(c, r32(c, V0), 0x18);
                    c.g[V1][0] = sx(i);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(heap));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3f0).lw(c);
                    c.g[V1][0] ^= 0x3;
                    c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
                }
            }
        }
        // 0x22a038: a classified node is tested again with the VU0 routine.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 26;
            for i in 0..6u32 {
                c.lwc1(i as usize, node.wrapping_add(0x60 + 4 * i));
            }
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 6);
            c.swc1(0, sp(c));
            c.swc1(1, sp(c).wrapping_add(0x4));
            c.swc1(2, sp(c).wrapping_add(0x8));
            c.swc1(3, sp(c).wrapping_add(0x10));
            c.swc1(4, sp(c).wrapping_add(0x14));
            c.swc1(5, sp(c).wrapping_add(0x18));
            c.swc1(6, sp(c).wrapping_add(0x1c));
            c.swc1(6, sp(c).wrapping_add(0xc));
            c.cfc2(4, 1);
            c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
            c.g[V0][0] = sx(node.wrapping_add(0x50));
            c.lqc2(20, sp(c));
            c.lqc2(21, r32(c, V1));
            c.lqc2(22, r32(c, V0));
            c.ctc2(6, 14);
            c.vcallms(0xdb8);
            c.g[V0][0] = (heap < 0x5) as u64;
        }
    }
    // 0x22a0b0: heap handles below five get a list insert.
    c.cycles += 2;
    let skip = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0048_0000;
    if !skip {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(NASH_INSTANCE_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let mut insert = true;
        match c.g[A0][0] as u32 {
            0x0022_a0d0 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x50a4).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(cell).off(0x209c).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x20a0).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x209c).sw(c, c.g[V1][0]);
                }
            }
            0x0022_a11c => {}
            0x0022_a0f8 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x50a4).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(cell).off(0x40a0).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x40a4).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x40a0).sw(c, c.g[V1][0]);
                }
            }
            target => {
                c.jump_out(target, 0x0022_9fc8, 0x0022_a128);
                return;
            }
        }
        if insert {
            c.cycles += 1;
            Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// `int func_00291710(SSXAudioSystem *sys, Rider *rider)` (0x291710): the
/// surface sound type for the rider's current surface. The rider's main
/// state (`Rider_GetMainState`) picks `+0x438` normally, or the system's
/// `+0x598c` word in state 4; the id + 1 then maps through the table at
/// [`SURFACE_SOUND_TABLE`] (ids above 0x14 give 0).
pub fn ssx_audio_get_surface_sound_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_GET_MAIN_STATE.call(c, 0x0029_1730)?;
        c.g[V1][0] = 0x4;
        let mut id;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = Addr(r32(c, S0)).off(0x438).lw(c);
            id = c.g[V0][0];
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x598c).lw(c);
            id = c.g[V0][0];
        }
        // 0x291740: lookup by id + 1.
        c.cycles += 4;
        c.g[V1][0] = sx((id as u32).wrapping_add(0x1));
        c.g[V0][0] = (c.g[V1][0] < 0x15) as u64;
        let out_of_range = c.g[V0][0] == 0;
        c.g[V0][0] = 0x0048_0000;
        if out_of_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(SURFACE_SOUND_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x0029_1768 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                }
                0x0029_1770 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2;
                }
                0x0029_1778 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x3;
                }
                0x0029_1780 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x7;
                }
                0x0029_1788 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x5;
                }
                0x0029_1790 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x4;
                }
                0x0029_1798 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x8;
                }
                0x0029_17a0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                target => {
                    c.jump_out(target, 0x0029_1710, 0x0029_17b8);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_002934D0(SSXAudioSystem *sys, int group, Rider *rider)`
/// (0x2934d0): the surface group a rider's current surface belongs to. Group
/// ids below nine are looked up in the table at [`SURFACE_GROUP_TABLE`]; one
/// handler asks `Rider_GetMainState` and returns 2 unless the state is 4.
pub fn ssx_audio_get_surface_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A1][0] < 0x9) as u64;
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut result = 0u64;
        if !out_of_range {
            c.cycles += 7;
            c.g[V0][0] = 0x0048_0000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx(SURFACE_GROUP_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x0029_34fc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[A2][0];
                    callee::RIDER_GET_MAIN_STATE.call(c, 0x0029_3504)?;
                    c.g[V0][0] ^= 0x4;
                    c.g[V1][0] = 0x2;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = 0;
                    }
                    result = c.g[V1][0];
                }
                0x0029_3514 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x1;
                    result = c.g[V1][0];
                }
                0x0029_351c => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x2;
                    result = c.g[V1][0];
                }
                0x0029_3524 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0;
                    result = 0;
                }
                target => {
                    c.jump_out(target, 0x0029_34d0, 0x0029_3538);
                    return Ok(());
                }
            }
        } else {
            // 0x293524: an unknown group id is group 0.
            c.cycles += 1;
            c.g[V1][0] = 0;
            result = 0;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = result;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, put_vec, put_words, Rng, DATA};

    /// A Rider with a main-state word the (real) `Rider_GetMainState` callee
    /// reads: `rider + 0x77c -> + 0xde0`.
    fn rider_state(c: &mut Cpu, r: &mut Rng, state: u32) -> u32 {
        let rider = DATA + 0x800;
        c.bus.write32(rider + 0x77c, DATA + 0x900);
        c.bus.write32(DATA + 0x900 + 0xde0, state);
        c.bus.write32(rider + 0x438, r.next_u32() % 24);
        rider
    }

    #[test]
    fn rider_get_dist_to_finish_matches() {
        check(
            ssx3_ee::s00::f_00113128__generated,
            rider_get_dist_to_finish,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1131_28, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x4d0, r.next_u32());
            },
        );
    }

    #[test]
    fn get_thread_id_matches() {
        check(ssx3_ee::s15::f_00423c90__generated, get_thread_id, 40, &|c, seed| {
            machine(c, seed, 0x423c_90, 0);
            c.g[A0][0] = sx(seed);
        });
    }

    #[test]
    fn signal_sema_matches() {
        check(ssx3_ee::s15::f_00423dc0__generated, signal_sema, 40, &|c, seed| {
            machine(c, seed, 0x423d_c0, 0);
            c.g[A0][0] = sx(seed);
        });
    }

    #[test]
    fn i_signal_sema_matches() {
        check(ssx3_ee::s15::f_00423dd0__generated, i_signal_sema, 40, &|c, seed| {
            machine(c, seed, 0x423d_d0, 0);
            c.g[A0][0] = sx(seed);
        });
    }

    #[test]
    fn wait_sema_matches() {
        check(ssx3_ee::s15::f_00423de0__generated, wait_sema, 40, &|c, seed| {
            machine(c, seed, 0x423d_e0, 0);
            c.g[A0][0] = sx(seed);
        });
    }

    /// The shipped replay state table at [`REPLAY_TABLE`].
    const REPLAY_HANDLERS: [u32; 16] = [
        0x0027_02ec,
        0x0027_02b0,
        0x0027_02ec,
        0x0027_02e8,
        0x0027_02d4,
        0x0027_02e8,
        0x0027_02e8,
        0x0027_02b0,
        0x0027_02e8,
        0x0027_02e8,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
    ];

        /// The shipped Nash patch table at [`NASH_PATCH_TABLE`].
    const NASH_PATCH_HANDLERS: [u32; 5] = [
        0x0022_a214,
        0x0022_a260,
        0x0022_a260,
        0x0022_a214,
        0x0022_a23c,
    ];
    /// The shipped Nash instance table at [`NASH_INSTANCE_TABLE`].
    const NASH_INSTANCE_HANDLERS: [u32; 5] = [
        0x0022_a0d0,
        0x0022_a11c,
        0x0022_a11c,
        0x0022_a0d0,
        0x0022_a0f8,
    ];

            /// The shipped surface sound-type table at [`SURFACE_SOUND_TABLE`].
    const SURFACE_SOUND_HANDLERS: [u32; 21] = [
        0x0029_17a0,
        0x0029_17a0,
        0x0029_1770,
        0x0029_1768,
        0x0029_1768,
        0x0029_1778,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_1780,
        0x0029_1780,
        0x0029_1788,
        0x0029_1790,
        0x0029_1798,
        0x0029_17a0,
        0x0029_1768,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_1780,
        0x0029_17a0,
    ];

        /// The shipped surface group table at [`SURFACE_GROUP_TABLE`].
    const SURFACE_GROUP_HANDLERS: [u32; 9] = [
        0x0029_3524,
        0x0029_3514,
        0x0029_3514,
        0x0029_34fc,
        0x0029_351c,
        0x0029_351c,
        0x0029_351c,
        0x0029_351c,
        0x0029_351c,
    ];

    }
