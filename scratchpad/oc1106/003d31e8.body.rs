pub fn f_003d31e8__generated(c: &mut Cpu) {
'b_3d3288: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[V0][0] = 0x450000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = c.g[A2][0];
c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
c.g[S2][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_3d3288;
} else {
c.cycles += 4;
c.g[V1][0] = 0x450000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
let t = c.g[V0][0] != 0u64;
c.g[S3][0] = c.g[V1][0];
if t {
break 'b_3d3288;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003d3230);
c.g[A0][0] = 0xffffffffffffffff;
c.pc = 0x003d5450; PATH_SelectMap(c);
if c.pc != 0x003d3230 { return; }
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x510000;
if t {
break 'b_3d3288;
} else {
c.cycles += 6;
c.g[V1][0] = 0xf000000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
c.g[S0][0] = c.g[S0][0] & c.g[V1][0];
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A0][0] = 0x3;
'l_3d3250: loop {
'b_3d3274: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_3d3274;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
let t = c.g[V0][0] == 0u64;

if t {
break 'b_3d3274;
} else {
c.cycles += 2;
c.sw((c.g[V1][0] as u32).wrapping_sub(0x4), c.g[S2][0]);
c.sw((c.g[V1][0] as u32), c.g[S1][0]);
break 'b_3d3274;
}
}
}
c.cycles += 2;
let t = (c.g[A0][0] as i64) >= 0;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x928));
if t {
if c.cycles >= c.next_event { c.pc = 0x003d3250; c.event(); } continue 'l_3d3250;
} else {
c.cycles += 3;
c.g[V0][0] = 0x450000;
c.sw((c.g[S3][0] as u32).wrapping_sub(0xbd0), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_sub(0xbe0), 0u64);
break 'b_3d3288;
}
}
}
}
}
}
c.cycles += 7;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return;
}
