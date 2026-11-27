/// `Avalanche_BakeNodeHistory` (0x002d66a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn avalanche_bake_node_history(c: &mut Cpu) {
    if c.pc != 0x002d66a0 {
        return ee::s09::f_002d66a0__generated(c);
    }
    guest(c, |c| {
'b_2d7af0: {
c.cycles += 99;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3e0));
c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
c.g[T0][0] = 0xa;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
c.g[S0][0] = 0x500000;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
c.g[S1][0] = 0x7f0000;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
c.g[S5][0] = c.g[A0][0];
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x3b8));
c.g[S6][0] = c.g[A1][0];
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x3b0));
c.g[A1][0] = 0x480000;
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x3a8));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7538));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
c.g[S1][0] = c.g[S1][0] | 0xffff;
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
c.sd((c.g[SP][0] as u32).wrapping_add(0x2f0), c.g[RA][0]);
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x3d0));
c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x3c8));
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x3c0));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 25);
c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x100));
c.g[A2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xf8));
c.cvt_w_s(1, 0);
c.mfc1(4, 1);
c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
c.lq(3, (c.g[A2][0] as u32).wrapping_add(0x40));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
c.mult(0, 0, 4, 8);
c.g[A2][0] = 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c0), c.g[V0][0]);
c.sq(3, (c.g[S5][0] as u32).wrapping_add(0xe0));
c.swc1(25, (c.g[S5][0] as u32).wrapping_add(0xec));
c.g[A0][0] = c.lo[0];
c.g[RA][0] = sx(0x002d675c);
c.sq(3, (c.g[SP][0] as u32));
call_ptr(c, 0x00317e30, 0x002d675c)?;
c.g[V1][0] = 0x1;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x4;
c.sb((c.g[S5][0] as u32).wrapping_add(0xf3), c.g[V1][0]);
c.g[RA][0] = sx(0x002d6778);
c.sw((c.g[S5][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
call_ptr(c, 0x00416210, 0x002d6778)?;
c.g[V0][0] = 0x500000;
c.lwc1(23, (c.g[GP][0] as u32).wrapping_add(0x990));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
c.sw((c.g[SP][0] as u32), 0u64);
c.lq(4, (c.g[V0][0] as u32));
c.neg_s(22, 23);
c.g[V0][0] = 0xffffffffffffffff;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), c.g[V0][0]);
c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3cf8));
c.lq(2, (c.g[S5][0] as u32).wrapping_add(0xe0));
c.lq(3, (c.g[S0][0] as u32));
c.mul_s(22, 22, 21);
c.lwc1(24, (c.g[SP][0] as u32));
c.mul_s(21, 23, 21);
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c8), 0u64);
c.g[S0][0] = 0x3f800000;
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sub_s(21, 21, 22);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
c.g[RA][0] = sx(0x002d67dc);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xc));
call_ptr(c, 0x003177f0, 0x002d67dc)?;
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
c.mtc1(2, 0);
c.sub_s(20, 0, 25);
c.mul_s(20, 21, 20);
c.g[RA][0] = sx(0x002d67f8);
c.add_s(20, 22, 20);
call_ptr(c, 0x003177f0, 0x002d67f8)?;
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x100));
c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.mtc1(2, 1);
c.c_lt_s(24, 4);
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
c.sub_s(0, 1, 25);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x5c));
c.mul_s(21, 21, 0);
c.add_s(22, 22, 21);
let t = !c.fcc();
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x54));
if t {
break 'b_2d7af0;
} else {
c.cycles += 3;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 28);
'l_2d6838: loop {
'b_2d68a0: {
'b_2d689c: {
'b_2d6874: {
c.cycles += 13;
c.lwc1(12, (c.g[S5][0] as u32).wrapping_add(0x108));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c8));
c.cvt_s_w(0, 0);
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3cf4));
c.sub_s(2, 4, 12);
c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
c.mov_s(27, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x6c));
c.sw((c.g[SP][0] as u32).wrapping_add(0x64), 0u64);
c.c_le_s(0, 2);
let t = !c.fcc();
c.sw((c.g[SP][0] as u32).wrapping_add(0x68), 0u64);
if t {
break 'b_2d6874;
} else {
c.cycles += 2;
let t = true;
c.lwc1(21, (c.g[S5][0] as u32).wrapping_add(0x110));
if t {
break 'b_2d689c;
} else {
break 'b_2d6874;
}
}
}
c.cycles += 9;
c.sub_s(0, 4, 27);
c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x110));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.mul_s(1, 1, 0);
c.div_s(21, 1, 12);
c.c_lt_s(2, 21);
let t = !c.fcc();
c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
if t {
break 'b_2d68a0;
} else {
c.cycles += 1;
c.sqrt_s(21, 21);
break 'b_2d689c;
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
break 'b_2d68a0;
}
'b_2d7690: {
c.cycles += 17;
c.g[AT][0] = 0x438c0000;
c.mtc1(1, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
c.g[S0][0] = 0u64;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
c.min_s(21, 21, 0);
c.g[AT][0] = 0x42700000;
c.mtc1(1, 1);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9d8));
c.max_s(21, 21, 1);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 26);
let t = (c.g[V1][0] as i64) <= 0;
c.sw((c.g[SP][0] as u32).wrapping_add(0x2dc), c.g[A0][0]);
if t {
break 'b_2d7690;
} else {
c.cycles += 17;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
c.g[S7][0] = 0x7f0000;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 23);
c.mov_s(22, 26);
c.sw((c.g[SP][0] as u32).wrapping_add(0x2d0), c.g[A1][0]);
c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x2d8), c.g[A2][0]);
c.g[S4][0] = 0x7f800000;
c.sw((c.g[SP][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
c.g[S3][0] = 0x7fc00000;
c.g[S2][0] = 0xffffffffff800000;
c.g[S1][0] = 0xffffffffffc00000;
c.g[S7][0] = c.g[S7][0] | 0xffff;
'l_2d6928: loop {
'b_2d6e2c: {
'b_2d6e28: {
c.cycles += 34;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[A0][0] = c.g[S5][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[S0][0]);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = sx(0x002d6968);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
Callee { addr: 0x002d64d8, f: ee::s09::Avalanche_ClampVector }.call(c, 0x002d6968)?;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2cc));
c.mov_s(12, 21);
c.sq(2, (c.g[SP][0] as u32));
c.g[A2][0] = 0u64;
c.g[RA][0] = sx(0x002d6988);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc));
call_ptr(c, 0x0032fb48, 0x002d6988)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c0));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2cc));
c.g[RA][0] = sx(0x002d699c);
c.g[A3][0] = 0x1;
call_ptr(c, 0x003378c0, 0x002d699c)?;
c.cvt_w_s(1, 0);
c.mfc1(2, 1);
let t = (c.g[V0][0] as i64) <= 0;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
if t {
break 'b_2d6e28;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
let t = c.g[A0][0] == 0u64;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
if t {
break 'b_2d6e2c;
} else {
'b_2d6a64: {
c.cycles += 4;
c.g[V0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9dc));
if t {
break 'b_2d6a64;
} else {
'b_2d6a60: {
c.cycles += 4;
c.g[RA][0] = sx(0x002d69d4);
call_ptr(c, 0x00350698, 0x002d69d4)?;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0u64;
if t {
break 'b_2d6a60;
} else {
'b_2d6a20: {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = c.g[V0][0];
if t {
break 'b_2d6a20;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x14));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
let t = c.g[V0][0] == c.g[A0][0];

if t {
break 'b_2d6a20;
} else {
c.cycles += 4;
c.g[A3][0] = c.g[A0][0];
c.g[A1][0] = c.g[A2][0];
c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x14));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
'l_2d6a08: loop {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
if t {
break 'b_2d6a20;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
if c.g[V0][0] != c.g[A3][0] {
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
if event_due(c) { deliver_event_here(c, 0x002d6a08); } continue 'l_2d6a08;
} else {
break 'b_2d6a20;
}
}
}
}
}
}
c.cycles += 2;
let t = c.g[V1][0] != c.g[A2][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9dc));
if t {
break 'b_2d6a64;
} else {
c.cycles += 14;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cf0));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
c.mul_s(0, 27, 0);
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.swc1(0, (c.g[V1][0] as u32));
c.g[V0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
c.g[V1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sh((c.g[S6][0] as u32).wrapping_add(0xc), c.g[V1][0]);
break 'b_2d6a60;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9dc));
break 'b_2d6a64;
}
}
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
if t {
break 'b_2d6e2c;
} else {
'b_2d6b88: {
c.cycles += 65;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.g[A1][0] = 0u64;
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A0][0] = 0u64;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.mfc1(3, 22);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x9a0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 0);
c.mul_s(0, 0, 1);
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
if c.g[V0][0] == c.g[S4][0] {
c.g[A0][0] = 0x1;
break 'b_2d6b88;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[A0][0] = 0x1;
break 'b_2d6b88;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[A0][0] = 0x1;
break 'b_2d6b88;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S1][0] {
c.g[A0][0] = 0x1;
break 'b_2d6b88;
} else {
break 'b_2d6b88;
}
}
}
}
}
'b_2d6c20: {
c.cycles += 2;
if c.g[A0][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d6c20;
} else {
'b_2d6bb8: {
'b_2d6bb4: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d6bb4;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6bb8;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6bb8;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6bb8;
} else {
break 'b_2d6bb4;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6bb8;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d6c20;
} else {
'b_2d6be8: {
'b_2d6be4: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d6be4;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6be8;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6be8;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6be8;
} else {
break 'b_2d6be4;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6be8;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d6c20;
} else {
'b_2d6c18: {
'b_2d6c14: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d6c14;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6c18;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6c18;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6c18;
} else {
break 'b_2d6c14;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6c18;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d6c20;
} else {
break 'b_2d6c20;
}
}
}
}
}
'b_2d6c34: {
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[V1][0] = 0x500000;
if t {
break 'b_2d6c34;
} else {
c.cycles += 3;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xee0));
c.lq(2, (c.g[V1][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
break 'b_2d6c34;
}
}
'b_2d6d30: {
'b_2d6d2c: {
c.cycles += 56;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A0][0] = c.g[S5][0];
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x99c));
c.g[A1][0] = c.g[FP][0];
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mfc1(3, 21);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x210));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x210));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x200));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x210));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x210));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = sx(0x002d6cac);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
Callee { addr: 0x002d64d8, f: ee::s09::Avalanche_ClampVector }.call(c, 0x002d6cac)?;
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x210));
c.g[A0][0] = 0u64;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x210));
c.g[V1][0] = 0u64;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x200));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x210));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x210));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V0][0] == c.g[S4][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
if t {
break 'b_2d6d2c;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6d30;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6d30;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6d30;
} else {
break 'b_2d6d2c;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6d30;
}
'b_2d6dc8: {
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d6dc8;
} else {
'b_2d6d60: {
'b_2d6d5c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d6d5c;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6d60;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6d60;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6d60;
} else {
break 'b_2d6d5c;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6d60;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d6dc8;
} else {
'b_2d6d90: {
'b_2d6d8c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d6d8c;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6d90;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6d90;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6d90;
} else {
break 'b_2d6d8c;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6d90;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d6dc8;
} else {
'b_2d6dc0: {
'b_2d6dbc: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d6dbc;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d6dc0;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d6dc0;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d6dc0;
} else {
break 'b_2d6dbc;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d6dc0;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d6dc8;
} else {
break 'b_2d6dc8;
}
}
}
}
}
'b_2d6e20: {
'b_2d6de4: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[A0][0] = 0x500000;
if t {
break 'b_2d6de4;
} else {
c.cycles += 5;
c.mtc1(0, 24);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xec0));
c.lq(2, (c.g[V1][0] as u32));
let t = true;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_2d6e20;
} else {
break 'b_2d6de4;
}
}
}
c.cycles += 15;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x978));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 2);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cec));
c.mul_s(1, 1, 2);
c.mul_s(0, 21, 0);
c.div_s(24, 1, 0);
break 'b_2d6e20;
}
c.cycles += 2;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ce8));
c.min_s(24, 24, 0);
break 'b_2d6e28;
}
}
}
}
c.cycles += 1;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
break 'b_2d6e2c;
}
'b_2d7654: {
'b_2d7650: {
c.cycles += 15;
c.g[A0][0] = c.g[S0][0];
c.mov_s(12, 21);
c.g[RA][0] = sx(0x002d6e3c);
c.g[A2][0] = 0u64;
call_ptr(c, 0x0032fb48, 0x002d6e3c)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c0));
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d8));
c.g[RA][0] = sx(0x002d6e50);
c.g[A3][0] = 0u64;
call_ptr(c, 0x00336850, 0x002d6e50)?;
c.mtc1(0, 1);
c.mov_s(26, 0);
c.c_le_s(1, 26);
let t = !c.fcc();
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
if t {
break 'b_2d7650;
} else {
c.cycles += 3;
c.g[V0][0] = 0xffffffffffffffff;
let t = c.g[V1][0] == c.g[V0][0];
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
if t {
break 'b_2d7654;
} else {
'b_2d6ebc: {
c.cycles += 15;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x960));
c.abs_s(0, 0);
c.c_lt_s(1, 0);
let t = !c.fcc();
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
if t {
break 'b_2d6ebc;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S6][0];
c.g[RA][0] = sx(0x002d6ebc);
c.g[A2][0] = c.g[S5][0];
Callee { addr: 0x002d9738, f: ee::s09::Avalanche_AddNodeEvent }.call(c, 0x002d6ebc)?;
break 'b_2d6ebc;
}
}
'b_2d7470: {
'b_2d7270: {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S5][0] as u32).wrapping_add(0xf0));
let t = c.g[V0][0] != 0u64;

