pub fn f_003a1360__generated(c: &mut Cpu) {
'b_3a13d4: {
c.cycles += 7;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
if c.g[A1][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3a13d4;
} else {
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
if c.g[A2][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3a13d4;
} else {
'b_3a13b8: {
c.cycles += 7;
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x003a13a8);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.pc = t; call_indirect(c, t);
if c.pc != 0x003a13a8 { return; }
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.g[V0][0];
if t {
break 'b_3a13b8;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x003a13b8);
c.g[A0][0] = c.g[S0][0];
c.pc = 0x003a1cf0; cUITextScroll_ReplaceWideString(c);
if c.pc != 0x003a13b8 { return; }
break 'b_3a13b8;
}
}
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = 0x1;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003a13d0);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.pc = t; call_indirect(c, t);
if c.pc != 0x003a13d0 { return; }
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3a13d4;
}
}
}
c.cycles += 3;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return;
}

