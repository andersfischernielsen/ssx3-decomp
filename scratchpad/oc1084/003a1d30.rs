pub fn f_003a1d30__generated(c: &mut Cpu) {
'b_3a1ee4: {
c.cycles += 25;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S0][0] = c.g[A1][0];
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S4][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A1][0] = 0x1;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003a1d80);
c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.pc = t; call_indirect(c, t);
if c.pc != 0x003a1d80 { return; }
c.g[RA][0] = sx(0x003a1d88);
c.g[A0][0] = c.g[S4][0];
c.pc = 0x003a12d0; cUITextScroll_GetFont(c);
if c.pc != 0x003a1d88 { return; }
c.g[S7][0] = c.g[V0][0];
if c.g[S7][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_3a1ee4;
} else {
'b_3a1ebc: {
c.cycles += 20;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003a1da0);
c.g[S5][0] = 0xffffffffffffffff;
c.pc = 0x003dbca0; call_indirect(c, 0x003dbca0);
if c.pc != 0x003a1da0 { return; }
c.g[S2][0] = 0u64;
c.g[S6][0] = c.g[V0][0];
c.g[A1][0] = 0x490000;
c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4078));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[A2][0] = 0x100;
c.mtc1(0, 20);
c.g[RA][0] = sx(0x003a1dc8);
c.g[A3][0] = 0u64;
c.pc = 0x00317e30; call_indirect(c, 0x00317e30);
if c.pc != 0x003a1dc8 { return; }
c.g[S1][0] = 0u64;
c.g[S3][0] = c.g[V0][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003a1ddc);
c.g[A0][0] = c.g[S3][0];
c.pc = 0x003dbe20; call_indirect(c, 0x003dbe20);
if c.pc != 0x003a1ddc { return; }
let t = (c.g[S6][0] as i64) <= 0;

if t {
break 'b_3a1ebc;
} else {
c.cycles += 3;
c.g[FP][0] = 0xffffffffffffffff;
c.g[V0][0] = sx((c.g[S2][0] as u32) << 1);
'l_3a1df0: loop {
'b_3a1eb0: {
'b_3a1e40: {
'b_3a1e3c: {
c.cycles += 6;
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[S0][0] = c.g[V0][0];
c.g[A0][0] = c.lhu((c.g[V1][0] as u32));
c.g[V0][0] = 0x5c;
let t = c.g[A0][0] != c.g[V0][0];
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
if t {
break 'b_3a1e3c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
let t = c.g[V0][0] != c.g[A0][0];
c.g[A0][0] = c.g[S7][0];
if t {
break 'b_3a1e40;
} else {
c.cycles += 10;
c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
c.sh((c.g[V1][0] as u32), 0u64);
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
c.g[A0][0] = c.g[S4][0];
c.g[RA][0] = sx(0x003a1e30);
c.g[S5][0] = 0xffffffffffffffff;
c.pc = 0x003a1bf8; cUITextScroll_AddLine(c);
if c.pc != 0x003a1e30 { return; }
c.mtc1(0, 20);
let t = true;
c.g[S1][0] = c.g[S2][0];
if t {
break 'b_3a1eb0;
} else {
break 'b_3a1e3c;
}
}
}
}
c.cycles += 1;
c.g[A0][0] = c.g[S7][0];
break 'b_3a1e40;
}
'b_3a1eac: {
c.cycles += 12;
c.g[RA][0] = sx(0x003a1e48);
c.g[A1][0] = c.lhu((c.g[S0][0] as u32));
c.pc = 0x00392c60; Font_GetCharWidthW(c);
if c.pc != 0x003a1e48 { return; }
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x50));
c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x60));
c.mul_s(0, 0, 1);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
c.g[V0][0] = c.g[V0][0] ^ 0x20;
c.add_s(20, 20, 0);
c.c_lt_s(2, 20);
let t = !c.fcc();
if c.g[V0][0] == 0 { c.g[S5][0] = c.g[S2][0]; }
if t {
break 'b_3a1eac;
} else {
c.cycles += 5;
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[S5][0] as i64)) as u64;
if c.g[V0][0] == 0 { c.g[S5][0] = c.g[V1][0]; }
let t = (c.g[S5][0] as i64) <= 0;
c.g[V0][0] = sx((c.g[S5][0] as u32) << 1);
if t {
break 'b_3a1eac;
} else {
c.cycles += 10;
c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[S2][0] = c.g[S5][0];
c.sh((c.g[V0][0] as u32), 0u64);
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.mtc1(0, 20);
c.g[RA][0] = sx(0x003a1ea4);
c.g[A0][0] = c.g[S4][0];
c.pc = 0x003a1bf8; cUITextScroll_AddLine(c);
if c.pc != 0x003a1ea4 { return; }
c.g[S5][0] = 0xffffffffffffffff;
c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
break 'b_3a1eac;
}
}
}
c.cycles += 1;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
break 'b_3a1eb0;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S6][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[S2][0] as u32) << 1);
if t {
if c.cycles >= c.next_event { c.pc = 0x003a1df0; c.event(); } continue 'l_3a1df0;
} else {
break 'b_3a1ebc;
}
}
}
}
'b_3a1ed0: {
c.cycles += 2;
let t = c.g[S1][0] == c.g[S2][0];
c.g[A0][0] = c.g[S4][0];
if t {
break 'b_3a1ed0;
} else {
c.cycles += 3;
c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
c.g[RA][0] = sx(0x003a1ed0);
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.pc = 0x003a1bf8; cUITextScroll_AddLine(c);
if c.pc != 0x003a1ed0 { return; }
break 'b_3a1ed0;
}
}
c.cycles += 2;
let t = c.g[S3][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_3a1ee4;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003a1ee0);
c.g[A0][0] = c.g[S3][0];
c.pc = 0x00317e98; call_indirect(c, 0x00317e98);
if c.pc != 0x003a1ee0 { return; }
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_3a1ee4;
}
}
}
c.cycles += 12;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return;
}

