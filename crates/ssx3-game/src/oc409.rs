//! Staging module oc409 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc409`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0010_fcd8, rider_base_ctor, "RiderBase_ctor")];

// Register-level instruction helpers for the constructor below (the
// compiled code is straight-line `addiu`/`lw`/`sw` over registers).

#[inline(always)]
fn rbc_addiu(c: &mut Cpu, rd: usize, rs: usize, k: u32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(k));
}
#[inline(always)]
fn rbc_subiu(c: &mut Cpu, rd: usize, rs: usize, k: u32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_sub(k));
}
#[inline(always)]
fn rbc_addu(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(c.g[rt][0] as u32));
}
#[inline(always)]
fn rbc_subu(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_sub(c.g[rt][0] as u32));
}
#[inline(always)]
fn rbc_lw(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    c.g[rt][0] = c.lw((c.g[base][0] as u32).wrapping_add(off));
}
#[inline(always)]
fn rbc_lhu(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    c.g[rt][0] = c.lhu((c.g[base][0] as u32).wrapping_add(off));
}
#[inline(always)]
fn rbc_ld(c: &mut Cpu, rt: usize, base: usize, off: u32) {
    c.g[rt][0] = c.ld((c.g[base][0] as u32).wrapping_add(off));
}
#[inline(always)]
fn rbc_sw(c: &mut Cpu, base: usize, off: u32, rt: usize) {
    c.sw((c.g[base][0] as u32).wrapping_add(off), c.g[rt][0]);
}
#[inline(always)]
fn rbc_sh(c: &mut Cpu, base: usize, off: u32, rt: usize) {
    c.sh((c.g[base][0] as u32).wrapping_add(off), c.g[rt][0]);
}
#[inline(always)]
fn rbc_sd(c: &mut Cpu, base: usize, off: u32, rt: usize) {
    c.sd((c.g[base][0] as u32).wrapping_add(off), c.g[rt][0]);
}
#[inline(always)]
fn rbc_sw_zero(c: &mut Cpu, base: usize, off: u32) {
    c.sw((c.g[base][0] as u32).wrapping_add(off), 0u64);
}
#[inline(always)]
fn rbc_lq(c: &mut Cpu, reg: usize, base: usize, off: u32) {
    c.lq(reg, (c.g[base][0] as u32).wrapping_add(off));
}
#[inline(always)]
fn rbc_sq(c: &mut Cpu, reg: usize, base: usize, off: u32) {
    c.sq(reg, (c.g[base][0] as u32).wrapping_add(off));
}

/// The compiler's 32-byte struct copy loop (4 doublewords through `tmp`,
/// `src`/`dst` advance by 0x20 until `src == end`), 11 cycles per round,
/// with the event check at the back edge. True when an event unwound the
/// thread (the loop head `head` is the resume entry).
fn rbc_copy32_loop(
    c: &mut Cpu,
    head: u32,
    tmp: [usize; 4],
    src: usize,
    dst: usize,
    end: usize,
) -> bool {
    loop {
        c.cycles += 11;
        for (i, &t) in tmp.iter().enumerate() {
            rbc_ld(c, t, src, 8 * i as u32);
        }
        for (i, &t) in tmp.iter().enumerate() {
            rbc_sd(c, dst, 8 * i as u32, t);
        }
        rbc_addiu(c, src, src, 0x20);
        let more = c.g[src][0] != c.g[end][0];
        rbc_addiu(c, dst, dst, 0x20);
        if !more {
            return false;
        }
        if back_edge_unwinds(c, head) {
            return true;
        }
    }
}

