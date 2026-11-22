//! Heap-owned wide-string storage and line indexing for native UIText.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_0eb0,
    ui_text_set_wide_string_internal,
    "UIText_SetWideStringInternal",
)];

const TEXT_FLAGS_SLOT: u32 = 0xc0;
const INVALIDATE_SLOT: u32 = 0x28;
const LINE_COUNT: u32 = 0xa8;
const LINES: u32 = 0xac;
const STRING: u32 = 0xb4;
const STRING_CAPACITY: u32 = 0xb8;
const LINE_CAPACITY: u32 = 0xba;
const SPLIT_HEAD: u32 = 0x003a_0fa8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void UIText_SetWideStringInternal(UIText* self, const u16* text)`
/// (0x3a0eb0). Invalidate text, select owned-string storage, free the old
/// buffers, then copy strings up to 800 characters. The u16 at +0xb8 is
/// the allocation's character capacity INCLUDING its NUL terminator.
/// For multiline text, create a line-pointer table and terminate each line
/// in the owned copy. This internal setter leaves the locale ID at +0xb0.
pub fn ui_text_set_wide_string_internal(c: &mut Cpu) {
    guest(c, |c| {
        let mut restore_s0 = true;
        match c.pc {
            0x003a_0eb0 => {
                c.cycles += 26;
                let frame = Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                c.g[A1][0] = 1;
                c.g[V0][0] = c.lw(at(c, S0, 8));
                c.g[A0][0] = c.lh(at(c, V0, INVALIDATE_SLOT));
                c.g[V1][0] = c.lw(at(c, V0, INVALIDATE_SLOT + 4));
                let target = r(c, V1);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x003a_0ee0)?;

                c.g[V0][0] = c.lw(at(c, S0, 8));
                c.g[A1][0] = 1;
                c.g[A0][0] = c.lh(at(c, V0, TEXT_FLAGS_SLOT));
                c.g[V1][0] = c.lw(at(c, V0, TEXT_FLAGS_SLOT + 4));
                let target = r(c, V1);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x003a_0ef8)?;

                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0b38, 0x003a_0f00)?;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003d_bca0, 0x003a_0f08)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < 801) as u64;
                let fits = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V1).wrapping_add(1));
                if fits {
                    c.cycles += 17;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0;
                    c.g[A0][0] = sx(r(c, V0) << 1);
                    c.g[A1][0] = sx(at(c, A1, 0x4088));
                    c.sh(at(c, S0, STRING_CAPACITY), c.g[V0][0]);
                    call_ptr(c, 0x0031_7e30, 0x003a_0f34)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.sw(at(c, S0, STRING), c.g[V0][0]);
                    call_ptr(c, 0x003d_be20, 0x003a_0f44)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_0b90, 0x003a_0f4c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
                    let one_line = c.g[V0][0] != 0;
                    c.sw(at(c, S0, LINE_COUNT), c.g[A0][0]);
                    if !one_line {
                        c.cycles += 16;
                        c.g[A1][0] = 0x0049_0000;
                        c.g[A0][0] = sx(r(c, A0) << 2);
                        c.g[A1][0] = sx(at(c, A1, 0x4098));
                        c.g[A2][0] = 0x100;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7e30, 0x003a_0f74)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw(at(c, S0, STRING));
                        c.g[V0][0] = c.lhu(at(c, S0, LINE_COUNT));
                        c.sw(at(c, S0, LINES), c.g[A1][0]);
                        c.g[A0][0] = c.g[V1][0];
                        c.sh(at(c, S0, LINE_CAPACITY), c.g[V0][0]);
                        c.sw(r(c, A1), c.g[V1][0]);
                        c.g[V0][0] = c.lhu(r(c, A0));
                        let empty = c.g[V0][0] == 0;
                        frame.restore(c, S0, 0x20);
                        if empty {
                            restore_s0 = false;
                        } else {
                            c.cycles += 3;
                            c.g[A2][0] = sx(at(c, A1, 4));
                            c.g[A3][0] = 0x0d;
                            c.g[V1][0] = c.lhu(r(c, A0));
                            split_lines(c)?;
                        }
                    }
                }
            }
            SPLIT_HEAD => split_lines(c)?,
            pc => {
                c.jump_out(pc, 0x003a_0eb0, 0x003a_1000);
                return Ok(());
            }
        }

        let frame = Frame { size: 0x30 };
        if restore_s0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn split_lines(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lhu(at(c, GP, 0x16b8));
        let separator = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = c.lhu(at(c, A0, 2));
        let doubled_separator = if separator {
            c.cycles += 2;
            c.g[A1][0] == c.g[V1][0]
        } else {
            false
        };

        let new_line = if doubled_separator {
            c.cycles += 3;
            c.sh(r(c, A0), 0);
            c.g[A0][0] = sx(at(c, A0, 4));
            true
        } else {
            c.cycles += 2;
            let carriage_return = c.g[V1][0] == c.g[A3][0];
            if carriage_return {
                c.cycles += 2;
                c.sh(r(c, A0), 0);
            }
            c.g[A0][0] = sx(at(c, A0, 2));
            carriage_return
        };
        if new_line {
            c.cycles += 2;
            c.sw(r(c, A2), c.g[A0][0]);
            c.g[A2][0] = sx(at(c, A2, 4));
        }

        c.cycles += 3;
        c.g[V0][0] = c.lhu(r(c, A0));
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.g[V1][0] = c.lhu(r(c, A0));
        if back_edge_unwinds(c, SPLIT_HEAD) {
            return Err(Unwound);
        }
    }
}
