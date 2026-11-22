//! Native SSXAudioSystem_PlayObjectSound (0x2974a0), retained uninstalled.
//!
//! Generated EE controls the register/cycle contract. The user forbids
//! automated tests and replays, so this translation has no installed slot.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0029_74a0;
const END: u32 = 0x0029_794c;
const FRAME_BYTES: u32 = 0x90;
const SUB: u32 = 0x1d8;
const CURSOR: u32 = 0x1f4;
const CURRENT_MANAGER: u32 = 0x118;
const SUPPRESSED_SOUND: u32 = 0x62a8;
const GET_BE: u32 = 0x0014_dc80;
const GET_INTERFACE: u32 = 0x0014_dd58;
const GET_TRACK_INFO: u32 = 0x0014_4bc0;
const IS_TRACKED: u32 = 0x0029_5028;
const MEMSET: u32 = 0x0041_6210;
const START_PARAMETERS: u32 = 0x0028_7968;
const SET_RADIUS_VANISH: u32 = 0x002a_9988;
const COMPUTE_POSITION: u32 = 0x0028_8940;
const PLAY_PARAMETERS: u32 = 0x0029_06b8;
const TRACK_SOUND: u32 = 0x0029_5628;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Compute one current parameter slot. These explicit register forms retain
/// the original alternating V0/V1 scratch values and memory-read ordering.
fn slot_v0(c: &mut Cpu, field: u32, shift: u32) {
    c.g[V0][0] = c.lw(at(c, S0, CURSOR));
    c.g[V1][0] = c.lw(at(c, S1, field));
    c.g[V0][0] = sx(r(c, V0) << shift);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
}

fn slot_v1(c: &mut Cpu, field: u32, shift: u32) {
    c.g[V1][0] = c.lw(at(c, S0, CURSOR));
    c.g[V0][0] = c.lw(at(c, S1, field));
    c.g[V1][0] = sx(r(c, V1) << shift);
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
}

/// Initialize the manager's next 24-byte channel from its default and clear
/// the associated voice ownership fields before the native memset call.
fn begin_parameters(c: &mut Cpu) {
    c.g[S0][0] = c.lw(r(c, S3));
    c.g[S2][0] = 0x18;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[V0][0] = c.lw(at(c, S0, CURSOR));
    c.g[S1][0] = sx(at(c, S0, SUB));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
    c.sw(at(c, S0, CURSOR), c.g[V0][0]);
    c.g[V1][0] = c.lw(at(c, S0, CURSOR));
    c.g[V0][0] = c.lw(at(c, S1, 0x20));
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    // Keep the native unaligned doubleword copies, including their scratch
    // register effects; this is not a host memcpy of the channel structure.
    c.ldl(V0, at(c, S0, 0x1e3));
    c.ldr(V0, at(c, S0, 0x1dc));
    c.ldl(A0, at(c, S0, 0x1eb));
    c.ldr(A0, at(c, S0, 0x1e4));
    c.ldl(A3, at(c, S0, 0x1f3));
    c.ldr(A3, at(c, S0, 0x1ec));
    c.sdl(V0, at(c, V1, 7));
    c.sdr(V0, r(c, V1));
    c.sdl(A0, at(c, V1, 0xf));
    c.sdr(A0, at(c, V1, 8));
    c.sdl(A3, at(c, V1, 0x17));
    c.sdr(A3, at(c, V1, 0x10));
    c.g[V0][0] = c.lw(at(c, S0, CURSOR));
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
    c.sw(at(c, V0, 0x1fc), 0);
    slot_v1(c, 0x34, 2);
    c.sw(r(c, V1), 0);
    c.g[A3][0] = c.lw(at(c, S0, CURSOR));
    c.g[A0][0] = c.lw(at(c, S0, CURSOR));
    c.g[T0][0] = c.lw(at(c, S1, 0x20));
    c.g[A3][0] = sx(r(c, A3) << 2);
    c.mult(0, A0, A0, S2);
    c.g[V0][0] = c.lw(at(c, S1, 0x38));
    c.g[A3][0] = sx(r(c, A3).wrapping_add(r(c, V0)));
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, T0)));
    c.g[V1][0] = c.lb(r(c, A0));
    c.sw(r(c, A3), c.g[V1][0]);
    slot_v0(c, 0x3c, 2);
    c.sw(r(c, V0), 0);
    c.g[A0][0] = c.lw(at(c, S0, CURSOR));
    c.g[A0][0] = sx(r(c, A0) << 3);
    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
    c.g[A0][0] = sx(at(c, A0, 0x40));
}

