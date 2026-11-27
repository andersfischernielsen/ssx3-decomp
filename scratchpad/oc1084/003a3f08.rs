pub fn f_003a3f08__generated(c: &mut Cpu) {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.g[V0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return;
}

