//! Staging module oc886 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc886`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001b_70d0,
    fe_state_net_account_mem_set_state,
    "cFEStateNetAccountMem_SetState",
)];

/// `void cFEStateNetAccountMem_SetState(this, int state)` (0x001b70d0): stores
/// the account-memory-card state and sets the popup text for it: card checks,
/// overwrite / save / format / delete confirmations, damaged data, or pops the
/// state stack when the flow is done. Calls go through the generated callees
/// (`call_ptr`/`Callee`) and unwind with `?`. A resume after an unwound callee
/// (any other `pc`) runs the generated body.
pub fn fe_state_net_account_mem_set_state(c: &mut Cpu) {
    if c.pc != 0x001b70d0 {
        return ee::s03::f_001b70d0__entries(c);
    }
    guest(c, |c| {
        'b_1b7634: {
            'b_1b7630: {
                c.cycles += 31;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A0][0] = 0x460000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x42e0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x6dc), c.g[S2][0]);
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = sx(0x001b7118);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x001b7118)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.g[V0][0];
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001b7128);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, t, 0x001b7128)?;
                c.g[A1][0] = 0x4a0000;
                c.g[S4][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001b7140);
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x002c2718, 0x001b7140)?;
                c.g[V0][0] = (c.g[S2][0] < 0x1eu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1b7630;
                } else {
                    'b_1b7624: {
                        'b_1b760c: {
                            'b_1b75fc: {
                                'b_1b75e8: {
                                    'b_1b75b0: {
                                        'b_1b75a8: {
                                            'b_1b7588: {
                                                'b_1b7550: {
                                                    'b_1b74c4: {
                                                        'b_1b74b0: {
                                                            'b_1b74a8: {
                                                                'b_1b7434: {
                                                                    'b_1b73ac: {
                                                                        'b_1b7388: {
                                                                            'b_1b7380: {
                                                                                'b_1b737c: {
                                                                                    'b_1b734c: {
                                                                                        'b_1b7328: {
                                                                                            'b_1b729c: {
                                                                                                'b_1b7278: {
                                                                                                    'b_1b7238: {
                                                                                                        'b_1b71ac: {
                                                                                                            'b_1b7188: {
                                                                                                                c.cycles += 6;
                                                                                                                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x43b0));
                                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                                let t = c.g[A0][0] as u32;
                                                                                                                match t {
0x001b7164 => {
c.cycles += 9;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
c.g[RA][0] = sx(0x001b7170);
c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
call_ptr(c, 0x00265d08, 0x001b7170)?;
c.g[RA][0] = sx(0x001b7178);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
call_ptr(c, 0x00265df0, 0x001b7178)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.g[A0][0] = 0x460000;
let t = true;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f58));
if t {
break 'b_1b75b0;
} else {
break 'b_1b7188;
}
}
0x001b7188 => {
break 'b_1b7188;
}
0x001b71ac => {
break 'b_1b71ac;
}
0x001b7238 => {
break 'b_1b7238;
}
0x001b7278 => {
break 'b_1b7278;
}
0x001b729c => {
break 'b_1b729c;
}
0x001b7328 => {
break 'b_1b7328;
}
0x001b734c => {
break 'b_1b734c;
}
0x001b737c => {
break 'b_1b737c;
}
0x001b7380 => {
break 'b_1b7380;
}
0x001b7388 => {
break 'b_1b7388;
}
0x001b73ac => {
break 'b_1b73ac;
}
0x001b7434 => {
break 'b_1b7434;
}
0x001b74a8 => {
break 'b_1b74a8;
}
0x001b74b0 => {
break 'b_1b74b0;
}
0x001b74c4 => {
break 'b_1b74c4;
}
0x001b7550 => {
break 'b_1b7550;
}
0x001b7588 => {
break 'b_1b7588;
}
0x001b760c => {
break 'b_1b760c;
}
0x001b7624 => {
break 'b_1b7624;
}
_ => { c.jump_out(t, 0x001b70d0, 0x001b7650); return Ok(()); }
}
                                                                                                            }
                                                                                                            c.cycles += 9;
                                                                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                                            c.g[RA][0] = sx(0x001b7194);
                                                                                                            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                                                                            call_ptr(c, 0x00265d08, 0x001b7194)?;
                                                                                                            c.g[RA][0] = sx(0x001b719c);
                                                                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                                            call_ptr(c, 0x00265df0, 0x001b719c)?;
                                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                                            c.g[A0][0] = 0x460000;
                                                                                                            let t = true;
                                                                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f58));
                                                                                                            if t {
break 'b_1b75b0;
} else {
break 'b_1b71ac;
}
                                                                                                        }
                                                                                                        c.cycles += 35;
                                                                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                                                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x6d8), 0u64);
                                                                                                        c.g[RA][0] = sx(0x001b71c0);
                                                                                                        c.sw((c.g[S3][0] as u32).wrapping_add(0xb04), 0u64);
                                                                                                        call_ptr(c, 0x00265d08, 0x001b71c0)?;
                                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                                        c.g[A0][0] = 0x460000;
                                                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x42f8));
                                                                                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                                                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                                        c.g[RA][0] = sx(0x001b71e0);
                                                                                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                        call_ptr(c, 0x00317670, 0x001b71e0)?;
                                                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                                                                        c.g[A0][0] = c.g[S0][0];
                                                                                                        let t = c.g[V1][0] as u32;
                                                                                                        c.g[RA][0] = sx(0x001b71f0);
                                                                                                        c.g[A1][0] = c.g[V0][0];
                                                                                                        call_ptr(c, t, 0x001b71f0)?;
                                                                                                        c.g[A1][0] = c.g[V0][0];
                                                                                                        c.g[A2][0] = c.g[S4][0];
                                                                                                        c.g[RA][0] = sx(0x001b7200);
                                                                                                        c.g[A0][0] = c.g[S2][0];
                                                                                                        call_ptr(c, 0x002c26d0, 0x001b7200)?;
                                                                                                        c.g[A0][0] = 0x460000;
                                                                                                        c.g[RA][0] = sx(0x001b720c);
                                                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                                                                                        call_ptr(c, 0x00317670, 0x001b720c)?;
                                                                                                        c.g[A0][0] = 0x460000;
                                                                                                        c.g[S0][0] = c.g[V0][0];
                                                                                                        c.g[RA][0] = sx(0x001b721c);
                                                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                                                                                        call_ptr(c, 0x00317670, 0x001b721c)?;
                                                                                                        c.g[A0][0] = c.g[S3][0];
                                                                                                        c.g[A3][0] = c.g[S0][0];
                                                                                                        c.g[T0][0] = c.g[V0][0];
                                                                                                        c.g[T2][0] = c.g[S2][0];
                                                                                                        c.g[A1][0] = 0x8;
                                                                                                        let t = true;
                                                                                                        c.g[A2][0] = 0x1;
                                                                                                        if t {
break 'b_1b75fc;
} else {
break 'b_1b7238;
}
                                                                                                    }
                                                                                                    'b_1b7264: {
                                                                                                        c.cycles += 6;
                                                                                                        c.g[RA][0] = sx(0x001b7240);
                                                                                                        c.g[A0][0] = c.g[S3][0];
                                                                                                        Callee { addr: 0x001b7778, f: ee::s03::cFEStateNetAccountMem_StoreAccountStrings }.call(c, 0x001b7240)?;
                                                                                                        c.g[V0][0] = 0x1;
                                                                                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6d8));
                                                                                                        let t = c.g[V1][0] == 0u64;
                                                                                                        c.sw((c.g[S3][0] as u32).wrapping_add(0xb04), c.g[V0][0]);
                                                                                                        if t {
break 'b_1b7264;
} else {
c.cycles += 5;
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x001b725c);
c.g[A1][0] = 0x9;
Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b725c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_1b7634;
} else {
break 'b_1b7264;
}
}
                                                                                                    }
                                                                                                    c.cycles += 5;
                                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                                    c.g[RA][0] = sx(0x001b7270);
                                                                                                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                                                                    call_ptr(c, 0x00265e00, 0x001b7270)?;
                                                                                                    let t = true;
                                                                                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                                    if t {
break 'b_1b75a8;
} else {
break 'b_1b7278;
}
                                                                                                }
                                                                                                c.cycles += 9;
                                                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                                c.g[RA][0] = sx(0x001b7284);
                                                                                                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                                                                call_ptr(c, 0x00265d08, 0x001b7284)?;
                                                                                                c.g[RA][0] = sx(0x001b728c);
                                                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                                call_ptr(c, 0x00265df0, 0x001b728c)?;
                                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                                c.g[A0][0] = 0x460000;
                                                                                                let t = true;
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f58));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1b75b0;
                                                                                                } else {
                                                                                                    break 'b_1b729c;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 35;
                                                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                                                            c.sw((c.g[S3][0] as u32).wrapping_add(0xb04), 0u64);
                                                                                            c.g[RA][0] = sx(0x001b72b0);
                                                                                            c.sw((c.g[S3][0] as u32).wrapping_add(0x6d8), 0u64);
                                                                                            call_ptr(c, 0x00265d08, 0x001b72b0)?;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                            c.g[A0][0] = 0x460000;
                                                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4330));
                                                                                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                                                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                                            c.g[RA][0] = sx(0x001b72d0);
                                                                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                            call_ptr(c, 0x00317670, 0x001b72d0)?;
                                                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                                                            c.g[A0][0] = c.g[S0][0];
                                                                                            let t = c.g[V1][0] as u32;
                                                                                            c.g[RA][0] = sx(0x001b72e0);
                                                                                            c.g[A1][0] = c.g[V0][0];
                                                                                            call_ptr(c, t, 0x001b72e0)?;
                                                                                            c.g[A1][0] = c.g[V0][0];
                                                                                            c.g[A2][0] = c.g[SP][0];
                                                                                            c.g[RA][0] = sx(0x001b72f0);
                                                                                            c.g[A0][0] = c.g[S2][0];
                                                                                            call_ptr(c, 0x002c26d0, 0x001b72f0)?;
                                                                                            c.g[A0][0] = 0x460000;
                                                                                            c.g[RA][0] = sx(0x001b72fc);
                                                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                                                                            call_ptr(c, 0x00317670, 0x001b72fc)?;
                                                                                            c.g[A0][0] = 0x460000;
                                                                                            c.g[S0][0] = c.g[V0][0];
                                                                                            c.g[RA][0] = sx(0x001b730c);
                                                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                                                                            call_ptr(c, 0x00317670, 0x001b730c)?;
                                                                                            c.g[A0][0] = c.g[S3][0];
                                                                                            c.g[A3][0] = c.g[S0][0];
                                                                                            c.g[T0][0] = c.g[V0][0];
                                                                                            c.g[T2][0] = c.g[S2][0];
                                                                                            c.g[A1][0] = 0xe;
                                                                                            let t =
                                                                                                true;
                                                                                            c.g[A2][0] = 0x1;
                                                                                            if t {
                                                                                                break 'b_1b75fc;
                                                                                            } else {
                                                                                                break 'b_1b7328;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            9;
                                                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                        c.g[V0]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        c.sw((c.g[S3][0] as u32).wrapping_add(0xb04), c.g[V0][0]);
                                                                                        c.g[RA][0] = sx(0x001b733c);
                                                                                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                                                        call_ptr(c, 0x00265e30, 0x001b733c)?;
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[A0][0] = 0x460000;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4348));
                                                                                        if t {
                                                                                            break 'b_1b75b0;
                                                                                        } else {
                                                                                            break 'b_1b734c;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 6;
                                                                                    c.sw((c.g[S3][0] as u32).wrapping_add(0xb04), 0u64);
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001b7358,
                                                                                    );
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00265a38,
                                                                                        0x001b7358,
                                                                                    )?;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[V0][0]
                                                                                        as i64)
                                                                                        < 17i64)
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[V0][0] =
                                                                                        0x3;
                                                                                    if t {
                                                                                        break 'b_1b7588;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S3]
                                                                                            [0];
                                                                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
                                                                                        c.g[RA][0] = sx(0x001b7374);
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0x11;
                                                                                        Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b7374)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                                                                        if t {
                                                                                            break 'b_1b7634;
                                                                                        } else {
                                                                                            break 'b_1b737c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 1;
                                                                                c.sw((c.g[S3][0] as u32).wrapping_add(0x6d8), 0u64);
                                                                                break 'b_1b7380;
                                                                            }
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.sw(
                                                                                (c.g[S3][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xb04,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            if t {
                                                                                break 'b_1b7630;
                                                                            } else {
                                                                                break 'b_1b7388;
                                                                            }
                                                                        }
                                                                        c.cycles += 9;
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2e0));
                                                                        c.g[RA][0] = sx(0x001b7394);
                                                                        c.g[S2][0] =
                                                                            sx((c.g[S3][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x702,
                                                                                ));
                                                                        call_ptr(
                                                                            c, 0x00265d08,
                                                                            0x001b7394,
                                                                        )?;
                                                                        c.g[RA][0] = sx(0x001b739c);
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2e0));
                                                                        call_ptr(
                                                                            c, 0x00265df0,
                                                                            0x001b739c,
                                                                        )?;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x3f58,
                                                                                ));
                                                                        if t {
                                                                            break 'b_1b75b0;
                                                                        } else {
                                                                            break 'b_1b73ac;
                                                                        }
                                                                    }
                                                                    c.cycles += 34;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_add(0x2e0));
                                                                    c.g[S2][0] = sx((c.g[S3][0]
                                                                        as u32)
                                                                        .wrapping_add(0x702));
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0xb04),
                                                                        0u64,
                                                                    );
                                                                    c.g[RA][0] = sx(0x001b73c0);
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x6d8),
                                                                        0u64,
                                                                    );
                                                                    call_ptr(
                                                                        c, 0x00265d08, 0x001b73c0,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    c.g[A0][0] = 0x460000;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4360));
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x8c));
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V0][0] = c
                                                                        .lh((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x20));
                                                                    c.g[RA][0] = sx(0x001b73e0);
                                                                    c.g[S0][0] = sx((c.g[S0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    call_ptr(
                                                                        c, 0x00317670, 0x001b73e0,
                                                                    )?;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    let t = c.g[V1][0] as u32;
                                                                    c.g[RA][0] = sx(0x001b73f0);
                                                                    c.g[A1][0] = c.g[V0][0];
                                                                    call_ptr(c, t, 0x001b73f0)?;
                                                                    c.g[A1][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x001b73fc);
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    call_ptr(
                                                                        c, 0x002c2508, 0x001b73fc,
                                                                    )?;
                                                                    c.g[A0][0] = 0x460000;
                                                                    c.g[RA][0] = sx(0x001b7408);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x2338));
                                                                    call_ptr(
                                                                        c, 0x00317670, 0x001b7408,
                                                                    )?;
                                                                    c.g[A0][0] = 0x460000;
                                                                    c.g[S0][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x001b7418);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x328));
                                                                    call_ptr(
                                                                        c, 0x00317670, 0x001b7418,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[S3][0];
                                                                    c.g[A3][0] = c.g[S0][0];
                                                                    c.g[T0][0] = c.g[V0][0];
                                                                    c.g[T2][0] = c.g[S2][0];
                                                                    c.g[A1][0] = 0x16;
                                                                    let t = true;
                                                                    c.g[A2][0] = 0x1;
                                                                    if t {
                                                                        break 'b_1b75fc;
                                                                    } else {
                                                                        break 'b_1b7434;
                                                                    }
                                                                }
                                                                'b_1b7458: {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = 0x1;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x6d8));
                                                                    let t = c.g[V1][0] == 0u64;
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0xb04),
                                                                        c.g[V0][0],
                                                                    );
                                                                    if t {
                                                                        break 'b_1b7458;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[A0][0] = c.g[S3][0];
                                                                        c.g[RA][0] = sx(0x001b7450);
                                                                        c.g[A1][0] = 0x17;
                                                                        Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b7450)?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x70),
                                                                        );
                                                                        if t {
                                                                            break 'b_1b7634;
                                                                        } else {
                                                                            break 'b_1b7458;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 20;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x2e0));
                                                                c.g[RA][0] = sx(0x001b7464);
                                                                c.g[S2][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x702));
                                                                call_ptr(
                                                                    c, 0x00265e20, 0x001b7464,
                                                                )?;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4380));
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8c));
                                                                c.g[S1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] = c
                                                                    .lh((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[RA][0] = sx(0x001b7484);
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001b7484,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[A0][0] = c.g[S0][0];
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001b7494);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(c, t, 0x001b7494)?;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001b74a0);
                                                                c.g[A0][0] = c.g[S2][0];
                                                                call_ptr(
                                                                    c, 0x002c2508, 0x001b74a0,
                                                                )?;
                                                                let t = true;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                if t {
                                                                    break 'b_1b75e8;
                                                                } else {
                                                                    break 'b_1b74a8;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            c.g[RA][0] = sx(0x001b74b0);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            Callee { addr: 0x001b7778, f: ee::s03::cFEStateNetAccountMem_StoreAccountStrings }.call(c, 0x001b74b0)?;
                                                            break 'b_1b74b0;
                                                        }
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001b74bc);
                                                        c.g[A1][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x6d4));
                                                        Callee { addr: 0x001b7e40, f: ee::s03::cFEStateNetAccountMem_ShowErrorPopup }.call(c, 0x001b74bc)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x70),
                                                        );
                                                        if t {
                                                            break 'b_1b7634;
                                                        } else {
                                                            break 'b_1b74c4;
                                                        }
                                                    }
                                                    c.cycles += 35;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x2e0)
                                                        );
                                                    c.g[S2][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0xb04),
                                                        0u64,
                                                    );
                                                    c.g[RA][0] = sx(0x001b74d8);
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x6d8),
                                                        0u64,
                                                    );
                                                    call_ptr(c, 0x00265d08, 0x001b74d8)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4398));
                                                    c.g[S0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                    c.g[S1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = c
                                                        .lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                    c.g[RA][0] = sx(0x001b74f8);
                                                    c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    call_ptr(c, 0x00317670, 0x001b74f8)?;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001b7508);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, t, 0x001b7508)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[A2][0] = c.g[SP][0];
                                                    c.g[RA][0] = sx(0x001b7518);
                                                    c.g[A0][0] = c.g[S2][0];
                                                    call_ptr(c, 0x002c26d0, 0x001b7518)?;
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[RA][0] = sx(0x001b7524);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x2338));
                                                    call_ptr(c, 0x00317670, 0x001b7524)?;
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001b7534);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                                    call_ptr(c, 0x00317670, 0x001b7534)?;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[A3][0] = c.g[S0][0];
                                                    c.g[T0][0] = c.g[V0][0];
                                                    c.g[T2][0] = c.g[S2][0];
                                                    c.g[A1][0] = 0x1a;
                                                    let t = true;
                                                    c.g[A2][0] = 0x1;
                                                    if t {
                                                        break 'b_1b75fc;
                                                    } else {
                                                        break 'b_1b7550;
                                                    }
                                                }
                                                'b_1b7574: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = 0x1;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x6d8)
                                                        );
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0xb04),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1b7574;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001b756c);
                                                        c.g[A1][0] = 0x1c;
                                                        Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b756c)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x70),
                                                        );
                                                        if t {
                                                            break 'b_1b7634;
                                                        } else {
                                                            break 'b_1b7574;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                c.g[RA][0] = sx(0x001b7580);
                                                c.g[S2][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                                call_ptr(c, 0x00265e20, 0x001b7580)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                if t {
                                                    break 'b_1b75a8;
                                                } else {
                                                    break 'b_1b7588;
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[V0][0] = 0x1;
                                            c.g[A0][0] = c.g[S3][0];
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0xb04),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001b759c);
                                            c.g[S2][0] =
                                                sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                            Callee { addr: 0x001b7778, f: ee::s03::cFEStateNetAccountMem_StoreAccountStrings }.call(c, 0x001b759c)?;
                                            c.g[RA][0] = sx(0x001b75a4);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                            call_ptr(c, 0x00265e10, 0x001b75a4)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            break 'b_1b75a8;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4310));
                                        break 'b_1b75b0;
                                    }
                                    c.cycles += 14;
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x001b75c4);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x00317670, 0x001b75c4)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b75d4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001b75d4)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001b75e4);
                                    c.g[A2][0] = c.g[SP][0];
                                    call_ptr(c, 0x002c26d0, 0x001b75e4)?;
                                    c.g[A0][0] = c.g[S3][0];
                                    break 'b_1b75e8;
                                }
                                c.cycles += 5;
                                c.g[T2][0] = c.g[S2][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.g[T0][0] = 0u64;
                                break 'b_1b75fc;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b7604);
                            c.g[T1][0] = 0u64;
                            Callee {
                                addr: 0x001b7650,
                                f: ee::s03::cFEStateNetAccountMem_ShowPopup,
                            }
                            .call(c, 0x001b7604)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            if t {
                                break 'b_1b7634;
                            } else {
                                break 'b_1b760c;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x001b761c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f190, 0x001b761c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        if t {
                            break 'b_1b7634;
                        } else {
                            break 'b_1b7624;
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001b7630);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6d4));
                    Callee {
                        addr: 0x001b7e40,
                        f: ee::s03::cFEStateNetAccountMem_ShowErrorPopup,
                    }
                    .call(c, 0x001b7630)?;
                    break 'b_1b7630;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1b7634;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    /// Account-memory state machine fixture: the self object at DATA+0x10000,
    /// the world pointer at gp-0x848 and the SharedLoginMan pointer at gp+0x2e0
    /// point into DATA, the state-handler table is the game's own (ELF image),
    /// and the per-state function pointer at +0x24 is a leaf that returns 0.
    #[test]
    fn fe_state_net_account_mem_set_state_matches() {
        check(
            ee::s03::f_001b70d0__generated,
            fe_state_net_account_mem_set_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x0b70_d0c0 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                let this = DATA + 0x1_0000;
                c.pc = 0x001b_70d0;
                c.g[SP][0] = sx(crate::difftest::SP);
                c.g[RA][0] = sx(crate::difftest::RA);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 36) as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x8c, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x4, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + 0x24, 0x0014_06c0);
                c.bus.write16(DATA + 0x4000 + 0x20, 0x10);
                c.bus.write32(GAME_GP + 0x2e0, DATA + 0x5000);
                c.bus.write32(this + 0x6d8, seed & 1);
            },
        );
    }
}
