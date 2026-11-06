//! Staging wave 210 (oc281): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::guest::Addr;
use ssx3_ee_rt::vu0::Src;

/// Rider sphere tree node: the bounding sphere's centre and its children.
mod sphere_tree_f {
    /// Centre of the node's bounding sphere (quadword).
    pub const CENTER: u32 = 0x10;
    /// Number of child spheres.
    pub const COUNT: u32 = 0x2c;
    /// First child (0x20 bytes each, position quadword first).
    pub const CHILDREN: u32 = 0x30;
    pub const CHILD_SIZE: u32 = 0x20;
}

/// `void RiderSphereTree_Translate(sSphereTreeNode* s, sSphereVec4* v)`
/// (0x329b40): moves a rider's collision sphere tree by `v`: adds it to the
/// node centre and then to each child's position (VU0 `vadd.xyzw`).
pub fn rider_sphere_tree_translate(c: &mut Cpu) {
    use sphere_tree_f::*;
    let node = r32(c, A0);
    c.cycles += 7;
    c.lqc2(3, node.wrapping_add(CENTER));
    c.lqc2(4, r32(c, A1));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, node.wrapping_add(CENTER));
    c.g[V0][0] = Addr(node.wrapping_add(COUNT)).lw(c);
    c.g[A2][0] = 0;
    if c.g[V0][0] as i64 > 0 {
        c.cycles += 1;
        c.g[V1][0] = sx(node.wrapping_add(CHILDREN));
        loop {
            // (Registers re-read every pass: an event may run in between.)
            let child = r32(c, V1);
            let node = r32(c, A0);
            c.cycles += 9;
            c.lqc2(3, child);
            c.lqc2(4, r32(c, A1));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, child);
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            let count = Addr(node.wrapping_add(COUNT)).lw(c);
            let more = (c.g[A2][0] as i64) < (count as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = sx(child.wrapping_add(CHILD_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9b60);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `SSXAudioSystem_GetListenerCount` (0x288ce8).
const AUDIO_GET_LISTENER_COUNT: Callee = Callee {
    addr: 0x0028_8ce8,
    f: ee::s07::SSXAudioSystem_GetListenerCount,
};
/// `SSXAudioSystem_GetViewPos` (0x288ac0).
const AUDIO_GET_VIEW_POS: Callee = Callee {
    addr: 0x0028_8ac0,
    f: ee::s07::SSXAudioSystem_GetViewPos,
};
/// The loop body of [`ssx_audio_find_view_in_range`] (a resume entry).
const FIND_VIEW_BODY: u32 = 0x0028_8b78;

/// `int SSXAudioSystem_FindViewInRange(void* self, sAudV4* pos, float radius)`
/// (0x288b40): the index of the first listener view whose position lies
/// within `radius * 100` of `pos` (squared 4D distance from VU0), else -1.
/// The loop head (0x288b78) is a resume entry: an event at the back edge
/// can unwind there.
pub fn ssx_audio_find_view_in_range(c: &mut Cpu) {
    let frame = Frame { size: 0x70 };
    guest(c, |c| {
        match c.pc {
            0x0028_8b40 => {
                c.cycles += 14;
                Frame::open(c, 0x70);
                c.g[AT][0] = 0x42c8_0000; // 100.0f
                c.mtc1(AT, 0);
                frame.save_fpr(c, 20, 0x60);
                c.mul_s(20, 12, 0);
                frame.save(c, S0, 0x50);
                frame.save(c, S1, 0x40);
                c.g[S0][0] = 0;
                frame.save(c, S2, 0x30);
                c.g[S1][0] = c.g[A0][0];
                // f20 = (radius * 100)^2
                c.mul_s(20, 20, 20);
                frame.save_ra(c, 0x20);
                c.g[S2][0] = c.g[A1][0];
            }
            FIND_VIEW_BODY => {
                if find_view_in_range_body(c, frame)? {
                    return find_view_in_range_exit(c, frame);
                }
            }
            pc => {
                c.jump_out(pc, 0x0028_8b40, 0x0028_8c04);
                return Ok(());
            }
        }
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            AUDIO_GET_LISTENER_COUNT.call(c, 0x0028_8bd8)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX; // -1: none in range
                break;
            }
            c.g[A0][0] = c.g[S1][0];
            if event_due(c) && crate::events::back_edge_unwinds(c, FIND_VIEW_BODY) {
                return Ok(());
            }
            if find_view_in_range_body(c, frame)? {
                break;
            }
        }
        find_view_in_range_exit(c, frame)
    })
}

/// One view of [`ssx_audio_find_view_in_range`] (0x288b78): true (with
/// v0 = index) when view `s0` is in range, else `s0 += 1`.
fn find_view_in_range_body(
    c: &mut Cpu,
    frame: Frame,
) -> Result<bool, crate::call::Unwound> {
    use ssx3_ee_rt::vu0::{W, Y, Z};
    c.cycles += 21;
    c.g[A1][0] = c.g[S0][0];
    AUDIO_GET_VIEW_POS.call(c, 0x0028_8b80)?;
    // d = view_pos - pos, through the stack as the original spills it.
    c.lqc2(3, r32(c, V0));
    c.lqc2(4, r32(c, S2));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x10));
    c.lq(V0, frame.at(c, 0x10));
    c.sq(V0, frame.at(c, 0));
    c.lqc2(3, frame.at(c, 0));
    c.lqc2(5, frame.at(c, 0));
    // dot(d, d) in vf4.x (vf6.x = 1.0)
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_lt_s(0, 20);
    c.g[V0][0] = c.g[S0][0];
    if c.fcc() {
        return Ok(true);
    }
    c.cycles += 1;
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
    Ok(false)
}

