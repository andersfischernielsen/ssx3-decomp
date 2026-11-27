pub fn f_003d30a8__generated(c: &mut Cpu) {
'b_3d3128: {
'b_3d3124: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[V0][0] = 0x450000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = c.g[A1][0];
c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
c.g[S2][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_3d3124;
} else {
c.cycles += 4;
c.g[S3][0] = 0x450000;
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0xbd0));
if c.g[V0][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_3d3128;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003d30f0);
c.g[A0][0] = 0xffffffffffffffff;
c.pc = 0x003d5450; PATH_SelectMap(c);
if c.pc != 0x003d30f0 { return; }
if c.g[V0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_3d3128;
} else {
'b_3d310c: {
c.cycles += 4;
c.g[RA][0] = sx(0x003d3100);
c.g[A0][0] = c.g[S0][0];
c.pc = 0x003d5290; PATH_FindTrack(c);
if c.pc != 0x003d3100 { return; }
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = 0xfffffffffffffff8;
if t {
break 'b_3d310c;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
break 'b_3d310c;
}
}
c.cycles += 6;
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x003d3118);
c.g[A2][0] = c.g[S2][0];
c.pc = 0x002ae768; call_indirect(c, 0x002ae768);
if c.pc != 0x003d3118 { return; }
c.g[V0][0] = 0x450000;
c.sw((c.g[S3][0] as u32).wrapping_sub(0xbd0), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_sub(0xbe0), 0u64);
break 'b_3d3124;
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_3d3128;
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return;
}