if t {
break 'b_2d7270;
} else {
c.cycles += 11;
c.g[RA][0] = sx(0x002d6ed0);
call_ptr(c, 0x003177f0, 0x002d6ed0)?;
c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
c.g[A1][0] = 0x3f800000;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x97c));
c.mtc1(2, 2);
c.sub_s(0, 2, 23);
c.c_lt_s(0, 1);
let t = !c.fcc();

if t {
break 'b_2d7270;
} else {
'b_2d70d0: {
c.cycles += 113;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.g[A1][0] = 0u64;
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A0][0] = 0u64;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.mfc1(3, 22);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x9a0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 0);
c.mul_s(0, 0, 1);
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x998));
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x99c));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
if c.g[V0][0] == c.g[S4][0] {
c.g[A0][0] = 0x1;
break 'b_2d70d0;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[A0][0] = 0x1;
break 'b_2d70d0;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[A0][0] = 0x1;
break 'b_2d70d0;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S1][0] {
c.g[A0][0] = 0x1;
break 'b_2d70d0;
} else {
break 'b_2d70d0;
}
}
}
}
}
'b_2d7168: {
c.cycles += 2;
if c.g[A0][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7168;
} else {
'b_2d7100: {
'b_2d70fc: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d70fc;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7100;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7100;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7100;
} else {
break 'b_2d70fc;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d7100;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7168;
} else {
'b_2d7130: {
'b_2d712c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d712c;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7130;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7130;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7130;
} else {
break 'b_2d712c;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d7130;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7168;
} else {
'b_2d7160: {
'b_2d715c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d715c;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7160;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7160;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7160;
} else {
break 'b_2d715c;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d7160;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7168;
} else {
break 'b_2d7168;
}
}
}
}
}
'b_2d717c: {
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[V1][0] = 0x500000;
if t {
break 'b_2d717c;
} else {
c.cycles += 3;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xee0));
c.lq(2, (c.g[V1][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
break 'b_2d717c;
}
}
c.cycles += 61;
c.g[RA][0] = sx(0x002d7184);
c.lwc1(20, (c.g[GP][0] as u32).wrapping_add(0x980));
call_ptr(c, 0x003177f0, 0x002d7184)?;
c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
c.g[A0][0] = 0x3f800000;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.neg_s(0, 20);
c.mtc1(2, 2);
c.sub_s(20, 20, 0);
c.sub_s(1, 2, 23);
c.mul_s(20, 20, 1);
c.add_s(0, 0, 20);
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lwc1(20, (c.g[GP][0] as u32).wrapping_add(0x984));
c.g[RA][0] = sx(0x002d720c);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
call_ptr(c, 0x003177f0, 0x002d720c)?;
c.g[V1][0] = 0x3f800000;
c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.sub_s(0, 1, 23);
c.mul_s(20, 20, 0);
c.mfc1(3, 20);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x200));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x200));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x988));
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
let t = true;

