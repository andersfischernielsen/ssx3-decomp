//! Native GMHandler1_OnRiderFinished0x23a760, retained uninstalled.
//!
//! Race handler1 records times per heat. Its completion gate is every human
//! player, not every AI racer. Unfinished racers receive native time estimates.
//! Heat3 sets manager+9c; manager+98 is not changed here. This translation
//! retains the original register/cycle/call and event-unwind contracts.
//! No tests, builds, replays or runtime installation are performed.

use crate::call::{Frame, Unwound, call_ptr, guest};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
const FRAME_BYTES: u32 = 0xc0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}
#[inline]
fn gp(c: &Cpu, offset: i32) -> u32 {
    r(c, GP).wrapping_add(offset as u32)
}

/// `void GMHandler1_OnRiderFinished(Handler1* self, int rider_index)`.
/// Record the native heat row (40 bytes per heat at+58); after the human
/// completion gate, estimate unfinished opponents, sort, advance heat policy,
/// mark result readiness on heat3 and retain the native first-player award.
pub fn gm_handler1_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        // Record the current heat's actual time, or the native DNF sentinel.
        'b_23a8b4: {
            'b_23a82c: {
                c.cycles += 37;
                Frame::open(c, FRAME_BYTES);
                frame.save(c, S2, 0x80);
                frame.save(c, S7, 0x30);
                c.g[S2][0] = c.g[A1][0];
                frame.save(c, S0, 0xa0);
                c.g[S7][0] = c.g[A0][0];
                frame.save(c, S1, 0x90);
                frame.save(c, S3, 0x70);
                frame.save(c, FP, 0x20);
                frame.save(c, S4, 0x60);
                frame.save(c, S5, 0x50);
                frame.save(c, S6, 0x40);
                frame.save_ra(c, 0x10);
                c.g[RA][0] = sx(0x0023a79c);
                frame.save_fpr(c, 20, 0xb0);
                call_ptr(c, 0x0014dc80, 0x0023a79c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023a7a8);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023a7a8)?;
                c.g[RA][0] = sx(0x0023a7b0);
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023a7b0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023a7bc);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0023a7bc)?;
                c.g[A0][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x0023a7c8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e98, 0x0023a7c8)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(gp(c, -(0x480i32)));
                c.g[V0][0] = c.lw(gp(c, -(0x848i32)));
                c.g[A1][0] = c.lw(r(c, V1));
                c.g[A0][0] = c.lw(at(c, V0, 0x84));
                c.g[S1][0] = sx(r(c, A1).wrapping_sub(0x1));
                c.g[S0][0] = c.lw(at(c, A0, 0xc));
                c.g[RA][0] = sx(0x0023a7ec);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0012a250, 0x0023a7ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lw(gp(c, -(0x848i32)));
                if t {
                    break 'b_23a82c;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = sx(r(c, S2) << 2);
                    c.g[S6][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw(at(c, A1, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[A0][0] = c.lw(at(c, V0, 0x28));
                    c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
                    c.g[A0][0] = sx(at(c, A0, 0x6c0));
                    c.g[A1][0] = c.lh(at(c, V1, 0x48));
                    c.g[V0][0] = c.lw(at(c, V1, 0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023a824);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
                    call_ptr(c, t, 0x0023a824)?;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_23a8b4;
                    } else {
                        break 'b_23a82c;
                    }
                }
            }
            'b_23a890: {
                c.cycles += 12;
                c.g[V0][0] = c.lw(gp(c, -(0x848i32)));
                c.g[A0][0] = sx(r(c, S2) << 2);
                c.g[A2][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                c.g[S6][0] = c.g[A0][0];
                c.g[V1][0] = c.lw(at(c, V0, 0x84));
                c.g[A1][0] = c.lw(at(c, A2, 0x28));
                c.g[V0][0] = c.lw(at(c, V1, 0xc));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(at(c, V0, 0x28));
                c.g[A0][0] = c.lw(at(c, V1, 0x480));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = c.lw(at(c, A1, 0x478));
                if t {
                    break 'b_23a890;
                } else {
                    c.cycles += 13;
                    c.g[V1][0] = 0x28;
                    c.g[A0][0] = 0x50000;
                    c.mult(0, 2, 17, 3);
                    c.g[A0][0] = c.g[A0][0] | 0x7e40;
                    c.g[A1][0] = 0x1;
                    c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, S6)));
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = sx(r(c, S7).wrapping_add(r(c, V1)));
                    c.g[V0][0] = sx(at(c, V0, 0x66a8));
                    c.sw(at(c, V1, 0x58), c.g[A0][0]);
                    c.g[V0][0] = sx(r(c, S6).wrapping_add(r(c, V0)));
                    let t = true;
                    c.sw(r(c, V0), c.g[A1][0]);
                    if t {
                        break 'b_23a8b4;
                    } else {
                        break 'b_23a890;
                    }
                }
            }
            c.cycles += 9;
            c.g[V0][0] = 0x28;
            c.g[V1][0] = 0x530000;
            c.mult(0, 4, 17, 2);
            c.g[V1][0] = sx(at(c, V1, 0x66a8));
            c.g[V1][0] = sx(r(c, S6).wrapping_add(r(c, V1)));
            c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, S6)));
            c.g[V0][0] = sx(r(c, S7).wrapping_add(r(c, V0)));
            c.sw(at(c, V0, 0x58), c.g[A1][0]);
            c.sw(r(c, V1), 0u64);
            break 'b_23a8b4;
        }
        // Finalization requires all human riders, then a human finish callback.
        'b_23abdc: {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0023a8bc);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0012a250, 0x0023a8bc)?;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw(gp(c, -(0x848i32)));
            if t {
                break 'b_23abdc;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.lw(at(c, V1, 0x84));
                c.g[V0][0] = c.lw(at(c, A0, 0xc));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S6)));
                c.g[A0][0] = c.lw(at(c, V0, 0x28));
                c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
                c.g[A0][0] = sx(at(c, A0, 0x6c0));
                c.g[A1][0] = c.lh(at(c, V1, 0x40));
                c.g[V0][0] = c.lw(at(c, V1, 0x44));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023a8ec);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
                call_ptr(c, t, 0x0023a8ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = sx(r(c, S3) << 2);
                if t {
                    break 'b_23abdc;
                } else {
                    'b_23a994: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw(gp(c, -(0x480i32)));
                        c.sw(r(c, SP), c.g[S3][0]);
                        c.g[V0][0] = c.lw(at(c, V1, 0x10));
                        c.g[V0][0] = sx(at(c, V0, 0x1));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S4][0] = 0u64;
                        if t {
                            break 'b_23a994;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = 0x28;
                            c.g[V1][0] = 0x530000;
                            c.mult(0, 17, 17, 2);
                            c.g[A0][0] = sx(at(c, S7, 0x58));
                            c.mtc1(0, 20);
                            c.g[S5][0] = sx(at(c, V1, 0x6640));
                            c.g[S3][0] = sx(at(c, S0, 0x28));
                            c.g[S2][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
                            // Estimate unfinished competitors; do not wait for their laps to end.
                            'l_23a930: loop {
                                'b_23a94c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw(r(c, S3));
                                    c.lwc1(0, at(c, A0, 0x470));
                                    c.c_le_s(20, 0);
                                    let t = c.fcc();
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_23a94c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = 0u64;
                                        break 'b_23a94c;
                                    }
                                }
                                'b_23a960: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx(at(c, S7, 0x58));
                                    if t {
                                        break 'b_23a960;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x0023a95c);
                                        call_ptr(c, 0x00122d78, 0x0023a95c)?;
                                        c.sw(r(c, S2), c.g[V0][0]);
                                        break 'b_23a960;
                                    }
                                }
                                c.cycles += 13;
                                c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, S1)));
                                c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                                c.g[V1][0] = c.lw(r(c, V0));
                                c.g[S4][0] = sx(at(c, S4, 0x1));
                                c.g[S1][0] = sx(at(c, S1, 0x4));
                                c.g[S2][0] = sx(at(c, S2, 0x4));
                                c.sw(r(c, S5), c.g[V1][0]);
                                c.g[S3][0] = sx(at(c, S3, 0x4));
                                c.g[V0][0] = c.lw(at(c, A0, 0x10));
                                c.g[V0][0] = sx(at(c, V0, 0x1));
                                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S5][0] = sx(at(c, S5, 0x4));
                                if t {
                                    if back_edge_unwinds(c, 0x0023_a930) {
                                        return Err(Unwound);
                                    }
                                    continue 'l_23a930;
                                } else {
                                    break 'b_23a994;
                                }
                            }
                        }
                    }
                    // Sort the heat and retain native qualification/career policy.
                    'b_23ab7c: {
                        'b_23ab64: {
                            'b_23ab60: {
                                'b_23aad0: {
                                    c.cycles += 14;
                                    c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                                    c.g[RA][0] = sx(0x0023a9a0);
                                    c.g[A1][0] = 0xa;
                                    call_ptr(c, 0x00238bf8, 0x0023a9a0)?;
                                    c.g[V1][0] = c.lw(r(c, SP));
                                    c.g[T0][0] = 0x530000;
                                    c.g[A2][0] = c.lw(gp(c, -(0x480i32)));
                                    c.g[V0][0] = sx(at(c, T0, 0x6730));
                                    c.g[A3][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                                    c.g[V1][0] = c.lw(at(c, A2, 0x70));
                                    c.sw(at(c, A2, 0x74), c.g[V1][0]);
                                    c.g[V0][0] = c.lw(r(c, A3));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_23aad0;
                                    } else {
                                        'b_23aa08: {
                                            c.cycles += 13;
                                            c.g[V0][0] = c.lw(r(c, A2));
                                            c.g[A0][0] = c.lw(gp(c, -(0x848i32)));
                                            c.g[T0][0] = 0x3;
                                            c.g[V0][0] = sx(at(c, V0, 0x1));
                                            c.sw(at(c, A2, 0x70), c.g[V0][0]);
                                            c.g[V1][0] = c.lw(at(c, A0, 0x84));
                                            c.g[V0][0] = c.lw(at(c, V1, 0xc));
                                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S6)));
                                            c.g[V1][0] = c.lw(at(c, V0, 0x28));
                                            c.sw(at(c, V1, 0x100), c.g[A1][0]);
                                            c.g[V0][0] = c.lw(r(c, A2));
                                            let t = c.g[V0][0] != c.g[T0][0];
                                            c.g[A0][0] = 0x530000;
                                            if t {
                                                break 'b_23aa08;
                                            } else {
                                                c.cycles += 2;
                                                c.sw(at(c, A2, 0x84), c.g[A1][0]);
                                                c.sw(at(c, A2, 0x70), c.g[T0][0]);
                                                break 'b_23aa08;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = sx(at(c, A0, 0x5bc8));
                                        c.g[V1][0] = c.lb(at(c, V0, 0x49));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                                        if t {
                                            break 'b_23ab64;
                                        } else {
                                            'b_23aab8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw(r(c, A3));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] = 0x3;
                                                if t {
                                                    break 'b_23aab8;
                                                } else {
                                                    'b_23aab4: {
                                                        'b_23aaa8: {
                                                            'b_23aa54: {
                                                                c.cycles += 5;
                                                                c.g[V0][0] =
                                                                    c.lw(gp(c, -(0x480i32)));
                                                                c.g[A2][0] = c.lw(r(c, V0));
                                                                c.g[V1][0] = ((c.g[A2][0] as i64)
                                                                    < 2i64)
                                                                    as u64;
                                                                let t = c.g[V1][0] == 0u64;
                                                                c.g[V1][0] = 0x530000;
                                                                if t {
                                                                    break 'b_23aa54;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] =
                                                                        sx(at(c, V1, 0x6640));
                                                                    c.g[A1][0] =
                                                                        c.lw(at(c, V1, 0x6640));
                                                                    c.g[V0][0] =
                                                                        c.lw(at(c, A0, 0x4));
                                                                    c.g[V0][0] = sx(r(c, V0)
                                                                        .wrapping_sub(r(c, A1)));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x259u64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    if t {
                                                                        break 'b_23aaa8;
                                                                    } else {
                                                                        break 'b_23aa54;
                                                                    }
                                                                }
                                                            }
                                                            'b_23aa7c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = 0x2;
                                                                let t = c.g[A2][0] != c.g[V0][0];
                                                                c.g[V1][0] = 0x530000;
                                                                if t {
                                                                    break 'b_23aa7c;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] =
                                                                        sx(at(c, V1, 0x6640));
                                                                    c.g[A1][0] =
                                                                        c.lw(at(c, V1, 0x6640));
                                                                    c.g[V0][0] =
                                                                        c.lw(at(c, A0, 0x4));
                                                                    c.g[V0][0] = sx(r(c, V0)
                                                                        .wrapping_sub(r(c, A1)));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x12du64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    if t {
                                                                        break 'b_23aaa8;
                                                                    } else {
                                                                        break 'b_23aa7c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] != c.g[T0][0];
                                                            c.g[V1][0] = 0x530000;
                                                            if t {
                                                                break 'b_23ab7c;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[A0][0] = sx(at(c, V1, 0x6640));
                                                                c.g[A1][0] =
                                                                    c.lw(at(c, V1, 0x6640));
                                                                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                                                                c.g[V0][0] =
                                                                    sx(r(c, V0)
                                                                        .wrapping_sub(r(c, A1)));
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0xb5u64) as u64;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[A0][0] =
                                                                    c.lw(gp(c, -(0x480i32)));
                                                                if t {
                                                                    break 'b_23aab4;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    break 'b_23aaa8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[RA][0] = sx(0x0023aab0);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x00147d20, 0x0023aab0)?;
                                                        c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                                                        break 'b_23aab4;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x3;
                                                    break 'b_23aab8;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(r(c, A0));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[V1][0] = 0x530000;
                                            if t {
                                                break 'b_23ab7c;
                                            } else {
                                                c.cycles += 3;
                                                c.sw(at(c, A0, 0x74), 0u64);
                                                let t = true;
                                                c.sw(at(c, A0, 0x70), 0u64);
                                                if t {
                                                    break 'b_23ab60;
                                                } else {
                                                    break 'b_23aad0;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[V1][0] = c.lw(r(c, A2));
                                c.g[V0][0] = 0x1;
                                c.g[A0][0] = c.lw(gp(c, -(0x848i32)));
                                c.g[T0][0] = 0x530000;
                                c.sw(at(c, A2, 0x70), c.g[V1][0]);
                                c.g[A1][0] = sx(at(c, T0, 0x5bc8));
                                c.sw(at(c, S7, 0xc), c.g[V0][0]);
                                c.sw(at(c, S7, 0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw(at(c, A0, 0x84));
                                c.g[V0][0] = c.lw(at(c, V1, 0xc));
                                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S6)));
                                c.g[V1][0] = c.lw(at(c, V0, 0x28));
                                c.sw(at(c, V1, 0x100), 0u64);
                                c.g[V0][0] = c.lb(at(c, A1, 0x49));
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                                    break 'b_23ab64;
                                } else {
                                    'b_23ab58: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw(r(c, A2));
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A0][0] = c.g[FP][0];
                                            break 'b_23ab58;
                                        } else {
                                            'b_23ab54: {
                                                'b_23ab3c: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_23ab3c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lw(r(c, A3));
                                                        c.g[V0][0] =
                                                            ((c.g[V0][0] as i64) < 4i64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_23ab54;
                                                        } else {
                                                            break 'b_23ab3c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[V1][0] = 0x530000;
                                                if t {
                                                    break 'b_23ab7c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = c.lw(r(c, A3));
                                                    c.g[V0][0] = 0x5;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                                                    if t {
                                                        break 'b_23ab64;
                                                    } else {
                                                        break 'b_23ab54;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A0][0] = c.g[FP][0];
                                            break 'b_23ab58;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x0023ab60);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x00147e18, 0x0023ab60)?;
                                    break 'b_23ab60;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw(gp(c, -(0x480i32)));
                            break 'b_23ab64;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = 0x3;
                        c.g[V0][0] = c.lw(r(c, A0));
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[V1][0] = 0x530000;
                        if t {
                            break 'b_23ab7c;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            c.sw(at(c, A0, 0x9c), c.g[V0][0]);
                            break 'b_23ab7c;
                        }
                    }
                    // First player's boost award follows the final native placement.
                    'b_23abc8: {
                        'b_23abb0: {
                            'b_23aba0: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw(r(c, SP));
                                c.g[V0][0] = sx(at(c, V1, 0x6730));
                                c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                                c.g[V1][0] = c.lw(r(c, V0));
                                let t = c.g[V1][0] != 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_23aba0;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, gp(c, -(0x5154i32)));
                                    let t = true;
                                    c.g[A0][0] = c.lw(gp(c, -(0x848i32)));
                                    if t {
                                        break 'b_23abb0;
                                    } else {
                                        break 'b_23aba0;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw(gp(c, -(0x848i32)));
                            if t {
                                break 'b_23abc8;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.lw(gp(c, -(0x848i32)));
                                c.lwc1(0, gp(c, -(0x5150i32)));
                                break 'b_23abb0;
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(at(c, A0, 0x84));
                        c.g[V1][0] = c.lw(at(c, V0, 0xc));
                        c.g[A0][0] = c.lw(at(c, V1, 0x40));
                        c.g[V0][0] = c.lw(at(c, A0, 0x18));
                        let t = true;
                        c.swc1(0, at(c, V0, 0x2f8));
                        if t {
                            break 'b_23abdc;
                        } else {
                            break 'b_23abc8;
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, V0, 0x84));
                    c.g[A0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = c.lw(at(c, A0, 0x40));
                    c.g[V1][0] = c.lw(at(c, V0, 0x18));
                    c.sw(at(c, V1, 0x2f8), 0u64);
                    break 'b_23abdc;
                }
            }
        }
        c.cycles += 13;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}
