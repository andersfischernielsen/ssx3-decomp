//! Staging module oc758 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc758`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002d_e368,
        c_board_wake_fx_reset_head,
        "cBoardWakeFX_ResetHead",
    ),
    (0x002e_f3b0, c_air_streamer_fx_ctor, "cAirStreamerFX_ctor"),
    (
        0x002e_aae0,
        rider_base_ctor_power_up_trail,
        "RiderBase_CtorPowerUpTrail",
    ),
    (0x002d_48f0, c_anger_fx_ctor, "cAngerFX_ctor"),
    (
        0x0020_0a70,
        event_select_transition_in,
        "EventSelect_TransitionIn",
    ),
];

/// `void cBoardWakeFX_ResetHead(sWake* self)` (0x2de368): clears the two head
/// words (+0x30, +0x34) and stores the same 16-byte default vector (read from
/// 0x4ff120 three times) into +0x00, +0x20 and +0x10.
pub fn c_board_wake_fx_reset_head(c: &mut Cpu) {
    c.cycles += 11;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x34), 0u64);
    c.g[A1][0] = 0x500000;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x30), 0u64);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0));
    c.lq(2, c.g[A1][0] as u32);
    c.sq(2, c.g[A0][0] as u32);
    c.lq(3, c.g[A1][0] as u32);
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lq(2, c.g[A1][0] as u32);
    let t = c.g[RA][0] as u32;
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `cAirStreamerFX* cAirStreamerFX_ctor(cAirStreamerFX* self, int is_derived)`
/// (0x2ef3b0): unless it is a base-class sub-construction (`a1 == 0`), runs
/// the rider base constructor (0x11b698) on `self+0x20`; then installs the
/// FX vtable and render hooks into the owner's slots (+0x6e8 / +0x6d0 / +0x6c0
/// of `*self`), copies two small descriptor blocks onto the stack (the
/// `a1 == 0` path only), allocates the 0x320-byte buffer through
/// `operator_new_named` into `self+4`, resets the FX and returns `self`.
pub fn c_air_streamer_fx_ctor(c: &mut Cpu) {
    'b_2ef3e0: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S1][0] = c.g[A1][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
        if t {
            break 'b_2ef3e0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002ef3e0);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.pc = 0x0011b698;
            call_indirect(c, 0x0011b698);
            if c.pc != 0x002ef3e0 {
                return;
            }
            break 'b_2ef3e0;
        }
    }
    'b_2ef4e8: {
        c.cycles += 17;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7730));
        c.g[A3][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
        c.g[V0][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
        c.g[V1][0] = 0x490000;
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        c.g[T3][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x76e8));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[S1][0] != 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
        if t {
            break 'b_2ef4e8;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
            c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            'l_2ef430: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002ef430);
                    }
                    continue 'l_2ef430;
                } else {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7730));
                    c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                    c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x76e8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = c.g[T1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    'l_2ef494: loop {
                        c.cycles += 11;
                        c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                        c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                        c.g[T4][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[T2][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002ef494);
                            }
                            continue 'l_2ef494;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x76e8));
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                            c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                            break 'b_2ef4e8;
                        }
                    }
                }
            }
        }
    }
    c.cycles += 18;
    c.g[A0][0] = 0x320;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7b78));
    c.g[A2][0] = 0u64;
    c.g[RA][0] = sx(0x002ef4fc);
    c.g[A3][0] = 0u64;
    c.pc = 0x00317e30;
    call_indirect(c, 0x00317e30);
    if c.pc != 0x002ef4fc {
        return;
    }
    c.g[A0][0] = 0xffffffffffffffff;
    c.g[V1][0] = 0x18;
    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
    c.g[RA][0] = sx(0x002ef514);
    c.g[A0][0] = c.g[S0][0];
    c.pc = 0x002ef6a0;
    ee::s09::f_002ef6a0(c);
    if c.pc != 0x002ef514 {
        return;
    }
    c.g[V0][0] = c.g[S0][0];
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
    c.pc = t;
    return;
}

