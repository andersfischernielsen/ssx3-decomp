//! Staging module oc1553 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1553`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_7038, console_clock_format_date, "ConsoleClock_FormatDate"),
    (0x002c_7100, console_clock_get_time, "ConsoleClock_GetTime"),
    (0x002c_7160, console_clock_get_date, "ConsoleClock_GetDate"),
    (0x002c_7208, date_days_in_month, "Date_DaysInMonth"),
    (0x002c_7260, date_add_days, "Date_AddDays"),
    (0x002c_73c0, console_clock_update, "ConsoleClock_Update"),
    (0x002c_a408, menu_sys_item_get_left_axis, "MenuSysItem_GetLeftAxis"),
    (0x002c_a428, menu_sys_item_get_right_axis, "MenuSysItem_GetRightAxis"),
];

/// Format strings in the game image: `yyyy:mm:dd` (mode 0) and the
/// `mm:dd:yyyy` / `dd:mm:yyyy` pair (modes 1 and 2).
const FMT_YEAR_FIRST: u32 = 0x0048_6540;
const FMT_DAY_OR_MONTH_FIRST: u32 = 0x0048_6550;

/// The EE `div` on signed 32-bit words: (LO, HI) with the hardware's results
/// for a zero divisor and for `MIN / -1`.
fn ee_div_s32_lo_hi(n: i32, d: i32) -> (i32, i32) {
    if d == 0 {
        (if n < 0 { 1 } else { -1 }, n)
    } else if n == i32::MIN && d == -1 {
        (i32::MIN, 0)
    } else {
        (n / d, n % d)
    }
}

// ---------------------------------------------------------------------------
// `ConsoleClock_Update` (0x002c73c0).
//
// Advances the clock by the whole ticks elapsed since its last update: the
// ticks are `(now - last) / period` with `now` and `period` from the game
// timer block behind `gp - 0x848`. When there are ticks, the time of day
// advances (`Time_AddSeconds` returns the days left over) and the date
// advances by those days (`Date_AddDays`).

pub fn console_clock_update(c: &mut Cpu) {
    guest(c, console_clock_update_body)
}

fn console_clock_update_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lw(c);
    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    let (q, r) = ee_div_s32_lo_hi(c.g[V1][0] as u32 as i32, c.g[V0][0] as u32 as i32);
    c.lo[0] = sx(q as u32);
    c.hi[0] = sx(r as u32);

    // Ticks elapsed (LO of the division).
    c.cycles += 3;
    c.g[T0][0] = c.lo[0];
    let no_ticks = c.g[T0][0] == 0;
    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
    if !no_ticks {
        c.cycles += 7;
        let stored = c.g[A0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x20)).sw(c, stored);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        Callee {
            addr: 0x002c_7328,
            f: ee::s08::f_002c7328,
        }
        .call(c, 0x002c_7410)?;
        let no_days_left = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_days_left {
            c.cycles += 5;
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            Callee {
                addr: 0x002c_7260,
                f: ee::s08::f_002c7260,
            }
            .call(c, 0x002c_742c)?;
        }
    }

    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// `ConsoleClock_GetTime` (0x002c7100) and `ConsoleClock_GetDate` (0x002c7160).
//
// Both update the clock first, then copy three of its words to the caller's
// pointers (`a1`, `a2`, `a3`). Time reads `+0x8`, `+0xc`, `+0x10`; date reads
// `+0x1c`, `+0x18`, `+0x14`.

/// `void ConsoleClock_GetTime(int* a, int* b, int* c)` (0x2c7100).
pub fn console_clock_get_time(c: &mut Cpu) {
    guest(c, |c| clock_read3_body(c, 0x002c_712c, [0x8, 0xc, 0x10]))
}

/// `void ConsoleClock_GetDate(int* a, int* b, int* c)` (0x2c7160).
pub fn console_clock_get_date(c: &mut Cpu) {
    guest(c, |c| clock_read3_body(c, 0x002c_718c, [0x1c, 0x18, 0x14]))
}

