/// `libmpeg_SequenceHeader` (0x00403620): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn lib_mpeg_sequence_header(c: &mut Cpu) {
    if c.pc != 0x00403620 {
        return ee::s15::f_00403620__generated(c);
    }
    guest(c, |c| {
'b_40366c: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.g[A1][0] = 0x20;
c.sd((c.g[SP][0] as u32), c.g[S0][0]);
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[RA][0] = sx(0x0040363c);
c.sw((c.g[S0][0] as u32).wrapping_add(0xd4), 0u64);
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x0040363c)?;
c.g[V1][0] = c.g[V0][0];
c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] & 0xfff;
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
c.sw((c.g[S0][0] as u32).wrapping_add(0x124), c.g[V1][0]);
c.g[A0][0] = ((c.g[V0][0] as i64) < 2801i64) as u64;
let t = c.g[A0][0] != 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x128), c.g[V0][0]);
if t {
break 'b_40366c;
} else {
c.cycles += 4;
c.g[A1][0] = 0x490000;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0040366c);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7cf8));
Callee { addr: 0x004034a8, f: ee::s15::libmpeg_Error }.call(c, 0x0040366c)?;
break 'b_40366c;
}
}
'b_4036e0: {
'b_4036c8: {
c.cycles += 14;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00403678);
c.g[A1][0] = 0x1e;
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x00403678)?;
c.g[V1][0] = c.g[V0][0];
c.g[A0][0] = c.g[S0][0];
c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[A1][0] = 0x1;
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 12);
c.g[V0][0] = c.g[V0][0] & 0x3ff;
c.sw((c.g[S0][0] as u32).wrapping_add(0x134), c.g[V1][0]);
c.g[RA][0] = sx(0x0040369c);
c.sw((c.g[S0][0] as u32).wrapping_add(0x138), c.g[V0][0]);
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x0040369c)?;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x840), c.g[V0][0]);
if t {
break 'b_4036c8;
} else {
c.cycles += 9;
c.g[RA][0] = sx(0x004036ac);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00406e20, f: ee::s15::libmpeg_IpuWaitBusy }.call(c, 0x004036ac)?;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x004036b8);
c.g[A1][0] = 0x50000000;
Callee { addr: 0x00406df0, f: ee::s15::libmpeg_IpuCommand }.call(c, 0x004036b8)?;
c.g[RA][0] = sx(0x004036c0);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00406e20, f: ee::s15::libmpeg_IpuWaitBusy }.call(c, 0x004036c0)?;
let t = true;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_4036e0;
} else {
break 'b_4036c8;
}
}
}
c.cycles += 6;
c.g[A2][0] = 0x450000;
c.g[A0][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3200));
c.g[RA][0] = sx(0x004036dc);
c.g[A1][0] = 0x50000000;
Callee { addr: 0x00403ad0, f: ee::s15::libmpeg_LoadIqMatrix }.call(c, 0x004036dc)?;
c.g[A0][0] = c.g[S0][0];
break 'b_4036e0;
}
'b_403728: {
'b_403714: {
c.cycles += 4;
c.g[RA][0] = sx(0x004036e8);
c.g[A1][0] = 0x1;
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x004036e8)?;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x844), c.g[V0][0]);
if t {
break 'b_403714;
} else {
c.cycles += 9;
c.g[RA][0] = sx(0x004036f8);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00406e20, f: ee::s15::libmpeg_IpuWaitBusy }.call(c, 0x004036f8)?;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00403704);
c.g[A1][0] = 0x58000000;
Callee { addr: 0x00406df0, f: ee::s15::libmpeg_IpuCommand }.call(c, 0x00403704)?;
c.g[RA][0] = sx(0x0040370c);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00406e20, f: ee::s15::libmpeg_IpuWaitBusy }.call(c, 0x0040370c)?;
let t = true;

