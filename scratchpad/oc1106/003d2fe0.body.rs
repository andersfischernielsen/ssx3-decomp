pub fn f_003d2fe0__generated(c: &mut Cpu) {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003d2ff0);
c.g[A0][0] = 0xffffffffffffffff;
c.pc = 0x003d2a68; PATH_StopTracks(c);
if c.pc != 0x003d2ff0 { return; }
c.g[V1][0] = 0x450000;
c.g[A0][0] = 0x450000;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.g[V0][0] = 0u64;
c.sb((c.g[V1][0] as u32).wrapping_sub(0xbd4), 0u64);
c.sw((c.g[A0][0] as u32).wrapping_sub(0xbd0), 0u64);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.pc = t; return;
}
