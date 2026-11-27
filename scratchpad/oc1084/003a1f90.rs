pub fn f_003a1f90__generated(c: &mut Cpu) {
'b_3a204c: {
c.cycles += 9;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
let t = c.g[V0][0] == 0u64;
c.g[S1][0] = c.g[A1][0];
if t {
break 'b_3a204c;
} else {
c.cycles += 4;
c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_3a204c;
} else {
'b_3a1ff4: {
c.cycles += 8;
c.g[A1][0] = 0x490000;
c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40a8));
c.g[A2][0] = 0x100;
c.g[RA][0] = sx(0x003a1fdc);
c.g[A3][0] = 0u64;
c.pc = 0x00317e30; call_indirect(c, 0x00317e30);
if c.pc != 0x003a1fdc { return; }
let t = (c.g[S1][0] as i64) <= 0;
c.g[S2][0] = c.g[V0][0];
if t {
break 'b_3a1ff4;
} else {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x003a1ff4);
c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
c.pc = 0x0041605c; call_indirect(c, 0x0041605c);
if c.pc != 0x003a1ff4 { return; }
break 'b_3a1ff4;
}
}
'b_3a2028: {
c.cycles += 5;
c.g[A2][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
if t {
break 'b_3a2028;
} else {
c.cycles += 8;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[S1][0] as u32)));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[RA][0] = sx(0x003a2028);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
c.pc = 0x0041605c; call_indirect(c, 0x0041605c);
if c.pc != 0x003a2028 { return; }
break 'b_3a2028;
}
}
'b_3a2040: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
if c.g[A0][0] == 0u64 {
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
break 'b_3a2040;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003a203c);
c.pc = 0x00317e98; call_indirect(c, 0x00317e98);
if c.pc != 0x003a203c { return; }
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
break 'b_3a2040;
}
}
c.cycles += 3;
c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.sh((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
break 'b_3a204c;
}
}
}
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t; return;
}

