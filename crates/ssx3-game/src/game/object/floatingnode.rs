//! `src/object/floatingnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::be::beprepoststate::gp_lw;
pub(crate) use crate::game::be::beprepoststate::gp_rel;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::sound::soundsys::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
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
        0x0034_ef08,
        floating_free_bound_box,
        "Floating_FreeBoundBox",
    ),
    (0x0034_f048, floating_v49, "Floating_v49"),
    (0x0034_f930, floating_v15, "Floating_v15"),
    (0x0034_fa88, floating_save, "Floating_Save"),
    (0x0034_fb00, world_obj_ctor, "WorldObj_ctor"),
    (
        0x0034_eeb8,
        floating_alloc_bound_box,
        "Floating_AllocBoundBox",
    ),
    (
        0x0034_f120,
        floating_node_build_matrices,
        "Floating_BuildMatrices",
    ),
    (
        0x0034_f600,
        floating_node_apply_force,
        "cFloatingNode_applyForce",
    ),
    (
        0x0034_f790,
        floating_node_update_intersect_info,
        "cFloatingNode_updateIntersectInfo",
    ),
    (0x0034_ef40, floating_v50_2, "Floating_v50"),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
}

/// `__builtin_delete` (0x317e50).
pub(crate) const DELETE: u32 = 0x0031_7e50;

