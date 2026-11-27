/// `int SPCH_AssignVariations(Phrase* phrase, SelectionSet* sel, int thr)`
/// (0x3d93c0): picks one sample variation per entry of a speech phrase. Every
/// entry's current choice (`+6`) starts at its first one (`+4`); a combination
/// is accepted when its sample-usage percentage (the sizes `SPCH_DecodeSampleEntry`
/// reports, times 100 over the global total at `gp+0x1748`) stays within `thr`
/// (0 = unchecked) and, for a phrase with sentence parts (`phrase[5]`),
/// `SPCH_BuildSentencePart` accepts it. Otherwise the choices advance like an
/// odometer (last entry first, each wrapping to its first choice); the search
/// ends with the accepted flag, or 0 once every combination was tried.
pub fn spch_assign_variations(c: &mut Cpu) {
    guest(c, |c| {
'b_3d9430: {
c.cycles += 18;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[FP][0] = 0u64;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S7][0] = c.g[A2][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S6][0] = c.g[A0][0];
c.sq(31, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
let t = c.g[V0][0] == 0u64;
c.g[S5][0] = c.g[A1][0];
if t {
break 'b_3d9430;
} else {
c.cycles += 2;
c.g[A0][0] = c.g[V0][0];
c.g[V1][0] = c.g[S5][0];
'l_3d9410: loop {
c.cycles += 7;
c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
c.sb((c.g[V1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
let t = c.g[A0][0] != 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003d9410); } continue 'l_3d9410;
} else {
c.cycles += 1;
break 'b_3d9430;
}
}
}
}
'l_3d9430: loop {
'b_3d94c4: {
c.cycles += 2;
let t = c.g[S7][0] == 0u64;
c.g[S4][0] = 0x1;
if t {
break 'b_3d94c4;
} else {
'b_3d9498: {
c.cycles += 4;
c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
let t = c.g[V0][0] == 0u64;
c.g[S3][0] = 0u64;
if t {
break 'b_3d9498;
} else {
c.cycles += 4;
c.g[S1][0] = c.g[V0][0];
c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x62));
c.g[S0][0] = c.g[S5][0];
'l_3d9458: loop {
'b_3d948c: {
c.cycles += 12;
c.g[V0][0] = c.lh((c.g[S0][0] as u32));
c.g[A2][0] = c.g[SP][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1734));
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
c.g[RA][0] = sx(0x003d9480);
c.g[A1][0] = c.lbu((c.g[V1][0] as u32));
Callee { addr: 0x003db838, f: ee::s14::SPCH_DecodeSampleEntry }.call(c, 0x003d9480)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
if t {
break 'b_3d948c;
} else {
c.cycles += 1;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
break 'b_3d948c;
}
}
c.cycles += 3;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
let t = c.g[S1][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003d9458); } continue 'l_3d9458;
} else {
break 'b_3d9498;
}
}
}
}
'b_3d94bc: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1748));
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_3d94bc;
} else {
'b_3d94b0: {
c.cycles += 3;
c.g[V0][0] = 0x64;
if c.g[V1][0] == 0u64 {
break 'b_3d94b0;
} else {
break 'b_3d94b0;
}
}
c.cycles += 3;
c.mult(0, 2, 19, 2);
c.div(0, 2, 3);
c.g[V0][0] = c.lo[0];
break 'b_3d94bc;
}
}
c.cycles += 2;
c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0 { c.g[S4][0] = 0u64; }
break 'b_3d94c4;
}
}
'b_3d9574: {
'b_3d94f0: {
c.cycles += 2;
if c.g[S4][0] == 0u64 {
c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
break 'b_3d94f0;
} else {
'b_3d94e4: {
c.cycles += 3;
c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x5));
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S5][0];
if t {
break 'b_3d94e4;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003d94e0);
c.g[A1][0] = c.g[S6][0];
Callee { addr: 0x003d8e58, f: ee::s14::SPCH_BuildSentencePart }.call(c, 0x003d94e0)?;
if c.g[V0][0] == 0 { c.g[S4][0] = 0u64; }
break 'b_3d94e4;
}
}
c.cycles += 2;
let t = c.g[S4][0] != 0u64;