fn clock_read3_body(c: &mut Cpu, update_ret: u32, fields: [u32; 3]) -> Flow {
    c.cycles += 24;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x40);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x30);
    frame.save(c, S2, 0x20);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x10);
    c.g[S2][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[S3][0] = c.g[A3][0];
    Callee {
        addr: 0x002c_73c0,
        f: ee::s08::f_002c73c0,
    }
    .call(c, update_ret)?;

    let this = c.g[S0][0] as u32;
    c.g[V0][0] = Addr(this.wrapping_add(fields[0])).lw(c);
    frame.restore_ra(c, 0);
    let stored = c.g[V0][0];
    Addr(c.g[S1][0] as u32).sw(c, stored);
    frame.restore(c, S1, 0x30);
    c.g[V1][0] = Addr(this.wrapping_add(fields[1])).lw(c);
    let stored = c.g[V1][0];
    Addr(c.g[S2][0] as u32).sw(c, stored);
    frame.restore(c, S2, 0x20);
    c.g[V0][0] = Addr(this.wrapping_add(fields[2])).lw(c);
    frame.restore(c, S0, 0x40);
    let stored = c.g[V0][0];
    Addr(c.g[S3][0] as u32).sw(c, stored);
    frame.restore(c, S3, 0x10);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// `Date_DaysInMonth` (0x002c7208).
//
// `int Date_DaysInMonth(self, int month, int year)`: 28 or 29 for February
// (leap years from `Date_IsLeapYear`), else 31 or 30: `m += (m >= 8)`, then
// an odd `m` has 31 days.

pub fn date_days_in_month(c: &mut Cpu) {
    guest(c, date_days_in_month_body)
}

fn date_days_in_month_body(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 2;
    let february = c.g[A1][0] == c.g[V0][0];
    frame.save_ra(c, 0);
    if february {
        c.cycles += 6;
        c.g[A1][0] = c.g[A2][0];
        Callee {
            addr: 0x002c_71c0,
            f: ee::s08::f_002c71c0,
        }
        .call(c, 0x002c_7240)?;
        c.g[A0][0] = 0x1d;
        c.g[V1][0] = 0x1c;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.g[V1][0];
    } else {
        c.cycles += 8;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 8) as u64;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        if c.g[V1][0] == 0 {
            c.g[A1][0] = c.g[V0][0];
        }
        c.g[A0][0] = 0x1e;
        c.g[V1][0] = c.g[A1][0] & 1;
        c.g[V0][0] = 0x1f;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.g[A0][0];
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// `Date_AddDays` (0x002c7260).
//
// `void Date_AddDays(self, int* day, int* month, int* year, int count)`:
// moves the date forward `count` days, month by month.

pub fn date_add_days(c: &mut Cpu) {
    guest(c, date_add_days_body)
}

fn date_add_days_body(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[T0][0];
    frame.save(c, S2, 0x30);
    c.g[S1][0] = c.g[A2][0];
    frame.save(c, S3, 0x20);
    c.g[S2][0] = c.g[A3][0];
    frame.save(c, S4, 0x10);
    c.g[S3][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    let no_days = (c.g[S0][0] as i64) <= 0;
    c.g[S4][0] = c.g[A0][0];
    if !no_days {
        c.cycles += 1;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
        loop {
            // Days left in the current month, from the day to the month's end.
            c.cycles += 9;
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = Addr(c.g[S2][0] as u32).lw(c);
            Callee {
                addr: 0x002c_7208,
                f: ee::s08::f_002c7208,
            }
            .call(c, 0x002c_72a4)?;
            c.g[A0][0] = Addr(c.g[S3][0] as u32).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let fits_month = c.g[V0][0] == 0;
            c.g[A2][0] = 1;
            if fits_month {
                c.cycles += 2;
                let stored = c.g[V1][0];
                Addr(c.g[S3][0] as u32).sw(c, stored);
                c.g[S0][0] = 0;
            } else {
                c.cycles += 9;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                let stored = c.g[A2][0];
                Addr(c.g[S3][0] as u32).sw(c, stored);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 13) as u64;
                let same_year = c.g[V1][0] != 0;
                let stored = c.g[V0][0];
                Addr(c.g[S1][0] as u32).sw(c, stored);
                if !same_year {
                    c.cycles += 5;
                    let stored = c.g[A2][0];
                    Addr(c.g[S1][0] as u32).sw(c, stored);
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let stored = c.g[V0][0];
                    Addr(c.g[S2][0] as u32).sw(c, stored);
                }
            }
            // Back edge: more days to add?
            c.cycles += 2;
            if (c.g[S0][0] as i64) > 0 {
                c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_7298);
                }
            } else {
                break;
            }
        }
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
}

// ---------------------------------------------------------------------------
// `cConsoleConfig::setDateString` (0x002c7038), `ConsoleClock_FormatDate`.
//
// `void setDateString(char* dst, int size, int day, int month, int year,
// int mode)`: formats the date into a 0x40-byte buffer with one of three
// orders (mode 0 `yyyy:mm:dd`, mode 1 `mm:dd:yyyy`, mode 2 `dd:mm:yyyy`) and
// copies at most `size` 16-bit units to `dst`. Any other mode copies the
// buffer as it is.

pub fn console_clock_format_date(c: &mut Cpu) {
    guest(c, format_date_body)
}

fn format_date_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    c.g[V1][0] = c.g[A3][0]; // day
    frame.save(c, S0, 0x60);
    c.g[A3][0] = c.g[T0][0]; // month
    frame.save(c, S1, 0x50);
    c.g[S0][0] = c.g[A1][0]; // dst
    frame.save_ra(c, 0x40);
    c.g[S1][0] = c.g[A2][0]; // size
    c.g[V0][0] = 1;
    let mode_is_1 = c.g[T2][0] == c.g[V0][0];
    c.g[T0][0] = c.g[T1][0]; // year

    if mode_is_1 {
        // mm:dd:yyyy
        c.cycles += 4;
        c.g[A1][0] = sx(FMT_DAY_OR_MONTH_FIRST);
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = c.g[V1][0];
        format_sprintf(c, frame, 4, 0x002c_70bc)?;
        c.g[A0][0] = c.g[S0][0];
        return format_date_copy(c, frame);
    }

    c.cycles += 3;
    let before_two = (c.g[T2][0] as i64) < 2;
    c.g[V0][0] = before_two as u64;
    let at_least_two = c.g[V0][0] == 0;
    c.g[V0][0] = 2;
    if at_least_two {
        c.cycles += 2;
        let mode_is_2 = c.g[T2][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if mode_is_2 {
            // dd:mm:yyyy
            c.g[A1][0] = sx(FMT_DAY_OR_MONTH_FIRST);
            c.g[A2][0] = c.g[V1][0];
            format_sprintf(c, frame, 6, 0x002c_70d8)?;
            c.g[A0][0] = c.g[S0][0];
            return format_date_copy(c, frame);
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[S1][0];
        return format_date_tail(c, frame);
    }

    c.cycles += 2;
    let mode_is_0 = c.g[T2][0] == 0;
    c.g[A0][0] = c.g[S0][0];
    if mode_is_0 {
        // yyyy:mm:dd
        c.cycles += 5;
        c.g[A1][0] = sx(FMT_YEAR_FIRST);
        c.g[A2][0] = c.g[T0][0];
        c.g[T0][0] = c.g[V1][0];
        format_sprintf(c, frame, 4, 0x002c_70bc)?;
        c.g[A0][0] = c.g[S0][0];
        return format_date_copy(c, frame);
    }
    c.cycles += 2;
    c.g[A2][0] = c.g[S1][0];
    format_date_tail(c, frame)
}

/// The buffer formatter (`UStr_SprintfAsciiFmt_2(buf, fmt, a2, a3, t0)`) with
/// `cycles` charged before the call, returning to `ret`.
fn format_sprintf(c: &mut Cpu, _frame: Frame, cycles: u64, ret: u32) -> Flow {
    c.cycles += cycles;
    c.g[A0][0] = c.g[SP][0];
    Callee {
        addr: 0x002c_27c0,
        f: ee::s08::f_002c27c0,
    }
    .call(c, ret)
}

/// The end of the formatter: `a2 = size`, then the copy.
fn format_date_copy(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A2][0] = c.g[S1][0];
    format_date_tail(c, frame)
}

/// `UStr_CopyN(dst, buf, size)` and the epilogue.
fn format_date_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[A1][0] = c.g[SP][0];
    Callee {
        addr: 0x002c_25b8,
        f: ee::s08::f_002c25b8,
    }
    .call(c, 0x002c_70e8)?;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// `MenuSysItem_GetLeftAxis` (0x002ca408) and `MenuSysItem_GetRightAxis`
// (0x002ca428): the item's menu system's axis value, from `item->sys +0x124`.

pub fn menu_sys_item_get_left_axis(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_item_axis_body(
            c,
            Callee {
                addr: 0x002c_c260,
                f: ee::s09::f_002cc260,
            },
            0x002c_a41c,
        )
    })
}

