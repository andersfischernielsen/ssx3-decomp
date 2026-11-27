//! Staging module oc974 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc974`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001f_6890,
    c_fe_state_career_stats_show_records_page,
    "cFEStateCareerStats_ShowRecordsPage",
)];

// Offsets of the 0x54-byte riders-best record copied to `sp + 0x70`.
const SC_REC: u32 = 0x70;

/// `sprintf(buf, fmt, ...)` (0x417828) returning to `ret`.
fn sc_sprintf(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    call_ptr(c, 0x0041_7828, ret)
}

/// `cFEStateCareerStats_SetText(self, name, text)` (0x1f6840).
fn sc_set_text(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    call_ptr(c, 0x001f_6840, ret)
}

fn sc_add(c: &Cpu, reg: usize, off: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(off))
}

fn sc_sub(c: &Cpu, reg: usize, off: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_sub(off))
}

/// One text row, the record at `sp + 0x70 + ..` has `n > 0`: format `n`
/// with `fmt`, set the value text, look up the rider name (`s3`, id in `s1`)
/// and set the name text. `name_a`/`name_b` set the label address registers.
fn sc_row_positive(
    c: &mut Cpu,
    cycles: u64,
    fmt: u32,
    rets: [u32; 5],
    name_a: &dyn Fn(&mut Cpu),
    name_b: &dyn Fn(&mut Cpu),
) -> crate::call::Flow {
    c.cycles += cycles;
    c.g[A1][0] = 0x4a0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = sc_add(c, A1, fmt);
    sc_sprintf(c, rets[0])?;
    name_a(c);
    c.g[A2][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S2][0];
    sc_set_text(c, rets[1])?;
    sc_name_and_set(c, rets[2], rets[3], rets[4], name_b)
}

/// The shared tail of a positive row: rider name lookup, format, set text.
fn sc_name_and_set(
    c: &mut Cpu,
    ret_name: u32,
    ret_fmt: u32,
    ret_set: u32,
    name_b: &dyn Fn(&mut Cpu),
) -> crate::call::Flow {
    c.g[S0][0] = 0x4a0000;
    c.g[A1][0] = c.g[S1][0];
    c.g[S0][0] = sc_add(c, S0, 0x23c0);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0014_4c60, ret_name)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[SP][0];
    sc_sprintf(c, ret_fmt)?;
    name_b(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[SP][0];
    sc_set_text(c, ret_set)
}

/// One text row with `n <= 0`: "none" placeholders for both texts.
fn sc_row_empty(
    c: &mut Cpu,
    cycles: u64,
    rets: [u32; 2],
    name_a: &dyn Fn(&mut Cpu),
    name_b: &dyn Fn(&mut Cpu),
) -> crate::call::Flow {
    c.cycles += cycles;
    c.g[A2][0] = 0x4a0000;
    name_a(c);
    c.g[A2][0] = sc_add(c, A2, 0x2108);
    c.g[A0][0] = c.g[S2][0];
    sc_set_text(c, rets[0])?;
    c.g[A2][0] = 0x4a0000;
    name_b(c);
    c.g[A2][0] = sc_add(c, A2, 0x2168);
    c.g[A0][0] = c.g[S2][0];
    sc_set_text(c, rets[1])
}

/// Row prologue: `a2` holds `n`; load the rider id into `s1` and branch.
fn sc_row(
    c: &mut Cpu,
    pre_cycles: u64,
    id_off: u32,
    positive: &dyn Fn(&mut Cpu) -> crate::call::Flow,
    empty: &dyn Fn(&mut Cpu) -> crate::call::Flow,
) -> crate::call::Flow {
    c.cycles += pre_cycles;
    let none = (c.g[A2][0] as i64) <= 0;
    c.g[S1][0] = c.lw(sp(c).wrapping_add(id_off));
    if none {
        empty(c)
    } else {
        positive(c)
    }
}