/// `void Floating_FreeBoundBox(this)` (0x34ef08): delete the bounding box at
/// `+0x78` and clear the pointer.
pub fn floating_free_bound_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0x78)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, DELETE, 0x0034_ef2c)?;
            c.sw((this as u32).wrapping_add(0x78), 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1 fN, off(base)`.
#[inline(always)]
pub(crate) fn ldf_at(c: &mut Cpu, n: usize, base: u64, off: u32) {
    c.lwc1(n, ea(base, off));
}

/// `swc1 fN, off(base)`.
#[inline(always)]
pub(crate) fn stf_at(c: &mut Cpu, n: usize, base: u64, off: u32) {
    c.swc1(n, ea(base, off));
}

// ---------------------------------------------------------------------------
// Comparators and predicates.

/// `float Floating_v15(Floating* self)` (0x34F930): one step of the
/// three-axis damped spring used by `sFloatTilt` (board float), then clamp the
/// lean angles to +-80 degrees and clear the inputs.
pub fn floating_v15(c: &mut Cpu) {
    c.cycles += 68;
    let a0 = A0;
    ldf_at(c, 14, c.g[a0][0], 0x50); // pos1
    c.g[V1][0] = sx(ea(c.g[a0][0], 0x2c));
    ldf_at(c, 2, c.g[a0][0], 0x2c); // k
    ldf_at(c, 0, c.g[a0][0], 0x4c); // rest1
    ldf_at(c, 1, c.g[a0][0], 0x38); // input1
    c.neg_s(2, 2);
    c.sub_s(0, 14, 0);
    ldf_at(c, 13, c.g[a0][0], 0x64); // vel2
    c.neg_s(1, 1);
    ldf_at(c, 12, c.g[a0][0], 0x54); // vel1
    ldf_at(c, 7, c.g[a0][0], 0x40); // gain
    ldf_at(c, 3, c.g[a0][0], 0x60); // k2
    c.mul_s(2, 2, 0);
    c.mul_s(5, 1, 13);
    ldf_at(c, 4, c.g[a0][0], 0x3c); // damp
    c.g[V0][0] = gp_lw(c, 0x2a74); // the frame counter object
    c.mul_s(3, 7, 3);
    ldf_at(c, 9, c.g[a0][0], 0x44); // input2
    c.mul_s(1, 1, 12);
    ldf_at(c, 0, c.g[a0][0], 0x70); // pos3
    c.neg_s(4, 4);
    ldf_at(c, 8, c.g[a0][0], 0x34); // pos2
    ldf_at(c, 15, c.g[V0][0], 0x10); // frame delta
    c.cvt_s_w(15, 15);
    ldf_at(c, 6, c.g[a0][0], 0x30); // damp2
    c.mul_s(7, 7, 0);
    ldf_at(c, 11, c.g[a0][0], 0x6c); // vel3
    c.mul_s(8, 8, 9);
    c.neg_s(6, 6);
    ldf_at(c, 9, c.g[a0][0], 0x48); // input3
    ldf_at(c, 10, c.g[a0][0], 0x5c); // rest2
    c.add_s(1, 1, 3);
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.mul_s(3, 4, 11);
    c.mul_s(4, 4, 10);
    c.div_s(0, 0, 15); // dt = 1 / fps
    c.add_s(2, 2, 8);
    c.add_s(5, 5, 7);
    c.lwc1(8, gp_rel(c, -0x2a98)); // +80 degrees
    c.mul_s(6, 6, 9);
    c.lwc1(7, gp_rel(c, -0x2a94)); // -80 degrees
    c.add_s(1, 1, 4);
    c.add_s(5, 5, 3);
    c.add_s(2, 2, 6);
    c.mul_s(3, 9, 0);
    c.mul_s(4, 10, 0);
    c.mul_s(2, 2, 0);
    c.mul_s(5, 5, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(0, 11, 0);
    c.add_s(14, 14, 3);
    c.add_s(9, 9, 2);
    c.add_s(13, 13, 0);
    c.add_s(12, 12, 4);
    stf_at(c, 14, c.g[a0][0], 0x50);
    c.add_s(10, 10, 1);
    stf_at(c, 9, c.g[a0][0], 0x48);
    c.add_s(11, 11, 5);
    stf_at(c, 13, c.g[a0][0], 0x64);
    stf_at(c, 12, c.g[a0][0], 0x54);
    stf_at(c, 10, c.g[a0][0], 0x5c);
    stf_at(c, 11, c.g[a0][0], 0x6c);
    ldf_at(c, 0, c.g[V1][0], 0x28);
    c.c_le_s(7, 0);
    let keep = !c.fcc();
    c.mov_s(15, 7);
    if !keep {
        c.cycles += 1;
        c.min_s(15, 0, 8);
    }
    stf_at(c, 15, c.g[a0][0], 0x54);
    c.cycles += 6;
    ldf_at(c, 0, c.g[V1][0], 0x38);
    c.c_le_s(7, 0);
    let keep = !c.fcc();
    c.mov_s(1, 7);
    if !keep {
        c.cycles += 1;
        c.min_s(1, 0, 8);
    }
    c.cycles += 9;
    stf_at(c, 1, c.g[a0][0], 0x64);
    c.g[V1][0] = c.lhu(ea(c.g[a0][0], 0x12));
    c.g[V0][0] = 1;
    c.g[V1][0] |= 1;
    c.sw(ea(c.g[a0][0], 0x44), 0);
    c.sh(ea(c.g[a0][0], 0x12), c.g[V1][0]);
    c.sw(ea(c.g[a0][0], 0x60), 0);
    c.sw(ea(c.g[a0][0], 0x70), 0);
    abi::ret(c);
}

/// `func_0034FA88` (0x34fa88): `Floating_Save`, the floating-modifier
/// serialiser. Saves the modifier object (0x356b30), writes a 4-byte flag
/// (`this->+0x78 > 0`) plus the base pointer from the stack, then the 0x48-byte
/// block at `this+0x2c` through the archive writer.
pub fn floating_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0034_faa4);
        c.g[S1][0] = c.g[A0][0];
        callee::MOD_OBJ_SAVE.call(c, 0x0034_faa4)?;
        c.g[V1][0] = lw_r(c, S0, 0);
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = lw_r(c, S1, 0x78);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh_r(c, V1, 0x8);
        c.g[A3][0] = lw_r(c, V1, 0xc);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        let target = low(c, A3);
        c.g[RA][0] = sx(0x0034_facc);
        sw_r(c, SP, 0, c.g[V0][0]);
        call_ptr(c, target, 0x0034_facc)?;
        c.g[V1][0] = lw_r(c, S0, 0);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x2c));
        c.g[A2][0] = 0x48;
        c.g[A0][0] = lh_r(c, V1, 0x8);
        c.g[V0][0] = lw_r(c, V1, 0xc);
        let target = low(c, V0);
        c.g[RA][0] = sx(0x0034_fae8);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_fae8)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Floating modifier.

/// `void Floating_v49(Floating* self)` (0x34f048): refresh the bound box from
/// the instance's min/max and notify the collision system, or run the base
/// update when there is no box info.
pub fn floating_v49(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S0, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        c.g[A1][0] = Addr(p(r32(c, S1), 0x78)).lw(c);
        let no_info = c.g[A1][0] == 0;
        if no_info {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0035_67e0, 0x0034_f10c)?;
            c.lq(S0, frame.at(c, 0x70));
        } else {
            c.cycles += 39;
            c.lq(2, r32(c, A1));
            c.g[V1][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            c.sq(2, p(r32(c, SP), 0x20));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            c.lq(2, p(r32(c, A1), 0x10));
            c.sq(2, p(r32(c, SP), 0x30));
            c.lwc1(0, p(r32(c, V1), 0x60));
            c.swc1(0, p(r32(c, SP), 0x40));
            c.lwc1(1, p(r32(c, V1), 0x64));
            c.swc1(1, p(r32(c, SP), 0x44));
            c.lwc1(0, p(r32(c, V1), 0x68));
            c.swc1(2, p(r32(c, SP), 0x4c));
            c.swc1(0, p(r32(c, SP), 0x48));
            c.lq(4, p(r32(c, SP), 0x40));
            c.sq(4, r32(c, SP));
            c.lwc1(0, p(r32(c, V1), 0x6c));
            c.swc1(0, p(r32(c, SP), 0x40));
            c.lwc1(1, p(r32(c, V1), 0x70));
            c.swc1(1, p(r32(c, SP), 0x44));
            c.lwc1(0, p(r32(c, V1), 0x74));
            c.swc1(0, p(r32(c, SP), 0x48));
            c.lq(2, p(r32(c, SP), 0x40));
            c.sq(2, p(r32(c, SP), 0x10));
            c.sq(4, r32(c, A1));
            c.lq(2, p(r32(c, SP), 0x10));
            c.sq(2, p(r32(c, A1), 0x10));
            c.g[S0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            call_ptr(c, 0x002d_1be0, 0x0034_f0dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(p(r32(c, SP), 0x20));
            call_ptr(c, 0x0032_91e0, 0x0034_f0f4)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0034_ef08, 0x0034_f0fc)?;
            c.lq(S0, frame.at(c, 0x70));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObj_ctor (0x34fb00)
// ---------------------------------------------------------------------------

/// WorldObj: vtable (+0xc) and its world node (+0x18).
pub(crate) const WOBJ_VTABLE: u32 = 0xc;

pub(crate) const WOBJ_NODE: u32 = 0x18;

/// The WorldObj vtable (`D_00491C80`).
pub(crate) const WOBJ_VTABLE_ADDR: u32 = 0x0049_1c80;

/// Node flags (+8): 0x100 = already in the focus manager.
pub(crate) const WOBJ_NODE_FLAGS: u32 = 0x8;

pub(crate) const WOBJ_NODE_FOCUSED: u64 = 0x100;

/// Process kinds that never take focus.
pub(crate) const WOBJ_KINDS_UNFOCUSED: [u64; 3] = [0x6, 0x10, 0x16];

/// `WorldObj* WorldObj_ctor(WorldObj* self, int list, int kind, Node* node)`
/// (0x34fb00): constructs the sync process (`SyncProcess_ctor(self, list,
/// kind)`), sets the node and the WorldObj vtable, links the node back to
/// the object, and adds the node to the focus manager unless it is already
/// there or the kind is 6, 0x10 or 0x16. Returns `self`.
pub fn world_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        Callee {
            addr: 0x0035_4850,
            f: ee::s11::f_00354850,
        }
        .call(c, 0x0034_fb24)?;
        let this = Addr(r32(c, S2));
        let node = c.g[S0][0];
        this.off(WOBJ_NODE).sw(c, node);
        c.g[V1][0] = sx(WOBJ_VTABLE_ADDR);
        c.g[A0][0] = c.g[S2][0];
        this.off(WOBJ_VTABLE).sw(c, sx(WOBJ_VTABLE_ADDR));
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x0034_fc78,
            f: ee::s11::f_0034fc78,
        }
        .call(c, 0x0034_fb40)?;
        let focused = Addr(r32(c, S0)).off(WOBJ_NODE_FLAGS).lw(c) & WOBJ_NODE_FOCUSED != 0;
        c.g[V0][0] = c.g[S2][0];
        if !focused {
            // Compare the kind against 6, 0x10, 0x16 in turn (each delay
            // slot loads the next constant into v0, the last one `self`).
            c.cycles += 3;
            let kind = c.g[S1][0];
            c.g[V0][0] = 0x10;
            let add = if kind == WOBJ_KINDS_UNFOCUSED[0] {
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
                false
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x16;
                if kind == WOBJ_KINDS_UNFOCUSED[1] {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    kind != WOBJ_KINDS_UNFOCUSED[2]
                }
            };
            if add {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002d_1bf0, 0x0034_fb74)?; // World_AddFocusObject
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
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

/// `void Floating_AllocBoundBox(cFloatingNode* self)` (0x34eeb8): allocates the
/// 0x30-byte bound box (`EA_MemAllocNamed`, name 0x48e890, flags 0x20000000)
/// into `self + 0x78`, then fills it with `WorldObj_GetBoundBox(self, box,
/// box + 0x20)`.
pub fn floating_alloc_bound_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x1770));
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x30;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0034_eee0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        let box_ptr = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x78).sw(c, box_ptr);
        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        Callee {
            addr: 0x0035_0570,
            f: ee::s11::f_00350570,
        }
        .call(c, 0x0034_eef4)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `base + off` as a guest address (the 32-bit wrap of a 64-bit register).
