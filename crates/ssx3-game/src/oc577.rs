//! Staging module oc577 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc577`.

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
    0x003083a0,
    wscript_task_handle_event,
    "WScriptTask_HandleEvent",
)];

/// `WScriptTask_HandleEvent` (0x003083a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn wscript_task_handle_event(c: &mut Cpu) {
    if c.pc != 0x003083a0 {
        return ee::s10::f_003083a0__generated(c);
    }
    guest(c, |c| 'l_3083a0: loop {
        'b_3084c0: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V0][0] = 0x2;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
            if c.g[V1][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_3084c0;
            } else {
                'b_3084bc: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lhu((c.g[A1][0] as u32));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        break 'b_3084bc;
                    } else {
                        'b_3084a8: {
                            'b_308498: {
                                'b_30848c: {
                                    'b_30847c: {
                                        'b_30846c: {
                                            'b_30845c: {
                                                'b_30844c: {
                                                    'b_308418: {
                                                        'b_308408: {
                                                            'b_3083f8: {
                                                                c.cycles += 6;
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x6760));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x003083a0 => {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x003083a0,
                                                                            );
                                                                        }
                                                                        continue 'l_3083a0;
                                                                    }
                                                                    0x003083e8 => {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x40));
                                                                        c.g[A0][0] = c.lh((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        let t = true;
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x3c));
                                                                        if t {
                                                                            break 'b_308498;
                                                                        } else {
                                                                            break 'b_3083f8;
                                                                        }
                                                                    }
                                                                    0x003083f8 => {
                                                                        break 'b_3083f8;
                                                                    }
                                                                    0x00308408 => {
                                                                        break 'b_308408;
                                                                    }
                                                                    0x00308418 => {
                                                                        break 'b_308418;
                                                                    }
                                                                    0x0030844c => {
                                                                        break 'b_30844c;
                                                                    }
                                                                    0x0030845c => {
                                                                        break 'b_30845c;
                                                                    }
                                                                    0x0030846c => {
                                                                        break 'b_30846c;
                                                                    }
                                                                    0x0030847c => {
                                                                        break 'b_30847c;
                                                                    }
                                                                    0x0030848c => {
                                                                        break 'b_30848c;
                                                                    }
                                                                    0x003084a8 => {
                                                                        break 'b_3084a8;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x003083a0,
                                                                            0x003084cc,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            if t {
                                                                break 'b_308498;
                                                            } else {
                                                                break 'b_308408;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                        let t = true;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        if t {
                                                            break 'b_308498;
                                                        } else {
                                                            break 'b_308418;
                                                        }
                                                    }
                                                    c.cycles += 13;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                                                    c.g[A0][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x29c),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x50));
                                                    c.g[A1][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x54));
                                                    let t = c.g[A1][0] as u32;
                                                    c.g[RA][0] = sx(0x0030843c);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x0030843c)?;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x29c),
                                                        0u64,
                                                    );
                                                    if t {
                                                        break 'b_3084bc;
                                                    } else {
                                                        break 'b_30844c;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                let t = true;
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                if t {
                                                    break 'b_308498;
                                                } else {
                                                    break 'b_30845c;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
                                            let t = true;
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                                            if t {
                                                break 'b_308498;
                                            } else {
                                                break 'b_30846c;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                                        if t {
                                            break 'b_308498;
                                        } else {
                                            break 'b_30847c;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                    if t {
                                        break 'b_308498;
                                    } else {
                                        break 'b_30848c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x78));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                break 'b_308498;
                            }
                            c.cycles += 4;
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x003084a0);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x003084a0)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_3084c0;
                            } else {
                                break 'b_3084a8;
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x003084bc);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x003084bc)?;
                        break 'b_3084bc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_3084c0;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
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
    fn wscript_task_handle_event_matches() {
        check(
            ee::s10::f_003083a0__generated,
            wscript_task_handle_event,
            200,
            &|c, seed| {
                machine(c, seed, 0x83a0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x003083a0;
            },
        );
    }
}
