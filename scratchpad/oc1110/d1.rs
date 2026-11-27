/// `SPCH_AssignVariations` (0x003d93c0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn spch_assign_variations(c: &mut Cpu) {
    if c.pc != 0x003d93c0 {
        return ee::s14::f_003d93c0__generated(c);
    }
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