/// The epilogue of [`ssx_audio_find_view_in_range`] (0x288be8).
fn find_view_in_range_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 20, 0x60);
    frame.close(c);
    Ok(())
}

/// `ModObj_ctor` (0x355280).
const MOD_OBJ_CTOR: Callee = Callee {
    addr: 0x0035_5280,
    f: ee::s11::ModObj_ctor,
};

/// Object (scene object node) fields.
mod object_f {
    /// Vtable pointer.
    pub const VTABLE: u32 = 0xc;
    /// The world node (instance) this object was built from.
    pub const NODE: u32 = 0x18;
    /// `Object`'s vtable (0x490e80).
    pub const OBJECT_VTABLE: u32 = 0x0049_0e80;
    /// Node flags word.
    pub const NODE_FLAGS: u32 = 0x8;
}

/// `Object* Object_ctor(Object* self, int list, WorldNode* node, int a3,
/// int a4)` (0x356db0): builds the `ModObj` base (process kind 0x11),
/// installs the `Object` vtable, then adjusts the node flags: with `a4` (or
/// when bits 0-1 are both set) clear bit 1 and set bit 2; with `a3` clear
/// bit 6 and set bit 5. Returns `self`.
pub fn object_ctor(c: &mut Cpu) {
    use object_f::*;
    let frame = Frame { size: 0x40 };
    guest(c, |c| {
        c.cycles += 15;
        Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[T0][0];
        c.g[A2][0] = 0x11;
        MOD_OBJ_CTOR.call(c, 0x0035_6ddc)?;
        let this = r32(c, S1);
        c.g[V1][0] = OBJECT_VTABLE as u64;
        Addr(this.wrapping_add(VTABLE)).sw(c, OBJECT_VTABLE as u64);
        let mut set_bit2 = c.g[S0][0] != 0;
        if !set_bit2 {
            c.cycles += 6;
            c.g[V0][0] = Addr(this.wrapping_add(NODE)).lw(c);
            c.g[A0][0] = 3;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(NODE_FLAGS)).lw(c) & 3;
            set_bit2 = c.g[V1][0] == 3;
        }
        if set_bit2 {
            c.cycles += 7;
            c.g[A0][0] = Addr(this.wrapping_add(NODE)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            let flags = Addr(r32(c, A0).wrapping_add(NODE_FLAGS)).lw(c);
            c.g[V0][0] = (flags & 0xffff_ffff_ffff_fffd) | 4;
            let v = c.g[V0][0];
            Addr(r32(c, A0).wrapping_add(NODE_FLAGS)).sw(c, v);
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        if c.g[S2][0] != 0 {
            c.cycles += 8;
            c.g[A0][0] = Addr(this.wrapping_add(NODE)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_ffbf;
            let flags = Addr(r32(c, A0).wrapping_add(NODE_FLAGS)).lw(c);
            c.g[V0][0] = (flags & 0xffff_ffff_ffff_ffbf) | 0x20;
            let v = c.g[V0][0];
            Addr(r32(c, A0).wrapping_add(NODE_FLAGS)).sw(c, v);
            c.g[V0][0] = c.g[S1][0];
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

/// `DLHeap_Free` (0x31ed60).
const DLHEAP_FREE: Callee = Callee {
    addr: 0x0031_ed60,
    f: ee::s10::DLHeap_Free,
};

/// `void MEM_HeapFree(DLHeap* heap, void* mem)` (0x3200c0): forwards to
/// `DLHeap_Free` (a 7-cycle `ra`-saving wrapper).
pub fn mem_heap_free(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, crate::call::Target::Direct(DLHEAP_FREE), 0x0032_00d0);
}

/// Path-network globals and records used by [`path_select_branch`].
mod path_f {
    /// `PATH_gNetwork` (the loaded path network), `lw 0x44f420`.
    pub const NETWORK: u32 = 0x0044_f420;
    /// Network: base of the packed node records.
    pub const NODE_BASE: u32 = 0x34;
    /// Network: node index -> record word offset (`i16[]`).
    pub const NODE_INDEX: u32 = 0x38;
    /// Network: per-track trackers (`slot` from the node header, + 0x58).
    pub const TRACKERS: u32 = 0x58;
    /// Tracker: the node it is queued on (`i16`).
    pub const TRACKER_NODE: u32 = 0x2c;
    /// Tracker: flags; low nibble = signed lane / remaining count.
    pub const TRACKER_LANE: u32 = 0x20;
    /// Node record: second word (branch count in bits 12-16).
    pub const NODE_WORD1: u32 = 0x4;
    /// Node record: the branch list (`{i8 lo, i8 hi, i16 target}`).
    pub const BRANCHES: u32 = 0x10;
}

/// `int PATH_SelectBranch(int node, int lane)` (0x3d37a0), EA path library:
/// pick the branch out of path node `node` whose lane range `[lo, hi]`
/// contains `lane`, else the branch whose range is closest (-1 for a node
/// without branches). If the node's tracker is queued on this node, its
/// own lane (low nibble of +0x20, sign-extended) replaces `lane`, and with a
/// positive node count it is consumed (decremented; at -1 the tracker is
/// released by setting its node to -1). A leaf; reproduces every scratch
/// register of the original.
pub fn path_select_branch(c: &mut Cpu) {
    use path_f::*;
    let sext4 = |v: u32| sx(((v << 28) as i32 >> 28) as u32);
    c.cycles += 19;
    c.g[V0][0] = 0x45_0000;
    c.g[T2][0] = c.g[A1][0]; // lane
    c.g[A2][0] = Addr(NETWORK).lw(c);
    c.g[V1][0] = sx(r32(c, A0) << 1);
    c.g[T0][0] = u64::MAX; // result: -1
    let net = r32(c, A2);
    c.g[A1][0] = Addr(net.wrapping_add(NODE_INDEX)).lw(c);
    c.g[A3][0] = Addr(net.wrapping_add(NODE_BASE)).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = Addr(r32(c, V1)).lh(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0))); // node record
    c.g[V1][0] = Addr(r32(c, A3)).lw(c); // header
    c.g[V0][0] = sx(r32(c, V1) >> 14) & 0x7c;
    c.g[A2][0] = sx(net.wrapping_add(r32(c, V0)));
    c.g[A1][0] = Addr(r32(c, A2).wrapping_add(TRACKERS)).lw(c); // tracker
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(TRACKER_NODE)).lh(c);
    c.g[T1][0] = sx(r32(c, A3).wrapping_add(BRANCHES));
    if c.g[V0][0] == c.g[A0][0] {
        // The tracker sits on this node: its lane decides.
        c.cycles += 6;
        let tracker = r32(c, A1);
        c.g[A0][0] = Addr(tracker.wrapping_add(TRACKER_LANE)).lw(c);
        c.g[V1][0] = sx(((r32(c, V1) as i32) >> 28) as u32); // node count
        c.g[V0][0] = sext4(r32(c, A0));
        c.g[T2][0] = c.g[V0][0] & 0x7f;
        if c.g[V1][0] as i64 > 0 {
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[V1][0] = c.g[A0][0] & 0xffff_ffff_ffff_fff0;
            c.g[A0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] |= c.g[A0][0];
            c.g[V0][0] = sext4(r32(c, V0));
            let lane = c.g[V1][0];
            Addr(tracker.wrapping_add(TRACKER_LANE)).sw(c, lane);
            if c.g[V0][0] == c.g[T0][0] {
                c.cycles += 1;
                let t0 = c.g[T0][0];
                Addr(tracker.wrapping_add(TRACKER_NODE)).sh(c, t0);
            }
        }
    }
    // First branch whose [lo, hi] contains the lane.
    c.cycles += 3;
    c.g[A0][0] = 0;
    c.g[A3][0] = Addr(r32(c, A3).wrapping_add(NODE_WORD1)).lw(c);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, A3) >> 12);
        c.g[A1][0] = c.g[V0][0] & 0x1f; // branch count
        let inside = c.g[A0][0] < c.g[A1][0];
        c.g[V1][0] = sx(r32(c, A0) << 2);
        if !inside {
            break;
        }
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T1)));
        let entry = r32(c, V1);
        c.g[V0][0] = Addr(entry).lb(c);
        c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(entry.wrapping_add(1)).lb(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[T0][0] = Addr(entry.wrapping_add(2)).lh(c);
                break;
            }
        }
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, 0x003d_3838);
        }
    }
    c.cycles += 2;
    if (c.g[T0][0] as i64) < 0 {
        // None contains it: the branch with the nearest end.
        c.cycles += 2;
        c.g[T3][0] = c.g[T1][0];
        if c.g[A1][0] as i64 <= 0 {
            c.cycles += 2;
            c.g[T0][0] = u64::MAX;
        } else {
            c.cycles += 2;
            c.g[T5][0] = 0x7f; // best distance
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[T1][0] = c.g[A1][0];
                c.g[T4][0] = u64::MAX;
                c.g[T0][0] = c.g[T3][0];
                loop {
                    c.cycles += 16;
                    let entry = r32(c, T0);
                    c.g[A1][0] = Addr(entry).lb(c);
                    c.g[A0][0] = Addr(entry.wrapping_add(1)).lb(c);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, T2)));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
                    c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[A3][0] = sx(0u32.wrapping_sub(r32(c, A1)));
                    c.g[A2][0] = sx(0u32.wrapping_sub(r32(c, A0)));
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[A3][0];
                    }
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    }
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = c.g[A1][0]; // distance
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
                    c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                    if c.g[V1][0] != 0 {
                        c.cycles += 2;
                        c.g[T5][0] = c.g[V0][0];
                        c.g[T3][0] = c.g[T0][0];
                    }
                    c.cycles += 2;
                    let more = c.g[T1][0] != 0;
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_38a0);
                    }
                }
            }
            c.cycles += 1;
            c.g[T0][0] = Addr(r32(c, T3).wrapping_add(2)).lhu(c);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    ret(c);
}

