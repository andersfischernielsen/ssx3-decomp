//! `src/util/statemachine.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::worldpainter::quadtree::create_with;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_6f78,
        console_clock_format_time,
        "ConsoleClock_FormatTime",
    ),
    (
        0x002c_7440,
        c_ps2_console_config_create,
        "cPS2ConsoleConfig_Create",
    ),
    (
        0x002c_7038,
        console_clock_format_date,
        "ConsoleClock_FormatDate",
    ),
    (0x002c_7100, console_clock_get_time, "ConsoleClock_GetTime"),
    (0x002c_7160, console_clock_get_date, "ConsoleClock_GetDate"),
    (0x002c_7208, date_days_in_month, "Date_DaysInMonth"),
    (0x002c_7260, date_add_days, "Date_AddDays"),
    (0x002c_73c0, console_clock_update, "ConsoleClock_Update"),
    (
        0x002c_7478,
        c_ps2_console_config_ctor,
        "cPS2ConsoleConfig_ctor",
    ),
    (
        0x002d_3fd8,
        c_ps2_console_config_v01,
        "cPS2ConsoleConfig_v01",
    ),
    (
        0x002d_4030,
        c_ps2_console_config_base_dtor,
        "cPS2ConsoleConfig_BaseDtor",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
}

/// `void* cPS2ConsoleConfig_Create()` (0x2c7440): allocate the 0x38-byte
/// console configuration and construct it.
pub fn c_ps2_console_config_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x38, 0x0048_6560, 0, 0x002c_7460, |c, ret| {
            callee::CONSOLE_CONFIG_CTOR.call(c, ret)
        })
    })
}

// ---------------------------------------------------------------------------
// `ConsoleClock_FormatTime` (0x002c6f78).
//
// `void ConsoleClock::FormatTime(char* out, int pad, int h, int m, int s,
//  int mode)`: format the clock into `out`.

pub fn console_clock_format_time(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c701c: {
            'b_2c7018: {
                'b_2c6fcc: {
                    'b_2c6fb4: {
                        c.cycles += 10;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S1][0] = c.g[A2][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[A2][0] = c.g[A3][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                        c.g[A3][0] = c.g[T0][0];
                        c.g[S0][0] = c.g[A1][0];
                        let t = c.g[T2][0] == 0u64;
                        c.g[T0][0] = c.g[T1][0];
                        if t {
                            break 'b_2c6fb4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[T2][0] == c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_2c6fcc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[A2][0] = c.g[S1][0];
                                if t {
                                    break 'b_2c701c;
                                } else {
                                    break 'b_2c6fb4;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002c6fc4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6500));
                    call_ptr(c, 0x002c27c0, 0x002c6fc4)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_2c7018;
                    } else {
                        break 'b_2c6fcc;
                    }
                }
                'b_2c6fec: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 12i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_2c6fec;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0x480000;
                        c.g[RA][0] = sx(0x002c6fe4);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6510));
                        call_ptr(c, 0x002c27c0, 0x002c6fe4)?;
                        let t = true;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2c7018;
                        } else {
                            break 'b_2c6fec;
                        }
                    }
                }
                'b_2c7000: {
                    c.cycles += 5;
                    c.g[V0][0] = 0xd;
                    c.g[A1][0] = 0x480000;
                    c.div(0, 6, 2);
                    if c.g[V0][0] == 0u64 {
                        break 'b_2c7000;
                    } else {
                        break 'b_2c7000;
                    }
                }
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6528));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.hi[0];
                c.g[RA][0] = sx(0x002c7014);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xc));
                call_ptr(c, 0x002c27c0, 0x002c7014)?;
                c.g[A0][0] = c.g[S0][0];
                break 'b_2c7018;
            }
            c.cycles += 1;
            c.g[A2][0] = c.g[S1][0];
            break 'b_2c701c;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x002c7024);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x002c25b8, 0x002c7024)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `cPS2ConsoleConfig::cPS2ConsoleConfig(this)` (0x002c7478): fills the console
