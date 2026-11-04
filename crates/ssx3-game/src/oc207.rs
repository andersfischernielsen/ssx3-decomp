//! Staging wave 201 (oc207): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_1b08, pack_d, "__pack_d"),
    (0x0032_6eb0, ps2_pad_update, "PS2Pad_Update"),
];

/// `double __pack_d(const fp_number_type* src)` (0x411b08, libgcc
/// soft-float, the inverse of `__unpack_d`): builds the IEEE double bits (in
/// `v0`) from the unpacked class, sign, unbiased exponent and fraction (the
/// mantissa `<< 8`, implicit one at bit 60). NaNs get exponent 0x7ff and the
/// quiet bit; infinity and an out-of-range exponent (>= 1024) give 0x7ff
/// with no fraction; zero (or a zero fraction) gives exponent 0. Below
/// -1022 the fraction is shifted down to a denormal (nothing left past 56
/// places) without rounding; normals round to nearest even on the 8 guard
/// bits and bump the exponent when the rounding carries past bit 60.
///
/// Leaves `a1` (final fraction before masking), `a3` (biased exponent),
/// `t0` (sign word), `a0` (`u64::MAX >> 1`), `a2` (result without sign) and
/// `v1` (exponent field) like the original.
pub fn pack_d(c: &mut Cpu) {
    use crate::game::lib::gcc::fp_number as fp;
    use crate::guest::Addr;

    const MASK52: u64 = u64::MAX >> 12;
    const QUIET_NAN: u64 = 1 << 51;
    const GUARD_MASK: u64 = 0xff;
    const GUARD_HALF: u64 = 0x80;
    const OVERFLOW_LIMIT: u64 = u64::MAX >> 3; // implicit one carried to bit 61
    const EXP_MAX: u64 = 0x7ff;
    const EXP_MIN: i64 = -1022;

    let src = Addr(c.g[A0][0] as u32);
    c.cycles += 6;
    let class = src.off(fp::CLASS).lw(c);
    let sign = src.off(fp::SIGN).lw(c);
    let mut fraction = src.off(fp::FRACTION).ld(c);
    let mut exp_field: u64 = 0;

    if class < fp::CLASS_ZERO {
        // signalling or quiet NaN
        c.cycles += 5;
        exp_field = EXP_MAX;
        fraction |= QUIET_NAN;
    } else if class == fp::CLASS_INFINITY {
        c.cycles += 3 + 2;
        exp_field = EXP_MAX;
        fraction = 0;
    } else if class == fp::CLASS_ZERO {
        c.cycles += 3 + 3 + 2;
        fraction = 0;
    } else if fraction == 0 {
        c.cycles += 3 + 3 + 2;
    } else {
        c.cycles += 3 + 3 + 2 + 4;
        let exp = src.off(fp::NORMAL_EXP).lw(c);
        if (exp as i64) < EXP_MIN {
            // denormal: shift down, no rounding
            c.cycles += 4;
            let shift = sx((EXP_MIN as u32).wrapping_sub(exp as u32));
            fraction >>= shift & 63;
            if (shift as i64) >= 57 {
                c.cycles += 2;
                fraction = 0;
            }
            c.cycles += 1;
            fraction >>= 8;
        } else {
            c.cycles += 3;
            exp_field = sx((exp as u32).wrapping_add(0x3ff));
            if (exp as i64) >= 1024 {
                // too large: infinity
                c.cycles += 1 + 2;
                exp_field = EXP_MAX;
                fraction = 0;
            } else {
                // round to nearest, ties to even
                c.cycles += 4;
                if fraction & GUARD_MASK != GUARD_HALF {
                    fraction = fraction.wrapping_add(0x7f);
                } else {
                    c.cycles += 3;
                    if fraction & 0x100 != 0 {
                        fraction = fraction.wrapping_add(0x80);
                    }
                }
                c.cycles += 5;
                if fraction > OVERFLOW_LIMIT {
                    c.cycles += 2 + 1;
                    fraction >>= 1;
                    exp_field = sx((exp_field as u32).wrapping_add(1));
                }
                fraction >>= 8;
            }
        }
    }

    c.cycles += 23;
    let exp_bits = (exp_field & EXP_MAX) << 52;
    let magnitude = (fraction & MASK52) | exp_bits;
    c.g[A1][0] = fraction;
    c.g[A3][0] = exp_field;
    c.g[T0][0] = sign;
    c.g[A0][0] = u64::MAX >> 1;
    c.g[A2][0] = magnitude;
    c.g[V1][0] = exp_bits;
    c.g[V0][0] = magnitude | (sign << 63);
    ret(c);
}

// ---------------------------------------------------------- PS2Pad_Update

