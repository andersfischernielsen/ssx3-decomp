//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the save/serialise helpers, the flag and spline
//! modifiers, the pad/input state machine and the streaming/lighting
//! helpers, each with a differential test against the generated body.
//!
//! - **Save walkers** (`TypedProcess_Save`, `SyncProcess_Save`,
//!   `WorldObj_Save`, `AnimObject_v52`): hand a sub-object and a byte count
//!   to the stream object's virtual save slot, then the next chunk.
//! - **`ReplayStream_WriteNodeRef`**: append an 8-byte node reference record.
//! - **`LunoVMRegister_dtor`**: release a Luno table reference, optionally
//!   freeing the value.
//! - **`cFlagManager_GroupUpdate`**: advance the four phase animators,
//!   rebuild the group's vertices when the frame parity matches, then scroll
//!   the texture offsets.
//! - **`IntersectInfo_EvaluateMatrix`**: evaluation pass, then the rotation
//!   matrix from `-rot * pi/180` and the translation.
//! - **`InputPad_Update`**: per-button debounce, press/release edges and
//!   auto-repeat timers.
//! - **`WorldLightMan_FindStrongestLights`**: insertion sort of the strongest
//!   light contributions.
//! - **`MultiSplineModifier_v24` / `_v11`**: rebuild if dirty, then refresh
//!   the element bounds / the model's boxes.
//! - **`SprStream_SetMode`**: the light manager's buffer mode switch.
//!
//! The bodies mirror the original's register allocation (the differential
//! test compares `v0`/`v1` as hard state), so loop state lives in the guest
//! registers the generated code uses, and every VU0/FPU operation is issued
//! in the original order.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_ed28,
        replay_stream_write_node_ref,
        "ReplayStream_WriteNodeRef",
    ),
    (0x0035_4948, typed_process_save, "TypedProcess_Save"),
    (0x0035_4980, sync_process_save, "SyncProcess_Save"),
    (0x0034_fe90, world_obj_save, "WorldObj_Save"),
    (0x0036_1b90, spline_modifier_v18, "SplineModifier_v18"),
    (0x0022_5b90, luno_vm_register_dtor, "LunoVMRegister_dtor"),
    (
        0x0034_b818,
        cflag_manager_group_update,
        "cFlagManager_GroupUpdate",
    ),
    (
        0x0035_1800,
        intersect_info_evaluate_matrix,
        "IntersectInfo_EvaluateMatrix",
    ),
    (0x0034_e348, anim_object_v52, "AnimObject_v52"),
    (
        0x002f_5b68,
        world_light_man_find_strongest_lights,
        "WorldLightMan_FindStrongestLights",
    ),
    (
        0x0035_a918,
        multi_spline_modifier_v24,
        "MultiSplineModifier_v24",
    ),
    (0x0032_1298, input_pad_update, "InputPad_Update"),
    (
        0x0035_b418,
        multi_spline_modifier_v11,
        "MultiSplineModifier_v11",
    ),
    (0x0038_f300, spr_stream_set_mode, "SprStream_SetMode"),
];

// ---------------------------------------------------------------------------
// Callees and field offsets.

mod callee {
    use super::{ee, Callee};

    pub const LUNO_TABLE_DTOR: Callee = Callee {
        addr: 0x0022_4df0,
        f: ee::s05::f_00224df0,
    };
    pub const WORLD_GET_FLAG_MANAGER: Callee = Callee {
        addr: 0x002d_1cb0,
        f: ee::s09::World_GetFlagManager,
    };
    pub const WORLD_GET_AI_FRAME_COUNT: Callee = Callee {
        addr: 0x002d_1c98,
        f: ee::s09::World_GetAIFrameCount,
    };
    pub const WORLD_GET_NASH: Callee = Callee {
        addr: 0x002d_1be0,
        f: ee::s09::World_GetNash,
    };
    pub const LIGHT_CONTRIBUTION: Callee = Callee {
        addr: 0x002f_5d30,
        f: ee::s09::WorldLightMan_LightContribution,
    };
    pub const MATH_SIN_COS: Callee = Callee {
        addr: 0x0031_be50,
        f: ee::s10::Math_SinCos,
    };
    pub const SPATIAL_HASH_UPDATE: Callee = Callee {
        addr: 0x0032_91e0,
        f: ee::s10::SpatialHash_Update,
    };
    pub const FLAG_MANAGER_GROUP_COMPUTE_VERTS: Callee = Callee {
        addr: 0x0034_bca0,
        f: ee::s11::cFlagManager_GroupComputeVerts,
    };
    pub const INTERSECT_INFO_EVALUATE: Callee = Callee {
        addr: 0x0035_1538,
        f: ee::s11::IntersectInfo_Evaluate,
    };
    pub const TYPED_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4948,
        f: ee::s11::TypedProcess_Save,
    };
    pub const SYNC_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4980,
        f: ee::s11::SyncProcess_Save,
    };
    pub const MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH: Callee = Callee {
        addr: 0x0034_5430,
        f: ee::s11::MultiSplineModifier_DebugDrawPath,
    };
    pub const EA_MEMSET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };
    pub const DMA_WAIT_FROM_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f768,
        f: ee::s12::Dma_WaitFromScratchpad,
    };
    pub const DMA_WAIT_TO_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f7b0,
        f: ee::s12::Dma_WaitToScratchpad,
    };
}

/// `cFlagManager` fields.
mod flag_manager {
    /// Frame parity the group was last rebuilt for (`+0x90`).
    pub const PARITY: u32 = 0x90;
    /// Vertex animation blend (`+0x4c`).
    pub const BLEND: u32 = 0x4c;
    /// Per-phase amount (`+0x4`, 4 floats) and accumulator (`+0x78`).
    pub const PHASE: u32 = 0x4;
    pub const SPEED: u32 = 0x78;
    /// Scroll speeds (`+0x50` v, `+0x54` u) and offsets (`+0x88` v, `+0x8c` u).
    pub const SCROLL_V: u32 = 0x50;
    pub const SCROLL_U: u32 = 0x54;
    pub const V: u32 = 0x88;
    pub const U: u32 = 0x8c;
}

/// `sInputPad` button array starts at `+4`, stride 0x1c.
mod pad {
    pub const COUNT: u32 = 0x0;
    pub const BUTTONS: u32 = 0x4;
    pub const BUTTON_SIZE: u32 = 0x1c;
}

/// `MultiSplineModifier` fields: the element array (`+0x3c`, stride 0x60) and
/// instance pointer array (`+0x44`) are parallel; the box is `{min, max}` at
/// `+0x30`/`+0x40`.
mod spline {
    pub const COUNT: u32 = 0x4;
    pub const DIRTY: u32 = 0x30;
    pub const BOXES: u32 = 0x3c;
    pub const INSTS: u32 = 0x44;
    pub const MODEL: u32 = 0x40;
    pub const MODE: u32 = 0x38;
    pub const FOLLOW: u32 = 0x1c;
    pub const RADIUS: u32 = 0x18;
    pub const ELEM_SIZE: u32 = 0x60;
}

