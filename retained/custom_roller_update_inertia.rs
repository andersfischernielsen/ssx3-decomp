//! Native RollerModifier_UpdateInertia (0x0035e248), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replays were
//! added or run, in accordance with the user's instruction.

use crate::abi;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0035_e248,
    roller_modifier_update_inertia,
    "RollerModifier_UpdateInertia",
)];

const ORIENTATION: u32 = 0x40;
const LOCAL_INVERSE_INERTIA: u32 = 0x280;
const WORLD_INVERSE_INERTIA: u32 = 0x80;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Copy a 36-byte matrix as the native four unaligned doubleword pairs,
/// followed by its final word. Keep the exact V0/V1/A1/A2 scratch results.
#[inline]
fn copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    for (register, offset) in [(V0, 0), (V1, 8), (A1, 16), (A2, 24)] {
        c.ldl(register, from.wrapping_add(offset + 7));
        c.ldr(register, from.wrapping_add(offset));
    }
    for (register, offset) in [(V0, 0), (V1, 8), (A1, 16), (A2, 24)] {
        c.sdl(register, to.wrapping_add(offset + 7));
        c.sdr(register, to.wrapping_add(offset));
    }
    c.g[V0][0] = c.lw(from.wrapping_add(32));
    c.sw(to.wrapping_add(32), c.g[V0][0]);
}

