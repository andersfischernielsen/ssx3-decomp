'b_38d60c: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[T5][0] = 0u64;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.g[T0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(30, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
if t {
break 'b_38d60c;
} else {
c.cycles += 18;
c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
c.g[T0][0] = 0xfff0000;
c.g[T9][0] = 0x60000;
c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[FP][0] = 0xffffffffffffffcf;
c.g[S7][0] = 0x10000000;
c.g[S7][0] = c.g[S7][0] | 0x6;
c.g[S6][0] = 0x20000000;
c.g[S5][0] = 0x6c040000;
c.g[S5][0] = c.g[S5][0] | 0x8080;
c.g[S5][0] = c.g[S5][0] << 32;
c.g[S4][0] = 0x14000000;
c.g[T0][0] = c.g[T0][0] | 0xffff;
c.g[T9][0] = c.g[T9][0] | 0x8000;
c.g[S3][0] = 0xffffffffffffffe0;
c.g[T8][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1440));
c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x13f0));
'l_38d4d0: loop {
'b_38d5f4: {
c.cycles += 4;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = 0x30000000;
if t {
break 'b_38d5f4;
} else {
'b_38d5e8: {
c.cycles += 37;
c.g[A0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x94));
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
c.g[T6][0] = c.lw((c.g[T1][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
c.g[V1][0] = c.g[T6][0] | c.g[A2][0];
c.g[A0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
c.g[V0][0] = c.g[V0][0] | 0x30;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32), c.g[S7][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S6][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
c.lqc2(1, (c.g[T8][0] as u32));
c.lqc2(2, (c.g[T8][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[T8][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[T8][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A1][0] as u32));
c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
c.sd((c.g[V1][0] as u32).wrapping_add(0x60), 0u64);
c.g[S0][0] = 0x1;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
c.g[T4][0] = c.g[V1][0] & c.g[T0][0];
c.sw((c.g[T1][0] as u32), c.g[T4][0]);
c.g[A3][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x18f0));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
if t {
break 'b_38d5e8;
} else {
c.cycles += 29;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T9][0] as u32)));
c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
c.g[A0][0] = c.g[A0][0] | c.g[A2][0];
c.g[V1][0] = c.g[V1][0] & 0x1f;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.ldl(3, (c.g[T2][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T2][0] as u32));
c.ldl(4, (c.g[T2][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[T2][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[V0][0] as u32));
c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[S0][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T6][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
break 'b_38d5e8;
}
}
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[T1][0] as u32), c.g[V0][0]);
break 'b_38d5f4;
}
}
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = (c.g[T5][0] < c.g[V0][0]) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0xa0));
if t {
if event_due(c) { deliver_event_here(c, 0x0038d4d0); } continue 'l_38d4d0;
} else {
break 'b_38d60c;
}
}
}
}
c.cycles += 11;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lq(30, (c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.pc = t; return;