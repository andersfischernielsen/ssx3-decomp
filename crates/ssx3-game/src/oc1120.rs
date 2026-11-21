//! Staging module oc1120 (wave ealib-1): `UStr_*` (15) plus the first
//! `EAFile_*` (5). Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1120`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_bd18, u_str_find, "UStr_Find"),
    (0x003d_bdb0, u_str_cmp, "UStr_Cmp"),
    (0x003d_bdd0, u_str_n_cmp, "UStr_NCmp"),
    (0x003d_be20, u_str_cpy, "UStr_Cpy"),
    (0x003d_be40, u_str_n_cpy, "UStr_NCpy"),
    (0x003d_be90, u_str_from_ascii, "UStr_FromAscii"),
    (
        0x003d_bed0,
        u_str_substitute_args_wrapper,
        "UStr_SubstituteArgs_Wrapper",
    ),
    (0x003d_bef0, u_str_substitute_args, "UStr_SubstituteArgs"),
    (0x003d_bfd8, u_str_format_u_int, "UStr_FormatUInt"),
    (0x003d_c038, u_str_format_int, "UStr_FormatInt"),
    (0x003d_c068, u_str_format_float, "UStr_FormatFloat"),
    (
        0x003d_c148,
        u_str_format_number_field,
        "UStr_FormatNumberField",
    ),
    (0x003d_c9f8, u_str_sprintf, "UStr_sprintf"),
    (
        0x003d_ca40,
        u_str_from_escaped_ascii,
        "UStr_FromEscapedAscii",
    ),
    (0x003d_cb20, u_str_to_escaped_ascii, "UStr_ToEscapedAscii"),
    (0x003d_cbd8, ea_file_startup, "EAFile_Startup"),
    (0x003d_cc88, ea_file_init, "EAFile_Init"),
    (0x003d_cd98, ea_file_calc_mem_size, "EAFile_CalcMemSize"),
    // NOTE: no EAFile_Shutdown row: its CdShutdown call spins on Sony CD
    // hardware calls, so no bounded fixture exists; the port below is
    // exact by inspection against the generated body and is covered by
    // the replay gates instead.
    (
        0x003d_ce90,
        ea_file_get_request_status,
        "EAFile_GetRequestStatus",
    ),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const U_STR_LEN: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::UStr_Len,
    };
    pub(crate) const U_STR_N_CMP: Callee = Callee {
        addr: 0x003d_bdd0,
        f: ee::s14::UStr_NCmp,
    };
    pub(crate) const U_STR_CPY: Callee = Callee {
        addr: 0x003d_be20,
        f: ee::s14::UStr_Cpy,
    };
    pub(crate) const U_STR_N_CPY: Callee = Callee {
        addr: 0x003d_be40,
        f: ee::s14::UStr_NCpy,
    };
    pub(crate) const U_STR_SUBSTITUTE_ARGS: Callee = Callee {
        addr: 0x003d_bef0,
        f: ee::s14::UStr_SubstituteArgs,
    };
    pub(crate) const U_STR_FORMAT_U_INT: Callee = Callee {
        addr: 0x003d_bfd8,
        f: ee::s14::UStr_FormatUInt,
    };
    pub(crate) const U_STR_FORMAT_INT: Callee = Callee {
        addr: 0x003d_c038,
        f: ee::s14::UStr_FormatInt,
    };
    pub(crate) const U_STR_FORMAT_FLOAT: Callee = Callee {
        addr: 0x003d_c068,
        f: ee::s14::UStr_FormatFloat,
    };
    pub(crate) const U_STR_VSPRINTF: Callee = Callee {
        addr: 0x003d_c450,
        f: ee::s14::UStr_vsprintf,
    };
    pub(crate) const EA_FILE_CALC_MEM_SIZE: Callee = Callee {
        addr: 0x003d_cd98,
        f: ee::s14::EAFile_CalcMemSize,
    };
    pub(crate) const EA_FILE_INIT: Callee = Callee {
        addr: 0x003d_cc88,
        f: ee::s14::EAFile_Init,
    };
    pub(crate) const EA_MUTEX_INIT: Callee = Callee {
        addr: 0x003e_5698,
        f: ee::s14::EAMutex_Init,
    };
    pub(crate) const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };
    pub(crate) const EA_FILE_CD_INIT: Callee = Callee {
        addr: 0x003e_4040,
        f: ee::s14::EAFile_CdInit,
    };
    pub(crate) const EA_FILE_PLATFORM_INIT: Callee = Callee {
        addr: 0x003e_3020,
        f: ee::s14::EAFile_PlatformInit,
    };

    /// `fcvt` (newlib), called through the function table.
    pub(crate) const FCVT: u32 = 0x0041_8d48;

    /// `strlen` (newlib), called through the function table.
    pub(crate) const STRLEN: u32 = 0x0041_6810;

    /// `sprintf` (newlib), called through the function table.
    pub(crate) const SPRINTF: u32 = 0x0041_7828;

    /// `__fixunsdfsi` (gcc), called through the function table.
    pub(crate) const FIXUNSDFSI: u32 = 0x0041_26a0;

    /// `__fixdfsi` (gcc), called through the function table.
    pub(crate) const FIXDFSI: u32 = 0x0041_2608;

    /// `__cmpdf2` (gcc), called through the function table.
    pub(crate) const CMPDF2: u32 = 0x0041_2500;
}

/// `u16 *UStr_Find(haystack, needle)` (0x3dbd18): the first position in
/// `haystack` where `needle` occurs, in UTF-16 units from `haystack`, or
/// -1 when `needle` is empty or absent. Compares `strlen(needle)` units at
/// each haystack position starting with the haystack's own first unit.
pub fn u_str_find(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_bd58;
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[A0][0] = c.g[S2][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_bd48);
        c.g[S3][0] = 0;
        callee::U_STR_LEN.call(c, 0x003d_bd48)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[S0][0];
        let mut found = false;
        if !empty {
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003d_bd68);
                c.g[A2][0] = c.g[S4][0];
                callee::U_STR_N_CMP.call(c, 0x003d_bd68)?;
                let mismatch = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S3][0];
                    found = true;
                    break;
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UStr_Cmp(a, b)` (0x3dbdb0): compare two UTF-16 strings with no
/// effective length limit (0x7fffffff units).
pub fn u_str_cmp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x7fff0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A2][0] | 0xffff;
        c.g[RA][0] = sx(0x003d_bdc4);
        callee::U_STR_N_CMP.call(c, 0x003d_bdc4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UStr_NCmp(a, b, max)` (0x3dbdd0): compare at most `max` UTF-16
