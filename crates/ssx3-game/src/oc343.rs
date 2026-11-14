//! Staging module oc343 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc343`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0010_8fa8,
    computer_actor_rider_dtor,
    "ComputerActorRider_dtor",
)];

// -- ComputerActorRider_dtor ----------------------------------------------
//
// Register-level helpers: the destructor is gcc's virtual-inheritance
// boilerplate whose every register value can be seen by an event at one of
// its 42 loop heads (it is re-entered there), so it is written as the
// sequence of register operations the compiled code performs.

/// `rd = imm` (lui/li).
#[inline(always)]
fn cad_li(c: &mut Cpu, rd: usize, v: u64) {
    c.g[rd][0] = v;
}

/// `rd = rs` (move).
#[inline(always)]
fn cad_mov(c: &mut Cpu, rd: usize, rs: usize) {
    c.g[rd][0] = c.g[rs][0];
}

/// `rd = sx(rs + imm)` (addiu).
#[inline(always)]
fn cad_addiu(c: &mut Cpu, rd: usize, rs: usize, imm: i32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(imm as u32));
}

/// `rd = sx(rs + rt)` (addu).
#[inline(always)]
fn cad_addu(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(c.g[rt][0] as u32));
}

/// `rd = sx(rs - rt)` (subu).
#[inline(always)]
fn cad_subu(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_sub(c.g[rt][0] as u32));
}

/// `rd = rs & imm` (andi).
#[inline(always)]
fn cad_andi(c: &mut Cpu, rd: usize, rs: usize, imm: u64) {
    c.g[rd][0] = c.g[rs][0] & imm;
}

