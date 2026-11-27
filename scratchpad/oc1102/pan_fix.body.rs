'b_3c37f8: {
'b_3c37c0: {
'b_3c37a4: {
c.cycles += 2;
let t = (c.g[A3][0] as i64) >= 0;

if t {
break 'b_3c37a4;
} else {
c.cycles += 2;
let t = (c.g[T0][0] as i64) >= 0;
c.g[V0][0] = 0xffffffffffffffff;
if t {
break 'b_3c37c0;
} else {
c.cycles += 5;
c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
c.g[T1][0] = 0x2;
c.g[V0][0] = 0x1;
let t = true;
if c.g[V1][0] != 0 { c.g[T1][0] = c.g[V0][0]; }
if t {
break 'b_3c37f8;
} else {
break 'b_3c37a4;
}
}
}
}
'b_3c37bc: {
c.cycles += 2;
let t = (c.g[T0][0] as i64) < 0;
c.g[T1][0] = 0x1;
if t {
break 'b_3c37bc;
} else {
c.cycles += 4;
c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
c.g[V0][0] = 0x2;
let t = true;
if c.g[V1][0] != 0 { c.g[T1][0] = c.g[V0][0]; }
if t {
break 'b_3c37f8;
} else {
break 'b_3c37bc;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0xffffffffffffffff;
break 'b_3c37c0;
}
'b_3c37e8: {
c.cycles += 4;
c.g[V1][0] = sx(0u32.wrapping_sub((c.g[A3][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
let t = (c.g[T0][0] as i64) >= 0;
if c.g[V0][0] != 0 { c.g[V1][0] = c.g[A3][0]; }
if t {
break 'b_3c37e8;
} else {
c.cycles += 4;
c.g[V0][0] = sx(0u32.wrapping_sub((c.g[T0][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = 0x4;
if t {
break 'b_3c37f8;
} else {
c.cycles += 2;
let t = true;
c.g[T1][0] = 0x3;
if t {
break 'b_3c37f8;
} else {
break 'b_3c37e8;
}
}
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = 0x4;
if t {
break 'b_3c37f8;
} else {
c.cycles += 1;
c.g[T1][0] = 0x3;
break 'b_3c37f8;
}
}
'b_3c3888: {
'b_3c3880: {
'b_3c3848: {
c.cycles += 5;
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4000));
c.g[V0][0] = 0x7fff;
c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = 0u64;
if t {
break 'b_3c3848;
} else {
'b_3c382c: {
c.cycles += 7;
c.lwc1(1, (c.g[A1][0] as u32));
c.g[V1][0] = 0x4;
c.lwc1(0, (c.g[A2][0] as u32));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.mov_s(2, 1);
if t {
break 'b_3c382c;
} else {
c.cycles += 1;
c.g[V1][0] = 0x3;
break 'b_3c382c;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x3;
if t {
break 'b_3c3888;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[A0][0] = 0x2;
if t {
break 'b_3c3888;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = c.g[T1][0] ^ 0x2;
if t {
break 'b_3c3880;
} else {
break 'b_3c3848;
}
}
}
}
}
'b_3c3868: {
c.cycles += 7;
c.lwc1(1, (c.g[A1][0] as u32));
c.g[V1][0] = 0x1;
c.lwc1(0, (c.g[A2][0] as u32));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.mov_s(2, 1);
if t {
break 'b_3c3868;
} else {
c.cycles += 1;
c.g[V1][0] = 0x2;
break 'b_3c3868;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
let t = c.g[V0][0] != 0u64;

if t {
break 'b_3c3888;
} else {
c.cycles += 3;
c.g[A0][0] = 0x2;
let t = c.g[V1][0] != c.g[A0][0];
c.g[V1][0] = c.g[T1][0] ^ 0x3;
if t {
break 'b_3c3888;
} else {
break 'b_3c3880;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0x1;
if c.g[V1][0] == 0 { c.g[A0][0] = c.g[V0][0]; }
break 'b_3c3888;
}
'b_3c3898: {
c.cycles += 2;
let t = (c.g[A3][0] as i64) >= 0;

if t {
break 'b_3c3898;
} else {
c.cycles += 2;
c.neg_s(0, 2);
c.swc1(0, (c.g[A1][0] as u32));
break 'b_3c3898;
}
}
'b_3c38ac: {
c.cycles += 2;
let t = (c.g[T0][0] as i64) >= 0;
c.g[V0][0] = 0x1;
if t {
break 'b_3c38ac;
} else {
c.cycles += 3;
c.lwc1(0, (c.g[A2][0] as u32));
c.neg_s(0, 0);
c.swc1(0, (c.g[A2][0] as u32));
break 'b_3c38ac;
}
}
c.cycles += 2;
let t = c.g[A0][0] != c.g[V0][0];
c.g[V0][0] = 0x2;
if t {
'b_3c38d8: {
c.cycles += 2;
let t = c.g[A0][0] != c.g[V0][0];

if t {
break 'b_3c38d8;
} else {
c.cycles += 3;
c.lwc1(0, (c.g[A2][0] as u32));
c.neg_s(0, 0);
c.swc1(0, (c.g[A2][0] as u32));
break 'b_3c38d8;
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.pc = t; return;
} else {
c.cycles += 4;
c.lwc1(0, (c.g[A1][0] as u32));
c.neg_s(0, 0);
let t = c.g[RA][0] as u32;
c.swc1(0, (c.g[A1][0] as u32));
c.pc = t; return;
}