/// Complete default playback fields and select this sound's bank/index.
/// The caller has already charged the original 154-cycle block; the native
/// memset continuation starts with no additional block charge.
fn finish_parameters(c: &mut Cpu) {
    c.g[V1][0] = c.lw(at(c, S0, CURSOR));
    c.g[A0][0] = 100;
    c.g[V0][0] = c.lw(at(c, S1, 0x60));
    c.g[A1][0] = 90;
    c.g[V1][0] = sx(r(c, V1) << 2);
    c.g[A2][0] = 50;
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    c.g[A3][0] = 127;
    c.sw(r(c, V1), 0);
    c.g[T0][0] = 1;
    slot_v0(c, 0x64, 2);
    c.sw(r(c, V0), 0);
    slot_v1(c, 0x68, 2);
    c.sw(r(c, V1), 0);
    slot_v0(c, 0x6c, 2);
    c.sw(r(c, V0), 0);
    slot_v1(c, 0x70, 4);
    c.sw(at(c, V1, 4), 0);
    slot_v0(c, 0x70, 4);
    c.sw(r(c, V0), c.g[A0][0]);
    slot_v1(c, 0x70, 4);
    c.sw(at(c, V1, 8), c.g[A1][0]);
    slot_v0(c, 0x70, 4);
    c.sw(at(c, V0, 12), c.g[A2][0]);
    slot_v1(c, 0x74, 2);
    c.sw(r(c, V1), 0);
    slot_v0(c, 0x80, 2);
    c.sw(r(c, V0), 0);
    slot_v1(c, 0x84, 2);
    c.sw(r(c, V1), 0);
    slot_v0(c, 0x78, 2);
    c.sw(r(c, V0), c.g[A3][0]);
    slot_v1(c, 0x7c, 2);
    c.sw(r(c, V1), c.g[T0][0]);
    c.g[A0][0] = c.lw(r(c, S3));
    c.g[V1][0] = c.lw(at(c, S3, 4));
    c.g[V0][0] = c.lw(at(c, A0, CURSOR));
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, V0), c.g[S5][0]);
    c.g[V1][0] = c.lw(r(c, S3));
    c.g[A0][0] = c.lw(at(c, S3, 8));
    c.g[V0][0] = c.lw(at(c, V1, CURSOR));
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
    c.sw(r(c, V0), c.g[S4][0]);
    c.g[V1][0] = c.lw(at(c, S3, CURRENT_MANAGER));
    c.g[A1][0] = c.lw(r(c, V1));
    c.g[V0][0] = c.lw(at(c, A1, CURSOR));
    c.g[A2][0] = sx(at(c, A1, SUB));
    c.g[V1][0] = c.lw(at(c, A2, 0x38));
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, V0), c.g[A3][0]);
    c.g[A0][0] = c.lw(at(c, A1, CURSOR));
    c.g[V1][0] = c.lw(at(c, A2, 0x1c));
    c.g[A3][0] = c.lw(at(c, A2, 0x38));
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx(r(c, V1) << 2);
    c.g[A1][0] = c.lw(at(c, A2, 0x20));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A3)));
    c.g[V0][0] = c.lw(r(c, V1));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
}

