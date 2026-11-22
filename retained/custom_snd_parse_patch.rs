//! Native SND_ParsePatch (0x3baac0), retained uninstalled.
//!
//! The generated EE defines registers, cycles, memory order and continuations.
//! No test, build, replay or game execution accompanies this translation.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x003b_aac0;
const END: u32 = 0x003b_b0c4;
const FRAME_BYTES: u32 = 0x3b0;
const MEMSET: u32 = 0x0041_6210;
const NEXT_TAG: u32 = 0x003c_7388;
const CURSOR: u32 = 0x300;
const TAG: u32 = 0x304;
const VALUE: u32 = 0x308;
const VALUE_POINTER: u32 = 0x30c;
const VALUE_WIDTH: u32 = 0x310;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `int SND_ParsePatch(byte** cursor, ParsedPatch* out)`.
/// Parse one FE-delimited sub-patch, returning1 at FE. Every invocation starts
/// from fresh defaults; low tags0..42 and high tags80..ad have separate value
/// and payload-pointer tables. The next invocation continues at the updated
/// cursor, allowing several sub-patches before the tag decoder stops.
/// FF/FC/FD token behavior belongs to the next-tag callee, not this function.
/// All native call returns and event-check loop heads remain resumable.
pub fn snd_parse_patch(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 26;
                    Frame::open(c, FRAME_BYTES);
                    c.g[A2][0] = 0x18;
                    frame.save(c, S6, 0x340);
                    frame.save(c, S4, 0x360);
                    c.g[S6][0] = c.g[A0][0];
                    frame.save(c, S7, 0x330);
                    c.g[S4][0] = c.g[A1][0];
                    c.g[A0][0] = sx(frame.at(c, 0x2e0));
                    frame.save(c, S0, 0x3a0);
                    frame.save(c, S1, 0x390);
                    c.g[A1][0] = 0;
                    frame.save(c, S2, 0x380);
                    frame.save(c, S3, 0x370);
                    frame.save(c, S5, 0x350);
                    frame.save_ra(c, 0x320);
                    c.g[S7][0] = c.g[A0][0];
                    call_ptr(c, MEMSET, 0x003b_ab04)?;
                    pc = 0x003b_ab04;
                }
                0x003b_ab04 => {
                    c.g[V0][0] = c.lw(r(c, S6));
                    c.g[S2][0] = sx(frame.at(c, 0xb0));
                    c.g[S0][0] = sx(frame.at(c, 0x220));
                    c.g[S1][0] = sx(frame.at(c, CURSOR));
                    c.sw(at(c, S4, 0xc8), c.g[V0][0]);
                    c.g[T8][0] = 0x2a;
                    c.sw(frame.at(c, CURSOR), c.g[V0][0]);
                    c.g[V1][0] = sx(frame.at(c, 0x158));
                    pc = 0x003b_ab28;
                }
                0x003b_ab28 => {
                    c.cycles += 7;
                    c.sw(r(c, V1), 0);
                    c.g[T8][0] = sx(r(c, T8).wrapping_sub(1));
                    let more = (c.g[T8][0] as i64) >= 0;
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(4));
                    if more {
                        if back_edge_unwinds(c, 0x003b_ab28) { return Ok(()) }
                        pc = 0x003b_ab28;
                    } else { pc = 0x003b_ab44; }
                }
                0x003b_ab44 => {
                    c.cycles += 5;
                    c.g[A0][0] = sx(at(c, S4, 0x98));
                    c.g[V1][0] = sx(at(c, S4, 0xb0));
                    c.g[T8][0] = 0x2d;
                    c.g[V0][0] = sx(at(c, S0, 0xb4));
                    pc = 0x003b_ab58;
                }
                0x003b_ab58 => {
                    c.cycles += 7;
                    c.sw(r(c, V0), 0);
                    c.g[T8][0] = sx(r(c, T8).wrapping_sub(1));
                    let more = (c.g[T8][0] as i64) >= 0;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    if more {
                        if back_edge_unwinds(c, 0x003b_ab58) { return Ok(()) }
                        pc = 0x003b_ab58;
                    } else { pc = 0x003b_ab74; }
                }
                0x003b_ab74 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0];
                    c.g[T8][0] = 5;
                    pc = 0x003b_ab80;
                }
                0x003b_ab80 => {
                    c.cycles += 7;
                    c.sw(r(c, V0), 0);
                    c.g[T8][0] = sx(r(c, T8).wrapping_sub(1));
                    c.sh(r(c, V1), 0);
                    c.g[V0][0] = sx(at(c, V0, 4));
                    let more = (c.g[T8][0] as i64) >= 0;
                    c.g[V1][0] = sx(at(c, V1, 2));
                    if more {
                        if back_edge_unwinds(c, 0x003b_ab80) { return Ok(()) }
                        pc = 0x003b_ab80;
                    } else { pc = 0x003b_ab9c; }
                }
                0x003b_ab9c => {
                    initialize_defaults(c);
                    pc = 0x003b_ae14;
                }
                0x003b_acac => {
                    c.cycles += 1;
                    pc = 0x003b_acb0;
                }
                0x003b_acb0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 43) as u64;
                    let high_tag = c.g[V0][0] == 0;
                    c.g[A0][0] = c.lw(frame.at(c, VALUE));
                    pc = if high_tag { 0x003b_ad20 } else { 0x003b_acbc };
                }
                0x003b_acbc => {
                    c.cycles += 12;
                    c.g[V0][0] = sx(r(c, V1) << 2);
                    c.g[V1][0] = sx(r(c, SP).wrapping_add(r(c, V0)));
                    c.g[A1][0] = 0x14;
                    c.sw(r(c, V1), c.g[A0][0]);
                    c.g[V0][0] = c.lw(frame.at(c, TAG));
                    c.g[A0][0] = c.lw(frame.at(c, VALUE_POINTER));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.sw(r(c, V0), c.g[A0][0]);
                    c.g[V1][0] = c.lw(frame.at(c, TAG));
                    let ordinary = c.g[V1][0] != c.g[A1][0];
                    c.g[A0][0] = c.lw(frame.at(c, VALUE_POINTER));
                    pc = if ordinary { 0x003b_ae14 } else { 0x003b_acec };
                }
                0x003b_acec => {
                    c.cycles += 13;
                    c.g[V1][0] = c.lb(at(c, S4, 0x17));
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[A1][0] = c.lw(frame.at(c, VALUE_WIDTH));
                    c.g[V1][0] = sx(r(c, S4).wrapping_add(r(c, V1)));
                    c.sw(at(c, V1, 0x34), c.g[A0][0]);
                    c.g[V0][0] = c.lb(at(c, S4, 0x17));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, S4).wrapping_add(r(c, V0)));
                    c.sw(at(c, V0, 0x44), c.g[A1][0]);
                    c.g[V1][0] = c.lbu(at(c, S4, 0x17));
                    c.g[V1][0] = sx(at(c, V1, 1));
                    c.sb(at(c, S4, 0x17), c.g[V1][0]);
                    pc = 0x003b_ae14;
                }
                0x003b_ad20 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx(r(c, V1).wrapping_sub(0x80));
                    c.g[V0][0] = (c.g[A0][0] < 0x2e) as u64;
                    let unsupported = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r(c, A0) << 2);
                    pc = if unsupported { 0x003b_ae00 } else { 0x003b_ad30 };
                }
                0x003b_ad30 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x98;
                    c.g[A0][0] = c.lw(frame.at(c, VALUE));
                    c.g[V1][0] = sx(r(c, SP).wrapping_add(r(c, V0)));
                    c.sw(at(c, V1, 0x160), c.g[A0][0]);
                    c.g[V0][0] = c.lw(frame.at(c, TAG));
                    c.g[V1][0] = c.lw(frame.at(c, VALUE_POINTER));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x80));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.sw(r(c, V0), c.g[V1][0]);
                    c.g[V1][0] = c.lw(frame.at(c, TAG));
                    let ordinary = c.g[V1][0] != c.g[A1][0];
                    c.g[V0][0] = 0x99;
                    pc = if ordinary { 0x003b_ad78 } else { 0x003b_ad64 };
                }
                0x003b_ad64 => {
                    save_extended_field(c, 0x98, 0xb0);
                    pc = 0x003b_ae14;
                }
                0x003b_ad78 => {
                    c.cycles += 2;
                    let ordinary = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x9a;
                    pc = if ordinary { 0x003b_ad94 } else { 0x003b_ad80 };
                }
                0x003b_ad80 => {
                    save_extended_field(c, 0x9c, 0xb2);
                    pc = 0x003b_ae14;
                }
                0x003b_ad94 => {
                    c.cycles += 2;
                    let ordinary = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x9b;
                    pc = if ordinary { 0x003b_adb0 } else { 0x003b_ad9c };
                }
                0x003b_ad9c => {
                    save_extended_field(c, 0xa0, 0xb4);
                    pc = 0x003b_ae14;
                }
                0x003b_adb0 => {
                    c.cycles += 2;
                    let ordinary = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xa4;
                    pc = if ordinary { 0x003b_adcc } else { 0x003b_adb8 };
                }
                0x003b_adb8 => {
                    save_extended_field(c, 0xa4, 0xb6);
                    pc = 0x003b_ae14;
                }
                0x003b_adcc => {
                    c.cycles += 2;
                    let ordinary = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xa5;
                    pc = if ordinary { 0x003b_ade8 } else { 0x003b_add4 };
                }
                0x003b_add4 => {
                    save_extended_field(c, 0xa8, 0xb8);
                    pc = 0x003b_ae14;
                }
                0x003b_ade8 => {
                    c.cycles += 2;
                    let ordinary = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(frame.at(c, VALUE_POINTER));
                    pc = if ordinary { 0x003b_ae14 } else { 0x003b_adf0 };
                }
                0x003b_adf0 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lhu(frame.at(c, VALUE_WIDTH));
                    c.sw(at(c, S4, 0xac), c.g[V0][0]);
                    c.sh(at(c, S4, 0xba), c.g[V1][0]);
                    pc = 0x003b_ae14;
                }
                0x003b_ae00 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xfe;
                    pc = if c.g[V1][0] != c.g[V0][0] { 0x003b_ae14 } else { 0x003b_ae0c };
                }
                0x003b_ae0c => {
                    c.cycles += 2;
                    c.g[S5][0] = 1;
                    pc = 0x003b_ae24;
                }
                0x003b_ae14 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, NEXT_TAG, 0x003b_ae1c)?;
                    pc = 0x003b_ae1c;
                }
                0x003b_ae1c => {
                    let parsed = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lw(frame.at(c, TAG));
                    if parsed {
                        if back_edge_unwinds(c, 0x003b_acb0) { return Ok(()) }
                        pc = 0x003b_acb0;
                    } else { pc = 0x003b_ae24; }
                }
                0x003b_ae24 => {
                    publish_patch(c);
                    pc = if (c.g[S3][0] as i64) <= 0 { 0x003b_b08c } else { 0x003b_b018 };
                }
                0x003b_b018 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x0051_0000;
                    c.g[A1][0] = c.g[S7][0];
                    c.g[A3][0] = sx(r(c, V0).wrapping_sub(0x5718));
                    c.g[T2][0] = 2;
                    c.g[T1][0] = sx(at(c, A3, 0x2e0));
                    c.g[T0][0] = 0xc;
                    c.g[A2][0] = sx(at(c, S4, 0xbc));
                    pc = 0x003b_b038;
                }
                0x003b_b038 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu(at(c, A3, 0x47));
                    let ordinary = c.g[V1][0] != c.g[T2][0];
                    c.g[V0][0] = c.lb(at(c, S4, 3));
                    pc = if ordinary { 0x003b_b050 } else { 0x003b_b044 };
                }
                0x003b_b044 => {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[V1][0] { c.sw(r(c, A1), 0); }
                    pc = 0x003b_b04c;
                }
                0x003b_b04c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(at(c, S4, 3));
                    pc = 0x003b_b050;
                }
                0x003b_b050 => {
                    c.cycles += 15;
                    c.g[V1][0] = sx(r(c, T8) << 1);
                    c.g[A0][0] = c.lhu(r(c, A1));
                    c.g[T8][0] = sx(at(c, T8, 1));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    c.g[A1][0] = sx(at(c, A1, 4));
                    c.mult(0, V0, V0, T0);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T1)));
                    c.g[V0][0] = c.lhu(r(c, V1));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.sh(r(c, A2), c.g[V0][0]);
                    c.g[V0][0] = c.lb(at(c, S4, 3));
                    c.g[V0][0] = ((c.g[T8][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 2));
                    if more {
                        if back_edge_unwinds(c, 0x003b_b038) { return Ok(()) }
                        pc = 0x003b_b038;
                    } else { pc = 0x003b_b08c; }
                }
                0x003b_b08c => {
                    c.cycles += 14;
                    c.g[V1][0] = c.lw(frame.at(c, CURSOR));
                    c.g[V0][0] = c.g[S5][0];
                    frame.restore(c, S0, 0x3a0);
                    c.sw(r(c, S6), c.g[V1][0]);
                    for (register, offset) in [(S1, 0x390), (S2, 0x380), (S3, 0x370),
                        (S4, 0x360), (S5, 0x350), (S6, 0x340), (S7, 0x330)] {
                        frame.restore(c, register, offset);
                    }
                    frame.restore_ra(c, 0x320);
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

/// The generated68-cycle block initializes every sub-patch afresh. In the
/// high-tag table, channels82=1, rate84=22050 and codecA0=5 are explicit.
fn initialize_defaults(c: &mut Cpu) {
    c.cycles += 68;
    c.g[V1][0] = 0x0045_0000;
    c.g[A1][0] = 0x7f;
    c.g[A3][0] = u64::MAX;
    c.g[A2][0] = 1;
    c.g[V0][0] = 0x3c;
    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x3a80));
    c.g[A0][0] = 0x40;
    c.sw(at(c, SP, 0x1c), c.g[V0][0]);
    c.sw(at(c, SP, 0x30), c.g[A0][0]);
    c.g[T0][0] = 0x5622;
    c.sw(at(c, SP, 0x64), c.g[V1][0]);
    c.g[T1][0] = 5;
    c.sw(at(c, SP, 0x70), c.g[A1][0]);
    c.g[V0][0] = 8;
    c.sw(at(c, SP, 4), 0);
    c.g[S5][0] = 0;
    for (offset, value) in [
        (0x8, c.g[A1][0]), (0xc, 0), (0x10, c.g[A1][0]), (0x18, 0),
        (0x20, c.g[A3][0]), (0x24, c.g[A2][0]), (0x28, 0), (0x34, 0),
        (0x38, c.g[A1][0]), (0x3c, 0), (0x40, 0), (0x44, 0), (0x48, 0), (0x4c, 0),
    ] { c.sw(at(c, SP, offset), value); }
    c.sb(at(c, S4, 0x17), 0);
    for (offset, value) in [
        (0x5c, 0), (0x68, 0), (0x98, 0), (0x9c, 0), (0xa0, 0), (0xa4, 0),
        (0xa8, 0), (0x74, 0), (0x78, 0), (0x7c, 0), (0x80, 0), (0x84, 0),
        (0x88, 0), (0x8c, 0), (0x168, c.g[A2][0]), (0x170, c.g[T0][0]),
        (0x17c, c.g[A3][0]), (0x1e0, c.g[T1][0]), (0x190, c.g[V0][0]),
        (0x90, 0), (0x94, c.g[A2][0]), (0x160, 0), (0x174, 0),
        (0x178, c.g[A3][0]), (0x180, 0), (0x1d0, 0), (0x184, 0),
        (0x1d4, 0), (0x1b0, 0), (0x1d8, 0), (0x1b4, 0), (0x1dc, 0),
        (0x1e8, 0), (0x1f8, 0), (0x1ec, 0), (0x1fc, 0),
    ] { c.sw(at(c, SP, offset), value); }
}