if t {
break 'b_3d9574;
} else {
c.cycles += 1;
c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
break 'b_3d94f0;
}
}
}
c.cycles += 12;
c.g[T1][0] = 0u64;
c.g[T0][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = sx((c.g[A3][0] as u32) << 3);
c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x5));
c.g[A1][0] = c.g[A0][0];
c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x6));
'l_3d9520: loop {
'b_3d9568: {
'b_3d953c: {
c.cycles += 5;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = c.g[V0][0] & 0xff;
c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V1][0] == 0u64;
c.sb((c.g[A0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
if t {
break 'b_3d953c;
} else {
c.cycles += 2;
let t = true;
c.g[T0][0] = 0x1;
if t {
break 'b_3d9568;
} else {
break 'b_3d953c;
}
}
}
'b_3d9558: {
c.cycles += 5;
c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x8));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
let t = (c.g[A3][0] as i64) >= 0;
c.sb((c.g[A0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
if t {
break 'b_3d9558;
} else {
c.cycles += 2;
c.g[T0][0] = 0x1;
c.g[T1][0] = 0x1;
break 'b_3d9558;
}
}
c.cycles += 4;
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
c.g[A0][0] = c.g[A1][0];
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
break 'b_3d9568;
}
c.cycles += 2;
if c.g[T0][0] == 0u64 {
c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x6));
if event_due(c) { deliver_event_here(c, 0x003d9520); } continue 'l_3d9520;
} else {
c.cycles += 1;
c.g[FP][0] = c.g[T1][0];
break 'b_3d9574;
}
}
}
'b_3d9584: {
c.cycles += 2;
let t = c.g[FP][0] != 0u64;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_3d9584;
} else {
c.cycles += 2;
let t = c.g[S4][0] == 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x003d9430); } continue 'l_3d9430;
} else {
break 'b_3d9584;
}
}
}
c.cycles += 12;
c.lq(31, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return Ok(());
}
    });
}

/// `libmpeg_SequenceHeader(dec)` (0x403620): parses an MPEG sequence header
/// from the bitstream: width/height (`dec+0x124/0x128`, `Error` when the width
/// field is 2801 or more), frame-rate and bit-rate words (`+0x134/0x138`), then
/// the intra and non-intra quantiser matrices (loaded from the stream through
/// the IPU, or the default tables), and the extension/user data. Then it
/// falls into the frame-buffer set-up of the decoder context at
/// `dec[0x858]->[0x40]`: macroblock dimensions (`+0x12c/0x130`) and, when the
/// picture size changed, the work-area allocations, `SetupFrameBuffers` and
/// the `SetPictureSize` calls (the last one a tail call).
pub fn lib_mpeg_sequence_header(c: &mut Cpu) {
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
c.pc = 0x00403600;
    ee::s15::libmpeg_SetPictureSize(c);
    return Ok(());
    });
}

