//! `src/main/loadscreens.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::events::back_edge_unwinds;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0017_b4e0, debug_menu_update, "DebugMenu_Update")];

/// `DebugMenu_Update` (0x0017b4e0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn debug_menu_update(c: &mut Cpu) {
    if c.pc != 0x0017b4e0 {
        return ee::s02::f_0017b4e0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0017b4e0;
        loop {
            match pc {
                0x0017b4e0 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = 0x65;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0017b520);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x00320c48, 0x0017b520)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x0017b5b4;
                        continue;
                    }
                    pc = 0x0017b528;
                    continue;
                }
                0x0017b528 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0017b530);
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0017d0a8,
                        f: ee::s02::DebugMenu_StartMultiplayer,
                    }
                    .call(c, 0x0017b530)?;
                    c.g[V0][0] = 0x8000;
                    c.g[A0][0] = 0xe;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc1c));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x0017b588;
                        continue;
                    }
                    pc = 0x0017b548;
                    continue;
                }
                0x0017b548 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = 0x1;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x31b8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x60), c.g[V1][0]);
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x0017b56c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x0017b56c)?;
                    c.g[RA][0] = sx(0x0017b574);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00232738, 0x0017b574)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x0017b580);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00316ef8, 0x0017b580)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x0017b5b4;
                        continue;
                    }
                    pc = 0x0017b588;
                    continue;
                }
                0x0017b588 => {
                    c.cycles += 11;
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x31a0));
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x0017b59c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x0017b59c)?;
                    c.g[RA][0] = sx(0x0017b5a4);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00232720, 0x0017b5a4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x0017b5b0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00316ef8, 0x0017b5b0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x0017b5b4;
                    continue;
                }
                0x0017b5b4 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0x3d;
                    c.g[RA][0] = sx(0x0017b5c0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x00320bf0, 0x0017b5c0)?;
                    c.cvt_w_s(1, 0);
                    c.mfc1(22, 1);
                    let t = c.g[S6][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0017b650;
                        continue;
                    }
                    pc = 0x0017b5d4;
                    continue;
                }
                0x0017b5d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x8000;
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc30));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0017b650;
                        continue;
                    }
                    pc = 0x0017b5ec;
                    continue;
                }
                0x0017b5ec => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc1c));
                    c.g[A1][0] = 0xf;

                    if c.g[A1][0] == 0u64 {
                        pc = 0x0017b5fc;
                        continue;
                    }
                    pc = 0x0017b5fc;
                    continue;
                }
                0x0017b5fc => {
                    c.cycles += 6;
                    c.g[A0][0] = 0xd;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe));
                    c.div(0, 2, 5);
                    c.g[V1][0] = c.hi[0];
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc1c), c.g[V1][0]);
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017b614;
                    continue;
                }
                0x0017b614 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x234));
                    if t {
                        pc = 0x0017b628;
                        continue;
                    }
                    pc = 0x0017b620;
                    continue;
                }
                0x0017b620 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1a;
                    if t {
                        pc = 0x0017b634;
                        continue;
                    }
                    pc = 0x0017b628;
                    continue;
                }
                0x0017b628 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) > 0;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x0017c5cc;
                        continue;
                    }
                    pc = 0x0017b630;
                    continue;
                }
                0x0017b630 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1b;
                    pc = 0x0017b634;
                    continue;
                }
                0x0017b634 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0xf;
                    c.div(0, 2, 5);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b644;
                        continue;
                    }
                    pc = 0x0017b644;
                    continue;
                }
                0x0017b644 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc1c), c.g[A0][0]);
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017b650;
                    continue;
                }
                0x0017b650 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8000;
                    if t {
                        pc = 0x0017b708;
                        continue;
                    }
                    pc = 0x0017b658;
                    continue;
                }
                0x0017b658 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc30));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x0017b70c;
                        continue;
                    }
                    pc = 0x0017b66c;
                    continue;
                }
                0x0017b66c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc1c));
                    c.g[A2][0] = 0xf;

                    if c.g[A2][0] == 0u64 {
                        pc = 0x0017b67c;
                        continue;
                    }
                    pc = 0x0017b67c;
                    continue;
                }
                0x0017b67c => {
                    c.cycles += 6;
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.div(0, 2, 6);
                    c.g[V1][0] = c.hi[0];
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xc1c), c.g[V1][0]);
                    if t {
                        pc = 0x0017b6bc;
                        continue;
                    }
                    pc = 0x0017b694;
                    continue;
                }
                0x0017b694 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x8000;
                    if t {
                        pc = 0x0017b6c0;
                        continue;
                    }
                    pc = 0x0017b6a0;
                    continue;
                }
                0x0017b6a0 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x1d;
                    c.g[V1][0] = 0xf;
                    c.div(0, 2, 6);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b6b4;
                        continue;
                    }
                    pc = 0x0017b6b4;
                    continue;
                }
                0x0017b6b4 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.hi[0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xc1c), c.g[A0][0]);
                    pc = 0x0017b6bc;
                    continue;
                }
                0x0017b6bc => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x8000;
                    pc = 0x0017b6c0;
                    continue;
                }
                0x0017b6c0 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0xd;
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc1c));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x0017c5cc;
                        continue;
                    }
                    pc = 0x0017b6d4;
                    continue;
                }
                0x0017b6d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x234));
                    if t {
                        pc = 0x0017b6e8;
                        continue;
                    }
                    pc = 0x0017b6e0;
                    continue;
                }
                0x0017b6e0 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) > 0;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x0017c5d0;
                        continue;
                    }
                    pc = 0x0017b6e8;
                    continue;
                }
                0x0017b6e8 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0xf;
                    c.g[V0][0] = 0x1d;
                    c.div(0, 2, 3);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b6fc;
                        continue;
                    }
                    pc = 0x0017b6fc;
                    continue;
                }
                0x0017b6fc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xc1c), c.g[A0][0]);
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017b708;
                    continue;
                }
                0x0017b708 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    pc = 0x0017b70c;
                    continue;
                }
                0x0017b70c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = (c.g[A0][0] < 0xfu64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x0017c580;
                        continue;
                    }
                    pc = 0x0017b71c;
                    continue;
                }
                0x0017b71c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3180));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x0017b734 => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017b744);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017b744)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = 0x3;
                    c.g[V0][0] = 0x530000;
                    c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    let t = c.g[S6][0] != c.g[A1][0];
                    c.g[S1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x49));
                    if t {
                        pc = 0x0017b80c;
                        continue;
                    }
                    pc = 0x0017b75c;
                    continue;
                }
                0x0017b75c => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = 0x3;
                    c.div(0, 2, 22);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b770;
                        continue;
                    }
                    pc = 0x0017b770;
                    continue;
                }
                0x0017b770 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.hi[0];
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0017b7f8;
                        continue;
                    }
                    pc = 0x0017b780;
                    continue;
                }
                0x0017b780 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0x8000;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc08));
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        pc = 0x0017b7dc;
                        continue;
                    }
                    pc = 0x0017b794;
                    continue;
                }
                0x0017b794 => {
                    c.cycles += 7;
                    c.g[S2][0] = 0x8c;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    c.mult(0, 3, 5, 18);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));

                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                        pc = 0x0017b7c0;
                        continue;
                    }
                    pc = 0x0017b7b0;
                    continue;
                }
                0x0017b7b0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017b7b8);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x0017a028,
                        f: ee::s02::DebugMenu_NextEnabledItem,
                    }
                    .call(c, 0x0017b7b8)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    pc = 0x0017b7c0;
                    continue;
                }
                0x0017b7c0 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x1;
                    c.mult(0, 3, 2, 18);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    let t = true;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x40), c.g[V1][0]);
                    if t {
                        pc = 0x0017b7f8;
                        continue;
                    }
                    pc = 0x0017b7dc;
                    continue;
                }
                0x0017b7dc => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = 0x3;
                    c.div(0, 2, 22);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b7f0;
                        continue;
                    }
                    pc = 0x0017b7f0;
                    continue;
                }
                0x0017b7f0 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.hi[0];
                    c.g[S1][0] = c.g[A0][0];
                    pc = 0x0017b7f8;
                    continue;
                }
                0x0017b7f8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0017b804);
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0017d008,
                        f: ee::s02::DebugMenu_SetGameMode,
                    }
                    .call(c, 0x0017b804)?;
                    let t = true;
                    c.g[V1][0] = 0x530000;
                    if t {
                        pc = 0x0017b8c0;
                        continue;
                    }
                    pc = 0x0017b80c;
                    continue;
                }
                0x0017b80c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0017b8bc;
                        continue;
                    }
                    pc = 0x0017b818;
                    continue;
                }
                0x0017b818 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = 0x3;
                    c.div(0, 2, 5);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b82c;
                        continue;
                    }
                    pc = 0x0017b82c;
                    continue;
                }
                0x0017b82c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.hi[0];
                    c.g[S1][0] = c.g[A0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A1][0] = c.g[S1][0];
                        pc = 0x0017b8b4;
                        continue;
                    }
                    pc = 0x0017b83c;
                    continue;
                }
                0x0017b83c => {
                    c.cycles += 5;
                    c.g[A0][0] = 0x8000;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc08));
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        pc = 0x0017b898;
                        continue;
                    }
                    pc = 0x0017b850;
                    continue;
                }
                0x0017b850 => {
                    c.cycles += 7;
                    c.g[S2][0] = 0x8c;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    c.mult(0, 3, 5, 18);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));

                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                        pc = 0x0017b87c;
                        continue;
                    }
                    pc = 0x0017b86c;
                    continue;
                }
                0x0017b86c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017b874);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x0017a028,
                        f: ee::s02::DebugMenu_NextEnabledItem,
                    }
                    .call(c, 0x0017b874)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    pc = 0x0017b87c;
                    continue;
                }
                0x0017b87c => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x1;
                    c.mult(0, 3, 2, 18);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    let t = true;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x40), c.g[V1][0]);
                    if t {
                        pc = 0x0017b8b0;
                        continue;
                    }
                    pc = 0x0017b898;
                    continue;
                }
                0x0017b898 => {
                    c.cycles += 4;
                    c.div(0, 22, 5);
                    c.g[V1][0] = 0x3;

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0017b8a8;
                        continue;
                    }
                    pc = 0x0017b8a8;
                    continue;
                }
                0x0017b8a8 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.hi[0];
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x0017b8b0;
                    continue;
                }
                0x0017b8b0 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S1][0];
                    pc = 0x0017b8b4;
                    continue;
                }
                0x0017b8b4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0017b8bc);
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0017d008,
                        f: ee::s02::DebugMenu_SetGameMode,
                    }
                    .call(c, 0x0017b8bc)?;
                    pc = 0x0017b8bc;
                    continue;
                }
                0x0017b8bc => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x530000;
                    pc = 0x0017b8c0;
                    continue;
                }
                0x0017b8c0 => {
                    c.cycles += 6;
                    c.g[S7][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[S2][0] = 0x7;
                    c.g[S1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    let t = true;
                    c.g[FP][0] = 0x8000;
                    if t {
                        pc = 0x0017b8f0;
                        continue;
                    }
                    pc = 0x0017b8d8;
                    continue;
                }
                0x0017b8d8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));

                    if c.g[S2][0] == 0u64 {
                        pc = 0x0017b8e4;
                        continue;
                    }
                    pc = 0x0017b8e4;
                    continue;
                }
                0x0017b8e4 => {
                    c.cycles += 3;
                    c.div(0, 2, 18);
                    c.g[V1][0] = c.hi[0];
                    c.g[S1][0] = c.g[V1][0];
                    pc = 0x0017b8f0;
                    continue;
                }
                0x0017b8f0 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                    pc = 0x0017b8f4;
                    continue;
                }
                0x0017b8f4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017b8fc);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00144df0, 0x0017b8fc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x0017b8d8) {
                            return Ok(());
                        }
                        pc = 0x0017b8d8;
                        continue;
                    }
                    pc = 0x0017b904;
                    continue;
                }
                0x0017b904 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    c.g[RA][0] = sx(0x0017b914);
                    c.g[A2][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0017a158,
                        f: ee::s02::DebugMenu_DispatchItemOp,
                    }
                    .call(c, 0x0017b914)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                        pc = 0x0017b938;
                        continue;
                    }
                    pc = 0x0017b91c;
                    continue;
                }
                0x0017b91c => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0017b930);
                    c.g[S7][0] = 0x1;
                    Callee {
                        addr: 0x0017a220,
                        f: ee::s02::DebugMenu_FindNextItem,
                    }
                    .call(c, 0x0017b930)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    pc = 0x0017b938;
                    continue;
                }
                0x0017b938 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        if back_edge_unwinds(c, 0x0017b8f4) {
                            return Ok(());
                        }
                        pc = 0x0017b8f4;
                        continue;
                    }
                    pc = 0x0017b940;
                    continue;
                }
                0x0017b940 => {
                    c.cycles += 2;
                    let t = c.g[S7][0] == 0u64;
                    c.g[V0][0] = 0x530000;
                    if t {
                        pc = 0x0017b9a8;
                        continue;
                    }
                    pc = 0x0017b948;
                    continue;
                }
                0x0017b948 => {
                    c.cycles += 17;
                    c.g[V1][0] = 0x8000;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc20));
                    c.g[A2][0] = 0x8c;
                    c.g[A0][0] = 0x1;
                    c.mult(0, 7, 2, 6);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc20));
                    c.mult(0, 5, 2, 6);
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), 0u64);
                        pc = 0x0017b98c;
                        continue;
                    }
                    pc = 0x0017b98c;
                    continue;
                }
                0x0017b98c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;

                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                        pc = 0x0017b9ac;
                        continue;
                    }
                    pc = 0x0017b998;
                    continue;
                }
                0x0017b998 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0017b9a8);
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, 0x00144b20, 0x0017b9a8)?;
                    pc = 0x0017b9a8;
                    continue;
                }
                0x0017b9a8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x0017b9ac;
                    continue;
                }
                0x0017b9ac => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017b9bc);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017b9bc)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[S0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[RA][0] = sx(0x0017b9d4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001451e8, 0x0017b9d4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x530000;
                    if t {
                        pc = 0x0017bc10;
                        continue;
                    }
                    pc = 0x0017b9dc;
                    continue;
                }
                0x0017b9dc => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = 0xe;
                    c.div(0, 3, 2);

                    if c.g[V0][0] == 0u64 {
                        pc = 0x0017b9f0;
                        continue;
                    }
                    pc = 0x0017b9f0;
                    continue;
                }
                0x0017b9f0 => {
                    c.cycles += 4;
                    c.g[S1][0] = 0xe;
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        pc = 0x0017ba14;
                        continue;
                    }
                    pc = 0x0017ba00;
                    continue;
                }
                0x0017ba00 => {
                    c.cycles += 2;

                    if c.g[S1][0] == 0u64 {
                        pc = 0x0017ba08;
                        continue;
                    }
                    pc = 0x0017ba08;
                    continue;
                }
                0x0017ba08 => {
                    c.cycles += 3;
                    c.div(0, 2, 17);
                    c.g[V1][0] = c.hi[0];
                    c.g[S0][0] = c.g[V1][0];
                    pc = 0x0017ba14;
                    continue;
                }
                0x0017ba14 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0017ba20);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001451e8, 0x0017ba20)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0017ba00) {
                            return Ok(());
                        }
                        pc = 0x0017ba00;
                        continue;
                    }
                    pc = 0x0017ba28;
                    continue;
                }
                0x0017ba28 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x0017bbfc;
                        continue;
                    }
                    pc = 0x0017ba30;
                    continue;
                }
                0x0017ba30 => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017ba40);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017ba40)?;
                    c.g[V1][0] = 0x530000;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017ba70;
                        continue;
                    }
                    pc = 0x0017ba58;
                    continue;
                }
                0x0017ba58 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0017ba64);
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x00144df0, 0x0017ba64)?;
                    c.g[A0][0] = c.g[S3][0];
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0017bc3c;
                        continue;
                    }
                    pc = 0x0017ba70;
                    continue;
                }
                0x0017ba70 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x48));
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017ba7c;
                    continue;
                }
                0x0017ba7c => {
                    c.cycles += 5;
                    c.g[FP][0] = 0u64;
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = 0x7;
                    c.g[S7][0] = 0x2;
                    c.g[A3][0] = 0x3;
                    pc = 0x0017ba90;
                    continue;
                }
                0x0017ba90 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == c.g[A3][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0017baa4;
                        continue;
                    }
                    pc = 0x0017ba98;
                    continue;
                }
                0x0017ba98 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[S7][0];
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x0017babc;
                        continue;
                    }
                    pc = 0x0017baa0;
                    continue;
                }
                0x0017baa0 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6));
                    pc = 0x0017baa4;
                    continue;
                }
                0x0017baa4 => {
                    c.cycles += 2;

                    if c.g[S2][0] == 0u64 {
                        pc = 0x0017baac;
                        continue;
                    }
                    pc = 0x0017baac;
                    continue;
                }
                0x0017baac => {
                    c.cycles += 4;
                    c.div(0, 2, 18);
                    c.g[V1][0] = c.hi[0];
                    c.g[S0][0] = c.g[V1][0];
                    c.g[A0][0] = c.g[S3][0];
                    pc = 0x0017babc;
                    continue;
                }
                0x0017babc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017bac4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00144df0, 0x0017bac4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = 0x3;
                    if t {
                        if back_edge_unwinds(c, 0x0017ba90) {
                            return Ok(());
                        }
                        pc = 0x0017ba90;
                        continue;
                    }
                    pc = 0x0017bacc;
                    continue;
                }
                0x0017bacc => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x8000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017bae4);
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));
                    Callee {
                        addr: 0x0017a158,
                        f: ee::s02::DebugMenu_DispatchItemOp,
                    }
                    .call(c, 0x0017bae4)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));
                        pc = 0x0017bb08;
                        continue;
                    }
                    pc = 0x0017baec;
                    continue;
                }
                0x0017baec => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017bb00);
                    c.g[FP][0] = 0x1;
                    Callee {
                        addr: 0x0017a220,
                        f: ee::s02::DebugMenu_FindNextItem,
                    }
                    .call(c, 0x0017bb00)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));
                    pc = 0x0017bb08;
                    continue;
                }
                0x0017bb08 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A3][0] = 0x3;
                    if t {
                        if back_edge_unwinds(c, 0x0017ba90) {
                            return Ok(());
                        }
                        pc = 0x0017ba90;
                        continue;
                    }
                    pc = 0x0017bb10;
                    continue;
                }
                0x0017bb10 => {
                    c.cycles += 2;
                    let t = c.g[FP][0] == 0u64;
                    c.g[V1][0] = 0x8000;
                    if t {
                        pc = 0x0017bb78;
                        continue;
                    }
                    pc = 0x0017bb18;
                    continue;
                }
                0x0017bb18 => {
                    c.cycles += 17;
                    c.g[A3][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc20));
                    c.g[A2][0] = 0x8c;
                    c.g[A0][0] = 0x1;
                    c.mult(0, 7, 2, 6);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc20));
                    c.mult(0, 5, 2, 6);
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), 0u64);
                        pc = 0x0017bb5c;
                        continue;
                    }
                    pc = 0x0017bb5c;
                    continue;
                }
                0x0017bb5c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;

                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                        pc = 0x0017bb7c;
                        continue;
                    }
                    pc = 0x0017bb68;
                    continue;
                }
                0x0017bb68 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0017bb78);
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, 0x00144b20, 0x0017bb78)?;
                    pc = 0x0017bb78;
                    continue;
                }
                0x0017bb78 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x0017bb7c;
                    continue;
                }
                0x0017bb7c => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017bb8c);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017bb8c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[S0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[RA][0] = sx(0x0017bba4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001451e8, 0x0017bba4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x530000;
                    if t {
                        pc = 0x0017bc10;
                        continue;
                    }
                    pc = 0x0017bbac;
                    continue;
                }
                0x0017bbac => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = 0xe;
                    c.div(0, 3, 2);

                    if c.g[V0][0] == 0u64 {
                        pc = 0x0017bbc0;
                        continue;
                    }
                    pc = 0x0017bbc0;
                    continue;
                }
                0x0017bbc0 => {
                    c.cycles += 4;
                    c.g[S1][0] = 0xe;
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        pc = 0x0017bbe4;
                        continue;
                    }
                    pc = 0x0017bbd0;
                    continue;
                }
                0x0017bbd0 => {
                    c.cycles += 2;

                    if c.g[S1][0] == 0u64 {
                        pc = 0x0017bbd8;
                        continue;
                    }
                    pc = 0x0017bbd8;
                    continue;
                }
                0x0017bbd8 => {
                    c.cycles += 3;
                    c.div(0, 2, 17);
                    c.g[V1][0] = c.hi[0];
                    c.g[S0][0] = c.g[V1][0];
                    pc = 0x0017bbe4;
                    continue;
                }
                0x0017bbe4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0017bbf0);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001451e8, 0x0017bbf0)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0017bbd0) {
                            return Ok(());
                        }
                        pc = 0x0017bbd0;
                        continue;
                    }
                    pc = 0x0017bbf8;
                    continue;
                }
                0x0017bbf8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x0017bbfc;
                    continue;
                }
                0x0017bbfc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017bc0c);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017bc0c)?;
                    c.g[V1][0] = 0x530000;
                    pc = 0x0017bc10;
                    continue;
                }
                0x0017bc10 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017bc2c);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0017d1b8,
                        f: ee::s02::DebugMenu_DispatchItemAction,
                    }
                    .call(c, 0x0017bc2c)?;
                    c.g[S0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x0017c580;
                        continue;
                    }
                    pc = 0x0017bc38;
                    continue;
                }
                0x0017bc38 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    pc = 0x0017bc3c;
                    continue;
                }
                0x0017bc3c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0017bc44);
                    call_ptr(c, 0x00144d98, 0x0017bc44)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017bc58);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017bc58)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017bc60;
                    continue;
                }
                0x0017bc60 => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017bc70);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017bc70)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    let t = c.g[A0][0] != 0u64;
                    c.g[S0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x4a));
                    if t {
                        pc = 0x0017bca8;
                        continue;
                    }
                    pc = 0x0017bc88;
                    continue;
                }
                0x0017bc88 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0017bc94);
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x00144df0, 0x0017bc94)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0017bca0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00144d98, 0x0017bca0)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x0017bd08;
                        continue;
                    }
                    pc = 0x0017bca8;
                    continue;
                }
                0x0017bca8 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        pc = 0x0017bd08;
                        continue;
                    }
                    pc = 0x0017bcb8;
                    continue;
                }
                0x0017bcb8 => {
                    c.cycles += 4;
                    c.g[S4][0] = 0x3;
                    c.g[S1][0] = 0xe;
                    c.g[S3][0] = 0x2;
                    pc = 0x0017bcc8;
                    continue;
                }
                0x0017bcc8 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == c.g[S4][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0017bcdc;
                        continue;
                    }
                    pc = 0x0017bcd0;
                    continue;
                }
                0x0017bcd0 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[S3][0];
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0017bcf4;
                        continue;
                    }
                    pc = 0x0017bcd8;
                    continue;
                }
                0x0017bcd8 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd));
                    pc = 0x0017bcdc;
                    continue;
                }
                0x0017bcdc => {
                    c.cycles += 2;

                    if c.g[S1][0] == 0u64 {
                        pc = 0x0017bce4;
                        continue;
                    }
                    pc = 0x0017bce4;
                    continue;
                }
                0x0017bce4 => {
                    c.cycles += 4;
                    c.div(0, 2, 17);
                    c.g[V1][0] = c.hi[0];
                    c.g[S0][0] = c.g[V1][0];
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x0017bcf4;
                    continue;
                }
                0x0017bcf4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017bcfc);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001451e8, 0x0017bcfc)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x0017bcc8) {
                            return Ok(());
                        }
                        pc = 0x0017bcc8;
                        continue;
                    }
                    pc = 0x0017bd04;
                    continue;
                }
                0x0017bd04 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    pc = 0x0017bd08;
                    continue;
                }
                0x0017bd08 => {
                    c.cycles += 15;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017bd18);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017bd18)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017bd38);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0017d1b8,
                        f: ee::s02::DebugMenu_DispatchItemAction,
                    }
                    .call(c, 0x0017bd38)?;
                    c.g[S0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0017c580;
                        continue;
                    }
                    pc = 0x0017bd44;
                    continue;
                }
                0x0017bd44 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0017bd4c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00144d98, 0x0017bd4c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017bd60);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017bd60)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017bd68;
                    continue;
                }
                0x0017bd68 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017bd78);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017bd78)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x0017bd88);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017bd88)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017bd98);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00145c38, 0x0017bd98)?;
                    c.g[V1][0] = 0x530000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017bdac;
                    continue;
                }
                0x0017bdac => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017bdb8;
                    continue;
                }
                0x0017bdb8 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = c.g[S6][0] ^ 0x2;
                    if t {
                        pc = 0x0017bdc8;
                        continue;
                    }
                    pc = 0x0017bdc0;
                    continue;
                }
                0x0017bdc0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0x1;
                    if t {
                        pc = 0x0017bdcc;
                        continue;
                    }
                    pc = 0x0017bdc8;
                    continue;
                }
                0x0017bdc8 => {
                    c.cycles += 1;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = 0u64;
                    }
                    pc = 0x0017bdcc;
                    continue;
                }
                0x0017bdcc => {
                    c.cycles += 16;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017bddc);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00145cb0, 0x0017bddc)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0017bdf0);
                    c.g[A3][0] = 0xe;
                    call_ptr(c, 0x00145dd0, 0x0017bdf0)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017be04);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017be04)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017be0c;
                    continue;
                }
                0x0017be0c => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017be1c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017be1c)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x530000;
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017be30;
                    continue;
                }
                0x0017be30 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x49));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x48));
                        pc = 0x0017beac;
                        continue;
                    }
                    pc = 0x0017be40;
                    continue;
                }
                0x0017be40 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8c));
                    if t {
                        pc = 0x0017be7c;
                        continue;
                    }
                    pc = 0x0017be4c;
                    continue;
                }
                0x0017be4c => {
                    c.cycles += 12;
                    c.g[S0][0] = 0x8000;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8c));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[S4][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017be70);
                    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
                    Callee {
                        addr: 0x0017a028,
                        f: ee::s02::DebugMenu_NextEnabledItem,
                    }
                    .call(c, 0x0017be70)?;
                    c.g[FP][0] = sx((c.g[S5][0] as u32).wrapping_add(0x88));
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    if t {
                        pc = 0x0017bf58;
                        continue;
                    }
                    pc = 0x0017be7c;
                    continue;
                }
                0x0017be7c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x2;
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[FP][0] = sx((c.g[S5][0] as u32).wrapping_add(0x88));
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                    if t {
                        pc = 0x0017bf58;
                        continue;
                    }
                    pc = 0x0017be90;
                    continue;
                }
                0x0017be90 => {
                    c.cycles += 7;
                    c.g[S0][0] = 0x8000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x0017bea4);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                    Callee {
                        addr: 0x0017a0b8,
                        f: ee::s02::DebugMenu_PrevEnabledItem,
                    }
                    .call(c, 0x0017bea4)?;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    if t {
                        pc = 0x0017bf58;
                        continue;
                    }
                    pc = 0x0017beac;
                    continue;
                }
                0x0017beac => {
                    c.cycles += 9;
                    c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8c));
                    c.g[V0][0] = 0x8000;
                    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
                    c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S7][0] = c.g[A0][0];
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[FP][0] = sx((c.g[S5][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = 0x3;
                    pc = 0x0017bed0;
                    continue;
                }
                0x0017bed0 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));
                    if t {
                        pc = 0x0017beec;
                        continue;
                    }
                    pc = 0x0017bed8;
                    continue;
                }
                0x0017bed8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017bee4);
                    c.g[A2][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x48));
                    Callee {
                        addr: 0x0017a220,
                        f: ee::s02::DebugMenu_FindNextItem,
                    }
                    .call(c, 0x0017bee4)?;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    if t {
                        pc = 0x0017befc;
                        continue;
                    }
                    pc = 0x0017beec;
                    continue;
                }
                0x0017beec => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017bef8);
                    c.g[A2][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x48));
                    Callee {
                        addr: 0x0017a2c8,
                        f: ee::s02::DebugMenu_FindPrevItem,
                    }
                    .call(c, 0x0017bef8)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                    pc = 0x0017befc;
                    continue;
                }
                0x0017befc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));

                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                        pc = 0x0017bf48;
                        continue;
                    }
                    pc = 0x0017bf08;
                    continue;
                }
                0x0017bf08 => {
                    c.cycles += 2;
                    c.g[S0][0] = 0x7;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x0017bf10;
                    continue;
                }
                0x0017bf10 => {
                    c.cycles += 2;

                    if c.g[S0][0] == 0u64 {
                        pc = 0x0017bf18;
                        continue;
                    }
                    pc = 0x0017bf18;
                    continue;
                }
                0x0017bf18 => {
                    c.cycles += 8;
                    c.div(0, 2, 16);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V1][0] = c.hi[0];
                    c.g[S2][0] = c.g[V1][0];
                    c.g[RA][0] = sx(0x0017bf30);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x00144df0, 0x0017bf30)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0017bf10) {
                            return Ok(());
                        }
                        pc = 0x0017bf10;
                        continue;
                    }
                    pc = 0x0017bf38;
                    continue;
                }
                0x0017bf38 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc20));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = 0x3;
                    if t {
                        if back_edge_unwinds(c, 0x0017bed0) {
                            return Ok(());
                        }
                        pc = 0x0017bed0;
                        continue;
                    }
                    pc = 0x0017bf44;
                    continue;
                }
                0x0017bf44 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x0017bf48;
                    continue;
                }
                0x0017bf48 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017bf58);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017bf58)?;
                    pc = 0x0017bf58;
                    continue;
                }
                0x0017bf58 => {
                    c.cycles += 19;
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = 0x8000;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc20));
                    c.g[A2][0] = 0x8c;
                    c.g[A0][0] = 0x1;
                    c.mult(0, 7, 2, 6);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc20));
                    c.mult(0, 5, 2, 6);
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), 0u64);
                        pc = 0x0017bfa4;
                        continue;
                    }
                    pc = 0x0017bfa4;
                    continue;
                }
                0x0017bfa4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;

                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                        pc = 0x0017c1cc;
                        continue;
                    }
                    pc = 0x0017bfb0;
                    continue;
                }
                0x0017bfb0 => {
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0017bfc0);
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, 0x00144b20, 0x0017bfc0)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x0017c1cc;
                        continue;
                    }
                    pc = 0x0017bfc8;
                    continue;
                }
                0x0017bfc8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[S6][0] == c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x0017bfdc;
                        continue;
                    }
                    pc = 0x0017bfd4;
                    continue;
                }
                0x0017bfd4 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x0017bfec;
                        continue;
                    }
                    pc = 0x0017bfdc;
                    continue;
                }
                0x0017bfdc => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x8000;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc24), 0u64);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x0017bfec;
                    continue;
                }
                0x0017bfec => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x8000;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc24));
                    let t = true;
                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                    if t {
                        pc = 0x0017c580;
                        continue;
                    }
                    pc = 0x0017c004;
                    continue;
                }
                0x0017c004 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0017c00c);
                    call_ptr(c, 0x0014dc80, 0x0017c00c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017c018);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x0017c018)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x3;

                    if c.g[S6][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x2;
                        pc = 0x0017c048;
                        continue;
                    }
                    pc = 0x0017c028;
                    continue;
                }
                0x0017c028 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0xa;
                    c.g[A1][0] = 0x8000;
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c03c;
                        continue;
                    }
                    pc = 0x0017c03c;
                    continue;
                }
                0x0017c03c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc28));
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0017c06c;
                        continue;
                    }
                    pc = 0x0017c048;
                    continue;
                }
                0x0017c048 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8000;
                    if t {
                        pc = 0x0017c07c;
                        continue;
                    }
                    pc = 0x0017c050;
                    continue;
                }
                0x0017c050 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0xa;
                    c.g[A1][0] = 0x8000;
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c064;
                        continue;
                    }
                    pc = 0x0017c064;
                    continue;
                }
                0x0017c064 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc28));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x9));
                    pc = 0x0017c06c;
                    continue;
                }
                0x0017c06c => {
                    c.cycles += 4;
                    c.div(0, 2, 4);
                    c.g[V1][0] = c.hi[0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xc28), c.g[V1][0]);
                    c.g[V0][0] = 0x8000;
                    pc = 0x0017c07c;
                    continue;
                }
                0x0017c07c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0017c090);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc28));
                    call_ptr(c, 0x00147448, 0x0017c090)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x0017c0ec;
                        continue;
                    }
                    pc = 0x0017c098;
                    continue;
                }
                0x0017c098 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x8000;
                    if t {
                        pc = 0x0017c0b8;
                        continue;
                    }
                    pc = 0x0017c0a8;
                    continue;
                }
                0x0017c0a8 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc2c));
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc2c), c.g[V0][0]);
                    pc = 0x0017c0b8;
                    continue;
                }
                0x0017c0b8 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x0017c0c0);
                    call_ptr(c, 0x0014dc80, 0x0017c0c0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017c0cc);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x0017c0cc)?;
                    c.g[V1][0] = 0x8000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc2c));
                    c.g[RA][0] = sx(0x0017c0e8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00147658, 0x0017c0e8)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    pc = 0x0017c0ec;
                    continue;
                }
                0x0017c0ec => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017c0fc);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017c0fc)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c104;
                    continue;
                }
                0x0017c104 => {
                    c.cycles += 18;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[S0][0] = 0x530000;
                    c.g[RA][0] = sx(0x0017c118);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017c118)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c138);
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0017d1b8,
                        f: ee::s02::DebugMenu_DispatchItemAction,
                    }
                    .call(c, 0x0017c138)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[S1][0] = c.g[S2][0];
                    }
                    let t = true;
                    c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c158;
                        continue;
                    }
                    pc = 0x0017c14c;
                    continue;
                }
                0x0017c14c => {
                    c.cycles += 1;
                    pc = 0x0017c150;
                    continue;
                }
                0x0017c150 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                        pc = 0x0017c1cc;
                        continue;
                    }
                    pc = 0x0017c158;
                    continue;
                }
                0x0017c158 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;

                    if c.g[S6][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x2;
                        pc = 0x0017c184;
                        continue;
                    }
                    pc = 0x0017c164;
                    continue;
                }
                0x0017c164 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c178;
                        continue;
                    }
                    pc = 0x0017c178;
                    continue;
                }
                0x0017c178 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0017c1a8;
                        continue;
                    }
                    pc = 0x0017c184;
                    continue;
                }
                0x0017c184 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x0017c1b8;
                        continue;
                    }
                    pc = 0x0017c18c;
                    continue;
                }
                0x0017c18c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c1a0;
                        continue;
                    }
                    pc = 0x0017c1a0;
                    continue;
                }
                0x0017c1a0 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    pc = 0x0017c1a8;
                    continue;
                }
                0x0017c1a8 => {
                    c.cycles += 4;
                    c.div(0, 2, 4);
                    c.g[V1][0] = c.hi[0];
                    c.g[S1][0] = c.g[V1][0];
                    c.g[A0][0] = c.g[S3][0];
                    pc = 0x0017c1b8;
                    continue;
                }
                0x0017c1b8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017c1c0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00144d98, 0x0017c1c0)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[S0][0] < 0x2u64) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x0017c150) {
                            return Ok(());
                        }
                        pc = 0x0017c150;
                        continue;
                    }
                    pc = 0x0017c1c8;
                    continue;
                }
                0x0017c1c8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x0017c1cc;
                    continue;
                }
                0x0017c1cc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017c1dc);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017c1dc)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c1e4;
                    continue;
                }
                0x0017c1e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x0017c22c;
                        continue;
                    }
                    pc = 0x0017c1f0;
                    continue;
                }
                0x0017c1f0 => {
                    c.cycles += 10;
                    c.g[V0][0] = 0x8c10;
                    c.g[A1][0] = 0x8000;
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc30));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c218;
                        continue;
                    }
                    pc = 0x0017c218;
                    continue;
                }
                0x0017c218 => {
                    c.cycles += 5;
                    c.div(0, 3, 4);
                    c.g[V0][0] = c.hi[0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = true;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xc30), c.g[V0][0]);
                    if t {
                        pc = 0x0017c270;
                        continue;
                    }
                    pc = 0x0017c22c;
                    continue;
                }
                0x0017c22c => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8000;
                    if t {
                        pc = 0x0017c274;
                        continue;
                    }
                    pc = 0x0017c234;
                    continue;
                }
                0x0017c234 => {
                    c.cycles += 10;
                    c.g[V0][0] = 0x8c10;
                    c.g[A2][0] = 0x8000;
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc30));
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c25c;
                        continue;
                    }
                    pc = 0x0017c25c;
                    continue;
                }
                0x0017c25c => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.div(0, 3, 4);
                    c.g[V0][0] = c.hi[0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc30), c.g[V0][0]);
                    pc = 0x0017c270;
                    continue;
                }
                0x0017c270 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x8000;
                    pc = 0x0017c274;
                    continue;
                }
                0x0017c274 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x8c10;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[RA][0] = sx(0x0017c288);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc30));
                    Callee {
                        addr: 0x0017ac98,
                        f: ee::s02::DebugMenu_SelectBackground,
                    }
                    .call(c, 0x0017c288)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c290;
                    continue;
                }
                0x0017c290 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017c2a0;
                    continue;
                }
                0x0017c2a0 => {
                    c.cycles += 13;
                    c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
                    c.g[S1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x48));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017c2bc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x0017c2bc)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S1][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017c2cc);
                    c.g[A1][0] = c.g[V0][0] ^ 0x1;
                    call_ptr(c, t, 0x0017c2cc)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c2d4;
                    continue;
                }
                0x0017c2d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;

                    if c.g[S6][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x2;
                        pc = 0x0017c304;
                        continue;
                    }
                    pc = 0x0017c2e0;
                    continue;
                }
                0x0017c2e0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0017c2e8);
                    c.g[S0][0] = 0x8000;
                    call_ptr(c, 0x0028b180, 0x0017c2e8)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6234));
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc34));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c2fc;
                        continue;
                    }
                    pc = 0x0017c2fc;
                    continue;
                }
                0x0017c2fc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0017c338;
                        continue;
                    }
                    pc = 0x0017c304;
                    continue;
                }
                0x0017c304 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c30c;
                    continue;
                }
                0x0017c30c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0017c314);
                    c.g[S0][0] = 0x8000;
                    call_ptr(c, 0x0028b180, 0x0017c314)?;
                    c.g[RA][0] = sx(0x0017c31c);
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6234));
                    call_ptr(c, 0x0028b180, 0x0017c31c)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6234));
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc34));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x0017c330;
                        continue;
                    }
                    pc = 0x0017c330;
                    continue;
                }
                0x0017c330 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    pc = 0x0017c338;
                    continue;
                }
                0x0017c338 => {
                    c.cycles += 15;
                    c.div(0, 2, 4);
                    c.g[V1][0] = c.hi[0];
                    c.g[RA][0] = sx(0x0017c348);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc34), c.g[V1][0]);
                    call_ptr(c, 0x0028b180, 0x0017c348)?;
                    c.g[RA][0] = sx(0x0017c350);
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0028b180, 0x0017c350)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc34));
                    c.g[V1][0] = 0x14;
                    c.g[A0][0] = c.g[S1][0];
                    c.mult(0, 5, 5, 3);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a4));
                    c.g[RA][0] = sx(0x0017c36c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, 0x00289520, 0x0017c36c)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c374;
                    continue;
                }
                0x0017c374 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    if t {
                        pc = 0x0017c3bc;
                        continue;
                    }
                    pc = 0x0017c380;
                    continue;
                }
                0x0017c380 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x8000;
                    c.g[A0][0] = 0x3;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x0017c3a8;
                        continue;
                    }
                    pc = 0x0017c398;
                    continue;
                }
                0x0017c398 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017c3a0);
                    call_ptr(c, 0x00256088, 0x0017c3a0)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    c.g[V0][0] = 0x4;
                    pc = 0x0017c3a8;
                    continue;
                }
                0x0017c3a8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    if t {
                        pc = 0x0017c3bc;
                        continue;
                    }
                    pc = 0x0017c3b0;
                    continue;
                }
                0x0017c3b0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0017c3b8);
                    call_ptr(c, 0x002560b0, 0x0017c3b8)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    pc = 0x0017c3bc;
                    continue;
                }
                0x0017c3bc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017c3c4;
                    continue;
                }
                0x0017c3c4 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x0017c3fc;
                        continue;
                    }
                    pc = 0x0017c3cc;
                    continue;
                }
                0x0017c3cc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x8000;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x234));
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc44));
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    let t = (c.g[V1][0] as i64) > 0;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                    if t {
                        pc = 0x0017c430;
                        continue;
                    }
                    pc = 0x0017c3e8;
                    continue;
                }
                0x0017c3e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), c.g[S6][0]);
                        pc = 0x0017c430;
                        continue;
                    }
                    pc = 0x0017c3f4;
                    continue;
                }
                0x0017c3f4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x8000;
                    if t {
                        pc = 0x0017c434;
                        continue;
                    }
                    pc = 0x0017c3fc;
                    continue;
                }
                0x0017c3fc => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V1][0] = 0x8000;
                    if t {
                        pc = 0x0017c434;
                        continue;
                    }
                    pc = 0x0017c404;
                    continue;
                }
                0x0017c404 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x8000;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x234));
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc44));
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) > 0;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                    if t {
                        pc = 0x0017c430;
                        continue;
                    }
                    pc = 0x0017c420;
                    continue;
                }
                0x0017c420 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), 0u64);
                        pc = 0x0017c430;
                        continue;
                    }
                    pc = 0x0017c430;
                    continue;
                }
                0x0017c430 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x8000;
                    pc = 0x0017c434;
                    continue;
                }
                0x0017c434 => {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 5i64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc44), 0u64);
                        pc = 0x0017c448;
                        continue;
                    }
                    pc = 0x0017c448;
                    continue;
                }
                0x0017c448 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x0017c460;
                        continue;
                    }
                    pc = 0x0017c454;
                    continue;
                }
                0x0017c454 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x0017c460;
                    continue;
                }
                0x0017c460 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0x7;
                    c.g[RA][0] = sx(0x0017c46c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017c46c)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x0017c480;
                        continue;
                    }
                    pc = 0x0017c47c;
                    continue;
                }
                0x0017c47c => {
                    c.cycles += 1;
                    c.g[A0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                    pc = 0x0017c480;
                    continue;
                }
                0x0017c480 => {
                    c.cycles += 15;
                    c.g[V1][0] = 0x530000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4b30), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017c4b0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017c4b0)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x0017c530;
                        continue;
                    }
                    pc = 0x0017c4bc;
                    continue;
                }
                0x0017c4bc => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    if t {
                        pc = 0x0017c580;
                        continue;
                    }
                    pc = 0x0017c4c4;
                    continue;
                }
                0x0017c4c4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c4d4;
                    continue;
                }
                0x0017c4d4 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0017c4dc);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00255f50, 0x0017c4dc)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c4f4;
                    continue;
                }
                0x0017c4f4 => {
                    c.cycles += 1;
                    pc = 0x0017c4f8;
                    continue;
                }
                0x0017c4f8 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x0017c500);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    call_ptr(c, 0x00255a20, 0x0017c500)?;
                    c.g[RA][0] = sx(0x0017c508);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x0017c508)?;
                    c.g[RA][0] = sx(0x0017c510);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x003e52b0, 0x0017c510)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x0017c4f8) {
                            return Ok(());
                        }
                        pc = 0x0017c4f8;
                        continue;
                    }
                    pc = 0x0017c528;
                    continue;
                }
                0x0017c528 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    if t {
                        pc = 0x0017c588;
                        continue;
                    }
                    pc = 0x0017c530;
                    continue;
                }
                0x0017c530 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c544;
                    continue;
                }
                0x0017c544 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017c54c);
                    call_ptr(c, 0x00255fa0, 0x0017c54c)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c554;
                    continue;
                }
                0x0017c554 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x234));
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017c560;
                    continue;
                }
                0x0017c560 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x0017c5c8;
                        continue;
                    }
                    pc = 0x0017c568;
                    continue;
                }
                0x0017c568 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    if t {
                        pc = 0x0017c584;
                        continue;
                    }
                    pc = 0x0017c570;
                    continue;
                }
                0x0017c570 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    pc = 0x0017c580;
                    continue;
                }
                0x0017c580 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                    pc = 0x0017c584;
                    continue;
                }
                0x0017c584 => {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    pc = 0x0017c588;
                    continue;
                }
                0x0017c588 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x0017c5cc;
                        continue;
                    }
                    pc = 0x0017c590;
                    continue;
                }
                0x0017c590 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x0017c598);
                    c.g[S0][0] = 0x8000;
                    call_ptr(c, 0x0014dc80, 0x0017c598)?;
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x0017c5a4);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x0017c5a4)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78));
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[RA][0] = sx(0x0017c5c4);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc24), c.g[V0][0]);
                    call_ptr(c, 0x00147488, 0x0017c5c4)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc28), c.g[V0][0]);
                    pc = 0x0017c5c8;
                    continue;
                }
                0x0017c5c8 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    pc = 0x0017c5cc;
                    continue;
                }
                0x0017c5cc => {
                    c.cycles += 1;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x0017c5d0;
                    continue;
                }
                0x0017c5d0 => {
                    c.cycles += 10;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0017b4e0, 0x0017c5f8);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP;

    pub(crate) const SELF: u32 = DATA + 0x2_0000;

    pub(crate) const APP: u32 = DATA + 0x3_0000;

    pub(crate) const BINDING: u32 = DATA + 0x1000;

    pub(crate) const MAP: u32 = DATA + 0x2000;

    /// The menu (`SelectedItem` at `+0x8c1c`, `CurrentBack` at `+0x8c30`),
    /// the app's input binding (a map whose VM stops at once, with value
    /// 0..3 so the bool and the direction vary) and the menu items.
    pub(crate) fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xb4e0, 0);
        c.bus.write32(GAME_GP - 0x848, APP);
        c.bus.write32(APP + 0xb0, BINDING);
        c.bus.write32(BINDING, 0);
        c.bus.write32(BINDING + 4, MAP);
        crate::game::camera::camera::tests::input_map(c, MAP);
        let value = [0.0f32, 1.0, 2.0, 3.0][(seed / 15 % 4) as usize];
        c.bus.write32(SP - 0xb0 - 0x10 - 0x1d0 + 8, value.to_bits());
        for i in 0..0x400u32 {
            c.bus.write32(SELF + 0x8000 + 4 * i, r.next_u32() & 0xf);
        }
        c.bus.write32(SELF + 0x8c1c, seed % 15);
        c.bus
            .write32(SELF + 0x8c30, if seed % 3 == 0 { 0 } else { u32::MAX });
        c.g[A0][0] = SELF as u64;
        c.pc = 0x0017_b4e0;
    }

    #[test]
    fn debug_menu_update_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s02::f_0017b4e0__generated);
            seen.insert(c.cycles - b);
        }
        // The input prologue and the arms that make few calls; the call-heavy
        // arms (race setup) need the whole back end.
        assert!(seen.len() >= 5, "paths: {} {seen:?}", seen.len());
    }

    #[test]
    fn debug_menu_update_matches() {
        check(
            ee::s02::f_0017b4e0__generated,
            debug_menu_update,
            200,
            &setup,
        );
    }
}
