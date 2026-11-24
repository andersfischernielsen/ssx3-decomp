//! Staging module oc544 (wave l536.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc544`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0027_7980,
        script_list_start_transition,
        "ScriptList_StartTransition",
    ),
    (0x0027_7c08, script_list_set_paused, "ScriptList_SetPaused"),
    (
        0x0027_7de8,
        script_list_get_movie_info,
        "ScriptList_GetMovieInfo",
    ),
];

/// `ScriptList_GetMovieInfo(list, key)` (0x277de8): the movie record for the
/// entry `key` (its type word `*key` indexes 28-byte records). It calls the BE
/// library (`cBE_getInterface_Fv(cBE_getBE(), 4)`, whose result is not used),
/// copies the 0x288-byte table at 0x535610 onto the stack, and returns
/// `&D_0048247C[*key]`, or `&D_004823DC[*key]` when the table's first word,
/// masked with 0x300000, equals 0x200000.
pub fn script_list_get_movie_info(c: &mut Cpu) {
    const BE_GET_INSTANCE: u32 = 0x0014_dc80;
    const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    const TABLE: u32 = 0x0053_5610;
    const TABLE_LEN: u32 = 0x280;
    const RECORDS_A: u32 = 0x0048_247c;
    const RECORDS_B: u32 = 0x0048_23dc;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, S0, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, BE_GET_INSTANCE, 0x0027_7dfc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, BE_GET_INTERFACE, 0x0027_7e08)?;
        // The table is 8-byte aligned (0x535610 & 7 == 0), so the generated
        // unaligned copy path (ldl/ldr) never runs: only the aligned loop does.
        c.g[V1][0] = sx(TABLE);
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 1;
        c.g[V0][0] = sx(TABLE.wrapping_add(TABLE_LEN));
        // The copy: 0x20 bytes per pass from the table to the stack buffer.
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[A1][0] = Addr(src).ld(c);
            c.g[A2][0] = Addr(src.wrapping_add(8)).ld(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let words = [c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]];
            for (i, w) in words.into_iter().enumerate() {
                Addr(dst.wrapping_add(8 * i as u32)).sd(c, w);
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_7e78);
            }
        }
        // The last 8 bytes (0x535890) go to sp+0x280.
        c.cycles += 10;
        let tail = Addr(c.g[V1][0] as u32).ld(c);
        c.g[V0][0] = tail;
        Addr(c.g[A0][0] as u32).sd(c, tail);
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 0x0030_0000;
        c.g[A0][0] = 0x0020_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        let t = c.g[V1][0] != c.g[A0][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.cycles += if t { 4 } else { 5 };
        c.g[A0][0] = 0x1c;
        c.mult(0, V1, V1, A0);
        c.g[V0][0] = sx(if t { RECORDS_A } else { RECORDS_B });
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S0, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_SetPaused(list, on, hard)` (0x277c08), the body of
/// `tNISList::DoPause`. In a playing list (state 3) a hard request sets the
/// paused flag (+0xa8) and a soft one counts the pauses (+0xc4). When nothing
/// else holds the list, the current entry is paused or resumed (a movie
/// through the movie player, a script through the engine), along with the
/// cutscene transition when the list has one (+4).
pub fn script_list_set_paused(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let list = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(list).off(0xa4).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            set_paused_exit_s0(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            // Hard: the paused flag must change, and no count may be pending.
            c.g[V0][0] = Addr(list).off(0xa8).lw(c);
            c.cycles += 2;
            if c.g[V0][0] == c.g[A1][0] {
                set_paused_exit_s0(c, frame);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(list).off(0xc4).lw(c);
            let on = c.g[A1][0];
            Addr(list).off(0xa8).sw(c, on);
        } else {
            // Soft: count the pauses; a nested pause needs no action.
            c.cycles += 2;
            let on = c.g[A1][0] != 0;
            c.g[V1][0] = Addr(list).off(0xc4).lw(c);
            if on {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                let nested = c.g[V1][0] != 0;
                let count = c.g[V0][0];
                Addr(list).off(0xc4).sw(c, count);
                if nested {
                    set_paused_exit_dc4(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(list).off(0xa8).lw(c);
            } else {
                c.cycles += 2;
                let none_left = (c.g[V1][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                if none_left {
                    set_paused_exit_dc4(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                let still = c.g[V0][0] != 0;
                let count = c.g[V0][0];
                Addr(list).off(0xc4).sw(c, count);
                if still {
                    set_paused_exit_dc4(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(list).off(0xa8).lw(c);
            }
        }
        // The paused flag (soft) or the count (hard) must be clear to go on.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            set_paused_exit_s0(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        let on = c.g[A1][0] != 0;
        c.g[A0][0] = sx(list.wrapping_add(0xc));
        if on {
            set_paused_on(c, frame, list)?;
        } else {
            set_paused_off(c, frame, list)?;
        }
        Ok(())
    });
}

/// The pause branch of [`script_list_set_paused`] (`on` != 0). Always ends
/// with the epilogue (or the one-cycle-longer `dc4` exit).
fn set_paused_on(c: &mut Cpu, frame: Frame, list: u32) -> Flow {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    const PAUSE_PROCESS: Callee = Callee {
        addr: 0x0028_26d8,
        f: ee::s07::f_002826d8,
    };
    const MOVIE_SET_PAUSED: Callee = Callee {
        addr: 0x0028_3b78,
        f: ee::s07::f_00283b78,
    };
    c.cycles += 8;
    c.g[A1][0] = 0;
    CMD_QUEUE_AT.call(c, 0x0027_7c90)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    IS_MOVIE_ENTRY.call(c, 0x0027_7ca0)?;
    let movie = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(list).lw(c);
    if movie {
        c.cycles += 4;
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
        MOVIE_SET_PAUSED.call(c, 0x0027_7ce0)?;
        c.g[V0][0] = Addr(list).off(4).lw(c);
    } else {
        c.cycles += 6;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        PROCESS_STATE.call(c, 0x0027_7cb0)?;
        let state_less_3 = sx((c.g[V0][0] as u32).wrapping_sub(3));
        c.g[V0][0] = (state_less_3 < 3) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(list).off(4).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = Addr(list).lw(c);
            PAUSE_PROCESS.call(c, 0x0027_7ccc)?;
            c.g[V0][0] = Addr(list).off(4).lw(c);
        }
    }
    // The cutscene handler (+4) pauses its transition too.
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    if c.g[V0][0] == 0 {
        set_paused_exit_dc4(c, frame);
        return Ok(());
    }
    c.cycles += 12;
    c.g[A1][0] = 1;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x64)).lw(c);
    call_ptr(c, 0x002e_a8b8, 0x0027_7cfc)?;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7d0c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x44).lw(c);
    let idle = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x20);
    if idle {
        set_paused_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = 1;
    call_ptr(c, 0x002e_4638, 0x0027_7d24)?;
    frame.restore(c, S0, 0x20);
    set_paused_epilogue(c, frame);
    Ok(())
}

/// The resume branch of [`script_list_set_paused`] (`on` == 0).
fn set_paused_off(c: &mut Cpu, frame: Frame, list: u32) -> Flow {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    const RESUME_PROCESS: Callee = Callee {
        addr: 0x0028_2720,
        f: ee::s07::f_00282720,
    };
    const MOVIE_SET_PAUSED: Callee = Callee {
        addr: 0x0028_3b78,
        f: ee::s07::f_00283b78,
    };
    c.cycles += 8;
    c.g[A1][0] = 0;
    CMD_QUEUE_AT.call(c, 0x0027_7d34)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    IS_MOVIE_ENTRY.call(c, 0x0027_7d44)?;
    let movie = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(list).lw(c);
    if movie {
        c.cycles += 4;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
        MOVIE_SET_PAUSED.call(c, 0x0027_7d80)?;
        c.g[V0][0] = Addr(list).off(4).lw(c);
    } else {
        c.cycles += 5;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        PROCESS_STATE.call(c, 0x0027_7d54)?;
        c.g[V1][0] = 5;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = Addr(list).off(4).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = Addr(list).lw(c);
            RESUME_PROCESS.call(c, 0x0027_7d6c)?;
            c.g[V0][0] = Addr(list).off(4).lw(c);
        }
    }
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    if c.g[V0][0] == 0 {
        set_paused_exit_dc4(c, frame);
        return Ok(());
    }
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x64)).lw(c);
    call_ptr(c, 0x002e_a8b8, 0x0027_7d9c)?;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7dac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x44).lw(c);
    let idle = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x20);
    if idle {
        set_paused_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    c.g[A1][0] = 0;
    call_ptr(c, 0x002e_4638, 0x0027_7dc4)?;
    set_paused_exit_dc4(c, frame);
    Ok(())
}

/// `break 'b_277dc8` after restoring s0 (the early exits of SetPaused).
fn set_paused_exit_s0(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x20);
    set_paused_epilogue(c, frame);
}

