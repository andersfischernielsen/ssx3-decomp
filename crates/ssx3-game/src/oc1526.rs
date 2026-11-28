//! Staging module oc1526 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1526`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_0008, crt_entry, "entry"),
    (0x0015_0f98, beecon_raise_attribute_level, "lbl_00150F98"),
    (0x0015_1310, beecon_attribute_cost_word, "lbl_00151310"),
    (
        0x001c_eea0,
        c_fe_state_reward_gallery_ctor,
        "cFEStateRewardGallery_ctor",
    ),
];

/// `entry` (0x00100008): the program entry. Clears the BSS (0x4a4c00..0x53f115, 16 bytes
/// at a time, with a back-edge event check), sets up the main thread (syscall 0x3c: gp, stack,
/// heap arguments) and the heap (syscall 0x3d), runs two init calls, enables interrupts, then
/// calls the game main (0x31af80) with the argument block at 0x4a5c00 and tail-calls `exit`.
/// Resumable: any `pc` of the generated `__entries` twin continues natively.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn crt_entry(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    // The straight-line twin charges the tail's 32 cycles before the first syscall; the
    // resumable twin charges them per block. Both are 105 + the loop, but the calls see
    // `cycles`, so a run from the start charges up front.
    let straight = pc == 0x0010_0008;
    loop {
        match pc {
            0x00100008 => {
                c.cycles += 62;
                c.padduw(1, 0, 0);
                c.padduw(2, 0, 0);
                c.padduw(3, 0, 0);
                c.padduw(4, 0, 0);
                c.padduw(5, 0, 0);
                c.padduw(6, 0, 0);
                c.padduw(7, 0, 0);
                c.padduw(8, 0, 0);
                c.padduw(9, 0, 0);
                c.padduw(10, 0, 0);
                c.padduw(11, 0, 0);
                c.padduw(12, 0, 0);
                c.padduw(13, 0, 0);
                c.padduw(14, 0, 0);
                c.padduw(15, 0, 0);
                c.padduw(16, 0, 0);
                c.padduw(17, 0, 0);
                c.padduw(18, 0, 0);
                c.padduw(19, 0, 0);
                c.padduw(20, 0, 0);
                c.padduw(21, 0, 0);
                c.padduw(22, 0, 0);
                c.padduw(23, 0, 0);
                c.padduw(24, 0, 0);
                c.padduw(25, 0, 0);
                c.padduw(28, 0, 0);
                c.padduw(29, 0, 0);
                c.padduw(30, 0, 0);
                c.padduw(31, 0, 0);
                c.hi[0] = 0u64;
                c.hi[1] = 0u64;
                c.lo[0] = 0u64;
                c.lo[1] = 0u64;
                c.mtsah(0, 0);
                c.mtc1(0, 0);
                c.mtc1(0, 1);
                c.mtc1(0, 2);
                c.mtc1(0, 3);
                c.mtc1(0, 4);
                c.mtc1(0, 5);
                c.mtc1(0, 6);
                c.mtc1(0, 7);
                c.mtc1(0, 8);
                c.mtc1(0, 9);
                c.mtc1(0, 10);
                c.mtc1(0, 11);
                c.mtc1(0, 12);
                c.mtc1(0, 13);
                c.mtc1(0, 14);
                c.mtc1(0, 15);
                c.mtc1(0, 16);
                c.mtc1(0, 17);
                c.mtc1(0, 18);
                c.mtc1(0, 19);
                c.mtc1(0, 20);
                c.mtc1(0, 21);
                c.mtc1(0, 22);
                c.mtc1(0, 23);
                c.mtc1(0, 24);
                c.mtc1(0, 25);
                c.mtc1(0, 26);
                c.mtc1(0, 27);
                pc = 0x00100100; continue;
            }
            0x00100100 => {
                c.cycles += 11;
                c.mtc1(0, 28);
                c.mtc1(0, 29);
                c.mtc1(0, 30);
                c.mtc1(0, 31);
                c.adda_s(0, 1);
                c.ctc1(0, 31);
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = 0x540000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4c00));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeeb));
                pc = 0x0010012c; continue;
            }
            0x0010012c => {
                c.cycles += 7;
                c.sq(0, (c.g[V0][0] as u32));
                c.g[AT][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                let t = c.g[AT][0] != 0u64; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                if t { if back_edge_unwinds(c, 0x0010012c) { return Ok(()); } pc = 0x0010012c; continue; }
                pc = 0x00100148; continue;
            }
            0x00100148 => {
                c.cycles += if straight { 32 } else { 22 };
                c.g[A0][0] = 0x4a0000;
                c.g[A1][0] = 0x1fe0000;
                c.g[A2][0] = 0x20000;
                c.g[A3][0] = 0x4a0000;
                c.g[T0][0] = 0x100000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30f0));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x0));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x0));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5c00));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1d0));
                c.g[GP][0] = c.g[A0][0];
                c.g[V1][0] = 0x3c;
                c.pc = 0x0010017c; c.syscall(0x00100178); if c.pc != 0x0010017c { return Ok(()); }
                if event_due(c) { deliver_event_here(c, 0x0010017c); }
                c.g[SP][0] = c.g[V0][0];
                c.g[A0][0] = 0x540000;
                c.g[A1][0] = 0x0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xeeb));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = 0x3d;
                c.pc = 0x00100198; c.syscall(0x00100194); if c.pc != 0x00100198 { return Ok(()); }
                if event_due(c) { deliver_event_here(c, 0x00100198); }
                c.g[RA][0] = sx(0x001001a0);
                call_ptr(c, 0x0042c300, 0x001001a0)?;
                pc = 0x001001a0; continue;
            }
            0x001001a0 => {
                if !straight { c.cycles += 2; }
                c.g[RA][0] = sx(0x001001a8);
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x00424020, 0x001001a8)?;
                pc = 0x001001a8; continue;
            }
            0x001001a8 => {
                if !straight { c.cycles += 6; }
                c.ei();
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5c00));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x001001c0);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x0031af80, 0x001001c0)?;
                pc = 0x001001c0; continue;
            }
            0x001001c0 => {
                if !straight { c.cycles += 2; }
                c.g[A0][0] = c.g[V0][0];
                c.pc = 0x0042c6e8; call_indirect(c, 0x0042c6e8); return Ok(());
            }
            _ => { c.jump_out(pc, 0x00100008, 0x001001c8); return Ok(()); }
        }
    }
    });
}

