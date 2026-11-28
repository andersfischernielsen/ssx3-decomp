//! `src/lib/scf.cpp`: hand-written system configuration (`libscf_*`, `sceScf*`) replacements.

use crate::abi::ret;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_b938, libscf_read_rom_ver, "libscf_ReadRomVer"),
    (0x0040_b9a8, libscf_is_t10k, "libscf_IsT10K"),
    (0x0040_b9e8, scescf_get_language, "sceScfGetLanguage"),
    (0x0040_ba68, scescf_get_aspect, "sceScfGetAspect"),
    (0x0040_baa8, scescf_get_spdif, "sceScfGetSpdif"),
    (0x0040_bae8, scescf_get_time_zone, "sceScfGetTimeZone"),
    (0x0040_bb38, scescf_get_date_format, "sceScfGetDateFormat"),
    (0x0040_bba0, scescf_get_summer_time, "sceScfGetSummerTime"),
    (
        0x0040_bc08,
        scescf_get_time_notation,
        "sceScfGetTimeNotation",
    ),
    (0x0040_bc70, libscf_bin_to_bcd, "libscf_BinToBcd"),
    (0x0040_bca0, libscf_bcd_to_bin, "libscf_BcdToBin"),
    (0x0040_bcc0, libscf_clock_bcd_to_bin, "libscf_ClockBcdToBin"),
    (0x0040_bd28, libscf_clock_bin_to_bcd, "libscf_ClockBinToBcd"),
    (0x0040_bd90, libscf_clock_inc_day, "libscf_ClockIncDay"),
    (0x0040_be48, libscf_clock_dec_day, "libscf_ClockDecDay"),
    (0x0040_bef0, libscf_clock_inc_hour, "libscf_ClockIncHour"),
    (0x0040_bf20, libscf_clock_dec_hour, "libscf_ClockDecHour"),
    (
        0x0040_bf48,
        libscf_clock_add_minutes,
        "libscf_ClockAddMinutes",
    ),
    (0x0040_bfd8, scescf_get_gmt_from_rtc, "sceScfGetGMTfromRTC"),
    (
        0x0040_bfe0,
        scescf_get_local_time_from_rtc,
        "sceScfGetLocalTimefromRTC",
    ),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(super) use crate::game::lib::kernel::callee::GET_PARAM;

    /// `libscf_ReadRomVer` (`0x0040_b938`).
    pub(crate) const READ_ROM_VER: Callee = Callee {
        addr: 0x0040_b938,
        f: ee::s15::libscf_ReadRomVer,
    };

    /// `libscf_IsT10K` (`0x0040_b9a8`).
    pub(crate) const IS_T10K: Callee = Callee {
        addr: 0x0040_b9a8,
        f: ee::s15::libscf_IsT10K,
    };

    /// `sceScfGetTimeZone` (`0x0040_bae8`).
    pub(crate) const GET_TIME_ZONE: Callee = Callee {
        addr: 0x0040_bae8,
        f: ee::s15::sceScfGetTimeZone,
    };

    /// `sceScfGetSummerTime` (`0x0040_bba0`).
    pub(crate) const GET_SUMMER_TIME: Callee = Callee {
        addr: 0x0040_bba0,
        f: ee::s15::sceScfGetSummerTime,
    };

    /// `libscf_BinToBcd` (`0x0040_bc70`).
    pub(crate) const BIN_TO_BCD: Callee = Callee {
        addr: 0x0040_bc70,
        f: ee::s15::libscf_BinToBcd,
    };

    /// `libscf_BcdToBin` (`0x0040_bca0`).
    pub(crate) const BCD_TO_BIN: Callee = Callee {
        addr: 0x0040_bca0,
        f: ee::s15::libscf_BcdToBin,
    };

    /// `libscf_ClockBcdToBin` (`0x0040_bcc0`).
    pub(crate) const CLOCK_BCD_TO_BIN: Callee = Callee {
        addr: 0x0040_bcc0,
        f: ee::s15::libscf_ClockBcdToBin,
    };

    /// `libscf_ClockIncHour` (`0x0040_bef0`).
    pub(crate) const CLOCK_INC_HOUR: Callee = Callee {
        addr: 0x0040_bef0,
        f: ee::s15::libscf_ClockIncHour,
    };

    /// `libscf_ClockDecHour` (`0x0040_bf20`).
    pub(crate) const CLOCK_DEC_HOUR: Callee = Callee {
        addr: 0x0040_bf20,
        f: ee::s15::libscf_ClockDecHour,
    };

    /// `GetOsdConfigParam2` (`0x0042_40b0`).
    pub(crate) const GET_OSD_PARAM2: Callee = Callee {
        addr: 0x0042_40b0,
        f: ee::s15::GetOsdConfigParam2,
    };

    /// `sceOpen` (`0x0042_7d60`).
    pub(crate) const SCE_OPEN: Callee = Callee {
        addr: 0x0042_7d60,
        f: ee::s15::sceOpen,
    };

    /// `sceClose` (`0x0042_7fe8`).
    pub(crate) const SCE_CLOSE: Callee = Callee {
        addr: 0x0042_7fe8,
        f: ee::s15::sceClose,
    };

    /// `sceRead` (`0x0042_83a0`).
    pub(crate) const SCE_READ: Callee = Callee {
        addr: 0x0042_83a0,
        f: ee::s15::sceRead,
    };
}

pub(crate) const CLOCK_DAY: u32 = 0x5;

pub(crate) const CLOCK_HOUR: u32 = 0x3;

pub(crate) const CLOCK_MIN: u32 = 0x2;

pub(crate) const CLOCK_MONTH: u32 = 0x6;

/// RTC clock layout for the `libscf_Clock*` helpers: seconds, minutes,
/// hours, day-of-month, month, year. Bytes `+0` and `+4` are carried but
/// never BCD converted.
pub(crate) const CLOCK_SEC: u32 = 0x1;

pub(crate) const CLOCK_YEAR: u32 = 0x7;

/// `u8 libscf_BcdToBin(u8 bcd)` (0x40bca0): decode one BCD byte
/// (`10 * high nibble + low nibble`).
pub fn libscf_bcd_to_bin(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = c.g[A0][0] & 0xff;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 4);
    c.g[A0][0] = 0x6;
    c.mult(0, 0, 3, 4);
    c.g[V1][0] = c.lo[0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V0][0] = c.g[V0][0] & 0xff;
    ret(c);
}