if t {
break 'b_403728;
} else {
break 'b_403714;
}
}
}
c.cycles += 5;
c.g[A2][0] = 0x450000;
c.g[A0][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3240));
c.g[RA][0] = sx(0x00403728);
c.g[A1][0] = 0x58000000;
Callee { addr: 0x00403ad0, f: ee::s15::libmpeg_LoadIqMatrix }.call(c, 0x00403728)?;
break 'b_403728;
}
c.cycles += 7;
c.g[RA][0] = sx(0x00403730);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x004076f8, f: ee::s15::libmpeg_ExtensionAndUserData }.call(c, 0x00403730)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.ld((c.g[SP][0] as u32));
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
'b_4037b0: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
c.g[A1][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[S7][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[S6][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S4][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S2][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S1][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S0][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[FP][0]);
c.g[FP][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x848));
if c.g[A2][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x124));
break 'b_4037b0;
} else {
c.cycles += 10;
c.g[V0][0] = 0x1;
c.g[V1][0] = 0x3;
c.g[A0][0] = 0x5;
c.sw((c.g[FP][0] as u32).wrapping_add(0x174), c.g[V1][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x144), c.g[A0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x188), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x124));
break 'b_4037b0;
}
}
'b_4037e8: {
'b_4037e0: {
'b_4037dc: {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
let t = c.g[A2][0] == 0u64;
c.sw((c.g[FP][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
if t {
break 'b_4037dc;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x13c));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x128));
if t {
break 'b_4037e0;
} else {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
if t {
break 'b_4037e8;
} else {
break 'b_4037dc;
}
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x128));
break 'b_4037e0;
}
c.cycles += 2;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
break 'b_4037e8;
}
'b_403810: {
c.cycles += 7;
c.sw((c.g[FP][0] as u32).wrapping_add(0x130), c.g[V0][0]);
c.g[S6][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x12c));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[S7][0] = sx((c.g[V0][0] as u32) << 4);
let t = c.g[S7][0] != c.g[V1][0];
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x528));
if t {
break 'b_403810;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
let t = c.g[S6][0] == c.g[V0][0];
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x528));
if t {
c.cycles += 12;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.pc = t; return Ok(());
} else {
break 'b_403810;
}
}
}
c.cycles += 108;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S6][0]);
c.g[S0][0] = 0x180;
c.sw((c.g[A1][0] as u32), c.g[S7][0]);
c.mult(0, 16, 22, 16);
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x108));
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x320));
c.g[A0][0] = c.g[S1][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1e8));
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x388));
c.mult(0, 16, 23, 16);
c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
c.g[S4][0] = sx((c.g[FP][0] as u32).wrapping_add(0x250));
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3f0));
c.g[S5][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2b8));
c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
c.g[S2][0] = sx(((c.g[S6][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x458));
c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4c0));
c.g[RA][0] = sx(0x00403870);
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
Callee { addr: 0x00402d78, f: ee::s15::libmpeg_WorkAreaRestore }.call(c, 0x00403870)?;
c.g[A0][0] = c.g[FP][0];
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00403884);
c.g[A3][0] = 0x40;
Callee { addr: 0x00402d88, f: ee::s15::libmpeg_WorkAreaAlloc }.call(c, 0x00403884)?;
c.sw((c.g[FP][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
c.g[A0][0] = c.g[FP][0];
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0040389c);
c.g[A3][0] = 0x40;
Callee { addr: 0x00402d88, f: ee::s15::libmpeg_WorkAreaAlloc }.call(c, 0x0040389c)?;
c.sw((c.g[FP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.g[S0][0];
c.g[A0][0] = c.g[FP][0];
c.g[RA][0] = sx(0x004038b4);
c.g[A3][0] = 0x40;
Callee { addr: 0x00402d88, f: ee::s15::libmpeg_WorkAreaAlloc }.call(c, 0x004038b4)?;
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
c.g[A0][0] = c.g[S3][0];
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
c.g[A1][0] = c.g[S4][0];
c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[A2][0] = c.g[S5][0];
c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
c.sw((c.g[FP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xfc));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x100));
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x104));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S7][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[S6][0]);
c.g[RA][0] = sx(0x00403904);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
Callee { addr: 0x004039f0, f: ee::s15::libmpeg_SetupFrameBuffers }.call(c, 0x00403904)?;
c.g[A0][0] = c.g[S3][0];
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403914);
c.g[A2][0] = c.g[S6][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403914)?;
c.g[A0][0] = c.g[S4][0];
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403924);
c.g[A2][0] = c.g[S6][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403924)?;
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403934);
c.g[A2][0] = c.g[S6][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403934)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403944);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403944)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403954);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403954)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403964);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403964)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403974);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403974)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x00403984);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x00403600, f: ee::s15::libmpeg_SetPictureSize }.call(c, 0x00403984)?;
c.g[A1][0] = c.g[S7][0];
c.g[A2][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.pc = 0x00403600; libmpeg_SetPictureSize(c); return Ok(());
    });
}
