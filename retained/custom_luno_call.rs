//! Native LunoVM_Call (0x002224b8), retained uninstalled.
//!
//! Generated EE is the authority. Automated tests and replays are disabled
//! by the user; no tests, builds or game runs were added or performed here.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, luno_vm_call, "LunoVM_Call")];

const ENTRY: u32 = 0x0022_24b8;
const END: u32 = 0x0022_2648;
const FRAME_BYTES: u32 = 0x90;
const REGISTER_BYTES: u32 = 12;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

/// `void LunoVM_Call(VM* vm, FunctionRef* function, Value* receiver,
/// Value* result)`.
///
/// Allocate twelve-byte VM registers using the first function descriptor's
/// word +8, initialize them, then place the receiver table in register zero
/// and the global default argument in register one. Both assignments happen
/// unconditionally: a native callable blob needs at least two registers even
/// when its own instructions mention only register zero. Register slots have
/// `{word0, word1, type}`; they differ from the VM's sixteen-byte named args.
///
/// The allocation has a sixteen-byte prefix whose first word holds its count;
/// `*vm` points past that prefix. The function delegates execution with the
/// original FunctionRef and result pointers, then restores its caller state.
pub fn luno_vm_call(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 32;
                    Frame::open(c, FRAME_BYTES);
                    c.g[V1][0] = REGISTER_BYTES as u64;
                    frame.save(c, S1, 0x70);
                    frame.save(c, S3, 0x50);
                    frame.save(c, S4, 0x40);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S5, 0x30);
                    c.g[S4][0] = c.g[A0][0];
                    frame.save(c, S6, 0x20);
                    c.g[A2][0] = 0x2000_0000;
                    frame.save(c, S7, 0x10);
                    c.g[S6][0] = c.g[A1][0];
                    frame.save(c, S0, 0x80);
                    c.g[S7][0] = c.g[A3][0];
                    frame.save(c, S2, 0x60);
                    c.g[A1][0] = 0x0048_0000;
                    frame.save_ra(c, 0);
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6880));
                    c.g[A3][0] = 0;
                    c.g[S5][0] = 0x004d_0000;
                    c.g[V0][0] = c.lw(r(c, S6));
                    c.g[A0][0] = c.lw(at(c, V0, 4));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[S0][0] = c.lw(at(c, V0, 8));
                    c.mult(0, V1, S0, V1);
                    c.g[A0][0] = sx(r(c, V1).wrapping_add(0x10));
                    call_ptr(c, 0x0031_7e30, 0x0022_2524)?;
                    pc = 0x0022_2524;
                }
                0x0022_2524 => {
                    c.g[S1][0] = sx(at(c, V0, 0x10));
                    c.sw(r(c, V0), c.g[S0][0]);
                    c.g[V1][0] = sx(r(c, S0).wrapping_sub(1));
                    let empty = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S1][0];
                    if empty {
                        pc = 0x0022_255c;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = u64::MAX;
                        pc = 0x0022_2540;
                    }
                }
                0x0022_2540 => {
                    c.cycles += 7;
                    c.sw(at(c, V0, 8), 0);
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                    c.sw(r(c, V0), 0);
                    let more = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = sx(at(c, V0, REGISTER_BYTES));
                    if more {
                        if back_edge_unwinds(c, 0x0022_2540) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0022_2540;
                    } else {
                        pc = 0x0022_255c;
                    }
                }
                0x0022_255c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S1, 8));
                    c.g[S2][0] = 4;
                    let release = c.g[V0][0] == c.g[S2][0];
                    c.sw(r(c, S4), c.g[S1][0]);
                    if release {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 2;
                        call_ptr(c, 0x0022_4df0, 0x0022_2578)?;
                    }
                    pc = 0x0022_2578;
                }
                0x0022_2578 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S3][0];
                    c.sw(at(c, S1, 8), 0);
                    c.sw(r(c, S1), 0);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0022_5068, 0x0022_258c)?;
                    pc = 0x0022_258c;
                }
                0x0022_258c => {
                    c.g[S3][0] = sx(r(c, S5).wrapping_sub(0x6f68));
                    c.g[S0][0] = c.lw(r(c, S4));
                    c.sw(at(c, S1, 8), c.g[S2][0]);
                    c.g[V0][0] = c.lw(at(c, S0, 0x14));
                    let release = c.g[V0][0] == c.g[S2][0];
                    c.g[S1][0] = sx(at(c, S0, REGISTER_BYTES));
                    if release {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 2;
                        call_ptr(c, 0x0022_4df0, 0x0022_25b0)?;
                    }
                    pc = 0x0022_25b0;
                }
                0x0022_25b0 => {
                    c.cycles += 5;
                    c.sw(at(c, S1, 8), 0);
                    c.sw(at(c, S0, REGISTER_BYTES), 0);
                    c.g[V1][0] = c.lw(at(c, S3, 8));
                    let table = c.g[V1][0] == c.g[S2][0];
                    c.g[V0][0] = 5;
                    if table {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, 0x0022_5068, 0x0022_25d0)?;
                        pc = 0x0022_25d0;
                    } else {
                        c.cycles += 2;
                        let pair = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = c.lw(r(c, S5).wrapping_sub(0x6f68));
                        if pair {
                            c.cycles += 7;
                            c.g[A1][0] = sx(r(c, S5).wrapping_sub(0x6f68));
                            c.ldl(V0, at(c, A1, 7));
                            c.ldr(V0, r(c, A1));
                            c.sdl(V0, at(c, S0, 0x13));
                            c.sdr(V0, at(c, S0, REGISTER_BYTES));
                            c.g[V0][0] = c.lw(at(c, S3, 8));
                        } else {
                            c.cycles += 2;
                            c.sw(at(c, S0, REGISTER_BYTES), c.g[V0][0]);
                            c.g[V0][0] = c.lw(at(c, S3, 8));
                        }
                        pc = 0x0022_2604;
                    }
                }
                0x0022_25d0 => {
                    c.g[V0][0] = c.lw(at(c, S3, 8));
                    pc = 0x0022_2604;
                }
                0x0022_2604 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S7][0];
                    c.sw(at(c, S1, 8), c.g[V0][0]);
                    call_ptr(c, 0x0022_27d0, 0x0022_2618)?;
                    pc = 0x0022_2618;
                }
                0x0022_2618 => {
                    frame.restore(c, S0, 0x80);
                    frame.restore(c, S1, 0x70);
                    frame.restore(c, S2, 0x60);
                    frame.restore(c, S3, 0x50);
                    frame.restore(c, S4, 0x40);
                    frame.restore(c, S5, 0x30);
                    frame.restore(c, S6, 0x20);
                    frame.restore(c, S7, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