/// `int libmpeg_SetupMacroblock(dec, mbAddr, b, flags, mtype)` (0x403e00): splits
/// the macroblock address by the picture width in macroblocks (`dec+0x12c`)
/// into column and row, waits for the IPU-side SPR DMA (`0x1000d400` bit 8),
/// and for a non-intra block (`flags & 1` clear) forms the predictions
/// (`Errorf` and 0 for a bad motion type 0 or above 3, else `FormPredictions`,
/// the DMA wait again, `FetchRefBlocks`). Then it records the per-buffer
/// entry of `dec[0x6b8..0x6c8 + cur * 0x140]`: done flag, two flags and the
/// macroblock's buffer address in the current frame (picture structure
/// `dec+0x174`: 3 frame, 2 the `+0x1e0` field buffer, else `+0x1d0`).
pub fn lib_mpeg_setup_macroblock(c: &mut Cpu) {
    guest(c, |c| {
'b_403e50: {
c.cycles += 20;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
c.g[FP][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
c.g[S5][0] = c.g[A3][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
c.g[V1][0] = c.g[S5][0] & 0x1;
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
c.sw((c.g[SP][0] as u32), c.g[V1][0]);
c.div(0, 5, 2);
if c.g[V0][0] == 0u64 {
break 'b_403e50;
} else {
break 'b_403e50;
}
}
'b_404020: {
'b_403f44: {
'b_403eb0: {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = c.hi[0];
c.g[S7][0] = c.lo[0];
c.g[S6][0] = c.g[V1][0];
c.g[A2][0] = sx((c.g[S7][0] as u32) << 4);
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = sx((c.g[S6][0] as u32) << 4);
if t {
break 'b_403eb0;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c4));
c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b8));
'l_403e80: loop {
c.cycles += 7;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0xd400;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
c.g[V1][0] = c.g[V1][0] & 0x1;
let t = c.g[V1][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00403e80); } continue 'l_403e80;
} else {
c.cycles += 5;
c.g[V0][0] = 0x140;
c.mult(0, 3, 4, 2);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
let t = true;
c.sw((c.g[V0][0] as u32), 0u64);
if t {
break 'b_403f44;
} else {
break 'b_403eb0;
}
}
}
}
}
'b_403ee4: {
c.cycles += 4;
c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_403ee4;
} else {
c.cycles += 9;
c.g[A1][0] = 0x490000;
c.g[A2][0] = c.g[T0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e10));
c.g[RA][0] = sx(0x00403ed4);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00403470, f: ee::s15::libmpeg_Errorf }.call(c, 0x00403ed4)?;
c.g[V1][0] = 0x1;
c.g[V0][0] = 0u64;
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
if t {
break 'b_404020;
} else {
break 'b_403ee4;
}
}
}
c.cycles += 9;
c.g[RA][0] = sx(0x00403eec);
c.g[A3][0] = c.g[S5][0];
Callee { addr: 0x00404050, f: ee::s15::libmpeg_FormPredictions }.call(c, 0x00403eec)?;
c.g[V1][0] = 0x10000000;
c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c4));
c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b8));
c.g[V1][0] = c.g[V1][0] | 0xd400;
'l_403f08: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00403f08); } continue 'l_403f08;
} else {
c.cycles += 8;
c.g[RA][0] = sx(0x00403f2c);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00408988, f: ee::s15::libmpeg_FetchRefBlocks }.call(c, 0x00403f2c)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[V1][0] = 0x140;
c.g[A0][0] = 0x1;
c.mult(0, 5, 2, 3);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.sw((c.g[V0][0] as u32), c.g[A0][0]);
break 'b_403f44;
}
}
}
'b_403f84: {
'b_403f74: {
c.cycles += 3;
c.g[V0][0] = 0x1;
if c.g[FP][0] != c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
break 'b_403f74;
} else {
'b_403f70: {
c.cycles += 3;
c.g[V0][0] = c.g[S5][0] & 0x2;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x140;
if t {
break 'b_403f70;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.mult(0, 4, 2, 3);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
let t = true;
c.sw((c.g[V0][0] as u32), c.g[FP][0]);
if t {
break 'b_403f84;
} else {
break 'b_403f70;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
break 'b_403f74;
}
}
c.cycles += 4;
c.g[V1][0] = 0x140;
c.mult(0, 4, 2, 3);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.sw((c.g[V0][0] as u32), 0u64);
break 'b_403f84;
}
'b_40401c: {
'b_403fe0: {
c.cycles += 10;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[A3][0] = 0x140;
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = 0x3;
c.mult(0, 4, 2, 7);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.sw((c.g[V0][0] as u32), c.g[A1][0]);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
let t = c.g[A0][0] != c.g[V1][0];
c.g[V0][0] = 0x2;
if t {
break 'b_403fe0;
} else {
c.cycles += 13;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[A2][0] = 0x180;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
c.mult(0, 3, 4, 7);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.mult(0, 5, 22, 2);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S7][0] as u32)));
c.mult(0, 2, 2, 6);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.sw((c.g[A0][0] as u32), c.g[V1][0]);
if t {
break 'b_40401c;
} else {
break 'b_403fe0;
}
}
}
'b_403fec: {
c.cycles += 2;
if c.g[A0][0] != c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d0));
break 'b_403fec;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e0));
break 'b_403fec;
}
}
c.cycles += 12;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A2][0] = 0x180;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[A1][0] = 0x140;
c.mult(0, 7, 22, 3);
c.g[V0][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S7][0] as u32)));
c.mult(0, 7, 4, 5);
c.mult(0, 3, 3, 6);
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
break 'b_40401c;
}
c.cycles += 1;
c.g[V0][0] = 0x1;
break 'b_404020;
}
c.cycles += 12;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return Ok(());
    });
}