/// `PS2Pad` (the per-controller libpad state; field names after the
/// reference's `inputparse.cpp`).
mod ps2pad {
    pub const PORT: u32 = 0x04;
    pub const SLOT: u32 = 0x08;
    /// Requested small / big motor values and the last ones sent.
    pub const ACT_SMALL: u32 = 0x30;
    pub const LAST_SMALL: u32 = 0x34;
    pub const ACT_BIG: u32 = 0x38;
    pub const LAST_BIG: u32 = 0x3c;
    /// The 6-byte actuator block handed to `scePadSetActDirect`.
    pub const ACT_DIRECT: u32 = 0x40;
    /// Connection state machine phase (0 = port closed, 10 = reading).
    pub const PHASE: u32 = 0x48;
    pub const RETRY: u32 = 0x4c;
    pub const MODE_SET: u32 = 0x50;
    /// Pressure-sensitive buttons enabled (else digital ones).
    pub const PRESSURE: u32 = 0x54;
    pub const ANALOG: u32 = 0x58;
    pub const ACTUATOR: u32 = 0x5c;
    /// The 64-byte aligned DMA buffer given to `scePadPortOpen`.
    pub const DMA_BUF: u32 = 0x80;

    pub const PORT_OPEN: u32 = 0x003f_f708;
    pub const GET_STATE: u32 = 0x003f_fbc0;
    pub const GET_REQ_STATE: u32 = 0x003f_fcd8;
    pub const INFO_ACT: u32 = 0x003f_fd60;
    pub const INFO_MODE: u32 = 0x003f_ffa0;
    pub const SET_MAIN_MODE: u32 = 0x0040_00d8;
    pub const SET_ACT_DIRECT: u32 = 0x0040_0190;
    pub const SET_ACT_ALIGN: u32 = 0x0040_0250;
    pub const INFO_PRESS_MODE: u32 = 0x0040_0490;
    pub const ENTER_PRESS_MODE: u32 = 0x0040_04f0;
    pub const READ: u32 = 0x003f_fa58;

    /// The `switch (phase)` jump table in `.rodata` and its case labels
    /// (phases 1..=10).
    pub const JUMP_TABLE: u32 = 0x0048_e460;
    pub const CASES: [u32; 10] = [
        0x0032_6f70, 0x0032_7028, 0x0032_707c, 0x0032_709c, 0x0032_70c8,
        0x0032_70e8, 0x0032_712c, 0x0032_715c, 0x0032_7188, 0x0032_71d8,
    ];
    /// `D_004A3FB8`, the actuator alignment (gp-relative).
    pub const ACT_ALIGN_GP: u32 = 0xec8;
    /// `1/255` scale for pressure bytes and for the stick ramps (gp-relative).
    pub const PRESSURE_SCALE_GP: u32 = 0x2e0c;
    pub const STICK_SCALE_GP: u32 = 0x2e08;
    /// Number of output floats written when the read succeeded.
    pub const OUT_COUNT: u64 = 0x18;
}

/// 1.0f
const PAD_ONE: u32 = 0x3f80_0000;

/// One active-low digital button as a float: `f0` = 1.0 when its bit is
/// clear (pressed), else 0.0; stored at `dst`. The caller charges the test.
fn pad_button(c: &mut Cpu, bits: u64, mask: u64, dst: u32) {
    c.f[0] = 0;
    if bits & mask == 0 {
        c.cycles += 3;
        c.g[AT][0] = PAD_ONE as u64;
        c.f[0] = PAD_ONE;
    }
    c.swc1(0, dst);
}

/// `mtc1 zero, f0` then 1.0 into `f0` (2 cycles) when the button is
/// pressed; no store (the next block stores it).
fn pad_button_late(c: &mut Cpu, pressed: bool) {
    c.f[0] = 0;
    if pressed {
        c.cycles += 2;
        c.g[AT][0] = PAD_ONE as u64;
        c.f[0] = PAD_ONE;
    }
}

/// `div n, 0x4f` (pipeline 0): sets LO/HI, returns the quotient.
fn pad_div79(c: &mut Cpu, n: u64) -> u64 {
    let n = n as i32;
    c.lo[0] = sx((n / 0x4f) as u32);
    c.hi[0] = sx((n % 0x4f) as u32);
    c.lo[0]
}

