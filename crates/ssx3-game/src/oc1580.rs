//! Staging module oc1580 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1580`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_0920, exec_man_base_dtor, "ExecManBase_dtor"),
    (0x0032_0c18, input_binding_eval_float_args, "InputBinding_EvalFloatArgs"),
    (0x0032_0c70, input_binding_eval_bool_args, "InputBinding_EvalBoolArgs"),
    (0x0032_5f44, func_00325f44, "func_00325F44"),
    (0x0032_c9b8, sphere_tree_collide_plane, "SphereTree_CollidePlane"),
    (0x0034_a838, c_spring_rail_node_update_intersect_info, "cSpringRailNode_updateIntersectInfo"),
    (0x0036_a048, c_graphics_man_render_spline, "cGraphicsMan_RenderSpline"),
    (0x0036_a3e0, morton_encode_8x8, "Morton_Encode8x8"),
];

/// `ExecManBase::~ExecManBase(this, flags)` (0x320920): stores the vtable
/// word (0x48dc30) at `this + 8`, then deletes the object when bit 0 of
/// `flags` is set.
pub fn exec_man_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x23d0));
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let delete_object = c.g[A1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        if delete_object {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::__builtin_delete,
            }
            .call(c, 0x0032_0944)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InputBinding_EvalFloatArgs(binding, a, b)` (0x320c18): calls
/// `InputMap_EvalFloatArgs(binding[1], binding[0], a, b)`, the binding's
/// words at +4 and +0 first.
pub fn input_binding_eval_float_args(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let binding = c.g[A0][0] as u32;
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(binding).lw(c);
        c.g[A0][0] = Addr(binding.wrapping_add(4)).lw(c);
        Callee {
            addr: 0x0032_1010,
            f: ee::s10::InputMap_EvalFloatArgs,
        }
        .call(c, 0x0032_0c38)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InputBinding_EvalBoolArgs(binding, a, b)` (0x320c70): the boolean twin
/// of [`input_binding_eval_float_args`], calling `InputMap_EvalBoolArgs`.
pub fn input_binding_eval_bool_args(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let binding = c.g[A0][0] as u32;
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(binding).lw(c);
        c.g[A0][0] = Addr(binding.wrapping_add(4)).lw(c);
        Callee {
            addr: 0x0032_1188,
            f: ee::s10::InputMap_EvalBoolArgs,
        }
        .call(c, 0x0032_0c90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The 4-byte stub at 0x325f44: a tail jump into `InputPreproc_Run`
/// (0x325f48), which returns straight to the stub's caller.
pub fn func_00325f44(c: &mut Cpu) {
    c.cycles += 1;
    c.pc = 0x0032_5f48;
    ee::s10::InputPreproc_Run(c);
}

/// `SphereTree_CollidePlane(self, plane, a2, a3)` (0x32c9b8): allocates the
/// bit tree of a compressed tree (`tree + 8` set), sorts the tree along the
/// plane with w cleared, clears `worstCase` (+0x94), runs the plane collision
/// and returns whether `worstCase` came out negative.
pub fn sphere_tree_collide_plane(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[S3][0] = c.g[A3][0];
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0032_df28, 0x0032_c9f4)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        }
        c.cycles += 23;
        c.lq(2, c.g[S1][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        // The plane copied to the stack with its w word cleared.
        c.sq(2, sp(c).wrapping_add(0x10));
        c.sw(sp(c).wrapping_add(0x1c), 0u64);
        c.lq(2, sp(c).wrapping_add(0x10));
        c.sq(2, sp(c));
        call_ptr(c, 0x0032_db40, 0x0032_ca1c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x94), 0u64);
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        call_ptr(c, 0x0032_d208, 0x0032_ca40)?;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x94));
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let negative = c.fcc();
        c.g[V0][0] = 0x1;
        if !negative {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `cSpringRailNode_updateIntersectInfo(self, node)` (0x34a838): unless
/// `node->Seg` is negative, adds for each rail node `i <= Seg` the cross
/// product of the node's offset `Ipos - Nodes[i].P` with `Nodes[i].W` onto
/// the node's velocity `Ivel` (+0x20). VU0 arithmetic as the generated code.
pub fn c_spring_rail_node_update_intersect_info(c: &mut Cpu) {
    'b_34a8bc: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        if t {
            break 'b_34a8bc;
        }
        c.cycles += 3;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x70));
        'l_34a850: loop {
            c.cycles += 27;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[A1][0] as u32);
            c.lqc2(4, c.g[V0][0] as u32);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x70));
            c.lq(3, sp(c).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, sp(c).wrapping_add(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp(c).wrapping_add(0x10));
            c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, sp(c).wrapping_add(0x20));
            c.lq(2, sp(c).wrapping_add(0x20));
            c.sq(2, sp(c));
            c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.lqc2(4, sp(c));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x70));
                if event_due(c) {
                    deliver_event_here(c, 0x0034_a850);
                }
                continue 'l_34a850;
            } else {
                break 'b_34a8bc;
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `cGraphicsMan_RenderSpline(self, BezMat, color, n)` (0x36a048): samples
/// the cubic Bezier `BezMat` at `n` evenly spaced parameters into `n` line
/// strip vertices (colour from `color` times 128, position from the VU0
/// matrix product) and passes them to the vtable's `RenderLineStrip`
/// (`self->vtable + 0x248`, `+0x24c`).
pub fn c_graphics_man_render_spline(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        'b_36a164: {
            c.cycles += 34;
            c.mtc1(7, 4);
            c.cvt_s_w(4, 4);
            frame = Frame::open(c, 0x410);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 6);
            c.g[T5][0] = c.g[A0][0];
            frame.save_ra(c, 0x400);
            c.g[V0][0] = 0x13;
            c.sub_s(4, 4, 6);
            c.g[AT][0] = 0x4300_0000;
            c.mtc1(1, 0);
            c.lwc1(5, c.g[A2][0] as u32);
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.mul_s(5, 5, 0);
            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
            c.mul_s(1, 1, 0);
            c.div_s(6, 6, 4);
            c.cvt_w_s(0, 2);
            c.mfc1(12, 0);
            c.cvt_w_s(0, 3);
            c.mfc1(11, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(10, 0);
            c.cvt_w_s(0, 5);
            c.mfc1(9, 0);
            c.mtc1(0, 2);
            let t = (c.g[A3][0] as i64) <= 0;
            c.g[T0][0] = sx(sp(c).wrapping_add(0x30));
            if t {
                break 'b_36a164;
            }
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 3);
            c.g[V1][0] = c.g[A2][0];
            c.g[A0][0] = c.g[A3][0];
            'l_36a0e0: loop {
                c.cycles += 33;
                c.mul_s(1, 2, 2);
                c.swc1(2, sp(c).wrapping_add(0x3e8));
                c.swc1(3, sp(c).wrapping_add(0x3ec));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.mul_s(0, 2, 1);
                c.swc1(1, sp(c).wrapping_add(0x3e4));
                c.add_s(2, 2, 6);
                c.swc1(0, sp(c).wrapping_add(0x3e0));
                c.lqc2(8, sp(c).wrapping_add(0x3e0));
                c.lqc2(4, c.g[A1][0] as u32);
                c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp(c).wrapping_add(0x3f0));
                c.lq(2, sp(c).wrapping_add(0x3f0));
                c.sq(2, sp(c));
                c.sq(2, sp(c).wrapping_add(0x3d0));
                c.sw(c.g[V1][0] as u32, 0u64);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[T3][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                c.lq(2, sp(c));
                c.sq(2, c.g[T0][0] as u32);
                let more = c.g[A0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_a0e0);
                    }
                    continue 'l_36a0e0;
                } else {
                    break 'b_36a164;
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x10d8));
        c.g[A1][0] = c.g[A3][0];
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x248));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0036_a180)?;
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    });
}

