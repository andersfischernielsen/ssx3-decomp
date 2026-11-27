pub fn f_003a1c80__generated(c: &mut Cpu) {
c.cycles += 28;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003a1c9c);
c.g[S1][0] = c.g[A1][0];
c.pc = 0x003a18c0; cUITextScroll_ClearLines(c);
if c.pc != 0x003a1c9c { return; }
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), 0u64);
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003a1cb8);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.pc = t; call_indirect(c, t);
if c.pc != 0x003a1cb8 { return; }
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = 0u64;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003a1cd0);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.pc = t; call_indirect(c, t);
if c.pc != 0x003a1cd0 { return; }
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003a1cdc);
c.g[A1][0] = c.g[S1][0];
c.pc = 0x003a19f8; cUITextScroll_SetString(c);
if c.pc != 0x003a1cdc { return; }
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return;
}

