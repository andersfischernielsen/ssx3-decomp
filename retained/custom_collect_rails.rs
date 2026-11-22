//! Native Cell_FindRailSubtree0x3411b8, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds, replays or game runs
//! were added or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY, cell_find_rail_subtree, "Cell_FindRailSubtree",
)];

const ENTRY: u32 = 0x0034_11b8;
const END: u32 = 0x0034_1368;
const OBJECT_LOOP: u32 = 0x0034_11f8;
const OBJECT_TAIL: u32 = 0x0034_129c;
const EPILOGUE: u32 = 0x0034_1348;
const CHILD_TESTS: [u32; 8] = [
    0x0034_12a4, 0x0034_12bc, 0x0034_12d0, 0x0034_12e4,
    0x0034_12f8, 0x0034_130c, 0x0034_1320, 0x0034_1334,
];
const CHILD_RETURNS: [u32; 8] = [
    0x0034_12b8, 0x0034_12cc, 0x0034_12e0, 0x0034_12f4,
    0x0034_1308, 0x0034_131c, 0x0034_1330, 0x0034_1344,
];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Complete one of the eight unrolled child branches. Its delay-slot load
/// executes both after a null child and after the recursive call returns.
#[inline]
fn after_child(c: &mut Cpu, frame: Frame, child: usize) -> u32 {
    if child == 7 {
        frame.restore(c, S0, 0x60);
        EPILOGUE
    } else {
        c.g[A0][0] = c.lw(at(c, S2, ((child + 1) * 4) as u32));
        CHILD_TESTS[child + 1]
    }
}

/// Walk every object and then all eight children of one spatial cell.
/// Object+0 is the intrusive next pointer; object+8 selects plain rail1,
/// modifier2 or flex3. Plain rails additionally require owner+0x1c to
/// intersect query+8's mask. There is no count/capacity or cell-flag gate
/// here. Query fields and pointers are forwarded to the closest-point
/// callees; this function neither inserts objects nor repairs their prefix.
pub fn cell_find_rail_subtree(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    Frame::open(c, 0x70);
                    frame.save(c, S0, 0x60);
                    frame.save(c, S2, 0x40);
                    frame.save(c, S3, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S4, 0x20);
                    frame.save(c, S5, 0x10);
                    frame.save_ra(c, 0);
                    frame.save(c, S1, 0x50);
                    c.g[S1][0] = c.lw(at(c, S2, 0x28));
                    let empty = c.g[S1][0] == 0;
                    c.g[S0][0] = c.g[A1][0];
                    if empty {
                        pc = CHILD_TESTS[0];
                    } else {
                        c.cycles += 4;
                        c.g[S5][0] = 2;
                        c.g[S4][0] = 3;
                        c.g[S3][0] = 1;
                        c.g[V0][0] = c.lw(at(c, S1, 8));
                        pc = OBJECT_LOOP;
                    }
                }
                OBJECT_LOOP => {
                    c.cycles += 2;
                    pc = if c.g[V0][0] == c.g[S5][0] {
                        0x0034_1200
                    } else {
                        0x0034_122c
                    };
                }
                0x0034_1200 | 0x0034_1234 => {
                    let modifier = pc == 0x0034_1200;
                    let ret = if modifier { 0x0034_1224 } else { 0x0034_1258 };
                    c.cycles += 11;
                    c.g[A1][0] = c.lw(at(c, S0, 0x14));
                    c.g[A0][0] = c.lw(at(c, S1, 0xc));
                    c.g[A3][0] = c.lw(r(c, S0));
                    c.g[A2][0] = c.g[A1][0];
                    c.g[T0][0] = c.lw(at(c, S0, 8));
                    c.g[T1][0] = c.lw(at(c, S0, 0xc));
                    c.g[T2][0] = c.lw(at(c, S0, 0x10));
                    c.g[RA][0] = sx(ret);
                    c.g[T3][0] = c.lw(at(c, S0, 4));
                    let callee = if modifier { 0x0035_c698 } else { 0x0034_8290 };
                    call_ptr(c, callee, ret)?;
                    pc = ret;
                }
                0x0034_1224 | 0x0034_1258 => {
                    c.g[S1][0] = c.lw(r(c, S1));
                    pc = OBJECT_TAIL;
                }
                0x0034_122c => {
                    c.cycles += 2;
                    pc = if c.g[V0][0] == c.g[S4][0] {
                        0x0034_1234
                    } else {
                        0x0034_1260
                    };
                }
                0x0034_1260 => {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S1][0] = c.lw(r(c, S1));
                        pc = OBJECT_TAIL;
                    } else {
                        pc = 0x0034_1268;
                    }
                }
                0x0034_1268 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, S1, 0x68));
                    c.g[A0][0] = c.lw(at(c, S0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0x1c));
                    c.g[V1][0] &= c.g[A0][0];
                    let disabled = c.g[V1][0] == 0;
                    c.g[A0][0] = c.g[S1][0];
                    pc = if disabled { 0x0034_1298 } else { 0x0034_1280 };
                }
                0x0034_1280 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(at(c, S0, 0x14));
                    c.g[A2][0] = c.lw(r(c, S0));
                    c.g[A3][0] = c.lw(at(c, S0, 0xc));
                    c.g[T0][0] = c.lw(at(c, S0, 0x10));
                    c.g[RA][0] = sx(0x0034_1298);
                    c.g[T1][0] = c.lw(at(c, S0, 4));
                    call_ptr(c, 0x0033_5128, 0x0034_1298)?;
                    pc = 0x0034_1298;
                }
                0x0034_1298 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lw(r(c, S1));
                    pc = OBJECT_TAIL;
                }
                OBJECT_TAIL => {
                    c.cycles += 2;
                    if c.g[S1][0] != 0 {
                        c.g[V0][0] = c.lw(at(c, S1, 8));
                        if event_due(c) { deliver_event_here(c, OBJECT_LOOP); }
                        pc = OBJECT_LOOP;
                    } else {
                        pc = CHILD_TESTS[0];
                    }
                }
                child_pc if CHILD_TESTS.contains(&child_pc) => {
                    let child = CHILD_TESTS.iter().position(|&p| p == child_pc).unwrap();
                    c.cycles += if child == 0 { 3 } else { 2 };
                    if child == 0 { c.g[A0][0] = c.lw(r(c, S2)); }
                    if c.g[A0][0] == 0 {
                        pc = after_child(c, frame, child);
                    } else {
                        c.cycles += 3;
                        let ret = CHILD_RETURNS[child];
                        c.g[RA][0] = sx(ret);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, ENTRY, ret)?;
                        pc = ret;
                    }
                }
                child_ret if CHILD_RETURNS.contains(&child_ret) => {
                    let child = CHILD_RETURNS.iter().position(|&p| p == child_ret).unwrap();
                    pc = after_child(c, frame, child);
                }
                EPILOGUE => {
                    c.cycles += 8;
                    frame.restore(c, S1, 0x50);
                    frame.restore(c, S2, 0x40);
                    frame.restore(c, S3, 0x30);
                    frame.restore(c, S4, 0x20);
                    frame.restore(c, S5, 0x10);
                    frame.restore_ra(c, 0);
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