fn save_extended_field(c: &mut Cpu, pointer: u32, width: u32) {
    c.cycles += 5;
    c.g[V0][0] = c.lw(at(c, SP, VALUE_POINTER));
    c.g[V1][0] = c.lhu(at(c, SP, VALUE_WIDTH));
    c.sw(at(c, S4, pointer), c.g[V0][0]);
    c.sh(at(c, S4, width), c.g[V1][0]);
}

/// Publish value/pointer tables into the native ParsedPatch layout in the
/// original load/store order. In particular +64(hex) receives the encoded
/// tag8a payload pointer and +68(hex) the source address from tag88.
fn publish_patch(c: &mut Cpu) {
    c.cycles += 125;
    c.g[V0][0] = c.lw(at(c, SP, 0x48));
    c.g[T8][0] = 0;
    c.g[V1][0] = c.lw(at(c, SP, 0xf8));
    c.g[S0][0] = c.lbu(at(c, SP, 4));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    for (register, offset) in [(S1, 0x8), (A0, 0xc), (V0, 0x10), (A1, 0x18),
        (A2, 0x1c), (A3, 0x20), (T0, 0x24), (T1, 0x28), (T2, 0x30),
        (T3, 0x34), (T4, 0x38), (T5, 0x3c)] {
        c.g[register][0] = c.lbu(at(c, SP, offset));
    }
    c.g[T6][0] = c.lhu(at(c, SP, 0x40));
    c.g[T7][0] = c.lhu(at(c, SP, 0x44));
    c.g[S2][0] = c.lbu(at(c, SP, 0x4c));
    for (offset, register) in [(0x8, S0), (0x9, S1), (0xa, A0), (0xb, V0),
        (0xc, A1), (0xd, A2), (0xe, A3), (0xf, T0), (0x10, T1),
        (0x11, T2), (0x12, T3), (0x14, T4), (0x15, T5)] {
        c.sb(at(c, S4, offset), c.g[register][0]);
    }
    c.sh(at(c, S4, 4), c.g[T6][0]);
    c.sh(at(c, S4, 6), c.g[T7][0]);
    c.sw(at(c, S4, 0x20), c.g[V1][0]);
    c.sb(at(c, S4, 0x16), c.g[S2][0]);
    c.g[V0][0] = c.lw(at(c, SP, 0x5c));
    c.g[V1][0] = c.lw(at(c, SP, 0x64));
    c.g[A1][0] = c.lw(at(c, SP, 0x10c));
    c.g[A0][0] = c.lw(at(c, SP, 0x114));
    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
    c.g[A2][0] = c.lw(at(c, SP, 0x124));
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lw(at(c, SP, 0x74));
    c.g[V1][0] = c.lw(at(c, SP, 0x80));
    c.g[A3][0] = c.lw(at(c, SP, 0x130));
    c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, V0)));
    c.g[T3][0] = c.lw(at(c, SP, 0x68));
    c.g[A3][0] = sx(r(c, A3).wrapping_add(r(c, V1)));
    for (register, offset) in [(T4, 0x98), (V0, 0x9c), (V1, 0xa0), (T0, 0xa4), (T1, 0xa8)] {
        c.g[register][0] = c.lw(at(c, SP, offset));
    }
    for (register, offset) in [(T2, 0x70), (T5, 0x78), (T6, 0x7c), (T7, 0x84)] {
        c.g[register][0] = c.lbu(at(c, SP, offset));
    }
    c.g[S0][0] = c.lhu(at(c, SP, 0x88));
    for (offset, register) in [(0x24, A1), (0x28, A0), (0x80, T3), (0x84, T4),
        (0x88, V0), (0x8c, V1), (0x90, T0), (0x94, T1)] {
        c.sw(at(c, S4, offset), c.g[register][0]);
    }
    c.sb(at(c, S4, 0x1a), c.g[T2][0]);
    c.sw(at(c, S4, 0x2c), c.g[A2][0]);
    c.sb(at(c, S4, 0x1b), c.g[T5][0]);
    c.sb(at(c, S4, 0x1c), c.g[T6][0]);
    c.sw(at(c, S4, 0x30), c.g[A3][0]);
    c.sb(at(c, S4, 0x1d), c.g[T7][0]);
    c.sh(at(c, S4, 0x54), c.g[S0][0]);
    c.g[V0][0] = c.lbu(at(c, SP, 0x94));
    c.g[V1][0] = c.lbu(at(c, SP, 0x160));
    c.g[A0][0] = c.lbu(at(c, SP, 0x1e0));
    c.g[A1][0] = c.lhu(at(c, SP, 0x170));
    c.g[A2][0] = c.lw(at(c, SP, 0x174));
    c.g[A3][0] = c.lw(at(c, SP, 0x178));
    c.g[T0][0] = c.lw(at(c, SP, 0x17c));
    c.g[T7][0] = c.lbu(at(c, SP, 0x8c));
    c.g[S0][0] = c.lhu(at(c, SP, 0x90));
    c.g[T1][0] = c.lhu(at(c, SP, 0x190));
    c.g[T2][0] = c.lw(at(c, SP, 0x248));
    c.g[T3][0] = c.lw(at(c, SP, 0x180));
    c.g[S2][0] = c.lbu(at(c, SP, 0x168));
    c.g[T4][0] = c.lw(at(c, SP, 0x1d0));
    c.g[T6][0] = c.lw(at(c, SP, 0x1d4));
    c.g[S3][0] = sx(r(c, S2) << 24);
    c.g[T5][0] = c.lw(at(c, SP, 0x184));
    c.g[S1][0] = c.lw(at(c, SP, 0x1b0));
    c.sb(at(c, S4, 0x1f), c.g[V0][0]);
    c.sb(at(c, S4, 2), c.g[V1][0]);
    c.sb(at(c, S4, 0x13), c.g[A0][0]);
    c.sh(at(c, S4, 0x56), c.g[A1][0]);
    c.sw(at(c, S4, 0x58), c.g[A2][0]);
    c.sw(at(c, S4, 0x5c), c.g[A3][0]);
    c.sw(at(c, S4, 0x60), c.g[T0][0]);
    c.sb(at(c, S4, 0x1e), c.g[T7][0]);
    c.sh(r(c, S4), c.g[S0][0]);
    c.sh(at(c, S4, 0x18), c.g[T1][0]);
    c.sw(at(c, S4, 0x64), c.g[T2][0]);
    c.sw(at(c, S4, 0x68), c.g[T3][0]);
    c.sw(at(c, SP, 0x2e0), c.g[T4][0]);
    c.sw(at(c, S4, 0x6c), c.g[T5][0]);
    c.sw(at(c, SP, 0x2e4), c.g[T6][0]);
    c.sb(at(c, S4, 3), c.g[S2][0]);
    c.sw(at(c, S4, 0x70), c.g[S1][0]);
    c.g[V0][0] = c.lw(at(c, SP, 0x1d8));
    c.g[V1][0] = c.lw(at(c, SP, 0x1b4));
    c.g[A0][0] = c.lw(at(c, SP, 0x1dc));
    c.g[A2][0] = c.lw(at(c, SP, 0x1f8));
    c.g[T0][0] = c.lw(at(c, SP, 0x1fc));
    c.g[A1][0] = c.lw(at(c, SP, 0x1e8));
    c.g[A3][0] = c.lw(at(c, SP, 0x1ec));
    c.sw(at(c, SP, 0x2e8), c.g[V0][0]);
    c.sw(at(c, S4, 0x74), c.g[V1][0]);
    c.sw(at(c, SP, 0x2ec), c.g[A0][0]);
    c.sw(at(c, S4, 0x78), c.g[A1][0]);
    c.sw(at(c, SP, 0x2f0), c.g[A2][0]);
    c.sw(at(c, S4, 0x7c), c.g[A3][0]);
    c.sw(at(c, SP, 0x2f4), c.g[T0][0]);
}