/// `void cFEStateCareerStats_ShowRecordsPage(cFEStateCareerStats* self)`
/// (0x1f6890): copy the riders-best record of the current character (BE
/// interface 8) to the stack and fill the records page: for each of seven
/// records (and the last "-"/count one) the value text and the name of the
/// rider that holds it ("none" placeholders when the count is 0).
pub fn c_fe_state_career_stats_show_records_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x170);
        frame.save(c, 18, 0x140);
        frame.save(c, 16, 0x160);
        frame.save(c, 19, 0x130);
        frame.save(c, 17, 0x150);
        frame.save(c, 20, 0x120);
        frame.save(c, 21, 0x110);
        frame.save(c, 22, 0x100);
        frame.save(c, 23, 0xf0);
        frame.save(c, 30, 0xe0);
        frame.save_ra(c, 0xd0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_68c4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001f_68d0)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_68d8)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001f_68e4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x001f_68f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0015_5288, 0x001f_6900)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = sc_add(c, SP, SC_REC);
        // Copy the first 0x40 bytes of the record.
        c.cycles += 7;
        c.g[V0][0] = sc_add(c, V1, 0x40);
        c.g[S0][0] = 0x4a0000;
        c.g[S4][0] = 0x470000;
        c.g[S5][0] = 0x470000;
        c.g[S6][0] = 0x470000;
        c.g[S7][0] = 0x4a0000;
        c.g[FP][0] = 0x470000;
        if !aligned {
            loop {
                c.cycles += 19;
                let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
                for (k, reg) in [5usize, 6, 7, 8].into_iter().enumerate() {
                    let o = 8 * k as u32;
                    c.ldl(reg, src.wrapping_add(o + 7));
                    c.ldr(reg, src.wrapping_add(o));
                }
                for (k, reg) in [5usize, 6, 7, 8].into_iter().enumerate() {
                    let o = 8 * k as u32;
                    c.sdl(reg, dst.wrapping_add(o + 7));
                    c.sdr(reg, dst.wrapping_add(o));
                }
                c.g[V1][0] = sc_add(c, V1, 0x20);
                let again = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sc_add(c, A0, 0x20);
                if !again {
                    c.cycles += 2;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001f_692c);
                }
            }
        } else {
            loop {
                c.cycles += 11;
                let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
                c.g[A1][0] = c.ld(src);
                c.g[A2][0] = c.ld(src.wrapping_add(8));
                c.g[A3][0] = c.ld(src.wrapping_add(0x10));
                c.g[T0][0] = c.ld(src.wrapping_add(0x18));
                c.sd(dst, c.g[A1][0]);
                c.sd(dst.wrapping_add(8), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
                c.g[V1][0] = sc_add(c, V1, 0x20);
                let again = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sc_add(c, A0, 0x20);
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001f_699c);
                }
            }
        }
        // The last 0x14 bytes.
        c.cycles += 13;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
        c.ldl(2, src.wrapping_add(7));
        c.ldr(2, src);
        c.ldl(5, src.wrapping_add(0xf));
        c.ldr(5, src.wrapping_add(8));
        c.g[A2][0] = c.lw(src.wrapping_add(0x10));
        c.sdl(2, dst.wrapping_add(7));
        c.sdr(2, dst);
        c.sdl(5, dst.wrapping_add(0xf));
        c.sdr(5, dst.wrapping_add(8));
        c.sw(dst.wrapping_add(0x10), c.g[A2][0]);
        c.g[A2][0] = c.lw(sp(c).wrapping_add(SC_REC));

        // Row 1 (n at +0x70, id +0x74).
        sc_row(
            c,
            0,
            0x74,
            &|c| {
                sc_row_positive(
                    c,
                    23,
                    0x2138,
                    [
                        0x001f_6a0c,
                        0x001f_6a1c,
                        0x001f_6a30,
                        0x001f_6a40,
                        0x001f_6a50,
                    ],
                    &|c| c.g[A1][0] = sc_add(c, S0, 0x23b8),
                    &|c| c.g[A1][0] = sc_sub(c, S4, 0xc48),
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    11,
                    [0x001f_6a6c, 0x001f_6a80],
                    &|c| c.g[A1][0] = sc_add(c, S0, 0x23b8),
                    &|c| c.g[A1][0] = sc_sub(c, S4, 0xc48),
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x98));
        // Row 2 (n +0x98, id +0x9c).
        sc_row(
            c,
            2,
            0x9c,
            &|c| {
                sc_row_positive(
                    c,
                    23,
                    0x23c8,
                    [
                        0x001f_6a9c,
                        0x001f_6aac,
                        0x001f_6ac0,
                        0x001f_6ad0,
                        0x001f_6ae0,
                    ],
                    &|c| c.g[A1][0] = sc_sub(c, S5, 0xc38),
                    &|c| c.g[A1][0] = sc_sub(c, S6, 0xc28),
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    11,
                    [0x001f_6afc, 0x001f_6b10],
                    &|c| c.g[A1][0] = sc_sub(c, S5, 0xc38),
                    &|c| c.g[A1][0] = sc_sub(c, S6, 0xc28),
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x78));
        // Row 3 (n +0x78, id +0x7c).
        sc_row(
            c,
            2,
            0x7c,
            &|c| {
                sc_row_positive(
                    c,
                    23,
                    0x2138,
                    [
                        0x001f_6b2c,
                        0x001f_6b3c,
                        0x001f_6b50,
                        0x001f_6b60,
                        0x001f_6b70,
                    ],
                    &|c| c.g[A1][0] = sc_add(c, S7, 0x23d0),
                    &|c| c.g[A1][0] = sc_sub(c, FP, 0xc10),
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    11,
                    [0x001f_6b8c, 0x001f_6ba0],
                    &|c| c.g[A1][0] = sc_add(c, S7, 0x23d0),
                    &|c| c.g[A1][0] = sc_sub(c, FP, 0xc10),
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x80));
        // Row 4 (n +0x80, id +0x84).
        sc_row(
            c,
            2,
            0x84,
            &|c| {
                sc_row_positive(
                    c,
                    25,
                    0x2138,
                    [
                        0x001f_6bbc,
                        0x001f_6bd0,
                        0x001f_6be4,
                        0x001f_6bf4,
                        0x001f_6c08,
                    ],
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xc00);
                    },
                    &|c| {
                        c.g[V0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, V0, 0xbf0);
                    },
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6c28, 0x001f_6c40],
                    &|c| {
                        c.g[A3][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A3, 0xc00);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xbf0);
                    },
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x88));
        // Row 5 (n +0x88, id +0x8c).
        sc_row(
            c,
            2,
            0x8c,
            &|c| {
                sc_row_positive(
                    c,
                    25,
                    0x2138,
                    [
                        0x001f_6c5c,
                        0x001f_6c70,
                        0x001f_6c84,
                        0x001f_6c94,
                        0x001f_6ca8,
                    ],
                    &|c| {
                        c.g[V0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, V0, 0xbd8);
                    },
                    &|c| {
                        c.g[A2][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A2, 0xbc8);
                    },
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6cc8, 0x001f_6ce0],
                    &|c| {
                        c.g[A3][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A3, 0xbd8);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xbc8);
                    },
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0xa0));
        // Row 6 (n +0xa0, id +0xa4): a percentage, n * 0.01 through 0x413af8.
        sc_row(
            c,
            2,
            0xa4,
            &|c| {
                c.cycles += 32;
                c.mtc1(6, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5548));
                c.g[S0][0] = 0x470000;
                c.g[S0][0] = sc_sub(c, S0, 0xbb0);
                c.mul_s(12, 0, 12);
                call_ptr(c, 0x0041_3af8, 0x001f_6d08)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                sc_sprintf(c, 0x001f_6d18)?;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = sc_sub(c, V0, 0xba0);
                c.g[A0][0] = c.g[S2][0];
                sc_set_text(c, 0x001f_6d2c)?;
                sc_name_and_set(c, 0x001f_6d40, 0x001f_6d50, 0x001f_6d64, &|c| {
                    c.g[A2][0] = 0x470000;
                    c.g[A1][0] = sc_sub(c, A2, 0xb90);
                })
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6d84, 0x001f_6d9c],
                    &|c| {
                        c.g[A3][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A3, 0xba0);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xb90);
                    },
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0xa8));
        // Row 7 (n +0xa8, id +0xac).
        sc_row(
            c,
            2,
            0xac,
            &|c| {
                c.cycles += 25;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sc_add(c, A1, 0x23c8);
                sc_sprintf(c, 0x001f_6db8)?;
                c.g[V0][0] = 0x4a0000;
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = sc_add(c, V0, 0x23d8);
                c.g[A0][0] = c.g[S2][0];
                sc_set_text(c, 0x001f_6dcc)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sc_add(c, S0, 0x23c0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0014_4c60, 0x001f_6de0)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                sc_sprintf(c, 0x001f_6df0)?;
                c.g[A2][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sc_sub(c, A2, 0xb80);
                c.g[A2][0] = c.g[SP][0];
                sc_set_text(c, 0x001f_6e04)
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6e24, 0x001f_6e3c],
                    &|c| {
                        c.g[A3][0] = 0x4a0000;
                        c.g[A1][0] = sc_add(c, A3, 0x23d8);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xb80);
                    },
                )
            },
        )?;
        c.g[V0][0] = 0x470000;
        // Last row: set the title text, then the "n" / "-" text.
        c.cycles += 15;
        c.g[A0][0] = sc_sub(c, V0, 0xb70);
        call_ptr(c, 0x0031_7670, 0x001f_6e48)?;
        c.g[A0][0] = c.lw(p_off(c.g[S2][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_6e54)?;
        c.g[A1][0] = 0x470000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sc_sub(c, A1, 0xb58);
        call_ptr(c, 0x0031_7670, 0x001f_6e64)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001f_6e70)?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x90));
        let none = (c.g[A2][0] as i64) <= 0;
        c.g[A1][0] = 0x4a0000;
        if none {
            c.cycles += 4;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = c.lhu(sc_add(c, V0, 0x2108) as u32);
            let buf = sp(c);
            c.sh(buf, c.g[V1][0]);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sc_add(c, A1, 0x2138);
            sc_sprintf(c, 0x001f_6e88)?;
        }
        c.g[A2][0] = 0x470000;
        c.cycles += 16;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sc_sub(c, A2, 0xb40);
        c.g[A2][0] = c.g[SP][0];
        sc_set_text(c, 0x001f_6eb0)?;
        frame.restore(c, 16, 0x160);
        frame.restore(c, 17, 0x150);
        frame.restore(c, 18, 0x140);
        frame.restore(c, 19, 0x130);
        frame.restore(c, 20, 0x120);
        frame.restore(c, 21, 0x110);
        frame.restore(c, 22, 0x100);
        frame.restore(c, 23, 0xf0);
        frame.restore(c, 30, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

fn p_off(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine_rt as machine, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA};

    /// `ShowRecordsPage` against the generated body: aligned and unaligned
    /// record copies, every row with n > 0, n == 0 and n < 0, and call
    /// unwinds (all callees are faked; the SetText and sprintf calls are
    /// compared through the call log).
    #[test]
    fn c_fe_state_career_stats_show_records_page_matches() {
        check(
            ee::s04::f_001f6890__generated,
            c_fe_state_career_stats_show_records_page,
            400,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x1f6890, 40);
                let this = DATA + 0x100;
                let screen = DATA + 0x400;
                // The record the 6th call (0x155288) returns, 8-aligned or not.
                let rec = DATA + 0x2000 + if seed % 2 == 0 { 0 } else { 1 + (seed / 2) % 7 };
                c.bus.write32(LOG_RESULTS + 4 * 5, rec);
                c.bus.write32(this + 0x40, screen);
                c.g[A0][0] = sx(this);
                // Counts at the row offsets: > 0, 0 or < 0 by seed bits.
                for (k, off) in [0u32, 0x28, 8, 0x10, 0x18, 0x30, 0x38, 0x20]
                    .into_iter()
                    .enumerate()
                {
                    let kind = (seed >> k) as u64 ^ r.next_u32() as u64;
                    let n: u32 = match kind % 3 {
                        0 => 0,
                        1 => 1 + r.next_u32() % 5000,
                        _ => (seed >> 3) | 0x8000_0000,
                    };
                    let n = if seed % 5 == 0 && k == 0 { 0 } else { n };
                    c.bus.write32(rec + off, n);
                    c.bus.write32(rec + off + 4, r.next_u32() % 40);
                }
                // 0.01f for the percentage row.
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_sub(0x5548), 0x3c23_d70a);
            },
        );
    }
}
