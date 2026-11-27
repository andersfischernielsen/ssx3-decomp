pub fn f_003d5290__generated(c: &mut Cpu) {
c.cycles += 14;
c.g[A1][0] = 0x510000;
c.g[A2][0] = 0xff0000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7610));
c.g[V0][0] = 0xfffffffff0000000;
c.g[V1][0] = 0xf000000;
c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x58));
c.g[T2][0] = c.g[A0][0] & c.g[V0][0];
c.g[T1][0] = c.g[A0][0] & c.g[V1][0];
c.g[A2][0] = c.g[A2][0] | 0xffff;
c.g[T0][0] = 0u64;
c.g[A0][0] = c.g[A0][0] & c.g[A2][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34));
c.g[A3][0] = 0u64;
'l_3d52c8: loop {
'b_3d5318: {
'b_3d5314: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x4));
if t {
break 'b_3d5314;
} else {
c.cycles += 3;
c.g[V0][0] = c.g[V1][0] & c.g[T2][0];
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[V1][0] & c.g[T1][0];
if t {
break 'b_3d5314;
} else {
c.cycles += 2;
if c.g[V0][0] == 0u64 {
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
break 'b_3d5318;
} else {
c.cycles += 2;
c.g[A2][0] = 0u64;
c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T3][0] as u32)));
'l_3d52f0: loop {
c.cycles += 4;
c.g[V0][0] = sx((c.g[A0][0] as u32) >> ((c.g[A2][0] as u32) & 31));
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
if t {
c.cycles += 3;
c.g[V0][0] = (c.g[A2][0] < 0x18u64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
if t {
if c.cycles >= c.next_event { c.pc = 0x003d52f0; c.event(); } continue 'l_3d52f0;
} else {
break 'b_3d5314;
}
} else {
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.pc = t; return;
}
}
}
}
}
}
c.cycles += 1;
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
break 'b_3d5318;
}
c.cycles += 4;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x928));
c.g[V0][0] = (c.g[T0][0] < 0x4u64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x928));
if t {
if c.cycles >= c.next_event { c.pc = 0x003d52c8; c.event(); } continue 'l_3d52c8;
} else {
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[V0][0] = 0u64;
c.pc = t; return;
}
}
}
