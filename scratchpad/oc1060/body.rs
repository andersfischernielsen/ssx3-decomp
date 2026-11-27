pub fn f_00383a10__generated(c: &mut Cpu) {
'b_383d9c: {
'b_383da4: {
c.cycles += 6;
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
c.g[V0][0] = c.g[T1][0] & 0x20;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x30;
if t {
break 'b_383da4;
} else {
'b_383b14: {
'b_383af4: {
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
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
let t = (c.g[V0][0] as i64) < 0;
c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
break 'b_383af4;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
if t {
break 'b_383b14;
} else {
break 'b_383af4;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_383b14;
}
'b_383b48: {
'b_383b30: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383b30;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_383b48;
} else {
break 'b_383b30;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_383b48;
}
'b_383b9c: {
'b_383b80: {
c.cycles += 10;
c.mul_s(0, 4, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383b80;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
if t {
break 'b_383b9c;
} else {
break 'b_383b80;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
break 'b_383b9c;
}
'b_383bd0: {
'b_383bb8: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383bb8;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_383bd0;
} else {
break 'b_383bb8;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_383bd0;
}
'b_383c24: {
'b_383c08: {
c.cycles += 10;
c.mul_s(0, 4, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T0][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383c08;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
if t {
break 'b_383c24;
} else {
break 'b_383c08;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
break 'b_383c24;
}
'b_383c58: {
'b_383c40: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383c40;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 4, 0);
if t {
break 'b_383c58;
} else {
break 'b_383c40;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 4, 0);
break 'b_383c58;
}
'b_383cac: {
'b_383c90: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383c90;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
if t {
break 'b_383cac;
} else {
break 'b_383c90;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
break 'b_383cac;
}
'b_383ce0: {
'b_383cc8: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383cc8;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 4, 0);
if t {
break 'b_383ce0;
} else {
break 'b_383cc8;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 4, 0);
break 'b_383ce0;
}
'b_383d20: {
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
if t {
break 'b_383d20;
} else {
c.cycles += 2;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32));
break 'b_383d20;
}
}
'b_383d44: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
c.c_lt_s(0, 1);
if !c.fcc() {
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_383d44;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x1;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_383d44;
}
}
'b_383d68: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b74));
c.c_lt_s(1, 0);
if !c.fcc() {
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_383d68;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x8;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_383d68;
}
}
'b_383d8c: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b84));
c.c_lt_s(0, 1);
if !c.fcc() {
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_383d8c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_383d8c;
}
}
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
if t {
break 'b_383da4;
} else {
break 'b_383d9c;
}
}
}
'b_384128: {
c.cycles += 3;
c.g[V0][0] = c.g[T1][0] & 0x10;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x30;
if t {
break 'b_384128;
} else {
'b_383ea0: {
'b_383e80: {
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
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
let t = (c.g[V0][0] as i64) < 0;
c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
break 'b_383e80;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
if t {
break 'b_383ea0;
} else {
break 'b_383e80;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_383ea0;
}
'b_383ed4: {
'b_383ebc: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383ebc;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_383ed4;
} else {
break 'b_383ebc;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_383ed4;
}
'b_383f28: {
'b_383f0c: {
c.cycles += 10;
c.mul_s(0, 5, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383f0c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
if t {
break 'b_383f28;
} else {
break 'b_383f0c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
break 'b_383f28;
}
'b_383f5c: {
'b_383f44: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383f44;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_383f5c;
} else {
break 'b_383f44;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_383f5c;
}
'b_383fb0: {
'b_383f94: {
c.cycles += 10;
c.mul_s(0, 5, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T0][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383f94;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
if t {
break 'b_383fb0;
} else {
break 'b_383f94;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
break 'b_383fb0;
}
'b_383fe4: {
'b_383fcc: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_383fcc;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 5, 0);
if t {
break 'b_383fe4;
} else {
break 'b_383fcc;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 5, 0);
break 'b_383fe4;
}
'b_384038: {
'b_38401c: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38401c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
if t {
break 'b_384038;
} else {
break 'b_38401c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
break 'b_384038;
}
'b_38406c: {
'b_384054: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384054;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 5, 0);
if t {
break 'b_38406c;
} else {
break 'b_384054;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 5, 0);
break 'b_38406c;
}
'b_3840ac: {
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
if t {
break 'b_3840ac;
} else {
c.cycles += 2;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32));
break 'b_3840ac;
}
}
'b_3840d0: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
c.c_lt_s(0, 1);
if !c.fcc() {
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_3840d0;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x1;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_3840d0;
}
}
'b_3840f4: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b74));
c.c_lt_s(1, 0);
if !c.fcc() {
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_3840f4;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x8;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_3840f4;
}
}
'b_384118: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b84));
c.c_lt_s(0, 1);
if !c.fcc() {
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_384118;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_384118;
}
}
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
if t {
break 'b_383d9c;
} else {
break 'b_384128;
}
}
}
'b_384470: {
c.cycles += 3;
c.g[V0][0] = c.g[T1][0] & 0x8;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x30;
if t {
break 'b_384470;
} else {
'b_384230: {
'b_384210: {
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
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
let t = (c.g[V0][0] as i64) < 0;
c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
break 'b_384210;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
if t {
break 'b_384230;
} else {
break 'b_384210;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_384230;
}
'b_384264: {
'b_38424c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38424c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_384264;
} else {
break 'b_38424c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_384264;
}
'b_3842b8: {
'b_38429c: {
c.cycles += 10;
c.mul_s(0, 6, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38429c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
if t {
break 'b_3842b8;
} else {
break 'b_38429c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
break 'b_3842b8;
}
'b_3842ec: {
'b_3842d4: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3842d4;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_3842ec;
} else {
break 'b_3842d4;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_3842ec;
}
'b_384340: {
'b_384324: {
c.cycles += 10;
c.mul_s(0, 6, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T0][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384324;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
if t {
break 'b_384340;
} else {
break 'b_384324;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
break 'b_384340;
}
'b_384374: {
'b_38435c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38435c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 6, 0);
if t {
break 'b_384374;
} else {
break 'b_38435c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 6, 0);
break 'b_384374;
}
'b_3843c8: {
'b_3843ac: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3843ac;
} else {
c.cycles += 4;
c.mtc1(2, 5);
c.cvt_s_w(5, 5);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
if t {
break 'b_3843c8;
} else {
break 'b_3843ac;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 5);
c.cvt_s_w(5, 5);
c.add_s(5, 5, 5);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
break 'b_3843c8;
}
'b_3843fc: {
'b_3843e4: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3843e4;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 6, 0);
if t {
break 'b_3843fc;
} else {
break 'b_3843e4;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 6, 0);
break 'b_3843fc;
}
'b_38443c: {
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
if t {
break 'b_38443c;
} else {
c.cycles += 2;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32));
break 'b_38443c;
}
}
'b_384460: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
c.c_lt_s(0, 1);
if !c.fcc() {
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_384460;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x1;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_384460;
}
}
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
if t {
break 'b_383d9c;
} else {
break 'b_384470;
}
}
}
'b_3847b8: {
c.cycles += 3;
c.g[V0][0] = c.g[T1][0] & 0x4;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x30;
if t {
break 'b_3847b8;
} else {
'b_384578: {
'b_384558: {
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
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
let t = (c.g[V0][0] as i64) < 0;
c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
break 'b_384558;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
if t {
break 'b_384578;
} else {
break 'b_384558;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_384578;
}
'b_3845ac: {
'b_384594: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384594;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_3845ac;
} else {
break 'b_384594;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_3845ac;
}
'b_384600: {
'b_3845e4: {
c.cycles += 10;
c.mul_s(0, 6, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3845e4;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
if t {
break 'b_384600;
} else {
break 'b_3845e4;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
break 'b_384600;
}
'b_384634: {
'b_38461c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38461c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_384634;
} else {
break 'b_38461c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_384634;
}
'b_384688: {
'b_38466c: {
c.cycles += 10;
c.mul_s(0, 6, 0);
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T0][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38466c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
if t {
break 'b_384688;
} else {
break 'b_38466c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
break 'b_384688;
}
'b_3846bc: {
'b_3846a4: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3846a4;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 6, 0);
if t {
break 'b_3846bc;
} else {
break 'b_3846a4;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 6, 0);
break 'b_3846bc;
}
'b_384710: {
'b_3846f4: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 9, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3846f4;
} else {
c.cycles += 4;
c.mtc1(2, 5);
c.cvt_s_w(5, 5);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
if t {
break 'b_384710;
} else {
break 'b_3846f4;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 5);
c.cvt_s_w(5, 5);
c.add_s(5, 5, 5);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
break 'b_384710;
}
'b_384744: {
'b_38472c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38472c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 6, 0);
if t {
break 'b_384744;
} else {
break 'b_38472c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 6, 0);
break 'b_384744;
}
'b_384784: {
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
if t {
break 'b_384784;
} else {
c.cycles += 2;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.lwc1(1, (c.g[V1][0] as u32));
break 'b_384784;
}
}
'b_3847a8: {
c.cycles += 5;
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
c.c_lt_s(0, 1);
if !c.fcc() {
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_3847a8;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x1;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
c.g[T0][0] = c.lw((c.g[A3][0] as u32));
break 'b_3847a8;
}
}
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
if t {
break 'b_383d9c;
} else {
break 'b_3847b8;
}
}
}
'b_384aa4: {
c.cycles += 3;
c.g[V0][0] = c.g[T1][0] & 0x2;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x30;
if t {
break 'b_384aa4;
} else {
'b_3848c0: {
'b_3848a0: {
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
let t = (c.g[V0][0] as i64) < 0;
c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
break 'b_3848a0;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
if t {
break 'b_3848c0;
} else {
break 'b_3848a0;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_3848c0;
}
'b_3848f4: {
'b_3848dc: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3848dc;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_3848f4;
} else {
break 'b_3848dc;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_3848f4;
}
'b_384948: {
'b_38492c: {
c.cycles += 10;
c.mul_s(0, 7, 0);
c.mult(0, 3, 8, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T3][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38492c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
if t {
break 'b_384948;
} else {
break 'b_38492c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
break 'b_384948;
}
'b_38497c: {
'b_384964: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384964;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_38497c;
} else {
break 'b_384964;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_38497c;
}
'b_3849d0: {
'b_3849b4: {
c.cycles += 10;
c.mul_s(0, 7, 0);
c.mult(0, 3, 8, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3849b4;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
if t {
break 'b_3849d0;
} else {
break 'b_3849b4;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
break 'b_3849d0;
}
'b_384a04: {
'b_3849ec: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3849ec;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.g[V0][0] = 0x30;
if t {
break 'b_384a04;
} else {
break 'b_3849ec;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.g[V0][0] = 0x30;
break 'b_384a04;
}
'b_384a58: {
'b_384a3c: {
c.cycles += 10;
c.mul_s(0, 7, 0);
c.mult(0, 3, 8, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T3][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384a3c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
if t {
break 'b_384a58;
} else {
break 'b_384a3c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
break 'b_384a58;
}
'b_384a8c: {
'b_384a74: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384a74;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 7, 0);
if t {
break 'b_384a8c;
} else {
break 'b_384a74;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 7, 0);
break 'b_384a8c;
}
c.cycles += 6;
c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.add_s(0, 1, 0);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T2][0] as u32));
c.sw((c.g[V0][0] as u32), 0u64);
break 'b_384aa4;
}
}
'b_384d8c: {
c.cycles += 3;
c.g[V0][0] = c.g[T1][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x30;
if t {
break 'b_384d8c;
} else {
'b_384ba8: {
'b_384b88: {
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
let t = (c.g[V0][0] as i64) < 0;
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
break 'b_384b88;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
if t {
break 'b_384ba8;
} else {
break 'b_384b88;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_384ba8;
}
'b_384bdc: {
'b_384bc4: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384bc4;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 7, 0);
if t {
break 'b_384bdc;
} else {
break 'b_384bc4;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 7, 0);
break 'b_384bdc;
}
'b_384c30: {
'b_384c14: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 4, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T0][0] as u32));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384c14;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
if t {
break 'b_384c30;
} else {
break 'b_384c14;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
break 'b_384c30;
}
'b_384c64: {
'b_384c4c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384c4c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 7, 0);
if t {
break 'b_384c64;
} else {
break 'b_384c4c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 7, 0);
break 'b_384c64;
}
'b_384cb8: {
'b_384c9c: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 4, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[A2][0] as u32));
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384c9c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
if t {
break 'b_384cb8;
} else {
break 'b_384c9c;
}
}
}
c.cycles += 7;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
break 'b_384cb8;
}
'b_384cec: {
'b_384cd4: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384cd4;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 7, 0);
if t {
break 'b_384cec;
} else {
break 'b_384cd4;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 7, 0);
break 'b_384cec;
}
'b_384d3c: {
'b_384d24: {
c.cycles += 10;
c.g[V0][0] = 0x30;
c.mult(0, 3, 4, 2);
c.add_s(0, 1, 0);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[T0][0] as u32));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384d24;
} else {
c.cycles += 4;
c.mtc1(2, 5);
c.cvt_s_w(5, 5);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
if t {
break 'b_384d3c;
} else {
break 'b_384d24;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 5);
c.cvt_s_w(5, 5);
c.add_s(5, 5, 5);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
break 'b_384d3c;
}
'b_384d74: {
'b_384d5c: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_384d5c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 7, 0);
if t {
break 'b_384d74;
} else {
break 'b_384d5c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 7, 0);
break 'b_384d74;
}
c.cycles += 6;
c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.add_s(0, 5, 0);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[A2][0] as u32));
c.sw((c.g[V0][0] as u32), 0u64);
break 'b_384d8c;
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[V0][0] = 0x1;
c.pc = t; return;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[V0][0] = 0u64;
c.pc = t; return;
}

/// Slot for the hand-written replacement of `f_00383a10` (crates/ssx3-game), set
/// once at boot before any guest code runs. Default: the generated body.
pub static mut OVERRIDE_00383A10: fn(&mut Cpu) = f_00383a10__generated;