/// `u8 libscf_BinToBcd(u8 bin)` (0x40bc70): encode one byte to BCD
/// (`16 * (x / 10) + x % 10`, computed as `6 * (x / 10) + x`).
pub fn libscf_bin_to_bcd(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0xa;
    c.g[A0][0] = c.g[A0][0] & 0xff;
    c.divu(0, 4, 3);
    c.g[A1][0] = 0x6;
    // `v1` is 10 here, so both arms of the generated `v1 == 0` split
    // rejoin with no extra work.
    c.cycles += 5;
    c.g[V0][0] = c.lo[0];
    c.mult(0, 3, 2, 5);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.g[V0][0] & 0xff;
    ret(c);
}

/// `void libscf_ClockAddMinutes(clock, minutes)` (0x40bf48): add a signed
/// minute delta to an RTC clock. The clock is decoded to binary, the delta
/// folded in, and whole hours carried through `libscf_ClockIncHour` /
/// `libscf_ClockDecHour` (which carry days themselves); the result is
/// re-encoded to BCD (tail call to `libscf_ClockBinToBcd`).
pub fn libscf_clock_add_minutes(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 11;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A1][0];
        callee::CLOCK_BCD_TO_BIN.call(c, 0x0040_bf64)?;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(CLOCK_MIN).lbu(c);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        // 0x40bfbc: the negative and early-store paths rejoin for the
        // shared tail; only the increment-loop exit skips it.
        let mut tail = true;
        if (c.g[S0][0] as i64) < 0 {
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::CLOCK_DEC_HOUR.call(c, 0x0040_bfb0)?;
                if (c.g[S0][0] as i64) < 0 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_bfa8);
                    }
                } else {
                    c.cycles += 1;
                    Addr(c.g[S1][0] as u32).off(CLOCK_MIN).sb(c, c.g[S0][0]);
                    break;
                }
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 61) as u64;
            if c.g[V0][0] != 0 {
                Addr(c.g[S1][0] as u32).off(CLOCK_MIN).sb(c, c.g[S0][0]);
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                loop {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c));
                    callee::CLOCK_INC_HOUR.call(c, 0x0040_bf90)?;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 61) as u64;
                    c.g[A0][0] = c.g[S1][0];
                    if c.g[V0][0] == 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_bf88);
                        }
                    } else {
                        c.cycles += 2;
                        Addr(c.g[S1][0] as u32).off(CLOCK_MIN).sb(c, c.g[S0][0]);
                        break;
                    }
                }
                tail = false;
            }
        }
        if tail {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        // Tail call: `ra` already points at our caller.
        c.pc = 0x0040_bd28;
        ee::s15::libscf_ClockBinToBcd(c);
        Ok(())
    })
}

/// `void libscf_ClockBcdToBin(clock)` (0x40bcc0): decode the clock's BCD
/// bytes (year, month, day, hour, minute, second) to binary in place.
pub fn libscf_clock_bcd_to_bin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_YEAR).lbu(c);
        callee::BCD_TO_BIN.call(c, 0x0040_bcd8)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_MONTH).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_YEAR).sb(c, c.g[V0][0]);
        callee::BCD_TO_BIN.call(c, 0x0040_bce4)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_DAY).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_MONTH).sb(c, c.g[V0][0]);
        callee::BCD_TO_BIN.call(c, 0x0040_bcf0)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_HOUR).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_DAY).sb(c, c.g[V0][0]);
        callee::BCD_TO_BIN.call(c, 0x0040_bcfc)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_MIN).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_HOUR).sb(c, c.g[V0][0]);
        callee::BCD_TO_BIN.call(c, 0x0040_bd08)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_SEC).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_MIN).sb(c, c.g[V0][0]);
        callee::BCD_TO_BIN.call(c, 0x0040_bd14)?;
        Addr(c.g[S0][0] as u32).off(CLOCK_SEC).sb(c, c.g[V0][0]);
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// `void libscf_ClockBinToBcd(clock)` (0x40bd28): encode the clock's
/// binary bytes (year, month, day, hour, minute, second) to BCD in place.
pub fn libscf_clock_bin_to_bcd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_YEAR).lbu(c);
        callee::BIN_TO_BCD.call(c, 0x0040_bd40)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_MONTH).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_YEAR).sb(c, c.g[V0][0]);
        callee::BIN_TO_BCD.call(c, 0x0040_bd4c)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_DAY).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_MONTH).sb(c, c.g[V0][0]);
        callee::BIN_TO_BCD.call(c, 0x0040_bd58)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_HOUR).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_DAY).sb(c, c.g[V0][0]);
        callee::BIN_TO_BCD.call(c, 0x0040_bd64)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_MIN).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_HOUR).sb(c, c.g[V0][0]);
        callee::BIN_TO_BCD.call(c, 0x0040_bd70)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(CLOCK_SEC).lbu(c);
        Addr(c.g[S0][0] as u32).off(CLOCK_MIN).sb(c, c.g[V0][0]);
        callee::BIN_TO_BCD.call(c, 0x0040_bd7c)?;
        Addr(c.g[S0][0] as u32).off(CLOCK_SEC).sb(c, c.g[V0][0]);
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// `void libscf_ClockDecDay(clock)` (0x40be48): back the day-of-month up
/// by one. On underflow the day becomes the previous month's length and
/// the month carries (rolling the year past 0 back to 99), again against
/// the ROM month lengths with the leap-year February patch.
pub fn libscf_clock_dec_day(c: &mut Cpu) {
    c.cycles += 18;
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CLOCK_DAY).lbu(c);
    c.g[V0][0] = 0x4a0000;
    c.g[A0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).lbu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7dd0));
    // Copy the 12 month lengths from ROM onto the stack frame.
    c.ldl(A2, (c.g[T1][0] as u32).wrapping_add(0x7));
    c.ldr(A2, c.g[T1][0] as u32);
    c.lwl(A3, (c.g[T1][0] as u32).wrapping_add(0xb));
    c.lwr(A3, (c.g[T1][0] as u32).wrapping_add(0x8));
    c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(A2, c.g[SP][0] as u32);
    c.swl(A3, (c.g[SP][0] as u32).wrapping_add(0xb));
    c.swr(A3, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.g[A0][0] & 0x3;
    let leap = c.g[A0][0] == 0;
    Addr(c.g[A1][0] as u32).off(CLOCK_DAY).sb(c, c.g[V1][0]);
    if leap {
        c.cycles += 2;
        c.g[V0][0] = 0x1d;
        Addr(c.g[SP][0] as u32).off(0x1).sb(c, c.g[V0][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_DAY).lbu(c);
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V1][0] = c.g[V0][0] & 0xff;
        Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).sb(c, c.g[V0][0]);
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).lbu(c);
            let year_nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if !year_nonzero {
                c.cycles += 1;
                c.g[V0][0] = 0x63;
            }
            c.cycles += 3;
            Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).sb(c, c.g[V0][0]);
            c.g[V0][0] = 0xc;
            Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).sb(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x1)).lbu(c);
        Addr(c.g[A1][0] as u32).off(CLOCK_DAY).sb(c, c.g[V1][0]);
    }
    c.cycles += 2;
    let ret = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ret;
}

