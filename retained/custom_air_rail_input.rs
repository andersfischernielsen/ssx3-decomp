//! Native natural-air input0x12f730, retained uninstalled.
//!
//! Rail attachment is attempted after reset, tweak and handplant handling,
//! before boost/air-animation input. No imported-course or surface filter
//! occurs in this caller. No automated validation or installation is made.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0012_f730;
const END: u32 = 0x0012_fb68;
const EPILOGUE: u32 = 0x0012_fb48;
const RIDER: u32 = 0x14;
const ANIMATION: u32 = 0x784;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}
#[inline]
fn lit(c: &Cpu, offset: u32) -> u32 { r(c, GP).wrapping_sub(offset) }

/// `void RiderNaturalAirSub_HandleInput(state, packed_input)`.
/// Retains original stack writes and instruction-boundary continuations.
pub fn rider_natural_air_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 18;
                    Frame::open(c, 0x50);
                    c.g[A2][0] = 0;
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    for (register, offset) in [(23, 0x48), (22, 0x40), (21, 0x38), (20, 0x30)] {
                        frame.save_fpr(c, register, offset);
                    }
                    c.g[A1][0] = c.lw(r(c, S1));
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A1][0] = sx(((r(c, A1) as i32) >> 12) as u32);
                    c.g[A1][0] &= 1;
                    call_ptr(c, 0x0011_6120, 0x0012_f770)?;
                    pc = 0x0012_f770;
                }
                0x0012_f770 => {
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        pc = EPILOGUE;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S0, 8));
                        let tweaking = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                        if tweaking {
                            c.cycles += 12;
                            c.g[V0][0] = c.lw(r(c, S1));
                            c.g[V0][0] &= 0xc000;
                            c.g[V0][0] = u64::from(c.g[V0][0] != 0);
                            c.sw(at(c, S0, 8), c.g[V0][0]);
                            c.g[A1][0] = c.lw(r(c, S1));
                            c.g[A2][0] = sx(((r(c, A1) as i32) >> 15) as u32) & 1;
                            c.g[A1][0] = sx(((r(c, A1) as i32) >> 14) as u32) & 1;
                            call_ptr(c, 0x0011_63b0, 0x0012_f7ac)?;
                            pc = 0x0012_f7ac;
                        } else {
                            c.cycles += 4;
                            c.g[A1][0] = 0;
                            c.g[A2][0] = 0;
                            call_ptr(c, 0x0011_63b0, 0x0012_f7c8)?;
                            pc = 0x0012_f7c8;
                        }
                    }
                }
                0x0012_f7ac => {
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        pc = EPILOGUE;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(r(c, S1));
                        pc = 0x0012_f7cc;
                    }
                }
                0x0012_f7c8 => {
                    c.g[A0][0] = c.lw(r(c, S1));
                    pc = 0x0012_f7cc;
                }
                0x0012_f7cc => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw(at(c, S1, 4));
                    c.g[V1][0] = sx(r(c, A0) << 2);
                    c.lwc1(2, lit(c, 0x7704));
                    c.g[V0][0] = sx(r(c, V0) << 26);
                    c.g[V1][0] = sx(((r(c, V1) as i32) >> 26) as u32);
                    c.g[V0][0] = sx(((r(c, V0) as i32) >> 26) as u32);
                    c.mtc1(V1, 0);
                    c.cvt_s_w(0, 0);
                    c.mtc1(V0, 1);
                    c.cvt_s_w(1, 1);
                    c.g[A0][0] &= 0x2000;
                    c.mul_s(22, 0, 2);
                    let handplant = c.g[A0][0] != 0;
                    c.mul_s(21, 1, 2);
                    if handplant {
                        c.cycles += 6;
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 0);
                        c.c_lt_s(0, 21);
                        c.g[A1][0] = 1;
                        if !c.fcc() { c.cycles += 1; c.g[A1][0] = 0; }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                        call_ptr(c, 0x0010_7578, 0x0012_f828)?;
                        pc = 0x0012_f828;
                    } else { pc = 0x0012_f838; }
                }
                0x0012_f828 => {
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        pc = EPILOGUE;
                    } else {
                        c.cycles += 2;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 21);
                        pc = 0x0012_f838;
                    }
                }
                0x0012_f838 => {
                    // The native attach attempt has no held-button gate.
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    call_ptr(c, 0x0010_6848, 0x0012_f840)?;
                    pc = 0x0012_f840;
                }
                0x0012_f840 => {
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        pc = EPILOGUE;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x0011_4130, 0x0012_f858)?;
                        pc = 0x0012_f858;
                    }
                }
                0x0012_f858 => {
                    c.g[V0][0] = c.lw(at(c, S0, 0xc));
                    let latched = c.g[V0][0] != 0;
                    c.g[V0][0] = u64::MAX;
                    let mut load_direction_charge = 1;
                    if latched {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, S0, 0x10));
                        if c.g[V1][0] != c.g[V0][0] { load_direction_charge = 0; }
                        else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lb(at(c, S1, 2));
                            c.sw(at(c, S0, 0x10), c.g[V0][0]);
                        }
                    }
                    c.cycles += load_direction_charge;
                    c.g[A0][0] = c.lb(at(c, S1, 2));
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S0, 0x10));
                    let mut load_latch_charge = 1;
                    if c.g[A0][0] != c.g[V0][0] { c.sw(at(c, S0, 0xc), 0); }
                    else {
                        c.cycles += 3;
                        c.g[V0][0] = u64::MAX;
                        if c.g[A0][0] != c.g[V0][0] { load_latch_charge = 0; }
                        else { c.cycles += 1; c.sw(at(c, S0, 0xc), 0); }
                    }
                    c.cycles += load_latch_charge;
                    c.g[V0][0] = c.lw(at(c, S0, 0xc));
                    c.cycles += 2;
                    let latched = c.g[V0][0] != 0;
                    c.g[V0][0] = u64::MAX;
                    if !latched {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, S0, 0x10));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.cycles += 3;
                            c.g[AT][0] = sx(0xbf80_0000);
                            c.mtc1(AT, 0);
                            c.swc1(0, at(c, S0, 4));
                        }
                    }
                    c.cycles += 5;
                    c.abs_s(0, 22);
                    c.c_le_s(21, 0);
                    c.mov_s(23, 21);
                    if c.fcc() { c.cycles += 1; c.mov_s(23, 0); }
                    c.cycles += 4;
                    c.mtc1(ZERO, 20);
                    c.c_eq_s(22, 20);
                    if !c.fcc() { pc = 0x0012_f910; }
                    else {
                        c.cycles += 4;
                        c.c_eq_s(21, 20);
                        let zero = c.fcc();
                        c.mov_s(2, 21);
                        if zero { pc = 0x0012_f950; }
                        else {
                            c.cycles += 4;
                            c.c_le_s(20, 21);
                            c.lwc1(2, lit(c, 0x7700));
                            let positive = c.fcc();
                            c.lwc1(0, at(c, S0, 4));
                            if positive { c.cycles += 3; c.lwc1(2, lit(c, 0x76fc)); }
                            pc = 0x0012_f954;
                        }
                    }
                }
                0x0012_f910 => {
                    c.cycles += 7;
                    c.div_s(12, 21, 22);
                    call_ptr(c, 0x0031_c228, 0x0012_f91c)?;
                    pc = 0x0012_f91c;
                }
                0x0012_f91c => {
                    c.c_lt_s(22, 20);
                    let negative = c.fcc();
                    c.mov_s(1, 0);
                    if negative {
                        c.cycles += 3;
                        c.c_lt_s(20, 21);
                        if c.fcc() {
                            c.cycles += 3;
                            c.lwc1(0, lit(c, 0x76f8));
                            c.add_s(1, 1, 0);
                        } else {
                            c.cycles += 2;
                            c.lwc1(0, lit(c, 0x76f4));
                            c.sub_s(1, 1, 0);
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(2, 1);
                    pc = 0x0012_f950;
                }
                0x0012_f950 => {
                    c.cycles += 1;
                    c.lwc1(0, at(c, S0, 4));
                    pc = 0x0012_f954;
                }
                0x0012_f954 => {
                    c.cycles += 4;
                    c.mtc1(ZERO, 1);
                    c.c_lt_s(0, 1);
                    let mut discard = c.fcc();
                    if !discard {
                        c.cycles += 7;
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 4);
                        c.sub_s(0, 0, 4);
                        c.max_s(0, 0, 1);
                        c.c_le_s(23, 0);
                        discard = c.fcc();
                        if !discard {
                            c.cycles += 10;
                            c.lwc1(1, r(c, S0));
                            c.lwc1(0, lit(c, 0x76f0));
                            c.sub_s(3, 2, 1);
                            c.mul_s(0, 3, 0);
                            c.add_s(0, 0, 4);
                            c.cvt_w_s(2, 0);
                            c.cvt_s_w(2, 2);
                            c.c_lt_s(0, 2);
                            if c.fcc() {
                                c.cycles += 3;
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(AT, 0);
                                c.sub_s(2, 2, 0);
                            }
                            c.cycles += 9;
                            c.lwc1(0, lit(c, 0x76ec));
                            c.lwc1(1, lit(c, 0x76e8));
                            c.mul_s(0, 2, 0);
                            c.sub_s(0, 3, 0);
                            c.abs_s(0, 0);
                            c.c_lt_s(1, 0);
                            discard = c.fcc();
                        }
                    }
                    if discard {
                        c.cycles += 6;
                        c.g[AT][0] = sx(0xbf80_0000);
                        c.mtc1(AT, 0);
                        c.mtc1(ZERO, 21);
                        c.swc1(0, at(c, S0, 4));
                        c.mov_s(22, 21);
                    }
                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                    c.cycles += 24;
                    c.g[A1][0] = 2;
                    c.lwc1(1, lit(c, 0x76e4));
                    c.swc1(22, at(c, V1, 0x1f8));
                    c.swc1(1, at(c, V1, 0x1f4));
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    c.lwc1(0, at(c, V0, 0x1f0));
                    c.swc1(1, at(c, V0, 0x200));
                    c.swc1(0, at(c, V0, 0x204));
                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                    c.swc1(1, at(c, V1, 0x224));
                    c.swc1(21, at(c, V1, 0x228));
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    c.swc1(1, at(c, A0, 0x218));
                    c.sw(at(c, A0, 0x21c), 0);
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                    call_ptr(c, 0x0031_2aa0, 0x0012_fa34)?;
                    pc = 0x0012_fa34;
                }
                0x0012_fa34 => {
                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                    c.mtc1(ZERO, 1);
                    c.lwc1(0, at(c, V1, 0x220));
                    c.c_eq_s(0, 1);
                    let mut has_input = !c.fcc();
                    c.g[S1][0] = c.g[V0][0];
                    let mut tail_charge = 1;
                    if !has_input {
                        c.cycles += 5;
                        c.lwc1(0, at(c, V1, 0x214));
                        c.c_eq_s(0, 1);
                        has_input = !c.fcc();
                        if has_input { tail_charge = 0; }
                        else {
                            c.cycles += 5;
                            c.lwc1(0, at(c, V1, 0x1f0));
                            c.c_eq_s(0, 1);
                            has_input = !c.fcc();
                            if has_input { tail_charge = 0; }
                            else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, S0, 0xc));
                                has_input = c.g[V0][0] != 0;
                                if has_input { tail_charge = 0; }
                            }
                        }
                    }
                    if has_input {
                        c.cycles += tail_charge;
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        pc = 0x0012_fad8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x11f;
                        if c.g[S1][0] == c.g[V0][0] {
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            pc = 0x0012_fac4;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = c.lw(at(c, V1, ANIMATION));
                            c.g[A1][0] = 2;
                            call_ptr(c, 0x0031_1ae8, 0x0012_fa9c)?;
                            pc = 0x0012_fa9c;
                        }
                    }
                }
                0x0012_fa9c => {
                    c.g[V1][0] = 9;
                    let special_task = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = 0x11f;
                    if special_task { pc = 0x0012_fac0; }
                    else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        c.g[AT][0] = sx(0xbf80_0000);
                        c.mtc1(AT, 12);
                        c.g[A2][0] = 0;
                        c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                        call_ptr(c, 0x0031_28e8, 0x0012_fac0)?;
                        pc = 0x0012_fac0;
                    }
                }
                0x0012_fac0 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    pc = 0x0012_fac4;
                }
                0x0012_fac4 => {
                    c.cycles += 4;
                    c.g[A1][0] = 5;
                    call_ptr(c, 0x0011_fec8, 0x0012_facc)?;
                    pc = 0x0012_facc;
                }
                0x0012_facc => {
                    frame.restore(c, S0, 0x20);
                    pc = EPILOGUE;
                }
                0x0012_fad8 => {
                    c.cycles += 6;
                    c.g[A1][0] = 2;
                    c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                    call_ptr(c, 0x0031_1ae8, 0x0012_fae4)?;
                    pc = 0x0012_fae4;
                }
                0x0012_fae4 => {
                    c.g[V1][0] = 9;
                    let special_task = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = 10;
                    if special_task { pc = 0x0012_fb44; }
                    else {
                        c.cycles += 14;
                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 1);
                        c.lwc1(0, at(c, A0, 0x1f0));
                        c.abs_s(0, 0);
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 3);
                        c.sub_s(1, 1, 0);
                        c.lwc1(2, at(c, A0, 0x220));
                        c.mul_s(1, 1, 3);
                        c.c_le_s(2, 1);
                        if c.fcc() { c.g[A1][0] = 9; }
                        c.cycles += 2;
                        let same_animation = c.g[A1][0] == c.g[S1][0];
                        frame.restore(c, S0, 0x20);
                        if same_animation { pc = EPILOGUE; }
                        else {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw(at(c, A0, ANIMATION));
                            c.g[AT][0] = sx(0xbf80_0000);
                            c.mtc1(AT, 12);
                            c.g[A2][0] = 0;
                            call_ptr(c, 0x0031_28e8, 0x0012_fb44)?;
                            pc = 0x0012_fb44;
                        }
                    }
                }
                0x0012_fb44 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    pc = EPILOGUE;
                }
                EPILOGUE => {
                    c.cycles += 8;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    for (register, offset) in [(23, 0x48), (22, 0x40), (21, 0x38), (20, 0x30)] {
                        frame.restore_fpr(c, register, offset);
                    }
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
