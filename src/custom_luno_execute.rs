//! Native Luno bytecode execution for world object and UI scripts.
//!
//! Luno is the game's own scripting language. This dispatcher executes its
//! typed values and native calls; it does not interpret EE instructions.
//! The return sites remain explicit so a native call or event may suspend
//! and resume the VM with exactly the retail register and stack state.

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(ENTRY, luno_vm_execute, "LunoVM_Execute")];
const ENTRY: u32 = 0x0022_27d0;
const DISPATCH: u32 = 0x0022_28b8;
const NEXT_INSTRUCTION: u32 = 0x0022_4b2c;
const LOAD_INTEGER: u32 = 0x0022_4878;
const LOAD_FLOAT: u32 = 0x0022_48e0;
const LOAD_RESOURCE_ID: u32 = 0x0022_4958;
const LOAD_SMALL_INTEGER: u32 = 0x0022_49d0;
const LOAD_SMALL_FLOAT: u32 = 0x0022_4a3c;
const FRAME_BYTES: u32 = 0x140;
const SAVED_RA: u32 = 0x90;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `LunoVM::execute(this, FunctionRef*, result)` (0x2227d0).
/// FunctionRef is {blob, index}. The blob header's +4 points to 16-byte
/// function descriptors; their +4 is an instruction index and +c is the
/// argument/local slot count. Instructions start at blob+0x10; the header
/// offsets are blob-relative and remain valid when the blob is moved.
///
/// Operands 0x25/0x26/0x27 consume the next word as int/float/resource ID;
/// 0x28 stores a small immediate. Each typed value is {word0,word1,type}.
/// VM stack entries add a fourth word containing the argument index.
/// Scope cleanup releases table values before restoring the saved frame.
pub fn luno_vm_execute(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    trace_entry(c);
                    c.cycles += 31;
                    Frame::open(c, FRAME_BYTES);
                    frame.save(c, 17, 0x110);
                    frame.save(c, 21, 0xd0);
                    frame.save(c, 23, 0xb0);
                    c.g[S5][0] = c.g[A0][0];
                    frame.save(c, 16, 0x120);
                    frame.save(c, 18, 0x100);
                    frame.save(c, 19, 0xf0);
                    frame.save(c, 20, 0xe0);
                    frame.save(c, 22, 0xc0);
                    frame.save(c, 30, 0xa0);
                    frame.save_ra(c, SAVED_RA);
                    frame.save_fpr(c, 20, 0x130);
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.sw(r(c, SP).wrapping_add(0x70), c.g[V0][0]);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(0x10));
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x4));
                    c.sw(r(c, SP).wrapping_add(0x7c), 0u64);
                    c.sw(r(c, SP).wrapping_add(0x74), c.g[A0][0]);
                    c.g[V1][0] = sx(r(c, V1) << 4);
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x70));
                    c.g[V0][0] = c.lw(r(c, V0).wrapping_add(0x4));
                    c.sw(r(c, SP).wrapping_add(0x80), 0u64);
                    c.g[S1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x10));
                    c.g[S1][0] = sx(r(c, S1).wrapping_add(r(c, V1)));
                    c.g[V0][0] = sx(r(c, V0) >> 2);
                    c.sw(r(c, SP).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S0][0] = c.lw(r(c, S1).wrapping_add(0xc));
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[S7][0] = c.g[A2][0];
                    if t {
                        pc = 0x002228a0;
                        continue;
                    }
                    pc = 0x0022284c;
                    continue;
                }
                0x0022284c => {
                    c.cycles += 12;
                    c.g[A0][0] = sx(r(c, S0) << 4);
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0xffffffffa0000000;
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(0x10));
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6870));
                    c.g[RA][0] = sx(0x00222868);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x00222868)?;
                    c.g[A2][0] = sx(r(c, V0).wrapping_add(0x10));
                    c.sw(r(c, V0), c.g[S0][0]);
                    c.g[V1][0] = sx(r(c, S0).wrapping_sub(0x1));
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.g[A2][0];
                    if t {
                        pc = 0x0022289c;
                        continue;
                    }
                    pc = 0x0022287c;
                    continue;
                }
                0x0022287c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0xffffffffffffffff;
                    pc = 0x00222880;
                    continue;
                }
                0x00222880 => {
                    c.cycles += 7;
                    c.sw(r(c, V0).wrapping_add(0xc), 0u64);
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x1));
                    c.sw(r(c, V0).wrapping_add(0x4), 0u64);
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x00222880) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x00222880;
                        continue;
                    }
                    pc = 0x0022289c;
                    continue;
                }
                0x0022289c => {
                    c.cycles += 1;
                    c.sw(r(c, SP).wrapping_add(0x7c), c.g[A2][0]);
                    pc = 0x002228a0;
                    continue;
                }
                0x002228a0 => {
                    c.cycles += 5;
                    c.g[S6][0] = c.lw(r(c, S1).wrapping_add(0x4));
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[A2][0] as i64)) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                        pc = 0x00224b40;
                        continue;
                    }
                    pc = 0x002228b4;
                    continue;
                }
                0x002228b4 => {
                    c.cycles += 1;
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    pc = 0x002228b8;
                    continue;
                }
                DISPATCH => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[S2][0] = c.lw(r(c, V0));
                    c.g[V1][0] = c.g[S2][0] & 0xff;
                    c.g[V0][0] = (c.g[V1][0] < 0x2bu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x002228d4;
                    continue;
                }
                0x002228d4 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x480000;
                    c.g[V1][0] = sx(r(c, V1) << 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x6850));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.g[A0][0] = c.lw(r(c, V1));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x002228f0 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A1][0] = sx(r(c, S2) >> 16);
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = sx(r(c, V0) << 4);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = 0xc;
                    c.g[A1][0] = c.g[A1][0] & 0xff;
                    c.sw(r(c, V1), c.g[A0][0]);
                    c.mult(0, 5, 5, 2);
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(0x4));
                    c.g[V0][0] = c.lw(r(c, S5));
                    c.g[S2][0] = 0x4;
                    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S2][0];
                    c.g[S0][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
                    if t {
                        pc = 0x00222940;
                        continue;
                    }
                    pc = 0x00222934;
                    continue;
                }
                0x00222934 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00222940);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00222940)?;
                    pc = 0x00222940;
                    continue;
                }
                0x00222940 => {
                    c.cycles += 5;
                    c.sw(r(c, S1).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S1), 0u64);
                    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S2][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00222968;
                        continue;
                    }
                    pc = 0x00222954;
                    continue;
                }
                0x00222954 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00222960);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00225068, 0x00222960)?;
                    let t = true;
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    if t {
                        pc = 0x0022298c;
                        continue;
                    }
                    pc = 0x00222968;
                    continue;
                }
                0x00222968 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(r(c, S0));
                        pc = 0x00222984;
                        continue;
                    }
                    pc = 0x00222970;
                    continue;
                }
                0x00222970 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, S0).wrapping_add(0x7));
                    c.ldr(7, r(c, S0));
                    c.sdl(7, r(c, S1).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S1));
                    if t {
                        pc = 0x00222988;
                        continue;
                    }
                    pc = 0x00222984;
                    continue;
                }
                0x00222984 => {
                    c.cycles += 1;
                    c.sw(r(c, S1), c.g[V0][0]);
                    pc = 0x00222988;
                    continue;
                }
                0x00222988 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    pc = 0x0022298c;
                    continue;
                }
                0x0022298c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(0x1));
                    c.sw(r(c, SP).wrapping_add(0x80), c.g[A1][0]);
                    let t = true;
                    c.sw(r(c, S1).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x002229a0;
                    continue;
                }
                0x002229a0 => {
                    c.cycles += 15;
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V1][0] = sx(r(c, S6) << 2);
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.mult(0, 4, 4, 2);
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[A1][0] = 0x4;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.sw(r(c, SP).wrapping_add(0x30), c.g[V0][0]);
                    c.g[S0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x002229e8;
                        continue;
                    }
                    pc = 0x002229dc;
                    continue;
                }
                0x002229dc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002229e8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002229e8)?;
                    pc = 0x002229e8;
                    continue;
                }
                0x002229e8 => {
                    c.cycles += 7;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.g[V0][0] = 0x2;
                    c.sw(r(c, S0), 0u64);
                    c.lwc1(0, r(c, SP).wrapping_add(0x30));
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V0][0]);
                    let t = true;
                    c.swc1(0, r(c, S0));
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00222a04;
                    continue;
                }
                0x00222a04 => {
                    c.cycles += 15;
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V1][0] = sx(r(c, S6) << 2);
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A3)));
                    c.mult(0, 4, 4, 2);
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[A1][0] = 0x4;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.sw(r(c, SP).wrapping_add(0x34), c.g[V0][0]);
                    c.g[S0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x00222a4c;
                        continue;
                    }
                    pc = 0x00222a40;
                    continue;
                }
                0x00222a40 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222a4c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00222a4c)?;
                    pc = 0x00222a4c;
                    continue;
                }
                0x00222a4c => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x34));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x00222a60;
                    continue;
                }
                0x00222a60 => {
                    c.cycles += 9;
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.mult(0, 2, 2, 4);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[V1][0] = c.lw(r(c, A0).wrapping_add(0x8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222a84;
                    continue;
                }
                0x00222a84 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(r(c, A0));
                        pc = 0x00222ac8;
                        continue;
                    }
                    pc = 0x00222a90;
                    continue;
                }
                0x00222a90 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00222abc;
                        continue;
                    }
                    pc = 0x00222a9c;
                    continue;
                }
                0x00222a9c => {
                    c.cycles += 6;
                    c.lwc1(1, r(c, A0));
                    c.mtc1(0, 0);
                    abi::set_fcc(c, FpuF32(c.f[1]).eq_s(FpuF32(c.f[0])));
                    let t = !c.fcc();
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222ab4;
                    continue;
                }
                0x00222ab4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222abc;
                    continue;
                }
                0x00222abc => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222ac4;
                    continue;
                }
                0x00222ac4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, A0));
                    pc = 0x00222ac8;
                    continue;
                }
                0x00222ac8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222ad0;
                    continue;
                }
                0x00222ad0 => {
                    c.cycles += 9;
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.mult(0, 2, 2, 4);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[V1][0] = c.lw(r(c, A0).wrapping_add(0x8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222af4;
                    continue;
                }
                0x00222af4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(r(c, A0));
                        pc = 0x00222b38;
                        continue;
                    }
                    pc = 0x00222b00;
                    continue;
                }
                0x00222b00 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00222b2c;
                        continue;
                    }
                    pc = 0x00222b0c;
                    continue;
                }
                0x00222b0c => {
                    c.cycles += 6;
                    c.lwc1(1, r(c, A0));
                    c.mtc1(0, 0);
                    abi::set_fcc(c, FpuF32(c.f[1]).eq_s(FpuF32(c.f[0])));
                    let t = c.fcc();
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222b24;
                    continue;
                }
                0x00222b24 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222b2c;
                    continue;
                }
                0x00222b2c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222b3c;
                        continue;
                    }
                    pc = 0x00222b34;
                    continue;
                }
                0x00222b34 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, A0));
                    pc = 0x00222b38;
                    continue;
                }
                0x00222b38 => {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    pc = 0x00222b3c;
                    continue;
                }
                0x00222b3c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    if t {
                        pc = 0x00224b30;
                        continue;
                    }
                    pc = 0x00222b44;
                    continue;
                }
                0x00222b44 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S6][0] = sx(r(c, S2) >> 16);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00222b4c;
                    continue;
                }
                0x00222b4c => {
                    c.cycles += 10;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A0][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x8));
                    let t = c.g[A0][0] != 0u64;
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00222b9c;
                        continue;
                    }
                    pc = 0x00222b74;
                    continue;
                }
                0x00222b74 => {
                    c.cycles += 10;
                    c.g[V0][0] = 0x4d0000;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    let t = true;
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x00222c18;
                        continue;
                    }
                    pc = 0x00222b9c;
                    continue;
                }
                0x00222b9c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x1;
                    let t = c.g[A0][0] != c.g[A1][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x00222bbc;
                        continue;
                    }
                    pc = 0x00222ba8;
                    continue;
                }
                0x00222ba8 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw(r(c, V1));
                    c.g[A0][0] = c.g[SP][0];
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    let t = true;
                    c.g[S0][0] = (c.g[S0][0] < 0x1u64) as u64;
                    if t {
                        pc = 0x00222be8;
                        continue;
                    }
                    pc = 0x00222bbc;
                    continue;
                }
                0x00222bbc => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00222bf8;
                        continue;
                    }
                    pc = 0x00222bc4;
                    continue;
                }
                0x00222bc4 => {
                    c.cycles += 6;
                    c.lwc1(1, r(c, V1));
                    c.mtc1(0, 0);
                    abi::set_fcc(c, FpuF32(c.f[1]).eq_s(FpuF32(c.f[0])));
                    let t = c.fcc();
                    c.g[S0][0] = 0x1;
                    if t {
                        pc = 0x00222be0;
                        continue;
                    }
                    pc = 0x00222bdc;
                    continue;
                }
                0x00222bdc => {
                    c.cycles += 1;
                    c.g[S0][0] = 0u64;
                    pc = 0x00222be0;
                    continue;
                }
                0x00222be0 => {
                    c.cycles += 2;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00222be8;
                    continue;
                }
                0x00222be8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00222bf0);
                    call_ptr(c, 0x00226610, 0x00222bf0)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00222c14;
                        continue;
                    }
                    pc = 0x00222bf8;
                    continue;
                }
                0x00222bf8 => {
                    c.cycles += 7;
                    c.g[A1][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(7, r(c, A1).wrapping_add(0x7));
                    c.ldr(7, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.sdl(7, r(c, SP).wrapping_add(0x7));
                    c.sdr(7, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00222c14;
                    continue;
                }
                0x00222c14 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x00222c18;
                    continue;
                }
                0x00222c18 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00222c40;
                        continue;
                    }
                    pc = 0x00222c34;
                    continue;
                }
                0x00222c34 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222c40);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00222c40)?;
                    pc = 0x00222c40;
                    continue;
                }
                0x00222c40 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00222c68;
                        continue;
                    }
                    pc = 0x00222c54;
                    continue;
                }
                0x00222c54 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222c60);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00222c60)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00222c8c;
                        continue;
                    }
                    pc = 0x00222c68;
                    continue;
                }
                0x00222c68 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00222c84;
                        continue;
                    }
                    pc = 0x00222c70;
                    continue;
                }
                0x00222c70 => {
                    c.cycles += 5;
                    c.ldl(6, r(c, SP).wrapping_add(0x7));
                    c.ldr(6, r(c, SP));
                    c.sdl(6, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(6, r(c, S0));
                    if t {
                        pc = 0x00222c88;
                        continue;
                    }
                    pc = 0x00222c84;
                    continue;
                }
                0x00222c84 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00222c88;
                    continue;
                }
                0x00222c88 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00222c8c;
                    continue;
                }
                0x00222c8c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00222c98;
                    continue;
                }
                0x00222c98 => {
                    c.cycles += 10;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A0][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[A1][0] = c.lw(r(c, V1).wrapping_add(0x8));
                    let t = c.g[A1][0] != 0u64;
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00222ce8;
                        continue;
                    }
                    pc = 0x00222cc0;
                    continue;
                }
                0x00222cc0 => {
                    c.cycles += 10;
                    c.g[V0][0] = 0x4d0000;
                    c.g[A3][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(4, r(c, A3).wrapping_add(0x7));
                    c.ldr(4, r(c, A3));
                    c.g[A1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.sdl(4, r(c, SP).wrapping_add(0x7));
                    c.sdr(4, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    let t = true;
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x00222d54;
                        continue;
                    }
                    pc = 0x00222ce8;
                    continue;
                }
                0x00222ce8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x00222d10;
                        continue;
                    }
                    pc = 0x00222cf4;
                    continue;
                }
                0x00222cf4 => {
                    c.cycles += 7;
                    c.g[S0][0] = c.lw(r(c, V1));
                    c.g[A0][0] = c.g[SP][0];
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[RA][0] = sx(0x00222d08);
                    c.g[S0][0] = sx(0u32.wrapping_sub(r(c, S0)));
                    call_ptr(c, 0x00226610, 0x00222d08)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00222d50;
                        continue;
                    }
                    pc = 0x00222d10;
                    continue;
                }
                0x00222d10 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00222d34;
                        continue;
                    }
                    pc = 0x00222d18;
                    continue;
                }
                0x00222d18 => {
                    c.cycles += 7;
                    c.lwc1(20, r(c, V1));
                    c.g[A0][0] = c.g[SP][0];
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[RA][0] = sx(0x00222d2c);
                    c.f[20] = (-FpuF32(c.f[20])).0;
                    call_ptr(c, 0x00226618, 0x00222d2c)?;
                    let t = true;
                    c.swc1(20, r(c, V0));
                    if t {
                        pc = 0x00222d50;
                        continue;
                    }
                    pc = 0x00222d34;
                    continue;
                }
                0x00222d34 => {
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    pc = 0x00222d50;
                    continue;
                }
                0x00222d50 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x00222d54;
                    continue;
                }
                0x00222d54 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00222d7c;
                        continue;
                    }
                    pc = 0x00222d70;
                    continue;
                }
                0x00222d70 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222d7c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00222d7c)?;
                    pc = 0x00222d7c;
                    continue;
                }
                0x00222d7c => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00222da4;
                        continue;
                    }
                    pc = 0x00222d90;
                    continue;
                }
                0x00222d90 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222d9c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00222d9c)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00222dc8;
                        continue;
                    }
                    pc = 0x00222da4;
                    continue;
                }
                0x00222da4 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00222dc0;
                        continue;
                    }
                    pc = 0x00222dac;
                    continue;
                }
                0x00222dac => {
                    c.cycles += 5;
                    c.ldl(7, r(c, SP).wrapping_add(0x7));
                    c.ldr(7, r(c, SP));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S0));
                    if t {
                        pc = 0x00222dc4;
                        continue;
                    }
                    pc = 0x00222dc0;
                    continue;
                }
                0x00222dc0 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00222dc4;
                    continue;
                }
                0x00222dc4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00222dc8;
                    continue;
                }
                0x00222dc8 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00222dd4;
                    continue;
                }
                0x00222dd4 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    let t = c.g[A2][0] == c.g[V1][0];
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00222e90;
                        continue;
                    }
                    pc = 0x00222e0c;
                    continue;
                }
                0x00222e0c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x00222e48;
                        continue;
                    }
                    pc = 0x00222e18;
                    continue;
                }
                0x00222e18 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A2][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x00222e4c;
                        continue;
                    }
                    pc = 0x00222e24;
                    continue;
                }
                0x00222e24 => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, T1));
                    abi::set_fcc(c, FpuF32(c.f[0]).eq_s(FpuF32(c.f[1])));
                    let t = c.fcc();
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x00222ea4;
                        continue;
                    }
                    pc = 0x00222e40;
                    continue;
                }
                0x00222e40 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222ea4;
                        continue;
                    }
                    pc = 0x00222e48;
                    continue;
                }
                0x00222e48 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00222e4c;
                    continue;
                }
                0x00222e4c => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00222e78;
                        continue;
                    }
                    pc = 0x00222e54;
                    continue;
                }
                0x00222e54 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x00222e78;
                        continue;
                    }
                    pc = 0x00222e64;
                    continue;
                }
                0x00222e64 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(3, 1);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, T1));
                    if t {
                        pc = 0x00222e98;
                        continue;
                    }
                    pc = 0x00222e78;
                    continue;
                }
                0x00222e78 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222ea0;
                        continue;
                    }
                    pc = 0x00222e80;
                    continue;
                }
                0x00222e80 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x00222ea0;
                        continue;
                    }
                    pc = 0x00222e90;
                    continue;
                }
                0x00222e90 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(r(c, T1));
                    c.g[V1][0] = c.lw(r(c, A1));
                    pc = 0x00222e98;
                    continue;
                }
                0x00222e98 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    pc = 0x00222ea0;
                    continue;
                }
                0x00222ea0 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x00222ea4;
                    continue;
                }
                0x00222ea4 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 4, 3);
                    c.sw(r(c, SP).wrapping_add(0x38), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x00222ed4;
                        continue;
                    }
                    pc = 0x00222ec4;
                    continue;
                }
                0x00222ec4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222ed0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00222ed0)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x00222ed4;
                    continue;
                }
                0x00222ed4 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x38));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x00222ee4;
                    continue;
                }
                0x00222ee4 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    let t = c.g[A2][0] == c.g[V1][0];
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00222fa0;
                        continue;
                    }
                    pc = 0x00222f1c;
                    continue;
                }
                0x00222f1c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x00222f58;
                        continue;
                    }
                    pc = 0x00222f28;
                    continue;
                }
                0x00222f28 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A2][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x00222f5c;
                        continue;
                    }
                    pc = 0x00222f34;
                    continue;
                }
                0x00222f34 => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, T1));
                    abi::set_fcc(c, FpuF32(c.f[0]).eq_s(FpuF32(c.f[1])));
                    let t = !c.fcc();
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x00222fb4;
                        continue;
                    }
                    pc = 0x00222f50;
                    continue;
                }
                0x00222f50 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222fb4;
                        continue;
                    }
                    pc = 0x00222f58;
                    continue;
                }
                0x00222f58 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00222f5c;
                    continue;
                }
                0x00222f5c => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00222f88;
                        continue;
                    }
                    pc = 0x00222f64;
                    continue;
                }
                0x00222f64 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x00222f88;
                        continue;
                    }
                    pc = 0x00222f74;
                    continue;
                }
                0x00222f74 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(3, 1);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, T1));
                    if t {
                        pc = 0x00222fa8;
                        continue;
                    }
                    pc = 0x00222f88;
                    continue;
                }
                0x00222f88 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00222fb0;
                        continue;
                    }
                    pc = 0x00222f90;
                    continue;
                }
                0x00222f90 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x00222fb0;
                        continue;
                    }
                    pc = 0x00222fa0;
                    continue;
                }
                0x00222fa0 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(r(c, T1));
                    c.g[V1][0] = c.lw(r(c, A1));
                    pc = 0x00222fa8;
                    continue;
                }
                0x00222fa8 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    pc = 0x00222fb0;
                    continue;
                }
                0x00222fb0 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x00222fb4;
                    continue;
                }
                0x00222fb4 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 4, 3);
                    c.sw(r(c, SP).wrapping_add(0x3c), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x00222fe4;
                        continue;
                    }
                    pc = 0x00222fd4;
                    continue;
                }
                0x00222fd4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00222fe0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00222fe0)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x00222fe4;
                    continue;
                }
                0x00222fe4 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x3c));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x00222ff4;
                    continue;
                }
                0x00222ff4 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    let t = c.g[A2][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00223038;
                        continue;
                    }
                    pc = 0x0022302c;
                    continue;
                }
                0x0022302c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, T1));
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, A1));
                    if t {
                        pc = 0x002230a0;
                        continue;
                    }
                    pc = 0x00223038;
                    continue;
                }
                0x00223038 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x00223074;
                        continue;
                    }
                    pc = 0x00223044;
                    continue;
                }
                0x00223044 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A2][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x00223078;
                        continue;
                    }
                    pc = 0x00223050;
                    continue;
                }
                0x00223050 => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, T1));
                    abi::set_fcc(c, FpuF32(c.f[1]).le(FpuF32(c.f[0])));
                    let t = c.fcc();
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x002230d8;
                        continue;
                    }
                    pc = 0x0022306c;
                    continue;
                }
                0x0022306c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002230d8;
                        continue;
                    }
                    pc = 0x00223074;
                    continue;
                }
                0x00223074 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00223078;
                    continue;
                }
                0x00223078 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x002230ac;
                        continue;
                    }
                    pc = 0x00223080;
                    continue;
                }
                0x00223080 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x002230ac;
                        continue;
                    }
                    pc = 0x00223090;
                    continue;
                }
                0x00223090 => {
                    c.cycles += 4;
                    c.lwc1(0, r(c, A1));
                    c.g[V0][0] = c.lw(r(c, T1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(3, 1);
                    pc = 0x002230a0;
                    continue;
                }
                0x002230a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    if t {
                        pc = 0x002230d4;
                        continue;
                    }
                    pc = 0x002230ac;
                    continue;
                }
                0x002230ac => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002230d4;
                        continue;
                    }
                    pc = 0x002230b4;
                    continue;
                }
                0x002230b4 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x002230d4;
                        continue;
                    }
                    pc = 0x002230c4;
                    continue;
                }
                0x002230c4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(r(c, T1));
                    c.g[V1][0] = c.lw(r(c, A1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    pc = 0x002230d4;
                    continue;
                }
                0x002230d4 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x002230d8;
                    continue;
                }
                0x002230d8 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 4, 3);
                    c.sw(r(c, SP).wrapping_add(0x40), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x00223108;
                        continue;
                    }
                    pc = 0x002230f8;
                    continue;
                }
                0x002230f8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223104);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223104)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x00223108;
                    continue;
                }
                0x00223108 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x40));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x00223118;
                    continue;
                }
                0x00223118 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    let t = c.g[A2][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00223160;
                        continue;
                    }
                    pc = 0x00223150;
                    continue;
                }
                0x00223150 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    let t = true;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x002231f4;
                        continue;
                    }
                    pc = 0x00223160;
                    continue;
                }
                0x00223160 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x0022319c;
                        continue;
                    }
                    pc = 0x0022316c;
                    continue;
                }
                0x0022316c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A2][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x002231a0;
                        continue;
                    }
                    pc = 0x00223178;
                    continue;
                }
                0x00223178 => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, T1));
                    abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[0])));
                    let t = c.fcc();
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x002231f8;
                        continue;
                    }
                    pc = 0x00223194;
                    continue;
                }
                0x00223194 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002231f8;
                        continue;
                    }
                    pc = 0x0022319c;
                    continue;
                }
                0x0022319c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x002231a0;
                    continue;
                }
                0x002231a0 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x002231d0;
                        continue;
                    }
                    pc = 0x002231a8;
                    continue;
                }
                0x002231a8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x002231d0;
                        continue;
                    }
                    pc = 0x002231b8;
                    continue;
                }
                0x002231b8 => {
                    c.cycles += 6;
                    c.lwc1(0, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    let t = true;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x002231f4;
                        continue;
                    }
                    pc = 0x002231d0;
                    continue;
                }
                0x002231d0 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002231f4;
                        continue;
                    }
                    pc = 0x002231d8;
                    continue;
                }
                0x002231d8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x002231f4;
                        continue;
                    }
                    pc = 0x002231e8;
                    continue;
                }
                0x002231e8 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    pc = 0x002231f4;
                    continue;
                }
                0x002231f4 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x002231f8;
                    continue;
                }
                0x002231f8 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 4, 3);
                    c.sw(r(c, SP).wrapping_add(0x44), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x00223228;
                        continue;
                    }
                    pc = 0x00223218;
                    continue;
                }
                0x00223218 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223224);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223224)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x00223228;
                    continue;
                }
                0x00223228 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x44));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x00223238;
                    continue;
                }
                0x00223238 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    let t = c.g[A2][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x0022327c;
                        continue;
                    }
                    pc = 0x00223270;
                    continue;
                }
                0x00223270 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r(c, T1));
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, A1));
                    if t {
                        pc = 0x002232e4;
                        continue;
                    }
                    pc = 0x0022327c;
                    continue;
                }
                0x0022327c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x002232b8;
                        continue;
                    }
                    pc = 0x00223288;
                    continue;
                }
                0x00223288 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A2][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x002232bc;
                        continue;
                    }
                    pc = 0x00223294;
                    continue;
                }
                0x00223294 => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, T1));
                    abi::set_fcc(c, FpuF32(c.f[0]).le(FpuF32(c.f[1])));
                    let t = c.fcc();
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x0022331c;
                        continue;
                    }
                    pc = 0x002232b0;
                    continue;
                }
                0x002232b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0022331c;
                        continue;
                    }
                    pc = 0x002232b8;
                    continue;
                }
                0x002232b8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x002232bc;
                    continue;
                }
                0x002232bc => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x002232f0;
                        continue;
                    }
                    pc = 0x002232c4;
                    continue;
                }
                0x002232c4 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x002232f0;
                        continue;
                    }
                    pc = 0x002232d4;
                    continue;
                }
                0x002232d4 => {
                    c.cycles += 4;
                    c.lwc1(0, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    pc = 0x002232e4;
                    continue;
                }
                0x002232e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    if t {
                        pc = 0x00223318;
                        continue;
                    }
                    pc = 0x002232f0;
                    continue;
                }
                0x002232f0 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00223318;
                        continue;
                    }
                    pc = 0x002232f8;
                    continue;
                }
                0x002232f8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x00223318;
                        continue;
                    }
                    pc = 0x00223308;
                    continue;
                }
                0x00223308 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    pc = 0x00223318;
                    continue;
                }
                0x00223318 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x0022331c;
                    continue;
                }
                0x0022331c => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 4, 3);
                    c.sw(r(c, SP).wrapping_add(0x48), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x0022334c;
                        continue;
                    }
                    pc = 0x0022333c;
                    continue;
                }
                0x0022333c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223348);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223348)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x0022334c;
                    continue;
                }
                0x0022334c => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x48));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x0022335c;
                    continue;
                }
                0x0022335c => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[T1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    let t = c.g[A2][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x002233a4;
                        continue;
                    }
                    pc = 0x00223394;
                    continue;
                }
                0x00223394 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    let t = true;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if t {
                        pc = 0x00223438;
                        continue;
                    }
                    pc = 0x002233a4;
                    continue;
                }
                0x002233a4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x002233e0;
                        continue;
                    }
                    pc = 0x002233b0;
                    continue;
                }
                0x002233b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A2][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, T1).wrapping_add(0x8));
                        pc = 0x002233e4;
                        continue;
                    }
                    pc = 0x002233bc;
                    continue;
                }
                0x002233bc => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, T1));
                    abi::set_fcc(c, FpuF32(c.f[0]).lt(FpuF32(c.f[1])));
                    let t = c.fcc();
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x0022343c;
                        continue;
                    }
                    pc = 0x002233d8;
                    continue;
                }
                0x002233d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0022343c;
                        continue;
                    }
                    pc = 0x002233e0;
                    continue;
                }
                0x002233e0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x002233e4;
                    continue;
                }
                0x002233e4 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00223414;
                        continue;
                    }
                    pc = 0x002233ec;
                    continue;
                }
                0x002233ec => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x00223414;
                        continue;
                    }
                    pc = 0x002233fc;
                    continue;
                }
                0x002233fc => {
                    c.cycles += 6;
                    c.lwc1(0, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    let t = true;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if t {
                        pc = 0x00223438;
                        continue;
                    }
                    pc = 0x00223414;
                    continue;
                }
                0x00223414 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00223438;
                        continue;
                    }
                    pc = 0x0022341c;
                    continue;
                }
                0x0022341c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x00223438;
                        continue;
                    }
                    pc = 0x0022342c;
                    continue;
                }
                0x0022342c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r(c, T1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    pc = 0x00223438;
                    continue;
                }
                0x00223438 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x0022343c;
                    continue;
                }
                0x0022343c => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 4, 3);
                    c.sw(r(c, SP).wrapping_add(0x4c), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x0022346c;
                        continue;
                    }
                    pc = 0x0022345c;
                    continue;
                }
                0x0022345c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223468);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223468)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x0022346c;
                    continue;
                }
                0x0022346c => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x4c));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x0022347c;
                    continue;
                }
                0x0022347c => {
                    c.cycles += 13;
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 6, 2, 4);
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[T1][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
                    c.g[A0][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    c.g[A2][0] = c.g[V0][0] & 0xff;
                    let t = c.g[A0][0] != 0u64;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    if t {
                        pc = 0x002234c8;
                        continue;
                    }
                    pc = 0x002234b0;
                    continue;
                }
                0x002234b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8));

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0u64;
                        pc = 0x002234f8;
                        continue;
                    }
                    pc = 0x002234bc;
                    continue;
                }
                0x002234bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, V1));
                    let t = true;
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    if t {
                        pc = 0x002234f8;
                        continue;
                    }
                    pc = 0x002234c8;
                    continue;
                }
                0x002234c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(r(c, V1));
                        pc = 0x002234dc;
                        continue;
                    }
                    pc = 0x002234d4;
                    continue;
                }
                0x002234d4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002234f8;
                        continue;
                    }
                    pc = 0x002234dc;
                    continue;
                }
                0x002234dc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x002234f0;
                        continue;
                    }
                    pc = 0x002234e4;
                    continue;
                }
                0x002234e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, T1));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0];
                    if t {
                        pc = 0x002234f8;
                        continue;
                    }
                    pc = 0x002234f0;
                    continue;
                }
                0x002234f0 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = c.g[V1][0];
                    pc = 0x002234f8;
                    continue;
                }
                0x002234f8 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 6, 3);
                    c.sw(r(c, SP).wrapping_add(0x50), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x0022352c;
                        continue;
                    }
                    pc = 0x0022351c;
                    continue;
                }
                0x0022351c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223528);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223528)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x0022352c;
                    continue;
                }
                0x0022352c => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x50));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x0022353c;
                    continue;
                }
                0x0022353c => {
                    c.cycles += 14;
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 6, 2, 4);
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[T1][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
                    c.g[A0][0] = c.lw(r(c, T1).wrapping_add(0x8));
                    c.g[A2][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    let t = c.g[A0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0022359c;
                        continue;
                    }
                    pc = 0x00223574;
                    continue;
                }
                0x00223574 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(r(c, V1));
                        pc = 0x00223588;
                        continue;
                    }
                    pc = 0x00223580;
                    continue;
                }
                0x00223580 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0022359c;
                        continue;
                    }
                    pc = 0x00223588;
                    continue;
                }
                0x00223588 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x00223598;
                        continue;
                    }
                    pc = 0x00223590;
                    continue;
                }
                0x00223590 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(r(c, T1));
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                    pc = 0x00223598;
                    continue;
                }
                0x00223598 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[V1][0];
                    pc = 0x0022359c;
                    continue;
                }
                0x0022359c => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 6, 3);
                    c.sw(r(c, SP).wrapping_add(0x54), c.g[V0][0]);
                    c.g[A0][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A0][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x002235d0;
                        continue;
                    }
                    pc = 0x002235c0;
                    continue;
                }
                0x002235c0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002235cc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002235cc)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x002235d0;
                    continue;
                }
                0x002235d0 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x54));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x002235e0;
                    continue;
                }
                0x002235e0 => {
                    c.cycles += 13;
                    c.g[V1][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = sx(r(c, S2) >> 16);
                    c.mult(0, 3, 3, 5);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 6, 2, 5);
                    c.g[S2][0] = 0x4;
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[S0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    if t {
                        pc = 0x00223620;
                        continue;
                    }
                    pc = 0x00223614;
                    continue;
                }
                0x00223614 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00223620);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223620)?;
                    pc = 0x00223620;
                    continue;
                }
                0x00223620 => {
                    c.cycles += 5;
                    c.sw(r(c, S1).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S1), 0u64);
                    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] == c.g[S2][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00224114;
                        continue;
                    }
                    pc = 0x00223634;
                    continue;
                }
                0x00223634 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];

                    if t {
                        pc = 0x00224130;
                        continue;
                    }
                    pc = 0x0022363c;
                    continue;
                }
                0x0022363c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S0));
                    if t {
                        pc = 0x00224144;
                        continue;
                    }
                    pc = 0x00223644;
                    continue;
                }
                0x00223644 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A3][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[A1][0];
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x002236dc;
                        continue;
                    }
                    pc = 0x0022367c;
                    continue;
                }
                0x0022367c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0022369c;
                        continue;
                    }
                    pc = 0x00223688;
                    continue;
                }
                0x00223688 => {
                    c.cycles += 5;
                    c.lwc1(20, r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    c.lwc1(0, r(c, A3));
                    let t = true;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    if t {
                        pc = 0x00223708;
                        continue;
                    }
                    pc = 0x0022369c;
                    continue;
                }
                0x0022369c => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x002236b4;
                        continue;
                    }
                    pc = 0x002236a4;
                    continue;
                }
                0x002236a4 => {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, A3));
                    if t {
                        pc = 0x00223748;
                        continue;
                    }
                    pc = 0x002236b4;
                    continue;
                }
                0x002236b4 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x0022375c;
                        continue;
                    }
                    pc = 0x002236bc;
                    continue;
                }
                0x002236bc => {
                    c.cycles += 8;
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw(r(c, A3));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[RA][0] = sx(0x002236d4);
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    call_ptr(c, 0x00226620, 0x002236d4)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00223778;
                        continue;
                    }
                    pc = 0x002236dc;
                    continue;
                }
                0x002236dc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[A1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x00223718;
                        continue;
                    }
                    pc = 0x002236e8;
                    continue;
                }
                0x002236e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x0022371c;
                        continue;
                    }
                    pc = 0x002236f4;
                    continue;
                }
                0x002236f4 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A3));
                    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                    c.lwc1(20, r(c, S0));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00223708;
                    continue;
                }
                0x00223708 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00223710);
                    c.f[20] = (FpuF32(c.f[20]) + FpuF32(c.f[0])).0;
                    call_ptr(c, 0x00226618, 0x00223710)?;
                    let t = true;
                    c.swc1(20, r(c, V0));
                    if t {
                        pc = 0x00223778;
                        continue;
                    }
                    pc = 0x00223718;
                    continue;
                }
                0x00223718 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0022371c;
                    continue;
                }
                0x0022371c => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x0022375c;
                        continue;
                    }
                    pc = 0x00223724;
                    continue;
                }
                0x00223724 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x0022375c;
                        continue;
                    }
                    pc = 0x00223734;
                    continue;
                }
                0x00223734 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A3));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    pc = 0x00223748;
                    continue;
                }
                0x00223748 => {
                    c.cycles += 5;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[RA][0] = sx(0x00223754);
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    call_ptr(c, 0x00226610, 0x00223754)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00223778;
                        continue;
                    }
                    pc = 0x0022375c;
                    continue;
                }
                0x0022375c => {
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    pc = 0x00223778;
                    continue;
                }
                0x00223778 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xc;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x002237a4;
                        continue;
                    }
                    pc = 0x00223798;
                    continue;
                }
                0x00223798 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002237a4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002237a4)?;
                    pc = 0x002237a4;
                    continue;
                }
                0x002237a4 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x002237cc;
                        continue;
                    }
                    pc = 0x002237b8;
                    continue;
                }
                0x002237b8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002237c4);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x002237c4)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x002237f0;
                        continue;
                    }
                    pc = 0x002237cc;
                    continue;
                }
                0x002237cc => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x002237e8;
                        continue;
                    }
                    pc = 0x002237d4;
                    continue;
                }
                0x002237d4 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, SP).wrapping_add(0x7));
                    c.ldr(7, r(c, SP));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S0));
                    if t {
                        pc = 0x002237ec;
                        continue;
                    }
                    pc = 0x002237e8;
                    continue;
                }
                0x002237e8 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x002237ec;
                    continue;
                }
                0x002237ec => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x002237f0;
                    continue;
                }
                0x002237f0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x002237fc;
                    continue;
                }
                0x002237fc => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A3][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[A1][0];
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x0022386c;
                        continue;
                    }
                    pc = 0x00223834;
                    continue;
                }
                0x00223834 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00223854;
                        continue;
                    }
                    pc = 0x00223840;
                    continue;
                }
                0x00223840 => {
                    c.cycles += 5;
                    c.lwc1(20, r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    c.lwc1(0, r(c, A3));
                    let t = true;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    if t {
                        pc = 0x00223898;
                        continue;
                    }
                    pc = 0x00223854;
                    continue;
                }
                0x00223854 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x002238ec;
                        continue;
                    }
                    pc = 0x0022385c;
                    continue;
                }
                0x0022385c => {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, A3));
                    if t {
                        pc = 0x002238d8;
                        continue;
                    }
                    pc = 0x0022386c;
                    continue;
                }
                0x0022386c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[A1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x002238a8;
                        continue;
                    }
                    pc = 0x00223878;
                    continue;
                }
                0x00223878 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x002238ac;
                        continue;
                    }
                    pc = 0x00223884;
                    continue;
                }
                0x00223884 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A3));
                    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                    c.lwc1(20, r(c, S0));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00223898;
                    continue;
                }
                0x00223898 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002238a0);
                    c.f[20] = (FpuF32(c.f[20]) - FpuF32(c.f[0])).0;
                    call_ptr(c, 0x00226618, 0x002238a0)?;
                    let t = true;
                    c.swc1(20, r(c, V0));
                    if t {
                        pc = 0x00223908;
                        continue;
                    }
                    pc = 0x002238a8;
                    continue;
                }
                0x002238a8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x002238ac;
                    continue;
                }
                0x002238ac => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x002238ec;
                        continue;
                    }
                    pc = 0x002238b4;
                    continue;
                }
                0x002238b4 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x002238ec;
                        continue;
                    }
                    pc = 0x002238c4;
                    continue;
                }
                0x002238c4 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A3));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    pc = 0x002238d8;
                    continue;
                }
                0x002238d8 => {
                    c.cycles += 5;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[RA][0] = sx(0x002238e4);
                    c.g[S0][0] = sx(r(c, S0).wrapping_sub(r(c, V0)));
                    call_ptr(c, 0x00226610, 0x002238e4)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00223908;
                        continue;
                    }
                    pc = 0x002238ec;
                    continue;
                }
                0x002238ec => {
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    pc = 0x00223908;
                    continue;
                }
                0x00223908 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xc;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00223934;
                        continue;
                    }
                    pc = 0x00223928;
                    continue;
                }
                0x00223928 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223934);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223934)?;
                    pc = 0x00223934;
                    continue;
                }
                0x00223934 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x0022395c;
                        continue;
                    }
                    pc = 0x00223948;
                    continue;
                }
                0x00223948 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223954);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00223954)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00223980;
                        continue;
                    }
                    pc = 0x0022395c;
                    continue;
                }
                0x0022395c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00223978;
                        continue;
                    }
                    pc = 0x00223964;
                    continue;
                }
                0x00223964 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, SP).wrapping_add(0x7));
                    c.ldr(7, r(c, SP));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S0));
                    if t {
                        pc = 0x0022397c;
                        continue;
                    }
                    pc = 0x00223978;
                    continue;
                }
                0x00223978 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x0022397c;
                    continue;
                }
                0x0022397c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00223980;
                    continue;
                }
                0x00223980 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x0022398c;
                    continue;
                }
                0x0022398c => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A3][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[A1][0];
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x002239fc;
                        continue;
                    }
                    pc = 0x002239c4;
                    continue;
                }
                0x002239c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002239e4;
                        continue;
                    }
                    pc = 0x002239d0;
                    continue;
                }
                0x002239d0 => {
                    c.cycles += 5;
                    c.lwc1(20, r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    c.lwc1(0, r(c, A3));
                    let t = true;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    if t {
                        pc = 0x00223a28;
                        continue;
                    }
                    pc = 0x002239e4;
                    continue;
                }
                0x002239e4 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223a7c;
                        continue;
                    }
                    pc = 0x002239ec;
                    continue;
                }
                0x002239ec => {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, A3));
                    if t {
                        pc = 0x00223a68;
                        continue;
                    }
                    pc = 0x002239fc;
                    continue;
                }
                0x002239fc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[A1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x00223a38;
                        continue;
                    }
                    pc = 0x00223a08;
                    continue;
                }
                0x00223a08 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x00223a3c;
                        continue;
                    }
                    pc = 0x00223a14;
                    continue;
                }
                0x00223a14 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A3));
                    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                    c.lwc1(20, r(c, S0));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00223a28;
                    continue;
                }
                0x00223a28 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00223a30);
                    c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0;
                    call_ptr(c, 0x00226618, 0x00223a30)?;
                    let t = true;
                    c.swc1(20, r(c, V0));
                    if t {
                        pc = 0x00223a98;
                        continue;
                    }
                    pc = 0x00223a38;
                    continue;
                }
                0x00223a38 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00223a3c;
                    continue;
                }
                0x00223a3c => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223a7c;
                        continue;
                    }
                    pc = 0x00223a44;
                    continue;
                }
                0x00223a44 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223a7c;
                        continue;
                    }
                    pc = 0x00223a54;
                    continue;
                }
                0x00223a54 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A3));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    pc = 0x00223a68;
                    continue;
                }
                0x00223a68 => {
                    c.cycles += 5;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    c.g[RA][0] = sx(0x00223a74);
                    c.mult(0, 16, 16, 2);
                    call_ptr(c, 0x00226610, 0x00223a74)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00223a98;
                        continue;
                    }
                    pc = 0x00223a7c;
                    continue;
                }
                0x00223a7c => {
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    pc = 0x00223a98;
                    continue;
                }
                0x00223a98 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xc;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00223ac4;
                        continue;
                    }
                    pc = 0x00223ab8;
                    continue;
                }
                0x00223ab8 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223ac4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223ac4)?;
                    pc = 0x00223ac4;
                    continue;
                }
                0x00223ac4 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00223aec;
                        continue;
                    }
                    pc = 0x00223ad8;
                    continue;
                }
                0x00223ad8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223ae4);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00223ae4)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00223b10;
                        continue;
                    }
                    pc = 0x00223aec;
                    continue;
                }
                0x00223aec => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00223b08;
                        continue;
                    }
                    pc = 0x00223af4;
                    continue;
                }
                0x00223af4 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, SP).wrapping_add(0x7));
                    c.ldr(7, r(c, SP));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S0));
                    if t {
                        pc = 0x00223b0c;
                        continue;
                    }
                    pc = 0x00223b08;
                    continue;
                }
                0x00223b08 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00223b0c;
                    continue;
                }
                0x00223b0c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00223b10;
                    continue;
                }
                0x00223b10 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00223b1c;
                    continue;
                }
                0x00223b1c => {
                    c.cycles += 14;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.mult(0, 2, 4, 5);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A0][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x00223b9c;
                        continue;
                    }
                    pc = 0x00223b54;
                    continue;
                }
                0x00223b54 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00223b74;
                        continue;
                    }
                    pc = 0x00223b60;
                    continue;
                }
                0x00223b60 => {
                    c.cycles += 5;
                    c.lwc1(20, r(c, S0));
                    c.g[A0][0] = c.g[SP][0];
                    c.lwc1(0, r(c, A1));
                    let t = true;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x00223bc8;
                        continue;
                    }
                    pc = 0x00223b74;
                    continue;
                }
                0x00223b74 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223c30;
                        continue;
                    }
                    pc = 0x00223b7c;
                    continue;
                }
                0x00223b7c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(r(c, S0));

                    if c.g[V0][0] == 0u64 {
                        pc = 0x00223b90;
                        continue;
                    }
                    pc = 0x00223b90;
                    continue;
                }
                0x00223b90 => {
                    c.cycles += 3;
                    c.div(0, 16, 2);
                    let t = true;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x00223c1c;
                        continue;
                    }
                    pc = 0x00223b9c;
                    continue;
                }
                0x00223b9c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x00223bdc;
                        continue;
                    }
                    pc = 0x00223ba8;
                    continue;
                }
                0x00223ba8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[A2][0] = c.lw(r(c, S0).wrapping_add(0x8));
                        pc = 0x00223be0;
                        continue;
                    }
                    pc = 0x00223bb4;
                    continue;
                }
                0x00223bb4 => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A1));
                    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                    c.lwc1(20, r(c, S0));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[V1][0]);
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00223bc8;
                    continue;
                }
                0x00223bc8 => {
                    c.cycles += 5;
                    c.f[20] = FpuF32(c.f[20]).div(FpuF32(c.f[0])).0;
                    c.g[RA][0] = sx(0x00223bd4);
                    call_ptr(c, 0x00226618, 0x00223bd4)?;
                    let t = true;
                    c.swc1(20, r(c, V0));
                    if t {
                        pc = 0x00223c4c;
                        continue;
                    }
                    pc = 0x00223bdc;
                    continue;
                }
                0x00223bdc => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00223be0;
                    continue;
                }
                0x00223be0 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223c30;
                        continue;
                    }
                    pc = 0x00223be8;
                    continue;
                }
                0x00223be8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223c30;
                        continue;
                    }
                    pc = 0x00223bf8;
                    continue;
                }
                0x00223bf8 => {
                    c.cycles += 8;
                    c.lwc1(0, r(c, A1));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(r(c, S0));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    c.div(0, 16, 2);

                    if c.g[V0][0] == 0u64 {
                        pc = 0x00223c18;
                        continue;
                    }
                    pc = 0x00223c18;
                    continue;
                }
                0x00223c18 => {
                    c.cycles += 1;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A2][0]);
                    pc = 0x00223c1c;
                    continue;
                }
                0x00223c1c => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lo[0];
                    c.g[RA][0] = sx(0x00223c28);
                    call_ptr(c, 0x00226610, 0x00223c28)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00223c4c;
                        continue;
                    }
                    pc = 0x00223c30;
                    continue;
                }
                0x00223c30 => {
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    pc = 0x00223c4c;
                    continue;
                }
                0x00223c4c => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xc;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00223c78;
                        continue;
                    }
                    pc = 0x00223c6c;
                    continue;
                }
                0x00223c6c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223c78);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223c78)?;
                    pc = 0x00223c78;
                    continue;
                }
                0x00223c78 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00223ca0;
                        continue;
                    }
                    pc = 0x00223c8c;
                    continue;
                }
                0x00223c8c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223c98);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00223c98)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00223cc4;
                        continue;
                    }
                    pc = 0x00223ca0;
                    continue;
                }
                0x00223ca0 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00223cbc;
                        continue;
                    }
                    pc = 0x00223ca8;
                    continue;
                }
                0x00223ca8 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, SP).wrapping_add(0x7));
                    c.ldr(7, r(c, SP));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S0));
                    if t {
                        pc = 0x00223cc0;
                        continue;
                    }
                    pc = 0x00223cbc;
                    continue;
                }
                0x00223cbc => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00223cc0;
                    continue;
                }
                0x00223cc0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00223cc4;
                    continue;
                }
                0x00223cc4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00223cd0;
                    continue;
                }
                0x00223cd0 => {
                    c.cycles += 15;
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 2, 4, 5);
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 5);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = c.lw(r(c, A0).wrapping_add(0x8));
                    c.g[S1][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223d68;
                        continue;
                    }
                    pc = 0x00223d0c;
                    continue;
                }
                0x00223d0c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        pc = 0x00223d44;
                        continue;
                    }
                    pc = 0x00223d18;
                    continue;
                }
                0x00223d18 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r(c, A0));
                    c.g[V1][0] = c.lw(r(c, V1));
                    c.g[A0][0] = c.g[SP][0];

                    if c.g[V0][0] == 0u64 {
                        pc = 0x00223d2c;
                        continue;
                    }
                    pc = 0x00223d2c;
                    continue;
                }
                0x00223d2c => {
                    c.cycles += 6;
                    c.div(0, 3, 2);
                    c.g[S0][0] = c.hi[0];
                    c.g[RA][0] = sx(0x00223d3c);
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A1][0]);
                    call_ptr(c, 0x00226610, 0x00223d3c)?;
                    let t = true;
                    c.sw(r(c, V0), c.g[S0][0]);
                    if t {
                        pc = 0x00223d84;
                        continue;
                    }
                    pc = 0x00223d44;
                    continue;
                }
                0x00223d44 => {
                    c.cycles += 9;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    let t = true;
                    c.g[V1][0] = 0xc;
                    if t {
                        pc = 0x00223d88;
                        continue;
                    }
                    pc = 0x00223d68;
                    continue;
                }
                0x00223d68 => {
                    c.cycles += 7;
                    c.g[A1][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(7, r(c, A1).wrapping_add(0x7));
                    c.ldr(7, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.sdl(7, r(c, SP).wrapping_add(0x7));
                    c.sdr(7, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00223d84;
                    continue;
                }
                0x00223d84 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xc;
                    pc = 0x00223d88;
                    continue;
                }
                0x00223d88 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 17, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00223db0;
                        continue;
                    }
                    pc = 0x00223da4;
                    continue;
                }
                0x00223da4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223db0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223db0)?;
                    pc = 0x00223db0;
                    continue;
                }
                0x00223db0 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00223dd8;
                        continue;
                    }
                    pc = 0x00223dc4;
                    continue;
                }
                0x00223dc4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223dd0);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00223dd0)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00223dfc;
                        continue;
                    }
                    pc = 0x00223dd8;
                    continue;
                }
                0x00223dd8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00223df4;
                        continue;
                    }
                    pc = 0x00223de0;
                    continue;
                }
                0x00223de0 => {
                    c.cycles += 5;
                    c.ldl(6, r(c, SP).wrapping_add(0x7));
                    c.ldr(6, r(c, SP));
                    c.sdl(6, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(6, r(c, S0));
                    if t {
                        pc = 0x00223df8;
                        continue;
                    }
                    pc = 0x00223df4;
                    continue;
                }
                0x00223df4 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00223df8;
                    continue;
                }
                0x00223df8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00223dfc;
                    continue;
                }
                0x00223dfc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00223e08;
                    continue;
                }
                0x00223e08 => {
                    c.cycles += 29;
                    c.g[A1][0] = 0xc;
                    c.g[A0][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 2, 4, 5);
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 5);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = c.lw(r(c, A0).wrapping_add(0x8));
                    c.g[A3][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = 0x4d0000;
                    c.g[A2][0] = sx(r(c, V0).wrapping_sub(0x6f68));
                    c.ldl(3, r(c, A2).wrapping_add(0x7));
                    c.ldr(3, r(c, A2));
                    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0x8));
                    c.sdl(3, r(c, SP).wrapping_add(0x7));
                    c.sdr(3, r(c, SP));
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[A0][0]);
                    c.g[V1][0] = 0xc;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 7, 3);
                    c.g[S1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00223e88;
                        continue;
                    }
                    pc = 0x00223e7c;
                    continue;
                }
                0x00223e7c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223e88);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223e88)?;
                    pc = 0x00223e88;
                    continue;
                }
                0x00223e88 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00223eb0;
                        continue;
                    }
                    pc = 0x00223e9c;
                    continue;
                }
                0x00223e9c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223ea8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00223ea8)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00223ed4;
                        continue;
                    }
                    pc = 0x00223eb0;
                    continue;
                }
                0x00223eb0 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00223ecc;
                        continue;
                    }
                    pc = 0x00223eb8;
                    continue;
                }
                0x00223eb8 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, SP).wrapping_add(0x7));
                    c.ldr(7, r(c, SP));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S0));
                    if t {
                        pc = 0x00223ed0;
                        continue;
                    }
                    pc = 0x00223ecc;
                    continue;
                }
                0x00223ecc => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00223ed0;
                    continue;
                }
                0x00223ed0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00223ed4;
                    continue;
                }
                0x00223ed4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[SP][0];
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00223ee0;
                    continue;
                }
                0x00223ee0 => {
                    c.cycles += 9;
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[S4][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 19, 3, 20);
                    c.g[V0][0] = sx(r(c, S3).wrapping_add(r(c, A1)));
                    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = sx(r(c, S2) >> 24);
                    if t {
                        pc = 0x00223f48;
                        continue;
                    }
                    pc = 0x00223f04;
                    continue;
                }
                0x00223f04 => {
                    c.cycles += 17;
                    c.g[S1][0] = sx(r(c, SP).wrapping_add(0x10));
                    c.mult(0, 2, 16, 20);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00223f18);
                    c.g[S0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
                    call_ptr(c, 0x002224a8, 0x00223f18)?;
                    c.g[T0][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.g[T0][0] = c.g[T0][0] & 0xff;
                    c.g[A0][0] = c.g[S1][0];
                    c.mult(0, 2, 8, 20);
                    c.g[A2][0] = c.g[A1][0];
                    c.g[A1][0] = sx(r(c, S3).wrapping_add(r(c, A1)));
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223f40);
                    c.g[T0][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                    call_ptr(c, 0x00222648, 0x00223f40)?;
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    if t {
                        pc = 0x00224b30;
                        continue;
                    }
                    pc = 0x00223f48;
                    continue;
                }
                0x00223f48 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00223f50);
                    call_ptr(c, 0x002224a0, 0x00223f50)?;
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    if t {
                        pc = 0x00224b30;
                        continue;
                    }
                    pc = 0x00223f58;
                    continue;
                }
                0x00223f58 => {
                    c.cycles += 26;
                    c.g[V0][0] = sx(r(c, S2) >> 14);
                    c.g[V1][0] = 0x440000;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(0x1f38));
                    c.g[V0][0] = c.g[V0][0] & 0x3fc;
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A3][0] = sx(r(c, S2) >> 24);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A1][0] = c.g[A3][0] & 0xff;
                    c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[S0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = sx(r(c, V1).wrapping_sub(r(c, A1)));
                    c.g[S0][0] = c.g[S0][0] & 0xff;
                    c.g[A2][0] = sx(r(c, A2) << 4);
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[A2][0] = sx(r(c, A0).wrapping_add(r(c, A2)));
                    c.g[S4][0] = c.g[A3][0];
                    c.g[A0][0] = c.g[SP][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00223fa4);
                    c.g[S1][0] = 0x4;
                    let builtin = (r(c, S2) >> 16) & 255;
                    if let Some(flow) = crate::custom_script_effects::builtin(c, builtin, 0x0022_3fa4) {
                        flow?;
                    } else {
                        call_ptr(c, t, 0x00223fa4)?;
                    }
                    pc = 0x00223fa4;
                    continue;
                }
                0x00223fa4 => {
                    crate::custom_script_effects::after_builtin(c, (r(c, S2) >> 16) & 255, 0x0022_3fa4)?;
                    c.g[V0][0] = 0xc;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.mult(0, 16, 16, 2);
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x00223fcc;
                        continue;
                    }
                    pc = 0x00223fc0;
                    continue;
                }
                0x00223fc0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223fcc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00223fcc)?;
                    pc = 0x00223fcc;
                    continue;
                }
                0x00223fcc => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00223ff4;
                        continue;
                    }
                    pc = 0x00223fe0;
                    continue;
                }
                0x00223fe0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00223fec);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00225068, 0x00223fec)?;
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    if t {
                        pc = 0x00224018;
                        continue;
                    }
                    pc = 0x00223ff4;
                    continue;
                }
                0x00223ff4 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP));
                    if t {
                        pc = 0x00224010;
                        continue;
                    }
                    pc = 0x00223ffc;
                    continue;
                }
                0x00223ffc => {
                    c.cycles += 5;
                    c.ldl(5, r(c, SP).wrapping_add(0x7));
                    c.ldr(5, r(c, SP));
                    c.sdl(5, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(5, r(c, S0));
                    if t {
                        pc = 0x00224014;
                        continue;
                    }
                    pc = 0x00224010;
                    continue;
                }
                0x00224010 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00224014;
                    continue;
                }
                0x00224014 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    pc = 0x00224018;
                    continue;
                }
                0x00224018 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    c.g[A1][0] = 0x2;
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(r(c, S4)));
                    let t = true;
                    c.sw(r(c, SP).wrapping_add(0x80), c.g[V1][0]);
                    if t {
                        pc = 0x00224650;
                        continue;
                    }
                    pc = 0x00224030;
                    continue;
                }
                0x00224030 => {
                    c.cycles += 16;
                    c.g[V0][0] = 0xc;
                    c.g[A2][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 3, 6, 2);
                    c.g[A3][0] = c.lw(r(c, S5));
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = sx(r(c, S2) >> 16);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[A1][0] = c.g[A1][0] & 0xff;
                    c.g[A2][0] = sx(r(c, V1).wrapping_add(r(c, A3)));
                    c.mult(0, 3, 4, 2);
                    c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, A3)));
                    c.mult(0, 3, 5, 2);
                    c.g[RA][0] = sx(0x00224068);
                    c.g[A1][0] = sx(r(c, V1).wrapping_add(r(c, A3)));
                    call_ptr(c, 0x00225338, 0x00224068)?;
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    if t {
                        pc = 0x00224b30;
                        continue;
                    }
                    pc = 0x00224070;
                    continue;
                }
                0x00224070 => {
                    c.cycles += 20;
                    c.g[V0][0] = 0xc;
                    c.g[A2][0] = sx(r(c, S2) >> 24);
                    c.mult(0, 3, 6, 2);
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[S0][0] = sx(r(c, S2) >> 16);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[S0][0] = c.g[S0][0] & 0xff;
                    c.mult(0, 16, 16, 2);
                    c.g[A2][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.mult(0, 3, 4, 2);
                    c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                    c.g[RA][0] = sx(0x002240a8);
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S0)));
                    call_ptr(c, 0x00225338, 0x002240a8)?;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S0));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1));
                    let t = true;
                    c.sw(r(c, S0), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x002240c0;
                    continue;
                }
                0x002240c0 => {
                    c.cycles += 13;
                    c.g[V1][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = sx(r(c, S2) >> 16);
                    c.mult(0, 3, 3, 5);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 6, 2, 5);
                    c.g[S2][0] = 0x4;
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[S0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    if t {
                        pc = 0x00224100;
                        continue;
                    }
                    pc = 0x002240f4;
                    continue;
                }
                0x002240f4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00224100);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224100)?;
                    pc = 0x00224100;
                    continue;
                }
                0x00224100 => {
                    c.cycles += 5;
                    c.sw(r(c, S1).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S1), 0u64);
                    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S2][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00224128;
                        continue;
                    }
                    pc = 0x00224114;
                    continue;
                }
                0x00224114 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00224120);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00225068, 0x00224120)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    if t {
                        pc = 0x0022414c;
                        continue;
                    }
                    pc = 0x00224128;
                    continue;
                }
                0x00224128 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(r(c, S0));
                        pc = 0x00224144;
                        continue;
                    }
                    pc = 0x00224130;
                    continue;
                }
                0x00224130 => {
                    c.cycles += 5;
                    c.ldl(7, r(c, S0).wrapping_add(0x7));
                    c.ldr(7, r(c, S0));
                    c.sdl(7, r(c, S1).wrapping_add(0x7));
                    let t = true;
                    c.sdr(7, r(c, S1));
                    if t {
                        pc = 0x00224148;
                        continue;
                    }
                    pc = 0x00224144;
                    continue;
                }
                0x00224144 => {
                    c.cycles += 1;
                    c.sw(r(c, S1), c.g[V0][0]);
                    pc = 0x00224148;
                    continue;
                }
                0x00224148 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    pc = 0x0022414c;
                    continue;
                }
                0x0022414c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(r(c, S1).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224154;
                    continue;
                }
                0x00224154 => {
                    c.cycles += 15;
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V1][0] = sx(r(c, S2) >> 8);
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
                    c.mult(0, 3, 3, 4);
                    c.g[A2][0] = c.lw(r(c, S5));
                    c.g[A0][0] = c.lw(r(c, V0));
                    c.g[A1][0] = 0x4;
                    c.sw(r(c, SP).wrapping_add(0x58), c.g[A0][0]);
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x0022419c;
                        continue;
                    }
                    pc = 0x00224190;
                    continue;
                }
                0x00224190 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022419c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x0022419c)?;
                    pc = 0x0022419c;
                    continue;
                }
                0x0022419c => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = 0x3;
                    c.sw(r(c, S0), 0u64);
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x58));
                    if t {
                        pc = 0x00224208;
                        continue;
                    }
                    pc = 0x002241b0;
                    continue;
                }
                0x002241b0 => {
                    c.cycles += 15;
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V1][0] = sx(r(c, S6) << 2);
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.mult(0, 4, 4, 2);
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[A1][0] = 0x4;
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.sw(r(c, SP).wrapping_add(0x5c), c.g[V0][0]);
                    c.g[S0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x002241f8;
                        continue;
                    }
                    pc = 0x002241ec;
                    continue;
                }
                0x002241ec => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002241f8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002241f8)?;
                    pc = 0x002241f8;
                    continue;
                }
                0x002241f8 => {
                    c.cycles += 4;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S0), 0u64);
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x5c));
                    pc = 0x00224208;
                    continue;
                }
                0x00224208 => {
                    c.cycles += 3;
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V1][0]);
                    let t = true;
                    c.sw(r(c, S0), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224214;
                    continue;
                }
                0x00224214 => {
                    c.cycles += 18;
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x70));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00224234);
                    c.g[A2][0] = c.lw(r(c, V0));
                    call_ptr(c, 0x00226600, 0x00224234)?;
                    c.g[V1][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 2);
                    c.g[A1][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[A1][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x0022426c;
                        continue;
                    }
                    pc = 0x0022425c;
                    continue;
                }
                0x0022425c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00224268);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224268)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x0022426c;
                    continue;
                }
                0x0022426c => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x5;
                    c.sw(r(c, S0), 0u64);
                    c.ldl(3, r(c, SP).wrapping_add(0x7));
                    c.ldr(3, r(c, SP));
                    c.sdl(3, r(c, S0).wrapping_add(0x7));
                    c.sdr(3, r(c, S0));
                    let t = true;
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x0022428c;
                    continue;
                }
                0x0022428c => {
                    c.cycles += 21;
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[S3][0] = 0x2;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[S0][0] = sx(r(c, S2) >> 24);
                    c.g[FP][0] = c.lw(r(c, V0));
                    c.g[S1][0] = c.g[S0][0] & 0xff;
                    c.sw(r(c, SP).wrapping_add(0x8), c.g[S3][0]);
                    c.g[RA][0] = sx(0x002242b8);
                    c.g[S4][0] = c.g[S0][0];
                    call_ptr(c, 0x00226618, 0x002242b8)?;
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    c.g[V1][0] = 0xc;
                    c.sw(r(c, V0), 0u64);
                    c.mult(0, 17, 17, 3);
                    c.g[V0][0] = c.lw(r(c, S5));
                    c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x8));
                    c.g[S1][0] = sx(r(c, S1).wrapping_add(r(c, V0)));
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A1][0] = c.g[SP][0];
                    if t {
                        pc = 0x002242f0;
                        continue;
                    }
                    pc = 0x002242e0;
                    continue;
                }
                0x002242e0 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, S1));
                    c.g[V0][0] = c.lw(r(c, SP));
                    let t = true;
                    c.g[S1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if t {
                        pc = 0x00224384;
                        continue;
                    }
                    pc = 0x002242f0;
                    continue;
                }
                0x002242f0 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[A0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                        pc = 0x0022432c;
                        continue;
                    }
                    pc = 0x002242f8;
                    continue;
                }
                0x002242f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                        pc = 0x00224330;
                        continue;
                    }
                    pc = 0x00224304;
                    continue;
                }
                0x00224304 => {
                    c.cycles += 7;
                    c.lwc1(1, r(c, SP));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, S1));
                    abi::set_fcc(c, FpuF32(c.f[0]).lt(FpuF32(c.f[1])));
                    let t = c.fcc();
                    c.g[V1][0] = 0x1;
                    if t {
                        pc = 0x00224324;
                        continue;
                    }
                    pc = 0x00224320;
                    continue;
                }
                0x00224320 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0u64;
                    pc = 0x00224324;
                    continue;
                }
                0x00224324 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = c.g[V1][0];
                    if t {
                        pc = 0x00224384;
                        continue;
                    }
                    pc = 0x0022432c;
                    continue;
                }
                0x0022432c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00224330;
                    continue;
                }
                0x00224330 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00224360;
                        continue;
                    }
                    pc = 0x00224338;
                    continue;
                }
                0x00224338 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00224360;
                        continue;
                    }
                    pc = 0x00224348;
                    continue;
                }
                0x00224348 => {
                    c.cycles += 6;
                    c.lwc1(0, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, S1));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    let t = true;
                    c.g[S1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if t {
                        pc = 0x00224384;
                        continue;
                    }
                    pc = 0x00224360;
                    continue;
                }
                0x00224360 => {
                    c.cycles += 2;

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[S1][0] = 0u64;
                        pc = 0x00224384;
                        continue;
                    }
                    pc = 0x00224368;
                    continue;
                }
                0x00224368 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S1][0] = 0u64;
                        pc = 0x00224384;
                        continue;
                    }
                    pc = 0x00224378;
                    continue;
                }
                0x00224378 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r(c, S1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[S1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    pc = 0x00224384;
                    continue;
                }
                0x00224384 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00224390);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00225b90, 0x00224390)?;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = sx(r(c, S2) >> 16);
                    if t {
                        pc = 0x002244d8;
                        continue;
                    }
                    pc = 0x00224398;
                    continue;
                }
                0x00224398 => {
                    c.cycles += 14;
                    c.g[A1][0] = 0xc;
                    c.g[A2][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.mult(0, 7, 2, 5);
                    c.g[V1][0] = c.g[A2][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.g[T0][0] = c.g[A0][0];
                    c.g[A1][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
                    c.g[A3][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[A0][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));

                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = 0x2;
                        pc = 0x002243dc;
                        continue;
                    }
                    pc = 0x002243d0;
                    continue;
                }
                0x002243d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, A3));
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, A1));
                    if t {
                        pc = 0x0022443c;
                        continue;
                    }
                    pc = 0x002243dc;
                    continue;
                }
                0x002243dc => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(r(c, A3).wrapping_add(0x8));
                        pc = 0x00224410;
                        continue;
                    }
                    pc = 0x002243e4;
                    continue;
                }
                0x002243e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(r(c, A3).wrapping_add(0x8));
                        pc = 0x00224414;
                        continue;
                    }
                    pc = 0x002243f0;
                    continue;
                }
                0x002243f0 => {
                    c.cycles += 6;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, A3));
                    abi::set_fcc(c, FpuF32(c.f[1]).le(FpuF32(c.f[0])));
                    let t = c.fcc();

                    if t {
                        pc = 0x00224470;
                        continue;
                    }
                    pc = 0x00224408;
                    continue;
                }
                0x00224408 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00224470;
                        continue;
                    }
                    pc = 0x00224410;
                    continue;
                }
                0x00224410 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00224414;
                    continue;
                }
                0x00224414 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00224448;
                        continue;
                    }
                    pc = 0x0022441c;
                    continue;
                }
                0x0022441c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x00224448;
                        continue;
                    }
                    pc = 0x0022442c;
                    continue;
                }
                0x0022442c => {
                    c.cycles += 4;
                    c.lwc1(0, r(c, A1));
                    c.g[V0][0] = c.lw(r(c, A3));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(3, 1);
                    pc = 0x0022443c;
                    continue;
                }
                0x0022443c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    if t {
                        pc = 0x00224470;
                        continue;
                    }
                    pc = 0x00224448;
                    continue;
                }
                0x00224448 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00224470;
                        continue;
                    }
                    pc = 0x00224450;
                    continue;
                }
                0x00224450 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x00224470;
                        continue;
                    }
                    pc = 0x00224460;
                    continue;
                }
                0x00224460 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(r(c, A3));
                    c.g[V1][0] = c.lw(r(c, A1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    pc = 0x00224470;
                    continue;
                }
                0x00224470 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224478;
                    continue;
                }
                0x00224478 => {
                    c.cycles += 12;
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.g[A2][0] = 0xc;
                    c.g[S2][0] = 0x4;
                    c.mult(0, 3, 2, 6);
                    c.mult(0, 2, 20, 6);
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, T0)));
                    c.g[A2][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[RA][0] = sx(0x0022449c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00226628, 0x0022449c)?;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[S1][0] = sx(r(c, SP).wrapping_add(0x20));
                    if t {
                        pc = 0x002244b4;
                        continue;
                    }
                    pc = 0x002244a8;
                    continue;
                }
                0x002244a8 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002244b4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002244b4)?;
                    pc = 0x002244b4;
                    continue;
                }
                0x002244b4 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    let t = c.g[V1][0] == c.g[S2][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00224608;
                        continue;
                    }
                    pc = 0x002244c8;
                    continue;
                }
                0x002244c8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x20));
                    if t {
                        pc = 0x00224624;
                        continue;
                    }
                    pc = 0x002244d0;
                    continue;
                }
                0x002244d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(r(c, S0), c.g[V0][0]);
                    if t {
                        pc = 0x00224640;
                        continue;
                    }
                    pc = 0x002244d8;
                    continue;
                }
                0x002244d8 => {
                    c.cycles += 14;
                    c.g[A1][0] = 0xc;
                    c.g[A2][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.mult(0, 7, 2, 5);
                    c.g[V1][0] = c.g[A2][0] & 0xff;
                    c.mult(0, 3, 3, 5);
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.g[T0][0] = c.g[A0][0];
                    c.g[A1][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
                    c.g[A3][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[A0][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0x8));

                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = 0x2;
                        pc = 0x0022451c;
                        continue;
                    }
                    pc = 0x00224510;
                    continue;
                }
                0x00224510 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r(c, A3));
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, A1));
                    if t {
                        pc = 0x0022457c;
                        continue;
                    }
                    pc = 0x0022451c;
                    continue;
                }
                0x0022451c => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(r(c, A3).wrapping_add(0x8));
                        pc = 0x00224550;
                        continue;
                    }
                    pc = 0x00224524;
                    continue;
                }
                0x00224524 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(r(c, A3).wrapping_add(0x8));
                        pc = 0x00224554;
                        continue;
                    }
                    pc = 0x00224530;
                    continue;
                }
                0x00224530 => {
                    c.cycles += 6;
                    c.lwc1(1, r(c, A1));
                    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                    c.lwc1(0, r(c, A3));
                    abi::set_fcc(c, FpuF32(c.f[0]).le(FpuF32(c.f[1])));
                    let t = c.fcc();

                    if t {
                        pc = 0x002245b0;
                        continue;
                    }
                    pc = 0x00224548;
                    continue;
                }
                0x00224548 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002245b0;
                        continue;
                    }
                    pc = 0x00224550;
                    continue;
                }
                0x00224550 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00224554;
                    continue;
                }
                0x00224554 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00224588;
                        continue;
                    }
                    pc = 0x0022455c;
                    continue;
                }
                0x0022455c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x3;
                        pc = 0x00224588;
                        continue;
                    }
                    pc = 0x0022456c;
                    continue;
                }
                0x0022456c => {
                    c.cycles += 4;
                    c.lwc1(0, r(c, A1));
                    c.g[V1][0] = c.lw(r(c, A3));
                    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
                    c.mfc1(2, 1);
                    pc = 0x0022457c;
                    continue;
                }
                0x0022457c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    if t {
                        pc = 0x002245b0;
                        continue;
                    }
                    pc = 0x00224588;
                    continue;
                }
                0x00224588 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002245b0;
                        continue;
                    }
                    pc = 0x00224590;
                    continue;
                }
                0x00224590 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0u64;
                        pc = 0x002245b0;
                        continue;
                    }
                    pc = 0x002245a0;
                    continue;
                }
                0x002245a0 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, A3));
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    pc = 0x002245b0;
                    continue;
                }
                0x002245b0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x002245b8;
                    continue;
                }
                0x002245b8 => {
                    c.cycles += 12;
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.g[A2][0] = 0xc;
                    c.g[S2][0] = 0x4;
                    c.mult(0, 3, 2, 6);
                    c.mult(0, 2, 20, 6);
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, T0)));
                    c.g[A2][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[RA][0] = sx(0x002245dc);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00226628, 0x002245dc)?;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[S1][0] = sx(r(c, SP).wrapping_add(0x20));
                    if t {
                        pc = 0x002245f4;
                        continue;
                    }
                    pc = 0x002245e8;
                    continue;
                }
                0x002245e8 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002245f4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002245f4)?;
                    pc = 0x002245f4;
                    continue;
                }
                0x002245f4 => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S2][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x0022461c;
                        continue;
                    }
                    pc = 0x00224608;
                    continue;
                }
                0x00224608 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00224614);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00225068, 0x00224614)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    if t {
                        pc = 0x00224644;
                        continue;
                    }
                    pc = 0x0022461c;
                    continue;
                }
                0x0022461c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x20));
                    if t {
                        pc = 0x0022463c;
                        continue;
                    }
                    pc = 0x00224624;
                    continue;
                }
                0x00224624 => {
                    c.cycles += 6;
                    c.ldl(3, r(c, SP).wrapping_add(0x27));
                    c.ldr(3, r(c, SP).wrapping_add(0x20));
                    c.sdl(3, r(c, S0).wrapping_add(0x7));
                    c.sdr(3, r(c, S0));
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    if t {
                        pc = 0x00224644;
                        continue;
                    }
                    pc = 0x0022463c;
                    continue;
                }
                0x0022463c => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x00224640;
                    continue;
                }
                0x00224640 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x8));
                    pc = 0x00224644;
                    continue;
                }
                0x00224644 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.g[A1][0] = 0x2;
                    c.g[S6][0] = c.g[FP][0];
                    pc = 0x00224650;
                    continue;
                }
                0x00224650 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00224658);
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V0][0]);
                    call_ptr(c, 0x00225b90, 0x00224658)?;
                    let t = true;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    if t {
                        pc = 0x00224b30;
                        continue;
                    }
                    pc = 0x00224660;
                    continue;
                }
                0x00224660 => {
                    c.cycles += 20;
                    c.g[A1][0] = sx(r(c, S2) >> 24);
                    c.g[S1][0] = 0xc;
                    c.mult(0, 2, 5, 17);
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.g[A0][0] = sx(r(c, S2) >> 16);
                    c.g[S0][0] = sx(r(c, S2) >> 8);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[S0][0] = c.g[S0][0] & 0xff;
                    c.g[S3][0] = 0x4;
                    c.g[A1][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.mult(0, 2, 4, 17);
                    c.g[RA][0] = sx(0x00224694);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    call_ptr(c, 0x00225248, 0x00224694)?;
                    c.mult(0, 16, 16, 17);
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, S5));
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V1][0] != c.g[S3][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x002246c0;
                        continue;
                    }
                    pc = 0x002246b0;
                    continue;
                }
                0x002246b0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002246bc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002246bc)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x002246c0;
                    continue;
                }
                0x002246c0 => {
                    c.cycles += 4;
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    let t = c.g[V1][0] == c.g[S3][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x002247b0;
                        continue;
                    }
                    pc = 0x002246d0;
                    continue;
                }
                0x002246d0 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(r(c, S2));
                        pc = 0x002247e8;
                        continue;
                    }
                    pc = 0x002246d8;
                    continue;
                }
                0x002246d8 => {
                    c.cycles += 5;
                    c.ldl(3, r(c, S2).wrapping_add(0x7));
                    c.ldr(3, r(c, S2));
                    c.sdl(3, r(c, S0).wrapping_add(0x7));
                    let t = true;
                    c.sdr(3, r(c, S0));
                    if t {
                        pc = 0x002247ec;
                        continue;
                    }
                    pc = 0x002246ec;
                    continue;
                }
                0x002246ec => {
                    c.cycles += 20;
                    c.g[S0][0] = sx(r(c, S2) >> 8);
                    c.g[A1][0] = 0x480000;
                    c.g[S0][0] = c.g[S0][0] & 0xff;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6860));
                    c.g[A0][0] = 0x4;
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00224710);
                    c.g[S2][0] = 0x4;
                    call_ptr(c, 0x00317d70, 0x00224710)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0022471c);
                    c.g[A1][0] = 0x20;
                    call_ptr(c, 0x00224da0, 0x0022471c)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(r(c, S5));
                    c.g[V0][0] = 0xc;
                    c.mult(0, 16, 16, 2);
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));

                    if c.g[V0][0] != c.g[S2][0] {
                        c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                        pc = 0x0022474c;
                        continue;
                    }
                    pc = 0x0022473c;
                    continue;
                }
                0x0022473c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00224748);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224748)?;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    pc = 0x0022474c;
                    continue;
                }
                0x0022474c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.sw(r(c, S0), 0u64);
                    c.g[RA][0] = sx(0x0022475c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00225068, 0x0022475c)?;
                    let t = true;
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[S2][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224764;
                    continue;
                }
                0x00224764 => {
                    c.cycles += 11;
                    c.g[V1][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A0][0] = c.lw(r(c, S5));
                    c.mult(0, 3, 3, 2);
                    c.g[S1][0] = 0x4d0000;
                    c.g[S3][0] = 0x4;
                    c.g[S0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S3][0];
                    c.g[S2][0] = sx(r(c, S1).wrapping_sub(0x6f68));
                    if t {
                        pc = 0x0022479c;
                        continue;
                    }
                    pc = 0x00224790;
                    continue;
                }
                0x00224790 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022479c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x0022479c)?;
                    pc = 0x0022479c;
                    continue;
                }
                0x0022479c => {
                    c.cycles += 5;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S0), 0u64);
                    c.g[V1][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S3][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x002247c4;
                        continue;
                    }
                    pc = 0x002247b0;
                    continue;
                }
                0x002247b0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002247bc);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x00225068, 0x002247bc)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x002247f0;
                        continue;
                    }
                    pc = 0x002247c4;
                    continue;
                }
                0x002247c4 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_sub(0x6f68));
                    if t {
                        pc = 0x002247e8;
                        continue;
                    }
                    pc = 0x002247cc;
                    continue;
                }
                0x002247cc => {
                    c.cycles += 7;
                    c.g[A0][0] = sx(r(c, S1).wrapping_sub(0x6f68));
                    c.ldl(7, r(c, A0).wrapping_add(0x7));
                    c.ldr(7, r(c, A0));
                    c.sdl(7, r(c, S0).wrapping_add(0x7));
                    c.sdr(7, r(c, S0));
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x002247f0;
                        continue;
                    }
                    pc = 0x002247e8;
                    continue;
                }
                0x002247e8 => {
                    c.cycles += 1;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x002247ec;
                    continue;
                }
                0x002247ec => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x002247f0;
                    continue;
                }
                0x002247f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x002247f8;
                    continue;
                }
                0x002247f8 => {
                    c.cycles += 9;
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, S5));
                    c.mult(0, 2, 2, 3);
                    c.g[A0][0] = c.lw(r(c, S7).wrapping_add(0x8));
                    c.g[S1][0] = 0x4;
                    let t = c.g[A0][0] != c.g[S1][0];
                    c.g[S0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
                    if t {
                        pc = 0x00224828;
                        continue;
                    }
                    pc = 0x0022481c;
                    continue;
                }
                0x0022481c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x00224828);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224828)?;
                    pc = 0x00224828;
                    continue;
                }
                0x00224828 => {
                    c.cycles += 5;
                    c.sw(r(c, S7).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S7), 0u64);
                    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00224850;
                        continue;
                    }
                    pc = 0x0022483c;
                    continue;
                }
                0x0022483c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x00224848);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00225068, 0x00224848)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    if t {
                        pc = 0x00224b24;
                        continue;
                    }
                    pc = 0x00224850;
                    continue;
                }
                0x00224850 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(r(c, S0));
                        pc = 0x0022486c;
                        continue;
                    }
                    pc = 0x00224858;
                    continue;
                }
                0x00224858 => {
                    c.cycles += 5;
                    c.ldl(5, r(c, S0).wrapping_add(0x7));
                    c.ldr(5, r(c, S0));
                    c.sdl(5, r(c, S7).wrapping_add(0x7));
                    let t = true;
                    c.sdr(5, r(c, S7));
                    if t {
                        pc = 0x00224870;
                        continue;
                    }
                    pc = 0x0022486c;
                    continue;
                }
                0x0022486c => {
                    c.cycles += 1;
                    c.sw(r(c, S7), c.g[V0][0]);
                    pc = 0x00224870;
                    continue;
                }
                0x00224870 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    if t {
                        pc = 0x00224b24;
                        continue;
                    }
                    pc = 0x00224878;
                    continue;
                }
                // Opcode 0x25: argument-index byte followed by an i32 word.
                LOAD_INTEGER => {
                    c.cycles += 17;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[V1][0] = sx(r(c, A1) << 4);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, V0));
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[S2][0] = sx(r(c, S1).wrapping_add(0x4));
                    c.sw(r(c, SP).wrapping_add(0x60), c.g[A1][0]);
                    c.g[V1][0] = 0x4;
                    c.sw(r(c, S1), c.g[A0][0]);
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x002248c8;
                        continue;
                    }
                    pc = 0x002248bc;
                    continue;
                }
                0x002248bc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002248c8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002248c8)?;
                    pc = 0x002248c8;
                    continue;
                }
                0x002248c8 => {
                    c.cycles += 6;
                    c.sw(r(c, S2).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S1).wrapping_add(0x4), 0u64);
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x60));
                    if t {
                        pc = 0x002249bc;
                        continue;
                    }
                    pc = 0x002248e0;
                    continue;
                }
                // Opcode 0x26: the next word is an FPU float's raw bits.
                LOAD_FLOAT => {
                    c.cycles += 17;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[V1][0] = sx(r(c, A1) << 4);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, V0));
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[S2][0] = sx(r(c, S1).wrapping_add(0x4));
                    c.sw(r(c, SP).wrapping_add(0x64), c.g[A1][0]);
                    c.g[V1][0] = 0x4;
                    c.sw(r(c, S1), c.g[A0][0]);
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x00224930;
                        continue;
                    }
                    pc = 0x00224924;
                    continue;
                }
                0x00224924 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00224930);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224930)?;
                    pc = 0x00224930;
                    continue;
                }
                0x00224930 => {
                    c.cycles += 10;
                    c.sw(r(c, S2).wrapping_add(0x8), 0u64);
                    c.g[V0][0] = 0x2;
                    c.sw(r(c, S1).wrapping_add(0x4), 0u64);
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.lwc1(0, r(c, SP).wrapping_add(0x64));
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(0x1));
                    c.sw(r(c, SP).wrapping_add(0x80), c.g[A3][0]);
                    c.swc1(0, r(c, S1).wrapping_add(0x4));
                    let t = true;
                    c.sw(r(c, S2).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224958;
                    continue;
                }
                // Opcode 0x27: preserve the ID's full resource/section word.
                LOAD_RESOURCE_ID => {
                    c.cycles += 17;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    c.g[V0][0] = sx(r(c, S6) << 2);
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A0][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[V1][0] = sx(r(c, A1) << 4);
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.g[A1][0] = c.lw(r(c, V0));
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
                    c.g[S2][0] = sx(r(c, S1).wrapping_add(0x4));
                    c.sw(r(c, SP).wrapping_add(0x68), c.g[A1][0]);
                    c.g[V1][0] = 0x4;
                    c.sw(r(c, S1), c.g[A0][0]);
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[S6][0] = sx(r(c, S6).wrapping_add(0x1));
                    if t {
                        pc = 0x002249a8;
                        continue;
                    }
                    pc = 0x0022499c;
                    continue;
                }
                0x0022499c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002249a8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x002249a8)?;
                    pc = 0x002249a8;
                    continue;
                }
                0x002249a8 => {
                    c.cycles += 5;
                    c.sw(r(c, S2).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S1).wrapping_add(0x4), 0u64);
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x68));
                    pc = 0x002249bc;
                    continue;
                }
                0x002249bc => {
                    c.cycles += 5;
                    c.g[A3][0] = sx(r(c, A3).wrapping_add(0x1));
                    c.sw(r(c, SP).wrapping_add(0x80), c.g[A3][0]);
                    c.sw(r(c, S1).wrapping_add(0x4), c.g[V0][0]);
                    let t = true;
                    c.sw(r(c, S2).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x002249d0;
                    continue;
                }
                // Opcode 0x28: unsigned byte immediate embedded in the instruction.
                LOAD_SMALL_INTEGER => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[V1][0] = sx(r(c, V0) << 4);
                    c.g[V0][0] = sx(r(c, S2) >> 8);
                    c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[S0][0] = sx(r(c, S1).wrapping_add(0x4));
                    c.sw(r(c, S1), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, S2) >> 16);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[A0][0] = 0x4;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[A0][0];
                    c.sw(r(c, SP).wrapping_add(0x10), c.g[V1][0]);
                    if t {
                        pc = 0x00224a14;
                        continue;
                    }
                    pc = 0x00224a08;
                    continue;
                }
                0x00224a08 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00224a14);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224a14)?;
                    pc = 0x00224a14;
                    continue;
                }
                0x00224a14 => {
                    c.cycles += 10;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = 0x1;
                    c.sw(r(c, S1).wrapping_add(0x4), 0u64);
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x10));
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(0x1));
                    c.sw(r(c, SP).wrapping_add(0x80), c.g[A1][0]);
                    c.sw(r(c, S1).wrapping_add(0x4), c.g[V0][0]);
                    let t = true;
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224a3c;
                    continue;
                }
                // Opcode 0x29: unsigned byte immediate converted to float.
                LOAD_SMALL_FLOAT => {
                    c.cycles += 16;
                    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.g[A0][0] = sx(r(c, S2) >> 16);
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    c.g[V1][0] = sx(r(c, S2) >> 8);
                    c.g[V0][0] = sx(r(c, A2) << 4);
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[S1][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    c.g[A0][0] = c.g[A0][0] & 0xff;
                    c.sw(r(c, S1), c.g[V1][0]);
                    c.mtc1(4, 0);
                    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                    c.g[S0][0] = sx(r(c, S1).wrapping_add(0x4));
                    c.g[V1][0] = 0x4;
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.swc1(0, r(c, SP).wrapping_add(0x6c));
                    if t {
                        pc = 0x00224a88;
                        continue;
                    }
                    pc = 0x00224a7c;
                    continue;
                }
                0x00224a7c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00224a88);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224a88)?;
                    pc = 0x00224a88;
                    continue;
                }
                0x00224a88 => {
                    c.cycles += 10;
                    c.sw(r(c, S0).wrapping_add(0x8), 0u64);
                    c.g[V0][0] = 0x2;
                    c.sw(r(c, S1).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x80));
                    c.lwc1(0, r(c, SP).wrapping_add(0x6c));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(0x1));
                    c.sw(r(c, SP).wrapping_add(0x80), c.g[V1][0]);
                    c.swc1(0, r(c, S1).wrapping_add(0x4));
                    let t = true;
                    c.sw(r(c, S0).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x00224b2c;
                        continue;
                    }
                    pc = 0x00224ab0;
                    continue;
                }
                0x00224ab0 => {
                    c.cycles += 5;
                    c.g[S0][0] = 0x4d0000;
                    c.g[V0][0] = c.lw(r(c, S7).wrapping_add(0x8));
                    c.g[S1][0] = 0x4;
                    let t = c.g[V0][0] != c.g[S1][0];
                    c.g[S2][0] = sx(r(c, S0).wrapping_sub(0x6f68));
                    if t {
                        pc = 0x00224ad0;
                        continue;
                    }
                    pc = 0x00224ac4;
                    continue;
                }
                0x00224ac4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x00224ad0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00224df0, 0x00224ad0)?;
                    pc = 0x00224ad0;
                    continue;
                }
                0x00224ad0 => {
                    c.cycles += 5;
                    c.sw(r(c, S7).wrapping_add(0x8), 0u64);
                    c.sw(r(c, S7), 0u64);
                    c.g[V1][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    let t = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00224af8;
                        continue;
                    }
                    pc = 0x00224ae4;
                    continue;
                }
                0x00224ae4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x00224af0);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x00225068, 0x00224af0)?;
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00224b24;
                        continue;
                    }
                    pc = 0x00224af8;
                    continue;
                }
                0x00224af8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_sub(0x6f68));
                    if t {
                        pc = 0x00224b1c;
                        continue;
                    }
                    pc = 0x00224b00;
                    continue;
                }
                0x00224b00 => {
                    c.cycles += 7;
                    c.g[A3][0] = sx(r(c, S0).wrapping_sub(0x6f68));
                    c.ldl(4, r(c, A3).wrapping_add(0x7));
                    c.ldr(4, r(c, A3));
                    c.sdl(4, r(c, S7).wrapping_add(0x7));
                    c.sdr(4, r(c, S7));
                    let t = true;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    if t {
                        pc = 0x00224b24;
                        continue;
                    }
                    pc = 0x00224b1c;
                    continue;
                }
                0x00224b1c => {
                    c.cycles += 2;
                    c.sw(r(c, S7), c.g[V0][0]);
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x8));
                    pc = 0x00224b24;
                    continue;
                }
                0x00224b24 => {
                    c.cycles += 2;
                    c.g[S6][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    c.sw(r(c, S7).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x00224b2c;
                    continue;
                }
                NEXT_INSTRUCTION => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(r(c, SP).wrapping_add(0x78));
                    pc = 0x00224b30;
                    continue;
                }
                0x00224b30 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x74));
                    if t {
                        if back_edge_unwinds(c, 0x002228b8) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x002228b8;
                        continue;
                    }
                    pc = 0x00224b3c;
                    continue;
                }
                0x00224b3c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    pc = 0x00224b40;
                    continue;
                }
                0x00224b40 => {
                    c.cycles += 2;

                    if c.g[A0][0] == 0u64 {
                        c.g[A1][0] = c.lw(r(c, S5));
                        pc = 0x00224b84;
                        continue;
                    }
                    pc = 0x00224b48;
                    continue;
                }
                0x00224b48 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r(c, A0).wrapping_sub(0x10));
                    c.g[V0][0] = sx(r(c, V0) << 4);
                    c.g[S0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                    let t = c.g[A0][0] == c.g[S0][0];
                    c.g[S1][0] = sx(r(c, A0).wrapping_sub(0x10));
                    if t {
                        pc = 0x00224b78;
                        continue;
                    }
                    pc = 0x00224b5c;
                    continue;
                }
                0x00224b5c => {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r(c, S0).wrapping_sub(0x10));
                    pc = 0x00224b60;
                    continue;
                }
                0x00224b60 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x00224b6c);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(0x4));
                    call_ptr(c, 0x00225b90, 0x00224b6c)?;
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x7c));
                    let t = c.g[A1][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r(c, S0).wrapping_sub(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x00224b60) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x00224b60;
                        continue;
                    }
                    pc = 0x00224b78;
                    continue;
                }
                0x00224b78 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00224b80);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00317e98, 0x00224b80)?;
                    c.g[A1][0] = c.lw(r(c, S5));
                    pc = 0x00224b84;
                    continue;
                }
                0x00224b84 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V0][0] = 0xc;
                    if t {
                        pc = 0x00224bcc;
                        continue;
                    }
                    pc = 0x00224b8c;
                    continue;
                }
                0x00224b8c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(r(c, A1).wrapping_sub(0x10));
                    c.mult(0, 4, 3, 2);
                    c.g[S0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));

                    if c.g[A1][0] == c.g[S0][0] {
                        c.g[A0][0] = c.lw(r(c, S5));
                        pc = 0x00224bc4;
                        continue;
                    }
                    pc = 0x00224ba0;
                    continue;
                }
                0x00224ba0 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx(r(c, S0).wrapping_sub(0xc));
                    pc = 0x00224ba8;
                    continue;
                }
                0x00224ba8 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00224bb4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00225b90, 0x00224bb4)?;
                    c.g[V0][0] = c.lw(r(c, S5));
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r(c, S0).wrapping_sub(0xc));
                    if t {
                        if back_edge_unwinds(c, 0x00224ba8) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x00224ba8;
                        continue;
                    }
                    pc = 0x00224bc0;
                    continue;
                }
                0x00224bc0 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(r(c, S5));
                    pc = 0x00224bc4;
                    continue;
                }
                0x00224bc4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00224bcc);
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x10));
                    call_ptr(c, 0x00317e98, 0x00224bcc)?;
                    pc = 0x00224bcc;
                    continue;
                }
                0x00224bcc => {
                    c.cycles += 13;
                    frame.restore(c, 16, 0x120);
                    frame.restore(c, 17, 0x110);
                    frame.restore(c, 18, 0x100);
                    frame.restore(c, 19, 0xf0);
                    frame.restore(c, 20, 0xe0);
                    frame.restore(c, 21, 0xd0);
                    frame.restore(c, 22, 0xc0);
                    frame.restore(c, 23, 0xb0);
                    frame.restore(c, 30, 0xa0);
                    frame.restore_ra(c, SAVED_RA);
                    frame.restore_fpr(c, 20, 0x130);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002227d0, 0x00224c00);
                    return Ok(());
                }
            }
        }
    });
}

/// Optional bounded asset diagnosis during a normal windowed session.
fn trace_entry(c: &mut Cpu) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static ENTRIES: AtomicUsize = AtomicUsize::new(0);
    if std::env::var_os("SSX3_LUNO_TRACE").is_none() {
        return;
    }
    if ENTRIES.fetch_add(1, Ordering::Relaxed) >= 10 {
        return;
    }
    let function = r(c, A1);
    let blob = c.bus.read32(function);
    let index = c.bus.read32(function.wrapping_add(4));
    let header: [u32; 4] = std::array::from_fn(|i| c.bus.read32(blob.wrapping_add(i as u32 * 4)));
    let descriptor = blob
        .wrapping_add(header[1])
        .wrapping_add(index.wrapping_mul(16));
    let fields: [u32; 4] =
        std::array::from_fn(|i| c.bus.read32(descriptor.wrapping_add(i as u32 * 4)));
    eprintln!("[luno] ref={function:08x} blob={blob:08x} index={index} header={header:08x?} descriptor={fields:08x?} ra={:08x}", r(c, RA));
}