/// `RiderBase* RiderBase_ctor(RiderBase* this, int in_chrg)` (0x10fcd8):
/// the rider base constructor. Unless `in_chrg` is 0 (a derived class
/// already ran the base chain) it points ~30 members' vtable slots at
/// `this+0xdf0` and runs `Rider_ctor`; then it constructs the state objects
/// (ground, wipeout, handplant, wipeout-sub, ...) and the visual effects
/// (board wake, sparks, trails, anger, ...). Each state's template is copied
/// from stack temporaries (0x40/0xa0-byte struct copies, [`rbc_copy32_loop`])
/// into its member. Returns `this` in v0. The block labels of the generated
/// body are the resume entries after an unwound event; anything else
/// re-enters through `jump_out`.
pub fn rider_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0010fcd8 => {
                    c.cycles += 14;
                    rbc_subiu(c, SP, SP, 0x1270);
                    c.sq(16, sp(c).wrapping_add(0x1260));
                    rbc_sw(c, SP, 0x1100, A1);
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, sp(c).wrapping_add(0x1250));
                    c.sq(18, sp(c).wrapping_add(0x1240));
                    c.sq(19, sp(c).wrapping_add(0x1230));
                    c.sq(20, sp(c).wrapping_add(0x1220));
                    c.sq(21, sp(c).wrapping_add(0x1210));
                    c.sq(22, sp(c).wrapping_add(0x1200));
                    c.sq(23, sp(c).wrapping_add(0x11f0));
                    c.sq(30, sp(c).wrapping_add(0x11e0));
                    let t = c.g[A1][0] == 0u64;
                    rbc_sd(c, SP, 0x11d0, RA);
                    pc = if t { 0x0010fd98 } else { 0x0010fd10 };
                }
                0x0010fd10 => {
                    c.cycles += 34;
                    rbc_addiu(c, V0, S0, 0xdf0);
                    c.g[A0][0] = c.g[V0][0];
                    rbc_sw(c, S0, 0xd20, V0);
                    rbc_sw(c, S0, 0xc70, V0);
                    rbc_sw(c, S0, 0xb40, V0);
                    rbc_sw(c, S0, 0xb00, V0);
                    rbc_sw(c, S0, 0xaf0, V0);
                    rbc_sw(c, S0, 0xad0, V0);
                    rbc_sw(c, S0, 0x9c0, V0);
                    rbc_sw(c, S0, 0x610, V0);
                    rbc_sw(c, S0, 0x520, V0);
                    rbc_sw(c, S0, 0x470, V0);
                    rbc_sw(c, S0, 0x3b0, V0);
                    rbc_sw(c, S0, 0x3a0, V0);
                    rbc_sw(c, S0, 0x398, V0);
                    rbc_sw(c, S0, 0x384, V0);
                    rbc_sw(c, S0, 0x364, V0);
                    rbc_sw(c, S0, 0x358, V0);
                    rbc_sw(c, S0, 0x340, V0);
                    rbc_sw(c, S0, 0x2b8, V0);
                    rbc_sw(c, S0, 0x2a4, V0);
                    rbc_sw(c, S0, 0x288, V0);
                    rbc_sw(c, S0, 0x224, V0);
                    rbc_sw(c, S0, 0x200, V0);
                    rbc_sw(c, S0, 0x1f0, V0);
                    rbc_sw(c, S0, 0x1e4, V0);
                    rbc_sw(c, S0, 0x1c0, V0);
                    rbc_sw(c, S0, 0x1b0, V0);
                    rbc_sw(c, S0, 0x100, V0);
                    rbc_sw(c, S0, 0xa0, V0);
                    rbc_sw(c, S0, 0x70, V0);
                    rbc_sw(c, S0, 0x24, V0);
                    rbc_sw(c, S0, 0x18, V0);
                    Callee {
                        addr: 0x0011b698,
                        f: ee::s00::Rider_ctor,
                    }
                    .call(c, 0x0010fd98)?; // Rider_ctor
                    pc = 0x0010fd98;
                }
                0x0010fd98 => {
                    c.cycles += 90;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0013c650, 0x0010fda4)?; // RiderGroundState_ctor
                    rbc_addiu(c, S6, S0, 0xa0);
                    rbc_addiu(c, V0, SP, 0x100);
                    rbc_addiu(c, V1, SP, 0x150);
                    rbc_sw(c, SP, 0x1164, V0);
                    rbc_addiu(c, A0, SP, 0x200);
                    rbc_sw(c, SP, 0x117c, V1);
                    rbc_addiu(c, A1, SP, 0x250);
                    rbc_sw(c, SP, 0x1108, A0);
                    rbc_addiu(c, A2, SP, 0x300);
                    rbc_addiu(c, A3, SP, 0x350);
                    rbc_addiu(c, T0, SP, 0x400);
                    rbc_addiu(c, T2, SP, 0x450);
                    rbc_addiu(c, T5, SP, 0x500);
                    rbc_addiu(c, V0, SP, 0x550);
                    rbc_addiu(c, V1, SP, 0x600);
                    rbc_sw(c, SP, 0x112c, A1);
                    rbc_addiu(c, A0, SP, 0x650);
                    rbc_sw(c, SP, 0x116c, A2);
                    rbc_addiu(c, A1, SP, 0x700);
                    rbc_sw(c, SP, 0x1188, A3);
                    rbc_addiu(c, A2, SP, 0x750);
                    rbc_sw(c, SP, 0x1110, T0);
                    rbc_addiu(c, A3, SP, 0x800);
                    rbc_sw(c, SP, 0x1138, T2);
                    rbc_addiu(c, T0, SP, 0x850);
                    rbc_sw(c, SP, 0x1170, T5);
                    rbc_addiu(c, T2, SP, 0x900);
                    rbc_sw(c, SP, 0x1190, V0);
                    rbc_addiu(c, T5, SP, 0x950);
                    rbc_sw(c, SP, 0x1120, V1);
                    rbc_addiu(c, V0, SP, 0xa00);
                    rbc_sw(c, SP, 0x1144, A0);
                    rbc_addiu(c, V1, SP, 0xa50);
                    rbc_sw(c, SP, 0x1174, A1);
                    rbc_addiu(c, A0, SP, 0xb00);
                    rbc_sw(c, SP, 0x119c, A2);
                    rbc_addiu(c, A1, SP, 0xb50);
                    rbc_sw(c, SP, 0x1128, A3);
                    rbc_addiu(c, A2, SP, 0xc00);
                    rbc_sw(c, SP, 0x1150, T0);
                    rbc_addiu(c, A3, SP, 0xc50);
                    rbc_sw(c, SP, 0x1180, T2);
                    rbc_addiu(c, T0, SP, 0xd00);
                    rbc_sw(c, SP, 0x11a4, T5);
                    rbc_addiu(c, T2, SP, 0xd50);
                    rbc_sw(c, SP, 0x1130, V0);
                    rbc_addiu(c, T5, SP, 0xe00);
                    rbc_sw(c, SP, 0x1154, V1);
                    rbc_addiu(c, V0, SP, 0xe50);
                    rbc_sw(c, SP, 0x118c, A0);
                    rbc_addiu(c, V1, SP, 0xf00);
                    rbc_sw(c, SP, 0x113c, A2);
                    rbc_addiu(c, A0, SP, 0xf50);
                    rbc_sw(c, SP, 0x115c, A3);
                    rbc_addiu(c, T1, S0, 0x20);
                    rbc_sw(c, SP, 0x1194, T0);
                    rbc_addiu(c, T3, SP, 0x50);
                    rbc_sw(c, SP, 0x11b8, T2);
                    rbc_addiu(c, T4, S0, 0x30);
                    rbc_sw(c, SP, 0x1148, T5);
                    rbc_addiu(c, T2, S0, 0x110);
                    rbc_sw(c, SP, 0x1160, V0);
                    rbc_addiu(c, T5, S0, 0x1f0);
                    rbc_sw(c, SP, 0x11a0, V1);
                    c.g[V0][0] = 0x460000;
                    rbc_sw(c, SP, 0x11ac, A1);
                    c.g[T0][0] = c.g[V0][0];
                    rbc_sw(c, SP, 0x11c4, A0);
                    rbc_subiu(c, V0, V0, 0x6660);
                    rbc_lw(c, A0, T1, 0x4);
                    c.g[V1][0] = 0x460000;
                    rbc_subiu(c, V1, V1, 0x6470);
                    c.g[A3][0] = c.g[V0][0];
                    rbc_sw(c, A0, 0x6e8, V0);
                    rbc_addiu(c, S1, S0, 0xb0);
                    c.g[A0][0] = 0x460000;
                    rbc_addiu(c, FP, S0, 0x1c0);
                    rbc_lw(c, A1, T1, 0x4);
                    c.g[A2][0] = c.g[A0][0];
                    rbc_subiu(c, A0, A0, 0x6618);
                    rbc_addiu(c, S2, S0, 0x1d0);
                    rbc_sw(c, A1, 0x6d0, V1);
                    rbc_addiu(c, S3, S0, 0x210);
                    rbc_addiu(c, A1, A3, 0x40);
                    c.g[V1][0] = c.g[SP][0];
                    rbc_lw(c, V0, T1, 0x4);
                    rbc_addiu(c, S5, S0, 0x230);
                    pc = 0x0010ff00;
                }
                0x0010ff00 => {
                    c.cycles += 41;
                    rbc_addiu(c, S4, S0, 0x290);
                    rbc_addiu(c, S7, S0, 0x2b0);
                    rbc_sw(c, V0, 0x6c0, A0);
                    rbc_addiu(c, V0, S0, 0x200);
                    rbc_addiu(c, A0, S0, 0x2c0);
                    rbc_sw(c, SP, 0x1198, T2);
                    rbc_sw(c, SP, 0x1134, T5);
                    rbc_addiu(c, T2, S0, 0x350);
                    rbc_sw(c, SP, 0x1140, V0);
                    rbc_addiu(c, T5, S0, 0x360);
                    rbc_sw(c, SP, 0x1178, A0);
                    rbc_addiu(c, V0, S0, 0x370);
                    rbc_addiu(c, A0, S0, 0x390);
                    rbc_sw(c, SP, 0x11bc, T2);
                    rbc_sw(c, SP, 0x11c8, T5);
                    rbc_addiu(c, T2, S0, 0x3a0);
                    rbc_sw(c, SP, 0x1104, V0);
                    rbc_addiu(c, T5, S0, 0x3b0);
                    rbc_sw(c, SP, 0x1114, A0);
                    rbc_addiu(c, V0, S0, 0x470);
                    rbc_addiu(c, A0, S0, 0x520);
                    rbc_sw(c, SP, 0x111c, T2);
                    rbc_sw(c, SP, 0x1124, T5);
                    rbc_addiu(c, T2, S0, 0x610);
                    rbc_sw(c, SP, 0x1168, V0);
                    rbc_addiu(c, T5, S0, 0x9c0);
                    rbc_sw(c, SP, 0x11b0, A0);
                    rbc_addiu(c, V0, S0, 0xad0);
                    rbc_addiu(c, A0, S0, 0xaf0);
                    rbc_sw(c, SP, 0x1158, T2);
                    rbc_sw(c, SP, 0x114c, T5);
                    rbc_addiu(c, T2, S0, 0xb00);
                    rbc_sw(c, SP, 0x11a8, V0);
                    rbc_addiu(c, T5, S0, 0xb40);
                    rbc_sw(c, SP, 0x11b4, A0);
                    rbc_addiu(c, V0, S0, 0xc70);
                    rbc_addiu(c, A0, S0, 0xd20);
                    rbc_sw(c, SP, 0x11c0, T2);
                    rbc_sw(c, SP, 0x1118, T5);
                    rbc_sw(c, SP, 0x1184, V0);
                    rbc_sw(c, SP, 0x110c, A0);
                    pc = 0x0010ffa4;
                }
                0x0010ffa4 => {
                    if rbc_copy32_loop(c, 0x0010ffa4, [T2, T5, V0, A0], A3, V1, A1) {
                        return Ok(());
                    }
                    pc = 0x0010ffd0;
                }
                0x0010ffd0 => {
                    c.cycles += 14;
                    rbc_ld(c, A1, A3, 0);
                    rbc_subiu(c, A0, T0, 0x6660);
                    rbc_sd(c, V1, 0, A1);
                    rbc_lw(c, V1, T1, 0x4);
                    rbc_subiu(c, A3, A2, 0x6618);
                    c.g[A1][0] = c.g[T3][0];
                    rbc_addiu(c, T2, A3, 0xa0);
                    rbc_sw(c, V1, 0x6e8, SP);
                    rbc_lw(c, V0, T1, 0x4);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, T0, T1, V0);
                    rbc_addu(c, V1, V1, T0);
                    rbc_sh(c, SP, 0x8, V1);
                    pc = 0x00110008;
                }
                0x00110008 => {
                    if rbc_copy32_loop(c, 0x00110008, [V1, A0, T5, V0], A3, A1, T2) {
                        return Ok(());
                    }
                    pc = 0x00110034;
                }
                0x00110034 => {
                    c.cycles += 28;
                    rbc_ld(c, V1, A3, 0);
                    rbc_ld(c, A0, A3, 0x8);
                    rbc_sd(c, A1, 0, V1);
                    rbc_subiu(c, A2, A2, 0x6618);
                    rbc_sd(c, A1, 0x8, A0);
                    rbc_lw(c, V1, T1, 0x4);
                    c.g[A0][0] = c.g[T4][0];
                    c.g[A1][0] = 0u64;
                    rbc_sw(c, V1, 0x6c0, T3);
                    rbc_lhu(c, V0, A2, 0x8);
                    rbc_addu(c, V0, V0, T0);
                    rbc_sh(c, SP, 0x58, V0);
                    call_ptr(c, 0x00136980, 0x00110068)?; // RiderWipeoutState_ctor
                    c.g[A0][0] = 0x460000;
                    rbc_lw(c, V1, S0, 0xa0);
                    rbc_subiu(c, V0, A0, 0x6948);
                    c.g[A1][0] = 0x460000;
                    rbc_subiu(c, A0, A1, 0x6470);
                    c.g[A2][0] = 0x460000;
                    rbc_sw(c, V1, 0x6e8, V0);
                    rbc_subiu(c, A1, A2, 0x6900);
                    c.g[A2][0] = c.g[V0][0];
                    rbc_lw(c, V0, S0, 0xa0);
                    rbc_addiu(c, T0, A2, 0x40);
                    rbc_lw(c, A3, SP, 0x1164);
                    rbc_sw(c, V0, 0x6d0, A0);
                    rbc_lw(c, V1, S0, 0xa0);
                    rbc_sw(c, V1, 0x6c0, A1);
                    pc = 0x001100a4;
                }
                0x001100a4 => {
                    if rbc_copy32_loop(c, 0x001100a4, [T1, T2, T3, T5], A2, A3, T0) {
                        return Ok(());
                    }
                    pc = 0x001100d0;
                }
                0x001100d0 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S6, 0);
                    c.g[V1][0] = 0x460000;
                    rbc_ld(c, A1, A2, 0);
                    rbc_subiu(c, A0, V1, 0x6948);
                    rbc_sd(c, A3, 0, A1);
                    c.g[T3][0] = 0x460000;
                    rbc_subiu(c, A2, T3, 0x6900);
                    rbc_lw(c, T5, SP, 0x1164);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, T5);
                    rbc_lw(c, V0, S6, 0);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_lw(c, A1, SP, 0x117c);
                    rbc_subu(c, T1, S6, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x108, V1);
                    pc = 0x00110114;
                }
                0x00110114 => {
                    if rbc_copy32_loop(c, 0x00110114, [V0, V1, A0, T0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110140;
                }
                0x00110140 => {
                    c.cycles += 30;
                    rbc_lw(c, V1, S6, 0);
                    c.g[T2][0] = 0x460000;
                    rbc_ld(c, T3, A2, 0);
                    rbc_ld(c, T5, A2, 0x8);
                    rbc_sd(c, A1, 0, T3);
                    rbc_subiu(c, A0, T2, 0x6900);
                    rbc_sd(c, A1, 0x8, T5);
                    c.g[T2][0] = 0x460000;
                    c.g[V0][0] = 0xffffffffffff0000;
                    rbc_subiu(c, A1, T2, 0x6758);
                    rbc_lw(c, T0, SP, 0x117c);
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[T3][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_sw(c, V1, 0x6c0, T0);
                    rbc_subiu(c, A2, T3, 0x6470);
                    rbc_subiu(c, A3, T5, 0x6710);
                    c.g[T0][0] = c.g[A1][0];
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_addiu(c, T2, T0, 0x40);
                    rbc_sw(c, S0, 0xd4, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_lw(c, A0, S1, 0x50);
                    rbc_sh(c, SP, 0x158, V1);
                    rbc_lw(c, T1, SP, 0x1108);
                    rbc_sw(c, A0, 0x6e8, A1);
                    rbc_lw(c, V0, S1, 0x50);
                    rbc_sw(c, V0, 0x6d0, A2);
                    rbc_lw(c, V1, S1, 0x50);
                    rbc_sw(c, V1, 0x6c0, A3);
                    pc = 0x001101b8;
                }
                0x001101b8 => {
                    if rbc_copy32_loop(c, 0x001101b8, [V0, V1, A0, A1], T0, T1, T2) {
                        return Ok(());
                    }
                    pc = 0x001101e4;
                }
                0x001101e4 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S1, 0x50);
                    c.g[A2][0] = 0x460000;
                    rbc_ld(c, A3, T0, 0);
                    c.g[V1][0] = 0x460000;
                    rbc_sd(c, T1, 0, A3);
                    rbc_subiu(c, A0, A2, 0x6758);
                    rbc_subiu(c, T0, V1, 0x6710);
                    rbc_lw(c, A1, SP, 0x1108);
                    rbc_sw(c, V0, 0x6e8, A1);
                    rbc_addiu(c, A1, T0, 0xa0);
                    rbc_lw(c, V0, S1, 0x50);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x60);
                    rbc_lw(c, A2, SP, 0x112c);
                    rbc_subu(c, A3, S1, V0);
                    rbc_addu(c, V1, V1, A3);
                    rbc_sh(c, SP, 0x208, V1);
                    pc = 0x00110228;
                }
                0x00110228 => {
                    if rbc_copy32_loop(c, 0x00110228, [T1, T2, T3, T5], T0, A2, A1) {
                        return Ok(());
                    }
                    pc = 0x00110254;
                }
                0x00110254 => {
                    c.cycles += 38;
                    rbc_lw(c, V1, S1, 0x50);
                    c.g[V0][0] = 0x460000;
                    rbc_ld(c, A0, T0, 0);
                    rbc_ld(c, T1, T0, 0x8);
                    rbc_sd(c, A2, 0, A0);
                    rbc_subiu(c, A1, V0, 0x6710);
                    rbc_sd(c, A2, 0x8, T1);
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A2][0] = 0x500000;
                    rbc_lw(c, T5, SP, 0x112c);
                    rbc_subiu(c, A2, A2, 0xea0);
                    rbc_sw(c, V1, 0x6c0, T5);
                    rbc_lhu(c, V1, A1, 0x8);
                    rbc_lw(c, A0, SP, 0x1198);
                    c.g[A1][0] = 0u64;
                    rbc_addu(c, V1, V1, A3);
                    rbc_sw(c, S1, 0x24, V0);
                    rbc_sw_zero(c, S1, 0x18);
                    rbc_sh(c, SP, 0x258, V1);
                    rbc_lq(c, 2, A2, 0);
                    rbc_sq(c, 2, S1, 0);
                    call_ptr(c, 0x00141480, 0x001102b0)?; // RiderHandplantState_ctor
                    c.g[A0][0] = 0x460000;
                    rbc_lw(c, V1, S0, 0x1c0);
                    rbc_subiu(c, V0, A0, 0x6b38);
                    c.g[A1][0] = 0x460000;
                    rbc_subiu(c, A0, A1, 0x6470);
                    c.g[A2][0] = 0x460000;
                    rbc_sw(c, V1, 0x6e8, V0);
                    rbc_subiu(c, A1, A2, 0x6af0);
                    c.g[A2][0] = c.g[V0][0];
                    rbc_lw(c, V0, S0, 0x1c0);
                    rbc_addiu(c, T0, A2, 0x40);
                    rbc_lw(c, A3, SP, 0x116c);
                    rbc_sw(c, V0, 0x6d0, A0);
                    rbc_lw(c, V1, S0, 0x1c0);
                    rbc_sw(c, V1, 0x6c0, A1);
                    pc = 0x001102ec;
                }
                0x001102ec => {
                    if rbc_copy32_loop(c, 0x001102ec, [T1, T2, T3, T5], A2, A3, T0) {
                        return Ok(());
                    }
                    pc = 0x00110318;
                }
                0x00110318 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, FP, 0);
                    c.g[V1][0] = 0x460000;
                    rbc_ld(c, A1, A2, 0);
                    rbc_subiu(c, A0, V1, 0x6b38);
                    rbc_sd(c, A3, 0, A1);
                    c.g[T3][0] = 0x460000;
                    rbc_subiu(c, A2, T3, 0x6af0);
                    rbc_lw(c, T5, SP, 0x116c);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, T5);
                    rbc_lw(c, V0, FP, 0);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_lw(c, A1, SP, 0x1188);
                    rbc_subu(c, T1, FP, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x308, V1);
                    pc = 0x0011035c;
                }
                0x0011035c => {
                    if rbc_copy32_loop(c, 0x0011035c, [V0, V1, A0, T0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110388;
                }
                0x00110388 => {
                    c.cycles += 27;
                    rbc_lw(c, V0, FP, 0);
                    c.g[T2][0] = 0x460000;
                    rbc_ld(c, T3, A2, 0);
                    rbc_ld(c, T5, A2, 0x8);
                    rbc_sd(c, A1, 0, T3);
                    rbc_subiu(c, V1, T2, 0x6af0);
                    rbc_sd(c, A1, 0x8, T5);
                    c.g[T0][0] = 0x460000;
                    rbc_subiu(c, A0, T0, 0x6c30);
                    c.g[T3][0] = 0x460000;
                    rbc_lw(c, T2, SP, 0x1188);
                    rbc_subiu(c, A1, T3, 0x6470);
                    c.g[T5][0] = 0x460000;
                    c.g[A3][0] = c.g[A0][0];
                    rbc_sw(c, V0, 0x6c0, T2);
                    rbc_subiu(c, A2, T5, 0x6be8);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_lw(c, V1, S2, 0x14);
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x1110);
                    rbc_sh(c, SP, 0x358, V0);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S2, 0x14);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S2, 0x14);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x001103f4;
                }
                0x001103f4 => {
                    if rbc_copy32_loop(c, 0x001103f4, [V0, V1, A0, A1], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110420;
                }
                0x00110420 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S2, 0x14);
                    c.g[A2][0] = 0x460000;
                    rbc_ld(c, T1, A3, 0);
                    rbc_subiu(c, A0, A2, 0x6c30);
                    rbc_sd(c, T0, 0, T1);
                    c.g[V1][0] = 0x460000;
                    rbc_subiu(c, A2, V1, 0x6be8);
                    rbc_lw(c, A1, SP, 0x1110);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, A1);
                    rbc_lw(c, A1, SP, 0x1138);
                    rbc_lw(c, V0, S2, 0x14);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x20);
                    rbc_subu(c, T1, S2, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x408, V1);
                    pc = 0x00110464;
                }
                0x00110464 => {
                    if rbc_copy32_loop(c, 0x00110464, [T0, T2, T3, T5], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110490;
                }
                0x00110490 => {
                    c.cycles += 27;
                    rbc_lw(c, V0, S2, 0x14);
                    c.g[A0][0] = 0x460000;
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x6be8);
                    rbc_sd(c, A1, 0x8, T0);
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, A1, SP, 0x1138);
                    rbc_subiu(c, A0, T5, 0x6d28);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x6ce0);
                    rbc_lw(c, V1, S0, 0x1f0);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x1170);
                    rbc_sh(c, SP, 0x458, V0);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S0, 0x1f0);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S0, 0x1f0);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x001104fc;
                }
                0x001104fc => {
                    if rbc_copy32_loop(c, 0x001104fc, [T1, T3, T5, V0], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110528;
                }
                0x00110528 => {
                    c.cycles += 19;
                    rbc_lw(c, V1, SP, 0x1134);
                    c.g[A1][0] = 0x460000;
                    rbc_subiu(c, A0, A1, 0x6d28);
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0);
                    rbc_ld(c, A2, A3, 0);
                    rbc_sd(c, T0, 0, A2);
                    rbc_subiu(c, A2, T5, 0x6ce0);
                    rbc_lw(c, V1, SP, 0x1170);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, T0, SP, 0x1134);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, T0, 0);
                    rbc_lw(c, A1, SP, 0x1190);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, T1, T0, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x508, V1);
                    pc = 0x00110574;
                }
                0x00110574 => {
                    if rbc_copy32_loop(c, 0x00110574, [T2, T3, T5, V0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x001105a0;
                }
                0x001105a0 => {
                    c.cycles += 28;
                    rbc_lw(c, V1, SP, 0x1134);
                    c.g[A0][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0);
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x6ce0);
                    rbc_sd(c, A1, 0x8, T0);
                    rbc_subiu(c, A0, T5, 0x6e20);
                    rbc_lw(c, A1, SP, 0x1190);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x6dd8);
                    rbc_lw(c, V1, S0, 0x200);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x1120);
                    rbc_sh(c, SP, 0x558, V0);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S0, 0x200);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S0, 0x200);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110610;
                }
                0x00110610 => {
                    if rbc_copy32_loop(c, 0x00110610, [T1, T3, T5, V0], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x0011063c;
                }
                0x0011063c => {
                    c.cycles += 19;
                    rbc_lw(c, V1, SP, 0x1140);
                    c.g[A1][0] = 0x460000;
                    rbc_subiu(c, A0, A1, 0x6e20);
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0);
                    rbc_ld(c, A2, A3, 0);
                    rbc_sd(c, T0, 0, A2);
                    rbc_subiu(c, A2, T5, 0x6dd8);
                    rbc_lw(c, V1, SP, 0x1120);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, T0, SP, 0x1140);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, T0, 0);
                    rbc_lw(c, A1, SP, 0x1144);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, T1, T0, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x608, V1);
                    pc = 0x00110688;
                }
                0x00110688 => {
                    if rbc_copy32_loop(c, 0x00110688, [T2, T3, T5, V0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x001106b4;
                }
                0x001106b4 => {
                    c.cycles += 28;
                    rbc_lw(c, V1, SP, 0x1140);
                    c.g[A0][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0);
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x6dd8);
                    rbc_sd(c, A1, 0x8, T0);
                    rbc_subiu(c, A0, T5, 0x6f18);
                    rbc_lw(c, A1, SP, 0x1144);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x6ed0);
                    rbc_lw(c, V1, S3, 0x14);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x1174);
                    rbc_sh(c, SP, 0x658, V0);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S3, 0x14);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S3, 0x14);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110724;
                }
                0x00110724 => {
                    if rbc_copy32_loop(c, 0x00110724, [T1, T3, T5, V0], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110750;
                }
                0x00110750 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S3, 0x14);
                    c.g[V1][0] = 0x460000;
                    rbc_ld(c, A1, A3, 0);
                    rbc_subiu(c, A0, V1, 0x6f18);
                    rbc_sd(c, T0, 0, A1);
                    c.g[T3][0] = 0x460000;
                    rbc_subiu(c, A2, T3, 0x6ed0);
                    rbc_lw(c, T5, SP, 0x1174);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, T5);
                    rbc_lw(c, V0, S3, 0x14);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x20);
                    rbc_lw(c, A1, SP, 0x119c);
                    rbc_subu(c, T1, S3, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x708, V1);
                    pc = 0x00110794;
                }
                0x00110794 => {
                    if rbc_copy32_loop(c, 0x00110794, [V0, V1, A0, T0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x001107c0;
                }
                0x001107c0 => {
                    c.cycles += 27;
                    rbc_lw(c, V0, S3, 0x14);
                    c.g[T2][0] = 0x460000;
                    rbc_ld(c, T3, A2, 0);
                    rbc_ld(c, T5, A2, 0x8);
                    rbc_sd(c, A1, 0, T3);
                    rbc_subiu(c, V1, T2, 0x6ed0);
                    rbc_sd(c, A1, 0x8, T5);
                    c.g[T0][0] = 0x460000;
                    rbc_subiu(c, A0, T0, 0x7010);
                    c.g[T3][0] = 0x460000;
                    rbc_lw(c, T2, SP, 0x119c);
                    rbc_subiu(c, A1, T3, 0x6470);
                    c.g[T5][0] = 0x460000;
                    c.g[A3][0] = c.g[A0][0];
                    rbc_sw(c, V0, 0x6c0, T2);
                    rbc_subiu(c, A2, T5, 0x6fc8);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_lw(c, V1, S5, 0x58);
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x1128);
                    rbc_sh(c, SP, 0x758, V0);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S5, 0x58);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S5, 0x58);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x0011082c;
                }
                0x0011082c => {
                    if rbc_copy32_loop(c, 0x0011082c, [V0, V1, A0, A1], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110858;
                }
                0x00110858 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S5, 0x58);
                    c.g[A2][0] = 0x460000;
                    rbc_ld(c, T1, A3, 0);
                    rbc_subiu(c, A0, A2, 0x7010);
                    rbc_sd(c, T0, 0, T1);
                    c.g[V1][0] = 0x460000;
                    rbc_subiu(c, A2, V1, 0x6fc8);
                    rbc_lw(c, A1, SP, 0x1128);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, A1);
                    rbc_lw(c, A1, SP, 0x1150);
                    rbc_lw(c, V0, S5, 0x58);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x60);
                    rbc_subu(c, T1, S5, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x808, V1);
                    pc = 0x0011089c;
                }
                0x0011089c => {
                    if rbc_copy32_loop(c, 0x0011089c, [T0, T2, T3, T5], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x001108c8;
                }
                0x001108c8 => {
                    c.cycles += 27;
                    rbc_lw(c, V0, S5, 0x58);
                    c.g[A0][0] = 0x460000;
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x6fc8);
                    rbc_sd(c, A1, 0x8, T0);
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, A1, SP, 0x1150);
                    rbc_subiu(c, A0, T5, 0x7108);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x70c0);
                    rbc_lw(c, V1, S4, 0x14);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x1180);
                    rbc_sh(c, SP, 0x858, V0);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S4, 0x14);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S4, 0x14);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110934;
                }
                0x00110934 => {
                    if rbc_copy32_loop(c, 0x00110934, [T1, T3, T5, V0], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110960;
                }
                0x00110960 => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S4, 0x14);
                    c.g[V1][0] = 0x460000;
                    rbc_ld(c, A1, A3, 0);
                    rbc_subiu(c, A0, V1, 0x7108);
                    rbc_sd(c, T0, 0, A1);
                    c.g[T3][0] = 0x460000;
                    rbc_subiu(c, A2, T3, 0x70c0);
                    rbc_lw(c, T5, SP, 0x1180);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, T5);
                    rbc_lw(c, V0, S4, 0x14);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x20);
                    rbc_lw(c, A1, SP, 0x11a4);
                    rbc_subu(c, T1, S4, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0x908, V1);
                    pc = 0x001109a4;
                }
                0x001109a4 => {
                    if rbc_copy32_loop(c, 0x001109a4, [V0, V1, A0, T0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x001109d0;
                }
                0x001109d0 => {
                    c.cycles += 28;
                    rbc_lw(c, V0, S4, 0x14);
                    c.g[T2][0] = 0x460000;
                    rbc_ld(c, T3, A2, 0);
                    rbc_ld(c, T5, A2, 0x8);
                    rbc_sd(c, A1, 0, T3);
                    rbc_subiu(c, V1, T2, 0x70c0);
                    rbc_sd(c, A1, 0x8, T5);
                    c.g[T0][0] = 0x460000;
                    rbc_subiu(c, A0, T0, 0x7200);
                    c.g[T3][0] = 0x460000;
                    rbc_lw(c, T2, SP, 0x11a4);
                    rbc_subiu(c, A1, T3, 0x6470);
                    c.g[T5][0] = 0x460000;
                    c.g[A3][0] = c.g[A0][0];
                    rbc_sw(c, V0, 0x6c0, T2);
                    rbc_subiu(c, A2, T5, 0x71b8);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_sw_zero(c, S4, 0);
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, V1, S7, 0x8);
                    rbc_sh(c, SP, 0x958, V0);
                    rbc_lw(c, T0, SP, 0x1130);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S7, 0x8);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S7, 0x8);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110a40;
                }
                0x00110a40 => {
                    if rbc_copy32_loop(c, 0x00110a40, [V0, V1, A0, A1], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110a6c;
                }
                0x00110a6c => {
                    c.cycles += 17;
                    rbc_lw(c, V0, S7, 0x8);
                    c.g[A2][0] = 0x460000;
                    rbc_ld(c, T1, A3, 0);
                    c.g[V1][0] = 0x460000;
                    rbc_sd(c, T0, 0, T1);
                    rbc_subiu(c, A0, A2, 0x7200);
                    rbc_subiu(c, A3, V1, 0x71b8);
                    rbc_lw(c, A1, SP, 0x1130);
                    rbc_addiu(c, T0, A3, 0xa0);
                    rbc_sw(c, V0, 0x6e8, A1);
                    rbc_lw(c, A1, SP, 0x1154);
                    rbc_lw(c, V0, S7, 0x8);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, A2, S7, V0);
                    rbc_addu(c, V1, V1, A2);
                    rbc_sh(c, SP, 0xa08, V1);
                    pc = 0x00110ab0;
                }
                0x00110ab0 => {
                    if rbc_copy32_loop(c, 0x00110ab0, [T1, T2, T3, T5], A3, A1, T0) {
                        return Ok(());
                    }
                    pc = 0x00110adc;
                }
                0x00110adc => {
                    c.cycles += 32;
                    rbc_lw(c, V0, S7, 0x8);
                    c.g[A0][0] = 0x460000;
                    rbc_ld(c, T0, A3, 0);
                    rbc_ld(c, T1, A3, 0x8);
                    rbc_sd(c, A1, 0, T0);
                    rbc_subiu(c, V1, A0, 0x71b8);
                    rbc_sd(c, A1, 0x8, T1);
                    c.g[A1][0] = 0u64;
                    rbc_lw(c, T5, SP, 0x1154);
                    rbc_lw(c, A0, SP, 0x1178);
                    rbc_sw(c, V0, 0x6c0, T5);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_addu(c, V0, V0, A2);
                    rbc_sh(c, SP, 0xa58, V0);
                    call_ptr(c, 0x00142388, 0x00110b18)?; // RiderWipeoutSub_ctor
                    rbc_lw(c, V0, SP, 0x11bc);
                    c.g[A0][0] = 0x460000;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0x460000;
                    rbc_lw(c, V1, V0, 0x8);
                    rbc_subiu(c, V0, A0, 0x75e0);
                    rbc_sw(c, V1, 0x6e8, V0);
                    rbc_subiu(c, A0, A1, 0x6470);
                    rbc_subiu(c, A1, A2, 0x7598);
                    rbc_lw(c, T1, SP, 0x11bc);
                    c.g[A2][0] = c.g[V0][0];
                    rbc_lw(c, A3, SP, 0x118c);
                    rbc_addiu(c, T0, A2, 0x40);
                    rbc_lw(c, V0, T1, 0x8);
                    rbc_sw(c, V0, 0x6d0, A0);
                    rbc_lw(c, V1, T1, 0x8);
                    rbc_sw(c, V1, 0x6c0, A1);
                    pc = 0x00110b5c;
                }
                0x00110b5c => {
                    if rbc_copy32_loop(c, 0x00110b5c, [T2, T3, T5, V0], A2, A3, T0) {
                        return Ok(());
                    }
                    pc = 0x00110b88;
                }
                0x00110b88 => {
                    c.cycles += 19;
                    rbc_lw(c, V1, SP, 0x11bc);
                    c.g[A1][0] = 0x460000;
                    rbc_subiu(c, A0, A1, 0x75e0);
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0x8);
                    rbc_ld(c, T0, A2, 0);
                    rbc_sd(c, A3, 0, T0);
                    rbc_subiu(c, A2, T5, 0x7598);
                    rbc_lw(c, V1, SP, 0x118c);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, T0, SP, 0x11bc);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, T0, 0x8);
                    rbc_lw(c, A1, SP, 0x11ac);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, T1, T0, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0xb08, V1);
                    pc = 0x00110bd4;
                }
                0x00110bd4 => {
                    if rbc_copy32_loop(c, 0x00110bd4, [T2, T3, T5, V0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110c00;
                }
                0x00110c00 => {
                    c.cycles += 30;
                    rbc_lw(c, V1, SP, 0x11bc);
                    c.g[A0][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0x8);
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x7598);
                    rbc_sd(c, A1, 0x8, T0);
                    rbc_subiu(c, A0, T5, 0x74e8);
                    rbc_lw(c, A1, SP, 0x11ac);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x74a0);
                    rbc_lw(c, T0, SP, 0x11c8);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_lw(c, V1, T0, 0x4);
                    rbc_sh(c, SP, 0xb58, V0);
                    rbc_lw(c, T0, SP, 0x113c);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, T1, SP, 0x11c8);
                    rbc_lw(c, V0, T1, 0x4);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, T1, 0x4);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110c78;
                }
                0x00110c78 => {
                    if rbc_copy32_loop(c, 0x00110c78, [T3, T5, V0, V1], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110ca4;
                }
                0x00110ca4 => {
                    c.cycles += 19;
                    rbc_lw(c, A0, SP, 0x11c8);
                    c.g[A1][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, A0, 0x4);
                    rbc_ld(c, A2, A3, 0);
                    rbc_subiu(c, A0, A1, 0x74e8);
                    rbc_sd(c, T0, 0, A2);
                    rbc_subiu(c, A2, T5, 0x74a0);
                    rbc_lw(c, V1, SP, 0x113c);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, T0, SP, 0x11c8);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, T0, 0x4);
                    rbc_lw(c, A1, SP, 0x115c);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, T1, T0, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0xc08, V1);
                    pc = 0x00110cf0;
                }
                0x00110cf0 => {
                    if rbc_copy32_loop(c, 0x00110cf0, [T2, T3, T5, V0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110d1c;
                }
                0x00110d1c => {
                    c.cycles += 30;
                    rbc_lw(c, V1, SP, 0x11c8);
                    c.g[A0][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0x4);
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x74a0);
                    rbc_sd(c, A1, 0x8, T0);
                    rbc_subiu(c, A0, T5, 0x77d0);
                    rbc_lw(c, A1, SP, 0x115c);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x7788);
                    rbc_lw(c, T0, SP, 0x1104);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_lw(c, V1, T0, 0x14);
                    rbc_sh(c, SP, 0xc58, V0);
                    rbc_lw(c, T0, SP, 0x1194);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, T1, SP, 0x1104);
                    rbc_lw(c, V0, T1, 0x14);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, T1, 0x14);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110d94;
                }
                0x00110d94 => {
                    if rbc_copy32_loop(c, 0x00110d94, [T3, T5, V0, V1], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110dc0;
                }
                0x00110dc0 => {
                    c.cycles += 19;
                    rbc_lw(c, A0, SP, 0x1104);
                    c.g[A1][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, A0, 0x14);
                    rbc_ld(c, A2, A3, 0);
                    rbc_subiu(c, A0, A1, 0x77d0);
                    rbc_sd(c, T0, 0, A2);
                    rbc_subiu(c, A2, T5, 0x7788);
                    rbc_lw(c, V1, SP, 0x1194);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, T0, SP, 0x1104);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, T0, 0x14);
                    rbc_lw(c, A1, SP, 0x11b8);
                    rbc_subiu(c, V0, V0, 0x20);
                    rbc_subu(c, T1, T0, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0xd08, V1);
                    pc = 0x00110e0c;
                }
                0x00110e0c => {
                    if rbc_copy32_loop(c, 0x00110e0c, [T2, T3, T5, V0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110e38;
                }
                0x00110e38 => {
                    c.cycles += 30;
                    rbc_lw(c, V1, SP, 0x1104);
                    c.g[A0][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0x14);
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x7788);
                    rbc_sd(c, A1, 0x8, T0);
                    rbc_subiu(c, A0, T5, 0x72f8);
                    rbc_lw(c, A1, SP, 0x11b8);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x72b0);
                    rbc_lw(c, T0, SP, 0x1114);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_lw(c, V1, T0, 0x8);
                    rbc_sh(c, SP, 0xd58, V0);
                    rbc_lw(c, T0, SP, 0x1148);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, T1, SP, 0x1114);
                    rbc_lw(c, V0, T1, 0x8);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, T1, 0x8);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110eb0;
                }
                0x00110eb0 => {
                    if rbc_copy32_loop(c, 0x00110eb0, [T3, T5, V0, V1], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110edc;
                }
                0x00110edc => {
                    c.cycles += 19;
                    rbc_lw(c, A0, SP, 0x1114);
                    c.g[A1][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, A0, 0x8);
                    rbc_ld(c, A2, A3, 0);
                    rbc_subiu(c, A0, A1, 0x72f8);
                    rbc_sd(c, T0, 0, A2);
                    rbc_subiu(c, A2, T5, 0x72b0);
                    rbc_lw(c, V1, SP, 0x1148);
                    rbc_addiu(c, A3, A2, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, T0, SP, 0x1114);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, T0, 0x8);
                    rbc_lw(c, A1, SP, 0x1160);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, T1, T0, V0);
                    rbc_addu(c, V1, V1, T1);
                    rbc_sh(c, SP, 0xe08, V1);
                    pc = 0x00110f28;
                }
                0x00110f28 => {
                    if rbc_copy32_loop(c, 0x00110f28, [T2, T3, T5, V0], A2, A1, A3) {
                        return Ok(());
                    }
                    pc = 0x00110f54;
                }
                0x00110f54 => {
                    c.cycles += 28;
                    rbc_lw(c, V1, SP, 0x1114);
                    c.g[A0][0] = 0x460000;
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0x8);
                    rbc_ld(c, A3, A2, 0);
                    rbc_ld(c, T0, A2, 0x8);
                    rbc_sd(c, A1, 0, A3);
                    rbc_subiu(c, V1, A0, 0x72b0);
                    rbc_sd(c, A1, 0x8, T0);
                    rbc_subiu(c, A0, T5, 0x76d8);
                    rbc_lw(c, A1, SP, 0x1160);
                    c.g[A2][0] = 0x460000;
                    c.g[A3][0] = 0x460000;
                    rbc_sw(c, V0, 0x6c0, A1);
                    rbc_subiu(c, A1, A2, 0x6470);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_subiu(c, A2, A3, 0x7690);
                    rbc_lw(c, V1, S0, 0x3a0);
                    c.g[A3][0] = c.g[A0][0];
                    rbc_addu(c, V0, V0, T1);
                    rbc_lw(c, T0, SP, 0x11a0);
                    rbc_sh(c, SP, 0xe58, V0);
                    rbc_addiu(c, T2, A3, 0x40);
                    rbc_sw(c, V1, 0x6e8, A0);
                    rbc_lw(c, V0, S0, 0x3a0);
                    rbc_sw(c, V0, 0x6d0, A1);
                    rbc_lw(c, V1, S0, 0x3a0);
                    rbc_sw(c, V1, 0x6c0, A2);
                    pc = 0x00110fc4;
                }
                0x00110fc4 => {
                    if rbc_copy32_loop(c, 0x00110fc4, [T1, T3, T5, V0], A3, T0, T2) {
                        return Ok(());
                    }
                    pc = 0x00110ff0;
                }
                0x00110ff0 => {
                    c.cycles += 19;
                    rbc_lw(c, V1, SP, 0x111c);
                    c.g[A1][0] = 0x460000;
                    rbc_subiu(c, A0, A1, 0x76d8);
                    c.g[T5][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0);
                    rbc_ld(c, A2, A3, 0);
                    rbc_sd(c, T0, 0, A2);
                    rbc_subiu(c, A3, T5, 0x7690);
                    rbc_lw(c, V1, SP, 0x11a0);
                    rbc_addiu(c, T0, A3, 0xa0);
                    rbc_sw(c, V0, 0x6e8, V1);
                    rbc_lw(c, A2, SP, 0x111c);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_lw(c, V0, A2, 0);
                    rbc_lw(c, A1, SP, 0x11c4);
                    rbc_subiu(c, V0, V0, 0x10);
                    rbc_subu(c, A2, A2, V0);
                    rbc_addu(c, V1, V1, A2);
                    rbc_sh(c, SP, 0xf08, V1);
                    pc = 0x0011103c;
                }
                0x0011103c => {
                    if rbc_copy32_loop(c, 0x0011103c, [T1, T2, T3, T5], A3, A1, T0) {
                        return Ok(());
                    }
                    pc = 0x00111068;
                }
                0x00111068 => {
                    c.cycles += 52;
                    rbc_lw(c, V1, SP, 0x111c);
                    c.g[A0][0] = 0x460000;
                    rbc_lw(c, V0, V1, 0);
                    rbc_ld(c, T0, A3, 0);
                    rbc_ld(c, T1, A3, 0x8);
                    rbc_sd(c, A1, 0, T0);
                    rbc_subiu(c, V1, A0, 0x7690);
                    rbc_sd(c, A1, 0x8, T1);
                    c.g[A1][0] = 0u64;
                    rbc_lw(c, T5, SP, 0x11c4);
                    rbc_lw(c, A0, SP, 0x1124);
                    rbc_sw(c, V0, 0x6c0, T5);
                    rbc_lhu(c, V0, V1, 0x8);
                    rbc_addu(c, V0, V0, A2);
                    rbc_sh(c, SP, 0xf58, V0);
                    call_ptr(c, 0x002dcb88, 0x001110a8)?; // cBoardWakeFX_ctor
                    rbc_lw(c, A0, SP, 0x1168);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002da390, 0x001110b4)?; // cBoardSparkFX_ctor
                    rbc_lw(c, A0, SP, 0x11b0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e81e8, 0x001110c0)?; // cBoardTrailFX_ctor
                    rbc_lw(c, A0, SP, 0x1158);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e6320, 0x001110cc)?; // cSpeedBoostStreamerFX_ctor
                    rbc_lw(c, A0, SP, 0x114c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002eaae0, 0x001110d8)?; // RiderBase_CtorPowerUpTrail
                    rbc_lw(c, A0, SP, 0x11a8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002ef3b0, 0x001110e4)?; // cAirStreamerFX_ctor
                    rbc_lw(c, A0, SP, 0x11b4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002d48f0, 0x001110f0)?; // cAngerFX_ctor
                    rbc_lw(c, A0, SP, 0x11c0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e3680, 0x001110fc)?; // cPlayerPulseFX_ctor
                    rbc_lw(c, A0, SP, 0x1118);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002de4a8, 0x00111108)?; // cDynamicSprayFX_ctor
                    rbc_lw(c, A0, SP, 0x1184);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002f0e88, 0x00111114)?; // RiderBase_CtorAttackFX
                    rbc_lw(c, A0, SP, 0x110c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002f6238, 0x00111120)?; // RiderBase_CtorCheatCode
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = 0x4;
                    rbc_lw(c, A1, S0, 0x18);
                    c.g[V0][0] = 0x4;
                    c.g[V1][0] = 0xffffffffffffffff;
                    pc = 0x00111138;
                }
                0x00111138 => {
                    c.cycles += 7;
                    rbc_subiu(c, V0, V0, 0x1);
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A3][0] = 0x460000;
                    if t && back_edge_unwinds(c, 0x00111138) {
                        return Ok(());
                    }
                    pc = if t { 0x00111138 } else { 0x00111154 };
                }
                0x00111154 => {
                    c.cycles += 12;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = 0x460000;
                    rbc_subiu(c, A2, V0, 0x78c8);
                    rbc_subiu(c, A0, V1, 0x6470);
                    rbc_sw(c, A1, 0x6e8, A2);
                    rbc_subiu(c, A1, A3, 0x7880);
                    rbc_lw(c, T0, SP, 0x1100);
                    rbc_lw(c, V0, S0, 0x18);
                    rbc_sw(c, V0, 0x6d0, A0);
                    rbc_lw(c, V1, S0, 0x18);
                    let t = c.g[T0][0] != 0u64;
                    rbc_sw(c, V1, 0x6c0, A1);
                    pc = if t { 0x0011125c } else { 0x00111184 };
                }
                0x00111184 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[A2][0];
                    rbc_addiu(c, V1, SP, 0x1000);
                    rbc_addiu(c, V0, A1, 0x40);
                    c.g[A2][0] = c.g[V1][0];
                    rbc_addiu(c, T0, SP, 0x1050);
                    pc = 0x00111198;
                }
                0x00111198 => {
                    if rbc_copy32_loop(c, 0x00111198, [T1, T2, T3, T5], A1, V1, V0) {
                        return Ok(());
                    }
                    pc = 0x001111c4;
                }
                0x001111c4 => {
                    c.cycles += 16;
                    rbc_lw(c, V0, S0, 0x18);
                    c.g[A3][0] = 0x460000;
                    rbc_ld(c, T1, A1, 0);
                    rbc_subiu(c, A0, A3, 0x78c8);
                    rbc_sd(c, V1, 0, T1);
                    rbc_sw(c, V0, 0x6e8, A2);
                    c.g[V1][0] = 0x460000;
                    rbc_subiu(c, A3, V1, 0x7880);
                    c.g[A2][0] = c.g[T0][0];
                    rbc_lw(c, V0, S0, 0x18);
                    rbc_addiu(c, T1, A3, 0xa0);
                    rbc_lhu(c, V1, A0, 0x8);
                    rbc_subiu(c, V0, V0, 0xdf0);
                    rbc_subu(c, A1, S0, V0);
                    rbc_addu(c, V1, V1, A1);
                    rbc_sh(c, SP, 0x1008, V1);
                    pc = 0x00111204;
                }
                0x00111204 => {
                    if rbc_copy32_loop(c, 0x00111204, [A0, T2, T3, T5], A3, A2, T1) {
                        return Ok(());
                    }
                    pc = 0x00111230;
                }
                0x00111230 => {
                    c.cycles += 11;
                    c.g[V1][0] = 0x460000;
                    rbc_lw(c, A0, S0, 0x18);
                    rbc_subiu(c, V0, V1, 0x7880);
                    rbc_ld(c, T1, A3, 0);
                    rbc_ld(c, T2, A3, 0x8);
                    rbc_sd(c, A2, 0, T1);
                    rbc_lhu(c, V1, V0, 0x8);
                    rbc_sd(c, A2, 0x8, T2);
                    rbc_sw(c, A0, 0x6c0, T0);
                    rbc_addu(c, V1, V1, A1);
                    rbc_sh(c, SP, 0x1058, V1);
                    pc = 0x0011125c;
                }
                0x0011125c => {
                    c.cycles += 22;
                    c.g[V0][0] = 0x460000;
                    rbc_lw(c, V1, S0, 0x18);
                    rbc_subiu(c, V0, V0, 0x78f0);
                    c.g[A0][0] = 0x6;
                    rbc_sw(c, S0, 0xde8, V0);
                    c.g[A1][0] = 0xe;
                    rbc_sw(c, V1, 0x77c, S0);
                    c.g[V0][0] = c.g[S0][0];
                    rbc_sw(c, S0, 0xde0, A0);
                    rbc_sw(c, S0, 0xde4, A1);
                    c.lq(16, sp(c).wrapping_add(0x1260));
                    c.lq(17, sp(c).wrapping_add(0x1250));
                    c.lq(18, sp(c).wrapping_add(0x1240));
                    c.lq(19, sp(c).wrapping_add(0x1230));
                    c.lq(20, sp(c).wrapping_add(0x1220));
                    c.lq(21, sp(c).wrapping_add(0x1210));
                    c.lq(22, sp(c).wrapping_add(0x1200));
                    c.lq(23, sp(c).wrapping_add(0x11f0));
                    c.lq(30, sp(c).wrapping_add(0x11e0));
                    rbc_ld(c, RA, SP, 0x11d0);
                    let t = c.g[RA][0] as u32;
                    rbc_addiu(c, SP, SP, 0x1270);
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0010_fcd8, 0x0011_12b4);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    /// The engine's size-class pool allocator (enabled through `gp + 0xdd8`)
    /// plus the dlmalloc arena the sub-constructors' allocations fall back
    /// to (the heap at 0x4ff250, backed by `DATA+0x40000..+0x80000`).
    fn rbc_test_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            for n in 0..16u32 {
                let node = nodes + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, nodes);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
    }

    /// `RiderBase_ctor` against the generated body: `in_chrg` 0 and 1, with
    /// the game image loaded (the state templates, `.lit4` constants) and
    /// the real sub-constructors running on zeroed data. The back end's
    /// singletons are dummies.
    #[test]
    fn rider_base_ctor_matches() {
        check(ssx3_ee::s00::f_0010fcd8__generated, rider_base_ctor, 6, &|c, seed| {
            let mut r = Rng(0x5eed_0409 ^ seed as u64);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9_0000);
            c.bus.write32(GAME_GP - 0x848, DATA + 0xa_0000);
            rbc_test_allocator(c);
            let this = DATA + 0x1_0000 + 0x1000 * (r.next_u32() % 8);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.pc = 0x0010_fcd8;
        });
    }
}