#[inline]
pub(crate) fn oc604_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `lqc2 vf..vf+3` from the address in GPR `reg`, `+0x10`, `+0x20`, `+0x30`.
pub(crate) fn oc604_quad_load(c: &mut Cpu, vf: usize, reg: usize) {
    let base = c.g[reg][0] as u32;
    for k in 0..4 {
        c.lqc2(vf + k, base.wrapping_add(0x10 * k as u32));
    }
}

/// `sqc2 vf..vf+3` to the address in GPR `reg`, `+0x10`, `+0x20`, `+0x30`.
pub(crate) fn oc604_quad_store(c: &mut Cpu, vf: usize, reg: usize) {
    let base = c.g[reg][0] as u32;
    for k in 0..4 {
        c.sqc2(vf + k, base.wrapping_add(0x10 * k as u32));
    }
}

/// One matrix-times-column step on VU0: rows vf4..vf7 times the broadcast
/// components of `col`, into `dst` (the generated `mula`/`madda`/`madd` quartet).
pub(crate) fn oc604_mul4(c: &mut Cpu, col: u8, dst: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(col, X));
    c.vu0().madda(0b1111, 5, Src::Bc(col, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(col, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(col, W));
}

/// The four column products of a 4x4 transform: rows vf4..vf7, columns
/// vf8..vf11, results vf12..vf15.
pub(crate) fn oc604_mat4_columns(c: &mut Cpu) {
    for k in 0..4u8 {
        oc604_mul4(c, 8 + k, 12 + k);
    }
}

/// The `add`/`mul`/`adda`/`madda`/`madd` pass that combines the two
/// vectors loaded in vf3 and vf5 (the generated axis-product sequence).
pub(crate) fn oc604_axis_pass(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

// ---------------------------------------------------------------------------
// cFloatingNode::updateOverlapSystem (0x34ef40), `Floating_v50`
// ---------------------------------------------------------------------------

/// `void cFloatingNode::updateOverlapSystem(void)` (0x34ef40): with an
/// `Info` block, the node's bounding box is its position (`nodePos`, from the
/// transform at +0xc through the vtable slot at +0xc0/+0xc4) plus and minus
/// `Info->Radius` (+0x20) on all three axes; the node is then moved in the
/// overlap system through `Nash::movenode` with the old and new boxes. Without
/// `Info` the base class does it (`cMoveNode::updateOverlapSystem`, 0x3568b0).
///
/// `self` (s0): `Info` at +0x78, `Instance` at +0x18.
pub(crate) fn floating_v50_body(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xa0));
    c.sq(16, sp(c).wrapping_add(0x90));
    c.sd(sp(c).wrapping_add(0x80), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.lw(oc604_ea(c.g[S0][0], 0x78));
    if c.g[A0][0] != 0 {
        c.cycles += 52;
        c.lq(3, c.g[A0][0] as u32);
        c.g[A1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
        c.sq(3, sp(c).wrapping_add(0x20));
        c.lq(2, oc604_ea(c.g[A0][0], 0x10));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lwc1(0, oc604_ea(c.g[A0][0], 0x20));
        c.sw(sp(c).wrapping_add(0x4c), 0u64);
        c.swc1(0, sp(c).wrapping_add(0x48));
        c.swc1(0, sp(c).wrapping_add(0x40));
        c.swc1(0, sp(c).wrapping_add(0x44));
        c.g[A0][0] = c.lh(oc604_ea(c.g[A1][0], 0xc0));
        c.g[V0][0] = c.lw(oc604_ea(c.g[A1][0], 0xc4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0034_ef94)?;
        c.lq(3, oc604_ea(c.g[V0][0], 0x30));
        c.sq(3, sp(c).wrapping_add(0x60));
        c.lqc2(3, sp(c).wrapping_add(0x60));
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x70));
        c.lq(2, sp(c).wrapping_add(0x70));
        c.g[V1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
        c.sq(2, sp(c));
        c.sq(2, sp(c).wrapping_add(0x50));
        c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0xc0));
        c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0xc4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0034_efcc)?;
        c.lq(3, oc604_ea(c.g[V0][0], 0x30));
        c.sq(3, sp(c).wrapping_add(0x60));
        c.lqc2(3, sp(c).wrapping_add(0x60));
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x70));
        c.lq(2, sp(c).wrapping_add(0x70));
        c.g[A0][0] = c.lw(oc604_ea(c.g[S0][0], 0x78));
        c.lq(3, sp(c));
        c.sq(2, sp(c).wrapping_add(0x10));
        c.sq(2, sp(c).wrapping_add(0x50));
        c.sq(3, c.g[A0][0] as u32);
        c.lq(2, sp(c).wrapping_add(0x10));
        c.sq(2, oc604_ea(c.g[A0][0], 0x10));
        c.g[S0][0] = c.lw(oc604_ea(c.g[S0][0], 0x18));
        call_ptr(c, 0x002d_1be0, 0x0034_f00c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = c.g[SP][0];
        c.g[T0][0] = sx(sp(c).wrapping_add(0x20));
        call_ptr(c, 0x0032_91e0, 0x0034_f024)?;
        c.lq(16, sp(c).wrapping_add(0x90));
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_68b0, 0x0034_f034)?;
        c.lq(16, sp(c).wrapping_add(0x90));
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x80));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xa0));
    c.pc = t;
    Ok(())
}

