/// `libmpeg_WaitBdec` (0x00405b70): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn lib_mpeg_wait_bdec(c: &mut Cpu) {
    if c.pc != 0x00405b70 {
        return ee::s15::f_00405b70__generated(c);
    }
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
