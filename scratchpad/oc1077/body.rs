'b_3a29a8: {
'b_3a29a4: {
c.cycles += 58;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x200));
c.mtc1(0, 0);
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[S4][0] = c.g[SP][0];
c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x1f8));
c.g[S6][0] = c.g[A3][0];
c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1e8));
c.mov_s(30, 12);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
c.c_le_s(30, 0);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[RA][0]);
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1d8));
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1d0));
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1c8));
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1c0));
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1b8));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A1][0] as u32));
c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A0][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[A2][0] as u32));
c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x18));
c.g[A0][0] = c.lw((c.g[A1][0] as u32));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.lwc1(1, (c.g[SP][0] as u32));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x104));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x108));
c.lwc1(22, (c.g[A1][0] as u32).wrapping_add(0xc));
c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0x18));
c.lwc1(21, (c.g[A1][0] as u32).wrapping_add(0x10));
let t = c.fcc();
c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x14));
if t {
break 'b_3a29a4;
} else {
c.cycles += 5;
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x100));
c.c_le_s(3, 0);
let t = c.fcc();
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
if t {
break 'b_3a29a8;
} else {
'b_3a24d8: {
c.cycles += 7;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[S6][0] & 0x1;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
c.div_s(27, 0, 30);
c.div_s(28, 1, 3);
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[S6][0] & 0x2;
if t {
break 'b_3a24d8;
} else {
c.cycles += 2;
c.mov_s(21, 29);
c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x10));
break 'b_3a24d8;
}
}
'b_3a24e8: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_3a24e8;
} else {
c.cycles += 2;
c.mov_s(22, 31);
c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0xc));
break 'b_3a24e8;
}
}
c.cycles += 10;
c.cvt_w_s(0, 30);
c.mfc1(17, 0);
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x108));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = 0xffffffffffffffff;
c.cvt_w_s(0, 0);
c.mfc1(21, 0);
c.g[V0][0] = 0x3;
'l_3a2510: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[V1][0];
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
if t {
if event_due(c) { deliver_event_here(c, 0x003a_2510); } continue 'l_3a2510;
} else {
c.cycles += 11;
c.mtc1(21, 1);
c.cvt_s_w(1, 1);
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
c.g[AT][0] = 0x437f0000;
c.mtc1(1, 2);
c.g[V1][0] = c.g[S2][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 3);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10c));
c.lwc1(0, (c.g[A2][0] as u32));
'l_3a2558: loop {
c.cycles += 20;
c.mul_s(0, 0, 4);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
c.mul_s(0, 0, 2);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.lwc1(0, (c.g[A2][0] as u32));
if event_due(c) { deliver_event_here(c, 0x003a_2558); } continue 'l_3a2558;
} else {
'b_3a2798: {
c.cycles += 2;
if c.g[S5][0] == 0u64 {
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10c));
break 'b_3a2798;
} else {
'b_3a2794: {
c.cycles += 3;
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x108));
let t = (c.g[S5][0] as i64) <= 0;
c.g[V1][0] = 0u64;
if t {
break 'b_3a2794;
} else {
c.cycles += 1;
'l_3a25c0: loop {
'b_3a2788: {
'b_3a26ac: {
c.cycles += 3;
c.g[V0][0] = c.g[S6][0] & 0x4;
let t = c.g[V0][0] == 0u64;
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x104));
if t {
break 'b_3a26ac;
} else {
'b_3a2638: {
c.cycles += 6;
c.add_s(1, 25, 28);
c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.mtc1(17, 23);
c.cvt_s_w(23, 23);
let t = (c.g[S1][0] as i64) <= 0;
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x110));
if t {
break 'b_3a2638;
} else {
c.cycles += 5;
c.sub_s(26, 29, 21);
c.g[S0][0] = c.g[S1][0];
c.sub_s(24, 31, 22);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
'l_3a25f8: loop {
c.cycles += 16;
c.g[A1][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.add_s(20, 20, 27);
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xec));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.g[RA][0] = sx(0x003a_2630);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xfc));
callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_2630)?;
if c.g[S0][0] != 0u64 {
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
if event_due(c) { deliver_event_here(c, 0x003a_25f8); } continue 'l_3a25f8;
} else {
break 'b_3a2638;
}
}
}
}
c.cycles += 4;
c.c_eq_s(23, 30);
let t = c.fcc();
c.g[V1][0] = c.g[S3][0];
if t {
break 'b_3a2788;
} else {
c.cycles += 25;
c.sub_s(3, 30, 23);
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
c.sub_s(0, 31, 22);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x104));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A1][0] = c.g[S2][0];
c.add_s(1, 2, 1);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.mul_s(0, 0, 3);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.sub_s(2, 29, 21);
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xec));
c.sub_s(1, 1, 20);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.add_s(0, 22, 0);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sub_s(0, 0, 22);
c.g[RA][0] = sx(0x003a_26a4);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_26a4)?;
let t = true;
c.g[V1][0] = c.g[S3][0];
if t {
break 'b_3a2788;
} else {
break 'b_3a26ac;
}
}
}
}
'b_3a2718: {
c.cycles += 6;
c.add_s(3, 25, 28);
c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.mtc1(17, 23);
c.cvt_s_w(23, 23);
let t = (c.g[S1][0] as i64) <= 0;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x110));
if t {
break 'b_3a2718;
} else {
c.cycles += 5;
c.sub_s(26, 29, 21);
c.g[S0][0] = c.g[S1][0];
c.sub_s(24, 31, 22);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
'l_3a26d8: loop {
c.cycles += 16;
c.g[A1][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.add_s(20, 20, 27);
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xec));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.g[RA][0] = sx(0x003a_2710);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xfc));
callee::DRAW_QUAD_UV.call(c, 0x003a_2710)?;
if c.g[S0][0] != 0u64 {
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
if event_due(c) { deliver_event_here(c, 0x003a_26d8); } continue 'l_3a26d8;
} else {
break 'b_3a2718;
}
}
}
}
c.cycles += 4;
c.c_eq_s(23, 30);
let t = c.fcc();
c.g[V1][0] = c.g[S3][0];
if t {
break 'b_3a2788;
} else {
c.cycles += 24;
c.sub_s(3, 30, 23);
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
c.sub_s(0, 29, 21);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x104));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A1][0] = c.g[S2][0];
c.add_s(1, 2, 1);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.mul_s(0, 0, 3);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.sub_s(2, 31, 22);
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xec));
c.sub_s(1, 1, 20);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.add_s(0, 21, 0);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xfc));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sub_s(0, 0, 21);
c.g[RA][0] = sx(0x003a_2784);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
callee::DRAW_QUAD_UV.call(c, 0x003a_2784)?;
c.g[V1][0] = c.g[S3][0];
break 'b_3a2788;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S5][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x110));
if t {
if event_due(c) { deliver_event_here(c, 0x003a_25c0); } continue 'l_3a25c0;
} else {
break 'b_3a2794;
}
}
}
}
c.cycles += 1;
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10c));
break 'b_3a2798;
}
}
c.cycles += 5;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
c.c_eq_s(3, 0);
if c.fcc() {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
break 'b_3a29a8;
} else {
'b_3a28b4: {
c.cycles += 3;
c.g[V0][0] = c.g[S6][0] & 0x4;
let t = c.g[V0][0] == 0u64;
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x104));
if t {
break 'b_3a28b4;
} else {
'b_3a2840: {
c.cycles += 14;
c.sub_s(3, 0, 3);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 1);
c.sub_s(2, 29, 21);
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0xc));
c.mtc1(17, 23);
c.cvt_s_w(23, 23);
c.sub_s(1, 1, 3);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x108));
c.mul_s(2, 2, 1);
c.add_s(0, 3, 0);
c.add_s(21, 21, 2);
let t = (c.g[S1][0] as i64) <= 0;
c.sub_s(24, 0, 25);
if t {
break 'b_3a2840;
} else {
c.cycles += 4;
c.sub_s(28, 29, 21);
c.g[S0][0] = c.g[S1][0];
c.sub_s(26, 31, 22);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
'l_3a2800: loop {
c.cycles += 16;
c.g[A1][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.add_s(20, 20, 27);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xec));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.g[RA][0] = sx(0x003a_2838);
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xfc));
callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_2838)?;
if c.g[S0][0] != 0u64 {
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
if event_due(c) { deliver_event_here(c, 0x003a_2800); } continue 'l_3a2800;
} else {
break 'b_3a2840;
}
}
}
}
c.cycles += 4;
c.c_eq_s(23, 30);
let t = c.fcc();
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
if t {
break 'b_3a29a8;
} else {
c.cycles += 25;
c.sub_s(2, 30, 23);
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
c.sub_s(0, 31, 22);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x104));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A1][0] = c.g[S2][0];
c.add_s(1, 3, 1);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.mul_s(0, 0, 2);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.sub_s(3, 29, 21);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xec));
c.sub_s(1, 1, 20);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.add_s(0, 22, 0);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sub_s(0, 0, 22);
c.g[RA][0] = sx(0x003a_28ac);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_28ac)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
if t {
break 'b_3a29a8;
} else {
break 'b_3a28b4;
}
}
}
}
'b_3a2938: {
c.cycles += 13;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mtc1(17, 23);
c.cvt_s_w(23, 23);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x10c));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x108));
c.sub_s(0, 1, 2);
c.sub_s(2, 31, 22);
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
c.add_s(1, 3, 1);
c.mul_s(2, 2, 0);
c.sub_s(24, 1, 25);
let t = (c.g[S1][0] as i64) <= 0;
c.add_s(31, 22, 2);
if t {
break 'b_3a2938;
} else {
c.cycles += 4;
c.sub_s(28, 29, 21);
c.g[S0][0] = c.g[S1][0];
c.sub_s(26, 31, 22);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
'l_3a28f8: loop {
c.cycles += 16;
c.g[A1][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.add_s(20, 20, 27);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xec));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.g[RA][0] = sx(0x003a_2930);
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xfc));
callee::DRAW_QUAD_UV.call(c, 0x003a_2930)?;
if c.g[S0][0] != 0u64 {
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
if event_due(c) { deliver_event_here(c, 0x003a_28f8); } continue 'l_3a28f8;
} else {
break 'b_3a2938;
}
}
}
}
c.cycles += 4;
c.c_eq_s(23, 30);
let t = c.fcc();
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
if t {
break 'b_3a29a8;
} else {
c.cycles += 23;
c.sub_s(2, 30, 23);
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
c.sub_s(0, 29, 21);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x104));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A1][0] = c.g[S2][0];
c.add_s(1, 3, 1);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.mul_s(0, 0, 2);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.sub_s(3, 31, 22);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xec));
c.sub_s(1, 1, 20);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.add_s(0, 21, 0);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xfc));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sub_s(0, 0, 21);
c.g[RA][0] = sx(0x003a_29a4);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
callee::DRAW_QUAD_UV.call(c, 0x003a_29a4)?;
break 'b_3a29a4;
}
}
}
}
}
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
break 'b_3a29a8;
}
c.cycles += 21;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x120));
c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x1f8));
c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1e8));
c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1e0));
c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1d8));
c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1d0));
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1c8));
c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1c0));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1b8));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
c.pc = t; return Ok(());