// ---------------------------------------------------------------------------
// Floating_BuildMatrices (0x34f120)
// ---------------------------------------------------------------------------

/// `Floating_BuildMatrices` (0x34f120): for each segment of the node's
/// spline (`count` at `[t6+4]`, the entries `t6`, `t6+0x40`... of 0x40 bytes
/// in the sample table at `self->Instance+0x80`), it evaluates the four
/// cubic basis rows on VU0 (`0x31be50` gives the segment weights) and writes the
/// per-sample matrices to the three output buffers `T5`, `T4`, `T3`, `T1`,
/// `T2` (generated register roles kept as named). The `-1` index entries are
/// skipped (the second, shorter path).
pub(crate) fn floating_build_matrices_body(c: &mut Cpu) -> Flow {
    c.cycles += 93;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x270));
    c.sq(16, sp(c).wrapping_add(0x260));
    c.sq(17, sp(c).wrapping_add(0x250));
    c.g[S0][0] = c.g[A0][0];
    c.sq(18, sp(c).wrapping_add(0x240));
    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
    c.sq(19, sp(c).wrapping_add(0x230));
    c.g[S2][0] = c.g[A1][0];
    c.sd(sp(c).wrapping_add(0x220), c.g[RA][0]);
    c.g[S1][0] = c.g[A2][0];
    c.g[A0][0] = sx(sp(c).wrapping_add(0x200));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x204));
    c.lq(2, oc604_ea(c.g[S2][0], 0x30));
    c.lwc1(1, oc604_ea(c.g[S0][0], 0x50));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.lwc1(0, sp(c).wrapping_add(0x48));
    c.lwc1(2, oc604_ea(c.g[V1][0], 0x28));
    c.sub_s(1, 1, 0);
    c.lwc1(3, oc604_ea(c.g[V1][0], 0x38));
    c.mov_s(12, 2);
    c.swc1(2, sp(c).wrapping_add(0x60));
    c.swc1(3, sp(c).wrapping_add(0x64));
    c.swc1(1, sp(c).wrapping_add(0x58));
    c.sw(sp(c).wrapping_add(0x50), 0u64);
    c.sw(sp(c).wrapping_add(0x54), 0u64);
    c.sw(sp(c).wrapping_add(0x68), 0u64);
    call_ptr(c, 0x0031_be50, 0x0034_f18c)?;
    c.lwc1(12, sp(c).wrapping_add(0x64));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x208));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20c));
    call_ptr(c, 0x0031_be50, 0x0034_f19c)?;
    c.lwc1(12, sp(c).wrapping_add(0x68));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x210));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x214));
    call_ptr(c, 0x0031_be50, 0x0034_f1ac)?;

    // Segment weights and the first basis rows (scalar FPU).
    c.lwc1(0, sp(c).wrapping_add(0x200));
    c.lwc1(2, sp(c).wrapping_add(0x208));
    c.lwc1(7, sp(c).wrapping_add(0x204));
    c.neg_s(13, 0);
    c.lwc1(10, sp(c).wrapping_add(0x210));
    c.mul_s(8, 0, 2);
    c.lwc1(9, sp(c).wrapping_add(0x214));
    c.mul_s(3, 7, 2);
    c.mul_s(16, 7, 10);
    c.lwc1(1, sp(c).wrapping_add(0x50));
    c.lwc1(4, sp(c).wrapping_add(0x20c));
    c.mul_s(14, 0, 9);
    c.swc1(1, sp(c).wrapping_add(0x30));
    c.mul_s(15, 8, 10);
    c.neg_s(6, 4);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 17);
    c.mul_s(1, 7, 9);
    c.lwc1(11, sp(c).wrapping_add(0x54));
    c.mul_s(5, 16, 2);
    c.lwc1(12, sp(c).wrapping_add(0x58));
    c.mul_s(0, 0, 10);
    c.swc1(11, sp(c).wrapping_add(0x34));
    c.mul_s(3, 3, 9);
    c.swc1(12, sp(c).wrapping_add(0x38));
    c.mul_s(8, 8, 9);
    c.swc1(2, sp(c).wrapping_add(0x8));
    c.mul_s(7, 7, 4);
    c.sw(sp(c).wrapping_add(0xc), 0u64);
    c.mul_s(13, 13, 4);
    c.sw(sp(c).wrapping_add(0x1c), 0u64);
    c.sub_s(0, 0, 3);
    c.sw(sp(c).wrapping_add(0x2c), 0u64);
    c.add_s(5, 5, 14);
    c.swc1(7, sp(c).wrapping_add(0x28));
    c.sub_s(1, 1, 15);
    c.swc1(13, sp(c).wrapping_add(0x18));
    c.mul_s(6, 6, 10);
    c.swc1(0, sp(c).wrapping_add(0x20));
    c.add_s(8, 8, 16);
    c.swc1(5, sp(c).wrapping_add(0x24));
    c.mul_s(4, 4, 9);
    c.swc1(1, sp(c).wrapping_add(0x14));
    c.swc1(6, sp(c).wrapping_add(0x4));
    c.swc1(8, sp(c).wrapping_add(0x10));
    c.swc1(4, sp(c));
    c.swc1(17, sp(c).wrapping_add(0x3c));
    c.g[V0][0] = c.lw(oc604_ea(c.g[S0][0], 0x18));
    c.g[T6][0] = c.lw(oc604_ea(c.g[V0][0], 0x80));
    c.lwc1(0, oc604_ea(c.g[V0][0], 0x84));
    c.g[A2][0] = c.lw(oc604_ea(c.g[T6][0], 0x8));
    c.swc1(0, sp(c).wrapping_add(0x78));
    c.swc1(17, sp(c).wrapping_add(0x7c));
    c.swc1(0, sp(c).wrapping_add(0x70));
    c.swc1(0, sp(c).wrapping_add(0x74));
    c.g[V0][0] = c.lw(oc604_ea(c.g[T6][0], 0x4));
    let t = (c.g[V0][0] as i64) <= 0;
    c.g[T7][0] = 0;
    if !t {
        c.cycles += 11;
        c.g[T0][0] = c.g[S1][0];
        c.g[S3][0] = u64::MAX;
        c.g[A3][0] = sx(sp(c).wrapping_add(0x80));
        c.g[T5][0] = sx(sp(c).wrapping_add(0x120));
        c.g[T2][0] = sx(sp(c).wrapping_add(0xc0));
        c.g[T4][0] = sx(sp(c).wrapping_add(0x100));
        c.g[T9][0] = c.g[SP][0];
        c.g[T3][0] = sx(sp(c).wrapping_add(0x1c0));
        c.g[T8][0] = sx(sp(c).wrapping_add(0x160));
        c.g[T1][0] = c.g[T0][0];
        loop {
            c.cycles += 3;
            c.g[A1][0] = c.lw(c.g[A2][0] as u32);
            let skip = c.g[A1][0] == c.g[S3][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
            if !skip {
                // A sample with a real index: blend the weights (in f0..f3),
                // then the three basis products into the T5/A3/T0 buffers.
                c.cycles += 77;
                c.g[A0][0] = c.lw(oc604_ea(c.g[A2][0], 0xc));
                c.lwc1(7, sp(c).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.lq(2, c.g[A0][0] as u32);
                c.lwc1(6, sp(c).wrapping_add(0x74));
                c.sq(2, sp(c).wrapping_add(0x100));
                c.sq(2, sp(c).wrapping_add(0xc0));
                c.lwc1(5, sp(c).wrapping_add(0x78));
                c.lq(2, oc604_ea(c.g[A0][0], 0x10));
                c.lwc1(4, sp(c).wrapping_add(0x7c));
                c.sq(2, sp(c).wrapping_add(0x100));
                c.sq(2, sp(c).wrapping_add(0xd0));
                c.lq(3, oc604_ea(c.g[A0][0], 0x20));
                c.sq(3, sp(c).wrapping_add(0x100));
                c.sq(3, sp(c).wrapping_add(0xe0));
                c.lq(2, oc604_ea(c.g[A0][0], 0x30));
                c.sq(2, sp(c).wrapping_add(0x110));
                c.lwc1(1, sp(c).wrapping_add(0x110));
                c.lwc1(2, sp(c).wrapping_add(0x114));
                c.lwc1(3, sp(c).wrapping_add(0x118));
                c.mul_s(1, 1, 7);
                c.lwc1(0, sp(c).wrapping_add(0x11c));
                c.mul_s(2, 2, 6);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, sp(c).wrapping_add(0x100));
                c.swc1(2, sp(c).wrapping_add(0x104));
                c.swc1(3, sp(c).wrapping_add(0x108));
                c.swc1(0, sp(c).wrapping_add(0x10c));
                c.lq(2, sp(c).wrapping_add(0x100));
                c.sq(2, sp(c).wrapping_add(0xf0));
                oc604_quad_load(c, 4, A1);
                oc604_quad_load(c, 8, T2);
                oc604_mat4_columns(c);
                oc604_quad_store(c, 12, T5);
                oc604_quad_load(c, 1, T5);
                oc604_quad_store(c, 1, A3);
                oc604_quad_load(c, 1, A3);
                oc604_quad_store(c, 1, T0);
                c.g[V0][0] = c.lw(oc604_ea(c.g[T6][0], 0x4));
            } else {
                // The index is -1: the same products from the other buffers.
                c.cycles += 111;
                oc604_quad_load(c, 4, S2);
                oc604_quad_load(c, 8, T9);
                oc604_mat4_columns(c);
                oc604_quad_store(c, 12, T4);
                oc604_quad_load(c, 1, T4);
                oc604_quad_store(c, 1, T2);
                c.g[A0][0] = c.lw(oc604_ea(c.g[A2][0], 0xc));
                c.lwc1(6, sp(c).wrapping_add(0x70));
                c.lq(2, c.g[A0][0] as u32);
                c.lwc1(7, sp(c).wrapping_add(0x74));
                c.sq(2, sp(c).wrapping_add(0x1a0));
                c.sq(2, sp(c).wrapping_add(0x160));
                c.lwc1(5, sp(c).wrapping_add(0x78));
                c.lq(2, oc604_ea(c.g[A0][0], 0x10));
                c.lwc1(4, sp(c).wrapping_add(0x7c));
                c.sq(2, sp(c).wrapping_add(0x1a0));
                c.sq(2, sp(c).wrapping_add(0x170));
                c.lq(3, oc604_ea(c.g[A0][0], 0x20));
                c.sq(3, sp(c).wrapping_add(0x1a0));
                c.sq(3, sp(c).wrapping_add(0x180));
                c.lq(2, oc604_ea(c.g[A0][0], 0x30));
                c.sq(2, sp(c).wrapping_add(0x1b0));
                c.lwc1(1, sp(c).wrapping_add(0x1b0));
                c.lwc1(2, sp(c).wrapping_add(0x1b4));
                c.lwc1(3, sp(c).wrapping_add(0x1b8));
                c.mul_s(1, 1, 6);
                c.lwc1(0, sp(c).wrapping_add(0x1bc));
                c.mul_s(2, 2, 7);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, sp(c).wrapping_add(0x1a0));
                c.swc1(2, sp(c).wrapping_add(0x1a4));
                c.swc1(3, sp(c).wrapping_add(0x1a8));
                c.swc1(0, sp(c).wrapping_add(0x1ac));
                c.lq(2, sp(c).wrapping_add(0x1a0));
                c.sq(2, sp(c).wrapping_add(0x190));
                oc604_quad_load(c, 4, T2);
                oc604_quad_load(c, 8, T8);
                oc604_mat4_columns(c);
                oc604_quad_store(c, 12, T3);
                oc604_quad_load(c, 1, T3);
                oc604_quad_store(c, 1, A3);
                oc604_quad_load(c, 1, A3);
                oc604_quad_store(c, 1, T1);
                c.g[V0][0] = c.lw(oc604_ea(c.g[T6][0], 0x4));
            }
            c.cycles += 6;
            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(1));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = ((c.g[T7][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_f2c0);
            }
        }
    }
    c.cycles += 10;
    c.g[V0][0] = c.lhu(oc604_ea(c.g[S0][0], 0x12));
    c.lq(17, sp(c).wrapping_add(0x250));
    c.g[V0][0] &= 0xfffe;
    c.lq(18, sp(c).wrapping_add(0x240));
    c.sh(oc604_ea(c.g[S0][0], 0x12), c.g[V0][0]);
    c.lq(16, sp(c).wrapping_add(0x260));
    c.lq(19, sp(c).wrapping_add(0x230));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x220));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x270));
    c.pc = t;
    Ok(())
}