/// One stick axis byte: the two ramp numerators `(0x4f - b) * 255` and
/// `(b - 0xb0) * 255` (computed `x << 8 - x`) and their `/ 0x4f`
/// quotients clamped at 0 (the clamp flag is `-1 < q`). Returns
/// `(neg_quotient, neg_flag, pos_quotient, pos_flag, pos_delta)`.
fn pad_stick(c: &mut Cpu, b: u64) -> (u64, u64, u64, u64, u64) {
    let neg = sx(0x4fu32.wrapping_sub(b as u32));
    let pos = sx((b as u32).wrapping_sub(0xb0));
    let neg_num = sx(((neg as u32) << 8).wrapping_sub(neg as u32));
    let pos_num = sx(((pos as u32) << 8).wrapping_sub(pos as u32));
    let qn = pad_div79(c, neg_num);
    let qp = pad_div79(c, pos_num);
    let fneg = ((qn as i64) > -1) as u64;
    let fpos = ((qp as i64) > -1) as u64;
    (
        if fneg == 0 { 0 } else { qn },
        fneg,
        if fpos == 0 { 0 } else { qp },
        fpos,
        pos,
    )
}

/// `int PS2Pad_Update(PS2Pad* pad, float* out)` (0x326eb0): one frame of
/// a controller. Opens the port if needed, then steps the libpad
/// connection state machine (`pad->phase`): probe the controller id and
/// mode (DualShock 2 / analog), lock analog mode, enter pressure mode,
/// query and align the vibration motors, and finally read (phase 10). On
/// a good read it fills 24 floats in `out`: d-pad, the 12 face/shoulder
/// buttons (pressure bytes / 255 when pressure mode is on, else 0/1 from
/// the active-low bits) and the 8 stick half-axes (ramp outside the dead
/// zone 0x4f..0xb0, scaled by 1/255), and returns 0x18; else returns 0.
/// While reading, sends changed motor values with `scePadSetActDirect`.
pub fn ps2_pad_update(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use ps2pad::*;

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let pad = Addr(c.g[S0][0] as u32);
        let out = c.g[S1][0] as u32;
        let gp = c.g[GP][0] as u32;

        // Phase 0: open the port; success resets the state.
        let phase = pad.off(PHASE).lw(c);
        c.g[V0][0] = phase;
        c.g[S3][0] = 0;
        if phase != 0 {
            c.cycles += 1;
        } else {
            c.cycles += 8;
            c.g[A0][0] = pad.off(PORT).lw(c);
            c.g[A2][0] = sx(pad.0.wrapping_add(DMA_BUF));
            c.g[A1][0] = pad.off(SLOT).lw(c);
            call_ptr(c, PORT_OPEN, 0x0032_6eec)?;
            let opened = c.g[V0][0];
            c.g[V1][0] = opened;
            c.g[V0][0] = 1;
            if opened == 1 {
                c.cycles += 7 + 1;
                c.g[V0][0] = u64::MAX;
                pad.off(PHASE).sw(c, opened);
                pad.off(LAST_BIG).sw(c, u64::MAX);
                pad.off(MODE_SET).sw(c, 0);
                pad.off(ACT_SMALL).sw(c, 0);
                pad.off(LAST_SMALL).sw(c, u64::MAX);
                pad.off(ACT_BIG).sw(c, 0);
            }
        }
        c.g[A1][0] = pad.off(SLOT).lw(c);

        // Disconnected (0) or error (7): start over at probing.
        c.cycles += 5;
        c.g[A0][0] = pad.off(PORT).lw(c);
        call_ptr(c, GET_STATE, 0x0032_6f24)?;
        let state = c.g[V0][0];
        c.g[A1][0] = state;
        c.g[V0][0] = 7;
        if state != 0 {
            c.cycles += 2;
        }
        if state == 0 || state == 7 {
            c.cycles += 4;
            c.g[V0][0] = 1;
            pad.off(MODE_SET).sw(c, 0);
            pad.off(PHASE).sw(c, 1);
        }
        let phase = pad.off(PHASE).lw(c);
        c.g[V0][0] = phase;

        c.cycles += 4;
        let index = sx((phase as u32).wrapping_sub(1));
        c.g[A0][0] = index;
        c.g[V1][0] = (index < 10) as u64;
        c.g[V0][0] = 0x0049_0000;
        let mut read_ok = false;
        if index >= 10 {
            c.cycles += 1;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(JUMP_TABLE.wrapping_add((index as u32) << 2));
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[A0][0] = CASES[index as usize] as u64;
            let port = |c: &mut Cpu| pad.off(PORT).lw(c);
            let slot = |c: &mut Cpu| pad.off(SLOT).lw(c);
            match index + 1 {
                // Stable (6) or busy (2): probe the controller id and mode.
                1 => {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    let mut probe = state == 6;
                    if !probe {
                        // `bne a1, 2` with `move s2, zero` in its delay
                        // slot: s2 is 0 at the InfoMode call (callees
                        // save it on their frames)
                        c.cycles += 2;
                        c.g[S2][0] = 0;
                        probe = state == 2;
                    }
                    if probe {
                        c.cycles += 11;
                        pad.off(PRESSURE).sw(c, 0);
                        c.g[A2][0] = 1;
                        pad.off(ANALOG).sw(c, 0);
                        c.g[A3][0] = 0;
                        pad.off(ACTUATOR).sw(c, 0);
                        c.g[A0][0] = port(c);
                        c.g[A1][0] = slot(c);
                        call_ptr(c, INFO_MODE, 0x0032_6fa4)?;
                        let id = c.g[V0][0];
                        c.g[S2][0] = id;
                        c.g[A2][0] = 2;
                        if id == 0 {
                            c.cycles += 1;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = port(c);
                            c.g[A1][0] = slot(c);
                            c.g[A3][0] = 0;
                            call_ptr(c, INFO_MODE, 0x0032_6fc0)?;
                            let ext = c.g[V0][0];
                            c.g[V1][0] = ext;
                            c.g[A0][0] = 4;
                            c.g[V0][0] = ((ext as i64) < 5) as u64;
                            if ext == 4 {
                                // digital DualShock: set the analog mode
                                c.cycles += 3 + 1;
                                c.g[V0][0] = 2;
                                pad.off(PHASE).sw(c, 2);
                            } else if (ext as i64) >= 5 {
                                c.cycles += 2 + 2;
                                c.g[V0][0] = 7;
                                if ext == 7 {
                                    // analog DualShock
                                    c.cycles += 5 + 1;
                                    c.g[V0][0] = 1;
                                    c.g[V1][0] = 2;
                                    pad.off(ANALOG).sw(c, 1);
                                    pad.off(PHASE).sw(c, 2);
                                } else {
                                    c.cycles += 2;
                                }
                            } else {
                                c.cycles += 2 + 2;
                                if ext != 0 {
                                    c.cycles += 2;
                                } else {
                                    c.cycles += 2;
                                    if id == 4 {
                                        // plain digital pad: read directly
                                        c.cycles += 2 + 2 + 1;
                                        c.g[V0][0] = 0xa;
                                        pad.off(PHASE).sw(c, 0xa);
                                    }
                                }
                            }
                        }
                    }
                }
                // Lock analog mode (unless already set for an analog pad).
                2 => {
                    c.cycles += 3;
                    let analog = pad.off(ANALOG).lw(c);
                    c.g[V0][0] = analog;
                    let mut set_mode = true;
                    if analog != 0 {
                        c.cycles += 3;
                        let mode_set = pad.off(MODE_SET).lw(c);
                        c.g[V0][0] = 4;
                        if mode_set != 0 {
                            set_mode = false;
                            c.cycles += 2 + 1;
                            pad.off(PHASE).sw(c, 4);
                        } else {
                            c.cycles += 1;
                        }
                    }
                    if set_mode {
                        c.g[A0][0] = port(c);
                        c.cycles += 8;
                        c.g[A2][0] = 1;
                        c.g[A1][0] = slot(c);
                        c.g[A3][0] = 3;
                        call_ptr(c, SET_MAIN_MODE, 0x0032_7054)?;
                        let r = c.g[V0][0];
                        c.g[V1][0] = r;
                        c.g[V0][0] = 1;
                        if r == 1 {
                            c.cycles += 4;
                            c.g[V0][0] = 3;
                            pad.off(MODE_SET).sw(c, r);
                            pad.off(PHASE).sw(c, 3);
                        }
                    }
                }
                // Wait for the mode change to finish.
                3 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, GET_STATE, 0x0032_7088)?;
                    c.g[V1][0] = 5;
                    let done = c.g[V0][0] == 5;
                    c.g[V0][0] = 1;
                    if !done {
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 1);
                    }
                    c.cycles += 1;
                }
                // Pressure mode available?
                4 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, INFO_PRESS_MODE, 0x0032_70a8)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] != 1 {
                        c.g[V0][0] = 7;
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 7);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 5;
                        pad.off(PHASE).sw(c, 5);
                    }
                    c.cycles += 1;
                }
                // Enter pressure mode.
                5 => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, ENTER_PRESS_MODE, 0x0032_70d4)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == 1 {
                        c.cycles += 2 + 3 + 1;
                        c.g[V0][0] = 6;
                        pad.off(RETRY).sw(c, 0);
                        pad.off(PHASE).sw(c, 6);
                    }
                }
                // Wait for the request; 5 failures: pressure mode on.
                6 | 9 => {
                    let ret = if index + 1 == 6 { 0x0032_70f4 } else { 0x0032_7194 };
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A1][0] = slot(c);
                    call_ptr(c, GET_REQ_STATE, ret)?;
                    c.g[A0][0] = 1;
                    let req = c.g[V0][0];
                    if req == 1 {
                        if index + 1 == 9 {
                            c.cycles += 1;
                        }
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 1);
                        pad.off(MODE_SET).sw(c, 0);
                    } else {
                        c.cycles += 2;
                        if req != 0 {
                            pad.off(RETRY).sw(c, 0);
                        } else {
                            c.cycles += 5;
                            let tries = sx((pad.off(RETRY).lw(c) as u32).wrapping_add(1));
                            c.g[V0][0] = tries;
                            c.g[V1][0] = ((tries as i64) < 5) as u64;
                            pad.off(RETRY).sw(c, tries);
                            if (tries as i64) >= 5 {
                                c.cycles += 4;
                                if index + 1 == 6 {
                                    c.g[V0][0] = 7;
                                    pad.off(PRESSURE).sw(c, 1);
                                    pad.off(PHASE).sw(c, 7);
                                } else {
                                    c.g[V0][0] = 0xa;
                                    pad.off(ACTUATOR).sw(c, 1);
                                    pad.off(PHASE).sw(c, 0xa);
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                }
                // Motors present? Align them, else go reading.
                7 => {
                    c.cycles += 7;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = u64::MAX;
                    c.g[A1][0] = slot(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, INFO_ACT, 0x0032_7140)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1 + 2;
                        c.g[V0][0] = 0xa;
                        pad.off(PHASE).sw(c, 0xa);
                    } else {
                        c.g[V0][0] = 8;
                        c.cycles += 2;
                        pad.off(PHASE).sw(c, 8);
                    }
                    c.cycles += 1;
                }
                8 => {
                    c.cycles += 7;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = sx(gp.wrapping_add(ACT_ALIGN_GP));
                    c.g[A1][0] = slot(c);
                    call_ptr(c, SET_ACT_ALIGN, 0x0032_716c)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == 1 {
                        c.cycles += 1 + 3 + 1;
                        c.g[V0][0] = 9;
                        pad.off(RETRY).sw(c, 0);
                        pad.off(PHASE).sw(c, 9);
                    }
                }
                // Read; a bad read or a non-zero status byte reprobes.
                _ => {
                    c.cycles += 6;
                    c.g[A0][0] = port(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = slot(c);
                    call_ptr(c, READ, 0x0032_71e8)?;
                    let len = c.g[V0][0];
                    let status = Addr(sp(c)).lbu(c);
                    c.g[V0][0] = status;
                    if len != 0 {
                        c.cycles += 2;
                        read_ok = status == 0;
                    }
                    if read_ok {
                        c.g[S3][0] = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        pad.off(MODE_SET).sw(c, 0);
                        pad.off(PHASE).sw(c, 1);
                    }
                    c.cycles += 1;
                }
            }
        }
        c.g[S2][0] = 0;

        c.cycles += 2;
        let data = Addr(sp(c));
        c.g[V0][0] = data.off(2).lbu(c);
        let mut result = 0;
        if read_ok {
            result = OUT_COUNT;
            c.g[S2][0] = OUT_COUNT;
            let b2 = data.off(2).lbu(c);
            // d-pad
            c.cycles += 5;
            pad_button_late(c, b2 & 0x01 == 0);
            c.cycles += 6;
            c.g[V1][0] = b2;
            c.swc1(0, out);
            pad_button(c, b2, 0x08, out + 0x04);
            c.cycles += 4;
            pad_button(c, b2, 0x02, out + 0x08);
            c.cycles += 4;
            pad_button(c, b2, 0x04, out + 0x0c);

            c.cycles += 3;
            if pad.off(PRESSURE).lw(c) != 0 {
                // 12 pressure bytes * (1/255)
                c.cycles += 55 + 6;
                let scale = Addr(gp.wrapping_sub(PRESSURE_SCALE_GP)).read_fpu(c);
                // destination FPRs of bytes 8..=0x13, in order
                const FREGS: [usize; 12] = [2, 3, 7, 8, 9, 10, 11, 12, 6, 4, 5, 1];
                for (i, &f) in FREGS.iter().enumerate() {
                    let b = data.off(8 + i as u32).lbu(c);
                    let v = FpuF32::from_i32(b as i32) * scale;
                    c.f[f] = v.0;
                    Addr(out + 0x10 + 4 * i as u32).write_fpu(c, v);
                }
                c.f[0] = scale.0;
                c.g[V0][0] = data.off(0x12).lbu(c);
                c.g[V1][0] = data.off(0x13).lbu(c);
                c.g[A0][0] = data.off(0x0e).lbu(c);
                c.g[A1][0] = data.off(0x0f).lbu(c);
                c.g[A2][0] = data.off(0x10).lbu(c);
                c.g[A3][0] = data.off(0x11).lbu(c);
            } else {
                // digital face and shoulder buttons
                c.cycles += 4;
                pad_button(c, b2, 0x20, out + 0x10);
                c.cycles += 4;
                pad_button(c, b2, 0x80, out + 0x14);
                c.cycles += 4;
                pad_button(c, b2, 0x10, out + 0x18);
                c.cycles += 4;
                let b3 = data.off(3).lbu(c);
                pad_button_late(c, b2 & 0x40 == 0);
                c.cycles += 5;
                c.g[V1][0] = b3;
                c.swc1(0, out + 0x1c);
                pad_button(c, b3, 0x10, out + 0x20);
                c.cycles += 4;
                pad_button(c, b3, 0x20, out + 0x24);
                c.cycles += 4;
                pad_button(c, b3, 0x40, out + 0x28);
                c.cycles += 4;
                pad_button(c, b3, 0x80, out + 0x2c);
                c.cycles += 4;
                pad_button(c, b3, 0x04, out + 0x30);
                c.cycles += 4;
                pad_button_late(c, b3 & 0x08 == 0);
                c.cycles += 6;
                c.swc1(0, out + 0x34);
                pad_button(c, b3, 0x01, out + 0x38);
                c.cycles += 4;
                pad_button(c, b3, 0x02, out + 0x3c);
                c.g[V0][0] = b3 & 0x02;
            }

            // stick half-axes
            c.cycles += 3;
            let analog = pad.off(ANALOG).lw(c);
            c.g[V0][0] = analog;
            c.g[V1][0] = data.off(4).lbu(c);
            if analog == 0 {
                c.cycles += 8;
                for i in 0..8 {
                    Addr(out + 0x40 + 4 * i).sw(c, 0);
                }
            } else {
                c.cycles += 6 + 103;
                let scale = Addr(gp.wrapping_sub(STICK_SCALE_GP)).read_fpu(c);
                c.f[2] = scale.0;
                // bytes 4..=7: left x, left y, right x, right y
                let axis = |c: &mut Cpu, i: u32| {
                    let b = data.off(4 + i).lbu(c);
                    let ramps = pad_stick(c, b);
                    let neg = FpuF32::from_i32(ramps.0 as i32) * scale;
                    let pos = FpuF32::from_i32(ramps.2 as i32) * scale;
                    Addr(out + 0x40 + 8 * i).write_fpu(c, neg);
                    Addr(out + 0x44 + 8 * i).write_fpu(c, pos);
                    c.f[0] = neg.0;
                    c.f[1] = pos.0;
                    ramps
                };
                axis(c, 0);
                axis(c, 1);
                axis(c, 2);
                let (qn, fneg, qp, fpos, pos) = axis(c, 3);
                c.g[V1][0] = qn;
                c.g[A0][0] = fneg;
                c.g[V0][0] = qp;
                c.g[A3][0] = fpos;
                c.g[A1][0] = pos;
                c.g[A2][0] = 0x4f;
                for r in [T0, T1, T2, T3, T4, T5] {
                    c.g[r][0] = 0x4f;
                }
            }
        }

        // While reading: send changed motor values.
        c.cycles += 4;
        c.g[V1][0] = pad.off(PHASE).lw(c);
        c.g[V0][0] = result;
        if c.g[V1][0] == 0xa {
            c.cycles += 4;
            let act_small = pad.off(ACT_SMALL).lw(c);
            c.g[T0][0] = act_small;
            c.g[V0][0] = pad.off(LAST_SMALL).lw(c);
            let mut changed = act_small != c.g[V0][0];
            if !changed {
                c.cycles += 4;
                c.g[V1][0] = pad.off(ACT_BIG).lw(c);
                let last_big = pad.off(LAST_BIG).lw(c);
                c.g[V0][0] = result;
                if c.g[V1][0] != last_big {
                    c.cycles += 1;
                    changed = true;
                }
            }
            if changed {
                c.g[V0][0] = pad.off(ACT_SMALL).lbu(c);
                c.cycles += 11;
                c.g[A2][0] = sx(pad.0.wrapping_add(ACT_DIRECT));
                c.g[V1][0] = pad.off(ACT_BIG).lbu(c);
                c.g[A3][0] = pad.off(ACT_BIG).lw(c);
                c.g[A0][0] = pad.off(PORT).lw(c);
                c.g[A1][0] = pad.off(SLOT).lw(c);
                let (small, big) = (c.g[V0][0], c.g[V1][0]);
                pad.off(ACT_DIRECT).sb(c, small);
                pad.off(ACT_DIRECT + 1).sb(c, big);
                pad.off(LAST_SMALL).sw(c, act_small);
                let a3 = c.g[A3][0];
                pad.off(LAST_BIG).sw(c, a3);
                call_ptr(c, SET_ACT_DIRECT, 0x0032_7718)?;
                c.g[V0][0] = result;
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------- __pack_d

    /// `__pack_d` against the generated body: every class (both NaNs, zero,
    /// normal, infinity, junk classes), zero and random fractions, exponents
    /// in range, at the edges (-1022, 1023, 1024), denormal (with shifts
    /// below and past 56 places) and huge; guard bytes forcing the
    /// round-to-even tie (both parities) and a carry past bit 60 that bumps
    /// the exponent.
    #[test]
    fn pack_d_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        use crate::game::lib::gcc::fp_number as fp;

        const SRC: u32 = DATA;
        check(ee::s15::f_00411b08__generated, pack_d, 300, &|c, seed| {
            let mut r = Rng(0x0411_b08d ^ seed as u64);
            scramble(c, &mut r);
            let rnd64 = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            let class: u32 = match seed % 10 {
                0 => 0,               // signalling NaN
                1 => 1,               // quiet NaN
                2 => 2,               // zero
                3 => 4,               // infinity
                4 => r.next_u32(),    // junk class (negative / large)
                _ => 3,               // normal number
            };
            let sign = if seed % 7 == 0 { r.next_u32() } else { r.next_u32() & 1 };
            let exp: i32 = match (seed / 10) % 9 {
                0 => -1022,
                1 => 1023,
                2 => 1024,
                3 => -1022 - 1 - (r.next_u32() % 56) as i32,  // shallow denormal
                4 => -1022 - 56 - (r.next_u32() % 8) as i32,  // around the 57 cut
                5 => r.next_u32() as i32,                     // anything
                _ => (r.next_u32() % 2046) as i32 - 1022,     // in range
            };
            // mantissa with the implicit one at bit 60 and 8 guard bits
            let mut frac = (rnd64(&mut r) & (u64::MAX >> 4)) | (1 << 60);
            match (seed / 3) % 8 {
                0 => frac = 0,
                1 => frac = (frac & !0xff) | 0x80,              // tie, odd/even random
                2 => frac = (frac & !0x1ff) | 0x80,             // tie, even
                3 => frac = (frac & !0x1ff) | 0x180,            // tie, odd
                4 => frac = ((u64::MAX >> 3) & !0x7f) | 0x81,  // rounds to bit 61
                5 => frac = rnd64(&mut r),                      // anything
                _ => {}
            }
            c.bus.write32(SRC + fp::CLASS, class);
            c.bus.write32(SRC + fp::SIGN, sign);
            c.bus.write32(SRC + fp::NORMAL_EXP, exp as u32);
            c.bus.write32(SRC + 0x0c, r.next_u32());
            c.bus.write32(SRC + fp::FRACTION, frac as u32);
            c.bus.write32(SRC + fp::FRACTION + 4, (frac >> 32) as u32);
            c.g[A0][0] = sx(SRC);
            c.pc = 0x0041_1b08;
        });
    }

    // --------------------------------------------------------- PS2Pad_Update

    /// `PS2Pad_Update` against the generated body, with the real libpad
    /// callees on their short paths: the pad's libpad port entry points at
    /// a test DMA frame (state, request state, mode bytes and pad data
    /// chosen per seed) or is closed (every call returns 0); the SIF RPC
    /// packet pool is empty so `scePadPortOpen`, `scePadSetMainMode` and
    /// `scePadSetActAlign` fail at once. Every phase 0..=10 plus junk
    /// phases, every state/req-state/id/ext answer, retry counts at the
    /// limit, pressure and digital buttons, analog sticks on both sides of
    /// the dead zone (and off), changed / unchanged motor values.
    #[test]
    fn ps2_pad_update_matches() {
        use super::ps2pad::*;
        use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};

        const OUT: u32 = DATA + 0x1000;
        /// The libpad frame pair (2 x 0x80) of the pad's port.
        const DMA: u32 = DATA + 0x2000;
        /// libpad's port table: entry `port * 0x70 + slot * 0x1c` holds the
        /// DMA buffer (+0) and the opened flag (+0x10).
        const LIBPAD_PORTS: u32 = 0x0052_6250;
        const SIF_RPC_PACKETS: u32 = 0x0052_d680;
        const LIBPAD_PRINTF: u32 = 0x0045_15cc;
        // DMA frame fields
        const F_FRAME: u32 = 0x58;
        const F_LENGTH: u32 = 0x60;
        const F_MODE_TABLE: u32 = 0x50;
        const F_MODE_CONFIG: u32 = 0x64;
        const F_MODE_CUR_ID: u32 = 0x65;
        const F_MODE_CUR_OFFS: u32 = 0x69;
        const F_STATE: u32 = 0x70;
        const F_REQ_STATE: u32 = 0x71;
        const F_OK: u32 = 0x72;

        check(ee::s10::f_00326eb0__generated, ps2_pad_update, 400, &|c, seed| {
            let mut r = Rng(0x0326_eb0d ^ (seed as u64) << 4);
            scramble(c, &mut r);
            let pick = |r: &mut Rng, xs: &[u32]| xs[(r.next_u32() as usize) % xs.len()];
            c.g[GP][0] = GAME_GP as u64;
            put_words(c, JUMP_TABLE, &CASES);
            let scale = if seed % 5 == 0 { r.f32(2.0) } else { 0x3b80_8081 };
            put_words(c, GAME_GP - PRESSURE_SCALE_GP, &[scale]);
            put_words(c, GAME_GP - STICK_SCALE_GP, &[scale ^ (seed & 1)]);
            put_words(c, LIBPAD_PRINTF, &[0]);
            put_words(c, SIF_RPC_PACKETS, &[0, 0, 0, 0]);

            // the pad (DMA buffer at +0x80 misaligned for some seeds)
            let pad = DATA + if seed % 7 == 3 { 0x10 } else { 0 };
            let port = r.next_u32() % 2;
            let slot = r.next_u32() % 2;
            fill(c, &mut r, pad, 0x30);
            c.bus.write32(pad + PORT, port);
            c.bus.write32(pad + SLOT, slot);
            let phase = match seed % 13 {
                11 => r.next_u32(),
                12 => 10,
                p => p,
            };
            c.bus.write32(pad + PHASE, phase);
            let retry = pick(&mut r, &[0, 3, 4, 4, 7]);
            c.bus.write32(pad + RETRY, retry);
            for f in [MODE_SET, PRESSURE, ANALOG, ACTUATOR] {
                let v = pick(&mut r, &[0, 1, 1]);
                c.bus.write32(pad + f, v);
            }
            let mut acts = [0u32; 4];
            for a in acts.iter_mut() {
                *a = pick(&mut r, &[0, 1, 0xff, 0xffff_ffff]);
            }

            // libpad port entry and its DMA frame (frame 0 is the newer)
            let entry = LIBPAD_PORTS + port * 0x70 + slot * 0x1c;
            fill(c, &mut r, entry, 7);
            c.bus.write32(entry, DMA);
            c.bus.write32(entry + 0xc, 0);
            c.bus.write32(entry + 0x10, (seed % 6 != 0) as u32);
            fill(c, &mut r, DMA, 0x40);
            c.bus.write32(DMA + F_FRAME, 1);
            c.bus.write32(DMA + 0x80 + F_FRAME, 0);
            let len = pick(&mut r, &[0, 0x12, 0x20, 0x20]);
            c.bus.write32(DMA + F_LENGTH, len);
            if r.next_u32() % 3 != 0 {
                c.bus.write8(DMA, 0);
            }
            for i in 4..8 {
                let any = r.next_u32() & 0xff;
                let b = pick(&mut r, &[0, 0x4e, 0x4f, 0x50, 0x80, 0xaf, 0xb0, 0xb1, 0xff, any]);
                c.bus.write8(DMA + i, b as u8);
            }
            let state = pick(&mut r, &[0, 1, 2, 5, 6, 6, 6, 7, 2]);
            c.bus.write8(DMA + F_STATE, state as u8);
            let req = pick(&mut r, &[0, 1, 2, 3]);
            c.bus.write8(DMA + F_REQ_STATE, req as u8);
            let ok = pick(&mut r, &[0, 1, 1, 1, 2]);
            c.bus.write8(DMA + F_OK, ok as u8);
            let any = r.next_u32() & 0xff;
            let config = pick(&mut r, &[0, 1, 2, any]);
            c.bus.write8(DMA + F_MODE_CONFIG, config as u8);
            let any = r.next_u32() & 0xff;
            let id = pick(&mut r, &[0x41, 0x73, 0x79, 0x23, 0xf3, any]);
            c.bus.write8(DMA + F_MODE_CUR_ID, id as u8);
            let offs = r.next_u32() % 4;
            c.bus.write8(DMA + F_MODE_CUR_OFFS, offs as u8);
            for i in 0..4 {
                let ext = pick(&mut r, &[0, 4, 7, 5, 1]);
                c.bus.write16(DMA + F_MODE_TABLE + 2 * i, ext as u16);
            }
            // keep scePadSetActDirect off its SIF DMA path (needs the
            // frame's ok byte 1): no motor change then
            if ok == 1 {
                acts[1] = acts[0];
                acts[3] = acts[2];
            }
            for (i, a) in acts.iter().enumerate() {
                c.bus.write32(pad + ACT_SMALL + 4 * i as u32, *a);
            }

            fill(c, &mut r, OUT, 0x18);
            c.g[A0][0] = sx(pad);
            c.g[A1][0] = sx(OUT);
            c.pc = 0x0032_6eb0;
        });
    }
}
