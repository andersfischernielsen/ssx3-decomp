'b_3cb490: {
'b_3cb374: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S2][0] = c.g[A3][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
let t = c.g[A0][0] == 0u64;
c.g[S3][0] = c.g[A1][0];
if t {
break 'b_3cb374;
} else {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[T0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
c.g[A2][0] = c.g[S2][0];
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x003cb36c);
c.g[A3][0] = c.g[S1][0];
c.pc = t; call_indirect(c, t);
if c.pc != 0x003cb36c { return; }
if (c.g[V0][0] as i64) <= 0 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_3cb490;
} else {
break 'b_3cb374;
}
}
}
'b_3cb488: {
c.cycles += 4;
c.g[A3][0] = c.g[S1][0];
c.g[A2][0] = c.g[S2][0];
let t = (c.g[S3][0] as i64) <= 0;
c.g[T3][0] = 0u64;
if t {
break 'b_3cb488;
} else {
c.cycles += 1;
'l_3cb388: loop {
'b_3cb3b0: {
'b_3cb3ac: {
c.cycles += 5;
c.g[A0][0] = c.lh((c.g[A3][0] as u32));
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x30));
if t {
break 'b_3cb3ac;
} else {
c.cycles += 4;
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T2][0] = c.g[A0][0];
if t {
break 'b_3cb3b0;
} else {
break 'b_3cb3ac;
}
}
}
c.cycles += 1;
c.g[T2][0] = c.g[V1][0];
break 'b_3cb3b0;
}
'b_3cb3dc: {
c.cycles += 6;
c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x2));
c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V1][0] != 0u64;
c.g[T1][0] = c.g[V0][0];
if t {
break 'b_3cb3dc;
} else {
c.cycles += 4;
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = c.g[V1][0];
if t {
break 'b_3cb3dc;
} else {
c.cycles += 1;
c.g[T1][0] = c.g[A0][0];
break 'b_3cb3dc;
}
}
}
'b_3cb408: {
c.cycles += 6;
c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V1][0] != 0u64;
c.g[T0][0] = c.g[V0][0];
if t {
break 'b_3cb408;
} else {
c.cycles += 4;
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T0][0] = c.g[V1][0];
if t {
break 'b_3cb408;
} else {
c.cycles += 1;
c.g[T0][0] = c.g[A0][0];
break 'b_3cb408;
}
}
}
'b_3cb434: {
c.cycles += 6;
c.g[V1][0] = sx((c.g[A1][0] as u32) << 16);
c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x6));
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.g[V1][0];
if t {
break 'b_3cb434;
} else {
c.cycles += 4;
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.g[V1][0];
if t {
break 'b_3cb434;
} else {
c.cycles += 1;
c.g[A1][0] = c.g[A0][0];
break 'b_3cb434;
}
}
}
c.cycles += 21;
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
c.g[A0][0] = ((c.g[T3][0] as i64) < (c.g[S3][0] as i64)) as u64;
c.mult(0, 3, 10, 3);
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 7) as u32);
c.sh((c.g[A2][0] as u32), c.g[V1][0]);
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
c.mult(0, 2, 9, 2);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
c.sh((c.g[A2][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
c.mult(0, 3, 8, 3);
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 7) as u32);
c.sh((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
c.mult(0, 2, 5, 2);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
c.sh((c.g[A2][0] as u32).wrapping_add(0x6), c.g[V0][0]);
let t = c.g[A0][0] != 0u64;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
if t {
if c.cycles >= c.next_event { c.pc = 0x003cb388; c.event(); } continue 'l_3cb388;
} else {
break 'b_3cb488;
}
}
}
}
c.cycles += 2;
c.g[V0][0] = c.g[S3][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_3cb490;
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return;
