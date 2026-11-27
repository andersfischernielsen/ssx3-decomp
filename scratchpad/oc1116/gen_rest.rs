pub fn f_00402d88__generated(c: &mut Cpu) {
'b_402d9c: {
c.cycles += 5;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
c.g[T0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if c.g[A3][0] == 0u64 {
break 'b_402d9c;
} else {
break 'b_402d9c;
}
}
'b_402de8: {
'b_402dd8: {
c.cycles += 13;
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.divu(0, 2, 7);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.lo[0];
c.mult(0, 2, 2, 7);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
if c.g[V1][0] != 0u64 {
c.g[A1][0] = 0x490000;
break 'b_402dd8;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
if t {
break 'b_402de8;
} else {
break 'b_402dd8;
}
}
}
c.cycles += 4;
c.g[A0][0] = c.g[T0][0];
c.g[RA][0] = sx(0x00402de4);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7c48));
c.pc = 0x004034a8; libmpeg_Error(c);
if c.pc != 0x00402de4 { return; }
c.g[V0][0] = 0u64;
break 'b_402de8;
}
c.cycles += 3;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.pc = t; return;
}

/// Slot for the hand-written replacement of `f_00402d88` (crates/ssx3-game), set
/// once at boot before any guest code runs. Default: the generated body.
pub fn f_00402f80__generated(c: &mut Cpu) {
'b_40300c: {
'b_402ff4: {
'b_402fb8: {
c.cycles += 11;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[V0][0] = 0xffffffffffffffff;
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
c.g[S3][0] = 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
c.g[S1][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
c.sd((c.g[SP][0] as u32), c.g[S0][0]);
let t = c.g[A2][0] == c.g[V0][0];
c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
if t {
break 'b_402fb8;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_402ff4;
} else {
break 'b_402fb8;
}
}
}
'b_402fd0: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_402fd0;
} else {
c.cycles += 3;
c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
break 'b_402fd0;
}
}
'b_402fec: {
c.cycles += 4;
c.g[RA][0] = sx(0x00402fd8);
c.g[A1][0] = 0u64;
c.pc = 0x00407f20; libmpeg_SetupRefFrames(c);
if c.pc != 0x00402fd8 { return; }
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0u64;
if t {
break 'b_402fec;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00402fe8);
c.g[A0][0] = c.g[S0][0];
c.pc = 0x00407dc0; libmpeg_DecodePictureSlices(c);
if c.pc != 0x00402fe8 { return; }
c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
break 'b_402fec;
}
}
c.cycles += 2;
let t = true;
c.g[S2][0] = c.g[V1][0];
if t {
break 'b_40300c;
} else {
break 'b_402ff4;
}
}
c.cycles += 6;
c.g[RA][0] = sx(0x00402ffc);
c.g[A1][0] = 0u64;
c.pc = 0x00407f20; libmpeg_SetupRefFrames(c);
if c.pc != 0x00402ffc { return; }
c.g[S3][0] = 0x1;
c.g[S2][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0040300c);
c.g[A0][0] = c.g[S1][0];
c.pc = 0x00402c80; libmpeg_CallbackNodata(c);
if c.pc != 0x0040300c { return; }
break 'b_40300c;
}
'b_403044: {
c.cycles += 8;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0040301c);
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.pc = 0x00407e90; libmpeg_OutputPicture(c);
if c.pc != 0x0040301c { return; }
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
c.g[V0][0] = 0x3;
if c.g[V1][0] == c.g[V0][0] {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xac));
break 'b_403044;
} else {
c.cycles += 2;
if c.g[S3][0] != 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xac));
break 'b_403044;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x120));
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x120), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xac));
break 'b_403044;
}
}
}
'b_403078: {
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x120));
let t = c.g[V1][0] != 0u64;
c.g[V0][0] = c.g[S2][0];
if t {
break 'b_403078;
} else {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sw((c.g[S0][0] as u32).wrapping_add(0x118), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.g[V0][0] = c.g[S2][0];
break 'b_403078;
}
}
c.cycles += 7;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return;
}

/// Slot for the hand-written replacement of `f_00402f80` (crates/ssx3-game), set
/// once at boot before any guest code runs. Default: the generated body.
pub fn f_00403098__generated(c: &mut Cpu) {
'b_4030d4: {
'b_4030c8: {
c.cycles += 8;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
c.g[A3][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V1][0] = 0x3;
c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x174));
let t = c.g[V0][0] == c.g[V1][0];

if t {
break 'b_4030c8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x004030c0);
c.g[A0][0] = c.g[A3][0];
c.pc = 0x004030e0; libmpeg_DecodeFieldPic(c);
if c.pc != 0x004030c0 { return; }
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
if t {
break 'b_4030d4;
} else {
break 'b_4030c8;
}
}
}
c.cycles += 3;
c.g[RA][0] = sx(0x004030d0);
c.g[A0][0] = c.g[A3][0];
c.pc = 0x00402f80; libmpeg_DecodeFramePic(c);
if c.pc != 0x004030d0 { return; }
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
break 'b_4030d4;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.pc = t; return;
}

