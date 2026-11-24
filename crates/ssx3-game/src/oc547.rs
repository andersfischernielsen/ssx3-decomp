//! Staging module oc547 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc547`.

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
    &[(0x002710a8, replay_write_save_block, "Replay_WriteSaveBlock")];

/// `Replay_WriteSaveBlock` (0x002710a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_write_save_block(c: &mut Cpu) {
    if c.pc != 0x002710a8 {
        return ee::s07::f_002710a8__generated(c);
    }
    guest(c, |c| {
        'b_27120c: {
            'b_2710e0: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0xb;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S2][0] = c.g[A1][0];
                if t {
                    break 'b_2710e0;
                } else {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32), 0u64);
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_27120c;
                    } else {
                        break 'b_2710e0;
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63c));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x64c), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x650), c.g[V0][0]);
            'l_2710f0: loop {
                'b_2711fc: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x002710f8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00270fc0,
                        f: ee::s07::Replay_FlushSaveOutput,
                    }
                    .call(c, 0x002710f8)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 16384i64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                        break 'b_2711fc;
                    } else {
                        'b_2711e0: {
                            'b_2711dc: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x480000;
                                if t {
                                    break 'b_2711dc;
                                } else {
                                    'b_2711d4: {
                                        'b_2711c4: {
                                            'b_2711b4: {
                                                'b_2711a4: {
                                                    'b_271194: {
                                                        'b_271184: {
                                                            'b_271174: {
                                                                'b_271164: {
                                                                    'b_271154: {
                                                                        'b_271144: {
                                                                            c.cycles += 6;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1820,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[V1][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[A0][0] as u32;
                                                                            match t {
                                                                                0x00271134 => {
                                                                                    c.cycles += 4;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0027113c,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    Callee { addr: 0x00271228, f: ee::s07::Replay_SaveBuildHeader }.call(c, 0x0027113c)?;
                                                                                    let t = true;
                                                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                                                                    if t {
                                                                                        break 'b_2711e0;
                                                                                    } else {
                                                                                        break 'b_271144;
                                                                                    }
                                                                                }
                                                                                0x00271144 => {
                                                                                    break 'b_271144;
                                                                                }
                                                                                0x00271154 => {
                                                                                    break 'b_271154;
                                                                                }
                                                                                0x00271164 => {
                                                                                    break 'b_271164;
                                                                                }
                                                                                0x00271174 => {
                                                                                    break 'b_271174;
                                                                                }
                                                                                0x00271184 => {
                                                                                    break 'b_271184;
                                                                                }
                                                                                0x00271194 => {
                                                                                    break 'b_271194;
                                                                                }
                                                                                0x002711a4 => {
                                                                                    break 'b_2711a4;
                                                                                }
                                                                                0x002711b4 => {
                                                                                    break 'b_2711b4;
                                                                                }
                                                                                0x002711c4 => {
                                                                                    break 'b_2711c4;
                                                                                }
                                                                                0x002711d4 => {
                                                                                    break 'b_2711d4;
                                                                                }
                                                                                0x002711dc => {
                                                                                    break 'b_2711dc;
                                                                                }
                                                                                _ => {
                                                                                    c.jump_out(
                                                                                        t,
                                                                                        0x002710a8,
                                                                                        0x00271224,
                                                                                    );
                                                                                    return Ok(());
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x0027114c);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        Callee { addr: 0x002712c0, f: ee::s07::Replay_SaveWriteHeader }.call(c, 0x0027114c)?;
                                                                        let t = true;
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x64c));
                                                                        if t {
                                                                            break 'b_2711e0;
                                                                        } else {
                                                                            break 'b_271154;
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0027115c);
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    Callee { addr: 0x00271348, f: ee::s07::Replay_SaveBeginSnapshots }.call(c, 0x0027115c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x64c));
                                                                    if t {
                                                                        break 'b_2711e0;
                                                                    } else {
                                                                        break 'b_271164;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0027116c);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x00271380, f: ee::s07::Replay_SaveWriteSnapshot }.call(c, 0x0027116c)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x64c));
                                                                if t {
                                                                    break 'b_2711e0;
                                                                } else {
                                                                    break 'b_271174;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x0027117c);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            Callee {
                                                                addr: 0x002714e0,
                                                                f: ee::s07::Replay_SaveBeginCaches,
                                                            }
                                                            .call(c, 0x0027117c)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x64c));
                                                            if t {
                                                                break 'b_2711e0;
                                                            } else {
                                                                break 'b_271184;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x0027118c);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        Callee {
                                                            addr: 0x00271510,
                                                            f: ee::s07::Replay_SaveWriteCache,
                                                        }
                                                        .call(c, 0x0027118c)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x64c));
                                                        if t {
                                                            break 'b_2711e0;
                                                        } else {
                                                            break 'b_271194;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x0027119c);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    Callee {
                                                        addr: 0x00271620,
                                                        f: ee::s07::Replay_SaveComputePadding,
                                                    }
                                                    .call(c, 0x0027119c)?;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x64c)
                                                        );
                                                    if t {
                                                        break 'b_2711e0;
                                                    } else {
                                                        break 'b_2711a4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x002711ac);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x00271688,
                                                    f: ee::s07::Replay_SaveWritePadding,
                                                }
                                                .call(c, 0x002711ac)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                                if t {
                                                    break 'b_2711e0;
                                                } else {
                                                    break 'b_2711b4;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002711bc);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x002716f0,
                                                f: ee::s07::Replay_NextSaveBlock,
                                            }
                                            .call(c, 0x002711bc)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                            if t {
                                                break 'b_2711e0;
                                            } else {
                                                break 'b_2711c4;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002711cc);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00271708,
                                            f: ee::s07::Replay_SaveWriteFooter,
                                        }
                                        .call(c, 0x002711cc)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                        if t {
                                            break 'b_2711e0;
                                        } else {
                                            break 'b_2711d4;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x638), c.g[V0][0]);
                                    break 'b_2711dc;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                            break 'b_2711e0;
                        }
                        'b_2711f8: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 16384i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0xb;
                            if t {
                                break 'b_2711f8;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002710f0);
                                    }
                                    continue 'l_2710f0;
                                } else {
                                    break 'b_2711f8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                        break 'b_2711fc;
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S1][0] as u32), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63c));
                c.sw((c.g[S2][0] as u32), c.g[V1][0]);
                break 'b_27120c;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
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
    fn replay_write_save_block_matches() {
        check(
            ee::s07::f_002710a8__generated,
            replay_write_save_block,
            200,
            &|c, seed| {
                machine(c, seed, 0x10a8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002710a8;
            },
        );
    }
}
