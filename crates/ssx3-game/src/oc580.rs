//! Staging module oc580 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc580`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x00321638, input_parser_free_node, "InputParser_FreeNode")];

/// `InputParser_FreeNode` (0x00321638): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn input_parser_free_node(c: &mut Cpu) {
    if c.pc != 0x00321638 {
        return ee::s10::f_00321638__generated(c);
    }
    guest(c, |c| {
        'b_3216d4: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            let t = c.g[S0][0] == 0u64;
            c.g[S1][0] = c.g[A0][0];
            if t {
                break 'b_3216d4;
            } else {
                'b_3216cc: {
                    'b_3216c8: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                        c.g[V0][0] = (c.g[V1][0] < 0x22u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x490000;
                        if t {
                            break 'b_3216c8;
                        } else {
                            'b_3216a4: {
                                'b_321694: {
                                    'b_321684: {
                                        c.cycles += 6;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22b0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x0032167c => {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_321694;
                                                } else {
                                                    break 'b_321684;
                                                }
                                            }
                                            0x00321684 => {
                                                break 'b_321684;
                                            }
                                            0x003216a4 => {
                                                break 'b_3216a4;
                                            }
                                            0x003216c8 => {
                                                break 'b_3216c8;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00321638, 0x003216e8);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x00321690);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00321638,
                                        f: ee::s10::InputParser_FreeNode,
                                    }
                                    .call(c, 0x00321690)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    break 'b_321694;
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0032169c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00321638,
                                    f: ee::s10::InputParser_FreeNode,
                                }
                                .call(c, 0x0032169c)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                                if t {
                                    break 'b_3216cc;
                                } else {
                                    break 'b_3216a4;
                                }
                            }
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[RA][0] = sx(0x003216b0);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00321638,
                                f: ee::s10::InputParser_FreeNode,
                            }
                            .call(c, 0x003216b0)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[RA][0] = sx(0x003216bc);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00321638,
                                f: ee::s10::InputParser_FreeNode,
                            }
                            .call(c, 0x003216bc)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[RA][0] = sx(0x003216c8);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00321638,
                                f: ee::s10::InputParser_FreeNode,
                            }
                            .call(c, 0x003216c8)?;
                            break 'b_3216c8;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                    break 'b_3216cc;
                }
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe0), c.g[S0][0]);
                break 'b_3216d4;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
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
    fn input_parser_free_node_matches() {
        check(
            ee::s10::f_00321638__generated,
            input_parser_free_node,
            200,
            &|c, seed| {
                machine(c, seed, 0x1638, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00321638;
            },
        );
    }
}