/// `break 'b_277dc4`: one more cycle, then the s0 exit.
fn set_paused_exit_dc4(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    set_paused_exit_s0(c, frame);
}

/// The epilogue: restore s1 and ra, pop the frame, return.
fn set_paused_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `ScriptList_StartTransition(list)` (0x277980): starts the transition into
/// the list's next entry (a movie's record, or a script's process info), or
/// reports its progress. Returns the list's transition state (+0xb0) or 0/1
/// while a stop or a wait is pending.
pub fn script_list_start_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let list = c.g[S1][0] as u32;
        c.g[V0][0] = Addr(list).off(0xb0).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = sx(list.wrapping_add(0xc));
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = Addr(list).off(0xac).lw(c);
            let waiting = c.g[V0][0] == 0;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if !waiting {
                // A resume is pending: stop the transition manager.
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                call_ptr(c, 0x0023_0698, 0x0027_79c0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002e_4578, 0x0027_79c8)?;
                c.g[V0][0] = 0;
                start_transition_epilogue(c, frame);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            start_transition_epilogue(c, frame);
            return Ok(());
        }
        if start_transition_main(c)? {
            c.cycles += 1;
            c.g[V0][0] = Addr(list).off(0xb0).lw(c);
        }
        start_transition_epilogue(c, frame);
        Ok(())
    });
}

