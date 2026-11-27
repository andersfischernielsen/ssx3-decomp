pub fn f_003d3140__generated(c: &mut Cpu) {
'b_3d31c8: {
'b_3d3198: {
'b_3d3190: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[V0][0] = 0x450000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S1][0] = c.g[A1][0];
c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
c.g[S3][0] = 0u64;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[V1][0] == 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_3d3190;
} else {
c.cycles += 4;
c.g[S2][0] = 0x450000;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0xbd0));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0xffffffffffffffee;
if t {
break 'b_3d31c8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003d3188);
c.g[A0][0] = 0xffffffffffffffff;
c.pc = 0x003d5450; PATH_SelectMap(c);
if c.pc != 0x003d3188 { return; }
let t = c.g[V0][0] != 0u64;

if t {
break 'b_3d3198;
} else {
break 'b_3d3190;
}
}
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = 0xffffffffffffffee;
if t {
break 'b_3d31c8;
} else {
break 'b_3d3198;
}
}
'b_3d31bc: {
c.cycles += 4;
c.g[RA][0] = sx(0x003d31a0);
c.g[A0][0] = c.g[S0][0];
c.pc = 0x003d5290; PATH_FindTrack(c);
if c.pc != 0x003d31a0 { return; }
if c.g[V0][0] == 0u64 {
c.g[V0][0] = 0x450000;
break 'b_3d31bc;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
c.g[RA][0] = sx(0x003d31b4);
c.g[A1][0] = c.g[S1][0];
c.pc = 0x002ae328; call_indirect(c, 0x002ae328);
if c.pc != 0x003d31b4 { return; }
c.g[S3][0] = c.g[V0][0];
c.g[V0][0] = 0x450000;
break 'b_3d31bc;
}
}
c.cycles += 3;
c.sw((c.g[S2][0] as u32).wrapping_sub(0xbd0), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_sub(0xbe0), 0u64);
c.g[V0][0] = c.g[S3][0];
break 'b_3d31c8;
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
