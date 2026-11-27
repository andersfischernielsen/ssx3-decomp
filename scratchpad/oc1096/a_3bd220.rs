/// `SNDENC_AnalysisFilterbank` (0x003bd220): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_analysis_filterbank(c: &mut Cpu) {
    if c.pc != 0x003bd220 {
        return ee::s13::f_003bd220__generated(c);
    }
    guest(c, |c| {
'b_3bd290: {
'b_3bd28c: {
c.cycles += 19;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[V0][0] = 0x510000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S1][0] = c.g[V0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.g[A0][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S4][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xa3c));
let t = c.g[V0][0] == 0u64;
c.g[S5][0] = c.g[A2][0];
if t {
break 'b_3bd28c;
} else {
c.cycles += 4;
c.g[V1][0] = 0x70000000;
c.g[V0][0] = c.g[S3][0] & c.g[V1][0];
let t = c.g[V0][0] == c.g[V1][0];
c.g[FP][0] = 0u64;
if t {
break 'b_3bd290;
} else {
c.cycles += 4;
c.g[V0][0] = c.g[S5][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
let t = true;
c.g[FP][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_3bd290;
} else {
break 'b_3bd28c;
}
}
}
}
c.cycles += 1;
c.g[FP][0] = 0u64;
break 'b_3bd290;
}
'b_3bd340: {
'b_3bd338: {
'b_3bd31c: {
c.cycles += 2;
let t = c.g[FP][0] == 0u64;
c.g[S0][0] = 0x510000;
if t {
break 'b_3bd31c;
} else {
c.cycles += 23;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
c.g[A3][0] = 0u64;
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A0][0] = c.g[S5][0];
c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[RA][0] = sx(0x003bd2b8);
c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bd2b8)?;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.g[S3][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = 0x200;
c.g[A1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[RA][0] = sx(0x003bd2dc);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
Callee { addr: 0x003bf440, f: ee::s13::SNDENC_PcmToFloat }.call(c, 0x003bd2dc)?;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0xd400;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.g[V1][0] & 0x100;
let t = c.g[V1][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_3bd338;
} else {
c.cycles += 1;
c.g[V1][0] = c.g[V1][0] | 0xd400;
'l_3bd2f8: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;
c.g[S2][0] = 0u64;
if t {
if event_due(c) { deliver_event_here(c, 0x003bd2f8); } continue 'l_3bd2f8;
} else {
c.cycles += 2;
let t = true;
c.g[S7][0] = 0x160;
if t {
break 'b_3bd340;
} else {
break 'b_3bd31c;
}
}
}
}
}
}
c.cycles += 7;
c.g[V0][0] = 0x510000;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[RA][0] = sx(0x003bd338);
c.g[A2][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x4));
Callee { addr: 0x003bf3e0, f: ee::s13::SNDENC_CopyBlock }.call(c, 0x003bd338)?;
break 'b_3bd338;
}
c.cycles += 2;
c.g[S2][0] = 0u64;
c.g[S7][0] = 0x160;
break 'b_3bd340;
}
c.cycles += 2;
c.g[S6][0] = 0x510000;
c.g[S0][0] = 0x510000;
'l_3bd348: loop {
'b_3bd474: {
'b_3bd3cc: {
c.cycles += 2;
let t = c.g[FP][0] == 0u64;
c.g[V1][0] = 0x510000;
if t {
break 'b_3bd3cc;
} else {
'b_3bd384: {
c.cycles += 5;
c.g[A0][0] = 0xe0;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5398));
c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
let t = c.g[V0][0] != c.g[A0][0];
c.g[V1][0] = sx((c.g[S2][0] as u32) << 9);
if t {
break 'b_3bd384;
} else {
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
c.g[RA][0] = sx(0x003bd37c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
Callee { addr: 0x003bf4c0, f: ee::s13::SNDENC_WindowFilter4 }.call(c, 0x003bd37c)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd474;
} else {
break 'b_3bd384;
}
}
}
'b_3bd3ac: {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S7][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd3ac;
} else {
c.cycles += 8;
c.g[V1][0] = sx((c.g[S2][0] as u32) << 9);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
c.g[RA][0] = sx(0x003bd3a4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
Callee { addr: 0x003bf5a0, f: ee::s13::SNDENC_WindowFilter6 }.call(c, 0x003bd3a4)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd474;
} else {
break 'b_3bd3ac;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[S2][0] as u32) << 9);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
c.g[RA][0] = sx(0x003bd3c4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
Callee { addr: 0x003bf6b0, f: ee::s13::SNDENC_WindowFilter8 }.call(c, 0x003bd3c4)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd474;
} else {
break 'b_3bd3cc;
}
}
}
'b_3bd454: {
'b_3bd420: {
c.cycles += 14;
c.g[A3][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.g[S3][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = 0x80;
c.g[A1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[RA][0] = sx(0x003bd3f0);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
Callee { addr: 0x003bf440, f: ee::s13::SNDENC_PcmToFloat }.call(c, 0x003bd3f0)?;
c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x5398));
c.g[V1][0] = 0xe0;
c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
let t = c.g[V0][0] != c.g[V1][0];

if t {
break 'b_3bd420;
} else {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd418);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
Callee { addr: 0x003bf4c0, f: ee::s13::SNDENC_WindowFilter4 }.call(c, 0x003bd418)?;
let t = true;
c.g[A0][0] = 0x510000;
if t {
break 'b_3bd454;
} else {
break 'b_3bd420;
}
}
}
'b_3bd440: {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S7][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd440;
} else {
c.cycles += 6;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd438);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
Callee { addr: 0x003bf5a0, f: ee::s13::SNDENC_WindowFilter6 }.call(c, 0x003bd438)?;
let t = true;
c.g[A0][0] = 0x510000;
if t {
break 'b_3bd454;
} else {
break 'b_3bd440;
}
}
}
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd450);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
Callee { addr: 0x003bf6b0, f: ee::s13::SNDENC_WindowFilter8 }.call(c, 0x003bd450)?;
c.g[A0][0] = 0x510000;
break 'b_3bd454;
}
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.g[A0][0];
c.g[RA][0] = sx(0x003bd470);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x200));
Callee { addr: 0x003bf3e0, f: ee::s13::SNDENC_CopyBlock }.call(c, 0x003bd470)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
break 'b_3bd474;
}
c.cycles += 49;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x100));
c.g[RA][0] = sx(0x003bd484);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bf800, f: ee::s13::SNDENC_FilterStage1 }.call(c, 0x003bd484)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd494);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bf8d0, f: ee::s13::SNDENC_FilterStage2 }.call(c, 0x003bd494)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd4a4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003bf960, f: ee::s13::SNDENC_FilterStage3 }.call(c, 0x003bd4a4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd4b4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bfac0, f: ee::s13::SNDENC_FilterStage4 }.call(c, 0x003bd4b4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd4c4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003bfb90, f: ee::s13::SNDENC_FilterStage5 }.call(c, 0x003bd4c4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
c.g[RA][0] = sx(0x003bd4d4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bfc50, f: ee::s13::SNDENC_FilterStage6 }.call(c, 0x003bd4d4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa2c));
c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd4e8);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bfd60, f: ee::s13::SNDENC_FilterStage7 }.call(c, 0x003bd4e8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa30));
c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd4fc);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003bfe80, f: ee::s13::SNDENC_FilterStage8 }.call(c, 0x003bd4fc)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa34));
c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd510);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bffa0, f: ee::s13::SNDENC_FilterStage9 }.call(c, 0x003bd510)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = c.g[S4][0];
c.g[A3][0] = 0x40;
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa38));
c.g[RA][0] = sx(0x003bd52c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003c01a0, f: ee::s13::SNDENC_FilterStage10 }.call(c, 0x003bd52c)?;
c.g[V0][0] = ((c.g[S2][0] as i64) < 4i64) as u64;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x003bd348); } continue 'l_3bd348;
} else {
'b_3bd588: {
'b_3bd56c: {
c.cycles += 2;
let t = c.g[FP][0] == 0u64;
c.g[A0][0] = 0x510000;
if t {
break 'b_3bd56c;
} else {
c.cycles += 11;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
c.g[A1][0] = c.g[S5][0];
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[A3][0] = 0x10000;
c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd564);
c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
Callee { addr: 0x003bca28, f: ee::s13::SNDENC_DmaFromSpr }.call(c, 0x003bd564)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_3bd588;
} else {
break 'b_3bd56c;
}
}
}
c.cycles += 7;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
c.g[A1][0] = c.g[S5][0];
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[RA][0] = sx(0x003bd584);
c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
Callee { addr: 0x003bf3e0, f: ee::s13::SNDENC_CopyBlock }.call(c, 0x003bd584)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_3bd588;
}
c.cycles += 11;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
}
}
    });
}