/// `libmpeg_SetupPredBlock(dec, ref, a2, a3, t0, h, bx, by, dx, dy, fld, avg)`
/// (0x404758): appends the luma and the chroma prediction block descriptors
/// for one motion vector (`dx`, `dy` in half pixels, at the stack) to the
/// current buffer entry of `dec` (`dec[0x6bc + cur * 0x140]` holds the count
/// `k`): block address inside the reference picture, the fraction-dependent
/// row split (`+8`/`+0xc` rows before/after the macroblock edge), buffer
/// pointers, and the interpolation-routine selectors from the tables at
/// `0x453280`/`0x4532a0`. Leaf, 8 stack arguments.
pub fn lib_mpeg_setup_pred_block(c: &mut Cpu) {
'b_404818: {
'b_404810: {
c.cycles += 42;
c.g[T8][0] = c.g[A0][0];
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(0x6bc));
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[FP][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
c.g[V1][0] = 0x140;
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
c.g[FP][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
c.g[T6][0] = c.g[T1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
c.g[S1][0] = c.g[A2][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
c.g[S0][0] = c.g[T0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S7][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x81c));
c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[A0][0] = 0x1c;
c.g[T5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T2][0]);
c.mult(0, 2, 2, 3);
c.sw((c.g[SP][0] as u32), c.g[A3][0]);
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[T2][0] = sx(((c.g[S6][0] as u32) as i32 >> 1) as u32);
c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb8));
c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S7][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x590));
c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.mult(0, 4, 23, 4);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
let t = c.g[T5][0] == 0u64;
c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
if t {
break 'b_404810;
} else {
c.cycles += 4;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
if t {
break 'b_404818;
} else {
break 'b_404810;
}
}
}
c.cycles += 2;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
break 'b_404818;
}
'b_4048e0: {
'b_4048dc: {
'b_4048a8: {
c.cycles += 22;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
c.g[S4][0] = sx(((c.g[T0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
c.mult(0, 5, 20, 5);
c.g[S5][0] = sx(((c.g[A2][0] as u32) as i32 >> 4) as u32);
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
c.sw((c.g[T4][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S5][0] as u32)));
c.g[V0][0] = sx((c.g[S5][0] as u32) << 4);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.sw((c.g[T4][0] as u32), c.g[A0][0]);
c.g[T9][0] = c.g[S2][0] & 0x1;
let t = c.g[T9][0] == 0u64;
c.g[S3][0] = c.g[S6][0] & 0x1;
if t {
break 'b_4048a8;
} else {
c.cycles += 5;
c.g[V0][0] = sx((c.g[T6][0] as u32) << ((c.g[T5][0] as u32) & 31));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < 16i64) as u64;
if c.g[V0][0] != 0u64 {
c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T6][0]);
break 'b_4048dc;
} else {
c.cycles += 9;
c.g[V0][0] = 0x10;
c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = sx((c.g[T6][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
let t = true;
c.sw((c.g[T4][0] as u32).wrapping_add(0xc), c.g[V1][0]);
if t {
break 'b_4048e0;
} else {
break 'b_4048a8;
}
}
}
}
c.cycles += 5;
c.g[V0][0] = sx((c.g[T6][0] as u32) << ((c.g[T5][0] as u32) & 31));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
if c.g[V0][0] != 0u64 {
c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T6][0]);
break 'b_4048dc;
} else {
c.cycles += 8;
c.g[V0][0] = 0x10;
c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
let t = true;
c.sw((c.g[T4][0] as u32).wrapping_add(0xc), c.g[A0][0]);
if t {
break 'b_4048e0;
} else {
break 'b_4048dc;
}
}
}
c.cycles += 1;
c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
break 'b_4048e0;
}
'b_4049a8: {
'b_404994: {
c.cycles += 38;
c.g[A1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
c.g[V0][0] = 0x140;
c.g[A3][0] = 0x600;
c.g[A0][0] = sx((c.g[S3][0] as u32) << 1);
c.mult(0, 3, 5, 2);
c.mult(0, 7, 23, 7);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc8));
c.g[T1][0] = sx((c.g[A2][0] as u32) << 4);
c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x300));
c.g[A2][0] = 0x10;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T5][0] as u32) & 31));
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T8][0] as u32)));
c.g[V0][0] = sx((c.g[S2][0] as u32) >> 31);
c.g[V1][0] = sx((c.g[S6][0] as u32) >> 31);
c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x590));
c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S3][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
c.g[S6][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
c.g[A0][0] = c.g[A3][0] | c.g[A0][0];
c.g[A0][0] = c.g[A0][0] | c.g[T9][0];
c.g[A3][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.sw((c.g[T4][0] as u32).wrapping_add(0x14), c.g[T1][0]);
c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
c.sw((c.g[T4][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[T4][0] as u32).wrapping_add(0x18), c.g[T2][0]);
let t = c.g[T5][0] == 0u64;
c.g[T1][0] = sx(((c.g[T6][0] as u32) as i32 >> 1) as u32);
if t {
break 'b_404994;
} else {
c.cycles += 7;
c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
c.g[A0][0] = sx(((c.g[T3][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = true;
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
if t {
break 'b_4049a8;
} else {
break 'b_404994;
}
}
}
c.cycles += 5;
c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
c.g[V1][0] = sx(((c.g[T3][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
break 'b_4049a8;
}
'b_404a5c: {
'b_404a58: {
'b_404a24: {
c.cycles += 17;
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.g[T2][0] = sx(((c.g[A2][0] as u32) as i32 >> 3) as u32);
c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A0][0] = sx((c.g[T2][0] as u32) << 3);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[S3][0] = c.g[S3][0] & 0x1;
c.g[A3][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x200));
c.g[V1][0] = sx((c.g[A3][0] as u32) << 3);
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.g[T9][0] = c.g[S6][0] & 0x1;
c.sw((c.g[T7][0] as u32).wrapping_add(0x4), c.g[V1][0]);
let t = c.g[T9][0] == 0u64;
c.sw((c.g[T7][0] as u32), c.g[V0][0]);
if t {
break 'b_404a24;
} else {
c.cycles += 5;
c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[T5][0] as u32) & 31));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < 8i64) as u64;
if c.g[V0][0] != 0u64 {
c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T1][0]);
break 'b_404a58;
} else {
c.cycles += 9;
c.g[V0][0] = 0x8;
c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
let t = true;
c.sw((c.g[T7][0] as u32).wrapping_add(0xc), c.g[V1][0]);
if t {
break 'b_404a5c;
} else {
break 'b_404a24;
}
}
}
}
c.cycles += 5;
c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[T5][0] as u32) & 31));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < 9i64) as u64;
if c.g[V0][0] != 0u64 {
c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T1][0]);
break 'b_404a58;
} else {
c.cycles += 8;
c.g[V0][0] = 0x8;
c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
let t = true;
c.sw((c.g[T7][0] as u32).wrapping_add(0xc), c.g[A0][0]);
if t {
break 'b_404a5c;
} else {
break 'b_404a58;
}
}
}
c.cycles += 1;
c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
break 'b_404a5c;
}
c.cycles += 70;
c.g[V0][0] = 0x8;
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[S4][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[T5][0] as u32) & 31));
c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub((c.g[S5][0] as u32)));
c.sw((c.g[T7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A1][0] = 0x140;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[V0][0] = 0x450000;
c.g[T0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
c.g[T2][0] = 0x180;
c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3280));
c.mult(0, 8, 8, 5);
c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.mult(0, 5, 4, 10);
c.mult(0, 13, 12, 10);
c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
c.g[T4][0] = c.lw((c.g[V1][0] as u32));
c.g[A3][0] = sx((c.g[S7][0] as u32) << 2);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
c.sw((c.g[V1][0] as u32).wrapping_add(0x5b8), c.g[T4][0]);
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x400));
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
c.g[T0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x100));
c.mult(0, 12, 11, 10);
c.g[A1][0] = 0x450000;
c.g[T1][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x32a0));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.sw((c.g[T7][0] as u32).wrapping_add(0x14), c.g[A0][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.sw((c.g[T7][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[A0][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[T3][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A1][0] = c.lw((c.g[T0][0] as u32));
c.g[V1][0] = c.g[A0][0];
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[T5][0] as u32)));
c.g[A2][0] = c.lw((c.g[V0][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.sw((c.g[A0][0] as u32).wrapping_add(0x598), c.g[T1][0]);
c.g[A3][0] = c.g[V1][0];
c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[V1][0] as u32).wrapping_add(0x5c8), c.g[A2][0]);
c.sw((c.g[A3][0] as u32).wrapping_add(0x5a8), c.g[T3][0]);
c.sw((c.g[T0][0] as u32), c.g[A1][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return;
}

/// `int libmpeg_WaitBdec(dec)` (0x405b70): waits for the IPU decode to finish.
/// Loops while the IPU-from DMA has quadwords queued (`0x1000b020`) and no
/// error is flagged in `IPU_CTRL` (`0x10002010`, ECD = bit 14), feeding the
/// stalled to-IPU DMA through the callback `dec[0x858]` (`DispatchCallback`,
/// command 1). With `dec[0x10a0]` set it aborts instead (interrupts off, DMA
/// registers cleared, `IPU_CTRL = 0x40000000`, returns 0). Otherwise it
/// stores `IPU_TOP` and the bit-position remainder (`dec+0x838/0x83c`) and,
/// when ECD is set, reports the error (`Error`, callbacks 2 and 3), resets the
/// DMA and returns 0; else 1.
pub fn lib_mpeg_wait_bdec(c: &mut Cpu) {
    guest(c, |c| {
'b_405c38: {
'b_405c34: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S2][0]);
c.g[S2][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S6][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S5][0]);
c.g[S6][0] = 0x1;
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S4][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S1][0]);
c.g[RA][0] = sx(0x00405ba0);
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S0][0]);
Callee { addr: 0x00406e20, f: ee::s15::libmpeg_IpuWaitBusy }.call(c, 0x00405ba0)?;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0xb020;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0x10000000;
if t {
break 'b_405c34;
} else {
c.cycles += 5;
c.g[V0][0] = c.g[V0][0] | 0x2010;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.g[V1][0] & 0x4000;
if c.g[V1][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
break 'b_405c38;
} else {
c.cycles += 10;
c.g[S4][0] = 0x10000000;
c.g[S3][0] = 0x10000000;
c.g[S1][0] = 0x10000000;
c.g[S0][0] = 0x10000000;
c.g[S4][0] = c.g[S4][0] | 0xb420;
c.g[S3][0] = c.g[S3][0] | 0xb400;
c.g[S5][0] = 0x1;
c.g[S1][0] = c.g[S1][0] | 0xb020;
c.g[S0][0] = c.g[S0][0] | 0x2010;
'l_405bf0: loop {
'b_405c18: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S4][0] as u32));
let t = c.g[V0][0] != 0u64;

if t {
break 'b_405c18;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S3][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.g[SP][0];
if t {
break 'b_405c18;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
c.g[RA][0] = sx(0x00405c18);
c.sw((c.g[SP][0] as u32), c.g[S5][0]);
Callee { addr: 0x00402c30, f: ee::s15::libmpeg_DispatchCallback }.call(c, 0x00405c18)?;
break 'b_405c18;
}
}
}
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
break 'b_405c38;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x4000;
let t = c.g[V0][0] == 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00405bf0); } continue 'l_405bf0;
} else {
break 'b_405c34;
}
}
}
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
break 'b_405c38;
}
'b_405de8: {
'b_405cd8: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x10000000;
if t {
break 'b_405cd8;
} else {
'b_405ca8: {
c.cycles += 23;
c.g[RA][0] = sx(0x00405c48);
Callee { addr: 0x0042c078, f: ee::s15::DIntr }.call(c, 0x00405c48)?;
c.g[A3][0] = 0x10000000;
c.g[A0][0] = 0x10000;
c.g[A3][0] = c.g[A3][0] | 0xf520;
c.g[T0][0] = 0x10000000;
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[T0][0] = c.g[T0][0] | 0xf590;
c.g[A1][0] = 0x10000000;
c.g[A2][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.g[A1][0] = c.g[A1][0] | 0xb400;
c.sw((c.g[T0][0] as u32), c.g[V1][0]);
c.g[A2][0] = c.g[A2][0] | 0xb000;
c.sw((c.g[A1][0] as u32), 0u64);
c.g[A0][0] = 0xfffffffffffe0000;
c.sw((c.g[A2][0] as u32), 0u64);
c.g[A0][0] = c.g[A0][0] | 0xffff;
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
c.sw((c.g[T0][0] as u32), c.g[V1][0]);
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x10000000;
if t {
break 'b_405ca8;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00405ca4);
Callee { addr: 0x0042c0c0, f: ee::s15::EIntr }.call(c, 0x00405ca4)?;
c.g[V0][0] = 0x10000000;
break 'b_405ca8;
}
}
c.cycles += 12;
c.g[V1][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0xb020;
c.g[V1][0] = c.g[V1][0] | 0xb420;
c.sw((c.g[V0][0] as u32), 0u64);
c.g[A0][0] = 0x10000000;
c.sw((c.g[V1][0] as u32), 0u64);
c.g[A0][0] = c.g[A0][0] | 0x2010;
c.g[V1][0] = 0x40000000;
c.g[V0][0] = 0u64;
c.sw((c.g[A0][0] as u32), c.g[V1][0]);
let t = true;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
if t {
break 'b_405de8;
} else {
break 'b_405cd8;
}
}
}
'b_405d18: {
'b_405d14: {
c.cycles += 8;
c.g[A0][0] = 0x10000000;
c.g[A0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x2030));
c.g[V0][0] = c.g[V0][0] | 0x2020;
c.g[V0][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.g[A0][0] << 32;
c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
let t = (c.g[A0][0] as i64) >= 0;
c.sw((c.g[S2][0] as u32).wrapping_add(0x838), c.g[V1][0]);
if t {
break 'b_405d14;
} else {
'b_405d0c: {
c.cycles += 3;
c.g[V1][0] = c.g[V0][0] & 0x1f;
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0x20;
if t {
break 'b_405d0c;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
if t {
break 'b_405d18;
} else {
break 'b_405d0c;
}
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = 0u64;
if t {
break 'b_405d18;
} else {
break 'b_405d14;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0x20;
break 'b_405d18;
}
'b_405de0: {
c.cycles += 7;
c.sw((c.g[S2][0] as u32).wrapping_add(0x83c), c.g[V0][0]);
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0x2010;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.g[V1][0] & 0x4000;
let t = c.g[V1][0] == 0u64;
c.g[A1][0] = 0x490000;
if t {
break 'b_405de0;
} else {
'b_405dd4: {
c.cycles += 37;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00405d40);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7eb0));
Callee { addr: 0x004034a8, f: ee::s15::libmpeg_Error }.call(c, 0x00405d40)?;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = 0x2;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00405d58);
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
Callee { addr: 0x00402c30, f: ee::s15::libmpeg_DispatchCallback }.call(c, 0x00405d58)?;
c.g[V1][0] = 0x40000000;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
c.g[AT][0] = 0x10000000;
c.sw((c.g[AT][0] as u32).wrapping_add(0x2010), c.g[V1][0]);
c.g[V0][0] = 0x3;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[RA][0] = sx(0x00405d78);
c.g[A1][0] = c.g[S0][0];
Callee { addr: 0x00402c30, f: ee::s15::libmpeg_DispatchCallback }.call(c, 0x00405d78)?;
c.g[RA][0] = sx(0x00405d80);
Callee { addr: 0x0042c078, f: ee::s15::DIntr }.call(c, 0x00405d80)?;
c.g[A2][0] = 0x10000000;
c.g[A3][0] = 0x10000000;
c.g[A2][0] = c.g[A2][0] | 0xf520;
c.g[T0][0] = 0x10000;
c.g[V1][0] = c.lw((c.g[A2][0] as u32));
c.g[A3][0] = c.g[A3][0] | 0xf590;
c.g[A0][0] = 0x10000000;
c.g[A1][0] = 0xfffffffffffe0000;
c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
c.g[A0][0] = c.g[A0][0] | 0xb000;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.g[A1][0] = c.g[A1][0] | 0xffff;
c.sw((c.g[A0][0] as u32), 0u64);
c.g[V1][0] = c.lw((c.g[A2][0] as u32));
c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x10000000;
if t {
break 'b_405dd4;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00405dd0);
Callee { addr: 0x0042c0c0, f: ee::s15::EIntr }.call(c, 0x00405dd0)?;
c.g[V0][0] = 0x10000000;
break 'b_405dd4;
}
}
c.cycles += 3;
c.g[S6][0] = 0u64;
c.g[V0][0] = c.g[V0][0] | 0xb020;
c.sw((c.g[V0][0] as u32), 0u64);
break 'b_405de0;
}
}
c.cycles += 2;
c.g[V0][0] = c.g[S6][0];
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
break 'b_405de8;
}
c.cycles += 9;
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
    });
}