/// `WorldLightMan` fields.
mod lights {
    pub const COUNT: u32 = 0x210;
    pub const ARRAY: u32 = 0x214;
    /// `sLight`: type at +8, intensity at +0x18.
    pub const TYPE: u32 = 0x8;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`, 32-bit wrap.
#[inline(always)]
fn off(a: u32, o: u32) -> u32 {
    a.wrapping_add(o)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, o: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(o as u32)
}

/// Loads and stores as methods: a method's receiver borrow is two-phased, so
/// an argument may read `c.g` as the original's address arithmetic does.
trait Oc49Mem {
    fn ldw(&mut self, a: u32) -> u64;
    fn ldh(&mut self, a: u32) -> u64;
    fn ldhu(&mut self, a: u32) -> u64;
    fn stw(&mut self, a: u32, v: u64);
    fn sth(&mut self, a: u32, v: u64);
}

impl Oc49Mem for Cpu {
    #[inline(always)]
    fn ldw(&mut self, a: u32) -> u64 {
        Addr(a).lw(self)
    }
    #[inline(always)]
    fn ldh(&mut self, a: u32) -> u64 {
        Addr(a).lh(self)
    }
    #[inline(always)]
    fn ldhu(&mut self, a: u32) -> u64 {
        Addr(a).lhu(self)
    }
    #[inline(always)]
    fn stw(&mut self, a: u32, v: u64) {
        Addr(a).sw(self, v);
    }
    #[inline(always)]
    fn sth(&mut self, a: u32, v: u64) {
        Addr(a).sh(self, v);
    }
}

/// `addu`: 32-bit add, sign-extended into the 64-bit register.
#[inline(always)]
fn addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------

/// `void ReplayStream_WriteNodeRef(ReplayStream *out, NodeRef *node)`
/// (0x0026ed28): append `{ u32 id; u16 type; }` (id = `node->+0x18->+0x78`,
/// type = `+0x18->+8`; `-1`/0 when the node has no reference) through
/// `out->vt[1](out + delta, &rec, 8)`.
pub fn replay_stream_write_node_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0x10);
        let rec = frame.at(c, 0);
        c.stw(rec, c.g[V0][0]);
        let out = abi::arg(c, 0);
        let node = abi::arg(c, 1);
        c.g[A3][0] = sx(out);
        c.g[V1][0] = c.ldw(off(node, 0x18));
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.sth(off(rec, 4), 0);
        } else {
            c.cycles += 5;
            let p = c.g[V1][0] as u32;
            c.g[V0][0] = c.ldw(off(p, 0x78));
            c.stw(rec, c.g[V0][0]);
            c.g[V1][0] = c.ldhu(off(p, 8));
            c.sth(off(rec, 4), c.g[V1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.ldw(out);
        c.g[A1][0] = sx(frame.at(c, 0));
        c.g[A2][0] = 8;
        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 8));
        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = addu(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x0026_ed7c)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void TypedProcess_Save(TypedProcess *self, void *obj)` (0x00354948):
/// `obj->vt[1](obj + delta, self + 0x10, 4)`.
pub fn typed_process_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let this = abi::arg(c, 0);
        let obj = abi::arg(c, 1);
        c.g[A3][0] = sx(obj);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(off(this, 0x10));
        c.g[A2][0] = 4;
        let vt = c.ldw(obj);
        c.g[V1][0] = vt;
        c.g[A0][0] = c.ldh(off(vt as u32, 8));
        c.g[V0][0] = c.ldw(off(vt as u32, 0xc));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = addu(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_4970)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SyncProcess_Save(SyncProcess *self, void *obj)` (0x00354980):
/// `TypedProcess_Save(self, obj)`, then `obj->vt[1](obj + delta,
/// self + 0x14, 4)`.
pub fn sync_process_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        let obj = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(obj);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::TYPED_PROCESS_SAVE.call(c, 0x0035_499c)?;
        c.g[V1][0] = c.ldw(c.g[S0][0] as u32);
        c.g[A1][0] = sx(off(c.g[S1][0] as u32, 0x14));
        c.g[A2][0] = 4;
        c.g[A0][0] = c.ldh(off(c.g[V1][0] as u32, 8));
        c.g[V0][0] = c.ldw(off(c.g[V1][0] as u32, 0xc));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_49b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldObj_Save(WorldObj *self, void *stream)` (0x0034fe90):
/// `SyncProcess_Save(self, stream)`, then `stream->vt[6](stream + delta,
/// self)`.
pub fn world_obj_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        let stream = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(stream);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SYNC_PROCESS_SAVE.call(c, 0x0034_feac)?;
        c.g[V0][0] = c.ldw(c.g[S0][0] as u32);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x30));
        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x34));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0034_fec4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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
            c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
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