/// The idle path: the next entry of the queue. `Ok(true)` ends on the normal
/// tail (`break 'b_277be8`), `Ok(false)` returns at once with v0 set.
fn start_transition_main(c: &mut Cpu) -> Result<bool, Unwound> {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    c.cycles += 8;
    c.g[A1][0] = 0;
    CMD_QUEUE_AT.call(c, 0x0027_79e0)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    IS_MOVIE_ENTRY.call(c, 0x0027_79f0)?;
    let movie = c.g[V0][0] != 0;
    c.g[A1][0] = c.g[S0][0];
    if movie {
        start_transition_movie(c)
    } else {
        start_transition_script(c)
    }
}

/// A movie entry: its record (from `ScriptList_GetMovieInfo`) gives the
/// duration (+0xc, a float), the transition kind (+4) and the target (+8).
fn start_transition_movie(c: &mut Cpu) -> Result<bool, Unwound> {
    const GET_MOVIE_INFO: Callee = Callee {
        addr: 0x0027_7de8,
        f: ee::s07::f_00277de8,
    };
    const TRANSITION_TARGET: Callee = Callee {
        addr: 0x0027_a4a0,
        f: ee::s07::f_0027a4a0,
    };
    c.cycles += 11;
    c.g[A0][0] = c.g[S1][0];
    GET_MOVIE_INFO.call(c, 0x0027_7b48)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7b5c)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x44).lw(c);
    if c.g[V0][0] == 0 {
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002e_4578, 0x0027_7b74)?;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
    }
    // The duration must be positive (`0.0 < f1`, latched in the FCC bit).
    c.cycles += 5;
    c.f[0] = 0;
    let zero = FpuF32(c.f[0]);
    let duration = FpuF32(c.f[1]);
    let positive = crate::abi::c_lt_s(c, zero, duration);
    c.g[V0][0] = 7;
    if !positive {
        return Ok(true);
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
    let captured = c.g[A1][0] == c.g[V0][0];
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
    if captured {
        c.cycles += 10;
        c.g[A1][0] = 7;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
        TRANSITION_TARGET.call(c, 0x0027_7bcc)?;
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        call_ptr(c, 0x002e_4540, 0x0027_7be4)?;
        Addr(c.g[S1][0] as u32).off(0xb8).sw(c, 0);
        return Ok(true);
    }
    c.cycles += 10;
    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
    TRANSITION_TARGET.call(c, 0x0027_7ba4)?;
    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 2;
    call_ptr(c, 0x002e_44f0, 0x0027_7bb8)?;
    Addr(c.g[S1][0] as u32).off(0xb8).sw(c, 0);
    Ok(true)
}

