pub fn f_003a4900__generated(c: &mut Cpu) {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.g[V0][0] = 0x490000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bd8));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
c.sw((c.g[S0][0] as u32), c.g[S0][0]);
c.g[RA][0] = sx(0x003a492c);
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.pc = 0x003a6548; UIObjectFlags_ctor(c);
if c.pc != 0x003a492c { return; }
c.g[V0][0] = c.g[S0][0];
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return;
}