/// `void LunoVMRegister_dtor(cLunoValue *self, int flags)` (0x00225b90):
/// release a table reference when the value's type is 4, clear the type, and
/// free the value when `flags & 1`.
pub fn luno_vm_register_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.ldw(off(c.g[S0][0] as u32, 8));
        let is_table = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if is_table {
            c.cycles += 2;
            c.g[A1][0] = 2;
            callee::LUNO_TABLE_DTOR.call(c, 0x0022_5bbc)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let no_free = c.g[V0][0] == 0;
        c.stw(off(c.g[S0][0] as u32, 8), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_5bd0)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlagManager_GroupUpdate(cFlagManager *self)` (0x0034b818): return
/// when the manager is not set up; otherwise advance the four phase
/// animators by `blend + wind*(1-blend)`, rebuild the group's vertices when
/// the frame parity matches, and scroll `u`/`v`.
pub fn cflag_manager_group_update(c: &mut Cpu) {
    use flag_manager::*;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x280);
        c.g[V0][0] = c.ldw(gp(c, 0x1150));
        frame.save(c, S0, 0x270);
        frame.save_ra(c, 0x260);
        let no_manager = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_lq = true;
        'body: {
            if no_manager {
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x5c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x270);
                tail_lq = false;
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x60));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x270);
                tail_lq = false;
                break 'body;
            }
            c.cycles += 14;
            callee::WORLD_GET_FLAG_MANAGER.call(c, 0x0034_b850)?;
            // t = blend + wind->rate * (1 - blend); advance the four phases.
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.g[A0][0] = sx(off(c.g[S0][0] as u32, PHASE));
            c.lwc1(0, off(c.g[S0][0] as u32, BLEND));
            c.g[V1][0] = sx(off(c.g[S0][0] as u32, SPEED));
            c.mov_s(3, 1);
            c.lwc1(2, off(c.g[V0][0] as u32, 0x10));
            c.sub_s(1, 1, 0);
            c.g[V0][0] = 3;
            c.mul_s(2, 2, 1);
            c.add_s(2, 0, 2);
            loop {
                c.cycles += 8;
                c.lwc1(0, c.g[A0][0] as u32);
                c.lwc1(1, c.g[V1][0] as u32);
                c.mul_s(0, 0, 2);
                c.add_s(0, 1, 0);
                c.c_le_s(3, 0);
                let no_wrap = !c.fcc();
                c.swc1(0, c.g[V1][0] as u32);
                if !no_wrap {
                    c.cycles += 2;
                    c.sub_s(0, 0, 3);
                    c.swc1(0, c.g[V1][0] as u32);
                }
                c.cycles += 4;
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let more = (c.g[V0][0] as i64) >= 0;
                c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_b880);
                }
            }
            // Frame parity: rebuild the group's vertices when it matches.
            c.cycles += 10;
            callee::WORLD_GET_AI_FRAME_COUNT.call(c, 0x0034_b8c0)?;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[A0][0] = c.ldw(off(c.g[S0][0] as u32, PARITY));
            c.g[V1][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if c.g[V0][0] != c.g[A0][0] {
                c.lwc1(1, off(c.g[S0][0] as u32, SCROLL_V));
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x31;
                c.g[V1][0] = u64::MAX;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    let again = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[S0][0];
                    if !again {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_b8e8);
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = c.g[SP][0];
                callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS.call(c, 0x0034_b90c)?;
                c.g[A2][0] = c.ldw(off(c.g[S0][0] as u32, 0x60));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
                c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
                c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, target, 0x0034_b928)?;
                c.lwc1(1, off(c.g[S0][0] as u32, SCROLL_V));
            }
            // Scroll the texture offsets.
            c.cycles += 5;
            c.lwc1(2, gp(c, -0x2b00));
            c.c_lt_s(2, 1);
            if c.fcc() {
                c.lwc1(0, off(c.g[S0][0] as u32, V));
            } else {
                c.cycles += 5;
                c.lwc1(0, off(c.g[S0][0] as u32, SCROLL_U));
                c.c_lt_s(2, 0);
                if !c.fcc() {
                    frame.restore(c, S0, 0x270);
                    tail_lq = false;
                    break 'body;
                }
                c.cycles += 1;
                c.lwc1(0, off(c.g[S0][0] as u32, V));
            }
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            let no_wrap = !c.fcc();
            c.swc1(0, off(c.g[S0][0] as u32, V));
            if !no_wrap {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, off(c.g[S0][0] as u32, V));
            }
            c.cycles += 7;
            c.lwc1(1, off(c.g[S0][0] as u32, U));
            c.lwc1(0, off(c.g[S0][0] as u32, SCROLL_U));
            c.add_s(0, 1, 0);
            c.c_lt_s(2, 0);
            let no_wrap = !c.fcc();
            c.swc1(0, off(c.g[S0][0] as u32, U));
            if !no_wrap {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, off(c.g[S0][0] as u32, U));
            }
        }
        if tail_lq {
            c.cycles += 1;
            frame.restore(c, S0, 0x270);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x260);
        frame.close(c);
        Ok(())
    })
}

/// `void IntersectInfo_EvaluateMatrix(IntersectInfo *self)` (0x00351800):
/// run the evaluation pass, then build the rotation matrix from
/// `-rot * pi/180` (three `Math_SinCos` calls) and the translation.
pub fn intersect_info_evaluate_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 81;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        callee::INTERSECT_INFO_EVALUATE.call(c, 0x0035_1814)?;
        // r[k] = -self->rot[k] * deg2rad, at sp+0..8.
        let selfp = c.g[S0][0] as u32;
        c.lwc1(2, off(selfp, 0x50));
        c.g[A0][0] = sx(off(c.g[SP][0] as u32, 0x10));
        c.lwc1(3, off(selfp, 0x54));
        c.g[A1][0] = sx(off(c.g[SP][0] as u32, 0x14));
        c.lwc1(1, off(selfp, 0x58));
        c.neg_s(2, 2);
        c.neg_s(3, 3);
        c.lwc1(0, gp(c, -0x2a90));
        c.neg_s(1, 1);
        c.mul_s(2, 2, 0);
        c.mul_s(3, 3, 0);
        c.mul_s(1, 1, 0);
        c.mov_s(12, 2);
        c.swc1(2, c.g[SP][0] as u32);
        c.swc1(3, off(c.g[SP][0] as u32, 4));
        c.swc1(1, off(c.g[SP][0] as u32, 8));
        let sp = c.g[SP][0] as u32;
        for k in 0..3u32 {
            if k > 0 {
                c.lwc1(12, off(sp, 4 * k));
                c.g[A0][0] = sx(off(sp, 0x10 + 8 * k));
                c.g[A1][0] = sx(off(sp, 0x14 + 8 * k));
            }
            let ret = 0x0035_1858 + 0x10 * k;
            callee::MATH_SIN_COS.call(c, ret)?;
        }
        // The matrix, from sc[0] = (sx, cx), sc[1] = (sy, cy), sc[2] = (sz, cz).
        let s = c.g[SP][0] as u32;
        let selfp = c.g[S0][0] as u32;
        c.lwc1(0, off(s, 0x10)); // sx
        c.lwc1(4, off(s, 0x18)); // sy
        c.lwc1(7, off(s, 0x14)); // cx
        c.neg_s(12, 0);
        c.lwc1(10, off(s, 0x20)); // sz
        c.mul_s(8, 0, 4); // sxsy
        c.lwc1(9, off(s, 0x24)); // cz
        c.mul_s(2, 7, 4); // cxsy
        c.mul_s(17, 7, 10); // cxsz
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lwc1(3, off(s, 0x1c)); // cy
        c.mul_s(15, 0, 9); // sx*cz
        c.mul_s(16, 8, 10); // sxsy*sz
        c.swc1(1, off(selfp, 0xcc));
        c.mul_s(1, 7, 9); // cx*cz
        c.lwc1(11, off(selfp, 0x44));
        c.mul_s(5, 17, 4); // cxsz*sy
        c.lwc1(13, off(selfp, 0x48));
        c.mul_s(0, 0, 10); // sx*sz
        c.lwc1(14, off(selfp, 0x4c));
        c.mul_s(2, 2, 9); // cxsy*cz
        c.swc1(11, off(selfp, 0xc0));
        c.neg_s(6, 3);
        c.swc1(13, off(selfp, 0xc4));
        c.mul_s(8, 8, 9); // sxsy*cz
        c.swc1(14, off(selfp, 0xc8));
        c.mul_s(7, 7, 3); // cx*cy
        c.swc1(4, off(selfp, 0x98));
        c.mul_s(12, 12, 3); // -sx*cy
        c.stw(off(selfp, 0x9c), 0);
        c.sub_s(0, 0, 2);
        c.stw(off(selfp, 0xac), 0);
        c.add_s(5, 5, 15);
        c.swc1(7, off(selfp, 0xb8));
        c.sub_s(1, 1, 16);
        c.swc1(12, off(selfp, 0xa8));
        c.mul_s(6, 6, 10); // -cy*sz
        c.swc1(0, off(selfp, 0xb0));
        c.add_s(8, 8, 17);
        c.swc1(5, off(selfp, 0xb4));
        c.mul_s(3, 3, 9); // cy*cz
        c.swc1(1, off(selfp, 0xa4));
        c.swc1(6, off(selfp, 0x94));
        c.swc1(8, off(selfp, 0xa0));
        c.swc1(3, off(selfp, 0x90));
        c.stw(off(selfp, 0xbc), 0);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimObject_v52(AnimObject *self)` (0x0034e348): move every flag
/// element's vertices to `self->time` (virtual slot 2 per element), then
/// mark the object dirty.
pub fn anim_object_v52(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
        let has_items = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if has_items {
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x40));
            let no_items = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !no_items {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.g[A1][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
                loop {
                    c.cycles += 13;
                    c.g[S1][0] = sx(off(c.g[S1][0] as u32, 1));
                    c.lwc1(12, off(c.g[S0][0] as u32, 4));
                    c.g[A1][0] = addu(c.g[S2][0], c.g[A1][0]);
                    c.g[V0][0] = c.ldw(off(c.g[A1][0] as u32, 0x84));
                    c.g[S2][0] = sx(off(c.g[S2][0] as u32, 0xd0));
                    c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x10));
                    c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x14));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = addu(c.g[A1][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0034_e3a4)?;
                    c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x40));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_e380);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x20);
        c.stw(off(c.g[S0][0] as u32, 0x10), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int WorldLightMan_FindStrongestLights(a, b, sLightMgr *mgr,
/// sLight **out, int max)` (0x002f5b68): clear the output, then insert each
/// type-6 light's contribution into the descending top-`max` list.
pub fn world_light_man_find_strongest_lights(c: &mut Cpu) {
    use lights::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x110);
        frame.save(c, S2, 0xf0);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0xe0);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S6, 0xb0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S7, 0xa0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, FP, 0x90);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S1, 0x100);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S4, 0xd0);
        frame.save(c, S5, 0xc0);
        frame.save_ra(c, 0x80);
        let no_out = (c.g[S2][0] as i64) <= 0;
        c.swc1(20, off(c.g[SP][0] as u32, 0x120));
        if !no_out {
            c.cycles += 3;
            c.g[A0][0] = c.g[SP][0];
            c.g[V1][0] = c.g[S3][0];
            c.g[V0][0] = c.g[S2][0];
            loop {
                c.cycles += 7;
                c.stw(c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.stw(c.g[A0][0] as u32, 0);
                c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5bc0);
                }
            }
        }
        c.cycles += 4;
        c.g[S5][0] = c.ldw(off(c.g[S6][0] as u32, COUNT));
        c.mtc1(0, 20);
        let no_lights = c.g[S5][0] == 0;
        c.g[V1][0] = 0;
        if !no_lights {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            loop {
                c.cycles += 7;
                c.g[S4][0] = sx(off(c.g[V1][0] as u32, 1));
                c.g[V0][0] = addu(c.g[S6][0], c.g[V0][0]);
                c.g[S1][0] = c.ldw(off(c.g[V0][0] as u32, ARRAY));
                c.g[V0][0] = 6;
                c.g[V1][0] = c.ldw(off(c.g[S1][0] as u32, TYPE));
                let not_light = c.g[V1][0] != c.g[V0][0];
                c.g[V1][0] = c.g[S4][0];
                if !not_light {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S7][0];
                    c.g[A2][0] = c.g[S1][0];
                    callee::LIGHT_CONTRIBUTION.call(c, 0x002f_5c1c)?;
                    c.mov_s(1, 0);
                    c.c_lt_s(20, 1);
                    let not_brighter = !c.fcc();
                    c.g[V1][0] = c.g[S4][0];
                    if !not_brighter {
                        // Find the insertion point: while k < count && s < score[k].
                        c.cycles += 3;
                        c.g[A3][0] = 0;
                        let empty = (c.g[S0][0] as i64) <= 0;
                        c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                        // The `s >= score[0]` exit of the search jumps out of
                        // the block, skipping the search tail's extra cycle.
                        let mut skipped_tail = false;
                        if !empty {
                            c.cycles += 5;
                            c.lwc1(0, c.g[SP][0] as u32);
                            c.c_lt_s(1, 0);
                            let not_first = !c.fcc();
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                            if !not_first {
                                c.cycles += 2;
                                c.g[V1][0] = c.g[SP][0];
                                c.g[A3][0] = sx(off(c.g[A3][0] as u32, 1));
                                loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S0][0] as i64)) as u64;
                                    let done = c.g[V0][0] == 0;
                                    c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                                    if done {
                                        break;
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, c.g[V1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if !c.fcc() {
                                        break;
                                    }
                                    c.g[A3][0] = sx(off(c.g[A3][0] as u32, 1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f_5c58);
                                    }
                                }
                            } else {
                                skipped_tail = true;
                            }
                        }
                        if !skipped_tail {
                            c.cycles += 1;
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        }
                        let insert = c.g[V0][0] != 0;
                        c.cycles += 2;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        if insert {
                            c.cycles += 5;
                            c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            let no_shift = c.g[V0][0] == 0;
                            c.g[T0][0] = sx((c.g[A3][0] as u32) << 2);
                            if !no_shift {
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                c.g[A1][0] = addu(c.g[V0][0], c.g[S3][0]);
                                c.g[A0][0] = addu(c.g[V0][0], c.g[SP][0]);
                                loop {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.ldw((c.g[A1][0] as u32).wrapping_sub(4));
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_sub(4));
                                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                    c.stw(c.g[A1][0] as u32, c.g[V0][0]);
                                    c.swc1(0, c.g[A0][0] as u32);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(4));
                                    let more = c.g[V1][0] != 0;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(4));
                                    if !more {
                                        break;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f_5ca8);
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = addu(c.g[T0][0], c.g[S3][0]);
                            c.g[V1][0] = addu(c.g[SP][0], c.g[T0][0]);
                            c.stw(c.g[V0][0] as u32, c.g[S1][0]);
                            c.swc1(1, c.g[V1][0] as u32);
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                        c.g[V1][0] = addu(c.g[SP][0], c.g[V0][0]);
                        c.lwc1(20, c.g[V1][0] as u32);
                        c.g[V1][0] = c.g[S4][0];
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[S5][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5bf0);
                }
            }
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 20, 0x120);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiSplineModifier_v24(MultiSplineModifier *self)` (0x0035a918):
/// rebuild when dirty, then for every element recompute the world-space box
/// from the instance position and update the instance and spatial hash.
pub fn multi_spline_modifier_v24(c: &mut Cpu) {
    use spline::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, DIRTY));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, COUNT));
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.ldw(c.g[S3][0] as u32);
            c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
            c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = addu(c.g[S3][0], c.g[A0][0]);
            call_ptr(c, target, 0x0035_a958)?;
            c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, COUNT));
        }
        c.cycles += 4;
        c.g[S4][0] = 1;
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let no_elems = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xb0);
        if !no_elems {
            c.cycles += 1;
            c.g[S1][0] = ELEM_SIZE as u64;
            loop {
                c.cycles += 80;
                c.g[A1][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.mult(0, S1, S4, S1);
                c.lwc1(0, off(c.g[S3][0] as u32, RADIUS));
                c.g[A0][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[S2][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[S4][0] = sx(off(c.g[S4][0] as u32, 1));
                c.g[A0][0] = addu(c.g[S2][0], c.g[A0][0]);
                c.g[A1][0] = addu(c.g[S1][0], c.g[A1][0]);
                c.lq(2, off(c.g[A1][0] as u32, 0x30));
                c.sq(2, c.g[SP][0] as u32);
                c.lq(3, off(c.g[A1][0] as u32, 0x40));
                c.swc1(0, off(c.g[SP][0] as u32, 0x28));
                c.swc1(0, off(c.g[SP][0] as u32, 0x20));
                c.swc1(0, off(c.g[SP][0] as u32, 0x24));
                c.sq(3, off(c.g[SP][0] as u32, 0x10));
                c.stw(off(c.g[SP][0] as u32, 0x2c), 0);
                c.g[V1][0] = c.ldw(c.g[A0][0] as u32);
                c.lq(2, off(c.g[V1][0] as u32, 0x40));
                c.sq(2, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(3, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(4, off(c.g[SP][0] as u32, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, off(c.g[SP][0] as u32, 0x50));
                c.lq(2, off(c.g[SP][0] as u32, 0x50));
                c.sq(2, off(c.g[SP][0] as u32, 0x30));
                c.sq(2, off(c.g[A1][0] as u32, 0x30));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[A1][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[V1][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                c.g[A1][0] = addu(c.g[S1][0], c.g[A1][0]);
                c.lq(2, off(c.g[A0][0] as u32, 0x40));
                c.sq(2, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(3, off(c.g[SP][0] as u32, 0x40));
                c.lqc2(4, off(c.g[SP][0] as u32, 0x20));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, off(c.g[SP][0] as u32, 0x50));
                c.lq(2, off(c.g[SP][0] as u32, 0x50));
                c.sq(2, off(c.g[SP][0] as u32, 0x30));
                c.sq(2, off(c.g[A1][0] as u32, 0x40));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[V1][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.g[S0][0] = c.ldw(c.g[V1][0] as u32);
                callee::WORLD_GET_NASH.call(c, 0x0035_aa20)?;
                c.g[A3][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A3][0] = addu(c.g[A3][0], c.g[S1][0]);
                c.g[T0][0] = c.g[SP][0];
                c.g[A3][0] = sx(off(c.g[A3][0] as u32, 0x30));
                callee::SPATIAL_HASH_UPDATE.call(c, 0x0035_aa40)?;
                c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[V0][0] = addu(c.g[S1][0], c.g[V0][0]);
                c.g[V0][0] = sx(off(c.g[V0][0] as u32, 0x30));
                c.g[V1][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.lwc1(0, c.g[V0][0] as u32);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                c.swc1(0, off(c.g[A0][0] as u32, 0x60));
                c.lwc1(1, off(c.g[V0][0] as u32, 4));
                c.swc1(1, off(c.g[A0][0] as u32, 0x64));
                c.lwc1(0, off(c.g[V0][0] as u32, 8));
                c.swc1(0, off(c.g[A0][0] as u32, 0x68));
                c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, BOXES));
                c.g[V1][0] = c.ldw(off(c.g[S3][0] as u32, INSTS));
                c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                c.g[S1][0] = sx(off(c.g[S1][0] as u32, 0x40));
                c.g[S2][0] = addu(c.g[S2][0], c.g[V1][0]);
                c.lwc1(0, c.g[S1][0] as u32);
                c.g[V1][0] = c.ldw(c.g[S2][0] as u32);
                c.swc1(0, off(c.g[V1][0] as u32, 0x6c));
                c.lwc1(1, off(c.g[S1][0] as u32, 4));
                c.swc1(1, off(c.g[V1][0] as u32, 0x70));
                c.lwc1(0, off(c.g[S1][0] as u32, 8));
                c.swc1(0, off(c.g[V1][0] as u32, 0x74));
                c.g[V0][0] = c.ldw(off(c.g[S3][0] as u32, COUNT));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = ELEM_SIZE as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a970);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPad_Update(sInputPad *self, int n, float *values)`
