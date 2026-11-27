'b_2df2ec: {
c.cycles += 23;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
c.g[V0][0] = 0x490000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ca0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[S1][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[T0][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[A0][0] = 0x460000;
c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
c.g[A3][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
c.g[V1][0] = 0x490000;
c.g[A1][0] = c.lw((c.g[S1][0] as u32));
c.g[T3][0] = c.g[V1][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7c58));
c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
let t = c.g[T0][0] != 0u64;
c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
if t {
break 'b_2df2ec;
} else {
c.cycles += 4;
c.g[A1][0] = c.g[A2][0];
c.g[V1][0] = c.g[SP][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
'l_2df234: loop {
c.cycles += 11;
c.g[A0][0] = c.ld((c.g[A1][0] as u32));
c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[T4][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
c.sd((c.g[V1][0] as u32), c.g[A0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
let t = c.g[A1][0] != c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x002df234); } continue 'l_2df234;
} else {
c.cycles += 14;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca0));
c.g[A2][0] = c.ld((c.g[A1][0] as u32));
c.sd((c.g[V1][0] as u32), c.g[A2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = c.g[T1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
'l_2df298: loop {
c.cycles += 11;
c.g[V0][0] = c.ld((c.g[A2][0] as u32));
c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
let t = c.g[A2][0] != c.g[T2][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x002df298); } continue 'l_2df298;
} else {
c.cycles += 10;
c.g[V0][0] = c.ld((c.g[A2][0] as u32));
c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
break 'b_2df2ec;
}
}
}
}
}
}
'b_2df348: {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[S3][0] = c.g[T0][0] & 0x1;
let t = c.g[A1][0] == 0u64;
c.g[S2][0] = c.g[T0][0] & 0x2;
if t {
break 'b_2df348;
} else {
'b_2df340: {
c.cycles += 6;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
c.g[V0][0] = 0x210;
c.mult(0, 4, 3, 2);
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
if c.g[A1][0] == c.g[S0][0] {
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2df340;
} else {
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x210));
'l_2df318: loop {
c.cycles += 9;
c.g[A1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002df330);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002df330)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
if c.g[V0][0] != c.g[S0][0] {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x210));
if event_due(c) { deliver_event_here(c, 0x002df318); } continue 'l_2df318;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2df340;
}
}
}
}
c.cycles += 2;
c.g[RA][0] = sx(0x002df348);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
call_ptr(c, 0x00317e98, 0x002df348)?;
break 'b_2df348;
}
}
'b_2df360: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
break 'b_2df360;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x002df35c);
call_ptr(c, 0x00317e98, 0x002df35c)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
break 'b_2df360;
}
}
'b_2df370: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;

if t {
break 'b_2df370;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x002df370);
call_ptr(c, 0x00317e98, 0x002df370)?;
break 'b_2df370;
}
}
'b_2df384: {
c.cycles += 2;
let t = c.g[S2][0] == 0u64;

if t {
break 'b_2df384;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[RA][0] = sx(0x002df384);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0011b978, 0x002df384)?;
break 'b_2df384;
}
}
'b_2df398: {
c.cycles += 2;
let t = c.g[S3][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
if t {
break 'b_2df398;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x002df394);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x00317e50, 0x002df394)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
break 'b_2df398;
}
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
c.pc = t; return Ok(());