//! Staging module oc589 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc589`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0035_e248,
    roller_modifier_update_inertia,
    "RollerModifier_UpdateInertia",
)];

/// `void cRollerModifier_updateMOI(cRollerModifier* self)` (0x35e248),
/// `RollerModifier_UpdateInertia`: turns the body's orientation quaternion
/// (Pose.Q, +0x40) into a rotation R and stores R * InvBodyMOI * R^T as the
/// rigid body's world-space InvMOI (+0x80). InvBodyMOI is the 3x3 float block
/// at +0x280; `self` arrives in a0.
///
/// The quaternion-to-matrix step runs on VU0 (issued as the generated
/// `vu0()` ops), the 3x3 products on the FPU. Every intermediate is also
/// written to a stack scratch slot, and those bytes are observable, so the
/// port keeps each store in the generated order.
pub fn roller_modifier_update_inertia(c: &mut Cpu) {
    c.cycles += 329;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
    let stack = c.g[SP][0] as u32;
    let obj = c.g[A0][0] as u32;
    c.swc1(25, stack.wrapping_add(0x158));
    c.swc1(24, stack.wrapping_add(0x150));
    c.swc1(23, stack.wrapping_add(0x148));
    c.swc1(22, stack.wrapping_add(0x140));
    c.swc1(21, stack.wrapping_add(0x138));
    c.swc1(20, stack.wrapping_add(0x130));
    c.lqc2(4, obj.wrapping_add(0x40));
    c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
    c.vu0().add(0b1110, 5, 4, Src::Vec(4));
    c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
    c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
    c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
    c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
    c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
    c.vu0().opmula(5, 4);
    c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
    c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
    c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
    c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
    c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
    c.vu0().suba(0b0010, 1, Src::Bc(6, X));
    c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
    c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
    c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
    c.vu0().adda(0b0010, 0, Src::Bc(8, X));
    c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
    c.vu0().adda(0b0100, 0, Src::Bc(8, X));
    c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
    c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
    c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
    c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
    c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
    c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
    c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
    c.sqc2(0, stack.wrapping_add(0x30));
    c.sqc2(10, stack);
    c.sqc2(11, stack.wrapping_add(0x10));
    c.sqc2(12, stack.wrapping_add(0x20));
    c.lwc1(0, stack);
    c.lwc1(1, stack.wrapping_add(0x4));
    c.lwc1(2, stack.wrapping_add(0x8));
    c.lwc1(3, stack.wrapping_add(0x10));
    c.lwc1(4, stack.wrapping_add(0x14));
    c.lwc1(5, stack.wrapping_add(0x18));
    c.lwc1(6, stack.wrapping_add(0x20));
    c.lwc1(7, stack.wrapping_add(0x24));
    c.lwc1(8, stack.wrapping_add(0x28));
    c.swc1(0, stack.wrapping_add(0xd0));
    c.swc1(1, stack.wrapping_add(0xd4));
    c.swc1(2, stack.wrapping_add(0xd8));
    c.swc1(3, stack.wrapping_add(0xdc));
    c.swc1(4, stack.wrapping_add(0xe0));
    c.swc1(5, stack.wrapping_add(0xe4));
    c.swc1(6, stack.wrapping_add(0xe8));
    c.swc1(7, stack.wrapping_add(0xec));
    c.swc1(8, stack.wrapping_add(0xf0));
    roller_inertia_copy_dwords(c, stack.wrapping_add(0xa0), stack.wrapping_add(0xd0));
    roller_inertia_copy_word(c, stack.wrapping_add(0xc0), stack.wrapping_add(0xf0));
    roller_inertia_copy_dwords(c, stack.wrapping_add(0x70), stack.wrapping_add(0xa0));
    roller_inertia_copy_word(c, stack.wrapping_add(0x90), stack.wrapping_add(0xc0));
    roller_inertia_copy_dwords(c, stack.wrapping_add(0x40), stack.wrapping_add(0x70));
    roller_inertia_copy_word(c, stack.wrapping_add(0x60), stack.wrapping_add(0x90));
    c.lwc1(17, stack.wrapping_add(0x74));
    c.lwc1(2, stack.wrapping_add(0x7c));
    c.swc1(17, stack.wrapping_add(0x7c));
    c.lwc1(17, stack.wrapping_add(0x78));
    c.lwc1(0, stack.wrapping_add(0x88));
    c.swc1(17, stack.wrapping_add(0x88));
    c.lwc1(1, stack.wrapping_add(0x8c));
    c.swc1(2, stack.wrapping_add(0x74));
    c.swc1(0, stack.wrapping_add(0x78));
    c.lwc1(17, stack.wrapping_add(0x84));
    c.swc1(1, stack.wrapping_add(0x84));
    c.lwc1(15, obj.wrapping_add(0x28c));
    c.lwc1(12, obj.wrapping_add(0x290));
    c.lwc1(1, obj.wrapping_add(0x280));
    c.lwc1(7, obj.wrapping_add(0x284));
    c.lwc1(5, stack.wrapping_add(0x48));
    c.lwc1(14, stack.wrapping_add(0x54));
    c.lwc1(19, obj.wrapping_add(0x294));
    c.mul_s(22, 5, 1);
    c.lwc1(0, obj.wrapping_add(0x298));
    c.mul_s(25, 14, 7);
    c.lwc1(2, obj.wrapping_add(0x29c));
    c.mul_s(24, 14, 12);
    c.lwc1(8, obj.wrapping_add(0x288));
    c.mul_s(21, 5, 15);
    c.lwc1(9, obj.wrapping_add(0x2a0));
    c.mul_s(14, 14, 2);
    c.lwc1(3, stack.wrapping_add(0x40));
    c.mul_s(5, 5, 0);
    c.lwc1(10, stack.wrapping_add(0x4c));
    c.add_s(22, 22, 25);
    c.lwc1(4, stack.wrapping_add(0x44));
    c.mul_s(18, 3, 0);
    c.lwc1(11, stack.wrapping_add(0x50));
    c.mul_s(23, 10, 2);
    c.swc1(17, stack.wrapping_add(0x8c));
    c.mul_s(6, 3, 1);
    c.mul_s(17, 10, 7);
    c.lwc1(13, stack.wrapping_add(0x58));
    c.mul_s(2, 11, 2);
    c.lwc1(20, stack.wrapping_add(0x60));
    c.mul_s(7, 11, 7);
    c.lwc1(16, stack.wrapping_add(0x5c));
    c.mul_s(0, 4, 0);
    c.mul_s(1, 4, 1);
    c.mul_s(10, 10, 12);
    c.mul_s(11, 11, 12);
    c.mul_s(3, 3, 15);
    c.mul_s(4, 4, 15);
    c.add_s(1, 1, 7);
    c.add_s(3, 3, 10);
    c.add_s(4, 4, 11);
    c.add_s(0, 0, 2);
    c.mul_s(11, 20, 9);
    c.mul_s(7, 13, 9);
    c.mul_s(10, 20, 8);
    c.mul_s(2, 13, 8);
    c.mul_s(9, 16, 9);
    c.mul_s(8, 16, 8);
    c.mul_s(20, 20, 19);
    c.mul_s(13, 13, 19);
    c.mul_s(16, 16, 19);
    c.add_s(6, 6, 17);
    c.add_s(18, 18, 23);
    c.add_s(21, 21, 24);
    c.add_s(5, 5, 14);
    c.add_s(6, 6, 2);
    c.add_s(3, 3, 13);
    c.add_s(18, 18, 7);
    c.add_s(1, 1, 8);
    c.swc1(6, stack.wrapping_add(0x100));
    c.add_s(4, 4, 16);
    c.swc1(3, stack.wrapping_add(0x10c));
    c.add_s(0, 0, 9);
    c.swc1(18, stack.wrapping_add(0x118));
    c.add_s(22, 22, 10);
    c.swc1(1, stack.wrapping_add(0x104));
    c.add_s(21, 21, 20);
    c.swc1(4, stack.wrapping_add(0x110));
    c.add_s(5, 5, 11);
    c.swc1(0, stack.wrapping_add(0x11c));
    c.swc1(22, stack.wrapping_add(0x108));
    c.swc1(21, stack.wrapping_add(0x114));
    c.swc1(5, stack.wrapping_add(0x120));
    roller_inertia_copy_dwords(c, stack.wrapping_add(0xd0), stack.wrapping_add(0x100));
    roller_inertia_copy_word(c, stack.wrapping_add(0xf0), stack.wrapping_add(0x120));
    c.lwc1(5, stack.wrapping_add(0xd0));
    c.lwc1(6, stack.wrapping_add(0x70));
    c.lwc1(11, stack.wrapping_add(0xdc));
    c.lwc1(12, stack.wrapping_add(0x74));
    c.mul_s(0, 5, 6);
    c.lwc1(19, stack.wrapping_add(0xe8));
    c.mul_s(2, 11, 12);
    c.lwc1(20, stack.wrapping_add(0x78));
    c.mul_s(1, 19, 20);
    c.add_s(0, 0, 2);
    c.add_s(0, 0, 1);
    c.swc1(0, stack.wrapping_add(0x100));
    c.lwc1(13, stack.wrapping_add(0x7c));
    c.lwc1(14, stack.wrapping_add(0x80));
    c.lwc1(3, stack.wrapping_add(0x88));
    c.mul_s(17, 5, 13);
    c.lwc1(8, stack.wrapping_add(0x8c));
    c.mul_s(23, 11, 14);
    c.lwc1(0, stack.wrapping_add(0xd4));
    c.mul_s(5, 5, 3);
    c.lwc1(4, stack.wrapping_add(0xe0));
    c.mul_s(11, 11, 8);
    c.lwc1(1, stack.wrapping_add(0xd8));
    c.mul_s(18, 0, 3);
    c.lwc1(2, stack.wrapping_add(0xe4));
    c.mul_s(22, 4, 8);
    c.mul_s(15, 1, 6);
    c.lwc1(9, stack.wrapping_add(0x90));
    c.mul_s(21, 2, 12);
    c.lwc1(10, stack.wrapping_add(0xf0));
    c.mul_s(8, 2, 8);
    c.lwc1(7, stack.wrapping_add(0xec));
    c.mul_s(12, 4, 12);
    c.lwc1(16, stack.wrapping_add(0x84));
    c.mul_s(3, 1, 3);
    c.lwc1(25, stack.wrapping_add(0x158));
    c.mul_s(6, 0, 6);
    c.lwc1(24, stack.wrapping_add(0x150));
    c.mul_s(2, 2, 14);
    c.mul_s(4, 4, 14);
    c.mul_s(1, 1, 13);
    c.mul_s(0, 0, 13);
    c.add_s(5, 5, 11);
    c.add_s(1, 1, 2);
    c.add_s(0, 0, 4);
    c.mul_s(11, 10, 9);
    c.mul_s(2, 19, 9);
    c.mul_s(4, 10, 20);
    c.mul_s(9, 7, 9);
    c.mul_s(20, 7, 20);
    c.add_s(17, 17, 23);
    c.add_s(6, 6, 12);
    c.lwc1(23, stack.wrapping_add(0x148));
    c.add_s(18, 18, 22);
    c.add_s(15, 15, 21);
    c.lwc1(22, stack.wrapping_add(0x140));
    c.add_s(3, 3, 8);
    c.lwc1(21, stack.wrapping_add(0x138));
    c.mul_s(10, 10, 16);
    c.mul_s(19, 19, 16);
    c.mul_s(7, 7, 16);
    c.add_s(6, 6, 20);
    c.add_s(17, 17, 19);
    c.lwc1(20, stack.wrapping_add(0x130));
    c.add_s(5, 5, 2);
    c.add_s(0, 0, 7);
    c.swc1(6, stack.wrapping_add(0x104));
    c.add_s(18, 18, 9);
    c.swc1(17, stack.wrapping_add(0x10c));
    c.add_s(15, 15, 4);
    c.swc1(5, stack.wrapping_add(0x118));
    c.add_s(1, 1, 10);
    c.swc1(0, stack.wrapping_add(0x110));
    c.add_s(3, 3, 11);
    c.swc1(18, stack.wrapping_add(0x11c));
    c.swc1(15, stack.wrapping_add(0x108));
    c.swc1(1, stack.wrapping_add(0x114));
    c.swc1(3, stack.wrapping_add(0x120));
    roller_inertia_copy_dwords(c, stack.wrapping_add(0xa0), stack.wrapping_add(0x100));
    roller_inertia_copy_word(c, stack.wrapping_add(0xc0), stack.wrapping_add(0x120));
    roller_inertia_copy_dwords(c, obj.wrapping_add(0x80), stack.wrapping_add(0xa0));
    roller_inertia_copy_word(c, obj.wrapping_add(0xa0), stack.wrapping_add(0xc0));
    c.g[SP][0] = sx(stack.wrapping_add(0x160));
    ret(c);
}

