//! Staging module oc1333: hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1333`.
//!
//! This lane ports `cBXString_FormatV` (the `%`-format scanner that sizes the
//! result buffer), `EAC_GetUnpackedSize` (the RefPack/EAC header probe) and
//! nine small EA-library utilities (`EA_FatalError`, `EA_DebugMessage`,
//! `EA_IsValidPointer`, `EA_MemMove`, `EA_AddShutdownHandler`, `EA_Crc16`,
//! `EA_RunFatalHandlers`, `EA_Abort`, `EA_AssertNotBusy`).
//!
//! Notes on the brief's call claims, checked against the generated bodies:
//! `EAC_GetUnpackedSize`, `EA_AddShutdownHandler` and `EA_Crc16` make no
//! calls (their `f_003b49b0`/`f_003e4af0`/`f_003e6328` neighbours are other
//! functions in the same unit, not callees). `EA_MemMove` calls
//! `EA_MemCopy` (0x3e6574), not `EA_MemZero`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, returned, sp, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
///
/// `EA_Abort` (0x3e4a10) is ported but has no entry: its `break` arm always
/// traps the machine, so no differential test can run it to completion.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_91c0, cbx_string_format_v, "cBXString_FormatV"),
    (0x003b_4818, eac_get_unpacked_size, "EAC_GetUnpackedSize"),
    (0x003e_60e8, ea_fatal_error, "EA_FatalError"),
    (0x003e_6070, ea_debug_message, "EA_DebugMessage"),
    (0x003e_6240, ea_is_valid_pointer, "EA_IsValidPointer"),
    (0x003e_6188, ea_mem_move, "EA_MemMove"),
    (
        0x003e_4a30,
        ea_add_shutdown_handler,
        "EA_AddShutdownHandler",
    ),
    (0x003e_62d0, ea_crc16, "EA_Crc16"),
    (0x003e_49b8, ea_run_fatal_handlers, "EA_RunFatalHandlers"),
    (0x003e_4a10, ea_abort, "EA_Abort"),
    (0x003e_62b0, ea_assert_not_busy, "EA_AssertNotBusy"),
];

pub(crate) mod callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub(crate) use crate::game::bx::bxstring::callee::CBX_STRING_GET_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::misc::callee::STRTOL;
    pub(crate) use crate::game::render::particle::callee::STRLEN;

    /// `void cBXString_ReleaseBuffer(cBXString* self, int n)` (0x318a88).
    pub(crate) const CBX_STRING_RELEASE_BUFFER: Callee = Callee {
        addr: 0x0031_8a88,
        f: ee::s10::cBXString_ReleaseBuffer,
    };

    /// `void EA_RunFatalHandlers(void)` (0x3e49b8).
    pub(crate) const EA_RUN_FATAL_HANDLERS: Callee = Callee {
        addr: 0x003e_49b8,
        f: ee::s14::EA_RunFatalHandlers,
    };

    /// `void EA_Abort(void)` (0x3e4a10).
    pub(crate) const EA_ABORT: Callee = Callee {
        addr: 0x003e_4a10,
        f: ee::s14::EA_Abort,
    };

    /// `void EA_MemCopy(void* dst, const void* src, size_t n)` (0x3e6574).
    pub(crate) const EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::f_003e6574,
    };

    /// `void Debug_Printf(int ch, const char* fmt, ...)` (0x3e5d30).
    pub(crate) const DEBUG_PRINTF: Callee = Callee {
        addr: 0x003e_5d30,
        f: ee::s14::Debug_Printf,
    };
}

// ---------------------------------------------------------------------------
// cBXString_FormatV.

/// Format-specifier classes of the first jump table (base 0x48d860, indexed
/// by the character minus `'F'`): only the length modifiers skip to the next
/// character, everything else enters the `c`/`C`/`s`/`S` chain.
fn format_v_table1_skip(spec: u32) -> bool {
    matches!(spec, 0 | 6 | 8 | 0x22 | 0x26)
}

/// Format-specifier classes of the second jump table (base 0x48d900, indexed
/// by the character minus `'G'`): float (`e`/`f`/`g`/`G`), integer
/// (`d`/`i`/`u`/`x`/`X`/`o`/`p`), count (`n`), or nothing.
#[derive(Clone, Copy, PartialEq)]
enum FormatVItem {
    Float,
    Int,
    Count,
    None,
}

fn format_v_table2_class(spec: u32) -> FormatVItem {
    match spec {
        0 | 30 | 31 | 32 => FormatVItem::Float,
        17 | 29 | 34 | 40 | 41 | 46 | 49 => FormatVItem::Int,
        39 => FormatVItem::Count,
        _ => FormatVItem::None,
    }
}

/// `void cBXString_FormatV(cBXString* self, const char* fmt, va_list ap)`
/// (0x3191c0): scan `fmt`, sizing the result (`maxLen`, grown per conversion
/// and flag), then `vsprintf` into a buffer of that size. The two jump
/// tables above dispatch the conversion characters; length modifiers
/// (`h`/`l`/`F`/`N`/`L`) only skip a character. `'*'` takes the width (or
/// precision) from the argument list, digits from the format otherwise.
pub fn cbx_string_format_v(c: &mut Cpu) {
    guest(c, |c| {
        const ENTRY: u32 = 0x0031_91c0;
        const SCAN_HEAD: u32 = 0x0031_9210;
        const FLAG_HEAD: u32 = 0x0031_9258;
        const END: u32 = 0x0031_94a8;
        const FRAME: u32 = 0xa0;
        // The frame stays open across unwinds, so the resume heads reuse it
        // instead of opening it again; the loop state lives in the registers
        // (S0/S1/S4) and the open frame.
        let frame = Frame { size: FRAME };
        match c.pc {
            ENTRY => {
                c.cycles += 18;
                Frame::open(c, FRAME);
                frame.save(c, S0, 0x90);
                frame.save(c, S1, 0x80);
                frame.save(c, S4, 0x50);
                frame.save(c, S5, 0x40);
                frame.save(c, S6, 0x30);
                frame.save(c, S7, 0x20);
                frame.save(c, S2, 0x70);
                frame.save(c, S3, 0x60);
                c.g[S4][0] = 0x1;
                c.g[S5][0] = c.g[A0][0];
                c.g[S6][0] = c.g[A1][0];
                c.g[S7][0] = c.g[A2][0];
                c.g[S1][0] = c.g[S7][0];
                frame.save_ra(c, 0x10);
                c.g[V0][0] = Addr(r32(c, S6)).lb(c);
                let empty = c.g[V0][0] == 0;
                c.g[S0][0] = c.g[S6][0];
                if !empty {
                    // 0x319208 runs once on entry: its only incoming edge is
                    // the entry fall-through, and every loop-back re-enters
                    // at 0x319210.
                    c.cycles += 2;
                    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
                    format_v_scan(c)?;
                }
                format_v_tail(c, &frame)
            }
            // 0x319210 resume: re-executes the +3 head test; V1 already holds
            // [S0], loaded (chargeless) by the back edge before the unwind.
            SCAN_HEAD => {
                format_v_scan(c)?;
                format_v_tail(c, &frame)
            }
            // 0x319258 resume: re-executes the flag test with the flag-run
            // registers intact, then rejoins the scan loop at its back edge.
            FLAG_HEAD => {
                format_v_flags(c)?;
                format_v_conversion(c)?;
                if !format_v_scan_back_edge(c)? {
                    format_v_scan(c)?;
                }
                format_v_tail(c, &frame)
            }
            pc => {
                c.jump_out(pc, ENTRY, END);
                Ok(())
            }
        }
    })
}

/// The scan loop of [`cbx_string_format_v`] from the +3 head test (0x319210,
/// post-hoist shape): V1 already holds `[S0]` on entry. Each iteration
/// dispatches one plain character or conversion; the back edge advances and
/// either exits or unwinds at the scan head.
fn format_v_scan(c: &mut Cpu) -> Flow {
    loop {
        // 0x319210: plain character or conversion.
        c.cycles += 3;
        c.g[V0][0] = 0x25;
        let plain = c.g[V1][0] != c.g[V0][0];
        if plain {
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
        } else {
            // 0x31921c: after '%': Verbatim '%' or a conversion.
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = Addr(r32(c, S0)).lb(c);
            let escaped = c.g[V0][0] != c.g[V1][0];
            c.g[S3][0] = 0;
            if !escaped {
                // 0x31922c: '%%'.
                c.cycles += 2;
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
            } else {
                // 0x319234: a conversion starts a flag run.
                c.cycles += 2;
                let end = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                if end {
                    format_v_conversion(c)?;
                } else {
                    // 0x31923c: flag characters.
                    c.cycles += 7;
                    c.g[T1][0] = 0x23;
                    c.g[T0][0] = 0x2a;
                    c.g[A3][0] = 0x2d;
                    c.g[A2][0] = 0x2b;
                    c.g[A1][0] = 0x30;
                    c.g[V1][0] = 0x20;
                    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                    format_v_flags(c)?;
                    format_v_conversion(c)?;
                }
            }
        }
        // 0x319444: scan-loop back edge.
        if format_v_scan_back_edge(c)? {
            return Ok(());
        }
    }
}

/// The scan-loop back edge of [`cbx_string_format_v`] (0x319444): advance
/// S0, reload V0 (and V1 when continuing, chargeless, as the generated
/// block does). Returns true on NUL (scan done); otherwise unwinds at the
/// scan head (0x319210) when an event asks for it.
fn format_v_scan_back_edge(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 4;
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
    if c.g[V0][0] == 0 {
        return Ok(true);
    }
    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    if back_edge_unwinds(c, 0x0031_9210) {
        return Err(Unwound);
    }
    Ok(false)
}