/// (0x00321298): resize the button array, then per button store the value,
/// run the debounce/settle logic and the auto-repeat timer.
pub fn input_pad_update(c: &mut Cpu) {
    use pad::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.ldw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S1][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if !fits {
            c.cycles += 9;
            c.g[A2][0] = BUTTON_SIZE as u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.mult(0, A0, A0, A2);
            c.g[A1][0] = 0;
            c.mult(0, 0, V0, A2);
            c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
            c.g[A2][0] = c.lo[0];
            c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
            callee::EA_MEMSET.call(c, 0x0032_12e8)?;
        }
        c.cycles += 3;
        c.stw(c.g[S0][0] as u32, c.g[S1][0]);
        c.g[T4][0] = 0;
        let empty = (c.g[S1][0] as i64) <= 0;
        if !empty {
            c.cycles += 11;
            c.g[T5][0] = sx(off(c.g[S0][0] as u32, 0x18));
            c.mtc1(0, 1);
            c.g[A2][0] = c.g[S2][0];
            c.g[T3][0] = 1;
            c.g[T2][0] = sx(off(c.g[S0][0] as u32, 0x14));
            c.g[A3][0] = c.g[T5][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = sx(off(c.g[S0][0] as u32, 0x10));
            c.g[A0][0] = sx(off(c.g[S0][0] as u32, 0x1c));
            c.g[T7][0] = 0x18;
            c.g[T6][0] = 0xc;
            loop {
                // value = *values; settle/timer below.
                c.cycles += 5;
                c.lwc1(0, c.g[A2][0] as u32);
                c.g[V1][0] = c.ldw(c.g[A0][0] as u32);
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let settling = c.g[V0][0] != 0;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x18));
                if settling {
                    c.cycles += 6;
                    c.g[V0][0] = sx(off(c.g[V1][0] as u32, 1));
                    c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                    c.stw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = sx(off(c.g[S0][0] as u32, 0x10));
                    c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                } else {
                    // on = 0 < value
                    c.cycles += 4;
                    c.c_lt_s(1, 0);
                    let on = c.fcc();
                    c.g[V1][0] = on as u64;
                    if !on {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw((c.g[A0][0] as u32).wrapping_sub(0xc));
                    let changed = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx(off(c.g[S0][0] as u32, 0x10));
                    if !changed {
                        c.cycles += 3;
                        c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                        c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                    } else {
                        c.cycles += 2;
                        let released = c.g[V1][0] == 0;
                        c.stw(c.g[T1][0] as u32, c.g[V1][0]);
                        if released {
                            c.cycles += 2;
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), c.g[T3][0]);
                        } else {
                            c.cycles += 3;
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), c.g[T3][0]);
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                        }
                        c.cycles += 1;
                        c.stw(c.g[A0][0] as u32, 0);
                    }
                }
                // Auto-repeat.
                c.cycles += 4;
                c.g[V0][0] = addu(c.g[A1][0], c.g[T0][0]);
                c.g[V1][0] = c.ldw(c.g[V0][0] as u32);
                let up = c.g[V1][0] == 0;
                c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                if up {
                    c.cycles += 3;
                    c.g[V1][0] = sx(off(c.g[T5][0] as u32, c.g[T0][0] as u32));
                    c.stw(c.g[V0][0] as u32, 0);
                    c.stw(c.g[V1][0] as u32, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw((c.g[A0][0] as u32).wrapping_sub(4));
                    let has_timer = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if !has_timer {
                        c.cycles += 3;
                        c.stw((c.g[A0][0] as u32).wrapping_sub(8), c.g[T3][0]);
                        c.stw((c.g[A0][0] as u32).wrapping_sub(4), c.g[T7][0]);
                    } else {
                        c.cycles += 2;
                        let still = (c.g[V0][0] as i64) > 0;
                        c.stw(c.g[A3][0] as u32, c.g[V0][0]);
                        if still {
                            c.cycles += 3;
                            c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                            c.stw(c.g[V0][0] as u32, 0);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                            c.stw(c.g[V0][0] as u32, c.g[T3][0]);
                            c.stw(c.g[A3][0] as u32, c.g[T6][0]);
                        }
                    }
                }
                c.cycles += 8;
                c.g[T4][0] = sx(off(c.g[T4][0] as u32, 1));
                c.g[A3][0] = sx(off(c.g[A3][0] as u32, BUTTON_SIZE));
                c.g[T0][0] = sx(off(c.g[T0][0] as u32, BUTTON_SIZE));
                c.g[T1][0] = sx(off(c.g[T1][0] as u32, BUTTON_SIZE));
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, BUTTON_SIZE));
                c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(off(c.g[A2][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1320);
                }
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

/// `void MultiSplineModifier_v11(MultiSplineModifier *self)` (0x0035b418):
/// rebuild when dirty, then update every instance's box against the spatial
/// hash and select the model's vertex format.
pub fn multi_spline_modifier_v11(c: &mut Cpu) {
    use spline::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S4, 0x80);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, DIRTY));
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.ldw(off(c.g[S4][0] as u32, MODEL));
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.ldw(c.g[S4][0] as u32);
            c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
            c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = addu(c.g[S4][0], c.g[A0][0]);
            call_ptr(c, target, 0x0035_b464)?;
            c.g[V1][0] = c.ldw(off(c.g[S4][0] as u32, MODEL));
        }
        c.cycles += 6;
        c.g[S6][0] = 1;
        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, COUNT));
        c.g[S5][0] = c.ldw(off(c.g[V1][0] as u32, 0xc));
        c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let no_elems = c.g[V0][0] == 0;
        c.g[S7][0] = c.ldw(gp(c, 0x2a90));
        if !no_elems {
            c.cycles += 2;
            c.g[V0][0] = 0x60;
            loop {
                c.cycles += 23;
                c.g[S1][0] = c.ldw(off(c.g[S7][0] as u32, 0x10d8));
                c.mult(0, V0, S6, V0);
                c.g[S0][0] = c.ldw(off(c.g[S4][0] as u32, BOXES));
                c.g[S2][0] = c.ldh(off(c.g[S1][0] as u32, 0x2e8));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.ldh(off(c.g[S1][0] as u32, 0x158));
                c.g[V1][0] = c.ldw(off(c.g[S1][0] as u32, 0x15c));
                c.g[S2][0] = addu(c.g[S7][0], c.g[S2][0]);
                c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                c.g[A1][0] = addu(c.g[S7][0], c.g[A1][0]);
                c.g[S3][0] = sx(off(c.g[S0][0] as u32, 0x40));
                let target = c.g[V1][0] as u32;
                c.g[S0][0] = sx(off(c.g[S0][0] as u32, 0x30));
                call_ptr(c, target, 0x0035_b4bc)?;
                c.g[V0][0] = c.ldw(off(c.g[S1][0] as u32, 0x2ec));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                let target = c.g[V0][0] as u32;
                c.g[A3][0] = c.g[SP][0];
                call_ptr(c, target, 0x0035_b4d4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                let hit = c.g[S0][0];
                let hit_one = hit == c.g[V0][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                let mut skip_extra = false;
                if !hit_one {
                    c.cycles += 12;
                    c.g[V1][0] = c.ldw(off(c.g[S4][0] as u32, INSTS));
                    c.g[A3][0] = c.ldw(off(c.g[S5][0] as u32, 0xc));
                    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                    c.g[A2][0] = c.ldw(off(c.g[S4][0] as u32, MODE));
                    c.g[A1][0] = c.ldw(c.g[V0][0] as u32);
                    c.g[A0][0] = c.ldh(off(c.g[A3][0] as u32, 0x108));
                    c.g[V0][0] = c.ldw(off(c.g[A3][0] as u32, 0x10c));
                    c.g[A1][0] = sx(off(c.g[A1][0] as u32, 0x10));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = addu(c.g[S5][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0035_b50c)?;
                    let miss = c.g[S0][0] == 0;
                    c.g[A2][0] = 0x420;
                    if miss {
                        c.cycles += 7;
                        c.g[V0][0] = c.ldw(off(c.g[S5][0] as u32, 0xc));
                        c.g[A2][0] = 0x400;
                        c.g[A1][0] = c.ldw(off(c.g[S4][0] as u32, MODE));
                        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x28));
                        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x2c));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = addu(c.g[S5][0], c.g[A0][0]);
                        call_ptr(c, target, 0x0035_b550)?;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.ldw(off(c.g[S5][0] as u32, 0xc));
                        c.g[A1][0] = c.ldw(off(c.g[S4][0] as u32, MODE));
                        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x28));
                        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x2c));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = addu(c.g[S5][0], c.g[A0][0]);
                        call_ptr(c, target, 0x0035_b52c)?;
                        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, COUNT));
                        skip_extra = true;
                    }
                }
                if !skip_extra {
                    c.cycles += 1;
                    c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, COUNT));
                }
                c.cycles += 4;
                c.g[S6][0] = sx(off(c.g[S6][0] as u32, 1));
                c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = 0x60;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_b488);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.ldw(off(c.g[S4][0] as u32, FOLLOW));
        let no_follow = c.g[V0][0] == 0;
        c.g[A1][0] = sx(off(c.g[S4][0] as u32, 0x20));
        if !no_follow {
            c.cycles += 2;
            c.g[A0][0] = sx(off(c.g[S4][0] as u32, 0x48));
            callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH.call(c, 0x0035_b578)?;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `int SprStream_SetMode(SprStream *self, int mode)` (0x0038f300): set the
/// ring-buffer mode, its element count/stride and the buffer addresses, then
/// flush the scratchpad DMA. The buffer loop is re-enterable at
/// `0x0038f3e8` (the trampoline resumes a loop whose event asked to unwind).
pub fn spr_stream_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        let resumed = c.pc == 0x0038_f3e8;
        if !resumed && c.pc != 0x0038_f300 {
            c.jump_out(c.pc, 0x0038_f300, 0x0038_f460);
            return Ok(());
        }
        let frame = if resumed {
            Frame { size: 0x10 }
        } else {
            Frame::open(c, 0x10)
        };
        let mut run_loop = resumed;
        if !resumed {
            c.cycles += 6;
            c.g[A2][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V0][0] = c.ldw(c.g[A2][0] as u32);
            let same = c.g[V0][0] == c.g[A1][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if same {
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 5;
            c.stw(c.g[A2][0] as u32, c.g[A1][0]);
            c.g[A1][0] = c.g[V0][0];
            let in_range = c.g[A1][0] < 7;
            c.g[V0][0] = in_range as u64;
            c.g[V0][0] = 0x49_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x2ef0));
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                match target {
                    0x0038_f344 => {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                    }
                    0x0038_f34c | 0x0038_f354 | 0x0038_f35c | 0x0038_f364 => {
                        c.cycles += 2;
                        c.g[V0][0] = 2;
                    }
                    0x0038_f36c => {
                        c.cycles += 1;
                        c.g[V0][0] = 3;
                    }
                    _ => {
                        c.jump_out(target, 0x0038_f300, 0x0038_f460);
                        return Ok(());
                    }
                }
                c.cycles += 1;
                c.stw(off(c.g[A2][0] as u32, 4), c.g[V0][0]);
            }
            c.cycles += 4;
            c.stw(off(c.g[A2][0] as u32, 8), 0);
            let stride_range = c.g[A1][0] < 7;
            c.g[V0][0] = stride_range as u64;
            c.stw(off(c.g[A2][0] as u32, 0xc), 0);
            if stride_range {
                c.cycles += 7;
                c.g[V0][0] = 0x49_0000;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x2f10));
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                // Each handler block has its own cost (0x38f3c8 is the
                // fall-through tail and costs one cycle, the rest two).
                let (handler, stride) = match target {
                    0x0038_f3a0 => (2u64, 0x400u64),
                    0x0038_f3a8 => (2, 0x200),
                    0x0038_f3b0 => (2, 0x1c0),
                    0x0038_f3b8 => (2, 0x180),
                    0x0038_f3c0 => (2, 0x140),
                    0x0038_f3c8 => (1, 0x100),
                    _ => {
                        c.jump_out(target, 0x0038_f300, 0x0038_f460);
                        return Ok(());
                    }
                };
                c.cycles += handler;
                c.cycles += 1;
                c.g[V0][0] = stride;
                c.stw(off(c.g[A2][0] as u32, 0x10), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
            let no_buffers = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0;
            if !no_buffers {
                c.cycles += 3;
                c.g[A1][0] = 0x7000_0000;
                c.g[V1][0] = sx(off(c.g[A2][0] as u32, 0x2c));
                run_loop = true;
            }
        }
        if run_loop {
            if spr_stream_buffer_loop(c).is_err() {
                return Ok(());
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 0x10));
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = ((c.g[V0][0] as i64) < 1024) as u64;
        let fits = c.g[V1][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        if fits {
            c.cycles += 1;
            c.stw(gp(c, 0x2a94), 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = 0x7000_0000;
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.stw(gp(c, 0x2a94), c.g[V0][0]);
        }
        c.cycles += 4;
        callee::DMA_WAIT_FROM_SCRATCHPAD.call(c, 0x0038_f44c)?;
        callee::DMA_WAIT_TO_SCRATCHPAD.call(c, 0x0038_f454)?;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `0x38f3e8` buffer-fill loop of `SprStream_SetMode`: writes
/// `0x70000000 + stride*i*16` and clears the per-entry words until
/// `i == count`. Returns `Ok` when the loop completed (the caller continues
/// at `0x38f418`) and `Err` when an event asked the thread to unwind.
fn spr_stream_buffer_loop(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 12;
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 0x10));
        c.stw((c.g[V1][0] as u32).wrapping_sub(0xc), 0);
        c.mult(0, V0, V0, A0);
        c.stw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx(off(c.g[A0][0] as u32, 1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
        c.stw((c.g[V1][0] as u32).wrapping_sub(0x18), c.g[V0][0]);
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, 0x0038_f3e8) {
            return Err(crate::call::Unwound);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{covered, machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, put_words, Rng, DATA, GAME_GP};

    fn put_f32(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    #[test]
    fn replay_stream_write_node_ref_matches() {
        covered(
            ssx3_ee::s07::f_0026ed28__generated,
            replay_stream_write_node_ref,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26ed28, 1);
                let out = DATA;
                let node = DATA + 0x400;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = node as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(out, vt);
                let target = if seed % 3 == 0 { 0 } else { DATA + 0x800 };
                c.bus.write32(node + 0x18, target);
                put_f32(c, target + 0x78, r.next_u32());
                c.bus.write16(target + 8, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn typed_process_save_matches() {
        covered(
            ssx3_ee::s11::f_00354948__generated,
            typed_process_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x354948, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = obj as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(obj, vt);
                c.bus.write32(this + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn sync_process_save_matches() {
        covered(
            ssx3_ee::s11::f_00354980__generated,
            sync_process_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x354980, 2);
                let this = DATA + 0x100;
                let obj = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = obj as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(obj, vt);
            },
        );
    }

    #[test]
    fn world_obj_save_matches() {
        covered(ssx3_ee::s11::f_0034fe90__generated, world_obj_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34fe90, 3);
            let this = DATA + 0x100;
            let stream = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            let vt = vtable(c, &mut r, &[8, 0x30]);
            c.bus.write32(stream, vt);
        });
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
    fn luno_vm_register_dtor_matches() {
        check(
            ssx3_ee::s05::f_00225b90__generated,
            luno_vm_register_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x225b_90 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let selfp = DATA + 0x100;
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                c.bus
                    .write32(selfp + 8, if seed % 3 == 0 { 4 } else { r.next_u32() % 4 });
                let vt = DATA + 0x800;
                c.bus.write32(selfp, vt);
                c.bus.write32(vt + 8, 2 + r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn cflag_manager_group_update_matches() {
        covered(
            ssx3_ee::s11::f_0034b818__generated,
            cflag_manager_group_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34b818, 1);
                let selfp = DATA;
                let world = DATA + 0x4000;
                let w84 = DATA + 0x4100;
                let mesh = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_add(0x1150), 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, w84);
                c.bus.write32(w84 + 0x70, DATA + 0x4200);
                c.bus.write32(w84 + 0xc, DATA + 0x4300);
                let frame = r.next_u32() % 8;
                c.bus.write32(DATA + 0x4308, frame);
                c.g[A0][0] = selfp as u64;
                c.bus
                    .write32(selfp + 0x5c, if seed % 7 == 0 { 0 } else { 1 });
                c.bus
                    .write32(selfp + 0x60, if seed % 5 == 0 { 0 } else { mesh });
                c.bus.write32(
                    selfp + 0x90,
                    if seed % 2 == 0 {
                        frame & 1
                    } else {
                        (frame + 1) & 1
                    },
                );
                put_f32(c, selfp + 0x4c, r.edge_f32());
                put_f32(c, DATA + 0x4200 + 0x10, r.edge_f32());
                for i in 0..4 {
                    put_f32(c, selfp + 4 + 4 * i, r.edge_f32());
                    put_f32(c, selfp + 0x78 + 4 * i, r.edge_f32());
                }
                put_f32(c, selfp + 0x50, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                put_f32(c, selfp + 0x54, if seed % 4 == 1 { 0 } else { r.f32(1.0) });
                put_f32(c, selfp + 0x88, r.f32(1.0));
                put_f32(c, selfp + 0x8c, r.f32(1.0));
                put_f32(c, GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001f
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mesh + 4, vt);
            },
        );
    }

    #[test]
    fn intersect_info_evaluate_matrix_matches() {
        check(
            ssx3_ee::s11::f_00351800__generated,
            intersect_info_evaluate_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x3518_00 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let selfp = DATA;
                c.g[A0][0] = selfp as u64;
                c.bus.write32(selfp + 0x40, 0);
                for k in 0..3 {
                    put_f32(c, selfp + 0x50 + 4 * k, r.f32(360.0));
                    put_f32(c, selfp + 0x44 + 4 * k, r.f32(1000.0));
                }
                put_f32(c, GAME_GP.wrapping_sub(0x2a90), 0x3c8e_fa36);
            },
        );
    }

    #[test]
    fn anim_object_v52_matches() {
        covered(
            ssx3_ee::s11::f_0034e348__generated,
            anim_object_v52,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34e348, 3);
                let selfp = DATA;
                c.g[A0][0] = selfp as u64;
                let items = DATA + 0x1000;
                let count = seed % 4;
                c.bus.write32(selfp + 0x40, count);
                c.bus
                    .write32(selfp + 0x48, if count == 0 { 0 } else { items });
                put_f32(c, selfp + 4, r.edge_f32());
                c.bus.write32(selfp + 0x10, r.next_u32());
                let vt = vtable(c, &mut r, &[0x10]);
                for i in 0..3 {
                    c.bus.write32(items + 0xd0 * i + 0x84, vt);
                }
            },
        );
    }

    #[test]
    fn world_light_man_find_strongest_lights_matches() {
        check(
            ssx3_ee::s09::f_002f5b68__generated,
            world_light_man_find_strongest_lights,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f5b_68 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let mgr = DATA;
                let out = DATA + 0x800;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = mgr as u64;
                c.g[A3][0] = out as u64;
                let max = 1 + seed % 5;
                c.g[T0][0] = max as u64;
                let count = seed % 5;
                c.bus.write32(mgr + 0x210, count);
                for i in 0..4 {
                    let light = DATA + 0x1000 + 0x100 * i as u32;
                    c.bus.write32(mgr + 0x214 + 4 * i, light);
                    c.bus.write32(
                        light + 8,
                        if i < count / 2 + (seed % 2) {
                            6
                        } else {
                            r.next_u32() % 8
                        },
                    );
                    c.bus.write32(light + 0x10, 0);
                    put_f32(c, light + 0x18, r.f32(2.0));
                }
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v24_matches() {
        covered(
            ssx3_ee::s11::f_0035a918__generated,
            multi_spline_modifier_v24,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35a918, 1);
                let vt = vtable(c, &mut r, &[0x18]);
                let selfp = DATA;
                let world = DATA + 0x4000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x4100);
                c.bus.write32(DATA + 0x4100 + 0x20, DATA + 0x4400);
                let elems = DATA + 0x1000;
                let insts = DATA + 0x3000;
                c.g[A0][0] = selfp as u64;
                c.bus
                    .write32(selfp + 0x30, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(selfp + 4, if seed % 3 == 0 { 1 } else { 2 });
                c.bus.write32(selfp + 0x3c, elems);
                c.bus.write32(selfp + 0x44, insts);
                put_f32(c, selfp + 0x18, 0);
                if seed % 2 != 0 {
                    c.bus.write32(selfp, vt);
                }
                for i in 0..2 {
                    c.bus
                        .write32(insts + 4 * i, DATA + 0x5000 + 0x100 * i as u32);
                }
            },
        );
    }

    #[test]
    fn input_pad_update_matches() {
        check_lohi(
            ssx3_ee::s10::f_00321298__generated,
            input_pad_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x3212_98 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let selfp = DATA;
                let count = seed % 3;
                let n = (seed / 3) % 5;
                let vals = DATA + 0x400;
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = vals as u64;
                c.bus.write32(selfp, count);
                for i in 0..4 {
                    let b = selfp + 4 + 0x1c * i;
                    put_f32(c, b, r.edge_f32());
                    c.bus.write32(b + 4, if i % 2 == 0 { 1 } else { 0 });
                    c.bus.write32(b + 8, 0);
                    c.bus.write32(b + 0xc, r.next_u32() % 2);
                    c.bus.write32(b + 0x10, 0);
                    c.bus.write32(b + 0x14, r.next_u32() % 0x20);
                    c.bus.write32(b + 0x18, r.next_u32() % 4);
                }
                for i in 0..4 {
                    put_f32(c, vals + 4 * i, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn multi_spline_modifier_v11_matches() {
        covered(
            ssx3_ee::s11::f_0035b418__generated,
            multi_spline_modifier_v11,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35b418, 5);
                let selfp = DATA;
                let world = DATA + 0x4000;
                let model = DATA + 0x7000;
                let modelobj = DATA + 0x8000;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), world);
                // vt used for slots 43 and 93 (w + 0x10d8).
                let vt1 = DATA + 0x5000;
                c.bus.write32(world + 0x10d8, vt1);
                c.bus.write16(vt1 + 0x158, 0);
                c.bus.write32(vt1 + 0x15c, FAKE);
                c.bus.write16(vt1 + 0x2e8, 0);
                c.bus.write32(vt1 + 0x2ec, FAKE + 0x10);
                // model -> object vtable slots 33 and 5.
                c.bus.write32(model + 0xc, modelobj);
                let vt2 = DATA + 0x6000;
                c.bus.write32(modelobj + 0xc, vt2);
                c.bus.write16(vt2 + 0x108, 0);
                c.bus.write32(vt2 + 0x10c, FAKE + 0x20);
                c.bus.write16(vt2 + 0x28, 0);
                c.bus.write32(vt2 + 0x2c, FAKE + 0x30);
                c.g[A0][0] = selfp as u64;
                c.bus.write32(selfp, 0);
                c.bus.write32(selfp + 4, if seed % 4 == 0 { 1 } else { 2 });
                c.bus
                    .write32(selfp + 0x30, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(selfp + 0x38, r.next_u32() % 2);
                c.bus.write32(selfp + 0x3c, DATA + 0x1000);
                c.bus.write32(selfp + 0x40, model);
                c.bus.write32(selfp + 0x44, DATA + 0x3000);
                c.bus.write32(selfp + 0x1c, 0);
                let forced = if seed % 3 == 0 { 1 } else { 2 };
                c.bus.write32(LOG_RESULTS + 4, forced);
                c.bus.write32(LOG_RESULTS + 8, forced);
                for i in 0..2 {
                    c.bus
                        .write32(DATA + 0x3000 + 4 * i, DATA + 0x9000 + 0x100 * i as u32);
                }
            },
        );
    }

    #[test]
    fn spr_stream_set_mode_matches() {
        driven!(gen_driven, ssx3_ee::s12::f_0038f300__generated);
        driven!(hand_driven, spr_stream_set_mode);
        fn setup(c: &mut Cpu, seed: u32) -> Events {
            let mut r = Rng(0x38f3_00 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let selfp = DATA;
            c.g[A0][0] = selfp as u64;
            let mode = 1 + seed % 8;
            c.g[A1][0] = mode as u64;
            c.bus.write32(selfp, if seed % 5 == 0 { mode } else { 0 });
            put_words(
                c,
                0x49_0000 + 0x2ef0,
                &[
                    0x0038_f344,
                    0x0038_f34c,
                    0x0038_f354,
                    0x0038_f35c,
                    0x0038_f364,
                    0x0038_f36c,
                    0x0038_f36c,
                ],
            );
            put_words(
                c,
                0x49_0000 + 0x2f10,
                &[
                    0x0038_f3a0,
                    0x0038_f3a8,
                    0x0038_f3b0,
                    0x0038_f3b8,
                    0x0038_f3c8,
                    0x0038_f3c0,
                    0x0038_f3c8,
                ],
            );
            schedule(c, &mut r, seed, 0x0038_f300)
        }
        check_events(gen_driven, hand_driven, 60, &setup);
    }
}
