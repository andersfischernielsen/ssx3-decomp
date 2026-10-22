//! `src/object/spring.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
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
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::sp;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::r;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_3fc0, flex_bridge_springs_step, "FlexBridge_SpringsStep"),
    (0x0035_45d8, flex_bridge_springs_save, "FlexBridge_SpringsSave"),
];


// ---------------------------------------------------------------------------
// FlexBridge spring chain.

/// `a + off`.
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// Set a GPR to a sign-extended word.
#[inline(always)]
pub(crate) fn set_r(c: &mut Cpu, reg: usize, v: u32) {
    c.g[reg][0] = ssx3_ee_rt::cpu::sx(v);
}


/// `void FlexBridge_SpringsStep(FlexBridge* self)` (0x353fc0): one step of the
/// spring chain hanging off the bridge -- an implicit Euler integration of a
/// chain of point masses.
///
/// Layout: `self->n` at `+0`, `rest` +0x4, `scale` +0x8, `c` +0xc, `damp`
/// +0x1c, `k` +0x1c? (see the field constants below), and `self->nodes` at
/// `+0x24`, an array of `0x50`-byte nodes. The stack holds `F[16]` (quads at
/// `sp+0x10`) and `T[16]` (floats at `sp+0x110`).
pub fn flex_bridge_springs_step(c: &mut Cpu) {
    c.cycles += 18;
    // g = (0, 0, -980, 0) as the first float of the frame scratch.
    c.g[AT][0] = 0xffffffff_c475_0000;
    c.mtc1(1, 0);
    let frame_base = r(c, SP).wrapping_sub(0x1c0);
    c.g[SP][0] = sx(frame_base);
    let sp = r(c, SP);
    c.sw(sp, 0);
    c.g[A3][0] = c.g[A0][0];
    c.swc1(0, at(sp, 0x8));
    c.g[V1][0] = 0xf;
    c.sw(at(sp, 0x4), 0);
    c.g[T0][0] = ssx3_ee_rt::cpu::sx(at(sp, 0x10));
    c.sw(at(sp, 0xc), 0);
    c.g[T2][0] = ssx3_ee_rt::cpu::sx(at(sp, 0x110));
    // dt = 1 / (float)frame_rate->denominator
    let clock = c.lw(at(r(c, GP), 0x2a74));
    c.g[V0][0] = clock;
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.lwc1(0, at(r(c, V0), 0x10));
    c.cvt_s_w(0, 0);
    c.div_s(6, 1, 0);

    // A 16-step synchronisation delay (`for (v1 = 15; v1 != -1; v1--)`).
    loop {
        c.cycles += 7;
        let v = r(c, V1).wrapping_sub(1);
        set_r(c, V1, v);
        let go = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = 0x500000;
        if !go {
            break;
        }
        if c.cycles >= c.next_event {
            c.pc = 0x0035_4008;
            c.event();
        }
    }

    // F[0] = F[n-1] = the zero vector; T[0] = T[n-1] = 0.
    c.cycles += 18;
    let n = c.lw(r(c, A3));
    set_r(c, V1, n as u32);
    let zero_vec = r(c, V0).wrapping_sub(0xee0);
    set_r(c, V0, zero_vec);
    c.mtc1(0, 0);
    c.lq(6, zero_vec);
    let v = r(c, V1) << 4;
    c.g[A1][0] = sx(v);
    let last_slot = at(sp, r(c, A1));
    set_r(c, V0, last_slot);
    let v = r(c, V1).wrapping_sub(1);
    c.g[V1][0] = sx(v);
    c.sq(6, last_slot);
    let v = r(c, A1).wrapping_add(r(c, T0));
    c.g[A1][0] = sx(v);
    let v = r(c, V1) << 2;
    c.g[A0][0] = sx(v);
    c.g[T1][0] = 0x1;
    let prev_slot = r(c, A1).wrapping_sub(0x10);
    c.lq(2, prev_slot);
    let v = r(c, T2).wrapping_add(r(c, A0));
    c.g[A0][0] = sx(v);
    let v = ((r(c, T1) as i64) < (r(c, V1) as i64)) as u32;
    c.g[V1][0] = sx(v);
    c.sq(2, at(sp, 0x10));
    c.swc1(0, r(c, A0));
    let no_interior = c.g[V1][0] == 0;
    c.swc1(0, at(sp, 0x110));

    if !no_interior {
        // First pass: spring forces and torques for the interior nodes.
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 8);
        c.g[T0][0] = ssx3_ee_rt::cpu::sx(at(r(c, T2), 0x4));
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 7);
        c.g[A2][0] = ssx3_ee_rt::cpu::sx(at(sp, 0x10));
        c.g[A1][0] = 0x50;
        loop {
            c.cycles += 167;
            // d1 = nodes[i-1].p - nodes[i].p, d2 = nodes[i+1].p - nodes[i].p.
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            let v = r(c, T1).wrapping_add(1);
            c.g[T1][0] = sx(v);
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[A0][0] = sx(v);
            let v = r(c, A0);
            c.g[V1][0] = sx(v);
            let node = r(c, A0);
            let node_minus = r(c, V1).wrapping_sub(0x50);
            c.lqc2(3, node_minus);
            c.lqc2(4, node);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x160));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.lq(2, at(sp, 0x160));
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[A0][0] = sx(v);
            c.sq(2, at(sp, 0x150));
            let v = r(c, A0);
            c.g[V1][0] = sx(v);
            let node = r(c, A0);
            c.lqc2(3, at(r(c, V1), 0x50));
            c.lqc2(4, node);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x170));
            c.lwc1(0, at(r(c, A3), 0x20));
            c.lwc1(1, at(r(c, A3), 0x1c));
            c.lq(2, at(sp, 0x170));
            c.mul_s(1, 1, 0);
            c.sq(2, at(sp, 0x160));
            // d1 -= d1 * (rest*scale/|d1|)
            c.lqc2(3, at(sp, 0x150));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0);
            c.div_s(1, 1, 0);
            c.sub_s(1, 8, 1);
            c.mfc1(2, 1);
            c.lqc2(4, at(sp, 0x150));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x150));
            // d2 -= d2 * (rest*scale/|d2|)
            c.lwc1(1, at(r(c, A3), 0x20));
            c.lqc2(3, at(sp, 0x160));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 2);
            c.lwc1(0, at(r(c, A3), 0x1c));
            c.mul_s(0, 0, 1);
            c.div_s(0, 0, 2);
            c.sub_s(0, 8, 0);
            c.mfc1(2, 0);
            c.lqc2(4, at(sp, 0x160));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x160));
            // F[i] = k * (d1 + d2) + (dt*k) * (v[i-1] + v[i+1] - 2*v[i])
            c.lwc1(1, at(r(c, A3), 0x4));
            c.lqc2(3, at(sp, 0x150));
            c.lqc2(4, at(sp, 0x160));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            c.sq(2, at(sp, 0x180));
            c.mfc1(3, 1);
            c.lqc2(4, at(sp, 0x180));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.sq(2, at(sp, 0x170));
            c.sq(2, at(r(c, A2), 0x10));
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[A0][0] = sx(v);
            let v = r(c, A0);
            c.g[V1][0] = sx(v);
            c.lwc1(0, at(r(c, A3), 0x4));
            let node = r(c, A0);
            c.lqc2(3, r(c, V1).wrapping_sub(0x40));
            c.lqc2(4, at(node, 0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1a0));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.mul_s(0, 6, 0);
            c.lq(2, at(sp, 0x1a0));
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            c.sq(2, at(sp, 0x190));
            c.mfc1(4, 7);
            c.lqc2(4, at(r(c, V1), 0x10));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x1b0));
            c.lq(2, at(sp, 0x1b0));
            c.sq(2, at(sp, 0x1a0));
            c.lqc2(3, at(sp, 0x190));
            c.lqc2(4, at(sp, 0x1a0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1b0));
            c.lq(2, at(sp, 0x1b0));
            c.sq(2, at(sp, 0x180));
            c.mfc1(3, 0);
            c.lqc2(4, at(sp, 0x180));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x1b0));
            c.lq(2, at(sp, 0x1b0));
            c.sq(2, at(sp, 0x170));
            c.lqc2(3, at(r(c, A2), 0x10));
            c.lqc2(4, at(sp, 0x170));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(r(c, A2), 0x10));
            // T[i] = c * (a[i-1] - a[i] + a[i+1] - a[i])
            //      + dt*c * (av[i-1] + av[i+1] - 2*av[i])
            let nodes = c.lw(at(r(c, A3), 0x24));
            let v = r(c, A1).wrapping_add(nodes as u32);
            c.g[A0][0] = sx(v);
            c.lwc1(3, at(r(c, A3), 0xc));
            c.lwc1(4, at(r(c, A3), 0x8));
            let node = r(c, A0);
            c.lwc1(2, at(node, 0x20));
            c.mul_s(5, 6, 3);
            c.lwc1(0, node.wrapping_sub(0x30));
            c.neg_s(4, 4);
            c.lwc1(1, at(node, 0x70));
            c.sub_s(0, 0, 2);
            c.add_s(0, 0, 1);
            c.sub_s(0, 0, 2);
            c.mul_s(3, 3, 0);
            c.swc1(3, r(c, T0));
            c.lwc1(2, at(node, 0x74));
            c.lwc1(1, node.wrapping_sub(0x2c));
            c.lwc1(0, at(node, 0x24));
            c.add_s(1, 1, 2);
            c.add_s(0, 0, 0);
            c.sub_s(1, 1, 0);
            c.mul_s(5, 5, 1);
            c.add_s(3, 3, 5);
            c.swc1(3, r(c, T0));
            // F[i] += v[i] * -damp + g
            c.mfc1(3, 4);
            c.lqc2(4, at(node, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            c.sq(2, at(sp, 0x180));
            c.lqc2(3, at(sp, 0x180));
            c.lqc2(4, sp);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            c.sq(2, at(sp, 0x170));
            c.lqc2(3, at(r(c, A2), 0x10));
            c.lqc2(4, at(sp, 0x170));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(r(c, A2), 0x10));
            // T[i] -= e * av[i]
            c.lwc1(0, at(r(c, A3), 0x10));
            let v = r(c, A2).wrapping_add(0x10);
            c.g[A2][0] = sx(v);
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            c.neg_s(0, 0);
            c.lwc1(1, r(c, T0));
            let v = r(c, A1).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.lwc1(2, at(r(c, V0), 0x24));
            let v = r(c, A1).wrapping_add(0x50);
            c.g[A1][0] = sx(v);
            c.mul_s(0, 0, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, r(c, T0));
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            let v = r(c, V0).wrapping_sub(1);
            c.g[V0][0] = sx(v);
            let v = ((r(c, T1) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            let more = c.g[V0][0] != 0;
            let v = r(c, T0).wrapping_add(0x4);
            c.g[T0][0] = sx(v);
            if !more {
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_4088;
                c.event();
            }
        }
    }

    // Second pass: F[i] += nodes[i].f, T[i] += nodes[i].t.
    c.cycles += 6;
    let v = c.lw(r(c, A3)) as u32;
    c.g[V0][0] = sx(v);
    c.g[T0][0] = 0x1;
    let v = r(c, V0).wrapping_sub(1);
    c.g[V0][0] = sx(v);
    let v = ((r(c, T0) as i64) < (r(c, V0) as i64)) as u32;
    c.g[V0][0] = sx(v);
    let has_interior = c.g[V0][0] != 0;
    if !has_interior {
        let v = c.lw(r(c, A3)) as u32;
        c.g[V0][0] = sx(v);
    } else {
        c.cycles += 3;
        let v = at(r(c, T2), 0x4);
        c.g[A1][0] = sx(v);
        c.g[A2][0] = 0x50;
        set_r(c, A0, at(sp, 0x20));
        loop {
            c.cycles += 20;
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            let v = r(c, T0).wrapping_add(1);
            c.g[T0][0] = sx(v);
            let v = r(c, A2).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.lqc2(3, r(c, A0));
            let node = r(c, V0);
            c.lqc2(4, at(node, 0x30));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, A0));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            let v = r(c, A0).wrapping_add(0x10);
            c.g[A0][0] = sx(v);
            c.lwc1(0, r(c, A1));
            let v = r(c, A2).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            c.lwc1(1, at(r(c, V1), 0x40));
            let v = r(c, A2).wrapping_add(0x50);
            c.g[A2][0] = sx(v);
            let v = r(c, V0).wrapping_sub(1);
            c.g[V0][0] = sx(v);
            c.add_s(0, 0, 1);
            let v = ((r(c, T0) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            c.swc1(0, r(c, A1));
            let more = c.g[V0][0] != 0;
            let v = r(c, A1).wrapping_add(0x4);
            c.g[A1][0] = sx(v);
            if !more {
                c.cycles += 1;
                let v = c.lw(r(c, A3)) as u32;
                c.g[V0][0] = sx(v);
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_4348;
                c.event();
            }
        }
    }

    // Third pass: integrate the interior nodes.
    c.cycles += 5;
    c.g[T1][0] = 0x1;
    let v = r(c, V0).wrapping_sub(1);
    c.g[V0][0] = sx(v);
    let v = ((r(c, T1) as i64) < (r(c, V0) as i64)) as u32;
    c.g[V0][0] = sx(v);
    let no_interior = c.g[V0][0] == 0;
    let v = at(r(c, T2), 0x4);
    c.g[A2][0] = sx(v);
    if !no_interior {
        c.cycles += 8;
        c.mul_s(0, 6, 6);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 8);
        c.g[T0][0] = 0x50;
        set_r(c, A1, at(sp, 0x20));
        c.add_s(5, 0, 0);
        c.mul_s(7, 0, 6);
        loop {
            c.cycles += 110;
            c.mfc1(2, 6);
            c.lqc2(4, r(c, A1));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x170));
            let v = r(c, T1) << 4;
            c.g[A0][0] = sx(v);
            c.lq(3, at(sp, 0x170));
            set_r(c, V0, at(sp, 0x20));
            c.lwc1(1, at(r(c, A3), 0x4));
            let v = r(c, V0).wrapping_add(r(c, A0));
            c.g[V0][0] = sx(v);
            c.sq(3, at(sp, 0x160));
            let v = r(c, T1).wrapping_add(1);
            c.g[T1][0] = sx(v);
            c.lqc2(3, r(c, A1).wrapping_sub(0x10));
            c.lqc2(4, r(c, V0));
            let node = r(c, V0);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1a0));
            c.mul_s(1, 7, 1);
            c.lq(2, at(sp, 0x1a0));
            let v = r(c, A1).wrapping_add(0x10);
            c.g[A1][0] = sx(v);
            c.sq(2, at(sp, 0x190));
            c.mfc1(3, 1);
            c.lqc2(4, at(sp, 0x190));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x1a0));
            c.lwc1(0, at(r(c, A3), 0x4));
            c.lq(2, at(sp, 0x1a0));
            c.mul_s(0, 5, 0);
            c.sq(2, at(sp, 0x180));
            c.add_s(0, 0, 8);
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at(sp, 0x180));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, at(sp, 0x1a0));
            c.lq(2, at(sp, 0x1a0));
            c.sq(2, at(sp, 0x170));
            c.lqc2(3, at(sp, 0x160));
            c.lqc2(4, at(sp, 0x170));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1a0));
            c.lwc1(0, at(r(c, A3), 0x4));
            c.lq(2, at(sp, 0x1a0));
            c.mul_s(0, 5, 0);
            c.sq(2, at(sp, 0x150));
            c.add_s(0, 0, 8);
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at(sp, 0x150));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, at(sp, 0x170));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.lq(2, at(sp, 0x170));
            let v = r(c, T0).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            c.sq(2, at(sp, 0x160));
            c.sq(2, at(sp, 0x150));
            let node = r(c, V1);
            c.lqc2(3, at(node, 0x10));
            c.lqc2(4, at(sp, 0x150));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(node, 0x10));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            let v = r(c, T0).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.mfc1(4, 6);
            let node = r(c, V0);
            c.lqc2(4, at(node, 0x10));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x170));
            let nodes = c.lw(at(r(c, A3), 0x24));
            c.lq(3, at(sp, 0x170));
            set_r(c, V0, nodes as u32);
            c.sq(3, at(sp, 0x160));
            let v = r(c, T0).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            let node = r(c, V0);
            c.lqc2(3, node);
            c.lqc2(4, at(sp, 0x160));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, node);
            c.lwc1(1, at(r(c, A3), 0xc));
            c.lwc1(2, at(r(c, A2), 0x4));
            c.lwc1(4, r(c, A2).wrapping_sub(0x4));
            c.mul_s(3, 5, 1);
            c.mul_s(1, 7, 1);
            c.lwc1(0, r(c, A2));
            c.add_s(4, 4, 2);
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            c.add_s(3, 3, 8);
            let v = r(c, A2).wrapping_add(0x4);
            c.g[A2][0] = sx(v);
            c.mul_s(0, 6, 0);
            let v = r(c, T0).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.mul_s(1, 1, 4);
            let node = r(c, V0);
            c.lwc1(2, at(node, 0x24));
            c.div_s(1, 1, 3);
            c.add_s(0, 0, 1);
            c.div_s(0, 0, 3);
            c.add_s(2, 2, 0);
            c.swc1(2, at(node, 0x24));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            let v = r(c, T0).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            let node = r(c, V1);
            c.lwc1(1, at(node, 0x24));
            c.lwc1(0, at(node, 0x20));
            c.mul_s(1, 6, 1);
            c.add_s(0, 0, 1);
            c.swc1(0, at(node, 0x20));
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            let v = r(c, V0).wrapping_sub(1);
            c.g[V0][0] = sx(v);
            let v = ((r(c, T1) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            let more = c.g[V0][0] != 0;
            let v = r(c, T0).wrapping_add(0x50);
            c.g[T0][0] = sx(v);
            if !more {
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_43d0;
                c.event();
            }
        }
    }

    // Fourth pass: clear the per-node force/torque accumulators.
    c.cycles += 3;
    let v = c.lw(r(c, A3)) as u32;
    c.g[V0][0] = sx(v);
    let none: bool = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if !none {
        c.cycles += 3;
        c.g[V0][0] = 0x500000;
        c.g[A1][0] = 0;
        let v = r(c, V0).wrapping_sub(0xee0);
        c.g[T0][0] = sx(v);
        loop {
            c.cycles += 12;
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, A0, nodes as u32);
            let v = r(c, A2).wrapping_add(1);
            c.g[A2][0] = sx(v);
            c.lq(3, r(c, T0));
            let v = r(c, A1).wrapping_add(r(c, A0));
            c.g[A0][0] = sx(v);
            let node = r(c, A0);
            c.sq(3, at(node, 0x30));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            let v = r(c, A1).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.sw(at(r(c, V0), 0x40), 0);
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            let v = ((r(c, A2) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            let more = c.g[V0][0] != 0;
            let v = r(c, A1).wrapping_add(0x50);
            c.g[A1][0] = sx(v);
            if !more {
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_45a0;
                c.event();
            }
        }
    }

    c.cycles += 2;
    let frame_base = r(c, SP).wrapping_add(0x1c0);
    c.g[SP][0] = sx(frame_base);
    abi::ret(c);
}