/// `void SSXAudioSystem_PlayObjectSound(AudioSystem* self, Instance* event,
/// int sound_id)` (0x2974a0). Suppress zero, an active global sound or an
/// already-tracked object. Map IDs to bank2/8/9/4, play a native positioned
/// type5 sound (radius300, priority80) and track it until native clock+50.
/// All native call returns and direct entries remain resumable.
pub fn play_object_sound(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    Frame::open(c, FRAME_BYTES);
                    frame.save(c, S0, 0x80);
                    frame.save(c, S3, 0x50);
                    c.g[S0][0] = c.g[A2][0];
                    frame.save(c, S6, 0x20);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S1, 0x70);
                    c.g[S6][0] = c.g[A1][0];
                    frame.save(c, S2, 0x60);
                    frame.save(c, S4, 0x40);
                    frame.save(c, S5, 0x30);
                    let empty = c.g[S0][0] == 0;
                    frame.save_ra(c, 0x10);
                    pc = if empty { 0x0029_7924 } else { 0x0029_74d4 };
                }
                0x0029_74d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x66;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, S3, SUPPRESSED_SOUND));
                        pc = 0x0029_750c;
                    } else {
                        pc = 0x0029_74e0;
                    }
                }
                0x0029_74e0 => {
                    c.cycles += 10;
                    call_ptr(c, GET_BE, 0x0029_74e8)?;
                    pc = 0x0029_74e8;
                }
                0x0029_74e8 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, GET_INTERFACE, 0x0029_74f4)?;
                    pc = 0x0029_74f4;
                }
                0x0029_74f4 => {
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, GET_TRACK_INFO, 0x0029_74fc)?;
                    pc = 0x0029_74fc;
                }
                0x0029_74fc => {
                    c.g[V1][0] = c.lw(r(c, V0));
                    if c.g[V1][0] == 0 {
                        frame.restore(c, S0, 0x80);
                        pc = 0x0029_7928;
                    } else {
                        pc = 0x0029_7508;
                    }
                }
                0x0029_7508 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S3, SUPPRESSED_SOUND));
                    pc = 0x0029_750c;
                }
                0x0029_750c => {
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) >= 0 {
                        frame.restore(c, S0, 0x80);
                        pc = 0x0029_7928;
                    } else {
                        pc = 0x0029_7514;
                    }
                }
                0x0029_7514 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = 0;
                    call_ptr(c, IS_TRACKED, 0x0029_7524)?;
                    pc = 0x0029_7524;
                }
                0x0029_7524 => {
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x80);
                        pc = 0x0029_7928;
                    } else {
                        pc = 0x0029_752c;
                    }
                }
                0x0029_752c => {
                    c.cycles += 5;
                    c.lq(V0, at(c, S6, 0x40));
                    c.g[S5][0] = 2;
                    c.g[V1][0] = ((c.g[S0][0] as i64) < 100) as u64;
                    let local_bank = c.g[V1][0] != 0;
                    c.sq(V0, r(c, SP));
                    pc = if local_bank { 0x0029_7540 } else { 0x0029_7548 };
                }
                0x0029_7540 => {
                    c.cycles += 2;
                    c.g[S4][0] = c.g[S0][0];
                    pc = 0x0029_7578;
                }
                0x0029_7548 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 150) as u64;
                    let bank8 = c.g[V0][0] != 0;
                    c.g[S4][0] = sx(r(c, S0).wrapping_sub(100));
                    pc = if bank8 { 0x0029_7554 } else { 0x0029_755c };
                }
                0x0029_7554 => {
                    c.cycles += 2;
                    c.g[S5][0] = 8;
                    pc = 0x0029_7578;
                }
                0x0029_755c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 200) as u64;
                    let bank9 = c.g[V0][0] != 0;
                    c.g[S4][0] = sx(r(c, S0).wrapping_sub(150));
                    pc = if bank9 { 0x0029_7568 } else { 0x0029_7570 };
                }
                0x0029_7568 => {
                    c.cycles += 2;
                    c.g[S5][0] = 9;
                    pc = 0x0029_7578;
                }
                0x0029_7570 => {
                    c.cycles += 2;
                    c.g[S4][0] = sx(r(c, S0).wrapping_sub(200));
                    c.g[S5][0] = 4;
                    pc = 0x0029_7578;
                }
                0x0029_7578 => {
                    c.cycles += 154;
                    begin_parameters(c);
                    call_ptr(c, MEMSET, 0x0029_764c)?;
                    pc = 0x0029_764c;
                }
                0x0029_764c => {
                    finish_parameters(c);
                    pc = if c.g[V0][0] == 0 { 0x0029_77f8 } else { 0x0029_77e0 };
                }
                0x0029_77e0 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, A2, 0x1c));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    let negative = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0;
                    pc = if negative { 0x0029_7824 } else { 0x0029_77f8 };
                }
                0x0029_77f8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, A2, 0x1c));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 127;
                        pc = 0x0029_7824;
                    } else {
                        pc = 0x0029_7814;
                    }
                }
                0x0029_7814 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, A2, 0x1c));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[V0][0] = c.lbu(r(c, V0));
                    pc = 0x0029_7824;
                }
                0x0029_7824 => {
                    c.cycles += 64;
                    c.sb(r(c, A0), c.g[V0][0]);
                    c.g[A2][0] = 0;
                    c.g[A1][0] = 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S0][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    call_ptr(c, START_PARAMETERS, 0x0029_783c)?;
                    pc = 0x0029_783c;
                }
                // Original secondary entry skips the call and charges only
                // its remaining block. Normal call returns enter at+783c.
                0x0029_7838 => {
                    c.cycles += 59;
                    c.g[S0][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    pc = 0x0029_783c;
                }
                0x0029_783c => {
                    c.g[A0][0] = c.lw(r(c, S0));
                    c.g[A2][0] = 1;
                    c.g[AT][0] = 0x4396_0000;
                    c.mtc1(AT, 12);
                    c.g[V1][0] = c.lw(at(c, A0, CURSOR));
                    c.g[A1][0] = c.lw(at(c, A0, 0x214));
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[AT][0] = sx(0xbf80_0000);
                    c.mtc1(AT, 13);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.sw(r(c, V1), c.g[V0][0]);
                    c.g[A0][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    c.g[V1][0] = c.lw(r(c, A0));
                    c.g[V0][0] = c.lw(at(c, V1, CURSOR));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.sw(at(c, V1, 0x1fc), c.g[SP][0]);
                    c.g[A0][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    c.g[V1][0] = c.lw(r(c, A0));
                    c.g[V0][0] = c.lw(at(c, V1, CURSOR));
                    c.g[A0][0] = c.lw(at(c, V1, 0x20c));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.sw(r(c, V0), c.g[A2][0]);
                    c.g[V1][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    c.g[A0][0] = c.lw(r(c, V1));
                    c.g[A0][0] = sx(at(c, A0, SUB));
                    call_ptr(c, SET_RADIUS_VANISH, 0x0029_78ac)?;
                    pc = 0x0029_78ac;
                }
                0x0029_78ac => {
                    c.g[A1][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, COMPUTE_POSITION, 0x0029_78bc)?;
                    pc = 0x0029_78bc;
                }
                0x0029_78bc => {
                    c.g[A1][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V1][0] = c.lw(at(c, A1, CURSOR));
                    c.g[A2][0] = c.lw(at(c, A1, 0x23c));
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.sw(r(c, V1), c.g[V0][0]);
                    c.g[A1][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    c.g[V1][0] = c.lw(r(c, A1));
                    c.g[V0][0] = c.lw(at(c, V1, CURSOR));
                    c.g[A1][0] = c.lw(at(c, V1, 0x250));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V1][0] = 80;
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
                    c.sw(r(c, V0), c.g[V1][0]);
                    call_ptr(c, PLAY_PARAMETERS, 0x0029_78fc)?;
                    pc = 0x0029_78fc;
                }
                0x0029_78fc => {
                    c.g[V1][0] = c.lw(at(c, S3, CURRENT_MANAGER));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[T0][0] = 0;
                    c.g[T1][0] = 0;
                    c.g[A3][0] = c.lw(at(c, V0, 0x260));
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(50));
                    call_ptr(c, TRACK_SOUND, 0x0029_7924)?;
                    pc = 0x0029_7924;
                }
                0x0029_7924 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x80);
                    pc = 0x0029_7928;
                }
                0x0029_7928 => {
                    c.cycles += 9;
                    for (register, offset) in [
                        (S1, 0x70), (S2, 0x60), (S3, 0x50),
                        (S4, 0x40), (S5, 0x30), (S6, 0x20),
                    ] {
                        frame.restore(c, register, offset);
                    }
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