/// `cFEStateRewardGallery_ctor` (0x001ceea0): constructs the reward gallery screen state:
/// base `cFEState` ctor, vtable and type fields, the table clears (7 words in each of 4 rows of
/// two arrays), the reward-category lookup (a jump table over the result of 0x146d98), the
/// virtual call on the looked-up object and the final stored lookups.
/// Resumable: any `pc` of the generated `__entries` twin continues natively.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn c_fe_state_reward_gallery_ctor(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x001ceea0 => {
                c.cycles += 54;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[T0][0];
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x001ceed0);
                c.g[S3][0] = c.g[A3][0];
                call_ptr(c, 0x0039e2a0, 0x001ceed0)?;
                c.g[V0][0] = 0x2f;
                c.g[V1][0] = 0x470000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5c38));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.g[A1][0] = c.g[S0][0];
                c.sb((c.g[S2][0] as u32).wrapping_add(0x44), c.g[S0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x001ceef8);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x001a1cd0, 0x001ceef8)?;
                c.g[V1][0] = 0x4;
                c.g[A0][0] = 0x8;
                c.g[A1][0] = 0xfffffffffffffffe;
                c.sb((c.g[S2][0] as u32).wrapping_add(0x15), c.g[V0][0]);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[S1][0]);
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x924));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x9a4));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[A0][0]);
                c.g[A2][0] = 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8ec), c.g[A1][0]);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x64), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8f4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8f8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x70), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x74), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8fc), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x900), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x904), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x908), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x90c), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x910), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x914), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x918), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x91c), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x920), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x68), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x6c), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x78), 0u64);
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 5);
                pc = 0x001cef78; continue;
            }
            0x001cef78 => {
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.g[A0][0] = 0x7;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                pc = 0x001cef88; continue;
            }
            0x001cef88 => {
                c.cycles += 7;
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = (c.g[A0][0] as i64) >= 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x001cef88) { return Ok(()); } pc = 0x001cef88; continue; }
                pc = 0x001cefa4; continue;
            }
            0x001cefa4 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[A1][0];
                c.g[V0][0] = ((c.g[A2][0] as i64) < 4i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 5);
                if t { if back_edge_unwinds(c, 0x001cef78) { return Ok(()); } pc = 0x001cef78; continue; }
                pc = 0x001cefb4; continue;
            }
            0x001cefb4 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x001cefbc);
                call_ptr(c, 0x0014dc80, 0x001cefbc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001cefc8);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x001cefc8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                c.g[RA][0] = sx(0x001cefd8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00147488, 0x001cefd8)?;
                let t = c.g[S3][0] == 0u64; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                if t { pc = 0x001cf04c; continue; }
                pc = 0x001cefe0; continue;
            }
            0x001cefe0 => {
                c.cycles += 9;
                c.g[RA][0] = sx(0x001cefe8);
                c.g[A0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                call_ptr(c, 0x0014a130, 0x001cefe8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001ceff4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00146d98, 0x001ceff4)?;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11));
                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x460000;
                if t { pc = 0x001cf044; continue; }
                pc = 0x001cf004; continue;
            }
            0x001cf004 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6d70));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            0x001cf01c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x001cf050; continue; }
                pc = 0x001cf024; continue;
            }
            0x001cf024 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x001cf050; continue; }
                pc = 0x001cf02c; continue;
            }
            0x001cf02c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x001cf050; continue; }
                pc = 0x001cf034; continue;
            }
            0x001cf034 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x001cf050; continue; }
                pc = 0x001cf03c; continue;
            }
            0x001cf03c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x3;
                if t { pc = 0x001cf050; continue; }
                pc = 0x001cf044; continue;
            }
            0x001cf044 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x001cf050; continue; }
                pc = 0x001cf04c; continue;
            }
            0x001cf04c => {
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                pc = 0x001cf050; continue;
            }
            0x001cf050 => {
                c.cycles += 24;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8f0), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001cf068);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001cf068)?;
                c.g[RA][0] = sx(0x001cf070);
                call_ptr(c, 0x0014dc80, 0x001cf070)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001cf07c);
                c.g[A1][0] = 0xb;
                call_ptr(c, 0x0014dd58, 0x001cf07c)?;
                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001cf08c);
                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                call_ptr(c, 0x00150928, 0x001cf08c)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                c.g[V0][0] = c.g[S2][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001ceea0, 0x001cf0b0); return Ok(()); }
        }
    }
    });
}

