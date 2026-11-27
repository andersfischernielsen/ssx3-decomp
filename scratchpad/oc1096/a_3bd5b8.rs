/// `SNDENC_EncodeChannelMode1` (0x003bd5b8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_encode_channel_mode1(c: &mut Cpu) {
    if c.pc != 0x003bd5b8 {
        return ee::s13::f_003bd5b8__generated(c);
    }
    guest(c, |c| {
c.cycles += 36;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.g[V0][0] = 0x510000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = 0x510000;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S4][0] = c.g[A0][0];
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5394));
c.g[S5][0] = c.g[V0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S2][0] = 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5398));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa20));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[RA][0] = sx(0x003bd610);
c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
Callee { addr: 0x003c03e0, f: ee::s13::SNDENC_FindSubbandPeaks }.call(c, 0x003bd610)?;
c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = 0x450000;
c.g[V1][0] = 0x450000;
c.g[AT][0] = 0x40800000;
c.mtc1(1, 3);
c.g[AT][0] = 0x352f0000;
c.g[AT][0] = c.g[AT][0] | 0x2507;
c.mtc1(1, 2);
c.g[A2][0] = c.g[S3][0];
c.g[T2][0] = c.g[S5][0];
c.g[T1][0] = c.g[S1][0];
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22c8));
c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x38a0));
'l_3bd648: loop {
'b_3bd6d0: {
'b_3bd68c: {
c.cycles += 12;
c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5394));
c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xa20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[V1][0] as u32));
c.mul_s(1, 0, 1);
c.c_lt_s(1, 3);
let t = !c.fcc();

if t {
break 'b_3bd68c;
} else {
c.cycles += 5;
c.cvt_w_s(0, 1);
c.mfc1(2, 0);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
let t = true;
c.g[A0][0] = c.lb((c.g[V0][0] as u32));
if t {
break 'b_3bd6d0;
} else {
break 'b_3bd68c;
}
}
}
c.cycles += 16;
c.mfc1(3, 1);
c.g[V0][0] = c.g[V1][0];
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.mul_s(0, 0, 2);
c.cvt_w_s(4, 0);
c.mfc1(3, 4);
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b8));
c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32));
c.cvt_s_w(0, 0);
c.c_lt_s(1, 0);
let t = !c.fcc();
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b7));
if t {
break 'b_3bd6d0;
} else {
c.cycles += 1;
c.g[A0][0] = c.g[A1][0];
break 'b_3bd6d0;
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
let t = c.g[V0][0] != 0u64;
c.sb((c.g[V1][0] as u32), c.g[A0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x003bd648); } continue 'l_3bd648;
} else {
c.cycles += 11;
c.g[V0][0] = 0xdc;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5398));
c.mult(0, 2, 20, 2);
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V1][0] = 0x450000;
c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa00));
c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x38a0));
c.g[S2][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xea));
c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
'l_3bd710: loop {
c.cycles += 110;
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 20);
c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
c.g[A2][0] = sx((c.g[S2][0] as u32) << 6);
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.g[S0][0];
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(12, (c.g[V0][0] as u32));
c.cvt_s_w(12, 12);
c.div_s(12, 20, 12);
c.g[RA][0] = sx(0x003bd74c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
Callee { addr: 0x003c0564, f: ee::s13::SNDENC_Quantize10 }.call(c, 0x003bd74c)?;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
c.g[A0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xa));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xe));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
c.sb((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x371), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x12));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x370), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x12));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x373), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x16));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x372), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x16));
c.sb((c.g[S1][0] as u32).wrapping_add(0x375), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x374), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x377), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x376), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1e));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x379), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1e));
c.sb((c.g[S1][0] as u32).wrapping_add(0x378), c.g[V1][0]);
let t = c.g[A0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa));
if t {
if event_due(c) { deliver_event_here(c, 0x003bd710); } continue 'l_3bd710;
} else {
'b_3bd998: {
c.cycles += 3;
c.g[V0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x450000;
if t {
break 'b_3bd998;
} else {
c.cycles += 1;
c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x38a0));
'l_3bd8d8: loop {
c.cycles += 48;
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
c.g[A0][0] = sx((c.g[S2][0] as u32) << 6);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(12, (c.g[V0][0] as u32));
c.cvt_s_w(12, 12);
c.div_s(12, 20, 12);
c.g[RA][0] = sx(0x003bd90c);
c.g[A1][0] = c.g[S0][0];
Callee { addr: 0x003c05e0, f: ee::s13::SNDENC_Quantize8 }.call(c, 0x003bd90c)?;
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
c.g[A0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
c.sb((c.g[S1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
c.sb((c.g[S1][0] as u32).wrapping_add(0x371), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
c.sb((c.g[S1][0] as u32).wrapping_add(0x370), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
c.sb((c.g[S1][0] as u32).wrapping_add(0x373), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
c.sb((c.g[S1][0] as u32).wrapping_add(0x372), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
c.sb((c.g[S1][0] as u32).wrapping_add(0x375), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
c.sb((c.g[S1][0] as u32).wrapping_add(0x374), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
c.sb((c.g[S1][0] as u32).wrapping_add(0x377), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf));
c.sb((c.g[S1][0] as u32).wrapping_add(0x376), c.g[V0][0]);
let t = c.g[A0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003bd8d8); } continue 'l_3bd8d8;
} else {
break 'b_3bd998;
}
}
}
}
c.cycles += 10;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return Ok(());
}
}
}
}
    });
}