/// `Floating_BuildMatrices` (0x34f120), see [`floating_build_matrices_body`].
pub fn floating_node_build_matrices(c: &mut Cpu) {
    guest(c, floating_build_matrices_body)
}

// ---------------------------------------------------------------------------
// cFloatingNode::applyForce (0x34f600), `Floating_v43`
// ---------------------------------------------------------------------------

/// `void cFloatingNode::applyForce(const tIntersectInfo&, const t4Vector&)`
/// (0x34f600): the contact arm is the intersection point minus the node
/// position (`transform` through the vtable slot at +0xc0/+0xc4); the force
/// adds to `Force` (+0x44) and the torque (arm x force) projected on the two
/// rotated axes adds to `TX` (+0x60) and `TY` (+0x70).
pub(crate) fn floating_apply_force_body(c: &mut Cpu) -> Flow {
    c.cycles += 99;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x90));
    c.sq(16, sp(c).wrapping_add(0x80));
    c.sq(17, sp(c).wrapping_add(0x70));
    c.g[S0][0] = c.g[A0][0];
    c.sq(18, sp(c).wrapping_add(0x60));
    c.g[S1][0] = c.g[A1][0];
    c.sd(sp(c).wrapping_add(0x50), c.g[RA][0]);
    c.g[S2][0] = c.g[A2][0];
    c.g[V1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
    c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0xc0));
    c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0xc4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0034_f634)?;
    c.lq(3, oc604_ea(c.g[V0][0], 0x30));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.lqc2(3, c.g[S1][0] as u32);
    c.lqc2(4, sp(c).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x30));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x44));
    c.lqc2(3, c.g[S2][0] as u32);
    c.lqc2(5, oc604_ea(c.g[S1][0], 0x10));
    oc604_axis_pass(c);
    c.qmfc2(3, 4);
    c.mtc1(3, 1);
    c.lq(2, sp(c).wrapping_add(0x30));
    c.add_s(0, 0, 1);
    c.sq(2, sp(c).wrapping_add(0x20));
    c.swc1(0, oc604_ea(c.g[S0][0], 0x44));
    c.lqc2(4, c.g[S2][0] as u32);
    c.lqc2(5, sp(c) as u32);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, sp(c).wrapping_add(0x40));
    c.lq(2, sp(c).wrapping_add(0x40));
    c.lwc1(1, oc604_ea(c.g[S0][0], 0x60));
    c.sq(2, sp(c).wrapping_add(0x30));
    c.lqc2(3, sp(c).wrapping_add(0x30));
    c.lqc2(5, sp(c).wrapping_add(0x10));
    oc604_axis_pass(c);
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    c.add_s(1, 1, 0);
    c.swc1(1, oc604_ea(c.g[S0][0], 0x60));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x70));
    c.lqc2(3, sp(c).wrapping_add(0x30));
    c.lqc2(5, sp(c).wrapping_add(0x20));
    oc604_axis_pass(c);
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.lq(17, sp(c).wrapping_add(0x70));
    c.add_s(0, 0, 1);
    c.lq(18, sp(c).wrapping_add(0x60));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x50));
    c.swc1(0, oc604_ea(c.g[S0][0], 0x70));
    c.lq(16, sp(c).wrapping_add(0x80));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
    c.pc = t;
    Ok(())
}