/// The size/format/release tail of [`cbx_string_format_v`] (0x319454):
/// size the result, `vsprintf` into it, release the buffer, restore and
/// close the frame.
fn format_v_tail(c: &mut Cpu, frame: &Frame) -> Flow {
    const GET_BUFFER_RET: u32 = 0x0031_9460;
    const VSPRINTF_RET: u32 = 0x0031_9470;
    const RELEASE_BUFFER_RET: u32 = 0x0031_947c;
    // 0x319454: size, format, release.
    c.cycles += 21;
    c.g[A1][0] = c.g[S4][0];
    c.g[RA][0] = sx(GET_BUFFER_RET);
    c.g[A0][0] = c.g[S5][0];
    callee::CBX_STRING_GET_BUFFER.call(c, GET_BUFFER_RET)?;
    c.g[A0][0] = Addr(r32(c, S5)).lw(c);
    c.g[A1][0] = c.g[S6][0];
    c.g[RA][0] = sx(VSPRINTF_RET);
    c.g[A2][0] = c.g[S7][0];
    call_ptr(c, callee::VSPRINTF, VSPRINTF_RET)?;
    c.g[A0][0] = c.g[S5][0];
    c.g[RA][0] = sx(RELEASE_BUFFER_RET);
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    callee::CBX_STRING_RELEASE_BUFFER.call(c, RELEASE_BUFFER_RET)?;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The flag run of [`cbx_string_format_v`] from the flag test (0x319258):
/// the 0x31923c setup left the flag characters in T1/T0/A3/A2/A1/V1 and the
/// current character in V0. `'#'` and `'*'` advance through 0x3192a0; the
/// `'-'`/`'+'`/`'0'` arms advance inline (as 0x3192a4 does) and rejoin at
/// the flag test. Breaks with V0 holding the first non-flag character. Like
/// the generated 0x3192a4, unwinds at the flag head (0x319258).
fn format_v_flags(c: &mut Cpu) -> Flow {
    'flags: loop {
        // 0x319258.
        c.cycles += 2;
        let not_hash = c.g[V0][0] != c.g[T1][0];
        if !not_hash {
            // 0x319260: '#'.
            c.cycles += 2;
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(2));
        } else {
            // 0x319268.
            c.cycles += 2;
            let not_star = c.g[V0][0] != c.g[T0][0];
            if !not_star {
                // 0x319270: '*' takes the width.
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
                c.g[A0][0] = 0x1;
                c.g[S3][0] = Addr(r32(c, S1).wrapping_sub(8)).lw(c);
            } else {
                // 0x319280: '-'. The generated arm advances
                // `S0` itself and rejoins at `0x3192a4`,
                // skipping the `0x3192a0` (`+1`, `S0+=1`)
                // that only `'#'`, `'*'` and `' '` take.
                c.cycles += 2;
                if c.g[V0][0] == c.g[A3][0] {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                    if c.g[V0][0] == 0 {
                        break 'flags;
                    }
                    if back_edge_unwinds(c, 0x0031_9258) {
                        return Err(Unwound);
                    }
                    continue 'flags;
                } else {
                    // 0x319288: '+' (same shape as '-').
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A2][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                        if c.g[V0][0] == 0 {
                            break 'flags;
                        }
                        if back_edge_unwinds(c, 0x0031_9258) {
                            return Err(Unwound);
                        }
                        continue 'flags;
                    } else {
                        // 0x319290: '0' (same shape).
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[A1][0] {
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                            c.cycles += 3;
                            c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                            if c.g[V0][0] == 0 {
                                break 'flags;
                            }
                            if back_edge_unwinds(c, 0x0031_9258) {
                                return Err(Unwound);
                            }
                            continue 'flags;
                        } else {
                            // 0x319298.
                            c.cycles += 2;
                            let other = c.g[V0][0] != c.g[V1][0];
                            if other {
                                break 'flags;
                            }
                        }
                    }
                }
            }
        }
        // 0x3192a0.
        c.cycles += 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        // 0x3192a4: flag-run back edge.
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        if c.g[V0][0] == 0 {
            break 'flags;
        }
        if back_edge_unwinds(c, 0x0031_9258) {
            return Err(Unwound);
        }
    }
    Ok(())
}

/// The `%`-conversion tail of [`cbx_string_format_v`] (0x3192b0-0x31933c):
/// width (a `'*'` takes it from the argument list, digits from the format),
/// precision, and the first specifier-table dispatch.
fn format_v_conversion(c: &mut Cpu) -> Flow {
    // 0x3192b0: width, unless '*' already gave it.
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    } else {
        // 0x3192b8: width digits.
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_92c8);
        c.g[A2][0] = 0xa;
        callee::STRTOL.call(c, 0x0031_92c8)?;
        c.g[S0][0] = Addr(sp(c)).lw(c);
        c.g[S3][0] = sx(r32(c, V0));
        c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    }
    // 0x3192d8: precision.
    c.cycles += 3;
    c.g[V0][0] = 0x2e;
    let no_precision = c.g[V1][0] != c.g[V0][0];
    c.g[S2][0] = 0;
    if !no_precision {
        // 0x3192e4.
        c.cycles += 5;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        c.g[V1][0] = 0x2a;
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let prec_digits = c.g[V0][0] != c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        if prec_digits {
            // 0x319308: precision digits.
            c.cycles += 6;
            c.g[A1][0] = c.g[SP][0] | 0x4;
            c.g[RA][0] = sx(0x0031_9314);
            c.g[A2][0] = 0xa;
            callee::STRTOL.call(c, 0x0031_9314)?;
            c.g[S0][0] = Addr(sp(c).wrapping_add(4)).lw(c);
            c.g[S2][0] = sx(r32(c, V0));
        } else {
            // 0x3192f8: '*' takes the precision.
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[S2][0] = Addr(r32(c, S1).wrapping_sub(8)).lw(c);
        }
    }
    // 0x319320: first specifier table (base 0x48d860, index char minus 'F').
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x46));
    c.g[V0][0] = sx(r32(c, V0).wrapping_shl(24));
    c.g[A0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
    c.g[V1][0] = (c.g[A0][0] < 0x27u64) as u64;
    let in_range = c.g[V1][0] != 0;
    c.g[V0][0] = 0x490000;
    if !in_range {
        // Out-of-range first-table index: the generated `0x319320` falls
        // through to the `c`/`C`/`s`/`S` chain at `0x319358`, not to the
        // second table (table base `0x48d860`, `char - 'F' >= 0x27`).
        return format_v_chain(c);
    }
    // 0x31933c: through the table (its words are in the game image).
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A0).wrapping_shl(2));
    c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x27a0));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let spec = r32(c, A0);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    if format_v_table1_skip(spec) {
        // 0x319354: length modifiers only skip a character.
        c.cycles += 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
    }
    format_v_chain(c)
}

/// The `c`/`C`/`s`/`S` chain of [`cbx_string_format_v`] (0x319358-0x3193c0):
/// `c`/`C` cost a fixed 2, `s`/`S` cost `max(1, strlen(arg))`, anything else
/// costs 0 here and falls into the second table.
fn format_v_chain(c: &mut Cpu) -> Flow {
    // 0x319358.
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    c.g[V0][0] = 0x53;
    let upper_s = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = 0;
    if upper_s {
        // 0x3193ac: 'S'.
        c.cycles += 1;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        return format_v_strlen(c);
    }
    // 0x319368.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 84) as u64;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0x43;
    if !high {
        // 0x319374.
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            // 'C': fixed cost 2.
            c.g[A1][0] = 0x2;
            // 0x3193a4.
            c.cycles += 2;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        } else {
            // 0x31937c.
            c.cycles += 2;
        }
        return format_v_clamp(c);
    }
    // 0x319384.
    c.cycles += 3;
    c.g[V0][0] = 0x63;
    let lower_c = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 0x73;
    if lower_c {
        // 0x3193a0: fixed cost 2.
        c.cycles += 1;
        c.g[A1][0] = 0x2;
        // 0x3193a4.
        c.cycles += 2;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        return format_v_clamp(c);
    }
    // 0x319390.
    c.cycles += 2;
    if c.g[V1][0] == c.g[V0][0] {
        // 's'.
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        return format_v_strlen(c);
    }
    // 0x319398.
    c.cycles += 2;
    format_v_clamp(c)
}

/// The `s`/`S` arm of [`cbx_string_format_v`] (0x3193b0): `max(1, strlen)`.
fn format_v_strlen(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[RA][0] = sx(0x0031_93b8);
    c.g[A0][0] = Addr(r32(c, S1).wrapping_sub(8)).lw(c);
    call_ptr(c, callee::STRLEN, 0x0031_93b8)?;
    c.g[A1][0] = 0x1;
    c.g[V1][0] = ((0i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A1][0] = c.g[V0][0];
    }
    format_v_clamp(c)
}

/// The item-size clamp of [`cbx_string_format_v`] (0x3193c4-0x3193d4):
/// `max(item, width)`, then `min(item, precision)` when a precision ran.
fn format_v_clamp(c: &mut Cpu) -> Flow {
    // 0x3193c4.
    c.cycles += 2;
    let no_item = c.g[A1][0] == 0;
    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if no_item {
        return format_v_second(c);
    }
    // 0x3193cc.
    c.cycles += 2;
    let no_precision = c.g[S2][0] == 0;
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[S3][0];
    }
    if no_precision {
        return format_v_advance(c);
    }
    // 0x3193d4.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[S2][0];
    }
    format_v_advance(c)
}