/// `RiderBase* RiderBase_CtorPowerUpTrail(RiderBase* self, int is_derived)`
/// (0x2eaae0): the same shape as the other FX constructors without the
/// buffer allocation: base constructor on `self+0x110` (unless `a1 == 0`
/// skips it), vtable/hook installs, the descriptor block copies, an empty
/// three-step array-construction loop, then `RiderBase_ResetPowerUpTrail`.
pub fn rider_base_ctor_power_up_trail(c: &mut Cpu) {
    'b_2eab0c: {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
        if t {
            break 'b_2eab0c;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x110));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002eab0c);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.pc = 0x0011b698;
            call_indirect(c, 0x0011b698);
            if c.pc != 0x002eab0c {
                return;
            }
            break 'b_2eab0c;
        }
    }
    'b_2eac10: {
        c.cycles += 15;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ba8));
        c.g[A0][0] = 0x460000;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
        c.g[V1][0] = 0x490000;
        c.g[A3][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
        c.g[T2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7b60));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[S1][0] != 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
        if t {
            break 'b_2eac10;
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[A2][0];
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            'l_2eab58: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[A1][0] as u32);
                c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002eab58);
                    }
                    continue 'l_2eab58;
                } else {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ba8));
                    c.g[A2][0] = c.ld(c.g[A1][0] as u32);
                    c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7b60));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[T0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x110));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    'l_2eabbc: loop {
                        c.cycles += 11;
                        c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                        c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                        c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                        c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[T1][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002eabbc);
                            }
                            continue 'l_2eabbc;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7b60));
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                            c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T0][0]);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                            break 'b_2eac10;
                        }
                    }
                }
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = 0x2;
    c.g[A0][0] = 0xffffffffffffffff;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    'l_2eac20: loop {
        c.cycles += 8;
        c.g[V0][0] = 0x3;
        c.g[V0][0] = c.g[V1][0];
        let t = c.g[V0][0] != c.g[A0][0];
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x002eac20);
            }
            continue 'l_2eac20;
        } else {
            c.cycles += 8;
            c.g[RA][0] = sx(0x002eac48);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x002eadc0;
            ee::s09::f_002eadc0(c);
            if c.pc != 0x002eac48 {
                return;
            }
            c.g[V0][0] = c.g[S0][0];
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
            c.pc = t;
            return;
        }
    }
}

/// `cAngerFX* cAngerFX_ctor(cAngerFX* self, int is_derived)` (0x2d48f0): the
/// air-streamer constructor's twin: base constructor on `self+0x10`, hook
/// installs, descriptor copies, a 0x58-byte `operator_new_named` buffer at
/// `self+4`, `cAngerFX_Reset`.
pub fn c_anger_fx_ctor(c: &mut Cpu) {
    'b_2d4920: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S1][0] = c.g[A1][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
        if t {
            break 'b_2d4920;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002d4920);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.pc = 0x0011b698;
            call_indirect(c, 0x0011b698);
            if c.pc != 0x002d4920 {
                return;
            }
            break 'b_2d4920;
        }
    }
    'b_2d4a28: {
        c.cycles += 17;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7540));
        c.g[A3][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
        c.g[V0][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
        c.g[V1][0] = 0x490000;
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        c.g[T3][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x74f8));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[S1][0] != 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
        if t {
            break 'b_2d4a28;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
            c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            'l_2d4970: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002d4970);
                    }
                    continue 'l_2d4970;
                } else {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7540));
                    c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                    c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x74f8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = c.g[T1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    'l_2d49d4: loop {
                        c.cycles += 11;
                        c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                        c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                        c.g[T4][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[T2][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002d49d4);
                            }
                            continue 'l_2d49d4;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x74f8));
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                            c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                            break 'b_2d4a28;
                        }
                    }
                }
            }
        }
    }
    c.cycles += 18;
    c.g[A0][0] = 0x58;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7518));
    c.g[A2][0] = 0u64;
    c.g[RA][0] = sx(0x002d4a3c);
    c.g[A3][0] = 0u64;
    c.pc = 0x00317e30;
    call_indirect(c, 0x00317e30);
    if c.pc != 0x002d4a3c {
        return;
    }
    c.g[A0][0] = 0xffffffffffffffff;
    c.g[V1][0] = 0x1;
    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
    c.g[RA][0] = sx(0x002d4a54);
    c.g[A0][0] = c.g[S0][0];
    c.pc = 0x002d4be0;
    ee::s09::f_002d4be0(c);
    if c.pc != 0x002d4a54 {
        return;
    }
    c.g[V0][0] = c.g[S0][0];
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
    c.pc = t;
    return;
}

