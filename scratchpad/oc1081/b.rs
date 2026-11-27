'b_3a15dc: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
c.g[S1][0] = c.g[A0][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.mov_s(21, 13);
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] != 0u64;
c.mov_s(20, 12);
if t {
break 'b_3a15dc;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003a15dc);
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x003a15dc)?;
break 'b_3a15dc;
}
}
'b_3a189c: {
c.cycles += 5;
c.g[RA][0] = sx(0x003a15e4);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x003a12d0, 0x003a15e4)?;
c.g[S2][0] = c.g[V0][0];
let t = c.g[S2][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
if t {
break 'b_3a189c;
} else {
'b_3a16b8: {
'b_3a16a0: {
c.cycles += 16;
c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x50));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x54));
c.swc1(3, (c.g[SP][0] as u32));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x34));
c.mul_s(0, 0, 3);
c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x38));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.mul_s(2, 2, 1);
c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x3c));
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.g[V0][0] = c.g[V0][0] & 0x1;
if c.g[V0][0] == 0u64 {
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
break 'b_3a16a0;
} else {
c.cycles += 28;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x7f));
c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x78));
c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x87));
c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0x80));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[AT][0] = 0x40000000;
c.mtc1(1, 1);
c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(7, (c.g[S2][0] as u32).wrapping_add(0x57));
c.sdr(7, (c.g[S2][0] as u32).wrapping_add(0x50));
c.sdl(8, (c.g[S2][0] as u32).wrapping_add(0x5f));
c.sdr(8, (c.g[S2][0] as u32).wrapping_add(0x58));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sdl(4, (c.g[S2][0] as u32).wrapping_add(0x2f));
c.sdr(4, (c.g[S2][0] as u32).wrapping_add(0x28));
let t = true;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
if t {
break 'b_3a16b8;
} else {
break 'b_3a16a0;
}
}
}
c.cycles += 6;
c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x2f));
c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x28));
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
break 'b_3a16b8;
}
'b_3a1884: {
'b_3a1880: {
c.cycles += 35;
c.g[A1][0] = 0xfffffffffffffc1f;
c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x15));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.g[V0][0] & 0x1f;
c.g[AT][0] = 0x40400000;
c.mtc1(1, 4);
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x9));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[V0][0] = c.g[V0][0] & 0x3e0;
c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x44));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x48));
c.add_s(0, 0, 20);
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x54));
c.add_s(1, 1, 21);
c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x64));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
c.cvt_s_w(0, 0);
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x34));
c.mul_s(0, 0, 1);
c.mul_s(2, 2, 0);
c.add_s(20, 2, 4);
c.div_s(3, 3, 20);
c.cvt_w_s(0, 3);
c.mfc1(2, 0);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
c.g[S4][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
let t = (c.g[S4][0] as i64) <= 0;
c.g[S3][0] = 0u64;
if t {
break 'b_3a1880;
} else {
c.cycles += 1;
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
'l_3a1748: loop {
c.cycles += 6;
c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[A0][0] = c.g[V0][0] & 0xffff;
c.g[V1][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
if c.g[V1][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
break 'b_3a1884;
} else {
'b_3a17c8: {
'b_3a17a4: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_3a17a4;
} else {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc4));
let t = c.g[A0][0] != c.g[V0][0];

if t {
break 'b_3a17a4;
} else {
c.cycles += 10;
c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0xaf));
c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0xa8));
c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0xb7));
c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0xb0));
c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x47));
c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x40));
c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0x4f));
c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0x48));
let t = true;
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
if t {
break 'b_3a17c8;
} else {
break 'b_3a17a4;
}
}
}
}
c.cycles += 9;
c.ldl(7, (c.g[S1][0] as u32).wrapping_add(0x23));
c.ldr(7, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.ldl(8, (c.g[S1][0] as u32).wrapping_add(0x2b));
c.ldr(8, (c.g[S1][0] as u32).wrapping_add(0x24));
c.sdl(7, (c.g[S2][0] as u32).wrapping_add(0x47));
c.sdr(7, (c.g[S2][0] as u32).wrapping_add(0x40));
c.sdl(8, (c.g[S2][0] as u32).wrapping_add(0x4f));
c.sdr(8, (c.g[S2][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
break 'b_3a17c8;
}
c.cycles += 46;
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
c.g[A3][0] = 0u64;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x54));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x50));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.g[S2][0];
c.g[S0][0] = c.lw((c.g[V0][0] as u32));
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
c.g[RA][0] = sx(0x003a17fc);
c.g[A1][0] = c.g[S0][0];
call_ptr(c, 0x003921f0, 0x003a17fc)?;
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x14));
c.cvt_s_w(1, 1);
c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x34));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x54));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
c.g[A0][0] = c.g[S1][0];
c.mul_s(1, 1, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0x48), 0u64);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.mul_s(0, 0, 1);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[T0][0]);
c.g[RA][0] = sx(0x003a184c);
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
call_ptr(c, 0x0039feb8, 0x003a184c)?;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.g[A1][0] = c.g[S0][0];
c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0xcc));
c.g[A0][0] = c.g[S2][0];
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
c.add_s(0, 0, 20);
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x50));
c.add_s(13, 1, 13);
c.g[RA][0] = sx(0x003a1874);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
call_ptr(c, 0x00391e30, 0x003a1874)?;
c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
if event_due(c) { deliver_event_here(c, 0x003a1748); } continue 'l_3a1748;
} else {
break 'b_3a1880;
}
}
}
}
}
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
break 'b_3a1884;
}
'b_3a1898: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[A2][0] = c.g[S4][0];
if t {
break 'b_3a1898;
} else {
c.cycles += 3;
c.g[A3][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc8));
c.g[RA][0] = sx(0x003a1898);
c.g[A1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
call_ptr(c, 0x0039da20, 0x003a1898)?;
break 'b_3a1898;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
break 'b_3a189c;
}
}
c.cycles += 9;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.pc = t;
return Ok(());