/// `void RollerModifier_UpdateInertia(RollerModifier* self)`.
/// Build a 3x3 rotation from the quaternion at self+0x40 with the original
/// VU0 kernel, rotate the local inverse-inertia tensor at +0x280..+0x2a0
/// into world space, and write the resulting tensor at +0x80..+0xa0.
/// Preserve the native scratch matrices, FPU ordering, VU state and saved
/// F20..F25. This leaf makes no calls and has no resumable return sites.
pub fn roller_modifier_update_inertia(c: &mut Cpu) {
    c.cycles += 329;
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x160));
    let stack = r(c, SP);
    let object = r(c, A0);
    c.swc1(25, at(c, SP, 0x158));
    c.swc1(24, at(c, SP, 0x150));
    c.swc1(23, at(c, SP, 0x148));
    c.swc1(22, at(c, SP, 0x140));
    c.swc1(21, at(c, SP, 0x138));
    c.swc1(20, at(c, SP, 0x130));
    // Build the orientation matrix from the native quaternion with VU0.
    c.lqc2(4, at(c, A0, ORIENTATION));
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
    c.sqc2(0, at(c, SP, 0x30));
    c.sqc2(10, r(c, SP));
    c.sqc2(11, at(c, SP, 0x10));
    c.sqc2(12, at(c, SP, 0x20));
    c.lwc1(0, r(c, SP));
    c.lwc1(1, at(c, SP, 0x4));
    c.lwc1(2, at(c, SP, 0x8));
    c.lwc1(3, at(c, SP, 0x10));
    c.lwc1(4, at(c, SP, 0x14));
    c.lwc1(5, at(c, SP, 0x18));
    c.lwc1(6, at(c, SP, 0x20));
    c.lwc1(7, at(c, SP, 0x24));
    c.lwc1(8, at(c, SP, 0x28));
    c.swc1(0, at(c, SP, 0xd0));
    c.swc1(1, at(c, SP, 0xd4));
    c.swc1(2, at(c, SP, 0xd8));
    c.swc1(3, at(c, SP, 0xdc));
    c.swc1(4, at(c, SP, 0xe0));
    c.swc1(5, at(c, SP, 0xe4));
    c.swc1(6, at(c, SP, 0xe8));
    c.swc1(7, at(c, SP, 0xec));
    c.swc1(8, at(c, SP, 0xf0));
    copy_matrix(c, stack.wrapping_add(0xd0), stack.wrapping_add(0xa0));
    copy_matrix(c, stack.wrapping_add(0xa0), stack.wrapping_add(0x70));
    copy_matrix(c, stack.wrapping_add(0x70), stack.wrapping_add(0x40));
    // Transpose one copy of the 3x3 rotation in place.
    c.lwc1(17, at(c, SP, 0x74));
    c.lwc1(2, at(c, SP, 0x7c));
    c.swc1(17, at(c, SP, 0x7c));
    c.lwc1(17, at(c, SP, 0x78));
    c.lwc1(0, at(c, SP, 0x88));
    c.swc1(17, at(c, SP, 0x88));
    c.lwc1(1, at(c, SP, 0x8c));
    c.swc1(2, at(c, SP, 0x74));
    c.swc1(0, at(c, SP, 0x78));
    c.lwc1(17, at(c, SP, 0x84));
    c.swc1(1, at(c, SP, 0x84));
    // First scalar product: rotation times local inverse inertia.
    c.lwc1(15, at(c, A0, LOCAL_INVERSE_INERTIA + 0xc));
    c.lwc1(12, at(c, A0, LOCAL_INVERSE_INERTIA + 0x10));
    c.lwc1(1, at(c, A0, LOCAL_INVERSE_INERTIA + 0x0));
    c.lwc1(7, at(c, A0, LOCAL_INVERSE_INERTIA + 0x4));
    c.lwc1(5, at(c, SP, 0x48));
    c.lwc1(14, at(c, SP, 0x54));
    c.lwc1(19, at(c, A0, LOCAL_INVERSE_INERTIA + 0x14));
    c.mul_s(22, 5, 1);
    c.lwc1(0, at(c, A0, LOCAL_INVERSE_INERTIA + 0x18));
    c.mul_s(25, 14, 7);
    c.lwc1(2, at(c, A0, LOCAL_INVERSE_INERTIA + 0x1c));
    c.mul_s(24, 14, 12);
    c.lwc1(8, at(c, A0, LOCAL_INVERSE_INERTIA + 0x8));
    c.mul_s(21, 5, 15);
    c.lwc1(9, at(c, A0, LOCAL_INVERSE_INERTIA + 0x20));
    c.mul_s(14, 14, 2);
    c.lwc1(3, at(c, SP, 0x40));
    c.mul_s(5, 5, 0);
    c.lwc1(10, at(c, SP, 0x4c));
    c.add_s(22, 22, 25);
    c.lwc1(4, at(c, SP, 0x44));
    c.mul_s(18, 3, 0);
    c.lwc1(11, at(c, SP, 0x50));
    c.mul_s(23, 10, 2);
    c.swc1(17, at(c, SP, 0x8c));
    c.mul_s(6, 3, 1);
    c.mul_s(17, 10, 7);
    c.lwc1(13, at(c, SP, 0x58));
    c.mul_s(2, 11, 2);
    c.lwc1(20, at(c, SP, 0x60));
    c.mul_s(7, 11, 7);
    c.lwc1(16, at(c, SP, 0x5c));
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
    c.swc1(6, at(c, SP, 0x100));
    c.add_s(4, 4, 16);
    c.swc1(3, at(c, SP, 0x10c));
    c.add_s(0, 0, 9);
    c.swc1(18, at(c, SP, 0x118));
    c.add_s(22, 22, 10);
    c.swc1(1, at(c, SP, 0x104));
    c.add_s(21, 21, 20);
    c.swc1(4, at(c, SP, 0x110));
    c.add_s(5, 5, 11);
    c.swc1(0, at(c, SP, 0x11c));
    c.swc1(22, at(c, SP, 0x108));
    c.swc1(21, at(c, SP, 0x114));
    c.swc1(5, at(c, SP, 0x120));
    copy_matrix(c, stack.wrapping_add(0x100), stack.wrapping_add(0xd0));
    // Complete the rotation into world space; retain scalar FPU order.
    c.lwc1(5, at(c, SP, 0xd0));
    c.lwc1(6, at(c, SP, 0x70));
    c.lwc1(11, at(c, SP, 0xdc));
    c.lwc1(12, at(c, SP, 0x74));
    c.mul_s(0, 5, 6);
    c.lwc1(19, at(c, SP, 0xe8));
    c.mul_s(2, 11, 12);
    c.lwc1(20, at(c, SP, 0x78));
    c.mul_s(1, 19, 20);
    c.add_s(0, 0, 2);
    c.add_s(0, 0, 1);
    c.swc1(0, at(c, SP, 0x100));
    c.lwc1(13, at(c, SP, 0x7c));
    c.lwc1(14, at(c, SP, 0x80));
    c.lwc1(3, at(c, SP, 0x88));
    c.mul_s(17, 5, 13);
    c.lwc1(8, at(c, SP, 0x8c));
    c.mul_s(23, 11, 14);
    c.lwc1(0, at(c, SP, 0xd4));
    c.mul_s(5, 5, 3);
    c.lwc1(4, at(c, SP, 0xe0));
    c.mul_s(11, 11, 8);
    c.lwc1(1, at(c, SP, 0xd8));
    c.mul_s(18, 0, 3);
    c.lwc1(2, at(c, SP, 0xe4));
    c.mul_s(22, 4, 8);
    c.mul_s(15, 1, 6);
    c.lwc1(9, at(c, SP, 0x90));
    c.mul_s(21, 2, 12);
    c.lwc1(10, at(c, SP, 0xf0));
    c.mul_s(8, 2, 8);
    c.lwc1(7, at(c, SP, 0xec));
    c.mul_s(12, 4, 12);
    c.lwc1(16, at(c, SP, 0x84));
    c.mul_s(3, 1, 3);
    c.lwc1(25, at(c, SP, 0x158));
    c.mul_s(6, 0, 6);
    c.lwc1(24, at(c, SP, 0x150));
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
    c.lwc1(23, at(c, SP, 0x148));
    c.add_s(18, 18, 22);
    c.add_s(15, 15, 21);
    c.lwc1(22, at(c, SP, 0x140));
    c.add_s(3, 3, 8);
    c.lwc1(21, at(c, SP, 0x138));
    c.mul_s(10, 10, 16);
    c.mul_s(19, 19, 16);
    c.mul_s(7, 7, 16);
    c.add_s(6, 6, 20);
    c.add_s(17, 17, 19);
    c.lwc1(20, at(c, SP, 0x130));
    c.add_s(5, 5, 2);
    c.add_s(0, 0, 7);
    c.swc1(6, at(c, SP, 0x104));
    c.add_s(18, 18, 9);
    c.swc1(17, at(c, SP, 0x10c));
    c.add_s(15, 15, 4);
    c.swc1(5, at(c, SP, 0x118));
    c.add_s(1, 1, 10);
    c.swc1(0, at(c, SP, 0x110));
    c.add_s(3, 3, 11);
    c.swc1(18, at(c, SP, 0x11c));
    c.swc1(15, at(c, SP, 0x108));
    c.swc1(1, at(c, SP, 0x114));
    c.swc1(3, at(c, SP, 0x120));
    copy_matrix(c, stack.wrapping_add(0x100), stack.wrapping_add(0xa0));
    copy_matrix(
        c,
        stack.wrapping_add(0xa0),
        object.wrapping_add(WORLD_INVERSE_INERTIA),
    );
    c.g[SP][0] = sx(at(c, SP, 0x160));
    abi::ret(c);
}