/// config from the system queries: +0x24 = 0, +0x34 = the string at 0x486798,
/// +0x28 (video mode flag), +0x2c = 1, +0x30 (region index through the jump table
/// at 0x486580, 0..7), +0 and +4 (language and aspect ratio), then the
/// date/time bytes (BCD) into +8..+0x1c (a default 2000-ish date when the query
/// fails), and finally +0x20 = world->+0x18. Returns this. Kept as the generated
/// block machine, so every resume entry also runs natively.
pub fn c_ps2_console_config_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002c7478 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[V0][0] = 0x480000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6798));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x24), 0u64);
                    c.g[RA][0] = sx(0x002c74b0);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    call_ptr(c, 0x0040baa8, 0x002c74b0)?;
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002c74c8;
                        continue;
                    }
                    pc = 0x002c74bc;
                    continue;
                }
                0x002c74bc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    if t {
                        pc = 0x002c74cc;
                        continue;
                    }
                    pc = 0x002c74c8;
                    continue;
                }
                0x002c74c8 => {
                    c.cycles += 1;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x28), 0u64);
                    pc = 0x002c74cc;
                    continue;
                }
                0x002c74cc => {
                    c.cycles += 7;
                    c.g[S0][0] = 0x1;
                    c.g[RA][0] = sx(0x002c74d8);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x2c), c.g[S0][0]);
                    call_ptr(c, 0x0040b9e8, 0x002c74d8)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002c755c;
                        continue;
                    }
                    pc = 0x002c74e8;
                    continue;
                }
                0x002c74e8 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6580));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t;
                    continue;
                }
                0x002c7500 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), 0u64);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c7508;
                    continue;
                }
                0x002c7508 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c7514;
                    continue;
                }
                0x002c7514 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c7520;
                    continue;
                }
                0x002c7520 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c752c;
                    continue;
                }
                0x002c752c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c7538;
                    continue;
                }
                0x002c7538 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c7544;
                    continue;
                }
                0x002c7544 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c7550;
                    continue;
                }
                0x002c7550 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x7;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        pc = 0x002c7560;
                        continue;
                    }
                    pc = 0x002c755c;
                    continue;
                }
                0x002c755c => {
                    c.cycles += 1;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[S0][0]);
                    pc = 0x002c7560;
                    continue;
                }
                0x002c7560 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x002c7568);
                    call_ptr(c, 0x0040bb38, 0x002c7568)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x002c758c;
                        continue;
                    }
                    pc = 0x002c7578;
                    continue;
                }
                0x002c7578 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.sw(c.g[S5][0] as u32, 0u64);
                        pc = 0x002c7590;
                        continue;
                    }
                    pc = 0x002c7580;
                    continue;
                }
                0x002c7580 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw(c.g[S5][0] as u32, 0u64);
                        pc = 0x002c7590;
                        continue;
                    }
                    pc = 0x002c758c;
                    continue;
                }
                0x002c758c => {
                    c.cycles += 1;
                    c.sw(c.g[S5][0] as u32, c.g[V1][0]);
                    pc = 0x002c7590;
                    continue;
                }
                0x002c7590 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002c7598);
                    call_ptr(c, 0x0040bc08, 0x002c7598)?;
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002c75b4;
                        continue;
                    }
                    pc = 0x002c75a4;
                    continue;
                }
                0x002c75a4 => {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw((c.g[S5][0] as u32).wrapping_add(0x4), 0u64);
                        pc = 0x002c75b8;
                        continue;
                    }
                    pc = 0x002c75ac;
                    continue;
                }
                0x002c75ac => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    if t {
                        pc = 0x002c75b8;
                        continue;
                    }
                    pc = 0x002c75b4;
                    continue;
                }
                0x002c75b4 => {
                    c.cycles += 1;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x4), 0u64);
                    pc = 0x002c75b8;
                    continue;
                }
                0x002c75b8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002c75c0);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00402520, 0x002c75c0)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x7d3;
                    if t {
                        pc = 0x002c766c;
                        continue;
                    }
                    pc = 0x002c75c8;
                    continue;
                }
                0x002c75c8 => {
                    c.cycles += 41;
                    c.g[RA][0] = sx(0x002c75d0);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0040bfe0, 0x002c75d0)?;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x7));
                    c.g[V1][0] = 0xa;
                    c.g[T2][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
                    c.g[T3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2));
                    c.g[T1][0] = sx((c.g[V0][0] as u32) >> 4);
                    c.g[A3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x1));
                    c.g[T6][0] = sx((c.g[T2][0] as u32) >> 4);
                    c.g[T0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                    c.g[T5][0] = sx((c.g[T3][0] as u32) >> 4);
                    c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x5));
                    c.g[T4][0] = sx((c.g[A3][0] as u32) >> 4);
                    c.g[A1][0] = sx((c.g[T0][0] as u32) >> 4);
                    c.mult(0, 9, 9, 3);
                    c.g[A2][0] = sx((c.g[A0][0] as u32) >> 4);
                    c.mult(0, 14, 14, 3);
                    c.mult(0, 6, 6, 3);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.mult(0, 13, 13, 3);
                    c.g[T2][0] = c.g[T2][0] & 0xf;
                    c.mult(0, 12, 12, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    c.mult(0, 5, 5, 3);
                    c.g[T3][0] = c.g[T3][0] & 0xf;
                    c.g[A3][0] = c.g[A3][0] & 0xf;
                    c.g[T0][0] = c.g[T0][0] & 0xf;
                    c.g[A0][0] = c.g[A0][0] & 0xf;
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T6][0] as u32));
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T5][0] as u32));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d0));
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0xc), c.g[T3][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                    if t {
                        pc = 0x002c768c;
                        continue;
                    }
                    pc = 0x002c766c;
                    continue;
                }
                0x002c766c => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xa;
                    c.g[A0][0] = 0xe;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x10), 0u64);
                    pc = 0x002c768c;
                    continue;
                }
                0x002c768c => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V0][0] = c.g[S5][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002c7478, 0x002c76c0);
                    return Ok(());
                }
            }
        }
    });
}