/// Slot for the hand-written replacement of `f_00403098` (crates/ssx3-game), set
/// once at boot before any guest code runs. Default: the generated body.
pub fn f_004030e0__generated(c: &mut Cpu) {
'b_403128: {
'b_403120: {
c.cycles += 13;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.g[V0][0] = 0xffffffffffffffff;
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
c.g[S3][0] = 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
c.g[S2][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
c.sd((c.g[SP][0] as u32), c.g[S0][0]);
c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
let t = c.g[A2][0] == c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x120), 0u64);
if t {
break 'b_403120;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
break 'b_403128;
} else {
break 'b_403120;
}
}
}
c.cycles += 2;
c.g[S3][0] = 0x1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
break 'b_403128;
}
'b_40313c: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_40313c;
} else {
c.cycles += 3;
c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
break 'b_40313c;
}
}
'b_403164: {
'b_403160: {
c.cycles += 4;
c.g[RA][0] = sx(0x00403144);
c.g[A1][0] = 0u64;
c.pc = 0x00407f20; libmpeg_SetupRefFrames(c);
if c.pc != 0x00403144 { return; }
let t = c.g[V0][0] == 0u64;
c.g[S1][0] = 0x1;
if t {
break 'b_403160;
} else {
c.cycles += 2;
let t = c.g[S3][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_403164;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x0040315c);
c.g[A0][0] = c.g[S0][0];
c.pc = 0x00407dc0; libmpeg_DecodePictureSlices(c);
if c.pc != 0x0040315c { return; }
c.g[S1][0] = 0x1;
break 'b_403160;
}
}
}
c.cycles += 1;
c.g[A0][0] = c.g[S0][0];
break 'b_403164;
}
'b_40321c: {
'b_403188: {
c.cycles += 4;
c.g[RA][0] = sx(0x0040316c);
c.sw((c.g[S0][0] as u32).wrapping_add(0x120), c.g[S1][0]);
c.pc = 0x00407520; libmpeg_GetHeader(c);
if c.pc != 0x0040316c { return; }
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd4));
break 'b_403188;
} else {
c.cycles += 5;
c.g[RA][0] = sx(0x0040317c);
c.g[A0][0] = c.g[S2][0];
c.pc = 0x00403240; libmpeg_FlushPendingPicture(c);
if c.pc != 0x0040317c { return; }
c.sw((c.g[S0][0] as u32), c.g[S1][0]);
let t = true;
c.g[V0][0] = 0u64;
if t {
break 'b_40321c;
} else {
break 'b_403188;
}
}
}
c.cycles += 6;
c.g[V1][0] = 0x2;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
c.g[V0][0] = c.g[V0][0] ^ 0x1;
if c.g[V0][0] != 0 { c.g[V1][0] = c.g[S1][0]; }
let t = c.g[A0][0] != c.g[V1][0];
c.g[V0][0] = 0xffffffffffffffff;
if t {
break 'b_40321c;
} else {
'b_4031d4: {
'b_4031d0: {
c.cycles += 7;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x004031ac);
c.g[A1][0] = 0x1;
c.pc = 0x00407f20; libmpeg_SetupRefFrames(c);
if c.pc != 0x004031ac { return; }
c.g[V1][0] = 0u64;
if c.g[V0][0] != 0 { c.g[V1][0] = c.g[S1][0]; }
let t = c.g[V1][0] == 0u64;
c.g[S4][0] = 0u64;
if t {
break 'b_4031d0;
} else {
c.cycles += 2;
if c.g[S3][0] == 0u64 {
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
break 'b_4031d4;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x004031cc);
c.g[A0][0] = c.g[S0][0];
c.pc = 0x00407dc0; libmpeg_DecodePictureSlices(c);
if c.pc != 0x004031cc { return; }
if c.g[V0][0] != 0 { c.g[S4][0] = c.g[S1][0]; }
break 'b_4031d0;
}
}
}
c.cycles += 1;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
break 'b_4031d4;
}
'b_403218: {
c.cycles += 15;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x004031e0);
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.pc = 0x00407e90; libmpeg_OutputPicture(c);
if c.pc != 0x004031e0 { return; }
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xac));
c.sw((c.g[S0][0] as u32).wrapping_add(0x120), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.sw((c.g[S0][0] as u32).wrapping_add(0x118), c.g[V1][0]);
let t = c.g[S3][0] != 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
if t {
break 'b_403218;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00403218);
c.g[A0][0] = c.g[S2][0];
c.pc = 0x00402c80; libmpeg_CallbackNodata(c);
if c.pc != 0x00403218 { return; }
break 'b_403218;
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S4][0];
break 'b_40321c;
}
}
c.cycles += 8;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return;
}

/// Slot for the hand-written replacement of `f_004030e0` (crates/ssx3-game), set
/// once at boot before any guest code runs. Default: the generated body.