/// The address `off(base)`.
#[inline(always)]
fn cad_ea(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// `lw rd, off(base)` (sign-extended).
#[inline(always)]
fn cad_lw(c: &mut Cpu, rd: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    c.g[rd][0] = c.lw(a);
}

/// `ld rd, off(base)`.
#[inline(always)]
fn cad_ld(c: &mut Cpu, rd: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    c.g[rd][0] = c.ld(a);
}

/// `lhu rd, off(base)`.
#[inline(always)]
fn cad_lhu(c: &mut Cpu, rd: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    c.g[rd][0] = c.lhu(a);
}

/// `sw rt, off(base)`.
#[inline(always)]
fn cad_sw(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    let v = c.g[rt][0];
    c.sw(a, v);
}

/// `sd rt, off(base)`.
#[inline(always)]
fn cad_sd(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    let v = c.g[rt][0];
    c.sd(a, v);
}

/// `sh rt, off(base)`.
#[inline(always)]
fn cad_sh(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    let v = c.g[rt][0];
    c.sh(a, v);
}

/// `sq rt, off(base)` (all 128 bits).
#[inline(always)]
fn cad_sq(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    c.sq(rt, a);
}

/// `lq rt, off(base)`.
#[inline(always)]
fn cad_lq(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    let a = cad_ea(c, base, off);
    c.lq(rt, a);
}

/// The registers of one vtable copy loop: 32 bytes per pass from `src` to
/// `dst` through the four `tmp` registers, until `src` reaches `end`.
#[derive(Clone, Copy)]
struct CadCopy {
    src: usize,
    dst: usize,
    end: usize,
    tmp: [usize; 4],
}

/// One vtable copy loop at `head` (11 cycles a pass, an event check at the
/// back edge). True when an event unwound the thread: the caller returns at
/// once, and the trampoline re-enters the destructor at `head` later.
fn cad_copy(c: &mut Cpu, head: u32, k: CadCopy) -> bool {
    loop {
        c.cycles += 11;
        for (i, &t) in k.tmp.iter().enumerate() {
            cad_ld(c, t, k.src, 8 * i as u32);
        }
        for (i, &t) in k.tmp.iter().enumerate() {
            cad_sd(c, t, k.dst, 8 * i as u32);
        }
        cad_addiu(c, k.src, k.src, 0x20);
        let more = c.g[k.src][0] != c.g[k.end][0];
        cad_addiu(c, k.dst, k.dst, 0x20);
        if !more {
            return false;
        }
        if back_edge_unwinds(c, head) {
            return true;
        }
    }
}

/// `Rider_dtor`, called directly.
const CAD_RIDER_DTOR: Callee = Callee {
    addr: 0x0011_b978,
    f: ee::s00::f_0011b978,
};

/// `ComputerActorRider::~ComputerActorRider(this, in_chrg)` (0x108fa8,
/// 0x400-byte frame): gcc's destructor for a class with a virtual `Rider`
/// base and eleven effect members that share that base.
///
/// For the object itself and then for each subobject (its virtual-base
/// pointer at `this + 0x3a0, 0x398, 0x384, 0x364, 0x358, 0x340, 0x2b8, 0x2a4,
/// 0x288, 0x224, 0x200, 0x1f0, 0x1e4, 0x1c0, 0x1b0, 0x100, 0xa0, 0x24, 0x18`)
/// it stores the in-destructor vtables into the virtual base (`+0x6e8`,
/// `+0x6d0` = 0x459b90, `+0x6c0`), copies the two construction vtables (0x48
/// and 0xb0 bytes, from 0x4585f0-0x459b90) onto the stack, moves their
/// this-adjust halfword (`+8`) by the subobject's offset inside the virtual
/// base and installs the stack copies. Between the object and the subobjects
/// it destroys the members (`RiderBase_DtorCheatCode` .. `cBoardWakeFX_dtor`,
/// each `(member, 0)`), before the last two the wipeout state
/// (`RiderWipeoutState_dtor(this + 0x30, 0)`). Then `in_chrg & 2` destroys
/// the virtual base (`Rider_dtor(*(this + 0x18), 0)`) and `in_chrg & 1`
/// frees the object (`__builtin_delete`).
///
/// Each copy loop checks events at its back edge and the destructor is
/// re-entered at that loop head after an unwind, so the body dispatches on
/// `pc` like the original and keeps all its state in the registers.
pub fn computer_actor_rider_dtor(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            // Prologue. `this->vptr` (+0xde8) and the Rider virtual base
            // (`*(this + 0x18)`) get the in-destructor vtables.
            0x0010_8fa8 => {
                c.cycles += 37;
                cad_addiu(c, SP, SP, -0x400);
                cad_li(c, V0, 0x0046_0000);
                cad_sq(c, S0, SP, 0x3f0);
                cad_li(c, V1, 0x0046_0000);
                cad_sq(c, S1, SP, 0x3e0);
                cad_mov(c, S0, A0);
                cad_sq(c, S2, SP, 0x3d0);
                cad_addiu(c, V0, V0, -0x7a10);
                cad_sq(c, S3, SP, 0x3c0);
                cad_addiu(c, A3, V1, -0x79e8);
                cad_sq(c, S4, SP, 0x3b0);
                cad_mov(c, T0, V1);
                cad_sq(c, S5, SP, 0x3a0);
                cad_li(c, V1, 0x0046_0000);
                cad_sq(c, S6, SP, 0x390);
                cad_mov(c, S2, V1);
                cad_sq(c, S7, SP, 0x380);
                cad_addiu(c, V1, V1, -0x79a0);
                cad_sq(c, FP, SP, 0x370);
                cad_mov(c, T4, A1);
                cad_sd(c, RA, SP, 0x360);
                cad_li(c, T6, 0x0046_0000);
                cad_sw(c, V0, S0, 0xde8);
                cad_li(c, T7, 0x0046_0000);
                cad_lw(c, A0, S0, 0x18);
                cad_li(c, V0, 0x0046_0000);
                cad_addiu(c, V0, V0, -0x6470);
                cad_li(c, A2, 0x0046_0000);
                cad_sw(c, A3, A0, 0x6e8);
                cad_addiu(c, T2, SP, 0x100);
                cad_addiu(c, T3, SP, 0x150);
                cad_addiu(c, S1, SP, 0x50);
                cad_lw(c, A0, S0, 0x18);
                cad_sw(c, V0, A0, 0x6d0);
                cad_lw(c, V0, S0, 0x18);
                let t = c.g[T4][0] != 0;
                cad_sw(c, V1, V0, 0x6c0);
                if t {
                    pc = 0x0010_9114;
                    continue;
                }
                pc = 0x0010_903c;
            }
            // Not in charge (in_chrg == 0): the Rider base gets construction
            // vtables built on the stack (0x48 bytes at sp+0x200, 0xb0 at sp+0x250),
            // their this-adjust halfwords moved by `this - (vbase - 0xf50)`.
            0x0010_903c => {
                c.cycles += 5;
                cad_mov(c, V1, A3);
                cad_addiu(c, V0, SP, 0x200);
                cad_addiu(c, A0, V1, 0x40);
                cad_mov(c, A3, V0);
                cad_addiu(c, T5, SP, 0x250);
                pc = 0x0010_9050;
            }
            0x0010_9050 => {
                if cad_copy(
                    c,
                    0x0010_9050,
                    CadCopy {
                        src: V1,
                        dst: V0,
                        end: A0,
                        tmp: [A1, T1, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_907c;
            }
            0x0010_907c => {
                c.cycles += 14;
                cad_ld(c, A0, V1, 0);
                cad_sd(c, A0, V0, 0);
                cad_lw(c, A0, S0, 0x18);
                cad_addiu(c, V0, T0, -0x79e8);
                cad_lhu(c, V1, V0, 0x8);
                cad_addiu(c, T0, S2, -0x79a0);
                cad_sw(c, A3, A0, 0x6e8);
                cad_addiu(c, T1, T0, 0xa0);
                cad_mov(c, A1, T5);
                cad_lw(c, V0, S0, 0x18);
                cad_addiu(c, V0, V0, -0xf50);
                cad_subu(c, A3, S0, V0);
                cad_addu(c, V1, V1, A3);
                cad_sh(c, V1, SP, 0x208);
                pc = 0x0010_90b4;
            }
            0x0010_90b4 => {
                if cad_copy(
                    c,
                    0x0010_90b4,
                    CadCopy {
                        src: T0,
                        dst: A1,
                        end: T1,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_90e0;
            }
            0x0010_90e0 => {
                c.cycles += 13;
                cad_ld(c, T1, T0, 0);
                cad_ld(c, S3, T0, 0x8);
                cad_sd(c, T1, A1, 0);
                cad_addiu(c, A0, S2, -0x79a0);
                cad_sd(c, S3, A1, 0x8);
                cad_lw(c, A1, S0, 0x18);
                cad_lhu(c, V1, A0, 0xa0);
                cad_lhu(c, V0, A0, 0x8);
                cad_sw(c, T5, A1, 0x6c0);
                cad_addu(c, V1, V1, A3);
                cad_addu(c, V0, V0, A3);
                cad_sh(c, V1, SP, 0x2f0);
                cad_sh(c, V0, SP, 0x258);
                pc = 0x0010_9114;
            }
            // The ComputerActorRider level is done: `this->vptr` = 0x458710, the
            // Rider base gets 0x458738 / 0x459b90 / 0x458780; spill the member
            // addresses and the in_chrg bits; stack copies at sp+0x100 / sp+0x150.
            0x0010_9114 => {
                c.cycles += 59;
                cad_lw(c, A0, S0, 0x18);
                cad_addiu(c, V0, T6, -0x78f0);
                cad_sw(c, V0, S0, 0xde8);
                cad_addiu(c, V1, T7, -0x78c8);
                cad_sw(c, V1, A0, 0x6e8);
                cad_andi(c, A3, T4, 0x1);
                cad_li(c, A0, 0x0046_0000);
                cad_addiu(c, T1, S0, 0xc70);
                cad_lw(c, V0, S0, 0x18);
                cad_addiu(c, A1, A0, -0x6470);
                cad_sw(c, A3, SP, 0x320);
                cad_addiu(c, S3, S0, 0xb40);
                cad_sw(c, A1, V0, 0x6d0);
                cad_addiu(c, S4, S0, 0xb00);
                cad_sw(c, T1, SP, 0x304);
                cad_addiu(c, A1, S0, 0xaf0);
                cad_sw(c, S3, SP, 0x348);
                cad_addiu(c, T1, S0, 0xad0);
                cad_addiu(c, A0, A2, -0x7880);
                cad_addiu(c, S3, S0, 0x9c0);
                cad_lw(c, V0, S0, 0x18);
                cad_andi(c, T4, T4, 0x2);
                cad_sw(c, S4, SP, 0x340);
                cad_addiu(c, T0, V1, 0x40);
                cad_sw(c, A1, SP, 0x33c);
                cad_addiu(c, S4, S0, 0x610);
                cad_sw(c, T1, SP, 0x32c);
                cad_addiu(c, A1, S0, 0x3b0);
                cad_sw(c, A0, V0, 0x6c0);
                cad_addiu(c, T1, S0, 0x290);
                cad_addiu(c, V0, S0, 0x520);
                cad_addiu(c, A0, S0, 0x470);
                cad_sw(c, V0, SP, 0x354);
                cad_mov(c, A3, T2);
                cad_sw(c, A0, SP, 0x334);
                cad_addiu(c, V0, S0, 0x230);
                cad_sw(c, A1, SP, 0x300);
                cad_addiu(c, A0, S0, 0x210);
                cad_addiu(c, A1, S0, 0x200);
                cad_sw(c, T1, SP, 0x344);
                cad_sw(c, V0, SP, 0x324);
                cad_addiu(c, T1, S0, 0x1f0);
                cad_sw(c, A0, SP, 0x31c);
                cad_addiu(c, V0, S0, 0x1d0);
                cad_sw(c, A1, SP, 0x318);
                cad_addiu(c, A0, S0, 0x1c0);
                cad_addiu(c, A1, S0, 0x110);
                cad_sw(c, T1, SP, 0x314);
                cad_sw(c, V0, SP, 0x310);
                cad_addiu(c, T1, S0, 0xb0);
                cad_sw(c, A0, SP, 0x308);
                cad_addiu(c, V0, S0, 0xa0);
                cad_sw(c, A1, SP, 0x358);
                cad_addiu(c, A0, S0, 0x30);
                cad_addiu(c, A1, S0, 0x20);
                cad_sw(c, S3, SP, 0x35c);
                cad_sw(c, S4, SP, 0x30c);
                cad_addiu(c, T5, S0, 0xd20);
                cad_sw(c, T1, SP, 0x350);
                pc = 0x0010_9200;
            }
            0x0010_9200 => {
                c.cycles += 11;
                cad_addiu(c, S6, S0, 0x3a0);
                cad_sw(c, V0, SP, 0x34c);
                cad_addiu(c, S2, S0, 0x390);
                cad_sw(c, A0, SP, 0x338);
                cad_addiu(c, S3, S0, 0x370);
                cad_sw(c, A1, SP, 0x330);
                cad_addiu(c, S4, S0, 0x360);
                cad_sw(c, T4, SP, 0x328);
                cad_addiu(c, S5, S0, 0x350);
                cad_addiu(c, S7, S0, 0x2c0);
                cad_addiu(c, FP, S0, 0x2b0);
                pc = 0x0010_922c;
            }
            0x0010_922c => {
                if cad_copy(
                    c,
                    0x0010_922c,
                    CadCopy {
                        src: V1,
                        dst: A3,
                        end: T0,
                        tmp: [T1, V0, A0, A1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9258;
            }
            0x0010_9258 => {
                c.cycles += 14;
                cad_ld(c, T0, V1, 0);
                cad_mov(c, A1, T3);
                cad_sd(c, T0, A3, 0);
                cad_lw(c, V1, S0, 0x18);
                cad_addiu(c, A0, T7, -0x78c8);
                cad_addiu(c, A3, A2, -0x7880);
                cad_sw(c, T2, V1, 0x6e8);
                cad_addiu(c, T1, A3, 0xa0);
                cad_lw(c, V0, S0, 0x18);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0xdf0);
                cad_subu(c, T0, S0, V0);
                cad_addu(c, V1, V1, T0);
                cad_sh(c, V1, SP, 0x108);
                pc = 0x0010_9290;
            }
            0x0010_9290 => {
                if cad_copy(
                    c,
                    0x0010_9290,
                    CadCopy {
                        src: A3,
                        dst: A1,
                        end: T1,
                        tmp: [T2, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_92bc;
            }
            // Finish the Rider base; destroy the members (not in charge), then
            // the virtual base at `*(this + 0x3a0)` gets 0x0045_8928 / 0x459b90 / 0x0045_8970
            // (stack copies at sp+0 / sp+0x50, reused by every subobject below).
            0x0010_92bc => {
                c.cycles += 58;
                cad_ld(c, T1, A3, 0);
                cad_ld(c, T2, A3, 0x8);
                cad_sd(c, T1, A1, 0);
                cad_addiu(c, A2, A2, -0x7880);
                cad_sd(c, T2, A1, 0x8);
                cad_mov(c, A0, T5);
                cad_lw(c, V1, S0, 0x18);
                cad_li(c, A1, 0);
                cad_sw(c, T3, V1, 0x6c0);
                cad_lhu(c, V0, A2, 0x8);
                cad_addu(c, V0, V0, T0);
                cad_sh(c, V0, SP, 0x158);
                if call_ptr(c, 0x002f_6388, 0x0010_92f0).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x304);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002f_0fe8, 0x0010_92fc).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x348);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002d_f1c8, 0x0010_9308).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x340);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002e_37d0, 0x0010_9314).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x33c);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002d_4a70, 0x0010_9320).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x32c);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002e_f530, 0x0010_932c).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x35c);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002e_ac60, 0x0010_9338).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x30c);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002e_64c0, 0x0010_9344).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x354);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002e_83f0, 0x0010_9350).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x334);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002d_a8e8, 0x0010_935c).is_err() {
                    return;
                }
                cad_lw(c, A0, SP, 0x300);
                cad_li(c, A1, 0);
                if call_ptr(c, 0x002d_cda0, 0x0010_9368).is_err() {
                    return;
                }
                cad_lw(c, V1, S0, 0x3a0);
                cad_li(c, A0, 0x0046_0000);
                cad_addiu(c, V0, A0, -0x76d8);
                cad_li(c, A1, 0x0046_0000);
                cad_sw(c, V0, V1, 0x6e8);
                cad_mov(c, A3, V0);
                cad_addiu(c, A0, A1, -0x6470);
                cad_li(c, A2, 0x0046_0000);
                cad_lw(c, V0, S0, 0x3a0);
                cad_addiu(c, A1, A2, -0x7690);
                cad_addiu(c, T0, A3, 0x40);
                cad_mov(c, A2, SP);
                cad_sw(c, A0, V0, 0x6d0);
                cad_lw(c, V1, S0, 0x3a0);
                cad_sw(c, A1, V1, 0x6c0);
                pc = 0x0010_93a4;
            }
            0x0010_93a4 => {
                if cad_copy(
                    c,
                    0x0010_93a4,
                    CadCopy {
                        src: A3,
                        dst: A2,
                        end: T0,
                        tmp: [T1, T2, V0, V1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_93d0;
            }
            0x0010_93d0 => {
                c.cycles += 16;
                cad_lw(c, V0, S6, 0);
                cad_li(c, A1, 0x0046_0000);
                cad_ld(c, T0, A3, 0);
                cad_addiu(c, A0, A1, -0x76d8);
                cad_sd(c, T0, A2, 0);
                cad_li(c, A1, 0x0046_0000);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, A1, -0x7690);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S6, 0);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, S6, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9410;
            }
            0x0010_9410 => {
                if cad_copy(
                    c,
                    0x0010_9410,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T0, T1, V0, V1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_943c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x398)`
            // gets 0x0045_8d08 / 0x459b90 / 0x0045_8d50.
            0x0010_943c => {
                c.cycles += 26;
                cad_ld(c, A0, A2, 0);
                cad_ld(c, A3, A2, 0x8);
                cad_sd(c, A0, A1, 0);
                cad_li(c, V0, 0x0046_0000);
                cad_sd(c, A3, A1, 0x8);
                cad_li(c, V1, 0x0046_0000);
                cad_addiu(c, A1, V0, -0x7690);
                cad_addiu(c, A0, V1, -0x72f8);
                cad_lw(c, V0, S6, 0);
                cad_li(c, A3, 0x0046_0000);
                cad_addiu(c, A2, A3, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x72b0);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_lhu(c, V0, A1, 0x8);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, V1, S2, 0x8);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S2, 0x8);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S2, 0x8);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_94a4;
            }
            0x0010_94a4 => {
                if cad_copy(
                    c,
                    0x0010_94a4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_94d0;
            }
            0x0010_94d0 => {
                c.cycles += 16;
                cad_lw(c, V0, S2, 0x8);
                cad_li(c, A1, 0x0046_0000);
                cad_ld(c, A2, T1, 0);
                cad_addiu(c, A0, A1, -0x72f8);
                cad_sd(c, A2, T0, 0);
                cad_li(c, A1, 0x0046_0000);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, A1, -0x72b0);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S2, 0x8);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, S2, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9510;
            }
            0x0010_9510 => {
                if cad_copy(
                    c,
                    0x0010_9510,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T0, T1, V0, V1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_953c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x384)`
            // gets 0x0045_8830 / 0x459b90 / 0x0045_8878.
            0x0010_953c => {
                c.cycles += 26;
                cad_ld(c, A0, A2, 0);
                cad_ld(c, A3, A2, 0x8);
                cad_sd(c, A0, A1, 0);
                cad_li(c, V0, 0x0046_0000);
                cad_sd(c, A3, A1, 0x8);
                cad_li(c, V1, 0x0046_0000);
                cad_addiu(c, A1, V0, -0x72b0);
                cad_addiu(c, A0, V1, -0x77d0);
                cad_lw(c, V0, S2, 0x8);
                cad_li(c, A3, 0x0046_0000);
                cad_addiu(c, A2, A3, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x7788);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_lhu(c, V0, A1, 0x8);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, V1, S3, 0x14);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S3, 0x14);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x14);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_95a4;
            }
            0x0010_95a4 => {
                if cad_copy(
                    c,
                    0x0010_95a4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_95d0;
            }
            0x0010_95d0 => {
                c.cycles += 16;
                cad_lw(c, V0, S3, 0x14);
                cad_li(c, A1, 0x0046_0000);
                cad_ld(c, A2, T1, 0);
                cad_addiu(c, A0, A1, -0x77d0);
                cad_sd(c, A2, T0, 0);
                cad_li(c, A1, 0x0046_0000);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, A1, -0x7788);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S3, 0x14);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x20);
                cad_subu(c, T2, S3, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9610;
            }
            0x0010_9610 => {
                if cad_copy(
                    c,
                    0x0010_9610,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T0, T1, V0, V1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_963c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x364)`
            // gets 0x0045_8b18 / 0x459b90 / 0x0045_8b60.
            0x0010_963c => {
                c.cycles += 26;
                cad_ld(c, A0, A2, 0);
                cad_ld(c, A3, A2, 0x8);
                cad_sd(c, A0, A1, 0);
                cad_li(c, V0, 0x0046_0000);
                cad_sd(c, A3, A1, 0x8);
                cad_li(c, V1, 0x0046_0000);
                cad_addiu(c, A1, V0, -0x7788);
                cad_addiu(c, A0, V1, -0x74e8);
                cad_lw(c, V0, S3, 0x14);
                cad_li(c, A3, 0x0046_0000);
                cad_addiu(c, A2, A3, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x74a0);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_lhu(c, V0, A1, 0x8);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, V1, S4, 0x4);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S4, 0x4);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S4, 0x4);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_96a4;
            }
            0x0010_96a4 => {
                if cad_copy(
                    c,
                    0x0010_96a4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, S3, V0, V1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_96d0;
            }
            0x0010_96d0 => {
                c.cycles += 16;
                cad_lw(c, V0, S4, 0x4);
                cad_li(c, A1, 0x0046_0000);
                cad_ld(c, A2, T1, 0);
                cad_li(c, V1, 0x0046_0000);
                cad_sd(c, A2, T0, 0);
                cad_addiu(c, A0, A1, -0x74e8);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, V1, -0x74a0);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S4, 0x4);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, S4, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9710;
            }
            0x0010_9710 => {
                if cad_copy(
                    c,
                    0x0010_9710,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [A0, T0, T1, S3],
                    },
                ) {
                    return;
                }
                pc = 0x0010_973c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x358)`
            // gets 0x0045_8a20 / 0x459b90 / 0x0045_8a68.
            0x0010_973c => {
                c.cycles += 26;
                cad_ld(c, V0, A2, 0);
                cad_ld(c, V1, A2, 0x8);
                cad_sd(c, V0, A1, 0);
                cad_li(c, T0, 0x0046_0000);
                cad_sd(c, V1, A1, 0x8);
                cad_li(c, T1, 0x0046_0000);
                cad_lw(c, V0, S4, 0x4);
                cad_addiu(c, A1, T0, -0x74a0);
                cad_addiu(c, A0, T1, -0x75e0);
                cad_li(c, S3, 0x0046_0000);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A2, S3, -0x6470);
                cad_li(c, S4, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_lhu(c, V0, A1, 0x8);
                cad_addiu(c, A3, S4, -0x7598);
                cad_lw(c, V1, S5, 0x8);
                cad_mov(c, T0, SP);
                cad_addu(c, V0, V0, T2);
                cad_addiu(c, T3, T1, 0x40);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S5, 0x8);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S5, 0x8);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_97a4;
            }
            0x0010_97a4 => {
                if cad_copy(
                    c,
                    0x0010_97a4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [V0, V1, A0, A1],
                    },
                ) {
                    return;
                }
                pc = 0x0010_97d0;
            }
            0x0010_97d0 => {
                c.cycles += 16;
                cad_lw(c, V0, S5, 0x8);
                cad_li(c, A2, 0x0046_0000);
                cad_ld(c, A3, T1, 0);
                cad_addiu(c, A0, A2, -0x75e0);
                cad_sd(c, A3, T0, 0);
                cad_li(c, V1, 0x0046_0000);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, V1, -0x7598);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S5, 0x8);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, S5, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9810;
            }
            0x0010_9810 => {
                if cad_copy(
                    c,
                    0x0010_9810,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [A0, T0, T1, S3],
                    },
                ) {
                    return;
                }
                pc = 0x0010_983c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x340)`
            // gets 0x0045_8c10 / 0x459b90 / 0x0045_8c58.
            0x0010_983c => {
                c.cycles += 26;
                cad_ld(c, S4, A2, 0);
                cad_ld(c, V0, A2, 0x8);
                cad_sd(c, S4, A1, 0);
                cad_li(c, A3, 0x0046_0000);
                cad_sd(c, V0, A1, 0x8);
                cad_li(c, T0, 0x0046_0000);
                cad_lw(c, V0, S5, 0x8);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, A1, A2, -0x7598);
                cad_addiu(c, A0, A3, -0x73f0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A2, T0, -0x6470);
                cad_li(c, T1, 0x0046_0000);
                cad_mov(c, T0, SP);
                cad_lhu(c, V0, A1, 0x8);
                cad_addiu(c, A3, T1, -0x73a8);
                cad_lw(c, V1, S7, 0x80);
                cad_mov(c, T1, A0);
                cad_addu(c, V0, V0, T2);
                cad_addiu(c, T3, T1, 0x40);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S7, 0x80);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S7, 0x80);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_98a4;
            }
            0x0010_98a4 => {
                if cad_copy(
                    c,
                    0x0010_98a4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_98d0;
            }
            0x0010_98d0 => {
                c.cycles += 16;
                cad_lw(c, V0, S7, 0x80);
                cad_li(c, V1, 0x0046_0000);
                cad_ld(c, A1, T1, 0);
                cad_addiu(c, A0, V1, -0x73f0);
                cad_sd(c, A1, T0, 0);
                cad_li(c, S3, 0x0046_0000);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S3, -0x73a8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S7, 0x80);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x90);
                cad_subu(c, T2, S7, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9910;
            }
            0x0010_9910 => {
                if cad_copy(
                    c,
                    0x0010_9910,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_993c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x2b8)`
            // gets 0x0045_8e00 / 0x459b90 / 0x0045_8e48.
            0x0010_993c => {
                c.cycles += 26;
                cad_ld(c, A3, A2, 0);
                cad_ld(c, T0, A2, 0x8);
                cad_sd(c, A3, A1, 0);
                cad_li(c, V0, 0x0046_0000);
                cad_sd(c, T0, A1, 0x8);
                cad_addiu(c, A0, V0, -0x7200);
                cad_li(c, S4, 0x0046_0000);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, S7, 0x80);
                cad_addiu(c, A1, S4, -0x73a8);
                cad_addiu(c, A2, V1, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x71b8);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_lhu(c, V0, A1, 0x8);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, V1, FP, 0x8);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, FP, 0x8);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, FP, 0x8);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_99a4;
            }
            0x0010_99a4 => {
                if cad_copy(
                    c,
                    0x0010_99a4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_99d0;
            }
            0x0010_99d0 => {
                c.cycles += 16;
                cad_lw(c, V0, FP, 0x8);
                cad_li(c, V1, 0x0046_0000);
                cad_ld(c, A1, T1, 0);
                cad_addiu(c, A0, V1, -0x7200);
                cad_sd(c, A1, T0, 0);
                cad_li(c, S3, 0x0046_0000);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S3, -0x71b8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, FP, 0x8);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, FP, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9a10;
            }
            0x0010_9a10 => {
                if cad_copy(
                    c,
                    0x0010_9a10,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9a3c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x2a4)`
            // gets 0x0045_8ef8 / 0x459b90 / 0x0045_8f40.
            0x0010_9a3c => {
                c.cycles += 27;
                cad_ld(c, A3, A2, 0);
                cad_ld(c, T0, A2, 0x8);
                cad_sd(c, A3, A1, 0);
                cad_li(c, V0, 0x0046_0000);
                cad_sd(c, T0, A1, 0x8);
                cad_addiu(c, A0, V0, -0x7108);
                cad_li(c, S4, 0x0046_0000);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, FP, 0x8);
                cad_addiu(c, A1, S4, -0x71b8);
                cad_addiu(c, A2, V1, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x70c0);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_lw(c, S3, SP, 0x344);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S3, 0x14);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S3, 0x14);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x14);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_9aa8;
            }
            0x0010_9aa8 => {
                if cad_copy(
                    c,
                    0x0010_9aa8,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9ad4;
            }
            0x0010_9ad4 => {
                c.cycles += 18;
                cad_lw(c, A1, SP, 0x344);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, A0, A2, -0x7108);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, A1, 0x14);
                cad_addiu(c, A2, V1, -0x70c0);
                cad_ld(c, A3, T1, 0);
                cad_mov(c, A1, S1);
                cad_sd(c, A3, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, T0, SP, 0x344);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, V0, T0, 0x14);
                cad_addiu(c, V0, V0, -0x20);
                cad_subu(c, T2, T0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9b1c;
            }
            0x0010_9b1c => {
                if cad_copy(
                    c,
                    0x0010_9b1c,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T1, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9b48;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x288)`
            // gets 0x0045_8ff0 / 0x459b90 / 0x0045_9038.
            0x0010_9b48 => {
                c.cycles += 28;
                cad_ld(c, V1, A2, 0);
                cad_ld(c, A0, A2, 0x8);
                cad_sd(c, V1, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, A0, A1, 0x8);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V1, SP, 0x344);
                cad_addiu(c, A0, S3, -0x7010);
                cad_addiu(c, A1, T1, -0x70c0);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0x14);
                cad_addiu(c, A2, S4, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6fc8);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, S3, SP, 0x324);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S3, 0x58);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S3, 0x58);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x58);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_9bb8;
            }
            0x0010_9bb8 => {
                if cad_copy(
                    c,
                    0x0010_9bb8,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9be4;
            }
            0x0010_9be4 => {
                c.cycles += 18;
                cad_lw(c, A1, SP, 0x324);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, A0, A2, -0x7010);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, A1, 0x58);
                cad_addiu(c, A2, V1, -0x6fc8);
                cad_ld(c, A3, T1, 0);
                cad_mov(c, A1, S1);
                cad_sd(c, A3, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, T0, SP, 0x324);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, V0, T0, 0x58);
                cad_addiu(c, V0, V0, -0x60);
                cad_subu(c, T2, T0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9c2c;
            }
            0x0010_9c2c => {
                if cad_copy(
                    c,
                    0x0010_9c2c,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T1, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9c58;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x224)`
            // gets 0x0045_90e8 / 0x459b90 / 0x0045_9130.
            0x0010_9c58 => {
                c.cycles += 28;
                cad_ld(c, V1, A2, 0);
                cad_ld(c, A0, A2, 0x8);
                cad_sd(c, V1, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, A0, A1, 0x8);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V1, SP, 0x324);
                cad_addiu(c, A0, S3, -0x6f18);
                cad_addiu(c, A1, T1, -0x6fc8);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0x58);
                cad_addiu(c, A2, S4, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6ed0);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, S3, SP, 0x31c);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S3, 0x14);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S3, 0x14);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x14);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_9cc8;
            }
            0x0010_9cc8 => {
                if cad_copy(
                    c,
                    0x0010_9cc8,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9cf4;
            }
            0x0010_9cf4 => {
                c.cycles += 18;
                cad_lw(c, A1, SP, 0x31c);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, A0, A2, -0x6f18);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, A1, 0x14);
                cad_addiu(c, A2, V1, -0x6ed0);
                cad_ld(c, A3, T1, 0);
                cad_mov(c, A1, S1);
                cad_sd(c, A3, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, T0, SP, 0x31c);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, V0, T0, 0x14);
                cad_addiu(c, V0, V0, -0x20);
                cad_subu(c, T2, T0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9d3c;
            }
            0x0010_9d3c => {
                if cad_copy(
                    c,
                    0x0010_9d3c,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T1, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9d68;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x200)`
            // gets 0x0045_91e0 / 0x459b90 / 0x0045_9228.
            0x0010_9d68 => {
                c.cycles += 27;
                cad_ld(c, V1, A2, 0);
                cad_ld(c, A0, A2, 0x8);
                cad_sd(c, V1, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, A0, A1, 0x8);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V1, SP, 0x31c);
                cad_addiu(c, A1, T1, -0x6ed0);
                cad_addiu(c, A0, S3, -0x6e20);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0x14);
                cad_addiu(c, A2, S4, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6dd8);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S0, 0x200);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S0, 0x200);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S0, 0x200);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_9dd4;
            }
            0x0010_9dd4 => {
                if cad_copy(
                    c,
                    0x0010_9dd4,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9e00;
            }
            0x0010_9e00 => {
                c.cycles += 18;
                cad_lw(c, V1, SP, 0x318);
                cad_li(c, A1, 0x0046_0000);
                cad_addiu(c, A0, A1, -0x6e20);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0);
                cad_mov(c, A1, S1);
                cad_ld(c, A2, T1, 0);
                cad_sd(c, A2, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S4, -0x6dd8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, V0, V1, 0);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, A0, SP, 0x318);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, A0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9e48;
            }
            0x0010_9e48 => {
                if cad_copy(
                    c,
                    0x0010_9e48,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T0, T1, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9e74;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x1f0)`
            // gets 0x0045_92d8 / 0x459b90 / 0x0045_9320.
            0x0010_9e74 => {
                c.cycles += 27;
                cad_ld(c, V0, A2, 0);
                cad_ld(c, V1, A2, 0x8);
                cad_sd(c, V0, A1, 0);
                cad_li(c, T0, 0x0046_0000);
                cad_sd(c, V1, A1, 0x8);
                cad_li(c, T1, 0x0046_0000);
                cad_addiu(c, A1, T0, -0x6dd8);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, S4, SP, 0x318);
                cad_addiu(c, A3, V1, -0x6ce0);
                cad_addiu(c, A0, T1, -0x6d28);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V0, S4, 0);
                cad_addiu(c, A2, S3, -0x6470);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S0, 0x1f0);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S0, 0x1f0);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S0, 0x1f0);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_9ee0;
            }
            0x0010_9ee0 => {
                if cad_copy(
                    c,
                    0x0010_9ee0,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [A0, A1, A2, A3],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9f0c;
            }
            0x0010_9f0c => {
                c.cycles += 18;
                cad_lw(c, T2, SP, 0x314);
                cad_li(c, S3, 0x0046_0000);
                cad_addiu(c, A0, S3, -0x6d28);
                cad_li(c, A3, 0x0046_0000);
                cad_lw(c, V0, T2, 0);
                cad_ld(c, S4, T1, 0);
                cad_sd(c, S4, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, A3, -0x6ce0);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, T2, 0);
                cad_lw(c, T0, SP, 0x314);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, T0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_9f54;
            }
            0x0010_9f54 => {
                if cad_copy(
                    c,
                    0x0010_9f54,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T1, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_9f80;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x1e4)`
            // gets 0x0045_93d0 / 0x459b90 / 0x0045_9418.
            0x0010_9f80 => {
                c.cycles += 28;
                cad_ld(c, V1, A2, 0);
                cad_ld(c, A0, A2, 0x8);
                cad_sd(c, V1, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, A0, A1, 0x8);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V1, SP, 0x314);
                cad_addiu(c, A0, S3, -0x6c30);
                cad_addiu(c, A1, T1, -0x6ce0);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0);
                cad_addiu(c, A2, S4, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6be8);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, S3, SP, 0x310);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S3, 0x14);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S3, 0x14);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x14);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_9ff0;
            }
            0x0010_9ff0 => {
                if cad_copy(
                    c,
                    0x0010_9ff0,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a01c;
            }
            0x0010_a01c => {
                c.cycles += 18;
                cad_lw(c, A1, SP, 0x310);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, A0, A2, -0x6c30);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, A1, 0x14);
                cad_addiu(c, A2, V1, -0x6be8);
                cad_ld(c, A3, T1, 0);
                cad_mov(c, A1, S1);
                cad_sd(c, A3, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, T0, SP, 0x310);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, V0, T0, 0x14);
                cad_addiu(c, V0, V0, -0x20);
                cad_subu(c, T2, T0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a064;
            }
            0x0010_a064 => {
                if cad_copy(
                    c,
                    0x0010_a064,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T1, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a090;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x1c0)`
            // gets 0x0045_94c8 / 0x459b90 / 0x0045_9510.
            0x0010_a090 => {
                c.cycles += 27;
                cad_ld(c, V1, A2, 0);
                cad_ld(c, A0, A2, 0x8);
                cad_sd(c, V1, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, A0, A1, 0x8);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V1, SP, 0x310);
                cad_addiu(c, A1, T1, -0x6be8);
                cad_addiu(c, A0, S3, -0x6b38);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0x14);
                cad_addiu(c, A2, S4, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6af0);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S0, 0x1c0);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S0, 0x1c0);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S0, 0x1c0);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_a0fc;
            }
            0x0010_a0fc => {
                if cad_copy(
                    c,
                    0x0010_a0fc,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a128;
            }
            0x0010_a128 => {
                c.cycles += 18;
                cad_lw(c, V1, SP, 0x308);
                cad_li(c, A1, 0x0046_0000);
                cad_addiu(c, A0, A1, -0x6b38);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0);
                cad_mov(c, A1, S1);
                cad_ld(c, A2, T1, 0);
                cad_sd(c, A2, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S4, -0x6af0);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, V0, V1, 0);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, A0, SP, 0x308);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, A0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a170;
            }
            0x0010_a170 => {
                if cad_copy(
                    c,
                    0x0010_a170,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T0, T1, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a19c;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x1b0)`
            // gets 0x0045_95c0 / 0x459b90 / 0x0045_9608.
            0x0010_a19c => {
                c.cycles += 28;
                cad_ld(c, V0, A2, 0);
                cad_ld(c, V1, A2, 0x8);
                cad_sd(c, V0, A1, 0);
                cad_li(c, T0, 0x0046_0000);
                cad_sd(c, V1, A1, 0x8);
                cad_li(c, T1, 0x0046_0000);
                cad_addiu(c, A1, T0, -0x6af0);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, S4, SP, 0x308);
                cad_addiu(c, A3, V1, -0x69f8);
                cad_addiu(c, A0, T1, -0x6a40);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V0, S4, 0);
                cad_addiu(c, A2, S3, -0x6470);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, A1, SP, 0x358);
                cad_addu(c, V0, V0, T2);
                cad_lw(c, V1, A1, 0xa0);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, A1, 0xa0);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, A1, 0xa0);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_a20c;
            }
            0x0010_a20c => {
                if cad_copy(
                    c,
                    0x0010_a20c,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [A2, A3, T2, S3],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a238;
            }
            0x0010_a238 => {
                c.cycles += 17;
                cad_lw(c, S4, SP, 0x358);
                cad_li(c, V1, 0x0046_0000);
                cad_addiu(c, A0, V1, -0x6a40);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V0, S4, 0xa0);
                cad_ld(c, A1, T1, 0);
                cad_sd(c, A1, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S3, -0x69f8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S4, 0xa0);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0xb0);
                cad_subu(c, T2, S4, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a27c;
            }
            0x0010_a27c => {
                if cad_copy(
                    c,
                    0x0010_a27c,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a2a8;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x100)`
            // gets 0x0045_98a8 / 0x459b90 / 0x0045_98f0.
            0x0010_a2a8 => {
                c.cycles += 28;
                cad_ld(c, A3, A2, 0);
                cad_ld(c, T0, A2, 0x8);
                cad_sd(c, A3, A1, 0);
                cad_li(c, V0, 0x0046_0000);
                cad_sd(c, T0, A1, 0x8);
                cad_addiu(c, A0, V0, -0x6758);
                cad_lw(c, A3, SP, 0x358);
                cad_li(c, S4, 0x0046_0000);
                cad_addiu(c, A1, S4, -0x69f8);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, A3, 0xa0);
                cad_addiu(c, A2, V1, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6710);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lw(c, S3, SP, 0x350);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S3, 0x50);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S3, 0x50);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x50);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_a318;
            }
            0x0010_a318 => {
                if cad_copy(
                    c,
                    0x0010_a318,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [S4, V0, V1, A0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a344;
            }
            0x0010_a344 => {
                c.cycles += 18;
                cad_lw(c, A1, SP, 0x350);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, A0, A2, -0x6758);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, V0, A1, 0x50);
                cad_addiu(c, A2, V1, -0x6710);
                cad_ld(c, A3, T1, 0);
                cad_mov(c, A1, S1);
                cad_sd(c, A3, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, T0, SP, 0x350);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, V0, T0, 0x50);
                cad_addiu(c, V0, V0, -0x60);
                cad_subu(c, T2, T0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a38c;
            }
            0x0010_a38c => {
                if cad_copy(
                    c,
                    0x0010_a38c,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T1, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a3b8;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0xa0)`
            // gets 0x0045_96b8 / 0x459b90 / 0x0045_9700.
            0x0010_a3b8 => {
                c.cycles += 27;
                cad_ld(c, V1, A2, 0);
                cad_ld(c, A0, A2, 0x8);
                cad_sd(c, V1, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, A0, A1, 0x8);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V1, SP, 0x350);
                cad_addiu(c, A1, T1, -0x6710);
                cad_addiu(c, A0, S3, -0x6948);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0x50);
                cad_addiu(c, A2, S4, -0x6470);
                cad_li(c, T0, 0x0046_0000);
                cad_mov(c, T1, A0);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, A3, T0, -0x6900);
                cad_mov(c, T0, SP);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S0, 0xa0);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S0, 0xa0);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S0, 0xa0);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_a424;
            }
            0x0010_a424 => {
                if cad_copy(
                    c,
                    0x0010_a424,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [T2, S3, S4, V0],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a450;
            }
            0x0010_a450 => {
                c.cycles += 18;
                cad_lw(c, V1, SP, 0x34c);
                cad_li(c, A1, 0x0046_0000);
                cad_addiu(c, A0, A1, -0x6948);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0);
                cad_mov(c, A1, S1);
                cad_ld(c, A2, T1, 0);
                cad_sd(c, A2, T0, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S4, -0x6900);
                cad_addiu(c, T0, A2, 0xa0);
                cad_lw(c, V0, V1, 0);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, A0, SP, 0x34c);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, A3, A0, V0);
                cad_addu(c, V1, V1, A3);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a498;
            }
            0x0010_a498 => {
                if cad_copy(
                    c,
                    0x0010_a498,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: T0,
                        tmp: [T1, T2, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a4c4;
            }
            // Finish the previous subobject; destroy the wipeout state member
            // (this+0x30); the virtual base at `*(this + 0x24)` gets 0x0045_99a0 / 0x0045_99e8.
            0x0010_a4c4 => {
                c.cycles += 31;
                cad_ld(c, V0, A2, 0);
                cad_ld(c, V1, A2, 0x8);
                cad_sd(c, V0, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, V1, A1, 0x8);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, A0, SP, 0x338);
                cad_addiu(c, A2, T1, -0x6900);
                cad_lw(c, T2, SP, 0x34c);
                cad_li(c, A1, 0);
                cad_lw(c, V1, T2, 0);
                cad_sw(c, S1, V1, 0x6c0);
                cad_lhu(c, V0, A2, 0x8);
                cad_addu(c, V0, V0, A3);
                cad_sh(c, V0, SP, 0x58);
                if call_ptr(c, 0x0013_6ad0, 0x0010_a504).is_err() {
                    return;
                }
                cad_lw(c, S3, SP, 0x330);
                cad_addiu(c, V0, S4, -0x6660);
                cad_mov(c, A3, V0);
                cad_li(c, A1, 0x0046_0000);
                cad_lw(c, V1, S3, 0x4);
                cad_addiu(c, A0, A1, -0x6470);
                cad_li(c, A2, 0x0046_0000);
                cad_addiu(c, T0, A3, 0x40);
                cad_sw(c, V0, V1, 0x6e8);
                cad_addiu(c, A1, A2, -0x6618);
                cad_mov(c, A2, SP);
                cad_lw(c, V0, S3, 0x4);
                cad_sw(c, A0, V0, 0x6d0);
                cad_lw(c, V1, S3, 0x4);
                cad_sw(c, A1, V1, 0x6c0);
                pc = 0x0010_a540;
            }
            0x0010_a540 => {
                if cad_copy(
                    c,
                    0x0010_a540,
                    CadCopy {
                        src: A3,
                        dst: A2,
                        end: T0,
                        tmp: [T1, T2, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a56c;
            }
            0x0010_a56c => {
                c.cycles += 18;
                cad_lw(c, V1, SP, 0x330);
                cad_li(c, A1, 0x0046_0000);
                cad_addiu(c, A0, A1, -0x6660);
                cad_li(c, S4, 0x0046_0000);
                cad_lw(c, V0, V1, 0x4);
                cad_mov(c, A1, S1);
                cad_ld(c, T0, A3, 0);
                cad_sd(c, T0, A2, 0);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, S4, -0x6618);
                cad_addiu(c, A3, A2, 0xa0);
                cad_lw(c, V0, V1, 0x4);
                cad_lhu(c, V1, A0, 0x8);
                cad_lw(c, A0, SP, 0x330);
                cad_addiu(c, V0, V0, -0x10);
                cad_subu(c, T2, A0, V0);
                cad_addu(c, V1, V1, T2);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a5b4;
            }
            0x0010_a5b4 => {
                if cad_copy(
                    c,
                    0x0010_a5b4,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: A3,
                        tmp: [T0, T1, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a5e0;
            }
            // Finish the previous subobject; the virtual base at `*(this + 0x18)`
            // gets 0x0045_9a98 / 0x459b90 / 0x0045_9ae0.
            0x0010_a5e0 => {
                c.cycles += 27;
                cad_ld(c, V0, A2, 0);
                cad_ld(c, V1, A2, 0x8);
                cad_sd(c, V0, A1, 0);
                cad_li(c, T0, 0x0046_0000);
                cad_sd(c, V1, A1, 0x8);
                cad_li(c, T1, 0x0046_0000);
                cad_addiu(c, A1, T0, -0x6618);
                cad_li(c, V1, 0x0046_0000);
                cad_lw(c, S4, SP, 0x330);
                cad_addiu(c, A3, V1, -0x6520);
                cad_addiu(c, A0, T1, -0x6568);
                cad_li(c, S3, 0x0046_0000);
                cad_lw(c, V0, S4, 0x4);
                cad_addiu(c, A2, S3, -0x6470);
                cad_mov(c, T1, A0);
                cad_mov(c, T0, SP);
                cad_sw(c, S1, V0, 0x6c0);
                cad_addiu(c, T3, T1, 0x40);
                cad_lhu(c, V0, A1, 0x8);
                cad_lw(c, V1, S0, 0x18);
                cad_addu(c, V0, V0, T2);
                cad_sh(c, V0, SP, 0x58);
                cad_sw(c, A0, V1, 0x6e8);
                cad_lw(c, V0, S0, 0x18);
                cad_sw(c, A2, V0, 0x6d0);
                cad_lw(c, V1, S0, 0x18);
                cad_sw(c, A3, V1, 0x6c0);
                pc = 0x0010_a64c;
            }
            0x0010_a64c => {
                if cad_copy(
                    c,
                    0x0010_a64c,
                    CadCopy {
                        src: T1,
                        dst: T0,
                        end: T3,
                        tmp: [A0, A1, A2, A3],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a678;
            }
            0x0010_a678 => {
                c.cycles += 16;
                cad_lw(c, V0, S0, 0x18);
                cad_li(c, T2, 0x0046_0000);
                cad_ld(c, S3, T1, 0);
                cad_li(c, A3, 0x0046_0000);
                cad_sd(c, S3, T0, 0);
                cad_addiu(c, A0, T2, -0x6568);
                cad_sw(c, SP, V0, 0x6e8);
                cad_addiu(c, A2, A3, -0x6520);
                cad_addiu(c, T0, A2, 0xa0);
                cad_mov(c, A1, S1);
                cad_lw(c, V0, S0, 0x18);
                cad_lhu(c, V1, A0, 0x8);
                cad_addiu(c, V0, V0, -0x20);
                cad_subu(c, A3, S0, V0);
                cad_addu(c, V1, V1, A3);
                cad_sh(c, V1, SP, 0x8);
                pc = 0x0010_a6b8;
            }
            0x0010_a6b8 => {
                if cad_copy(
                    c,
                    0x0010_a6b8,
                    CadCopy {
                        src: A2,
                        dst: A1,
                        end: T0,
                        tmp: [T1, T2, S3, S4],
                    },
                ) {
                    return;
                }
                pc = 0x0010_a6e4;
            }
            // Finish the last subobject; in_chrg & 2 destroys the Rider virtual base.
            0x0010_a6e4 => {
                c.cycles += 13;
                cad_ld(c, V0, A2, 0);
                cad_ld(c, V1, A2, 0x8);
                cad_sd(c, V0, A1, 0);
                cad_li(c, T1, 0x0046_0000);
                cad_sd(c, V1, A1, 0x8);
                cad_lw(c, V1, S0, 0x18);
                cad_addiu(c, A0, T1, -0x6520);
                cad_sw(c, S1, V1, 0x6c0);
                cad_lhu(c, V0, A0, 0x8);
                cad_lw(c, T2, SP, 0x328);
                cad_addu(c, V0, V0, A3);
                let t = c.g[T2][0] == 0;
                cad_sh(c, V0, SP, 0x58);
                if t {
                    pc = 0x0010_a724;
                    continue;
                }
                pc = 0x0010_a718;
            }
            0x0010_a718 => {
                c.cycles += 3;
                cad_lw(c, A0, S0, 0x18);
                cad_li(c, A1, 0);
                if CAD_RIDER_DTOR.call(c, 0x0010_a724).is_err() {
                    return;
                }
                pc = 0x0010_a724;
            }
            // in_chrg & 1: free the object.
            0x0010_a724 => {
                c.cycles += 3;
                cad_lw(c, S3, SP, 0x320);
                if c.g[S3][0] == 0 {
                    cad_lq(c, S0, SP, 0x3f0);
                    pc = 0x0010_a73c;
                    continue;
                }
                pc = 0x0010_a730;
            }
            0x0010_a730 => {
                c.cycles += 3;
                cad_mov(c, A0, S0);
                if call_ptr(c, 0x0031_7e50, 0x0010_a738).is_err() {
                    return;
                }
                cad_lq(c, S0, SP, 0x3f0);
                pc = 0x0010_a73c;
            }
            // Epilogue.
            0x0010_a73c => {
                c.cycles += 11;
                cad_lq(c, S1, SP, 0x3e0);
                cad_lq(c, S2, SP, 0x3d0);
                cad_lq(c, S3, SP, 0x3c0);
                cad_lq(c, S4, SP, 0x3b0);
                cad_lq(c, S5, SP, 0x3a0);
                cad_lq(c, S6, SP, 0x390);
                cad_lq(c, S7, SP, 0x380);
                cad_lq(c, FP, SP, 0x370);
                cad_ld(c, RA, SP, 0x360);
                let ra = c.g[RA][0] as u32;
                cad_addiu(c, SP, SP, 0x400);
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0010_8fa8, 0x0010_a768);
                return;
            }
        }
    }
}

/// The object `computer_actor_rider_dtor_matches` destroys.
#[cfg(test)]
pub(crate) const CAD_THIS: u32 = 0x0080_1000;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (other tests' machines fall back).
    pub(crate) const CAD_CTRL: u32 = DATA + 0xf_f000;
    pub(crate) const CAD_MAGIC: u32 = 0xcad0_d70e;

    macro_rules! cad_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CAD_CTRL) != CAD_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    cad_gated!(
        cad_fake_audio_instance,
        CAD_PREV_AUDIO_INSTANCE,
        0x0028_b180,
        ee::s07::f_0028b180__generated
    );
    cad_gated!(
        cad_fake_kill_rider_timers,
        CAD_PREV_KILL_RIDER_TIMERS,
        0x0029_2ae8,
        ee::s07::f_00292ae8__generated
    );
    cad_gated!(
        cad_fake_script_unregister,
        CAD_PREV_SCRIPT_UNREGISTER,
        0x0028_32d8,
        ee::s07::f_002832d8__generated
    );
    cad_gated!(
        cad_fake_script_dtor,
        CAD_PREV_SCRIPT_DTOR,
        0x0028_3228,
        ee::s07::f_00283228__generated
    );
    cad_gated!(
        cad_fake_delete,
        CAD_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    /// `Rider_dtor`'s callees that need a game (audio system, script
    /// objects, the allocator) become logging fakes.
    fn cad_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            cad_fake_audio_instance,
            &CAD_PREV_AUDIO_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00292AE8),
            cad_fake_kill_rider_timers,
            &CAD_PREV_KILL_RIDER_TIMERS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002832D8),
            cad_fake_script_unregister,
            &CAD_PREV_SCRIPT_UNREGISTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283228),
            cad_fake_script_dtor,
            &CAD_PREV_SCRIPT_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            cad_fake_delete,
            &CAD_PREV_DELETE,
        );
    }

    /// Virtual bases the subobjects point at (zeroed; the first is the
    /// `Rider` base `Rider_dtor` reads: every optional pointer 0).
    const CAD_VBASES: [u32; 4] = [
        DATA + 0x1_0000,
        DATA + 0x1_4000,
        DATA + 0x1_8000,
        DATA + 0x1_c000,
    ];

    /// Words of the object that hold a virtual-base pointer: the ones the
    /// destructor reads, the members' own (`+0` of each member, the wipeout
    /// state's `+0x40`).
    const CAD_HOLDERS: [u32; 31] = [
        0x18, 0x24, 0xa0, 0x100, 0x1b0, 0x1c0, 0x1e4, 0x1f0, 0x200, 0x224, 0x288, 0x2a4, 0x2b8,
        0x340, 0x358, 0x364, 0x384, 0x398, 0x3a0, 0x3b0, 0x470, 0x520, 0x610, 0x9c0, 0xad0, 0xaf0,
        0xb00, 0xb40, 0xc70, 0xd20, 0x70,
    ];

    /// The object at `CAD_THIS` (members' other fields 0: their destructors
    /// free nothing), random vtable sources, `in_chrg` by seed: 0 (not in
    /// charge), 1 (free), 2 (destroy the base), 3, or random bits.
    fn cad_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(CAD_CTRL, CAD_MAGIC);
        fill(c, r, 0x0045_8600, (0x0045_9c00 - 0x0045_8600) / 4);
        for &h in CAD_HOLDERS.iter() {
            let vb = CAD_VBASES[(r.next_u32() % 4) as usize];
            c.bus.write32(CAD_THIS + h, vb);
        }
        c.bus.write32(CAD_THIS + 0x18, CAD_VBASES[0]);
        c.g[A0][0] = sx(CAD_THIS);
        c.g[A1][0] = match seed % 5 {
            k @ 0..=3 => k as u64,
            _ => (r.next_u32() as u64) << 32 | (r.next_u32() | 4) as u64,
        };
        c.pc = 0x0010_8fa8;
    }

    /// `ComputerActorRider_dtor` against the generated body: every `in_chrg`
    /// path, with real member destructors and fakes (some unwinding) under
    /// `Rider_dtor` and for the final free.
    #[test]
    fn computer_actor_rider_dtor_matches() {
        cad_install_fakes();
        covered(
            ee::s00::f_00108fa8__generated,
            computer_actor_rider_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_8fa8, 11);
                cad_fixture(c, &mut r, seed);
            },
        );
    }

    driven!(cad_generated_driven, ee::s00::f_00108fa8__generated);
    driven!(cad_hand_driven, computer_actor_rider_dtor);

    /// The copy loops' event checks: delivered, and unwinding with the
    /// destructor re-entered at the loop head (both sides driven like the
    /// trampoline).
    #[test]
    fn computer_actor_rider_dtor_events_match() {
        cad_install_fakes();
        check_events(cad_generated_driven, cad_hand_driven, 45, &|c, seed| {
            let mut r = machine(c, seed, 0x10_8fa9, 0);
            cad_fixture(c, &mut r, seed / 3);
            schedule(c, &mut r, seed, 0x0010_8fa8)
        });
    }
}