/// `int EventSelect_TransitionIn(EventSelect* self, int enter)` (0x200a70):
/// when `enter != 0` sets the screen mode from the global at `gp-0xb54`,
/// builds the peak texts and the event list and sets the map tab mode from
/// `*self`; returns 0.
pub fn event_select_transition_in(c: &mut Cpu) {
    'b_200aa8: {
        c.cycles += 5;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let t = c.g[A1][0] == 0u64;
        c.g[S0][0] = c.g[A0][0];
        if t {
            break 'b_200aa8;
        } else {
            c.cycles += 9;
            c.g[RA][0] = sx(0x00200a8c);
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
            c.pc = 0x002030a0;
            ee::s05::f_002030a0(c);
            if c.pc != 0x00200a8c {
                return;
            }
            c.g[RA][0] = sx(0x00200a94);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x00200de8;
            ee::s05::f_00200de8(c);
            if c.pc != 0x00200a94 {
                return;
            }
            c.g[RA][0] = sx(0x00200a9c);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x00207430;
            ee::s05::f_00207430(c);
            if c.pc != 0x00200a9c {
                return;
            }
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00200aa8);
            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
            c.pc = 0x00203cb8;
            ee::s05::f_00203cb8(c);
            if c.pc != 0x00200aa8 {
                return;
            }
            break 'b_200aa8;
        }
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = 0u64;
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) const OC758_CTRL: u32 = crate::difftest::DATA + 0x20_0000;

