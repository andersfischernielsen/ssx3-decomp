//! Staging module oc548 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc548`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0026ded8,
    world_state_create_object,
    "WorldState_CreateObject",
)];

/// `WorldState_CreateObject` (0x0026ded8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn world_state_create_object(c: &mut Cpu) {
    if c.pc != 0x0026ded8 {
        return ee::s07::f_0026ded8__generated(c);
    }
    guest(c, |c| {
        'b_26e32c: {
            'b_26e328: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S0][0] = c.g[A1][0];
                c.g[RA][0] = sx(0x0026def8);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0026ea68,
                    f: ee::s07::ReplayStream_PeekInt,
                }
                .call(c, 0x0026def8)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0x17u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    break 'b_26e328;
                } else {
                    'b_26e2e0: {
                        'b_26e2b0: {
                            'b_26e280: {
                                'b_26e250: {
                                    'b_26e220: {
                                        'b_26e1f0: {
                                            'b_26e1c0: {
                                                'b_26e190: {
                                                    'b_26e160: {
                                                        'b_26e130: {
                                                            'b_26e100: {
                                                                'b_26e0d0: {
                                                                    'b_26e0a0: {
                                                                        'b_26e070: {
                                                                            'b_26e040: {
                                                                                'b_26e010: {
                                                                                    'b_26dfe0: {
                                                                                        'b_26dfb0: {
                                                                                            'b_26df80: {
                                                                                                'b_26df50: {
                                                                                                    c.cycles += 6;
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16c0));
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                                                    let t = c.g[A0][0] as u32;
                                                                                                    match t {
0x0026df20 => {
c.cycles += 12;
c.g[A1][0] = 0x480000;
c.g[A0][0] = 0x34;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x15f0));
c.g[A2][0] = 0x20000000;
c.g[RA][0] = sx(0x0026df38);
c.g[A3][0] = 0u64;
call_ptr(c, 0x00317d70, 0x0026df38)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0026df48);
c.g[A2][0] = c.g[S1][0];
call_ptr(c, 0x00342d10, 0x0026df48)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_26e32c;
} else {
break 'b_26df50;
}
}
0x0026df50 => {
break 'b_26df50;
}
0x0026df80 => {
break 'b_26df80;
}
0x0026dfb0 => {
break 'b_26dfb0;
}
0x0026dfe0 => {
break 'b_26dfe0;
}
0x0026e010 => {
break 'b_26e010;
}
0x0026e040 => {
break 'b_26e040;
}
0x0026e070 => {
break 'b_26e070;
}
0x0026e0a0 => {
break 'b_26e0a0;
}
0x0026e0d0 => {
break 'b_26e0d0;
}
0x0026e100 => {
break 'b_26e100;
}
0x0026e130 => {
break 'b_26e130;
}
0x0026e160 => {
break 'b_26e160;
}
0x0026e190 => {
break 'b_26e190;
}
0x0026e1c0 => {
break 'b_26e1c0;
}
0x0026e1f0 => {
break 'b_26e1f0;
}
0x0026e220 => {
break 'b_26e220;
}
0x0026e250 => {
break 'b_26e250;
}
0x0026e280 => {
break 'b_26e280;
}
0x0026e2b0 => {
break 'b_26e2b0;
}
0x0026e2e0 => {
break 'b_26e2e0;
}
0x0026e328 => {
break 'b_26e328;
}
_ => { c.jump_out(t, 0x0026ded8, 0x0026e33c); return Ok(()); }
}
                                                                                                }
                                                                                                c.cycles += 12;
                                                                                                c.g[A1][0] = 0x480000;
                                                                                                c.g[A0][0] = 0x6c;
                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1600));
                                                                                                c.g[A2][0] = 0x20000000;
                                                                                                c.g[RA][0] = sx(0x0026df68);
                                                                                                c.g[A3][0] = 0u64;
                                                                                                call_ptr(c, 0x00317d70, 0x0026df68)?;
                                                                                                c.g[A0][0] = c.g[V0][0];
                                                                                                c.g[A1][0] = c.g[S0][0];
                                                                                                c.g[RA][0] = sx(0x0026df78);
                                                                                                c.g[A2][0] = c.g[S1][0];
                                                                                                call_ptr(c, 0x00341c80, 0x0026df78)?;
                                                                                                let t = true;
                                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_26e32c;
                                                                                                } else {
                                                                                                    break 'b_26df80;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 12;
                                                                                            c.g[A1][0] = 0x480000;
                                                                                            c.g[A0][0] = 0x70;
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1610));
                                                                                            c.g[A2][0] = 0x20000000;
                                                                                            c.g[RA][0] = sx(0x0026df98);
                                                                                            c.g[A3][0] = 0u64;
                                                                                            call_ptr(c, 0x00317d70, 0x0026df98)?;
                                                                                            c.g[A0][0] = c.g[V0][0];
                                                                                            c.g[A1][0] = c.g[S0][0];
                                                                                            c.g[RA][0] = sx(0x0026dfa8);
                                                                                            c.g[A2][0] = c.g[S1][0];
                                                                                            call_ptr(c, 0x00359270, 0x0026dfa8)?;
                                                                                            let t =
                                                                                                true;
                                                                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                            if t {
                                                                                                break 'b_26e32c;
                                                                                            } else {
                                                                                                break 'b_26dfb0;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            12;
                                                                                        c.g[A1][0] = 0x480000;
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0x88;
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1620));
                                                                                        c.g[A2][0] = 0x20000000;
                                                                                        c.g[RA][0] = sx(0x0026dfc8);
                                                                                        c.g[A3]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        call_ptr(c, 0x00317d70, 0x0026dfc8)?;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0];
                                                                                        c.g[A1]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        c.g[RA][0] = sx(0x0026dfd8);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        call_ptr(c, 0x00358db0, 0x0026dfd8)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                        if t {
                                                                                            break 'b_26e32c;
                                                                                        } else {
                                                                                            break 'b_26dfe0;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 12;
                                                                                    c.g[A1][0] =
                                                                                        0x480000;
                                                                                    c.g[A0][0] =
                                                                                        0x80;
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1630));
                                                                                    c.g[A2][0] =
                                                                                        0x20000000;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0026dff8,
                                                                                    );
                                                                                    c.g[A3][0] =
                                                                                        0u64;
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00317d70,
                                                                                        0x0026dff8,
                                                                                    )?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A1][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0026e008,
                                                                                    );
                                                                                    c.g[A2][0] =
                                                                                        c.g[S1][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003422e8,
                                                                                        0x0026e008,
                                                                                    )?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                    if t {
                                                                                        break 'b_26e32c;
                                                                                    } else {
                                                                                        break 'b_26e010;
                                                                                    }
                                                                                }
                                                                                c.cycles += 12;
                                                                                c.g[A1][0] =
                                                                                    0x480000;
                                                                                c.g[A0][0] = 0xb0;
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1640));
                                                                                c.g[A2][0] =
                                                                                    0x20000000;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0026e028);
                                                                                c.g[A3][0] = 0u64;
                                                                                call_ptr(
                                                                                    c, 0x00317d70,
                                                                                    0x0026e028,
                                                                                )?;
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[A1][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x0026e038);
                                                                                c.g[A2][0] =
                                                                                    c.g[S1][0];
                                                                                call_ptr(
                                                                                    c, 0x003464e0,
                                                                                    0x0026e038,
                                                                                )?;
                                                                                let t = true;
                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                if t {
                                                                                    break 'b_26e32c;
                                                                                } else {
                                                                                    break 'b_26e040;
                                                                                }
                                                                            }
                                                                            c.cycles += 12;
                                                                            c.g[A1][0] = 0x4a0000;
                                                                            c.g[A0][0] = 0x24;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x3440,
                                                                                    ));
                                                                            c.g[A2][0] = 0x20000000;
                                                                            c.g[RA][0] =
                                                                                sx(0x0026e058);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x0026e058,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            c.g[A1][0] = c.g[S0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x0026e068);
                                                                            c.g[A2][0] = c.g[S1][0];
                                                                            call_ptr(
                                                                                c, 0x003437c0,
                                                                                0x0026e068,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_26e32c;
                                                                            } else {
                                                                                break 'b_26e070;
                                                                            }
                                                                        }
                                                                        c.cycles += 12;
                                                                        c.g[A1][0] = 0x4a0000;
                                                                        c.g[A0][0] = 0x50;
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x3448,
                                                                                ));
                                                                        c.g[A2][0] = 0x20000000;
                                                                        c.g[RA][0] = sx(0x0026e088);
                                                                        c.g[A3][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00317d70,
                                                                            0x0026e088,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        c.g[RA][0] = sx(0x0026e098);
                                                                        c.g[A2][0] = c.g[S1][0];
                                                                        call_ptr(
                                                                            c, 0x00341548,
                                                                            0x0026e098,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        if t {
                                                                            break 'b_26e32c;
                                                                        } else {
                                                                            break 'b_26e0a0;
                                                                        }
                                                                    }
                                                                    c.cycles += 12;
                                                                    c.g[A1][0] = 0x4a0000;
                                                                    c.g[A0][0] = 0x10c;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x3450));
                                                                    c.g[A2][0] = 0x20000000;
                                                                    c.g[RA][0] = sx(0x0026e0b8);
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x0026e0b8,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0026e0c8);
                                                                    c.g[A2][0] = c.g[S1][0];
                                                                    call_ptr(
                                                                        c, 0x0034cbe8, 0x0026e0c8,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    if t {
                                                                        break 'b_26e32c;
                                                                    } else {
                                                                        break 'b_26e0d0;
                                                                    }
                                                                }
                                                                c.cycles += 12;
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[A0][0] = 0x1c;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x3458));
                                                                c.g[A2][0] = 0x20000000;
                                                                c.g[RA][0] = sx(0x0026e0e8);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x0026e0e8,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x0026e0f8);
                                                                c.g[A2][0] = c.g[S1][0];
                                                                call_ptr(
                                                                    c, 0x0034ae68, 0x0026e0f8,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_26e32c;
                                                                } else {
                                                                    break 'b_26e100;
                                                                }
                                                            }
                                                            c.cycles += 12;
                                                            c.g[A1][0] = 0x480000;
                                                            c.g[A0][0] = 0x6c0;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1650));
                                                            c.g[A2][0] = 0x20000000;
                                                            c.g[RA][0] = sx(0x0026e118);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x0026e118)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x0026e128);
                                                            c.g[A2][0] = c.g[S1][0];
                                                            call_ptr(c, 0x00352168, 0x0026e128)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_26e32c;
                                                            } else {
                                                                break 'b_26e130;
                                                            }
                                                        }
                                                        c.cycles += 12;
                                                        c.g[A1][0] = 0x480000;
                                                        c.g[A0][0] = 0x2c;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1660));
                                                        c.g[A2][0] = 0x20000000;
                                                        c.g[RA][0] = sx(0x0026e148);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x0026e148)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0026e158);
                                                        c.g[A2][0] = c.g[S1][0];
                                                        call_ptr(c, 0x00357918, 0x0026e158)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_26e32c;
                                                        } else {
                                                            break 'b_26e160;
                                                        }
                                                    }
                                                    c.cycles += 12;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = 0x218;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3460));
                                                    c.g[A2][0] = 0x20000000;
                                                    c.g[RA][0] = sx(0x0026e178);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x0026e178)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x0026e188);
                                                    c.g[A2][0] = c.g[S1][0];
                                                    call_ptr(c, 0x00358588, 0x0026e188)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_26e32c;
                                                    } else {
                                                        break 'b_26e190;
                                                    }
                                                }
                                                c.cycles += 12;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = 0x1c;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x3468));
                                                c.g[A2][0] = 0x20000000;
                                                c.g[RA][0] = sx(0x0026e1a8);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x0026e1a8)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x0026e1b8);
                                                c.g[A2][0] = c.g[S1][0];
                                                call_ptr(c, 0x00350fd8, 0x0026e1b8)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_26e32c;
                                                } else {
                                                    break 'b_26e1c0;
                                                }
                                            }
                                            c.cycles += 12;
                                            c.g[A1][0] = 0x480000;
                                            c.g[A0][0] = 0x7c;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1670));
                                            c.g[A2][0] = 0x20000000;
                                            c.g[RA][0] = sx(0x0026e1d8);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x0026e1d8)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0026e1e8);
                                            c.g[A2][0] = c.g[S1][0];
                                            call_ptr(c, 0x0034ed88, 0x0026e1e8)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_26e32c;
                                            } else {
                                                break 'b_26e1f0;
                                            }
                                        }
                                        c.cycles += 12;
                                        c.g[A1][0] = 0x480000;
                                        c.g[A0][0] = 0x48;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1680));
                                        c.g[A2][0] = 0x20000000;
                                        c.g[RA][0] = sx(0x0026e208);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x0026e208)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0026e218);
                                        c.g[A2][0] = c.g[S1][0];
                                        call_ptr(c, 0x003430d0, 0x0026e218)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_26e32c;
                                        } else {
                                            break 'b_26e220;
                                        }
                                    }
                                    c.cycles += 12;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = 0x1c;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3470));
                                    c.g[A2][0] = 0x20000000;
                                    c.g[RA][0] = sx(0x0026e238);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x0026e238)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0026e248);
                                    c.g[A2][0] = c.g[S1][0];
                                    call_ptr(c, 0x00350f08, 0x0026e248)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_26e32c;
                                    } else {
                                        break 'b_26e250;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A1][0] = 0x4a0000;
                                c.g[A0][0] = 0x2c;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3478));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x0026e268);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x0026e268)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0026e278);
                                c.g[A2][0] = c.g[S1][0];
                                call_ptr(c, 0x00356e60, 0x0026e278)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_26e32c;
                                } else {
                                    break 'b_26e280;
                                }
                            }
                            c.cycles += 12;
                            c.g[A1][0] = 0x480000;
                            c.g[A0][0] = 0x80;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1690));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x0026e298);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0026e298)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0026e2a8);
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00348fa0, 0x0026e2a8)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_26e32c;
                            } else {
                                break 'b_26e2b0;
                            }
                        }
                        c.cycles += 12;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x80;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16a0));
                        c.g[A2][0] = 0x20000000;
                        c.g[RA][0] = sx(0x0026e2c8);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x0026e2c8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0026e2d8);
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x00349db0, 0x0026e2d8)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_26e32c;
                        } else {
                            break 'b_26e2e0;
                        }
                    }
                    c.cycles += 13;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x3c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16b0));
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x0026e2f8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x0026e2f8)?;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0026e308);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00350b98, 0x0026e308)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        break 'b_26e328;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0026e328);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0026e328)?;
                        break 'b_26e328;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_26e32c;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn world_state_create_object_matches() {
        check(
            ee::s07::f_0026ded8__generated,
            world_state_create_object,
            200,
            &|c, seed| {
                machine(c, seed, 0xded8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0026ded8;
            },
        );
    }
}