/// The second specifier table of [`cbx_string_format_v`] (0x3193e0-0x319440,
/// base 0x48d900, index char minus `'G'`): floats cost at least 128, integers
/// (and `%p`) at least 32, `%n` only advances the argument list.
fn format_v_second(c: &mut Cpu) -> Flow {
    // 0x3193e0.
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x47));
    c.g[V0][0] = sx(r32(c, V0).wrapping_shl(24));
    c.g[A0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
    c.g[V1][0] = (c.g[A0][0] < 0x32u64) as u64;
    let in_range = c.g[V1][0] != 0;
    c.g[V0][0] = 0x490000;
    if !in_range {
        return format_v_advance(c);
    }
    // 0x3193fc: through the table (its words are in the game image).
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A0).wrapping_shl(2));
    c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x2700));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let spec = r32(c, A0);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match format_v_table2_class(spec) {
        FormatVItem::Float => {
            // 0x319414.
            c.cycles += 4;
            c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, S2)));
            c.g[A1][0] = 0x80;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 128) as u64;
        }
        FormatVItem::Int => {
            // 0x319424.
            c.cycles += 3;
            c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, S2)));
            c.g[A1][0] = 0x20;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 32) as u64;
        }
        FormatVItem::Count => {
            // 0x31943c: '%n'.
            c.cycles += 1;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
            return format_v_advance(c);
        }
        FormatVItem::None => return format_v_advance(c),
    }
    // 0x319430.
    c.cycles += 3;
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    format_v_advance(c)
}

/// The per-conversion tail of [`cbx_string_format_v`] (0x319440): grow the
/// result size by the conversion's item length.
fn format_v_advance(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[S4][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
    Ok(())
}

// ---------------------------------------------------------------------------
// EAC_GetUnpackedSize.

/// `int EAC_GetUnpackedSize(void* data)` (0x3b4818): the unpacked size from
/// an EAC/RefPack header: when `data[1]` is `0xFB`, a 24-bit size for the
/// short-form tags (`0x10`, `0x18`, `0x1A`, `0x1E`, `0x30`, `0x32`, `0x34`,
/// `0x46`), a 32-bit size for the long-form tags (`0x90`, `0x98`, `0x9A`,
/// `0x9E`, `0xB0`, `0xB2`, `0xB4`, `0xC6`), and 0 for `0xC0` and everything
/// else. The tag test masks the low bit (`p[0] & 0xFE`), so odd neighbours
/// of a listed tag take the same arm; the range dispatch below mirrors the
/// generated decision tree block for block (charges included): `0x90` goes
/// straight to the 32-bit read, every other tagged input passes through the
/// `0x3b48cc` recheck head (`+2`) into the short or long dispatch, and the
/// size arms rejoin through the `+1; V0 = A0` head while the default arms
/// return `V0 = 0` without it.
pub fn eac_get_unpacked_size(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = 0xfb;
    c.g[V1][0] = Addr(r32(c, A2)).off(1).lbu(c);
    let untagged = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = 0;
    if untagged {
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A2)).lbu(c);
        c.g[V1][0] = 0x90;
        c.g[A1][0] = c.g[V0][0] & 0xfe;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 145) as u64;
        if c.g[A1][0] == c.g[V1][0] {
            // `0x90` (and odd `0x91`): straight to the 32-bit size read,
            // skipping the recheck head and range dispatch below.
            eac_get32(c);
        } else {
            // The generated `0x3b48cc` head rechecks the range (`+2`) before
            // the dispatch: `tag >= 145` takes the long form, otherwise the
            // short form (whose `0x46`-miss falls through to the long form).
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                eac_long_form(c);
            } else {
                eac_short_form(c);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// The short-form tags of [`eac_get_unpacked_size`] (`0x10`, `0x18`, `0x1A`,
/// `0x1E`, `0x30`, `0x32`, `0x34`, `0x46`, all `< 145` and not `0x90`): a
/// 24-bit size, else 0 (a `0x46`-miss is the default arm).
/// Charges match the generated range dispatch.
fn eac_short_form(c: &mut Cpu) {
    // Entered after the caller's `0x3b48cc` recheck head (`+2`), so the
    // short arm implies `tag < 145` and `tag != 0x90` from here on.
    // 0x1e?
    c.cycles += 2;
    if c.g[A1][0] == 0x1e {
        eac_get24(c);
        return;
    }
    // 31 <= tag?
    c.cycles += 2;
    if (c.g[A1][0] as i64) >= 31 {
        // 0x32?
        c.cycles += 3;
        if c.g[A1][0] == 0x32 {
            eac_get24(c);
            return;
        }
        // 51 <= tag?
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 51 {
            // 0x34?
            c.cycles += 3;
            if c.g[A1][0] == 0x34 {
                eac_get24(c);
                return;
            }
            // 0x46?
            c.cycles += 2;
            if c.g[A1][0] == 0x46 {
                eac_get24(c);
                return;
            }
            // Miss: the generated `0x46`-miss breaks straight to the
            // default (`'b_3b49a8`, `V0` already `A0` = 0). Only tags
            // `>= 145` reach the long dispatch, via the `0x3b48cc` head
            // break — never a short-form miss.
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            return;
        }
        // 0x30?
        c.cycles += 2;
        if c.g[A1][0] == 0x30 {
            eac_get24(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        return;
    }
    // 0x18?
    c.cycles += 2;
    if c.g[A1][0] == 0x18 {
        eac_get24(c);
        return;
    }
    // 25 <= tag?
    c.cycles += 2;
    if (c.g[A1][0] as i64) >= 25 {
        // 0x1a?
        c.cycles += 3;
        if c.g[A1][0] == 0x1a {
            eac_get24(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        return;
    }
    // 0x10?
    c.cycles += 2;
    if c.g[A1][0] == 0x10 {
        eac_get24(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
}

/// The long-form tags of [`eac_get_unpacked_size`] (`>= 145`; `0x90` goes
/// straight to the size read and never enters here): a 32-bit size for
/// `0x98`, `0x9A`, `0x9E`, `0xB0`, `0xB2`, `0xB4`, `0xC6`, 0 for `0xC0` and
/// everything else.
fn eac_long_form(c: &mut Cpu) {
    // 0xb0?
    c.cycles += 3;
    if c.g[A1][0] == 0xb0 {
        eac_get32(c);
        return;
    }
    // 177 <= tag?
    c.cycles += 2;
    if (c.g[A1][0] as i64) >= 177 {
        // 0xb4?
        c.cycles += 3;
        if c.g[A1][0] == 0xb4 {
            eac_get32(c);
            return;
        }
        // 181 <= tag?
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 181 {
            // 0xc0: present but unpacked size 0.
            c.cycles += 3;
            if c.g[A1][0] == 0xc0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[A0][0];
                return;
            }
            // 0xc6?
            c.cycles += 2;
            if c.g[A1][0] == 0xc6 {
                eac_get32(c);
                return;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            return;
        }
        // 0xb2?
        c.cycles += 2;
        if c.g[A1][0] == 0xb2 {
            eac_get32(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        return;
    }
    // 0x9a?
    c.cycles += 2;
    if c.g[A1][0] == 0x9a {
        eac_get32(c);
        return;
    }
    // 155 <= tag?
    c.cycles += 2;
    if (c.g[A1][0] as i64) >= 155 {
        // 0x9e?
        c.cycles += 3;
        if c.g[A1][0] == 0x9e {
            eac_get32(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        return;
    }
    // 0x98?
    c.cycles += 2;
    if c.g[A1][0] == 0x98 {
        eac_get32(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
}

/// The 24-bit size read of [`eac_get_unpacked_size`]: `p[2..5]` big-endian.
fn eac_get24(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(2));
    c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(1)).lbu(c);
    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(2)).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_shl(16));
    c.g[V1][0] = sx(r32(c, V1).wrapping_shl(8));
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[A0][0] = c.g[V0][0] | c.g[A1][0];
    c.cycles += 1;
    c.g[V0][0] = c.g[A0][0];
}

/// The 32-bit size read of [`eac_get_unpacked_size`]: `p[2..6]` big-endian.
fn eac_get32(c: &mut Cpu) {
    c.cycles += 11;
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(2));
    c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(1)).lbu(c);
    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(2)).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_shl(24));
    c.g[V1][0] = sx(r32(c, V1).wrapping_shl(16));
    c.g[A2][0] = Addr(r32(c, A0).wrapping_add(3)).lbu(c);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[A1][0] = sx(r32(c, A1).wrapping_shl(8));
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    c.g[A0][0] = c.g[V0][0] | c.g[A2][0];
    c.cycles += 1;
    c.g[V0][0] = c.g[A0][0];
}

// ---------------------------------------------------------------------------
// Small EA utilities.

/// `void EA_FatalError(const char* fmt, ...)` (0x3e60e8): format into a
/// stack buffer, print it on channel 2 (twice when the verbose flag at
/// `0x45100c` is set), then abort. The abort trap ends the machine: tests
/// cover everything up to it with the abort call faked to return.
pub fn ea_fatal_error(c: &mut Cpu) {
    guest(c, |c| {
        const PRINT_RET: u32 = 0x003e_6150;
        const PRINT_TAIL_RET: u32 = 0x003e_6174;
        const ABORT_RET: u32 = 0x003e_617c;
        c.cycles += 14;
        let frame = Frame::open(c, 0x290);
        frame.save_ra(c, 0x200);
        Addr(sp(c).wrapping_add(0x258)).sd(c, c.g[A1][0]);
        Addr(sp(c).wrapping_add(0x260)).sd(c, c.g[A2][0]);
        Addr(sp(c).wrapping_add(0x268)).sd(c, c.g[A3][0]);
        Addr(sp(c).wrapping_add(0x270)).sd(c, c.g[T0][0]);
        Addr(sp(c).wrapping_add(0x278)).sd(c, c.g[T1][0]);
        Addr(sp(c).wrapping_add(0x280)).sd(c, c.g[T2][0]);
        Addr(sp(c).wrapping_add(0x288)).sd(c, c.g[T3][0]);
        c.swc1(12, sp(c).wrapping_add(0x248));
        c.swc1(14, sp(c).wrapping_add(0x24c));
        c.swc1(16, sp(c).wrapping_add(0x250));
        let null_fmt = c.g[A0][0] == 0;
        c.swc1(18, sp(c).wrapping_add(0x254));
        if null_fmt {
            c.cycles += 2;
            c.sb(sp(c), 0);
            c.g[A1][0] = 0x490000;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[A0][0];
            c.g[A2][0] = sx(sp(c).wrapping_add(0x258));
            c.g[RA][0] = sx(0x003e_6130);
            c.g[A0][0] = sx(sp(c));
            call_ptr(c, callee::VSPRINTF, 0x003e_6130)?;
            c.g[A1][0] = 0x490000;
        }
        // Print the buffer on channel 2, then again with the verbose detail
        // when the flag at 0x45100c is set.
        c.cycles += 8;
        c.g[A2][0] = sx(sp(c));
        c.g[A0][0] = 0x2;
        c.g[RA][0] = sx(PRINT_RET);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x5ef0));
        callee::DEBUG_PRINTF.call(c, PRINT_RET)?;
        c.g[V0][0] = 0x450000;
        c.g[A2][0] = Addr(r32(c, V0).wrapping_add(0x100c)).lw(c);
        let verbose = c.g[A2][0] != 0;
        c.g[V0][0] = 0x450000;
        if verbose {
            c.cycles += 5;
            c.g[A1][0] = 0x490000;
            c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x1010)).lw(c);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x5f00));
            c.g[RA][0] = sx(PRINT_TAIL_RET);
            c.g[A0][0] = 0x2;
            callee::DEBUG_PRINTF.call(c, PRINT_TAIL_RET)?;
        }
        c.cycles += 5;
        callee::EA_ABORT.call(c, ABORT_RET)?;
        c.g[RA][0] = Addr(sp(c).wrapping_add(0x200)).ld(c);
        frame.close(c);
        Ok(())
    })
}

