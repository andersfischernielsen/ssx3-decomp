//! Native world-script manager update and imported deferred graph segments.
//!
//! No tests, builds or replays were added or run: the user has disabled
//! automated validation. Native process/command ordering is retained.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, wscript_man_update, "WScriptMan_Update")];

const ENTRY: u32 = 0x0030_9270;
const END: u32 = 0x0030_94c8;
const FRAME_BYTES: u32 = 0x220;
const PENDING_PROCESSES: u32 = 0x2bc;
const ACTIVE_PROCESSES: u32 = 0x2b8;
const COMMAND_QUEUE: u32 = 0x40;
const COMMAND_QUEUE_END: u32 = 0x1c0;
const CURRENT_INSTANCE: u32 = 0x290;
const CURRENT_PROCESS: u32 = 0x2a4;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void WScriptMan_Update(WScriptMan* manager)` (0x309270).
/// Start pending processes, snapshot/clear the 32-entry command queue, dispatch
/// its commands to active processes, then update every active process. A
/// command's low 16 flag bits zero bind its instance RID; upper bits 0x20000
/// and 0x10000 dispatch to the process and current task respectively.
///
/// Every native call return and event-check back edge remains resumable. The
/// current instance and process are transient manager fields, cleared between
/// commands and after process updates. Rider/contact context is not rewritten.
pub fn wscript_man_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 18;
                    Frame::open(c, FRAME_BYTES);
                    frame.save(c, S2, 0x1f0);
                    frame.save(c, S0, 0x210);
                    frame.save(c, S1, 0x200);
                    frame.save(c, S3, 0x1e0);
                    frame.save(c, S4, 0x1d0);
                    frame.save(c, S5, 0x1c0);
                    frame.save(c, S6, 0x1b0);
                    frame.save(c, S7, 0x1a0);
                    frame.save_ra(c, 0x190);
                    c.g[S7][0] = 0;
                    c.g[S2][0] = c.g[A0][0];
                    crate::custom_script_queue::begin_update(c);
                    crate::custom_script_materials::begin_update(c);
                    crate::custom_initial_debounce::begin_update(c);
                    crate::custom_cracked_glass::begin_update(c);
                    crate::custom_crowds::begin_update(c);
                    crate::custom_movies::begin_update(c);
                    call_ptr(c, 0x0030_9118, 0x0030_92a0)?;
                    pc = 0x0030_92a0;
                }
                0x0030_92a0 => {
                    for offset in [4, 0x10, 0x1c, 0x28, 0x34] {
                        c.sw(at(c, S2, offset), 0);
                    }
                    pc = 0x0030_92cc;
                }
                0x0030_92b8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_b260, 0x0030_92c0)?;
                    pc = 0x0030_92c0;
                }
                0x0030_92c0 => {
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0030_9750, 0x0030_92cc)?;
                    pc = 0x0030_92cc;
                }
                0x0030_92cc => {
                    c.cycles += 5;
                    c.g[A0][0] = sx(at(c, S2, PENDING_PROCESSES));
                    call_ptr(c, 0x0030_b208, 0x0030_92d4)?;
                    pc = 0x0030_92d4;
                }
                0x0030_92d4 => {
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] != 0 {
                        c.g[A0][0] = sx(at(c, S2, 0x2b4));
                        if back_edge_unwinds(c, 0x0030_92b8) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0030_92b8;
                    } else {
                        pc = 0x0030_92e0;
                    }
                }
                0x0030_92e0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(at(c, S2, COMMAND_QUEUE));
                    c.g[V0][0] = c.g[V1][0] & 7;
                    c.g[A0][0] = c.g[SP][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = sx(at(c, S2, COMMAND_QUEUE_END));
                        pc = 0x0030_934c;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx(at(c, S2, COMMAND_QUEUE_END));
                        pc = 0x0030_92f4;
                    }
                }
                0x0030_92f4 => {
                    // Preserve the original unaligned copy and its exposed
                    // argument registers, not just the resulting byte array.
                    c.cycles += 19;
                    for (register, offset) in [(A1, 0), (A2, 8), (A3, 16), (T0, 24)] {
                        c.ldl(register, at(c, V1, offset + 7));
                        c.ldr(register, at(c, V1, offset));
                    }
                    for (register, offset) in [(A1, 0), (A2, 8), (A3, 16), (T0, 24)] {
                        c.sdl(register, at(c, A0, offset + 7));
                        c.sdr(register, at(c, A0, offset));
                    }
                    c.g[V1][0] = sx(at(c, V1, 0x20));
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(at(c, A0, 0x20));
                    if more {
                        if back_edge_unwinds(c, 0x0030_92f4) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        c.cycles += 2;
                        pc = 0x0030_9378;
                    }
                }
                0x0030_934c => {
                    c.cycles += 11;
                    for (register, offset) in [(A1, 0), (A2, 8), (A3, 16), (T0, 24)] {
                        c.g[register][0] = c.ld(at(c, V1, offset));
                    }
                    for (register, offset) in [(A1, 0), (A2, 8), (A3, 16), (T0, 24)] {
                        c.sd(at(c, A0, offset), c.g[register][0]);
                    }
                    c.g[V1][0] = sx(at(c, V1, 0x20));
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(at(c, A0, 0x20));
                    if more {
                        if back_edge_unwinds(c, 0x0030_934c) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        pc = 0x0030_9378;
                    }
                }
                0x0030_9378 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.sw(r(c, A0), c.g[V0][0]);
                    c.sw(at(c, S2, COMMAND_QUEUE), 0);
                    c.g[V0][0] = c.lw(r(c, SP));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[A2][0] = 0;
                    if empty {
                        pc = 0x0030_9470;
                    } else {
                        c.cycles += 2;
                        c.g[S7][0] = 12;
                        pc = 0x0030_9398;
                    }
                }
                0x0030_9398 => {
                    c.cycles += 6;
                    c.mult(0, V0, A2, S7);
                    c.g[V0][0] = sx(at(c, V0, 4));
                    c.g[S3][0] = sx(r(c, SP).wrapping_add(r(c, V0)));
                    c.g[V1][0] = c.lhu(r(c, S3));
                    if c.g[V1][0] == 0 {
                        c.cycles += 10;
                        c.g[V1][0] = c.lw(at(c, S2, 0x28c));
                        c.g[A1][0] = c.lw(at(c, S3, 4));
                        c.g[A0][0] = c.lw(r(c, V1));
                        c.g[V0][0] = c.g[A1][0] & 255;
                        c.g[V1][0] = c.lw(at(c, A0, 8));
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        let missing_section = c.g[V1][0] == 0;
                        c.g[V0][0] = sx(r(c, A1) >> 8);
                        let mut missing = missing_section;
                        if !missing_section {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            missing = c.g[V0][0] == 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                        }
                        if missing {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                        c.cycles += 2;
                        c.sw(at(c, S2, CURRENT_INSTANCE), c.g[V0][0]);
                    }
                    c.g[S0][0] = c.lw(at(c, S2, ACTIVE_PROCESSES));
                    c.cycles += 2;
                    let no_process = c.g[S0][0] == 0;
                    c.g[S4][0] = sx(at(c, A2, 1));
                    if no_process {
                        pc = 0x0030_9458;
                    } else {
                        c.cycles += 4;
                        c.g[S6][0] = 0x20000;
                        c.g[S5][0] = 0x10000;
                        c.g[S1][0] = c.g[S0][0];
                        pc = 0x0030_9418;
                    }
                }
                0x0030_9418 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r(c, S3));
                    c.g[S0][0] = c.lw(at(c, S0, 0x18));
                    c.g[V0][0] &= c.g[S6][0];
                    let skip_process = c.g[V0][0] == 0;
                    c.sw(at(c, S2, CURRENT_PROCESS), c.g[S1][0]);
                    if skip_process {
                        pc = 0x0030_9438;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, 0x0030_78c0, 0x0030_9438)?;
                        pc = 0x0030_9438;
                    }
                }
                0x0030_9438 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(r(c, S3));
                    c.g[V0][0] &= c.g[S5][0];
                    let skip_task = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S1][0];
                    if !skip_task {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, 0x0030_7a10, 0x0030_9450)?;
                    }
                    pc = 0x0030_9450;
                }
                0x0030_9450 => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[S1][0] = c.g[S0][0];
                        if back_edge_unwinds(c, 0x0030_9418) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0030_9418;
                    } else {
                        pc = 0x0030_9458;
                    }
                }
                0x0030_9458 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r(c, SP));
                    c.g[A2][0] = c.g[S4][0];
                    c.sw(at(c, S2, CURRENT_INSTANCE), 0);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.sw(at(c, S2, CURRENT_PROCESS), 0);
                    if more {
                        if back_edge_unwinds(c, 0x0030_9398) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0030_9398;
                    } else {
                        pc = 0x0030_9470;
                    }
                }
                0x0030_9470 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, ACTIVE_PROCESSES));
                    if c.g[S0][0] == 0 {
                        c.sw(at(c, S2, CURRENT_PROCESS), 0);
                        pc = 0x0030_949c;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S0][0];
                        pc = 0x0030_9480;
                    }
                }
                0x0030_9480 => {
                    c.cycles += 6;
                    c.g[S0][0] = c.lw(at(c, S0, 0x18));
                    c.g[A0][0] = c.g[V0][0];
                    c.sw(at(c, S2, CURRENT_PROCESS), c.g[V0][0]);
                    call_ptr(c, 0x0030_77f8, 0x0030_9490)?;
                    pc = 0x0030_9490;
                }
                0x0030_9490 => {
                    let more = c.g[S0][0] != 0;
                    c.g[V0][0] = c.g[S0][0];
                    if more {
                        if back_edge_unwinds(c, 0x0030_9480) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0030_9480;
                    } else {
                        c.cycles += 1;
                        c.sw(at(c, S2, CURRENT_PROCESS), 0);
                        pc = 0x0030_949c;
                    }
                }
                0x0030_949c => {
                    if c.g[S7][0] != 0 {
                        crate::custom_script_queue::finish(c, r(c, S2), r(c, SP));
                        c.g[S7][0] = 0;
                    }
                    if let Some((function, owner)) = crate::custom_script_materials::next_expired(c) {
                        c.g[A0][0] = sx(owner);
                        call_ptr(c, function, 0x0030_949c)?;
                        pc = 0x0030_949c;
                        continue;
                    }
                    if let Some(node) = crate::custom_cracked_glass::next_ready(c) {
                        c.g[A0][0] = sx(node);
                        c.g[A1][0] = 1;
                        call_ptr(c, 0x0034_fd00, 0x0030_949c)?;
                        pc = 0x0030_949c;
                        continue;
                    }
                    if let Some((manager, index)) = crate::custom_movies::next_frame(c) {
                        c.g[A0][0] = sx(manager);
                        c.g[A1][0] = sx(index);
                        call_ptr(c, 0x0036_9098, 0x0030_949c)?;
                        pc = 0x0030_949c;
                        continue;
                    }
                    if crate::custom_script_queue::prepare_next(c, r(c, S2), r(c, SP)) {
                        c.g[S7][0] = 1;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x0030_9c88, 0x0030_949c)?;
                        pc = 0x0030_949c;
                        continue;
                    }
                    c.cycles += 11;
                    frame.restore(c, S0, 0x210);
                    frame.restore(c, S1, 0x200);
                    frame.restore(c, S2, 0x1f0);
                    frame.restore(c, S3, 0x1e0);
                    frame.restore(c, S4, 0x1d0);
                    frame.restore(c, S5, 0x1c0);
                    frame.restore(c, S6, 0x1b0);
                    frame.restore(c, S7, 0x1a0);
                    frame.restore_ra(c, 0x190);
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