/// `FlexBridge_SpringsSave` (0x3545D8): serialize a bridge's springs through
/// the stream's writer slot (twice: the header at +0x24 and the spring array
/// at +0x24 * 0x50 bytes).
pub fn flex_bridge_springs_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x24;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, write, 0x0035_460c)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = 0x50;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.mult(0, ZERO, A0, A2);
        c.g[A1][0] = lw_at(c, S0, 0x24);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
        c.g[A2][0] = c.lo[0];
        let write = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, write, 0x0035_4634)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::ONE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;

    /// Where the springs test keeps the chain (16 nodes of 0x50 bytes).
    pub(crate) const NODES: u32 = 0x0000_0200;


    #[test]
    fn flex_bridge_springs_step_matches() {
        check(
            ssx3_ee::s11::f_00353fc0__generated,
            flex_bridge_springs_step,
            120,
            &|c, seed| {
                let mut r = Rng(0x353f_c001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(crate::difftest::GAME_GP);
                // The frame-clock singleton the step length comes from.
                c.bus.write32(crate::difftest::GAME_GP + 0x2a74, DATA + 0x300);
                c.bus.write32(DATA + 0x310, 0x42f0_0000); // 120.0f
                // Zero vector used for F[0]/F[n-1] and the cleared forces.
                put_vec(c, 0x004f_f120, [0, 0, 0, ONE]);
                // Three or four nodes: both interior-loop lengths, and never
                // the empty chain (whose `F[0] = F[-1]` write is a no-op).
                let n = 3 + seed % 2;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x4, r.edge_f32()); // k
                c.bus.write32(DATA + 0x8, r.edge_f32()); // damp
                c.bus.write32(DATA + 0xc, r.edge_f32()); // c
                c.bus.write32(DATA + 0x10, r.edge_f32()); // e
                c.bus.write32(DATA + 0x1c, r.edge_f32()); // rest
                c.bus.write32(DATA + 0x20, r.edge_f32()); // scale
                // The chain sits low in RAM: the node pointer is then small,
                // so the fixture stays bounded whatever the loops read as the
                // node count.
                c.bus.write32(DATA + 0x24, NODES);
                for i in 0..16 * 0x50 / 4 {
                    c.bus.write32(NODES + 4 * i, r.edge_f32());
                }
                // The count last, so nothing in the fixture can overwrite it.
                c.bus.write32(DATA, n);
                c.pc = 0x0035_3fc0;
            },
        );
    }


    #[test]
    fn flex_bridge_springs_save_matches() {
        check(ee::s11::f_003545d8__generated, flex_bridge_springs_save, SEEDS, &|c, seed| {
            machine(c, seed, 0x35_45d8, 2);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2200, DATA + 0x2400);
            slot(c, DATA + 0x2400, 0x8, 0, FAKE);
            w(c, DATA + 0x2000, 32);
            w(c, DATA + 0x2000 + 0x24, 0x10);
            c.pc = 0x0035_45d8;
        });
    }
}