/// `void EA_DebugMessage(const char* fmt, ...)` (0x3e6070): format into a
/// stack buffer and hand it to the abort-message hook at `0x450dd8`; a null
/// `fmt` sends an empty message.
pub fn ea_debug_message(c: &mut Cpu) {
    guest(c, |c| {
        const VSPRINTF_RET: u32 = 0x003e_60b8;
        const HOOK_RET: u32 = 0x003e_60dc;
        const HOOK_TABLE: u32 = 0x0045_0dd8;
        c.cycles += 14;
        let frame = Frame::open(c, 0x290);
        frame.save_ra(c, 0x200);
        Addr(sp(c).wrapping_add(0x258)).sd(c, c.g[A1][0]);
        Addr(sp(c).wrapping_add(0x260)).sd(c, c.g[A2][0]);
        Addr(sp(c).wrapping_add(0x268)).sd(c, c.g[A3][0]);
        Addr(sp(c).wrapping_add(0x270)).sd(c, c.g[T0][0]);
        Addr(sp(c).wrapping_add(0x278)).sd(c, c.g[T1][0]);
        Addr(sp(c).wrapping_add(0x280)).sd(c, c.g[T2][0]);
        Addr(sp(c).wrapping_add(0x288)).sd(c, c.g[T3][0]);
        c.swc1(12, sp(c).wrapping_add(0x248));
        c.swc1(14, sp(c).wrapping_add(0x24c));
        c.swc1(16, sp(c).wrapping_add(0x250));
        let null_fmt = c.g[A0][0] == 0;
        c.swc1(18, sp(c).wrapping_add(0x254));
        if null_fmt {
            c.cycles += 2;
            c.sb(sp(c), 0);
            c.g[V0][0] = 0x450000;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[A0][0];
            c.g[A2][0] = sx(sp(c).wrapping_add(0x258));
            c.g[RA][0] = sx(VSPRINTF_RET);
            c.g[A0][0] = sx(sp(c));
            call_ptr(c, callee::VSPRINTF, VSPRINTF_RET)?;
            c.g[V0][0] = 0x450000;
        }
        c.cycles += 8;
        c.g[A0][0] = 0x490000;
        c.g[V1][0] = Addr(HOOK_TABLE).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5ee8));
        c.g[RA][0] = sx(HOOK_RET);
        c.g[A1][0] = sx(sp(c));
        let hook = r32(c, V1);
        c.pc = hook;
        call_indirect(c, hook);
        returned(c, HOOK_RET)?;
        c.g[RA][0] = Addr(sp(c).wrapping_add(0x200)).ld(c);
        frame.close(c);
        Ok(())
    })
}