/// The shared body of the rider sub-state `SaveState` forwarders:
/// `vt = *(VTable**)stream; vt[1].fn(stream + vt[1].delta, self, size)` — the
/// state stream's write method (vtable slot 1, at +8) saves `size` bytes of
/// `self`. `stream` (a1) is kept in a3, `self` becomes the second argument.
fn rider_sub_save_state_call(c: &mut Cpu, ret_addr: u32, size: u32) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[A3][0] = c.g[A1][0];
    let stream = c.g[A3][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = c.g[A0][0];
    c.g[A2][0] = sx(size);
    c.g[V1][0] = Addr(stream as u32).lw(c);
    let slot = crate::call::VSlot::read(c, c.g[V1][0], 0x8);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(stream);
    call_ptr(c, slot.func as u32, ret_addr)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderSpinSub_SaveState(RiderSpinSub* self, StateStream* s)`
/// (0x1361f8): `s->Write(self, 0x58)`.
pub fn rider_spin_sub_save_state(c: &mut Cpu) {
    guest(c, |c| rider_sub_save_state_call(c, 0x0013_6220, 0x58))
}

/// `void RiderGateSub_SaveState(RiderGateSub* self, StateStream* s)`
/// (0x12c558): `s->Write(self, 0x14)`.
pub fn rider_gate_sub_save_state(c: &mut Cpu) {
    guest(c, |c| rider_sub_save_state_call(c, 0x0012_c580, 0x14))
}

/// Speech history header bytes (EA speech library, `SPCH`).
mod spch_hist_f {
    /// Flags: low nibble = extra word count, bit 7 = a bitmap follows.
    pub const FLAGS: u32 = 0x4;
    /// Width (bits per bitmap row).
    pub const WIDTH: u32 = 0x5;
    /// History length (ring size).
    pub const COUNT: u32 = 0x6;
}

/// `int SPCH_HistoryFind(SpchHistory* h, int key)` (0x3d6e30): look `key`
/// up in the speech history ring, newest first; returns its age + 1, or 0
/// when it is not there. The ring (a write-position byte, then `count`
/// entries) lies past the header tables, whose size follows from the
/// flags/width bytes. A leaf.
pub fn spch_history_find(c: &mut Cpu) {
    use spch_hist_f::*;
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    let hist = r32(c, A2);
    c.g[T0][0] = Addr(hist.wrapping_add(COUNT)).lbu(c);
    c.g[T3][0] = 0; // result: not found
    if c.g[T0][0] as i64 > 0 {
        c.cycles += 13;
        c.g[A0][0] = Addr(hist.wrapping_add(FLAGS)).lbu(c);
        c.g[T1][0] = 0; // bitmap bytes
        c.g[A3][0] = Addr(hist.wrapping_add(WIDTH)).lbu(c);
        c.g[V1][0] = c.g[A0][0] & 0xf;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(2));
        c.mult(0, V0, A3, V0);
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xf)) & 0x3ffc;
        c.g[V0][0] = sx(hist.wrapping_add(r32(c, V0)));
        c.g[A0][0] &= 0x80;
        c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        if c.g[A0][0] != 0 {
            // A bitmap of (width + 7) / 8 bytes plus one.
            c.cycles += 7;
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(7));
            c.g[V1][0] = ((-1i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V0][0] = if c.g[V1][0] != 0 {
                c.g[A0][0]
            } else {
                sx(r32(c, A3).wrapping_add(0xe))
            };
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
            c.g[T1][0] = sx(r32(c, V0).wrapping_add(1));
        }
        c.cycles += 8;
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, T1)));
        c.g[A3][0] = 0; // age
        c.g[A2][0] = Addr(r32(c, A0)).lbu(c); // write position
        c.g[T1][0] = sx(r32(c, T0).wrapping_sub(1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1)); // entries
        c.g[T2][0] = u64::MAX;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        loop {
            c.cycles += 6;
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T1][0]; // wrap to the last entry
            }
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
            c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
            if c.g[V0][0] == c.g[A1][0] {
                c.cycles += 2;
                c.g[T3][0] = sx(r32(c, A3).wrapping_add(1));
                break;
            }
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_6eb0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T3][0];
    ret(c);
}

/// `MEM_PoolAlloc` (0x319e48).
const MEM_POOL_ALLOC_281: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::MEM_PoolAlloc,
};
/// 128-byte pool: the free-list head (offset from the pool).
const POOL128_FREE_HEAD: u32 = 0x2_5800;

/// `void* MEM_Pool128Alloc(Pool128* pool)` (0x3203b8): pop a 128-byte block
/// off the pool's free list; when the list is empty, fall back to
/// `MEM_PoolAlloc(0x84, align)` (the next size class, or dlmalloc).
pub fn mem_pool128_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let pool = r32(c, A0);
        c.g[V0][0] = sx(pool.wrapping_add(0x2_0000));
        frame.save_ra(c, 0);
        let head = Addr(pool.wrapping_add(POOL128_FREE_HEAD)).lw(c);
        c.g[V1][0] = 0x2_0000;
        if head == 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x84;
            MEM_POOL_ALLOC_281.call(c, 0x0032_03dc)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            let head_at = pool.wrapping_add(POOL128_FREE_HEAD);
            c.g[V1][0] = sx(head_at);
            c.g[V0][0] = Addr(head_at).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            let next = c.g[A0][0];
            Addr(head_at).sw(c, next);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `DLHeap_InitState` (0x31e260).
const DLHEAP_INIT_STATE: Callee = Callee {
    addr: 0x0031_e260,
    f: ee::s10::DLHeap_InitState,
};

/// dlmalloc state (`mstate`) and chunk fields used by
/// [`dlheap_consolidate`].
mod dl_consolidate_f {
    /// `max_fast` (low bits are flags: bit 1 = have fast chunks).
    pub const MAX_FAST: u32 = 0x0;
    /// `fastbins[]`.
    pub const FASTBINS: u32 = 0x4;
    /// `top`.
    pub const TOP: u32 = 0x30;
    /// The unsorted bin (`bins[0]` as a fake chunk).
    pub const UNSORTED: u32 = 0x38;
    /// Chunk: `prev_size`, `size` (bit 0 = previous in use), `fd`, `bk`.
    pub const PREV_SIZE: u32 = 0x0;
    pub const SIZE: u32 = 0x4;
    pub const FD: u32 = 0x8;
    pub const BK: u32 = 0xc;
}

/// Unlink a free chunk from its bin (`FD->bk = BK; BK->fd = FD`), with the
/// original's register use (`v0` = fd, `v1` = bk).
fn dl_consolidate_unlink(c: &mut Cpu, chunk: u32) {
    use dl_consolidate_f::*;
    c.g[V0][0] = Addr(chunk.wrapping_add(FD)).lw(c);
    c.g[V1][0] = Addr(chunk.wrapping_add(BK)).lw(c);
    let (fd, bk) = (c.g[V0][0], c.g[V1][0]);
    Addr((fd as u32).wrapping_add(BK)).sw(c, bk);
    Addr((bk as u32).wrapping_add(FD)).sw(c, fd);
}

/// `void DLHeap_Consolidate(DLHeap* ms, mstate av)` (0x31eee8): dlmalloc's
/// `malloc_consolidate`: with the state initialised (`max_fast != 0`),
/// clear the fast-chunks flag and free every chunk in every fast bin
/// properly: merge it with a free previous / next neighbour and put it in
/// the unsorted bin, or into the top chunk when it borders it. An
/// uninitialised state is set up with `DLHeap_InitState(av)` instead.
pub fn dlheap_consolidate(c: &mut Cpu) {
    use dl_consolidate_f::*;
    let frame = Frame { size: 0x10 };
    guest(c, |c| {
        c.cycles += 4;
        Frame::open(c, 0x10);
        let av = r32(c, A1);
        c.g[V0][0] = Addr(av.wrapping_add(MAX_FAST)).lw(c);
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[A1][0];
            DLHEAP_INIT_STATE.call(c, 0x0031_f00c)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 10;
        c.g[T5][0] = sx(av.wrapping_add(UNSORTED));
        c.g[T4][0] = sx(av.wrapping_add(FASTBINS));
        c.g[V1][0] = c.g[V0][0] & 0xffff_ffff_ffff_fffd; // clear_fastchunks
        let max_fast = c.g[V1][0];
        Addr(av.wrapping_add(MAX_FAST)).sw(c, max_fast);
        c.g[V0][0] = sx(((r32(c, V1) >> 3) << 2).wrapping_sub(4));
        c.g[T7][0] = sx(av.wrapping_add(r32(c, V0))); // last fast bin
        loop {
            // One fast bin (t4).
            c.cycles += 3;
            c.g[A2][0] = Addr(r32(c, T4)).lw(c);
            c.g[T6][0] = sx(r32(c, T4).wrapping_add(4));
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                c.g[T3][0] = 0xffff_ffff_ffff_fffe; // ~PREV_INUSE
                c.g[T2][0] = 0xffff_ffff_ffff_fffc; // ~SIZE_BITS
                Addr(r32(c, T4)).sw(c, 0);
                loop {
                    // One chunk (a2); t1 = the next in the bin.
                    c.cycles += 9;
                    let chunk = r32(c, A2);
                    c.g[V1][0] = Addr(chunk.wrapping_add(SIZE)).lw(c);
                    c.g[T1][0] = Addr(chunk.wrapping_add(FD)).lw(c);
                    c.g[A3][0] = c.g[V1][0] & c.g[T3][0]; // size
                    c.g[A0][0] = sx(chunk.wrapping_add(r32(c, A3))); // next chunk
                    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SIZE)).lw(c);
                    c.g[V1][0] = (c.g[V1][0] ^ 1) & 1;
                    c.g[T0][0] = c.g[V0][0] & c.g[T2][0]; // next size
                    if c.g[V1][0] != 0 {
                        // The previous chunk is free: merge backwards.
                        c.cycles += 7;
                        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(PREV_SIZE)).lw(c);
                        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
                        let prev = r32(c, A2);
                        dl_consolidate_unlink(c, prev);
                    }
                    c.cycles += 3;
                    let top = Addr(av.wrapping_add(TOP)).lw(c);
                    let next = r32(c, A0);
                    c.g[V0][0] = sx(next.wrapping_add(r32(c, T0)));
                    if c.g[A0][0] != top {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SIZE)).lw(c) & 1;
                        let next_size = c.g[T0][0];
                        Addr(next.wrapping_add(SIZE)).sw(c, next_size); // clear inuse
                        if c.g[V1][0] == 0 {
                            // The next chunk is free too: merge forwards.
                            c.cycles += 5;
                            c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, T0)));
                            dl_consolidate_unlink(c, next);
                        }
                        // Into the unsorted bin.
                        c.cycles += 10;
                        let bin = r32(c, T5);
                        let chunk = r32(c, A2);
                        let size = c.g[A3][0];
                        c.g[V0][0] = Addr(bin.wrapping_add(FD)).lw(c);
                        let first = c.g[V0][0];
                        c.g[V1][0] = size | 1;
                        let head = c.g[V1][0];
                        Addr(bin.wrapping_add(FD)).sw(c, chunk as u64);
                        c.g[A0][0] = sx(chunk.wrapping_add(size as u32));
                        Addr((first as u32).wrapping_add(BK)).sw(c, chunk as u64);
                        Addr(chunk.wrapping_add(SIZE)).sw(c, head);
                        Addr(chunk.wrapping_add(FD)).sw(c, first);
                        Addr(chunk.wrapping_add(BK)).sw(c, bin as u64);
                        Addr(r32(c, A0)).sw(c, size); // foot
                    } else {
                        // Borders the top chunk: becomes part of it.
                        c.cycles += 4;
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, T0)));
                        c.g[V0][0] = c.g[A3][0] | 1;
                        let (chunk, head) = (r32(c, A2), c.g[V0][0]);
                        Addr(chunk.wrapping_add(SIZE)).sw(c, head);
                        Addr(av.wrapping_add(TOP)).sw(c, chunk as u64);
                    }
                    c.cycles += 2;
                    c.g[A2][0] = c.g[T1][0];
                    if c.g[T1][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_ef40);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[T4][0];
            let more = c.g[V0][0] != c.g[T7][0];
            c.g[T4][0] = c.g[T6][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_ef20);
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_9b40, rider_sphere_tree_translate, "RiderSphereTree_Translate"),
    (0x0028_8b40, ssx_audio_find_view_in_range, "SSXAudioSystem_FindViewInRange"),
    (0x0035_6db0, object_ctor, "Object_ctor"),
    (0x0032_00c0, mem_heap_free, "MEM_HeapFree"),
    (0x003d_37a0, path_select_branch, "PATH_SelectBranch"),
    (0x0013_61f8, rider_spin_sub_save_state, "RiderSpinSub_SaveState"),
    (0x0012_c558, rider_gate_sub_save_state, "RiderGateSub_SaveState"),
    (0x003d_6e30, spch_history_find, "SPCH_HistoryFind"),
    (0x0032_03b8, mem_pool128_alloc, "MEM_Pool128Alloc"),
    (0x0031_eee8, dlheap_consolidate, "DLHeap_Consolidate"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, put_vec, scramble, Rng, DATA};

    /// A node with 0..6 children (sometimes a negative count) translated by
    /// a random vector.
    #[test]
    fn rider_sphere_tree_translate_matches() {
        check(ee::s10::f_00329b40__generated, rider_sphere_tree_translate, 300, &|c, seed| {
            let mut r = Rng(0x329b_40a5 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0032_9b40;
            let node = DATA;
            let v = DATA + 0x8000;
            c.g[A0][0] = sx(node);
            c.g[A1][0] = sx(v);
            fill(c, &mut r, node, 0x30 / 4 + 6 * 8);
            let count = match seed % 8 {
                0 => 0,
                1 => (r.next_u32() | 0x8000_0000) as i32,
                k => (k - 2) as i32,
            };
            c.bus.write32(node + sphere_tree_f::COUNT, count as u32);
            put_vec(c, node + sphere_tree_f::CENTER, [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.edge_f32()]);
            for i in 0..6 {
                let at = node + sphere_tree_f::CHILDREN + i * sphere_tree_f::CHILD_SIZE;
                put_vec(c, at, [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.edge_f32()]);
            }
            put_vec(c, v, [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.edge_f32()]);
        });
    }

    /// The audio world (gp-0x848) with an app whose view list holds 0..3
    /// views (or no list: one listener, view 0 read from low RAM) and a
    /// listener position near or far from them.
    #[test]
    fn ssx_audio_find_view_in_range_matches() {
        use crate::difftest::GAME_GP;
        check(ee::s07::f_00288b40__generated, ssx_audio_find_view_in_range, 300, &|c, seed| {
            let mut r = Rng(0x288b_40c3 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0028_8b40;
            c.g[GP][0] = sx(GAME_GP);
            let world = DATA;
            let app = DATA + 0x100;
            let list = DATA + 0x200;
            let pos = DATA + 0x400;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, app);
            let has_list = seed % 7 != 0;
            c.bus.write32(app + 0x84, if has_list { list } else { 0 });
            let n = seed % 4;
            c.bus.write32(list + 0x10, n);
            let spread = if seed % 3 == 0 { 1.0 } else { 20.0 };
            for i in 0..3 {
                let view = DATA + 0x1000 + i * 0x40;
                c.bus.write32(list + 4 + i * 4, view);
                put_vec(c, view + 0x20, [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)]);
            }
            put_vec(c, pos, [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)]);
            c.g[A0][0] = sx(DATA + 0x800);
            c.g[A1][0] = sx(pos);
            c.f[12] = if seed % 11 == 0 { r.edge_f32() } else { r.f32(0.2) & 0x7fff_ffff };
        });
    }

    /// `Object_ctor` on the real ModObj/WorldObj constructor chain (fixture
    /// as in `world_obj_ctor_matches`): node flags with bits 0-1 both set
    /// or not, a3/a4 zero or not.
    #[test]
    fn object_ctor_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(ee::s11::f_00356db0__generated, object_ctor, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_6db0, 0);
            c.pc = 0x0035_6db0;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            fill(c, &mut r, SELF, 0x10);
            fill(c, &mut r, NODE, 0x10);
            let mut flags = r.next_u32();
            if seed % 4 == 0 {
                flags |= 0x100;
            }
            if seed % 3 == 0 {
                flags |= 3;
            }
            w(c, NODE + object_f::NODE_FLAGS, flags);
            let list = seed % 4;
            w(c, GAME_GP + 0x2898 + 4, PROCS);
            for i in 0..4 {
                w(c, PROCS + 0x44 * i, r.next_u32());
                w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
            }
            fill(c, &mut r, 0x0044_aff0, 0x20);
            // World -> game -> race -> focus manager.
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD + 0x100);
            w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
            w(c, WORLD + 0x200 + 0xa4, FOCUS);
            w(c, FOCUS + 0xd4, if seed % 5 == 0 { 0 } else { r.next_u32() });
            w(c, FOCUS + 0xd8, r.next_u32() % 8);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(NODE);
            c.g[A3][0] = if seed % 2 == 0 { 0 } else { sx(r.next_u32() | 1) };
            c.g[T0][0] = if (seed / 2) % 2 == 0 { 0 } else { sx(r.next_u32() | 1) };
        });
    }

    /// Run a guest function from a test setup until it returns.
    fn oc281_run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `MEM_HeapFree` on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, a few mallocs, some freed first): null, small and
    /// large chunks, as in `dlheap_free_matches`.
    #[test]
    fn mem_heap_free_matches() {
        use crate::difftest::fakes::machine;
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        check(ee::s10::f_003200c0__generated, mem_heap_free, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0032_00c0, 0);
            let ra = c.g[RA][0];
            oc281_run(c, 0x0031_ff60, ee::s10::DLHeap_InitArena, &[HEAP as u64, ARENA as u64, 0x4_0000]);
            let n = 1 + r.next_u32() % 6;
            let mut ptrs = Vec::new();
            for _ in 0..n {
                let size = match r.next_u32() % 3 {
                    0 => r.next_u32() % 0x40,
                    1 => 0x40 + r.next_u32() % 0x800,
                    _ => 0x1000 + r.next_u32() % 0x3000,
                };
                ptrs.push(oc281_run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, size as u64]));
            }
            let target = (r.next_u32() % n) as usize;
            for (i, &p) in ptrs.iter().enumerate() {
                if i != target && p != 0 && r.next_u32() % 3 == 0 {
                    oc281_run(c, 0x0031_ed60, ee::s10::DLHeap_Free, &[HEAP as u64, p]);
                }
            }
            c.g[RA][0] = ra;
            c.pc = 0x0032_00c0;
            c.g[A0][0] = sx(HEAP);
            c.g[A1][0] = if seed % 10 == 0 { 0 } else { ptrs[target] };
        });
    }

    /// `PATH_SelectBranch`: a network (at 0x44f420) whose node `a0` has
    /// 0..9 branches (sometimes 31: past the written ones, zeros) with small
    /// lane ranges, a tracker queued on that node or elsewhere, positive or
    /// non-positive node counts (lane consumed, tracker released at -1), and
    /// lanes inside or outside every range (nearest-branch search).
    #[test]
    fn path_select_branch_matches() {
        use path_f::*;
        const NET: u32 = DATA;
        const IDX: u32 = DATA + 0x800;
        const BASE: u32 = DATA + 0x1000;
        const TRACKER: u32 = DATA + 0x2000;
        check(ee::s14::f_003d37a0__generated, path_select_branch, 300, &|c, seed| {
            let mut r = Rng(0x3d37_a0b1 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x003d_37a0;
            c.bus.write32(NETWORK, NET);
            c.bus.write32(NET + NODE_BASE, BASE);
            c.bus.write32(NET + NODE_INDEX, IDX);
            for k in 0..32 {
                c.bus.write32(NET + TRACKERS + 4 * k, TRACKER);
            }
            let node = r.next_u32() % 16;
            let word = 4 + r.next_u32() % 32;
            c.bus.write16(IDX + 2 * node, word as u16);
            let rec = BASE + 4 * word;
            let count = match seed % 4 {
                0 => 0,
                1 => 8 + r.next_u32() % 8, // negative count
                _ => 1 + r.next_u32() % 7,
            };
            let slot = r.next_u32() % 32;
            c.bus.write32(rec, count << 28 | slot << 16 | (r.next_u32() & 0xffff));
            let branches = match seed % 13 {
                0 => 0,
                1 => 31,
                _ => 1 + r.next_u32() % 9,
            };
            c.bus.write32(rec + NODE_WORD1, (r.next_u32() & !0x1f000) | branches << 12);
            for i in 0..9 {
                let lo = (r.next_u32() % 21) as i32 - 10;
                let hi = lo + (r.next_u32() % 6) as i32 - 1;
                let e = rec + BRANCHES + 4 * i;
                c.bus.write8(e, lo as u8);
                c.bus.write8(e + 1, hi as u8);
                c.bus.write16(e + 2, r.next_u32() as u16);
            }
            let on_node = seed % 3 != 0;
            c.bus.write16(TRACKER + TRACKER_NODE, if on_node { node as u16 } else { (node + 1) as u16 });
            let lane_word = if seed % 5 == 0 {
                r.next_u32() & !0xf // lane 0: released after the decrement
            } else {
                r.next_u32()
            };
            c.bus.write32(TRACKER + TRACKER_LANE, lane_word);
            c.g[A0][0] = node as u64;
            c.g[A1][0] = if seed % 7 == 0 {
                sx(r.next_u32())
            } else {
                sx(((r.next_u32() % 25) as i32 - 12) as u32)
            };
        });
    }

    /// A rider sub-state `SaveState` forwarder: `self` at DATA+0x400, the
    /// stream at DATA+0x800 whose vtable slot 1 is a fake (which unwinds for
    /// some seeds).
    fn rider_sub_save_state_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes;
        let mut r = fakes::machine(c, seed, salt, 1);
        c.pc = entry;
        let stream = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(stream);
        let vt = fakes::vtable(c, &mut r, &[0x8]);
        c.bus.write32(stream, vt);
    }

    #[test]
    fn rider_spin_sub_save_state_matches() {
        crate::difftest::fakes::covered(ee::s01::f_001361f8__generated, rider_spin_sub_save_state, 200, &|c, seed| {
            rider_sub_save_state_setup(c, seed, 0x1361_f8, 0x0013_61f8);
        });
    }

    #[test]
    fn rider_gate_sub_save_state_matches() {
        crate::difftest::fakes::covered(ee::s00::f_0012c558__generated, rider_gate_sub_save_state, 200, &|c, seed| {
            rider_sub_save_state_setup(c, seed, 0x12c5_58, 0x0012_c558);
        });
    }

    /// `SPCH_HistoryFind`: empty or 1..12-entry histories with random
    /// flags/width (bitmap or not), write positions inside the ring (and 0:
    /// wrap at once), keys present (some several times) or absent.
    #[test]
    fn spch_history_find_matches() {
        use crate::difftest::check_lohi;
        use spch_hist_f::*;
        check_lohi(ee::s14::f_003d6e30__generated, spch_history_find, 300, &|c, seed| {
            let mut r = Rng(0x3d6e_30c7 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x003d_6e30;
            let hist = DATA;
            fill(c, &mut r, hist, 0x40);
            let count = if seed % 9 == 0 { 0 } else { 1 + r.next_u32() % 12 };
            c.bus.write8(hist + COUNT, count as u8);
            c.bus.write8(hist + FLAGS, r.next_u32() as u8);
            c.bus.write8(hist + WIDTH, r.next_u32() as u8);
            // The ring and everything after the header: small values so keys hit.
            for i in 8..0x400 {
                c.bus.write8(hist + i, (r.next_u32() % 8) as u8);
            }
            // Write positions: mostly within 0..=count (some hit the wrap).
            let flags = c.bus.read8(hist + FLAGS) as u32;
            let width = c.bus.read8(hist + WIDTH) as u32;
            let t = flags & 0xf;
            let mut ring = hist + ((width * (t + 2) + 0xf) & 0x3ffc) + t * 4;
            if flags & 0x80 != 0 {
                ring += (width + 7) / 8 + 1;
            }
            c.bus.write8(ring, (r.next_u32() % (count + 1)) as u8);
            c.g[A0][0] = sx(hist);
            c.g[A1][0] = if seed % 11 == 0 { sx(r.next_u32()) } else { (r.next_u32() % 9) as u64 };
        });
    }

    /// `MEM_Pool128Alloc`: a non-empty free list (pop), or an empty one with
    /// the real `MEM_PoolAlloc` behind it: pools ready (the 256-byte pool
    /// serves it from its free list) or not yet (dlmalloc on an arena the
    /// setup initialises).
    #[test]
    fn mem_pool128_alloc_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        check(ee::s10::f_003203b8__generated, mem_pool128_alloc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_03b8, 0);
            let pool = DATA + 0x1000;
            let blocks = DATA + 0x3_0000;
            match seed % 3 {
                0 => {
                    // Free list of a few 0x80-byte blocks.
                    let n = 1 + r.next_u32() % 4;
                    for i in 0..n {
                        let node = blocks + 0x80 * i;
                        c.bus.write32(node, if i + 1 == n { 0 } else { node + 0x80 });
                    }
                    c.bus.write32(pool + POOL128_FREE_HEAD, blocks);
                }
                1 => {
                    // Empty: MEM_PoolAlloc takes the ready 256-byte pool.
                    c.bus.write32(pool + POOL128_FREE_HEAD, 0);
                    c.bus.write32(GAME_GP + 0xdd8, 1);
                    let p256 = DATA + 0x4_0000;
                    let base = DATA + 0x6_8000;
                    for n in 0..8u32 {
                        let node = base + 0x100 * n;
                        c.bus.write32(node, if n == 7 { 0 } else { node + 0x100 });
                    }
                    c.bus.write32(p256 + 0x2_0000, base);
                    c.bus.write32(GAME_GP + 0x2a78, p256);
                }
                _ => {
                    // Empty, pools not ready: the dlmalloc heap.
                    let ra = c.g[RA][0];
                    oc281_run(c, 0x0031_ff60, ee::s10::DLHeap_InitArena, &[0x004f_f250, (DATA + 0x8_0000) as u64, 0x4_0000]);
                    c.g[RA][0] = ra;
                    c.bus.write32(pool + POOL128_FREE_HEAD, 0);
                    c.bus.write32(GAME_GP + 0xdd8, 0);
                }
            }
            c.pc = 0x0032_03b8;
            c.g[A0][0] = sx(pool);
            c.g[A1][0] = [0u64, 4, 8, 16, 64][(r.next_u32() % 5) as usize];
        });
    }

    /// `DLHeap_Consolidate` on a real dlmalloc heap (`DLHeap_InitArena` on
    /// a 256 KiB arena): 2..12 mallocs of fast and larger sizes, some fast
    /// ones freed (into the fast bins), some larger ones freed (free
    /// neighbours: backward / forward merges), the last one bordering top
    /// (merge into top); and an uninitialised state (`DLHeap_InitState`).
    #[test]
    fn dlheap_consolidate_matches() {
        use crate::difftest::fakes::machine;
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        check(ee::s10::f_0031eee8__generated, dlheap_consolidate, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_eee8, 0);
            let ra = c.g[RA][0];
            oc281_run(c, 0x0031_ff60, ee::s10::DLHeap_InitArena, &[HEAP as u64, ARENA as u64, 0x4_0000]);
            let n = 2 + r.next_u32() % 11;
            let mut ptrs = Vec::new();
            for _ in 0..n {
                let fast = r.next_u32() % 3 != 0;
                let size = if fast { r.next_u32() % 0x39 } else { 0x48 + r.next_u32() % 0x200 };
                let p = oc281_run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, size as u64]);
                ptrs.push((p, fast));
            }
            for &(p, fast) in ptrs.iter() {
                let free = if fast { r.next_u32() % 4 != 0 } else { r.next_u32() % 2 == 0 };
                if p != 0 && free {
                    oc281_run(c, 0x0031_ed60, ee::s10::DLHeap_Free, &[HEAP as u64, p]);
                }
            }
            let av = c.bus.read32(HEAP);
            if seed % 10 == 0 {
                c.bus.write32(av, 0);
            }
            c.g[RA][0] = ra;
            c.pc = 0x0031_eee8;
            c.g[A0][0] = sx(HEAP);
            c.g[A1][0] = sx(av);
        });
    }
}
