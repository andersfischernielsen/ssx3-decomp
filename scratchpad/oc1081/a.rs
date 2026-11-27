'b_39dab4: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S3][0] = c.g[A1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S4][0] = c.g[A3][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S5][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
c.g[RA][0] = sx(0x0039da5c);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00416810, 0x0039da5c)?;
if c.g[V0][0] == 0u64 {
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
break 'b_39dab4;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
break 'b_39dab4;
} else {
c.cycles += 3;
c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
if c.g[S1][0] == 0u64 {
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
break 'b_39dab4;
} else {
'b_39dab0: {
c.cycles += 9;
c.g[RA][0] = sx(0x0039da88);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00317670, 0x0039da88)?;
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0039da94);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x0039c778, 0x0039da94)?;
c.g[A1][0] = c.g[V0][0];
c.g[V0][0] = 0xffff;
let t = c.g[A1][0] == c.g[V0][0];
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_39dab0;
} else {
c.cycles += 3;
c.g[A1][0] = c.g[A1][0] & 0xffff;
c.g[RA][0] = sx(0x0039dab0);
c.g[A2][0] = 0x1;
call_ptr(c, 0x0039c870, 0x0039dab0)?;
break 'b_39dab0;
}
}
c.cycles += 1;
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
break 'b_39dab4;
}
}
}
}
'b_39db14: {
c.cycles += 4;
c.g[RA][0] = sx(0x0039dabc);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00416810, 0x0039dabc)?;
if c.g[V0][0] == 0u64 {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
break 'b_39db14;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
break 'b_39db14;
} else {
c.cycles += 3;
c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
if c.g[S1][0] == 0u64 {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
break 'b_39db14;
} else {
'b_39db10: {
c.cycles += 9;
c.g[RA][0] = sx(0x0039dae8);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00317670, 0x0039dae8)?;
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0039daf4);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x0039c778, 0x0039daf4)?;
c.g[A1][0] = c.g[V0][0];
c.g[V0][0] = 0xffff;
let t = c.g[A1][0] == c.g[V0][0];
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_39db10;
} else {
c.cycles += 3;
c.g[A1][0] = c.g[A1][0] & 0xffff;
c.g[RA][0] = sx(0x0039db10);
c.g[A2][0] = 0x1;
call_ptr(c, 0x0039c870, 0x0039db10)?;
break 'b_39db10;
}
}
c.cycles += 1;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
break 'b_39db14;
}
}
}
}
'b_39de48: {
'b_39de44: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[S4][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S3][0]);
let t = c.g[A2][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[S5][0]);
if t {
break 'b_39de44;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
if c.g[V0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
break 'b_39de48;
} else {
'b_39dcc8: {
'b_39dca8: {
c.cycles += 32;
c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x4b));
c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4c));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039db78);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039db78)?;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A1][0];
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039db98);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039db98)?;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0039dba4);
c.g[A1][0] = 0x1;
call_ptr(c, 0x0039de68, 0x0039dba4)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
let t = c.g[V1][0] == 0u64;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
if t {
break 'b_39dca8;
} else {
'b_39dc14: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x24));
if t {
break 'b_39dc14;
} else {
c.cycles += 20;
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc0));
c.cvt_s_w(1, 1);
c.mtc1(3, 5);
c.cvt_s_w(5, 5);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc8));
c.cvt_s_w(0, 0);
c.lwc1(2, (c.g[SP][0] as u32));
c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.div_s(1, 1, 5);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.div_s(0, 0, 5);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(1, 6, 1);
c.mul_s(0, 6, 0);
c.add_s(3, 3, 1);
c.c_lt_s(6, 0);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
let t = !c.fcc();
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
if t {
break 'b_39dc14;
} else {
c.cycles += 1;
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x34));
break 'b_39dc14;
}
}
}
'b_39dc60: {
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
if c.g[A2][0] == 0u64 {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
break 'b_39dc60;
} else {
'b_39dc48: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
if t {
break 'b_39dc48;
} else {
c.cycles += 7;
c.g[A1][0] = 0u64;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039dc40);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039dc40)?;
let t = true;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
if t {
break 'b_39dc60;
} else {
break 'b_39dc48;
}
}
}
c.cycles += 6;
c.g[A1][0] = 0x1;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039dc5c);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039dc5c)?;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
break 'b_39dc60;
}
}
c.cycles += 2;
let t = c.g[A2][0] == 0u64;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
if t {
break 'b_39dcc8;
} else {
'b_39dc90: {
'b_39dc8c: {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
if t {
break 'b_39dc8c;
} else {
c.cycles += 2;
let t = true;
c.g[A1][0] = 0u64;
if t {
break 'b_39dc90;
} else {
break 'b_39dc8c;
}
}
}
c.cycles += 1;
c.g[A1][0] = 0x1;
break 'b_39dc90;
}
c.cycles += 6;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039dca0);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039dca0)?;
let t = true;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
if t {
break 'b_39dcc8;
} else {
break 'b_39dca8;
}
}
}
}
c.cycles += 8;
c.lwc1(1, (c.g[SP][0] as u32));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
break 'b_39dcc8;
}
'b_39de10: {
'b_39ddb4: {
c.cycles += 5;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
c.c_eq_s(1, 0);
let t = !c.fcc();
c.g[V0][0] = 0x1;
if t {
break 'b_39ddb4;
} else {
'b_39dd90: {
'b_39dd80: {
'b_39dd54: {
'b_39dd20: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
if t {
break 'b_39dd20;
} else {
'b_39dd10: {
'b_39dd00: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_39dd00;
} else {
c.cycles += 2;
if c.g[V1][0] == 0u64 {
c.g[A0][0] = c.g[S0][0];
break 'b_39dd10;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
if t {
break 'b_39de10;
} else {
break 'b_39dd00;
}
}
}
}
c.cycles += 2;
if c.g[V1][0] == c.g[V0][0] {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
break 'b_39dd54;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
if t {
break 'b_39de10;
} else {
break 'b_39dd10;
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x0039dd18);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0039de68, 0x0039dd18)?;
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
if t {
break 'b_39de10;
} else {
break 'b_39dd20;
}
}
}
'b_39dd40: {
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_39dd40;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039dd40);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039dd40)?;
break 'b_39dd40;
}
}
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_39dd90;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
if t {
break 'b_39dd80;
} else {
break 'b_39dd54;
}
}
}
'b_39dd70: {
c.cycles += 2;
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0x1;
if t {
break 'b_39dd70;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039dd70);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039dd70)?;
break 'b_39dd70;
}
}
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0x1;
if t {
break 'b_39dd90;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
break 'b_39dd80;
}
}
c.cycles += 4;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039dd90);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039dd90)?;
break 'b_39dd90;
}
c.cycles += 9;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
c.g[A1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039ddac);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039ddac)?;
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
if t {
break 'b_39de10;
} else {
break 'b_39ddb4;
}
}
}
'b_39ddd4: {
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0x1;
if t {
break 'b_39ddd4;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039ddd4);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039ddd4)?;
break 'b_39ddd4;
}
}
c.cycles += 15;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
c.g[A1][0] = 0x1;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039ddf0);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039ddf0)?;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
c.g[A1][0] = 0x1;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039de0c);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039de0c)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
break 'b_39de10;
}
c.cycles += 13;
c.g[A1][0] = c.g[S1][0];
c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa8));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0039de44);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0039de44)?;
break 'b_39de44;
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
break 'b_39de48;
}
c.cycles += 8;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t;
return Ok(());