pub fn f_003a2220__generated(c: &mut Cpu) {
c.cycles += 46;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A1][0] as u32));
c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
c.sub_s(12, 2, 12);
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A2][0] as u32));
c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x18));
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sub_s(2, 2, 12);
c.g[A1][0] = c.lw((c.g[A0][0] as u32));
c.g[A2][0] = c.g[SP][0];
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = 0x3;
c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.g[V0][0] = 0xffffffffffffffff;
c.lwc1(5, (c.g[A0][0] as u32).wrapping_add(0xc));
c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x18));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.sub_s(4, 6, 5);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.mul_s(3, 12, 0);
c.lwc1(8, (c.g[A0][0] as u32).wrapping_add(0x14));
c.mul_s(0, 0, 2);
c.lwc1(7, (c.g[A0][0] as u32).wrapping_add(0x10));
c.mul_s(12, 12, 4);
c.add_s(1, 1, 3);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.add_s(5, 5, 12);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
'l_3a22d8: loop {
c.cycles += 7;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = c.g[V1][0] != c.g[V0][0];
c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0xc0));
if t {
if c.cycles >= c.next_event { c.pc = 0x003a22d8; c.event(); } continue 'l_3a22d8;
} else {
c.cycles += 9;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
c.g[A3][0] = c.g[A3][0] & 0x4;
c.g[AT][0] = 0x437f0000;
c.mtc1(1, 2);
c.g[V1][0] = c.g[T1][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 3);
c.lwc1(0, (c.g[T0][0] as u32));
'l_3a2318: loop {
c.cycles += 20;
c.mul_s(0, 0, 4);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lwc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
c.mul_s(0, 0, 2);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.lwc1(0, (c.g[T0][0] as u32));
if c.cycles >= c.next_event { c.pc = 0x003a2318; c.event(); } continue 'l_3a2318;
} else {
'b_3a23b0: {
'b_3a23a0: {
c.cycles += 9;
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x54));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x80));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xb4));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x20));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x50));
let t = c.g[A3][0] == 0u64;
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x84));
if t {
break 'b_3a23a0;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[RA][0] = sx(0x003a2398);
c.g[A1][0] = c.g[T1][0];
c.pc = 0x0038e200; call_indirect(c, 0x0038e200);
if c.pc != 0x003a2398 { return; }
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
if t {
break 'b_3a23b0;
} else {
break 'b_3a23a0;
}
}
}
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[RA][0] = sx(0x003a23ac);
c.g[A1][0] = c.g[T1][0];
c.pc = 0x0038e118; call_indirect(c, 0x0038e118);
if c.pc != 0x003a23ac { return; }
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
break 'b_3a23b0;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.pc = t; return;
}
}
}
}
}