/// `cFloatingNode::applyForce` (0x34f600), see [`floating_apply_force_body`].
pub fn floating_node_apply_force(c: &mut Cpu) {
    guest(c, floating_apply_force_body)
}

// ---------------------------------------------------------------------------
// cFloatingNode::updateIntersectInfo (0x34f790), `Floating_v42`
// ---------------------------------------------------------------------------

/// `void cFloatingNode::updateIntersectInfo(tIntersectInfo&)` (0x34f790): the
/// base class fills `intInfo` first (`cMoveNode::updateIntersectInfo`,
/// 0x356ae0); then the node adds its own contribution: the arm from the
/// node position, the angular velocity from the two rotated axes scaled by
/// the spring rates (`WX`/`WY` at +0x40/+0x44 of the spring state), and the
/// linear velocity (`Vel`) with `arm x angVel`.
pub(crate) fn floating_update_intersect_info_body(c: &mut Cpu) -> Flow {
    c.cycles += 103;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xb0));
    c.sq(16, sp(c).wrapping_add(0xa0));
    c.sq(17, sp(c).wrapping_add(0x90));
    c.g[S0][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x80), c.g[RA][0]);
    c.g[S1][0] = c.g[A1][0];
    call_ptr(c, 0x0035_6ae0, 0x0034_f7ac)?;
    c.g[V1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
    c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0xc0));
    c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0xc4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0034_f7c0)?;
    c.lq(3, oc604_ea(c.g[V0][0], 0x30));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.lqc2(3, c.g[S1][0] as u32);
    c.lqc2(4, sp(c).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x30));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x30));
    c.mfc1(3, 0);
    c.lqc2(4, sp(c).wrapping_add(0x10));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x50));
    c.lq(2, sp(c).wrapping_add(0x50));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x40));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.mfc1(3, 0);
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x60));
    c.lq(2, sp(c).wrapping_add(0x60));
    c.sq(2, sp(c).wrapping_add(0x50));
    c.lqc2(3, sp(c).wrapping_add(0x40));
    c.lqc2(4, sp(c).wrapping_add(0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x60));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x1c));
    c.lq(2, sp(c).wrapping_add(0x60));
    c.swc1(0, sp(c).wrapping_add(0x58));
    c.sq(2, sp(c).wrapping_add(0x30));
    c.sw(sp(c).wrapping_add(0x50), 0u64);
    c.sw(sp(c).wrapping_add(0x54), 0u64);
    c.sw(sp(c).wrapping_add(0x5c), 0u64);
    c.lqc2(4, sp(c));
    c.lqc2(5, sp(c).wrapping_add(0x30));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, sp(c).wrapping_add(0x70));
    c.lq(16, sp(c).wrapping_add(0xa0));
    c.lq(2, sp(c).wrapping_add(0x70));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x80));
    c.sq(2, sp(c).wrapping_add(0x60));
    c.lqc2(3, sp(c).wrapping_add(0x50));
    c.lqc2(4, sp(c).wrapping_add(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x70));
    c.lq(2, sp(c).wrapping_add(0x70));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.lqc2(3, oc604_ea(c.g[S1][0], 0x20));
    c.lqc2(4, sp(c).wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, oc604_ea(c.g[S1][0], 0x20));
    c.lqc2(3, oc604_ea(c.g[S1][0], 0x30));
    c.lqc2(4, sp(c).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, oc604_ea(c.g[S1][0], 0x30));
    c.lq(17, sp(c).wrapping_add(0x90));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
    c.pc = t;
    Ok(())
}