/// `lbl_00150f98` (0x00150f98): raises one per-rider level byte
/// (`0x4a6ca8 + 0xf88*t4 + 0x9b50*t3 + 0xbe5`); at 5 (the maximum) it returns 0 untouched,
/// otherwise it adds the table word at `0x440550 + 8*((level-1)/5) + 4` to the word at
/// `cell + 0xac4`, increments the level and leaves v0 = 1.
/// Resumable: any `pc` of the generated `__entries` twin continues natively.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn beecon_raise_attribute_level(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00150f98 => {
                c.cycles += 9;
                c.g[V1][0] = 0xf88;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 3, 12, 3);
                c.mult(0, 4, 11, 4);
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe5));
                pc = 0x00150fbc; continue;
            }
            0x00150fbc => {
                c.cycles += 4;
                c.g[T0][0] = c.lb((c.g[T1][0] as u32));
                c.g[V0][0] = 0x5;
                let t = c.g[T0][0] == c.g[V0][0]; 
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                if t { pc = 0x00151034; continue; }
                pc = 0x00150fcc; continue;
            }
            0x00150fcc => {
                c.cycles += 10;
                c.g[A1][0] = 0xf88;
                c.div(0, 8, 2);
                c.g[A3][0] = 0x9b50;
                c.g[A2][0] = 0x440000;
                c.g[V1][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A0][0] = 0x5;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x550));
                
                if c.g[A0][0] == 0u64 {  pc = 0x00150ff4; continue; }
                pc = 0x00150ff4; continue;
            }
            0x00150ff4 => {
                c.cycles += 16;
                c.g[V0][0] = 0x1;
                c.g[T0][0] = c.lo[0];
                c.mult(0, 5, 12, 5);
                c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.mult(0, 7, 11, 7);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xac4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[A1][0] as u32).wrapping_add(0xac4), c.g[A0][0]);
                c.g[V1][0] = c.lbu((c.g[T1][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[RA][0] as u32;
                c.sb((c.g[T1][0] as u32), c.g[V1][0]);
                c.pc = t; return Ok(());
            }
            0x00151034 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0u64;
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00150f98, 0x0015103c); return Ok(()); }
        }
    }
    });
}