/// units. 0 when they match (or `max` runs out); otherwise the first
/// difference, `a`-unit minus `b`-unit.
///
/// Dispatcher-shaped in the original (`0x3dbde8` is the loop head): it can
/// be re-entered there after an event unwind, so the port keeps the
/// dispatch and its state in the registers.
pub fn u_str_n_cmp(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003d_bdd0 => {
                c.cycles += 3;
                c.g[T0][0] = 0;
                let done = (c.g[A2][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if done { 0x003d_be14 } else { 0x003d_bddc };
            }
            0x003d_bddc => {
                c.cycles += 2;
                c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
                pc = 0x003d_be08;
            }
            0x003d_bde4 => {
                c.cycles += 1;
                pc = 0x003d_bde8;
            }
            0x003d_bde8 => {
                c.cycles += 5;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let done = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                pc = if done { 0x003d_be14 } else { 0x003d_bdfc };
            }
            0x003d_bdfc => {
                c.cycles += 2;
                let done = c.g[A3][0] != 0;
                pc = if done { 0x003d_be14 } else { 0x003d_be04 };
            }
            0x003d_be04 => {
                c.cycles += 1;
                c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
                pc = 0x003d_be08;
            }
            0x003d_be08 => {
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                let more = c.g[V1][0] != 0;
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if more {
                    if event_due(c) && back_edge_unwinds(c, 0x003d_bde8) {
                        return;
                    }
                    pc = 0x003d_bde8;
                } else {
                    pc = 0x003d_be14;
                }
            }
            0x003d_be14 => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[A3][0];
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003d_bdd0, 0x003d_be1c);
                return;
            }
        }
    }
}

/// `u16 *UStr_Cpy(dst, src)` (0x3dbe20): copy a UTF-16 string with no
/// effective length limit (0x7fffffff units). Returns the end of the copy.
pub fn u_str_cpy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x7fff0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A2][0] | 0xffff;
        c.g[RA][0] = sx(0x003d_be34);
        callee::U_STR_N_CPY.call(c, 0x003d_be34)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_NCpy(dst, src, max)` (0x3dbe40): copy at most `max` UTF-16
/// units, stopping at the source terminator, and NUL-terminate the copy.
/// Returns the end of the copied units (`dst + 2 * units`).
pub fn u_str_n_cpy(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_be58;
    c.cycles += 2;
    let done = (c.g[A2][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !done {
        c.cycles += 3;
        c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[T0][0] = c.g[A0][0];
        if !empty {
            c.cycles += 1;
            c.g[V1][0] = c.lhu(c.g[A1][0] as u32);
            loop {
                c.cycles += 6;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.sh(c.g[T0][0] as u32, c.g[V1][0]);
                let full = c.g[V0][0] == 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2));
                if full {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lhu(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[V0][0] as u32, 0);
    c.pc = ra;
}

/// `u16 *UStr_FromAscii(dst, src)` (0x3dbe90): expand an ASCII (byte)
/// string to UTF-16, terminator included. Returns the end of the copy and
/// leaves the unit count in a2.
pub fn u_str_from_ascii(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_bea0;
    c.cycles += 3;
    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = 0;
    if !empty {
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sh(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[V0][0] as u32, 0);
    c.pc = ra;
}

/// `u16 *UStr_SubstituteArgs_Wrapper(dst, fmt, arg1, arg2)` (0x3dbed0):
/// substitute `%1`/`%2` in `fmt` with `arg1`/`arg2` (see
/// [`u_str_substitute_args`]), with the fourth slot cleared.
pub fn u_str_substitute_args_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x003d_bee0);
        callee::U_STR_SUBSTITUTE_ARGS.call(c, 0x003d_bee0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_SubstituteArgs(dst, fmt, arg1, arg2)` (0x3dbef0): copy the