/// Copies 32 bytes as four 8-byte chunks from `src` to `dst` (both 8-aligned),
/// the way the generated body does it: the ldl/ldr pairs load each chunk into
/// v0, v1, a1 and a2 (registers 2, 3, 5, 6), then the sdl/sdr pairs store them.
/// Those registers keep the last chunk loaded.
fn roller_inertia_copy_dwords(c: &mut Cpu, dst: u32, src: u32) {
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.ldl(3, src.wrapping_add(15));
    c.ldr(3, src.wrapping_add(8));
    c.ldl(5, src.wrapping_add(23));
    c.ldr(5, src.wrapping_add(16));
    c.ldl(6, src.wrapping_add(31));
    c.ldr(6, src.wrapping_add(24));
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    c.sdl(3, dst.wrapping_add(15));
    c.sdr(3, dst.wrapping_add(8));
    c.sdl(5, dst.wrapping_add(23));
    c.sdr(5, dst.wrapping_add(16));
    c.sdl(6, dst.wrapping_add(31));
    c.sdr(6, dst.wrapping_add(24));
}

/// One 32-bit copy through v0 (`lw` then `sw`), as the generated body does.
fn roller_inertia_copy_word(c: &mut Cpu, dst: u32, src: u32) {
    c.g[V0][0] = c.lw(src);
    c.sw(dst, c.g[V0][0]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, check, fill, scramble, Rng, DATA};

    /// The body is straight-line code, so one fixture covers every path; the
    /// object's quaternion and InvBodyMOI are random (or edge-case floats on
    /// every fifth seed), and the stored InvMOI starts random too, so a
    /// missing store shows.
    #[test]
    fn roller_modifier_update_inertia_matches() {
        check(
            ee::s11::f_0035e248__generated,
            roller_modifier_update_inertia,
            300,
            &|c, seed| {
                let mut r = Rng(0x35e2_48aa ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0035_e248;
                c.g[RA][0] = sx(difftest::RA);
                c.g[SP][0] = sx(difftest::SP);
                c.g[A0][0] = sx(DATA);
                let edge = seed % 5 == 0;
                for k in 0..4 {
                    let v = if edge { r.edge_f32() } else { r.f32(2.0) };
                    c.bus.write32(DATA + 0x40 + 4 * k, v);
                }
                for k in 0..9 {
                    let v = if edge { r.edge_f32() } else { r.f32(2.0) };
                    c.bus.write32(DATA + 0x280 + 4 * k, v);
                }
                fill(c, &mut r, DATA + 0x80, 9);
            },
        );
    }
}