/// `lbl_00151310` (0x00151310): reads the per-rider level byte
/// (`0x4a6ca8 + 0xf88*a2 + 0x9b50*a1 + 0xbe5`) and returns the table word at
/// `0x440550 + 8*((level+1)/5) + 4`.
/// Resumable: any `pc` of the generated `__entries` twin continues natively.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn beecon_attribute_cost_word(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00151310 => {
                c.cycles += 9;
                c.g[V1][0] = 0xf88;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 3, 6, 3);
                c.mult(0, 4, 5, 4);
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe5));
                pc = 0x00151334; continue;
            }
            0x00151334 => {
                c.cycles += 5;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32));
                c.g[A0][0] = 0x5;
                c.g[V1][0] = 0x440000;
                
                if c.g[A0][0] == 0u64 {  pc = 0x00151348; continue; }
                pc = 0x00151348; continue;
            }
            0x00151348 => {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x550));
                c.div(0, 2, 4);
                c.g[V0][0] = c.lo[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00151310, 0x00151368); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, check_reaching};
    use crate::difftest::{check, elf_image, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1526_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC1526_MAGIC: u32 = 0x1526_f00d;

    /// Fixture objects, clear of the allocator pools, the arena and the call log.
    pub(crate) const OC1526_BASE: u32 = DATA + 0x9_0000;

    macro_rules! oc1526_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1526_CTRL) != OC1526_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1526_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1526_fakes! {
        oc1526_fake_init_a, 0x0042_c300, ee::s15::OVERRIDE_0042C300, ee::s15::f_0042c300__generated;
        oc1526_fake_init_b, 0x0042_4020, ee::s15::OVERRIDE_00424020, ee::s15::f_00424020__generated;
        oc1526_fake_main, 0x0031_af80, ee::s10::OVERRIDE_0031AF80, ee::s10::f_0031af80__generated;
        oc1526_fake_exit, 0x0042_c6e8, ee::s15::OVERRIDE_0042C6E8, ee::s15::f_0042c6e8__generated;
        oc1526_fake_state_ctor, 0x0039_e2a0, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
        oc1526_fake_get_a, 0x001a_1cd0, ee::s03::OVERRIDE_001A1CD0, ee::s03::f_001a1cd0__generated;
        oc1526_fake_get_b, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1526_fake_get_c, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1526_fake_get_d, 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc1526_fake_get_e, 0x0014_a130, ee::s01::OVERRIDE_0014A130, ee::s01::f_0014a130__generated;
        oc1526_fake_get_f, 0x0014_6d98, ee::s01::OVERRIDE_00146D98, ee::s01::f_00146d98__generated;
        oc1526_fake_get_g, 0x0015_0928, ee::s01::OVERRIDE_00150928, ee::s01::f_00150928__generated;
    }

    /// Fixture for `entry`: the init calls, the main call and `exit` are fakes, the two
    /// kernel syscalls go to the fake syscall hook; the function takes no inputs.
    pub(crate) fn oc1526_entry_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        oc1526_hook();
        let r = fakes::machine(c, seed, 0x1526, 6);
        c.bus.write32(OC1526_CTRL, OC1526_MAGIC);
        c.pc = 0x0010_0008;
        r
    }

    /// Clears the BSS, both syscalls, the init calls, main and the `exit` tail call.
    #[test]
    fn crt_entry_matches() {
        check_reaching(
            ee::s00::f_00100008__generated,
            crt_entry,
            12,
            &[0x0010_01a0, 0x0010_01a8, 0x0010_01c0],
            &|c, seed| {
                oc1526_entry_setup(c, seed);
            },
        );
    }

    /// Every resume point of the generated twin (after an unwound call or event) and a
    /// `pc` outside any block (the `jump_out` arm) continue the same way natively.
    #[test]
    fn crt_entry_resumes_match() {
        for pc in [
            0x0010_0100u32,
            0x0010_012c,
            0x0010_0148,
            0x0010_01a0,
            0x0010_01a8,
            0x0010_01c0,
            0x0010_0010,
        ] {
            check(
                ee::s00::f_00100008__generated,
                crt_entry,
                6,
                &|c, seed| {
                    oc1526_entry_setup(c, seed);
                    c.pc = pc;
                    if pc == 0x0010_012c {
                        // Inside the clear loop, a few stores from its end.
                        c.g[V0][0] = (0x0053_f100 - 16 * (seed % 4)) as u64;
                        c.g[V1][0] = 0x0053_f115;
                    }
                },
            );
        }
    }

    /// Fixture for the reward gallery constructor: the base constructor and every
    /// lookup are fakes with forced results (call 3 returns the looked-up object, call 6
    /// the category that picks the jump-table arm); `a3` and the byte argument vary.
    pub(crate) fn oc1526_ctor_setup(c: &mut Cpu, seed: u32, calls: u32) -> crate::difftest::Rng {
        oc1526_hook();
        let r = fakes::machine_rt(c, seed, 0x1526_c, calls);
        elf_image(c);
        c.bus.write32(OC1526_CTRL, OC1526_MAGIC);
        let h = fakes::mix(seed as u64 ^ 0x1526_0000);
        let b = OC1526_BASE;
        let (obj, vt, st) = (b + 0x1000, b + 0x2000, b + 0x3000);
        c.bus.write32(b + 0x10, st);
        c.bus.write32(obj + 0xc, vt);
        c.bus.write16(vt + 0x10, (((h >> 20) % 16) * 4) as u16);
        c.bus.write32(vt + 0x14, fakes::FAKE + 0x10);
        // Call 3 (the object lookup) returns `obj`.
        c.bus.write32(fakes::LOG_RESULTS + 4 * 3, obj);
        let s3_set = (h & 1) != 0;
        // With a3 set, call 6 returns the category: 0x11..0x15 pick a jump-table arm,
        // others fall through to the default.
        let cats = [0x11u32, 0x12, 0x13, 0x14, 0x15, 0x30, 0x10];
        c.bus.write32(
            fakes::LOG_RESULTS + 4 * 6,
            cats[((h >> 4) % cats.len() as u64) as usize],
        );
        c.g[A0][0] = b as u64;
        c.g[A2][0] = (h >> 8) & 0xff;
        c.g[A3][0] = if s3_set { 0x1234_5678 } else { 0 };
        c.g[T0][0] = (h >> 16) & 0xffff_ffff;
        c.pc = 0x001c_eea0;
        r
    }

    /// Both zeroing loops, the lookup with and without a3, all jump-table arms and the
    /// default, the virtual call and the closing lookups.
    #[test]
    fn c_fe_state_reward_gallery_ctor_matches() {
        check_reaching(
            ee::s04::f_001ceea0__generated,
            c_fe_state_reward_gallery_ctor,
            240,
            &[
                0x001c_eed0,
                0x001c_eef8,
                0x001c_efbc,
                0x001c_efc8,
                0x001c_efd8,
                0x001c_efe8,
                0x001c_eff4,
                0x001c_f068,
                0x001c_f070,
                0x001c_f07c,
                0x001c_f08c,
            ],
            &|c, seed| {
                oc1526_ctor_setup(c, seed, 11);
            },
        );
    }

    /// The seeds take many distinct paths (a3 set or not, each jump-table arm, the
    /// default, unwinds at different calls).
    #[test]
    fn c_fe_state_reward_gallery_ctor_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..64 {
            let mut c = crate::difftest::fresh();
            oc1526_ctor_setup(&mut c, seed, 11);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s04::f_001ceea0__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 6, "only {} distinct paths", seen.len());
    }

    driven!(oc1526_ctor_gen_driven, ee::s04::f_001ceea0__generated);
    driven!(oc1526_ctor_hand_driven, c_fe_state_reward_gallery_ctor);

    /// Events at the zeroing loops' back edges: delivered, or unwinding and resumed natively.
    #[test]
    fn c_fe_state_reward_gallery_ctor_events() {
        check_events(
            oc1526_ctor_gen_driven,
            oc1526_ctor_hand_driven,
            120,
            &|c, seed| {
                let mut r = oc1526_ctor_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x001c_eea0)
            },
        );
    }

    /// Resume points of the constructor's twin: the loop heads and a call return.
    #[test]
    fn c_fe_state_reward_gallery_ctor_resumes_match() {
        for pc in [
            0x001c_ef78u32,
            0x001c_ef88,
            0x001c_efa4,
            0x001c_efb4,
            0x001c_f004,
            0x001c_f044,
            0x001c_f050,
            0x001c_f0a0,
        ] {
            check(
                ee::s04::f_001ceea0__generated,
                c_fe_state_reward_gallery_ctor,
                8,
                &|c, seed| {
                    oc1526_ctor_setup(c, seed, 11);
                    // A frame as the first half would have left it.
                    c.g[SP][0] = (crate::difftest::SP - 0x100) as u64;
                    c.g[S2][0] = OC1526_BASE as u64;
                    c.g[S0][0] = (OC1526_BASE + 0x1000) as u64;
                    c.g[S3][0] = if seed & 1 == 0 { 0 } else { 1 };
                    // The clearing loop at 0x1cef88 counts a0 down from 7.
                    if pc == 0x001c_ef88 {
                        c.g[A0][0] = 7;
                    }
                    c.g[A2][0] = (seed % 4) as u64;
                    // The outer loop at 0x1cefa4 takes a2 from a1 (a2 + 1 after a pass).
                    c.g[A1][0] = (seed % 4) as u64 + 1;
                    c.g[A3][0] = (OC1526_BASE + 0x9a4) as u64;
                    c.g[T0][0] = (OC1526_BASE + 0x924) as u64;
                    c.g[V0][0] = (seed % 4) as u64 * 32;
                    c.g[V1][0] = (seed % 5) as u64;
                    c.pc = pc;
                },
            );
        }
    }

    /// Fixture for the two reward-level leaves: the per-rider byte at
    /// `0x4a6ca8 + 0xf88*b + 0x9b50*a + 0xbe5` and the neighbouring total word.
    pub(crate) fn oc1526_level_cell(c: &mut Cpu, a: u32, b: u32, level: u8, total: u32) {
        let base = 0x004a_6ca8u32
            .wrapping_add(0x0f88u32.wrapping_mul(b))
            .wrapping_add(0x9b50u32.wrapping_mul(a));
        c.bus.write8(base + 0xbe5, level);
        c.bus.write32(base + 0xac4, total);
    }

    const OC1526_LEVELS: [u8; 8] = [0, 1, 3, 4, 5, 6, 9, 0xff];

    /// `lbl_00150f98`: at level 5 it returns 0 untouched; otherwise it adds the table cost to
    /// the total word and increments the level.
    #[test]
    fn beecon_raise_attribute_level_matches() {
        check(
            ee::s01::f_00150f98__generated,
            beecon_raise_attribute_level,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x1526_a, 0);
                elf_image(c);
                let h = fakes::mix(seed as u64 ^ 0x1526_a000);
                let (a, b) = ((h % 3) as u32, ((h >> 4) % 3) as u32);
                // t3 is the 0x9b50 stride index, t4 the 0xf88 stride index.
                c.g[T3][0] = a as u64;
                c.g[T4][0] = b as u64;
                oc1526_level_cell(
                    c,
                    a,
                    b,
                    OC1526_LEVELS[((h >> 8) % 8) as usize],
                    (h >> 16) as u32,
                );
                c.pc = 0x0015_0f98;
            },
        );
    }

    /// Resume points of the twin of `lbl_00150f98`, with plausible registers.
    #[test]
    fn beecon_raise_attribute_level_resumes_match() {
        for pc in [0x0015_0fbcu32, 0x0015_0fcc, 0x0015_0ff4, 0x0015_1034, 0x0015_0f9c] {
            check(
                ee::s01::f_00150f98__generated,
                beecon_raise_attribute_level,
                16,
                &|c, seed| {
                    fakes::machine(c, seed, 0x1526_b, 0);
                    elf_image(c);
                    let h = fakes::mix(seed as u64 ^ 0x1526_b000);
                    c.bus.write8(DATA + 0x1000, OC1526_LEVELS[(h % 8) as usize]);
                    c.g[T1][0] = (DATA + 0x1000) as u64;
                    c.g[T3][0] = (h >> 4) % 3;
                    c.g[T4][0] = (h >> 8) % 3;
                    c.g[V0][0] = 5;
                    c.g[A1][0] = (DATA + 0x2000) as u64;
                    c.g[A2][0] = 0x0044_0550;
                    c.g[A3][0] = 0;
                    c.g[V1][0] = 0;
                    c.g[T0][0] = (h >> 12) % 8;
                    c.lo[0] = (h >> 16) % 3;
                    c.pc = pc;
                },
            );
        }
    }

    /// `lbl_00151310`: the cost word for the level byte of rider cell (a1, a2).
    #[test]
    fn beecon_attribute_cost_word_matches() {
        check(
            ee::s01::f_00151310__generated,
            beecon_attribute_cost_word,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x1526_d, 0);
                elf_image(c);
                let h = fakes::mix(seed as u64 ^ 0x1526_d000);
                let (a, b) = ((h % 3) as u32, ((h >> 4) % 3) as u32);
                c.g[A1][0] = a as u64;
                c.g[A2][0] = b as u64;
                oc1526_level_cell(c, a, b, OC1526_LEVELS[((h >> 8) % 8) as usize], 0);
                c.pc = 0x0015_1310;
            },
        );
    }

    /// Resume points of the twin of `lbl_00151310`.
    #[test]
    fn beecon_attribute_cost_word_resumes_match() {
        for pc in [0x0015_1334u32, 0x0015_1348, 0x0015_1314] {
            check(
                ee::s01::f_00151310__generated,
                beecon_attribute_cost_word,
                16,
                &|c, seed| {
                    fakes::machine(c, seed, 0x1526_e, 0);
                    elf_image(c);
                    let h = fakes::mix(seed as u64 ^ 0x1526_e000);
                    c.bus.write8(DATA + 0x1000, OC1526_LEVELS[(h % 8) as usize]);
                    c.g[V1][0] = (DATA + 0x1000) as u64;
                    c.g[V0][0] = (h >> 4) % 10;
                    c.g[A0][0] = 5;
                    c.g[A1][0] = (h >> 8) % 3;
                    c.g[A2][0] = (h >> 12) % 3;
                    c.pc = pc;
                },
            );
        }
    }
}