/// `int EA_IsValidPointer(void* p)` (0x3e6240): nonzero when `p` lies in a
/// mapped range (low RAM, the exception vectors, the two cached mirrors, or
/// the scratchpad window).
pub fn ea_is_valid_pointer(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0xffff_ffff_fff0_0000;
    c.g[A1][0] = 0x07ef_0000;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    c.g[A1][0] = c.g[A1][0] | 0xffff;
    c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    let low_ram = c.g[V0][0] == 0;
    c.g[A2][0] = 0;
    // Each range below returns 1 on a hit; the scratchpad test is last and
    // its miss falls out with `A2` still 0.
    let valid = if low_ram {
        true
    } else {
        c.cycles += 6;
        c.g[V0][0] = 0xffff_ffff_f000_0000;
        c.g[V1][0] = 0x0400_0000;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        let vectors = c.g[V1][0] == 0;
        c.g[V0][0] = 0xffff_ffff_dff0_0000;
        if vectors {
            true
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            let cached = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_cff0_0000;
            if cached {
                true
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                let uncached = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_9000_0000;
                if uncached {
                    true
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = (c.g[V0][0] < 0x4000u64) as u64;
                    c.g[V0][0] != 0
                }
            }
        }
    };
    if valid {
        c.cycles += 1;
        c.g[A2][0] = 0x1;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

/// `void EA_MemMove(void* dst, const void* src, size_t n)` (0x3e6188):
/// copy `n` bytes, going backwards from the ends when `dst` overlaps the
/// tail of `[src, src+n)`; otherwise forward through `EA_MemCopy`.
pub fn ea_mem_move(c: &mut Cpu) {
    guest(c, |c| {
        const FORWARD_RET: u32 = 0x003e_61b0;
        const BACK_HEAD: u32 = 0x003e_61d0;
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        // Unsigned `src < dst` (`0x3e6188` compares full 64-bit addresses;
        // a signed compare misroutes non-canonical `src` like seed 3's
        // `0xffffffff7fffffff` to the backward arm).
        c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
        let forward = c.g[V0][0] == 0;
        c.sd(sp(c), c.g[RA][0]);
        if forward {
            // `src >= dst`: plain forward copy.
            c.cycles += 4;
            callee::EA_MEM_COPY.call(c, FORWARD_RET)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
            // Unsigned `dst < src+n` (same non-canonical reason as above).
            c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                // `dst` past the end: plain forward copy.
                c.cycles += 4;
                callee::EA_MEM_COPY.call(c, FORWARD_RET)?;
                c.g[RA][0] = Addr(sp(c)).ld(c);
            } else {
                // Overlap: copy backwards from `dst+n`/`src+n`.
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
                c.cycles += 4;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                let empty = c.g[A2][0] == c.g[V0][0];
                c.g[A1][0] = c.g[V1][0];
                if !empty {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    loop {
                        c.cycles += 7;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                        c.g[V0][0] = Addr(r32(c, A1)).lbu(c);
                        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                        let more = c.g[A2][0] != c.g[V1][0];
                        Addr(r32(c, A0)).sb(c, c.g[V0][0]);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, BACK_HEAD);
                        }
                    }
                }
                c.cycles += 1;
                c.g[RA][0] = Addr(sp(c)).ld(c);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void EA_AddShutdownHandler(void (*fn)(void))` (0x3e4a30): register `fn`
/// in the 64-slot table at `0x450c78` unless already present; a second scan
/// finds the first empty slot when the table is full of others (slot 0
/// written directly when it is free). Silently keeps a full table.
pub fn ea_add_shutdown_handler(c: &mut Cpu) {
    const TABLE: u32 = 0x0045_0c78;
    const SLOTS: u64 = 64;
    const SCAN_HEAD: u32 = 0x003e_4a40;
    const EMPTY_HEAD: u32 = 0x003e_4a74;
    c.cycles += 4;
    c.g[A3][0] = 0x450000;
    c.g[A2][0] = c.g[A0][0];
    c.g[A1][0] = 0;
    c.g[V1][0] = sx(TABLE);
    'outer: loop {
        'done: {
            // First scan: already registered?
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            let found = c.g[V0][0] == c.g[A2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            if found {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < SLOTS as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, SCAN_HEAD);
                }
                continue 'outer;
            }
            // Second scan: first empty slot.
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A3).wrapping_add(0xc78)).lw(c);
            let slot0_used = c.g[V0][0] != 0;
            c.g[A1][0] = 0;
            if !slot0_used {
                // Slot 0 free: the generated `0x3e4a30` writes and returns
                // at once (`+2`, `sw`, `pc = RA`), skipping the shared
                // `+2` return the other paths take below.
                c.cycles += 2;
                Addr(r32(c, A3).wrapping_add(0xc78)).sw(c, c.g[A2][0]);
                ret(c);
                return;
            }
            c.cycles += 1;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < SLOTS as i64) as u64;
                let full = c.g[V0][0] == 0;
                c.g[V0][0] = sx(r32(c, A3).wrapping_add(0xc78));
                if full {
                    break 'done;
                }
                c.cycles += 5;
                c.g[V1][0] = sx(r32(c, A1).wrapping_shl(2));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                let occupied = c.g[A0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                if occupied {
                    if event_due(c) {
                        deliver_event_here(c, EMPTY_HEAD);
                    }
                } else {
                    c.cycles += 1;
                    Addr(r32(c, V1)).sw(c, c.g[A2][0]);
                    break 'done;
                }
            }
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `int EA_Crc16(void* buf, int size, int seed)` (0x3e62d0): CRC-16 over
/// `size` bytes with the table at `0x451018`, seeded with `seed & 0xFFFF`;
/// an empty (or backwards) range returns the seed unchanged.
pub fn ea_crc16(c: &mut Cpu) {
    const HEAD: u32 = 0x003e_62f0;
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A2][0] & 0xffff;
    c.g[A2][0] = sx(r32(c, A3).wrapping_add(r32(c, A1)));
    // Unsigned `buf < buf+size` (`0x3e62d0` compares full addresses; a
    // signed compare takes seed 50's non-canonical `buf` with `size 0`
    // into the byte loop instead of the empty arm).
    c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[V0][0] = 0x450000;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = c.g[V1][0] & 0xffff;
    } else {
        c.cycles += 2;
        c.g[T0][0] = sx(0x0045_0000u32.wrapping_add(0x1018));
        loop {
            c.cycles += 11;
            c.g[V0][0] = Addr(r32(c, A3)).lbu(c);
            c.g[A1][0] = sx(r32(c, V1).wrapping_shr(8));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
            c.g[A0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
            c.g[V0][0] = c.g[V0][0] & 0xff;
            c.g[V0][0] = sx(r32(c, V0).wrapping_shl(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
            c.g[V1][0] = Addr(r32(c, V0)).lhu(c);
            let cont = c.g[A0][0] != 0;
            c.g[V1][0] = c.g[V1][0] ^ c.g[A1][0];
            if !cont {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[V1][0] & 0xffff;
    }
    ret(c);
}

/// `void EA_RunFatalHandlers(void)` (0x3e49b8): call every set handler in
/// the table at `0x450c78` from the top slot down, clearing each slot after
/// it runs (or is skipped).
pub fn ea_run_fatal_handlers(c: &mut Cpu) {
    guest(c, |c| {
        const TABLE_TOP: u32 = 0x0045_0d74;
        const SLOTS: u64 = 0x40;
        const HANDLER_RET: u32 = 0x003e_49ec;
        const HEAD: u32 = 0x003e_49d8;
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_0000u32.wrapping_add(0xc78));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(TABLE_TOP);
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = SLOTS - 1;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            let empty = c.g[V0][0] == 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            if !empty {
                c.cycles += 2;
                c.g[RA][0] = sx(HANDLER_RET);
                let handler = r32(c, V0);
                c.pc = handler;
                call_indirect(c, handler);
                returned(c, HANDLER_RET)?;
            }
            c.cycles += 3;
            Addr(r32(c, S0)).sw(c, 0);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EA_Abort(void)` (0x3e4a10): run the fatal handlers, then trap
/// (`break 0xffff`). The trap stops the machine, so this port has no
/// differential test and no `OVERRIDES` entry: see the lane report.
#[allow(dead_code)]
pub fn ea_abort(c: &mut Cpu) {
    guest(c, |c| {
        const HANDLERS_RET: u32 = 0x003e_4a20;
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::EA_RUN_FATAL_HANDLERS.call(c, HANDLERS_RET)?;
        c.brk(0x003e_4a20, 65535);
        Ok(())
    })
}

/// `void EA_AssertNotBusy(void)` (0x3e62b0): trap unless the busy flag at
/// `0x450da4` is clear. Only the clear arm returns, so only it is tested.
pub fn ea_assert_not_busy(c: &mut Cpu) {
    const BUSY: u32 = 0x0045_0da4;
    c.cycles += 4;
    c.g[V0][0] = 0x450000;
    c.g[V1][0] = Addr(BUSY).lw(c);
    let idle = c.g[V1][0] == 0;
    if idle {
        c.cycles += 2;
        ret(c);
    } else {
        c.cycles += 1;
        c.brk(0x003e_62c0, 6);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::Rng;

    // ------------------------------------------------- cBXString_FormatV.

    /// Fixture addresses (all with nonzero low bytes).
    const FV_THIS: u32 = crate::difftest::DATA + 0x104;
    const FV_BUF: u32 = crate::difftest::DATA + 0x1304;
    const FV_FMT: u32 = crate::difftest::DATA + 0x2104;
    const FV_ARGS: u32 = crate::difftest::DATA + 0x2204;
    const FV_SARG: u32 = crate::difftest::DATA + 0x2304;
    const FV_SARG2: u32 = crate::difftest::DATA + 0x2310;
    const FV_NPTR: u32 = crate::difftest::DATA + 0x2410;

    /// One format per specifier class the two jump tables dispatch: ints,
    /// chars (`c`/`C`), strings (`s`/`S`), floats (`e`/`f`/`g`, `G` through
    /// the first table's default), `%X`, count, unknown, length modifiers,
    /// digit widths, digit precisions, `'*'` widths/precisions, every flag,
    /// mixed forms, escapes, a bare trailing `%` (which scans one past the
    /// NUL) and the empty format.
    const FV_FMTS: [&[u8]; 45] = [
        b"%d",
        b"%x",
        b"%o",
        b"%u",
        b"%i",
        b"%p",
        b"%c",
        b"%C",
        b"%s",
        b"%S",
        b"%e",
        b"%f",
        b"%g",
        b"%G",
        b"%X",
        b"%d%n",
        b"%q",
        b"%ld",
        b"%hd",
        b"%Fd",
        b"%Nd",
        b"%5d",
        b"%.3d",
        b"%8.3d",
        b"%.2s",
        b"%8.2s",
        b"%3s",
        b"%.0s",
        b"%*d",
        b"%.*d",
        b"%-d",
        b"%+d",
        b"% d",
        b"%#x",
        b"%05d",
        b"v=%d s=%s!",
        b"x%d%s",
        b"%d%s",
        b"%d-%d",
        b"100%%",
        b"plain",
        b"%d%",
        b"a%db",
        b"%#d",
        b"",
    ];

    /// Slot values for the format at `which`: every specifier the format
    /// consumes finds a valid argument (strings for `%s`, 1.0 for floats,
    /// small widths for `'*'`, a writable word for `%n`). Ints vary on
    /// `seed / 45`, the empty/nonempty string choice on `seed / 9`.
    fn fv_slots(c: &mut Cpu, seed: u32, which: usize) {
        let d1 = [0u64, 1, 42, 0xffff_ffff, 123456][seed as usize / 45 % 5];
        let d2 = (seed % 100) as u64;
        let d3 = ((seed * 7) % 100) as u64;
        let dw = (1 + seed % 6) as u64;
        const ONE: u64 = 0x3ff0_0000_0000_0000;
        let (s0, s1) = match which {
            8 | 9 | 24 | 25 | 26 | 27 => (FV_SARG as u64, d2),
            10 | 11 | 12 | 13 => (ONE, d2),
            28 | 29 => (dw, d1),
            15 => (d1, FV_NPTR as u64),
            35 | 36 | 37 => (d1, [FV_SARG as u64, FV_SARG2 as u64][seed as usize / 9 % 2]),
            38 => (d1, d2),
            _ => (d1, d2),
        };
        c.bus.write64(FV_ARGS, s0);
        c.bus.write64(FV_ARGS + 8, s1);
        c.bus.write64(FV_ARGS + 16, d3);
    }

    /// The shared FormatV fixture: the game image first (the two jump
    /// tables the generated side dispatches through), then a heap-backed
    /// string object, the format and its argument slots, float and scratch
    /// registers, and the entry state. `which` selects the slot layout for
    /// `fmt` (an index into [`FV_FMTS`] for the same shape).
    fn fv_fixture(c: &mut Cpu, seed: u32, fmt: &[u8], which: usize) {
        use crate::difftest::{elf_image, fill};
        use crate::game::bx::bxstring::tests::string_obj;
        use crate::game::bx::cubicspline::tests::heap_alloc_machine;
        use crate::game::misc::tests::put_cstr;
        elf_image(c);
        assert_eq!(
            c.bus.read32(0x0048_d860),
            0x0031_9354,
            "format table missing"
        );
        let mut r = heap_alloc_machine(c, seed, 0x3191_c0);
        string_obj(c, FV_THIS, FV_BUF, 1, 5, 0x100);
        put_cstr(c, FV_BUF, b"hello");
        fill(c, &mut r, FV_BUF + 8, 4);
        put_cstr(c, FV_SARG, b"w\xe9rld");
        put_cstr(c, FV_SARG2, b"");
        c.bus.write32(FV_NPTR, 0);
        put_cstr(c, FV_FMT, fmt);
        // A trailing '%' scans one past the NUL: keep the guard zeroed so it
        // terminates deterministically.
        for i in 1..64u32 {
            c.bus.write8(FV_FMT + fmt.len() as u32 + i, 0);
        }
        fv_slots(c, seed, which);
        for (i, f) in [12, 14, 16, 18].into_iter().enumerate() {
            c.f[f] = 0x3f80_0000 + 0x0011_0000 * i as u32 + seed;
        }
        for (i, t) in [T0, T1, T2, T3].into_iter().enumerate() {
            c.g[t][0] = 0x1357_0000_0000_0100 * (i as u64 + 1) + seed as u64;
        }
        c.pc = 0x0031_91c0;
        c.g[A0][0] = FV_THIS as u64;
        c.g[A1][0] = FV_FMT as u64;
        c.g[A2][0] = FV_ARGS as u64;
    }

    /// `cBXString_FormatV`: every jump-table class against the generated
    /// body, argument values crossed on `seed / 45` digits. No `check_paths`:
    /// the totals depend on the real callees (`strtol`, `strlen`,
    /// `vsprintf`), which this lane cannot run to pin.
    #[test]
    fn cbx_string_format_v_matches() {
        use crate::difftest::check;
        check(ee::s10::f_003191c0__generated, cbx_string_format_v, 200, &|c, seed| {
            let which = seed as usize % FV_FMTS.len();
            fv_fixture(c, seed, FV_FMTS[which], which);
        });
    }

    use crate::difftest::events::{check_events, driven, schedule};

    driven!(gen_format_v, ee::s10::f_003191c0__generated);

    driven!(hand_format_v, cbx_string_format_v);

    /// Short event formats by `seed % 4`: a leading plain character (the
    /// first unwind lands on the scan head), a leading flag (the flag head),
    /// a call-heavy format (delivered but never unwound, so no mid-call
    /// unwind), and a call-free format.
    const FV_EVENT_FMTS: [&[u8]; 4] = [b"a%db", b"%#x", b"%d%s", b"ab"];

    /// Event setup for [`cbx_string_format_v_events`]: the shared fixture
    /// with a short format, then a main-thread event schedule. Unwind mode
    /// only runs where the first back edge precedes any call; the call-heavy
    /// format is forced to deliver-only. Returns the event mode.
    fn fv_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let variant = (seed % 4) as usize;
        let mut mode = seed / 4 % 3;
        if variant == 2 && mode == 1 {
            // "%d%s" would unwind inside `strtol`: deliver without unwinding.
            mode = 2;
        }
        // Slot layouts matching each short format's shape.
        let which = [42, 33, 37, 40][variant];
        fv_fixture(c, seed, FV_EVENT_FMTS[variant], which);
        // `schedule` starts the thread (clobbering the entry registers), so
        // the entry state goes in after it.
        let mut r = Rng(0x3191_c000 ^ seed as u64);
        let ev = schedule(c, &mut r, mode, 0x0031_91c0);
        c.pc = 0x0031_91c0;
        c.g[A0][0] = FV_THIS as u64;
        c.g[A1][0] = FV_FMT as u64;
        c.g[A2][0] = FV_ARGS as u64;
        c.cycles = 1000;
        if mode != 0 {
            // First back edge delivers at once, before any call runs.
            c.next_event = 1000;
        }
        ev
    }

    /// `cBXString_FormatV` with real events: both sides driven through
    /// unwinds, the first unwind landing on the scan head (seed 4) and the
    /// flag head (seed 5) in turn. `RESUMES` counts print with the test.
    #[test]
    fn cbx_string_format_v_events() {
        check_events(gen_format_v, hand_format_v, 48, &fv_events_setup);
        use crate::difftest::{fresh, run_generated};
        for (seed, head) in [(4u32, 0x0031_9210u32), (5, 0x0031_9258)] {
            let mut c = fresh();
            fv_events_setup(&mut c, seed);
            c.next_event = c.cycles;
            run_generated(&mut c, ee::s10::f_003191c0__generated);
            assert_eq!(c.pc, head | 1, "seed {seed}: first unwind");
        }
    }

    // ------------------------------------------------- EAC_GetUnpackedSize.

    /// Header bytes for the EAC fixture, `seed % 28`: the eight short-form
    /// tags, the eight long-form tags, `0xC0`, nine defaults at different
    /// dispatch depths, and two odd neighbours proving the low-bit mask
    /// (`0x11` takes the `0x10` arm, `0x91` the `0x90` arm).
    const EAC_TAGS: [u8; 28] = [
        0x10, 0x18, 0x1a, 0x1e, 0x30, 0x32, 0x34, 0x46, 0x90, 0x98, 0x9a, 0x9e, 0xb0, 0xb2, 0xb4,
        0xc6, 0xc0, 0x00, 0x1b, 0x40, 0x50, 0xa0, 0xc2, 0xff, 0x7f, 0x11, 0x91, 0x20,
    ];

    /// `EAC_GetUnpackedSize`: every tag class and armlength, the tagged bit
    /// on `seed / 28`, the data bytes from the fixture RNG. Totals are the
    /// generated costs observed in the run (no calls), listed in the
    /// `check_paths`.
    #[test]
    fn eac_get_unpacked_size_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check_paths, fill, DATA};
        check_paths(
            ee::s13::f_003b4818__generated,
            eac_get_unpacked_size,
            200,
            &[8, 24, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36, 37, 38, 41],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b48_18, 0);
                const HDR: u32 = DATA + 0x105;
                fill(c, &mut r, HDR + 2, 1);
                c.bus.write8(HDR, EAC_TAGS[seed as usize % 28]);
                c.bus.write8(
                    HDR + 1,
                    if seed / 28 % 2 == 0 {
                        0xfb
                    } else {
                        [0x00, 0xfa, 0xfc][seed as usize / 56 % 3]
                    },
                );
                c.pc = 0x003b_4818;
                c.g[A0][0] = HDR as u64
                    | if seed / 112 % 2 == 1 {
                        0xffff_0000_0000_0000
                    } else {
                        0
                    };
            },
        );
    }

    // ------------------------------------------------- EA_IsValidPointer.

    /// Probe addresses, `seed % 16`: each range's edges in and out, plus
    /// misses (zero, past the end, all bits set). Reads never happen (pure
    /// arithmetic), so any mapped or unmapped address is safe; the exact
    /// boundaries are the point.
    const VALID_PTRS: [u64; 16] = [
        0x0010_0000,
        0x008e_ffff,
        0x0080_0000,
        0x1000_0000,
        0x1400_0000,
        0x1400_0001,
        0x2010_0000,
        0x27ff_ffff,
        0x3010_0000,
        0x37ff_ffff,
        0x7000_0000,
        0x7000_3fff,
        0x0000_0000,
        0x0090_0000,
        0x7000_4000,
        0xffff_ffff,
    ];

    /// `EA_IsValidPointer`: every range arm, totals hand-counted (no calls).
    #[test]
    fn ea_is_valid_pointer_matches() {
        use crate::difftest::check_paths;
        use crate::difftest::fakes::machine;
        check_paths(
            ee::s14::f_003e6240__generated,
            ea_is_valid_pointer,
            160,
            &[10, 16, 20, 24, 27, 28],
            &|c, seed| {
                machine(c, seed, 0x3e62_40, 0);
                c.pc = 0x003e_6240;
                c.g[A0][0] = VALID_PTRS[seed as usize % 16];
            },
        );
    }

    // ------------------------------------------------- EA_MemMove.

    /// `EA_MemMove`: forward disjoint (`dst` below and above `src`),
    /// backwards overlap, and a backwards-with-zero-length case built from a
    /// non-canonical address (`seed % 4`), lengths `seed / 4 % 25` (0
    /// included). The zero-length backward arm only triggers when the 32-bit
    /// wrapped end still exceeds `dst` while the full 64-bit `src` precedes
    /// it. No `check_paths`: the forward arm's totals run through the real
    /// `EA_MemCopy`.
    #[test]
    fn ea_mem_move_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(ee::s14::f_003e6188__generated, ea_mem_move, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3e61_88, 0);
            const BASE: u32 = DATA + 0x204;
            fill(c, &mut r, BASE, 16);
            let n = seed / 4 % 25;
            let (dst, src, len) = match seed % 4 {
                0 => (BASE as u64, (BASE + 0x20) as u64, n as u64),
                1 => ((BASE + 0x20) as u64, BASE as u64, n as u64),
                2 => ((BASE + 8) as u64, BASE as u64, n as u64),
                _ => (0, 0xffff_ffff_7fff_ffff, 0),
            };
            c.pc = 0x003e_6188;
            c.g[A0][0] = dst;
            c.g[A1][0] = src;
            c.g[A2][0] = len;
        });
    }

    driven!(gen_mem_move, ee::s14::f_003e6188__generated);

    driven!(hand_mem_move, ea_mem_move);

    /// `EA_MemMove` with real events: the backward loop's head delivers on
    /// both sides (structured loop, no unwind); forward seeds run the real
    /// copy under the same schedule (a mid-call unwind restarts both sides
    /// identically).
    #[test]
    fn ea_mem_move_events() {
        check_events(gen_mem_move, hand_mem_move, 48, &|c, seed| {
            use crate::difftest::events::schedule;
            use crate::difftest::fakes::machine;
            use crate::difftest::{fill, DATA};
            let mut r = machine(c, seed, 0x3e61_88, 0);
            const BASE: u32 = DATA + 0x204;
            fill(c, &mut r, BASE, 16);
            let back = seed % 2 == 0;
            let n = 8 + seed % 8;
            c.pc = 0x003e_6188;
            c.g[A0][0] = (if back { BASE + 8 } else { BASE }) as u64;
            c.g[A1][0] = BASE as u64;
            c.g[A2][0] = n as u64;
            let mut r2 = Rng(0x3e61_8800 ^ seed as u64);
            let mode = seed / 2 % 3;
            let ev = schedule(c, &mut r2, mode, 0x003e_6188);
            c.pc = 0x003e_6188;
            c.g[A0][0] = (if back { BASE + 8 } else { BASE }) as u64;
            c.g[A1][0] = BASE as u64;
            c.g[A2][0] = n as u64;
            c.cycles = 1000;
            if mode != 0 {
                c.next_event = 1000;
            }
            let _ = r;
            ev
        });
        use crate::difftest::{fresh, run_generated};
        let mut c = fresh();
        {
            use crate::difftest::fakes::machine;
            use crate::difftest::{fill, DATA};
            let mut r = machine(&mut c, 0, 0x3e61_88, 0);
            const BASE: u32 = DATA + 0x204;
            fill(&mut c, &mut r, BASE, 16);
            c.pc = 0x003e_6188;
            c.g[A0][0] = (BASE + 8) as u64;
            c.g[A1][0] = BASE as u64;
            c.g[A2][0] = 12;
            c.cycles = 1000;
            c.next_event = 1000;
        }
        run_generated(&mut c, ee::s14::f_003e6188__generated);
        // Structured loop (`deliver_event_here` ignores the unwind request):
        // the event delivers and the function returns to `RA`, it never
        // resumes at the head. `RA` is what the framework records.
        assert_eq!(c.pc, crate::difftest::RA, "backward: deliver-only, no unwind");
    }

    // ------------------------------------------------- EA_AddShutdownHandler.

    /// `EA_AddShutdownHandler`: the walk runs from the top slot down, so a
    /// handler at slot 63 is found at once (+9) and at slot 0 last (+387);
    /// insert at slot 0 (+393), slot 1 (+403) and slot 5 (+435), and a full
    /// table (+901), on `seed % 6`. Totals are the generated costs observed
    /// in the run (no calls). Filler words stay distinct from the handler.
    #[test]
    fn ea_add_shutdown_handler_matches() {
        use crate::difftest::check_paths;
        use crate::difftest::fakes::machine;
        check_paths(
            ee::s14::f_003e4a30__generated,
            ea_add_shutdown_handler,
            200,
            &[9, 387, 393, 403, 435, 901],
            &|c, seed| {
                machine(c, seed, 0x3e4a_30, 0);
                const TABLE: u32 = 0x0045_0c78;
                let mode = seed % 6;
                let handler = 0x0100_00a0u64 + (seed / 6 % 16) as u64;
                for i in 0..64u32 {
                    let used = 0x0200_0101u64 + i as u64;
                    let v = match mode {
                        0 => {
                            if i == 0 {
                                handler as u32
                            } else {
                                used as u32
                            }
                        }
                        1 => {
                            if i == 63 {
                                handler as u32
                            } else {
                                used as u32
                            }
                        }
                        2 => {
                            if i == 0 {
                                0
                            } else {
                                used as u32
                            }
                        }
                        3 => {
                            if i == 0 {
                                used as u32
                            } else if i == 1 {
                                0
                            } else {
                                used as u32
                            }
                        }
                        4 => {
                            if i < 5 {
                                used as u32
                            } else if i == 5 {
                                0
                            } else {
                                used as u32
                            }
                        }
                        _ => used as u32,
                    };
                    c.bus.write32(TABLE + 4 * i, v);
                }
                c.pc = 0x003e_4a30;
                c.g[A0][0] = handler;
            },
        );
    }

    driven!(gen_add_shutdown_handler, ee::s14::f_003e4a30__generated);

    driven!(hand_add_shutdown_handler, ea_add_shutdown_handler);

    /// Full-table setup shared by the shutdown-handler event seeds: the
    /// handler is absent, so both scans run their whole length.
    fn shutdown_full_table(c: &mut Cpu) {
        const TABLE: u32 = 0x0045_0c78;
        for i in 0..64u32 {
            c.bus.write32(TABLE + 4 * i, 0x0200_0101 + i);
        }
    }

    /// `EA_AddShutdownHandler` with real events, `seed % 3`: deliver at the
    /// first scan's head, deliver at the second scan's head (the deadline is
    /// hand-counted past the whole first scan), and deliver-only. Both
    /// loops are structured (`deliver_event_here` ignores the unwind
    /// request), so no seed unwinds: the extra runs below assert the
    /// deliver-only return to `RA` per deadline class.
    #[test]
    fn ea_add_shutdown_handler_events() {
        check_events(
            gen_add_shutdown_handler,
            hand_add_shutdown_handler,
            48,
            &|c, seed| {
                use crate::difftest::events::schedule;
                use crate::difftest::fakes::machine;
                machine(c, seed, 0x3e4a_30, 0);
                shutdown_full_table(c);
                let variant = seed % 3;
                let mode = [1u32, 1, 2][variant as usize];
                let mut r = Rng(0x3e4a_3000 ^ seed as u64);
                let ev = schedule(c, &mut r, mode, 0x003e_4a30);
                c.pc = 0x003e_4a30;
                c.g[A0][0] = 0x0100_00a0;
                c.cycles = 1000;
                // The first scan's last back edge runs at cycle 1385 and the
                // second scan's first at 1392: 1390 lands the first unwind
                // on the second head.
                c.next_event = if variant == 1 { 1390 } else { 1000 };
                ev
            },
        );
        use crate::difftest::fakes::machine;
        use crate::difftest::{fresh, run_generated};
        for (seed, at) in [(0u32, 1000u64), (1, 1390)] {
            let mut c = fresh();
            machine(&mut c, seed, 0x3e4a_30, 0);
            shutdown_full_table(&mut c);
            c.pc = 0x003e_4a30;
            c.g[A0][0] = 0x0100_00a0;
            c.cycles = 1000;
            c.next_event = at;
            run_generated(&mut c, ee::s14::f_003e4a30__generated);
            assert_eq!(c.pc, crate::difftest::RA, "seed {seed}: deliver-only, no unwind");
        }
    }

    // ------------------------------------------------- EA_Crc16.

    /// `EA_Crc16`: lengths `seed % 25` (0 takes the skip arm), negative
    /// sizes and high-bit pointers on higher digits (same skip total), the
    /// table and buffer filled from the fixture RNG. Totals hand-counted:
    /// 8 empty, `10 + 11 * len` otherwise.
    #[test]
    fn ea_crc16_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check_paths, fill, DATA};
        check_paths(
            ee::s14::f_003e62d0__generated,
            ea_crc16,
            200,
            &[
                8, 21, 32, 43, 54, 65, 76, 87, 98, 109, 120, 131, 142, 153, 164, 175, 186, 197,
                208, 219, 230, 241, 252, 263, 274,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e62_d0, 0);
                const BUF: u32 = DATA + 0x204;
                const TAB: u32 = 0x0045_1018;
                fill(c, &mut r, TAB, 128);
                fill(c, &mut r, BUF, 8);
                let len = seed % 25;
                let neg = seed / 25 % 2 == 1;
                c.pc = 0x003e_62d0;
                c.g[A0][0] = BUF as u64
                    | if seed / 50 % 2 == 1 {
                        0xffff_0000_0000_0000
                    } else {
                        0
                    };
                c.g[A1][0] = if neg {
                    0xffff_ffff_ffff_fff8
                } else {
                    len as u64
                };
                c.g[A2][0] = seed as u64 * 0x1_2345;
            },
        );
    }

    driven!(gen_crc16, ee::s14::f_003e62d0__generated);

    driven!(hand_crc16, ea_crc16);

    /// `EA_Crc16` with real events on the byte-loop head (no calls;
    /// structured loop, so deliver-only: the extra run asserts the return
    /// to `RA`, never a resume at the head).
    #[test]
    fn ea_crc16_events() {
        check_events(gen_crc16, hand_crc16, 48, &|c, seed| {
            use crate::difftest::events::schedule;
            use crate::difftest::fakes::machine;
            use crate::difftest::{fill, DATA};
            let mut r = machine(c, seed, 0x3e62_d0, 0);
            const BUF: u32 = DATA + 0x204;
            const TAB: u32 = 0x0045_1018;
            fill(c, &mut r, TAB, 128);
            fill(c, &mut r, BUF, 8);
            let mode = seed % 3;
            let mut r2 = Rng(0x3e62_d000 ^ seed as u64);
            let ev = schedule(c, &mut r2, mode, 0x003e_62d0);
            c.pc = 0x003e_62d0;
            c.g[A0][0] = BUF as u64;
            c.g[A1][0] = (8 + seed % 8) as u64;
            c.g[A2][0] = seed as u64;
            c.cycles = 1000;
            if mode != 0 {
                c.next_event = 1000;
            }
            let _ = r;
            ev
        });
        use crate::difftest::{fresh, run_generated};
        let mut c = fresh();
        {
            use crate::difftest::fakes::machine;
            use crate::difftest::{fill, DATA};
            let mut r = machine(&mut c, 0, 0x3e62_d0, 0);
            const BUF: u32 = DATA + 0x204;
            const TAB: u32 = 0x0045_1018;
            fill(&mut c, &mut r, TAB, 128);
            fill(&mut c, &mut r, BUF, 8);
            c.pc = 0x003e_62d0;
            c.g[A0][0] = BUF as u64;
            c.g[A1][0] = 12;
            c.g[A2][0] = 0;
            c.cycles = 1000;
            c.next_event = 1000;
        }
        run_generated(&mut c, ee::s14::f_003e62d0__generated);
        assert_eq!(c.pc, crate::difftest::RA, "byte loop: deliver-only, no unwind");
    }

    // ------------------------------------------------- EA_RunFatalHandlers.

    /// `EA_RunFatalHandlers`: zero, one and two set handlers (`seed % 3`,
    /// positions on higher digits) against `EA_AssertNotBusy` as the handler
    /// leaf. Totals are the generated costs observed in the run: 397
    /// empty, +8 per call.
    #[test]
    fn ea_run_fatal_handlers_matches() {
        use crate::difftest::check_paths;
        use crate::difftest::fakes::machine;
        check_paths(
            ee::s14::f_003e49b8__generated,
            ea_run_fatal_handlers,
            120,
            &[397, 405, 413],
            &|c, seed| {
                machine(c, seed, 0x3e49_b8, 0);
                const TABLE: u32 = 0x0045_0c78;
                const LEAF: u32 = 0x003e_62b0;
                c.bus.write32(0x0045_0da4, 0);
                for i in 0..64u32 {
                    c.bus.write32(TABLE + 4 * i, 0);
                }
                let mode = seed % 3;
                if mode >= 1 {
                    c.bus.write32(TABLE + 4 * (seed / 3 % 64), LEAF);
                }
                if mode >= 2 {
                    c.bus.write32(TABLE + 4 * ((seed / 3 % 64 + 32) % 64), LEAF);
                }
                c.pc = 0x003e_49b8;
            },
        );
    }

    driven!(gen_run_fatal_handlers, ee::s14::f_003e49b8__generated);

    driven!(hand_run_fatal_handlers, ea_run_fatal_handlers);

    /// `EA_RunFatalHandlers` with real events on the slot-walk head (the
    /// handler leaf has no loops, so no mid-call unwind; the walk itself is
    /// structured, so deliver-only: the extra run asserts the return to
    /// `RA`, never a resume at the head).
    #[test]
    fn ea_run_fatal_handlers_events() {
        check_events(
            gen_run_fatal_handlers,
            hand_run_fatal_handlers,
            48,
            &|c, seed| {
                use crate::difftest::events::schedule;
                use crate::difftest::fakes::machine;
                machine(c, seed, 0x3e49_b8, 0);
                const TABLE: u32 = 0x0045_0c78;
                const LEAF: u32 = 0x003e_62b0;
                c.bus.write32(0x0045_0da4, 0);
                for i in 0..64u32 {
                    c.bus.write32(TABLE + 4 * i, 0);
                }
                if seed % 2 == 0 {
                    c.bus.write32(TABLE + 4 * (seed / 2 % 64), LEAF);
                }
                let mode = seed / 2 % 3;
                let mut r = Rng(0x3e49_b800 ^ seed as u64);
                let ev = schedule(c, &mut r, mode, 0x003e_49b8);
                c.pc = 0x003e_49b8;
                c.cycles = 1000;
                if mode != 0 {
                    c.next_event = 1000;
                }
                ev
            },
        );
        use crate::difftest::fakes::machine;
        use crate::difftest::{fresh, run_generated};
        let mut c = fresh();
        machine(&mut c, 0, 0x3e49_b8, 0);
        c.bus.write32(0x0045_0da4, 0);
        c.pc = 0x003e_49b8;
        c.cycles = 1000;
        c.next_event = 1000;
        run_generated(&mut c, ee::s14::f_003e49b8__generated);
        assert_eq!(c.pc, crate::difftest::RA, "slot walk: deliver-only, no unwind");
    }

    // ------------------------------------------------- EA_FatalError.

    /// Gate word (in the machine's RAM) for this module's abort fake.
    const EA_ABORT_TEST_CTRL: u32 = crate::difftest::DATA + 0x3e_0000;

    const EA_ABORT_TEST_MAGIC: u32 = 0x0c13_3300;

    use std::sync::atomic::AtomicUsize;

    static EA_ABORT_TEST_PREV_ABORT: AtomicUsize = AtomicUsize::new(0);

    /// The abort call at the end of `EA_FatalError`, faked to return when
    /// this module's gate word is set (the trap itself cannot run in a
    /// differential test); otherwise whatever was installed before.
    fn ea_abort_fake_abort(c: &mut Cpu) {
        use crate::difftest::fakes::fall_back;
        if c.bus.read32(EA_ABORT_TEST_CTRL) == EA_ABORT_TEST_MAGIC {
            c.pc = c.g[RA][0] as u32;
            return;
        }
        fall_back(c, &EA_ABORT_TEST_PREV_ABORT, ee::s14::f_003e4a10__generated);
    }

    fn ea_abort_install_fake() {
        use crate::difftest::fakes::hook;
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E4A10),
            ea_abort_fake_abort,
            &EA_ABORT_TEST_PREV_ABORT,
        );
    }

    /// `EA_FatalError`: null and formatted messages (`seed % 2`), the
    /// verbose second print on `seed / 8`, argument values on higher digits.
    /// The abort call is faked to return (gate word set); everything up to
    /// the trap is compared.
    #[test]
    fn ea_fatal_error_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image, DATA};
        use crate::game::misc::tests::put_cstr;
        ea_abort_install_fake();
        check(ee::s14::f_003e60e8__generated, ea_fatal_error, 48, &|c, seed| {
            elf_image(c);
            machine(c, seed, 0x3e60_e8, 0);
            c.bus.write32(EA_ABORT_TEST_CTRL, EA_ABORT_TEST_MAGIC);
            const FMT: u32 = DATA + 0x2104;
            const SARG: u32 = DATA + 0x2304;
            put_cstr(c, SARG, b"w\xe9rld");
            put_cstr(c, FMT, b"e=%s! n=%d");
            let verbose = seed / 8 % 2 == 1;
            c.bus.write32(0x0045_100c, verbose as u32);
            c.bus.write32(0x0045_1010, SARG);
            // `%s` always reads the first slot: it stays a valid string;
            // the `%d` varies through the second slot.
            for (i, t) in [A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
                c.g[t][0] = (seed as u64 + 1) * (0x100 + 0x110 * i as u64);
            }
            c.g[A1][0] = SARG as u64;
            for (i, f) in [12, 14, 16, 18].into_iter().enumerate() {
                c.f[f] = 0x3f80_0000 + 0x0011_0000 * i as u32 + seed;
            }
            c.pc = 0x003e_60e8;
            c.g[A0][0] = if seed % 2 == 0 { 0 } else { FMT as u64 };
        });
    }

    // ------------------------------------------------- EA_DebugMessage.

    /// `EA_DebugMessage`: null and formatted messages, the hook table
    /// pointing at a tiny leaf. Both arms return, so both are tested.
    #[test]
    fn ea_debug_message_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image, DATA};
        use crate::game::misc::tests::put_cstr;
        check(ee::s14::f_003e6070__generated, ea_debug_message, 48, &|c, seed| {
            elf_image(c);
            machine(c, seed, 0x3e60_70, 0);
            // The abort-message hook: a tiny leaf that returns while the
            // busy flag is clear.
            c.bus.write32(0x0045_0dd8, 0x003e_62b0);
            c.bus.write32(0x0045_0da4, 0);
            const FMT: u32 = DATA + 0x2104;
            const SARG: u32 = DATA + 0x2304;
            put_cstr(c, SARG, b"w\xe9rld");
            put_cstr(
                c,
                FMT,
                [b"%d".as_slice(), b"%s", b"%d-%d", b"x"][seed as usize % 4],
            );
            for (i, t) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
                c.g[t][0] = (seed as u64 + 1) * (0x100 + 0x110 * i as u64);
            }
            c.g[A1][0] = if seed % 4 == 1 {
                SARG as u64
            } else {
                seed as u64
            };
            for (i, f) in [12, 14, 16, 18].into_iter().enumerate() {
                c.f[f] = 0x3f80_0000 + 0x0011_0000 * i as u32 + seed;
            }
            c.pc = 0x003e_6070;
            c.g[A0][0] = if seed % 8 < 2 { 0 } else { FMT as u64 };
        });
    }

    // ------------------------------------------------- EA_AssertNotBusy.

    /// `EA_AssertNotBusy`: the clear-flag arm (the only returnable one). The
    /// trap arm (`0x450da4` nonzero) ends the machine and is not tested.
    #[test]
    fn ea_assert_not_busy_matches() {
        use crate::difftest::check_paths;
        use crate::difftest::fakes::machine;
        check_paths(
            ee::s14::f_003e62b0__generated,
            ea_assert_not_busy,
            24,
            &[6],
            &|c, seed| {
                machine(c, seed, 0x3e62_b0, 0);
                c.bus.write32(0x0045_0da4, 0);
                c.pc = 0x003e_62b0;
            },
        );
    }
}