/// `void libscf_ClockDecHour(clock)` (0x40bf20): back the hour up one;
/// when it wraps past 0, set 23 and borrow a day (`libscf_ClockDecDay`,
/// tail call).
pub fn libscf_clock_dec_hour(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(CLOCK_HOUR).lbu(c);
        let wrap = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !wrap {
            c.cycles += 2;
            Addr(c.g[V1][0] as u32).off(CLOCK_HOUR).sb(c, c.g[V0][0]);
            ret(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 0x17;
        Addr(c.g[V1][0] as u32).off(CLOCK_HOUR).sb(c, c.g[V0][0]);
        // Tail call: `ra` already points at our caller.
        c.pc = 0x0040_be48;
        ee::s15::libscf_ClockDecDay(c);
        Ok(())
    })
}

/// `void libscf_ClockIncDay(clock)` (0x40bd90): advance the day-of-month
/// by one. On month overflow the day resets to 1 and the month carries
/// (rolling the year past 99 back to 0); whether the month fits is checked
/// against the 12 month lengths read from ROM, with February patched to 29
/// in leap years (`year & 3 == 0`).
pub fn libscf_clock_inc_day(c: &mut Cpu) {
    c.cycles += 18;
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CLOCK_DAY).lbu(c);
    c.g[V0][0] = 0x4a0000;
    c.g[A0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).lbu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7dd0));
    // Copy the 12 month lengths from ROM onto the stack frame.
    c.ldl(A2, (c.g[T1][0] as u32).wrapping_add(0x7));
    c.ldr(A2, c.g[T1][0] as u32);
    c.lwl(A3, (c.g[T1][0] as u32).wrapping_add(0xb));
    c.lwr(A3, (c.g[T1][0] as u32).wrapping_add(0x8));
    c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(A2, c.g[SP][0] as u32);
    c.swl(A3, (c.g[SP][0] as u32).wrapping_add(0xb));
    c.swr(A3, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.g[A0][0] & 0x3;
    let leap = c.g[A0][0] == 0;
    Addr(c.g[A1][0] as u32).off(CLOCK_DAY).sb(c, c.g[V1][0]);
    if leap {
        c.cycles += 2;
        c.g[V0][0] = 0x1d;
        Addr(c.g[SP][0] as u32).off(0x1).sb(c, c.g[V0][0]);
    }
    c.cycles += 7;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).lbu(c);
    c.g[A0][0] = Addr(c.g[A1][0] as u32).off(CLOCK_DAY).lbu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_sub(0x1));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V0][0] = 0x1;
    if c.g[V1][0] != 0 {
        c.cycles += 7;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).lbu(c);
        Addr(c.g[A1][0] as u32).off(CLOCK_DAY).sb(c, c.g[V0][0]);
        c.g[A0][0] = 0xd;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = c.g[V1][0] & 0xff;
        let year_carry = c.g[V0][0] == c.g[A0][0];
        Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).sb(c, c.g[V1][0]);
        if year_carry {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).lbu(c);
            c.g[V0][0] = 0x63;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.cycles += 1;
                Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).sb(c, c.g[V0][0]);
            } else {
                c.cycles += 2;
                Addr(c.g[A1][0] as u32).off(CLOCK_YEAR).sb(c, 0u64);
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            Addr(c.g[A1][0] as u32).off(CLOCK_MONTH).sb(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    let ret = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ret;
}

/// `void libscf_ClockIncHour(clock)` (0x40bef0): advance the hour by one;
/// when it reaches 24, reset it and carry a day (`libscf_ClockIncDay`,
/// tail call).
pub fn libscf_clock_inc_hour(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = 0x18;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CLOCK_HOUR).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[A0][0] = c.g[V1][0] & 0xff;
        let wrap = c.g[A0][0] == c.g[V0][0];
        Addr(c.g[A1][0] as u32).off(CLOCK_HOUR).sb(c, c.g[V1][0]);
        if !wrap {
            c.cycles += 2;
            ret(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[A1][0];
        Addr(c.g[A1][0] as u32).off(CLOCK_HOUR).sb(c, 0u64);
        // Tail call: `ra` already points at our caller.
        c.pc = 0x0040_bd90;
        ee::s15::libscf_ClockIncDay(c);
        Ok(())
    })
}

/// `int libscf_IsT10K(void)` (0x40b9a8): whether this is a TOOL/T10K
/// machine: 1 when the cached ROMVER kind byte is 'T'. Warms the cache
/// with `libscf_ReadRomVer` first when it is empty.
pub fn libscf_is_t10k(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x450000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[V0][0] = c.lb(0x0045_3400);
        c.g[S0][0] = sx(0x0045_3400);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0x10);
        if empty {
            c.cycles += 2;
            callee::READ_ROM_VER.call(c, 0x0040_b9cc)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[V0][0] = c.g[V0][0] ^ 0x54;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// `char *libscf_ReadRomVer(void)` (0x40b938): the cached `rom0:ROMVER`
/// version string, reading it from disc the first time.
pub fn libscf_read_rom_ver(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S1][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[V0][0] = c.lb(0x0045_3400);
        c.g[S2][0] = sx(0x0045_3400);
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        let empty = c.g[V0][0] == 0;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if empty {
            c.cycles += 7;
            c.g[A0][0] = sx(0x004a_0000u32.wrapping_sub(0x7de0));
            c.g[A1][0] = 0x1;
            callee::SCE_OPEN.call(c, 0x0040_b96c)?;
            c.g[S0][0] = c.g[V0][0];
            let failed = (c.g[S0][0] as i64) < 0;
            c.g[A1][0] = c.g[S2][0];
            if !failed {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0xe;
                callee::SCE_READ.call(c, 0x0040_b984)?;
                c.g[A0][0] = c.g[S0][0];
                callee::SCE_CLOSE.call(c, 0x0040_b98c)?;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3400));
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

/// `int sceScfGetAspect(void)` (0x40ba68): the display aspect flag: the
/// cached T10K byte, else bits 1-2 of the OSD config word.
pub fn scescf_get_aspect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        callee::IS_T10K.call(c, 0x0040_ba78)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V1][0] = 0x450000;
        if use_osd {
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_ba90)?;
            c.g[V0][0] = c.lw(sp(c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x3;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33fa).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int sceScfGetDateFormat(void)` (0x40bb38): the date display format:
/// the cached T10K byte, else 0 when the OSD config has no region, else
/// bits 6-7 of the OSD extra byte.
pub fn scescf_get_date_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        callee::IS_T10K.call(c, 0x0040_bb48)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        if use_osd {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_bb60)?;
            c.g[V0][0] = c.lw(sp(c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
            c.g[V0][0] = c.g[V0][0] & 0x7;
            c.g[A0][0] = c.g[SP][0] | 0x4;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                callee::GET_OSD_PARAM2.call(c, 0x0040_bb88)?;
                c.g[V1][0] = Addr(sp(c)).off(0x4).lbu(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 6);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33fb).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void sceScfGetGMTfromRTC(clock)` (0x40bfd8): add -540 minutes to
/// `clock` (tail call to `libscf_ClockAddMinutes`).
pub fn scescf_get_gmt_from_rtc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        c.g[A1][0] = 0xffff_ffff_ffff_fde4;
        // Tail call: `ra` already points at our caller.
        c.pc = 0x0040_bf48;
        ee::s15::libscf_ClockAddMinutes(c);
        Ok(())
    })
}

