//! Staging module oc616 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc616`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x00362478, gs_pkt_add_alpha, "GsPkt_AddAlpha")];

/// `GsPkt_AddAlpha` (0x00362478): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn gs_pkt_add_alpha(c: &mut Cpu) {
    if c.pc != 0x00362478 {
        return ee::s12::f_00362478__generated(c);
    }
    guest(c, |c| {
        'b_36249c: {
            'b_362498: {
                'b_362494: {
                    c.cycles += 2;
                    let t = c.g[A3][0] == 0u64;
                    c.g[T0][0] = c.g[A0][0];
                    if t {
                        break 'b_362494;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_362498;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x43;
                            if t {
                                break 'b_36249c;
                            } else {
                                break 'b_362494;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                break 'b_362498;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x42;
            break 'b_36249c;
        }
        'b_3625a0: {
            c.cycles += 4;
            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V0][0] = (c.g[A1][0] < 0x16u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x490000;
            if t {
                break 'b_3625a0;
            } else {
                'b_36259c: {
                    'b_362594: {
                        'b_362588: {
                            'b_36257c: {
                                'b_362568: {
                                    'b_36255c: {
                                        'b_362550: {
                                            'b_362540: {
                                                'b_362530: {
                                                    'b_362524: {
                                                        'b_362518: {
                                                            'b_36250c: {
                                                                'b_362500: {
                                                                    'b_3624f4: {
                                                                        'b_3624e8: {
                                                                            'b_3624dc: {
                                                                                'b_3624d0: {
                                                                                    c.cycles += 6;
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[A1][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1fb0));
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                    c.g[A0][0] = c
                                                                                        .lw(
                                                                                        (c.g[V1][0]
                                                                                            as u32),
                                                                                    );
                                                                                    let t = c.g[A0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    match t {
0x003624c4 => {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
let t = true;
c.g[V0][0] = 0xaa;
if t {
break 'b_36259c;
} else {
break 'b_3624d0;
}
}
0x003624d0 => {
break 'b_3624d0;
}
0x003624dc => {
break 'b_3624dc;
}
0x003624e8 => {
break 'b_3624e8;
}
0x003624f4 => {
break 'b_3624f4;
}
0x00362500 => {
break 'b_362500;
}
0x0036250c => {
break 'b_36250c;
}
0x00362518 => {
break 'b_362518;
}
0x00362524 => {
break 'b_362524;
}
0x00362530 => {
break 'b_362530;
}
0x00362540 => {
break 'b_362540;
}
0x00362550 => {
break 'b_362550;
}
0x00362568 => {
break 'b_362568;
}
0x0036257c => {
break 'b_36257c;
}
0x00362588 => {
break 'b_362588;
}
0x00362594 => {
break 'b_362594;
}
0x003625a0 => {
break 'b_3625a0;
}
_ => { c.jump_out(t, 0x00362478, 0x003625bc); return Ok(()); }
}
                                                                                }
                                                                                c.cycles += 3;
                                                                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                                                let t = true;
                                                                                c.g[V0][0] = 0x2a;
                                                                                if t {
                                                                                    break 'b_36259c;
                                                                                } else {
                                                                                    break 'b_3624dc;
                                                                                }
                                                                            }
                                                                            c.cycles += 3;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            let t = true;
                                                                            c.g[V0][0] = 0x44;
                                                                            if t {
                                                                                break 'b_36259c;
                                                                            } else {
                                                                                break 'b_3624e8;
                                                                            }
                                                                        }
                                                                        c.cycles += 3;
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[T0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        let t = true;
                                                                        c.g[V0][0] = 0x46;
                                                                        if t {
                                                                            break 'b_36259c;
                                                                        } else {
                                                                            break 'b_3624f4;
                                                                        }
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    let t = true;
                                                                    c.g[V0][0] = 0x48;
                                                                    if t {
                                                                        break 'b_36259c;
                                                                    } else {
                                                                        break 'b_362500;
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                let t = true;
                                                                c.g[V0][0] = 0x81;
                                                                if t {
                                                                    break 'b_36259c;
                                                                } else {
                                                                    break 'b_36250c;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V1][0] = c.lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            let t = true;
                                                            c.g[V0][0] = 0x42;
                                                            if t {
                                                                break 'b_36259c;
                                                            } else {
                                                                break 'b_362518;
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        let t = true;
                                                        c.g[V0][0] = 0x89;
                                                        if t {
                                                            break 'b_36259c;
                                                        } else {
                                                            break 'b_362524;
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                    let t = true;
                                                    c.g[V0][0] = 0x6a;
                                                    if t {
                                                        break 'b_36259c;
                                                    } else {
                                                        break 'b_362530;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[V0][0] = c.g[A2][0] << 32;
                                                c.g[A0][0] =
                                                    c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                let t = true;
                                                c.g[V1][0] = 0x64;
                                                if t {
                                                    break 'b_36255c;
                                                } else {
                                                    break 'b_362540;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] = c.g[A2][0] << 32;
                                            c.g[A0][0] =
                                                c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                            let t = true;
                                            c.g[V1][0] = 0x68;
                                            if t {
                                                break 'b_36255c;
                                            } else {
                                                break 'b_362550;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A2][0] << 32;
                                        c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = 0x88;
                                        break 'b_36255c;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    let t = true;
                                    c.sd((c.g[A0][0] as u32), c.g[V0][0]);
                                    if t {
                                        break 'b_3625a0;
                                    } else {
                                        break 'b_362568;
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = 0x80;
                                c.g[V0][0] = c.g[V0][0] << 32;
                                c.g[V0][0] = c.g[V0][0] | 0x68;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_36259c;
                                } else {
                                    break 'b_36257c;
                                }
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                            let t = true;
                            c.g[V0][0] = 0x98;
                            if t {
                                break 'b_36259c;
                            } else {
                                break 'b_362588;
                            }
                        }
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                        let t = true;
                        c.g[V0][0] = 0x58;
                        if t {
                            break 'b_36259c;
                        } else {
                            break 'b_362594;
                        }
                    }
                    c.cycles += 2;
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x49;
                    break 'b_36259c;
                }
                c.cycles += 1;
                c.sd((c.g[V1][0] as u32), c.g[V0][0]);
                break 'b_3625a0;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.sw((c.g[T0][0] as u32), c.g[V1][0]);
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
    fn gs_pkt_add_alpha_matches() {
        check(
            ee::s12::f_00362478__generated,
            gs_pkt_add_alpha,
            200,
            &|c, seed| {
                machine(c, seed, 0x2478, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00362478;
            },
        );
    }
}
