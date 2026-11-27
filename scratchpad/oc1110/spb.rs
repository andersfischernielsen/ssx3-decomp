/// `libmpeg_SetupPredBlock` (0x00404758): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn lib_mpeg_setup_pred_block(c: &mut Cpu) {
    if c.pc != 0x00404758 {
        return ee::s15::f_00404758__generated(c);
    }
    guest(c, |c| {
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
c.pc = t; return Ok(());
    });
}
