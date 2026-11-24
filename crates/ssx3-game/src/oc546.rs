//! Staging module oc546 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc546`.

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
    &[(0x0026fe50, replay_process_command, "Replay_ProcessCommand")];

/// `Replay_ProcessCommand` (0x0026fe50): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_process_command(c: &mut Cpu) {
    if c.pc != 0x0026fe50 {
        return ee::s07::f_0026fe50__generated(c);
    }
    guest(c, |c| {
        'b_270020: {
            'b_27001c: {
                'b_26fffc: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x618));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = (c.g[A0][0] < 0x8u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_26fffc;
                    } else {
                        'b_26ffec: {
                            'b_26ffe8: {
                                'b_26ffc8: {
                                    'b_26ffa8: {
                                        'b_26ff98: {
                                            'b_26ff70: {
                                                'b_26ff5c: {
                                                    'b_26fef0: {
                                                        'b_26fec4: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = 0x480000;
                                                            c.g[V1][0] =
                                                                sx((c.g[A0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1790));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x0026fe94 => {
                                                                    c.cycles += 10;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0026fea0);
                                                                    c.g[A1][0] = 0u64;
                                                                    Callee { addr: 0x002702f8, f: ee::s07::Replay_SeekToSnapshot }.call(c, 0x0026fea0)?;
                                                                    c.g[S1][0] = 0x1;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0026feb0);
                                                                    c.g[A1][0] = 0x1;
                                                                    Callee { addr: 0x002703f0, f: ee::s07::Replay_StepFrame }.call(c, 0x0026feb0)?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x484));
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    if t {
                                                                        break 'b_26ff5c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;

                                                                        if t {
                                                                            break 'b_26ff98;
                                                                        } else {
                                                                            break 'b_26fec4;
                                                                        }
                                                                    }
                                                                }
                                                                0x0026fec4 => {
                                                                    break 'b_26fec4;
                                                                }
                                                                0x0026fef0 => {
                                                                    break 'b_26fef0;
                                                                }
                                                                0x0026ff70 => {
                                                                    break 'b_26ff70;
                                                                }
                                                                0x0026ffa8 => {
                                                                    break 'b_26ffa8;
                                                                }
                                                                0x0026ffc8 => {
                                                                    break 'b_26ffc8;
                                                                }
                                                                0x0026ffe8 => {
                                                                    break 'b_26ffe8;
                                                                }
                                                                0x0027001c => {
                                                                    break 'b_27001c;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x0026fe50, 0x00270070,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 9;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0026fed0);
                                                        c.g[S1][0] = 0x1;
                                                        Callee {
                                                            addr: 0x00270378,
                                                            f: ee::s07::Replay_SeekToEnd,
                                                        }
                                                        .call(c, 0x0026fed0)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0026fedc);
                                                        c.g[A1][0] = 0x1;
                                                        Callee {
                                                            addr: 0x002703f0,
                                                            f: ee::s07::Replay_StepFrame,
                                                        }
                                                        .call(c, 0x0026fedc)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x484));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        if t {
                                                            break 'b_26ff5c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_26ff98;
                                                            } else {
                                                                break 'b_26fef0;
                                                            }
                                                        }
                                                    }
                                                    'b_26ff48: {
                                                        'b_26ff28: {
                                                            c.cycles += 10;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x484));
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x0026ff00);
                                                            c.g[S1][0] = 0x1;
                                                            Callee {
                                                                addr: 0x0026e448,
                                                                f: ee::s07::Replay_FindSnapshot,
                                                            }
                                                            .call(c, 0x0026ff00)?;
                                                            c.g[V1][0] = c.g[V0][0];
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x484));
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x488),
                                                                c.g[V1][0],
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x30));
                                                            if c.g[A0][0] != c.g[V0][0] {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_26ff28;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0026ff20);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x0026e568, f: ee::s07::Replay_SeekPrevSnapshot }.call(c, 0x0026ff20)?;
                                                                let t = true;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                if t {
                                                                    break 'b_26ff48;
                                                                } else {
                                                                    break 'b_26ff28;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[A0][0] = c.g[S0][0];
                                                            break 'b_26ff48;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x30));
                                                            let t = c.g[V0][0] != c.g[A0][0];
                                                            c.g[A0][0] = c.g[S0][0];
                                                            if t {
                                                                break 'b_26ff48;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x0026ff44);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x0026e568, f: ee::s07::Replay_SeekPrevSnapshot }.call(c, 0x0026ff44)?;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                break 'b_26ff48;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[RA][0] = sx(0x0026ff50);
                                                    c.g[A1][0] = 0x1;
                                                    Callee {
                                                        addr: 0x002703f0,
                                                        f: ee::s07::Replay_StepFrame,
                                                    }
                                                    .call(c, 0x0026ff50)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x484)
                                                        );
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_26ff98;
                                                    } else {
                                                        break 'b_26ff5c;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x0026ff68);
                                                c.g[A1][0] = 0x6;
                                                Callee {
                                                    addr: 0x0026f4a0,
                                                    f: ee::s07::Replay_SetState,
                                                }
                                                .call(c, 0x0026ff68)?;
                                                let t = true;
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                                if t {
                                                    break 'b_270020;
                                                } else {
                                                    break 'b_26ff70;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x488));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[S1][0] = 0x1;
                                            if t {
                                                break 'b_27001c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x0026ff88);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x0026e528,
                                                    f: ee::s07::Replay_SeekNextSnapshot,
                                                }
                                                .call(c, 0x0026ff88)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x0026ff94);
                                                c.g[A1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002703f0,
                                                    f: ee::s07::Replay_StepFrame,
                                                }
                                                .call(c, 0x0026ff94)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                break 'b_26ff98;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0026ffa0);
                                        c.g[A1][0] = 0x5;
                                        Callee {
                                            addr: 0x0026f4a0,
                                            f: ee::s07::Replay_SetState,
                                        }
                                        .call(c, 0x0026ffa0)?;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                        if t {
                                            break 'b_270020;
                                        } else {
                                            break 'b_26ffa8;
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0026ffb4);
                                    c.g[A1][0] = 0u64;
                                    Callee {
                                        addr: 0x002703f0,
                                        f: ee::s07::Replay_StepFrame,
                                    }
                                    .call(c, 0x0026ffb4)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0026ffc0);
                                    c.g[A1][0] = 0x4;
                                    Callee {
                                        addr: 0x0026f4a0,
                                        f: ee::s07::Replay_SetState,
                                    }
                                    .call(c, 0x0026ffc0)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                    if t {
                                        break 'b_270020;
                                    } else {
                                        break 'b_26ffc8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_26ffec;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0026ffe0);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x0026f4a0,
                                        f: ee::s07::Replay_SetState,
                                    }
                                    .call(c, 0x0026ffe0)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                    if t {
                                        break 'b_270020;
                                    } else {
                                        break 'b_26ffe8;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_26ffec;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0026fff4);
                        c.g[A1][0] = 0x3;
                        Callee {
                            addr: 0x0026f4a0,
                            f: ee::s07::Replay_SetState,
                        }
                        .call(c, 0x0026fff4)?;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                        if t {
                            break 'b_270020;
                        } else {
                            break 'b_26fffc;
                        }
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = 0x3;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_27001c;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x604), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0027001c);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x002703f0,
                        f: ee::s07::Replay_StepFrame,
                    }
                    .call(c, 0x0027001c)?;
                    break 'b_27001c;
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
            break 'b_270020;
        }
        'b_27005c: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_27005c;
            } else {
                c.cycles += 13;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa8));
                c.g[RA][0] = sx(0x00270040);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001620d0, 0x00270040)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00270054);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00270054)?;
                c.g[RA][0] = sx(0x0027005c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0022e840, 0x0027005c)?;
                break 'b_27005c;
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
    fn replay_process_command_matches() {
        check(
            ee::s07::f_0026fe50__generated,
            replay_process_command,
            200,
            &|c, seed| {
                machine(c, seed, 0xfe50, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0026fe50;
            },
        );
    }
}
