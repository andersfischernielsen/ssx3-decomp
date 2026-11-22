//! Native board-trail draw-window builder0x2ea538, retained uninstalled.
//! No tests, builds, replays or game runs: the user disabled them.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x002e_a538;
const LOOP: u32 = 0x002e_a5a0;
const DRAW: u32 = 0x002e_a630;
const RETURN: u32 = 0x002e_a650;
const END: u32 = 0x002e_a66c;
const GFX_GP: u32 = 0x854;
const STATE_TOP: u32 = 0xe84;
const TRAIL_HEAD: u32 = 0x8c;
const TRAIL_COUNT: u32 = 0x90;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 { r(c, reg).wrapping_add(offset) }

/// `void cBoardTrailFX_RenderBatch(cBoardTrailFX** trails, int count)`.
/// Push the graphics state, build at most16 descriptors for the54-slice
/// trail rings, submit through graphics slot89, then pop the state. Each
/// descriptor contains points/start/count/fade/fadeStep; no texture or
/// palette is selected by this function. The renderer owns that binding.
pub fn render_batch(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x150 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 19;
                    Frame::open(c, frame.size);
                    c.g[A2][0] = c.lw(r(c, GP).wrapping_sub(GFX_GP));
                    frame.save_ra(c, 0x140);
                    c.g[T2][0] = 0;
                    c.g[V1][0] = c.lw(at(c, A2, STATE_TOP));
                    c.ldl(V0, at(c, V1, 7));
                    c.ldr(V0, r(c, V1));
                    c.ldl(A3, at(c, V1, 0xf));
                    c.ldr(A3, at(c, V1, 8));
                    c.g[T0][0] = c.lw(at(c, V1, 0x10));
                    c.sdl(V0, at(c, V1, 0x1b));
                    c.sdr(V0, at(c, V1, 0x14));
                    c.sdl(A3, at(c, V1, 0x23));
                    c.sdr(A3, at(c, V1, 0x1c));
                    c.sw(at(c, V1, 0x24), c.g[T0][0]);
                    c.g[V0][0] = c.lw(at(c, A2, STATE_TOP));
                    c.g[V0][0] = sx(at(c, V0, 0x14));
                    let empty = (c.g[A1][0] as i64) <= 0;
                    c.sw(at(c, A2, STATE_TOP), c.g[V0][0]);
                    if empty {
                        pc = DRAW;
                    } else {
                        c.cycles += 7;
                        c.g[T4][0] = c.lw(at(c, GP, 0x14d8));
                        c.g[T3][0] = 54;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 1);
                        c.g[T5][0] = 0;
                        c.g[T0][0] = c.g[SP][0];
                        pc = LOOP;
                    }
                }
                LOOP => {
                    c.cycles += 5;
                    c.g[T1][0] = c.lw(r(c, A0));
                    c.g[A2][0] = c.lw(at(c, T1, TRAIL_COUNT));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw(at(c, T1, TRAIL_HEAD));
                        c.g[V0][0] = sx(r(c, A2).wrapping_sub(57));
                        c.g[A3][0] = sx(r(c, A2).wrapping_sub(50));
                        c.g[A2][0] = sx(r(c, A2).wrapping_sub(2));
                        let capped = (c.g[A3][0] as i64) > 0;
                        c.g[V1][0] = sx(r(c, V1).wrapping_sub(r(c, V0)));
                        if capped {
                            c.cycles += 2;
                            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A3)));
                            c.g[A2][0] = sx(r(c, A2).wrapping_sub(r(c, A3)));
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
                        if c.g[V0][0] == 0 {
                            c.cycles += 5;
                            c.div(0, V1, T3);
                            c.mtc1(T4, 0);
                            c.cvt_s_w(0, 0);
                            // Original T3==T5 branch has identical successors.
                            c.cycles += 5;
                            c.g[V0][0] = ((c.g[T2][0] as i64) < 16) as u64;
                            c.div_s(0, 1, 0);
                            c.g[V1][0] = c.hi[0];
                            let room = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(at(c, T1, 0x10));
                            if room {
                                c.cycles += 7;
                                c.sw(at(c, T0, 4), c.g[V1][0]);
                                c.sw(r(c, T0), c.g[V0][0]);
                                c.g[T2][0] = sx(r(c, T2).wrapping_add(1));
                                c.sw(at(c, T0, 8), c.g[A2][0]);
                                c.swc1(0, at(c, T0, 0x10));
                                c.sw(at(c, T0, 0xc), 0);
                                c.g[T0][0] = sx(at(c, T0, 0x14));
                            }
                            c.cycles += 1;
                        }
                    }
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    c.cycles += 2;
                    let more = (c.g[A1][0] as i64) > 0;
                    c.g[A0][0] = sx(at(c, A0, 4));
                    if more {
                        if event_due(c) { deliver_event_here(c, LOOP); }
                        pc = LOOP;
                    } else {
                        pc = DRAW;
                    }
                }
                DRAW => {
                    c.cycles += 15;
                    c.g[A3][0] = c.lw(r(c, GP).wrapping_sub(GFX_GP));
                    c.g[A2][0] = c.g[T2][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw(at(c, A3, 0x10d8));
                    c.g[A0][0] = c.lh(at(c, V0, 0x2c8));
                    c.g[V1][0] = c.lw(at(c, V0, 0x2cc));
                    let target = r(c, V1);
                    c.g[RA][0] = sx(RETURN);
                    c.g[A0][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
                    call_ptr(c, target, RETURN)?;
                    pc = RETURN;
                }
                RETURN => {
                    c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(GFX_GP));
                    frame.restore_ra(c, 0x140);
                    c.g[V0][0] = c.lw(at(c, A0, STATE_TOP));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x14));
                    c.sw(at(c, A0, STATE_TOP), c.g[V0][0]);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