/// A script entry: its process (`ScriptEngine_GetProcessInSeq`) gives the
/// transition info (`ScriptData_GetInfo`): three durations (+0x12, +0x14,
/// +0x16), a kind (+0x10) and a target (+0x11).
fn start_transition_script(c: &mut Cpu) -> Result<bool, Unwound> {
    const PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    const SCRIPT_DATA_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::f_0027c070,
    };
    const TRANSITION_TARGET: Callee = Callee {
        addr: 0x0027_a4a0,
        f: ee::s07::f_0027a4a0,
    };
    c.cycles += 14;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
    PROCESS_IN_SEQ.call(c, 0x0027_7a04)?;
    c.g[A0][0] = c.g[V0][0];
    SCRIPT_DATA_INFO.call(c, 0x0027_7a0c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7a20)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x44).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002e_4578, 0x0027_7a38)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
    }
    // The three durations: nothing to start when they sum to 0 or less.
    c.cycles += 6;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lh(c);
    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x16).lh(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = 7;
    if none {
        return Ok(true);
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lb(c);
    let captured = c.g[A1][0] == c.g[V0][0];
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
    if captured {
        // A captured transition (kind 7).
        c.cycles += 14;
        c.g[A1][0] = 7;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x11).lb(c);
        TRANSITION_TARGET.call(c, 0x0027_7afc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47f8));
        c.g[A2][0] = c.g[V0][0];
        c.f[12] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
        call_ptr(c, 0x002e_4540, 0x0027_7b24)?;
        Addr(c.g[S1][0] as u32).off(0xb8).sw(c, 0);
    } else {
        // A transition to the first target; none means it cannot start.
        c.cycles += 6;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x11).lb(c);
        TRANSITION_TARGET.call(c, 0x0027_7a6c)?;
        c.g[S3][0] = c.g[V0][0];
        if c.g[S3][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return Ok(false);
        }
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lb(c);
        c.cycles += 28;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x11).lb(c);
        TRANSITION_TARGET.call(c, 0x0027_7a8c)?;
        c.g[A3][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lh(c);
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x16).lh(c);
        c.f[12] = FpuF32::from_i32(c.g[A3][0] as u32 as i32).0;
        c.f[13] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47fc));
        c.f[14] = FpuF32::from_i32(c.g[A1][0] as u32 as i32).0;
        c.g[A3][0] = c.g[V0][0];
        c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
        c.g[A1][0] = 2;
        c.f[13] = (FpuF32(c.f[13]) * FpuF32(c.f[0])).0;
        c.g[T0][0] = 0;
        c.f[14] = (FpuF32(c.f[14]) * FpuF32(c.f[0])).0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x002e_4370, 0x0027_7adc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lh(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let frames = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0xb8).sw(c, frames);
    }
    // The started transition's frames: the list then waits for it.
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0xb8).lw(c);
    let no_frames = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = 1;
    if no_frames {
        return Ok(true);
    }
    c.cycles += 3;
    Addr(c.g[S1][0] as u32).off(0xac).sw(c, 0);
    let one = c.g[V0][0];
    Addr(c.g[S1][0] as u32).off(0xb0).sw(c, one);
    Ok(true)
}

