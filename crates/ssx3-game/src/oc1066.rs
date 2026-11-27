//! Staging module oc1066 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1066`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_0198, tex_man_swizzle_t8, "TexMan_SwizzleT8"),
    (
        0x0039_9970,
        ui_list_box_handle_input,
        "UIListBox_HandleInput",
    ),
];

/// `int TexMan_SwizzleT8(self, int w, int h, void* dst, void* src)` (0x390198):
/// swizzles an 8-bit texture of w x h (each a power of two from 1024 down to 1,
/// else it returns -1; sizes are checked with the same index scan the original
/// does) block by block: each 128x64 block is copied row by row into a stack
/// buffer, run through `TexMan_SwizzleT8Page` (0x38fc48) and copied row by row
/// to its destination block. Stack frame 0x40d0: self at +0x4000, dst +0x4004,
/// src +0x4008, row count +0x400c, block height +0x4010 and half height
/// +0x4014. Returns 0 on success.
pub fn tex_man_swizzle_t8(c: &mut Cpu) {
    guest(c, |c| {
        'b_390204: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40d0));
            c.g[V0][0] = 0x400;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40c0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40b0));
            c.g[S0][0] = c.g[A1][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4000), c.g[A0][0]);
            c.g[S1][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40a0));
            c.g[A0][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x4090));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x4080));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x4070));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x4060));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x4050));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x4040));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x4030), c.g[RA][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4004), c.g[A3][0]);
            let t = c.g[S0][0] == c.g[V0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4008), c.g[T0][0]);
            if t {
                break 'b_390204;
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0x400;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                'l_3901f0: loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 11i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] =
                        sx(((c.g[V1][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
                    if t {
                        break 'b_390204;
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x003901f0);
                            }
                            continue 'l_3901f0;
                        } else {
                            break 'b_390204;
                        }
                    }
                }
            }
        }
        'b_390428: {
            'b_390248: {
                'b_390240: {
                    c.cycles += 3;
                    c.g[V0][0] = 0xb;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = 0x400;
                    if t {
                        break 'b_390240;
                    } else {
                        'b_390234: {
                            c.cycles += 2;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_390234;
                            } else {
                                c.cycles += 2;
                                c.g[V1][0] = 0x400;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                'l_390220: loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A0][0] as i64) < 11i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32
                                        >> ((c.g[A0][0] as u32) & 31))
                                        as u32);
                                    if t {
                                        break 'b_390234;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S1][0] != c.g[V0][0] {
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00390220);
                                            }
                                            continue 'l_390220;
                                        } else {
                                            break 'b_390234;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0xb;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[SP][0];
                        if t {
                            break 'b_390248;
                        } else {
                            break 'b_390240;
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_390428;
                } else {
                    break 'b_390248;
                }
            }
            'b_3902b4: {
                'b_3902ac: {
                    c.cycles += 22;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00390254);
                    c.g[A2][0] = 0x2000;
                    call_ptr(c, 0x00416210, 0x00390254)?;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2000));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00390264);
                    c.g[A2][0] = 0x2000;
                    call_ptr(c, 0x00416210, 0x00390264)?;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[A2][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3e));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7e));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    if c.g[A2][0] != 0 {
                        c.g[V0][0] = c.g[A1][0];
                    }
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.g[A3][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    let t = c.g[A0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x400c), c.g[V0][0]);
                    if t {
                        break 'b_3902ac;
                    } else {
                        c.cycles += 3;
                        c.g[S6][0] = c.g[S0][0];
                        let t = true;
                        c.g[S7][0] = sx((c.g[S6][0] as u32) << 1);
                        if t {
                            break 'b_3902b4;
                        } else {
                            break 'b_3902ac;
                        }
                    }
                }
                c.cycles += 2;
                c.g[S6][0] = 0x80;
                c.g[S7][0] = 0x100;
                break 'b_3902b4;
            }
            'b_3902e0: {
                'b_3902d8: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400c));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x40;
                    if t {
                        break 'b_3902d8;
                    } else {
                        c.cycles += 5;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4010), c.g[S1][0]);
                        c.g[V0][0] = sx((c.g[S1][0] as u32) >> 31);
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        let t = true;
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                        if t {
                            break 'b_3902e0;
                        } else {
                            break 'b_3902d8;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 0x20;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4010), c.g[A1][0]);
                break 'b_3902e0;
            }
            'b_390424: {
                c.cycles += 4;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4014), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400c));
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_390424;
                } else {
                    c.cycles += 10;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4014));
                    c.g[V0][0] = sx((c.g[FP][0] as u32) << 6);
                    c.mult(0, 3, 23, 5);
                    c.mult(0, 5, 22, 2);
                    c.mult(0, 0, 3, 30);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4018), c.g[A1][0]);
                    c.g[V0][0] = c.lo[0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4020), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    'l_390318: loop {
                        'b_390410: {
                            c.cycles += 3;
                            c.g[S3][0] = 0u64;
                            let t = (c.g[FP][0] as i64) <= 0;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4028), c.g[V1][0]);
                            if t {
                                break 'b_390410;
                            } else {
                                c.cycles += 15;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4008));
                                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2000));
                                c.lo[0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4020));
                                c.madd(0, 0, 2, 4);
                                c.g[V0][0] = c.lo[0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x4024), c.g[V0][0]);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4004));
                                c.lo[0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4018));
                                c.madd(0, 0, 2, 4);
                                c.g[V0][0] = c.lo[0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x401c), c.g[V0][0]);
                                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                'l_390360: loop {
                                    'b_3903a8: {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x401c));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x402c), c.g[V1][0]);
                                        c.g[S2][0] = c.g[SP][0];
                                        c.lo[0] = c.g[V0][0];
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4010));
                                        let t = (c.g[V1][0] as i64) <= 0;
                                        c.madd(0, 17, 22, 19);
                                        if t {
                                            break 'b_3903a8;
                                        } else {
                                            c.cycles += 3;
                                            c.mult(0, 20, 22, 30);
                                            c.g[S0][0] = c.g[V1][0];
                                            'l_390388: loop {
                                                c.cycles += 8;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A2][0] = c.g[S6][0];
                                                c.g[RA][0] = sx(0x0039039c);
                                                c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[S4][0] as u32)));
                                                call_ptr(c, 0x0041605c, 0x0039039c)?;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x80));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00390388);
                                                    }
                                                    continue 'l_390388;
                                                } else {
                                                    break 'b_3903a8;
                                                }
                                            }
                                        }
                                    }
                                    'b_390400: {
                                        c.cycles += 12;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4000));
                                        c.g[A1][0] = 0x80;
                                        c.g[A2][0] = 0x40;
                                        c.g[A3][0] = c.g[SP][0];
                                        c.g[T0][0] = c.g[S5][0];
                                        c.g[RA][0] = sx(0x003903c4);
                                        c.g[S2][0] = c.g[S5][0];
                                        call_ptr(c, 0x0038fc48, 0x003903c4)?; // TexMan_SwizzleT8Page
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4024));
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4014));
                                        c.lo[0] = c.g[V0][0];
                                        let t = (c.g[V1][0] as i64) <= 0;
                                        c.madd(0, 17, 23, 19);
                                        if t {
                                            break 'b_390400;
                                        } else {
                                            c.cycles += 2;
                                            c.mult(0, 19, 23, 30);
                                            c.g[S0][0] = c.g[V1][0];
                                            'l_3903e0: loop {
                                                c.cycles += 8;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A2][0] = c.g[S7][0];
                                                c.g[RA][0] = sx(0x003903f4);
                                                c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[S3][0] as u32)));
                                                call_ptr(c, 0x0041605c, 0x003903f4)?;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x100));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003903e0);
                                                    }
                                                    continue 'l_3903e0;
                                                } else {
                                                    break 'b_390400;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x402c));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00390360);
                                        }
                                        continue 'l_390360;
                                    } else {
                                        break 'b_390410;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4028));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400c));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00390318);
                            }
                            continue 'l_390318;
                        } else {
                            break 'b_390424;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_390428;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40c0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40b0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40a0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x4090));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x4080));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x4070));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x4060));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x4050));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x4040));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x4030));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40d0));
        c.pc = t;
        return Ok(());
    })
}