/// Format strings in the game image: `yyyy:mm:dd` (mode 0) and the
/// `mm:dd:yyyy` / `dd:mm:yyyy` pair (modes 1 and 2).
pub(crate) const FMT_YEAR_FIRST: u32 = 0x0048_6540;

pub(crate) const FMT_DAY_OR_MONTH_FIRST: u32 = 0x0048_6550;

/// The EE `div` on signed 32-bit words: (LO, HI) with the hardware's results
/// for a zero divisor and for `MIN / -1`.
pub(crate) fn ee_div_s32_lo_hi(n: i32, d: i32) -> (i32, i32) {
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

pub(crate) fn console_clock_update_body(c: &mut Cpu) -> Flow {
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

pub(crate) fn clock_read3_body(c: &mut Cpu, update_ret: u32, fields: [u32; 3]) -> Flow {
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

pub(crate) fn date_days_in_month_body(c: &mut Cpu) -> Flow {
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

pub(crate) fn date_add_days_body(c: &mut Cpu) -> Flow {
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

pub(crate) fn format_date_body(c: &mut Cpu) -> Flow {
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
pub(crate) fn format_sprintf(c: &mut Cpu, _frame: Frame, cycles: u64, ret: u32) -> Flow {
    c.cycles += cycles;
    c.g[A0][0] = c.g[SP][0];
    Callee {
        addr: 0x002c_27c0,
        f: ee::s08::f_002c27c0,
    }
    .call(c, ret)
}

/// The end of the formatter: `a2 = size`, then the copy.
pub(crate) fn format_date_copy(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[A2][0] = c.g[S1][0];
    format_date_tail(c, frame)
}

/// `UStr_CopyN(dst, buf, size)` and the epilogue.
pub(crate) fn format_date_tail(c: &mut Cpu, frame: Frame) -> Flow {
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

/// `cPS2ConsoleConfig` deleting destructor (0x2d3fd8): the vtable 0x487480 at
/// +0x34, then `operator delete(this)` when bit 0 of the flags is set.
pub fn c_ps2_console_config_v01(c: &mut Cpu) {
    crate::game::visualfx::worldlightman::deleting_dtor(c, 0x34, 0x0048_7480, 0x002d_3ffc);
}

/// `cPS2ConsoleConfig` base destructor (0x2d4030): the same body as
/// [`c_ps2_console_config_v01`], returning from the delete to 0x2d4054.
pub fn c_ps2_console_config_base_dtor(c: &mut Cpu) {
    crate::game::visualfx::worldlightman::deleting_dtor(c, 0x34, 0x0048_7480, 0x002d_4054);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;

    #[test]
    fn c_ps2_console_config_create_matches() {
        check(
            ee::s08::f_002c7440__generated,
            c_ps2_console_config_create,
            60,
            &|c, seed| {
                alloc_machine(c, seed, 0x2c74_40);
            },
        );
    }

    #[test]
    fn console_clock_format_time_matches() {
        strict(
            ee::s08::f_002c6f78__generated,
            console_clock_format_time,
            80,
            &|c, seed| {
                let mut r = Rng(0x002c_6f78 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = (r.next_u32() % 16) as u64;
                c.g[A3][0] = (r.next_u32() % 60) as u64;
                c.g[T0][0] = (r.next_u32() % 60) as u64;
                c.g[T1][0] = (r.next_u32() % 8) as u64;
                c.g[T2][0] = [0u64, 1, 2][(seed % 3) as usize];
            },
        );
    }

    use crate::difftest::fakes::machine;

    pub(crate) use crate::game::dirtysock::dnasauth::tests::oc1527_ctor_setup;

    /// `cPS2ConsoleConfig_ctor`: every arm of the region jump table (0..7 and
    /// out of range), the language and aspect arms, the date present and
    /// absent, the resume entries and an unwind at each of the six calls.
    #[test]
    fn c_ps2_console_config_ctor_matches() {
        check(
            ee::s08::f_002c7478__generated,
            c_ps2_console_config_ctor,
            400,
            &|c, seed| oc1527_ctor_setup(c, seed, 6),
        );
    }

    /// The ConsoleClock object and the game timer block behind `gp - 0x848`.
    pub(crate) const CLOCK: u32 = DATA + 0x100;

    pub(crate) const TIMER: u32 = DATA + 0x200;

    /// Output words for the getters (`a1`, `a2`, `a3`).
    pub(crate) const OUT: u32 = DATA + 0x300;

    /// A machine with a running clock: time of day, date, the last tick, and a
    /// timer whose `now - last` is sometimes zero, sometimes long.
    pub(crate) fn clock_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
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

    #[test]
    fn c_ps2_console_config_v01_matches() {
        crate::game::visualfx::worldlightman::tests::dtor_case(
            ee::s09::f_002d3fd8__generated,
            c_ps2_console_config_v01,
            0x2d3fd8,
        );
    }

    #[test]
    fn c_ps2_console_config_base_dtor_matches() {
        crate::game::visualfx::worldlightman::tests::dtor_case(
            ee::s09::f_002d4030__generated,
            c_ps2_console_config_base_dtor,
            0x2d4030,
        );
    }
}