/// `int sceScfGetLanguage(void)` (0x40b9e8): the UI language: the cached
/// T10K byte, else the OSD language field (bits 16-20) when the OSD
/// version field is set, else the legacy language bit (bit 4).
pub fn scescf_get_language(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        callee::GET_PARAM.call(c, 0x0040_b9f8)?;
        callee::IS_T10K.call(c, 0x0040_ba00)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        if use_osd {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_ba18)?;
            c.g[V1][0] = c.lw(sp(c));
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 13);
            c.g[V0][0] = c.g[V0][0] & 0x7;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 16);
                c.cycles += 1;
                c.g[V0][0] = c.g[V0][0] & 0x1f;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 4);
                c.g[V0][0] = c.g[V0][0] & 0x1;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33fc).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void sceScfGetLocalTimefromRTC(clock)` (0x40bfe0): read the timezone
/// and daylight-saving offsets, combine them (`tz + 60*dst - 540` minutes)
/// and add the delta to `clock` (tail call to `libscf_ClockAddMinutes`).
pub fn scescf_get_local_time_from_rtc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        callee::GET_TIME_ZONE.call(c, 0x0040_bff8)?;
        c.g[S1][0] = c.g[V0][0];
        callee::GET_SUMMER_TIME.call(c, 0x0040_c000)?;
        c.g[V1][0] = 0x3c;
        c.g[A0][0] = c.g[S0][0];
        c.mult(0, 2, 2, 3);
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x21c));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        // Tail call: `ra` already points at our caller.
        c.pc = 0x0040_bf48;
        ee::s15::libscf_ClockAddMinutes(c);
        Ok(())
    })
}

/// `int sceScfGetSpdif(void)` (0x40baa8): whether S/PDIF digital audio is
/// enabled: the cached T10K byte, else bit 0 of the OSD config word.
pub fn scescf_get_spdif(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        callee::IS_T10K.call(c, 0x0040_bab8)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V1][0] = 0x450000;
        if use_osd {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_bad0)?;
            c.g[V0][0] = c.lw(sp(c));
            c.g[V0][0] = c.g[V0][0] & 0x1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33fd).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int sceScfGetSummerTime(void)` (0x40bba0): whether daylight saving is
/// in effect: the cached T10K byte, else 0 when the OSD config has no
/// region, else bit 4 of the OSD extra byte.
pub fn scescf_get_summer_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        callee::IS_T10K.call(c, 0x0040_bbb0)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        if use_osd {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_bbc8)?;
            c.g[V0][0] = c.lw(sp(c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
            c.g[V0][0] = c.g[V0][0] & 0x7;
            c.g[A0][0] = c.g[SP][0] | 0x4;
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                callee::GET_OSD_PARAM2.call(c, 0x0040_bbf0)?;
                c.g[V1][0] = Addr(sp(c)).off(0x4).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 4);
                c.g[V0][0] = c.g[V1][0] & 0x1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33fe).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int sceScfGetTimeNotation(void)` (0x40bc08): 12/24-hour notation: the
/// cached T10K byte, else 0 when the OSD config has no region, else bit 5
/// of the OSD extra byte.
pub fn scescf_get_time_notation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        callee::IS_T10K.call(c, 0x0040_bc18)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        if use_osd {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_bc30)?;
            c.g[V0][0] = c.lw(sp(c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
            c.g[V0][0] = c.g[V0][0] & 0x7;
            c.g[A0][0] = c.g[SP][0] | 0x4;
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                callee::GET_OSD_PARAM2.call(c, 0x0040_bc58)?;
                c.g[V1][0] = Addr(sp(c)).off(0x4).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 5);
                c.g[V0][0] = c.g[V1][0] & 0x1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33ff).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int sceScfGetTimeZone(void)` (0x40bae8): the timezone offset: the
