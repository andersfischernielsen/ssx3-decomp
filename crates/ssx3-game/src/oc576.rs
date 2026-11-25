//! Staging module oc576 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc576`.

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
    0x003078c0,
    wscript_process_dispatch_event,
    "WScriptProcess_DispatchEvent",
)];

/// `WScriptProcess_DispatchEvent` (0x003078c0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn wscript_process_dispatch_event(c: &mut Cpu) {
    if c.pc != 0x003078c0 {
        return ee::s10::f_003078c0__generated(c);
    }
    guest(c, |c| {
        'b_3079c8: {
            'b_3079c4: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lhu((c.g[A1][0] as u32));
                c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_3079c4;
                } else {
                    'b_3079b0: {
                        'b_3079a0: {
                            'b_307994: {
                                'b_307984: {
                                    'b_307974: {
                                        'b_307964: {
                                            'b_307954: {
                                                'b_307928: {
                                                    'b_307918: {
                                                        'b_307908: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = 0x490000;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x67a0));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x003078f8 => {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x5c));
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x38));
                                                                    let t = true;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x3c));
                                                                    if t {
                                                                        break 'b_3079a0;
                                                                    } else {
                                                                        break 'b_307908;
                                                                    }
                                                                }
                                                                0x00307908 => {
                                                                    break 'b_307908;
                                                                }
                                                                0x00307918 => {
                                                                    break 'b_307918;
                                                                }
                                                                0x00307928 => {
                                                                    break 'b_307928;
                                                                }
                                                                0x00307954 => {
                                                                    break 'b_307954;
                                                                }
                                                                0x00307964 => {
                                                                    break 'b_307964;
                                                                }
                                                                0x00307974 => {
                                                                    break 'b_307974;
                                                                }
                                                                0x00307984 => {
                                                                    break 'b_307984;
                                                                }
                                                                0x00307994 => {
                                                                    break 'b_307994;
                                                                }
                                                                0x003079b0 => {
                                                                    break 'b_3079b0;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x003078c0, 0x003079d4,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x5c));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                        let t = true;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                        if t {
                                                            break 'b_3079a0;
                                                        } else {
                                                            break 'b_307918;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                    if t {
                                                        break 'b_3079a0;
                                                    } else {
                                                        break 'b_307928;
                                                    }
                                                }
                                                c.cycles += 11;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                c.sw(
                                                    (c.g[A0][0] as u32).wrapping_add(0x29c),
                                                    c.g[V0][0],
                                                );
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                                                c.g[A1][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                                                let t = c.g[A1][0] as u32;
                                                c.g[RA][0] = sx(0x00307948);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00307948)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                let t = true;
                                                c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), 0u64);
                                                if t {
                                                    break 'b_3079c4;
                                                } else {
                                                    break 'b_307954;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            let t = true;
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            if t {
                                                break 'b_3079a0;
                                            } else {
                                                break 'b_307964;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                                        if t {
                                            break 'b_3079a0;
                                        } else {
                                            break 'b_307974;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                                    if t {
                                        break 'b_3079a0;
                                    } else {
                                        break 'b_307984;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                if t {
                                    break 'b_3079a0;
                                } else {
                                    break 'b_307994;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x78));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                            break 'b_3079a0;
                        }
                        c.cycles += 4;
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x003079a8);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x003079a8)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_3079c8;
                        } else {
                            break 'b_3079b0;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x003079c4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x003079c4)?;
                    break 'b_3079c4;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_3079c8;
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
    fn wscript_process_dispatch_event_matches() {
        check(
            ee::s10::f_003078c0__generated,
            wscript_process_dispatch_event,
            200,
            &|c, seed| {
                machine(c, seed, 0x78c0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x003078c0;
            },
        );
    }
}