#[cfg(test)]
pub(crate) const OC758_MAGIC: u32 = 0x0758_c0de;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gated fakes in the override slots of the callees of this module's
    /// functions: each logs its call like `difftest::fakes::fake`; with the
    /// gate (`OC758_MAGIC` at `OC758_CTRL` in the machine's RAM) off, the slot
    /// runs the body it held, so other tests in the binary are unaffected.
    macro_rules! oc758_gated {
        ($($fake:ident => $addr:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC758_CTRL) != OC758_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                }
            )*

            pub(crate) fn oc758_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc758_gated! {
        oc758_fake_0011b698 => 0x0011_b698, ee::s00::OVERRIDE_0011B698, ee::s00::f_0011b698__generated;
        oc758_fake_00317e30 => 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc758_fake_002ef6a0 => 0x002e_f6a0, ee::s09::OVERRIDE_002EF6A0, ee::s09::f_002ef6a0__generated;
        oc758_fake_002eadc0 => 0x002e_adc0, ee::s09::OVERRIDE_002EADC0, ee::s09::f_002eadc0__generated;
        oc758_fake_002d4be0 => 0x002d_4be0, ee::s09::OVERRIDE_002D4BE0, ee::s09::f_002d4be0__generated;
        oc758_fake_002030a0 => 0x0020_30a0, ee::s05::OVERRIDE_002030A0, ee::s05::f_002030a0__generated;
        oc758_fake_00203cb8 => 0x0020_3cb8, ee::s05::OVERRIDE_00203CB8, ee::s05::f_00203cb8__generated;
        oc758_fake_00208438 => 0x0020_8438, ee::s05::OVERRIDE_00208438, ee::s05::f_00208438__generated;
        oc758_fake_0014dc80 => 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc758_fake_0014dd58 => 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc758_fake_00317670 => 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc758_fake_0039d860 => 0x0039_d860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated;
    }

    /// `cBoardWakeFX_ResetHead`: random default vectors at 0x4ff120, random
    /// object contents; leaf, so a plain comparison.
    #[test]
    fn c_board_wake_fx_reset_head_matches() {
        check(
            ee::s09::f_002de368__generated,
            c_board_wake_fx_reset_head,
            40,
            &|c, seed| {
                let mut r = Rng(0x2de3_6800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002d_e368;
                fill(c, &mut r, 0x004f_f120, 12);
                let this = DATA + 0x100 * (seed % 8);
                fill(c, &mut r, this, 16);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// Shared fixture of the three FX constructors: `self` with a word at
    /// +0 pointing at an owner block (its slots +0x6c0/+0x6d0/+0x6e8 get
    /// written), `a1 == 0` (the copy loops run, no base constructor) or
    /// non-zero (base constructor call, no copy loops), a fake unwinding at
    /// one of the (up to three) calls.
    fn oc758_ctor_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 3);
        oc758_install_fakes();
        c.bus.write32(OC758_CTRL, OC758_MAGIC);
        c.pc = entry;
        let this = DATA + 0x1000;
        c.bus.write32(this, DATA + 0x4000);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = if seed % 2 == 0 {
            0
        } else {
            sx(r.next_u32() | 1)
        };
    }

    /// `cAirStreamerFX_ctor` with both `a1` modes and unwinds at each call.
    #[test]
    fn c_air_streamer_fx_ctor_matches() {
        covered(
            ee::s09::f_002ef3b0__generated,
            c_air_streamer_fx_ctor,
            120,
            &|c, seed| oc758_ctor_setup(c, seed, 0x2ef3b0, 0x002e_f3b0),
        );
    }

    /// `RiderBase_CtorPowerUpTrail` with both `a1` modes and unwinds.
    #[test]
    fn rider_base_ctor_power_up_trail_matches() {
        covered(
            ee::s09::f_002eaae0__generated,
            rider_base_ctor_power_up_trail,
            120,
            &|c, seed| oc758_ctor_setup(c, seed, 0x2eaae0, 0x002e_aae0),
        );
    }

    /// `cAngerFX_ctor` with both `a1` modes and unwinds.
    #[test]
    fn c_anger_fx_ctor_matches() {
        covered(
            ee::s09::f_002d48f0__generated,
            c_anger_fx_ctor,
            120,
            &|c, seed| oc758_ctor_setup(c, seed, 0x2d48f0, 0x002d_48f0),
        );
    }

    /// `EventSelect_TransitionIn`: `enter == 0` (nothing happens), and
    /// `enter != 0` with the fake unwinding at the mode call (0) or at the
    /// first call inside the real `EventSelect_SetupPeakTexts` (1). The
    /// full path (the real peak-text and event-list builders over fake
    /// pointers) is the next test.
    #[test]
    fn event_select_transition_in_matches() {
        covered(
            ee::s05::f_00200a70__generated,
            event_select_transition_in,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x200a70, 2);
                oc758_install_fakes();
                c.bus.write32(OC758_CTRL, OC758_MAGIC);
                c.pc = 0x0020_0a70;
                let this = DATA + 0x1000;
                c.bus.write32(this, r.next_u32());
                c.bus.write32(GAME_GP - 0xb54, r.next_u32());
                c.g[A0][0] = sx(this);
                if seed % 3 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    c.g[A1][0] = sx(r.next_u32() | 1);
                    c.bus
                        .write32(fakes::LOG_UNWIND_AT, if seed % 3 == 1 { 0 } else { 1 });
                }
            },
        );
    }

    /// `EventSelect_TransitionIn`, `enter != 0`, no unwind: every callee runs
    /// (the real `SetupPeakTexts` / `RefreshEventList` over zeroed RAM and
    /// fake results). Dropped from the `OVERRIDES`-gate if this runs away.
    #[test]
    fn event_select_transition_in_full_matches() {
        check(
            ee::s05::f_00200a70__generated,
            event_select_transition_in,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x200a71, 0);
                oc758_install_fakes();
                c.bus.write32(OC758_CTRL, OC758_MAGIC);
                c.pc = 0x0020_0a70;
                let this = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0xb54, r.next_u32() % 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() | 1);
            },
        );
    }
}