pub fn menu_sys_item_get_right_axis(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_item_axis_body(
            c,
            Callee {
                addr: 0x002c_c280,
                f: ee::s09::f_002cc280,
            },
            0x002c_a43c,
        )
    })
}

fn menu_sys_item_axis_body(c: &mut Cpu, axis: Callee, ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x124)).lw(c);
    axis.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    /// The ConsoleClock object and the game timer block behind `gp - 0x848`.
    const CLOCK: u32 = DATA + 0x100;
    const TIMER: u32 = DATA + 0x200;
    /// Output words for the getters (`a1`, `a2`, `a3`).
    const OUT: u32 = DATA + 0x300;

    /// A machine with a running clock: time of day, date, the last tick, and a
    /// timer whose `now - last` is sometimes zero, sometimes long.
    fn clock_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), TIMER);
        let now = r.next_u32();
        let diff = r.next_u32() % 4_000_000;
        let last = if seed % 7 == 0 {
            now.wrapping_add(diff)
        } else {
            now.wrapping_sub(diff)
        };
        let period = [0u32, 1, 60, 3600, 7, 1000][(seed % 6) as usize];
        c.bus.write32(TIMER + 0x10, period);
        c.bus.write32(TIMER + 0x18, now);
        c.bus.write32(CLOCK + 0x20, last);
        c.bus.write32(CLOCK + 0x8, r.next_u32() % 60);
        c.bus.write32(CLOCK + 0xc, r.next_u32() % 60);
        c.bus.write32(CLOCK + 0x10, r.next_u32() % 24);
        c.bus.write32(CLOCK + 0x14, 1990 + r.next_u32() % 60);
        c.bus.write32(CLOCK + 0x18, 1 + r.next_u32() % 12);
        c.bus.write32(CLOCK + 0x1c, 1 + r.next_u32() % 28);
        c.g[A0][0] = sx(CLOCK);
        r
    }

    #[test]
    fn console_clock_update_matches() {
        check(
            ee::s08::f_002c73c0__generated,
            console_clock_update,
            200,
            &|c, seed| {
                clock_machine(c, seed, 0x002c_73c0);
            },
        );
    }

    #[test]
    fn console_clock_get_time_matches() {
        check(
            ee::s08::f_002c7100__generated,
            console_clock_get_time,
            200,
            &|c, seed| {
                clock_machine(c, seed, 0x002c_7100);
                c.g[A1][0] = sx(OUT);
                c.g[A2][0] = sx(OUT + 4);
                c.g[A3][0] = sx(OUT + 8);
            },
        );
    }

    #[test]
    fn console_clock_get_date_matches() {
        check(
            ee::s08::f_002c7160__generated,
            console_clock_get_date,
            200,
            &|c, seed| {
                clock_machine(c, seed, 0x002c_7160);
                c.g[A1][0] = sx(OUT);
                c.g[A2][0] = sx(OUT + 4);
                c.g[A3][0] = sx(OUT + 8);
            },
        );
    }

    #[test]
    fn date_days_in_month_matches() {
        check(
            ee::s08::f_002c7208__generated,
            date_days_in_month,
            200,
            &|c, seed| {
                machine(c, seed, 0x002c_7208, 0);
                c.g[A1][0] = (seed % 14) as u64;
                c.g[A2][0] = (1896 + seed % 230) as u64;
            },
        );
    }

    #[test]
    fn date_add_days_matches() {
        check(
            ee::s08::f_002c7260__generated,
            date_add_days,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_7260, 0);
                c.bus.write32(DATA + 0x400, 1 + r.next_u32() % 31);
                c.bus.write32(DATA + 0x404, 1 + r.next_u32() % 12);
                c.bus.write32(DATA + 0x408, 1990 + r.next_u32() % 60);
                c.g[A0][0] = sx(CLOCK);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x404);
                c.g[A3][0] = sx(DATA + 0x408);
                c.g[T0][0] = sx((r.next_u32() % 140).wrapping_sub(10));
            },
        );
    }

    #[test]
    fn console_clock_format_date_matches() {
        check(
            ee::s08::f_002c7038__generated,
            console_clock_format_date,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_7038, 0);
                elf_image(c);
                c.g[A0][0] = sx(CLOCK);
                c.g[A1][0] = sx(DATA + 0x600);
                c.g[A2][0] = (r.next_u32() % 40) as u64;
                c.g[A3][0] = (r.next_u32() % 40) as u64;
                c.g[T0][0] = (r.next_u32() % 14) as u64;
                c.g[T1][0] = (1900 + r.next_u32() % 300) as u64;
                c.g[T2][0] = [0u64, 1, 2, 3, u64::MAX][(seed % 5) as usize];
            },
        );
    }

    /// A menu item: `*item` is the menu system, whose `+0x124` is the menu
    /// whose `+0x5c` is the input binding (zeroed).
    fn menu_item_machine(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        let (item, sys, menu, bind) = (DATA + 0x800, DATA + 0x880, DATA + 0x900, DATA + 0x980);
        c.bus.write32(item, sys);
        c.bus.write32(sys + 0x124, menu);
        c.bus.write32(menu + 0x5c, bind);
        c.g[A0][0] = sx(item);
    }

    #[test]
    fn menu_sys_item_get_left_axis_matches() {
        check(
            ee::s09::f_002ca408__generated,
            menu_sys_item_get_left_axis,
            100,
            &|c, seed| menu_item_machine(c, seed, 0x002c_a408),
        );
    }

    #[test]
    fn menu_sys_item_get_right_axis_matches() {
        check(
            ee::s09::f_002ca428__generated,
            menu_sys_item_get_right_axis,
            100,
            &|c, seed| menu_item_machine(c, seed, 0x002c_a428),
        );
    }
}