/// `cFloatingNode::updateIntersectInfo` (0x34f790), see
/// [`floating_update_intersect_info_body`].
pub fn floating_node_update_intersect_info(c: &mut Cpu) {
    guest(c, floating_update_intersect_info_body)
}

/// `Floating_v50(self)` (0x34ef40): the floating node's per-frame update. With
/// a second part (`self + 0x78`) it takes the sum and difference of two
/// virtual matrix rows (VU0 `sub` then `add`) into the part's first rows, then
/// `World_GetNash` and `SpatialHash_Update`. Without one it calls `ModObj_v50`.
pub fn floating_v50_2(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_34f038: {
            'b_34f02c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                let t = c.g[A0][0] == 0u64;

                if t {
                    break 'b_34f02c;
                } else {
                    c.cycles += 52;
                    c.lq(3, c.g[A0][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0034ef94);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0034ef94)?;
                    c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sq(2, c.g[SP][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0034efcc);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0034efcc)?;
                    c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                    c.lq(3, c.g[SP][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(3, c.g[A0][0] as u32);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0034f00c);
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x002d1be0, 0x0034f00c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0034f024);
                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x003291e0, 0x0034f024)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        break 'b_34f038;
                    } else {
                        break 'b_34f02c;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0034f034);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x003568b0,
                f: ee::s11::ModObj_v50,
            }
            .call(c, 0x0034f034)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_34f038;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::small_pointer;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS;
    pub(crate) use crate::game::object::animnode::tests::fb;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base;

    pub(crate) use crate::game::sound::soundsys::tests::wf;

    use ssx3_ee as ee;

    #[test]
    fn floating_free_bound_box_matches() {
        check(
            ssx3_ee::s11::f_0034ef08__generated,
            floating_free_bound_box,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34ef_08, 0);
                c.g[A0][0] = DATA as u64;
                let p = if seed % 3 == 0 {
                    0
                } else {
                    small_pointer(seed)
                };
                c.bus.write32(DATA + 0x78, p);
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn floating_v15_matches() {
        check(
            ee::s11::f_0034f930__generated,
            floating_v15,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_f930, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                for off in [
                    0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x5c, 0x60,
                    0x64, 0x6c, 0x70,
                ] {
                    w(c, DATA + off, r.edge_f32());
                }
                w(c, GAME_GP + 0x2a74, DATA + 0x900);
                w(c, DATA + 0x910, 24 + (seed % 6));
                w(c, GAME_GP - 0x2a98, fb(1.396_263_5));
                w(c, GAME_GP - 0x2a94, fb(-1.396_263_5));
            },
        );
    }

    #[test]
    fn floating_save_matches() {
        check(
            ee::s11::f_0034fa88__generated,
            floating_save,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x34fa88, 4);
                be_library(c);
                world(c);
                let this = DATA;
                let arch = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arch as u64;
                w(c, arch, DATA + 0x2000);
                slot(c, DATA + 0x2000, 8, 0, FAKE);
                w(c, this + 0x78, (seed % 3) as u32);
                w(c, this + 0x70, DATA + 0x900);
            },
        );
    }

    #[test]
    fn floating_v49_matches() {
        check(
            ee::s11::f_0034f048__generated,
            floating_v49,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x34f048, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                w(c, this + 0x78, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
                w(c, this + 0x18, DATA + 0x300);
                let info = DATA + 0x200;
                clear(c, info, 0x100);
                for k in 0..8u32 {
                    wf(c, info + 4 * k, 1.0 + k as f32);
                }
                let inst = DATA + 0x300;
                clear(c, inst, 0x100);
                for k in 0..8u32 {
                    wf(c, inst + 0x60 + 4 * k, 0.25 * k as f32);
                }
            },
        );
    }

    use crate::difftest::fill;

    /// `WorldObj_ctor`: lists 0..3 (each a process-list slot whose "pending"
    /// bit picks the tail at +8 or +0x28), kinds 6/0x10/0x16 (no focus) or
    /// others (counters in the typed-process table), nodes already focused
    /// (flag 0x100) or not; the real SyncProcess_ctor chain, LinkInstance
    /// and World_AddFocusObject (FocusMan_AddObject on a focus manager
    /// reached through the world at gp-0x848).
    #[test]
    fn world_obj_ctor_matches() {
        use crate::difftest::{check, GAME_GP};
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(
            ee::s11::f_0034fb00__generated,
            world_obj_ctor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_fb00, 0);
                c.pc = 0x0034_fb00;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                fill(c, &mut r, SELF, 0x10);
                fill(c, &mut r, NODE, 0x10);
                let flags = r.next_u32();
                w(
                    c,
                    NODE + WOBJ_NODE_FLAGS,
                    if seed % 4 == 0 {
                        flags | 0x100
                    } else {
                        flags & !0x100
                    },
                );
                let list = seed % 4;
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                for i in 0..4 {
                    w(c, PROCS + 0x44 * i, r.next_u32());
                    w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                    w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
                }
                let kind = match (seed / 4) % 6 {
                    0 => 6,
                    1 => 0x10,
                    2 => 0x16,
                    _ => r.next_u32() % 0x20,
                };
                fill(c, &mut r, 0x0044_aff0, 0x20);
                // World -> game -> race -> focus manager.
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, WORLD + 0x100);
                w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
                w(c, WORLD + 0x200 + 0xa4, FOCUS);
                w(
                    c,
                    FOCUS + 0xd4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                w(c, FOCUS + 0xd8, r.next_u32() % 8);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(kind);
                c.g[A3][0] = sx(NODE);
            },
        );
    }

    use crate::difftest::scramble;
    use crate::difftest::Rng;

    /// The dlmalloc arena the bound-box allocation falls back to (the gp+0xdd8
    /// pool switch stays 0), initialised as `rbc_test_allocator` does.
    pub(crate) fn floating_test_heap(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
    }

    #[test]
    fn floating_alloc_bound_box_matches() {
        check(
            ee::s11::f_0034eeb8__generated,
            floating_alloc_bound_box,
            4,
            &|c, seed| {
                let mut r = Rng(0x0034_eeb8 ^ seed as u64);
                scramble(c, &mut r);
                floating_test_heap(c);
                c.g[A0][0] = (DATA + 0x1000 + 0x100 * (seed % 4)) as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0034_eeb8;
            },
        );
    }

    /// A leaf that returns 0 (`0x1406c0`): the vtable target of the fixtures.
    pub(crate) const LEAF: u32 = 0x0014_06c0;

    #[test]
    fn floating_v43_matches() {
        check(
            ee::s11::f_0034f600__generated,
            floating_node_apply_force,
            200,
            &|c, seed| {
                let mut r = Rng(0x43 ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.g[A2][0] = sx(DATA + 0x1200);
                c.bus.write32(node + 0xc, block);
                c.bus.write16(block + 0xc0, 0);
                c.bus.write32(block + 0xc4, LEAF);
                for k in 0..0x180 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0x44, (seed as f32 * 0.5).to_bits());
                c.bus.write32(node + 0x60, r.next_u32());
                c.bus.write32(node + 0x70, r.next_u32());
            },
        );
    }

    #[test]
    fn floating_v42_matches() {
        check(
            ee::s11::f_0034f790__generated,
            floating_node_update_intersect_info,
            200,
            &|c, seed| {
                let mut r = Rng(0x42 ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.bus.write32(node + 0xc, block);
                c.bus.write16(block + 0xc0, 0);
                c.bus.write32(block + 0xc4, LEAF);
                for k in 0..0x180 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0x30, (seed as f32).to_bits());
                c.bus.write32(node + 0x40, r.next_u32());
                c.bus.write32(node + 0x1c, r.next_u32());
                c.bus.write32(node + 0x18, (seed as f32).to_bits());
            },
        );
    }

    #[test]
    fn floating_build_matrices_matches() {
        check(
            ee::s11::f_0034f120__generated,
            floating_node_build_matrices,
            200,
            &|c, seed| {
                let mut r = Rng(0x120 ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                let inst = DATA + 0x300;
                let samples = DATA + 0x400;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1400);
                c.bus.write32(node + 0x18, inst);
                c.bus.write32(inst + 0x80, samples);
                c.bus.write32(inst + 0x84, (seed as f32).to_bits());
                c.bus
                    .write32(samples + 0x4, if seed % 3 == 0 { 0 } else { 2 });
                c.bus.write32(samples + 0x8, DATA + 0x500);
                c.bus
                    .write32(DATA + 0x500, if seed % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(DATA + 0x500 + 0xc, DATA + 0x600);
                for k in 0..0x200 {
                    c.bus.write32(DATA + 0x600 + 4 * k, r.next_u32());
                }
                for k in 0..0x100 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, r.next_u32());
                }
                c.bus.write16(node + 0x12, 0xffff);
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::object::flexrailnode::tests::FIX;

    pub(crate) use crate::game::object::flexrailnode::tests::oc687_gate;
    pub(crate) use crate::game::object::flexrailnode::tests::oc687_install_fakes;

    /// `Floating_v50`: no second part (`ModObj_v50` instead), or a part: the
    /// two virtual matrix getters, the VU0 `sub` / `add` of their rows, the
    /// nash lookup and the spatial update.
    #[test]
    fn floating_v50_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0034ef40__generated,
            floating_v50_2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34_ef40, 4);
                oc687_gate(c);
                c.pc = 0x0034_ef40;
                let (me, owner, part) = (FIX, FIX + 0x1000, FIX + 0x2000);
                c.bus.write32(me + 0xc, owner);
                c.bus.write32(me + 0x18, r.next_u32());
                c.bus
                    .write32(me + 0x78, if seed % 3 == 0 { 0 } else { part });
                // The owner's two virtual matrix getters share one vtable word
                // pair at +0xc0 (`{delta, fn}`).
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(owner + 0xc0, delta as u16);
                c.bus.write32(owner + 0xc4, fakes::FAKE);
                fill(c, &mut r, part, 0x10);
                for i in 0..12 {
                    c.bus.write32(part + 0x20 + 4 * i, r.f32(4.0));
                }
                c.g[A0][0] = me as u64;
            },
        );
    }
}
