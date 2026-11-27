pub fn f_003a2a00__generated(c: &mut Cpu) {
'b_3a2ad0: {
c.cycles += 50;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A1][0] as u32));
c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.g[A0][0];
c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A2][0] as u32));
c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x1f));
c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x18));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x2f));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
c.ldr(10, (c.g[A3][0] as u32));
c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0xf));
c.ldr(2, (c.g[A3][0] as u32).wrapping_add(0x8));
c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x47));
c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x4f));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x48));
c.g[A2][0] = c.g[SP][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
c.g[A1][0] = c.g[T0][0] & 0x1;
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.lwc1(6, (c.g[T1][0] as u32).wrapping_add(0xc));
c.lwc1(4, (c.g[T1][0] as u32).wrapping_add(0x18));
c.lwc1(5, (c.g[T1][0] as u32).wrapping_add(0x10));
let t = c.g[A1][0] == 0u64;
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x14));
if t {
break 'b_3a2ad0;
} else {
c.cycles += 2;
c.mov_s(5, 3);
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_3a2ad0;
}
}
'b_3a2ae4: {
c.cycles += 3;
c.g[V0][0] = c.g[T0][0] & 0x2;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.g[T0][0] & 0x4;
if t {
break 'b_3a2ae4;
} else {
c.cycles += 2;
c.mov_s(6, 4);
c.lwc1(4, (c.g[T1][0] as u32).wrapping_add(0xc));
break 'b_3a2ae4;
}
}
c.cycles += 3;
c.g[V0][0] = 0x3;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V1][0] = 0xffffffffffffffff;
'l_3a2af0: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[V1][0];
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
if t {
if c.cycles >= c.next_event { c.pc = 0x003a2af0; c.event(); } continue 'l_3a2af0;
} else {
c.cycles += 9;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 8);
c.g[V1][0] = c.g[A1][0];
c.g[AT][0] = 0x437f0000;
c.mtc1(1, 2);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 7);
c.lwc1(0, (c.g[T2][0] as u32));
'l_3a2b30: loop {
c.cycles += 20;
c.mul_s(0, 0, 8);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[T2][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
c.mul_s(0, 0, 2);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x8));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.lwc1(0, (c.g[T2][0] as u32));
if c.cycles >= c.next_event { c.pc = 0x003a2b30; c.event(); } continue 'l_3a2b30;
} else {
'b_3a2bc0: {
'b_3a2bb4: {
c.cycles += 9;
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x84));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x54));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x80));
let t = c.g[T0][0] == 0u64;
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xb4));
if t {
break 'b_3a2bb4;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003a2bac);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.pc = 0x0038e468; call_indirect(c, 0x0038e468);
if c.pc != 0x003a2bac { return; }
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
if t {
break 'b_3a2bc0;
} else {
break 'b_3a2bb4;
}
}
}
c.cycles += 3;
c.g[RA][0] = sx(0x003a2bbc);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.pc = 0x0038e2e8; call_indirect(c, 0x0038e2e8);
if c.pc != 0x003a2bbc { return; }
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
break 'b_3a2bc0;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
c.pc = t; return;
}
}
}
}
}