/// UTF-16 `fmt` to `dst`, replacing `%1` with `arg1` and `%2` with `arg2`.
/// Any other `%x` pair copies the `%` and re-reads `x` next round.
/// Returns the caller's `dst`.
pub fn u_str_substitute_args(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_bf38;
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        if !empty {
            c.cycles += 3;
            c.g[S6][0] = 0x25;
            c.g[S5][0] = 0x31;
            c.g[S4][0] = 0x32;
            loop {
                'b_3dbf9c: {
                    'b_3dbf90: {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[S6][0] {
                            c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                            break 'b_3dbf90;
                        }
                        'b_3dbf7c: {
                            'b_3dbf60: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                                let not_one = c.g[V0][0] != c.g[S5][0];
                                c.g[A0][0] = c.g[S0][0];
                                if not_one {
                                    break 'b_3dbf60;
                                }
                                c.cycles += 5;
                                c.g[RA][0] = sx(0x003d_bf54);
                                c.g[A1][0] = c.g[S3][0];
                                callee::U_STR_CPY.call(c, 0x003d_bf54)?;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S3][0];
                                break 'b_3dbf7c;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[S4][0] {
                                c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                                break 'b_3dbf90;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x003d_bf74);
                            c.g[A1][0] = c.g[S2][0];
                            callee::U_STR_CPY.call(c, 0x003d_bf74)?;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[S2][0];
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x003d_bf84);
                        callee::U_STR_LEN.call(c, 0x003d_bf84)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        break 'b_3dbf9c;
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                    c.sh(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                    break 'b_3dbf9c;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 11;
        c.sh(c.g[S0][0] as u32, 0);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_FormatUInt(dst, value, base, digits)` (0x3dbfd8): write
/// `value` in `base` at `dst`, going backwards, using the `digits` table at
/// a3 (indexed by twice the remainder). A zero `value` writes one digit.
/// Returns the start of the written units.
pub fn u_str_format_u_int(c: &mut Cpu) {
    c.cycles += 2;
    let nonzero = c.g[A1][0] != 0;
    c.g[T0][0] = 0;
    if !nonzero {
        c.cycles += 4;
        c.g[V0][0] = c.lhu(c.g[A3][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2));
        c.sh(c.g[A0][0] as u32, c.g[V0][0]);
    } else {
        const HEAD: u32 = 0x003d_bff8;
        c.cycles += 2;
        c.divu(0, 5, 6);
        loop {
            c.cycles += 2;
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = c.hi[0];
            c.g[V1][0] = c.lo[0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
            c.sh(c.g[A0][0] as u32, c.g[V1][0]);
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                break;
            }
            c.divu(0, 5, 6);
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `u16 *UStr_FormatInt(dst, value, base, digits)` (0x3dc038): format the
/// absolute value (see [`u_str_format_u_int`]); the caller writes any sign.
pub fn u_str_format_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V1][0] = c.g[A1][0];
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_c058);
        if c.g[V0][0] == 0 {
            c.g[A1][0] = c.g[V1][0];
        }
        callee::U_STR_FORMAT_U_INT.call(c, 0x003d_c058)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_FormatFloat(dst, bits, precision)` (0x3dc068): convert the
/// float in `bits` with `fcvt`, then copy its decimal digits backwards to
/// `dst`, inserting `.` after `precision` digits and padding the integer
/// part with `0`. Returns the start of the written units (`dst` unchanged
/// when the value needs no digits).
pub fn u_str_format_float(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_c0d8;
        c.cycles += 25;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[A3][0] = c.g[SP][0] | 0x4;
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S4, 0x20);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003d_c0a8);
        c.g[S4][0] = 0;
        c.pc = 0x0041_8d48;
        call_ptr(c, callee::FCVT, 0x003d_c0a8)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003d_c0b4);
        c.g[A0][0] = c.g[S3][0];
        c.pc = 0x0041_6810;
        call_ptr(c, callee::STRLEN, 0x003d_c0b4)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.g[V0][0];
            c.g[T0][0] = 0x2e;
            c.g[A3][0] = 0x30;
            loop {
                c.cycles += 2;
                let at_end = c.g[S1][0] != c.g[S2][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[S4][0] as u32));
                if !at_end {
                    // Dot: leaves the loop directly, without the
                    // digit counter step below.
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                    c.sh(c.g[S0][0] as u32, c.g[T0][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let no_more = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    if no_more {
                        c.cycles += 5;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.sh(c.g[S0][0] as u32, c.g[V0][0]);
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                        c.sh(c.g[S0][0] as u32, c.g[A3][0]);
                    }
                    c.cycles += 1;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                let more = c.g[S1][0] != c.g[A1][0];
                c.g[V0][0] = c.g[S0][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_FormatNumberField(dst, value, base, flags, width, prec, ...)`
/// (0x3dc148): format one `printf`-style field. Flags bit 0x40 selects
/// floating point (`fcvt` via [`u_str_format_float`], 0x80/0x100 pick the
/// call), otherwise 0x20 selects unsigned ([`u_str_format_u_int`] after
/// `__fixunsdfsi`) or signed ([`u_str_format_int`] after `__fixdfsi`).
/// Then zero padding, the sign/`0x`/space prefix, width padding and the
/// digits are written. Returns the end of the written field.
pub fn u_str_format_number_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x1a0);
        frame.save(c, S0, 0x190);
        frame.save(c, S1, 0x180);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x170);
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xfe));
        frame.save(c, S4, 0x150);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S5, 0x140);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, S6, 0x130);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, FP, 0x110);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S7, 0x120);
        c.g[FP][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[A2][0] = c.g[T1][0];
        frame.save(c, S3, 0x160);
        c.g[V0][0] = c.g[S0][0] & 0x40;
        c.g[S3][0] = c.lw(c.g[FP][0] as u32);
        let int_path = c.g[V0][0] == 0;
        c.sh((c.g[SP][0] as u32).wrapping_add(0xfe), 0);
        if int_path {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x20;
            let signed = c.g[V0][0] == 0;
            if !signed {
                c.cycles += 9;
                c.g[RA][0] = sx(0x003d_c1fc);
                c.g[A0][0] = c.g[S2][0];
                c.pc = 0x0041_26a0;
                call_ptr(c, callee::FIXUNSDFSI, 0x003d_c1fc)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x003d_c210);
                c.g[A2][0] = c.g[S6][0];
                callee::U_STR_FORMAT_U_INT.call(c, 0x003d_c210)?;
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003d_c220);
                c.g[A0][0] = c.g[S2][0];
                c.pc = 0x0041_2608;
                call_ptr(c, callee::FIXDFSI, 0x003d_c220)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x003d_c234);
                c.g[A2][0] = c.g[S6][0];
                callee::U_STR_FORMAT_INT.call(c, 0x003d_c234)?;
                c.g[S1][0] = c.g[V0][0];
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x80;
            let no_float_first = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[S0][0] & 0x100;
            if !no_float_first {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_c1b8);
                c.g[A0][0] = c.g[S1][0];
                callee::U_STR_FORMAT_FLOAT.call(c, 0x003d_c1b8)?;
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                let no_float_second = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !no_float_second {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_c1d0);
                    c.g[A1][0] = c.g[S2][0];
                    callee::U_STR_FORMAT_FLOAT.call(c, 0x003d_c1d0)?;
                    c.g[S1][0] = c.g[V0][0];
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_c1e0);
                    c.g[A1][0] = c.g[S2][0];
                    callee::U_STR_FORMAT_FLOAT.call(c, 0x003d_c1e0)?;
                    c.g[S1][0] = c.g[V0][0];
                }
            }
        }
        // Zero padding to the width.
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0] & 0x18;
        c.g[V0][0] = 0x8;
        let no_zero_pad = c.g[V1][0] != c.g[V0][0];
        c.g[S7][0] = c.g[S0][0] & 0x10;
        if !no_zero_pad {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_c254);
            c.g[S4][0] = 0x30;
            callee::U_STR_LEN.call(c, 0x003d_c254)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S5][0] as i64)) as u64;
            let fits = c.g[V0][0] == 0;
            c.g[V0][0] = 0xa;
            if !fits {
                const HEAD: u32 = 0x003d_c268;
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                loop {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V1][0] != 0;
                    c.sh(c.g[S1][0] as u32, c.g[S4][0]);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0xa;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0xa;
        }
        // Sign / prefix.
        c.cycles += 2;
        let no_prefix = c.g[S6][0] != c.g[V0][0];
        c.g[V0][0] = 0x8;
        // Only the full chain below takes the block tail (cycles + 1,
        // v0 = 8); the skips jump straight to the `0` flag.
        let mut prefix_tail = !no_prefix;
        if !no_prefix {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x20;
            let unsigned = c.g[V0][0] != 0;
            c.g[V0][0] = 0x8;
            if !unsigned {
                c.cycles += 6;
                c.g[S4][0] = 0;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003d_c2ac);
                c.g[A1][0] = c.g[S4][0];
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c2ac)?;
                let non_negative = (c.g[V0][0] as i64) >= 0;
                c.g[A1][0] = c.g[S4][0];
                if !non_negative {
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = 0x2d;
                    c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_c2c8);
                c.g[A0][0] = c.g[S2][0];
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c2c8)?;
                let positive = (c.g[V0][0] as i64) > 0;
                c.g[V0][0] = c.g[S0][0] & 0x1;
                if positive {
                    c.cycles += 2;
                    let no_plus = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x2b;
                    if !no_plus {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                        c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003d_c2ec);
                c.g[A1][0] = 0;
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c2ec)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !nonzero {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S0][0] & 0x1;
                    let no_space = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x20;
                    if !no_space {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                        c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_c314);
                c.g[A1][0] = 0;
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c314)?;
                let negative = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = c.g[S0][0] & 0x5;
                if !negative {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let no_alt = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8;
                    if !no_alt {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                        c.g[V0][0] = 0x20;
                        c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                    } else {
                        prefix_tail = false;
                    }
                }
            } else {
                prefix_tail = false;
            }
        }
        if prefix_tail {
            c.cycles += 1;
            c.g[V0][0] = 0x8;
        }
        // `0` flag without `0x` prefix: one `0`.
        c.cycles += 2;
        let no_zero = c.g[S6][0] != c.g[V0][0];
        c.g[V0][0] = 0x10;
        if !no_zero {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x2;
            let no_zero_flag = c.g[V0][0] == 0;
            c.g[V0][0] = 0x30;
            if !no_zero_flag {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                c.sh(c.g[S1][0] as u32, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 0x10;
        }
        // `0x` prefix.
        c.cycles += 2;
        let no_hex = c.g[S6][0] != c.g[V0][0];
        c.g[V0][0] = c.g[S0][0] & 0x2;
        if !no_hex {
            c.cycles += 2;
            let no_hex_flag = c.g[V0][0] == 0;
            c.g[V0][0] = 0x78;
            if !no_hex_flag {
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = 0x30;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                c.sh(c.g[S1][0] as u32, c.g[V1][0]);
            }
        }
        // Space padding to the width, then copy the digits in.
        c.cycles += 2;
        let no_space_pad = c.g[S7][0] != 0;
        c.g[V1][0] = c.g[S0][0] & 0x8;
        if !no_space_pad {
            c.cycles += 9;
            c.g[V0][0] = 0x20;
            c.g[S0][0] = 0x30;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_c398);
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[V0][0];
            }
            callee::U_STR_LEN.call(c, 0x003d_c398)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S5][0] as i64)) as u64;
            let fits = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if !fits {
                const HEAD: u32 = 0x003d_c3a8;
                loop {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V1][0] != 0;
                    c.sh(c.g[S1][0] as u32, c.g[S0][0]);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x003d_c3cc);
        c.g[A0][0] = c.g[S1][0];
        callee::U_STR_LEN.call(c, 0x003d_c3cc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x003d_c3dc);
        c.g[A1][0] = c.g[S1][0];
        callee::U_STR_CPY.call(c, 0x003d_c3dc)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
        let pad_rest = c.g[S7][0] == 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !pad_rest {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
            let fits = c.g[V0][0] == 0;
            c.g[V1][0] = 0x20;
            if !fits {
                const HEAD: u32 = 0x003d_c3f8;
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                loop {
                    c.cycles += 7;
                    c.sh(c.g[S3][0] as u32, c.g[V1][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[S0][0] != 0;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2));
                    if !more {
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S5][0];
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
        }
        c.cycles += 14;
        c.sw(c.g[FP][0] as u32, c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x190);
        frame.restore(c, S1, 0x180);
        frame.restore(c, S2, 0x170);
        frame.restore(c, S3, 0x160);
        frame.restore(c, S4, 0x150);
        frame.restore(c, S5, 0x140);
        frame.restore(c, S6, 0x130);
        frame.restore(c, S7, 0x120);
        frame.restore(c, FP, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_sprintf(dst, fmt, ...)` (0x3dc9f8): format into `dst` (see
/// `UStr_vsprintf`), spilling the register and FPU varargs onto the stack
/// for it. Returns `dst`.
pub fn u_str_sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
        frame.save_fpr(c, 12, 0x40);
        frame.save_fpr(c, 14, 0x44);
        frame.save_fpr(c, 16, 0x48);
        c.g[RA][0] = sx(0x003d_ca30);
        frame.save_fpr(c, 18, 0x4c);
        callee::U_STR_VSPRINTF.call(c, 0x003d_ca30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_FromEscapedAscii(dst, src)` (0x3dca40): expand ASCII `src`
/// to UTF-16, turning `\x` + up to 4 lowercase hex digits into the byte
/// they encode (a NUL or a non-hex digit ends the digits early; a lone
/// `\`, `\x` NUL or invalid digit copies the `\` and moves one byte).
/// Returns the end of the copy.
pub fn u_str_from_escaped_ascii(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_ca58;
    const INNER: u32 = 0x003d_ca98;
    c.cycles += 3;
    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
    let empty = c.g[V0][0] == 0;
    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
    if !empty {
        c.cycles += 3;
        c.g[T4][0] = 0x5c;
        c.g[T3][0] = 0x78;
        loop {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
            c.g[T1][0] = 0;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V1][0] = 0;
            let not_escape = c.g[V0][0] != c.g[T4][0];
            c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            if !not_escape {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x1));
                let not_x = c.g[V0][0] != c.g[T3][0];
                if !not_x {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x2));
                    let nul_after_x = c.g[V0][0] == 0;
                    c.g[T1][0] = 0x1;
                    if !nul_after_x {
                        c.cycles += 4;
                        c.g[A3][0] = c.g[A1][0];
                        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                        loop {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                            let not_digit = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[A2][0] & 0xff;
                            if not_digit {
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x61));
                                c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[T1][0] = 0;
                                    break;
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57));
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                            }
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            let full = (c.g[A3][0] as i64) >= (c.g[T0][0] as i64);
                            if full {
                                break;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x2));
                            if c.g[V0][0] != 0 {
                                c.g[A2][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                if event_due(c) {
                                    deliver_event_here(c, INNER);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            if c.g[T1][0] == 0 {
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.cycles += 2;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.sh(c.g[A0][0] as u32, c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.sh(c.g[A0][0] as u32, c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6));
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[A0][0] as u32, 0);
    c.pc = ra;
}

/// `char *UStr_ToEscapedAscii(dst, src)` (0x3dcb20): copy UTF-16 `src` to
/// ASCII `dst`, formatting every unit at or above 0x100 and every `\x`
/// pair with `sprintf`. Returns the end of the copy.
pub fn u_str_to_escaped_ascii(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_cb60;
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !empty {
            c.cycles += 5;
            c.g[S4][0] = 0x5c;
            c.g[S3][0] = 0x78;
            c.g[S2][0] = 0x490000;
            c.g[V1][0] = c.g[V0][0] & 0xffff;
            loop {
                c.cycles += 3;
                let wide = c.g[V1][0] >= 0x100;
                let mut use_sprintf = wide;
                if !wide {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[S4][0] {
                        c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                        if c.g[V0][0] != c.g[S3][0] {
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                        } else {
                            c.cycles += 1;
                            use_sprintf = true;
                        }
                    }
                }
                if use_sprintf {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.lhu(c.g[S1][0] as u32);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5d18));
                    c.g[RA][0] = sx(0x003d_cb94);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6));
                    c.pc = 0x0041_7828;
                    call_ptr(c, callee::SPRINTF, 0x003d_cb94)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                } else {
                    c.cycles += 3;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[V0][0] & 0xffff;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 9;
        c.sb(c.g[S0][0] as u32, 0);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int EAFile_Startup(a0, a1, a2)` (0x3dcbd8): size the file layer (see
/// [`ea_file_calc_mem_size`]), allocate it through the callback at `+4` of
/// the `0x519ab0` block, and initialise it (see [`ea_file_init`]).
/// Returns 0 when the `0x519ab0` block says it already started.
pub fn ea_file_startup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x520000;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6550));
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let fresh = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        if !fresh {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 20;
            c.g[S0][0] = 0x450000;
            c.g[S1][0] = 0x490000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc10));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5d28));
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003d_cc34);
            c.g[A2][0] = c.g[S3][0];
            callee::EA_FILE_CALC_MEM_SIZE.call(c, 0x003d_cc34)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            let alloc = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x003d_cc48);
            c.g[A2][0] = 0x100;
            c.pc = alloc;
            call_ptr(c, alloc, 0x003d_cc48)?;
            c.g[V1][0] = 0x520000;
            c.g[A0][0] = c.g[S4][0];
            c.sw((c.g[V1][0] as u32).wrapping_sub(0x6518), c.g[V0][0]);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x003d_cc64);
            c.g[A3][0] = c.g[V0][0];
            callee::EA_FILE_INIT.call(c, 0x003d_cc64)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int EAFile_Init(a0, a1, a2, a3)` (0x3dcc88): set up the `0x519ab0`
/// file-layer block (mutex, allocator sizes, zeroed request area) and the
/// CD layer unless `0x450c60` says to skip it. Returns 1, or 0 when the
/// block says it already started.
pub fn ea_file_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = 0x520000;
        frame.save(c, S1, 0x50);
        frame.save(c, S4, 0x20);
        c.g[S1][0] = 0x20;
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 0x3e8;
        frame.save(c, S0, 0x60);
        c.g[S5][0] = 0x10;
        frame.save(c, S3, 0x30);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.g[S5][0] = c.g[A0][0];
        }
        if c.g[A1][0] != 0 {
            c.g[S4][0] = c.g[A1][0];
        }
        let started = c.g[V0][0] != 0;
        if c.g[A2][0] != 0 {
            c.g[S1][0] = c.g[A2][0];
        }
        if !started {
            c.cycles += 24;
            c.g[V0][0] = 0xff;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_sub(0x6550), c.g[S1][0]);
            c.g[RA][0] = sx(0x003d_ccec);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S5][0]);
            callee::EA_MUTEX_INIT.call(c, 0x003d_ccec)?;
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x003d_cd00);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S3][0]);
            callee::EA_FILE_CALC_MEM_SIZE.call(c, 0x003d_cd00)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[A2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003d_cd10);
            c.g[A1][0] = 0;
            callee::EA_MEM_SET.call(c, 0x003d_cd10)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6550));
            c.g[A0][0] = 0x30;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = 0x450000;
            c.mult(0, 2, 2, 4);
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc60));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let skip_cd = c.g[A1][0] == 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
            if !skip_cd {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x110;
                c.g[A0][0] = c.g[S4][0];
                c.mult(0, 3, 2, 5);
                c.g[RA][0] = sx(0x003d_cd4c);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                callee::EA_FILE_CD_INIT.call(c, 0x003d_cd4c)?;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003d_cd64);
            c.g[A0][0] = c.g[S5][0];
            callee::EA_FILE_PLATFORM_INIT.call(c, 0x003d_cd64)?;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_CalcMemSize(a0, a1, a2)` (0x3dcd98): the request-area bytes