/// `Morton_Encode8x8(even, odd)` (0x36a3e0): the 16-bit `even` spread to the
/// even bits and `odd` to the odd bits, `Morton_Spread8(even) | Morton_Spread8(odd) << 1`.
pub fn morton_encode_8x8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = c.g[A0][0] & 0xffff;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0] & 0xffff;
        Callee {
            addr: 0x0036_a3b0,
            f: ee::s12::Morton_Spread8,
        }
        .call(c, 0x0036_a3fc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0036_a3b0,
            f: ee::s12::Morton_Spread8,
        }
        .call(c, 0x0036_a408)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[S1][0] | c.g[V0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words of the fakes below: one per fake, each in the machine's
    /// own RAM (beyond the fakes' call log), armed with `OC1580_MAGIC`.
    const OC1580_MAGIC: u32 = 0x1580_0001;
    const OC1580_DELETE_CTRL: u32 = DATA + 0x2f_0000;
    const OC1580_INPUT_FLOAT_CTRL: u32 = DATA + 0x2f_1000;
    const OC1580_INPUT_BOOL_CTRL: u32 = DATA + 0x2f_2000;
    const OC1580_PREPROC_CTRL: u32 = DATA + 0x2f_3000;
    const OC1580_ALLOC_CTRL: u32 = DATA + 0x2f_4000;
    const OC1580_POPULATE_CTRL: u32 = DATA + 0x2f_5000;
    const OC1580_PLANE_CTRL: u32 = DATA + 0x2f_6000;

    static OC1580_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC1580_PREV_INPUT_FLOAT: AtomicUsize = AtomicUsize::new(0);
    static OC1580_PREV_INPUT_BOOL: AtomicUsize = AtomicUsize::new(0);
    static OC1580_PREV_PREPROC: AtomicUsize = AtomicUsize::new(0);
    static OC1580_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static OC1580_PREV_POPULATE: AtomicUsize = AtomicUsize::new(0);
    static OC1580_PREV_PLANE: AtomicUsize = AtomicUsize::new(0);

    /// `__builtin_delete`: a logged no-op call.
    fn oc1580_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1580_DELETE_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `InputMap_EvalFloatArgs`: a logged no-op call.
    fn oc1580_fake_input_float(c: &mut Cpu) {
        if c.bus.read32(OC1580_INPUT_FLOAT_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_INPUT_FLOAT, ee::s10::f_00321010__generated);
        }
        fakes::fake_call(c, 0x0032_1010);
    }

    /// `InputMap_EvalBoolArgs`: a logged no-op call.
    fn oc1580_fake_input_bool(c: &mut Cpu) {
        if c.bus.read32(OC1580_INPUT_BOOL_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_INPUT_BOOL, ee::s10::f_00321188__generated);
        }
        fakes::fake_call(c, 0x0032_1188);
    }

    /// `InputPreproc_Run`: a logged no-op call.
    fn oc1580_fake_preproc(c: &mut Cpu) {
        if c.bus.read32(OC1580_PREPROC_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_PREPROC, ee::s10::f_00325f48__generated);
        }
        fakes::fake_call(c, 0x0032_5f48);
    }

    /// `SphereTree_AllocBuf`: returns the buffer pointer in the mailbox at +4.
    fn oc1580_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC1580_ALLOC_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_ALLOC, ee::s11::f_0032df28__generated);
        }
        c.cycles += 3;
        c.g[V0][0] = c.bus.read32(OC1580_ALLOC_CTRL + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// The populate-sort call (`0x32db40`): a logged no-op call.
    fn oc1580_fake_populate(c: &mut Cpu) {
        if c.bus.read32(OC1580_POPULATE_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_POPULATE, ee::s11::f_0032db40__generated);
        }
        fakes::fake_call(c, 0x0032_db40);
    }

    /// The plane collision (`0x32d208`): stores the mailbox word at +4 as
    /// `worstCase` of the tree at `s0`, then a logged no-op call.
    fn oc1580_fake_plane(c: &mut Cpu) {
        if c.bus.read32(OC1580_PLANE_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_PLANE, ee::s11::f_0032d208__generated);
        }
        let worst = (c.g[S0][0] as u32).wrapping_add(0x94);
        let v = c.bus.read32(OC1580_PLANE_CTRL + 4);
        c.bus.write32(worst, v);
        fakes::fake_call(c, 0x0032_d208);
    }

    fn oc1580_install_delete() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1580_fake_delete,
            &OC1580_PREV_DELETE,
        );
    }

    fn oc1580_install_input() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321010),
            oc1580_fake_input_float,
            &OC1580_PREV_INPUT_FLOAT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321188),
            oc1580_fake_input_bool,
            &OC1580_PREV_INPUT_BOOL,
        );
    }

    fn oc1580_install_preproc() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00325F48),
            oc1580_fake_preproc,
            &OC1580_PREV_PREPROC,
        );
    }

    fn oc1580_install_plane() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0032DF28),
            oc1580_fake_alloc,
            &OC1580_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0032DB40),
            oc1580_fake_populate,
            &OC1580_PREV_POPULATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0032D208),
            oc1580_fake_plane,
            &OC1580_PREV_PLANE,
        );
    }

    #[test]
    fn exec_man_base_dtor_matches() {
        check(
            ee::s10::f_00320920__generated,
            exec_man_base_dtor,
            200,
            &|c, seed| {
                oc1580_install_delete();
                let mut r = machine(c, seed, 0x0032_0920, 0);
                c.bus.write32(OC1580_DELETE_CTRL, OC1580_MAGIC);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] &= !1u64;
                } else {
                    c.g[A1][0] |= 1;
                }
            },
        );
    }

    #[test]
    fn input_binding_eval_float_args_matches() {
        check(
            ee::s10::f_00320c18__generated,
            input_binding_eval_float_args,
            200,
            &|c, seed| {
                oc1580_install_input();
                let mut r = machine(c, seed, 0x0032_0c18, 1);
                c.bus.write32(OC1580_INPUT_FLOAT_CTRL, OC1580_MAGIC);
                let binding = DATA + 0x1000;
                c.bus.write32(binding, r.next_u32());
                c.bus.write32(binding + 4, r.next_u32());
                c.g[A0][0] = binding as u64;
            },
        );
    }

    #[test]
    fn input_binding_eval_bool_args_matches() {
        check(
            ee::s10::f_00320c70__generated,
            input_binding_eval_bool_args,
            200,
            &|c, seed| {
                oc1580_install_input();
                let mut r = machine(c, seed, 0x0032_0c70, 1);
                c.bus.write32(OC1580_INPUT_BOOL_CTRL, OC1580_MAGIC);
                let binding = DATA + 0x1000;
                c.bus.write32(binding, r.next_u32());
                c.bus.write32(binding + 4, r.next_u32());
                c.g[A0][0] = binding as u64;
            },
        );
    }

    #[test]
    fn func_00325f44_matches() {
        check(
            ee::s10::f_00325f44__generated,
            func_00325f44,
            200,
            &|c, seed| {
                oc1580_install_preproc();
                let _r = machine(c, seed, 0x0032_5f44, 1);
                c.bus.write32(OC1580_PREPROC_CTRL, OC1580_MAGIC);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_plane_matches() {
        check(
            ee::s10::f_0032c9b8__generated,
            sphere_tree_collide_plane,
            300,
            &|c, seed| {
                oc1580_install_plane();
                let mut r = machine(c, seed, 0x0032_c9b8, 1);
                c.bus.write32(OC1580_ALLOC_CTRL, OC1580_MAGIC);
                c.bus.write32(OC1580_ALLOC_CTRL + 4, DATA + 0x5000);
                c.bus.write32(OC1580_POPULATE_CTRL, OC1580_MAGIC);
                c.bus.write32(OC1580_PLANE_CTRL, OC1580_MAGIC);
                c.bus.write32(OC1580_PLANE_CTRL + 4, r.edge_f32());
                let this = DATA + 0x1000;
                let tree = DATA + 0x2000;
                c.bus.write32(this + 0x98, tree);
                c.bus.write32(tree + 0x8, seed % 2);
                fill(c, &mut r, this + 0x80, 4);
                fill(c, &mut r, DATA + 0x3000, 4);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64;
                c.g[A2][0] = (DATA + 0x4000) as u64;
                c.g[A3][0] = (DATA + 0x4800) as u64;
            },
        );
    }

    #[test]
    fn c_spring_rail_node_update_intersect_info_matches() {
        check(
            ee::s11::f_0034a838__generated,
            c_spring_rail_node_update_intersect_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_a838, 0);
                let this = DATA + 0x1000;
                let nodes = DATA + 0x2000;
                let node = DATA + 0x3000;
                c.bus.write32(this + 0x70, nodes);
                fill(c, &mut r, nodes, 0x80);
                fill(c, &mut r, node, 0x20);
                let seg = (seed % 6) as i32 - 1;
                c.bus.write32(node + 0x64, seg as u32);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = node as u64;
            },
        );
    }

    #[test]
    fn c_graphics_man_render_spline_matches() {
        check(
            ee::s12::f_0036a048__generated,
            c_graphics_man_render_spline,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_a048, 1);
                let this = DATA + 0x1000;
                let table = DATA + 0x2000;
                let mat = DATA + 0x3000;
                let color = DATA + 0x3100;
                c.bus.write32(this + 0x10d8, table);
                c.bus.write16(table + 0x248, 0x10);
                c.bus.write32(table + 0x24c, 0x00AB_0000);
                fill(c, &mut r, mat, 16);
                for i in 0..4 {
                    c.bus.write32(color + 4 * i, r.edge_f32());
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = mat as u64;
                c.g[A2][0] = color as u64;
                c.g[A3][0] = ((seed % 6) as i64 - 1) as u64;
            },
        );
    }

    #[test]
    fn morton_encode_8x8_matches() {
        check(
            ee::s12::f_0036a3e0__generated,
            morton_encode_8x8,
            300,
            &|c, seed| {
                let mut r = Rng(0x0036_a3e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
