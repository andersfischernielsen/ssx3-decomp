/// `libmpeg_SetupMacroblock` (0x00403e00): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn lib_mpeg_setup_macroblock(c: &mut Cpu) {
    if c.pc != 0x00403e00 {
        return ee::s15::f_00403e00__generated(c);
    }
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
