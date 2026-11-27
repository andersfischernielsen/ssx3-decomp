/// The four colour channels of a clipped end (`verts[k].r..a`, `u32` words at
/// +0x10..+0x1c): `(float)v0 + frac * (float)(v1 - v0)` truncated, the unsigned
/// word turned into a float the compiler's way (halve, keep the low bit,
/// double when bit 31 is set). `fr` is the FPU register holding the factor,
/// `cr` the register of the last channel's `(float)v0`. The scratch registers it
/// leaves differ per stage, so the instruction order is kept exactly.
#[inline(never)]
fn clipline_colours(c: &mut Cpu, fr: usize, cr: usize) {
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
    if neg {
        c.cycles += 8;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    } else {
        c.cycles += 4;
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.g[V0][0] = 0x30;
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V0][0] = 0x30;
    }
    c.cycles += 10;
    c.mul_s(0, fr, 0);
    c.mult(0, 3, 9, 2);
    c.add_s(0, 1, 0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[T2][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    } else {
        c.cycles += 4;
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.g[V0][0] = 0x30;
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V0][0] = 0x30;
    }
    c.cycles += 10;
    c.mul_s(0, fr, 0);
    c.mult(0, 3, 9, 2);
    c.add_s(0, 1, 0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[T0][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    } else {
        c.cycles += 4;
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.mul_s(0, fr, 0);
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, fr, 0);
    }
    c.cycles += 10;
    c.g[V0][0] = 0x30;
    c.mult(0, 3, 9, 2);
    c.add_s(0, 1, 0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[T2][0] as u32));
    c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, cr);
        c.cvt_s_w(cr, cr);
        c.add_s(cr, cr, cr);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    } else {
        c.cycles += 4;
        c.mtc1(2, cr);
        c.cvt_s_w(cr, cr);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.mul_s(0, fr, 0);
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, fr, 0);
    }
}