/// cached T10K halfword, else the OSD config word's high bits (or 540
/// minutes when its region field is 0).
pub fn scescf_get_time_zone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        callee::IS_T10K.call(c, 0x0040_baf8)?;
        let use_osd = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        if use_osd {
            c.cycles += 8;
            c.g[A0][0] = c.g[SP][0];
            callee::GET_PARAM.call(c, 0x0040_bb10)?;
            c.g[V0][0] = c.lw(sp(c));
            c.g[A0][0] = 0x21c;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 13);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 21) as u32);
            c.g[V1][0] = c.g[V1][0] & 0x7;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(0x0045_33f8).lh(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::require_paths;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use ssx3_ee_rt::Cpu;

    /// `u8 libscf_BcdToBin(u8 bcd)`: valid BCD plus out-of-range bytes.
    #[test]
    fn libscf_bcd_to_bin_matches() {
        let scf_bcd_to_bin_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0001, 0);
            c.g[A0][0] = match seed % 8 {
                0 => 0,
                1 => 0x59,
                2 => 0x99,
                3 => 0x23,
                4 => 0xff,
                5 => 0xa0,
                6 => 0x0a,
                _ => r.next_u32() as u64,
            };
            c.pc = 0x0040_bca0;
        };
        check(
            ee::s15::f_0040bca0__generated,
            libscf_bcd_to_bin,
            SEEDS,
            scf_bcd_to_bin_call,
        );
        require_paths(
            ee::s15::f_0040bca0__generated,
            SEEDS,
            &[8],
            scf_bcd_to_bin_call,
        );
    }

    /// `u8 libscf_BinToBcd(u8 bin)`: every input byte value.
    #[test]
    fn libscf_bin_to_bcd_matches() {
        let scf_bin_to_bcd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0000, 0);
            c.g[A0][0] = match seed % 8 {
                0 => 0,
                1 => 9,
                2 => 10,
                3 => 59,
                4 => 99,
                5 => 100,
                6 => 0xff,
                _ => r.next_u32() as u64,
            };
            c.pc = 0x0040_bc70;
        };
        check(
            ee::s15::f_0040bc70__generated,
            libscf_bin_to_bcd,
            SEEDS,
            scf_bin_to_bcd_call,
        );
        require_paths(
            ee::s15::f_0040bc70__generated,
            SEEDS,
            &[11],
            scf_bin_to_bcd_call,
        );
    }

    /// `void libscf_ClockAddMinutes(clock, minutes)`: the same carry loops
    /// with real events on a running thread (structured back edges: the
    /// unwind request is deferred by the machine, both sides loop on).
    #[test]
    fn libscf_clock_add_minutes_events() {
        check_events(
            ee::s15::f_0040bf48__generated,
            libscf_clock_add_minutes,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1242_0021, 0);
                month_table(c);
                let clock = DATA + 0x100;
                c.g[A0][0] = clock as u64;
                // Five to ten carry iterations, so the back-edge event
                // check fires on several passes.
                let delta: i32 = (300 + (seed % 300) as i32) * if seed % 2 == 0 { 1 } else { -1 };
                c.g[A1][0] = (delta as u32) as u64;
                c.bus.write8(clock, 0);
                c.bus.write8(clock + CLOCK_SEC, 0);
                c.bus.write8(clock + CLOCK_MIN, 30);
                c.bus.write8(clock + CLOCK_HOUR, 12);
                c.bus.write8(clock + 0x4, 0);
                c.bus.write8(clock + CLOCK_DAY, 15);
                c.bus.write8(clock + CLOCK_MONTH, 6);
                c.bus.write8(clock + CLOCK_YEAR, 7);
                c.pc = 0x0040_bf48;
                schedule(c, &mut r, seed, 0x0040_bf48)
            },
        );
    }

    /// `void libscf_ClockAddMinutes(clock, minutes)`: small positive deltas
    /// (direct store), large positive ones (hour-carry loop), negatives
    /// (day-borrow loop) and zero. Seeds from 200 on start at minute 0 with
    /// the boundary deltas 61 (stored as is), 121 (61 after one carry) and
    /// -60 (0 after one borrow), at varied hours.
    #[test]
    fn libscf_clock_add_minutes_matches() {
        // Signed minute deltas; the register holds the low 32 bits.
        const DELTAS: [i32; 8] = [0, 1, 5, 61, 150, 1439, -1, -540];
        let scf_clock_add_minutes_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0008, 0);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            let (delta, minute) = match seed.checked_sub(SEEDS) {
                None => (DELTAS[seed as usize % DELTAS.len()], seed % 60),
                Some(k) => ([61, 121, -60][(k % 3) as usize], 0),
            };
            c.g[A1][0] = (delta as u32) as u64;
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, minute as u8);
            c.bus.write8(clock + CLOCK_HOUR, ((seed / 7) % 24) as u8);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, 15);
            c.bus.write8(clock + CLOCK_MONTH, 6);
            c.bus.write8(clock + CLOCK_YEAR, 7);
            c.pc = 0x0040_bf48;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040bf48__generated,
            libscf_clock_add_minutes,
            SEEDS + 24,
            scf_clock_add_minutes_call,
        );
        require_paths(
            ee::s15::f_0040bf48__generated,
            SEEDS + 24,
            &[186, 194, 203, 217, 231, 274, 298, 539, 553],
            scf_clock_add_minutes_call,
        );
    }

    /// `void libscf_ClockBcdToBin(clock)`: six BCD bytes decoded in place.
    #[test]
    fn libscf_clock_bcd_to_bin_matches() {
        let scf_clock_bcd_to_bin_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0002, 0);
            seed_clock(c, &mut r, true);
            c.pc = 0x0040_bcc0;
        };
        check(
            ee::s15::f_0040bcc0__generated,
            libscf_clock_bcd_to_bin,
            SEEDS,
            scf_clock_bcd_to_bin_call,
        );
        require_paths(
            ee::s15::f_0040bcc0__generated,
            SEEDS,
            &[74],
            scf_clock_bcd_to_bin_call,
        );
    }

    /// `void libscf_ClockBinToBcd(clock)`: six binary bytes encoded in place.
    #[test]
    fn libscf_clock_bin_to_bcd_matches() {
        let scf_clock_bin_to_bcd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0003, 0);
            seed_clock(c, &mut r, false);
            c.pc = 0x0040_bd28;
        };
        check(
            ee::s15::f_0040bd28__generated,
            libscf_clock_bin_to_bcd,
            SEEDS,
            scf_clock_bin_to_bcd_call,
        );
        require_paths(
            ee::s15::f_0040bd28__generated,
            SEEDS,
            &[92],
            scf_clock_bin_to_bcd_call,
        );
    }

    /// `void libscf_ClockDecDay(clock)`: day borrow, including falling back
    /// into December and rolling the year past 0 back to 99.
    #[test]
    fn libscf_clock_dec_day_matches() {
        let scf_clock_dec_day_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0005, 0);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            // Year 0 (rolls to 99) and leap years (Feb has 29).
            let year = [0, 1, 2, 4, 5, 96, 99, 7][seed as usize % 8];
            // January often (year borrow), March sometimes (Feb length).
            let month: u32 = if seed % 4 == 0 {
                1
            } else if seed % 7 == 6 {
                3
            } else {
                1 + seed % 12
            };
            // The 1st (borrow) and later days (plain decrement).
            let day = [1, 1, 2, 15, 28, 30][seed as usize % 6];
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, 0);
            c.bus.write8(clock + CLOCK_HOUR, 12);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, day as u8);
            c.bus.write8(clock + CLOCK_MONTH, month as u8);
            c.bus.write8(clock + CLOCK_YEAR, year as u8);
            c.pc = 0x0040_be48;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040be48__generated,
            libscf_clock_dec_day,
            SEEDS,
            scf_clock_dec_day_call,
        );
        require_paths(
            ee::s15::f_0040be48__generated,
            SEEDS,
            &[23, 25, 32, 34, 38, 41],
            scf_clock_dec_day_call,
        );
    }

    /// `void libscf_ClockDecHour(clock)`: plain decrement plus the 0->23
    /// wrap that borrows a day (landing on month/year edges).
    #[test]
    fn libscf_clock_dec_hour_matches() {
        let scf_clock_dec_hour_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0007, 0);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            let hour = seed % 25;
            // On the wrap hour, sit on day/month/year edges.
            let (day, month, year) = if hour == 0 {
                match seed % 4 {
                    0 => (1, 3, 4),
                    1 => (1, 1, 1),
                    2 => (1, 1, 0),
                    _ => (15, 6, 7),
                }
            } else {
                (15, 6, 7)
            };
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, 0);
            c.bus.write8(clock + CLOCK_HOUR, hour as u8);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, day as u8);
            c.bus.write8(clock + CLOCK_MONTH, month as u8);
            c.bus.write8(clock + CLOCK_YEAR, year as u8);
            c.pc = 0x0040_bf20;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040bf20__generated,
            libscf_clock_dec_hour,
            SEEDS,
            scf_clock_dec_hour_call,
        );
        require_paths(
            ee::s15::f_0040bf20__generated,
            SEEDS,
            &[6, 30, 41, 45, 48],
            scf_clock_dec_hour_call,
        );
    }

    /// `void libscf_ClockIncDay(clock)`: day carry, including month/year
    /// rollover and the leap-year February patch.
    #[test]
    fn libscf_clock_inc_day_matches() {
        let scf_clock_inc_day_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0004, 0);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            // Leap years (Feb 29 exists) and common years.
            let year = [0, 1, 2, 3, 4, 63, 96, 98, 99][seed as usize % 9];
            // February often (leap patch), December sometimes (rollover).
            let month: u32 = if seed % 4 == 0 {
                2
            } else if seed % 9 == 8 {
                12
            } else {
                1 + seed % 12
            };
            // Below, at, and above each month's length.
            let day = [1, 27, 28, 29, 30, 31][seed as usize % 6];
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, 0);
            c.bus.write8(clock + CLOCK_HOUR, 12);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, day as u8);
            c.bus.write8(clock + CLOCK_MONTH, month as u8);
            c.bus.write8(clock + CLOCK_YEAR, year as u8);
            c.pc = 0x0040_bd90;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040bd90__generated,
            libscf_clock_inc_day,
            SEEDS,
            scf_clock_inc_day_call,
        );
        require_paths(
            ee::s15::f_0040bd90__generated,
            SEEDS,
            &[27, 29, 34, 36, 41, 42],
            scf_clock_inc_day_call,
        );
    }

    /// `void libscf_ClockIncHour(clock)`: plain increment plus the 23->0
    /// wrap that carries a day (landing on month/year edges).
    #[test]
    fn libscf_clock_inc_hour_matches() {
        let scf_clock_inc_hour_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0006, 0);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            let hour = seed % 25;
            // On the wrap hour, sit on day/month/year edges.
            let (day, month, year) = if hour == 24 {
                match seed % 4 {
                    0 => (28, 2, 4),
                    1 => (31, 12, 99),
                    2 => (30, 4, 7),
                    _ => (31, 1, 0),
                }
            } else {
                (15, 6, 7)
            };
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, 0);
            c.bus.write8(clock + CLOCK_HOUR, hour as u8);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, day as u8);
            c.bus.write8(clock + CLOCK_MONTH, month as u8);
            c.bus.write8(clock + CLOCK_YEAR, year as u8);
            c.pc = 0x0040_bef0;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040bef0__generated,
            libscf_clock_inc_hour,
            SEEDS,
            scf_clock_inc_hour_call,
        );
        require_paths(
            ee::s15::f_0040bef0__generated,
            SEEDS,
            &[9, 37],
            scf_clock_inc_hour_call,
        );
    }

    /// `int libscf_IsT10K(void)`: cached flag set, or empty (the call to
    /// `libscf_ReadRomVer`, whose `sceOpen` fails); the kind byte is 'T',
    /// 'U' or neither.
    #[test]
    fn libscf_is_t10k_matches() {
        let scf_is_t10k_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_000b, 0);
            // Warm cache with varied flag bytes, or empty.
            c.bus
                .write8(ROMVER_CACHED, [1, 2, 0x20, 0xff, 0][seed as usize % 5]);
            open_fails(c, &mut r);
            // 'T', one past it, or any byte with bits 2 and 4 clear.
            c.bus.write8(
                ROMVER_KIND,
                match seed / 4 % 3 {
                    0 => 0x54,
                    1 => 0x55,
                    _ => (seed % 256) as u8 & 0xab,
                },
            );
            c.pc = 0x0040_b9a8;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040b9a8__generated,
            libscf_is_t10k,
            SEEDS,
            scf_is_t10k_call,
        );
        require_paths(
            ee::s15::f_0040b9a8__generated,
            SEEDS,
            &[14, 418],
            scf_is_t10k_call,
        );
    }

    /// `char *libscf_ReadRomVer(void)`: the cached-pointer path, and the
    /// cold path with `sceOpen` failing before its RPC (every file
    /// descriptor in use), so no read or close follows.
    #[test]
    fn libscf_read_rom_ver_matches() {
        let scf_read_rom_ver_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_000c, 0);
            // Warm cache, so the body returns the pointer with no calls.
            c.bus
                .write8(ROMVER_CACHED, [1, 2, 0x20, 0xff, 0][seed as usize % 5]);
            open_fails(c, &mut r);
            c.pc = 0x0040_b938;
        };
        check(
            ee::s15::f_0040b938__generated,
            libscf_read_rom_ver,
            SEEDS,
            scf_read_rom_ver_call,
        );
        require_paths(
            ee::s15::f_0040b938__generated,
            SEEDS,
            &[16, 402],
            scf_read_rom_ver_call,
        );
    }

    /// `char *libscf_ReadRomVer(void)` from a cold cache with the file
    /// server answering: `sceOpen` hands out descriptor 0 (the first free
    /// one), then the version is read and the file closed.
    #[test]
    fn libscf_read_rom_ver_opened_matches() {
        let scf_read_rom_ver_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x0040_b938 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            crate::difftest::elf_image(c);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            crate::game::lib::cdvd::tests::rpc_answered(c, FIO_CLIENT, 0);
            ssx3_ps2::kernel::start_main_thread(c, 0x0040_b938, 0, 0);
            c.g[RA][0] = crate::difftest::RA as u64;
            c.bus.write8(ROMVER_CACHED, 0);
            // File library set up and bound to the current server.
            c.bus.write32(0x0045_520c, 1);
            let server = c.bus.read32(0x0045_5164);
            c.bus.write32(FIO_CLIENT + 0x28, server);
            // The open reply word, and the open result (0) in sceOpen's frame.
            c.bus.write32(0x0052_e340, 1 + r.next_u32() % 7);
            c.bus.write32(crate::difftest::SP - 0x160, 0);
        };
        check(
            ee::s15::f_0040b938__generated,
            libscf_read_rom_ver,
            8,
            scf_read_rom_ver_call,
        );
        require_paths(
            ee::s15::f_0040b938__generated,
            8,
            &[1827],
            scf_read_rom_ver_call,
        );
    }

    /// Make `sceOpen` fail before its RPC: the file library set up (no
    /// rebind) and all 32 file descriptors in use, so it returns -19.
    fn open_fails(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(0x0045_520c, 1);
        for fd in 0..32 {
            c.bus.write32(0x0052_ec04 + fd * 0x10, 1 + r.next_u32() % 7);
        }
    }

    /// The file-I/O SIF RPC client.
    const FIO_CLIENT: u32 = 0x0052_ee00;

    /// `int sceScfGetAspect(void)`: T10K byte or OSD bits 1-2.
    #[test]
    fn scescf_get_aspect_matches() {
        let scescf_get_aspect_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0011, 1);
            scf_fixture(c, &mut r, seed, seed.wrapping_mul(0x1000_3));
            c.pc = 0x0040_ba68;
        };
        check(
            ee::s15::f_0040ba68__generated,
            scescf_get_aspect,
            SEEDS,
            scescf_get_aspect_call,
        );
        require_paths(
            ee::s15::f_0040ba68__generated,
            SEEDS,
            &[25, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42],
            scescf_get_aspect_call,
        );
    }

    /// `int sceScfGetDateFormat(void)`: T10K byte, 0 for region 0, else OSD
    /// extra-byte bits 6-7.
    #[test]
    fn scescf_get_date_format_matches() {
        let scescf_get_date_format_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0014, 2);
            let w0 = [0u32, 0x2000, 0xe000, 0xabcd_ef00][seed as usize % 4];
            scf_fixture(c, &mut r, seed, w0);
            c.pc = 0x0040_bb38;
        };
        check(
            ee::s15::f_0040bb38__generated,
            scescf_get_date_format,
            SEEDS,
            scescf_get_date_format_call,
        );
        require_paths(
            ee::s15::f_0040bb38__generated,
            SEEDS,
            &[
                25, 34, 35, 36, 37, 38, 39, 40, 41, 42, 45, 46, 48, 51, 52, 55, 56, 59, 60, 63,
            ],
            scescf_get_date_format_call,
        );
    }

    /// `void sceScfGetGMTfromRTC(clock)`: fixed -540 delta through the day
    /// borrow loop (hour wraps exercise `libscf_ClockDecDay`).
    #[test]
    fn scescf_get_gmt_from_rtc_matches() {
        let scescf_get_gmt_from_rtc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0009, 0);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, (seed % 60) as u8);
            c.bus.write8(clock + CLOCK_HOUR, (seed % 24) as u8);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, (1 + seed % 28) as u8);
            c.bus.write8(clock + CLOCK_MONTH, (1 + seed % 12) as u8);
            c.bus.write8(clock + CLOCK_YEAR, (seed % 100) as u8);
            c.pc = 0x0040_bfd8;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040bfd8__generated,
            scescf_get_gmt_from_rtc,
            SEEDS,
            scescf_get_gmt_from_rtc_call,
        );
        require_paths(
            ee::s15::f_0040bfd8__generated,
            SEEDS,
            &[276, 300, 302, 309, 311, 317, 318],
            scescf_get_gmt_from_rtc_call,
        );
    }

    /// `int sceScfGetLanguage(void)`: T10K byte, OSD language field, or the
    /// legacy language bit.
    #[test]
    fn scescf_get_language_matches() {
        let scescf_get_language_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0010, 2);
            // Legacy bit clear/set, OSD language field clear/set.
            let w0 = [0u32, 0x2000, 0x10, 0x1f_fff0][seed as usize % 4];
            scf_fixture(c, &mut r, seed, w0);
            c.pc = 0x0040_b9e8;
        };
        check(
            ee::s15::f_0040b9e8__generated,
            scescf_get_language,
            SEEDS,
            scescf_get_language_call,
        );
        require_paths(
            ee::s15::f_0040b9e8__generated,
            SEEDS,
            &[
                15, 16, 17, 18, 19, 20, 21, 22, 34, 35, 36, 37, 38, 39, 40, 41, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 63,
            ],
            scescf_get_language_call,
        );
    }

    /// `void sceScfGetLocalTimefromRTC(clock)`: timezone + DST combined into
    /// a minute delta. Mostly the T10K path (small offsets); some seeds take
    /// the OSD path with small config words.
    #[test]
    fn scescf_get_local_time_from_rtc_matches() {
        let scescf_get_local_time_from_rtc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_000a, 2);
            month_table(c);
            let clock = DATA + 0x100;
            c.g[A0][0] = clock as u64;
            c.bus.write8(clock, 0);
            c.bus.write8(clock + CLOCK_SEC, 0);
            c.bus.write8(clock + CLOCK_MIN, (seed % 60) as u8);
            c.bus.write8(clock + CLOCK_HOUR, ((seed / 3) % 24) as u8);
            c.bus.write8(clock + 0x4, 0);
            c.bus.write8(clock + CLOCK_DAY, 15);
            c.bus.write8(clock + CLOCK_MONTH, 6);
            c.bus.write8(clock + CLOCK_YEAR, 7);
            // Warm cache, so `libscf_IsT10K` never reads the disc.
            c.bus.write8(ROMVER_CACHED, 1);
            let sp = crate::difftest::SP;
            if seed % 4 == 3 {
                // OSD path: region field set or clear, extra byte small.
                c.bus.write8(ROMVER_KIND, 0);
                let w0 = [0u32, 0x2000, 0xffe0_0000][seed as usize % 3];
                c.bus.write32(sp, w0);
                c.bus.write32(sp + 4, (seed % 256) as u32);
            } else {
                // T10K path: small timezone (-20..+20), DST on or off.
                c.bus.write8(ROMVER_KIND, 0x54);
                c.bus.write16(T10K_SCF, (seed % 41).wrapping_sub(20) as u16);
                c.bus.write8(T10K_SCF + 6, (seed % 2) as u8);
            }
            c.pc = 0x0040_bfe0;
            let _ = r.next_u32();
        };
        check(
            ee::s15::f_0040bfe0__generated,
            scescf_get_local_time_from_rtc,
            SEEDS,
            scescf_get_local_time_from_rtc_call,
        );
        require_paths(
            ee::s15::f_0040bfe0__generated,
            SEEDS,
            &[
                54, 56, 58, 60, 90, 93, 95, 97, 102, 286, 287, 289, 290, 292, 305, 309, 323, 329,
                332, 342, 346, 352, 356, 366, 367, 369, 376, 381, 388, 392, 407, 413, 417, 428,
                429, 443, 446, 461, 487, 488, 497, 507, 521, 523, 529, 539, 547, 557, 579,
            ],
            scescf_get_local_time_from_rtc_call,
        );
    }

    /// `int sceScfGetSpdif(void)`: T10K byte or OSD bit 0.
    #[test]
    fn scescf_get_spdif_matches() {
        let scescf_get_spdif_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0012, 1);
            scf_fixture(c, &mut r, seed, seed.wrapping_mul(0x2000_5));
            c.pc = 0x0040_baa8;
        };
        check(
            ee::s15::f_0040baa8__generated,
            scescf_get_spdif,
            SEEDS,
            scescf_get_spdif_call,
        );
        require_paths(
            ee::s15::f_0040baa8__generated,
            SEEDS,
            &[25, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41],
            scescf_get_spdif_call,
        );
    }

    /// `int sceScfGetSummerTime(void)`: T10K byte, 0 for region 0, else OSD
    /// extra-byte bit 4.
    #[test]
    fn scescf_get_summer_time_matches() {
        let scescf_get_summer_time_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0015, 2);
            let w0 = [0u32, 0x2000, 0xe000, 0x55aa_55aa][seed as usize % 4];
            scf_fixture(c, &mut r, seed, w0);
            c.pc = 0x0040_bba0;
        };
        check(
            ee::s15::f_0040bba0__generated,
            scescf_get_summer_time,
            SEEDS,
            scescf_get_summer_time_call,
        );
        require_paths(
            ee::s15::f_0040bba0__generated,
            SEEDS,
            &[
                25, 34, 35, 37, 38, 39, 40, 41, 42, 43, 48, 51, 52, 55, 56, 59, 60, 63,
            ],
            scescf_get_summer_time_call,
        );
    }

    /// `int sceScfGetTimeNotation(void)`: T10K byte, 0 for region 0, else
    /// OSD extra-byte bit 5.
    #[test]
    fn scescf_get_time_notation_matches() {
        let scescf_get_time_notation_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0016, 2);
            let w0 = [0u32, 0x2000, 0xe000, 0xdead_beef][seed as usize % 4];
            scf_fixture(c, &mut r, seed, w0);
            c.pc = 0x0040_bc08;
        };
        check(
            ee::s15::f_0040bc08__generated,
            scescf_get_time_notation,
            SEEDS,
            scescf_get_time_notation_call,
        );
        require_paths(
            ee::s15::f_0040bc08__generated,
            SEEDS,
            &[
                25, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 48, 51, 52, 55, 56, 59, 60, 63,
            ],
            scescf_get_time_notation_call,
        );
    }

    /// `int sceScfGetTimeZone(void)`: T10K halfword, 540 for region 0, else
    /// the OSD word's high bits.
    #[test]
    fn scescf_get_time_zone_matches() {
        let scescf_get_time_zone_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1242_0013, 1);
            // Region field 0 (-> 540), nonzero, and arbitrary words.
            let w0 = [0u32, 0x2000, 0xffe0_0000, 0x1234_5678][seed as usize % 4];
            scf_fixture(c, &mut r, seed, w0);
            c.pc = 0x0040_bae8;
        };
        check(
            ee::s15::f_0040bae8__generated,
            scescf_get_time_zone,
            SEEDS,
            scescf_get_time_zone_call,
        );
        require_paths(
            ee::s15::f_0040bae8__generated,
            SEEDS,
            &[25, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
            scescf_get_time_zone_call,
        );
    }

    /// `libscf_ReadRomVer`'s cache flag: nonzero means the version string is
    /// already cached here, so no `sceOpen` happens.
    pub(crate) const ROMVER_CACHED: u32 = 0x0045_3400;

    /// Second ROMVER-cache byte: `0x54` ('T') marks a TOOL/T10K machine.
    pub(crate) const ROMVER_KIND: u32 = 0x0045_3404;

    pub(crate) const SEEDS: u32 = 200;

    /// First T10K fallback byte the `sceScfGet*` getters read.
    pub(crate) const T10K_SCF: u32 = 0x0045_33f8;

    /// Standard month lengths (31 28 31 30 31 30 31 31 30 31 30 31) as the
    /// day carries expect them: the leap-year February patch (`29` at the
    /// frame's second byte) is applied by the functions themselves.
    pub(crate) fn month_table(c: &mut Cpu) {
        crate::difftest::put_words(c, MONTH_TABLE, &[0x1e1f_1c1f, 0x1f1f_1e1f, 0x1f1e_1f1e]);
    }

    /// Shared fixture for the `sceScfGet*` getters: warm cache (so
    /// `libscf_IsT10K` never reads the disc), T10K or OSD path by `seed`,
    /// and the OSD words the decode reads. Returns nothing; both paths of
    /// the T10K check and the decode are covered across seeds.
    pub(crate) fn scf_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, w0: u32) {
        c.bus.write8(ROMVER_CACHED, 1);
        let sp = crate::difftest::SP;
        if seed % 2 == 0 {
            // T10K path: kind byte 'T', all eight fallback bytes varied.
            c.bus.write8(ROMVER_KIND, 0x54);
            let mut x = seed.wrapping_mul(0x9e37_79b9).wrapping_add(0x85eb_ca6b);
            for i in 0..8 {
                c.bus.write8(T10K_SCF + i, (x & 0xff) as u8);
                x = x.wrapping_mul(0x27d4_eb2f).wrapping_add(0x1656_63b5);
            }
        } else {
            // OSD path: kind byte not 'T', config words from the seed.
            c.bus.write8(ROMVER_KIND, (seed % 256) as u8 & 0xab);
            c.bus.write32(sp, w0);
            c.bus.write32(sp + 4, seed.wrapping_mul(0x9e37_79b9));
        }
        let _ = r.next_u32();
    }

    /// Point `a0` at a scratch clock and fill its converted bytes from `r`.
    /// `valid_bcd` writes nibble-valid BCD (for the decode direction),
    /// otherwise raw binary bytes.
    pub(crate) fn seed_clock(c: &mut Cpu, r: &mut Rng, valid_bcd: bool) -> u32 {
        let clock = DATA + 0x100;
        c.g[A0][0] = clock as u64;
        let byte = |v: u32| {
            if valid_bcd {
                (((v / 10) % 10) << 4 | v % 10) as u8
            } else {
                (v & 0x7f) as u8
            }
        };
        c.bus.write8(clock, 0);
        c.bus.write8(clock + CLOCK_SEC, byte(r.next_u32() % 60));
        c.bus.write8(clock + CLOCK_MIN, byte(r.next_u32() % 60));
        c.bus.write8(clock + CLOCK_HOUR, byte(r.next_u32() % 24));
        c.bus.write8(clock + 0x4, 0);
        c.bus.write8(clock + CLOCK_DAY, byte(1 + r.next_u32() % 28));
        c.bus
            .write8(clock + CLOCK_MONTH, byte(1 + r.next_u32() % 12));
        c.bus.write8(clock + CLOCK_YEAR, byte(r.next_u32() % 100));
        clock
    }

    /// Month-length table the day carries read (12 bytes at a fixed ROM
    /// address, copied onto the stack frame).
    pub(crate) const MONTH_TABLE: u32 = 0x0049_8230;
}
