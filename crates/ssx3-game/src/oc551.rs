//! Staging module oc551 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc551`.

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
    0x0027a9f0,
    script_engine_skip_finish_script,
    "ScriptEngine_SkipFinishScript",
)];

/// `ScriptEngine_SkipFinishScript` (0x0027a9f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn script_engine_skip_finish_script(c: &mut Cpu) {
    if c.pc != 0x0027a9f0 {
        return ee::s07::f_0027a9f0__generated(c);
    }
    guest(c, |c| {
        'b_27aa50: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A1][0] = 0x1;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x0027aa10);
            c.g[S1][0] = c.g[A0][0];
            Callee {
                addr: 0x00279148,
                f: ee::s07::ScriptEngine_ListGetEntryType,
            }
            .call(c, 0x0027aa10)?;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
            c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_27aa50;
            } else {
                'b_27aa4c: {
                    'b_27aa44: {
                        c.cycles += 7;
                        c.g[V0][0] = 0x480000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                        let t = c.g[A0][0] as u32;
                        match t {
                            0x0027aa3c => {
                                c.cycles += 2;
                                let t = true;
                                c.g[S0][0] = 0x3;
                                if t {
                                    break 'b_27aa50;
                                } else {
                                    break 'b_27aa44;
                                }
                            }
                            0x0027aa44 => {
                                break 'b_27aa44;
                            }
                            0x0027aa4c => {
                                break 'b_27aa4c;
                            }
                            0x0027aa50 => {
                                break 'b_27aa50;
                            }
                            _ => {
                                c.jump_out(t, 0x0027a9f0, 0x0027aa7c);
                                return Ok(());
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0x2;
                    if t {
                        break 'b_27aa50;
                    } else {
                        break 'b_27aa4c;
                    }
                }
                c.cycles += 1;
                c.g[S0][0] = 0x1;
                break 'b_27aa50;
            }
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0027aa64);
        c.g[A3][0] = 0x1;
        Callee {
            addr: 0x00279070,
            f: ee::s07::ScriptEngine_ListSkip,
        }
        .call(c, 0x0027aa64)?;
        c.g[V0][0] = (0u64 < c.g[S0][0]) as u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
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
    fn script_engine_skip_finish_script_matches() {
        check(
            ee::s07::f_0027a9f0__generated,
            script_engine_skip_finish_script,
            200,
            &|c, seed| {
                machine(c, seed, 0xa9f0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0027a9f0;
            },
        );
    }
}