/// The epilogue of [`script_list_start_transition`].
fn start_transition_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the callee fakes act only in machines that hold it.
    pub(crate) const OC544_GATE: u32 = DATA + 0xe_0000;
    pub(crate) const OC544_MAGIC: u32 = 0x0544_0001;
    /// The fake's answers: the target (+0x10 of the gate) and the process state (+0x14).
    pub(crate) const OC544_TARGET_AT: u32 = OC544_GATE + 0x10;
    pub(crate) const OC544_STATE_AT: u32 = OC544_GATE + 0x14;
    /// The table `ScriptList_GetMovieInfo` copies (game RAM at 0x535610).
    pub(crate) const OC544_TABLE: u32 = 0x0053_5610;
    const OC544_WORLD: u32 = DATA + 0x1000;
    const OC544_ENGINE: u32 = DATA + 0x2000;
    const OC544_GAME: u32 = DATA + 0x3000;
    const OC544_TMAN: u32 = DATA + 0x4000;
    const OC544_CUTMAN: u32 = DATA + 0x5000;
    const OC544_LIST: u32 = DATA + 0x6000;
    const OC544_INFO: u32 = DATA + 0x7000;
    const OC544_PROC: u32 = DATA + 0x8000;
    const OC544_BE: u32 = DATA + 0x9000;
    const OC544_KEY: u32 = DATA + 0xa000;
    const OC544_TARGET: u32 = DATA + 0xb000;

    /// Gated fake: records the call (`fakes::fake`), then sets the result.
    macro_rules! oc544_gated {
        ($fake:ident, $prev:ident, $gen:path, $target:expr, |$c:ident| $body:block) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake($c: &mut Cpu) {
                if $c.bus.read32(OC544_GATE) != OC544_MAGIC {
                    return fakes::fall_back($c, &$prev, $gen);
                }
                let ra = $c.g[RA][0] as u32;
                fakes::fake($c, $target, ra);
                $body
            }
        };
    }

    oc544_gated!(
        oc544_fake_get_be,
        OC544_PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        |c| {
            c.g[V0][0] = sx(OC544_BE);
        }
    );
    oc544_gated!(
        oc544_fake_get_iface,
        OC544_PREV_GET_IFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_proc_in_seq,
        OC544_PREV_PROC_IN_SEQ,
        ee::s07::f_00282bf0__generated,
        0x0028_2bf0,
        |c| {
            c.g[V0][0] = sx(OC544_PROC);
        }
    );
    oc544_gated!(
        oc544_fake_script_info,
        OC544_PREV_SCRIPT_INFO,
        ee::s07::f_0027c070__generated,
        0x0027_c070,
        |c| {
            c.g[V0][0] = sx(OC544_INFO);
        }
    );
    oc544_gated!(
        oc544_fake_transition_man,
        OC544_PREV_TRANSITION_MAN,
        ee::s06::f_00230698__generated,
        0x0023_0698,
        |c| {
            c.g[V0][0] = sx(OC544_TMAN);
        }
    );
    oc544_gated!(
        oc544_fake_tman_stop,
        OC544_PREV_TMAN_STOP,
        ee::s09::f_002e4578__generated,
        0x002e_4578,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_target,
        OC544_PREV_TARGET,
        ee::s07::f_0027a4a0__generated,
        0x0027_a4a0,
        |c| {
            c.g[V0][0] = sx(c.bus.read32(OC544_TARGET_AT));
        }
    );
    oc544_gated!(
        oc544_fake_captured,
        OC544_PREV_CAPTURED,
        ee::s09::f_002e4540__generated,
        0x002e_4540,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_start_out,
        OC544_PREV_START_OUT,
        ee::s09::f_002e44f0__generated,
        0x002e_44f0,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_process_state,
        OC544_PREV_PROCESS_STATE,
        ee::s07::f_00282798__generated,
        0x0028_2798,
        |c| {
            c.g[V0][0] = sx(c.bus.read32(OC544_STATE_AT));
        }
    );
    oc544_gated!(
        oc544_fake_pause_process,
        OC544_PREV_PAUSE_PROCESS,
        ee::s07::f_002826d8__generated,
        0x0028_26d8,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_resume_process,
        OC544_PREV_RESUME_PROCESS,
        ee::s07::f_00282720__generated,
        0x0028_2720,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_movie_paused,
        OC544_PREV_MOVIE_PAUSED,
        ee::s07::f_00283b78__generated,
        0x0028_3b78,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_cutscene_pause,
        OC544_PREV_CUTSCENE_PAUSE,
        ee::s09::f_002ea8b8__generated,
        0x002e_a8b8,
        |c| {}
    );
    oc544_gated!(
        oc544_fake_pause_transition,
        OC544_PREV_PAUSE_TRANSITION,
        ee::s09::f_002e4638__generated,
        0x002e_4638,
        |c| {}
    );

    /// Puts every fake in its override slot (once per process).
    fn oc544_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc544_fake_get_be,
            &OC544_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc544_fake_get_iface,
            &OC544_PREV_GET_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282BF0),
            oc544_fake_proc_in_seq,
            &OC544_PREV_PROC_IN_SEQ,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027C070),
            oc544_fake_script_info,
            &OC544_PREV_SCRIPT_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            oc544_fake_transition_man,
            &OC544_PREV_TRANSITION_MAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4578),
            oc544_fake_tman_stop,
            &OC544_PREV_TMAN_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027A4A0),
            oc544_fake_target,
            &OC544_PREV_TARGET,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4540),
            oc544_fake_captured,
            &OC544_PREV_CAPTURED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E44F0),
            oc544_fake_start_out,
            &OC544_PREV_START_OUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282798),
            oc544_fake_process_state,
            &OC544_PREV_PROCESS_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002826D8),
            oc544_fake_pause_process,
            &OC544_PREV_PAUSE_PROCESS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282720),
            oc544_fake_resume_process,
            &OC544_PREV_RESUME_PROCESS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283B78),
            oc544_fake_movie_paused,
            &OC544_PREV_MOVIE_PAUSED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EA8B8),
            oc544_fake_cutscene_pause,
            &OC544_PREV_CUTSCENE_PAUSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4638),
            oc544_fake_pause_transition,
            &OC544_PREV_PAUSE_TRANSITION,
        );
    }

    /// The gate, the world (gp-0x848), the engine (gp-0x84c), the game and the
    /// transition manager, and the two transition floats (gp-0x47fc, -0x47f8).
    fn oc544_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC544_GATE, OC544_MAGIC);
        let gp = c.g[GP][0] as u32;
        c.bus.write32(gp.wrapping_sub(0x848), OC544_WORLD);
        c.bus.write32(gp.wrapping_sub(0x84c), OC544_ENGINE);
        c.bus.write32(gp.wrapping_sub(0x47fc), 0x3c88_8889);
        c.bus.write32(gp.wrapping_sub(0x47f8), 0x3c88_8889);
        c.bus.write32(OC544_WORLD + 0x84, OC544_GAME);
        c.bus.write32(OC544_GAME + 0x64, OC544_CUTMAN);
        c.bus.write32(OC544_TMAN + 0x44, r.next_u32() % 2);
    }

    /// The command queue of a list at `list` (ring of five 28-byte entries
    /// whose type words are all movies (>= 29) or all scripts).
    fn oc544_queue(c: &mut Cpu, r: &mut Rng, list: u32, movie: bool) {
        let queue = list + 0xc;
        c.bus.write32(queue + 0x8c, r.next_u32() % 6);
        for k in 0..5 {
            c.bus
                .write32(queue + k * 28, if movie { 29 + k } else { k % 29 });
        }
    }

    /// The copied table (0x535610) with random words; its first word's flags
    /// (masked 0x300000) pick the record table.
    fn oc544_table(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for i in 0..(0x288 / 4) {
            c.bus.write32(OC544_TABLE + 4 * i, r.next_u32());
        }
        let w0 = if seed % 2 == 0 {
            (r.next_u32() & !0x0030_0000) | 0x0020_0000
        } else {
            r.next_u32() | 0x0030_0000
        };
        c.bus.write32(OC544_TABLE, w0);
    }

    #[test]
    fn script_list_get_movie_info_matches() {
        oc544_install();
        check(
            ee::s07::f_00277de8__generated,
            script_list_get_movie_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277de8, 0);
                c.bus.write32(OC544_GATE, OC544_MAGIC);
                c.g[A1][0] = sx(OC544_KEY);
                c.bus.write32(OC544_KEY, r.next_u32() % 40);
                oc544_table(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn script_list_set_paused_matches() {
        oc544_install();
        check(
            ee::s07::f_00277c08__generated,
            script_list_set_paused,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277c08, 0);
                oc544_world(c, &mut r);
                let list = OC544_LIST;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = ((seed / 2) % 2) as u64;
                c.g[A2][0] = (seed % 3 == 0) as u64;
                c.bus.write32(list, OC544_ENGINE);
                c.bus.write32(list + 0x4, (seed % 5 < 2) as u32);
                c.bus
                    .write32(list + 0xa4, if seed % 6 == 0 { 2 } else { 3 });
                c.bus.write32(list + 0xa8, (seed % 4 == 1) as u32);
                c.bus.write32(list + 0xc4, r.next_u32() % 3);
                oc544_queue(c, &mut r, list, seed % 3 == 1);
                c.bus.write32(OC544_STATE_AT, seed % 7);
            },
        );
    }

    #[test]
    fn script_list_start_transition_matches() {
        oc544_install();
        check(
            ee::s07::f_00277980__generated,
            script_list_start_transition,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277980, 0);
                oc544_world(c, &mut r);
                let list = OC544_LIST;
                c.g[A0][0] = sx(list);
                c.bus.write32(list, OC544_ENGINE);
                // Idle (the queue's next entry), waiting, or a pending stop.
                c.bus.write32(list + 0xb0, (seed % 2) as u32);
                c.bus
                    .write32(list + 0xac, if seed % 4 == 1 { 0 } else { 1 });
                c.bus.write32(list + 0xb8, r.next_u32());
                let movie = seed % 3 == 0;
                oc544_queue(c, &mut r, list, movie);
                // Movie records (both tables) for the five entries' keys.
                for key in 29..34 {
                    for base in [0x0048_247c_u32, 0x0048_23dc] {
                        let rec = base + key * 28;
                        c.bus.write32(rec + 4, if seed % 2 == 0 { 7 } else { 3 });
                        c.bus.write32(rec + 8, r.next_u32());
                        let duration = if seed % 4 == 0 {
                            -1.0_f32
                        } else {
                            0.25 + (seed % 3) as f32
                        };
                        c.bus.write32(rec + 0xc, duration.to_bits());
                    }
                }
                oc544_table(c, &mut r, seed);
                // Script info: kind 7 or not, durations that sum to <= 0 or not.
                c.bus
                    .write8(OC544_INFO + 0x10, if seed % 5 == 0 { 7 } else { 2 });
                c.bus.write8(OC544_INFO + 0x11, 3);
                let h12: i16 = if seed % 6 == 0 { -3 } else { 2 };
                c.bus.write16(OC544_INFO + 0x12, h12 as u16);
                c.bus.write16(OC544_INFO + 0x14, 1);
                c.bus
                    .write16(OC544_INFO + 0x16, if seed % 3 == 0 { 3 } else { 0 });
                // The target: none (the script cannot start) or a pointer.
                c.bus.write32(
                    OC544_TARGET_AT,
                    if seed % 7 == 0 { 0 } else { OC544_TARGET },
                );
            },
        );
    }
}
