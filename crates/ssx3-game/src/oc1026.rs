//! Staging module oc1026 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1026`.

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
    0x0029_def0,
    ssx_audio_system_set_avalanche_sound,
    "SSXAudioSystem_SetAvalancheSound",
)];

/// `SSXAudioSystem_SetAvalancheSound(self, on)`: reference-counted avalanche
/// rumble. `on != 0` raises the user count at `self+0x6040`; the first user
/// allocates a voice (slot `count` of the bank's sub-player arrays), places it
/// at the listener position, sets its volume and radius, and remembers the
/// playing handle in `self+0x5ff4`. `on == 0` lowers the count; the last user
/// stops the handle and clears the three slot ids at `self+0x6038`.
///
/// Registers, cycles and call order follow the generated body block by block.
pub fn ssx_audio_system_set_avalanche_sound(c: &mut Cpu) {
    guest(c, |c| {
        // Block charges as in the generated body (13 + the arms + 12 on exit).
        {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            let skip_arg = c.g[A1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            if !skip_arg {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x6040));
                c.g[V1][0] = 0x1;
                c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                let reuse = c.g[S7][0] != c.g[V1][0];
                c.sw((c.g[S6][0] as u32).wrapping_add(0x6040), c.g[S7][0]);
                if !reuse {
                    c.cycles += 192;
                    c.g[S0][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00288ae0, 0x0029df40)?;
                    c.g[S2][0] = 0x18;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S6][0];
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[S0][0];
                    }
                    c.g[S4][0] = 0x8;
                    call_ptr(c, 0x00285d98, 0x0029df58)?;
                    c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6050));
                    c.g[FP][0] = c.g[V0][0];
                    c.g[S5][0] = c.g[S3][0];
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x6c0));
                    c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0029df7c)?;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x6070), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0029df94)?;
                    c.lwc1(1, (c.g[V0][0] as u32));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x002da1c0, 0x0029dfb8)?;
                    c.lq(3, (c.g[SP][0] as u32));
                    c.g[A2][0] = 0x8;
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0u64;
                    c.g[S0][0] = c.lw((c.g[S6][0] as u32));
                    c.sq(3, (c.g[S6][0] as u32).wrapping_add(0x6050));
                    c.sq(2, (c.g[S6][0] as u32).wrapping_add(0x6060));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, 3, 3, 18);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, (c.g[V1][0] as u32));
                    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                    c.mult(0, 4, 4, 18);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x00416210, 0x0029e098)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[A3][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[T0][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[T1][0] = 0x7f;
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[T1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[T1][0] = 0x2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[S7][0]);
                    c.g[A2][0] = c.lw((c.g[S6][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[S4][0]);
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32));
                    c.g[A2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[T1][0]);
                    c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x0029e438, 0x0029e1f0)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.mult(0, 4, 4, 18);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                    let ge127 = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    let second_read = if ge127 {
                        true
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        let neg = (c.g[V1][0] as i64) < 0;
                        c.g[V0][0] = 0u64;
                        !neg
                    };
                    if second_read {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                        if c.g[V1][0] == 0u64 {
                            c.g[V0][0] = 0x7f;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                            c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                        }
                    }
                    c.cycles += 64;
                    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = 0x5;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x00287968, 0x0029e294)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 13);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[AT][0] = 0x42c80000;
                    c.mtc1(1, 12);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[S5][0]);
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    call_ptr(c, 0x002a9988, 0x0029e2e4)?;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    c.g[A3][0] = 0x4a0000;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                    c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x36d0));
                    c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[T1][0] as u32));
                    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(2, (c.g[SP][0] as u32));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[SP][0] as u32));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x00288a20, 0x0029e334)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A3][0] = 0x1;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1f4));
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x23c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                    call_ptr(c, 0x002906b8, 0x0029e374)?;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x5ff4), c.g[V0][0]);
                }
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x6040));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.sw((c.g[S6][0] as u32).wrapping_add(0x6040), c.g[V0][0]);
                if c.g[V0][0] == 0u64 {
                    c.cycles += 23;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = 0x1;
                    c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5ff4));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[AT][0] = 0x40000000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    call_ptr(c, 0x002ad5f0, 0x0029e3ac)?;
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x118));
                    c.g[V0][0] = 0x4a0000;
                    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x36d7));
                    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x36d0));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xab0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    call_ptr(c, t, 0x0029e3d8)?;
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6038));
                    c.g[V0][0] = 0x2;
                    loop {
                        c.cycles += 7;
                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[V0][0] as i64) >= 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
                        if !t {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0029_e3e8);
                        }
                    }
                }
            }
            c.cycles += 12;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
            c.pc = t;
            Ok(())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const OC1026_CTRL: u32 = DATA + 0xf_0000;
    const OC1026_MAGIC: u32 = 0x1026_c0de;
    /// Nonzero: the `0x288ae0` fake returns 0.
    const OC1026_ZERO_288AE0: u32 = OC1026_CTRL + 4;

    /// A gated fake for one callee slot: logs the call like a missing function.
    macro_rules! oc1026_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1026_CTRL) != OC1026_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if $addr == 0x0028_8ae0 && c.bus.read32(OC1026_ZERO_288AE0) != 0 {
                    c.g[V0][0] = 0;
                }
            }
        };
    }
    oc1026_fake!(
        oc1026_f_288ae0,
        OC1026_P_288AE0,
        ee::s07::f_00288ae0__generated,
        0x0028_8ae0
    );
    oc1026_fake!(
        oc1026_f_285d98,
        OC1026_P_285D98,
        ee::s07::f_00285d98__generated,
        0x0028_5d98
    );
    oc1026_fake!(
        oc1026_f_2da1c0,
        OC1026_P_2DA1C0,
        ee::s09::f_002da1c0__generated,
        0x002d_a1c0
    );
    oc1026_fake!(
        oc1026_f_29e438,
        OC1026_P_29E438,
        ee::s08::f_0029e438__generated,
        0x0029_e438
    );
    oc1026_fake!(
        oc1026_f_287968,
        OC1026_P_287968,
        ee::s07::f_00287968__generated,
        0x0028_7968
    );
    oc1026_fake!(
        oc1026_f_2a9988,
        OC1026_P_2A9988,
        ee::s08::f_002a9988__generated,
        0x002a_9988
    );
    oc1026_fake!(
        oc1026_f_288a20,
        OC1026_P_288A20,
        ee::s07::f_00288a20__generated,
        0x0028_8a20
    );
    oc1026_fake!(
        oc1026_f_2906b8,
        OC1026_P_2906B8,
        ee::s07::f_002906b8__generated,
        0x0029_06b8
    );
    oc1026_fake!(
        oc1026_f_2ad5f0,
        OC1026_P_2AD5F0,
        ee::s08::f_002ad5f0__generated,
        0x002a_d5f0
    );

    fn oc1026_gate(c: &mut Cpu) {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288AE0),
            oc1026_f_288ae0,
            &OC1026_P_288AE0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285D98),
            oc1026_f_285d98,
            &OC1026_P_285D98,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002DA1C0),
            oc1026_f_2da1c0,
            &OC1026_P_2DA1C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029E438),
            oc1026_f_29e438,
            &OC1026_P_29E438,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287968),
            oc1026_f_287968,
            &OC1026_P_287968,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A9988),
            oc1026_f_2a9988,
            &OC1026_P_2A9988,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288A20),
            oc1026_f_288a20,
            &OC1026_P_288A20,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002906B8),
            oc1026_f_2906b8,
            &OC1026_P_2906B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD5F0),
            oc1026_f_2ad5f0,
            &OC1026_P_2AD5F0,
        );
        c.bus.write32(OC1026_CTRL, OC1026_MAGIC);
    }

    /// Avalanche audio instance with its bank, in all arms: first user (4
    /// volume-byte cases, `0x288ae0` zero or not, 0..2 earlier voices), a
    /// later user (count only), the last user stopping (vtable call + clearing
    /// loop) and a non-last release.
    #[test]
    fn ssx_audio_system_set_avalanche_sound_matches() {
        const SELF: u32 = DATA + 0x1000;
        const BANK: u32 = DATA + 0x9000;
        const P118: u32 = DATA + 0x8f00;
        const FP: u32 = DATA + 0xc000;
        const VT: u32 = DATA + 0xd000;
        const VT2: u32 = DATA + 0xd800;
        const VEC: u32 = DATA + 0xe000;
        fakes::covered(
            ee::s08::f_0029def0__generated,
            ssx_audio_system_set_avalanche_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29def0, 10);
                oc1026_gate(c);
                c.bus.write32(OC1026_ZERO_288AE0, (seed / 2) % 2);
                // The self object: bank, sub-player arrays, bank-pointer cell.
                c.bus.write32(SELF, BANK);
                c.bus.write32(SELF + 4, DATA + 0xa000);
                c.bus.write32(SELF + 8, DATA + 0xa100);
                c.bus.write32(SELF + 0x118, P118);
                c.bus.write32(P118, BANK);
                c.bus.write32(SELF + 0x6038, r.next_u32());
                c.bus.write32(SELF + 0x5ff4, r.next_u32());
                // Bank: voice count and the per-voice arrays.
                let count = seed % 3;
                c.bus.write32(BANK + 0x1f4, count);
                let s1 = BANK + 0x1d8;
                let tables = [
                    (0x20, 0xb000),
                    (0x34, 0xb100),
                    (0x38, 0xb200),
                    (0x3c, 0xb300),
                    (0x40, 0xb400),
                    (0x60, 0xb500),
                    (0x64, 0xb600),
                    (0x68, 0xb700),
                    (0x6c, 0xb800),
                    (0x70, 0xb900),
                    (0x74, 0xba00),
                    (0x78, 0xbb00),
                    (0x7c, 0xbc00),
                    (0x80, 0xbd00),
                    (0x84, 0xbe00),
                ];
                for (off, at) in tables {
                    c.bus.write32(s1 + off, DATA + at);
                }
                // Voice parameters template at bank+0x1dc.. and the volume byte.
                for k in 0..6 {
                    c.bus.write32(BANK + 0x1dc + 4 * k, r.next_u32());
                }
                let volume = [0x7fu8, 0xfb, 40, 0][((seed / 3) % 4) as usize];
                c.bus.write8(DATA + 0xb000 + 0x18 * (count + 1), volume);
                c.bus.write32(DATA + 0xa000 + 4 * (count + 1), r.next_u32());
                // The listener object: vtable slots 0x28 (position) and 0x38.
                c.bus.write32(FP + 0x6c0, VT);
                for (i, off) in [0x28u32, 0x38].into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(VT + off, delta as u16);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
                }
                for k in 0..3 {
                    c.bus.write32(VEC + 4 * k, r.f32(100.0));
                }
                c.bus.write32(LOG_RESULTS + 4, FP);
                c.bus.write32(LOG_RESULTS + 12, VEC);
                // Stop path: the bank's callback table.
                c.bus.write32(BANK + 0xab0, VT2);
                c.bus
                    .write16(VT2 + 0x18, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(VT2 + 0x1c, FAKE + 0x20);
                // D_004A36D0: the 8-byte constant copied into the voice.
                c.bus.write32(0x004a_36d0, r.next_u32());
                c.bus.write32(0x004a_36d4, r.next_u32());
                let (on, users) = match seed % 6 {
                    0 | 1 | 5 => (1 + r.next_u32() % 3, 0),
                    2 => (1, 3),
                    3 => (0, 1),
                    _ => (0, 5),
                };
                c.bus.write32(SELF + 0x6040, users);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = on as u64;
                c.pc = 0x0029_def0;
            },
        );
    }
}
