//! Rider (`docs/investigation/structs/rider.md`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, Vec4, VuF32};
use ssx3_ee_rt::Cpu;

/// A rider object (`RiderBase`; the fields used here sit in its first 0x200
/// bytes).
#[derive(Clone, Copy, Debug)]
pub struct Rider(pub Addr);

impl Rider {
    /// World position (x, y, z; w unused).
    pub fn position(self) -> Addr {
        self.0.off(0x110)
    }
    /// Orientation quaternion (x, y, z, w).
    pub fn rotation(self) -> Addr {
        self.0.off(0x120)
    }
    /// World matrix: three basis rows and the translation row.
    pub fn world_matrix(self) -> Addr {
        self.0.off(0x1a0)
    }
}

/// `Rider_BuildWorldMatrix(Rider*)` (0x11e098): renormalise the rotation
/// quaternion in place and build the 4x4 world matrix from it and the
/// position. Returns the matrix address.
///
/// Original: hand-written VU0 macro code (46 EE cycles, no calls). The
/// arithmetic below follows its operation order lane by lane so every
/// result is bit-identical, and the VU0 registers it used as scratch are
/// left with the same values (later code can spill them to the stack).
pub fn build_world_matrix(c: &mut Cpu) {
    c.cycles += 46;
    let rider = Rider(Addr(abi::arg(c, 0)));
    let m = rider.world_matrix();
    abi::set_ret(c, m.0);
    let one = VuF32::ONE;

    // |q|^2 = ((x^2 + y^2) + z^2) + w^2, then q /= |q|, stored back.
    let q = rider.rotation().read_vec4(c);
    let (xx, yy, zz, ww) = (q.x * q.x, q.y * q.y, q.z * q.z, q.w * q.w);
    let len2 = VuF32::madd(VuF32::madd(xx + yy, one, zz), one, ww);
    let inv = VuF32::rsqrt(one, len2);
    let n = Vec4 { x: q.x * inv, y: q.y * inv, z: q.z * inv, w: q.w * inv };
    rider.rotation().write_vec4(c, n);

    let pos = rider.position().read_vec4(c);
    let (x2, y2, z2) = (n.x + n.x, n.y + n.y, n.z + n.z);
    // Squares, products with w, and cross products (doubled).
    let (sx, sy, sz) = (x2 * n.x, y2 * n.y, z2 * n.z);
    let (xw, yw, zw) = (x2 * n.w, y2 * n.w, z2 * n.w);
    let zero = VuF32::ZERO;
    let yz = VuF32::madd(y2 * n.z, zero, zero);
    let zx = VuF32::madd(z2 * n.x, zero, zero);
    let xy = VuF32::madd(x2 * n.y, zero, zero);

    let row0 = Vec4 {
        x: VuF32::msub(one - sy, one, sz),
        y: VuF32::madd(zero + xy, one, zw),
        z: VuF32::msub(zero + zx, one, yw),
        w: zero,
    };
    let row1 = Vec4 {
        x: VuF32::msub(zero + xy, one, zw),
        y: VuF32::msub(one - sz, one, sx),
        z: VuF32::madd(zero + yz, one, xw),
        w: zero,
    };
    let row2 = Vec4 {
        x: VuF32::madd(zero + zx, one, yw),
        y: VuF32::msub(zero + yz, one, xw),
        z: VuF32::msub(one - sx, one, sy),
        w: zero,
    };
    // VU0 scratch registers as the original leaves them (VU0 context savers
    // can copy them, ACC included, to memory).
    let v = |x, y, z, w| Vec4 { x, y, z, w };
    vu0_scratch::set_vf(c, 1, 0b1110, v(one, one, one, zero));
    vu0_scratch::set_vf(c, 3, 0b1111, pos);
    vu0_scratch::set_vf(c, 4, 0b1111, n);
    vu0_scratch::set_vf(c, 5, 0b1111, v(x2, y2, z2, n.w));
    vu0_scratch::set_vf(c, 6, 0b1110, v(sx, sy, sz, zero));
    vu0_scratch::set_vf(c, 7, 0b1110, v(xw, yw, zw, zero));
    vu0_scratch::set_vf(c, 8, 0b1110, v(yz, zx, xy, zero));
    vu0_scratch::set_vf(c, 10, 0b1111, row0);
    vu0_scratch::set_vf(c, 11, 0b1111, row1);
    vu0_scratch::set_vf(c, 12, 0b1111, row2);
    vu0_scratch::set_q(c, inv);
    // ACC as the last ADDAs leave it (x: 0+zx, y: 0+yz, z: 0+zx; w untouched).
    vu0_scratch::set_acc(c, 0b1110, v(zero + zx, zero + yz, zero + zx, zero));

    m.write_vec4(c, row0);
    m.off(0x10).write_vec4(c, row1);
    m.off(0x20).write_vec4(c, row2);
    m.off(0x30).write_vec4(c, pos);
    abi::ret(c);
}
