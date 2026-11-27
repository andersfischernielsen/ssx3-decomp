/// Stage of `PS2GraphicsMan_ClipLineSegment`: near plane (outcode bit 0x20): the end that is behind `clip_min.w` moves onto the plane; z/w come from the plane, x/y/u/v/colour are interpolated with the perspective factor `frac * w1 / plane`. Re-derives that end's outcode and rejects when the two outcodes now share a bit.
#[inline(never)]
fn clipline_near(c: &mut Cpu) -> bool {
    c.g[V0][0] = c.g[T1][0] & 0x20;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 47;
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = sx(((c.g[T0][0] as u32) as i32 >> 5) as u32);
        c.lwc1(7, (c.g[A0][0] as u32).wrapping_add(0x6b7c));
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.lwc1(6, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.sub_s(0, 2, 7);
        c.lwc1(5, (c.g[A2][0] as u32));
        c.sub_s(2, 2, 6);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.sub_s(1, 5, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.div_s(0, 0, 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.sub_s(3, 3, 4);
        c.mult(0, 8, 9, 3);
        c.mul_s(1, 0, 1);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.mul_s(3, 0, 3);
        c.mul_s(0, 0, 6);
        c.sub_s(5, 5, 1);
        c.div_s(4, 0, 7);
        c.swc1(5, (c.g[V0][0] as u32));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.sub_s(0, 0, 3);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b78));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b7c));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 4, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32));
        c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 4, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 4, 1);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 1, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.c_lt_s(1, 0);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x8;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x4;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] == 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if !t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: far plane (outcode bit 0x10): as the near plane, against `clip_max.w`.
#[inline(never)]
fn clipline_far(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x10;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 48;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x6b8c));
        c.lwc1(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.sub_s(0, 1, 6);
        c.lwc1(4, (c.g[A2][0] as u32));
        c.sub_s(1, 1, 5);
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.sub_s(2, 4, 2);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.div_s(0, 0, 1);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.sub_s(3, 3, 1);
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.mult(0, 8, 9, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.mul_s(2, 0, 2);
        c.mul_s(3, 0, 3);
        c.mul_s(0, 0, 5);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sub_s(4, 4, 2);
        c.div_s(5, 0, 6);
        c.swc1(4, (c.g[V0][0] as u32));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.sub_s(0, 0, 3);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b88));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b8c));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 5, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32));
        c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 5, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 5, 1);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 1, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.c_lt_s(1, 0);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x8;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x4;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: bottom edge (outcode bit 8): the end below `clip_min.y` moves onto the edge; x/z/w/u/v/colour interpolate linearly. Rejects on a shared outcode bit (x outcodes re-derived).
#[inline(never)]
fn clipline_y_min(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x8;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 51;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.sub_s(1, 0, 1);
        c.lwc1(5, (c.g[A2][0] as u32));
        c.sub_s(0, 0, 2);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(3, 5, 3);
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(6, 1, 0);
        c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.sub_s(4, 4, 1);
        c.sub_s(2, 2, 0);
        c.mult(0, 8, 9, 3);
        c.mul_s(3, 6, 3);
        c.mul_s(4, 6, 4);
        c.mul_s(2, 6, 2);
        c.sub_s(5, 5, 3);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.swc1(5, (c.g[V0][0] as u32));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 6, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 6, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 6, 5);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 5, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: top edge (outcode bit 4): as the bottom edge, against `clip_max.y`.
#[inline(never)]
fn clipline_y_max(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x4;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 51;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.sub_s(1, 0, 1);
        c.lwc1(5, (c.g[A2][0] as u32));
        c.sub_s(0, 0, 2);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(3, 5, 3);
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(6, 1, 0);
        c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.sub_s(4, 4, 1);
        c.sub_s(2, 2, 0);
        c.mult(0, 8, 9, 3);
        c.mul_s(3, 6, 3);
        c.mul_s(4, 6, 4);
        c.mul_s(2, 6, 2);
        c.sub_s(5, 5, 3);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.swc1(5, (c.g[V0][0] as u32));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 6, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 6, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 6, 5);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 5, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: left edge (outcode bit 2): clip against `clip_min.x`; the moved end's outcode becomes 0 (no reject test: it is the last but one stage).
#[inline(never)]
fn clipline_x_min(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x2;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 51;
        c.lwc1(0, (c.g[A2][0] as u32));
        c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.sub_s(2, 0, 6);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(0, 0, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[T0][0] = c.g[T0][0] ^ 0x1;
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(7, 2, 0);
        c.g[T0][0] = c.g[T0][0] & 0x1;
        c.sub_s(3, 3, 1);
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.mult(0, 10, 8, 3);
        c.lwc1(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.sub_s(4, 4, 0);
        c.swc1(6, (c.g[V0][0] as u32));
        c.mul_s(3, 7, 3);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.mul_s(4, 7, 4);
        c.sub_s(2, 2, 5);
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sub_s(1, 1, 3);
        c.mul_s(2, 7, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 7, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 7, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
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
        c.mul_s(0, 7, 0);
        c.mult(0, 3, 8, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
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
        c.mul_s(0, 7, 0);
        c.mult(0, 3, 8, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T2][0] as u32));
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
            c.g[V0][0] = 0x30;
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = 0x30;
        }
        c.cycles += 10;
        c.mul_s(0, 7, 0);
        c.mult(0, 3, 8, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 7;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
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
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.add_s(0, 1, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T2][0] as u32));
        c.sw((c.g[V0][0] as u32), 0u64);
    }
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: right edge (outcode bit 1): clip against `clip_max.x`; the moved end's outcode becomes 0.
#[inline(never)]
fn clipline_x_max(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x1;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 50;
        c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.lwc1(0, (c.g[A2][0] as u32));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.sub_s(2, 0, 6);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(0, 0, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.g[V0][0] ^ 0x1;
        c.g[A0][0] = c.g[A0][0] & 0x1;
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(7, 2, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
        c.sub_s(3, 3, 1);
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.lwc1(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.sub_s(4, 4, 0);
        c.swc1(6, (c.g[V0][0] as u32));
        c.mult(0, 8, 4, 3);
        c.sub_s(2, 2, 5);
        c.mul_s(3, 7, 3);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.mul_s(4, 7, 4);
        c.mul_s(2, 7, 2);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sub_s(1, 1, 3);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 7, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 7, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
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
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = 0x30;
        c.mult(0, 3, 4, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T0][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
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
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = 0x30;
        c.mult(0, 3, 4, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[A2][0] as u32));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
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
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = 0x30;
        c.mult(0, 3, 4, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T0][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 5);
            c.cvt_s_w(5, 5);
            c.add_s(5, 5, 5);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        } else {
            c.cycles += 4;
            c.mtc1(2, 5);
            c.cvt_s_w(5, 5);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.add_s(0, 5, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[A2][0] as u32));
        c.sw((c.g[V0][0] as u32), 0u64);
    }
}