if t {
break 'b_2d7470;
} else {
break 'b_2d7270;
}
}
}
}
'b_2d7390: {
c.cycles += 66;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.g[A1][0] = 0u64;
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A0][0] = 0u64;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.mfc1(3, 22);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x9a0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 0);
c.mul_s(0, 0, 1);
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
if c.g[V0][0] == c.g[S4][0] {
c.g[A0][0] = 0x1;
break 'b_2d7390;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[A0][0] = 0x1;
break 'b_2d7390;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[A0][0] = 0x1;
break 'b_2d7390;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S1][0] {
c.g[A0][0] = 0x1;
break 'b_2d7390;
} else {
break 'b_2d7390;
}
}
}
}
}
'b_2d7428: {
c.cycles += 2;
if c.g[A0][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7428;
} else {
'b_2d73c0: {
'b_2d73bc: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d73bc;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d73c0;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d73c0;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d73c0;
} else {
break 'b_2d73bc;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d73c0;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7428;
} else {
'b_2d73f0: {
'b_2d73ec: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d73ec;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d73f0;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d73f0;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d73f0;
} else {
break 'b_2d73ec;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d73f0;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7428;
} else {
'b_2d7420: {
'b_2d741c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d741c;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7420;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7420;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7420;
} else {
break 'b_2d741c;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d7420;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2d7428;
} else {
break 'b_2d7428;
}
}
}
}
}
'b_2d743c: {
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[V1][0] = 0x500000;
if t {
break 'b_2d743c;
} else {
c.cycles += 3;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xee0));
c.lq(2, (c.g[V1][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
break 'b_2d743c;
}
}
c.cycles += 13;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x99c));
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
break 'b_2d7470;
}
'b_2d7558: {
'b_2d7554: {
c.cycles += 51;
c.mfc1(2, 21);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A0][0] = c.g[S5][0];
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A1][0] = c.g[FP][0];
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sub_s(0, 23, 0);
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x002d74d4);
Callee { addr: 0x002d64d8, f: ee::s09::Avalanche_ClampVector }.call(c, 0x002d74d4)?;
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[A0][0] = 0u64;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.g[V1][0] = 0u64;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x130));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V0][0] == c.g[S4][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
if t {
break 'b_2d7554;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7558;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7558;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7558;
} else {
break 'b_2d7554;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d7558;
}
'b_2d75f0: {
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d75f0;
} else {
'b_2d7588: {
'b_2d7584: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d7584;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7588;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7588;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7588;
} else {
break 'b_2d7584;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d7588;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d75f0;
} else {
'b_2d75b8: {
'b_2d75b4: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d75b4;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d75b8;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d75b8;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d75b8;
} else {
break 'b_2d75b4;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d75b8;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d75f0;
} else {
'b_2d75e8: {
'b_2d75e4: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
let t = c.g[V0][0] == c.g[S4][0];
c.g[V1][0] = 0u64;
if t {
break 'b_2d75e4;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d75e8;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d75e8;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d75e8;
} else {
break 'b_2d75e4;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = 0x1;
break 'b_2d75e8;
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2d75f0;
} else {
break 'b_2d75f0;
}
}
}
}
}
'b_2d7648: {
'b_2d760c: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[A0][0] = 0x500000;
if t {
break 'b_2d760c;
} else {
c.cycles += 5;
c.mtc1(0, 24);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xec0));
c.lq(2, (c.g[V1][0] as u32));
let t = true;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_2d7648;
} else {
break 'b_2d760c;
}
}
}
c.cycles += 15;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x978));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 2);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ce4));
c.mul_s(1, 1, 2);
c.mul_s(0, 21, 0);
c.div_s(24, 1, 0);
break 'b_2d7648;
}
c.cycles += 2;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ce0));
c.min_s(24, 24, 0);
break 'b_2d7650;
}
}
}
c.cycles += 1;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
break 'b_2d7654;
}
c.cycles += 15;
c.g[A0][0] = c.g[S5][0];
c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e0));
c.g[A1][0] = c.g[FP][0];
c.sub_s(0, 23, 0);
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x002d7680);
Callee { addr: 0x002d64d8, f: ee::s09::Avalanche_ClampVector }.call(c, 0x002d7680)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9d8));
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x002d6928); } continue 'l_2d6928;
} else {
break 'b_2d7690;
}
}
}
}
'b_2d7958: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_le_s(0, 26);
let t = !c.fcc();
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf4));
if t {
break 'b_2d7958;
} else {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
if t {
break 'b_2d7958;
} else {
'b_2d76f0: {
c.cycles += 4;
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x156));
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
let t = (c.g[A1][0] as i64) < 0;
c.g[V0][0] = 0u64;
if t {
break 'b_2d76f0;
} else {
c.cycles += 7;
c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x155));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
c.g[V1][0] = c.g[V1][0] ^ 0x6;
if c.g[V1][0] != 0u64 {
c.g[V0][0] = 0u64;
break 'b_2d76f0;
} else {
c.cycles += 6;
c.g[V0][0] = 0x18;
c.mult(0, 3, 5, 2);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3f0));
c.g[V1][0] = c.g[V1][0] ^ 0x3;
c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
break 'b_2d76f0;
}
}
}
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_2d7958;
} else {
'b_2d77e8: {
c.cycles += 50;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf4));
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10c));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[RA][0] = sx(0x002d770c);
c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x110));
Callee { addr: 0x002edb20, f: ee::s09::WorldPainter_SampleSurfaceColours }.call(c, 0x002d770c)?;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = 0xffffffffffffffff;
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x8c));
c.sub_s(1, 25, 1);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x88));
c.sub_s(2, 25, 2);
c.sub_s(4, 25, 4);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x9b4));
c.sub_s(3, 25, 3);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x150));
c.mul_s(9, 1, 0);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x154));
c.mul_s(7, 2, 0);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x120));
c.mul_s(8, 4, 0);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x124));
c.mul_s(0, 3, 0);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x128));
c.sub_s(6, 25, 9);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x12c));
c.sub_s(5, 25, 7);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x158));
c.sub_s(2, 25, 0);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x15c));
c.sub_s(1, 25, 8);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x130));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x134));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x138));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x13c));
c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x140));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x144));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x148));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x14c));
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x137));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x130));
c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x13f));
c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x138));
c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x87));
c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x8f));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x88));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
let t = c.g[A0][0] != c.g[V0][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
if t {
break 'b_2d77e8;
} else {
c.cycles += 10;
c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x87));
c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x8f));
c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x88));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x47));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x4f));
c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x48));
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), 0u64);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
break 'b_2d77e8;
}
}
'b_2d7910: {
c.cycles += 13;
c.g[S0][0] = 0u64;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x47));
c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x40));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x4f));
c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x48));
c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x137));
c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x13f));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x138));
let t = (c.g[V0][0] as i64) <= 0;
c.g[S2][0] = c.g[V0][0];
if t {
break 'b_2d7910;
} else {
c.cycles += 7;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c4));
c.cvt_s_w(0, 0);
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
c.g[V0][0] = 0xa;
c.mult(0, 17, 6, 2);
c.sub_s(20, 27, 0);
'l_2d7838: loop {
c.cycles += 54;
c.mtc1(16, 4);
c.cvt_s_w(4, 4);
c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x44));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
c.div_s(4, 4, 20);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x48));
c.sub_s(0, 0, 7);
c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x4c));
c.sub_s(1, 1, 8);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x88));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x8c));
c.sub_s(2, 2, 6);
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
c.sub_s(3, 3, 9);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x160));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x164));
c.mul_s(0, 0, 4);
c.mul_s(1, 1, 4);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x168));
c.mul_s(5, 3, 4);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x16c));
c.mul_s(2, 2, 4);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x150));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x154));
c.add_s(0, 0, 7);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x15c));
c.add_s(1, 1, 8);
c.add_s(6, 2, 6);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x158));
c.add_s(5, 5, 9);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x140));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x144));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x148));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x14c));
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x147));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x140));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x14f));
c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x148));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x137));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x13f));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x138));
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x134));
c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x138));
c.g[RA][0] = sx(0x002d7904);
c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x13c));
Callee { addr: 0x002d55c0, f: ee::s09::Avalanche_PackClamped255x3 }.call(c, 0x002d7904)?;
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x002d7838); } continue 'l_2d7838;
} else {
break 'b_2d7910;
}
}
}
}
c.cycles += 18;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
c.g[A0][0] = 0xa;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
c.mult(0, 4, 3, 4);
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x84));
c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x88));
c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x8c));
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), c.g[V1][0]);
c.g[RA][0] = sx(0x002d7938);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
Callee { addr: 0x002d55c0, f: ee::s09::Avalanche_PackClamped255x3 }.call(c, 0x002d7938)?;
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x87));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x8f));
c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x88));
c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x47));
c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x4f));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x48));
break 'b_2d7958;
}
}
}
}
'b_2d7a5c: {
c.cycles += 44;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
c.g[V0][0] = 0xa;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.mult(0, 16, 3, 2);
c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3cdc));
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
c.mul_s(15, 0, 15);
c.lwc1(12, (c.g[SP][0] as u32));
c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[RA][0] = sx(0x002d7988);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
Callee { addr: 0x002d5658, f: ee::s09::Avalanche_PackSnorm3Alpha }.call(c, 0x002d7988)?;
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.sub_s(0, 0, 3);
c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x74));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x78));
c.sub_s(2, 2, 4);
c.mul_s(0, 0, 28);
c.g[AT][0] = 0x42f00000;
c.mtc1(1, 5);
c.sub_s(1, 1, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), 0u64);
c.mul_s(2, 2, 28);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x120));
c.mul_s(1, 1, 28);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x124));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x128));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 0);
c.c_lt_s(5, 0);
if !c.fcc() {
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
break 'b_2d7a5c;
} else {
c.cycles += 21;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
c.mfc1(3, 5);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x140));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
break 'b_2d7a5c;
}
}
c.cycles += 37;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x124));
c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x128));
c.g[RA][0] = sx(0x002d7a78);
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c8), c.g[A1][0]);
Callee { addr: 0x002d5598, f: ee::s09::Avalanche_PackBytes3 }.call(c, 0x002d7a78)?;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c8));
c.cvt_s_w(3, 3);
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2));
c.g[A0][0] = c.lb((c.g[V0][0] as u32));
c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
c.mtc1(3, 1);
c.cvt_s_w(1, 1);
c.mtc1(4, 2);
c.cvt_s_w(2, 2);
c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
c.mtc1(5, 0);
c.cvt_s_w(0, 0);
c.add_s(1, 1, 1);
c.add_s(2, 2, 2);
c.add_s(0, 0, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x148));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x144));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x140));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x100));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
c.c_lt_s(3, 4);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.fcc();
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
if t {
if event_due(c) { deliver_event_here(c, 0x002d6838); } continue 'l_2d6838;
} else {
break 'b_2d7af0;
}
}
}
}
'b_2d7b3c: {
c.cycles += 6;
c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[A2][0]);
c.g[V0][0] = ((c.g[A0][0] as i64) < 32i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x110));
if t {
break 'b_2d7b3c;
} else {
c.cycles += 6;
c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cd8));
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S6][0] as u32)));
'l_2d7b20: loop {
c.cycles += 7;
c.swc1(0, (c.g[A1][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.sw((c.g[V1][0] as u32), 0u64);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = ((c.g[A0][0] as i64) < 32i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x002d7b20); } continue 'l_2d7b20;
} else {
break 'b_2d7b3c;
}
}
}
}
'b_2d7b7c: {
c.cycles += 4;
c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xe));
c.g[V0][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
if t {
break 'b_2d7b7c;
} else {
c.cycles += 5;
c.g[A2][0] = 0xffff;
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x112));
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S6][0] as u32)));
'l_2d7b60: loop {
c.cycles += 7;
c.sh((c.g[A1][0] as u32), c.g[A2][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.sh((c.g[V1][0] as u32), 0u64);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x002d7b60); } continue 'l_2d7b60;
} else {
break 'b_2d7b7c;
}
}
}
}
c.cycles += 33;
c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[A3][0] = 0x2d0000;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5718));
c.g[RA][0] = sx(0x002d7b94);
c.g[A2][0] = 0x8;
call_ptr(c, 0x00418ef8, 0x002d7b94)?;
c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xe));
c.g[A3][0] = 0x2d0000;
c.g[A0][0] = c.g[S0][0];
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5750));
c.g[RA][0] = sx(0x002d7bac);
c.g[A2][0] = 0x4;
call_ptr(c, 0x00418ef8, 0x002d7bac)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2f0));
c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x3d0));
c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x3c8));
c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x3c0));
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x3b8));
c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x3b0));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x3a8));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3e0));
c.pc = t; return Ok(());
    });
}