/// for the given queue counts, or the defaults (0x10/0x20/0x3e8) where an
/// argument is 0, plus the table and header overhead.
pub fn ea_file_calc_mem_size(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = 0x450000;
    c.g[A3][0] = 0x10;
    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc5c));
    if c.g[A0][0] != 0 {
        c.g[A3][0] = c.g[A0][0];
    }
    c.g[V0][0] = 0x20;
    c.g[V1][0] = 0x3e8;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.g[A0][0] = 0x30;
    c.g[A2][0] = 0x110;
    c.mult(0, 2, 2, 4);
    c.mult(0, 7, 7, 6);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
    c.mult(0, 3, 3, 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}

/// `u32 EAFile_GetRequestStatus(request)` (0x3dce90): the status nibble of
/// `request` (bits 16..20 of the request word ^ the handle), or -3 when the
/// status word is clear; with the word at `sp+4` it then loads the
/// request's status field.
pub fn ea_file_get_request_status(c: &mut Cpu) {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A3][0] = c.g[A0][0];
    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
    c.g[V0][0] = 0;
    let null = c.g[A3][0] == 0;
    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
    if !null {
        c.cycles += 12;
        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = 0x30;
        c.g[A2][0] = 0x520000;
        c.g[A0][0] = 0xf0000;
        c.mult(0, 3, 3, 2);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x6528));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    }
    c.cycles += 2;
    let clear = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
    if !clear {
        c.cycles += 9;
        c.g[A1][0] = 0x30;
        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = 0x520000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6528));
        c.mult(0, 3, 3, 5);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0xfffffffffffffffd;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Write `units` as UTF-16 at `addr`.
    fn put_units(c: &mut Cpu, addr: u32, units: &[u16]) {
        for (i, &u) in units.iter().enumerate() {
            c.bus.write16(addr + 2 * i as u32, u);
        }
    }

    /// Write `s` as bytes at `addr`, NUL-terminated.
    fn put_ascii(c: &mut Cpu, addr: u32, s: &str) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// Haystack/needle pairs: found at 0, found late, absent, empty
    /// needle, needle longer than the haystack. Plain `check`: the call
    /// into the dispatcher-shaped `UStr_NCmp` cannot unwind mid-call in
    /// the test machine, so the wrapper is compared without events (its
    /// own loop still runs every arm; `UStr_NCmp`'s event test covers
    /// re-entry at its head).
    #[test]
    fn u_str_find_matches() {
        check(
            ee::s14::f_003dbd18__generated,
            u_str_find,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_bd18, 0);
                let hay = DATA;
                let ndl = DATA + 0x400;
                let hlen = 1 + seed % 20;
                let mut nlen = seed / 20 % 9;
                let mode = seed / 180 % 4;
                if mode == 3 {
                    nlen = 0;
                }
                let mut needle = vec![0u16; nlen as usize];
                for u in needle.iter_mut() {
                    let b = r.next_u32();
                    *u = (1 + b % 0x2ff) as u16;
                }
                let mut haystack = vec![0u16; hlen as usize];
                for u in haystack.iter_mut() {
                    let b = r.next_u32();
                    *u = (1 + b % 0x2ff) as u16;
                }
                if mode == 0 && nlen <= hlen && nlen > 0 {
                    haystack[..nlen as usize].copy_from_slice(&needle);
                }
                if mode == 1 && nlen <= hlen && nlen > 0 {
                    haystack[hlen as usize - nlen as usize..].copy_from_slice(&needle);
                }
                put_units(c, hay, &haystack);
                c.bus.write16(hay + 2 * hlen, 0);
                put_units(c, ndl, &needle);
                c.bus.write16(ndl + 2 * nlen, 0);
                // Pre-fill past the ends so an over-read shows.
                c.bus.write16(hay + 2 * hlen + 2, 0xbeef);
                c.bus.write16(ndl + 2 * nlen + 2, 0xbeef);
                c.g[A0][0] = sx(hay);
                c.g[A1][0] = sx(ndl);
                c.pc = 0x003d_bd18;
            },
        );
    }

    /// Equal, early/late difference, prefix, empty strings. Plain `check`
    /// for the same dispatcher-callee reason as [`u_str_find_matches`].
    #[test]
    fn u_str_cmp_matches() {
        check(
            ee::s14::f_003dbdb0__generated,
            u_str_cmp,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_bdb0, 0);
                let len = seed % 24;
                let mode = seed / 24 % 5;
                let mut a = vec![0u16; len as usize];
                for u in a.iter_mut() {
                    *u = (1 + r.next_u32() % 0x2ff) as u16;
                }
                let mut b = a.clone();
                match mode {
                    1 => {
                        if len > 0 {
                            b[0] = b[0].wrapping_add(1);
                        }
                    }
                    2 => {
                        if len > 0 {
                            b[len as usize - 1] = b[len as usize - 1].wrapping_add(1);
                        }
                    }
                    3 => {
                        b.push(0x41);
                    }
                    4 => {
                        a.clear();
                    }
                    _ => {}
                }
                put_units(c, DATA, &a);
                c.bus.write16(DATA + 2 * a.len() as u32, 0);
                put_units(c, DATA + 0x400, &b);
                c.bus.write16(DATA + 0x400 + 2 * b.len() as u32, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.pc = 0x003d_bdb0;
            },
        );
    }

    driven!(gen_u_str_n_cmp, ee::s14::f_003dbdd0__generated);
    driven!(hand_u_str_n_cmp, u_str_n_cmp);

    /// Bounded compares: max 0, max below/above the difference point,
    /// NUL inside the window, empty inputs.
    #[test]
    fn u_str_n_cmp_events() {
        check_events(gen_u_str_n_cmp, hand_u_str_n_cmp, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_bdd0, 0);
            let len = 1 + seed % 16;
            let max = [0, 1, 3, 7, 100][seed as usize / 17 % 5];
            let mode = seed / 85 % 5;
            let mut a = vec![0u16; len as usize];
            for u in a.iter_mut() {
                *u = (1 + r.next_u32() % 0x2ff) as u16;
            }
            let mut b = a.clone();
            match mode {
                1 => {
                    b[0] = b[0].wrapping_add(1);
                }
                2 => {
                    let i = len as usize / 2;
                    b[i] = b[i].wrapping_add(1);
                }
                3 => {
                    a[len as usize / 2] = 0;
                }
                4 => {
                    a.clear();
                    b.clear();
                }
                _ => {}
            }
            put_units(c, DATA, &a);
            c.bus.write16(DATA + 2 * a.len() as u32, 0);
            put_units(c, DATA + 0x400, &b);
            c.bus.write16(DATA + 0x400 + 2 * b.len() as u32, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = max;
            schedule(c, &mut r, seed, 0x003d_bdd0)
        });
    }

    driven!(gen_u_str_cpy, ee::s14::f_003dbe20__generated);
    driven!(hand_u_str_cpy, u_str_cpy);

    /// Copies of empty, short and 23-unit strings; the destination is
    /// pre-filled so a missing store shows.
    #[test]
    fn u_str_cpy_events() {
        check_events(gen_u_str_cpy, hand_u_str_cpy, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_be20, 0);
            let len = seed % 24;
            let mut s = vec![0u16; len as usize];
            for u in s.iter_mut() {
                *u = (1 + r.next_u32() % 0x2ff) as u16;
            }
            put_units(c, DATA + 0x400, &s);
            c.bus.write16(DATA + 0x400 + 2 * len, 0);
            for i in 0..32u32 {
                c.bus.write16(DATA + 2 * i, 0xbeef);
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x400);
            schedule(c, &mut r, seed, 0x003d_be20)
        });
    }

    /// Max 0, truncated, exact and over-long copies; empty source.
    /// Split: edge arms (max 0, empty) under `check`, the looping arms
    /// under `check_events` so enough seeds reach the event check.
    #[test]
    fn u_str_n_cpy_edges() {
        check(
            ee::s14::f_003dbe40__generated,
            u_str_n_cpy,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_be40, 0);
                let len = 2 + seed % 6;
                let mut s = vec![0u16; len as usize];
                for u in s.iter_mut() {
                    *u = (1 + r.next_u32() % 0x2ff) as u16;
                }
                if seed % 2 == 0 {
                    s.clear();
                }
                put_units(c, DATA + 0x400, &s);
                c.bus.write16(DATA + 0x400 + 2 * s.len() as u32, 0);
                for i in 0..32u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = 0;
                c.pc = 0x003d_be40;
            },
        );
    }

    driven!(gen_u_str_n_cpy_loop, ee::s14::f_003dbe40__generated);
    driven!(hand_u_str_n_cpy_loop, u_str_n_cpy);

    /// Truncated, exact and over-long copies over 8..31 units; the
    /// destination is pre-filled.
    #[test]
    fn u_str_n_cpy_loop_events() {
        check_events(
            gen_u_str_n_cpy_loop,
            hand_u_str_n_cpy_loop,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_be40, 1);
                let len = 8 + seed % 24;
                let max = [4u32, 9, 17, 100][seed as usize / 32 % 4];
                let mut s = vec![0u16; len as usize];
                for u in s.iter_mut() {
                    *u = (1 + r.next_u32() % 0x2ff) as u16;
                }
                put_units(c, DATA + 0x400, &s);
                c.bus.write16(DATA + 0x400 + 2 * len, 0);
                for i in 0..40u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = max as u64;
                schedule(c, &mut r, seed, 0x003d_be40)
            },
        );
    }

    driven!(gen_u_str_from_ascii, ee::s14::f_003dbe90__generated);
    driven!(hand_u_str_from_ascii, u_str_from_ascii);

    /// Empty, short and 31-byte sources with high bytes; the destination
    /// is pre-filled.
    #[test]
    fn u_str_from_ascii_events() {
        check_events(
            gen_u_str_from_ascii,
            hand_u_str_from_ascii,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_be90, 0);
                let len = seed % 32;
                for i in 0..len {
                    let b = match r.next_u32() % 4 {
                        0 => 0x80 + r.next_u32() % 0x80,
                        _ => 1 + r.next_u32() % 0x7e,
                    };
                    c.bus.write8(DATA + 0x400 + i, b as u8);
                }
                c.bus.write8(DATA + 0x400 + len, 0);
                for i in 0..40u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                schedule(c, &mut r, seed, 0x003d_be90)
            },
        );
    }

    driven!(
        gen_u_str_substitute_args_wrapper,
        ee::s14::f_003dbed0__generated
    );
    driven!(
        hand_u_str_substitute_args_wrapper,
        u_str_substitute_args_wrapper
    );

    /// Formats with no escapes, `%1`/`%2`, `%x`, a trailing `%` and an
    /// empty format; the destinations are pre-filled.
    #[test]
    fn u_str_substitute_args_wrapper_events() {
        check_events(
            gen_u_str_substitute_args_wrapper,
            hand_u_str_substitute_args_wrapper,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_bed0, 0);
                let fmts = [
                    "score ",
                    "p1=%1 ok",
                    "%2/%1",
                    "%1%2%1",
                    "100%x done",
                    "trailing %",
                    "",
                    "%%1",
                ];
                let fmt = fmts[seed as usize / 37 % fmts.len()];
                let mut f: Vec<u16> = fmt.encode_utf16().collect();
                if seed / 296 % 2 == 0 {
                    f.push(b'x' as u16);
                }
                put_units(c, DATA + 0x400, &f);
                c.bus.write16(DATA + 0x400 + 2 * f.len() as u32, 0);
                put_units(c, DATA + 0x800, &[0x41, 0x42, 0]);
                put_units(c, DATA + 0xc00, &[0x58, 0x59, 0x5a, 0]);
                for i in 0..64u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x800);
                schedule(c, &mut r, seed, 0x003d_bed0)
            },
        );
    }

    driven!(gen_u_str_substitute_args, ee::s14::f_003dbef0__generated);
    driven!(hand_u_str_substitute_args, u_str_substitute_args);

    /// Same shapes as the wrapper, with a real second argument.
    #[test]
    fn u_str_substitute_args_events() {
        check_events(
            gen_u_str_substitute_args,
            hand_u_str_substitute_args,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_bef0, 0);
                let fmts = [
                    "score ",
                    "p1=%1 ok",
                    "%2/%1",
                    "%1%2%1",
                    "100%x done",
                    "trailing %",
                    "",
                    "%%2",
                ];
                let fmt = fmts[seed as usize / 37 % fmts.len()];
                let f: Vec<u16> = fmt.encode_utf16().collect();
                put_units(c, DATA + 0x400, &f);
                c.bus.write16(DATA + 0x400 + 2 * f.len() as u32, 0);
                put_units(c, DATA + 0x800, &[0x41, 0x42, 0]);
                put_units(c, DATA + 0xc00, &[0x58, 0x59, 0x5a, 0]);
                for i in 0..64u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x800);
                c.g[A3][0] = sx(DATA + 0xc00);
                schedule(c, &mut r, seed, 0x003d_bef0)
            },
        );
    }

    driven!(gen_u_str_format_u_int, ee::s14::f_003dbfd8__generated);
    driven!(hand_u_str_format_u_int, u_str_format_u_int);

    /// Zero, small and 32-bit values in bases 8/10/16; the destination is
    /// pre-filled and the write runs backwards from its end.
    #[test]
    fn u_str_format_u_int_events() {
        check_events(
            gen_u_str_format_u_int,
            hand_u_str_format_u_int,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_bfd8, 0);
                let bases = [8u32, 10, 16];
                let base = bases[seed as usize % 3];
                let value = match seed as usize / 3 % 6 {
                    0 => 0,
                    1 => seed,
                    2 => 0xffff_ffff,
                    3 => 0x8000_0000,
                    4 => 123456789,
                    _ => r.next_u32(),
                };
                put_ascii(c, DATA + 0x400, "0123456789abcdef");
                for i in 0..40u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA + 80);
                c.g[A1][0] = value as u64;
                c.g[A2][0] = base as u64;
                c.g[A3][0] = sx(DATA + 0x400);
                schedule(c, &mut r, seed, 0x003d_bfd8)
            },
        );
    }

    driven!(gen_u_str_format_int, ee::s14::f_003dc038__generated);
    driven!(hand_u_str_format_int, u_str_format_int);

    /// Negative, zero and positive values (the sign is the caller's job;
    /// this formats the absolute value).
    #[test]
    fn u_str_format_int_events() {
        check_events(
            gen_u_str_format_int,
            hand_u_str_format_int,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_c038, 0);
                let value = match seed as usize % 5 {
                    0 => 0,
                    1 => seed as i32,
                    2 => -(seed as i32),
                    3 => i32::MIN,
                    _ => i32::MAX,
                };
                put_ascii(c, DATA + 0x400, "0123456789abcdef");
                for i in 0..40u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA + 80);
                c.g[A1][0] = value as u32 as u64;
                c.g[A2][0] = 10;
                c.g[A3][0] = sx(DATA + 0x400);
                schedule(c, &mut r, seed, 0x003d_c038)
            },
        );
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Write `units` as UTF-16 at `addr`.
    fn put_units(c: &mut Cpu, addr: u32, units: &[u16]) {
        for (i, &u) in units.iter().enumerate() {
            c.bus.write16(addr + 2 * i as u32, u);
        }
    }

    /// Write `s` as bytes at `addr`, NUL-terminated.
    fn put_ascii(c: &mut Cpu, addr: u32, s: &str) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    driven!(gen_u_str_format_float, ee::s14::f_003dc068__generated);
    driven!(hand_u_str_format_float, u_str_format_float);

    /// Zero, integer, fractional, negative and large magnitudes with
    /// precisions 0..5; the destination is pre-filled.
    #[test]
    fn u_str_format_float_events() {
        check_events(
            gen_u_str_format_float,
            hand_u_str_format_float,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_c068, 0);
                let bits = match seed as usize / 6 % 8 {
                    0 => 0x0000_0000,
                    1 => 0x3f80_0000,
                    2 => 0xbf80_0000,
                    3 => 0x42f6_e979,
                    4 => 0xc2f6_e979,
                    5 => 0x4e6e_6b28,
                    6 => 0x3a83_126f,
                    _ => r.edge_f32(),
                };
                for i in 0..48u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA + 96);
                c.g[A1][0] = bits as u64;
                c.g[A2][0] = (seed % 6) as u64;
                schedule(c, &mut r, seed, 0x003d_c068)
            },
        );
    }

    driven!(
        gen_u_str_format_number_field,
        ee::s14::f_003dc148__generated
    );
    driven!(hand_u_str_format_number_field, u_str_format_number_field);

    /// The flag matrix (0x40 float vs int, 0x80/0x100 float choice, 0x20
    /// unsigned, 0x10/0x8/0x2/0x1 padding and prefix) over bases 8/10/16,
    /// widths, precisions and double magnitudes.
    #[test]
    fn u_str_format_number_field_events() {
        check_events(
            gen_u_str_format_number_field,
            hand_u_str_format_number_field,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_c148, 0);
                let flags = [
                    0x00u32,
                    0x20,
                    0x40,
                    0x40 | 0x80,
                    0x40 | 0x100,
                    0x18 | 0x10,
                    0x08 | 0x20,
                    0x01,
                    0x05,
                    0x02,
                    0x20 | 0x01,
                    0x40 | 0x80 | 0x10,
                ];
                let bases = [8u32, 10, 16];
                let doubles = [
                    0x0000_0000_0000_0000u64,
                    0x3ff0_0000_0000_0000,
                    0xbff0_0000_0000_0000,
                    0x405e_d2f1_a9fc_9722,
                    0xc05e_d2f1_a9fc_9722,
                    0x432d_f8b5_5818_0000,
                    0x3fe0_0000_0000_0000,
                    0xbff0_0000_0000_0001,
                ];
                let flag = flags[seed as usize % flags.len()];
                let base = bases[seed as usize / 12 % 3];
                let width = seed / 36 % 9;
                let prec = seed / 324 % 7;
                let dbl = doubles[seed as usize / 2268 % doubles.len()];
                put_ascii(c, DATA + 0x600, "0123456789abcdef");
                for i in 0..64u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.bus.write32(DATA + 0x100, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = dbl;
                c.g[A2][0] = base as u64;
                c.g[A3][0] = flag as u64;
                c.g[T0][0] = width as u64;
                c.g[T1][0] = prec as u64;
                c.g[T2][0] = sx(DATA + 0x600);
                c.f[12] = dbl as u32;
                c.f[13] = (dbl >> 32) as u32;
                schedule(c, &mut r, seed, 0x003d_c148)
            },
        );
    }

    driven!(gen_u_str_sprintf, ee::s14::f_003dc9f8__generated);
    driven!(hand_u_str_sprintf, u_str_sprintf);

    /// `%d`/`%u`/`%x`/`%s`/`%c` formats with register varargs; the
    /// destination is pre-filled and the game image is loaded for the
    /// formatter's table.
    #[test]
    fn u_str_sprintf_events() {
        check_events(gen_u_str_sprintf, hand_u_str_sprintf, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_c9f8, 0);
            elf_image(c);
            let fmts = [
                "n=%d",
                "u=%u x=%x",
                "s=%s c=%c",
                "%d%d%d",
                "plain",
                "mix %s %d %x",
            ];
            let fmt = fmts[seed as usize % fmts.len()];
            put_ascii(c, DATA + 0x400, fmt);
            put_ascii(c, DATA + 0x800, "hi");
            for i in 0..64u32 {
                c.bus.write16(DATA + 2 * i, 0xbeef);
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = seed as u64;
            c.g[A3][0] = sx(DATA + 0x800);
            c.g[T0][0] = (seed / 7) as u64;
            c.g[T1][0] = 0x41;
            c.g[T2][0] = (seed / 13 % 256) as u64;
            c.g[T3][0] = 0x1234_5678;
            c.f[12] = seed;
            schedule(c, &mut r, seed, 0x003d_c9f8)
        });
    }

    driven!(gen_u_str_from_escaped_ascii, ee::s14::f_003dca40__generated);
    driven!(hand_u_str_from_escaped_ascii, u_str_from_escaped_ascii);

    /// Plain text, valid `\xHH` escapes (1-4 digits), lone backslashes,
    /// `\x` NUL, invalid hex and empty input.
    #[test]
    fn u_str_from_escaped_ascii_events() {
        check_events(
            gen_u_str_from_escaped_ascii,
            hand_u_str_from_escaped_ascii,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_ca40, 0);
                let srcs = [
                    "plain",
                    "\\x41\\x42",
                    "a\\x43b",
                    "lone\\backslash",
                    "bad\\xzz",
                    "\\x41",
                    "trail\\",
                    "\\",
                    "",
                    "\\x41424344",
                    "mix\\x41z\\x42",
                ];
                let mut src = srcs[seed as usize % srcs.len()].to_string();
                if seed / 11 % 3 == 0 {
                    src.push((b'a' + seed as u8 % 26) as char);
                }
                put_ascii(c, DATA + 0x400, &src);
                for i in 0..48u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                schedule(c, &mut r, seed, 0x003d_ca40)
            },
        );
    }

    driven!(gen_u_str_to_escaped_ascii, ee::s14::f_003dcb20__generated);
    driven!(hand_u_str_to_escaped_ascii, u_str_to_escaped_ascii);

    /// ASCII, wide units, `\x` pairs, lone backslashes and empty input;
    /// the game image is loaded for the `%`-format string.
    #[test]
    fn u_str_to_escaped_ascii_events() {
        check_events(
            gen_u_str_to_escaped_ascii,
            hand_u_str_to_escaped_ascii,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_cb20, 0);
                elf_image(c);
                let srcs: &[&[u16]] = &[
                    &[0x41, 0x42, 0x43],
                    &[0x141, 0x42],
                    &[0x5c, 0x78, 0x41],
                    &[0x5c, 0x41],
                    &[0x5c],
                    &[],
                    &[0x20, 0x100, 0x5c, 0x78],
                    &[0x41, 0x5c, 0x78, 0x42],
                ];
                let src = srcs[seed as usize % srcs.len()];
                put_units(c, DATA + 0x400, src);
                c.bus.write16(DATA + 0x400 + 2 * src.len() as u32, 0);
                for i in 0..64u32 {
                    c.bus.write8(DATA + i, 0xa5);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                schedule(c, &mut r, seed, 0x003d_cb20)
            },
        );
    }

    /// Started and fresh states; the fresh path sizes, allocates through
    /// the test fake and initialises (already-started, so it returns 0).
    #[test]
    fn ea_file_startup_matches() {
        check(
            ee::s14::f_003dcbd8__generated,
            ea_file_startup,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cbd8, 0);
                // Request area, allocator slot, started flag.
                c.bus.write32(0x0045_0c10, 0x1000);
                c.bus.write32(0x0045_0c14, 0x2000);
                c.bus.write32(0x0049_5d28, 0x3000);
                c.bus.write32(0x0051_9ab0, seed % 2);
                c.g[A0][0] = 0x10;
                c.g[A1][0] = 0x20;
                c.g[A2][0] = 0x30;
                c.pc = 0x003d_cbd8;
            },
        );
    }

    /// Already-started (0) and fresh (1, without the CD init: the CD
    /// path spins on Sony hardware calls, so it stays unreached at unit
    /// level and is covered by the replay gates instead).
    #[test]
    fn ea_file_init_matches() {
        check(
            ee::s14::f_003dcc88__generated,
            ea_file_init,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cc88, 0);
                c.bus.write32(0x0051_9ab0, (seed % 2) as u32);
                c.bus.write32(0x0045_0c60, 0);
                c.bus.write32(0x0045_0c5c, 0x40);
                c.bus.write32(0x0051_9ad8, DATA + 0x800);
                for i in 0..16u32 {
                    c.bus.write32(DATA + 0x800 + 4 * i, 0);
                }
                c.g[A0][0] = 0x10;
                c.g[A1][0] = 0x20;
                c.g[A2][0] = 0x30;
                c.g[A3][0] = (DATA + 0x900) as u64;
                c.pc = 0x003d_cc88;
            },
        );
    }

    /// Default and explicit queue counts; LO/HI compared too.
    #[test]
    fn ea_file_calc_mem_size_matches() {
        check_lohi(
            ee::s14::f_003dcd98__generated,
            ea_file_calc_mem_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cd98, 0);
                c.bus.write32(0x0045_0c5c, 0x40);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { 0x18 };
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { 0x400 };
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { 0x28 };
                c.pc = 0x003d_cd98;
            },
        );
    }

    /// Null requests (-3) and crafted handles hitting the table both ways.
    #[test]
    fn ea_file_get_request_status_matches() {
        check(
            ee::s14::f_003dce90__generated,
            ea_file_get_request_status,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ce90, 0);
                c.bus.write32(0x0051_9ad8, DATA + 0x800);
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x800 + 8 * i, 0);
                    c.bus.write32(DATA + 0x800 + 8 * i + 4, 0);
                }
                let mode = seed % 4;
                if mode == 0 {
                    c.g[A0][0] = 0;
                } else {
                    // Byte 3 selects the table slot; the word there
                    // decides the status bit.
                    let i = seed % 8;
                    c.bus.write32(DATA + 0x800 + 8 * i, 0x100 << (i % 3));
                    c.g[A0][0] = 0x34_0000 + ((i << 24) as u64);
                }
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_ce90;
            },
        );
    }
} // mod tests2