/// `bool UIListBox_HandleInput(UIListBox* self, Input* in)` (0x399970): the
/// list box's navigation. With no entries (byte +0x318) it returns 0. Otherwise
/// the input object's vtable (slots +0x88/+0x90/+0x98/+0xa0, +0x30 for the
/// accept key) is asked whether up (19 / 18), down (20 / 17) or the accept
/// key is pressed; the current entry (byte +0x319) moves (wrapping when the
/// attribute word +0x14 has bit 7), the screen object gets focus-notify (9 /
/// 6) and focus-move calls, a key-frame thread is started when the move
/// result has bit 8, and the UI sound listener is told the result through
/// its vtable (+0x14). Returns 1 when the input was consumed, else 0.
pub fn ui_list_box_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_399d60: {
            'b_399d5c: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A1][0];
                if t {
                    break 'b_399d5c;
                } else {
                    'b_399d38: {
                        'b_399d2c: {
                            'b_399d20: {
                                'b_399b18: {
                                    'b_399a10: {
                                        'b_3999e4: {
                                            c.cycles += 13;
                                            c.g[S2][0] =
                                                sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                                            c.g[RA][0] = sx(0x003999a4);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039ff50, 0x003999a4)?; // UIObject_GetStateData
                                            c.g[S3][0] = 0u64;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                                            if c.g[A1][0] != 0 {
                                                c.g[S2][0] = c.g[A1][0];
                                            }
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x003999c4);
                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x003999c4)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_3999e4;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V0][0] = c.lbu(
                                                        (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    );
                                                    break 'b_399a10;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_3999e4;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x003999f4);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x003999f4)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_399b18;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_399b18;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lbu((c.g[S0][0] as u32).wrapping_add(0x319));
                                                break 'b_399a10;
                                            }
                                        }
                                    }
                                    'b_399a8c: {
                                        'b_399a48: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            if t {
                                                break 'b_399a48;
                                            } else {
                                                c.cycles += 12;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.sb(
                                                    (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    c.g[V0][0],
                                                );
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A2][0] = 0x9;
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399a40);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00399a40)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_399a8c;
                                                } else {
                                                    break 'b_399a48;
                                                }
                                            }
                                        }
                                        'b_399a88: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399a88;
                                            } else {
                                                c.cycles += 11;
                                                c.g[V0][0] =
                                                    c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A2][0] = 0x9;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                c.sb(
                                                    (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    c.g[V0][0],
                                                );
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399a88);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00399a88)?;
                                                break 'b_399a88;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_399a8c;
                                    }
                                    'b_399ab0: {
                                        'b_399aa8: {
                                            c.cycles += 4;
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399aa8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                let t = true;
                                                c.g[A2][0] = 0u64;
                                                if t {
                                                    break 'b_399ab0;
                                                } else {
                                                    break 'b_399aa8;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                        c.g[A2][0] = 0x3;
                                        break 'b_399ab0;
                                    }
                                    'b_399b08: {
                                        'b_399b04: {
                                            c.cycles += 10;
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399ac8);
                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00399ac8)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S1][0] & 0x100;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x8;
                                            if t {
                                                break 'b_399b04;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x00399ae0);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x0039ff78, 0x00399ae0)?; // UIObject_GetEventThread
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_399b08;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[A0][0] == 0u64;

                                                    if t {
                                                        break 'b_399b08;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[RA][0] = sx(0x00399b04);
                                                        c.g[A2][0] = 0x1;
                                                        call_ptr(c, 0x0039c870, 0x00399b04)?; // UIScreen_StartThread
                                                        break 'b_399b04;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S1][0] & 0x1;
                                        break 'b_399b08;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[S1][0] & 0x10;
                                    if t {
                                        break 'b_399d20;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[S3][0] = 0x1;
                                        if t {
                                            break 'b_399d38;
                                        } else {
                                            break 'b_399b18;
                                        }
                                    }
                                }
                                'b_399c78: {
                                    'b_399b74: {
                                        'b_399b48: {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399b28);
                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00399b28)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_399b48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] = c.lbu(
                                                        (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    );
                                                    break 'b_399b74;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_399b48;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x00399b58);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x00399b58)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_399c78;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_399c78;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V1][0] =
                                                    c.lbu((c.g[S0][0] as u32).wrapping_add(0x319));
                                                break 'b_399b74;
                                            }
                                        }
                                    }
                                    'b_399bec: {
                                        'b_399bb4: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399bb4;
                                            } else {
                                                c.cycles += 11;
                                                c.sb(
                                                    (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    c.g[V1][0],
                                                );
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A2][0] = 0x9;
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399bac);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00399bac)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_399bec;
                                                } else {
                                                    break 'b_399bb4;
                                                }
                                            }
                                        }
                                        'b_399be8: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A2][0] = 0x9;
                                            if t {
                                                break 'b_399be8;
                                            } else {
                                                c.cycles += 8;
                                                c.sb((c.g[S0][0] as u32).wrapping_add(0x319), 0u64);
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399be8);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00399be8)?;
                                                break 'b_399be8;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_399bec;
                                    }
                                    'b_399c10: {
                                        'b_399c08: {
                                            c.cycles += 4;
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399c08;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                let t = true;
                                                c.g[A2][0] = 0x1;
                                                if t {
                                                    break 'b_399c10;
                                                } else {
                                                    break 'b_399c08;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                        c.g[A2][0] = 0x2;
                                        break 'b_399c10;
                                    }
                                    'b_399c68: {
                                        'b_399c64: {
                                            c.cycles += 10;
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399c28);
                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00399c28)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S1][0] & 0x100;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x9;
                                            if t {
                                                break 'b_399c64;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x00399c40);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x0039ff78, 0x00399c40)?; // UIObject_GetEventThread
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_399c68;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[A0][0] == 0u64;

                                                    if t {
                                                        break 'b_399c68;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[RA][0] = sx(0x00399c64);
                                                        c.g[A2][0] = 0x1;
                                                        call_ptr(c, 0x0039c870, 0x00399c64)?; // UIScreen_StartThread
                                                        break 'b_399c64;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S1][0] & 0x1;
                                        break 'b_399c68;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S3][0] = 0x1;
                                        break 'b_399d38;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.g[S1][0] & 0x10;
                                        if t {
                                            break 'b_399d20;
                                        } else {
                                            break 'b_399c78;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00399c88);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x00399c88)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[S0][0];
                                if t {
                                    break 'b_399d5c;
                                } else {
                                    'b_399d0c: {
                                        'b_399d08: {
                                            c.cycles += 21;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                            c.g[A2][0] = 0x6;
                                            c.g[A3][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00399cb0);
                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00399cb0)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A2][0] = 0x4;
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399cd4);
                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00399cd4)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S1][0] & 0x100;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x3;
                                            if t {
                                                break 'b_399d08;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x00399cec);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x0039ff78, 0x00399cec)?; // UIObject_GetEventThread
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_399d0c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    c.g[RA][0] = sx(0x00399d08);
                                                    c.g[A2][0] = 0x1;
                                                    call_ptr(c, 0x0039c870, 0x00399d08)?; // UIScreen_StartThread
                                                    break 'b_399d08;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S1][0] & 0x1;
                                        break 'b_399d0c;
                                    }
                                    'b_399d1c: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[S0][0];
                                        if t {
                                            break 'b_399d1c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_399d2c;
                                            } else {
                                                break 'b_399d1c;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.g[S1][0] & 0x10;
                                    break 'b_399d20;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_399d38;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = 0u64;
                                break 'b_399d2c;
                            }
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x00399d34);
                        call_ptr(c, 0x003a0318, 0x00399d34)?; // UIObject_GetUserData
                        c.g[S3][0] = c.g[V0][0];
                        break 'b_399d38;
                    }
                    'b_399d54: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.g[A1][0] = c.g[S3][0];
                        if t {
                            break 'b_399d54;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00399d54);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00399d54)?;
                            break 'b_399d54;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_399d60;
                    } else {
                        break 'b_399d5c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_399d60;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// `TexMan_SwizzleT8`: valid sizes (first index of the scan, later ones,
    /// one block or several, one block row or several) and sizes that are
    /// not a power of two (w, then h, returns -1). The real memset, row
    /// copier and `TexMan_SwizzleT8Page` run on random texture bytes.
    #[test]
    fn tex_man_swizzle_t8_matches() {
        const SIZES: [(u32, u32); 12] = [
            (128, 64),
            (256, 128),
            (64, 32),
            (1, 1),
            (1024, 16),
            (128, 1024),
            (100, 64),
            (128, 100),
            (3, 1024),
            (512, 256),
            (32, 256),
            (256, 8),
        ];
        check(
            ee::s12::f_00390198__generated,
            tex_man_swizzle_t8,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_0198, 0);
                elf_image(c);
                let (w, h) = SIZES[(seed % SIZES.len() as u32) as usize];
                let dst = DATA + 0x1_0000;
                let src = DATA + 0x5_0000;
                // Valid sizes copy w*h bytes; keep the fill bounded.
                fill(c, &mut r, src, (w * h / 4).min(0x8000) + 1);
                fill(c, &mut r, dst, 0x100);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = w as u64;
                c.g[A2][0] = h as u64;
                c.g[A3][0] = sx(dst);
                c.g[T0][0] = sx(src);
                c.pc = 0x0039_0198;
            },
        );
    }

    const OC1066_GATE: u32 = DATA + 0x2c_0000;
    const OC1066_MAGIC: u32 = 0x1066_0001;
    fn oc1066_no_prev(_c: &mut Cpu) {
        unreachable!("oc1066 fake called before it was hooked");
    }

    /// Gated fake of `UIScreen_StartThread` (0x39c870): logs and returns.
    fn oc1066_fake_start_thread(c: &mut Cpu) {
        if c.bus.read32(OC1066_GATE) != OC1066_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1066_fake_start_thread),
                oc1066_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0039_c870, ra);
    }

    /// `UIListBox_HandleInput`: the input vtable's up/down/accept slots answer
    /// zero (a real leaf) or non-zero (a logged fake) at random, the attribute
    /// word and the current entry are random (wrap or not), and the move
    /// result (read through the real `UIObject_GetStateData` chain) takes the
    /// values 0, 1, 0x10 and 0x100 in all combinations (with and without a
    /// key-frame thread for the event). The listener is the object's own or
    /// the default one in the globals. Fakes unwind at random calls.
    #[test]
    fn ui_list_box_handle_input_matches() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc1066_fake_start_thread,
            fakes::prev_cell(oc1066_fake_start_thread),
        );
        covered(
            ee::s13::f_00399970__generated,
            ui_list_box_handle_input,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x0039_9970, 6);
                let zero = 0x0014_06c0u32;
                let this = DATA + 0x1000;
                let scr = DATA + 0x1400;
                let a3 = DATA + 0x1800;
                let ov = DATA + 0x1c00;
                let state = DATA + 0x2000;
                let listener = DATA + 0x2400;
                let lvt = DATA + 0x2800;
                let table = DATA + 0x1200;
                let p = DATA + 0x3000;
                let q = DATA + 0x3400;
                let inp = DATA + 0x4000;
                let iv = DATA + 0x4400;
                c.bus.write32(OC1066_GATE, OC1066_MAGIC);
                // The list box.
                let count = if seed % 9 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 6
                };
                c.bus.write8(this + 0x318, count as u8);
                c.bus
                    .write8(this + 0x319, (r.next_u32() % (count + 1)) as u8);
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus
                    .write32(this + 0x5c, if seed % 11 == 0 { 0 } else { scr });
                c.bus
                    .write32(this + 0x3c, if seed % 3 == 0 { 0 } else { table });
                for k in 0..16u32 {
                    let v = if r.next_u32() % 2 == 0 {
                        0xffff
                    } else {
                        0x10 + k
                    };
                    c.bus.write16(table + 2 * k, v as u16);
                }
                // Screen object chain: scr+0xd0 -> a3, a3+0x10 -> state.
                c.bus.write32(scr + 0xd0, a3);
                c.bus.write32(a3 + 8, ov);
                c.bus.write32(a3 + 0x10, state);
                c.bus
                    .write32(state + 0x14, if seed % 2 == 0 { 0 } else { listener });
                c.bus.write32(listener, lvt);
                c.bus.write32(GAME_GP + 0x2968, lvt);
                c.bus
                    .write16(lvt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(lvt + 0x14, fakes::FAKE);
                // Object vtable: focus notify (0x98), focus move (0xb0, the
                // real chain reader returning the move result).
                c.bus
                    .write16(ov + 0x98, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(ov + 0x9c, fakes::FAKE + 0x10);
                c.bus.write16(ov + 0xb0, 0);
                c.bus.write32(ov + 0xb4, 0x0039_ff50);
                c.bus.write32(a3 + 0x5c, p);
                c.bus.write32(p + 0xd0, q);
                const MOVES: [u32; 8] = [0, 1, 0x10, 0x11, 0x100, 0x101, 0x110, 0x111];
                c.bus.write32(q + 0x10, MOVES[(r.next_u32() % 8) as usize]);
                // The input's vtable slots: zero or non-zero answers.
                c.bus.write32(inp + 8, iv);
                for (i, off) in [0x30u32, 0x88, 0x90, 0x98, 0xa0].into_iter().enumerate() {
                    c.bus
                        .write16(iv + off, (r.next_u32() as i16 as i32 & !3) as u16);
                    let f = if r.next_u32() % 2 == 0 {
                        zero
                    } else {
                        fakes::FAKE + 0x20 + 0x10 * i as u32
                    };
                    c.bus.write32(iv + off + 4, f);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(inp);
                c.pc = 0x0039_9970;
            },
        );
    }
}
